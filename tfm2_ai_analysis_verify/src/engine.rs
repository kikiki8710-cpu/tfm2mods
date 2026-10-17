//! 게임을 추가 호출하지 않는 관측·독립예상값·비교·비동기 덤프 배선.
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use crate::{config::Config, generated, hook, models, platform};

pub const MAX_FIELDS: usize = 96;
pub const MODEL_COUNT: usize = 8;
pub trait Memory { fn read(&mut self, address: u64, width: u8) -> Result<u64, &'static str>; }
#[derive(Clone, Debug)]
pub struct Metric {
    pub name: &'static str, pub lhs: i128, pub rhs: i128,
    pub relation: &'static str, pub predicate: bool, pub observed_branch: bool,
}
#[derive(Clone, Debug)]
pub struct Evaluation {
    pub expected: u64, pub mask: u64, pub output_ptr: Option<u64>,
    pub output_fields: Vec<(u64,u8,u64)>, pub metrics: Vec<Metric>,
}
#[derive(Clone, Debug)]
struct Field { address: u64, width: u8, value: u64 }
struct Capture { reads: Vec<Field> }

fn live_read(address: u64, width: u8) -> Result<u64, &'static str> {
    let mut bytes = [0u8;8];
    if !matches!(width,1|2|4|8) {return Err("unsupported_field_width");}
    let p = usize::try_from(address).map_err(|_| "address_width")?;
    platform::safe_read_bytes(p,&mut bytes[..width as usize])?;
    Ok(u64::from_le_bytes(bytes))
}
impl Memory for Capture {
    fn read(&mut self, address: u64, width: u8) -> Result<u64,&'static str> {
        if let Some(f) = self.reads.iter().find(|f|f.address==address&&f.width==width) {return Ok(f.value);}
        if self.reads.len()>=MAX_FIELDS {return Err("snapshot_field_budget");}
        let value=live_read(address,width)?;
        self.reads.push(Field{address,width,value}); Ok(value)
    }
}
struct Replay<'a>(&'a [Field]);
impl Memory for Replay<'_> {
    fn read(&mut self,address:u64,width:u8)->Result<u64,&'static str> {
        self.0.iter().find(|f|f.address==address&&f.width==width).map(|f|f.value).ok_or("snapshot_missing_field")
    }
}

#[derive(Default)]
struct Stats {
    calls:AtomicU64, sample_attempts:AtomicU64, compared:AtomicU64, matches:AtomicU64,
    mismatches:AtomicU64, skipped:AtomicU64, unstable:AtomicU64, model_panics:AtomicU64,
    dropped:AtomicU64, foreign_matches:AtomicU64, foreign_mismatches:AtomicU64,
    processing_errors:AtomicU64, unlogged_errors:AtomicU64,
}
#[derive(Clone)]
struct InstallStatus { id:usize, text:String, foreign_chain:bool, original:usize, target:usize }
#[derive(Clone)]
struct Event {
    id:usize,seq:u64,thread_id:u32,args:[u64;4],actual:u64,
    evaluation:Option<Evaluation>,reads:Vec<Field>,output_fields:Vec<(u64,u8,u64,u64)>,
    comparison:&'static str,reason:&'static str,foreign_chain:bool,
}
struct State {
    dir:PathBuf, run_id:String, config:Config, identity_verified:bool, actual_sha256:String,
    stats:[Stats;MODEL_COUNT], install:Mutex<Vec<InstallStatus>>,
    queue:Mutex<VecDeque<Event>>, writer:Mutex<BufWriter<File>>,
    frames:AtomicU64, sequence:AtomicU64, install_tried:AtomicUsize, io_errors:AtomicU64,
    init_ready:AtomicUsize, io_lost_or_uncertain_events:AtomicU64,
}
static STATE:OnceLock<State>=OnceLock::new();
pub static ORIGINALS:[AtomicUsize;MODEL_COUNT]=[const {AtomicUsize::new(0)};MODEL_COUNT];
static DOMAINS:[AtomicUsize;MODEL_COUNT]=[const {AtomicUsize::new(0)};MODEL_COUNT];

fn counter(x:&AtomicU64)->u64 {x.load(Ordering::Relaxed)}
pub fn json_quote(s:&str)->String {
    let mut out=String::from("\"");
    for c in s.chars() {match c {
        '"'=>out.push_str("\\\""),'\\'=>out.push_str("\\\\"),'\n'=>out.push_str("\\n"),
        '\r'=>out.push_str("\\r"),'\t'=>out.push_str("\\t"),
        c if c < ' '=>out.push_str(&format!("\\u{:04x}",c as u32)), _=>out.push(c),
    }} out.push('"');out
}
fn hexadecimal(x:u64)->String {format!("\"0x{x:x}\"")}
fn write_atomic(path:&Path,text:&str)->std::io::Result<()> {
    let temp=path.with_extension("json.tmp");
    {let mut f=File::create(&temp)?;f.write_all(text.as_bytes())?;f.sync_all()?;}
    std::fs::rename(&temp,path)
}

pub fn initialize(dir:PathBuf, config:Config,actual_sha256:String,identity_verified:bool)->Result<(),String> {
    for rva in &config.probes {
        if !generated::PROBES.iter().any(|p|p.rva==*rva) {return Err(format!("미지원/진입불가 probe: 0x{rva:x}"));}
    }
    std::fs::create_dir_all(&dir).map_err(|e|e.to_string())?;
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?.as_nanos();
    let run_id=format!("{}_{stamp}",std::process::id());
    let output=dir.join(format!("observations_{run_id}.jsonl"));
    let f=OpenOptions::new().create_new(true).write(true).open(output).map_err(|e|e.to_string())?;
    let capacity=config.queue_capacity;
    STATE.set(State{dir,run_id,config,identity_verified,actual_sha256,
        stats:std::array::from_fn(|_|Stats::default()),install:Mutex::new(Vec::new()),
        queue:Mutex::new(VecDeque::with_capacity(capacity)),writer:Mutex::new(BufWriter::new(f)),
        frames:AtomicU64::new(0),sequence:AtomicU64::new(0),install_tried:AtomicUsize::new(0),io_errors:AtomicU64::new(0),
        init_ready:AtomicUsize::new(0),io_lost_or_uncertain_events:AtomicU64::new(0),
    }).map_err(|_|"duplicate_initialize".to_string())?;
    flush().map_err(|e|e.to_string())?;
    STATE.get().unwrap().init_ready.store(1,Ordering::Release);
    Ok(())
}

// 공통 관측 처리. 게임 본체의 CALL은 반드시 한 곳에서 한 번만 수행한다.
unsafe fn observe(id:usize,args:[u64;4])->u64 {
    let original=ORIGINALS[id].load(Ordering::Acquire);
    // 설치 측에서 유효한 trampoline을 선게시한다. 이 불변조건 이전에는 wrapper에 도달할 수 없다.
    let function:unsafe extern "system-unwind" fn(u64,u64,u64,u64)->u64=std::mem::transmute(original);
    let Some(state)=STATE.get() else {return function(args[0],args[1],args[2],args[3]);};
    let stats=&state.stats[id];
    let calls=stats.calls.fetch_add(1,Ordering::Relaxed);
    if DOMAINS[id].load(Ordering::Acquire)<2 {
        stats.skipped.fetch_add(1,Ordering::Relaxed);
        return function(args[0],args[1],args[2],args[3]);
    }
    if calls%state.config.sample_stride!=0 {return function(args[0],args[1],args[2],args[3]);}
    let ticket=stats.sample_attempts.fetch_add(1,Ordering::Relaxed);
    if ticket>=state.config.max_samples_per_probe {return function(args[0],args[1],args[2],args[3]);}
    let prepared=catch_unwind(AssertUnwindSafe(|| {
        let mut capture=Capture{reads:Vec::with_capacity(MAX_FIELDS)};
        let evaluation=models::evaluate(id,args,&mut capture);
        (evaluation,capture.reads)
    }));
    // 원본 예외는 잡아서 재호출하지 않는다. 모델 panic도 원본 1회 규칙에 영향을 주지 않는다.
    let actual=function(args[0],args[1],args[2],args[3]);
    let completed=catch_unwind(AssertUnwindSafe(|| {
        let mut event=Event{id,seq:state.sequence.fetch_add(1,Ordering::Relaxed),thread_id:current_thread_id(),
            args,actual,evaluation:None,reads:Vec::new(),output_fields:Vec::new(),
            comparison:"SKIP",reason:"",foreign_chain:DOMAINS[id].load(Ordering::Acquire)==3};
        match prepared {
            Err(_)=> {event.reason="MODEL_PANIC";stats.model_panics.fetch_add(1,Ordering::Relaxed);}
            Ok((Err(reason),reads))=> {event.reason=reason;event.reads=reads;}
            Ok((Ok(evaluation),reads))=> {
                let mut replay=Replay(&reads);
                let second=models::evaluate(id,args,&mut replay)?;
                if second.expected!=evaluation.expected||second.mask!=evaluation.mask||second.output_fields!=evaluation.output_fields {
                    return Err("MODEL_SNAPSHOT_REPLAY_DIFFERENCE");
                }
                event.evaluation=Some(evaluation.clone()); event.reads=reads;
                for field in &event.reads {
                    match live_read(field.address,field.width) {
                        Ok(now) if now==field.value=>{},
                        Ok(_)=> {event.comparison="UNSTABLE";event.reason="INPUT_CHANGED_DURING_ORIGINAL";break;},
                        Err(_)=> {event.comparison="UNSTABLE";event.reason="INPUT_UNREADABLE_AFTER_ORIGINAL";break;},
                    }
                }
                if event.comparison!="UNSTABLE" {
                    let mut equal=(actual&evaluation.mask)==(evaluation.expected&evaluation.mask);
                    for &(address,width,expected) in &evaluation.output_fields {
                        let value=live_read(address,width)?;equal&=value==expected;
                        event.output_fields.push((address,width,expected,value));
                    }
                    event.comparison=match (event.foreign_chain,equal) {
                        (false,true)=>"MATCH",(false,false)=>"MISMATCH",(true,true)=>"FOREIGN_MATCH",(true,false)=>"FOREIGN_MISMATCH",
                    };
                }
            }
        }
        match event.comparison {
            "MATCH"=>{stats.compared.fetch_add(1,Ordering::Relaxed);stats.matches.fetch_add(1,Ordering::Relaxed);},
            "MISMATCH"=>{stats.compared.fetch_add(1,Ordering::Relaxed);stats.mismatches.fetch_add(1,Ordering::Relaxed);},
            "FOREIGN_MATCH"=>{stats.foreign_matches.fetch_add(1,Ordering::Relaxed);},
            "FOREIGN_MISMATCH"=>{stats.foreign_mismatches.fetch_add(1,Ordering::Relaxed);},
            "UNSTABLE"=>{stats.unstable.fetch_add(1,Ordering::Relaxed);},
            _=>{stats.skipped.fetch_add(1,Ordering::Relaxed);},
        }
        if let Ok(mut queue)=state.queue.try_lock() {
            if queue.len()<state.config.queue_capacity {queue.push_back(event);}
            else {stats.dropped.fetch_add(1,Ordering::Relaxed);}
        } else {stats.dropped.fetch_add(1,Ordering::Relaxed);}
        Ok::<(),&'static str>(())
    }));
    let failure=match completed {Ok(Ok(()))=>None,Ok(Err(e))=>Some(e),Err(_)=>Some("POSTPROCESSING_PANIC")};
    if let Some(reason)=failure {
        stats.skipped.fetch_add(1,Ordering::Relaxed);stats.processing_errors.fetch_add(1,Ordering::Relaxed);
        let event=Event{id,seq:state.sequence.fetch_add(1,Ordering::Relaxed),thread_id:current_thread_id(),args,actual,
            evaluation:None,reads:Vec::new(),output_fields:Vec::new(),comparison:"SKIP",reason,
            foreign_chain:DOMAINS[id].load(Ordering::Acquire)==3};
        let mut logged=false;
        if let Ok(mut queue)=state.queue.try_lock() {if queue.len()<state.config.queue_capacity {queue.push_back(event);logged=true;}}
        if !logged {stats.unlogged_errors.fetch_add(1,Ordering::Relaxed);stats.dropped.fetch_add(1,Ordering::Relaxed);}
    }
    actual
}

#[link(name="kernel32")]
extern "system" {fn GetCurrentThreadId()->u32;}
fn current_thread_id()->u32 {unsafe{GetCurrentThreadId()}}
macro_rules! wrapper {
    ($name:ident,$id:expr)=>{unsafe extern "system-unwind" fn $name(a:u64,b:u64,c:u64,d:u64)->u64{observe($id,[a,b,c,d])}};
}
wrapper!(hook0,0);wrapper!(hook1,1);wrapper!(hook2,2);wrapper!(hook3,3);
wrapper!(hook4,4);wrapper!(hook5,5);wrapper!(hook6,6);wrapper!(hook7,7);
fn wrappers()->[usize;MODEL_COUNT] {[hook0 as *const () as usize,hook1 as *const () as usize,hook2 as *const () as usize,hook3 as *const () as usize,
    hook4 as *const () as usize,hook5 as *const () as usize,hook6 as *const () as usize,hook7 as *const () as usize]}

pub fn post_update() {
    let Some(state)=STATE.get() else {return;};
    if state.init_ready.load(Ordering::Acquire)==0 {return;}
    let frame=state.frames.fetch_add(1,Ordering::Relaxed)+1;
    if frame>=state.config.install_delay_frames&&state.install_tried.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).is_ok() {
        let mut statuses=Vec::new();let wrapper=wrappers();
        for p in generated::PROBES {
            let mut entry=InstallStatus{id:p.id,text:"NOT_SELECTED".into(),foreign_chain:false,original:0,target:0};
            if !state.config.armed {entry.text="DISARMED".into();}
            else if !state.identity_verified {entry.text="EXE_IDENTITY_MISMATCH".into();}
            else if state.config.probes.contains(&p.rva) {
                // ABI가 검토된 모델 8개 한정. 범위 밖 id는 거절한다.
                if p.id>=MODEL_COUNT {entry.text="UNSUPPORTED_MODEL_ID".into();statuses.push(entry);continue;}
                let spec=hook::HookSpec{rva:p.rva,prologue:p.prologue,body:p.body,wrapper:wrapper[p.id],
                    allow_foreign_chain:state.config.allow_foreign_chain};
                // 설치 성공이 게시되기 전까지 원본만 실행하며 비교하지 않는다.
                DOMAINS[p.id].store(1,Ordering::Release);
                match hook::install_once(&spec,&ORIGINALS[p.id]) {
                    Ok(installed)=>{entry.text="INSTALLED_UNVALIDATED_RUNTIME".into();entry.foreign_chain=installed.foreign_chain;
                        entry.original=installed.original;entry.target=installed.target;
                        DOMAINS[p.id].store(if installed.foreign_chain{3}else{2},Ordering::Release);},
                    Err(reason)=>{entry.text=format!("{}:{reason}",if ORIGINALS[p.id].load(Ordering::Acquire)!=0 {
                        "INSTALL_ERROR_ORIGINAL_RETAINED_ENTRY_UNVERIFIED"}else{"REFUSED_BEFORE_WRITE"});},
                }
            }
            statuses.push(entry);
        }
        *state.install.lock().unwrap_or_else(|e|e.into_inner())=statuses;
        let _=flush();
    }
    if frame%180==0 {let _=flush();}
}

fn event_json(state:&State,e:&Event)->String {
    let p=generated::PROBES.iter().find(|p|p.id==e.id);
    let (expected,mask)=e.evaluation.as_ref().map(|v|(hexadecimal(v.expected),hexadecimal(v.mask))).unwrap_or(("null".into(),"null".into()));
    let reads=e.reads.iter().map(|f|format!("{{\"address\":{},\"width\":{},\"value\":{}}}",hexadecimal(f.address),f.width,hexadecimal(f.value))).collect::<Vec<_>>().join(",");
    let metrics=e.evaluation.as_ref().map(|v|v.metrics.iter().map(|m|format!("{{\"name\":{},\"lhs\":{},\"rhs\":{},\"relation\":{},\"predicate\":{},\"observed_branch\":{}}}",
        json_quote(m.name),json_quote(&m.lhs.to_string()),json_quote(&m.rhs.to_string()),json_quote(m.relation),m.predicate,m.observed_branch)).collect::<Vec<_>>().join(",")).unwrap_or_default();
    let outputs=e.output_fields.iter().map(|&(a,w,x,y)|format!("{{\"address\":{},\"width\":{},\"expected\":{},\"actual\":{}}}",hexadecimal(a),w,hexadecimal(x),hexadecimal(y))).collect::<Vec<_>>().join(",");
    format!("{{\"schema_version\":1,\"run_id\":{},\"kind\":\"sample\",\"probe_id\":{},\"rva\":{},\"thread_id\":{},\"seq\":{},\"comparison\":{},\"reason\":{},\"domain\":{},\"args\":[{}],\"expected\":{},\"actual\":{},\"mask\":{},\"reads\":[{}],\"metrics\":[{}],\"output_fields\":[{}]}}",
        json_quote(&state.run_id),e.id,hexadecimal(p.map(|p|p.rva as u64).unwrap_or(0)),e.thread_id,e.seq,json_quote(e.comparison),json_quote(e.reason),
        json_quote(if e.foreign_chain{"foreign_chain"}else{"vanilla"}),e.args.iter().map(|&x|hexadecimal(x)).collect::<Vec<_>>().join(","),expected,hexadecimal(e.actual),mask,reads,metrics,outputs)
}

pub fn flush()->std::io::Result<()> {
    let Some(state)=STATE.get() else {return Ok(());};
    let events:Vec<Event>={let mut q=state.queue.lock().unwrap_or_else(|e|e.into_inner());q.drain(..).collect()};
    let written=(|| {let mut writer=state.writer.lock().unwrap_or_else(|e|e.into_inner());
        for e in &events {
            writeln!(writer,"{}",event_json(state,e))?;
        }
        writer.flush()
    })();
    if let Err(error)=written {
        state.io_errors.fetch_add(1,Ordering::Relaxed);
        state.io_lost_or_uncertain_events.fetch_add(events.len() as u64,Ordering::Relaxed);
        return Err(error);
    }
    let installs=state.install.lock().unwrap_or_else(|e|e.into_inner()).clone();
    let probes=generated::PROBES.iter().map(|p| {
        let s=&state.stats[p.id];let status=installs.iter().find(|x|x.id==p.id);
        format!("{{\"id\":{},\"rva\":{},\"semantic_id\":{},\"install_status\":{},\"foreign_chain\":{},\"original\":{},\"target\":{},\"calls\":{},\"sample_attempts\":{},\"compared\":{},\"matches\":{},\"mismatches\":{},\"skipped\":{},\"unstable\":{},\"model_panics\":{},\"dropped\":{},\"foreign_matches\":{},\"foreign_mismatches\":{},\"processing_errors\":{},\"unlogged_errors\":{},\"verdict\":{}}}",
            p.id,hexadecimal(p.rva as u64),json_quote(p.semantic_id),json_quote(status.map(|x|x.text.as_str()).unwrap_or("WAITING_INSTALL_DELAY")),status.map(|x|x.foreign_chain).unwrap_or(false),
            hexadecimal(status.map(|x|x.original as u64).unwrap_or(0)),hexadecimal(status.map(|x|x.target as u64).unwrap_or(0)),
            counter(&s.calls),counter(&s.sample_attempts),counter(&s.compared),counter(&s.matches),counter(&s.mismatches),counter(&s.skipped),counter(&s.unstable),counter(&s.model_panics),counter(&s.dropped),counter(&s.foreign_matches),counter(&s.foreign_mismatches),counter(&s.processing_errors),counter(&s.unlogged_errors),
            json_quote(if counter(&s.compared)==0 {"NO_SAMPLES"} else if counter(&s.mismatches)>0 {"MISMATCH_OBSERVED"} else {"OBSERVED_SAMPLES_MATCH_ONLY"}))
    }).collect::<Vec<_>>().join(",");
    let summary=format!("{{\"schema_version\":1,\"run_id\":{},\"baseline_sha256\":{},\"actual_sha256\":{},\"identity_verified\":{},\"armed\":{},\"catalog_total\":{},\"prepared_probes\":{},\"io_errors\":{},\"io_lost_or_uncertain_events\":{},\"sample_stride\":{},\"max_samples_per_probe\":{},\"probes\":[{}],\"scope\":\"관측표본의출력대조만. 실제표본0은검증불성립; 전체분기·모든도메인·임계값독립조절가능성증명아님\"}}\n",
        json_quote(&state.run_id),json_quote(generated::BASELINE_EXE_SHA256),json_quote(&state.actual_sha256),state.identity_verified,state.config.armed,generated::CENSUS_FUNCTIONS,generated::PROBES.len(),counter(&state.io_errors),counter(&state.io_lost_or_uncertain_events),state.config.sample_stride,state.config.max_samples_per_probe,probes);
    let written=write_atomic(&state.dir.join("summary.json"),&summary);
    if written.is_err() {state.io_errors.fetch_add(1,Ordering::Relaxed);}
    written
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn quotes_cover_json_controls() {
        assert_eq!(json_quote("\"\\\n\t\u{1}가"),"\"\\\"\\\\\\n\\t\\u0001가\"");
    }
    #[test] fn missing_snapshot_is_an_error() {
        let mut r=Replay(&[]);assert!(r.read(0x10000,8).is_err());
    }
    static FAKE_CALLS:AtomicU64=AtomicU64::new(0);
    static FAKE_BOOLEAN_CALLS:AtomicU64=AtomicU64::new(0);
    const FAKE_RESULT:u64=0x1122_3344_5566_7788;
    unsafe extern "system-unwind" fn fake_original(a:u64,_b:u64,mode:u64,_d:u64)->u64 {
        FAKE_CALLS.fetch_add(1,Ordering::Relaxed);
        if mode==1 {
            // 테스트가 소유한 Vec의 count만 변경한다. 게임 메모리에 대한 쓰기는 아니다.
            std::ptr::write_unaligned((a as usize+0x840)as *mut u64,1);
        }
        if mode==2 {0}else{FAKE_RESULT}
    }
    unsafe extern "system-unwind" fn fake_boolean_original(_a:u64,_b:u64,_c:u64,_d:u64)->u64 {
        FAKE_BOOLEAN_CALLS.fetch_add(1,Ordering::Relaxed);0x1122_3344_5566_7700
    }
    #[test] fn native_fake_original_once_and_initial_io_failure_never_installs() {
        let dir=std::env::temp_dir().join(format!("tfm2_aiav_engine_selftest_{}_{}",std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(dir.join("summary.json")).unwrap();
        let configuration=Config{sample_stride:1,max_samples_per_probe:20,queue_capacity:8,..Config::default()};
        assert!(initialize(dir.clone(),configuration,"ENGINE_SELFTEST_NOT_A_GAME_SESSION".into(),false).is_err());
        let state=STATE.get().unwrap();
        for _ in 0..200 {post_update();}
        assert_eq!(state.install_tried.load(Ordering::Acquire),0);
        assert!(ORIGINALS.iter().all(|x|x.load(Ordering::Acquire)==0));
        std::fs::remove_dir(dir.join("summary.json")).unwrap();
        platform::init_safe_reader().unwrap();
        // 아래는 설치 없이 가짜 원본에만 연결하는 시험이다. 실제 게임 원문 변경은 없다.
        ORIGINALS[7].store(fake_original as *const () as usize,Ordering::Release);
        DOMAINS[7].store(2,Ordering::Release);
        let mut own=vec![0u64;0x848/8];let address=own.as_mut_ptr()as u64;
        assert_eq!(unsafe{observe(7,[address,99,0,0])},FAKE_RESULT); // 예상0과 불일치
        assert_eq!(unsafe{observe(7,[address,99,1,0])},FAKE_RESULT); // 원본 중 입력변경
        own[0x840/8]=0;
        assert_eq!(unsafe{observe(7,[0,99,0,0])},FAKE_RESULT);       // 읽기실패여도 원반환 유지
        DOMAINS[7].store(3,Ordering::Release);
        assert_eq!(unsafe{observe(7,[address,99,0,0])},FAKE_RESULT); // 외부 체인은 별도
        DOMAINS[7].store(2,Ordering::Release);
        assert_eq!(unsafe{observe(7,[address,99,2,0])},0);           // 예상0과 일치
        for _ in 0..30 {assert_eq!(unsafe{observe(7,[address,99,0,0])},FAKE_RESULT);}
        let s=&state.stats[7];
        assert_eq!(FAKE_CALLS.load(Ordering::Relaxed),35);
        assert_eq!(counter(&s.calls),35);
        assert_eq!(counter(&s.compared),17);
        assert_eq!(counter(&s.matches),1);
        assert_eq!(counter(&s.mismatches),16);
        assert_eq!(counter(&s.unstable),1);
        assert_eq!(counter(&s.skipped),1);
        assert_eq!(counter(&s.foreign_mismatches),1);
        assert_eq!(counter(&s.dropped),12);
        flush().unwrap();
        ORIGINALS[6].store(fake_boolean_original as *const () as usize,Ordering::Release);
        DOMAINS[6].store(1,Ordering::Release);
        assert_eq!(unsafe{observe(6,[0,0,0,0])},0x1122_3344_5566_7700);
        assert_eq!(counter(&state.stats[6].sample_attempts),0); // pending에서는 모델도 읽기도 하지 않는다.
        let player=vec![0u64;0x898/8];let bank=vec![0u64;0x2c8/8];
        let mut data=[0u64;3];data[2]=bank.as_ptr()as u64;
        DOMAINS[6].store(2,Ordering::Release);
        assert_eq!(unsafe{observe(6,[0,player.as_ptr()as u64,data.as_ptr()as u64,0])},0x1122_3344_5566_7700);
        assert_eq!(FAKE_BOOLEAN_CALLS.load(Ordering::Relaxed),2);
        assert_eq!(counter(&state.stats[6].matches),1); // AL=0 일치, 상위56비트는 비교하지 않고 그대로 반환.
        // 쓰기 불가능한 테스트 파일에 writer를 연결해 flush 실패·유실상한을 실제 시험한다.
        *state.writer.lock().unwrap()=BufWriter::new(File::open(dir.join("summary.json")).unwrap());
        assert!(flush().is_err());
        assert_eq!(counter(&state.io_lost_or_uncertain_events),1);
        let path=dir.join(format!("observations_{}.jsonl",state.run_id));
        *state.writer.lock().unwrap()=BufWriter::new(OpenOptions::new().append(true).open(path).unwrap());
        flush().unwrap();
        let text=std::fs::read_to_string(dir.join("summary.json")).unwrap();
        assert!(text.contains("ENGINE_SELFTEST_NOT_A_GAME_SESSION")&&text.contains("\"identity_verified\":false")
            &&text.contains("\"io_lost_or_uncertain_events\":1"));
    }
}
