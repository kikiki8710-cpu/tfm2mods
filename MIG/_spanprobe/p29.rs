#![crate_type="lib"]
extern crate game_ai;
use game_ai::BigGoal as G;
pub fn v(t: G) { match t { G::Line{..}=>(), G::Jungle{..}=>(), G::Epic=>(), } }
