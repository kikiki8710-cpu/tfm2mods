//! 0.6.5 전투 추가 전술의 읽기 전용 산술 변환.
//! 원본 Frame만 읽으며 게임 helper를 호출하거나 게임 메모리를 쓰지 않는다.
//! 기본 단계/원본값/주체 회수 실패는 공용 dispatch에서 원본으로 우회한다.
use crate::micro::Frame;

fn n(v: i64, min: i64, max: i64) -> Option<u64> {
    if v < min || v > max { None } else { Some(v as u64) }
}
pub(crate) fn tower_hits(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(0)?.wrapping_mul(n(v, 1, 10)?))
}
pub(crate) fn chase_ratio_rdx(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(2)?.wrapping_mul(n(v, 1, 20)?))
}
pub(crate) fn chase_ratio_rcx(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(1)?.wrapping_mul(n(v, 1, 20)?))
}
pub(crate) fn radius_square(f: &Frame, v: i64) -> Option<u64> {
    let _ = f;
    let r = n(v, 1_000, 65_535)?;
    Some(r * r)
}
pub(crate) fn radius_square_minus_one(f: &Frame, v: i64) -> Option<u64> {
    Some(radius_square(f, v)? - 1)
}
pub(crate) fn trade_hp_divisor(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(1)? / n(v, 1, 32)?)
}
pub(crate) fn heal_r8(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(1)?.wrapping_add(f.reg(8)?.wrapping_mul(n(v, 0, 10)?)))
}
pub(crate) fn shield_r8(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(8)?.wrapping_mul(n(v, 0, 10)? + 1))
}
pub(crate) fn heal_r14(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(1)?.wrapping_add(f.reg(14)?.wrapping_mul(n(v, 0, 10)?)))
}
pub(crate) fn shield_r14(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(14)?.wrapping_mul(n(v, 0, 10)? + 1))
}
pub(crate) fn buff_arrival_ticks(f: &Frame, v: i64) -> Option<u64> {
    let twice_tps = f.reg(1)?;
    // 원래 입력은 직전 ADD RCX,RCX의 결과다. 홀수는 계약 밖이므로 원본 복귀.
    if twice_tps & 1 != 0 { return None; }
    Some((twice_tps / 2).wrapping_mul(n(v, 1, 30)?))
}
pub(crate) fn isolated_percent(f: &Frame, v: i64) -> Option<u64> {
    // 직전 LEA가 5X를 만든다. 5의 역원으로 wrap64의 X를 정확히 회수한다.
    const INV5: u64 = 0xcccc_cccc_cccc_cccd;
    Some(f.reg(0)?.wrapping_mul(INV5).wrapping_mul(n(v, 0, 1_000)?))
}
pub(crate) fn front_ally_distance_score(f: &Frame, v: i64) -> Option<u64> {
    let k = n(v, 1, 400)?;
    // 원래 unsigned /4000 뒤에 배율을 적용한다. 상한도 같은 배율로 바꾼다.
    Some(((f.reg(0)? / 4_000) as u128 * k as u128 / 100) as u64)
}
pub(crate) fn front_ally_cap(_f: &Frame, v: i64) -> Option<u64> {
    Some(n(v, 1, 400)? * 25 / 100)
}
pub(crate) fn front_ally_total_cap(_f: &Frame, v: i64) -> Option<u64> {
    Some(n(v, 1, 400)? * 60 / 100)
}
pub(crate) fn wave_risk_divisor(f: &Frame, v: i64) -> Option<u64> {
    Some(((f.reg(9)? as i64) / n(v, 1, 32)? as i64) as u64)
}
pub(crate) fn risk_retreat_high(_f: &Frame, v: i64) -> Option<u64> {
    Some(n(v, 1, 50)? * 24 / 10)
}
/// 0.7.0: 창 0x1015310 시점 H 는 rcx (0.6.5 = rax).
pub(crate) fn risk_retreat_pressure(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(1)?.wrapping_mul(n(v, 1, 50)? * 2))
}
/// 0.7.0: 피제수 r14 판(0.6.5 = rdi). 몫은 dyn 이 rax 로 넣는다(창 0x105f9bd).
pub(crate) fn unseen_risk_divisor(f: &Frame, v: i64) -> Option<u64> {
    Some(((f.reg(14)? as i64) / n(v, 1, 32)? as i64) as u64)
}
pub(crate) fn relocate_inner(_f: &Frame, v: i64) -> Option<u64> {
    Some(n(v, 25, 200)? * 8_000 / 100)
}
pub(crate) fn relocate_mid(_f: &Frame, v: i64) -> Option<u64> {
    Some(n(v, 25, 200)? * 30_000 / 100)
}
pub(crate) fn relocate_outer(_f: &Frame, v: i64) -> Option<u64> {
    Some(n(v, 25, 200)? * 70_000 / 100)
}
pub(crate) fn relocate_self_near(_f: &Frame, v: i64) -> Option<u64> {
    Some(n(v, 25, 200)? * 24_000 / 100)
}
pub(crate) fn relocate_self_far(_f: &Frame, v: i64) -> Option<u64> {
    Some(n(v, 25, 200)? * 48_000 / 100)
}
/// 0.7.0: lea rax,[rdx+rcx+0x7530] — 합 레지스터 rcx (0.6.5 = rsi).
pub(crate) fn relocate_mid_result(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(2)?.wrapping_add(f.reg(1)?).wrapping_add(relocate_mid(f, v)?))
}
/// 0.7.0: lea rcx,[rdx+rcx+0x11170] — 합 레지스터 rcx (0.6.5 = rsi).
pub(crate) fn relocate_outer_result(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(2)?.wrapping_add(f.reg(1)?).wrapping_add(relocate_outer(f, v)?))
}
pub(crate) fn candidate_move_budget(f: &Frame, v: i64) -> Option<u64> {
    // 동적 명령은 원래 LEA로 RCX=2M을 만들고 마지막 SUB로 뺀다.
    // 32M을 (K+2)M으로 바꿔 결과와 마지막 SUB의 flags를 보존한다.
    Some(f.reg(0)?.wrapping_mul(n(v, 0, 120)? + 2))
}
/// 0.7.0: 현위치 피해 r14 판(0.6.5 = rdi). dyn 이 r14 에 결과를 넣고 뒤 xor edi/sub rax,r14 를 재실행(창 0x106058e).
pub(crate) fn forward_credit_percent(f: &Frame, v: i64) -> Option<u64> {
    Some((f.reg(14)? as u128 * n(v, 0, 100)? as u128 / 100) as u64)
}
pub(crate) fn move_penalty_negative(_f: &Frame, v: i64) -> Option<u64> {
    Some(n(v, 0, 1_000)?.wrapping_neg())
}
pub(crate) fn move_penalty_result(f: &Frame, v: i64) -> Option<u64> {
    Some(f.reg(13)?.wrapping_sub(n(v, 0, 1_000)?))
}
