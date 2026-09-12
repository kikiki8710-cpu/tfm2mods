#![crate_type="lib"]
extern crate game_ai;
extern crate game_core;
pub fn f<T: game_ai::AbstractEntity>(x: &T) {
    x.is_minion(1,2,3,4,5);
}
pub fn g<T: game_ai::AbstractEntity>(x: &T) {
    x.zzz_probe_nonexistent();
}
