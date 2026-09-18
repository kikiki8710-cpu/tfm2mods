//! draft_scene_stable — 0.6.0 밴픽 씬(`MatchUIRunner`) 포인터 캡처 + 스왑 단계 order/픽 raw 읽기 (stable 껍데기 모드 공용).
//! 근거 RE: `mods_report/tfm2_champ_pos_lock/RE/2026-09-18_0.6.0-스왑단계-씬레이아웃-함수RVA.md`
//!   · 씬 포인터 = `MatchUIRunner::update`(RVA 0x2452e50, vtable 호출·매프레임) 진입 **rdx**(rcx 는 sret). 진입 12B = push×8(경계 정확).
//!   · 씬(size 0x478): pick1 Vec<String> 0x168 / pick2 0x180 / **order A(블루) Vec<u64> 0x198** / **order B(레드) 0x1b0** — `order[포지션] = 픽 인덱스`
//!     / phase u32 0x3a8(**7 = 스왑**, 8 = 다음) / T1(블루) team id 0x3f8 / 0x400 = T2 id(추정) / 0x466 = SwapDone 전송됨 / rule 0xce(픽 수 = rule+2).
//!   · 유저 클릭(select_swap)·코치 위임(apply_coach_swap_order — **Vec 버퍼 교체**라 ptr 매프레임 재독)·최종 배정(run_apply_swap) 전부 두 order 배열에 반영.
//! 안전: 읽기는 전부 VirtualQuery 검사(`readable`) 경유 · 캡처 함수 안에서는 원자 저장만(format!/lock 금지) · 포인터 신선도 = `tick()` 이 히트 증가를 못 보면 stale 처리.
//! 멀티모드: 같은 함수를 다른 모드(pos_lock·view_plus)가 먼저 후킹했으면 **체인**(진입 12B 가 `48 b8 <tgt> ff e0` 이면 그 12B 를 스텁 꼬리에 담아 tgt 로 점프). 재체인 금지(1회 설치 확정).
//! 사용: `#[path = r"C:\tfm2mods\ui_kit\draft_scene_stable.rs"] mod draft_scene;` → 매프레임 `draft_scene::tick()` → `draft_scene::read()`.
#![allow(dead_code)]
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub const GAME_VER: &str = "0.6.0";
pub const RVA_UPDATE: usize = 0x2452e50;
pub const PROL_UPDATE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
pub const OFF_RULE: usize = 0xce;
pub const OFF_PICK1: usize = 0x168;
pub const OFF_PICK2: usize = 0x180;
pub const OFF_ORDER_A: usize = 0x198;
pub const OFF_ORDER_B: usize = 0x1b0;
pub const OFF_PHASE: usize = 0x3a8;
pub const OFF_T1_ID: usize = 0x3f8;
pub const OFF_T2_ID: usize = 0x400;
pub const OFF_SENT: usize = 0x466;
pub const PHASE_SWAP: u32 = 7;
/// 히트가 이만큼의 tick 동안 안 늘면 씬 포인터를 stale 로 본다(씬 해제 후 재사용 페이지 오독 방지).
const STALE_TICKS: u64 = 30;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(n: *const u16) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
}
#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32, region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || addr >= (1usize << 48) || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02 | 0x04 | 0x20 | 0x40; const GUARD: u32 = 0x01 | 0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr.wrapping_add(len) <= mbi.base.wrapping_add(mbi.region_size)
}
unsafe fn rd_u64(a: usize) -> Option<u64> { if readable(a, 8) { Some(core::ptr::read_unaligned(a as *const u64)) } else { None } }
unsafe fn rd_u32(a: usize) -> Option<u32> { if readable(a, 4) { Some(core::ptr::read_unaligned(a as *const u32)) } else { None } }
unsafe fn rd_u8(a: usize) -> Option<u8> { if readable(a, 1) { Some(core::ptr::read(a as *const u8)) } else { None } }

static SCENE: AtomicUsize = AtomicUsize::new(0);
static HITS: AtomicU64 = AtomicU64::new(0);
static LAST_HITS: AtomicU64 = AtomicU64::new(0);
static IDLE_TICKS: AtomicU64 = AtomicU64::new(0);
/// 0 미설치 · 1 설치 · 2 영구 실패
pub static INSTALL_STATE: AtomicUsize = AtomicUsize::new(0);
pub static CHAINED: AtomicUsize = AtomicUsize::new(0);

// saved 레이아웃(스텁 push 순서 r12 rsi rdi rbx r11 r10 r9 r8 rdx rcx): +0 rcx +8 rdx …
unsafe extern "C" fn cap_update(saved: *const usize) {
    let rdx = *saved.add(1);
    SCENE.store(rdx, Ordering::Relaxed);
    HITS.fetch_add(1, Ordering::Relaxed);
}

/// 캡처 detour: 진입 12B 를 `movabs rax,stub; jmp rax` 로 덮고, 스텁 = push 10 regs → cap_fn(saved) → pop → 원본 12B(또는 외부 훅 12B) → `jmp [rip+0]` fn+12(또는 외부 스텁).
/// ⚠복귀 점프는 `jmp qword [rip+0]`(rax 무클로버 — 09-17 교훈: 원본 12B 가 rax 를 정의하면 `movabs rax` 복귀는 hang).
unsafe fn install_capture(rva: usize, prologue: &[u8; 12], cap_fn: usize) -> Result<(usize, bool), &'static str> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return Err("module 0"); }
    let fn_addr = base + rva;
    if !readable(fn_addr, 12) { return Err("entry unreadable"); }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != *prologue { return Err("prologue mismatch"); }
    let resume: usize = if chained { usize::from_le_bytes(cur[2..10].try_into().unwrap()) } else { fn_addr + 12 };
    let stub = VirtualAlloc(0, 256, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = Vec::with_capacity(128);
    s.extend_from_slice(&[0x41, 0x54, 0x56, 0x57, 0x53, 0x41, 0x53, 0x41, 0x52, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51]); // push r12 rsi rdi rbx r11 r10 r9 r8 rdx rcx
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);       // mov rcx, rsp
    s.extend_from_slice(&[0x48, 0x89, 0xe3]);       // mov rbx, rsp
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]); // and rsp, -16
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]); // sub rsp, 0x20
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // movabs rax, cap_fn
    s.extend_from_slice(&[0xff, 0xd0]);             // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xdc]);       // mov rsp, rbx
    s.extend_from_slice(&[0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x41, 0x5b, 0x5b, 0x5f, 0x5e, 0x41, 0x5c]); // pop 역순
    if chained {
        // 외부 훅의 `movabs rax,tgt; jmp rax` 12B 를 그대로 실행 = 외부 스텁으로 점프(체인)
        s.extend_from_slice(&cur);
    } else {
        s.extend_from_slice(prologue);                  // 원본 12B(push 들)
        s.extend_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); // jmp qword [rip+0]
        s.extend_from_slice(&resume.to_le_bytes());
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = [0x90u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok((stub, chained))
}

/// 1회 설치(실패는 영구 — RVA stale 이면 클릭 추적 폴백으로). 반환 = 사람이 읽을 결과 문자열(로그용).
pub fn install_once() -> Option<String> {
    if INSTALL_STATE.load(Ordering::Relaxed) != 0 { return None; }
    let r = unsafe { install_capture(RVA_UPDATE, &PROL_UPDATE, cap_update as unsafe extern "C" fn(*const usize) as *const () as usize) };
    Some(match r {
        Ok((stub, chained)) => { INSTALL_STATE.store(1, Ordering::Relaxed); CHAINED.store(chained as usize, Ordering::Relaxed); format!("draft_scene: update 캡처 설치 OK rva={:#x} stub={:#x} chained={}", RVA_UPDATE, stub, chained) }
        Err(e) => { INSTALL_STATE.store(2, Ordering::Relaxed); format!("draft_scene: update 캡처 설치 실패: {e} (RVA stale?) — raw 스왑 읽기 없이 진행") }
    })
}

/// 매프레임(post_update) 호출 — 히트 증가 여부로 씬 포인터 신선도 유지.
pub fn tick() {
    let h = HITS.load(Ordering::Relaxed);
    if h != LAST_HITS.swap(h, Ordering::Relaxed) { IDLE_TICKS.store(0, Ordering::Relaxed); } else { IDLE_TICKS.fetch_add(1, Ordering::Relaxed); }
}
pub fn hits() -> u64 { HITS.load(Ordering::Relaxed) }
/// 지금 살아 있는(최근 히트) 씬 포인터.
pub fn scene() -> Option<usize> {
    if INSTALL_STATE.load(Ordering::Relaxed) != 1 { return None; }
    if IDLE_TICKS.load(Ordering::Relaxed) > STALE_TICKS { return None; }
    let s = SCENE.load(Ordering::Relaxed);
    if s < 0x10000 || s >= (1usize << 48) { None } else { Some(s) }
}

#[derive(Debug, Clone, Default)]
pub struct SwapState {
    pub phase: u32,
    pub rule: u8,
    pub sent: u8,
    pub t1_id: u64,
    pub t2_id: u64,
    pub pick1: Vec<String>,
    pub pick2: Vec<String>,
    pub order_a: Vec<u64>,
    pub order_b: Vec<u64>,
}
impl SwapState {
    pub fn is_swap(&self) -> bool { self.phase == PHASE_SWAP }
    /// 진영(0 블루 1 레드)의 포지션 p → 챔피언 id. order 가 비었으면(픽 미완) None.
    pub fn champ_at(&self, side: usize, p: usize) -> Option<&str> {
        let (picks, order) = if side == 0 { (&self.pick1, &self.order_a) } else { (&self.pick2, &self.order_b) };
        let i = *order.get(p)? as usize;
        picks.get(i).map(|s| s.as_str())
    }
    /// 진영의 포지션 순 챔피언 목록(길이 = order 길이, 못 푼 칸은 None).
    pub fn lineup(&self, side: usize) -> Vec<Option<String>> {
        let n = if side == 0 { self.order_a.len() } else { self.order_b.len() };
        (0..n).map(|p| self.champ_at(side, p).map(|s| s.to_string())).collect()
    }
}

unsafe fn read_vec_string(base: usize) -> Option<Vec<String>> {
    let ptr = rd_u64(base + 8)? as usize; let len = rd_u64(base + 0x10)? as usize;
    if len == 0 { return Some(Vec::new()); }
    if len > 16 || !readable(ptr, len * 0x18) { return None; }
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let e = ptr + i * 0x18;
        let sp = rd_u64(e + 8)? as usize; let sl = rd_u64(e + 0x10)? as usize;
        if sl == 0 || sl > 64 || !readable(sp, sl) { return None; }
        let bytes = core::slice::from_raw_parts(sp as *const u8, sl);
        out.push(String::from_utf8_lossy(bytes).into_owned());
    }
    Some(out)
}
unsafe fn read_vec_u64(base: usize) -> Option<Vec<u64>> {
    let ptr = rd_u64(base + 8)? as usize; let len = rd_u64(base + 0x10)? as usize;
    if len == 0 { return Some(Vec::new()); }
    if len > 8 || !readable(ptr, len * 8) { return None; }
    let mut v = Vec::with_capacity(len);
    for i in 0..len { let x = rd_u64(ptr + i * 8)?; if x >= 16 { return None; } v.push(x); }
    Some(v)
}

/// 씬 raw 읽기(어느 phase 든). 스왑 판정은 `is_swap()`. 형태 이상(길이 초과·비매핑)이면 None.
pub fn read() -> Option<SwapState> {
    let s = scene()?;
    unsafe {
        if !readable(s, 0x478) { return None; }
        let phase = rd_u32(s + OFF_PHASE)?;
        let rule = rd_u8(s + OFF_RULE)?;
        let sent = rd_u8(s + OFF_SENT)?;
        let t1_id = rd_u64(s + OFF_T1_ID)?;
        let t2_id = rd_u64(s + OFF_T2_ID)?;
        let pick1 = read_vec_string(s + OFF_PICK1)?;
        let pick2 = read_vec_string(s + OFF_PICK2)?;
        let order_a = read_vec_u64(s + OFF_ORDER_A)?;
        let order_b = read_vec_u64(s + OFF_ORDER_B)?;
        Some(SwapState { phase, rule, sent, t1_id, t2_id, pick1, pick2, order_a, order_b })
    }
}
