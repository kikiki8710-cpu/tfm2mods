//! combat_score — `0xd5bbf0`(action_score.rs:982~1517, 19,586B) 전투행동 점수의 **순수 포팅**(2026-09-07 01:0x~).
//!   정본 = `RE\2026-09-06_action_score-전투행동점수-0xd5bbf0-구조RE-0.5.8.md`(S0~S15 · 콜리 30종 판정표).
//!   계약: `(mode, prof, rec, ctx{W,sim,agents}, bb, sp(data,vt), slot, tgt, p9) -> i64`
//!         = `tower_support + risk_neg + pos_term + main` (조기반환 3종: 특수형 / −9,999,999 하드리젝트 / 처형 100+v).
//!   미포팅 콜리는 `na(tag)` 로 집계만 하고 None(=NA) — 리플레이 1판이면 어느 경로가 실제로 쓰이는지 빈도로 나온다.
#![allow(dead_code)]
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::passive_jungle::estimate_damage;
use super::as_callees::{isqrt_fast, pct_c, kind_pred};
use super::action_score::threat_sum;
use super::dn_cache;

const REC_SIDE: usize = 0x930; const REC_ROLE_O: usize = 0x9c0; const REC_SEED2: usize = 0x928;
const REC_464: usize = 0x464; const REC_208: usize = 0x208; const REC_210: usize = 0x210; const REC_448: usize = 0x448;
const BB_R: usize = 0x918; const BB_998: usize = 0x998; const BB_9A0: usize = 0x9a0; const BB_988: usize = 0x988; const BB_9B0: usize = 0x9b0;
const BB_1500: usize = 0x1500;
const BB_ALLY_PTR: usize = 0x14b8; const BB_ALLY_LEN: usize = 0x14d0;
const BB_ENEMY_PTR: usize = 0x14d8; const BB_ENEMY_LEN: usize = 0x14f0;
const RT_KILL: usize = 0x80; const RT_170: usize = 0x98; const RT_HANDLE: usize = 0x58;
const ENT_KIND: usize = 0x68; const ENT_488: usize = 0x488; const ENT_4C0: usize = 0x4c0;
const SLOT_BASE: usize = 0x10; const SLOT_PERLV: usize = 0x18; const SLOT_FLAG: usize = 0x30;
const CFG_8A8: usize = 0x8a8; const CFG_13F8: usize = 0x13f8;
const STRUCT_OFFS: [usize; 6] = [0x180, 0x1a0, 0x1c0, 0x190, 0x1b0, 0x1d0];

// ── 미포팅 콜리 집계 ─────────────────────────────────────────────────────────────────────
use std::sync::atomic::{AtomicU64, Ordering};
static NA_KEYS: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
static NA_CNTS: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
thread_local! { static STG: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; static TAGGED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
                /// 이번 호출이 **어느 반환 경로**로 나갔는지(DIFF 진단용)
                pub static PATH: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }
#[inline] fn pth_set(t: &str) { PATH.with(|c| c.set(tag8(t))); }
pub fn path_str() -> String { let v = PATH.with(|c| c.get()); v.to_le_bytes().iter().take_while(|b| **b != 0).map(|b| *b as char).collect() }
#[inline] fn stg(t: u64) { STG.with(|c| c.set(t)); }
fn na(tag: u64) -> Option<i64> {
    TAGGED.with(|c| c.set(true));
    for i in 0..32 {
        let k = NA_KEYS[i].load(Ordering::Relaxed);
        if k == tag { NA_CNTS[i].fetch_add(1, Ordering::Relaxed); return None; }
        if k == 0 && NA_KEYS[i].compare_exchange(0, tag, Ordering::Relaxed, Ordering::Relaxed).is_ok() { NA_CNTS[i].fetch_add(1, Ordering::Relaxed); return None; }
    }
    None
}
/// 다른 모듈(buff_value 등)에서 미포팅 지점을 집계할 때 쓰는 공개 창구
pub fn na_tag(t: &str) -> Option<i64> { na(tag8(t)) }
thread_local! { pub static ALLYD: std::cell::Cell<[i64; 10]> = const { std::cell::Cell::new([0; 10]) }; }
pub fn ally_diag() -> [i64; 10] { ALLYD.with(|c| c.get()) }
thread_local! { pub static S12ST: std::cell::Cell<[i64; 20]> = const { std::cell::Cell::new([0; 20]) }; }
/// [st, T, dmg, tps, burst, tgt.hp]
/// S5 위험항 진단: [near!=0, near.kind, near.0x88, dist(me,near), r_t, safe, bb.0x9b0 원값]
thread_local! { pub static S5D: std::cell::Cell<[i64; 21]> = const { std::cell::Cell::new([0; 21]) }; }
pub fn s5_diag() -> [i64; 21] { S5D.with(|c| c.get()) }
thread_local! { pub static CDLY: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
pub fn cast_dly() -> i64 { CDLY.with(|c| c.get()) }
/// 직전 S12 호출의 스테로이드 창 기여분(진단용)
pub fn s12_st() -> [i64; 20] { S12ST.with(|c| c.get()) }
/// ★스테로이드 `st` 상한 후보 대조기(검증 한정).
/// 실측: T=60 DIFF 4건이 전부 `st` 포화 상태에서 정확히 −5 였다 → 상한이 80 이 아닐 가능성.
/// 상수를 바꿔 맞추는 대신 후보를 **동시에 세어** 어느 것이 표본을 가장 많이 설명하는지 본다.
pub const CAPCAND: [i64; 6] = [80, 75, 70, 90, 100, 60];
pub static CAPHIT: [AtomicU64; 6] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
pub static CAPTOT: AtomicU64 = AtomicU64::new(0);
pub static CAPSAT: AtomicU64 = AtomicU64::new(0);
pub fn cap_tally(game: i64, mine: i64) {
    let z = s12_st();
    let (raw, st) = (z[10], z[6]);
    if z[1] == 0 { return; }
    CAPTOT.fetch_add(1, Ordering::Relaxed);
    if raw > 60 { CAPSAT.fetch_add(1, Ordering::Relaxed); }
    for i in 0..6 { if mine - st + raw.min(CAPCAND[i]) == game { CAPHIT[i].fetch_add(1, Ordering::Relaxed); } }
}
pub fn cap_report() -> String {
    let t = CAPTOT.load(Ordering::Relaxed);
    if t == 0 { return String::new(); }
    let mut s = format!("[S12cap] tot={} sat(raw>60)={}
", t, CAPSAT.load(Ordering::Relaxed));
    for i in 0..6 { s += &format!("  cap={:>3} hit={} ({:.3}%)
", CAPCAND[i], CAPHIT[i].load(Ordering::Relaxed), CAPHIT[i].load(Ordering::Relaxed) as f64 * 100.0 / t as f64); }
    s
}
/// ★★상한·임계치는 **상수가 아니라 실행중 바이트**다.
/// ai_adjust 자신이 이 즉치들을 노브(`sc_*`/`bv_*`) 값으로 덮어쓴다(detour.rs:1425-1445, 1980-2027).
/// exe 원본 상수를 하드코딩하면 노브를 건드린 순간 그 경로가 전부 DIFF 된다
/// (2026-09-07 실측: fcap=75·kcap=70 이어서 S12 잔차 5/10/3 이 나왔다).
#[inline] unsafe fn imm8(site: usize, orig: i64) -> i64 { super::super::live_imm8(site, orig as u8) as i64 }
#[inline] unsafe fn imm32(site: usize, orig: i64) -> i64 { super::super::live_imm32(site, orig as u32) as i64 }
/// ⛔`0xd729b0` 진입점 캡처는 **크래시**했다(2026-09-07 23:01, `c0000005` @ `exe+0xd72a48`,
/// 함수 내부 vec 루프에서 쓰레기 주소 역참조). 12바이트 프롤로그 패치가 이 함수엔 안전하지 않다.
/// 재시도 금지 — 필요하면 **호출부 쪽**(`0xd5cf83` 직전)에 다른 방식으로 걸 것.
pub fn nch_report() -> String { String::new() }
pub fn na_report() -> String {
    let mut v: Vec<(u64, u64)> = (0..32).filter_map(|i| { let k = NA_KEYS[i].load(Ordering::Relaxed); if k == 0 { None } else { Some((k, NA_CNTS[i].load(Ordering::Relaxed))) } }).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    if v.is_empty() { return String::new(); }
    let mut s = String::from("=== combat_score 미포팅 콜리(경로별 도달 수) ===\n");
    for (k, c) in v { let t: String = k.to_le_bytes().iter().take_while(|b| **b != 0).map(|b| *b as char).collect(); s += &format!("{:<10} x{}\n", t, c); }
    s
}
thread_local! { pub static S12D: std::cell::Cell<[i64; 8]> = const { std::cell::Cell::new([0; 8]) }; pub static LAST: std::cell::Cell<[i64; 18]> = const { std::cell::Cell::new([0; 18]) }; }
/// DIFF 로그용 단계 값
pub unsafe fn diag(_p1: usize, _p3: usize, _p4: usize) -> String {
    let v = LAST.with(|c| c.get());
    format!("risk_neg={} tower={} pos={} main={} urgent={} C={} thr_s={} chase={} bb998={} b9b0={} cast={} hp={} thr={} thrlen={}",
        v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12], v[13])
        + &format!(" game_thr={} bb970={} bb9a0={} bb988={}", v[14], v[15], v[16], v[17])
        + &{ let q = S12D.with(|c| c.get()); let z = s12_st(); format!(" | S12[D={} X={} Ct={} kill={} score={} e01450={} e019d0={} e02020={} st={} T={} dmg={} selfN={} burst={} thp={} stRaw={} bonus={} q={} pk={} i88={:#x} tid={:#x} m16={} v10={} aa={} THP={} v={} slN={}] A[{:?}]", q[0], q[1], q[2], q[3], q[4], q[5], q[6], q[7], z[0], z[1], z[2], z[3], z[4], z[5], z[10], z[7], z[8], z[9], z[12], z[13], z[14], z[15], z[16], z[17], z[18], z[19], ally_diag()) }
        + &{ let d = s5_diag(); format!(" S5[near={} kind={} f88={} dn={} rt={} safe={} raw9b0={} vis={} t0d={} t0a={} cdly={} alen={} th={:#x} a0={:#x} a1={:#x} pg={} d2={} a8={} eLen={} aLen={}] NIC[cand={} dmin={} mlen={} nstruct={}]", d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7], d[8], d[9], cast_dly(), d[10], d[11], d[12], d[13], d[14], d[15], d[16], d[19], d[20], nic_diag()[0], nic_diag()[1], nic_diag()[2], nic_diag()[3]) }
        + &unsafe { let caps = crate::judge::cap_util_c87fe0::last_p2().unwrap_or(0);
            // ★★`path=`·`S13[…]` 를 **caps 게이트 밖으로** 뺐다 — 잔차 23건이 전부 `caps=none` 이라
            //   경로를 안 찍은 것처럼 보였고, 그것 때문에 조기반환으로 오진단했다(RE 2026-09-07).
            format!("{} path={}{}", super::buff_value::s13_diag(), path_str(),
                if crate::ptr_ok(caps) { format!(" gameR={:#x} gameBB={:#x} gameBB998={:?}", rd_u64(caps + 0x18).unwrap_or(0), rd_u64(caps + 0x20).unwrap_or(0),
                    rd_u64(caps + 0x20).and_then(|b| rd_i64(b as usize + 0x998))) } else { " caps=none".into() }) }

}
#[inline] fn tag8(s: &str) -> u64 { let mut b = [0u8; 8]; for (i, c) in s.bytes().take(8).enumerate() { b[i] = c; } u64::from_le_bytes(b) }

// ── 0x16047b0 오판 플래그(결정적 해시, splitmix64 4회) ─────────────────────────────────
//   인자 (seed, rec.928, now, tps, min(100,rec.208), min(100,rec.210), rec.448) → (al, dl); 호출부는 둘을 OR.
//   capstone 전수(2026-09-07 01:25, 0x16047b0~0x1604a76): 나눗셈 매직 5종 = /6000 · /10000 · /10000 · /1000 · /1000.
#[inline] fn splitmix(x: u64) -> u64 {
    let z = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
#[inline] fn mulhi(a: u64, b: u64) -> u64 { ((a as u128).wrapping_mul(b as u128) >> 64) as u64 }
#[allow(clippy::too_many_arguments)]
pub fn misjudge(seed: u64, key: u64, now: u64, tps: u64, s1: u64, s2: u64, v7: u64) -> (bool, bool) {
    let step = (10u64.wrapping_mul(tps)).max(1);
    let (q, rem) = (now / step, now % step);
    let h0 = (seed ^ (key << 4) ^ (q << 40)) ^ 0x1a75e;
    let den = (810u64.wrapping_mul(tps)).max(1);
    let over = now.saturating_sub(210u64.wrapping_mul(tps));
    let ramp = ((over.wrapping_mul(6000)) / den).min(0x1a5e);
    let (t3, t3p) = (3u64.wrapping_mul(tps), 3u64.wrapping_mul(tps).wrapping_add(1));
    let a1 = { let h = splitmix(h0.wrapping_add(0x9e37_79b9_7f4a_7c15)); if t3p != 0 { mulhi(h, t3p).wrapping_add(t3) } else { h } };
    let win_a = (7u64.wrapping_mul(tps)).min((ramp.wrapping_mul(tps) / 6000).wrapping_add(a1));
    let win_b = { let h = splitmix(h0.wrapping_add(0x3c6e_f372_fe94_f82a)); if t3p != 0 { t3.wrapping_add(mulhi(h, t3p)) } else { h } };
    // 첫 판정: (해시 0..9999) < (100−s1)² · ramp / 10000  이면 rem < win_a
    let p1 = 100u64.saturating_sub(s1) as u32;
    let thr_a = ((0xd1b7_1759u64).wrapping_mul((ramp as u32).wrapping_mul(p1.wrapping_mul(p1)) as u64)) >> 45;
    let roll_a = mulhi(splitmix(h0.wrapping_add(0xdaa6_6d2c_7ddf_743f)), 10000);
    let al = if (roll_a as u32) < (thr_a as u32) { rem < win_a } else { false };
    // 둘째 판정: (해시 0..9999) < ((100−s2)²·4500/10000 · (1000−v7)/1000 · (1000−v7)/1000) ∧ rem < win_b
    let p2 = 100u64.saturating_sub(s2) as u32; let p3 = 1000u64.saturating_sub(v7) as u32;
    let e1 = (p2.wrapping_mul(p2)).wrapping_mul(4500) as u64;
    let e2 = (e1.wrapping_mul(0x68db_8bb)) >> 40;
    let e3 = ((e2 as u32).wrapping_mul(p3)) as u64;
    let e4 = (e3.wrapping_mul(0x83_126f)) >> 33;
    let e5 = ((e4 as u32).wrapping_mul(p3)) as u64;
    let thr_b = (e5.wrapping_mul(0x1062_4dd3)) >> 38;
    let roll_b = mulhi(splitmix(h0.wrapping_add(0x78dd_e6e5_fd29_f054)), 10000);
    let dl = (rem < win_b) && ((roll_b as u32) < (thr_b as u32));
    (al, dl)
}

// ── 헬퍼 ────────────────────────────────────────────────────────────────────────────────
#[inline] fn absd(a: u64, b: u64) -> u64 { if a < b { b - a } else { a - b } }
#[inline] fn sq(x: u64) -> u64 { x.wrapping_mul(x) }
#[inline] fn d2_xy(ax: u64, ay: u64, bx: u64, by: u64) -> u64 { let dx = absd(ax, bx); let dy = absd(ay, by); dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) }
#[inline] unsafe fn xy(e: usize) -> Option<(u64, u64)> { Some((rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?)) }
#[inline] unsafe fn d2_ee(a: usize, b: usize) -> Option<u64> { let (ax, ay) = xy(a)?; let (bx, by) = xy(b)?; Some(d2_xy(ax, ay, bx, by)) }
/// 0x12a07d0: 두 점 거리(isqrt)
#[inline] unsafe fn dist(a: usize, b: usize) -> Option<u64> { Some(isqrt_fast(d2_ee(a, b)?)) }
#[inline] unsafe fn rng_of(e: usize) -> Option<u64> {
    let p = rd_i32(e + ENT_F470)? as i64; let r = rd_u64(e + ENT_F680)?;
    Some(if p == 0 { r } else { ((p + 100).wrapping_mul(r as i64)) as u64 / 100 })
}
/// slot vt+0xe8 (사거리 기여) — dn_reach 디스패처 재사용
#[inline] unsafe fn slot_e8(slot: usize, e: usize, other: usize) -> Option<u64> {
    super::dn_reach::eff_e8(rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize, e, other, 0)
}
/// reach(e, slot, other) = slot.base + (slot.flag==0 ? rng(e) : 0) + e.438 + (lv−1)*slot.perlv + rng(other) + slot.vt_e8
/// `reach` 와 같되 `slot.vt+0xe8` 의 3번째 인자만 `e8t` 로 바꾼 판(S3 구조물 경로가 near 를 넘긴다)
unsafe fn reach_e8(e: usize, slot: usize, other: usize, e8t: usize) -> Option<u64> {
    let base = rd_u64(slot + SLOT_BASE)?; let per = rd_u64(slot + SLOT_PERLV)?;
    let own = rng_of(e)?;                      // ★S3 경로 식엔 SLOT_FLAG 게이트가 없다(RE 확정)
    Some(base.wrapping_add(own).wrapping_add(rd_u64(e + ENT_F438)?)
        .wrapping_add(rd_u64(e + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(per))
        .wrapping_add(rng_of(other)?).wrapping_add(slot_e8(slot, e, e8t)?))
}
unsafe fn reach(e: usize, slot: usize, other: usize) -> Option<u64> {
    let base = rd_u64(slot + SLOT_BASE)?; let per = rd_u64(slot + SLOT_PERLV)?;
    let own = if rd_i32(slot + SLOT_FLAG)? == 0 { rng_of(e)? } else { 0 };
    Some(base.wrapping_add(own).wrapping_add(rd_u64(e + ENT_F438)?)
        .wrapping_add(rd_u64(e + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(per))
        .wrapping_add(rng_of(other)?).wrapping_add(slot_e8(slot, e, other)?))
}
#[inline] unsafe fn est(slot: usize, e: usize, t: usize) -> Option<u64> { estimate_damage(slot, e, t, false) }
/// bb 의 Record Vec 에서 핸들이 일치하는 원소(0xd31bb0 계열 매칭)
unsafe fn find_rec(bb: usize, ptr_off: usize, len_off: usize, handle: u64) -> Option<Option<usize>> {
    let n = rd_u64(bb + len_off)?; if n == 0 { return Some(None); }
    let p = rd_u64(bb + ptr_off)? as usize; if !ptr_ok(p) { return None; }
    for i in 0..n.min(CAP_ITER) as usize { let r = p + i * AS_REC_STRIDE; if rd_u64(r + RT_HANDLE)? == handle { return Some(Some(r)); } }
    Some(None)
}
/// 적 로스터 중 dist² < lim 이고 (보임 ∨ last_seen+120 ≥ now) 인 원소가 있는가 / 목록
unsafe fn enemy_visible_near(w: &World, agents: usize, side: u64, from: usize, lim: u64, now: u64) -> Option<bool> {
    let other = 1 - side;
    for i in 0..5usize {
        let e = rd_u64(w.x + X_ROSTER + (other as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize;
        if e == 0 { continue; }
        if d2_ee(e, from)? >= lim { continue; }
        let h = rd_u64(e + ENT_HANDLE)?;
        if w.visible(side, h)? { return Some(true); }
        let rec = w.roster_rec(h)?;
        if rec != 0 { let role = rd_u32(rec + REC_ROLE_O) as usize;
            if rd_u64(agents + (other as usize) * LANE_STRIDE + LANE_ROSTER + role * 8)?.wrapping_add(120) >= now { return Some(true); } }
    }
    Some(false)
}
/// 0xd729b0 형: 체인(적/아군 구조물 6 + 미니언 Vec)에서 self 에 가장 가까운 원소
thread_local! { pub static NIC: std::cell::Cell<[i64; 4]> = const { std::cell::Cell::new([0; 4]) }; }
pub fn nic_diag() -> [i64; 4] { NIC.with(|c| c.get()) }
/// ★게이트① 진단 — [후보 수(널 아님), 임계 무시 최소 d2>>8, 미니언 len, 구조물 널 아님 수]
unsafe fn nearest_in_chain(w: &World, side: u64, me: usize) -> Option<Option<(usize, u64)>> {
    let (mx, my) = xy(me)?;
    let mut best: Option<(usize, u64)> = None;
    let (mut ncand, mut dmin, mut nstruct) = (0i64, i64::MAX, 0i64);
    // ★후보 필터: dist(cand, self)² >> 8 <= 0x53d1ac0 (= 150,000 이내). 누락 시 먼 타워를 잡아
    //   구조물 경로로 과도하게 들어간다(RE 0xd72aad, 2026-09-07). 갱신은 strict `<`(동률이면 먼저 온 것).
    let mut consider = |e: usize, best: &mut Option<(usize, u64)>| -> Option<()> {
        // ★게임은 널 체크를 안 하지만(널이 안 들어오므로), 재현이 널을 역참조하면 NA 가 된다.
        //   읽기 실패는 **NA 가 아니라 skip** 으로 흡수한다(2026-09-07 실측: 제거만 하니 NA 104).
        let (ex, ey) = match xy(e) { Some(v) => v, None => return Some(()) };
        let d = d2_xy(ex, ey, mx, my);
        ncand += 1; dmin = dmin.min((d >> 8) as i64);
        if (d >> 8) > 0x53d1ac0 { return Some(()); }
        if best.map_or(true, |(_, bd)| d < bd) { *best = Some((e, d)); }
        Some(())
    };
    for off in STRUCT_OFFS { let e = rd_u64(w.x + off + (side as usize) * 8)? as usize; if e != 0 { nstruct += 1; consider(e, &mut best)?; } }
    let n = rd_u64(w.x + X_MINION_LEN + (side as usize) * 0x20)?; let p = rd_u64(w.x + X_MINION_PTR + (side as usize) * 0x20)? as usize;
    if n != 0 { if !ptr_ok(p) { return None; }
        // ★게임의 vec 루프는 **원소 널 체크를 안 한다**(`0xd72a6c` 가 곧바로 `+0x660` 역참조) — RE 2026-09-07
        for i in 0..n.min(4096) as usize { let e = rd_u64(p + i * 8)? as usize; consider(e, &mut best)?; } }
    NIC.with(|c| c.set([ncand, if dmin == i64::MAX { -1 } else { dmin }, n as i64, nstruct]));
    Some(best)
}


// ── S12 dyn 게터 ─────────────────────────────────────────────────────────────────────────
//   단순 impl + 자식 순회 합성(자식 = [p+8] ptr · [p+0x10] len · stride 0x10)의 두 형태뿐이다(capstone 2026-09-07 03:0x).
#[inline] unsafe fn kids(p: usize) -> Option<(usize, u64)> { Some((rd_u64(p + 8)? as usize, rd_u64(p + 0x10)?)) }
/// slot.vt+0xa8 → Option<[u64;6]>(sret). `0x109ba90` = tag 0(None) · `0x12a68e0` = 자식 중 **첫 Some**.
/// ★`slot_a8`/`slot_c8` 의 일부 잎은 `me`(r9) 의 스탯을 읽는데 이 함수들은 me 를 안 받는다 —
/// `combat_score` 진입부에서 TLS 에 담아 둔다(RE 2026-09-07).
thread_local! { static LEAF_ME: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
pub(super) fn set_leaf_me(me: usize) { LEAF_ME.with(|c| c.set(me)); }
#[inline] fn me_of_a8() -> Option<usize> { let v = LEAF_ME.with(|c| c.get()); if v == 0 { None } else { Some(v) } }
/// ★`(x >> 2) / 100` = `x/400`. 게임은 `shr 2` 후 `/100` 매직을 쓴다(RE 2026-09-07).
#[inline] fn q_s2(x: u64) -> u64 { (x >> 2) / 100 }
pub(super) unsafe fn slot_a8_pub(data: usize, vt: usize, depth: u32) -> Option<Option<[u64; 6]>> { slot_a8(data, vt, depth) }
unsafe fn slot_a8(data: usize, vt: usize, depth: u32) -> Option<Option<[u64; 6]>> {
    if depth > 40 { super::dyn_eff::unseen(0x601, depth as usize); return None; }
    let r = super::dyn_eff::impl_rva(vt, 0xa8)?; let p = super::dyn_eff::arc_payload(data, vt)?;
    match r {
        0x109ba90 => Some(None),
        // 0x17cc480: [tag=1, 0, self[0x28], 0, 0, self[0x18]] — 상수+self 복사뿐(RE 2026-09-07)
        0x17cc480 => Some(Some([1, 0, rd_u64(p + 0x28)?, 0, 0, rd_u64(p + 0x18)?])),
        // ★신규 2종 — 이 둘을 채우면 `slot_a8` 은 정적으로 **완결**된다(RE 2026-09-07 census)
        0x153b460 => Some(Some([1, rd_u64(p + 0x10)?, rd_u64(p + 0x18)?, 0, 0, rd_u64(p + 8)?])),
        0x1341a20 => { let s1 = rd_u64(me_of_a8()? + 0x620)?;
            Some(Some([1, q_s2(rd_u64(p + 0x10)?.wrapping_mul(s1)).wrapping_add(rd_u64(p + 8)?), 0,
                       q_s2(rd_u64(p + 0x20)?.wrapping_mul(s1)).wrapping_add(rd_u64(p + 0x18)?),
                       rd_u64(p + 0x28)?, rd_u64(p)?])) }
        0x12a68e0 => { let (arr, n) = kids(p)?; if n == 0 { return Some(None); } if !ptr_ok(arr) { return None; }
            for i in 0..n.min(CAP_ITER) as usize {
                let v = slot_a8(rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize, depth + 1)?;
                if v.is_some() { return Some(v); } }
            Some(None) }
        // `0x1153840` = `impl Trait for &T` 포워딩 썽크(구현체 아님, RE 2026-09-07).
        //   payload 안의 내부 fat-ptr `(p+0, p+8)` 로 재디스패치하되 **내부 data 는 Arc 보정 없이 원본**이다.
        0x1153840 => { let (id, iv) = (rd_u64(p)? as usize, rd_u64(p + 8)? as usize);
            if !ptr_ok(id) || !ptr_ok(iv) { return None; }
            slot_a8_p(id, iv, depth + 1) }
        _ => { super::dyn_eff::unseen(0x5a8, r); None }
    }
}
/// `slot_a8` 의 "페이로드를 직접 받는" 판 — 썽크가 넘긴 내부 포인터용(Arc 보정 금지).
unsafe fn slot_a8_p(payload: usize, vt: usize, depth: u32) -> Option<Option<[u64; 6]>> {
    if depth > 40 { super::dyn_eff::unseen(0x607, depth as usize); return None; }
    let r = super::dyn_eff::impl_rva(vt, 0xa8)?;
    match r {
        0x109ba90 => Some(None),
        0x17cc480 => Some(Some([1, 0, rd_u64(payload + 0x28)?, 0, 0, rd_u64(payload + 0x18)?])),
        0x12a68e0 => { let (arr, n) = (rd_u64(payload + 8)? as usize, rd_u64(payload + 0x10)?);
            if n == 0 { return Some(None); } if !ptr_ok(arr) { return None; }
            for i in 0..n.min(CAP_ITER) as usize {
                let v = slot_a8(rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize, depth + 1)?;
                if v.is_some() { return Some(v); } }
            Some(None) }
        0x1153840 => { let (id, iv) = (rd_u64(payload)? as usize, rd_u64(payload + 8)? as usize);
            if !ptr_ok(id) || !ptr_ok(iv) { return None; }
            slot_a8_p(id, iv, depth + 1) }
        _ => { super::dyn_eff::unseen(0x5a9, r); None }
    }
}
/// slot.vt+0xc0 → bool. `0x9db70` = false · `0x12a6b10` = any(child)
unsafe fn slot_c0(data: usize, vt: usize, depth: u32) -> Option<bool> {
    if depth > 40 { super::dyn_eff::unseen(0x602, depth as usize); return None; }
    let r = super::dyn_eff::impl_rva(vt, 0xc0)?; let p = super::dyn_eff::arc_payload(data, vt)?;
    match r {
        EFF_E8_ZERO => Some(false), EFF_TRUE => Some(true),
        0x12a6b10 => { let (arr, n) = kids(p)?; if n == 0 { return Some(false); } if !ptr_ok(arr) { return None; }
            for i in 0..n.min(CAP_ITER) as usize {
                if slot_c0(rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize, depth + 1)? { return Some(true); } }
            Some(false) }
        // any(list1 @p+0x50/len p+0x58, stride 0x18) || any(list2 @p+0x68/len p+0x70, stride 0x10)
        0x153b490 => {
            for (po, lo, st) in [(0x50usize, 0x58usize, 0x18usize), (0x68, 0x70, 0x10)] {
                let n = rd_u64(p + lo)?; if n == 0 { continue; }
                let arr = rd_u64(p + po)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n.min(CAP_ITER) as usize {
                    let e = arr + i * st;
                    if slot_c0(rd_u64(e)? as usize, rd_u64(e + 8)? as usize, depth + 1)? { return Some(true); }
                }
            }
            Some(false)
        }
        _ => { super::dyn_eff::unseen(0x5c0, r); None }
    }
}
/// slot.vt+0xc8 → (T, f1, f2) sret. `0x109bac0` 은 f2←2 만 쓴다(= 호출부에서 즉시 0).
unsafe fn slot_c8(data: usize, vt: usize, depth: u32) -> Option<(u64, u8, u8)> {
    if depth > 40 { super::dyn_eff::unseen(0x603, depth as usize); return None; }
    let r = super::dyn_eff::impl_rva(vt, 0xc8)?; let p = super::dyn_eff::arc_payload(data, vt)?;
    match r {
        0x109bac0 => Some((0, 0, 2)),
        // ★신규 3종 — 채우면 `slot_c8` 완결(RE 2026-09-07)
        0x1706e70 => Some((rd_u64(p)?, 0, 1)),
        0x170fa90 => { let s1 = rd_u64(me_of_a8()? + 0x620)?;
            Some((rd_u64(p)?.wrapping_add(q_s2(rd_u64(p + 8)?.wrapping_mul(s1))), 0, 1)) }
        0x1728dd0 => { let s1 = rd_u64(me_of_a8()? + 0x620)?;
            Some((rd_u64(p)?.wrapping_add(q_s2(rd_u64(p + 8)?.wrapping_mul(s1))), 1, 0)) }
        0x12a6970 => { let (arr, n) = kids(p)?; if n == 0 { return Some((0, 0, 2)); } if !ptr_ok(arr) { return None; }
            for i in 0..n.min(CAP_ITER) as usize {
                let v = slot_c8(rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize, depth + 1)?;
                if v.2 != 2 { return Some(v); } }
            Some((0, 0, 2)) }
        _ => { super::dyn_eff::unseen(0x5c8, r); None }
    }
}
/// `vt+0x80` 의 (flag, value) 쌍. `0x1146c40` = 자식(ptr p+0x20 / len p+0x28 / stride 0x18) 중
///   **첫 `al&1`** 인 자식의 rax·rdx 를 그대로 돌려준다(0x88 아니라 0x80 으로 내려감 — RE 2026-09-07).
unsafe fn slot_80_pair(data: usize, vt: usize, depth: u32) -> Option<(bool, u64)> {
    if depth > 40 { super::dyn_eff::unseen(0x606, depth as usize); return None; }
    let r = super::dyn_eff::impl_rva(vt, 0x80)?;
    let p = super::dyn_eff::arc_payload(data, vt)?;
    // ★`0x109bab0` 은 `slot_88` 에는 있는데 여기엔 빠져 있었다(RE 2026-09-07)
    if r == 0x109bab0 { return Some((true, rd_u64(p + 0x10)?.wrapping_add(rd_u64(p + 0x18)?))); }
    match r {
        EFF_E8_ZERO => Some((false, 0)),
        EFF_TRUE | 0x122fcf0 => { super::dyn_eff::unseen(0x6a2, r); Some((true, 0)) }
        0x1145640 => Some((true, 1)),
        0x1147250 => Some((true, rd_u64(p)?)),
        0x16adaa0 => Some((true, rd_u64(p + 0x30)?)),          // mov rdx,[rcx+0x30]; mov eax,1; ret
        0x13bfa20 => Some((true, rd_u64(p + 8)?)),            // mov rdx,[rcx+8];    mov eax,1; ret
        0x1606550 => slot_80_pair(rd_u64(p + 0x18)? as usize, rd_u64(p + 0x20)? as usize, depth + 1),
        0x1146c40 => {
            let n = rd_u64(p + 0x28)?; if n == 0 { return Some((false, 0)); }
            let arr = rd_u64(p + 0x20)? as usize;
            if !ptr_ok(arr) { super::dyn_eff::unseen(0x693, r); return None; }
            for i in 0..n.min(CAP_ITER) as usize {
                let v = slot_80_pair(rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize, depth + 1)?;
                if v.0 { return Some(v); }
            }
            Some((false, 0)) }
        0x12a6b80 => {
            let n = rd_u64(p + 0x10)?; if n == 0 { return Some((false, 0)); }
            let arr = rd_u64(p + 8)? as usize;
            if !ptr_ok(arr) { super::dyn_eff::unseen(0x693, r); return None; }
            for i in 0..n.min(CAP_ITER) as usize {
                let v = slot_80_pair(rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize, depth + 1)?;
                if v.0 { return Some(v); }
            }
            Some((false, 0)) }
        _ => { super::dyn_eff::unseen(0x680, r); None }
    }
}
/// slot.vt+0x88 → (flag, T). `0x1147250` = (1, [p]) · `0x1145640` = (1,1) · `0x12a57b0` = 자식 중 첫 flag&1
/// `slot_88` 인데 **이미 payload 인 포인터**를 받는 판. 교차 위임 썽크(`0x115c2f0` 등)는
/// 자식에게 **Arc 보정 없이 raw 데이터**를 그대로 넘기므로, `arc_payload` 를 되돌린 뒤 재사용한다.
pub(super) unsafe fn slot_88_rawp(payload: usize, vt: usize, depth: u32) -> Option<(bool, u64)> {
    let align = rd_u64(vt + 0x10)?;
    let off = (((align.wrapping_sub(1)) & !0xfu64) as usize).wrapping_add(0x10);
    slot_88(payload.wrapping_sub(off), vt, depth)
}
pub(super) unsafe fn slot_88(data: usize, vt: usize, depth: u32) -> Option<(bool, u64)> {
    if depth > 40 { super::dyn_eff::unseen(0x604, depth as usize); return None; }
    let r = match super::dyn_eff::impl_rva(vt, 0x88) {
        Some(r) => r, None => { super::dyn_eff::unseen(0x688, rd_u64(vt + 0x88).unwrap_or(0) as usize & 0xffff_ffff); return None; } };
    let p = match super::dyn_eff::arc_payload(data, vt) {
        Some(p) => p, None => { super::dyn_eff::unseen(0x689, 0); return None; } };
    // 조용한 None 을 남기지 않는다 — 모든 실패 지점에 표식(0x68x)을 단다.
    macro_rules! rq { ($e:expr, $c:expr) => { match $e { Some(v) => v, None => { super::dyn_eff::unseen($c, r); return None; } } } }
    match r {
        EFF_E8_ZERO => Some((false, 0)),
        0x1145640 => { super::dyn_eff::unseen(0x6a3, r); Some((true, 1)) }
        0x1147250 => Some((true, rq!(rd_u64(p), 0x68a))),
        // ★분리 필수 — 셋을 `[p+0x00]` 으로 묶어 놓은 것이 S12 잔여 DIFF 의 원인이었다(RE 2026-09-07 전수 census)
        0x109bab0 => Some((true, rq!(rd_u64(p + 0x10), 0x68a).wrapping_add(rq!(rd_u64(p + 0x18), 0x68a)))),
        0x122f090 => Some((true, rq!(rd_u64(p + 0x40), 0x68a))),
        0x122fcf0 => Some((true, rq!(rd_u64(p + 0x10), 0x68a))),   // ★신규(미처리였음)
        0x1151480 => Some((true, rq!(rd_u64(p + 0x18), 0x68a))),   // ★신규
        0x133e490 => { let t = ((rq!(rd_u64(p + 0x30), 0x68a) != 0) as u64).max(rq!(rd_u64(p + 0x48), 0x68a));
                       Some((t != 0, t)) }                          // ★신규
        0x13bfa20 => Some((true, rq!(rd_u64(p + 8), 0x68a))),    // ★+8 (RE 2026-09-07). 구 코드는 p 를 읽었다
        0x16adaa0 => Some((true, rq!(rd_u64(p + 0x30), 0x68a))), // ★+0x30
        // ⚠아래 arm 들은 T 를 **임의로** 0/1 로 둔다 — 실측상 게임은 60(=tps)을 내는 경우가 있다.
        //   어느 impl 이 그 자리에 있는지 표식으로 특정한다(2026-09-07).
        // ★★`17c2bd0: mov rdx,[rcx]; xor eax,eax; test rdx,rdx; setne al; ret`
        //   = `T = [p+0x00]`, `ok = (T != 0)`. ~~`(f, 0)`~~ 은 **구조적으로 절대 못 맞힌다**
        //   (ok=true ⟺ T≠0 인데 T 를 0 으로 고정했으니). 574 표본이 전부 T=0 이던 서명이 이것.
        0x17c2bd0 => { let v = rq!(rd_u64(p), 0x68a); Some((v != 0, v)) }
        0x1701740 => { let f = rq!(rd_u64(p + 0x48), 0x68a) != 0; if f { super::dyn_eff::unseen(0x6a4, r); } Some((f, 1)) }
        0x1153880 => { let cd = rq!(rd_u64(p), 0x68a) as usize; let v2 = rq!(rd_u64(p + 8), 0x68a) as usize;
            // ★썽크는 자식 **TraitB `vt+0xb0`** 로 rcx=cd(무보정 raw) 테일콜한다. TraitB +0xb0 은
            //   `0x9db70`(=0) 과 `0x1151480` 둘뿐이라 이걸로 완결(RE 2026-09-07).
            if let Some(r2) = super::dyn_eff::impl_rva(v2, 0xb0) { if r2 == 0x1151480 { return Some((true, rq!(rd_u64(cd + 0x18), 0x68a))); } }
            let r2 = rq!(super::dyn_eff::impl_rva(v2, 0xb0), 0x68b);
            match r2 { EFF_E8_ZERO => Some((false, 0)), EFF_TRUE => { super::dyn_eff::unseen(0x6a1, r2); Some((true, 0)) },
                       _ => { super::dyn_eff::unseen(0x5b0, r2); None } } }
        // ★"첫 ok" 가 아니라 **ok 인 자식들의 unsigned max** — 합성형 계열과 같은 규약(RE 2026-09-07)
        0x1606700 | 0x164ecc0 => { let o = if r == 0x1606700 { 0x18 } else { 0 };
            let a = slot_88(rq!(rd_u64(p + o), 0x68a) as usize, rq!(rd_u64(p + o + 8), 0x68a) as usize, depth + 1)?;
            let b = slot_88(rq!(rd_u64(p + o + 0x10), 0x68a) as usize, rq!(rd_u64(p + o + 0x18), 0x68a) as usize, depth + 1)?;
            Some(match (a.0, b.0) { (true, true) => (true, a.1.max(b.1)), (true, false) => a, (false, true) => b, _ => (false, 0) }) }
        // ★이 계열은 전부 같은 모노모픽 템플릿이다(RE 2026-09-07, 0x12a57b0/0x12a5100/0x12481a0 기계어 확인):
        //   ①모든 리스트를 **이어붙여** 순회 ②ok(al&1) 인 자식들의 **unsigned max** ③ok 가 하나도 없으면 (false, 0).
        //   예전 "리스트별로 돌다 첫 ok 에서 return" 은 자식이 2개 이상일 때 값이 달라진다.
        0x12a57b0 | 0x12a61c0 | 0x12481a0 | 0x13bfc40 | 0x1340ef0 | 0x16a3450 | 0x153b5a0 | 0x12a5100 => {
            let lists: &[(usize, usize, usize)] = match r {     // (ptr, len, stride)
                0x12a57b0 => &[(8, 0x10, 0x10)],
                0x12a61c0 => &[(0x20, 0x28, 0x18)],
                0x12481a0 => &[(0x50, 0x58, 0x18)],
                0x12a5100 => &[(0x48, 0x50, 0x10)],             // ★정정: 구세대 표는 (0x50,0x58,0x18) 이었다
                0x13bfc40 => &[(8, 0x10, 0x18)],
                0x1340ef0 => &[(0x68, 0x70, 0x18), (0x80, 0x88, 0x10)],
                0x16a3450 => &[(0x50, 0x58, 0x18), (0x68, 0x70, 0x18)],   // ★list2 stride 0x18(0x153b5a0 과 다름)
                0x153b5a0 => &[(0x50, 0x58, 0x18), (0x68, 0x70, 0x10)],
                _ => &[],
            };
            let mut best: Option<u64> = None;
            for (po, lo, st) in lists {
                let n = rq!(rd_u64(p + lo), 0x68c); if n == 0 { continue; }
                let arr = rq!(rd_u64(p + po), 0x68c) as usize;
                if !ptr_ok(arr) { super::dyn_eff::unseen(0x68d, r); return None; }
                for i in 0..n.min(CAP_ITER) as usize {
                    let (ok, v) = slot_88(rq!(rd_u64(arr + i * st), 0x68c) as usize,
                                          rq!(rd_u64(arr + i * st + 8), 0x68c) as usize, depth + 1)?;
                    if ok { best = Some(best.map_or(v, |b| b.max(v))); }
                }
            }
            Some(match best { Some(v) => (true, v), None => (false, 0) }) }
        // 0x1146c40 은 slot 0x88 에 놓여도 **자식은 vt+0x80 으로** 내려간다(RE 2026-09-07).
        0x1146c40 => slot_80_pair(data, vt, depth),
        // ★골격 스캐너 폴백: 자식 순회 "any" 형이면 RVA 표 없이 처리
        _ => {
            let f = rq!(rd_u64(vt + 0x88), 0x68e) as usize;
            if let Some((lp, n)) = super::dyn_eff::composite_loops(f, 0x88) {
                let mut gmax: Option<u64> = None;
                for k in 0..n {
                    let (lo, po, st) = lp[k];
                    let cnt = rq!(rd_u64(p + lo), 0x68f); if cnt == 0 { continue; }
                    let arr = rq!(rd_u64(p + po), 0x68f) as usize;
                    if !ptr_ok(arr) { super::dyn_eff::unseen(0x690, r); return None; }
                    for i in 0..cnt.min(CAP_ITER) as usize {
                        let e = arr + i * st;
                        // ★해독된 이 계열 impl 은 전부 "ok 자식들의 max" 였다 → 폴백도 같은 의미로 맞춘다
                        let (ok, v) = slot_88(rq!(rd_u64(e), 0x68f) as usize, rq!(rd_u64(e + 8), 0x68f) as usize, depth + 1)?;
                        if ok { gmax = Some(gmax.map_or(v, |b: u64| b.max(v))); }
                    }
                }
                return Some(match gmax { Some(v) => (true, v), None => (false, 0) });
            }
            if let Some((da, db, sl)) = super::as_callees::delegate_pair(f) {
                if sl == 0x88 { return slot_88(rq!(rd_u64(p + da), 0x691) as usize, rq!(rd_u64(p + db), 0x691) as usize, depth + 1); }
            }
            super::dyn_eff::unseen(0x588, r); None
        }
    }
}
/// 공격 주기(0xe01450 안): max(3, prov90 * 100 / max(1, e.3fc+100)) — 원본은 `< 4 → 3` 로 클램프
unsafe fn atk_iv2(e: usize) -> Option<u64> {
    let base = super::dyn_eff::prov90_cooltime(rd_u64(e + 0x570)? as usize, rd_u64(e + 0x578)? as usize, e)?;
    let a = (rd_i32(e + 0x3fc)?).wrapping_add(100); let den = if a < 2 { 1u64 } else { a as u32 as u64 };
    let v = base.wrapping_mul(100) / den; Some(if v < 4 { 3 } else { v })
}
/// 자기 진영 로스터에서 tgt 150,000 이내인 원소를 도는 공통 루프. f(ally, ally_agent_rec)
unsafe fn near_allies<F: FnMut(usize, usize) -> Option<()>>(w: &World, side: u64, tgt: usize, skip_h: Option<u64>, mut f: F) -> Option<()> {
    let (tx, ty) = xy(tgt)?;
    for i in 0..5usize {
        let e = rd_u64(w.x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize;
        if e == 0 { continue; }
        if let Some(h) = skip_h { if rd_u64(e + ENT_HANDLE)? == h { continue; } }
        let (ex, ey) = xy(e)?;
        if d2_xy(ex, ey, tx, ty) > 0x53d1ac100 { continue; }
        let rec = w.roster_rec(rd_u64(e + ENT_HANDLE)?)?; if rec == 0 { continue; }
        f(e, rec)?;
    }
    Some(())
}
/// score_parameter 표: W + 0x280 + side*0xfa0 + role*0x320
#[inline] unsafe fn mat(w: &World, rec: usize) -> Option<usize> {
    let side = rd_u64(rec + REC_SIDE)?; if side > 1 { return None; }
    Some(w.x + 0x280 + (side as usize) * 0xfa0 + (rd_u32(rec + REC_ROLE_O) as usize) * 0x320)
}
// ── S12 스테로이드 창 블록 (0xd5f2be~0xd5fdac) ────────────────────────────────────────
//   RE 전수해독 2026-09-07: RE\2026-09-07_combat_score-S12스테로이드창-전수해독-0.5.8.md
//   구조: (A)T결정 → (B)타깃role → (C)피해행렬합→st → (D)버스트(슬롯0~3) → (E)합산·보너스

/// 슬롯0 쿨다운 필드 오프셋(JT `0x1433dc0e0`). None = kind 0/3 = **비교 없이 포함**.
#[inline] fn cd0_off(kind: u64) -> Option<usize> {
    Some(match kind { 1 => 0xb8, 2 => 0x110, 4 => 0xe8, 5 => 0x1f0, 6 => 0x1f0, 7 => 0xe8,
                      8 => 0xb0, 9 => 0xc8, 10 => 0xf0, 11 => 0xd8, 12 => 0xd0, 13 => 0xb0, _ => return None })
}
/// 상태 리스트(`e.0x2c8` ptr / `e.0x2d0` len / stride 0x28 / tag = i32@+0)
#[inline] unsafe fn st_list(e: usize) -> Option<(usize, u64)> {
    let n = rd_u64(e + 0x2d0)?; if n == 0 { return Some((0, 0)); }
    if n > 4096 { return None; }
    let l = rd_u64(e + 0x2c8)? as usize; if !ptr_ok(l) { return None; }
    Some((l, n))
}
/// 프로바이더 vtable 의 단순 게터(obj = data **원본**). `decode_getter` 패턴 + `max([d+K],1)` 형(0x1716a40).
unsafe fn prov_getter(data: usize, vt: usize, slot: usize) -> Option<u64> {
    let f = rd_u64(vt + slot)? as usize; if !ptr_ok(f) { return None; }
    let (b0, b1, b2) = (rd_u8(f), rd_u8(f + 1), rd_u8(f + 2));
    // mov rax,[rcx+K]; cmp rax,1; adc rax,0; ret  →  max(v, 1)
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x81 && rd_u8(f + 7) == 0x48 && rd_u8(f + 8) == 0x83
        && rd_u8(f + 9) == 0xf8 && rd_u8(f + 10) == 0x01 && rd_u8(f + 11) == 0x48
        && rd_u8(f + 12) == 0x83 && rd_u8(f + 13) == 0xd0 {
        return Some(rd_u64(data.wrapping_add(rd_u32(f + 3) as usize))?.max(1));
    }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x41 && rd_u8(f + 4) == 0x48 && rd_u8(f + 5) == 0x83
        && rd_u8(f + 6) == 0xf8 && rd_u8(f + 7) == 0x01 && rd_u8(f + 8) == 0x48
        && rd_u8(f + 9) == 0x83 && rd_u8(f + 10) == 0xd0 {
        return Some(rd_u64(data.wrapping_add(rd_u8(f + 3) as usize))?.max(1));
    }
    super::as_callees::decode_getter(f, data)
}
/// `0x128cc90` — 슬롯0 즉시가용 술어
unsafe fn st_p0(me: usize) -> Option<bool> {
    let (l, n) = st_list(me)?;
    for i in 0..n as usize { if rd_i32(l + i * 0x28)? == 3 { return Some(false); } }
    let kind = rd_u64(me + ENT_KIND)?;
    let off = match cd0_off(kind) { Some(o) => o, None => return Some(false) };  // kind 0/3 → false
    for i in 0..n as usize { let t = rd_i32(l + i * 0x28)?; if !(2..=5).contains(&t) { return Some(false); } }
    Some(rd_u64(me + off)? == 0)
}
/// `0x12a0180` — 타깃 필터. sel = `slotN+0x28`(u32)
unsafe fn st_valid_target(sel: u32, me: usize, tgt: usize) -> Option<bool> {
    if !(rd_u8(tgt + 0x6b9) == 1 && rd_u64(tgt + 0x6a0)? == 0) { return Some(false); }
    let (a0, a8) = (rd_u64(me)?, rd_u64(me + 8)?);
    let (b0, b8) = (rd_u64(tgt)?, rd_u64(tgt + 8)?);
    let same_owner = a0 == b0 && (a0 != 0 || a8 == b8);
    let tk = rd_u64(tgt + ENT_KIND)?;
    let (mh, th) = (rd_u64(me + ENT_HANDLE)?, rd_u64(tgt + ENT_HANDLE)?);
    // has_state(tgt, mask 0x347) = tag ∈ {0,1,2,6,8,9}
    let hs = |m: u32| -> Option<bool> {
        let (l, n) = st_list(tgt)?;
        for i in 0..n as usize { let t = rd_i32(l + i * 0x28)?;
            if (0..=9).contains(&t) && (m >> t) & 1 == 1 { return Some(true); } }
        Some(false)
    };
    Some(match sel {
        0 => same_owner,
        1 => same_owner && tk == 13,
        2 => same_owner && tk == 13 && hs(0x347)?,
        3 => same_owner && tk == 13 && th != mh,
        4 => th == mh,
        5 => !same_owner,
        6 => !same_owner && (tk & !1) != 2,
        7 => !same_owner && tk == 13,
        8 => !same_owner && tk == 13 && hs(0x347)?,
        9 => !same_owner && tk == 13 && { if a8 >= 2 { return None; }        // 게임은 여기서 패닉
                                          rd_u64(tgt + 0x688 + (a8 as usize) * 8)? != 0 },
        10 => true,
        11 => th != mh && (tk & !1) != 2,
        12 => tk == 13,
        13 => false,
        _ => return None,
    })
}
/// `0x129ed50`(n=1) / `0x128cf70`(n=2) / `0x129d130`(n=3) — 동형 술어.
///   ★kind != 13 이면 **모든 경로가 false** 라 그 경우는 즉시 반환(게터 없이 판정 가능).
unsafe fn st_pn(me: usize, n: usize) -> Option<bool> {
    if rd_u64(me + ENT_KIND)? != 13 { return Some(false); }
    let (l, cnt) = match st_list(me) { Some(v) => v, None => return na_tag("STp_ls").map(|_| false) };
    let (mut has4, mut has5, mut has_other) = (false, false, false);
    for i in 0..cnt as usize { let t = rd_i32(l + i * 0x28)?;
        if t == 4 { has4 = true; } if t == 5 { has5 = true; }
        if !(2..=5).contains(&t) { has_other = true; } }
    if has4 { return Some(false); }
    let lv = rd_u64(me + ENT_LEVEL)?;
    let empty = crate::exe_base().wrapping_add(0x436230);
    let (sl, need_lv) = match n {
        1 => (me + 0x4c8, 0u64),
        2 => (if lv >= 3 { me + 0x500 } else { empty }, 3),
        _ => (if lv >= 5 { me + 0x538 } else { empty }, 5),
    };
    if has5 && rd_i32(sl + 0x30)? != -1 {
        let (sd, sv) = (rd_u64(sl)? as usize, rd_u64(sl + 8)? as usize);
        let f = rd_u64(sv + 0x120)? as usize;
        let inl = match super::dyn_eff::arc_payload(sd, sv) { Some(v) => v, None => return na_tag("STp_ap").map(|_| false) };
        match super::as_callees::decode_getter(f, inl) {
            Some(v) => { if v == 1 { return Some(false); } }
            None => { super::dyn_eff::unseen(0x1120, super::dyn_eff::impl_rva(sv, 0x120).unwrap_or(0)); return None; }
        }
    }
    // 쿨다운 여유 판정
    let (pa, pb, extra, clamp_lo, cdf) = match n {
        1 => (0x580usize, 0x580usize, 0i64, 3u64, 0xb8usize),
        2 => (0x590, if lv >= 3 { 0x590 } else { 0x5b0 }, 0, 1, 0xc0),
        _ => (0x5a0, if lv >= 5 { 0x5a0 } else { 0x5b0 }, rd_i32(me + 0x46c)? as i64, 1, 0xc8),
    };
    // ★프로바이더 계열 vtable(size 0x1a8) — self = **data 원본**(Arc 보정 금지).
    //   `0x1716a40` = `mov rax,[rcx+0x178]; cmp rax,1; adc rax,0; ret` = max(v,1) → 게임이 b==0 로 패닉하지 않는 이유.
    let a = {
        let (d, v) = (rd_u64(me + pa)? as usize, rd_u64(me + pa + 8)? as usize);
        // ★레벨 의존 impl(`0x1725060`/`0x17033a0`/`0x12462a0`)은 `prov_getter` 의 바이트패턴으로 안 잡힌다 —
        //   `prov90_cooltime` 이 이미 그 셋을 갖고 있으므로 폴백한다(판당 173 NA, 2026-09-07).
        match prov_getter(d, v, 0x90).or_else(|| super::dyn_eff::prov90_cooltime(d, v, me)) { Some(x) => x as i64,
            None => { super::dyn_eff::unseen(0x1090, super::dyn_eff::impl_rva(v, 0x90).unwrap_or(0)); return None; } }
    };
    let b = {
        let (d, v) = (rd_u64(me + pb)? as usize, rd_u64(me + pb + 8)? as usize);
        match prov_getter(d, v, 0xa8) { Some(x) => x,
            None => { super::dyn_eff::unseen(0x10a8, super::dyn_eff::impl_rva(v, 0xa8).unwrap_or(0)); return None; } }
    };
    if b == 0 {                                             // 게임은 패닉 — 여기 오면 vt+0xa8 디코드가 틀린 것
        let v = rd_u64(me + pb + 8)? as usize;
        super::dyn_eff::unseen(0x50a8, super::dyn_eff::impl_rva(v, 0xa8).unwrap_or(0));
        return na_tag("STp_b0").map(|_| false);
    }
    let d = (rd_i32(me + 0x400)? as i64).wrapping_add(extra).wrapping_add(100).max(1) as u64;
    let mut q = (a.wrapping_mul(100) as u64) / d;
    if q < clamp_lo { q = clamp_lo; }
    if rd_u64(me + cdf)? > q.wrapping_sub(q / b) { return Some(false); }
    if has_other { return Some(false); }
    Some(match n { 1 => rd_i32(me + 0x4f8)? != -1, _ => rd_i32(sl + 0x30)? != -1 && lv >= need_lv })
}
/// (B)~(E). `t` = 스테로이드 창 길이. 반환 = score 에 더할 값(st + 보너스). None = 미재현.
#[allow(clippy::too_many_arguments)]
unsafe fn s12_steroid(w: &World, me: usize, tgt: usize, side: u64, role: usize,
                      t: u64, ct: i64, tps: u64, d_est: i64) -> Option<i64> {
    // ── (B) 타깃 역할. 0xd31bb0 이 NULL 이면 블록 전체 스킵
    // ★게임은 `0xd31bb0`(로스터 10칸 → 평행 레코드 배열)을 쓴다. ~~`roster_rec`(w.data 레코드 선형스캔)~~
    //   와 대개 같은 답이지만 다른 배열이라 어긋날 수 있고, 어긋나면 **열 인덱스 tr 이 통째로 바뀌어**
    //   피해행렬 5행이 전부 틀린다(RE 2026-09-07 `0xd5f350`).
    let rec_t = match w.rec_by_roster(rd_u64(tgt + ENT_HANDLE)?) { Some(v) => v, None => return na_tag("ST_rec").map(|_| 0) };
    if rec_t == 0 { return Some(0); }
    let tr = rd_u32(rec_t + REC_ROLE_O) as usize;
    let thp = rd_i64(tgt + ENT_HP)?;
    // ── (C) 피해행렬 합 → st
    if side > 1 { return None; }
    let b_side = w.x + 0x280 + (side as usize) * 0xfa0;
    let mut dmg = rd_i64(b_side + role * 0x320 + tr * 8 + 0x190)?;      // ★자기 항은 col0 만
    let dmg_self = dmg; let mut n_ally = 0i64;
    let (mx, my) = xy(me)?;                                            // ★기준점 = self (tgt 아님)
    for k in 0..5usize {
        if k == role { continue; }
        let e = rd_u64(w.x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + k * 8)? as usize;
        if e == 0 { ALLYD.with(|c| { let mut z = c.get(); z[k * 2 + 1] = -1; c.set(z); }); continue; }
        let (ex, ey) = xy(e)?;
        let d2 = d2_xy(ex, ey, mx, my);
        let bk = b_side + k * 0x320;
        // ★★거부된 슬롯까지 **전량** 남긴다 — 게임이 채택하는 아군을 재현이 빠뜨리고 있다는 것이
        //   RE 의 결론이라(matsum 이 아군 한 명분 부족), 합계만 봐서는 어느 칸인지 알 수 없다.
        //   `z[k*2]` = 그 슬롯의 4열합(채택 여부 무관) · `z[k*2+1]` = 거리(√d2, 임계는 150000)
        let mut per = 0i64;
        for o in [0x190usize, 0x1b8, 0x1e0, 0x208] { per = per.wrapping_add(rd_i64(bk + tr * 8 + o).unwrap_or(0)); }
        ALLYD.with(|c| { let mut z = c.get(); z[k * 2] = per; z[k * 2 + 1] = isqrt_fast(d2) as i64; c.set(z); });
        if d2 > 0x53d1ac100 { continue; }
        n_ally += 1;
        dmg = dmg.wrapping_add(per);
    }
    S12ST.with(|c| { let mut z = c.get(); z[3] = dmg_self * 1000 + n_ally; c.set(z); });
    let den = if tps < 1 { 1 } else { tps };
    let q = ((dmg.wrapping_mul(t as i64) as u64) / den) as i64;        // ★UNSIGNED div
    if thp == 0 { return None; }
    let st_raw = q.wrapping_mul(ct) / thp;                             // ★SIGNED div
    let st = st_raw.min(imm8(SITE_SC_FOCUS_CAP_IMM, 80));
    // ── (D) 버스트(슬롯0~3 즉시딜 합)
    let kind = rd_u64(me + ENT_KIND)?;
    let mut burst: i64 = 0;
    // 슬롯0: p0 || kind∈{0,3} || self.CD0[kind] <= T
    let g0 = match st_p0(me) { Some(v) => v, None => return na_tag("ST_p0").map(|_| 0) }
             || match cd0_off(kind) { None => true, Some(o) => rd_u64(me + o)? <= t };
    if g0 && rd_i32(me + ENT_4C0)? != -1 {
        burst = match est(me + 0x490, me, tgt) { Some(v) => v as i64, None => return na_tag("ST_est0").map(|_| 0) };
    }
    // 슬롯1~3: pN || kind != 13 || self.CDF <= T
    let lv = rd_u64(me + ENT_LEVEL)?;
    let empty = crate::exe_base().wrapping_add(0x3daf38);   // ★호출부 EMPTY(술어 내부의 0x436230 과 다른 정적)
    for (n, sl, cdf) in [(1usize, me + 0x4c8, 0xb8usize),
                         (2, if lv >= 3 { me + 0x500 } else { empty }, 0xc0),
                         (3, if lv >= 5 { me + 0x538 } else { empty }, 0xc8)] {
        let gate = kind != 13 || rd_u64(me + cdf)? <= t
                   || match st_pn(me, n) { Some(v) => v, None => return na_tag("ST_pn").map(|_| 0) };
        if !gate { continue; }
        if rd_i32(sl + 0x30)? == -1 { continue; }
        let sel = rd_u32(sl + 0x28);
        match st_valid_target(sel, me, tgt) { Some(true) => {}, Some(false) => continue,
                                              None => return na_tag("ST_vt").map(|_| 0) }
        burst = burst.wrapping_add(match est(sl, me, tgt) { Some(v) => v as i64,
                                                            None => return na_tag("ST_estN").map(|_| 0) });
    }
    // ── (E) 합산 & 보너스
    let mut out = st;
    burst = burst.wrapping_add(d_est);
    let mut bonus = 0i64;
    let kcap = imm8(SITE_SC_KILL_CAP_IMM, 80);
    if burst >= thp { bonus = ct.min(kcap); }
    else if thp > 0 && burst.wrapping_mul(100) / thp >= imm8(SITE_SC_KILL_PCT_IMM, 60) { bonus = ct.min(kcap) / 3; }
    out += bonus;
    S12ST.with(|c| { let zz = c.get(); c.set([out, t as i64, dmg, zz[3], burst, thp, st, bonus, q, 0, st_raw, 0, zz[12], zz[13], zz[14], zz[15], zz[16], zz[17], zz[18], zz[19]]); });
    Some(out)
}

/// 0xe01450 — 아군 화력 기반 가산(상한 160)
#[allow(clippy::too_many_arguments)]
unsafe fn e01450(w: &World, sim: usize, rec: usize, slot: usize, tgt: usize, c_t: i64, tps: u64) -> Option<i64> {
    let v = match slot_a8(rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize, 0)? { Some(v) => v, None => return Some(0) };
    let (v1, v2, v3, v4, v5) = (v[1] as i64, v[2], v[3] as i64, v[4], v[5]);
    let rec_t = w.roster_rec(rd_u64(tgt + ENT_HANDLE)?)?; if rec_t == 0 { return Some(0); }
    if tps == 0 { return None; }
    let n = (v5 / tps).max(1);
    let side = rd_u64(rec + REC_SIDE)?; if side > 1 { return None; }
    let role_t = rd_u32(rec_t + REC_ROLE_O) as usize;
    let (mut msum, mut dps) = (0u64, 0u64);
    near_allies(w, side, tgt, None, |e, r3| {
        let b = match mat(w, r3) { Some(b) => b, None => return None };
        for o in [0x190usize, 0x1b8, 0x1e0, 0x208] { msum = msum.wrapping_add(rd_u64(b + role_t * 8 + o)?); }
        dps = dps.wrapping_add(tps.wrapping_mul(100) / atk_iv2(e)?);
        Some(())
    })?;
    let cap2 = rd_u64(tgt + ENT_MAXHP)?.wrapping_mul(2);
    let ms = { let x = msum.wrapping_mul(n); x.min(cap2) };
    let dp = dps.wrapping_mul(n) / 100;
    let dp = if v4 != 0 { dp.min(v4) } else { dp };
    let acc = (dp.wrapping_mul(v3 as u64) as i64).wrapping_add(v1).wrapping_add((ms.wrapping_mul(v2) / 100) as i64);
    let acc = acc.min(cap2 as i64);
    let hp = { let h = rd_i64(tgt + ENT_HP)?; if h >= 2 { h } else { 1 } };
    let _ = sim;
    Some((acc.wrapping_mul(c_t) / hp).min(imm32(SITE_BV_CAP_MAIN_IMM, 160)))
}
/// 0xe019d0 — 자기 스테로이드 기여(상한 80)
unsafe fn e019d0(w: &World, sim: usize, slot: usize, tgt: usize, c_t: i64, tps: u64) -> Option<i64> {
    let (t, f1, f2) = slot_c8(rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize, 0)?;
    if f2 == 2 { return Some(0); }
    let rec_t = w.roster_rec(rd_u64(tgt + ENT_HANDLE)?)?; if rec_t == 0 { return Some(0); }
    if tps == 0 { return None; }
    let b = mat(w, rec_t)?;
    let n = (t / tps).max(1);
    let mut acc = 0u64;
    if f1 != 0 { let mut x = 0u64; for k in 0..5usize { x = x.wrapping_add(rd_u64(b + 0x190 + k * 8)?); } acc = x / 10; }
    if f2 & 1 == 1 { let mut x = 0u64; for k in 0..15usize { x = x.wrapping_add(rd_u64(b + 0x1b8 + k * 8)?); } acc = acc.wrapping_add(x / 10); }
    acc = acc.wrapping_mul(n).min(rd_u64(tgt + ENT_MAXHP)?);
    let hp = { let h = rd_i64(tgt + ENT_HP)?; if h >= 2 { h } else { 1 } };
    let _ = sim;
    Some(((acc.wrapping_mul(c_t as u64) as i64) / hp / 2).min(imm8(SITE_BV_CAP_HALF_E019D0_IMM, 80)))
}
/// 0xe02020 — 아군 스테로이드 기여(상한 80, 최종 감산 항)
#[allow(clippy::too_many_arguments)]
unsafe fn e02020(w: &World, sim: usize, rec: usize, slot: usize, me: usize, tgt: usize, c_t: i64, tps: u64) -> Option<i64> {
    let (sd, sv) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    if !slot_c0(sd, sv, 0)? { return Some(0); }
    let (ok, t) = slot_88(sd, sv, 0)?; if !ok { return Some(0); }
    if tps == 0 { return None; }
    let n = (t / tps).max(1);
    let rec_t = w.roster_rec(rd_u64(tgt + ENT_HANDLE)?)?; if rec_t == 0 { return Some(0); }
    let side = rd_u64(rec + REC_SIDE)?; if side > 1 { return None; }
    let role_t = rd_u32(rec_t + REC_ROLE_O) as usize;
    let me_h = rd_u64(me + ENT_HANDLE)?;
    let mut acc = 0u64;
    near_allies(w, side, tgt, Some(me_h), |_e, r3| {
        let b = match mat(w, r3) { Some(b) => b, None => return None };
        for o in [0x190usize, 0x1b8, 0x1e0, 0x208] { acc = acc.wrapping_add(rd_u64(b + role_t * 8 + o)?); }
        Some(())
    })?;
    acc = acc.wrapping_mul(n).min(rd_u64(tgt + ENT_MAXHP)?);
    let hp = { let h = rd_i64(tgt + ENT_HP)?; if h >= 2 { h } else { 1 } };
    let _ = sim;
    Some(((acc.wrapping_mul(c_t as u64) as i64) / hp / 2).min(imm8(SITE_BV_CAP_HALF_E02020_IMM, 80)))
}

/// 본체. 반환 None = 미재현(NA).
#[allow(clippy::too_many_arguments)]
pub unsafe fn combat_score(mode: usize, _prof: usize, rec: usize, ctx: usize, bb: usize, sp: usize, slot: usize, tgt: usize, p9: usize) -> Option<i64> {
    TAGGED.with(|c| c.set(false)); stg(tag8("S0"));
    pth_set("?");
    // ★진단 TLS 는 호출마다 초기화한다 — 안 하면 조기반환 경로에서 직전 호출의 값이 그대로 찍혀
    //   원인 분석이 통째로 헛돈다(2026-09-07 실측: st=76 인데 main=0 인 모순 로그).
    S12ST.with(|c| c.set([0; 20])); ALLYD.with(|c| c.set([0; 10])); S5D.with(|c| c.set([0; 21])); S12D.with(|c| c.set([0; 8]));
    let r = combat_score_inner(mode, _prof, rec, ctx, bb, sp, slot, tgt, p9);
    if r.is_none() && !TAGGED.with(|c| c.get()) { let t = STG.with(|c| c.get()); na(t); }
    r
}
unsafe fn combat_score_inner(mode: usize, _prof: usize, rec: usize, ctx: usize, bb: usize, sp: usize, slot: usize, tgt: usize, _p9: usize) -> Option<i64> {
    if !ptr_ok(rec) || !ptr_ok(ctx) || !ptr_ok(bb) || !ptr_ok(sp) || !ptr_ok(slot) || !ptr_ok(tgt) { return None; }
    // ── S0 프롤로그 ──
    let side = rd_u64(rec + REC_SIDE)?; if side > 1 { return None; }
    let role = rd_u32(rec + REC_ROLE_O);
    let wroot = rd_u64(ctx)? as usize; let sim = rd_u64(ctx + 8)? as usize; let agents = rd_u64(ctx + 0x10)? as usize;
    if !ptr_ok(wroot) || !ptr_ok(sim) || !ptr_ok(agents) { return None; }
    let w = World { x: wroot, data: rd_u64(wroot)? as usize, vt: rd_u64(wroot + 8)? as usize };
    if !ptr_ok(w.data) || !ptr_ok(w.vt) { return None; }
    let me = rd_u64(w.x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + (role as usize) * 8)? as usize;
    if me == 0 { return None; }
    let cfg = rd_u64(sim + 8)? as usize; if !ptr_ok(cfg) { return None; }
    let tps = rd_u64(cfg + CFG_TPS)?;
    let now = rd_u64(w.data + W_TICK)?;
    let (hp, maxhp) = (rd_u64(me + ENT_HP)?, rd_u64(me + ENT_MAXHP)?);
    let urgent = if rd_u8(me + ENT_488) != 0 { true }
        else if mode < 2 { false }
        else {
            let b = dn_cache::bits(rec, ctx)?;   // 0xc87850 순수 재현(bit0|8|16)
            if b & 0x100 != 0 { true }
            else if hp.wrapping_mul(100) <= maxhp.max(1).wrapping_mul(35) { false }
            else { b & 0x10001 != 0 }
        };
    // ── S1 특수형 조기반환(0xe04400) ──
    stg(tag8("S1"));
    if let Some(v) = special_early(ctx, rec, me, sp, tgt)? { return Some(v); }
    let tgt_kind = rd_i32(tgt + ENT_KIND)?;
    // ★게임은 **핸들 비교**다(`0xd5e52d cmp rsi,[rbp+0x618]` = tgt.0x5c0 vs me.0x5c0).
    //   ~~포인터 비교(`me == tgt`)~~ 는 같은 유닛의 다른 스냅샷이 오면 false 가 되어
    //   자기대상 경로(0xd5e53a)를 S15z 로 오분류한다(RE 2026-09-07).
    set_leaf_me(me);
    let self_is_tgt = rd_u64(me + ENT_HANDLE)? == rd_u64(tgt + ENT_HANDLE)?;
    // ── S2 사거리 게이트 ──
    stg(tag8("S2"));
    // ★★게이트는 self 판정이 아니라 **"다른 팀"** 판정이다(`0xd5be5d~0xd5be78`).
    //   `same_owner ⟺ (a0==b0) && (a0!=0 || a8==b8)` — S4 게이트(`0xd5ca6b`, L866)와 명령 시퀀스가 동일한데
    //   S2 에만 정정이 안 들어가 있었다. ~~`!self_is_tgt`~~ 는 **아군 타깃**에서 게임은 S2 를 통째로 건너뛰는데
    //   재현만 진입해 −9,999,999 를 냈다 — 잔여 DIFF 809건의 원인(RE 2026-09-07).
    let s2_other_team = {
        let (a0, a8) = (rd_u64(me)?, rd_u64(me + 8)?);
        let (b0, b8) = (rd_u64(tgt)?, rd_u64(tgt + 8)?);
        !(a0 == b0 && (a0 != 0 || a8 == b8))
    };
    if mode > 1 && tgt_kind == 13 && s2_other_team {
        let (mtag, _) = w.mode()?;
        if mtag != 2 {
            let d = dist(me, tgt)?; let r = reach(me, slot, tgt)?;
            if r < d {
                // ★게임은 `test byte[rbp+0x808],1`(0xd5c369) — **bit0 만** 본다.
                //   ~~`!= 0`~~ 은 0x1500 이 2/4 일 때 재현만 리젝트한다(RE 2026-09-07).
                if rd_u8(bb + BB_1500) & 1 != 0 { return Some(-9_999_999); }
                if !urgent {
                    if let Some((_, _, t)) = approach(ctx, me, tgt, slot)? {
                        if t != 0 {
                            // 0xca21c0: 적 로스터 중 150,000 이내 · (보임 ∨ 최근 120틱 목격)
                            let mut vis: [usize; 8] = [0; 8]; let mut n = 0usize;
                            for i in 0..5usize {
                                let e = rd_u64(w.x + X_ROSTER + ((1 - side) as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize;
                                if e == 0 || n >= 8 { continue; }
                                // ★게임은 `ja` = `d2 > 150000²` 일 때만 제외 — 정확히 150,000 은 **통과**
                                if d2_ee(e, me)? > 0x53d1ac100 { continue; }
                                let h = rd_u64(e + ENT_HANDLE)?;
                                let ok = w.visible(side, h)? || {
                                    let rc = w.roster_rec(h)?;
                                    rc != 0 && rd_u64(agents + ((1 - side) as usize) * LANE_STRIDE + LANE_ROSTER + (rd_u32(rc + REC_ROLE_O) as usize) * 8)?.wrapping_add(120) >= now
                                };
                                if ok { vis[n] = e; n += 1; }
                            }
                            if n > 0 {
                                let mut hdr = [0u64; 4]; hdr[0] = vis.as_ptr() as u64; hdr[3] = n as u64;
                                let mut emp = [0u64; 4]; emp[0] = vis.as_ptr() as u64;
                                let fc = super::fight_check::fight_check_memo(mode as u64, ctx, rec, me, hdr.as_ptr() as usize, emp.as_ptr() as usize)?;
                                // ★`0xd5c4e4 ja` = **부호 없는** 비교. ~~`as i64` 부호있는 비교~~ 은
                                //   fight_check 가 큰 u64(센티넬)를 돌려줄 때 게임과 반대로 갈린다(RE 2026-09-07).
                                if fc <= cast_delay0(sp)?.wrapping_add(t) { pth_set("S2"); return Some(-9_999_999); }
                            }
                        }
                    }
                }
            }
        }
    }
    // ── S3 위협·적 구조물 스캔 ──
    stg(tag8("S3"));
    let cast_delay = cast_delay0(sp)?;
    CDLY.with(|c| c.set(cast_delay as i64));
    let thr = threat_sum(bb + BB_R, cast_delay.wrapping_add(30))?;
    let my_handle = rd_u64(me + ENT_HANDLE)?;
    let seen = w.visible(1 - side, my_handle)?;
    let thr_s = if seen { thr } else { thr / 3 };
    // ★threat 리스트 첫 엔트리(delay·amount)와 visible 을 계측 — thrlen=1 인데 thr=0 인 표본을 가르기 위함
    S5D.with(|c| { let mut z = c.get();
        z[7] = seen as i64;
        if rd_u64(bb + BB_R + AS_REC_THR_LEN).unwrap_or(0) > 0 {
            if let Some(pp) = rd_u64(bb + BB_R + AS_REC_THR_PTR) {
                z[8] = rd_u64(pp as usize + 8).unwrap_or(0) as i64;      // delay
                z[9] = rd_i64(pp as usize + 0x10).unwrap_or(0);          // amount
            } }
        c.set(z); });
    // ★구조물 경로(0xd5d762→0xd5e7e4→0xd5e913)에서는 **타깃 종류를 보지 않고** bb.0x9b0 을 넣는다.
    //   ~~`bonus9b0 = tgt_kind==13 ? bb.0x9b0 : 0`~~ 이 최대 불일치 원인이었다(2026-09-07 확정, 로그 역산으로 검증).
    //   r_t 에는 `slot_e8(near..)` 항이 없고, 비교는 `<=`, extra 의 reach 는 vt+0xe8 3번째 인자가 **near** 다.
    let raw9b0 = rd_i64(bb + BB_9B0)?;
    let mut d5 = [0i64; 19]; d5[6] = raw9b0;
    let mut bonus9b0 = if tgt_kind == 13 { raw9b0 } else { 0 };
    let mut safe = true;
    if let Some((near, _)) = nearest_in_chain(&w, 1 - side, me)? {
        d5[0] = 1; d5[1] = rd_i32(near + ENT_KIND)? as i64; d5[2] = rd_u64(near + 0x88)? as i64;
        if rd_i32(near + ENT_KIND)? == 2 && rd_u64(near + 0x88)? == 0 {
            let extra = { let d = dist(tgt, me)? as i64; let r = reach_e8(me, slot, tgt, near)? as i64; (d - r).max(0) as u64 };
            let r_t = rd_u64(near + ENT_F438)?.wrapping_add(rd_u64(near + 0x4a0)?)
                .wrapping_add(rd_u64(near + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(rd_u64(near + 0x4a8)?))
                .wrapping_add(rng_of(near)?).wrapping_add(rng_of(me)?)
                .wrapping_add(imm32(SITE_SC_DIVE_MARGIN_IMM, 15000) as u64).wrapping_add(extra);
            d5[3] = dist(me, near)? as i64; d5[4] = r_t as i64;
            safe = dist(me, near)? <= r_t;
            d5[5] = safe as i64;
            bonus9b0 = if safe || tgt_kind == 13 { rd_i64(bb + BB_9B0)? } else { 0 };
            safe = safe || tgt_kind == 13;
        }
    }
    // ── S4 추격 위험 ──
    stg(tag8("S4"));
    let phase = rd_u8(sim + 0x38);
    let mut chase: i64 = 0;
    // ★게이트 = `다른 팀 && tgt.kind==13`(0xd5ca6b~0xd5cb47). ~~`!self_is_tgt && safe`~~ 는 우연히 비슷했을 뿐(2026-09-07 정정)
    let other_team = {
        let (a0, a8) = (rd_u64(me)?, rd_u64(me + 8)?);
        let (b0, b8) = (rd_u64(tgt)?, rd_u64(tgt + 8)?);
        !(a0 == b0 && (a0 != 0 || a8 == b8))
    };
    S5D.with(|c| { let mut z = c.get();
        z[17] = tgt_kind as i64;
        z[18] = (matches!(phase, 0 | 5 | 7 | 8) as i64)
              | ((rd_u64(cfg + CFG_8A8).unwrap_or(u64::MAX).saturating_sub(30u64.wrapping_mul(tps)) <= now) as i64) << 1
              | (other_team as i64) << 2 | ((tgt_kind == 13) as i64) << 3;
        c.set(z); });
    if matches!(phase, 0 | 5 | 7 | 8) && rd_u64(cfg + CFG_8A8)?.saturating_sub(30u64.wrapping_mul(tps)) <= now && other_team && tgt_kind == 13 {
        if let Some((ax, ay, t)) = approach(ctx, me, tgt, slot)? {
            let h = (cast_delay.wrapping_add(t)).clamp(tps / 2, 2 * tps);
            let (mx, my) = xy(me)?;
            let a = super::position_eval::exposure(w.x, me, mx, my, h, tps)?;
            let b = super::position_eval::exposure(w.x, me, ax as u64, ay as u64, h, tps)?;
            let hp_pct = hp.wrapping_mul(100) / maxhp.max(1);
            let b_pct = b.wrapping_mul(100) / hp.max(1);
            let keep = b != 0 && (hp <= b || b_pct > 49 || (hp_pct < 66 && b_pct > 29) || (hp_pct < 41 && b_pct > 17) || (hp_pct < 26 && b_pct > 9));
            chase = ((if keep { b } else { 0 }) as i64).wrapping_sub((a / 2) as i64).max(0);
        }
    }
    // ── S5 위험항 ──
    stg(tag8("S5"));
    let c = pct_c(bb, bb + BB_R)?; let c_val = c;
    if hp == 0 { return None; }
    let risk_neg: i64 = if urgent { -1 } else {
        let inner = (rd_i64(bb + BB_998)?).wrapping_add(chase).wrapping_add(bonus9b0).wrapping_add(thr_s);
        !(inner.wrapping_mul(c) / hp as i64)
    };
    // ── S6 아군 구조물·적 근접 ──
    stg(tag8("S6"));
    let near_ally = nearest_in_chain(&w, side, me)?;
    let enemy_near = enemy_visible_near(&w, agents, side, me, 0x37e11d600, now)?;
    // ── S7 타워 지원 ──
    stg(tag8("S7"));
    let mut tower_support: i64 = 0;
    // ★`1-side` 실험 폐기 — RE 2026-09-07 이 `0xd5cf83`(S7) 은 `side`, `0xd5c993`(S3) 은 `1-side` 임을
    //   바이트로 확정했고 재현이 이미 그렇게 분리돼 있다. 게이트 4항목도 전부 "동일" 판정.
    if let Some((na_ent, _)) = near_ally {
        let rt = rd_u64(na_ent + ENT_F438)?.wrapping_add(rd_u64(na_ent + 0x4a0)?)
            .wrapping_add(rd_u64(na_ent + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(rd_u64(na_ent + 0x4a8)?))
            .wrapping_add(if rd_i32(na_ent + ENT_4C0)? == 0 { rng_of(na_ent)? } else { 0 })
            .wrapping_add(rng_of(tgt)?).wrapping_add(slot_e8(na_ent + 0x490, na_ent, tgt)?);
        if enemy_near && d2_ee(tgt, na_ent)? <= sq(rt) && now < rd_u64(cfg + CFG_13F8)? {
            let thp = rd_u64(tgt + ENT_HP)?; if thp == 0 { return None; }
            // ★게임은 **부호 없는** 나눗셈(`div`)이다 — RE 2026-09-07
            tower_support = ((est(na_ent + 0x490, na_ent, tgt)?.wrapping_mul(100) / thp) as i64).min(100);
        }
    }
    // ── S8 위치항 ──
    stg(tag8("S8"));
    let mut pos_term: i64 = 0;
    let pgate = match slot_pos_gate(slot) { Some(v) => v, None => return na(tag8("S8gate")) };
    S5D.with(|c| { let mut z = c.get(); z[14] = pgate as i64; z[15] = d2_ee(me, tgt).unwrap_or(0) as i64; c.set(z); });
    if pgate && d2_ee(me, tgt)? >= 0x49040441 {
        let (tx, ty) = xy(tgt)?;
        let (cx, cy) = ((tx / 32000).min(29), (ty / 32000).min(29));
        let (qx8, qy8) = (cx * 32000 + 16000, cy * 32000 + 16000);
        // 게임은 래퍼 0xd84db0 을 부른다 = TLS 메모 경유. 본체 재현이 None 이면 게임이 방금 채워 둔 메모 표를 읽는다.
        let a8 = match super::position_eval::position_eval(mode as u64, rec, ctx, qx8, qy8, 0xc) {
            Some(o) => o.a,
            None => match super::position_eval::memo_lookup(mode, rec, ctx, qx8 as usize, qy8 as usize, 0xc) { Some(w) => w[0] as i64,
                // ★position_eval 이 어느 블록에서 None 을 냈는지까지 태그에 담는다(판당 3,322건의 원인 특정용)
                None => { let r = super::position_eval::last_na(); return na(if r != 0 { r } else { tag8("S8pe") }) } },
        };
        // ★게임은 위치항을 **뺀다**(실측: mine − game == 2*pos 가 3표본 정확히 일치, 2026-09-07)
        S5D.with(|c| { let mut z = c.get(); z[16] = a8; c.set(z); });
        pos_term = -(a8.wrapping_mul(c) / 100);
    }
    // ── S9 오판 플래그 ──
    stg(tag8("S9"));
    let mis = if rd_i32(rec + REC_464)? == 1 {
        let (a, b) = misjudge(rd_u64(w.data + 0xec90)?, rd_u64(rec + REC_SEED2)?, now, tps,
                              rd_u64(rec + REC_208)?.min(100), rd_u64(rec + REC_210)?.min(100), rd_u64(rec + REC_448)?);
        a || b
    } else { false };
    // ── S10 처형 경로 ──
    stg(tag8("S10"));
    if sp_vt_b8(sp)? {
        if d2_ee(me, tgt)? <= sq(reach(me, slot, tgt)?) {
            let th = rd_u64(tgt + ENT_HANDLE)?;
            let rec_t = find_rec(bb, BB_ENEMY_PTR, BB_ENEMY_LEN, th)?;
            // ★경로 선택(S12/S14/S13/S15z)은 이 두 Record 조회가 좌우한다 — 여섯 항이 전부 0 인데
            //   게임 main 이 4 라면 **게임이 다른 경로를 탔다**는 뜻이므로, 조회 입력을 찍는다(2026-09-07).
            S5D.with(|c| { let mut z = c.get();
                z[19] = rd_u64(bb + BB_ENEMY_LEN).unwrap_or(0) as i64;
                z[20] = rd_u64(bb + 0x14d0).unwrap_or(0) as i64;   // 아군 Record len
                c.set(z); });
            let dv = dist(me, tgt)? as i64;
            // ★적 Record 에 없으면 게임은 **조기반환하지 않고 S11(0xd5de82)로 폴백**한다(RE 0xd5e393, 2026-09-07).
            //   구조물 타깃은 bb.0x14d8 에 절대 없으므로 예전 `_ => 100 + …` 폴백이 통째로 오답이었다.
            if rec_t.is_none() { /* fall through to S11 */ } else {
            pth_set("S10"); return Some(match rec_t {
                Some(rt) if !mis => {
                    let d = est(slot, me, tgt)? as i64;
                    let x = threat_sum(rt, cast_delay.wrapping_add(30))?.wrapping_add(rd_i64(rt + RT_170)?);
                    let ct = pct_c(bb, rt)?;
                    let mut v = x / 4 + rd_i64(rt + RT_KILL)? / 2 + d;
                    if rd_u8(tgt + ENT_488) == 1 { v = v.min((rd_i64(tgt + ENT_HP)? - 1).max(0)); }
                    let thp = rd_i64(tgt + ENT_HP)?; if thp == 0 { return None; }
                    100 + v.wrapping_mul(ct) / thp
                }
                _ => 100 + (150000 - dv).max(0) / 1500,      // mis 갈래(레코드는 있음)
            }); }
        }
    }
    // ── S11~S14 ──
    stg(tag8("S11"));
    let th = rd_u64(tgt + ENT_HANDLE)?;
    let rec_t = find_rec(bb, BB_ENEMY_PTR, BB_ENEMY_LEN, th)?;
    let rec_a = find_rec(bb, BB_ALLY_PTR, BB_ALLY_LEN, th)?;
    // ★S15z 가 정말 "양쪽 미발견" 인지 계측 — 아군 Vec 길이와 첫 두 핸들, 그리고 찾는 핸들
    S5D.with(|c| { let mut z = c.get();
        z[10] = rd_u64(bb + BB_ALLY_LEN).unwrap_or(u64::MAX) as i64;
        z[11] = (th & 0xffff_ffff) as i64;
        if let Some(pp) = rd_u64(bb + BB_ALLY_PTR) { let pp = pp as usize;
            if z[10] > 0 { z[12] = (rd_u64(pp + RT_HANDLE).unwrap_or(0) & 0xffff_ffff) as i64; }
            if z[10] > 1 { z[13] = (rd_u64(pp + AS_REC_STRIDE + RT_HANDLE).unwrap_or(0) & 0xffff_ffff) as i64; } }
        c.set(z); });
    // S11 오판 경로: mis && 적 레코드 존재 → 거리 감쇠항만 (0xd5de97~0xd5df15)
    if mis && rec_t.is_some() {
        let dv = dist(me, tgt)? as i64;
        pth_set("S11mis"); return Some(risk_neg + tower_support + pos_term + (150000 - dv).max(0) / 1500);
    }
    let main: i64 = if let Some(rt) = rec_t {
        // ── S12 적 타깃 ──
        stg(tag8("S12"));
        let d = match est(slot, me, tgt) { Some(v) => v as i64, None => return na(tag8("S12_est")) };
        let x = match threat_sum(rt, cast_delay.wrapping_add(30)) { Some(v) => v.wrapping_add(rd_i64(rt + RT_170)?), None => return na(tag8("S12_thr")) };
        let ct = match pct_c(bb, rt) { Some(v) => v, None => return na(tag8("S12_ct")) };
        let kill = rd_i64(rt + RT_KILL)?;
        let thp = rd_i64(tgt + ENT_HP)?; if thp == 0 { return None; }
        let mut v = x / 4 + kill / 2 + d;
        // ★★게임의 `v` 는 재현보다 크다 — 실측 4표본(1/12/14/18)에서 각각 +50/+50/+60/+80 을 넣으면
        //   게임 반환값과 **정확히** 일치한다(12·14·18 은 오차 0, 1 은 thp 불확실성 안). 그 정체를 특정하려고
        //   재현이 이미 갖고 있는 후보들을 전부 찍는다: 평타추정(0x490)·슬롯1~3 추정.
        S12ST.with(|c| { let mut z = c.get();
            z[16] = est(me + 0x490, me, tgt).unwrap_or(u64::MAX) as i64;
            z[17] = thp; z[18] = v;
            let lv = rd_u64(me + ENT_LEVEL).unwrap_or(1);
            z[19] = est(me + 0x4c8, me, tgt).unwrap_or(0) as i64
                  + if lv >= 3 { est(me + 0x500, me, tgt).unwrap_or(0) as i64 } else { 0 }
                  + if lv >= 5 { est(me + 0x538, me, tgt).unwrap_or(0) as i64 } else { 0 };
            c.set(z); });
        if rd_u8(tgt + ENT_488) != 0 { v = v.min((thp - 1).max(0)); }
        let mut score = v.wrapping_mul(ct) / thp;
        if safe {
            let cap = ct.min(100);
            if d >= thp { score += cap; }
            else if kill / 2 + d >= thp { score += 3 * cap / 4; }
            else if (est(me + 0x490, me, tgt)? as i64) + d >= thp { score += cap / 2; }
        }
        // 스테로이드 창: sp.vt+0x68 의 dyn Any 가 스테로이드형이면 그 T, 아니면 slot.vt+0x88 의 (ok, T)
        let (sd, sv) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
        let mut st_add: i64 = 0;
        let pre_st = score;                 // 스테로이드 창 전의 score
        // (A) T 결정: sp.vt+0x68 의 dyn Any TypeId 가 0x33d4f70 이고 p.0x10 != 0 이면 그 값, 아니면 slot.vt+0x88 폴백
        let mut t_win: Option<u64> = None;
        // ★게임은 TypeId **16바이트 내용**을 비교한다(`pcmpeqb`+`pmovmskb == 0xffff`, 0xd5f2e9).
        //   ~~상수의 주소(RVA 0x3d4f70)와 비교~~ 하던 것은 틀렸다 — 실측 TypeId 주소가 56종이나
        //   나오고 그 중 0x3d4f70 은 **하나도 없었다**(최빈 0x3412350 이 32만 회). 2026-09-07.
        if sp_type_id_matches(sp) {
            let v = rd_u64(rd_u64(sp)? as usize + 0x10)?;
            if v != 0 { t_win = Some(v); }
        }
        if t_win.is_none() {
            let (ok88, t88) = match slot_88(sd, sv, 0) { Some(v) => v, None => return na(tag8("S12_88")) };
            if ok88 { t_win = Some(t88); }
            // ★T=0 은 거의 확실히 오답이다(실측 4표본 역산이 전부 T=60 으로 수렴). 어느 impl 이
            //   (true, 0) 을 냈는지 표식으로 특정한다 — slot_88 안엔 T 를 임의로 0 으로 두는 arm 이 몇 개 있다.
            if ok88 && t88 == 0 { super::dyn_eff::unseen(0x694, super::dyn_eff::impl_rva(sv, 0x88).unwrap_or(0)); }
        }
        // ★계측: t_win 이 None 인 이유를 특정한다(잔여 DIFF 의 유력 후보 = 스테로이드 창 미검출)
        S12ST.with(|c| { let mut z = c.get();
            z[12] = super::dyn_eff::impl_rva(sv, 0x88).unwrap_or(0) as i64;
            z[13] = sp_type_id(sp).unwrap_or(0) as i64;
            z[14] = (sp_type_id_matches(sp) as i64) * 2 + (rd_u64(sp).and_then(|q| rd_u64(q as usize + 0x10)).unwrap_or(0) != 0) as i64;
            z[15] = rd_u64(sp).and_then(|q| rd_u64(q as usize + 0x10)).unwrap_or(0) as i64;
            c.set(z); });
        if let Some(tw) = t_win {
            match s12_steroid(&w, me, tgt, side, role as usize, tw, ct, tps, d) {
                Some(v) => { st_add = v; score += v; }
                None => return na(tag8("S12ster")),
            }
        }
        let a1 = match e01450(&w, sim, rec, slot, tgt, ct, tps) { Some(v) => v, None => return na(tag8("S12_1450")) };
        let a2 = match e019d0(&w, sim, slot, tgt, ct, tps) { Some(v) => v, None => return na(tag8("S12_19d0")) };
        let a3 = match e02020(&w, sim, rec, slot, me, tgt, ct, tps) { Some(v) => v, None => return na(tag8("S12_2020")) };
        S12D.with(|c| c.set([d, x, ct, kill, score, a1, a2, a3])); let _ = st_add;
        S12ST.with(|c| { let mut z = c.get(); z[9] = pre_st * 4 + safe as i64 * 2 + (t_win.is_some()) as i64; z[11] = a1 + a2 - a3; c.set(z); });
        score + a1 + a2 - a3
    } else { 0 };
    if rec_t.is_some() { let _ = main; }
    if rec_t.is_none() && (rec_a.is_some() || self_is_tgt) {
        stg(tag8("S13x"));
        // ★S15 는 `risk_neg + tower_support + pos_term + main` 이다 — s13_s14 는 main 만 돌려준다
        let bc = super::buff_value::BCtx { mode, prof: _prof, rec, ctx, bb, sp, slot, tgt, me, p9: _p9,
                                           sim, w: w.x, c: c_val, inc_base: bonus9b0.wrapping_add(thr_s), cast_delay };
        let m = super::buff_value::s13_s14(&bc, rec_a)?;
        // LAST 를 이 경로에서도 채운다(안 그러면 DIFF 로그의 risk_neg/tower/pos 가 직전 호출의 잔값이다)
        LAST.with(|c| c.set([risk_neg, tower_support, pos_term, m, urgent as i64, c_val, thr_s, chase,
                             rd_i64(bb + BB_998).unwrap_or(-1), bonus9b0, cast_delay as i64, hp as i64, thr,
                             rd_u64(bb + BB_R + AS_REC_THR_LEN).unwrap_or(0) as i64,
                             crate::judge::cap_as_d83230::last().map(|v| v as i64).unwrap_or(-999),
                             rd_i64(bb + 0x970).unwrap_or(0), rd_i64(bb + 0x9a0).unwrap_or(0), rd_i64(bb + 0x988).unwrap_or(0)]));
        pth_set(if rec_a.is_some() { "S14" } else { "S13" }); return Some(risk_neg + tower_support + pos_term + m);
    }
    let _ = (bonus9b0, thr_s, sp, my_handle, seen);
    LAST.with(|c| c.set([risk_neg, tower_support, pos_term, main, urgent as i64, c_val, thr_s, chase, rd_i64(bb + BB_998).unwrap_or(-1), bonus9b0, cast_delay as i64, hp as i64, thr, rd_u64(bb + BB_R + AS_REC_THR_LEN).unwrap_or(0) as i64, crate::judge::cap_as_d83230::last().map(|v| v as i64).unwrap_or(-999), rd_i64(bb + 0x970).unwrap_or(0), rd_i64(bb + 0x9a0).unwrap_or(0), rd_i64(bb + 0x988).unwrap_or(0)]));
    pth_set(if rec_t.is_some() { "S12" } else { "S15z" });
    Some(risk_neg + tower_support + pos_term + main)
}

// ── 아직 못 옮긴 dyn 게터(도달 빈도만 집계) ─────────────────────────────────────────────
/// sp.vt+0x80 (시전 지연, 추정)
#[inline] unsafe fn cast_delay0(sp: usize) -> Option<u64> { sp_vt80(sp) }
unsafe fn sp_vt80(sp: usize) -> Option<u64> {
    let (d, v) = (rd_u64(sp)? as usize, rd_u64(sp + 8)? as usize);
    let f = rd_u64(v + 0x80)? as usize;
    if let Some(x) = super::as_callees::decode_getter(f, d) { return Some(x); }
    if let Some(r) = super::dyn_eff::impl_rva(v, 0x80) { super::dyn_eff::unseen(0x480, r); }
    None
}
/// sp.vt+0xb8 → 처형 경로 bool
unsafe fn sp_vt_b8(sp: usize) -> Option<bool> {
    let (d, v) = (rd_u64(sp)? as usize, rd_u64(sp + 8)? as usize);
    let f = rd_u64(v + 0xb8)? as usize;
    if let Some(x) = super::as_callees::decode_getter(f, d) { return Some(x & 1 == 1); }
    if let Some(r) = super::dyn_eff::impl_rva(v, 0xb8) { super::dyn_eff::unseen(0x4b8, r); }
    None
}
/// slot.vt+0x60 / +0x68 (위치항 게이트)
unsafe fn slot_pos_gate(slot: usize) -> Option<bool> {
    let (d, v) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    let a = super::as_callees::eff_bool(d, v, 0x68, 0)?;
    if a { return Some(true); }
    super::as_callees::eff_bool(d, v, 0x60, 0)
}
// ── 0xe04400 특수형 조기반환 ─────────────────────────────────────────────────────────────
//   ⬜순수 경계(임시): 판정이 `sp.vt+0x68` 이 돌려주는 dyn Any 의 **type_id** 로 갈리는데, 그 게터 impl 이 수십 종이라
//   (judge_dyn 실측: slot+0x468 에 0x115c7c0 68만 · 0x17c4f00 7.7만 …) 전부 옮기기 전까지는 게임 반환을 캡처해 쓴다.
//   호출자는 combat_score(0xd5bde8) 하나뿐이므로 "직전 1건" 이 곧 이번 호출의 결과다(exe 전역 call 사이트 전수 확인).
thread_local! { static E04400: std::cell::Cell<(usize, u64, u64)> = const { std::cell::Cell::new((0, 0, 0)) }; }
pub fn e04400_record(sp: usize, tag: u64, val: u64) { E04400.with(|c| c.set((sp, tag, val))); }
/// `sp.vt+0x68` 이 돌려주는 dyn Any 의 TypeId RVA. 두 단계 모두 **정적 게터**라 기계어를 디코드해 재현한다.
///   ① impl: `48 89 c8`(mov rax,rcx) + `48 8d 15 d32`(lea rdx,[rip+d]) + ret → vt = f+10+d
///   ② vt+0x18: `48 89 c8` + `0f 10 05 d32`(movups xmm0,[rip+d]) + `0f 11 01` + ret → TypeId 주소 = g+10+d
unsafe fn sp_type_id(sp: usize) -> Option<usize> {
    let v = rd_u64(sp + 8)? as usize; if !ptr_ok(v) { return None; }
    let f = rd_u64(v + 0x68)? as usize; if !ptr_ok(f) { return None; }
    if !(rd_u8(f) == 0x48 && rd_u8(f + 1) == 0x89 && rd_u8(f + 2) == 0xc8 && rd_u8(f + 3) == 0x48 && rd_u8(f + 4) == 0x8d && rd_u8(f + 5) == 0x15) {
        if let Some(r) = super::dyn_eff::impl_rva(v, 0x68) { super::dyn_eff::unseen(0x468, r); } return None;
    }
    let vt = (f as isize + 10 + rd_i32(f + 6)? as isize) as usize; if !ptr_ok(vt) { return None; }
    let g = rd_u64(vt + 0x18)? as usize; if !ptr_ok(g) { return None; }
    if !(rd_u8(g) == 0x48 && rd_u8(g + 1) == 0x89 && rd_u8(g + 2) == 0xc8 && rd_u8(g + 3) == 0x0f && rd_u8(g + 4) == 0x10 && rd_u8(g + 5) == 0x05) {
        super::dyn_eff::unseen(0x418, g.wrapping_sub(crate::exe_base())); return None;
    }
    let tid = (g as isize + 10 + rd_i32(g + 6)? as isize) as usize;
    let b = crate::exe_base(); if b == 0 || tid <= b { return None; }
    Some(tid - b)
}
/// 위 TypeId 가 스테로이드형 상수(`0x1433d4f70`)와 **16바이트 동일**한가.
unsafe fn sp_type_id_matches(sp: usize) -> bool {
    let b = crate::exe_base(); if b == 0 { return false; }
    // ★★상수의 진짜 RVA = `0x33d4f70` (`0xd5f2f1: pcmpeqb xmm0,[rip+0x2675c77]`, RE 2026-09-07).
    //   ~~`0x3d4f70`~~ 은 자릿수 하나가 빠진 오기라 **`.text` 한복판**을 가리켰다 — 그래서 이 비교가
    //   **한 번도 참이 될 수 없었고**, 스테로이드 창이 (A) 경로로는 절대 안 열렸다.
    //   16바이트 = 80 9A 7A FF E5 89 EC C3 / 6A 4E A2 AD 47 04 4D 7F (소유 타입 = serpen_hunt sub_plan)
    let want = b.wrapping_add(0x33d4f70);
    let got = match sp_type_id(sp) { Some(r) => b.wrapping_add(r), None => return false };
    if !ptr_ok(got) || !ptr_ok(want) { return false; }
    match (rd_u64(got), rd_u64(got + 8), rd_u64(want), rd_u64(want + 8)) {
        (Some(a0), Some(a1), Some(w0), Some(w1)) => a0 == w0 && a1 == w1,
        _ => false,
    }
}
/// 0xe279f0 접근점: 사거리 안이면 (self.xy, t=0) · 밖이면 tgt 에서 self 쪽으로 (reach−15000) 지점을 그리드 보정한 좌표와 도달 틱.
///   tgt 가 내 진영에 안 보이면 None(게임 tag=0). capstone 전수(2026-09-07 01:30, 0xe279f0~0xe27c5c).
unsafe fn approach(ctx: usize, me: usize, tgt: usize, slot: usize) -> Option<Option<(i64, i64, u64)>> {
    let own = if rd_i32(slot + SLOT_FLAG)? != 0 { 0 } else { rng_of(me)? };
    let reach = rd_u64(slot + SLOT_BASE)?.wrapping_add(own).wrapping_add(rd_u64(me + ENT_F438)?)
        .wrapping_add(rd_u64(me + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(rd_u64(slot + SLOT_PERLV)?))
        .wrapping_add(rng_of(tgt)?).wrapping_add(slot_e8(slot, me, tgt)?);
    let (mx, my) = xy(me)?; let (tx, ty) = xy(tgt)?;
    let d = isqrt_fast(d2_xy(mx, my, tx, ty));
    if d <= reach { return Some(Some((mx as i64, my as i64, 0))); }
    if rd_u8(me) == 0 {
        let side = rd_u64(me + 8)?; if side > 1 { return None; }
        if rd_u64(tgt + 0x38 + (side as usize) * 0x18)? != 0 { return Some(None); }
    }
    let (dx, dy) = ((mx as i64).wrapping_sub(tx as i64), (my as i64).wrapping_sub(ty as i64));
    let len = isqrt_fast((dx.wrapping_mul(dx) as u64).wrapping_add(dy.wrapping_mul(dy) as u64));
    if len == 0 { return Some(Some((mx as i64, my as i64, 0))); }
    let back = reach.saturating_sub(15000) as i64;
    let px = dx.wrapping_mul(back) / len as i64 + tx as i64;
    let py = dy.wrapping_mul(back) / len as i64 + ty as i64;
    let sim = rd_u64(ctx + 8)? as usize; if !ptr_ok(sim) { return None; }
    let cfg = rd_u64(sim + 8)? as usize; let map = rd_u64(sim + 0x20)? as usize;
    if !ptr_ok(cfg) || !ptr_ok(map) { return None; }
    let (ax, ay) = super::as_callees::adjust_target(map, cfg, px, py)?;
    let sp640 = rd_u64(me + 0x640)?.max(1);
    let t = d.saturating_sub(reach) / sp640;
    Some(Some((ax, ay, t)))
}
/// 0xdef530: 적 로스터 중 (x,y) 에서 200,000 이내이고 (보임 ∨ 최근 120틱 목격) 인 적이 있는가
unsafe fn enemy_near_xy(w: &World, agents: usize, side: u64, x: u64, y: u64, now: u64) -> Option<bool> {
    let other = 1 - side;
    for i in 0..5usize {
        let e = rd_u64(w.x + X_ROSTER + (other as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize;
        if e == 0 { continue; }
        let (ex, ey) = xy(e)?;
        if d2_xy(ex, ey, x, y) >= 0x9502f9001 { continue; }
        let h = rd_u64(e + ENT_HANDLE)?;
        if w.visible(side, h)? { return Some(true); }
        let rec = w.roster_rec(h)?;
        if rec != 0 { let role = rd_u32(rec + REC_ROLE_O) as usize;
            if rd_u64(agents + (other as usize) * LANE_STRIDE + LANE_ROSTER + role * 8)?.wrapping_add(120) >= now { return Some(true); } }
    }
    Some(false)
}
/// 0xe04400 특수형 조기반환. TypeId 4종만 tag=1(조기반환), 그 외는 통과.
unsafe fn special_early(ctx: usize, _rec: usize, me: usize, sp: usize, tgt: usize) -> Option<Option<i64>> {
    let tid = match sp_type_id(sp) { Some(t) => t, None => return na(tag8("e04400")).map(|_| None) };
    if !matches!(tid, 0x33e1d30 | 0x33e1d40 | 0x33e1d50 | 0x33e1d60) { return Some(None); }
    let wroot = rd_u64(ctx)? as usize; let agents = rd_u64(ctx + 0x10)? as usize;
    let w = World { x: wroot, data: rd_u64(wroot)? as usize, vt: rd_u64(wroot + 8)? as usize };
    if !ptr_ok(w.data) || !ptr_ok(w.vt) { return None; }
    let side = rd_u64(me + 8)?; if side > 1 { return None; }
    let now = rd_u64(w.data + W_TICK)?;
    let (tx, ty) = xy(tgt)?;
    let v: i64 = match tid {
        // A: 자기 대상일 때만 의미. slot0 없으면 10, 있으면 clamp(est(slot0)*25 / max(3, sp.vt90*100/max(1, self.3fc+100)), 10, 90),
        //    단 주변에 적이 없으면 5. ⬜sp.vt+0x90(시전 계수) 미포팅이라 그 가지는 NA 로 남긴다.
        0x33e1d30 => {
            if me != tgt { 0 } else {
                let (mx, my) = xy(me)?;
                if !enemy_near_xy(&w, agents, side, mx, my, now)? { 5 }
                else if rd_i32(me + ENT_4C0)? == -1 { 10 }
                else { return na(tag8("e044_A")).map(|_| None); }
            }
        }
        0x33e1d40 => if enemy_near_xy(&w, agents, side, tx, ty, now)? { 25 } else { 8 },
        0x33e1d50 => {
            let n = rd_u64(w.x + 0x108 + (side as usize) * 0x20)?; let p = rd_u64(w.x + 0xf0 + (side as usize) * 0x20)? as usize;
            let mut cnt = 0i64;
            if n != 0 { if !ptr_ok(p) { return None; }
                for i in 0..n.min(CAP_ITER) as usize { let e = rd_u64(p + i * 8)? as usize; if e != 0 && rd_i32(e + ENT_KIND)? == 7 { cnt += 1; } } }
            if cnt == 0 { -100 } else if me == tgt { 0 } else { (18 * cnt).min(60) }
        }
        _ => if enemy_near_xy(&w, agents, side, tx, ty, now)? { 90 } else { 30 },
    };
    Some(Some(v))
}
