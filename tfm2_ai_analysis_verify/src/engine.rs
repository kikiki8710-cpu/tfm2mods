//! 게임을 추가 호출하지 않는 관측·독립예상값·비교·비동기 덤프 배선.
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver, TrySendError};
use std::thread::JoinHandle;
use std::time::Duration;
use crate::{config::Config, generated, hook, models, platform};

pub const MAX_FIELDS: usize = 96;
pub const MODEL_COUNT: usize = 19;
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
    persisted:AtomicU64, enqueued:AtomicU64, eligible_calls:AtomicU64,queue_full:AtomicU64, queue_disconnected:AtomicU64,
}
#[derive(Clone)]
struct InstallStatus { id:usize, text:String, foreign_chain:bool, original:usize, target:usize }
#[derive(Clone)]
struct Event {
    id:usize,seq:u64,thread_id:u32,args:[u64;4],actual:u64,
    evaluation:Option<Evaluation>,reads:Vec<Field>,output_fields:Vec<(u64,u8,u64,u64)>,
    comparison:&'static str,reason:&'static str,foreign_chain:bool,
    capture_label:String,scene_code:usize,
}
struct State {
    dir:PathBuf, run_id:String, config:Config, identity_verified:bool, actual_sha256:String,
    stats:[Stats;MODEL_COUNT], install:Mutex<Vec<InstallStatus>>,
    queue:SyncSender<Event>, receiver:Mutex<Receiver<Event>>,writer:Mutex<BufWriter<File>>,flush_lock:Mutex<()>,
    frames:AtomicU64, sequence:AtomicU64, install_tried:AtomicUsize, io_errors:AtomicU64,
    init_ready:AtomicUsize, io_lost_or_uncertain_events:AtomicU64,summary_retries:AtomicU64,last_io_error:Mutex<String>,
    capture_enabled:AtomicUsize,capture_label:OnceLock<String>,control_sequence:AtomicU64,
    scene_code:AtomicUsize,admission:AtomicU64,queued_events:AtomicU64,stop_pending_sequence:AtomicU64,stop_worker:AtomicUsize,worker_state:AtomicUsize,
    worker:Mutex<Option<JoinHandle<()>>>,control_error:Mutex<String>,
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
#[derive(Debug)]
struct AtomicWriteError{stage:&'static str,error:std::io::Error,retries:u64}
fn atomic_error(stage:&'static str,error:std::io::Error)->AtomicWriteError{
    AtomicWriteError{stage,error,retries:0}
}
// 공유 위반성 rename만 최초 시도+최대20회 재시도한다. 예정 sleep 합계 최대100ms이며
// 파일 IO/OS 스케줄링까지 포함한 실제 wall-clock 상한을 보장하는 것은 아니다.
fn write_atomic(path:&Path,text:&str,retry_counter:&AtomicU64)->Result<(),AtomicWriteError>{
    let temp=path.with_extension("json.tmp");
    {
        let mut f=File::create(&temp).map_err(|e|atomic_error("summary_create_temp",e))?;
        f.write_all(text.as_bytes()).map_err(|e|atomic_error("summary_write_temp",e))?;
        f.sync_all().map_err(|e|atomic_error("summary_sync_temp",e))?;
    }
    let mut retries=0;
    loop{
        match std::fs::rename(&temp,path){
            Ok(())=>{
                // 최종 성공한 교체의 추가 시도만 누계한다. 최종 실패는 io_errors로 별도 기록.
                retry_counter.fetch_add(retries,Ordering::Relaxed);return Ok(());
            },
            Err(error) if cfg!(windows)&&retries<20&&matches!(error.raw_os_error(),Some(5|32|33))=>{
                retries+=1;std::thread::sleep(Duration::from_millis(5));
            },
            Err(error)=>return Err(AtomicWriteError{stage:"summary_rename",error,retries}),
        }
    }
}
fn record_io_failure(s:&State,stage:&str,error:&std::io::Error,retries:u64){
    *s.last_io_error.lock().unwrap_or_else(|e|e.into_inner())=format!(
        "stage={stage};os_error={:?};retries={retries};message={error}",error.raw_os_error());
    s.io_errors.fetch_add(1,Ordering::Relaxed);close_capture(s);
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
    let (queue,receiver)=sync_channel(capacity);
    STATE.set(State{dir,run_id,config,identity_verified,actual_sha256,
        stats:std::array::from_fn(|_|Stats::default()),install:Mutex::new(Vec::new()),
        queue,receiver:Mutex::new(receiver),writer:Mutex::new(BufWriter::new(f)),flush_lock:Mutex::new(()),
        frames:AtomicU64::new(0),sequence:AtomicU64::new(0),install_tried:AtomicUsize::new(0),io_errors:AtomicU64::new(0),
        init_ready:AtomicUsize::new(0),io_lost_or_uncertain_events:AtomicU64::new(0),
        summary_retries:AtomicU64::new(0),last_io_error:Mutex::new(String::new()),
        capture_enabled:AtomicUsize::new(0),capture_label:OnceLock::new(),control_sequence:AtomicU64::new(0),
        scene_code:AtomicUsize::new(usize::MAX),admission:AtomicU64::new(ADMISSION_CLOSED),queued_events:AtomicU64::new(0),
        stop_pending_sequence:AtomicU64::new(0),stop_worker:AtomicUsize::new(0),worker_state:AtomicUsize::new(0),
        worker:Mutex::new(None),control_error:Mutex::new(String::new()),
    }).map_err(|_|"duplicate_initialize".to_string())?;
    flush().map_err(|e|e.to_string())?;
    STATE.get().unwrap().init_ready.store(1,Ordering::Release);
    // init/로더 콜백에서는 스레드를 만들지 않는다. 첫 update에서만 시작한다.
    Ok(())
}

const ADMISSION_CLOSED:u64=1u64<<63;
const ADMISSION_COUNT:u64=ADMISSION_CLOSED-1;
fn in_flight(s:&State)->u64{s.admission.load(Ordering::Acquire)&ADMISSION_COUNT}

// closed 비트와 진행 수는 같은 원자어다. stop과 새 관측 진입의 선형화 순서가 정해진다.
fn admit(s:&'static State)->Option<InFlight>{
    let mut seen=s.admission.load(Ordering::Acquire);
    loop {
        if seen&ADMISSION_CLOSED!=0 || seen&ADMISSION_COUNT==ADMISSION_COUNT{return None;}
        match s.admission.compare_exchange_weak(seen,seen+1,Ordering::AcqRel,Ordering::Acquire){
            Ok(_)=>return Some(InFlight(&s.admission)),Err(now)=>seen=now,
        }
    }
}
fn close_capture(s:&State){
    s.admission.fetch_or(ADMISSION_CLOSED,Ordering::AcqRel);
    s.capture_enabled.store(0,Ordering::Release);
}

fn start_worker(s:&'static State)->Result<(),String>{
    let mut slot=s.worker.lock().unwrap_or_else(|e|e.into_inner());
    if s.stop_worker.load(Ordering::Acquire)!=0{return Err("종료 뒤 작업자 재시작 거절".into());}
    if slot.is_some(){return Ok(());}
    if s.worker_state.load(Ordering::Acquire)!=0{return Err("기록 작업자 재시작 거절".into());}
    // disarmed도 worker를 만들므로 handler 설치와 무관하게 DLL 수명을 먼저 pin한다.
    platform::pin_own_module(post_update as *const () as usize).map_err(|e|{
        s.worker_state.store(3,Ordering::Release);s.init_ready.store(0,Ordering::Release);e.to_string()
    })?;
    // 게임 포인터나 host context 없이 owned Event와 파일만 처리한다.
    s.worker_state.store(1,Ordering::Release);
    let created=std::thread::Builder::new().name("aiav-log".into()).spawn(move||{
        loop {
            if s.stop_worker.load(Ordering::Acquire)!=0 && in_flight(s)==0 && counter(&s.queued_events)==0{break;}
            if catch_unwind(AssertUnwindSafe(||{
                if s.stop_worker.load(Ordering::Acquire)==0{process_control();}
                let _=flush();
            })).is_err(){
                close_capture(s);s.io_errors.fetch_add(1,Ordering::Relaxed);
                *s.last_io_error.lock().unwrap_or_else(|e|e.into_inner())="stage=worker_panic;os_error=None".into();
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let _=flush();
        s.worker_state.store(2,Ordering::Release);
    });
    match created{
        Ok(handle)=>{*slot=Some(handle);Ok(())},
        Err(e)=>{s.worker_state.store(3,Ordering::Release);s.init_ready.store(0,Ordering::Release);Err(e.to_string())},
    }
}

#[derive(Debug)]
struct Control<'a>{run:&'a str,sequence:u64,command:&'a str,label:Option<&'a str>}
fn parse_control(text:&str)->Result<Control<'_>,&'static str>{
    if text.len()>16*1024{return Err("control 파일 16KB 초과");}
    let mut run=None;let mut sequence=None;let mut command=None;let mut label=None;
    let mut seen=std::collections::BTreeSet::new();
    for line in text.lines().map(str::trim).filter(|x|!x.is_empty()&&!x.starts_with('#')){
        let(k,v)=line.split_once('=').ok_or("control key=value 필요")?;
        let(k,v)=(k.trim(),v.trim());
        if !seen.insert(k){return Err("control 중복 키");}
        match k{
            "run_id"=>run=Some(v),
            "sequence"=>sequence=Some(v.parse::<u64>().map_err(|_|"control sequence 정수 오류")?),
            "command"=>command=Some(v),"label"=>label=Some(v),_=>return Err("control 알 수 없는 키"),
        }
    }
    Ok(Control{run:run.ok_or("control run_id 누락")?,sequence:sequence.ok_or("control sequence 누락")?,
        command:command.ok_or("control command 누락")?,label})
}

// 실행 ID를 알아야 여는 단일 수집 세션. 이전 실행의 control 파일은 자동 무시한다.
fn process_control() {
    let Some(s)=STATE.get() else{return;};
    // metadata만 믿지 않고 실제 읽기도 제한하여 동시 파일 확장/대형 입력을 제한한다.
    let file=match File::open(s.dir.join("capturectl.cfg")){
        Ok(f)=>f,Err(e) if e.kind()==std::io::ErrorKind::NotFound=>return,
        Err(_)=>{*s.control_error.lock().unwrap_or_else(|e|e.into_inner())="control 파일 읽기 실패".into();return;},
    };
    let mut bytes=Vec::new();
    if file.take(16*1024+1).read_to_end(&mut bytes).is_err(){
        *s.control_error.lock().unwrap_or_else(|e|e.into_inner())="control 파일 읽기 실패".into();return;
    }
    let text=match String::from_utf8(bytes){
        Ok(t)=>t,Err(_)=>{*s.control_error.lock().unwrap_or_else(|e|e.into_inner())="control UTF-8 오류".into();return;},
    };
    let parsed=(||->Result<(),&'static str>{
        let control=parse_control(&text)?;
        if control.run!=s.run_id{return Ok(());}
        let applied=s.control_sequence.load(Ordering::Acquire).max(s.stop_pending_sequence.load(Ordering::Acquire));
        if control.sequence<=applied{return Ok(());}
        match control.command{
            "start"=>{
                let name=control.label.ok_or("control label 누락")?;
                if name.is_empty()||name.len()>200{return Err("control label 길이 오류");}
                if !s.config.armed||!s.identity_verified||s.init_ready.load(Ordering::Acquire)==0{return Err("미준비/disarmed/버전 불일치");}
                if s.stop_worker.load(Ordering::Acquire)!=0{return Err("종료 중 수집 시작 거절");}
                if s.stop_pending_sequence.load(Ordering::Acquire)!=0{return Err("stop 요청 뒤 재시작 금지; 새 실행 필요");}
                if s.capture_label.get().is_some(){return Err("실행당 수집 세션 한 개만 허용; 새 실행 필요");}
                s.capture_label.set(name.to_string()).map_err(|_|"수집 세션 중복")?;
                // 실행당 단일 start. 종료와 경합하면 이미 닫힌 상태를 다시 열지 않는다.
                if s.stop_worker.load(Ordering::Acquire)!=0{return Err("종료 중 수집 시작 거절");}
                s.admission.compare_exchange(ADMISSION_CLOSED,0,Ordering::AcqRel,Ordering::Acquire)
                    .map_err(|_|"수집 진입 상태 오류")?;
                s.capture_enabled.store(1,Ordering::Release);
                // shutdown이 이 사이 닫았으면 mirror도 다시 닫힌 값으로 맞춘다.
                if s.stop_worker.load(Ordering::Acquire)!=0{close_capture(s);return Err("종료 중 수집 시작 거절");}
                s.control_sequence.store(control.sequence,Ordering::Release);
            },
            "stop"=>{
                close_capture(s);
                // ack는 flush가 admission/queue drain을 확인한 뒤에만 게시한다.
                s.stop_pending_sequence.store(control.sequence,Ordering::Release);
            },
            _=>return Err("control command는 start/stop"),
        }
        Ok(())
    })();
    *s.control_error.lock().unwrap_or_else(|e|e.into_inner())=parsed.err().unwrap_or("").into();
}

pub fn set_scene(code:usize){if let Some(s)=STATE.get(){s.scene_code.store(code,Ordering::Release);}}
pub fn shutdown(){
    let Some(s)=STATE.get() else{return;};
    // host on_end 의미는 확인되지 않아 연결하지 않는다. Drop에서는 기다리지 않는다.
    s.stop_worker.store(1,Ordering::Release);close_capture(s);
    // 원본이 UI/host를 기다릴 수도 있으므로 Drop/loader lock 안에서 drain/join 금지.
    // 미완료 handle은 process-lifetime STATE에 남긴다. spawn 전 DLL PIN이 선행한다.
    let handle=match s.worker.try_lock(){
        Ok(mut slot)=>if slot.as_ref().is_some_and(|h|h.is_finished()){slot.take()}else{None},
        Err(_)=>None,
    };
    if let Some(handle)=handle{let _=handle.join();}
}
#[cfg(test)]
fn shutdown_and_join_for_test(){
    shutdown();
    let s=STATE.get().unwrap();
    let handle=s.worker.lock().unwrap().take();
    if let Some(handle)=handle{handle.join().unwrap();}
    flush().unwrap();
}
struct InFlight(&'static AtomicU64);
impl Drop for InFlight{fn drop(&mut self){self.0.fetch_sub(1,Ordering::AcqRel);}}
fn enqueue(s:&State,event:Event)->bool{
    let stats=&s.stats[event.id];
    // 송신 전에 예약하므로 worker가 즉시 drain해도 pending이 underflow하지 않는다.
    s.queued_events.fetch_add(1,Ordering::AcqRel);
    match s.queue.try_send(event){
        Ok(())=>{stats.enqueued.fetch_add(1,Ordering::Relaxed);true},
        Err(TrySendError::Full(_))=>{s.queued_events.fetch_sub(1,Ordering::AcqRel);stats.queue_full.fetch_add(1,Ordering::Relaxed);stats.dropped.fetch_add(1,Ordering::Relaxed);false},
        Err(TrySendError::Disconnected(_))=>{s.queued_events.fetch_sub(1,Ordering::AcqRel);stats.queue_disconnected.fetch_add(1,Ordering::Relaxed);stats.dropped.fetch_add(1,Ordering::Relaxed);false},
    }
}

// 공통 관측 처리. 게임 본체의 CALL은 반드시 한 곳에서 한 번만 수행한다.
unsafe fn observe(id:usize,args:[u64;4])->u64 {
    let original=ORIGINALS[id].load(Ordering::Acquire);
    // 설치 측에서 유효한 trampoline을 선게시한다. 이 불변조건 이전에는 wrapper에 도달할 수 없다.
    let function:unsafe extern "system-unwind" fn(u64,u64,u64,u64)->u64=std::mem::transmute(original);
    let Some(state)=STATE.get() else {return function(args[0],args[1],args[2],args[3]);};
    let stats=&state.stats[id];
    stats.calls.fetch_add(1,Ordering::Relaxed);
    // 화면 타이틀/Loading/정보없음은 수집하지 않고 상한도 소비하지 않는다.
    let scene=state.scene_code.load(Ordering::Acquire);
    if !matches!(scene,4|5){return function(args[0],args[1],args[2],args[3]);}
    let Some(_flight)=admit(state) else{return function(args[0],args[1],args[2],args[3]);};
    // scene 전환을 진입 예약 뒤 다시 확인한다. off/Title은 eligible/상한을 소비하지 않는다.
    if state.scene_code.load(Ordering::Acquire)!=scene{return function(args[0],args[1],args[2],args[3]);}
    if DOMAINS[id].load(Ordering::Acquire)<2 {
        stats.skipped.fetch_add(1,Ordering::Relaxed);
        return function(args[0],args[1],args[2],args[3]);
    }
    let eligible=stats.eligible_calls.fetch_add(1,Ordering::Relaxed);
    // 드문 함수는 수집 시작 뒤 첫64호출을 모두 관측하고, 고빈도 함수는 그 뒤 간격을 둔다.
    if eligible>=64&&eligible%state.config.sample_stride!=0 {return function(args[0],args[1],args[2],args[3]);}
    let ticket=stats.sample_attempts.fetch_add(1,Ordering::Relaxed);
    if ticket>=state.config.max_samples_per_probe {return function(args[0],args[1],args[2],args[3]);}
    let capture_label=state.capture_label.get().cloned().unwrap_or_default();
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
            comparison:"SKIP",reason:"",foreign_chain:DOMAINS[id].load(Ordering::Acquire)==3,
            capture_label:capture_label.clone(),scene_code:scene};
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
        enqueue(state,event);
        Ok::<(),&'static str>(())
    }));
    let failure=match completed {Ok(Ok(()))=>None,Ok(Err(e))=>Some(e),Err(_)=>Some("POSTPROCESSING_PANIC")};
    if let Some(reason)=failure {
        stats.skipped.fetch_add(1,Ordering::Relaxed);stats.processing_errors.fetch_add(1,Ordering::Relaxed);
        let event=Event{id,seq:state.sequence.fetch_add(1,Ordering::Relaxed),thread_id:current_thread_id(),args,actual,
            evaluation:None,reads:Vec::new(),output_fields:Vec::new(),comparison:"SKIP",reason,
            foreign_chain:DOMAINS[id].load(Ordering::Acquire)==3,capture_label,scene_code:scene};
        if !enqueue(state,event){stats.unlogged_errors.fetch_add(1,Ordering::Relaxed);}
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
wrapper!(hook8,8);wrapper!(hook9,9);wrapper!(hook10,10);wrapper!(hook11,11);
wrapper!(hook12,12);wrapper!(hook13,13);wrapper!(hook14,14);wrapper!(hook15,15);
wrapper!(hook16,16);wrapper!(hook17,17);wrapper!(hook18,18);
fn wrappers()->[usize;MODEL_COUNT] {[hook0 as *const () as usize,hook1 as *const () as usize,hook2 as *const () as usize,hook3 as *const () as usize,
    hook4 as *const () as usize,hook5 as *const () as usize,hook6 as *const () as usize,hook7 as *const () as usize,
    hook8 as *const () as usize,hook9 as *const () as usize,hook10 as *const () as usize,hook11 as *const () as usize,
    hook12 as *const () as usize,hook13 as *const () as usize,hook14 as *const () as usize,hook15 as *const () as usize,
    hook16 as *const () as usize,hook17 as *const () as usize,hook18 as *const () as usize]}

pub fn post_update() {
    let Some(state)=STATE.get() else {return;};
    if state.init_ready.load(Ordering::Acquire)==0 {return;}
    if let Err(error)=start_worker(state){
        *state.control_error.lock().unwrap_or_else(|e|e.into_inner())=format!("기록 작업자 시작 실패: {error}");
        let _=flush();return;
    }
    let frame=state.frames.fetch_add(1,Ordering::Relaxed)+1;
    if frame>=state.config.install_delay_frames&&state.install_tried.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).is_ok() {
        let mut statuses=Vec::new();let wrapper=wrappers();
        for p in generated::PROBES {
            let mut entry=InstallStatus{id:p.id,text:"NOT_SELECTED".into(),foreign_chain:false,original:0,target:0};
            if !state.config.armed {entry.text="DISARMED".into();}
            else if !state.identity_verified {entry.text="EXE_IDENTITY_MISMATCH".into();}
            else if state.config.probes.contains(&p.rva) {
                // 고정 원문/ABI가 검토된 모델만 연결하며 범위 밖 id는 거절한다.
                if p.id>=MODEL_COUNT {entry.text="UNSUPPORTED_MODEL_ID".into();statuses.push(entry);continue;}
                let spec=hook::HookSpec{rva:p.rva,prologue:p.prologue,body:p.body,wrapper:wrapper[p.id],
                    allow_foreign_chain:state.config.allow_foreign_chain,branches:p.branches};
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
    }
    // 파일 flush는 UI 프레임이 막혀도 도는 전용 작업자가 담당한다.
}

fn event_json(state:&State,e:&Event)->String {
    let p=generated::PROBES.iter().find(|p|p.id==e.id);
    let (expected,mask)=e.evaluation.as_ref().map(|v|(hexadecimal(v.expected),hexadecimal(v.mask))).unwrap_or(("null".into(),"null".into()));
    let reads=e.reads.iter().map(|f|format!("{{\"address\":{},\"width\":{},\"value\":{}}}",hexadecimal(f.address),f.width,hexadecimal(f.value))).collect::<Vec<_>>().join(",");
    let metrics=e.evaluation.as_ref().map(|v|v.metrics.iter().map(|m|format!("{{\"name\":{},\"lhs\":{},\"rhs\":{},\"relation\":{},\"predicate\":{},\"observed_branch\":{}}}",
        json_quote(m.name),json_quote(&m.lhs.to_string()),json_quote(&m.rhs.to_string()),json_quote(m.relation),m.predicate,m.observed_branch)).collect::<Vec<_>>().join(",")).unwrap_or_default();
    let outputs=e.output_fields.iter().map(|&(a,w,x,y)|format!("{{\"address\":{},\"width\":{},\"expected\":{},\"actual\":{}}}",hexadecimal(a),w,hexadecimal(x),hexadecimal(y))).collect::<Vec<_>>().join(",");
    let original=format!("{{\"schema_version\":1,\"run_id\":{},\"kind\":\"sample\",\"probe_id\":{},\"rva\":{},\"thread_id\":{},\"seq\":{},\"comparison\":{},\"reason\":{},\"domain\":{},\"args\":[{}],\"expected\":{},\"actual\":{},\"mask\":{},\"reads\":[{}],\"metrics\":[{}],\"output_fields\":[{}]}}",
        json_quote(&state.run_id),e.id,hexadecimal(p.map(|p|p.rva as u64).unwrap_or(0)),e.thread_id,e.seq,json_quote(e.comparison),json_quote(e.reason),
        json_quote(if e.foreign_chain{"foreign_chain"}else{"vanilla"}),e.args.iter().map(|&x|hexadecimal(x)).collect::<Vec<_>>().join(","),expected,hexadecimal(e.actual),mask,reads,metrics,outputs);
    format!("{},\"capture_label\":{},\"scene_code\":{}}}",original.trim_end_matches('}'),json_quote(&e.capture_label),e.scene_code)
}

pub fn flush()->std::io::Result<()> {
    let Some(state)=STATE.get() else {return Ok(());};
    let _serial=state.flush_lock.lock().unwrap_or_else(|e|e.into_inner());
    let events:Vec<Event>={let q=state.receiver.lock().unwrap_or_else(|e|e.into_inner());q.try_iter().take(state.config.queue_capacity).collect()};
    state.queued_events.fetch_sub(events.len() as u64,Ordering::AcqRel);
    let written=(|| {let mut writer=state.writer.lock().unwrap_or_else(|e|e.into_inner());
        for e in &events {
            writeln!(writer,"{}",event_json(state,e))?;
        }
        writer.flush()
    })();
    if let Err(error)=written {
        record_io_failure(state,"observations_write_flush",&error,0);
        state.io_lost_or_uncertain_events.fetch_add(events.len() as u64,Ordering::Relaxed);
        return Err(error);
    }
    for e in &events {state.stats[e.id].persisted.fetch_add(1,Ordering::Relaxed);}
    // 이미 닫힌 admission에서는 새 관측 CAS가 성공하지 않는다. 진행/큐가 모두 0인
    // 성공 flush 뒤 stop sequence를 ack하므로 ack 이후 표본이 새로 생길 수 없다.
    let pending=state.stop_pending_sequence.load(Ordering::Acquire);
    if state.admission.load(Ordering::Acquire)&ADMISSION_CLOSED!=0 && in_flight(state)==0
        && counter(&state.queued_events)==0 && pending>state.control_sequence.load(Ordering::Acquire){
        state.control_sequence.store(pending,Ordering::Release);
    }
    let installs=state.install.lock().unwrap_or_else(|e|e.into_inner()).clone();
    let probes=generated::PROBES.iter().map(|p| {
        let s=&state.stats[p.id];let status=installs.iter().find(|x|x.id==p.id);
        let original=format!("{{\"id\":{},\"rva\":{},\"semantic_id\":{},\"install_status\":{},\"foreign_chain\":{},\"original\":{},\"target\":{},\"calls\":{},\"sample_attempts\":{},\"compared\":{},\"matches\":{},\"mismatches\":{},\"skipped\":{},\"unstable\":{},\"model_panics\":{},\"dropped\":{},\"foreign_matches\":{},\"foreign_mismatches\":{},\"processing_errors\":{},\"unlogged_errors\":{},\"verdict\":{}}}",
            p.id,hexadecimal(p.rva as u64),json_quote(p.semantic_id),json_quote(status.map(|x|x.text.as_str()).unwrap_or("WAITING_INSTALL_DELAY")),status.map(|x|x.foreign_chain).unwrap_or(false),
            hexadecimal(status.map(|x|x.original as u64).unwrap_or(0)),hexadecimal(status.map(|x|x.target as u64).unwrap_or(0)),
            counter(&s.calls),counter(&s.sample_attempts),counter(&s.compared),counter(&s.matches),counter(&s.mismatches),counter(&s.skipped),counter(&s.unstable),counter(&s.model_panics),counter(&s.dropped),counter(&s.foreign_matches),counter(&s.foreign_mismatches),counter(&s.processing_errors),counter(&s.unlogged_errors),
            json_quote(if counter(&s.compared)==0 {"NO_SAMPLES"} else if counter(&s.mismatches)>0 {"MISMATCH_OBSERVED"} else {"OBSERVED_SAMPLES_MATCH_ONLY"}));
        format!("{},\"persisted\":{},\"enqueued\":{},\"eligible_calls\":{},\"queue_full\":{},\"queue_disconnected\":{}}}",original.trim_end_matches('}'),counter(&s.persisted),counter(&s.enqueued),counter(&s.eligible_calls),counter(&s.queue_full),counter(&s.queue_disconnected))
    }).collect::<Vec<_>>().join(",");
    let summary=format!("{{\"schema_version\":1,\"run_id\":{},\"baseline_sha256\":{},\"actual_sha256\":{},\"identity_verified\":{},\"armed\":{},\"catalog_total\":{},\"prepared_probes\":{},\"io_errors\":{},\"io_lost_or_uncertain_events\":{},\"sample_stride\":{},\"max_samples_per_probe\":{},\"probes\":[{}],\"scope\":\"관측표본의출력대조만. 실제표본0은검증불성립; 전체분기·모든도메인·임계값독립조절가능성증명아님\"}}\n",
        json_quote(&state.run_id),json_quote(generated::BASELINE_EXE_SHA256),json_quote(&state.actual_sha256),state.identity_verified,state.config.armed,generated::CENSUS_FUNCTIONS,generated::PROBES.len(),counter(&state.io_errors),counter(&state.io_lost_or_uncertain_events),state.config.sample_stride,state.config.max_samples_per_probe,probes);
    let summary=format!("{},\"capture_enabled\":{},\"capture_label\":{},\"control_sequence\":{},\"control_error\":{},\"in_flight\":{},\"scene_code\":{}}}\n",
        summary.trim().trim_end_matches('}'),state.capture_enabled.load(Ordering::Acquire)!=0,
        json_quote(state.capture_label.get().map(String::as_str).unwrap_or("")),counter(&state.control_sequence),
        json_quote(&state.control_error.lock().unwrap_or_else(|e|e.into_inner())),in_flight(state),state.scene_code.load(Ordering::Acquire));
    let queue_pending=counter(&state.queued_events);
    let summary=format!("{},\"queued_events\":{},\"queue_pending\":{},\"stop_pending_sequence\":{},\"worker_state\":{}}}\n",
        summary.trim().trim_end_matches('}'),queue_pending,queue_pending,counter(&state.stop_pending_sequence),state.worker_state.load(Ordering::Acquire));
    let summary=format!("{},\"summary_retries\":{},\"last_io_error\":{},\"init_ready\":{}}}\n",
        summary.trim().trim_end_matches('}'),counter(&state.summary_retries),
        json_quote(&state.last_io_error.lock().unwrap_or_else(|e|e.into_inner())),state.init_ready.load(Ordering::Acquire)!=0);
    match write_atomic(&state.dir.join("summary.json"),&summary,&state.summary_retries){
        Ok(())=>Ok(()),
        Err(failure)=>{
            record_io_failure(state,failure.stage,&failure.error,failure.retries);Err(failure.error)
        },
    }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn quotes_cover_json_controls() {
        assert_eq!(json_quote("\"\\\n\t\u{1}가"),"\"\\\"\\\\\\n\\t\\u0001가\"");
    }
    #[test] fn missing_snapshot_is_an_error() {
        let mut r=Replay(&[]);assert!(r.read(0x10000,8).is_err());
    }
    #[test] fn control_parser_rejects_duplicates_and_requires_complete_keys(){
        let valid="run_id=one\nsequence=2\ncommand=start\nlabel=match";
        assert!(parse_control(&"x".repeat(16*1024+1)).is_err());
        let parsed=parse_control(valid).unwrap();
        assert_eq!((parsed.run,parsed.sequence,parsed.command,parsed.label),("one",2,"start",Some("match")));
        for duplicate in ["run_id=one","sequence=3","command=stop","label=other"]{
            assert!(parse_control(&format!("{valid}\n{duplicate}")).is_err());
        }
        for bad in ["sequence=1\ncommand=stop","run_id=x\ncommand=stop","run_id=x\nsequence=1","run_id=x\nsequence=no\ncommand=stop"]{
            assert!(parse_control(bad).is_err());
        }
    }
    #[cfg(windows)]
    #[test] fn summary_share_conflict_retry_success_and_bounded_failure(){
        use std::os::windows::fs::OpenOptionsExt;
        let dir=std::env::temp_dir().join(format!("aiav_summary_retry_{}_{}",std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&dir).unwrap();let summary=dir.join("summary.json");
        std::fs::write(&summary,"old").unwrap();let retries=AtomicU64::new(0);
        // Rust 기본 open의 share-delete 대신 READ|WRITE만 주어 Python reader 충돌을 재현.
        let held=OpenOptions::new().read(true).share_mode(1|2).open(&summary).unwrap();
        let release=std::thread::spawn(move||{std::thread::sleep(Duration::from_millis(25));drop(held);});
        write_atomic(&summary,"replaced",&retries).unwrap();release.join().unwrap();
        assert!(counter(&retries)>0&&counter(&retries)<=20);
        assert_eq!(std::fs::read_to_string(&summary).unwrap(),"replaced");
        let successful_retries=counter(&retries);
        let held=OpenOptions::new().read(true).share_mode(1|2).open(&summary).unwrap();
        let began=std::time::Instant::now();let failure=write_atomic(&summary,"must_not_replace",&retries).unwrap_err();
        assert_eq!(failure.stage,"summary_rename");assert_eq!(failure.retries,20);
        assert!(matches!(failure.error.raw_os_error(),Some(5|32|33)));
        assert!(began.elapsed()>=Duration::from_millis(100));
        assert_eq!(counter(&retries),successful_retries); // 실패의20회는 성공retry누계에 섞지 않는다.
        drop(held);assert_eq!(std::fs::read_to_string(&summary).unwrap(),"replaced");
        assert!(summary.with_extension("json.tmp").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }
    static BLOCK_ENTERED:AtomicUsize=AtomicUsize::new(0);
    static BLOCK_RELEASE:AtomicUsize=AtomicUsize::new(0);
    unsafe extern "system-unwind" fn fake_blocking_original(_a:u64,_b:u64,c:u64,_d:u64)->u64{
        if c==3{
            BLOCK_ENTERED.store(1,Ordering::Release);
            let deadline=std::time::Instant::now()+Duration::from_secs(10);
            while BLOCK_RELEASE.load(Ordering::Acquire)==0{
                if std::time::Instant::now()>=deadline{break;}
                std::thread::yield_now();
            }
        }
        0
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
        let configuration=Config{armed:true,sample_stride:1,max_samples_per_probe:20,queue_capacity:8,..Config::default()};
        assert!(initialize(dir.clone(),configuration,"ENGINE_SELFTEST_NOT_A_GAME_SESSION".into(),true).is_err());
        let state=STATE.get().unwrap();
        for _ in 0..200 {post_update();}
        assert_eq!(state.install_tried.load(Ordering::Acquire),0);
        assert!(ORIGINALS.iter().all(|x|x.load(Ordering::Acquire)==0));
        std::fs::remove_dir(dir.join("summary.json")).unwrap();
        platform::init_safe_reader().unwrap();
        assert!(state.worker.lock().unwrap().is_none());
        assert_eq!(state.worker_state.load(Ordering::Acquire),0);
        assert_eq!(counter(&state.queued_events),0);
        // init I/O 실패 뒤에는 worker도 설치도 시작하지 않았다. 이후는 가짜 원본 전용 수동 fixture다.
        state.init_ready.store(1,Ordering::Release);
        state.install_tried.store(1,Ordering::Release); // 이후 update도 실제 게임 패치는 금지.
        let control=dir.join("capturectl.cfg");
        std::fs::write(&control,format!("run_id=stale-run\nsequence=99\ncommand=start\nlabel=stale")).unwrap();
        process_control();assert!(state.capture_label.get().is_none());assert_eq!(counter(&state.control_sequence),0);
        set_scene(4);
        // 아래는 설치 없이 가짜 원본에만 연결하는 시험이다. 실제 게임 원문 변경은 없다.
        ORIGINALS[7].store(fake_original as *const () as usize,Ordering::Release);
        DOMAINS[7].store(2,Ordering::Release);
        let mut own=vec![0u64;0x848/8];let address=own.as_mut_ptr()as u64;
        // 수집 OFF이면 eligible/attempt/상한 모두 0이고 원본 반환은 유지한다.
        assert_eq!(unsafe{observe(7,[address,99,0,0])},FAKE_RESULT);
        assert_eq!(counter(&state.stats[7].eligible_calls),0);assert_eq!(counter(&state.stats[7].sample_attempts),0);
        std::fs::write(&control,format!("run_id={}\nsequence=1\ncommand=start\nlabel=SELFTEST",state.run_id)).unwrap();
        process_control();assert_eq!(counter(&state.control_sequence),1);
        assert_eq!(state.capture_enabled.load(Ordering::Acquire),1);
        set_scene(1);
        assert_eq!(unsafe{observe(7,[address,99,0,0])},FAKE_RESULT);
        assert_eq!(counter(&state.stats[7].eligible_calls),0);assert_eq!(counter(&state.stats[7].sample_attempts),0);
        assert_eq!(in_flight(state),0);set_scene(4);
        // 기존 원본1회·20표본·큐유실 fixture의 집계를 독립 구간으로 유지한다.
        FAKE_CALLS.store(0,Ordering::Relaxed);state.stats[7].calls.store(0,Ordering::Relaxed);

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
        assert!(state.last_io_error.lock().unwrap().contains("stage=observations_write_flush;os_error="));
        assert_eq!(state.capture_enabled.load(Ordering::Acquire),0);assert!(admit(state).is_none());
        let path=dir.join(format!("observations_{}.jsonl",state.run_id));
        *state.writer.lock().unwrap()=BufWriter::new(OpenOptions::new().append(true).open(path).unwrap());
        flush().unwrap();
        let text=std::fs::read_to_string(dir.join("summary.json")).unwrap();
        assert!(text.contains("ENGINE_SELFTEST_NOT_A_GAME_SESSION")&&text.contains("\"identity_verified\":true")
            &&text.contains("\"io_lost_or_uncertain_events\":1"));
        assert_eq!(counter(&state.queued_events),0);
        // 아래 별도 경합 축은 게임 재시작 없이 단일 STATE를 재사용하는 수동 시험 fixture다.
        // 실제 control 경로는 IO 오류 뒤 같은 label로 다시 start할 수 없다.
        state.admission.store(0,Ordering::Release);state.capture_enabled.store(1,Ordering::Release);
        // stop보다 먼저 등록된 표본을 가짜 원본 안에서 멈춰 ack 지연을 확인한다.
        ORIGINALS[7].store(fake_blocking_original as *const () as usize,Ordering::Release);
        state.stats[7].sample_attempts.store(0,Ordering::Relaxed);
        let delayed=std::thread::spawn(move||unsafe{observe(7,[address,99,3,0])});
        let deadline=std::time::Instant::now()+Duration::from_secs(5);
        while BLOCK_ENTERED.load(Ordering::Acquire)==0{
            assert!(std::time::Instant::now()<deadline,"가짜 원본 진입 timeout");std::thread::yield_now();
        }
        assert_eq!(in_flight(state),1);
        // gate를 예전 값으로 읽은 진입도 stop 뒤 CAS를 성공시킬 수 없다.
        let stale_admission=state.admission.load(Ordering::Acquire);
        std::fs::write(&control,format!("run_id={}\nsequence=2\ncommand=stop",state.run_id)).unwrap();
        process_control();assert_eq!(state.capture_enabled.load(Ordering::Acquire),0);
        assert!(state.admission.compare_exchange(stale_admission,stale_admission+1,Ordering::AcqRel,Ordering::Acquire).is_err());
        assert!(admit(state).is_none());flush().unwrap();
        assert_eq!(counter(&state.control_sequence),1);assert_eq!(counter(&state.stop_pending_sequence),2);
        let attempts=counter(&state.stats[7].sample_attempts);
        std::fs::write(&control,"x".repeat(16*1024+200)).unwrap();process_control();
        assert!(state.control_error.lock().unwrap().contains("16KB"));assert_eq!(counter(&state.control_sequence),1);
        assert_eq!(unsafe{observe(7,[address,99,0,0])},0);
        assert_eq!(counter(&state.stats[7].sample_attempts),attempts);
        BLOCK_RELEASE.store(1,Ordering::Release);assert_eq!(delayed.join().unwrap(),0);
        assert_eq!(in_flight(state),0);assert_eq!(counter(&state.queued_events),1);
        assert_eq!(counter(&state.control_sequence),1);flush().unwrap();
        assert_eq!(counter(&state.control_sequence),2);assert_eq!(counter(&state.queued_events),0);
        // stop ack 뒤 gate가 닫혀 있고, 같은 run의 오래된 제어도 다시 열지 않는다.
        std::fs::write(&control,format!("run_id={}\nsequence=1\ncommand=start\nlabel=stale",state.run_id)).unwrap();
        process_control();assert_eq!(counter(&state.control_sequence),2);assert!(admit(state).is_none());
        for _ in 0..8{assert_eq!(unsafe{observe(7,[address,99,0,0])},0);}
        assert_eq!(counter(&state.stats[7].sample_attempts),attempts);assert_eq!(counter(&state.queued_events),0);
        // enqueue의 자기 송신 결과만으로 unlogged를 판정한다. 다른 발화의 drop 증가는 읽지 않는다.
        let event=Event{id:7,seq:9999,thread_id:0,args:[0;4],actual:0,evaluation:None,reads:Vec::new(),
            output_fields:Vec::new(),comparison:"SKIP",reason:"SELFTEST",foreign_chain:false,capture_label:"SELFTEST".into(),scene_code:4};
        assert!(enqueue(state,event.clone()));
        state.stats[7].dropped.fetch_add(1,Ordering::Relaxed); // 독립 다른 발화의 유실
        assert_eq!(counter(&state.stats[7].unlogged_errors),0);
        for _ in 1..8{assert!(enqueue(state,event.clone()));}
        assert!(!enqueue(state,event.clone()));assert_eq!(counter(&state.queued_events),8);
        flush().unwrap();assert_eq!(counter(&state.queued_events),0);
        // 첫 update에서만 기록 worker 시작. 시작에 성공하기 전 실제 설치 불가.
        assert!(state.worker.lock().unwrap().is_none());post_update();
        assert!(state.worker.lock().unwrap().is_some());assert_eq!(state.worker_state.load(Ordering::Acquire),1);
        assert!(enqueue(state,event));shutdown_and_join_for_test();
        assert!(state.worker.lock().unwrap().is_none());assert_eq!(state.worker_state.load(Ordering::Acquire),2);
        assert_eq!(in_flight(state),0);assert_eq!(counter(&state.queued_events),0);
        assert!(start_worker(state).is_err());shutdown(); // 재시작 거절/종료 멱등

    }
}
