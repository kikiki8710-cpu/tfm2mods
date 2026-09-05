//! hunt_battle — Plan 핸들러 `hunt_and_battle`(epic disc 13 `0xccc010` · serpen disc 15 `0xccc3c0`) 공통 본체.
//!   두 함수는 목표 슬롯(T0/T1)·p7 타이머 오프셋·else 코드(0xb/0xe)만 다르고 나머지 143명령이 동일하다(디컴 대조).
//!   게임 0.5.8 · 원본 행 29~32 · 콜리 = `0xe7a8c0`(lib.rs:1626 ability_pick — **RNG 소비**) · vt+0x40(assert) · vt+0x1f0(리졸버)
//!
//! 계약(디컴): p1=out(MovePriority) · p2=Plan payload(목표 슬롯 보유) · p5=선수 sim · p6=&Holder(X·G) · p7=타이머 구조체
//!   side=[p5+0x930](≥2 panic) · role=[p5+0x9c0] · ent=X 로스터[side*5+role] (0 → panic)
//!   maxhp=[ent+0x628](0 → panic) · hp=[ent+0x670] · pct=hp*100/maxhp
//!   (tag, mode) = vt+0x40(data); tag!=0 → panic ; tgt = mode.T_LEN!=0 ? 리졸버(**mode.T_PTR) : NULL   (모드 데이터 = world+0xed00)
//!   in_home = 홈존 박스(G→+0x20→+0x6d70+side*0x20) 안에 (x,y)
//!   pick = ability_pick(&local, p3, p4(rng), p5, data, vt, G)  ← 검증 단계에서는 게임 콜리 캡처값 사용(포팅 전)
//!   if tgt==NULL || tgt.hp!=tgt.maxhp || (!(hp<maxhp && in_home) && pct>50 && pick.r0==0):
//!        if [p7+TB] < tps + [p7+TA] && payload[0]!=0 : out+8=[payload+8], out+0x10(u16)=1, out+0x12(u8)=0, code=9
//!        else                                        : out+8(u8)=0, code=CODE_ELSE(0xb epic / 0xe serpen)
//!   else code=5
//!   ⚠쓰기집합이 경로마다 다르다(잔재 유지) — MpOut 으로 정확히 그 바이트만 쓴다/대조한다.
//!
//! live 승격 조건: ability_pick(0xe7a8c0) 순수 포팅(RNG 재현 `RngSim` + 하위 `0x105fae0` + dyn-desc 슬롯 0x50/0x68/0x70/0x80) 완료 후.
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::super::{Args8, MpOut};
use super::super::cap_ability_pick;

pub struct HbVariant { pub t_len: usize, pub t_ptr: usize, pub p7_ta: usize, pub p7_tb: usize, pub code_else: u64, pub site_phase_imm: usize }
pub const EPIC: HbVariant = HbVariant { t_len: T0_LEN, t_ptr: T0_PTR, p7_ta: P7_EPIC_TA, p7_tb: P7_EPIC_TB, code_else: MP_CODE_EPIC_HB, site_phase_imm: SITE_HD_PHASE_EPIC_IMM };
pub const SERPEN: HbVariant = HbVariant { t_len: T1_LEN, t_ptr: T1_PTR, p7_ta: P7_SERPEN_TA, p7_tb: P7_SERPEN_TB, code_else: MP_CODE_SERPEN_HB, site_phase_imm: SITE_HD_PHASE_SERPEN_IMM };

/// 반환 None = 게임이 panic 하는 경로·읽기 실패·콜리 캡처 없음 → 검증 NA / live passthrough.
pub unsafe fn hunt_battle(a: &Args8, v: &HbVariant) -> Option<MpOut> {
    let (payload, p5, p6, p7) = (a.p2, a.p5, a.p6, a.p7);
    if !ptr_ok(payload) || !ptr_ok(p5) || !ptr_ok(p7) { return None; }
    let side = rd_u64(p5 + P5_SIDE)?;
    if side > 1 { return None; }                                   // 게임: panic
    let role = rd_u32(p5 + P5_ROLE);
    let h = Holder::new(p6)?;
    let w = h.world()?;
    let ent = w.roster(side, role)?;
    if ent == 0 { return None; }                                   // 게임: panic(unwrap None)
    let e = Ent(ent);
    let maxhp = e.max_hp()? as u64;
    if maxhp == 0 { return None; }                                 // 게임: panic(div by zero)
    let hp = e.hp()? as u64;
    let pct = hp.wrapping_mul(100) / maxhp;
    // vt+0x40(data) = (tag, &mode_data): tag!=0 → 게임 panic(None). 목표 = 모드 데이터 핸들 Vec 첫 원소 → 리졸버.
    //   ★[2026-09-06 정정] 첫 포팅은 payload(p2) 기준으로 읽었다 — 디컴의 `*(uVar11+0x1a8)` 는 vt+0x40 이 rdx 로 돌려준 모드 데이터다.
    let moba = w.moba()?;
    let tgt = match w.first_target(moba, v.t_len, v.t_ptr)? { Some(h) => { super::super::tr(3, h); w.entity(h) } None => None };
    super::super::tr(1, moba as u64); super::super::tr(4, tgt.map(|t| t.0 as u64).unwrap_or(0));
    let g = h.g()?;
    let in_home = g.home_box(side)?.contains(e.x()? as u64, e.y()? as u64);
    let pick = cap_ability_pick::take()?;                          // 게임 콜리 결과(검증 전용). live 전환 시 순수 포팅으로 교체.
    let tgt_full = match tgt { Some(t) => t.hp()? == t.max_hp()?, None => false };
    let mut o = MpOut::default();
    let go = tgt.is_none() || !tgt_full || (!(hp < maxhp && in_home) && (pct > 0x32 && pick.0 == 0));
    if !go { o.code(MP_CODE_AROUND); return Some(o); }
    let tps = g.tps()? as u64;
    if rd_u64(p7 + v.p7_tb)? < tps.wrapping_add(rd_u64(p7 + v.p7_ta)?) && rd_u8(payload) != 0 {
        o.push(0x8, 8, rd_u64(payload + 8)?);
        o.push(0x10, 2, super::super::live_imm16(v.site_phase_imm, 1) as u64);   // 라이브 즉치(노브 hd_phase, 원본 1)
        o.push(0x12, 1, 0);
        o.code(MP_CODE_LINE_ATTACK);
    } else {
        o.push(0x8, 1, 0);
        o.code(v.code_else);
    }
    Some(o)
}
