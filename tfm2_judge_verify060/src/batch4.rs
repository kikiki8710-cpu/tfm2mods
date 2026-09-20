//! batch4 — 2단계 재현체 배치 4(2026-09-20).
//!   #244 SmallActionRecall::is_end(fdb0e0 · vt tick 만) · #44 LegacyPlanHandler::take_misunderstood_received_chat(d59280 · `&mut self` = 첫 **상태 변이** 함수 —
//!   SELF_DIFF 기구: 호출 전 Vec 스냅샷 → 원본 실행 → 스냅샷 위에서 재현 실행 → 반환 + 사후 버퍼(len·원소) 비교).
#![allow(non_snake_case, dead_code)]
use crate::sweep060::{note, pop, r32, r64, r8, top, SW};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::Ordering;

type Fn1 = unsafe extern "C" fn(usize) -> u64;

// ═══════════════════════════════════════════════════════════════════════════
// #244 SmallActionRecall::is_end — 0.6.0 fdb0e0 · ABI (rcx=&self, rdx=&mut StdRng(미사용), r8=version, r9=&PlayerState, [rsp+0x28]=&OperationData) -> al
//   self+0x48 start_tick · +0x50/+0x58 goal xy · +0x60 end_delay · MapDef(ctx+0x20)+0x6d70+team*32 = 우물 사각형 {lx +0, ly +8, rx +0x10, ry +0x18}
// ═══════════════════════════════════════════════════════════════════════════
unsafe fn my_is_end(me: usize, version: u64, player: usize, data: usize) -> bool {
    let team = r64(player, 0xa00); if team >= 2 { panic!("bounds"); }
    let pos = r32(player, 0xa90) as usize; let cache = r64(data, 0) as usize;
    let champ = r64(cache, 0x1e0 + team as usize * 40 + pos * 8) as usize; if champ == 0 { panic!("unwrap None"); }
    let mut inf = false;
    if version >= 2 {
        let map = r64(r64(data, 8) as usize, 0x20) as usize; let f = map + 0x6d70 + team as usize * 32;
        let x = r64(champ, 0x660);
        if x >= r64(f, 0) && x <= r64(f, 0x10) { let y = r64(champ, 0x668); inf = y >= r64(f, 8) && y <= r64(f, 0x18); }
    }
    let game = r64(cache, 0) as usize; let vt = r64(cache, 8) as usize; let tick: Fn1 = core::mem::transmute(r64(vt, 0x28) as usize);
    let now = tick(game);
    if now >= r64(me, 0x60).wrapping_add(r64(me, 0x48)) { return true; }
    let dx = r64(champ, 0x660).abs_diff(r64(me, 0x50)); let dy = r64(champ, 0x668).abs_diff(r64(me, 0x58));
    if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < 100000001 { return true; }
    inf
}
pub unsafe extern "C" fn w_is_end(a0: usize, a1: usize, a2: u64, a3: usize, a4: usize) -> bool {
    const I: usize = 11;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, usize, u64, usize, usize) -> u64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2, a3, a4) & 0xff != 0;
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_is_end(a0, a2, a3, a4))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | self={:#x} ver={} team={} start={} delay={} goal=({},{})", g, m, a0, a2, r64(a3, 0xa00), r64(a0, 0x48), r64(a0, 0x60), r64(a0, 0x50), r64(a0, 0x58))); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}

// ═══════════════════════════════════════════════════════════════════════════
// #44 LegacyPlanHandler::take_misunderstood_received_chat — 0.6.0 d59280 · ABI (rcx=&mut self, rdx=tick, r8d=from i32, r9=&Chat(24B)) -> al
//   self+0x1188 Vec.ptr · +0x1190 Vec.len (원소 40B: tick +0 · from i32 +8 · chat 태그 +0x10 · 페이로드 +0x11/+0x12/+0x13 바이트 · +0x14 u32 · +0x18 u64 · +0x20 u64)
//   Chat 인자: 태그 +0 · +1/+2/+3 바이트 · +4 u32 · +8 u64 · +0x10 u64. derive(PartialEq) = 태그별 페이로드 비교(점프표 57 · 0.6.0 @0x3a7464c 디코드):
// ═══════════════════════════════════════════════════════════════════════════
/// 태그 → 비교 종류: 0 q8 · 1 d4 · 2 b1+q8 · 3 q8+q10 · 4 b1 · 5 b1+b2 · 6 b1+b2+b3 · 7 즉시일치
static CHAT_EQ: [u8; 57] = [0, 1, 2, 3, 3, 2, 3, 4, 2, 2, 2, 2, 2, 2, 2, 0, 0, 4, 2, 0, 2, 2, 2, 0, 3, 0, 0, 0, 0, 0, 0, 4, 4, 3, 0, 0, 0, 0, 0, 0, 7, 4, 2, 0, 2, 2, 2, 5, 5, 6, 5, 2, 2, 2, 2, 5, 2];
unsafe fn chat_eq(elem: usize, chat: usize) -> bool {
    let tag = r8(chat, 0) as usize;
    if r8(elem, 0x10) as usize != tag { return false; }
    let kind = if tag < 57 { CHAT_EQ[tag] } else { 0xff };
    let b = |k: usize| r8(elem, 0x10 + k) == r8(chat, k);
    match kind {
        0 => r64(elem, 0x18) == r64(chat, 8),
        1 => r32(elem, 0x14) == r32(chat, 4),
        2 => b(1) && r64(elem, 0x18) == r64(chat, 8),
        3 => r64(elem, 0x18) == r64(chat, 8) && r64(elem, 0x20) == r64(chat, 0x10),
        4 => b(1),
        5 => b(1) && b(2),
        6 => b(1) && b(2) && b(3),
        7 => true,
        _ => panic!("chat tag {} 범위 밖", tag),
    }
}
const ELEM: usize = 40; const SNAP_MAX: usize = 64;
/// 스냅샷(buf[0..len*40]) 위에서 재현: 일치 index 를 찾으면 swap_remove 를 스냅샷에 적용하고 (true, new_len) 반환
unsafe fn my_take(buf: &mut [u8], len: usize, tick: u64, from: u32, chat: usize) -> (bool, usize) {
    let base = buf.as_ptr() as usize;
    for i in 0..len {
        let e = base + i * ELEM;
        if r64(e, 0) != tick || r32(e, 8) != from || !chat_eq(e, chat) { continue; }
        let last = len - 1;
        let src = buf[last * ELEM..last * ELEM + ELEM].to_vec();
        buf[i * ELEM..i * ELEM + ELEM].copy_from_slice(&src);
        return (true, last);
    }
    (false, len)
}
pub unsafe extern "C" fn w_take_misunderstood(a0: usize, a1: u64, a2: u32, a3: usize) -> bool {
    const I: usize = 12;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, u64, u32, usize) -> u64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    // 호출 전 스냅샷(len ≤ 64 일 때만 비교)
    let len0 = r64(a0, 0x1190) as usize; let ptr0 = r64(a0, 0x1188) as usize;
    let mut snap = [0u8; ELEM * SNAP_MAX]; let ok = t && len0 <= SNAP_MAX;
    if ok && len0 > 0 { core::ptr::copy_nonoverlapping(ptr0 as *const u8, snap.as_mut_ptr(), len0 * ELEM); }
    let g = f(a0, a1, a2, a3) & 0xff != 0;
    if ok {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_take(&mut snap[..], len0, a1, a2, a3))) {
            Ok((m, mlen)) => {
                let glen = r64(a0, 0x1190) as usize; let gptr = r64(a0, 0x1188) as usize;
                let mut sd = None;
                if m == g && mlen == glen && glen <= SNAP_MAX {
                    for k in 0..glen * ELEM { if *((gptr + k) as *const u8) != snap[k] { sd = Some(k); break; } }
                }
                if m != g || mlen != glen || sd.is_some() { note(I, format!("g={} m={} len {}→g{} m{} state_diff@{:?} | tick={} from={} chat.tag={}", g, m, len0, glen, mlen, sd, a1, a2, r8(a3, 0))); }
            }
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}
