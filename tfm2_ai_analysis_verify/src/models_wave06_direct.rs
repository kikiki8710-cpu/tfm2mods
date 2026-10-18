//! wave06 부분Option/iterator scratch. 게임helper0, 실제소스와분리.
use crate::engine::{Memory,Evaluation,Metric};
fn span(a:u64,w:u64)->Result<(u64,u64),&'static str>{let end=a.checked_add(w).ok_or("ADDRESS_SPAN")?;if a<0x10000||end>(1u64<<48)||end<=a{return Err("ADDRESS_SPAN");}Ok((a,end))}
fn overlaps(a:(u64,u64),b:(u64,u64))->bool{a.0<b.1&&b.0<a.1}
fn read(m:&mut dyn Memory,b:u64,o:u64,w:u8,output:(u64,u64),cursor:Option<(u64,u64)>)->Result<u64,&'static str>{let address=b.wrapping_add(o);let s=span(address,w as u64)?;if overlaps(s,output){return Err("OUTPUT_INPUT_ALIAS");}if cursor.is_some_and(|c|overlaps(s,c)){return Err("CURSOR_INPUT_ALIAS");}m.read(address,w)}
fn mark(v:&mut Vec<Metric>,n:&'static str,l:u64,r:u64,rel:&'static str,p:bool){v.push(Metric{name:n,lhs:l as i128,rhs:r as i128,relation:rel,predicate:p,observed_branch:false});}
fn done(base:u64,x:u64,mask:u64,fields:Vec<(u64,u8,u64)>,metrics:Vec<Metric>)->Evaluation{Evaluation{expected:x,mask,output_ptr:Some(base),output_fields:fields,metrics}}
pub fn evaluate(id:usize,a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
 let base=a[0];let output=span(base,if id==37{17}else{24})?;let mut metrics=vec![];
 if id==37{
  let k=a[2].wrapping_sub(6);let allowed=k<=3;mark(&mut metrics,"w06_effect_action_kind_allowed",k,3,"<=",allowed);
  if !allowed{return Ok(done(base,0,0,vec![(base.wrapping_add(16),1,2)],metrics));}
  let actor=a[1];let kind=a[2];let effect;
  if kind==6||kind==7{effect=actor.wrapping_add(if kind==6{0x488}else{0x4c0});}else{
   let level=read(m,actor,0x5c0,8,output,None)?;let min=if kind==8{3}else{5};let ready=level>=min;
   mark(&mut metrics,if kind==8{"w06_effect_level_ge3"}else{"w06_effect_level_ge5"},level,min,">=",ready);
   if !ready{return Err("STATIC_DESCRIPTOR_REQUIRES_LOADED_CAPTURE");}
   effect=actor.wrapping_add(if kind==8{0x4f8}else{0x530});
  }
  let tag=read(m,effect,0x30,4,output,None)?;let present=tag!=0xffff_ffff;mark(&mut metrics,"w06_effect_kind_present",tag,0xffff_ffff,"!=",present);
  if !present{return Err("E20D00_EFFECT_KIND_NONE_GAME_PANIC");}
  let current_kind=read(m,actor,0x68,8,output,None)?;let current_state=read(m,actor,0x70,8,output,None)?;let diff=(current_kind^15)|(current_state^(kind-3));let matches=diff==0;
  mark(&mut metrics,"w06_effect_action_match",diff,0,"==",matches);
  return Ok(done(base,effect,u64::MAX,vec![(base,8,effect),(base.wrapping_add(8),8,a[3]),(base.wrapping_add(16),1,matches as u64)],metrics));
 }
 if id==38{
  let iterator=a[1];let begin=read(m,iterator,0,8,output,None)?;let end=read(m,iterator,8,8,output,None)?;
  if begin==end{return Ok(done(base,base,u64::MAX,vec![(base,8,0)],metrics));}
  if end<begin||end-begin>32{return Err("ITERATOR_CAPTURE_LIMIT32_NOT_SOURCE_BOUND");}
  let cursor=Some(span(iterator,8)?);
  let wrapper=read(m,a[2],0,8,output,cursor)?;let tr_a=read(m,wrapper,8,8,output,cursor)?;let team_a=read(m,tr_a,0,8,output,cursor)?;
  if team_a>1{return Err("ITERATOR_BB_TEAM_PANIC_AFTER_CURSOR_WRITE");}
  let line_ref=read(m,a[2],8,8,output,cursor)?;let world=read(m,a[3],0,8,output,cursor)?;let tr_b=read(m,a[3],8,8,output,cursor)?;let bb=read(m,wrapper,0,8,output,cursor)?;let bank=bb.wrapping_add(team_a*0xac8);let mut index=begin;
  while index!=end{
   let key=read(m,iterator,0x10u64.wrapping_add(index.wrapping_mul(4)),4,output,cursor)?;
   let state=read(m,bank,key*32+0x7f0,1,output,cursor)?;let zero=state==0;mark(&mut metrics,"w06_iterator_state_zero",state,0,"==",zero);
   if zero{
    let line=read(m,bank,key*32+0x7f1,1,output,cursor)?;let wanted=read(m,line_ref,0,1,output,cursor)?;let same=line==wanted;mark(&mut metrics,"w06_iterator_line_equal",line,wanted,"==",same);
    if same{
     let team_b=read(m,tr_b,0,8,output,cursor)?;if team_b>1{return Err("ITERATOR_WORLD_TEAM_PANIC_AFTER_CURSOR_WRITE");}
     let entity=read(m,world,0x238+team_b*40+key*8,8,output,cursor)?;mark(&mut metrics,"w06_iterator_entity_present",entity,0,"!=",entity!=0);
     if entity!=0{
      index=index.wrapping_add(1);let reference=read(m,a[3],0x10,8,output,cursor)?;let rx=read(m,reference,0x658,8,output,cursor)?;let ry=read(m,reference,0x660,8,output,cursor)?;let ex=read(m,entity,0x658,8,output,cursor)?;let ey=read(m,entity,0x660,8,output,cursor)?;
      let dx=rx.wrapping_sub(ex);let dy=ry.wrapping_sub(ey);let d2=dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
      return Ok(done(base,base,u64::MAX,vec![(iterator,8,index),(base.wrapping_add(8),8,d2),(base.wrapping_add(16),8,key),(base,8,1)],metrics));
     }
    }
   }
   index=index.wrapping_add(1);
  }
  return Ok(done(base,base,u64::MAX,vec![(iterator,8,end),(base,8,0)],metrics));
 }
 Err("MODEL_NOT_OWNED")
}

#[cfg(test)]
#[path="models_wave06_direct_tests.rs"]
mod boundary_tests;
