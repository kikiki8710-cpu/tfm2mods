//! pin02 — `#02 AttackNexusPlan::sub_plan` **midpin sweep**(2026-09-13).
//! ===========================================================================
//! 왜 midpin 인가 = `#02` 는 exe 에 독립 진입부가 없다. `BigPlan::sub_plan`(0xcaf9f0) 안 점프테이블
//!   idx14 arm(`0xcafa57`, +0x67)에 LTO 인라인됐다(RE `2026-09-12_02번_BigPlan_sub_plan_확정_ghidra.md`).
//!   진입부 detour 로는 「BigPlan 16 variant 합계」만 보이고 attack_nexus 만 따로 대조할 수 없다.
//!
//! 기계 = 핀 2개(둘 다 7B 명령 하나를 훔쳐 `jmp rel32` + nop×2):
//!   A `0xcafa57` `mov rcx,[r11+0x930]`(7B) — arm 진입. 이 시점 레지스터(디스어셈 실측 `MIG\disrva.py`):
//!        rsi = sret(SubPlan 72B) · rdx = &BigPlan(페이로드 = rdx+8 = &AttackNexusPlan) · rbx = 태그(16)
//!        r8 = version · r9 = rng · r11 = &PlayerState · r10 = &OperationData · rdi = %7(team_plan 자리) · rax = &DebugFrameData
//!        (BigPlan::sub_plan 스택 인자 [rsp+0xa0..0xc8] 를 프롤로그가 이 레지스터들에 실어 둔 상태)
//!     ⟹ 여기서 **내 링크사본**을 같은 인자로 불러 내 버퍼(72B)에 받고, sret 주소를 TLS 에 기억한다.
//!   B `0xcafdaa` `mov rax,rsi; add rsp,0x50`(3+4B) — 공유 에필로그. 16 arm 전부 + 콜드 tag16 경로가 여기로 온다.
//!     TLS 에 pending 이 있고 `rsi == 기억한 sret` 이면 **게임 sret vs 내 버퍼**를 SubPlan live 맵
//!     (`sweep20::enumlive_cmp_2_0` · structlive 자동 생성)으로 비교한다. 그 외 호출은 그냥 지나간다.
//!
//! 왜 두 핀인가 = arm 안에서 sret 이 채워지는 시점은 arm 끝(`mov [rsi], r9`)이고 그 뒤 곧장 에필로그로
//!   점프한다. A 에서는 입력만 있고 게임 출력이 아직 없다 ⟹ 출력을 B 에서 읽는다. arm 은 호출을 안 하므로
//!   A 와 B 사이에 같은 스레드의 재진입은 없다(패닉 = ud2 = 게임이 죽는 경로라 B 미도달 = pending 잔존 →
//!   다음 A 가 `STALE` 로 세고 덮어쓴다 · B 는 sret 주소 일치까지 확인하므로 남의 arm 출력을 대조하지 않는다).
//!
//! 스텁(손 어셈) = 15 GPR push + pushfq + xmm0~5 저장 → `and rsp,-16; sub rsp,0x20` → `call pin_x(ctx)` →
//!   복원 → **훔친 7B 원본 실행** → `jmp [rip+0]` 절대 복귀. 레지스터·플래그 전부 보존(A 직후 `cmp` 가 플래그를
//!   다시 만들지만 보존이 싸다). ctx 배치(rcx = pushfq 위치): [0]=rflags [8]=r15 … [40]=r11 [48]=r10 [56]=r9
//!   [64]=r8 [72]=rdi [80]=rsi [88]=rbp [96]=rbx [104]=rdx [112]=rcx [120]=rax.
//! ⚠전제: 훔친 7B 에 rip-상대·분기 없음(둘 다 실측) · 유일 진입 = 점프테이블/`jmp cafdaa`(테이블은 코드패치 무관).
//! ⚠stage-1 진입부 프로브(0xcaf9f0 +0..14)·#13 호출부 프로브(+0x237)와 **구간이 겹치지 않는다**.
#![allow(dead_code)]
use crate::{exe_base, readable, FlushInstructionCache, GetCurrentProcess, VirtualProtect};
use std::cell::Cell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

pub const RVA_A: usize = 0xcafa57;
pub const RVA_B: usize = 0xcafdaa;
const STOLEN_A: [u8; 7] = [0x49, 0x8b, 0x8b, 0x30, 0x09, 0x00, 0x00]; // mov rcx,[r11+0x930]
const STOLEN_B: [u8; 7] = [0x48, 0x89, 0xf0, 0x48, 0x83, 0xc4, 0x50]; // mov rax,rsi ; add rsp,0x50
const RWX: u32 = 0x40;
const TAG_ATTACK_NEXUS: u64 = 16;

extern "Rust" {
    /// `#02` — rlib 링크사본(llvm-nm `T`). IR m12.ll:34867 `(sret 72B, &AttackNexusPlan 16B, usize, &mut StdRng, &PlayerState, &OperationData, %6 nonnull readnone, &mut DebugFrameData)`.
    #[link_name = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12attack_nexusNtB2_15AttackNexusPlan8sub_plan"]
    fn my_2(sret: *mut u8, a1: *const u8, a2: i64, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: *const u8);
}

pub static CALLS: AtomicU64 = AtomicU64::new(0);   // A 진입(= attack_nexus arm 실행 횟수)
pub static CMP: AtomicU64 = AtomicU64::new(0);     // B 에서 대조한 횟수
pub static DIFF: AtomicU64 = AtomicU64::new(0);
pub static PAN: AtomicU64 = AtomicU64::new(0);     // 내 사본 패닉
pub static STALE: AtomicU64 = AtomicU64::new(0);   // A 진입 시 pending 잔존(직전 arm 이 B 미도달)
pub static BADTAG: AtomicU64 = AtomicU64::new(0);  // A 에서 rbx != 16 (핀이 엉뚱한 경로에서 발화 = 즉시 의심)
pub static B_CALLS: AtomicU64 = AtomicU64::new(0); // B 진입 전체(= BigPlan::sub_plan 반환 전체)
pub static STUB_A: AtomicUsize = AtomicUsize::new(0);
pub static STUB_B: AtomicUsize = AtomicUsize::new(0);
static FIRST: Mutex<Vec<String>> = Mutex::new(Vec::new());
const FIRST_MAX: usize = 16;

thread_local! {
    static PEND_SRET: Cell<usize> = const { Cell::new(0) };
    static MINE: core::cell::UnsafeCell<[u8; 72]> = const { core::cell::UnsafeCell::new([0u8; 72]) };
}

#[repr(C)]
struct Ctx { rflags: u64, r15: u64, r14: u64, r13: u64, r12: u64, r11: u64, r10: u64, r9: u64, r8: u64,
             rdi: u64, rsi: u64, rbp: u64, rbx: u64, rdx: u64, rcx: u64, rax: u64 }

unsafe extern "C" fn pin_a(ctx: *const Ctx) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let c = &*ctx;
        CALLS.fetch_add(1, Ordering::Relaxed);
        if c.rbx != TAG_ATTACK_NEXUS { BADTAG.fetch_add(1, Ordering::Relaxed); return; }
        if PEND_SRET.with(|p| p.get()) != 0 { STALE.fetch_add(1, Ordering::Relaxed); }
        let sret = c.rsi as usize;
        let selfp = (c.rdx as usize).wrapping_add(8);
        // 내 사본 — 입력은 전부 readonly/readnone(IR attr) 이라 스냅샷·되돌리기 없음.
        let r = catch_unwind(AssertUnwindSafe(|| {
            MINE.with(|m| {
                let mb = (*m.get()).as_mut_ptr();
                core::ptr::write_bytes(mb, 0xf0, 72);   // 게임 sret 의 잔재와 다른 값으로 채워 「비교 안 한 칸」이 우연히 같지 않게
                my_2(mb, selfp as *const u8, c.r8 as i64, c.r9 as *const u8, c.r11 as *const u8, c.r10 as *const u8,
                     c.rdi as *const u8, c.rax as *const u8);
            });
        }));
        match r {
            Ok(()) => PEND_SRET.with(|p| p.set(sret)),
            Err(_) => { PAN.fetch_add(1, Ordering::Relaxed); PEND_SRET.with(|p| p.set(0)); }
        }
    }));
}

unsafe extern "C" fn pin_b(ctx: *const Ctx) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        B_CALLS.fetch_add(1, Ordering::Relaxed);
        let c = &*ctx;
        let pend = PEND_SRET.with(|p| p.get());
        if pend == 0 { return; }
        if pend != c.rsi as usize { return; }   // 다른 arm 의 반환(같은 스레드의 다른 sret) — 대조 아님
        PEND_SRET.with(|p| p.set(0));
        let gb = pend;
        let mb = MINE.with(|m| (*m.get()).as_ptr() as usize);
        if !readable(gb, 72) { return; }
        CMP.fetch_add(1, Ordering::Relaxed);
        let gt = core::ptr::read_unaligned(gb as *const u64);
        let mt = core::ptr::read_unaligned(mb as *const u64);
        let d = if gt != mt { Some(format!("tag g={} m={}", gt, mt)) }
                else { crate::sweep20::enumlive_cmp_2_0(gt, gb, mb).map(|x| format!("(tag {}){}", gt, x)) };
        if let Some(d) = d {
            DIFF.fetch_add(1, Ordering::Relaxed);
            let mut g = FIRST.lock().unwrap_or_else(|e| e.into_inner());
            if g.len() < FIRST_MAX {
                let gs: Vec<String> = (0..72).map(|i| format!("{:02x}", *((gb + i) as *const u8))).collect();
                let ms: Vec<String> = (0..72).map(|i| format!("{:02x}", *((mb + i) as *const u8))).collect();
                g.push(format!("#02 {} | g={} | m={}", d, gs.join(""), ms.join("")));
            }
        }
    }));
}

/// 스텁 기계어 — `pin` 을 ctx 포인터로 부르고 훔친 바이트를 실행한 뒤 `ret_abs` 로 절대 점프.
fn build_stub(pin: usize, stolen: &[u8; 7], ret_abs: usize) -> Vec<u8> {
    let mut s: Vec<u8> = Vec::with_capacity(160);
    // push rax,rcx,rdx,rbx,rbp,rsi,rdi,r8..r15
    s.extend_from_slice(&[0x50, 0x51, 0x52, 0x53, 0x55, 0x56, 0x57,
                          0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, 0x41, 0x54, 0x41, 0x55, 0x41, 0x56, 0x41, 0x57]);
    s.push(0x9c);                                   // pushfq
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x60]); // sub rsp,0x60
    // movdqu [rsp+k*16], xmmk  (k=0..5)
    for (k, m) in [(0u8, 0x04u8), (1, 0x4c), (2, 0x54), (3, 0x5c), (4, 0x64), (5, 0x6c)] {
        s.extend_from_slice(&[0xf3, 0x0f, 0x7f, m, 0x24]);
        if k > 0 { s.push(k * 0x10); }
    }
    s.extend_from_slice(&[0x48, 0x8d, 0x4c, 0x24, 0x60]); // lea rcx,[rsp+0x60]  (= ctx: pushfq 위치)
    s.extend_from_slice(&[0x48, 0x89, 0xe5]);             // mov rbp,rsp
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]);       // and rsp,-16
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]);       // sub rsp,0x20 (shadow)
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&pin.to_le_bytes()); // movabs rax,pin
    s.extend_from_slice(&[0xff, 0xd0]);                   // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xec]);             // mov rsp,rbp
    for (k, m) in [(0u8, 0x04u8), (1, 0x4c), (2, 0x54), (3, 0x5c), (4, 0x64), (5, 0x6c)] {
        s.extend_from_slice(&[0xf3, 0x0f, 0x6f, m, 0x24]); // movdqu xmmk,[rsp+k*16]
        if k > 0 { s.push(k * 0x10); }
    }
    s.extend_from_slice(&[0x48, 0x83, 0xc4, 0x60]); // add rsp,0x60
    s.push(0x9d);                                   // popfq
    // pop r15..r8, rdi, rsi, rbp, rbx, rdx, rcx, rax
    s.extend_from_slice(&[0x41, 0x5f, 0x41, 0x5e, 0x41, 0x5d, 0x41, 0x5c, 0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58,
                          0x5f, 0x5e, 0x5d, 0x5b, 0x5a, 0x59, 0x58]);
    s.extend_from_slice(stolen);
    s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); // jmp [rip+0]
    s.extend_from_slice(&ret_abs.to_le_bytes());
    s
}

unsafe fn install_one(rva: usize, stolen: &[u8; 7], pin: usize, slot: &AtomicUsize) -> Result<usize, &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("exe_base 0"); }
    let site = mbase.wrapping_add(rva);
    if !readable(site, 16) { return Err("사이트 읽기불가"); }
    for i in 0..7 { if *((site + i) as *const u8) != stolen[i] { return Err("훔칠 7B 불일치(스테일 RVA/패치판) — 미설치"); } }
    let stub = crate::probe::alloc_near(&[site], 256, rva);
    if stub == 0 { return Err("근접 할당 실패(rel32 사거리)"); }
    let code = build_stub(pin, stolen, site + 7);
    core::ptr::copy_nonoverlapping(code.as_ptr(), stub as *mut u8, code.len());
    FlushInstructionCache(GetCurrentProcess(), stub, code.len());
    slot.store(stub, Ordering::SeqCst);
    // 사이트 = E9 rel32 + 90 90
    let rel = (stub as i64) - (site as i64 + 5);
    if rel > 0x7fff_0000 || rel < -0x7fff_0000 { return Err("rel32 사거리 밖"); }
    let mut patch = [0x90u8; 7];
    patch[0] = 0xe9;
    patch[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(site, 7, RWX, &mut old) == 0 { return Err("VirtualProtect 실패"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), site as *mut u8, 7);
    VirtualProtect(site, 7, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), site, 7);
    Ok(stub)
}

/// 두 핀 설치. **B 를 먼저** 건다(A 가 먼저 살아 pending 을 만들었는데 B 가 없으면 STALE 만 쌓인다 — 무해하지만 지저분).
pub unsafe fn install(log: &mut String) -> bool {
    let b = install_one(RVA_B, &STOLEN_B, pin_b as usize, &STUB_B);
    match &b { Ok(s) => log.push_str(&format!("[pin02] B 에필로그 @{:#x} OK 스텁 {:#x}\n", RVA_B, s)),
               Err(e) => { log.push_str(&format!("[pin02] B 에필로그 @{:#x} 실패: {}\n", RVA_B, e)); return false; } }
    let a = install_one(RVA_A, &STOLEN_A, pin_a as usize, &STUB_A);
    match &a { Ok(s) => log.push_str(&format!("[pin02] A arm @{:#x} OK 스텁 {:#x}\n", RVA_A, s)),
               Err(e) => log.push_str(&format!("[pin02] A arm @{:#x} 실패: {}\n", RVA_A, e)) }
    a.is_ok()
}

pub fn report() -> String {
    let mut s = String::new();
    s.push_str("\n--- #02 AttackNexusPlan::sub_plan midpin (BigPlan::sub_plan 0xcaf9f0 의 idx14 arm 0xcafa57 → 공유 에필로그 0xcafdaa)\n");
    s.push_str(&format!("    설치: A {:#x} · B {:#x}\n", STUB_A.load(Ordering::Relaxed), STUB_B.load(Ordering::Relaxed)));
    s.push_str(&format!("    arm 진입 {} · 대조 {} · DIFF {} · panic {} · stale {} · badtag {} · 에필로그 통과(전 variant) {}\n",
        CALLS.load(Ordering::Relaxed), CMP.load(Ordering::Relaxed), DIFF.load(Ordering::Relaxed), PAN.load(Ordering::Relaxed),
        STALE.load(Ordering::Relaxed), BADTAG.load(Ordering::Relaxed), B_CALLS.load(Ordering::Relaxed)));
    s.push_str("    (arm 진입 = attack_nexus 인라인 본문이 실제 실행된 횟수 · 대조 = 그 sret 을 에필로그에서 내 사본과 비교한 횟수 · badtag/stale 은 0 이어야)\n");
    let g = FIRST.lock().unwrap_or_else(|e| e.into_inner());
    if !g.is_empty() {
        s.push_str(&format!("    첫 DIFF 덤프 {}건:\n", g.len()));
        for l in g.iter() { s.push_str("      "); s.push_str(l); s.push('\n'); }
    }
    s
}
