#![allow(unused, dead_code, non_snake_case)]
//! 22차 배치D 오라클 — #132 champion_hp_value_uncached (hidden define, m04.ll:52225)
//! 시도 ①: `extern "Rust"` + `#[link_name]` 로 hidden 심볼 직접 호출 (fat LTO 병합 모듈이라 해석 가능한지 실증)
//! 인자 = 제로버퍼 ScoreParameter(5384B) / ChampionScoreParameter(216B) — 함수가 읽는 오프셋만 채운다
//!   (IR 확인: %0 은 0x9d0/0x978/0x9d8/0x9e0/0x9e8/0x14b8/0x14d0/0x14d8/0x14f0 만, %1 은 0x60/0xa8/0xb0/0xb8/0xc0/0xc8/0xd0 만)
//! 사용: o132.exe <case>  — 케이스당 프로세스 1개(TLS 아님이지만 규약 준수)
use std::mem::{size_of, align_of};

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai5utils26champion_hp_value_uncached"]
    fn uncached(p: *const u8, t: *const u8) -> i64;
}

#[repr(C, align(8))]
struct Buf<const N: usize>([u8; N]);

fn w64(b: &mut [u8], off: usize, v: i64) { b[off..off + 8].copy_from_slice(&v.to_le_bytes()); }
fn wptr(b: &mut [u8], off: usize, p: *const u8) { b[off..off + 8].copy_from_slice(&(p as usize).to_le_bytes()); }

#[derive(Clone, Copy, Default)]
struct C { team: i64, atk: i64, ub: i64, cc: i64, bf: i64, av: i64, uv: i64 }

fn csp(c: &C) -> Box<Buf<216>> {
    let mut b = Box::new(Buf([0u8; 216]));
    w64(&mut b.0, 0x60, c.team);
    w64(&mut b.0, 0xa8, c.av);
    w64(&mut b.0, 0xb0, c.uv);
    w64(&mut b.0, 0xb8, c.atk);
    w64(&mut b.0, 0xc0, c.ub);
    w64(&mut b.0, 0xc8, c.cc);
    w64(&mut b.0, 0xd0, c.bf);
    b
}

/// 독립 재구현(명세 logic 그대로) — 오라클과 대조
fn model(me: &C, allies: &[C], enemies: &[C], t: &C) -> i64 {
    let mut atk = [0i64; 10]; let mut util = [0i64; 10];
    let (mut n, mut same, mut opp) = (0usize, 0i64, 0i64);
    atk[0] = me.atk; n = 1;
    if me.team == t.team { same += me.atk } else { opp += me.atk }
    for p in allies { if n >= 10 { break } atk[n] = p.atk; if p.team == t.team { same += p.atk } else { opp += p.atk } n += 1; }
    for p in enemies { if n >= 10 { break } atk[n] = p.atk; if p.team == t.team { same += p.atk } else { opp += p.atk } n += 1; }
    let upc = |p: &C, d: i64| p.ub + p.cc * d / 1000 + p.bf * d / 10000;
    let ally_dps = if me.team == t.team { same } else { opp };
    util[0] = upc(me, ally_dps);
    let mut idx = 1;
    for p in allies { if idx >= n { break } let d = if p.team == t.team { same } else { opp }; util[idx] = upc(p, d); idx += 1; }
    for p in enemies { if idx >= n { break } let d = if p.team == t.team { same } else { opp }; util[idx] = upc(p, d); idx += 1; }
    let ta = t.atk; let tu = upc(t, same);
    let rank = |vals: &[i64], tg: i64| -> i64 {
        let n = vals.len() as i64; if n < 2 { return 100 }
        let (mut lower, mut equal) = (0i64, 0i64);
        for v in vals { if *v < tg { lower += 1 } if *v == tg { equal += 1 } }
        let middle = lower + equal / 2;
        let r = middle * 100 / (n - 1);
        let rx = r.clamp(0, 100);
        if r < 51 { rx + 50 } else { rx * 2 }
    };
    let a = rank(&atk[..n], ta); let u = rank(&util[..n], tu);
    (t.av * a + t.uv * u) / 100
}

fn main() {
    if std::env::args().count() > 99 { let _ = game_ai::champion_hp_value as *const (); }
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    // 케이스 표 — 분기당 ≥2: n<2(100 배율) / rank<51 (+50) / rank>=51 (×2) / 동률 절반 / 10명 초과 잘림 / 팀 불일치 dps
    let mk = |team: i64, atk: i64, ub: i64, cc: i64, bf: i64| C { team, atk, ub, cc, bf, av: 1000, uv: 500 };
    let (me, allies, enemies, t): (C, Vec<C>, Vec<C>, C) = match case {
        0 => (mk(0, 100, 10, 0, 0), vec![], vec![], mk(0, 100, 10, 0, 0)),                    // n=1 → 배율 100/100
        1 => (mk(0, 100, 10, 0, 0), vec![mk(0, 50, 5, 0, 0)], vec![], mk(0, 100, 10, 0, 0)),   // n=2, target=me(equal 1) atk: lower1 → middle 1 → 100 → ×2
        2 => (mk(0, 100, 10, 0, 0), vec![mk(0, 200, 50, 0, 0)], vec![], mk(0, 100, 10, 0, 0)), // target 꼴찌: lower0 equal1 → middle0 → 0 → +50
        3 => (mk(0, 100, 10, 0, 0), vec![mk(0, 100, 10, 0, 0), mk(0, 100, 10, 0, 0)], vec![], mk(0, 100, 10, 0, 0)), // 전원 동률: equal 3 → middle 1 → 50 → r<51 → 100
        4 => (mk(0, 100, 10, 0, 0), vec![mk(0, 90, 10, 0, 0), mk(0, 80, 10, 0, 0)], vec![mk(1, 300, 0, 100, 100)], mk(1, 300, 0, 100, 100)), // 적 target 1위 + 팀별 dps
        5 => (mk(0, 100, 10, 0, 0), vec![mk(0, 1, 0, 0, 0); 9], vec![mk(1, 999, 0, 0, 0); 5], mk(1, 999, 0, 0, 0)), // 10 초과 잘림: 적은 안 들어감 → target 999 는 lower=10 → r=100/9*...
        6 => (mk(0, 100, 10, 0, 0), vec![mk(0, 1, 0, 0, 0); 9], vec![mk(1, 999, 0, 0, 0); 5], mk(0, 1, 0, 0, 0)),   // 잘림 + target 동률 9명
        7 => (mk(1, 100, 10, 0, 0), vec![mk(1, 50, 0, 1000, 0)], vec![mk(0, 70, 0, 0, 1000)], mk(0, 70, 0, 0, 1000)), // cc/bf 항 + 팀 불일치 dps(opp)
        8 => (mk(0, 100, 10, 0, 0), vec![mk(0, 101, 10, 0, 0)], vec![], mk(0, 100, 10, 0, 0)), // n=2 target 하위: r=0 → 50 ; util 동률 → middle 1 → 100 → 200
        _ => (mk(0, 100, 10, 0, 0), vec![mk(0, 100, 10, 0, 0)], vec![mk(1, 100, 10, 0, 0)], mk(0, 100, 10, 0, 0)), // 3명 전원 동률 atk
    };
    let cs_me = csp(&me);
    let al: Vec<Box<Buf<216>>> = allies.iter().map(csp).collect();
    let en: Vec<Box<Buf<216>>> = enemies.iter().map(csp).collect();
    // 연속 배열로 복사(stride 216)
    let mut alb = vec![0u8; 216 * allies.len().max(1)];
    for (i, b) in al.iter().enumerate() { alb[i * 216..(i + 1) * 216].copy_from_slice(&b.0); }
    let mut enb = vec![0u8; 216 * enemies.len().max(1)];
    for (i, b) in en.iter().enumerate() { enb[i * 216..(i + 1) * 216].copy_from_slice(&b.0); }
    let mut sp = Box::new(Buf([0u8; 5384]));
    sp.0[0x918..0x918 + 216].copy_from_slice(&cs_me.0);
    wptr(&mut sp.0, 0x14b8, alb.as_ptr()); w64(&mut sp.0, 0x14b8 + 0x18, allies.len() as i64);
    wptr(&mut sp.0, 0x14d8, enb.as_ptr()); w64(&mut sp.0, 0x14d8 + 0x18, enemies.len() as i64);
    let tb = csp(&t);
    let got = unsafe { uncached(sp.0.as_ptr(), tb.0.as_ptr()) };
    let exp = model(&me, &allies, &enemies, &t);
    println!("case={}\tgot={}\tmodel={}\t{}", case, got, exp, if got == exp { "MATCH" } else { "DIFF" });
}
