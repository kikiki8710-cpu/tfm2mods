//! custom_tier_assignment — 챔피언 정보 화면 '나만의 티어 분류' (daram2 원작 · 0.6.0 stable 포팅 2026-09-17).
//! 원작(클래식): .ui 오버라이드(버튼 3 + 설정 패널), db 직접 읽기(champion_info·pre_patch_data·champion_patch_statistics), post_render 이름 색칠.
//! stable 판:
//!   · 버튼 3개 = `champion_info.data.action_buttons` 아래 스폰(x 790/1000/1104 — 게임 위임·초기화 버튼과 value_mode 사이 빈 공간)
//!   · 설정 패널 = `champion_info` 아래 `cta_panel:color`(980×724, 중앙) — 값 라벨은 `ui_set_text` 로 갱신(원작은 21개 라벨 스위치)
//!   · 스탯 = 클라 `champion_brief(name)`(stat/growth/category, 현재 패치값) + 공속/사거리 = 번들 `asset/base/setting/champion_info` 시트(기본값; 모드 챔프는 0)
//!   · 경기 성적 = **서버**에서 LeagueCompetition/TournamentCompetition 레코드 `statistics[athlete].champion_detail{matches,wins}` 집계
//!     (클라는 Match 류 레코드가 0개 — 서버 전용). 밴률 = 대회 matches → Match.replays → MatchReplay.blue_ban/red_ban 집계(세트 수 분모).
//!   · 티어 쓰기 = 서버 `team_set_json(team,"champion_tiers",…)` + `team_sync::unicast_team`(즉시 반영)
//!   · 버프/너프 색칠 = 현재 brief vs 번들 시트 기본값(6 스탯) → 카드 `name` 라벨 color
//! 흐름: 클라가 (config + 챔프별 [카테고리, stat_norm]) 을 `send_command("cta_apply")` → 서버가 성적 혼합·컷오프·쓰기·유니캐스트 → `cta_result` 이벤트.
//! 설정 = mods\custom_tier_assignment\custom_tier_assignment.cfg (key=value).
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, ClientSceneKindV1, CommandResultV1, LogLevel, RecordKindV1, StableClient, StableCommand, StableExtension, StableHost, StableMod, StableServerCtx, StableServerExtension};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::Mutex;
#[path = r"C:\tfm2mods\ui_kit\team_sync_stable.rs"]
mod team_sync;

const MOD_ID: &str = "custom_tier_assignment";
const DBG: bool = false; // 09-19 확정 배포(진단 시 true)
const VIEW: &str = "main.top.right.champion_info";
const ACTIONS: &str = "main.top.right.champion_info.data.action_buttons";
const CARDS: &str = "main.top.right.champion_info.data.champions.contents";
const CMD: &str = "cta_apply";
const EVT: &str = "cta_result";
const MAX_LEVEL: usize = 12;
const STEP: f32 = 0.05;
const COLOR_BUFF: &str = "#4dd166ff";
const COLOR_NERF: &str = "#ed5c57ff";
const COLOR_NORMAL: &str = "#e8e8e8ff"; // 카드 name 라벨 기본색(⬜실측 대조)
/// ★09-18: 통계→챔피언 통계 표(statistics_view_plus 0.6.0 잔여 기능 흡수 — 티어 지정은 게임 내장, 이름 버프/너프 색만 남음)
const STAT_VIEW: &str = "main.top.right.statistics.data.champion";
const STAT_ROWS: &str = "main.top.right.statistics.data.champion.data.contents";
static STAT_COLOR: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
static STAT_NAME_MAP: Mutex<Option<HashMap<String, String>>> = Mutex::new(None); // 표시명 → id
static STAT_TINT_AT: AtomicU64 = AtomicU64::new(u64::MAX);
fn stat_name_map(ctx: &StableClient<'_>) -> HashMap<String, String> {
    let mut g = STAT_NAME_MAP.lock().unwrap_or_else(|e| e.into_inner());
    if g.as_ref().map(|m| m.len()).unwrap_or(0) < ctx.champion_names().len() {
        let mut m = HashMap::new();
        for id in ctx.champion_names() { if let Some(n) = ctx.i18n(&format!("#asset/base/text/champion?description.{}.name", id)) { if !n.is_empty() { m.insert(n, id.clone()); } } }
        *g = Some(m);
    }
    g.clone().unwrap_or_default()
}
/// 통계 챔피언 표 이름 색칠(뷰가 보일 때만 · 6프레임마다 · 색 캐시).
fn stat_color_tick(ctx: &mut StableClient<'_>, f: u64) {
    if DBG && f % 600 == 0 { let rows = ctx.ui_child_names(STAT_ROWS); let k0 = rows.first().map(|r| ctx.ui_child_names(&format!("{}.{}", STAT_ROWS, r))).unwrap_or_default(); let k1 = rows.first().map(|r| ctx.ui_child_names(&format!("{}.{}.data", STAT_ROWS, r))).unwrap_or_default(); let t0 = rows.first().and_then(|r| ctx.ui_text(&format!("{}.{}.data.champion_name.text", STAT_ROWS, r))); log(&format!("statdiag: view_vis={:?} exists={} rows={} kids0={:?} data_kids={:?} text0={:?} stat_kids={:?}", ctx.ui_visible(STAT_VIEW), ctx.ui_exists(STAT_VIEW), rows.len(), k0, k1, t0, ctx.ui_child_names("main.top.right.statistics"))); }
    if ctx.ui_visible(STAT_VIEW) != Some(true) { if STAT_COLOR.lock().unwrap_or_else(|e| e.into_inner()).is_some() { *STAT_COLOR.lock().unwrap_or_else(|e| e.into_inner()) = None; } return; }
    let on = cfg().name_color >= 0.5;
    if on && (TINTS.lock().unwrap_or_else(|e| e.into_inner()).is_none() || f.saturating_sub(STAT_TINT_AT.load(Ordering::Relaxed)) > 600) { *TINTS.lock().unwrap_or_else(|e| e.into_inner()) = Some(compute_tints(ctx)); STAT_TINT_AT.store(f, Ordering::Relaxed); }
    let tints = TINTS.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_default();
    let nm = stat_name_map(ctx);
    let mut g = STAT_COLOR.lock().unwrap_or_else(|e| e.into_inner());
    let cache = g.get_or_insert_with(HashMap::new);
    for row in ctx.ui_child_names(STAT_ROWS) {
        let lp = format!("{}.{}.data.champion_name.text", STAT_ROWS, row);
        let Some(txt) = ctx.ui_text(&lp) else { continue };
        let id = nm.get(txt.trim()).cloned().unwrap_or_default();
        let want = if on { match tints.get(&id).copied().unwrap_or(0) { 1 => COLOR_BUFF, -1 => COLOR_NERF, _ => COLOR_NORMAL } } else { COLOR_NORMAL };
        let key = format!("{}|{}", row, txt);
        if cache.get(&key).map(|c| c == want).unwrap_or(false) { continue; }
        if want == COLOR_NORMAL && !cache.keys().any(|k| k.starts_with(&format!("{}|", row))) { cache.insert(key, want.to_string()); continue; } // 처음 보는 행이 기본색이면 set 생략
        ctx.ui_set_properties(&lp, &format!("color: {};", want)); cache.retain(|k, _| !k.starts_with(&format!("{}|", row))); cache.insert(key, want.to_string());
    }
}

// ───────── 설정 ─────────
#[derive(Clone, Copy)]
struct Config { cut_s: f32, cut_a: f32, cut_b: f32, cut_c: f32, w_damage: f32, w_hp: f32, w_defence: f32, w_mr: f32, w_move: f32, w_as: f32, w_range: f32, level: f32, cross: f32, live_weight: f32, winban: f32, bayes_k: f32, patch_mode: f32, auto_daily: f32, name_color: f32 }
impl Default for Config {
    fn default() -> Self { Config { cut_s: 0.15, cut_a: 0.35, cut_b: 0.65, cut_c: 0.85, w_damage: 0.35, w_hp: 0.35, w_defence: 0.10, w_mr: 0.10, w_move: 0.10, w_as: 0.0, w_range: 0.0, level: 6.5, cross: 0.0, live_weight: 0.35, winban: 0.65, bayes_k: 20.0, patch_mode: 0.0, auto_daily: 1.0, name_color: 1.0 } }
}
const KEYS: [&str; 19] = ["cut_s", "cut_a", "cut_b", "cut_c", "damage", "hp", "defence", "mr", "move", "as", "range", "level", "cross", "live_weight", "winban", "bayes_k", "patch_mode", "auto", "name_color"];
impl Config {
    fn get(&self, k: &str) -> f32 {
        match k { "cut_s" => self.cut_s, "cut_a" => self.cut_a, "cut_b" => self.cut_b, "cut_c" => self.cut_c, "damage" => self.w_damage, "hp" => self.w_hp, "defence" => self.w_defence, "mr" => self.w_mr, "move" => self.w_move, "as" => self.w_as, "range" => self.w_range, "level" => self.level, "cross" => self.cross, "live_weight" => self.live_weight, "winban" => self.winban, "bayes_k" => self.bayes_k, "patch_mode" => self.patch_mode, "auto" => self.auto_daily, "name_color" => self.name_color, _ => 0.0 }
    }
    fn set(&mut self, k: &str, v: f32) {
        let r = match k { "cut_s" => &mut self.cut_s, "cut_a" => &mut self.cut_a, "cut_b" => &mut self.cut_b, "cut_c" => &mut self.cut_c, "damage" => &mut self.w_damage, "hp" => &mut self.w_hp, "defence" => &mut self.w_defence, "mr" => &mut self.w_mr, "move" => &mut self.w_move, "as" => &mut self.w_as, "range" => &mut self.w_range, "level" => &mut self.level, "cross" => &mut self.cross, "live_weight" => &mut self.live_weight, "winban" => &mut self.winban, "bayes_k" => &mut self.bayes_k, "patch_mode" => &mut self.patch_mode, "auto" => &mut self.auto_daily, "name_color" => &mut self.name_color, _ => return };
        *r = v;
    }
    /// 원작 adjust: 컷오프는 0..1 & S<A<B<C 유지, 가중치 0..1, level 1..12(0.5 단위), 비중 0..1
    fn adjust(&mut self, k: &str, d: f32) {
        let v = self.get(k);
        let nv = match k {
            "level" => (v + d * 10.0).clamp(1.0, 12.0),
            "cut_s" => (v + d).clamp(0.0, self.cut_a),
            "cut_a" => (v + d).clamp(self.cut_s, self.cut_b),
            "cut_b" => (v + d).clamp(self.cut_a, self.cut_c),
            "cut_c" => (v + d).clamp(self.cut_b, 1.0),
            _ => (v + d).clamp(0.0, 1.0),
        };
        self.set(k, (nv * 1000.0).round() / 1000.0);
    }
    fn load() -> Config {
        let mut c = Config::default();
        if let Some(t) = cfg_path().and_then(|p| std::fs::read_to_string(p).ok()) {
            for l in t.lines() {
                let l = l.trim(); if l.is_empty() || l.starts_with('#') { continue; }
                if let Some((k, v)) = l.split_once('=') { if let Ok(f) = v.trim().parse::<f32>() { c.set(k.trim(), f); } }
            }
        }
        c
    }
    fn save(&self) {
        if let Some(p) = cfg_path() {
            let mut s = String::from("# custom_tier_assignment 0.6.0 stable\n");
            for k in KEYS { s.push_str(&format!("{}={}\n", k, self.get(k))); }
            let _ = std::fs::write(p, s);
        }
    }
    fn encode(&self) -> String { KEYS.iter().map(|k| format!("{}={}", k, self.get(k))).collect::<Vec<_>>().join(";") }
    fn decode(s: &str) -> Config { let mut c = Config::default(); for kv in s.split(';') { if let Some((k, v)) = kv.split_once('=') { if let Ok(f) = v.parse::<f32>() { c.set(k, f); } } } c }
}
static CFG: Mutex<Config> = Mutex::new(Config { cut_s: 0.15, cut_a: 0.35, cut_b: 0.65, cut_c: 0.85, w_damage: 0.35, w_hp: 0.35, w_defence: 0.10, w_mr: 0.10, w_move: 0.10, w_as: 0.0, w_range: 0.0, level: 6.5, cross: 0.0, live_weight: 0.35, winban: 0.65, bayes_k: 20.0, patch_mode: 0.0, auto_daily: 1.0, name_color: 1.0 });
fn cfg() -> Config { *CFG.lock().unwrap_or_else(|e| e.into_inner()) }
fn cfg_mut(f: impl FnOnce(&mut Config)) { let mut g = CFG.lock().unwrap_or_else(|e| e.into_inner()); f(&mut g); g.save(); }

// ───────── 공통 ─────────
static FRAME: AtomicU64 = AtomicU64::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static CLICKS_REGISTERED: AtomicBool = AtomicBool::new(false);
static PANEL_OPEN: AtomicBool = AtomicBool::new(false);
static PANEL_DIRTY: AtomicBool = AtomicBool::new(true);
static APPLY_REQ: AtomicBool = AtomicBool::new(false);
static LAST_DAY: AtomicI64 = AtomicI64::new(i64::MIN);
static TOAST_UNTIL: AtomicU64 = AtomicU64::new(0);
static TOAST_TEXT: Mutex<Option<String>> = Mutex::new(None);
static SHEET: Mutex<Option<HashMap<String, SheetEntry>>> = Mutex::new(None);
static NAME_COLOR: Mutex<Option<HashMap<String, String>>> = Mutex::new(None); // 카드 → 적용된 색
static TINTS: Mutex<Option<HashMap<String, i8>>> = Mutex::new(None);
static KEY_LOGGED: AtomicBool = AtomicBool::new(false);

#[link(name = "kernel32")]
extern "system" { fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32; }
fn exe_dir() -> Option<String> {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    exe.rfind(|c| c == '\\' || c == '/').map(|i| exe[..i].to_string())
}
fn mod_dir() -> Option<String> { exe_dir().map(|d| format!(r"{}\mods\{}", d, MOD_ID)) }
fn cfg_path() -> Option<String> { mod_dir().map(|d| format!(r"{}\custom_tier_assignment.cfg", d)) }
fn log(s: &str) {
    if !DBG { return; }
    if let Some(d) = mod_dir() {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!(r"{}\debug.log", d)) {
            let _ = writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), s);
        }
    }
}
fn ko() -> bool { LANG_KO.load(Ordering::Relaxed) }
static LANG_KO: AtomicBool = AtomicBool::new(true);
fn t(key: &str) -> &'static str {
    let k = ko();
    match key {
        "classify" => if k { "나만의 티어 분류" } else { "Assign Custom Tiers" },
        "settings" => if k { "설정" } else { "Config" },
        "color_on" => if k { "색칠 켬" } else { "Tint on" },
        "color_off" => if k { "색칠 끔" } else { "Tint off" },
        "title" => if k { "나만의 티어 설정" } else { "Custom Tier Settings" },
        "sec_cutoff" => if k { "티어 컷오프 (상위 비율)" } else { "Tier Cutoffs (top share)" },
        "desc_cutoff" => if k { "점수순으로 줄 세워 위에서 어디까지 각 등급으로 끊을지. 작을수록 그 등급이 귀해짐. S, A, B, C 순서 유지" } else { "Ranked by score; top share that becomes each tier. Lower is rarer. Keep S, A, B, C in order" },
        "cut_s" => if k { "S 컷오프" } else { "S cutoff" }, "cut_a" => if k { "A 컷오프" } else { "A cutoff" }, "cut_b" => if k { "B 컷오프" } else { "B cutoff" }, "cut_c" => if k { "C 컷오프" } else { "C cutoff" },
        "sec_weights" => if k { "스탯 가중치" } else { "Stat Weights" },
        "desc_weights" => if k { "각 스탯이 점수에 반영되는 크기. 클수록 그 스탯 높은 챔피언이 상위 등급. 합 100 권장" } else { "How much each stat counts. Bigger means champions strong in it rank higher. Sum about 100" },
        "damage" => if k { "딜 가중치" } else { "Damage" }, "hp" => if k { "체력 가중치" } else { "HP" }, "defence" => if k { "방어 가중치" } else { "Defense" }, "mr" => if k { "마저 가중치" } else { "Magic resist" }, "move" => if k { "이속 가중치" } else { "Move speed" }, "as" => if k { "공속 가중치" } else { "Attack speed" }, "range" => if k { "사거리 가중치" } else { "Attack range" },
        "sec_level" => if k { "레벨 비중" } else { "Level Blend" },
        "desc_level" => if k { "몇 레벨 스탯으로 평가할지. 낮으면 초반(1), 높으면 만렙(12). 6.5 이 한가운데" } else { "Level stats are judged at. Low is early (1), high is max (12). 6.5 is midpoint" },
        "level" => if k { "기준 레벨" } else { "Reference level" },
        "sec_method" => if k { "분류 옵션" } else { "Classification" },
        "desc_method" => if k { "직업별: 같은 직업끼리 S~D. 전체: 다 함께 비교. 매일 자동: 날짜 바뀔 때 재계산" } else { "Per role: within role. Overall: all together. Auto daily: re-run each new day" },
        "cross" => if k { "티어 기준" } else { "Tier basis" }, "cross_cat" => if k { "직업별" } else { "Per role" }, "cross_all" => if k { "전체" } else { "Overall" },
        "auto" => if k { "매일 자동 분류" } else { "Auto daily" }, "on" => if k { "켬" } else { "On" }, "off" => if k { "끔" } else { "Off" },
        "sec_live" => if k { "경기 성적 반영" } else { "Live Performance" },
        "desc_live" => if k { "시즌 승률을 티어에 반영(성적 비중 0 이면 안 함). 성적 비중: 스탯 대 성적. 성적 범위: 현재 시즌만 / 전체" } else { "Blend season win rates (0 is off). Live weight: stats vs live. Scope: current season / all" },
        "live_weight" => if k { "성적 비중" } else { "Live weight" }, "winban" => if k { "승률 비중" } else { "Win-rate share" },
        "patch_mode" => if k { "성적 범위" } else { "Scope" }, "patch_cur" => if k { "현재 시즌" } else { "Current" }, "patch_all" => if k { "전체" } else { "All" },
        "reset" => if k { "기본값 복원" } else { "Restore Defaults" },
        "hint" => if k { "값 바꾼 뒤 나만의 티어 분류 버튼(또는 grave 키)을 다시 누르면 적용" } else { "Press Assign Custom Tiers (or the grave key) again to apply" },
        _ => "",
    }
}

// ───────── 번들 시트(공속·사거리·기본 스탯) ─────────
#[derive(Clone, Default)]
struct SheetEntry { stat: [f32; 6], growth: [f32; 6], cooltime: f32, range: f32 }
fn stat6(v: &Value) -> [f32; 6] {
    let g = |k: &str| v.get(k).and_then(|x| x.as_f64()).unwrap_or(0.0) as f32;
    [g("attack"), g("magic_power"), g("hp"), g("defence"), g("magic_resistance"), g("move_speed")]
}
fn load_sheet() -> HashMap<String, SheetEntry> {
    use std::io::{Read, Seek, SeekFrom};
    let mut out = HashMap::new();
    let Some(dir) = exe_dir() else { return out };
    let Ok(mut f) = std::fs::File::open(format!(r"{}\bundle.game_data", dir)) else { log("bundle 열기 실패"); return out };
    let mut u32b = [0u8; 4];
    if f.read_exact(&mut u32b).is_err() { return out; }
    let mut body: Option<Vec<u8>> = None;
    for _ in 0..100_000 {
        if f.read_exact(&mut u32b).is_err() { break; }
        let extlen = u32::from_le_bytes(u32b) as usize; if extlen == 0 || extlen > 64 { break; }
        let mut ext = vec![0u8; extlen]; if f.read_exact(&mut ext).is_err() { break; }
        if f.read_exact(&mut u32b).is_err() { break; }
        let pl = u32::from_le_bytes(u32b) as usize; if pl > 1024 { break; }
        let mut path = vec![0u8; pl]; if f.read_exact(&mut path).is_err() { break; }
        if f.read_exact(&mut u32b).is_err() { break; }
        let bl = u32::from_le_bytes(u32b) as u64;
        if path == b"asset/base/setting/champion_info" {
            let mut b = vec![0u8; bl as usize]; if f.read_exact(&mut b).is_ok() { body = Some(b); } break;
        }
        if f.seek(SeekFrom::Current(bl as i64)).is_err() { break; }
    }
    let Some(b) = body else { log("bundle 에 champion_info 시트 없음"); return out };
    let Ok(v) = serde_json::from_slice::<Value>(&b) else { log("시트 JSON 파싱 실패"); return out };
    let mut push = |name: &str, e: &Value| {
        let se = SheetEntry {
            stat: e.get("stat").map(stat6).unwrap_or_default(), growth: e.get("growth").map(stat6).unwrap_or_default(),
            cooltime: e.get("attack").and_then(|a| a.get("cooltime")).and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
            range: e.get("attack").and_then(|a| a.get("range")).and_then(|x| x.as_f64()).unwrap_or(0.0) as f32,
        };
        out.insert(name.to_string(), se);
    };
    if let Some(m) = v.as_object() {
        for (k, e) in m {
            if k == "mod_champions" { if let Some(arr) = e.as_array() { for e in arr { if let Some(id) = e.get("id").and_then(|x| x.as_str()) { push(id, e); } } } }
            else if e.is_object() { push(k, e); }
        }
    }
    log(&format!("번들 시트 로드: {}개", out.len()));
    out
}
fn sheet() -> HashMap<String, SheetEntry> {
    let mut g = SHEET.lock().unwrap_or_else(|e| e.into_inner());
    g.get_or_insert_with(load_sheet).clone()
}

// ───────── 점수(클라) ─────────
fn percentile_ranks(values: &[f32]) -> Vec<f32> {
    let n = values.len(); if n <= 1 { return vec![0.5; n]; }
    values.iter().map(|&v| { let (mut less, mut eq) = (0f32, 0f32); for &o in values { if o < v { less += 1.0; } else if o == v { eq += 1.0; } } (less + 0.5 * eq) / n as f32 }).collect()
}
struct Row { name: String, cat: usize, stats: [f32; 8] }
fn cat_index(c: Option<mod_api_stable::ChampionCategoryV1>) -> usize {
    use mod_api_stable::ChampionCategoryV1 as C;
    match c { Some(C::Melee) => 0, Some(C::Magician) => 1, Some(C::Assassin) => 2, Some(C::Util) => 3, Some(C::Range) => 4, None => 0 }
}
/// (name, cat, stat_norm) — 원작 compute_tiers 의 stat_norm 까지. 성적 혼합·컷오프는 서버.
fn stat_scores(ctx: &StableClient<'_>, c: &Config) -> Vec<(String, usize, f32)> {
    let sh = sheet();
    let lv = (MAX_LEVEL - 1) as f32;
    let tt = ((c.level - 1.0) / 11.0).clamp(0.0, 1.0);
    let blend = |b: f32, g: f32| (1.0 - tt) * b + tt * (b + g * lv);
    let mut rows: Vec<Row> = Vec::new();
    for name in ctx.champion_names() {
        let Some(b) = ctx.champion_brief(&name) else { continue };
        let (s, g) = (&b.stat, &b.growth);
        let e = sh.get(&name).cloned().unwrap_or_default();
        let as_val = if e.cooltime > 0.0 { 60.0 / e.cooltime } else { 0.0 };
        rows.push(Row { name: name.clone(), cat: cat_index(b.category), stats: [
            blend(s.attack as f32, g.attack as f32), blend(s.magic_power as f32, g.magic_power as f32), blend(s.hp as f32, g.hp as f32),
            blend(s.defence as f32, g.defence as f32), blend(s.magic_resistance as f32, g.magic_resistance as f32), blend(s.move_speed as f32, g.move_speed as f32), as_val, e.range] });
    }
    let n = rows.len(); if n == 0 { return Vec::new(); }
    let mut normed = vec![[0f32; 8]; n];
    for k in 0..8 { let col: Vec<f32> = rows.iter().map(|r| r.stats[k]).collect(); let pr = percentile_ranks(&col); for i in 0..n { normed[i][k] = pr[i]; } }
    let wsum = c.w_damage + c.w_hp + c.w_defence + c.w_mr + c.w_move + c.w_as + c.w_range;
    rows.iter().enumerate().map(|(i, r)| {
        let nm = &normed[i];
        let s = c.w_damage * nm[0].max(nm[1]) + c.w_hp * nm[2] + c.w_defence * nm[3] + c.w_mr * nm[4] + c.w_move * nm[5] + c.w_as * nm[6] + c.w_range * nm[7];
        (r.name.clone(), r.cat, if wsum > 0.0 { s / wsum } else { 0.0 })
    }).collect()
}
/// 버프/너프 판정: 현재 brief vs 번들 시트 기본값(6 스탯, 1레벨+만렙 합 상대변화 합산)
fn compute_tints(ctx: &StableClient<'_>) -> HashMap<String, i8> {
    let sh = sheet(); let mut out = HashMap::new(); let lv = (MAX_LEVEL - 1) as f32;
    for name in ctx.champion_names() {
        let (Some(b), Some(e)) = (ctx.champion_brief(&name), sh.get(&name)) else { continue };
        let cur = [b.stat.attack, b.stat.magic_power, b.stat.hp, b.stat.defence, b.stat.magic_resistance, b.stat.move_speed].map(|x| x as f32);
        let cg = [b.growth.attack, b.growth.magic_power, b.growth.hp, b.growth.defence, b.growth.magic_resistance, b.growth.move_speed].map(|x| x as f32);
        let mut net = 0f32;
        for k in 0..6 { let isum = e.stat[k] + (e.stat[k] + e.growth[k] * lv); if isum > 0.0 { net += (cur[k] + (cur[k] + cg[k] * lv) - isum) / isum; } }
        out.insert(name, if net > 0.01 { 1 } else if net < -0.01 { -1 } else { 0 });
    }
    out
}

// ───────── UI 소스 ─────────
fn btn(id: &str, x: i32, w: i32, text: &str) -> String {
    format!("{}:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: {}px; y: 0px; width: {}px; height: 40px; text: {{ text: \"{}\"; size: 16; align_x: Center; align_y: Center; }} }}", id, x, w, text)
}
/// 패널 행: name 라벨 + [-] + 값 + [+]  (x0 = 열 시작)
fn row_src(key: &str, x0: i32, y: i32) -> String {
    format!("  #n_{k}:label {{ @\"asset/base/style/main#label\"; x: {x}px; y: {y}px; width: 200px; height: 36px; size: 16; align_y: Center; color: #e8e8e8ff; text: \"{t}\"; }}\n  #dec_{k}:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: {x1}px; y: {y}px; width: 40px; height: 36px; text: {{ text: \"-\"; size: 20; align_x: Center; align_y: Center; }} }}\n  #v_{k}:label {{ @\"asset/base/style/main#label\"; x: {x2}px; y: {y}px; width: 96px; height: 36px; size: 17; align_x: Center; align_y: Center; color: #f2c14eff; text: \"\"; }}\n  #inc_{k}:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: {x3}px; y: {y}px; width: 40px; height: 36px; text: {{ text: \"+\"; size: 20; align_x: Center; align_y: Center; }} }}\n",
        k = key, x = x0, x1 = x0 + 206, x2 = x0 + 250, x3 = x0 + 350, y = y, t = t(key))
}
fn toggle_src(key: &str, x0: i32, y: i32, a: &str, b: &str) -> String {
    format!("  #n_{k}:label {{ @\"asset/base/style/main#label\"; x: {x}px; y: {y}px; width: 200px; height: 36px; size: 16; align_y: Center; color: #e8e8e8ff; text: \"{t}\"; }}\n  #off_{k}:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: {x1}px; y: {y}px; width: 90px; height: 36px; text: {{ text: \"{a}\"; size: 15; align_x: Center; align_y: Center; }} }}\n  #on_{k}:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: {x2}px; y: {y}px; width: 90px; height: 36px; text: {{ text: \"{b}\"; size: 15; align_x: Center; align_y: Center; }} }}\n  #v_{k}:label {{ @\"asset/base/style/main#label\"; x: {x3}px; y: {y}px; width: 60px; height: 36px; size: 15; align_x: Center; align_y: Center; color: #f2c14eff; text: \"\"; }}\n",
        k = key, x = x0, x1 = x0 + 206, x2 = x0 + 300, x3 = x0 + 394, y = y, t = t(key), a = a, b = b)
}
fn sec(id: &str, x: i32, y: i32, w: i32, title: &str, desc: &str) -> String {
    format!("  #s_{id}:label {{ @\"asset/base/style/main#bold_label\"; x: {x}px; y: {y}px; width: {w}px; height: 24px; size: 17; color: #f2c14eff; align_y: Center; text: \"{title}\"; }}\n  #d_{id}:label {{ @\"asset/base/style/main#label\"; x: {x}px; y: {y2}px; width: {w}px; height: 32px; size: 13; line_height: 17; color: #9a9ca6ff; text: \"{desc}\"; }}\n", id = id, x = x, y = y, y2 = y + 26, w = w, title = title, desc = desc)
}
fn panel_src() -> String {
    // ★루트를 color 로 두면 z 가 자식보다 앞이라 자식이 가려진다(0.6.0 실측: 빈 상자만 보임) → 루트 empty + #bg:color
    let mut s = String::from("cta_panel:empty { visible: false; z: 1200; anchor_x: 0.5; pivot_x: 0.5; anchor_y: 0.5; pivot_y: 0.5; width: 980px; height: 724px;\n  #bg:color { ignore_event: true; width: 100%; height: 100%; color: #161721ff; rounding: Uniform { rounding: 12; } }\n");
    s.push_str(&format!("  #border:color {{ ignore_event: true; x: 24px; y: 50px; width: 932px; height: 2px; color: #f2c14eff; }}\n  #title:label {{ @\"asset/base/style/main#bold_label\"; x: 24px; y: 18px; width: 600px; height: 32px; size: 20; color: #ffffffff; align_y: Center; text: \"{}\"; }}\n", t("title")));
    s.push_str("  #close:color_icon_button { @\"asset/base/style/main#tertiary_button\"; anchor_x: 1; pivot_x: 1; x: -20px; y: 12px; width: 36px; height: 32px; text: { text: \"X\"; size: 16; align_x: Center; align_y: Center; } }\n");
    // 열 1 (x 24): 컷오프 4 + 레벨 + 방법(cross/auto)
    s.push_str(&sec("cut", 24, 58, 460, t("sec_cutoff"), t("desc_cutoff")));
    for (i, k) in ["cut_s", "cut_a", "cut_b", "cut_c"].iter().enumerate() { s.push_str(&row_src(k, 24, 124 + 40 * i as i32)); }
    s.push_str(&sec("level", 24, 296, 460, t("sec_level"), t("desc_level")));
    s.push_str(&row_src("level", 24, 362));
    s.push_str(&sec("method", 24, 412, 460, t("sec_method"), t("desc_method")));
    s.push_str(&toggle_src("cross", 24, 478, t("cross_cat"), t("cross_all")));
    s.push_str(&toggle_src("auto", 24, 518, t("off"), t("on")));
    // 열 2 (x 520): 가중치 7 + 성적 3
    s.push_str(&sec("w", 520, 58, 436, t("sec_weights"), t("desc_weights")));
    for (i, k) in ["damage", "hp", "defence", "mr", "move", "as", "range"].iter().enumerate() { s.push_str(&row_src(k, 520, 124 + 40 * i as i32)); }
    s.push_str(&sec("live", 520, 412, 436, t("sec_live"), t("desc_live")));
    s.push_str(&row_src("live_weight", 520, 478));
    s.push_str(&row_src("winban", 520, 518));
    s.push_str(&toggle_src("patch_mode", 520, 558, t("patch_cur"), t("patch_all")));
    s.push_str(&format!("  #hint:label {{ @\"asset/base/style/main#label\"; x: 24px; y: 616px; width: 932px; height: 28px; size: 13; color: #9a9ca6ff; text: \"{}\"; }}\n", t("hint")));
    s.push_str(&format!("  #reset:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: 24px; y: 664px; width: 200px; height: 36px; text: {{ text: \"{}\"; size: 16; align_x: Center; align_y: Center; }} }}\n", t("reset")));
    s.push_str("}");
    s
}
const ROW_KEYS: [&str; 14] = ["cut_s", "cut_a", "cut_b", "cut_c", "level", "damage", "hp", "defence", "mr", "move", "as", "range", "live_weight", "winban"];
const TOGGLE_KEYS: [&str; 3] = ["cross", "auto", "patch_mode"];
fn fmt_val(k: &str, v: f32) -> String { match k { "level" => format!("{:.1}", v), _ => format!("{:.0}%", v * 100.0) } }
fn refresh_panel(ctx: &mut StableClient<'_>, panel: &str) {
    let c = cfg();
    for k in ROW_KEYS { ctx.ui_set_text(&format!("{}.v_{}", panel, k), &fmt_val(k, c.get(k))); }
    for k in TOGGLE_KEYS {
        let on = c.get(k) >= 0.5;
        let txt = match k { "cross" => if on { t("cross_all") } else { t("cross_cat") }, "patch_mode" => if on { t("patch_all") } else { t("patch_cur") }, _ => if on { t("on") } else { t("off") } };
        ctx.ui_set_text(&format!("{}.v_{}", panel, k), txt);
    }
}
fn toast(s: &str) { *TOAST_TEXT.lock().unwrap_or_else(|e| e.into_inner()) = Some(s.to_string()); TOAST_UNTIL.store(FRAME.load(Ordering::Relaxed) + 60 * 5, Ordering::Relaxed); }

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f = FRAME.fetch_add(1, Ordering::Relaxed);
            for ev in ctx.take_events() { if ev.event == EVT { let m = String::from_utf8_lossy(&ev.payload).to_string(); log(&format!("서버 응답: {}", m)); toast(&m); } }
            // 단축키 ` (엔진 키 이름 확인용: 처음 누른 키 몇 개 로그)
            let evs = ctx.input_events();
            if !evs.is_empty() && !KEY_LOGGED.load(Ordering::Relaxed) { log(&format!("input_events 표본: {:?}", evs.iter().map(|e| (e.kind, e.key.clone())).take(4).collect::<Vec<_>>())); if evs.len() >= 1 { KEY_LOGGED.store(true, Ordering::Relaxed); } }
            if ctx.client_scene_kind() != Some(ClientSceneKindV1::Main) { deactivate(); return; }
            if evs.iter().any(|e| e.kind == Some(mod_api_stable::InputEventKindV1::KeyPressed) && matches!(e.key.as_str(), "Backquote" | "Grave" | "`" | "Backtick")) { APPLY_REQ.store(true, Ordering::Relaxed); }
            // 매일 자동
            if let (Some((y, m, d)), true) = (ctx.game_date(), ctx.ui_exists("main.top.right")) {
                let today = y as i64 * 10000 + m as i64 * 100 + d as i64;
                let prev = LAST_DAY.swap(today, Ordering::Relaxed);
                if prev != i64::MIN && prev != today && cfg().auto_daily >= 0.5 { APPLY_REQ.store(true, Ordering::Relaxed); log("날짜 변경 → 자동 분류"); }
            }
            // 적용 요청 → 서버
            if APPLY_REQ.swap(false, Ordering::Relaxed) {
                let c = cfg();
                let rows = stat_scores(ctx, &c);
                if rows.is_empty() { log("champion 0개 — 적용 불가"); }
                else {
                    let mut p = c.encode(); p.push('\n');
                    for (n, cat, s) in &rows { p.push_str(&format!("{}\t{}\t{:.5}\n", n, cat, s)); }
                    let ok = ctx.send_command(CMD, p.as_bytes());
                    log(&format!("send_command {} rows={} → {}", CMD, rows.len(), ok));
                }
            }
            if f % 6 != 0 { return; }
            stat_color_tick(ctx, f);
            if !ctx.ui_exists(ACTIONS) || !ctx.ui_visible(VIEW).unwrap_or(false) { deactivate(); return; }
            if !ACTIVE.swap(true, Ordering::Relaxed) {
                log("champion_info 뷰 활성");
                // 언어: 게임 텍스트로 판정
                let sample = ctx.i18n("#asset/base/text/ui?main.champion_info.reset_tier").unwrap_or_default();
                LANG_KO.store(sample.is_empty() || sample.chars().any(|c| ('\u{ac00}'..='\u{d7a3}').contains(&c)), Ordering::Relaxed);
                *TINTS.lock().unwrap_or_else(|e| e.into_inner()) = Some(compute_tints(ctx));
            }
            let (b_cls, b_cfg, b_col, panel, tst) = (format!("{}.cta_btn", ACTIONS), format!("{}.cta_cfg", ACTIONS), format!("{}.cta_color", ACTIONS), format!("{}.cta_panel", VIEW), format!("{}.data.cta_toast", VIEW));
            if !ctx.ui_exists(&b_cls) {
                let o1 = ctx.ui_spawn_source(ACTIONS, &btn("cta_btn", 790, 200, t("classify")));
                let o2 = ctx.ui_spawn_source(ACTIONS, &btn("cta_cfg", 1000, 96, t("settings")));
                let o3 = ctx.ui_spawn_source(ACTIONS, &btn("cta_color", 1104, 90, if cfg().name_color >= 0.5 { t("color_on") } else { t("color_off") }));
                let o4 = ctx.ui_spawn_source(VIEW, &panel_src());
                // 토스트 = 카드 목록 하단에 5초 오버레이(배경 상자 + 라벨). action_buttons 행엔 빈 자리가 없다.
                let o5 = ctx.ui_spawn_source("main.top.right.champion_info.data", "cta_toast:empty { x: 500px; y: 492px; width: 900px; height: 30px; z: 900; visible: false;\n  #bg:color { ignore_event: true; width: 100%; height: 100%; color: #161721f0; rounding: Uniform { rounding: 8; } }\n  #t:label { @\"asset/base/style/main#bold_label\"; ignore_event: true; width: 100%; height: 100%; size: 14; color: #4dd999ff; align_x: Center; align_y: Center; text: \"\"; }\n}");
                log(&format!("스폰 btn={} cfg={} color={} panel={} toast={} rect={:?}", o1, o2, o3, o4, o5, ctx.ui_node_rect(&b_cls)));
                PANEL_DIRTY.store(true, Ordering::Relaxed);
                *NAME_COLOR.lock().unwrap_or_else(|e| e.into_inner()) = None;
                if !CLICKS_REGISTERED.swap(true, Ordering::Relaxed) {
                    ctx.ui_register_click(&b_cls, "", |_| APPLY_REQ.store(true, Ordering::Relaxed));
                    ctx.ui_register_click(&b_cfg, "", |_| { let v = !PANEL_OPEN.load(Ordering::Relaxed); PANEL_OPEN.store(v, Ordering::Relaxed); PANEL_DIRTY.store(true, Ordering::Relaxed); });
                    ctx.ui_register_click(&b_col, "", |_| { cfg_mut(|c| c.name_color = if c.name_color >= 0.5 { 0.0 } else { 1.0 }); PANEL_DIRTY.store(true, Ordering::Relaxed); });
                    ctx.ui_register_click(&format!("{}.close", panel), "", |_| PANEL_OPEN.store(false, Ordering::Relaxed));
                    ctx.ui_register_click(&format!("{}.reset", panel), "", |_| { cfg_mut(|c| *c = Config::default()); PANEL_DIRTY.store(true, Ordering::Relaxed); });
                    for k in ROW_KEYS {
                        let kk: &'static str = k;
                        ctx.ui_register_click(&format!("{}.dec_{}", panel, k), "", move |_| { cfg_mut(|c| c.adjust(kk, -STEP)); PANEL_DIRTY.store(true, Ordering::Relaxed); });
                        ctx.ui_register_click(&format!("{}.inc_{}", panel, k), "", move |_| { cfg_mut(|c| c.adjust(kk, STEP)); PANEL_DIRTY.store(true, Ordering::Relaxed); });
                    }
                    for k in TOGGLE_KEYS {
                        let kk: &'static str = k;
                        ctx.ui_register_click(&format!("{}.off_{}", panel, k), "", move |_| { cfg_mut(|c| c.set(kk, 0.0)); PANEL_DIRTY.store(true, Ordering::Relaxed); });
                        ctx.ui_register_click(&format!("{}.on_{}", panel, k), "", move |_| { cfg_mut(|c| c.set(kk, 1.0)); PANEL_DIRTY.store(true, Ordering::Relaxed); });
                    }
                }
            }
            // 패널 표시/값
            let open = PANEL_OPEN.load(Ordering::Relaxed);
            if ctx.ui_visible(&panel).unwrap_or(false) != open { ctx.ui_set_visible(&panel, open); }
            if PANEL_DIRTY.swap(false, Ordering::Relaxed) {
                if open { refresh_panel(ctx, &panel); }
                ctx.ui_set_text(&b_col, if cfg().name_color >= 0.5 { t("color_on") } else { t("color_off") });
            }
            // 토스트
            if ctx.ui_exists(&tst) {
                let show = f < TOAST_UNTIL.load(Ordering::Relaxed);
                if show { if let Some(m) = TOAST_TEXT.lock().unwrap_or_else(|e| e.into_inner()).as_ref() { ctx.ui_set_text(&format!("{}.t", tst), m); } }
                if ctx.ui_visible(&tst).unwrap_or(false) != show { ctx.ui_set_visible(&tst, show); }
            }
            // 이름 색칠(카드 목록은 필터/검색으로 바뀌므로 매번 대조, 색 캐시로 set 최소화)
            let on = cfg().name_color >= 0.5;
            let tints = TINTS.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_default();
            let mut g = NAME_COLOR.lock().unwrap_or_else(|e| e.into_inner());
            let cache = g.get_or_insert_with(HashMap::new);
            for card in ctx.ui_child_names(CARDS) {
                let want = if on { match tints.get(&card).copied().unwrap_or(0) { 1 => COLOR_BUFF, -1 => COLOR_NERF, _ => COLOR_NORMAL } } else { COLOR_NORMAL };
                if cache.get(&card).map(|c| c == want).unwrap_or(want == COLOR_NORMAL && !on && !cache.contains_key(&card)) { continue; }
                let p = format!("{}.{}.name", CARDS, card);
                if ctx.ui_exists(&p) { ctx.ui_set_properties(&p, &format!("color: {};", want)); cache.insert(card, want.to_string()); }
            }
        }));
    }
}
fn deactivate() {
    if ACTIVE.swap(false, Ordering::Relaxed) { PANEL_OPEN.store(false, Ordering::Relaxed); *NAME_COLOR.lock().unwrap_or_else(|e| e.into_inner()) = None; }
}

// ───────── 서버 ─────────
fn ji(v: &Value, k: &str) -> i64 { v.get(k).and_then(|x| x.as_i64()).or_else(|| v.get(k).and_then(|x| x.as_f64()).map(|f| f as i64)).unwrap_or(0) }
/// 시즌 성적: 챔피언별 (wins, matches, bans) + (대회 수, 세트 수).
/// wins/matches = Competition `statistics[athlete].champion_detail`; bans = 대회 matches → Match.replays → MatchReplay.blue_ban/red_ban.
fn live_stats(ctx: &StableServerCtx<'_>, all: bool) -> (HashMap<String, (i64, i64, i64)>, usize, usize) {
    let mut out: HashMap<String, (i64, i64, i64)> = HashMap::new();
    let (mut comps, mut sets) = (0usize, 0usize);
    let mut match_ids: Vec<i64> = Vec::new();
    for kind in [RecordKindV1::LeagueCompetition, RecordKindV1::TournamentCompetition] {
        for id in ctx.record_ids(kind) {
            let Some(j) = ctx.record_get_json(kind, id, "") else { continue };
            let Ok(v) = serde_json::from_str::<Value>(&j) else { continue };
            let finalized = v.get("finalized").and_then(|x| x.as_bool()).unwrap_or(false);
            if !all && finalized { continue; }
            let Some(stats) = v.get("statistics").and_then(|x| x.as_object()) else { continue };
            if stats.is_empty() { continue; }
            comps += 1;
            for key in ["matches", "playoffs", "promotion_series"] { if let Some(arr) = v.get(key).and_then(|x| x.as_array()) { match_ids.extend(arr.iter().filter_map(|x| x.as_i64())); } }
            for (_aid, a) in stats {
                if let Some(cd) = a.get("champion_detail").and_then(|x| x.as_object()) {
                    for (champ, d) in cd { let e = out.entry(champ.clone()).or_insert((0, 0, 0)); e.0 += ji(d, "wins"); e.1 += ji(d, "matches"); }
                }
            }
        }
    }
    // 밴: Match.replays → MatchReplay.{blue_ban,red_ban}
    for mid in match_ids {
        let Some(mj) = ctx.record_get_json(RecordKindV1::Match, mid as usize, "replays") else { continue };
        let Ok(rv) = serde_json::from_str::<Value>(&mj) else { continue };
        for rid in rv.as_array().into_iter().flatten().filter_map(|x| x.as_i64()) {
            let Some(rj) = ctx.record_get_json(RecordKindV1::MatchReplay, rid as usize, "") else { continue };
            let Ok(r) = serde_json::from_str::<Value>(&rj) else { continue };
            sets += 1;
            for key in ["blue_ban", "red_ban"] {
                for b in r.get(key).and_then(|x| x.as_array()).into_iter().flatten().filter_map(|x| x.as_str()) { out.entry(b.to_string()).or_insert((0, 0, 0)).2 += 1; }
            }
        }
    }
    (out, comps, sets)
}
fn cut(c: &Config, frac: f32) -> &'static str { if frac < c.cut_s { "S" } else if frac < c.cut_a { "A" } else if frac < c.cut_b { "B" } else if frac < c.cut_c { "C" } else { "D" } }
struct Srv;
impl StableServerExtension for Srv {
    fn handle_command(&self, ctx: &mut StableServerCtx<'_>, cmd: &StableCommand<'_>) -> CommandResultV1 {
        if cmd.command != CMD { return CommandResultV1::Pass; }
        let reply = cmd.reply_target();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| apply(ctx, cmd)));
        let msg = match r { Ok(m) => m, Err(_) => "오류: 서버 패닉".to_string() };
        log(&format!("[server] {}", msg));
        ctx.emit_event(reply, EVT, msg.as_bytes());
        CommandResultV1::Handled
    }
}
fn apply(ctx: &mut StableServerCtx<'_>, cmd: &StableCommand<'_>) -> String {
    let p = String::from_utf8_lossy(cmd.payload).to_string();
    let mut lines = p.lines();
    let c = Config::decode(lines.next().unwrap_or(""));
    let mut rows: Vec<(String, usize, f32)> = Vec::new();
    for l in lines { let parts: Vec<&str> = l.split('\t').collect(); if parts.len() == 3 { if let (Ok(cat), Ok(s)) = (parts[1].parse::<usize>(), parts[2].parse::<f32>()) { rows.push((parts[0].to_string(), cat.min(4), s)); } } }
    let n = rows.len(); if n == 0 { return "오류: 챔피언 0개".into(); }
    let Some(team) = cmd.sender_team_id else { return "오류: 팀 없음".into() };
    // 성적 혼합
    let mut scores: Vec<f32> = rows.iter().map(|r| r.2).collect();
    let mut live_note = String::new();
    if c.live_weight > 0.0 {
        let (st, comps, sets) = live_stats(ctx, c.patch_mode >= 0.5);
        let k = c.bayes_k.max(0.0);
        let wr: Vec<f32> = rows.iter().map(|r| { let (w, m, _) = st.get(&r.0).copied().unwrap_or((0, 0, 0)); let d = m as f32 + k; if d > 0.0 { (w as f32 + k * 0.5) / d } else { 0.5 } }).collect();
        let br: Vec<f32> = rows.iter().map(|r| { let b = st.get(&r.0).map(|x| x.2).unwrap_or(0); if sets > 0 { b as f32 / sets as f32 } else { 0.0 } }).collect();
        let (p_wr, p_br) = (percentile_ranks(&wr), percentile_ranks(&br));
        let a = c.live_weight.clamp(0.0, 1.0);
        let bw = c.winban.clamp(0.0, 1.0);
        for i in 0..n { let emp = bw * p_wr[i] + (1.0 - bw) * p_br[i]; scores[i] = (1.0 - a) * rows[i].2 + a * emp; }
        live_note = format!(" · 성적 {}대회 {}세트", comps, sets);
    }
    // 컷오프
    let mut tiers: Vec<(String, &str)> = Vec::new();
    if c.cross >= 0.5 {
        let gr = percentile_ranks(&scores);
        for i in 0..n { tiers.push((rows[i].0.clone(), cut(&c, 1.0 - gr[i]))); }
    } else {
        let mut buckets: [Vec<(usize, f32)>; 5] = Default::default();
        for i in 0..n { buckets[rows[i].1].push((i, scores[i])); }
        for b in buckets.iter_mut() {
            b.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap_or(std::cmp::Ordering::Equal));
            let m = b.len().max(1) as f32;
            for (j, (i, _)) in b.iter().enumerate() { tiers.push((rows[*i].0.clone(), cut(&c, (j as f32 + 0.5) / m))); }
        }
    }
    // 쓰기: 기존 맵 병합
    let cur = ctx.team_get_json(team, "champion_tiers").unwrap_or_else(|| "{}".into());
    let mut map: serde_json::Map<String, Value> = serde_json::from_str::<Value>(&cur).ok().and_then(|v| v.as_object().cloned()).unwrap_or_default();
    let mut cnt = [0usize; 5];
    for (name, tier) in &tiers { map.insert(name.clone(), Value::String(tier.to_string())); cnt[match *tier { "S" => 0, "A" => 1, "B" => 2, "C" => 3, _ => 4 }] += 1; }
    let body = Value::Object(map).to_string();
    let ok = ctx.team_set_json(team, "champion_tiers", &body);
    if !ok { return format!("오류: champion_tiers 쓰기 거부({}B)", body.len()); }
    let sync = match team_sync::unicast_team(ctx, team) { Ok(m) => { log(&format!("[server] sync: {}", m)); "" } Err(e) => { log(&format!("[server] sync 실패: {}", e)); " (화면 반영은 다음 진행 시)" } };
    format!("티어 {}개 적용: S{} A{} B{} C{} D{}{}{}", n, cnt[0], cnt[1], cnt[2], cnt[3], cnt[4], live_note, sync)
}

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "custom_tier_assignment (stable 0.6.0)");
    *CFG.lock().unwrap_or_else(|e| e.into_inner()) = Config::load();
    let v = host.game_version();
    log(&format!("INIT game {}.{}.{} host_abi={} cfg={}", v.major, v.minor, v.patch, host.abi_level(), cfg().encode()));
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d.set_server_extension(Srv);
    d
}
declare_stable_mod!(init);
