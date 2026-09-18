//! coaching_staff_view_plus — 코치진 화면 빈 공간에 코치 능력치 10개 상시 표시 (daram2 원작 · 0.6.0 stable 포팅 2026-09-16).
//! 원작(클래식): `.ui` 오버라이드로 헤더 10칸을 넣고, post_render 에서 렌더 명령을 가로채 값을 그렸다.
//! stable 판:
//!   · 헤더 = `main.top.right.staff.data.category` 아래 `csb_hdr:empty{LeftToRight}` 를 `ui_spawn_source` 로 1회 스폰
//!   · 값   = 각 행 `…data.contents.<staff_id>` 아래 `csb_vals:empty{…10 label}` 스폰(행이 재생성되면 다시)
//!   · 데이터 = `record_get_json(Staff, id, "stat")` → banpick/strategy/… 10필드
//!   · 헤더 클릭 정렬은 **미지원**(stable 에 자식 순서 변경 API 없음 — 게임 자체 정렬(이름/나이/연봉/직책)만).
//! ⬜0.6.0 인게임: 스폰 위치(x 680px 기준)·색·값 확인. DBG=true 면 Staff 레코드 JSON 을 debug.log 에 1회 남긴다.
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, ClientSceneKindV1, LogLevel, RecordKindV1, StableClient, StableExtension, StableHost, StableMod};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

const MOD_ID: &str = "coaching_staff_view_plus";
const DBG: bool = true;
const VIEW: &str = "main.top.right.staff";
const NUM_STATS: usize = 10;
const STAT_KEYS: [&str; NUM_STATS] = ["banpick", "strategy", "negotiation", "judge_ability", "judge_potential", "feedback", "power_analysis", "control_coaching", "judgment_coaching", "mental_coaching"];
/// 컬럼 시작 x·폭(ui px). 초기값은 0.5.x 레이아웃, 활성화 후 `role` 헤더 rect 로 재계산(0.6.0 은 직책 컬럼이 더 오른쪽).
static COL_X: Mutex<f32> = Mutex::new(680.0);
static COL_W: Mutex<f32> = Mutex::new(92.0);
static LAYOUT_DONE: AtomicBool = AtomicBool::new(false);
fn col_x() -> f32 { *COL_X.lock().unwrap_or_else(|e| e.into_inner()) }
fn col_w() -> f32 { *COL_W.lock().unwrap_or_else(|e| e.into_inner()) }
const HDR_ID: &str = "csb_hdr";
const VAL_ID: &str = "csb_vals";

static FRAME: AtomicU64 = AtomicU64::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static JSON_DUMPED: AtomicBool = AtomicBool::new(false);
/// staff_id → (값 10개) 캐시 (레코드 JSON 은 매 프레임 읽지 않는다)
static STATS: Mutex<Option<HashMap<usize, [i64; NUM_STATS]>>> = Mutex::new(None);
static LAST_ROWS: Mutex<Option<Vec<String>>> = Mutex::new(None);

#[link(name = "kernel32")]
extern "system" { fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32; }
fn log(s: &str) {
    if !DBG { return; }
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    if let Some(i) = exe.rfind(|c| c == '\\' || c == '/') {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!(r"{}\mods\{}\debug.log", &exe[..i], MOD_ID)) {
            let _ = writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), s);
        }
    }
}

fn tier_color(v: i64) -> &'static str {
    if v >= 81 { "#ff8c33ff" } else if v >= 61 { "#b873e6ff" } else if v >= 41 { "#599ef2ff" } else if v >= 21 { "#4dd999ff" } else { "#a8a8b3ff" }
}
/// `{"banpick":12,"strategy":34,…}` 에서 정수 필드 뽑기(의존성 없는 미니 파서).
fn json_int(j: &str, key: &str) -> Option<i64> {
    let pat = format!("\"{}\":", key);
    let i = j.find(&pat)? + pat.len();
    let rest = j[i..].trim_start();
    let end = rest.find(|c: char| !(c.is_ascii_digit() || c == '-' || c == '.')).unwrap_or(rest.len());
    rest[..end].split('.').next()?.parse().ok()
}
fn staff_stats(ctx: &StableClient<'_>, sid: usize) -> Option<[i64; NUM_STATS]> {
    {
        let g = STATS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(m) = g.as_ref() { if let Some(v) = m.get(&sid) { return Some(*v); } }
    }
    let j = ctx.record_get_json(RecordKindV1::Staff, sid, "stat")
        .or_else(|| ctx.record_get_json(RecordKindV1::Staff, sid, ""))?;
    if !JSON_DUMPED.swap(true, Ordering::Relaxed) { log(&format!("Staff {} json: {}", sid, &j[..j.len().min(1500)])); }
    let mut out = [0i64; NUM_STATS];
    for (i, k) in STAT_KEYS.iter().enumerate() {
        out[i] = json_int(&j, k).or_else(|| { let s = format!("\"stat\":{{"); j.find(&s).and_then(|p| json_int(&j[p..], k)) })?;
    }
    let mut g = STATS.lock().unwrap_or_else(|e| e.into_inner());
    g.get_or_insert_with(HashMap::new).insert(sid, out);
    Some(out)
}

fn header_src() -> String {
    let mut s = format!("{}:empty {{ x: {}px; width: {}px; height: 20px; child_type: LeftToRight {{ spacing: 0; }}\n", HDR_ID, col_x() as i32, (col_w() * NUM_STATS as f32) as i32);
    for (i, k) in STAT_KEYS.iter().enumerate() {
        s.push_str(&format!("  #csb_h{}:label {{ @\"asset/base/style/main#label\"; width: {}px; height: 20px; size: 13; align_x: Center; align_y: Center; ignore_event: true; text: \"#asset/base/text/ui?staff.stat.{}\"; }}\n", i, col_w() as i32, k));
    }
    s.push('}');
    s
}
fn values_src(v: &[i64; NUM_STATS]) -> String {
    let mut s = format!("{}:empty {{ x: {}px; width: {}px; height: 100%; ignore_event: true; child_type: LeftToRight {{ spacing: 0; }}\n", VAL_ID, col_x() as i32, (col_w() * NUM_STATS as f32) as i32);
    for (i, val) in v.iter().enumerate() {
        s.push_str(&format!("  #csb_v{}:label {{ @\"asset/base/style/main#label\"; width: {}px; height: 100%; size: 18; align_x: Center; align_y: Center; ignore_event: true; color: {}; text: \"{}\"; }}\n", i, col_w() as i32, tier_color(*val), val));
    }
    s.push('}');
    s
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
            if !ACTIVE.swap(true, Ordering::Relaxed) { log("staff 뷰 활성"); }
            // 헤더
            let hdr = format!("{}.{}", cat, HDR_ID);
            if !ctx.ui_exists(&hdr) {
                let ok = ctx.ui_spawn_source(&cat, &header_src());
                log(&format!("헤더 스폰 {} → exists={} rect={:?}", ok, ctx.ui_exists(&hdr), ctx.ui_node_rect(&hdr)));
            }
            // 레이아웃 보정(1회): 스폰된 헤더의 실측 폭으로 ui px↔screen 배율을 얻고, `role` 헤더 오른쪽부터 시작.
            if !LAYOUT_DONE.load(Ordering::Relaxed) {
                if let (Some(hr), Some(cr), Some(rr)) = (ctx.ui_node_rect(&hdr), ctx.ui_node_rect(&cat), ctx.ui_node_rect(&format!("{}.role", cat))) {
                    if hr.2 > 1.0 && cr.2 > 1.0 {
                        let scale = hr.2 / (col_w() * NUM_STATS as f32);          // screen px per ui px
                        let free_x = (rr.0 + rr.2 - cr.0) / scale + 24.0;         // role 오른쪽 + 여백 (ui px)
                        let avail = cr.2 / scale - free_x - 8.0;
                        let w = (avail / NUM_STATS as f32).min(92.0).max(56.0);
                        *COL_X.lock().unwrap_or_else(|e| e.into_inner()) = free_x;
                        *COL_W.lock().unwrap_or_else(|e| e.into_inner()) = w;
                        LAYOUT_DONE.store(true, Ordering::Relaxed);
                        log(&format!("레이아웃 보정: scale={:.3} cat={:?} role={:?} hdr={:?} → x={:.0} w={:.0}", scale, cr, rr, hr, free_x, w));
                        ctx.ui_remove_node(&hdr);
                        let _ = ctx.ui_spawn_source(&cat, &header_src());
                        for r in ctx.ui_child_names(&contents) { let v = format!("{}.{}.{}", contents, r, VAL_ID); if ctx.ui_exists(&v) { ctx.ui_remove_node(&v); } }
                    }
                }
            }
            // 행
            let rows = ctx.ui_child_names(&contents);
            {
                let mut g = LAST_ROWS.lock().unwrap_or_else(|e| e.into_inner());
                if g.as_ref() != Some(&rows) { log(&format!("rows={:?}", rows)); *g = Some(rows.clone()); }
            }
            for r in rows {
                let Ok(sid) = r.parse::<usize>() else { continue };
                let row = format!("{}.{}", contents, r);
                let vals = format!("{}.{}", row, VAL_ID);
                if ctx.ui_exists(&vals) { continue; }
                let Some(v) = staff_stats(ctx, sid) else { log(&format!("staff {} stat 읽기 실패", sid)); continue };
                let ok = ctx.ui_spawn_source(&row, &values_src(&v));
                if !ok { log(&format!("값 스폰 실패 row={}", row)); }
                else if f < 3000 { log(&format!("값 스폰 row={} rect={:?} vals={:?}", row, ctx.ui_node_rect(&vals), v)); }
            }
        }));
    }
}
fn deactivate() {
    if ACTIVE.swap(false, Ordering::Relaxed) {
        LAYOUT_DONE.store(false, Ordering::Relaxed);
        *STATS.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *LAST_ROWS.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
}
fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "coaching_staff_view_plus (stable 0.6.0)");
    let v = host.game_version();
    log(&format!("INIT game {}.{}.{} host_abi={}", v.major, v.minor, v.patch, host.abi_level()));
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d
}
declare_stable_mod!(init);
