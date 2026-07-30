use mod_api::*;
fn nt(a:&Assets){
  let t = a.get::<NodeTemplate>("x").unwrap();
  // 인스턴스화 메서드 가설
  let _=t.instantiate();
  let _=t.build();
  let _=t.create();
  let _=t.to_node();
  let _=t.node();
  let _=t.root();
  let _=t.make();
  let _=t.clone();
}
fn main(){}
