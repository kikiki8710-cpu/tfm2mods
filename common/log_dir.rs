//! 로그 출력 폴더 결정 — 모드 공용 (2026-10-21). 각 모드는 복사하지 말고 `#[path = "../../common/log_dir.rs"] mod log_dir;` 로 가져온다.
//!
//! `<게임>\mods\_log_root.txt` 의 첫 줄(폴더 경로, 예 `E:\claude\tfm2\logs\game_mods`)이 있으면 `<그 폴더>\<mod_id>` 에 쓴다.
//! 파일이 없거나 비었거나 폴더를 못 만들면 원래대로 모드 폴더에 쓴다 → 배포본(다른 유저 PC)은 동작 그대로.
//! ⚠ 로그(진단·덤프·크래시 기록)만 옮긴다. 설정·상태 파일(*.cfg, mod_order.txt 등 모드가 다시 읽는 파일)은 모드 폴더 그대로.
//! 패닉 없음(unwrap 금지) — detour·VEH 경로에서 불려도 안전하게 실패하면 모드 폴더를 돌려준다.
#![allow(dead_code)]
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static CACHE: OnceLock<Option<PathBuf>> = OnceLock::new();

fn resolve(mod_dir: &Path) -> Option<PathBuf> {
    let id = mod_dir.file_name()?;
    let text = std::fs::read_to_string(mod_dir.parent()?.join("_log_root.txt")).ok()?;
    let root = text.lines().next()?.trim().trim_start_matches('\u{feff}');
    if root.is_empty() {
        return None;
    }
    let d = PathBuf::from(root).join(id);
    std::fs::create_dir_all(&d).ok()?;
    Some(d)
}

/// 로그를 쓸 폴더. 첫 호출 결과를 프로세스 동안 고정한다(매 줄마다 파일을 다시 읽지 않게).
pub fn log_dir_path(mod_dir: &Path) -> PathBuf {
    CACHE.get_or_init(|| resolve(mod_dir)).clone().unwrap_or_else(|| mod_dir.to_path_buf())
}

/// 문자열 경로를 쓰는 모드용(`format!(r"{}\debug.log", d)` 류).
pub fn log_dir(mod_dir: &str) -> String {
    log_dir_path(Path::new(mod_dir)).to_string_lossy().into_owned()
}
