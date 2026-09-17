//! tfm2_banpick_illust v1.4.0 — ★0.6.0 stable ABI 껍데기, **쇼케이스 전용**(2026-09-17).
//! 클래식 v1.3.3(`_classic_058\src`)의 기능 중 픽슬롯 배경 일러·버프/너프 이름색·8각 레이더는 0.6.0 에서
//! `banpick_view_plus`(stable 재작성) 가 담당 → 여기선 **밴/픽 셀렉트 연출(쇼케이스) 카드에 일러스트 표시**(`showcase.rs`,
//! RVA 훅 3 + FFI 14 + 기하 패치 12 — 0.6.0 재핀 29/29 PASS)만 남긴다. `.ui` 오버라이드도 제거(뷰플러스와 같은 파일 충돌 방지).
//! 일러 소스 = `<mod>\illust\{blue,red,red_noflip}\<champ>.png`(에셋 키 `asset/tfm2_banpick_illust/illust/<side>/<champ>`).
//! 설정 = `tfm2_banpick_illust.cfg`(showcase/red_flip/zoom/debug 만 의미 있음).
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, LogLevel, StableClient, StableExtension, StableHost, StableMod};

mod config;
mod keys;
mod showcase;

pub(crate) const MOD_ID: &str = "tfm2_banpick_illust";
const ILLUST_KEY: &str = "asset/tfm2_banpick_illust/illust/";

#[link(name = "kernel32")]
extern "system" { fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32; }
pub(crate) fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\{}", dir.display(), MOD_ID))
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, _ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !config::get().enabled { return; }
            showcase::tick(); // 늦은 1회 설치(멱등)
        }));
    }
}

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "tfm2_banpick_illust v1.4.0 (stable 0.6.0, showcase only)");
    config::load();
    let v = host.game_version();
    if config::get().debug { config::write_diag("load_stamp.txt", &format!("init game {}.{}.{} host_abi={}\n", v.major, v.minor, v.patch, host.abi_level())); }
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d
}
declare_stable_mod!(init);
