# exe → IR 정체 추정(extid · 상수 지문)

| rva | 현재 이름 | exe B / 상수 | IR 후보 1(공유·IR명령·크기비) | 후보 2 | 판정 |
|---|---|---|---|---|---|
| `d70620` | tower_discipline 공용 래퍼(aggro_damage·engage_requires_dive·survival_incoming 호출 · 9.9억회 · 지문 없음) | 226B / 12 | `x::position_eval_at::z` 공유 4 · 점수 6.6 · IR 102 · 비 0.55 | `rngs::std::StdRng` 공유 4 · 점수 6.6 · IR 152 · 비 0.37 | 후보(공유 4) |
| `d95d00` | game_ai simulation.rs:1848 캐시 래퍼(cached_damage_against 호출 5.2억회) | 1154B / 65 | `2::En::new` 공유 23 · 점수 63.5 · IR 1719 · 비 0.17 | `clone::Clone::clone` 공유 22 · 점수 62.5 · IR 988 · 비 0.29 | 후보(공유 23) |
| `e25450` | can_tower_focused 캐시 래퍼(1.04억회) | 952B / 56 | `clone::Clone::clone` 공유 21 · 점수 69.2 · IR 988 · 비 0.24 | `1::Wind::new` 공유 14 · 점수 45.2 · IR 342 · 비 0.70 | 후보(공유 21) |
| `e0cf10` | max_range_cached TLS 래퍼(1.44억회) | 889B / 48 | `handler::check_kill` 공유 15 · 점수 50.1 · IR 1471 · 비 0.15 | `utils::precompute_champion_powers` 공유 14 · 점수 47.2 · IR 709 · 비 0.31 | 후보(공유 15) |
| `ca89a0` | SerpenStanceData::update_plan 클로저 호출부 | 240B / 13 | `1::WaitA::new` 공유 7 · 점수 13.9 · IR 84 · 비 0.71 | `abstract_input::wait_around` 공유 7 · 점수 13.9 · IR 89 · 비 0.67 | 후보(공유 7) |
| `d96190` | simulation.rs:1848/game.rs:210 래퍼(estimate_damage_to 호출) | 2418B / 117 | `MAX_RANGE_CACHE::rust_std_internal_init_f::X` 공유 41 · 점수 136.3 · IR 3053 · 비 0.20 | `FIGHT_KIT_RANGE_CACHE::rust_std_internal_init_f::X` 공유 41 · 점수 136.3 · IR 3053 · 비 0.20 | 후보(공유 41) |
| `ca8f60` | EpicStanceData::update_plan 클로저 호출부 | 240B / 13 | `team_plan::8Tea::can_near_enemies_range` 공유 7 · 점수 13.6 · IR 162 · 비 0.37 | `1::WaitA::new` 공유 7 · 점수 13.3 · IR 84 · 비 0.71 | 후보(공유 7) |
| `eb8b00` | available_cc_in_window 래퍼 | 123B / 12 | `position_eval::entity_positioning_cache_cached` 공유 9 · 점수 20.3 · IR 106 · 비 0.29 | `team_plan::8Tea::can_near_enemies_range` 공유 9 · 점수 19.8 · IR 162 · 비 0.19 | 후보(공유 9) |
| `e0d720` | support_min_action_range 래퍼 | 889B / 48 | `handler::check_kill` 공유 15 · 점수 50.1 · IR 1471 · 비 0.15 | `utils::precompute_champion_powers` 공유 14 · 점수 47.2 · IR 709 · 비 0.31 | 후보(공유 15) |
| `e49a50` | LegacyPlanHandler::update 아웃라인 조각(GoalData::update·passive_plan·handle_chat 호출) | 2734B / 121 | `chat::17L::handle_chat_inner` 공유 41 · 점수 212.2 · IR 3322 · 비 0.21 · 콜리✓ | `MAX_RANGE_CACHE::rust_std_internal_init_f::X` 공유 42 · 점수 206.2 · IR 3053 · 비 0.22 | 후보(공유 41) |
| `d70530` | position_eval 공용 래퍼(count_in_range_fold·engage_requires_dive 호출) | 226B / 11 | `x::position_eval_at::z` 공유 4 · 점수 6.7 · IR 102 · 비 0.55 | `rngs::std::StdRng` 공유 4 · 점수 6.7 · IR 152 · 비 0.37 | 후보(공유 4) |
| `eb5dd0` | — | 742B / 52 | `ptr::drop_glue::f5S1uJLkH_9game_core10simulation6entity6EntityECshdEBA0ozCnw_7game_ai` 공유 11 · 점수 48.3 · IR 1005 · 비 0.18 | `defense_nexus::19D::score` 공유 14 · 점수 42.7 · IR 541 · 비 0.34 | 후보(공유 11) |
| `c9ef30` | — | 267B / 13 | `1::WaitA::new` 공유 7 · 점수 14.5 · IR 84 · 비 0.79 | `abstract_input::wait_around` 공유 7 · 점수 14.5 · IR 89 · 비 0.75 | 후보(공유 7) |
| `dd9f30` | — | 745B / 52 | `clone::Clone::clone` 공유 31 · 점수 99.7 · IR 988 · 비 0.19 | `1::Wind::new` 공유 28 · 점수 92.2 · IR 342 · 비 0.54 | 후보(공유 31) |
| `e9bf10` | upgrade_item 래퍼 | 35B / 2 | — | — | — |
| `e9c610` | buy_item 래퍼 | 25B / 4 | `death_scene::21No::new` 공유 2 · 점수 4.6 · IR 14 · 비 0.45 | `1::DeathSce::new` 공유 2 · 점수 4.5 · IR 22 · 비 0.28 | — |
| `ca1560` | — | 275B / 13 | `1::WaitA::new` 공유 7 · 점수 14.4 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 7 · 점수 14.4 · IR 89 · 비 0.77 | 후보(공유 7) |
| `ca6700` | SubPlan::score 디스패처(serpen_check·steal) | 1007B / 61 | `utils::precompute_champion_powers` 공유 28 · 점수 115.6 · IR 709 · 비 0.36 | `calculate_positioning_score::0E::u_` 공유 21 · 점수 85.6 · IR 512 · 비 0.49 | 후보(공유 28) |
| `d958b0` | simulation.rs:1848 래퍼 | 1102B / 61 | `2::En::new` 공유 22 · 점수 60.6 · IR 1719 · 비 0.16 | `clone::Clone::clone` 공유 21 · 점수 57.7 · IR 988 · 비 0.28 | 후보(공유 22) |
| `c8d140` | — | 405B / 27 | `2::SmallActi::has_safe_minion_attack_stance` 공유 15 · 점수 46.0 · IR 370 · 비 0.27 | `1::Wind::new` 공유 15 · 점수 45.9 · IR 342 · 비 0.30 | 후보(공유 15) |
| `ca19b0` | — | 267B / 13 | `1::WaitA::new` 공유 7 · 점수 14.5 · IR 84 · 비 0.79 | `abstract_input::wait_around` 공유 7 · 점수 14.5 · IR 89 · 비 0.75 | 후보(공유 7) |
| `caf2e0` | BigPlan::update JT 디스패처(passive_line·battle·line_ganker update) | 441B / 28 | `types::7::update` 공유 4 · 점수 27.8 · IR 99 · 비 0.90 · 콜리✓ | `battle::10Ba::new_region` 공유 8 · 점수 27.7 · IR 115 · 비 0.96 | 후보(공유 4) |
| `d758e0` | — | 323B / 25 | `minion_wave_risk::enemy_minion_line_action_damage_at` 공유 6 · 점수 15.7 · IR 477 · 비 0.17 | `1::Jun::score` 공유 5 · 점수 14.8 · IR 221 · 비 0.37 | 후보(공유 6) |
| `e35bd0` | SubPlan::action_candidates JT 디스패처(서브플랜 15종 전부를 여기서 호출) | 1230B / 67 | `1::EpicCh::action_candidates` 공유 12 · 점수 31.0 · IR 1250 · 비 0.25 | `clone::Clone::clone` 공유 11 · 점수 27.4 · IR 988 · 비 0.31 | 후보(공유 12) |
| `d96d00` | tower_discipline.rs:533 LocalKey::with 캐시 래퍼 | 1311B / 77 | `utils::build_minion_wave_snapshot` 공유 33 · 점수 92.9 · IR 2103 · 비 0.16 | `2::En::new` 공유 29 · 점수 82.3 · IR 1719 · 비 0.19 | 후보(공유 33) |
| `e27c70` | — | 476B / 29 | `2::SmallActi::choose_goal` 공유 15 · 점수 39.6 · IR 475 · 비 0.25 | `steal::steal_damage_and_range_within` 공유 12 · 점수 34.1 · IR 509 · 비 0.23 | 후보(공유 15) |
| `c88300` | action_score.rs:577/594 interaction 클로저·TLS 래퍼 | 4592B / 132 | `MAX_RANGE_CACHE::rust_std_internal_init_f::X` 공유 38 · 점수 123.1 · IR 3053 · 비 0.38 | `FIGHT_KIT_RANGE_CACHE::rust_std_internal_init_f::X` 공유 38 · 점수 123.1 · IR 3053 · 비 0.38 | 후보(공유 38) |
| `e360e0` | SubPlan::calculate_score_parameter_value 디스패처(line_defense·battle) | 9844B / 130 | `AgentVerHamster::8::clone_bo` 공유 20 · 점수 86.4 · IR 3153 · 비 0.78 | `MAX_RANGE_CACHE::rust_std_internal_init_f::X` 공유 20 · 점수 83.3 · IR 3053 · 비 0.81 | 후보(공유 20) |
| `c986c0` | is_cleared 호출 헬퍼 | 253B / 11 | `1::Passive::update` 공유 4 · 점수 12.7 · IR 96 · 비 0.66 · 콜리✓ | `buff_value::defensive_crisis` 공유 6 · 점수 11.8 · IR 401 · 비 0.16 | 후보(공유 4) |
| `d717e0` | — | 286B / 18 | `key::f5S1uJLkH_9game_core10simulation6entity6EntityyNCB2h_s5_0E0EB3W_4foldTyB4K_ENCINvNvB3W_6min_by4fo::INvB3U_7compareB4K_yEE0EB2p_` 공유 5 · 점수 11.4 · IR 209 · 비 0.34 | `t_s1::K_E::x_` 공유 5 · 점수 11.3 · IR 149 · 비 0.48 | 후보(공유 5) |
| `d70900` | — | 226B / 12 | `x::position_eval_at::z` 공유 4 · 점수 6.6 · IR 102 · 비 0.55 | `rngs::std::StdRng` 공유 4 · 점수 6.6 · IR 152 · 비 0.37 | 후보(공유 4) |
| `e0b030` | — | 1445B / 63 | `clone::Clone::clone` 공유 30 · 점수 91.6 · IR 988 · 비 0.37 | `utils::build_minion_wave_snapshot` 공유 31 · 점수 88.4 · IR 2103 · 비 0.17 | 후보(공유 30) |
| `ca33c0` | — | 275B / 13 | `1::WaitA::new` 공유 7 · 점수 14.4 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 7 · 점수 14.4 · IR 89 · 비 0.77 | 후보(공유 7) |
| `e03360` | simulation.rs:1905 래퍼 | 2922B / 110 | `MAX_RANGE_CACHE::rust_std_internal_init_f::X` 공유 47 · 점수 152.2 · IR 3053 · 비 0.24 | `FIGHT_KIT_RANGE_CACHE::rust_std_internal_init_f::X` 공유 47 · 점수 152.2 · IR 3053 · 비 0.24 | 후보(공유 47) |
| `c93f60` | v30 tower_aggro_risk 래퍼 | 478B / 28 | `18LineDefenseSubPlan1::action_::0EB` 공유 4 · 점수 18.1 · IR 154 · 비 0.78 · 콜리✓ | `line_defense::18LineDefenseSubPlan1::action_` 공유 4 · 점수 18.1 · IR 154 · 비 0.78 · 콜리✓ | 후보(공유 4) |
| `e25030` | — | 1045B / 66 | `small_action::cast::is_safe_recall` 공유 26 · 점수 65.4 · IR 1636 · 비 0.16 | `2::En::new` 공유 23 · 점수 60.2 · IR 1719 · 비 0.15 | 후보(공유 26) |
| `c94140` | LineDefenseSubPlan::score 래퍼 | 960B / 61 | `clone::Clone::clone` 공유 24 · 점수 71.0 · IR 988 · 비 0.24 | `1::Wind::new` 공유 19 · 점수 61.4 · IR 342 · 비 0.70 | 후보(공유 24) |
| `ca4650` | — | 236B / 13 | `team_plan::8Tea::can_near_enemies_range` 공유 7 · 점수 14.1 · IR 162 · 비 0.36 | `1::WaitA::new` 공유 7 · 점수 13.9 · IR 84 · 비 0.70 | 후보(공유 7) |
| `d6abe0` | simulation.rs:1905 래퍼(v21_defensive_cc_score 호출) | 1656B / 81 | `2::En::new` 공유 37 · 점수 111.8 · IR 1719 · 비 0.24 | `utils::build_minion_wave_snapshot` 공유 36 · 점수 102.6 · IR 2103 · 비 0.20 | 후보(공유 37) |
| `c9f210` | — | 274B / 14 | `1::WaitA::new` 공유 8 · 점수 16.8 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 8 · 점수 16.8 · IR 89 · 비 0.77 | 후보(공유 8) |
| `ea0c00` | line_defense.rs:125 클로저(max_range_nearly_can_use 호출) | 348B / 25 | `death_battle::deathmatch_dodge_danger` 공유 12 · 점수 24.1 · IR 535 · 비 0.16 | `engage::17L::v2_response_retreat_stance` 공유 11 · 점수 22.8 · IR 394 · 비 0.22 | 후보(공유 12) |
| `c94500` | v19 non_champion_walkup 래퍼 | 644B / 36 | `old::fight_model::resolve_fight_full` 공유 15 · 점수 38.6 · IR 1030 · 비 0.16 | `clone::Clone::clone` 공유 15 · 점수 37.4 · IR 988 · 비 0.16 | 후보(공유 15) |
| `cafe10` | BigPlan::next_plan JT 디스패처(passive_jungle·line_ganker) | 237B / 17 | `tower_discipline::can_trace_without_tower` 공유 11 · 점수 30.5 · IR 163 · 비 0.36 | `2::SmallActi::has_safe_minion_attack_stance` 공유 10 · 점수 28.5 · IR 370 · 비 0.16 | 후보(공유 11) |
| `d84c60` | abstract_input::attack 래퍼(position_eval 근방) | 325B / 18 | `1::WaitA::get_input` 공유 8 · 점수 17.8 · IR 151 · 비 0.54 | `abstract_input::maintain_distance` 공유 8 · 점수 17.8 · IR 209 · 비 0.39 | 후보(공유 8) |
| `d2e3f0` | — | 265B / 21 | `battle::10Ba::new_region` 공유 8 · 점수 22.3 · IR 115 · 비 0.58 | `battle::10Ba::new` 공유 8 · 점수 22.3 · IR 158 · 비 0.42 | 후보(공유 8) |
| `d37680` | — | 129B / 11 | `get_input::::` 공유 8 · 점수 18.2 · IR 151 · 비 0.21 | `get_input::0E0::` 공유 8 · 점수 18.2 · IR 151 · 비 0.21 | 후보(공유 8) |
| `ec9d60` | — | 115B / 5 | `1::OpenChu::new` 공유 2 · 점수 4.4 · IR 12 · 비 0.42 | `death_scene::17I::new` 공유 2 · 점수 4.4 · IR 12 · 비 0.42 | — |
| `dd7250` | — | 351B / 25 | `1::LegacyP::new` 공유 9 · 점수 23.4 · IR 561 · 비 0.16 | `2::SmallActi::has_safe_minion_attack_stance` 공유 8 · 점수 19.1 · IR 370 · 비 0.24 | 후보(공유 9) |
| `d663f0` | simulation.rs:1905 래퍼 | 405B / 19 | `team_plan::8Tea::can_near_enemies` 공유 10 · 점수 20.2 · IR 143 · 비 0.71 | `::team_plan::8TeamPlan16can_near_enemies0ENCB3Y_s_0EEB48` 공유 10 · 점수 20.0 · IR 325 · 비 0.31 | 후보(공유 10) |
| `dea420` | — | 120B / 12 | `2::SmallActi::has_current_explicit_minion_action` 공유 5 · 점수 12.1 · IR 156 · 비 0.19 | `1::Rec::score` 공유 4 · 점수 10.4 · IR 169 · 비 0.18 | 후보(공유 5) |
| `e26080` | — | 418B / 24 | `2::SmallActi::has_safe_minion_attack_stance` 공유 10 · 점수 29.1 · IR 370 · 비 0.28 | `tower_discipline::can_trace_without_tower` 공유 8 · 점수 28.4 · IR 163 · 비 0.64 · 콜리✓ | 후보(공유 10) |
| `ca1840` | — | 275B / 13 | `1::WaitA::new` 공유 7 · 점수 14.4 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 7 · 점수 14.4 · IR 89 · 비 0.77 | 후보(공유 7) |
| `ca2ad0` | — | 267B / 13 | `1::WaitA::new` 공유 7 · 점수 14.5 · IR 84 · 비 0.79 | `abstract_input::wait_around` 공유 7 · 점수 14.5 · IR 89 · 비 0.75 | 후보(공유 7) |
| `d1a480` | — | 421B / 23 | `23Smal::get_input::0` 공유 10 · 점수 24.5 · IR 193 · 비 0.55 | `around::23Smal::get_input` 공유 10 · 점수 24.5 · IR 193 · 비 0.55 | 후보(공유 10) |
| `e27f50` | — | 257B / 20 | `1::Wind::new` 공유 6 · 점수 19.3 · IR 342 · 비 0.19 | `minion_wave_risk::enemy_minion_wave_risk_damage_at` 공유 6 · 점수 16.6 · IR 333 · 비 0.19 | 후보(공유 6) |
| `ca16d0` | — | 275B / 13 | `1::WaitA::new` 공유 7 · 점수 14.4 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 7 · 점수 14.4 · IR 89 · 비 0.77 | 후보(공유 7) |
| `ca2960` | — | 274B / 14 | `1::WaitA::new` 공유 8 · 점수 16.8 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 8 · 점수 16.8 · IR 89 · 비 0.77 | 후보(공유 8) |
| `dfb5d0` | — | 481B / 33 | `battle::10Ba::new_region` 공유 25 · 점수 101.7 · IR 115 · 비 0.96 · 콜리✓ | `battle::10Ba::new` 공유 25 · 점수 95.7 · IR 158 · 비 0.76 | 후보(공유 25) |
| `e89fe0` | — | 273B / 15 | `buff_value::v55_mark_value` 공유 7 · 점수 18.4 · IR 344 · 비 0.20 | `path_finder::18En::new_filtered` 공유 6 · 점수 18.3 · IR 398 · 비 0.17 | 후보(공유 7) |
| `ca1ca0` | — | 274B / 14 | `1::WaitA::new` 공유 8 · 점수 16.8 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 8 · 점수 16.8 · IR 89 · 비 0.77 | 후보(공유 8) |
| `ca0240` | — | 274B / 14 | `1::WaitA::new` 공유 8 · 점수 16.8 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 8 · 점수 16.8 · IR 89 · 비 0.77 | 후보(공유 8) |
| `cd5ee0` | — | 1193B / 54 | `2::SmallActi::choose_goal` 공유 23 · 점수 63.1 · IR 475 · 비 0.63 | `2::En::new` 공유 22 · 점수 61.0 · IR 1719 · 비 0.17 | 후보(공유 23) |
| `de0390` | — | 51B / 6 | `team_plan::8Tea::should_delay_morgard_for_wave_priority` 공유 2 · 점수 14.9 · IR 26 · 비 0.49 · 콜리✓ | `team_plan::8Tea::should_delay_serpen_for_wave_priority` 공유 2 · 점수 14.9 · IR 29 · 비 0.44 · 콜리✓ | — |
| `ca13f0` | — | 274B / 14 | `1::WaitA::new` 공유 8 · 점수 16.8 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 8 · 점수 16.8 · IR 89 · 비 0.77 | 후보(공유 8) |
| `db50d0` | — | 565B / 27 | `23Smal::get_input::0` 공유 12 · 점수 32.5 · IR 193 · 비 0.73 | `around::23Smal::get_input` 공유 12 · 점수 32.5 · IR 193 · 비 0.73 | 후보(공유 12) |
| `c8d360` | — | 389B / 24 | `battle::10Ba::new_region` 공유 12 · 점수 37.9 · IR 115 · 비 0.85 | `battle::10Ba::new` 공유 12 · 점수 37.9 · IR 158 · 비 0.62 | 후보(공유 12) |
| `c8cf40` | — | 373B / 22 | `tower_discipline::can_trace_without_tower` 공유 11 · 점수 32.6 · IR 163 · 비 0.57 | `2::SmallActi::has_safe_minion_attack_stance` 공유 11 · 점수 32.3 · IR 370 · 비 0.25 | 후보(공유 11) |
| `ca0eb0` | — | 275B / 13 | `1::WaitA::new` 공유 7 · 점수 14.4 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 7 · 점수 14.4 · IR 89 · 비 0.77 | 후보(공유 7) |
| `c9fd60` | — | 274B / 14 | `1::WaitA::new` 공유 8 · 점수 16.8 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 8 · 점수 16.8 · IR 89 · 비 0.77 | 후보(공유 8) |
| `ca0560` | — | 274B / 14 | `1::WaitA::new` 공유 8 · 점수 16.8 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 8 · 점수 16.8 · IR 89 · 비 0.77 | 후보(공유 8) |
| `ca2c30` | — | 275B / 13 | `1::WaitA::new` 공유 7 · 점수 14.4 · IR 84 · 비 0.82 | `abstract_input::wait_around` 공유 7 · 점수 14.4 · IR 89 · 비 0.77 | 후보(공유 7) |
| `c965f0` | — | 574B / 34 | `1::Wind::new` 공유 9 · 점수 24.8 · IR 342 · 비 0.42 | `1::LegacyP::new` 공유 10 · 점수 21.3 · IR 561 · 비 0.26 | 후보(공유 9) |
| `e339d0` | — | 676B / 35 | `2::SmallActi::choose_goal` 공유 20 · 점수 49.0 · IR 475 · 비 0.36 | `abstract_input::kite_in_boundary` 공유 20 · 점수 47.1 · IR 487 · 비 0.35 | 후보(공유 20) |
| `c8cd30` | — | 389B / 24 | `battle::10Ba::new_region` 공유 12 · 점수 37.9 · IR 115 · 비 0.85 | `battle::10Ba::new` 공유 12 · 점수 37.9 · IR 158 · 비 0.62 | 후보(공유 12) |
| `c8d570` | — | 373B / 22 | `tower_discipline::can_trace_without_tower` 공유 11 · 점수 32.6 · IR 163 · 비 0.57 | `2::SmallActi::has_safe_minion_attack_stance` 공유 11 · 점수 32.3 · IR 370 · 비 0.25 | 후보(공유 11) |
| `e2f4f0` | — | 915B / 45 | `2::SmallActi::choose_goal` 공유 26 · 점수 73.8 · IR 475 · 비 0.48 | `2::SmallActi::has_safe_minion_attack_stance` 공유 24 · 점수 67.5 · IR 370 · 비 0.62 | 후보(공유 26) |
| `c8f880` | — | 574B / 34 | `1::Wind::new` 공유 9 · 점수 24.8 · IR 342 · 비 0.42 | `1::LegacyP::new` 공유 10 · 점수 21.3 · IR 561 · 비 0.26 | 후보(공유 9) |
| `de0340` | — | 66B / 8 | `team_plan::8Tea::should_delay_morgard_for_wave_priority` 공유 2 · 점수 14.6 · IR 26 · 비 0.63 · 콜리✓ | `team_plan::8Tea::should_delay_serpen_for_wave_priority` 공유 2 · 점수 14.6 · IR 29 · 비 0.57 · 콜리✓ | — |
| `ecb440` | — | 260B / 18 | `2::SmallActi::has_safe_minion_attack_stance` 공유 8 · 점수 19.3 · IR 370 · 비 0.18 | `position_eval::entity_positioning_cache_cached` 공유 8 · 점수 17.3 · IR 106 · 비 0.61 | 후보(공유 8) |
| `ca8180` | — | 282B / 14 | `1::WaitA::new` 공유 8 · 점수 16.6 · IR 84 · 비 0.84 | `abstract_input::wait_around` 공유 8 · 점수 16.6 · IR 89 · 비 0.79 | 후보(공유 8) |
| `c984e0` | — | 388B / 18 | `tower_discipline::can_trace_without_tower` 공유 8 · 점수 16.6 · IR 163 · 비 0.60 | `2::SmallActi::has_safe_minion_attack_stance` 공유 7 · 점수 14.7 · IR 370 · 비 0.26 | 후보(공유 8) |
| `d70ae0` | — | 226B / 12 | `x::position_eval_at::z` 공유 4 · 점수 6.6 · IR 102 · 비 0.55 | `rngs::std::StdRng` 공유 4 · 점수 6.6 · IR 152 · 비 0.37 | 후보(공유 4) |
| `d709f0` | — | 226B / 12 | `x::position_eval_at::z` 공유 4 · 점수 6.6 · IR 102 · 비 0.55 | `rngs::std::StdRng` 공유 4 · 점수 6.6 · IR 152 · 비 0.37 | 후보(공유 4) |
| `e1db70` | — | 428B / 29 | `1::WaitA::get_input` 공유 9 · 점수 20.8 · IR 151 · 비 0.71 | `abstract_input::maintain_distance` 공유 9 · 점수 20.8 · IR 209 · 비 0.51 | 후보(공유 9) |
| `e02bc0` | — | 871B / 54 | `clone::Clone::clone` 공유 25 · 점수 75.3 · IR 988 · 비 0.22 | `2::SmallActi::choose_goal` 공유 24 · 점수 65.1 · IR 475 · 비 0.46 | 후보(공유 25) |
| `ca82f0` | — | 284B / 15 | `1::WaitA::new` 공유 9 · 점수 19.3 · IR 84 · 비 0.85 | `abstract_input::wait_around` 공유 9 · 점수 19.3 · IR 89 · 비 0.80 | 후보(공유 9) |
| `ca8460` | — | 284B / 14 | `1::WaitA::new` 공유 8 · 점수 16.8 · IR 84 · 비 0.85 | `abstract_input::wait_around` 공유 8 · 점수 16.8 · IR 89 · 비 0.80 | 후보(공유 8) |
| `df1f50` | — | 5418B / 248 | `MAX_RANGE_CACHE::rust_std_internal_init_f::X` 공유 99 · 점수 466.7 · IR 3053 · 비 0.44 | `FIGHT_KIT_RANGE_CACHE::rust_std_internal_init_f::X` 공유 99 · 점수 466.7 · IR 3053 · 비 0.44 | 후보(공유 99) |
| `dcea30` | — | 750B / 42 | `clone::Clone::clone` 공유 10 · 점수 24.8 · IR 988 · 비 0.19 | `utils::line_backfight_support_focus` 공유 12 · 점수 24.7 · IR 902 · 비 0.21 | 후보(공유 10) |
| `d70bd0` | — | 226B / 12 | `x::position_eval_at::z` 공유 4 · 점수 6.6 · IR 102 · 비 0.55 | `rngs::std::StdRng` 공유 4 · 점수 6.6 · IR 152 · 비 0.37 | 후보(공유 4) |
| `d70710` | — | 226B / 12 | `x::position_eval_at::z` 공유 4 · 점수 6.6 · IR 102 · 비 0.55 | `rngs::std::StdRng` 공유 4 · 점수 6.6 · IR 152 · 비 0.37 | 후보(공유 4) |
| `d9a9d0` | — | 168B / 5 | `min_by::fold::INvB2N_7comparejyEE0E0EB3j_` 공유 2 · 점수 10.6 · IR 126 · 비 0.33 · 콜리✓ | `steal::steal_wait_bush` 공유 2 · 점수 10.3 · IR 97 · 비 0.43 · 콜리✓ | — |
| `eba320` | — | 1478B / 94 | `LAG_MEMO::rust_std_internal_init_f::E_` 공유 27 · 점수 97.7 · IR 2199 · 비 0.17 | `utils::precompute_champion_powers` 공유 25 · 점수 90.5 · IR 709 · 비 0.52 | 후보(공유 27) |
| `c91520` | — | 580B / 36 | `vec::3V::from_iter_in` 공유 12 · 점수 24.8 · IR 619 · 비 0.23 | `abstract_input::wait_around` 공유 12 · 점수 24.7 · IR 89 · 비 0.61 | 후보(공유 12) |
| `d71500` | — | 224B / 16 | `x::position_eval_at::z` 공유 4 · 점수 6.3 · IR 102 · 비 0.55 | `rngs::std::StdRng` 공유 4 · 점수 6.3 · IR 152 · 비 0.37 | 후보(공유 4) |
| `ca5da0` | — | 256B / 11 | `fight_check::v48_projectile_profile` 공유 3 · 점수 7.4 · IR 389 · 비 0.16 | `1::Single::new_region` 공유 4 · 점수 7.4 · IR 55 · 비 0.86 | 후보(공유 3) |
| `c98290` | — | 580B / 36 | `vec::3V::from_iter_in` 공유 12 · 점수 24.8 · IR 619 · 비 0.23 | `abstract_input::wait_around` 공유 12 · 점수 24.7 · IR 89 · 비 0.61 | 후보(공유 12) |
| `c8b710` | — | 433B / 23 | `23Smal::get_input::0` 공유 10 · 점수 24.5 · IR 193 · 비 0.56 | `around::23Smal::get_input` 공유 10 · 점수 24.5 · IR 193 · 비 0.56 | 후보(공유 10) |
| `dfdd50` | — | 481B / 33 | `battle::10Ba::new_region` 공유 25 · 점수 101.7 · IR 115 · 비 0.96 · 콜리✓ | `battle::10Ba::new` 공유 25 · 점수 95.7 · IR 158 · 비 0.76 | 후보(공유 25) |
| `c9d5b0` | — | 305B / 12 | `move_actions::17Sm::is_end` 공유 6 · 점수 10.9 · IR 123 · 비 0.62 | `::5FnMut::f5S1uJLkH_9game_core10simulation6entity6EntityEEE8call_mutBW_` 공유 6 · 점수 10.5 · IR 298 · 비 0.26 | 후보(공유 6) |
| `ca0a10` | — | 324B / 11 | `around::29Sm::get_input` 공유 4 · 점수 7.1 · IR 133 · 비 0.61 | `old::fight_model::resolve_fight_stake` 공유 4 · 점수 7.1 · IR 317 · 비 0.26 | 후보(공유 4) |
| `ca4790` | — | 280B / 10 | `around::29Sm::get_input` 공유 4 · 점수 7.2 · IR 133 · 비 0.53 | `old::fight_model::resolve_fight_stake` 공유 4 · 점수 7.1 · IR 317 · 비 0.22 | 후보(공유 4) |
| `d66a10` | — | 1477B / 55 | `utils::build_minion_wave_snapshot` 공유 18 · 점수 44.3 · IR 2103 · 비 0.18 | `::15Ag::update_small_action` 공유 15 · 점수 43.5 · IR 1699 · 비 0.22 | 후보(공유 18) |
| `d69690` | — | 2282B / 86 | `action_score::interaction_score` 공유 23 · 점수 65.9 · IR 2550 · 비 0.22 | `AgentVerHamster::8::clone_bo` 공유 19 · 점수 57.3 · IR 3153 · 비 0.18 | 후보(공유 23) |
| `e04c60` | — | 643B / 30 | `2::SmallActi::has_safe_minion_attack_stance` 공유 12 · 점수 30.1 · IR 370 · 비 0.43 | `2::SmallActi::choose_goal` 공유 12 · 점수 30.0 · IR 475 · 비 0.34 | 후보(공유 12) |
| `e0ad90` | — | 667B / 48 | `clone::Clone::clone` 공유 23 · 점수 66.8 · IR 988 · 비 0.17 | `2::SmallActi::has_safe_minion_attack_stance` 공유 20 · 점수 51.8 · IR 370 · 비 0.45 | 후보(공유 23) |
| `e704b0` | — | 227B / 13 | `compare::h_yEE0E::a_` 공유 5 · 점수 12.7 · IR 303 · 비 0.19 | `e_B::j_E::x_` 공유 5 · 점수 9.3 · IR 132 · 비 0.43 | 후보(공유 5) |
| `ebe790` | — | 821B / 23 | `battle::14B::base_sub_goal` 공유 9 · 점수 16.5 · IR 663 · 비 0.31 | `engage::17L::v2_response_retreat_stance` 공유 9 · 점수 16.5 · IR 394 · 비 0.52 | 후보(공유 9) |
| `d96b10` | — | 485B / 39 | `goal_data::8::update` 공유 9 · 점수 23.7 · IR 766 · 비 0.16 | `rule_scope::chat_allowed` 공유 8 · 점수 23.3 · IR 668 · 비 0.18 | 후보(공유 9) |
| `d6fc40` | — | 1190B / 41 | `clone::Clone::clone` 공유 25 · 점수 73.9 · IR 988 · 비 0.30 | `2::En::new` 공유 24 · 점수 69.3 · IR 1719 · 비 0.17 | 후보(공유 25) |
| `d71230` | — | 283B / 19 | `A::call_mut::N_` 공유 5 · 점수 8.6 · IR 95 · 비 0.74 | `B::call_mut::O_` 공유 5 · 점수 8.6 · IR 102 · 비 0.69 | 후보(공유 5) |
| `d3c610` | — | 108B / 8 | `get_input::0::` 공유 6 · 점수 13.9 · IR 117 · 비 0.23 | `SmallActionAroundRegion::get_input::` 공유 6 · 점수 13.9 · IR 117 · 비 0.23 | 후보(공유 6) |
| `d3c680` | — | 114B / 9 | `get_input::0::` 공유 6 · 점수 13.6 · IR 117 · 비 0.24 | `SmallActionAroundRegion::get_input::` 공유 6 · 점수 13.6 · IR 117 · 비 0.24 | 후보(공유 6) |
| `e03ed0` | — | 1324B / 60 | `buff_value::v55_spirit_trigger_value` 공유 21 · 점수 65.8 · IR 649 · 비 0.51 | `utils::build_minion_wave_snapshot` 공유 20 · 점수 49.1 · IR 2103 · 비 0.16 | 후보(공유 21) |
| `e26230` | — | 1135B / 75 | `2::SmallActi::has_safe_minion_attack_stance` 공유 36 · 점수 122.8 · IR 370 · 비 0.77 | `2::SmallActi::choose_goal` 공유 33 · 점수 110.4 · IR 475 · 비 0.60 | 후보(공유 36) |
| `e248f0` | — | 1731B / 97 | `2::En::new` 공유 46 · 점수 149.0 · IR 1719 · 비 0.25 | `clone::Clone::clone` 공유 42 · 점수 144.3 · IR 988 · 비 0.44 | 후보(공유 46) |
| `e9bf40` | — | 100B / 6 | `death_scene::21No::new` 공유 2 · 점수 4.1 · IR 14 · 비 0.56 | `fold::T_IN::7compareB1Q_xEE0EB35` 공유 2 · 점수 4.1 · IR 80 · 비 0.31 | — |
| `e34d50` | — | 930B / 55 | `clone::Clone::clone` 공유 30 · 점수 94.2 · IR 988 · 비 0.24 | `2::SmallActi::has_safe_minion_attack_stance` 공유 26 · 점수 76.5 · IR 370 · 비 0.63 | 후보(공유 30) |
| `e35160` | — | 294B / 18 | `tower_discipline::can_trace_without_tower` 공유 14 · 점수 32.9 · IR 163 · 비 0.45 | `2::SmallActi::has_safe_minion_attack_stance` 공유 13 · 점수 30.7 · IR 370 · 비 0.20 | 후보(공유 14) |
| `e34a30` | — | 795B / 27 | `SmallActionPositioning::get_input::` 공유 8 · 점수 17.5 · IR 970 · 비 0.20 | `SmallActionAroundPositionBush::get_input::` 공유 8 · 점수 17.5 · IR 970 · 비 0.20 | 후보(공유 8) |
| `ebe220` | — | 351B / 24 | `fight_check::fight_dps` 공유 9 · 점수 31.0 · IR 116 · 비 0.76 | `x::fight_kit_range_cached::D` 공유 7 · 점수 23.5 · IR 333 · 비 0.26 | 후보(공유 9) |
| `ca0d30` | — | 291B / 13 | `1::WaitA::new` 공유 7 · 점수 14.2 · IR 84 · 비 0.87 | `abstract_input::wait_around` 공유 7 · 점수 14.2 · IR 89 · 비 0.82 | 후보(공유 7) |
| `d6a860` | — | 889B / 49 | `utils::line_backfight_support_focus` 공유 16 · 점수 37.0 · IR 902 · 비 0.25 | `old::serpen::serpen_passive_plan` 공유 15 · 점수 36.2 · IR 985 · 비 0.23 | 후보(공유 16) |
| `ca4900` | — | 303B / 14 | `old::fight_model::resolve_fight_stake` 공유 6 · 점수 11.7 · IR 317 · 비 0.24 | `engage::17L::v2_response_retreat_stance` 공유 6 · 점수 11.5 · IR 394 · 비 0.19 | 후보(공유 6) |