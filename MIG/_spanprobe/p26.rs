#![crate_type="lib"]
extern crate game_ai;
use game_ai::TutorialType as T;
pub fn v(t: T) { match t { T::None=>(), T::First=>(), T::TopSolo=>(), T::Bottom=>(), T::MidSolo=>(), T::MidBottom=>(), } }
