//! tfm2_view_plus — daram2 "뷰 플러스" 9종 통합 재구현 (순수 SDK, 패치내성)
//! ===========================================================================
//! 설계 원칙:
//!   - 하드코딩 RVA·raw 메모리 오프셋 0개. mod_api(공식 SDK) 타입드 API만 사용.
//!     → 게임 패치 시 mod-sdk만 교체 후 재빌드하면 동작(RVA 마이그레이션 불필요).
//!   - 기능별 on/off = mods\tfm2_view_plus\tfm2_view_plus.cfg (자동생성·보수).
//!   - 통합 대상 8기능: finance / coaching / roster / recruitment /
//!                      facility / statistics / banpick / save_compat
//!     (티어 자동분배 엔진 stat_tier_assignment 은 범위 제외)
//! ===========================================================================
#![allow(dead_code, unused_imports, unused_variables)]

use mod_api::*;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

mod banpick;
mod coaching;
mod config;
mod facility;
mod finance;
mod recruitment;
mod roster;
mod statistics;
mod ui;

// 기본 tfm2_view_plus. 테스트 스왑용으로 빌드시 VP_MOD_ID 로 오버라이드 가능
// (예: banpick_view_plus 워크샵 폴더에 얹어 인게임 검증 — enabled_mods id 일치용).
pub(crate) const MOD_ID: &str = match option_env!("VP_MOD_ID") {
    Some(s) => s,
    None => "tfm2_view_plus",
};

// ── 게임 exe 기준 모드 폴더 (설치위치 무관) ──
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
}
fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 {
        return None;
    }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\{}", dir.display(), MOD_ID))
}
/// 디버그 구조 덤프(기능별 DUMP 플래그 켰을 때만 호출됨).
pub fn log_dump(file: &str, contents: &str) -> std::io::Result<()> {
    if let Some(d) = mod_dir() {
        let _ = std::fs::create_dir_all(&d);
        std::fs::write(format!("{d}\\{file}"), contents)
    } else {
        Ok(())
    }
}

// ── 진입 ──────────────────────────────────────────────────────────────────
struct ViewPlusExt;

impl ModExtension for ViewPlusExt {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, assets: &mut Assets, dt: f32) {
        // detour/확장 패닉이 게임으로 unwind 되지 않도록 방어.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            ui::start_frame(ui);
            finance::tick(ui);
            coaching::tick(scene, ui);
            roster::tick(scene, ui);
            recruitment::tick(scene, ui);
            statistics::tick(scene, ui);
            facility::tick(scene, ui);
            banpick::tick(scene, ui);
        }));
        let _ = (assets, dt);
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    // config 를 첫 로드(자동 생성/보수). 실패해도 기본값으로 진행.
    config::load();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ViewPlusExt);
    reg
}

declare_mod!(init);
