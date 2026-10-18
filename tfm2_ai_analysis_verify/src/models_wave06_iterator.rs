//! id39 scratch. 게임/원본 호출·쓰기 없음. 관측 cap은 원문의 배열 상한이 아니다.
use crate::engine::{Evaluation,Memory,Metric};
pub const MODEL_ID:usize=39;
pub const SCAN_CAP:u64=16;
fn span(a:u64,n:u64)->Result<(u64,u64),&'static str>{
    let b=a.checked_add(n).ok_or("W06_ADDRESS_OVERFLOW")?;
    if a<0x10000||n==0||b>1u64<<48{return Err("W06_ADDRESS_RANGE");} Ok((a,b))
}
fn overlap(a:(u64,u64),b:(u64,u64))->bool{a.0<b.1&&b.0<a.1}
struct Reader<'a>{m:&'a mut dyn Memory,out:(u64,u64),cursor:(u64,u64)}
impl Reader<'_>{
    fn read(&mut self,b:u64,o:u64,initial_cursor:bool)->Result<u64,&'static str>{
        let p=b.wrapping_add(o);let s=span(p,8)?;
        if overlap(s,self.out)||(!initial_cursor&&overlap(s,self.cursor)){return Err("W06_ITERATOR_INPUT_OUTPUT_ALIAS");}
        self.m.read(p,8)
    }
}
fn metric(name:&'static str,lhs:u64,rhs:u64,relation:&'static str,p:bool)->Metric{
    Metric{name,lhs:lhs as i128,rhs:rhs as i128,relation,predicate:p,observed_branch:false}
}
pub fn evaluate(id:usize,a:[u64;4],m:&mut dyn Memory)->Result<Evaluation,&'static str>{
    if id!=39{return Err("W06_ITERATOR_UNSUPPORTED_ID");}
    let out=span(a[0],32)?;let cursor=span(a[1].checked_add(8).ok_or("W06_ADDRESS_OVERFLOW")?,8)?;
    if overlap(out,cursor){return Err("W06_OUT_CURSOR_ALIAS");}
    let mut r=Reader{m,out,cursor};let start=r.read(a[1],8,true)?;let end=r.read(a[1],16,false)?;
    let mut fields=Vec::new();let mut metrics=Vec::new();
    if start==end{
        fields.push((a[0],8,0));
    }else{
        if start>end{return Err("W06_NONMONOTONIC_CURSOR_DOMAIN");}
        let checkroot=r.read(a[1],0,false)?;let xref=r.read(a[2],0,false)?;let yref=r.read(a[2],8,false)?;
        let mut current=start;let mut scanned=0;
        loop{
            if scanned>=SCAN_CAP{return Err("W06_ITERATOR_SCAN_CAPTURE_CAP");}scanned+=1;
            let offset=current.wrapping_mul(16);let y=r.read(a[1],offset.wrapping_add(32),false)?;
            if y>29{return Err("W06_Y_BOUNDS_PANIC_WITH_CURSOR_WRITE");}
            let x=r.read(a[1],offset.wrapping_add(24),false)?;
            if x>=30{return Err("W06_X_BOUNDS_PANIC_WITH_CURSOR_WRITE");}
            let cell=y*240+x*8+0x1c98;let occupied=r.read(checkroot,cell,false)?;
            metrics.push(metric("w06_cell_occupied",occupied,0,"!=",occupied!=0));
            if occupied!=0{
                let cx=x*32000+16000;let cy=y*32000+16000;
                let rx=r.read(xref,0,false)?;let ry=r.read(yref,0,false)?;
                let dx=cx.wrapping_sub(rx);let dy=cy.wrapping_sub(ry);
                let d2=dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));let near=d2<=22500000000;
                metrics.push(metric("w06_cell_radius_squared_inclusive",d2,22500000000,"<=",near));
                if near{
                    let next=current+1;
                    // 원문은 cursor를 먼저 쓴 뒤 rootOutput을 읽는다. 그 alias는 위 Reader가 거절한다.
                    let outputroot=r.read(a[1],0x488,false)?;let id=r.read(outputroot,cell,false)?;
                    fields.extend([(cursor.0,8,next),(a[0]+8,8,id),(a[0]+16,8,cx),(a[0]+24,8,cy),(a[0],8,1)]);break;
                }
            }
            current+=1;
            if current==end{fields.extend([(cursor.0,8,end),(a[0],8,0)]);break;}
        }
    }
    Ok(Evaluation{expected:a[0],mask:u64::MAX,output_ptr:Some(a[0]),output_fields:fields,metrics})
}
#[cfg(test)]
#[path="models_wave06_iterator_tests.rs"]
mod tests;
