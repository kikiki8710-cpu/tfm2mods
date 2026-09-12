#![allow(unused, dead_code, non_snake_case)]
//! 시그니처 발굴용 — 일부러 인자 수를 틀리게 호출해 rustc 가 실제 시그니처를 찍게 한다.
use game_core::*;
use game_ai::plan_legacy::old::{LineGankCoverPlan, LineGankerPlan};

fn main() {
    let c = LineGankCoverPlan::new();
    let g = LineGankerPlan::new();
    let _ = c.sub_plan();
    let _ = g.sub_plan();
    let _ = g.update();
    let _ = g.is_end();
    let _ = game_ai::plan_legacy::rule_scope::goal_allowed();
    let _ = game_ai::plan_legacy::rule_scope::chat_allowed();
    let _ = game_ai::plan_legacy::rule_scope::position_exists();
    let _ = game_ai::plan_legacy::rule_scope::line_exists();
    let _ = game_ai::plan_legacy::rule_scope::morgard_exists();
    let _ = game_ai::plan_legacy::rule_scope::serpen_exists();
    let _ = game_ai::plan_legacy::rule_scope::plan_allowed();
    let h = game_ai::plan_legacy::handler::LegacyPlanHandler::new();
    h.v3_fall_back_to_passive();
    h.handle_chat();
}
