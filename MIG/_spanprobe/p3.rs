#![crate_type="lib"]
extern crate game_core;
use game_core::simulation::entity::AbstractEntity;
pub fn f(x: &dyn AbstractEntity) {
    x.is_minion(1,2,3,4,5);
}
