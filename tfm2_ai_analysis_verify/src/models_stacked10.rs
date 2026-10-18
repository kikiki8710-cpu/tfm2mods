//! typed6독립모델. FILE9지원표/96고유readcap은loaded값검증·원본상한이아니다.
use crate::engine::{Evaluation,Memory,Metric};
pub type ModelResult=Evaluation;
const OFFSETS:[u64;9]=[0x13f8,0x1400,0x1400,0x1400,0x1400,0x1408,0x1400,0x13f8,0x13f8];
struct Reader<'a>{memory:&'a mut dyn Memory,fields:Vec<(u64,u8)>,id:usize}
impl Reader<'_>{fn read(&mut self,b:u64,o:u64,w:u8)->Result<u64,&'static str>{
 let p=b.wrapping_add(o);let end=p.checked_add(w as u64).ok_or("INPUT_SPAN")?;
 if p<0x10000||end>(1u64<<48){return Err("INPUT_SPAN")}
 if !self.fields.contains(&(p,w)){self.fields.push((p,w));if self.fields.len()>96{return Err(if self.id==49{"W10_EED_CAPTURE_UNIQUE_FIELDS96_NOT_SOURCE_BOUND"}else{"W10_CAPTURE_FIELDS96"})}}
 self.memory.read(p,w)
}}
fn mark(v:&mut Vec<Metric>,name:&'static str,l:i128,r:i128,rel:&'static str,p:bool){v.push(Metric{name,lhs:l,rhs:r,relation:rel,predicate:p,observed_branch:false});}
fn done(value:u64,id:usize,metrics:Vec<Metric>)->ModelResult{Evaluation{expected:value,mask:if id==49{255}else{u64::MAX},output_ptr:None,output_fields:vec![],metrics}}
fn div1000(v:u64)->u64{((((v>>3)as u128*0x20c49ba5e353f7cfu128)>>64)>>4)as u64}
fn scaled_radius(m:i32,r:u64)->u64{if m==0{r}else{let n=(m as i64 as u64).wrapping_add(100).wrapping_mul(r);((((n>>2)as u128*0x28f5c28f5c28f5c3u128)>>64)>>2)as u64}}
fn check_tower(r:&mut Reader<'_>,actor:u64,own:u64,x:u64,y:u64,pad:u64,metrics:&mut Vec<Metric>)->Result<bool,&'static str>{
 let niche=r.read(actor,0x4b8,4)?;let present=niche!=0xffffffff;mark(metrics,"w10_tower_effect_present",niche as i128,0xffffffff,"!=",present);if !present{return Ok(false)}
 let m=r.read(actor,0x468,4)?as u32 as i32;mark(metrics,"w10_tower_structure_modifier_nonzero",m as i128,0,"!=",m!=0);
 let rad=r.read(actor,0x678,8)?;let sr=scaled_radius(m,rad);
 let m=r.read(own,0x468,4)?as u32 as i32;mark(metrics,"w10_tower_own_modifier_nonzero",m as i128,0,"!=",m!=0);
 let rad=r.read(own,0x678,8)?;let ownr=scaled_radius(m,rad);
 let level=r.read(actor,0x5c0,8)?;let growth=r.read(actor,0x4a0,8)?;let base=r.read(actor,0x498,8)?;let buff=r.read(actor,0x430,8)?;
 let reach=base.wrapping_add(pad.wrapping_add(15000)).wrapping_add(buff).wrapping_add(level.wrapping_sub(1).wrapping_mul(growth)).wrapping_add(sr).wrapping_add(ownr);
 let boundary=reach.saturating_add(1);let square=(boundary as u128)*(boundary as u128);let high=(square>>64)as u64;
 mark(metrics,"w10_tower_square_overflow",high as i128,0,"!=",high!=0);if high!=0{return Ok(true)}
 let ax=r.read(actor,0x658,8)?;let ay=r.read(actor,0x660,8)?;let dx=x.wrapping_sub(ax);let dy=y.wrapping_sub(ay);let d=dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
 mark(metrics,"w10_tower_distance_strict_inside",d as i128,square as u64 as i128,"<",d<square as u64);Ok(d<square as u64)
}
pub fn evaluate(id:usize,args:&[u64],memory:&mut dyn Memory)->Result<ModelResult,&'static str>{
 if !matches!(id,47|48|49){return Err("MODEL_NOT_OWNED")}
 if args.len()!=6{return Err("STACKED_ARITY_OR_ARGUMENTS")}
 let mut r=Reader{memory,fields:vec![],id};let mut metrics=vec![];
 if id==47||id==48{
  let player=args[1];let data=args[2];let array=args[3];let n=args[4];let mode=args[5]&255;let stat=r.read(player,0x1f8,8)?;
  mark(&mut metrics,"w10_band_mode_nonzero",mode as i128,0,"!=",mode!=0);mark(&mut metrics,"w10_band_stat_below100",stat as i128,100,"<",stat<100);
  let t=stat.min(100);let a=50u64.saturating_sub(t);let b=t.saturating_sub(50);
  let(lo_d,hi_d)=if mode!=0{((200u64-2*t+a)as u32,(400u64.wrapping_sub(4*t).wrapping_add(2*a).wrapping_sub(b))as u32)}
  else{let rem=100u64.saturating_sub(stat);((6*rem+a).wrapping_sub(b)as u32,(5*rem+300+2*a).wrapping_sub(2*b)as u32)};
  let ls=lo_d as i32;let hs=hi_d as i32;let low=ls.max(0)as u64;let high=if hs as i64<=low as i64{low}else{hi_d as u64};
  mark(&mut metrics,"w10_band_low_signed_positive",ls as i128,0,">",ls>0);mark(&mut metrics,"w10_band_high_signed_le_low",hs as i128,low as i128,"<=",hs as i64<=low as i64);
  let rawlo=div1000(low.wrapping_mul(n));let rawhi=div1000(high.wrapping_mul(n));let last=n.wrapping_sub(1);let lo=rawlo.min(last);let unusual=rawhi<lo;
  mark(&mut metrics,"w10_band_raw_high_below_clamped_low",rawhi as i128,lo as i128,"<",unusual);
  let hi=(if unusual{rawlo}else{rawhi}).min(last);if hi>=n{return Err("W10_BAND_RANGE_GAME_PANIC")}
  let end=hi.wrapping_add(1);if end<lo{return Err("W10_BAND_RANGE_GAME_PANIC")}
  let span=end.wrapping_sub(lo);if span==0{return Err("W10_BAND_EMPTY_SPAN_GAME_PANIC")}
  let cache=r.read(data,0,8)?;let context=r.read(data,8,8)?;let now=r.read(cache,0x1b8,8)?;
  let mut seed=0;if id==47{seed=r.read(player,0x7f8,8)?}
  let cfg=r.read(context,8,8)?;let rate=r.read(cfg,0x12f8,8)?;if id==48{seed=r.read(player,0x7f8,8)?}
  let div=rate.wrapping_mul(6).max(1);let highbits=(now|div)>>32;mark(&mut metrics,"w10_band_bucket_div32_path",highbits as i128,0,"==",highbits==0);
  let seed=(now/div).wrapping_shl(32)^seed;let z=(seed^(seed>>33)^0x51ce).wrapping_mul(0x9e3779b97f4a7c15);let hash=z^(z>>29);let index=lo+hash%span;
  return Ok(done(array.wrapping_add(index.wrapping_mul(if id==47{24}else{16})),id,metrics))
 }
 let context=args[0];let cache=args[1];let player=args[2];let x=args[3];let y=args[4];let pad=args[5];
 let now=r.read(cache,0x1b8,8)?;let mode=r.read(context,0x38,1)?;let cfg=r.read(context,8,8)?;
 if mode>=9{return Err("W10_EED_MODE_OUTSIDE_PINNED_TABLE9_NOT_SOURCE_BOUND")}
 let cutoff=r.read(cfg,OFFSETS[mode as usize],8)?;mark(&mut metrics,"w10_tower_tick_before_cutoff",now as i128,cutoff as i128,"<",now<cutoff);
 if now>=cutoff{return Ok(done(0,id,metrics))}
 let team=r.read(player,0x800,8)?;if team>=2{return Err("W10_EED_TEAM_GAME_PANIC")}
 let role=r.read(player,0x890,4)?;let own=r.read(cache,0x238+team*40+role*8,8)?;if own==0{return Ok(done(0,id,metrics))}
 let enemy=1-team;let base=r.read(cache,0x1c8+enemy*8,8)?;
 let mut fixed=[0;6];for(i,offset)in[0x1d8,0x1f8,0x218,0x1e8,0x208,0x228].iter().enumerate(){fixed[i]=r.read(cache,*offset+enemy*8,8)?}
 let mut ptr=r.read(cache,0x170+enemy*32,8)?;let len=r.read(cache,0x188+enemy*32,8)?;let end=ptr.wrapping_add(len.wrapping_mul(8));
 for actor in fixed{if actor!=0&&check_tower(&mut r,actor,own,x,y,pad,&mut metrics)?{return Ok(done(1,id,metrics))}}
 while ptr!=end{let actor=r.read(ptr,0,8)?;ptr=ptr.wrapping_add(8);if check_tower(&mut r,actor,own,x,y,pad,&mut metrics)?{return Ok(done(1,id,metrics))}}
 if base!=0&&check_tower(&mut r,base,own,x,y,pad,&mut metrics)?{return Ok(done(1,id,metrics))}
 Ok(done(0,id,metrics))
}
#[cfg(test)]mod tests{include!("models_stacked10_tests.rs");}
