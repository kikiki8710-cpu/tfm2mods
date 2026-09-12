#![crate_type="lib"]
extern crate game_ai;
use game_ai::{TutorialType, Chat};
#[inline(never)] pub fn pc(t: &TutorialType) -> usize { t.player_count() }
pub fn ch() {
    let _: () = Chat::SerpenPrepare;
    let _: () = Chat::MorgardPrepare;
}
