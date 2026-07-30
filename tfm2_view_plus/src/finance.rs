//! finance — 재정 화면 금액 색 구분 (+ 구성 비율). 순수 SDK, .ui 오버라이드 없음.
//! 원본(daram2 finance_view_plus)이 만지는 노드 id: income_detail_box / expense_detail_box /
//! year_expected_in / expected_benefit / year_expected_be / expected_expense / year_expected_ex / *_balance.
//! 규칙: 총잔고·수익 = 흑자 초록 / 적자 빨강. 수입 항목 = 부호대로. 지출 항목 = 반대(늘면 빨강).
//!
//! ⚠ 재정 화면의 정확한 노드 트리(금액 라벨 위치)는 인게임 확인 필요 →
//!   DUMP_TREE=true 로 한 번 덤프해 구조 확정 후 세부 튜닝.

use crate::config;
use mod_api::*;

const DUMP_TREE: bool = false; // 배포 전 false. 인게임서 재정화면 켜고 구조 덤프.

// 색 (제자리 변경이라 타입 임포트 불필요).
const POS: (f32, f32, f32) = (0.32, 0.82, 0.45); // 흑자/이익 초록
const NEG: (f32, f32, f32) = (0.90, 0.32, 0.32); // 적자/손실 빨강

fn set_color(n: &mut Node, rgb: (f32, f32, f32)) -> bool {
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

fn label_text(n: &Node) -> Option<&str> {
    n.runner
        .as_any()
        .downcast_ref::<LabelRunner>()
        .map(|lr| lr.text.as_str())
}

/// 표시 금액 문자열 → 부호. '-' 또는 '▼'/괄호 등 음수표기 감지.
fn sign_of(text: &str) -> Option<i32> {
    let has_digit = text.chars().any(|c| c.is_ascii_digit());
    if !has_digit {
        return None;
    }
    let neg = text.contains('-') || text.contains('▼') || text.trim_start().starts_with('(');
    Some(if neg { -1 } else { 1 })
}

/// 컨테이너 하위의 금액 라벨을 부호대로 색칠. reverse=true 면 지출(늘면 빨강).
fn recolor_amounts(n: &mut Node, reverse: bool) {
    if let Some(t) = label_text(n) {
        if let Some(s) = sign_of(t) {
            let good = if reverse { s < 0 } else { s >= 0 };
            let rgb = if good { POS } else { NEG };
            set_color(n, rgb);
        }
    }
    for c in n.child.iter_mut() {
        recolor_amounts(c, reverse);
    }
}

fn find_mut<'a>(n: &'a mut Node, id: &str) -> Option<&'a mut Node> {
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

fn has_id(n: &Node, id: &str) -> bool {
    n.id == id || n.child.iter().any(|c| has_id(c, id))
}

fn dump(n: &Node, depth: usize, out: &mut String) {
    let t = label_text(n).unwrap_or("");
    out.push_str(&format!(
        "{}#{} [{}] '{}'\n",
        "  ".repeat(depth),
        n.id,
        n.runner.type_name(),
        t
    ));
    for c in n.child.iter() {
        dump(c, depth + 1, out);
    }
}

pub fn tick(ui: &mut GameUI) {
    if !config::get().finance {
        return;
    }
    // 재정 화면 판정: income/expense 상세 박스가 트리에 있으면 재정 화면.
    if !has_id(&ui.root, "income_detail_box") && !has_id(&ui.root, "expense_detail_box") {
        return;
    }

    if DUMP_TREE {
        let mut out = String::new();
        dump(&ui.root, 0, &mut out);
        let _ = crate::log_dump("finance_tree.txt", &out);
    }

    // 수입 상세: 부호대로. 지출 상세: 반대.
    if let Some(box_) = find_mut(&mut ui.root, "income_detail_box") {
        recolor_amounts(box_, false);
    }
    if let Some(box_) = find_mut(&mut ui.root, "expense_detail_box") {
        recolor_amounts(box_, true);
    }
    // 총잔고/수익 헤드라인: 부호대로(흑자 초록/적자 빨강).
    for id in ["year_expected_be", "expected_benefit"] {
        if let Some(n) = find_mut(&mut ui.root, id) {
            recolor_amounts(n, false);
        }
    }
}
