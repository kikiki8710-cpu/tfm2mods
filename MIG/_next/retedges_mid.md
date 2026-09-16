# 실측 호출 간선(리턴 주소 히스토그램 · 판7중간 · probe20.txt)

> RET 행 695 → 지도 안 간선 414(직접 호출과 겹침 392 · **새 간접 간선 22**) · 지도 밖 호출자 77 · 오버플로 슬롯 2 · 미해소 0

## 새 간접 간선(정적 지도에 없던 것 · 횟수순)

| 호출자 | → 피호출 | 횟수 |
|---|---|---|
| `dcee40` handle_none_or_gank_objective | `d3cfa0` handle_line_defense | 20,981,396 |
| `e05450` resolve_fight_full | `e083c0` fight_model::resolve_fight_uncached | 3,645,149 |
| `dfb840` update | `e0b730` v25_scoped_battle_objective | 2,830,948 |
| `e8d6f0` AgentVerHamster::update_state | `cce0c0` trace::SmallActionTrace::is_end | 1,671,182 |
| `e83390` LineDefenseSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 1,267,289 |
| `df36e0` update_v32 | `dfb220` BattlePlan::with_runaway | 763,236 |
| `e4c5c0` update | `e595b0` handle_chat | 758,856 |
| `e4c5c0` update | `e4b8c0` LegacyPlanHandler::take_misunderstood_received_chat | 758,856 |
| `e4c5c0` update | `e59b20` handle_chat_inner | 758,856 |
| `df36e0` update_v32 | `e06df0` resolve_fight_stake | 370,047 |
| `e59b20` handle_chat_inner | `e0bf70` is_unreasonable_tower_dive_enemy | 92,658 |
| `eaeda0` EpicHuntSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 51,924 |
| `cd74c0` with_runaway | `e0bf70` is_unreasonable_tower_dive_enemy | 46,307 |
| `d5bbf0` calculate_interaction_action_score | `e01c40` defensive_crisis | 39,729 |
| `cb1ce0` SerpenHuntSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 33,989 |
| `e7caf0` SerpenPokeSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 32,346 |
| `e07430` tower_dive_is_viable | `e06df0` resolve_fight_stake | 21,711 |
| `cc6170` EpicPokeSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 20,918 |
| `e900b0` AgentVerHamster::get_input | `e4b5d0` v3_fall_back_to_passive | 4,765 |
| `e59b20` handle_chat_inner | `d3cfa0` handle_line_defense | 1,652 |
| `e4c5c0` update | `e59190` v50_fold_dive_episode | 1,634 |
| `cbbdb0` BattleSubPlan::action_candidates | `e06df0` resolve_fight_stake | 760 |

## 지도 밖 호출자(game_core 등 · 상위 40)

| 호출자 rva | → 피호출 | 횟수 |
|---|---|---|
| `0x1879080` | `e900b0` AgentVerHamster::get_input | 79,783,974 |
| `0xe25450` | `d97300` can_tower_focused | 50,344,350 |
| `0xc87850` | `d405d0` base_attacking_minion_uncached | 21,239,878 |
| `0xc87850` | `d3e660` defense_nexus::nexus_last_stand_uncached | 21,239,724 |
| `0xc87850` | `d3fe50` defense_nexus::nexus_final_stand_uncached | 21,239,711 |
| `0xec9640` | `ec9400` is_wave_priority_start_line | 18,160,789 |
| `0xeb82d0` | `eba9b0` fight_check::check_kill_die_tick_uncached | 17,057,064 |
| `0xe9bf10` | `e8e160` AgentVerHamster::upgrade_item | 12,069,589 |
| `0xe9c610` | `e8fc80` AgentVerHamster::buy_item | 12,050,241 |
| `0xe49a50` | `de0770` update | 10,058,435 |
| `0xe49a50` | `dccc60` GoalData::update | 10,058,435 |
| `0xca6700` | `e388c0` serpen_check::SerpenCheckSubPlan::score | 9,765,038 |
| `0xe7b9f0` | `e7b0b0` build_game_finish_check_state | 9,704,845 |
| `0xcaf2e0` | `d28800` passive_line::PassiveLinePlan::update | 5,126,486 |
| `0xc986c0` | `d408e0` is_cleared | 4,207,272 |
| `0xe360e0` | `e8aeb0` line_defense::LineDefenseSubPlan::calculate_score_parameter_value | 2,897,273 |
| `0xe35bd0` | `e83390` LineDefenseSubPlan::action_candidates | 2,896,407 |
| `0xcaf2e0` | `dfb840` update | 2,563,785 |
| `0xc93f60` | `d99f60` tower_discipline::v30_line_champion_action_tower_aggro_risk | 2,471,429 |
| `0xe49a50` | `e46bc0` passive_plan | 1,985,671 |
| `0xc94140` | `e8b200` score | 1,583,719 |
| `0xe360e0` | `cc20a0` battle::BattleSubPlan::calculate_score_parameter_value | 1,498,298 |
| `0xe35bd0` | `cbbdb0` BattleSubPlan::action_candidates | 1,495,513 |
| `0xd6abe0` | `d67060` battle_common::v21_defensive_cc_score | 1,434,604 |
| `0xea0c00` | `e0daa0` max_range_nearly_can_use | 1,334,481 |
| `0xc94500` | `e8a100` line_defense::LineDefenseSubPlan::unsafe_v19_non_champion_walkup | 1,109,086 |
| `0xcafe10` | `d2f180` PassiveJunglePlan::next_plan | 1,008,348 |
| `0xd84c60` | `d35d10` abstract_input::attack | 958,897 |
| `0xd2e3f0` | `d408e0` is_cleared | 893,400 |
| `0xeb6100` | `ebcbd0` should_add_self_etc_buff_action | 659,176 |
| `0xe35bd0` | `cc4260` JungleSubPlan::action_candidates | 564,144 |
| `0xdd7250` | `ecacc0` v23_recent_visible_enemies_near_point | 550,904 |
| `0xdea420` | `dea800` v3_epic_formation_role | 508,408 |
| `0xe35bd0` | `cc5ca0` recall::RecallSubPlan::action_candidates | 484,180 |
| `0xe26080` | `d97300` can_tower_focused | 466,341 |
| `0xec9d60` | `ec9840` v23_healthy_allies_near_point | 356,022 |
| `0xec9d60` | `ecacc0` v23_recent_visible_enemies_near_point | 356,022 |
| `0xe35bd0` | `e928f0` DefenseNexusSubPlan::action_candidates | 270,942 |
| `0xdfb5d0` | `dff080` base_sub_goal | 256,143 |
| `0xe35bd0` | `eaeda0` EpicHuntSubPlan::action_candidates | 221,646 |