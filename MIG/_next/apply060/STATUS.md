# apply060 STATUS — logic_060 병합 현황

병합 56 · 형식 오류 0 · 미작성(변경 중) 17

| 구 | 신 | 함수 | confidence | logic_060 길이 |
|---|---|---|---|---|
| `ca6700` | `e13c10` | get_small_action_score_closure | A(클로저 본체) / B(f71610 요약 — r22 a 의존). | 4893 |
| `cb1ce0` | `e986c0` | SerpenHuntSubPlan::action_candidates | A(신규 게이트) / B(나머지 = RE 동치 추정 + 태그·슬롯 기계 치환 · 미확인 오프셋 5종). | 28933 |
| `d26900` | `ec4c20` | PassiveLinePlan::v46_stage1 | A — 변경 지점·시그니처·오프셋 전부 디컴/호출부 대조. 특성 플래그 정체만 미상. | 6972 |
| `d28800` | `ec7180` | PassiveLinePlan::update | B+ — 변경 5건(undying · Gank 게이트 ×2 · 특성 bound · 배치 J 래치 3) 은 a | 25175 |
| `d2c5d0` | `ecb2b0` | PassiveLinePlan::sub_plan | A — 핵심 6건 전부 0.6.0 디컴/asm 으로 확인 · RE 대비 정정 1건(갱크 게이트 OR) 반영. | 9454 |
| `d2e500` | `fa3980` | PassiveJunglePlan::sub_plan | A- — 변경 지점 전부 asm 로 확인, 불변 구간(L154~199)은 RE 판정 의존. | 5335 |
| `d3b2a0` | `eed7f0` | v46_flee_gate_check | A — 변경 3지점 전부 디컴에서 직접 확인. | 5682 |
| `d40b20` | `fafe50` | best_jungle_goal | B — 게이트·A·C′·레거시·B 골격은 디컴/디스어셈 확인, B 세부(faf2a0 산식·좌표 방향)는 r2 | 9848 |
| `d40f10` | `fb1510` | evaluate_gank_opportunity_with_score | A — 본체·헬퍼 전문 디컴 대조. 유일한 「변경」 주장(/10→/20)이 반증돼 실질 동치(헬퍼 아웃라인+ | 3525 |
| `d59940` | `f749a0` | calculate_action_score | A — 변경 5지점 + 신규 조기반환 전부 디컴에서 확인. de5780/ff1fc0 내부만 RE 의존. | 9447 |
| `d639f0` | `1010060` | check_serpen_giveup | A — 변경 한 줄(판정식)을 디스어셈 수준으로 확인, 나머지 본문은 디컴에서 상수·구조 일치 확인. | 4312 |
| `d65620` | `1011d20` | serpen_passive_plan | A — 디컴이 RE w9 §2 등가 Rust 와 일치. | 4899 |
| `d851d0` | `ff62e0` | position_eval_at_uncached | B — 신규 항은 디컴으로 정정·확정했으나 6000명령 본체의 줄단위 대조는 미완(RE §5 「부분」 판정  | 34039 |
| `d9ac10` | `e85f00` | steal::should_steal_now | A — 양 버전 전문 디컴 대조, 차이 = 4 술어뿐. | 4606 |
| `db8e60` | `fa9d60` | LineGankerPlan::make_gank_battle | A — 전 분기 디컴 확인. | 3000 |
| `db90f0` | `faa440` | update | A — 소형 함수 전문 디컴이 RE 배치 G §3 과 줄 단위로 일치. | 3873 |
| `dbd260` | `fdb210` | SmallActionRecall::get_input | A(v3 블록) / B(나머지 본체 = 0.5.8 명세 + RE 동치 판정 의존). | 13321 |
| `dc2070` | `ecfa40` | SmallActionAroundPositionBush::get_input | A — 본체·헬퍼·클로저 판정 지점 모두 대조. | 2955 |
| `dccc60` | `ff11f0` | GoalData::update | A — 신규 블록 전문 디컴 · 나머지 양버전 대조. | 3988 |
| `dce220` | `ef4690` | v3_epicops_buff_window | A — 신설 블록 전체를 명령 단위로 옮겼고 신규 콜리 2개(f1e3c0·efa190)의 출력 계약을 디컴으 | 5323 |
| `dd5db0` | `f0cfe0` | TeamPlan::v24_objective_setup_should_check_camp | A — 변경 1건(L91) 디컴 확인 · 본문 동치 양 버전 디컴 대조. | 4654 |
| `dd6b40` | `f0dc20` | v24_objective_setup_lane_pressure_ready | A — 전문 디컴이 RE 배치 G §5 와 일치, 변경은 진입 게이트 1곳. | 4113 |
| `de0770` | `f18f90` | TeamPlan::update | A(레거시 본체·에필로그) / C(프롤로그 감사 블록 — 요약만). | 9368 |
| `de92d0` | `fb5ff0` | epic_passive_plan | A — 전문 디컴이 RE w9 §3 과 일치하며 f904b0 의미를 추가 확정. | 5105 |
| `defa20` | `fd7b40` | is_end | A — 변경 2건 디컴 확인 · 본문 동치 확인. | 3996 |
| `defcd0` | `fd7f70` | EpicHuntAndPokePlan::sub_plan | A- — 변경 지점 전부 asm 로 확인. | 6528 |
| `df0a90` | `dcb890` | SerpenHuntAndPokePlan::is_end | A — 변경 2건 양버전 디컴 대조 · 꼬리 동치 확인. | 3100 |
| `df0e90` | `dcbe00` | SerpenHuntAndPokePlan::sub_plan | A- — 변경 지점 전부 asm 로 확인, 불변 D/E 는 상수 매칭 + RE 판정. | 5930 |
| `dfe2a0` | `ed5720` | FightSituation::build | A — 삭제 5필드·유지 4항 전부 0.6.0 디컴으로 확인. | 3393 |
| `dff080` | `ddd7e0` | base_sub_goal | A — 함수 전체를 0.6.0 디컴으로 대조했고 변경점이 꼬리 한 블록에 국한. | 3081 |
| `dffa10` | `e80960` | buff_value_v54 | A(§9)/B(전체) — 변경 항목은 asm 수준 확정. 본문 나머지는 동치 판정 인용. | 10085 |
| `e01c40` | `e82c00` | defensive_crisis | A — 변경 1건(extra_tick) 및 ABI 변경 디컴 확인 · 나머지 동치. | 3436 |
| `e04f50` | `ee3e40` | fight_participants | B+ — 본체·신규 술어 2단 전문 대조. ee0800 의 최하단 콜리(ed5fd0 등)와 bb+0x4d8  | 4905 |
| `e05e70` | `ee4e70` | resolve_join_stake | A(변경 4건 · ABI) / B(콜리 내부 동치 인용). | 5740 |
| `e06df0` | `ee5e80` | resolve_fight_stake | A — 본체 전 분기·오프셋 디컴 확인. ee3c80 내부 산식만 B. | 4091 |
| `e07430` | `ee6d40` | tower_dive_is_viable | A — 변경점 2건(클로저·pen) 과 흐름 전부 디컴 확인. | 5882 |
| `e083c0` | `ee8450` | resolve_fight_uncached | B — 변경점(bias 임계) asm 확인 · 나머지 본문은 RE 의 상수/콜리 동치 판정에 의존(명령 단위 | 9720 |
| `e0b030` | `eebb40` | resolve_fight_stake_roster | A — 전 분기 디컴 확인. | 2586 |
| `e0b730` | `eec620` | v25_scoped_battle_objective | A — 전 분기 디컴 확인. | 1998 |
| `e29b40` | `100c2f0` | action_eval::evaluate_action | A(게이트·선택·reach·th) / B(감쇠식 세부·합산 순서) — 핵심 상수·오프셋은 디컴 확인, 산식  | 6607 |
| `e388c0` | `ed3dd0` | SerpenCheckSubPlan::score | A(arm·후처리) / B(f71fc0 산식 요약 — 콜리 3개 내부 미대조). | 4282 |
| `e49a50` | `d51a70` | LegacyPlanHandler::update_on_dead | A — 본체 전 구간 디컴+디스어셈 대조. | 5661 |
| `e4b070` | `d57b00` | v2_apply_assign_commit | A — 본체 전량 디스어셈 대조 · 헬퍼는 r22 c 의존(B). | 6495 |
| `e5ca10` | `d5f3c0` | try_engage | A(본체) / B(ee8090·필드 의미) — 본체는 디스어셈까지 전량 대조, 신규 필드의 의미는 미상. | 5306 |
| `e5d300` | `d5ff20` | try_engage_dive | A — 변경점 5줄 전부 디컴에서 직접 확인. 남은 불확실은 screening 플래그의 콜리 내부 의미(이  | 2678 |
| `e6b800` | `d72e70` | check_kill | A — 본체 디컴 전량 대조로 분기·상수·오프셋 전부 확인. 신규 콜리 3종(eeb5b0·de9900·e0f | 8357 |
| `e7acd0` | `f27590` | should_recall_to_shop | A — 변경 한 줄 양버전 디컴 대조. | 3098 |
| `e7b640` | `f27ea0` | buy_item | A — 변경 지점(선검사·len>2 삭제) 디컴 직접 확인. switch 본체만 0.5.8 인용. | 3262 |
| `e7b9f0` | `f28320` | end_check | A — 프렐류드 디컴 확인 · 본문 3전략 디컴 대조(상수·표·오프셋 일치). | 5756 |
| `e8aeb0` | `f34d10` | LineDefenseSubPlan::calculate_score_parameter_value | A — 전 항 0.6.0 디컴으로 확인. | 2075 |
| `e8b5e0` | `e9de70` | SerpenCheckSubPlan::action_candidates | A — 변경 블록 전문 디컴 · 꼬리 capstone 대조 · 0.5.8 대조. | 6220 |
| `e8d6f0` | `f3adb0` | AgentVerHamster::update_state | B — 본 함수의 변경점 3건은 전부 디컴 확인. 단 d52c00 이 LPH 판단 상태를 바꾸는지 미확정이라 | 6312 |
| `e8fd70` | `f3d660` | AgentVerHamster::item_v26 | A — 변경 한 줄(임계)과 오프셋 전부 디컴 확인. | 2666 |
| `e900b0` | `f3da50` | get_input | A(v3 폴백·prelude·+0x5f0·오프셋 표) / B(StayEvent·freeze 세부 = 0.5. | 20661 |
| `eaeda0` | `f6bda0` | EpicHuntSubPlan::action_candidates | A(신설 분기) / B(나머지 = RE 동치 판정 + §A 규칙 기계 치환 · 미확인 오프셋 2종). | 29345 |
| `ec9de0` | `f88880` | wave_priority_clearer_position | A — 본체·클로저 둘 다 디컴 대조. | 2121 |

## 미작성(변경 판정인데 logic_060 없음)

- `dfb840` BattlePlan::update (변경·심층(r21 §D: w2 §1))
- `e59b20` handle_chat_inner (변경·심층(r21 §D: w7 §3))
- `e46bc0` passive_plan (변경·심층(r21 §D: w6 §1))
- `dcee40` TeamPlan::handle_none_or_gank_objective (변경·심층(r21 §D: w7 §2))
- `dda220` TeamPlan::update_objective_after_steal (변경·심층(r21 §D: w7 §1))
- `df36e0` BattlePlan::update_v32 (변경·심층(r21 §D: w2 §0·§2))
- `e5d5d0` LegacyPlanHandler::handle_interact_battle (변경·심층(r21 §D: w5 §4))
- `e4c5c0` LegacyPlanHandler::update (변경·심층(r21 §D: w4 §3~§5))
- `dc2960` SmallActionAroundBush::get_input (변경(r19: ★로직 변경: path_finder 이름)
- `d5bbf0` calculate_interaction_action_score (변경·심층(r21 §D: w10 §3))
- `cb03b0` EpicCheckSubPlan::action_candidates (변경·심층(r21 §D: w10 §2))
- `cc4260` JungleSubPlan::action_candidates (변경·심층(r21 §D: w3 §1))
- `cb7540` HideSubPlan::action_candidates (변경·심층(r21 §D: w8 §B))
- `db9430` LineGankerPlan::next_plan (변경·심층(r21 §D: w8 §A))
- `d2f180` PassiveJunglePlan::next_plan (변경·심층(r21 §D: w3 §3~4))
- `e65b10` LegacyPlanHandler::get_small_action (변경·심층(r21 §D: w6 §2))
- `eba9b0` fight_check::check_kill_die_tick_uncached (변경·심층(r21 §D: w1 §1))