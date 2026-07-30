//! ui — 순수 SDK 노드 헬퍼(벤더링, 공유 ui_kit 미사용). raw 오프셋 없음.
#![allow(dead_code)]
use mod_api::*;
use std::cell::RefCell;
use std::rc::Rc;

// ── 클릭 옵저버 (filter_handler 비소비 필터, 전 기능 공유) ──
// scroll_fix 검증 패턴: 매 프레임 존재확인 후 재등록(Rc 동일성), 절대 소비 안 함.
thread_local! {
    static MY_FILTER: RefCell<Option<Rc<dyn Fn(&UIEvent) -> bool>>> = RefCell::new(None);
    static CLICK_QUEUE: RefCell<Vec<String>> = RefCell::new(Vec::new());
    static FRAME_CLICKS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

/// 매 프레임 post_update 맨 앞에서 호출: 지난 프레임 큐를 이번 프레임 목록으로 옮기고 옵저버 보장.
pub fn start_frame(ui: &mut GameUI) {
    CLICK_QUEUE.with(|q| {
        FRAME_CLICKS.with(|f| {
            let mut q = q.borrow_mut();
            let mut f = f.borrow_mut();
            f.clear();
            f.append(&mut q);
        })
    });
    ensure_observer(ui);
}

fn ensure_observer(ui: &mut GameUI) {
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
                CLICK_QUEUE.with(|q| q.borrow_mut().push(path.clone()));
            }
            false // 관찰만, 절대 소비 안 함
        });
        let handler: Rc<dyn Fn(&mut UIEventHandlerContext<(), UIOutEvent>)> = Rc::new(|_| {});
        ui.filter_handler.push((filter.clone(), handler));
        *slot = Some(filter);
    });
}

fn last_seg(p: &str) -> &str {
    p.rsplit(|c| c == '/' || c == '.').next().unwrap_or(p)
}

/// 이번 프레임에 주어진 노드 id 가 클릭됐나(경로 마지막 세그먼트 일치).
pub fn clicked(id: &str) -> bool {
    FRAME_CLICKS.with(|f| f.borrow().iter().any(|p| last_seg(p) == id))
}
/// 이번 프레임 클릭 경로 전체(행 식별 등 라우팅용).
pub fn click_paths() -> Vec<String> {
    FRAME_CLICKS.with(|f| f.borrow().clone())
}
/// 경로가 세그먼트로 주어진 값을 포함하나.
pub fn path_has_segment(path: &str, seg: &str) -> bool {
    path.split(|c| c == '/' || c == '.').any(|s| s == seg)
}
pub fn last_segment(path: &str) -> &str {
    last_seg(path)
}

// ── 탐색 ──
pub fn find<'a>(n: &'a Node, id: &str) -> Option<&'a Node> {
    if n.id == id {
        return Some(n);
    }
    for c in n.child.iter() {
        if let Some(f) = find(c, id) {
            return Some(f);
        }
    }
    None
}
pub fn find_mut<'a>(n: &'a mut Node, id: &str) -> Option<&'a mut Node> {
    if n.id == id {
        return Some(n);
    }
    for c in n.child.iter_mut() {
        if let Some(f) = find_mut(c, id) {
            return Some(f);
        }
    }
    None
}
pub fn has_id(n: &Node, id: &str) -> bool {
    n.id == id || n.child.iter().any(|c| has_id(c, id))
}
/// 직속 자식 중 주어진 id 라벨의 텍스트(행↔데이터 이름매칭용).
pub fn child_label_text<'a>(n: &'a Node, child_id: &str) -> Option<&'a str> {
    n.child
        .iter()
        .find(|c| c.id == child_id)
        .and_then(label_text)
}
/// 서브트리에서 id 라벨을 찾아 텍스트(중첩 name_slot 대응).
pub fn find_text<'a>(n: &'a Node, id: &str) -> Option<&'a str> {
    find(n, id).and_then(label_text)
}
/// 서브트리에서 id 노드 visible 설정(첫 하나).
pub fn set_visible(n: &mut Node, id: &str, on: bool) -> bool {
    if let Some(t) = find_mut(n, id) {
        t.visible = on;
        true
    } else {
        false
    }
}
/// 서브트리에서 id 노드 전부 visible 설정(같은 id 여러 개 대응).
pub fn set_visible_all(n: &mut Node, id: &str, on: bool) {
    if n.id == id {
        n.visible = on;
    }
    for c in n.child.iter_mut() {
        set_visible_all(c, id, on);
    }
}
/// 재귀 방문(가변).
pub fn for_each_mut(n: &mut Node, f: &mut impl FnMut(&mut Node)) {
    f(n);
    for c in n.child.iter_mut() {
        for_each_mut(c, f);
    }
}
/// 재귀 방문(불변).
pub fn for_each(n: &Node, f: &mut impl FnMut(&Node)) {
    f(n);
    for c in n.child.iter() {
        for_each(c, f);
    }
}

// ── 라벨 read/write ──
pub fn label_text(n: &Node) -> Option<&str> {
    n.runner
        .as_any()
        .downcast_ref::<LabelRunner>()
        .map(|lr| lr.text.as_str())
}
pub fn set_label_text(n: &mut Node, text: &str) -> bool {
    if let Some(lr) = n.runner.as_any_mut().downcast_mut::<LabelRunner>() {
        lr.text = text.to_string();
        true
    } else {
        false
    }
}
pub fn set_label_color(n: &mut Node, rgb: (f32, f32, f32)) -> bool {
    if let Some(lr) = n.runner.as_any_mut().downcast_mut::<LabelRunner>() {
        let c = &mut lr.style.normal.color;
        c.r = rgb.0;
        c.g = rgb.1;
        c.b = rgb.2;
        true
    } else {
        false
    }
}
/// 라벨 텍스트 + 색을 한 번에.
pub fn set_label(n: &mut Node, text: &str, rgb: (f32, f32, f32)) {
    set_label_text(n, text);
    set_label_color(n, rgb);
}

// ── 능력치 값(0~100 가정) → 크기별 색상. ⚠ 실제 스탯 범위는 인게임 확인 후 튜닝. ──
pub fn stat_color(v: usize) -> (f32, f32, f32) {
    match v {
        0..=29 => (0.86, 0.36, 0.36),   // 낮음 빨강
        30..=49 => (0.90, 0.62, 0.36),  // 주황
        50..=69 => (0.85, 0.82, 0.45),  // 노랑
        70..=84 => (0.55, 0.80, 0.50),  // 연초록
        _ => (0.35, 0.85, 0.55),        // 높음 초록
    }
}
