//! position_eval — `0xd84db0` 래퍼(sret 0x38B, TLS 메모) + 본체 `0xd851d0`(30KB) 의 **순수 포팅**(2026-09-06 21:40~).
//!   정본 = REPORT RE 5건: `RE\2026-09-06_position_eval-래퍼-…구조RE`, `…S0~S5-쌍레코드-0xd815e0…`, `…S6~S12-…`, `…S11-액션장판…`, `…S13~S21-…`.
//!   계약: position_eval(out, mode, sim, H{X,G,lanes}, qx, qy, purpose) → out: +0 A(위협 합, 노이즈) · +8 B(적 분수 9999/적 타워 커버) · +0x10 C · +0x18 maxC · +0x20 0 · +0x28 half28 · +0x30 f30 · +0x31 f31.
//!   순수 재현 경계: 타워다이브 `0xd96d00` 은 전투 시뮬 `0xe05450` 에 의존 → verify 는 게임 TLS 메모(+0x16050) 를 조회해 (dive, tgt) 를 얻는다(`dive_lookup`). live 전환 시 별도 재현 필요.
//!   dyn 패밀리(Effect vt+0x28/0x38/0x40/0x48/0x80/0x88/0xf8/0x120) 는 dyn_eff/fight_check/as_callees 의 디스패처를 재사용하고 미재현 impl 은 unseen 으로 수집한다.
#![allow(dead_code)]
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::dyn_eff as dy;
use super::passive_jungle::estimate_damage;
use super::as_callees::{eff_bool, kind_pred, isqrt_fast};
use super::action_score::{sim_of_handle, champ_kind, champ_tags, champ_vt30_w2, tags_has};
use super::fight_check::eff_vt120;
use super::dn_reach::eff_e8;

const W_SEED: usize = 0xec90;
const ENT_KIND: usize = 0x68;
const SLOT0: usize = 0x490; const SLOT1: usize = 0x4c8; const SLOT2: usize = 0x500; const SLOT3: usize = 0x538;
const CFG_8A8: usize = 0x8a8; const CFG_1490: usize = 0x1490;
const G_PHASE: usize = 0x38; const G_MAP: usize = 0x20;
const X_OBJ_PTR: usize = 0xd0; const X_OBJ_LEN: usize = 0xe8;
const X_SIDE_UNITS_PTR: usize = 0xf0; const X_SIDE_UNITS_LEN: usize = 0x108;
const X_ACTIONS_PTR: usize = 0x810; const X_ACTIONS_LEN: usize = 0x818; const ACTION_STRIDE: usize = 0x138;
const CFGTICK_TBL: [usize; 9] = [0x13f8, 0x1400, 0x13f8, 0x1400, 0x13f8, 0x1408, 0x13f8, 0x13f8, 0x13f8];

// ── 공통 헬퍼 ────────────────────────────────────────────────────────────────────────────
#[inline] fn absd(a: u64, b: u64) -> u64 { if a < b { b - a } else { a - b } }
#[inline] fn sat_sq(a: u64, b: u64) -> u64 { let d = absd(a, b) as u128; let p = d * d; if p >> 64 != 0 { u64::MAX } else { p as u64 } }
#[inline] fn sat_d2(ax: u64, ay: u64, bx: u64, by: u64) -> u64 { sat_sq(ax, bx).saturating_add(sat_sq(ay, by)) }
#[inline] fn wrap_d2(ax: u64, ay: u64, bx: u64, by: u64) -> u64 { let dx = absd(ax, bx); let dyv = absd(ay, by); dx.wrapping_mul(dx).wrapping_add(dyv.wrapping_mul(dyv)) }
#[inline] fn sq(x: u64) -> u64 { x.wrapping_mul(x) }
#[inline] fn cap150(x: u64, scale: u64) -> u64 { let p = (x as u128) * (scale as u128); if (p >> 96) != 0 { 150 } else { ((p >> 32) as u64).min(150) } }
#[inline] fn third(v: u64) -> u64 { ((((v as u32).wrapping_mul(0xab)) as u16 as u32) >> 9) as u64 }
#[inline] fn sdiv2(v: i64) -> i64 { v.wrapping_div(2) }
#[inline] fn sdiv3(v: i64) -> i64 { v.wrapping_div(3) }
#[inline] unsafe fn xy(e: usize) -> Option<(u64, u64)> { Some((rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?)) }
#[inline] unsafe fn radius(e: usize) -> Option<u64> {
    let p = rd_i32(e + ENT_F470)? as i64; let r = rd_u64(e + ENT_F680)?;
    Some(if p == 0 { r } else { ((p + 100).wrapping_mul(r as i64)) as u64 / 100 })
}
/// 슬롯 k 의 desc 주소(ZERO desc 는 None = id −1 취급)
#[inline] unsafe fn slot_of(e: usize, k: usize) -> Option<Option<usize>> {
    let lv = rd_u64(e + ENT_LEVEL)?;
    Some(match k { 0 => Some(e + SLOT0), 1 => Some(e + SLOT1), 2 => if lv >= 3 { Some(e + SLOT2) } else { None }, _ => if lv >= 5 { Some(e + SLOT3) } else { None } })
}
#[inline] unsafe fn slot_id(s: Option<usize>) -> Option<i32> { match s { Some(p) => rd_i32(p + 0x30), None => Some(-1) } }
/// Effect vt+0xe8(사거리 보너스)
#[inline] unsafe fn vt_e8(slot: usize, owner: usize, other: usize) -> Option<u64> { eff_e8(rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize, owner, other, 0) }
/// e 의 슬롯0 기준 사거리(dyn 보너스 포함, 반경 제외)
unsafe fn base_range(e: usize, other: usize) -> Option<u64> {
    Some(rd_u64(e + ENT_F438)?.wrapping_add(rd_u64(e + 0x4a0)?).wrapping_add(rd_u64(e + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(rd_u64(e + 0x4a8)?)).wrapping_add(vt_e8(e + SLOT0, e, other)?))
}
/// 12854c0 형: +(4c0==0 ? radius(e) : 0) + radius(t)
unsafe fn range_g(e: usize, t: usize) -> Option<u64> { let r = if rd_i32(e + 0x4c0)? == 0 { radius(e)? } else { 0 }; Some(base_range(e, t)?.wrapping_add(r).wrapping_add(radius(t)?)) }
/// 4c0 게이트 없이 radius(e) 항상
unsafe fn range_u(e: usize, t: usize) -> Option<u64> { Some(base_range(e, t)?.wrapping_add(radius(e)?).wrapping_add(radius(t)?)) }
/// Effect bool 게터(vt+0xf8 skillshot 등): 0x9db70=0 · EFF_TRUE=1 · 그 외 unseen(slot)
unsafe fn eff_flag(data: usize, vt: usize, slot: usize) -> Option<u64> { eff_flag_d(data, vt, slot, 0) }
unsafe fn eff_flag_d(data: usize, vt: usize, slot: usize, depth: u32) -> Option<u64> {
    if depth > 40 { super::dyn_eff::unseen(0x620, depth as usize); return None; }
    let r = dy::impl_rva(vt, slot)?; let p = dy::arc_payload(data, vt)?;
    match (slot, r) {
        (_, EFF_E8_ZERO) => Some(0), (_, EFF_TRUE) => Some(1),
        (0xf8, 0x1225480) | (0xf8, 0x1147250) | (0xf8, 0x122fcf0) | (0xf8, 0x16ab6d0) | (0xf8, 0x16adaa0) | (0xf8, 0x13bfa20) => Some(1),   // rax=1, rdx=[p+0x78]/[p]/[p+0x10]/[p+0x38]
        (0xf8, 0x12a5ac0) => Some((rd_u64(p + 0x18)? != 0) as u64),
        (0xf8, 0x16065b0) => eff_flag_d(rd_u64(p + 0x18)? as usize, rd_u64(p + 0x20)? as usize, slot, depth + 1),   // 단일 자식 tail-call
        (0xf8, 0x12a6c70) => {                                                           // 자식(stride 0x10 @p+8/len p+0x10) 중 첫 al&1 → 그 rax, 없으면 0
            let n = rd_u64(p + 0x10)?; if n == 0 { return Some(0); } let arr = rd_u64(p + 8)? as usize; if !ptr_ok(arr) { return None; }
            for i in 0..n.min(CAP_ITER) as usize { let v = eff_flag_d(rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize, slot, depth + 1)?; if v & 1 == 1 { return Some(v); } }
            Some(0)
        }
        _ => { dy::unseen(0x200 + slot as u32, r); None }
    }
}
#[inline] unsafe fn skillshot(slot: Option<usize>) -> Option<bool> {
    match slot { None => Some(false), Some(s) => { if rd_i32(s + 0x30)? == -1 { return Some(false); } Some(eff_flag(rd_u64(s)? as usize, rd_u64(s + 8)? as usize, 0xf8)? == 1) } }
}
/// 상태(버프) 리스트(+0x2c8/+0x2d0, stride 0x28, [0] i32 kind)
unsafe fn status_has(e: usize, kind: i32) -> Option<bool> {
    let n = rd_u64(e + 0x2d0)?; if n == 0 { return Some(false); }
    let p = rd_u64(e + 0x2c8)? as usize; if !ptr_ok(p) { return None; }
    for i in 0..n.min(CAP_ITER) as usize { if rd_i32(p + i * 0x28)? == kind { return Some(true); } }
    Some(false)
}
unsafe fn status_any_not_2345(e: usize) -> Option<bool> {
    let n = rd_u64(e + 0x2d0)?; if n == 0 { return Some(false); }
    let p = rd_u64(e + 0x2c8)? as usize; if !ptr_ok(p) { return None; }
    for i in 0..n.min(CAP_ITER) as usize { let k = rd_i32(p + i * 0x28)?; if ((k.wrapping_sub(6)) as u32) < 0xfffffffc { return Some(true); } }
    Some(false)
}
#[inline] fn same_team_ab(a0: u64, a8: u64, b0: u64, b8: u64) -> bool { a0 == b0 && (a0 != 0 || a8 == b8) }

// ── 쌍 레코드(0xd815e0) ───────────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct Rec { pub w: [u64; 52], pub ss: [u8; 3] }
impl Rec { fn zero() -> Rec { Rec { w: [0; 52], ss: [0; 3] } } }
#[inline] unsafe fn pct(v: u64, hp: u64) -> u64 { (v.wrapping_mul(100) / if hp == 0 { 1 } else { hp }).min(150) }
/// d815e0(A=self, B=other, simA, simB, tblA, tblB, flag)
pub unsafe fn pair_record(a: usize, b: usize, sim_a: usize, sim_b: usize, tbl_a: usize, tbl_b: usize, flag: bool) -> Option<Rec> {
    let ra8 = rd_u32(sim_a + P5_ROLE) as usize * 8; let rb8 = rd_u32(sim_b + P5_ROLE) as usize * 8;
    let (a0, a8, b0, b8) = (rd_u64(a)?, rd_u64(a + 8)?, rd_u64(b)?, rd_u64(b + 8)?);
    let enemy = a0 != b0 || (a0 == 0 && a8 != b8);
    let (hpa, hpb) = (rd_u64(a + ENT_HP)?, rd_u64(b + ENT_HP)?);
    let (lva, lvb) = (rd_u64(a + ENT_LEVEL)?, rd_u64(b + ENT_LEVEL)?);
    let ra = radius(a)?; let rb = radius(b)?;
    let mut r = Rec::zero();
    let t = |base: usize, off: usize| -> Option<u64> { rd_u64(base + off) };
    if enemy {
        if rd_i32(b + 0x4c0)? == -1 || rd_i32(a + 0x4c0)? == -1 { return None; }
        for k in 0..4usize {
            r.w[k] = pct((t(tbl_b, 0x190 + 0x28 * k + ra8)? >> 1).wrapping_add(t(tbl_b, 0x28 * k + ra8)?), hpa);
            r.w[4 + k] = pct((t(tbl_a, 0x190 + 0x28 * k + rb8)? >> 1).wrapping_add(t(tbl_a, 0x28 * k + rb8)?), hpb);
        }
        for j in 0..3usize {
            let sb = slot_of(b, j + 1)?;
            r.w[8 + j] = match sb { Some(s) if rd_i32(s + 0x30)? != -1 => ra.wrapping_add(rd_u64(s + 0x10)?).wrapping_add(rd_u64(b + ENT_F438)?).wrapping_add(rd_u64(s + 0x18)?.wrapping_mul(lvb.wrapping_sub(1))).wrapping_add(rb).wrapping_add(vt_e8(s, b, a)?).wrapping_add(18000), _ => 0 };
            let sa = slot_of(a, j + 1)?;
            r.w[0xb + j] = match sa { Some(s) if rd_i32(s + 0x30)? != -1 => rd_u64(s + 0x10)?.wrapping_add(rd_u64(a + ENT_F438)?).wrapping_add(rd_u64(s + 0x18)?.wrapping_mul(lva.wrapping_sub(1))).wrapping_add(ra).wrapping_add(rb).wrapping_add(vt_e8(s, a, b)?), _ => 0 };
        }
        let r0b = rd_u64(b + ENT_F438)?.wrapping_add(rd_u64(b + 0x4a0)?).wrapping_add(rd_u64(b + 0x4a8)?.wrapping_mul(lvb.wrapping_sub(1))).wrapping_add(rb).wrapping_add(vt_e8(b + SLOT0, b, a)?);
        r.w[0xe] = sq(r0b.wrapping_add(ra).wrapping_add(18000)); r.w[0xf] = sq(r0b.wrapping_add(ra).wrapping_add(50000));
        let r0a = rd_u64(a + ENT_F438)?.wrapping_add(rd_u64(a + 0x4a0)?).wrapping_add(rd_u64(a + 0x4a8)?.wrapping_mul(lva.wrapping_sub(1))).wrapping_add(ra).wrapping_add(rb).wrapping_add(vt_e8(a + SLOT0, a, b)?);
        r.w[0x19] = sq(r0a); r.w[0x1a] = sq(r0a.wrapping_add(32000));
    } else {
        let mut ha = t(tbl_b, 0xa0 + ra8)?.wrapping_add(t(tbl_b, 0x230 + ra8)? >> 1);
        if flag { ha = ha.wrapping_add(t(tbl_b, 0x118 + ra8)?).wrapping_add(t(tbl_b, 0x280 + ra8)? >> 1); }
        r.w[0] = 0; r.w[1] = pct(ha, hpa);
        let mut h2 = t(tbl_b, 0xc8 + ra8)?.wrapping_add(t(tbl_b, 0x258 + ra8)? >> 1); if flag { h2 = h2.wrapping_add(t(tbl_b, 0x140 + ra8)?).wrapping_add(t(tbl_b, 0x2a8 + ra8)? >> 1); }
        r.w[2] = pct(h2, hpa);
        let mut h3 = t(tbl_b, 0xf0 + ra8)?.wrapping_add(t(tbl_b, 0x2d0 + ra8)? >> 1); if flag { h3 = h3.wrapping_add(t(tbl_b, 0x168 + ra8)?).wrapping_add(t(tbl_b, 0x2f8 + ra8)? >> 1); }
        r.w[3] = pct(h3, hpa);
        r.w[4] = 0;
        r.w[5] = pct(t(tbl_a, 0xa0 + rb8)?.wrapping_add(t(tbl_a, 0x230 + rb8)? >> 1).wrapping_add(t(tbl_a, 0x118 + rb8)?).wrapping_add(t(tbl_a, 0x280 + rb8)? >> 1), hpb);
        r.w[6] = pct(t(tbl_a, 0xc8 + rb8)?.wrapping_add(t(tbl_a, 0x258 + rb8)? >> 1).wrapping_add(t(tbl_a, 0x140 + rb8)?).wrapping_add(t(tbl_a, 0x2a8 + rb8)? >> 1), hpb);
        r.w[7] = pct(t(tbl_a, 0xf0 + rb8)?.wrapping_add(t(tbl_a, 0x2d0 + rb8)? >> 1).wrapping_add(t(tbl_a, 0x168 + rb8)?).wrapping_add(t(tbl_a, 0x2f8 + rb8)? >> 1), hpb);
        for j in 0..3usize {
            let sb = slot_of(b, j + 1)?;
            r.w[8 + j] = match sb { Some(s) if rd_i32(s + 0x30)? != -1 => rd_u64(b + ENT_F438)?.wrapping_add(rd_u64(s + 0x10)?).wrapping_add(rd_u64(s + 0x18)?.wrapping_mul(lvb.wrapping_sub(1))).wrapping_add(rb).wrapping_add(ra).wrapping_add(vt_e8(s, b, a)?), _ => 0 };
            let sa = slot_of(a, j + 1)?;
            r.w[0xb + j] = match sa { Some(s) if rd_i32(s + 0x30)? != -1 => rd_u64(a + ENT_F438)?.wrapping_add(rd_u64(s + 0x10)?).wrapping_add(rd_u64(s + 0x18)?.wrapping_mul(lva.wrapping_sub(1))).wrapping_add(ra).wrapping_add(rb).wrapping_add(vt_e8(s, a, b)?), _ => 0 };
        }
    }
    for j in 0..3usize {
        let vb = r.w[8 + j]; r.w[0x10 + 3 * j] = sq(vb); r.w[0x11 + 3 * j] = sq(vb.wrapping_add(32000)); r.w[0x12 + 3 * j] = sq(vb >> 1);
        let va = r.w[0xb + j]; r.w[0x1b + 3 * j] = sq(va); r.w[0x1c + 3 * j] = sq(va.wrapping_add(32000)); r.w[0x1d + 3 * j] = sq(va >> 1);
    }
    let v = rd_u64(sim_b + 0x1f0)?.min(100) as u32;
    let f = |mul: u32, add: u32| -> u64 { let t = (v.wrapping_mul(mul).wrapping_add(add)) as u16; let t2 = (t >> 2) as u32; (((t2 * 5243) >> 16) >> 1) as u64 };
    r.w[0x24] = f(25, 300); r.w[0x25] = f(60, 500);
    for j in 0..3usize { r.ss[j] = skillshot(slot_of(b, j + 1)?)? as u8; }
    Some(r)
}

// ── 마스크(0xc7ebc0)·facet(0xc57f20) ───────────────────────────────────────────────────
unsafe fn masks(w: &World, lanes: usize, sim: usize, side: u64, eside: u64, tick: u64) -> Option<(u8, u8)> {
    let (mut em, mut am) = (0u8, 0u8);
    for i in 0..5usize {
        let l = lanes + (eside as usize) * LANE_STRIDE;
        let e = rd_u64(w.x + X_ROSTER + (eside as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize;
        if rd_i32(l + 0x78 + i * 0x18)? != -1 && e != 0 {
            let h = rd_u64(e + ENT_HANDLE)?;
            if w.visible(side, h)? { em |= 1 << i; }
            else { let rec = w.roster_rec(h)?; if rec != 0 { let idx = rd_u32(rec + REC_ROLE) as usize; if tick <= rd_u64(l + LANE_ROSTER + idx * 8)?.wrapping_add(120) { em |= 1 << i; } } }
        }
        let lm = lanes + (side as usize) * LANE_STRIDE;
        let o = rd_u64(w.x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize;
        if rd_i32(lm + 0x78 + i * 0x18)? != -1 && o != 0 { am |= 1 << i; }
        let _ = sim;
    }
    Some((em, am))
}
struct Facet { cfg_tick: u64, class: u8, ss: [bool; 3] }
unsafe fn facet(sim: usize, g: usize, tgt: usize) -> Option<Facet> {
    let cfg = rd_u64(g + G_CFG)? as usize; if !ptr_ok(cfg) { trs(|| "FACET_NA:cfg".into()); return None; }
    let tag = (rd_u8(g + G_PHASE) as usize).min(8);
    let cfg_tick = match rd_u64(cfg + CFGTICK_TBL[tag]) { Some(v) => v, None => { trs(|| "FACET_NA:cfgtick".into()); return None } };
    let (cd0, cv) = (rd_u64(sim + P5_CHAMP_DATA)? as usize, rd_u64(sim + P5_CHAMP_VT)? as usize);
    let cd = dy::arc_payload(cd0, cv)?;   // Box<dyn Champion> 페이로드 정렬(c580c4~c580d3) — 생 포인터를 넘기면 tags 가 쓰레기(2026-09-06 22:33)
    let fd = |what: &str| { let b = crate::exe_base(); let f30 = rd_u64(cv + 0x30).unwrap_or(0) as usize; let f28 = rd_u64(cv + 0x28).unwrap_or(0) as usize; let bytes = (0..16).map(|k| format!("{:02x}", rd_u8(f30 + k))).collect::<Vec<_>>().join(" ") + " f28:" + &(0..24).map(|k| format!("{:02x}", rd_u8(f28 + k))).collect::<Vec<_>>().join(" ");
        trs(|| format!("FACET_NA:{} vt={:#x} f20={:#x} f28={:#x} f30={:#x} [{}]", what, cv.wrapping_sub(b), rd_u64(cv + 0x20).unwrap_or(0).wrapping_sub(b as u64), rd_u64(cv + 0x28).unwrap_or(0).wrapping_sub(b as u64), f30.wrapping_sub(b), bytes)); };
    let kind = match champ_kind(cd, cv) { Some(v) => v, None => { fd("kind"); return None } };
    let (tp, tl) = match champ_tags(cd, cv) { Some(v) => v, None => { fd("tags"); return None } };
    let w2 = |need: bool| -> Option<u64> { if !need { return Some(0); } match champ_vt30_w2(cd, cv) { Some(v) => Some(v), None => { fd("vt30"); None } } };
    let t8 = match tags_has(tp, tl, 8) { Some(v) => v, None => { trs(|| format!("FACET_NA:tags8 tp={:#x} tl={}", tp, tl)); fd("tags8"); return None } };
    let class: u8 = if t8 { 0 } else { match kind {
        0 => if w2(true)? < 1200 { 2 } else { 0 },
        2 => 3,
        3 => if tags_has(tp, tl, 2)? || tags_has(tp, tl, 3)? { 4 } else if w2(true)? < 950 { 4 } else { 0 },
        4 => 5,
        _ => 2,
    } };
    let mut ss = [false; 3];
    for k in 0..3usize { ss[k] = match slot_of(tgt, k + 1).and_then(|sl| skillshot(sl)) { Some(v) => v, None => { trs(|| format!("FACET_NA:ss{}", k)); return None } }; }
    Some(Facet { cfg_tick, class, ss })
}

// ── 본체 상태 ───────────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
struct Elem { rec: Rec, sim: usize, ent: usize, d2: u64 }
pub struct Out { pub a: i64, pub b: i64, pub c: i64, pub maxc: i64, pub half28: i64, pub f30: u8, pub f31: u8 }

/// 본체 0xd851d0 (S0~S21). 반환 None = 읽기 실패/미재현(NA).
pub unsafe fn position_eval(mode: u64, sim: usize, holder: usize, qx: u64, qy: u64, purpose: u8) -> Option<Out> {
    let side = rd_u64(sim + P5_SIDE)?; if side > 1 { return None; }
    let role = rd_u32(sim + P5_ROLE) as usize; if role > 4 { return None; }
    let x = rd_u64(holder)? as usize; let g = rd_u64(holder + 8)? as usize; let lanes = rd_u64(holder + 0x10)? as usize;
    if !ptr_ok(x) || !ptr_ok(g) || !ptr_ok(lanes) { return None; }
    let w = World { x, data: rd_u64(x)? as usize, vt: rd_u64(x + 8)? as usize }; if !ptr_ok(w.data) || !ptr_ok(w.vt) { return None; }
    let tgt = rd_u64(x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + role * 8)? as usize;
    if tgt == 0 { return Some(Out { a: 0, b: 0, c: 0, maxc: 0, half28: 0, f30: 0, f31: 0 }); }
    let gx = qx / 32000; let gy = qy / 32000; let gxc = gx.min(29) as usize; let gyc = gy.min(29) as usize;
    let map = rd_u64(g + G_MAP)? as usize; if !ptr_ok(map) { return None; }
    trs(|| "P".into());
    if rd_u64(map + 0x78 + gyc * 0xf0 + gxc * 8)? != 0 { return Some(Out { a: 9999, b: 0, c: 0, maxc: 0, half28: 0, f30: 0, f31: 0 }); }
    let hp = rd_u64(tgt + ENT_HP)?; let hp1 = if hp == 0 { 1 } else { hp };
    let inv = 0x1_0000_0000u64 / hp1; let scale = inv.wrapping_mul(100);
    let eside = 1 - side;
    let vis_me = (rd_i32(w.data + W_GRID + (eside as usize) * W_GRID_SIDE + gyc * W_GRID_ROW + gxc * 4)? > 0) as u8;
    // S1
    let b0: i64 = {
        let inb = if side == 1 { (qx <= 64000 && qy.wrapping_sub(800000) < 160001) || (qx <= 160000 && qy.wrapping_sub(896000) < 64001) }
                  else { (qx.wrapping_sub(800000) < 160001 && qy <= 64000) || (qx.wrapping_sub(896000) < 64001 && qy <= 160000) };
        if inb { 9999 } else { 0 }
    };
    // S2
    let tick = rd_u64(w.data + W_TICK)?;
    let (emask, amask) = match masks(&w, lanes, sim, side, eside, tick) { Some(v) => v, None => { trs(|| "NA:masks".into()); return None } };
    trs(|| format!("M[e={:#x} a={:#x}]", emask, amask));
    let tbl_a = x + 0x280 + (side as usize) * 0xfa0 + role * 0x320;
    // S3
    let mut e_list: Vec<Elem> = Vec::with_capacity(5);
    for i in 0..5usize {
        if (emask >> i) & 1 == 0 { continue; }
        let ent = rd_u64(x + X_ROSTER + (eside as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize; if ent == 0 { return None; }
        let (ex, ey) = xy(ent)?; let d2 = sat_d2(ex, ey, qx, qy); if d2 > 40_000_000_000 { continue; }
        let simb = rd_u64(x + X_SIM_TABLE + (eside as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize; if simb == 0 { return None; }
        let sideb = rd_u64(simb + P5_SIDE)?; if sideb > 1 { return None; }
        let tbl_b = x + 0x280 + (sideb as usize) * 0xfa0 + (rd_u32(simb + P5_ROLE) as usize) * 0x320;
        let rec = match pair_record(tgt, ent, sim, simb, tbl_a, tbl_b, false) { Some(r) => r, None => { trs(|| format!("NA:pairE{}", i)); return None } };
        e_list.push(Elem { rec, sim: simb, ent, d2 });
    }
    // S4
    let flag = !e_list.is_empty();
    let mut a_list: Vec<Elem> = Vec::with_capacity(4);
    let th = rd_u64(tgt + ENT_HANDLE)?;
    for i in 0..5usize {
        if (amask >> i) & 1 == 0 { continue; }
        let ent = rd_u64(x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize; if ent == 0 { return None; }
        let (ex, ey) = xy(ent)?; let d2 = sat_d2(ex, ey, qx, qy); if d2 > 40_000_000_000 { continue; }
        if rd_u64(ent + ENT_HANDLE)? == th { continue; }
        let simb = rd_u64(x + X_SIM_TABLE + (side as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize; if simb == 0 { return None; }
        let sideb = rd_u64(simb + P5_SIDE)?; if sideb > 1 { return None; }
        let tbl_b = x + 0x280 + (sideb as usize) * 0xfa0 + (rd_u32(simb + P5_ROLE) as usize) * 0x320;
        let rec = match pair_record(tgt, ent, sim, simb, tbl_a, tbl_b, flag) { Some(r) => r, None => { trs(|| format!("NA:pairA{}", i)); return None } };
        a_list.push(Elem { rec, sim: simb, ent, d2 });
    }
    // S5
    let fc = match facet(sim, g, tgt) { Some(v) => v, None => { trs(|| "NA:facet".into()); return None } };
    let st = St { mode, sim, holder, x, g, lanes, w, tgt, side, eside, role, qx, qy, purpose, gx, gy, hp, hp1, scale, vis_me, b0, tick, e_list, a_list, fc };
    body(&st)
}
struct St { mode: u64, sim: usize, holder: usize, x: usize, g: usize, lanes: usize, w: World, tgt: usize, side: u64, eside: u64, role: usize, qx: u64, qy: u64, purpose: u8, gx: u64, gy: u64, hp: u64, hp1: u64, scale: u64, vis_me: u8, b0: i64, tick: u64, e_list: Vec<Elem>, a_list: Vec<Elem>, fc: Facet }

// ── 타워다이브 0xd96d00 캡처 링(순수 재현 경계: 전투 시뮬 0xe05450) ─────────────────────────────
//   훅(mod.rs cap_as_d96d00)이 (side, tower.5c0, tick) → (rax, rdx) 를 넣고, 포팅은 최신 항목부터 찾는다. 없으면 NA.
thread_local! { static DIVE: std::cell::Cell<([(u64, u64, u64, u64, u64); 32], usize)> = const { std::cell::Cell::new(([(0, 0, 0, 0, 0); 32], 0)) }; static DIVE_Q: std::cell::Cell<[(u64, u64, u64); 32]> = const { std::cell::Cell::new([(0, 0, 0); 32]) }; static DIVE_F: std::cell::Cell<[[u64; 8]; 32]> = const { std::cell::Cell::new([[0; 8]; 32]) }; }
/// 진단: 최근 d96d00 호출 시 콜러 프레임의 사거리 성분 [438,4a0,4a8,e8,r_item,r_tgt,lv,tgt]
pub fn dive_f_recent(side: u64, th: u64) -> Vec<[u64; 8]> { let pre = PRE_N.with(|c| c.get()); DIVE.with(|c| { let (a, n) = c.get(); DIVE_F.with(|q| { let qa = q.get(); (pre.max(n.saturating_sub(32))..n).filter(|&i| a[i % 32].0 == side && a[i % 32].1 == th).map(|i| qa[i % 32]).collect() }) }) }
/// 진단: 최근 d96d00 호출 시 콜러(position_eval 본체) 프레임의 qx/qy/item ([rbp+0x7a0]/[0x7a8]/[0x718])
pub fn dive_q_recent(side: u64, th: u64) -> Vec<(u64, u64, u64)> { let pre = PRE_N.with(|c| c.get()); DIVE.with(|c| { let (a, n) = c.get(); DIVE_Q.with(|q| { let qa = q.get(); (pre.max(n.saturating_sub(32))..n).filter(|&i| a[i % 32].0 == side && a[i % 32].1 == th).map(|i| qa[i % 32]).collect() }) }) }
thread_local! { static PRE_N: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
pub fn pre_mark() { let n = DIVE.with(|c| c.get().1); PRE_N.with(|c| c.set(n)); }
/// 이번 position_eval 호출 중 게임이 d96d00 을 (side, th) 로 부른 횟수
/// 이번 호출 중 게임이 d96d00 을 부른 (tower handle, rax, rdx) 전부
pub fn dive_all_since() -> Vec<(u64, u64, u64)> { let pre = PRE_N.with(|c| c.get()); DIVE.with(|c| { let (a, n) = c.get(); (pre.max(n.saturating_sub(32))..n).map(|i| (a[i % 32].1, a[i % 32].3, a[i % 32].4)).collect() }) }
pub fn dive_calls_since(side: u64, th: u64) -> usize { let pre = PRE_N.with(|c| c.get()); DIVE.with(|c| { let (a, n) = c.get(); (pre.max(n.saturating_sub(32))..n).filter(|&i| a[i % 32].0 == side && a[i % 32].1 == th).count() }) }
pub fn dive_push(side: u64, th: u64, tick: u64, r: u64, d: u64, q: (u64, u64, u64), f: [u64; 8]) { DIVE.with(|c| { let (mut a, n) = c.get(); a[n % 32] = (side, th, tick, r, d); DIVE_Q.with(|qq| { let mut b = qq.get(); b[n % 32] = q; qq.set(b); }); DIVE_F.with(|ff| { let mut b = ff.get(); b[n % 32] = f; ff.set(b); }); c.set((a, n + 1)); }); }
pub fn dive_lookup(side: u64, th: u64, tick: u64) -> Option<(u64, u64)> { DIVE.with(|c| { let (a, n) = c.get(); (n.saturating_sub(32)..n).rev().map(|i| a[i % 32]).find(|e| e.0 == side && e.1 == th && e.2 == tick).map(|e| (e.3, e.4)) }) }
/// 훅에서 부른다: p1=mode p2=&Holder p3=sim p4=tower
pub unsafe fn dive_record(p2: usize, p3: usize, p4: usize, r: u64, d: u64, rbp: usize, ra: usize, gbx: u64) {
    // ★진짜 필터 = 반환주소. position_eval 본체의 콜사이트는 `call 0xd96d00` @0xd88043 → 복귀 0xd88048.
    //   다른 함수에서 온 호출은 프레임이 달라 [rbp+…] 이 전부 쓰레기다.
    let base = crate::exe_base(); if base == 0 || ra != base + 0xd88048 { return; }
    // ★0xd96d00 은 position_eval 본체(0xd851d0 @d88043) 말고 **다른 함수에서도** 불린다.
    //   그 호출들은 rbp 프레임이 전혀 달라 [rbp+0x7a0] 등이 쓰레기값이고, 그대로 링에 넣으면
    //   "게임이 이 타워로 d96d00 을 불렀는가" 오라클이 오염된다(2026-09-07 03:30 실측:
    //   q=(70,0) f=[0,0,0,0,0,0,0,18] 같은 표본 다수 → 게이트 후보 통계가 rg+24000 쪽으로 끌려감).
    //   본체 프레임에서는 [rbp+0x718] 이 곧 item(=p4) 이므로 이것으로 걸러낸다.
    // ★`[rbp+0x718]` 은 **포인터가 아니라 스칼라 재사용 슬롯**이다(0xd879bf 에서 거리 항, 0xd87541 에서 좌표).
    //   이걸 item 과 비교하던 필터가 표본의 43% 를 **편향되게** 버리고 있었다 — 반환주소 필터만으로 충분하다
    //   (`0xd88043` 은 이미지 전역에서 유일한 진입로이고 우회 분기 0건, RE 2026-09-07 13:0x).
    if !ptr_ok(rbp) { return; }
    let x = rd_u64(p2).unwrap_or(0) as usize; let data = if ptr_ok(x) { rd_u64(x).unwrap_or(0) as usize } else { 0 };
    let tick = if ptr_ok(data) { rd_u64(data + W_TICK).unwrap_or(0) } else { 0 };
    let q = if ptr_ok(rbp) { (rd_u64(rbp + 0x7a0).unwrap_or(0), rd_u64(rbp + 0x7a8).unwrap_or(0), rd_u64(rbp + 0x718).unwrap_or(0)) } else { (0, 0, 0) };
    let g = |o: usize| if ptr_ok(rbp) { rd_u64(rbp + o).unwrap_or(u64::MAX) } else { 0 };
    let f = [g(0x378), g(0x380), g(0x388), g(0x390), g(0x3d8), g(0x3e0), g(0x408), g(0x720)];
    // ★결정적 계측: 게임이 실제로 게이트를 통과한 그 순간의 (타워좌표, q, 임계 성분) 을 그대로 남긴다.
    //   게이트 = |item.x−qx|² + |item.y−qy|² <= (438+4a0+(lv−1)*4a8+vt_e8+ri'+rt'+18000)²  (d87ea1~d8800d)
    //   여기서 d2 > T² 인 줄이 하나라도 나오면 게이트 변수 식별이 틀린 것이다.
    if ptr_ok(rbp) && ptr_ok(p4) {
        let (ix, iy) = (rd_u64(p4 + ENT_X).unwrap_or(0), rd_u64(p4 + ENT_Y).unwrap_or(0));
        let lv = f[6];
        let t = f[0].wrapping_add(f[1]).wrapping_add(lv.wrapping_sub(1).wrapping_mul(f[2])).wrapping_add(f[3])
                    .wrapping_add(f[4]).wrapping_add(f[5]).wrapping_add(18000);
        let d2 = absd(ix, q.0).wrapping_mul(absd(ix, q.0)).wrapping_add(absd(iy, q.1).wrapping_mul(absd(iy, q.1)));
        // 게이트 자체 임시 슬롯: [rbp+0x710]=item.y · [rbp+0x708]=item.438 · [rbp+0x6e8]=item.4a8
        // [rbp+0x698]/[rbp+0x6a0] = 게임이 q 를 가리키는 포인터(d8544f/d8545d 에서 lea). 이게 rbp+0x7a0/0x7a8 이 아니면
        // 게이트가 읽는 q 와 내가 읽는 q 가 다른 것이다. 포인터와 그 대상까지 같이 찍는다.
        // ★내 모델(range_g(item, self) + 18000)을 **게임이 실제로 통과시킨 그 (item, q)** 에서 검증한다.
        //   캡처된 호출은 전부 pass 표본이므로, 내 모델이 fail 을 내면 그건 확실한 오답이다(단측 오라클).
        {
            // ★self 는 유도하지 말고 게임 프레임에서 직접 읽는다 — 게이트가 쓰는 건 `[rbp+0x720]` 이고,
            //   `0xd853da` 에서 한 번만 기록된다(RE 2026-09-07). 유도값(p2/p3 기반)과 다르면 radius(self) 가
            //   달라져 T 가 통째로 어긋난다.
            {
                let se_frame = rd_u64(rbp + 0x720).unwrap_or(0) as usize;
                let side = rd_u64(p3 + P5_SIDE).unwrap_or(9);
                let role = if ptr_ok(p3) { rd_u32(p3 + P5_ROLE) as usize } else { 99 };
                let se_derived = if side <= 1 && role <= 4 && ptr_ok(x) {
                    rd_u64(x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + role * 8).unwrap_or(0) as usize
                } else { 0 };
                self_tally(se_frame == se_derived);
                let se = if ptr_ok(se_frame) { se_frame } else { se_derived };
                {
                    if ptr_ok(se) {
                        if let Some(rg) = range_g(p4, se) {
                            // ★RE 2026-09-07: 이 게이트의 상수는 **18000 하나뿐**이고 6000 은 exe 어디에도 없다.
                            //   그런데 `q = [rbp+0x7a0]/[rbp+0x7a8]` 로 만든 d2 는 rg+18000 을 20% 초과한다
                            //   ⟹ **q 슬롯이 틀렸다**는 뜻이다. 프레임을 훑어 `d2 <= (rg+18000)²` 를 만족하는
                            //   (o, o+8) 쌍을 전수로 찾고, 모든 표본에서 통하는 오프셋만 남긴다.
                            let t18 = rg.wrapping_add(18000);
                            // ⚠`rbx` 로 게임의 d² 를 읽으려 했으나 **실패**: 게이트 `cmp rbx,rax`(0xd8800d) 와
                            //   `call`(0xd88043) 사이에서 게임이 rbx 를 재사용해, 훅 진입 시점엔 항상 1 이었다
                            //   (실측 3,481,894건 전부 `game_d2=1`). callee-saved 라도 **같은 프레임 안에서는
                            //   자유롭게 덮어쓴다** — 호출 경계를 넘는 보존과 혼동하지 말 것.
                            let _ = gbx;
                            // ★RE 2026-09-07 정정 슬롯으로 T 를 다시 만들고, 성분별로 내 값과 대조한다.
                            //   f378=item.0x438 · f380=item.0x4a0 · f388=item.0x4a8 · f408=item.0x5c8(lv)
                            //   f390=vcall 반환 · f3d8=radius(item)(0x4c0 게이트 **미적용**) · f3e0=radius(self)
                            if ptr_ok(rbp) {
                                let fr = |o: usize| rd_u64(rbp + o).unwrap_or(u64::MAX);
                                let (f378, f380, f388, f408, f390, f3d8, f3e0) =
                                    (fr(0x378), fr(0x380), fr(0x388), fr(0x408), fr(0x390), fr(0x3d8), fr(0x3e0));
                                let gate4c0 = rd_i32(p4 + 0x4c0).unwrap_or(-9) == 0;
                                let rg_f = f380.wrapping_add(f378)
                                    .wrapping_add(f408.wrapping_sub(1).wrapping_mul(f388))
                                    .wrapping_add(f390)
                                    .wrapping_add(if gate4c0 { f3d8 } else { 0 })
                                    .wrapping_add(f3e0);
                                let rg_fu = rg_f.wrapping_sub(if gate4c0 { 0 } else { 0 }).wrapping_add(if gate4c0 { 0 } else { f3d8 }); // 게이트 미적용판
                                cmp_tally(0, d2 <= t18.wrapping_mul(t18));                       // 내 rg
                                cmp_tally(1, d2 <= (rg_f + 18000).wrapping_mul(rg_f + 18000));    // 프레임 rg(게이트 적용)
                                cmp_tally(2, d2 <= (rg_fu + 18000).wrapping_mul(rg_fu + 18000));  // 프레임 rg(게이트 미적용)
                                // 성분별 불일치 집계
                                comp_tally(0, f378 == rd_u64(p4 + ENT_F438).unwrap_or(u64::MAX));
                                comp_tally(1, f380 == rd_u64(p4 + 0x4a0).unwrap_or(u64::MAX));
                                comp_tally(2, f388 == rd_u64(p4 + 0x4a8).unwrap_or(u64::MAX));
                                comp_tally(3, f408 == rd_u64(p4 + ENT_LEVEL).unwrap_or(u64::MAX));
                                comp_tally(4, f390 == vt_e8(p4 + SLOT0, p4, se).unwrap_or(u64::MAX));
                                comp_tally(5, f3d8 == radius(p4).unwrap_or(u64::MAX));
                                comp_tally(6, f3e0 == radius(se).unwrap_or(u64::MAX));
                                comp_tally(7, rg_f == rg);
                            }
                            if ptr_ok(rbp) {
                                let (ix2, iy2) = (rd_u64(p4 + ENT_X).unwrap_or(0), rd_u64(p4 + ENT_Y).unwrap_or(0));
                                for k in 0..QSCAN_N {
                                    let o = QSCAN_BASE + k * 8;
                                    let (cx, cy) = (rd_u64(rbp + o).unwrap_or(u64::MAX), rd_u64(rbp + o + 8).unwrap_or(u64::MAX));
                                    let ok = cx != u64::MAX && cy != u64::MAX && cx < (1u64 << 40) && cy < (1u64 << 40)
                                             && wrap_d2(ix2, iy2, cx, cy) <= t18.wrapping_mul(t18);
                                    qscan(k, ok);
                                }
                            }
                            mine_truth(d2, rg.wrapping_add(18000), p4, se, q.0, q.1);
                            // ★후보별 단측 적중: 올바른 임계라면 게임이 통과시킨 표본을 **100%** 통과해야 한다.
                            //   100% 인 것들 중 **가장 작은 것**이 정답(더 큰 임계는 다른 곳에서 과다 통과한다).
                            let (ri, rt) = (radius(p4).unwrap_or(0), radius(se).unwrap_or(0));
                            let ru = range_u(p4, se).unwrap_or(0);
                            let cands = [rg + 18000, rg + 20000, rg + 22000, rg + 24000, rg + 26000, rg + 30000,
                                         rg + 18000 + ri, rg + 18000 + rt, rg + 18000 + ri + rt,
                                         ru + 18000, ru + 18000 + ri, ru + 24000,
                                         rg + 32000, rg + 50000, rg * 2, rg];
                            for (i, c) in cands.iter().enumerate() { cand_tally(i, d2 <= c.wrapping_mul(*c)); }
                            // ★필요 임계 역산: 게임이 통과시켰으므로 T_true >= need. 각 형태의 **최댓값**이
                            //   곧 그 형태가 성립하기 위한 하한이고, 그게 딱 18000 이면 그 형태가 정답이다.
                            let need = isqrt_fast(d2);
                            need_max(0, need.saturating_sub(rg));
                            need_max(1, need.saturating_sub(rg).saturating_sub(ri));
                            need_max(2, need.saturating_sub(rg).saturating_sub(rt));
                            need_max(3, need.saturating_sub(rg).saturating_sub(ri).saturating_sub(rt));
                            need_max(4, need.saturating_sub(ru));
                            need_max(5, need.saturating_sub(rg).saturating_sub(ri / 2));
                        }
                    }
                }
            }
        }
        let (qp, qp2) = (g(0x698), g(0x6a0));
        let gs = [g(0x710), g(0x708), g(0x6e8), qp.wrapping_sub((rbp + 0x7a0) as u64), qp2.wrapping_sub((rbp + 0x7a8) as u64),
                  rd_u64(qp as usize).unwrap_or(u64::MAX), rd_u64(qp2 as usize).unwrap_or(u64::MAX), rd_u64(p4 + 0x4a0).unwrap_or(0)];
        gate_truth(d2, t, ix, iy, q.0, q.1, &f, &gs);
    }
    dive_push(rd_u64(p3 + P5_SIDE).unwrap_or(9), rd_u64(p4 + ENT_HANDLE).unwrap_or(0), tick, r, d, q, f);
}

// ── S6~S21 콜리 ───────────────────────────────────────────────────────────────────────────
/// c7f0f0 → 12857f0 (est 메모: 투명). o.4c0==-1 이면 게임 panic → None
#[inline] unsafe fn est(o: usize, t: usize) -> Option<u64> { if rd_i32(o + 0x4c0)? == -1 { return None; } estimate_damage(o + SLOT0, o, t, false) }
/// 공격 주기(틱): vt90(item.570/578) ×100 / max(1, 3fc+100) , ≥3
unsafe fn atk_iv(e: usize) -> Option<u64> {
    let base = dy::prov90_cooltime(rd_u64(e + 0x570)? as usize, rd_u64(e + 0x578)? as usize, e)?;
    let a = (rd_i32(e + 0x3fc)?).wrapping_add(100); let den = if a < 2 { 1u64 } else { a as u32 as u64 };
    Some((base.wrapping_mul(100) / den).max(3))
}
unsafe fn tower_v(item: usize, es: u64, tps: u64, scale: u64) -> Option<u64> {
    let iv = atk_iv(item)?; let hits = tps.wrapping_mul(es) / iv; Some(cap150(hits.wrapping_add(es), scale))
}
/// 내 편 유닛 3리스트(X+0x10/0x50/0x90 + side*0x20, len +0x18) 순회
unsafe fn units<F: FnMut(usize) -> Option<()>>(x: usize, side: u64, mut f: F) -> Option<()> {
    for base in [0x10usize, 0x50, 0x90] {
        let ptr = rd_u64(x + base + (side as usize) * 0x20)? as usize; let len = rd_u64(x + base + (side as usize) * 0x20 + 0x18)?;
        if len == 0 || ptr == 0 { continue; } if !ptr_ok(ptr) { return None; }
        for i in 0..len.min(512) as usize { let u = rd_u64(ptr + i * 8)? as usize; if u == 0 { return None; } f(u)?; }
    }
    Some(())
}
/// c7f370: 내 편(side) 유닛 중 item 사거리(range_g) 안 개수
unsafe fn count_in_range(x: usize, side: u64, item: usize) -> Option<u64> {
    if rd_i32(item + 0x4c0)? == -1 { return None; }
    let (ix, iy) = xy(item)?; let mut acc = 0u64;
    units(x, side, |u| { let (ux, uy) = xy(u)?; let r = range_g(item, u)?; if wrap_d2(ux, uy, ix, iy) <= sq(r) { acc += 1; } Some(()) })?;
    Some(acc)
}
/// 0xd95d00: 적 kind1 유닛 노출 합
pub unsafe fn exposure(x: usize, tgt: usize, qx: u64, qy: u64, t_raw: u64, tps: u64) -> Option<u64> {
    if rd_u8(tgt) != 0 { return Some(0); }
    let eside = 1u64.wrapping_sub(rd_u64(tgt + 8)?); if eside > 1 { return None; }
    let tp = (tps >> 1).max(t_raw);   // 게임 0xd95d00: Tp = max(cfg.12f8>>1, T)
    let th = rd_u64(tgt + ENT_HANDLE)?; let wk = if rd_i32(tgt + ENT_KIND)? == 13 { 60u64 } else { 40 };
    let mut acc = 0u64;
    units(x, eside, |u| {
        let (ux, uy) = xy(u)?; let d2s = sat_d2(ux, uy, qx, qy);
        if imp(7, d2s < 0x3_5a4e_9001, d2s < imm64(SITE_PE_COUNT_D2_INC_IMM, 0x3_5a4e_9001)) && rd_i32(u + ENT_KIND)? == 1 && rd_i32(u + 0x4c0)? != -1 {
            let es = est(u, tgt)?; if es == 0 { return Some(()); }
            let iv = atk_iv(u)?; let hits = es.wrapping_mul(tp) / iv;
            let w1 = if rd_i32(u + 0x88)? == 1 { if rd_u64(u + 0x90)? == th { 100 } else { wk } } else { 80u64 };
            let w2 = if d2s > 8_100_000_000 { if rd_u8(u + 0x118) != 0 { 65 } else { 45 } } else { 100u64 };
            acc = acc.saturating_add(w1.wrapping_mul(hits.wrapping_add(es)).wrapping_mul(w2) / 10000);
        }
        Some(())
    })?;
    let hp = rd_u64(tgt + ENT_HP)? as i64; let mut cap = if hp < 0 { u64::MAX } else { (2 * hp) as u64 }; if cap == 0 { cap = 1; }
    Some(acc.min(cap))
}
/// 공통 티어(S13/S14/S16): 비교 unsigned, 나눗셈 signed 0 방향
#[inline] fn tier(v: i64, d2: u64, flag: bool, ra: u64, rb: u64, rc: u64, rng: u64, r_b: u64, r_a: u64) -> Option<i64> {
    if !flag { if d2 <= ra { Some(v) } else if d2 <= rb { Some(sdiv2(v)) } else { None } }
    else if d2 <= ra { if rng > r_b.wrapping_add(80000).wrapping_add(r_a) && d2 > rc { Some(sdiv2(v)) } else { Some(v) } }
    else if d2 <= rc { Some(v) } else if d2 <= rb { Some(sdiv3(v)) } else { None }
}
/// c875a0 usable(e,k): 슬롯 있음 ∧ (쿨 < 0xb5 ∨ kind≠13)
unsafe fn usable(e: usize, k: usize) -> Option<bool> {
    let s = slot_of(e, k)?; if slot_id(s)? == -1 { return Some(false); }
    let cd = rd_u64(e + 0xb8 + (k - 1) * 8)?; Some(cd < 0xb5 || rd_i32(e + ENT_KIND)? != 13)
}
/// Effect vt+0x88 플래그 트리(c872f0 값): 0x9db70=0 · 0x1147250/0x1145640=1 · 합성 0x12a57b0(stride 0x10 @+8/+0x10) · 0x12a61c0(stride 0x18 @+0x20/+0x28)
unsafe fn vt88_flag(data: usize, vt: usize, depth: u32) -> Option<u64> {
    if depth > 40 { super::dyn_eff::unseen(0x621, depth as usize); return None; }
    if !ptr_ok(vt) { return None; }
    let r = dy::impl_rva(vt, 0x88)?; let p = dy::arc_payload(data, vt)?;
    let any = |ptr_off: usize, len_off: usize, stride: usize| -> Option<u64> {
        let n = rd_u64(p + len_off)?; if n == 0 { return Some(0); } let arr = rd_u64(p + ptr_off)? as usize; if !ptr_ok(arr) { return None; }
        for i in 0..n.min(CAP_ITER) as usize { if vt88_flag(rd_u64(arr + i * stride)? as usize, rd_u64(arr + i * stride + 8)? as usize, depth + 1)? & 1 == 1 { return Some(1); } } Some(0)
    };
    match r { EFF_E8_ZERO => Some(0), 0x1147250 | 0x1145640 => Some(1), 0x12a57b0 => any(8, 0x10, 0x10), 0x12a61c0 => any(0x20, 0x28, 0x18),
        0x12481a0 => any(0x50, 0x58, 0x18), 0x13bfc40 => any(8, 0x10, 0x18), 0x13bfa20 | 0x16adaa0 | 0x109bab0 | 0x122f090 => Some(1),
        0x17c2bd0 => Some((rd_u64(p)? != 0) as u64),
        0x1701740 => Some((rd_u64(p + 0x48)? != 0) as u64),
        0x1146c40 => { let n = rd_u64(p + 0x28)?; if n == 0 { return Some(0); } let arr = rd_u64(p + 0x20)? as usize; if !ptr_ok(arr) { return None; }   // 자식(stride 0x18 @p+0x20/0x28) 의 vt+0x80 중 첫 al&1 의 rax
            for i in 0..n.min(CAP_ITER) as usize { let v = vt80_flag(rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize, depth + 1)?; if v & 1 == 1 { return Some(v); } } Some(0) }
        0x1153880 => vtb0_flag(rd_u64(p)? as usize, rd_u64(p + 8)? as usize),
        0x164ecc0 => { let a = vt88_flag(rd_u64(p)? as usize, rd_u64(p + 8)? as usize, depth + 1)?; let b = vt88_flag(rd_u64(p + 0x10)? as usize, rd_u64(p + 0x18)? as usize, depth + 1)?; Some(if a & 1 == 1 { a } else { b }) }   // 두 자식: al1 이면 r1 아니면 r2
        0x1606700 => { let a = vt88_flag(rd_u64(p + 0x18)? as usize, rd_u64(p + 0x20)? as usize, depth + 1)?; let b = vt88_flag(rd_u64(p + 0x28)? as usize, rd_u64(p + 0x30)? as usize, depth + 1)?; Some(if a & 1 == 1 { a } else { b }) }
        0x16a3450 => {   // 리스트1(stride 0x18 @p+0x50/0x58) → 리스트2(stride 0x18 @p+0x68/0x70)
            let n1 = rd_u64(p + 0x58)?; if n1 != 0 { let arr = rd_u64(p + 0x50)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n1.min(CAP_ITER) as usize { if vt88_flag(rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize, depth + 1)? & 1 == 1 { return Some(1); } } }
            any(0x68, 0x70, 0x18)
        }
        0x1340ef0 => {   // 리스트1(stride 0x18 @p+0x68/0x70) → 리스트2(stride 0x10 @p+0x80/0x88)
            let n1 = rd_u64(p + 0x70)?; if n1 != 0 { let arr = rd_u64(p + 0x68)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n1.min(CAP_ITER) as usize { if vt88_flag(rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize, depth + 1)? & 1 == 1 { return Some(1); } } }
            any(0x80, 0x88, 0x10)
        }
        0x153b5a0 => {   // 리스트1(stride 0x18 @p+0x50/len p+0x58) 다음 리스트2(stride 0x10 @p+0x68/len p+0x70) 중 al&1 → 1
            let n1 = rd_u64(p + 0x58)?; if n1 != 0 { let arr = rd_u64(p + 0x50)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n1.min(CAP_ITER) as usize { if vt88_flag(rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize, depth + 1)? & 1 == 1 { return Some(1); } } }
            any(0x68, 0x70, 0x10)
        }
        0x12a5100 => {   // 자식(stride 0x10 @p+0x48/len p+0x50) 중 첫 al&1 인 자식의 rax 가 정확히 1 이면 1 (capstone 2026-09-06 22:00)
            let n = rd_u64(p + 0x50)?; if n == 0 { return Some(0); } let arr = rd_u64(p + 0x48)? as usize; if !ptr_ok(arr) { return None; }
            for i in 0..n.min(CAP_ITER) as usize { let v = vt88_flag(rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize, depth + 1)?; if v & 1 == 1 { return Some((v == 1) as u64); } }
            Some(0)
        }
        _ => { dy::unseen(0x388, r); None } }
}
#[inline] unsafe fn memo88(e: usize, k: usize) -> Option<u64> { match slot_of(e, k)? { None => Some(0), Some(s) => { if rd_i32(s + 0x30)? == -1 { return Some(0); } vt88_flag(rd_u64(s)? as usize, rd_u64(s + 8)? as usize, 0) } } }
#[inline] unsafe fn vt120_of(s: usize) -> Option<bool> { eff_vt120(rd_u64(s)? as usize, rd_u64(s + 8)? as usize, 0) }
/// 0x126be70 / 0xe2c770 방어 적용 피해(ty=a.128 / e2c770 은 ty=1)
unsafe fn conv(src: usize, tgt: usize, mut amt: u64, ty: u32, flag: u32) -> Option<u64> {
    if flag > 1 { return Some(amt); }
    let amp = |a: u64| -> Option<u64> { let v = rd_u64(src + 0x460)?; Some(if v != 0 { v.wrapping_add(100).wrapping_mul(a) / 100 } else { a }) };
    let own = |a: u64| -> Option<u64> { let v = rd_u64(src + 0x448)?; Some(if v != 0 { a.wrapping_add(v.wrapping_mul(rd_u64(src + ENT_MAXHP)?) / 100) } else { a }) };
    if ty == 2 || ty == 3 { amt = amp(amt)?; }
    else if ty == 0 { let v = rd_u64(src + 0x440)?; if v != 0 { amt = amt.wrapping_add(v.wrapping_mul(rd_u64(tgt + ENT_MAXHP)?) / 100); } amt = own(amt)?; }
    else { let v = rd_u64(src + 0x450)?; if v != 0 { amt = amt.wrapping_add(v.wrapping_mul(rd_u64(tgt + ENT_MAXHP)?) / 100); } if (ty & 6) == 2 { amt = amp(amt)?; } else { amt = own(amt)?; } }
    let (mut res, pen) = if flag == 0 { (rd_u64(tgt + 0x630)?, rd_u64(src + 0x418)?) } else { (rd_u64(tgt + 0x638)?, rd_u64(src + 0x420)?) };
    if pen != 0 { res = (if pen < 101 { 100 - pen } else { 0 }).wrapping_mul(res) / 100; }
    let den = res.wrapping_add(100); if den == 0 { return None; }
    let q = amt.wrapping_mul(100) / den; Some(q.max(1))
}
/// 점 P ↔ 선분 A→B 거리 (0x12a04b0)
fn seg_dist(px: u64, py: u64, ax: u64, ay: u64, bx: u64, by: u64) -> Option<u64> {
    let (dx, dyv) = (bx.wrapping_sub(ax) as i64, by.wrapping_sub(ay) as i64); let (vx, vy) = (px.wrapping_sub(ax) as i64, py.wrapping_sub(ay) as i64);
    let len2 = dx.wrapping_mul(dx).wrapping_add(dyv.wrapping_mul(dyv));
    let t = if len2 == 0 { 0 } else { let num = vx.wrapping_mul(dx).wrapping_add(vy.wrapping_mul(dyv)).wrapping_mul(10000); if num == i64::MIN && len2 == -1 { return None; } (num / len2).clamp(0, 10000) };
    let rx = vx.wrapping_sub(dx.wrapping_mul(t) / 10000); let ry = vy.wrapping_sub(dyv.wrapping_mul(t) / 10000);
    let d2 = (rx.wrapping_mul(rx) as u64).wrapping_add(ry.wrapping_mul(ry) as u64);
    Some(isqrt_fast(d2))
}
// ── S11 액션 ─────────────────────────────────────────────────────────────────────────────
struct Act { a: usize, k: u64 }
#[inline] unsafe fn same_team_ae(a: usize, e: usize) -> Option<bool> { Some(same_team_ab(rd_u64(a)?, rd_u64(a + 8)?, rd_u64(e)?, rd_u64(e + 8)?)) }
unsafe fn has_status_set(e: usize) -> Option<bool> {
    let n = rd_u64(e + 0x2d0)?; if n == 0 { return Some(false); } let p = rd_u64(e + 0x2c8)? as usize; if !ptr_ok(p) { return None; }
    for i in 0..n.min(CAP_ITER) as usize { let k = rd_u32(p + i * 0x28); if k <= 9 && (0x347u32 >> k) & 1 == 1 { return Some(true); } } Some(false)
}
/// 0x129feb0 relevant(tt, a, e) — 시전자 핸들 a.f8 기준
unsafe fn relevant(tt: u32, a: usize, e: usize) -> Option<bool> {
    if rd_u8(e + 0x6b9) != 1 || rd_u64(e + 0x6a0)? != 0 { return Some(false); }
    let same = same_team_ae(a, e)?; let ek = rd_i32(e + ENT_KIND)?; let not23 = (ek & !1) != 2; let selfh = rd_u64(a + 0xf8)? == rd_u64(e + ENT_HANDLE)?;
    Some(match tt {
        0 => same, 1 => same && ek == 13, 2 => same && ek == 13 && has_status_set(e)?, 3 => same && !selfh, 4 => selfh,
        5 => !same, 6 => !same && not23, 7 => !same && ek == 13, 8 => !same && ek == 13 && has_status_set(e)?,
        9 => { let (a0, a8) = (rd_u64(a)?, rd_u64(a + 8)?); if a0 != 0 || rd_u64(e)? != 0 || a8 == rd_u64(e + 8)? || ek != 13 { false } else { if a8 >= 2 { return None; } rd_u64(e + 0x688 + (a8 as usize) * 8)? != 0 } }
        10 => true, 11 => !selfh && not23, 12 => ek == 13, 13 => false, _ => return None,
    })
}
/// 0x13381c0 도형 커버 (shape @s: +0 tag, +8 w, +0x10.., ; a=origin b=pos)
unsafe fn shape_cover(s: usize, ax: u64, ay: u64, bx: u64, by: u64, extra: u64, qx: u64, qy: u64, r: u64) -> Option<bool> {
    let w = rd_u64(s + 8)?;
    match rd_u64(s)? {
        0 => { let rr = r.wrapping_add(extra).wrapping_add(w); Some(wrap_d2(qx, qy, bx, by) <= sq(rr)) }
        1 => {
            let (px, py, ox, oy) = (rd_u64(s + 0x10)? as i64, rd_u64(s + 0x18)? as i64, rd_u64(s + 0x20)? as i64, rd_u64(s + 0x28)? as i64);
            let (vx, vy) = (ox.wrapping_sub(px), oy.wrapping_sub(py)); let l = vx.wrapping_mul(vx).wrapping_add(vy.wrapping_mul(vy));
            let (ux, uy) = ((qx as i64).wrapping_sub(px), (qy as i64).wrapping_sub(py));
            let t = if l == 0 { 0 } else { let num = ux.wrapping_mul(vx).wrapping_add(uy.wrapping_mul(vy)).wrapping_mul(10000); if num == i64::MIN && l == -1 { return None; } (num / l).clamp(0, 10000) };
            let (cx, cy) = (px.wrapping_add(vx.wrapping_mul(t) / 10000), py.wrapping_add(vy.wrapping_mul(t) / 10000));
            let rr = r.wrapping_add(w); Some(wrap_d2(qx, qy, cx as u64, cy as u64) <= sq(rr))
        }
        2 => Some(absd(qx, bx) <= (w >> 1).wrapping_add(r) && absd(qy, by) <= (rd_u64(s + 0x10)? >> 1).wrapping_add(r)),
        3 => {
            let d2 = wrap_d2(qx, qy, bx, by); let rr = r.wrapping_add(extra).wrapping_add(w); if d2 > sq(rr) { return Some(false); }
            let dot = ((qx.wrapping_sub(bx)) as i64).wrapping_mul((bx.wrapping_sub(ax)) as i64).wrapping_add(((qy.wrapping_sub(by)) as i64).wrapping_mul((by.wrapping_sub(ay)) as i64));
            let ab2 = wrap_d2(bx, by, ax, ay);
            let thr = (isqrt_fast(d2) as i64).wrapping_mul(rd_u64(s + 0x10)? as i64).wrapping_mul(isqrt_fast(ab2) as i64) / 1000;
            Some(dot >= thr)
        }
        _ => None,
    }
}
/// 0x132aa00 cover(a, q, r)
unsafe fn cover(ac: &Act, qx: u64, qy: u64, r: u64) -> Option<bool> {
    let a = ac.a;
    match ac.k {
        k if k > 8 => Some(true), 4..=7 => Some(true),
        1..=3 => shape_cover(a + 0x10, rd_u64(a + 0x110)?, rd_u64(a + 0x118)?, rd_u64(a + 0x100)?, rd_u64(a + 0x108)?, 0, qx, qy, r),
        8 => { let (cx, cy) = (rd_u64(a + 0x70)?, rd_u64(a + 0x78)?); shape_cover(a + 0x10, cx, cy, cx, cy, 0, qx, qy, r) }
        _ => match rd_u64(a + 0x10)? {
            0 => Some(seg_dist(qx, qy, rd_u64(a + 0x100)?, rd_u64(a + 0x108)?, rd_u64(a + 0x68)?, rd_u64(a + 0x70)?)? <= r.wrapping_add(rd_u64(a + 0x18)?).wrapping_add(15000)),
            1 => Some(seg_dist(qx, qy, rd_u64(a + 0x20)?, rd_u64(a + 0x28)?, rd_u64(a + 0x30)?, rd_u64(a + 0x38)?)? <= r.wrapping_add(rd_u64(a + 0x18)?).wrapping_add(15000)),
            2 => Some(absd(qx, rd_u64(a + 0x100)?) <= (rd_u64(a + 0x18)? >> 1).wrapping_add(r).wrapping_add(15000) && absd(qy, rd_u64(a + 0x108)?) <= (rd_u64(a + 0x20)? >> 1).wrapping_add(r).wrapping_add(15000)),
            _ => None,
        },
    }
}
/// hitset(a.c0 ctrl, a.c8 mask, a.d8 items) 핸들 멤버십
unsafe fn hit(a: usize, e: usize) -> Option<bool> {
    if rd_u64(a + 0xd8)? == 0 { return Some(false); }
    let ctrl = rd_u64(a + 0xc0)? as usize; let mask = rd_u64(a + 0xc8)?; if !ptr_ok(ctrl) { return None; }
    let h = rd_u64(e + ENT_HANDLE)?;
    for i in 0..=mask.min(4095) as usize { if rd_u8(ctrl + i) & 0x80 == 0 { if rd_u64(ctrl - (i + 1) * 8)? == h { return Some(true); } } }
    Some(false)
}
#[inline] unsafe fn ticks(a: usize) -> Option<u64> { let m = rd_u64(a + 0x68)?.max(1); Some(rd_u64(a + 0x60)?.wrapping_add(m).wrapping_sub(1) / m) }
/// Σ vt+0x28 / vt+0x38 / vt+0x40 / vt+0x48 / any vt+0x80 over a (data, vt) list
unsafe fn list_iter<F: FnMut(usize, usize) -> Option<()>>(ptr: usize, len: u64, stride: usize, mut f: F) -> Option<()> {
    if len == 0 { return Some(()); } if !ptr_ok(ptr) { return None; }
    for i in 0..len.min(CAP_ITER) as usize { f(rd_u64(ptr + i * stride)? as usize, rd_u64(ptr + i * stride + 8)? as usize)?; } Some(())
}
unsafe fn eff48_shield(data: usize, vt: usize, src: usize) -> Option<u64> {
    let r = dy::impl_rva(vt, 0x48)?; let p = dy::arc_payload(data, vt)?;
    match r {
        EFF_E8_ZERO => Some(0),
        // 0x1147260: st=src.618 → p.138 + p.140*st[0]/100 + p.148*st[8]/100 + p.150*st[0x10]/100 (capstone 2026-09-06 22:50)
        0x1147260 => { let st = src + ENT_STATS;
            let a = rd_u64(p + 0x140)?.wrapping_mul(rd_u64(st)?) / 100;
            let b = rd_u64(p + 0x148)?.wrapping_mul(rd_u64(st + 8)?) / 100;
            let c = rd_u64(p + 0x150)?.wrapping_mul(rd_u64(st + 0x10)?) / 100;
            Some(a.wrapping_add(b).wrapping_add(rd_u64(p + 0x138)?).wrapping_add(c)) }
        _ => { dy::unseen(0x348, r); None }
    }
}
/// Effect vt+0x80(self) 플래그: 0x9db70=0 · EFF_TRUE/0x1147250=1 · 0x12a6b80 = 자식(stride 0x10 @p+8/0x10) 중 첫 al&1 의 rax
unsafe fn vt80_flag(data: usize, vt: usize, depth: u32) -> Option<u64> {
    if depth > 40 { super::dyn_eff::unseen(0x622, depth as usize); return None; }
    if !ptr_ok(vt) { return None; }
    let r = dy::impl_rva(vt, 0x80)?; let p = dy::arc_payload(data, vt)?;
    match r {
        EFF_E8_ZERO => Some(0), EFF_TRUE | 0x1147250 | 0x1145640 | 0x122fcf0 => Some(1),
        0x1606550 => vt80_flag(rd_u64(p + 0x18)? as usize, rd_u64(p + 0x20)? as usize, depth + 1),
        0x12a6b80 => { let n = rd_u64(p + 0x10)?; if n == 0 { return Some(0); } let arr = rd_u64(p + 8)? as usize; if !ptr_ok(arr) { return None; }
            for i in 0..n.min(CAP_ITER) as usize { let v = vt80_flag(rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize, depth + 1)?; if v & 1 == 1 { return Some(v); } } Some(0) }
        // 0x1146c40: 자식(ptr p+0x20 / len p+0x28 / stride 0x18) 중 첫 `al&1` 의 rax 전체. 없으면 0. (RE 2026-09-07)
        0x1146c40 => { let n = rd_u64(p + 0x28)?; if n == 0 { return Some(0); }
            let arr = rd_u64(p + 0x20)? as usize; if !ptr_ok(arr) { return None; }
            for i in 0..n.min(CAP_ITER) as usize {
                let v = vt80_flag(rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize, depth + 1)?;
                if v & 1 == 1 { return Some(v); } }
            Some(0) }
        0x16adaa0 => Some(1),                     // mov rdx,[rcx+0x30]; mov eax,1; ret
        0x13bfa20 => Some(1),                     // mov rdx,[rcx+8];    mov eax,1; ret
        _ => { dy::unseen(0x380, r); None }
    }
}
unsafe fn eff80_cc(data: usize, vt: usize) -> Option<bool> { Some(vt80_flag(data, vt, 0)? == 1) }
/// 자식 Box<dyn> 의 vt+0xb0(data 그대로, 정렬 없음) — 0x1153880 이 tail-call 한다. 구현체는 unseen(0x3b0) 으로 수집
unsafe fn vtb0_flag(data: usize, vt: usize) -> Option<u64> { let r = dy::impl_rva(vt, 0xb0)?; let _ = data; match r { EFF_E8_ZERO => Some(0), EFF_TRUE => Some(1), _ => { dy::unseen(0x3b0, r); None } } }
unsafe fn dmg_list(ptr: usize, len: u64, stride: usize, a: usize, src: usize, tgt: usize) -> Option<u64> {
    let (mut p, mut m, mut pc) = (0u64, 0u64, 0u64);
    list_iter(ptr, len, stride, |d, v| { let (x, y) = dy::eff28_damage(d, v, src)?; p = p.wrapping_add(x); m = m.wrapping_add(y); pc = pc.wrapping_add(dy::eff38_pct(d, v, src)?); Some(()) })?;
    let raw = p.wrapping_add(pc.wrapping_mul(rd_u64(tgt + ENT_MAXHP)?) / 100);
    if raw == 0 && m == 0 { return Some(0); }
    // ★공통 꼬리(132bba3~132bbe8, k=0/1/3/4/5/6/7): `raw|M != 0` 이면 **두 conv 를 모두** 부른다 → 0 인 쪽도 conv 의 `cmp 1; adc 0` 로 1 을 기여한다.
    //   k=2 만 각 항을 따로 0 검사해 건너뛴다(`dmg_list_k2`). 2026-09-07 00:1x 아암 전수 디스어셈으로 확정.
    let ty = rd_u32(a + 0x128); Some(conv(src, tgt, raw, ty, 0)?.wrapping_add(conv(src, tgt, m, ty, 1)?))
}
/// k=2 아암 전용: 각 항이 0 이면 conv 를 **호출하지 않는다**(게임 132be36 `je` · 132be85 `test r13,r13; je` · L_50 쪽 132bfa6·132bfe0 동일).
unsafe fn dmg_list_k2(ptr: usize, len: u64, stride: usize, a: usize, src: usize, tgt: usize) -> Option<u64> {
    let (mut p, mut m, mut pc) = (0u64, 0u64, 0u64);
    list_iter(ptr, len, stride, |d, v| { let (x, y) = dy::eff28_damage(d, v, src)?; p = p.wrapping_add(x); m = m.wrapping_add(y); pc = pc.wrapping_add(dy::eff38_pct(d, v, src)?); Some(()) })?;
    let raw = p.wrapping_add(pc.wrapping_mul(rd_u64(tgt + ENT_MAXHP)?) / 100);
    let ty = rd_u32(a + 0x128);
    let ca = if raw != 0 { conv(src, tgt, raw, ty, 0)? } else { 0 };
    let cb = if m != 0 { conv(src, tgt, m, ty, 1)? } else { 0 };
    Some(ca.wrapping_add(cb))
}
unsafe fn sum_list(ptr: usize, len: u64, stride: usize, slot: usize, src: usize) -> Option<u64> {
    let mut acc = 0u64;
    list_iter(ptr, len, stride, |d, v| { acc = acc.wrapping_add(if slot == 0x40 { dy::eff40_heal(d, v, src)? } else { eff48_shield(d, v, src)? }); Some(()) })?; Some(acc)
}
#[inline] unsafe fn lb0(a: usize) -> Option<(usize, u64)> { Some((rd_u64(a + 0xb0)? as usize, rd_u64(a + 0xb8)?)) }
#[inline] unsafe fn l50(a: usize) -> Option<(usize, u64)> { Some((rd_u64(a + 0x50)? as usize, rd_u64(a + 0x58)?)) }
/// 0x132b310 threat / 0x132ad30 heal / 0x132c200 shield (관련성은 호출부가 검사)
unsafe fn threat(ac: &Act, src: usize, tgt: usize) -> Option<u64> {
    let a = ac.a; let (b0, bn) = lb0(a)?;
    match ac.k {
        0 => { let (p, n) = l50(a)?; Some((if hit(a, tgt)? { 0 } else { dmg_list(b0, bn, 0x18, a, src, tgt)? }).wrapping_add(dmg_list(p, n, 0x10, a, src, tgt)?)) }
        1 => if rd_u8(a + 0x58) != 0 { Some(0) } else { dmg_list(b0, bn, 0x18, a, src, tgt) },
        2 => { let (p, n) = l50(a)?; Some(dmg_list_k2(b0, bn, 0x18, a, src, tgt)?.wrapping_mul(ticks(a)?).wrapping_add(dmg_list_k2(p, n, 0x18, a, src, tgt)?)) }
        3 | 6 | 7 => if hit(a, tgt)? { Some(0) } else { dmg_list(b0, bn, 0x18, a, src, tgt) },
        4 | 5 => dmg_list(b0, bn, 0x18, a, src, tgt),
        8 => { let (p, n) = l50(a)?; dmg_list(p, n, 0x10, a, src, tgt) }
        _ => Some(0),
    }
}
unsafe fn heal_shield(ac: &Act, src: usize, tgt: usize, slot: usize) -> Option<u64> {
    let a = ac.a; let (b0, bn) = lb0(a)?;
    let s = match ac.k {
        0 => { let (p, n) = l50(a)?; (if hit(a, tgt)? { 0 } else { sum_list(b0, bn, 0x18, slot, src)? }).wrapping_add(sum_list(p, n, 0x10, slot, src)?) }
        1 => if rd_u8(a + 0x58) != 0 { 0 } else { sum_list(b0, bn, 0x18, slot, src)? },
        2 => { let (p, n) = l50(a)?; sum_list(b0, bn, 0x18, slot, src)?.wrapping_mul(ticks(a)?).wrapping_add(sum_list(p, n, 0x18, slot, src)?) }
        3 => if hit(a, tgt)? { 0 } else { sum_list(b0, bn, 0x18, slot, src)? },
        4 | 5 => sum_list(b0, bn, 0x18, slot, src)?,
        8 => { let (p, n) = l50(a)?; sum_list(p, n, 0x10, slot, src)? }
        _ => 0,
    };
    if slot == 0x40 { let (mh, hp) = (rd_u64(tgt + ENT_MAXHP)?, rd_u64(tgt + ENT_HP)?); let missing = if mh >= hp { mh - hp } else { 0 }; Some(s.min(missing)) } else { Some(s) }
}
unsafe fn is_cc(ac: &Act) -> Option<bool> {
    let a = ac.a; let (b0, bn) = lb0(a)?; let mut any = false;
    list_iter(b0, bn, 0x18, |d, v| { if eff80_cc(d, v)? { any = true; } Some(()) })?; if any { return Some(true); }
    let stride = match ac.k { 0 | 8 => 0x10, 2 => 0x18, _ => return Some(false) };
    let (p, n) = l50(a)?; list_iter(p, n, stride, |d, v| { if eff80_cc(d, v)? { any = true; } Some(()) })?; Some(any)
}

// ── 단계 트레이스(진단: DIFF 로그 재계산 시에만 켠다) ────────────────────────────────────────
thread_local! { static TRACE: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) }; static LAST_ARGS: std::cell::Cell<(usize, usize, usize)> = const { std::cell::Cell::new((0, 0, 0)) }; }
fn trs(f: impl FnOnce() -> String) { TRACE.with(|t| { if let Some(s) = t.borrow_mut().as_mut() { s.push_str(&f()); s.push(' '); } }); }
// ── 본체 S6~S21 ───────────────────────────────────────────────────────────────────────────
unsafe fn body(st: &St) -> Option<Out> {
    trs(|| format!("S0[b0={} vis_me={} E={} A={} cfgtick={} class={} ss={:?}]", st.b0, st.vis_me, st.e_list.len(), st.a_list.len(), st.fc.cfg_tick, st.fc.class, st.fc.ss));
    let (x, g, w, tgt, side, eside, qx, qy, tick, scale) = (st.x, st.g, st.w, st.tgt, st.side, st.eside, st.qx, st.qy, st.tick, st.scale);
    let cfg = rd_u64(g + G_CFG)? as usize; if !ptr_ok(cfg) { return None; }
    let tps = rd_u64(cfg + CFG_TPS)?;
    let th = rd_u64(tgt + ENT_HANDLE)?; let (tx, ty) = xy(tgt)?; let (t0, t8) = (rd_u64(tgt)?, rd_u64(tgt + 8)?);
    let cap = |v: u64| cap150(v, scale);
    let mut a_acc: i64 = st.b0; let mut b_acc: i64 = st.b0; let mut c_acc: i64 = 0; let mut bstruct: i64 = st.b0;
    // S6 오브젝트
    {
        let n = rd_u64(x + X_OBJ_LEN)?; let p = rd_u64(x + X_OBJ_PTR)? as usize; if n != 0 && !ptr_ok(p) { return None; }
        for i in 0..n.min(CAP_ITER) as usize {
            let o = rd_u64(p + i * 8)? as usize; if o == 0 { return None; }
            let (ox, oy) = xy(o)?; let d2s = sat_d2(ox, oy, qx, qy); if imp(0, (d2s >> 8) >= imm32(SITE_PE_FILTER_D2_SHR8_IMM, 87_890_625), (d2s >> 8) >= 87_890_625) { continue; }
            let kind = rd_u64(o + ENT_KIND)?;
            if kind == 4 {
                if rd_u8(o + 0x88) != 0 && rd_u64(o + 0x90)? == th {
                    let v = cap(match est(o, tgt) { Some(v) => v, None => { trs(|| format!("NA:S6est{:#x}", o)); return None } });
                    if wrap_d2(ox, oy, qx, qy) <= sq(range_g(o, tgt)?) { a_acc = a_acc.wrapping_add(v as i64); }
                    else { let r = range_u(o, tgt)?.wrapping_add(32000); if d2s <= sq(r) { a_acc = a_acc.wrapping_add((v >> 1) as i64); } }
                }
            } else if kind as i32 == 5 && rd_u64(o + 0x88)? != 0 {
                let v = cap(match est(o, tgt) { Some(v) => v, None => { trs(|| format!("NA:S6est{:#x}", o)); return None } });
                if wrap_d2(ox, oy, qx, qy) <= sq(range_g(o, tgt)?) { a_acc = a_acc.wrapping_add((if rd_u64(o + 0x90)? == th { v } else { third(v) }) as i64); }
            }
        }
    }
    trs(|| format!("S6[a={}]", a_acc));
    // S7 구조물/미니언 체인 (tick ≤ cfgTick)
    if tick <= st.fc.cfg_tick {
        if rd_u8(tgt) != 0 { return None; }
        let mut items: Vec<(usize, bool)> = Vec::with_capacity(64);   // (ent, enemy)
        for (s, en) in [(eside, true), (side, false)] {
            for off in [0x180usize, 0x1a0, 0x1c0, 0x190, 0x1b0, 0x1d0] { let e = rd_u64(x + off + (s as usize) * 8)? as usize; if e != 0 { items.push((e, en)); } }
            let n = rd_u64(x + X_MINION_LEN + (s as usize) * 0x20)?; let p = rd_u64(x + X_MINION_PTR + (s as usize) * 0x20)? as usize; if n != 0 && !ptr_ok(p) { return None; }
            for i in 0..n.min(CAP_ITER) as usize { let e = rd_u64(p + i * 8)? as usize; if e == 0 { return None; } items.push((e, en)); }
        }
        trs(|| { let mut o = format!("GDIVE{:?} STRUCT[", dive_all_since()); for (it, en) in &items { if let (Some(k), Some((ix, iy))) = (rd_i32(*it + ENT_KIND), xy(*it)) { if k == 2 {
            let mut mind = u64::MAX; for h in &st.e_list { if let Some((hx, hy)) = xy(h.ent) { mind = mind.min(wrap_d2(ix, iy, hx, hy)); } }
            o += &format!("{}{:#x}:d2={} eh={} ", if *en { "E" } else { "A" }, rd_u64(*it + ENT_HANDLE).unwrap_or(0), sat_d2(ix, iy, qx, qy), mind); } } } o + "]" });
        for (item, enemy) in items {
            let (ix, iy) = xy(item)?;
            let mut go = { let z = sat_d2(ix, iy, qx, qy) >> 8; imp(1, z < imm32(SITE_PE_FILTER_D2_SHR8_IMM, 87_890_625), z < 87_890_625) };
            // ★양쪽 체인 모두 E(적 영웅) 리스트를 쓴다 — 게임 d86f2b(적 체인)·d8702d(내 체인) 둘 다 [rbp+0x5d0]/[0x5e8]=E (2026-09-06 23:00 정정, RE 의 "내 아이템이면 M" 은 오류)
            if !go { for h in &st.e_list { let (hx, hy) = xy(h.ent)?; if { let z = wrap_d2(ix, iy, hx, hy) >> 8; imp(2, z < imm32(SITE_PE_NEAR_D2_SHR8_IMM, 19_140_625), z < 19_140_625) } { go = true; break; } } }
            let _ = enemy;
            if !go { continue; }
            let (i0, i8) = (rd_u64(item)?, rd_u64(item + 8)?);
            let same = same_team_ab(i0, i8, t0, t8);
            let cnt = if same { 0 } else { match count_in_range(x, t8, item) { Some(v) => v, None => { trs(|| "NA:cnt".into()); return None } } };
            if rd_i32(item + 0x4c0)? == -1 { return None; }
            // ★게임의 적/아군 분기는 **원시 쌍 비교**다: 적 ⟺ `(other[0] != self[0]) || (other[8] != self[8])`
            //   (RE 0xd874d2/0xd874e0). ~~`i0 == 0 && i8 == t8`~~ 은 self 의 소유 태그가 0 일 때만 같다.
            if i0 == t0 && i8 == t8 {
                if rd_i32(item + ENT_KIND)? == 2 {
                    let mut first: Option<usize> = None;
                    for e in &st.e_list { let (ex, ey) = xy(e.ent)?; if wrap_d2(ix, iy, ex, ey) <= sq(range_g(item, e.ent)?) { first = Some(e.ent); break; } }
                    let e = match first { Some(e) => e, None => continue };
                    let v = match est(item, e).and_then(|es| tower_v(item, es, tps, scale)) { Some(v) => v, None => { trs(|| "NA:mytower".into()); return None } };
                    if wrap_d2(ix, iy, qx, qy) <= sq(range_g(item, tgt)?) { c_acc = c_acc.wrapping_add(third(v) as i64); }
                }
            } else if rd_i32(item + ENT_KIND)? == 2 {
                let mut v = match est(item, tgt).and_then(|es| tower_v(item, es, tps, scale)) { Some(v) => v, None => { trs(|| "NA:etower".into()); return None } };
                let r_u = range_u(item, tgt)?;
                let d2g = wrap_d2(ix, iy, qx, qy); let rgb = range_g(item, tgt)?; let ri = radius(item)?; let rt = radius(tgt)?;
                // ★★게이트 = `range_g + 24000` — **확증(2026-09-07 11:40, 깨끗한 단측 오라클)**.
                //   오래 미해결이던 "디스어셈은 +18000 인데 실측은 +24000" 모순을 이번에 매듭지었다.
                //
                //   ① 옛 오라클이 틀렸다: `dive_calls_since(side, handle)` 는 **틱 구간 내 (item) 단위**인데
                //      게이트는 **(item, qx, qy) 단위**다. position_eval 은 틱마다 여러 쿼리점으로 불리므로
                //      임계를 키울수록 "언젠가 불렸다"에 더 맞는다 — 90.3%→99.72%→99.86% 의 단조 개선이 그 인공물.
                //      (그 전 단계 오염 = `0xd96d00` xref 5곳 → 반환주소 필터 `ra == base+0xd88048` 로 이미 해결.)
                //   ② 새 오라클 = **단측**: 게임이 `0xd88043` 에서 실제로 통과시킨 바로 그 `(item, q)` 에 대해
                //      내 모델이 통과하는가만 본다(캡처된 호출은 전부 pass 표본이므로 위양성이 원리적으로 없다).
                //   ③ 실측(리플레이 1판, 표본 3,445,518):
                //        rg+18000 79.53% · rg+22000 99.71% · **rg+24000 100.0000%** · rg+18000+ri 100%
                //      그리고 **필요 임계 역산 `max(need − rg) = 23999`** = 정확히 24000(23999 는 isqrt 절삭).
                //      다른 형태는 전부 부자연스러운 상수로 떨어진다(need−rg−ri=13999 · −ri−rt=3999 · −ri/2=18999).
                //   ⟹ **`rg + 24000` 이 가장 타이트한 정답.** 디스어셈의 `add rax, 0x4650`(18000) 은 여러 가산 중
                //      하나이고, RE 의 7항 읽기가 **6000 짜리 항 하나를 빠뜨린 것**으로 보인다(그 항의 정체는 미규명).
                let game_pass = d2g <= sq(rgb.wrapping_add(imm32(SITE_PE_TOWER_MARGIN_A_IMM, 18_000)));
                { let gp = dive_calls_since(side, rd_u64(item + ENT_HANDLE)?) > 0;
                  gate_edge(rgb, isqrt_fast(d2g), gp);
                  if gp != game_pass { gate_log(d2g, rgb, ri, rt, rd_u64(item + ENT_F438)?, rd_u64(item + 0x4a0)?, rd_u64(item + 0x4a8)?, rd_u64(item + ENT_LEVEL)?, rd_i32(item + 0x4c0)? as i64, rd_i32(item + 0x470)? as i64, rd_u64(item + 0x680)?, rd_i32(tgt + 0x470)? as i64, rd_u64(tgt + 0x680)?, gp); } }
                // 진단: 게임의 실제 게이트(= 그 타워로 d96d00 호출) 대비 후보별 적중 집계
                { let gp = dive_calls_since(side, rd_u64(item + ENT_HANDLE)?) > 0;
                  let cands = [rgb + 18000, rgb + 19000, rgb + 20000, rgb + 22000, rgb + 24000, rgb + 18000 + ri, rgb + 18000 + rt, rgb];
                  for (i, c) in cands.iter().enumerate() { let ok = (d2g <= sq(*c)) == gp; gate_stat(i, ok); }
                  // ★결정적 통계: `gcalls == 0` 은 **깨끗한 음성**(그 타워로 한 번도 안 불렀다 = 게이트 탈락).
                  //   그 표본들의 `need − rg` **최솟값**이, 게임 통과 표본의 최댓값(23999)보다 작거나 같으면
                  //   **상수 임계로는 원리적으로 불가능**하다는 뜻이다 = 거리/기준점 자체가 다르다.
                  if !gp {
                      let need = isqrt_fast(d2g);
                      neg_min(0, need.saturating_sub(rgb));
                      let (sx, sy) = xy(tgt)?; let dself = isqrt_fast(wrap_d2(ix, iy, sx, sy));
                      neg_min(1, dself.saturating_sub(rgb));
                      neg_cnt();
                  } }
                trs(|| { let d2g = wrap_d2(ix, iy, qx, qy); let rg = range_g(item, tgt).unwrap_or(0); let ri = radius(item).unwrap_or(0); let rt = radius(tgt).unwrap_or(0);
                    let cands = [rg + 18000, rg + 18000 + ri, rg + 18000 + rt, rg + 32000, rg + 50000, rg.wrapping_sub(ri) + 18000];
                    format!("G[d2={} isq={} rg={} ri={} rt={} pass={:?}]", d2g, isqrt_fast(d2g), rg, ri, rt, cands.iter().map(|c| d2g <= sq(*c)).collect::<Vec<_>>()) });
                trs(|| format!("T[{:#x} @({},{}) q=({},{}) tgt@({:?}) v={} d2={} rg={} dive={:?} gcalls={} gq={:?} gf={:?} 438={} 4a0={} 4a8={} lv={} e8={:?} e8impl={:#x} 4c0={} 470={} 680={} tgt470={} tgt680={} tgt={:#x}]", item, ix, iy, qx, qy, xy(tgt), v, wrap_d2(ix, iy, qx, qy), range_g(item, tgt).unwrap_or(0).wrapping_add(18000), dive_lookup(side, rd_u64(item + ENT_HANDLE).unwrap_or(0), tick), dive_calls_since(side, rd_u64(item + ENT_HANDLE).unwrap_or(0)), dive_q_recent(side, rd_u64(item + ENT_HANDLE).unwrap_or(0)), dive_f_recent(side, rd_u64(item + ENT_HANDLE).unwrap_or(0)),
                    rd_u64(item + 0x438).unwrap_or(0), rd_u64(item + 0x4a0).unwrap_or(0), rd_u64(item + 0x4a8).unwrap_or(0), rd_u64(item + ENT_LEVEL).unwrap_or(0), vt_e8(item + SLOT0, item, tgt), dy::impl_rva(rd_u64(item + SLOT0 + 8).unwrap_or(0) as usize, 0xe8).unwrap_or(0), rd_i32(item + 0x4c0).unwrap_or(-9), rd_i32(item + 0x470).unwrap_or(-9), rd_u64(item + 0x680).unwrap_or(0), rd_i32(tgt + 0x470).unwrap_or(-9), rd_u64(tgt + 0x680).unwrap_or(0), tgt));
                if game_pass {
                    let (dive, tgt_id) = match dive_lookup(side, rd_u64(item + ENT_HANDLE)?, tick) { Some(v) => v, None => { trs(|| "NA:dive".into()); return None } };
                    let hit_me = tgt_id == th && (dive & 1) == 1;
                    let atk = rd_u64(item + 0x88)?;
                    if (atk & 1) == 1 && rd_u64(item + 0x98)? == th {
                        if hit_me { v = third(v); }
                        a_acc = a_acc.wrapping_add(v as i64); bstruct = bstruct.wrapping_add(v as i64);
                    } else {
                        let d2s = sat_d2(ix, iy, qx, qy); let half = r_u >> 1; let wgt: u32 = if d2s < sq(half) { 100 } else { 50 };
                        if hit_me { v = third(v); }
                        else {
                            let mut branch_count = true;
                            if dive == 1 && (atk & 1) == 1 && rd_u64(item + 0x98)? != th {
                                if let Some(tg) = w.entity(rd_u64(item + 0x98)?) {
                                    let es2 = estimate_damage(item + SLOT0, item, tg.0, false)?;
                                    if rd_u64(tg.0 + ENT_HP)? > es2 { v = ((3 * wgt * 0x290) >> 16) as u64; branch_count = false; }
                                    else if !st.e_list.is_empty() { v >>= 1; branch_count = false; }
                                }
                            }
                            if branch_count && !st.e_list.is_empty() { if atk == 1 { v >>= 1; } branch_count = false; }
                            if branch_count {
                                if cnt == 2 { let t1 = (((2 * v as u32) as u16 as u32) * 0x5556) >> 16; v = (((wgt * t1) as u16 as u32) / 100) as u64; }
                                else if cnt >= 3 { v = ((3 * wgt * 0x290) >> 16) as u64; }
                            }
                        }
                        b_acc = b_acc.wrapping_add(v as i64); a_acc = a_acc.wrapping_add(v as i64); bstruct = bstruct.wrapping_add(v as i64);
                    }
                }
            }
        }
    }
    trs(|| format!("S7[a={} b={} c={} bs={}]", a_acc, b_acc, c_acc, bstruct));
    // S8 게이트
    let phase = rd_u8(g + G_PHASE); let thr = rd_u64(cfg + CFG_8A8)?.saturating_sub(30u64.wrapping_mul(tps));
    let phase_late = phase <= 8 && (0x1a1u32 >> phase) & 1 == 1 && tick >= thr;
    let mut count_mode = false;
    if !phase_late && st.purpose <= 10 && imp(6, (imm32(SITE_PE_KIND_MASK_IMM, 0x503) as u32 >> st.purpose) & 1 == 1, (0x503u32 >> st.purpose) & 1 == 1) {
        if t0 & 1 == 1 { count_mode = true; }
        else { match w.mode() { Some((0, md)) => { if rd_u64(md + 0x240 + (eside as usize) * 8)? == 0 { count_mode = true; } } Some(_) => count_mode = true, None => return None } }
    }
    if count_mode {
        let (mut ca, mut cb): (Option<u64>, Option<u64>) = (None, None);
        units(x, eside, |u| {
            let (ux, uy) = xy(u)?; let d2s = sat_d2(ux, uy, qx, qy); if imp(3, (d2s >> 8) >= imm32(SITE_PE_FILTER_D2_SHR8_IMM, 87_890_625), (d2s >> 8) >= 87_890_625) { return Some(()); }
            let (atk, to, use_b) = if rd_i32(u + ENT_KIND)? == 1 { let atk = rd_u8(u + 0x88); (atk, atk == 1 && rd_u64(u + 0x90)? != th, rd_u8(u + 0x118) != 0) } else { (1u8, false, false) };
            let v0 = if use_b { match cb { Some(v) => v, None => { let v = cap(est(u, tgt)?); cb = Some(v); v } } } else { match ca { Some(v) => v, None => { let v = cap(est(u, tgt)?); ca = Some(v); v } } };
            let mut v = v0 as i64;
            if atk != 1 { v = sdiv2(v); } else if to { v = sdiv3(v); }
            if d2s >= 0xf4240001 { v = 0; }
            a_acc = a_acc.wrapping_add(v); Some(())
        })?;
    } else {
        // S9 노출
        let r = match exposure(x, tgt, qx, qy, tps >> 1, tps) { Some(v) => v, None => { trs(|| "NA:expo".into()); return None } };
        let mut add: i64 = 0;
        if r != 0 {
            let hp = st.hp; let hpd = hp.wrapping_add((hp == 0) as u64); let r100 = r.wrapping_mul(100);
            if r100 as i64 == i64::MIN && hpd as i64 == -1 { return None; }
            let ratio = (r100 as i64).wrapping_div(hpd as i64);
            add = ratio.min(140);
            if ratio == 0 { add = 0; }
            else if !phase_late && (st.purpose < 2 || (st.purpose & 0xe) == 8) {
                let hpp = hp.wrapping_mul(100) / rd_u64(tgt + ENT_MAXHP)?.max(1); let rat = r100 / hpd;
                if r < hp && rat < 50 && (hpp > 65 || rat < 30) && (hpp > 40 || rat < 18) && (hpp > 25 || rat < 10) { add = ((if add < 0 { add + 3 } else { add }) >> 2).min(18); }
            }
        }
        a_acc = a_acc.wrapping_add(add);
    }
    trs(|| format!("S9[a={} count={}]", a_acc, count_mode));
    // S10 X+0xf0 리스트(적 사이드)
    {
        let n = rd_u64(x + X_SIDE_UNITS_LEN + (eside as usize) * 0x20)?; let p = rd_u64(x + X_SIDE_UNITS_PTR + (eside as usize) * 0x20)? as usize; if n != 0 && !ptr_ok(p) { return None; }
        for i in 0..n.min(CAP_ITER) as usize {
            let u = rd_u64(p + i * 8)? as usize; if u == 0 { return None; }
            let (ux, uy) = xy(u)?; let d2s = sat_d2(ux, uy, qx, qy); if imp(4, (d2s >> 8) >= imm32(SITE_PE_FILTER_D2_SHR8_IMM, 87_890_625), (d2s >> 8) >= 87_890_625) || rd_i32(u + 0x4c0)? == -1 { continue; }   // ★off-by-one 정정: 게임은 (d2>>8) < 0x53d1ac1
            let v = cap(match est(u, tgt) { Some(v) => v, None => { trs(|| format!("NA:S10est{:#x}", u)); return None } }); let r = range_u(u, tgt)?; let k = rd_u64(u + ENT_KIND)?;
            let tm = if k == 7 || k as i32 == 9 { rd_i32(u + 0x88)? == 1 && rd_u64(u + 0x90)? == th } else if k as i32 == 10 { rd_i32(u + 0x70)? == 1 && rd_u64(u + 0x78)? == th } else { false };
            if d2s <= sq(r) { a_acc = a_acc.wrapping_add((if tm { v } else { v >> 1 }) as i64); }
            else if d2s <= sq(r.wrapping_add(32000)) { a_acc = a_acc.wrapping_add(third(v) as i64); }
        }
    }
    trs(|| format!("S10[a={}]", a_acc));
    // S11 활성 액션 장판
    let (mut f30, mut f31, mut cc) = (0u8, 0u8, false);
    {
        let n = rd_u64(w.data + X_ACTIONS_LEN)?; let p = rd_u64(w.data + X_ACTIONS_PTR)? as usize; if n != 0 && !ptr_ok(p) { return None; }
        let r_t = radius(tgt)?;
        for i in 0..n.min(CAP_ITER) as usize {
            let a = p + i * ACTION_STRIDE;
            let (ax, ay) = (rd_u64(a + 0x100)?, rd_u64(a + 0x108)?); let d2q = wrap_d2(ax, ay, qx, qy);
            if imp(5, (d2q >> 8) > imm32(SITE_PE_FIELD_D2_SHR8_IMM, 244_140_624), (d2q >> 8) > 244_140_624) { continue; }
            if rd_u8(a + 0x131) == 0 {
                let ok = same_team_ae(a, tgt)? && rd_u64(a + 0xa0)? == 10 && { let np = rd_u64(a + 0x98)? as usize; ptr_ok(np) && (0..10).all(|j| rd_u8(np + j) == b"knight_ult"[j]) };
                if !ok { continue; }
            }
            let a40 = rd_u64(a + 0x40)?; let k = if a40 >= 2 { a40 - 2 } else { 7 };
            if k == 4 || k == 5 || (k == 7 && a40 == 1) { continue; }
            let src = match w.entity(rd_u64(a + 0xf8)?) { Some(e) => e.0, None => continue };
            let tt = rd_u32(a + 0x12c);
            if !(match relevant(tt, a, tgt) { Some(v) => v, None => { trs(|| format!("NA:rel{}", tt)); return None } }) { continue; }
            let ac = Act { a, k };
            if !(match cover(&ac, qx, qy, r_t.wrapping_add(imm32(SITE_PE_TOWER_MARGIN_B_IMM, 18_000))) { Some(v) => v, None => { trs(|| format!("NA:cover{}", k)); return None } }) { continue; }
            if !same_team_ae(a, tgt)? {
                let raw = match threat(&ac, src, tgt) { Some(v) => v, None => { trs(|| format!("NA:threat{}", k)); return None } }; let v = cap(raw);
                trs(|| { let (b0, bn) = lb0(a).unwrap_or((0, 0)); let (p, n) = l50(a).unwrap_or((0, 0)); format!("ACT[{:#x} k={} tt={} raw={} v={} ticks={:?} a60={} a68={} b0={:?} l50={:?} hit={:?} ty={} src={:#x}]", a, k, tt, raw, v, ticks(a), rd_u64(a + 0x60).unwrap_or(0), rd_u64(a + 0x68).unwrap_or(0), dmg_list(b0, bn, 0x18, a, src, tgt), dmg_list(p, n, if k == 2 { 0x18 } else { 0x10 }, a, src, tgt), hit(a, tgt), rd_u32(a + 0x128), src) });
                let mut near: Option<u64> = None;
                for f in &st.a_list { let fe = f.ent; if !relevant(tt, a, fe)? { continue; } let (fx, fy) = xy(fe)?; if !cover(&ac, fx, fy, radius(fe)?)? { continue; } let d2f = wrap_d2(fx, fy, ax, ay); near = Some(match near { None => d2f, Some(m) => m.min(d2f) }); }
                if k == 0 { if let Some(nr) = near { if rd_u8(a + 0x78) == 0 && nr < d2q { continue; } } f30 = 1; }
                else if k == 2 { f31 = 1; } else { f30 = 1; }
                a_acc = a_acc.wrapping_add(v as i64);
                if !cc { cc = match is_cc(&ac) { Some(v) => v, None => { trs(|| format!("NA:cc{}", k)); return None } }; }
            } else {
                let h = match heal_shield(&ac, src, tgt, 0x40).and_then(|a| heal_shield(&ac, src, tgt, 0x48).map(|b| a.wrapping_add(b))) { Some(v) => v, None => { trs(|| format!("NA:heal{}", k)); return None } };
                if h != 0 { let q = (h.wrapping_mul(100) as i64).wrapping_div(st.hp1 as i64); a_acc = a_acc.wrapping_sub(q.min(150)); }
            }
        }
    }
    trs(|| format!("S11[a={} f30={} f31={} cc={}]", a_acc, f30, f31, cc));
    // S12 내 분수(= S1 과 같은 영역식, 디스어셈 d89632~d8973e 확인)
    if st.b0 == 9999 { a_acc = a_acc.wrapping_add(cap(rd_u64(cfg + CFG_1490)?) as i64); }
    // S13 maxC
    let r_me = radius(tgt)?;
    let mut maxc: i64 = 0;
    if !status_has(tgt, 3)? {
        for rec in &st.e_list { let v = rec.rec.w[4] as i64; if rec.d2 <= rec.rec.w[0x19] { maxc = maxc.max(v); } else if rec.d2 <= rec.rec.w[0x1a] { maxc = maxc.max(sdiv2(v)); } }
    }
    for j in 1..=3usize {
        if !(match usable(tgt, j) { Some(v) => v, None => { trs(|| "NA:usable".into()); return None } }) { continue; }
        for rec in st.e_list.iter().chain(st.a_list.iter()) {
            let wv = &rec.rec.w; let jj = j - 1;
            if let Some(t) = tier(wv[4 + j] as i64, rec.d2, st.fc.ss[jj], wv[0x1b + 3 * jj], wv[0x1c + 3 * jj], wv[0x1d + 3 * jj], wv[0xb + jj], radius(rec.ent)?, r_me) { maxc = maxc.max(t); }
        }
    }
    c_acc = c_acc.wrapping_add(maxc);
    trs(|| format!("S13[maxc={}]", maxc));
    // S14 적 챔프 P3
    let (mut sum_e, mut half28): (i64, i64) = (0, 0);
    let vis = w.visible(eside, th)?;
    for rec in &st.e_list {
        let d2 = rec.d2; let e = rec.ent; let wv = &rec.rec.w;
        if (d2 >> 10) >= 0x9502f9 && !vis { continue; }
        let v0 = wv[0] as i64;
        let m88 = |k: usize| -> Option<bool> { match memo88(e, k) { Some(v) => Some(v == 1), None => { trs(|| format!("NA:m88_{}", k)); None } } };
        let (mut can_hit, base) = if d2 <= wv[0xe] { (m88(0)?, v0.max(0)) } else if d2 <= wv[0xf] { (m88(0)?, sdiv2(v0).max(0)) } else { (false, 0) };
        let mut s = [0u64; 4];
        s[0] = if status_has(e, 3)? { (base as u64) / 3 } else { base as u64 };
        for k in 1..=3usize {
            if !usable(e, k)? { continue; }
            let kk = k - 1; let v = wv[k] as i64; let flag = rec.rec.ss[kk] != 0;
            let (ra, rb, rc, rng) = (wv[0x10 + 3 * kk], wv[0x11 + 3 * kk], wv[0x12 + 3 * kk], wv[8 + kk]);
            if flag {
                if d2 <= rb && !can_hit { can_hit = m88(k)?; }
                s[k] = tier(v, d2, true, ra, rb, rc, rng, radius(e)?, r_me).map(|t| t.max(0)).unwrap_or(0) as u64;
            } else if d2 <= ra { s[k] = v.max(0) as u64; if !can_hit { can_hit = m88(k)?; } }
            else if d2 <= rb { s[k] = sdiv2(v).max(0) as u64; if !can_hit { can_hit = m88(k)?; } }
        }
        if status_has(e, 4)? { for k in 1..4 { s[k] /= 3; } }
        if status_has(e, 5)? { for k in 1..4 { if let Some(sl) = slot_of(e, k)? { if rd_i32(sl + 0x30)? != -1 && (match vt120_of(sl) { Some(v) => v, None => { trs(|| "NA:vt120".into()); return None } }) { s[k] /= 3; } } } }
        if status_any_not_2345(e)? { for k in 0..4 { s[k] >>= 1; } }
        // 캐스팅 세그먼트
        let mut castv = 0u64;
        let e308 = rd_u64(e + 0x308)?; let tag = if (e308 as i64) < 0 { e308 ^ 0x8000_0000_0000_0000 } else { 4 };
        if tag == 3 || tag == 4 {
            let (lp, ln, kind, sx, sy, width, tf) = if tag == 3 { (rd_u64(e + 0x318)? as usize, rd_u64(e + 0x320)?, rd_u32(e + 0x35c), rd_u64(e + 0x330)?, rd_u64(e + 0x338)?, rd_u64(e + 0x340)?, rd_u64(e + 0x348)?) }
                                                   else { (rd_u64(e + 0x310)? as usize, rd_u64(e + 0x318)?, rd_u32(e + 0x36c), rd_u64(e + 0x340)?, rd_u64(e + 0x348)?, rd_u64(e + 0x350)?, rd_u64(e + 0x358)?) };
            let (ex, ey) = xy(e)?;
            if kind_pred(kind, e, tgt)? && tick >= tf && seg_dist(qx, qy, ex, ey, sx, sy)? <= width.wrapping_add(r_me).wrapping_add(20000) {
                let (mut ph, mut mg) = (0u64, 0u64);
                if list_iter(lp, ln, 0x18, |d, v| { let (p, m) = dy::eff28_damage(d, v, e)?; ph = ph.wrapping_add(p); mg = mg.wrapping_add(m); Some(()) }).is_none() { trs(|| "NA:cast28".into()); return None; }
                let dmg = match conv(e, tgt, ph, 1, 0).and_then(|a| conv(e, tgt, mg, 1, 1).map(|b| a.wrapping_add(b))) { Some(v) => v, None => { trs(|| "NA:castconv".into()); return None } };
                castv = cap(dmg); f30 = 1;
                if !can_hit { let mut any = false; list_iter(lp, ln, 0x18, |d, v| { if vt88_flag(d, v, 0)? == 1 { any = true; } Some(()) })?; can_hit = any; }
            }
        }
        let mut t: i64 = if can_hit || cc { (s[0].wrapping_add(s[1]).wrapping_add(s[2]).wrapping_add(s[3]).wrapping_add(castv)) as i64 }
                         else { (s[0] as i64).max(s[1] as i64).max(s[2] as i64).max(s[3] as i64).wrapping_add(castv as i64) };
        if (st.fc.class & 6) == 2 && t > 0 && !st.a_list.is_empty() {
            let (ex, ey) = xy(e)?;
            for ar in &st.a_list { let (ax, ay) = xy(ar.ent)?; if wrap_d2(ax, ay, ex, ey) < d2 { if seg_dist(ax, ay, qx, qy, ex, ey)? <= radius(ar.ent)?.wrapping_add(28000) { t = ((t as u64) >> 1) as i64; break; } } }
        }
        trs(|| { let ra8 = (rd_u32(st.sim + P5_ROLE) as usize) * 8; let sb = rd_u64(rec.sim + P5_SIDE).unwrap_or(9); let rb = rd_u32(rec.sim + P5_ROLE) as usize;
            let tb = st.x + 0x280 + (sb as usize) * 0xfa0 + rb * 0x320;
            let raw: Vec<u64> = (0..4).map(|k| rd_u64(tb + 0x28 * k + ra8).unwrap_or(0)).collect();
            let raw2: Vec<u64> = (0..4).map(|k| rd_u64(tb + 0x190 + 0x28 * k + ra8).unwrap_or(0)).collect();
            format!("P3[e={:#x} d2={} vis={} s={:?} cast={} can={} cc={} T={} rec03={:?} hp={} R={:?} Rh={:?} usable={:?} tri={:?} 0xe={} 0xf={}]", e, d2, vis, s, castv, can_hit, cc, t,
                &wv[0..4], st.hp, raw, raw2, (1..=3).map(|k| usable(e, k)).collect::<Vec<_>>(), (0..3).map(|j| (wv[0x10 + 3 * j], wv[0x11 + 3 * j], wv[0x12 + 3 * j])).collect::<Vec<_>>(), wv[0xe], wv[0xf]) });
        if st.vis_me == 0 { let h = sdiv2(t); half28 = half28.wrapping_add(t.wrapping_sub(h)); t = h; }
        a_acc = a_acc.wrapping_add(t); sum_e = sum_e.wrapping_add(t);
    }
    trs(|| format!("S14[a={} sumE={} half28={}]", a_acc, sum_e, half28));
    // S15
    if sum_e > 0 {
        // ★★L445·L998·L1022 의 120000² 는 `pe_count_radius` 사이트(0xd8b66b/0xd8beea)가 **아니다**.
    //   RE 2026-09-07 은 그렇게 대응시켰지만 실측이 반증했다 — live(150000²) 로 바꾸면 판정이 2.05%(488만 건)
    //   뒤집히면서 DIFF 가 2.1181% → 2.1871% 로 **악화**한다. 정적 원본으로 되돌리면 1.6534%.
    //   ⟹ 이 세 줄이 모델하는 게임 코드는 패치 대상이 아닌 다른 120000² 이다(사이트 미특정).
    let cd2 = 0x3_5a4e_9001u64;
        let na = st.a_list.iter().filter(|r| r.d2 < cd2).count(); let ne = st.e_list.iter().filter(|r| r.d2 < cd2).count();
        trs(|| format!("S15[na={} ne={} ad2={:?} ed2={:?}]", na, ne, st.a_list.iter().map(|r| r.d2).collect::<Vec<_>>(), st.e_list.iter().map(|r| r.d2).collect::<Vec<_>>()));
        if na == 0 && ne >= 2 { let n = ne.min(4) as i64; a_acc = a_acc.wrapping_add(sum_e.wrapping_mul(n).wrapping_mul(25) / 100); }
    }
    // S16 아군 스킬 지원
    for rec in &st.a_list {
        let ae = rec.ent; let wv = &rec.rec.w; let mut val: i64 = 0; let r_b = radius(ae)?;
        for k in 1..=3usize {
            let kk = k - 1;
            if slot_id(slot_of(ae, k)?)? == -1 { continue; }
            let t = tier(wv[k] as i64, rec.d2, rec.rec.ss[kk] != 0, wv[0x10 + 3 * kk], wv[0x11 + 3 * kk], wv[0x12 + 3 * kk], wv[8 + kk], r_b, r_me).unwrap_or(0);
            val = val.max(t);
        }
        c_acc = c_acc.wrapping_add(val);
    }
    // S17 선분 몸빵
    if st.fc.class <= 1 {
        let mut acc: u64 = 0;
        for rec in &st.a_list {
            let (cd0, cv) = (rd_u64(rec.sim + P5_CHAMP_DATA)? as usize, rd_u64(rec.sim + P5_CHAMP_VT)? as usize); let cd = dy::arc_payload(cd0, cv)?;
            let kind = champ_kind(cd, cv)?; let (tp, tl) = champ_tags(cd, cv)?;
            if tags_has(tp, tl, 8)? { continue; }
            match kind { 4 | 3 => continue, 0 => { if champ_vt30_w2(cd, cv)? >= 0x4b0 { continue; } } _ => {} }
            let ae = rec.ent; let (ax, ay) = xy(ae)?;
            if sat_d2(ax, ay, qx, qy) > 0x3_5a4e_9000 { continue; }   // A/B: 정적 원본
            for er in &st.e_list {
                let (ex, ey) = xy(er.ent)?; let d2ae = wrap_d2(ax, ay, ex, ey);
                if er.d2 >= d2ae { continue; }
                if seg_dist(qx, qy, ax, ay, ex, ey)? > r_me.wrapping_add(28000) { continue; }
                if rd_i32(ae + 0x4c0)? != -1 { let dmg = estimate_damage(ae + SLOT0, ae, er.ent, false)?; acc = acc.wrapping_add(dmg.wrapping_mul(100) / rd_u64(er.ent + ENT_MAXHP)?.max(1)); }
                break;
            }
        }
        c_acc = c_acc.wrapping_add(acc as i64);
    }
    trs(|| format!("S17[a={} b={} c={}]", a_acc, b_acc, c_acc));
    // S18~S19
    c_acc = c_acc.wrapping_add((maxc.wrapping_sub(a_acc.wrapping_sub(bstruct).max(0))).max(0));
    let pc = if st.purpose >= 2 { st.purpose - 2 } else { 7 };
    let mut a_out = a_acc;
    if pc == 8 { a_out = a_acc.wrapping_mul(3) / 2; b_acc = b_acc.wrapping_mul(3) / 2; }
    else if pc == 7 { if st.purpose & 1 == 1 { a_out = a_acc.wrapping_mul(120) / 100; } else { c_acc = c_acc.wrapping_mul(120) / 100; } }
    // S21 노이즈
    let lv = rd_u64(st.sim + 0x1f8)?; let v = lv.min(100);
    let cond = if st.mode < 2 { lv < 100 } else { (pc.wrapping_add(0xfd)) < 0xfe && lv < 100 };
    if cond {
        let diff = a_out.wrapping_sub(b_acc);
        if diff != 0 {
            let tag = w.mode()?.0;
            let amp0: u32 = ((((v as u32) * 99) as u16 as u32 * 0x199a) >> 16) + 10;
            let amp: u64 = if tag == 2 { (1000 - amp0) as u64 } else { (((2000u32.wrapping_sub(2 * amp0)) as u16 as u32 * 0x5556) >> 16) as u64 };
            let mut t = tick; if st.mode >= 2 { t /= tps.wrapping_mul(6).max(1); }
            const K: u64 = 0x9E37_79B9_7F4A_7C15;
            let h = ((t ^ rd_u64(st.sim + P5_MEMO_KEY)?).wrapping_mul(K) ^ st.gx).wrapping_mul(K) ^ st.gy; let h = h.wrapping_mul(K);
            let n = ((h ^ (h >> 31)) % (2 * amp + 1)) as i64 - amp as i64;
            let before = a_out;
            a_out = b_acc.wrapping_add(n.wrapping_add(1000).wrapping_mul(diff) / 1000);
            trs(|| format!("S21[v={} amp0={} amp={} tag={} t={} tick={} tps={} gx={} gy={} key={:#x} h={:#x} n={} diff={} before={} after={}]", v, amp0, amp, tag, t, tick, tps, st.gx, st.gy, rd_u64(st.sim + P5_MEMO_KEY).unwrap_or(0), h, n, diff, before, a_out));
        }
    }
    trs(|| format!("S19[pc={} a_out={} b={} c={}]", pc, a_out, b_acc, c_acc));
    Some(Out { a: a_out, b: b_acc, c: c_acc, maxc, half28, f30, f31 })
}

/// 게이트 임계 역산: rg 값별로 (게임 통과한 최대 need, 게임 탈락한 최소 need) 를 모은다.
///   need = isqrt(d2). 임계 T 가 존재하면 true_max < T <= false_min 이어야 한다 — 역전되면 rg 외의 변수가 있다는 증거.
pub static EDGE: [std::sync::atomic::AtomicU64; 24] = [const { std::sync::atomic::AtomicU64::new(0) }; 24];
fn gate_edge(rg: u64, need: u64, game_pass: bool) {
    use std::sync::atomic::Ordering::Relaxed;
    let slot = match rg { 0..=59999 => 0, 60000..=89999 => 1, 90000..=99999 => 2, 100000..=119999 => 3, 120000..=159999 => 4, _ => 5 };
    let b = slot * 4;
    if game_pass { EDGE[b].fetch_max(need, Relaxed); EDGE[b + 1].fetch_add(1, Relaxed); }
    else { let mut cur = EDGE[b + 2].load(Relaxed); if cur == 0 { cur = u64::MAX; EDGE[b + 2].store(u64::MAX, Relaxed); }
           let _ = cur; EDGE[b + 2].fetch_min(need, Relaxed); EDGE[b + 3].fetch_add(1, Relaxed); }
}
pub fn edge_report() -> String {
    use std::sync::atomic::Ordering::Relaxed;
    let lab = ["rg<60k", "rg 60~90k", "rg 90~100k", "rg 100~120k", "rg 120~160k", "rg>=160k"];
    let mut s = String::from("=== 게이트 임계 역산(need = isqrt(d2)) ===
");
    for i in 0..6 { let (tmax, tn, fmin, fn_) = (EDGE[i * 4].load(Relaxed), EDGE[i * 4 + 1].load(Relaxed), EDGE[i * 4 + 2].load(Relaxed), EDGE[i * 4 + 3].load(Relaxed));
        if tn + fn_ != 0 { s += &format!("{:<12} 통과n={} 최대need={} | 탈락n={} 최소need={} {}
", lab[i], tn, tmax, fn_, if fmin == u64::MAX { 0 } else { fmin }, if fmin != u64::MAX && tmax >= fmin { "★역전" } else { "" }); } }
    s
}
/// 게이트 공식 후보별 적중 집계(후보 8종 × ok/ng)
pub static GATE_STAT: [std::sync::atomic::AtomicU64; 16] = [const { std::sync::atomic::AtomicU64::new(0) }; 16];
#[inline] fn gate_stat(i: usize, ok: bool) { if i < 8 { GATE_STAT[i * 2 + usize::from(!ok)].fetch_add(1, std::sync::atomic::Ordering::Relaxed); } }
pub fn gate_report() -> String {
    let n = ["rg+18000", "rg+19000", "rg+20000", "rg+22000", "rg+24000", "rg+18000+ri", "rg+18000+rt", "rg+0"];
    let mut s = String::from("=== S7 적 타워 게이트 후보 적중(게임 = d96d00 호출 여부) ===
");
    for i in 0..8 { let (ok, ng) = (GATE_STAT[i * 2].load(std::sync::atomic::Ordering::Relaxed), GATE_STAT[i * 2 + 1].load(std::sync::atomic::Ordering::Relaxed));
        let t = ok + ng; if t != 0 { s += &format!("{:<22} ok={} ng={} ({:.3}%)
", n[i], ok, ng, ok as f64 * 100.0 / t as f64); } }
    s
}
/// 게이트 공식 역산용 표본 로그(최대 200줄). judge_pe_gate.txt
/// 게임이 게이트를 통과한 순간의 실측 — PASS/FAIL 집계 + FAIL 표본 200 줄
pub static TRUTH: [std::sync::atomic::AtomicU64; 2] = [const { std::sync::atomic::AtomicU64::new(0) }; 2];
static TRUTH_N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn gate_truth(d2: u64, t: u64, ix: u64, iy: u64, qx: u64, qy: u64, f: &[u64; 8], gs: &[u64; 8]) {
    let ok = d2 <= t.wrapping_mul(t);
    TRUTH[usize::from(!ok)].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if !ok && TRUTH_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 200 {
        if let Some(p) = crate::pth("judge_pe_gate_truth.txt") {
            let line = format!("d2={} T={} T2={} item=({},{}) q=({},{}) f={:?} gs={:?}
", d2, t, t.wrapping_mul(t), ix, iy, qx, qy, f, gs);
            let _ = std::fs::OpenOptions::new().create(true).append(true).open(p).and_then(|mut fh| { use std::io::Write; fh.write_all(line.as_bytes()) });
        }
    }
}
static MTRUTH: [std::sync::atomic::AtomicU64; 2] = [std::sync::atomic::AtomicU64::new(0), std::sync::atomic::AtomicU64::new(0)];
static MTRUTH_N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// ★단측 오라클: 게임이 `0xd88043` 에서 실제로 통과시킨 (item, q) 에 대해 **내 모델**이 통과하는가.
unsafe fn mine_truth(d2: u64, t: u64, item: usize, se: usize, qx: u64, qy: u64) {
    let ok = d2 <= t.wrapping_mul(t);
    MTRUTH[usize::from(!ok)].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if !ok && MTRUTH_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 200 {
        if let Some(p) = crate::pth("judge_pe_gate_mine.txt") {
            let need = isqrt_fast(d2);
            let e8 = vt_e8(item + SLOT0, item, se);
            let line = format!("need={} T={} 부족={} | 438={} 4a0={} 4a8={} lv={} e8={:?} e8impl={:#x} 4c0={} ri={:?} rt={:?} q=({},{}) item=({:?},{:?})
",
                need, t, need.saturating_sub(t),
                rd_u64(item + ENT_F438).unwrap_or(0), rd_u64(item + 0x4a0).unwrap_or(0), rd_u64(item + 0x4a8).unwrap_or(0),
                rd_u64(item + ENT_LEVEL).unwrap_or(0), e8,
                dy::impl_rva(rd_u64(item + SLOT0 + 8).unwrap_or(0) as usize, 0xe8).unwrap_or(0),
                rd_i32(item + 0x4c0).unwrap_or(-9), radius(item), radius(se), qx, qy, rd_u64(item + ENT_X), rd_u64(item + ENT_Y));
            let _ = std::fs::OpenOptions::new().create(true).append(true).open(p).and_then(|mut fh| { use std::io::Write; fh.write_all(line.as_bytes()) });
        }
    }
}
static GBX: [std::sync::atomic::AtomicU64; 4] = [const { std::sync::atomic::AtomicU64::new(0) }; 4];
static GBXN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn gbx_tally(same: bool, pass18: bool) {
    GBX[usize::from(!same)].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    GBX[2 + usize::from(!pass18)].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}
unsafe fn gbx_log(gbx: u64, d2: u64, rg: u64, item: usize, qx: u64, qy: u64) {
    if GBXN.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= 60 { return; }
    if let Some(p) = crate::pth("judge_pe_gbx.txt") {
        let line = format!("game_d2={} mine_d2={} isq_g={} isq_m={} rg={} T18={} item=({:?},{:?}) q=({},{})
",
            gbx, d2, isqrt_fast(gbx), isqrt_fast(d2), rg, rg + 18000,
            rd_u64(item + ENT_X), rd_u64(item + ENT_Y), qx, qy);
        let _ = std::fs::OpenOptions::new().create(true).append(true).open(p).and_then(|mut f| { use std::io::Write; f.write_all(line.as_bytes()) });
    }
}
pub fn gbx_report() -> String {
    let (a, b) = (GBX[0].load(std::sync::atomic::Ordering::Relaxed), GBX[1].load(std::sync::atomic::Ordering::Relaxed));
    if a + b == 0 { return String::new(); }
    let (c, e) = (GBX[2].load(std::sync::atomic::Ordering::Relaxed), GBX[3].load(std::sync::atomic::Ordering::Relaxed));
    format!("=== ★게임의 rbx(=게이트 d²) vs 내 d² ===
같음={} 다름={} ({:.3}%)  |  게임 rbx 로 rg+18000 판정: 통과={} 탈락={} ({:.3}%)
",
        a, b, a as f64 * 100.0 / (a + b) as f64, c, e, c as f64 * 100.0 / (c + e) as f64)
}
static CMPT: [[std::sync::atomic::AtomicU64; 2]; 3] = [const { [const { std::sync::atomic::AtomicU64::new(0) }; 2] }; 3];
fn cmp_tally(i: usize, ok: bool) { CMPT[i][usize::from(!ok)].fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
static COMPT: [[std::sync::atomic::AtomicU64; 2]; 8] = [const { [const { std::sync::atomic::AtomicU64::new(0) }; 2] }; 8];
fn comp_tally(i: usize, same: bool) { COMPT[i][usize::from(!same)].fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
pub fn comp_report() -> String {
    const M: [&str; 3] = ["내 rg + 18000", "프레임 rg(4c0 게이트) + 18000", "프레임 rg(게이트 미적용) + 18000"];
    const C: [&str; 8] = ["f378 vs item.438", "f380 vs item.4a0", "f388 vs item.4a8", "f408 vs item.5c8",
                          "f390 vs vt_e8", "f3d8 vs radius(item)", "f3e0 vs radius(self)", "rg_frame vs rg_mine"];
    let t = CMPT[0][0].load(std::sync::atomic::Ordering::Relaxed) + CMPT[0][1].load(std::sync::atomic::Ordering::Relaxed);
    if t == 0 { return String::new(); }
    let mut s = String::from("=== ★모델별 단측 적중(정정 슬롯) ===
");
    for i in 0..3 { let (a, b) = (CMPT[i][0].load(std::sync::atomic::Ordering::Relaxed), CMPT[i][1].load(std::sync::atomic::Ordering::Relaxed));
        s += &format!("{:<32} 통과={:<10} 탈락={:<10} {:.3}%
", M[i], a, b, a as f64 * 100.0 / (a + b) as f64); }
    s += "=== ★성분별 프레임↔재현 일치 ===
";
    for i in 0..8 { let (a, b) = (COMPT[i][0].load(std::sync::atomic::Ordering::Relaxed), COMPT[i][1].load(std::sync::atomic::Ordering::Relaxed));
        s += &format!("{:<24} 같음={:<10} 다름={:<10} {:.3}%
", C[i], a, b, a as f64 * 100.0 / (a + b) as f64); }
    s
}
/// 프레임 q 슬롯 탐색: rbp+0x600 .. rbp+0x800 을 8바이트 간격으로 훑는다.
const QSCAN_BASE: usize = 0x600; const QSCAN_N: usize = 64;
static QSCAN: [[std::sync::atomic::AtomicU64; 2]; QSCAN_N] = [const { [const { std::sync::atomic::AtomicU64::new(0) }; 2] }; QSCAN_N];
fn qscan(k: usize, ok: bool) { QSCAN[k][usize::from(!ok)].fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
pub fn qscan_report() -> String {
    let tot = QSCAN[0][0].load(std::sync::atomic::Ordering::Relaxed) + QSCAN[0][1].load(std::sync::atomic::Ordering::Relaxed);
    if tot == 0 { return String::new(); }
    let mut v: Vec<(usize, u64, u64)> = (0..QSCAN_N).map(|k| (k,
        QSCAN[k][0].load(std::sync::atomic::Ordering::Relaxed), QSCAN[k][1].load(std::sync::atomic::Ordering::Relaxed))).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    let mut s = format!("=== ★프레임 q 슬롯 탐색(게임 통과 표본 {} 건에서 d2<= (rg+18000)² 를 만족하는 (rbp+o, rbp+o+8)) ===
", tot);
    for (k, a, b) in v.into_iter().take(8) {
        s += &format!("rbp+{:#05x}  만족={:<10} 불만족={:<10} {:.3}%
", QSCAN_BASE + k * 8, a, b, a as f64 * 100.0 / (a + b) as f64);
    }
    s
}
static NEGMIN: [std::sync::atomic::AtomicU64; 2] = [const { std::sync::atomic::AtomicU64::new(u64::MAX) }; 2];
static NEGN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn neg_min(i: usize, v: u64) { NEGMIN[i].fetch_min(v, std::sync::atomic::Ordering::Relaxed); }
fn neg_cnt() { NEGN.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
pub fn neg_report() -> String {
    let n = NEGN.load(std::sync::atomic::Ordering::Relaxed); if n == 0 { return String::new(); }
    format!("=== ★깨끗한 음성(gcalls==0) 표본의 하한 (n={}) ===
min(need_q − rg)={}  min(need_self − rg)={}
  ⟹ 이 값이 통과표본 max(need−rg)=23999 이하면 상수 임계는 불가능
",
        n, NEGMIN[0].load(std::sync::atomic::Ordering::Relaxed), NEGMIN[1].load(std::sync::atomic::Ordering::Relaxed))
}
static NEEDMAX: [std::sync::atomic::AtomicU64; 6] = [const { std::sync::atomic::AtomicU64::new(0) }; 6];
fn need_max(i: usize, v: u64) { NEEDMAX[i].fetch_max(v, std::sync::atomic::Ordering::Relaxed); }
pub fn need_report() -> String {
    const N: [&str; 6] = ["need-rg", "need-rg-ri", "need-rg-rt", "need-rg-ri-rt", "need-ru", "need-rg-ri/2"];
    if NEEDMAX[0].load(std::sync::atomic::Ordering::Relaxed) == 0 { return String::new(); }
    let mut s = String::from("=== ★필요 임계 역산(게임 통과 표본의 최댓값 = 그 형태의 하한. 18000 이면 정답) ===
");
    for i in 0..6 { s += &format!("{:<14} max={}
", N[i], NEEDMAX[i].load(std::sync::atomic::Ordering::Relaxed)); }
    s
}
const NCAND: usize = 16;
static CAND: [[std::sync::atomic::AtomicU64; 2]; NCAND] = [const { [const { std::sync::atomic::AtomicU64::new(0) }; 2] }; NCAND];
fn cand_tally(i: usize, pass: bool) { CAND[i][usize::from(!pass)].fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
pub fn cand_report() -> String {
    const NAMES: [&str; NCAND] = ["rg+18000", "rg+20000", "rg+22000", "rg+24000", "rg+26000", "rg+30000",
                                  "rg+18000+ri", "rg+18000+rt", "rg+18000+ri+rt",
                                  "ru+18000", "ru+18000+ri", "ru+24000", "rg+32000", "rg+50000", "rg*2", "rg"];
    let tot: u64 = CAND[0][0].load(std::sync::atomic::Ordering::Relaxed) + CAND[0][1].load(std::sync::atomic::Ordering::Relaxed);
    if tot == 0 { return String::new(); }
    let mut s = String::from("=== ★후보별 단측 적중(게임이 통과시킨 표본만 · 정답은 100%) ===
");
    for i in 0..NCAND {
        let (a, b) = (CAND[i][0].load(std::sync::atomic::Ordering::Relaxed), CAND[i][1].load(std::sync::atomic::Ordering::Relaxed));
        s += &format!("{:<16} 통과={:<10} 탈락={:<10} {:.4}%
", NAMES[i], a, b, a as f64 * 100.0 / (a + b) as f64);
    }
    s
}
static SELFT: [std::sync::atomic::AtomicU64; 2] = [const { std::sync::atomic::AtomicU64::new(0) }; 2];
fn self_tally(same: bool) { SELFT[usize::from(!same)].fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
pub fn self_report() -> String {
    let (a, b) = (SELFT[0].load(std::sync::atomic::Ordering::Relaxed), SELFT[1].load(std::sync::atomic::Ordering::Relaxed));
    if a + b == 0 { return String::new(); }
    format!("=== ★self: 프레임 [rbp+0x720] vs 로스터 유도 ===
같음={} 다름={} ({:.3}%)
", a, b, a as f64 * 100.0 / (a + b) as f64)
}
pub fn mine_truth_report() -> String {
    let (a, b) = (MTRUTH[0].load(std::sync::atomic::Ordering::Relaxed), MTRUTH[1].load(std::sync::atomic::Ordering::Relaxed));
    if a + b == 0 { return String::new(); }
    format!("=== ★단측 오라클: 게임이 통과시킨 (item,q) 에서 **내 모델**(range_g+18000) 판정 ===
내 모델도 통과={} 내 모델은 탈락={} | {:.3}%
", a, b, a as f64 * 100.0 / (a + b) as f64)
}
pub fn truth_report() -> String {
    let (a, b) = (TRUTH[0].load(std::sync::atomic::Ordering::Relaxed), TRUTH[1].load(std::sync::atomic::Ordering::Relaxed));
    if a + b == 0 { return String::new(); }
    format!("=== 게임이 d96d00 을 부른 순간의 게이트 실측(모델: d2 <= (rg+18000)²) ===
모델도 통과={} 모델은 탈락={} | 모델 적중 {:.3}%
", a, b, a as f64 * 100.0 / (a + b) as f64)
}
static GATE_N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[allow(clippy::too_many_arguments)]
fn gate_log(d2: u64, rg: u64, ri: u64, rt: u64, f438: u64, f4a0: u64, f4a8: u64, lv: u64, f4c0: i64, f470: i64, f680: u64, t470: i64, t680: u64, game: bool) {
    if GATE_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= 200 { return; }
    let need = isqrt_fast(d2);
    let line = format!("d2={} need={} rg={} ri={} rt={} 438={} 4a0={} 4a8={} lv={} 4c0={} 470={} 680={} t470={} t680={} game={}
", d2, need, rg, ri, rt, f438, f4a0, f4a8, lv, f4c0, f470, f680, t470, t680, game);
    if let Some(p) = crate::pth("judge_pe_gate_ng.txt") { let _ = std::fs::OpenOptions::new().create(true).append(true).open(p).and_then(|mut f| { use std::io::Write; f.write_all(line.as_bytes()) }); }
}

/// 훅 대조용: threat(a, G, src, tgt) 순수 재현. relevant 게이트 포함(게임 0x132b310 전제).
pub unsafe fn threat_diag(a: usize, src: usize, tgt: usize) -> String {
    if !ptr_ok(a) || !ptr_ok(src) || !ptr_ok(tgt) { return "bad ptr".into(); }
    let a40 = rd_u64(a + 0x40).unwrap_or(0); let k = if a40 >= 2 { a40 - 2 } else { 7 };
    let (b0, bn) = lb0(a).unwrap_or((0, 0)); let (l5, l5n) = l50(a).unwrap_or((0, 0));
    let (mut p, mut m, mut pc) = (0u64, 0u64, 0u64); let mut impls = String::new();
    let _ = list_iter(b0, bn, 0x18, |d, v| {
        let (x, y) = dy::eff28_damage(d, v, src)?; let z = dy::eff38_pct(d, v, src)?;
        impls += &format!("<i38={:#x} pct={} {}>", dy::impl_rva(v, 0x38).unwrap_or(0), z, dy::eff28_trace(d, v, src, 0));
        p = p.wrapping_add(x); m = m.wrapping_add(y); pc = pc.wrapping_add(z); Some(()) });
    let mh = rd_u64(tgt + ENT_MAXHP).unwrap_or(0);
    let raw = p.wrapping_add(pc.wrapping_mul(mh) / 100); let ty = rd_u32(a + 0x128);
    format!("k={} tt={} bn={} l5n={} p={} m={} pct={} mh={} raw={} ty={} conv0={:?} conv1={:?} ticks={:?} a60={} a68={} src440={:?} src448={:?} src450={:?} src460={:?} src418={:?} src420={:?} t630={:?} t638={:?} {}",
        k, rd_u32(a + 0x12c), bn, l5n, p, m, pc, mh, raw, ty, conv(src, tgt, raw, ty, 0), conv(src, tgt, m, ty, 1), ticks(a), rd_u64(a + 0x60).unwrap_or(0), rd_u64(a + 0x68).unwrap_or(0),
        rd_u64(src + 0x440), rd_u64(src + 0x448), rd_u64(src + 0x450), rd_u64(src + 0x460), rd_u64(src + 0x418), rd_u64(src + 0x420), rd_u64(tgt + 0x630), rd_u64(tgt + 0x638), impls)
}
pub unsafe fn threat_cmp(a: usize, src: usize, tgt: usize) -> Option<u64> {
    if !ptr_ok(a) || !ptr_ok(src) || !ptr_ok(tgt) { return None; }
    if !relevant(rd_u32(a + 0x12c), a, tgt)? { return Some(0); }
    let a40 = rd_u64(a + 0x40)?; let k = if a40 >= 2 { a40 - 2 } else { 7 };
    threat(&Act { a, k }, src, tgt)
}

// ── 래퍼 0xd84db0 의 TLS 메모(512버킷 × 0x68B) 직접 조회 ──────────────────────────────────
//   ★래퍼는 계산을 하지 않는다. 히트하면 게임 out 은 **그 틱의 이전 계산 결과**라, 본체를 새로 돌린 값과 다를 수 있다
//   (2026-09-06 23:25 실사고: 게임이 d96d00 을 한 번도 안 불렀는데 B 가산이 있던 케이스 = 캐시 히트).
//   그래서 호출 **전**에 게임 표를 조회해 히트면 그 엔트리를 그대로 재현값으로 쓴다(dn_cache 선례와 동일).
const PE_TLS_OFF: usize = 0x1410;
const GOLDEN: u64 = 0x9E37_79B9_7F4A_7C15;
unsafe fn wrap_cell() -> Option<usize> {
    let b = crate::exe_base(); if b == 0 { return None; }
    let idx = rd_u32(b + CAMP_MEMO_TLS_IDX) as usize; if idx > 0x1000 { return None; }
    let teb: usize; core::arch::asm!("mov {}, gs:[0x58]", out(reg) teb, options(nostack, readonly, preserves_flags));
    if !ptr_ok(teb) { return None; }
    let blk = rd_u64(teb + idx * 8)? as usize; if !ptr_ok(blk) { return None; }
    Some(blk + PE_TLS_OFF)
}
/// purpose → pkey (JT 0x33ddd6c 실덤프, case = purpose>=2 ? purpose-2 : 7)
fn pkey_of(purpose: u8) -> Option<u64> {
    let case = if purpose >= 2 { purpose - 2 } else { 7 };
    Some(match case { 0 => 0, 1 => 0x200, 2 => 0x400, 3 => 0x600, 4 => 0x800, 5 => 0xa00, 6 => 0xc00, 7 => ((purpose as u64) << 9) | 0x1000, 8 => 0x1400, 9 => 0x1600, 10 => 0x1800, _ => return None })
}
thread_local! { static PRE_HIT: std::cell::Cell<(bool, [u64; 9])> = const { std::cell::Cell::new((false, [0; 9])) }; static PRE_BODY: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }
/// 게임이 이번 호출에서 본체(0xd851d0)를 실제로 돌렸는가 = 래퍼 메모 **미스**. 내 pre_memo 판정의 정답 레이블.
pub fn game_body_ran() -> bool { crate::judge::cap_as_d851d0::count() > PRE_BODY.with(|c| c.get()) }
/// pre_memo 판정 대조 집계 [히트일치, 히트오판(내 히트·게임 미스), 미스오판(내 미스·게임 히트), 미스일치]
pub static MEMO_STAT: [std::sync::atomic::AtomicU64; 4] = [const { std::sync::atomic::AtomicU64::new(0) }; 4];
/// ★★임계치·마진·마스크는 **상수가 아니라 실행중 바이트**다 — ai_adjust 자신이 `pe_*` 노브로 덮어쓴다.
/// 정적 exe 상수를 하드코딩하면 노브를 건드린 순간 그 경로가 전부 DIFF 된다.
/// (2026-09-07 실측: tower_margin 18000→24000 · count_radius 120000²→150000² · kind_mask 0x503→0x303)
#[inline] unsafe fn imm32(site: usize, orig: u32) -> u64 { super::super::live_imm32(site, orig) as u64 }
/// ★사이트별 영향도 계수(검증 한정). `live` 값과 `static` 원본값이 판정을 **실제로 뒤집는** 표본만 센다.
/// 8곳을 한꺼번에 배선했는데 DIFF 가 오히려 늘어, **어느 사이트가 진짜로 쓰이는지**를 먼저 재야 한다.
pub static IMPACT: [std::sync::atomic::AtomicU64; 8] = [const { std::sync::atomic::AtomicU64::new(0) }; 8];
pub static IMPACT_N: [std::sync::atomic::AtomicU64; 8] = [const { std::sync::atomic::AtomicU64::new(0) }; 8];
#[inline] fn imp(i: usize, live: bool, stat: bool) -> bool {
    IMPACT_N[i].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if live != stat { IMPACT[i].fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
    live
}
pub fn impact_report() -> String {
    let nm = ["filter>=(716)", "filter<(745)", "near<(747)", "filter>=(849)", "filter>=(880)", "field>(896)", "kindmask(842)", "count/tower"];
    let mut s = String::from("[PEimm] live≠static 로 판정이 뒤집힌 표본
");
    for i in 0..8 { let n = IMPACT_N[i].load(std::sync::atomic::Ordering::Relaxed); if n == 0 { continue; }
        s += &format!("  {:<16} flip={} / n={} ({:.4}%)
", nm[i], IMPACT[i].load(std::sync::atomic::Ordering::Relaxed), n, IMPACT[i].load(std::sync::atomic::Ordering::Relaxed) as f64 * 100.0 / n as f64); }
    s
}
#[inline] unsafe fn imm64(site: usize, orig: u64) -> u64 { super::super::live_imm64(site, orig) }
pub fn memo_report() -> String {
    let g = |i: usize| MEMO_STAT[i].load(std::sync::atomic::Ordering::Relaxed);
    let t = g(0) + g(1) + g(2) + g(3);
    if t == 0 { return String::new(); }
    format!("=== 래퍼 메모 판정 대조(정답 = 게임이 본체 0xd851d0 을 돌았는가) ===
히트일치={} 히트오판(내히트·게임미스)={} 미스일치={} 미스오판(내미스·게임히트)={} | 정확도 {:.3}%
",
        g(0), g(1), g(2), g(3), (g(0) + g(2)) as f64 * 100.0 / t as f64)
}
/// 호출 전 스냅샷: 게임 래퍼 메모가 이 (tick, sim.928, qx, qy, purpose, mode) 로 히트하는가.
pub unsafe fn pre_memo(mode: usize, sim: usize, holder: usize, qx: usize, qy: usize, purpose: usize) {
    PRE_HIT.with(|c| c.set((false, [0; 9])));
    let hit = memo_lookup(mode, sim, holder, qx, qy, purpose);
    if let Some(w) = hit { PRE_HIT.with(|c| c.set((true, w))); }
    PRE_BODY.with(|c| c.set(crate::judge::cap_as_d851d0::count()));
}
/// 래퍼 0xd84db0 의 TLS 메모 표를 그대로 조회한다(게임이 히트시키는 것과 같은 키).
pub unsafe fn memo_lookup(mode: usize, sim: usize, holder: usize, qx: usize, qy: usize, purpose: usize) -> Option<[u64; 9]> {
    (|| -> Option<[u64; 9]> {
        if !ptr_ok(sim) || !ptr_ok(holder) { return None; }
        let x = rd_u64(holder)? as usize; if !ptr_ok(x) { return None; }
        let data = rd_u64(x)? as usize; if !ptr_ok(data) { return None; }
        let (seed, tick) = (rd_u64(data + W_SEED)?, rd_u64(data + W_TICK)?);
        let cell = wrap_cell()?;
        if rd_u8(cell + 0x18) != 1 || rd_u64(cell)? != 0 { return None; }
        if rd_u64(cell + 0x10)? != seed { return None; }
        let tbl = rd_u64(cell + 8)? as usize; if !ptr_ok(tbl) { return None; }
        let key928 = rd_u64(sim + P5_MEMO_KEY)?;
        let (qx, qy, pu) = (qx as u64, qy as u64, (purpose & 0xff) as u8);
        let h = ((qy << 21) ^ (key928 << 42) | pkey_of(pu)?) ^ qx;
        let b = (h.wrapping_mul(GOLDEN) >> 55) as usize; if b >= 512 { return None; }
        let e = tbl + b * 0x68;
        if rd_u64(e)? != tick || rd_u64(e + 8)? != key928 || rd_u64(e + 0x10)? != qx || rd_u64(e + 0x18)? != qy
           || rd_u8(e + 0x20) != pu || rd_u64(e + 0x28)? != mode as u64 { return None; }
        let mut w = [0u64; 9];
        for i in 0..7 { w[i] = rd_u64(e + 0x30 + i * 8)?; }
        Some(w)
    })()
}

// ── 훅 어댑터 ─────────────────────────────────────────────────────────────────────────────
/// 래퍼 0xd84db0 인자(out, mode, sim, H, qx, qy, purpose) → out 9워드 (w[6] 하위 2바이트 = f30|f31<<8)
pub unsafe fn pe_from_args(p2: usize, p3: usize, p4: usize, p5: usize, p6: usize, p7: usize) -> Option<[u64; 9]> {
    if !ptr_ok(p3) || !ptr_ok(p4) { return None; }
    LAST_ARGS.with(|c| c.set((p5, p6, p7)));
    // 래퍼 메모 히트 = 게임이 계산을 안 했다 → 그 캐시값이 곧 게임 출력
    let (hit, w) = PRE_HIT.with(|c| c.get());
    { let ran = game_body_ran(); MEMO_STAT[usize::from(hit == ran) + usize::from(!hit) * 2].fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
    if hit { return Some(w); }
    let o = position_eval(p2 as u64, p3, p4, p5 as u64, p6 as u64, (p7 & 0xff) as u8)?;
    Some([o.a as u64, o.b as u64, o.c as u64, o.maxc as u64, 0, o.half28 as u64, (o.f30 as u64) | ((o.f31 as u64) << 8), 0, 0])
}
pub fn pe_eq(g: &[u64; 9], m: &[u64; 9]) -> bool { g[..6] == m[..6] && (g[6] & 0xffff) == (m[6] & 0xffff) }
/// 진단 문자열(DIFF 로그용)
pub unsafe fn diag(p2: usize, p3: usize, p4: usize) -> String {
    let side = rd_u64(p3 + P5_SIDE).unwrap_or(9); let role = if ptr_ok(p3) { rd_u32(p3 + P5_ROLE) } else { 99 };
    let x = rd_u64(p4).unwrap_or(0) as usize; let tick = if ptr_ok(x) { rd_u64(x).and_then(|d| rd_u64(d as usize + W_TICK)).unwrap_or(0) } else { 0 };
    let unseen = dy::unseen_report(); let un: Vec<&str> = unseen.lines().skip(1).take(4).collect();
    let (p5, p6, p7) = LAST_ARGS.with(|c| c.get());
    TRACE.with(|t| *t.borrow_mut() = Some(String::new()));
    let _ = position_eval(p2 as u64, p3, p4, p5 as u64, p6 as u64, (p7 & 0xff) as u8);
    let trace = TRACE.with(|t| t.borrow_mut().take()).unwrap_or_default();
    format!("side={} role={} tick={} mode={} q=({},{}) purpose={} | {} | unseen: {}", side, role, tick, p2, p5, p6, p7 & 0xff, trace, un.join(" ; "))
}
