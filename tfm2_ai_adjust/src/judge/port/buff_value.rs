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
    if no == 2 && slot != usize::MAX && stride != usize::MAX && (stride == 0x10 || stride == 0x18) {
        Some((offs[0], offs[1], stride, slot))
    } else { None }
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

/// dyn 슬롯 값(i64). 합성이면 자식 합, 잎이면 디코드/스탯식, 그 외엔 unseen 기록 후 NA.
pub unsafe fn slot_sum(data: usize, vt: usize, slot: usize, me: usize, depth: u32, tag: &str) -> Option<i64> {
    if depth > 8 { return na_tag(tag); }
    let f = rd_u64(vt + slot)? as usize;          // ★절대주소(impl_rva 는 RVA)
    let p = inline_self(data, vt)?;
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
    let _ = spec0;
    // ⬜여기부터 미포팅: 0xdffa10(버프가치) → 0xe022d0 → 0xe03360/0xe02540 → 0xe02bc0/0xe03ed0
    na_tag(if ally.is_some() { "B14spec" } else { "B13spec" })
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
/// slot.vt+0xb8 → 버프 def 포인터(null 가능)
unsafe fn slot_def_b8(data: usize, vt: usize) -> Option<usize> {
    let f = rd_u64(vt + 0xb8)? as usize;
    let p = inline_self(data, vt)?;
    // ★실측 최빈형: `mov rax,rcx; lea rdx,[rip+D]; ret` = **fat ptr 반환**(rax = inline self, rdx = 정적 vtable).
    //   즉 def 데이터는 자기 자신이다(0x1701710 · 0x13c0670 · 0x183eca0 · 0x106a430 전부 이 형태).
    if rd_u8(f) == 0x48 && rd_u8(f + 1) == 0x89 && rd_u8(f + 2) == 0xc8 { return Some(p); }
    if let Some(v) = super::as_callees::decode_getter(f, p) { return Some(v as usize); }
    if let Some(r) = super::dyn_eff::impl_rva(vt, 0xb8) { super::dyn_eff::unseen(0x9b8, r); }
    None
}

/// `0xe047c0`. 성공하면 `Some(BuffSpec)`(0x120 바이트 · i32 필드만 채움), 실패(=tag −1)면 `Some(None)`.
pub unsafe fn e047c0(slot: usize, ctx: usize, me: usize) -> Option<Option<[i32; 72]>> {
    let (sd, sv) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    if !ptr_ok(sd) || !ptr_ok(sv) { return None; }
    let def = slot_def_b8(sd, sv)?;
    if def == 0 { return Some(None); }
    let tid = typeid_rva(sv)?;
    let hit = if tid == TID_BUFF { def }
        else if tid == TID_CONT {
            let n = rd_u64(def + 0x10)?; if n == 0 { return Some(None); }
            let arr = rd_u64(def + 8)? as usize; if !ptr_ok(arr) { return None; }
            let mut found = 0usize;
            for i in 0..n.min(64) as usize {
                let (cd, cv) = (rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize);
                if !ptr_ok(cd) || !ptr_ok(cv) { continue; }
                let d2 = match slot_def_b8(cd, cv) { Some(v) => v, None => continue };
                if d2 == 0 { continue; }
                if typeid_rva(cv)? == TID_BUFF { found = d2; break; }
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
