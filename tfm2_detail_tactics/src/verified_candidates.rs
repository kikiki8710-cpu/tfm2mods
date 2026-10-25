//! 다시보기에서 변화가 확인된 후보의 읽기 전용 변환. 수치·주체 검사를 보존한다.

use crate::micro::{Frame,Src};

pub fn negative(_: &Frame,v:i64)->Option<u64>{Some((v.checked_neg()?) as u64)}

pub fn square_shift8(_: &Frame,v:i64)->Option<u64>{let r=u64::try_from(v).ok()?;Some(r.checked_mul(r)?>>8)}

fn ops_valid(v:i64,lo:i64,hi:i64)->Option<u64>{if (lo..=hi).contains(&v){Some(v as u64)}else{None}}

pub(crate) fn ops_radius_squared(_: &Frame,v:i64)->Option<u64>{
    let r=ops_valid(v,1000,2_000_000)?;Some(r*r)
}

pub(crate) fn ops_camp_tps(f:&Frame,v:i64)->Option<u64>{
    Some(f.scalar(Src::Mem(1,0x12f8))?.wrapping_mul(ops_valid(v,0,10)?))
}

pub(crate) fn ops_gather_square(f:&Frame,v:i64)->Option<u64>{
    let r=ops_valid(v,1000,2_000_000)?;
    // dispatch의 known-return/내 계약 선수 검사에 더해 현재 Cache 귀속을 확인한다.
    let player=f.scalar(Src::Stack(0x30))?;
    let team=f.scalar(Src::StackMem(0x30,0x820))?;
    let pos=f.scalar(Src::StackMem(0x30,0x8b0))? & 0xffff_ffff;
    if team>1 || pos>4 || f.reg(9)?!=team*5{return None;}
    let slot=(team*0x28+pos*8) as i32;
    if f.scalar(Src::Mem(2,0x288+slot))?!=player{return None;}
    let champ=f.scalar(Src::Mem(2,0x238+slot))?;
    if champ<0x10000 || champ>=1u64<<48{return None;}
    // 슬롯 재확인은 관측 도중 roster가 바뀐 표본을 원본으로 돌린다.
    if f.scalar(Src::Mem(2,0x288+slot))?!=player || f.scalar(Src::Mem(2,0x238+slot))?!=champ{return None;}
    Some(r*r)
}

pub(crate) fn combat_rsi_add(f: &crate::micro::Frame, v: i64) -> Option<u64> {
    if !(0..=1000).contains(&v) { return None; }
    Some(f.reg(6)?.wrapping_add(v as u64))
}

pub(crate) fn combat_negative(_f: &crate::micro::Frame, v: i64) -> Option<u64> {
    if !(0..=1000).contains(&v) { return None; }
    Some((v as u64).wrapping_neg())
}
