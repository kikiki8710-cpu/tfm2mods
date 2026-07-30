use mod_api::*;
extern crate engine_ui;
use engine_ui::runner as R;
// 필요한 runner 전부 Default 생성 + mod_api Node.runner 에 꽂기
fn build_text() -> Node { let mut n = Node::new("t"); n.runner = Box::new(R::TextEditRunner::default()); n }
fn build_color() -> Node { let mut n = Node::new("c"); n.runner = Box::new(R::ColorRunner::default()); n }
fn build_label() -> Node { let mut n = Node::new("l"); n.runner = Box::new(R::LabelRunner::default()); n }
fn build_button()-> Node { let mut n = Node::new("b"); n.runner = Box::new(R::ButtonRunner::default()); n }
fn build_scroll()-> Node { let mut n = Node::new("s"); n.runner = Box::new(R::ScrollViewRunner::default()); n }
fn build_image() -> Node { let mut n = Node::new("i"); n.runner = Box::new(R::ImageRunner::default()); n }
// 자식 중첩 + push
fn nest() -> Node { let mut p = build_color(); p.child.push(build_label()); p.child.push(build_text()); p }
fn main(){}
