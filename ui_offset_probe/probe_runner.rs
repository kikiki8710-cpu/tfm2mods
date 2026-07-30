extern crate engine_ui;
extern crate engine_core;
use engine_ui::runner as R;
// 1) 필드 공개 여부 (빈 분해 → E0027 전체필드 / "inaccessible" → private)
fn te(r:&R::TextEditRunner){ let R::TextEditRunner{}=r; }
fn co(r:&R::ColorRunner){ let R::ColorRunner{}=r; }
fn la(r:&R::LabelRunner){ let R::LabelRunner{}=r; }
fn bu(r:&R::ButtonRunner){ let R::ButtonRunner{}=r; }
// 2) Default 로 생성 가능?
fn d_te()->R::TextEditRunner{ Default::default() }
fn d_co()->R::ColorRunner{ Default::default() }
// 3) NodeRunner 로 Box 가능? (Node.runner 에 넣으려면)
fn boxable(){ let _b: Box<dyn engine_core::ui::runner::NodeRunner> = Box::new(R::TextEditRunner::default()); }
fn main(){}
