//! dyn_eff — 이펙트(`Arc<dyn Effect>`, vt 291종)·프로바이더(`Box<dyn DataAction>`, vt 210종) 슬롯의 **구현 RVA 디스패치** 재현.
//!   규칙: 구현체 RVA(= `[vt+slot] − exe_base`)로 갈라 재현한다. 미재현 RVA 는 `None`(NA) 을 돌려주고 `unseen` 표에 (slot, rva, 횟수)를 남긴다
//!   → 리플레이 한 번이면 "실제로 불리는 구현" 목록이 `judge_dyn.txt` 에 나오고, 그것부터 포팅한다(291×5 슬롯 전부를 미리 옮기지 않는다).
//!   구현체 분포(0.5.8 정적 열거, scratchpad dyn_families.txt): +0x28 39종(0 ×152) · +0x30 1종(항상 None) · +0x38 12종(0 ×194) · +0x40 15종(0 ×197) · +0xa0 26종(None ×171) · 프로바이더 +0x90 19종.
//!   1차 리플레이(13:40) 실측 상위: +0x28 0x1708310(13만) · +0x40 0x12a5660(9.4만)/0x122e360(6.8만)/0x12a5ae0/0x1248040 · +0xa0 0x12266f0 → 전부 재현(아래).
//! ⚠페이로드 주소: Arc<dyn> 은 `data + ((align−1) & !0xf) + 0x10`(align=[vt+0x10]), Box<dyn>(프로바이더)은 data 그대로.
//!   인자 규약(effect 슬롯): (rcx=payload, rdx=W, r8=entity, r9=ENT_VT 0x33d8768). ENT_VT+0x38(e) = &e+0x618 스탯 스냅샷, +0x30 = 스냅샷 sret(같은 0x48B) → 직접 오프셋으로 읽는다.
#![allow(dead_code)]
use crate::*;
use super::super::layout::*;
use std::sync::atomic::{AtomicU64, Ordering};

// ── 미확인 구현 RVA 표(락 없음: 64칸 선형탐색, 키 = slot<<32 | rva) ──
const N: usize = 64;
static KEYS: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
static CNTS: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
pub fn unseen(slot: u32, rva: usize) {
    let key = ((slot as u64) << 32) | (rva as u64 & 0xffff_ffff);
    for i in 0..N {
        let k = KEYS[i].load(Ordering::Relaxed);
        if k == key { CNTS[i].fetch_add(1, Ordering::Relaxed); return; }
        if k == 0 { if KEYS[i].compare_exchange(0, key, Ordering::Relaxed, Ordering::Relaxed).is_ok() { CNTS[i].fetch_add(1, Ordering::Relaxed); return; } }
    }
}
pub fn unseen_report() -> String {
    let mut v: Vec<(u32, usize, u64)> = (0..N).filter_map(|i| { let k = KEYS[i].load(Ordering::Relaxed); if k == 0 { None } else { Some(((k >> 32) as u32, (k & 0xffff_ffff) as usize, CNTS[i].load(Ordering::Relaxed))) } }).collect();
    v.sort_by(|a, b| b.2.cmp(&a.2));
    let mut s = String::from("=== 미재현 dyn 구현체(slot, impl RVA, 호출 수) — 많은 순으로 포팅 ===\n");
    for (slot, rva, c) in v { s.push_str(&format!("slot+{:#x} impl {:#x} x{}\n", slot, rva, c)); }
    s
}

/// **합성 impl 골격 스캐너**(eff28/eff38 공용). 컴파일러가 모노몰픽 복제한 "자식 순회 후 합산" 형을
///   RVA 표 없이 해석한다. 규칙:
///     · 직전 call 이후 처음 나오는 **disp8 메모리 로드 2개**(SIB 없음) = (len_off, ptr_off)
///     · `call qword ptr [r?+SLOT]` 로 루프 확정(모든 루프의 SLOT 은 같아야 한다)
///     · 그 뒤 첫 `add r?, imm8` 이 stride
///   루프는 최대 2개까지. 그 밖의 형태면 None(=fail-closed).
///   실측 대상: `0x122e450`(2루프) · `0x13bfb60` · `0x12a51e0` · `0x1248270` · `0x1341090` 등.
pub unsafe fn composite_loops(f: usize, want_slot: usize) -> Option<([(usize, usize, usize); 2], usize)> {
    if !ptr_ok(f) { return None; }
    let mut lp = [(0usize, 0usize, 0usize); 2];
    let mut n = 0usize;
    let mut pend = [usize::MAX; 2]; let mut np = 0usize;
    let mut want_stride = false;
    let mut i = 0usize;
    while i < 0x180 {
        let a = f + i;
        if rd_u8(a) == 0xcc && rd_u8(a + 1) == 0xcc && rd_u8(a + 2) == 0xcc { break; }
        let p0 = rd_u8(a);
        // disp8 메모리 로드: 48/49/4c/4d 8b <mod=01, rm!=4> disp8
        if matches!(p0, 0x48 | 0x49 | 0x4c | 0x4d) && rd_u8(a + 1) == 0x8b {
            let m = rd_u8(a + 2);
            if (m >> 6) == 1 && (m & 7) != 4 {
                if !want_stride && np < 2 { pend[np] = rd_u8(a + 3) as usize; np += 1; }
                i += 4; continue;
            }
        }
        // add r?, imm8 (49/48 83 c? ib) — call 뒤 첫 개가 stride
        if matches!(p0, 0x48 | 0x49) && rd_u8(a + 1) == 0x83 && (rd_u8(a + 2) & 0xF8) == 0xC0 {
            if want_stride && n > 0 { lp[n - 1].2 = rd_u8(a + 3) as usize; want_stride = false; }
            i += 4; continue;
        }
        // call qword ptr [r8..r15 + disp8 / disp32]
        if p0 == 0x41 && rd_u8(a + 1) == 0xff {
            let m = rd_u8(a + 2);
            if (m & 0xF8) == 0x50 || (m & 0xF8) == 0x90 {
                let slot = if (m & 0xF8) == 0x50 { rd_u8(a + 3) as usize } else { rd_i32(a + 3)? as usize };
                if slot != want_slot || np != 2 || n >= 2 { return None; }
                lp[n] = (pend[0], pend[1], 0); n += 1;
                np = 0; pend = [usize::MAX; 2]; want_stride = true;
                i += if (m & 0xF8) == 0x50 { 4 } else { 7 }; continue;
            }
        }
        i += 1;
    }
    if n == 0 { return None; }
    for k in 0..n { if lp[k].2 != 0x10 && lp[k].2 != 0x18 { return None; } }
    Some((lp, n))
}
#[inline] pub unsafe fn impl_rva(vt: usize, slot: usize) -> Option<usize> {
    let b = exe_base(); if b == 0 || !ptr_ok(vt) { return None; }
    // ★.text 는 0x1000‥0x32d4600. 이전 상한(0x8000000)은 `.rdata` 를 통과시켜, 잘못 읽은 vt 에서 나온
    //   vtable 주소가 "미재현 구현체"로 둔갑했다(2026-09-07 RE). 이제 즉시 None 이 된다.
    let t = rd_u64(vt + slot)? as usize; if t <= b || t - b >= 0x32d4600 { return None; }
    Some(t - b)
}
#[inline] pub unsafe fn arc_payload(data: usize, vt: usize) -> Option<usize> {
    let align = rd_u64(vt + 0x10)?; Some(data.wrapping_add(((align.wrapping_sub(1)) & !0xfu64) as usize).wrapping_add(0x10))
}
/// 게임의 `shr rcx,2 → mul 0x28f5c28f5c28f5c3 → shr rdx,2` 는 **통째로 x/100** (매직 상수 = 2^68/100 → (x>>2)/25 = x/100).
/// ~~(x>>2)/100~~ 로 옮겼던 것이 피해량 1/4 오류의 원인(2026-09-06 캡처 교차검사: AD 8·계수 100 → 게임 8, 재현 2). 이름은 호출부 보존을 위해 유지.
#[inline] fn q400(x: u64) -> u64 { x / 100 }

/// 자식 (data, vt) 배열 합산 — Vec 원소 stride 가 0x10 또는 0x18(둘 다 [+0]=data, [+8]=vt)
unsafe fn sum_children_40(ptr: usize, len: u64, stride: usize, ent: usize, depth: u32) -> Option<u64> {
    if len == 0 { return Some(0); } if !ptr_ok(ptr) { return None; }
    let mut acc: u64 = 0;
    for i in 0..len.min(CAP_ITER) as usize {
        let (d, v) = (rd_u64(ptr + i * stride)? as usize, rd_u64(ptr + i * stride + 8)? as usize);
        acc = acc.wrapping_add(eff40_heal_d(d, v, ent, depth + 1)?);
    }
    Some(acc)
}

/// effect `+0x28` 피해 (rax=물리, rdx=마법). 인자 (payload, W, attacker, ENT_VT) — 재현본은 attacker 만 필요.
pub unsafe fn eff28_damage(data: usize, vt: usize, att: usize) -> Option<(u64, u64)> {
    let rva = impl_rva(vt, 0x28)?; let me = arc_payload(data, vt)?;
    match rva {
        EFF28_ZERO => Some((0, 0)),
        // ★0x146b2f0 은 EFF28_BASE_AD 와 계산식이 완전히 같다(RE 2026-09-07 census)
        EFF28_BASE_AD | 0x146b2f0 => Some((rd_u64(me)?.wrapping_add(q400(rd_u64(me + 8)?.wrapping_mul(rd_u64(att + ENT_STATS)?))), 0)),
        // ── ★specemu 가 분기·div 때문에 실행 못 하는 4종(네이티브 필수) — RE 2026-09-07 `eff28_slot_contracts` ──
        //   나머지 미포팅 잎은 arm 을 두지 않는다(에뮬이 실행 가능한 것을 손으로 옮기면 오히려 나빠진다).
        0x1145e60 => { let ad = rd_u64(att + ENT_STATS)?;                       // 판당 1,591 (최대)
            let d = rd_u64(me + 0x30)?; let n = rd_u64(me + 0x28)? / if d < 1 { 1 } else { d } + 1;   // ★몫 +1
            Some((rd_u64(me + 8)?.wrapping_add(q400(rd_u64(me + 0x10)?.wrapping_mul(ad))).wrapping_mul(n), 0)) }
        0x12267a0 => { let ap = rd_u64(att + ENT_STATS + 8)?;                   // ★+1 없음
            let d = rd_u64(me + 8)?; let n = rd_u64(me)? / if d < 1 { 1 } else { d };
            Some((0, rd_u64(me + 0x18)?.wrapping_add(q400(rd_u64(me + 0x20)?.wrapping_mul(ap))).wrapping_mul(n))) }
        0x1151400 => { let ap = rd_u64(att + ENT_STATS + 8)?;                   // ★분모 0 → k=0 (max 아님)
            let d = rd_u64(me + 0x20)?; let k = if d == 0 { 0 } else { rd_u64(me + 0x18)? / d };
            Some((0, rd_u64(me)?.wrapping_add(q400(rd_u64(me + 8)?.wrapping_mul(ap))).wrapping_mul(k))) }
        0x13c5680 => { let ad = rd_u64(att + ENT_STATS)?;                       // 힙에 임시 Arc 를 만들어 0x1708310 을 부른다
            Some((rd_u64(me + 0x20)?.wrapping_add(q400(rd_u64(me + 0x18)?.wrapping_mul(ad)))
                    .wrapping_mul(rd_u64(me + 0xa0)?), 0)) }
        // 0x16ada20: p = me0 + me10 + (me8·AD)/100 + (me18·AD)/100 , m = 0 (RE 2026-09-07)
        0x16ada20 => { let ad = rd_u64(att + ENT_STATS)?;
            Some((q400(rd_u64(me + 8)?.wrapping_mul(ad)).wrapping_add(rd_u64(me)?)
                    .wrapping_add(rd_u64(me + 0x10)?)
                    .wrapping_add(q400(rd_u64(me + 0x18)?.wrapping_mul(ad))), 0)) }
        EFF28_SUM_20_18 => {
            // 0x1146bb0: Σ 자식(+0x28) — rax(p)·rdx(m) 둘 다 합산
            let n = rd_u64(me + 0x28)?; if n == 0 { return Some((0, 0)); }
            let arr = rd_u64(me + 0x20)? as usize; if !ptr_ok(arr) { return None; }
            let (mut p, mut m) = (0u64, 0u64);
            for i in 0..n.min(CAP_ITER) as usize { let (d, v) = (rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize); let (a, b) = eff28_damage(d, v, att)?; p = p.wrapping_add(a); m = m.wrapping_add(b); }
            Some((p, m))
        }
        EFF28_PAIR_RAW => Some((rd_u64(me)?, rd_u64(me + 8)?)),
        0x12a56e0 => {
            // Σ 자식(stride 0x10, data@+0/vt@+8, ptr [me+8], len [me+0x10]) 의 (p, m)
            let n = rd_u64(me + 0x10)?; if n == 0 { return Some((0, 0)); }
            let arr = rd_u64(me + 8)? as usize; if !ptr_ok(arr) { return None; }
            let (mut p, mut m) = (0u64, 0u64);
            for i in 0..n.min(CAP_ITER) as usize { let (d, v) = (rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize); let (a, b) = eff28_damage(d, v, att)?; p = p.wrapping_add(a); m = m.wrapping_add(b); }
            Some((p, m))
        }
        0x164eaa0 => {
            // SwitchByLevel: desc.vt+0x48(=0xc8c890 [att+0x5c8] 레벨) >= 3 → 자식 1(data @me+0x10, vt @me+0x18), 아니면 자식 0(@me+0/+8) (capstone 2026-09-06 20:33)
            let idx = if rd_u64(att + ENT_LEVEL)? >= 3 { 1usize } else { 0 };
            let (d, v) = (rd_u64(me + idx * 0x10)? as usize, rd_u64(me + 8 + idx * 0x10)? as usize);
            eff28_damage(d, v, att)
        }
        0x16abff0 => {
            // 감쇠 누적: n = min(att.650+1, me.30); coef=me.18; acc += coef*att.618/100; coef = max(coef*me.20/100, me.28) (capstone 2026-09-06 20:38); m=0
            let stat = rd_u64(att + ENT_STATS)?; let n = rd_u64(att + 0x650)?.wrapping_add(1).min(rd_u64(me + 0x30)?);
            let (mut coef, mut acc) = (rd_u64(me + 0x18)?, 0u64); let (m20, m28) = (rd_u64(me + 0x20)?, rd_u64(me + 0x28)?);
            for _ in 0..n.min(4096) { acc = acc.wrapping_add(q400(coef.wrapping_mul(stat))); coef = q400(coef.wrapping_mul(m20)).max(m28); }
            Some((acc, 0))
        }
        0x1715d70 => {
            // 마법형 GENERIC: desc.vt+0x38(att)=&att.618 → m = me.10 + me.18*att.620/100 + me.20*att.628/100 ; p=0 (capstone 2026-09-06 21:30)
            let a = q400(rd_u64(me + 0x18)?.wrapping_mul(rd_u64(att + ENT_STATS + 8)?));
            let b = q400(rd_u64(me + 0x20)?.wrapping_mul(rd_u64(att + ENT_STATS + 0x10)?));
            Some((0, a.wrapping_add(rd_u64(me + 0x10)?).wrapping_add(b)))
        }
        0x1340ac0 => {
            // Σ 리스트1(stride 0x18 @me+0x68/len me+0x70) + Σ 리스트2(stride 0x10 @me+0x80/len me+0x88) 의 (p,m)
            let (mut p, mut m) = (0u64, 0u64);
            let n1 = rd_u64(me + 0x70)?; if n1 != 0 { let arr = rd_u64(me + 0x68)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n1.min(CAP_ITER) as usize { let (d, v) = (rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize); let (a, b) = eff28_damage(d, v, att)?; p = p.wrapping_add(a); m = m.wrapping_add(b); } }
            let n2 = rd_u64(me + 0x88)?; if n2 != 0 { let arr = rd_u64(me + 0x80)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n2.min(CAP_ITER) as usize { let (d, v) = (rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize); let (a, b) = eff28_damage(d, v, att)?; p = p.wrapping_add(a); m = m.wrapping_add(b); } }
            Some((p, m))
        }
        0x1606470 => {
            // SwitchByBuff: desc.vt+0x50(=0x128f470 buff_lookup)(att, [me+8], [me+0x10]) != 0 → 자식 1, 아니면 자식 0 (capstone 2026-09-06 20:22)
            let idx = if buff_lookup(att, rd_u64(me + 8)? as usize, rd_u64(me + 0x10)?)? != 0 { 1usize } else { 0 };
            let (d, v) = (rd_u64(me + 0x18 + idx * 0x10)? as usize, rd_u64(me + 0x20 + idx * 0x10)? as usize);
            eff28_damage(d, v, att)
        }
        0x16a7550 => {   // 스탯 스냅샷(ENT_VT+0x30 = att.618): p = ((me.8*st[0]/100 + me.0) * (me.10*st[0x38] + 100)) / 100 ; m=0 (capstone 2026-09-06 22:50)
            let st = att + ENT_STATS;
            let base = q400(rd_u64(me + 8)?.wrapping_mul(rd_u64(st)?)).wrapping_add(rd_u64(me)?);
            let f = rd_u64(me + 0x10)?.wrapping_mul(rd_u64(st + 0x38)?).wrapping_add(100);
            Some((q400(f.wrapping_mul(base)), 0))
        }
        0x12a4fb0 => {   // Σ 자식(stride 0x10 @me+0x48/len me+0x50) (capstone 2026-09-07 00:25)
            let n = rd_u64(me + 0x50)?; if n == 0 { return Some((0, 0)); } let arr = rd_u64(me + 0x48)? as usize; if !ptr_ok(arr) { return None; }
            let (mut p, mut m) = (0u64, 0u64);
            for i in 0..n.min(CAP_ITER) as usize { let (d, v) = (rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize); let (a, b) = eff28_damage(d, v, att)?; p = p.wrapping_add(a); m = m.wrapping_add(b); }
            Some((p, m))
        }
        0x12480c0 => {   // Σ 자식(stride 0x18 @me+0x50/len me+0x58) (capstone 2026-09-06 22:40)
            let n = rd_u64(me + 0x58)?; if n == 0 { return Some((0, 0)); } let arr = rd_u64(me + 0x50)? as usize; if !ptr_ok(arr) { return None; }
            let (mut p, mut m) = (0u64, 0u64);
            for i in 0..n.min(CAP_ITER) as usize { let (d, v) = (rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize); let (a, b) = eff28_damage(d, v, att)?; p = p.wrapping_add(a); m = m.wrapping_add(b); }
            Some((p, m))
        }
        0x16a32a0 => {   // (Σ 리스트1 @0x50/0x58 s0x18) × n + Σ 리스트2 @0x68/0x70 s0x18 ; n = me.78 / max(1, me.80) (capstone 2026-09-06 22:40)
            let sum = |po: usize, lo: usize| -> Option<(u64, u64)> { let n = rd_u64(me + lo)?; if n == 0 { return Some((0, 0)); } let arr = rd_u64(me + po)? as usize; if !ptr_ok(arr) { return None; }
                let (mut p, mut m) = (0u64, 0u64); for i in 0..n.min(CAP_ITER) as usize { let (d, v) = (rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize); let (a, b) = eff28_damage(d, v, att)?; p = p.wrapping_add(a); m = m.wrapping_add(b); } Some((p, m)) };
            let (p1, m1) = sum(0x50, 0x58)?; let (p2, m2) = sum(0x68, 0x70)?;
            let per = rd_u64(me + 0x80)?.max(1); let n = rd_u64(me + 0x78)? / per;
            Some((p1.wrapping_mul(n).wrapping_add(p2), m1.wrapping_mul(n).wrapping_add(m2)))
        }
        EFF28_PAIR_BYKIND => { let v = rd_u64(me)?; if rd_i32(me + 8)? == 1 { Some((0, v)) } else { Some((v, 0)) } }
        EFF28_GENERIC => {
            // 0x1708310: [s+0x18]*stats[0]/100 + [s+0x20]*stats[0x10]/100 + [s+0x10] ; m=0   (stats = e+0x618)
            let a = q400(rd_u64(me + 0x18)?.wrapping_mul(rd_u64(att + ENT_STATS)?));
            let b = q400(rd_u64(me + 0x20)?.wrapping_mul(rd_u64(att + ENT_STATS + 0x10)?));
            let base = rd_u64(me + 0x10)?;
            super::super::tr(5, a.min(0xffff) | b.min(0xffff) << 16 | base.min(0xffff) << 32 | ((me as u64) & 0xffff) << 48);
            Some((a.wrapping_add(base).wrapping_add(b), 0))
        }
        // p.0x00 + p.0x08 * AD / 100   (EST[0x30] 스탯복사판 / EST[0x38] 포인터판)
        0x134ac70 | 0x106a230 => Some((rd_u64(me)?.wrapping_add(rd_u64(me + 8)?.wrapping_mul(rd_u64(att + ENT_STATS)?) / 100), 0)),
        // 마법 쪽으로: (0, p.0x00 + p.0x08 * AP / 100)
        0x122fcb0 => Some((0, rd_u64(me)?.wrapping_add(rd_u64(me + 8)?.wrapping_mul(rd_u64(att + ENT_STATS + 8)?) / 100))),
        // ((p.0x38 >> 1) + p.0x30) * AD / 100
        0x16a8c80 => Some(((rd_u64(me + 0x38)? >> 1).wrapping_add(rd_u64(me + 0x30)?).wrapping_mul(rd_u64(att + ENT_STATS)?) / 100, 0)),
        // p.0x08 + p.0x10 * AD / 100
        0x183ff50 => Some((rd_u64(me + 8)?.wrapping_add(rd_u64(me + 0x10)?.wrapping_mul(rd_u64(att + ENT_STATS)?) / 100), 0)),
        // ((p.0x10 + 100) * p.0x08 / 100) * AD / 100  +  2 * p.0x00
        0x16a76d0 => {
            let t = rd_u64(me + 0x10)?.wrapping_add(100).wrapping_mul(rd_u64(me + 8)?) / 100;
            Some((t.wrapping_mul(rd_u64(att + ENT_STATS)?) / 100 + rd_u64(me)?.wrapping_mul(2), 0))
        }
        // 레벨(self.0x5c8) >= 3 이면 자식 (p+0x10,p+0x18), 아니면 (p+0,p+8) 로 위임
        // ★골격 스캐너 폴백: 자식 순회 합산형이면 RVA 표 없이 처리(rax·rdx 둘 다 합산)
        _ => {
            let f = rd_u64(vt + 0x28)? as usize;
            // ★잎이면 specemu 로 **실제 실행**한다 — `vt+0x28` 규약(sret 없음, 반환 (rax,rdx)).
            //   손으로 옮기는 것보다 정확하다(2026-09-07 확정).
            if composite_loops(f, 0x28).is_none() {
                // EST 서술자는 인스턴스가 둘(0x33d8768 / 0x33da3d0)인데 슬롯 내용이 동일하다(RE 확인)
                let eb = crate::exe_base();
                if eb != 0 { if let Some(v) = super::specemu::run_eff28(f, me as u64, 0, att as u64, (eb + 0x33da3d0) as u64) { return Some(v); } }
            }
            if let Some((lp, n)) = composite_loops(f, 0x28) {
                let (mut p, mut m) = (0u64, 0u64);
                for k in 0..n {
                    let (lo, po, st) = lp[k];
                    let cnt = rd_u64(me + lo)?; if cnt == 0 { continue; }
                    let arr = rd_u64(me + po)? as usize; if !ptr_ok(arr) { return None; }
                    for i in 0..cnt.min(CAP_ITER) as usize {
                        let e = arr + i * st;
                        let (d, v) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
                        if !ptr_ok(d) || !ptr_ok(v) { return None; }
                        let (x, y) = eff28_damage(d, v, att)?;
                        p = p.wrapping_add(x); m = m.wrapping_add(y);
                    }
                }
                return Some((p, m));
            }
            unseen(0x28, rva); None
        }
    }
}
/// 진단: eff28 구현 트리(impl RVA · (p,m) · 페이로드 앞 워드)를 문자열로
pub unsafe fn eff28_trace(data: usize, vt: usize, att: usize, depth: u32) -> String {
    if depth > 4 { return "…".into(); }
    let rva = match impl_rva(vt, 0x28) { Some(r) => r, None => return "?vt".into() };
    let me = match arc_payload(data, vt) { Some(m) => m, None => return "?pl".into() };
    let v = eff28_damage(data, vt, att);
    let mut out = format!("[{:#x}->{:?} w={:?}", rva, v, (0..4).map(|i| rd_u64(me + i * 8).unwrap_or(0)).collect::<Vec<_>>());
    let kids = |po: usize, lo: usize, st: usize| -> String {
        let n = rd_u64(me + lo).unwrap_or(0); if n == 0 || n > 64 { return String::new(); }
        let arr = rd_u64(me + po).unwrap_or(0) as usize; if !ptr_ok(arr) { return String::new(); }
        (0..n as usize).map(|i| eff28_trace(rd_u64(arr + i * st).unwrap_or(0) as usize, rd_u64(arr + i * st + 8).unwrap_or(0) as usize, att, depth + 1)).collect::<Vec<_>>().join("")
    };
    match rva { 0x12a56e0 => out += &kids(8, 0x10, 0x10), EFF28_SUM_20_18 => out += &kids(0x20, 0x28, 0x18),
        // SwitchByBuff: 어느 자식을 골랐는지와 그 자식의 계산을 그대로 펼친다
        0x1606470 => {
            let bl = buff_lookup(att, rd_u64(me + 8).unwrap_or(0) as usize, rd_u64(me + 0x10).unwrap_or(0));
            let nm: String = (0..rd_u64(me + 0x10).unwrap_or(0).min(CAP_ITER) as usize)
                .map(|i| rd_u8(rd_u64(me + 8).unwrap_or(0) as usize + i) as char).collect();
            let idx = if matches!(bl, Some(x) if x != 0) { 1usize } else { 0 };
            out += &format!(" nm='{}' bl={:?} idx={} ", nm, bl, idx);
            out += &eff28_trace(rd_u64(me + 0x18 + idx * 0x10).unwrap_or(0) as usize,
                                rd_u64(me + 0x20 + idx * 0x10).unwrap_or(0) as usize, att, depth + 1);
            out += &format!(" alt{}", eff28_trace(rd_u64(me + 0x18 + (1 - idx) * 0x10).unwrap_or(0) as usize,
                                rd_u64(me + 0x20 + (1 - idx) * 0x10).unwrap_or(0) as usize, att, depth + 1));
        }
        _ => {} }
    out + "]"
}
/// effect `+0x38` 최대체력 비율 피해(%)
pub unsafe fn eff38_pct(data: usize, vt: usize, _att: usize) -> Option<u64> {
    let rva = impl_rva(vt, 0x38)?; let me = arc_payload(data, vt)?;
    match rva {
        EFF38_ZERO => Some(0),
        EFF38_GET28 => rd_u64(me + 0x28),
        0x1341090 => {   // Σ 리스트1(@0x68/0x70 s0x18) + Σ 리스트2(@0x80/0x88 **s0x10**) 의 +0x38
            // ★stride 정정 2026-09-07(RE `0x134114a add r13,0x10`). 0x18 로 훑으면 원소가 어긋나 (vt_i, data_{i+1})
            //   쌍이 만들어지고 vt 자리에 Arc data 가 들어가 `.rdata` 를 vtable 로 오독한다(=slot+0x38 의 0x34xxxxx NA).
            let mut acc = 0u64;
            for (po, lo, st) in [(0x68usize, 0x70usize, 0x18usize), (0x80, 0x88, 0x10)] { let n = rd_u64(me + lo)?; if n == 0 { continue; } let arr = rd_u64(me + po)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n.min(CAP_ITER) as usize { acc = acc.wrapping_add(eff38_pct(rd_u64(arr + i * st)? as usize, rd_u64(arr + i * st + 8)? as usize, _att)?); } }
            Some(acc)
        }
        0x1248270 => {   // Σ 자식(stride 0x18 @me+0x50/len me+0x58) 의 +0x38 (capstone 2026-09-06 22:45)
            let n = rd_u64(me + 0x58)?; if n == 0 { return Some(0); } let arr = rd_u64(me + 0x50)? as usize; if !ptr_ok(arr) { return None; }
            let mut acc = 0u64; for i in 0..n.min(CAP_ITER) as usize { acc = acc.wrapping_add(eff38_pct(rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize, _att)?); } Some(acc)
        }
        0x16a3610 => {   // ★꼬리 확정 2026-09-07: n*sum1 + sum2, n = me[0x78] / max(me[0x80],1) (unsigned)
            let mut sums = [0u64; 2];
            for (k, (po, lo)) in [(0x50usize, 0x58usize), (0x68, 0x70)].into_iter().enumerate() {
                let n = rd_u64(me + lo)?; if n == 0 { continue; } let arr = rd_u64(me + po)? as usize; if !ptr_ok(arr) { return None; }
                for i in 0..n.min(CAP_ITER) as usize { sums[k] = sums[k].wrapping_add(eff38_pct(rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize, _att)?); } }
            let c = rd_u64(me + 0x80)?.max(1);
            Some((rd_u64(me + 0x78)? / c).wrapping_mul(sums[0]).wrapping_add(sums[1]))
        }
        0x16067b0 => {   // SwitchByBuff(+0x38 판): buff_lookup(att, [me+8], [me+0x10]) != 0 → 자식1(@0x28/0x30) 아니면 자식0(@0x18/0x20)
            let idx = if buff_lookup(_att, rd_u64(me + 8)? as usize, rd_u64(me + 0x10)?)? != 0 { 1usize } else { 0 };
            eff38_pct(rd_u64(me + 0x18 + idx * 0x10)? as usize, rd_u64(me + 0x20 + idx * 0x10)? as usize, _att)
        }
        0x12a5890 => {
            // Σ 자식(stride 0x10, ptr [me+8], len [me+0x10]) 의 +0x38 (capstone 2026-09-06 20:38)
            let n = rd_u64(me + 0x10)?; if n == 0 { return Some(0); }
            let arr = rd_u64(me + 8)? as usize; if !ptr_ok(arr) { return None; }
            let mut acc = 0u64;
            for i in 0..n.min(CAP_ITER) as usize { let (d, v) = (rd_u64(arr + i * 0x10)? as usize, rd_u64(arr + i * 0x10 + 8)? as usize); acc = acc.wrapping_add(eff38_pct(d, v, _att)?); }
            Some(acc)
        }
        EFF38_SUM_20_18 => {
            let n = rd_u64(me + 0x28)?; if n == 0 { return Some(0); }
            let arr = rd_u64(me + 0x20)? as usize; if !ptr_ok(arr) { return None; }
            let mut acc = 0u64;
            for i in 0..n.min(CAP_ITER) as usize { let (d, v) = (rd_u64(arr + i * 0x18)? as usize, rd_u64(arr + i * 0x18 + 8)? as usize); acc = acc.wrapping_add(eff38_pct(d, v, _att)?); }
            Some(acc)
        }
        // 레벨(att.0x5c8) >= 3 이면 자식 (p+0x10,p+0x18), 아니면 (p+0,p+8) 로 그대로 위임
        0x164ed70 => {
            let o = if rd_u64(_att + 0x5c8)? >= 3 { 0x10usize } else { 0 };
            let (cd, cv) = (rd_u64(me + o)? as usize, rd_u64(me + o + 8)? as usize);
            if !ptr_ok(cd) || !ptr_ok(cv) { return None; }
            eff38_pct(cd, cv, _att)
        }
        _ => {
            let f = rd_u64(vt + 0x38)? as usize;
            if let Some((lp, n)) = composite_loops(f, 0x38) {
                let mut acc = 0u64;
                for k in 0..n {
                    let (lo, po, st) = lp[k];
                    let cnt = rd_u64(me + lo)?; if cnt == 0 { continue; }
                    let arr = rd_u64(me + po)? as usize; if !ptr_ok(arr) { return None; }
                    for i in 0..cnt.min(CAP_ITER) as usize {
                        let e = arr + i * st;
                        let (d, v) = (rd_u64(e)? as usize, rd_u64(e + 8)? as usize);
                        if !ptr_ok(d) || !ptr_ok(v) { return None; }
                        acc = acc.wrapping_add(eff38_pct(d, v, _att)?);
                    }
                }
                return Some(acc);
            }
            unseen(0x38, rva); None
        }
    }
}
/// effect `+0x40` 회복량 추정
pub unsafe fn eff40_heal(data: usize, vt: usize, ent: usize) -> Option<u64> { eff40_heal_d(data, vt, ent, 0) }
thread_local! { static CHAIN40: core::cell::Cell<[(usize, usize, usize); 48]> = const { core::cell::Cell::new([(0, 0, 0); 48]) }; }
static CHAIN40_DUMPED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// 진단: 깊이 초과 시 그 스레드가 밟아 온 (vt rva, impl rva, data) 체인을 `judge_eff40_chain.txt` 에 기록(최초 8회)
unsafe fn chain40_dump(depth: u32, data: usize, vt: usize) {
    if CHAIN40_DUMPED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= 8 { return; }
    let b = exe_base();
    let mut out = format!("depth={} data={:#x} vt={:#x}(rva {:#x}) impl40={:#x}\n", depth, data, vt, vt.wrapping_sub(b), impl_rva(vt, 0x40).unwrap_or(0));
    let ch = CHAIN40.with(|c| c.get());
    for (i, (v, im, d)) in ch.iter().enumerate().take(depth.min(48) as usize) {
        let me = arc_payload(*d, *v).unwrap_or(0);
        let words: Vec<String> = (0..12).map(|k| format!("{:#x}", rd_u64(me + k * 8).unwrap_or(0))).collect();
        out += &format!("  [{}] vt_rva={:#x} impl40={:#x} data={:#x} payload={:#x} | {}\n", i, v.wrapping_sub(b), im, d, me, words.join(" "));
    }
    if let Some(p) = super::super::pth("judge_eff40_chain.txt") { let _ = std::fs::OpenOptions::new().create(true).append(true).open(p).and_then(|mut f| { use std::io::Write; f.write_all(out.as_bytes()) }); }
}
unsafe fn eff40_heal_d(data: usize, vt: usize, ent: usize, depth: u32) -> Option<u64> {
    if depth > 40 { chain40_dump(depth, data, vt); unseen(0xdd, impl_rva(vt, 0x40).unwrap_or(0)); return None; }   // 중첩 SwitchByBuff 11단 실측(14:37 리플레이) → 40
    let rva = impl_rva(vt, 0x40)?; let me = arc_payload(data, vt)?;
    CHAIN40.with(|c| { let mut a = c.get(); a[depth as usize] = (vt, rva, data); c.set(a); });
    match rva {
        EFF40_ZERO => Some(0),
        EFF40_SUM_08_10 => sum_children_40(rd_u64(me + 8)? as usize, rd_u64(me + 0x10)?, 0x10, ent, depth),
        EFF40_SUM_20_18 => sum_children_40(rd_u64(me + 0x20)? as usize, rd_u64(me + 0x28)?, 0x18, ent, depth),
        EFF40_SUM_50_18 => sum_children_40(rd_u64(me + 0x50)? as usize, rd_u64(me + 0x58)?, 0x18, ent, depth),
        EFF40_SUM_50_18_68_10 => {
            let a = sum_children_40(rd_u64(me + 0x50)? as usize, rd_u64(me + 0x58)?, 0x18, ent, depth)?;
            let b = sum_children_40(rd_u64(me + 0x68)? as usize, rd_u64(me + 0x70)?, 0x10, ent, depth)?;
            Some(a.wrapping_add(b))
        }
        EFF40_LEAF_ADAP => {
            // 0x117ca10: [s+0x18]*stats[0]/100 + [s+0x20]*stats[8]/100 + [s+0x28]
            let a = q400(rd_u64(me + 0x18)?.wrapping_mul(rd_u64(ent + ENT_STATS)?));
            let b = q400(rd_u64(me + 0x20)?.wrapping_mul(rd_u64(ent + ENT_STATS + 8)?));
            Some(a.wrapping_add(rd_u64(me + 0x28)?).wrapping_add(b))
        }
        EFF40_LEAF_STACK => {
            // 0x12b2e90: [s+0x10] + [s+0x18] * (stats[0x38] + 1)
            Some(rd_u64(me + 0x10)?.wrapping_add(rd_u64(me + 0x18)?.wrapping_mul(rd_u64(ent + ENT_STATS + 0x38)?.wrapping_add(1))))
        }
        EFF40_SUM_08_18 => sum_children_40(rd_u64(me + 8)? as usize, rd_u64(me + 0x10)?, 0x18, ent, depth),
        EFF40_SUM_68_18_80_10 => {
            // 0x13409d0: Σ[0x68/0x70 s0x18] + Σ[0x80/0x88 s0x10]
            let a = sum_children_40(rd_u64(me + 0x68)? as usize, rd_u64(me + 0x70)?, 0x18, ent, depth)?;
            let b = sum_children_40(rd_u64(me + 0x80)? as usize, rd_u64(me + 0x88)?, 0x10, ent, depth)?;
            Some(a.wrapping_add(b))
        }
        EFF40_SUM_48_10 => sum_children_40(rd_u64(me + 0x48)? as usize, rd_u64(me + 0x50)?, 0x10, ent, depth),
        EFF40_SUM_RATIO_68_50 => {
            // 0x16a3190: a = Σ[0x68/0x70 s0x18] ; b = Σ[0x50/0x58 s0x18] ; d = max(1,[s+0x80]) ; ([s+0x78]/d)*b + a
            let a = sum_children_40(rd_u64(me + 0x68)? as usize, rd_u64(me + 0x70)?, 0x18, ent, depth)?;
            let b = sum_children_40(rd_u64(me + 0x50)? as usize, rd_u64(me + 0x58)?, 0x18, ent, depth)?;
            let d = rd_u64(me + 0x80)?.max(1);
            Some((rd_u64(me + 0x78)? / d).wrapping_mul(b).wrapping_add(a))
        }
        EFF40_SWITCH_BY_BUFF => {
            // 0x16063d0: has_buff(ent, name [s+8]/[s+0x10]) ? 자식1([s+0x28],[s+0x30]) : 자식0([s+0x18],[s+0x20]) → 그 +0x40
            let idx = if buff_lookup(ent, rd_u64(me + 8)? as usize, rd_u64(me + 0x10)?)? != 0 { 0x10 } else { 0 };
            let (d, v) = (rd_u64(me + 0x18 + idx)? as usize, rd_u64(me + 0x20 + idx)? as usize);
            eff40_heal_d(d, v, ent, depth + 1)
        }
        EFF40_SWITCH_BY_LEVEL3 => {
            // 0x164ea30: level(ent+0x5c8) >= 3 ? 자식1 : 자식0
            let idx = if rd_u64(ent + ENT_LEVEL)? >= 3 { 0x10 } else { 0 };
            let (d, v) = (rd_u64(me + idx)? as usize, rd_u64(me + idx + 8)? as usize);
            eff40_heal_d(d, v, ent, depth + 1)
        }
        EFF40_LEAF_RATIO => {
            // 0x12b1550: ([s] + [s+8]*snap[8]) * ([s+0x20] / [s+0x28]) ; [s+0x28]==0 → div0 panic
            let d = rd_u64(me + 0x28)?; if d == 0 { return None; }
            let base = rd_u64(me)?.wrapping_add(rd_u64(me + 8)?.wrapping_mul(rd_u64(ent + ENT_STATS + 8)?));
            Some(base.wrapping_mul(rd_u64(me + 0x20)? / d))
        }
        _ => { unseen(0x40, rva); None }
    }
}
/// 0x128f470 — 엔티티 버프 목록([ent+0x2e0], len [ent+0x2e8], stride 0x120)에서 이름(len==[b] && memcmp(b+4, name)) 첫 항목 주소, 없으면 0.
pub unsafe fn buff_lookup(ent: usize, name: usize, len: u64) -> Option<usize> {
    let n = rd_u64(ent + ENT_BUFFS_LEN)?; if n == 0 { return Some(0); }
    let base = rd_u64(ent + ENT_BUFFS_PTR)? as usize; if !ptr_ok(base) || (len > 0 && !ptr_ok(name)) { return None; }
    for i in 0..n.min(CAP_ITER) as usize {
        let b = base + i * 0x120;
        if rd_u32(b) as u64 != len { continue; }
        let mut eq = true;
        for k in 0..len.min(0x100) as usize { if rd_u8(b + 4 + k) != rd_u8(name + k) { eq = false; break; } }
        if eq { return Some(b); }
    }
    Some(0)
}
/// BuffState 병합(0x126e6c0/0x126ec80/0x126f240 공통): 자식 중 type≠−1 인 첫 것을 채택하고 이후 자식의 +0x80(vamp, i32)을 더한다(paddd).
unsafe fn merge_a0(acc: &mut (i32, i32), ptr: usize, len: u64, stride: usize, ent: usize, depth: u32) -> Option<()> {
    if len == 0 { return Some(()); } if !ptr_ok(ptr) { return None; }
    for i in 0..len.min(CAP_ITER) as usize {
        let (d, v) = (rd_u64(ptr + i * stride)? as usize, rd_u64(ptr + i * stride + 8)? as usize);
        let (ty, vamp) = effa0_buff_d(d, v, ent, depth + 1)?;
        if ty == -1 { continue; }
        if acc.0 == -1 { *acc = (ty, vamp); } else { acc.1 = acc.1.wrapping_add(vamp); }
    }
    Some(())
}
/// effect `+0xa0` BuffState sret — 핸들러가 읽는 두 필드만: (+0x48 type i32(−1=None), +0x80 vamp i32)
pub unsafe fn effa0_buff(data: usize, vt: usize, ent: usize) -> Option<(i32, i32)> { effa0_buff_d(data, vt, ent, 0) }
unsafe fn effa0_buff_d(data: usize, vt: usize, ent: usize, depth: u32) -> Option<(i32, i32)> { effa0_buff_p(arc_payload(data, vt)?, vt, ent, depth) }
/// payload 주소를 직접 받는 판(0x1153860 위임은 자식 data 를 Arc 조정 없이 그대로 넘긴다)
unsafe fn effa0_buff_p(me: usize, vt: usize, ent: usize, depth: u32) -> Option<(i32, i32)> {
    if depth > 40 { unseen(0xde, impl_rva(vt, 0xa0).unwrap_or(0)); return None; }
    let rva = impl_rva(vt, 0xa0)?;
    match rva {
        EFFA0_MERGE_50_18_68_18 => {
            let mut acc = (-1, 0);
            merge_a0(&mut acc, rd_u64(me + 0x50)? as usize, rd_u64(me + 0x58)?, 0x18, ent, depth)?;
            merge_a0(&mut acc, rd_u64(me + 0x68)? as usize, rd_u64(me + 0x70)?, 0x18, ent, depth)?;
            Some(acc)
        }
        EFFA0_MERGE_68_18_80_10 => {
            let mut acc = (-1, 0);
            merge_a0(&mut acc, rd_u64(me + 0x68)? as usize, rd_u64(me + 0x70)?, 0x18, ent, depth)?;
            merge_a0(&mut acc, rd_u64(me + 0x80)? as usize, rd_u64(me + 0x88)?, 0x10, ent, depth)?;
            Some(acc)
        }
        EFFA0_MERGE_08_10_B => { let mut acc = (-1, 0); merge_a0(&mut acc, rd_u64(me + 8)? as usize, rd_u64(me + 0x10)?, 0x10, ent, depth)?; Some(acc) }
        EFFA0_MERGE_08_18 => { let mut acc = (-1, 0); merge_a0(&mut acc, rd_u64(me + 8)? as usize, rd_u64(me + 0x10)?, 0x18, ent, depth)?; Some(acc) }
        EFFA0_MERGE_48_10 => { let mut acc = (-1, 0); merge_a0(&mut acc, rd_u64(me + 0x48)? as usize, rd_u64(me + 0x50)?, 0x10, ent, depth)?; Some(acc) }
        EFFA0_DELEGATE_RAW => effa0_buff_p(rd_u64(me)? as usize, rd_u64(me + 8)? as usize, ent, depth + 1),
        EFFA0_SWITCH_BY_BUFF => {
            let idx = if buff_lookup(ent, rd_u64(me + 8)? as usize, rd_u64(me + 0x10)?)? != 0 { 0x10 } else { 0 };
            let (d, v) = (rd_u64(me + 0x18 + idx)? as usize, rd_u64(me + 0x20 + idx)? as usize);
            effa0_buff_d(d, v, ent, depth + 1)
        }
        EFFA0_CONST0 => Some((0, 0)),
        EFFA0_CONST1_ENCH => Some((1, 0)),
        EFFA0_CONST1_ENCH2 => Some((1, 0)),
        EFFA0_CONST1_BARD2 => Some((1, 0)),
        EFFA0_CONST1_STAT => Some((1, 0)),
        EFFA0_COPY_STATE => Some((rd_i32(me + 0x48)?, rd_i32(me + 0x80)?)),
        EFFA0_STATSCALED => Some((rd_i32(me + 0x48)?, rd_i32(me + 0x80)?)),
        EFFA0_MERGE_20_18_INLINE => { let mut acc = (-1, 0); merge_a0(&mut acc, rd_u64(me + 0x20)? as usize, rd_u64(me + 0x28)?, 0x18, ent, depth)?; Some(acc) }
        EFFA0_SWITCH_BY_LEVEL3 => {
            let idx = if rd_u64(ent + ENT_LEVEL)? >= 3 { 0x10 } else { 0 };
            let (d, v) = (rd_u64(me + idx)? as usize, rd_u64(me + idx + 8)? as usize);
            effa0_buff_d(d, v, ent, depth + 1)
        }
        EFFA0_INLINE_STATE => { if rd_u8(me + 0x120) != 0 { Some((-1, 0)) } else { Some((rd_i32(me + 0x48)?, rd_i32(me + 0x80)?)) } }
        EFFA0_NONE => Some((-1, 0)),
        EFFA0_WIND_SPEED => Some((1, 0)),          // 0x12266f0: 상수 생성(type 1, +0x80 = 0)
        EFFA0_MERGE_08_10 => { let mut acc = (-1, 0); merge_a0(&mut acc, rd_u64(me + 8)? as usize, rd_u64(me + 0x10)?, 0x10, ent, depth)?; Some(acc) }
        EFFA0_MERGE_50_18 => { let mut acc = (-1, 0); merge_a0(&mut acc, rd_u64(me + 0x50)? as usize, rd_u64(me + 0x58)?, 0x18, ent, depth)?; Some(acc) }
        EFFA0_MERGE_20_18 => { let mut acc = (-1, 0); merge_a0(&mut acc, rd_u64(me + 0x20)? as usize, rd_u64(me + 0x28)?, 0x18, ent, depth)?; Some(acc) }
        EFFA0_MERGE_50_18_68_10 => {
            let mut acc = (-1, 0);
            merge_a0(&mut acc, rd_u64(me + 0x50)? as usize, rd_u64(me + 0x58)?, 0x18, ent, depth)?;
            merge_a0(&mut acc, rd_u64(me + 0x68)? as usize, rd_u64(me + 0x70)?, 0x10, ent, depth)?;
            Some(acc)
        }
        _ => { unseen(0xa0, rva); None }
    }
}
/// 프로바이더(Box<dyn DataAction>) `+0x90` cooltime — 구현체 바이트를 해석: `48 8b 41 K c3` / `48 8b 81 K32 c3` = [data+K], `31 c0 c3` = 0.
pub unsafe fn prov90_cooltime(data: usize, vt: usize, ent: usize) -> Option<u64> {
    let rva = impl_rva(vt, 0x90)?; let b = exe_base(); let t = b + rva;
    // 레벨 의존 impl (fight_check 검증 2026-09-06 20:20, capstone): 0x1725060 = sat_sub(d.50, lv*d.108) · 0x12462a0 = max(sat_sub(d.50, lv*d.f8), d.100+60)
    if rva == 0x1725060 { let lv = rd_u64(ent + 0x5c8)?; return Some(rd_u64(data + 0x50)?.saturating_sub(lv.wrapping_mul(rd_u64(data + 0x108)?))); }
    if rva == 0x17033a0 { let lv = rd_u64(ent + 0x5c8)?; return Some(rd_u64(data + 0x50)?.saturating_sub(lv.wrapping_mul(rd_u64(data + 0x138)?))); }
    if rva == 0x12462a0 { let lv = rd_u64(ent + 0x5c8)?; let prod = (rd_u64(data + 0xf8)? as u128) * (lv as u128); if prod >> 64 != 0 { return None; }
        let v = rd_u64(data + 0x50)?.saturating_sub(prod as u64); return Some(rd_u64(data + 0x100)?.wrapping_add(0x3c).max(v)); }
    let (b0, b1, b2) = (rd_u8(t), rd_u8(t + 1), rd_u8(t + 2));
    if b0 == 0x31 && b1 == 0xc0 && b2 == 0xc3 { return Some(0); }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x01 && rd_u8(t + 3) == 0xc3 { return rd_u64(data); }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x41 && rd_u8(t + 4) == 0xc3 { return rd_u64(data + rd_u8(t + 3) as usize); }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x81 && rd_u8(t + 7) == 0xc3 { return rd_u64(data + rd_u32(t + 3) as usize); }
    unseen(0x90, rva); None
}
