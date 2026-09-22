//! helpers — game_core 순수 헬퍼 재현체(진입부에 분기가 있어 12B 훅이 불가한 것은 **호출자 wrap 안에서** 원본 주소를 직접 불러 대조한다).
//!   is_near_line(19903f0 · 606B · `push rsi; test r9b,r9b; je` 라 진입부 훅 불가) — map_regions::is_near_line(ctx, x, y, line)
#![allow(dead_code)]
use crate::sweep060::r64;

#[inline(always)] fn umax(a: u64, b: u64) -> u64 { if a > b { a } else { b } }
#[inline(always)] fn umin(a: u64, b: u64) -> u64 { if a < b { a } else { b } }

/// 0.6.0 19903f0 을 분기 단위로 옮긴 것. setting = ctx+8 · width +0x12b8 · height +0x12c0 · c = height − y(wrapping)
pub unsafe fn my_is_near_line(ctx: usize, x: u64, y: u64, line: u8) -> bool {
    // ★레지스터: rdx=x · r8=y (첫 판은 둘을 바꿔 읽어 Top/Bottom 이 314/14766 갈렸다 · Mid 는 맞았다)
    let s = r64(ctx, 8) as usize; let height = r64(s, 0x12c0); let width = r64(s, 0x12b8);
    let c = height.wrapping_sub(y);
    if umax(c, x) < 0x2ee01 { return true; }                          // 코너(원점 쪽)
    if umin(c, x) >= width.wrapping_sub(0x2ee00) { return true; }     // 코너(반대쪽)
    if line == 1 { return x.abs_diff(c) < 0xfa00; }                   // Mid = 대각선 |x−c| < 64000
    let w2 = width.wrapping_sub(0xabe00) >> 1;
    let out_x = x < w2 || x > w2.wrapping_add(0xabe00);
    let inner = !out_x && { let h2 = height.wrapping_sub(0xabe00) >> 1; !(y < h2 || y > h2.wrapping_add(0xabe00)) };
    if line == 0 {                                                    // Top
        if !inner {                                                   // 19904e9
            if c < x || c < 0x2ee01 { return false; }
            return c.wrapping_sub(x) >= 0xfa00;
        }
        let d = c.abs_diff(x);                                        // 19905c0 (y 는 안 쓴다 · 전부 x 대 c)
        if c < 0x2ee01 || c < x || d > 0x176ff { return false; }
        return c.wrapping_sub(x) >= 0xfa00;
    }
    if !inner {                                                       // Bottom · 199059b
        if x < 0x2ee01 || x < c { return false; }
        return x.wrapping_sub(c) >= 0xfa00;
    }
    let d = c.abs_diff(x);                                            // 199060d
    if c > x || x < 0x2ee01 || d > 0x176ff { return false; }
    x.wrapping_sub(c) >= 0xfa00
}

// ── 헬퍼 자가검증(호출자 wrap 에서 실좌표로 원본 19903f0 vs 재현 대조) ──
use std::sync::atomic::{AtomicU64, Ordering};
pub static H_CALLS: AtomicU64 = AtomicU64::new(0);
pub static H_DIFF: AtomicU64 = AtomicU64::new(0);
pub static H_NOTE: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
pub const IS_NEAR_LINE_RVA: usize = 0x19903f0;
type FnNear = unsafe extern "C" fn(usize, u64, u64, u8) -> u64;
pub unsafe fn selftest_is_near_line(ctx: usize, x: u64, y: u64) {
    let f: FnNear = core::mem::transmute(crate::BASE.load(Ordering::Relaxed) + IS_NEAR_LINE_RVA);
    for line in 0..3u8 {
        let g = f(ctx, x, y, line) & 0xff != 0;
        let m = my_is_near_line(ctx, x, y, line);
        H_CALLS.fetch_add(1, Ordering::Relaxed);
        if g != m {
            let n = H_DIFF.fetch_add(1, Ordering::Relaxed);
            if n < 32 { let mut v = H_NOTE.lock().unwrap_or_else(|e| e.into_inner()); v.push(format!("[hdiff] is_near_line g={} m={} | x={} y={} line={} w={} h={}", g, m, x, y, line, r64(r64(ctx, 8) as usize, 0x12b8), r64(r64(ctx, 8) as usize, 0x12c0))); }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// #34 has_line_defense_threat — 0.6.0 e768f0 · ABI (rcx=&PlayerState, rdx=&OperationData, r8b=line, r9=tower_id) -> al
//   bb = data+0x10 + team*0x5c8 · minion_state = bb+{0(Top),0x28(Mid),0x50(그 외)} · from_mid +0x10(i64) · minion_count +0x20(i32)
//   cache.minions(1-team) = cache + (1-team)*32 + {0x10/0x28 top, 0x50/0x68 mid, 0x90/0xa8 bottom}(ptr/len) 순서 연결(151bce0 · bump Vec 은 안 만든다)
//   any(is_near_line(ctx, m.x, m.y, line) && m.ty(+0x68)==1 && m.nearest_enemy 태그(+0x88)==1 && 값(+0x90)==tower_id)
// ═══════════════════════════════════════════════════════════════════════════
use crate::sweep060::{note, pop, r32, top, SW};
use std::panic::{catch_unwind, AssertUnwindSafe};
pub unsafe fn my_has_line_defense_threat(player: usize, data: usize, line: u8, tower: u64) -> bool {
    let team = r64(player, 0xa00); if team > 1 { panic!("bounds"); }
    let ms = r64(data, 0x10) as usize + team as usize * 0x5c8 + match line { 0 => 0usize, 1 => 0x28, _ => 0x50 };
    let pushed = (r64(ms, 0x10) as i64) < -3000 || (r32(ms, 0x20) as i32) < -2;
    if !pushed { return false; }
    let cache = r64(data, 0) as usize; let ctx = r64(data, 8) as usize; let enemy = (1 - team) as usize;
    for base in [0x10usize, 0x50, 0x90] {
        let lst = cache + enemy * 32 + base; let (p, n) = (r64(lst, 0) as usize, r64(lst, 0x18) as usize);
        for k in 0..n {
            let m = r64(p, k * 8) as usize;
            if !my_is_near_line(ctx, r64(m, 0x660), r64(m, 0x668), line) { continue; }
            if r32(m, 0x68) != 1 || r32(m, 0x88) != 1 || r64(m, 0x90) != tower { continue; }
            return true;
        }
    }
    false
}
pub unsafe extern "C" fn w_has_line_defense_threat(a0: usize, a1: usize, a2: u8, a3: u64) -> bool {
    const I: usize = 13;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, usize, u8, u64) -> u64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1, a2, a3) & 0xff != 0;
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_has_line_defense_threat(a0, a1, a2, a3))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | team={} line={} tower={} player={:#x} data={:#x}", g, m, r64(a0, 0xa00), a2, a3, a0, a1)); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}
