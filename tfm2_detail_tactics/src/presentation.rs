//! UI 분류와 표시 포지션. 저장·AI에서 사용하는 TACTICS 순서는 바꾸지 않는다.
use crate::tactics;

pub const ROWS: usize = 8;
pub struct Category {
    pub name: &'static str,
    pub ids: &'static [&'static str],
}
pub const CATEGORIES: &[Category] = &[
    Category { name: "전투·생존", ids: &[
        "combat_retreat_hp", "line_chase_retreat_hp", "line_chase_target_guard_hp",
        "crowd_retreat_weight", "combat_avoid_hp", "combat_avoid_count_margin",
        "self_risk_ignore_hp", "lowhp_ult_bonus_low",
    ] },
    Category { name: "라인·이동", ids: &[
        "lane_retreat_final", "minion_damage_ratio", "lane_assign_hp",
        "lane_opportunity_own_min", "support_minion_block", "bot_duo_partner",
        "hazard_avoid_padding",
    ] },
    Category { name: "정글", ids: &[
        "jg_gank_hp_start", "jg_gank_hp_floor", "jungle_safe_radius", "jungle_stop_hp",
    ] },
    Category { name: "갱·로밍", ids: &[
        "gank_abort_hp", "gank_abort_tower", "gank_call_accept_hp",
        "liner_roam_self_hp", "liner_roam_lane_healthy_hp", "liner_roam_call_chance",
    ] },
    Category { name: "푸시·타워", ids: &[
        "push_sustain_min_healthy", "push_sustain_margin", "tower_chase_own_hp",
    ] },
    Category { name: "기지 방어", ids: &[
        "base_defense_min_enemies", "base_defense_hold_retreat_hp",
        "base_defense_hold_nexus_hp", "base_defense_hold_dmg_ratio",
    ] },
];

pub fn tactic_at(tab: usize, row: usize) -> Option<usize> {
    let id = CATEGORIES.get(tab)?.ids.get(row)?;
    tactics::index_of(id)
}

/// 정상 게임 경로에서 해당 선수의 설정이 소비되는 포지션(0..4).
pub fn player_visible(id: &str, col: usize) -> bool {
    if col >= 5 { return false; }
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
    fn tabs_cover_every_saved_tactic_exactly_once() {
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
        assert!(seen.iter().all(|x| *x));
        assert_eq!(tactic_at(CATEGORIES.len(), 0), None);
    }
}
