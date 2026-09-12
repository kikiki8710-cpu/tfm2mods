#![crate_type="lib"]
extern crate game_core;
use game_core::AbstractEntity;
pub fn f<T: AbstractEntity>(x: &T) {
    x.is_minion(1,2,3,4,5);
    x.is_any_type_minion(1,2,3,4,5);
    x.is_top_minion(1,2,3,4,5);
    x.is_mid_minion(1,2,3,4,5);
    x.is_bottom_minion(1,2,3,4,5);
    x.is_epic(1,2,3,4,5);
    x.attack_type(1,2,3,4,5);
}
