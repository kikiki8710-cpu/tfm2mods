//! 운영 추가 전술: 프레임의 원본 입력만 읽는 값 변환. 게임 함수 호출 없음.
use crate::micro::{Frame, Src};

fn valid(v:i64, lo:i64, hi:i64)->Option<u64> { if (lo..=hi).contains(&v) {Some(v as u64)} else {None} }
fn scale(n:u64, v:u64)->u64 {((n as u128*v as u128)/100) as u64}

pub(crate) fn collapse_metric(_: &Frame,v:i64)->Option<u64> {
    let p=valid(v,50,150)?;
    Some((-(3000i64*100/p as i64)) as u64)
}
fn stage(v:i64, last:bool)->Option<u64> {
    valid(v,50,150)?;
    let n=if v<100 {-3i64} else if v>100 {-1} else {-2};
    Some((n-i64::from(last)) as u64)
}
pub(crate) fn collapse_stage(_: &Frame,v:i64)->Option<u64> {stage(v,false)}
pub(crate) fn collapse_last_stage(_: &Frame,v:i64)->Option<u64> {stage(v,true)}
pub(crate) fn radius_squared(_: &Frame,v:i64)->Option<u64> {let n=valid(v,1000,2_000_000)?;Some(n*n)}
pub(crate) fn pressure_stage(_: &Frame,v:i64)->Option<u64> {Some(scale(3,valid(v,50,150)?).max(1))}
pub(crate) fn pressure_metric(_: &Frame,v:i64)->Option<u64> {Some(scale(2000,valid(v,50,150)?))}
pub(crate) fn pressure_count(_: &Frame,v:i64)->Option<u64> {Some(scale(2,valid(v,50,150)?).max(1))}
pub(crate) fn defense_threat_hp(_: &Frame,v:i64)->Option<u64> {Some(valid(v,11,100)?-10)}
pub(crate) fn structure_rcx(f:&Frame,v:i64)->Option<u64> {Some(f.reg(1)?.wrapping_mul(valid(v,0,100)?))}
pub(crate) fn structure_rdx(f:&Frame,v:i64)->Option<u64> {Some(f.reg(2)?.wrapping_mul(valid(v,0,100)?))}
pub(crate) fn respawn_seconds(f:&Frame,v:i64)->Option<u64> {
    let seconds=valid(v,1,120)?;
    let tps=f.scalar(Src::Mem2(15,8,0x12f8))?; // 0.7.0 r15=[rsi+8] (0.6.5 r13)
    Some(tps.wrapping_mul(seconds))
}
pub(crate) fn damage_increment(f:&Frame,v:i64)->Option<u64> {
    Some(f.reg(0)?.wrapping_mul(valid(v,1,10)?-1))
}
pub(crate) fn finish_seconds3(f:&Frame,v:i64)->Option<u64> {Some(f.reg(15)?.wrapping_mul(valid(v,1,30)?))} // 0.7.1 TPS=r15 (0.7.0 r14 = 이제 컨텍스트 포인터)
pub(crate) fn finish_increment2(f:&Frame,v:i64)->Option<u64> {
    // 원래 ADD의 CF/OF까지 유지하도록 전체 결과가 아니라 덧셈 항을 반환한다.
    let n=valid(v,1,30)?;
    Some(f.reg(15)?.wrapping_mul(n.wrapping_sub(2)))
}
pub(crate) fn camp_survival_margin(f:&Frame,v:i64)->Option<u64> {Some(scale(f.scalar(Src::Mem(5,-0x10))?,valid(v,0,300)?))} // 0.7.2 TPS=[rbp-0x10] (0.7.1 [rbp-0x20] · 0.7.0 [rbp-0x18])
pub(crate) fn camp_arrival_margin(f:&Frame,v:i64)->Option<u64> {Some(scale(f.reg(1)?>>1,valid(v,0,300)?))}
fn tower_radius(v:i64, base:u64)->Option<u64> {Some(scale(base,valid(v,50,150)?))}
pub(crate) fn tower_radius_high(f:&Frame,v:i64)->Option<u64> {let _=f;let r=tower_radius(v,140_000)?;Some(r*r+1)}
pub(crate) fn tower_radius_low(f:&Frame,v:i64)->Option<u64> {let _=f;let r=tower_radius(v,100_000)?;Some(r*r+1)}
pub(crate) fn tower_minion_gate(_: &Frame,v:i64)->Option<u64> {valid(v,50,150)?;Some(if v<100 {2} else if v>100 {0} else {1})}
pub(crate) fn tower_wave_limit(_: &Frame,v:i64)->Option<u64> {Some(300_000/valid(v,50,150)?)}
// ── 0.7.2 신설 전술(E001~E003 · 유저 승인 2026-11-11 · 근거 = REPORT RE 2026-11-11_0.7.2-새전술4_창RE)
// E001 넥서스 마무리 시작 거리: 헬퍼 0xea1800 「적 넥서스/남은 구조물이 R 안」 · 원본 상수 250001² = (R+1)² 꼴(R=250000).
fn finish_reach(v:i64)->Option<u64> {let r=scale(250_000,valid(v,25,400)?)+1;Some(r*r)}
pub(crate) fn finish_reach_lt(_: &Frame,v:i64)->Option<u64> {finish_reach(v)}            // 0xea1897 거리² < (R+1)²
pub(crate) fn finish_reach_le(_: &Frame,v:i64)->Option<u64> {Some(finish_reach(v)?-1)}  // 0xea18c1 거리² ≤ (R+1)²−1
// E002 전력 부족 시 마무리 신중도: 0x101b430 계수 = 100 + min(할증,95) → 할증에 v% 를 곱한다(v=0 이면 0.7.1 식 a+m<b).
pub(crate) fn finish_caution_alone(_: &Frame,v:i64)->Option<u64> {Some(scale(95,valid(v,0,130)?)+100)}  // 0x101b53f 아군 없음 계수 195
pub(crate) fn finish_caution_scale(f:&Frame,v:i64)->Option<u64> {let c=f.reg(1)?&0xffff_ffff;Some(scale(c,valid(v,0,130)?)&0xffff_ffff)} // 0x101b575 ecx=min(할증,95) — 동적 조각이 뒤에 원래 add ecx,100
// E003 적 접근 시 교전 요청 거리: engage 0xe65690 체력>59 갈래 · 적↔나 R(250000) · 에픽·세르펜 목표면 적↔지점 230000 도 같은 비율.
pub(crate) fn approach_call_objective(_: &Frame,v:i64)->Option<u64> {let r=scale(230_000,valid(v,25,400)?);Some(r*r)}   // 0xe6a264 r13
pub(crate) fn approach_call_enemy(_: &Frame,v:i64)->Option<u64> {let r=scale(250_000,valid(v,25,400)?);Some(r*r)}       // 적↔나 ≤ R² ×5
pub(crate) fn approach_call_enemy_free(_: &Frame,v:i64)->Option<u64> {let r=scale(250_000,valid(v,25,400)?);Some(r*r+1)} // 오브젝트 없음 적↔나 < R²+1 ×5

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn signed_profiles_and_inverse_boundary() {
        assert_eq!(stage(100,false),Some((-2i64) as u64));
        assert_eq!(stage(100,true),Some((-3i64) as u64));
        for v in [50,75,100,125,150] { assert_eq!(stage(v,true).unwrap(),stage(v,false).unwrap().wrapping_sub(1)); }
    }
    #[test] fn exact_default_and_fractional_scale() {
        for n in [0,1,29,30,u32::MAX as u64,u64::MAX] {assert_eq!(scale(n,100),n);}
        assert_eq!(scale(3,50),1);assert_eq!(scale(3,150),4);
        assert_eq!(tower_radius(100,140_000),Some(140_000));
    }
}
