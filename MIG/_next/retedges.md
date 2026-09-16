# 실측 호출 간선(리턴 주소 히스토그램 · 판 7 · probe20_r17_run7_retaddr.txt)

> RET 행 695 → 지도 안 간선 414(직접 호출과 겹침 392 · **새 간접 간선 22**) · 지도 밖 호출자 77 · 오버플로 슬롯 2 · 미해소 0

## 새 간접 간선(정적 지도에 없던 것 · 횟수순)

| 호출자 | → 피호출 | 횟수 |
|---|---|---|
| `dcee40` handle_none_or_gank_objective | `d3cfa0` handle_line_defense | 43,577,792 |
| `e05450` resolve_fight_full | `e083c0` fight_model::resolve_fight_uncached | 7,701,476 |
| `dfb840` update | `e0b730` v25_scoped_battle_objective | 6,005,299 |
| `e8d6f0` AgentVerHamster::update_state | `cce0c0` trace::SmallActionTrace::is_end | 3,542,235 |
| `e83390` LineDefenseSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 2,628,917 |
| `df36e0` update_v32 | `dfb220` BattlePlan::with_runaway | 1,608,267 |
| `e4c5c0` update | `e595b0` handle_chat | 1,606,702 |
| `e4c5c0` update | `e4b8c0` LegacyPlanHandler::take_misunderstood_received_chat | 1,606,702 |
| `e4c5c0` update | `e59b20` handle_chat_inner | 1,606,702 |
| `df36e0` update_v32 | `e06df0` resolve_fight_stake | 791,223 |
| `e59b20` handle_chat_inner | `e0bf70` is_unreasonable_tower_dive_enemy | 197,210 |
| `eaeda0` EpicHuntSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 112,945 |
| `d5bbf0` calculate_interaction_action_score | `e01c40` defensive_crisis | 101,413 |
| `cd74c0` with_runaway | `e0bf70` is_unreasonable_tower_dive_enemy | 95,232 |
| `cb1ce0` SerpenHuntSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 69,794 |
| `e7caf0` SerpenPokeSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 66,699 |
| `e07430` tower_dive_is_viable | `e06df0` resolve_fight_stake | 46,081 |
| `cc6170` EpicPokeSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 45,461 |
| `e900b0` AgentVerHamster::get_input | `e4b5d0` v3_fall_back_to_passive | 10,522 |
| `e59b20` handle_chat_inner | `d3cfa0` handle_line_defense | 3,581 |
| `e4c5c0` update | `e59190` v50_fold_dive_episode | 3,484 |
| `cbbdb0` BattleSubPlan::action_candidates | `e06df0` resolve_fight_stake | 1,744 |

## 지도 밖 호출자(game_core 등 · 상위 40)

| 호출자 rva | → 피호출 | 횟수 |
|---|---|---|
| `0x1879080` | `e900b0` AgentVerHamster::get_input | 166,770,196 |
| `0xe25450` | `d97300` can_tower_focused | 104,282,059 |
| `0xc87850` | `d405d0` base_attacking_minion_uncached | 44,790,758 |
| `0xc87850` | `d3e660` defense_nexus::nexus_last_stand_uncached | 44,790,583 |
| `0xc87850` | `d3fe50` defense_nexus::nexus_final_stand_uncached | 44,790,569 |
| `0xec9640` | `ec9400` is_wave_priority_start_line | 37,254,207 |
| `0xeb82d0` | `eba9b0` fight_check::check_kill_die_tick_uncached | 35,942,861 |
| `0xe9bf10` | `e8e160` AgentVerHamster::upgrade_item | 25,600,275 |
| `0xe9c610` | `e8fc80` AgentVerHamster::buy_item | 25,561,146 |
| `0xe49a50` | `de0770` update | 21,430,517 |
| `0xe49a50` | `dccc60` GoalData::update | 21,430,517 |
| `0xe7b9f0` | `e7b0b0` build_game_finish_check_state | 20,754,492 |
| `0xca6700` | `e388c0` serpen_check::SerpenCheckSubPlan::score | 20,426,251 |
| `0xcaf2e0` | `d28800` passive_line::PassiveLinePlan::update | 10,655,293 |
| `0xc986c0` | `d408e0` is_cleared | 8,633,840 |
| `0xe360e0` | `e8aeb0` line_defense::LineDefenseSubPlan::calculate_score_parameter_value | 6,028,005 |
| `0xe35bd0` | `e83390` LineDefenseSubPlan::action_candidates | 6,026,065 |
| `0xcaf2e0` | `dfb840` update | 5,427,280 |
| `0xc93f60` | `d99f60` tower_discipline::v30_line_champion_action_tower_aggro_risk | 5,124,606 |
| `0xe49a50` | `e46bc0` passive_plan | 4,294,030 |
| `0xc94140` | `e8b200` score | 3,256,822 |
| `0xe360e0` | `cc20a0` battle::BattleSubPlan::calculate_score_parameter_value | 3,165,111 |
| `0xe35bd0` | `cbbdb0` BattleSubPlan::action_candidates | 3,159,586 |
| `0xd6abe0` | `d67060` battle_common::v21_defensive_cc_score | 3,022,040 |
| `0xea0c00` | `e0daa0` max_range_nearly_can_use | 2,761,869 |
| `0xc94500` | `e8a100` line_defense::LineDefenseSubPlan::unsafe_v19_non_champion_walkup | 2,299,998 |
| `0xcafe10` | `d2f180` PassiveJunglePlan::next_plan | 2,073,214 |
| `0xd84c60` | `d35d10` abstract_input::attack | 2,012,976 |
| `0xd2e3f0` | `d408e0` is_cleared | 1,835,683 |
| `0xeb6100` | `ebcbd0` should_add_self_etc_buff_action | 1,372,696 |
| `0xdd7250` | `ecacc0` v23_recent_visible_enemies_near_point | 1,162,398 |
| `0xe35bd0` | `cc4260` JungleSubPlan::action_candidates | 1,162,221 |
| `0xdea420` | `dea800` v3_epic_formation_role | 1,102,833 |
| `0xe35bd0` | `cc5ca0` recall::RecallSubPlan::action_candidates | 1,006,596 |
| `0xe26080` | `d97300` can_tower_focused | 946,516 |
| `0xec9d60` | `ec9840` v23_healthy_allies_near_point | 761,558 |
| `0xec9d60` | `ecacc0` v23_recent_visible_enemies_near_point | 761,558 |
| `0xe35bd0` | `e928f0` DefenseNexusSubPlan::action_candidates | 575,122 |
| `0xdfb5d0` | `dff080` base_sub_goal | 554,601 |
| `0xe35bd0` | `eaeda0` EpicHuntSubPlan::action_candidates | 472,367 |