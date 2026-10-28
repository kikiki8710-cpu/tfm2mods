//! 라인·콜 추가전술의 원래 register snapshot만 읽는 계산 콜백.
//! 0.7.0 입력: lane_range_a=rdx(창 0x1063d91) · lane_range_b=r15(0x107d52e) · lane_range_c=rdi(0xfd2c0b, 불변) · gank_recheck=r15 · gank_deadline=[rbp+0xc8].
use crate::micro::{Frame,Src};
fn valid_tps(t:u64)->Option<u64>{if t>0 && t<=100000 {Some(t)} else {None}}
fn half_tps(t:u64)->Option<u64>{if t&1!=0 {None}else{valid_tps(t/2)}}
fn tps_mul(t:u64,v:i64)->Option<u64>{Some(valid_tps(t)?.wrapping_mul(u64::try_from(v).ok()?))}
pub(crate) fn gank_deadline(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.scalar(Src::Mem(5,0xc8))?,v)} // 0.7.0 TPS=[rbp+0xc8] (0.6.5 0xb8)
pub(crate) fn call_early(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.reg(1)?,v)}
pub(crate) fn call_diagnostic(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.reg(1)?,v)}
pub(crate) fn call_general(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.reg(0)?,v)}
pub(crate) fn same_lane_retry(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(half_tps(f.reg(0)?)?,v)}
pub(crate) fn pregank_hp(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some((35u64).wrapping_mul(v as u64)/100)}
pub(crate) fn pregank_radius(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}{let r=200000u64.wrapping_mul(v as u64)/100;Some(r.wrapping_mul(r))}}
pub(crate) fn defense_risk(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(35u64.wrapping_mul(v as u64)/100)}
pub(crate) fn defense_long(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(25u64.wrapping_mul(v as u64)/100)}
pub(crate) fn defense_short(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(10u64.wrapping_mul(v as u64)/100)}
pub(crate) fn gank_recheck(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.reg(15)?,v)} // 0.7.0 TPS=r15 (0.6.5 r14)
pub(crate) fn roam_extra(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.reg(1)?,v)}
pub(crate) fn lane_range_a(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(f.reg(2)?.wrapping_mul(v as u64+2))}
pub(crate) fn lane_range_b(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(f.reg(15)?.wrapping_mul(v as u64+2))}
pub(crate) fn lane_range_c(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(f.reg(7)?.wrapping_mul(v as u64+2))}
pub(crate) fn superiority_hp(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(20+5*v as u64)}
pub(crate) fn superiority_count(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(1+v as u64)}
pub(crate) fn lane_call_cap(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.scalar(Src::Mem2(7,8,0x12f8))?,v)}
pub(crate) fn lane_call_deadline_half(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}{let t=f.reg(0)?;if v%2!=0{return None;}tps_mul(t,v/2)}}
pub(crate) fn mia_global(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}{let t=valid_tps(f.reg(1)?)?;Some(t.wrapping_mul(v as u64)/100)}}
pub(crate) fn mia_memory(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}{let t=valid_tps(f.reg(1)?)?;Some(f.reg(2)?.wrapping_add(t.wrapping_mul(4).wrapping_mul(v as u64)/100))}}
pub(crate) fn mia_hp(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(60u64.wrapping_mul(v as u64)/100)}
pub(crate) fn mia_radius(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}{if v==0{return None;}let r=170000u64.wrapping_mul(100)/(v as u64);Some((r.wrapping_mul(r)>>8).wrapping_sub(1))}}
pub(crate) fn relay_limit(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}{let t=valid_tps(f.scalar(Src::Mem(9,0x12f8))?)?;Some(t.wrapping_mul(v as u64)/100)}}
/// 0.7.0 전술 로밍(Player+0x2c5) 경로 자기 체력 검사 — 창 0xe3e78c `test r14,r14 ; je 거부`의 첫 명령을 `test r11,r11` 로 바꿔 쓴다.
/// r14 = 자기 엔티티. 반환 0 = 거부(je 성립), 그 밖 = r14 그대로(= 원래 test 결과와 같음).
/// HP% = [e+0x668]*100 / max([e+0x620],1) — 고전 창 0xe42e62~0xe42f1a 와 같은 부호 없는 식, 비교도 같은 `HP% < 값 → 거부`.
/// 원본 경로엔 이 검사가 없으므로 기본 단계(값 미설정)에서는 dispatch 가 원본 창으로 보내 바닐라와 같다.
pub(crate) fn tactical_roam_self_hp(f:&Frame,v:i64)->Option<u64>{
    if !(0..=127).contains(&v){return None;}
    let e=f.reg(14)?; if e==0 {return Some(0);}
    let hp=f.scalar(Src::Mem(14,0x668))?; let max=f.scalar(Src::Mem(14,0x620))?.max(1);
    Some(if hp.wrapping_mul(100)/max<v as u64 {0} else {e})
}
