//! fight_check — `0xeba9b0` "self 가 지금 자리에서 버틸 수 있는 틱 수" + 메모 래퍼 `0xeb82d0` 의 **2단계 순수 포팅**(2026-09-06 20:10).
//!   정본 = REPORT RE\2026-09-06_fight_check-생존시간추정-0xeba9b0-메모래퍼-0xeb82d0-노이즈해시-매트릭스-순수재현용-0.5.8.md
//!   result = sat_sub(hp + Σself.effs.vt90, burst) * 60 / max(1, incoming)
//!     burst   = Σ_A max(평타,s1,s2,궁 × 노이즈) + Σ_B est×노이즈 + Σ_적측유닛(150k∧가시) est
//!     incoming = sat_sub(Σ_A T[10/11/12]×노이즈 + Σ_B tps·est·노이즈/공격간격 + Σ_적측 tps·est/공격간격 + Σ_A effs.vt80, Σ_아군 effs.vt88) (+ 조건부 d958b0)
//!   노이즈 = splitmix64 스텝을 "계산 사이트" 에서만 진행(§2 표). 표 T = X+0x280 [[[[u64;5];20];5];2] score_parameter 매트릭스.
//!   콜사이트(base cat4 0xd597ed / combat 0xd5c4c2): (p1, _, holder{X,G,lanes}, sim, self, &A(적 챔프 ≤8), &B(빈 Vec)).
//!   메모(eb82d0): lenA<9 && lenB<13 일 때 키 {A 핸들 8, B 핸들 12, p1, sim.928, self.h, lenA, lenB} + 에포크 (seed w+0xec90, tick w+0xec98) — 스레드로컬 미러.
//!   dyn: 프로바이더 vt+0x90(쿨)/+0xa8(충전) 바이트 디코드 · 스킬 vt+0x120(§3-d) · 효과 리스트 +0x2f8 vt+0x80/+0x88/+0x90 = 미확정 패밀리(unseen 로 수집).
#![allow(dead_code)]
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::dyn_eff as dy;
use super::passive_jungle::estimate_damage;
use super::as_callees::eff_bool;
use super::action_score::sim_of_handle;

const GOLDEN: u64 = 0x9E3779B97F4A7C15;
const X_MATRIX: usize = 0x280; const MAT_SIDE: usize = 0xfa0; const MAT_ROLE: usize = 0x320; const MAT_K: usize = 0x28;
const X_SIDE_UNITS_PTR: usize = 0xf0; const X_SIDE_UNITS_LEN: usize = 0x108;
const X_LANE_UNITS: [usize; 3] = [0x10, 0x50, 0x90];
const SIM_218: usize = 0x218; const SIM_450: usize = 0x450;
const CFG_8A8: usize = 0x8a8; const CFG_TPS: usize = 0x12f8;
const W_SEED_EC90: usize = 0xec90;
const G_PHASE: usize = 0x38;
const ENT_KIND: usize = 0x68; const ENT_ATK_TIMER: usize = 0xb0; const ENT_S1_CD: usize = 0xb8; const ENT_S2_CD: usize = 0xc0; const ENT_ULT_CD: usize = 0xc8;
const ENT_3FC: usize = 0x3fc; const ENT_400: usize = 0x400; const ENT_46C: usize = 0x46c;
const ENT_88: usize = 0x88; const ENT_90: usize = 0x90; const ENT_118: usize = 0x118;
const ENT_BUFF_PTR: usize = 0x2c8; const ENT_BUFF_LEN: usize = 0x2d0;
const PROV_ATK: usize = 0x570; const PROV_S1: usize = 0x580; const PROV_S2: usize = 0x590; const PROV_ULT: usize = 0x5a0; const PROV_5B0: usize = 0x5b0;
const SLOT0: usize = 0x490; const SLOT1: usize = 0x4c8; const SLOT2: usize = 0x500; const SLOT3: usize = 0x538;
const D2_150K_P1: u64 = 0x53d1ac101; const D2_120K: u64 = 0x35a4e9000; const D2_90K: u64 = 0x1e2cc3100;
const MODE_240: usize = 0x240;
const PHASE_MASK: u32 = 0x1a1;

#[inline] fn coef(h: u64, mid: u64, span: u64) -> u64 {
    let mut z = h; z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9); z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB); z ^= z >> 31;
    mid.wrapping_add(((z as u128 * span as u128) >> 64) as u64)
}
#[inline] unsafe fn xy(e: usize) -> Option<(u64, u64)> { Some((rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?)) }
#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
#[inline] fn sqd_sat(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx } as u128; let dy = if ay < by { by - ay } else { ay - by } as u128;
    let s = dx * dx + dy * dy; if s > u64::MAX as u128 { u64::MAX } else { s as u64 }
}
/// (has3, has4, has5, any_not_in{2,3,4,5}) — 버프 Vec(+0x2c8/+0x2d0, stride 0x28, [0] i32 kind)
unsafe fn buff_kinds(e: usize) -> Option<(bool, bool, bool, bool)> {
    let n = rd_u64(e + ENT_BUFF_LEN)?; if n == 0 { return Some((false, false, false, false)); }
    let p = rd_u64(e + ENT_BUFF_PTR)? as usize; if !ptr_ok(p) { return None; }
    let (mut h3, mut h4, mut h5, mut bad) = (false, false, false, false);
    for i in 0..n.min(256) as usize { let k = rd_i32(p + i * 0x28)?; match k { 3 => h3 = true, 4 => h4 = true, 5 => h5 = true, 2 => {}, _ => bad = true } }
    Some((h3, h4, h5, bad))
}
/// kind 별 타이머 오프셋(JT 0x1433f24b0 / 0x143440a84): None = 타이머 없이 항상 계산(kind 0/3)
fn timer_off(kind: u32) -> Option<Option<usize>> {
    Some(match kind { 0 | 3 => None, 1 => Some(0xb8), 2 => Some(0x110), 4 | 7 => Some(0xe8), 5 | 6 => Some(0x1f0), 8 | 13 => Some(0xb0), 9 => Some(0xc8), 10 => Some(0xf0), 11 => Some(0xd8), 12 => Some(0xd0), _ => return None })
}
/// 0x128cc90 "지금 평타 가능": 버프 kind 3 → false · kind 0/3 → false · else timer==0 && 모든 버프 kind ∈ {2,4,5}
unsafe fn can_basic_attack(e: usize) -> Option<bool> {
    let (h3, _, _, bad) = buff_kinds(e)?; if h3 { return Some(false); }
    let kind = rd_u32(e + ENT_KIND);
    match timer_off(kind)? { None => Some(false), Some(off) => Some(rd_u64(e + off)? == 0 && !bad) }
}
/// 프로바이더 vt+0xa8 충전수 — 바이트 디코드(`b8 01 00 00 00 c3`=1 · `48 8b 41 K c3`/`48 8b 81 K32 c3`=[data+K]); 그 외 unseen(0xa8)
unsafe fn prov_a8_charges(data: usize, vt: usize) -> Option<u64> {
    let rva = dy::impl_rva(vt, 0xa8)?; let t = exe_base() + rva;
    let (b0, b1, b2) = (rd_u8(t), rd_u8(t + 1), rd_u8(t + 2));
    if b0 == 0xb8 && rd_u32(t + 1) == 1 && rd_u8(t + 5) == 0xc3 { return Some(1); }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x41 && rd_u8(t + 4) == 0xc3 { return rd_u64(data + rd_u8(t + 3) as usize); }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x81 && rd_u8(t + 7) == 0xc3 { return rd_u64(data + rd_u32(t + 3) as usize); }
    // `mov rax,[rcx+K32]; cmp rax,1; adc rax,0; ret` = max([d+K],1) (0x1716a40 등)
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x81 && rd_u8(t + 7) == 0x48 && rd_u8(t + 8) == 0x83 && rd_u8(t + 9) == 0xf8 && rd_u8(t + 10) == 0x01 && rd_u8(t + 11) == 0x48 && rd_u8(t + 12) == 0x83 && rd_u8(t + 13) == 0xd0 { return Some(rd_u64(data + rd_u32(t + 3) as usize)?.max(1)); }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x41 && rd_u8(t + 4) == 0x48 && rd_u8(t + 5) == 0x83 && rd_u8(t + 6) == 0xf8 && rd_u8(t + 7) == 0x01 && rd_u8(t + 8) == 0x48 && rd_u8(t + 9) == 0x83 && rd_u8(t + 10) == 0xd0 { return Some(rd_u64(data + rd_u8(t + 3) as usize)?.max(1)); }
    dy::unseen(0xa8, rva); None
}
#[inline] unsafe fn prov(e: usize, off: usize) -> Option<(usize, usize)> { Some((rd_u64(e + off)? as usize, rd_u64(e + off + 8)? as usize)) }
/// 스킬 vt+0x120 (RE §3-d 구현체 전수): false/true/[p+0x18]!=0/합성(자식 vt60‖vt68) — 나머지 unseen(0x120)
pub unsafe fn eff_vt120(data: usize, vt: usize, depth: u32) -> Option<bool> {
    if depth > 8 || !ptr_ok(vt) { return None; }
    let r = dy::impl_rva(vt, 0x120)?; let p = dy::arc_payload(data, vt)?;
    let any_children = |arr_off: usize, len_off: usize, stride: usize| -> Option<bool> {
        let n = rd_u64(p + len_off)?; if n == 0 { return Some(false); }
        let arr = rd_u64(p + arr_off)? as usize; if !ptr_ok(arr) { return None; }
        for i in 0..n.min(64) as usize {
            let (cd, cv) = (rd_u64(arr + i * stride)? as usize, rd_u64(arr + i * stride + 8)? as usize);
            if eff_bool(cd, cv, 0x60, depth + 1)? || eff_bool(cd, cv, 0x68, depth + 1)? { return Some(true); }
        }
        Some(false)
    };
    match r {
        EFF_E8_ZERO => Some(false),
        EFF_TRUE => Some(true),
        EFF68_NONZERO18 => Some(rd_u64(p + 0x18)? != 0),
        0x1309230 | 0x14142d0 => any_children(8, 0x10, 0x10),
        0x13bede0 => { let n = rd_u64(p + 0x58)?; if n == 0 { return Some(false); } let arr = rd_u64(p + 0x50)? as usize; if !ptr_ok(arr) { return None; }
            for i in 0..n.min(64) as usize { let (cd, cv) = (rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize); if eff_bool(cd, cv, 0x68, depth + 1)? { return Some(true); } } Some(false) }
        // 0x122e650: 리스트1(stride 0x18 @p+0x50/len p+0x58) any(vt68) → 없으면 리스트2(stride 0x10 @p+0x68/len p+0x70) any(vt68) (capstone 2026-09-06 20:20)
        0x122e650 => {
            let n1 = rd_u64(p + 0x58)?; if n1 != 0 { let arr = rd_u64(p + 0x50)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n1.min(64) as usize { let (cd, cv) = (rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize); if eff_bool(cd, cv, 0x68, depth + 1)? { return Some(true); } } }
            let n2 = rd_u64(p + 0x70)?; if n2 == 0 { return Some(false); } let arr = rd_u64(p + 0x68)? as usize; if !ptr_ok(arr) { return None; }
            for i in 0..n2.min(64) as usize { let (cd, cv) = (rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize); if eff_bool(cd, cv, 0x68, depth + 1)? { return Some(true); } } Some(false) }
        // 0x1652b00: 단일 자식 (data @p+0x18, vt @p+0x20): vt60 ‖ vt68 ‖ vt58(sret u8 — 미재현 → unseen 0x58)
        0x1652b00 => {
            let (cd, cv) = (rd_u64(p + 0x18)? as usize, rd_u64(p + 0x20)? as usize);
            if eff_bool(cd, cv, 0x60, depth + 1)? || eff_bool(cd, cv, 0x68, depth + 1)? { return Some(true); }
            let r58 = dy::impl_rva(cv, 0x58)?; dy::unseen(0x58, r58); None }
        _ => { dy::unseen(0x120, r); None }
    }
}
#[inline] unsafe fn rng_of(e: usize) -> Option<u64> {
    let p = rd_i32(e + ENT_F470)? as i64; let r = rd_u64(e + ENT_F680)?;
    Some(if p == 0 { r } else { ((p + 100) as u64).wrapping_mul(r) / 100 })
}
#[inline] unsafe fn d2_ee(a: usize, b: usize) -> Option<u64> { let (ax, ay) = xy(a)?; let (bx, by) = xy(b)?; Some(sqd(ax, ay, bx, by)) }
/// 0x126be70(att, tgt, amount, kind=1, flag): flag 0 = 물리(tgt.630 / att.418) · 1 = 마법(tgt.638 / att.420) · 그 외 = 그대로. kind 1: +att.450*tgt.628/100 +att.448*att.628/100. 결과 max(amount*100/(resist+100), 1)
unsafe fn def_dmg(att: usize, tgt: usize, mut amount: u64, flag: u32) -> Option<u64> {
    if flag > 1 { return Some(amount); }
    let a450 = rd_u64(att + 0x450)?; if a450 != 0 { amount = amount.wrapping_add(a450.wrapping_mul(rd_u64(tgt + ENT_MAXHP)?) / 100); }
    let a448 = rd_u64(att + 0x448)?; if a448 != 0 { amount = amount.wrapping_add(a448.wrapping_mul(rd_u64(att + ENT_MAXHP)?) / 100); }
    let (mut resist, pen) = if flag == 0 { (rd_u64(tgt + 0x630)?, rd_u64(att + 0x418)?) } else { (rd_u64(tgt + 0x638)?, rd_u64(att + 0x420)?) };
    if pen != 0 { let f = if pen < 0x65 { 100 - pen } else { 0 }; resist = f.wrapping_mul(resist) / 100; }
    let resist = resist.wrapping_add(100); if resist == 0 { return None; }
    let v = amount.wrapping_mul(100) / resist; Some(if v == 0 { 1 } else { v })
}
/// 효과 vt+0x80(data, G, e=공격자, self) → DPS 기여 (구현체별, 디컴 2026-09-06 20:25). 미재현 → unseen(0x180)
unsafe fn eff80_dps(rva: usize, d: usize, e: usize, me: usize, tps: u64) -> Option<u64> {
    match rva {
        EFF_E8_ZERO => Some(0),
        0x16b96e0 => {
            if rd_u32(me + ENT_KIND) != 13 { return Some(0); }
            let (mx, my) = xy(me)?; let r = rd_u64(d + 0x18)?;
            if sqd(mx, my, rd_u64(d + 8)?, rd_u64(d + 0x10)?) > r.wrapping_mul(r) { return Some(0); }
            let amt = rd_u64(d + 0x38)?.wrapping_mul(rd_u64(me + ENT_MAXHP)?) / 100 + rd_u64(d + 0x40)?.wrapping_mul(rd_u64(e + 0x620)?) / 100;
            let dmg = def_dmg(e, me, amt, 1)?; let per = { let v = rd_u64(d + 0x28)?; if v == 0 { 1 } else { v } };
            Some(dmg.wrapping_mul(tps) / per)
        }
        0x137dbd0 => {
            let r = rng_of(e)?.wrapping_add(rd_u64(d + 0x18)?).wrapping_add(rng_of(me)?);
            if r.wrapping_mul(r) < d2_ee(e, me)? { return Some(0); }
            let inner = rd_u64(d)? as usize; if !ptr_ok(inner) { return None; }
            let n = rd_u64(inner + 0x38)?; let (mut pp, mut mm) = (0u64, 0u64);
            if n != 0 { let arr = rd_u64(inner + 0x30)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n.min(64) as usize { let (cd, cv) = (rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize); let (a, b) = dy::eff28_damage(cd, cv, e)?; pp = pp.wrapping_add(a); mm = mm.wrapping_add(b); } }
            let dmg = def_dmg(e, me, pp, 0)?.wrapping_add(def_dmg(e, me, mm, 1)?);
            let per = { let v = rd_u64(d + 0x20)?; if v == 0 { 1 } else { v } };
            Some(dmg.wrapping_mul(tps) / per)
        }
        0x12bad20 => {
            let r = rd_u64(d + 0x20)?; if r.wrapping_mul(r) < d2_ee(e, me)? { return Some(0); }
            let mut amt = rd_u64(e + 0x618)?.wrapping_mul(rd_u64(d + 0x30)?) / 100 + rd_u64(d + 0x28)?;
            let a450 = rd_u64(e + 0x450)?; if a450 != 0 { amt = amt.wrapping_add(a450.wrapping_mul(rd_u64(me + ENT_MAXHP)?) / 100); }
            let a448 = rd_u64(e + 0x448)?; if a448 != 0 { amt = amt.wrapping_add(a448.wrapping_mul(rd_u64(e + ENT_MAXHP)?) / 100); }
            let mut resist = rd_u64(me + 0x630)?; let pen = rd_u64(e + 0x418)?;
            if pen != 0 { let f = if pen < 0x65 { 100 - pen } else { 0 }; resist = f.wrapping_mul(resist) / 100; }
            let resist = resist.wrapping_add(100); if resist == 0 { return None; }
            let v = amt.wrapping_mul(100) / resist; let v = if v == 0 { 1 } else { v };
            let per = { let x = rd_u64(d + 0x10)?; if x == 0 { 1 } else { x } };
            Some(v.wrapping_mul(tps) / per)
        }
        _ => { dy::unseen(0x180, rva); None }
    }
}
/// 효과 vt+0x88(data, G, ally, self) → 아군 감쇄 기여. 미재현 → unseen(0x188)
unsafe fn eff88_red(rva: usize, d: usize, ally: usize, me: usize, tps: u64) -> Option<u64> {
    match rva {
        EFF_E8_ZERO => Some(0),
        0x1152530 => {
            if rd_u32(me + ENT_KIND) != 13 || rd_u64(me)? != rd_u64(ally)? { return Some(0); }
            if rd_u64(me)? == 0 && rd_u64(me + 8)? != rd_u64(ally + 8)? { return Some(0); }
            if rd_u64(me + ENT_HP)? == 0 { return Some(0); }
            let r = rd_u64(d + 0x18)?; if d2_ee(ally, me)? > r.wrapping_mul(r) { return Some(0); }
            let per = { let v = rd_u64(d + 0x30)?; if v == 0 { 1 } else { v } };
            Some((rd_u64(d + 0x28)?.wrapping_mul(rd_u64(ally + 0x620)?) / 100).wrapping_add(rd_u64(d + 0x20)?).wrapping_mul(tps) / per)
        }
        _ => { dy::unseen(0x188, rva); None }
    }
}
/// 효과 리스트(+0x2f8/+0x300 Box<dyn>, 정렬 보정 없음) vt+slot 합. slot 0x80: (e, self) · 0x88: (ally, self) · 0x90: (self) — 0x90 은 0x9db70 만 재현(unseen 0x190)
unsafe fn effs_sum(ent: usize, slot: usize, other: usize, tps: u64) -> Option<u64> {
    let n = rd_u64(ent + ENT_EFFS_LEN)?; if n == 0 { return Some(0); }
    let p = rd_u64(ent + ENT_EFFS_PTR)? as usize; if !ptr_ok(p) { return None; }
    let mut acc = 0u64;
    for i in 0..n.min(64) as usize {
        let (d, v) = (rd_u64(p + i * 16)? as usize, rd_u64(p + i * 16 + 8)? as usize); let r = dy::impl_rva(v, slot)?;
        let val = match slot {
            0x80 => eff80_dps(r, d, ent, other, tps)?,
            0x88 => eff88_red(r, d, ent, other, tps)?,
            _ => match r { EFF_E8_ZERO => 0, _ => { dy::unseen(0x100 + slot as u32, r); return None; } },
        };
        acc = acc.wrapping_add(val);
    }
    Some(acc)
}
/// 술어 3종(0x129ed50 / 0x128cf70 / 0x129d130): "지금 스킬 k 시전 가능"
unsafe fn pred_skill(e: usize, k: u32) -> Option<bool> {
    let (_, h4, h5, bad) = buff_kinds(e)?; if h4 { return Some(false); }
    let lv = rd_u64(e + ENT_LEVEL)?;
    let (slot, cd_off, prov_off, chg_off, has_slot) = match k {
        1 => (Some(e + SLOT1), ENT_S1_CD, PROV_S1, PROV_S1, true),
        2 => (if lv >= 3 { Some(e + SLOT2) } else { None }, ENT_S2_CD, PROV_S2, if lv < 3 { PROV_5B0 } else { PROV_S2 }, lv >= 3),
        _ => (if lv >= 5 { Some(e + SLOT3) } else { None }, ENT_ULT_CD, PROV_ULT, if lv < 5 { PROV_5B0 } else { PROV_ULT }, lv >= 5),
    };
    let slot_id = match slot { Some(s) => rd_i32(s + 0x30)?, None => -1 };
    if h5 { if let Some(s) = slot { if slot_id != -1 && eff_vt120(rd_u64(s)? as usize, rd_u64(s + 8)? as usize, 0)? { return Some(false); } } }
    if k == 1 && rd_u32(e + ENT_KIND) != 13 { return Some(false); }
    let (pd, pv) = prov(e, prov_off)?; let base = dy::prov90_cooltime(pd, pv, e)?;
    let (cd, cv) = prov(e, chg_off)?; let chg = prov_a8_charges(cd, cv)?; if chg == 0 { return None; }
    let denom = if k == 3 { (rd_i32(e + ENT_400)? as i64).wrapping_add(rd_i32(e + ENT_46C)? as i64).wrapping_add(100) } else { (rd_i32(e + ENT_400)? as i64).wrapping_add(100) };
    let denom = denom.max(1) as u64;
    let eff = (base.wrapping_mul(100) / denom).max(if k == 1 { 3 } else { 1 });
    if eff.wrapping_sub(eff / chg) < rd_u64(e + cd_off)? { return Some(false); }
    if slot_id == -1 { return Some(false); }
    if bad { return Some(false); }
    Some(has_slot)
}
/// 공격간격 = max(3, prov(0x570).vt90*100 / max(1, (i32)e.3fc+100))
unsafe fn atk_interval(e: usize) -> Option<u64> {
    let (pd, pv) = prov(e, PROV_ATK)?; let base = dy::prov90_cooltime(pd, pv, e)?;
    let t = (rd_i32(e + ENT_3FC)? as i64).wrapping_add(100); let t = if t < 2 { 1 } else { t } as u64;
    Some((base.wrapping_mul(100) / t).max(3))
}
/// 0xd958b0: 적측 라인 유닛(kind 1) 의 DPS 합(가중 80/100/60/40 · 100/65/45)
unsafe fn lane_minion_dps(x: usize, me: usize, tps: u64) -> Option<u64> {
    if rd_u8(me) != 0 { return Some(0); }
    let opp = 1u64.wrapping_sub(rd_u64(me + 8)?); if opp > 1 { return None; }
    let (mx, my) = xy(me)?; let meh = rd_u64(me + ENT_HANDLE)?; let mekind = rd_u32(me + ENT_KIND);
    let mut acc = 0u64;
    for base in X_LANE_UNITS {
        let ptr = rd_u64(x + base + (opp as usize) * 0x20)? as usize; let len = rd_u64(x + base + (opp as usize) * 0x20 + 0x18)?;
        if len == 0 { continue; } if !ptr_ok(ptr) { return None; }
        for i in 0..len.min(512) as usize {
            let e = rd_u64(ptr + i * 8)? as usize; if e == 0 { return None; }
            let (ex, ey) = xy(e)?; let d2 = sqd_sat(ex, ey, mx, my);
            if d2 > D2_120K || rd_u32(e + ENT_KIND) != 1 || rd_i32(e + SLOT0 + 0x30)? == -1 { continue; }
            let dmg = estimate_damage(e + SLOT0, e, me, false)?; if dmg == 0 { continue; }
            let ai = atk_interval(e)?; let dps = tps.wrapping_mul(dmg) / ai;
            let mut p1 = 80u64;
            if rd_i32(e + ENT_88)? == 1 { p1 = if rd_u64(e + ENT_90)? == meh { 100 } else if mekind == 13 { 60 } else { 40 }; }
            let p2 = if d2 > D2_90K { if rd_u8(e + ENT_118) != 0 { 65 } else { 45 } } else { 100u64 };
            acc = acc.saturating_add(p1.wrapping_mul(dps).wrapping_mul(p2) / 10000);
        }
    }
    Some(acc)
}

/// 0xeba9b0 본체. a/b = bump Vec {ptr@0, alloc@8, cap@0x10, len@0x18}
pub unsafe fn fight_check(p1: u64, holder: usize, sim: usize, me: usize, a: usize, b: usize) -> Option<u64> {
    let _ = p1;
    if !ptr_ok(holder) || !ptr_ok(sim) || !ptr_ok(me) || !ptr_ok(a) || !ptr_ok(b) { return None; }
    if rd_u8(me + ENT_IMMOBILE) != 0 { return Some(i64::MAX as u64); }
    let x = rd_u64(holder)? as usize; let g = rd_u64(holder + 8)? as usize; if !ptr_ok(x) || !ptr_ok(g) { return None; }
    let w = World { x, data: rd_u64(x)? as usize, vt: rd_u64(x + 8)? as usize }; if !ptr_ok(w.data) || !ptr_ok(w.vt) { return None; }
    let cfg = rd_u64(g + 8)? as usize; if !ptr_ok(cfg) { return None; }
    let meh = rd_u64(me + ENT_HANDLE)?;
    let rec = sim_of_handle(x, meh)?; if rec == 0 { return None; }
    let tps = rd_u64(cfg + CFG_TPS)?; let tick = rd_u64(w.data + W_TICK)?;
    let d = (if tps == 0 { 1 } else { tps }).wrapping_mul(2);
    let q = ((rd_i64(sim + SIM_450)?).wrapping_mul(rd_i64(sim + SIM_218)?)) as u64 / 1000;
    let c = q.min(100); let span0 = 900u64.wrapping_sub(c.wrapping_mul(9)); let mid = 1000u64.wrapping_sub(span0 >> 1); let span = span0 | 1;
    let mut h = (meh << 24) ^ rd_u64(sim + P5_MEMO_KEY)?.wrapping_mul(GOLDEN) ^ (tick / d);
    let role_self = rd_u32(rec + REC_ROLE) as usize;
    let side_self = rd_u64(sim + P5_SIDE)?; if side_self > 1 { return None; }   // 가시성(vt+0xf8) 뷰어 = 인자 sim 의 side
    let side_rec = rd_u64(rec + P5_SIDE)?; if side_rec > 1 { return None; }       // ★opp·아군 로스터·BL = self 엔티티 로스터 sim(d31bb0) 의 side — cat4 는 E2 를 self 로 두고도 부른다(2026-09-06 20:37 DIFF 원인)
    let (mx, my) = xy(me)?;
    let (mut burst, mut dps) = (0u64, 0u64);
    let mut terms = String::new();
    // A 루프
    let (ap, al) = (rd_u64(a)? as usize, rd_u64(a + 0x18)?);
    if al != 0 && !ptr_ok(ap) { return None; }
    for i in 0..al.min(64) as usize {
        let e = rd_u64(ap + i * 8)? as usize; if e == 0 { return None; }
        let es = sim_of_handle(x, rd_u64(e + ENT_HANDLE)?)?; if es == 0 { return None; }
        let side_o = rd_u64(es + P5_SIDE)?; if side_o > 1 { return None; }
        let role_o = rd_u32(es + REC_ROLE) as usize;
        let tval = |k: usize| -> Option<u64> { rd_u64(x + X_MATRIX + (side_o as usize) * MAT_SIDE + role_o * MAT_ROLE + k * MAT_K + role_self * 8) };
        let kind = rd_u32(e + ENT_KIND);
        let compute_atk = can_basic_attack(e)? || match timer_off(kind)? { None => true, Some(off) => rd_u64(e + off)? <= tps };
        let mut best = 0u64; let mut tv = [0u64; 7]; let mut used = [0u8; 7];
        if compute_atk { h = h.wrapping_add(GOLDEN); best = coef(h, mid, span).wrapping_mul(tval(0)?) / 1000; tv[0] = best; used[0] = 1; }
        for (k, cd) in [(1usize, ENT_S1_CD), (2, ENT_S2_CD), (3, ENT_ULT_CD)] {
            if pred_skill(e, k as u32)? || kind != 13 || rd_u64(e + cd)? <= tps { h = h.wrapping_add(GOLDEN); let v = coef(h, mid, span).wrapping_mul(tval(k)?) / 1000; tv[k] = v; used[k] = 1; best = best.max(v); }
        }
        let (h3, h4, h5, _) = buff_kinds(e)?;
        if !h3 { h = h.wrapping_add(GOLDEN); let v = coef(h, mid, span).wrapping_mul(tval(10)?) / 1000; tv[4] = v; used[4] = 1; dps = dps.wrapping_add(v); }
        if !h4 {
            let s1ok = !h5 || rd_i32(e + SLOT1 + 0x30)? == -1 || !eff_vt120(rd_u64(e + SLOT1)? as usize, rd_u64(e + SLOT1 + 8)? as usize, 0)?;
            if s1ok { h = h.wrapping_add(GOLDEN); let v = coef(h, mid, span).wrapping_mul(tval(11)?) / 1000; tv[5] = v; used[5] = 1; dps = dps.wrapping_add(v); }
            let lv = rd_u64(e + ENT_LEVEL)?;
            let s2ok = !h5 || lv < 3 || rd_i32(e + SLOT2 + 0x30)? == -1 || !eff_vt120(rd_u64(e + SLOT2)? as usize, rd_u64(e + SLOT2 + 8)? as usize, 0)?;
            if s2ok { h = h.wrapping_add(GOLDEN); let v = coef(h, mid, span).wrapping_mul(tval(12)?) / 1000; tv[6] = v; used[6] = 1; dps = dps.wrapping_add(v); }
        }
        terms += &format!(" A{}[side={} ro={} rs={} atk={}{} s1={}{} s2={}{} s3={}{} t10={}{} t11={}{} t12={}{} T0..3=({},{},{},{}) T10..12=({},{},{})]", i, side_o, role_o, role_self,
            tv[0], if used[0]==1 {""} else {"x"}, tv[1], if used[1]==1 {""} else {"x"}, tv[2], if used[2]==1 {""} else {"x"}, tv[3], if used[3]==1 {""} else {"x"}, tv[4], if used[4]==1 {""} else {"x"}, tv[5], if used[5]==1 {""} else {"x"}, tv[6], if used[6]==1 {""} else {"x"},
            tval(0).unwrap_or(0), tval(1).unwrap_or(0), tval(2).unwrap_or(0), tval(3).unwrap_or(0), tval(10).unwrap_or(0), tval(11).unwrap_or(0), tval(12).unwrap_or(0));
        burst = burst.wrapping_add(best);
    }
    // B 루프
    let (bp, bl) = (rd_u64(b)? as usize, rd_u64(b + 0x18)?);
    if bl != 0 && !ptr_ok(bp) { return None; }
    for i in 0..bl.min(64) as usize {
        let e = rd_u64(bp + i * 8)? as usize; if e == 0 { return None; }
        if rd_i32(e + SLOT0 + 0x30)? == -1 { return None; }
        let dmg = estimate_damage(e + SLOT0, e, me, false)?;
        let ai = atk_interval(e)?;
        let c1 = coef(h.wrapping_add(GOLDEN), mid, span);
        dps = dps.wrapping_add((tps.wrapping_mul(dmg).wrapping_mul(c1) / 1000) / ai);
        h = h.wrapping_add(GOLDEN.wrapping_mul(2)); let c2 = coef(h, mid, span);
        burst = burst.wrapping_add(dmg.wrapping_mul(c2) / 1000);
    }
    // 적측 유닛 루프 (X+0xf0+opp*0x20)
    let opp = 1 - side_rec;
    let (up, ul) = (rd_u64(x + X_SIDE_UNITS_PTR + (opp as usize) * 0x20)? as usize, rd_u64(x + X_SIDE_UNITS_LEN + (opp as usize) * 0x20)?);
    if ul != 0 && !ptr_ok(up) { return None; }
    let (mut ub, mut ud, mut uc) = (0u64, 0u64, 0u32);
    for i in 0..ul.min(512) as usize {
        let u = rd_u64(up + i * 8)? as usize; if u == 0 { return None; }
        let (ux, uy) = xy(u)?; let d2 = sqd(ux, uy, mx, my);
        if d2 < D2_150K_P1 && w.visible(side_self, rd_u64(u + ENT_HANDLE)?)? && rd_i32(u + SLOT0 + 0x30)? != -1 {
            let dmg = estimate_damage(u + SLOT0, u, me, false)?; let ai = atk_interval(u)?;
            let dd = dmg.wrapping_mul(tps) / ai; dps = dps.wrapping_add(dd); burst = burst.wrapping_add(dmg); ub += dmg; ud += dd; uc += 1;
            terms += &format!(" U{}[k={} dmg={} ai={} dd={}]", i, rd_u32(u + ENT_KIND), dmg, ai, dd);
        }
    }
    terms += &format!(" | units n={} burst={} dps={} ul={}", uc, ub, ud, ul);
    // 효과 리스트
    let mut e80 = 0u64;
    for i in 0..al.min(64) as usize { let e = rd_u64(ap + i * 8)? as usize; let v = effs_sum(e, 0x80, me, tps)?; e80 = e80.wrapping_add(v); dps = dps.wrapping_add(v); }
    terms += &format!(" e80={}", e80);
    TERMS.with(|c| *c.borrow_mut() = terms.clone());
    let mut ally_red = 0u64;
    for i in 0..5usize { let al_e = rd_u64(x + X_ROSTER + (side_rec as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize; if al_e != 0 { ally_red = ally_red.wrapping_add(effs_sum(al_e, 0x88, me, tps)?); } }
    let mut incoming = dps.saturating_sub(ally_red);
    // 꼬리
    let (tag, mdata) = w.mode()?;
    let bl_flag = if tag != 0 || mdata == 0 { true } else { rd_u64(mdata + MODE_240 + (opp as usize) * 8)? == 0 };
    let hp = rd_u64(me + ENT_HP)?; let maxhp = { let m = rd_u64(me + ENT_MAXHP)?; if m == 0 { 1 } else { m } };
    let phase = rd_u8(g + G_PHASE) as u32;
    let call = if phase < 9 && (PHASE_MASK >> phase) & 1 == 1 {
        !bl_flag || (hp.wrapping_mul(100) <= maxhp.wrapping_mul(75) && tick >= rd_u64(cfg + CFG_8A8)?.saturating_sub(tps.wrapping_mul(30)))
    } else { !bl_flag };
    if call { incoming = incoming.wrapping_add(lane_minion_dps(x, me, tps)?); }
    let hp_eff = hp.wrapping_add(effs_sum(me, 0x90, me, tps)?);
    LAST.with(|c| c.set([burst, dps, ally_red, incoming, hp_eff, call as u64, bl_flag as u64, h, mid, span]));
    let num = hp_eff.saturating_sub(burst).wrapping_mul(60);
    Some(num / if incoming == 0 { 1 } else { incoming })
}

// ── 메모 래퍼(0xeb82d0) 미러: 에포크 (seed, tick) + 키
#[derive(Clone, Copy, PartialEq, Eq)]
struct Key { a: [u64; 8], b: [u64; 12], p1: u64, k928: u64, selfh: u64, la: u8, lb: u8 }
thread_local! { pub static LAST: std::cell::Cell<[u64; 10]> = const { std::cell::Cell::new([0; 10]) }; pub static TERMS: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; static MEMO: std::cell::RefCell<((u64, u64), Vec<(Key, u64)>)> = const { std::cell::RefCell::new(((0, 0), Vec::new())) }; }
pub fn memo_reset() { MEMO.with(|c| { let mut m = c.borrow_mut(); m.0 = (0, 0); m.1.clear(); }); }
/// eb82d0(p1, _, holder, sim, self, &A, &B) 계약 그대로. lenA<9 && lenB<13 이면 메모, 아니면 직접 계산.
pub unsafe fn fight_check_memo(p1: u64, holder: usize, sim: usize, me: usize, a: usize, b: usize) -> Option<u64> {
    if !ptr_ok(holder) || !ptr_ok(a) || !ptr_ok(b) || !ptr_ok(sim) || !ptr_ok(me) { return None; }
    let la = rd_u64(a + 0x18)?; let lb = rd_u64(b + 0x18)?;
    if !(la < 9 && lb < 13) { return fight_check(p1, holder, sim, me, a, b); }
    let x = rd_u64(holder)? as usize; if !ptr_ok(x) { return None; }
    let wd = rd_u64(x)? as usize; if !ptr_ok(wd) { return None; }
    let epoch = (rd_u64(wd + W_SEED_EC90)?, rd_u64(wd + W_TICK)?);
    let mut key = Key { a: [0; 8], b: [0; 12], p1, k928: rd_u64(sim + P5_MEMO_KEY)?, selfh: rd_u64(me + ENT_HANDLE)?, la: la as u8, lb: lb as u8 };
    let ap = rd_u64(a)? as usize; for i in 0..la as usize { let e = rd_u64(ap + i * 8)? as usize; key.a[i] = rd_u64(e + ENT_HANDLE)?; }
    let bp = rd_u64(b)? as usize; for i in 0..lb as usize { let e = rd_u64(bp + i * 8)? as usize; key.b[i] = rd_u64(e + ENT_HANDLE)?; }
    let hit = MEMO.with(|c| { let mut m = c.borrow_mut(); if m.0 != epoch { m.0 = epoch; m.1.clear(); } m.1.iter().find(|e| e.0 == key).map(|e| e.1) });
    if let Some(v) = hit { TERMS.with(|c| *c.borrow_mut() = " (memo hit — terms stale)".into()); return Some(v); }
    let v = fight_check(p1, holder, sim, me, a, b)?;
    MEMO.with(|c| { let mut m = c.borrow_mut(); if m.1.len() < 1024 { m.1.push((key, v)); } });
    Some(v)
}
/// 진단 문자열(DIFF/NA 줄)
pub unsafe fn diag(holder: usize, sim: usize, me: usize, a: usize, b: usize) -> String {
    if !ptr_ok(holder) || !ptr_ok(a) || !ptr_ok(b) || !ptr_ok(me) || !ptr_ok(sim) { return "bad ptr".into(); }
    let la = rd_u64(a + 0x18).unwrap_or(99); let lb = rd_u64(b + 0x18).unwrap_or(99);
    let x = rd_u64(holder).unwrap_or(0) as usize; let g = rd_u64(holder + 8).unwrap_or(0) as usize;
    let cfg = if ptr_ok(g) { rd_u64(g + 8).unwrap_or(0) as usize } else { 0 };
    let tps = if ptr_ok(cfg) { rd_u64(cfg + CFG_TPS).unwrap_or(0) } else { 0 };
    let recside = { let xx = rd_u64(holder).unwrap_or(0) as usize; if ptr_ok(xx) { sim_of_handle(xx, rd_u64(me + ENT_HANDLE).unwrap_or(0)).map(|r| if r != 0 { rd_u64(r + P5_SIDE).unwrap_or(9) } else { 8 }) } else { None } };
    let mut s = format!("recside={:?} la={} lb={} imm={} hp={:?} maxhp={:?} tps={} phase={} side={:?} 450={:?} 218={:?}", recside, la, lb, rd_u8(me + ENT_IMMOBILE), rd_u64(me + ENT_HP), rd_u64(me + ENT_MAXHP), tps, if ptr_ok(g) { rd_u8(g + G_PHASE) } else { 255 }, rd_u64(sim + P5_SIDE), rd_i64(sim + SIM_450), rd_i64(sim + SIM_218));
    let ap = rd_u64(a).unwrap_or(0) as usize;
    for i in 0..la.min(8) as usize {
        let e = rd_u64(ap + i * 8).unwrap_or(0) as usize; if !ptr_ok(e) { break; }
        let (h3, h4, h5, bad) = buff_kinds(e).unwrap_or((false, false, false, false));
        s += &format!(" A{}=[k={} b0={:?} b8={:?} c0={:?} c8={:?} lv={:?} buffs={}{}{}{} cc90={:?} p1={:?} p2={:?} p3={:?} effs={:?}]", i, rd_u32(e + ENT_KIND), rd_u64(e + 0xb0), rd_u64(e + 0xb8), rd_u64(e + 0xc0), rd_u64(e + 0xc8), rd_u64(e + ENT_LEVEL),
            h3 as u8, h4 as u8, h5 as u8, bad as u8, can_basic_attack(e), pred_skill(e, 1), pred_skill(e, 2), pred_skill(e, 3), rd_u64(e + ENT_EFFS_LEN));
    }
    let l = LAST.with(|c| c.get());
    s += &format!(" | mine: burst={} dps={} ally_red={} incoming={} hp_eff={} call={} bl={} h={:#x} mid={} span={} |{}", l[0], l[1], l[2], l[3], l[4], l[5], l[6], l[7], l[8], l[9], TERMS.with(|c| c.borrow().clone()));
    let _ = x; s
}
