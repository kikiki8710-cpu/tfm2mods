//! 분석 근거와 실제 게임 출력의 관측 비교 전용. 게임의 판단·계수·반환값을 바꾸지 않는다.
//! 지원된 ABI의 호출만 원본 한 번 → 독립모델 비교로 관측한다. 일반 shadow-call은 하지 않는다.
#![allow(dead_code)]
mod config;
mod engine;
mod generated;
mod hook;
mod models;
mod models_extra;
mod models_wave03;
mod models_wave04;
mod models_wave05_direct;
mod models_wave05_outputs;
mod models_wave06_direct;
mod models_wave06_iterator;
mod models_d8a1f0;
mod models_next;
mod models_candidates;
mod models_wave08_direct;
mod loaded_constants;
mod platform;
mod sha256;

use mod_api_stable::{declare_stable_mod,LogLevel,StableClient,StableExtension,StableHost,StableMod};
use std::panic::{catch_unwind,AssertUnwindSafe};

pub const MOD_ID:&str="tfm2_ai_analysis_verify";
#[used] static SOURCE_PATH:&str=file!();
struct Extension;
impl StableExtension for Extension {
    fn post_update(&self,ctx:&mut StableClient<'_>,_dt_micros:u64) {
        let _=catch_unwind(AssertUnwindSafe(||{
            engine::set_scene(ctx.scene_kind().map(|x|x as usize).unwrap_or(usize::MAX));
            engine::post_update();
        }));
    }
}
impl Drop for Extension{fn drop(&mut self){engine::shutdown();}}

fn setup()->Result<(),String> {
    let path=platform::own_module_path(init as *const () as usize).map_err(str::to_string)?;
    let dir=path.parent().ok_or("모드 DLL 부모경로 없음")?.to_path_buf();
    let settings=dir.join("verify.cfg");
    let configuration=match std::fs::read_to_string(settings) {
        Ok(text)=>config::parse(&text)?,
        Err(e) if e.kind()==std::io::ErrorKind::NotFound=>config::Config::default(),
        Err(e)=>return Err(format!("설정 읽기 실패: {e}")),
    };
    let image_path=platform::module_path().map_err(str::to_string)?;
    let bytes=std::fs::read(&image_path).map_err(|e|format!("EXE 읽기 실패: {e}"))?;
    let digest=sha256::digest(&bytes);drop(bytes);
    let identity=digest==generated::BASELINE_EXE_SHA256;
    if configuration.armed&&identity {platform::init_safe_reader().map_err(str::to_string)?;}
    engine::initialize(dir,configuration,digest,identity)
}

fn init(host:&StableHost)->StableMod {
    let result=catch_unwind(AssertUnwindSafe(setup));
    match result {
        Ok(Ok(()))=>host.log(LogLevel::Info,"AI 분석 검증 준비. summary.json의 armed/identity/표본 수를 확인하세요."),
        Ok(Err(error))=>host.log(LogLevel::Error,&format!("AI 분석 검증 설치 거절: {error}")),
        Err(_)=>host.log(LogLevel::Error,"AI 분석 검증 초기화 panic. 원본 함수에 훅을 설치하지 않았습니다."),
    }
    let mut module=StableMod::new(MOD_ID);
    module.set_extension(Extension);module
}
declare_stable_mod!(init);
