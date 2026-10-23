//! 상세전술 적용 장치. 코드 설치는 한 번, 값은 각 판단의 스택에 격리한다.
use crate::{platform, sites, tactics};
use std::sync::{OnceLock, atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::{Relaxed, Acquire, Release}}};
#[derive(Clone, Copy, Debug)]
pub enum Src {
    None, Const(u64), Reg(u8), Stack(i32), Mem(u8,i32), StackMem(i32,i32), Mem2(u8,i32,i32),
    Caller(i32,&'static [Caller]), CachePlayer { operation:u8, entity:u8, team_stack:i32 },
    CacheSlot { cache:&'static Src, team:&'static Src, pos:&'static Src, entity:&'static Src },
}
#[derive(Clone, Copy, Debug)]
pub struct Caller { pub ret_rva:usize, pub src:Src }
#[derive(Clone, Copy, Debug)]
pub enum Tf { Add(i64), Sq1, Map { on:i64, off:i64 }, Compute(fn(&Frame,i64)->Option<u64>) }
pub struct Field { pub off:u8, pub width:u8, pub tf:Tf, pub max:i64 }
pub struct Branch { pub off:u8, pub len:u8, pub cc:Option<u8>, pub target_rva:usize }
pub struct Patch {
    pub name:&'static str, pub tactic:usize, pub rva:usize, pub win:&'static [u8],
    pub fields:&'static [Field], pub branches:&'static [Branch], pub src:Src, pub team_chk:Src,
    pub first_len:usize, pub dynamic:&'static [u8],
}
pub(crate) const CAP:usize=256;
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
pub(crate) struct Frame { xmm:[u8;96], regs:[u64;15], flags:u64, value:u64, target:u64 }
impl Frame {
    pub(crate) fn flags(&self)->u64 {self.flags}
    pub(crate) fn carry(&self)->bool {self.flags&1!=0}
    pub(crate) fn reg(&self,r:u8)->Option<u64>{
        if r==4 { return Some(self as *const Self as u64 + 240); }
        ORDER.iter().position(|v|*v==r).map(|i|self.regs[14-i])
    }
    pub(crate) fn scalar(&self,s:Src)->Option<u64>{
        fn at(base:u64,d:i32)->Option<u64>{platform::fast_read_u64(base.wrapping_add_signed(d as i64) as usize)}
        match s { Src::Const(v)=>Some(v), Src::Reg(r)=>self.reg(r), Src::Stack(d)=>at(self.reg(4)?,d),
            Src::Mem(r,d)=>at(self.reg(r)?,d), Src::StackMem(a,b)=>at(at(self.reg(4)?,a)?,b),
            Src::Mem2(r,a,b)=>at(at(self.reg(r)?,a)?,b), _=>None,
        }
    }
    pub(crate) fn src(&self,s:Src)->Option<u64>{self.src_depth(s,0)}
    fn src_depth(&self,s:Src,depth:u8)->Option<u64>{
        fn at(base:u64,d:i32)->Option<u64>{platform::fast_read_u64(base.wrapping_add_signed(d as i64) as usize)}
        match s {
            Src::Caller(offset,cases)=>{
                if depth>=2 {return None;}
                let ret=at(self.reg(4)?,offset)? as usize;
                let rva=ret.checked_sub(platform::module_base().ok()?)?;
                let source=cases.iter().find(|c|c.ret_rva==rva)?.src;
                self.src_depth(source,depth+1)
            }
            Src::CacheSlot{cache:cache_src,team:team_src,pos:pos_src,entity}=>{
                let cache=self.scalar(*cache_src)?; let team=self.scalar(*team_src)?; let pos=self.scalar(*pos_src)?;
                if team>1 || pos>4 {return None;}
                let slot=(team*0x28+pos*8) as i32;
                let champion=at(cache,0x238+slot)?;
                if champion<0x10000 || champion>=1u64<<48 {return None;}
                if !matches!(entity,Src::None) && self.scalar(*entity)!=Some(champion){return None;}
                // 후보 슬롯을 알아도 중복 엔티티가 있는 손상된 캐시는 거절한다.
                for other in 0..5u64 {
                    if other!=pos && at(cache,0x238+(team*0x28+other*8) as i32)?==champion{return None;}
                }
                let player=at(cache,0x288+slot)?;
                if at(player,0x820)?!=team || platform::fast_read_u32(player.wrapping_add(0x8b0) as usize)? as u64!=pos{return None;}
                if at(cache,0x238+slot)?!=champion || at(cache,0x288+slot)?!=player{return None;}
                if self.scalar(*cache_src)!=Some(cache) || self.scalar(*team_src)!=Some(team) || self.scalar(*pos_src)!=Some(pos){return None;}
                Some(player)
            }
            Src::CachePlayer{operation,entity,team_stack}=>{
                let cache=at(self.reg(operation)?,0)?;
                let entity=self.reg(entity)?;
                let team=at(self.reg(4)?,team_stack)?;
                if team>1 || entity<0x10000 {return None;}
                // 0.6.5 cache champion[2][5] / Player[2][5].
                // 게임 자체 lookup helper 0xd787e0의 대응을 안전 읽기로 재현한다.
                let mut found=None;
                for pos in 0..5u64 {
                    let slot=team*0x28+pos*8;
                    if at(cache,0x238+slot as i32)?!=entity {continue;}
                    if found.is_some(){return None;} // 중복 엔티티 슬롯은 모호하므로 거절.
                    found=Some((at(cache,0x288+slot as i32)?,pos,slot));
                }
                let (player,pos,slot)=found?;
                if at(player,0x820)?!=team || platform::fast_read_u32(player.wrapping_add(0x8b0) as usize)? as u64!=pos {return None;}
                // 캐시 재사용/불완전한 snapshot은 원래 판단으로 돌린다.
                if at(cache,0x238+slot as i32)?!=entity || at(cache,0x288+slot as i32)?!=player {return None;}
                Some(player)
            }
            _=>self.scalar(s),
        }
    }
}
fn transform(f:&Field,v:i64,frame:&Frame)->Option<u64>{
    if let Tf::Compute(calc)=f.tf {return calc(frame,v);}
    let n=match f.tf { Tf::Add(d)=>v.checked_add(d)?, Tf::Sq1=>v.checked_mul(v)?.checked_add(1)?,
        Tf::Map{on,off}=>if v==0 {off}else{on}, Tf::Compute(_)=>unreachable!() };
    if n<0 || n>f.max {None}else{Some(n as u64)}
}
unsafe extern "C" fn dispatch(index:usize,frame:*mut Frame)->usize {
    let fallback=ORIGINAL.get(index).map(|v|v.load(Acquire)).unwrap_or(0);
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let p=sites::PATCHES.get(index)?;
        HITS[index].fetch_add(1,Relaxed);
        let f=&mut *frame;
        let player=match f.src(p.src){Some(v)=>v as usize,None=>{BAD_SUBJECT[index].fetch_add(1,Relaxed);return None;}};
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
        let revision=tactics::config_revision()?;
        let value=tactics::value_for(p.tactic,athlete,pos)?;
        let transformed=transform(p.fields.first()?,value,f)?;
        if !tactics::config_unchanged(revision){return None;}
        f.value=transformed;
        APPLIED[index].fetch_add(1,Relaxed);
        Some(DYNAMIC[index].load(Acquire))
    })).ok().flatten().unwrap_or(fallback)
}
fn abs_jump(code:&mut Vec<u8>,target:usize){code.extend_from_slice(&[0xff,0x25,0,0,0,0]);code.extend_from_slice(&(target as u64).to_le_bytes());}
fn branch_toggle(code:&mut Vec<u8>,branch:&Branch,base:usize,off:i64)->Result<(), &'static str>{
    let cc=branch.cc.filter(|v|*v<=15).ok_or("toggle needs conditional branch")?;
    let off=u8::try_from(off).map_err(|_|"toggle opcode out of range")?;
    // 판단별 값은 진입 rsp-16. pushfq 후에는 rsp-8이며 원래 분기 플래그를
    // 비교 전에 보관한다. 두 경로 모두 popfq 후 분기해 GPR/RSP/플래그 불변.
    code.extend_from_slice(&[0x9c,0x80,0x7c,0x24,0xf8,off,0x75,15,0x9d]);
    abs_jump(code,base+branch.target_rva);
    code.extend_from_slice(&[0x9d,0x70|(cc^1),14]);
    abs_jump(code,base+branch.target_rva);
    Ok(())
}
fn relocated(p:&Patch,base:usize,dynamic:bool)->Result<Vec<u8>, &'static str>{
    if p.fields.len()!=1 || p.first_len>p.win.len() {return Err("invalid generated window");}
    let mut out=Vec::new(); let mut at=0;
    if dynamic {
        if !p.dynamic.is_empty() {out.extend_from_slice(p.dynamic);
        } else if let Some(branch)=p.branches.iter().find(|b|b.off==0) {
            let Tf::Map{off,..}=p.fields[0].tf else{return Err("first branch needs toggle");};
            branch_toggle(&mut out,branch,base,off)?;
        } else {return Err("dynamic replacement missing");}
        at=p.first_len;
    }
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
    #[test] fn complete_catalog_has_relocatable_windows_and_default_bypass(){
        assert_eq!(tactics::N,88);assert_eq!(sites::PATCHES.len(),197);
        let mut counts=[0usize;tactics::N];
        for p in sites::PATCHES{
            counts[p.tactic]+=1;assert!(!matches!(p.src,Src::None));
            assert!(!relocated(p,0x140000000,false).unwrap().is_empty());
            assert!(!relocated(p,0x140000000,true).unwrap().is_empty());
        }
        assert!(counts.iter().all(|n|*n>0));assert!(sites::UNSUPPORTED.is_empty());
        tactics::clear_levels();tactics::set_my_athletes(&[101]);tactics::set_armed(true);
        for t in 0..tactics::N{for pos in 0..5{assert_eq!(tactics::value_for(t,101,pos),None);}}
        tactics::set_armed(false);
    }
    #[repr(C)] struct WideFixture {frame:Frame,stack:[u64;96]}
    fn put_reg(f:&mut Frame,r:u8,v:u64){f.regs[14-ORDER.iter().position(|x|*x==r).unwrap()]=v;}
    fn seed_scalar(s:Src,v:u64,f:&mut WideFixture,parent:&mut [u64],context:&mut [u64],team_mem:&mut [u64]){
        match s {
            Src::Const(n)=>assert_eq!(n,v), Src::Reg(r)=>put_reg(&mut f.frame,r,v),
            Src::Stack(d)=>f.stack[d as usize/8]=v,
            Src::StackMem(d,off)=>{f.stack[d as usize/8]=parent.as_ptr() as u64;parent[off as usize/8]=v;},
            Src::Mem(r,off)=>{put_reg(&mut f.frame,r,context.as_ptr() as u64);context[off as usize/8]=v;},
            Src::Mem2(r,a,b)=>{put_reg(&mut f.frame,r,context.as_ptr() as u64);context[a as usize/8]=team_mem.as_ptr() as u64;team_mem[b as usize/8]=v;},
            _=>panic!("not a scalar"),
        }
    }
    fn source_paths(s:Src,path:Vec<(i32,usize)>,out:&mut Vec<(Vec<(i32,usize)>,Src)>){
        if let Src::Caller(off,cases)=s {
            for c in cases {let mut next=path.clone();next.push((off,c.ret_rva));source_paths(c.src,next,out);}
        } else {out.push((path,s));}
    }
    #[test] fn cache_slot_checks_candidate_identity_and_rejects_ambiguous_sources(){
        platform::init_safe_reader().unwrap();
        let mut f:WideFixture=unsafe{std::mem::zeroed()};
        let mut cache=vec![0u64;0x300/8];let mut player=vec![0u64;0x900/8];
        let mut parent=vec![0u64;0x3000/8];let mut context=[0u64;4];let mut team_mem=[0u64;2];
        let source=Src::CacheSlot{cache:&Src::Mem(10,8),team:&Src::Mem2(10,0x10,0),pos:&Src::Reg(14),entity:&Src::Reg(13)};
        let Src::CacheSlot{cache:cs,team:ts,pos:ps,entity:es}=source else{unreachable!()};
        let ptr=player.as_ptr() as u64;let entity=0x12345678;
        for team in 0..2u64 {for pos in 0..5u64 {
            cache.fill(0);let slot=(team*0x28+pos*8) as usize;
            cache[(0x238+slot)/8]=entity;cache[(0x288+slot)/8]=ptr;
            player[0x810/8]=0;player[0x820/8]=team;player[0x8b0/8]=pos;
            for (s,v) in [(*cs,cache.as_ptr() as u64),(*ts,team),(*ps,pos),(*es,entity)]{seed_scalar(s,v,&mut f,&mut parent,&mut context,&mut team_mem);}
            assert_eq!(f.frame.src(source),Some(ptr));
            put_reg(&mut f.frame,13,entity+1);assert_eq!(f.frame.src(source),None);put_reg(&mut f.frame,13,entity);
            player[0x820/8]=team^1;assert_eq!(f.frame.src(source),None);player[0x820/8]=team;
            player[0x8b0/8]=(pos+1)%5;assert_eq!(f.frame.src(source),None);player[0x8b0/8]=pos;
            let duplicate=(team*0x28+(pos+1)%5*8) as usize;
            cache[(0x238+duplicate)/8]=entity;assert_eq!(f.frame.src(source),None);cache[(0x238+duplicate)/8]=0;
            cache[(0x288+slot)/8]=1;assert_eq!(f.frame.src(source),None);cache[(0x288+slot)/8]=ptr;
        }}
        put_reg(&mut f.frame,14,5);assert_eq!(f.frame.src(source),None);
        put_reg(&mut f.frame,14,4);unsafe{std::ptr::write_volatile(&mut team_mem[0],2)};assert_eq!(f.frame.src(source),None);
        // CacheSlot 내부에서 Caller/CacheSlot을 또 해석하지 않는다.
        assert_eq!(f.frame.scalar(source),None);
    }
    #[test] fn final_four_sites_use_own_subject_and_all_known_caller_paths(){
        platform::init_safe_reader().unwrap();
        let mut f:WideFixture=unsafe{std::mem::zeroed()};
        let mut parent=vec![0u64;0x3000/8];let mut context=[0u64;4];let mut team_mem=[0u64;2];
        let mut cache=vec![0u64;0x300/8];let mut player=vec![0u64;0x900/8];
        let ptr=player.as_ptr() as u64;let entity=0x12345678;let athlete=0x123456789abu64;
        let base=platform::module_base().unwrap();let mut seen=std::collections::HashSet::new();
        tactics::clear_levels();tactics::reset_values();tactics::set_my_athletes(&[athlete]);tactics::set_armed(true);
        for (i,p) in sites::PATCHES.iter().enumerate().filter(|(_,p)|[11,14,16,19].contains(&p.tactic)){
            seen.insert(p.tactic);ORIGINAL[i].store(0x10000,Release);DYNAMIC[i].store(0x20000,Release);ENABLED[p.tactic].store(true,Release);
            let mut paths=Vec::new();source_paths(p.src,Vec::new(),&mut paths);
            for (guards,source) in paths {for team in 0..2u64 {for pos in 0..5u64 {
                if let Src::CacheSlot{pos:ps,..}=source {if let Src::Const(n)=ps {if *n!=pos{continue;}}}
                f.frame=unsafe{std::mem::zeroed()};f.stack.fill(0);parent.fill(0);context.fill(0);cache.fill(0);
                // helper의 현재 RBP는 이미 덮였어도 저장 RBP만으로 복구해야 한다.
                put_reg(&mut f.frame,5,1);put_reg(&mut f.frame,9,0xdeadbeef);
                player[0x810/8]=athlete;player[0x820/8]=team;player[0x8b0/8]=pos;
                let slot=(team*0x28+pos*8) as usize;cache[(0x238+slot)/8]=entity;cache[(0x288+slot)/8]=ptr;
                for &(off,ret) in &guards {f.stack[off as usize/8]=(base+ret) as u64;}
                if let Src::CacheSlot{cache:cs,team:ts,pos:ps,entity:es}=source {
                    for (s,v) in [(*cs,cache.as_ptr() as u64),(*ts,team),(*ps,pos)]{seed_scalar(s,v,&mut f,&mut parent,&mut context,&mut team_mem);}
                    if !matches!(es,Src::None){seed_scalar(*es,entity,&mut f,&mut parent,&mut context,&mut team_mem);}
                }else{seed_scalar(source,ptr,&mut f,&mut parent,&mut context,&mut team_mem);}
                tactics::set_level(p.tactic,pos as usize,tactics::LV_LOW);
                assert_eq!(unsafe{dispatch(i,&mut f.frame)},0x20000,"{} team={team} pos={pos}",p.name);
                assert_eq!(f.frame.value,transform(&p.fields[0],tactics::value_of(p.tactic,1),&f.frame).unwrap());
                player[0x810/8]=athlete+1;assert_eq!(unsafe{dispatch(i,&mut f.frame)},0x10000);player[0x810/8]=athlete;
                tactics::set_armed(false);assert_eq!(unsafe{dispatch(i,&mut f.frame)},0x10000);tactics::set_armed(true);
                tactics::clear_levels();assert_eq!(unsafe{dispatch(i,&mut f.frame)},0x10000);tactics::set_level(p.tactic,pos as usize,1);
                for &(off,ret) in &guards {
                    f.stack[off as usize/8]=(base+0x12345) as u64;assert_eq!(unsafe{dispatch(i,&mut f.frame)},0x10000);
                    f.stack[off as usize/8]=(base+ret) as u64;
                }
            }}}
            ENABLED[p.tactic].store(false,Release);
        }
        assert_eq!(seen.len(),4);
        tactics::set_armed(false);tactics::clear_levels();tactics::reset_values();tactics::set_my_athletes(&[]);
    }
    #[test] fn caller_source_requires_known_return_and_current_stack() {
        platform::init_safe_reader().unwrap();
        #[repr(C)] struct Fixture { frame:Frame, stack:[u64;32] }
        let mut f:Fixture=unsafe{std::mem::zeroed()};
        static CASES:[Caller;2]=[
            Caller{ret_rva:0x12345,src:Src::Stack(0x48)},
            Caller{ret_rva:0x54321,src:Src::StackMem(0x30,0x10)},
        ];
        let src=Src::Caller(0x68,&CASES);
        let base=platform::module_base().unwrap();
        f.stack[0x68/8]=(base+0x12345) as u64; f.stack[0x48/8]=0x123456789ab;
        assert_eq!(f.frame.src(src),Some(0x123456789ab));
        let mut nested=[0u64;4]; nested[2]=0x23456789abc;
        f.stack[0x30/8]=nested.as_ptr() as u64; f.stack[0x68/8]=(base+0x54321) as u64;
        assert_eq!(f.frame.src(src),Some(0x23456789abc));
        f.stack[0x68/8]=(base+0x99999) as u64;
        assert_eq!(f.frame.src(src),None); // 모르는 호출자는 어떤 슬롯도 선수로 추정하지 않는다.
        f.stack[0x68/8]=1; assert_eq!(f.frame.src(src),None);
        f.stack[0x68/8]=(base+0x54321) as u64; f.stack[0x30/8]=1;
        assert_eq!(f.frame.src(src),None); // 잘못된 저장 포인터도 원본.
    }
    #[test] fn cache_subject_matches_entity_team_and_position_without_global_side() {
        platform::init_safe_reader().unwrap();
        #[repr(C)] struct Fixture { frame:Frame, stack:[u64;32] }
        let mut f:Fixture=unsafe{std::mem::zeroed()};
        let mut cache=vec![0u64;0x300/8]; let mut player=vec![0u64;0x900/8];
        let operation=[cache.as_ptr() as u64];
        let entity=0x12345678u64;
        f.frame.regs[14-ORDER.iter().position(|r|*r==6).unwrap()]=operation.as_ptr() as u64;
        f.frame.regs[14-ORDER.iter().position(|r|*r==12).unwrap()]=entity;
        f.stack[0xe0/8]=1;
        let slot=0x28+3*8;
        cache[(0x238+slot)/8]=entity; cache[(0x288+slot)/8]=player.as_ptr() as u64;
        player[0x810/8]=0; player[0x820/8]=1; player[0x8b0/8]=3;
        let source=Src::CachePlayer{operation:6,entity:12,team_stack:0xe0};
        for team in 0..2usize {for pos in 0..5usize {
            cache.fill(0); let s=team*0x28+pos*8;
            f.stack[0xe0/8]=team as u64; player[0x820/8]=team as u64; player[0x8b0/8]=pos as u64;
            cache[(0x238+s)/8]=entity;cache[(0x288+s)/8]=player.as_ptr() as u64;
            assert_eq!(f.frame.src(source),Some(player.as_ptr() as u64));
        }}
        cache.fill(0); f.stack[0xe0/8]=1;player[0x820/8]=1;player[0x8b0/8]=3;
        cache[(0x238+slot)/8]=entity;cache[(0x288+slot)/8]=player.as_ptr() as u64;
        assert_eq!(f.frame.src(source),Some(player.as_ptr() as u64));
        cache[(0x238+0x28)/8]=entity;assert_eq!(f.frame.src(source),None);
        cache[(0x238+0x28)/8]=0;
        player[0x820/8]=0; assert_eq!(f.frame.src(source),None);
        player[0x820/8]=1; player[0x8b0/8]=2; assert_eq!(f.frame.src(source),None);
        player[0x8b0/8]=3; f.stack[0xe0/8]=0; assert_eq!(f.frame.src(source),None);
        f.stack[0xe0/8]=2; assert_eq!(f.frame.src(source),None);
        f.stack[0xe0/8]=1; cache[(0x238+slot)/8]=entity+1; assert_eq!(f.frame.src(source),None);
        cache[(0x238+slot)/8]=entity; cache[(0x288+slot)/8]=1; assert_eq!(f.frame.src(source),None);
        // 같은 side의 다른 경기 캐시를 가로질러 검색하지 않는다.
        let empty_cache=vec![0u64;0x300/8]; let other_operation=[empty_cache.as_ptr() as u64];
        f.frame.regs[14-ORDER.iter().position(|r|*r==6).unwrap()]=other_operation.as_ptr() as u64;
        assert_eq!(f.frame.src(source),None);
    }
    #[test] fn frame_layout_and_codegen_fixture(){
        assert_eq!(std::mem::size_of::<Frame>(),240);
        assert_eq!(std::mem::offset_of!(Frame,value),224);
        assert_eq!(std::mem::offset_of!(Frame,target),232);
        let mut covered=std::collections::HashSet::new();
        let list:Vec<_>=sites::PATCHES.iter().enumerate().filter(|(_,p)|p.tactic<32).map(|(i,p)|{
            covered.insert(p.tactic);
            let d=&tactics::TACTICS[p.tactic];let f=&p.fields[0];
            let frame:Frame=unsafe{std::mem::zeroed()};
            for v in [d.min,d.default,d.max,d.low,d.high]{assert!(transform(f,v,&frame).is_some(),"{}: {v}",p.name);}
            let toggle=p.branches.iter().find(|b|b.off==0).map(|b|serde_json::json!({
                "target":0x140000000usize+b.target_rva,"on":p.win[0],"off":0xeb}));
            serde_json::json!({"name":p.name,"rva":p.rva,"win":p.win,"first_len":p.first_len,"branch_toggle":toggle,
                "dynamic_first":p.dynamic,"original":relocated(p,0x140000000,false).unwrap(),
                "dynamic":relocated(p,0x140000000,true).unwrap(),"stub":stub(i),
                "dispatch":dispatch as *const () as usize,"field_off":f.off,"field_width":f.width,
                "values":([d.min,d.default,d.max,d.low,d.high].iter().map(|v|transform(f,*v,&frame).unwrap()).collect::<Vec<_>>())})
        }).collect();
        assert_eq!(covered.len(),32);assert_eq!(list.len(),53);
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
    #[test] fn restored_sites_dispatch_only_the_recovered_own_player() {
        platform::init_safe_reader().unwrap();
        #[repr(C)] struct Fixture { frame:Frame, stack:[u64;32] }
        let mut f:Fixture=unsafe{std::mem::zeroed()};
        let mut player=vec![0u64;0x900/8]; let ptr=player.as_ptr() as u64;
        let mut parent=vec![0u64;0x400/8]; parent[0x3d0/8]=ptr;
        let mut cache=vec![0u64;0x300/8]; let operation=[cache.as_ptr() as u64];
        let athlete=0x123456789abu64; let base=platform::module_base().unwrap();
        tactics::clear_levels();tactics::reset_values();tactics::set_my_athletes(&[athlete]);tactics::set_armed(true);
        for (i,p) in sites::PATCHES.iter().enumerate().filter(|(_,p)|p.tactic==23||p.tactic==27) {
            ORIGINAL[i].store(0x10000,Release);DYNAMIC[i].store(0x20000,Release);ENABLED[p.tactic].store(true,Release);
            for team in 0..2u64 {for pos in 0..5usize {
                player[0x810/8]=athlete;player[0x820/8]=team;player[0x8b0/8]=pos as u64;
                f.frame.regs[14-ORDER.iter().position(|r|*r==1).unwrap()]=team;
                tactics::set_level(p.tactic,if p.tactic==27{5}else{pos},tactics::LV_LOW);
                let cases=match p.src {Src::Caller(_,cases)=>cases,_=>&[]};
                for case in 0..cases.len().max(1) {
                    f.stack.fill(0);
                    if let Src::Caller(off,_) = p.src {
                        let c=cases[case];f.stack[off as usize/8]=(base+c.ret_rva) as u64;
                        match c.src {
                            Src::Stack(d)=>f.stack[d as usize/8]=ptr,
                            Src::StackMem(d,_)=>f.stack[d as usize/8]=parent.as_ptr() as u64,
                            _=>panic!("unexpected caller source"),
                        }
                    } else {
                        f.stack[0xe0/8]=team;
                        f.frame.regs[14-ORDER.iter().position(|r|*r==6).unwrap()]=operation.as_ptr() as u64;
                        f.frame.regs[14-ORDER.iter().position(|r|*r==12).unwrap()]=0x12345678;
                        cache.fill(0);let slot=team as usize*0x28+pos*8;
                        cache[(0x238+slot)/8]=0x12345678;cache[(0x288+slot)/8]=ptr;
                    }
                    assert_eq!(unsafe{dispatch(i,&mut f.frame)},0x20000,"{} team={team} pos={pos} caller={case}",p.name);
                    assert_eq!(f.frame.value,transform(&p.fields[0],tactics::value_of(p.tactic,1),&f.frame).unwrap());
                    player[0x810/8]=athlete+1;
                    assert_eq!(unsafe{dispatch(i,&mut f.frame)},0x10000); // 같은 team/pos라도 상대 선수면 원본.
                    player[0x810/8]=athlete;
                    tactics::set_armed(false);assert_eq!(unsafe{dispatch(i,&mut f.frame)},0x10000);tactics::set_armed(true);
                }
                if let Src::Caller(off,_) = p.src {
                    f.stack[off as usize/8]=(base+0x12345) as u64;
                    assert_eq!(unsafe{dispatch(i,&mut f.frame)},0x10000); // 모르는 호출자는 원본.
                }
            }}
            ENABLED[p.tactic].store(false,Release);
        }
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
