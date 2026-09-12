#![crate_type="lib"]
extern crate game_ai;
use game_ai::{TutorialType, GameContext};
pub fn v(t: TutorialType) { match t {} }
pub fn f(c: &GameContext) { let _ = c.zzz_probe; }
