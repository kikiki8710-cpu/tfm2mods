//! hookw — **sweep 전용 wrap 설치기**(트램폴린 + 진입부 `movabs rax; jmp rax`).
//! ===========================================================================
//! 이관 원본 = `tfm2_ai_adjust\src\judge\hook.rs::install_wrap_bytes`. 기계는 같다:
//!   트램폴린 = [원본 프롤로그 len 바이트] + `jmp qword ptr [rip+0]` + `dq fn+len`
//!   진입부   = `48 b8 <wrap:8> ff e0` 12B + 나머지 NOP
//!   반환     = 트램폴린 주소 = **게임 원본을 그대로 부르는 함수 포인터**(wrap 이 이걸로 원본을 실행).
//!
//! 원본과 **다르게 만든 것 2가지** (이유를 남긴다)
//!   ①★`orig` 를 **진입부 패치 전에** 저장한다(`orig: &AtomicUsize` 를 받는다).
//!      원본은 `install_wrap_bytes` 가 패치까지 끝낸 뒤 호출자가 `S[i].orig.store(o)` 를 했다 —
//!      그 사이에 **다른 스레드가 그 함수에 진입하면** wrap 이 `orig==0` 을 transmute 해서
//!      **널 호출**을 한다. 이 게임은 배경 sim 을 rayon 워커로 돌리므로(판정 함수가 워커 스레드에서
//!      불린다 = 1단계 실측으로 확인된 사실) 실재하는 경합이다. ⟹ 저장을 먼저 한다.
//!   ②**체인 후킹 미지원**. 진입부가 이미 `48 b8`(다른 모드가 선점)이면 **설치하지 않는다.**
//!      1단계 프로브의 규율을 그대로 따른다 — 07-18 에 두 모드가 서로를 재체인해 게임이 먹통이 된
//!      실사고가 있다(CLAUDE.md §3). 검증 모드가 프로덕션 모드의 훅 위에 얹힐 이유가 없다.
//!
//! ⚠프롤로그를 **옮겨 실행**하므로 그 안에 분기/call/rip-상대가 있으면 안 된다 —
//!   `MIG\gensweep20.py` 가 capstone 으로 명령 경계를 잡고(분기·rip 만나면 중단) 12B 미확보분은
//!   애초에 표에서 제외한다. 여기서는 **채록된 바이트와 완전 일치**할 때만 패치한다(스테일 RVA 차단).
//! ⚠복귀 점프는 레지스터를 안 건드리는 `jmp qword ptr [rip+0]` + imm64 다.
//!   구 `movabs rax, fn+len; jmp rax` 는 옮긴 프롤로그가 rax 를 세팅하는 함수에서 그 값을 파괴해
//!   원본이 엉뚱한 분기를 탔다(ai_adjust 2026-09-06 크래시 기록).
#![allow(dead_code)]
use crate::{
    exe_base, readable, stub_reg, FlushInstructionCache, GetCurrentProcess, VirtualAlloc,
    VirtualProtect,
};
use std::sync::atomic::{AtomicUsize, Ordering};

const MEM_CR: u32 = 0x1000 | 0x2000; // MEM_COMMIT|MEM_RESERVE
const RWX: u32 = 0x40; // PAGE_EXECUTE_READWRITE

/// wrap 설치. 성공 시 트램폴린 주소를 `orig` 에 저장하고 같은 값을 반환한다.
/// 실패 시 `.text` 는 **무손상**(아무것도 안 쓴다).
pub unsafe fn install_wrap(
    rva: usize,
    prolog: &[u8],
    wrap_fn: usize,
    orig: &AtomicUsize,
) -> Result<usize, &'static str> {
    let len = prolog.len();
    if len < 12 || len > 32 {
        return Err("프롤로그 길이 범위(12..=32) 밖 — 표 이상");
    }
    if wrap_fn == 0 {
        return Err("wrap 함수 주소 0");
    }
    if orig.load(Ordering::Relaxed) != 0 {
        return Err("이미 설치됨(중복 호출)");
    }
    let mbase = exe_base();
    if mbase == 0 {
        return Err("exe_base 0");
    }
    let fn_addr = mbase.wrapping_add(rva);
    if fn_addr < 0x10000 || fn_addr >= (1usize << 48) {
        return Err("fn 주소 범위 밖");
    }
    if !readable(fn_addr, len + 4) {
        return Err("fn 읽기불가(RVA 가 코드가 아님)");
    }
    // ★이미 누가 훅한 함수는 절대 건드리지 않는다(체인 금지 — 위 헤더 ②).
    if *(fn_addr as *const u8) == 0x48 && *((fn_addr + 1) as *const u8) == 0xb8 {
        return Err("이미 훅됨(다른 모드가 선점) — 체인 미지원이라 설치 안 함");
    }
    for i in 0..len {
        if *((fn_addr + i) as *const u8) != prolog[i] {
            return Err("프롤로그 바이트 불일치(스테일 RVA/패치판) — 미설치");
        }
    }

    // ── ① 트램폴린 ──
    let stub = stub_reg(VirtualAlloc(0, 128, MEM_CR, RWX), 128, rva);
    if stub == 0 {
        return Err("VirtualAlloc 실패");
    }
    let ret_addr = fn_addr + len;
    let mut s: Vec<u8> = Vec::with_capacity(len + 14);
    s.extend_from_slice(&prolog[..len]);
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); // jmp [rip+0]
    s.extend_from_slice(&ret_addr.to_le_bytes()); // dq fn+len
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());

    // ── ② ★orig 를 **패치 전에** 게시(위 헤더 ①) ──
    orig.store(stub, Ordering::SeqCst);

    // ── ③ 진입부 패치 ──
    let mut patch = vec![0x90u8; len];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&wrap_fn.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, len, RWX, &mut old) == 0 {
        orig.store(0, Ordering::SeqCst); // 게시 취소 — 패치 안 됐으니 wrap 은 불리지 않는다
        return Err("VirtualProtect 실패");
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, len);
    VirtualProtect(fn_addr, len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, len);
    Ok(stub)
}
