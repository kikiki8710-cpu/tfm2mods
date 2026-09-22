//! auto_swap — ★09-20 내 팀 자동 스왑(유저 요구 "밴픽 끝나고 자동 스왑 기능 없어").
//! 스왑 단계(phase 7) 진입 시 내 배치가 규칙 위반이면 `assign::best_order`(맞게 앉는 인원 최대 · 원본과 최대 일치)로 목표 순열을 구해
//! **게임 함수 `select_swap(scene, row)`(0.6.0 RVA 0x23ac540 · rcx=씬, rdx=행 인덱스)** 를 유저 클릭과 똑같이 두 번씩 호출해(전치 1회 = 호출 2회) 옮긴다.
//! 게임 자신의 핸들러라 표 UI 갱신·player 팀 판정·phase 검사 전부 게임이 한다(우리는 raw order 를 직접 쓰지 않는다).
//! 세트당 1회(키 = t1·t2·픽 목록 지문) — 이후 유저의 수동 스왑은 존중, 코치 위임은 게임이 다시 배치. 호출 전 검사: 프롤로그 12B · selected(씬+0)==None · sent(씬+0x466)==0.
//! 근거 RE = `REPORT\tfm2_champ_pos_lock\RE\2026-09-18_0.6.0-스왑단계-씬레이아웃-함수RVA.md`(select_swap 행).
use crate::draft_scene::SwapState;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

pub const RVA_SELECT_SWAP: usize = 0x2424490;
const PROL: [u8; 12] = [0x48, 0x83, 0xec, 0x28, 0x80, 0xb9, 0x69, 0x04, 0x00, 0x00, 0x00, 0x0f];
const OFF_SELECTED_TAG: usize = 0x0;
const OFF_SENT: usize = 0x466;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(n: *const u16) -> usize;
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

static DONE_KEY: AtomicU64 = AtomicU64::new(0);   // 이번 세트에서 이미 시도한 지문
static PROL_STATE: AtomicU8 = AtomicU8::new(0);   // 0 미검사 1 OK 2 불일치(영구 skip)
pub static CNT_APPLIED: AtomicU64 = AtomicU64::new(0);

fn set_key(st: &SwapState) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    (st.t1_id, st.t2_id, &st.pick1, &st.pick2).hash(&mut h);
    h.finish() | 1
}

/// 이미 이번 세트에 시도했나.
pub fn attempted(st: &SwapState) -> bool { DONE_KEY.load(Ordering::Relaxed) == set_key(st) }

/// 스왑 게이트가 "위반" 판정을 냈을 때 호출. 세트당 1회만 실행. 반환 = 로그 한 줄(무동작이면 None).
pub fn try_auto(st: &SwapState, my_side: usize) -> Option<String> {
    let key = set_key(st);
    if DONE_KEY.load(Ordering::Relaxed) == key { return None; }
    let (picks, cur) = if my_side == 0 { (&st.pick1, &st.order_a) } else { (&st.pick2, &st.order_b) };
    if cur.len() < 2 || cur.len() != picks.len() { return None; }
    // 순열 검증
    let mut seen = vec![false; cur.len()];
    for &x in cur { if x as usize >= cur.len() || seen[x as usize] { return None; } seen[x as usize] = true; }
    let masks: Vec<u8> = picks.iter().map(|n| crate::mask_of(n)).collect();
    let (want, best_n) = crate::assign::best_order(&masks, cur);
    let cur_n = crate::assign::order_matched(&masks, cur);
    if best_n <= cur_n || want == *cur { DONE_KEY.store(key, Ordering::Relaxed); return Some(format!("autoswap: 개선 불가(맞춤 {}/{} 최적 {}) — 무동작", cur_n, cur.len(), best_n)); }
    let Some(scene) = crate::draft_scene::scene() else { return None };
    unsafe {
        if !readable(scene, 0x478) { return None; }
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { return None; }
        let f = base + RVA_SELECT_SWAP;
        if PROL_STATE.load(Ordering::Relaxed) == 0 {
            let ok = readable(f, 12) && core::slice::from_raw_parts(f as *const u8, 12) == PROL;
            PROL_STATE.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
            if !ok { DONE_KEY.store(key, Ordering::Relaxed); return Some("autoswap: select_swap 프롤로그 불일치(RVA stale?) — 자동 스왑 비활성".into()); }
        }
        if PROL_STATE.load(Ordering::Relaxed) != 1 { return None; }
        // 유저가 행 하나를 선택해 둔 상태(selected=Some)면 이번 프레임은 건너뛴다(다음 프레임 재시도) · 이미 확정 전송됐으면 포기
        if core::ptr::read_unaligned((scene + OFF_SELECTED_TAG) as *const u64) != 0 { return None; }
        if core::ptr::read_unaligned((scene + OFF_SENT) as *const u8) != 0 { DONE_KEY.store(key, Ordering::Relaxed); return None; }
        DONE_KEY.store(key, Ordering::Relaxed);
        let select_swap: extern "C" fn(usize, usize) = core::mem::transmute(f);
        // cur → want 를 전치 열로: 포지션 p 의 목표 픽 want[p] 가 지금 q 자리에 있으면 (p,q) 교환
        let mut work = cur.clone();
        let mut swaps = 0usize;
        for p in 0..work.len() {
            if work[p] == want[p] { continue; }
            let Some(q) = (p + 1..work.len()).find(|&q| work[q] == want[p]) else { continue };
            let r = std::panic::catch_unwind(|| { select_swap(scene, p); select_swap(scene, q); });
            if r.is_err() { return Some(format!("autoswap: select_swap 패닉 @({},{}) — 중단", p, q)); }
            work.swap(p, q);
            swaps += 1;
        }
        CNT_APPLIED.fetch_add(1, Ordering::Relaxed);
        Some(format!("autoswap: {:?} -> {:?} (맞춤 {}→{}/{} · 전치 {}회 · side={})", cur, want, cur_n, best_n, cur.len(), swaps, my_side))
    }
}
