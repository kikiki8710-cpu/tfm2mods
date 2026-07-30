// 주입 API 심문: Assets / GameUI / Node 구조 + 레이아웃 인스턴스화 함수 유무
use mod_api::*;
// 빈 패턴 분해 → rustc 가 전체 필드 나열 (E0026/E0027) 또는 private 보고
fn assets_fields(a: &Assets) { let Assets {} = a; }
fn gameui_fields(u: &GameUI) { let GameUI {} = u; }
fn node_fields(n: &Node) { let Node {} = n; }
fn main() {}
