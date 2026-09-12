#![crate_type="lib"]
extern crate game_ai;
use game_ai::plan_legacy as pl;
use game_ai::{GameContext, Position, BigGoal, Chat, LineType};
use game_ai::plan_legacy::types::BigPlan;
use pl::sub_plan::SubPlan;

#[inline(never)] pub fn f1(c:&GameContext, p:Position)->bool { pl::rule_scope::position_exists(c,p) }
#[inline(never)] pub fn f2(c:&GameContext)->&'static [LineType] { pl::rule_scope::valid_lines(c) }
#[inline(never)] pub fn f3(c:&GameContext, g:BigGoal)->bool { pl::rule_scope::goal_allowed(c,g) }
#[inline(never)] pub fn f4(a:&mut SubPlan, b:SubPlan) { SubPlan::merge(a,b) }
#[inline(never)] pub fn f5(c:&GameContext, ch:&Chat)->bool { pl::rule_scope::chat_allowed(c,ch) }
#[inline(never)] pub fn f6(c:&GameContext, p:BigPlan)->bool { pl::rule_scope::plan_allowed(c,&p) }
#[inline(never)] pub fn f7(c:&GameContext)->bool { pl::rule_scope::serpen_exists(c) }
