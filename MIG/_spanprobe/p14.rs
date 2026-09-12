#![crate_type="lib"]
extern crate game_ai;
use game_ai::plan_legacy as pl;
use pl::sub_plan::SubPlan;

#[inline(never)]
pub fn call_merge(a: &mut SubPlan, b: SubPlan) { SubPlan::merge(a, b) }
