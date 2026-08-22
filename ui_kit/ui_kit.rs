// ui_kit.rs — TFM2 UI 위젯 조작 헬퍼 (함수처럼 쓰는 UI 툴킷)
// ===========================================================================
// 목적: 매번 메모리 덤프로 오프셋 찾던 짓을 끝내고, 검증된 오프셋을 한 곳에 모아
//       label/color/toggle/slider/textedit 를 함수 한 줄로 읽고 쓴다.
//
// 사용:  다른 모드의 lib.rs 에서  `mod ui_kit; use ui_kit::*;`  로 포함.
//        (또는 내용을 복붙.) 의존성 없음 — mod_api + std 만 사용. 기본 build_mod.bat 로 빌드.
//
// 오프셋 출처: discovered-ui-runner-offsets.md (engine_ui rlib 에서 offset_of! 로 컴파일타임 확정).
//        text@352 등이 legit 문서 실측과 일치 → rlib 레이아웃 = 실게임 레이아웃.
//
// ⚠️ 주의(문서 §3): selected/색만 바꾸면 시각은 갱신되나 게임 내부 onclick 연동은 안 탐.
//        → "모드 자기 UI"(.ui override 로 깐 노드)에 쓰는 게 정석. 게임 기존 위젯 조작 아님.
//        게임 업데이트로 레이아웃 어긋날 수 있음 → set_* 는 type_name 게이트 + (텍스트는) 내장.
//
// ★★유지보수 규칙 — 이 파일은 "UI 지식의 코드 정본"이다.
//   다른 모드 작업 중 새 UI 사실(러너 오프셋/Node 레이아웃/.ui 문법/엔진 동작·버그)을 알아내면
//   그 모드 안에만 두지 말고 **여기에 헬퍼·상수·주석으로 반영**할 것. 안 그러면 다음 모드가
//   같은 함정을 다시 밟는다(실제 사례: "인터랙티브 노드 hover 크기 튐" = 4상태 블록 미인지로
//   수개월 우회, 2026-07-08 규명). 문서(아틀라스/메모리)는 지식을, 이 파일은 **실행 가능한 형태**를 담는다.
//   현재 사용 모드 7개: Spectator_Chat / tfm2_ai_adjust / tfm2_draft_overlay / tfm2_scrim /
//                       tfm2_item_editor / tfm2_fog / tfm2_mod_scroll_fix  → 변경 시 전부 재빌드 확인.
//   ⚠ #[path] 로 소스째 포함되므로 여기서 깨지면 7개 모드가 동시에 깨진다. pub 시그니처 변경 금지(추가는 자유).
// ===========================================================================
#![allow(dead_code)]

use mod_api::*;
use std::any::Any;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};

// --------------------------- 오프셋 상수 (확정) -----------------------------
pub mod off {
    // LabelRunner (496)
    pub const LABEL_TEXT: usize = 352;          // String
    pub const LABEL_COLOR: usize = 40;          // Color (style.normal.color)
    pub const LABEL_SIZE: usize = 72;           // f32  (style.normal.size)
    // ColorRunner (412)
    pub const COLOR_BACK: usize = 20;           // Color (style.normal.back_color)
    pub const COLOR_MAIN: usize = 80;           // Color (style.normal.color = 테두리/주색)
    pub const COLOR_STROKE: usize = 96;         // f32
    pub const COLOR_STATE_STRIDE: usize = 100;  // normal0 / hover100 / active200 / disabled300
    // ImageRunner (848)
    pub const IMAGE_SOURCE: usize = 0;          // ⚠️ 비표준 레이아웃 가능 — 런타임 확인
    // CheckboxRunner (480)
    pub const CHECKBOX_SELECTED: usize = 472;   // bool
    pub const CHECKBOX_TEXT: usize = 448;       // String
    // SelectableRunner (2472)
    pub const SEL_SELECTED: usize = 2464;       // bool
    pub const SEL_TEXT: usize = 2368;           // String
    // ColorSelectableRunner (4952)
    pub const CSEL_SELECTED: usize = 4944;      // bool
    pub const CSEL_TEXT: usize = 4032;          // String
    // SliderRunner (1712)
    pub const SLIDER_RATIO: usize = 1696;       // f32 0..1
    // TextEditRunner (808)
    pub const TEXTEDIT_TEXT: usize = 656;       // String
    // ScrollViewRunner (1760)
    pub const SCROLL_RATIO: usize = 1740;       // f32
}

// ------------------------------ Color (RGBA f32) ----------------------------
/// engine_ui 의 common::color::Color 와 동일 레이아웃: { r,g,b,a: f32 }, 각 0.0~1.0
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Rgba { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }

impl Rgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self { Rgba { r, g, b, a } }
    /// 0xRRGGBBAA (예: 0x37d5b3ff)
    pub fn hex(v: u32) -> Self {
        Rgba {
            r: ((v >> 24) & 0xff) as f32 / 255.0,
            g: ((v >> 16) & 0xff) as f32 / 255.0,
            b: ((v >> 8) & 0xff) as f32 / 255.0,
            a: (v & 0xff) as f32 / 255.0,
        }
    }
    pub fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 }
    }
}

// ----------------------------- 안전 메모리 가드 -----------------------------
#[link(name = "kernel32")]
extern "system" {
    fn IsBadReadPtr(lp: usize, ucb: usize) -> i32;
    fn GetModuleHandleW(name: *const u16) -> usize; // 게임 모듈 base (NativeDropdown 용)
}

#[inline]
fn readable(p: usize, n: usize) -> bool {
    p >= 0x10000 && (p as u64) < 0x0000_8000_0000_0000 && unsafe { IsBadReadPtr(p, n) } == 0
}

/// runner 인스턴스 데이터 포인터(= 오프셋 0). type_name 이 `want` 포함할 때만 Some.
unsafe fn base_of(node: &Node, want: &str) -> Option<*mut u8> {
    if !node.runner.type_name().contains(want) { return None; }
    let any: &dyn Any = node.runner.as_any();
    let parts: [usize; 2] = std::mem::transmute::<*const dyn Any, [usize; 2]>(any as *const dyn Any);
    let base = parts[0];
    if base < 0x10000 { return None; }
    Some(base as *mut u8)
}

#[inline]
unsafe fn wr_f32(base: *mut u8, off: usize, v: f32) { *(base.add(off) as *mut f32) = v; }
#[inline]
unsafe fn rd_f32(base: *mut u8, off: usize) -> f32 { *(base.add(off) as *const f32) }
#[inline]
unsafe fn wr_bool(base: *mut u8, off: usize, v: bool) { *(base.add(off) as *mut u8) = v as u8; }
#[inline]
unsafe fn rd_bool(base: *mut u8, off: usize) -> bool { *(base.add(off) as *const u8) != 0 }
#[inline]
unsafe fn wr_color(base: *mut u8, off: usize, c: Rgba) { *(base.add(off) as *mut Rgba) = c; }

/// String 정식 제자리 교체(기존 drop + 이동). 레이아웃 일치 시에만 안전.
unsafe fn wr_string(base: *mut u8, off: usize, s: &str) { *(base.add(off) as *mut String) = s.to_string(); }
unsafe fn rd_string(base: *mut u8, off: usize) -> Option<String> {
    let p = base.add(off) as *const String;
    if !readable(p as usize, 24) { return None; }
    Some((*p).clone())
}

// =========================== 트리 탐색 헬퍼 ================================
pub fn id_str(n: &Node) -> String { n.id.to_string() }

/// 트리에서 id == target 인 첫 노드(불변).
pub fn find<'a>(n: &'a Node, target: &str) -> Option<&'a Node> {
    if n.id == target { return Some(n); }
    for c in n.child.iter() { if let Some(f) = find(c, target) { return Some(f); } }
    None
}
/// 트리에서 id == target 인 첫 노드(가변).
pub fn find_mut<'a>(n: &'a mut Node, target: &str) -> Option<&'a mut Node> {
    if n.id == target { return Some(n); }
    for c in n.child.iter_mut() { if let Some(f) = find_mut(c, target) { return Some(f); } }
    None
}

// =========================== VISIBILITY (동적 UI의 핵심) ===================
// ★ 규칙(검증됨): 코드로 트리에 push 한 노드는 *애초에 렌더되지 않는다*.
//   (게임은 .ui 에서 로드한 노드만 그림. "매 프레임 되돌림"이 아니라 처음부터 무시.)
//   → 동적 UI = 필요한 노드를 .ui override 에 *전부 미리 선언* + 코드는 visible/내용만 토글.
//   이 함수들이 그 visible 토글 경로다. (드롭다운 펼침/접힘도 이걸로.)
pub fn set_visible(n: &mut Node, on: bool) { n.visible = on; }
pub fn show(n: &mut Node) { n.visible = true; }
pub fn hide(n: &mut Node) { n.visible = false; }
pub fn get_visible(n: &Node) -> bool { n.visible }
/// root 에서 id 찾아 visible 설정. 찾으면 true.
pub fn set_visible_by_id(root: &mut Node, id: &str, on: bool) -> bool {
    match find_mut(root, id) { Some(n) => { n.visible = on; true } None => false }
}

/// runner 종류 짧은 이름.
pub fn kind(n: &Node) -> &'static str {
    let t = n.runner.type_name();
    for k in [
        "LabelRunner","ColorRunner","ImageRunner","ButtonRunner","DropdownRunner",
        "CheckboxRunner","SelectableRunner","ColorSelectableRunner","SliderRunner",
        "TextEditRunner","ScrollViewRunner","TreeViewRunner","AnimationRunner","SvgRunner","CanvasRunner",
    ] { if t.contains(k) { return k; } }
    "?"
}

// =========================== RUNNER 진단 (read-only) ======================
/// 노드 runner의 게임 타입명(예: "...DraggablePopupRunner").
pub fn runner_type_name(n: &Node) -> String { n.runner.type_name().to_string() }
/// runner 인스턴스 데이터 포인터(want 포함 시). 진단/필드접근용 pub 래퍼.
pub fn runner_base(n: &Node, want: &str) -> Option<usize> { unsafe { base_of(n, want).map(|p| p as usize) } }
/// runner 필드 f32 안전 read(범위가드).
pub fn runner_rd_f32(base: usize, off: usize) -> Option<f32> {
    if base == 0 || !readable(base + off, 4) { return None; }
    unsafe { Some(rd_f32(base as *mut u8, off)) }
}
/// runner 필드 u8 안전 read(범위가드).
pub fn runner_rd_u8(base: usize, off: usize) -> Option<u8> {
    if base == 0 || !readable(base + off, 1) { return None; }
    unsafe { Some(*((base + off) as *const u8)) }
}
/// runner 필드 f32 안전 write(범위가드 — read 가능 = 매핑됨 확인 후). 성공 시 true.
pub fn runner_wr_f32(base: usize, off: usize, v: f32) -> bool {
    if base == 0 || !readable(base + off, 4) { return false; }
    unsafe { wr_f32(base as *mut u8, off, v); }
    true
}

// =========================== LABEL ========================================
pub fn label_get(n: &Node) -> Option<String> {
    unsafe { base_of(n, "LabelRunner").and_then(|b| rd_string(b, off::LABEL_TEXT)) }
}
/// 라벨 텍스트 교체. (set 전 read 로 레이아웃 검증)
pub fn label_set(n: &mut Node, text: &str) -> bool {
    unsafe {
        let Some(b) = base_of(n, "LabelRunner") else { return false; };
        if rd_string(b, off::LABEL_TEXT).is_none() { return false; } // 레이아웃 깨짐 → 스킵
        wr_string(b, off::LABEL_TEXT, text);
        true
    }
}
pub fn label_set_color(n: &mut Node, c: Rgba) -> bool {
    unsafe { match base_of(n, "LabelRunner") { Some(b) => { wr_color(b, off::LABEL_COLOR, c); true } None => false } }
}
pub fn label_set_size(n: &mut Node, size: f32) -> bool {
    unsafe { match base_of(n, "LabelRunner") { Some(b) => { wr_f32(b, off::LABEL_SIZE, size); true } None => false } }
}

// =========================== COLOR (사각형/배경) ============================
/// 배경색(back_color). 패널/버튼 배경 채움.
pub fn rect_set_back(n: &mut Node, c: Rgba) -> bool {
    unsafe { match base_of(n, "ColorRunner") { Some(b) => { wr_color(b, off::COLOR_BACK, c); true } None => false } }
}
/// 주색/테두리색(color).
pub fn rect_set_color(n: &mut Node, c: Rgba) -> bool {
    unsafe { match base_of(n, "ColorRunner") { Some(b) => { wr_color(b, off::COLOR_MAIN, c); true } None => false } }
}
/// 모든 상태(normal/hover/active/disabled)에 동일 배경색 강제(상태 무관 고정용).
pub fn rect_set_back_all(n: &mut Node, c: Rgba) -> bool {
    unsafe {
        let Some(b) = base_of(n, "ColorRunner") else { return false; };
        for s in 0..4 { wr_color(b, off::COLOR_BACK + s * off::COLOR_STATE_STRIDE, c); }
        true
    }
}

// =========================== TOGGLE (통합) =================================
// Checkbox / Selectable / ColorSelectable 의 `selected` bool 을 종류 자동판별로 처리.
// ★세그먼트 탭(n개 간격없이 나열·1개만 활성 — 스케줄 리스트/달력·전술 전환·훈련 탭 상단 패턴):
//   .ui = 래퍼 color(stroke:1, rounding:8, padding:4) + child_type:LeftToRight{spacing:0px}
//        + n개 color_selectable(@"asset/base/style/main#strategy_option", label/selected_label).
//   상호배타는 엔진이 안 해줌(러너는 자기 selected bool뿐) → 모드 UI는 ensure_clicks 라우팅 +
//   매 프레임 toggle_set_by_id(클릭한 것 true, 나머지 false)로 강제. write만으로 시각 갱신됨.
//   ⚠게임 '기존' 탭은 selected write해도 화면 전환 로직 안 탐(시각만). 상세=ANA\UI-모딩-종합가이드.md §3.
fn toggle_loc(n: &Node) -> Option<(*mut u8, usize)> {
    unsafe {
        if let Some(b) = base_of(n, "ColorSelectableRunner") { return Some((b, off::CSEL_SELECTED)); }
        if let Some(b) = base_of(n, "SelectableRunner")      { return Some((b, off::SEL_SELECTED)); }
        if let Some(b) = base_of(n, "CheckboxRunner")        { return Some((b, off::CHECKBOX_SELECTED)); }
        None
    }
}
pub fn toggle_get(n: &Node) -> Option<bool> {
    toggle_loc(n).map(|(b, o)| unsafe { rd_bool(b, o) })
}
pub fn toggle_set(n: &mut Node, on: bool) -> bool {
    match toggle_loc(n) { Some((b, o)) => { unsafe { wr_bool(b, o, on); } true } None => false }
}
pub fn toggle_flip(n: &mut Node) -> Option<bool> {
    let cur = toggle_get(n)?; toggle_set(n, !cur); Some(!cur)
}
/// id 로 찾아 selected(bool) 설정 (Checkbox/Selectable/ColorSelectable). 찾으면 true.
pub fn toggle_set_by_id(root: &mut Node, id: &str, on: bool) -> bool {
    match find_mut(root, id) { Some(n) => toggle_set(n, on), None => false }
}
/// 토글류의 라벨 텍스트(있으면).
pub fn toggle_text_set(n: &mut Node, text: &str) -> bool {
    unsafe {
        if let Some(b) = base_of(n, "ColorSelectableRunner") { if rd_string(b, off::CSEL_TEXT).is_some() { wr_string(b, off::CSEL_TEXT, text); return true; } }
        if let Some(b) = base_of(n, "SelectableRunner")      { if rd_string(b, off::SEL_TEXT).is_some()  { wr_string(b, off::SEL_TEXT, text);  return true; } }
        if let Some(b) = base_of(n, "CheckboxRunner")        { if rd_string(b, off::CHECKBOX_TEXT).is_some(){ wr_string(b, off::CHECKBOX_TEXT, text); return true; } }
        false
    }
}

// =========================== SLIDER / SCROLL ===============================
pub fn slider_get(n: &Node) -> Option<f32> {
    unsafe { base_of(n, "SliderRunner").map(|b| rd_f32(b, off::SLIDER_RATIO)) }
}
pub fn slider_set(n: &mut Node, ratio01: f32) -> bool {
    unsafe { match base_of(n, "SliderRunner") { Some(b) => { wr_f32(b, off::SLIDER_RATIO, ratio01.clamp(0.0, 1.0)); true } None => false } }
}
pub fn scroll_get(n: &Node) -> Option<f32> {
    unsafe { base_of(n, "ScrollViewRunner").map(|b| rd_f32(b, off::SCROLL_RATIO)) }
}
/// 스크롤 위치 설정(0..1). ⚠️ 0=맨위 가정 — 런타임 확인 권장. (네이티브 휠 스크롤은 자동.)
pub fn scroll_set(n: &mut Node, ratio01: f32) -> bool {
    unsafe { match base_of(n, "ScrollViewRunner") { Some(b) => { wr_f32(b, off::SCROLL_RATIO, ratio01.clamp(0.0, 1.0)); true } None => false } }
}
pub fn scroll_set_by_id(root: &mut Node, id: &str, ratio01: f32) -> bool {
    match find_mut(root, id) { Some(n) => scroll_set(n, ratio01), None => false }
}

// =========================== TEXT EDIT =====================================
pub fn textedit_get(n: &Node) -> Option<String> {
    unsafe { base_of(n, "TextEditRunner").and_then(|b| rd_string(b, off::TEXTEDIT_TEXT)) }
}
pub fn textedit_set(n: &mut Node, text: &str) -> bool {
    unsafe {
        let Some(b) = base_of(n, "TextEditRunner") else { return false; };
        if rd_string(b, off::TEXTEDIT_TEXT).is_none() { return false; }
        wr_string(b, off::TEXTEDIT_TEXT, text); true
    }
}

// =========================== DROPDOWN (제한) ===============================
// DropdownRunner 의 selected/items 는 private → 직접 못 씀(문서 §2).
// 표시중인 선택값은 자식 LabelRunner 텍스트로 읽는다.
pub fn dropdown_selected_text(dropdown_node: &Node) -> Option<String> {
    fn first_label(n: &Node) -> Option<String> {
        if let Some(s) = label_get(n) { if !s.is_empty() { return Some(s); } }
        for c in n.child.iter() { if let Some(s) = first_label(c) { return Some(s); } }
        None
    }
    first_label(dropdown_node)
}

// =========================== CLICK (filter_handler) ========================
// 입력 경로: 게임은 클릭을 `ui.filter_handler: Vec<(filter, handler)>` 로 흘림.
//   filter:  Rc<dyn Fn(&UIEvent) -> bool>   // true 반환 = 이벤트 소비(게임 기본동작 차단)
//   handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)>
//   UIEvent::Click { path, item }           // path = 점(.) 구분 노드 경로
// ★ scene 전환 시 게임이 filter_handler 를 통째로 갈아끼움 → len 줄면 재등록(LAST_FH 비교).
//   (Spectator_Chat 의 ensure_chat_toggle_handler 패턴을 일반화.)
//
// 사용: post_update 매 프레임 호출. routes 의 (경로suffix, 콜백). 클릭이 suffix 로 끝나면
//   콜백 실행 + 이벤트 소비. 콜백은 보통 AtomicBool/Usize 상태를 바꾸고,
//   그 상태를 같은 post_update 뒷부분에서 set_visible/toggle_set 로 반영한다.
pub type ClickFn = Rc<dyn Fn()>;

/// 클릭 라우트 재등록(필요 시). last 는 모드가 소유한 static AtomicUsize.
pub fn ensure_clicks(ui: &mut GameUI, last: &AtomicUsize, routes: Vec<(String, ClickFn)>) {
    let cur = ui.filter_handler.len();
    let prev = last.load(Ordering::Relaxed);
    if prev == usize::MAX || cur < prev {
        let filter: Rc<dyn Fn(&UIEvent) -> bool> = Rc::new(move |e: &UIEvent| {
            if let UIEvent::Click { path, .. } = e {
                for (suf, cb) in routes.iter() {
                    if path.ends_with(suf.as_str()) || path == suf {
                        cb();
                        return true; // 소비(게임 기본 클릭 차단). 차단 원치 않으면 false 로.
                    }
                }
            }
            false
        });
        let handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = Rc::new(|_ctx| {});
        ui.filter_handler.push((filter, handler));
    }
    last.store(ui.filter_handler.len(), Ordering::Relaxed);
}

/// ensure_clicks 와 동일하나 필터를 **맨 앞(index 0)** 에 삽입 → 게임 기본 핸들러보다 먼저
/// 클릭을 소비한다. clone 한 네이티브 버튼(메뉴 이동 바인딩 보유)에 자기 동작을 붙일 때 필수.
/// (그냥 push 하면 게임 이동이 먼저 일어나 선수단 등으로 넘어가버림.)
pub fn ensure_clicks_front(ui: &mut GameUI, last: &AtomicUsize, routes: Vec<(String, ClickFn)>) {
    let cur = ui.filter_handler.len();
    let prev = last.load(Ordering::Relaxed);
    if prev == usize::MAX || cur < prev {
        let filter: Rc<dyn Fn(&UIEvent) -> bool> = Rc::new(move |e: &UIEvent| {
            if let UIEvent::Click { path, .. } = e {
                for (suf, cb) in routes.iter() {
                    if path.ends_with(suf.as_str()) || path == suf {
                        cb();
                        return true;
                    }
                }
            }
            false
        });
        let handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = Rc::new(|_ctx| {});
        ui.filter_handler.insert(0, (filter, handler));
    }
    last.store(ui.filter_handler.len(), Ordering::Relaxed);
}

/// ensure_clicks 와 동일하나 **이벤트를 소비하지 않는다**(항상 false 반환) = 관찰 전용.
/// 게임 기본 동작은 그대로 두고 "눌렸다" 는 사실만 알고 싶을 때 쓴다.
/// (소비하는 ensure_clicks 를 게임 버튼에 걸면 게임 동작이 죽는다 — 예: 스왑 코치 위임.)
pub fn ensure_clicks_observe(ui: &mut GameUI, last: &AtomicUsize, routes: Vec<(String, ClickFn)>) {
    let cur = ui.filter_handler.len();
    let prev = last.load(Ordering::Relaxed);
    if prev == usize::MAX || cur < prev {
        let filter: Rc<dyn Fn(&UIEvent) -> bool> = Rc::new(move |e: &UIEvent| {
            if let UIEvent::Click { path, .. } = e {
                for (suf, cb) in routes.iter() {
                    if path.ends_with(suf.as_str()) || path == suf {
                        cb();
                    }
                }
            }
            false // ★소비하지 않는다
        });
        let handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = Rc::new(|_ctx| {});
        ui.filter_handler.push((filter, handler));
    }
    last.store(ui.filter_handler.len(), Ordering::Relaxed);
}

/// 라우트 1개 편의 생성자: (노드 id, 콜백). 매칭은 `.id` 또는 정확히 id.
pub fn route(id: &str, cb: ClickFn) -> (String, ClickFn) {
    (format!(".{id}"), cb)
}

// =========================== 깊은 텍스트/리스트 ============================
/// 이 노드가 라벨이면 텍스트, 아니면 첫 자손 LabelRunner 의 텍스트를 채움.
pub fn text_set_deep(n: &mut Node, text: &str) -> bool {
    if label_set(n, text) { return true; }
    for c in n.child.iter_mut() { if text_set_deep(c, text) { return true; } }
    false
}

/// 목록: `.ui` 에 미리 박은 행 `{prefix}0..{max-1}` 에 items 를 1:1 채우고,
/// 데이터 없는 행은 hide. (엔진 제약상 행을 코드로 추가 못 함 → 미리 충분히 선언해둘 것.)
/// 반환 = 실제로 보인 행 수.
pub fn fill_list(root: &mut Node, prefix: &str, max_rows: usize, items: &[&str]) -> usize {
    let mut shown = 0;
    for i in 0..max_rows {
        if let Some(n) = find_mut(root, &format!("{prefix}{i}")) {
            match items.get(i) {
                Some(t) => { text_set_deep(n, t); n.visible = true; shown += 1; }
                None => { n.visible = false; }
            }
        }
    }
    shown
}

// =========================== Frame (클릭+시각 일괄) =========================
// 한 프레임의 클릭 라우트 + 색/선택 반영을 모아 `apply` 한 번에 처리.
// 내부적으로 ensure_clicks 를 *한 번만* 호출 → 핸들러 중복/재등록 꼬임 없음.
// 매 프레임 새로 만들어 쓰는 용도(가볍다). 상태는 모드의 static Atomic 으로 보관.
//
// 예) static ON: AtomicBool = ..; static SEL: AtomicUsize = ..; static FH: AtomicUsize = MAX;
//   let mut f = Frame::new();
//   f.button("ok_btn", Rc::new(|| { /* 실행 */ }));
//   f.toggle_color("box", &ON, Rgba::hex(0x37d5b3ff), Rgba::hex(0x1d1f2cff));
//   f.select(&["tab0","tab1","tab2"], &SEL, Rgba::hex(0x124f43ff), Rgba::hex(0x1d1f2cff));
//   f.apply(ui, &FH);
pub struct Frame {
    routes: Vec<(String, ClickFn)>,
    paints: Vec<Box<dyn Fn(&mut Node)>>,
}
impl Frame {
    pub fn new() -> Self { Frame { routes: Vec::new(), paints: Vec::new() } }

    /// 단순 버튼: 클릭 시 콜백 실행.
    pub fn button(&mut self, id: &str, cb: ClickFn) -> &mut Self {
        self.routes.push(route(id, cb)); self
    }
    /// 클릭할 때마다 bool 토글 + 그 상태로 배경색(:color 노드).
    pub fn toggle_color(&mut self, id: &str, state: &'static AtomicBool, on: Rgba, off: Rgba) -> &mut Self {
        let s = state;
        self.routes.push(route(id, Rc::new(move || { s.fetch_xor(true, Ordering::Relaxed); })));
        let idn = id.to_string();
        self.paints.push(Box::new(move |root: &mut Node| {
            if let Some(n) = find_mut(root, &idn) {
                rect_set_back(n, if s.load(Ordering::Relaxed) { on } else { off });
            }
        }));
        self
    }
    /// 클릭 토글 bool 만(색 없이). 시각은 직접 반영하고 싶을 때.
    pub fn toggle_flag(&mut self, id: &str, state: &'static AtomicBool) -> &mut Self {
        let s = state;
        self.routes.push(route(id, Rc::new(move || { s.fetch_xor(true, Ordering::Relaxed); })));
        self
    }
    /// 라디오/탭/목록 선택: ids 중 클릭한 것만 강조색(:color 노드), 나머지 base.
    pub fn select(&mut self, ids: &[&str], sel: &'static AtomicUsize, hi: Rgba, base: Rgba) -> &mut Self {
        for (i, id) in ids.iter().enumerate() {
            let s = sel; let idx = i;
            self.routes.push(route(id, Rc::new(move || s.store(idx, Ordering::Relaxed))));
        }
        let ids2: Vec<String> = ids.iter().map(|s| s.to_string()).collect();
        self.paints.push(Box::new(move |root: &mut Node| {
            let sv = sel.load(Ordering::Relaxed);
            for (i, id) in ids2.iter().enumerate() {
                if let Some(n) = find_mut(root, id) { rect_set_back(n, if i == sv { hi } else { base }); }
            }
        }));
        self
    }
    /// 임의 클릭 라우트(저수준).
    pub fn on_click(&mut self, id: &str, cb: ClickFn) -> &mut Self { self.button(id, cb) }

    /// 한 프레임 적용: 클릭 재등록(1회) + 모든 시각 반영.
    pub fn apply(&self, ui: &mut GameUI, last: &AtomicUsize) {
        ensure_clicks(ui, last, self.routes.clone());
        for p in &self.paints { p(&mut ui.root); }
    }
}

// =========================== Dropdown (커스텀, 완전제어) ====================
// DropdownRunner(네이티브)는 selected/items 가 private → 못 씀. 대신 color_selectable
// 행들로 직접 구성한 드롭다운. 모든 상태가 public 이라 펼침/선택/라벨 완전 제어.
// `.ui` 선언 필요(예제 examples/dropdown_example 의 .ui 스니펫 참고):
//   header(클릭=펼침) + header_label(선택값 표시) + panel(컨테이너) + item0..N(color_selectable)
pub struct Dropdown {
    pub header_id: &'static str,
    pub header_label_id: &'static str,
    pub panel_id: &'static str,
    pub item_ids: &'static [&'static str],
    pub items: &'static [&'static str],
    pub open: &'static AtomicBool,
    pub sel: &'static AtomicUsize,
}
impl Dropdown {
    pub fn selected(&self) -> usize { self.sel.load(Ordering::Relaxed) }
    pub fn selected_text(&self) -> &'static str { self.items.get(self.selected()).copied().unwrap_or("") }

    /// 매 프레임 호출: 클릭(헤더=펼침토글, 행=선택+닫기) 재등록 + 상태 반영.
    pub fn update(&self, ui: &mut GameUI, last: &AtomicUsize) {
        let mut routes: Vec<(String, ClickFn)> = Vec::new();
        let open = self.open;
        routes.push(route(self.header_id, Rc::new(move || { open.fetch_xor(true, Ordering::Relaxed); })));
        for (i, id) in self.item_ids.iter().enumerate() {
            let sel = self.sel; let openc = self.open; let idx = i;
            routes.push(route(id, Rc::new(move || { sel.store(idx, Ordering::Relaxed); openc.store(false, Ordering::Relaxed); })));
        }
        ensure_clicks(ui, last, routes);

        let is_open = self.open.load(Ordering::Relaxed);
        let sv = self.sel.load(Ordering::Relaxed);
        set_visible_by_id(&mut ui.root, self.panel_id, is_open);
        if let Some(h) = find_mut(&mut ui.root, self.header_label_id) {
            label_set(h, self.items.get(sv).copied().unwrap_or(""));
        }
        for (i, id) in self.item_ids.iter().enumerate() {
            if let Some(n) = find_mut(&mut ui.root, id) {
                toggle_set(n, i == sv);
                toggle_text_set(n, self.items.get(i).copied().unwrap_or(""));
            }
        }
    }
}

// =========================== 자가 검증 =====================================
/// 알려진 노드의 텍스트가 읽히면 오프셋 유효(게임 패치 감지용). post_update 초반 1회 권장.
pub fn self_check(root: &Node, known_label_id: &str) -> bool {
    find(root, known_label_id).and_then(label_get).is_some()
}

// =========================== 드래그 이동 (DraggableWindow) ==================
// UIEvent 에 마우스이동/드래그 없음 → WinAPI GetCursorPos + GetAsyncKeyState 폴링.
//
// 디버그 로그 (임시):
//   <모드폴더>\drag_debug.txt  — update() 호출·HWND·버튼 누름 기록
//   <모드폴더>\len_off_detect.txt — Length f32 오프셋 감지 결과

// ── Node 레이아웃 = stride 0x80 으로 4번 반복되는 "상태 블록" ─────────────────
// 블록 베이스: node+0x70(=112) / +0xf0 / +0x170 / +0x1f0
//   = normal / hover / press / disabled  (normal 이 첫 블록인 건 확실, 나머지 순서는 미확인)
// 블록 내 필드: w@+0x00, h@+0x08, x@+0x10, y@+0x18. 각 Length = { tag u32 @+0, f32 값 @+4 }.
// ⇒ 값 오프셋 = 블록베이스 + 필드 + LEN_VAL_OFF(4).
//
// ★ 읽기는 normal 블록으로 충분하지만 **쓰기는 4블록 전부** 해야 한다.
//   normal 만 쓰면 hover/press 시 .ui 선언값으로 되돌아간다(= 오래된 "인터랙티브 노드
//   크기 override 하면 호버 때 크기 튐" 함정의 정체). 엔진 draggable_popup Runner
//   (on_mouse_move 0x15e6190)도 4블록 전부에 쓴다.
//   → 아래 `*_all_states` 계열을 쓰면 인터랙티브 노드도 안전하다.
// 근거: 2026-07-08 Ghidra + Spectator_Chat 리사이즈 인게임 검증.
const NODE_LAYOUT_W: usize = 112;   // 0x70 = normal 블록의 w (tag 위치)
const NODE_LAYOUT_H: usize = 120;   // 0x78
const NODE_LAYOUT_X: usize = 128;   // 0x80
const NODE_LAYOUT_Y: usize = 136;   // 0x88
/// 레이아웃 상태 블록 4개의 베이스 오프셋(normal/hover/press/disabled).
pub const NODE_LAYOUT_BLOCKS: [usize; 4] = [0x70, 0xf0, 0x170, 0x1f0];
/// 블록 베이스 기준 필드 오프셋.
pub const NODE_LF_W: usize = 0x00;
pub const NODE_LF_H: usize = 0x08;
pub const NODE_LF_X: usize = 0x10;
pub const NODE_LF_Y: usize = 0x18;

#[link(name = "user32")]
extern "system" {
    fn GetCursorPos(pt: *mut [i32; 2]) -> i32;
    fn GetAsyncKeyState(vk: i32) -> i16;
    fn GetSystemMetrics(n: i32) -> i32;
    fn ScreenToClient(hwnd: usize, pt: *mut [i32; 2]) -> i32;
    fn GetClientRect(hwnd: usize, rect: *mut [i32; 4]) -> i32;
    fn EnumWindows(cb: extern "system" fn(usize, isize) -> i32, lparam: isize) -> i32;
    fn GetWindowThreadProcessId(hwnd: usize, pid: *mut u32) -> u32;
    fn IsWindowVisible(hwnd: usize) -> i32;
    fn GetWindow(hwnd: usize, cmd: u32) -> usize;
}
#[link(name = "kernel32")]
extern "system" { fn GetCurrentProcessId() -> u32; }
const VK_LBUTTON: i32 = 0x01;
const SM_CXSCREEN: i32 = 0;
const SM_CYSCREEN: i32 = 1;
const GW_OWNER: u32 = 4;

// 게임 메인 창 HWND (현재 프로세스 소유의 가장 큰 top-level visible 창). 1회 탐색 후 캐시.
static GAME_HWND: AtomicUsize = AtomicUsize::new(0);

extern "system" fn enum_win_cb(hwnd: usize, lparam: isize) -> i32 {
    unsafe {
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, &mut pid);
        if pid == GetCurrentProcessId()
            && IsWindowVisible(hwnd) != 0
            && GetWindow(hwnd, GW_OWNER) == 0
        {
            let mut r = [0i32; 4];
            if GetClientRect(hwnd, &mut r) != 0 {
                let area = (r[2] - r[0]) as i64 * (r[3] - r[1]) as i64;
                let best = lparam as *mut (usize, i64);
                if area > (*best).1 { (*best).0 = hwnd; (*best).1 = area; }
            }
        }
    }
    1 // 계속 열거 (가장 큰 창 선택)
}

fn game_hwnd() -> usize {
    let c = GAME_HWND.load(Ordering::Relaxed);
    if c != 0 { return c; }
    let mut best: (usize, i64) = (0, 0);
    unsafe { EnumWindows(enum_win_cb, &mut best as *mut (usize, i64) as isize); }
    if best.0 != 0 { GAME_HWND.store(best.0, Ordering::Relaxed); }
    best.0
}

// 커서(스크린) → 게임 가상좌표(1920x1080).
// 게임은 16:9 콘텐츠를 클라이언트 안에 비율 유지로 맞추고 남는 공간을 레터박스(검은 여백)로 둔다.
// 따라서 실제 게임 콘텐츠 영역(여백 제외)만 0..1920 / 0..1080 에 매핑해야 정확.
// 16:9 창에선 여백이 0이라 자동으로 전체 매핑과 같아짐. HWND 못 잡으면 모니터 전체 기준 폴백.
pub fn cursor_to_game() -> (f32, f32) {
    unsafe {
        let mut pt = [0i32; 2];
        GetCursorPos(&mut pt);
        let hwnd = game_hwnd();
        if hwnd != 0 {
            let mut r = [0i32; 4];
            if GetClientRect(hwnd, &mut r) != 0 {
                let cw = (r[2] - r[0]) as f32;
                let ch = (r[3] - r[1]) as f32;
                if cw >= 1.0 && ch >= 1.0 {
                    ScreenToClient(hwnd, &mut pt);   // pt 가 클라이언트 좌표로 변형됨
                    let target = 1920.0 / 1080.0;
                    let aspect = cw / ch;
                    // 16:9 콘텐츠가 클라이언트 안에서 차지하는 실제 영역 + 여백 오프셋
                    let (gw, gh, ox, oy) = if aspect > target {
                        let w = ch * target; (w, ch, (cw - w) * 0.5, 0.0)   // 좌우 레터박스
                    } else {
                        let h = cw / target; (cw, h, 0.0, (ch - h) * 0.5)   // 상하 레터박스
                    };
                    return ((pt[0] as f32 - ox) / gw * 1920.0,
                            (pt[1] as f32 - oy) / gh * 1080.0);
                }
            }
        }
        // 폴백: 모니터 전체 해상도 기준 (여기 도달 시 pt 는 아직 스크린 좌표)
        let sw = GetSystemMetrics(SM_CXSCREEN).max(1) as f32;
        let sh = GetSystemMetrics(SM_CYSCREEN).max(1) as f32;
        (pt[0] as f32 * 1920.0 / sw, pt[1] as f32 * 1080.0 / sh)
    }
}

// Length = { tag u32 @+0 (1=Px), f32 값 @+4 }. 값 오프셋은 **상수 4로 확정**
// (2026-07-08 Ghidra: draggable_popup on_mouse_move 0x15e6190 이 tag=1 + f32@+4 로 기입.
//  0.4.13/0.4.14/0.5.0 공통). 구 런타임 휴리스틱(detect_len_off + len_off_detect.txt)은 폐기.
const LEN_VAL_OFF: usize = 4;

// 로그 쓰기 헬퍼 (두 경로 중 첫 번째 성공 경로 사용)
// log_dirs: 시도할 폴더 목록 (앞쪽 우선)
fn try_log_append(dirs: &[&str], filename: &str, msg: &str) {
    use std::io::Write;
    for d in dirs {
        let p = format!("{}\\{}", d, filename);
        if std::fs::OpenOptions::new().append(true).create(true)
            .open(&p).and_then(|mut f| f.write_all(msg.as_bytes())).is_ok() { return; }
    }
}
fn try_log_write(dirs: &[&str], filename: &str, msg: &str) {
    for d in dirs {
        let p = format!("{}\\{}", d, filename);
        if std::fs::write(&p, msg).is_ok() { return; }
    }
}

unsafe fn node_read_layout_xy(n: &Node, _log_dirs: &[&str]) -> (f32, f32) {
    let b = n as *const Node as *const u8;
    (*(b.add(NODE_LAYOUT_X + LEN_VAL_OFF) as *const f32),
     *(b.add(NODE_LAYOUT_Y + LEN_VAL_OFF) as *const f32))
}
unsafe fn node_write_layout_xy(n: &mut Node, x: f32, y: f32, _log_dirs: &[&str]) {
    let b = n as *mut Node as *mut u8;
    *(b.add(NODE_LAYOUT_X + LEN_VAL_OFF) as *mut f32) = x;
    *(b.add(NODE_LAYOUT_Y + LEN_VAL_OFF) as *mut f32) = y;
}

// ----------------------- layout 직접 쓰기 (위치/크기) ----------------------
// Node 의 x/y/width/height(Length 의 f32)를 런타임에 덮어쓴다.
// ⚠️ .ui 선언이 px 인 노드에만 써라. `width:100%`(Percent) 노드에 px f32 를 쓰면 280%로 해석돼 깨진다.
// ⚠️ 아래 non-`_all_states` 함수들은 **normal 블록만** 쓴다 → 인터랙티브 노드
//    (scroll_view/color_selectable/button)는 hover/상태전이 때 .ui 선언값으로 되돌아간다("크기 튐").
//    → 인터랙티브 노드엔 `*_all_states` 를 써라. color/empty/label 은 어느 쪽이든 안전.

/// 노드의 layout x,y(px) 설정. **normal 블록만** — 인터랙티브 노드엔 `set_layout_xy_all_states` 사용.
pub fn set_layout_xy(n: &mut Node, x: f32, y: f32) {
    unsafe { node_write_layout_xy(n, x, y, &[]); }
}
/// 노드의 layout width,height(px) 설정. 0 이하인 축은 건드리지 않음(한쪽만 바꿀 때 0 전달).
/// **normal 블록만** — 인터랙티브 노드엔 `set_layout_size_all_states` 사용.
pub fn set_layout_size(n: &mut Node, w: f32, h: f32) {
    unsafe {
        let b = n as *mut Node as *mut u8;
        if w > 0.0 { *(b.add(NODE_LAYOUT_W + LEN_VAL_OFF) as *mut f32) = w; }
        if h > 0.0 { *(b.add(NODE_LAYOUT_H + LEN_VAL_OFF) as *mut f32) = h; }
    }
}
/// id 로 찾아 x,y 설정. 찾으면 true.
pub fn set_layout_xy_by_id(root: &mut Node, id: &str, x: f32, y: f32) -> bool {
    match find_mut(root, id) { Some(n) => { set_layout_xy(n, x, y); true } None => false }
}
/// id 로 찾아 width,height 설정(0 이하 축은 유지). 찾으면 true.
pub fn set_layout_size_by_id(root: &mut Node, id: &str, w: f32, h: f32) -> bool {
    match find_mut(root, id) { Some(n) => { set_layout_size(n, w, h); true } None => false }
}

// ---- 4상태 블록 전체 write (인터랙티브 노드 안전) ----
// 엔진 러너와 동일하게 normal/hover/press/disabled 4개 블록에 모두 기입한다.
// 읽기는 normal 블록 하나로 충분(4블록은 항상 같이 갱신되므로).

/// 노드의 레이아웃 필드(NODE_LF_*) 값을 normal 블록에서 읽는다.
pub fn node_layout_read(n: &Node, field: usize) -> f32 {
    unsafe {
        let b = n as *const Node as *const u8;
        *(b.add(NODE_LAYOUT_BLOCKS[0] + field + LEN_VAL_OFF) as *const f32)
    }
}
/// 노드의 레이아웃 필드(NODE_LF_*) 값을 **4개 상태 블록 전부**에 기입한다.
pub fn node_layout_write_all(n: &mut Node, field: usize, v: f32) {
    unsafe {
        let b = n as *mut Node as *mut u8;
        for blk in NODE_LAYOUT_BLOCKS {
            *(b.add(blk + field + LEN_VAL_OFF) as *mut f32) = v;
        }
    }
}
/// x,y 를 4상태 블록 전부에 설정(인터랙티브 노드 안전).
pub fn set_layout_xy_all_states(n: &mut Node, x: f32, y: f32) {
    node_layout_write_all(n, NODE_LF_X, x);
    node_layout_write_all(n, NODE_LF_Y, y);
}
/// width,height 를 4상태 블록 전부에 설정(0 이하 축은 유지). 인터랙티브 노드 안전.
pub fn set_layout_size_all_states(n: &mut Node, w: f32, h: f32) {
    if w > 0.0 { node_layout_write_all(n, NODE_LF_W, w); }
    if h > 0.0 { node_layout_write_all(n, NODE_LF_H, h); }
}
/// id 로 찾아 4상태 x,y 설정. 찾으면 true.
pub fn set_layout_xy_all_states_by_id(root: &mut Node, id: &str, x: f32, y: f32) -> bool {
    match find_mut(root, id) { Some(n) => { set_layout_xy_all_states(n, x, y); true } None => false }
}
/// id 로 찾아 4상태 width,height 설정(0 이하 축은 유지). 찾으면 true.
pub fn set_layout_size_all_states_by_id(root: &mut Node, id: &str, w: f32, h: f32) -> bool {
    match find_mut(root, id) { Some(n) => { set_layout_size_all_states(n, w, h); true } None => false }
}

/// ⚠️ **DEPRECATED (2026-07-08) — 신규 코드에서 쓰지 말 것. 현재 실사용 모드 0개.**
///
/// Win32 폴링(GetCursorPos + GetAsyncKeyState) 기반 창 드래그. 엔진에 UIEvent 마우스이동이
/// 없던 시절의 우회책이다. 지금은 **엔진 내장 `draggable_popup` Runner** 로 완전 대체됐다:
/// `.ui` 루트를 `#id:draggable_popup { back_color:#00000000; }` 으로 선언하고,
/// 매 프레임 러너 필드를 충전하면 이동 + 4변 리사이즈가 공짜로 붙는다.
/// ```text
/// runner+0x1c0 header_height   (드래그 밴드 높이. 창 전체면 큰 값)
/// runner+0x1c4 min_w           (0이면 폭 0까지 줄어듦 → 필수)
/// runner+0x1c8 min_h
/// runner+0x1cc resize_handle   (>0 이어야 리사이즈 ON. 4변/4코너 감지 밴드 폭 px)
/// ```
/// ⚠ 엔진 드래그 위치 클램프는 `clamp(0,1820)/clamp(0,1030)` 하드코딩이라 **창 크기를 무시**한다
///   → 오른쪽·아래로 화면 밖으로 나감. 모드가 매 프레임 `clamp(x,0,1920-w)`,`clamp(y,0,1080-h)` 재적용할 것.
/// 실사용 예: `Spectator_Chat`(이동+리사이즈+자식 추종). 상세 = 메모리 `tfm2-minimap-camera-drag`.
///
/// 같은 노드 x/y 를 이 구조체와 러너가 동시에 쓰면 충돌하므로 **병용 금지**.
/// (남겨둔 이유: 러너를 못 쓰는 비-popup 노드용 폴백 + Win32 헬퍼(`cursor_to_game`)와 코드 공유.)
///
/// ```rust
/// // log_dirs: 디버그 로그 쓸 폴더 (빈 슬라이스면 로그 없음)
/// static DRAG: DraggableWindow = DraggableWindow::new("win_root", "", 400.0, 200.0);
/// // post_update:
/// DRAG.update(&mut ui.root, &["C:\\mod\\path"]);
/// ```
pub struct DraggableWindow {
    pub node_id:   &'static str,
    pub handle_id: &'static str,
    pub panel_w:   f32,
    pub panel_h:   f32,
    // 이동 허용 경계 (게임 가상좌표 1920x1080 기준). (0,0,0,0) = 무제한.
    pub bx1: f32, pub by1: f32, pub bx2: f32, pub by2: f32,
    enabled:   AtomicBool,
    dragging:  AtomicBool,
    prev_btn:  AtomicBool,
    start_mx:  AtomicI32,
    start_my:  AtomicI32,
    start_nx:  AtomicI32,
    start_ny:  AtomicI32,
    dbg_ctr:   AtomicUsize,
}

impl DraggableWindow {
    /// `bounds` = (x1,y1,x2,y2): 패널이 벗어날 수 없는 사각 영역(게임 가상좌표 1920x1080).
    /// (0.0, 0.0, 0.0, 0.0) 이면 제한 없음. 패널 우/하단이 (x2,y2)를 넘지 않도록 clamp.
    pub const fn new(
        node_id: &'static str,
        handle_id: &'static str,
        panel_w: f32,
        panel_h: f32,
        bounds: (f32, f32, f32, f32),
    ) -> Self {
        DraggableWindow {
            node_id, handle_id, panel_w, panel_h,
            bx1: bounds.0, by1: bounds.1, bx2: bounds.2, by2: bounds.3,
            enabled:  AtomicBool::new(true),
            dragging: AtomicBool::new(false),
            prev_btn: AtomicBool::new(false),
            start_mx: AtomicI32::new(0), start_my: AtomicI32::new(0),
            start_nx: AtomicI32::new(0), start_ny: AtomicI32::new(0),
            dbg_ctr:  AtomicUsize::new(0),
        }
    }

    /// 경계 내로 위치 제한. 경계가 (0,0,0,0)이면 그대로 통과.
    fn clamp_pos(&self, x: f32, y: f32) -> (f32, f32) {
        if self.bx1 == 0.0 && self.by1 == 0.0 && self.bx2 == 0.0 && self.by2 == 0.0 {
            return (x, y);
        }
        let xmax = (self.bx2 - self.panel_w).max(self.bx1);
        let ymax = (self.by2 - self.panel_h).max(self.by1);
        (x.clamp(self.bx1, xmax), y.clamp(self.by1, ymax))
    }

    /// 드래그 활성/비활성 토글 (런타임). false면 update()가 즉시 반환.
    pub fn set_enabled(&self, on: bool) { self.enabled.store(on, Ordering::Relaxed); }
    pub fn is_enabled(&self) -> bool { self.enabled.load(Ordering::Relaxed) }

    /// post_update 에서 매 프레임 호출.
    /// `log_dirs`: 디버그 로그 폴더 목록 (앞쪽 우선; 빈 슬라이스면 로그 없음).
    pub fn update(&self, root: &mut Node, log_dirs: &[&str]) {
        if !self.enabled.load(Ordering::Relaxed) { return; }
        let ctr = self.dbg_ctr.fetch_add(1, Ordering::Relaxed);
        let (gx, gy) = cursor_to_game();

        // 초반 + 1초 시점 기본 상태 로그
        if ctr < 3 || ctr == 60 {
            try_log_append(log_dirs, "drag_debug.txt",
                &format!("frame={} gx={:.0} gy={:.0}\n", ctr, gx, gy));
        }

        let btn = unsafe { (GetAsyncKeyState(VK_LBUTTON) as u16) & 0x8000 != 0 };
        let just_pressed = btn && !self.prev_btn.load(Ordering::Relaxed);
        self.prev_btn.store(btn, Ordering::Relaxed);

        if self.dragging.load(Ordering::Relaxed) {
            if !btn {
                self.dragging.store(false, Ordering::Relaxed);
            } else {
                let smx = f32::from_bits(self.start_mx.load(Ordering::Relaxed) as u32);
                let smy = f32::from_bits(self.start_my.load(Ordering::Relaxed) as u32);
                let snx = f32::from_bits(self.start_nx.load(Ordering::Relaxed) as u32);
                let sny = f32::from_bits(self.start_ny.load(Ordering::Relaxed) as u32);
                if let Some(n) = find_mut(root, self.node_id) {
                    let (nx, ny) = self.clamp_pos(snx + (gx - smx), sny + (gy - smy));
                    unsafe { node_write_layout_xy(n, nx, ny, log_dirs); }
                }
            }
            return;
        }

        if just_pressed {
            let hid = if self.handle_id.is_empty() { self.node_id } else { self.handle_id };
            let (hx, hy) = match find(root, hid) {
                Some(n) => unsafe { node_read_layout_xy(n, log_dirs) },
                None => {
                    try_log_append(log_dirs, "drag_debug.txt",
                        &format!("PRESS node_not_found='{}' gx={:.0} gy={:.0}\n", hid, gx, gy));
                    return;
                }
            };
            let hit = gx >= hx && gx < hx + self.panel_w && gy >= hy && gy < hy + self.panel_h;
            try_log_append(log_dirs, "drag_debug.txt",
                &format!("PRESS gx={:.0} gy={:.0} hx={:.0} hy={:.0} pw={:.0} ph={:.0} HIT={}\n",
                    gx, gy, hx, hy, self.panel_w, self.panel_h, hit));
            if hit {
                let (snx, sny) = find(root, self.node_id)
                    .map(|n| unsafe { node_read_layout_xy(n, log_dirs) })
                    .unwrap_or((hx, hy));
                self.start_mx.store(gx.to_bits() as i32, Ordering::Relaxed);
                self.start_my.store(gy.to_bits() as i32, Ordering::Relaxed);
                self.start_nx.store(snx.to_bits() as i32, Ordering::Relaxed);
                self.start_ny.store(sny.to_bits() as i32, Ordering::Relaxed);
                self.dragging.store(true, Ordering::Relaxed);
            }
        }
    }

    pub fn is_dragging(&self) -> bool { self.dragging.load(Ordering::Relaxed) }

    /// 지정 라벨 노드에 디버그 정보 출력 (파일 I/O 불가 환경용).
    /// ctr < limit 동안만 동작 (limit=0 → 항상).
    pub fn debug_label(&self, root: &mut Node, label_id: &str, limit: usize) {
        let ctr = self.dbg_ctr.load(Ordering::Relaxed);
        if limit > 0 && ctr >= limit { return; }
        let (gx, gy) = cursor_to_game();
        let drag = self.dragging.load(Ordering::Relaxed);
        let (hx, hy) = {
            let r: &Node = root;
            find(r, self.node_id)
                .map(|n| unsafe { node_read_layout_xy(n, &[]) })
                .unwrap_or((-1.0, -1.0))
        };
        // 진단: 게임 창 클라이언트 크기 + 마우스 raw 클라이언트 좌표
        let (cw, ch, rcx, rcy) = unsafe {
            let h = game_hwnd();
            let mut r = [0i32; 4]; let mut p = [0i32; 2];
            GetCursorPos(&mut p);
            if h != 0 { GetClientRect(h, &mut r); ScreenToClient(h, &mut p); }
            (r[2] - r[0], r[3] - r[1], p[0], p[1])
        };
        let msg = format!("cli{}x{} raw{},{} cur{:.0},{:.0} nd{:.0},{:.0} d{}",
            cw, ch, rcx, rcy, gx, gy, hx, hy, drag as u8);
        if let Some(n) = find_mut(root, label_id) { label_set(n, &msg); }
    }

    pub fn set_pos(&self, root: &mut Node, x: f32, y: f32, log_dirs: &[&str]) {
        let (x, y) = self.clamp_pos(x, y);
        if let Some(n) = find_mut(root, self.node_id) {
            unsafe { node_write_layout_xy(n, x, y, log_dirs); }
        }
    }
}

// =========================== NativeDropdown ================================
// SDK 미노출 네이티브 DropdownRunner 를 메모리 RE 로 제어 — 임의 동적 옵션 주입 + 선택 읽기.
// `.ui` 에 `#id:dropdown { @"asset/base/style/main#dropdown"; text:{size:18;} item_text:{size:18;}
//   item_layout:{height:40px;} item_text_layout:{x:14px;y:6px;width:208px;height:28px;} }` 선언 필요.
// ★스크롤 풀다운: .ui 선언에 `max_items_height:280;` 추가 → 옵션 총높이가 그 px 넘으면 펼침목록
//   자동 스크롤(엔진이 스크롤바/클립 처리). 커스텀 color_selectable 불필요.
//   값 ≈ (보일 항목수)×item_layout.height. 게임 실측: produce 280 / option 280~400.
//   재사용 스니펫·원문 = examples/native_scroll_dropdown.md.
// ★★0.5.6 재핀·구조 재확정 (2026-08-23) — 그 전 값은 **두 군데 다 틀렸다**:
//   ① 구 `DD_SETOPT_RVA = 0x1fa5e30` 은 0.5.6 에서 **함수 시작이 아니라 코드 중간** = 호출 시 즉사.
//   ② 구 `DdOpt` 는 **0x28**(40B) 였는데 실제 원소는 **0xf8**(248B) — 그대로 넘기면 게임이
//      우리 버퍼 6배 바깥까지 읽는다.
//   근거·검증 = REPORT	fm2_champ_pos_lock\RE6-08-23_네이티브드롭다운-0.5.6-재핀-옵션구조.md
// ⚠RVA 는 패치마다 바뀐다. **호출 전 디스어셈블로 함수 프롤로그인지 1회 확인**할 것(5초).
const DD_SETOPT_RVA:  usize = 0x1c1710;  // (rcx=runner, rdx=sel, r8=&[cap,ptr,len])
const DD_SELECTED_OFF: usize = 0x1788;   // runner + 현재선택 idx (u64) — 0.5.6 확인
const DD_ICON_LEN: usize = 0xd0;         // 옵션 앞부분 = IconProperty. 0 채우기 안전(아래 주석)

/// 드롭다운 옵션 1개 = 0xf8. 게임이 이 배열을 **그대로 소유**한다.
/// ★0 채우기 안전성(검증): 원소 drop 은 `+0x00/+0x18/+0x30` 을 **cap!=0 && cap!=-1 일 때만** free 하고,
///   이어지는 `+0x48` drop 은 **첫 줄이 `if (p[1] != 0)`** 라 0 이면 즉시 반환한다.
///   ⟹ 아이콘을 안 쓰면 앞 0xd0 을 전부 0 으로 두면 된다.
#[repr(C)]
struct DdOpt {
    // ⚠크기가 0xf8 이 아니면 게임이 우리 버퍼 밖을 읽는다 — 컴파일타임에 못 박는다.
    // (아래 const 어서션은 struct 정의 뒤에 온다)
    icon: [u8; DD_ICON_LEN], // +0x00 IconProperty (0 = 아이콘 없음)
    color: [f32; 4],         // +0xd0 RGBA
    s_cap: usize,            // +0xe0 ★0 이면 게임이 문자열 버퍼를 free 하지 않는다(우리가 leak)
    s_ptr: usize,            // +0xe8
    s_len: usize,            // +0xf0
}
const _: () = assert!(core::mem::size_of::<DdOpt>() == 0xf8);
const _: () = assert!(core::mem::offset_of!(DdOpt, color) == 0xd0);
const _: () = assert!(core::mem::offset_of!(DdOpt, s_cap) == 0xe0);

pub struct NativeDropdown { pub node_id: &'static str }
impl NativeDropdown {
    pub const fn new(node_id: &'static str) -> Self { NativeDropdown { node_id } }

    /// 옵션 주입. `sel`=초기 선택 인덱스.
    /// ⚠ABI 호출 — 게임 로드 시점에 부르면 멈춘다. **팝업이 실제로 열린 뒤 1회만** 호출할 것.
    /// 텍스트는 평문 또는 `#asset/base/text/...?key` i18n 태그 둘 다 된다(게임이 해석).
    pub fn set_options(&self, root: &Node, items: &[&str], sel: u64) -> bool {
        let Some(node) = find(root, self.node_id) else { return false; };
        let base = match unsafe { base_of(node, "DropdownRunner") } {
            Some(b) => b as usize, None => return false,
        };
        let mut opts: Vec<DdOpt> = Vec::with_capacity(items.len());
        for &it in items {
            let s = it.to_string();
            let (ptr, len) = (s.as_ptr() as usize, s.len());
            std::mem::forget(s); // 게임이 free 안 하므로(cap=0) 우리도 drop 하지 않는다 = 의도적 leak
            opts.push(DdOpt {
                icon: [0u8; DD_ICON_LEN],
                color: [0.909_803_9, 0.909_803_9, 0.909_803_9, 1.0], // 게임 기본 #e8e8e8ff
                s_cap: 0,
                s_ptr: ptr,
                s_len: len,
            });
        }
        let param3: [usize; 3] = [0, opts.as_ptr() as usize, opts.len()]; // cap=0 → 게임이 배열 free 안 함
        unsafe {
            let addr = GetModuleHandleW(std::ptr::null()) + DD_SETOPT_RVA;
            let f: unsafe extern "system" fn(usize, u64, *const [usize; 3]) = std::mem::transmute(addr);
            f(base, sel, &param3);
        }
        std::mem::forget(opts); // 배열 소유권도 게임에 넘김
        true
    }

    /// 펼침 목록 최대높이 → 넘치면 스크롤. ★정석은 .ui 의 `max_items_height:NNN;` 선언
    /// (파서가 처리, 안전). 이건 .ui 를 못 고칠 때의 보조 폴백: runner+0x1d8 직접 라이트(실험적).
    pub fn set_popup_max_height(&self, root: &Node, h: u32) {
        if let Some(node) = find(root, self.node_id) {
            if let Some(base) = unsafe { base_of(node, "DropdownRunner") } {
                unsafe { *((base as usize + 0x1d8) as *mut u32) = h; } // +472 (max visible items?)
            }
        }
    }

    /// 현재 선택 인덱스 (게임이 옵션 클릭 시 자동 기록). 노드 없거나 무효(-1)면 None.
    pub fn selected(&self, root: &Node) -> Option<usize> {
        let node = find(root, self.node_id)?;
        let base = unsafe { base_of(node, "DropdownRunner")? } as usize;
        let v = unsafe { *((base + DD_SELECTED_OFF) as *const u64) };
        if v == u64::MAX { None } else { Some(v as usize) }
    }
}
