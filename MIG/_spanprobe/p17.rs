#![crate_type="lib"]
extern crate game_ai;
use game_ai::plan_legacy::rule_scope as rs;
pub fn z() {
    let _ = rs::valid_lines(1,2,3,4,5,6,7,8,9);
    let _ = rs::line_exists(1,2,3,4,5,6,7,8,9);
    let _ = rs::fallback_line(1,2,3,4,5,6,7,8,9);
    let _ = rs::position_exists(1,2,3,4,5,6,7,8,9);
    let _ = rs::morgard_exists(1,2,3,4,5,6,7,8,9);
    let _ = rs::serpen_exists(1,2,3,4,5,6,7,8,9);
    let _ = rs::steal_target_allowed(1,2,3,4,5,6,7,8,9);
    let _ = rs::steal_action_allowed(1,2,3,4,5,6,7,8,9);
    let _ = rs::main_objective_allowed(1,2,3,4,5,6,7,8,9);
    let _ = rs::sub_objective_allowed(1,2,3,4,5,6,7,8,9);
    let _ = rs::goal_allowed(1,2,3,4,5,6,7,8,9);
    let _ = rs::plan_allowed(1,2,3,4,5,6,7,8,9);
    let _ = rs::line_from_code(1,2,3,4,5,6,7,8,9);
    let _ = rs::push_line_code_allowed(1,2,3,4,5,6,7,8,9);
    let _ = rs::play_code_allowed(1,2,3,4,5,6,7,8,9);
    let _ = rs::chat_allowed(1,2,3,4,5,6,7,8,9);
}
