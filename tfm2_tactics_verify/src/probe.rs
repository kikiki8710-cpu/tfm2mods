//! 비교 명령 INT3 게이트 설치·무장 해제. 값 기록은 platform::handle_site_gate 가 한다.
use crate::platform::{self, cap_word, CAP_NONE, CAP_REG, CAP_STACK, CAP_VALUE};
use crate::sites::{Cap, GATES};
use std::collections::BTreeMap;

pub struct GateStatus {
    pub id: &'static str,
    pub rva: usize,
    pub slot: Option<usize>,
    pub entry: usize,
    pub first: u8,
    pub armed: bool,
    pub patched: bool,
    pub b_value: Option<i64>,
    pub install_result: String,
    pub result: String,
}

fn cap(c: Cap, bytes: &[u8]) -> Result<([u64; 2], Option<i64>), String> {
    let (word, value, immediate) = match c {
        Cap::None => (cap_word(CAP_NONE, 0, 0, 0), 0, None),
        Cap::Reg { off, width } => {
            if ![1, 2, 4, 8].contains(&width) || !(0x78..=0xf0).contains(&off) || off % 8 != 0 {
                return Err("레지스터 캡처 범위 오류".into());
            }
            (cap_word(CAP_REG, width, off, 0), 0, None)
        }
        Cap::Stack { base, disp, width } => {
            if ![0x98, 0xa0].contains(&base) || ![1, 2, 4, 8].contains(&width) {
                return Err("스택 캡처 규칙 오류".into());
            }
            (cap_word(CAP_STACK, width, base, disp), 0, None)
        }
        Cap::Const { value, width } => {
            if ![1, 2, 4, 8].contains(&width) { return Err("상수 폭 오류".into()); }
            (cap_word(CAP_VALUE, width, 0, 0), value as u64, Some(value))
        }
        Cap::Imm { at, size, width } => {
            if ![1, 2, 4, 8].contains(&size) || ![1, 2, 4, 8].contains(&width) || size > width {
                return Err("즉치 폭 오류".into());
            }
            let raw = bytes.get(at as usize..at as usize + size as usize).ok_or("즉치 범위 오류")?;
            let mut buf = [0u8; 8]; buf[..raw.len()].copy_from_slice(raw);
            let value = platform::sign(u64::from_le_bytes(buf), size);
            (cap_word(CAP_VALUE, width, 0, 0), value as u64, Some(value))
        }
    };
    Ok(([word, value], immediate))
}

fn stub_bytes(instruction: &[u8], next: usize) -> Vec<u8> {
    let mut code = instruction.to_vec();
    // RIP 간접 절대 점프: 레지스터와 비교 플래그를 보존한다。
    code.extend_from_slice(&[0xff, 0x25, 0, 0, 0, 0]);
    code.extend_from_slice(&(next as u64).to_le_bytes());
    code
}

pub fn install(base: usize, effective: &BTreeMap<usize, Vec<u8>>, filter: &[String]) -> Vec<GateStatus> {
    GATES.iter().map(|g| {
        let bytes = effective.get(&g.rva).map(Vec::as_slice).unwrap_or(g.orig);
        let mut status = GateStatus { id: g.id, rva: g.rva, slot: None, entry: 0,
            first: bytes.first().copied().unwrap_or(0), armed: false,
            patched: bytes != g.orig, b_value: None, install_result: String::new(), result: String::new() };
        let outcome = (|| -> Result<(), String> {
            if !filter.is_empty() && !filter.iter().any(|f| g.id.starts_with(f)) {
                return Err("필터 제외".into());
            }
            if platform::debugger_present() { return Err("디버거 감지: INT3 설치 거절".into()); }
            if bytes.len() != g.orig.len() || bytes.is_empty() || bytes[0] == 0xcc {
                return Err("명령 길이/첫 바이트 오류".into());
            }
            let entry = base.checked_add(g.rva).ok_or("주소 overflow")?;
            let next = entry.checked_add(bytes.len()).ok_or("주소 overflow")?;
            platform::executable_range(entry, bytes.len()).map_err(str::to_string)?;
            let mut current = vec![0; bytes.len()];
            platform::safe_read_bytes(entry, &mut current).map_err(str::to_string)?;
            if current != bytes { return Err("실제 명령 바이트 불일치".into()); }
            let (a, _) = cap(g.a, bytes)?;
            let (b, b_value) = cap(g.b, bytes)?;
            status.b_value = b_value;
            let stub = platform::allocate_code(&stub_bytes(bytes, next)).map_err(str::to_string)?;
            let slot = match platform::register_site_gate(entry, stub, a, b) {
                Ok(slot) => slot,
                Err(e) => { unsafe { platform::free_unpublished_code(stub); } return Err(e.into()); }
            };
            // 여기부터는 게시된 descriptor와 스텁을 실패 시에도 회수하지 않는다。
            status.slot = Some(slot); status.entry = entry;
            let mut stopped = platform::SuspendedThreads::acquire(entry, bytes.len()).map_err(str::to_string)?;
            let same = platform::safe_read_bytes(entry, &mut current).map(|_| current == bytes);
            let write = if same == Ok(true) {
                Some(unsafe { platform::publish_code_byte(entry, bytes[0], 0xcc) })
            } else { None };
            let resumed = stopped.resume_all();
            if let Some(write) = write {
                status.armed = write.attempted;
                write.result.map_err(str::to_string)?;
            } else { return Err("설치 직전 바이트 재검사 실패".into()); }
            resumed.map_err(str::to_string)?;
            Ok(())
        })();
        status.result = match outcome { Ok(()) => "설치 완료".into(), Err(e) => format!("거절/오류: {e}") };
        status.install_result = status.result.clone();
        status
    }).collect()
}

pub fn disarm(gate: &mut GateStatus) {
    if !gate.armed { return; }
    let outcome = unsafe { platform::publish_code_byte(gate.entry, 0xcc, gate.first) };
    if outcome.attempted { gate.armed = false; }
    gate.result = match outcome.result {
        Ok(()) => "호출 예산 도달: 무장 해제".into(),
        Err(e) => format!("무장 해제 오류: {e}"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn immediates_follow_effective_bytes_and_sign_extension() {
        let c = Cap::Imm { at: 3, size: 1, width: 8 };
        assert_eq!(cap(c, &[0x48, 0x83, 0xf8, 55]).unwrap().1, Some(55));
        assert_eq!(cap(c, &[0x48, 0x83, 0xf8, 0xff]).unwrap().1, Some(-1));
        assert!(cap(c, &[]).is_err());
    }
    #[test]
    fn generated_caps_and_non_clobbering_stub() {
        for g in GATES { cap(g.a, g.orig).unwrap(); cap(g.b, g.orig).unwrap(); }
        let code = stub_bytes(&[0x85, 0xc0], 0x140001234);
        assert_eq!(&code[..8], &[0x85, 0xc0, 0xff, 0x25, 0, 0, 0, 0]);
        assert_eq!(&code[8..], &0x140001234u64.to_le_bytes());
    }
}
