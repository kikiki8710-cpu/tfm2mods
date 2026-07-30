use mod_api::*;
// Assets 메서드 가설 검증 (E0599=없음, 타입에러=있음+시그니처)
fn a1(a:&mut Assets){ let _=a.get_layout("x"); }
fn a2(a:&mut Assets){ let _=a.layout("x"); }
fn a3(a:&mut Assets){ let _=a.instantiate("x"); }
fn a4(a:&mut Assets){ let _=a.load("x"); }
fn a5(a:&mut Assets){ let _=a.get("x"); }
fn a6(a:&mut Assets){ let _=a.node("x"); }
fn a7(a:&mut Assets){ let _=a.ui("x"); }
fn a8(a:&mut Assets){ let _=a.spawn("x"); }
// Node 의 다른 생성/파싱 연관함수?
fn n2(){ let _=Node::from_str("x"); }
fn n3(){ let _=Node::parse("x"); }
fn n4(){ let _=Node::from_ui("x"); }
fn main(){}
