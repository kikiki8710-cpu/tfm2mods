#![crate_type="lib"]
extern crate game_ai;
#[inline(never)]
pub fn f() { game_ai::install(); }
