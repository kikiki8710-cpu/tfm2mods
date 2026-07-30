use mod_api::*;
// Node 연관함수 가설
fn n_a(t:&NodeTemplate){ let _=Node::from_template(t); }
fn n_b(t:&NodeTemplate){ let _=Node::build(t); }
fn n_c(t:&NodeTemplate){ let _=Node::instantiate(t); }
fn n_d(t:&NodeTemplate){ let _=Node::render(t); }
fn n_e(t:&NodeTemplate){ let _=Node::spawn(t); }
// 변환 트레잇
fn c_from(t:NodeTemplate){ let _:Node = Node::from(t); }
fn c_into(t:NodeTemplate){ let _:Node = t.into(); }
fn c_fromref(t:&NodeTemplate){ let _:Node = Node::from(t); }
// NodeTemplate 메서드 (assets 인자 가능성)
fn t_a(t:&NodeTemplate){ let _=t.realize(); }
fn t_b(t:&NodeTemplate){ let _=t.expand(); }
fn t_c(t:&NodeTemplate){ let _=t.render(); }
fn t_d(t:&NodeTemplate){ let _=t.spawn(); }
fn t_e(t:&NodeTemplate){ let _=t.into_node(); }
fn main(){}
