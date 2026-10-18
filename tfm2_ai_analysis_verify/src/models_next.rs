// D3EEC0 scratch1000. M소스와배선에는연결하지않는다.
use crate::engine::{Memory,Evaluation,Metric};
fn read(memory:&mut dyn Memory, out:u64, fields:&mut Vec<(u64,u8)>, base:u64, offset:u64, width:u8, seed:bool)->Result<u64,&'static str>{
    let p=base.wrapping_add(offset);
    let end=p.checked_add(width as u64).ok_or("INPUT_SPAN")?;
    if p<0x10000 || end>(1u64<<48){return Err("INPUT_SPAN")}
    if !seed && p<out+24 && out<end{return Err("OUTPUT_INPUT_ALIAS")}
    if !fields.contains(&(p,width)){if fields.len()>=96{return Err("FIELD_BUDGET")}fields.push((p,width));}
    memory.read(p,width)
}
pub fn evaluate(id:usize,args:[u64;4],memory:&mut dyn Memory)->Result<Evaluation,&'static str>{
    if id!=1000{return Err("SCRATCH_ID_NOT_OWNED")}
    let [out,cap,flag,_]=args;
    if out<0x10000 || out.checked_add(24).filter(|&v|v<=(1u64<<48)).is_none(){return Err("OUTPUT_SPAN")}
    let mut fields=Vec::new();let mut metrics=Vec::new();
    macro_rules! r{($b:expr,$o:expr,$w:expr)=>{read(memory,out,&mut fields,$b,$o,$w,false)?}}
    let mut cost=read(memory,out,&mut fields,out,0,8,true)?;
    let mut index=read(memory,out,&mut fields,out,8,8,true)?;
    let teamref=r!(cap,0x18,8);let world=r!(cap,8,8);let lines=r!(cap,0x20,8);let excludedref=r!(cap,0,8);
    let bb=r!(cap,0x10,8);let allowedref=r!(cap,0x28,8);let reference=r!(cap,0x30,8);let excluded=r!(excludedref,0,8);
    let mut linebest=255u64;
    for i in 0..5u64{
        if i==excluded{continue}
        let team=r!(teamref,0,8);if team>=2{return Err("TEAM_PANIC_WITH_PARTIAL_OUTPUT")}
        let entity=r!(world,0x238+team*40+i*8,8);if entity==0{continue}
        let line=r!(lines,i,1);if line==255{continue}
        if flag as u8!=0{
            if r!(bb,team*0xac8+i*32+0x7f0,1)!=0{continue}
            if r!(bb,team*0xac8+i*32+0x7f1,1)!=line{continue}
        }
        let allowed=r!(allowedref,0,8);let first=r!(allowed,0,1);
        if first!=255 && first!=line && r!(allowed,1,1)!=line{continue}
        let rx=r!(reference,0x658,8);let ry=r!(reference,0x660,8);let x=r!(entity,0x658,8);let y=r!(entity,0x660,8);
        let dx=rx.wrapping_sub(x);let dy=ry.wrapping_sub(y);
        let d2=dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
        let slot=if line==0{0}else if line==1{1}else{2};
        let progress=r!(bb,team*0xac8+slot*0x28+0x10,8);
        let candidate=(d2/1_000_000).wrapping_add(progress.wrapping_mul(2));
        let firstcandidate=linebest==255;let improves=(candidate as i64)<(cost as i64);
        metrics.push(Metric{name:"next_peer_first_candidate",lhs:linebest as i128,rhs:255,relation:"==",predicate:firstcandidate,observed_branch:false});
        metrics.push(Metric{name:"next_peer_cost_improves",lhs:(candidate as i64)as i128,rhs:(cost as i64)as i128,relation:"<",predicate:improves,observed_branch:false});
        if firstcandidate || improves{cost=candidate;index=i;linebest=line;}
    }
    Ok(Evaluation{expected:out,mask:u64::MAX,output_ptr:Some(out),output_fields:vec![(out+8,8,index),(out,8,cost),(out+16,1,linebest)],metrics})
}

#[cfg(test)]
#[path="models_next_tests.rs"]
mod tests;
