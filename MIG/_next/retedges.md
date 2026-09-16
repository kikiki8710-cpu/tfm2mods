# 실측 호출 간선 합본(판 8+9 = 판 8 + 판 9)

> 지도 안 간선 784(새 간접 27) · 지도 밖 호출자 177 · 판별 고유 간선 = 판 8 에만 8 · 판 9 에만 5

## 판 8 에만 있던 지도 안 간선(횟수순 · 상위 40)

| 호출자 | → 피호출 | 횟수 | 종류 |
|---|---|---|---|
| `d1d9c0` new_tower_avoid_v3 | `12857f0` estimate_damage_to | 7,731,820 | direct |
| `d377a0` utils::build_minion_wave_snapshot | `12857f0` estimate_damage_to | 7,220,428 | direct |
| `dc0800` merge | `d84db0` position_eval_at | 6,857,721 | direct |
| `db63c0` FUN_db63c0 | `d214e0` FUN_d214e0 | 167,842 | direct |
| `cfac80` FUN_cfac80 | `dc8090` FUN_dc8090 | 145,916 | direct |
| `d1b7e0` FUN_d1b7e0 | `d214e0` FUN_d214e0 | 60,351 | direct |
| `c654d0` FUN_c654d0 | `dc8550` FUN_dc8550 | 41,284 | direct |
| `e80070` FUN_e80070 | `eb77a0` battle_ally_action | 5 | direct |

## 판 9 에만 있던 지도 안 간선(횟수순 · 상위 40)

| 호출자 | → 피호출 | 횟수 | 종류 |
|---|---|---|---|
| `e8a100` line_defense::LineDefenseSubPlan::unsafe_v19_non_champion_walkup | `12857f0` estimate_damage_to | 1,963,610 | direct |
| `d3b2a0` v46_flee_gate_check | `12857f0` estimate_damage_to | 956,602 | direct |
| `c58240` 21SmallActionAroundBush9 | `d214e0` FUN_d214e0 | 135,636 | direct |
| `cf1bb0` get_input | `dc8090` FUN_dc8090 | 9,861 | direct |
| `c66620` FUN_c66620 | `dc8550` FUN_dc8550 | 6,349 | direct |

## 새 간접 간선 합본(정적 지도에 없던 것 · 횟수순)

| 호출자 | → 피호출 | 합계 | 판 8 | 판 9 |
|---|---|---|---|---|
| `dcee40` handle_none_or_gank_objective | `d3cfa0` handle_line_defense | 85,519,066 | 42,810,046 | 42,709,020 |
| `e05450` resolve_fight_full | `e083c0` fight_model::resolve_fight_uncached | 15,165,802 | 7,499,130 | 7,666,672 |
| `dfb840` update | `e0b730` v25_scoped_battle_objective | 11,847,235 | 5,874,354 | 5,972,881 |
| `e83390` LineDefenseSubPlan::action_candidates | `e202b0` FUN_e202b0 | 9,517,317 | 4,770,860 | 4,746,457 |
| `e8d6f0` AgentVerHamster::update_state | `cce0c0` trace::SmallActionTrace::is_end | 6,989,622 | 3,475,274 | 3,514,348 |
| `e83390` LineDefenseSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 5,195,253 | 2,598,430 | 2,596,823 |
| `df36e0` update_v32 | `dfb220` BattlePlan::with_runaway | 3,174,280 | 1,574,581 | 1,599,699 |
| `e4c5c0` update | `e595b0` handle_chat | 3,150,171 | 1,573,592 | 1,576,579 |
| `e4c5c0` update | `e4b8c0` LegacyPlanHandler::take_misunderstood_received_chat | 3,150,171 | 1,573,592 | 1,576,579 |
| `e4c5c0` update | `e59b20` handle_chat_inner | 3,150,171 | 1,573,592 | 1,576,579 |
| `e06df0` resolve_fight_stake | `e05450` resolve_fight_full | 1,657,879 | 820,085 | 837,794 |
| `df36e0` update_v32 | `e06df0` resolve_fight_stake | 1,519,571 | 755,931 | 763,640 |
| `cc20a0` battle::BattleSubPlan::calculate_score_parameter_value | `e20a00` v3_tower_burst_feasible | 1,036,397 | 514,538 | 521,859 |
| `e05e70` resolve_join_stake | `e05450` resolve_fight_full | 635,268 | 311,913 | 323,355 |
| `e59b20` handle_chat_inner | `e0bf70` is_unreasonable_tower_dive_enemy | 389,907 | 192,847 | 197,060 |
| `eaeda0` EpicHuntSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 221,212 | 107,907 | 113,305 |
| `d5bbf0` calculate_interaction_action_score | `e01c40` defensive_crisis | 216,826 | 107,535 | 109,291 |
| `cd74c0` with_runaway | `e0bf70` is_unreasonable_tower_dive_enemy | 192,352 | 94,642 | 97,710 |
| `cb1ce0` SerpenHuntSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 139,215 | 70,387 | 68,828 |
| `e7caf0` SerpenPokeSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 129,782 | 67,042 | 62,740 |
| `e07430` tower_dive_is_viable | `e06df0` resolve_fight_stake | 94,010 | 44,359 | 49,651 |
| `cc6170` EpicPokeSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 87,245 | 43,635 | 43,610 |
| `d666c0` v3_serpen_contest_clear_win | `e05450` resolve_fight_full | 84,319 | 40,225 | 44,094 |
| `e900b0` AgentVerHamster::get_input | `e4b5d0` v3_fall_back_to_passive | 20,133 | 9,968 | 10,165 |
| `e59b20` handle_chat_inner | `d3cfa0` handle_line_defense | 6,970 | 3,458 | 3,512 |
| `e4c5c0` update | `e59190` v50_fold_dive_episode | 6,747 | 3,231 | 3,516 |
| `cbbdb0` BattleSubPlan::action_candidates | `e06df0` resolve_fight_stake | 3,402 | 1,687 | 1,715 |