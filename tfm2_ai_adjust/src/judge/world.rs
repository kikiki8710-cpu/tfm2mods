//! world — 판단 함수가 읽는 게임 상태를 **메모리 읽기만으로** 제공한다(게임 함수 호출 0 — CLAUDE.md §3 완전 재구현 원칙).
//! 읽기는 전부 `rd_*`(VEH 경유 safe read). 오프셋은 layout.rs(버전 태그). 실패 = None(호출부가 가드로 취급 → passthrough).
#![allow(dead_code)]
use crate::*;
use super::layout::*;

/// World-ish X 의 (data, vtable). `*p6 = X`, `X+0 = data`, `X+8 = WorldOps vt` — steal `0xcbbca0` 진입부 실측
/// (`mov rsi,[rdx]; mov rcx,[rsi]; mov rax,[rsi+8]`).
#[derive(Clone, Copy)]
pub struct World { pub x: usize, pub data: usize, pub vt: usize }

/// ★[2026-09-08] `AbstractGame` vt 게터 정적 디코드 — `vt+0x20 = seed()` · `vt+0x28 = tick()`.
///   Game/SingleLaneGame/DeathMatchGame 은 `[self+0xec90]/[self+0xec98]` 이지만 **`ExpectedGame`(presim 전용)은
///   `tick() = [self+0x90]`(자기 필드) · `seed() = 내부 게임 vt+0x20 으로 위임`**(_gaibc/m04.ll:67448 · _gcbc/g08.ll:203429).
///   고정 오프셋 `W_TICK` 직독은 presim 스레드에서 틱이 어긋나 해시 계수·메모 에포크·시각 비교가 전부 틀렸다
///   (as_eb82d0/as_d84db0 의 스레드 2개 한정 대칭 DIFF 버스트의 원인). 디코드 실패 시 위임 1단(내부 fat ptr)으로 재귀.
pub unsafe fn ag_get(data: usize, vt: usize, slot: usize, depth: u32) -> Option<u64> {
    if !ptr_ok(data) || !ptr_ok(vt) { return None; }
    let f = rd_u64(vt + slot)? as usize;
    if let Some(v) = crate::judge::port::as_callees::decode_getter(f, data) { return Some(v); }
    if depth < 3 {
        let (d2, v2) = (rd_u64(data)? as usize, rd_u64(data + 8)? as usize);
        let b = exe_base();
        if ptr_ok(d2) && ptr_ok(v2) && b != 0 && v2 > b && v2 - b < 0x4000000 { return ag_get(d2, v2, slot, depth + 1); }
    }
    crate::judge::port::dyn_eff::unseen(0x2000 + slot as u32, f.wrapping_sub(exe_base()));
    None
}
pub unsafe fn game_tick(data: usize, vt: usize) -> Option<u64> { ag_get(data, vt, 0x28, 0) }
pub unsafe fn game_seed(data: usize, vt: usize) -> Option<u64> { ag_get(data, vt, 0x20, 0) }

impl World {
    #[inline] pub unsafe fn tick(&self) -> Option<u64> { game_tick(self.data, self.vt) }
    #[inline] pub unsafe fn seed(&self) -> Option<u64> { game_seed(self.data, self.vt) }
    /// ★[2026-09-08] 이 World 가 `ExpectedGame`(presim 전용 AbstractGame 구현체)인가 — 판정 = `vt+0x28`(tick) 게터가
    ///   `mov rax,[rcx+0x90]; ret`(ExpectedGame.tick 필드). 레이아웃(_gcbc/g08.ll DI): `game:&dyn AbstractGame`(+0/+8) ·
    ///   `mutable_entities: HashMap<usize, Entity>`(+0x10, RawTable ctrl +0x10 · bucket_mask +0x18 · items +0x28) ·
    ///   `mutable_projectiles`(+0x50) · `tick`(+0x90). **기본 게임 오프셋(W_*)을 이 객체에 대고 읽으면 전부 쓰레기**다.
    pub unsafe fn is_expected(&self) -> bool {
        let f = rd_u64(self.vt + 0x28).unwrap_or(0) as usize;
        ptr_ok(f) && rd_u8(f) == 0x48 && rd_u8(f + 1) == 0x8b && rd_u8(f + 2) == 0x81 && rd_i32(f + 3) == Some(0x90) && rd_u8(f + 7) == 0xc3
    }
    /// ExpectedGame 이면 내부 게임(`[data]`,`[data+8]`)으로 내려간다(최대 3단). W_* 고정 오프셋 읽기는 **반드시** 이 결과에 대고 한다.
    ///   ExpectedGame 의 AbstractGame 메서드는 `is_visible/is_visible_cell/get_game_mode/cfg/side_cfg/...` 전부 내부 위임
    ///   (_gcbc/g08.ll:200059 등) — `get_entity_by_id` 만 예외(오버레이 우선, `entity()` 참조).
    pub unsafe fn base(&self) -> World {
        let mut w = *self;
        for _ in 0..3 {
            if !w.is_expected() { break; }
            let (d, v) = (rd_u64(w.data).unwrap_or(0) as usize, rd_u64(w.data + 8).unwrap_or(0) as usize);
            if !ptr_ok(d) || !ptr_ok(v) { break; }
            w = World { x: w.x, data: d, vt: v };
        }
        w
    }
    /// `ExpectedGame::get_entity_by_id`(g08.ll:201090) 의 오버레이 조회: `mutable_entities` 에 h 가 있으면 그 **수정본 Entity**.
    ///   hashbrown RawTable: ctrl=[+0x10] · bucket_mask=[+0x18] · items=[+0x28]; 버킷 i 는 `ctrl - (i+1)*0x6c8`, `{key u64, Entity 0x6c0}`.
    ///   해시 탐색 대신 전 버킷 선형 조회(ctrl 바이트 최상위 비트 0 = 채워짐) — 결과는 동일(키 유일).
    pub unsafe fn overlay_entity(&self, h: u64) -> Option<usize> {
        let d = self.data;
        let items = rd_u64(d + 0x28)?; if items == 0 { return None; }
        let ctrl = rd_u64(d + 0x10)? as usize; let mask = rd_u64(d + 0x18)?;
        if !ptr_ok(ctrl) || mask > 4096 { return None; }
        for i in 0..=mask as usize {
            if rd_u8(ctrl + i) & 0x80 != 0 { continue; }
            let start = ctrl.wrapping_sub((i + 1) * (ENT_STRIDE + 8));
            if !ptr_ok(start) { return None; }
            if rd_u64(start)? == h { return Some(start + 8); }
        }
        None
    }
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
        if self.is_expected() { if let Some(e) = self.overlay_entity(h) { return Some(Ent(e)); } return self.base().entity(h); }
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

    /// vt+0x40 순수 재현: 모드 태그(0=MOBA) 와 모드 데이터 포인터. [ghidra-re 2026-09-06]
    pub unsafe fn mode(&self) -> Option<(u8, usize)> {
        if self.is_expected() { return self.base().mode(); }
        if !ptr_ok(self.data) { return None; }
        // ★태그는 vtable 별 상수(모노모픽) — vt+0x40 구현 RVA 로 판정한다(순수 read). `w+0xecc2` 바이트는 보조 기록용.
        //   [2026-09-06 03:05 검증판] 바이트 기준 판정은 500/500 NA(tag!=0) — 그 바이트는 모드 태그가 아니었다.
        let impl_rva = self.slot_target_rva(VT_WORLD_MODE);
        super::tr(8, self.vt as u64); super::tr(9, impl_rva as u64); super::tr(10, rd_u8(self.data + W_MODE_TAG) as u64 | 0x100);
        let tag = match impl_rva { VT40_IMPL_MOBA => 0u8, VT40_IMPL_TAG1 => 1, VT40_IMPL_TAG2 => 2, _ => return None };
        Some((tag, self.data + if tag == 0 { W_MODE_DATA_MOBA } else { W_MODE_DATA_OTHER }))
    }
    /// MOBA 모드 데이터(아니면 None = 게임은 panic 경로).
    pub unsafe fn moba(&self) -> Option<usize> { let (t, m) = self.mode()?; if t == 0 { Some(m) } else { None } }
    /// 모드 데이터의 목표 핸들 Vec(len_off/ptr_off) 첫 핸들. None = Vec 비어 있음(게임: 목표 없음). Err 대신 (found, handle) 로 구분.
    pub unsafe fn first_target(&self, moba: usize, len_off: usize, ptr_off: usize) -> Option<Option<u64>> {
        if rd_u64(moba + len_off)? == 0 { return Some(None); }
        let p = rd_u64(moba + ptr_off)? as usize;
        if !ptr_ok(p) { return None; }
        Some(Some(rd_u64(p)?))
    }
    /// vt+0xf8 순수 재현: 핸들 h 가 viewer side 에 지금 보이는가. [ghidra-re 2026-09-06 = 0.5.0 vt0x68 동일]
    pub unsafe fn visible(&self, viewer_side: u64, h: u64) -> Option<bool> {
        if viewer_side > 1 { return None; }
        if self.is_expected() { return self.base().visible(viewer_side, h); }
        match self.entity_slotmap(h)? { Some(e) => Some(rd_u64(e + ENT_VIS_BASE + (viewer_side as usize) * ENT_VIS_STRIDE)? == 0), None => Some(false) }
    }
    /// 슬롯맵 경로만(싱글턴 폴백 없음) — vt+0xf8 이 쓰는 형태.
    pub unsafe fn entity_slotmap(&self, h: u64) -> Option<Option<usize>> {
        if self.is_expected() { return self.base().entity_slotmap(h); }
        let g = self.data;
        if h < rd_u64(g + W_L3_CNT)? {
            let t3 = rd_u64(g + W_L3_TBL)? as usize;
            let slot = t3.wrapping_add((h as usize).wrapping_mul(W_SLOT_STRIDE));
            if ptr_ok(t3) && ptr_ok(slot) && rd_i32(slot)? == 1 {
                let u = rd_u64(slot + 8)?;
                if u < rd_u64(g + W_ENT_CNT)? {
                    let base = rd_u64(g + W_ENT_BASE)? as usize;
                    let e = base.wrapping_add((u as usize).wrapping_mul(ENT_STRIDE));
                    if ptr_ok(base) && ptr_ok(e) { return Some(Some(e)); }
                }
            }
        }
        Some(None)
    }
    /// vt+0x150 순수 재현: 핸들 → AI 로스터 레코드(0 = 없음). 선형스캔 stride 0x9e0. [ghidra-re 2026-09-06]
    pub unsafe fn roster_rec(&self, h: u64) -> Option<usize> {
        if self.is_expected() { return self.base().roster_rec(h); }
        let cnt = rd_u64(self.data + W_ROSTER_REC_CNT)?;
        if cnt == 0 { return Some(0); }
        let base = rd_u64(self.data + W_ROSTER_REC_BASE)? as usize;
        if !ptr_ok(base) { return None; }
        for i in 0..cnt.min(64) as usize {
            let rec = base + i * REC_STRIDE;
            if rd_u64(rec + REC_ALIVE)? != 0 && rd_u64(rec + REC_HANDLE)? == h { return Some(rec); }
        }
        Some(0)
    }
    /// ★`FUN_140d31bb0(W, handle)` 순수 이식 — `roster_rec` 와 **다른 배열을 뒤진다**.
    /// 이쪽은 로스터 10칸(`W+0x1e0..0x228`, 팀0 0..4 / 팀1 5..9)을 엔티티 포인터로 훑어
    /// `[e+0x5c0] == handle` 인 칸의 **평행 레코드 배열** `[W+0x230+i*8]` 을 돌려준다.
    /// ~~`roster_rec`(=`w.data` 의 레코드 리스트 선형스캔)~~ 는 대개 같은 답을 내지만
    /// 로스터에서 빠진 유닛에서 갈린다(RE 2026-09-07, `0xd5f350` 호출부).
    pub unsafe fn rec_by_roster(&self, h: u64) -> Option<usize> {
        for i in 0..10usize {
            let e = rd_u64(self.x + X_ROSTER + i * 8)? as usize;
            if e != 0 && rd_u64(e + ENT_HANDLE)? == h { return Some(rd_u64(self.x + 0x230 + i * 8)? as usize); }
        }
        Some(0)
    }
    /// vt+0xe8 순수 재현: 설정 플래그 u8.
    pub unsafe fn cfg_flag(&self) -> Option<u8> { let b = self.base(); if ptr_ok(b.data) { Some(rd_u8(b.data + W_CFG_FLAG)) } else { None } }
    /// vt+0x108 순수 재현: 사이드별 24B 설정의 주소(호출부가 필요한 필드만 읽는다).
    pub unsafe fn side_cfg(&self, side: u64) -> Option<usize> { if side > 1 { return None; } Some(self.base().data + W_SIDE_CFG + (side as usize) * W_SIDE_CFG_STRIDE) }
    /// vt+0x290 순수 재현: 사이드 킬 카운터 (side0, side1).
    pub unsafe fn kills(&self) -> Option<(u64, u64)> { let b = self.base(); Some((rd_u64(b.data + W_KILLS)?, rd_u64(b.data + W_KILLS + 8)?)) }

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
