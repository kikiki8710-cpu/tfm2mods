# repro060 작업 순서 — 재현체(Rust 직접 구현) 잎부터 · 265 함수(09-17)

tier 1 잎·동치 120 · tier 2 동치(콜리 변경) 73 · tier 3 변경 A 55 · tier 4 변경 B/C·미독 17

| tier | 콜리 수 | i | 구 | 신 | 함수 | 판정 | conf | 콜리 변경 | ★미독 |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 117 | `de5300` | `f20f20` | camp_idx | 동치 | A | 0 | 0 |
| 1 | 1 | 120 | `e28060` | `e73260` | line_recall_pressure_penalty | 동치 | A | 0 | 0 |
| 1 | 2 | 9 | `ebd570` | `edfb50` | check_favorable_engage_formation | 동치 | A | 0 | 0 |
| 1 | 2 | 58 | `db8ba0` | `fb9b70` | target_bush_v41 | ✅동치 | A | 0 | 0 |
| 1 | 2 | 229 | `e8f850` | `f3d140` | AgentVerHamster::count_nearby_enemies | 동치 | A | 0 | 0 |
| 1 | 3 | 24 | `ec9840` | `f882e0` | v23_healthy_allies_near_point | 동치 | A | 0 | 0 |
| 1 | 3 | 44 | `e4b8c0` | `d59280` | LegacyPlanHandler::take_misunderstood_receiv | 동치 | A | 0 | 0 |
| 1 | 3 | 132 | `d390a0` | `de5780` | champion_hp_value_uncached | 동치 | A | 0 | 0 |
| 1 | 4 | 113 | `d97700` | `1008a40` | v3_deadly_edge_cells | 동치 | A | 0 | 0 |
| 1 | 4 | 140 | `dc2550` | `ecfff0` | SmallActionAroundBush::new_with_target | 동치 | A | 0 | 0 |
| 1 | 4 | 222 | `d3e660` | `e76aa0` | nexus_last_stand_uncached | 동치 | A | 0 | 0 |
| 1 | 5 | 13 | `df1c80` | `fa82a0` | target_bush_v30 | 동치 | A | 0 | 0 |
| 1 | 5 | 61 | `d9aa80` | `e85d70` | bush_distance_sq | 동치 | A | 0 | 0 |
| 1 | 5 | 63 | `de03d0` | `f18bf0` | should_keep_object_for_contested_wave_priori | 동치 | A | 0 | 0 |
| 1 | 5 | 134 | `dc8550` | `ee0960` | around::check_cell | 동치 | A | 0 | 0 |
| 1 | 5 | 136 | `ecb300` | `f89c30` | v23_should_break_objective_hunt_anchor | 동치 | A | 0 | 0 |
| 1 | 6 | 42 | `d51fb0` | `dd87b0` | SinglePlanBattle::with_runaway | 동치 | A | 0 | 0 |
| 1 | 6 | 43 | `dfb220` | `dd87b0` | BattlePlan::with_runaway | 동치 | A | 0 | 0 |
| 1 | 6 | 145 | `e23170` | `e5d4e0` | SmallActionPlay::evaluation_position | 동치 | A | 0 | 0 |
| 1 | 6 | 237 | `cc5fc0` | `ea8450` | RecallSubPlan::score | 동치 | A | 0 | 0 |
| 1 | 6 | 261 | `cd5ee0` | `ebab70` | kite_reposition_point | 동치 | A | 0 | 0 |
| 1 | 7 | 7 | `ccc010` | `e60420` | sub_plan | ✅동치 | A | 0 | 0 |
| 1 | 7 | 40 | `e4a780` | `d55830` | v3_assign_anchor | 동치 | A | 0 | 0 |
| 1 | 7 | 118 | `d405d0` | `e78a10` | base_attacking_minion_uncached | 동치 | A | 0 | 0 |
| 1 | 8 | 5 | `e59190` | `d5aff0` | v50_fold_dive_episode | ✅동치 | A | 0 | 0 |
| 1 | 8 | 26 | `eca9a0` | `f892d0` | v23_objective_setup_pressure_line | 동치 | A | 0 | 0 |
| 1 | 8 | 30 | `ec8af0` | `f86f50` | objective_is_damaged | 동치 | A | 0 | 0 |
| 1 | 8 | 34 | `d3e4b0` | `e768f0` | has_line_defense_threat | 동치 | A | 0 | 0 |
| 1 | 8 | 85 | `ebbb80` | `ede160` | should_disengage_object_hunt | 동치 | A | 0 | 0 |
| 1 | 8 | 122 | `d34470` | `de0a80` | convert_to_move_action_target | 동치 | A | 0 | 0 |
| 1 | 8 | 126 | `e02020` | `e83000` | v55_banish_penalty | 동치 | A | 0 | 0 |
| 1 | 8 | 142 | `e29520` | `e64920` | path_needs_tower_escape | 동치 | A | 0 | 0 |
| 1 | 8 | 233 | `d3fe50` | `e78290` | nexus_final_stand_uncached | 동치 | A | 0 | 0 |
| 1 | 8 | 236 | `d69120` | `fe59e0` | v15_can_keep_support_pressure | 동치 | A | 0 | 0 |
| 1 | 8 | 258 | `e248f0` | `e936c0` | SmallActionLaneMinionPosition::choose_goal | 동치 | A | 0 | 0 |
| 1 | 9 | 28 | `e0bd60` | `eecc90` | v21_should_defer_support_target | 동치 | A | 0 | 0 |
| 1 | 9 | 39 | `ec9400` | `f87ea0` | is_wave_priority_start_line | 동치 | A | 0 | 0 |
| 1 | 9 | 59 | `e0bf70` | `eecea0` | is_unreasonable_tower_dive_enemy | 동치 | A | 0 | 0 |
| 1 | 9 | 60 | `d98210` | `1009560` | can_trace_without_tower | 동치 | A | 0 | 0 |
| 1 | 9 | 119 | `d9f3c0` | `de8f80` | check_nontarget | 동치 | A | 0 | 0 |
| 1 | 9 | 239 | `dc2330` | `ecfdd0` | around::SmallActionAroundBush::update_state | 동치 | A | 0 | 0 |
| 1 | 10 | 31 | `dea800` | `fb78b0` | v3_epic_formation_role | 동치 | A | 0 | 0 |
| 1 | 10 | 52 | `eca430` | `f88d60` | v25_objective_far_split_pressure | 동치 | A | 0 | 0 |
| 1 | 10 | 115 | `d408e0` | `faf060` | is_cleared | 동치 | A | 0 | 0 |
| 1 | 10 | 131 | `e02540` | `e83520` | noncombat_steroid_value | 동치 | A | 0 | 0 |
| 1 | 10 | 137 | `cd0480` | `eb5140` | v48_on_cast_line | 동치 | A | 0 | 0 |
| 1 | 10 | 151 | `d845e0` | `ddd010` | SmallActionSkill::get_input | 동치 | A | 0 | 0 |
| 1 | 10 | 154 | `d84a30` | `ddd460` | SmallActionUlt::get_input | 동치 | A | 0 | 0 |
| 1 | 10 | 156 | `dd5270` | `f0c4a0` | v27_objective_discipline_action | 동치 | A | 0 | 0 |
| 1 | 10 | 161 | `e2a830` | `eb3470` | lane_minion_position_action | ✅동치 | A | 0 | 0 |
| 1 | 10 | 228 | `e35a40` | `dc0040` | TeamPlan::handle_nexus_attack | 동치 | A | 0 | 0 |
| 1 | 10 | 241 | `cc5ca0` | `ea8130` | RecallSubPlan::action_candidates | 동치 | A | 0 | 0 |
| 1 | 10 | 257 | `e0dfc0` | `dea9b0` | v30_wave_danger_chase_guard | 동치 | A | 0 | 0 |
| 1 | 11 | 129 | `ebcbd0` | `edf1b0` | should_add_self_etc_buff_action | 동치 | A | 0 | 0 |
| 1 | 11 | 153 | `d84800` | `ddd230` | SmallActionSkill2::get_input | 동치 | A | 0 | 0 |
| 1 | 12 | 20 | `dd50e0` | `f0c310` | v27_active_objective_discipline | 동치 | A | 0 | 0 |
| 1 | 12 | 55 | `d815e0` | `feefc0` | EntityPositioningCache::new | 동치 | A | 0 | 0 |
| 1 | 12 | 64 | `eca200` | `f88b30` | v25_objective_splitter_can_stay | 동치 | A | 0 | 0 |
| 1 | 12 | 125 | `e269a0` | `e95a30` | SmallActionLaneMinionPosition::should_suppre | 동치 | A | 0 | 0 |
| 1 | 12 | 244 | `dbd130` | `fdb0e0` | move_actions::SmallActionRecall::is_end | 동치 | A | 0 | 0 |
| 1 | 13 | 16 | `e0daa0` | `dea490` | max_range_nearly_can_use | 동치 | A | 0 | 0 |
| 1 | 13 | 66 | `d40f10` | `fb1510` | evaluate_gank_opportunity_with_score | ✅동치 | A | 0 | 0 |
| 1 | 13 | 79 | `e38c90` | `dc1c10` | engage::can_battle_triggered_filtered | 동치 | A | 0 | 0 |
| 1 | 13 | 94 | `d61330` | `100d990` | check_serpen_hunt | 동치 | A | 0 | 0 |
| 1 | 13 | 242 | `cbbca0` | `e601b0` | StealSubPlan::score | 동치 | A | 0 | 0 |
| 1 | 14 | 4 | `d3cfa0` | `e753e0` | handle_line_defense | 동치 | A | 0 | 0 |
| 1 | 14 | 46 | `d3c700` | `e74b40` | need_defense_nexus | 동치 | A | 0 | 0 |
| 1 | 14 | 57 | `dcd930` | `ef3da0` | handle_press_epic | 동치 | A | 0 | 0 |
| 1 | 14 | 77 | `d2da10` | `e71d40` | DefenseNexusPlan::sub_plan | ✅동치 | A | 0 | 0 |
| 1 | 14 | 144 | `e266e0` | `e95770` | SmallActionLaneMinionPosition::has_current_e | 동치 | A | 0 | 0 |
| 1 | 14 | 167 | `e26c40` | `e95f70` | SmallActionLaneMinionPosition::get_input | 동치 | A | 0 | 0 |
| 1 | 14 | 243 | `cce0c0` | `fd9770` | trace::SmallActionTrace::is_end | 동치 | A | 0 | 0 |
| 1 | 15 | 29 | `ecacc0` | `f895f0` | v23_recent_visible_enemies_near_point | 동치 | A | 0 | 0 |
| 1 | 15 | 32 | `d665e0` | `1012e50` | serpen_giveup_chat_reason | 동치 | A | 0 | 0 |
| 1 | 15 | 38 | `d988d0` | `1009c20` | can_tower_focused_when_battle | 동치 | A | 0 | 0 |
| 1 | 15 | 128 | `caff00` | `fe16c0` | SmallActionTrace::expected_goal_position | 동치 | A | 0 | 0 |
| 1 | 15 | 158 | `e2ac00` | `eb3980` | line_minion_action_candidates | 동치 | A | 0 | 0 |
| 1 | 15 | 264 | `e25450` | `e944e0` | SmallActionLaneMinionPosition::push_candidat | 동치 | A | 0 | 0 |
| 1 | 16 | 27 | `d3fa80` | `e77ec0` | nexus_under_direct_attack | 동치 | A | 0 | 0 |
| 1 | 16 | 45 | `d36480` | `de2a90` | can_recall | 동치 | A | 0 | 0 |
| 1 | 16 | 51 | `de40c0` | `f1fce0` | check_press_tower_opportunity | 동치 | A | 0 | 0 |
| 1 | 16 | 139 | `e81390` | `f64d30` | SerpenPokeSubPlan::score | 동치 | A | 0 | 0 |
| 1 | 16 | 201 | `d35d10` | `de2320` | abstract_input::attack | 동치 | A | 0 | 0 |
| 1 | 17 | 10 | `e0c560` | `eed490` | should_end_object_finish_kill_priority_battl | 동치 | A | 0 | 0 |
| 1 | 17 | 47 | `dce520` | `ef4ab0` | TeamPlan::handle_epic_line_change | 동치 | A | 0 | 0 |
| 1 | 17 | 62 | `ec9190` | `f87c30` | v23_enemy_object_pressure | 동치 | A | 0 | 0 |
| 1 | 17 | 112 | `d399a0` | `de6080` | nontarget_windup_perceived | 동치 | A | 0 | 0 |
| 1 | 17 | 138 | `ccaa10` | `eacee0` | EpicPokeSubPlan::score | 동치 | A | 0 | 0 |
| 1 | 17 | 163 | `d35930` | `de1f40` | abstract_input::skill | 동치 | A | 0 | 0 |
| 1 | 18 | 81 | `de5340` | `fb1b70` | check_epic_hunt | ✅동치 | A | 0 | 0 |
| 1 | 18 | 100 | `dd90c0` | `f0e5f0` | TeamPlan::update_steal | ✅동치 | A | 0 | 0 |
| 1 | 18 | 221 | `d39ba0` | `de6280` | precompute_champion_powers | 동치 | A | 0 | 0 |
| 1 | 18 | 226 | `e7b0b0` | `f27910` | build_game_finish_check_state | 동치 | A | 0 | 0 |
| 1 | 18 | 259 | `eba320` | `eda1a0` | v48_projectile_profile | 동치 | A | 0 | 0 |
| 1 | 20 | 147 | `e28410` | `e73610` | line_action_economy_adjustment | 동치 | A | 0 | 0 |
| 1 | 20 | 148 | `ccf670` | `eb4320` | serpen_action_score | 동치 | A | 0 | 0 |
| 1 | 20 | 149 | `ec7c90` | `f860e0` | epic_action_score | 동치 | A | 0 | 0 |
| 1 | 20 | 224 | `d67060` | `fe3970` | v21_defensive_cc_score | 동치 | A | 0 | 0 |
| 1 | 21 | 95 | `d62bb0` | `100f210` | check_serpen_setup | 동치 | A | 0 | 0 |
| 1 | 21 | 164 | `d360a0` | `de26b0` | abstract_input::skill2 | 동치 | A | 0 | 0 |
| 1 | 21 | 172 | `dbf680` | `eccaf0` | SmallActionPositioning::get_input | 동치 | A | 0 | 0 |
| 1 | 22 | 50 | `de81b0` | `fb49b0` | check_epic_giveup | ✅동치 | A | 0 | 0 |
| 1 | 22 | 127 | `d9ddc0` | `de7980` | is_safe_recall | 동치 | A | 0 | 0 |
| 1 | 22 | 238 | `d9a220` | `100b570` | v22_current_line_non_champion_action_tower_r | ✅동치 | A | 0 | 0 |
| 1 | 23 | 114 | `eb8b80` | `ed8550` | attack_summon_action | 동치 | A | 0 | 0 |
| 1 | 23 | 123 | `ebc140` | `ede720` | attack_structure_skill_action | 동치 | A | 0 | 0 |
| 1 | 24 | 234 | `cc5a10` | `eb28e0` | JungleSubPlan::score | ✅동치 | A | 0 | 0 |
| 1 | 25 | 25 | `e0c310` | `eed240` | v22_visible_enemy_is_runaway_threat | 동치 | A | 0 | 0 |
| 1 | 25 | 36 | `e6d000` | `d75ad0` | calculate_nexus_defense_count | 동치 | A | 0 | 0 |
| 1 | 25 | 106 | `dd26e0` | `f09860` | TeamPlan::v25_objective_posture | ✅동치 | A | 0 | 0 |
| 1 | 25 | 121 | `d34750` | `de0d60` | safe_move_avoiding_enemy_well | 동치 | A | 0 | 0 |
| 1 | 27 | 0 | `d354c0` | `de1ad0` | ult | 동치 | A | 0 | 0 |
| 1 | 27 | 53 | `de6ce0` | `fb34e0` | check_epic_setup | 동치 | A | 0 | 0 |
| 1 | 28 | 37 | `d3d560` | `e759a0` | i_am_chosen_defender | ✅동치 | A | 0 | 0 |
| 1 | 30 | 159 | `eb77a0` | `ed79a0` | battle_ally_action | 동치 | A | 0 | 0 |
| 1 | 31 | 252 | `d68090` | `fe4950` | v16_knight_ult_zone_bonus | 동치 | A | 0 | 0 |
| 1 | 33 | 210 | `cba660` | `e5eb00` | StealSubPlan::action_candidates | ✅동치 | A | 0 | 0 |
| 1 | 40 | 195 | `eb43a0` | `ead1c0` | LineWaitSubPlan::action_candidates | ✅동치 | A | 0 | 0 |
| 1 | 41 | 190 | `e81680` | `f65030` | AttackNexusSubPlan::action_candidates | 동치 | A | 0 | 0 |
| 1 | 43 | 253 | `ccacf0` | `f39240` | LineSafeSubPlan::action_candidates | 동치 | A | 0 | 0 |
| 2 | 4 | 84 | `eb9570` | `ed8f40` | battle_check_with_list | 동치 | A | 1 | 0 |
| 2 | 4 | 146 | `dc27a0` | `ed0240` | SmallActionAroundBush::new_with_out_line | 동치 | A | 1 | 0 |
| 2 | 4 | 152 | `e281f0` | `e733f0` | line_champion_trade_allowance | 동치 | A | 1 | 0 |
| 2 | 6 | 124 | `e019d0` | `e82990` | v55_seal_value | 동치 | A | 1 | 0 |
| 2 | 7 | 1 | `d5ba80` | `f776b0` | calculate_jungle_action_score | 동치 | A | 1 | 0 |
| 2 | 7 | 166 | `d3ab40` | `de7220` | utils::can1v1win | 동치 | A | 1 | 0 |
| 2 | 8 | 116 | `d98740` | `1009a90` | can_tower_focused_when_attack | 동치 | A | 1 | 0 |
| 2 | 9 | 6 | `e657a0` | `d6bf10` | v2_response_retreat_stance | ✅동치 | A | 2 | 0 |
| 2 | 9 | 245 | `e8e160` | `f3b9e0` | AgentVerHamster::upgrade_item | 동치 | A | 1 | 0 |
| 2 | 10 | 162 | `e04400` | `e85460` | v57_summon_command_score | 동치 | A | 2 | 0 |
| 2 | 10 | 187 | `d84db0` | `ff5ec0` | position_eval_at | 동치 | A | 1 | 0 |
| 2 | 10 | 217 | `d69f80` | `fe6840` | v17_runaway_counterattack_bonus | ✅동치 | A | 2 | 0 |
| 2 | 11 | 65 | `de3d90` | `100b8d0` | check_epic_kill_time_with_hp | 동치 | A | 1 | 0 |
| 2 | 11 | 130 | `e01450` | `e82410` | v55_mark_value | 동치 | A | 1 | 0 |
| 2 | 12 | 15 | `e5c1f0` | `d5ebb0` | single_try_engage | ✅동치 | A | 3 | 0 |
| 2 | 12 | 22 | `d97300` | `1008640` | can_tower_focused | 동치 | A | 1 | 0 |
| 2 | 12 | 101 | `d3a3a0` | `de6a80` | line_backfight_support_focus | 동치 | A | 2 | 0 |
| 2 | 12 | 219 | `cc20a0` | `ea6240` | BattleSubPlan::calculate_score_parameter_val | ✅동치 | A | 1 | 0 |
| 2 | 12 | 247 | `e8fc80` | `f3d570` | AgentVerHamster::buy_item | 동치 | A | 2 | 0 |
| 2 | 13 | 135 | `d98420` | `1009770` | v3_lethal_tower_position | 동치 | A | 1 | 0 |
| 2 | 13 | 169 | `ccfbc0` | `eb4880` | calculate_serpen_action_score | 동치 | A | 1 | 0 |
| 2 | 13 | 170 | `ec81e0` | `f86640` | calculate_epic_action_score | 동치 | A | 1 | 0 |
| 2 | 14 | 235 | `e83080` | `f66a30` | AttackNexusSubPlan::score | ✅동치 | A | 1 | 0 |
| 2 | 15 | 12 | `e595b0` | `d5b410` | handle_chat | 동치 | A | 1 | 0 |
| 2 | 15 | 230 | `ccbb10` | `f3a070` | LineSafeSubPlan::score | 동치 | A | 2 | 0 |
| 2 | 15 | 231 | `eb5840` | `eae650` | LineWaitSubPlan::score | 동치 | A | 2 | 0 |
| 2 | 15 | 254 | `e03360` | `e84340` | noncombat_steroid_window | ✅동치 | A | 2 | 0 |
| 2 | 15 | 262 | `e25030` | `e93e00` | SmallActionLaneMinionPosition::target_score | 동치 | A | 1 | 0 |
| 2 | 15 | 267 | `eb5dd0` | `ed5fd0` | expected_dps | 동치 | A | 1 | 0 |
| 2 | 16 | 33 | `e4aec0` | `d56570` | v2_obj_restore_safe | ✅동치 | A | 1 | 0 |
| 2 | 16 | 160 | `de3630` | `100bb90` | get_die_tick_player | 동치 | A | 1 | 0 |
| 2 | 16 | 212 | `e8e560` | `f3bde0` | AgentVerHamster::update_small_action | ✅동치 | A | 1 | 0 |
| 2 | 17 | 215 | `d676c0` | `fe3fd0` | v16_gambler_ult_cc_bonus | ✅동치 | A | 2 | 0 |
| 2 | 17 | 227 | `d98ac0` | `1009e10` | v47_tower_focus_position_dangerous | 동치 | A | 2 | 0 |
| 2 | 18 | 248 | `e23750` | `e5da70` | cast::SmallActionUlt::is_end | ✅동치 | A | 1 | 0 |
| 2 | 18 | 265 | `e02bc0` | `e83ba0` | v54_aoe_ally_heal_value | 동치 | A | 1 | 0 |
| 2 | 19 | 21 | `e7a8c0` | `f27140` | upgrade_item | ✅동치 | A | 1 | 0 |
| 2 | 19 | 150 | `d99030` | `100a380` | v22_lane_tower_pressure_attack_allowed | 동치 | A | 2 | 0 |
| 2 | 19 | 171 | `e288c0` | `e73ac0` | line_projected_punish_damage_at | 동치 | A | 1 | 0 |
| 2 | 20 | 11 | `e4b5d0` | `d582a0` | v3_fall_back_to_passive | 동치 | A | 2 | 0 |
| 2 | 21 | 76 | `de1ee0` | `ff4940` | EpicStanceData::update_plan | ✅동치 | A | 2 | 0 |
| 2 | 21 | 143 | `e8b200` | `f352d0` | LineDefenseSubPlan::score | 동치 | A | 2 | 0 |
| 2 | 21 | 220 | `d27d50` | `ec62f0` | PassiveLinePlan::v46_stage2 | 동치 | A | 1 | 0 |
| 2 | 22 | 78 | `d3dcc0` | `e76100` | objective_defense_role | 동치 | A | 1 | 0 |
| 2 | 22 | 98 | `d9bce0` | `e87300` | steal::evaluate_steal_for_target | 동치 | A | 1 | 0 |
| 2 | 23 | 266 | `dd9f30` | `f10c60` | TeamPlan::can_near_enemies_range | 동치 | A | 1 | 0 |
| 2 | 25 | 90 | `ec8ba0` | `f87600` | v3_epicops_defer_serpen | ✅동치 | A | 1 | 0 |
| 2 | 25 | 141 | `e23fa0` | `e5e0e0` | SmallActionPlay::get_input | 동치 | A | 11 | 0 |
| 2 | 25 | 176 | `cce210` | `fd98c0` | SmallActionTrace::get_input | 동치 | A | 7 | 0 |
| 2 | 25 | 223 | `e95ae0` | `f43b10` | DefenseNexusSubPlan::score | ✅동치 | A | 1 | 0 |
| 2 | 27 | 185 | `d31f20` | `dde530` | get_input_target | 동치 | A | 1 | 0 |
| 2 | 27 | 232 | `d99f60` | `100b2b0` | v30_line_champion_action_tower_aggro_risk | 동치 | A | 1 | 0 |
| 2 | 28 | 75 | `dd73b0` | `ff2450` | SerpenStanceData::update_plan | ✅동치 | A | 1 | 0 |
| 2 | 28 | 177 | `dbbd60` | `ed4350` | SmallActionAroundRegion::get_input | 동치 | A | 1 | 0 |
| 2 | 29 | 209 | `d377a0` | `de3e80` | build_minion_wave_snapshot | 동치 | A | 1 | 0 |
| 2 | 30 | 175 | `e80070` | `f63a10` | SerpenPokeSubPlan::action_candidates_old | 동치 | A | 1 | 0 |
| 2 | 33 | 174 | `cc9740` | `eabbb0` | EpicPokeSubPlan::action_candidates_old | ✅동치 | A | 1 | 0 |
| 2 | 35 | 214 | `cc3080` | `ea6f40` | BattleSubPlan::score | 동치 | A | 2 | 0 |
| 2 | 36 | 250 | `df1f50` | `fa8570` | next_plan | ✅동치 | A | 1 | 0 |
| 2 | 38 | 178 | `dc0800` | `ece1d0` | SmallActionAroundPosition::get_input | 동치 | A | 1 | 0 |
| 2 | 41 | 183 | `d57540` | `f72620` | interaction_score | ✅동치 | A | 3 | 0 |
| 2 | 47 | 216 | `e8a100` | `f33fa0` | unsafe_v19_non_champion_walkup | ✅동치 | A | 2 | 0 |
| 2 | 48 | 103 | `d8ca70` | `ffdda0` | position_risk_all_zero_near | 동치 | A | 1 | 0 |
| 2 | 48 | 251 | `eb6100` | `ed6300` | battle_action | 동치 | A | 1 | 0 |
| 2 | 49 | 189 | `e928f0` | `f40930` | DefenseNexusSubPlan::action_candidates | ✅동치 | A | 2 | 0 |
| 2 | 53 | 179 | `db6ff0` | `ec3060` | SmallActionAround::get_input | ✅동치 | A | 1 | 0 |
| 2 | 54 | 182 | `dc3240` | `fddcd0` | SmallActionRunAway::get_input | ✅동치 | A | 3 | 0 |
| 2 | 67 | 199 | `cc6170` | `ea8610` | EpicPokeSubPlan::action_candidates | ✅동치 | A | 5 | 0 |
| 2 | 76 | 200 | `e7caf0` | `f60480` | SerpenPokeSubPlan::action_candidates | ✅동치 | A | 2 | 0 |
| 2 | 79 | 204 | `d8eff0` | `1000320` | calculate_score_parameter | ✅동치 | A | 5 | 0 |
| 2 | 85 | 188 | `cbbdb0` | `e9fe00` | BattleSubPlan::action_candidates | ✅동치 | A | 3 | 0 |
| 2 | 88 | 249 | `cd05f0` | `eb52b0` | base_battle_action | ✅동치 | A | 1 | 0 |
| 2 | 122 | 196 | `e83390` | `f2caf0` | LineDefenseSubPlan::action_candidates | ✅동치 | A | 3 | 0 |
| 3 | 3 | 240 | `e8aeb0` | `f34d10` | LineDefenseSubPlan::calculate_score_paramete | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 4 | 73 | `ec9de0` | `f88880` | wave_priority_clearer_position | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 5 | 35 | `e06df0` | `ee5e80` | resolve_fight_stake | 변경 | A | 0 | 0 |
| 3 | 5 | 157 | `dc2070` | `ecfa40` | SmallActionAroundPositionBush::get_input | ⚠변경·다건 | A | 0 | 0 |
| 3 | 5 | 263 | `ca6700` | `e13c10` | get_small_action_score_closure | 변경 | A | 0 | 0 |
| 3 | 7 | 99 | `d9ac10` | `e85f00` | steal::should_steal_now | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 8 | 68 | `dff080` | `ddd7e0` | base_sub_goal | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 8 | 260 | `e0b030` | `eebb40` | resolve_fight_stake_roster | 변경·소 | A | 0 | 0 |
| 3 | 9 | 48 | `e0b730` | `eec620` | v25_scoped_battle_objective | 변경·한 줄 | A | 0 | 0 |
| 3 | 10 | 155 | `db8e60` | `fa9d60` | LineGankerPlan::make_gank_battle | 변경·소 | A | 0 | 0 |
| 3 | 10 | 168 | `dc2960` | `ed0400` | SmallActionAroundBush::get_input | 변경 | A | 0 | 0 |
| 3 | 11 | 96 | `d65620` | `1011d20` | serpen_passive_plan | 변경·심층 | A | 0 | 0 |
| 3 | 13 | 14 | `db90f0` | `faa440` | update | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 14 | 69 | `e5d300` | `d5ff20` | try_engage_dive | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 14 | 86 | `dfe2a0` | `ed5720` | FightSituation::build | ⚠변경·다건 | A | 0 | 0 |
| 3 | 15 | 18 | `dce220` | `ef4690` | v3_epicops_buff_window | ⚠변경·다건 | A | 0 | 0 |
| 3 | 15 | 225 | `e8fd70` | `f3d660` | AgentVerHamster::item_v26 | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 16 | 70 | `e07430` | `ee6d40` | tower_dive_is_viable | ⚠변경·다건 | A | 0 | 0 |
| 3 | 16 | 88 | `e4b070` | `d57b00` | v2_apply_assign_commit | 변경 | A | 0 | 0 |
| 3 | 17 | 80 | `e5ca10` | `d5f3c0` | try_engage | ⚠변경·다건 | A | 0 | 0 |
| 3 | 17 | 93 | `d639f0` | `1010060` | check_serpen_giveup | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 17 | 246 | `e388c0` | `ed3dd0` | SerpenCheckSubPlan::score | ⚠변경·다건 | A | 0 | 0 |
| 3 | 18 | 67 | `dd6b40` | `f0dc20` | v24_objective_setup_lane_pressure_ready | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 18 | 97 | `df0e90` | `dcbe00` | SerpenHuntAndPokePlan::sub_plan | 변경·심층 | A | 0 | 0 |
| 3 | 19 | 82 | `de92d0` | `fb5ff0` | epic_passive_plan | 변경·심층 | A | 0 | 0 |
| 3 | 19 | 133 | `dffa10` | `e80960` | buff_value_v54 | ⚠변경·다건 | A | 0 | 0 |
| 3 | 20 | 89 | `dd5db0` | `f0cfe0` | TeamPlan::v24_objective_setup_should_check_c | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 21 | 72 | `df0a90` | `dcb890` | SerpenHuntAndPokePlan::is_end | ⚠변경·다건 | A | 0 | 0 |
| 3 | 22 | 83 | `defcd0` | `fd7f70` | EpicHuntAndPokePlan::sub_plan | 변경·심층 | A | 0 | 0 |
| 3 | 22 | 255 | `e7b9f0` | `f28320` | end_check | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 23 | 8 | `defa20` | `fd7b40` | is_end | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 24 | 3 | `e01c40` | `e82c00` | defensive_crisis | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 24 | 23 | `e7b640` | `f27ea0` | buy_item | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 24 | 71 | `e7acd0` | `f27590` | should_recall_to_shop | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 24 | 87 | `e6b800` | `d72e70` | check_kill | 변경·심층 | A | 0 | 0 |
| 3 | 25 | 74 | `dccc60` | `ff11f0` | GoalData::update | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 25 | 91 | `d2e500` | `fa3980` | PassiveJunglePlan::sub_plan | 변경·심층 | A | 0 | 0 |
| 3 | 25 | 256 | `e49a50` | `d51a70` | LegacyPlanHandler::update_on_dead | 변경 | A | 0 | 0 |
| 3 | 26 | 165 | `e29b40` | `100c2f0` | action_eval::evaluate_action | ⚠변경·다건 | A | 0 | 0 |
| 3 | 26 | 192 | `e8b5e0` | `e9de70` | SerpenCheckSubPlan::action_candidates | ⚠변경·다건 | A | 0 | 0 |
| 3 | 28 | 213 | `d26900` | `ec4c20` | PassiveLinePlan::v46_stage1 | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 29 | 49 | `e05e70` | `ee4e70` | resolve_join_stake | ⚠변경·다건 | A | 0 | 0 |
| 3 | 29 | 54 | `de0770` | `f18f90` | TeamPlan::update | 변경 | A | 0 | 0 |
| 3 | 29 | 92 | `d3b2a0` | `eed7f0` | v46_flee_gate_check | ⚠변경·다건 | A | 0 | 0 |
| 3 | 30 | 186 | `d59940` | `f749a0` | calculate_action_score | 변경·심층 | A | 0 | 0 |
| 3 | 32 | 191 | `cb03b0` | `e96780` | EpicCheckSubPlan::action_candidates | 변경·심층 | A | 0 | 0 |
| 3 | 33 | 56 | `d2c5d0` | `ecb2b0` | PassiveLinePlan::sub_plan | ⚠변경·다건 | A | 0 | 0 |
| 3 | 35 | 202 | `db9430` | `faa7c0` | LineGankerPlan::next_plan | 변경·심층 | A | 0 | 0 |
| 3 | 38 | 102 | `dfb840` | `dd8e70` | BattlePlan::update | 변경·심층 | A | 0 | 0 |
| 3 | 40 | 211 | `eba9b0` | `edb240` | fight_check::check_kill_die_tick_uncached | 변경·심층 | A | 0 | 0 |
| 3 | 42 | 184 | `dbd260` | `fdb210` | SmallActionRecall::get_input | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 45 | 208 | `e900b0` | `f3da50` | get_input | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 92 | 181 | `d5bbf0` | `f77820` | calculate_interaction_action_score | 변경·심층 | A | 0 | 0 |
| 3 | 98 | 197 | `cb1ce0` | `e986c0` | SerpenHuntSubPlan::action_candidates | ⚠변경·한 줄 | A | 0 | 0 |
| 3 | 113 | 198 | `eaeda0` | `f6bda0` | EpicHuntSubPlan::action_candidates | ⚠변경·한 줄 | A | 0 | 0 |
| 4 | 8 | 41 | `e04f50` | `ee3e40` | fight_participants | ⚠변경·다건 | B | 0 | 0 |
| 4 | 10 | 19 | `d40b20` | `fafe50` | best_jungle_goal | 변경 | B | 0 | 0 |
| 4 | 15 | 207 | `e083c0` | `ee8450` | resolve_fight_uncached | ⚠변경·한 줄 | B | 0 | 0 |
| 4 | 28 | 218 | `e8d6f0` | `f3adb0` | AgentVerHamster::update_state | ⚠변경·다건 | B | 0 | 0 |
| 4 | 36 | 105 | `e46bc0` | `d38180` | passive_plan | 변경·심층 | B | 0 | 10 |
| 4 | 41 | 104 | `e59b20` | `d5b980` | handle_chat_inner | 변경·심층 | B | 0 | 2 |
| 4 | 42 | 193 | `cc4260` | `eafb50` | JungleSubPlan::action_candidates | 변경·심층 | A | 0 | 2 |
| 4 | 51 | 194 | `cb7540` | `f356b0` | HideSubPlan::action_candidates | 변경·심층 | B | 0 | 3 |
| 4 | 60 | 109 | `df36e0` | `dcce40` | BattlePlan::update_v32 | 변경·심층 | B | 0 | 6 |
| 4 | 65 | 203 | `d2f180` | `fa47a0` | PassiveJunglePlan::next_plan | 변경·심층 | B | 0 | 2 |
| 4 | 70 | 205 | `e65b10` | `d6c2b0` | LegacyPlanHandler::get_small_action | 변경·심층 | A | 0 | 1 |
| 4 | 77 | 107 | `dcee40` | `f024d0` | TeamPlan::handle_none_or_gank_objective | 변경·심층 | B | 0 | 5 |
| 4 | 77 | 110 | `e5d5d0` | `d606b0` | LegacyPlanHandler::handle_interact_battle | 변경·심층 | B | 0 | 1 |
| 4 | 84 | 206 | `d28800` | `ec7180` | PassiveLinePlan::update | ⚠변경·다건 | B | 0 | 0 |
| 4 | 111 | 108 | `dda220` | `f10f70` | TeamPlan::update_objective_after_steal | 변경·심층 | B | 0 | 16 |
| 4 | 115 | 180 | `d851d0` | `ff62e0` | position_eval_at_uncached | ⚠변경·부분규명 | B | 0 | 0 |
| 4 | 133 | 111 | `e4c5c0` | `d3d210` | LegacyPlanHandler::update | 변경·심층 | C | 0 | 9 |