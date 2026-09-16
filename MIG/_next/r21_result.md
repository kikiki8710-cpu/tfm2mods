# r21 — 티어1 심층 21 + 전담 4 + 미발견 12 디컴 대조 결과(0.5.8→0.6.0 · 09-17 · 웨이브 w1~w11)

정본 = 패치 대장 `spec_patch_060.md` §D(함수별 요지 + RE 절 포인터) · RE 원문 = `REPORT\tfm2_judge_verify\RE\2026-09-17_r21_심층_w{1..11}_*_원문.md`. 이 파일은 판정표·회계·잔여만.

## 판정표(§D 순서 · 함수 33 + 헬퍼 정체표)

| w | 구 | 신 | 함수 | 판정 | 요지 |
|---|---|---|---|---|---|
| 1 | `eba9b0` | `edb240` | check_kill_die_tick_uncached | ⚠변경·다건 | v2 = 구 본체 + bool 게이트 3 · **v3 = 신규 타임라인 알고리즘** · ABI 변경(&towers · Option<(x,y,t)>) |
| 1 | `cc4260` | `eafb50` | JungleSubPlan::action_candidates | ⚠변경·다건 | position_score 셀 중심 6인자 · v3 RunAway 조건 · v3 카정 move |
| 3 | `d2e500` | `fa3980` | PassiveJunglePlan::sub_plan | ⚠변경·다건 | v3 undying → 매복블록 · cj_ambush Some 이면 적 정글러 kill 시간 vs 내 도달 → Jungle/Hide |
| 3 | `d2f180` | `fa47a0` | PassiveJunglePlan::next_plan | ⚠변경·다건 | v3 매복 유지/해제 · 매복 시도 fa2670 · lead_action fa3350 |
| 2 | `dfb840` | `dd8e70` | BattlePlan::update | ⚠변경·다건 | 본체 동치(재번호·이중모드) · v3 에필로그 REBAR |
| 2 | `df36e0` | `dcce40` | BattlePlan::update_v32 | ❗대폭 | 신규 구역 8(플립플롭·TANKAVAIL·TWBIN·솔로듀얼…) |
| 5 | `e5d5d0` | `d606b0` | LPH::handle_interact_battle | ❗대폭 | 신규 프롤로그(latch·pending tb) · §C 합류 대폭 |
| 4 | `e4c5c0` | `d3d210` | LPH::update | ❗대폭 | 구 본체 오프셋만 · 신규 17.5KB 오케스트레이션(미독 6 구역) |
| 6 | `e46bc0` | `d38180` | passive_plan | ❗대폭 | 에고웨이브 · ⑧ cc7 폐기 → ambient · cc2 플래너 · 후처리 E |
| 6 | `e65b10` | `d6c2b0` | LPH::get_small_action | ⚠변경·다건 | 본체 동치 + v3 아군 궁 · 재번호 |
| 7 | `dda220` | `f10f70` | TeamPlan::update_objective_after_steal | ❗대폭 | V6 pre-pass/KILL · 씬 핸들러 I/J · phase 핸들러 12(5 미독) |
| 7 | `dcee40` | `f024d0` | TeamPlan::handle_none_or_gank_objective | ❗대폭 | V6 commit 블록 · 미독 f02ef6..f04f8f |
| 7 | `e59b20` | `d5b980` | handle_chat_inner | ⚠변경·다건 | d5d700 아웃라인(미독) · 무시 게이트 · Press/HideLine/Repair |
| 8 | `db9430` | `faa7c0` | LineGankerPlan::next_plan | ⚠변경·다건 | 3분기 · 내 라인 존 적 제외 · site 인자 |
| 8 | `cb7540` | `f356b0` | HideSubPlan::action_candidates | ⚠변경·다건 | stealth 이동 블록 · 후보 비면 Stop |
| 9 | `d65620` | `1011d20` | serpen_passive_plan | ❗대폭 | CONTEST 분기 · cc0 면 구 표 사장 · k-랭킹/lapse |
| 9 | `de92d0` | `fb5ff0` | epic_passive_plan | ❗대폭 | 동형(k-랭킹 없음 · lead/since 분기) |
| 9 | `df0e90` | `dcbe00` | SerpenHuntAndPokePlan::sub_plan | ⚠변경·다건 | ObjContest 반환 · finish_before_enemy · contest_can_hold |
| 9 | `defcd0` | `fd7f70` | EpicHuntAndPokePlan::sub_plan | ⚠변경·다건 | 동형(slot 1) |
| 9 | — | 20 | ObjContest 헬퍼 정체표 | (정체) | ef5f20/ef57f0/efb0c0/ef6da0/de9c80/ef7590/ef9de0/ef65a0/efb690/efb3d0/ef9c50/… |
| 10 | `d59940` | `f749a0` | calculate_action_score | ⚠변경·다건 | v3 적 챔피언 조기 반환 · 특성 flag(v2/v3 공통) · 인자 10 |
| 10 | `cb03b0` | `e96780` | EpicCheckSubPlan::action_candidates | ⚠변경·다건 | rs:65~75 재작성(SerpenCheck 패턴) · 인자 8 |
| 10 | `d5bbf0` | `f77820` | calculate_interaction_action_score | ⚠변경·다건 | 5지점(§C pmd 산식 공통 · lapse 인자 · shield 유지 v3 · walk_ticks · vt[0x70] 패널티) |
| 10 | `cce210` | `fd98c0` | SmallActionTrace::get_input | ✅동치 | 디버그 로그만(r20 「전담」 정정) |
| 11 | `e49a50` | `d51a70` | LPH::update_on_dead(★짝) | ⚠변경·다건 | version 0x5e 상수→실제 · 씬 종료+lock 프롤로그 · +0x24d6 assign |
| 11 | `ca6700` | `e13c10` | get_small_action_score_closure(★짝) | ⚠변경·한 줄 | v3 veto f71610(본체 미독) · 가중치표 동치 |
| 11 | `d40b20` | `fafe50` | best_jungle_goal | ⚠변경·다건 | v2 동치 · v3 3블록(B 미독) · allow_invade 인자 |
| 11 | `db8e60` | `fa9d60` | LineGankerPlan::make_gank_battle | ⚠변경·소 | gank_open_site/line/snap 기록 · 나머지 동치 |
| 11 | `dd9f30` | `f10c60` | TeamPlan::can_near_enemies_range | ✅동치 | lapse 8번째 인자 · DM 경로만 |
| 11 | `de0770` | `f18f90` | TeamPlan::update | ⚠변경·다건 | 레거시 본체 동치 · 감사 프롤로그(영향 없음 추정) · finish_race 캐시 에필로그 |
| 11 | `e06df0` | `ee5e80` | resolve_fight_stake | ⚠변경·다건 | bias · arrivals=포킹취약 6tps+1 |
| 11 | `e0b030` | `eebb40` | resolve_fight_stake_roster | ⚠변경·소 | bias |
| 11 | `e0b730` | `eec620` | v25_scoped_battle_objective | ⚠변경·한 줄 | phase {1,2}→{1,2,4} |
| 11 | `e25030` | `e93e00` | SmallActionLaneMinionPosition::target_score | ✅동치 | 인라인만 |
| 11 | `e4b070` | `d57b00` | v2_apply_assign_commit | ⚠변경·다건 | obj_key 이중모드 · 헬퍼화 5 · 라인 래치 꼬리 |
| — | `dc6ec0` | — | SmallActionAroundHide::get_input | 소멸 | 0.6.0 에 없음(enum 제거) |

(w1 의 나머지: AroundPosition/Hide 생성자 · w2 BattlePlan 필드표 · w4 LPH::update 콜리표 · w5 §C 합류 · w7 TeamPlan 전수표 194 는 §A 재료로 흡수.)

## 회계(specs20_v060 · 268 = `mkspec060.py` 09-17 최종)

| 구분 | 수 | 근거 |
|---|---|---|
| **동치(오프셋·태그·슬롯·RVA 갱신만)** | **192** | mig060_same 확정 150 + r19/r20/r21 디컴 동치 42 |
| **변경(재명세 대상)** | **73** | 한 줄 24 · 다건 16 · 심층/대폭(§D) 23 · 기타 변경 7 · 소 2 · 부분규명 1(position_eval_at_uncached 종반 항) |
| 소멸 | 1 | dc6ec0 AroundHide::get_input |
| exe 없음(명세만) | 2 | i=? sub_plan · new(0.5.8 도 exe 짝 없음) |

이전 회계(r20 시점 유효 177 · 재명세 65 · 미발견 12)와의 차: 미발견 12 → 짝 11 확정(동치 2 · 변경 9) + 소멸 1 · r20 「전담」 4 중 Trace 동치 · r20 G 7 이 규칙 파서에 누락돼 있던 것을 회수(regex [A-F]→[A-G]).

## 정정 누적(r21 이 뒤집은 것)
- `+0xcc7` 세팅처 = f0f080 **TeamPlan pre-update**(TeamPlan::update f18f90 과 별개 · w7 라벨 「TeamPlan::update 프롤로그」 정정 · w11).
- TeamPlan +0x3e4/+0x404 = 씬 Option take_born 니치(「mode」 아님 · w7).
- LPH +0x24d5 = Support 슬롯 제외(「exclude_jungler」 아님 · w6) · LPH team_plan +0xf8(w5).
- bb minion_count +0x48 mid · +0x70 bottom(배치 A 오기 · w9) · 10167f0 = 건강 아군 수(w9) · ef7590 두번째 반환값(w9).
- SmallActionTrace::get_input = 동치(배치 B 「전담·레이아웃 변경」 오독 · w10) · ChampionScoreParameter +0x70 applyed/+0x80 risk(힌트 오기 · w10).
- 0.5.8 update_on_dead 는 version 상수 0x5e 를 넘겼다(w11) · eeb5b0 = open_chase_race_hopeless(w1 「escape_possible」 라벨 · w11).

## 잔여 → `r21_pending.md`(미독 구역 r22 후보 · 게임 필요 보류)
