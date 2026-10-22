//! 상세전술: 공식 UI·세이브 API + 내 팀 한정 AI 문턱 적용.
#![allow(dead_code)]
mod platform; mod micro; mod sites; mod tactics; mod values; mod save; mod sha256; mod roster; mod ui;
#[path="../../common/log_dir.rs"] mod log_dir;
use mod_api_stable::{declare_stable_mod,LogLevel,StableClient,StableExtension,StableHost,StableMod,SceneKindV1};
use std::sync::{Mutex,OnceLock,atomic::{AtomicU64,Ordering::Relaxed}};
use std::path::PathBuf;
static LOG:OnceLock<PathBuf>=OnceLock::new();
pub static FRAME:AtomicU64=AtomicU64::new(0);
pub fn log(s:&str){
    use std::io::Write;
    if let Some(p)=LOG.get(){if let Ok(mut f)=std::fs::OpenOptions::new().create(true).append(true).open(p){let _=writeln!(f,"[{}] {s}",FRAME.load(Relaxed));}}
}
#[derive(Default)] struct Runtime { roster:roster::Roster, installed:bool, inside:bool }
struct Extension(Mutex<Runtime>);
impl StableExtension for Extension {
    fn post_update(&self,ctx:&mut StableClient<'_>,_:u64){
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
            let frame=FRAME.fetch_add(1,Relaxed)+1;
            let mut r=self.0.lock().unwrap_or_else(|e|e.into_inner());
            if !r.installed && frame>=120 {r.installed=true;for l in micro::install(){log(l);}}
            if ctx.scene_kind()!=Some(SceneKindV1::InGame){
                tactics::set_armed(false);
                if r.inside {save::leave();ui::leave();r.roster=roster::Roster::default();r.inside=false;}
                return;
            }
            r.inside=true;
            save::tick(ctx);
            let own=r.roster.tick(ctx,frame);
            tactics::set_armed(save::loaded()&&own);
            for l in values::reload_if_changed(frame,false){log(&l);}
            ui::tick(ctx,frame);
            if frame%1800==0{for l in micro::diagnostics(){log(&l);}}
        })).is_err(){tactics::set_armed(false);log("post_update panic: 적용 중지");}
    }
    fn on_end(&self){tactics::set_armed(false);for l in micro::diagnostics(){log(&l);}}
}
fn init(host:&StableHost)->StableMod {
    let setup=(||->Result<(),String>{
        let dll=platform::own_module_path(init as *const () as usize).map_err(str::to_string)?;
        let dir=dll.parent().ok_or("DLL 경로 없음")?;
        let out=log_dir::log_dir_path(dir);std::fs::create_dir_all(&out).map_err(|e|e.to_string())?;
        let _=LOG.set(out.join("detail_tactics.log"));
        for l in values::init(dir){log(&l);}
        Ok(())
    })();
    match setup {Ok(())=>host.log(LogLevel::Info,"상세전술 준비: 팀전술 화면의 상세전술 버튼"),Err(e)=>host.log(LogLevel::Error,&e)}
    let mut module=StableMod::new("tfm2_detail_tactics");module.set_extension(Extension(Mutex::new(Runtime::default())));module
}
declare_stable_mod!(init);
