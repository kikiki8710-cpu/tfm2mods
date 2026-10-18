//! 0.6.4 전용 scratch 독립 모델. 게임 함수 호출/외부 쓰기0.
use crate::engine::{Memory,Evaluation,Metric};
fn rd(m:&mut dyn Memory,b:u64,o:u64,w:u8)->Result<u64,&'static str>{m.read(b.wrapping_add(o),w)}
fn mark(v:&mut Vec<Metric>,name:&'static str,l:i128,r:i128,rel:&'static str,p:bool){v.push(Metric{name,lhs:l,rhs:r,relation:rel,predicate:p,observed_branch:false});}
fn done(x:u64,mask:u64,metrics:Vec<Metric>,fields:Vec<(u64,u8,u64)>)->Evaluation{Evaluation{expected:x,mask,output_ptr:fields.first().map(|f|f.0),output_fields:fields,metrics}}
fn div(n:i64,d:i64)->Result<i64,&'static str>{n.checked_div(d).ok_or("SIGNED_DIV_PANIC")}
fn mixed(n:u64,d:u64)->Result<i64,&'static str>{if d==0{return Err("ZERO_DIV_PANIC");}if(n|d)>>32==0{Ok((n/d)as i64)}else{div(n as i64,d as i64)}}
fn pair(m:&mut dyn Memory,b:u64,o:u64)->Result<(u64,u64),&'static str>{Ok((rd(m,b,o,8)?,rd(m,b,o.wrapping_add(8),8)?))}
fn history(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
 let mut metrics=Vec::new();let cap=rd(m,a[0],0,8)?;let player=rd(m,cap,8,8)?;let own=rd(m,player,0x800,8)?;let opp=1u64.wrapping_sub(own);
 if opp>1{return Err("EA07F0_TEAM_BOUNDS");}let role=a[1];if role>=5{return Err("EA07F0_ROLE_BOUNDS");}
 let data=rd(m,cap,0,8)?;let world=rd(m,data,0,8)?;let entity=rd(m,world,0x238+opp*40+role*8,8)?;
 if entity==0{return Ok(done(0,u64::MAX,metrics,vec![]));}
 let bb=rd(m,data,0x10,8)?;let bank=bb.wrapping_add(opp*0xac8);let clock=rd(m,world,0x1b8,8)?;let cutoff=clock.saturating_sub(20);
 let mut sub=rd(m,bank,role*32+0x7f1,1)?;let(mut lo,mut hi)=pair(m,bank,role*32+0x7f8)?;let stamp=rd(m,bank,role*32+0x7e8,8)?;
 mark(&mut metrics,"w05_history_recent_after_cutoff",stamp as i128,cutoff as i128,">",stamp>cutoff);
 let tag;
 if stamp<=cutoff{tag=rd(m,bank,role*32+0x7f0,1)?;}else{
  let mut selected=None;
  for i in 0..4u64{
   let row=bank.wrapping_add(role*0xe0+0x388+i*56);let time=rd(m,row,0x18,8)?;let before=rd(m,row,0,1)?;let after=rd(m,row,0x20,1)?;
   if time==0&&before==255&&after==255{continue;}
   mark(&mut metrics,"w05_history_row_after_cutoff",time as i128,cutoff as i128,">",time>cutoff);
   if time>cutoff{if selected.is_none(){let su=rd(m,row,1,1)?;let(l,h)=pair(m,row,8)?;selected=Some((before,su,l,h));}break;}
   let su=rd(m,row,0x21,1)?;let(l,h)=pair(m,row,0x28)?;selected=Some((after,su,l,h));
  }
  let choice=if let Some(c)=selected{c}else{let b=bank.wrapping_add(0x310+role*24);let t=rd(m,b,0,1)?;let su=rd(m,b,1,1)?;let(l,h)=pair(m,b,8)?;(t,su,l,h)};
  tag=choice.0;sub=choice.1;lo=choice.2;hi=choice.3;
 }
 if tag==255{return Ok(done(0,u64::MAX,metrics,vec![]));}
 let request=rd(m,cap,0x18,8)?;let reqtag=rd(m,request,0,1)?;
 mark(&mut metrics,"w05_history_tag_equal",tag as i128,reqtag as i128,"==",tag==reqtag);
 if tag!=reqtag{return Ok(done(0,u64::MAX,metrics,vec![]));}
 if tag<=5{
  let reqsub=rd(m,request,1,1)?;let reqlo=rd(m,request,8,8)?;
  let matched=match tag{0=>sub==reqsub,1=>sub==reqsub&&lo==reqlo,4=>lo==reqlo,5=>{if lo&1!=0{let h=rd(m,request,0x10,8)?;hi==h&&reqlo&1!=0}else{reqlo&1==0}},_=>true};
  if !matched{return Ok(done(0,u64::MAX,metrics,vec![]));}
 }
 let x=rd(m,entity,0x658,8)?;let y=rd(m,entity,0x660,8)?;let xr=rd(m,cap,0x20,8)?;let yr=rd(m,cap,0x28,8)?;let cx=rd(m,xr,0,8)?;let cy=rd(m,yr,0,8)?;
 let dx=x.wrapping_sub(cx);let dy=y.wrapping_sub(cy);let d2=dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));let inside=d2<62_500_000_001;
 mark(&mut metrics,"w05_history_distance_squared",d2 as i128,62_500_000_001,"<",inside);
 Ok(done(if inside{entity}else{0},u64::MAX,metrics,vec![]))
}
fn economy(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
 let mut metrics=Vec::new();if rd(m,a[0],0x480,1)?!=0{return Ok(done(0,u64::MAX,metrics,vec![]));}
 let maximum=rd(m,a[0],0x620,8)?;let hp=rd(m,a[0],0x668,8)?;let den=maximum.max(1);let damage=a[1];let scale=a[2];
 let remain=(hp.wrapping_sub(damage)as i64).max(0)as u64;let q=mixed(remain.wrapping_mul(100),den)?;
 mark(&mut metrics,"w05_economy_remaining_percent_lt45",q as i128,45,"<",q<45);
 let mut result=0u64;
 if q<45{result=div(45u64.wrapping_sub(q as u64).wrapping_mul(scale)as i64,60)?as u64;
  mark(&mut metrics,"w05_economy_remaining_percent_lt25",q as i128,25,"<",q<25);
  if q<25{result=result.wrapping_add(div(scale as i64,5)?as u64);}}
 let damagepct=mixed(damage.wrapping_mul(100),den)?;
 mark(&mut metrics,"w05_economy_damage_percent_gt11",damagepct as i128,11,">",damagepct>11);
 if damagepct>11{let ratio=mixed(scale.wrapping_mul(damage),hp.max(1))?;result=result.wrapping_add(div(ratio,2)?as u64);}
 Ok(done(result,u64::MAX,metrics,vec![]))
}
fn noalias(m:&mut dyn Memory,b:u64,o:u64,w:u8,cursor:u64)->Result<u64,&'static str>{
 let addr=b.wrapping_add(o);let end=addr.checked_add(w as u64).ok_or("ADDRESS_SPAN_WRAP")?;let ce=cursor.checked_add(8).ok_or("ADDRESS_SPAN_WRAP")?;
 if addr<ce&&cursor<end{return Err("CURSOR_INPUT_ALIAS");}m.read(addr,w)
}
fn cursor(pid:usize,a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
 let mut metrics=Vec::new();let c=a[0];let begin=rd(m,c,0,8)?;let end=rd(m,c,8,8)?;
 if(if pid==32{begin==end}else{begin>=end}){return Ok(done(0,255,metrics,vec![]));}
 if pid==32{if end<begin||(end-begin)%8!=0||(end-begin)/8>32{return Err("CURSOR_ARRAY_CAPTURE_LIMIT32");}}else if end-begin>32{return Err("CURSOR_RANGE_CAPTURE_LIMIT32");}
 let cap=a[1];let teamref=noalias(m,cap,0x18,8,c)?;
 let mut anchor=0;let(excludedref,world,bb,line)=if pid==32{let w=noalias(m,cap,8,8,c)?;let ex=noalias(m,cap,0,8,c)?;let b=noalias(m,cap,0x10,8,c)?;let l=noalias(m,cap,0x20,8,c)?;(ex,w,b,l)}else if pid==34{let b=noalias(m,cap,0x10,8,c)?;let l=noalias(m,cap,0x20,8,c)?;let ex=noalias(m,cap,0,8,c)?;let w=noalias(m,cap,8,8,c)?;anchor=noalias(m,cap,0x28,8,c)?;(ex,w,b,l)}else{let ex=noalias(m,cap,0,8,c)?;let w=noalias(m,cap,8,8,c)?;let b=noalias(m,cap,0x10,8,c)?;let l=noalias(m,cap,0x20,8,c)?;(ex,w,b,l)};
 let excluded=noalias(m,excludedref,0,8,c)?;let mut next=begin;let mut found=false;
 while next!=end{
  let key=if pid==32{noalias(m,next,0,8,c)?}else{next};next=next.wrapping_add(if pid==32{8}else{1});if pid==35||pid==36{let fixed=key==4;mark(&mut metrics,"w05_cursor_skip_role4",key as i128,4,"==",fixed);if fixed{continue;}}let skip=key==excluded;
  mark(&mut metrics,"w05_cursor_excluded_equal",key as i128,excluded as i128,"==",skip);if skip{continue;}
  let team=noalias(m,teamref,0,8,c)?;if team>1{return Err("CURSOR_TEAM_BOUNDS_PANIC_WRITES_NEXT");}if key>=5{return Err("CURSOR_ID_BOUNDS_PANIC_WRITES_NEXT");}
  if pid==34{if noalias(m,bb,team*0xac8+key*32+0x7f0,1,c)?!=0{continue;}let wanted=noalias(m,line,0,1,c)?;let actual=noalias(m,bb,team*0xac8+key*32+0x7f1,1,c)?;let equal=actual==wanted;mark(&mut metrics,"w05_cursor_line_equal",actual as i128,wanted as i128,"==",equal);if !equal{continue;}}
  let entity=noalias(m,world,0x238+team*40+key*8,8,c)?;mark(&mut metrics,"w05_cursor_entity_present",entity as i128,0,"!=",entity!=0);if entity==0{continue;}
  if pid==34{let x=noalias(m,entity,0x658,8,c)?;let y=noalias(m,entity,0x660,8,c)?;let ax=noalias(m,anchor,0x658,8,c)?;let ay=noalias(m,anchor,0x660,8,c)?;let dx=x.wrapping_sub(ax);let dy=y.wrapping_sub(ay);let distance=dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))>>8;let near=distance<87_890_625;mark(&mut metrics,"w05_cursor_near_distance_shift8",distance as i128,87_890_625,"<",near);if near{found=true;break;}continue;}
  if noalias(m,bb,team*0xac8+key*32+0x7f0,1,c)?!=0{continue;}
  let actual=noalias(m,bb,team*0xac8+key*32+0x7f1,1,c)?;let wanted=noalias(m,line,if pid==32{0x11c}else{0},1,c)?;let equal=actual==wanted;
  mark(&mut metrics,"w05_cursor_line_equal",actual as i128,wanted as i128,"==",equal);if equal{found=true;break;}
 }
 Ok(done(found as u64,255,metrics,vec![(c,8,next)]))
}
pub fn evaluate(id:usize,args:[u64;4],memory:&mut dyn Memory)->Result<Evaluation,&'static str>{match id{27=>history(args,memory),28=>economy(args,memory),32|33|34|35|36=>cursor(id,args,memory),_=>Err("MODEL_NOT_OWNED")}}

#[cfg(test)]
#[path="models_wave05_direct_tests.rs"]
mod boundary_tests;
