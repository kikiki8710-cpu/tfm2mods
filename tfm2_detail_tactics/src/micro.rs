//! 상세전술 적용 장치. 코드 설치는 한 번, 값은 각 판단의 스택에 격리한다.
use crate::{platform, sites, tactics};
use std::sync::{OnceLock, atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::{Relaxed, Acquire, Release}}};
#[derive(Clone, Copy, Debug)]
pub enum Src { None, Reg(u8), Stack(i32), Mem(u8,i32), StackMem(i32,i32) }
#[derive(Clone, Copy, Debug)]
pub enum Tf { Add(i64), Sq1, Map { on:i64, off:i64 } }
pub struct Field { pub off:u8, pub width:u8, pub tf:Tf, pub max:i64 }
pub struct Branch { pub off:u8, pub len:u8, pub cc:Option<u8>, pub target_rva:usize }
pub struct Patch {
    pub name:&'static str, pub tactic:usize, pub rva:usize, pub win:&'static [u8],
    pub fields:&'static [Field], pub branches:&'static [Branch], pub src:Src, pub team_chk:Src,
    pub first_len:usize, pub dynamic:&'static [u8],
}
const CAP:usize=96;
static ENABLED:[AtomicBool;tactics::N]=[const{AtomicBool::new(false)};tactics::N];
static ORIGINAL:[AtomicUsize;CAP]=[const{AtomicUsize::new(0)};CAP];
static DYNAMIC:[AtomicUsize;CAP]=[const{AtomicUsize::new(0)};CAP];
pub static HITS:[AtomicU64;CAP]=[const{AtomicU64::new(0)};CAP];
pub static MINE:[AtomicU64;CAP]=[const{AtomicU64::new(0)};CAP];
pub static APPLIED:[AtomicU64;CAP]=[const{AtomicU64::new(0)};CAP];
static BAD_SUBJECT:[AtomicU64;CAP]=[const{AtomicU64::new(0)};CAP];
static SAMPLE_AT:AtomicUsize=AtomicUsize::new(0);
struct Sample { athlete:AtomicU64, team:AtomicU64, pos:AtomicU64, site:AtomicUsize }
static SAMPLES:[Sample;16]=[const{Sample{athlete:AtomicU64::new(0),team:AtomicU64::new(0),pos:AtomicU64::new(0),site:AtomicUsize::new(0)}};16];
const ORDER:[u8;15]=[0,1,2,3,5,6,7,8,9,10,11,12,13,14,15];
#[repr(C)]
struct Frame { xmm:[u8;96], regs:[u64;15], flags:u64, value:u64, target:u64 }
impl Frame {
    fn reg(&self,r:u8)->Option<u64>{
        if r==4 { return Some(self as *const Self as u64 + 240); }
        ORDER.iter().position(|v|*v==r).map(|i|self.regs[14-i])
    }
    fn src(&self,s:Src)->Option<u64>{
        fn at(base:u64,d:i32)->Option<u64>{platform::fast_read_u64(base.wrapping_add_signed(d as i64) as usize)}
        match s { Src::None=>None, Src::Reg(r)=>self.reg(r), Src::Stack(d)=>at(self.reg(4)?,d),
            Src::Mem(r,d)=>at(self.reg(r)?,d), Src::StackMem(a,b)=>at(at(self.reg(4)?,a)?,b) }
    }
}
fn transform(f:&Field,v:i64)->Option<u64>{
    let n=match f.tf { Tf::Add(d)=>v.checked_add(d)?, Tf::Sq1=>v.checked_mul(v)?.checked_add(1)?,
        Tf::Map{on,off}=>if v==0 {off}else{on} };
    if n<0 || n>f.max {None}else{Some(n as u64)}
}
unsafe extern "C" fn dispatch(index:usize,frame:*mut Frame)->usize {
    let fallback=ORIGINAL.get(index).map(|v|v.load(Acquire)).unwrap_or(0);
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let p=sites::PATCHES.get(index)?;
        HITS[index].fetch_add(1,Relaxed);
        let f=&mut *frame;
        let player=f.src(p.src)? as usize;
        let athlete=platform::fast_read_u64(player.wrapping_add(0x810))?;
        let team=platform::fast_read_u64(player.wrapping_add(0x820))?;
        let pos=platform::fast_read_u32(player.wrapping_add(0x8b0))? as usize;
        if team>1 || pos>4 { BAD_SUBJECT[index].fetch_add(1,Relaxed); return None; }
        if !matches!(p.team_chk,Src::None) && f.src(p.team_chk)!=Some(team) {
            BAD_SUBJECT[index].fetch_add(1,Relaxed); return None;
        }
        if let Ok(k)=SAMPLE_AT.fetch_update(Relaxed,Relaxed,|v|if v<SAMPLES.len(){Some(v+1)}else{None}) {
            let s=&SAMPLES[k]; s.team.store(team,Relaxed);s.pos.store(pos as u64,Relaxed);s.site.store(index,Relaxed);s.athlete.store(athlete,Release);
        }
        if !tactics::is_my_athlete(athlete) {return None;}
        MINE[index].fetch_add(1,Relaxed);
        if !ENABLED[p.tactic].load(Acquire) {return None;}
        let value=tactics::value_for(p.tactic,athlete,pos)?;
        f.value=transform(p.fields.first()?,value)?;
        APPLIED[index].fetch_add(1,Relaxed);
        Some(DYNAMIC[index].load(Acquire))
    })).ok().flatten().unwrap_or(fallback)
}
fn abs_jump(code:&mut Vec<u8>,target:usize){code.extend_from_slice(&[0xff,0x25,0,0,0,0]);code.extend_from_slice(&(target as u64).to_le_bytes());}
fn relocated(p:&Patch,base:usize,dynamic:bool)->Result<Vec<u8>, &'static str>{
    if p.fields.len()!=1 || p.first_len>p.win.len() {return Err("invalid generated window");}
    let mut out=Vec::new(); let mut at=0;
    if dynamic {out.extend_from_slice(p.dynamic);at=p.first_len;}
    while at<p.win.len(){
        if let Some(b)=p.branches.iter().find(|b|b.off as usize==at){
            if let Some(cc)=b.cc {if cc>15{return Err("bad condition");} out.extend_from_slice(&[0x70|(cc^1),14]);}
            abs_jump(&mut out,base+b.target_rva); at+=b.len as usize;
        }else{out.push(p.win[at]);at+=1;}
    }
    abs_jump(&mut out,base+p.rva+p.win.len()); Ok(out)
}
fn stub(index:usize)->Vec<u8>{
    let mut c=vec![0x48,0x8d,0x64,0x24,0xf0,0x9c];
    for r in ORDER { if r>=8{c.push(0x41);}c.push(0x50+(r&7)); }
    c.extend_from_slice(&[0x48,0x83,0xec,0x60]);
    for n in 0..6u8 {c.extend_from_slice(&[0xf3,0x0f,0x7f,0x84|(n<<3),0x24]);c.extend_from_slice(&(n as u32*16).to_le_bytes());}
    c.extend_from_slice(&[0x48,0x89,0xe5,0x48,0x89,0xea,0x48,0xb9]);
    c.extend_from_slice(&(index as u64).to_le_bytes());
    c.extend_from_slice(&[0x48,0x83,0xe4,0xf0,0x48,0x83,0xec,0x20,0xfc,0x48,0xb8]);
    c.extend_from_slice(&(dispatch as *const () as u64).to_le_bytes());c.extend_from_slice(&[0xff,0xd0]);
    c.extend_from_slice(&[0x48,0x89,0x85]);c.extend_from_slice(&232u32.to_le_bytes());
    c.extend_from_slice(&[0x48,0x89,0xec]);
    for n in 0..6u8 {c.extend_from_slice(&[0xf3,0x0f,0x6f,0x84|(n<<3),0x24]);c.extend_from_slice(&(n as u32*16).to_le_bytes());}
    c.extend_from_slice(&[0x48,0x83,0xc4,0x60]);
    for r in ORDER.into_iter().rev() {if r>=8{c.push(0x41);}c.push(0x58+(r&7));}
    c.extend_from_slice(&[0x9d,0x48,0x8d,0x64,0x24,0x10,0xff,0x64,0x24,0xf8]);c
}
fn install_site(index:usize,base:usize)->Result<(), &'static str>{
    let p=&sites::PATCHES[index];let addr=base+p.rva;
    let orig=platform::allocate_code(&relocated(p,base,false)?)?;
    let dynamic=platform::allocate_code(&relocated(p,base,true)?)?;
    let entry=platform::allocate_code_near(addr,&stub(index))?;
    let delta=i32::try_from(entry as i128-(addr+5) as i128).map_err(|_|"rel32 out of range")?;
    let mut jump=vec![0x90;p.win.len()];jump[0]=0xe9;jump[1..5].copy_from_slice(&delta.to_le_bytes());
    ORIGINAL[index].store(orig,Release);DYNAMIC[index].store(dynamic,Release);
    platform::prepare_gate(index,addr,orig)?;
    let mut now=vec![0;p.win.len()];platform::safe_read_bytes(addr,&mut now)?;
    if now!=p.win {return Err("original bytes differ / another mod owns site");}
    unsafe{platform::publish_code_byte(addr,p.win[0],0xcc)}.result?;
    // 실패하면 INT3를 영구 원본 우회로로 남긴다. 재설치/롤백 없음.
    let mut stopped=platform::SuspendedThreads::acquire(addr,p.win.len())?;
    platform::safe_read_bytes(addr,&mut now)?;
    if now[0]!=0xcc || now[1..]!=p.win[1..] {return Err("window changed under gate");}
    unsafe{platform::write_code(addr+1,&jump[1..])}.result?;
    unsafe{platform::publish_code_byte(addr,0xcc,0xe9)}.result?;
    stopped.resume_all()?; Ok(())
}
static INSTALL:OnceLock<Vec<String>>=OnceLock::new();
pub fn install()-> &'static [String] {INSTALL.get_or_init(||{
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(install_inner)).unwrap_or_else(|_|{
        for t in 0..tactics::N{ENABLED[t].store(false,Release);tactics::set_status(t,tactics::ST_FAILED);}
        vec!["적용 장치 설치 panic: 전부 원본 유지".into()]
    })
})}
fn install_inner()->Vec<String>{
    let mut logs=Vec::new();
    for t in 0..tactics::N{tactics::set_status(t,tactics::ST_UNSUPPORTED);}
    let setup=(||{
        if sites::PATCHES.len()>CAP{return Err("too many sites".to_string());}
        for name in ["tfm2_ai_analysis_verify.dll","tfm2_tactics_verify.dll"] {if platform::module_loaded(name){return Err(format!("동시 사용 불가: {name}"));}}
        let exe=platform::module_path().map_err(str::to_string)?;
        let bytes=std::fs::read(exe).map_err(|e|e.to_string())?;
        if !crate::sha256::digest(&bytes).eq_ignore_ascii_case(sites::EXE_SHA256){return Err("게임 EXE SHA256 불일치".into());}
        platform::init_safe_reader().map_err(str::to_string)?;
        platform::module_base().map_err(str::to_string)
    })();
    let base=match setup{Ok(v)=>v,Err(e)=>{for p in sites::PATCHES{tactics::set_status(p.tactic,tactics::ST_FAILED);}logs.push(e);return logs;}};
    for t in 0..tactics::N{
        let indices:Vec<_>=sites::PATCHES.iter().enumerate().filter(|(_,p)|p.tactic==t).map(|(i,_)|i).collect();
        if indices.is_empty(){continue;}
        let valid=indices.iter().all(|&i|{let p=&sites::PATCHES[i];let mut b=vec![0;p.win.len()];platform::safe_read_bytes(base+p.rva,&mut b).is_ok()&&b==p.win});
        let mut ok=valid;
        if valid{for &i in &indices{if let Err(e)=install_site(i,base){logs.push(format!("{}: {e}",sites::PATCHES[i].name));ok=false;break;}}}
        ENABLED[t].store(ok,Release);tactics::set_status(t,if ok{tactics::ST_ACTIVE}else{tactics::ST_FAILED});
        logs.push(format!("{}: {} ({}곳)",tactics::TACTICS[t].id,if ok{"설치"}else{"원본 유지"},indices.len()));
    }
    logs.push(format!("정지 전 스레드 목록 안정화 재시도={}",platform::snapshot_retries()));
    logs.push(format!("전체 스레드 장벽 복구 후 재시도={}",platform::barrier_retries()));
    logs
}
pub fn diagnostics()->Vec<String>{
    let mut lines=Vec::new();
    for(i,p)in sites::PATCHES.iter().enumerate(){lines.push(format!("{} calls={} mine={} applied={} subject_reject={}",p.name,HITS[i].load(Relaxed),MINE[i].load(Relaxed),APPLIED[i].load(Relaxed),BAD_SUBJECT[i].load(Relaxed)));}
    for s in &SAMPLES{let a=s.athlete.load(Acquire);if a!=0{lines.push(format!("sample site={} athlete_id={a} team={} pos={}",s.site.load(Relaxed),s.team.load(Relaxed),s.pos.load(Relaxed)));}}
    lines
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn frame_layout_and_codegen_fixture(){
        assert_eq!(std::mem::size_of::<Frame>(),240);
        assert_eq!(std::mem::offset_of!(Frame,value),224);
        assert_eq!(std::mem::offset_of!(Frame,target),232);
        let mut covered=std::collections::HashSet::new();
        let list:Vec<_>=sites::PATCHES.iter().enumerate().map(|(i,p)|{
            covered.insert(p.tactic);
            let d=&tactics::TACTICS[p.tactic];let f=&p.fields[0];
            for v in [d.min,d.default,d.max,d.low,d.high]{assert!(transform(f,v).is_some(),"{}: {v}",p.name);}
            serde_json::json!({"name":p.name,"rva":p.rva,"win":p.win,"first_len":p.first_len,
                "dynamic_first":p.dynamic,"original":relocated(p,0x140000000,false).unwrap(),
                "dynamic":relocated(p,0x140000000,true).unwrap(),"stub":stub(i),
                "dispatch":dispatch as *const () as usize,"field_off":f.off,"field_width":f.width,
                "values":([d.min,d.default,d.max,d.low,d.high].iter().map(|v|transform(f,*v).unwrap()).collect::<Vec<_>>())})
        }).collect();
        assert_eq!(covered.len(),26);assert_eq!(list.len(),42);
        if let Some(path)=std::env::var_os("DT_CODEGEN_DUMP") {std::fs::write(path,serde_json::to_vec_pretty(&list).unwrap()).unwrap();}
    }
    #[test] fn dispatch_team_position_and_live_value(){
        platform::init_safe_reader().unwrap();
        let athlete=0x123456789abu64;
        let mut player=vec![0u64;0x900/8];
        player[0x810/8]=athlete;player[0x820/8]=1;player[0x8b0/8]=2;
        let mut frame:Frame=unsafe{std::mem::zeroed()};
        frame.regs[14-ORDER.iter().position(|r|*r==7).unwrap()]=player.as_ptr() as u64;
        ORIGINAL[0].store(0x10000,Release);DYNAMIC[0].store(0x20000,Release);ENABLED[0].store(true,Release);
        tactics::clear_levels();tactics::reset_values();tactics::set_my_athletes(&[athlete]);tactics::set_armed(true);
        tactics::set_level(0,2,tactics::LV_HIGH);
        assert_eq!(unsafe{dispatch(0,&mut frame)},0x20000);assert_eq!(frame.value,50);
        assert!(tactics::set_value(0,tactics::LV_HIGH,60));
        assert_eq!(unsafe{dispatch(0,&mut frame)},0x20000);assert_eq!(frame.value,60);
        player[0x810/8]=athlete+1;assert_eq!(unsafe{dispatch(0,&mut frame)},0x10000);
        player[0x810/8]=athlete;player[0x8b0/8]=1;assert_eq!(unsafe{dispatch(0,&mut frame)},0x10000);
        player[0x8b0/8]=2;ENABLED[0].store(false,Release);assert_eq!(unsafe{dispatch(0,&mut frame)},0x10000);
        tactics::set_armed(false);tactics::clear_levels();tactics::reset_values();tactics::set_my_athletes(&[]);
    }
    #[test] fn native_install_and_dispatch_on_owned_fixture(){
        // 게임 프로세스/코드를 건드리지 않는다. 테스트 소유 RX 함수에 실제 설치기를 실행.
        platform::init_safe_reader().unwrap();
        let i=7;let p=&sites::PATCHES[i];assert!(p.branches.is_empty());
        let mut body=p.win.to_vec();body.extend_from_slice(&[0x48,0x89,0xc8,0xc3]); // return rcx
        let target=platform::allocate_code(&body).unwrap();
        // Windows ABI wrapper: 인수2의 Player를 rdi에 놓고 실제 stub을 호출.
        let wrapper=platform::allocate_code(&[0x57,0x48,0x89,0xd7,0x48,0x83,0xec,0x20,0x48,0x89,0xc8,0xff,0xd0,0x48,0x83,0xc4,0x20,0x5f,0xc3]).unwrap();
        let call:unsafe extern "C" fn(usize,usize)->u64=unsafe{std::mem::transmute(wrapper)};
        let mut player=vec![0u64;0x900/8];player[0x810/8]=0x123456789ab;player[0x820/8]=1;
        let ptr=player.as_ptr() as usize;
        assert_eq!(unsafe{call(target,ptr)},300);
        install_site(i,target-p.rva).unwrap();
        tactics::clear_levels();tactics::reset_values();tactics::set_my_athletes(&[player[0x810/8]]);tactics::set_armed(true);
        ENABLED[p.tactic].store(true,Release);
        assert_eq!(unsafe{call(target,ptr)},300); // 기본
        tactics::set_level(p.tactic,0,tactics::LV_LOW);
        assert_eq!(unsafe{call(target,ptr)},150);
        tactics::set_value(p.tactic,tactics::LV_LOW,777);
        assert_eq!(unsafe{call(target,ptr)},777); // 설정 변경 즉시
        player[0x810/8]+=1;assert_eq!(unsafe{call(target,ptr)},300); // 상대 선수
        player[0x810/8]-=1;tactics::set_armed(false);assert_eq!(unsafe{call(target,ptr)},300);
        // INT3 영구 원본우회(설치 실패 시 경로)도 네이티브 실행으로 확인.
        let mut stopped=platform::SuspendedThreads::acquire(target,p.win.len()).unwrap();
        unsafe{platform::publish_code_byte(target,0xe9,0xcc)}.result.unwrap();
        stopped.resume_all().unwrap();
        assert_eq!(unsafe{call(target,ptr)},300);
        ENABLED[p.tactic].store(false,Release);tactics::clear_levels();tactics::reset_values();tactics::set_my_athletes(&[]);
    }
}
