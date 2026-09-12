#![crate_type="lib"]
extern crate game_ai;
use game_ai::TutorialType;
#[inline(never)] pub fn pc(t: &TutorialType) -> usize { t.player_count() }
