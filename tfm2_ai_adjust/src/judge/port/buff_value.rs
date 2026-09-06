//! buff_value — combat_score `0xd5bbf0` 의 S13(자기 버프)·S14(아군 버프) 경로 콜리 포팅.
//!   정본 RE = `RE\2026-09-07_buff_value-0xdffa10-전수해독-0.5.8.md`
//!           · `RE\2026-09-07_BuffSpec레이아웃-0xe047c0-0xe03360-0xe02540-전수해독-0.5.8.md`
//!           · `RE\2026-09-07_combat_score-S13S14-버프콜리4종-0xe03ed0-0xe02bc0-0xe01c40-0xe022d0-0.5.8.md`
//!   순서: 순수 함수(e022d0 · e02540)부터. dyn 슬롯이 필요한 것은 `unseen` 으로 impl 을 모아 단계적으로 연다.
#![allow(dead_code)]
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::action_score::sim_of_handle;

pub const SPEC_SIZE: usize = 0x120;
const BV_ENEMY_PTR: usize = 0x14d8; const BV_ENEMY_LEN: usize = 0x14f0;

// ── 공통 관용구 ───────────────────────────────────────────────────────────────────────────
/// dyn fat-ptr `{[0]=Arc 데이터, [8]=vtable}` 의 inline self = `data + ((vt[0x10]-1) & !0xF) + 0x10`
#[inline]
pub unsafe fn inline_self(data: usize, vt: usize) -> Option<usize> {
    let align = rd_u64(vt + 0x10)?;
    Some(data.wrapping_add((align.wrapping_sub(1) & !0xFu64) as usize).wrapping_add(0x10))
}
/// `r(e) = e.0x470==0 ? e.0x680 : ((e.0x470 + 100) * e.0x680) / 100`  (부호없는 나눗셈)
#[inline]
pub unsafe fn body_radius(e: usize) -> Option<u64> {
    let p = rd_i32(e + 0x470)? as i64; let r = rd_u64(e + 0x680)?;
    Some(if p == 0 { r } else { (((p + 100) as u64).wrapping_mul(r)) / 100 })
}
/// `reach(e) = e.0x4c0 == -1 ? 0 : e.0x438 + e.0x4a0 + (e.0x5c8 - 1) * e.0x4a8`
#[inline]
pub unsafe fn skill_reach(e: usize) -> Option<u64> {
    if rd_i32(e + 0x4c0)? == -1 { return Some(0); }
    Some(rd_u64(e + 0x438)?
        .wrapping_add(rd_u64(e + 0x4a0)?)
        .wrapping_add(rd_u64(e + 0x5c8)?.wrapping_sub(1).wrapping_mul(rd_u64(e + 0x4a8)?)))
}
#[inline] fn absd(a: u64, b: u64) -> u64 { if a >= b { a - b } else { b - a } }
#[inline] unsafe fn d2(a: usize, b: usize) -> Option<u64> {
    let (ax, ay) = (rd_u64(a + ENT_X)?, rd_u64(a + ENT_Y)?);
    let (bx, by) = (rd_u64(b + ENT_X)?, rd_u64(b + ENT_Y)?);
    let (dx, dy) = (absd(ax, bx), absd(ay, by));
    Some(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)))
}
/// 60000² + 1 = 3,600,000,001 (판정은 `dist <= 60000`)
pub const D2_60K: u64 = 0xd693_a401;
/// 120000² + 1 = 14,400,000,001
pub const D2_120K: u64 = 0x3_5a4e_9001;

// ── 0xe022d0 — 근접 아군 수 배수 (611B, 완전 순수) ────────────────────────────────────────
/// `(mode 미사용, slot, ctx, rec, self, value) -> i64`
/// `value <= 0` 이거나 `slot.vt+0xe0(inline)==false` 면 그대로. 아니면 60000 이내 아군 수 n 에 대해
/// `value + ((min(max(n,1),3) - 1) * value) / 2`  (n=1/2/3+ → 1.0 / 1.5 / 2.0배)
pub unsafe fn e022d0(slot: usize, ctx: usize, rec: usize, me: usize, value: i64) -> Option<i64> {
    if value <= 0 { return Some(value); }
    let (sd, sv) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    if !slot_flag_e0(sd, sv)? { return Some(value); }
    let side = rd_u64(rec + 0x930)?; if side > 1 { return None; }
    let w = rd_u64(ctx)? as usize; if !ptr_ok(w) { return None; }
    let lanes = w + 0x1e0 + (side as usize) * 0x28;
    let mut n: i64 = 0;
    for k in 0..5usize {
        let e = rd_u64(lanes + k * 8)? as usize; if e == 0 { continue; }
        if d2(e, me)? < D2_60K { n += 1; }
    }
    let m = n.max(1).min(3);
    Some(value.wrapping_add((m - 1).wrapping_mul(value) / 2))
}
/// slot.vt+0xe0 → bool. `0x9db70` = false · EFF_TRUE = true · 그 외는 기계어 디코드, 실패 시 unseen.
unsafe fn slot_flag_e0(data: usize, vt: usize) -> Option<bool> {
    let f = rd_u64(vt + 0xe0)? as usize;          // ★절대주소(impl_rva 는 RVA 라 decode_getter 에 넣으면 안 된다)
    let p = inline_self(data, vt)?;
    if let Some(v) = super::as_callees::decode_getter(f, p) { return Some(v != 0); }
    if let Some(r) = super::dyn_eff::impl_rva(vt, 0xe0) { super::dyn_eff::unseen(0x8e0, r); }
    None
}

// ── 0xe02540 — BuffSpec → 점수 (1,658B, 완전 순수) ────────────────────────────────────────
/// `(ctx, spec, self, st) -> i64`. st 는 `0xe03360` 의 EAX 바이트(0/1/2).
pub unsafe fn e02540(ctx: usize, spec: usize, me: usize, st: u8) -> Option<i64> {
    let a = rd_i64(me + 0x618)?;            // attack
    let mp = rd_i64(me + 0x620)?;           // magic_power
    let hmax = rd_i64(me + 0x628)?;         // maxHP
    let w = rd_u64(ctx)? as usize; if !ptr_ok(w) { return None; }

    // 물리 피해 비중 ratio (per-mille), 기본 500
    let mut ratio: i64 = 500;
    let rec = sim_of_handle(w, rd_u64(me + ENT_HANDLE)?)?;
    if rec != 0 {
        let side = rd_u64(rec + 0x930)?; if side > 1 { return None; }
        let role = rd_u32(rec + 0x9c0) as usize;
        let b = w + (side as usize) * 0xfa0 + role * 0x320;
        let mut phys: i64 = 0;
        for k in 0..5usize { phys = phys.wrapping_add(rd_i64(b + 0x410 + k * 8)?); }
        let mut total = phys;
        for k in 0..15usize { total = total.wrapping_add(rd_i64(b + 0x438 + k * 8)?); }
        if total != 0 { ratio = phys.wrapping_mul(1000).wrapping_div(total); }
    }

    let s32 = |o: usize| -> Option<i32> { rd_i32(spec + o) };
    let s64 = |o: usize| -> Option<i64> { rd_i64(spec + o) };
    let mut acc: i64 = 0;
    if s32(0x5c)? > 0 { acc = ((s32(0x5c)? as i64).wrapping_mul(a) / 100).min(40); }
    if s32(0x8c)? > 0 { acc += (((s32(0x8c)? as i64).wrapping_mul(a) / 200).wrapping_mul(ratio) / 500).min(40); }
    acc += s32(0x58)?.clamp(0, 30) as i64;
    acc += s32(0x60)?.clamp(0, 30) as i64;
    if s32(0x64)? > 0 { acc += (mp.wrapping_mul(s32(0x64)? as i64) / 100).min(40); }
    if s32(0x104)? > 0 { acc += ((((s32(0x104)? as u32 / 5) as i64).wrapping_mul(ratio)) / 500).min(20); }
    if s64(0xa8)? != 0 { acc += (s64(0xa8)? / 3).min(20); }
    if s64(0xb0)? != 0 { acc += (s64(0xb0)? / 3).min(20); }
    if s64(0xc8)? != 0 {
        let x = { let t = s64(0xc8)? / 3000; if t < 2 { 1 } else { t } };
        let y = { let t = a / 50;            if t < 2 { 1 } else { t } };
        acc += x.wrapping_mul(y).min(25);
    }
    if s32(0x90)? > 0 && rd_u8(me) == 0 {
        let team = rd_u64(me + 8)?; if team > 1 { return None; }
        let base = w + 0x1e0 + (team as usize) * 0x28;
        let mut cnt = 0i64;
        for k in 0..5usize { let e = rd_u64(base + k * 8)? as usize; if e != 0 && d2(e, me)? < D2_60K { cnt += 1; } }
        if cnt > 1 { acc += ((1000 - ratio).wrapping_mul((s32(0x90)? as u32 / 5) as i64) / 500).min(15); }
    }
    if s32(0x80)? > 0 && (st != 0 || rd_i64(me + ENT_HP)? < hmax) {
        acc += (ratio.wrapping_mul((s32(0x80)? as u32 / 5) as i64) / 500).min(15);
    }
    Some((acc / 2).min(40))
}

// ── S13/S14 골격 ─────────────────────────────────────────────────────────────────────────
//   정본 = `RE\2026-09-07_combat_score-S13S14-본체구간-정밀전사-0.5.8.md`
//   미포팅 지점은 `na("태그")` 로 도달 수만 집계하고 None(=NA) — 리플레이 1판이면 어느 조각이 실제 벽인지 나온다.
use super::combat_score::na_tag;

pub struct BCtx {
    pub mode: usize, pub prof: usize, pub rec: usize, pub ctx: usize, pub bb: usize,
    pub sp: usize, pub slot: usize, pub tgt: usize, pub me: usize, pub p9: usize,
    pub sim: usize, pub w: usize,
    /// S5 의 퍼센타일 계수 C (자기 기준)
    pub c: i64,
    /// `[rbp+0x7d8]` 초기값 = bonus9b0 + thr_s
    pub inc_base: i64,
    /// `sp.vt[0x80]()` = 시전 지연(위협 합산 임계에 +30 해서 쓴다)
    pub cast_delay: u64,
}

/// slot fat-ptr → (data, vt, inline)
#[inline] unsafe fn slot3(slot: usize) -> Option<(usize, usize, usize)> {
    let (d, v) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    if !ptr_ok(d) || !ptr_ok(v) { return None; }
    Some((d, v, inline_self(d, v)?))
}

/// **합성 impl 판별**(자식 순회 후 합산). 실측 6종이 바이트 골격까지 같아 하나로 처리한다:
///   `0x12a5660`(vt40) `0x12a6a90`(vt48) `0x12a6ee0`(vt b0) `0x12a5ae0` `0x12a4f30` `0x1248040`.
///   형태: `mov rXX,[rcx+LEN]` · `mov rYY,[rcx+PTR]` · `add rYY,8` · 루프에서 `call [r?+SLOT]` · `add rYY,STRIDE`.
///   반환 = (len_off, ptr_off, stride, child_slot).
unsafe fn composite_shape(f: usize) -> Option<(usize, usize, usize, usize)> {
    if !ptr_ok(f) { return None; }
    let mut offs: [usize; 2] = [usize::MAX; 2]; let mut no = 0usize;
    let (mut slot, mut stride) = (usize::MAX, usize::MAX);
    let mut i = 0usize;
    while i < 0x60 {
        let a = f + i;
        // 4c 8b <mod=01, rm=001(rcx)> disp8  → mov r8..r15, [rcx+disp8]
        if rd_u8(a) == 0x4c && rd_u8(a + 1) == 0x8b && (rd_u8(a + 2) & 0xC7) == 0x41 && no < 2 {
            offs[no] = rd_u8(a + 3) as usize; no += 1; i += 4; continue;
        }
        // 41 ff 5? disp8 → call qword ptr [r8..r15 + disp8]
        if rd_u8(a) == 0x41 && rd_u8(a + 1) == 0xff && (rd_u8(a + 2) & 0xF8) == 0x50 && slot == usize::MAX {
            slot = rd_u8(a + 3) as usize; i += 4; continue;
        }
        // 41 ff 9? disp32 → call qword ptr [r8..r15 + disp32]  (슬롯 ≥ 0x80 은 disp8 로 표현 불가라 이 형태다)
        if rd_u8(a) == 0x41 && rd_u8(a + 1) == 0xff && (rd_u8(a + 2) & 0xF8) == 0x90 && slot == usize::MAX {
            slot = rd_i32(a + 3)? as usize; i += 7; continue;
        }
        // 49 83 c? imm8 → add r8..r15, imm8  (호출 뒤의 것이 stride)
        if slot != usize::MAX && rd_u8(a) == 0x49 && rd_u8(a + 1) == 0x83 && (rd_u8(a + 2) & 0xF8) == 0xC0 {
            stride = rd_u8(a + 3) as usize; break;
        }
        i += 1;
    }
    // ★루프가 2개 이상이면(= `call [r?+S]` 2회 이상) 거부한다. 첫 루프만 합산해 **조용히 틀리는 것**을 막기 위함이다
    //   (0x122e360/0x122e560 은 두 리스트를 더한다 — 스캐너가 첫 리스트만 보면 값이 절반이 된다).
    let mut calls = 0usize;
    for k in 0..0x120usize {
        let a = f + k;
        // int3 3연속 = 함수 끝 패딩 → 다음 함수의 call 을 세지 않도록 여기서 멈춘다
        if rd_u8(a) == 0xcc && rd_u8(a + 1) == 0xcc && rd_u8(a + 2) == 0xcc { break; }
        if rd_u8(a) == 0x41 && rd_u8(a + 1) == 0xff && ((rd_u8(a + 2) & 0xF8) == 0x50 || (rd_u8(a + 2) & 0xF8) == 0x90) { calls += 1; }
    }
    if calls != 1 { return None; }
    if no == 2 && slot != usize::MAX && stride != usize::MAX && (stride == 0x10 || stride == 0x18) {
        Some((offs[0], offs[1], stride, slot))
    } else { None }
}
/// **레벨 게이트형 위임**: `call [r9+0x48]`(= EST 서술자 → self.0x5c8 = 레벨) 이 3 이상이면 자식 (p+0x10, p+0x18),
///   아니면 (p+0, p+8) 로 같은 트레이트를 그대로 위임한다. 넘길 슬롯은 `mov r10,[rcx+SLOT]` 에서 뽑는다.
///   실측: `0x164ea30`(→0x40) · `0x164eb10`(→0x48).
unsafe fn level_delegate(f: usize) -> Option<usize> {
    if !ptr_ok(f) { return None; }
    let (mut c48, mut cmp3) = (false, false);
    for i in 0..0x50usize {
        let a = f + i;
        if rd_u8(a) == 0x41 && rd_u8(a + 1) == 0xff && rd_u8(a + 2) == 0x51 && rd_u8(a + 3) == 0x48 { c48 = true; }
        if rd_u8(a) == 0x48 && rd_u8(a + 1) == 0x83 && rd_u8(a + 2) == 0xf8 && rd_u8(a + 3) == 0x03 { cmp3 = true; }
        if c48 && cmp3 && rd_u8(a) == 0x4c && rd_u8(a + 1) == 0x8b && rd_u8(a + 2) == 0x51 { return Some(rd_u8(a + 3) as usize); }
    }
    None
}

/// EST 서술자(`0x1433da3d0`)의 `+0x38` = `lea rax,[rcx+0x618]` — 즉 **자기 스탯 블록**(0x618 AD · 0x620 AP · 0x628 maxHP).
#[inline] unsafe fn stat_at(me: usize, k: usize) -> Option<u64> { rd_u64(me + 0x618 + k * 8) }
/// 잎 impl: `flat + Σ spec.K[i] * stat[i] / 100` (무부호). 실측 `0x1147260`(shield) · `0x117ca10`(heal).
unsafe fn leaf_stat_scaled(rva: usize, p: usize, me: usize) -> Option<i64> {
    let (flat, ks): (usize, &[usize]) = match rva {
        0x1147260 => (0x138, &[0x140, 0x148, 0x150]),
        0x117ca10 => (0x28,  &[0x18, 0x20]),
        _ => return None,
    };
    let mut acc = rd_u64(p + flat)?;
    for (i, k) in ks.iter().enumerate() {
        let v = rd_u64(p + *k)?.wrapping_mul(stat_at(me, i)?);
        acc = acc.wrapping_add(v / 100);
    }
    Some(acc as i64)
}

/// 자식 배열 하나를 돌며 같은 슬롯을 합산
unsafe fn sum_list(p: usize, ptr_o: usize, len_o: usize, stride: usize, cs: usize, me: usize, depth: u32, tag: &str) -> Option<i64> {
    let n = rd_u64(p + len_o)?; if n == 0 { return Some(0); }
    let arr = rd_u64(p + ptr_o)? as usize; if !ptr_ok(arr) { return None; }
    let mut acc: i64 = 0;
    for i in 0..n.min(64) as usize {
        let e = arr + i * stride;
        let (cd, cv) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
        if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
        acc = acc.wrapping_add(slot_sum(cd, cv, cs, me, depth + 1, tag)?);
    }
    Some(acc)
}
/// dyn 슬롯 값(i64). 합성이면 자식 합, 잎이면 디코드/스탯식, 그 외엔 unseen 기록 후 NA.
pub unsafe fn slot_sum(data: usize, vt: usize, slot: usize, me: usize, depth: u32, tag: &str) -> Option<i64> {
    if depth > 8 { return na_tag(tag); }
    let f = rd_u64(vt + slot)? as usize;          // ★절대주소(impl_rva 는 RVA)
    let p = inline_self(data, vt)?;
    // ── 골격 스캐너로 안 잡히는 변종 3종(실측 vt+0x40) ─────────────────────────────────
    let eb = crate::exe_base();
    if eb != 0 && f > eb {
        match f - eb {
            // Σlist(p+0x68/len p+0x70/stride 0x18) + (p+0x78 / max(1,p+0x80)) * Σlist(p+0x50/len p+0x58/stride 0x18)
            0x16a3190 => {
                let s1 = sum_list(p, 0x68, 0x70, 0x18, 0x40, me, depth, tag)?;
                let s2 = sum_list(p, 0x50, 0x58, 0x18, 0x40, me, depth, tag)?;
                let a = rd_u64(p + 0x78)?; let mut b = rd_u64(p + 0x80)?; if b == 0 { b = 1; }
                return Some(((a / b) as i64).wrapping_mul(s2).wrapping_add(s1));
            }
            // Σlist(p+0x50/len p+0x58/stride 0x18) + Σlist(p+0x68/len p+0x70/stride 0x10)
            0x122e360 => {
                let s1 = sum_list(p, 0x50, 0x58, 0x18, 0x40, me, depth, tag)?;
                let s2 = sum_list(p, 0x68, 0x70, 0x10, 0x40, me, depth, tag)?;
                return Some(s1.wrapping_add(s2));
            }
            // 같은 두-리스트 형의 vt+0x48 판(자식 슬롯만 0x48)
            0x122e560 => {
                let s1 = sum_list(p, 0x50, 0x58, 0x18, 0x48, me, depth, tag)?;
                let s2 = sum_list(p, 0x68, 0x70, 0x10, 0x48, me, depth, tag)?;
                return Some(s1.wrapping_add(s2));
            }
            _ => {}
        }
    }
    if let Some(cs) = level_delegate(f) {
        let o = if rd_u64(me + 0x5c8)? >= 3 { 0x10usize } else { 0 };
        let (cd, cv) = (rd_u64(p + o)? as usize, rd_u64(p + o + 8)? as usize);
        if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
        return slot_sum(cd, cv, cs, me, depth + 1, tag);
    }
    if let Some((len_o, ptr_o, stride, cs)) = composite_shape(f) {
        let n = rd_u64(p + len_o)?; if n == 0 { return Some(0); }
        let arr = rd_u64(p + ptr_o)? as usize; if !ptr_ok(arr) { return None; }
        let mut acc: i64 = 0;
        for i in 0..n.min(64) as usize {
            let e = arr + i * stride;
            let (cd, cv) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
            acc = acc.wrapping_add(slot_sum(cd, cv, cs, me, depth + 1, tag)?);
        }
        return Some(acc);
    }
    if let Some(v) = super::as_callees::decode_getter(f, p) { return Some(v as i64); }
    let base = crate::exe_base();
    if base != 0 && f > base { if let Some(v) = leaf_stat_scaled(f - base, p, me) { return Some(v); } }
    if let Some(r) = super::dyn_eff::impl_rva(vt, slot) { super::dyn_eff::unseen(0x900 + slot as u32, r); }
    na_tag(tag)
}

/// S13(자기 버프) · S14(아군 버프). `ally` = 아군 Record(S14) / None(S13).
/// 아직 `0xe047c0`·`0xdffa10` 등이 미포팅이라 최종값은 NA — 지금은 **어느 조각이 벽인지** 집계가 목적이다.
pub unsafe fn s13_s14(b: &BCtx, ally: Option<usize>) -> Option<i64> {
    let (_sd, sv, sin) = slot3(b.slot)?;
    let t = b.tgt;
    let (maxhp, hp) = (rd_u64(t + ENT_MAXHP)?, rd_u64(t + ENT_HP)?);
    let missing_raw = maxhp.wrapping_sub(hp);
    let heal0 = if hp <= maxhp { missing_raw } else { 0 };

    // slot.vt+0x40 / +0x48 : (inline, sim, self, EST) -> i64
    let heal_cap = slot_sum(_sd, sv, 0x40, b.me, 0, "B40")?;
    let heal = heal0.min(heal_cap.max(0) as u64) as i64;
    let shield = slot_sum(_sd, sv, 0x48, b.me, 0, "B48")?;
    // slot.vt+0xb0 : (inline, sim, self) -> i64 아우라
    let aura = slot_sum(_sd, sv, 0xb0, b.me, 0, "Bb0")?;

    // inc / heal_e / shield_e — S13 은 bb 필드, S14 는 아군 Record 필드에서 온다
    let (inc, shield_ok, miss) = if let Some(ra) = ally {
        let x = super::action_score::threat_sum(ra, b.cast_delay.wrapping_add(30))?.wrapping_add(rd_i64(ra + 0x98)?);
        let (r14, r10) = (rd_i64(ra + 0x88)?, rd_i64(ra + 0x70)?);
        let mut so = if r14 == 0 { 0 } else { shield };
        if r10 > 0 || x > 0 { so = shield; }
        (r10.wrapping_add(r14).wrapping_add(x), so, (missing_raw as i64).max(0))
    } else {
        let (g9a0, g988) = (rd_i64(b.bb + 0x9a0)?, rd_i64(b.bb + 0x988)?);
        let mut so = if (g9a0 | g988) == 0 { 0 } else { shield };
        if b.inc_base > 0 { so = shield; }
        let inc = b.inc_base.wrapping_add(g988).wrapping_add(g9a0);
        let miss = (rd_i64(b.me + ENT_MAXHP)? - rd_i64(b.me + ENT_HP)?).max(0);
        (inc, so, miss)
    };
    let heal_e = heal.min(miss.wrapping_add(2 * inc));
    let shield_e = shield_ok.min(3 * inc);
    let _ = (heal_e, shield_e, aura, sin, b.mode, b.prof, b.rec, b.ctx, b.sp, b.me, b.p9, b.sim, b.w, b.c);

    let spec0 = e047c0(b.slot, b.ctx, t)?;
    // has = spec0 있음 ∨ slot.vt+0xa0(sret BuffSpec) 의 tag != −1  ·  b90 = slot.vt+0x90(bool)
    let _has = match spec0 { Some(_) => true, None => { sret_spec_tag(_sd, sv, b.me, "Ba0")? != -1 } };
    let _b90 = slot_bool90(_sd, sv, 0)?;
    // 조립 항 중 이미 옮긴 것들(값은 아직 안 쓰지만 도달·NA 집계로 검증 순서를 잡는다)
    let _aoe = e02bc0(b.slot, b.ctx, b.bb, b.me, if ally.is_some() { t } else { b.me },
                      rd_u64(if ally.is_some() { t } else { b.me } + ENT_HANDLE)?, b.cast_delay)?;
    if ally.is_none() { let _trig = e03ed0(b.slot, b.ctx, b.rec, b.bb, b.me, b.c)?; }
    // ⬜남은 미포팅: 0xdffa10(버프가치) → 0xe022d0 → 0xe01c40 → 0xe03360/0xe02540
    na_tag(if ally.is_some() { "B14spec" } else { "B13spec" })
}

// ── slot.vt+0xa0 : sret BuffSpec(0x120) ─────────────────────────────────────────────────
//   정본 = `RE\2026-09-07_BuffSpec-fold-0x126e6c0-0.5.8.md`
//   `0x126e6c0` = 자식들의 BuffSpec 을 접는다: **첫 유효(=+0x48 != −1) 자식을 통째로 채택**하고,
//   이후 유효 자식은 아래 오프셋 집합만 wrapping 합(i32/i64) 또는 논리합(bool). 태그·페이로드는 버린다.
const FOLD_I32: [usize; 18] = [0x58, 0x5c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x90, 0xfc, 0x100, 0x104];
const FOLD_I64: [usize; 14] = [0x98, 0xa0, 0xa8, 0xb0, 0xb8, 0xc0, 0xc8, 0xd0, 0xd8, 0xe0, 0xe8, 0xf0, 0x108, 0x110];
const FOLD_BOOL: [usize; 3] = [0xf8, 0x118, 0x119];

#[inline] fn sp_i32(b: &[u8; SPEC_SIZE], o: usize) -> i32 { i32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) }
#[inline] fn sp_set_i32(b: &mut [u8; SPEC_SIZE], o: usize, v: i32) { b[o..o + 4].copy_from_slice(&v.to_le_bytes()); }
#[inline] fn sp_i64(b: &[u8; SPEC_SIZE], o: usize) -> i64 { let mut a = [0u8; 8]; a.copy_from_slice(&b[o..o + 8]); i64::from_le_bytes(a) }
#[inline] fn sp_set_i64(b: &mut [u8; SPEC_SIZE], o: usize, v: i64) { b[o..o + 8].copy_from_slice(&v.to_le_bytes()); }

/// 자식 배열 (ptr@p+ptr_o, len@p+len_o, stride) 을 `0x126e6c0` 규칙으로 접는다.
unsafe fn fold_children(p: usize, ptr_o: usize, len_o: usize, stride: usize, me: usize, depth: u32) -> Option<Option<[u8; SPEC_SIZE]>> {
    let n = rd_u64(p + len_o)?;
    let arr = rd_u64(p + ptr_o)? as usize;
    if n == 0 { return Some(None); }
    if !ptr_ok(arr) { return None; }
    let mut acc: Option<[u8; SPEC_SIZE]> = None;
    for i in 0..n.min(64) as usize {
        let e = arr + i * stride;
        let (cd, cv) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
        if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
        let t = match spec_a0(cd, cv, me, depth + 1)? { Some(t) => t, None => continue };
        match acc.as_mut() {
            None => acc = Some(t),
            Some(a) => {
                for o in FOLD_I32 { sp_set_i32(a, o, sp_i32(a, o).wrapping_add(sp_i32(&t, o))); }
                for o in FOLD_I64 { sp_set_i64(a, o, sp_i64(a, o).wrapping_add(sp_i64(&t, o))); }
                for o in FOLD_BOOL { a[o] |= t[o]; }
            }
        }
    }
    Some(acc)
}

/// slot.vt+0x90 : bool. 실측 최빈 impl `0x12a71e0` = 자식(ptr p+8 / len p+0x10 / stride 0x10) 중 **하나라도 true**.
pub unsafe fn slot_bool90(data: usize, vt: usize, depth: u32) -> Option<bool> {
    if depth > 8 { return na_tag("B90d").map(|_| false); }
    let f = rd_u64(vt + 0x90)? as usize;
    let p = inline_self(data, vt)?;
    let eb = crate::exe_base(); if eb == 0 || f <= eb { return None; }
    if f - eb == 0x12a71e0 {
        let n = rd_u64(p + 0x10)?; if n == 0 { return Some(false); }
        let arr = rd_u64(p + 8)? as usize; if !ptr_ok(arr) { return None; }
        for i in 0..n.min(64) as usize {
            let e = arr + i * 0x10;
            let (cd, cv) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
            if slot_bool90(cd, cv, depth + 1)? { return Some(true); }
        }
        return Some(false);
    }
    if let Some(v) = super::as_callees::decode_getter(f, p) { return Some(v & 1 == 1); }
    if let Some((da, db, sl)) = super::as_callees::delegate_pair(f) {
        if sl == 0x90 {
            let (cd, cv) = (rd_u64(p + da)? as usize, rd_u64(p + db)? as usize);
            if ptr_ok(cd) && ptr_ok(cv) { return slot_bool90(cd, cv, depth + 1); }
        }
    }
    if let Some(r) = super::dyn_eff::impl_rva(vt, 0x90) { super::dyn_eff::unseen(0x990, r); }
    na_tag("B90").map(|_| false)
}
/// `slot.vt+0xa0(sret, inline, sim, self, EST)` 재현. `Some(None)` = tag −1(버프 없음).
pub unsafe fn spec_a0(data: usize, vt: usize, me: usize, depth: u32) -> Option<Option<[u8; SPEC_SIZE]>> {
    if depth > 8 { return na_tag("Ba0d").map(|_| None); }
    let f = rd_u64(vt + 0xa0)? as usize;
    let p = inline_self(data, vt)?;
    let eb = crate::exe_base(); if eb == 0 || f <= eb { return None; }
    match f - eb {
        0x109baa0 => Some(None),                                                  // 기본 impl: +0x48 = −1
        // inline+0x120 플래그가 서면 없음, 아니면 inline 에 저장된 spec 을 그대로 복사
        0x11507c0 => {
            if rd_u8(p + 0x120) != 0 { return Some(None); }
            let mut b = [0u8; SPEC_SIZE]; for i in 0..SPEC_SIZE { b[i] = rd_u8(p + i); }
            if i32::from_le_bytes([b[0x48], b[0x49], b[0x4a], b[0x4b]]) == -1 { return Some(None); }
            Some(Some(b))
        }
        // 두 리스트(둘 다 stride 0x18)를 같은 규칙으로 이어서 접는다(0x126f800)
        0x16a33f0 => {
            let a1 = fold_children(p, 0x50, 0x58, 0x18, me, depth)?;
            let a2 = fold_children(p, 0x68, 0x70, 0x18, me, depth)?;
            Some(match (a1, a2) {
                (None, x) => x,
                (Some(x), None) => Some(x),
                (Some(mut x), Some(y)) => {
                    for o in FOLD_I32 { let v = sp_i32(&x, o).wrapping_add(sp_i32(&y, o)); sp_set_i32(&mut x, o, v); }
                    for o in FOLD_I64 { let v = sp_i64(&x, o).wrapping_add(sp_i64(&y, o)); sp_set_i64(&mut x, o, v); }
                    for o in FOLD_BOOL { x[o] |= y[o]; }
                    Some(x)
                }
            })
        }
        0x12a5770 | 0x13be350 => fold_children(p, 8, 0x10, 0x10, me, depth),      // Vec<Arc<dyn>> {cap,ptr,len}
        0x12a50c0 => fold_children(p, 0x48, 0x50, 0x10, me, depth),
        0x1248150 => fold_children(p, 0x50, 0x58, 0x18, me, depth),
        // 레벨(self.0x5c8) >= 3 이면 자식 (p+0x10,p+0x18), 아니면 (p+0,p+8) 로 위임
        0x164eba0 => {
            let o = if rd_u64(me + 0x5c8)? >= 3 { 0x10usize } else { 0 };
            let (cd, cv) = (rd_u64(p + o)? as usize, rd_u64(p + o + 8)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
            spec_a0(cd, cv, me, depth + 1)
        }
        // 두 리스트(첫째 stride 0x18, 둘째 stride 0x10)
        0x153b540 => {
            let a1 = fold_children(p, 0x50, 0x58, 0x18, me, depth)?;
            let a2 = fold_children(p, 0x68, 0x70, 0x10, me, depth)?;
            Some(match (a1, a2) {
                (None, x) => x,
                (Some(x), None) => Some(x),
                (Some(mut x), Some(y)) => {
                    for o in FOLD_I32 { let v = sp_i32(&x, o).wrapping_add(sp_i32(&y, o)); sp_set_i32(&mut x, o, v); }
                    for o in FOLD_I64 { let v = sp_i64(&x, o).wrapping_add(sp_i64(&y, o)); sp_set_i64(&mut x, o, v); }
                    for o in FOLD_BOOL { x[o] |= y[o]; }
                    Some(x)
                }
            })
        }
        0x12a5be0 => fold_children(p, 0x20, 0x28, 0x18, me, depth),
        0x106a440 => {                                                            // memcpy(sret, inline, 0x120)
            let mut b = [0u8; SPEC_SIZE];
            for i in 0..SPEC_SIZE { b[i] = rd_u8(p + i); }
            if i32::from_le_bytes([b[0x48], b[0x49], b[0x4a], b[0x4b]]) == -1 { return Some(None); }
            Some(Some(b))
        }
        _ => { if let Some(r) = super::dyn_eff::impl_rva(vt, 0xa0) { super::dyn_eff::unseen(0x9a0, r); } na_tag("Ba0").map(|_| None) }
    }
}
/// slot.vt+0xa0 의 태그만 (−1 = 없음)
unsafe fn sret_spec_tag(data: usize, vt: usize, me: usize, _tag: &str) -> Option<i32> {
    Ok::<(), ()>(()).ok();
    match spec_a0(data, vt, me, 0)? {
        None => Some(-1),
        Some(b) => Some(i32::from_le_bytes([b[0x48], b[0x49], b[0x4a], b[0x4b]])),
    }
}

// ── 0xe047c0 — BuffSpec 생성 (1,172B) ────────────────────────────────────────────────────
//   `(sret, slot, ctx, self)` : slot.vt+0xb8 로 버프 def 를 얻고, def 의 TypeId 가 버프형이면
//   챔피언 어빌리티 전부의 BuffSpec 성분합 × def.0x00(퍼센트) / 100 을 만든다.
//   정본 = `RE\2026-09-07_BuffSpec레이아웃-0xe047c0-0xe03360-0xe02540-전수해독-0.5.8.md` §2
pub const TID_BUFF: usize = 0x33e1d70;
pub const TID_CONT: usize = 0x33e1cf0;

/// `vt+0x18`(Any::type_id) impl 이 복사해 오는 **정적 TypeId 상수의 RVA**.
///   codegen: `mov rax,rcx` + `movups xmm0,[rip+d]` … (`0x48 0x89 0xc8 0x0f 0x10 0x05 d32`)
unsafe fn typeid_rva(vt: usize) -> Option<usize> {
    let g = rd_u64(vt + 0x18)? as usize; if !ptr_ok(g) { return None; }
    if !(rd_u8(g) == 0x48 && rd_u8(g + 1) == 0x89 && rd_u8(g + 2) == 0xc8 && rd_u8(g + 3) == 0x0f && rd_u8(g + 4) == 0x10 && rd_u8(g + 5) == 0x05) {
        super::dyn_eff::unseen(0xa18, g.wrapping_sub(crate::exe_base())); return None;
    }
    let tid = (g as isize + 10 + rd_i32(g + 6)? as isize) as usize;
    let b = crate::exe_base(); if b == 0 || tid <= b { return None; }
    Some(tid - b)
}
/// slot.vt+0xb8 → 버프 def **fat 포인터** `(data, vt)`. data==0 이면 none.
///   ★호출부(`0xe047c0` @e0481b)는 이 호출이 `rdx`(=두 번째 반환값 = def 의 vtable)를 덮어쓴다는 것을 그대로 이용해
///   바로 다음 줄에서 `mov rsi,[rdx+0x18]` 로 **def 자신의 `type_id`** 를 집는다. slot.vt+0x18 이 아니다(2026-09-07 04:35 정정).
///   실측 최빈형: `mov rax,rcx; lea rdx,[rip+D]; ret` → (inline self, f+10+D).
unsafe fn slot_def_b8(data: usize, vt: usize) -> Option<(usize, usize)> {
    let f = rd_u64(vt + 0xb8)? as usize;
    let p = inline_self(data, vt)?;
    if rd_u8(f) == 0x48 && rd_u8(f + 1) == 0x89 && rd_u8(f + 2) == 0xc8
       && rd_u8(f + 3) == 0x48 && rd_u8(f + 4) == 0x8d && rd_u8(f + 5) == 0x15 {
        let dv = (f as isize + 10 + rd_i32(f + 6)? as isize) as usize;
        if !ptr_ok(dv) { return None; }
        return Some((p, dv));
    }
    // `xor eax,eax; ret` 류(=def 없음)는 그대로 null fat ptr
    if let Some(v) = super::as_callees::decode_getter(f, p) { if v == 0 { return Some((0, 0)); } }
    if let Some(r) = super::dyn_eff::impl_rva(vt, 0xb8) { super::dyn_eff::unseen(0x9b8, r); }
    None
}

/// `0xe047c0`. 성공하면 `Some(BuffSpec)`(0x120 바이트 · i32 필드만 채움), 실패(=tag −1)면 `Some(None)`.
pub unsafe fn e047c0(slot: usize, ctx: usize, me: usize) -> Option<Option<[i32; 72]>> {
    let (sd, sv) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    if !ptr_ok(sd) || !ptr_ok(sv) { return None; }
    let (def, dvt) = slot_def_b8(sd, sv)?;
    if def == 0 { return Some(None); }
    let tid = typeid_rva(dvt)?;
    let hit = if tid == TID_BUFF { def }
        else if tid == TID_CONT {
            let n = rd_u64(def + 0x10)?; if n == 0 { return Some(None); }
            let arr = rd_u64(def + 8)? as usize; if !ptr_ok(arr) { return None; }
            let mut found = 0usize;
            for i in 0..n.min(64) as usize {
                let (cd, cv) = (rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize);
                if !ptr_ok(cd) || !ptr_ok(cv) { continue; }
                let (d2, dv2) = match slot_def_b8(cd, cv) { Some(v) => v, None => continue };
                if d2 == 0 { continue; }
                if typeid_rva(dv2)? == TID_BUFF { found = d2; break; }
            }
            if found == 0 { return Some(None); }
            found
        } else { return Some(None) };

    let w = rd_u64(ctx)? as usize; if !ptr_ok(w) { return None; }
    let rec = sim_of_handle(w, rd_u64(me + ENT_HANDLE)?)?;
    if rec == 0 { return Some(None); }
    let mult = rd_u32(hit) as i32;                     // 매칭된 def 의 +0x00 = 퍼센트

    // 챔피언 어빌리티 전부의 BuffSpec 성분합 (전부 i32 wrapping)
    let n = rd_u64(rec + 0x4a8)?;
    let arr = rd_u64(rec + 0x4a0)? as usize;
    if n != 0 && !ptr_ok(arr) { return None; }
    let mut sum = [0i32; 72];
    for i in 0..n.min(32) as usize {
        let e = arr + i * 0x10;
        let (_ed, ev) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
        if !ptr_ok(ev) { return None; }
        // ⬜어빌리티별 `vt+0x78`(sret BuffSpec 0x120) 미포팅 — impl 목록부터 모은다
        if let Some(r) = super::dyn_eff::impl_rva(ev, 0x78) { super::dyn_eff::unseen(0xa78, r); }
        return na_tag("B78").map(|_| None);
    }
    let mut out = [0i32; 72];
    for k in [0x58usize, 0x5c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x90, 0x104] {
        out[k / 4] = sum[k / 4].wrapping_mul(mult) / 100;
    }
    out[0x48 / 4] = 0;
    Some(Some(out))
}

// ── 0xe03ed0 — Spirit Caller 표식 소모 가치 (1,324B, S13 전용) ──────────────────────────
//   정본 = `RE\2026-09-07_combat_score-S13S14-버프콜리4종-…-0.5.8.md` §1
//   `(slot, ctx, rec, bb, self, C) -> i64` : 아군/적 로스터에서 "spirit_caller_skill1/2" 버프를 가진
//   유닛을 반경 안에서 찾아 기여도를 합산. 표식 2개 미만이면 0, 상한 160.
const TID_SPIRIT: usize = 0x33e1ce0;   // e03ed0 의 TID_A
const EFF_STRIDE: usize = 0x120;

/// 엔티티의 버프 목록(`e.0x2e0` ptr / `e.0x2e8` len, stride 0x120)에서 이름이 `name`(길이 20)인 것이 있는가
unsafe fn has_effect20(e: usize, tail4: &[u8; 4]) -> Option<bool> {
    let n = rd_u64(e + 0x2e8)?; if n == 0 { return Some(false); }
    let p = rd_u64(e + 0x2e0)? as usize; if !ptr_ok(p) { return None; }
    const HEAD: &[u8; 16] = b"spirit_caller_sk";
    for i in 0..n.min(64) as usize {
        let eff = p + i * EFF_STRIDE;
        if rd_u32(eff) != 0x14 { continue; }
        let mut ok = true;
        for k in 0..16 { if rd_u8(eff + 4 + k) != HEAD[k] { ok = false; break; } }
        if !ok { continue; }
        for k in 0..4 { if rd_u8(eff + 20 + k) != tail4[k] { ok = false; break; } }
        if ok { return Some(true); }
    }
    Some(false)
}

pub unsafe fn e03ed0(slot: usize, ctx: usize, rec: usize, bb: usize, me: usize, c: i64) -> Option<i64> {
    let (sd, sv) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    if !ptr_ok(sd) || !ptr_ok(sv) { return None; }
    let (def, dvt) = slot_def_b8(sd, sv)?;
    if def == 0 { return Some(0); }
    // TypeId 가 표식형이 아니면 컨테이너 1단계만 훑어 표식형 자식을 찾는다
    let hit = if typeid_rva(dvt)? == TID_SPIRIT { def } else {
        if typeid_rva(dvt)? != TID_CONT { return Some(0); }
        let n = rd_u64(def + 0x10)?; if n == 0 { return Some(0); }
        let arr = rd_u64(def + 8)? as usize; if !ptr_ok(arr) { return None; }
        let mut found = 0usize;
        for i in 0..n.min(64) as usize {
            let (cd, cv) = (rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { continue; }
            let (d2, dv2) = match slot_def_b8(cd, cv) { Some(v) => v, None => continue };
            if d2 == 0 { continue; }
            if typeid_rva(dv2)? == TID_SPIRIT { found = d2; break; }
        }
        if found == 0 { return Some(0); }
        found
    };

    let st = rd_u64(me + 0x620)?;                       // 주문력(추정) — RE §1 의 `self.0x620`
    let amt1 = rd_i64(hit + 0x08)?.wrapping_add((rd_u64(hit + 0x10)?.wrapping_mul(st) / 100) as i64);
    let amt2 = rd_i64(hit + 0x30)?.wrapping_add((rd_u64(hit + 0x38)?.wrapping_mul(st) / 100) as i64);
    let r = rd_u64(hit)?; let r2 = r.wrapping_mul(r);
    let side = rd_u64(rec + 0x930)?; if side > 1 { return None; }
    let w = rd_u64(ctx)? as usize; if !ptr_ok(w) { return None; }
    let (sx, sy) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    let (mut sum, mut cnt): (i64, u64) = (0, 0);

    // 루프1 — 내 팀 5칸, "…skill1"
    for i in 0..5usize {
        let e = rd_u64(w + 0x1e0 + (side as usize) * 0x28 + i * 8)? as usize; if e == 0 { continue; }
        let (ex, ey) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
        let (dx, dy) = (absd(ex, sx), absd(ey, sy));
        if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) > r2 { continue; }
        if !has_effect20(e, b"ill1")? { continue; }
        let hp = rd_i64(e + ENT_HP)?;
        let den = if hp < 2 { 1 } else { hp };
        sum = sum.wrapping_add(amt1.min(hp).wrapping_mul(c) / den);
        cnt += 1;
    }
    // 루프2 — 적 팀 5칸, "…skill2"
    let half = c / 2;
    let (rp, rn) = (rd_u64(bb + BV_ENEMY_PTR)? as usize, rd_u64(bb + BV_ENEMY_LEN)?);
    for j in 0..5usize {
        let e = rd_u64(w + 0x1e0 + ((1 - side) as usize) * 0x28 + j * 8)? as usize; if e == 0 { continue; }
        let (ex, ey) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
        let (dx, dy) = (absd(ex, sx), absd(ey, sy));
        if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) > r2 { continue; }
        if !has_effect20(e, b"ill2")? { continue; }
        cnt += 1;
        let h = rd_u64(e + ENT_HANDLE)?;
        let mut mult = half;
        if rn != 0 && ptr_ok(rp) {
            for k in 0..rn.min(64) as usize {
                let r = rp + k * 0xd8;
                if rd_u64(r + 0x58)? == h { mult = super::as_callees::pct_c(bb, r)?; break; }
            }
        }
        let hp = rd_i64(e + ENT_HP)?;
        let den = if hp < 2 { 1 } else { hp };
        sum = sum.wrapping_add(amt2.min(hp).wrapping_mul(mult) / den);
    }
    if cnt < 2 { return Some(0); }
    Some(sum.min(160))
}

// ── 0xe02bc0 — 광역 힐/실드 가치 (871B) ──────────────────────────────────────────────────
//   `(mode 미사용, slot, ctx, bb, self, tgt, tgt.handle, sp) -> i64`
//   `slot.vt+0xd0` 이 **(kind, aoeRadius) 2값**을 돌려주고, kind != 1 이면 즉시 0.
unsafe fn slot_d0(data: usize, vt: usize) -> Option<(u64, u64)> {
    let f = rd_u64(vt + 0xd0)? as usize;
    let p = inline_self(data, vt)?;
    // `mov rax,[rcx+A]; mov rdx,[rcx+B]; ret` 형(2값 게터)
    if rd_u8(f) == 0x48 && rd_u8(f + 1) == 0x8b && (rd_u8(f + 2) & 0xC7) == 0x41 {
        let a = rd_u8(f + 3) as usize;
        let g = f + 4;
        if rd_u8(g) == 0x48 && rd_u8(g + 1) == 0x8b && (rd_u8(g + 2) & 0xC7) == 0x51 && rd_u8(g + 4) == 0xc3 {
            return Some((rd_u64(p + a)?, rd_u64(p + rd_u8(g + 3) as usize)?));
        }
    }
    let eb = crate::exe_base();
    if eb != 0 && f > eb {
        match f - eb {
            // 자식 중 **kind&1 이 선 첫 자식**의 (kind, 반경). 없으면 (0, _)
            0x12a6e80 => {
                let n = rd_u64(p + 0x10)?; if n == 0 { return Some((0, 0)); }
                let arr = rd_u64(p + 8)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n.min(64) as usize {
                    let e = arr + i * 0x10;
                    let (cd, cv) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
                    if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
                    let (k, r) = slot_d0(cd, cv)?;
                    if k & 1 != 0 { return Some((k, r)); }
                }
                return Some((0, 0));
            }
            0x9db70 => return Some((0, 0)),          // xor eax,eax; ret
            _ => {}
        }
    }
    if let Some(r) = super::dyn_eff::impl_rva(vt, 0xd0) { super::dyn_eff::unseen(0x9d0, r); }
    na_tag("Bd0").map(|_| (0, 0))
}

pub unsafe fn e02bc0(slot: usize, ctx: usize, bb: usize, me: usize, tgt: usize, tgt_h: u64, cast_delay: u64) -> Option<i64> {
    let (sd, sv) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    if !ptr_ok(sd) || !ptr_ok(sv) { return None; }
    let (kind, aoe_r) = slot_d0(sd, sv)?;
    let n = rd_u64(bb + 0x14d0)?;                    // 아군 Record len
    if kind != 1 || n == 0 { return Some(0); }
    let arr = rd_u64(bb + 0x14b8)? as usize; if !ptr_ok(arr) { return None; }
    let w = rd_u64(ctx)? as usize; if !ptr_ok(w) { return None; }
    let wd = rd_u64(w)? as usize; let wv = rd_u64(w + 8)? as usize;
    if !ptr_ok(wd) || !ptr_ok(wv) { return None; }
    let wr = World { x: w, data: wd, vt: wv };
    let (cx, cy) = (rd_u64(tgt + ENT_X)?, rd_u64(tgt + ENT_Y)?);
    let mut acc: i64 = 0;
    for i in 0..n.min(64) as usize {
        let rec = arr + i * 0xd8;
        let h = rd_u64(rec + 0x58)?;
        if h == tgt_h { continue; }
        let e = match wr.entity(h) { Some(x) => x.0, None => continue };
        let r = body_radius(e)?.wrapping_add(aoe_r);
        let (ex, ey) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
        let (dx, dy) = (absd(ex, cx), absd(ey, cy));
        if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) > r.wrapping_mul(r) { continue; }
        let heal_cap = slot_sum(sd, sv, 0x40, me, 0, "B40")?;
        let heal = rd_u64(e + ENT_MAXHP)?.saturating_sub(rd_u64(e + ENT_HP)?).min(heal_cap.max(0) as u64) as i64;
        let x = super::action_score::threat_sum(rec, cast_delay.wrapping_add(30))?
            .wrapping_add(rd_i64(rec + 0x70)?).wrapping_add(rd_i64(rec + 0x88)?);
        let shield = slot_sum(sd, sv, 0x48, me, 0, "B48")?.min(3 * x);
        let total = shield.wrapping_add(heal);
        if total <= 0 { continue; }
        let ce = super::as_callees::pct_c(bb, rec)?;
        let hp = rd_i64(e + ENT_HP)?;
        let den = if hp < 2 { 1 } else { hp };
        acc = acc.wrapping_add(ce.wrapping_mul(total) / den);
    }
    Some(acc)
}
