//! `plan_legacy::types::BigPlan::sub_plan`(RVA `0xcaf9f0`) — **모든 플랜의 서브플랜 결정이 지나는 깔때기**.
//!   정본 = SDK IR `_gaibc\m02.ll` `BigPlan::sub_plan`(types.rs L233~250) + 인라인된 각 플랜 `sub_plan`.
//!   ★RVA 확정 근거(2026-09-09, capstone): 0xcaf9f0 안에 `cmp r11,[r9+r10+0x6d70]`(진영 사각형, r10=side*32)
//!     → `[r8+0x660]/[0x668]`(x,y) 범위 → `[r8+0x670] < [r8+0x628]`(hp<maxhp) → `mov r9d,5` 가 그대로 있다.
//!     이는 IR `AttackNexusPlan::sub_plan`(attack_nexus.rs L36~52) 과 완전 일치한다.
//!   계약: `sub_plan(sret[72], self, version, p3, player, data, p6, p7, p8, p9)` — Win64 로
//!     `p1=sret · p2=self · p3=version · p4=p3 · p5=player · p6=data · p7.. = 나머지`.
//!   분기 = `idx = if self[0] > 1 { self[0] - 2 } else { 4 }` 의 16-arm 스위치(`self[0] != 6` 가정).
//!
//! ★출력은 **arm 마다 쓰는 바이트가 다르다**(나머지는 콜러 스택 잔재) → `MpOut` 쓰기집합으로 돌려주고
//!   대조는 **쓴 바이트만** 한다. 전 바이트 비교는 가짜 DIFF 를 만든다(이 모드가 2026-07-22 에 같은 성질로 크래시).
//!   각 플랜 arm 은 이미 개별 RVA 로 포팅·검증된 핸들러에 **위임**한다 — 그것들이 곧 `*Plan::sub_plan` 이다.
#![allow(dead_code)]
use crate::*;
use super::super::layout::*;
use super::super::{Args8, MpOut};
use super::combat_score::na_tag;

const AN_REGION: usize = 28016;      // cfg + 28016 + side*32 = 팀별 (lx, ly, rx, ry)
const TOWERV_LEN: usize = 328;
#[inline] fn lv(key: &str) -> bool { crate::judge::live_mode(key) == 2 }       // x + 328 + side*32 = 그 팀 여분 타워 Vec 의 len

/// arm 14 — `AttackNexusPlan::sub_plan`(m12.ll:34867, attack_nexus.rs L36~52)
unsafe fn arm_attack_nexus(sf: usize, player: usize, data: usize) -> Option<MpOut> {
    let mut o = MpOut::default();
    let side = rd_u64(player + P5_SIDE)?; if side > 1 { return None; }
    let role = rd_u32(player + P5_ROLE) as usize; if role >= 5 { return None; }
    let x = rd_u64(data)? as usize; if !ptr_ok(x) { return None; }
    let me = rd_u64(x + X_ROSTER + (side as usize) * 0x28 + role * 8)? as usize;
    if me == 0 { return None; }                              // 게임은 unwrap 패닉
    let ctx = rd_u64(data + 8)? as usize; if !ptr_ok(ctx) { return None; }
    let cfg = rd_u64(ctx + 32)? as usize; if !ptr_ok(cfg) { return None; }
    let reg = cfg + AN_REGION + (side as usize) * 32;
    let (lx, ly, rx, ry) = (rd_u64(reg)?, rd_u64(reg + 8)?, rd_u64(reg + 16)?, rd_u64(reg + 24)?);
    let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    if mx >= lx && mx <= rx && my >= ly && my <= ry && rd_u64(me + ENT_HP)? < rd_u64(me + ENT_MAXHP)? {
        o.code(5); return Some(o);                           // 진영 안 + 체력 부족 → 5
    }
    let opp = 1 - side;
    if rd_u64(x + TOWERV_LEN + (opp as usize) * 32)? == 0 { o.code(16); return Some(o); }
    o.push(8, 1, 0); o.push(9, 1, rd_u8(sf + 8) as u64); o.push(10, 1, 2); o.code(2);
    Some(o)
}

/// arm 3 — 인라인(types.rs L237, 콜리 L880): 지원 목표·목표점·전술을 그대로 옮겨 담고 태그 7
unsafe fn arm_support(sf: usize) -> Option<MpOut> {
    let mut o = MpOut::default();
    let tactic = rd_u8(sf + 147); let with_dive = rd_u8(sf + 144);
    o.push(8, 8, rd_u64(sf + 8)?);
    o.push(16, 8, rd_u64(sf + 16)?);
    o.push(24, 8, rd_u64(sf + 96)?);
    o.push(32, 8, rd_u64(sf + 104)?);
    o.push(40, 8, 0);
    o.push(48, 1, ((tactic != 1) && (with_dive & 1 == 1)) as u64);
    o.push(49, 1, with_dive as u64);
    o.push(50, 2, 0);                                        // +50, +51 = 0 (2바이트 묶음)
    o.push(52, 1, tactic as u64);
    o.push(53, 1, 0);
    o.code(7);
    Some(o)
}

/// arm 4 — 인라인(types.rs L238, 콜리 L1669) = **라인전(plan 0·1, 실측 최다)**. 태그 = `self[0]` 그대로.
unsafe fn arm_line(sf: usize, variant: u64) -> Option<MpOut> {
    let mut o = MpOut::default();
    o.code(variant);
    o.push(8, 8, rd_u64(sf + 8)?);
    // memcpy 24B: self+168 → out+16 (8바이트 3개로 쪼갠다 — MpWrite 최대 len 8)
    o.push(0x10, 8, rd_u64(sf + 0xa8)?);   // movups [rsi+0x10] ← [rdx+0xa8] (16B)
    o.push(0x18, 8, rd_u64(sf + 0xb0)?);
    o.push(0x20, 8, rd_u64(sf + 0xb8)?);   // mov [rsi+0x20] ← [rdx+0xb8]
    o.push(0x28, 8, rd_u64(sf + 0xe8)?);   // movups [rsi+0x28] ← [rdx+0xe8] (16B)
    o.push(0x30, 8, rd_u64(sf + 0xf0)?);
    o.push(0x38, 8, rd_u64(sf + 0x120)?);  // mov [rsi+0x38] ← [rdx+0x120]
    // 0x40 = self[0x173] · 0x41..0x45 = `movq/punpcklbw/pshufd/pshuflw/pshufd/packuswb/movd` 4바이트 셔플
    //   (실코드 0xcafb8a~0xcafbad 해독: self+0x170 의 8바이트 b0..b7 → [b0, b4, b6, b7]) · 0x45 = self[0x178]
    o.push(0x40, 1, rd_u8(sf + 0x173) as u64);
    o.push(0x41, 1, rd_u8(sf + 0x170) as u64);
    o.push(0x42, 1, rd_u8(sf + 0x174) as u64);
    o.push(0x43, 1, rd_u8(sf + 0x176) as u64);
    o.push(0x44, 1, rd_u8(sf + 0x177) as u64);
    o.push(0x45, 1, rd_u8(sf + 0x178) as u64);
    Some(o)
}

/// `LineGankerPlan::target_bush_v41`(m08.ll, ganker.rs L311~349) — 국면(phase)·팀·역할로 **부시 인덱스**.
///   호출 규약(실코드 0xcafd72): `f(self[0x30], player.side, player.role, x, ctx)`. ctx 의 cfg = `ctx[8]`.
unsafe fn target_bush_v41(phase: u8, side: u64, role: u32, x: usize, ctx: usize) -> Option<u64> {
    if side >= 2 || role >= 5 { return None; }           // 게임은 bounds panic
    let s = side as usize;
    let pick = |t: [u64; 7], off: usize| -> Option<u64> {
        let k = rd_u64(x + off + s * 8)?; if k >= 7 { return None; }   // 게임은 panic
        Some(t[k as usize])
    };
    match phase {
        0 => pick(if side == 0 { [16, 6, 3, 3, 3, 2, 2] } else { [2, 3, 6, 6, 6, 16, 16] }, 8640),
        2 => pick(if side == 0 { [21, 20, 15, 15, 15, 9, 7] } else { [9, 15, 20, 20, 20, 21, 23] }, 8672),
        1 => {
            let me = rd_u64(x + X_ROSTER + s * 0x28 + role as usize * 8)? as usize;
            if me == 0 { return None; }                  // 게임은 unwrap panic
            let cfg = rd_u64(ctx + 8)? as usize; if !ptr_ok(cfg) { return None; }
            // flip = (cfg[4800] - me.y) < me.x  — 맵 대각 기준 위/아래
            let flip = rd_u64(cfg + 4800)?.wrapping_sub(rd_u64(me + ENT_Y)?) < rd_u64(me + ENT_X)?;
            let (a, b, c) = (if flip { 14 } else { 11 }, if flip { 21 } else { 17 }, if flip { 9 } else { 4 });
            let (t0, t5) = if side == 0 { (b, c) } else { (c, b) };
            pick([t0, a, a, a, a, t5, t5], 8656)
        }
        _ => None,                                       // 게임은 unreachable
    }
}

/// arm 8 — LineGanker. 공통 꼬리 = `code 9 · [8]=ret · word[0x10]=1 · [0x12]=0`(실코드 0xcafd95~0xcafda3).
unsafe fn arm_line_ganker(sf: usize, player: usize, data: usize) -> Option<MpOut> {
    let side = rd_u64(player + P5_SIDE)?; let role = rd_u32(player + P5_ROLE);
    let x = rd_u64(data)? as usize; let ctx = rd_u64(data + 8)? as usize;
    if !ptr_ok(x) || !ptr_ok(ctx) { return None; }
    let r = target_bush_v41(rd_u8(sf + 0x30), side, role, x, ctx)?;
    let mut o = MpOut::default();
    o.push(8, 8, r); o.push(0x10, 2, 1); o.push(0x12, 1, 0); o.code(9);
    Some(o)
}

/// ★`BigPlan::sub_plan` 본체. 각 arm 은 **자기 플랜의 `*Plan::sub_plan`**(= 이미 포팅·검증된 핸들러)에 위임한다.
///   위임 시 `p2` 는 게임과 같이 **`self + 8`**(플랜 payload) 로 바꿔 넘긴다(IR: `%19 = gep %1, i64 8`).
pub unsafe fn big_plan_sub_plan(a: &Args8) -> Option<MpOut> {
    let sf = a.p2; let player = a.p5; let data = a.p6;
    if !ptr_ok(sf) { return None; }
    let v = rd_u64(sf)?;
    let idx = if v > 1 { v.wrapping_sub(2) } else { 4 };
    crate::judge::tr(11, 0x5000_0000 | (v << 8) | (idx & 0xff));   // ★DIFF 로그에 variant/arm 을 남긴다
    // ★위임 인자 매핑(실코드 0xcafcec~0xcafd07 등 전 arm 공통):
    //   callee(out, self+8, version, p4, **player**, **data**, **caller p8**, **caller p10**)
    //   = IR `PlanX::sub_plan(sret %0, %19, %2, %3, %4, %5, %7, %9)`. p7/p8 을 그대로 넘기면 어긋난다.
    let (_, p10, _, _) = crate::judge::HOOK_EXTRA.with(|c| c.get());
    // arm 마다 **넘기는 인자가 다르다**(실코드 호출부 전수, 슬롯 0x20=5번째 … 0x40=9번째):
    //   0xccc010/0xccc3c0 : 5=p5 6=p6 7=p7
    //   0xdf0e90/0xdefcd0 : 5=p5 6=p6 7=p7 8=p8 9=p10
    //   0xd2c5d0/0xd781e0 : 5=p5 6=p6 7=p8 8=p10      ← p7 을 건너뛴다
    //   0xd2da10          : 5=p5 6=p6 7=p7 8=p10
    //   0xdfdfc0/0xd2e500 : 5=p5 6=p6 7=p10
    let base = Args8 { p2: sf + 8, ..*a };
    let inner_skip7 = Args8 { p7: a.p8, p8: p10, ..base };     // passive_line · single_line
    let inner_p10_7 = Args8 { p7: p10, ..base };               // battle · passive_jungle
    let inner_dn    = Args8 { p8: p10, ..base };               // defense_nexus
    let inner       = base;                                    // epic/serpen poke(9인자) · 그 외
    match idx {
        0 | 6 => { let mut o = MpOut::default(); o.code(5); Some(o) }
        3 => arm_support(sf),
        4 => arm_line(sf, v),
        14 => arm_attack_nexus(sf + 8, player, data),
        // ★위임 arm 이 **자기 훅에서 live(shadow) 치환 중**이면 out 에는 노브 적용판이 들어 있다.
        //   그때는 재현도 `_live` 판을 써야 대조가 성립한다(안 그러면 knob_eff 만큼 가짜 DIFF).
        1 => if lv("judge_live_passive_line") { super::passive_line::passive_line_live(&inner_skip7) }
             else { super::passive_line::passive_line(&inner_skip7) },
        5 => if lv("judge_live_passive_jungle") { super::passive_jungle::passive_jungle_live(&inner_p10_7) }
             else { super::passive_jungle::passive_jungle(&inner_p10_7) },
        7 => if lv("judge_live_battle") { super::battle::battle_live(&inner_p10_7) }
             else { super::battle::battle(&inner_p10_7) },
        10 => super::hunt_poke::epic_hunt_poke(&inner),
        12 => super::hunt_poke::serpen_hunt_poke(&inner),
        15 => if lv("judge_live_defense_nexus") { super::defense_nexus::defense_nexus_live(&inner_dn) }
              else { super::defense_nexus::defense_nexus(&inner_dn) },
        2 => { let _ = na_tag("SP_single_line"); None }        // 0.5.8 미발화(entered=0)
        8 => arm_line_ganker(sf, player, data),
        9 => { let _ = na_tag("SP_gank_cover"); None }
        11 => { let _ = na_tag("SP_epic_battle"); None }
        13 => { let _ = na_tag("SP_serpen_battle"); None }
        _ => { let _ = na_tag("SP_unreachable"); None }        // 게임은 unreachable
    }
}

/// 진단 — arm 번호와 태그를 남긴다.
pub unsafe fn sp_diag(a: &Args8) -> String {
    let f = || -> Option<String> {
        let v = rd_u64(a.p2)?;
        let idx = if v > 1 { v.wrapping_sub(2) } else { 4 };
        let side = if ptr_ok(a.p5) { rd_u64(a.p5 + P5_SIDE)? } else { 9 };
        let mine = big_plan_sub_plan(a);
        Some(format!("variant={} arm={} side={} mine_code={:?} n={:?}", v, idx, side,
                     mine.as_ref().and_then(|m| m.get_code()), mine.as_ref().map(|m| m.n)))
    };
    f().unwrap_or_else(|| "sp diag NA".into())
}
