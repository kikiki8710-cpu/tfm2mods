#![allow(unused, dead_code, non_snake_case)]
use game_core::*;
fn main() {
    let a = SingleLaneGame::new();
    let b = DeathMatchGame::new();
    let c = game_ai::plan_legacy::handler::LegacyPlanHandler::new(1usize);
    let d = AbstractGameWithCache::new();
    let e: () = game_ai::plan_legacy::rule_scope::valid_lines();
    let f: () = game_ai::plan_legacy::rule_scope::fallback_line();
    let g: () = game_ai::plan_legacy::rule_scope::main_objective_allowed();
    let h: () = game_ai::plan_legacy::rule_scope::sub_objective_allowed();
    let i: () = game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan();
}
