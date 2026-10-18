//! wave04 scratch. root integrate 전까지 M/src에복사하지않는다.
//! 정상helper0;panic/읽기실패/명시capturebudget초과는Err. 실제게임·native미검증.
use crate::engine::{Evaluation,Memory,Metric};
pub const FIRST_MODEL_ID:usize=19;
pub const MODEL_COUNT:usize=8;
fn r(m:&mut dyn Memory,b:u64,o:u64,w:u8)->Result<u64,&'static str>{
    let x=m.read(b.wrapping_add(o),w)?;
    match w{1=>Ok(x&255),4=>Ok(x&0xffff_ffff),8=>Ok(x),_=>Err("W04_READ_WIDTH")}
}
fn met(name:&'static str,a:i128,b:i128,op:&'static str,p:bool)->Metric{Metric{name,lhs:a,rhs:b,relation:op,predicate:p,observed_branch:false}}
fn out(x:u64,mask:u64,metrics:Vec<Metric>)->Evaluation{Evaluation{expected:x,mask,metrics,output_ptr:None,output_fields:Vec::new()}}
fn boolean(x:bool,metrics:Vec<Metric>)->Evaluation{out(u64::from(x),255,metrics)}
pub fn evaluate(id:usize,a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{match id{
    19=>lookup(a,m),20=>forecast(a,m),21=>count_except(a,m),22=>count_opponent(a,m),
    23=>pressure(a,m),24=>open_slot(a,m),25=>candidate_hp(a,m),26=>geometry_count(a,m),
    _=>Err("W04_UNSUPPORTED_ID")}}

fn lookup(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let n=r(m,a[0],0x840,8)?;let mut metrics=vec![met("w04_lookup_count_zero",n as i128,0,"==",n==0)];
    if n==0{return Ok(out(0,u64::MAX,metrics));}
    for i in 0..12u64{
        let id=r(m,a[0],i*0xb0+0x90,8)?;let same=id==a[1];
        metrics.push(met("w04_lookup_id_equal",id as i128,a[1]as i128,"==",same));
        if same{return Ok(out(a[0].wrapping_add(i*0xb0),u64::MAX,metrics));}
        let ended=n==i+1;metrics.push(met("w04_lookup_end_equals_count",n as i128,(i+1)as i128,"==",ended));
        if ended{return Ok(out(0,u64::MAX,metrics));}
    }
    Err("E1B430_BOUNDS_PANIC_AFTER_TWELFTH_MISS")
}
fn forecast(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let t=a[1];if t==0{return Ok(out(r(m,a[0],0x98,8)?,u64::MAX,Vec::new()));}
    let k=t/5;let n=r(m,a[0],0xa0,8)?;
    // 게임의upfrontguard가아닌관측domain. n19/t1등원문정상팔도이번모델은제외한다.
    if n>18{return Err("E1EDE0_CAPTURE_COUNT_ABOVE18");}
    let mut metrics=vec![met("w04_forecast_index_ge_count",k as i128,n as i128,">=",k>=n)];
    if k>=n{return Ok(out(r(m,a[0],if n==0{0x98}else{(n-1)*8},8)?,u64::MAX,metrics));}
    metrics.push(met("w04_forecast_time_lt5",t as i128,5,"<",t<5));
    let (before,after,rem)=if t<5{let next=r(m,a[0],0,8)?;let prev=r(m,a[0],0x98,8)?;(prev,next,t)}
        else{if k==0||k-1>17||t>=90{return Err("E1EDE0_INTERPOLATION_PANIC_DOMAIN");}
            let prev=r(m,a[0],(k-1)*8,8)?;let next=r(m,a[0],k*8,8)?;(prev,next,t-k*5)};
    let product=after.wrapping_sub(before).wrapping_mul(rem)as i64;
    Ok(out(before.wrapping_add((product/5)as u64),u64::MAX,metrics))
}
fn count_except(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let start=r(m,a[0],0x18,8)?;let end=r(m,a[0],0x20,8)?;
    if start>=end{return Ok(out(0,u64::MAX,Vec::new()));}
    if end-start>32{return Err("D81CE0_CAPTURE_RANGE_ABOVE32");}
    let team_ref=r(m,a[0],0x10,8)?;let excluded_ref=r(m,a[0],0,8)?;let world=r(m,a[0],8,8)?;
    let excluded=r(m,excluded_ref,0,8)?;let mut count=0u64;let mut metrics=Vec::new();
    for id in start..end{
        let skip=id==excluded;metrics.push(met("w04_count_excluded_id",id as i128,excluded as i128,"==",skip));
        if skip{continue;}
        let team=r(m,team_ref,0,8)?;if team>1{return Err("D81CE0_TEAM_BOUNDS_PANIC");}
        if id>=5{return Err("D81CE0_ROLE_BOUNDS_PANIC");}
        let ptr=r(m,world,0x238+team*40+id*8,8)?;let present=ptr!=0;
        metrics.push(met("w04_count_slot_present",ptr as i128,0,"!=",present));count+=u64::from(present);
    }
    Ok(out(count,u64::MAX,metrics))
}
fn count_opponent(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let start=r(m,a[0],0x10,8)?;let end=r(m,a[0],0x18,8)?;
    if start>=end{return Ok(out(0,u64::MAX,Vec::new()));}
    let team_ref=r(m,a[0],8,8)?;let team=1u64.wrapping_sub(r(m,team_ref,0,8)?);
    if team>1{return Err("D81BD0_TEAM_BOUNDS_PANIC");}
    if start>=5{return Err("D81BD0_START_BOUNDS_PANIC");}
    if end>5{return Err("D81BD0_CAPTURE_RANGE_END_ABOVE5");}
    let world=r(m,a[0],0,8)?;let mut count=0;let mut metrics=Vec::new();
    for id in start..end{let ptr=r(m,world,0x238+team*40+id*8,8)?;let present=ptr!=0;
        metrics.push(met("w04_opponent_slot_present",ptr as i128,0,"!=",present));count+=u64::from(present);}
    Ok(out(count,u64::MAX,metrics))
}
fn pressure(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let context=r(m,a[1],8,8)?;let mode=r(m,context,0x38,1)?;let line=a[2]&255;
    let slot=if line==0{0}else if line==1{1}else{2};let masks=[0x185u64,0x1b1,0x1ab];
    let allowed=mode<9&&((masks[slot]>>mode)&1)!=0;
    let mut metrics=vec![met("w04_pressure_mode_lt9",mode as i128,9,"<",mode<9),
        met("w04_pressure_mode_allowed",u64::from(allowed)as i128,1,"==",allowed)];
    if !allowed{return Ok(boolean(false,metrics));}
    let team=r(m,a[0],0x800,8)?;if team>=2{return Err("E54190_TEAM_BOUNDS_PANIC");}
    let bb=r(m,a[1],0x10,8)?;let row=bb.wrapping_add(team*0xac8+slot as u64*0x28);
    let progress=r(m,row,0x10,8)?as i64;
    metrics.push(met("w04_pressure_progress_le_minus3001",progress as i128,-3001,"<=",progress<=-3001));
    if progress> -3001{return Ok(boolean(false,metrics));}
    let count=r(m,row,0x20,4)?as u32 as i32;
    metrics.push(met("w04_pressure_count_lt_minus3",count as i128,-3,"<",count< -3));
    Ok(boolean(count< -3,metrics))
}
// .rdata 3B69980/84/88의DWORD=[0,2,3]. loaded자료동일성은별도실행전제.
fn role(id:u64)->Option<u64>{match id{0=>Some(0),2=>Some(2),3=>Some(3),_=>None}}
fn allowed_role(role:u64,mode:u64)->Option<u64>{
    let (mask,offset)=match role{0=>(0x185u64,0),2=>(0x1b1,0x20),3=>(0x1ab,0x40),_=>return None};
    if mode<9&&((mask>>mode)&1)!=0{Some(offset)}else{None}
}
fn open_slot(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let outer=r(m,a[0],0,8)?;let capture=r(m,outer,0,8)?;let team_ref=r(m,capture,8,8)?;
    let team=r(m,team_ref,0,8)?;if team>1{return Err("FFA970_TEAM_BOUNDS_PANIC");}
    let id=r(m,a[1],0,8)?;if id>=5{return Err("FFA970_ROLE_BOUNDS_PANIC");}
    let data=r(m,capture,0,8)?;let world=r(m,data,0,8)?;let ptr=r(m,world,0x238+team*40+id*8,8)?;
    if ptr==0{return Ok(boolean(false,Vec::new()));}
    let which=match role(id){Some(x)=>x,None=>return Ok(boolean(false,Vec::new()))};
    let context=r(m,data,8,8)?;let mode=r(m,context,0x38,1)?;let offset=match allowed_role(which,mode){Some(x)=>x,None=>return Ok(boolean(false,Vec::new()))};
    let first=r(m,world,0x1d8+offset,8)?;
    let mut metrics=vec![met("w04_lane_open_first_absent",first as i128,0,"==",first==0)];
    if first==0{return Ok(boolean(true,metrics));}
    let second=r(m,world,0x1e0+offset,8)?;metrics.push(met("w04_lane_open_second_absent",second as i128,0,"==",second==0));
    Ok(boolean(second==0,metrics))
}
fn candidate_hp(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let id=a[1];if id>4{return Err("FD4260_ID_BOUNDS_PANIC");}
    let wrapper=r(m,a[0],0,8)?;let excluded=r(m,wrapper,0,8)?;
    if r(m,excluded,id,1)?!=0{return Ok(boolean(false,Vec::new()));}
    let data=r(m,wrapper,8,8)?;
    let which=match role(id){Some(x)=>x,None=>return Ok(boolean(false,Vec::new()))};
    let context=r(m,data,8,8)?;let mode=r(m,context,0x38,1)?;
    let offset=match allowed_role(which,mode){Some(x)=>x,None=>return Ok(boolean(false,Vec::new()))};
    let world_a=r(m,data,0,8)?;
    if r(m,world_a,0x1d8+offset,8)?!=0&&r(m,world_a,0x1e0+offset,8)?!=0{return Ok(boolean(false,Vec::new()));}
    let team_a_ref=r(m,wrapper,0x10,8)?;let team_a=r(m,team_a_ref,0,8)?;
    if team_a>=2{return Err("FD4260_WRAPPER_TEAM_BOUNDS_PANIC");}
    if r(m,world_a,0x238+team_a*40+id*8,8)?==0{return Ok(boolean(false,Vec::new()));}
    let team_b_ref=r(m,a[0],0x10,8)?;let team_b=r(m,team_b_ref,0,8)?;
    if team_b>=2{return Err("FD4260_CAPTURE_TEAM_BOUNDS_PANIC");}
    let world_b=r(m,a[0],8,8)?;let entity=r(m,world_b,0x238+team_b*40+id*8,8)?;
    if entity==0{return Ok(boolean(false,Vec::new()));}
    if r(m,entity,0x68,4)?==15{
        // MOV RCX,[RAX+70]는8B읽기지만CMP ECX,1은low32비교다.
        if (r(m,entity,0x70,8)?as u32)==1{return Ok(boolean(false,Vec::new()));}
    }
    let maximum=r(m,entity,0x620,8)?.max(1);
    let percent=r(m,entity,0x668,8)?.wrapping_mul(100)/maximum;
    Ok(boolean(percent>=30,vec![met("w04_candidate_hp_percent_ge30",percent as i128,30,">=",percent>=30)]))
}
fn geometry_count(a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let begin=r(m,a[0],0,8)?;let end=r(m,a[0],8,8)?;
    if begin==end{return Ok(out(0,u64::MAX,Vec::new()));}
    if end<begin||((end-begin)&7)!=0{return Err("DC0D80_INVALID_CAPTURE_SLICE");}
    let n=(end-begin)/8;if n>16{return Err("DC0D80_CAPTURE_LENGTH_ABOVE16");}
    let event_ref=r(m,a[0],0x10,8)?;let context=r(m,a[0],0x18,8)?;
    let event=r(m,event_ref,0,1)?;let settings=r(m,context,8,8)?;
    let mut count=0;let mut metrics=Vec::new();
    for i in 0..n{
        let entity=r(m,begin,i*8,8)?;if entity==0{continue;}
        let maximum=r(m,entity,0x620,8)?;if maximum==0{return Err("DC0D80_DIVISOR_ZERO_GAME_PANIC_DOMAIN");}
        if event!=4&&event!=5{continue;}
        let percent=r(m,entity,0x668,8)?.wrapping_mul(100)/maximum;
        metrics.push(met("w04_geometry_hp_percent_ge50",percent as i128,50,">=",percent>=50));if percent<50{continue;}
        let x=r(m,entity,0x658,8)?;
        if event==4{
            metrics.push(met("w04_geometry_tag4_x_le192000",x as i128,192000,"<=",x<=192000));
            if x<=192000{count+=1;continue;}
            let height=r(m,settings,0x12c0,8)?;let q=height.wrapping_sub(r(m,entity,0x660,8)?);
            metrics.push(met("w04_geometry_tag4_x_le_q",x as i128,q as i128,"<=",x<=q));if x<=q{count+=1;continue;}
            let cut=r(m,settings,0x12b8,8)?.wrapping_sub(192000);
            metrics.push(met("w04_geometry_tag4_q_ge_cut",q as i128,cut as i128,">=",q>=cut));if q>=cut{count+=1;continue;}
            let gap=x.wrapping_sub(q);metrics.push(met("w04_geometry_tag4_gap_lt64000",gap as i128,64000,"<",gap<64000));count+=u64::from(gap<64000);
        }else{
            let height=r(m,settings,0x12c0,8)?;let q=height.wrapping_sub(r(m,entity,0x660,8)?);
            metrics.push(met("w04_geometry_tag5_q_le_x",q as i128,x as i128,"<=",q<=x));
            metrics.push(met("w04_geometry_tag5_q_le192000",q as i128,192000,"<=",q<=192000));
            if q<=x||q<=192000{count+=1;continue;}
            let cut=r(m,settings,0x12b8,8)?.wrapping_sub(192000);
            metrics.push(met("w04_geometry_tag5_x_ge_cut",x as i128,cut as i128,">=",x>=cut));if x>=cut{count+=1;continue;}
            let gap=q.wrapping_sub(x);metrics.push(met("w04_geometry_tag5_gap_lt64000",gap as i128,64000,"<",gap<64000));count+=u64::from(gap<64000);
        }
    }
    Ok(out(count,u64::MAX,metrics))
}

#[cfg(test)]
#[path="models_wave04_tests.rs"]
mod tests;
