#![crate_type="lib"]
extern crate game_ai;
use game_ai::plan_legacy as pl;
pub fn a() { let _: () = pl::sub_plan::SubPlan::merge; }
pub fn b() { let _: () = pl::rule_scope::goal_allowed; }
pub fn c() { let _: () = pl::rule_scope::morgard_exists; }
pub fn d() { let _: () = pl::old::EpicHuntAndBattlePlan::sub_plan; }
