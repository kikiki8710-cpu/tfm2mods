//! helpers — game_core 순수 헬퍼 재현체(진입부에 분기가 있어 12B 훅이 불가한 것은 **호출자 wrap 안에서** 원본 주소를 직접 불러 대조한다).
//!   is_near_line(19903f0 · 606B · `push rsi; test r9b,r9b; je` 라 진입부 훅 불가) — map_regions::is_near_line(ctx, x, y, line)
#![allow(dead_code)]
use crate::sweep060::r64;

#[inline(always)] fn umax(a: u64, b: u64) -> u64 { if a > b { a } else { b } }
#[inline(always)] fn umin(a: u64, b: u64) -> u64 { if a < b { a } else { b } }

/// 0.6.0 19903f0 을 분기 단위로 옮긴 것. setting = ctx+8 · width +0x12b8 · height +0x12c0 · c = height − y(wrapping)
pub unsafe fn my_is_near_line(ctx: usize, x: u64, y: u64, line: u8) -> bool {
    let s = r64(ctx, 8) as usize; let height = r64(s, 0x12c0); let width = r64(s, 0x12b8);
    let c = height.wrapping_sub(y);
    if line == 1 {                                                   // Mid
        if umax(c, x) < 0x2ee01 { return true; }
        if umin(c, x) >= width.wrapping_sub(0x2ee00) { return true; }
        return x.abs_diff(c) < 0xfa00;
    }
    if umax(c, x) < 0x2ee01 { return true; }                          // Top/Bottom 공통 머리
    if umin(c, x) >= width.wrapping_sub(0x2ee00) { return true; }
    let w2 = width.wrapping_sub(0xabe00) >> 1;
    let out_y = y < w2 || y > w2.wrapping_add(0xabe00);
    let path_b = if out_y { true } else {
        let h2 = height.wrapping_sub(0xabe00) >> 1;
        x < h2 || x > h2.wrapping_add(0xabe00)
    };
    if path_b {                                                       // 199059b
        if y < 0x2ee01 { return false; }
        if y < c { return false; }
        return y.wrapping_sub(c) >= 0xfa00;
    }
    if line == 0 {                                                    // 19905c0 (Top · 대각 위쪽 c ≥ y)
        let d = c.abs_diff(y);
        if c < 0x2ee01 { return false; }
        if c < y { return false; }
        if d > 0x176ff { return false; }
        return c.wrapping_sub(y) >= 0xfa00;
    }
    // 199060d (Bottom · 대각 아래쪽 c ≤ y)
    let d = c.abs_diff(y);
    if c > y { return false; }
    if y < 0x2ee01 { return false; }
    if d > 0x176ff { return false; }
    y.wrapping_sub(c) >= 0xfa00
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
