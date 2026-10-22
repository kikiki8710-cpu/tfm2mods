//! 상세전술 문턱 관측. armed + 기준 EXE 일치 때만 한 번 설치한다.
#![allow(dead_code)]
mod config;
mod patch;
mod platform;
mod probe;
mod sha256;
mod sites;
mod summary;

use mod_api_stable::{declare_stable_mod, LogLevel, StableClient, StableExtension, StableHost, StableMod};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::Ordering::Relaxed;

pub const MOD_ID: &str = "tfm2_tactics_verify";
struct Runtime {
    dir: std::path::PathBuf,
    cfg: config::Config,
    digest: String,
    identity: bool,
    base: usize,
    attempted: bool,
    worker: Option<summary::Worker>,
}
static RUNTIME: OnceLock<Mutex<Runtime>> = OnceLock::new();

fn setup() -> Result<(), String> {
    let dll = platform::own_module_path(init as *const () as usize).map_err(str::to_string)?;
    let dir = dll.parent().ok_or("DLL 부모경로 없음")?.to_path_buf();
    let text = std::fs::read_to_string(dir.join("tactics_verify.cfg"));
    let cfg = match text {
        Ok(text) => config::parse(&text)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => config::Config::default(),
        Err(e) => return Err(format!("cfg 읽기 오류: {e}")),
    };
    let exe = platform::module_path().map_err(str::to_string)?;
    let digest = sha256::digest(&std::fs::read(exe).map_err(|e| format!("EXE 읽기 오류: {e}"))?);
    let identity = digest == sites::BASELINE_EXE_SHA256;
    if !identity {
        summary::event(&dir, &format!("EXE SHA256 불일치: {digest}; 설치 거절"));
        return Err(format!("EXE SHA256 불일치: {digest}; 아무것도 설치하지 않음"));
    }
    if cfg.armed { platform::init_safe_reader().map_err(str::to_string)?; }
    let base = platform::module_base().map_err(str::to_string)?;
    summary::event(&dir, &format!("준비 armed={} identity={identity}", cfg.armed));
    RUNTIME.set(Mutex::new(Runtime { dir, cfg, digest, identity, base, attempted: false, worker: None }))
        .map_err(|_| "이미 초기화됨".to_string())
}

struct Extension;
impl StableExtension for Extension {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt_micros: u64) {
        let result = catch_unwind(AssertUnwindSafe(|| {
            platform::MAIN_TID.store(platform::current_thread_id(), Relaxed);
            let scene = ctx.scene_kind().map(|s| s as u64).unwrap_or(u64::MAX);
            let in_match = matches!(scene, 4 | 5);
            platform::IN_MATCH.store(in_match, Relaxed);
            summary::SCENE.store(scene, Relaxed);
            let frame = summary::FRAME.fetch_add(1, Relaxed).wrapping_add(1);
            if in_match { summary::MATCH_FRAMES.fetch_add(1, Relaxed); }
            let Some(runtime) = RUNTIME.get() else { return; };
            let mut r = runtime.lock().unwrap_or_else(|e| e.into_inner());
            if r.attempted || !r.cfg.armed || !r.identity || frame < r.cfg.install_delay_frames { return; }
            // panic/일부 거절도 다시 설치하지 않는다. 지연은 전체 post_update 프레임 기준이다.
            r.attempted = true;
            let install_ms = summary::now_ms();
            let (knobs, effective) = patch::apply(r.base, &r.cfg.sets);
            let gates = probe::install(r.base, &effective, &r.cfg.gates);
            let session = summary::Session { dir: r.dir.clone(), cfg: r.cfg.clone(), exe_sha256: r.digest.clone(),
                identity: r.identity, install_ms, install_frame: frame, knobs, gates };
            match summary::start(session) {
                Ok(worker) => r.worker = Some(worker),
                Err(e) => summary::event(&r.dir, &e),
            }
        }));
        if result.is_err() {
            if let Some(runtime) = RUNTIME.get() {
                let r = runtime.lock().unwrap_or_else(|e| e.into_inner());
                summary::event(&r.dir, "상세전술 검증 post_update panic: 재설치하지 않음");
            }
        }
    }
}
impl Drop for Extension {
    fn drop(&mut self) {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            if let Some(runtime) = RUNTIME.get() {
                let worker = runtime.lock().unwrap_or_else(|e| e.into_inner()).worker.take();
                drop(worker);
            }
        }));
    }
}

fn init(host: &StableHost) -> StableMod {
    match catch_unwind(AssertUnwindSafe(setup)) {
        Ok(Ok(())) => host.log(LogLevel::Info, "상세전술 문턱 검증 준비. 기본 armed=false, cfg는 시작 시 한 번 읽음."),
        Ok(Err(e)) => host.log(LogLevel::Error, &format!("상세전술 문턱 검증 설치 거절: {e}")),
        Err(_) => host.log(LogLevel::Error, "상세전술 문턱 검증 setup panic: 설치하지 않음"),
    }
    let mut module = StableMod::new(MOD_ID);
    module.set_extension(Extension); module
}
declare_stable_mod!(init);
