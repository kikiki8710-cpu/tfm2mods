# 지도 밖 함수 정체 추정(extname · exe 콜리 ↔ IR 콜리)

| rva | 현재 이름 | exe 크기 | exe 콜리(이름) | IR 후보(교집합/필요 · IR 명령수) |
|---|---|---|---|---|
| `d70620` | tower_discipline 공용 래퍼(aggro_damage·engage_requires_dive·survival_incoming 호출 · 9.9억회 · 지문 없음) | 226B | engage_requires_dive · v3_pve_monster_aggro_damage · v3_survival_incoming | — |
| `d95d00` | game_ai simulation.rs:1848 캐시 래퍼(cached_damage_against 호출 5.2억회) | 1154B | cached_damage_against | — |
| `e25450` | can_tower_focused 캐시 래퍼(1.04억회) | 952B | can_tower_focused · position_eval_at | `tower_discipline::can_trace_without_tower` 1/2 (163) · `position_eval::position_score_value_at_position` 1/2 (86) · `lane_economy::line_projected_punish_damage_at` 1/2 (867) |
| `e0cf10` | max_range_cached TLS 래퍼(1.44억회) | 889B | with | — |
| `ca89a0` | SerpenStanceData::update_plan 클로저 호출부 | 240B | (지도 이름 콜리 없음) | — |
| `d96190` | simulation.rs:1848/game.rs:210 래퍼(estimate_damage_to 호출) | 2418B | estimate_damage_to | — |
| `ca8f60` | EpicStanceData::update_plan 클로저 호출부 | 240B | (지도 이름 콜리 없음) | — |
| `eb8b00` | available_cc_in_window 래퍼 | 123B | available_cc_in_window | `old::fight_model::resolve_fight_uncached` 1/1 (3266) |
| `e0d720` | support_min_action_range 래퍼 | 889B | support_min_action_range | `auction::17L::get_small_action` 1/1 (6294) · `1::Bat::action_candidates` 1/1 (8011) |
| `e49a50` | LegacyPlanHandler::update 아웃라인 조각(GoalData::update·passive_plan·handle_chat 호출) | 2734B | handle_chat · passive_plan · take_misunderstood_received_chat · update | `1::LegacyP::update_on_dead` 4/4 ★(501) · `1::LegacyP::update` 4/4 (13249) · `chat::17L::handle_chat_inner` 2/4 (3322) |
| `d70530` | position_eval 공용 래퍼(count_in_range_fold·engage_requires_dive 호출) | 226B | count_in_range_fold · engage_requires_dive · v3_survival_incoming | — |
| `eb5dd0` | — | 742B | estimate_damage_to | — |
| `c9ef30` | — | 267B | dive_chase_catchable | `battle::10Ba::update_v32` 1/1 (8229) |
| `dd9f30` | — | 745B | fmt | `fmt::5Debu::fmt` 1/1 (28) · `fmt::5Debu::fmt` 1/1 (28) · `fmt::5Debu::fmt` 1/1 (28) |
| `e9bf10` | upgrade_item 래퍼 | 35B | upgrade_item | `15Ag::f5S1uJLkH_9game_core10simulation12ai_interface7AiAgent12upgrade_item` 1/1 ★(8) · `::15Ag::upgrade_item` 1/1 (111) · `hunt_and_battle::2::E` 1/1 (200) |
| `e9c610` | buy_item 래퍼 | 25B | buy_item | `15Ag::f5S1uJLkH_9game_core10simulation12ai_interface7AiAgent8buy_item` 1/1 ★(8) · `::15Ag::buy_item` 1/1 (96) · `should_recall_to_shop` 1/1 (581) |
| `ca1560` | — | 275B | (지도 이름 콜리 없음) | — |
| `ca6700` | SubPlan::score 디스패처(serpen_check·steal) | 1007B | score | `18LineDefenseSubPlan1::action_::0EBY` 1/1 ★(222) · `get_small_action::::` 1/1 ★(339) · `serpen_hunt::17SerpenHuntSubPlan17::0EBY` 1/1 ★(174) |
| `d958b0` | simulation.rs:1848 래퍼 | 1102B | cached_damage_against | — |
| `c8d140` | — | 405B | positioning_score_at | `1::LineDefe::unsafe_v19_non_champion_walkup` 1/1 (1025) |
| `ca19b0` | — | 267B | (지도 이름 콜리 없음) | — |
| `caf2e0` | BigPlan::update JT 디스패처(passive_line·battle·line_ganker update) | 441B | update | `types::7::update` 1/1 ★(99) · `1::Line::make_gank_battle` 1/1 ★(142) · `engage::17L::try_engage_dive` 1/1 ★(171) |
| `d758e0` | — | 323B | estimate_damage_to | — |
| `e35bd0` | SubPlan::action_candidates JT 디스패처(서브플랜 15종 전부를 여기서 호출) | 1230B | action_candidates | `7Su::action_candidates` 1/1 ★(283) · `auction::17L::get_small_action` 1/1 (6294) |
| `d96d00` | tower_discipline.rs:533 LocalKey::with 캐시 래퍼 | 1311B | estimate_damage_to · resolve_fight_full · v22_enemy_tower_cover_breaks_immediately | — |
| `e27c70` | — | 476B | estimate_damage_to | — |
| `c88300` | action_score.rs:577/594 interaction 클로저·TLS 래퍼 | 4592B | (지도 이름 콜리 없음) | — |
| `e360e0` | SubPlan::calculate_score_parameter_value 디스패처(line_defense·battle) | 9844B | calculate_score_parameter_value | `7Su::calculate_score_parameter_value` 1/1 ★(1485) · `auction::17L::get_small_action` 1/1 ★(6294) |
| `c986c0` | is_cleared 호출 헬퍼 | 253B | is_cleared | `1::Passive::update` 1/1 ★(96) · `old::passive_jungle::is_side_cleared` 1/1 ★(41) · `best_jungle_goal::5FnMut::f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeEE8call_mutBY_` 1/1 ★(29) |
| `d717e0` | — | 286B | aoe_heal_covers_low_ally · noncombat_steroid_window | `serpen_poke::17SerpenPokeSubPlan17::0` 1/2 (2491) · `epic_poke::15EpicPokeSubPlan17ac::0` 1/2 (2491) · `serpen_hunt::17SerpenHuntSubPlan17::0EB` 1/2 (2731) |
| `d70900` | — | 226B | engage_requires_dive · v3_survival_incoming | `modes::17L::single_try_engage` 1/2 (331) · `engage::17L::try_engage` 1/2 (495) · `auction::17L::get_small_action` 1/2 (6294) |
| `e0b030` | — | 1445B | resolve_fight_full | `old::fight_model::resolve_fight_stake` 1/1 ★(317) · `old::fight_model::resolve_fight_stake_roster` 1/1 ★(486) · `old::fight_model::resolve_join_stake` 1/1 (1230) |
| `ca33c0` | — | 275B | is_skip_serpen | `old::serpen::serpen_passive_plan` 1/1 (985) · `old::serpen::check_serpen_setup` 1/1 (1472) |
| `e03360` | simulation.rs:1905 래퍼 | 2922B | aoe_heal_covers_low_ally · noncombat_steroid_window · noncombat_steroid_window_cl | — |
| `c93f60` | v30 tower_aggro_risk 래퍼 | 478B | v30_line_champion_action_tower_aggro_risk | `18LineDefenseSubPlan1::action_::0EB` 1/1 ★(154) · `line_defense::18LineDefenseSubPlan1::action_` 1/1 ★(154) · `retain::line_safe::15LineSafeSubPlan17ac` 1/1 ★(154) |
| `e25030` | — | 1045B | estimate_damage_to | — |
| `c94140` | LineDefenseSubPlan::score 래퍼 | 960B | score | `18LineDefenseSubPlan1::action_::0EBY` 1/1 ★(222) · `serpen_hunt::17SerpenHuntSubPlan17::0EBY` 1/1 ★(174) · `serpen_poke::17SerpenPokeSubPlan17::0E` 1/1 ★(174) |
| `ca4650` | — | 236B | new_dive | `engage::17L::try_engage_dive` 1/1 ★(171) · `modes::17L::single_try_engage` 1/1 (331) · `engage::17L::try_engage` 1/1 (495) |
| `d6abe0` | simulation.rs:1905 래퍼(v21_defensive_cc_score 호출) | 1656B | v21_defensive_cc_score | `battle_common::v21_runaway_defensive_cc_hold_penalty` 1/1 ★(260) · `battle_common::v21_runaway_defensive_cc_bonus` 1/1 (25) |
| `c9f210` | — | 274B | (지도 이름 콜리 없음) | — |
| `ea0c00` | line_defense.rs:125 클로저(max_range_nearly_can_use 호출) | 348B | max_range_nearly_can_use | `unsafe_v19_non_champion_walkup::5FnMut::f5S1uJLkH_9game_core10simulation6entity6EntityEE8call_mutBZ_` 1/1 ★(106) · `1::SerpenP::action_candidates` 1/1 (3281) · `1::EpicP::action_candidates` 1/1 (4189) |
| `c94500` | v19 non_champion_walkup 래퍼 | 644B | unsafe_v19_non_champion_walkup | `18LineDefenseSubPlan1::action_::0EBY_` 1/1 ★(178) |
| `cafe10` | BigPlan::next_plan JT 디스패처(passive_jungle·line_ganker) | 237B | next_plan | `types::7::next_plan` 1/1 (199) · `1::LegacyP::update` 1/1 (13249) |
| `d84c60` | abstract_input::attack 래퍼(position_eval 근방) | 325B | attack | `cast::17S::get_input` 1/1 ★(65) |
| `d2e3f0` | — | 265B | best_jungle_goal · is_cleared | `1::Passive::update` 2/2 ★(96) · `old::passive_jungle::is_side_cleared` 1/2 (41) · `best_jungle_goal::5FnMut::f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeEE8call_mutBY_` 1/2 (29) |
| `d37680` | — | 129B | can1v1win | `1::H::action_candidates` 1/1 (4680) |
| `ec9d60` | — | 115B | v23_healthy_allies_near_point · v23_recent_visible_enemies_near_point | `team_plan::objective_helpers::v23_should_break_objective_hunt_anchor` 2/2 ★(50) · `team_plan::objective_helpers::v25_objective_splitter_should_join_contest` 2/2 ★(52) · `team_plan::objective_helpers::v23_visible_objective_overload` 2/2 ★(13) |
| `dd7250` | — | 351B | v23_objective_setup_pressure_line · v23_recent_visible_enemies_near_point | `objective_discipline::8Te::v24_objective_setup_should_release_to_passive` 2/2 ★(64) · `objective_discipline::8Te::v24_objective_setup_lane_pressure_ready` 2/2 (787) · `team_plan::objective_helpers::v25_objective_splitter_can_stay` 1/2 (122) |
| `d663f0` | simulation.rs:1905 래퍼 | 405B | (지도 이름 콜리 없음) | — |
| `dea420` | — | 120B | v3_epic_formation_role | `old::epic::v3_epic_formation` 1/1 ★(18) · `9team::TeamPlan::v3_epicops_buff_window` 1/1 (353) |
| `e26080` | — | 418B | can_tower_focused | `tower_discipline::can_trace_without_tower` 1/1 ★(163) · `lane_economy::line_projected_punish_damage_at` 1/1 (867) · `utils::is_safe_pos` 1/1 (1070) |
| `ca1840` | — | 275B | v30_wave_danger_chase_guard | `1::Single::update` 1/1 (6733) · `1::DeathM::update` 1/1 (8029) · `battle::10Ba::update_v32` 1/1 (8229) |
| `ca2ad0` | — | 267B | open_chase_race_hopeless | `1::Line::make_gank_battle` 1/1 ★(142) · `engage::17L::try_engage_dive` 1/1 ★(171) · `engage::17L::try_engage` 1/1 (495) |
| `d1a480` | — | 421B | is_enemy_danger_cell | `get_input::0::` 1/1 ★(117) · `SmallActionAroundRegion::get_input::` 1/1 ★(117) · `get_input::::` 1/1 ★(151) |
| `e27f50` | — | 257B | estimate_damage_to | — |
| `ca16d0` | — | 275B | max_range_cached | `update::5FnMut::f5S1uJLkH_9game_core10simulation6entity6EntityEE8call_mutBZ_` 1/1 ★(80) · `update::5FnMut::f5S1uJLkH_9game_core10simulation6entity6EntityEE8call_mutBZ_` 1/1 ★(80) · `action_candidates::5FnMut::f5S1uJLkH_9game_core10simulation6entity6EntityEE8call_mutBZ_` 1/1 ★(59) |
| `ca2960` | — | 274B | (지도 이름 콜리 없음) | — |
| `dfb5d0` | — | 481B | base_sub_goal | `1::DeathM::new` 1/1 ★(123) · `1::DeathM::new_dive` 1/1 ★(123) · `battle::10Ba::new_region` 1/1 ★(115) |
| `e89fe0` | — | 273B | position_eval_at | `position_eval::position_score_value_at_position` 1/1 ★(86) · `position_eval::position_score_at_position` 1/1 ★(32) · `position_eval::position_score_at_cell` 1/1 ★(28) |
| `ca1ca0` | — | 274B | v57_summon_command_score | `action_score::calculate_interaction_action_score` 1/1 (3812) |
| `ca0240` | — | 274B | (지도 이름 콜리 없음) | — |
| `cd5ee0` | — | 1193B | position_eval_at · v48_on_cast_line | `battle::kite_reposition_point` 1/2 (418) · `action_eval::evaluate_action` 1/2 (981) · `position_eval::position_score_value_at_position` 1/2 (86) |
| `de0390` | — | 51B | should_keep_object_for_contested_wave_priority | `team_plan::8Tea::should_delay_morgard_for_wave_priority` 1/1 ★(26) · `team_plan::8Tea::should_delay_serpen_for_wave_priority` 1/1 ★(29) · `team_plan::8Tea::should_delay_object_setup_for_wave_priority` 1/1 (73) |
| `ca13f0` | — | 274B | (지도 이름 콜리 없음) | — |
| `db50d0` | — | 565B | (지도 이름 콜리 없음) | — |
| `c8d360` | — | 389B | (지도 이름 콜리 없음) | — |
| `c8cf40` | — | 373B | (지도 이름 콜리 없음) | — |
| `ca0eb0` | — | 275B | with_runaway | `1::Single::update` 1/1 (6733) · `1::DeathM::update` 1/1 (8029) · `battle::10Ba::update_v32` 1/1 (8229) |
| `c9fd60` | — | 274B | (지도 이름 콜리 없음) | — |
| `ca0560` | — | 274B | (지도 이름 콜리 없음) | — |
| `ca2c30` | — | 275B | can_enemy_hit_objective | `old::fight_model::should_ignore_object_finish_kill_priority_target` 1/1 ★(35) · `17Leg::handle_interact_battle::` 1/1 (345) · `old::fight_model::should_end_object_finish_kill_priority_battle` 1/1 (450) |
| `c965f0` | — | 574B | epic_action_score | `1::EpicH::score` 1/1 (10) · `1::EpicH::action_candidates` 1/1 (4895) |
| `e339d0` | — | 676B | can1v1win | `1::H::action_candidates` 1/1 (4680) |
| `c8cd30` | — | 389B | (지도 이름 콜리 없음) | — |
| `c8d570` | — | 373B | (지도 이름 콜리 없음) | — |
| `e2f4f0` | — | 915B | can1v1win | `1::H::action_candidates` 1/1 (4680) |
| `c8f880` | — | 574B | serpen_action_score | `1::SerpenH::score` 1/1 (10) · `1::SerpenH::action_candidates` 1/1 (6182) |
| `de0340` | — | 66B | should_keep_object_for_contested_wave_priority | `team_plan::8Tea::should_delay_morgard_for_wave_priority` 1/1 ★(26) · `team_plan::8Tea::should_delay_serpen_for_wave_priority` 1/1 ★(29) · `team_plan::8Tea::should_delay_object_setup_for_wave_priority` 1/1 (73) |
| `ecb440` | — | 260B | v23_enemy_object_pressure · v23_healthy_allies_near_point · v23_recent_visible_enemies_nea | `team_plan::objective_helpers::v25_objective_splitter_should_join_contest` 3/3 ★(52) · `team_plan::objective_helpers::v23_should_break_objective_hunt_anchor` 2/3 (50) · `team_plan::objective_helpers::v25_objective_splitter_can_stay` 2/3 (122) |
| `ca8180` | — | 282B | (지도 이름 콜리 없음) | — |
| `c984e0` | — | 388B | (지도 이름 콜리 없음) | — |
| `d70ae0` | — | 226B | engage_requires_dive · v3_survival_incoming | `modes::17L::single_try_engage` 1/2 (331) · `engage::17L::try_engage` 1/2 (495) · `auction::17L::get_small_action` 1/2 (6294) |
| `d709f0` | — | 226B | engage_requires_dive · v3_survival_incoming | `modes::17L::single_try_engage` 1/2 (331) · `engage::17L::try_engage` 1/2 (495) · `auction::17L::get_small_action` 1/2 (6294) |
| `e1db70` | — | 428B | is_wave_priority_start_line | `g_7com::s::` 1/1 ★(218) · `find_wave_priority_clear_line::5FnMut::f5S1uJLkH_9game_core10simulation5state6player8LineTypeEE8call_mutBY_` 1/1 (12) |
| `e02bc0` | — | 871B | can1v1win | `1::H::action_candidates` 1/1 (4680) |
| `ca82f0` | — | 284B | (지도 이름 콜리 없음) | — |
| `ca8460` | — | 284B | (지도 이름 콜리 없음) | — |
| `df1f50` | — | 5418B | get_die_tick_player · next_plan · target_bush_v30 | `1::LineGan::next_plan` 2/3 (1648) |
| `dcea30` | — | 750B | end_check · handle_nexus_attack | `team_plan::8Tea::update_objective_after_steal` 2/2 (10416) · `objective_handlers::8Te::handle_nexus_attack` 1/2 (383) · `objective_handlers::8Te::handle_none_or_gank_objective` 1/2 (5610) |
| `d70bd0` | — | 226B | engage_requires_dive · v3_survival_incoming | `modes::17L::single_try_engage` 1/2 (331) · `engage::17L::try_engage` 1/2 (495) · `auction::17L::get_small_action` 1/2 (6294) |
| `d70710` | — | 226B | engage_requires_dive · v30_line_action_stance · v3_survival_incoming | — |
| `d9a9d0` | — | 168B | bush_distance_sq | `steal::steal_wait_bush` 1/1 ★(97) · `min_by::fold::INvB2N_7comparejyEE0E0EB3j_` 1/1 ★(126) · `steal::evaluate_steal_for_target` 1/1 (2156) |
| `eba320` | — | 1478B | (지도 이름 콜리 없음) | — |
| `c91520` | — | 580B | score | `max_by::fold::INvB2o_7compareB3e_xEE0EB1r_` 1/1 ★(126) · `max_by::fold::INvB2o_7compareB3e_xEE0EB1r_` 1/1 ★(126) · `max_by::fold::INvB2o_7compareB3e_xEE0EB1r_` 1/1 ★(126) |
| `d71500` | — | 224B | (지도 이름 콜리 없음) | — |
| `ca5da0` | — | 256B | evaluate_gank_opportunity_with_score | `lead_action_v37::0EEB::K_` 1/1 ★(178) · `chat::17L::handle_chat_inner` 1/1 (3322) · `1::LegacyP::update` 1/1 (13249) |
| `c98290` | — | 580B | score | `max_by::fold::INvB2o_7compareB3e_xEE0EB1r_` 1/1 ★(126) · `max_by::fold::INvB2o_7compareB3e_xEE0EB1r_` 1/1 ★(126) · `max_by::fold::INvB2o_7compareB3e_xEE0EB1r_` 1/1 ★(126) |
| `c8b710` | — | 433B | clone | `small_action::SmallActionPlay::call_mut` 1/1 ★(133) · `clone::Clone::clone` 1/1 ★(151) · `Iterator::next::T` 1/1 ★(76) |
| `dfdd50` | — | 481B | base_sub_goal | `1::DeathM::new` 1/1 ★(123) · `1::DeathM::new_dive` 1/1 ★(123) · `battle::10Ba::new_region` 1/1 ★(115) |
| `c9d5b0` | — | 305B | (지도 이름 콜리 없음) | — |
| `ca0a10` | — | 324B | (지도 이름 콜리 없음) | — |
| `ca4790` | — | 280B | (지도 이름 콜리 없음) | — |
| `d66a10` | — | 1477B | (지도 이름 콜리 없음) | — |
| `d69690` | — | 2282B | (지도 이름 콜리 없음) | — |
| `e04c60` | — | 643B | with | — |
| `e0ad90` | — | 667B | (지도 이름 콜리 없음) | — |
| `e704b0` | — | 227B | (지도 이름 콜리 없음) | — |
| `ebe790` | — | 821B | (지도 이름 콜리 없음) | — |
| `d96b10` | — | 485B | (지도 이름 콜리 없음) | — |
| `d6fc40` | — | 1190B | (지도 이름 콜리 없음) | — |
| `d71230` | — | 283B | (지도 이름 콜리 없음) | — |
| `d3c610` | — | 108B | with | — |
| `d3c680` | — | 114B | with | — |
| `e03ed0` | — | 1324B | (지도 이름 콜리 없음) | — |
| `e26230` | — | 1135B | (지도 이름 콜리 없음) | — |
| `e248f0` | — | 1731B | (지도 이름 콜리 없음) | — |
| `e9bf40` | — | 100B | (지도 이름 콜리 없음) | — |
| `e34d50` | — | 930B | with | — |
| `e35160` | — | 294B | with | — |
| `e34a30` | — | 795B | (지도 이름 콜리 없음) | — |
| `ebe220` | — | 351B | estimate_damage_to | — |
| `ca0d30` | — | 291B | (지도 이름 콜리 없음) | — |
| `d6a860` | — | 889B | (지도 이름 콜리 없음) | — |
| `ca4900` | — | 303B | (지도 이름 콜리 없음) | — |