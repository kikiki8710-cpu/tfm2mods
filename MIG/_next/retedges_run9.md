# 실측 호출 간선(리턴 주소 히스토그램 · 판 9 · probe20_run9_replayB_IonWeavers_MatrixFalcons_set1.txt)

> RET 행 1283 → 지도 안 간선 776(직접 호출과 겹침 749 · **새 간접 간선 27**) · 지도 밖 호출자 176 · 오버플로 슬롯 6 · 미해소 0

## 새 간접 간선(정적 지도에 없던 것 · 횟수순)

| 호출자 | → 피호출 | 횟수 |
|---|---|---|
| `dcee40` handle_none_or_gank_objective | `d3cfa0` handle_line_defense | 42,709,020 |
| `e05450` resolve_fight_full | `e083c0` fight_model::resolve_fight_uncached | 7,666,672 |
| `dfb840` update | `e0b730` v25_scoped_battle_objective | 5,972,881 |
| `e83390` LineDefenseSubPlan::action_candidates | `e202b0` FUN_e202b0 | 4,746,457 |
| `e8d6f0` AgentVerHamster::update_state | `cce0c0` trace::SmallActionTrace::is_end | 3,514,348 |
| `e83390` LineDefenseSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 2,596,823 |
| `df36e0` update_v32 | `dfb220` BattlePlan::with_runaway | 1,599,699 |
| `e4c5c0` update | `e595b0` handle_chat | 1,576,579 |
| `e4c5c0` update | `e4b8c0` LegacyPlanHandler::take_misunderstood_received_chat | 1,576,579 |
| `e4c5c0` update | `e59b20` handle_chat_inner | 1,576,579 |
| `e06df0` resolve_fight_stake | `e05450` resolve_fight_full | 837,794 |
| `df36e0` update_v32 | `e06df0` resolve_fight_stake | 763,640 |
| `cc20a0` battle::BattleSubPlan::calculate_score_parameter_value | `e20a00` v3_tower_burst_feasible | 521,859 |
| `e05e70` resolve_join_stake | `e05450` resolve_fight_full | 323,355 |
| `e59b20` handle_chat_inner | `e0bf70` is_unreasonable_tower_dive_enemy | 197,060 |
| `eaeda0` EpicHuntSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 113,305 |
| `d5bbf0` calculate_interaction_action_score | `e01c40` defensive_crisis | 109,291 |
| `cd74c0` with_runaway | `e0bf70` is_unreasonable_tower_dive_enemy | 97,710 |
| `cb1ce0` SerpenHuntSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 68,828 |
| `e7caf0` SerpenPokeSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 62,740 |
| `e07430` tower_dive_is_viable | `e06df0` resolve_fight_stake | 49,651 |
| `d666c0` v3_serpen_contest_clear_win | `e05450` resolve_fight_full | 44,094 |
| `cc6170` EpicPokeSubPlan::action_candidates | `e0daa0` max_range_nearly_can_use | 43,610 |
| `e900b0` AgentVerHamster::get_input | `e4b5d0` v3_fall_back_to_passive | 10,165 |
| `e4c5c0` update | `e59190` v50_fold_dive_episode | 3,516 |
| `e59b20` handle_chat_inner | `d3cfa0` handle_line_defense | 3,512 |
| `cbbdb0` BattleSubPlan::action_candidates | `e06df0` resolve_fight_stake | 1,715 |

## 지도 밖 호출자(game_core 등 · 상위 40)

| 호출자 rva | → 피호출 | 횟수 |
|---|---|---|
| `0xd70620` | `e3b840` v3_pve_monster_aggro_damage | 519,051,769 |
| `0xd95d00` | `c7f0f0` cached_damage_against | 516,417,420 |
| `0xd70620` | `e39bf0` engage_requires_dive | 412,249,114 |
| `0x1879080` | `e900b0` AgentVerHamster::get_input | 163,615,498 |
| `0xe0cf10` | `c809d0` LocalKey::with (max_range_cached TLS 메모 본체) | 147,761,175 |
| `0xe25450` | `d97300` can_tower_focused | 102,321,207 |
| `0xd70620` | `e1e350` v3_survival_incoming | 75,527,170 |
| `0xca89a0` | `deaf70` SerpenStanceData::update_plan::{closure#4} | 75,087,110 |
| `0xeb5dd0` | `12857f0` estimate_damage_to | 70,752,171 |
| `0xd96190` | `12857f0` estimate_damage_to | 65,208,855 |
| `0xeb8b00` | `c872f0` available_cc_in_window | 62,793,595 |
| `0xca8f60` | `deb110` EpicStanceData::update_plan::{closure#4} | 62,333,705 |
| `0xeb82d0` | `c7e150` check_kill_die_tick | 51,725,407 |
| `0xe0d720` | `c80f80` support_min_action_range | 50,431,412 |
| `0xc87850` | `d405d0` base_attacking_minion_uncached | 43,324,911 |
| `0xc87850` | `d3e660` defense_nexus::nexus_last_stand_uncached | 43,324,911 |
| `0xc87850` | `d3fe50` defense_nexus::nexus_final_stand_uncached | 43,324,911 |
| `0xe25450` | `d84db0` position_eval_at | 43,036,380 |
| `0xeb82d0` | `eba9b0` fight_check::check_kill_die_tick_uncached | 35,799,022 |
| `0xec9640` | `ec9400` is_wave_priority_start_line | 35,252,311 |
| `0xc9ef30` | `e10da0` dive_chase_catchable | 34,821,884 |
| `0xdd9f30` | `c99fe0` fmt | 27,398,228 |
| `0xe9bf10` | `e8e160` AgentVerHamster::upgrade_item | 24,479,014 |
| `0xe9c610` | `e8fc80` AgentVerHamster::buy_item | 24,441,125 |
| `0xd70530` | `e3b690` count_in_range_fold | 22,229,911 |
| `0xe27c70` | `12857f0` estimate_damage_to | 22,142,755 |
| `0xe49a50` | `de0770` update | 20,424,842 |
| `0xe49a50` | `dccc60` GoalData::update | 20,424,842 |
| `0xca1560` | `e11290` FUN_e11290 | 20,379,048 |
| `0xe7b9f0` | `e7b0b0` build_game_finish_check_state | 20,347,211 |
| `0xca6700` | `e388c0` serpen_check::SerpenCheckSubPlan::score | 20,070,211 |
| `0xd958b0` | `c7f0f0` cached_damage_against | 19,044,453 |
| `0xc8d140` | `ea07a0` positioning_score_at | 18,404,887 |
| `0xd70530` | `e39a40` engage_requires_dive | 18,177,515 |
| `0xca19b0` | `e11cb0` FUN_e11cb0 | 18,095,016 |
| `0xd758e0` | `12857f0` estimate_damage_to | 15,229,858 |
| `0xd96d00` | `c7ff90` v22_enemy_tower_cover_breaks_immediately | 10,978,193 |
| `0xcaf2e0` | `d28800` passive_line::PassiveLinePlan::update | 10,380,625 |
| `0xeb6100` | `ca5170` FUN_ca5170 | 9,477,359 |
| `0xc88300` | `e2fee0` FUN_e2fee0 | 9,398,131 |