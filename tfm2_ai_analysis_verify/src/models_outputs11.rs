//! 출력5/6/3인수의순수모델. output_fields만예상하며실메모리에쓰지않는다.
use crate::engine::{Evaluation,Memory,Metric};
pub type ModelResult=Evaluation;
const HIGH:u64=1<<63;
fn checked(p:u64,w:u64)->Result<(),&'static str>{let end=p.checked_add(w).ok_or("W11_INPUT_OR_OUTPUT_SPAN")?;if p<0x10000||end>(1u64<<48){Err("W11_INPUT_OR_OUTPUT_SPAN")}else{Ok(())}}
struct Reader<'a>{memory:&'a mut dyn Memory,out:u64,size:u64,fields:Vec<(u64,u8,u64)>}
impl Reader<'_>{fn read(&mut self,b:u64,o:u64,w:u8)->Result<u64,&'static str>{let p=b.wrapping_add(o);checked(p,w as u64)?;
 if p<self.out+self.size&&self.out<p+w as u64{return Err("W11_OUTPUT_INPUT_ALIAS_UNSUPPORTED")}
 if let Some(x)=self.fields.iter().find(|x|x.0==p&&x.1==w){return Ok(x.2)}
 if self.fields.len()>=96{return Err("W11_UNIQUE_READ_CAPTURE_CAP96_NOT_SOURCE_BOUND")}
 let value=self.memory.read(p,w)?;if w<8&&value>=(1u64<<(w*8)){return Err("W11_CAPTURE_VALUE_WIDTH")}
 self.fields.push((p,w,value));Ok(value)
}}
fn mark(v:&mut Vec<Metric>,name:&'static str,l:u64,r:u64,relation:&'static str,predicate:bool){v.push(Metric{name,lhs:l as i128,rhs:r as i128,relation,predicate,observed_branch:false});}
fn done(id:usize,out:u64,fields:Vec<(u64,u8,u64)>,metrics:Vec<Metric>)->ModelResult{Evaluation{expected:if id==52{0}else{out},mask:if id==52{0}else{u64::MAX},output_ptr:Some(out),output_fields:fields.into_iter().map(|(o,w,v)|(out.wrapping_add(o),w,v)).collect(),metrics}}
fn resolve_entity(r:&mut Reader<'_>,world:u64,entity:u64,metrics:&mut Vec<Metric>)->Result<Option<u64>,&'static str>{
 let byte=r.read(entity,0,1)?;mark(metrics,"w11_entity_byte_zero",byte,0,"==",byte==0);if byte!=0{return Ok(None)}
 let team=r.read(entity,8,8)?;mark(metrics,"w11_entity_team_lt2",team,2,"<",team<2);if team>=2{return Ok(None)}
 let typ=r.read(entity,0x68,4)?;mark(metrics,"w11_entity_type_is15",typ,15,"==",typ==15);
 if typ==15{let pos=r.read(entity,0xd8,4)?;let ptr=r.read(world,0x238+team*40+pos*8,8)?;mark(metrics,"w11_entity_direct_slot_match",ptr,entity,"==",ptr==entity);if ptr==entity{return Ok(Some(team*5+pos))}}
 for pos in 0..5{let ptr=r.read(world,0x238+team*40+pos*8,8)?;mark(metrics,"w11_entity_scan_slot_match",ptr,entity,"==",ptr==entity);if ptr==entity{return Ok(Some(team*5+pos))}}
 Ok(None)
}
fn distance(x:u64,y:u64,rx:u64,ry:u64)->u64{let dx=x.wrapping_sub(rx);let dy=y.wrapping_sub(ry);dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))}
pub fn evaluate(id:usize,args:&[u64],memory:&mut dyn Memory)->Result<ModelResult,&'static str>{
 let arity=match id{50=>5,51=>6,52=>3,_=>return Err("MODEL_NOT_OWNED")};if args.len()!=arity{return Err("OUTPUT11_ARITY_OR_ARGUMENTS")}
 let out=if id==52{args[1]}else{args[0]};let size=match id{50=>24,51=>40,_=>32};checked(out,size)?;
 let mut r=Reader{memory,out,size,fields:vec![]};let mut metrics=vec![];
 if id==50{
  let world=args[1];let main=args[2];let slice=args[3];let len=args[4];let index=match resolve_entity(&mut r,world,main,&mut metrics)?{Some(i)=>i,None=>return Ok(done(id,out,vec![(0,8,0)],metrics))};
  mark(&mut metrics,"w11_entity_slice_len_zero",len,0,"==",len==0);let mut mask=0u32;
  if len!=0{let bytes=len.wrapping_mul(8);let mut offset=0u64;loop{
   let entity=r.read(slice,offset,8)?;let i=match resolve_entity(&mut r,world,entity,&mut metrics)?{Some(i)=>i,None=>return Ok(done(id,out,vec![(0,8,0)],metrics))};
   let bit=1u32.wrapping_shl((i&31)as u32);let already=mask&bit!=0;mark(&mut metrics,"w11_extra_bit_already_set",already as u64,0,"!=",already);
   if already{return Ok(done(id,out,vec![(0,8,0)],metrics))}mask|=bit;offset=offset.wrapping_add(8);if offset==bytes{break}
  }}
  return Ok(done(id,out,vec![(8,8,index),(16,2,(mask&0xffff)as u64),(0,8,1)],metrics))
 }
 if id==51{
  let state=args[1];let player=args[4];let data=args[5];let team=r.read(player,0x800,8)?;if team>=2{return Err("W11_NEXUS_TEAM_GAME_PANIC")}
  let role=r.read(player,0x890,4)?;let world=r.read(data,0,8)?;let own=r.read(world,0x238+team*40+role*8,8)?;if own==0{return Err("W11_NEXUS_OWN_NULL_GAME_PANIC")}
  let y=r.read(own,0x660,8)?;let context=r.read(data,8,8)?;let root=r.read(context,0x20,8)?;let base=0x6d70+team*32;
  let ymin=r.read(root,base+8,8)?;let mut inside=y>=ymin;mark(&mut metrics,"w11_nexus_y_ge_min",y,ymin,">=",inside);
  if inside{let x=r.read(own,0x658,8)?;let xmin=r.read(root,base,8)?;inside=x>=xmin;mark(&mut metrics,"w11_nexus_x_ge_min",x,xmin,">=",inside);
   if inside{let xmax=r.read(root,base+16,8)?;inside=x<=xmax;mark(&mut metrics,"w11_nexus_x_le_max",x,xmax,"<=",inside);
    if inside{let ymax=r.read(root,base+24,8)?;inside=y<=ymax;mark(&mut metrics,"w11_nexus_y_le_max",y,ymax,"<=",inside);}
   }
  }
  if inside{let hp=r.read(own,0x668,8)?;let max=r.read(own,0x620,8)?;mark(&mut metrics,"w11_nexus_hp_lt_max",hp,max,"<",hp<max);if hp<max{return Ok(done(id,out,vec![(0,8,HIGH+3)],metrics))}}
  let enemy=1-team;let nexus=r.read(world,0x1c8+enemy*8,8)?;mark(&mut metrics,"w11_opponent_nexus_present",nexus,0,"!=",nexus!=0);
  if nexus!=0{let flag=r.read(nexus,0x6b1,1)?;mark(&mut metrics,"w11_opponent_flag_exact1",flag,1,"==",flag==1);
   if flag==1{let len=r.read(world,0x188+enemy*32,8)?;let q=r.read(nexus,0x698,8)?;let joined=len|q;mark(&mut metrics,"w11_opponent_payload_or_zero",joined,0,"==",joined==0);if joined==0{return Ok(done(id,out,vec![(0,8,HIGH+14)],metrics))}}
  }
  let byte=r.read(state,8,1)?;return Ok(done(id,out,vec![(8,8,0),(24,8,0),(32,2,0),(34,1,byte),(35,1,2),(0,8,HIGH)],metrics))
 }
 let source=args[0];let capture=args[2];let i1=r.read(source,8,8)?;let header=r.read(capture,0,8)?;let length=r.read(header,24,8)?;
 if i1>=length{return Err("W11_SORT4_INDEX1_GAME_PANIC")}
 let i0=r.read(source,0,8)?;if i0>=length{return Err("W11_SORT4_INDEX0_GAME_PANIC")}
 let array=r.read(header,0,8)?;let a1=r.read(array,i1.wrapping_mul(8),8)?;let x1=r.read(a1,0x658,8)?;let y1=r.read(a1,0x660,8)?;
 let reference=r.read(capture,8,8)?;let ry=r.read(reference,0x660,8)?;let rx=r.read(reference,0x658,8)?;
 let a0=r.read(array,i0.wrapping_mul(8),8)?;let x0=r.read(a0,0x658,8)?;let y0=r.read(a0,0x660,8)?;
 let i3=r.read(source,24,8)?;if i3>=length{return Err("W11_SORT4_INDEX3_GAME_PANIC")}
 let i2=r.read(source,16,8)?;if i2>=length{return Err("W11_SORT4_INDEX2_GAME_PANIC")}
 let a3=r.read(array,i3.wrapping_mul(8),8)?;let x3=r.read(a3,0x658,8)?;let y3=r.read(a3,0x660,8)?;
 let a2=r.read(array,i2.wrapping_mul(8),8)?;let x2=r.read(a2,0x658,8)?;let y2=r.read(a2,0x660,8)?;
 let keys=[distance(x0,y0,rx,ry),distance(x1,y1,rx,ry),distance(x2,y2,rx,ry),distance(x3,y3,rx,ry)];let values=[i0,i1,i2,i3];
 let b=keys[0]<keys[1];let d=keys[2]<keys[3];mark(&mut metrics,"w11_sort4_pair01_key_lt",keys[0],keys[1],"<",b);mark(&mut metrics,"w11_sort4_pair23_key_lt",keys[2],keys[3],"<",d);
 let(lh,lt)=if b{(1,0)}else{(0,1)};let(rh,rt)=if d{(3,2)}else{(2,3)};
 let tails=keys[lt]<keys[rt];let heads=keys[lh]<keys[rh];mark(&mut metrics,"w11_sort4_tails_key_lt",keys[lt],keys[rt],"<",tails);mark(&mut metrics,"w11_sort4_heads_key_lt",keys[lh],keys[rh],"<",heads);
 let mut m0=if tails{rh}else{lt};if heads{m0=lh}let mut m1=if heads{lt}else{rh};if tails{m1=rt}
 let swap=keys[m0]<keys[m1];mark(&mut metrics,"w11_sort4_middle_key_lt",keys[m0],keys[m1],"<",swap);
 let order=[if heads{rh}else{lh},if swap{m1}else{m0},if swap{m0}else{m1},if tails{lt}else{rt}];
 Ok(done(id,out,order.iter().enumerate().map(|(n,&k)|((n*8)as u64,8,values[k])).collect(),metrics))
}
#[cfg(test)]mod tests{include!("models_outputs11_tests.rs");}
