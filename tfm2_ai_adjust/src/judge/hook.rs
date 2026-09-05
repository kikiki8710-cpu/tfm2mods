//! hook — judge 계층 전용 wrap 설치기. detour.rs `install_wrap` 과 같은 기계(순수 트램폴린 + 진입부 `movabs rax; jmp rax`)지만,
//! 프롤로그를 push8 고정이 아니라 **gen_fns 에 채록된 바이트와 완전 일치**할 때만 패치한다.
//!   · orig_len = 옮기는 바이트 수 = 명령 경계(≥12). aiport 가 capstone 으로 계산하고 rip-상대/분기 명령이 없음을 확인해 둔 값.
//!   · 바이트가 하나라도 다르면 미설치(Err) — 스테일 RVA·패치판·다른 모드의 선행 훅 전부 여기서 걸러진다(fail-safe).
//!   · 반환 = 트램폴린 주소(= 게임 원본을 그대로 부르는 함수 포인터). wrap 이 이걸로 원본을 실행한다.
//! ⚠ 체인 후킹(진입부가 이미 외부 훅)은 지원하지 않는다 — judge 대상 함수는 다른 모드가 훅하지 않는 AI 내부 함수뿐이라는 전제.
//!   만약 필요해지면 CLAUDE.md §3 체인 규약(`48 b8 <tgt> ff e0` 감지 → tgt 로 점프)을 여기 추가한다.
use crate::*;

pub unsafe fn install_wrap_bytes(rva: usize, prolog: &[u8], cap_fn: usize) -> Result<usize, &'static str> {
    let orig_len = prolog.len();
    if orig_len < 12 || orig_len > 32 { return Err("orig_len 범위(12..=32) 밖"); }
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len + 4) { return Err("fn unreadable"); }
    for i in 0..orig_len {
        if *((fn_addr + i) as *const u8) != prolog[i] {
            return Err(if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8 { "진입부가 이미 훅됨(48 b8) — 체인 미지원" } else { "프롤로그 바이트 불일치(스테일 RVA/패치판) — 미설치" });
        }
    }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 128, MEM_CR, RWX), 128, rva);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::with_capacity(orig_len + 12);
    s.extend_from_slice(prolog);                                                             // 옮긴 원본 프롤로그
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); s.extend_from_slice(&[0xff, 0xe0]);   // movabs rax, fn+len; jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&cap_fn.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(stub)
}
