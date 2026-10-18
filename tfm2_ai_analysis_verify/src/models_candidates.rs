//! 미배선 기존 함수2개의 예상식. probe id가 아니라 고정 RVA로 scratch를 구별한다.
use crate::engine::{Evaluation,Memory,Metric};
fn span(a:u64,n:u64)->Result<(u64,u64),&'static str>{
    let b=a.checked_add(n).ok_or("ADDRESS_RANGE_OR_OVERFLOW")?;
    if a<0x10000||n==0||b>(1u64<<48){return Err("ADDRESS_RANGE_OR_OVERFLOW");}Ok((a,b))
}
fn overlap(a:(u64,u64),b:(u64,u64))->bool{a.0<b.1&&b.0<a.1}
struct Reader<'a>{m:&'a mut dyn Memory,out:(u64,u64)}
impl Reader<'_>{
    fn read(&mut self,b:u64,o:u64,w:u8)->Result<u64,&'static str>{
        let a=b.wrapping_add(o);let s=span(a,w as u64)?;
        if overlap(s,self.out){return Err("READONLY_OUTPUT_ALIAS");}
        let v=self.m.read(a,w)?;
        match w{1=>Ok(v&255),4=>Ok(v&0xffff_ffff),8=>Ok(v),_=>Err("INVALID_WIDTH")}
    }
}
fn metric(name:&'static str,lhs:i128,rhs:i128,relation:&'static str,predicate:bool)->Metric{
    Metric{name,lhs,rhs,relation,predicate,observed_branch:false}
}
pub fn evaluate(rva:usize,a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    match rva{0xe1b5b0=>effect(a,m),0xd8c680=>merge(a,m),_=>Err("SCRATCH_RVA_NOT_OWNED")}
}
fn effect(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let(out,e,t)=(a[0],a[1],a[2]);let mut r=Reader{m,out:span(out,72)?};
    if r.read(e,0x4b8,4)?==0xffff_ffff{return Err("BASIC_NONE_PANIC");}
    let speed=r.read(e,0x638,8)?;let extra=r.read(e,0x430,8)?;let base=r.read(e,0x498,8)?;
    let per=r.read(e,0x4a0,8)?;let level=r.read(e,0x5c0,8)?;let modifier=r.read(e,0x468,4)? as u32 as i32 as i64;
    let mut metrics=vec![metric("basic_radius_modifier_nonzero",modifier as i128,0,"!=",modifier!=0)];
    let rawradius=r.read(e,0x678,8)?;
    let radius=if modifier==0{rawradius}else{(modifier.wrapping_add(100)as u64).wrapping_mul(rawradius)/100};
    let kind=r.read(e,0x68,8)?;
    if kind>15{return Err("UNGUARDED_KIND_JT_DOMAIN");}
    let idx=if kind<2{6}else{kind-2};
    let offsets=[None,Some(0xb8),Some(0x110),None,Some(0xe8),Some(0xd0),Some(0x1e8),Some(0xe8),Some(0xb0),Some(0xc8),Some(0xf0),Some(0xc8),Some(0xc0),Some(0xb0)];
    let selected=match offsets[idx as usize]{Some(o)=>r.read(e,o,8)?,None=>0};let main=selected.max(t);
    let mut skill1=e.wrapping_add(0x4c0);if r.read(e,0x4f0,4)?==0xffff_ffff{skill1=0;}
    let mut s1=r.read(e,0xb8,8)?.max(t);
    metrics.push(metric("effect_level3_available",level as i128,3,">=",level>=3));
    if level<3{return Err("STATIC_NONE_LOADED_ADDRESS_UNSUPPORTED");}
    let mut skill2=e.wrapping_add(0x4f8);if r.read(e,0x528,4)?==0xffff_ffff{skill2=0;}
    let range=extra.wrapping_add(base).wrapping_add(radius).wrapping_add(level.wrapping_sub(1).wrapping_mul(per));
    let mut s2=r.read(e,0xc0,8)?.max(t);
    metrics.push(metric("effect_time15_branch",idx as i128,13,"==",idx==13));
    if idx!=13{s1=t;s2=t;}
    let basic=e.wrapping_add(0x488);
    Ok(Evaluation{expected:basic,mask:u64::MAX,output_ptr:Some(out),output_fields:vec![(out,8,e),(out+16,8,speed),(out+8,8,basic),(out+24,8,range),(out+32,8,main),(out+40,8,skill1),(out+48,8,s1),(out+56,8,skill2),(out+64,8,s2)],metrics})
}
fn pair(r:&mut Reader<'_>,source:u64,header:u64,rightpos:u64,leftpos:u64,metrics:&mut Vec<Metric>)->Result<(u64,u64,bool),&'static str>{
    let right=r.read(source,rightpos.wrapping_mul(8),8)?;let len=r.read(header,8,8)?;
    if right>=len{return Err("MERGE_RIGHT_BOUNDS_PANIC");}
    let left=r.read(source,leftpos.wrapping_mul(8),8)?;if left>=len{return Err("MERGE_LEFT_BOUNDS_PANIC");}
    let scores=r.read(header,0,8)?;let ls=r.read(scores,left.wrapping_mul(8),8)?;let rs=r.read(scores,right.wrapping_mul(8),8)?;
    let mut better=(ls as i64)<(rs as i64);
    metrics.push(metric("merge_signed_score_precedes",ls as i64 as i128,rs as i64 as i128,"<",better));
    if ls==rs{better=right<left;metrics.push(metric("merge_unsigned_index_tie",right as i128,left as i128,"<",better));}
    Ok((right,left,better))
}
fn merge(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let(source,n,out,header)=(a[0],a[1],a[2],a[3]);
    if !(2..=16).contains(&n){return Err("MERGE_CAPTURE_LEN_2_TO16");}
    let output=span(out,n*8)?;if overlap(output,span(source,n*8)?){return Err("MERGE_INPUT_OUTPUT_ALIAS");}
    let mut r=Reader{m,out:output};let half=n/2;
    let(mut lh,mut rh,mut lt,mut rt)=(0u64,half,half-1,n-1);let mut fields=Vec::new();let mut metrics=Vec::new();
    for i in 0..half{
        let(right,left,rightfirst)=pair(&mut r,source,header,rh,lh,&mut metrics)?;
        fields.push((out+i*8,8,if rightfirst{right}else{left}));
        let(right,left,leftlast)=pair(&mut r,source,header,rt,lt,&mut metrics)?;
        rh+=u64::from(rightfirst);lh+=u64::from(!rightfirst);
        fields.push((out+(n-1-i)*8,8,if leftlast{left}else{right}));
        lt=lt.wrapping_sub(u64::from(leftlast));rt=rt.wrapping_sub(u64::from(!leftlast));
    }
    let lend=lt.wrapping_add(1);let rend=rt.wrapping_add(1);
    if n%2!=0{
        let remaining=if lh<lend{let x=r.read(source,lh*8,8)?;lh+=1;x}else{let x=r.read(source,rh*8,8)?;rh+=1;x};
        fields.push((out+half*8,8,remaining));
    }
    if lh!=lend||rh!=rend{return Err("MERGE_RUN_POINTERS_DID_NOT_MEET_PANIC");}
    Ok(Evaluation{expected:0,mask:u64::MAX,output_ptr:Some(out),output_fields:fields,metrics})
}
#[cfg(test)]
#[path="models_candidates_tests.rs"]
mod tests;
