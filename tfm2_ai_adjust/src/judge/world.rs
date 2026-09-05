//! world — 판단 함수가 읽는 게임 상태를 **메모리 읽기만으로** 제공한다(게임 함수 호출 0 — CLAUDE.md §3 완전 재구현 원칙).
//! 읽기는 전부 `rd_*`(VEH 경유 safe read). 오프셋은 layout.rs(버전 태그). 실패 = None(호출부가 가드로 취급 → passthrough).
#![allow(dead_code)]
use crate::*;
use super::layout::*;

/// World-ish X 의 (data, vtable). `*p6 = X`, `X+0 = data`, `X+8 = WorldOps vt` — steal `0xcbbca0` 진입부 실측
/// (`mov rsi,[rdx]; mov rcx,[rsi]; mov rax,[rsi+8]`).
#[derive(Clone, Copy)]
pub struct World { pub x: usize, pub data: usize, pub vt: usize }

impl World {
    pub unsafe fn from_holder(p6: usize) -> Option<World> {
        if !ptr_ok(p6) { return None; }
        let x = rd_u64(p6)? as usize;
        if !ptr_ok(x) { return None; }
        let data = rd_u64(x)? as usize;
        let vt = rd_u64(x + 8)? as usize;
        if !ptr_ok(data) || !ptr_ok(vt) { return None; }
        Some(World { x, data, vt })
    }

    /// 진단 전용(순수 read): vt+slot 이 가리키는 함수의 RVA. 0 = 못 읽음/exe 밖.
    pub unsafe fn slot_target_rva(&self, slot: usize) -> usize {
        let t = rd_u64(self.vt + slot).unwrap_or(0) as usize;
        let base = exe_base();
        if base != 0 && t > base && t - base < 0x4000000 { t - base } else { 0 }
    }

    /// WorldOps `vt+0x1f0`(핸들 → 엔티티) 순수 재현. ⬜0.5.8 오프셋 = 가설(layout.rs W_*) — 검증 = 스코어러 DIFF=0 + ghidra-re 대조.
    /// 게임 반환 NULL ↔ None. 읽기 실패도 None(구분이 필요해지면 Result 로 바꾼다).
    pub unsafe fn entity(&self, h: u64) -> Option<Ent> {
        let g = self.data;
        if h < rd_u64(g + W_L3_CNT)? {
            let t3 = rd_u64(g + W_L3_TBL)? as usize;
            let slot = t3.wrapping_add((h as usize).wrapping_mul(W_SLOT_STRIDE));
            if ptr_ok(t3) && ptr_ok(slot) && rd_i32(slot)? == 1 {
                let u = rd_u64(slot + 8)?;
                if u < rd_u64(g + W_ENT_CNT)? {
                    let base = rd_u64(g + W_ENT_BASE)? as usize;
                    let e = base.wrapping_add((u as usize).wrapping_mul(ENT_STRIDE));
                    if ptr_ok(base) && ptr_ok(e) { return Some(Ent(e)); }
                }
            }
        }
        if rd_u64(g + W_SINGLETON_H)? == h && rd_i32(g + W_SINGLETON_TAG)? != -1 { return Some(Ent(g + W_SINGLETON_TAG)); }
        None
    }

    /// 로스터: X+0x1e0 + side*0x28 + role*8 → 엔티티 ptr(0 = 없음). recall `0xcc5fc0` 실측.
    pub unsafe fn roster(&self, side: u64, role: u32) -> Option<usize> {
        if side > 1 || role > 4 { return None; }
        let p = rd_u64(self.x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + (role as usize) * 8)? as usize;
        Some(p)
    }
}

/// Plan 핸들러의 p6 = &Holder { X(월드), G(컨텍스트), … }. [디컴 0xccc010: `puVar2=*param_6`, `lVar7=param_6[1]`]
#[derive(Clone, Copy)]
pub struct Holder(pub usize);
impl Holder {
    pub unsafe fn new(p6: usize) -> Option<Holder> { if ptr_ok(p6) { Some(Holder(p6)) } else { None } }
    pub unsafe fn world(&self) -> Option<World> { World::from_holder(self.0 + HOLDER_X) }
    pub unsafe fn g(&self) -> Option<G> { let g = rd_u64(self.0 + HOLDER_G)? as usize; if ptr_ok(g) { Some(G(g)) } else { None } }
}

/// G(컨텍스트): +8 → cfg(tick/sec) · +0x20 → 홈존 박스 표.
#[derive(Clone, Copy)]
pub struct G(pub usize);
impl G {
    pub unsafe fn tps(&self) -> Option<i64> { let c = rd_u64(self.0 + G_CFG)? as usize; if !ptr_ok(c) { return None; } rd_i64(c + CFG_TPS) }
    /// side 진영 홈존 박스(부호없는 비교). [디컴 0xccc010 · 0xcaf9f0 아암16 동일]
    pub unsafe fn home_box(&self, side: u64) -> Option<HomeBox> {
        if side > 1 { return None; }
        let t = rd_u64(self.0 + G_BOXES)? as usize; if !ptr_ok(t) { return None; }
        let b = t + BOX_BASE + (side as usize) * BOX_STRIDE;
        Some(HomeBox { xlo: rd_u64(b)?, ylo: rd_u64(b + 8)?, xhi: rd_u64(b + 0x10)?, yhi: rd_u64(b + 0x18)? })
    }
}
#[derive(Clone, Copy)]
pub struct HomeBox { pub xlo: u64, pub ylo: u64, pub xhi: u64, pub yhi: u64 }
impl HomeBox { #[inline] pub fn contains(&self, x: u64, y: u64) -> bool { self.xlo <= x && x <= self.xhi && self.ylo <= y && y <= self.yhi } }

/// 엔티티(전투 유닛) 뷰. 값 접근자는 전부 Option(읽기 실패 = None).
#[derive(Clone, Copy)]
pub struct Ent(pub usize);
impl Ent {
    #[inline] pub unsafe fn handle(&self) -> Option<u64> { rd_u64(self.0 + ENT_HANDLE) }
    #[inline] pub unsafe fn level(&self) -> Option<u64> { rd_u64(self.0 + ENT_LEVEL) }
    #[inline] pub unsafe fn hp(&self) -> Option<i64> { rd_i64(self.0 + ENT_HP) }
    #[inline] pub unsafe fn max_hp(&self) -> Option<i64> { rd_i64(self.0 + ENT_MAXHP) }
    #[inline] pub unsafe fn x(&self) -> Option<i64> { rd_i64(self.0 + ENT_X) }
    #[inline] pub unsafe fn y(&self) -> Option<i64> { rd_i64(self.0 + ENT_Y) }
}

/// 선수 sim 상태(p5) 뷰.
#[derive(Clone, Copy)]
pub struct Player(pub usize);
impl Player {
    #[inline] pub unsafe fn side(&self) -> Option<u64> { rd_u64(self.0 + P5_SIDE) }
    #[inline] pub unsafe fn role(&self) -> Option<u32> { if ptr_ok(self.0) { Some(rd_u32(self.0 + P5_ROLE)) } else { None } }
}

/// SmallAction(p7) 뷰.
#[derive(Clone, Copy)]
pub struct SmallAction(pub usize);
impl SmallAction {
    #[inline] pub unsafe fn tag(&self) -> Option<u8> { if ptr_ok(self.0) { Some(rd_u8(self.0 + SA_TAG)) } else { None } }
    #[inline] pub unsafe fn u64_at(&self, off: usize) -> Option<u64> { rd_u64(self.0 + off) }
}
