//! battle — Plan 핸들러 disc 9(Battle) `0xdfdfc0`(733B, census 밖 = 패닉 Location 없음). 게임 0.5.8 · capstone 디스어셈 + REST 디컴 포팅(2026-09-06 15:20).
//!   리플레이 실측 발화 = 판당 ~72만(passive_line 다음 2위).
//!
//! 계약: p1=out(MovePriority) · p2=Plan payload(디스패처가 `add rdx,8` 한 뒤 넘김) · p5=선수 sim(+0x930 side) · p6=&Holder(X: +0 world data · +8 vt · 리스트)
//!   payload: [+0..+0x10] 16B 그대로 out+8 로 복사 · +0x58 kind(u64) · +0x60 target 핸들 · +0xf6 flag · +0xfd fd · +0xfe lane(0xff=없음)
//!   flag==0 → in_range=0·chase=0 (계산 없음)
//!   flag!=0 && (0x6f >> (kind&63))&1 && 리졸버(target)!=NULL && lane!=0xff :
//!       적(1-side) 유닛 = 0x1820ee0 = 고정 6슬롯(X+0x180/0x1a0/0x1c0/0x190/0x1b0/0x1d0 + side*8, NULL 제외) + 미니언 슬라이스(X+0x130/+0x148 + side*0x20)
//!       그중 kind(+0x68)==2 이고 lane(+0x128)==lane (lane∈{3,4} 이면 {3,4} 모두) 인 유닛 e 에 대해
//!         reach² = slot0.flag(+0x4c0)==-1 ? 120000² : ([e+0x438] + [e+0x4a0] + (level−1)*[e+0x4a8] + 120000)²
//!         dist²(target, e) <= reach² 이면 in_range=1 (첫 일치에서 중단)
//!   chase = (fd==1) ? 0 : !in_range
//!   out: code 7 · +8/+0x10 = payload[0..16] · +0x18 kind · +0x20 handle · +0x28 0 · +0x30 chase · +0x31 flag · +0x32 in_range · +0x33 0 · +0x34 fd · +0x35 0
//!   ⚠0x1820ee0 은 Vec 을 힙에 만들고 핸들러는 해제하지 않는다(게임 쪽 누수/아레나 여부 무관) — 재현은 같은 순서로 순회만 한다.
//! 노브: `bt_tower_margin`(기본 120000 = 게임값·바이트패치 사이트 없음) — 타워 사거리 여유. from_cfg 가 아닌 한 게임 동치.
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::super::{Args8, MpOut, tr};

#[derive(Clone, Copy)]
pub struct Knobs { pub margin: u64 }
impl Knobs {
    pub fn game_equiv() -> Knobs { Knobs { margin: BT_TOWER_MARGIN } }
    pub fn from_cfg() -> Knobs { Knobs { margin: tune("bt_tower_margin", BT_TOWER_MARGIN as i64).max(0) as u64 } }
}
pub unsafe fn battle(a: &Args8) -> Option<MpOut> { battle_k(a, &Knobs::game_equiv()) }
pub unsafe fn battle_live(a: &Args8) -> Option<MpOut> { battle_k(a, &Knobs::from_cfg()) }

#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}

/// 0x1820ee0 의 순회 순서 그대로 적 유닛 포인터를 콜백에 넘긴다(고정 6슬롯 → 미니언). 콜백이 true 를 돌려주면 중단하고 Some(true).
unsafe fn for_each_unit(x: usize, side: u64, mut f: impl FnMut(usize) -> Option<bool>) -> Option<bool> {
    if side > 1 { return None; }
    let s = side as usize;
    for off in X_FIXED6 { let e = rd_u64(x + off + s * 8)? as usize; if e != 0 { if f(e)? { return Some(true); } } }
    let ptr = rd_u64(x + X_MINION_PTR + s * 0x20)? as usize; let len = rd_u64(x + X_MINION_LEN + s * 0x20)?;
    if len != 0 && !ptr_ok(ptr) { return None; }
    for i in 0..len.min(CAP_ITER) as usize { let e = rd_u64(ptr + i * 8)? as usize; if f(e)? { return Some(true); } }
    Some(false)
}

pub unsafe fn battle_k(a: &Args8, k: &Knobs) -> Option<MpOut> {
    let (p2, p5, p6) = (a.p2, a.p5, a.p6);
    if !ptr_ok(p2) { return None; }
    let flag = rd_u8(p2 + BT_PL_FLAG); let kind = rd_u64(p2 + BT_PL_KIND)?; let handle = rd_u64(p2 + BT_PL_HANDLE)?;
    let fd = rd_u8(p2 + BT_PL_FD);
    let (w0, w1) = (rd_u64(p2)?, rd_u64(p2 + 8)?);
    let mut in_range: u8 = 0;
    let mut chase: u8 = 0;
    if flag != 0 {
        tr(0, 0x100 | (kind & 0xff) | (fd as u64) << 8 | (rd_u8(p2 + BT_PL_LANE) as u64) << 16);
        if (0x6fu64 >> (kind & 63)) & 1 == 1 {
            if !ptr_ok(p5) { return None; }
            let h = Holder::new(p6)?; let w = h.world()?;
            if let Some(t) = w.entity(handle) {
                let lane = rd_u8(p2 + BT_PL_LANE);
                if lane != 0xff {
                    let side = rd_u64(p5 + P5_SIDE)?; if side > 1 { return None; }   // 게임: index panic(0x1820ee0)
                    let (tx, ty) = (rd_u64(t.0 + ENT_X)?, rd_u64(t.0 + ENT_Y)?);
                    let wide = lane.wrapping_sub(3) < 2;                                   // lane 3·4 → {3,4} 모두 매치
                    let hit = for_each_unit(w.x, 1 - side, |e| {
                        if rd_u32(e + ENT_KIND) != 2 { return Some(false); }
                        let l = rd_u8(e + ENT_LANE);
                        if !(l == lane || (wide && l.wrapping_sub(3) < 2)) { return Some(false); }
                        let reach2 = if rd_i32(e + ENT_SLOT0_FLAG)? == -1 { k.margin.wrapping_mul(k.margin) } else {
                            let r = rd_u64(e + ENT_F438)?.wrapping_add(rd_u64(e + ENT_SLOT0 + 0x10)?)
                                .wrapping_add(rd_u64(e + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(rd_u64(e + ENT_SLOT0 + 0x18)?))
                                .wrapping_add(k.margin);
                            r.wrapping_mul(r)
                        };
                        Some(sqd(tx, ty, rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?) <= reach2)
                    })?;
                    in_range = hit as u8;
                    tr(1, 0x100 | hit as u64);
                }
            }
        }
        chase = if fd == 1 { 0 } else { in_range ^ 1 };
    }
    let mut o = MpOut::default();
    o.push(8, 8, w0); o.push(0x10, 8, w1); o.push(0x18, 8, kind); o.push(0x20, 8, handle); o.push(0x28, 8, 0);
    o.push(0x30, 1, chase as u64); o.push(0x31, 1, flag as u64); o.push(0x32, 1, in_range as u64);
    o.push(0x33, 1, 0); o.push(0x34, 1, fd as u64); o.push(0x35, 1, 0);
    o.code(7);
    tr(2, 0x100 | chase as u64 | (in_range as u64) << 1 | (flag as u64) << 2);
    Some(o)
}
