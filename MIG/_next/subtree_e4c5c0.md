# `handler::update`(0xe4c5c0) 호출 서브트리 선별표 — 작업 순서 = 아래에서 위로 (생성 2026-09-13 · 게임 0.5.8)

> 생성물(`MIG\subtree_rank.py 0xe4c5c0`) · 손편집 금지. 점수 = 호출자수/크기×1000(재사용↑·크기↓ 우선). 종류: 잎=지도 내 콜리 0 · 중간=<8KB · 거대=≥8KB. **⬜도달 가능성(사장 서브트리) 봉인은 미실시** — 착수 시 `irann.py`(교훈 68)로 NA 를 먼저 찍고 이 표에서 뺄 것. 이름 `?` = 지도 미명명(dllmatch/percolate 로 이름부터).
> 합계: 서브트리 162 · 잎 88 · 중간 59 · 거대 10 · 이미 명세/ev1 16 · 제외(데스매치) 4 · game_core 경계 함수 24(명세 대상 아님 · 계약만)

| 순위 | RVA | 함수 | 모듈 | 크기 | 호출자 | 콜리 | 깊이 | 종류 | 점수 | 상태/제외 |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `0xdd50e0` | `v27_active_objective_discipline` | objective_discipline | 396B | 5 | 0 | 2 | 잎 | 12.63 | ⬜ |
| 2 | `0xe7a8c0` | `upgrade_item` | lib | 932B | 11 | 0 | 3 | 잎 | 11.8 | ⬜ |
| 3 | `0xd97300` | `can_tower_focused` | tower_discipline | 1019B | 9 | 0 | 3 | 잎 | 8.83 | ⬜ |
| 4 | `0xe7b640` | `buy_item` | lib | 856B | 7 | 0 | 3 | 잎 | 8.18 | ⬜ |
| 5 | `0xec9840` | `v23_healthy_allies_near_point` | objective_helpers | 930B | 7 | 0 | 2 | 잎 | 7.53 | ⬜ |
| 6 | `0xe0c310` | `v22_visible_enemy_is_runaway_threat` | fight_model | 468B | 3 | 0 | 2 | 잎 | 6.41 | ⬜ |
| 7 | `0xeca9a0` | `v23_objective_setup_pressure_line` | objective_helpers | 787B | 5 | 0 | 3 | 잎 | 6.35 | ⬜ |
| 8 | `0xd3fa80` | `nexus_under_direct_attack` | defense_nexus | 796B | 5 | 0 | 2 | 잎 | 6.28 | ⬜ |
| 9 | `0xe0bd60` | `v21_should_defer_support_target` | fight_model | 516B | 3 | 0 | 3 | 잎 | 5.81 | ⬜ |
| 10 | `0xecacc0` | `v23_recent_visible_enemies_near_point` | objective_helpers | 1586B | 8 | 0 | 3 | 잎 | 5.04 | ⬜ |
| 11 | `0xec8af0` | `objective_is_damaged` | objective_helpers | 175B | 1 | 0 | 2 | 잎 | 5.0 | ⬜ |
| 12 | `0xd472b0` | `?` | serpen | 139B | 1 | 0 | 3 | 잎 | 5.0 | ⬜ |
| 13 | `0xdeb970` | `?` | team_plan | 126B | 1 | 0 | 3 | 잎 | 5.0 | ⬜ |
| 14 | `0xdeb8f0` | `?` | objective_discipline | 126B | 1 | 0 | 3 | 잎 | 5.0 | ⬜ |
| 15 | `0xdea800` | `v3_epic_formation_role` | epic | 610B | 3 | 0 | 2 | 잎 | 4.92 | ⬜ |
| 16 | `0xd665e0` | `serpen_giveup_chat_reason` | serpen | 212B | 1 | 0 | 2 | 잎 | 4.72 | ⬜ |
| 17 | `0xe4aec0` | `v2_obj_restore_safe` | handler | 432B | 2 | 0 | 2 | 잎 | 4.63 | ⬜ |
| 18 | `0xe705a0` | `update_on_dead` | handler | 242B | 1 | 0 | 2 | 잎 | 4.13 | ⬜ |
| 19 | `0xc9bea0` | `?` | goal_data | 259B | 1 | 0 | 3 | 잎 | 3.86 | ⬜ |
| 20 | `0xc9c160` | `?` | goal_data | 259B | 1 | 0 | 3 | 잎 | 3.86 | ⬜ |
| 21 | `0xc9c000` | `?` | goal_data | 264B | 1 | 0 | 3 | 잎 | 3.79 | ⬜ |
| 22 | `0xc9c2c0` | `?` | goal_data | 264B | 1 | 0 | 3 | 잎 | 3.79 | ⬜ |
| 23 | `0xe3b570` | `band_pred` | objective_discipline | 273B | 1 | 0 | 4 | 잎 | 3.66 | ⬜ |
| 24 | `0xca3530` | `?` | serpen | 295B | 1 | 0 | 5 | 잎 | 3.39 | ⬜ |
| 25 | `0xe33890` | `?` | steal | 305B | 1 | 0 | 5 | 잎 | 3.28 | ⬜ |
| 26 | `0xdeb9f0` | `?` | team_plan | 327B | 1 | 0 | 3 | 잎 | 3.06 | ⬜ |
| 27 | `0xd3e4b0` | `has_line_defense_threat` | defense_nexus | 337B | 1 | 0 | 4 | 잎 | 2.97 | ⬜ |
| 28 | `0xdb8ba0` | `target_bush` | ganker | 698B | 2 | 0 | 2 | 잎 | 2.87 | ⬜ |
| 29 | `0xe346e0` | `?` | passive_jungle | 362B | 1 | 0 | 3 | 잎 | 2.76 | ⬜ |
| 30 | `0xe126b0` | `resolve_fight_stake` | fight_model | 369B | 1 | 0 | 2 | 잎 | 2.71 | ⬜ |
| 31 | `0xca2790` | `?` | handler | 378B | 1 | 0 | 2 | 잎 | 2.65 | ⬜ |
| 32 | `0xe70330` | `handle_chat_inner::filter().count() 인스턴스` | chat | 379B | 1 | 0 | 3 | 잎 | 2.64 | ⬜ |
| 33 | `0xe6d000` | `calculate_nexus_defense_count` | handler | 1152B | 3 | 0 | 2 | 잎 | 2.6 | ⬜ |
| 34 | `0xe32140` | `v3_epic_formation` | hunt_and_poke | 404B | 1 | 0 | 3 | 잎 | 2.48 | ⬜ |
| 35 | `0xe332e0` | `v3_epic_formation` | epic | 404B | 1 | 0 | 3 | 잎 | 2.48 | ⬜ |
| 36 | `0xe31fa0` | `v3_epic_formation` | hunt_and_poke | 404B | 1 | 0 | 4 | 잎 | 2.48 | ⬜ |
| 37 | `0xca31c0` | `?` | passive_jungle | 420B | 1 | 0 | 2 | 잎 | 2.38 | ⬜ |
| 38 | `0xd41600` | `can_recall` | utils | 881B | 2 | 0 | 2 | 잎 | 2.27 | ⬜ |
| 39 | `0xe3b020` | `?` | engage | 449B | 1 | 0 | 3 | 잎 | 2.23 | ⬜ |
| 40 | `0xd3d560` | `i_am_chosen_defender` | defense_nexus | 1873B | 4 | 0 | 2 | 잎 | 2.14 | ⬜ |
| 41 | `0xdec010` | `?` | team_plan | 473B | 1 | 0 | 3 | 잎 | 2.11 | ⬜ |
| 42 | `0xd988d0` | `can_tower_focused_when_battle` | tower_discipline | 490B | 1 | 0 | 2 | 잎 | 2.04 | ⬜ |
| 43 | `0xdeb6f0` | `?` | objective_discipline | 509B | 1 | 0 | 3 | 잎 | 1.96 | ⬜ |
| 44 | `0xe2f230` | `is_wave_priority_start_line` | objective_helpers | 531B | 1 | 0 | 3 | 잎 | 1.88 | ⬜ |
| 45 | `0xc9b990` | `?` | goal_data | 548B | 1 | 0 | 3 | 잎 | 1.82 | ⬜ |
| 46 | `0xca4a80` | `Vec<&Entity>::retain 모노모프(single_handle_solokill)` | modes | 571B | 1 | 0 | 1 | 잎 | 1.75 | ⬜ |
| 47 | `0xc9d9d0` | `update_v32` | single_battle | 570B | 1 | 0 | 3 | 잎 | 1.75 | ⬜ |
| 48 | `0xc9bc10` | `?` | goal_data | 574B | 1 | 0 | 3 | 잎 | 1.74 | ⬜ |
| 49 | `0xe4a780` | `v3_assign_anchor` | handler | 583B | 1 | 0 | 2 | 잎 | 1.72 | ⬜ |
| 50 | `0xe04f50` | `fight_participants` | fight_model | 1170B | 2 | 0 | 3 | 잎 | 1.71 | ⬜ |
| 51 | `0xe32df0` | `resolve_fight_stake_roster` | fight_model | 585B | 1 | 0 | 4 | 잎 | 1.71 | ⬜ |
| 52 | `0xe305f0` | `with_runaway` | single_battle | 588B | 1 | 0 | 3 | 잎 | 1.7 | ⬜ |
| 53 | `0xdcc100` | `enemy_could_arrive` | objective_discipline | 631B | 1 | 0 | 4 | 잎 | 1.58 | ⬜ |
| 54 | `0xe33040` | `with_runaway` | battle | 670B | 1 | 0 | 2 | 잎 | 1.49 | ⬜ |
| 55 | `0xe316b0` | `handle_interact_battle_cl` | engage | 680B | 1 | 0 | 3 | 잎 | 1.47 | ⬜ |
| 56 | `0xe2e560` | `can_recall` | utils | 1378B | 2 | 0 | 2 | 잎 | 1.45 | ⬜ |
| 57 | `0xeca430` | `v25_objective_far_split_pressure` | objective_helpers | 1378B | 2 | 0 | 3 | 잎 | 1.45 | ⬜ |
| 58 | `0xca5310` | `?` | position_eval | 716B | 1 | 0 | 3 | 잎 | 1.4 | ⬜ |
| 59 | `0xe2fa70` | `objective_min_by` | objective_discipline | 733B | 1 | 0 | 4 | 잎 | 1.36 | ⬜ |
| 60 | `0xe70c70` | `take_misunderstood_received_chat` | handler | 809B | 1 | 0 | 1 | 잎 | 1.24 | ⬜ |
| 61 | `0xd36480` | `can_recall` | utils | 816B | 1 | 0 | 3 | 잎 | 1.23 | ⬜ |
| 62 | `0xe32920` | `base_battle_action` | battle | 825B | 1 | 0 | 3 | 잎 | 1.21 | ⬜ |
| 63 | `0xca4cc0` | `?` | engage | 845B | 1 | 0 | 2 | 잎 | 1.18 | ⬜ |
| 64 | `0xc9eb60` | `collect_allies_near` | fight_model | 883B | 1 | 0 | 3 | 잎 | 1.13 | ⬜ |
| 65 | `0xca85d0` | `?` | goal_data | 890B | 1 | 0 | 3 | 잎 | 1.12 | ⬜ |
| 66 | `0xd3c700` | `need_defense_nexus` | defense_nexus | 2001B | 2 | 0 | 2 | 잎 | 1.0 | ⬜ |
| 67 | `0xca8ae0` | `?` | goal_data | 1071B | 1 | 0 | 3 | 잎 | 0.93 | ⬜ |
| 68 | `0xc9e660` | `?` | passive_jungle | 1193B | 1 | 0 | 2 | 잎 | 0.84 | ⬜ |
| 69 | `0xdce520` | `handle_epic_line_change` | epic | 1284B | 1 | 0 | 2 | 잎 | 0.78 | ⬜ |
| 70 | `0xc999e0` | `sub_plan` | battle | 1452B | 1 | 0 | 3 | 잎 | 0.69 | ⬜ |
| 71 | `0xc9b390` | `?` | single_battle | 1452B | 1 | 0 | 3 | 잎 | 0.69 | ⬜ |
| 72 | `0xc9a790` | `v25_scoped_battle_objective` | fight_model | 1452B | 1 | 0 | 3 | 잎 | 0.69 | ⬜ |
| 73 | `0xe05e70` | `resolve_join_stake` | fight_model | 3571B | 2 | 0 | 2 | 잎 | 0.56 | ⬜ |
| 74 | `0xde81b0` | `check_epic_giveup` | epic | 4216B | 2 | 0 | 2 | 잎 | 0.47 | ⬜ |
| 75 | `0xdcd930` | `handle_press_epic` | epic | 2288B | 1 | 0 | 3 | 잎 | 0.44 | ⬜ |
| 76 | `0xde40c0` | `check_press_tower_opportunity` | team_plan | 4664B | 2 | 0 | 2 | 잎 | 0.43 | ⬜ |
| 77 | `0xde0770` | `update` | team_plan | 5026B | 2 | 0 | 1 | 잎 | 0.4 | ⬜ |
| 78 | `0xde6ce0` | `check_epic_setup` | epic | 5192B | 2 | 0 | 2 | 잎 | 0.39 | ⬜ |
| 79 | `0xd815e0` | `new` | score_parameter | 7241B | 2 | 0 | 3 | 잎 | 0.28 | ⬜ |
| 80 | `0xd781e0` | `sub_plan` | single_line | 5111B | 1 | 0 | 2 | 잎 | 0.2 | ⬜ |
| 81 | `0xd2c5d0` | `sub_plan` | passive_line | 5182B | 1 | 0 | 2 | 잎 | 0.19 | ⬜ |
| 82 | `0xe0bf70` | `is_unreasonable_tower_dive_enemy` | fight_model | 915B | 12 | 3 | 1 | 중간 | 13.11 | ⬜ |
| 83 | `0xd98210` | `can_trace_without_tower` | tower_discipline | 519B | 3 | 1 | 2 | 중간 | 5.78 | ⬜ |
| 84 | `0xd9aa80` | `bush_distance_sq` | steal | 395B | 2 | 1 | 4 | 중간 | 5.06 | ⬜ |
| 85 | `0xec9190` | `v23_enemy_object_pressure` | objective_helpers | 621B | 3 | 2 | 3 | 중간 | 4.83 | ⬜ |
| 86 | `0xde03d0` | `sanitize_rule_scope` | team_plan | 917B | 4 | 1 | 2 | 중간 | 4.36 | ⬜ |
| 87 | `0xeca200` | `v25_objective_splitter_can_stay` | objective_helpers | 559B | 2 | 3 | 3 | 중간 | 3.58 | ⬜ |
| 88 | `0xdeeda0` | `?` | hunt_and_poke | 386B | 1 | 1 | 3 | 중간 | 2.59 | ⬜ |
| 89 | `0xde3d90` | `check_epic_kill_time_with_hp` | goal_data | 802B | 2 | 1 | 3 | 중간 | 2.49 | ⬜ |
| 90 | `0xcaf0d0` | `?` | active_recall | 522B | 1 | 2 | 1 | 중간 | 1.92 | ⬜ |
| 91 | `0xcaf9f0` | `BigPlan::sub_plan (host)` | types | 1049B | 2 | 10 | 1 | 중간 | 1.91 | ⬜ |
| 92 | `0xd40f10` | `evaluate_gank_opportunity_with_score` | passive_jungle | 1615B | 3 | 2 | 1 | 중간 | 1.86 | ⬜ |
| 93 | `0xccc3c0` | `hunt_and_battle` | hunt_and_battle | 591B | 1 | 1 | 2 | 중간 | 1.69 | ⬜ |
| 94 | `0xdd6b40` | `v24_objective_setup_lane_pressure_ready` | objective_discipline | 1803B | 3 | 5 | 2 | 중간 | 1.66 | ⬜ |
| 95 | `0xdff080` | `base_sub_goal` | battle | 2440B | 4 | 1 | 1 | 중간 | 1.64 | ⬜ |
| 96 | `0xe5d300` | `try_engage_dive` | engage | 654B | 1 | 1 | 2 | 중간 | 1.53 | ⬜ |
| 97 | `0xe07430` | `tower_dive_is_viable` | fight_model | 2848B | 4 | 2 | 2 | 중간 | 1.4 | ⬜ |
| 98 | `0xd60920` | `single_tower_dive_is_viable` | single_battle | 1684B | 2 | 1 | 2 | 중간 | 1.19 | ⬜ |
| 99 | `0xdee750` | `fight_kit_range_cached` | battle | 947B | 1 | 1 | 3 | 중간 | 1.06 | ⬜ |
| 100 | `0xe7acd0` | `should_recall_to_shop` | lib | 989B | 1 | 3 | 2 | 중간 | 1.01 | ⬜ |
| 101 | `0xdf0a90` | `is_end` | hunt_and_poke | 1016B | 1 | 1 | 2 | 중간 | 0.98 | ⬜ |
| 102 | `0xec9de0` | `wave_priority_clearer_position` | objective_helpers | 1045B | 1 | 1 | 2 | 중간 | 0.96 | ⬜ |
| 103 | `0xd3b2a0` | `v46_flee_gate_check` | passive_line | 4673B | 4 | 1 | 1 | 중간 | 0.86 | ⬜ |
| 104 | `0xdfe2a0` | `build` | fight_model | 3474B | 3 | 1 | 3 | 중간 | 0.86 | ⬜ |
| 105 | `0xe38c90` | `can_battle_triggered_filtered` | engage | 1181B | 1 | 1 | 2 | 중간 | 0.85 | ⬜ |
| 106 | `0xe4b070` | `v2_apply_assign_commit` | handler | 1371B | 1 | 1 | 1 | 중간 | 0.73 | ⬜ |
| 107 | `0xebbb80` | `should_disengage_object_hunt` | fight_check | 1407B | 1 | 1 | 2 | 중간 | 0.71 | ⬜ |
| 108 | `0xdccc60` | `?` | goal_data | 2888B | 2 | 3 | 1 | 중간 | 0.69 | ⬜ |
| 109 | `0xec8ba0` | `v3_epicops_defer_serpen` | objective_helpers | 1507B | 1 | 1 | 3 | 중간 | 0.66 | ⬜ |
| 110 | `0xdd5db0` | `v24_objective_setup_should_check_camp` | objective_discipline | 3467B | 2 | 5 | 3 | 중간 | 0.58 | ⬜ |
| 111 | `0xd62bb0` | `check_serpen_setup` | serpen | 3646B | 2 | 1 | 2 | 중간 | 0.55 | ⬜ |
| 112 | `0xd3dcc0` | `objective_defense_role` | defense_nexus | 1854B | 1 | 3 | 2 | 중간 | 0.54 | ⬜ |
| 113 | `0xe6b800` | `check_kill` | handler | 5680B | 3 | 2 | 1 | 중간 | 0.53 | ⬜ |
| 114 | `0xe2ead0` | `is_dash_worth` | utils | 1877B | 1 | 2 | 2 | 중간 | 0.53 | ⬜ |
| 115 | `0xd3a3a0` | `line_backfight_support_focus` | utils | 1951B | 1 | 3 | 1 | 중간 | 0.51 | ⬜ |
| 116 | `0xd2da10` | `sub_plan` | defense_nexus | 2104B | 1 | 1 | 2 | 중간 | 0.48 | ⬜ |
| 117 | `0xe5ca10` | `try_engage` | engage | 2148B | 1 | 2 | 2 | 중간 | 0.47 | ⬜ |
| 118 | `0xdd90c0` | `update_steal` | team_plan | 2601B | 1 | 1 | 1 | 중간 | 0.38 | ⬜ |
| 119 | `0xeb9570` | `battle_check_with_list` | fight_check | 2890B | 1 | 1 | 3 | 중간 | 0.35 | ⬜ |
| 120 | `0xd2e500` | `sub_plan` | passive_jungle | 2993B | 1 | 1 | 2 | 중간 | 0.33 | ⬜ |
| 121 | `0xdefcd0` | `sub_plan` | hunt_and_poke | 3413B | 1 | 4 | 2 | 중간 | 0.29 | ⬜ |
| 122 | `0xdf0e90` | `sub_plan` | hunt_and_poke | 3471B | 1 | 4 | 2 | 중간 | 0.29 | ⬜ |
| 123 | `0xd639f0` | `check_serpen_giveup` | serpen | 7210B | 2 | 2 | 2 | 중간 | 0.28 | ⬜ |
| 124 | `0xd65620` | `serpen_passive_plan` | serpen | 3528B | 1 | 2 | 2 | 중간 | 0.28 | ⬜ |
| 125 | `0xd9ac10` | `should_steal_now` | steal | 4296B | 1 | 1 | 2 | 중간 | 0.23 | ⬜ |
| 126 | `0xde92d0` | `epic_passive_plan` | epic | 4417B | 1 | 3 | 2 | 중간 | 0.23 | ⬜ |
| 127 | `0xde1ee0` | `?` | goal_data | 4642B | 1 | 5 | 2 | 중간 | 0.22 | ⬜ |
| 128 | `0xd61330` | `check_serpen_hunt` | serpen | 6263B | 1 | 1 | 2 | 중간 | 0.16 | ⬜ |
| 129 | `0xd9bce0` | `evaluate_steal_for_target` | steal | 6113B | 1 | 2 | 3 | 중간 | 0.16 | ⬜ |
| 130 | `0xdd73b0` | `update` | goal_data | 6536B | 1 | 7 | 2 | 중간 | 0.15 | ⬜ |
| 131 | `0xde5340` | `check_epic_hunt` | epic | 6556B | 1 | 1 | 2 | 중간 | 0.15 | ⬜ |
| 132 | `0xdfb840` | `update` | battle | 9320B | 8 | 6 | 1 | 거대 | 0.86 | ⬜ |
| 133 | `0xd8ca70` | `position_risk_all_zero_near` | position_eval | 9406B | 3 | 3 | 2 | 거대 | 0.32 | ⬜ |
| 134 | `0xe59b20` | `handle_chat_inner` | chat | 9416B | 1 | 9 | 2 | 거대 | 0.11 | ⬜ |
| 135 | `0xdd26e0` | `v25_objective_posture` | objective_discipline | 10749B | 5 | 1 | 2 | 거대 | 0.47 | ⬜ |
| 136 | `0xe46bc0` | `passive_plan` | handler | 11615B | 4 | 8 | 1 | 거대 | 0.34 | ⬜ |
| 137 | `0xdcee40` | `fmt` | team_plan | 14041B | 1 | 19 | 2 | 거대 | 0.07 | ⬜ |
| 138 | `0xd52030` | `update` | single_battle | 20218B | 2 | 10 | 2 | 거대 | 0.1 | ⬜ |
| 139 | `0xdda220` | `v24_objective_setup_should_release_to_passive` | objective_discipline | 24674B | 1 | 21 | 1 | 거대 | 0.04 | ⬜ |
| 140 | `0xdf36e0` | `update_v32` | battle | 29389B | 1 | 8 | 2 | 거대 | 0.03 | ⬜ |
| 141 | `0xe5d5d0` | `handle_interact_battle` | engage | 32177B | 1 | 13 | 1 | 거대 | 0.03 | ⬜ |
| 142 | `0xe59190` | `v50_fold_dive_episode` | handler | 889B | 1 | 0 | 1 | 잎 | 1.12 | #05 ✅ DIFF 0 |
| 143 | `0xe595b0` | `handle_chat` | chat | 905B | 2 | 1 | 1 | 중간 | 2.21 | #12 ✅ DIFF 0 |
| 144 | `0xe5c1f0` | `single_try_engage` | modes | 1791B | 1 | 3 | 1 | 중간 | 0.56 | #15 ⬜ 미발화 |
| 145 | `0xebd570` | `check_favorable_engage_formation` | fight_check | 3242B | 3 | 0 | 1 | 잎 | 0.93 | #09 ✅ DIFF 0 |
| 146 | `0xdefa20` | `is_end` | hunt_and_poke | 675B | 1 | 1 | 2 | 중간 | 1.48 | #08 ✅ DIFF 0 |
| 147 | `0xccc010` | `sub_plan` | hunt_and_battle | 591B | 1 | 1 | 2 | 중간 | 1.69 | #07 ⬜ 미발화 |
| 148 | `0xdf1c80` | `target_bush_v30` | line_gank/cover | 713B | 2 | 0 | 2 | 잎 | 2.81 | #13 ✅ DIFF 0 |
| 149 | `0xdeaa70` | `v3_epicops_repair_need` | epic | 1230B | 2 | 0 | 2 | 잎 | 1.63 | #22 DIFF 0(명세 밖·회계 보류) |
| 150 | `0xe0c560` | `should_end_object_finish_kill_priority_battle` | fight_model | 851B | 3 | 0 | 2 | 잎 | 3.53 | #10 ⬜ 미발화 |
| 151 | `0xd40b20` | `best_jungle_goal` | passive_jungle | 898B | 2 | 1 | 2 | 중간 | 2.23 | #19 ✅ DIFF 0 |
| 152 | `0xe657a0` | `v2_response_retreat_stance` | engage | 877B | 1 | 2 | 2 | 중간 | 1.14 | #06 ✅ DIFF 0 |
| 153 | `0xd3cfa0` | `handle_line_defense` | defense_nexus | 1295B | 2 | 1 | 3 | 중간 | 1.54 | #04 ✅ DIFF 0 |
| 154 | `0xdce220` | `v3_epicops_buff_window` | epic | 764B | 1 | 5 | 3 | 중간 | 1.31 | #18 ✅ DIFF 0 |
| 155 | `0xec9bf0` | `is_object_being_taken_by_enemy` | objective_helpers | 364B | 4 | 0 | 3 | 잎 | 10.99 | #23 DIFF 0(명세 밖·회계 보류) |
| 156 | `0xd666c0` | `v3_serpen_contest_clear_win` | serpen | 641B | 2 | 1 | 4 | 중간 | 3.12 | #24 DIFF 0(명세 밖·회계 보류) |
| 157 | `0xdea4a0` | `v3_epic_group_line` | epic | 180B | 1 | 0 | 4 | 잎 | 5.0 | #21 DIFF 0(명세 밖·회계 보류) |
| 158 | `0xd478c0` | `decide_deathmatch` | death_battle | 16861B | 1 | 2 | 1 | 거대 | 0.06 | 데스매치 전용(MOBA 미사용 · #17 과 같은 판정) |
| 159 | `0x12857f0` | `estimate_damage_to` | fight_model | 1111B | 91 | 0 | 2 | 잎 | 81.91 | game_core 경계(명세 대상 아님 · 계약만 확정) |
| 160 | `0xebf380` | `dm_flee_survival` | death_battle | 3191B | 1 | 0 | 2 | 잎 | 0.31 | 데스매치 전용(MOBA 미사용 · #17 과 같은 판정) |
| 161 | `0xec7930` | `deathmatch_incoming_threat` | death_battle | 859B | 2 | 1 | 2 | 중간 | 2.33 | 데스매치 전용(MOBA 미사용 · #17 과 같은 판정) |
| 162 | `0xea96e0` | `deathmatch_incoming_threat` | death_battle | 686B | 1 | 0 | 3 | 잎 | 1.46 | 데스매치 전용(MOBA 미사용 · #17 과 같은 판정) |

## game_core/engine 경계(지도 밖 · 서브트리가 부르는 것 · 대략 대역 0x1200000~0x1900000)

`0x1285320` `0x128cf70` `0x128ef00` `0x129d130` `0x129ed50` `0x129feb0` `0x12a0180` `0x12a06a0` `0x12a07d0` `0x131f300` `0x1323a00` `0x132b310` `0x144b810` `0x1450750` `0x1453260` `0x16047b0` `0x16fc160` `0x18096a0` `0x181cce0` `0x1821120` `0x18211f0` `0x1821530` `0x1821740`

이들은 명세 대상이 아니라 **계약(시그니처·반환 의미)만** 확정한다(tcx `--deep`/오라클). 예: `estimate_damage_to`(0x12857f0 · 호출자 91).