//! dn_reach — 0xd3fe50 "적이 내 넥서스에 닿는가" 순수 재현 (defense_nexus 틱 메모 캐시 0xc87850 의 bit8).
//!   게임 0.5.8 · 0xd3fe50 (1906B) + 슬롯4 헬퍼 0x1285320 (364B) + 이펙트 vt `+0xe8`(사거리 보너스) 구현체 4종:
//!     0x9db70 (×280) = 0 · 0x12a67a0 (×9) = 자식 이펙트들의 max · 0x12b9e60 (×1) = level≥3 && 넥서스 +0x298 표에 [+0x34]==1 있으면 [self+0x40] · 0x16a8c10 (×1) = [self+0x62]==1 && 버프 vt+0x48 ==1 → 9,999,999
//!   RE = REPORT\RE\2026-09-06_defense_nexus-핸들러-…(디컴 원문) + 이 파일 상단 주석(capstone, 4종은 Ghidra 미등록 함수라 create_function 실패 → 디스어셈 포팅).
//! ⚠0x16a8c10 의 버프 vt `+0x48` 은 또 한 층의 dyn — 그 플래그([self+0x62]==1)를 쓰는 이펙트가 291 중 1개뿐이라 **그 경로만 None(NA)** 로 두고 나머지는 전부 재현.
//! 판정 요지: 내 사이드 미니언이 0 이고(X+0x148+side*0x20 == 0), (적 유닛 kind1 이 넥서스를 때리는 중) 또는 (적 챔프 0~4 중 사거리 = base+슬롯+레벨항+보정+보너스 가 넥서스까지 닿음).
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use std::cell::Cell;

/// 진단(검증 기간 한정): 마지막 reach 호출의 내부값. [0]=my_minion_len·[1]=lists_hit·[2]=nexus 0x470|0x680<<32, 이후 적별 9워드: e, flag4c0, level, base438, slot10, slot18, e470|e680<<32, bonus|impl_rva<<32, dist2, range
thread_local! { pub static DBG: Cell<[u64; 3 + 5 * 10]> = const { Cell::new([0; 53]) }; }
#[inline] fn dbg_set(i: usize, v: u64) { DBG.with(|c| { let mut a = c.get(); if i < a.len() { a[i] = v; } c.set(a); }); }
pub fn dbg_fmt() -> String {
    DBG.with(|c| { let a = c.get(); let mut s = format!("my_minion_len={} lists_hit={} nexus470|680={:#x}", a[0], a[1], a[2]);
        for r in 0..5 { let b = 3 + r * 10; if a[b] == 0 { continue; }
            s.push_str(&format!(" | e{}={:#x} flag={} lvl={} base={} s10={} s18={} e470|680={:#x} bonus={} impl={:#x} d2={} range={}", r, a[b], a[b+1] as i64, a[b+2], a[b+3], a[b+4], a[b+5], a[b+6], a[b+7] & 0xffff_ffff, a[b+7] >> 32, a[b+8], a[b+9])); }
        s })
}

#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
/// Arc<dyn Effect> 페이로드 주소: data + ((align−1) & !0xf) + 0x10  (게임 0xd40023~0xd40031 그대로)
#[inline] unsafe fn eff_payload(data: usize, vt: usize) -> Option<usize> {
    let align = rd_u64(vt + 0x10)?; Some(data.wrapping_add(((align.wrapping_sub(1)) & !0xfu64) as usize).wrapping_add(0x10))
}
/// 이펙트 vt `+0xe8`(사거리 보너스) 디스패치 — 구현 RVA 로 판정(버전 태그 상수). depth 는 래퍼 재귀 가드.
unsafe fn eff_e8(data: usize, vt: usize, ent: usize, nexus: usize, depth: u32) -> Option<u64> {
    if depth > 4 || !ptr_ok(vt) { return None; }
    let base = exe_base(); if base == 0 { return None; }
    let tgt = rd_u64(vt + EFF_SLOT_E8)? as usize;
    let rva = tgt.wrapping_sub(base);
    let me = eff_payload(data, vt)?;
    if rva == EFF_E8_ZERO { return Some(0); }
    if rva == EFF_E8_MAX_CHILDREN {
        // self+0x8 = &[(data, vt); n], self+0x10 = n  → 첫 자식 값에서 시작해 max
        let n = rd_u64(me + 0x10)?; if n == 0 { return Some(0); }
        let arr = rd_u64(me + 8)? as usize; if !ptr_ok(arr) { return None; }
        let mut best = 0u64;
        for i in 0..n.min(64) as usize {
            let (d, v) = (rd_u64(arr + i * 16)? as usize, rd_u64(arr + i * 16 + 8)? as usize);
            let r = eff_e8(d, v, ent, nexus, depth + 1)?;
            best = if i == 0 { r } else { best.max(r) };
        }
        return Some(best);
    }
    if rva == EFF_E8_LVL3_NEXUS_TABLE {
        if rd_u64(ent + ENT_LEVEL)? < 3 { return Some(0); }
        let tbl = rd_u64(nexus + 0x298)? as usize; let n = rd_u64(nexus + 0x2a0)?;
        if n == 0 { return Some(0); } if !ptr_ok(tbl) { return None; }
        for i in 0..n.min(256) as usize { if rd_u8(tbl + i * 0x38 + 0x34) == 1 { return rd_u64(me + 0x40); } }
        return Some(0);
    }
    if rva == EFF_E8_BUFF_FLAG {
        if rd_u8(me + 0x62) != 1 { return Some(0); }
        return None;                                                   // 버프 vt+0x48 한 층 더 — 미재현(NA)
    }
    None
}
/// 사거리 = [e+0x438] + [slot+0x10] + (level−1)*[slot+0x18] + (slot.flag==0 ? hp680항(e) : 0) + hp680항(넥서스) + 보너스 ; dist² <= 사거리²
unsafe fn in_reach(e: usize, slot: usize, nexus: usize, r: usize) -> Option<bool> {
    let (data, vt) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    let bonus = eff_e8(data, vt, e, nexus, 0)?;
    let impl_rva = { let b = exe_base(); if b == 0 { 0 } else { (rd_u64(vt + EFF_SLOT_E8).unwrap_or(0) as usize).wrapping_sub(b) as u64 } };
    let term = |x: usize| -> Option<u64> { let a = rd_i32(x + ENT_F470)? as i64; let h = rd_u64(x + ENT_F680)?; Some(if a == 0 { h } else { (((a + 100) as u64).wrapping_mul(h)) / 100 }) };
    let r8 = if rd_i32(slot + 0x30)? == 0 { term(e)? } else { 0 };
    let rn = term(nexus)?;
    let range = rd_u64(e + ENT_F438)?.wrapping_add(rd_u64(slot + 0x10)?).wrapping_add(rd_u64(e + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(rd_u64(slot + 0x18)?))
        .wrapping_add(r8).wrapping_add(rn).wrapping_add(bonus);
    let d2 = sqd(rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?, rd_u64(nexus + ENT_X)?, rd_u64(nexus + ENT_Y)?);
    let b = 3 + r * 10;
    dbg_set(b, e as u64); dbg_set(b + 1, rd_i32(slot + 0x30)? as i64 as u64); dbg_set(b + 2, rd_u64(e + ENT_LEVEL)?); dbg_set(b + 3, rd_u64(e + ENT_F438)?);
    dbg_set(b + 4, rd_u64(slot + 0x10)?); dbg_set(b + 5, rd_u64(slot + 0x18)?); dbg_set(b + 6, (rd_i32(e + ENT_F470)? as u32 as u64) | (rd_u64(e + ENT_F680)? << 32));
    dbg_set(b + 7, (bonus & 0xffff_ffff) | ((impl_rva & 0xffff_ffff) << 32)); dbg_set(b + 8, d2); dbg_set(b + 9, range);
    Some(d2 <= range.wrapping_mul(range))
}

/// 0xd3fe50 (p5 = 선수 sim, p6 = &Holder) → bool
pub unsafe fn reach(p5: usize, p6: usize) -> Option<bool> {
    let side = rd_u64(p5 + P5_SIDE)?; if side > 1 { return None; }
    let h = Holder::new(p6)?; let w = h.world()?;
    DBG.with(|c| c.set([0; 53]));
    let nexus = rd_u64(w.x + X_NEXUS + (side as usize) * 8)? as usize; if nexus == 0 { return Some(false); }
    let mml = rd_u64(w.x + X_MINION_LEN + (side as usize) * 0x20)?; dbg_set(0, mml);
    dbg_set(2, (rd_i32(nexus + ENT_F470)? as u32 as u64) | (rd_u64(nexus + ENT_F680)? << 32));
    if mml != 0 { return Some(false); }
    let other = 1 - side; let nh = rd_u64(nexus + ENT_HANDLE)?;
    for i in 0..3usize {
        let ptr = rd_u64(w.x + X_LIST3_PTR[i] + (other as usize) * 0x20)? as usize;
        let len = rd_u64(w.x + X_LIST3_LEN[i] + (other as usize) * 0x20)?;
        if len == 0 { continue; } if !ptr_ok(ptr) { return None; }
        for j in 0..len.min(4096) as usize {
            let u = rd_u64(ptr + j * 8)? as usize; if !ptr_ok(u) { return None; }
            if rd_i32(u + ENT_KIND)? == 1 && rd_i32(u + ENT_F88)? == 1 && rd_u64(u + ENT_TARGET_H)? == nh { dbg_set(1, 1); return Some(true); }
        }
    }
    for r in 0..5u32 {
        let e = w.roster(other, r)?; if e == 0 { continue; }
        if rd_i32(e + ENT_SLOT0_FLAG)? == -1 { continue; }
        if in_reach(e, e + ENT_SLOT0, nexus, r as usize)? { return Some(true); }
    }
    Some(false)
}
