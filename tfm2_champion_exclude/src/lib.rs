//! tfm2_champion_exclude v0.6.0 — 인게임 시즌 패치의 신챔프 추가 대상에서 특정 챔피언 영구 제외.
//! ★게임 0.6.0 stable ABI 이식(2026-09-17). 클래식 0.5.8 소스 = `_classic_058\src`(detour + 클래식 UI 주입 + 번들 UV 아이콘).
//! ===========================================================================
//! 기능(클래식과 동일): 패치데이 "후보 Vec<String> 생성" 함수(HOOK_RVA) 진입 트램폴린 → 원본 호출 후 제외 id 를 swap_remove.
//!   후보에서 빠지면 셔플/선택/available push/액션 등록/팀 티어/뉴스까지 자연 배제. '*' = 전량 차단.
//!   빈 후보 = 바닐라 "전 챔피언 출시완료"와 비트동일(알림 0건 — RE 2026-08-20 정적 확증).
//! 0.6.0 stable 로 바뀐 것:
//!   · 설정 저장   = `save_get/set_string`(세이브 mod save data, ns=모드·키 "exclude"·텍스트 형식 그대로). 쓰기는 큐잉 → 로컬 캐시 선반영(v0.4.1 교훈).
//!   · 설정 UI     = 환경설정 게임플레이 탭 행 + 팝업을 `ui_spawn_source` 로 스폰(경로 `pause_ui.option.option.contents`, champ_pos_lock 0.7.0 동형).
//!                   클래스 필터 = 드롭다운 대신 선택탭 6개(stable 엔 드롭다운 옵션 주입 없음). 아이콘 = `ui_set_champion_icon`.
//!   · 미출시 목록 = `champion_names()`(registry 전체: 바닐라+모드) − available_champions(raw ClientDatabase 읽기, `avail.rs`) —
//!                   raw 읽기 실패 시 패치데이 관측 캐시(seen) ∪ 세이브 설정으로 폴백(라벨로 표시).
//!   · 이름/정렬   = `ctx.i18n("#asset/base/text/champion?description.<id>.name")`(현재 언어 자동) — 클래식의 lang 파일 파서 불요.
//! 안전: detour 본문 catch_unwind 격리·프롤로그 17B 검증·외부 훅 체인·post_update 패닉 페이로드 로그.
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, ClientSceneKindV1, LogLevel, SceneKindV1, StableClient, StableExtension, StableHost, StableMod};
use std::collections::{HashMap, HashSet};
use std::io::Write as _;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub mod avail;
pub mod hook;
pub mod ui;
#[path = r"C:\tfm2mods\ui_kit\ui_kit_stable.rs"]
pub mod uk;
#[path = r"C:\tfm2mods\ui_kit\client_db_stable.rs"]
pub mod cdb;
#[path = r"C:\tfm2mods\ui_kit\dropdown_stable.rs"]
pub mod dd;

pub const MOD_ID: &str = "tfm2_champion_exclude";
pub const VERSION: &str = "0.6.0";
pub const I18N: &str = "#asset/base/text/ui?champ_excl.";
/// 진단 로그(mods\tfm2_champion_exclude\champion_exclude.txt). 패치데이·UI 이벤트 단위라 저볼륨 — 상시 on.
const LOG_ENABLED: bool = false; // 09-19 확정 배포(진단 시 true)

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
fn ts() -> String {
    let ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let s = ms / 1000 + 9 * 3600;
    format!("{:02}:{:02}:{:02}.{:03}", (s / 3600) % 24, (s / 60) % 60, s % 60, ms % 1000)
}
pub fn log(msg: &str) {
    if !LOG_ENABLED { return; }
    if let Some(d) = mod_dir() {
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!("{}\\champion_exclude.txt", d)) { let _ = writeln!(f, "[{}] {}", ts(), msg); }
    }
}

// ── 제외 목록(세이브 텍스트) ──────────────────────────────────────────────
/// '#' 주석·'*'=전면차단·소문자 정규화.
pub fn parse_exclude_text(text: &str) -> (Vec<String>, bool) {
    let mut list = Vec::new();
    let mut block_all = false;
    for line in text.trim_start_matches('\u{feff}').lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() { continue; }
        if line == "*" { block_all = true; continue; }
        list.push(line.to_ascii_lowercase());
    }
    (list, block_all)
}
pub const SAVE_KEY: &str = "exclude";
pub const SAVE_NS_VERSION: usize = 1;
/// 현재 세이브의 제외 목록 캐시(None = 설정 없음/세이브 밖). detour 가 읽는다.
static SAVE_EXCL: Mutex<Option<(Vec<String>, bool)>> = Mutex::new(None);
/// UI 확인 → 기록 대기 본문(클릭 콜백엔 ctx 가 없어 프레임으로 이월).
pub static PENDING_SAVE: Mutex<Option<String>> = Mutex::new(None);
static SAVE_POLL_AT: AtomicU64 = AtomicU64::new(0);
static SAVE_WRITTEN_AT: AtomicU64 = AtomicU64::new(0);

pub fn effective_exclusion() -> (Vec<String>, bool, &'static str) {
    if let Some((l, s)) = SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner()).clone() { return (l, s, "save"); }
    (Vec::new(), false, "none")
}
pub fn has_save_setting() -> bool { SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner()).is_some() }

fn save_tick(ctx: &mut StableClient<'_>, f: u64) {
    // ① UI 확인이 이월한 기록
    let pending = PENDING_SAVE.lock().unwrap_or_else(|e| e.into_inner()).take();
    if let Some(body) = pending {
        if ctx.save_can_write() {
            ctx.save_set_version(SAVE_NS_VERSION);
            let ok = ctx.save_set_string(SAVE_KEY, &body);
            log(&format!("save write {}: {}B", if ok { "OK" } else { "REJECTED(false)" }, body.len()));
            if ok {
                // ★엔진 쓰기는 큐잉 — get 이 다음 동기까지 옛값을 줄 수 있어 로컬 캐시 선반영(v0.4.1 교훈).
                *SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner()) = Some(parse_exclude_text(&body));
                SAVE_WRITTEN_AT.store(f, Ordering::Relaxed);
            }
        } else {
            log("cannot write save: save_can_write=false (multiplayer non-host?) - selection not saved");
        }
    }
    // ② 세이브 현행 설정 폴링(60프레임) — 방금 쓴 뒤 300프레임은 큐잉 지연 동안 캐시 유지
    if f.saturating_sub(SAVE_POLL_AT.load(Ordering::Relaxed)) < 60 { return; }
    SAVE_POLL_AT.store(f, Ordering::Relaxed);
    if f.saturating_sub(SAVE_WRITTEN_AT.load(Ordering::Relaxed)) < 300 && SAVE_WRITTEN_AT.load(Ordering::Relaxed) != 0 { return; }
    if let Some(t) = ctx.save_get_string(SAVE_KEY) {
        let parsed = parse_exclude_text(&t);
        let mut g = SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner());
        if g.as_ref() != Some(&parsed) { log(&format!("save setting loaded: {} ids{}", parsed.0.len(), if parsed.1 { " + *" } else { "" })); *g = Some(parsed); }
    }
}

// ── 후보(미출시 챔피언) ──────────────────────────────────────────────────
#[derive(Default)]
pub struct Cands {
    /// 미출시 id(표시명 순 정렬)
    pub ids: Vec<String>,
    pub names: HashMap<String, String>,
    pub cats: HashMap<String, u8>,
    /// available 을 raw 로 읽었는가(false = seen/세이브 폴백)
    pub avail_ok: bool,
    pub registry_n: usize,
    pub avail_n: usize,
    pub sig: u64,
}
static CANDS: Mutex<Option<Arc<Cands>>> = Mutex::new(None);
pub fn cands() -> Option<Arc<Cands>> { CANDS.lock().unwrap_or_else(|e| e.into_inner()).clone() }
static CAND_AT: AtomicU64 = AtomicU64::new(0);
pub static CAND_FORCE: AtomicBool = AtomicBool::new(false);
pub static FRAME: AtomicU64 = AtomicU64::new(0);

fn recompute_candidates(ctx: &StableClient<'_>, f: u64) {
    if !CAND_FORCE.swap(false, Ordering::Relaxed) && f.saturating_sub(CAND_AT.load(Ordering::Relaxed)) < 600 && CAND_AT.load(Ordering::Relaxed) != 0 { return; }
    CAND_AT.store(f, Ordering::Relaxed);
    let registry: Vec<String> = ctx.champion_names().iter().map(|s| s.to_ascii_lowercase()).collect();
    if registry.is_empty() { return; }
    let reg_set: HashSet<String> = registry.iter().cloned().collect();
    let avail = avail::read(ctx, &reg_set);
    let (ids_raw, avail_ok, avail_n): (Vec<String>, bool, usize) = match &avail {
        Some(a) => (registry.iter().filter(|id| !a.contains(*id)).cloned().collect(), true, a.len()),
        None => {
            // 폴백: 패치데이 관측 캐시 ∪ 세이브 설정 중 registry 에 있는 것
            let mut s = hook::load_seen();
            s.extend(effective_exclusion().0);
            (registry.iter().filter(|id| s.contains(*id)).cloned().collect(), false, 0)
        }
    };
    let sig = { use std::hash::{Hash, Hasher}; let mut h = std::collections::hash_map::DefaultHasher::new(); (&ids_raw, avail_ok, registry.len()).hash(&mut h); h.finish() };
    if cands().map(|c| c.sig == sig).unwrap_or(false) { return; }
    let mut names = HashMap::new();
    let mut cats = HashMap::new();
    for id in &ids_raw {
        let n = ctx.i18n(&format!("#asset/base/text/champion?description.{}.name", id)).filter(|s| !s.is_empty() && !s.starts_with('#')).unwrap_or_else(|| id.clone());
        names.insert(id.clone(), n);
        let c = ctx.champion_brief(id).map(|b| b.category.map(|c| c as u32 as u8).unwrap_or(255)).unwrap_or(255);
        cats.insert(id.clone(), c);
    }
    let mut ids = ids_raw.clone();
    ids.sort_by(|a, b| names.get(a).cloned().unwrap_or_default().to_lowercase().cmp(&names.get(b).cloned().unwrap_or_default().to_lowercase()).then_with(|| a.cmp(b)));
    log(&format!("candidates recomputed: {} unreleased (registry {}, available {}{})", ids.len(), registry.len(), avail_n, if avail_ok { "" } else { " — raw read FAILED, fallback seen∪save" }));
    *CANDS.lock().unwrap_or_else(|e| e.into_inner()) = Some(Arc::new(Cands { ids, names, cats, avail_ok, registry_n: registry.len(), avail_n, sig }));
    ui::grid_dirty();
}

// ── 확장 ─────────────────────────────────────────────────────────────────
struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f = FRAME.fetch_add(1, Ordering::Relaxed) + 1;
            uk::frame_begin();
            if ctx.scene_kind() != Some(SceneKindV1::InGame) {
                // 세이브 밖: 다른 세이브에 이전 설정이 새지 않게 캐시 해제
                *SAVE_EXCL.lock().unwrap_or_else(|e| e.into_inner()) = None;
                *CANDS.lock().unwrap_or_else(|e| e.into_inner()) = None;
                CAND_AT.store(0, Ordering::Relaxed);
                ui::hidden();
                ui::reset_registrations();
                return;
            }
            save_tick(ctx, f);
            let csk = ctx.client_scene_kind();
            if csk == Some(ClientSceneKindV1::Main) {
                recompute_candidates(ctx, f);
                if let Some(contents) = ui::option_contents(ctx, f) { ui::tick(ctx, &contents); } else { ui::hidden(); }
            } else {
                ui::hidden();
                // ★진단: 메인이 아닌 클라 씬에서 옵션 contents 가 보이면(=행 미표시 원인 후보) 씬 종류를 1회 기록
                if f % 120 == 0 && ctx.ui_exists("pause_ui.option.option.contents.current_database_edit") {
                    let code = csk.map(|k| k as u32 as u64).unwrap_or(u64::MAX);
                    if NONMAIN_LOGGED.swap(code, Ordering::Relaxed) != code { log(&format!("option contents visible but client_scene_kind={:?} (not Main) - rows skipped", csk)); }
                }
            }
        }));
        if let Err(e) = r {
            let msg = e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_else(|| "?".into());
            let n = PANIC_CNT.fetch_add(1, Ordering::Relaxed);
            if n < 5 { log(&format!("post_update panic #{}: {}", n + 1, msg)); }
        }
    }
}
static PANIC_CNT: AtomicUsize = AtomicUsize::new(0);
static NONMAIN_LOGGED: AtomicU64 = AtomicU64::new(0);

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "tfm2_champion_exclude v0.6.0 (stable 0.6.0)");
    let v = host.game_version();
    log(&format!("mod init v{} game {}.{}.{} host_abi={} - settings = per-save (mod save data) only", VERSION, v.major, v.minor, v.patch, host.abi_level()));
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { hook::install_hook() })) {
        Ok(Ok(m)) => log(&format!("HOOK OK: {}", m)),
        Ok(Err(e)) => log(&format!("HOOK FAIL: {}", e)),
        Err(_) => log("HOOK FAIL: panic in install_hook"),
    }
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d
}
declare_stable_mod!(init);
