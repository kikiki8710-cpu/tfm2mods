//! tfm2_champ_pos_lock v0.7.0 — 챔피언 포지션 제한 (★0.6.0 stable ABI 재설계 2026-09-17).
//! ===========================================================================
//! 클래식(0.5.8, `_classic_058\src`)은 detour 20여 축 + 밴픽 씬 raw 읽기였다. 0.6.0 은 클래식 SDK 가 없고
//! 밴픽 코드가 바뀌어(밴픽 순서 기능 신설) 씬 오프셋·RVA 4건이 미해결 → 공식 stable API 로 재설계:
//!   · AI/코치 픽 게이트  = `StableDraftHook`(score_pick 점수 캡처 → decide_pick 이 합법 후보 중 게임 점수 최대) — `draft.rs`
//!   · 유저 픽 차단       = 밴픽 카드(`main.champions.contents.<id>`) 위에 회색 버튼 오버레이 스폰(클릭 흡수 + 사유 툴팁) — `ui_block.rs`
//!   · 스왑 확정 게이트   = 스왑 화면 표에서 (포지션, 선수) 를 읽어 위반 시 확정 버튼 비활성 + 툴팁 — `ui_block.rs`
//!   · 설정 UI            = 환경설정 게임플레이 탭에 행 스폰 + 팝업 스폰(`assets\*.ui` 임베드, `ui_spawn_source`) — `ui_popup.rs`
//!   · 설정 저장          = 세이브 mod save data(`save_get/set_string`) — 클래식과 같은 텍스트 형식(`config.rs`)
//!   · 밴픽 룰 관측       = 옵션 화면 `banpick_style`/`ban_count` 선택값 + 밴픽 화면 밴 슬롯 수
//!   · (선택) AI 팀 포지션 배정 강제 = 클래식 detour 2축(hookA 마스크·cprod 스왑 order) — `legacy_assign.rs`(cfg `ai_assign_mask`/`swap_force`)
//! 마스크·최소 선택 수·정확식(`config.rs`)·배정 순수 로직(`assign.rs`)은 클래식 그대로.
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, LogLevel, SceneKindV1, StableClient, StableExtension, StableHost, StableMod};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub mod assign;
pub mod config;
pub mod draft;
pub mod i18n;
pub mod legacy_assign;
pub mod ui_block;
pub mod ui_popup;
#[path = r"C:\tfm2mods\ui_kit\ui_kit_stable.rs"]
pub mod uk;

pub const MOD_ID: &str = "tfm2_champ_pos_lock";
pub const VERSION: &str = "0.7.0";

#[link(name = "kernel32")]
extern "system" { fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32; fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut usize) -> i32; }
pub fn mod_dir() -> Option<String> {
    let mut h: usize = 0;
    if unsafe { GetModuleHandleExW(0x4 | 0x2, mod_dir as *const () as *const u16, &mut h) } == 0 || h == 0 { return None; }
    let mut buf = [0u16; 1024];
    let n = unsafe { GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let p = String::from_utf16_lossy(&buf[..n]);
    p.rfind(|c| c == '\\' || c == '/').map(|i| p[..i].to_string())
}
pub fn dlog(s: &str) { config::dlog(s); }
/// 차단 카드 클릭 시 안내문. reason = 그 챔프의 허용 포지션 라벨("탑/정글" 등).
pub fn block_msg(id: &str, reason: Option<&str>) -> String {
    let name = disp_name(id);
    match reason { Some(p) if !p.is_empty() => i18n::trf("block_msg_pos", &[("name", &name), ("pos", p)]), _ => i18n::trf("block_msg", &[("name", &name)]) }
}

// ───────── 로스터(현재 세이브의 사용 가능 챔피언) ─────────
#[derive(Default, Clone)]
pub struct Roster {
    /// 소문자 id (게임 순서)
    pub ids: Vec<String>,
    /// id → 표시명(현재 언어)
    pub names: HashMap<String, String>,
    /// id → 클래스(0 전사 1 원거리 2 마법사 3 전투보조 4 암살자, 255 미상)
    pub cats: HashMap<String, u8>,
    /// 표시명 순 정렬
    pub sorted: Vec<String>,
    pub sig: u64,
}
static ROSTER: Mutex<Option<Arc<Roster>>> = Mutex::new(None);
pub static ROSTER_DIRTY: AtomicBool = AtomicBool::new(true);
pub fn roster() -> Option<Arc<Roster>> { ROSTER.lock().unwrap_or_else(|e| e.into_inner()).clone() }
pub fn disp_name(id: &str) -> String { roster().and_then(|r| r.names.get(id).cloned()).unwrap_or_else(|| id.to_string()) }

fn roster_sig(ids: &[String]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    ids.hash(&mut h);
    h.finish()
}

/// 로스터 캡처: `champion_names()` + i18n 표시명 + `champion_brief` 클래스. 서명이 같으면 게시하지 않는다.
fn capture_roster(ctx: &StableClient<'_>) {
    let raw = ctx.champion_names();
    if raw.is_empty() { return; }
    let ids: Vec<String> = raw.iter().map(|s| s.to_ascii_lowercase()).collect();
    let sig = roster_sig(&ids);
    if roster().map(|r| r.sig == sig).unwrap_or(false) { return; }
    let mut names = HashMap::new();
    let mut cats = HashMap::new();
    for id in &ids {
        let n = ctx.i18n(&format!("#asset/base/text/champion?description.{}.name", id)).filter(|s| !s.is_empty() && !s.starts_with('#')).unwrap_or_else(|| id.clone());
        names.insert(id.clone(), n);
        let c = ctx.champion_brief(id).map(|b| b.category.map(|c| c as u32 as u8).unwrap_or(255)).unwrap_or(255);
        cats.insert(id.clone(), c);
    }
    let mut sorted = ids.clone();
    sorted.sort_by_cached_key(|id| names.get(id).cloned().unwrap_or_default().to_lowercase());
    config::slog(&format!("로스터 갱신: 챔프 {}종", ids.len()));
    config::set_roster(&ids);
    *ROSTER.lock().unwrap_or_else(|e| e.into_inner()) = Some(Arc::new(Roster { ids, names, cats, sorted, sig }));
    MASK_VER.store(u64::MAX, Ordering::Relaxed);
}

// ───────── 마스크 캐시(id → 5비트) — state/roster 버전으로 무효화 ─────────
static MASKS: Mutex<Option<Arc<HashMap<String, u8>>>> = Mutex::new(None);
static MASK_VER: AtomicU64 = AtomicU64::new(u64::MAX);
/// 현재 설정 기준 챔프 마스크 표(미지정 챔프는 config 규칙대로 — 목록 있는 자리엔 못 감).
pub fn masks() -> Option<Arc<HashMap<String, u8>>> {
    let ver = config::state_version();
    if MASK_VER.load(Ordering::Relaxed) != ver {
        let r = roster()?;
        let mut m = HashMap::with_capacity(r.ids.len());
        for id in &r.ids { m.insert(id.clone(), config::mask_of(id)); }
        *MASKS.lock().unwrap_or_else(|e| e.into_inner()) = Some(Arc::new(m));
        MASK_VER.store(ver, Ordering::Relaxed);
    }
    MASKS.lock().unwrap_or_else(|e| e.into_inner()).clone()
}
pub fn mask_of(id_lower: &str) -> u8 { masks().and_then(|m| m.get(id_lower).copied()).unwrap_or_else(|| config::mask_of(id_lower)) }

// ───────── 세이브 연동 ─────────
const SAVE_KEY: &str = "positions";
const SAVE_NS_VERSION: usize = 1;
static SAVE_LOADED: AtomicBool = AtomicBool::new(false);
static SAVE_MISS: AtomicUsize = AtomicUsize::new(0);
const SAVE_MISS_GRACE: usize = 240;
pub static PENDING_SAVE: Mutex<Option<String>> = Mutex::new(None);
pub static PLAYER_TEAM: AtomicU64 = AtomicU64::new(u64::MAX);
pub static FRAME: AtomicU64 = AtomicU64::new(0);

fn save_tick(ctx: &mut StableClient<'_>) {
    // ① UI 확인이 이월한 기록 대기분
    let pending = PENDING_SAVE.lock().unwrap_or_else(|e| e.into_inner()).take();
    if let Some(body) = pending {
        if ctx.save_can_write() {
            ctx.save_set_version(SAVE_NS_VERSION);
            let ok = ctx.save_set_string(SAVE_KEY, &body);
            config::slog(&format!("세이브 기록 {}: {}B", if ok { "OK" } else { "거부(FALSE)" }, body.len()));
            if ok { config::apply_state_text(&body); SAVE_LOADED.store(true, Ordering::Relaxed); }
        } else {
            config::slog("세이브 기록 불가: save_can_write=false — 이번 저장은 반영 안 됨");
            *PENDING_SAVE.lock().unwrap_or_else(|e| e.into_inner()) = Some(body); // 다음 프레임 재시도
        }
    }
    // ② 최초 1회 로드(로스터 캡처 후)
    if !SAVE_LOADED.load(Ordering::Relaxed) && config::roster_ready() {
        match ctx.save_get_string(SAVE_KEY) {
            Some(txt) => {
                config::apply_state_text(&txt);
                SAVE_LOADED.store(true, Ordering::Relaxed);
                let miss = SAVE_MISS.swap(0, Ordering::Relaxed);
                config::slog(&format!("세이브 설정 로드: {}B / 포지션별 {:?} (대기 {miss}프레임)", txt.len(), (0..5).map(config::pos_count).collect::<Vec<_>>()));
            }
            None => {
                let n = SAVE_MISS.fetch_add(1, Ordering::Relaxed) + 1;
                if n >= SAVE_MISS_GRACE {
                    config::apply_state_text("");
                    SAVE_LOADED.store(true, Ordering::Relaxed);
                    config::slog(&format!("이 세이브엔 설정 없음({n}프레임 확인) → 제한 없음 / 로스터 {}종", roster().map(|r| r.ids.len()).unwrap_or(0)));
                }
            }
        }
    }
}
fn leave_save() {
    SAVE_MISS.store(0, Ordering::Relaxed);
    if SAVE_LOADED.swap(false, Ordering::Relaxed) {
        config::clear_state();
        ROSTER_DIRTY.store(true, Ordering::Relaxed);
        PLAYER_TEAM.store(u64::MAX, Ordering::Relaxed);
        ui_block::reset();
        config::slog("세이브 밖으로 나감 → 설정·내팀 비움");
    }
}

// ───────── 밴픽 룰 관측(옵션 화면) ─────────
static OPT_PATH: Mutex<Option<String>> = Mutex::new(None); // 옵션 화면 루트(`current_database_edit` 의 조상 = contents 의 부모)
static OPT_SCAN_AT: AtomicU64 = AtomicU64::new(0);
/// 옵션 화면 `contents` 경로(= `current_database_edit` 의 부모). 없으면 None. 60프레임마다 재탐색.
pub fn option_contents(ctx: &StableClient<'_>) -> Option<String> {
    if let Some(p) = OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()).clone() {
        if ctx.ui_exists(&p) { return Some(p); }
        *OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
    let f = FRAME.load(Ordering::Relaxed);
    if f.saturating_sub(OPT_SCAN_AT.swap(f, Ordering::Relaxed)) < 60 && f > 60 { return None; }
    // 상위 루트들에서 얕은 DFS(깊이 5)
    fn dfs(ctx: &StableClient<'_>, path: &str, depth: usize) -> Option<String> {
        if depth > 5 { return None; }
        for c in ctx.ui_child_names(path) {
            let full = if path.is_empty() { c.clone() } else { format!("{}.{}", path, c) };
            if c == "current_database_edit" { return Some(path.to_string()); }
            if let Some(r) = dfs(ctx, &full, depth + 1) { return Some(r); }
        }
        None
    }
    let found = dfs(ctx, "", 0);
    if let Some(p) = &found { dlog(&format!("옵션 contents 경로 = {}", p)); }
    *OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()) = found.clone();
    found
}
/// 옵션 화면의 밴픽 스타일/밴 수 선택값 → config::set_rule. (게임플레이 탭이 보일 때만 의미 있음)
fn observe_rule_from_option(ctx: &StableClient<'_>, contents: &str) {
    let style_root = format!("{}.banpick_style.bg", contents);
    if !ctx.ui_exists(&style_root) { return; }
    let style = if ctx.ui_selectable_selected(&format!("{}.fearless_hard", style_root)) == Some(true) { 2u8 }
        else if ctx.ui_selectable_selected(&format!("{}.fearless", style_root)) == Some(true) { 1 } else { 0 };
    let ban_root = format!("{}.ban_count.bg", contents);
    let mut ban: Option<usize> = None;
    for (i, id) in ["ban1", "ban2", "ban3", "ban4", "ban5", "ban5_split"].iter().enumerate() {
        if ctx.ui_selectable_selected(&format!("{}.{}", ban_root, id)) == Some(true) { ban = Some((i + 1).min(5)); }
    }
    let (cs, cb) = config::cur_rule();
    if cs != style || (ban.is_some() && cb != ban) { config::set_rule(style, ban.or(cb)); }
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            FRAME.fetch_add(1, Ordering::Relaxed);
            uk::frame_begin();
            let cfg = config::get();
            if !cfg.enabled { return; }
            i18n::poll_lang();
            if ctx.scene_kind() != Some(SceneKindV1::InGame) { leave_save(); return; }
            // 내 팀 id(비0 을 봤으면 0 으로 후퇴 안 함 — 조합테스트 등)
            if let Some(t) = ctx.player_team_id() { if t != 0 || PLAYER_TEAM.load(Ordering::Relaxed) == u64::MAX { PLAYER_TEAM.store(t as u64, Ordering::Relaxed); } }
            if ROSTER_DIRTY.swap(false, Ordering::Relaxed) || roster().is_none() { capture_roster(ctx); }
            save_tick(ctx);
            let _ = masks();
            legacy_assign::install_once();
            // 옵션 화면(환경설정): 행/팝업 + 룰 관측
            if let Some(contents) = option_contents(ctx) {
                observe_rule_from_option(ctx, &contents);
                ui_popup::tick(ctx, &contents);
            } else { ui_popup::hidden(); }
            // 밴픽/스왑 화면
            ui_block::tick(ctx);
            draft::drain_logs();
        }));
    }
}

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "tfm2_champ_pos_lock v0.7.0 (stable 0.6.0)");
    config::load();
    i18n::load();
    let v = host.game_version();
    config::slog(&format!("INIT v{} game {}.{}.{} host_abi={}", VERSION, v.major, v.minor, v.patch, host.abi_level()));
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d.add_draft_score_hook(draft::PosLockDraft);
    d
}
declare_stable_mod!(init);
