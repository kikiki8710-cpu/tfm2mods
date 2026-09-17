# probe060 실경기 구간 증분(MARK 이후 · Bo3 3세트 즉시결과)

마크 이전 미발화 → 실경기에서 발화 0 · 여전히 미발화 22

## 실경기에서 처음 발화


## 여전히 미발화(배경 sim + 실경기 Bo3)

- [6] `e60420` sub_plan
- [9] `eed490` should_end_object_finish_kill_priority_battle
- [13] `d5ebb0` single_try_engage
- [28] `1012e50` serpen_giveup_chat_reason
- [41] `ef4ab0` TeamPlan::handle_epic_line_change
- [44] `fb49b0` check_epic_giveup
- [47] `fb34e0` check_epic_setup
- [51] `ef3da0` handle_press_epic
- [55] `e85d70` bush_distance_sq
- [75] `fb1b70` check_epic_hunt
- [79] `ede160` should_disengage_object_hunt
- [87] `1010060` check_serpen_giveup
- [89] `100f210` check_serpen_setup
- [92] `e87300` steal::evaluate_steal_for_target
- [131] `eacee0` EpicPokeSubPlan::score
- [165] `eccaf0` SmallActionPositioning::get_input
- [166] `eabbb0` EpicPokeSubPlan::action_candidates_old
- [169] `ed4350` SmallActionAroundRegion::get_input
- [191] `ea8610` EpicPokeSubPlan::action_candidates
- [202] `e5eb00` StealSubPlan::action_candidates
- [234] `e601b0` StealSubPlan::score
- [242] `fa8570` next_plan

## 실경기 증분 상위 30

| idx | rva | 함수 | 증분 | 누적 |
|---|---|---|---|---|
| 259 | `ed5fd0` | expected_dps | 95589920 | 392278692 |
| 128 | `1009770` | v3_lethal_tower_position | 56749750 | 230425608 |
| 19 | `1008640` | can_tower_focused | 49838280 | 205227638 |
| 179 | `ff5ec0` | position_eval_at | 43901446 | 183420925 |
| 200 | `f3da50` | get_input | 39210519 | 161504022 |
| 256 | `e944e0` | SmallActionLaneMinionPosition::push_candidat | 34232124 | 149966235 |
| 203 | `edb240` | fight_check::check_kill_die_tick_uncached | 27836894 | 114489585 |
| 172 | `ff62e0` | position_eval_at_uncached | 22439223 | 93707273 |
| 106 | `de6080` | nontarget_windup_perceived | 22097364 | 91664498 |
| 59 | `100b8d0` | check_epic_kill_time_with_hp | 14567804 | 57908330 |
| 247 | `f28320` | end_check | 13524322 | 55792765 |
| 111 | `e78a10` | base_attacking_minion_uncached | 12837227 | 50969106 |
| 48 | `f18f90` | TeamPlan::update | 12107010 | 47878129 |
| 68 | `ff11f0` | GoalData::update | 12107010 | 47878129 |
| 69 | `ff2450` | SerpenStanceData::update_plan | 12107010 | 47878129 |
| 70 | `ff4940` | EpicStanceData::update_plan | 12107010 | 47878129 |
| 214 | `e76aa0` | nexus_last_stand_uncached | 12107010 | 47878129 |
| 225 | `e78290` | nexus_final_stand_uncached | 12107010 | 47878129 |
| 3 | `e753e0` | handle_line_defense | 11314201 | 47170204 |
| 35 | `f87ea0` | is_wave_priority_start_line | 10812381 | 42360690 |
| 213 | `de6280` | precompute_champion_powers | 9485969 | 38953547 |
| 49 | `feefc0` | EntityPositioningCache::new | 8404985 | 34423930 |
| 53 | `eecea0` | is_unreasonable_tower_dive_enemy | 7782731 | 31519501 |
| 34 | `1009c20` | can_tower_focused_when_battle | 7697080 | 31173684 |
| 237 | `f3b9e0` | AgentVerHamster::upgrade_item | 7623948 | 29490151 |
| 239 | `f3d570` | AgentVerHamster::buy_item | 7594217 | 29364162 |
| 248 | `d51a70` | LegacyPlanHandler::update_on_dead | 6608851 | 25245448 |
| 258 | `f10c60` | TeamPlan::can_near_enemies_range | 6552993 | 26311920 |
| 134 | `e5e0e0` | SmallActionPlay::get_input | 5755032 | 23669800 |
| 221 | `f3d140` | AgentVerHamster::count_nearby_enemies | 5519312 | 22681391 |