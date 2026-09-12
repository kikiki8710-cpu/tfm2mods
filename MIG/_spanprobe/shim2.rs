#![crate_type="lib"]
#![allow(unused_imports)]
extern crate game_ai;
pub use game_ai::plan_legacy::*;
pub mod old { pub use game_ai::plan_legacy::old::*; }
