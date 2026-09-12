#![crate_type="lib"]
extern crate game_ai;
use game_ai::{GameContext, BigGoal};
use game_ai::plan_legacy as pl;
#[inline(never)]
pub fn t(a: &mut u32, b: &mut u32) { std::mem::swap(a, b) }
#[inline(never)]
pub fn u(x: &GameContext, g: BigGoal) -> bool { pl::rule_scope::goal_allowed(x, g) }
#[inline(never)]
pub fn v(x: &GameContext) -> bool { pl::rule_scope::morgard_exists(x) }
