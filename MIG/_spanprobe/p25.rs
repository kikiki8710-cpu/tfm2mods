#![crate_type="lib"]
extern crate game_ai;
use game_ai::{TutorialType as T, GameContext};
pub fn v(t: T) { match t { T::None=>(), T::First=>(), T::TopSolo=>(), } }
pub fn f(c: &GameContext) { let _: () = c.tutorial_type; }
