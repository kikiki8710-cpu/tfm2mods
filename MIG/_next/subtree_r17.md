# `score_parameter::calculate_score_parameter`(0xd8eff0) + `auction::get_small_action`(0xe65b10) + `passive_line::update`(0xd28800) + `fight_model::resolve_fight_uncached`(0xe083c0) + `lib::get_input`(0xe900b0) + `utils::build_minion_wave_snapshot`(0xd377a0) + `defense_nexus::nexus_last_stand_uncached`(0xd3e660) + `steal::action_candidates`(0xcba660) + `passive_line::v46_stage1`(0xd26900) + `battle::score`(0xcc3080) + `lib::update_small_action`(0xe8e560) + `fight_check::check_kill_die_tick_uncached`(0xeba9b0) + `battle::calculate_score_parameter_value`(0xcc20a0) + `line_defense::unsafe_v19_non_champion_walkup`(0xe8a100) + `battle_common::v16_gambler_ult_cc_bonus`(0xd676c0) + `battle_common::v21_support_pressure_too_risky`(0xd69f80) + `passive_line::v46_stage2`(0xd27d50) + `lib::clone_bo`(0xe8d6f0) + `utils::precompute_champion_powers`(0xd39ba0) + `defense_nexus::nexus_final_stand_uncached`(0xd3fe50) + `battle_common::v17_escape_is_costly`(0xd67060) + `defense_nexus::score`(0xe95ae0) + `tower_discipline::v47_tower_focus_position_dangerous`(0xd98ac0) + `lib::build_game_finish_check_state`(0xe7b0b0) + `lib::count_nearby_enemies`(0xe8f850) + `line_safe::score`(0xccbb10) + `line_wait::score`(0xeb5840) + `line_defense::calculate_score_parameter_value`(0xe8aeb0) + `lib::item_v26`(0xe8fd70) + `attack_nexus::score`(0xe83080) + `tower_discipline::v30_line_champion_action_tower_aggro_risk`(0xd99f60) + `jungle::score`(0xcc5a10) + `battle_common::v15_can_keep_support_pressure`(0xd69120) + `recall::action_candidates`(0xcc5ca0) + `tower_discipline::v22_current_line_non_champion_action_tower_risk`(0xd9a220) + `around::update_state`(0xdc2330) + `recall::score`(0xcc5fc0) + `objective_handlers::handle_nexus_attack`(0xe35a40) + `fight_model::None`(0xe33570) + `fight_model::None`(0xe33700) + `hunt_and_battle::update`(0xccbeb0) + `hunt_and_battle::update`(0xccc260) + `trace::is_end`(0xcce0c0) + `move_actions::is_end`(0xdbd130) + `utils::hp_at_tick`(0xd57420) + `steal::score`(0xcbbca0) + `lib::upgrade_item`(0xe8e160) + `lib::buy_item`(0xe8fc80) 호출 서브트리 선별표 — 작업 순서 = 아래에서 위로 (생성 2026-09-16 · 게임 0.5.8 · `--minus 0xe4c5c0` 서브트리 제외)

> 생성물(`MIG\subtree_rank.py 0xr17`) · 손편집 금지. 점수 = 호출자수/크기×1000(재사용↑·크기↓ 우선). 종류: 잎=지도 내 콜리 0 · 중간=<8KB · 거대=≥8KB. **⬜도달 가능성(사장 서브트리) 봉인은 미실시** — 착수 시 `irann.py`(교훈 68)로 NA 를 먼저 찍고 이 표에서 뺄 것. 이름 `?` = 지도 미명명(dllmatch/percolate 로 이름부터).
> 합계: 서브트리 266 · 잎 69 · 중간 189 · 거대 7 · 이미 명세/ev1 61 · 제외(데스매치) 1 · game_core 경계 함수 29(명세 대상 아님 · 계약만)

| 순위 | RVA | 함수 | 모듈 | 크기 | 호출자 | 콜리 | 깊이 | 종류 | 점수 | 상태/제외 |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `0xd20830` | `?` | path_finder | 897B | 47 | 0 | 4 | 잎 | 52.4 | ⬜ |
| 2 | `0xd1faa0` | `verdict` | path_finder | 347B | 10 | 0 | 6 | 잎 | 28.82 | ⬜ |
| 3 | `0xc8c620` | `clone` | path_finder | 549B | 12 | 0 | 6 | 잎 | 21.86 | ⬜ |
| 4 | `0xd1fc00` | `is_enemy_danger_cell` | path_finder | 549B | 5 | 0 | 5 | 잎 | 9.11 | ⬜ |
| 5 | `0xdc7e60` | `?` | path_finder | 549B | 5 | 0 | 6 | 잎 | 9.11 | ⬜ |
| 6 | `0xd1b950` | `?` | path_finder | 456B | 3 | 0 | 4 | 잎 | 6.58 | ⬜ |
| 7 | `0xd1f740` | `verdict` | path_finder | 314B | 2 | 0 | 6 | 잎 | 6.37 | ⬜ |
| 8 | `0xdb65e0` | `?` | path_finder | 609B | 3 | 0 | 5 | 잎 | 4.93 | ⬜ |
| 9 | `0xd1bcc0` | `?` | path_finder | 209B | 1 | 0 | 3 | 잎 | 4.78 | ⬜ |
| 10 | `0xd200b0` | `enemy_knows_my_position` | path_finder | 454B | 2 | 0 | 3 | 잎 | 4.41 | ⬜ |
| 11 | `0xc80340` | `?` | utils | 302B | 1 | 0 | 1 | 잎 | 3.31 | ⬜ |
| 12 | `0xc872f0` | `available_cc_in_window` | fight_check | 625B | 2 | 0 | 4 | 잎 | 3.2 | ⬜ |
| 13 | `0xea0d60` | `clone_bo` | lib | 379B | 1 | 0 | 1 | 잎 | 2.64 | ⬜ |
| 14 | `0xe30210` | `is_end` | around | 394B | 1 | 0 | 3 | 잎 | 2.54 | ⬜ |
| 15 | `0xe32c60` | `get_input` | move_actions | 394B | 1 | 0 | 3 | 잎 | 2.54 | ⬜ |
| 16 | `0xd47620` | `in_reach` | action_score | 407B | 1 | 0 | 2 | 잎 | 2.46 | ⬜ |
| 17 | `0xd1bb20` | `draw_debug` | path_finder | 413B | 1 | 0 | 1 | 잎 | 2.42 | ⬜ |
| 18 | `0xd6c290` | `v17_runaway_counterattack_penalty` | battle_common | 414B | 1 | 0 | 1 | 잎 | 2.42 | ⬜ |
| 19 | `0xd1d670` | `collect_visible_enemy_positions` | path_finder | 834B | 2 | 0 | 4 | 잎 | 2.4 | ⬜ |
| 20 | `0xc80160` | `is_line_ok` | utils | 433B | 1 | 0 | 1 | 잎 | 2.31 | ⬜ |
| 21 | `0xd20bc0` | `collect_known_enemy_positions` | path_finder | 1304B | 3 | 0 | 3 | 잎 | 2.3 | ⬜ |
| 22 | `0xd470f0` | `v17_runaway_counterattack_bonus` | battle_common | 447B | 1 | 0 | 1 | 잎 | 2.24 | ⬜ |
| 23 | `0xca1fa0` | `interaction_score_cl` | action_score | 449B | 1 | 0 | 2 | 잎 | 2.23 | ⬜ |
| 24 | `0xca21c0` | `interaction_score_cl` | action_score | 449B | 1 | 0 | 3 | 잎 | 2.23 | ⬜ |
| 25 | `0xd808b0` | `v3_lethal_tower_position` | tower_discipline | 452B | 1 | 0 | 5 | 잎 | 2.21 | ⬜ |
| 26 | `0xc7ee80` | `position_eval_cache_probe` | position_eval | 574B | 1 | 0 | 3 | 잎 | 1.74 | ⬜ |
| 27 | `0xd1f2a0` | `new` | path_finder | 1176B | 2 | 0 | 3 | 잎 | 1.7 | ⬜ |
| 28 | `0xc875a0` | `v48_projectile_profile` | fight_check | 626B | 1 | 0 | 4 | 잎 | 1.6 | ⬜ |
| 29 | `0xc7ebc0` | `side_visibility_masks_cached` | position_eval | 630B | 1 | 0 | 4 | 잎 | 1.59 | ⬜ |
| 30 | `0xd75a30` | `?` | around | 636B | 1 | 0 | 1 | 잎 | 1.57 | ⬜ |
| 31 | `0xd79da0` | `?` | around | 669B | 1 | 0 | 2 | 잎 | 1.49 | ⬜ |
| 32 | `0xc7f370` | `cached_units_in_range_of` | position_eval | 670B | 1 | 0 | 4 | 잎 | 1.49 | ⬜ |
| 33 | `0xc990e0` | `v46_carry_over` | passive_line | 685B | 1 | 0 | 1 | 잎 | 1.46 | ⬜ |
| 34 | `0xda45e0` | `?` | path_field | 1406B | 2 | 0 | 6 | 잎 | 1.42 | ⬜ |
| 35 | `0xc98dc0` | `has_lead` | passive_line | 718B | 1 | 0 | 1 | 잎 | 1.39 | ⬜ |
| 36 | `0xca6090` | `?` | score_parameter | 746B | 1 | 0 | 1 | 잎 | 1.34 | ⬜ |
| 37 | `0xdb3a30` | `?` | path_field | 1849B | 2 | 0 | 6 | 잎 | 1.08 | ⬜ |
| 38 | `0xda1c10` | `?` | path_field | 996B | 1 | 0 | 6 | 잎 | 1.0 | ⬜ |
| 39 | `0xd1e1e0` | `?` | path_finder | 2052B | 2 | 0 | 4 | 잎 | 0.97 | ⬜ |
| 40 | `0xd1ea70` | `new_filtered` | path_finder | 2095B | 2 | 0 | 3 | 잎 | 0.95 | ⬜ |
| 41 | `0xe34150` | `?` | score_parameter | 1409B | 1 | 0 | 2 | 잎 | 0.71 | ⬜ |
| 42 | `0xc72c60` | `?` | path_finder | 2874B | 2 | 0 | 4 | 잎 | 0.7 | ⬜ |
| 43 | `0xd20280` | `collect_unseen_enemy_estimates` | path_finder | 1454B | 1 | 0 | 3 | 잎 | 0.69 | ⬜ |
| 44 | `0xdafd10` | `?` | path_field | 1572B | 1 | 0 | 6 | 잎 | 0.64 | ⬜ |
| 45 | `0xdb6850` | `?` | free_dist | 1770B | 1 | 0 | 4 | 잎 | 0.56 | ⬜ |
| 46 | `0xe20a00` | `v3_tower_burst_feasible` | battle | 1861B | 1 | 0 | 2 | 잎 | 0.54 | ⬜ |
| 47 | `0xc82590` | `get_input_cl` | path_finder | 1933B | 1 | 0 | 4 | 잎 | 0.52 | ⬜ |
| 48 | `0xc7d7d0` | `end_check` | lib | 2008B | 1 | 0 | 1 | 잎 | 0.5 | ⬜ |
| 49 | `0xc86a70` | `?` | path_finder | 2049B | 1 | 0 | 4 | 잎 | 0.49 | ⬜ |
| 50 | `0xc7ad50` | `?` | path_finder | 4305B | 2 | 0 | 4 | 잎 | 0.46 | ⬜ |
| 51 | `0xc86050` | `get_input_cl` | path_finder | 2468B | 1 | 0 | 4 | 잎 | 0.41 | ⬜ |
| 52 | `0xc7cb20` | `?` | path_finder | 3178B | 1 | 0 | 4 | 잎 | 0.31 | ⬜ |
| 53 | `0xc7be70` | `?` | path_finder | 3179B | 1 | 0 | 5 | 잎 | 0.31 | ⬜ |
| 54 | `0xd214e0` | `?` | path_finder | 548B | 19 | 2 | 4 | 중간 | 34.67 | ⬜ |
| 55 | `0xdc8090` | `?` | free_dist | 938B | 27 | 1 | 3 | 중간 | 28.78 | ⬜ |
| 56 | `0xd1d9c0` | `new_tower_avoid_v3` | path_finder | 658B | 9 | 3 | 3 | 중간 | 13.68 | ⬜ |
| 57 | `0xc87fe0` | `can1v1win` | utils | 724B | 8 | 1 | 2 | 중간 | 11.05 | ⬜ |
| 58 | `0xd1f880` | `new` | path_finder | 529B | 5 | 1 | 3 | 중간 | 9.45 | ⬜ |
| 59 | `0xd1b7e0` | `?` | path_finder | 355B | 3 | 2 | 4 | 중간 | 8.45 | ⬜ |
| 60 | `0xc8bde0` | `get_input_cl` | path_finder | 764B | 5 | 2 | 5 | 중간 | 6.54 | ⬜ |
| 61 | `0xc8b450` | `get_input_cl` | path_finder | 697B | 4 | 3 | 5 | 중간 | 5.74 | ⬜ |
| 62 | `0xdb63c0` | `?` | path_finder | 543B | 3 | 2 | 5 | 중간 | 5.52 | ⬜ |
| 63 | `0xc7f0f0` | `cached_damage_against` | position_eval | 591B | 3 | 1 | 4 | 중간 | 5.08 | ⬜ |
| 64 | `0xdb4d70` | `get_input_cl` | path_finder | 854B | 4 | 3 | 5 | 중간 | 4.68 | ⬜ |
| 65 | `0xdb5a10` | `?` | path_finder | 911B | 4 | 2 | 5 | 중간 | 4.39 | ⬜ |
| 66 | `0xd1ac60` | `get_input` | path_finder | 764B | 3 | 2 | 4 | 중간 | 3.93 | ⬜ |
| 67 | `0xca5ef0` | `?` | score_parameter | 318B | 1 | 1 | 1 | 중간 | 3.14 | ⬜ |
| 68 | `0xc8b8d0` | `?` | path_finder | 1283B | 4 | 3 | 5 | 중간 | 3.12 | ⬜ |
| 69 | `0xd19eb0` | `?` | path_finder | 993B | 3 | 2 | 4 | 중간 | 3.02 | ⬜ |
| 70 | `0xc8c0e0` | `get_input` | path_finder | 1339B | 4 | 5 | 5 | 중간 | 2.99 | ⬜ |
| 71 | `0xdb5da0` | `get_input` | path_finder | 1553B | 4 | 5 | 5 | 중간 | 2.58 | ⬜ |
| 72 | `0xdb5310` | `get_input` | path_finder | 1783B | 4 | 3 | 5 | 중간 | 2.24 | ⬜ |
| 73 | `0xd1dc60` | `new_unnecessary_tower_avoid` | path_finder | 1398B | 3 | 4 | 3 | 중간 | 2.15 | ⬜ |
| 74 | `0xd1a2a0` | `?` | path_finder | 471B | 1 | 1 | 4 | 중간 | 2.12 | ⬜ |
| 75 | `0xe388c0` | `?` | serpen_check | 966B | 2 | 15 | 1 | 중간 | 2.07 | ⬜ |
| 76 | `0xd1af60` | `new` | path_finder | 487B | 1 | 1 | 4 | 중간 | 2.05 | ⬜ |
| 77 | `0xda2950` | `?` | path_field | 1046B | 2 | 1 | 6 | 중간 | 1.91 | ⬜ |
| 78 | `0xd1a630` | `?` | path_finder | 1578B | 3 | 2 | 4 | 중간 | 1.9 | ⬜ |
| 79 | `0xda0af0` | `?` | path_field | 1062B | 2 | 1 | 6 | 중간 | 1.88 | ⬜ |
| 80 | `0xda2000` | `?` | path_field | 1062B | 2 | 1 | 6 | 중간 | 1.88 | ⬜ |
| 81 | `0xda3320` | `?` | path_field | 1062B | 2 | 1 | 6 | 중간 | 1.88 | ⬜ |
| 82 | `0xd1bda0` | `get_input` | path_finder | 5986B | 11 | 1 | 3 | 중간 | 1.84 | ⬜ |
| 83 | `0xd1b150` | `?` | path_finder | 1677B | 3 | 3 | 4 | 중간 | 1.79 | ⬜ |
| 84 | `0xda2d70` | `?` | path_field | 1441B | 2 | 1 | 6 | 중간 | 1.39 | ⬜ |
| 85 | `0xdb1110` | `?` | path_field | 1547B | 2 | 1 | 6 | 중간 | 1.29 | ⬜ |
| 86 | `0xdae600` | `?` | path_field | 1568B | 2 | 1 | 6 | 중간 | 1.28 | ⬜ |
| 87 | `0xdb0380` | `?` | path_field | 1568B | 2 | 1 | 6 | 중간 | 1.28 | ⬜ |
| 88 | `0xdb1ee0` | `?` | path_field | 1568B | 2 | 1 | 6 | 중간 | 1.28 | ⬜ |
| 89 | `0xdac8c0` | `shared_free_dist` | free_dist | 1725B | 2 | 2 | 4 | 중간 | 1.16 | ⬜ |
| 90 | `0xdac1c0` | `shared_free_dist` | free_dist | 1725B | 2 | 2 | 4 | 중간 | 1.16 | ⬜ |
| 91 | `0xda7320` | `shared_free_dist` | free_dist | 1725B | 2 | 1 | 4 | 중간 | 1.16 | ⬜ |
| 92 | `0xda8820` | `?` | free_dist | 1764B | 2 | 2 | 4 | 중간 | 1.13 | ⬜ |
| 93 | `0xda4b60` | `?` | free_dist | 1820B | 2 | 2 | 4 | 중간 | 1.1 | ⬜ |
| 94 | `0xdb1760` | `21SmallActionAroundBush9g` | path_field | 1841B | 2 | 1 | 6 | 중간 | 1.09 | ⬜ |
| 95 | `0xda41c0` | `?` | path_field | 1046B | 1 | 1 | 6 | 중간 | 0.96 | ⬜ |
| 96 | `0xda3da0` | `?` | path_field | 1046B | 1 | 1 | 6 | 중간 | 0.96 | ⬜ |
| 97 | `0xc89530` | `?` | action_eval | 1307B | 1 | 1 | 2 | 중간 | 0.77 | ⬜ |
| 98 | `0xda2430` | `?` | path_field | 1304B | 1 | 1 | 6 | 중간 | 0.77 | ⬜ |
| 99 | `0xc700a0` | `?` | path_finder | 2874B | 2 | 1 | 4 | 중간 | 0.7 | ⬜ |
| 100 | `0xc737e0` | `?` | path_finder | 2874B | 2 | 1 | 4 | 중간 | 0.7 | ⬜ |
| 101 | `0xc780e0` | `?` | path_finder | 2874B | 2 | 1 | 4 | 중간 | 0.7 | ⬜ |
| 102 | `0xda1630` | `?` | path_field | 1497B | 1 | 1 | 6 | 중간 | 0.67 | ⬜ |
| 103 | `0xda0520` | `?` | path_field | 1484B | 1 | 1 | 6 | 중간 | 0.67 | ⬜ |
| 104 | `0xc74360` | `?` | path_finder | 3066B | 2 | 1 | 4 | 중간 | 0.65 | ⬜ |
| 105 | `0xdb33e0` | `?` | path_field | 1547B | 1 | 1 | 6 | 중간 | 0.65 | ⬜ |
| 106 | `0xdb2d90` | `?` | path_field | 1547B | 1 | 1 | 6 | 중간 | 0.65 | ⬜ |
| 107 | `0xda3750` | `?` | path_field | 1614B | 1 | 1 | 6 | 중간 | 0.62 | ⬜ |
| 108 | `0xc6f320` | `?` | path_finder | 3382B | 2 | 1 | 4 | 중간 | 0.59 | ⬜ |
| 109 | `0xda52c0` | `shared_free_dist` | free_dist | 1725B | 1 | 2 | 4 | 중간 | 0.58 | ⬜ |
| 110 | `0xda7a20` | `shared_free_dist` | free_dist | 1725B | 1 | 2 | 4 | 중간 | 0.58 | ⬜ |
| 111 | `0xda8f50` | `shared_free_dist` | free_dist | 1725B | 1 | 2 | 4 | 중간 | 0.58 | ⬜ |
| 112 | `0xdaabf0` | `shared_free_dist` | free_dist | 1725B | 1 | 2 | 4 | 중간 | 0.58 | ⬜ |
| 113 | `0xda6250` | `shared_free_dist` | free_dist | 1725B | 1 | 2 | 5 | 중간 | 0.58 | ⬜ |
| 114 | `0xda8120` | `shared_free_dist` | free_dist | 1725B | 1 | 2 | 5 | 중간 | 0.58 | ⬜ |
| 115 | `0xda9650` | `shared_free_dist` | free_dist | 1725B | 1 | 2 | 5 | 중간 | 0.58 | ⬜ |
| 116 | `0xdabac0` | `shared_free_dist` | free_dist | 1725B | 1 | 2 | 5 | 중간 | 0.58 | ⬜ |
| 117 | `0xdad710` | `?` | free_dist | 1801B | 1 | 1 | 4 | 중간 | 0.56 | ⬜ |
| 118 | `0xdacfc0` | `?` | free_dist | 1801B | 1 | 1 | 5 | 중간 | 0.56 | ⬜ |
| 119 | `0xda0f20` | `get_input_cl` | path_field | 1795B | 1 | 1 | 6 | 중간 | 0.56 | ⬜ |
| 120 | `0xdb09e0` | `?` | path_field | 1770B | 1 | 1 | 6 | 중간 | 0.56 | ⬜ |
| 121 | `0xda9d50` | `?` | free_dist | 1806B | 1 | 2 | 4 | 중간 | 0.55 | ⬜ |
| 122 | `0xdaa4a0` | `?` | free_dist | 1806B | 1 | 2 | 5 | 중간 | 0.55 | ⬜ |
| 123 | `0xdade60` | `?` | path_field | 1873B | 1 | 1 | 6 | 중간 | 0.53 | ⬜ |
| 124 | `0xceae90` | `get_input` | path_finder | 3825B | 2 | 3 | 3 | 중간 | 0.52 | ⬜ |
| 125 | `0xc81d90` | `get_input_cl` | path_finder | 1933B | 1 | 1 | 4 | 중간 | 0.52 | ⬜ |
| 126 | `0xc82d90` | `get_input` | path_finder | 1933B | 1 | 1 | 4 | 중간 | 0.52 | ⬜ |
| 127 | `0xc83df0` | `?` | path_finder | 1933B | 1 | 1 | 4 | 중간 | 0.52 | ⬜ |
| 128 | `0xc84ea0` | `?` | path_finder | 1933B | 1 | 1 | 4 | 중간 | 0.52 | ⬜ |
| 129 | `0xdab2f0` | `?` | free_dist | 1926B | 1 | 2 | 4 | 중간 | 0.52 | ⬜ |
| 130 | `0xdaf530` | `get_input_cl` | path_field | 1943B | 1 | 1 | 6 | 중간 | 0.51 | ⬜ |
| 131 | `0xd74400` | `max_poke_range` | battle | 1999B | 1 | 1 | 1 | 중간 | 0.5 | ⬜ |
| 132 | `0xc83590` | `?` | path_finder | 2017B | 1 | 1 | 4 | 중간 | 0.5 | ⬜ |
| 133 | `0xc81510` | `?` | path_finder | 2061B | 1 | 1 | 4 | 중간 | 0.49 | ⬜ |
| 134 | `0xdb2540` | `18SmallActionRunAway9get` | path_field | 2049B | 1 | 1 | 6 | 중간 | 0.49 | ⬜ |
| 135 | `0xc845f0` | `?` | path_finder | 2081B | 1 | 1 | 4 | 중간 | 0.48 | ⬜ |
| 136 | `0xe23750` | `is_end` | around | 2118B | 1 | 3 | 1 | 중간 | 0.47 | ⬜ |
| 137 | `0xda59c0` | `?` | free_dist | 2128B | 1 | 2 | 4 | 중간 | 0.47 | ⬜ |
| 138 | `0xc79c30` | `?` | path_finder | 4315B | 2 | 2 | 4 | 중간 | 0.46 | ⬜ |
| 139 | `0xdaec60` | `get_input_cl` | path_field | 2190B | 1 | 1 | 6 | 중간 | 0.46 | ⬜ |
| 140 | `0xc7f640` | `score_parameter_cached` | position_eval | 2332B | 1 | 1 | 4 | 중간 | 0.43 | ⬜ |
| 141 | `0xc856a0` | `get_input` | path_finder | 2356B | 1 | 2 | 4 | 중간 | 0.42 | ⬜ |
| 142 | `0xda6950` | `?` | free_dist | 2440B | 1 | 2 | 4 | 중간 | 0.41 | ⬜ |
| 143 | `0xc74fa0` | `?` | path_finder | 2874B | 1 | 1 | 4 | 중간 | 0.35 | ⬜ |
| 144 | `0xc75b20` | `?` | path_finder | 2872B | 1 | 1 | 5 | 중간 | 0.35 | ⬜ |
| 145 | `0xc766a0` | `?` | path_finder | 3274B | 1 | 1 | 4 | 중간 | 0.31 | ⬜ |
| 146 | `0xc773b0` | `?` | path_finder | 3308B | 1 | 1 | 5 | 중간 | 0.3 | ⬜ |
| 147 | `0xd07c10` | `get_input_cl` | path_finder | 7540B | 2 | 4 | 3 | 중간 | 0.27 | ⬜ |
| 148 | `0xce0e60` | `get_input` | path_finder | 7539B | 2 | 4 | 3 | 중간 | 0.27 | ⬜ |
| 149 | `0xc71de0` | `dodge_tower_cell_with_context` | path_finder | 3638B | 1 | 1 | 4 | 중간 | 0.27 | ⬜ |
| 150 | `0xceedd0` | `?` | path_finder | 3901B | 1 | 3 | 3 | 중간 | 0.26 | ⬜ |
| 151 | `0xcefd50` | `?` | path_finder | 3801B | 1 | 3 | 3 | 중간 | 0.26 | ⬜ |
| 152 | `0xcf0c70` | `?` | path_finder | 3834B | 1 | 2 | 3 | 중간 | 0.26 | ⬜ |
| 153 | `0xce7ed0` | `get_input_cl` | path_finder | 4027B | 1 | 3 | 3 | 중간 | 0.25 | ⬜ |
| 154 | `0xce8ed0` | `get_input_cl` | path_finder | 3957B | 1 | 2 | 3 | 중간 | 0.25 | ⬜ |
| 155 | `0xce9e90` | `get_input` | path_finder | 4027B | 1 | 3 | 3 | 중간 | 0.25 | ⬜ |
| 156 | `0xcebdd0` | `get_input` | path_finder | 4027B | 1 | 3 | 3 | 중간 | 0.25 | ⬜ |
| 157 | `0xcecdd0` | `get_input` | path_finder | 4027B | 1 | 3 | 3 | 중간 | 0.25 | ⬜ |
| 158 | `0xceddd0` | `get_input_cl` | path_finder | 4027B | 1 | 3 | 3 | 중간 | 0.25 | ⬜ |
| 159 | `0xce6ec0` | `?` | path_finder | 4047B | 1 | 3 | 3 | 중간 | 0.25 | ⬜ |
| 160 | `0xc78c60` | `?` | path_finder | 3971B | 1 | 1 | 4 | 중간 | 0.25 | ⬜ |
| 161 | `0xc61cf0` | `?` | path_field | 4024B | 1 | 3 | 5 | 중간 | 0.25 | ⬜ |
| 162 | `0xc67770` | `?` | path_field | 4042B | 1 | 3 | 5 | 중간 | 0.25 | ⬜ |
| 163 | `0xc69a60` | `?` | path_field | 4042B | 1 | 3 | 6 | 중간 | 0.25 | ⬜ |
| 164 | `0xc6e210` | `21SmallActionAroundBush9` | path_field | 4198B | 1 | 2 | 5 | 중간 | 0.24 | ⬜ |
| 165 | `0xc6d100` | `21SmallActionAroundBush9g` | path_field | 4198B | 1 | 2 | 6 | 중간 | 0.24 | ⬜ |
| 166 | `0xc5cd40` | `get_input_cl` | path_field | 4331B | 1 | 3 | 5 | 중간 | 0.23 | ⬜ |
| 167 | `0xc5ded0` | `?` | path_field | 4372B | 1 | 4 | 5 | 중간 | 0.23 | ⬜ |
| 168 | `0xc654d0` | `?` | path_field | 4265B | 1 | 3 | 5 | 중간 | 0.23 | ⬜ |
| 169 | `0xc58240` | `21SmallActionAroundBush9` | path_field | 4275B | 1 | 3 | 5 | 중간 | 0.23 | ⬜ |
| 170 | `0xc66620` | `?` | path_field | 4265B | 1 | 3 | 6 | 중간 | 0.23 | ⬜ |
| 171 | `0xc70c20` | `?` | path_finder | 4479B | 1 | 1 | 4 | 중간 | 0.22 | ⬜ |
| 172 | `0xc6aad0` | `18SmallActionRunAway9ge` | path_field | 4575B | 1 | 4 | 5 | 중간 | 0.22 | ⬜ |
| 173 | `0xc687e0` | `?` | path_field | 4562B | 1 | 3 | 5 | 중간 | 0.22 | ⬜ |
| 174 | `0xc6bd50` | `?` | path_field | 4876B | 1 | 2 | 5 | 중간 | 0.21 | ⬜ |
| 175 | `0xc593a0` | `get_input_cl` | path_field | 4724B | 1 | 6 | 5 | 중간 | 0.21 | ⬜ |
| 176 | `0xc5a6c0` | `17SmallActionAround9ge_cl` | path_field | 4800B | 1 | 3 | 5 | 중간 | 0.21 | ⬜ |
| 177 | `0xc5ba20` | `17SmallActionAround9g_cl` | path_field | 4724B | 1 | 6 | 6 | 중간 | 0.21 | ⬜ |
| 178 | `0xc62d50` | `get_input_cl` | path_field | 4887B | 1 | 5 | 5 | 중간 | 0.2 | ⬜ |
| 179 | `0xc64110` | `get_input_cl` | path_field | 4887B | 1 | 5 | 6 | 중간 | 0.2 | ⬜ |
| 180 | `0xc5f090` | `?` | path_field | 5505B | 1 | 6 | 5 | 중간 | 0.18 | ⬜ |
| 181 | `0xc606c0` | `?` | path_field | 5505B | 1 | 6 | 6 | 중간 | 0.18 | ⬜ |
| 182 | `0xd00470` | `get_input` | path_finder | 6982B | 1 | 4 | 4 | 중간 | 0.14 | ⬜ |
| 183 | `0xd02130` | `get_input` | path_finder | 6982B | 1 | 4 | 4 | 중간 | 0.14 | ⬜ |
| 184 | `0xcf39a0` | `get_input` | path_finder | 6982B | 1 | 4 | 4 | 중간 | 0.14 | ⬜ |
| 185 | `0xcf5660` | `?` | path_finder | 6982B | 1 | 3 | 4 | 중간 | 0.14 | ⬜ |
| 186 | `0xcf7320` | `get_input` | path_finder | 6982B | 1 | 4 | 4 | 중간 | 0.14 | ⬜ |
| 187 | `0xcf8fe0` | `?` | path_finder | 6952B | 1 | 4 | 4 | 중간 | 0.14 | ⬜ |
| 188 | `0xcfac80` | `?` | path_finder | 6982B | 1 | 4 | 4 | 중간 | 0.14 | ⬜ |
| 189 | `0xcfe7b0` | `get_input_cl` | path_finder | 6982B | 1 | 4 | 4 | 중간 | 0.14 | ⬜ |
| 190 | `0xcf1bb0` | `get_input` | path_finder | 7286B | 1 | 4 | 4 | 중간 | 0.14 | ⬜ |
| 191 | `0xd11bc0` | `get_input` | path_finder | 7508B | 1 | 4 | 3 | 중간 | 0.13 | ⬜ |
| 192 | `0xd13b30` | `get_input_cl` | path_finder | 7540B | 1 | 4 | 3 | 중간 | 0.13 | ⬜ |
| 193 | `0xd05c60` | `get_input_cl` | path_finder | 7572B | 1 | 4 | 3 | 중간 | 0.13 | ⬜ |
| 194 | `0xcdcda0` | `get_input_cl` | path_finder | 7961B | 1 | 4 | 3 | 중간 | 0.13 | ⬜ |
| 195 | `0xcdeed0` | `get_input` | path_finder | 7540B | 1 | 3 | 3 | 중간 | 0.13 | ⬜ |
| 196 | `0xd09ba0` | `get_input` | path_finder | 7564B | 1 | 4 | 3 | 중간 | 0.13 | ⬜ |
| 197 | `0xd0bb40` | `get_input` | path_finder | 7564B | 1 | 4 | 3 | 중간 | 0.13 | ⬜ |
| 198 | `0xce2df0` | `get_input` | path_finder | 7793B | 1 | 4 | 3 | 중간 | 0.13 | ⬜ |
| 199 | `0xd0dae0` | `get_input` | path_finder | 7564B | 1 | 4 | 3 | 중간 | 0.13 | ⬜ |
| 200 | `0xd0fa80` | `get_input` | path_finder | 7973B | 1 | 4 | 3 | 중간 | 0.13 | ⬜ |
| 201 | `0xce4e80` | `?` | path_finder | 7713B | 1 | 3 | 3 | 중간 | 0.13 | ⬜ |
| 202 | `0xcdacd0` | `get_input_cl` | path_finder | 7857B | 1 | 4 | 3 | 중간 | 0.13 | ⬜ |
| 203 | `0xcfc940` | `get_input_cl` | path_finder | 7411B | 1 | 4 | 4 | 중간 | 0.13 | ⬜ |
| 204 | `0xd03df0` | `?` | path_finder | 7411B | 1 | 3 | 4 | 중간 | 0.13 | ⬜ |
| 205 | `0xe23fa0` | `get_input` | small_action | 2318B | 8 | 15 | 1 | 중간 | 3.45 | #141 ✅r13+22차 |
| 206 | `0xe4b5d0` | `v3_fall_back_to_passive` | handler | 672B | 1 | 2 | 1 | 중간 | 1.49 | #11 1 |
| 207 | `0xd405d0` | `base_attacking_minion_uncached` | defense_nexus | 774B | 3 | 0 | 1 | 잎 | 3.88 | #118 ✅r11+20차 |
| 208 | `0xdc2550` | `new_with_target` | around | 527B | 2 | 1 | 1 | 중간 | 3.8 | #140 ✅r13+22차 |
| 209 | `0xd57540` | `interaction_score` | action_score | 9212B | 14 | 7 | 1 | 거대 | 1.52 | #183 ✅r14+23차 |
| 210 | `0xd59940` | `calculate_action_score` | action_score | 8506B | 6 | 2 | 1 | 거대 | 0.71 | #186 ✅r14+23차 |
| 211 | `0xe28410` | `line_action_economy_adjustment` | lane_economy | 1192B | 3 | 3 | 1 | 중간 | 2.52 | #147 ✅r14+23차 |
| 212 | `0xe29b40` | `evaluate_action` | action_eval | 3297B | 3 | 5 | 1 | 중간 | 0.91 | #165 ✅r14+23차 |
| 213 | `0xd5ba80` | `calculate_jungle_action_score` | action_score | 366B | 1 | 1 | 1 | 중간 | 2.73 | #01 ✅ DIFF 0 |
| 214 | `0xeb8b80` | `attack_summon_action` | fight_check | 1874B | 14 | 0 | 1 | 잎 | 7.47 | #114 ✅r11+20차 |
| 215 | `0xccaa10` | `score` | epic_poke | 725B | 3 | 1 | 2 | 중간 | 4.14 | #138 ✅r13+22차 |
| 216 | `0xccf670` | `serpen_action_score` | serpen_hunt | 1278B | 3 | 2 | 2 | 중간 | 2.35 | #148 ✅r14+23차 |
| 217 | `0xe81390` | `score` | serpen_poke | 747B | 3 | 1 | 2 | 중간 | 4.02 | #139 ✅r13+22차 |
| 218 | `0xe8b200` | `score` | line_defense | 986B | 3 | 4 | 2 | 중간 | 3.04 | #143 ✅r14+23차 |
| 219 | `0xec7c90` | `epic_action_score` | epic_hunt | 1278B | 3 | 2 | 2 | 중간 | 2.35 | #149 ✅r14+23차 |
| 220 | `0xcce210` | `get_input` | trace | 4742B | 1 | 11 | 2 | 중간 | 0.21 | #176 ✅r14+23차 |
| 221 | `0xd34750` | `safe_move_avoiding_enemy_well` | abstract_input | 3440B | 8 | 0 | 2 | 잎 | 2.33 | #121 ✅r13+22차 |
| 222 | `0xd845e0` | `get_input` | cast | 532B | 1 | 3 | 2 | 중간 | 1.88 | #151 ✅r14+23차 |
| 223 | `0xd84800` | `get_input` | cast | 548B | 1 | 3 | 2 | 중간 | 1.82 | #153 ✅r14+23차 |
| 224 | `0xd84a30` | `get_input` | cast | 548B | 1 | 3 | 2 | 중간 | 1.82 | #154 ✅r14+23차 |
| 225 | `0xdb6ff0` | `clone` | small_action | 6900B | 1 | 16 | 2 | 중간 | 0.14 | #179 ✅r14+23차 |
| 226 | `0xdbbd60` | `?` | small_action | 4947B | 1 | 9 | 2 | 중간 | 0.2 | #177 ✅r14+23차 |
| 227 | `0xdbd260` | `get_input` | move_actions | 8891B | 1 | 13 | 2 | 거대 | 0.11 | #184 ✅r14+23차 |
| 228 | `0xdbf680` | `?` | small_action | 3451B | 1 | 8 | 2 | 중간 | 0.29 | #172 ✅r14+23차 |
| 229 | `0xdc0800` | `merge` | small_action | 6137B | 1 | 12 | 2 | 중간 | 0.16 | #178 ✅r14+23차 |
| 230 | `0xdc2070` | `get_input` | around | 690B | 1 | 4 | 2 | 중간 | 1.45 | #157 ✅r14+23차 |
| 231 | `0xdc2960` | `get_input` | around | 1804B | 1 | 4 | 2 | 중간 | 0.55 | #168 ✅r14+23차 |
| 232 | `0xdc3240` | `get_input` | move_actions | 14264B | 4 | 16 | 2 | 거대 | 0.28 | #182 ✅r14+23차 |
| 233 | `0xdc6ec0` | `is_end` | small_action | 3887B | 1 | 6 | 2 | 중간 | 0.26 | #173 ✅r14+23차 |
| 234 | `0xe26c40` | `get_input` | lane_minion | 1755B | 1 | 7 | 2 | 중간 | 0.57 | #167 ✅r14+23차 |
| 235 | `0xd5bbf0` | `calculate_interaction_action_score` | action_score | 19586B | 1 | 11 | 2 | 거대 | 0.05 | #181 ✅r14+23차 |
| 236 | `0xd84db0` | `position_eval_at` | position_eval | 841B | 32 | 2 | 2 | 중간 | 38.05 | #187 ✅r14+23차 |
| 237 | `0xe23170` | `evaluation_position` | small_action | 729B | 2 | 1 | 2 | 중간 | 2.74 | #145 ✅r14+23차 |
| 238 | `0xe266e0` | `has_current_explicit_minion_action` | lane_minion | 692B | 2 | 1 | 2 | 중간 | 2.89 | #144 ✅r14+23차 |
| 239 | `0xe28060` | `line_recall_pressure_penalty` | lane_economy | 398B | 1 | 0 | 2 | 잎 | 2.51 | #120 ✅r12+21차 |
| 240 | `0xe281f0` | `line_champion_trade_allowance` | lane_economy | 535B | 1 | 2 | 2 | 중간 | 1.87 | #152 ✅r14+23차 |
| 241 | `0xe288c0` | `line_projected_punish_damage_at` | lane_economy | 3167B | 1 | 2 | 2 | 중간 | 0.32 | #171 ✅r14+23차 |
| 242 | `0xccfbc0` | `calculate_serpen_action_score` | serpen_hunt | 2230B | 1 | 1 | 3 | 중간 | 0.45 | #169 ✅r14+23차 |
| 243 | `0xec81e0` | `calculate_epic_action_score` | epic_hunt | 2313B | 1 | 1 | 3 | 중간 | 0.43 | #170 ✅r14+23차 |
| 244 | `0xd97700` | `v3_deadly_edge_cells` | tower_discipline | 533B | 5 | 0 | 3 | 잎 | 9.38 | #113 ✅r11+20차 |
| 245 | `0xe29520` | `path_needs_tower_escape` | small_action | 922B | 3 | 1 | 3 | 중간 | 3.25 | #142 ✅r13+22차 |
| 246 | `0xd35930` | `skill` | abstract_input | 978B | 1 | 2 | 3 | 중간 | 1.02 | #163 ✅r14+23차 |
| 247 | `0xd9f3c0` | `check_nontarget` | cast | 804B | 3 | 0 | 3 | 잎 | 3.73 | #119 ✅r11+20차 |
| 248 | `0xd360a0` | `skill2` | abstract_input | 981B | 1 | 2 | 3 | 중간 | 1.02 | #164 ✅r14+23차 |
| 249 | `0xd354c0` | `ult` | abstract_input | 1135B | 1 | 2 | 3 | 중간 | 0.88 | #00 ✅ DIFF 0 |
| 250 | `0xd98420` | `v3_lethal_tower_hp` | tower_discipline | 475B | 4 | 1 | 3 | 중간 | 8.42 | #135 ✅r13+22차 |
| 251 | `0xd9ddc0` | `is_safe_recall` | cast | 5533B | 8 | 0 | 3 | 잎 | 1.45 | #127 ✅r13+22차 |
| 252 | `0xd390a0` | `champion_hp_value_uncached` | utils | 2304B | 1 | 0 | 3 | 잎 | 0.43 | #132 ✅r13+22차 |
| 253 | `0xdffa10` | `buff_value_v54` | buff_value | 6720B | 1 | 0 | 3 | 잎 | 0.15 | #133 ✅r13+22차 |
| 254 | `0xe01450` | `v55_mark_value` | buff_value | 1394B | 1 | 0 | 3 | 잎 | 0.72 | #130 ✅r13+22차 |
| 255 | `0xe019d0` | `v55_seal_value` | buff_value | 614B | 1 | 0 | 3 | 잎 | 1.63 | #124 ✅r13+22차 |
| 256 | `0xe01c40` | `defensive_crisis` | buff_value | 897B | 1 | 0 | 3 | 잎 | 1.11 | #03 ✅ DIFF 0 |
| 257 | `0xe02020` | `v55_banish_penalty` | buff_value | 686B | 1 | 0 | 3 | 잎 | 1.46 | #126 ✅r13+22차 |
| 258 | `0xe02540` | `noncombat_steroid_value` | buff_value | 1658B | 1 | 0 | 3 | 잎 | 0.6 | #131 ✅r13+22차 |
| 259 | `0xe04400` | `v57_summon_command_score` | buff_value | 960B | 1 | 1 | 3 | 중간 | 1.04 | #162 ✅r14+23차 |
| 260 | `0xd851d0` | `position_eval_at_uncached` | position_eval | 30055B | 1 | 7 | 3 | 거대 | 0.03 | #180 ✅r14+23차 |
| 261 | `0xcaff00` | `expected_goal_position` | trace | 1029B | 1 | 0 | 3 | 잎 | 0.97 | #128 ✅r13+22차 |
| 262 | `0xd34470` | `convert_to_move_action_target` | abstract_input | 450B | 1 | 0 | 4 | 잎 | 2.22 | #122 ✅r13+22차 |
| 263 | `0xd99030` | `v22_lane_tower_pressure_attack_allowed` | tower_discipline | 1942B | 4 | 2 | 4 | 중간 | 2.06 | #150 ✅r14+23차 |
| 264 | `0xd31f20` | `get_input_target` | abstract_input | 8863B | 4 | 1 | 4 | 거대 | 0.45 | #185 ✅r14+23차 |
| 265 | `0xdc8550` | `?` | around | 1126B | 17 | 1 | 4 | 중간 | 15.1 | #134 ✅r13+22차 |
| 266 | `0xeade00` | `score` | death_battle | 3913B | 1 | 9 | 2 | 중간 | 0.26 | 데스매치 전용(MOBA 미사용 · #17 과 같은 판정) |

## game_core/engine 경계(지도 밖 · 서브트리가 부르는 것 · 대략 대역 0x1200000~0x1900000)

`0x1285320` `0x12854c0` `0x128cf70` `0x128ef00` `0x129d130` `0x129d800` `0x129ed50` `0x129feb0` `0x12a0180` `0x12a04b0` `0x12a06a0` `0x12a07d0` `0x12a0c80` `0x1323a00` `0x132aa00` `0x132ad30` `0x132b310` `0x132c200` `0x132d450` `0x1453260` `0x14535b0` `0x16047b0` `0x16fb370` `0x16fc160` `0x18035f0` `0x18096a0` `0x1820ee0` `0x1821120` `0x183ed40`

이들은 명세 대상이 아니라 **계약(시그니처·반환 의미)만** 확정한다(tcx `--deep`/오라클). 예: `estimate_damage_to`(0x12857f0 · 호출자 91).