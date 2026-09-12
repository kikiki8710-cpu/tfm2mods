#![crate_type="lib"]
extern crate game_ai;
use game_ai::plan_legacy as pl;
pub fn a() { let _: () = pl::rule_scope::position_exists; }
pub fn b() { let _: () = pl::rule_scope::valid_lines; }
pub fn c() { let _: () = pl::old::EpicHuntAndBattlePlan::sub_plan; }
pub fn d() { let _: () = pl::rule_scope::chat_allowed; }
pub fn e() { let _: () = pl::rule_scope::plan_allowed; }
