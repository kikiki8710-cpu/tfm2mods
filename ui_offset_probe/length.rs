use mod_api::*;
// mod_api 가 Length 를 노출하나?
fn reexport() { let _: Length = todo!(); }
// Length 변형 탐색
fn variants(n:&Node){
    let l = &n.layout.normal.x;
    let _ = matches!(l, Length::Px(_));
    let _ = matches!(l, Length::Percent(_));
    let _ = matches!(l, Length::Pixel(_));
    let _ = matches!(l, Length::Point(_));
    let _ = matches!(l, Length::Auto);
}
fn main(){}
