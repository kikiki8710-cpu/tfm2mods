//! tfm2_mod_order v0.2.0 — 모드 관리 목록 순서 변경 (키보드) · ★0.6.0 stable ABI 껍데기 판 (2026-09-18)
//! ===========================================================================
//! 목표: 타이틀 "모드 관리" 팝업(title.ui #mods_popup)의 모드 목록 표시 순서를 바꾼다.
//!   게임 기본순서 = mod_id(폴더명) ASCII 정렬(순서 저장 필드 없음 — 메모리 [[tfm2-mod-order-mod]] RE 정본).
//!
//! 방식(0.6.0 stable): 클래식은 행 노드 child Vec 을 재배치했지만 stable 엔 트리 재배치 API 가 없다.
//!   ⟹ 행 컨테이너(`…left_panel.table.contents.contents`, TopToBottom spacing 8 · 행 56px = pitch 64)의 각 행에
//!   `y: (원하는 인덱스 − 게임 인덱스) × pitch` 오프셋을 set_properties 로 줘서 **화면상 순서만** 바꾼다(게임 데이터·핸들러 무관).
//!   pitch 는 첫 두 행 rect 로 실측(오프셋 0 인 프레임) · 폴백 64.
//!   · 조작: 행 클릭 = 선택(행 배경 강조) · ↑/↓ = 선택 이동 · Ctrl+↑/↓ = 순서 이동(즉시 저장).
//!   · 한계: 선택이 화면 밖이면 자동 스크롤 없음(stable 엔 scroll_view 쓰기 API 없음) — 마우스 휠로 스크롤.
//! 파일(게임 exe 기준 동적 도출): <게임>\mods\tfm2_mod_order\mod_order.txt (행당 mod_id 1개, 위→아래 · 수동 편집 가능)
//! ===========================================================================
use mod_api_stable::{declare_stable_mod, LogLevel, SceneKindV1, StableClient, StableExtension, StableHost, StableMod};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

const MOD_ID: &str = "tfm2_mod_order";
const LOG_ENABLED: bool = false; // 진단 시 true(경로·pitch·덤프 로그)
const PITCH_FALLBACK: f32 = 64.0;
const REPEAT_DELAY_FRAMES: u64 = 45;
const REPEAT_RATE_FRAMES: u64 = 4;
const SEL_BG: &str = "#2f4a44ff";
const DEF_BG: &str = "#1d1f2cff";

#[link(name = "kernel32")]
extern "system" { fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32; }
#[link(name = "user32")]
extern "system" { fn GetAsyncKeyState(vk: i32) -> i16; }
const VK_CONTROL: i32 = 0x11; const VK_UP: i32 = 0x26; const VK_DOWN: i32 = 0x28;
fn key_down(vk: i32) -> bool { unsafe { (GetAsyncKeyState(vk) as u16) & 0x8000 != 0 } }
/// 눌림 상태 + "마지막 호출 이후 눌렸음"(0x0001) — 한 프레임 안에 끝나는 짧은 탭을 놓치지 않게.
fn key_tap_or_down(vk: i32) -> bool { let st = unsafe { GetAsyncKeyState(vk) as u16 }; st & 0x8000 != 0 || st & 0x0001 != 0 }

static FRAME: AtomicU64 = AtomicU64::new(0);
static POPUP_PATH: Mutex<Option<String>> = Mutex::new(None);
static CONT_PATH: Mutex<Option<String>> = Mutex::new(None);
static WAS_OPEN: AtomicBool = AtomicBool::new(false);
static ORDER: Mutex<Vec<String>> = Mutex::new(Vec::new());
static ORDER_LOADED: AtomicBool = AtomicBool::new(false);
static SELECTED: Mutex<Option<String>> = Mutex::new(None);
static CLICKS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LAST_CLICK: Mutex<Option<(u64, String)>> = Mutex::new(None);
static PROP_CACHE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
static PITCH: Mutex<Option<f32>> = Mutex::new(None);
static ROWS_SIG: AtomicU64 = AtomicU64::new(0);
static UP_REP: Mutex<(u64, bool)> = Mutex::new((0, false));
static DN_REP: Mutex<(u64, bool)> = Mutex::new((0, false));
static REG_OPEN: AtomicUsize = AtomicUsize::new(0);

fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\{}", dir.display(), MOD_ID))
}
fn log(msg: &str) {
    if !LOG_ENABLED { return; }
    if let Some(d) = mod_dir() {
        use std::io::Write;
        let _ = std::fs::create_dir_all(&d);
        let _ = std::fs::OpenOptions::new().append(true).create(true).open(format!("{d}\\mod_order_log.txt")).and_then(|mut f| writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), msg));
    }
}

fn load_order_from_file() -> Vec<String> {
    let Some(d) = mod_dir() else { return Vec::new() };
    let Ok(s) = std::fs::read_to_string(format!("{d}\\mod_order.txt")) else { return Vec::new() };
    s.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty() && !l.starts_with('#')).collect()
}
fn save_order_to_file(order: &[String]) {
    let Some(d) = mod_dir() else { return };
    let mut out = String::new();
    out.push_str("# tfm2_mod_order — 모드 목록 순서 (자동 저장됨)\n# 모드 관리에서 행 선택 후 Ctrl+↑/↓ 로 이동하면 이 파일이 갱신됩니다.\n# 직접 편집도 가능: 행당 mod_id(폴더명) 1개, 위→아래. #=주석.\n");
    for m in order { out.push_str(m); out.push('\n'); }
    let _ = std::fs::create_dir_all(&d);
    let _ = std::fs::write(format!("{d}\\mod_order.txt"), out);
}

fn set_prop(ctx: &mut StableClient<'_>, key: &str, path: &str, css: &str) {
    { let mut g = PROP_CACHE.lock().unwrap_or_else(|e| e.into_inner()); let m = g.get_or_insert_with(HashMap::new); if m.get(key).map(|s| s.as_str()) == Some(css) { return; } m.insert(key.to_string(), css.to_string()); }
    ctx.ui_set_properties(path, css);
}

/// 팝업·행 컨테이너 경로 탐색(루트 자식 × 후보). 캐시.
fn find_popup(ctx: &StableClient<'_>) -> Option<(String, String)> {
    if let (Some(p), Some(c)) = (POPUP_PATH.lock().unwrap_or_else(|e| e.into_inner()).clone(), CONT_PATH.lock().unwrap_or_else(|e| e.into_inner()).clone()) { if ctx.ui_exists(&p) { return Some((p, c)); } }
    let mut roots = ctx.ui_child_names("");
    roots.push(String::new());
    for r in roots {
        let p = if r.is_empty() { "mods_popup".to_string() } else { format!("{}.mods_popup", r) };
        if !ctx.ui_exists(&p) { continue; }
        let c = format!("{}.left_panel.table.contents.contents", p);
        let cont = if ctx.ui_exists(&c) { c } else {
            // 폴백: 팝업 아래 2~4단계에서 자식 이름이 전부 mod_ 인 노드
            let mut found: Option<String> = None;
            let mut stack: Vec<(String, usize)> = vec![(p.clone(), 0)];
            while let Some((n, d)) = stack.pop() { if d > 5 { continue; } let kids = ctx.ui_child_names(&n); if kids.len() >= 2 && kids.iter().all(|k| k.starts_with("mod_")) { found = Some(n); break; } for k in kids { stack.push((format!("{}.{}", n, k), d + 1)); } }
            match found { Some(f) => f, None => continue }
        };
        log(&format!("팝업 경로 = {} / 행 컨테이너 = {}", p, cont));
        *POPUP_PATH.lock().unwrap_or_else(|e| e.into_inner()) = Some(p.clone());
        *CONT_PATH.lock().unwrap_or_else(|e| e.into_inner()) = Some(cont.clone());
        return Some((p, cont));
    }
    None
}

/// 표시 순서 계산: 게임 순서 rows 를 ORDER 순위로 안정 정렬.
fn desired_order(rows: &[String]) -> Vec<String> {
    let order = ORDER.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let mut v: Vec<(usize, String)> = rows.iter().map(|r| (order.iter().position(|o| o == r).unwrap_or(usize::MAX), r.clone())).collect();
    v.sort_by_key(|(k, _)| *k);
    v.into_iter().map(|(_, r)| r).collect()
}

fn repeat_fire(pressed: bool, f: u64, st: &Mutex<(u64, bool)>) -> bool {
    let mut g = st.lock().unwrap_or_else(|e| e.into_inner());
    if !pressed { *g = (0, false); return false; }
    if !g.1 { *g = (f, true); return true; }
    let held = f.saturating_sub(g.0);
    held >= REPEAT_DELAY_FRAMES && (held - REPEAT_DELAY_FRAMES) % REPEAT_RATE_FRAMES == 0
}

fn tick(ctx: &mut StableClient<'_>) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);
    if ctx.scene_kind() != Some(SceneKindV1::Title) { if WAS_OPEN.swap(false, Ordering::Relaxed) { *PROP_CACHE.lock().unwrap_or_else(|e| e.into_inner()) = None; } return; }
    let Some((pop, cont)) = find_popup(ctx) else { return };
    let open = ctx.ui_visible(&pop) == Some(true);
    if !open { if WAS_OPEN.swap(false, Ordering::Relaxed) { *PROP_CACHE.lock().unwrap_or_else(|e| e.into_inner()) = None; *PITCH.lock().unwrap_or_else(|e| e.into_inner()) = None; ROWS_SIG.store(0, Ordering::Relaxed); } return; }
    let first_open = !WAS_OPEN.swap(true, Ordering::Relaxed);
    if first_open { *ORDER.lock().unwrap_or_else(|e| e.into_inner()) = load_order_from_file(); ORDER_LOADED.store(true, Ordering::Relaxed); }
    let rows: Vec<String> = ctx.ui_child_names(&cont).into_iter().filter(|k| k.starts_with("mod_")).collect();
    if rows.len() < 2 { return; }
    // 행 집합이 바뀌면(재스폰) 클릭 재등록 + 캐시 초기화 + pitch 재측정
    let sig = { use std::hash::{Hash, Hasher}; let mut h = std::collections::hash_map::DefaultHasher::new(); rows.hash(&mut h); h.finish() };
    if ROWS_SIG.swap(sig, Ordering::Relaxed) != sig {
        *PROP_CACHE.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *PITCH.lock().unwrap_or_else(|e| e.into_inner()) = None;
        for r in &rows { let rp = format!("{}.{}", cont, r); let id = r.trim_start_matches("mod_").to_string(); ctx.ui_register_click(&rp, "", move |_| CLICKS.lock().unwrap_or_else(|e| e.into_inner()).push(id.clone())); ctx.ui_set_properties(&rp, "y: 0px;"); }
        REG_OPEN.fetch_add(1, Ordering::Relaxed);
        log(&format!("행 {}개 등록 (컨테이너 {})", rows.len(), cont));
        return; // 오프셋 0 상태로 한 프레임 렌더 → 다음 프레임에 pitch 측정
    }
    // pitch 실측(오프셋 0 인 첫 프레임)
    if PITCH.lock().unwrap_or_else(|e| e.into_inner()).is_none() {
        let r0 = ctx.ui_node_rect(&format!("{}.{}", cont, rows[0])); let r1 = ctx.ui_node_rect(&format!("{}.{}", cont, rows[1]));
        let p = match (r0, r1) { (Some(a), Some(b)) if b.1 > a.1 + 1.0 => b.1 - a.1, _ => PITCH_FALLBACK };
        *PITCH.lock().unwrap_or_else(|e| e.into_inner()) = Some(p);
        log(&format!("pitch = {} (r0={:?} r1={:?})", p, r0, r1));
    }
    let pitch = PITCH.lock().unwrap_or_else(|e| e.into_inner()).unwrap_or(PITCH_FALLBACK);
    // 클릭 → 선택
    let clicks: Vec<String> = std::mem::take(&mut *CLICKS.lock().unwrap_or_else(|e| e.into_inner()));
    for c in clicks { { let mut lc = LAST_CLICK.lock().unwrap_or_else(|e| e.into_inner()); if lc.as_ref().map(|(ff, t)| *ff == f && *t == c).unwrap_or(false) { continue; } *lc = Some((f, c.clone())); } *SELECTED.lock().unwrap_or_else(|e| e.into_inner()) = Some(c.clone()); log(&format!("선택 {}", c)); }
    // 키 입력
    let ids: Vec<String> = rows.iter().map(|r| r.trim_start_matches("mod_").to_string()).collect();
    let mut disp = desired_order(&ids);
    let ctrl = key_down(VK_CONTROL);
    let up = repeat_fire(key_tap_or_down(VK_UP), f, &UP_REP); let dn = repeat_fire(key_tap_or_down(VK_DOWN), f, &DN_REP);
    if up || dn {
        let dir: i32 = if up { -1 } else { 1 };
        let sel = SELECTED.lock().unwrap_or_else(|e| e.into_inner()).clone();
        match sel {
            None => { *SELECTED.lock().unwrap_or_else(|e| e.into_inner()) = disp.first().cloned(); }
            Some(s) => if let Some(i) = disp.iter().position(|x| *x == s) {
                let t = i as i32 + dir;
                if t >= 0 && (t as usize) < disp.len() {
                    if ctrl { disp.swap(i, t as usize); save_order_to_file(&disp); *ORDER.lock().unwrap_or_else(|e| e.into_inner()) = disp.clone(); log(&format!("이동 {} {}->{} 저장", s, i, t)); }
                    else { *SELECTED.lock().unwrap_or_else(|e| e.into_inner()) = Some(disp[t as usize].clone()); }
                }
            },
        }
    }
    // 오프셋·강조 적용
    let sel = SELECTED.lock().unwrap_or_else(|e| e.into_inner()).clone();
    // ★실측(09-18, 77행 rect 덤프로 규명): TopToBottom 흐름에서 자식 `y` 의 규칙 = `pos_k = cursor_k + y_k` · `cursor_{k+1} = max(cursor_k, pos_k + pitch)`
    //   (커서는 "지금까지 가장 아래 끝" — 음수 y 로 위로 간 행은 커서를 되돌리지 않는다). ⟹ 목표 위치 target_k = desired_k×pitch 를 앞에서부터 시뮬레이션해
    //   y_k = target_k − cursor_k 로 주면 모든 행이 정확히 제자리에 앉는다(전체 높이 = N×pitch 불변).
    let mut cursor = 0.0f32;
    for (gi, id) in ids.iter().enumerate() {
        let rp = format!("{}.mod_{}", cont, id);
        let di = disp.iter().position(|x| x == id).unwrap_or(gi);
        let target = di as f32 * pitch;
        let off = target - cursor;
        cursor = cursor.max(target + pitch);
        set_prop(ctx, &format!("{}|y", rp), &rp, &format!("y: {}px;", off.round() as i32));
        let bg = if sel.as_deref() == Some(id.as_str()) { SEL_BG } else { DEF_BG };
        set_prop(ctx, &format!("{}|bg", rp), &rp, &format!("btn: {{ back_color: {bg}; }} hover: {{ btn: {{ back_color: {bg}; }} }}"));
    }
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tick(ctx)));
    }
}
fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "tfm2_mod_order v0.2.0 (stable 0.6.0)");
    log("init v0.2.0");
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d
}
declare_stable_mod!(init);
