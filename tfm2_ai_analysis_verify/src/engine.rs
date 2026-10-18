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
pub const MODEL_COUNT: usize = 41;
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
    stats:[Stats;MODEL_COUNT], probe_strides:[u64;MODEL_COUNT], install:Mutex<Vec<InstallStatus>>,
    queue:SyncSender<Event>, receiver:Mutex<Receiver<Event>>,writer:Mutex<BufWriter<File>>,summary_journal:Mutex<BufWriter<File>>,flush_lock:Mutex<()>,
    frames:AtomicU64, sequence:AtomicU64, install_tried:AtomicUsize, io_errors:AtomicU64,
    init_ready:AtomicUsize, io_lost_or_uncertain_events:AtomicU64,summary_retries:AtomicU64,last_io_error:Mutex<String>,
    summary_record_sequence:AtomicU64,journal_failed:AtomicUsize,summary_alias_errors:AtomicU64,last_summary_alias_error:Mutex<String>,
    capture_enabled:AtomicUsize,capture_label:OnceLock<String>,control_sequence:AtomicU64,
    scene_code:AtomicUsize,admission:AtomicU64,queued_events:AtomicU64,stop_pending_sequence:AtomicU64,stop_worker:AtomicUsize,worker_state:AtomicUsize,
    worker:Mutex<Option<JoinHandle<()>>>,control_error:Mutex<String>,
    first_mismatch:OnceLock<Event>,first_mismatch_file_state:AtomicUsize,
}
static STATE:OnceLock<State>=OnceLock::new();

// 모델별 원문 쓰기 surface를 등록한다. 모델이 임의 주소를 출력이라고 선언할 수 없다.
fn checked_span(address:u64,width:u64)->Result<(u64,u64),&'static str>{
    let end=address.checked_add(width).ok_or("OUTPUT_ADDRESS_OVERFLOW")?;
    if address<0x10000||width==0||end>(1u64<<48){return Err("OUTPUT_ADDRESS_RANGE");}
    Ok((address,end))
}
fn spans_overlap(a:(u64,u64),b:(u64,u64))->bool{a.0<b.1&&b.0<a.1}

// wave06 typed policy: output schema support와 entry ABI 승인은 별개다.
fn validate_wave06_outputs(id:usize,args:[u64;4],e:&Evaluation,reads:&[Field])->Result<(),&'static str>{
    if !matches!(id,37|38){return Err("W06_OUTPUT_ID");}
    if e.output_ptr!=Some(args[0]){return Err("W06_OUTPUT_PTR");}
    let out=checked_span(args[0],if id==37{24}else{32})?;
    let fields=&e.output_fields;
    let mut cursor=None;
    let shape:Vec<(u64,u8)>=if id==37{
        if fields.len()==1{
            if fields[0].2!=2||e.mask!=0{return Err("W06_EFFECT_NONE_TAG_OR_MASK");}
            vec![(args[0]+16,1)]
        }else{
            if fields.len()!=3||fields[2].2>1||e.mask!=u64::MAX||e.expected!=fields[0].2{return Err("W06_EFFECT_SOME_FIELDS");}
            vec![(args[0],8),(args[0]+8,8),(args[0]+16,1)]
        }
    }else{
        if e.mask!=u64::MAX||e.expected!=args[0]{return Err("W06_ITER_RETURN_CONTRACT");}
        let cur=args[1].checked_add(8).ok_or("W06_CURSOR_ADDRESS_OVERFLOW")?;
        let cs=checked_span(cur,8)?;if spans_overlap(cs,out){return Err("W06_OUT_CURSOR_ALIAS");}
        if reads.first().map(|f|(f.address,f.width))!=Some((cur,8)){return Err("W06_CURSOR_FIRST_READ");}
        let start=reads[0].value;
        let end_address=args[1].checked_add(16).ok_or("W06_END_ADDRESS_OVERFLOW")?;
        let end=reads.iter().find(|f|f.address==end_address&&f.width==8).ok_or("W06_END_READ")?.value;
        match fields.len(){
            1=>{if start!=end||fields[0].2!=0{return Err("W06_EMPTY_ITERATOR_SHAPE");}vec![(args[0],8)]},
            2=>{
                if start>=end||fields[0].2!=end||fields[1].2!=0{return Err("W06_EXHAUSTED_ITERATOR_SHAPE");}
                cursor=Some(cur);vec![(cur,8),(args[0],8)]
            },
            5=>{
                if start>=end||fields[0].2<=start||fields[0].2>end||fields[4].2!=1{return Err("W06_SOME_ITERATOR_SHAPE");}
                cursor=Some(cur);vec![(cur,8),(args[0]+8,8),(args[0]+16,8),(args[0]+24,8),(args[0],8)]
            },
            _=>return Err("W06_ITERATOR_FIELD_COUNT"),
        }
    };
    if fields.len()!=shape.len(){return Err("W06_FIELD_COUNT");}
    for(&(address,width,_),&(expected_address,expected_width))in fields.iter().zip(shape.iter()){
        if address!=expected_address||width!=expected_width{return Err("W06_TYPED_FIELD_ADDRESS_WIDTH_ORDER");}
        checked_span(address,width as u64)?;
    }
    // 실제등록된byte만쓰기다. None미작성payload/padding은안정성검사에서제외하지않는다.
    for(j,f)in reads.iter().enumerate(){
        if !matches!(f.width,1|2|4|8){return Err("W06_INPUT_WIDTH");}
        let span=checked_span(f.address,f.width as u64)?;
        for &(a,w,_)in fields{
            if spans_overlap(span,checked_span(a,w as u64)?)
                && !(j==0&&cursor==Some(f.address)&&f.width==8&&a==f.address&&w==8){return Err("W06_REGISTERED_WRITE_INPUT_ALIAS");}
        }
    }
    Ok(())
}

// v026 제안: runtime39=D8A1F0, runtime40=D3EEC0. 설치 선택/정적 ABI 승인은 별도다.
fn validate_insert_outputs(a:[u64;4],e:&Evaluation,reads:&[Field])->Result<(),&'static str>{
    let(begin,pos,header)=(a[0],a[1],a[2]);
    if pos<=begin||(pos-begin)%8!=0{return Err("INSERT_POLICY_PREDECESSOR_OR_ALIGNMENT");}
    let count=(pos-begin)/8+1;
    if count>20{return Err("INSERT_POLICY_CAPTURE_CAP");}
    let indices=checked_span(begin,count*8)?;
    let hlen=header.checked_add(8).ok_or("INSERT_POLICY_ADDRESS_OVERFLOW")?;
    if spans_overlap(indices,checked_span(header,16)?){return Err("INSERT_POLICY_HEADER_ALIAS");}
    let read=|p:u64|->Result<u64,&'static str>{
        reads.iter().find(|f|f.address==p&&f.width==8).map(|f|f.value).ok_or("INSERT_POLICY_MISSING_SNAPSHOT")
    };
    if reads.len()<3||reads[0].address!=pos||reads[0].width!=8
        ||reads[1].address!=hlen||reads[1].width!=8
        ||reads[2].address!=pos-8||reads[2].width!=8{return Err("INSERT_POLICY_READ_ORDER");}
    let key=read(pos)?;let len=read(hlen)?;let scores=read(header)?;
    if key>=len||e.mask!=u64::MAX{return Err("INSERT_POLICY_KEY_OR_MASK");}
    let key_score=read(scores.wrapping_add(key.wrapping_mul(8)))?;
    let should_shift=|pred:u64|->Result<bool,&'static str>{
        let ps=read(scores.wrapping_add(pred.wrapping_mul(8)))?;
        Ok((ps as i64)<(key_score as i64)||(ps==key_score&&key<pred))
    };
    // 등록된 index read와 score/header의 readonly 역할을 구별한다.
    for f in reads{
        if f.width!=8{return Err("INSERT_POLICY_WIDTH");}
        let p=checked_span(f.address,8)?;
        if spans_overlap(p,indices){
            if f.address<begin||f.address>pos||(f.address-begin)%8!=0||f.value>=len{return Err("INSERT_POLICY_INDEX_READ");}
            let score=scores.wrapping_add(f.value.wrapping_mul(8));
            if spans_overlap(checked_span(score,8)?,indices){return Err("INSERT_POLICY_SCORE_ALIAS");}
        }
    }
    let fields=&e.output_fields;
    let last=if fields.is_empty(){
        if e.output_ptr.is_some()||should_shift(read(pos-8)?)?{return Err("INSERT_POLICY_NO_WRITE_PTR_OR_COMPARATOR");}pos
    }else{
        if e.output_ptr!=Some(begin)||fields.len()<2||fields.len()>count as usize{return Err("INSERT_POLICY_FIELD_COUNT_OR_PTR");}
        for(j,&(address,width,value))in fields.iter().enumerate(){
            let expected_address=pos-j as u64*8;
            let expected_value=if j+1==fields.len(){key}else{read(expected_address-8)?};
            if address!=expected_address||width!=8||value!=expected_value{return Err("INSERT_POLICY_FIELD_ADDRESS_WIDTH_OR_VALUE");}
            if j+1<fields.len()&&!should_shift(value)?{return Err("INSERT_POLICY_UNEXPECTED_SHIFT");}
        }
        let last=fields.last().unwrap().0;
        if last>begin&&should_shift(read(last-8)?)?{return Err("INSERT_POLICY_PREMATURE_STOP");}last
    };
    let last_pred=if last==begin{begin}else{last-8};
    if e.expected!=read(last_pred)?{return Err("INSERT_POLICY_RESIDUAL_RAX");}
    Ok(())
}
fn validate_seeded_peer_outputs(a:[u64;4],e:&Evaluation,reads:&[Field])->Result<(),&'static str>{
    let out=checked_span(a[0],24)?;
    if e.expected!=a[0]||e.mask!=u64::MAX||e.output_ptr!=Some(a[0])||e.output_fields.len()!=3{return Err("PEER_POLICY_RETURN_OR_COUNT");}
    if reads.len()<2||reads[0].address!=a[0]||reads[0].width!=8||reads[1].address!=a[0]+8||reads[1].width!=8{return Err("PEER_POLICY_SEED_READ_ORDER");}
    let fields=&e.output_fields;
    for(&(address,width,_),&(expected_address,expected_width))in fields.iter().zip([(a[0]+8,8),(a[0],8),(a[0]+16,1)].iter()){
        if address!=expected_address||width!=expected_width{return Err("PEER_POLICY_TYPED_FIELDS");}
        checked_span(address,width as u64)?;
    }
    let line=fields[2].2;
    if line>255{return Err("PEER_POLICY_LINE_BYTE");}
    if line==255{
        if fields[0].2!=reads[1].value||fields[1].2!=reads[0].value{return Err("PEER_POLICY_NO_CANDIDATE_SEEDS");}
    }else if fields[0].2>=5{return Err("PEER_POLICY_CANDIDATE_ID");}
    for(j,f)in reads.iter().enumerate(){
        if !matches!(f.width,1|2|4|8){return Err("PEER_POLICY_INPUT_WIDTH");}
        let s=checked_span(f.address,f.width as u64)?;
        if j>=2&&spans_overlap(s,out){return Err("PEER_POLICY_NON_SEED_ALIAS");}
    }
    Ok(())
}

fn validate_registered_outputs(id:usize,args:[u64;4],e:&Evaluation,reads:&[Field])->Result<(),&'static str>{
    if id==39{return validate_insert_outputs(args,e,reads);}
    if id==40{return validate_seeded_peer_outputs(args,e,reads);}
    if id>=37{return validate_wave06_outputs(id,args,e,reads);}
    let (base,count)=match id{
        0..=28=>{if e.output_ptr.is_some()||!e.output_fields.is_empty(){return Err("UNREGISTERED_OUTPUT");}return Ok(());},
        29=>(args[0],3),30|31=>(args[1],4),
        32..=36=>{
            if e.output_fields.is_empty(){if e.output_ptr.is_some(){return Err("EMPTY_CURSOR_OUTPUT_PTR");}return Ok(());}
            (args[0],1)
        },
        _=>return Err("UNREGISTERED_OUTPUT_MODEL_ID"),
    };
    let surface=checked_span(base,count*8)?;
    if e.output_ptr!=Some(base)||e.output_fields.len()!=count as usize{return Err("OUTPUT_SURFACE_COUNT_OR_PTR");}
    for(j,&(address,width,_))in e.output_fields.iter().enumerate(){
        if width!=8||address!=base+j as u64*8{return Err("OUTPUT_SURFACE_ADDRESS_OR_WIDTH");}
    }
    // readonly 출력3은 입력과alias되면 미지원이다. cursor는 첫 header Q8 read만 예외다.
    for(j,f)in reads.iter().enumerate(){
        if !matches!(f.width,1|2|4|8){return Err("INPUT_FIELD_WIDTH");}
        if spans_overlap(checked_span(f.address,f.width as u64)?,surface)
            && !(((32..=36).contains(&id))&&j==0&&f.address==args[0]&&f.width==8){
            return Err("REGISTERED_OUTPUT_READ_ALIAS");
        }
    }
    Ok(())
}
fn unchanged_outside_registered_writes(address:u64,width:u8,before:u64,after:u64,outputs:&[(u64,u8,u64)])->Result<bool,&'static str>{
    if !matches!(width,1|2|4|8){return Err("INPUT_FIELD_WIDTH");}
    checked_span(address,width as u64)?;
    for byte in 0..width{
        let at=address+byte as u64;
        let mut written=false;
        for &(a,w,_)in outputs{
            let span=checked_span(a,w as u64)?;
            if at>=span.0&&at<span.1{written=true;break;}
        }
        if !written&&((before>>(byte*8))&255)!=((after>>(byte*8))&255){return Ok(false);}
    }
    Ok(true)
}
fn write_first_mismatch_new(path:&Path,text:&str)->std::io::Result<()>{
    let mut file=OpenOptions::new().create_new(true).write(true).open(path)?;
    writeln!(file,"{text}")?;file.flush()?;file.sync_all()
}
fn persist_first_mismatch(state:&State)->std::io::Result<()>{
    match state.first_mismatch_file_state.load(Ordering::Acquire){
        1=>return Ok(()),2=>return Err(std::io::Error::other("first mismatch file disabled after hard failure; restart required")),_=>{}
    }
    let Some(event)=state.first_mismatch.get()else{return Ok(());};
    // create_new 또는 부분 write 실패 뒤 같은 파일을 이어쓰거나 덮어쓰지 않는다.
    state.first_mismatch_file_state.store(2,Ordering::Release);
    let result=write_first_mismatch_new(&state.dir.join(format!("first_mismatch_{}.json",state.run_id)),&event_json(state,event));
    if let Err(error)=result{record_io_failure(state,"first_mismatch_create_write_flush",&error,0);return Err(error);}
    state.first_mismatch_file_state.store(1,Ordering::Release);Ok(())
}

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
// 파일 교체 helper의 재시도 예산은 호출자가 정한다(runtime alias=0, 구 경계시험=20).
// 파일 IO/OS 스케줄링까지 포함한 실제 wall-clock 상한을 보장하는 것은 아니다.
fn write_atomic(path:&Path,text:&str,retry_counter:&AtomicU64,max_retries:u64)->Result<(),AtomicWriteError>{
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
            Err(error) if cfg!(windows)&&retries<max_retries&&matches!(error.raw_os_error(),Some(5|32|33))=>{
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

fn resolve_probe_strides(config:&Config)->Result<[u64;MODEL_COUNT],String> {
    if !(1..=1_000_000).contains(&config.sample_stride) { return Err("sample_stride 범위 오류".into()); }
    let mut strides=[config.sample_stride;MODEL_COUNT];
    for (&rva,&stride) in &config.probe_strides {
        if !config.probes.contains(&rva) { return Err(format!("선택되지 않은 probe_strides RVA: 0x{rva:x}")); }
        if !(1..=1_000_000).contains(&stride) { return Err("probe_strides 범위 오류".into()); }
        let p=generated::PROBES.iter().find(|p|p.rva==rva).ok_or("probe_strides 미지원 RVA")?;
        if p.id>=MODEL_COUNT { return Err("probe_strides model id 범위 오류".into()); }
        strides[p.id]=stride;
    }
    Ok(strides)
}

fn validate_probe_selection(config:&Config)->Result<(),String>{
    for rva in &config.probes {
        if !generated::PROBES.iter().any(|p|p.rva==*rva){return Err(format!("미지원/진입불가 probe: 0x{rva:x}"));}
    }Ok(())
}

pub fn initialize(dir:PathBuf, config:Config,actual_sha256:String,identity_verified:bool)->Result<(),String> {
    let mut ids=std::collections::BTreeSet::new();let mut rvas=std::collections::BTreeSet::new();
    if generated::PROBES.len()!=MODEL_COUNT{return Err("모델/진입정의 개수 불일치".into());}
    for p in generated::PROBES {
        if p.id>=MODEL_COUNT||!ids.insert(p.id)||!rvas.insert(p.rva){return Err("진입정의 id/RVA 중복 또는 범위 오류".into());}
    }
    let probe_strides=resolve_probe_strides(&config)?;
    validate_probe_selection(&config)?;
    std::fs::create_dir_all(&dir).map_err(|e|e.to_string())?;
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?.as_nanos();
    let run_id=format!("{}_{stamp}",std::process::id());
    if config.armed && identity_verified && actual_sha256==generated::BASELINE_EXE_SHA256 {
        crate::loaded_constants::capture(&dir,&run_id)?;
    }
    let output=dir.join(format!("observations_{run_id}.jsonl"));
    let f=OpenOptions::new().create_new(true).write(true).open(output).map_err(|e|e.to_string())?;
    let journal_output=dir.join(format!("summaries_{run_id}.jsonl"));
    let journal=OpenOptions::new().create_new(true).append(true).open(journal_output).map_err(|e|e.to_string())?;
    let capacity=config.queue_capacity;
    let (queue,receiver)=sync_channel(capacity);
    STATE.set(State{dir,run_id,config,identity_verified,actual_sha256,
        stats:std::array::from_fn(|_|Stats::default()),probe_strides,install:Mutex::new(Vec::new()),
        queue,receiver:Mutex::new(receiver),writer:Mutex::new(BufWriter::new(f)),summary_journal:Mutex::new(BufWriter::new(journal)),flush_lock:Mutex::new(()),
        frames:AtomicU64::new(0),sequence:AtomicU64::new(0),install_tried:AtomicUsize::new(0),io_errors:AtomicU64::new(0),
        init_ready:AtomicUsize::new(0),io_lost_or_uncertain_events:AtomicU64::new(0),
        summary_retries:AtomicU64::new(0),last_io_error:Mutex::new(String::new()),
        summary_record_sequence:AtomicU64::new(0),journal_failed:AtomicUsize::new(0),summary_alias_errors:AtomicU64::new(0),last_summary_alias_error:Mutex::new(String::new()),
        capture_enabled:AtomicUsize::new(0),capture_label:OnceLock::new(),control_sequence:AtomicU64::new(0),
        scene_code:AtomicUsize::new(usize::MAX),admission:AtomicU64::new(ADMISSION_CLOSED),queued_events:AtomicU64::new(0),
        stop_pending_sequence:AtomicU64::new(0),stop_worker:AtomicUsize::new(0),worker_state:AtomicUsize::new(0),
        worker:Mutex::new(None),control_error:Mutex::new(String::new()),
        first_mismatch:OnceLock::new(),first_mismatch_file_state:AtomicUsize::new(0),
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
                if !s.config.armed||!s.identity_verified||s.init_ready.load(Ordering::Acquire)==0||counter(&s.io_errors)!=0{return Err("미준비/disarmed/버전 불일치");}
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
    if eligible>=64&&eligible%state.probe_strides[id]!=0 {return function(args[0],args[1],args[2],args[3]);}
    let ticket=stats.sample_attempts.fetch_add(1,Ordering::Relaxed);
    if ticket>=state.config.max_samples_per_probe {return function(args[0],args[1],args[2],args[3]);}
    let capture_label=state.capture_label.get().cloned().unwrap_or_default();
    let prepared=catch_unwind(AssertUnwindSafe(|| {
        let mut capture=Capture{reads:Vec::with_capacity(MAX_FIELDS)};
        let evaluation=models::evaluate(id,args,&mut capture).and_then(|value|{
            validate_registered_outputs(id,args,&value,&capture.reads)?;Ok(value)
        });
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
                if second.expected!=evaluation.expected||second.mask!=evaluation.mask||second.output_ptr!=evaluation.output_ptr||second.output_fields!=evaluation.output_fields {
                    return Err("MODEL_SNAPSHOT_REPLAY_DIFFERENCE");
                }
                event.evaluation=Some(evaluation.clone()); event.reads=reads;
                for field in &event.reads {
                    match live_read(field.address,field.width) {
                        Ok(now) if unchanged_outside_registered_writes(field.address,field.width,field.value,now,&evaluation.output_fields)?=>{},
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
            "MISMATCH"=>{
                // 관측된 첫 vanilla 차이에서 새 admission을 닫는다. 기존 in-flight는 원본1회 뒤 drain.
                close_capture(state);
                if state.first_mismatch.get().is_none(){let _=state.first_mismatch.set(event.clone());}
                stats.compared.fetch_add(1,Ordering::Relaxed);stats.mismatches.fetch_add(1,Ordering::Relaxed);
            },
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
wrapper!(hook19,19);wrapper!(hook20,20);wrapper!(hook21,21);wrapper!(hook22,22);wrapper!(hook23,23);wrapper!(hook24,24);wrapper!(hook25,25);wrapper!(hook26,26);
wrapper!(hook27,27);
wrapper!(hook28,28);
wrapper!(hook29,29);
wrapper!(hook30,30);
wrapper!(hook31,31);
wrapper!(hook32,32);
wrapper!(hook33,33);
wrapper!(hook34,34);
wrapper!(hook35,35);
wrapper!(hook36,36);
wrapper!(hook37,37);
wrapper!(hook38,38);
wrapper!(hook39,39);
wrapper!(hook40,40);
fn wrappers()->[usize;MODEL_COUNT] {[hook0 as *const () as usize,hook1 as *const () as usize,hook2 as *const () as usize,hook3 as *const () as usize,
    hook4 as *const () as usize,hook5 as *const () as usize,hook6 as *const () as usize,hook7 as *const () as usize,
    hook8 as *const () as usize,hook9 as *const () as usize,hook10 as *const () as usize,hook11 as *const () as usize,
    hook12 as *const () as usize,hook13 as *const () as usize,hook14 as *const () as usize,hook15 as *const () as usize,
    hook16 as *const () as usize,hook17 as *const () as usize,hook18 as *const () as usize,hook19 as *const () as usize,hook20 as *const () as usize,hook21 as *const () as usize,hook22 as *const () as usize,hook23 as *const () as usize,hook24 as *const () as usize,hook25 as *const () as usize,hook26 as *const () as usize,hook27 as *const () as usize,hook28 as *const () as usize,hook29 as *const () as usize,hook30 as *const () as usize,hook31 as *const () as usize,hook32 as *const () as usize,hook33 as *const () as usize,hook34 as *const () as usize,hook35 as *const () as usize,hook36 as *const () as usize,hook37 as *const () as usize,hook38 as *const () as usize,hook39 as *const () as usize,hook40 as *const () as usize]}

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
    persist_first_mismatch(state)?;
    // 부분 append 실패 뒤 같은 BufWriter를 다시 flush하면 부분 줄을 중복/이어쓸 수 있다.
    // 정본은 실행당 terminal failure로 처리하고 새 실행만 허용한다. 관측 큐 drain은 위에서 수행한다.
    if state.journal_failed.load(Ordering::Acquire)!=0{
        return Err(std::io::Error::other("summary journal disabled after hard failure; restart required"));
    }
    // ack 후보는 정본 journal append+flush 성공 뒤에만 게시한다.
    let pending=state.stop_pending_sequence.load(Ordering::Acquire);
    let current_ack=state.control_sequence.load(Ordering::Acquire);
    let ack=if state.admission.load(Ordering::Acquire)&ADMISSION_CLOSED!=0 && in_flight(state)==0
        && counter(&state.queued_events)==0 && pending>current_ack
        && (state.first_mismatch.get().is_none()||state.first_mismatch_file_state.load(Ordering::Acquire)==1){pending}else{current_ack};
    let installs=state.install.lock().unwrap_or_else(|e|e.into_inner()).clone();
    let probes=generated::PROBES.iter().map(|p| {
        let s=&state.stats[p.id];let status=installs.iter().find(|x|x.id==p.id);
        let original=format!("{{\"id\":{},\"rva\":{},\"semantic_id\":{},\"install_status\":{},\"foreign_chain\":{},\"original\":{},\"target\":{},\"calls\":{},\"sample_attempts\":{},\"compared\":{},\"matches\":{},\"mismatches\":{},\"skipped\":{},\"unstable\":{},\"model_panics\":{},\"dropped\":{},\"foreign_matches\":{},\"foreign_mismatches\":{},\"processing_errors\":{},\"unlogged_errors\":{},\"verdict\":{}}}",
            p.id,hexadecimal(p.rva as u64),json_quote(p.semantic_id),json_quote(status.map(|x|x.text.as_str()).unwrap_or("WAITING_INSTALL_DELAY")),status.map(|x|x.foreign_chain).unwrap_or(false),
            hexadecimal(status.map(|x|x.original as u64).unwrap_or(0)),hexadecimal(status.map(|x|x.target as u64).unwrap_or(0)),
            counter(&s.calls),counter(&s.sample_attempts),counter(&s.compared),counter(&s.matches),counter(&s.mismatches),counter(&s.skipped),counter(&s.unstable),counter(&s.model_panics),counter(&s.dropped),counter(&s.foreign_matches),counter(&s.foreign_mismatches),counter(&s.processing_errors),counter(&s.unlogged_errors),
            json_quote(if counter(&s.compared)==0 {"NO_SAMPLES"} else if counter(&s.mismatches)>0 {"MISMATCH_OBSERVED"} else {"OBSERVED_SAMPLES_MATCH_ONLY"}));
        format!("{},\"persisted\":{},\"enqueued\":{},\"eligible_calls\":{},\"queue_full\":{},\"queue_disconnected\":{},\"effective_sample_stride\":{}}}",original.trim_end_matches('}'),counter(&s.persisted),counter(&s.enqueued),counter(&s.eligible_calls),counter(&s.queue_full),counter(&s.queue_disconnected),state.probe_strides[p.id])
    }).collect::<Vec<_>>().join(",");
    let summary=format!("{{\"schema_version\":1,\"run_id\":{},\"baseline_sha256\":{},\"actual_sha256\":{},\"identity_verified\":{},\"armed\":{},\"catalog_total\":{},\"prepared_probes\":{},\"io_errors\":{},\"io_lost_or_uncertain_events\":{},\"sample_stride\":{},\"max_samples_per_probe\":{},\"probes\":[{}],\"scope\":\"관측표본의출력대조만. 실제표본0은검증불성립; 전체분기·모든도메인·임계값독립조절가능성증명아님\"}}\n",
        json_quote(&state.run_id),json_quote(generated::BASELINE_EXE_SHA256),json_quote(&state.actual_sha256),state.identity_verified,state.config.armed,generated::CENSUS_FUNCTIONS,generated::PROBES.len(),counter(&state.io_errors),counter(&state.io_lost_or_uncertain_events),state.config.sample_stride,state.config.max_samples_per_probe,probes);
    let summary=format!("{},\"capture_enabled\":{},\"capture_label\":{},\"control_sequence\":{},\"control_error\":{},\"in_flight\":{},\"scene_code\":{}}}\n",
        summary.trim().trim_end_matches('}'),state.capture_enabled.load(Ordering::Acquire)!=0,
        json_quote(state.capture_label.get().map(String::as_str).unwrap_or("")),ack,
        json_quote(&state.control_error.lock().unwrap_or_else(|e|e.into_inner())),in_flight(state),state.scene_code.load(Ordering::Acquire));
    let queue_pending=counter(&state.queued_events);
    let summary=format!("{},\"queued_events\":{},\"queue_pending\":{},\"stop_pending_sequence\":{},\"worker_state\":{}}}\n",
        summary.trim().trim_end_matches('}'),queue_pending,queue_pending,counter(&state.stop_pending_sequence),state.worker_state.load(Ordering::Acquire));
    let summary=format!("{},\"summary_retries\":{},\"last_io_error\":{},\"init_ready\":{}}}\n",
        summary.trim().trim_end_matches('}'),counter(&state.summary_retries),
        json_quote(&state.last_io_error.lock().unwrap_or_else(|e|e.into_inner())),state.init_ready.load(Ordering::Acquire)!=0);
    let first=state.first_mismatch.get();
    let first_name=if first.is_some(){json_quote(&format!("first_mismatch_{}.json",state.run_id))}else{"null".into()};
    let first_seq=first.map(|e|e.seq.to_string()).unwrap_or_else(||"null".into());
    let summary=format!("{},\"first_mismatch_present\":{},\"first_mismatch_seq\":{},\"first_mismatch_file\":{},\"first_mismatch_persisted\":{}}}\n",
        summary.trim().trim_end_matches('}'),first.is_some(),first_seq,first_name,state.first_mismatch_file_state.load(Ordering::Acquire)==1);
    let record_sequence=state.summary_record_sequence.load(Ordering::Acquire)+1;
    let timestamp_ms=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis();
    let summary=format!("{},\"summary_source\":\"journal\",\"record_seq\":{},\"summary_timestamp_unix_ms\":{},\"summary_record_sequence\":{},\"summary_journal\":{},\"summary_alias_errors\":{},\"last_summary_alias_error\":{}}}\n",
        summary.trim().trim_end_matches('}'),record_sequence,timestamp_ms,record_sequence,json_quote(&format!("summaries_{}.jsonl",state.run_id)),
        counter(&state.summary_alias_errors),json_quote(&state.last_summary_alias_error.lock().unwrap_or_else(|e|e.into_inner())));
    // observations가 먼저 flush된 뒤 정본 snapshot을 append한다. 교체/DELETE 공유권이 필요 없다.
    let journal_written=(||{
        let mut journal=state.summary_journal.lock().unwrap_or_else(|e|e.into_inner());
        journal.write_all(summary.as_bytes())?;journal.flush()
    })();
    if let Err(error)=journal_written{
        state.journal_failed.store(1,Ordering::Release);
        record_io_failure(state,"summary_journal_append_flush",&error,0);return Err(error);
    }
    state.summary_record_sequence.store(record_sequence,Ordering::Release);
    if ack>current_ack{state.control_sequence.store(ack,Ordering::Release);}
    // 별칭은 한 번만 시도한다. 파일 교체 경합이 정본/수집 gate를 닫거나 writer를 재시도로 막지 않는다.
    if let Err(failure)=write_atomic(&state.dir.join("summary.json"),&summary,&state.summary_retries,0){
        state.summary_alias_errors.fetch_add(1,Ordering::Relaxed);
        *state.last_summary_alias_error.lock().unwrap_or_else(|e|e.into_inner())=format!(
            "stage={};os_error={:?};retries={};message={}",failure.stage,failure.error.raw_os_error(),failure.retries,failure.error);
    }
    Ok(())
}

#[cfg(test)] mod tests {
#[test]fn next_insert_policy_exact_writes_and_no_write(){
    let a=[0x10000,0x10010,0x20000,0];
    let f=|address,value|Field{address,width:8,value};
    let reads=vec![f(a[1],2),f(a[2]+8,3),f(a[1]-8,1),f(a[2],0x30000),f(0x30008,20),f(0x30010,30),f(a[0],0),f(0x30000,10)];
    let mut e=Evaluation{expected:0,mask:u64::MAX,output_ptr:Some(a[0]),output_fields:vec![(a[1],8,1),(a[1]-8,8,0),(a[0],8,2)],metrics:vec![]};
    assert!(validate_registered_outputs(39,a,&e,&reads).is_ok());
    e.output_fields[1].0=a[1];assert!(validate_registered_outputs(39,a,&e,&reads).is_err());e.output_fields[1].0=a[1]-8;
    e.output_fields[0].1=4;assert!(validate_registered_outputs(39,a,&e,&reads).is_err());e.output_fields[0].1=8;
    e.output_fields[2].2=7;assert!(validate_registered_outputs(39,a,&e,&reads).is_err());e.output_fields[2].2=2;
    e.expected=7;assert!(validate_registered_outputs(39,a,&e,&reads).is_err());e.expected=0;
    let mut alias=reads.clone();alias[3].value=a[0];assert!(validate_registered_outputs(39,a,&e,&alias).is_err());
    let mut short=e.clone();short.output_fields=vec![(a[1],8,1),(a[1]-8,8,2)];
    assert!(validate_registered_outputs(39,a,&short,&reads).is_err()); // comparator가true인앞 index까지이동해야한다.
    e.output_fields.clear();e.output_ptr=None;e.expected=1;
    let mut no_write=reads[..6].to_vec();no_write[4].value=30;no_write[5].value=20;
    assert!(validate_registered_outputs(39,a,&e,&no_write).is_ok());
    assert!(!unchanged_outside_registered_writes(a[1]-8,8,1,2,&e.output_fields).unwrap());
    e.output_ptr=Some(a[0]);assert!(validate_registered_outputs(39,a,&e,&reads).is_err());
}
#[test]fn next_insert_only_actual_written_words_are_exempt(){
    let outputs=vec![(0x10010,8,1),(0x10008,8,2)];
    assert!(unchanged_outside_registered_writes(0x10010,8,2,1,&outputs).unwrap());
    assert!(!unchanged_outside_registered_writes(0x10000,8,0,7,&outputs).unwrap());
    assert!(!unchanged_outside_registered_writes(0x30000,8,10,11,&outputs).unwrap());
}
#[test]fn next_peer_seed_order_and_qqbyte_surface(){
    let a=[0x10000,0x20000,0,0];
    let reads=vec![Field{address:a[0],width:8,value:7},Field{address:a[0]+8,width:8,value:9}];
    let mut e=Evaluation{expected:a[0],mask:u64::MAX,output_ptr:Some(a[0]),output_fields:vec![(a[0]+8,8,9),(a[0],8,7),(a[0]+16,1,255)],metrics:vec![]};
    assert!(validate_registered_outputs(40,a,&e,&reads).is_ok());
    e.output_fields[1].2=8;assert!(validate_registered_outputs(40,a,&e,&reads).is_err());e.output_fields[1].2=7;
    e.output_fields[2].1=8;assert!(validate_registered_outputs(40,a,&e,&reads).is_err());e.output_fields[2].1=1;
    e.output_fields[2].2=256;assert!(validate_registered_outputs(40,a,&e,&reads).is_err());
    e.output_fields=vec![(a[0]+8,8,2),(a[0],8,11),(a[0]+16,1,3)];
    assert!(validate_registered_outputs(40,a,&e,&reads).is_ok());
    e.output_fields[0].2=5;assert!(validate_registered_outputs(40,a,&e,&reads).is_err());e.output_fields[0].2=2;
    let mut alias=reads.clone();alias.push(Field{address:a[0]+16,width:1,value:1});
    assert!(validate_registered_outputs(40,a,&e,&alias).is_err());
    let swapped=vec![reads[1].clone(),reads[0].clone()];assert!(validate_registered_outputs(40,a,&e,&swapped).is_err());
    e.expected=a[0]^8;assert!(validate_registered_outputs(40,a,&e,&reads).is_err());
}
#[test]fn next_peer_byte_registration_never_exempts_padding(){
    // 안정성 helper의 byte폭 시험. 실제 D3EEC0은 padding을 입력으로 읽지 않는다.
    let outputs=vec![(0x10008,8,2),(0x10000,8,11),(0x10010,1,3)];
    assert!(unchanged_outside_registered_writes(0x10010,8,0xaa00,0xaa03,&outputs).unwrap());
    assert!(!unchanged_outside_registered_writes(0x10010,8,0xaa00,0xbb03,&outputs).unwrap());
}


// root만 단일 STATE native 시험에 통합한다. 실제 게임 호출/실행 증거가 아니다.
static WAVE06_FAKE_CALLS:AtomicU64=AtomicU64::new(0);
const W06_POISON:u64=0xaaaa_bbbb_cccc_dddd;
const W06_EFFECT_WRONG_RETURN:u64=0xe012;
unsafe extern "system-unwind" fn fake_effect_option(out:u64,actor:u64,kind:u64,opaque:u64)->u64{
    WAVE06_FAKE_CALLS.fetch_add(1,Ordering::Relaxed);
    if kind==99{std::ptr::write((out+16)as *mut u8,2);return 0x1234_5678_9abc_def0;}
    // 이 fixture는 kind6의 자기소유 actor만 전달한다.
    assert_eq!(kind,6);let effect=actor+0x488;
    let current_kind=std::ptr::read_unaligned((actor+0x68)as *const u64);
    let current_state=std::ptr::read_unaligned((actor+0x70)as *const u64);
    let bit=u8::from(current_kind==15&&current_state==3);
    std::ptr::write_unaligned(out as *mut u64,effect);
    std::ptr::write_unaligned((out+8)as *mut u64,opaque);
    std::ptr::write((out+16)as *mut u8,bit);
    if opaque==W06_EFFECT_WRONG_RETURN{effect^8}else{effect}
}
// mode는 원문에서 미입력인 R9를 시험용 선택자로만 쓰며 production ABI를 확장하지 않는다.
// 0=정상Some,1=고갈None,2=empty의뜻밖cursor쓰기,3=Some뒤readonly셀변조,
// 4=Some ID오류,5=Some 반환오류. 아래 setup별로만 사용한다.
unsafe extern "system-unwind" fn fake_cell_option(out:u64,it:u64,_refs:u64,mode:u64)->u64{
    WAVE06_FAKE_CALLS.fetch_add(1,Ordering::Relaxed);
    let start=std::ptr::read_unaligned((it+8)as *const u64);
    let end=std::ptr::read_unaligned((it+16)as *const u64);
    if start==end{
        if mode==2{std::ptr::write_unaligned((it+8)as *mut u64,start+1);}
        std::ptr::write_unaligned(out as *mut u64,0);return out;
    }
    if mode==1{
        std::ptr::write_unaligned((it+8)as *mut u64,end);
        std::ptr::write_unaligned(out as *mut u64,0);return out;
    }
    assert_eq!((start,end),(0,1));
    std::ptr::write_unaligned((it+8)as *mut u64,1);
    // 원문과 같은 cursor→payload→tag 순서. 관측기는 최종값만 읽으므로 순서 직접관측 주장은 금지.
    std::ptr::write_unaligned((out+8)as *mut u64,if mode==4{43}else{42});
    std::ptr::write_unaligned((out+16)as *mut u64,16000);
    std::ptr::write_unaligned((out+24)as *mut u64,16000);
    std::ptr::write_unaligned(out as *mut u64,1);
    if mode==3{
        let root=std::ptr::read_unaligned(it as *const u64);
        std::ptr::write_unaligned((root+0x1c98)as *mut u64,0);
    }
    if mode==5{out^8}else{out}
}
fn wave06_actual_observe_axis(state:&'static State){
    reopen_for_independent_fixture(state);set_scene(4);
    let first_before=state.first_mismatch.get().map(|e|e.seq);
    let all_calls_before=counter(&WAVE06_FAKE_CALLS);
    // 매 호출 직전·직후 가짜 원본 1회를 검사한다. 게임 함수는 사용하지 않는다.
    let call=|id:usize,args:[u64;4],expected_return:u64|{
        let before=counter(&WAVE06_FAKE_CALLS);
        assert_eq!(unsafe{observe(id,args)},expected_return);
        assert_eq!(counter(&WAVE06_FAKE_CALLS),before+1);
        assert_eq!(in_flight(state),0);
    };
    let mut out=[W06_POISON;4];let base=out.as_mut_ptr()as u64;
    ORIGINALS[37].store(fake_effect_option as *const ()as usize,Ordering::Release);DOMAINS[37].store(2,Ordering::Release);
    let m37=counter(&state.stats[37].matches);
    call(37,[base,0,99,0],0x1234_5678_9abc_def0);
    assert_eq!(out,[W06_POISON,W06_POISON,0xaaaa_bbbb_cccc_dd02,W06_POISON]);
    assert_eq!(counter(&state.stats[37].matches),m37+1);flush().unwrap();
    let mut actor=vec![0u64;0x700/8];actor[0x68/8]=15;actor[0x70/8]=3;
    let actorptr=actor.as_mut_ptr()as u64;out.fill(W06_POISON);
    call(37,[base,actorptr,6,7],actorptr+0x488);
    assert_eq!(out,[actorptr+0x488,7,0xaaaa_bbbb_cccc_dd01,W06_POISON]);
    assert_eq!(counter(&state.stats[37].matches),m37+2);flush().unwrap();
    // Some matchbool0은 payload/정상RAX가같아도 BYTE0을 독립 비교한다.
    actor[0x70/8]=4;out.fill(W06_POISON);
    call(37,[base,actorptr,6,9],actorptr+0x488);
    assert_eq!(out,[actorptr+0x488,9,0xaaaa_bbbb_cccc_dd00,W06_POISON]);
    assert_eq!(counter(&state.stats[37].matches),m37+3);actor[0x70/8]=3;flush().unwrap();
    ORIGINALS[38].store(fake_cell_option as *const ()as usize,Ordering::Release);DOMAINS[38].store(2,Ordering::Release);
    let mut it=vec![0u64;0x490/8];let ip=it.as_mut_ptr()as u64;let m38=counter(&state.stats[38].matches);
    out.fill(W06_POISON);call(38,[base,ip,0,0],base);
    assert_eq!(out,[0,W06_POISON,W06_POISON,W06_POISON]);assert_eq!(it[1],0);
    assert_eq!(counter(&state.stats[38].matches),m38+1);flush().unwrap();
    let mut check=vec![0u64;0x1ca0/8];let mut output=vec![0u64;0x1ca0/8];
    let mut x=16000u64;let y=16000u64;let refs=[&x as *const u64 as u64,&y as *const u64 as u64];
    it[0]=check.as_mut_ptr()as u64;it[0x488/8]=output.as_mut_ptr()as u64;
    it[1]=0;it[2]=1;it[3]=0;it[4]=0;check[0x1c98/8]=1;output[0x1c98/8]=42;
    out.fill(W06_POISON);call(38,[base,ip,refs.as_ptr()as u64,0],base);
    assert_eq!(out,[1,42,16000,16000]);assert_eq!(it[1],1);
    assert_eq!(counter(&state.stats[38].matches),m38+2);flush().unwrap();
    // 음수 좌표 입력도 signed Q 비트패턴 그대로 전달한다. 중심출력은 양수 셀중심이다.
    it[1]=0;x=(-100000i64)as u64;out.fill(W06_POISON);
    call(38,[base,ip,refs.as_ptr()as u64,0],base);
    assert_eq!(out,[1,42,16000,16000]);assert_eq!(it[1],1);
    assert_eq!(counter(&state.stats[38].matches),m38+3);x=16000;flush().unwrap();
    // 고갈None은empty와달리cursor를쓴다. payload를포이즌으로재설정해미쓰기도검사한다.
    it[1]=0;it[2]=1;check[0x1c98/8]=0;out.fill(W06_POISON);
    call(38,[base,ip,refs.as_ptr()as u64,1],base);
    assert_eq!(out,[0,W06_POISON,W06_POISON,W06_POISON]);assert_eq!(it[1],1);
    assert_eq!(counter(&state.stats[38].matches),m38+4);flush().unwrap();
    // Empty에서예상되지않은cursor쓰기: mutable-span통째예외를주면이시험이거짓MATCH된다.
    it[1]=0;it[2]=0;out.fill(W06_POISON);let u38=counter(&state.stats[38].unstable);
    call(38,[base,ip,0,2],base);
    assert_eq!(counter(&state.stats[38].unstable),u38+1);
    assert_eq!(out,[0,W06_POISON,W06_POISON,W06_POISON]);assert_eq!(it[1],1);flush().unwrap();
    // Some의정상cursor쓰기외readonly입력cell변동은계속UNSTABLE이다.
    it[1]=0;it[2]=1;check[0x1c98/8]=1;out.fill(W06_POISON);
    call(38,[base,ip,refs.as_ptr()as u64,3],base);
    assert_eq!(counter(&state.stats[38].unstable),u38+2);assert_eq!(out,[1,42,16000,16000]);flush().unwrap();
    // 같은예상output/tag에서ID만틀리는경우에도첫차이닫힘. 그뒤호출은original-only이다.
    it[1]=0;check[0x1c98/8]=1;out.fill(W06_POISON);let bad38=counter(&state.stats[38].mismatches);
    call(38,[base,ip,refs.as_ptr()as u64,4],base);
    assert_eq!(out,[1,43,16000,16000]);assert_eq!(counter(&state.stats[38].mismatches),bad38+1);assert!(admit(state).is_none());
    let attempts=counter(&state.stats[38].sample_attempts);let eligible=counter(&state.stats[38].eligible_calls);
    call(38,[base,ip,0,0],base); // cursor==end이므로가짜원본만None을쓴다. model/read/attempt는추가0.
    assert_eq!(counter(&state.stats[38].sample_attempts),attempts);assert_eq!(counter(&state.stats[38].eligible_calls),eligible);flush().unwrap();
    // 독립된 다음 시험만 gate를 다시 연다. production의 재개 허용이 아니다.
    reopen_for_independent_fixture(state);it[1]=0;it[2]=1;check[0x1c98/8]=1;out.fill(W06_POISON);
    call(38,[base,ip,refs.as_ptr()as u64,5],base^8);
    assert_eq!(out,[1,42,16000,16000]);assert_eq!(counter(&state.stats[38].mismatches),bad38+2);assert!(admit(state).is_none());flush().unwrap();
    // Effect Some는mask0이아니다. 출력이완전히맞아도원래RAX가틀리면차이로닫힌다.
    reopen_for_independent_fixture(state);out.fill(W06_POISON);let bad37=counter(&state.stats[37].mismatches);
    call(37,[base,actorptr,6,W06_EFFECT_WRONG_RETURN],(actorptr+0x488)^8);
    assert_eq!(out,[actorptr+0x488,W06_EFFECT_WRONG_RETURN,0xaaaa_bbbb_cccc_dd01,W06_POISON]);
    assert_eq!(counter(&state.stats[37].mismatches),bad37+1);assert!(admit(state).is_none());flush().unwrap();
    assert_eq!(counter(&WAVE06_FAKE_CALLS)-all_calls_before,13);
    if let Some(seq)=first_before{assert_eq!(state.first_mismatch.get().unwrap().seq,seq);}
    assert_eq!(in_flight(state),0);assert_eq!(counter(&state.queued_events),0);
    assert_eq!(counter(&state.io_errors),0);
    // 호출자는다음독립I/Ofixture앞에서reopen_for_independent_fixture(state)를한번수행해야한다.
}

// cfg(test) 단일 STATE 안에서만 사용한다. 모델/PROBES/count41 root배선 후 실행 가능하다.
// R9는 두 게임 원문의 미입력이며 이 가짜원본에서만 고의 오동작 선택자로 쓴다.
static NEXT_OUTPUT_FAKE_CALLS:AtomicU64=AtomicU64::new(0);
const NEXT_OUTPUT_POISON:u64=0xaaaa_bbbb_cccc_dddd;
unsafe extern "system-unwind" fn fake_next_insert(begin:u64,pos:u64,header:u64,mode:u64)->u64{
    NEXT_OUTPUT_FAKE_CALLS.fetch_add(1,Ordering::Relaxed);
    let key=std::ptr::read_unaligned(pos as *const u64);
    let scores=std::ptr::read_unaligned(header as *const u64);
    let mut cursor=pos-8;let mut hole=pos;
    let mut pred=std::ptr::read_unaligned(cursor as *const u64);
    loop{
        let ps=std::ptr::read_unaligned(scores.wrapping_add(pred.wrapping_mul(8))as *const u64);
        let ks=std::ptr::read_unaligned(scores.wrapping_add(key.wrapping_mul(8))as *const u64);
        if !((ps as i64)<(ks as i64)||(ps==ks&&key<pred)){break;}
        std::ptr::write_unaligned(hole as *mut u64,pred);hole=cursor;
        if cursor==begin{break;}
        cursor-=8;pred=std::ptr::read_unaligned(cursor as *const u64);
    }
    if hole!=pos{std::ptr::write_unaligned(hole as *mut u64,key);}
    if mode==1{std::ptr::write_unaligned(scores as *mut u64,99);}
    if mode==2{std::ptr::write_unaligned((begin+8)as *mut u64,99);}
    if mode==3{std::ptr::write_unaligned(begin as *mut u64,99);}
    if mode==4{pred^8}else{pred}
}
unsafe extern "system-unwind" fn fake_next_seeded_peer(out:u64,cap:u64,_flag:u64,mode:u64)->u64{
    NEXT_OUTPUT_FAKE_CALLS.fetch_add(1,Ordering::Relaxed);
    let mut cost=std::ptr::read_unaligned(out as *const u64);
    let mut id=std::ptr::read_unaligned((out+8)as *const u64);
    let world=std::ptr::read_unaligned((cap+8)as *const u64);
    let bb=std::ptr::read_unaligned((cap+16)as *const u64);
    let reference=std::ptr::read_unaligned((cap+48)as *const u64);
    let entity=std::ptr::read_unaligned((world+0x238)as *const u64);
    let mut line=255;
    if entity!=0{
        let rx=std::ptr::read_unaligned((reference+0x658)as *const u64);
        let ry=std::ptr::read_unaligned((reference+0x660)as *const u64);
        let x=std::ptr::read_unaligned((entity+0x658)as *const u64);
        let y=std::ptr::read_unaligned((entity+0x660)as *const u64);
        let progress=std::ptr::read_unaligned((bb+0x10)as *const u64);
        let dx=rx.wrapping_sub(x);let dy=ry.wrapping_sub(y);
        cost=dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))/1_000_000;
        cost=cost.wrapping_add(progress.wrapping_mul(2));id=0;line=0;
    }
    // 정상 원문과 같은 Q8→Q0→BYTE16 순서. 최종값 비교를 쓰기순서 실측으로 부풀리지 않는다.
    std::ptr::write_unaligned((out+8)as *mut u64,id);
    std::ptr::write_unaligned(out as *mut u64,if mode==2{cost^1}else{cost});
    std::ptr::write((out+16)as *mut u8,line);
    if mode==1{std::ptr::write_unaligned((reference+0x658)as *mut u64,1001);}
    if mode==3{out^8}else{out}
}
fn next_output_actual_observe_axis(state:&'static State){
    reopen_for_independent_fixture(state);set_scene(4);
    let all=counter(&NEXT_OUTPUT_FAKE_CALLS);
    let first=state.first_mismatch.get().map(|e|e.seq);
    let call=|id,args,ret|{
        let before=counter(&NEXT_OUTPUT_FAKE_CALLS);
        assert_eq!(unsafe{observe(id,args)},ret);
        assert_eq!(counter(&NEXT_OUTPUT_FAKE_CALLS),before+1);assert_eq!(in_flight(state),0);
    };
    ORIGINALS[39].store(fake_next_insert as *const ()as usize,Ordering::Release);DOMAINS[39].store(2,Ordering::Release);
    let(mut ix,mut scores)=([0u64,1,2,NEXT_OUTPUT_POISON],[30u64,20,10]);
    let begin=ix.as_mut_ptr()as u64;let scoreptr=scores.as_mut_ptr()as u64;
    let mut header=[scoreptr,3u64];let hp=header.as_mut_ptr()as u64;let a=[begin,begin+16,hp,0];
    let m39=counter(&state.stats[39].matches);let u39=counter(&state.stats[39].unstable);let b39=counter(&state.stats[39].mismatches);
    // 이동 없음: 쓰기 예외도 없다. 반환 residue는 마지막 predecessor 1이다.
    call(39,a,1);assert_eq!(ix,[0,1,2,NEXT_OUTPUT_POISON]);assert_eq!(counter(&state.stats[39].matches),m39+1);flush().unwrap();
    ix.copy_from_slice(&[0,1,2,NEXT_OUTPUT_POISON]);scores.copy_from_slice(&[10,20,30]);
    call(39,a,0);assert_eq!(ix,[2,0,1,NEXT_OUTPUT_POISON]);assert_eq!(counter(&state.stats[39].matches),m39+2);flush().unwrap();
    ix.copy_from_slice(&[0,1,2,NEXT_OUTPUT_POISON]);scores.copy_from_slice(&[30,10,20]);
    call(39,a,0);assert_eq!(ix,[0,2,1,NEXT_OUTPUT_POISON]);assert_eq!(counter(&state.stats[39].matches),m39+3);flush().unwrap();
    ix.copy_from_slice(&[0,1,2,NEXT_OUTPUT_POISON]);scores.copy_from_slice(&[10,20,30]);
    call(39,[begin,begin+16,hp,1],0);assert_eq!(ix,[2,0,1,NEXT_OUTPUT_POISON]);assert_eq!(counter(&state.stats[39].unstable),u39+1);flush().unwrap();
    ix.copy_from_slice(&[0,1,2,NEXT_OUTPUT_POISON]);scores.copy_from_slice(&[30,20,10]);
    call(39,[begin,begin+16,hp,2],1);assert_eq!(ix,[0,99,2,NEXT_OUTPUT_POISON]);assert_eq!(counter(&state.stats[39].unstable),u39+2);flush().unwrap();
    ix.copy_from_slice(&[0,1,2,NEXT_OUTPUT_POISON]);scores.copy_from_slice(&[10,20,30]);
    call(39,[begin,begin+16,hp,3],0);assert_eq!(ix,[99,0,1,NEXT_OUTPUT_POISON]);
    assert_eq!(counter(&state.stats[39].mismatches),b39+1);assert!(admit(state).is_none());
    let at=counter(&state.stats[39].sample_attempts);let el=counter(&state.stats[39].eligible_calls);
    ix.copy_from_slice(&[0,1,2,NEXT_OUTPUT_POISON]);call(39,a,0);
    assert_eq!(ix,[2,0,1,NEXT_OUTPUT_POISON]);
    assert_eq!(counter(&state.stats[39].sample_attempts),at);assert_eq!(counter(&state.stats[39].eligible_calls),el);flush().unwrap();
    reopen_for_independent_fixture(state);ix.copy_from_slice(&[0,1,2,NEXT_OUTPUT_POISON]);
    call(39,[begin,begin+16,hp,4],8);assert_eq!(ix,[2,0,1,NEXT_OUTPUT_POISON]);assert_eq!(counter(&state.stats[39].mismatches),b39+2);assert!(admit(state).is_none());flush().unwrap();

    reopen_for_independent_fixture(state);
    ORIGINALS[40].store(fake_next_seeded_peer as *const ()as usize,Ordering::Release);DOMAINS[40].store(2,Ordering::Release);
    let mut out=[100u64,99,NEXT_OUTPUT_POISON];let op=out.as_mut_ptr()as u64;
    let mut world=vec![0u64;0x300/8];let mut bb=vec![0u64;0xb00/8];let mut reference=vec![0u64;0x668/8];let mut entity=vec![0u64;0x668/8];
    reference[0x658/8]=1000;bb[0x10/8]=5;
    let mut team=0u64;let mut excluded=99u64;let mut lines=[0u8,255,255,255,255];let mut allowed=[255u8,255];
    let mut allowed_ptr=allowed.as_mut_ptr()as u64;
    let mut cap=[&mut excluded as *mut u64 as u64,world.as_mut_ptr()as u64,bb.as_mut_ptr()as u64,&mut team as *mut u64 as u64,lines.as_mut_ptr()as u64,&mut allowed_ptr as *mut u64 as u64,reference.as_mut_ptr()as u64];
    let cp=cap.as_mut_ptr()as u64;
    let m40=counter(&state.stats[40].matches);let u40=counter(&state.stats[40].unstable);let b40=counter(&state.stats[40].mismatches);
    call(40,[op,cp,0,0],op);assert_eq!(out,[100,99,0xaaaa_bbbb_cccc_ddff]);assert_eq!(counter(&state.stats[40].matches),m40+1);flush().unwrap();
    world[0x238/8]=entity.as_mut_ptr()as u64;out.copy_from_slice(&[100,99,NEXT_OUTPUT_POISON]);
    call(40,[op,cp,0,0],op);assert_eq!(out,[11,0,0xaaaa_bbbb_cccc_dd00]);assert_eq!(counter(&state.stats[40].matches),m40+2);flush().unwrap();
    out.copy_from_slice(&[100,99,NEXT_OUTPUT_POISON]);
    call(40,[op,cp,0,1],op);assert_eq!(out,[11,0,0xaaaa_bbbb_cccc_dd00]);assert_eq!(counter(&state.stats[40].unstable),u40+1);flush().unwrap();
    reference[0x658/8]=1000;out.copy_from_slice(&[100,99,NEXT_OUTPUT_POISON]);
    call(40,[op,cp,0,2],op);assert_eq!(out,[10,0,0xaaaa_bbbb_cccc_dd00]);assert_eq!(counter(&state.stats[40].mismatches),b40+1);assert!(admit(state).is_none());
    let at=counter(&state.stats[40].sample_attempts);let el=counter(&state.stats[40].eligible_calls);
    call(40,[op,cp,0,0],op);assert_eq!(counter(&state.stats[40].sample_attempts),at);assert_eq!(counter(&state.stats[40].eligible_calls),el);flush().unwrap();
    assert_eq!(out,[11,0,0xaaaa_bbbb_cccc_dd00]);
    reopen_for_independent_fixture(state);out.copy_from_slice(&[100,99,NEXT_OUTPUT_POISON]);
    call(40,[op,cp,0,3],op^8);assert_eq!(out,[11,0,0xaaaa_bbbb_cccc_dd00]);assert_eq!(counter(&state.stats[40].mismatches),b40+2);assert!(admit(state).is_none());flush().unwrap();
    assert_eq!(counter(&NEXT_OUTPUT_FAKE_CALLS)-all,14);
    if let Some(seq)=first{assert_eq!(state.first_mismatch.get().unwrap().seq,seq);}
    assert_eq!(in_flight(state),0);assert_eq!(counter(&state.queued_events),0);assert_eq!(counter(&state.io_errors),0);
    // root는 이 호출 바로 뒤 기존 I/O 독립 축 앞에서 test 전용 reopen을 유지한다.
}

    #[test]fn wave06_effect_none_is_byte_only_and_scalar_mask_zero(){
        let a=[0x10000,0x20000,99,0];
        let mut e=Evaluation{expected:0,mask:0,output_ptr:Some(a[0]),output_fields:vec![(a[0]+16,1,2)],metrics:vec![]};
        assert!(validate_registered_outputs(37,a,&e,&[]).is_ok());
        e.mask=u64::MAX;assert!(validate_registered_outputs(37,a,&e,&[]).is_err());
        e.mask=0;e.output_fields[0].1=8;assert!(validate_registered_outputs(37,a,&e,&[]).is_err());
        e.output_fields=vec![(a[0],8,0x30000),(a[0]+8,8,4),(a[0]+16,1,1)];e.expected=0x30000;e.mask=u64::MAX;
        assert!(validate_registered_outputs(37,a,&e,&[]).is_ok());
        assert!(!unchanged_outside_registered_writes(a[0]+16,8,0,1u64<<8,&e.output_fields).unwrap());
    }
    #[test]fn wave06_iterator_partial_none_and_cursor_contract(){
        let a=[0x10000,0x20000,0x30000,0];let cur=a[1]+8;
        let mut reads=vec![Field{address:cur,width:8,value:2},Field{address:a[1]+16,width:8,value:4}];
        let mut e=Evaluation{expected:a[0],mask:u64::MAX,output_ptr:Some(a[0]),output_fields:vec![(cur,8,4),(a[0],8,0)],metrics:vec![]};
        assert!(validate_registered_outputs(38,a,&e,&reads).is_ok());
        assert!(!unchanged_outside_registered_writes(a[0]+8,8,3,4,&e.output_fields).unwrap());
        e.output_fields=vec![(cur,8,3),(a[0]+8,8,99),(a[0]+16,8,16000),(a[0]+24,8,48000),(a[0],8,1)];
        assert!(validate_registered_outputs(38,a,&e,&reads).is_ok());
        reads.push(Field{address:cur+4,width:4,value:0});assert!(validate_registered_outputs(38,a,&e,&reads).is_err());reads.pop();
        reads[1].value=2;e.output_fields=vec![(a[0],8,0)];assert!(validate_registered_outputs(38,a,&e,&reads).is_ok());
        assert!(!unchanged_outside_registered_writes(cur,8,2,3,&e.output_fields).unwrap());
    }
    #[test]fn wave06_rdx_return_candidate_remains_entry_unsupported(){
        let mut c=Config::default();c.probes=[0x1016b00].into_iter().collect();
        assert!(validate_probe_selection(&c).unwrap_err().contains("미지원/진입불가"));
        c.probes=[0xe20d00,0x1014470].into_iter().collect();assert!(validate_probe_selection(&c).is_ok());
    }


    #[test]fn first_mismatch_create_new_refuses_existing_evidence(){
        let dir=std::env::temp_dir().join(format!("aiav_first_difference_{}_{}",std::process::id(),std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&dir).unwrap();let file=dir.join("first.json");
        write_first_mismatch_new(&file,"{\"first\":true}").unwrap();
        let original=std::fs::read(&file).unwrap();
        assert!(write_first_mismatch_new(&file,"replacement").is_err());assert_eq!(std::fs::read(&file).unwrap(),original);
        assert!(write_first_mismatch_new(&dir,"not a file").is_err());std::fs::remove_dir_all(dir).unwrap();
    }

    #[test] fn registered_outputs_are_exact_and_not_generic_abi(){
        let a=[0x10000,0x20000,0x30000,4];
        let mut e=Evaluation{expected:0,mask:u64::MAX,output_ptr:Some(a[0]),output_fields:vec![(a[0],8,1),(a[0]+8,8,2),(a[0]+16,8,3)],metrics:Vec::new()};
        assert!(validate_registered_outputs(29,a,&e,&[]).is_ok());
        assert!(validate_registered_outputs(28,a,&e,&[]).is_err());
        e.output_fields[1].0=a[0];assert!(validate_registered_outputs(29,a,&e,&[]).is_err());
        e.output_fields[1].0=a[0]+8;e.output_fields[1].1=4;assert!(validate_registered_outputs(29,a,&e,&[]).is_err());
        e.output_fields[1].1=8;e.output_fields.pop();assert!(validate_registered_outputs(29,a,&e,&[]).is_err());
        let bad=[u64::MAX-7,0,0,0];e.output_ptr=Some(bad[0]);e.output_fields=vec![(bad[0],8,0);3];
        assert!(validate_registered_outputs(29,bad,&e,&[]).is_err());
    }
    #[test] fn partial_write_never_exempts_the_other_bytes(){
        let outputs=[(0x10000,4,0x44443333)];let before=0xaaaabbbb11112222;
        assert!(unchanged_outside_registered_writes(0x10000,8,before,0xaaaabbbb44443333,&outputs).unwrap());
        assert!(!unchanged_outside_registered_writes(0x10000,8,before,0xaaaabbbc44443333,&outputs).unwrap());
        assert!(!unchanged_outside_registered_writes(0x10000,8,before,0xaaaabbbb44443333,&[]).unwrap());
        assert!(unchanged_outside_registered_writes(u64::MAX-3,8,0,0,&outputs).is_err());
    }
    #[test] fn cursor_only_first_header_is_a_registered_mutable_input(){
        let a=[0x10000,0,0,0];let header=Field{address:0x10000,width:8,value:3};
        let mut e=Evaluation{expected:1,mask:255,output_ptr:Some(a[0]),output_fields:vec![(a[0],8,4)],metrics:Vec::new()};
        for id in 32..=36 {assert!(validate_registered_outputs(id,a,&e,&[header.clone()]).is_ok());}
        assert!(validate_registered_outputs(33,a,&e,&[header.clone(),Field{address:0x10004,width:4,value:0}]).is_err());
        assert!(validate_registered_outputs(29,a,&e,&[header.clone()]).is_err());
        e.output_fields.clear();e.output_ptr=None;for id in 32..=36 {assert!(validate_registered_outputs(id,a,&e,&[header.clone()]).is_ok());}
        assert!(!unchanged_outside_registered_writes(header.address,header.width,3,4,&e.output_fields).unwrap());
    }

    use super::*;
    #[test] fn per_probe_resolution_rejects_invalid_programmatic_config() {
        let c=crate::config::parse("probes=deadbeef\nprobe_strides=deadbeef:1").unwrap();
        assert!(resolve_probe_strides(&c).is_err()); // 선택됐어도 generated에없음
        let mut c=crate::config::parse("probes=dc0bb0\nprobe_strides=dc0bb0:1").unwrap();
        c.probe_strides.insert(0xdc0bb0,0);assert!(resolve_probe_strides(&c).is_err());
        c.probe_strides.insert(0xdc0bb0,1);c.probes.clear();assert!(resolve_probe_strides(&c).is_err());
    }
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
        write_atomic(&summary,"replaced",&retries,20).unwrap();release.join().unwrap();
        assert!(counter(&retries)>0&&counter(&retries)<=20);
        assert_eq!(std::fs::read_to_string(&summary).unwrap(),"replaced");
        let successful_retries=counter(&retries);
        let held=OpenOptions::new().read(true).share_mode(1|2).open(&summary).unwrap();
        let began=std::time::Instant::now();let failure=write_atomic(&summary,"must_not_replace",&retries,20).unwrap_err();
        assert_eq!(failure.stage,"summary_rename");assert_eq!(failure.retries,20);
        assert!(matches!(failure.error.raw_os_error(),Some(5|32|33)));
        assert!(began.elapsed()>=Duration::from_millis(100));
        assert_eq!(counter(&retries),successful_retries); // 실패의20회는 성공retry누계에 섞지 않는다.
        drop(held);assert_eq!(std::fs::read_to_string(&summary).unwrap(),"replaced");
        assert!(summary.with_extension("json.tmp").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }
    fn reopen_for_independent_fixture(state:&State){
        assert_eq!(in_flight(state),0);state.admission.store(0,Ordering::Release);state.capture_enabled.store(1,Ordering::Release);
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
    static FAKE_SAMPLING_CALLS:AtomicU64=AtomicU64::new(0);
    unsafe extern "system-unwind" fn fake_sampling_original(_a:u64,_b:u64,_c:u64,_d:u64)->u64 {
        FAKE_SAMPLING_CALLS.fetch_add(1,Ordering::Relaxed);FAKE_RESULT
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
// root가 MODEL_COUNT34/dispatch와 함께 기존 단일 STATE native 시험에 통합할 조각.
// 이 파일 자체를 실행하거나 production 모델을 가짜로 대체하지 않았다.
// 테스트 모듈 상단에 둘 항목:
static OUTPUT_FIXTURE_CALLS:AtomicU64=AtomicU64::new(0);
unsafe extern "system-unwind" fn fake_population_output(out:u64,_team:u64,cache:u64,mode:u64)->u64{
    OUTPUT_FIXTURE_CALLS.fetch_add(1,Ordering::Relaxed);
    for (j,value) in [2u64,1,if mode==1{999}else{1}].into_iter().enumerate(){
        std::ptr::write_unaligned((out as usize+j*8)as *mut u64,value);
    }
    if mode==2{
        let entity=std::ptr::read_unaligned((cache as usize+0x238)as *const u64);
        std::ptr::write_unaligned((entity as usize+0x668)as *mut u64,0);
    }
    2 // outptr가 아닌 원문 RAX ownCount를 그대로 모사한다.
}

// 실제 observe→원본1회→실출력 safe_read 비교 축. cfg(test)의 기존 한 STATE 안에서 호출한다.
fn output_fake_original_observe_axis(state:&'static State){
    let mut out=[0x5555_aaaa_9999_7777u64;5];let output=out[1..].as_mut_ptr()as u64;
    let mut first=vec![0u64;0x670/8];let mut second=vec![0u64;0x670/8];
    first[0x620/8]=100;first[0x668/8]=40;second[0x620/8]=100;second[0x668/8]=39;
    let mut cache=vec![0u64;0x288/8];cache[0x238/8]=first.as_ptr()as u64;
    cache[0x240/8]=second.as_ptr()as u64;cache[0x260/8]=first.as_ptr()as u64;
    ORIGINALS[29].store(fake_population_output as *const () as usize,Ordering::Release);
    DOMAINS[29].store(2,Ordering::Release);set_scene(4);
    let args=[output,0,cache.as_ptr()as u64,0];
    reopen_for_independent_fixture(state);
    let calls=counter(&OUTPUT_FIXTURE_CALLS);let matches=counter(&state.stats[29].matches);
    assert_eq!(unsafe{observe(29,args)},2);assert_eq!(counter(&OUTPUT_FIXTURE_CALLS),calls+1);
    assert_eq!(&out[1..4],&[2,1,1]);assert_eq!(out[0],0x5555_aaaa_9999_7777);
    assert_eq!(out[4],0x5555_aaaa_9999_7777);assert_eq!(counter(&state.stats[29].matches),matches+1);
    flush().unwrap();
    // 등록 출력과 다른 입력 변화를 숨기지 않는다.
    let unstable=counter(&state.stats[29].unstable);
    assert_eq!(unsafe{observe(29,[args[0],args[1],args[2],2])},2);
    assert_eq!(counter(&state.stats[29].unstable),unstable+1);first[0x668/8]=40;flush().unwrap();
    // 예상과 다른 output[2]만으로도 차이/수집닫힘이 발생한다. 원반환2는 유지한다.
    let mismatch=counter(&state.stats[29].mismatches);
    assert_eq!(unsafe{observe(29,[args[0],args[1],args[2],1])},2);
    assert_eq!(counter(&state.stats[29].mismatches),mismatch+1);assert!(admit(state).is_none());
    let attempts=counter(&state.stats[29].sample_attempts);let calls=counter(&OUTPUT_FIXTURE_CALLS);
    assert_eq!(unsafe{observe(29,args)},2);assert_eq!(counter(&OUTPUT_FIXTURE_CALLS),calls+1);
    assert_eq!(counter(&state.stats[29].sample_attempts),attempts);flush().unwrap();
}

    #[test] fn native_fake_original_once_and_journal_hard_io_is_failclosed() {
        let dir=std::env::temp_dir().join(format!("tfm2_aiav_engine_selftest_{}_{}",std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(dir.join("summary.json")).unwrap();
        let mut configuration=crate::config::parse("armed=true\nprobes=fdb9a0,dc0bb0,dc0f80,f27690,f27860,ef2410,fa9600,1019890,fe2700\nsample_stride=4096\nprobe_strides=dc0f80:1,f27690:2,f27860:3,ef2410:1,fa9600:1,1019890:1,fe2700:1").unwrap();
        configuration.max_samples_per_probe=20;configuration.queue_capacity=8;
        initialize(dir.clone(),configuration,"ENGINE_SELFTEST_NOT_A_GAME_SESSION".into(),true).unwrap();
        let state=STATE.get().unwrap();
        assert_eq!(state.install_tried.load(Ordering::Acquire),0);
        assert_eq!(counter(&state.io_errors),0);assert_eq!(counter(&state.summary_alias_errors),1);
        assert_eq!(state.summary_record_sequence.load(Ordering::Acquire),1);
        assert!(dir.join(format!("summaries_{}.jsonl",state.run_id)).exists());
        assert!(ORIGINALS.iter().all(|x|x.load(Ordering::Acquire)==0));
        std::fs::remove_dir(dir.join("summary.json")).unwrap();
        platform::init_safe_reader().unwrap();
        assert!(state.worker.lock().unwrap().is_none());
        assert_eq!(state.worker_state.load(Ordering::Acquire),0);
        assert_eq!(counter(&state.queued_events),0);
        // 초기 alias 실패는 soft이며 worker/실제 설치는 아직0. 이후는 가짜 원본 수동 fixture다.
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
        // alias가 잠겨도 정본 journal flush와 수집은 유지된다.
        flush().unwrap();
        {
            use std::os::windows::fs::OpenOptionsExt;
            let alias=OpenOptions::new().read(true).share_mode(1|2).open(dir.join("summary.json")).unwrap();
            let journal_reader=OpenOptions::new().read(true).share_mode(1|2)
                .open(dir.join(format!("summaries_{}.jsonl",state.run_id))).unwrap();
            let before=counter(&state.summary_alias_errors);let record=counter(&state.summary_record_sequence);
            flush().unwrap();assert_eq!(counter(&state.summary_alias_errors),before+1);
            assert_eq!(counter(&state.summary_record_sequence),record+1);
            assert_eq!(counter(&state.io_errors),0);assert_eq!(state.capture_enabled.load(Ordering::Acquire),1);
            assert!(state.last_summary_alias_error.lock().unwrap().contains("stage=summary_rename;os_error="));drop(alias);drop(journal_reader);
        }

        // 실제 observe에서 첫 차이/큐full/원본1회/닫힌뒤추가모델0/별도원문보존을 시험한다.
        let filler=Event{id:7,seq:900000,thread_id:0,args:[0;4],actual:0,evaluation:None,reads:Vec::new(),output_fields:Vec::new(),
            comparison:"SKIP",reason:"QUEUE_FILLER",foreign_chain:false,capture_label:"SELFTEST".into(),scene_code:4};
        for _ in 0..state.config.queue_capacity{assert!(enqueue(state,filler.clone()));}
        let first_seq=counter(&state.sequence);let calls=counter(&FAKE_CALLS);
        assert_eq!(unsafe{observe(7,[address,99,0,0])},FAKE_RESULT);
        assert_eq!(counter(&FAKE_CALLS),calls+1);assert!(admit(state).is_none());
        assert_eq!(counter(&state.stats[7].dropped),1);assert_eq!(state.first_mismatch.get().unwrap().seq,first_seq);
        assert_eq!(state.first_mismatch.get().unwrap().actual,FAKE_RESULT);
        assert_eq!(state.first_mismatch.get().unwrap().reads[0].value,0);
        let attempts=counter(&state.stats[7].sample_attempts);
        assert_eq!(unsafe{observe(7,[0,99,0,0])},FAKE_RESULT); // 모델이실행되면NULL read지만닫혔으므로original만1회
        assert_eq!(counter(&FAKE_CALLS),calls+2);assert_eq!(counter(&state.stats[7].sample_attempts),attempts);
        assert_eq!(counter(&state.queued_events),8);flush().unwrap();assert_eq!(counter(&state.queued_events),0);
        let first_path=dir.join(format!("first_mismatch_{}.json",state.run_id));
        let first_text=std::fs::read_to_string(&first_path).unwrap();
        assert_eq!(first_text,format!("{}\n",event_json(state,state.first_mismatch.get().unwrap())));
        assert_eq!(state.first_mismatch_file_state.load(Ordering::Acquire),1);
        let first_bytes=std::fs::read(&first_path).unwrap();flush().unwrap();assert_eq!(std::fs::read(&first_path).unwrap(),first_bytes);
        assert_eq!(counter(&state.io_errors),0);reopen_for_independent_fixture(state);
        // 아래집계를독립fixture로초기화하되 first_mismatch 슬롯/원문은영구첫것을유지한다.
        let s=&state.stats[7];
        for value in [&s.calls,&s.sample_attempts,&s.compared,&s.matches,&s.mismatches,&s.skipped,&s.unstable,&s.model_panics,&s.dropped,
            &s.foreign_matches,&s.foreign_mismatches,&s.processing_errors,&s.unlogged_errors,&s.persisted,&s.enqueued,&s.eligible_calls,&s.queue_full,&s.queue_disconnected]{value.store(0,Ordering::Relaxed);}
        // 기존 원본1회·20표본·큐유실 fixture의 집계를 독립 구간으로 유지한다.
        FAKE_CALLS.store(0,Ordering::Relaxed);state.stats[7].calls.store(0,Ordering::Relaxed);

        assert_eq!(unsafe{observe(7,[address,99,0,0])},FAKE_RESULT); // 예상0과 불일치
        reopen_for_independent_fixture(state);assert_eq!(unsafe{observe(7,[address,99,1,0])},FAKE_RESULT); // 원본 중 입력변경
        own[0x840/8]=0;
        assert_eq!(unsafe{observe(7,[0,99,0,0])},FAKE_RESULT);       // 읽기실패여도 원반환 유지
        DOMAINS[7].store(3,Ordering::Release);
        assert_eq!(unsafe{observe(7,[address,99,0,0])},FAKE_RESULT); // 외부 체인은 별도
        DOMAINS[7].store(2,Ordering::Release);
        assert_eq!(unsafe{observe(7,[address,99,2,0])},0);           // 예상0과 일치
        for _ in 0..30 {reopen_for_independent_fixture(state);assert_eq!(unsafe{observe(7,[address,99,0,0])},FAKE_RESULT);}
        reopen_for_independent_fixture(state);
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
        // sampler 8경계: 실제 parse→초기배열→observe→가짜원본이며 게임/추가shadow CALL은 없다.
        flush().unwrap();
        assert_eq!(state.probe_strides[8],4096);assert_eq!(state.probe_strides[9],1);
        let run_sampling=|id:usize,n:u64| {
            ORIGINALS[id].store(fake_sampling_original as *const () as usize,Ordering::Release);
            DOMAINS[id].store(2,Ordering::Release);
            let before=FAKE_SAMPLING_CALLS.load(Ordering::Relaxed);
            for _ in 0..n {
                let before_mismatches=counter(&state.stats[id].mismatches);
                assert_eq!(unsafe{observe(id,[0,0,0,0])},FAKE_RESULT);
                if counter(&state.stats[id].mismatches)>before_mismatches {
                    // 고의로다른가짜원본반환을사용하는서로독립된sampler fixture만재개한다.
                    assert_eq!(counter(&state.stats[id].mismatches),before_mismatches+1);
                    assert_eq!(state.capture_enabled.load(Ordering::Acquire),0);
                    assert!(admit(state).is_none());assert_eq!(counter(&state.io_errors),0);
                    reopen_for_independent_fixture(state);
                }
            }
            assert_eq!(FAKE_SAMPLING_CALLS.load(Ordering::Relaxed)-before,n);
            assert_eq!(in_flight(state),0);
        };
        run_sampling(8,64);assert_eq!(counter(&state.stats[8].sample_attempts),64); // 첫64 샘플후보 전부
        run_sampling(8,1);assert_eq!(counter(&state.stats[8].sample_attempts),64);  // eligible64 skip
        run_sampling(8,4031);assert_eq!(counter(&state.stats[8].eligible_calls),4096);
        assert_eq!(counter(&state.stats[8].sample_attempts),64);                 // eligible4095까지skip
        run_sampling(8,1);assert_eq!(counter(&state.stats[8].sample_attempts),65); // eligible4096관측
        flush().unwrap();
        run_sampling(9,67);assert_eq!(counter(&state.stats[9].sample_attempts),67);flush().unwrap();
        run_sampling(10,68);assert_eq!(counter(&state.stats[10].sample_attempts),66);flush().unwrap();
        run_sampling(11,70);assert_eq!(counter(&state.stats[11].sample_attempts),66);flush().unwrap();
        // 수집off:예산0/원본1회. test전용으로기존한세션의admission만되돌린다.
        close_capture(state);run_sampling(12,3);assert_eq!(counter(&state.stats[12].sample_attempts),0);
        state.admission.store(0,Ordering::Release);state.capture_enabled.store(1,Ordering::Release);
        set_scene(1);run_sampling(13,3);assert_eq!(counter(&state.stats[13].eligible_calls),0);set_scene(4);
        ORIGINALS[14].store(fake_sampling_original as *const () as usize,Ordering::Release);
        DOMAINS[14].store(1,Ordering::Release);let before=FAKE_SAMPLING_CALLS.load(Ordering::Relaxed);
        for _ in 0..3 {assert_eq!(unsafe{observe(14,[0;4])},FAKE_RESULT);}
        assert_eq!(FAKE_SAMPLING_CALLS.load(Ordering::Relaxed)-before,3);
        assert_eq!(counter(&state.stats[14].eligible_calls),0);assert_eq!(counter(&state.stats[14].sample_attempts),0);
        run_sampling(15,25);let s15=&state.stats[15];assert_eq!(counter(&s15.sample_attempts),25);
        assert_eq!(counter(&s15.compared)+counter(&s15.skipped)+counter(&s15.unstable),20); // 상한유지
        flush().unwrap();
        output_fake_original_observe_axis(state);
        wave06_actual_observe_axis(state);
        next_output_actual_observe_axis(state);
        reopen_for_independent_fixture(state);
        // 기존 I/O실패fixture의기록1개를다시만들어 sampler배치flush와분리한다.
        assert_eq!(unsafe{observe(6,[0,player.as_ptr()as u64,data.as_ptr()as u64,0])},0x1122_3344_5566_7700);
        assert_eq!(counter(&state.queued_events),1);
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
        let journal_text=std::fs::read_to_string(dir.join(format!("summaries_{}.jsonl",state.run_id))).unwrap();
        let journal_ack=journal_text.lines().last().unwrap();
        assert!(journal_ack.contains("\"control_sequence\":2")&&journal_ack.contains("\"queue_pending\":0")
            &&journal_ack.contains("\"in_flight\":0")&&journal_ack.contains("\"summary_source\":\"journal\""));

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
        // 정본 기록 실패에서는 ack 후보3을 게시하지 않고 hardclose한다.
        let journal_path=dir.join(format!("summaries_{}.jsonl",state.run_id));
        // 끝에 부분 줄을 둔 뒤 hard append 실패를 주입한다. 이후 재시도가 그 줄을 이어쓰면 안 된다.
        {let mut f=OpenOptions::new().append(true).open(&journal_path).unwrap();f.write_all(b"{\"partial\":").unwrap();f.flush().unwrap();}
        let failed_length=std::fs::metadata(&journal_path).unwrap().len();
        *state.summary_journal.lock().unwrap()=BufWriter::new(File::open(&journal_path).unwrap());
        std::fs::write(&control,format!("run_id={}\nsequence=3\ncommand=stop",state.run_id)).unwrap();process_control();
        let errors=counter(&state.io_errors);let record=counter(&state.summary_record_sequence);
        assert!(flush().is_err());assert_eq!(counter(&state.io_errors),errors+1);
        assert_eq!(counter(&state.control_sequence),2);assert_eq!(counter(&state.summary_record_sequence),record);
        assert!(state.last_io_error.lock().unwrap().contains("stage=summary_journal_append_flush;os_error="));
        assert!(admit(state).is_none());
        // 부분 실패 후 같은 writer 재시도/새 ack는 금지한다.
        assert_eq!(state.journal_failed.load(Ordering::Acquire),1);
        let errors=counter(&state.io_errors);assert!(flush().is_err());
        assert_eq!(counter(&state.io_errors),errors);assert_eq!(counter(&state.control_sequence),2);
        assert_eq!(std::fs::metadata(&journal_path).unwrap().len(),failed_length);


    }
}
