//! batch3 — 2단계 재현체 배치 3(2026-09-20).
//!   #132 champion_hp_value_uncached(de5780 · 콜리 0 · 명세 logic + 0.6.0 디스어셈 오프셋).
#![allow(non_snake_case, dead_code)]
use crate::sweep060::{note, pop, r64, top, SW};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::Ordering;

#[inline(always)] unsafe fn i64at(p: usize, off: usize) -> i64 { r64(p, off) as i64 }

// ═══════════════════════════════════════════════════════════════════════════
// #132 champion_hp_value_uncached — 0.6.0 de5780 · ABI (rcx=&ScoreParameter, rdx=&ChampionScoreParameter) -> rax i64
//   ScoreParameter: player.team +0x978 · player.attack_power +0x9d0 · util_power_base +0x9d8 · cc_time_x_inv_cd +0x9e0 · buff_inv_cd_count +0x9e8
//     near_allies {ptr +0x14b8, len +0x14d0} · near_enemies {ptr +0x14d8, len +0x14f0} · 원소 = ChampionScoreParameter(0xd8B)
//   ChampionScoreParameter: team +0x60 · attack_value +0xa8 · util_value +0xb0 · attack_power +0xb8 · util_power_base +0xc0 · cc_time_x_inv_cd +0xc8 · buff_inv_cd_count +0xd0
//   util(p, dps) = base + cc*dps/1000 + buff*dps/10000 (signed 절사) · rank_scale = clamp(middle*100/(n-1), 0, 100) → x<51 ? +50 : ×2
// ═══════════════════════════════════════════════════════════════════════════
#[inline(always)] unsafe fn util(base: i64, cc: i64, buff: i64, dps: i64) -> i64 {
    base.wrapping_add(cc.wrapping_mul(dps) / 1000).wrapping_add(buff.wrapping_mul(dps) / 10000)
}
fn rank_scale(vals: &[i64], target: i64) -> i64 {
    let n = vals.len(); if n < 2 { return 100; }
    let (mut lower, mut equal) = (0i64, 0u64);
    for &v in vals { if v < target { lower += 1; } if v == target { equal += 1; } }
    let middle = lower.wrapping_add((equal >> 1) as i64);
    let x = middle.wrapping_mul(100) / (n as i64 - 1);
    let c = if x > 0 { x } else { 0 }; let c = if c < 100 { c } else { 100 };
    if x < 51 { c + 50 } else { c * 2 }
}
unsafe fn my_champion_hp_value(p: usize, t: usize) -> i64 {
    let mut atk = [0i64; 10]; let mut ut = [0i64; 10];
    let me_team = r64(p, 0x978); let t_team = r64(t, 0x60);
    let me_atk = i64at(p, 0x9d0);
    let (mut same, mut opp) = (0i64, 0i64);
    atk[0] = me_atk; let mut n = 1usize;
    if me_team == t_team { same = same.wrapping_add(me_atk); } else { opp = opp.wrapping_add(me_atk); }
    let (ap, al) = (r64(p, 0x14b8) as usize, r64(p, 0x14d0) as usize);
    let (ep, el) = (r64(p, 0x14d8) as usize, r64(p, 0x14f0) as usize);
    for k in 0..al { if n >= 10 { break; } let e = ap + k * 0xd8; let a = i64at(e, 0xb8); atk[n] = a;
        if r64(e, 0x60) == t_team { same = same.wrapping_add(a); } else { opp = opp.wrapping_add(a); } n += 1; }
    for k in 0..el { if n >= 10 { break; } let e = ep + k * 0xd8; let a = i64at(e, 0xb8); atk[n] = a;
        if r64(e, 0x60) == t_team { same = same.wrapping_add(a); } else { opp = opp.wrapping_add(a); } n += 1; }
    let me_dps = if me_team == t_team { same } else { opp };
    ut[0] = util(i64at(p, 0x9d8), i64at(p, 0x9e0), i64at(p, 0x9e8), me_dps);
    let mut idx = 1usize;
    for k in 0..al { if idx >= n { break; } let e = ap + k * 0xd8; let d = if r64(e, 0x60) == t_team { same } else { opp };
        ut[idx] = util(i64at(e, 0xc0), i64at(e, 0xc8), i64at(e, 0xd0), d); idx += 1; }
    for k in 0..el { if idx >= n { break; } if idx >= 10 { panic!("bounds"); } let e = ep + k * 0xd8; let d = if r64(e, 0x60) == t_team { same } else { opp };
        ut[idx] = util(i64at(e, 0xc0), i64at(e, 0xc8), i64at(e, 0xd0), d); idx += 1; }
    let t_atk = i64at(t, 0xb8);
    let t_util = util(i64at(t, 0xc0), i64at(t, 0xc8), i64at(t, 0xd0), same);
    if n > 10 { panic!("slice"); }
    let as_ = rank_scale(&atk[..n], t_atk); let us = rank_scale(&ut[..n], t_util);
    i64at(t, 0xa8).wrapping_mul(as_).wrapping_add(i64at(t, 0xb0).wrapping_mul(us)) / 100
}
pub unsafe extern "C" fn w_champion_hp_value(a0: usize, a1: usize) -> i64 {
    const I: usize = 10;
    SW[I].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe extern "C" fn(usize, usize) -> i64 = core::mem::transmute(SW[I].orig.load(Ordering::Relaxed));
    let t = top();
    let g = f(a0, a1);
    if t {
        SW[I].cmp.fetch_add(1, Ordering::Relaxed);
        match catch_unwind(AssertUnwindSafe(|| my_champion_hp_value(a0, a1))) {
            Ok(m) => if m != g { note(I, format!("g={} m={} | p={:#x} t={:#x} me.team={} t.team={} allies={} enemies={} t.atk={} t.av={} t.uv={}", g, m, a0, a1, r64(a0, 0x978), r64(a1, 0x60), r64(a0, 0x14d0), r64(a0, 0x14f0), i64at(a1, 0xb8), i64at(a1, 0xa8), i64at(a1, 0xb0))); },
            Err(_) => { SW[I].pan.fetch_add(1, Ordering::Relaxed); }
        }
    }
    pop(); g
}
