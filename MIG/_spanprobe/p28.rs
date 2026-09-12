#![crate_type="lib"]
extern crate game_ai;
use game_ai::BigGoal as G;
pub fn v(t: G) { match t {} }
pub fn w(t: G) { match t { G::Line(..)=>(), } }
