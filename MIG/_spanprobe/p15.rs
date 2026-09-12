#![crate_type="lib"]
extern crate game_ai;
extern crate game_core;
pub fn t(a: &mut u32, b: &mut u32) { std::mem::swap(a, b) }
pub fn u(x: &GameContext, g: BigGoal) -> bool { game_ai::plan_legacy::rule_scope::goal_allowed(x, g) }
