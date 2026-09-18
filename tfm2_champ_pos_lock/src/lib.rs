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
pub mod swap_confirm_hook;
pub mod ai_swap;
pub mod ui_popup;
#[path = r"C:\tfm2mods\ui_kit\ui_kit_stable.rs"]
pub mod uk;
#[path = r"C:\tfm2mods\ui_kit\client_db_stable.rs"]
pub mod cdb;
#[path = r"C:\tfm2mods\ui_kit\draft_scene_stable.rs"]
pub mod draft_scene;
#[path = r"C:\tfm2mods\ui_kit\dropdown_stable.rs"]
pub mod dd;

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

/// ClientDatabase.available_champions(Vec<String>) raw 읽기 — 원소 전부 registry id 일 때만 Some.
fn read_available(ctx: &StableClient<'_>, registry: &[String]) -> Option<std::collections::HashSet<String>> {
    const OFF_AVAIL_CAP: usize = 0xe740;
    let db = cdb::client_db(ctx)?;
    unsafe {
        let cap = cdb::rd_u64(db + OFF_AVAIL_CAP)? as usize;
        let ptr = cdb::rd_u64(db + OFF_AVAIL_CAP + 8)? as usize;
        let len = cdb::rd_u64(db + OFF_AVAIL_CAP + 0x10)? as usize;
        if len == 0 || len > cap || len > 1024 || !cdb::readable(ptr, len * 0x18) { return None; }
        let reg: std::collections::HashSet<&String> = registry.iter().collect();
        let mut out = std::collections::HashSet::with_capacity(len);
        for i in 0..len {
            let e = ptr + i * 0x18;
            let sp = cdb::rd_u64(e + 8)? as usize; let sl = cdb::rd_u64(e + 0x10)? as usize;
            if sl == 0 || sl > 64 || !cdb::readable(sp, sl) { return None; }
            let name = std::str::from_utf8(core::slice::from_raw_parts(sp as *const u8, sl)).ok()?.to_ascii_lowercase();
            if !reg.contains(&name) { return None; }
            out.insert(name);
        }
        Some(out)
    }
}

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
    let mut ids: Vec<String> = raw.iter().map(|s| s.to_ascii_lowercase()).collect();
    // ★0.6.0(09-17 유저 제보 "아직 추가 안 된 챔피언이 목록에 있다"): champion_names() 는 registry 전체(미출시 포함) →
    //   ClientDatabase.available_champions(cdb+0xe740, RE 09-17)로 출시분만 남긴다. 읽기 실패(레이아웃 stale)면 전체 유지.
    if let Some(avail) = read_available(ctx, &ids) { ids.retain(|id| avail.contains(id)); if ids.is_empty() { return; } }
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
static AISWAP_LAST_FIRE: AtomicU64 = AtomicU64::new(0);

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
                // ★09-17: 현재 세이브의 설정 본문을 파일로도 덤프(사용자 문의·지원용 — 세이브 안에만 있어 밖에서 볼 수 없었다)
                if let Some(d) = mod_dir() { let _ = std::fs::write(format!("{}\\champ_pos_lock_state.txt", d), &txt); }
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
/// 옵션 화면 `contents` 경로(= `current_database_edit` 의 부모). 없으면 None.
/// ★성능(2026-09-17 유저 제보 "밴픽창 너무 느려"): 전 트리 DFS 를 매초 돌리면 밴픽 화면(카드 160장×자식) 에서 수천 호출 스파이크.
///   → 관리 씬(Main)에서만, 최상위 루트 이름에 "option" 이 있을 때 그 루트 아래만 얕게(깊이 4) 탐색. 120프레임 간격.
pub fn option_contents(ctx: &StableClient<'_>) -> Option<String> {
    // ★09-17: 가드 임시값 수명 데드락 방지(comptest 실사고와 같은 패턴) — 가드를 별도 문장으로 먼저 해제.
    let cached = OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if let Some(p) = cached {
        if ctx.ui_exists(&p) { return Some(p); }
        *OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
    if ctx.client_scene_kind() != Some(mod_api_stable::ClientSceneKindV1::Main) { return None; }
    let f = FRAME.load(Ordering::Relaxed);
    if f.saturating_sub(OPT_SCAN_AT.load(Ordering::Relaxed)) < 120 && f > 120 { return None; }
    OPT_SCAN_AT.store(f, Ordering::Relaxed);
    fn dfs(ctx: &StableClient<'_>, path: &str, depth: usize) -> Option<String> {
        if depth > 4 { return None; }
        for c in ctx.ui_child_names(path) {
            let full = if path.is_empty() { c.clone() } else { format!("{}.{}", path, c) };
            if c == "current_database_edit" { return Some(path.to_string()); }
            if let Some(r) = dfs(ctx, &full, depth + 1) { return Some(r); }
        }
        None
    }
    // ① 싼 후보 경로 먼저(옵션 팝업 루트 id 가 "option" 인 경우)
    let mut found: Option<String> = ["pause_ui.option.option.contents", "option.option.contents", "body.option.option.contents", "pause_ui.option.contents", "option.contents", "main.option.option.contents", "main.pause_ui.option.option.contents"].iter()
        .find(|p| ctx.ui_exists(&format!("{}.current_database_edit", p))).map(|p| p.to_string());
    // ② 최상위 루트 중 이름에 option 이 든 것만 얕게 DFS (main 전체 트리는 절대 안 훑음)
    let roots = ctx.ui_child_names("");
    if found.is_none() { for r in roots.iter().filter(|r| r.to_ascii_lowercase().contains("option")) { if let Some(p) = dfs(ctx, r, 1) { found = Some(p); break; } } }
    if found.is_none() && OPT_ROOTS_LOGGED.fetch_add(1, Ordering::Relaxed) < 6 {
        // ★진단(0.6.0 검증): 루트 열거가 [] 라 후보 루트를 ui_exists 로 직접 더듬는다 — 6회까지만
        let probe = ["pause_ui", "pause_ui.option", "pause_ui.pause", "pause_ui.fade", "main.pause_ui", "option.option", "main", "body", "pause", "option", "popup", "overlay", "modal", "root", "ingame", "top", "system", "dialog", "menu", "layer", "screen", "main.pause", "main.option", "main.popup", "main.overlay", "main.top", "body.option", "pause.option", "popup.option", "overlay.option"];
        let mut hits: Vec<String> = Vec::new();
        for r in probe { if ctx.ui_exists(r) { let kids = ctx.ui_child_names(r); hits.push(format!("{}={:?}", r, kids.iter().take(24).collect::<Vec<_>>())); } }
        fn lst(ctx: &StableClient<'_>, path: &str, depth: usize, out: &mut Vec<String>) {
            if depth == 0 || out.len() > 240 { return; }
            for c in ctx.ui_child_names(path) { let full = format!("{}.{}", path, c); out.push(full.clone()); lst(ctx, &full, depth - 1, out); }
        }
        let mut tree: Vec<String> = Vec::new();
        for r in ["main.bottom", "main.tooltip", "main.bg", "main.top.left"] { lst(ctx, r, 3, &mut tree); }
        dlog(&format!("옵션 루트 탐색 실패 — 최상위 루트 = {:?} | 프로브 hit = {:?} | 트리 = {:?}", roots, hits, tree));
    }
    if let Some(p) = &found { dlog(&format!("옵션 contents 경로 = {}", p)); }
    *OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()) = found.clone();
    found
}
static OPT_ROOTS_LOGGED: AtomicUsize = AtomicUsize::new(0);
static RULE_DIAG: AtomicBool = AtomicBool::new(false);
/// ★0.6.0(09-17, RE `RE\2026-09-17_0.6.0-GamePlayOption-…`): 옵션 값을 UI 로 못 읽음(pause 팝업 안 selectable = None) →
/// ClientDatabase 의 GamePlayOption 을 raw 로 읽는다. banpick_style u8 cdb+0x738(0/1/2) · room_practice_ban_count u64 cdb+0x720
/// (0=기본) · available len cdb+0xe750. 실효 밴 수 = 게임 공식(5v5): (1≤n≤5 && avail ≥ 2n+20·style+15) ? n : default,
/// default = (avail≥40 ? 3 : 2), avail < 2·default+20·style+15 면 default=2. 옵션 화면이 아니어도 120프레임마다 관측(밴픽 전 확정).
const OFF_BAN_COUNT: usize = 0x720;
const OFF_TWO_PHASE: usize = 0x733;
const OFF_BANPICK_STYLE: usize = 0x738;
const OFF_AVAIL_LEN: usize = 0xe750;
static RULE_LOGGED: AtomicU64 = AtomicU64::new(u64::MAX);
fn observe_rule_raw(ctx: &StableClient<'_>) {
    if ctx.ui_exists("main.champions.contents") { return; } // 밴픽 중엔 슬롯 실측(밴카드 관측)이 우선 — 왕복 덮어쓰기 방지
    let Some(db) = cdb::client_db(ctx) else { return };
    let (style, n, two, avail) = unsafe {
        (cdb::rd_u32(db + OFF_BANPICK_STYLE).map(|v| (v & 0xff) as u8), cdb::rd_u64(db + OFF_BAN_COUNT),
         cdb::rd_u32(db + OFF_TWO_PHASE).map(|v| (v & 0xff) as u8), cdb::rd_u64(db + OFF_AVAIL_LEN))
    };
    let (Some(style), Some(n), Some(two), Some(avail)) = (style, n, two, avail) else { return };
    if style > 2 || n > 64 || two > 1 || avail > 4096 { return; } // 레이아웃 stale 가드
    let n = n as usize; let avail = avail as usize; let st = style as usize;
    let mut default = if avail >= 40 { 3 } else { 2 };
    if avail < 2 * default + 20 * st + 15 { default = 2; }
    let eff = if (1..=5).contains(&n) && avail >= 2 * n + 20 * st + 15 { n } else { default };
    let sig = ((style as u64) << 48) | ((n as u64) << 32) | ((two as u64) << 24) | ((eff as u64) << 16) | (avail as u64 & 0xffff);
    if RULE_LOGGED.swap(sig, Ordering::Relaxed) != sig { dlog(&format!("룰 raw: style={} ban_opt={}(0=기본) two_phase={} avail={} → 실효 밴 {}", style, n, two, avail, eff)); }
    let (cs, cb) = config::cur_rule();
    if cs != style || cb != Some(eff) { config::set_rule(style, Some(eff)); }
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            FRAME.fetch_add(1, Ordering::Relaxed);
            uk::frame_begin();
            let cfg = config::get();
            if !cfg.enabled { return; }
            i18n::poll_lang();
            if ctx.scene_kind() != Some(SceneKindV1::InGame) { leave_save(); ui_popup::reset_registrations(); ui_block::reset(); *OPT_PATH.lock().unwrap_or_else(|e| e.into_inner()) = None; return; }
            // 내 팀 id(비0 을 봤으면 0 으로 후퇴 안 함 — 조합테스트 등)
            if let Some(t) = ctx.player_team_id() { if t != 0 || PLAYER_TEAM.load(Ordering::Relaxed) == u64::MAX { PLAYER_TEAM.store(t as u64, Ordering::Relaxed); } }
            if ROSTER_DIRTY.swap(false, Ordering::Relaxed) || roster().is_none() || FRAME.load(Ordering::Relaxed) % 600 == 0 { capture_roster(ctx); } // 600프레임 주기 재캡처 = 패치데이 출시분 반영(서명 같으면 no-op)
            save_tick(ctx);
            let _ = masks();
            legacy_assign::install_once();
            // 밴픽 씬 포인터 캡처(update 진입 detour · 09-18 RE) — 스왑 order raw 읽기용. 설치 결과 1회 로그.
            if let Some(msg) = draft_scene::install_once() { config::dlog(&msg); config::llog(&msg); }
            draft_scene::tick();
            if let Some(msg) = swap_confirm_hook::install_once() { config::dlog(&msg); config::llog(&msg); }
            if let Some(msg) = ai_swap::install_once() { config::dlog(&msg); config::llog(&msg); }
            if let Some(msg) = ai_swap::drain_log() { config::llog(&msg); config::dlog(&msg); }
            if FRAME.load(Ordering::Relaxed) % 600 == 0 { let f = ai_swap::CNT_FIRE.load(Ordering::Relaxed); if f != AISWAP_LAST_FIRE.swap(f, Ordering::Relaxed) { config::llog(&format!("aiswap counters: fire={} rewrite={} skip={}", f, ai_swap::CNT_REWRITE.load(Ordering::Relaxed), ai_swap::CNT_SKIP.load(Ordering::Relaxed))); } }
            // 옵션 화면(환경설정): 행/팝업 + 룰 관측
            if FRAME.load(Ordering::Relaxed) % 120 == 0 { observe_rule_raw(ctx); }
            if let Some(contents) = option_contents(ctx) {
                ui_popup::tick(ctx, &contents);
            } else { ui_popup::hidden(); }
            // 밴픽/스왑 화면
            ui_block::tick(ctx);
            draft::drain_logs();
        }));
        // ★09-17: post_update 안 패닉은 조용히 삼켜져 뒷단(ui_block)이 통째로 죽는다 → 페이로드를 로그(같은 메시지 1회).
        if let Err(e) = r {
            let msg = e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_else(|| "?".into());
            let n = PANIC_CNT.fetch_add(1, Ordering::Relaxed);
            if n < 5 { config::slog(&format!("post_update 패닉 #{}: {}", n + 1, msg)); }
        }
    }
}
static PANIC_CNT: AtomicUsize = AtomicUsize::new(0);

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
