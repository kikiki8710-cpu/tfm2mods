# 지도 밖 호출자 편입(fnmap_ext · retedges.json)

| rva | 이름 | 단 | 근거 | 실측 호출수 | → 콜리(지도) | 정적 호출자(지도 안) |
|---|---|---|---|---|---|---|
| `d70620` | tower_discipline 공용 래퍼(aggro_damage·engage_requires_dive·survival_incoming 호출 · 9.9억회 · 지문 없음) | 1 | 추정/정적 | 1,997,255,248 | v3_pve_monster_aggro_damage · engage_requires_dive · v3_survival_incoming | can_tower_focused |
| `d95d00` | game_ai simulation.rs:1848 캐시 래퍼(cached_damage_against 호출 5.2억회) | 1 | 추정/정적 | 1,032,281,459 | cached_damage_against | simulation.rs:1848/game.rs:210 래퍼(estimate_damage_to 호출) · FUN_e0dfc0 · FUN_d19eb0 · FUN_d1a630 |
| `1879080` | game_core mode.rs 시뮬레이션 틱 루프(AI 루트 · Agent::get_input 호출) | 1 | 확정 | 327,382,720 | AgentVerHamster::get_input · game_core simulation.rs(1603/1741/1542 · 57 Location · 시뮬 본체) |  |
| `e25450` | can_tower_focused 캐시 래퍼(1.04억회) | 1 | 추정/정적 | 292,520,750 | can_tower_focused · position_eval_at | FUN_e248f0 |
| `e0cf10` | max_range_cached TLS 래퍼(1.44억회) | 1 | 추정/정적 | 291,998,412 | LocalKey::with (max_range_cached TLS 메모 본체) | FUN_c9d5b0 · FUN_ca0a10 · FUN_ca4790 · FUN_d66a10 |
| `c87850` | last_stand_flags LocalKey::with(LAST_STAND_MEMO 작성자) | 1 | 확정 | 261,096,987 | base_attacking_minion_uncached · defense_nexus::nexus_last_stand_uncached · defense_nexus::nexus_final_stand_uncached | battle::base_battle_action(battle.rs:1306 · 본체 4/4) · FUN_d3c610 · FUN_d3c680 · BattleSubPlan::action_candidates |
| `eb82d0` | check_kill_die_tick LocalKey::with(DieTickCache 래퍼) | 1 | 확정 | 173,533,035 | check_kill_die_tick · fight_check::check_kill_die_tick_uncached | FUN_e04c60 · FUN_e34d50 · FUN_e35160 · FUN_ebe380 |
| `ca89a0` | SerpenStanceData::update_plan 클로저 호출부 | 1 | 추정/정적 | 149,991,310 | SerpenStanceData::update_plan::{closure#4} | SerpenStanceData::update_plan |
| `d96190` | simulation.rs:1848/game.rs:210 래퍼(estimate_damage_to 호출) | 1 | 추정/정적 | 131,011,370 | estimate_damage_to · game_ai simulation.rs:1848 캐시 래퍼(cached_damage_against 호출 5.2억회) | FUN_e0dfc0 · FUN_d96b10 |
| `ca8f60` | EpicStanceData::update_plan 클로저 호출부 | 1 | 추정/정적 | 125,177,970 | EpicStanceData::update_plan::{closure#4} | EpicStanceData::update_plan |
| `eb8b00` | available_cc_in_window 래퍼 | 1 | 추정/정적 | 124,271,141 | available_cc_in_window | position_eval_at_uncached |
| `e0d720` | support_min_action_range 래퍼 | 1 | 추정/정적 | 99,920,922 | support_min_action_range | with_runaway · with_runaway · with_runaway · FUN_d6bd40 |
| `e49a50` | LegacyPlanHandler::update 아웃라인 조각(GoalData::update·passive_plan·handle_chat 호출) | 1 | 추정/정적 | 91,740,819 | update · GoalData::update · passive_plan · handle_chat · LegacyPlanHandler::take_misunderstood_received_chat … | FUN_e9bf40 |
| `d70530` | position_eval 공용 래퍼(count_in_range_fold·engage_requires_dive 호출) | 1 | 추정/정적 | 87,382,681 | count_in_range_fold · engage_requires_dive · v3_survival_incoming | cached_units_in_range_of |
| `eb5dd0` | (미명명) | 1 | 미확정 | 70,752,171 | estimate_damage_to | FUN_e34a30 · FUN_ebe220 · decide_deathmatch · fight_model::resolve_fight_uncached |
| `ec9640` | find_wave_priority_clear_line | 1 | 확정 | 70,504,937 | is_wave_priority_start_line · FUN_e1db70 | update |
| `c9ef30` | (미명명) | 1 | 미확정 | 69,037,315 | dive_chase_catchable | build |
| `dd9f30` | (미명명) | 1 | 미확정 | 54,703,182 | fmt | sub_plan · update · update · SinglePlanLine::sub_plan |
| `e9bf10` | upgrade_item 래퍼 | 1 | 추정/정적 | 49,350,013 | AgentVerHamster::upgrade_item |  |
| `e9c610` | buy_item 래퍼 | 1 | 추정/정적 | 49,273,990 | AgentVerHamster::buy_item |  |
| `e7b9f0` | end_check(lib.rs:1211) | 1 | 확정 | 40,515,278 | build_game_finish_check_state | FUN_dcea30 · v24_objective_setup_should_release_to_passive |
| `ca1560` | (미명명) | 1 | 미확정 | 40,364,931 | FUN_e11290 | update_v32 |
| `ca6700` | SubPlan::score 디스패처(serpen_check·steal) | 1 | 추정/정적 | 40,155,041 | serpen_check::SerpenCheckSubPlan::score · steal::StealSubPlan::score | auction::LegacyPlanHandler::get_small_action |
| `d958b0` | simulation.rs:1848 래퍼 | 1 | 추정/정적 | 37,575,762 | cached_damage_against | fight_check::check_kill_die_tick_uncached |
| `c8d140` | (미명명) | 1 | 미확정 | 36,958,322 | positioning_score_at · action_candidates_cl | LineDefenseSubPlan::action_candidates |
| `ca19b0` | (미명명) | 1 | 미확정 | 35,772,123 | FUN_e11cb0 | update |
| `caf2e0` | BigPlan::update JT 디스패처(passive_line·battle·line_ganker update) | 1 | 추정/정적 | 31,779,042 | passive_line::PassiveLinePlan::update · update · update · FUN_d2e3f0 | update |
| `d758e0` | (미명명) | 1 | 미확정 | 30,495,471 | estimate_damage_to | v3_deadly_edge_cells |
| `e35bd0` | SubPlan::action_candidates JT 디스패처(서브플랜 15종 전부를 여기서 호출) | 1 | 추정/정적 | 26,384,930 | LineDefenseSubPlan::action_candidates · BattleSubPlan::action_candidates · JungleSubPlan::action_candidates · recall::RecallSubPlan::action_candidates · DefenseNexusSubPlan::action_candidates … | auction::LegacyPlanHandler::get_small_action |
| `d96d00` | tower_discipline.rs:533 LocalKey::with 캐시 래퍼 | 1 | 추정/정적 | 25,902,231 | v22_enemy_tower_cover_breaks_immediately · v47_siege_stance::{closure} call_mut · resolve_fight_full | position_eval_at_uncached · tower_discipline::v47_tower_focus_position_dangerous · v22_lane_tower_pressure_attack_allowed · LineDefenseSubPlan::action_candidates |
| `e27c70` | (미명명) | 1 | 미확정 | 22,142,755 | estimate_damage_to | line_projected_punish_damage_at |
| `eb6100` | fight_check::battle_action(fight_check.rs:622 · 본체 2/2) | 1 | 확정 | 21,700,722 | FUN_ca5170 · should_add_self_etc_buff_action | LineSafeSubPlan::action_candidates(line_safe.rs:26 · 본체 2/2) · EpicCheckSubPlan::action_candidates · SerpenHuntSubPlan::action_candidates · HideSubPlan::action_candidates |
| `c88300` | action_score.rs:577/594 interaction 클로저·TLS 래퍼 | 1 | 추정/정적 | 18,779,483 | FUN_e2fee0 | interaction_score |
| `e360e0` | SubPlan::calculate_score_parameter_value 디스패처(line_defense·battle) | 1 | 추정/정적 | 18,028,111 | line_defense::LineDefenseSubPlan::calculate_score_parameter_value · battle::BattleSubPlan::calculate_score_parameter_value | auction::LegacyPlanHandler::get_small_action |
| `c986c0` | is_cleared 호출 헬퍼 | 1 | 추정/정적 | 16,913,220 | is_cleared | best_jungle_goal |
| `d717e0` | (미명명) | 1 | 미확정 | 14,713,256 | noncombat_steroid_window · aoe_heal_covers_low_ally | simulation.rs:1905 래퍼 |
| `d70900` | (미명명) | 1 | 미확정 | 13,134,964 | FUN_e3bd30 · engage_requires_dive · v3_survival_incoming | score_parameter::calculate_score_parameter |
| `e0b030` | (미명명) | 1 | 미확정 | 11,452,905 | resolve_fight_full | BattleSubPlan::action_candidates · update_v32 |
| `ca33c0` | (미명명) | 1 | 미확정 | 11,423,918 | is_skip_serpen | simulation.rs:1905 래퍼 |
| `181cd60` | game_core simulation.rs(1603/1741/1542 · 57 Location · 시뮬 본체) | 1 | 확정 | 10,769,132 | estimate_damage_to | game_core mode.rs 시뮬레이션 틱 루프(AI 루트 · Agent::get_input 호출) |
| `cd05f0` | battle::base_battle_action(battle.rs:1306 · 본체 4/4) | 1 | 확정 | 10,687,060 | v3_beyond_enemy_line · last_stand_flags LocalKey::with(LAST_STAND_MEMO 작성자) | BattleSubPlan::action_candidates |
| `e03360` | simulation.rs:1905 래퍼 | 1 | 추정/정적 | 10,508,057 | noncombat_steroid_window_cl · noncombat_steroid_window · aoe_heal_covers_low_ally · FUN_d717e0 | calculate_interaction_action_score |
| `c93f60` | v30 tower_aggro_risk 래퍼 | 1 | 추정/정적 | 10,052,264 | tower_discipline::v30_line_champion_action_tower_aggro_risk | LineSafeSubPlan::action_candidates(line_safe.rs:26 · 본체 2/2) · LineDefenseSubPlan::action_candidates · LineWaitSubPlan::action_candidates |
| `e25030` | (미명명) | 1 | 미확정 | 8,168,055 | estimate_damage_to | FUN_d6fc40 · FUN_d71230 |
| `c94140` | LineDefenseSubPlan::score 래퍼 | 1 | 추정/정적 | 6,374,276 | score | LineDefenseSubPlan::action_candidates |
| `ca4650` | (미명명) | 1 | 미확정 | 6,050,965 | new_dive | update_v32 |
| `d6abe0` | simulation.rs:1905 래퍼(v21_defensive_cc_score 호출) | 1 | 추정/정적 | 5,923,689 | battle_common::v21_defensive_cc_score | battle::BattleSubPlan::score · score |
| `c9f210` | (미명명) | 1 | 미확정 | 5,889,341 | v2_obj_restore_safe::{{closure}}#0 | v2_obj_restore_safe |
| `ea0c00` | line_defense.rs:125 클로저(max_range_nearly_can_use 호출) | 1 | 추정/정적 | 5,413,238 | max_range_nearly_can_use | FUN_ca0d30 |
| `c94500` | v19 non_champion_walkup 래퍼 | 1 | 추정/정적 | 4,480,522 | line_defense::LineDefenseSubPlan::unsafe_v19_non_champion_walkup | LineDefenseSubPlan::action_candidates |
| `cafe10` | BigPlan::next_plan JT 디스패처(passive_jungle·line_ganker) | 1 | 추정/정적 | 4,315,170 | PassiveJunglePlan::next_plan · LineGankerPlan::next_plan · FUN_df1f50 | update |
| `d84c60` | abstract_input::attack 래퍼(position_eval 근방) | 1 | 추정/정적 | 3,973,681 | abstract_input::attack | get_input |
| `d2e3f0` | (미명명) | 1 | 미확정 | 3,949,695 | is_cleared · best_jungle_goal | BigPlan::update JT 디스패처(passive_line·battle·line_ganker update) |
| `d37680` | (미명명) | 1 | 미확정 | 3,238,577 | can1v1win | FUN_d68090 · FUN_e03ed0 · calculate_interaction_action_score · battle_common::v16_gambler_ult_cc_bonus |
| `ec9d60` | (미명명) | 1 | 미확정 | 3,022,656 | v23_healthy_allies_near_point · v23_recent_visible_enemies_near_point | check_epic_giveup |
| `dd7250` | (미명명) | 1 | 미확정 | 2,897,357 | v23_recent_visible_enemies_near_point · v23_objective_setup_pressure_line | is_end · is_end |
| `d663f0` | simulation.rs:1905 래퍼 | 1 | 추정/정적 | 2,608,781 | FUN_c9c590 · FUN_ca33c0 | check_serpen_setup · v3_serpen_contest_clear_win · v3_epicops_defer_serpen |
| `dea420` | (미명명) | 1 | 미확정 | 2,163,174 | v3_epic_formation_role | passive_plan |
| `e26080` | (미명명) | 1 | 미확정 | 1,919,604 | can_tower_focused | FUN_e26230 |
| `ca1840` | (미명명) | 1 | 미확정 | 1,515,035 | v30_wave_danger_chase_guard | update_v32 |
| `ca2ad0` | (미명명) | 1 | 미확정 | 1,403,999 | open_chase_race_hopeless | tower_dive_is_viable |
| `d1a480` | (미명명) | 1 | 미확정 | 1,402,642 | is_enemy_danger_cell | get_input · get_input_cl · FUN_cf5660 |
| `e27f50` | (미명명) | 1 | 미확정 | 1,308,753 | estimate_damage_to | line_action_economy_adjustment |
| `ca16d0` | (미명명) | 1 | 미확정 | 1,307,615 | max_range_cached | update_v32 |
| `ca2960` | (미명명) | 1 | 미확정 | 1,249,084 | resolve_join_stake::{{closure}}#0 | resolve_join_stake |
| `dfb5d0` | (미명명) | 1 | 미확정 | 1,105,009 | base_sub_goal | FUN_df1f50 · PassiveJunglePlan::next_plan · make_gank_battle · LineGankerPlan::next_plan |
| `e89fe0` | (미명명) | 1 | 미확정 | 1,033,801 | position_eval_at | line_defense::LineDefenseSubPlan::unsafe_v19_non_champion_walkup |
| `ca1ca0` | (미명명) | 1 | 미확정 | 938,945 | v57_summon_command_score | defensive_crisis |
| `ca0240` | (미명명) | 1 | 미확정 | 925,661 | FUN_e3b1f0 | v2_response_retreat_stance |
| `cd5ee0` | (미명명) | 1 | 미확정 | 843,744 | v48_on_cast_line | BattleSubPlan::action_candidates |
| `de0390` | (미명명) | 1 | 미확정 | 827,895 | should_keep_object_for_contested_wave_priority | v24_objective_setup_should_release_to_passive |
| `ca13f0` | (미명명) | 1 | 미확정 | 582,194 | FUN_ecc2a0 | EpicHuntSubPlan::action_candidates |
| `db50d0` | (미명명) | 1 | 미확정 | 526,941 | FUN_dc7e60 | FUN_da1c10 · shared_free_dist · FUN_dafd10 |
| `c8d360` | (미명명) | 1 | 미확정 | 490,809 | FUN_ecb870 · action_candidates_cl · action_candidates_cl | EpicHuntSubPlan::action_candidates |
| `c8cf40` | (미명명) | 1 | 미확정 | 483,383 | FUN_e9fdc0 · action_candidates_cl · action_candidates_cl | SerpenPokeSubPlan::action_candidates |
| `ca0eb0` | (미명명) | 1 | 미확정 | 461,399 | with_runaway | BattleSubPlan::action_candidates |
| `c9fd60` | (미명명) | 1 | 미확정 | 439,770 | FUN_d41fe0 | FUN_d41980 |
| `ccacf0` | LineSafeSubPlan::action_candidates(line_safe.rs:26 · 본체 2/2) | 1 | 확정 | 430,330 | attack_summon_action · attack_structure_skill_action · line_minion_action_candidates · v22_lane_tower_pressure_attack_allowed · v30 tower_aggro_risk 래퍼 … | SubPlan::action_candidates JT 디스패처(서브플랜 15종 전부를 여기서 호출) |
| `ca0560` | (미명명) | 1 | 미확정 | 383,190 | FUN_cd7370 | SerpenHuntSubPlan::action_candidates |
| `ca2c30` | (미명명) | 1 | 미확정 | 376,234 | can_enemy_hit_objective | tower_dive_is_viable |
| `c965f0` | (미명명) | 1 | 미확정 | 358,294 | epic_action_score | EpicHuntSubPlan::action_candidates |
| `e339d0` | (미명명) | 1 | 미확정 | 342,947 | can1v1win | buff_value_v54 |
| `c8cd30` | (미명명) | 1 | 미확정 | 333,307 | FUN_cd6940 · action_candidates_cl · action_candidates_cl | SerpenHuntSubPlan::action_candidates |
| `c8d570` | (미명명) | 1 | 미확정 | 274,280 | FUN_cd7660 · action_candidates_cl · action_candidates_cl | EpicPokeSubPlan::action_candidates |
| `e2f4f0` | (미명명) | 1 | 미확정 | 254,388 | can1v1win | buff_value_v54 |
| `c8f880` | (미명명) | 1 | 미확정 | 240,570 | serpen_action_score | SerpenHuntSubPlan::action_candidates |
| `de0340` | (미명명) | 1 | 미확정 | 227,026 | should_keep_object_for_contested_wave_priority | v24_objective_setup_should_release_to_passive |
| `ecb440` | (미명명) | 1 | 미확정 | 177,428 | v23_enemy_object_pressure · v23_healthy_allies_near_point · v23_recent_visible_enemies_near_point | serpen_passive_plan · epic_passive_plan |
| `ca8180` | (미명명) | 1 | 미확정 | 149,320 | FUN_dc8b20 | LineGankerPlan::next_plan |
| `c984e0` | (미명명) | 1 | 미확정 | 94,225 | FUN_d41980 | PassiveJunglePlan::next_plan |
| `d70ae0` | (미명명) | 1 | 미확정 | 89,638 | FUN_e3c090 · engage_requires_dive · v3_survival_incoming | score_parameter::calculate_score_parameter |
| `d709f0` | (미명명) | 1 | 미확정 | 84,821 | FUN_e3bee0 · engage_requires_dive · v3_survival_incoming | score_parameter::calculate_score_parameter |
| `e1db70` | (미명명) | 1 | 미확정 | 81,919 | is_wave_priority_start_line | find_wave_priority_clear_line |
| `e02bc0` | (미명명) | 1 | 미확정 | 80,297 | can1v1win | calculate_interaction_action_score |
| `ca82f0` | (미명명) | 1 | 미확정 | 78,500 | FUN_dc8c40 | LineGankerPlan::next_plan |
| `ca8460` | (미명명) | 1 | 미확정 | 78,500 | FUN_dc8d50 | LineGankerPlan::next_plan |
| `df1f50` | (미명명) | 1 | 미확정 | 72,262 | get_die_tick_player · next_plan · FUN_dfb5d0 | BigPlan::next_plan JT 디스패처(passive_jungle·line_ganker) |
| `dcea30` | (미명명) | 1 | 미확정 | 67,179 | objective_handlers::TeamPlan::handle_nexus_attack · end_check(lib.rs:1211) | handle_none_or_gank_objective · v24_objective_setup_should_release_to_passive |
| `d70bd0` | (미명명) | 1 | 미확정 | 53,875 | FUN_e3c240 · engage_requires_dive · v3_survival_incoming | score_parameter::calculate_score_parameter |
| `d70710` | (미명명) | 1 | 미확정 | 46,777 | v30_line_action_stance · engage_requires_dive · v3_survival_incoming | can_tower_focused_when_attack |
| `d9a9d0` | (미명명) | 1 | 미확정 | 35,663 | bush_distance_sq | steal::StealSubPlan::action_candidates |
| `eba320` | (미명명) | 1 | 미확정 | 33,264 | FUN_c7e5a0 · FUN_c7e920 | v48_cast_beams (LocalKey::with 클로저) |
| `c91520` | (미명명) | 1 | 미확정 | 15,013 | score | SerpenPokeSubPlan::action_candidates |
| `d71500` | (미명명) | 1 | 미확정 | 13,980 | FUN_d7ac00 · FUN_d7a630 · FUN_e21650 | can_tower_focused_when_attack |
| `ca5da0` | (미명명) | 1 | 미확정 | 10,582 | evaluate_gank_opportunity_with_score | PassiveJunglePlan::next_plan |
| `c98290` | (미명명) | 1 | 미확정 | 8,678 | score | EpicPokeSubPlan::action_candidates |
| `c8b710` | (미명명) | 1 | 미확정 | 7,084 | clone | FUN_c5ded0 · FUN_c72c60 · get_input_cl |
| `dfdd50` | (미명명) | 1 | 미확정 | 4,006 | base_sub_goal | try_engage · try_engage_dive |
| `c9d5b0` | (미명명) | 2 | 미확정 | 0 | max_range_cached TLS 래퍼(1.44억회) |  |
| `ca0a10` | (미명명) | 2 | 미확정 | 0 | max_range_cached TLS 래퍼(1.44억회) |  |
| `ca4790` | (미명명) | 2 | 미확정 | 0 | max_range_cached TLS 래퍼(1.44억회) |  |
| `d66a10` | (미명명) | 2 | 미확정 | 0 | max_range_cached TLS 래퍼(1.44억회) |  |
| `d69690` | (미명명) | 2 | 미확정 | 0 | max_range_cached TLS 래퍼(1.44억회) |  |
| `e04c60` | (미명명) | 2 | 미확정 | 0 | max_range_cached TLS 래퍼(1.44억회) · check_kill_die_tick LocalKey::with(DieTickCache 래퍼) |  |
| `e0ad90` | (미명명) | 2 | 미확정 | 0 | max_range_cached TLS 래퍼(1.44억회) |  |
| `e704b0` | (미명명) | 2 | 미확정 | 0 | max_range_cached TLS 래퍼(1.44억회) |  |
| `ebe790` | (미명명) | 2 | 미확정 | 0 | max_range_cached TLS 래퍼(1.44억회) |  |
| `e0dfc0` | (미명명) | 2 | 미확정 | 0 | game_ai simulation.rs:1848 캐시 래퍼(cached_damage_against 호출 5.2억회) · simulation.rs:1848/game.rs:210 래퍼(estimate_damage_to 호출) |  |
| `d6fc40` | (미명명) | 2 | 미확정 | 0 | FUN_e25030 |  |
| `d71230` | (미명명) | 2 | 미확정 | 0 | FUN_e25030 |  |
| `d68090` | (미명명) | 2 | 미확정 | 0 | FUN_d37680 |  |
| `e03ed0` | (미명명) | 2 | 미확정 | 0 | FUN_d37680 |  |
| `e34a30` | (미명명) | 2 | 미확정 | 0 | FUN_eb5dd0 |  |
| `ebe220` | (미명명) | 2 | 미확정 | 0 | FUN_eb5dd0 |  |
| `d96b10` | (미명명) | 2 | 미확정 | 0 | simulation.rs:1848/game.rs:210 래퍼(estimate_damage_to 호출) |  |
| `ca0d30` | (미명명) | 2 | 미확정 | 0 | line_defense.rs:125 클로저(max_range_nearly_can_use 호출) |  |
| `e34d50` | (미명명) | 2 | 미확정 | 0 | check_kill_die_tick LocalKey::with(DieTickCache 래퍼) |  |
| `e35160` | (미명명) | 2 | 미확정 | 0 | check_kill_die_tick LocalKey::with(DieTickCache 래퍼) |  |
| `ebe380` | (미명명) | 2 | 미확정 | 0 | check_kill_die_tick LocalKey::with(DieTickCache 래퍼) |  |
| `e248f0` | (미명명) | 2 | 미확정 | 0 | can_tower_focused 캐시 래퍼(1.04억회) |  |
| `d3c610` | (미명명) | 2 | 미확정 | 0 | last_stand_flags LocalKey::with(LAST_STAND_MEMO 작성자) |  |
| `d3c680` | (미명명) | 2 | 미확정 | 0 | last_stand_flags LocalKey::with(LAST_STAND_MEMO 작성자) |  |
| `e26230` | (미명명) | 2 | 미확정 | 0 | FUN_e26080 |  |
| `e9bf40` | (미명명) | 2 | 미확정 | 0 | LegacyPlanHandler::update 아웃라인 조각(GoalData::update·passive_plan·handle_chat 호출) |  |