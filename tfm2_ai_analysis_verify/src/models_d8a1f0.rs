//! 미배선 scratch: 기존 D8A1F0. 전역 probe id를 배정하지 않는다.
use crate::engine::{Evaluation,Memory,Metric};
pub const INDEX_PREFIX_CAP:u64=20;
fn span(a:u64,n:u64)->Result<(u64,u64),&'static str>{
    let b=a.checked_add(n).ok_or("INSERT_ADDRESS_RANGE_OR_OVERFLOW")?;
    if a<0x10000||n==0||b>(1u64<<48){return Err("INSERT_ADDRESS_RANGE_OR_OVERFLOW");}Ok((a,b))
}
struct Reader<'a>{memory:&'a mut dyn Memory,indices:(u64,u64)}
impl Reader<'_>{
    fn read(&mut self,a:u64,index:bool)->Result<u64,&'static str>{
        let s=span(a,8)?;
        if !index&&s.0<self.indices.1&&self.indices.0<s.1{return Err("INSERT_READONLY_INDEX_ALIAS");}
        self.memory.read(a,8)
    }
    fn score(&mut self,p:u64,i:u64)->Result<u64,&'static str>{
        let a=p.wrapping_add(i.wrapping_mul(8));
        self.read(a,false)
    }
}
fn precedes(pred:u64,key:u64,ps:u64,ks:u64,metrics:&mut Vec<Metric>)->bool{
    let less=(ps as i64)<(ks as i64);
    metrics.push(Metric{name:"insert_predecessor_score_lt_key",lhs:ps as i64 as i128,rhs:ks as i64 as i128,relation:"<",predicate:less,observed_branch:false});
    if ps==ks{
        let tie=key<pred;
        metrics.push(Metric{name:"insert_equal_score_unsigned_index_tie",lhs:key as i128,rhs:pred as i128,relation:"<",predicate:tie,observed_branch:false});tie
    }else{less}
}
pub fn evaluate(a:[u64;4],memory:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let(begin,pos,header)=(a[0],a[1],a[2]);
    if pos<=begin||(pos-begin)%8!=0{return Err("INSERT_PREDECESSOR_OR_ALIGNMENT");}
    let count=(pos-begin)/8+1;
    if count>INDEX_PREFIX_CAP{return Err("INSERT_INDEX_CAPTURE_CAP");}
    let indices=span(begin,count*8)?;let mut r=Reader{memory,indices};
    let key=r.read(pos,true)?;
    let mut length=r.read(header.checked_add(8).ok_or("INSERT_ADDRESS_RANGE_OR_OVERFLOW")?,false)?;
    if key>=length{return Err("INSERT_KEY_BOUNDS_PANIC");}
    let mut cursor=pos-8;let mut pred=r.read(cursor,true)?;
    if pred>=length{return Err("INSERT_PREDECESSOR_BOUNDS_PANIC");}
    let mut ptr=r.read(header,false)?;
    let mut ps=r.score(ptr,pred)?;let mut ks=r.score(ptr,key)?;
    let mut metrics=Vec::new();let mut fields=Vec::new();
    if precedes(pred,key,ps,ks,&mut metrics){
        let mut hole=pos;
        loop{
            fields.push((hole,8,pred));hole=cursor;
            if cursor==begin{break;}
            length=r.read(header.checked_add(8).ok_or("INSERT_ADDRESS_RANGE_OR_OVERFLOW")?,false)?;
            if key>=length{return Err("INSERT_LATE_KEY_BOUNDS_PANIC");}
            cursor-=8;pred=r.read(cursor,true)?;
            if pred>=length{return Err("INSERT_LATE_PREDECESSOR_BOUNDS_PANIC");}
            ptr=r.read(header,false)?;ps=r.score(ptr,pred)?;ks=r.score(ptr,key)?;
            if !precedes(pred,key,ps,ks,&mut metrics){break;}
        }
        fields.push((hole,8,key));
    }
    Ok(Evaluation{expected:pred,mask:u64::MAX,output_ptr:if fields.is_empty(){None}else{Some(begin)},output_fields:fields,metrics})
}
#[cfg(test)]
#[path="models_d8a1f0_tests.rs"]
mod tests;
