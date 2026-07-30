use mod_api::*;
fn t_child(t:&NodeTemplate){ let _:()=t.child; }       // child 타입
fn t_prop(t:&NodeTemplate){ let _:()=t.property; }      // property 타입
fn t_name(t:&NodeTemplate){ let _:()=t.name; }          // name 타입
fn t_style(t:&NodeTemplate){ let _:()=t.style_path; }   // style_path 타입
fn t_clone(t:&NodeTemplate){ let _:NodeTemplate=t.clone(); } // Clone?
fn gm(a:&mut Assets){ let _:Option<&mut NodeTemplate>=a.get_mut::<NodeTemplate>("x"); } // get_mut 반환
fn main(){}
