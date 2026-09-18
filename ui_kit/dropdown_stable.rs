//! dropdown_stable — stable 껍데기 모드용 "게임 dropdown 모양" 풀다운 (버튼 + 목록 패널).
//! 배경: stable API 엔 `dropdown` 위젯 항목 주입이 없다(state_set_json 쓰기 키 = checkbox/text_edit/slider/selectable 뿐 · `dropdown` 은 selected_item 읽기만).
//!   ⟹ 게임 `style/main#dropdown` 정의(배경 stroke #4a4c56·채움 #1d1f2c·rounding 8 · hover stroke #a5a5ab · 화살표 icons/dropdown 8.78×5.06 우측 -20px ·
//!   텍스트 regular #e8e8e8 x14 · 항목 텍스트 #a3a9b6 hover #e8e8e8 · 선택 항목 우측 -25px 체크 icons/check 14×9.57)를 color_icon_button/label/image 로 재현한다.
//! 동작: 버튼 클릭 = 열기/닫기 · 항목 클릭 = 선택+닫기 · **바깥 클릭 = 닫기**(GetAsyncKeyState 좌클릭 에지 + 커서가 버튼∪목록 밖) · 항목 hover 글자색 · 선택 항목 체크.
//! 클릭 등록(버튼/항목)은 모드가 `ui_register_click` 으로 직접(모드마다 재등록 정책이 다름) — 이 모듈은 소스 생성 + 매프레임 `tick` 만.
//! 사용: `#[path = r"C:\tfm2mods\ui_kit\dropdown_stable.rs"] mod dd;` → .ui 생성 시 `dd::button_source`/`dd::list_source` → 매프레임 `dd::tick(...)`.
#![allow(dead_code)]
use mod_api_stable::StableClient;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub const COL_TEXT: &str = "#e8e8e8ff";
pub const COL_ITEM: &str = "#a3a9b6ff";
pub const COL_ITEM_HOVER: &str = "#e8e8e8ff";
pub const LIST_GAP: f32 = 2.0;

#[link(name = "user32")]
extern "system" {
    fn GetForegroundWindow() -> usize;
    fn GetWindowThreadProcessId(h: usize, pid: *mut u32) -> u32;
    fn GetCursorPos(p: *mut [i32; 2]) -> i32;
    fn ScreenToClient(h: usize, p: *mut [i32; 2]) -> i32;
    fn GetClientRect(h: usize, r: *mut [i32; 4]) -> i32;
    fn GetAsyncKeyState(vk: i32) -> i16;
}
#[link(name = "kernel32")]
extern "system" { fn GetCurrentProcessId() -> u32; }

/// 커서 위치(UI 1920×1080 좌표계, 게임 창이 전면일 때만).
pub fn cursor_ui() -> Option<(f32, f32)> {
    unsafe {
        let h = GetForegroundWindow(); if h == 0 { return None; }
        let mut pid = 0u32; GetWindowThreadProcessId(h, &mut pid);
        if pid != GetCurrentProcessId() { return None; }
        let mut p = [0i32; 2]; if GetCursorPos(&mut p) == 0 { return None; }
        if ScreenToClient(h, &mut p) == 0 { return None; }
        let mut r = [0i32; 4]; if GetClientRect(h, &mut r) == 0 || r[2] <= 0 || r[3] <= 0 { return None; }
        Some((p[0] as f32 * 1920.0 / r[2] as f32, p[1] as f32 * 1080.0 / r[3] as f32))
    }
}
pub fn inside(r: (f32, f32, f32, f32), p: (f32, f32)) -> bool { p.0 >= r.0 && p.0 <= r.0 + r.2 && p.1 >= r.1 && p.1 <= r.1 + r.3 }

static LAST_DOWN: AtomicBool = AtomicBool::new(false);
/// 이번 프레임에 좌클릭이 "눌린 순간"인지(에지). 프레임당 1회만 호출할 것(여러 풀다운이면 결과를 공유).
pub fn click_edge() -> bool {
    // ⚠짧은 클릭(down→up 이 한 프레임 안)은 0x8000 에지로 못 잡는다(09-18 실측: 자동화 클릭) → "마지막 호출 이후 눌림" 비트(0x0001)도 본다.
    let st = unsafe { GetAsyncKeyState(0x01) } as u16;
    let down = st & 0x8000 != 0;
    let was = LAST_DOWN.swap(down, Ordering::Relaxed);
    (down && !was) || (st & 0x0001 != 0)
}

/// 닫힌 상태 버튼(.ui 소스). `label_tag` = i18n 태그 또는 평문. 게임 dropdown 배경·화살표·텍스트 규격.
pub fn button_source(id: &str, x: i32, y: i32, w: i32, h: i32, label_tag: &str, size: i32) -> String {
    let ax = w - 20 - 9; let ay = (h - 5) / 2;
    format!(
"#{id}:color_icon_button {{ x: {x}px; y: {y}px; width: {w}px; height: {h}px;
  btn: {{ color: #4a4c56ff; stroke: 1; back_color: #1d1f2cff; rounding: Uniform {{ rounding: 8; }} }}
  icon: {{ source: \"asset/base/ui/icons/dropdown\"; color: #a5a5abff; rect: {{ x: {ax}; y: {ay}; w: 9; h: 5; }} }}
  hover: {{ btn: {{ color: #a5a5abff; }} icon: {{ color: #e8e8e8ff; }} }}
  hover_sound: \"asset/base/sound/sfx/UI_mouse_hover\"; click_sound: \"asset/base/sound/sfx/UI_mouse_click\";
  #label:label {{ ignore_event: true; x: 14px; y: 0px; width: {lw}px; height: {h}px; font: \"asset/base/font/set/regular\"; size: {size}; color: {ct}; align_y: Center; text: \"{label_tag}\"; }}
}}
", lw = w - 14 - 30, ct = COL_TEXT)
}

/// 열린 목록 패널(.ui 소스) — 팝업 루트의 **마지막 자식**으로 넣어야 그리드 위에 그려진다. `items` = (노드 id, 텍스트 태그).
pub fn list_source(id: &str, x: i32, y: i32, w: i32, item_h: i32, size: i32, items: &[(&str, &str)]) -> String {
    let mut s = format!(
"#{id}:color {{ visible: false; x: {x}px; y: {y}px; width: {w}px; height: {hh}px;
  color: #1d1f2cff; stroke: 1; back_color: #4a4c56ff; rounding: Uniform {{ rounding: 8; }}
  padding: {{ left: 0px; right: 0px; top: 0px; bottom: 0px; }}
  child_type: TopToBottom {{ spacing: 0px; }}
", hh = item_h * items.len() as i32);
    for (iid, tag) in items {
        s.push_str(&format!(
"  #{iid}:color_icon_button {{ width: {w}px; height: {item_h}px;
    btn: {{ color: #00000000; stroke: 0; back_color: #00000000; rounding: Uniform {{ rounding: 8; }} }}
    hover_sound: \"asset/base/sound/sfx/UI_mouse_hover\"; click_sound: \"asset/base/sound/sfx/UI_mouse_click\";
    #label:label {{ ignore_event: true; x: 14px; y: 0px; width: {lw}px; height: {item_h}px; font: \"asset/base/font/set/regular\"; size: {size}; color: {ci}; align_y: Center; text: \"{tag}\"; }}
    #check:image {{ ignore_event: true; visible: false; anchor_x: 1; pivot_x: 1; x: -25px; anchor_y: 0.5; pivot_y: 0.5; width: 14px; height: 10px; source: \"asset/base/ui/icons/check\"; color: #a5a5abff; }}
  }}
", lw = w - 14 - 40, ci = COL_ITEM));
    }
    s.push_str("}\n");
    s
}

static COLOR_CACHE: Mutex<Option<std::collections::HashMap<String, String>>> = Mutex::new(None);
fn set_color(ctx: &mut StableClient<'_>, path: &str, col: &str) {
    { let mut g = COLOR_CACHE.lock().unwrap_or_else(|e| e.into_inner()); let m = g.get_or_insert_with(std::collections::HashMap::new); if m.get(path).map(|s| s.as_str()) == Some(col) { return; } m.insert(path.to_string(), col.to_string()); }
    ctx.ui_set_properties(path, &format!("color: {};", col));
}
/// 팝업이 닫히거나 재스폰됐을 때 색 캐시 초기화(노드가 새로 생겨 캐시가 거짓이 됨).
pub fn reset_cache() { *COLOR_CACHE.lock().unwrap_or_else(|e| e.into_inner()) = None; }

/// 매프레임 갱신. `open` = 열림 상태(버튼/항목 클릭 핸들러가 토글·해제) · `sel` = 선택 인덱스 · `label_tag` = 버튼에 보일 텍스트 · `edge` = 이번 프레임 `click_edge()`.
/// 바깥 클릭이면 `open` 을 false 로 내린다.
pub fn tick(ctx: &mut StableClient<'_>, btn: &str, list: &str, item_ids: &[&str], open: &AtomicBool, sel: usize, label_tag: &str, edge: bool) {
    let cur = cursor_ui();
    let mut is_open = open.load(Ordering::Relaxed);
    if is_open && edge {
        let inb = ctx.ui_node_rect(btn).zip(cur).map(|(r, p)| inside(r, p)).unwrap_or(false);
        let inl = ctx.ui_node_rect(list).zip(cur).map(|(r, p)| inside(r, p)).unwrap_or(false);
        if !inb && !inl { open.store(false, Ordering::Relaxed); is_open = false; }
    }
    if ctx.ui_visible(list) != Some(is_open) { ctx.ui_set_visible(list, is_open); }
    let lp = format!("{}.label", btn);
    if ctx.ui_text(&lp).as_deref() != Some(label_tag) { ctx.ui_set_text(&lp, label_tag); }
    for (i, iid) in item_ids.iter().enumerate() {
        let ip = format!("{}.{}", list, iid);
        let hov = is_open && ctx.ui_node_rect(&ip).zip(cur).map(|(r, p)| inside(r, p)).unwrap_or(false);
        set_color(ctx, &format!("{}.label", ip), if hov { COL_ITEM_HOVER } else { COL_ITEM });
        let cp = format!("{}.check", ip);
        if ctx.ui_visible(&cp) != Some(i == sel) { ctx.ui_set_visible(&cp, i == sel); }
    }
}
