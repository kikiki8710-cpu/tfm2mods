//! training_view_plus v0.6.0 — 개인 훈련 계획 "챔피언 훈련 일괄" (daram2 0.5.2 클래식의 0.6.0 stable 껍데기 재작성, 2026-09-18)
//! ===========================================================================
//! 기능: 훈련 화면에 버튼 → 패널(탑/정글/미드/바텀/서포터 × 역할군 풀다운 · 매일 자동 재적용) → 적용 시 해당 포지션 선수 전원의
//!   훈련 챔피언을 그 역할군 상위 티어 4개(각 25%)로 설정. '팀 훈련 따름' = 그 포지션 선수의 개인 설정 삭제. '변경 안 함' = 그대로.
//! 구조: 클라 = stable UI(ui_spawn_source · 공용 dropdown_stable) + `send_command("tvp_apply")` / 서버 = `handle_command` 에서 티어(team champion_tiers)·
//!   선수(포지션·계약)·챔피언 역할군(클라가 payload 로 전달)으로 목록을 만들고 `ui_kit\training_plan_stable.rs`(0.6.0 RE 경로 B: Database 훈련계획 직접 쓰기 + ResponseTrainingPlan 유니캐스트).
//! 근거 RE = `mods_report/daram2_view_plus/RE/2026-09-18_0.6.0-훈련계획-TeamTrainingPlan-저장처-응답경로.md`.
//! 파일: <게임>\mods\training_view_plus\training_view_plus.cfg(pos0..4=역할군 idx · auto=0/1) · tvp_log.txt(DBG).
//! ===========================================================================
use mod_api_stable::{declare_stable_mod, ClientSceneKindV1, CommandResultV1, LogLevel, RecordKindV1, StableClient, StableCommand, StableExtension, StableHost, StableMod, StableServerCtx, StableServerExtension};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

#[path = r"C:\tfm2mods\ui_kit\dropdown_stable.rs"]
mod dd;
#[path = r"C:\tfm2mods\ui_kit\training_plan_stable.rs"]
mod training_plan;

const MOD_ID: &str = "training_view_plus";
const DBG: bool = false; // 진단 시 true(tvp_log.txt)
const CMD: &str = "tvp_apply";
const EVT: &str = "tvp_result";
const VIEW: &str = "main.top.right"; // ★09-19: training 뷰 안(루트·panel)에 스폰하면 클릭이 우리 핸들러에 안 오고 뷰가 재생성됨(실측) → 뷰 형제 레벨(main.top.right)에 스폰하고 training 이 보일 때만 표시
const VIEW_ROOT: &str = "main.top.right.training";
const BTN_PARENT: &str = "main.top.right.training.tabs"; // 실험: 게임 버튼 컨테이너 안이면 클릭이 오는가
const BTN: &str = "tvp_open";
const PANEL: &str = "tvp_panel";
const I18N: &str = "#asset/base/text/ui?training_view_plus.";
const POS_KEYS: [&str; 5] = ["line_top", "line_jungle", "line_mid", "line_bottom", "line_support"];
const ROLE_KEYS: [&str; 7] = ["role_keep", "role_team", "role_melee", "role_range", "role_magician", "role_util", "role_assassin"];
const ROLE_IDS: [&str; 7] = ["r_keep", "r_team", "r_melee", "r_range", "r_magician", "r_util", "r_assassin"];
const PANEL_X: i32 = 400; const PANEL_Y: i32 = 110; const PANEL_W: i32 = 640; const PANEL_H: i32 = 570;
const ROW_Y0: i32 = 150; const ROW_H: i32 = 46; const DD_X: i32 = 180; const DD_W: i32 = 220; const DD_H: i32 = 36;

#[link(name = "kernel32")]
extern "system" { fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32; }
fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    Some(format!("{}\\mods\\{}", std::path::Path::new(&exe).parent()?.display(), MOD_ID))
}
fn log(s: &str) {
    if !DBG { return; }
    if let Some(d) = mod_dir() { use std::io::Write; let _ = std::fs::create_dir_all(&d); let _ = std::fs::OpenOptions::new().append(true).create(true).open(format!("{d}\\tvp_log.txt")).and_then(|mut f| writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), s)); }
}

// ── 설정(영속) ──
static ROLES: Mutex<[usize; 5]> = Mutex::new([0; 5]);
static AUTO: AtomicBool = AtomicBool::new(false);
static CFG_LOADED: AtomicBool = AtomicBool::new(false);
fn cfg_load() {
    if CFG_LOADED.swap(true, Ordering::Relaxed) { return; }
    let Some(d) = mod_dir() else { return };
    let Ok(t) = std::fs::read_to_string(format!("{d}\\{MOD_ID}.cfg")) else { return };
    let mut r = ROLES.lock().unwrap_or_else(|e| e.into_inner());
    for line in t.lines() {
        let Some((k, v)) = line.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        if let Some(i) = k.strip_prefix("pos").and_then(|x| x.parse::<usize>().ok()) { if i < 5 { r[i] = v.parse::<usize>().unwrap_or(0).min(6); } }
        if k == "auto" { AUTO.store(v == "1", Ordering::Relaxed); }
    }
}
fn cfg_save() {
    let Some(d) = mod_dir() else { return };
    let r = *ROLES.lock().unwrap_or_else(|e| e.into_inner());
    let mut s = String::from("# training_view_plus — 포지션별 역할군(0=변경 안 함 1=팀 훈련 따름 2=전사 3=원거리 4=마법사 5=전투보조 6=암살자) · auto=매일 자동 재적용\n");
    for i in 0..5 { s.push_str(&format!("pos{}={}\n", i, r[i])); }
    s.push_str(&format!("auto={}\n", if AUTO.load(Ordering::Relaxed) { 1 } else { 0 }));
    let _ = std::fs::create_dir_all(&d);
    let _ = std::fs::write(format!("{d}\\{MOD_ID}.cfg"), s);
}

// ── 클라 상태 ──
static FRAME: AtomicU64 = AtomicU64::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static PANEL_OPEN: AtomicBool = AtomicBool::new(false);
static DD_OPEN: [AtomicBool; 5] = [AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false)];
static CLICKS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LAST_CLICK: Mutex<Option<(u64, String)>> = Mutex::new(None);
static APPLY_REQ: AtomicBool = AtomicBool::new(false);
static LAST_DAY: AtomicI64 = AtomicI64::new(i64::MIN);
static STATUS: Mutex<Option<String>> = Mutex::new(None);
static SPAWN_FAIL: AtomicUsize = AtomicUsize::new(0);
static HAD_PANEL: AtomicBool = AtomicBool::new(false);

fn push_click(s: &str) { CLICKS.lock().unwrap_or_else(|e| e.into_inner()).push(s.to_string()); }
fn t(ctx: &StableClient<'_>, k: &str) -> String { ctx.i18n(&format!("{}{}", I18N, k)).unwrap_or_else(|| k.to_string()) }

fn panel_source() -> String {
    let mut s = format!(
"#{PANEL}:color {{ visible: false; x: {PANEL_X}px; y: {PANEL_Y}px; width: {PANEL_W}px; height: {PANEL_H}px; color: #4a4c56ff; stroke: 1; back_color: #161721ff; rounding: Uniform {{ rounding: 12; }}
  #title:label {{ @\"asset/base/style/main#bold_label\"; x: 24px; y: 18px; width: 580px; height: 36px; size: 22; align_y: Center; text: \"{I18N}title\"; }}
  #desc:label {{ @\"asset/base/style/main#label\"; x: 24px; y: 58px; width: 592px; height: 80px; size: 14; line_height: 20; color: #a3a9b6ff; align_y: Center; text: \"{I18N}desc\"; }}
  #close_x:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; anchor_x: 1; pivot_x: 1; x: -20px; y: 18px; width: 36px; height: 36px; icon: {{ source: \"asset/base/ui/icons/cross\"; rect: {{ x: 10; y: 10; w: 16; h: 16; }} }} }}
");
    for i in 0..5 {
        let y = ROW_Y0 + i as i32 * ROW_H;
        s.push_str(&format!("  #pos{i}:label {{ @\"asset/base/style/main#bold_label\"; x: 32px; y: {y}px; width: 130px; height: {DD_H}px; size: 17; align_y: Center; text: \"{I18N}{}\"; }}\n", POS_KEYS[i]));
        s.push_str(&dd::button_source(&format!("dd{i}"), DD_X, y, DD_W, DD_H, &format!("{I18N}role_keep"), 15).replace("\n#", "\n  #"));
    }
    let ya = ROW_Y0 + 5 * ROW_H + 8;
    s.push_str(&format!("  #auto_l:label {{ @\"asset/base/style/main#bold_label\"; x: 32px; y: {ya}px; width: 300px; height: 34px; size: 16; align_y: Center; text: \"{I18N}auto\"; }}\n"));
    s.push_str(&format!("  #auto_b:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: {DD_X}px; y: {ya}px; width: 100px; height: 34px; text: {{ text: \"{I18N}off\"; font: \"asset/base/font/set/bold\"; size: 15; align_x: Center; align_y: Center; }} }}\n"));
    s.push_str(&format!("  #note:label {{ @\"asset/base/style/main#label\"; x: 24px; y: {}px; width: 592px; height: 44px; size: 13; line_height: 18; color: #858d9dff; align_y: Center; text: \"{I18N}note\"; }}\n", ya + 40));
    s.push_str(&format!("  #status:label {{ @\"asset/base/style/main#label\"; x: 24px; y: {}px; width: 592px; height: 24px; size: 14; color: #37d5b3ff; align_y: Center; text: \"\"; }}\n", PANEL_H - 78));
    s.push_str(&format!("  #apply:color_icon_button {{ @\"asset/base/style/main#primary_button\"; x: {}px; y: {}px; width: 180px; height: 40px; text: {{ text: \"{I18N}apply\"; font: \"asset/base/font/set/bold\"; size: 17; align_x: Center; align_y: Center; }} }}\n", PANEL_W - 24 - 180 - 12 - 140, PANEL_H - 52));
    s.push_str(&format!("  #close:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: {}px; y: {}px; width: 140px; height: 40px; text: {{ text: \"{I18N}close\"; font: \"asset/base/font/set/bold\"; size: 17; align_x: Center; align_y: Center; }} }}\n", PANEL_W - 24 - 140, PANEL_H - 52));
    s.push_str("}\n");
    s
}
fn list_source(i: usize) -> String {
    let items: Vec<(String, String)> = ROLE_IDS.iter().zip(ROLE_KEYS.iter()).map(|(id, k)| (id.to_string(), format!("{I18N}{k}"))).collect();
    let refs: Vec<(&str, &str)> = items.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
    dd::list_source(&format!("dl{i}"), PANEL_X + DD_X, PANEL_Y + ROW_Y0 + i as i32 * ROW_H + DD_H + dd::LIST_GAP as i32, DD_W, 34, 15, &refs)
}
fn btn_source() -> String {
    format!("#{BTN}:color_icon_button {{ @\"asset/base/style/main#secondary_button\"; x: 240px; width: 180px; height: 31px; text: {{ font: \"asset/base/font/set/bold\"; text: \"{I18N}open_btn\"; size: 14; align_x: Center; align_y: Center; }} }}")
}

fn register(ctx: &mut StableClient<'_>) {
    let p = format!("{VIEW}.{PANEL}");
    let okb = ctx.ui_register_click(&format!("{BTN_PARENT}.{BTN}"), "", |_| push_click("open"));
    log(&format!("register open btn → {}", okb));
    for k in ["close_x", "close", "apply", "auto_b"] { let kk: &'static str = k; ctx.ui_register_click(&format!("{p}.{k}"), "", move |_| push_click(kk)); }
    for i in 0..5 {
        let tag = format!("dd{i}"); ctx.ui_register_click(&format!("{p}.dd{i}"), "", move |_| push_click(&tag));
        for (j, rid) in ROLE_IDS.iter().enumerate() { let tag = format!("sel{i}_{j}"); ctx.ui_register_click(&format!("{VIEW}.dl{i}.{rid}"), "", move |_| push_click(&tag)); }
    }
}

fn set_status(s: String) { *STATUS.lock().unwrap_or_else(|e| e.into_inner()) = Some(s); }

/// 적용 payload: roles + 챔피언 역할군 표(서버엔 champion_brief 가 없음).
fn build_payload(ctx: &StableClient<'_>) -> String {
    let r = *ROLES.lock().unwrap_or_else(|e| e.into_inner());
    let mut p = format!("roles\t{},{},{},{},{}\n", r[0], r[1], r[2], r[3], r[4]);
    for id in ctx.champion_names() {
        let cat = ctx.champion_brief(&id).and_then(|b| b.category).map(|c| c as i64).unwrap_or(-1);
        p.push_str(&format!("c\t{}\t{}\n", id, cat));
    }
    p
}

fn tick(ctx: &mut StableClient<'_>) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);
    cfg_load();
    for ev in ctx.take_events() { if ev.event == EVT { let m = String::from_utf8_lossy(&ev.payload).to_string(); log(&format!("서버 응답: {}", m)); set_status(m); } }
    if ctx.client_scene_kind() != Some(ClientSceneKindV1::Main) { if ACTIVE.swap(false, Ordering::Relaxed) { PANEL_OPEN.store(false, Ordering::Relaxed); } return; }
    // 매일 자동
    if let (Some((y, m, d)), true) = (ctx.game_date(), ctx.ui_exists("main.top.right")) {
        let today = y as i64 * 10000 + m as i64 * 100 + d as i64;
        let prev = LAST_DAY.swap(today, Ordering::Relaxed);
        if prev != i64::MIN && prev != today && AUTO.load(Ordering::Relaxed) { APPLY_REQ.store(true, Ordering::Relaxed); log("날짜 변경 → 자동 재적용"); }
    }
    if APPLY_REQ.swap(false, Ordering::Relaxed) {
        let p = build_payload(ctx);
        let ok = ctx.send_command(CMD, p.as_bytes());
        log(&format!("send_command {} ({}B) → {}", CMD, p.len(), ok));
        set_status(t(ctx, "applying"));
    }
    if f % 3 != 0 { return; }
    let vis = ctx.ui_exists(VIEW) && ctx.ui_visible(VIEW_ROOT).unwrap_or(false);
    let bpath = format!("{BTN_PARENT}.{BTN}");
    if !vis { if ACTIVE.swap(false, Ordering::Relaxed) { PANEL_OPEN.store(false, Ordering::Relaxed); for d in &DD_OPEN { d.store(false, Ordering::Relaxed); } }
        if ctx.ui_exists(&bpath) && ctx.ui_visible(&bpath) != Some(false) { ctx.ui_set_visible(&bpath, false); }
        let pp = format!("{VIEW}.{PANEL}"); if ctx.ui_exists(&pp) && ctx.ui_visible(&pp) != Some(false) { ctx.ui_set_visible(&pp, false); }
        for i in 0..5 { let l = format!("{VIEW}.dl{i}"); if ctx.ui_exists(&l) && ctx.ui_visible(&l) != Some(false) { ctx.ui_set_visible(&l, false); } }
        return; }
    if ctx.ui_exists(&bpath) && ctx.ui_visible(&bpath) != Some(true) { ctx.ui_set_visible(&bpath, true); }
    let p = format!("{VIEW}.{PANEL}");
    { let now = ctx.ui_exists(&p); let was = HAD_PANEL.swap(now, Ordering::Relaxed); if was && !now { log(&format!("★노드 소실 f{} kids={:?} edge={}", f, ctx.ui_child_names(VIEW), dd::click_edge())); } }
    if !ctx.ui_exists(&p) || !ctx.ui_exists(&format!("{BTN_PARENT}.{BTN}")) {
        log(&format!("재스폰 진입: panel={} btn={} lists={:?} kids={:?}", ctx.ui_exists(&p), ctx.ui_exists(&format!("{BTN_PARENT}.{BTN}")), (0..5).map(|i| ctx.ui_exists(&format!("{VIEW}.dl{i}"))).collect::<Vec<_>>(), ctx.ui_child_names(VIEW)));
        if SPAWN_FAIL.load(Ordering::Relaxed) > 5 { return; } // ★스폰 소스의 최상위 노드는 `#` 없이(champ_excl_popup 관례) — 09-19 실측: `#id` 로 주면 spawn false
        let ok1 = ctx.ui_exists(&format!("{BTN_PARENT}.{BTN}")) || ctx.ui_spawn_source(BTN_PARENT, btn_source().trim_start_matches('#'));
        let ok2 = ctx.ui_exists(&p) || ctx.ui_spawn_source(VIEW, panel_source().trim_start_matches('#'));
        let mut ok3 = true;
        for i in 0..5 { if !ctx.ui_exists(&format!("{VIEW}.dl{i}")) { ok3 &= ctx.ui_spawn_source(VIEW, list_source(i).trim_start_matches('#')); } }
        log(&format!("스폰 btn={} panel={} lists={} kids={:?}", ok1, ok2, ok3, ctx.ui_child_names(VIEW)));
        if !(ok1 && ok2 && ok3) { SPAWN_FAIL.fetch_add(1, Ordering::Relaxed); return; }
        register(ctx); dd::reset_cache();
        ACTIVE.store(true, Ordering::Relaxed);
    } else if !ACTIVE.swap(true, Ordering::Relaxed) { register(ctx); dd::reset_cache(); }
    // 클릭 처리
    let clicks: Vec<String> = std::mem::take(&mut *CLICKS.lock().unwrap_or_else(|e| e.into_inner()));
    for c in clicks {
        { let mut lc = LAST_CLICK.lock().unwrap_or_else(|e| e.into_inner()); if lc.as_ref().map(|(ff, s)| *ff == f && *s == c).unwrap_or(false) { continue; } *lc = Some((f, c.clone())); }
        match c.as_str() {
            "open" => { PANEL_OPEN.fetch_xor(true, Ordering::Relaxed); set_status(t(ctx, "status_idle")); }
            "close" | "close_x" => { PANEL_OPEN.store(false, Ordering::Relaxed); for d in &DD_OPEN { d.store(false, Ordering::Relaxed); } }
            "apply" => { cfg_save(); APPLY_REQ.store(true, Ordering::Relaxed); }
            "auto_b" => { AUTO.fetch_xor(true, Ordering::Relaxed); cfg_save(); }
            s if s.starts_with("dd") => { if let Ok(i) = s[2..].parse::<usize>() { if i < 5 { let was = DD_OPEN[i].load(Ordering::Relaxed); for d in &DD_OPEN { d.store(false, Ordering::Relaxed); } DD_OPEN[i].store(!was, Ordering::Relaxed); } } }
            s if s.starts_with("sel") => { if let Some((a, b)) = s[3..].split_once('_') { if let (Ok(i), Ok(j)) = (a.parse::<usize>(), b.parse::<usize>()) { if i < 5 && j < 7 { ROLES.lock().unwrap_or_else(|e| e.into_inner())[i] = j; DD_OPEN[i].store(false, Ordering::Relaxed); cfg_save(); } } } }
            _ => {}
        }
        log(&format!("클릭 {}", c));
    }
    let open = PANEL_OPEN.load(Ordering::Relaxed);
    if ctx.ui_visible(&p) != Some(open) { ctx.ui_set_visible(&p, open); }
    if !open { for i in 0..5 { let l = format!("{VIEW}.dl{i}"); if ctx.ui_visible(&l) == Some(true) { ctx.ui_set_visible(&l, false); } } return; }
    let edge = dd::click_edge();
    let roles = *ROLES.lock().unwrap_or_else(|e| e.into_inner());
    for i in 0..5 { dd::tick(ctx, &format!("{p}.dd{i}"), &format!("{VIEW}.dl{i}"), &ROLE_IDS, &DD_OPEN[i], roles[i], &format!("{I18N}{}", ROLE_KEYS[roles[i]]), edge); }
    let auto_txt = t(ctx, if AUTO.load(Ordering::Relaxed) { "on" } else { "off" });
    if ctx.ui_text(&format!("{p}.auto_b")).as_deref() != Some(&auto_txt) { ctx.ui_set_text(&format!("{p}.auto_b"), &auto_txt); }
    if let Some(s) = STATUS.lock().unwrap_or_else(|e| e.into_inner()).clone() { if ctx.ui_text(&format!("{p}.status")).as_deref() != Some(&s) { ctx.ui_set_text(&format!("{p}.status"), &s); } }
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) { let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tick(ctx))); }
}

// ───────── 서버 ─────────
fn find_team_id(v: &Value) -> Option<i64> {
    match v { Value::Object(m) => { if let Some(t) = m.get("team_id").and_then(|t| t.as_i64()) { return Some(t); } m.values().find_map(find_team_id) } Value::Array(a) => a.iter().find_map(find_team_id), _ => None }
}
fn pos_of_value(v: &Value) -> Option<usize> {
    match v {
        Value::String(s) => match s.to_ascii_lowercase().as_str() { "top" => Some(0), "jungle" | "jg" => Some(1), "mid" | "middle" => Some(2), "bottom" | "bot" | "adc" => Some(3), "support" | "sup" => Some(4), _ => None },
        Value::Number(n) => n.as_u64().filter(|x| *x < 5).map(|x| x as usize),
        Value::Object(m) => m.keys().next().and_then(|k| pos_of_value(&Value::String(k.clone()))),
        _ => None,
    }
}
/// 선수 JSON 에서 (주)포지션 — main_position 우선, 그다음 position (깊이 무관).
fn find_position(v: &Value) -> Option<usize> {
    fn walk(v: &Value, key: &str) -> Option<usize> {
        match v { Value::Object(m) => { if let Some(x) = m.get(key).and_then(pos_of_value) { return Some(x); } m.values().find_map(|c| walk(c, key)) } Value::Array(a) => a.iter().find_map(|c| walk(c, key)), _ => None }
    }
    walk(v, "main_position").or_else(|| walk(v, "position"))
}
fn tier_rank(t: &str) -> usize { match t.trim().to_ascii_uppercase().as_str() { "S" => 0, "A" => 1, "B" => 2, "C" => 3, "D" => 4, _ => 5 } }

fn apply_server(ctx: &mut StableServerCtx<'_>, cmd: &StableCommand<'_>) -> String {
    let team = cmd.sender_team_id.or_else(|| cmd.sender_player_id.and_then(|p| ctx.player_team_id(p)));
    let Some(team) = team else { return "오류: 팀 id 없음".into() };
    let p = String::from_utf8_lossy(cmd.payload).to_string();
    let mut roles = [0usize; 5];
    let mut cats: HashMap<String, i64> = HashMap::new();
    for line in p.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() >= 2 && f[0] == "roles" { for (i, x) in f[1].split(',').enumerate() { if i < 5 { roles[i] = x.trim().parse().unwrap_or(0); } } }
        if f.len() >= 3 && f[0] == "c" { cats.insert(f[1].to_string(), f[2].parse().unwrap_or(-1)); }
    }
    // 티어
    let tiers: HashMap<String, usize> = ctx.team_get_json(team, "champion_tiers").and_then(|s| serde_json::from_str::<Value>(&s).ok()).and_then(|v| v.as_object().cloned())
        .map(|m| m.iter().filter_map(|(k, v)| v.as_str().map(|t| (k.clone(), tier_rank(t)))).collect()).unwrap_or_default();
    // 역할군별 상위 4
    let mut per_role: [Option<String>; 7] = Default::default();
    for r in 2..7usize {
        let cat = (r - 2) as i64;
        let mut cand: Vec<(usize, String)> = cats.iter().filter(|(_, c)| **c == cat).map(|(n, _)| (tiers.get(n).copied().unwrap_or(5), n.clone())).collect();
        cand.sort();
        let top: Vec<String> = cand.into_iter().take(4).map(|(_, n)| n).collect();
        if top.is_empty() { continue; }
        let share = 100 / top.len();
        per_role[r] = Some(top.iter().map(|n| format!("{}:{}", n, share)).collect::<Vec<_>>().join("|"));
    }
    // 포지션 = Team.last_starting[포지션 idx] = athlete id (선발 5명) · 벤치 = season_statistics[*].by_position 최다 출전 포지션 · 둘 다 없으면 제외
    let starting: Vec<i64> = ctx.team_get_json(team, "last_starting").and_then(|s| serde_json::from_str::<Value>(&s).ok()).and_then(|v| v.as_array().map(|a| a.iter().filter_map(|x| x.as_i64()).collect())).unwrap_or_default();
    fn pos_by_stats(root: &Value) -> Option<usize> {
        let mut cnt = [0i64; 5];
        fn walk(v: &Value, cnt: &mut [i64; 5]) { match v { Value::Object(m) => { if let Some(bp) = m.get("by_position").and_then(|x| x.as_object()) { for (k, x) in bp { if let Some(p) = pos_of_value(&Value::String(k.clone())) { let n = x.get("matches").and_then(|n| n.as_i64()).unwrap_or(0); cnt[p] += n.max(1); } } } for x in m.values() { walk(x, cnt); } } Value::Array(a) => for x in a { walk(x, cnt); }, _ => {} } }
        walk(root, &mut cnt);
        let (i, &m) = cnt.iter().enumerate().max_by_key(|(_, c)| **c)?;
        if m > 0 { Some(i) } else { None }
    }
    // 선수
    let mut set: Vec<(u64, String)> = Vec::new();
    let mut clear: Vec<u64> = Vec::new();
    let mut nopos = 0;
    for id in ctx.record_ids(RecordKindV1::Athlete) {
        let Some(c) = ctx.athlete_get_json(id, "contract").and_then(|s| serde_json::from_str::<Value>(&s).ok()) else { continue };
        if find_team_id(&c) != Some(team as i64) { continue; }
        let root = ctx.athlete_get_json(id, "").and_then(|s| serde_json::from_str::<Value>(&s).ok());
        let pos = starting.iter().position(|a| *a == id as i64).filter(|p| *p < 5).or_else(|| root.as_ref().and_then(find_position)).or_else(|| root.as_ref().and_then(pos_by_stats));
        let Some(pos) = pos else { nopos += 1; continue };
        match roles[pos] {
            0 => {}
            1 => clear.push(id as u64),
            r => { if let Some(v) = &per_role[r] { set.push((id as u64, v.clone())); } }
        }
    }
    log(&format!("[server] team={} roles={:?} set={} clear={} nopos={} tiers={} cats={}", team, roles, set.len(), clear.len(), nopos, tiers.len(), cats.len()));
    if set.is_empty() && clear.is_empty() { return format!("변경 없음(대상 선수 0 · 포지션 미확인 {})", nopos); }
    match training_plan::apply(ctx, team, &set, &clear) {
        Ok(m) => { log(&format!("[server] {}", m)); format!("적용 완료: 설정 {}명 · 팀 훈련 따름 {}명{}", set.len(), clear.len(), if nopos > 0 { format!(" (포지션 미확인 {}명 제외)", nopos) } else { String::new() }) }
        Err(e) => { log(&format!("[server] 실패: {}", e)); format!("오류: {}", e) }
    }
}
struct Srv;
impl StableServerExtension for Srv {
    fn handle_command(&self, ctx: &mut StableServerCtx<'_>, cmd: &StableCommand<'_>) -> CommandResultV1 {
        if cmd.command != CMD { return CommandResultV1::Pass; }
        let reply = cmd.reply_target();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| apply_server(ctx, cmd)));
        let msg = match r { Ok(m) => m, Err(_) => "오류: 서버 처리 중 패닉".to_string() };
        ctx.emit_event(reply, EVT, msg.as_bytes());
        CommandResultV1::Handled
    }
}

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "training_view_plus v0.6.0 (stable)");
    log("init v0.6.0");
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d.set_server_extension(Srv);
    d
}
declare_stable_mod!(init);
