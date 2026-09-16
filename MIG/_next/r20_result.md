# r20 — 변경 99 중 티어 2/3 84 디컴 대조 결과(0.5.8→0.6.0 · 09-17 · RE 배치 A~F)

판정: ✅동치 32 · ✅동치(+필드/게이트/텔레메트리) 6 · ⚠변경·한 줄 20 · ⚠변경·다건 16 · ⚠변경·부분규명(전담) 4 · ❗대폭(티어1 이관) 6

| 배치 | 구 | 신 | 함수 | 판정 | 요지 |
|---|---|---|---|---|---|
| A | `d3d560` | `e759a0` | i_am_chosen_defender | ✅동치 |  |
| A | `d2da10` | `e71d40` | DefenseNexusPlan::sub_plan | ✅동치 | SubPlan 태그 인코딩만 |
| A | `e4aec0` | `d56570` | v2_obj_restore_safe | ✅동치 | 콜리 ABI 만 |
| A | `e8a100` | `f33fa0` | unsafe_v19_non_champion_walkup | ✅동치 | 콜리 ABI 만 |
| A | `dd90c0` | `f0e5f0` | TeamPlan::update_steal | ✅동치(+필드/게이트/텔레메트리) | TeamPlan 오프셋·죽은 호출 1 |
| A | `e03360` | `e84340` | noncombat_steroid_window | ✅동치 |  |
| A | `e23750` | `e5da70` | cast::SmallActionUlt::is_end | ✅동치 | Ult arm |
| A | `d40f10` | `fb1510` | evaluate_gank_opportunity_with_score | ⚠변경·한 줄 | 노이즈 폭 /10→/20 |
| A | `e7b640` | `f27ea0` | buy_item | ⚠변경·한 줄 | 활성템 보유 >3 게이트 |
| A | `d26900` | `ec4c20` | PassiveLinePlan::v46_stage1 | ⚠변경·한 줄 | 특성 +0x49d/+0x49e 로 (c,b,a′) |
| A | `df0a90` | `dcb890` | SerpenHuntAndPokePlan::is_end | ⚠변경·다건 | TeamPlan 이중모드 게이트 |
| A | `d65620` | `1011d20` | serpen_passive_plan | ❗대폭(티어1 이관) | ObjContest CONTEST 분기 |
| A | `d59940` | `f749a0` | calculate_action_score | ❗대폭(티어1 이관) | parameter 장부·특성·return 25 |
| A | `cb03b0` | `e96780` | EpicCheckSubPlan::action_candidates | ❗대폭(티어1 이관) | +0xcc0 게이트·캠프 후보 |
| B | `d69f80` | `fe6840` | v17_runaway_counterattack_bonus | ✅동치 |  |
| B | `e657a0` | `d6bf10` | v2_response_retreat_stance | ✅동치 |  |
| B | `e7a8c0` | `f27140` | upgrade_item | ✅동치 |  |
| B | `d57540` | `f72620` | interaction_score | ✅동치 |  |
| B | `dd73b0` | `ff2450` | SerpenStanceData::update_plan | ✅동치 | dps 헬퍼 분해 |
| B | `cc5a10` | `eb28e0` | JungleSubPlan::score | ✅동치 | 디버그 로그만 |
| B | `cc20a0` | `ea6240` | BattleSubPlan::calculate_score_parameter_v | ✅동치 |  |
| B | `e01c40` | `fe7120` | defensive_crisis | ⚠변경·한 줄 | extra_tick 인자(v3) · 짝 e82c00 |
| B | `e7acd0` | `f27590` | should_recall_to_shop | ⚠변경·한 줄 | 활성템 >2→>3 |
| B | `defa20` | `fd7b40` | is_end | ⚠변경·한 줄 | TeamPlan 이중모드 |
| B | `e04f50` | `ee3e40` | fight_participants | ⚠변경·다건 | 포킹취약 술어 ee3c80·+0xcc1 |
| B | `d5bbf0` | `f77820` | calculate_interaction_action_score | ⚠변경·부분규명(전담) | v3 분기 5지점 |
| B | `cce210` | `fd98c0` | SmallActionTrace::get_input | ⚠변경·부분규명(전담) | v2 우물 회피/lethal |
| B | `defcd0` | `fd7f70` | EpicHuntAndPokePlan::sub_plan | ⚠변경·부분규명(전담) | ObjContest 반환·hp 51→50 |
| C | `e2a830` | `eb3470` | lane_minion_position_action | ✅동치 |  |
| C | `e95ae0` | `f43b10` | DefenseNexusSubPlan::score | ✅동치 |  |
| C | `ec8ba0` | `f87600` | v3_epicops_defer_serpen | ✅동치 |  |
| C | `e8e560` | `f3bde0` | AgentVerHamster::update_small_action | ✅동치 |  |
| C | `de5340` | `fb1b70` | check_epic_hunt | ✅동치 |  |
| C | `db6ff0` | `ec3060` | SmallActionAround::get_input | ✅동치 |  |
| C | `d8eff0` | `1000320` | calculate_score_parameter | ✅동치 |  |
| C | `de81b0` | `fb49b0` | check_epic_giveup | ✅동치(+필드/게이트/텔레메트리) | 게이트 필드 재편 |
| C | `dd26e0` | `f09860` | TeamPlan::v25_objective_posture | ✅동치(+필드/게이트/텔레메트리) | +0xcc7 게이트 |
| C | `dccc60` | `ff11f0` | GoalData::update | ⚠변경·한 줄 | v3 undying → heal_commit=false |
| C | `e8b5e0` | `e9de70` | SerpenCheckSubPlan::action_candidates | ⚠변경·다건 | +0xcc0 그룹 진입 게이트·표 2 |
| C | `d2e500` | `fa3980` | PassiveJunglePlan::sub_plan | ❗대폭(티어1 이관) | v3 역정글 매복 Hide |
| C | `d28800` | `ec7180` | PassiveLinePlan::update | ⚠변경·다건 | undying·특성 bound·bb Vec 갱크 |
| C | `d851d0` | `ff62e0` | position_eval_at_uncached | ⚠변경·부분규명(전담) | 종반 신규 항(중심점 이격) |
| D | `e7caf0` | `f60480` | SerpenPokeSubPlan::action_candidates | ✅동치 |  |
| D | `cc6170` | `ea8610` | EpicPokeSubPlan::action_candidates | ✅동치 |  |
| D | `e83390` | `f2caf0` | LineDefenseSubPlan::action_candidates | ✅동치(+필드/게이트/텔레메트리) | 텔레메트리만 |
| D | `df1f50` | `fa8570` | next_plan | ✅동치(+필드/게이트/텔레메트리) | gank_line 기록 |
| D | `e5d300` | `d5ff20` | try_engage_dive | ⚠변경·한 줄 | screening 플래그 |
| D | `e8fd70` | `f3d660` | AgentVerHamster::item_v26 | ⚠변경·한 줄 | 활성템 3→4 |
| D | `d639f0` | `1010060` | check_serpen_giveup | ⚠변경·한 줄 | 이중모드 판정식 |
| D | `dce220` | `ef4690` | v3_epicops_buff_window | ⚠변경·다건 | +0xcc5 계약 레코드 |
| D | `e8d6f0` | `f3adb0` | AgentVerHamster::update_state | ⚠변경·다건 | awareness 8번째 인자·TRAIT_AUD d52c00 |
| D | `dffa10` | `e80960` | buff_value_v54 | ⚠변경·다건 | v3 crisis undying 전체 0 |
| D | `e07430` | `ee6d40` | tower_dive_is_viable | ⚠변경·다건 | v3 클로저·판단력 패널티 |
| D | `eaeda0` | `f6bda0` | EpicHuntSubPlan::action_candidates | ⚠변경·한 줄 | +0xcc7 집결점 후보 |
| D | `df0e90` | `dcbe00` | SerpenHuntAndPokePlan::sub_plan | ❗대폭(티어1 이관) | ObjContest 반환 2·v3 귀환 |
| D | `de92d0` | `fb5ff0` | epic_passive_plan | ❗대폭(티어1 이관) | +0xcc7 전면 분기 |
| E | `e59190` | `d5aff0` | v50_fold_dive_episode | ✅동치 |  |
| E | `d9a220` | `100b570` | v22_current_line_non_champion_action_tower | ✅동치 |  |
| E | `d676c0` | `fe3fd0` | v16_gambler_ult_cc_bonus | ✅동치 |  |
| E | `de1ee0` | `ff4940` | EpicStanceData::update_plan | ✅동치 |  |
| E | `cd05f0` | `eb52b0` | base_battle_action | ✅동치 |  |
| E | `e5c1f0` | `d5ebb0` | single_try_engage | ✅동치 |  |
| E | `e8aeb0` | `f34d10` | LineDefenseSubPlan::calculate_score_parame | ⚠변경·한 줄 | 특성 가치·+0x1501 |
| E | `e388c0` | `ed3dd0` | SerpenCheckSubPlan::score | ⚠변경·다건 | SubPlan::score 후처리(교환비 −9999) |
| E | `e05e70` | `ee4e70` | resolve_join_stake | ⚠변경·다건 | committed/horizon 인자·+0xcc1 |
| E | `e7b9f0` | `f28320` | end_check | ⚠변경·한 줄 | v3 finish_race 프렐류드 |
| E | `dd5db0` | `f0cfe0` | TeamPlan::v24_objective_setup_should_check | ⚠변경·한 줄 | 이중모드 게이트 |
| E | `d2c5d0` | `ecb2b0` | PassiveLinePlan::sub_plan | ⚠변경·다건 | 6건(+0x117/+0x119·특성·bb Vec) |
| E | `dfe2a0` | `ed5720` | FightSituation::build | ⚠변경·다건 | 128→96B 5필드 삭제 |
| E | `e083c0` | `ee8450` | resolve_fight_uncached | ⚠변경·한 줄 | bias 인자(0 이면 동치) |
| F | `eb43a0` | `ead1c0` | LineWaitSubPlan::action_candidates | ✅동치 |  |
| F | `cba660` | `e5eb00` | StealSubPlan::action_candidates | ✅동치 |  |
| F | `e928f0` | `f40930` | DefenseNexusSubPlan::action_candidates | ✅동치 |  |
| F | `dc3240` | `fddcd0` | SmallActionRunAway::get_input | ✅동치 |  |
| F | `cbbdb0` | `e9fe00` | BattleSubPlan::action_candidates | ✅동치(+필드/게이트/텔레메트리) | 본체 동치·콜리 변경 |
| F | `ec9de0` | `f88880` | wave_priority_clearer_position | ⚠변경·한 줄 | exclude_jungler 인자 |
| F | `dc2070` | `ecfa40` | SmallActionAroundPositionBush::get_input | ⚠변경·다건 | region 2·7 차단 |
| F | `e5ca10` | `d5f3c0` | try_engage | ⚠변경·다건 | v3 게이트 2·update 2회 |
| F | `dff080` | `ddd7e0` | base_sub_goal | ⚠변경·한 줄 | v3 Response→Kiting |
| F | `e29b40` | `100c2f0` | action_eval::evaluate_action | ⚠변경·다건 | 특성 항 2 |
| F | `d3b2a0` | `eed7f0` | v46_flee_gate_check | ⚠변경·다건 | 특성 A/B·v3 *tps |
| F | `dbd260` | `fdb210` | SmallActionRecall::get_input | ⚠변경·한 줄 | v3 직접귀환 커밋 |
| F | `e900b0` | `f3da50` | get_input | ⚠변경·한 줄 | v3 우물 Move 폴백 |
| F | `cb1ce0` | `e986c0` | SerpenHuntSubPlan::action_candidates | ⚠변경·한 줄 | ObjContest 진입점 |

## 회계

- 동치(E+E+) 38 → 명세 유효 135 + 38 = **173** · 재명세 = 100 − 38 = **62**(티어1 15 + 이관 6 + 티어2/3 변경 40).
- 재명세 62 중 한 줄 반영 20 · 다건 16 · 부분규명(전담) 4 · 대폭(티어1) 6 + 티어1 원래 15 = 심층 21.

## 횡단 발견

- ★**`version ≥ 3` 게이트 신설**(에이전트 version = `AgentVerHamster+0x3608` · 값 2/3 정적 미확정 → 런타임 1회 확인 필요 · 3 이면 undying·역정글 매복·특성·finish_race·직접귀환·우물 Move 폴백 등 v3 분기 전부 활성). 명세의 「version=2 고정」 가정 재검토.
- ★**챔피언 특성 플래그 확정**: PlayerState `+0x49d Aggressive · +0x49e Defensive · +0x49f FightJoiner · +0x4a0 LaneIntervention`(1416dcdd0 이 traits Vec +0x4b0 순회). 소비: LineDefense 가치 · SubPlan::score 후처리(교환비 −9999) · evaluate_action(±항) · v46 bound(1416fbbc0) · PassiveLine style/Recall · TRAIT_AUD d52c00.
- **TeamPlan 이중모드**: +0x3e4 = serpen_state.mode · +0x404 = epic_state.mode(2 = 레거시 objective +0xcd5/+0xcd6/+0xcd7 사용) · 신규 게이트 +0xcc0/+0xcc1/+0xcc2/+0xcc5/+0xcc7 · 마스크 +0xca3/+0xca4/+0xca5 · 계약 레코드 +0xc0/+0xd0/+0xd8/+0xe0 · 카운터 +0xc30/+0xc38/+0xc70/+0xc78.
- **ABI 변경**: check_kill_die_tick eb82d0→eda920(rnd·debug 제거 · bool×3 · Option) · defensive_crisis(extra_tick) · resolve_join_stake(committed, horizon_sec) · resolve_fight_uncached/full(bias:i8) · wave_priority_clearer_position(exclude_jungler) · buff_value_v54(+2) · FightSituation 128→96B.
- **인코딩**: SubPlan 태그 u8 `idx+2` → u64 `0x8000…+idx`(0.5.8 17 · 0.6.0 18 · 14 AttackNexus 유지 · 17 ObjContest 추가) · BigPlan 0.6.0 Battle(idx 7) 니치 dataful(태그 0/1) 나머지 idx+2.
- **mig060 짝 정정**: defensive_crisis e01c40 ~~fe7120~~→e82c00 · EpicStance 콜리 c9c160→e080d0 · c9c2c0→e08230 · try_engage_dive 콜리 new_dive=ddc9f0.
- 배치 B 「ee0800 신규」 → 구 fight_dps ebe220 이동(배치 E 정정).

## 티어 1(심층 · 15 + 이관 6)

- `cb7540`→`f356b0` HideSubPlan::action_candidates 11547→13814B
- `e59b20`→`d5b980` handle_chat_inner 9416→7100B
- `e6b800`→`d72e70` check_kill 5680→8014B
- `e65b10`→`d6c2b0` LegacyPlanHandler::get_small_action 19492→23419B
- `d2f180`→`fa47a0` PassiveJunglePlan::next_plan 10479→14676B
- `db9430`→`faa7c0` LineGankerPlan::next_plan 9919→14470B
- `cc4260`→`eafb50` JungleSubPlan::action_candidates 5349→10734B
- `dfb840`→`dd8e70` BattlePlan::update 9320→15066B
- `dda220`→`f10f70` TeamPlan::update_objective_after_steal 24674→30871B
- `eba9b0`→`edb240` fight_check::check_kill_die_tick_uncached 4429→11569B
- `e46bc0`→`d38180` passive_plan 11615→20085B
- `dcee40`→`f024d0` TeamPlan::handle_none_or_gank_objective 14041→23078B
- `df36e0`→`dcce40` BattlePlan::update_v32 29389→42894B
- `e5d5d0`→`d606b0` LegacyPlanHandler::handle_interact_battle 32177→45755B
- `e4c5c0`→`d3d210` LegacyPlanHandler::update 42054→77103B
- `d65620`→`1011d20` serpen_passive_plan(이관 · ObjContest CONTEST 분기)
- `d59940`→`f749a0` calculate_action_score(이관 · parameter 장부·특성·return 25)
- `cb03b0`→`e96780` EpicCheckSubPlan::action_candidates(이관 · +0xcc0 게이트·캠프 후보)
- `d2e500`→`fa3980` PassiveJunglePlan::sub_plan(이관 · v3 역정글 매복 Hide)
- `df0e90`→`dcbe00` SerpenHuntAndPokePlan::sub_plan(이관 · ObjContest 반환 2·v3 귀환)
- `de92d0`→`fb5ff0` epic_passive_plan(이관 · +0xcc7 전면 분기)