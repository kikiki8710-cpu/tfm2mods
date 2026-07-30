use mod_api::*;
// 타입 폭로: `let _:() = expr;` → "expected (), found <TYPE>"
fn t_runner(n:&Node){ let _:()=n.runner; }
fn t_layout(n:&Node){ let _:()=n.layout; }
fn t_rect(n:&Node){ let _:()=n.rect; }
fn t_crect(n:&Node){ let _:()=n.contents_rect; }
fn t_child(n:&Node){ let _:()=n.child; }
fn t_rendermap(u:&GameUI){ let _:()=u.render_map; }
// Node 가 Default 로 생성 가능한가?
fn n_default()->Node{ Default::default() }
// Node::new 류?
fn n_new(){ let _ = Node::new(); }
fn main(){}
