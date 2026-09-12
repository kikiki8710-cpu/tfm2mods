#![crate_type="lib"]
extern crate game_core;
pub fn f(e: &game_core::EntityType) {
    e.is_minion(1,2,3);
    e.is_any_type_minion(1,2,3);
    e.is_jungle(1,2,3);
    e.is_any_jungle(1,2,3);
    e.is_epic(1,2,3);
    e.is_tower(1,2,3);
}
