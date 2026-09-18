//! tfm2_mod_order — 모드 관리 목록 순서 변경 (키보드 조작)
//! ===========================================================================
//! 목표: 타이틀 "모드 관리" 팝업(title.ui #mods_popup)의 모드 목록 순서를 바꾼다.
//!   게임 기본순서 = mod_id(폴더명) ASCII 정렬(게임이 collect 후 mod_id 오름차순 sort).
//!
//! 방식(순수 SDK — 게임함수 후킹/RVA 0개, 패치 무관):
//!   · 행 노드 트리(#mods_popup … 안쪽 #contents 의 자식들 #mod_<MOD_ID>)를 매 프레임
//!     원하는 순서로 child 재배치. (검증완: child Vec 재배치 = 화면 행 순서 즉시 반영.)
//!   · 조작: 행을 클릭해 선택(게임이 하이라이트) → Ctrl+↑ / Ctrl+↓ 로 선택 모드를 위/아래 이동.
//!     이동 시 현재 전체 표시순서를 스냅샷·스왑해 mod_order.txt 에 저장(영속) → 다음 프레임 반영.
//!   · 선택 감지: filter_handler(비소비)로 클릭 path 에서 "mod_<id>" 세그먼트 추출.
//!
//! 파일(게임 exe 기준 동적 도출 — 설치경로 하드코딩 금지):
//!   <게임>\mods\tfm2_mod_order\mod_order.txt        — 순서(행당 mod_id 1개, 위→아래). 자동 저장·수동편집 가능.
//!   <게임>\mods\tfm2_mod_order\mod_order_recon.txt  — 팝업 첫 오픈 시 행 구조 덤프(진단용).
//!   <게임>\mods\tfm2_mod_order\mod_order_log.txt     — 디버그 로그(LOG_ENABLED).
//! ===========================================================================
#![allow(dead_code, unused_imports)]
use mod_api::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[path = "C:/tfm2mods/ui_kit/ui_kit.rs"]
mod ui_kit;

const MOD_ID: &str = "tfm2_mod_order";
const LOG_ENABLED: bool = false; // 배포 OFF. 진단 시 true(로그 + 정찰덤프 활성).

// build_inj.ps1 신원검증(dll 안에 소스 절대경로 존재)용 — LOG OFF 로 정찰이 제거돼도
// 경로 문자열이 남도록 #[used] 로 강제 유지.
#[used]
static SRC_PATH: &str = file!();

// ── 상태 ──
static POPUP_OPEN: AtomicBool = AtomicBool::new(false);
static RECON_DONE: AtomicBool = AtomicBool::new(false);
static ORDER_LOADED: AtomicBool = AtomicBool::new(false);
static BOOT_LOGGED: AtomicBool = AtomicBool::new(false);
static FRAME: AtomicUsize = AtomicUsize::new(0);
static LAST_FH: AtomicUsize = AtomicUsize::new(usize::MAX);
// 선택 이동 후 스크롤 따라가기 요청(수동 스크롤과 안 싸우게 선택변경 시에만).
static SCROLL_TO_SEL: AtomicBool = AtomicBool::new(false);
// 화면에 보이는 행 수 추정(높이 못 읽을 때 폴백).
const VISIBLE_ROWS_FALLBACK: usize = 9;
// 키 홀드 자동반복: 첫 입력 1회 → REPEAT_DELAY 초 홀드 후 REPEAT_RATE 초 간격 반복.
const REPEAT_DELAY: f32 = 0.8; // 첫 반복까지 홀드 시간(초)
const REPEAT_RATE: f32 = 0.06; // 반복 간격(초) — 약간 빠르게
// 클릭 path 로깅(초반 몇 개만)
static PATH_LOG_LEFT: AtomicUsize = AtomicUsize::new(8);

thread_local! {
    // 우선순위 순서(= mod_order.txt 내용). 비면 게임 기본순서.
    static ORDER: RefCell<Vec<String>> = RefCell::new(Vec::new());
    // 현재 선택된 mod_id(클릭 또는 ↑/↓ 로 갱신).
    static SELECTED: RefCell<Option<String>> = RefCell::new(None);
    // 현재 하이라이트 중인 행(mod_id, 원래 이름색) — 선택 변경 시 원복용.
    static HIGHLIGHTED: RefCell<Option<(String, [f32; 4])>> = RefCell::new(None);
    // filter_handler 에 등록한 내 필터 Rc(존재확인용).
    static MY_FILTER: RefCell<Option<Rc<dyn Fn(&UIEvent) -> bool>>> = RefCell::new(None);
    // 키 자동반복 상태(held_time, next_fire_time, active) — ↑/↓ 각각.
    static UP_REPEAT: RefCell<(f32, f32, bool)> = RefCell::new((0.0, 0.0, false));
    static DOWN_REPEAT: RefCell<(f32, f32, bool)> = RefCell::new((0.0, 0.0, false));
}

/// 키 홀드 자동반복: 이번 프레임에 발화할 횟수. 첫 프레스=1, 홀드 시 지연 후 주기 발화.
fn repeat_fire(
    pressed: bool,
    dt: f32,
    key: &'static std::thread::LocalKey<RefCell<(f32, f32, bool)>>,
) -> u32 {
    key.with(|s| {
        let (held, next, active) = &mut *s.borrow_mut();
        if !pressed {
            *held = 0.0;
            *next = 0.0;
            *active = false;
            return 0;
        }
        if !*active {
            *active = true;
            *held = 0.0;
            *next = REPEAT_DELAY;
            return 1; // 첫 입력 즉시 1회
        }
        *held += dt.max(0.0);
        let mut fires = 0;
        while *held >= *next {
            fires += 1;
            *next += REPEAT_RATE;
            if fires > 8 {
                break; // dt 급증 시 폭주 방지
            }
        }
        fires
    })
}

// 하이라이트(선택 커서) 이름색 — 밝은 노랑, 게임 기본색과 뚜렷이 구분.
const ACCENT: ui_kit::Rgba = ui_kit::Rgba::new(1.0, 0.82, 0.24, 1.0);

// ── WinAPI ──
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
}
#[link(name = "user32")]
extern "system" {
    fn GetAsyncKeyState(vk: i32) -> i16;
}
const VK_CONTROL: i32 = 0x11;
const VK_UP: i32 = 0x26;
const VK_DOWN: i32 = 0x28;

fn key_down(vk: i32) -> bool {
    unsafe { (GetAsyncKeyState(vk) as u16) & 0x8000 != 0 }
}

fn mod_dir() -> Option<String> {
    let mut buf = [0u16; 512];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), 512) } as usize;
    if n == 0 || n >= 512 {
        return None;
    }
    let exe = String::from_utf16_lossy(&buf[..n]);
    let dir = std::path::Path::new(&exe).parent()?;
    Some(format!("{}\\mods\\{}", dir.display(), MOD_ID))
}

fn log(msg: &str) {
    if !LOG_ENABLED {
        return;
    }
    if let Some(d) = mod_dir() {
        use std::io::Write;
        let _ = std::fs::create_dir_all(&d);
        let _ = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{d}\\mod_order_log.txt"))
            .and_then(|mut f| writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), msg));
    }
}

// ===========================================================================
// 트리 헬퍼
// ===========================================================================
fn deep_first_label(n: &Node) -> Option<String> {
    if let Some(s) = ui_kit::label_get(n) {
        if !s.trim().is_empty() {
            return Some(s);
        }
    }
    for c in n.child.iter() {
        if let Some(s) = deep_first_label(c) {
            return Some(s);
        }
    }
    None
}

fn has_desc_id(n: &Node, targets: &[&str]) -> bool {
    if targets.iter().any(|t| n.id == *t) {
        return true;
    }
    n.child.iter().any(|c| has_desc_id(c, targets))
}
fn has_toggle_runner(n: &Node) -> bool {
    let t = n.runner.type_name();
    if t.contains("SelectableRunner") || t.contains("CheckboxRunner") || t.contains("ColorSelectableRunner") {
        return true;
    }
    n.child.iter().any(has_toggle_runner)
}
/// 모드 행 = 서브트리에 활성/비활성 토글을 가짐.
fn is_mod_row(n: &Node) -> bool {
    has_desc_id(n, &["enabled", "disabled"]) || has_toggle_runner(n)
}
/// 행의 표시이름(진단용).
fn row_key(n: &Node) -> String {
    deep_first_label(n).unwrap_or_default()
}
/// 행의 mod_id. 행 노드 id = "mod_<MOD_ID>"(정찰 확정) → 접두 제거.
fn row_mod_id(n: &Node) -> String {
    n.id.to_string().strip_prefix("mod_").map(|s| s.to_string()).unwrap_or_default()
}
/// 순위 = order 에서 mod_id 정확일치(부분일치 금지). 없으면 usize::MAX.
fn row_rank(order: &[String], n: &Node) -> usize {
    let mid = row_mod_id(n);
    if mid.is_empty() {
        return usize::MAX;
    }
    order.iter().position(|o| o == &mid).unwrap_or(usize::MAX)
}

/// 팝업 서브트리에서 행 컨테이너(직속 자식이 전부 모드 행, 2개 이상)를 찾아 &mut 로.
/// 발견 시 클로저에 넘겨 실행하고 그 반환을 돌려줌.
fn with_row_container<R>(popup: &mut Node, f: impl FnOnce(&mut Node) -> R) -> Option<R> {
    fn walk<R>(n: &mut Node, f: &mut Option<impl FnOnce(&mut Node) -> R>) -> Option<R> {
        let rows = n.child.iter().filter(|c| is_mod_row(c)).count();
        if rows >= 2 && rows == n.child.len() {
            if let Some(cb) = f.take() {
                return Some(cb(n));
            }
        }
        for c in n.child.iter_mut() {
            if f.is_some() {
                if let Some(r) = walk(c, f) {
                    return Some(r);
                }
            }
        }
        None
    }
    let mut slot = Some(f);
    walk(popup, &mut slot)
}

// ===========================================================================
// 순서 파일 I/O
// ===========================================================================
fn load_order_from_file() -> Vec<String> {
    let Some(d) = mod_dir() else { return Vec::new() };
    let Ok(s) = std::fs::read_to_string(format!("{d}\\mod_order.txt")) else {
        return Vec::new();
    };
    s.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

fn save_order_to_file(order: &[String]) {
    let Some(d) = mod_dir() else { return };
    let mut out = String::new();
    out.push_str("# tfm2_mod_order — 모드 목록 순서 (자동 저장됨)\n");
    out.push_str("# 모드 관리에서 행 선택 후 Ctrl+↑/↓ 로 이동하면 이 파일이 갱신됩니다.\n");
    out.push_str("# 직접 편집도 가능: 행당 mod_id(폴더명) 1개, 위→아래. #=주석.\n");
    for m in order {
        out.push_str(m);
        out.push('\n');
    }
    let _ = std::fs::create_dir_all(&d);
    let _ = std::fs::write(format!("{d}\\mod_order.txt"), out);
}

// ===========================================================================
// 재정렬 적용
// ===========================================================================
fn apply_reorder(popup: &mut Node) {
    ORDER.with(|o| {
        let order = o.borrow();
        if order.is_empty() {
            return;
        }
        with_row_container(popup, |cont| {
            let mut kids = std::mem::take(&mut cont.child);
            kids.sort_by_key(|r| row_rank(&order, r)); // 안정정렬: 미지정 행은 기본순 유지
            cont.child = kids;
        });
    });
}

/// 현재 화면 표시순서(행 컨테이너의 top→bottom mod_id 리스트).
fn current_display_order(popup: &mut Node) -> Vec<String> {
    with_row_container(popup, |cont| {
        cont.child
            .iter()
            .map(row_mod_id)
            .filter(|m| !m.is_empty())
            .collect::<Vec<_>>()
    })
    .unwrap_or_default()
}

/// 선택 모드를 dir(-1=위, +1=아래) 만큼 이동. 현재 전체순서 스냅샷→스왑→저장→ORDER 갱신.
fn move_selected(popup: &mut Node, dir: i32) {
    let sel = SELECTED.with(|s| s.borrow().clone());
    let Some(sel) = sel else {
        log("move: 선택된 모드 없음(행을 먼저 클릭)");
        return;
    };
    let mut order = current_display_order(popup);
    let Some(idx) = order.iter().position(|m| m == &sel) else {
        log(&format!("move: 선택 '{sel}' 를 표시목록에서 못 찾음"));
        return;
    };
    let n = order.len();
    let tgt = idx as i32 + dir;
    if tgt < 0 || tgt >= n as i32 {
        log(&format!("move: '{sel}' 이미 끝(idx={idx}, dir={dir})"));
        return;
    }
    order.swap(idx, tgt as usize);
    save_order_to_file(&order);
    ORDER.with(|o| *o.borrow_mut() = order);
    SCROLL_TO_SEL.store(true, Ordering::Relaxed);
    log(&format!("move '{sel}' {idx}->{tgt} (저장완)"));
}

// ===========================================================================
// 선택(하이라이트) 이동 + 렌더
// ===========================================================================
/// 행 이름 라벨의 현재 색(RGBA 4f32) 읽기.
fn read_name_color(root: &mut Node, mid: &str) -> Option<[f32; 4]> {
    let row = ui_kit::find_mut(root, &format!("mod_{mid}"))?;
    let name = ui_kit::find_mut(row, "name")?;
    let base = ui_kit::runner_base(name, "LabelRunner")?;
    Some([
        ui_kit::runner_rd_f32(base, ui_kit::off::LABEL_COLOR)?,
        ui_kit::runner_rd_f32(base, ui_kit::off::LABEL_COLOR + 4)?,
        ui_kit::runner_rd_f32(base, ui_kit::off::LABEL_COLOR + 8)?,
        ui_kit::runner_rd_f32(base, ui_kit::off::LABEL_COLOR + 12)?,
    ])
}
/// 행 이름 라벨 색 설정.
fn set_name_color(root: &mut Node, mid: &str, c: ui_kit::Rgba) {
    if let Some(row) = ui_kit::find_mut(root, &format!("mod_{mid}")) {
        if let Some(name) = ui_kit::find_mut(row, "name") {
            ui_kit::label_set_color(name, c);
        }
    }
}

/// ↑/↓(Ctrl 없이): 선택을 표시순서에서 dir(-1 위/+1 아래) 만큼 이동.
fn move_selection(popup: &mut Node, dir: i32) {
    let order = current_display_order(popup);
    if order.is_empty() {
        return;
    }
    let n = order.len() as i32;
    let cur = SELECTED.with(|s| s.borrow().clone());
    let new_idx = match cur.as_ref().and_then(|s| order.iter().position(|m| m == s)) {
        Some(i) => (i as i32 + dir).clamp(0, n - 1),
        None => {
            if dir > 0 {
                0
            } else {
                n - 1
            }
        }
    };
    let new_sel = order[new_idx as usize].clone();
    SELECTED.with(|s| *s.borrow_mut() = Some(new_sel));
    SCROLL_TO_SEL.store(true, Ordering::Relaxed);
}

/// 화면에 보이는 행 수 추정: 뷰포트 높이 / 행 높이. 못 읽으면 폴백.
fn visible_rows(popup: &Node) -> usize {
    let vp = ui_kit::find(popup, "contents").map(|n| ui_kit::node_layout_read(n, ui_kit::NODE_LF_H));
    let row_h = with_row_container_ref(popup, |cont| {
        cont.child.first().map(|r| ui_kit::node_layout_read(r, ui_kit::NODE_LF_H))
    })
    .flatten();
    match (vp, row_h) {
        // 뷰포트는 px(>200)로 보일 때만 신뢰(percent 이면 100 근처 → 폴백).
        (Some(vp), Some(rh)) if vp > 200.0 && rh > 20.0 && rh < 200.0 => {
            ((vp / rh).floor() as usize).max(1)
        }
        _ => VISIBLE_ROWS_FALLBACK,
    }
}

/// 불변 버전 행 컨테이너 접근(높이 읽기용).
fn with_row_container_ref<R>(popup: &Node, f: impl FnOnce(&Node) -> R) -> Option<R> {
    fn walk<R>(n: &Node, f: &mut Option<impl FnOnce(&Node) -> R>) -> Option<R> {
        let rows = n.child.iter().filter(|c| is_mod_row(c)).count();
        if rows >= 2 && rows == n.child.len() {
            if let Some(cb) = f.take() {
                return Some(cb(n));
            }
        }
        for c in n.child.iter() {
            if f.is_some() {
                if let Some(r) = walk(c, f) {
                    return Some(r);
                }
            }
        }
        None
    }
    let mut slot = Some(f);
    walk(popup, &mut slot)
}

/// 선택 행이 화면 밖이면 스크롤 ratio 조정해 보이게 한다(선택 변경 시 1회).
fn ensure_selected_visible(popup: &mut Node) {
    let sel = SELECTED.with(|s| s.borrow().clone());
    let Some(sel) = sel else { return };
    let order = current_display_order(popup);
    let Some(i) = order.iter().position(|m| m == &sel) else { return };
    let n = order.len();
    // 높이/행높이 계산은 부분적으로 잘리는 마지막 행까지 세어 실제보다 1 크다(실측:
    // 계산 11 vs 실제 10). 1행 여유를 빼 선택 행이 화면 안에 완전히 들어오게 한다.
    let v = visible_rows(popup).saturating_sub(1).max(1);
    let scroll_node = ui_kit::find_mut(popup, "contents");
    let Some(scroll_node) = scroll_node else { return };
    if n <= v {
        ui_kit::scroll_set(scroll_node, 0.0);
        return;
    }
    let denom = (n - v) as f32;
    let r = ui_kit::scroll_get(scroll_node).unwrap_or(0.0);
    let top = (r * denom).round() as i32;
    let vi = i as i32;
    let vlast = top + v as i32 - 1;
    let new_top = if vi < top {
        vi
    } else if vi > vlast {
        vi - v as i32 + 1
    } else {
        return; // 이미 보임
    };
    let nr = (new_top as f32 / denom).clamp(0.0, 1.0);
    ui_kit::scroll_set(scroll_node, nr);
    log(&format!("scroll follow: sel='{sel}' i={i}/{n} v={v} ratio {r:.3}->{nr:.3}"));
}

/// 선택된 행 이름을 ACCENT 색으로. 선택이 바뀌면 옛 행 원복 + 새 행 원색 스냅샷.
fn update_highlight(root: &mut Node) {
    let sel = SELECTED.with(|s| s.borrow().clone());
    HIGHLIGHTED.with(|h| {
        let cur = h.borrow().as_ref().map(|(m, _)| m.clone());
        if cur != sel {
            // 옛 하이라이트 원복
            if let Some((old_mid, old_col)) = h.borrow_mut().take() {
                set_name_color(root, &old_mid, ui_kit::Rgba::new(old_col[0], old_col[1], old_col[2], old_col[3]));
            }
            // 새 선택 원색 스냅샷
            if let Some(s) = &sel {
                if let Some(orig) = read_name_color(root, s) {
                    *h.borrow_mut() = Some((s.clone(), orig));
                }
            }
        }
        // 매 프레임 ACCENT 재적용(게임 리페인트 덮어쓰기 대비)
        if let Some(s) = &sel {
            set_name_color(root, s, ACCENT);
        }
    });
}

// ===========================================================================
// 클릭 → 선택 감지 (filter_handler, 비소비)
// ===========================================================================
fn ensure_click_select(ui: &mut GameUI) {
    MY_FILTER.with(|slot| {
        let mut slot = slot.borrow_mut();
        let present = slot
            .as_ref()
            .map_or(false, |mine| ui.filter_handler.iter().any(|(f, _)| Rc::ptr_eq(f, mine)));
        if present {
            return;
        }
        let filter: Rc<dyn Fn(&UIEvent) -> bool> = Rc::new(|e: &UIEvent| {
            if let UIEvent::Click { path, .. } = e {
                if POPUP_OPEN.load(Ordering::Relaxed) {
                    if PATH_LOG_LEFT.load(Ordering::Relaxed) > 0 {
                        PATH_LOG_LEFT.fetch_sub(1, Ordering::Relaxed);
                        log(&format!("CLICK path='{path}'"));
                    }
                    // path 의 '.' 세그먼트 중 "mod_" 로 시작하는 것 → 선택 mod_id.
                    if let Some(mid) = path
                        .split('.')
                        .find_map(|seg| seg.strip_prefix("mod_"))
                    {
                        if !mid.is_empty() {
                            SELECTED.with(|s| *s.borrow_mut() = Some(mid.to_string()));
                        }
                    }
                }
            }
            false // 절대 소비하지 않음(게임 선택/토글 정상 동작)
        });
        let handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = Rc::new(|_| {});
        ui.filter_handler.push((filter.clone(), handler));
        *slot = Some(filter);
        log(&format!("click filter (re)registered (fh len {})", ui.filter_handler.len()));
    });
}

// ===========================================================================
// 정찰 덤프 (구조 진단)
// ===========================================================================
fn dump_node(n: &Node, depth: usize, out: &mut String, max_depth: usize) {
    let indent = "  ".repeat(depth);
    let txt = ui_kit::label_get(n)
        .map(|s| {
            let s: String = s.replace('\n', "\\n").chars().take(40).collect();
            format!("  txt=\"{s}\"")
        })
        .unwrap_or_default();
    let vis = if n.visible { "" } else { " (hidden)" };
    out.push_str(&format!(
        "{indent}#{} <{}> children={}{}{}\n",
        n.id,
        ui_kit::kind(n),
        n.child.len(),
        txt,
        vis
    ));
    if depth >= max_depth {
        return;
    }
    for c in n.child.iter() {
        dump_node(c, depth + 1, out, max_depth);
    }
}
fn do_recon(popup: &Node) {
    let mut out = String::new();
    out.push_str("=== tfm2_mod_order recon — mods_popup subtree ===\n");
    out.push_str(&format!("src: {}\n\n", file!())); // 신원검증(소스경로 임베드)
    dump_node(popup, 0, &mut out, 8);
    if let Some(d) = mod_dir() {
        let _ = std::fs::create_dir_all(&d);
        let _ = std::fs::write(format!("{d}\\mod_order_recon.txt"), &out);
    }
}

// ===========================================================================
// 프레임 틱
// ===========================================================================
fn tick(ui: &mut GameUI, dt: f32) {
    FRAME.fetch_add(1, Ordering::Relaxed);
    if !BOOT_LOGGED.swap(true, Ordering::Relaxed) {
        log("mod alive — first post_update");
    }

    let visible = ui_kit::find(&ui.root, "mods_popup").map(|p| p.visible).unwrap_or(false);
    if !visible {
        if POPUP_OPEN.swap(false, Ordering::Relaxed) {
            log("popup gone/hidden");
            RECON_DONE.store(false, Ordering::Relaxed);
            ORDER_LOADED.store(false, Ordering::Relaxed);
            SELECTED.with(|s| *s.borrow_mut() = None);
        }
        return;
    }
    if !POPUP_OPEN.swap(true, Ordering::Relaxed) {
        log("popup visible");
    }

    // 팝업 오픈당 1회: 순서 파일 로드 + 정찰
    if !ORDER_LOADED.swap(true, Ordering::Relaxed) {
        let ord = load_order_from_file();
        log(&format!("order loaded: {} entries", ord.len()));
        ORDER.with(|o| *o.borrow_mut() = ord);
    }
    if LOG_ENABLED && !RECON_DONE.swap(true, Ordering::Relaxed) {
        if let Some(p) = ui_kit::find(&ui.root, "mods_popup") {
            do_recon(p);
        }
    }

    // 클릭→선택 필터 등록
    ensure_click_select(ui);

    // 키보드: ↑/↓ 자동반복(홀드 시 지연 후 연속). Ctrl 여부로 분기.
    //   Ctrl 없이 = 선택(하이라이트) 이동 / Ctrl = 순서 이동. Ctrl 은 발화 시점에 판정.
    let ctrl = key_down(VK_CONTROL);
    let up_fires = repeat_fire(key_down(VK_UP), dt, &UP_REPEAT);
    let down_fires = repeat_fire(key_down(VK_DOWN), dt, &DOWN_REPEAT);
    for _ in 0..up_fires {
        if let Some(p) = ui_kit::find_mut(&mut ui.root, "mods_popup") {
            if ctrl {
                move_selected(p, -1);
            } else {
                move_selection(p, -1);
            }
        }
    }
    for _ in 0..down_fires {
        if let Some(p) = ui_kit::find_mut(&mut ui.root, "mods_popup") {
            if ctrl {
                move_selected(p, 1);
            } else {
                move_selection(p, 1);
            }
        }
    }

    // 재정렬 적용(매 프레임 — 게임 리빌드에 되돌려져도 재적용)
    if let Some(p) = ui_kit::find_mut(&mut ui.root, "mods_popup") {
        apply_reorder(p);
    }
    // 선택 하이라이트 렌더
    update_highlight(&mut ui.root);
    // 선택 이동 시 스크롤 따라가기(수동 스크롤과 안 싸우게 변경 시 1회)
    if SCROLL_TO_SEL.swap(false, Ordering::Relaxed) {
        if let Some(p) = ui_kit::find_mut(&mut ui.root, "mods_popup") {
            ensure_selected_visible(p);
        }
    }
}

// ===========================================================================
// 모드 등록
// ===========================================================================
struct ModOrderExt;
impl ModExtension for ModOrderExt {
    fn post_update(&self, _scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, dt: f32) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            tick(ui, dt);
        }));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ModOrderExt);
    reg
}
declare_mod!(init);
