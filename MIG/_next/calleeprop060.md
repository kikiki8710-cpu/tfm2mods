# calleeprop060 — 본체 동치 함수의 콜리 변경 전파(09-17)

판정: 콜리 전부 동치 119 · 콜리 변경 있음 55 · 콜리 변경?/짝 불일치 18

> 본체가 동치라도 콜리가 변경이면 **명세 logic 은 재사용 가능 · 런타임 값은 콜리를 v3 로 바꿔야 game==mine**. 아래 콜리 목록이 재현 시 같이 갈아야 할 것.

| 구 | 신 | 함수 | 본체 판정 | 변경 콜리(구→신 이름 · 최종 판정) |
|---|---|---|---|---|
| `cc3080` | `ea6f40` | BattleSubPlan::score | 동치(r19: 동치(goal 4→3·vt | `12857f0→1643790` estimate_damage_to · 변경<br>`d59940→f749a0` calculate_action_score · 변경 |
| `cc6170` | `ea8610` | EpicPokeSubPlan::action_candidates | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경<br>`c8d570→df9db0` FUN_c8d570 · 콜리 짝 불일치<br>`c96830→e03070` FUN_c96830 · 콜리 짝 불일치<br>`c98290→e04ad0` FUN_c98290 · 콜리 짝 불일치<br>`eb82d0→eda920` check_kill_die_tick LocalKey::with(D · 변경 |
| `cc9740` | `eabbb0` | EpicPokeSubPlan::action_candidates_old | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경 |
| `ccbb10` | `f3a070` | LineSafeSubPlan::score | 동치(r19: 동치) | `d59940→f749a0` calculate_action_score · 변경<br>`e29b40→100c2f0` action_eval::evaluate_action · 변경 |
| `cce210` | `fd98c0` | SmallActionTrace::get_input | 동치(r21 w10: 디버그 로그만 ·  | `12857f0→1643790` estimate_damage_to · 변경<br>`cefd50→381ebcf` FUN_cefd50 · 콜리 짝 불일치<br>`d11bc0→e48880` get_input · 콜리 짝 불일치<br>`d13b30→e4abd0` get_input_cl · 콜리 짝 불일치<br>`d1d9c0→1643790` new_tower_avoid_v3 · 콜리 짝 불일치<br>`d1d9c0→e60c80` new_tower_avoid_v3 · 콜리 짝 불일치<br>`d1dc60→33a50` new_unnecessary_tower_avoid · 콜리 짝 불일치 |
| `ccfbc0` | `eb4880` | calculate_serpen_action_score | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `cd05f0` | `eb52b0` | base_battle_action | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경 |
| `d27d50` | `ec62f0` | PassiveLinePlan::v46_stage2 | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `d377a0` | `de3e80` | build_minion_wave_snapshot | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `d3ab40` | `de7220` | utils::can1v1win | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `d3dcc0` | `e76100` | objective_defense_role | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `d57540` | `f72620` | interaction_score | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경<br>`d5bbf0→f77820` calculate_interaction_action_score · 변경<br>`eb82d0→eda920` check_kill_die_tick LocalKey::with(D · 변경 |
| `d5ba80` | `f776b0` | calculate_jungle_action_score | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `d676c0` | `fe3fd0` | v16_gambler_ult_cc_bonus | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경<br>`d433f0→fd47a0` ? · 미지 |
| `d69f80` | `fe6840` | v17_runaway_counterattack_bonus | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경<br>`d433f0→fd47a0` ? · 미지 |
| `d84db0` | `ff5ec0` | position_eval_at | 동치(mig060_same 확정: 동일) | `d851d0→ff62e0` position_eval_at_uncached · 변경 |
| `d8ca70` | `ffdda0` | position_risk_all_zero_near | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `d8eff0` | `1000320` | calculate_score_parameter | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경<br>`d70900→e7c5b0` FUN_d70900 · 콜리 짝 불일치<br>`d709f0→e7c6a0` FUN_d709f0 · 콜리 짝 불일치<br>`d70ae0→e7c790` FUN_d70ae0 · 콜리 짝 불일치<br>`d70bd0→e7c880` FUN_d70bd0 · 콜리 짝 불일치 |
| `d98420` | `1009770` | v3_lethal_tower_position | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `d98ac0` | `1009e10` | v47_tower_focus_position_dangerous | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경<br>`d96d00→1008030` tower_discipline.rs:533 LocalKey::wi · 변경? |
| `d99030` | `100a380` | v22_lane_tower_pressure_attack_allowed | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경<br>`d96d00→1008030` tower_discipline.rs:533 LocalKey::wi · 변경? |
| `d99f60` | `100b2b0` | v30_line_champion_action_tower_aggro_ris | 동치(r19: 동치) | `12857f0→1643790` estimate_damage_to · 변경 |
| `d9bce0` | `e87300` | steal::evaluate_steal_for_target | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `db6ff0` | `ec3060` | SmallActionAround::get_input | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경 |
| `dbbd60` | `ed4350` | SmallActionAroundRegion::get_input | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `dc0800` | `ece1d0` | SmallActionAroundPosition::get_input | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `dc3240` | `fddcd0` | SmallActionRunAway::get_input | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경<br>`ceddd0→e2e5a0` get_input_cl · 콜리 짝 불일치<br>`d0dae0→e50aa0` get_input · 콜리 짝 불일치 |
| `dd73b0` | `ff2450` | SerpenStanceData::update_plan | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경 |
| `de1ee0` | `ff4940` | EpicStanceData::update_plan | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경<br>`de3d90→137cbc0` check_epic_kill_time_with_hp · 콜리 짝 불일치 |
| `de3630` | `100bb90` | get_die_tick_player | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `de3d90` | `100b8d0` | check_epic_kill_time_with_hp | 동치(mig060_same 확정: 오프셋 | `12857f0→1643790` estimate_damage_to · 변경 |
| `e04400` | `e85460` | v57_summon_command_score | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경<br>`ded690→e7abf0` ? · 미지 |
| `e25030` | `e93e00` | SmallActionLaneMinionPosition::target_sc | 동치(r21 w11: 인라인만) | `12857f0→1643790` estimate_damage_to · 변경 |
| `e281f0` | `e733f0` | line_champion_trade_allowance | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `e288c0` | `e73ac0` | line_projected_punish_damage_at | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `e4aec0` | `d56570` | v2_obj_restore_safe | ✅동치(r20) | `eb82d0→eda920` check_kill_die_tick LocalKey::with(D · 변경 |
| `e4b5d0` | `d582a0` | v3_fall_back_to_passive | 동치(mig060_same 확정: 동일) | `caf9f0→ec2b40` BigPlan::sub_plan (host) · 변경?<br>`e46bc0→d38180` passive_plan · 변경 |
| `e595b0` | `d5b410` | handle_chat | 동치(mig060_same 확정: 동일) | `e59b20→d5b980` handle_chat_inner · 변경 |
| `e5c1f0` | `d5ebb0` | single_try_engage | ✅동치(r20) | `d52030→f9bfe0` update · 변경?<br>`d60920→fae830` single_tower_dive_is_viable · 변경?<br>`dff080→ddd7e0` base_sub_goal · 변경 |
| `e657a0` | `d6bf10` | v2_response_retreat_stance | ✅동치(r20) | `ca0240→e0cb10` FUN_ca0240 · 콜리 짝 불일치<br>`eb82d0→eda920` check_kill_die_tick LocalKey::with(D · 변경 |
| `e7caf0` | `f60480` | SerpenPokeSubPlan::action_candidates | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경<br>`eb82d0→eda920` check_kill_die_tick LocalKey::with(D · 변경 |
| `e80070` | `f63a10` | SerpenPokeSubPlan::action_candidates_old | 동치(mig060_same 확정: 이동만 | `12857f0→1643790` estimate_damage_to · 변경 |
| `e83080` | `f66a30` | AttackNexusSubPlan::score | ✅동치(r20) | `d59940→f749a0` calculate_action_score · 변경 |
| `e8a100` | `f33fa0` | unsafe_v19_non_champion_walkup | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경<br>`eb82d0→eda920` check_kill_die_tick LocalKey::with(D · 변경 |
| `e8b200` | `f352d0` | LineDefenseSubPlan::score | 동치(mig060_same 확정: 이동만 | `d59940→f749a0` calculate_action_score · 변경<br>`e29b40→100c2f0` action_eval::evaluate_action · 변경 |
| `e8e160` | `f3b9e0` | AgentVerHamster::upgrade_item | 동치(mig060_same 확정: 동일) | `e8fd70→f3d660` AgentVerHamster::item_v26 · 변경 |
| `e8e560` | `f3bde0` | AgentVerHamster::update_small_action | ✅동치(r20) | `e65b10→d6c2b0` LegacyPlanHandler::get_small_action · 변경 |
| `e8fc80` | `f3d570` | AgentVerHamster::buy_item | 동치(mig060_same 확정: 동일) | `e7b640→f27ea0` buy_item · 변경<br>`e8fd70→f3d660` AgentVerHamster::item_v26 · 변경 |
| `e928f0` | `f40930` | DefenseNexusSubPlan::action_candidates | ✅동치(r20) | `12857f0→1643790` estimate_damage_to · 변경<br>`eb82d0→eda920` check_kill_die_tick LocalKey::with(D · 변경 |
| `e95ae0` | `f43b10` | DefenseNexusSubPlan::score | ✅동치(r20) | `d59940→f749a0` calculate_action_score · 변경 |
| `eb5840` | `eae650` | LineWaitSubPlan::score | 동치(r19: 동치) | `d59940→f749a0` calculate_action_score · 변경<br>`e29b40→100c2f0` action_eval::evaluate_action · 변경 |
| `eb5dd0` | `ed5fd0` | expected_dps | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `eb6100` | `ed6300` | battle_action | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `eb9570` | `ed8f40` | battle_check_with_list | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `ec81e0` | `f86640` | calculate_epic_action_score | 동치(mig060_same 확정: 동일) | `12857f0→1643790` estimate_damage_to · 변경 |
| `cbbdb0` | `e9fe00` | BattleSubPlan::action_candidates | ✅동치(+필드/게이트/텔레메트리)(r20 | `12a07d0→db1eb0` ? · 미지<br>`c89a90→e0daa0` v48_cast_beams (LocalKey::with 클로저) · 콜리 짝 불일치<br>`c993e0→e060b0` update_v32 · 변경? |
| `cc20a0` | `ea6240` | BattleSubPlan::calculate_score_parameter | ✅동치(r20) | `d74400→e70d80` max_poke_range · 변경? |
| `d31f20` | `dde530` | get_input_target | 동치(mig060_same 확정: 동일) | `12a0c80→1660f50` ? · 미지 |
| `d3a3a0` | `de6a80` | line_backfight_support_focus | 동치(mig060_same 확정: 동일) | `d41600→dec130` line_backfight_support_focus::{closu · 변경?<br>`e2e560→db6630` line_backfight_support_focus min_by_ · 변경? |
| `d97300` | `1008640` | can_tower_focused | 동치(mig060_same 확정: 동일) | `d70620→e7c2d0` can_tower_focused 의 Chain<Chain<…>>  · 콜리 짝 불일치 |
| `d98740` | `1009a90` | can_tower_focused_when_attack | 동치(mig060_same 확정: 동일) | `d70710→e7c3c0` FUN_d70710 · 콜리 짝 불일치 |
| `dc27a0` | `ed0240` | SmallActionAroundBush::new_with_out_line | 동치(mig060_same 확정: 동일) | `d7a080→e8a070` FUN_d7a080 · 콜리 짝 불일치 |
| `dd9f30` | `f10c60` | TeamPlan::can_near_enemies_range | 동치(r21 w11: 본체 동치 · la | `c99fe0→e066b0` TeamPlan::can_near_enemies_range · 변경? |
| `df1f50` | `fa8570` | next_plan | ✅동치(+필드/게이트/텔레메트리)(r20 | `dfb5d0→dd8b60` FUN_dfb5d0 · 변경? |
| `e01450` | `e82410` | v55_mark_value | 동치(mig060_same 확정: 동일) | `ded690→e7abf0` ? · 미지 |
| `e019d0` | `e82990` | v55_seal_value | 동치(mig060_same 확정: 동일) | `ded690→e7abf0` ? · 미지 |
| `e02bc0` | `e83ba0` | v54_aoe_ally_heal_value | 동치(mig060_same 확정: 동일) | `ded690→e7abf0` ? · 미지 |
| `e03360` | `e84340` | noncombat_steroid_window | ✅동치(r20) | `d75cb0→e7ff60` noncombat_steroid_window::{클로저}(추정) · 콜리 짝 불일치<br>`def0a0→e7ff60` noncombat_steroid_window_cl · 콜리 짝 불일치 |
| `e23750` | `e5da70` | cast::SmallActionUlt::is_end | ✅동치(r20) | `e266e0→e95dc0` SmallActionLaneMinionPosition::has_c · 콜리 짝 불일치 |
| `e23fa0` | `e5e0e0` | SmallActionPlay::get_input | 동치(r19: 동치(AroundHide  | `cce210→ec3060` SmallActionTrace::get_input · 콜리 짝 불일치<br>`d845e0→ed4350` SmallActionSkill::get_input · 콜리 짝 불일치<br>`d84800→ddd690` SmallActionSkill2::get_input · 콜리 짝 불일치<br>`d84a30→eccaf0` SmallActionUlt::get_input · 콜리 짝 불일치<br>`d84c60→e95f70` abstract_input::attack 래퍼(position_e · 콜리 짝 불일치<br>`db6ff0→ddd460` SmallActionAround::get_input · 콜리 짝 불일치<br>`db6ff0→fdb210` SmallActionAround::get_input · 콜리 짝 불일치<br>`dbbd60→ddd010` SmallActionAroundRegion::get_input · 콜리 짝 불일치<br>`dbd260→fd98c0` SmallActionRecall::get_input · 콜리 짝 불일치<br>`dc2070→ed0400` SmallActionAroundPositionBush::get_i · 콜리 짝 불일치<br>`dc2960→ecfa40` SmallActionAroundBush::get_input · 콜리 짝 불일치 |
| `e7a8c0` | `f27140` | upgrade_item | ✅동치(r20) | `105fae0→16a8100` ? · 미지 |
| `e83390` | `f2caf0` | LineDefenseSubPlan::action_candidates | ✅동치(+필드/게이트/텔레메트리)(r20 | `c91770→dfddb0` FUN_c91770 · 변경?<br>`d73c30→e706b0` FUN_d73c30 · 변경?<br>`d96d00→1008030` tower_discipline.rs:533 LocalKey::wi · 변경? |
| `ec8ba0` | `f87600` | v3_epicops_defer_serpen | ✅동치(r20) | `d666c0→1012f30` v3_serpen_contest_clear_win · 변경? |