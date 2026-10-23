//! 라인·콜 추가전술의 원래 register snapshot만 읽는 계산 콜백.
use crate::micro::{Frame,Src};
fn valid_tps(t:u64)->Option<u64>{if t>0 && t<=100000 {Some(t)} else {None}}
fn half_tps(t:u64)->Option<u64>{if t&1!=0 {None}else{valid_tps(t/2)}}
fn tps_mul(t:u64,v:i64)->Option<u64>{Some(valid_tps(t)?.wrapping_mul(u64::try_from(v).ok()?))}
pub(crate) fn gank_deadline(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.scalar(Src::Mem(5,0xb8))?,v)}
pub(crate) fn call_early(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.reg(1)?,v)}
pub(crate) fn call_diagnostic(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.reg(1)?,v)}
pub(crate) fn call_general(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.reg(0)?,v)}
pub(crate) fn same_lane_retry(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(half_tps(f.reg(0)?)?,v)}
pub(crate) fn pregank_hp(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some((35u64).wrapping_mul(v as u64)/100)}
pub(crate) fn pregank_radius(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}{let r=200000u64.wrapping_mul(v as u64)/100;Some(r.wrapping_mul(r))}}
pub(crate) fn defense_risk(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(35u64.wrapping_mul(v as u64)/100)}
pub(crate) fn defense_long(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(25u64.wrapping_mul(v as u64)/100)}
pub(crate) fn defense_short(_f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(10u64.wrapping_mul(v as u64)/100)}
pub(crate) fn gank_recheck(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.reg(14)?,v)}
pub(crate) fn roam_extra(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}tps_mul(f.reg(1)?,v)}
pub(crate) fn lane_range_a(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(f.reg(7)?.wrapping_mul(v as u64+2))}
pub(crate) fn lane_range_b(f:&Frame,v:i64)->Option<u64>{if !(0..=1000000).contains(&v){return None;}Some(f.reg(12)?.wrapping_mul(v as u64+2))}
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
