# probe060 1단계 결과 — 0.6.0 발화수(관리 화면 배경 리그 sim · 마지막 스냅샷 f17100)

[install] base=0x7ff6f47c0000 선택 260 · 설치 260 · 프롤로그 불일치 0 · 실패 0

발화 238 / 미발화 22 (프로브 260)

## 미발화(sweep 표본 없음 — 실경기/다른 씬 필요)

- [6] `e60420` sub_plan (i=7 · ✅동치)
- [9] `eed490` should_end_object_finish_kill_priority_battle (i=10 · 동치)
- [13] `d5ebb0` single_try_engage (i=15 · ✅동치)
- [28] `1012e50` serpen_giveup_chat_reason (i=32 · 동치)
- [41] `ef4ab0` TeamPlan::handle_epic_line_change (i=47 · 동치)
- [44] `fb49b0` check_epic_giveup (i=50 · ✅동치)
- [47] `fb34e0` check_epic_setup (i=53 · 동치)
- [51] `ef3da0` handle_press_epic (i=57 · 동치)
- [55] `e85d70` bush_distance_sq (i=61 · 동치)
- [75] `fb1b70` check_epic_hunt (i=81 · ✅동치)
- [79] `ede160` should_disengage_object_hunt (i=85 · 동치)
- [87] `1010060` check_serpen_giveup (i=93 · ⚠변경·한 줄)
- [89] `100f210` check_serpen_setup (i=95 · 동치)
- [92] `e87300` steal::evaluate_steal_for_target (i=98 · 동치)
- [131] `eacee0` EpicPokeSubPlan::score (i=138 · 동치)
- [165] `eccaf0` SmallActionPositioning::get_input (i=172 · 동치)
- [166] `eabbb0` EpicPokeSubPlan::action_candidates_old (i=174 · ✅동치)
- [169] `ed4350` SmallActionAroundRegion::get_input (i=177 · 동치)
- [191] `ea8610` EpicPokeSubPlan::action_candidates (i=199 · ✅동치)
- [202] `e5eb00` StealSubPlan::action_candidates (i=210 · ✅동치)
- [234] `e601b0` StealSubPlan::score (i=242 · 동치)
- [242] `fa8570` next_plan (i=250 · ✅동치)

## 발화 오름차순(콜러 리턴 RVA ≤8 · +ovf = 9 이상)

| idx | rva | 함수 | i | 판정 | 발화 | 콜러 |
|---|---|---|---|---|---|---|
| 63 | `d5ff20` | try_engage_dive | 69 | ⚠변경·한 줄 | 142 | d676b8 d6b5e2 d68456 |
| 167 | `f63a10` | SerpenPokeSubPlan::action_candidates_old | 175 | 동치 | 365 | f6052a |
| 192 | `f60480` | SerpenPokeSubPlan::action_candidates | 200 | ✅동치 | 365 | ed0f37 |
| 183 | `e96780` | EpicCheckSubPlan::action_candidates | 191 | 변경·심층 | 402 | ed0fb4 |
| 184 | `e9de70` | SerpenCheckSubPlan::action_candidates | 192 | ⚠변경·다건 | 609 | ed0d1e |
| 83 | `f0cfe0` | TeamPlan::v24_objective_setup_should_check_c | 89 | ⚠변경·한 줄 | 707 | dccd1c |
| 132 | `f64d30` | SerpenPokeSubPlan::score | 139 | 동치 | 751 | f629ea ed3f11 dfdcc7 f62a7d |
| 88 | `100d990` | check_serpen_hunt | 94 | 동치 | 1667 | eff5bb |
| 29 | `d56570` | v2_obj_restore_safe | 33 | ✅동치 | 1989 | d3a61c d3a9f3 d57c5b |
| 61 | `f0dc20` | v24_objective_setup_lane_pressure_ready | 67 | ⚠변경·한 줄 | 2160 | eff53c 100eaa3 |
| 220 | `dc0040` | TeamPlan::handle_nexus_attack | 228 | 동치 | 3070 | efde99 |
| 148 | `fa9d60` | LineGankerPlan::make_gank_battle | 155 | 변경·소 | 3363 | fac14d fad94c fadb37 fad8d2 fadecc fad741 |
| 46 | `f88d60` | v25_objective_far_split_pressure | 52 | 동치 | 3843 | f88ccb |
| 10 | `d582a0` | v3_fall_back_to_passive | 11 | 동치 | 4990 | f3dede |
| 110 | `1009a90` | can_tower_focused_when_attack | 116 | 동치 | 5460 | f43165 f66441 |
| 159 | `de7220` | utils::can1v1win | 166 | 동치 | 6955 | f384e6 f388ca |
| 139 | `ed0240` | SmallActionAroundBush::new_with_out_line | 146 | 동치 | 8029 | f3699c |
| 23 | `f892d0` | v23_objective_setup_pressure_line | 26 | 동치 | 9939 | d39944 fb8f30 fb8e37 f0d0ab f0e52a |
| 4 | `d5aff0` | v50_fold_dive_episode | 5 | ✅동치 | 10969 | d4a52c d4f237 |
| 251 | `eda1a0` | v48_projectile_profile | 259 | 동치 | 11124 | db2206 |
| 72 | `e76100` | objective_defense_role | 78 | 동치 | 17560 | effbb3 eff027 |
| 133 | `ecfff0` | SmallActionAroundBush::new_with_target | 140 | 동치 | 18474 | f368b6 |
| 76 | `fb5ff0` | epic_passive_plan | 82 | 변경·심층 | 20271 | d3a75b |
| 5 | `d6bf10` | v2_response_retreat_stance | 6 | ✅동치 | 26553 | d6664a d65c82 |
| 57 | `f18bf0` | should_keep_object_for_contested_wave_priori | 63 | 동치 | 28774 | f18be6 eff575 |
| 182 | `f65030` | AttackNexusSubPlan::action_candidates | 190 | 동치 | 29359 | ed0cf3 |
| 58 | `f88b30` | v25_objective_splitter_can_stay | 64 | 동치 | 39169 | dcc471 fd868a |
| 60 | `fb1510` | evaluate_gank_opportunity_with_score | 66 | ✅동치 | 41468 | fa359d d46bf5 d5cf93 d44b6c |
| 2 | `e82c00` | defensive_crisis | 3 | ⚠변경·한 줄 | 43350 | f7aa67 f7aec0 f7c2ca d6d030 |
| 231 | `ecfdd0` | around::SmallActionAroundBush::update_state | 239 | 동치 | 50445 | f3b26c |
| 161 | `ed0400` | SmallActionAroundBush::get_input | 168 | 변경 | 50446 | e5e752 |
| 78 | `ed8f40` | battle_check_with_list | 84 | 동치 | 52038 | f04448 |
| 187 | `ead1c0` | LineWaitSubPlan::action_candidates | 195 | ✅동치 | 55372 | ed0db2 |
| 245 | `f39240` | LineSafeSubPlan::action_candidates | 253 | 동치 | 61326 | ed0e3c |
| 253 | `ebab70` | kite_reposition_point | 261 | 동치 | 67727 | ea1e5a |
| 0 | `de1ad0` | ult | 0 | 동치 | 68881 | ddd60e |
| 186 | `f356b0` | HideSubPlan::action_candidates | 194 | 변경·심층 | 75203 | ed0e1b |
| 147 | `ddd460` | SmallActionUlt::get_input | 154 | 동치 | 79209 | e5e525 |
| 8 | `edfb50` | check_favorable_engage_formation | 9 | 동치 | 89869 | d638a1 d68bc6 d629e1 d675f0 d69872 d683cd |
| 227 | `f66a30` | AttackNexusSubPlan::score | 235 | ✅동치 | 100064 | ed4139 |
| 124 | `e83520` | noncombat_steroid_value | 131 | 동치 | 102259 | f7bd13 f7b6f8 |
| 90 | `1011d20` | serpen_passive_plan | 96 | 변경·심층 | 111139 | d3a3c1 |
| 74 | `d5f3c0` | try_engage | 80 | ⚠변경·다건 | 111568 | d6b026 d68c20 d63908 d639e1 d62beb d62c71 |
| 150 | `ecfa40` | SmallActionAroundPositionBush::get_input | 157 | ⚠변경·다건 | 126773 | e5e65c |
| 162 | `eb4880` | calculate_serpen_action_score | 169 | 동치 | 136678 | eb46ff |
| 115 | `de0a80` | convert_to_move_action_target | 122 | 동치 | 137005 | e5c627 e5ca85 e5cfb6 |
| 163 | `f86640` | calculate_epic_action_score | 170 | 동치 | 138950 | f864bf |
| 12 | `faa440` | update | 14 | ⚠변경·한 줄 | 142219 | ec25ee |
| 194 | `faa7c0` | LineGankerPlan::next_plan | 202 | 변경·심층 | 143562 | ec3055 |
| 223 | `eae650` | LineWaitSubPlan::score | 231 | 동치 | 158986 | ed3f71 |
| 222 | `f3a070` | LineSafeSubPlan::score | 230 | 동치 | 174397 | ed40d9 |
| 153 | `100bb90` | get_die_tick_player | 160 | 동치 | 181286 | fad2fd fad33b ef7130 |
| 207 | `fe3fd0` | v16_gambler_ult_cc_bonus | 215 | ✅동치 | 195088 | ea7dfd |
| 244 | `fe4950` | v16_knight_ult_zone_bonus | 252 | 동치 | 195088 | ea7e2f |
| 212 | `ec62f0` | PassiveLinePlan::v46_stage2 | 220 | 동치 | 195273 | ec929f ec9c10 |
| 190 | `f6bda0` | EpicHuntSubPlan::action_candidates | 198 | ⚠변경·한 줄 | 201020 | ed0d91 |
| 189 | `e986c0` | SerpenHuntSubPlan::action_candidates | 197 | ⚠변경·한 줄 | 207396 | ed0d66 |
| 181 | `f40930` | DefenseNexusSubPlan::action_candidates | 189 | ✅동치 | 234144 | ed0dd8 |
| 160 | `e95f70` | SmallActionLaneMinionPosition::get_input | 167 | 동치 | 267427 | e5e5cf |
| 145 | `e733f0` | line_champion_trade_allowance | 152 | 동치 | 271128 | e73a26 |
| 52 | `fb9b70` | target_bush_v41 | 58 | ✅동치 | 284976 | ec2ee4 facc5d d5d018 |
| 62 | `ddd7e0` | base_sub_goal | 68 | ⚠변경·한 줄 | 301142 | dd8bea ddca7a |
| 33 | `e759a0` | i_am_chosen_defender | 37 | ✅동치 | 346813 | f02665 d41502 e764f5 efeb32 f0155e f01f31 f08477 e76248 +ovf |
| 32 | `d75ad0` | calculate_nexus_defense_count | 36 | 동치 | 348470 | f02648 d414d5 effb9d efeb15 f01541 f01f14 eff011 f0845a +ovf |
| 157 | `de26b0` | abstract_input::skill2 | 164 | 동치 | 368298 | ddd3de |
| 1 | `f776b0` | calculate_jungle_action_score | 1 | 동치 | 375828 | eb2c09 eb2b32 |
| 146 | `ddd230` | SmallActionSkill2::get_input | 153 | 동치 | 395084 | e5e6d9 |
| 129 | `f89c30` | v23_should_break_objective_hunt_anchor | 136 | 동치 | 407736 | e9a2e2 e9b01f f6d981 f6e6cf |
| 149 | `f0c4a0` | v27_objective_discipline_action | 156 | 동치 | 408302 | e989a1 f6c0cb f604e6 |
| 142 | `f860e0` | epic_action_score | 149 | 동치 | 427791 | e02f93 ed41fa f70195 f70218 e02f03 |
| 71 | `e71d40` | DefenseNexusPlan::sub_plan | 77 | ✅동치 | 431709 | ec2da3 |
| 7 | `fd7b40` | is_end | 8 | ⚠변경·한 줄 | 434556 | ec2349 |
| 66 | `dcb890` | SerpenHuntAndPokePlan::is_end | 72 | ⚠변경·다건 | 439361 | ec2313 |
| 77 | `fd7f70` | EpicHuntAndPokePlan::sub_plan | 83 | 변경·심층 | 440171 | ec2ed0 |
| 141 | `eb4320` | serpen_action_score | 148 | 동치 | 444459 | ed3eb5 e9cb15 dfc023 dfbf93 e9cb98 |
| 143 | `100a380` | v22_lane_tower_pressure_attack_allowed | 150 | 동치 | 455994 | e60f48 f31790 eadf04 f39cde |
| 91 | `dcbe00` | SerpenHuntAndPokePlan::sub_plan | 97 | 변경·심층 | 457599 | ec2d0d |
| 156 | `de1f40` | abstract_input::skill | 163 | 동치 | 476994 | ddd1ae |
| 24 | `e77ec0` | nexus_under_direct_attack | 27 | 동치 | 484915 | e724c7 e7613e e72455 f415e1 ff1450 fdeb02 |
| 11 | `d5b410` | handle_chat | 12 | 동치 | 626511 | d45641 d520e2 |
| 38 | `d59280` | LegacyPlanHandler::take_misunderstood_receiv | 44 | 동치 | 626511 | d455e4 d5208a |
| 98 | `d5b980` | handle_chat_inner | 104 | 변경·심층 | 626511 | d45641 d520e2 |
| 144 | `ddd010` | SmallActionSkill::get_input | 151 | 동치 | 645768 | e5e5a2 |
| 15 | `ef4690` | v3_epicops_buff_window | 18 | ⚠변경·다건 | 697381 | f060b7 |
| 219 | `1009e10` | v47_tower_focus_position_dangerous | 227 | 동치 | 737810 | f34495 |
| 21 | `f882e0` | v23_healthy_allies_near_point | 24 | 동치 | 750220 | f89cb2 f89d1a f87dbf f88cf7 f89e20 |
| 26 | `f895f0` | v23_recent_visible_enemies_near_point | 29 | 동치 | 751522 | f89cd8 f89d40 f87de5 f0e4af f88d1d f89e46 f0ddbc |
| 43 | `ee4e70` | resolve_join_stake | 49 | ⚠변경·다건 | 782701 | d5dbbd d3811e dd0f02 |
| 246 | `e84340` | noncombat_steroid_window | 254 | ✅동치 | 802105 | f7bc81 f7b641 |
| 233 | `ea8130` | RecallSubPlan::action_candidates | 241 | 동치 | 802688 | ed0df5 |
| 230 | `100b570` | v22_current_line_non_champion_action_tower_r | 238 | ✅동치 | 829995 | f3421e |
| 215 | `f43b10` | DefenseNexusSubPlan::score | 223 | ✅동치 | 871409 | ed3e83 |
| 229 | `ea8450` | RecallSubPlan::score | 237 | 동치 | 898062 | ed3fd1 |
| 64 | `ee6d40` | tower_dive_is_viable | 70 | ⚠변경·다건 | 900618 | dcf89a d61c8e dd5983 d5dd31 dd2928 d617bb d5f75a d5e411 +ovf |
| 185 | `eafb50` | JungleSubPlan::action_candidates | 193 | 변경·심층 | 957993 | ed0d3b |
| 126 | `e80960` | buff_value_v54 | 133 | ⚠변경·다건 | 970252 | f7af38 f7ab15 |
| 31 | `ee5e80` | resolve_fight_stake | 35 | 변경 | 1032768 | dd1e3d ee7be4 ea4474 |
| 130 | `eb5140` | v48_on_cast_line | 137 | 동치 | 1094557 | ebad4d ebaf8a ea5785 ea4bff ea5484 |
| 112 | `de8f80` | check_nontarget | 119 | 동치 | 1117529 | ddd17a ddd3aa ddd5da |
| 97 | `ffdda0` | position_risk_all_zero_near | 103 | 동치 | 1154472 | ddb4cf |
| 257 | `e83ba0` | v54_aoe_ally_heal_value | 265 | 동치 | 1177818 | f7c38f f7bf4f |
| 236 | `fdb0e0` | move_actions::SmallActionRecall::is_end | 244 | 동치 | 1178407 | e5dc9a e5dc9a |
| 228 | `fe59e0` | v15_can_keep_support_pressure | 236 | 동치 | 1227431 | ea70a3 ea62f0 |
| 67 | `f88880` | wave_priority_clearer_position | 73 | ⚠변경·한 줄 | 1280509 | d39c44 d39be6 d3996a fb993c d397ff |
| 113 | `e73260` | line_recall_pressure_penalty | 120 | 동치 | 1317642 | e73a06 |
| 25 | `eecc90` | v21_should_defer_support_target | 28 | 동치 | 1370104 | dcf225 |
| 217 | `f3d660` | AgentVerHamster::item_v26 | 225 | ⚠변경·한 줄 | 1426049 | f3d5f6 f3ba6c |
| 84 | `f87600` | v3_epicops_defer_serpen | 90 | ✅동치 | 1461957 | f12384 efc653 f02ceb |
| 176 | `fdb210` | SmallActionRecall::get_input | 184 | ⚠변경·한 줄 | 1472347 | e5e689 e5e689 |
| 56 | `f87c30` | v23_enemy_object_pressure | 62 | 동치 | 1584265 | d39920 fb8e17 fb8f10 f89d94 |
| 208 | `f33fa0` | unsafe_v19_non_champion_walkup | 216 | ✅동치 | 1586447 | e00eea e00e3a |
| 193 | `de2320` | abstract_input::attack | 201 | 동치 | 1591744 | ddd7bd |
| 118 | `e95a30` | SmallActionLaneMinionPosition::should_suppre | 125 | 동치 | 1648873 | eb3e64 |
| 195 | `fa47a0` | PassiveJunglePlan::next_plan | 203 | 변경·심층 | 1712340 | ec302e |
| 85 | `fa3980` | PassiveJunglePlan::sub_plan | 91 | 변경·심층 | 1714507 | ec2ea9 |
| 122 | `edf1b0` | should_add_self_etc_buff_action | 129 | 동치 | 1772098 | ed6e02 ed7df7 ed7508 ed81bf |
| 27 | `fb78b0` | v3_epic_formation_role | 31 | 동치 | 1772579 | ef48d5 fb753d d6697a ef48ff ef492f ef495b ef4987 |
| 16 | `fafe50` | best_jungle_goal | 19 | 변경 | 1801897 | d3b345 fa3203 fa5bb2 fa66bb d3c6be fa4a45 |
| 100 | `f09860` | TeamPlan::v25_objective_posture | 106 | ✅동치 | 1807184 | d65604 e99a8a e9e514 f6d1ba d63d41 e96c03 |
| 17 | `f0c310` | v27_active_objective_discipline | 20 | 동치 | 1813680 | d6568f f0c4e7 |
| 250 | `e936c0` | SmallActionLaneMinionPosition::choose_goal | 258 | 동치 | 1917557 | eb387c e9614d |
| 30 | `e768f0` | has_line_defense_threat | 34 | 동치 | 1974432 | e755e1 |
| 226 | `eb28e0` | JungleSubPlan::score | 234 | ✅동치 | 2013322 | ed3ee3 |
| 170 | `ece1d0` | SmallActionAroundPosition::get_input | 178 | 동치 | 2064308 | e5e6fe |
| 137 | `e95770` | SmallActionLaneMinionPosition::has_current_e | 144 | 동치 | 2184758 | eb37c8 e95edf |
| 174 | `fddcd0` | SmallActionRunAway::get_input | 182 | ✅동치 | 2210354 | e5e4fd fdaec9 ec46b9 e965c0 |
| 117 | `e82990` | v55_seal_value | 124 | 동치 | 2216240 | f7bb74 |
| 119 | `e83000` | v55_banish_penalty | 126 | 동치 | 2216240 | f7bba9 |
| 123 | `e82410` | v55_mark_value | 130 | 동치 | 2216240 | f7bb56 |
| 216 | `fe3970` | v21_defensive_cc_score | 224 | 동치 | 2264574 | fe7990 fe78a4 ea7a64 ea7317 fe7abd ea7d11 |
| 121 | `fe16c0` | SmallActionTrace::expected_goal_position | 128 | 동치 | 2338594 | e5d621 |
| 209 | `fe6840` | v17_runaway_counterattack_bonus | 217 | ✅동치 | 2344405 | ea759a ea7b75 ea7c35 ea7e6f |
| 177 | `dde530` | get_input_target | 185 | 동치 | 2459257 | de2037 de2417 de27ad de1cac |
| 235 | `fd9770` | trace::SmallActionTrace::is_end | 243 | 동치 | 2518853 | f3b38f |
| 168 | `fd98c0` | SmallActionTrace::get_input | 176 | 동치 | 2563843 | e5e728 |
| 180 | `e9fe00` | BattleSubPlan::action_candidates | 188 | ✅동치 | 2745865 | ed0e96 |
| 211 | `ea6240` | BattleSubPlan::calculate_score_parameter_val | 219 | ✅동치 | 2749441 | ed3197 |
| 164 | `e73ac0` | line_projected_punish_damage_at | 171 | 동치 | 3049660 | e73983 |
| 86 | `eed7f0` | v46_flee_gate_check | 92 | ⚠변경·다건 | 3059610 | eca67d dcc3eb |
| 205 | `ec4c20` | PassiveLinePlan::v46_stage1 | 213 | ⚠변경·한 줄 | 3117998 | ec924d ec9bcd ec90c1 |
| 36 | `ee3e40` | fight_participants | 41 | ⚠변경·다건 | 3242383 | dd1c73 ea49a5 dd130a |
| 252 | `eebb40` | resolve_fight_stake_roster | 260 | 변경·소 | 3242383 | dd1d36 ea4a4d dd13b8 |
| 80 | `ed5720` | FightSituation::build | 86 | ⚠변경·다건 | 3421523 | dd47e3 |
| 93 | `e85f00` | steal::should_steal_now | 99 | ⚠변경·한 줄 | 3450310 | f0e666 |
| 249 | `dea9b0` | v30_wave_danger_chase_guard | 257 | 동치 | 3451929 | dd44d8 |
| 14 | `dea490` | max_range_nearly_can_use | 16 | 동치 | 3478351 | f30558 f50baf f307ea e9a83e e9bd63 e9a96e f6deee f6f3e1 +ovf |
| 224 | `100b2b0` | v30_line_champion_action_tower_aggro_risk | 232 | 동치 | 3870412 | e008aa e0083a |
| 152 | `ed79a0` | battle_ally_action | 159 | 동치 | 4463520 | f2e566 |
| 188 | `f2caf0` | LineDefenseSubPlan::action_candidates | 196 | ✅동치 | 4463520 | ed0c50 |
| 232 | `f34d10` | LineDefenseSubPlan::calculate_score_paramete | 240 | ⚠변경·한 줄 | 4464374 | ed1102 |
| 151 | `eb3980` | line_minion_action_candidates | 158 | 동치 | 4502366 | f3982f f2ddba ead97b |
| 22 | `eed240` | v22_visible_enemy_is_runaway_threat | 25 | 동치 | 4715853 | ddb859 ddb88c ddb8bf ddb8ee ddb960 ddb9aa ddb9f4 ddba3e +ovf |
| 116 | `ede720` | attack_structure_skill_action | 123 | 동치 | 4755365 | f39dc7 f2e4cf eadffd f43289 f66688 |
| 241 | `eb52b0` | base_battle_action | 249 | ✅동치 | 4802680 | ea1011 ea2b5b ea14fe ea158f ea3bc7 |
| 103 | `dcce40` | BattlePlan::update_v32 | 109 | 변경·심층 | 4880263 | ddbd40 |
| 42 | `eec620` | v25_scoped_battle_objective | 48 | 변경·한 줄 | 5000385 | dd8ff5 |
| 96 | `dd8e70` | BattlePlan::update | 102 | 변경·심층 | 5000385 | d5faeb d5fb97 ec254f d666b1 d667b8 d5e480 d64d05 fa9f51 +ovf |
| 206 | `ea6f40` | BattleSubPlan::score | 214 | 동치 | 5003144 | ed410b |
| 140 | `e73610` | line_action_economy_adjustment | 147 | 동치 | 5189387 | f3a179 f353e8 eae759 |
| 37 | `dd87b0` | SinglePlanBattle::with_runaway | 42 | 동치 | 5250006 | dd60c1 dd0206 dd6d5c dd591c dd4318 |
| 254 | `e93e00` | SmallActionLaneMinionPosition::target_score | 262 | 동치 | 5446107 | e7cfe2 e6bea9 e6c042 |
| 171 | `ec3060` | SmallActionAround::get_input | 179 | ✅동치 | 6493533 | e5e5fc |
| 178 | `f749a0` | calculate_action_score | 186 | 변경·심층 | 6622319 | f3a38b f3561a eae96b ea7541 ea795d ea7213 ea77c4 f43e7c +ovf |
| 154 | `eb3470` | lane_minion_position_action | 161 | ✅동치 | 6727611 | f2d746 f316c2 |
| 95 | `de6a80` | line_backfight_support_focus | 101 | 동치 | 6967603 | d4915b |
| 198 | `ec7180` | PassiveLinePlan::update | 206 | ⚠변경·다건 | 6967603 | ec25cc |
| 243 | `ed6300` | battle_action | 251 | 동치 | 6979934 | f397f1 f2dd67 eb0223 ead93c ea8266 f321cd f38666 e98cd5 +ovf |
| 108 | `ed8550` | attack_summon_action | 114 | 동치 | 6997686 | f3985f f2de33 eb0259 ead9b1 ea82ee f373cf e98d0b e9f2d4 +ovf |
| 155 | `e85460` | v57_summon_command_score | 162 | 동치 | 7384249 | f77a08 |
| 173 | `f77820` | calculate_interaction_action_score | 181 | 변경·심층 | 7384249 | f72fe8 f72dcb |
| 199 | `ee8450` | resolve_fight_uncached | 207 | ⚠변경·한 줄 | 7880582 | ee4c8f |
| 109 | `faf060` | is_cleared | 115 | 동치 | 8006125 | fa68a2 e05361 fa2c5b fa33b9 fa3417 |
| 20 | `f27ea0` | buy_item | 23 | ⚠변경·한 줄 | 8024858 | f27757 f27757 f3d635 ec73e7 fa68da f04b6c f3e667 |
| 50 | `ecb2b0` | PassiveLinePlan::sub_plan | 56 | ⚠변경·다건 | 8367523 | ec2d7e |
| 82 | `d57b00` | v2_apply_assign_commit | 88 | 변경 | 8498785 | d4a0c9 d47a9c d472c4 d51da3 |
| 18 | `f27140` | upgrade_item | 21 | ✅동치 | 8919401 | f3bab7 f2777b ec741d fa6904 f04bba dcc008 fd80d1 f3e72b |
| 135 | `e64920` | path_needs_tower_escape | 142 | 동치 | 9118876 | ec45f1 fdaa70 e964f8 |
| 120 | `de7980` | is_safe_recall | 127 | 동치 | 9691771 | ec4723 ecf7b3 e965fa fdb8b2 |
| 39 | `de2a90` | can_recall | 45 | 동치 | 9841748 | f2768b f2768b |
| 45 | `f1fce0` | check_press_tower_opportunity | 51 | 동치 | 9843145 | f061eb f02ddf d427af f08a7e |
| 255 | `e13c10` | get_small_action_score_closure | 263 | 변경 | 9921437 | d6d91b |
| 196 | `1000320` | calculate_score_parameter | 204 | ✅동치 | 9928627 | d6c37c |
| 197 | `d6c2b0` | LegacyPlanHandler::get_small_action | 205 | 변경·심층 | 9928627 | f3c24d |
| 201 | `de3e80` | build_minion_wave_snapshot | 209 | 동치 | 9928627 | d6c4f7 |
| 204 | `f3bde0` | AgentVerHamster::update_small_action | 212 | ✅동치 | 9928627 | f3de0b f3b538 f3df03 |
| 127 | `ee0960` | around::check_cell | 134 | 동치 | 10932200 | fd6839 e5ad4c fcabec e20d26 e2104f e2d984 d8cc2a db41bc +ovf |
| 65 | `f27590` | should_recall_to_shop | 71 | ⚠변경·한 줄 | 11037256 | f181e7 f181e7 |
| 125 | `de5780` | champion_hp_value_uncached | 132 | 동치 | 11161027 | db05c2 |
| 54 | `1009560` | can_trace_without_tower | 60 | 동치 | 11205247 | eecfdc ea7478 |
| 107 | `1008a40` | v3_deadly_edge_cells | 113 | 동치 | 11397027 | ec3889 ecf037 fe0b7e fd9e70 fda673 fdd322 |
| 81 | `d72e70` | check_kill | 87 | 변경·심층 | 11850893 | d6a332 fadaa5 |
| 101 | `f024d0` | TeamPlan::handle_none_or_gank_objective | 107 | 변경·심층 | 11968876 | f13cf4 |
| 136 | `f352d0` | LineDefenseSubPlan::score | 143 | 동치 | 12034988 | f3262e ed40a7 e00bba f326cb e00c37 |
| 158 | `100c2f0` | action_eval::evaluate_action | 165 | ⚠변경·다건 | 12368371 | f3a0e8 f3534d eae6c8 |
| 138 | `e5d4e0` | SmallActionPlay::evaluation_position | 145 | 동치 | 12860864 | 100c433 f7360c f72ebb |
| 218 | `f27910` | build_game_finish_check_state | 226 | 동치 | 13945355 | f2856e f28a16 |
| 238 | `ed3dd0` | SerpenCheckSubPlan::score | 246 | ⚠변경·다건 | 14967363 | e13e9a d6ee36 |
| 175 | `f72620` | interaction_score | 183 | ✅동치 | 15143636 | f3a159 eb2a90 f353c3 eae739 ea6ffd ea8497 ed3e51 eb43ae +ovf |
| 99 | `d38180` | passive_plan | 105 | 변경·심층 | 15510841 | d4a053 d473a0 d47269 d4add6 d51d45 d422c9 d58384 d42aff |
| 40 | `e74b40` | need_defense_nexus | 46 | 동치 | 15898050 | f02623 efeff3 f0843c efe1b0 eff8dd f01523 efeaf7 effb7f +ovf |
| 73 | `dc1c10` | engage::can_battle_triggered_filtered | 79 | 동치 | 16274834 | d659c3 d659c3 d449d1 |
| 104 | `d606b0` | LegacyPlanHandler::handle_interact_battle | 110 | 변경·심층 | 16723577 | d46e55 d46e55 |
| 114 | `de0d60` | safe_move_avoiding_enemy_well | 121 | 동치 | 16832003 | e5e7bb de22d9 de266c ddd1e1 de2a4c ddd411 de1f06 ddd641 |
| 240 | `e5da70` | cast::SmallActionUlt::is_end | 248 | ✅동치 | 16942158 | f3b38f |
| 94 | `f0e5f0` | TeamPlan::update_steal | 100 | ✅동치 | 17134522 | f0fb77 f0fb77 |
| 102 | `f10f70` | TeamPlan::update_objective_after_steal | 108 | 변경·심층 | 17134522 | f0fba1 |
| 105 | `d3d210` | LegacyPlanHandler::update | 111 | 변경·심층 | 17134522 | d5a987 |
| 210 | `f3adb0` | AgentVerHamster::update_state | 218 | ⚠변경·다건 | 17134522 | f3dbb9 |
| 221 | `f3d140` | AgentVerHamster::count_nearby_enemies | 229 | 동치 | 17162079 | f3b476 f3b476 f3cd79 |
| 134 | `e5e0e0` | SmallActionPlay::get_input | 141 | 동치 | 17914768 | f3ddc2 f3de56 f32aaa e9ce89 f3df4e f70509 f3e121 f62d69 |
| 248 | `d51a70` | LegacyPlanHandler::update_on_dead | 256 | 변경 | 18636597 | f4d1c8 |
| 258 | `f10c60` | TeamPlan::can_near_enemies_range | 266 | 동치 | 19758927 | ecc11f d6a5f2 dcd584 dcd6c3 d664a0 d6309c d686ab f04532 +ovf |
| 239 | `f3d570` | AgentVerHamster::buy_item | 247 | 동치 | 21769945 | f4ddd3 |
| 237 | `f3b9e0` | AgentVerHamster::upgrade_item | 245 | 동치 | 21866203 | f4d08a |
| 34 | `1009c20` | can_tower_focused_when_battle | 38 | 동치 | 23476604 | eecfa8 |
| 53 | `eecea0` | is_unreasonable_tower_dive_enemy | 59 | 동치 | 23736770 | dec6ef dd1b38 dd2c06 d5d797 dc1f49 dce838 dcf2e2 ebc75f +ovf |
| 49 | `feefc0` | EntityPositioningCache::new | 55 | 동치 | 26018945 | da8596 ffe0e5 |
| 213 | `de6280` | precompute_champion_powers | 221 | 동치 | 29467578 | 1000c62 100107f 10013c0 |
| 35 | `f87ea0` | is_wave_priority_start_line | 39 | 동치 | 31548309 | f88189 e6bca7 |
| 48 | `f18f90` | TeamPlan::update | 54 | 변경 | 35771119 | d3e3cb d521c5 |
| 68 | `ff11f0` | GoalData::update | 74 | ⚠변경·한 줄 | 35771119 | d3e52c d5227e |
| 69 | `ff2450` | SerpenStanceData::update_plan | 75 | ✅동치 | 35771119 | ff18e6 |
| 70 | `ff4940` | EpicStanceData::update_plan | 76 | ✅동치 | 35771119 | ff18c1 |
| 214 | `e76aa0` | nexus_last_stand_uncached | 222 | 동치 | 35771119 | db030e |
| 225 | `e78290` | nexus_final_stand_uncached | 233 | 동치 | 35771119 | db031d |
| 3 | `e753e0` | handle_line_defense | 4 | 동치 | 35856003 | f02768 f02827 f0290b |
| 111 | `e78a10` | base_attacking_minion_uncached | 118 | 동치 | 38131879 | db032c e77e8a f40b19 |
| 247 | `f28320` | end_check | 255 | ⚠변경·한 줄 | 42268443 | efdd47 efec03 |
| 59 | `100b8d0` | check_epic_kill_time_with_hp | 65 | 동치 | 43340526 | ff261c ff2890 ff2b2a f12719 ef5df4 efca2b f11d8a ef9f43 +ovf |
| 106 | `de6080` | nontarget_windup_perceived | 112 | 동치 | 69567134 | f2cd5b f2f5db eafd6b ea0054 ea0244 e997bb f5f258 e9e24b +ovf |
| 172 | `ff62e0` | position_eval_at_uncached | 180 | ⚠변경·부분규명 | 71268050 | ff606d |
| 203 | `edb240` | fight_check::check_kill_die_tick_uncached | 211 | 변경·심층 | 86652691 | edaf52 edaa6e |
| 256 | `e944e0` | SmallActionLaneMinionPosition::push_candidat | 264 | 동치 | 115734111 | e938fa e93d5d e93a56 |
| 200 | `f3da50` | get_input | 208 | ⚠변경·한 줄 | 122293503 | 174f9d2 |
| 179 | `ff5ec0` | position_eval_at | 187 | 동치 | 139519479 | f2d04d 100c470 f7363e f2f928 eaffe1 ec3faf ec494a ec4a15 +ovf |
| 19 | `1008640` | can_tower_focused | 22 | 동치 | 155389358 | e6478f e73e1d e95904 e9463c e95286 e64b78 e64c21 1009732 +ovf |
| 128 | `1009770` | v3_lethal_tower_position | 135 | 동치 | 173675858 | ec3dd9 ece71e fdfd41 |
| 259 | `ed5fd0` | expected_dps | 267 | 동치 | 296688772 | fe2913 ee0826 eebaa3 ee92b3 |