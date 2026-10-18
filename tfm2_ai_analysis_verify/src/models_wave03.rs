//! wave03 조건부 순수 모델 3개. 게임 helper/원본/TLS를 호출하지 않는다.
//! panic 도메인과 읽기 실패는 Err이며 예상값0으로 숨기지 않는다.
use crate::engine::{Evaluation,Memory,Metric};

pub const FIRST_MODEL_ID:usize=16;
pub const MODEL_COUNT:usize=3;
// FE8F92의FILE-backed RVA3B6D318 3QWORD. 고정0.6.4 PE에서[10,90,D0]확인.
const LINE_WORLD_OFFSETS:[u64;3]=[0x10,0x90,0xd0];

fn read(m:&mut dyn Memory,base:u64,offset:u64,width:u8)->Result<u64,&'static str>{
    let value=m.read(base.wrapping_add(offset),width)?;
    match width{1=>Ok(value&0xff),8=>Ok(value),_=>Err("WAVE03_INVALID_WIDTH")}
}
fn metric(name:&'static str,lhs:i128,rhs:i128,relation:&'static str,predicate:bool)->Metric{
    Metric{name,lhs,rhs,relation,predicate,observed_branch:false}
}
fn output(expected:u64,mask:u64,metrics:Vec<Metric>)->Evaluation{
    Evaluation{expected,mask,output_ptr:None,output_fields:Vec::new(),metrics}
}
fn quotient(numerator:u64,divisor:u64)->Result<i64,&'static str>{
    let n=numerator as i64;let d=divisor as i64;
    if d==0{return Err("WAVE03_DIVISOR_ZERO_GAME_PANIC_DOMAIN");}
    if n==i64::MIN&&d == -1{return Err("WAVE03_SIGNED_DIVISION_OVERFLOW_GAME_PANIC_DOMAIN");}
    // 두피연산자모두상위32=0인DIV32도동일signed양수몫이다. 나머지IDIV64는0방향절삭.
    Ok(n/d)
}

/// 인수는Win64 RCX/RDX/R8/R9원비트다. 미지원id는Err로남긴다.
pub fn evaluate(id:usize,a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    match id{
        16=>optional_scaled_cap(a,m),
        17=>scaled_delta_positive(a,m),
        18=>line_state_or_opponent_structure(a,m),
        _=>Err("UNSUPPORTED_WAVE03_MODEL_ID")
    }
}

// FD19F0: 태그는fullQWORD==1. tag!=1은capture미소비/fallback그대로반환.
fn optional_scaled_cap(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let active=a[0]==1;
    let mut metrics=vec![metric("optional_scale_tag_exact1",a[0]as i128,1,"==",active)];
    if !active{return Ok(output(a[2],u64::MAX,metrics));}
    let divisor_ref=read(m,a[3],8,8)?;
    let divisor=read(m,divisor_ref,0,8)?;
    if divisor==0{return Err("WAVE03_DIVISOR_ZERO_GAME_PANIC_DOMAIN");}
    let factor_ref=read(m,a[3],0,8)?;
    let factor=read(m,factor_ref,0,8)?;
    let n=a[1].wrapping_mul(factor);
    let q=quotient(n,divisor)?;
    let cap_ref=read(m,a[3],0x10,8)?;
    let cap=read(m,cap_ref,0,8)?as i64;
    metrics.push(metric("optional_scale_signed_cap",q as i128,cap as i128,"<=",q<=cap));
    Ok(output(q.min(cap)as u64,u64::MAX,metrics))
}

// FD37F0: quotient뒤SUB는wrap64. TEST결과의signed양수부이므로무한정밀차와다르다.
fn scaled_delta_positive(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let divisor_ref=read(m,a[0],0x10,8)?;
    let divisor=read(m,divisor_ref,0,8)?;
    if divisor==0{return Err("WAVE03_DIVISOR_ZERO_GAME_PANIC_DOMAIN");}
    let object=read(m,a[0],0,8)?;
    let factor_ref=read(m,a[0],8,8)?;
    let factor=read(m,factor_ref,0,8)?;
    let n=factor.wrapping_mul(read(m,object,0x98,8)?);
    let q=quotient(n,divisor)?;
    let subtract_ref=read(m,a[0],0x18,8)?;
    let subtract=read(m,subtract_ref,0,8)?;
    let delta=(q as u64).wrapping_sub(subtract)as i64;
    let positive=delta>0;
    Ok(output(if positive{delta as u64}else{0},u64::MAX,
        vec![metric("scaled_delta_positive_after_wrap",delta as i128,0,">",positive)]))
}

// FE8F30: line>=2의레코드선택과static표의원래lowBYTE인덱스는같은clamp가아니다.
// 정상line0..2만모델로지원한다. 그밖에는먼저Err로제외하며원본분기를모방했다고하지않는다.
fn line_state_or_opponent_structure(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    if a[1]>1{return Err("FE8F30_TEAM_BOUNDS_GAME_PANIC_DOMAIN");}
    let line=(a[2]&0xff)as usize;
    if line>2{return Err("FE8F30_LINE_CAPTURE_DOMAIN_ONLY_0_TO_2");}
    let blackboard=read(m,a[0],0x10,8)?;
    let record=blackboard.wrapping_add(a[1]*0xac8).wrapping_add((line as u64)*0x28);
    let ready=read(m,record,0x26,1)?;
    let mut metrics=vec![metric("lane_record26_nonzero",ready as i128,0,"!=",ready!=0)];
    if ready!=0{return Ok(output(1,0xff,metrics));}
    let pressure=read(m,record,0x27,1)?as u8 as i8;
    metrics.push(metric("lane_record27_signed_positive",pressure as i128,0,">",pressure>0));
    if pressure>0{return Ok(output(0,0xff,metrics));}
    let blocked=read(m,record,0x25,1)?;
    metrics.push(metric("lane_record25_nonzero",blocked as i128,0,"!=",blocked!=0));
    if blocked!=0{return Ok(output(0,0xff,metrics));}
    let world=read(m,a[0],0,8)?;
    let offset=LINE_WORLD_OFFSETS[line]+((1-a[1])<<5)+0x18;
    let structure=read(m,world,offset,8)?;
    metrics.push(metric("lane_opponent_structure_present",structure as i128,0,"!=",structure!=0));
    Ok(output(u64::from(structure!=0),0xff,metrics))
}

#[cfg(test)]mod tests{
    use super::*;
    use std::collections::BTreeMap;
    #[derive(Default)]struct Fixture{fields:BTreeMap<(u64,u8),u64>,reads:Vec<(u64,u8)>}
    impl Fixture{
        fn p(&mut self,a:u64,w:u8,v:u64){self.fields.insert((a,w),v);}
        fn q(&mut self,a:u64,v:u64){self.p(a,8,v);}
    }
    impl Memory for Fixture{
        fn read(&mut self,a:u64,w:u8)->Result<u64,&'static str>{self.reads.push((a,w));self.fields.get(&(a,w)).copied().ok_or("WAVE03_FIXTURE_MISSING_READ")}
    }
    fn value(id:usize,a:[u64;4],m:&mut Fixture)->u64{
        let e=evaluate(id,a,m).unwrap();assert!(e.metrics.iter().all(|x|!x.observed_branch));assert!(e.output_fields.is_empty());e.expected
    }
    fn scale(divisor:u64,cap:u64)->Fixture{
        let mut f=Fixture::default();f.q(0x1000,0x2000);f.q(0x1008,0x3000);f.q(0x1010,0x4000);
        f.q(0x2000,1);f.q(0x3000,divisor);f.q(0x4000,cap);f
    }
    #[test]fn optional_tag_is_full64_and_inactive_has_no_reads(){
        for tag in [0,2,0x1_0000_0001,u64::MAX]{let mut f=Fixture::default();assert_eq!(value(16,[tag,0,0xabcdef01_23456789,0],&mut f),0xabcdef01_23456789);assert!(f.reads.is_empty());}
    }
    #[test]fn optional_cap_minus_equal_plus_and_signed_truncation(){
        for (input,want)in [(39,39),(40,40),(41,40)]{let mut f=scale(1,40);let e=evaluate(16,[1,input,0,0x1000],&mut f).unwrap();assert_eq!((e.expected,e.mask),(want,u64::MAX));assert_eq!(f.reads.len(),6);}
        let mut f=scale(3,40);assert_eq!(value(16,[1,(-8i64)as u64,0,0x1000],&mut f),(-2i64)as u64);
        let mut f=scale((-3i64)as u64,40);assert_eq!(value(16,[1,8,0,0x1000],&mut f),(-2i64)as u64);
        let mut f=scale(1,(-3i64)as u64);assert_eq!(value(16,[1,8,0,0x1000],&mut f),(-3i64)as u64);
    }
    #[test]fn optional_wrap_div32_div64_and_panic_domains(){
        let mut f=scale(2,u64::MAX>>1);f.q(0x2000,2);assert_eq!(value(16,[1,u64::MAX,0,0x1000],&mut f),u64::MAX);
        let mut f=scale(u32::MAX as u64,i64::MAX as u64);assert_eq!(value(16,[1,u32::MAX as u64,0,0x1000],&mut f),1);
        let mut f=scale(1<<40,i64::MAX as u64);assert_eq!(value(16,[1,1<<41,0,0x1000],&mut f),2);
        let mut f=scale(0,40);assert!(evaluate(16,[1,1,0,0x1000],&mut f).is_err());assert!(!f.reads.contains(&(0x1000,8)));
        let mut f=scale(u64::MAX,40);assert!(evaluate(16,[1,i64::MIN as u64,0,0x1000],&mut f).is_err());assert!(!f.reads.contains(&(0x1010,8)));
        assert!(evaluate(16,[1,1,0,0x1000],&mut Fixture::default()).is_err());
    }
    fn delta(input:u64,factor:u64,divisor:u64,subtract:u64)->Fixture{
        let mut f=Fixture::default();f.q(0x1000,0x2000);f.q(0x1008,0x3000);f.q(0x1010,0x4000);f.q(0x1018,0x5000);
        f.q(0x2098,input);f.q(0x3000,factor);f.q(0x4000,divisor);f.q(0x5000,subtract);f
    }
    #[test]fn delta_zero_minus_equal_plus_and_wrap(){
        for (input,want)in [(39,0),(40,0),(41,1)]{let mut f=delta(input,1,1,40);assert_eq!(value(17,[0x1000,0,0,0],&mut f),want);assert_eq!(f.reads.len(),8);}
        let mut f=delta(i64::MAX as u64,1,1,u64::MAX);assert_eq!(value(17,[0x1000,0,0,0],&mut f),0);
        let mut f=delta(i64::MIN as u64,1,1,1);assert_eq!(value(17,[0x1000,0,0,0],&mut f),i64::MAX as u64);
    }
    #[test]fn delta_division_width_signed_and_panic_domains(){
        let mut f=delta(u32::MAX as u64,1,u32::MAX as u64,0);assert_eq!(value(17,[0x1000,0,0,0],&mut f),1);
        let mut f=delta(1<<41,1,1<<40,0);assert_eq!(value(17,[0x1000,0,0,0],&mut f),2);
        let mut f=delta((-8i64)as u64,1,3,(-4i64)as u64);assert_eq!(value(17,[0x1000,0,0,0],&mut f),2);
        let mut f=delta(1,1,0,0);assert!(evaluate(17,[0x1000,0,0,0],&mut f).is_err());assert!(!f.reads.contains(&(0x1000,8)));
        let mut f=delta(i64::MIN as u64,1,u64::MAX,0);assert!(evaluate(17,[0x1000,0,0,0],&mut f).is_err());assert!(!f.reads.contains(&(0x1018,8)));
        assert!(evaluate(17,[0x1000,0,0,0],&mut Fixture::default()).is_err());
    }
    fn lane(team:u64,line:u64,pressure:u64,structure:u64)->Fixture{
        let mut f=Fixture::default();f.q(0x1010,0x2000);f.q(0x1000,0x5000);
        let record=0x2000+team*0xac8+line*0x28;f.p(record+0x26,1,0);f.p(record+0x27,1,pressure);f.p(record+0x25,1,0);
        f.q(0x5000+LINE_WORLD_OFFSETS[line as usize]+((1-team)<<5)+0x18,structure);f
    }
    #[test]fn lane_all_teams_lines_and_low_byte_alias(){
        for team in 0..2{for line in 0..3{for structure in [0,0x100]{let mut f=lane(team,line,0,structure);let e=evaluate(18,[0x1000,team,line|0x100,0],&mut f).unwrap();assert_eq!((e.expected,e.mask),(u64::from(structure!=0),0xff));assert_eq!(f.reads.len(),6);}}}
    }
    #[test]fn lane_signed_byte_minus_equal_plus_and_shortcuts(){
        for (pressure,want)in [(255,1),(0,1),(1,0),(128,1),(127,0)]{let mut f=lane(0,0,pressure,1);assert_eq!(value(18,[0x1000,0,0,0],&mut f),want);}
        let mut f=Fixture::default();f.q(0x1010,0x2000);f.p(0x2026,1,1);assert_eq!(value(18,[0x1000,0,0,0],&mut f),1);assert_eq!(f.reads.len(),2);
        let mut f=lane(0,0,0,1);f.p(0x2025,1,2);f.fields.remove(&(0x1000,8));assert_eq!(value(18,[0x1000,0,0,0],&mut f),0);
    }
    #[test]fn lane_capture_domain_and_missing_read_are_errors(){
        let mut f=Fixture::default();assert!(evaluate(18,[0x1000,2,0,0],&mut f).is_err());assert!(f.reads.is_empty());
        assert!(evaluate(18,[0x1000,0,3,0],&mut f).is_err());assert!(f.reads.is_empty());
        assert!(evaluate(18,[0x1000,0,0,0],&mut f).is_err());assert!(evaluate(99,[0;4],&mut f).is_err());
    }
}
