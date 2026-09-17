# apply060 STATUS — logic_060 병합 현황

병합 27 · 형식 오류 0 · 미작성(변경 중) 46

| 구 | 신 | 함수 | confidence | logic_060 길이 |
|---|---|---|---|---|
| `ca6700` | `e13c10` | get_small_action_score_closure | A(클로저 본체) / B(f71610 요약 — r22 a 의존). | 4893 |
| `cb1ce0` | `e986c0` | SerpenHuntSubPlan::action_candidates | A(신규 게이트) / B(나머지 = RE 동치 추정 + 태그·슬롯 기계 치환 · 미확인 오프셋 5종). | 28933 |
| `d3b2a0` | `eed7f0` | v46_flee_gate_check | A — 변경 3지점 전부 디컴에서 직접 확인. | 5682 |
| `d40b20` | `fafe50` | best_jungle_goal | B — 게이트·A·C′·레거시·B 골격은 디컴/디스어셈 확인, B 세부(faf2a0 산식·좌표 방향)는 r2 | 9848 |
| `d639f0` | `1010060` | check_serpen_giveup | A — 변경 한 줄(판정식)을 디스어셈 수준으로 확인, 나머지 본문은 디컴에서 상수·구조 일치 확인. | 4312 |
| `db8e60` | `fa9d60` | LineGankerPlan::make_gank_battle | A — 전 분기 디컴 확인. | 3000 |
| `dbd260` | `fdb210` | SmallActionRecall::get_input | A(v3 블록) / B(나머지 본체 = 0.5.8 명세 + RE 동치 판정 의존). | 13321 |
| `dc2070` | `ecfa40` | SmallActionAroundPositionBush::get_input | A — 본체·헬퍼·클로저 판정 지점 모두 대조. | 2955 |
| `dce220` | `ef4690` | v3_epicops_buff_window | A — 신설 블록 전체를 명령 단위로 옮겼고 신규 콜리 2개(f1e3c0·efa190)의 출력 계약을 디컴으 | 5323 |
| `de0770` | `f18f90` | TeamPlan::update | A(레거시 본체·에필로그) / C(프롤로그 감사 블록 — 요약만). | 9368 |
| `dff080` | `ddd7e0` | base_sub_goal | A — 함수 전체를 0.6.0 디컴으로 대조했고 변경점이 꼬리 한 블록에 국한. | 3081 |
| `dffa10` | `e80960` | buff_value_v54 | A(§9)/B(전체) — 변경 항목은 asm 수준 확정. 본문 나머지는 동치 판정 인용. | 10085 |
| `e05e70` | `ee4e70` | resolve_join_stake | A(변경 4건 · ABI) / B(콜리 내부 동치 인용). | 5740 |
| `e06df0` | `ee5e80` | resolve_fight_stake | A — 본체 전 분기·오프셋 디컴 확인. ee3c80 내부 산식만 B. | 4091 |
| `e07430` | `ee6d40` | tower_dive_is_viable | A — 변경점 2건(클로저·pen) 과 흐름 전부 디컴 확인. | 5882 |
| `e0b030` | `eebb40` | resolve_fight_stake_roster | A — 전 분기 디컴 확인. | 2586 |
| `e0b730` | `eec620` | v25_scoped_battle_objective | A — 전 분기 디컴 확인. | 1998 |
| `e29b40` | `100c2f0` | action_eval::evaluate_action | A(게이트·선택·reach·th) / B(감쇠식 세부·합산 순서) — 핵심 상수·오프셋은 디컴 확인, 산식  | 6607 |
| `e49a50` | `d51a70` | LegacyPlanHandler::update_on_dead | A — 본체 전 구간 디컴+디스어셈 대조. | 5661 |
| `e4b070` | `d57b00` | v2_apply_assign_commit | A — 본체 전량 디스어셈 대조 · 헬퍼는 r22 c 의존(B). | 6495 |
| `e5ca10` | `d5f3c0` | try_engage | A(본체) / B(ee8090·필드 의미) — 본체는 디스어셈까지 전량 대조, 신규 필드의 의미는 미상. | 5306 |
| `e5d300` | `d5ff20` | try_engage_dive | A — 변경점 5줄 전부 디컴에서 직접 확인. 남은 불확실은 screening 플래그의 콜리 내부 의미(이  | 2678 |
| `e8d6f0` | `f3adb0` | AgentVerHamster::update_state | B — 본 함수의 변경점 3건은 전부 디컴 확인. 단 d52c00 이 LPH 판단 상태를 바꾸는지 미확정이라 | 6312 |
| `e8fd70` | `f3d660` | AgentVerHamster::item_v26 | A — 변경 한 줄(임계)과 오프셋 전부 디컴 확인. | 2666 |
| `e900b0` | `f3da50` | get_input | A(v3 폴백·prelude·+0x5f0·오프셋 표) / B(StayEvent·freeze 세부 = 0.5. | 20661 |
| `eaeda0` | `f6bda0` | EpicHuntSubPlan::action_candidates | A(신설 분기) / B(나머지 = RE 동치 판정 + §A 규칙 기계 치환 · 미확인 오프셋 2종). | 29345 |
| `ec9de0` | `f88880` | wave_priority_clearer_position | A — 본체·클로저 둘 다 디컴 대조. | 2121 |

## 미작성(변경 판정인데 logic_060 없음)

- `e01c40` defensive_crisis (⚠변경·한 줄(r20))
- `defa20` is_end (⚠변경·한 줄(r20))
- `db90f0` update (⚠변경·한 줄(r20))
- `e7b640` buy_item (⚠변경·한 줄(r20))
- `e04f50` fight_participants (⚠변경·다건(r20))
- `d2c5d0` PassiveLinePlan::sub_plan (⚠변경·다건(r20))
- `d40f10` evaluate_gank_opportunity_with_score (⚠변경·한 줄(r20))
- `dd6b40` v24_objective_setup_lane_pressure_ready (⚠변경·한 줄(r20))
- `e7acd0` should_recall_to_shop (⚠변경·한 줄(r20))
- `df0a90` SerpenHuntAndPokePlan::is_end (⚠변경·다건(r20))
- `dccc60` GoalData::update (⚠변경·한 줄(r20))
- `de92d0` epic_passive_plan (변경·심층(r21 §D: w9 §3))
- `defcd0` EpicHuntAndPokePlan::sub_plan (변경·심층(r21 §D: w9 §5))
- `dfe2a0` FightSituation::build (⚠변경·다건(r20))
- `e6b800` check_kill (변경·심층(r21 §D: w1 §2))
- `dd5db0` TeamPlan::v24_objective_setup_should_check_camp (⚠변경·한 줄(r20))
- `d2e500` PassiveJunglePlan::sub_plan (변경·심층(r21 §D: w3 §2))
- `d65620` serpen_passive_plan (변경·심층(r21 §D: w9 §2))
- `df0e90` SerpenHuntAndPokePlan::sub_plan (변경·심층(r21 §D: w9 §4))
- `d9ac10` steal::should_steal_now (⚠변경·한 줄(r20))
- `dfb840` BattlePlan::update (변경·심층(r21 §D: w2 §1))
- `e59b20` handle_chat_inner (변경·심층(r21 §D: w7 §3))
- `e46bc0` passive_plan (변경·심층(r21 §D: w6 §1))
- `dcee40` TeamPlan::handle_none_or_gank_objective (변경·심층(r21 §D: w7 §2))
- `dda220` TeamPlan::update_objective_after_steal (변경·심층(r21 §D: w7 §1))
- `df36e0` BattlePlan::update_v32 (변경·심층(r21 §D: w2 §0·§2))
- `e5d5d0` LegacyPlanHandler::handle_interact_battle (변경·심층(r21 §D: w5 §4))
- `e4c5c0` LegacyPlanHandler::update (변경·심층(r21 §D: w4 §3~§5))
- `dc2960` SmallActionAroundBush::get_input (변경(r19: ★로직 변경: path_finder 이름)
- `d851d0` position_eval_at_uncached (⚠변경·부분규명(전담)(r20))
- `d5bbf0` calculate_interaction_action_score (변경·심층(r21 §D: w10 §3))
- `d59940` calculate_action_score (변경·심층(r21 §D: w10 §1))
- `cb03b0` EpicCheckSubPlan::action_candidates (변경·심층(r21 §D: w10 §2))
- `e8b5e0` SerpenCheckSubPlan::action_candidates (⚠변경·다건(r20))
- `cc4260` JungleSubPlan::action_candidates (변경·심층(r21 §D: w3 §1))
- `cb7540` HideSubPlan::action_candidates (변경·심층(r21 §D: w8 §B))
- `db9430` LineGankerPlan::next_plan (변경·심층(r21 §D: w8 §A))
- `d2f180` PassiveJunglePlan::next_plan (변경·심층(r21 §D: w3 §3~4))
- `e65b10` LegacyPlanHandler::get_small_action (변경·심층(r21 §D: w6 §2))
- `d28800` PassiveLinePlan::update (⚠변경·다건(r20))
- `e083c0` resolve_fight_uncached (⚠변경·한 줄(r20))
- `eba9b0` fight_check::check_kill_die_tick_uncached (변경·심층(r21 §D: w1 §1))
- `d26900` PassiveLinePlan::v46_stage1 (⚠변경·한 줄(r20))
- `e8aeb0` LineDefenseSubPlan::calculate_score_parameter_value (⚠변경·한 줄(r20))
- `e388c0` SerpenCheckSubPlan::score (⚠변경·다건(r20))
- `e7b9f0` end_check (⚠변경·한 줄(r20))