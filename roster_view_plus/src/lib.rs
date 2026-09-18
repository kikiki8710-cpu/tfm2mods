//! roster_view_plus — 선수단 화면 우측 컬럼을 경기 스탯 ↔ 선수 능력치 12개로 전환 (daram2 원작 · 0.6.0 stable 포팅 2026-09-16).
//! 원작(클래식): `.ui` 오버라이드(토글 버튼·능력치 헤더) + post_render 렌더 명령 가로채기 + 숙련 챔프 얼굴 UV 계산.
//! stable 판:
//!   · 토글 = `main.top.right.squad` 아래 `rvp_toggle:color_icon_button` 스폰 + `ui_register_click`
//!   · 능력치 모드 = 게임 컬럼(age~rating) 헤더·행 라벨 `ui_set_visible(false)` + 우리 헤더/값 스폰
//!   · 숙련 챔프 상위 4 = 행마다 `rvp_faces` 스폰, 얼굴은 `ui_set_champion_icon`(모드챔프 포함·UV 계산 불요)
//!   · 데이터 = `record_get_json(Athlete, id, "stat")` / `"champion_proficiency"`
//!   · 헤더 클릭 정렬 = **미지원**(stable 에 자식 순서 변경 없음)
//! 상태(모드 on/off) = mods\roster_view_plus\roster_view_plus.cfg (`show_ability=1`).
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, ClientSceneKindV1, LogLevel, RecordKindV1, StableClient, StableExtension, StableHost, StableMod};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

const MOD_ID: &str = "roster_view_plus";
const DBG: bool = true;
const VIEW: &str = "main.top.right.squad";
const NA: usize = 12;
const STAT_KEYS: [&str; NA] = ["last_hit", "skill_avoid", "skill_hit", "control_speed", "positioning", "judgement", "mental", "concentration", "order", "roaming", "aggressive", "ego"];
const HIDE_COLS: [&str; 10] = ["age", "salary", "main_position", "position", "game", "kill", "death", "assist", "level", "rating"];
const HDR_ID: &str = "rvp_hdr";
const VAL_ID: &str = "rvp_vals";
const FACE_ID: &str = "rvp_faces";
const BTN_ID: &str = "rvp_toggle";
const MAX_PROF: usize = 4;
const PROF_BOX: f32 = 46.0;
const PROF_GAP: f32 = 4.0;
const PROF_NUM_H: f32 = 12.0;

static FRAME: AtomicU64 = AtomicU64::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static SHOW: AtomicBool = AtomicBool::new(false);      // 능력치 모드
static APPLIED: AtomicBool = AtomicBool::new(false);   // 현재 화면에 능력치 모드가 적용돼 있음
static JSON_DUMPED: AtomicBool = AtomicBool::new(false);
static LAYOUT: Mutex<Option<(f32, f32, f32, f32)>> = Mutex::new(None); // (face_x, face_w, col_x, col_w) ui px
static STATS: Mutex<Option<HashMap<usize, ([i64; NA], Vec<(String, i64)>)>>> = Mutex::new(None);

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
fn cfg_path() -> Option<String> { mod_dir().map(|d| format!(r"{}\roster_view_plus.cfg", d)) }
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
/// `{"archer":{"value":123,…},"knight":45,…}` → [(id, value)] 상위 MAX_PROF (value 는 객체면 "value" 키, 아니면 숫자).
fn top_profs(j: &str) -> Vec<(String, i64)> {
    let mut out: Vec<(String, i64)> = Vec::new();
    let mut i = 0usize;
    let b = j.as_bytes();
    // 최상위 객체의 키만 훑는다: depth 1 에서 "key": 뒤 값
    let mut depth = 0i32; let mut in_str = false; let mut key_start = None::<usize>; let mut last_key = String::new();
    while i < b.len() {
        let c = b[i] as char;
        if in_str {
            if c == '\\' { i += 2; continue; }
            if c == '"' { in_str = false; if depth == 1 { if let Some(s) = key_start { last_key = j[s..i].to_string(); key_start = None; } } }
            i += 1; continue;
        }
        match c {
            '"' => { in_str = true; if depth == 1 { key_start = Some(i + 1); } }
            '{' => { depth += 1; if depth == 2 && !last_key.is_empty() { let sub_end = j[i..].find('}').map(|e| i + e).unwrap_or(j.len()); if let Some(v) = json_int(&j[i..sub_end], "value") { out.push((last_key.clone(), v)); } } }
            '}' => { depth -= 1; }
            ':' => { if depth == 1 && !last_key.is_empty() { let rest = j[i + 1..].trim_start(); if rest.starts_with(|ch: char| ch.is_ascii_digit()) { let end = rest.find(|ch: char| !(ch.is_ascii_digit() || ch == '.')).unwrap_or(rest.len()); if let Ok(v) = rest[..end].split('.').next().unwrap_or("0").parse::<i64>() { out.push((last_key.clone(), v)); } } } }
            _ => {}
        }
        i += 1;
    }
    out.sort_by(|x, y| y.1.cmp(&x.1).then_with(|| x.0.cmp(&y.0)));
    out.truncate(MAX_PROF);
    out
}
fn athlete_data(ctx: &StableClient<'_>, aid: usize) -> Option<([i64; NA], Vec<(String, i64)>)> {
    {
        let g = STATS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(m) = g.as_ref() { if let Some(v) = m.get(&aid) { return Some(v.clone()); } }
    }
    let j = ctx.record_get_json(RecordKindV1::Athlete, aid, "stat")?;
    let pj = ctx.record_get_json(RecordKindV1::Athlete, aid, "champion_proficiency").unwrap_or_default();
    if !JSON_DUMPED.swap(true, Ordering::Relaxed) { log(&format!("Athlete {} stat: {} | prof: {}", aid, &j[..j.len().min(800)], &pj[..pj.len().min(800)])); }
    let mut st = [0i64; NA];
    for (i, k) in STAT_KEYS.iter().enumerate() { st[i] = json_int(&j, k)?; }
    let profs = top_profs(&pj);
    let v = (st, profs);
    STATS.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(HashMap::new).insert(aid, v.clone());
    Some(v)
}

fn header_src(cx: f32, cw: f32) -> String {
    let mut s = format!("{}:empty {{ x: {}px; width: {}px; height: 20px; child_type: LeftToRight {{ spacing: 0; }}\n", HDR_ID, cx as i32, (cw * NA as f32) as i32);
    for (i, k) in STAT_KEYS.iter().enumerate() {
        s.push_str(&format!("  #rvp_h{}:label {{ @\"asset/base/style/main#label\"; width: {}px; height: 20px; size: 13; align_x: Center; align_y: Center; ignore_event: true; text: \"#asset/base/text/athlete?stat.{}\"; }}\n", i, cw as i32, k));
    }
    s.push('}'); s
}
fn values_src(cx: f32, cw: f32, v: &[i64; NA]) -> String {
    let mut s = format!("{}:empty {{ x: {}px; width: {}px; height: 100%; ignore_event: true; child_type: LeftToRight {{ spacing: 0; }}\n", VAL_ID, cx as i32, (cw * NA as f32) as i32);
    for (i, val) in v.iter().enumerate() {
        s.push_str(&format!("  #rvp_v{}:label {{ @\"asset/base/style/main#label\"; width: {}px; height: 100%; size: 18; align_x: Center; align_y: Center; ignore_event: true; color: {}; text: \"{}\"; }}\n", i, cw as i32, tier_color(*val), val));
    }
    s.push('}'); s
}
fn faces_src(fx: f32, box_w: f32, n: usize) -> String {
    let slot_h = box_w + PROF_NUM_H;
    let mut s = format!("{}:empty {{ x: {}px; anchor_y: 0.5; pivot_y: 0.5; width: {}px; height: {}px; ignore_event: true;\n", FACE_ID, fx as i32, (MAX_PROF as f32 * (box_w + PROF_GAP)) as i32, slot_h as i32);
    for k in 0..n {
        let bx = k as f32 * (box_w + PROF_GAP);
        s.push_str(&format!("  #pb{}:color {{ ignore_event: true; x: {:.0}px; y: 0px; width: {:.0}px; height: {:.0}px; color: #2a2d3aff; rounding: Uniform {{ rounding: 6; }} }}\n", k, bx, box_w, slot_h));
        s.push_str(&format!("  #pi{}:image {{ ignore_event: true; x: {:.0}px; y: 2px; width: {:.0}px; height: {:.0}px; }}\n", k, bx + 2.0, box_w - 4.0, box_w - 4.0));
        s.push_str(&format!("  #pn{}:label {{ @\"asset/base/style/main#bold_label\"; ignore_event: true; x: {:.0}px; y: {:.0}px; width: {:.0}px; height: {:.0}px; size: 12; align_x: Center; align_y: Center; text: \"\"; }}\n", k, bx, box_w, box_w, PROF_NUM_H));
    }
    s.push('}'); s
}
fn toggle_src() -> String {
    format!("{}:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: 320px; anchor_y: 1; pivot_y: 1; y: -12px; width: 40px; height: 40px; icon: {{ source: \"asset/base/ui/icons/swap\"; rect: {{ x: 10; y: 10; w: 20; h: 20; }} }}\n  #icon:image {{ ignore_event: true; x: 10px; y: 10px; width: 20px; height: 20px; source: \"asset/base/ui/icons/swap\"; color: #d7dbe4ff; }}\n}}", BTN_ID)
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f = FRAME.fetch_add(1, Ordering::Relaxed);
            if f % 15 != 0 { return; }
            if ctx.client_scene_kind() != Some(ClientSceneKindV1::Main) { deactivate(); return; }
            if !ctx.ui_exists(VIEW) || !ctx.ui_visible(VIEW).unwrap_or(false) { deactivate(); return; }
            let cat = format!("{}.data.category", VIEW);
            let contents = format!("{}.data.contents", VIEW);
            if !ctx.ui_exists(&cat) || !ctx.ui_exists(&contents) { return; }
            if !ACTIVE.swap(true, Ordering::Relaxed) { log("squad 뷰 활성"); }
            // 토글 버튼(1회)
            let btn = format!("{}.{}", VIEW, BTN_ID);
            if !ctx.ui_exists(&btn) {
                let ok = ctx.ui_spawn_source(VIEW, &toggle_src());
                log(&format!("토글 스폰 {} rect={:?}", ok, ctx.ui_node_rect(&btn)));
                if ok { ctx.ui_register_click(&btn, "", |_c| { let v = !SHOW.load(Ordering::Relaxed); SHOW.store(v, Ordering::Relaxed); save_show(v); }); }
            }
            let show = SHOW.load(Ordering::Relaxed);
            let applied = APPLIED.load(Ordering::Relaxed);
            let rows: Vec<String> = ctx.ui_child_names(&contents);
            if show != applied {
                // 게임 컬럼 표시 전환
                for c in HIDE_COLS { let p = format!("{}.{}", cat, c); if ctx.ui_exists(&p) { ctx.ui_set_visible(&p, !show); } }
                for r in &rows { for c in HIDE_COLS { let p = format!("{}.{}.{}", contents, r, c); if ctx.ui_exists(&p) { ctx.ui_set_visible(&p, !show); } } }
                if !show {
                    let h = format!("{}.{}", cat, HDR_ID); if ctx.ui_exists(&h) { ctx.ui_remove_node(&h); }
                    for r in &rows { for id in [VAL_ID, FACE_ID] { let p = format!("{}.{}.{}", contents, r, id); if ctx.ui_exists(&p) { ctx.ui_remove_node(&p); } } }
                }
                APPLIED.store(show, Ordering::Relaxed);
                log(&format!("모드 전환 show={}", show));
            }
            if !show { return; }
            // 레이아웃(1회): name 헤더 오른쪽부터 [얼굴 스트립][12 컬럼]. 배율 = 카테고리 실측폭 / (게임 헤더 rect 로 추정 불가) → 헤더 스폰 후 실측.
            let layout = *LAYOUT.lock().unwrap_or_else(|e| e.into_inner());
            let hdr = format!("{}.{}", cat, HDR_ID);
            let (fx, fw, cx, cw) = match layout {
                Some(l) => l,
                None => {
                    if !ctx.ui_exists(&hdr) { let _ = ctx.ui_spawn_source(&cat, &header_src(600.0, 60.0)); return; }
                    let (Some(hr), Some(cr0)) = (ctx.ui_node_rect(&hdr), ctx.ui_node_rect(&cat)) else { return };
                    // ★가시 폭 = data 노드(클리핑 부모) 폭 기준 — category 는 1600px 이지만 data 가 더 좁다(0.6.0 실측: 7컬럼만 보임)
                    let dr = ctx.ui_node_rect(&format!("{}.data", VIEW)).unwrap_or(cr0);
                    let cr = (cr0.0, cr0.1, (dr.0 + dr.2 - cr0.0).min(cr0.2), cr0.3);
                    if hr.2 <= 1.0 || cr.2 <= 1.0 { return; }
                    let name_r = ctx.ui_node_rect(&format!("{}.name", cat)).unwrap_or((cr.0, cr.1, 200.0, 20.0));
                    let scale = hr.2 / (60.0 * NA as f32);
                    // ★부모(category)가 자동 배치(LeftToRight)라 `x:` 가 절대값이 아니다 — 프로브 스폰(x=600)의 실측 x 로 보정치를 구한다.
                    let delta = (hr.0 - cr.0) / scale - 600.0;
                    let name_right = (name_r.0 + name_r.2 - cr.0) / scale + 16.0 - delta;
                    let total = cr.2 / scale - (name_right + delta) - 8.0;
                    let face_w = (MAX_PROF as f32 * (PROF_BOX + PROF_GAP)).min(total * 0.3);
                    let col_w = ((total - face_w - 8.0) / NA as f32).min(85.0).max(48.0);
                    let l = (name_right, face_w, name_right + face_w + 8.0, col_w);
                    *LAYOUT.lock().unwrap_or_else(|e| e.into_inner()) = Some(l);
                    log(&format!("레이아웃: scale={:.3} delta={:.0} hdr={:?} cat(clip)={:?} name={:?} → face_x={:.0} face_w={:.0} col_x={:.0} col_w={:.0}", scale, delta, hr, cr, name_r, l.0, l.1, l.2, l.3));
                    ctx.ui_remove_node(&hdr);
                    l
                }
            };
            if !ctx.ui_exists(&hdr) { let _ = ctx.ui_spawn_source(&cat, &header_src(cx, cw)); }
            let box_w = ((fw - (MAX_PROF as f32 - 1.0) * PROF_GAP) / MAX_PROF as f32).min(PROF_BOX).floor();
            for r in rows {
                let Ok(aid) = r.parse::<usize>() else { continue };
                let row = format!("{}.{}", contents, r);
                let vals = format!("{}.{}", row, VAL_ID);
                if ctx.ui_exists(&vals) { continue; }
                let Some((st, profs)) = athlete_data(ctx, aid) else { log(&format!("athlete {} 읽기 실패", aid)); continue };
                // ★행은 LeftToRight 자동배치: 스폰 순서 = 화면 순서. 얼굴 스트립을 먼저(name_slot 뒤 16px), 값 컬럼을 그 뒤(8px).
                let _ = (fx, cx);
                let faces = format!("{}.{}", row, FACE_ID);
                let mut fok = false;
                if box_w >= 14.0 && !profs.is_empty() {
                    fok = ctx.ui_spawn_source(&row, &faces_src(16.0, box_w, profs.len()));
                    if !fok { log(&format!("얼굴 스트립 스폰 실패 {}", row)); }
                }
                let vx = if fok { 8.0 } else { 16.0 + fw + 8.0 };
                if !ctx.ui_spawn_source(&row, &values_src(vx, cw, &st)) { log(&format!("값 스폰 실패 {}", row)); continue; }
                if fok {
                    {
                        for (k, (id, val)) in profs.iter().enumerate() {
                            let ok = ctx.ui_set_champion_icon(&format!("{}.pi{}", faces, k), id, box_w - 4.0, box_w - 4.0, 2.0);
                            ctx.ui_set_text(&format!("{}.pn{}", faces, k), &format!("{}", (val + 5) / 10));
                            if !ok { log(&format!("얼굴 실패 aid={} champ={}", aid, id)); }
                        }
                    }
                }
            }
        }));
    }
}
fn deactivate() {
    if ACTIVE.swap(false, Ordering::Relaxed) {
        APPLIED.store(false, Ordering::Relaxed);
        *LAYOUT.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *STATS.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
}
fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "roster_view_plus (stable 0.6.0)");
    SHOW.store(load_show(), Ordering::Relaxed);
    let v = host.game_version();
    log(&format!("INIT game {}.{}.{} host_abi={} show={}", v.major, v.minor, v.patch, host.abi_level(), SHOW.load(Ordering::Relaxed)));
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d
}
declare_stable_mod!(init);
