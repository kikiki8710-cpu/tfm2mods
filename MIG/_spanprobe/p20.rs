#![crate_type="lib"]
extern crate game_ai;
use game_ai::{TutorialType, Chat};
#[inline(never)] pub fn pc(t: &TutorialType) -> usize { t.player_count() }
pub fn ch() {
    let _ = Chat::SerpenPrepare(1,2,3,4,5);
    let _ = Chat::MorgardPrepare(1,2,3,4,5);
    let _ = Chat::SerpenPrepare { zzz: 1 };
}
