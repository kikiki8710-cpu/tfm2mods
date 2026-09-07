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
/// DIFF 로그용 S13/S14 성분 [aoe, trig, aura_t, etc, hs_term, buff, raw, dur]
thread_local! {
    pub static S13D: std::cell::Cell<[i64; 10]> = const { std::cell::Cell::new([0; 10]) };
    pub static S13E: std::cell::Cell<[i64; 12]> = const { std::cell::Cell::new([0; 12]) };
    /// ★0xe03ed0(trig) 이탈지점 추적 — [exit, def!=0, tid, n, r, st, sum, cnt]
    ///   exit: 1=slot_def_b8 없음 2=def==0 3=tid 비표식·비컨테이너 4=컨테이너 비었음 5=자식에 표식 없음 9=끝까지 계산
    pub static E3D: std::cell::Cell<[i64; 8]> = const { std::cell::Cell::new([0; 8]) };
}
pub fn s13_diag() -> String {
    let v = S13D.with(|c| c.get()); let e = S13E.with(|c| c.get()); let f = S14D.with(|c| c.get()); let g = E3D.with(|c| c.get()); let h = AOED.with(|c| c.get()); let k = A0CH.with(|c| c.get());
    format!(" S13[aoe={} trig={} auraT={} etc={} hs={} buff={} raw={} dur={} AURA={} b0i={:#x}] S13E[healE={} shieldE={} inc={} gate={} total={} has={} ally={} mainRaw={} healRaw={} cap={} miss={} shRaw={}] S14[v={} k={} vd={} st={} b1={} b2={} src={} raw={} fp={} ns={} a0i={:#x}] E3[exit={} def={} tid={:#x} n={} r={} st={} sum={} cnt={}] AOE[kind={} R={} n={} slf={} noe={} dst={} cap={} tot={} poff={} vt={:#x} d0i={:#x} i40={:#x}] A0CH[{:#x} {:#x} {:#x} {:#x}]",
        v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], e[0], e[1], e[2], e[3], e[4], e[5], e[6], e[7], e[8], e[9], e[10], e[11],
        f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], g[0], g[1], g[2], g[3], g[4], g[5], g[6], g[7],
        h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7], h[8], h[9], h[10], h[11], k[0], k[1], k[2], k[3])
}
/// 잎 에뮬레이터가 필요로 하는 두 컨텍스트(sim · EST 서술자 절대주소). S13/S14 진입 때 한 번 세운다.
thread_local! {
    static SIM_TLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static EST_DESC_RVA_ABS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}
pub unsafe fn set_leaf_ctx(sim: usize) { SIM_TLS.with(|c| c.set(sim)); EST_DESC_RVA_ABS.with(|c| c.set((crate::exe_base() + 0x33da3d0) as u64)); }
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
/// `e02540` 의 바이트 배열판(재현이 만든 BuffSpec 을 그대로 넘길 때)
pub unsafe fn e02540_bytes(ctx: usize, spec: &[u8; SPEC_SIZE], me: usize, st: u8) -> Option<i64> {
    e02540_impl(ctx, |o, sz| { let mut v = 0u64; for i in 0..sz { v |= (spec[o + i] as u64) << (8 * i); } Some(v) }, me, st)
}
pub unsafe fn e02540(ctx: usize, spec: usize, me: usize, st: u8) -> Option<i64> {
    e02540_impl(ctx, |o, sz| match sz { 4 => Some(rd_u32(spec + o) as u64), _ => rd_u64(spec + o) }, me, st)
}
unsafe fn e02540_impl<F: Fn(usize, usize) -> Option<u64>>(ctx: usize, rd: F, me: usize, st: u8) -> Option<i64> {
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

    let s32 = |o: usize| -> Option<i32> { rd(o, 4).map(|v| v as u32 as i32) };
    let s64 = |o: usize| -> Option<i64> { rd(o, 8).map(|v| v as i64) };
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
    for i in 0..n.min(CAP_ITER) as usize {
        let e = arr + i * stride;
        let (cd, cv) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
        if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
        acc = acc.wrapping_add(slot_sum(cd, cv, cs, me, depth + 1, tag)?);
    }
    Some(acc)
}
/// `slot_sum` 의 "페이로드를 직접 받는" 판 — 썽크가 넘긴 내부 포인터용(Arc 보정 금지).
pub unsafe fn slot_sum_p(payload: usize, vt: usize, slot: usize, me: usize, depth: u32, tag: &str) -> Option<i64> {
    if depth > 40 { super::dyn_eff::unseen(0x613, depth as usize); return na_tag(tag); }
    let f = rd_u64(vt + slot)? as usize;
    if let Some(v) = super::as_callees::decode_getter(f, payload) { return Some(v as i64); }
    let base = crate::exe_base();
    if base != 0 && f > base { if let Some(v) = leaf_stat_scaled(f - base, payload, me) { return Some(v); } }
    // ★`0x12b1550`(vt+0x40) — 분기+div 라 specemu 가 못 돈다(RE 2026-09-07)
    if base != 0 && f == base + 0x12b1550 {
        let d = rd_u64(payload + 0x28)?; if d == 0 { return na_tag(tag); }
        let q = rd_u64(payload + 0x20)? / d;
        return Some((rd_u64(payload)?.wrapping_add(rd_u64(payload + 8)?.wrapping_mul(rd_u64(me + 0x620)?)).wrapping_mul(q)) as i64);
    }
    // ★잎이면 specemu 로 실제 실행(non-sret 규약). 손으로 옮기는 것보다 정확하다.
    if base != 0 && f > base { if let Some(v) = super::specemu::run_leaf_i64(f, payload as u64, me as u64, (base + 0x33da3d0) as u64) { return Some(v); } }
    if let Some(r) = super::dyn_eff::impl_rva(vt, slot) { super::dyn_eff::unseen(0x800 + slot as u32, r); }
    na_tag(tag)
}
/// dyn 슬롯 값(i64). 합성이면 자식 합, 잎이면 디코드/스탯식, 그 외엔 unseen 기록 후 NA.
pub unsafe fn slot_sum(data: usize, vt: usize, slot: usize, me: usize, depth: u32, tag: &str) -> Option<i64> {
    if depth > 40 { super::dyn_eff::unseen(0x610, depth as usize); return na_tag(tag); }
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
    // SwitchByBuff(vt+0x40 계열): 버프 유무로 두 자식 중 하나를 **같은 슬롯**으로 테일콜
    if eb != 0 && f > eb {
        match f - eb {
            // ★SwitchByBuff 패밀리 7종 — **자식 슬롯이 함수마다 하드코딩**이다(RE 2026-09-07).
            //   부모 `slot` 으로 재귀하던 구 코드는 둘이 우연히 같을 때만 맞았다.
            0x1606360 | 0x16063d0 | 0x1606470 | 0x16064e0 | 0x16065e0 | 0x1606690 | 0x16067b0 => {
                let cs = match f - eb { 0x1606360 => 0x50usize, 0x16063d0 => 0x40, 0x1606470 => 0x28,
                                        0x16064e0 => 0x48, 0x16065e0 => 0xa0, 0x1606690 => 0x70, _ => 0x38 };
                let idx = if super::dyn_eff::buff_lookup(me, rd_u64(p + 8)? as usize, rd_u64(p + 0x10)?)? != 0 { 0x10usize } else { 0 };
                let (cd, cv) = (rd_u64(p + 0x18 + idx)? as usize, rd_u64(p + 0x20 + idx)? as usize);
                if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
                return slot_sum(cd, cv, cs, me, depth + 1, tag);
            }
            // ★`0x1606550`·`0x1606580` 은 SwitchByBuff 가 **아니다** — buff_lookup 없이 무조건 자식0 으로
            //   위임하는 썽크(자식 슬롯 0x80). 구 코드는 이걸 SwitchByBuff 로 처리해 버프가 있으면
            //   자식1을 타는 조용한 DIFF 를 만들고 있었다.
            0x1606550 | 0x1606580 => {
                let (cd, cv) = (rd_u64(p + 0x18)? as usize, rd_u64(p + 0x20)? as usize);
                if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
                return slot_sum(cd, cv, 0x80, me, depth + 1, tag);
            }
            // 포워딩 썽크(self=rcx). 내부 fat-ptr 로 재디스패치하되 **Arc 보정 없이** 원본 data 를 넘긴다.
            0x1153880 => { let (id, iv) = (rd_u64(p)? as usize, rd_u64(p + 8)? as usize);
                if !ptr_ok(id) || !ptr_ok(iv) { return None; }
                return slot_sum_p(id, iv, 0xb0, me, depth + 1, tag); }
            0x11538a0 => { let (id, iv) = (rd_u64(p)? as usize, rd_u64(p + 8)? as usize);
                if !ptr_ok(id) || !ptr_ok(iv) { return None; }
                return slot_sum_p(id, iv, 0x98, me, depth + 1, tag); }
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
        for i in 0..n.min(CAP_ITER) as usize {
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
    // ★`0x12b1550`(vt+0x40) 네이티브 — 분기+div 라 specemu 불가(RE 2026-09-07)
    if eb != 0 && f == eb + 0x12b1550 {
        let d = rd_u64(p + 0x28)?; if d == 0 { return na_tag(tag); }
        let q = rd_u64(p + 0x20)? / d;
        return Some((rd_u64(p)?.wrapping_add(rd_u64(p + 8)?.wrapping_mul(rd_u64(me + 0x620)?)).wrapping_mul(q)) as i64);
    }
    // ★잎이면 specemu 로 실제 실행(non-sret 규약)
    if eb != 0 && f > eb { if let Some(v) = super::specemu::run_leaf_i64(f, p as u64, me as u64, (eb + 0x33da3d0) as u64) { return Some(v); } }
    if let Some(r) = super::dyn_eff::impl_rva(vt, slot) { super::dyn_eff::unseen(0x900 + slot as u32, r); }
    na_tag(tag)
}

// ── slot.vt+0xa0 : sret BuffSpec(0x120) ─────────────────────────────────────────────────
//   정본 = `RE\2026-09-07_BuffSpec-fold-0x126e6c0-0.5.8.md`
//   `0x126e6c0` = 자식들의 BuffSpec 을 접는다: **첫 유효(=+0x48 != −1) 자식을 통째로 채택**하고,
//   이후 유효 자식은 아래 오프셋 집합만 wrapping 합(i32/i64) 또는 논리합(bool). 태그·페이로드는 버린다.
const FOLD_I32: [usize; 18] = [0x58, 0x5c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x90, 0xfc, 0x100, 0x104];
const FOLD_I64: [usize; 14] = [0x98, 0xa0, 0xa8, 0xb0, 0xb8, 0xc0, 0xc8, 0xd0, 0xd8, 0xe0, 0xe8, 0xf0, 0x108, 0x110];
const FOLD_BOOL: [usize; 3] = [0xf8, 0x118, 0x119];

#[inline] fn sp_i32(b: &[u8; SPEC_SIZE], o: usize) -> i32 { i32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) }
/// ★BuffSpec 앞부분은 **이름**이다 — `[0x00]` u32 len + `[0x04..]` 바이트.
/// 게임의 vt+0xa0 잎들은 전부 이름을 쓴다. 이름을 빼먹으면 이름을 키로 쓰는 하류(버프 조회 등)가 어긋난다
/// (2026-09-07 실측: 이름 없이 배선했더니 DIFF 0.089% → 0.195% 로 악화).
#[inline] fn sp_set_name(b: &mut [u8; SPEC_SIZE], nm: &str) {
    let by = nm.as_bytes(); b[0..4].copy_from_slice(&(by.len() as u32).to_le_bytes());
    b[4..4 + by.len()].copy_from_slice(by);
}
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
    for i in 0..n.min(CAP_ITER) as usize {
        let e = arr + i * stride;
        let (cd, cv) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
        if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
        // 자식 impl RVA 기록(진단) — 어느 잎이 빈 spec 을 내는지 특정
        { let eb = crate::exe_base(); let f = rd_u64(cv + 0xa0).unwrap_or(0) as usize;
          let r = if eb != 0 && f > eb { (f - eb) as i64 } else { 0 };
          A0CH.with(|c| { let mut z = c.get(); for k in 0..4 { if z[k] == 0 { z[k] = r; break; } if z[k] == r { break; } } c.set(z); }); }
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
    if depth > 40 { super::dyn_eff::unseen(0x611, depth as usize); return na_tag("B90d").map(|_| false); }
    let f = rd_u64(vt + 0x90)? as usize;
    let p = inline_self(data, vt)?;
    let eb = crate::exe_base(); if eb == 0 || f <= eb { return None; }
    if f - eb == 0x12a71e0 {
        let n = rd_u64(p + 0x10)?; if n == 0 { return Some(false); }
        let arr = rd_u64(p + 8)? as usize; if !ptr_ok(arr) { return None; }
        for i in 0..n.min(CAP_ITER) as usize {
            let e = arr + i * 0x10;
            let (cd, cv) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
            if slot_bool90(cd, cv, depth + 1)? { return Some(true); }
        }
        return Some(false);
    }
    if f - eb == 0x13bf4b0 {
        let n = rd_u64(p + 0x58)?; if n == 0 { return Some(false); }
        let arr = rd_u64(p + 0x50)? as usize; if !ptr_ok(arr) { return None; }
        for i in 0..n.min(CAP_ITER) as usize {
            let e = arr + i * 0x18;
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
#[inline] unsafe fn rd_i32_at(a: usize) -> i32 { crate::rd_i32(a).unwrap_or(0) }
pub unsafe fn spec_a0(data: usize, vt: usize, me: usize, depth: u32) -> Option<Option<[u8; SPEC_SIZE]>> {
    spec_a0_inline(inline_self(data, vt)?, vt, me, depth)
}
/// `spec_a0` 의 inline 기준판. 일부 impl(`0x1153860`)은 자식에게 **Arc 보정 없이** payload 를 그대로 넘긴다.
pub unsafe fn spec_a0_inline(p: usize, vt: usize, me: usize, depth: u32) -> Option<Option<[u8; SPEC_SIZE]>> {
    let sim = SIM_TLS.with(|c| c.get()) as u64;
    if depth > 40 { super::dyn_eff::unseen(0x612, depth as usize); return na_tag("Ba0d").map(|_| None); }
    let f = rd_u64(vt + 0xa0)? as usize;
    let eb = crate::exe_base(); if eb == 0 || f <= eb { return None; }
    match f - eb {
        0x109baa0 => Some(None),                                                  // 기본 impl: +0x48 = −1
        // inline+0x120 플래그가 서면 없음, 아니면 inline 에 저장된 spec 을 그대로 복사
        // ── ★vt+0xa0 미처리 12종 네이티브 배선 (RE 2026-09-07 `vt+0xa0-미처리12종-전량해독`) ──
        //   전부 sret 를 0 으로 초기화한 뒤 아래 필드만 기록한다(게임이 xorps+movups 로 명시 제로필).
        //   `p` 는 Arc 보정 후, `arg3` 은 12종 전부 미사용, 스탯은 `me`(caster) 기준.
        //   ⚠specemu 로는 `0x12266f0`(0x66 prefix)·`0x133e390`(분기)·`0x18890b0`(0x01)이 실패해
        //     표본이 NA 로 버려지고 있었다 — 네이티브가 정답.
        // ⚠EMU-OK 5종(`0x12bc970`·`0x12c21b0`·`0x12aa290`·`0x17c9e90`·`0x17cd2e0`)은 **arm 을 두지 않는다** —
        //   RE 가 specemu 로 완벽 실행됨을 확인했고, 손으로 다시 쓰면 게임이 쓰는 필드를 빠뜨리기만 한다
        //   (2026-09-07 실측: 8종 전부 손으로 쓰니 DIFF 0.089% → 0.195%, 이름을 채워도 0.237%).
        //   아래 3종은 specemu 가 명령 미지원으로 **실패**하던 것들이라 네이티브가 유일한 방법이다.
        0x12266f0 => { let mut b = [0u8; SPEC_SIZE]; sp_set_name(&mut b, "wind_mage_skill1_speed");                    // "wind_mage_skill1_speed"
            sp_set_i32(&mut b, 0x48, 1); sp_set_i64(&mut b, 0x50, rd_i64(p + 8)?);
            sp_set_i32(&mut b, 0x88, rd_i32_at(p + 0x28)); Some(Some(b)) }
        0x133e390 => {                                                   // "icemage_ult_slow" (조건부)
            let d = rd_i64(p + 0x40)?; if d == 0 { return Some(None); }  // tag −1
            let mut b = [0u8; SPEC_SIZE]; sp_set_name(&mut b, "icemage_ult_slow");
            sp_set_i32(&mut b, 0x48, 1); sp_set_i64(&mut b, 0x50, d);
            sp_set_i32(&mut b, 0x88, 0i32.wrapping_sub(rd_i32_at(p + 0x38)));   // ★부호 반전
            Some(Some(b)) }
        0x18890b0 => { let mut b = [0u8; SPEC_SIZE]; sp_set_name(&mut b, "plague_doctor_ult");                    // "plague_doctor_ult"
            let ap = rd_u64(me + 0x620)?;
            let t = (rd_u64(p + 0x10)?.wrapping_mul(ap) >> 2) / 100;
            sp_set_i32(&mut b, 0x48, 1); sp_set_i64(&mut b, 0x50, rd_i64(p + 0x20)?);
            sp_set_i32(&mut b, 0x58, rd_i32_at(p));
            sp_set_i32(&mut b, 0x88, rd_i32_at(p + 0x18));
            sp_set_i32(&mut b, 0x8c, (t as i32).wrapping_add(rd_i32_at(p + 8)));
            b[0x118] = 1; Some(Some(b)) }
        // fold 3종 (삼중항은 RE 검증 완료)
        0x13bfbf0 => fold_children(p, 8, 0x10, 0x18, me, depth),
        0x1455d70 => fold_children(p, 0x20, 0x28, 0x18, me, depth),
        0x1340e80 => { let a1 = fold_children(p, 0x68, 0x70, 0x18, me, depth)?;
            let a2 = fold_children(p, 0x80, 0x88, 0x10, me, depth)?;
            Some(match (a1, a2) { (None, x) => x, (x, None) => x,
                (Some(mut x), Some(y)) => { for o in FOLD_I32 { let v = sp_i32(&x, o).wrapping_add(sp_i32(&y, o)); sp_set_i32(&mut x, o, v); }
                    for o in FOLD_I64 { let v = sp_i64(&x, o).wrapping_add(sp_i64(&y, o)); sp_set_i64(&mut x, o, v); }
                    for o in FOLD_BOOL { x[o] |= y[o]; } Some(x) } }) }
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
        // SwitchByBuff sret 판 — 버프 유무로 자식0/자식1 을 고르고 **자식 슬롯 0xa0** 으로 위임(RE 2026-09-07)
        0x16065e0 => {
            let idx = if super::dyn_eff::buff_lookup(me, rd_u64(p + 8)? as usize, rd_u64(p + 0x10)?)? != 0 { 0x10usize } else { 0 };
            let (cd, cv) = (rd_u64(p + 0x18 + idx)? as usize, rd_u64(p + 0x20 + idx)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
            spec_a0(cd, cv, me, depth + 1)
        }
        // ★0x114b3f0 → 본체 0x1140b10: 원본 BuffSpec(0x120) 을 통째 복사한 뒤 12개 스케일 계수를
        //   엔티티 스탯 S0..S4(= ent+0x618/0x620/0x628/0x630/0x638, 전부 하위 i32)로 가산한다.
        //   `+0x48`(태그)·`+0x80`(vamp)은 **절대 수정하지 않는다**(RE 2026-09-07).
        0x114b3f0 => {
            let mut b = [0u8; SPEC_SIZE]; for i in 0..SPEC_SIZE { b[i] = rd_u8(p + i); }
            if i32::from_le_bytes([b[0x48], b[0x49], b[0x4a], b[0x4b]]) == -1 { return Some(None); }
            let st = |k: usize| -> Option<i32> { Some(rd_u64(me + 0x618 + k * 8)? as i32) };
            let (s0, s1, s2, s3, s4) = (st(0)?, st(1)?, st(2)?, st(3)?, st(4)?);
            let q = |m: usize, sc: i32| -> i32 { (rd_i32_at(p + m)).wrapping_mul(sc) / 100 };
            let add = |b: &mut [u8; SPEC_SIZE], o: usize, v: i32| { let x = sp_i32(b, o).wrapping_add(v); sp_set_i32(b, o, x); };
            add(&mut b, 0x8c, q(0x120, s0).wrapping_add(q(0x124, s1)));
            add(&mut b, 0x88, q(0x128, s0).wrapping_add(q(0x12c, s3)).wrapping_add(q(0x130, s1)));
            add(&mut b, 0x84, q(0x138, s2));
            add(&mut b, 0x68, q(0x144, s1));
            add(&mut b, 0x78, q(0x148, s1));
            // 아래 3개는 i32 로 더한 뒤 ≤0 이면 0 으로 클램프하고 **u64 로** 저장(상위 32b = 0)
            let clamp_u64 = |b: &mut [u8; SPEC_SIZE], o: usize, t: i32| {
                let v = if t > 0 { t as u32 as i64 } else { 0 }; sp_set_i64(b, o, v);
            };
            let te8 = sp_i32(&b, 0xe8).wrapping_add(q(0x13c, s2)).wrapping_add(q(0x140, s4));
            clamp_u64(&mut b, 0xe8, te8);
            let tc8 = sp_i32(&b, 0xc8).wrapping_add(q(0x134, s1).wrapping_mul(1000));
            clamp_u64(&mut b, 0xc8, tc8);
            if sp_i32(&b, 0x48) == 1 {
                let t50 = sp_i32(&b, 0x50).wrapping_add(q(0x14c, s1));
                clamp_u64(&mut b, 0x50, t50);
            }
            Some(Some(b))
        }
        // `mov rax,[rdx]; mov rdx,[rdx+8]; jmp [rdx+0xa0]` — 자식 fat-ptr 을 **보정 없이** 그대로 넘기는 위임
        0x1153860 => {
            let (cd, cv) = (rd_u64(p)? as usize, rd_u64(p + 8)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
            spec_a0_inline(cd, cv, me, depth + 1)
        }
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
        // ★잎(챔피언 어빌리티별 스펙 빌더)은 미니 에뮬레이터로 해석한다. 모르는 명령이면 None → NA(조용히 틀리지 않음).
        _ => {
            if let Some(bspec) = super::specemu::run_spec_leaf(f, p as u64, sim, me as u64, EST_DESC_RVA_ABS.with(|c| c.get())) {
                if i32::from_le_bytes([bspec[0x48], bspec[0x49], bspec[0x4a], bspec[0x4b]]) == -1 { return Some(None); }
                return Some(Some(bspec));
            }
            if let Some(r) = super::dyn_eff::impl_rva(vt, 0xa0) { super::dyn_eff::unseen(0x9a0, r); }
            na_tag("Ba0").map(|_| None)
        }
    }
}
/// S13(자기 버프) · S14(아군 버프). `ally` = 아군 Record(S14) / None(S13).
///   정본 = `RE\2026-09-07_combat_score-S13S14-본체구간-정밀전사-0.5.8.md`
/// ★0xe02bc0(aoe) 진단 — [kind, R, n, self_skip, noent, dist_skip, heal_cap, total]
thread_local! { /// ★spec_a0 fold 가 만난 자식 impl RVA (최대 4) — 빈 spec 을 내는 자식을 특정한다
pub static A0CH: std::cell::Cell<[i64; 4]> = const { std::cell::Cell::new([0; 4]) };
pub static AOED: std::cell::Cell<[i64; 12]> = const { std::cell::Cell::new([0; 12]) }; }
pub fn aoe_diag() -> [i64; 12] { AOED.with(|c| c.get()) }
pub fn a0ch_diag() -> [i64; 4] { A0CH.with(|c| c.get()) }
thread_local! { pub static S14D: std::cell::Cell<[i64; 11]> = const { std::cell::Cell::new([0; 11]) }; }
/// [dffa10 v, decay k, decay 후 v, e03360 st, 1차 buff, 2차 buff]
pub fn s14_diag() -> [i64; 11] { S14D.with(|c| c.get()) }
pub unsafe fn s13_s14(b: &BCtx, ally: Option<usize>) -> Option<i64> {
    // ★S13D/S13E 도 함께 리셋 — 안 하면 다른 경로 표본에 직전 호출의 잔값이 찍혀 진단이 헛돌다(RE 2026-09-07)
    S14D.with(|c| c.set([0; 11])); AOED.with(|c| c.set([0; 12])); A0CH.with(|c| c.set([0; 4])); S13D.with(|c| c.set([0; 10])); S13E.with(|c| c.set([0; 12]));
    set_leaf_ctx(b.sim);
    let (sd, sv, _sin) = slot3(b.slot)?;
    let t = b.tgt;
    let (maxhp, hp) = (rd_u64(t + ENT_MAXHP)?, rd_u64(t + ENT_HP)?);
    let missing_raw = maxhp.wrapping_sub(hp);
    let heal0 = if hp <= maxhp { missing_raw } else { 0 };

    let heal_cap = slot_sum(sd, sv, 0x40, b.me, 0, "B40")?;
    let heal = heal0.min(heal_cap.max(0) as u64) as i64;
    let shield = slot_sum(sd, sv, 0x48, b.me, 0, "B48")?;
    let aura = slot_sum(sd, sv, 0xb0, b.me, 0, "Bb0")?;
    // ★`aura`(vt+0xb0) 는 버프항 게이트(`has || aura>0`)를 여는 값인데 로그에 없었다 —
    //   F1 계열(has=0·6항 전부 0·게임 main 9/26)의 유일한 남은 후보다(2026-09-07).
    S13D.with(|c| { let mut z = c.get(); z[8] = aura;
        // ★RE 권고: b0(=aura 원천) 뿐 아니라 48(shield)·98(aura_t 게이트) impl 도 함께 봐야 A/B 가 갈린다
        z[9] = (super::dyn_eff::impl_rva(sv, 0xb0).unwrap_or(0) as i64)
             | ((super::dyn_eff::impl_rva(sv, 0x48).unwrap_or(0) as i64) << 32);
        c.set(z); });

    // spec0 = 0xe047c0(slot, ctx, tgt) · has = spec0 있음 ∨ slot.vt+0xa0 의 tag != −1
    let spec0 = match e047c0(b.slot, b.ctx, t) { Some(v) => v, None => return na_tag("B047") };
    let a0 = spec_a0(sd, sv, b.me, 0)?;
    let has = spec0.is_some() || a0.is_some();
    // ★어느 vt+0xa0 impl 이 a0 을 만들었는지 — a0 이 전부 0 으로 나오는 표본의 원인 특정용(2026-09-07)
    S14D.with(|c| { let mut z = c.get(); z[10] = super::dyn_eff::impl_rva(sv, 0xa0).unwrap_or(0) as i64; c.set(z); });
    let b90 = slot_bool90(sd, sv, 0)?;
    // etc = (!has && aura<=0 && b90) ? (vt_a8()[0]==0 ? 5 : 0) : 0   ⬜vt+0xa8 미포팅
    let etc: i64 = if !has && aura <= 0 && b90 {
        // ★`slot_a8` 로 배선한다 — 합성형(`0x12a68e0`)·잎 계약이 거기 다 있는데
        //   ~~specemu 직접 호출~~ 이라 합성형에서 통째로 NA 가 났다(판당 572, RE 2026-09-07).
        match super::combat_score::slot_a8_pub(sd, sv, 0) {
            Some(Some(v)) => if v[0] == 0 { 5 } else { 0 },
            Some(None) => 5,
            None => { if let Some(r) = super::dyn_eff::impl_rva(sv, 0xa8) { super::dyn_eff::unseen(0x9a8, r); } return na_tag("Ba8"); }
        }
    } else { 0 };

    // inc / heal_e / shield_e
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
    S13E.with(|c| c.set([heal_e, shield_e, inc, 0, heal_e + shield_e, has as i64, ally.is_some() as i64, 0, heal, heal_cap, miss, shield]));
    // dffa10 의 8번째 인자(gate) — S13 은 bb.0x9a0, S14 는 아군 Record 의 0x88
    let gate = match ally { Some(ra) => rd_i64(ra + 0x88)?, None => rd_i64(b.bb + 0x9a0)? };
    // dffa10 의 대상/계수 — S13 은 self·C, S14 는 tgt·C_ally
    let (dtgt, dc) = match ally {
        Some(ra) => (t, super::as_callees::pct_c(b.bb, ra)?),
        None => (b.me, b.c),
    };

    // ── 버프 항 ──
    let mut buff: i64 = 0;
    let mut need_second = false;
    if has || aura > 0 {
        let spec = match spec0 { Some(x) => Some(x), None => a0 };
        match spec {
            Some(sp) => {
                let hs_needed = sp[0xf8] != 0 || sp[0x118] != 0 || sp[0xb8..0xc0].iter().any(|&x| x != 0);
                let hs = if hs_needed { Some(e01c40(b, dtgt)?) } else { None };
                let d = Dffa { self_e: dtgt, ctx: b.ctx, rec: b.rec, bb: b.bb, hs, inc, gate, aura, c: dc };
                let v = dffa10(&sp, &d)?;
                S14D.with(|c| { let mut z = c.get(); z[0] = v; c.set(z); });
                let v = if ally.is_some() { decay_s14(b, t, v)? } else { v };
                S14D.with(|c| { let mut z = c.get(); z[2] = v; c.set(z); });
                buff = e022d0(b.slot, b.ctx, b.rec, dtgt, v)?;
                S14D.with(|c| { let mut z = c.get(); z[4] = buff; c.set(z); });
                if has && buff == 0 { need_second = true; }
            }
            None => {
                if aura <= 0 { if !has { buff = 0; } else { need_second = true; } }
                else {
                    let z = [0u8; SPEC_SIZE];
                    let d = Dffa { self_e: dtgt, ctx: b.ctx, rec: b.rec, bb: b.bb, hs: None, inc, gate, aura, c: dc };
                    let v = dffa10(&z, &d)?;                    // ★E2 경로엔 /6 감쇠가 없다
                    buff = e022d0(b.slot, b.ctx, b.rec, dtgt, v)?;
                    if has && buff == 0 { need_second = true; }
                }
            }
        }
    }
    if need_second {
        // 2차 시도: S2 = spec0 또는 vt+0xa0 · (st,dd) = 0xe03360 · st==2 또는 tag −1 이면 buff = 0
        // ★★게임의 2단은 `vt+0xa0` 를 **새로 호출**하고 `spec0`(0xe047c0)을 **전혀 쓰지 않는다**
        //   (S13 2단 0xd5fec8 / S14 2단 0xd5f8cf 둘 다). ~~`spec0` 우선~~ 은 spec0 이 있는 표본에서
        //   재현이 더 큰 buff 를 내게 만든다 — S14 잔차 −46/−97 의 부호·크기와 맞는다(RE 2026-09-07).
        let s2 = a0;
        let (e3, e4) = if ally.is_some() { (b.me, t) } else { (b.me, b.me) };
        let st = e03360(b, e3, e4)?;
        S14D.with(|c| { let mut z = c.get(); z[3] = st as i64; c.set(z); });
        S14D.with(|c| { let mut z = c.get(); z[9] = 1; c.set(z); });   // need_second 진입 표식
        match s2 {
            None => buff = 0,
            Some(_) if st == 2 => buff = 0,
            Some(sp) => {
                let v2 = e02540_bytes(b.ctx, &sp, dtgt, st)?;
                buff = e022d0(b.slot, b.ctx, b.rec, dtgt, v2)?;
                // ★어떤 spec 이 쓰였고(1=spec0 / 2=a0) e02540 원값·spec 지문이 무엇인지 —
                //   "e02540 가 0" 이 spec 이 비어서인지 계산이 틀려서인지 가른다(RE 2026-09-07)
                let fp: i64 = [0x58usize, 0x5c, 0x60, 0x64, 0x80, 0x8c, 0x90, 0x104].iter()
                    .map(|&o| i32::from_le_bytes([sp[o], sp[o + 1], sp[o + 2], sp[o + 3]]) as i64).sum::<i64>()
                    + [0xa8usize, 0xb0, 0xc8].iter().map(|&o| sp_i64(&sp, o)).sum::<i64>();
                S14D.with(|c| { let mut z = c.get(); z[5] = buff; z[6] = if spec0.is_some() { 1 } else { 2 };
                                z[7] = v2; z[8] = fp; c.set(z); });
            }
        }
    }

    // ── 조립 ──
    // ★★분모는 **항상 `tgt.hp`** 다(`[rbp+0x898] = arg8[0x670]`, `0xd5e558`).
    //   ~~S13 에서 `b.me`~~ 는 self-target 표본에선 우연히 같지만 **비-self 갈래(0xd5e709)에서 틀린다**
    //   (RE 2026-09-07). `miss` 는 `me` 기준이 맞다 — 둘은 서로 다른 엔티티를 본다.
    let thp = rd_i64(t + ENT_HP)?;
    if thp == 0 { return None; }
    let total = heal_e.wrapping_add(shield_e);
    let mut hs_term = dc.wrapping_mul(total) / thp;
    if ally.is_some() && total > 0 && hs_term == 0 {
        let ra = ally.unwrap();
        let x = super::action_score::threat_sum(ra, b.cast_delay.wrapping_add(30))?.wrapping_add(rd_i64(ra + 0x98)?);
        hs_term = (rd_i64(ra + 0x70)? > 0 || x > 0 || rd_u64(t + ENT_MAXHP)? > rd_u64(t + ENT_HP)? || rd_i64(ra + 0x80)? != 0) as i64;
    }
    let atgt = if ally.is_some() { t } else { b.me };
    // ★self-skip 키(arg7)는 게임에서 `[rbp+0x618]` = **me+0x5c0 고정**이다 — 좌표(arg6)만 타깃을 따른다.
    //   ~~`atgt + ENT_HANDLE`~~ 은 S14(atgt=아군)에서 틀린다(RE 2026-09-07).
    let ah = rd_u64(b.me + ENT_HANDLE)?;
    let aoe = match e02bc0(b.slot, b.ctx, b.bb, b.me, atgt, ah, b.cast_delay) { Some(v) => v, None => return na_tag("B2bc0") };
    let main_raw = if ally.is_some() {
        aoe + etc + buff + hs_term
    } else {
        // S13 전용: 아군 아우라 루프 + 0xe03ed0
        let wroot = rd_u64(b.ctx)? as usize;
        let mut aura_t: i64 = 0;
        let (an, ap) = (rd_u64(b.bb + 0x14d0)?, rd_u64(b.bb + 0x14b8)? as usize);
        for i in 0..5usize {
            let e = rd_u64(wroot + X_ROSTER + (rd_u64(b.rec + 0x930)? as usize) * 0x28 + i * 8)? as usize; if e == 0 { continue; }
            if slot_i64_98(sd, sv, b.sim, b.me, e)? == 0 { continue; }
            let h = rd_u64(e + ENT_HANDLE)?;
            let mut rec_e = 0usize;
            if an != 0 { if !ptr_ok(ap) { return None; }
                for k in 0..an.min(CAP_ITER) as usize { let r = ap + k * 0xd8; if rd_u64(r + 0x58)? == h { rec_e = r; break; } } }
            if rec_e == 0 { continue; }
            if !e01c40(b, e)?.0 { continue; }
            aura_t += super::as_callees::pct_c(b.bb, rec_e)?.min(80);
        }
        let trig = match e03ed0(b.slot, b.ctx, b.rec, b.bb, b.me, b.c) { Some(v) => v, None => return na_tag("B3ed0") };
        S13D.with(|c| { let mut v = c.get(); v[1] = trig; v[2] = aura_t; c.set(v); });
        aoe + trig + aura_t + etc + hs_term + buff
    };
    S13D.with(|c| { let mut v = c.get(); v[0] = aoe; v[3] = etc; v[4] = hs_term; v[5] = buff; c.set(v); });
    S13E.with(|c| { let mut v = c.get(); v[3] = gate; v[7] = main_raw; c.set(v); });
    Some(if main_raw != 0 { main_raw } else if total > 0 || has { -10 } else { 0 })
}

/// slot.vt+0x98 : (inline, sim, self, 아군엔티티) -> i64. 단순 게터/위임만 처리.
unsafe fn slot_i64_98(data: usize, vt: usize, _sim: usize, me: usize, _e: usize) -> Option<i64> {
    slot_sum(data, vt, 0x98, me, 0, "B98")
}

/// S14 전용 도달시간 감쇠: `buff = e022d0(..., (k*v)/6)`, `k = min(6, max(0, 6 − t))`
unsafe fn decay_s14(b: &BCtx, tgt: usize, v: i64) -> Option<i64> {
    // k 는 아래에서 S14D[1] 에 기록한다
    let (sd2, sv, sin) = slot3(b.slot)?;
    let _ = sin;
    // slot.vt+0xe8(inline, self, tgt) — 이미 재현된 dn_reach::eff_e8 을 그대로 쓴다
    let e8 = match super::dn_reach::eff_e8(sd2, sv, b.me, tgt, 0) { Some(x) => x as i64, None => return na_tag("Be8") };
    let me = b.me;
    let reach = rd_i64(me + 0x438)?
        .wrapping_add(rd_i64(b.slot + 0x10)?)
        .wrapping_add(e8)
        .wrapping_add((rd_i64(me + 0x5c8)? - 1).wrapping_mul(rd_i64(b.slot + 0x18)?))
        .wrapping_add(body_radius(me)? as i64)
        .wrapping_add(body_radius(tgt)? as i64);
    let d = super::as_callees::isqrt_fast(
        { let (dx, dy) = (absd(rd_u64(me + ENT_X)?, rd_u64(tgt + ENT_X)?), absd(rd_u64(me + ENT_Y)?, rd_u64(tgt + ENT_Y)?));
          dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) }) as i64;
    let cfg = rd_u64(rd_u64(b.ctx + 8)? as usize + 8)? as usize;
    let tps = rd_i64(cfg + 0x12f8)?; if tps == 0 { return None; }
    let over = if d >= reach { (d - reach) as u64 } else { 0 };
    let t1 = (over / (rd_u64(me + 0x640)?.max(1))) as i64 / tps;
    let k = (6 - t1).max(0).min(6);
    S14D.with(|c| { let mut z = c.get(); z[1] = k; c.set(z); });
    Some(k.wrapping_mul(v) / 6)
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
pub unsafe fn e047c0(slot: usize, ctx: usize, me: usize) -> Option<Option<[u8; SPEC_SIZE]>> {
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
            for i in 0..n.min(CAP_ITER) as usize {
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
    let rec = super::action_score::sim_of_handle(w, rd_u64(me + ENT_HANDLE)?)?;
    if rec == 0 { return Some(None); }
    let mult = rd_u32(hit) as i32;                     // 매칭된 def 의 +0x00 = 퍼센트

    // 챔피언 어빌리티 전부의 BuffSpec 성분합 (전부 i32 wrapping) — 각 어빌리티는 `vt+0x78` sret
    const G: [usize; 16] = [0x58, 0x5c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x90, 0x104];
    let mut sum = [0i32; 16];
    let n = rd_u64(rec + 0x4a8)?;
    let arr = rd_u64(rec + 0x4a0)? as usize;
    if n != 0 && !ptr_ok(arr) { return None; }
    let (sim, est) = (SIM_TLS.with(|c| c.get()) as u64, EST_DESC_RVA_ABS.with(|c| c.get()));
    for i in 0..n.min(CAP_ITER) as usize {
        let e = arr + i * 0x10;
        let (ed, ev) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
        if !ptr_ok(ev) { return None; }
        let f = rd_u64(ev + 0x78)? as usize;
        let tmp = match super::specemu::run_spec_leaf(f, ed as u64, sim, me as u64, est) {
            Some(t) => t,
            None => { if let Some(r) = super::dyn_eff::impl_rva(ev, 0x78) { super::dyn_eff::unseen(0xa78, r); } return na_tag("B78").map(|_| None); }
        };
        for (k, o) in G.iter().enumerate() {
            sum[k] = sum[k].wrapping_add(i32::from_le_bytes([tmp[*o], tmp[*o + 1], tmp[*o + 2], tmp[*o + 3]]));
        }
    }
    let mut out = [0u8; SPEC_SIZE];
    for (k, o) in G.iter().enumerate() {
        let v = sum[k].wrapping_mul(mult) / 100;
        out[*o..*o + 4].copy_from_slice(&v.to_le_bytes());
    }
    // tag A = 0, tag B = 0 (e047c0 은 성공 시 +0x48 에 0 을 쓴다)
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
    for i in 0..n.min(CAP_ITER) as usize {
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
    let e3 = |k: i64, a: i64, b: i64, c2: i64| { E3D.with(|z| { let mut v = z.get(); v[0] = k; v[1] = a; v[2] = b; v[3] = c2; z.set(v); }); };
    let (def, dvt) = match slot_def_b8(sd, sv) { Some(v) => v, None => { e3(1, 0, 0, 0); return None } };
    if def == 0 { e3(2, 0, 0, 0); return Some(0); }
    // TypeId 가 표식형이 아니면 컨테이너 1단계만 훑어 표식형 자식을 찾는다
    let hit = if typeid_rva(dvt)? == TID_SPIRIT { def } else {
        if typeid_rva(dvt)? != TID_CONT { e3(3, 1, typeid_rva(dvt)? as i64, 0); return Some(0); }
        let n = rd_u64(def + 0x10)?; if n == 0 { e3(4, 1, typeid_rva(dvt)? as i64, 0); return Some(0); }
        let arr = rd_u64(def + 8)? as usize; if !ptr_ok(arr) { return None; }
        let mut found = 0usize;
        for i in 0..n.min(CAP_ITER) as usize {
            let (cd, cv) = (rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { continue; }
            let (d2, dv2) = match slot_def_b8(cd, cv) { Some(v) => v, None => continue };
            if d2 == 0 { continue; }
            if typeid_rva(dv2)? == TID_SPIRIT { found = d2; break; }
        }
        if found == 0 { e3(5, 1, typeid_rva(dvt)? as i64, n as i64); return Some(0); }
        found
    };

    let st = rd_u64(me + 0x620)?;                       // 주문력(추정) — RE §1 의 `self.0x620`
    e3(9, 1, typeid_rva(dvt).unwrap_or(0) as i64, 0);
    let amt1 = rd_i64(hit + 0x08)?.wrapping_add((rd_u64(hit + 0x10)?.wrapping_mul(st) / 100) as i64);
    let amt2 = rd_i64(hit + 0x30)?.wrapping_add((rd_u64(hit + 0x38)?.wrapping_mul(st) / 100) as i64);
    let r = rd_u64(hit)?; let r2 = r.wrapping_mul(r);
    let side = rd_u64(rec + 0x930)?; if side > 1 { return None; }
    let w = rd_u64(ctx)? as usize; if !ptr_ok(w) { return None; }
    let (sx, sy) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    let (mut sum, mut cnt): (i64, u64) = (0, 0);
    E3D.with(|z| { let mut v = z.get(); v[4] = r as i64; v[5] = st as i64; z.set(v); });

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
            for k in 0..rn.min(CAP_ITER) as usize {
                let r = rp + k * 0xd8;
                if rd_u64(r + 0x58)? == h { mult = super::as_callees::pct_c(bb, r)?; break; }
            }
        }
        let hp = rd_i64(e + ENT_HP)?;
        let den = if hp < 2 { 1 } else { hp };
        sum = sum.wrapping_add(amt2.min(hp).wrapping_mul(mult) / den);
    }
    E3D.with(|z| { let mut v = z.get(); v[6] = sum; v[7] = cnt as i64; z.set(v); });
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
                for i in 0..n.min(CAP_ITER) as usize {
                    let e = arr + i * 0x10;
                    let (cd, cv) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
                    if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
                    let (k, r) = slot_d0(cd, cv)?;
                    if k & 1 != 0 { return Some((k, r)); }
                }
                return Some((0, 0));
            }
            0x9db70 => return Some((0, 0)),          // xor eax,eax; ret
            // p.0x10 != 0 → 없음 · p.0x58(u32) 가 2 이거나 3 초과면 없음 · 아니면 (1, p.0x18)
            0x1145e30 => {
                if rd_u64(p + 0x10)? != 0 { return Some((0, 0)); }
                let k = rd_u32(p + 0x58);
                if k > 3 || k == 2 { return Some((0, 0)); }
                return Some((1, rd_u64(p + 0x18)?));
            }
            // (kind = (p.u32 == 3), radius = p.0x08)
            0x108dc10 => return Some(((rd_u32(p) == 3) as u64, rd_u64(p + 8)?)),
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
    // ★kind 와 heal_cap 이 **둘 다** p(=inline_self) 에서 읽히는데 둘 다 0 이면 원인은 p 자체다.
    //   data / p / vt RVA / p 주변 u32 를 같이 찍어 오프셋을 특정한다(2026-09-07).
    {   let pp = inline_self(sd, sv).unwrap_or(0);
        let vr = crate::exe_base(); let vrv = if vr != 0 && sv > vr { (sv - vr) as i64 } else { 0 };
        let ir = |slot: usize| -> i64 { super::dyn_eff::impl_rva(sv, slot).unwrap_or(0) as i64 };
        AOED.with(|c| c.set([kind as i64, aoe_r as i64, n as i64, 0, 0, 0, 0, 0,
                             (pp as i64).wrapping_sub(sd as i64), vrv,
                             ir(0xd0), ir(0x40)])); }
    if kind != 1 || n == 0 { return Some(0); }
    let arr = rd_u64(bb + 0x14b8)? as usize; if !ptr_ok(arr) { return None; }
    let w = rd_u64(ctx)? as usize; if !ptr_ok(w) { return None; }
    let wd = rd_u64(w)? as usize; let wv = rd_u64(w + 8)? as usize;
    if !ptr_ok(wd) || !ptr_ok(wv) { return None; }
    let wr = World { x: w, data: wd, vt: wv };
    let (cx, cy) = (rd_u64(tgt + ENT_X)?, rd_u64(tgt + ENT_Y)?);
    let mut acc: i64 = 0;
    for i in 0..n.min(CAP_ITER) as usize {
        let rec = arr + i * 0xd8;
        let h = rd_u64(rec + 0x58)?;
        if h == tgt_h { AOED.with(|c| { let mut z = c.get(); z[3] += 1; c.set(z); }); continue; }
        let e = match wr.entity(h) { Some(x) => x.0,
            None => { AOED.with(|c| { let mut z = c.get(); z[4] += 1; c.set(z); }); continue } };
        let r = body_radius(e)?.wrapping_add(aoe_r);
        let (ex, ey) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
        let (dx, dy) = (absd(ex, cx), absd(ey, cy));
        if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) > r.wrapping_mul(r) { AOED.with(|c| { let mut z = c.get(); z[5] += 1; c.set(z); }); continue; }
        let heal_cap = slot_sum(sd, sv, 0x40, me, 0, "B40")?;
        let heal = rd_u64(e + ENT_MAXHP)?.saturating_sub(rd_u64(e + ENT_HP)?).min(heal_cap.max(0) as u64) as i64;
        let x = super::action_score::threat_sum(rec, cast_delay.wrapping_add(30))?
            .wrapping_add(rd_i64(rec + 0x70)?).wrapping_add(rd_i64(rec + 0x88)?);
        let shield = slot_sum(sd, sv, 0x48, me, 0, "B48")?.min(3 * x);
        let total = shield.wrapping_add(heal);
        AOED.with(|c| { let mut z = c.get(); z[6] = heal_cap; z[7] = total; c.set(z); });
        if total <= 0 { continue; }
        let ce = super::as_callees::pct_c(bb, rec)?;
        let hp = rd_i64(e + ENT_HP)?;
        let den = if hp < 2 { 1 } else { hp };
        acc = acc.wrapping_add(ce.wrapping_mul(total) / den);
    }
    Some(acc)
}

// ── 0xdffa10 — 버프 가치 평가기 (6,720B) ─────────────────────────────────────────────────
//   정본 = `RE\2026-09-07_buff_value-0xdffa10-전수해독-0.5.8.md`
//   `(spec, self, ctx, rec, bb, hs_opt, inc, gate, aura, C) -> i64`, 최종 clamp(0,160)
#[inline] fn sum_n(base: usize, off: usize, n: usize) -> Option<u64> {
    let mut a = 0u64; for i in 0..n { a = a.wrapping_add(unsafe { rd_u64(base + off + i * 8) }?); } Some(a)
}
/// `self.0x578` 의 vt+0x90 = 평타 base 쿨. 단순 게터 impl 만 처리(그 외 None).
unsafe fn basic_cool(me: usize) -> Option<u64> {
    let (d, v) = (rd_u64(me + 0x570)? as usize, rd_u64(me + 0x578)? as usize);
    if !ptr_ok(v) { return None; }
    let f = rd_u64(v + 0x90)? as usize;
    if let Some(x) = super::as_callees::decode_getter(f, d) { return Some(x); }
    if let Some(r) = super::dyn_eff::impl_rva(v, 0x90) { super::dyn_eff::unseen(0xb90, r); }
    None
}

pub struct Dffa {
    pub self_e: usize, pub ctx: usize, pub rec: usize, pub bb: usize,
    pub hs: Option<(bool, bool)>, pub inc: i64, pub gate: i64, pub aura: i64, pub c: i64,
}

pub unsafe fn dffa10(spec: &[u8; SPEC_SIZE], a: &Dffa) -> Option<i64> {
    let s32 = |o: usize| -> i64 { i32::from_le_bytes([spec[o], spec[o + 1], spec[o + 2], spec[o + 3]]) as i64 };
    let s64 = |o: usize| -> i64 { let mut b = [0u8; 8]; b.copy_from_slice(&spec[o..o + 8]); i64::from_le_bytes(b) };
    let me = a.self_e;
    let cfg = rd_u64(rd_u64(a.ctx + 8)? as usize + 8)? as usize; if !ptr_ok(cfg) { return None; }
    let x = rd_i64(cfg + 0x12f8)?;                                   // tps
    let mut dur: i64 = 6;
    if s32(0x48) == 1 { if x == 0 { return None; } dur = (s64(0x50) / x).clamp(1, 6); }

    let sim = rd_u64(a.ctx)? as usize; if !ptr_ok(sim) { return None; }
    let r = super::action_score::sim_of_handle(sim, rd_u64(me + ENT_HANDLE)?)?;
    let (av, bv, cv) = if r == 0 { (0u64, 0u64, 0u64) } else {
        let t = rd_u64(r + 0x930)?; if t > 1 { return None; }
        let pos = rd_u32(r + 0x9c0) as usize;
        let blk = sim + 0x410 + (t as usize) * 0xfa0 + pos * 0x320;
        (sum_n(blk, 0, 5)? / 5, sum_n(blk, 0x28, 10)? / 5, sum_n(blk, 0x78, 5)? / 5)
    };
    let (av, bv, cv) = (av as i64, bv as i64, cv as i64);

    let myteam = rd_u64(a.rec + 0x930)?; if myteam > 1 { return None; }
    let opp = 1 - myteam;
    let (hpmax, arm, mr) = (rd_i64(me + 0x628)?, rd_i64(me + 0x630)?, rd_i64(me + 0x638)?);
    let (mut sh, mut sa, mut sm, mut cnt) = (0i64, 0i64, 0i64, 0i64);
    for i in 0..5usize {
        let e = rd_u64(sim + 0x1e0 + (opp as usize) * 0x28 + i * 8)? as usize; if e == 0 { continue; }
        sh += rd_i64(e + 0x628)?; sa += rd_i64(e + 0x630)?; sm += rd_i64(e + 0x638)?; cnt += 1;
    }
    let (ehp, earm, emr) = if cnt > 0 { (sh / cnt, sa / cnt, sm / cnt) } else { (sh, sa, sm) };

    // ── 공격 가치 raw ──
    let mut raw: i64 = 0;
    if s32(0x5c) > 0 { raw += s32(0x5c) * av / 100; }
    if s32(0x58) > 0 { raw += s32(0x58) * av / rd_i64(me + 0x618)?.max(1); }
    if s32(0x8c) > 0 { raw += av * s32(0x8c) / 100; }
    if s32(0x104) > 0 { raw += s32(0x104) * av / 100; }
    if s32(0x60) > 0 { raw += (cv + bv) * s32(0x60) / rd_i64(me + 0x620)?.max(1); }
    if s32(0x64) > 0 { raw += (bv + cv) * s32(0x64) / 100; }
    if cnt > 0 && s64(0xa8) != 0 { let af = (100 - s64(0xa8)).max(0) * earm / 100; raw += (earm - af) * av / (af.max(-99) + 100); }
    if cnt > 0 && s64(0xb0) != 0 { let af = (100 - s64(0xb0)).max(0) * emr / 100; raw += (emr - af) * (cv + bv) / (af.max(-99) + 100); }
    if s64(0xf0) != 0 { raw += s64(0xf0) * bv / 200; }
    if s64(0xc8) != 0 {
        let den = if rd_i32(me + 0x4c0)? == -1 { 1 } else {
            let s = rd_i64(me + 0x438)? + rd_i64(me + 0x4a0)? + (rd_i64(me + 0x5c8)? - 1) * rd_i64(me + 0x4a8)?;
            s + (s == 0) as i64
        };
        raw += s64(0xc8) * av / den;
    }
    if s32(0x100) > 0 { raw += s32(0x100) * bv / 200; }
    raw *= dur;

    // 평타 횟수 k (호출마다 재계산 — 게임도 vtable 을 다시 부른다)
    let kf = || -> Option<i64> {
        let cool = basic_cool(me)? as i64;
        let itv = (cool * 100 / (rd_i64(me + 0x3fc)? + 100).max(1)).max(3);
        Some((dur * x / itv.max(1)).max(1))
    };
    if a.aura > 0 { raw += kf()? * a.aura; }
    if cnt > 0 && s64(0xd0) != 0 { raw += (s64(0xd0) * ehp / 100) * kf()?; }
    if cnt > 0 && s64(0xe0) != 0 { raw += s64(0xe0) * ehp / 100; }
    if s64(0xd8) != 0 { raw += (s64(0xd8) * hpmax / 100) * kf()?; }
    if s64(0xc0) != 0 {
        let mut d = 0i64;
        for pos in 0..5usize { d += (sum_n(sim + 0x4b0 + (opp as usize) * 0xfa0 + pos * 0x320, 0, 10)? / 5) as i64; }
        raw += (d * s64(0xc0) / 100) * dur;
    }
    if s64(0xa0) != 0 {
        let mut n = 0i64;
        let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
        for i in 0..5usize {
            let e = rd_u64(sim + 0x1e0 + (myteam as usize) * 0x28 + i * 8)? as usize; if e == 0 { continue; }
            let (dx, dy) = (absd(rd_u64(e + ENT_X)?, mx), absd(rd_u64(e + ENT_Y)?, my));
            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) <= D2_120K - 1 { n += 1; }
        }
        raw += n.min(3) * dur * ((cv + bv + av) * s64(0xa0) / 100);
    }

    // ── 앵커 정규화 (RE `2026-09-07_dffa10-앵커블록-확정-0.5.8.md`) ──
    //   ★`gate <= 0` 은 **함수를 끝내지 않는다** — result 만 0 으로 두고 아래 (5)(6)(7) 을 계속 더한다.
    let rng_e = |e: usize| -> Option<i64> {
        let v = rd_i32(e + 0x470)? as i64;
        Some(if v == 0 { rd_i64(e + 0x680)? } else { ((v + 100).wrapping_mul(rd_i64(e + 0x680)?) as u64 / 100) as i64 })
    };
    let reach_of = |e: usize| -> Option<i64> {
        let base = if rd_i32(e + 0x4c0)? == -1 { 0 }
                   else { rd_i64(e + 0x438)? + rd_i64(e + 0x4a0)? + (rd_i64(e + 0x5c8)? - 1) * rd_i64(e + 0x4a8)? };
        Some(base + rd_i64(e + 0x640)? * 120 + rng_e(e)?)
    };
    let mut result: i64 = 0;
    if raw > 0 {
        let self_reach = reach_of(me)?;
        let ally = rd_u64(sim + 0x1e0 + (myteam as usize) * 0x28 + (rd_u32(a.rec + 0x9c0) as usize) * 8)? as usize;
        if ally == 0 { return None; }                       // 게임은 여기서 패닉(buff_value.rs:322)
        let ally_reach = reach_of(ally)?;
        let (wd, wv) = (rd_u64(sim)? as usize, rd_u64(sim + 8)? as usize);
        if !ptr_ok(wd) || !ptr_ok(wv) { return None; }
        let w = World { x: sim, data: wd, vt: wv };
        let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
        let (ax0, ay0) = (rd_u64(ally + ENT_X)?, rd_u64(ally + ENT_Y)?);
        let n = rd_u64(a.bb + BV_ENEMY_LEN)?;
        let mut best: Option<(i64, i64)> = None;
        if n != 0 {
            let arr = rd_u64(a.bb + BV_ENEMY_PTR)? as usize; if !ptr_ok(arr) { return None; }
            for i in 0..n.min(CAP_ITER) as usize {
                let elem = arr + i * 0xd8;
                let h = rd_u64(elem + 0x58)?;
                let e = match w.entity(h) { Some(x) => x.0, None => continue };
                let r = rng_e(e)? as u64;
                let (ex, ey) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
                let (dx, dy) = (absd(ex, mx), absd(ey, my));
                let rs = (self_reach as u64).wrapping_add(r);
                let ok = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) <= rs.wrapping_mul(rs) || {
                    let (bx, by) = (absd(ex, ax0), absd(ey, ay0));
                    let ra = r.wrapping_add(ally_reach as u64);
                    bx.wrapping_mul(bx).wrapping_add(by.wrapping_mul(by)) <= ra.wrapping_mul(ra)
                };
                if !ok { continue; }
                let v = super::as_callees::pct_c(a.bb, elem)?;
                let hp = rd_i64(e + ENT_HP)?;
                if hp < 1 { continue; }
                match best { Some((bv, _)) if bv > v => {} _ => best = Some((v, hp)) }   // 동률이면 나중 원소
            }
        }
        result = match best {
            Some((v, hp)) => v.wrapping_mul(raw) / if hp >= 2 { hp } else { 1 },
            None => if a.gate <= 0 { 0 } else { a.c.wrapping_mul(raw) / if hpmax >= 2 { hpmax } else { 1 } },
        };
    }

    S13D.with(|c| { let mut v = c.get(); v[6] = raw; v[7] = dur; c.set(v); });
    // ── 방어/유틸 가치 ──
    if s32(0x90) > 0 || s32(0xfc) > 0 {
        let mut n = 0i64;
        let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
        for i in 0..5usize {
            let e = rd_u64(sim + 0x1e0 + (myteam as usize) * 0x28 + i * 8)? as usize; if e == 0 { continue; }
            let (dx, dy) = (absd(rd_u64(e + ENT_X)?, mx), absd(rd_u64(e + ENT_Y)?, my));
            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < D2_60K { n += 1; }
        }
        if n >= 2 {
            let mut v = 0i64;
            if s32(0x90) > 0 { v = (bv + cv) * s32(0x90) / (s32(0x90) + 100); }
            if s32(0xfc) > 0 { v += cv * s32(0xfc) / (s32(0xfc) + 100); }
            result += dur * a.c * v / rd_i64(me + ENT_HP)?.max(1);
        }
    }
    let heal = (if s32(0x80) > 0 { (s32(0x80) * av / 100) * dur } else { 0 })
             + (if s32(0x74) > 0 { dur * s32(0x74) } else { 0 });
    let capped = heal.min((hpmax - rd_i64(me + ENT_HP)?).max(0) + a.inc);
    if capped > 0 { result += capped * a.c / rd_i64(me + ENT_HP)?.max(1); }

    if a.inc > 0 {
        let mut d = 0i64;
        let da = s32(0x6c) * arm / 100 + s32(0x68);
        if da != 0 { d = (a.inc >> 1) * da / (da + arm + 100).max(1); }
        let dm = s32(0x7c) * mr / 100 + s32(0x78);
        if dm != 0 { d += (a.inc >> 1) * dm / (dm + mr + 100).max(1); }
        if s64(0xe8) != 0 { d += s64(0xe8) * a.inc / 100; }
        if s64(0x108) != 0 { d += s64(0x108) * (a.inc >> 1) / 100; }
        if s64(0x110) != 0 { d += s64(0x110) * (a.inc >> 1) / 100; }
        if s64(0x98) != 0 { d += s64(0x98) * a.inc / 100; }
        let sh2 = hpmax * s32(0x84) / 100 + s32(0x70);
        let sh2 = if sh2 > 0 { sh2.min(a.inc) } else { 0 };
        if sh2 + d > 0 { result += (sh2 + d) * a.c / rd_i64(me + ENT_HP)?.max(1); }
    }

    // ── 오더 게이트 보너스 ── (ctx[2] = agents. o 의 의미는 추정이나 식은 확정)
    if s32(0x88) > 0 || (spec[0x119] & 1) != 0 {
        let agents = rd_u64(a.ctx + 0x10)? as usize;
        let r2 = super::action_score::sim_of_handle(sim, rd_u64(me + ENT_HANDLE)?)?;
        if r2 != 0 && ptr_ok(agents) {
            let t = rd_u64(r2 + 0x930)?; if t > 1 { return None; }
            let o = rd_i64(agents + (t as usize) * 0x2e8 + 0x78 + (rd_u32(r2 + 0x9c0) as usize) * 0x18)?;
            let ok = (o == 0 && a.inc >= 1) || (o == 4 && rd_u64(a.bb + BV_ENEMY_LEN)? != 0);
            if ok { result += (s32(0x88) + if spec[0x119] & 1 != 0 { 10 } else { 0 }) * a.c / 100; }
        }
    }

    if let Some((h0, h1)) = a.hs {
        if spec[0x118] != 0 { result += if h0 { a.c } else { 0 }; }
        if spec[0xf8] != 0 && h1 { result += if !h0 { a.c / 3 } else { a.c }; }
        if s64(0xb8) != 0 && h1 { result += s64(0xb8) * a.c / 200; }
    }
    Some(result.clamp(0, 160))
}

// ── 0xe01c40 — (반격 여부, 적 대형기 준비) (897B) ────────────────────────────────────────
//   정본 = `RE\2026-09-07_combat_score-S13S14-버프콜리4종-…-0.5.8.md` §3
//   `(mode, prof, rec, ctx, subject, p9) -> (al, dl)` — 호출부는 `&1` 로만 쓴다.
const FOUNTAIN: [(u64, u64); 2] = [(64000, 800000), (160000, 896000)];

/// 술어 `0xe11e90`
unsafe fn e11e90(w: &World, agents: usize, rec: usize, subject: usize, e: usize, now: u64) -> Option<bool> {
    let reach = super::as_callees::max_reach(e, subject)?;
    let (ex, ey) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
    let (sx, sy) = (rd_u64(subject + ENT_X)?, rd_u64(subject + ENT_Y)?);
    let (dx, dy) = (absd(ex, sx), absd(ey, sy));
    let lim = reach.wrapping_add(30000);
    if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) > lim.wrapping_mul(lim) { return Some(false); }
    let side = rd_u64(rec + 0x930)?; if side > 1 { return None; }
    let eside = 1 - side;
    if rd_u8(e) == 0 && rd_u64(e + 8)? == eside {
        for (a, bnd) in FOUNTAIN {
            if side == 1 { if ex <= a && (bnd..=960000).contains(&ey) { return Some(false); } }
            else { if (bnd..=960000).contains(&ex) && ey <= a { return Some(false); } }
        }
    }
    let h = rd_u64(e + ENT_HANDLE)?;
    if w.visible(side, h)? { return Some(true); }
    let rc = w.roster_rec(h)?; if rc == 0 { return Some(false); }
    let seen = rd_u64(agents + (eside as usize) * LANE_STRIDE + LANE_ROSTER + (rd_u32(rc + 0x9c0) as usize) * 8)?;
    Some(now <= seen.wrapping_add(120))
}

/// 스킬 슬롯의 `vt+0x88` == 1 인가(대형기 준비). 단순 게터 impl 만 처리.
unsafe fn skill_ready88(slot: usize) -> Option<bool> {
    let (d, v) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    if !ptr_ok(v) { return None; }
    // ★combat_score::slot_88 이 이 슬롯의 전체 impl 표(리스트 삼중항·max 의미)를 갖고 있다 — 재사용.
    //   여기서는 플래그만 쓰므로 값(max)은 버린다.
    super::combat_score::slot_88(d, v, 0).map(|(ok, _)| ok)
}

pub unsafe fn e01c40(b: &BCtx, subject: usize) -> Option<(bool, bool)> {
    let wroot = rd_u64(b.ctx)? as usize;
    let (wd, wv) = (rd_u64(wroot)? as usize, rd_u64(wroot + 8)? as usize);
    if !ptr_ok(wd) || !ptr_ok(wv) { return None; }
    let w = World { x: wroot, data: wd, vt: wv };
    let agents = rd_u64(b.ctx + 0x10)? as usize; if !ptr_ok(agents) { return None; }
    let cfg = rd_u64(b.sim + 8)? as usize; if !ptr_ok(cfg) { return None; }
    let tps = rd_u64(cfg + 0x12f8)?;
    let now = rd_u64(wd + W_TICK)?;
    let side = rd_u64(b.rec + 0x930)?; if side > 1 { return None; }
    let eside = 1 - side;

    let mut list = [0usize; 5]; let mut n = 0usize;
    for i in 0..5usize {
        let e = rd_u64(wroot + X_ROSTER + (eside as usize) * 0x28 + i * 8)? as usize; if e == 0 { continue; }
        if e11e90(&w, agents, b.rec, subject, e, now)? { list[n] = e; n += 1; }
    }
    if n == 0 { return Some((false, false)); }

    // ① fights_back
    let rr = super::action_score::sim_of_handle(wroot, rd_u64(subject + ENT_HANDLE)?)?;
    let mut fights_back = false;
    if rr != 0 {
        let mut hdr = [0u64; 4]; hdr[0] = list.as_ptr() as u64; hdr[3] = n as u64;
        let emp = [0u64; 4];
        let t = super::fight_check::fight_check_memo(0, b.ctx, rr, subject, hdr.as_ptr() as usize, emp.as_ptr() as usize)?;
        fights_back = t < 2u64.wrapping_mul(tps);
    }
    // ② big_ready
    let mut big = false;
    for k in 0..n {
        let e = list[k];
        let kind = rd_i32(e + ENT_KIND)?;
        let lv = rd_u64(e + 0x5c8)?;
        let (c1, c2, c3) = if kind == 0xd { (rd_u64(e + 0xb8)?, rd_u64(e + 0xc0)?, rd_u64(e + 0xc8)?) } else { (0, 0, 0) };
        let ready1 = if kind == 0xd { c1 <= tps } else { true };
        if ready1 && rd_i32(e + 0x4f8)? != -1 && skill_ready88(e + 0x4c8)? { big = true; break; }
        let s2 = if lv >= 3 { e + 0x500 } else { crate::exe_base() + 0x33e21a0 };
        if c2 <= tps && rd_i32(s2 + 0x30)? != -1 && skill_ready88(s2)? { big = true; break; }
        let s3 = if lv >= 5 { e + 0x538 } else { crate::exe_base() + 0x33e21a0 };
        if c3 <= tps && rd_i32(s3 + 0x30)? != -1 && skill_ready88(s3)? { big = true; break; }
    }
    Some((fights_back, big))
}

// ── 0xe03360 — 공격 대상 종류 판정 (2,922B) ──────────────────────────────────────────────
//   정본 = `RE\2026-09-07_BuffSpec레이아웃-0xe047c0-0xe03360-0xe02540-전수해독-0.5.8.md` §3
//   반환 EAX = 0/1/2. 호출부는 `st == 2` 면 그 후보 점수를 0 으로 확정한다.
//   ①적 챔프가 위협 사거리 안 + (가시 ∨ 120틱 내 목격) → 2   ②self 무기 없음 → 2
//   ③중립/포탑/미니언/넥서스/유닛리스트는 술어 `0xdef0a0` 필요 — ⬜미포팅
pub unsafe fn e03360(b: &BCtx, e3: usize, e4: usize) -> Option<u8> {
    let wroot = rd_u64(b.ctx)? as usize;
    let (wd, wv) = (rd_u64(wroot)? as usize, rd_u64(wroot + 8)? as usize);
    if !ptr_ok(wd) || !ptr_ok(wv) { return None; }
    let w = World { x: wroot, data: wd, vt: wv };
    let agents = rd_u64(b.ctx + 0x10)? as usize; if !ptr_ok(agents) { return None; }
    let side = rd_u64(b.rec + 0x930)?; if side > 1 { return None; }
    let opp = 1 - side;
    let now = rd_u64(wd + W_TICK)?;

    let reach_of = |e: usize| -> Option<i64> {
        let base = if rd_i32(e + 0x4c0)? == -1 { 0 }
                   else { rd_i64(e + 0x438)? + rd_i64(e + 0x4a0)? + (rd_i64(e + 0x5c8)? - 1) * rd_i64(e + 0x4a8)? };
        Some(base + rd_i64(e + 0x640)? * 120 + body_radius(e)? as i64)
    };
    let no_wep = rd_i32(e4 + 0x4c0)? == -1;
    let r4 = reach_of(e4)? as u64;
    let r3 = reach_of(e3)? as u64;
    let (e4x, e4y) = (rd_u64(e4 + ENT_X)?, rd_u64(e4 + ENT_Y)?);
    let (e3x, e3y) = (rd_u64(e3 + ENT_X)?, rd_u64(e3 + ENT_Y)?);

    // ① 적 챔피언 5슬롯
    for k in 0..5usize {
        let en = rd_u64(wroot + X_ROSTER + (opp as usize) * 0x28 + k * 8)? as usize; if en == 0 { continue; }
        let re = body_radius(en)?;
        let (ex, ey) = (rd_u64(en + ENT_X)?, rd_u64(en + ENT_Y)?);
        let (ax, ay) = (absd(ex, e4x), absd(ey, e4y));
        let lim_a = re.wrapping_add(r4);
        let in_a = ax.wrapping_mul(ax).wrapping_add(ay.wrapping_mul(ay)) <= lim_a.wrapping_mul(lim_a);
        let (bx, by) = (absd(ex, e3x), absd(ey, e3y));
        let lim_b = re.wrapping_add(r3);
        let in_b = bx.wrapping_mul(bx).wrapping_add(by.wrapping_mul(by)) <= lim_b.wrapping_mul(lim_b);
        if !in_a && !in_b { continue; }
        let h = rd_u64(en + ENT_HANDLE)?;
        if w.visible(side, h)? { return Some(2); }
        let rc = w.roster_rec(h)?;
        if rc != 0 {
            let t = rd_u64(agents + (opp as usize) * LANE_STRIDE + LANE_ROSTER + (rd_u32(rc + 0x9c0) as usize) * 8)?;
            if now <= t.wrapping_add(120) { return Some(2); }
        }
    }
    if no_wep { return Some(2); }

    // ② 중립 리스트(팀 무관) — 통과하면 1
    let n0 = rd_u64(wroot + 0xe8)?;
    if n0 != 0 {
        let p0 = rd_u64(wroot + 0xd0)? as usize; if !ptr_ok(p0) { return None; }
        for i in 0..n0.min(CAP_ITER) as usize {
            let u = rd_u64(p0 + i * 8)? as usize; if u == 0 { continue; }
            if alive_target(u)? && order_pred(b, e4, u)? { return Some(1); }
        }
    }
    // ③ 적 고정 6슬롯(포탑) — 검사 순서 고정
    for off in [0x180usize, 0x1a0, 0x1c0, 0x190, 0x1b0, 0x1d0] {
        let s0 = rd_u64(wroot + off + (opp as usize) * 8)? as usize; if s0 == 0 { continue; }
        if alive_target(s0)? && order_pred(b, e4, s0)? { return Some(0); }
    }
    // ④ 적 미니언
    let nm = rd_u64(wroot + X_MINION_LEN + (opp as usize) * 0x20)?;
    if nm != 0 {
        let pm = rd_u64(wroot + X_MINION_PTR + (opp as usize) * 0x20)? as usize; if !ptr_ok(pm) { return None; }
        for i in 0..nm.min(CAP_ITER) as usize {
            let m = rd_u64(pm + i * 8)? as usize; if m == 0 { continue; }
            if alive_target(m)? && order_pred(b, e4, m)? { return Some(0); }
        }
    }
    // ⑤ 적 넥서스
    let nx = rd_u64(wroot + X_NEXUS + (opp as usize) * 8)? as usize;
    if nx != 0 && alive_target(nx)? && order_pred(b, e4, nx)? { return Some(0); }
    // ⑥ 폴백: 적 유닛리스트 3종
    for (po, lo) in [(0x10usize, 0x28usize), (0x50, 0x68), (0x90, 0xa8)] {
        let n = rd_u64(wroot + lo + (opp as usize) * 0x20)?;
        if n == 0 { continue; }
        let p = rd_u64(wroot + po + (opp as usize) * 0x20)? as usize; if !ptr_ok(p) { return None; }
        for i in 0..n.min(CAP_ITER) as usize {
            let u = rd_u64(p + i * 8)? as usize; if u == 0 { continue; }
            // ★게임은 술어 앞에 `alive_target` 을 본다(`0xd71823`/`0xd7182c`) — 재현에 빠져 있었다
            if rd_u8(u + 0x6b9) != 1 || rd_u64(u + 0x6a0)? != 0 { continue; }
            if order_pred(b, e4, u)? { return Some(0); }
        }
    }
    Some(2)
}

// ── 0xdef0a0 — "우리 팀 누군가의 오더 타깃이고 내 사거리 안인가" 술어 ────────────────────
//   정본 = `RE\2026-09-07_def0a0-오더타깃술어-0.5.8.md`
//   오더 표: kind @ `lanes + side*0x2e8 + 0x78 + role*0x18` · target handle @ `+8`
//   kind ∈ {2,4,6,7,8,9} 만 타깃 비교(3·5 및 범위 밖은 그 role skip). role 4 는 불일치 시 즉시 false.
unsafe fn order_pred(b: &BCtx, me: usize, cand: usize) -> Option<bool> {
    let wroot = rd_u64(b.ctx)? as usize;
    let lanes = rd_u64(b.ctx + 0x10)? as usize; if !ptr_ok(lanes) { return None; }
    let side = rd_u64(b.rec + 0x930)?; if side > 1 { return None; }
    let h = rd_u64(cand + ENT_HANDLE)?;
    let mh = rd_u64(me + ENT_HANDLE)?;
    let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    let lane = lanes + (side as usize) * LANE_STRIDE;
    let mut hit = false;
    for role in 0..5usize {
        let ally = rd_u64(wroot + X_ROSTER + (side as usize) * 0x28 + role * 8)? as usize;
        if ally == 0 { continue; }
        if rd_u64(ally + ENT_HANDLE)? != mh {                      // 나 자신이면 거리 게이트 생략
            let (dx, dy) = (absd(rd_u64(ally + ENT_X)?, mx), absd(rd_u64(ally + ENT_Y)?, my));
            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) > 3_600_000_000 { continue; }
        }
        let kind = rd_u64(lane + 0x78 + role * 0x18)?;
        if kind.wrapping_sub(2) > 7 { continue; }
        if kind == 3 || kind == 5 { continue; }
        if rd_u64(lane + 0x80 + role * 0x18)? == h { hit = true; break; }
        if role == 4 { return Some(false); }                       // 마지막 role 만 즉시 false
    }
    if !hit { return Some(false); }
    // 사거리 = 0x438 + 0x4a0 + (lv−1)*0x4a8 + sz(me) + sz(cand) + vt+0xe8 + speed*30
    let s = me + 0x490;
    let (sd, sv) = (rd_u64(s)? as usize, rd_u64(s + 8)? as usize);
    if !ptr_ok(sv) { return None; }
    let bonus = super::dn_reach::eff_e8(sd, sv, me, cand, 0)?;
    let range = rd_u64(me + 0x438)?
        .wrapping_add(rd_u64(s + 0x10)?)
        .wrapping_add(rd_u64(me + 0x5c8)?.wrapping_sub(1).wrapping_mul(rd_u64(s + 0x18)?))
        .wrapping_add(body_radius(me)?)
        .wrapping_add(body_radius(cand)?)
        .wrapping_add(bonus)
        .wrapping_add(rd_u64(me + 0x640)?.wrapping_mul(30));
    let (dx, dy) = (absd(rd_u64(cand + ENT_X)?, mx), absd(rd_u64(cand + ENT_Y)?, my));
    Some(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) <= range.wrapping_mul(range))
}
#[inline] unsafe fn alive_target(e: usize) -> Option<bool> { Some(rd_u8(e + 0x6b9) == 1 && rd_u64(e + 0x6a0)? == 0) }
