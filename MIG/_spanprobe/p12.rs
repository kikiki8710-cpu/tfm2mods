#![crate_type="lib"]
extern crate game_ai;
use game_ai::plan_legacy as pl;
pub fn a() { let _ = pl::sub_plan::SubPlan::merge(1,2,3,4,5); }
pub fn b() { let _ = pl::rule_scope::goal_allowed(1,2,3,4,5); }
pub fn c() { let _ = pl::rule_scope::RuleScope::goal_allowed(1,2,3,4,5); }
pub fn d() { let _ = pl::old::defense_nexus::morgard_exists(1,2,3,4,5); }
pub fn e() { let _ = pl::handler::chat::handle_chat(1,2,3,4,5); }
