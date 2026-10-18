//! runtime42/43 순수 독립 모델 준비. 게임helper와외부메모리쓰기는없다.
use crate::engine::{Evaluation,Memory,Metric};
struct Reader<'a>{memory:&'a mut dyn Memory,fields:Vec<(u64,u8)>}
impl Reader<'_>{
 fn read(&mut self,base:u64,offset:u64,width:u8)->Result<u64,&'static str>{
  let p=base.wrapping_add(offset);let end=p.checked_add(width as u64).ok_or("INPUT_SPAN")?;
  if p<0x10000||end>(1u64<<48){return Err("INPUT_SPAN")}
  if !self.fields.contains(&(p,width)){self.fields.push((p,width));if self.fields.len()>96{return Err("W08_FIELD_CAPTURE_CAP96")}}
  self.memory.read(p,width)
 }
}
fn mark(v:&mut Vec<Metric>,name:&'static str,lhs:i128,rhs:i128,relation:&'static str,predicate:bool){
 v.push(Metric{name,lhs,rhs,relation,predicate,observed_branch:false});
}
fn done(expected:u64,metrics:Vec<Metric>)->Evaluation{
 Evaluation{expected,mask:255,output_ptr:None,output_fields:vec![],metrics}
}
fn d2(ax:u64,ay:u64,bx:u64,by:u64)->u64{let dx=ax.wrapping_sub(bx);let dy=ay.wrapping_sub(by);dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))}
pub fn evaluate(id:usize,args:[u64;4],memory:&mut dyn Memory)->Result<Evaluation,&'static str>{
 if id!=42&&id!=43{return Err("MODEL_NOT_OWNED")}
 let mut r=Reader{memory,fields:vec![]};let mut metrics=vec![];
 if id==42{
  let[team,cache,context,_]=args;
  if team>1{return Err("W08_WAVE_TEAM_GAME_PANIC")}
  let base=r.read(cache,0x1c8+team*8,8)?;
  if base==0{return Err("W08_WAVE_BASE_GAME_PANIC")}
  let ownlen=r.read(cache,0x188+team*32,8)?;
  mark(&mut metrics,"w08_wave_own_nonempty",ownlen as i128,0,"!=",ownlen!=0);
  if ownlen==0{return Ok(done(0,metrics))}
  let ownvec=r.read(cache,0x170+team*32,8)?;let first=r.read(ownvec,0,8)?;
  let fx=r.read(first,0x658,8)?;let fy=r.read(first,0x660,8)?;
  let bx=r.read(base,0x658,8)?;let by=r.read(base,0x660,8)?;
  let mode=r.read(context,0x38,1)?;
  mark(&mut metrics,"w08_wave_mode_le8",mode as i128,8,"<=",mode<=8);
  if mode>8{return Ok(done(0,metrics))}
  let reference=d2(fx,fy,bx,by);let opponent=1-team;let mut records=0u64;
  let lanes:&[u64]=match mode{0|7|8=>&[0,1,2],1|3=>&[2],2=>&[0],4=>&[1],5=>&[1,2],_=>&[]};
  for &lane in lanes{
   let a=r.read(cache,0x1d8+lane*32+team*8,8)?;let b=r.read(cache,0x1e8+lane*32+team*8,8)?;let pair=a|b;
   mark(&mut metrics,"w08_wave_lane_pair_zero",pair as i128,0,"==",pair==0);
   if pair!=0{continue}
   let header=cache.wrapping_add(opponent*32+[0x10,0x90,0xd0][lane as usize]);
   let length=r.read(header,0x18,8)?;
   if length==0{continue}
   if length>24||records+length>24{return Err("W08_WAVE_RECORD_CAPTURE_CAP24_NOT_SOURCE_BOUND")}
   let vector=r.read(header,0,8)?;let mut count=0u64;
   for index in 0..length{
    let entity=r.read(vector,index*8,8)?;let x=r.read(entity,0x658,8)?;let y=r.read(entity,0x660,8)?;
    let distance=d2(x,y,bx,by);let inside=distance<=reference;
    mark(&mut metrics,"w08_wave_distance_within_reference",distance as i128,reference as i128,"<=",inside);
    count=count.wrapping_add(inside as u64);records+=1;
   }
   let accepted=count>=6;
   mark(&mut metrics,"w08_wave_qualifying_count_ge6",count as i128,6,">=",accepted);
   if accepted{return Ok(done(1,metrics))}
  }
  return Ok(done(0,metrics))
 }
 let[player,data,candidates,length]=args;
 if length==0{return Ok(done(255,metrics))}
 let context=r.read(data,8,8)?;let team=r.read(player,0x800,8)?;let mode=r.read(context,0x38,1)?;
 if team>=2{return Err("W08_OBJECTIVE_DIAGNOSTIC_TEAM_DOMAIN_UNSUPPORTED")}
 let world=r.read(data,0,8)?;let bb=r.read(data,0x10,8)?;
 mark(&mut metrics,"w08_objective_mode_le8",mode as i128,8,"<=",mode<=8);
 if mode>8||mode==6{return Ok(done(255,metrics))}
 if length>32{return Err("W08_OBJECTIVE_CANDIDATE_CAPTURE_CAP32_NOT_SOURCE_BOUND")}
 let mut answer=255u64;let mut best=0u64;let bank=bb.wrapping_add(team*0xac8);
 for index in 0..length{
  let key=r.read(candidates,index,1)?;
  let admitted=if mode==2{key==0}else if mode==4{key==1}else if key==0{(0x185u64>>mode)&1!=0}else if key==2{true}else{(0x1b1u64>>mode)&1!=0};
  mark(&mut metrics,"w08_objective_candidate_admitted",admitted as i128,0,"!=",admitted);
  if !admitted{continue}
  let group=if key==0{0}else if key==2{2}else{1};let lane=bank.wrapping_add(group*40);
  let rank=r.read(world,0x2d8+team*8+key*16,8)?;let lowrank=rank<3;
  mark(&mut metrics,"w08_objective_rank_below3",rank as i128,3,"<",lowrank);
  let metadata=r.read(lane,0x10,8)?;
  if !lowrank{
   let lowmeta=(metadata as i64)<2000;
   mark(&mut metrics,"w08_objective_metadata_below2000",(metadata as i64)as i128,2000,"<",lowmeta);
   if !lowmeta{
    let state=r.read(lane,0x20,4)?;let lowstate=(state as u32 as i32)<2;
    mark(&mut metrics,"w08_objective_state_below2",(state as u32 as i32)as i128,2,"<",lowstate);
    if !lowstate{continue}
   }
  }
  let state=r.read(lane,0x20,4)?;
  let cost=rank.wrapping_mul(10000).wrapping_add(metadata).wrapping_add((state as u32 as i32 as i64 as u64).wrapping_mul(1000));
  let first=answer==255;let improves=(cost as i64)<(best as i64);
  mark(&mut metrics,"w08_objective_first_candidate",answer as i128,255,"==",first);
  mark(&mut metrics,"w08_objective_cost_improves",(cost as i64)as i128,(best as i64)as i128,"<",improves);
  if first||improves{answer=key;best=cost}
 }
 Ok(done(answer,metrics))
}

#[cfg(test)]
#[path="models_wave08_direct_tests.rs"]
mod boundary_tests;
