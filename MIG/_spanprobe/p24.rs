#![crate_type="lib"]
extern crate game_ai;
use game_ai::GameContext;
pub fn f(c: &GameContext) {
    let _: () = c.tutorial_type;
    let _: () = c.tutorial;
    let _: () = c.tutorial_mode;
}
