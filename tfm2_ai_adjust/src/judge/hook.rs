//! hook — judge 계층 전용 wrap 설치기. detour.rs `install_wrap` 과 같은 기계(순수 트램폴린 + 진입부 `movabs rax; jmp rax`)지만,
//! 프롤로그를 push8 고정이 아니라 **gen_fns 에 채록된 바이트와 완전 일치**할 때만 패치한다.
//!   · orig_len = 옮기는 바이트 수 = 명령 경계(≥12). aiport 가 capstone 으로 계산하고 rip-상대/분기 명령이 없음을 확인해 둔 값.
//!   · 바이트가 하나라도 다르면 미설치(Err) — 스테일 RVA·패치판·다른 모드의 선행 훅 전부 여기서 걸러진다(fail-safe).
//!   · 반환 = 트램폴린 주소(= 게임 원본을 그대로 부르는 함수 포인터). wrap 이 이걸로 원본을 실행한다.
//! ★체인 후킹 지원(2026-09-09) — 진입부가 이미 `48 b8 <tgt> ff e0` 면 그 tgt 로 점프하는 스텁을 만들어 **바깥 훅**이 된다.
//!   이 모드 자신이 `install_replace_detour` 로 먼저 잡은 함수(예: 0xcaf9f0 BigPlan::sub_plan)를 judge 가 사후 대조하려면 이게 필요하다.
use crate::*;

pub unsafe fn install_wrap_bytes(rva: usize, prolog: &[u8], cap_fn: usize) -> Result<usize, &'static str> {
    let orig_len = prolog.len();
    if orig_len < 12 || orig_len > 32 { return Err("orig_len 범위(12..=32) 밖"); }
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, orig_len + 4) { return Err("fn unreadable"); }
    // ★[2026-09-09] 체인 후킹(CLAUDE.md §3) — 진입부가 이미 `48 b8 <tgt:8> ff e0` 면 **원본 대신 tgt**(선행 훅의 스텁)로 간다.
    //   선행 훅이 원본 프롤로그를 자기 스텁으로 이미 옮겼으므로 여기선 **옮길 프롤로그가 없다** = 스텁은 `jmp tgt` 한 줄.
    //   진입부는 12B 만 덮어쓴다(나머지는 선행 훅이 남긴 nop). ⚠늦게 설치하는 쪽이 바깥이 된다 — judge 는 모드 detour 뒤에 설치된다.
    if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8
        && *((fn_addr + 10) as *const u8) == 0xff && *((fn_addr + 11) as *const u8) == 0xe0 {
        let tgt = core::ptr::read_unaligned((fn_addr + 2) as *const usize);
        if tgt == cap_fn { return Err("체인: 이미 내가 훅함"); }
        if !ptr_ok(tgt) || !readable(tgt, 4) { return Err("체인 tgt 비정상"); }
        const MEM_CR2: u32 = 0x1000 | 0x2000; const RWX2: u32 = 0x40;
        let stub = stub_reg(VirtualAlloc(0, 64, MEM_CR2, RWX2), 64, rva);
        if stub == 0 { return Err("VirtualAlloc(chain)"); }
        let mut s: Vec<u8> = Vec::with_capacity(14);
        s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); s.extend_from_slice(&tgt.to_le_bytes());   // jmp [rip+0]; dq tgt
        core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
        let mut patch = [0u8; 12];
        patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&cap_fn.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
        let mut old: u32 = 0;
        if VirtualProtect(fn_addr, 12, RWX2, &mut old) == 0 { return Err("VirtualProtect(chain)"); }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
        VirtualProtect(fn_addr, 12, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
        return Ok(stub);
    }
    for i in 0..orig_len {
        if *((fn_addr + i) as *const u8) != prolog[i] {
            return Err(if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8 { "진입부가 이미 훅됨(48 b8) — 체인 미지원" } else { "프롤로그 바이트 불일치(스테일 RVA/패치판) — 미설치" });
        }
    }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = stub_reg(VirtualAlloc(0, 128, MEM_CR, RWX), 128, rva);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::with_capacity(orig_len + 14);
    s.extend_from_slice(prolog);                                                             // 옮긴 원본 프롤로그
    // ★[2026-09-06 크래시 후 수정] 복귀 점프는 레지스터를 건드리지 않는 `jmp qword ptr [rip+0]` + imm64 로.
    //   구 `movabs rax, fn+len; jmp rax` 는 옮긴 프롤로그가 rax 를 세팅하는 함수(steal: `movzx eax,[rcx+8]` = phase)에서
    //   그 값을 파괴해 원본이 엉뚱한 분기를 탔다. detour.rs 의 install_wrap 은 push8 전용이라 같은 문제가 안 드러났을 뿐이다.
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); s.extend_from_slice(&ret_addr.to_le_bytes());   // jmp [rip+0]; dq fn+len
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
