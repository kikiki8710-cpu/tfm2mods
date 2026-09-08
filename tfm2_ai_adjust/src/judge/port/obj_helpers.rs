//! obj_helpers — hunt_and_poke(disc 12/14) 가 부르는 team_plan 헬퍼 계층의 순수 재현. 게임 0.5.8 · REST 디컴 + capstone(2026-09-06 15:40~).
//!   0x12a07d0 isqrt · 0xec9840 count_my · 0xecacc0 count_enemy_seen · 0xeca430 my_ready · 0xeca200 can_attack
//!   0xeca9a0 objective_pref · 0xec9bf0 poke_timer_gate · 0xe3b570 band_pred · 0xe2fa70(min-by, 인라인) · 0xdcc100 enemy_could_arrive
//!   0xdd5db0 engage_gate · WorldOps vt+0x100 grid_ok(0x184acb0)
//! 검증: `cap_obj_can_attack`(0xeca200 rax) · `cap_obj_engage_gate`(0xdd5db0 rax) 캡처 wrap 안에서 같은 인자로 내 재현을 돌려 대조(est_damage 패턴).
//! 계약 요지:
//!   sim(p5류) = 선수 sim(+0x930 side · +0x9c0 role) · holder(p6류) = {+0 X, +8 G, +0x10 lanes base(+side*0x2e8), +0x1a0/0x1a8 에픽 타겟, +0x1d0/0x1d8 세르펜 타겟}
//!   G+0x38 맵 종류(0..8) · G+0x20 map_def(camp_pos 메모 키) · cfg=[G+8]: +0x12b8 W · +0x12c0 H · +0x12f8 tps
//!   X+0x1e0+side*0x28+role*8 로스터 · X+0x21c0+kind*0x10+side*8 오브젝티브 카운터
//!   order(p8류, 0x41f/0x420 플랜·SF) : +0x80/+0x88 타임스탬프 · +0x230+i*0x10 적 i 마지막 위치(x,y) · +0x2d0+i*8 마지막 관측 틱
//!   p7(타이머): +0x88/+0xc0 (에픽/세르펜)
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::super::tr;
use super::passive_line_callees::in_lane;
use super::passive_jungle::camp_pos_game;

pub const THR_ECA200: [u64; 9] = [4, 2, 1, 2, 1, 3, 1, 4, 4];     // DAT_1433f21b0 (맵 종류별 필요 아군 수)
pub const THR_DD5DB0: [u64; 9] = [2, 2, 1, 2, 1, 2, 1, 2, 2];     // DAT_1433e16e8
pub const ROLE_BONUS: [u64; 5] = [60000, 0, 40000, 60000, 20000]; // DAT_1433e16c0 / DAT_1433e8668[0..5]
const KINDS_M4: [u8; 2] = [0, 1];                                  // DAT_1433e0b8b
const KINDS_M5: [u8; 2] = [2, 1];                                  // DAT_1433e0b8d
const CFG_W: usize = 0x12b8; const CFG_H: usize = 0x12c0;
const BAND_R: u64 = 0x2ee00; const BAND_W: u64 = 64000;

#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
/// 0x12a07d0 — utils.rs isqrt: 두 경로(표+뉴턴 / 이분탐색) 모두 floor(sqrt(dist²)).
pub fn isqrt(x: u64) -> u64 {
    if x == 0 { return 0; }
    let mut r = (x as f64).sqrt() as u64;
    while r.checked_mul(r).map_or(true, |v| v > x) { r -= 1; }
    while (r + 1).checked_mul(r + 1).map_or(false, |v| v <= x) { r += 1; }
    r
}
#[inline] pub fn dist(ax: u64, ay: u64, bx: u64, by: u64) -> u64 { isqrt(sqd(ax, ay, bx, by)) }
#[inline] unsafe fn pct(e: usize) -> Option<u64> { let m = rd_u64(e + ENT_MAXHP)?; if m == 0 { return None; } Some(rd_u64(e + ENT_HP)?.wrapping_mul(100) / m) }   // maxhp 0 = 게임 panic
#[inline] fn mode_kind(m: u8) -> u8 { if m == 4 { 2 } else if m == 5 { 0 } else { 0xff } }
#[inline] fn map_mask(kind: u8) -> u32 { match kind { 0 => 0x185, 1 => 0x1b1, _ => 0x1ab } }
#[inline] unsafe fn roster(x: usize, side: u64, i: usize) -> Option<usize> { Some(rd_u64(x + X_ROSTER + side as usize * 0x28 + i * 8)? as usize) }
#[inline] unsafe fn lanes(p6: usize, side: u64) -> Option<usize> { let b = rd_u64(p6 + HOLDER_LANES)? as usize; if !ptr_ok(b) { return None; } Some(b + side as usize * LANE_STRIDE) }

/// 0xec9840 — 내 로스터 중 hp% ≥ hpmin 이고 (x,y) 반경 r 안인 챔피언 수.
pub unsafe fn count_my(sim: usize, p6: usize, x: u64, y: u64, r: u64, hpmin: u64) -> Option<u8> {
    let side = rd_u64(sim + P5_SIDE)?; if side > 1 { return None; }
    let w = Holder::new(p6)?.world()?; let r2 = r.wrapping_mul(r); let mut n = 0u8;
    for i in 0..5 { let e = roster(w.x, side, i)?; if e == 0 { continue; }
        if pct(e)? < hpmin { continue; }
        if sqd(x, y, rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?) <= r2 { n += 1; } }
    Some(n)
}
/// 0xecacc0 — 적 로스터 중 hp% ≥ hpmin · 반경 r 안 · (지금 보임 ‖ 로스터 기록 last+0x78 ≥ tick) 인 수.
pub unsafe fn count_enemy_seen(sim: usize, p6: usize, x: u64, y: u64, r: u64, hpmin: u64) -> Option<u8> {
    let side = rd_u64(sim + P5_SIDE)?; let other = 1u64.wrapping_sub(side); if other > 1 { return None; }
    let w = Holder::new(p6)?.world()?; let ln = lanes(p6, other)?; let r2 = r.wrapping_mul(r); let mut n = 0u8;
    for i in 0..5 { let e = roster(w.x, other, i)?; if e == 0 { continue; }
        if pct(e)? < hpmin { continue; }
        if sqd(x, y, rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?) > r2 { continue; }
        let h = rd_u64(e + ENT_HANDLE)?;
        let seen = if w.visible(side, h)? { true } else {
            let rec = w.roster_rec(h)?; if rec == 0 { false } else {
                let last = rd_u64(ln + LANE_ROSTER + rd_u32(rec + REC_ROLE) as usize * 8)?;
                w.tick()? <= last.wrapping_add(0x78) } };
        if seen { n += 1; } }
    Some(n)
}
/// 0xeca430 — 내 팀이 오브젝티브 m(4 에픽/5 세르펜)을 칠 준비가 됐나(저바이트).
pub unsafe fn my_ready(sim: usize, p6: usize, m: u8) -> Option<u8> {
    if m < 4 { return Some(0); }
    let kind = mode_kind(m);
    let g = rd_u64(p6 + HOLDER_G)? as usize; if !ptr_ok(g) { return None; }
    let mk = rd_u8(g + G_PHASE); if mk > 8 { return Some(0); }
    if (map_mask(kind) >> mk) & 1 == 0 { return Some(0); }
    let side = rd_u64(sim + P5_SIDE)?; if side > 1 { return None; }
    let map_def = rd_u64(g + G_BOXES)? as usize;
    let (cx, cy, _) = camp_pos_game(map_def, m, (side != 0) as usize)?;
    let w = Holder::new(p6)?.world()?; let mut n = 0u8;
    for i in 0..5 { let e = roster(w.x, side, i)?; if e == 0 { continue; }
        if pct(e)? < 40 { continue; }
        let (x, y) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
        if !in_lane(g, x, y, kind)? { continue; }
        if (sqd(cx, cy, x, y) >> 8) > 0xe8d4a50 { n += 1; } }
    if n == 0 { return Some(0); }
    if rd_u64(w.x + X_OBJ_CNT + (kind as usize) * 0x10 + side as usize * 8)? > 1 { return Some(1); }
    let ln = lanes(p6, side)?; let off = if kind == 0 { 0 } else { 0x50 };
    let t = rd_i64(ln + LANE_OBJ_T + off)?; if t > 2999 { return Some(1); }
    let c = rd_i32(ln + LANE_OBJ_N + off)?;
    Some((if t < 1000 { c > 3 } else { c > 1 }) as u8)
}
/// 0xeca200 — 오브젝티브 m 공격 가능 판정(bool).
pub unsafe fn can_attack(sim: usize, p6: usize, m: u8) -> Option<bool> {
    if m < 4 { return Some(false); }
    let kind = mode_kind(m);
    let g = rd_u64(p6 + HOLDER_G)? as usize; if !ptr_ok(g) { return None; }
    let mk = rd_u8(g + G_PHASE); if mk > 8 { return Some(false); }
    if (map_mask(kind) >> mk) & 1 == 0 { return Some(false); }
    let side = rd_u64(sim + P5_SIDE)?; if side > 1 { return None; }
    let w = Holder::new(p6)?.world()?;
    let me = roster(w.x, side, rd_u32(sim + P5_ROLE) as usize)?; if me == 0 { return Some(false); }
    let map_def = rd_u64(g + G_BOXES)? as usize;
    let (cx, cy, _) = camp_pos_game(map_def, m, side as usize)?;
    let (x, y) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    if !in_lane(g, x, y, kind)? { return Some(false); }
    if (sqd(cx, cy, x, y) >> 8) <= 0xe8d4a50 { return Some(false); }
    if my_ready(sim, p6, m)? == 0 { return Some(false); }
    let my = count_my(sim, p6, cx, cy, 180000, 40)?;
    let en = count_enemy_seen(sim, p6, cx, cy, 180000, 40)?;
    tr(4, 0x100 | my as u64 | (en as u64) << 8 | (mk as u64) << 16);
    Some(en == 0 && THR_ECA200[mk as usize] <= my as u64)
}
/// 0xeca9a0 — kinds 배열 중 "아직 확보 안 된" 오브젝티브 후보의 우선값(작을수록 우선) 최소 kind. 없으면 0xff.
pub unsafe fn objective_pref(sim: usize, p6: usize, kinds: &[u8]) -> Option<u8> {
    if kinds.is_empty() { return Some(0xff); }
    let side = rd_u64(sim + P5_SIDE)?; if side > 1 { return None; }   // 게임: switch → panic/0xff
    let g = rd_u64(p6 + HOLDER_G)? as usize; if !ptr_ok(g) { return None; }
    let mk = rd_u8(g + G_PHASE);
    let w = Holder::new(p6)?.world()?; let ln = lanes(p6, side)?;
    let (mut best, mut bestv) = (0xffu8, 0i64);
    let mut consider = |kv: u8, base: usize, best: &mut u8, bestv: &mut i64| -> Option<()> {
        let cnt = rd_u64(w.x + X_OBJ_CNT + kv as usize * 0x10 + side as usize * 8)?;
        let t = rd_i64(base + LANE_OBJ_T)?; let c = rd_i32(base + LANE_OBJ_N)?;
        if cnt >= 3 && t >= 2000 && c >= 2 { return Some(()); }                 // 확보됨 → 후보 아님
        let v = (c as i64).wrapping_mul(1000).wrapping_add((cnt as i64).wrapping_mul(10000)).wrapping_add(t);
        if v < *bestv || *best == 0xff { *bestv = v; *best = kv; }
        Some(())
    };
    for &kv in kinds {
        if mk > 8 { return Some(best); }
        if (0x1abu32 >> mk) & 1 != 0 {
            match kv {
                0 => { if (0x185u32 >> mk) & 1 != 0 { consider(0, ln, &mut best, &mut bestv)?; } }
                2 => { consider(2, ln + 0x50, &mut best, &mut bestv)?; }
                _ => { if (0x1b1u32 >> mk) & 1 != 0 { consider(kv, ln + 0x28, &mut best, &mut bestv)?; } }
            }
        } else if mk == 2 { if kv == 0 { consider(0, ln, &mut best, &mut bestv)?; } }
        else if mk == 4 { if kv == 1 { consider(1, ln + 0x28, &mut best, &mut bestv)?; } }
        else { return Some(best); }
    }
    Some(best)
}
/// ★타겟 Vec 은 holder 가 아니라 **모드 데이터**(vt+0x40 반환 rdx = w+0xed00)의 +0x1a0/0x1a8(에픽)·+0x1d0/0x1d8(세르펜)에 있다.
///   디컴이 `param_2[0x3b]`/`param_3+0x1a8` 로 찍은 것은 vt+0x40 호출 뒤 **rdx(=반환 pair 의 두 번째)** 를 인자 레지스터로 오인한 것(16:10 실측: holder+0x1d8 은 스택 쓰레기).
///   캡처 wrap 이 원본 호출 전에 `pre_read_targets(p6)` 로 (len, handle) 을 스냅샷해 두고 재현은 그 스냅샷을 쓴다(없으면 직접 읽음).
thread_local! { static PRE_T: core::cell::Cell<[u64; 6]> = const { core::cell::Cell::new([0; 6]) }; }
pub unsafe fn pre_read_targets(p6: usize) {
    let mut v = [0u64; 6];
    if let Some(w) = Holder::new(p6).and_then(|h| h.world()) {
        if let Some((0, mode)) = w.mode() {
            for (k, (tp, tl)) in [(T0_PTR, T0_LEN), (T1_PTR, T1_LEN)].iter().enumerate() {
                let len = rd_u64(mode + tl).unwrap_or(0); let ptr = rd_u64(mode + tp).unwrap_or(0) as usize;
                let h = if len != 0 && ptr_ok(ptr) { rd_u64(ptr).unwrap_or(u64::MAX) } else { u64::MAX };
                v[k * 3] = 1; v[k * 3 + 1] = len; v[k * 3 + 2] = h;
            }
        }
    }
    PRE_T.with(|c| c.set(v));
}
pub fn pre_clear() { PRE_T.with(|c| c.set([0; 6])); }
/// 0xec9bf0 — 견제 타이머 게이트(bool). serpen=false: 에픽(order+0x80 · p7+0x88 · 타겟 holder+0x1a0/0x1a8) / true: 세르펜(+0x88 · +0xc0 · +0x1d0/0x1d8).
pub unsafe fn poke_timer_gate(sim: usize, p6: usize, p7: usize, order: usize, serpen: bool) -> Option<bool> {
    let g = rd_u64(p6 + HOLDER_G)? as usize; if !ptr_ok(g) { return None; }
    let tps = rd_u64(rd_u64(g + G_CFG)? as usize + CFG_TPS)?; let mk = rd_u8(g + G_PHASE);
    tr(3, 0x100 | mk as u64 | (serpen as u64) << 8);
    let (ts, lim, tptr, tlen) = if !serpen {
        if mk.wrapping_sub(1) < 6 { tr(3, 0x1_0000); return Some(false); }
        (rd_u64(order + ORDER_TS4)?, rd_u64(p7 + P7_EPIC_TA - 0x10)?, T0_PTR, T0_LEN)
    } else {
        if mk > 8 || (0x1a1u32 >> mk) & 1 == 0 { tr(3, 0x2_0000); return Some(false); }
        (rd_u64(order + ORDER_TS5)?, rd_u64(p7 + P7_SERPEN_TA - 0x10)?, T1_PTR, T1_LEN)
    };
    let w = Holder::new(p6)?.world()?;
    let tick = w.tick()?;
    let pre = PRE_T.with(|c| c.get()); let k = serpen as usize * 3;
    let (tlen_v, hval) = if pre[k] != 0 { (pre[k + 1], pre[k + 2]) } else {
        let (tag, mode) = w.mode()?; if tag != 0 { return None; }                       // 게임: unwrap None panic
        let l = rd_u64(mode + tlen)?; let hp_ptr = rd_u64(mode + tptr)? as usize;
        (l, if l != 0 { if !ptr_ok(hp_ptr) { return None; } rd_u64(hp_ptr)? } else { u64::MAX }) };
    if tlen_v == 0 { tr(3, 0x3_0000); return Some(false); }
    if hval == u64::MAX { return None; }
    let hp_ptr = 0usize;
    let t = match w.entity(hval) { Some(e) => e.0, None => {
        let l3c = rd_u64(w.data + W_L3_CNT).unwrap_or(0); let tag = if hval < l3c { rd_i32(rd_u64(w.data + W_L3_TBL).unwrap_or(0) as usize + hval as usize * W_SLOT_STRIDE).unwrap_or(-9) } else { -8 };
        tr(3, 0x4_0000 | hval.min(0xffff) << 20 | (l3c.min(0xffff)) << 36 | ((tag as u64) & 0xff) << 52);
        tr(2, hp_ptr as u64); tr(1, rd_u64(p6 + tlen).unwrap_or(0) | (rd_u64(p6 + tptr).unwrap_or(0) & 0xffff_ffff) << 32);
        return Some(false) } };
    let sd = rd_u64(sim + P5_SIDE)?;
    tr(2, t as u64); tr(0, 0x100 | rd_u64(t + ENT_VIS_BASE + sd as usize * ENT_VIS_STRIDE).unwrap_or(0xff).min(0xff) | (hval.min(0xffff)) << 16 | (rd_u64(t + ENT_HANDLE).unwrap_or(0).min(0xffff)) << 32 | (rd_u32(t + ENT_KIND) as u64 & 0xff) << 48);
    if !w.visible(sd, rd_u64(t + ENT_HANDLE)?)? { tr(3, 0x5_0000); return Some(false); }
    if ts.wrapping_add(tps) < tick { tr(3, 0x6_0000 | ts.min(0xfffff) << 20 | tick.min(0xfffff) << 40); return Some(false); }
    if rd_u64(t + ENT_HP)? < rd_u64(t + ENT_MAXHP)? { tr(3, 0x7_0000 | lim.min(0xfffff) << 20 | tps.min(0xfff) << 40); return Some(lim <= tps.wrapping_mul(20)); }
    tr(1, rd_u64(t + ENT_HP)?.min(0xffff) | rd_u64(t + ENT_MAXHP)?.min(0xffff) << 16 | lim.min(0xffff) << 32 | tps.min(0xffff) << 48);
    tr(3, 0x8_0000);
    // 진단 덤프(≤12줄): 리졸버 내부값 + 싱글턴 폴백 후보
    static DBG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if DBG.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 12 {
        let d = w.data; let l3c = rd_u64(d + W_L3_CNT).unwrap_or(0); let tbl = rd_u64(d + W_L3_TBL).unwrap_or(0) as usize;
        let slot = tbl + hval as usize * W_SLOT_STRIDE; let tag = rd_i32(slot).unwrap_or(-99); let idx = rd_u64(slot + 8).unwrap_or(0);
        let entc = rd_u64(d + W_ENT_CNT).unwrap_or(0); let base = rd_u64(d + W_ENT_BASE).unwrap_or(0);
        let sh = rd_u64(d + W_SINGLETON_H).unwrap_or(0); let stag = rd_i32(d + W_SINGLETON_TAG).unwrap_or(-99); let se = d + W_SINGLETON_TAG;
        let line = format!("hval={} l3cnt={} tag={} idx={} entcnt={} base={:#x} e={:#x} kind={} hp={}/{} | singleton h={} tag={} kind={} hp={}/{} | tlen={} tptr={:#x} serpen={} p6={:#x}
",
            hval, l3c, tag, idx, entc, base, t, rd_u32(t + ENT_KIND), rd_u64(t + ENT_HP).unwrap_or(0), rd_u64(t + ENT_MAXHP).unwrap_or(0),
            sh, stag, rd_u32(se + ENT_KIND), rd_u64(se + ENT_HP).unwrap_or(0), rd_u64(se + ENT_MAXHP).unwrap_or(0), tlen_v, w.mode().map(|m| rd_u64(m.1 + tptr).unwrap_or(0)).unwrap_or(0), serpen, p6);
        if let Some(p) = super::super::pth("judge_obj_poke_dbg.txt") { let _ = std::fs::OpenOptions::new().create(true).append(true).open(p).and_then(|mut f| { use std::io::Write; f.write_all(line.as_bytes()) }); }
    }
    Some(false)
}
/// 0xe3b570 — 챔피언 e 가 "hp% ≥ 40 이고 오브젝티브 m 쪽 띠에 있음" (m 4/5 의 대각 띠 판정).
pub unsafe fn band_pred(g: usize, m: u8, e: usize) -> Option<bool> {
    if pct(e)? < 40 { return Some(false); }
    let cfg = rd_u64(g + G_CFG)? as usize; if !ptr_ok(cfg) { return None; }
    let (h, wd) = (rd_u64(cfg + CFG_H)?, rd_u64(cfg + CFG_W)?);
    let (x, y) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
    let d = h.wrapping_sub(y);
    Some(match m {
        5 => { if d < BAND_R + 1 || d <= x { true } else if wd.wrapping_sub(BAND_R) <= x { true } else { d - x < BAND_W } }
        4 => { if x < BAND_R + 1 { true } else if x <= d { true } else if wd.wrapping_sub(BAND_R) <= d { true } else { x - d < BAND_W } }
        _ => false,
    })
}
/// WorldOps vt+0x100 (0x184acb0) — 사이드 영향력 그리드 30×30 의 셀(gx,gy) > 0.
pub unsafe fn grid_ok(w: &World, side: u64, gx: u64, gy: u64) -> Option<bool> {
    if gx >= 30 || gy >= 30 { return Some(false); }
    if side > 1 { return None; }
    Some(rd_i32(w.data + W_GRID + side as usize * W_GRID_SIDE + gy as usize * W_GRID_ROW + gx as usize * 4)? > 0)
}
/// 0xdcc100 — 안 보이는 적 챔피언(hp% > 49)이 마지막 관측 이후 오브젝티브 150000 안까지 올 수 있었나.
pub unsafe fn enemy_could_arrive(w: &World, lanes_base: usize, side: u64, order: usize, cx: u64, cy: u64, tick: u64) -> Option<bool> {
    let other = 1u64.wrapping_sub(side); if other > 1 { return None; }
    if !ptr_ok(lanes_base) { return None; } let ln = lanes_base + other as usize * LANE_STRIDE;
    for i in 0..5 { let e = roster(w.x, other, i)?; if e == 0 { continue; }
        let pc = pct(e)?; let mut d8 = 0x100 | pc.min(0xff);
        let trace = |v: u64| tr(if i < 4 { 8 + i } else { 2 }, v);
        if pc <= 0x31 { trace(d8); continue; }
        let h = rd_u64(e + ENT_HANDLE)?;
        if w.visible(side, h)? { trace(d8 | 1 << 9); continue; }
        let rec = w.roster_rec(h)?;
        if rec != 0 { d8 |= 1 << 10; let last = rd_u64(ln + LANE_ROSTER + rd_u32(rec + REC_ROLE) as usize * 8)?; if !(last.wrapping_add(0x78) < tick) { trace(d8 | 1 << 11); continue; } }
        let d = dist(rd_u64(order + ORDER_LASTPOS + i * 0x10)?, rd_u64(order + ORDER_LASTPOS + 8 + i * 0x10)?, cx, cy);
        let need = d.saturating_sub(150000);
        let seen = rd_u64(order + ORDER_LASTTICK + i * 8)?; let el = tick.saturating_sub(seen);
        let cap = el.wrapping_mul(rd_u64(e + ENT_SPEED)?);
        trace(d8 | 1 << 12 | need.min(0xfffff) << 16 | cap.min(0xfffff) << 40 | (rd_u64(e + ENT_SPEED)?.min(0xf)) << 60);
        if need <= cap { return Some(true); } }
    Some(false)
}
/// 0xdd5db0 — 교전 전 최종 게이트(bool). (order=p8, sim=p5, holder=p6, p7=타이머, m=4/5)
pub unsafe fn engage_gate(order: usize, sim: usize, p6: usize, p7: usize, m: u8) -> Option<bool> {
    let kinds: &[u8] = if m == 5 {
        if rd_u8(order + ORDER_SF) != 1 || rd_u8(order + ORDER_PLAN) != 1 { return Some(false); } &KINDS_M5
    } else if m == 4 {
        if rd_u8(order + ORDER_PLAN) != 0 || rd_u8(order + ORDER_SF) != 1 { return Some(false); } &KINDS_M4
    } else { return Some(false); };
    if objective_pref(sim, p6, kinds)? != 0xff { tr(9, 0x101); return Some(false); }
    let side = rd_u64(sim + P5_SIDE)?; if side > 1 { return None; }
    let h = Holder::new(p6)?; let w = h.world()?;
    let g = rd_u64(p6 + HOLDER_G)? as usize; if !ptr_ok(g) { return None; }
    let mk = rd_u8(g + G_PHASE); if mk > 8 { return None; }   // 게임: 표 범위 밖 read(미정의) — NA
    let mut cnt = 0u64;
    for i in 0..5 { let e = roster(w.x, side, i)?; if e == 0 { continue; } if band_pred(g, m, e)? { cnt += 1; } }
    if cnt < THR_DD5DB0[mk as usize] { tr(9, 0x102); return Some(false); }
    let map_def = rd_u64(g + G_BOXES)? as usize;
    let (cx, cy, _) = camp_pos_game(map_def, m, side as usize)?;
    let role = rd_u32(sim + P5_ROLE) as usize;
    if roster(w.x, side, role)? == 0 { return Some(false); }
    // 첫 통과 인덱스 k → 이후 min-by(dist + ROLE_BONUS)
    let (mut best_e, mut best_i, mut best_v) = (0usize, 0usize, 0u64);
    for j in 0..5 { let e = roster(w.x, side, j)?; if e == 0 { continue; }
        if !band_pred(g, m, e)? { continue; }
        let v = dist(rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?, cx, cy).wrapping_add(ROLE_BONUS[j]);
        if best_e == 0 || v < best_v { best_e = e; best_i = j; best_v = v; } }
    if best_e == 0 { tr(9, 0x103); return Some(false); }
    tr(5, 0x100 | best_i as u64 | (role as u64) << 4 | (cnt) << 8);
    if best_i != role { return Some(false); }
    if poke_timer_gate(sim, p6, p7, order, m == 5)? { tr(9, 0x104); return Some(true); }
    let tick = w.tick()?; let tps = rd_u64(rd_u64(g + G_CFG)? as usize + CFG_TPS)?;
    let ts = rd_u64(order + if m == 4 { ORDER_TS4 } else { ORDER_TS5 })?;
    let gok = grid_ok(&w, side, cx / 32000, cy / 32000)?;
    tr(6, 0x100 | gok as u64 | ((cx / 32000) & 0xff) << 8 | ((cy / 32000) & 0xff) << 16 | side << 24 | (rd_i32(w.data + W_GRID + side as usize * W_GRID_SIDE + (cy / 32000) as usize * W_GRID_ROW + (cx / 32000) as usize * 4).unwrap_or(-99) as u64 & 0xffff) << 32);
    tr(7, tick.min(0xfffff) | ts.min(0xfffff) << 20 | tps.min(0xff) << 40);
    if tick <= tps.wrapping_mul(2).wrapping_add(ts) && gok { tr(9, 0x105); return enemy_could_arrive(&w, rd_u64(p6 + HOLDER_LANES)? as usize, side, order, cx, cy, tick); }
    tr(9, 0x106); Some(true)
}
/// 0xdcc100 캡처 대조용 — 클로저 env(p2: [0]=&{X,sim} [1]=data [2]=vt [3]=lanes [4]=sim [5]=order [6]=&cx [7]=&cy [8]=&tick) 와 범위 p1({idx,end}, 원본 호출 **전** 값)로 재현.
pub unsafe fn could_arrive_env(idx0: u64, end: u64, env: usize) -> Option<bool> {
    if !ptr_ok(env) || idx0 != 0 || end != 5 { return None; }
    let xs = rd_u64(env)? as usize; if !ptr_ok(xs) { return None; }
    let x = rd_u64(xs)? as usize; let sim = rd_u64(xs + 8)? as usize;
    let w = World { x, data: rd_u64(env + 8)? as usize, vt: rd_u64(env + 0x10)? as usize };
    let lanes_base = rd_u64(env + 0x18)? as usize; let order = rd_u64(env + 0x28)? as usize;
    let cx = rd_u64(rd_u64(env + 0x30)? as usize)?; let cy = rd_u64(rd_u64(env + 0x38)? as usize)?; let tick = rd_u64(rd_u64(env + 0x40)? as usize)?;
    if !ptr_ok(sim) || !ptr_ok(order) { return None; }
    enemy_could_arrive(&w, lanes_base, rd_u64(sim + P5_SIDE)?, order, cx, cy, tick)
}

