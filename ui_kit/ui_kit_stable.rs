// ui_kit_stable.rs — ★0.6.0+ stable ABI 용 UI 키트 (클래식 `ui_kit.rs` 의 후속).
// ───────────────────────────────────────────────────────────────────────────
// 왜: 0.6.0 부터 클래식 SDK(mod_api::Node/GameUI) 가 없다. stable API 는 노드를 **경로 문자열**
//   ("body.popup.close") 로만 다루므로, 클래식 ui_kit 의 `find(&root,"id")` 식 API 를
//   "id → 경로" 인덱스 위에 같은 이름으로 다시 제공한다(모드 코드 이식 최소화).
// 사용: `#[path = r"C:\tfm2mods\ui_kit\ui_kit_stable.rs"] mod uk;`  → `uk::label_set(ctx, "status", "…")`
// 규칙:
//   · 모든 함수는 `ctx: &mut StableClient` + 노드 **id** 를 받는다(경로가 아니라 id — 클래식과 동일 감각).
//   · id→경로 인덱스는 `index_rebuild(ctx, root)` 로 만든다(우리 조각 스폰 직후 1회 + 조회 실패 시 자동 1회).
//     ⚠전체 트리 DFS 는 비싸다(ui_child_names 호출 수 = 노드 수). 루트를 우리 조각(예 "body.draft_root")
//     으로 좁혀서 돌릴 것. 게임 노드가 필요하면 그 서브트리만 별도 rebuild.
//   · 색/크기/좌표 = `.ui` 속성 문자열(`ui_set_properties`) — 매 프레임 같은 값을 다시 쓰지 말 것
//     (파서 비용). `*_if_changed` 캐시가 같은 값 재적용을 걸러 준다.
//   · 클릭 = `route(ctx, id, handler)`: (경로, "") 에 1회 등록 — 호스트 등록은 영구·해제 불가이므로
//     같은 id 에 두 번 등록하지 않게 `ROUTED` 로 가드. 트리가 재생성돼도(씬 재진입) 경로가 같으면 유지.
//   · 클래식에만 있던 raw 러너 필드 접근(runner_rd/wr_f32·scroll_get·raise_to_top·hide_hover_tooltips)은
//     **없다**. 대체: 높이 = `set_layout_size` / 스크롤 = `scroll_ratio`(rect 계산) / z 순서 = 스폰 위치+z 속성.
// 검증: 2026-09-16 신설(미검증). 인게임 확인 항목 = REPORT\tfm2_draft_overlay\02_구현정보.md 0.6.0 절.

#![allow(dead_code)]
use mod_api_stable::StableClient;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rgba(pub u32); // 0xrrggbbaa
impl Rgba {
    pub fn hex(v: u32) -> Self { Rgba(v) }
    pub fn rgb8(r: u8, g: u8, b: u8) -> Self { Rgba(((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xff) }
    pub fn css(&self) -> String { format!("#{:08x}", self.0) }
}

// ── id → 경로 인덱스 ─────────────────────────────────────────────────────────
static INDEX: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
static ROUTED: Mutex<Option<HashSet<String>>> = Mutex::new(None);
static PROP_CACHE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
static REBUILD_BUDGET: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn idx() -> std::sync::MutexGuard<'static, Option<HashMap<String, String>>> {
    INDEX.lock().unwrap_or_else(|e| e.into_inner())
}

/// `root` 서브트리(경로)를 DFS 해 id→경로 인덱스를 채운다(기존 항목은 덮어씀). 반환 = 등록 수.
pub fn index_rebuild(ctx: &StableClient<'_>, root: &str) -> usize {
    let mut map: HashMap<String, String> = idx().take().unwrap_or_default();
    let mut n = 0usize;
    let mut stack: Vec<String> = vec![root.to_string()];
    let mut guard = 0usize;
    while let Some(p) = stack.pop() {
        guard += 1;
        if guard > 20_000 { break; } // 폭주 방지
        for name in ctx.ui_child_names(&p) {
            let child = if p.is_empty() { name.clone() } else { format!("{}.{}", p, name) };
            map.insert(name, child.clone());
            n += 1;
            stack.push(child);
        }
    }
    *idx() = Some(map);
    n
}

/// 인덱스를 비운다(씬 전환·우리 조각 제거 시).
pub fn index_clear() {
    *idx() = None;
    *PROP_CACHE.lock().unwrap_or_else(|e| e.into_inner()) = None;
}

/// id → 경로. 인덱스에 없거나 경로가 죽었으면 `fallback_root` 아래를 1회 재색인(프레임당 예산 1회).
pub fn find(ctx: &StableClient<'_>, id: &str) -> Option<String> {
    if let Some(m) = idx().as_ref() {
        if let Some(p) = m.get(id) {
            if ctx.ui_exists(p) { return Some(p.clone()); }
        }
    }
    // ★2026-09-17: 경로 통과 — `id` 가 이미 전체 경로("a.b.c")면 그대로 쓴다(serpen/champ_pos_lock 이 경로로 호출).
    if id.contains('.') && ctx.ui_exists(id) { return Some(id.to_string()); }
    None
}

/// 조회 실패 시 `root` 를 재색인하고 다시 찾는다(프레임 시작에서 `frame_begin` 으로 예산 리셋).
pub fn find_or_rebuild(ctx: &StableClient<'_>, id: &str, root: &str) -> Option<String> {
    if let Some(p) = find(ctx, id) { return Some(p); }
    if REBUILD_BUDGET.fetch_update(std::sync::atomic::Ordering::Relaxed, std::sync::atomic::Ordering::Relaxed,
        |b| if b > 0 { Some(b - 1) } else { None }).is_ok() {
        index_rebuild(ctx, root);
        return find(ctx, id);
    }
    None
}

/// 매 프레임 첫 줄에서 호출: 재색인 예산 리셋.
pub fn frame_begin() { REBUILD_BUDGET.store(1, std::sync::atomic::Ordering::Relaxed); }

pub fn exists(ctx: &StableClient<'_>, id: &str) -> bool { find(ctx, id).is_some() }

// ── 가시성 ───────────────────────────────────────────────────────────────────
pub fn get_visible(ctx: &StableClient<'_>, id: &str) -> Option<bool> { find(ctx, id).and_then(|p| ctx.ui_visible(&p)) }
pub fn set_visible(ctx: &mut StableClient<'_>, id: &str, on: bool) -> bool {
    match find(ctx, id) {
        Some(p) => { if ctx.ui_visible(&p) == Some(on) { return true; } ctx.ui_set_visible(&p, on) }
        None => false,
    }
}
pub fn show(ctx: &mut StableClient<'_>, id: &str) -> bool { set_visible(ctx, id, true) }
pub fn hide(ctx: &mut StableClient<'_>, id: &str) -> bool { set_visible(ctx, id, false) }

// ── 라벨 ─────────────────────────────────────────────────────────────────────
pub fn label_get(ctx: &StableClient<'_>, id: &str) -> Option<String> { find(ctx, id).and_then(|p| ctx.ui_text(&p)) }
/// 같은 텍스트면 쓰지 않는다(호스트 호출 절약).
pub fn label_set(ctx: &mut StableClient<'_>, id: &str, text: &str) -> bool {
    let Some(p) = find(ctx, id) else { return false };
    if ctx.ui_text(&p).as_deref() == Some(text) { return true; }
    ctx.ui_set_text(&p, text)
}

// ── 속성(.ui 문법) — 값이 바뀔 때만 적용 ─────────────────────────────────────
pub fn set_props_if_changed(ctx: &mut StableClient<'_>, id: &str, key: &str, value: &str) -> bool {
    let Some(p) = find(ctx, id) else { return false };
    let ck = format!("{}|{}", p, key);
    {
        let mut c = PROP_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        let m = c.get_or_insert_with(HashMap::new);
        if m.get(&ck).map(|v| v == value).unwrap_or(false) { return true; }
        m.insert(ck, value.to_string());
    }
    ctx.ui_set_properties(&p, &format!("{}: {};", key, value))
}
pub fn label_set_color(ctx: &mut StableClient<'_>, id: &str, c: Rgba) -> bool { set_props_if_changed(ctx, id, "color", &c.css()) }
pub fn rect_set_color(ctx: &mut StableClient<'_>, id: &str, c: Rgba) -> bool { set_props_if_changed(ctx, id, "color", &c.css()) }
pub fn label_set_size(ctx: &mut StableClient<'_>, id: &str, size: f32) -> bool { set_props_if_changed(ctx, id, "font_size", &format!("{}", size)) }
/// w/h 중 0 이하는 건드리지 않는다(클래식 set_layout_size 와 동일 계약).
pub fn set_layout_size(ctx: &mut StableClient<'_>, id: &str, w: f32, h: f32) -> bool {
    let mut ok = true;
    if w > 0.0 { ok &= set_props_if_changed(ctx, id, "width", &format!("{:.1}px", w)); }
    if h > 0.0 { ok &= set_props_if_changed(ctx, id, "height", &format!("{:.1}px", h)); }
    ok
}
pub fn set_layout_xy(ctx: &mut StableClient<'_>, id: &str, x: f32, y: f32) -> bool {
    set_props_if_changed(ctx, id, "x", &format!("{:.1}px", x)) & set_props_if_changed(ctx, id, "y", &format!("{:.1}px", y))
}
pub fn set_disabled(ctx: &mut StableClient<'_>, id: &str, on: bool) -> bool { set_props_if_changed(ctx, id, "disable", if on { "true" } else { "false" }) }
/// 이미지 러너 소스 교체(`source: "asset/…#sheet"` + 선택적 `rect_tag`).
pub fn image_set_source(ctx: &mut StableClient<'_>, id: &str, source: &str, rect_tag: Option<&str>) -> bool {
    let mut ok = set_props_if_changed(ctx, id, "source", &format!("\"{}\"", source));
    if let Some(t) = rect_tag { ok &= set_props_if_changed(ctx, id, "rect_tag", &format!("\"{}\"", t)); }
    ok
}
/// 챔피언 얼굴 아이콘(게임 자체 아이콘 계산). 실패(=미지원 호스트/없는 챔프)면 false.
pub fn image_set_champion_face(ctx: &mut StableClient<'_>, id: &str, champ: &str, w: f32, h: f32) -> bool {
    let Some(p) = find(ctx, id) else { return false };
    let ck = format!("{}|face", p);
    {
        let mut c = PROP_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        let m = c.get_or_insert_with(HashMap::new);
        if m.get(&ck).map(|v| v == champ).unwrap_or(false) { return true; }
        m.insert(ck, champ.to_string());
    }
    ctx.ui_set_champion_icon(&p, champ, w, h, 2.0)
}

// ── 위젯 상태 ────────────────────────────────────────────────────────────────
pub fn textedit_get(ctx: &StableClient<'_>, id: &str) -> Option<String> { find(ctx, id).and_then(|p| ctx.ui_text_edit_text(&p)) }
pub fn textedit_set(ctx: &mut StableClient<'_>, id: &str, text: &str) -> bool { match find(ctx, id) { Some(p) => ctx.ui_set_text_edit_text(&p, text), None => false } }
pub fn toggle_get(ctx: &StableClient<'_>, id: &str) -> Option<bool> { find(ctx, id).and_then(|p| ctx.ui_checkbox_selected(&p)) }
pub fn toggle_set(ctx: &mut StableClient<'_>, id: &str, on: bool) -> bool { match find(ctx, id) { Some(p) => ctx.ui_set_checkbox_selected(&p, on), None => false } }
pub fn kind(ctx: &StableClient<'_>, id: &str) -> Option<String> { find(ctx, id).and_then(|p| ctx.ui_runner_name(&p)) }
/// (x, y, w, h) 화면 좌표.
pub fn node_rect(ctx: &StableClient<'_>, id: &str) -> Option<(f32, f32, f32, f32)> { find(ctx, id).and_then(|p| ctx.ui_node_rect(&p)) }
pub fn contents_rect(ctx: &StableClient<'_>, id: &str) -> Option<(f32, f32, f32, f32)> { find(ctx, id).and_then(|p| ctx.ui_contents_rect(&p)) }
/// 스크롤 비율(0..1) 추정 = (노드 y − 콘텐츠 y) / (콘텐츠 h − 노드 h). 콘텐츠가 뷰포트보다 작으면 0.
pub fn scroll_ratio(ctx: &StableClient<'_>, id: &str) -> Option<f32> {
    let (nx, ny, nw, nh) = node_rect(ctx, id)?;
    let (_cx, cy, _cw, ch) = contents_rect(ctx, id)?;
    let _ = (nx, nw);
    if ch <= nh + 1.0 { return Some(0.0); }
    Some(((ny - cy) / (ch - nh)).clamp(0.0, 1.0))
}

// ── 클릭 라우팅 ─────────────────────────────────────────────────────────────
pub type ClickFn = Arc<dyn Fn() + Send + Sync + 'static>;
/// (경로,"") 에 1회 등록. 이미 등록된 경로면 no-op. 반환 = 등록됐거나 이미 있음.
pub fn route(ctx: &mut StableClient<'_>, id: &str, f: ClickFn) -> bool {
    let Some(p) = find(ctx, id) else { return false };
    {
        let mut r = ROUTED.lock().unwrap_or_else(|e| e.into_inner());
        let s = r.get_or_insert_with(HashSet::new);
        if s.contains(&p) { return true; }
        s.insert(p.clone());
    }
    let f2 = f.clone();
    ctx.ui_register_click(&p, "", move |_c| { f2(); })
}
/// 여러 라우트를 한 번에(클래식 `ensure_clicks` 대응).
pub fn ensure_clicks(ctx: &mut StableClient<'_>, routes: &[(String, ClickFn)]) -> usize {
    routes.iter().filter(|(id, f)| route(ctx, id, f.clone())).count()
}

// ── 스폰/제거 ────────────────────────────────────────────────────────────────
/// `.ui` 소스를 parent 경로 아래에 스폰하고 그 서브트리를 색인한다. 반환 = 스폰된 루트 경로.
pub fn spawn_source(ctx: &mut StableClient<'_>, parent_path: &str, root_id: &str, source: &str) -> Option<String> {
    if !ctx.ui_spawn_source(parent_path, source) { return None; }
    let root = if parent_path.is_empty() { root_id.to_string() } else { format!("{}.{}", parent_path, root_id) };
    if !ctx.ui_exists(&root) { return None; }
    idx().get_or_insert_with(HashMap::new).insert(root_id.to_string(), root.clone());
    index_rebuild(ctx, &root);
    Some(root)
}
pub fn remove(ctx: &mut StableClient<'_>, path: &str) -> bool { let r = ctx.ui_remove_node(path); index_clear(); r }

/// 디버그: 서브트리 덤프(id:kind).
pub fn dump_tree(ctx: &StableClient<'_>, path: &str, depth: usize, out: &mut String, max_depth: usize) {
    if depth > max_depth { return; }
    for name in ctx.ui_child_names(path) {
        let child = if path.is_empty() { name.clone() } else { format!("{}.{}", path, name) };
        let k = ctx.ui_runner_name(&child).unwrap_or_default();
        let v = ctx.ui_visible(&child).map(|b| if b { "" } else { " (hidden)" }).unwrap_or("");
        out.push_str(&format!("{}{}:{}{}\n", "  ".repeat(depth), name, k, v));
        dump_tree(ctx, &child, depth + 1, out, max_depth);
    }
}

// ── 커서 → 게임 가상좌표(1920×1080) ─────────────────────────────────────────
// 클래식 `cursor_to_game` 이식(2026-09-17, serpen 포팅 때 추가). 포그라운드 창이 이 프로세스 것일 때만 Some.
// 게임은 16:9 콘텐츠를 클라이언트 안에 비율 유지로 맞추고 남는 공간을 레터박스로 둔다 → 여백 제외 영역만 매핑.
#[link(name = "user32")]
extern "system" {
    fn GetCursorPos(p: *mut [i32; 2]) -> i32;
    fn GetForegroundWindow() -> usize;
    fn GetWindowThreadProcessId(h: usize, pid: *mut u32) -> u32;
    fn ScreenToClient(h: usize, p: *mut [i32; 2]) -> i32;
    fn GetClientRect(h: usize, r: *mut [i32; 4]) -> i32;
    fn GetAsyncKeyState(vk: i32) -> i16;
}
#[link(name = "kernel32")]
extern "system" { fn GetCurrentProcessId() -> u32; }
pub fn cursor_ui() -> Option<(f32, f32)> {
    unsafe {
        let h = GetForegroundWindow(); if h == 0 { return None; }
        let mut pid = 0u32; GetWindowThreadProcessId(h, &mut pid); if pid != GetCurrentProcessId() { return None; }
        let mut p = [0i32; 2]; if GetCursorPos(&mut p) == 0 || ScreenToClient(h, &mut p) == 0 { return None; }
        let mut r = [0i32; 4]; if GetClientRect(h, &mut r) == 0 { return None; }
        let (cw, ch) = ((r[2] - r[0]) as f32, (r[3] - r[1]) as f32);
        if cw < 1.0 || ch < 1.0 { return None; }
        let target = 1920.0 / 1080.0;
        let (gw, gh, ox, oy) = if cw / ch > target { let w = ch * target; (w, ch, (cw - w) * 0.5, 0.0) } else { let h2 = cw / target; (cw, h2, 0.0, (ch - h2) * 0.5) };
        Some(((p[0] as f32 - ox) / gw * 1920.0, (p[1] as f32 - oy) / gh * 1080.0))
    }
}
/// 왼쪽 마우스 버튼 눌림(폴링).
pub fn lbutton_down() -> bool { unsafe { (GetAsyncKeyState(0x01) as u16 & 0x8000) != 0 } }
/// rect(x,y,w,h) 안에 커서가 있는가.
pub fn cursor_in(rect: (f32, f32, f32, f32)) -> bool {
    let Some((mx, my)) = cursor_ui() else { return false };
    let (x, y, w, h) = rect;
    w > 0.0 && h > 0.0 && mx >= x && mx <= x + w && my >= y && my <= y + h
}
