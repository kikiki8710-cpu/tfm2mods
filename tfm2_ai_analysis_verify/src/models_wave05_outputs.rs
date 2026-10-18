//! wave05 출력3 scratch. 게임 호출/출력 쓰기 없음. 실제 src로 통합하기 전 검수 필요.
use crate::engine::{Evaluation,Memory,Metric};
pub const FIRST_MODEL_ID:usize=29;
pub const MODEL_COUNT:usize=3;
const ADDRESS_END:u64=1<<48;
fn range(a:u64,n:u64)->Result<(u64,u64),&'static str>{
    let e=a.checked_add(n).ok_or("W05_ADDRESS_OVERFLOW")?;
    if a<0x10000||e>ADDRESS_END||n==0{return Err("W05_ADDRESS_RANGE");} Ok((a,e))
}
fn overlap(a:(u64,u64),b:(u64,u64))->bool{a.0<b.1&&b.0<a.1}
struct Reader<'a>{memory:&'a mut dyn Memory,output:(u64,u64)}
impl Reader<'_>{
    fn read(&mut self,b:u64,o:u64,w:u8)->Result<u64,&'static str>{
        if !matches!(w,1|4|8){return Err("W05_READ_WIDTH");}
        let a=b.wrapping_add(o);let input=range(a,w as u64)?;
        if overlap(input,self.output){return Err("W05_OUTPUT_INPUT_ALIAS");}
        let x=self.memory.read(a,w)?;
        Ok(match w{1=>x&255,4=>x&0xffff_ffff,_=>x})
    }
}
fn metric(name:&'static str,lhs:i128,rhs:i128,relation:&'static str,predicate:bool)->Metric{
    Metric{name,lhs,rhs,relation,predicate,observed_branch:false}
}
fn finish(base:u64,values:&[u64],metrics:Vec<Metric>,rax:u64)->Evaluation{
    Evaluation{expected:rax,mask:u64::MAX,output_ptr:Some(base),
        output_fields:values.iter().enumerate().map(|(j,&x)|(base+j as u64*8,8,x)).collect(),metrics}
}
pub fn evaluate(id:usize,args:[u64;4],memory:&mut dyn Memory)->Result<Evaluation,&'static str>{
    match id{29=>population(args,memory),30|31=>sort4(id,args,memory),_=>Err("W05_OUTPUT_UNSUPPORTED_ID")}
}
fn population(a:[u64;4],memory:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let out=range(a[0],24)?;if a[1]>1{return Err("DBF730_TEAM_BOUNDS_PANIC");}
    let mut r=Reader{memory,output:out};let mut own=[0u64;5];
    for(j,x)in own.iter_mut().enumerate(){*x=r.read(a[2],0x238+a[1]*40+j as u64*8,8)?;}
    let mut healthy=0u64;let mut metrics=Vec::new();
    for entity in own.iter().copied().filter(|&x|x!=0){
        let maximum=r.read(entity,0x620,8)?;if maximum==0{return Err("DBF730_DIVISOR_ZERO_GAME_PANIC");}
        let hp=r.read(entity,0x668,8)?;let percent=hp.wrapping_mul(100)/maximum;
        let good=percent>=40;healthy+=u64::from(good);
        metrics.push(metric("w05_healthy_hp_percent_ge40",percent as i128,40,">=",good));
    }
    // 실제 opponent 읽기 순서: slot1,0,2,3,4. 모델은 값을 초기 출력에서 얻지 않는다.
    let mut opponent=0;
    for j in [1u64,0,2,3,4]{opponent+=u64::from(r.read(a[2],0x238+(1-a[1])*40+j*8,8)?!=0);}
    let present=own.iter().filter(|&&x|x!=0).count()as u64;
    Ok(finish(a[0],&[present,healthy,opponent],metrics,present))
}
fn sort4(id:usize,a:[u64;4],memory:&mut dyn Memory)->Result<Evaluation,&'static str>{
    let output=range(a[1],32)?;if overlap(range(a[0],32)?,output){return Err("W05_OUTPUT_SOURCE4_ALIAS");}
    let mut r=Reader{memory,output};let mut values=[0u64;4];
    // 고정 source4를 먼저 캡처하는 관측 모델. 원문 late reload는 비alias domain에서 동치.
    for(j,v)in values.iter_mut().enumerate(){*v=r.read(a[0],j as u64*8,8)?;}
    let(len,data)=if id==30{(a[3],a[2])}else{(r.read(a[2],0x10,8)?,r.read(a[2],8,8)?)};
    if values.iter().any(|&x|x>=len){return Err("W05_SORT_INDEX_BOUNDS_PANIC");}
    let mut metrics=Vec::new();
    // 5비교 정렬망. signed CMP를 wrap 차이의 부호로 대체하지 않는다.
    for(i,j)in [(0,1),(2,3),(0,2),(1,3),(1,2)]{
        let left=values[i];let right=values[j];
        let right_first=if id==30{
            let x=r.read(data,left.wrapping_mul(8),8)?as i64;
            let y=r.read(data,right.wrapping_mul(8),8)?as i64;
            metrics.push(metric("w05_sort_score_left_lt_right",x as i128,y as i128,"<",x<y));
            if x==y{metrics.push(metric("w05_sort_unsigned_index_tie",right as i128,left as i128,"<",right<left));right<left}else{x<y}
        }else{
            let lc=r.read(data,left.wrapping_mul(400).wrapping_add(0x188),1)?;
            let rc=r.read(data,right.wrapping_mul(400).wrapping_add(0x188),1)?;
            let lp=lc>=5;let rp=rc>=5;
            metrics.push(metric("w05_record_class_partition",lp as i128,rp as i128,"!=",lp!=rp));
            if lp!=rp{lp}else{
                let x=r.read(data,left.wrapping_mul(400).wrapping_add(0x108),4)?as u32 as i32;
                let y=r.read(data,right.wrapping_mul(400).wrapping_add(0x108),4)?as u32 as i32;
                metrics.push(metric("w05_record_metric_left_lt_right",x as i128,y as i128,"<",x<y));
                if x==y{let ri=right as i64;let li=left as i64;metrics.push(metric("w05_record_signed_index_tie",ri as i128,li as i128,"<",ri<li));ri<li}else{x<y}
            }
        };
        if right_first{values.swap(i,j);}
    }
    Ok(finish(a[1],&values,metrics,values[3]))
}

#[cfg(test)]
#[path="models_wave05_outputs_tests.rs"]
mod tests;
