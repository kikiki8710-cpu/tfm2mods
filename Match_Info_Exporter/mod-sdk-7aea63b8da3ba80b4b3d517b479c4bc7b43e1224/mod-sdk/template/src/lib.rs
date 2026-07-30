use std::sync::atomic::{AtomicBool, Ordering};

use mod_api::*;

struct MyModExtension;

impl ModExtension for MyModExtension {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        // Your mod logic here.
        // Pattern-match on `scene` to react to specific game scenes.
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new("my_mod");
    reg.set_extension(MyModExtension);
    reg
}

declare_mod(init);
