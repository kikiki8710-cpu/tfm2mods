use mod_api::*;
// get 의 시그니처/반환 폭로
fn g_ret(a:&Assets){ let _:()=a.get::<Node>("x"); }          // 반환형 폭로
fn g_node(a:&Assets){ let _ = a.get::<Node>("x"); }          // T=Node 허용되나
fn gm_ret(a:&mut Assets){ let _:()=a.get_mut::<Node>("x"); } // get_mut 반환형
fn main(){}
