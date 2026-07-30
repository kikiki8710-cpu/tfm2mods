use mod_api::*;
struct M;
impl ModExtension for M {
    // 후보 훅들 — 트레잇 멤버 아니면 E0407, 맞으면 통과
    fn pre_update(&self, _s:&mut Scene, _u:&mut GameUI, _a:&mut Assets, _d:f32) {}
    fn on_load(&self, _a:&mut Assets) {}
    fn on_init(&self, _c:&GameCtx) {}
    fn on_assets_loaded(&self, _a:&mut Assets) {}
    fn on_ui_build(&self, _u:&mut GameUI) {}
    fn before_ui(&self, _u:&mut GameUI) {}
    fn on_scene_load(&self, _s:&mut Scene) {}
    fn post_load(&self, _a:&mut Assets) {}
}
fn main(){}
