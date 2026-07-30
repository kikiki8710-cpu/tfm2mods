use mod_api::*;
struct M;
impl ModExtension for M {
    fn pre_update(&self) {}   // 일부러 빈 시그니처 → E0050 이 올바른 시그니처 알려줌
    fn on_init(&self) {}
}
fn main(){}
