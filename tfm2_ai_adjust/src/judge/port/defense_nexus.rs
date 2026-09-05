//! defense_nexus — game-ai\src\plan_legacy\old\defense_nexus.rs (Plan 17 넥서스 방어 핸들러)
//!   게임 0.5.8 · RVA 0xd2da10 (2104B / 524명령) · 콜리 0xd3fa80(넥서스 위협, 209명령 — 아래 `nexus_threat`) · 0xc87850(틱 메모 캐시, 캡처) · 0x10ea8e0/0x1821530(Vec 성장·3리스트 연결 = 순수 자료구조, 재현 불요)
//!   vtable 슬롯 0x1f0(핸들→엔티티) · 0x20/0x28(seed/tick — 캐시 키) · 콜리 안 0xf8/0x150/0x28
//! ★디컴은 못 쓴다: Ghidra 가 이 함수의 phase 점프테이블(0x33da0b8, 9엔트리)을 이웃 passive_jungle 테이블까지 이어 읽어 2,300행 오염 디컴을 낸다.
//!   이 포팅은 **capstone 디스어셈 526행 + 점프테이블 수동 디코드**(scratchpad dn_d2da10.txt) 로 옮겼다. 콜리 0xd3fa80/0xd3fe50 은 디컴이 깨끗했다.
//!
//! 계약: p1=out · p3=u64(≤1 / >1 분기 — 포인터 아님) · p5=선수 sim(+0x930 side·+0x9c0 role) · p6=&Holder(X,G,lanes) · p7=오더(+0xf0 byte)
//!   출력 write-set: code 5 (out+0=5) / code 0x11 (out+8(u64)=0 · out+0x18(u8)=0 · out+0=0x11)
//!
//! 판단 요지(사람말): 적 3레인 목표물이 내 넥서스 120000 안에 있는지(L0..L2) 를 국면(phase 0..8)별 우선표로 합쳐 `ok` 를 만들고,
//!   넥서스 HP≤50% 면 `nexus_threat`(적 챔프가 240000 안·보임/최근목격, 또는 적 미니언이 넥서스를 때리는 중) 로 즉시 방어(0x11).
//!   p3>1 이고 오더 플래그가 꺼져 있으면 틱 메모 캐시(0xc87850) 의 bit8(=0xd3fe50 "적 사거리가 넥서스에 닿음") 도 본다.
//!   그 외엔 (내 HP%·홈존 여부·적 근접 여부) 로 5(대기) / 0x11(방어) 를 가른다.
//!
//! ★바이트패치 사이트 11곳(`aiport sites` 실측) = 이 모드의 `nx_dn_*` 노브(`apply_objective_imm`, nx_enable 게이트). 검증판은 라이브 즉치, live 판은 cfg.
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::super::{Args8, MpOut, tr, live_imm8, live_imm64};

#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}

/// nx_dn_* 노브 묶음. 거리는 게임처럼 제곱(+1) 도메인으로 들고 다닌다.
#[derive(Clone, Copy)]
pub struct Knobs {
    pub near_d2_p1: u64,   // nx_dn_near_dist  원본 120000 → dist² < 120000²+1 (사이트 0xd2dc04/0xd2dd30)
    pub near_d2: u64,      //                  같은 노브 → dist² <= 120000² (사이트 0xd2df98)
    pub nexus_hp: u64,     // nx_dn_nexus_hp   원본 50  (넥서스 HP% <= N 이면 위협 검사)
    pub hp_low: u64,       // nx_dn_hp_low     원본 31  (내 HP% < N → 5)
    pub hp_crit: u64,      // nx_dn_hp_crit    원본 21  (내 HP% >= N || 홈존 → 0x11)
    pub pred_d2_p1: u64,   // nx_dn_pred_dist  원본 240000 → dist² < 240000²+1 (콜리 0xd3fc05)
    pub vision_mem: u64,   // nx_dn_vision_mem 원본 120 (콜리 0xd3fc73)
}
impl Knobs {
    pub unsafe fn game_equiv() -> Knobs {
        Knobs { near_d2_p1: live_imm64(SITE_DN_NEAR_P1_IMM, 14_400_000_001), near_d2: live_imm64(SITE_DN_NEAR_IMM, 14_400_000_000),
                nexus_hp: live_imm8(SITE_DN_NEXUS_HP_IMM, 50) as u64, hp_low: live_imm8(SITE_DN_HP_LOW_IMM, 31) as u64, hp_crit: live_imm8(SITE_DN_HP_CRIT_IMM, 21) as u64,
                pred_d2_p1: live_imm64(SITE_DN_PRED_P1_IMM, 57_600_000_001), vision_mem: live_imm8(SITE_DN_VISION_IMM, 0x78) as u64 }
    }
    /// apply_objective_imm 과 같은 규칙: nx_enable=0 이면 전부 원본.
    pub unsafe fn from_cfg() -> Knobs {
        let en = tune("nx_enable", 0) != 0;
        let k = |key: &str, orig: i64| if en { let v = tune(key, orig); if v < 0 { orig } else { v } } else { orig };
        let nd = k("nx_dn_near_dist", 120000) as u64; let pd = k("nx_dn_pred_dist", 240000) as u64;
        Knobs { near_d2_p1: nd * nd + 1, near_d2: nd * nd, nexus_hp: k("nx_dn_nexus_hp", 0x32) as u64, hp_low: k("nx_dn_hp_low", 0x1f) as u64,
                hp_crit: k("nx_dn_hp_crit", 0x15) as u64, pred_d2_p1: pd * pd + 1, vision_mem: k("nx_dn_vision_mem", 0x78) as u64 }
    }
}
pub unsafe fn defense_nexus(a: &Args8) -> Option<MpOut> { defense_nexus_k(a, &Knobs::game_equiv()) }
pub unsafe fn defense_nexus_live(a: &Args8) -> Option<MpOut> { defense_nexus_k(a, &Knobs::from_cfg()) }

/// 0xd3fa80 — "넥서스가 위협받는가": 적 챔프(보임 || 최근목격 vision_mem)가 넥서스 pred_dist 안 / 적 유닛(kind 1)이 넥서스를 공격 중.
pub unsafe fn nexus_threat(p5: usize, p6: usize, k: &Knobs) -> Option<bool> {
    let side = rd_u64(p5 + P5_SIDE)?; if side > 1 { return None; }
    let h = Holder::new(p6)?; let w = h.world()?;
    let nexus = rd_u64(w.x + X_NEXUS + (side as usize) * 8)? as usize; if nexus == 0 { return Some(false); }
    let other = 1 - side;
    let lanes = rd_u64(p6 + HOLDER_LANES)? as usize; if !ptr_ok(lanes) { return None; }
    let lane_o = lanes + (other as usize) * LANE_STRIDE;
    let (nx, ny) = (rd_u64(nexus + ENT_X)?, rd_u64(nexus + ENT_Y)?);
    let tick = rd_u64(w.data + W_TICK)?;
    for r in 0..5u32 {
        let e = w.roster(other, r)?; if e == 0 { continue; }
        let eh = rd_u64(e + ENT_HANDLE)?;
        let mut seen = w.visible(side, eh)?;
        if !seen {
            let rec = w.roster_rec(eh)?;
            if rec != 0 {
                let last = rd_u64(lane_o + LANE_ROSTER + (rd_u32(rec + REC_ROLE) as usize) * 8)?;
                seen = tick <= last.wrapping_add(k.vision_mem);
            }
        }
        if seen && sqd(rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?, nx, ny) < k.pred_d2_p1 { return Some(true); }
    }
    let nh = rd_u64(nexus + ENT_HANDLE)?;
    for i in 0..3usize {
        let ptr = rd_u64(w.x + X_LIST3_PTR[i] + (other as usize) * 0x20)? as usize;
        let len = rd_u64(w.x + X_LIST3_LEN[i] + (other as usize) * 0x20)?;
        if len == 0 { continue; } if !ptr_ok(ptr) { return None; }
        for j in 0..len.min(4096) as usize {
            let u = rd_u64(ptr + j * 8)? as usize; if !ptr_ok(u) { return None; }
            if rd_i32(u + ENT_KIND)? == 1 && rd_i32(u + ENT_F88)? == 1 && rd_u64(u + ENT_TARGET_H)? == nh { return Some(true); }
        }
    }
    Some(false)
}

pub unsafe fn defense_nexus_k(a: &Args8, k: &Knobs) -> Option<MpOut> {
    let (p3, p5, p6, p7) = (a.p3 as u64, a.p5, a.p6, a.p7);
    if !ptr_ok(p5) || !ptr_ok(p7) { return None; }
    let side = rd_u64(p5 + P5_SIDE)?; if side > 1 { return None; }
    let role = rd_u32(p5 + P5_ROLE);
    let h = Holder::new(p6)?; let w = h.world()?; let g = h.g()?;
    let lanes = rd_u64(p6 + HOLDER_LANES)? as usize; if !ptr_ok(lanes) { return None; }
    let me = w.roster(side, role)?; if me == 0 { return None; }                     // 게임: panic
    let maxhp = rd_u64(me + ENT_MAXHP)?; if maxhp == 0 { return None; }               // 게임: div0 panic
    let hp = rd_u64(me + ENT_HP)?; let pct = hp.wrapping_mul(100) / maxhp;
    let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    let in_home = g.home_box(side)?.contains(mx, my);
    let nexus = rd_u64(w.x + X_NEXUS + (side as usize) * 8)? as usize; if nexus == 0 { return None; }   // 게임: panic
    let (nx, ny) = (rd_u64(nexus + ENT_X)?, rd_u64(nexus + ENT_Y)?);
    let other = 1 - side;
    let lane_o = lanes + (other as usize) * LANE_STRIDE;
    // 적 3레인 목표물 → 내 넥서스 near 안인가 (L0..L2). 레인0 은 state 바이트 !=0, 레인1/2 는 i32 ==1 (디스어셈 그대로)
    let subs = [0usize, LANE_SUB_SIDE, LANE_SUB_MID];
    let mut l = [false; 3];
    for i in 0..3 {
        let rec = lane_o + subs[i];
        let on = if i == 0 { rd_u8(rec + LR_STATE) != 0 } else { rd_i32(rec + LR_STATE)? == 1 };
        if !on { continue; }
        if let Some(t) = w.entity(rd_u64(rec + LR_TARGET_H)?) {
            l[i] = sqd(rd_u64(t.0 + ENT_X)?, rd_u64(t.0 + ENT_Y)?, nx, ny) < k.near_d2_p1;
        }
    }
    let phase = rd_u8(g.0 + G_PHASE);
    tr(0, 0x100 | phase as u64); tr(1, 0x100 | (l[0] as u64) | (l[1] as u64) << 1 | (l[2] as u64) << 2); tr(2, 0x100 | pct.min(0xff)); tr(3, 0x100 | in_home as u64 | (p3.min(0x7f) << 1));
    // 국면별 우선표(점프테이블 0x33da0b8 → 상수표 3바이트 + A/B 초기 플래그)
    let (t, ainit, binit, six) = match phase {
        0 | 7 | 8 => ([0u8, 1, 2], false, false, false),
        1 | 3 => ([2u8, 0, 0], true, false, false),
        2 => ([0u8, 0, 0], true, false, false),
        4 => ([1u8, 0, 0], true, false, false),
        5 => ([1u8, 2, 0], false, true, false),
        6 => ([0u8, 0, 0], false, false, true),
        _ => return None,                                                            // 테이블 밖(9엔트리) — 게임은 UB
    };
    let chain = |f: &dyn Fn(u8) -> bool| -> bool { let c0 = f(t[0]); if ainit || c0 { c0 } else { let c1 = f(t[1]); if binit || c1 { c1 } else { f(t[2]) } } };
    let ok: bool = if six { false }
        else if l[2] { if l[1] { if l[0] { false } else { chain(&|x| x == 0) } } else if l[0] { chain(&|x| x == 1) } else { chain(&|x| x != 2) } }
        else if l[1] { if l[0] { chain(&|x| x == 2) } else { chain(&|x| x != 1) } }
        else if l[0] { chain(&|x| x != 0) } else { true };
    // 적 5명 중 넥서스 near 안(<=) 이 하나라도
    let mut near = false;
    for r in 0..5u32 {
        let e = w.roster(other, r)?; if e == 0 { continue; }
        if sqd(rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?, nx, ny) <= k.near_d2 { near = true; break; }
    }
    tr(4, 0x100 | ok as u64 | (near as u64) << 1);
    let mut o = MpOut::default();
    let code5 = |o: &mut MpOut| { o.code(5); };
    let code11 = |o: &mut MpOut| { o.push(0x8, 8, 0); o.push(0x18, 1, 0); o.code(0x11); };
    let nexus_pct = || -> Option<u64> { let m = rd_u64(nexus + ENT_MAXHP)?.max(1); Some(rd_u64(nexus + ENT_HP)?.wrapping_mul(100) / m) };
    if p3 > 1 {
        if rd_u8(p7 + ORDER_F0) != 0 { code5(&mut o); return Some(o); }
        let np = nexus_pct()?; tr(5, 0x100 | np.min(0xff));
        if np <= k.nexus_hp && nexus_threat(p5, p6, k)? { code11(&mut o); return Some(o); }
        // 틱 메모 캐시(0xc87850) bit8 = 0xd3fe50 "적 사거리가 넥서스에 닿음" — 검증 단계는 캡처값
        let flags = super::super::cap_dn_cache::take()?; tr(6, 0x1_0000 | (flags & 0xffff));
        if flags & 0x100 != 0 { code11(&mut o); return Some(o); }
    } else {
        let np = nexus_pct()?; tr(5, 0x100 | np.min(0xff));
        if np <= k.nexus_hp && nexus_threat(p5, p6, k)? { code11(&mut o); return Some(o); }
    }
    // E18B
    let th = nexus_threat(p5, p6, k)?; tr(7, 0x100 | th as u64);
    if (!ok && near) || th {
        if pct >= k.hp_crit || in_home { code11(&mut o); } else { code5(&mut o); }
        return Some(o);
    }
    if pct < k.hp_low { code5(&mut o); return Some(o); }
    if hp < maxhp && in_home { code5(&mut o); } else { code11(&mut o); }
    Some(o)
}
