//! Opt-in QA only: original native dispatch, verified roster and arming remain unchanged.
//! Native counters are process-interval counters, never claimed to be one World's counters.
use crate::{micro,sha256,tactics};
use mod_api_stable::{ClientSceneKindV1,StableClient,StableMatchHook,StableSim};
use serde_json::{json,Value};
use std::{path::{Path,PathBuf},sync::{Mutex,OnceLock,atomic::{AtomicBool,Ordering::Relaxed}}};

const FILE:&str="detail_tactics_replay_ab.json";
static PATHS:OnceLock<(PathBuf,PathBuf)>=OnceLock::new();
static ACTIVE:AtomicBool=AtomicBool::new(false);
static QA:Mutex<Option<Qa>>=Mutex::new(None);
static OBS:Mutex<Observations>=Mutex::new(Observations{callbacks:0,origins:Vec::new(),blocks:Vec::new()});

#[derive(Clone,Debug)]
struct Control {label:String,state:String}
struct Session {control:Control,hash:String,run_number:u64,accepted_frame:u64,original:String,applied:String,values_hash:Option<String>,counters:Value,observation_offset:usize}
struct View {first:Value,last:Value,start:Value,first_frame:u64,last_frame:u64,samples:u64,fingerprint:u64,missing:u8}
struct Qa {session:Option<Session>,view:Option<View>,seen:String,poll:u64,serial:u64}
struct Observations {callbacks:u64,origins:Vec<u32>,blocks:Vec<Value>}

pub fn init(dir:&Path,log:&Path){let _=PATHS.set((dir.join(FILE),log.join("replay_ab")));}
pub fn active()->bool {ACTIVE.load(Relaxed)||PATHS.get().map_or(false,|p|p.0.exists())}
fn hash(bytes:&[u8])->String {sha256::digest(bytes)}
fn values_hash()->Option<String>{let p=PATHS.get()?.0.parent()?.join(crate::values::FILE_NAME);std::fs::read(p).ok().map(|b|hash(&b))}
fn write(path:&Path,value:&Value){
    if let Some(parent)=path.parent(){let _=std::fs::create_dir_all(parent);}
    match serde_json::to_vec_pretty(value).map_err(|e|e.to_string()).and_then(|mut b|{b.push(b'\n');std::fs::write(path,b).map_err(|e|e.to_string())}) {
        Ok(())=>(),Err(e)=>crate::log(&format!("replay_ab output error: {e}"))
    }
}
fn receipt(label:Option<&str>,value:Value){if let Some(p)=PATHS.get(){let path=label.map_or_else(||p.1.join("control_status.json"),|l|p.1.join(l).join("receipt.json"));write(&path,&value);}}
fn verify_prior_label(control:&Control,prior:&Value)->Result<(),String>{
    let label=prior.get("label").and_then(Value::as_str).ok_or("saved label missing")?;
    let state=prior.get("state_text").and_then(Value::as_str).ok_or("saved state_text missing")?;
    let normalized=parse_control(&serde_json::to_vec(&json!({"label":label,"state_text":state})).map_err(|e|e.to_string())?)?;
    if normalized.label!=control.label||normalized.state!=control.state{return Err("label already belongs to different levels; choose a new label".into());}Ok(())
}
fn create_record(path:&Path,value:&Value)->Result<(),String>{
    use std::io::Write;
    std::fs::create_dir_all(path.parent().ok_or("record parent missing")?).map_err(|e|e.to_string())?;
    let mut file=std::fs::OpenOptions::new().create_new(true).write(true).open(path).map_err(|e|e.to_string())?;
    let mut bytes=serde_json::to_vec_pretty(value).map_err(|e|e.to_string())?;bytes.push(b'\n');
    file.write_all(&bytes).and_then(|_|file.flush()).map_err(|e|e.to_string())
}
fn reserve_label(control:&Control,out:&Path,frame:u64)->Result<u64,String>{
    let dir=out.join(&control.label);let profile=dir.join("profile.json");
    if profile.exists(){
        let prior:Value=serde_json::from_slice(&std::fs::read(&profile).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        verify_prior_label(control,&prior)?;
    }else{
        // Older QA builds only had receipts; normalize their sparse state before comparison.
        let receipt=dir.join("receipt.json");
        if receipt.exists(){let prior:Value=serde_json::from_slice(&std::fs::read(&receipt).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
            if prior["applied"]==true{verify_prior_label(control,&prior)?;}}
        create_record(&profile,&json!({"label":control.label,"state_text":control.state,"normalized_state_sha256":hash(control.state.as_bytes())}))?;
    }
    for number in 1..=100000u64 {
        let path=dir.join("runs").join(format!("{number}.json"));
        if path.exists(){continue;}
        create_record(&path,&json!({"label":control.label,"run_number":number,"frame":frame,"normalized_state_sha256":hash(control.state.as_bytes())}))?;
        return Ok(number);
    }Err("too many reserved runs for label".into())
}

fn parse_control(raw:&[u8])->Result<Control,String>{
    if raw.len()>131072{return Err("control exceeds 128KiB".into());}
    let v:Value=serde_json::from_slice(raw).map_err(|e|format!("invalid JSON: {e}"))?;
    let object=v.as_object().ok_or("control must be an object")?;
    if object.keys().any(|k|k!="label"&&k!="state_text"){return Err("unknown control field".into());}
    let label=v.get("label").and_then(Value::as_str).ok_or("label must be a string")?;
    if label.is_empty()||label.len()>64||!label.bytes().all(|c|c.is_ascii_alphanumeric()||c==b'_'||c==b'-'){return Err("label must contain 1..64 ASCII letters/digits/_/-".into());}
    let txt=v.get("state_text").and_then(Value::as_str).ok_or("state_text must be a string")?;
    let mut lines=txt.lines();
    if lines.next()!=Some("v1"){return Err("state_text must start with v1".into());}
    let mut rows=vec![[0u8;6];tactics::N];let mut seen=vec![false;tactics::N];
    for (line_no,line) in lines.enumerate(){
        if line.trim().is_empty(){continue;}
        let parts:Vec<_>=line.split_whitespace().collect();
        if parts.len()!=7{return Err(format!("line {} requires id plus exactly six levels",line_no+2));}
        let t=tactics::index_of(parts[0]).ok_or_else(||format!("unknown tactic {}",parts[0]))?;
        if seen[t]{return Err(format!("duplicate tactic {}",parts[0]));}seen[t]=true;
        for col in 0..6 {let lv=parts[col+1].parse::<u8>().map_err(|_|"level must be an integer")?;
            let max=if tactics::TACTICS[t].kind==tactics::Kind::Toggle{1}else{2};
            if lv>max{return Err(format!("invalid level for {}",parts[0]));}rows[t][col]=lv;
        }
    }
    let mut state=String::from("v1\n");
    for (t,row) in rows.iter().enumerate(){state+=&format!("{} {} {} {} {} {} {}\n",tactics::TACTICS[t].id,row[0],row[1],row[2],row[3],row[4],row[5]);}
    Ok(Control{label:label.to_owned(),state})
}
fn clock_seconds(s:&str)->Option<u64>{
    let (m,s)=s.trim().split_once(':')?;
    if m.is_empty()||s.len()!=2||!m.bytes().chain(s.bytes()).all(|b|b.is_ascii_digit()){return None;}
    let (m,s)=(m.parse::<u64>().ok()?,s.parse::<u64>().ok()?);if s>=60{return None;}m.checked_mul(60)?.checked_add(s)
}
fn ui_sample(ctx:&StableClient<'_>,frame:u64)->Option<Value>{
    if ctx.ui_visible("ingame")!=Some(true){return None;}
    let clock=ctx.ui_text("ingame.header.game_time.value")?;let sec=clock_seconds(&clock)?;
    let mut teams=Vec::new();
    for side in ["blue","red"] {
        let stat=format!("ingame.header.{side}_stat");let info=format!("ingame.header.{side}_info");
        teams.push(json!({"name":ctx.ui_text(&format!("{info}.team_name")),"kill":ctx.ui_text(&format!("{stat}.kill")),
            "gold":ctx.ui_text(&format!("{stat}.gold.value")),"tower":ctx.ui_text(&format!("{stat}.tower.value")),
            "serpen":ctx.ui_text(&format!("{stat}.serpen.value")),"morgard":ctx.ui_text(&format!("{stat}.morgard.value"))}));
    }
    // The UI asset contains these placeholders before a real GameFrameData arrives.
    if clock.trim()=="12:34"&&teams.iter().all(|t|t["kill"].as_str()==Some("99")&&t["gold"].as_str()==Some("15.3K")){return None;}
    Some(json!({"frame":frame,"clock":clock,"seconds":sec,"teams":teams}))
}
fn fingerprint(prev:u64,value:&Value)->u64 {let mut h=prev;for b in value.to_string().bytes(){h^=u64::from(b);h=h.wrapping_mul(1099511628211);}h}
fn counter_diff(a:&Value,b:&Value)->Value {
    let Some(a)=a.as_array() else{return Value::Null;};let Some(b)=b.as_array() else{return Value::Null;};
    Value::Array(a.iter().zip(b).map(|(a,b)|{
        let mut o=b.clone();for key in ["calls","mine","applied","subject_reject"]{o[key]=json!(b[key].as_u64().unwrap_or(0).wrapping_sub(a[key].as_u64().unwrap_or(0)));}o
    }).collect())
}
fn finish_view(q:&mut Qa,frame:u64,reason:&str){
    let Some(view)=q.view.take()else{return;};let Some(session)=q.session.as_ref()else{return;};
    let end=micro::replay_ab_snapshot();let obs=OBS.lock().unwrap_or_else(|e|e.into_inner());
    let blocks:Vec<_>=obs.blocks.iter().skip(session.observation_offset).cloned().collect();
    let actual=tactics::state_text();let current_values=values_hash();
    let out=json!({"schema":1,"label":session.control.label,"run_number":session.run_number,"exe_sha256":crate::sites::EXE_SHA256,"frame_end":frame,"reason":reason,
        "control_sha256":session.hash,"selection_sha256":hash(session.applied.as_bytes()),"selection_unchanged":actual==session.applied,
        "value_config_sha256":session.values_hash,"value_config_unchanged":current_values==session.values_hash,
        "state_text":session.applied,"owned_athletes":tactics::my_athletes(),"armed":tactics::armed(),
        "ui":{"first":view.first,"last":view.last,"first_frame":view.first_frame,"last_frame":view.last_frame,"samples":view.samples,
            "observed_from_start":view.first["seconds"].as_u64().map_or(false,|s|s<=2),"fingerprint_fnv1a":format!("{:016x}",view.fingerprint),
            "winner":Value::Null,"completion":"last_visible_frame_only; UI disappearance is not a proved match end"},
        "counters":{"scope":"process_interval_including_possible_background_worlds","control_start_frame":session.accepted_frame,
            "view_start_frame":view.first_frame,"end_frame":frame,"control_start":session.counters,"view_start":view.start,"end":end,
            "control_delta":counter_diff(&session.counters,&end),"view_delta":counter_diff(&view.start,&end),"atomic_snapshot":"individual_relaxed_loads_not_a_global_barrier"},
        "sdk":{"callbacks_total":obs.callbacks,"observed_origin_kinds":obs.origins,"blocks":blocks,
            "availability":"runtime_callback_observation_required; no callback is not a zero-result match","winner":Value::Null}});
    q.serial+=1;if let Some(p)=PATHS.get(){write(&p.1.join(&session.control.label).join(format!("capture-{}-{frame}-{}.json",session.run_number,q.serial)),&out);}
    crate::log(&format!("replay_ab capture label={} reason={reason} ui_last={}",session.control.label,view.last));
}

pub fn tick(ctx:&StableClient<'_>,frame:u64){
    let Some(paths)=PATHS.get()else{return;};
    let mut lock=QA.lock().unwrap_or_else(|e|e.into_inner());
    let q=lock.get_or_insert_with(||Qa{session:None,view:None,seen:String::new(),poll:0,serial:0});
    let main=ctx.client_scene_kind()==Some(ClientSceneKindV1::Main);
    if q.session.is_some(){
        match ui_sample(ctx,frame){
            Some(sample)=>{
                if q.view.as_ref().map_or(false,|v|sample["seconds"].as_u64()<v.last["seconds"].as_u64()){finish_view(q,frame,"clock_regression_or_seek");}
                if q.view.is_none(){q.view=Some(View{first:sample.clone(),last:sample.clone(),start:micro::replay_ab_snapshot(),first_frame:frame,last_frame:frame,samples:0,fingerprint:14695981039346656037,missing:0});}
                let v=q.view.as_mut().unwrap();v.missing=0;
                // Fingerprint excludes wall/client frame: identical sampled clock+score sequences can compare across runs.
                if v.samples==0||v.last["clock"]!=sample["clock"]||v.last["teams"]!=sample["teams"]{
                    v.fingerprint=fingerprint(v.fingerprint,&json!({"clock":sample["clock"],"teams":sample["teams"]}));v.samples+=1;
                }v.last=sample;v.last_frame=frame;
            }
            None=>{if let Some(v)=q.view.as_mut(){v.missing=v.missing.saturating_add(1);if v.missing>=2{finish_view(q,frame,"ui_hidden");}}}
        }
    }
    if frame<q.poll{return;}q.poll=frame+15;
    if let Some(session)=q.session.as_ref(){
        let obs=OBS.lock().unwrap_or_else(|e|e.into_inner());
        write(&paths.1.join(&session.control.label).join("live.json"),&json!({"frame":frame,"label":session.control.label,"run_number":session.run_number,
            "selection_unchanged":tactics::state_text()==session.applied,"armed":tactics::armed(),"owned_athletes":tactics::my_athletes(),
            "ui_last":q.view.as_ref().map(|v|v.last.clone()),"sdk_callbacks_total":obs.callbacks,"sdk_origin_kinds":obs.origins,
            "sdk_blocks":obs.blocks.iter().skip(session.observation_offset).collect::<Vec<_>>(),"winner":Value::Null}));
    }
    let raw=match std::fs::read(&paths.0){Ok(r)=>r,Err(e)if e.kind()==std::io::ErrorKind::NotFound=>{
        if main&&q.view.is_none(){if let Some(s)=q.session.take(){tactics::apply_state_text(&s.original);crate::log("replay_ab control removed: original in-memory levels restored");}q.seen.clear();ACTIVE.store(false,Relaxed);}return;
    },Err(e)=>{receipt(None,json!({"valid":false,"applied":false,"frame":frame,"error":e.to_string()}));return;}};
    let digest=hash(&raw);if q.seen==digest{return;}
    if !main||q.view.is_some(){receipt(None,json!({"valid":Value::Null,"applied":false,"frame":frame,"control_sha256":digest,"status":"waiting_for_Main_and_no_active_view"}));return;}
    let control=match parse_control(&raw){Ok(c)=>c,Err(e)=>{q.seen=digest.clone();receipt(None,json!({"valid":false,"applied":false,"frame":frame,"control_sha256":digest,"error":e}));return;}};
    if !tactics::armed()||!crate::save::loaded()||tactics::my_athletes().is_empty(){receipt(Some(&control.label),json!({"valid":true,"applied":false,"frame":frame,"status":"waiting_for_original_save_loaded_and_verified_roster_armed"}));return;}
    if let Some(s)=q.session.as_ref(){if s.control.label==control.label&&s.control.state!=control.state{q.seen=digest;receipt(Some(&control.label),json!({"valid":true,"applied":false,"frame":frame,"error":"reuse of a label with different levels is forbidden"}));return;}}
    let run_number=match reserve_label(&control,&paths.1,frame){Ok(n)=>n,Err(e)=>{q.seen=digest.clone();receipt(Some(&control.label),json!({"valid":true,"applied":false,"frame":frame,"error":e}));return;}};
    let original=q.session.as_ref().map(|s|s.original.clone()).unwrap_or_else(tactics::state_text);
    tactics::apply_state_text(&control.state);let applied=tactics::state_text();let vhash=values_hash();let roster=tactics::my_athletes();
    let offset=OBS.lock().unwrap_or_else(|e|e.into_inner()).blocks.len();
    receipt(Some(&control.label),json!({"valid":true,"applied":true,"frame":frame,"run_number":run_number,"scene":"Main","control_sha256":digest,
        "selection_sha256":hash(applied.as_bytes()),"state_text":applied,"normalized_rows":tactics::N,"value_config_sha256":vhash,
        "owned_athletes":roster,"armed":tactics::armed(),"save_writes":false,"counter_sites":crate::sites::PATCHES.len(),
        "note":"base means original bypass; toggle low means behavior disabled, not QA off"}));
    crate::log(&format!("replay_ab applied label={} control_sha256={digest}",control.label));
    q.session=Some(Session{control,hash:digest.clone(),run_number,accepted_frame:frame,original,applied,values_hash:vhash,counters:micro::replay_ab_snapshot(),observation_offset:offset});q.seen=digest;ACTIVE.store(true,Relaxed);
}
pub fn leave(frame:u64){
    let mut g=QA.lock().unwrap_or_else(|e|e.into_inner());if let Some(q)=g.as_mut(){finish_view(q,frame,"extension_left_game");q.session=None;q.seen.clear();}ACTIVE.store(false,Relaxed);
}

pub struct Observer;
fn observe(sim:&StableSim<'_>,start:bool){
    if !active(){return;}
    let mut o=OBS.lock().unwrap_or_else(|e|e.into_inner());o.callbacks=o.callbacks.wrapping_add(1);
    let Some(origin)=sim.sim_origin()else{return;};if !o.origins.contains(&origin.kind){o.origins.push(origin.kind);}
    if ![2,3,4].contains(&origin.kind){return;}
    let tick=sim.tick();if !start&&tick%30!=0&&!sim.is_end(){return;}
    let key=json!([origin.kind,origin.match_id,origin.replay_id,origin.set_index,sim.seed()]);
    let players:Vec<_>=(0..sim.player_count()).filter_map(|i|sim.player_at(i)).map(|p|json!({"id":p.id(),"team":p.team(),"lane":p.lane().map(|l|l as u32),"kills":p.kills(),"deaths":p.deaths(),"gold_carried":p.gold(),"alive":p.is_alive()})).collect();
    let sample=json!({"tick":tick,"score_diff_team0":sim.score_diff(0),"is_end":sim.is_end(),"kill_log_count":sim.kill_log_count(),"players":players});
    let found=if start{None}else{o.blocks.iter().rposition(|b|b["key"]==key&&b["last"]["is_end"]!=true&&b["last"]["tick"].as_u64().unwrap_or(0)<=tick as u64)};
    if let Some(i)=found {let b=&mut o.blocks[i];b["last"]=sample;b["samples"]=json!(b["samples"].as_u64().unwrap_or(0)+1);}
    else if o.blocks.len()<256 {o.blocks.push(json!({"key":key,"first":sample,"last":sample,"samples":1,"winner":Value::Null,"time_unit":"raw_sim_ticks; no assumed ticks_per_second"}));}
}
impl StableMatchHook for Observer {
    fn on_match_start(&self,sim:&mut StableSim<'_>){let _=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||observe(sim,true)));}
    fn on_match_tick(&self,sim:&mut StableSim<'_>,_:u64){let _=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||observe(sim,false)));}
    fn check_match_end(&self,_:&mut StableSim<'_>)->Option<bool>{None}
}

#[cfg(test)]mod tests{
    use super::*;
    fn input(label:&str,state:&str)->Vec<u8>{serde_json::to_vec(&json!({"label":label,"state_text":state})).unwrap()}
    #[test]fn strict_profiles_and_sparse_default(){let c=parse_control(&input("A1","v1\n")).unwrap();assert_eq!(c.state.lines().count(),89);assert_eq!(c.label,"A1");
        let id=tactics::TACTICS[0].id;let c=parse_control(&input("B_1",&format!("v1\n{id} 0 1 2 0 0 0\n"))).unwrap();assert!(c.state.contains(&format!("{id} 0 1 2 0 0 0")));}
    #[test]fn rejects_ambiguous_or_unknown_input(){let id=tactics::TACTICS[0].id;for s in ["v2\n".to_string(),"v1\nunknown 0 0 0 0 0 0\n".into(),format!("v1\n{id} 0 0 0 0 0\n"),format!("v1\n{id} 0 0 0 0 0 3\n"),format!("v1\n{id} 0 0 0 0 0 0\n{id} 0 0 0 0 0 0\n")]{assert!(parse_control(&input("test",&s)).is_err());}assert!(parse_control(&input("../escape","v1\n")).is_err());assert!(parse_control(br#"{"label":"A","state_text":"v1\n","armed":true}"#).is_err());
        if let Some(d)=tactics::TACTICS.iter().find(|d|d.kind==tactics::Kind::Toggle){assert!(parse_control(&input("bad_toggle",&format!("v1\n{} 0 0 0 0 0 2\n",d.id))).is_err());}}
    #[test]fn clock_and_deltas(){assert_eq!(clock_seconds("00:00"),Some(0));assert_eq!(clock_seconds("12:34"),Some(754));assert_eq!(clock_seconds("00:60"),None);assert_eq!(clock_seconds("1:2"),None);assert_eq!(clock_seconds("1:02:03"),None);
        let a=json!([{"calls":8,"mine":3,"applied":1,"subject_reject":2}]);let b=json!([{"calls":13,"mine":5,"applied":2,"subject_reject":4}]);assert_eq!(counter_diff(&a,&b)[0]["calls"],5);assert_eq!(counter_diff(&a,&b)[0]["applied"],1);}
    #[test]fn persistent_label_reuse_and_numbered_runs(){
        let dir=std::env::temp_dir().join(format!("detail_tactics_replay_ab_unit_{}_{}",std::process::id(),crate::FRAME.load(Relaxed)));
        std::fs::create_dir(&dir).unwrap();
        let a=parse_control(&input("A","v1\n")).unwrap();let b=parse_control(&input("B","v1\n")).unwrap();
        assert_eq!(reserve_label(&a,&dir,10).unwrap(),1);assert_eq!(reserve_label(&b,&dir,11).unwrap(),1);
        let changed=parse_control(&input("A",&format!("v1\n{} 1 0 0 0 0 0\n",tactics::TACTICS[0].id))).unwrap();
        assert!(reserve_label(&changed,&dir,12).is_err()); // A -> B -> different A, including a fresh process's disk-only check.
        assert_eq!(reserve_label(&a,&dir,13).unwrap(),2);
        assert!(verify_prior_label(&a,&json!({"label":"A","state_text":"v1\n"})).is_ok());
        assert!(verify_prior_label(&changed,&json!({"label":"A","state_text":"v1\n"})).is_err());
        // Remove only the files this test created, with no recursive filesystem operation.
        for label in ["A","B"] {let p=dir.join(label);for number in 1..=2 {let f=p.join("runs").join(format!("{number}.json"));if f.exists(){std::fs::remove_file(f).unwrap();}}
            std::fs::remove_dir(p.join("runs")).unwrap();std::fs::remove_file(p.join("profile.json")).unwrap();std::fs::remove_dir(p).unwrap();}
        std::fs::remove_dir(dir).unwrap();
    }
}
