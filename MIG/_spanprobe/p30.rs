#![crate_type="lib"]
extern crate game_ai;
use game_ai::BigGoal as G;
pub fn v(t: G) { match t { G::Line{..}=>(), G::Jungle{..}=>(), G::Epic=>(), G::Serpen=>(), G::Nexus{..}=>(), G::Battle{..}=>(), } }
pub fn h() { let _ = game_ai::plan_legacy::old::defense_nexus::handle_line_defense(1,2,3,4,5,6,7,8,9); }
