//! wave09의5/5/6인수독립모델. 게임helper/외부쓰기없음. 입력arity를생략하지않는다.
use crate::engine::{Evaluation,Memory,Metric};
struct Reader<'a>{memory:&'a mut dyn Memory,fields:Vec<(u64,u8)>}
impl Reader<'_>{fn read(&mut self,base:u64,offset:u64,width:u8)->Result<u64,&'static str>{
 let p=base.wrapping_add(offset);let end=p.checked_add(width as u64).ok_or("INPUT_SPAN")?;
 if p<0x10000||end>(1u64<<48){return Err("INPUT_SPAN")}
 if !self.fields.contains(&(p,width)){self.fields.push((p,width));if self.fields.len()>96{return Err("W09_FIELD_CAPTURE_CAP96")}}
 self.memory.read(p,width)
}}
fn mark(v:&mut Vec<Metric>,name:&'static str,lhs:u64,rhs:u64,relation:&'static str,predicate:bool){v.push(Metric{name,lhs:lhs as i128,rhs:rhs as i128,relation,predicate,observed_branch:false});}
fn done(value:u64,id:usize,metrics:Vec<Metric>)->Evaluation{Evaluation{expected:value,mask:if id==44{255}else{u64::MAX},output_ptr:None,output_fields:vec![],metrics}}
fn d2(x:u64,y:u64,ox:u64,oy:u64)->u64{let dx=x.wrapping_sub(ox);let dy=y.wrapping_sub(oy);dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))}
pub fn evaluate(id:usize,args:&[u64],memory:&mut dyn Memory)->Result<Evaluation,&'static str>{
 let arity=match id{44|45=>5,46=>6,_=>return Err("MODEL_NOT_OWNED")};
 if args.len()!=arity{return Err("STACKED_ARITY_OR_ARGUMENTS")}
 let mut r=Reader{memory,fields:vec![]};let mut metrics=vec![];
 if id==44{
  let capture=args[0];let player=args[3];let data=args[4];let team=r.read(player,0x800,8)?;
  if team>=2{return Err("W09_MOVE_TEAM_GAME_PANIC")}
  let role=r.read(player,0x890,4)?;let cache=r.read(data,0,8)?;let actor=r.read(cache,0x238+team*40+role*8,8)?;
  if actor==0{return Err("W09_MOVE_ACTOR_NULL_GAME_PANIC")}
  let x=r.read(actor,0x658,8)?;let y=r.read(actor,0x660,8)?;let context=r.read(data,8,8)?;let config=r.read(context,0x20,8)?;
  let rect=config.wrapping_add(team*32+0x6d70);let lowy=r.read(rect,8,8)?;let mut inside=false;
  mark(&mut metrics,"w09_move_y_ge_rect_min",y,lowy,">=",y>=lowy);
  if y>=lowy{let lowx=r.read(rect,0,8)?;mark(&mut metrics,"w09_move_x_ge_rect_min",x,lowx,">=",x>=lowx);
   if x>=lowx{let highx=r.read(rect,16,8)?;mark(&mut metrics,"w09_move_x_le_rect_max",x,highx,"<=",x<=highx);
    if x<=highx{let highy=r.read(rect,24,8)?;inside=y<=highy;mark(&mut metrics,"w09_move_y_le_rect_max",y,highy,"<=",inside);}}}
  let duration=r.read(capture,0x78,8)?;let stamp=r.read(capture,0x60,8)?;let expiry=duration.wrapping_add(stamp);let now=r.read(cache,0x1b8,8)?;
  mark(&mut metrics,"w09_move_now_ge_expiry",now,expiry,">=",now>=expiry);
  if now>=expiry{return Ok(done(1,id,metrics))}
  let oldx=r.read(capture,0x68,8)?;let oldy=r.read(capture,0x70,8)?;let distance=d2(x,y,oldx,oldy);let near=distance<=100000000;
  mark(&mut metrics,"w09_move_previous_d2_le_100m",distance,100000000,"<=",near);
  return Ok(done((inside||near)as u64,id,metrics))
 }
 if id==45{
  let lane=args[0]&255;let team=args[1];let position=args[2]&0xffffffff;let world=args[3];let data=args[4];
  mark(&mut metrics,"w09_selector_lane_is0",lane,0,"==",lane==0);
  if lane!=0{mark(&mut metrics,"w09_selector_lane_is1",lane,1,"==",lane==1)}
  if team>=2{return Err("W09_SELECTOR_TEAM_GAME_PANIC")}
  let(table,offset)=if lane==0{(if team==0{[16,6,3,3,3,2,2]}else{[2,3,6,6,6,16,16]},0x2d8)}
  else if lane==1{
   let actor=r.read(world,0x238+team*40+position*8,8)?;if actor==0{return Err("W09_SELECTOR_ACTOR_NULL_GAME_PANIC")}
   let config=r.read(data,8,8)?;let diagonal=r.read(config,0x12c0,8)?;let y=r.read(actor,0x660,8)?;let cut=diagonal.wrapping_sub(y);let x=r.read(actor,0x658,8)?;
   let side=(cut<x)as u64;mark(&mut metrics,"w09_selector_diagonal_side",cut,x,"<",side!=0);
   let a=11+3*side;let b=17+4*side;let c=4+5*side;(if team==0{[b,a,a,a,a,c,c]}else{[c,a,a,a,a,b,b]},0x2e8)
  }else{(if team==0{[21,20,15,15,15,9,7]}else{[9,15,20,20,20,21,23]},0x2f8)};
  let index=r.read(world,offset+team*8,8)?;mark(&mut metrics,"w09_selector_index_lt6",index,6,"<",index<6);
  return Ok(done(table[index.min(6)as usize],id,metrics))
 }
 let player=args[0];let data=args[1];let x=args[2];let y=args[3];let radius=args[4];let threshold=args[5];
 let team=r.read(player,0x800,8)?;if team>=2{return Err("W09_COUNT_TEAM_GAME_PANIC")}
 let radius2=radius.wrapping_mul(radius);let cache=r.read(data,0,8)?;let mut count=0u64;
 for role in 0..5{let actor=r.read(cache,0x238+team*40+role*8,8)?;if actor==0{continue}
  let maximum=r.read(actor,0x620,8)?;if maximum==0{return Err("W09_COUNT_MAXHP_ZERO_GAME_PANIC")}
  let hp=r.read(actor,0x668,8)?;let numerator=hp.wrapping_mul(100);let high=(numerator|maximum)>>32;
  mark(&mut metrics,"w09_count_hp_div32_path",high,0,"==",high==0);
  let pct=numerator/maximum;mark(&mut metrics,"w09_count_hp_ge_input",pct,threshold,">=",pct>=threshold);
  if pct<threshold{continue}
  let ax=r.read(actor,0x658,8)?;let ay=r.read(actor,0x660,8)?;let distance=d2(ax,ay,x,y);
  mark(&mut metrics,"w09_count_d2_le_radius2",distance,radius2,"<=",distance<=radius2);
  count+=u64::from(distance<=radius2);
 }
 Ok(done(count,id,metrics))
}
#[cfg(test)]
mod tests{include!("models_stacked09_tests.rs");}
