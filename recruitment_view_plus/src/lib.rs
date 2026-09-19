//! recruitment_view_plus — 영입(스카우트) 리스트를 영입 정보 ↔ 능력치로 전환 (daram2 원작 · 0.6.0 stable 포팅 2026-09-16).
//! 원작(클래식): `scout.ui` 오버라이드(토글 버튼·능력치 헤더 `sab_hdr`) + post_render 렌더 명령 가로채기 + 헤더 클릭 정렬.
//! stable 판:
//!   · 토글 = `main.top.right.scout` 아래 `rab_toggle:color_icon_button` 스폰 + `ui_register_click`
//!   · 탭(섹션) = contents.{search_list,interested,released,all_players} 중 보이는 것 1개. 행 = `…<section>.list.contents.<row>`
//!     행 id `candidate_<athlete>` → 선수 12 능력치, `staff_<staff>` → 코치 10 능력치 (원작 row_kind 와 동일)
//!   · 능력치 모드 = 섹션별 hide 컬럼(헤더·행) `ui_set_visible(false)` + 우리 헤더(`rab_hdr`)/값(`rab_vals`) 스폰
//!     ★헤더·행이 LeftToRight 자동배치라 우리 컬럼은 항상 게임 컬럼(delete 포함) **뒤**에 붙는다(자식 순서 변경 API 없음).
//!   · 데이터 = `record_get_json(Athlete|Staff, id, "stat")`
//!   · 헤더 클릭 정렬 = **미지원**(stable 에 자식 순서 변경 없음)
//! 상태(모드 on/off) = mods\recruitment_view_plus\recruitment_view_plus.cfg (`show_ability=1`).
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, ClientSceneKindV1, LogLevel, RecordKindV1, StableClient, StableExtension, StableHost, StableMod};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

const MOD_ID: &str = "recruitment_view_plus";
const DBG: bool = false; // 09-19 확정 배포(진단 시 true)
const VIEW: &str = "main.top.right.scout";
const NA: usize = 12;
const NS: usize = 10;
const PLAYER_KEYS: [&str; NA] = ["last_hit", "skill_avoid", "skill_hit", "control_speed", "positioning", "judgement", "mental", "concentration", "order", "roaming", "aggressive", "ego"];
const STAFF_KEYS: [&str; NS] = ["banpick", "strategy", "negotiation", "judge_ability", "judge_potential", "feedback", "power_analysis", "control_coaching", "judgment_coaching", "mental_coaching"];
const HDR_ID: &str = "rab_hdr";
const VAL_ID: &str = "rab_vals";
const BTN_ID: &str = "rab_toggle";
const PROBE_W: f32 = 60.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Mode { Player, Staff }
struct TabSpec { section: &'static str, hide_player: &'static [&'static str], hide_staff: &'static [&'static str] }
/// 원작 TABS 그대로(0.6.0 search_list 헤더 = name/age/position/team/contract/squad_status/potential/salary/transfer_fee/league/recommendation/report_date/delete).
const TABS: &[TabSpec] = &[
    TabSpec { section: "search_list", hide_player: &["team", "contract", "squad_status", "salary", "transfer_fee", "league", "report_date"], hide_staff: &[] },
    TabSpec { section: "interested", hide_player: &["team", "contract", "salary", "transfer_fee", "spacer"], hide_staff: &["team", "contract", "salary", "transfer_fee", "recommendation", "potential", "spacer"] },
    TabSpec { section: "released", hide_player: &["team", "contract", "salary", "transfer_fee", "spacer"], hide_staff: &["team", "contract", "salary", "transfer_fee", "recommendation", "potential", "spacer"] },
    TabSpec { section: "all_players", hide_player: &["team", "contract", "salary", "transfer_fee", "spacer"], hide_staff: &["team", "contract", "salary", "transfer_fee", "recommendation", "potential", "spacer"] },
];
fn hide_of(t: &TabSpec, m: Mode) -> &'static [&'static str] { match m { Mode::Player => t.hide_player, Mode::Staff => t.hide_staff } }
fn keys_of(m: Mode) -> &'static [&'static str] { match m { Mode::Player => &PLAYER_KEYS, Mode::Staff => &STAFF_KEYS } }
fn row_kind(id: &str) -> Option<(Mode, usize)> {
    if let Some(n) = id.strip_prefix("candidate_") { return n.parse().ok().map(|a| (Mode::Player, a)); }
    if let Some(n) = id.strip_prefix("staff_") { return n.parse().ok().map(|s| (Mode::Staff, s)); }
    None
}

static FRAME: AtomicU64 = AtomicU64::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static SHOW: AtomicBool = AtomicBool::new(false);
static JSON_DUMPED: AtomicBool = AtomicBool::new(false);
static CLICK_REGISTERED: AtomicBool = AtomicBool::new(false);
/// 섹션 → 현재 적용 상태 (eff_show, mode)
static APPLIED: Mutex<Option<HashMap<&'static str, (bool, Mode)>>> = Mutex::new(None);
/// (섹션, 모드) → 컬럼 폭(ui px)
static LAYOUT: Mutex<Option<HashMap<(&'static str, Mode), f32>>> = Mutex::new(None);
static STATS: Mutex<Option<HashMap<(Mode, usize), Vec<i64>>>> = Mutex::new(None);
static LAST_ROWS: Mutex<Option<Vec<String>>> = Mutex::new(None);

#[link(name = "kernel32")]
extern "system" { fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32; }
fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    exe.rfind(|c| c == '\\' || c == '/').map(|i| format!(r"{}\mods\{}", &exe[..i], MOD_ID))
}
fn log(s: &str) {
    if !DBG { return; }
    if let Some(d) = mod_dir() {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!(r"{}\debug.log", d)) {
            let _ = writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), s);
        }
    }
}
fn cfg_path() -> Option<String> { mod_dir().map(|d| format!(r"{}\recruitment_view_plus.cfg", d)) }
fn load_show() -> bool {
    cfg_path().and_then(|p| std::fs::read_to_string(p).ok()).map(|t| t.lines().any(|l| l.trim() == "show_ability=1")).unwrap_or(false)
}
fn save_show(on: bool) { if let Some(p) = cfg_path() { let _ = std::fs::write(p, format!("show_ability={}\n", if on { 1 } else { 0 })); } }

fn tier_color(v: i64) -> &'static str {
    if v >= 81 { "#ff8c33ff" } else if v >= 61 { "#b873e6ff" } else if v >= 41 { "#599ef2ff" } else if v >= 21 { "#4dd999ff" } else { "#a8a8b3ff" }
}
fn json_int(j: &str, key: &str) -> Option<i64> {
    let pat = format!("\"{}\":", key);
    let i = j.find(&pat)? + pat.len();
    let rest = j[i..].trim_start();
    let end = rest.find(|c: char| !(c.is_ascii_digit() || c == '-' || c == '.')).unwrap_or(rest.len());
    rest[..end].split('.').next()?.parse().ok()
}
fn stats(ctx: &StableClient<'_>, m: Mode, id: usize) -> Option<Vec<i64>> {
    {
        let g = STATS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(v) = g.as_ref().and_then(|h| h.get(&(m, id))) { return Some(v.clone()); }
    }
    let kind = match m { Mode::Player => RecordKindV1::Athlete, Mode::Staff => RecordKindV1::Staff };
    let j = ctx.record_get_json(kind, id, "stat")?;
    if !JSON_DUMPED.swap(true, Ordering::Relaxed) { log(&format!("{:?} {} stat: {}", m, id, &j[..j.len().min(800)])); }
    let mut v = Vec::with_capacity(keys_of(m).len());
    for k in keys_of(m) { v.push(json_int(&j, k)?); }
    STATS.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(HashMap::new).insert((m, id), v.clone());
    Some(v)
}

fn header_src(m: Mode, cw: f32) -> String {
    let keys = keys_of(m);
    let mut s = format!("{}:empty {{ x: 0px; width: {}px; height: 100%; child_type: LeftToRight {{ spacing: 0; }}\n", HDR_ID, (cw * keys.len() as f32) as i32);
    for (i, k) in keys.iter().enumerate() {
        let text = match m { Mode::Player => format!("#asset/base/text/athlete?stat.{}", k), Mode::Staff => format!("#asset/base/text/ui?staff.stat.{}", k) };
        s.push_str(&format!("  #rab_h{}:label {{ @\"asset/base/style/main#label\"; width: {}px; height: 100%; size: 12; align_x: Center; align_y: Center; ignore_event: true; text: \"{}\"; }}\n", i, cw as i32, text));
    }
    s.push('}'); s
}
fn values_src(cw: f32, v: &[i64]) -> String {
    let mut s = format!("{}:empty {{ x: 0px; width: {}px; height: 100%; ignore_event: true; child_type: LeftToRight {{ spacing: 0; }}\n", VAL_ID, (cw * v.len() as f32) as i32);
    for (i, val) in v.iter().enumerate() {
        s.push_str(&format!("  #rab_v{}:label {{ @\"asset/base/style/main#label\"; width: {}px; height: 100%; size: 17; align_x: Center; align_y: Center; ignore_event: true; color: {}; text: \"{}\"; }}\n", i, cw as i32, tier_color(*val), val));
    }
    s.push('}'); s
}
/// 토글 버튼 소스. 부모 = `scout.tabs`(LeftToRight) 끝에 붙는다.
/// (scout 루트 직속 스폰 2회 실패의 진짜 원인은 핸들러 중복 등록(상쇄)이었다 — 위치 자체는 문제 없었을 가능성 큼. tabs 뒤 배치가 보기 좋아 유지.)
fn toggle_src() -> String {
    format!("{}:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: 12px; y: 0px; width: 40px; height: 36px; icon: {{ source: \"asset/base/ui/icons/swap\"; rect: {{ x: 10; y: 8; w: 20; h: 20; }} }}
  #icon:image {{ ignore_event: true; x: 10px; y: 8px; width: 20px; height: 20px; source: \"asset/base/ui/icons/swap\"; color: #d7dbe4ff; }}
}}", BTN_ID)
}

fn set_cols(ctx: &mut StableClient<'_>, base: &str, cols: &[&str], visible: bool) {
    for c in cols { let p = format!("{}.{}", base, c); if ctx.ui_exists(&p) { ctx.ui_set_visible(&p, visible); } }
}
fn remove_ours(ctx: &mut StableClient<'_>, header: &str, contents: &str, rows: &[String]) {
    let h = format!("{}.{}", header, HDR_ID); if ctx.ui_exists(&h) { ctx.ui_remove_node(&h); }
    for r in rows { let p = format!("{}.{}.{}", contents, r, VAL_ID); if ctx.ui_exists(&p) { ctx.ui_remove_node(&p); } }
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f = FRAME.fetch_add(1, Ordering::Relaxed);
            if f % 15 != 0 { return; }
            if ctx.client_scene_kind() != Some(ClientSceneKindV1::Main) { deactivate(); return; }
            if !ctx.ui_exists(VIEW) || !ctx.ui_visible(VIEW).unwrap_or(false) { deactivate(); return; }
            if !ACTIVE.swap(true, Ordering::Relaxed) { log("scout 뷰 활성"); }
            // 토글 버튼(뷰가 재생성되면 다시 — 탭 전환마다 scout 서브트리가 재생성된다: 0.6.0 실측)
            let tabs = format!("{}.tabs", VIEW);
            let btn = format!("{}.{}", tabs, BTN_ID);
            if !ctx.ui_exists(&btn) {
                let ok = ctx.ui_spawn_source(&tabs, &toggle_src());
                log(&format!("토글 스폰 {} tabs={:?}", ok, ctx.ui_node_rect(&tabs)));
                // 서브트리가 새로 만들어졌으니 섹션별 적용 상태도 무효(게임 컬럼이 다시 보이고 우리 헤더는 없다)
                *APPLIED.lock().unwrap_or_else(|e| e.into_inner()) = None;
                // ★클릭 핸들러는 경로 키로 영구 등록된다 — 노드가 재생성돼도 살아 있으므로 프로세스당 1회만(2회 등록 = 2회 발화 = 토글 상쇄, 0.6.0 실측)
                if ok && !CLICK_REGISTERED.swap(true, Ordering::Relaxed) { ctx.ui_register_click(&btn, "", |_c| { let v = !SHOW.load(Ordering::Relaxed); SHOW.store(v, Ordering::Relaxed); save_show(v); log(&format!("토글 클릭 → show={}", v)); }); }
            }
            // 활성 섹션 = contents.<section> 중 보이는 것
            let active = TABS.iter().find(|t| ctx.ui_visible(&format!("{}.contents.{}", VIEW, t.section)).unwrap_or(false));
            let Some(tab) = active else { if ctx.ui_exists(&btn) { ctx.ui_set_visible(&btn, false); } return; };
            let header = format!("{}.contents.{}.header", VIEW, tab.section);
            let contents = format!("{}.contents.{}.list.contents", VIEW, tab.section);
            if !ctx.ui_exists(&header) || !ctx.ui_exists(&contents) { return; }
            let rows: Vec<String> = ctx.ui_child_names(&contents);
            {
                let mut g = LAST_ROWS.lock().unwrap_or_else(|e| e.into_inner());
                if g.as_ref() != Some(&rows) { log(&format!("{} rows={:?}", tab.section, &rows[..rows.len().min(6)])); *g = Some(rows.clone()); }
            }
            // 모드 = 첫 행 종류; 행이 없으면 마지막 적용 모드 유지(없으면 Player)
            let prev = APPLIED.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|h| h.get(tab.section).copied());
            let mode = rows.iter().find_map(|r| row_kind(r).map(|(m, _)| m)).or(prev.map(|p| p.1)).unwrap_or(Mode::Player);
            let hide = hide_of(tab, mode);
            let can_show = !hide.is_empty();
            let show = SHOW.load(Ordering::Relaxed);
            let eff_show = show && can_show;
            if ctx.ui_exists(&btn) { ctx.ui_set_visible(&btn, can_show); }
            // 상태 전환(섹션별)
            if prev != Some((eff_show, mode)) {
                if let Some((_, pm)) = prev {
                    // 이전 상태 원복
                    set_cols(ctx, &header, hide_of(tab, pm), true);
                    for r in &rows { set_cols(ctx, &format!("{}.{}", contents, r), hide_of(tab, pm), true); }
                }
                remove_ours(ctx, &header, &contents, &rows);
                if eff_show {
                    set_cols(ctx, &header, hide, false);
                    for r in &rows { set_cols(ctx, &format!("{}.{}", contents, r), hide, false); }
                }
                APPLIED.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(HashMap::new).insert(tab.section, (eff_show, mode));
                log(&format!("[{}] 전환 show={} mode={:?}", tab.section, eff_show, mode));
            }
            if !eff_show { return; }
            // 레이아웃(섹션·모드별 1회): 프로브 헤더(60px×N) 스폰 → 실측으로 배율·남은 폭 계산 → 컬럼 폭 확정
            let hdr = format!("{}.{}", header, HDR_ID);
            let n = keys_of(mode).len() as f32;
            let known = LAYOUT.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|h| h.get(&(tab.section, mode)).copied());
            let cw = match known {
                Some(w) => w,
                None => {
                    if !ctx.ui_exists(&hdr) { let _ = ctx.ui_spawn_source(&header, &header_src(mode, PROBE_W)); return; }
                    let (Some(pr), Some(hr)) = (ctx.ui_node_rect(&hdr), ctx.ui_node_rect(&header)) else { return };
                    if pr.2 <= 1.0 || hr.2 <= 1.0 { return; }
                    let scale = pr.2 / (PROBE_W * n);
                    // 클리핑 부모(contents.<section>) 폭으로 가시 폭 제한
                    let sr = ctx.ui_node_rect(&format!("{}.contents.{}", VIEW, tab.section)).unwrap_or(hr);
                    let right = (hr.0 + hr.2).min(sr.0 + sr.2);
                    let avail = (right - pr.0) / scale - 8.0;
                    let w = (avail / n).min(80.0).max(40.0).floor();
                    LAYOUT.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(HashMap::new).insert((tab.section, mode), w);
                    log(&format!("[{}/{:?}] 레이아웃: scale={:.3} probe={:?} header={:?} section={:?} avail={:.0} → col_w={:.0}", tab.section, mode, scale, pr, hr, sr, avail, w));
                    ctx.ui_remove_node(&hdr);
                    w
                }
            };
            if !ctx.ui_exists(&hdr) { let _ = ctx.ui_spawn_source(&header, &header_src(mode, cw)); }
            // 행: 값 없는 행에 hide + 값 스폰(리스트 갱신으로 새 행이 생겨도 커버)
            for r in rows {
                let Some((m, id)) = row_kind(&r) else { continue };
                if m != mode { continue; }
                let row = format!("{}.{}", contents, r);
                let vals = format!("{}.{}", row, VAL_ID);
                if ctx.ui_exists(&vals) { continue; }
                set_cols(ctx, &row, hide, false);
                let Some(v) = stats(ctx, m, id) else { log(&format!("{:?} {} stat 읽기 실패", m, id)); continue };
                if !ctx.ui_spawn_source(&row, &values_src(cw, &v)) { log(&format!("값 스폰 실패 {}", row)); }
                else if f < 6000 { log(&format!("값 스폰 {} rect={:?}", row, ctx.ui_node_rect(&vals))); }
            }
        }));
    }
}
fn deactivate() {
    if ACTIVE.swap(false, Ordering::Relaxed) {
        *APPLIED.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *LAYOUT.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *STATS.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *LAST_ROWS.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
}
fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "recruitment_view_plus (stable 0.6.0)");
    SHOW.store(load_show(), Ordering::Relaxed);
    let v = host.game_version();
    log(&format!("INIT game {}.{}.{} host_abi={} show={}", v.major, v.minor, v.patch, host.abi_level(), SHOW.load(Ordering::Relaxed)));
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d
}
declare_stable_mod!(init);
