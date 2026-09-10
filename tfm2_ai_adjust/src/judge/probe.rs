//! probe — **카운트 전용 프로브**. "이 함수가 실제 경기에서 뜨는가" 만 센다.
//!
//! 왜 필요한가: 남은 포팅 범위를 정적 콜그래프로만 추정하면 **죽은 코드까지 세게 된다**.
//!   실측 선례 = tower_dive 의 is_unreasonable_tower_dive_enemy 서브트리(약 1,900줄)가 version≤1 전용이라
//!   0.5.8 경기에선 단 한 번도 안 탄다. 이런 걸 빼야 "얼마 남았나"가 숫자로 의미를 가진다.
//!
//! 안전성(=인자·반환형을 몰라도 되는 이유): 스텁이 **레지스터·스택을 일절 건드리지 않는다.**
//!   stub+0  : u64 카운터
//!   stub+8  : F0 48 FF 05 F0FFFFFF   lock inc qword [rip-16]   ← 오직 EFLAGS 만 변경(x64 ABI 상 비보존)
//!   stub+16 : 원본 프롤로그 len 바이트 (aiprobe.py 가 capstone 으로 rip-상대/분기 없음을 확인)
//!   stub+16+len : FF 25 00000000 / dq (fn+len)   원본으로 복귀
//!   진입부는 `48 b8 <stub+8> ff e0` 12B 로 교체.
//! ⚠ rax 를 쓰지 않는다 — 진입부 패치의 movabs 가 rax 를 깨지만 그건 함수 첫 명령 이전이라 무해하고,
//!   스텁 본문은 rax 를 안 건드리므로 프롤로그가 rax 를 세팅하는 함수도 안전하다.
//!
//! 기본 꺼짐. `judge_probe = 1` 일 때만 설치한다(91개 진입부를 패치하므로 상시 켤 것이 아니다).
use crate::*;

static mut SLOTS: [usize; 128] = [0; 128];   // 프로브 i 의 스텁 주소(= 카운터 주소)
static mut NPROBE: usize = 0;

/// 프로브 설치. 반환 = (설치 성공 수, 시도 수).
pub unsafe fn install_all() -> (usize, usize) {
    let tbl = super::probe_tbl::PROBES;
    let mbase = exe_base();
    if mbase == 0 { return (0, 0); }
    let mut ok = 0usize;
    let n = tbl.len().min(128);
    for (i, p) in tbl.iter().take(n).enumerate() {
        match install_one(mbase, p) {
            Ok(stub) => { SLOTS[i] = stub; ok += 1; }
            Err(_) => { SLOTS[i] = 0; }
        }
    }
    NPROBE = n;
    (ok, n)
}

unsafe fn install_one(mbase: usize, p: &super::probe_tbl::Probe) -> Result<usize, &'static str> {
    let len = p.len as usize;
    if len < 12 || len > 24 { return Err("len"); }
    let fn_addr = mbase + p.rva;
    if !readable(fn_addr, len + 4) { return Err("unreadable"); }
    // 이미 누가 훅한 함수는 건드리지 않는다(체인까지 만들 이유가 없다 — 그건 이미 "뜬다"가 확인된 함수다).
    if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8 { return Err("이미 훅됨"); }
    for i in 0..len {
        if *((fn_addr + i) as *const u8) != p.prolog[i] { return Err("프롤로그 불일치"); }
    }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 128, MEM_CR, RWX), 128, p.rva);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = Vec::with_capacity(64);
    s.extend_from_slice(&0u64.to_le_bytes());                                   // +0  카운터
    s.extend_from_slice(&[0xf0, 0x48, 0xff, 0x05, 0xf0, 0xff, 0xff, 0xff]);     // +8  lock inc qword [rip-16]
    s.extend_from_slice(&p.prolog[..len]);                                      // +16 원본 프롤로그
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);                 //     jmp [rip+0]
    s.extend_from_slice(&(fn_addr + len).to_le_bytes());                        //     dq fn+len
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let entry = stub + 8;                                                        // 코드 시작(카운터 8B 뒤)
    let mut patch = vec![0x90u8; len];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&entry.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, len);
    VirtualProtect(fn_addr, len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, len);
    Ok(stub)
}

/// 결과 덤프 — 발화한 것 / 안 한 것을 명령수와 함께.
pub unsafe fn report() -> String {
    let tbl = super::probe_tbl::PROBES;
    let mut hit: Vec<(u64, usize, &str, u32)> = Vec::new();
    let mut zero: Vec<(usize, &str, u32)> = Vec::new();
    let mut miss = 0usize;
    for (i, p) in tbl.iter().take(NPROBE).enumerate() {
        if SLOTS[i] == 0 { miss += 1; continue; }
        let c = core::ptr::read_volatile(SLOTS[i] as *const u64);
        if c > 0 { hit.push((c, p.rva, p.name, p.ins)); } else { zero.push((p.rva, p.name, p.ins)); }
    }
    hit.sort_by(|a, b| b.0.cmp(&a.0));
    let live_ins: u32 = hit.iter().map(|h| h.3).sum();
    let dead_ins: u32 = zero.iter().map(|z| z.2).sum();
    let mut s = format!(
        "[probe] 설치 {} · 미설치 {} | 발화 {}개({} 명령) · 미발화 {}개({} 명령) → 죽은 비율 {:.0}%\n",
        NPROBE - miss, miss, hit.len(), live_ins, zero.len(), dead_ins,
        100.0 * dead_ins as f64 / (live_ins + dead_ins).max(1) as f64);
    s.push_str("--- 발화(호출수 내림차순)\n");
    for (c, rva, nm, ins) in hit.iter() { s.push_str(&format!("  {:>12} 0x{:x} {:5} ins  {}\n", c, rva, ins, nm)); }
    s.push_str("--- 미발화(= 이번 판에서 죽은 코드)\n");
    for (rva, nm, ins) in zero.iter() { s.push_str(&format!("             . 0x{:x} {:5} ins  {}\n", rva, ins, nm)); }
    s
}
