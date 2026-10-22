//! 문턱 즉치 패치. 원래 바이트를 대조하고, 다른 스레드를 멈춘 뒤(RIP 가 그 명령 안이면 재시도) 쓴다.
//! 되돌리기는 하지 않는다(게임 재시작 = 원복). 같은 명령에 게이트가 붙으면 게이트 스텁은 패치된 바이트를 쓴다.
use crate::platform;
use crate::sites::{KnobDef, Tf, KNOBS};
use std::collections::BTreeMap;

pub struct KnobStatus { pub id: String, pub value: i64, pub default: i64, pub result: String }

fn encode(tf: Tf, value: i64, width: usize) -> Result<Vec<u8>, String> {
    match tf {
        Tf::Add(d) => {
            if ![1, 2, 4, 8].contains(&width) { return Err("즉치 폭 오류".into()); }
            let v = value.checked_add(d).ok_or("즉치 덧셈 overflow")?;
            let max = if width >= 8 { i64::MAX } else { (1i64 << (width * 8)) - 1 };
            // imm8 비교는 부호 확장된다 — 노브 범위(min/max)가 0..127 로 막는다.
            if v < 0 || v > max { return Err(format!("바이트 범위 밖: {v}")); }
            Ok((v as u64).to_le_bytes()[..width].to_vec())
        }
        Tf::SquarePlus1 => {
            let v = (value as i128) * (value as i128) + 1;
            if width != 8 || v > i64::MAX as i128 { return Err("R²+1 범위 밖".into()); }
            Ok((v as u64).to_le_bytes().to_vec())
        }
        Tf::Toggle { on, off } => {
            if width != 1 { return Err("toggle 폭 오류".into()); }
            Ok(vec![if value != 0 { on } else { off }])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transformations_and_bounds() {
        assert_eq!(encode(Tf::Add(-1), 60, 1).unwrap(), vec![59]);
        assert_eq!(encode(Tf::Add(1), 2, 1).unwrap(), vec![3]);
        assert_eq!(encode(Tf::SquarePlus1, 150000, 8).unwrap(), 22_500_000_001u64.to_le_bytes());
        assert_eq!(encode(Tf::Toggle { on: 0x75, off: 0xeb }, 0, 1).unwrap(), vec![0xeb]);
        assert!(encode(Tf::Add(1), i64::MAX, 8).is_err());
        assert!(encode(Tf::Add(0), 256, 1).is_err());
        assert!(encode(Tf::Add(0), -1, 1).is_err());
        assert!(encode(Tf::Add(0), 1, 9).is_err());
    }
    #[test]
    fn generated_knob_defaults_reproduce_original_bytes() {
        for k in KNOBS {
            for w in k.writes {
                assert_eq!(encode(w.tf, k.default, w.width).unwrap(), w.orig[w.at..w.at + w.width], "{}", k.id);
            }
        }
    }
}

fn write_one(addr: usize, orig: &[u8], at: usize, bytes: &[u8]) -> Result<(), String> {
    let mut cur = vec![0u8; orig.len()];
    platform::safe_read_bytes(addr, &mut cur).map_err(str::to_string)?;
    if cur != orig { return Err(format!("원래 바이트 불일치: {:02x?}", cur)); }
    let mut last = String::new();
    for _ in 0..50 {
        match platform::SuspendedThreads::acquire(addr, orig.len()) {
            Ok(mut stopped) => {
                // 최초 검사 뒤 정지 장벽까지의 경쟁을 다시 확인한다. 할당은 정지 전에 끝냈다.
                let matches = platform::safe_read_bytes(addr, &mut cur).map(|_| cur == orig);
                let outcome = if matches == Ok(true) {
                    Some(unsafe { platform::write_code(addr + at, bytes) })
                } else { None };
                let resumed = stopped.resume_all();
                let outcome = outcome.ok_or("패치 직전 원래 바이트 재검사 실패")?;
                outcome.result.map_err(str::to_string)?;
                resumed.map_err(str::to_string)?;
                return Ok(());
            }
            Err(e) => { last = e.to_string(); std::thread::sleep(std::time::Duration::from_millis(2)); }
        }
    }
    Err(format!("스레드 정지 실패: {last}"))
}

/// 반환: 노브 상태 목록, 패치 뒤 명령 바이트(rva → 바이트). 게이트 스텁이 이것을 쓴다.
pub fn apply(base: usize, sets: &[(String, i64)]) -> (Vec<KnobStatus>, BTreeMap<usize, Vec<u8>>) {
    let mut status = Vec::new();
    let mut effective: BTreeMap<usize, Vec<u8>> = BTreeMap::new();
    for (id, value) in sets {
        let Some(k): Option<&KnobDef> = KNOBS.iter().find(|k| k.id == id) else {
            status.push(KnobStatus { id: id.clone(), value: *value, default: 0, result: "알 수 없는 id".into() });
            continue;
        };
        let mut st = KnobStatus { id: id.clone(), value: *value, default: k.default, result: String::new() };
        if *value < k.min || *value > k.max {
            st.result = format!("범위 {}..{} 밖", k.min, k.max);
            status.push(st); continue;
        }
        if *value == k.default { st.result = "기본값(쓰기 없음)".into(); status.push(st); continue; }
        // 먼저 전부 인코딩해 본 뒤 쓴다 — 일부만 바뀐 노브를 만들지 않는다.
        let mut plan = Vec::new();
        for w in k.writes {
            match encode(w.tf, *value, w.width) {
                Ok(b) => plan.push((w, b)),
                Err(e) => { st.result = e; break; }
            }
        }
        if !st.result.is_empty() { status.push(st); continue; }
        let mut done = 0;
        for (w, bytes) in &plan {
            let addr = base + w.rva;
            match write_one(addr, w.orig, w.at, bytes) {
                Ok(()) => {
                    let mut eff = w.orig.to_vec();
                    eff[w.at..w.at + w.width].copy_from_slice(bytes);
                    effective.insert(w.rva, eff);
                    done += 1;
                }
                Err(e) => { st.result = format!("{:#x}: {e}", w.rva); break; }
            }
        }
        if st.result.is_empty() { st.result = format!("적용 {done}곳"); }
        else if done > 0 { st.result = format!("⚠일부만 적용 {done}/{} — 이 판 결과는 쓰지 말 것 · {}", plan.len(), st.result); }
        status.push(st);
    }
    (status, effective)
}
