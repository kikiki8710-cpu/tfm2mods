//! facility_view_plus — 상품 생산시설 화면에 [전체 추가 생산]·[전체 신규 생산] 버튼 (daram2 원작 · 0.6.0 stable 포팅 2026-09-16).
//! 0.6.0 상품 시스템 = `Team.merchandise_products[]`(선수 athlete_id × product_type 레코드, 17필드).
//!   · 클라: `main.top.right.facility.contents.merchandise` 에 버튼 2개 + 수량 팝업(± 버튼) 스폰 → `send_command("fvp_produce", …)`
//!   · 서버: `handle_command` 에서 `team_get_json(merchandise_products / total_balance)` 읽고 계산 후 `team_set_json` 으로 반영
//!     - 추가 생산: 기존 레코드 stock += n, 비용 = n × unit_cost × (1 − 절감율)
//!     - 신규 생산: 우리 선수 × 없는 product_type 레코드 생성, 비용 = dev_cost + n × unit_cost × (1 − 절감율)
//!       (파생 필드 초기값 = 게임 ProduceMerchandise 핸들러 재현 — RE 결과 반영, `new_product()`)
//!   · 상품 정의(dev_cost/unit_cost/margin/demand) = data_setting.merchandise_product_setting 0.6.0 값 하드코딩(PRODUCTS)
//!   · 절감율 = 시설 화면 라벨 `merch_cost_reduction_value`("7%") 파싱, 날짜 = `game_date()` — 둘 다 클라가 payload 로 전달
//! 상태(수량) = mods\facility_view_plus\facility_view_plus.cfg (`count=1000`).
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, ClientSceneKindV1, CommandResultV1, LogLevel, RecordKindV1, StableClient, StableCommand, StableExtension, StableHost, StableMod, StableServerCtx, StableServerExtension};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::Mutex;
#[path = r"C:\tfm2mods\ui_kit\team_sync_stable.rs"]
mod team_sync;

const MOD_ID: &str = "facility_view_plus";
const DBG: bool = true;
const VIEW: &str = "main.top.right.facility";
const PANEL: &str = "main.top.right.facility.contents.merchandise";
const CMD: &str = "fvp_produce";
const EVT: &str = "fvp_result";
const BTN_ADD: &str = "fvp_btn_add";
const BTN_NEW: &str = "fvp_btn_new";
const POPUP: &str = "fvp_popup";
const TOAST: &str = "fvp_toast";
const COUNT_MIN: i64 = 100;
const COUNT_MAX: i64 = 100_000;

/// data_setting.merchandise_product_setting (0.6.0): (dev_cost, unit_cost, default_margin %, demand_pct)
const PRODUCTS: [(i64, i64, i64, i64); 10] = [
    (9_000_000, 8_000, 150, 60),    // 0 wrist_guard
    (9_000_000, 15_000, 130, 40),   // 1 glasses
    (15_000_000, 150_000, 80, 10),  // 2 gaming_chair
    (12_000_000, 30_000, 100, 30),  // 3 mouse
    (10_000_000, 5_000, 200, 80),   // 4 mousepad
    (10_000_000, 50_000, 80, 20),   // 5 keyboard
    (18_000_000, 10_000, 150, 100), // 6 lightstick
    (6_000_000, 2_000, 250, 100),   // 7 poster
    (8_000_000, 5_000, 200, 80),    // 8 calendar
    (15_000_000, 25_000, 100, 50),  // 9 uniform
];
const DAILY_RATE_PER_FAN: i64 = 20;
const TEAM_RATE_PER_FAN: i64 = 1;
const PRICE_ELASTICITY_X100: i64 = 200;

static FRAME: AtomicU64 = AtomicU64::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static CLICKS_REGISTERED: AtomicBool = AtomicBool::new(false);
static COUNT: AtomicI64 = AtomicI64::new(1000);
/// 팝업이 열려 있으면 Some(mode): "add" | "new"
static POPUP_MODE: Mutex<Option<&'static str>> = Mutex::new(None);
static PENDING_SEND: Mutex<Option<&'static str>> = Mutex::new(None);
static TOAST_UNTIL: AtomicU64 = AtomicU64::new(0);
static TOAST_TEXT: Mutex<Option<String>> = Mutex::new(None);

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
fn cfg_path() -> Option<String> { mod_dir().map(|d| format!(r"{}\facility_view_plus.cfg", d)) }
fn load_count() -> i64 {
    cfg_path().and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| t.lines().find_map(|l| l.trim().strip_prefix("count=").and_then(|v| v.trim().parse::<i64>().ok())))
        .unwrap_or(1000).clamp(COUNT_MIN, COUNT_MAX)
}
fn save_count(n: i64) { if let Some(p) = cfg_path() { let _ = std::fs::write(p, format!("count={}\n", n)); } }
fn fmt_won(v: f64) -> String {
    let a = v.abs();
    let s = if a >= 1e8 { format!("{:.2} 억원", a / 1e8) } else if a >= 1e4 { format!("{:.0} 만원", a / 1e4) } else { format!("{:.0} 원", a) };
    if v < 0.0 { format!("-{}", s) } else { s }
}

// ───────── 클라이언트 UI ─────────
fn btn_src(id: &str, x: i32, text: &str) -> String {
    format!("{}:color_icon_button {{ @\"asset/base/style/main#secondary_button\"; x: {}px; y: 701px; width: 162px; height: 40px; text: {{ text: \"{}\"; font: \"asset/base/font/set/bold\"; size: 17; align_x: Center; align_y: Center; }} }}", id, x, text)
}
fn popup_src() -> String {
    // facility 루트 기준 전체 덮개 + 중앙 박스. 게임 popup_layer 와 같은 기하(1920×1080, x:-296 y:-24).
    let mut s = format!("{}:empty {{ width: 1920px; height: 1080px; x: -296px; y: -24px; z: 1500; anchor_x: 0; pivot_x: 0; anchor_y: 0; pivot_y: 0; visible: false;\n", POPUP);
    s.push_str("  #fade:color_icon_button { width: 100%; height: 100%; btn: { color: #60708540; } }\n");
    s.push_str("  #box:color { width: 560px; height: 300px; anchor_x: 0.5; pivot_x: 0.5; anchor_y: 0.5; pivot_y: 0.5; color: #161721ff; rounding: Uniform { rounding: 12; }\n");
    s.push_str("    #title:label { @\"asset/base/style/main#bold_label\"; x: 24px; y: 20px; width: 500px; height: 28px; size: 18; align_y: Center; text: \"\"; }\n");
    s.push_str("    #desc:label { @\"asset/base/style/main#label\"; x: 24px; y: 60px; width: 512px; height: 60px; size: 14; color: #a8a8b3ff; text: \"\"; }\n");
    s.push_str("    #count_label:label { @\"asset/base/style/main#label\"; x: 24px; y: 140px; width: 120px; height: 36px; size: 16; align_y: Center; text: \"상품별 수량\"; }\n");
    for (i, (id, dx, t)) in [("m500", 150, "-500"), ("m100", 214, "-100"), ("p100", 366, "+100"), ("p500", 430, "+500")].iter().enumerate() {
        let _ = i;
        s.push_str(&format!("    #{}:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x: {}px; y: 140px; width: 58px; height: 36px; text: {{ text: \"{}\"; size: 14; align_x: Center; align_y: Center; }} }}\n", id, dx, t));
    }
    s.push_str("    #count:label { @\"asset/base/style/main#bold_label\"; x: 278px; y: 140px; width: 82px; height: 36px; size: 18; align_x: Center; align_y: Center; text: \"1000\"; }\n");
    s.push_str("    #cancel:color_icon_button { @\"asset/base/style/main#tertiary_button\"; x: 24px; y: 236px; width: 248px; height: 44px; text: { text: \"취소\"; size: 16; align_x: Center; align_y: Center; } }\n");
    s.push_str("    #confirm:color_icon_button { @\"asset/base/style/main#secondary_button\"; x: 288px; y: 236px; width: 248px; height: 44px; text: { text: \"확인\"; font: \"asset/base/font/set/bold\"; size: 16; align_x: Center; align_y: Center; } }\n");
    s.push_str("  }\n}");
    s
}
fn toast_src() -> String {
    format!("{}:label {{ @\"asset/base/style/main#bold_label\"; anchor_x: 1; pivot_x: 1; x: -200px; y: 701px; width: 700px; height: 40px; size: 15; color: #4dd999ff; align_x: Right; align_y: Center; visible: false; text: \"\"; }}", TOAST)
}
fn set_popup_mode(m: Option<&'static str>) { *POPUP_MODE.lock().unwrap_or_else(|e| e.into_inner()) = m; }
fn popup_mode() -> Option<&'static str> { *POPUP_MODE.lock().unwrap_or_else(|e| e.into_inner()) }
fn bump(d: i64) { let n = (COUNT.load(Ordering::Relaxed) + d).clamp(COUNT_MIN, COUNT_MAX); COUNT.store(n, Ordering::Relaxed); save_count(n); }

/// "7%" → 0.07
fn parse_pct(s: &str) -> Option<f64> {
    let t: String = s.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
    t.parse::<f64>().ok().map(|v| v / 100.0)
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f = FRAME.fetch_add(1, Ordering::Relaxed);
            // 서버 응답
            for ev in ctx.take_events() {
                if ev.event == EVT {
                    let msg = String::from_utf8_lossy(&ev.payload).to_string();
                    log(&format!("서버 응답: {}", msg));
                    *TOAST_TEXT.lock().unwrap_or_else(|e| e.into_inner()) = Some(msg);
                    TOAST_UNTIL.store(f + 60 * 6, Ordering::Relaxed);
                }
            }
            if f % 6 != 0 { return; }
            if ctx.client_scene_kind() != Some(ClientSceneKindV1::Main) { deactivate(); return; }
            if !ctx.ui_exists(PANEL) || !ctx.ui_visible(VIEW).unwrap_or(false) { deactivate(); return; }
            if !ACTIVE.swap(true, Ordering::Relaxed) { log("facility 뷰 활성"); }
            let (b_add, b_new, popup, toast) = (format!("{}.{}", PANEL, BTN_ADD), format!("{}.{}", PANEL, BTN_NEW), format!("{}.{}", VIEW, POPUP), format!("{}.{}", PANEL, TOAST));
            if !ctx.ui_exists(&b_add) {
                let ok1 = ctx.ui_spawn_source(PANEL, &btn_src(BTN_ADD, 20, "전체 추가 생산"));
                let ok2 = ctx.ui_spawn_source(PANEL, &btn_src(BTN_NEW, 192, "전체 신규 생산"));
                let ok3 = ctx.ui_spawn_source(VIEW, &popup_src());
                let ok4 = ctx.ui_spawn_source(PANEL, &toast_src());
                log(&format!("스폰 add={} new={} popup={} toast={} add_rect={:?} popup_rect={:?}", ok1, ok2, ok3, ok4, ctx.ui_node_rect(&b_add), ctx.ui_node_rect(&popup)));
                // ★핸들러는 경로 키 영구 등록 — 프로세스당 1회만(재등록 = 중복 발화)
                if !CLICKS_REGISTERED.swap(true, Ordering::Relaxed) {
                    ctx.ui_register_click(&b_add, "", |_| set_popup_mode(Some("add")));
                    ctx.ui_register_click(&b_new, "", |_| set_popup_mode(Some("new")));
                    let bx = format!("{}.box", popup);
                    ctx.ui_register_click(&format!("{}.fade", popup), "", |_| set_popup_mode(None));
                    ctx.ui_register_click(&format!("{}.cancel", bx), "", |_| set_popup_mode(None));
                    ctx.ui_register_click(&format!("{}.m500", bx), "", |_| bump(-500));
                    ctx.ui_register_click(&format!("{}.m100", bx), "", |_| bump(-100));
                    ctx.ui_register_click(&format!("{}.p100", bx), "", |_| bump(100));
                    ctx.ui_register_click(&format!("{}.p500", bx), "", |_| bump(500));
                    ctx.ui_register_click(&format!("{}.confirm", bx), "", |_| { let m = popup_mode(); *PENDING_SEND.lock().unwrap_or_else(|e| e.into_inner()) = m; set_popup_mode(None); });
                }
            }
            // 팝업 표시 동기화
            let mode = popup_mode();
            let vis = ctx.ui_visible(&popup).unwrap_or(false);
            if vis != mode.is_some() { ctx.ui_set_visible(&popup, mode.is_some()); }
            if let Some(m) = mode {
                let bx = format!("{}.box", popup);
                let n = COUNT.load(Ordering::Relaxed);
                let red = ctx.ui_text(&format!("{}.merch_cost_reduction_value", PANEL)).and_then(|t| parse_pct(&t)).unwrap_or(0.0);
                let (title, desc) = if m == "add" {
                    ("전체 추가 생산".to_string(), format!("기존 모든 상품의 재고를 상품별 {}개씩 보충합니다.\n비용 = 수량 × 원가 × (1 − 절감율 {:.0}%). 자금이 모자라면 거기서 멈춥니다.", n, red * 100.0))
                } else {
                    ("전체 신규 생산".to_string(), format!("우리 선수 전원 × 아직 없는 상품 종류를 {}개씩 생산합니다(판매가 기본).\n비용 = 개발비 + 수량 × 원가 × (1 − 절감율 {:.0}%). 자금이 모자라면 거기서 멈춥니다.", n, red * 100.0))
                };
                ctx.ui_set_text(&format!("{}.title", bx), &title);
                ctx.ui_set_text(&format!("{}.desc", bx), &desc);
                ctx.ui_set_text(&format!("{}.count", bx), &n.to_string());
            }
            // 전송
            let pending = PENDING_SEND.lock().unwrap_or_else(|e| e.into_inner()).take();
            if let Some(m) = pending {
                let n = COUNT.load(Ordering::Relaxed);
                let red = ctx.ui_text(&format!("{}.merch_cost_reduction_value", PANEL)).and_then(|t| parse_pct(&t)).unwrap_or(0.0);
                let (y, mo, d) = ctx.game_date().unwrap_or((2026, 1, 1));
                let team = ctx.player_team_id().unwrap_or(0);
                let payload = format!("{}\t{}\t{}\t{:04}-{:02}-{:02}\t{}", m, n, red, y, mo, d, team);
                let ok = ctx.send_command(CMD, payload.as_bytes());
                log(&format!("send_command {} {:?} → {}", CMD, payload, ok));
            }
            // 토스트
            if ctx.ui_exists(&toast) {
                let until = TOAST_UNTIL.load(Ordering::Relaxed);
                let show = f < until;
                if show { if let Some(t) = TOAST_TEXT.lock().unwrap_or_else(|e| e.into_inner()).as_ref() { ctx.ui_set_text(&toast, t); } }
                if ctx.ui_visible(&toast).unwrap_or(false) != show { ctx.ui_set_visible(&toast, show); }
            }
        }));
    }
}
fn deactivate() {
    if ACTIVE.swap(false, Ordering::Relaxed) { set_popup_mode(None); }
}

// ───────── 서버 ─────────
fn jf(v: &Value, k: &str) -> f64 { v.get(k).and_then(|x| x.as_f64()).unwrap_or(0.0) }
fn ji(v: &Value, k: &str) -> i64 { v.get(k).and_then(|x| x.as_i64()).or_else(|| v.get(k).and_then(|x| x.as_f64()).map(|f| f as i64)).unwrap_or(0) }
/// 우리 팀 소속 선수 id 목록: Athlete 레코드 contract JSON 안의 team_id 로 판정.
fn team_athletes(ctx: &StableServerCtx<'_>, team: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for id in ctx.record_ids(RecordKindV1::Athlete) {
        let Some(c) = ctx.athlete_get_json(id, "contract") else { continue };
        if let Ok(v) = serde_json::from_str::<Value>(&c) {
            // {"InContract":{"team_id":N,…}} / {"type":"InContract","team_id":N} / 평면 등 — team_id 키를 깊이 무관하게 찾는다
            fn find_team(v: &Value) -> Option<i64> {
                match v {
                    Value::Object(m) => { if let Some(t) = m.get("team_id").and_then(|t| t.as_i64()) { return Some(t); } m.values().find_map(find_team) }
                    Value::Array(a) => a.iter().find_map(find_team),
                    _ => None,
                }
            }
            if find_team(&v) == Some(team as i64) { out.push(id); }
        }
    }
    out
}
/// 신규 MerchandiseProduct 레코드 = 게임 ProduceMerchandise 재현(RE 2026-09-16, REPORT daram2_view_plus/RE/…ProduceMerchandise…):
///   eff = round(unit×(100−red)/100); anchor_sell = eff + margin×eff/100; anchor_rate = max(1, demand×base_player/100);
///   daily = round(anchor_rate×(anchor_sell/sell)^(e/100)) (sell==anchor 이면 = anchor_rate); base_player/base_team = 설정 원값(20/1, 팬 무관).
/// ⚠서버는 anchor_rate 를 **세이브 DB 의 product def** 로 계산한다(구세이브 demand_pct=0 → 1, 실측). 에셋값(60→12)과 다를 수 있어
///   같은 product_type 의 기존 레코드가 있으면 그 anchor_purchase_rate 를 복사(`anchor_hint`)해 game==mine 을 맞춘다.
fn new_product(athlete_id: usize, ptype: usize, n: i64, red: f64, date: &str, anchor_hint: Option<i64>) -> Value {
    let (_dev, unit, margin, demand) = PRODUCTS[ptype];
    let eff = (unit as f64 * (100.0 - red * 100.0) / 100.0).round() as i64;
    let anchor_sell = eff + margin * eff / 100;
    let anchor_rate = anchor_hint.unwrap_or((demand * DAILY_RATE_PER_FAN / 100).max(1));
    let sell = anchor_sell; // 판매가 기본 = anchor → daily == anchor_rate
    json!({
        "product_type": ptype, "stock": n, "sell_price": sell,
        "yearly_sales": 0, "yearly_revenue": 0, "total_sales": 0, "total_revenue": 0,
        "daily_purchase_rate": anchor_rate, "last_produced_date": date, "last_sold_date": Value::Null,
        "base_player_daily_purchase_rate": DAILY_RATE_PER_FAN, "athlete_id": athlete_id,
        "base_team_daily_purchase_rate": TEAM_RATE_PER_FAN, "price_elasticity_x100": PRICE_ELASTICITY_X100,
        "daily_sales_remainder": 0, "anchor_sell_price": anchor_sell, "anchor_purchase_rate": anchor_rate
    })
}
struct Srv;
impl StableServerExtension for Srv {
    fn handle_command(&self, ctx: &mut StableServerCtx<'_>, cmd: &StableCommand<'_>) -> CommandResultV1 {
        if cmd.command != CMD { return CommandResultV1::Pass; }
        let reply = cmd.reply_target();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| produce(ctx, cmd)));
        let msg = match r { Ok(m) => m, Err(_) => "오류: 서버 처리 중 패닉".to_string() };
        log(&format!("[server] {}", msg));
        ctx.emit_event(reply, EVT, msg.as_bytes());
        CommandResultV1::Handled
    }
}
fn produce(ctx: &mut StableServerCtx<'_>, cmd: &StableCommand<'_>) -> String {
    let p = String::from_utf8_lossy(cmd.payload).to_string();
    let parts: Vec<&str> = p.split('\t').collect();
    if parts.len() < 5 { return format!("오류: payload {:?}", p); }
    let (mode, n, red, date) = (parts[0], parts[1].parse::<i64>().unwrap_or(1000).clamp(COUNT_MIN, COUNT_MAX), parts[2].parse::<f64>().unwrap_or(0.0), parts[3]);
    let team = cmd.sender_team_id.or_else(|| parts[4].parse().ok()).unwrap_or(0);
    let cur = ctx.team_get_json(team, "merchandise_products").unwrap_or_else(|| "[]".into());
    let mut products: Vec<Value> = serde_json::from_str(&cur).unwrap_or_default();
    let mut balance = ctx.team_get_json(team, "total_balance").and_then(|s| s.trim().parse::<f64>().ok()).unwrap_or(0.0);
    let start_balance = balance;
    log(&format!("[server] {} team={} n={} red={} date={} products={} balance={:.0}", mode, team, n, red, date, products.len(), balance));
    let (mut done, mut stopped) = (0usize, false);
    if mode == "add" {
        for pr in products.iter_mut() {
            let pt = ji(pr, "product_type") as usize;
            let Some(&(_, unit, _, _)) = PRODUCTS.get(pt) else { continue };
            let cost = n as f64 * (unit as f64 * (100.0 - red * 100.0) / 100.0).round();
            if balance < cost { stopped = true; break; }
            balance -= cost;
            pr["stock"] = json!(ji(pr, "stock") + n);
            pr["last_produced_date"] = json!(date);
            done += 1;
        }
    } else {
        let athletes = team_athletes(ctx, team);
        log(&format!("[server] 우리 선수 {}명 {:?}", athletes.len(), athletes));
        if athletes.is_empty() {
            if let Some(id) = ctx.record_ids(RecordKindV1::Athlete).first() {
                let root = ctx.athlete_get_json(*id, "").unwrap_or_default();
                log(&format!("[server] 표본 Athlete {} root({}B): {}", id, root.len(), &root[..root.len().min(2500)]));
            }
            return "오류: 우리 팀 선수를 찾지 못함(contract.team_id)".into();
        }
        if DBG { if let Some(&aid) = athletes.first() { let root = ctx.athlete_get_json(aid, "").unwrap_or_default(); log(&format!("[server] 표본 Athlete {} root({}B): {}", aid, root.len(), &root[..root.len().min(2500)])); } }
        // 같은 product_type 의 게임 생성 레코드가 있으면 anchor_purchase_rate 힌트(세이브 DB product def 반영)
        let hints: Vec<Option<i64>> = (0..PRODUCTS.len()).map(|pt| products.iter().find(|pr| ji(pr, "product_type") as usize == pt).map(|pr| ji(pr, "anchor_purchase_rate"))).collect();
        'outer: for &aid in &athletes {
            for pt in 0..PRODUCTS.len() {
                if products.iter().any(|pr| ji(pr, "athlete_id") as usize == aid && ji(pr, "product_type") as usize == pt) { continue; }
                let (dev, unit, _, _) = PRODUCTS[pt];
                let eff = (unit as f64 * (100.0 - red * 100.0) / 100.0).round();
                let cost = dev as f64 + n as f64 * eff;
                if balance < cost { stopped = true; break 'outer; }
                balance -= cost;
                products.push(new_product(aid, pt, n, red, date, hints[pt]));
                done += 1;
            }
        }
    }
    if done == 0 { return if stopped { "자금 부족: 아무것도 생산하지 못함".into() } else { "생산할 대상이 없음".into() }; }
    let body = serde_json::to_string(&products).unwrap_or_default();
    let ok1 = ctx.team_set_json(team, "merchandise_products", &body);
    let ok2 = ctx.team_set_json(team, "total_balance", &format!("{}", balance));
    log(&format!("[server] set merchandise_products({}B)={} total_balance={} → {:.0}", body.len(), ok1, ok2, balance));
    if !ok1 || !ok2 { return format!("오류: 저장 거부 products={} balance={}", ok1, ok2); }
    // ★team_set_json 은 서버 DB 만 바꾸고 클라 복제본은 다음 게임 패킷/틱까지 갱신되지 않는다(0.6.0 실측: 잔고·표 그대로).
    //   뉴스 푸시가 팀 동기화를 유발하는지 시험(⬜) — 겸사겸사 생산 내역도 소식함에 남긴다.
    let title = format!("상품 {} 생산 {}건", if mode == "add" { "추가" } else { "신규" }, done);
    let content = format!("{} 상품별 {}개, 총 비용 {}{}", if mode == "add" { "기존 상품 재고 보충:" } else { "선수 전원 신규 상품:" }, n, fmt_won(start_balance - balance), if stopped { " (자금 부족으로 중단)" } else { "" });
    let nk = ctx.news_push(team, &title, &content, "facility_view_plus");
    log(&format!("[server] news_push={}", nk));
    // 클라 복제본 즉시 갱신: ResponseTeam 유니캐스트(네이티브 레시피, ui_kit/team_sync_stable.rs). 실패해도 데이터는 이미 서버에 반영됨.
    let synced = match team_sync::unicast_team(ctx, team) { Ok(m) => { log(&format!("[server] sync: {}", m)); true } Err(e) => { log(&format!("[server] sync 실패: {}", e)); false } };
    let sync_note = if synced { "" } else { " (화면 반영은 다음 진행 시)" };
    format!("{} {}건 생산 완료 · 비용 {}{}{}", if mode == "add" { "추가" } else { "신규" }, done, fmt_won(start_balance - balance), if stopped { " (자금 부족으로 중단)" } else { "" }, sync_note)
}

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "facility_view_plus (stable 0.6.0)");
    COUNT.store(load_count(), Ordering::Relaxed);
    let v = host.game_version();
    log(&format!("INIT game {}.{}.{} host_abi={} count={}", v.major, v.minor, v.patch, host.abi_level(), COUNT.load(Ordering::Relaxed)));
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d.set_server_extension(Srv);
    d
}
declare_stable_mod!(init);
