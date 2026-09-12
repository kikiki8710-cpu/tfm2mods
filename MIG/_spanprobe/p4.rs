#![crate_type="lib"]
extern crate game_core;
use game_core::AbstractEntity;
pub fn f(x: &dyn AbstractEntity) {
    x.attack_type(1,2,3,4,5);
    x.is_mionion(1);
}
