//! UI 분류와 표시 포지션. 저장·AI에서 사용하는 TACTICS 순서는 바꾸지 않는다.
use crate::tactics;

pub const ROWS: usize = 17;
pub struct Category {
    pub name: &'static str,
    pub ids: &'static [&'static str],
}
pub const CATEGORIES: &[Category] = &[
    Category { name: "전투·생존", ids: &[
        "combat_retreat_hp",
        "line_chase_retreat_hp",
        "line_chase_target_guard_hp",
        "crowd_retreat_weight",
        "combat_avoid_hp",
        "combat_avoid_count_margin",
        "self_risk_ignore_hp",
        "lowhp_ult_bonus_low",
        "kill_bonus_cap",
        "battle_risk_retreat_threshold",
        "battle_move_candidate_penalty",
        "tracked_target_chase_bonus",
        "champion_ultimate_waste_penalty",
    ] },
    Category { name: "라인·이동", ids: &[
        "lane_retreat_final",
        "minion_damage_ratio",
        "lane_assign_hp",
        "lane_opportunity_own_min",
        "support_minion_block",
        "bot_duo_partner",
        "hazard_avoid_padding",
        "line_defense_approval_profile",
        "lane_attack_move_allowance",
        "mia_notice_caution_profile",
        "lane_wait_front_minion_distance",
        "purchase_recall_walk_ticks",
        "purchase_recall_threat_radius",
        "purchase_recall_threat_memory",
        "same_lane_partner_radius",
        "support_zero_net_penalty",
        "lane_opportunity_enemy_limit",
    ] },
    Category { name: "정글", ids: &[
        "jg_gank_hp_start",
        "jg_gank_hp_floor",
        "jungle_safe_radius",
        "jungle_stop_hp",
        "camp_deadline_arrival_seconds",
    ] },
    Category { name: "갱·로밍", ids: &[
        "gank_abort_hp",
        "gank_abort_tower",
        "gank_call_accept_hp",
        "liner_roam_self_hp",
        "liner_roam_lane_healthy_hp",
        "liner_roam_call_chance",
        "shared_battle_role_markers",
        "gank_reevaluate_seconds",
        "enemy_base_allies_radius",
        "ambush_wait_accept_radius",
    ] },
    Category { name: "푸시·타워", ids: &[
        "push_sustain_min_healthy",
        "push_sustain_margin",
        "tower_chase_own_hp",
        "tower_tank_hits",
    ] },
    Category { name: "기지 방어", ids: &[
        "base_defense_min_enemies",
        "base_defense_hold_retreat_hp",
        "base_defense_hold_nexus_hp",
        "base_defense_hold_dmg_ratio",
    ] },
    Category { name: "오브젝트", ids: &[
        "boss_reposition_improvement",
    ] },
    Category { name: "위치·기술", ids: &[
        "support_cap_multiplier",
        "position_isolation_risk_percent",
        "front_ally_risk_scale",
        "movement_skill_period_margin",
        "battle_reposition_distance_scale",
        "candidate_move_budget_margin",
        "skill_arrival_prediction_margin",
    ] },
];
pub fn tactic_at(tab: usize, row: usize) -> Option<usize> {
    let id = CATEGORIES.get(tab)?.ids.get(row)?;
    tactics::index_of(id)
}

/// 정상 게임 경로에서 해당 선수의 설정이 소비되는 포지션(0..4).
pub fn player_visible(id: &str, col: usize) -> bool {
    if col >= 5 { return false; }
    // BEGIN EXTRA VISIBILITY
    match id {
        "gank_call_rng" => return 31 & (1 << col) != 0,
        "pickoff_hp_bands" => return 31 & (1 << col) != 0,
        "structure_retreat_keep_hp" => return 31 & (1 << col) != 0,
        "pre_gank_engage_profile" => return 31 & (1 << col) != 0,
        "gank_travel_deadline" => return 2 & (1 << col) != 0,
        "battle_call_response_seconds" => return 31 & (1 << col) != 0,
        "objective_power_allowance" => return 31 & (1 << col) != 0,
        "same_lane_gank_retry_seconds" => return 31 & (1 << col) != 0,
        "chase_speed_ratio" => return 31 & (1 << col) != 0,
        "tower_tank_hits" => return 31 & (1 << col) != 0,
        "kill_bonus_cap" => return 31 & (1 << col) != 0,
        "ally_spacing_radius" => return 31 & (1 << col) != 0,
        "near_ally_link_margin" => return 31 & (1 << col) != 0,
        "line_defense_approval_profile" => return 31 & (1 << col) != 0,
        "base_battle_call_range" => return 31 & (1 << col) != 0,
        "shared_battle_role_markers" => return 31 & (1 << col) != 0,
        "basic_trade_hp_divisor" => return 31 & (1 << col) != 0,
        "support_cap_multiplier" => return 31 & (1 << col) != 0,
        "buff_arrival_value_seconds" => return 31 & (1 << col) != 0,
        "push_start_base" => return 31 & (1 << col) != 0,
        "other_lane_collapse_profile" => return 31 & (1 << col) != 0,
        "gank_reevaluate_seconds" => return 2 & (1 << col) != 0,
        "gank_candidate_margin" => return 2 & (1 << col) != 0,
        "ambush_engage_radius" => return 31 & (1 << col) != 0,
        "objective_lane_pressure_profile" => return 31 & (1 << col) != 0,
        "roam_retry_extra_seconds" => return 29 & (1 << col) != 0,
        "defense_leave_hp" => return 31 & (1 << col) != 0,
        "structure_skill_bonus" => return 31 & (1 << col) != 0,
        "epic_final_allies_min" => return 31 & (1 << col) != 0,
        "boss_respawn_wait_seconds" => return 31 & (1 << col) != 0,
        "boss_reposition_improvement" => return 31 & (1 << col) != 0,
        "boss_poke_damage_factor" => return 31 & (1 << col) != 0,
        "active_recall_slack_profile" => return 31 & (1 << col) != 0,
        "position_isolation_risk_percent" => return 31 & (1 << col) != 0,
        "front_ally_risk_scale" => return 31 & (1 << col) != 0,
        "battle_skill_safety_margin" => return 31 & (1 << col) != 0,
        "wave_position_risk_divisor" => return 31 & (1 << col) != 0,
        "movement_skill_period_margin" => return 31 & (1 << col) != 0,
        "early_gank_target_min_hp" => return 31 & (1 << col) != 0,
        "camp_time_margin_profile" => return 31 & (1 << col) != 0,
        "battle_risk_retreat_threshold" => return 31 & (1 << col) != 0,
        "unseen_self_risk_divisor" => return 31 & (1 << col) != 0,
        "battle_reposition_distance_scale" => return 31 & (1 << col) != 0,
        "candidate_move_budget_margin" => return 31 & (1 << col) != 0,
        "forward_damage_credit_percent" => return 31 & (1 << col) != 0,
        "battle_move_candidate_penalty" => return 31 & (1 << col) != 0,
        "lane_attack_move_allowance" => return 31 & (1 << col) != 0,
        "lane_defense_support_margin" => return 31 & (1 << col) != 0,
        "lane_exchange_advantage_ticks" => return 31 & (1 << col) != 0,
        "lane_superiority_profile" => return 31 & (1 << col) != 0,
        "lane_call_travel_seconds" => return 31 & (1 << col) != 0,
        "mia_notice_caution_profile" => return 31 & (1 << col) != 0,
        "relay_undying_limit_centiseconds" => return 31 & (1 << col) != 0,
        "gank_lane_tower_wave_profile" => return 31 & (1 << col) != 0,
        "lane_wait_front_minion_distance" => return 31 & (1 << col) != 0,
        "support_roam_adc_min_hp" => return 16 & (1 << col) != 0,
        "purchase_recall_walk_ticks" => return 31 & (1 << col) != 0,
        "purchase_recall_threat_radius" => return 31 & (1 << col) != 0,
        "purchase_recall_threat_memory" => return 31 & (1 << col) != 0,
        "same_lane_partner_radius" => return 31 & (1 << col) != 0,
        "support_zero_net_penalty" => return 31 & (1 << col) != 0,
        "camp_deadline_arrival_seconds" => return 2 & (1 << col) != 0,
        "enemy_base_allies_radius" => return 31 & (1 << col) != 0,
        "ambush_wait_accept_radius" => return 31 & (1 << col) != 0,
        "tracked_target_chase_bonus" => return 31 & (1 << col) != 0,
        "champion_ultimate_waste_penalty" => return 31 & (1 << col) != 0,
        "lane_opportunity_enemy_limit" => return 31 & (1 << col) != 0,
        "skill_arrival_prediction_margin" => return 31 & (1 << col) != 0,
        _ => {}
    }
    // END EXTRA VISIBILITY
    match id {
        "jg_gank_hp_start" | "jg_gank_hp_floor" | "jungle_safe_radius" | "jungle_stop_hp" => col == 1,
        "liner_roam_self_hp" | "liner_roam_lane_healthy_hp" | "liner_roam_call_chance" => col != 1,
        "support_minion_block" => col == 4,
        "lane_assign_hp" => matches!(col, 0 | 2 | 3),
        "bot_duo_partner" => matches!(col, 3 | 4),
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tabs_cover_visible_tactics_once_and_preserve_saved_catalog() {
        let mut seen = [false; tactics::N];
        for (tab, category) in CATEGORIES.iter().enumerate() {
            assert!(!category.ids.is_empty() && category.ids.len() <= ROWS);
            for row in 0..category.ids.len() {
                let t = tactic_at(tab, row).expect("알 수 없는 전술 ID");
                assert!(!seen[t], "중복 전술: {}", category.ids[row]);
                seen[t] = true;
            }
            assert_eq!(tactic_at(tab, category.ids.len()), None);
        }
        assert_eq!(seen.iter().filter(|x| **x).count(), 61);
        // 숨김은 저장 카탈로그 삭제가 아니다. 기존32종은 모두 남는다.
        assert!(seen[..32].iter().all(|x| *x));
        for id in ["kill_bonus_cap", "shared_battle_role_markers", "boss_reposition_improvement", "movement_skill_period_margin"] {
            assert!(seen[tactics::index_of(id).unwrap()]);
        }
        for id in ["gank_call_rng", "support_roam_adc_min_hp", "defense_leave_hp"] {
            assert!(!seen[tactics::index_of(id).unwrap()]);
        }
        assert_eq!(tactics::N, 100);
        assert_eq!(tactic_at(CATEGORIES.len(), 0), None);
    }
}
