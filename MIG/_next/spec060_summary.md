# specs20_v060 요약(0.6.0 v060 블록) — 268 specs

판정 분포: 동치 150 · ✅동치 43 · 변경·심층 23 · ⚠변경·한 줄 22 · ⚠변경·다건 16 · 변경 7 · exe 없음 2 · 변경·소 2 · 변경·한 줄 1 · 소멸 1 · ⚠변경·부분규명 1

| i | 구 | 신 | 함수 | 판정 | 패치 |
|---|---|---|---|---|---|
| 0 | `d354c0` | `de1ad0` | ult | 동치(mig060_same 확정: 동일) |  |
| 1 | `d5ba80` | `f776b0` | calculate_jungle_action_score | 동치(mig060_same 확정: 동일) |  |
| 2 | `None` | `None` | sub_plan | exe 없음 |  |
| 3 | `e01c40` | `e82c00` | defensive_crisis | ⚠변경·한 줄(r20) | 한 줄 · 6번째 인자 extra_tick · `die_imminent = death < (v>=3 ? extra_ti |
| 4 | `d3cfa0` | `e753e0` | handle_line_defense | 동치(mig060_same 확정: 동일) |  |
| 5 | `e59190` | `d5aff0` | v50_fold_dive_episode | ✅동치(r20) |  |
| 6 | `e657a0` | `d6bf10` | v2_response_retreat_stance | ✅동치(r20) |  |
| 7 | `ccc010` | `e60420` | sub_plan | ✅동치(r20) |  |
| 8 | `defa20` | `fd7b40` | is_end | ⚠변경·한 줄(r20) | 한 줄 · L164 `tp.3e4!=2 → true` + `tp.404==2 && tp.cd5!=0 → true` ·  |
| 9 | `ebd570` | `edfb50` | check_favorable_engage_formation | 동치(mig060_same 확정: 동일) |  |
| 10 | `e0c560` | `eed490` | should_end_object_finish_kill_priority_b | 동치(mig060_same 확정: 동일) |  |
| 11 | `e4b5d0` | `d582a0` | v3_fall_back_to_passive | 동치(mig060_same 확정: 동일) |  |
| 12 | `e595b0` | `d5b410` | handle_chat | 동치(mig060_same 확정: 동일) |  |
| 13 | `df1c80` | `fa82a0` | target_bush_v30 | 동치(mig060_same 확정: 동일) |  |
| 14 | `db90f0` | `faa440` | update | ⚠변경·한 줄(r20) | 심층 · 저HP 취소 시 `+0xaf=1` · 부시 도착 `if +0x38==0 {+0x38=tick}` |
| 15 | `e5c1f0` | `d5ebb0` | single_try_engage | ✅동치(r20) |  |
| 16 | `e0daa0` | `dea490` | max_range_nearly_can_use | 동치(mig060_same 확정: 동일) |  |
| 17 | `None` | `None` | new | exe 없음 |  |
| 18 | `dce220` | `ef4690` | v3_epicops_buff_window | ⚠변경·다건(r20) | 다건 · [2] 세르펜 징벌: cc5==0 이면 구 · else 계약 레코드(+0xc0/d0/d8/e0 · f1e3c |
| 19 | `d40b20` | `fafe50` | best_jungle_goal | 변경(r21 w11: v2 동치 · v3 3블록 · allow_invade 인자) | 심층(r21) · **반환 = (camp:u8, owner_team)**(블록 B → 1−team) · 게이트: `in_set |
| 20 | `dd50e0` | `f0c310` | v27_active_objective_discipline | 동치(mig060_same 확정: 동일) |  |
| 21 | `e7a8c0` | `f27140` | upgrade_item | ✅동치(r20) |  |
| 22 | `d97300` | `1008640` | can_tower_focused | 동치(mig060_same 확정: 동일) |  |
| 23 | `e7b640` | `f27ea0` | buy_item | ⚠변경·한 줄(r20) | 한 줄 · `items.len()>2 → continue` 삭제 → 선검사 `active_owned = items.fi |
| 24 | `ec9840` | `f882e0` | v23_healthy_allies_near_point | 동치(mig060_same 확정: 동일) |  |
| 25 | `e0c310` | `eed240` | v22_visible_enemy_is_runaway_threat | 동치(mig060_same 확정: 동일) |  |
| 26 | `eca9a0` | `f892d0` | v23_objective_setup_pressure_line | 동치(mig060_same 확정: 동일) |  |
| 27 | `d3fa80` | `e77ec0` | nexus_under_direct_attack | 동치(mig060_same 확정: 동일) |  |
| 28 | `e0bd60` | `eecc90` | v21_should_defer_support_target | 동치(mig060_same 확정: 동일) |  |
| 29 | `ecacc0` | `f895f0` | v23_recent_visible_enemies_near_point | 동치(mig060_same 확정: 동일) |  |
| 30 | `ec8af0` | `f86f50` | objective_is_damaged | 동치(mig060_same 확정: 동일) |  |
| 31 | `dea800` | `fb78b0` | v3_epic_formation_role | 동치(mig060_same 확정: 동일) |  |
| 32 | `d665e0` | `1012e50` | serpen_giveup_chat_reason | 동치(mig060_same 확정: 동일) |  |
| 33 | `e4aec0` | `d56570` | v2_obj_restore_safe | ✅동치(r20) |  |
| 34 | `d3e4b0` | `e768f0` | has_line_defense_threat | 동치(mig060_same 확정: 동일) |  |
| 35 | `e06df0` | `ee5e80` | resolve_fight_stake | 변경(r21 w11: bias · arrivals=ee3c80 포킹취약 6tps+1) | 심층(r21) · `bias = aggr?1 : def?-1 : 0` · **`arrivals[i] = ee3c80(versi |
| 36 | `e6d000` | `d75ad0` | calculate_nexus_defense_count | 동치(mig060_same 확정: 동일) |  |
| 37 | `d3d560` | `e759a0` | i_am_chosen_defender | ✅동치(r20) |  |
| 38 | `d988d0` | `1009c20` | can_tower_focused_when_battle | 동치(mig060_same 확정: 동일) |  |
| 39 | `ec9400` | `f87ea0` | is_wave_priority_start_line | 동치(mig060_same 확정: 동일) |  |
| 40 | `e4a780` | `d55830` | v3_assign_anchor | 동치(r19: 동치(LPH 레이아웃)) |  |
| 41 | `e04f50` | `ee3e40` | fight_participants | ⚠변경·다건(r20) | 다건 · 근접 아군 push `(a, poke_vulnerable? tps*6+1 : 0, false)` · +0xc |
| 42 | `d51fb0` | `dd87b0` | SinglePlanBattle::with_runaway | 동치(mig060_same 확정: 동일) |  |
| 43 | `dfb220` | `dd87b0` | BattlePlan::with_runaway | 동치(mig060_same 확정: 동일) |  |
| 44 | `e4b8c0` | `d59280` | LegacyPlanHandler::take_misunderstood_re | 동치(mig060_same 확정: 동일) |  |
| 45 | `d36480` | `de2a90` | can_recall | 동치(mig060_same 확정: 동일) |  |
| 46 | `d3c700` | `e74b40` | need_defense_nexus | 동치(mig060_same 확정: 동일) |  |
| 47 | `dce520` | `ef4ab0` | TeamPlan::handle_epic_line_change | 동치(mig060_same 확정: 동일) |  |
| 48 | `e0b730` | `eec620` | v25_scoped_battle_objective | 변경·한 줄(r21 w11: phase {1,2}→{1,2,4}) | 심층(r21) · 한 줄: phase 집합 `{1,2}` → **`{1,2,4}`**(마스크 0x16 · 4 = take_bo |
| 49 | `e05e70` | `ee4e70` | resolve_join_stake | ⚠변경·다건(r20) | 다건 · 인자 (committed:i8, horizon_sec) · horizon=tps*horizon_sec · c |
| 50 | `de81b0` | `fb49b0` | check_epic_giveup | ✅동치(+필드/게이트/텔레메트리)(r20) |  |
| 51 | `de40c0` | `f1fce0` | check_press_tower_opportunity | 동치(mig060_same 확정: 동일) |  |
| 52 | `eca430` | `f88d60` | v25_objective_far_split_pressure | 동치(mig060_same 확정: 동일) |  |
| 53 | `de6ce0` | `fb34e0` | check_epic_setup | 동치(mig060_same 확정: 동일) |  |
| 54 | `de0770` | `f18f90` | TeamPlan::update | 변경(r21 w11: 레거시 본체 동치 · 감사 프롤로그 · finish_race 에필로그 | 심층(r21) · **레거시 본체(f1a5a3~f1d3da) = 0.5.8 L295~L442 동치** · ★정정: **+0xc |
| 55 | `d815e0` | `feefc0` | EntityPositioningCache::new | 동치(mig060_same 확정: 동일) |  |
| 56 | `d2c5d0` | `ecb2b0` | PassiveLinePlan::sub_plan | ⚠변경·다건(r20) | 다건 · 6건: +0x117→LineWait · 갱크 게이트 bb Vec · +0x119 action_type 0 · |
| 57 | `dcd930` | `ef3da0` | handle_press_epic | 동치(mig060_same 확정: 동일) |  |
| 58 | `db8ba0` | `fb9b70` | target_bush_v41 | ✅동치(r20) |  |
| 59 | `e0bf70` | `eecea0` | is_unreasonable_tower_dive_enemy | 동치(mig060_same 확정: 동일) |  |
| 60 | `d98210` | `1009560` | can_trace_without_tower | 동치(mig060_same 확정: 동일) |  |
| 61 | `d9aa80` | `e85d70` | bush_distance_sq | 동치(mig060_same 확정: 동일) |  |
| 62 | `ec9190` | `f87c30` | v23_enemy_object_pressure | 동치(mig060_same 확정: 동일) |  |
| 63 | `de03d0` | `f18bf0` | should_keep_object_for_contested_wave_pr | 동치(mig060_same 확정: 동일) |  |
| 64 | `eca200` | `f88b30` | v25_objective_splitter_can_stay | 동치(mig060_same 확정: 동일) |  |
| 65 | `de3d90` | `100b8d0` | check_epic_kill_time_with_hp | 동치(mig060_same 확정: 오프셋만 변경) |  |
| 66 | `d40f10` | `fb1510` | evaluate_gank_opportunity_with_score | ✅동치(r20) | ~~한 줄~~→**동치(apply060 b05 정정: 0.5.8 도 /20 · 변경은 노이즈 헬퍼 de2fa0 아웃라인뿐 · RNG 소비 동일)** · 노이즈 폭 `k=(1000-judge)/10` → `/20` (헬퍼 de2fa0 · RNG 마스크 달라짐) |
| 67 | `dd6b40` | `f0dc20` | v24_objective_setup_lane_pressure_ready | ⚠변경·한 줄(r20) | 심층 · L13 take_setup_like 게이트 이중모드(dd5db0 L91 과 동일 패턴) |
| 68 | `dff080` | `ddd7e0` | base_sub_goal | ⚠변경·한 줄(r20) | 한 줄 · `Some(_) => if v>=3 && Response && ally_within_120000 { Kiti |
| 69 | `e5d300` | `d5ff20` | try_engage_dive | ⚠변경·한 줄(r20) | 한 줄 · `plan.screening=true; update; screening=false; if sub_goal>= |
| 70 | `e07430` | `ee6d40` | tower_dive_is_viable | ⚠변경·다건(r20) | 다건 · v3 near_enemies 클로저 e0fc40(대상 기준 150000·최근가시) · pen=(900-9J) |
| 71 | `e7acd0` | `f27590` | should_recall_to_shop | ⚠변경·한 줄(r20) | 한 줄 · `if active_cnt > 3 && item_list[next].tier()==0 { return fal |
| 72 | `df0a90` | `dcb890` | SerpenHuntAndPokePlan::is_end | ⚠변경·다건(r20) | 다건 · 진입: `tp.3e4==2 → (tp.404!=2 || tp.cd5!=1) → true` · setup_li |
| 73 | `ec9de0` | `f88880` | wave_priority_clearer_position | ⚠변경·한 줄(r20) | 한 줄 · 4번째 인자 exclude_jungler: `pick(excl).or_else(|| pick(false))` |
| 74 | `dccc60` | `ff11f0` | GoalData::update | ⚠변경·한 줄(r20) | 한 줄 · goal_data.rs:27 앞 `if v>=3 && champ.undying { remain=max(und |
| 75 | `dd73b0` | `ff2450` | SerpenStanceData::update_plan | ✅동치(r20) |  |
| 76 | `de1ee0` | `ff4940` | EpicStanceData::update_plan | ✅동치(r20) |  |
| 77 | `d2da10` | `e71d40` | DefenseNexusPlan::sub_plan | ✅동치(r20) |  |
| 78 | `d3dcc0` | `e76100` | objective_defense_role | 동치(mig060_same 확정: 동일) |  |
| 79 | `e38c90` | `dc1c10` | engage::can_battle_triggered_filtered | 동치(mig060_same 확정: 동일) |  |
| 80 | `e5ca10` | `d5f3c0` | try_engage | ⚠변경·다건(r20) | 다건 · v3 게이트 A(LPH+0x2060/68/70 최근 실패 교전 reason∈{7,13,15,16}·3tps· |
| 81 | `de5340` | `fb1b70` | check_epic_hunt | ✅동치(r20) |  |
| 82 | `de92d0` | `fb5ff0` | epic_passive_plan | 변경·심층(r21 §D: w9 §3) | 심층(r21) · serpen 동형 · cc0 대체 Setup = remain(epic +0x1b0) → `since=tick |
| 83 | `defcd0` | `fd7f70` | EpicHuntAndPokePlan::sub_plan | 변경·심층(r21 §D: w9 §5) | 심층(r21) · dcbe00 동형: slot 1 · ef7590(1) · C `3e4!=2 → Recall; 404==2 & |
| 84 | `eb9570` | `ed8f40` | battle_check_with_list | 동치(mig060_same 확정: 동일) |  |
| 85 | `ebbb80` | `ede160` | should_disengage_object_hunt | 동치(mig060_same 확정: 동일) |  |
| 86 | `dfe2a0` | `ed5720` | FightSituation::build | ⚠변경·다건(r20) | 다건 · FightSituation 96B: endangered_carry·my_skills_ready·my_ult_ |
| 87 | `e6b800` | `d72e70` | check_kill | 변경·심층(r21 §D: w1 §2) | 심층(r21) · v3 확장 후보 e0f970 · escape_possible eeb5b0 · (c1,c2,c3) 특성 · v |
| 88 | `e4b070` | `d57b00` | v2_apply_assign_commit | 변경(r21 w11: obj_key 이중모드 · 헬퍼화 · 라인 래치 꼬리) | 심층(r21) · 필드 v2_obj_part +0x24a2/+0x24a3 · v2_assign +0x24a6/+0x24a7 · |
| 89 | `dd5db0` | `f0cfe0` | TeamPlan::v24_objective_setup_should_che | ⚠변경·한 줄(r20) | 한 줄 · L91 게이트: 3e4!=2 → camp 5 && 3e0==1 → [2,1] · 404!=2 → camp 4 |
| 90 | `ec8ba0` | `f87600` | v3_epicops_defer_serpen | ✅동치(r20) |  |
| 91 | `d2e500` | `fa3980` | PassiveJunglePlan::sub_plan | 변경·심층(r21 §D: w3 §2) | 심층(r21) · v3 undying → 매복블록 직행 · fb12a0 recall_need · **cj_ambush Some |
| 92 | `d3b2a0` | `eed7f0` | v46_flee_gate_check | ⚠변경·다건(r20) | 다건 · A = aggr?0 : def?240 : 구 · B = 67(특성) : 구 · v3 `*60→*tps` |
| 93 | `d639f0` | `1010060` | check_serpen_giveup | ⚠변경·한 줄(r20) | 한 줄 · take_hunt_commit = (3e4==2) ? (404==2&&cd5==1&&cd6==3) : (3e |
| 94 | `d61330` | `100d990` | check_serpen_hunt | 동치(mig060_same 확정: 동일) |  |
| 95 | `d62bb0` | `100f210` | check_serpen_setup | 동치(mig060_same 확정: 동일) |  |
| 96 | `d65620` | `1011d20` | serpen_passive_plan | 변경·심층(r21 §D: w9 §2) | 심층(r21) · 진입 match 에 `phase>2 && cc7 → CONTEST` · `phase==2 && cc7 → C |
| 97 | `df0e90` | `dcbe00` | SerpenHuntAndPokePlan::sub_plan | 변경·심층(r21 §D: w9 §4) | 심층(r21) · B v≥3: `serpen.hp==max && (can_upgrade ‖ (in_heal && hp<max) |
| 98 | `d9bce0` | `e87300` | steal::evaluate_steal_for_target | 동치(mig060_same 확정: 동일) |  |
| 99 | `d9ac10` | `e85f00` | steal::should_steal_now | ⚠변경·한 줄(r20) | 심층 · `objective!=Morgard/Serpen` 4곳 → `not_morgard = 3e4!=2 || (4 |
| 100 | `dd90c0` | `f0e5f0` | TeamPlan::update_steal | ✅동치(+필드/게이트/텔레메트리)(r20) |  |
| 101 | `d3a3a0` | `de6a80` | line_backfight_support_focus | 동치(mig060_same 확정: 동일) |  |
| 102 | `dfb840` | `dd8e70` | BattlePlan::update | 변경·심층(r21 §D: w2 §1) | 심층(r21) · 본체 동치(재번호·이중모드) · Defense 넥서스 거리 End 는 v<3 만 · battle_recenc |
| 103 | `d8ca70` | `ffdda0` | position_risk_all_zero_near | 동치(mig060_same 확정: 동일) |  |
| 104 | `e59b20` | `d5b980` | handle_chat_inner | 변경·심층(r21 §D: w7 §3) | 심층(r21) · Battle/Dive/Help 처리 → d5d700 아웃라인(5.2KB · 미독) · 종류별 무시 게이트 L |
| 105 | `e46bc0` | `d38180` | passive_plan | 변경·심층(r21 §D: w6 §1) | 심층(r21) · 반환 (BigPlan 0x228,code) · objective JT 재배치 · 에고웨이브 `roam>49  |
| 106 | `dd26e0` | `f09860` | TeamPlan::v25_objective_posture | ✅동치(+필드/게이트/텔레메트리)(r20) |  |
| 107 | `dcee40` | `f024d0` | TeamPlan::handle_none_or_gank_objective | 변경·심층(r21 §D: w7 §2) | 심층(r21) · v3 chat 억제 · armed 시 압박탑 생략·L910 직행 · **V6 commit 블록**(gathe |
| 108 | `dda220` | `f10f70` | TeamPlan::update_objective_after_steal | 변경·심층(r21 §D: w7 §1) | 심층(r21) · armed 면 v27 discipline **비활성** · V6 track 종료 감시 · 목표 사망(v3 n |
| 109 | `df36e0` | `dcce40` | BattlePlan::update_v32 | 변경·심층(r21 §D: w2 §0·§2) | 심층(r21) · 신규 구역 8: 플립플롭 계측/브레이커(exit 12) · TANKAVAIL(die_avail) · TWBI |
| 110 | `e5d5d0` | `d606b0` | LegacyPlanHandler::handle_interact_battl | 변경·심층(r21 §D: w5 §4) | 심층(r21) · 신규 프롤로그(flee_latch/latch_v3 · pending tower_binary +0x1ef8 동 |
| 111 | `e4c5c0` | `d3d210` | LegacyPlanHandler::update | 변경·심층(r21 §D: w4 §3~§5) | 심층(r21) · 호출자 = 래퍼 d5a940 · 구 본체 오프셋만(egowave 상수 스케일 1건) · 신규 17.5KB 목 |
| 112 | `d399a0` | `de6080` | nontarget_windup_perceived | 동치(mig060_same 확정: 동일) |  |
| 113 | `d97700` | `1008a40` | v3_deadly_edge_cells | 동치(mig060_same 확정: 동일) |  |
| 114 | `eb8b80` | `ed8550` | attack_summon_action | 동치(mig060_same 확정: 동일) |  |
| 115 | `d408e0` | `faf060` | is_cleared | 동치(mig060_same 확정: 동일) |  |
| 116 | `d98740` | `1009a90` | can_tower_focused_when_attack | 동치(mig060_same 확정: 동일) |  |
| 117 | `de5300` | `f20f20` | camp_idx | 동치(mig060_same 확정: 동일) |  |
| 118 | `d405d0` | `e78a10` | base_attacking_minion_uncached | 동치(r19: 동치) |  |
| 119 | `d9f3c0` | `de8f80` | check_nontarget | 동치(mig060_same 확정: 동일) |  |
| 120 | `e28060` | `e73260` | line_recall_pressure_penalty | 동치(mig060_same 확정: 동일) |  |
| 121 | `d34750` | `de0d60` | safe_move_avoiding_enemy_well | 동치(mig060_same 확정: 동일) |  |
| 122 | `d34470` | `de0a80` | convert_to_move_action_target | 동치(mig060_same 확정: 동일) |  |
| 123 | `ebc140` | `ede720` | attack_structure_skill_action | 동치(mig060_same 확정: 동일) |  |
| 124 | `e019d0` | `e82990` | v55_seal_value | 동치(mig060_same 확정: 동일) |  |
| 125 | `e269a0` | `e95a30` | SmallActionLaneMinionPosition::should_su | 동치(mig060_same 확정: 동일) |  |
| 126 | `e02020` | `e83000` | v55_banish_penalty | 동치(mig060_same 확정: 동일) |  |
| 127 | `d9ddc0` | `de7980` | is_safe_recall | 동치(mig060_same 확정: 동일) |  |
| 128 | `caff00` | `fe16c0` | SmallActionTrace::expected_goal_position | 동치(mig060_same 확정: 동일) |  |
| 129 | `ebcbd0` | `edf1b0` | should_add_self_etc_buff_action | 동치(mig060_same 확정: 동일) |  |
| 130 | `e01450` | `e82410` | v55_mark_value | 동치(mig060_same 확정: 동일) |  |
| 131 | `e02540` | `e83520` | noncombat_steroid_value | 동치(mig060_same 확정: 동일) |  |
| 132 | `d390a0` | `de5780` | champion_hp_value_uncached | 동치(mig060_same 확정: 동일) |  |
| 133 | `dffa10` | `e80960` | buff_value_v54 | ⚠변경·다건(r20) | 다건 · 시그니처 +version,+champ_incoming · v3 crisis: undying 이면 !(cris |
| 134 | `dc8550` | `ee0960` | around::check_cell | 동치(mig060_same 확정: 동일) |  |
| 135 | `d98420` | `1009770` | v3_lethal_tower_position | 동치(mig060_same 확정: 동일) |  |
| 136 | `ecb300` | `f89c30` | v23_should_break_objective_hunt_anchor | 동치(mig060_same 확정: 동일) |  |
| 137 | `cd0480` | `eb5140` | v48_on_cast_line | 동치(mig060_same 확정: 동일) |  |
| 138 | `ccaa10` | `eacee0` | EpicPokeSubPlan::score | 동치(r19: 동치) |  |
| 139 | `e81390` | `f64d30` | SerpenPokeSubPlan::score | 동치(r19: 동치) |  |
| 140 | `dc2550` | `ecfff0` | SmallActionAroundBush::new_with_target | 동치(mig060_same 확정: 동일) |  |
| 141 | `e23fa0` | `e5e0e0` | SmallActionPlay::get_input | 동치(r19: 동치(AroundHide arm 소멸)) |  |
| 142 | `e29520` | `e64920` | path_needs_tower_escape | 동치(mig060_same 확정: 동일) |  |
| 143 | `e8b200` | `f352d0` | LineDefenseSubPlan::score | 동치(mig060_same 확정: 이동만) |  |
| 144 | `e266e0` | `e95770` | SmallActionLaneMinionPosition::has_curre | 동치(mig060_same 확정: 동일) |  |
| 145 | `e23170` | `e5d4e0` | SmallActionPlay::evaluation_position | 동치(r19: 동치(vt 0x60/0x68→0x80/0x88)) |  |
| 146 | `dc27a0` | `ed0240` | SmallActionAroundBush::new_with_out_line | 동치(mig060_same 확정: 동일) |  |
| 147 | `e28410` | `e73610` | line_action_economy_adjustment | 동치(r19: 동치) |  |
| 148 | `ccf670` | `eb4320` | serpen_action_score | 동치(mig060_same 확정: 오프셋 이동) |  |
| 149 | `ec7c90` | `f860e0` | epic_action_score | 동치(mig060_same 확정: 오프셋 이동) |  |
| 150 | `d99030` | `100a380` | v22_lane_tower_pressure_attack_allowed | 동치(mig060_same 확정: 동일) |  |
| 151 | `d845e0` | `ddd010` | SmallActionSkill::get_input | 동치(mig060_same 확정: 동일) |  |
| 152 | `e281f0` | `e733f0` | line_champion_trade_allowance | 동치(mig060_same 확정: 동일) |  |
| 153 | `d84800` | `ddd230` | SmallActionSkill2::get_input | 동치(mig060_same 확정: 동일) |  |
| 154 | `d84a30` | `ddd460` | SmallActionUlt::get_input | 동치(mig060_same 확정: 동일) |  |
| 155 | `db8e60` | `fa9d60` | LineGankerPlan::make_gank_battle | 변경·소(r21 w11: gank_open_site/line/snap 기록 · sub_go | 심층(r21) · 소변경: site 인자 → `bp.+0x214 gank_open_site=site` · `bp.+0x205  |
| 156 | `dd5270` | `f0c4a0` | v27_objective_discipline_action | 동치(mig060_same 확정: 동일) |  |
| 157 | `dc2070` | `ecfa40` | SmallActionAroundPositionBush::get_input | ⚠변경·다건(r20) | 다건 · PathVerdict: `ok && ((region==2&&flag2)||(region==7&&flag7)) |
| 158 | `e2ac00` | `eb3980` | line_minion_action_candidates | 동치(mig060_same 확정: 동일) |  |
| 159 | `eb77a0` | `ed79a0` | battle_ally_action | 동치(mig060_same 확정: 동일) |  |
| 160 | `de3630` | `100bb90` | get_die_tick_player | 동치(mig060_same 확정: 동일) |  |
| 161 | `e2a830` | `eb3470` | lane_minion_position_action | ✅동치(r20) |  |
| 162 | `e04400` | `e85460` | v57_summon_command_score | 동치(mig060_same 확정: 동일) |  |
| 163 | `d35930` | `de1f40` | abstract_input::skill | 동치(mig060_same 확정: 동일) |  |
| 164 | `d360a0` | `de26b0` | abstract_input::skill2 | 동치(mig060_same 확정: 동일) |  |
| 165 | `e29b40` | `100c2f0` | action_eval::evaluate_action | ⚠변경·다건(r20) | 다건 · score += aggressive_gain(aggr: 최근접 적 피해×hp_value/hp · reach+ |
| 166 | `d3ab40` | `de7220` | utils::can1v1win | 동치(mig060_same 확정: 동일) |  |
| 167 | `e26c40` | `e95f70` | SmallActionLaneMinionPosition::get_input | 동치(r19: 동치) |  |
| 168 | `dc2960` | `ed0400` | SmallActionAroundBush::get_input | 변경(r19: ★로직 변경: path_finder 이름≠around_bush 면 폐기·재생 | 한 줄 · `if self.path_finder.as_ref().is_some_and(|pf| pf.name != "a |
| 169 | `ccfbc0` | `eb4880` | calculate_serpen_action_score | 동치(mig060_same 확정: 동일) |  |
| 170 | `ec81e0` | `f86640` | calculate_epic_action_score | 동치(mig060_same 확정: 동일) |  |
| 171 | `e288c0` | `e73ac0` | line_projected_punish_damage_at | 동치(mig060_same 확정: 동일) |  |
| 172 | `dbf680` | `eccaf0` | SmallActionPositioning::get_input | 동치(mig060_same 확정: 동일) |  |
| 173 | `dc6ec0` | `None` | SmallActionAroundHide::get_input | 소멸(0.6.0 에 없음) |  |
| 174 | `cc9740` | `eabbb0` | EpicPokeSubPlan::action_candidates_old | ✅동치(r20) |  |
| 175 | `e80070` | `f63a10` | SerpenPokeSubPlan::action_candidates_old | 동치(mig060_same 확정: 이동만) |  |
| 176 | `cce210` | `fd98c0` | SmallActionTrace::get_input | 동치(r21 w10: 디버그 로그만 · 배치 B 「전담」 정정) | 심층(r21) · **로직 동치** — self 레이아웃 동일(+0x10 PathFinder 인라인 · +0x60~+0x95) |
| 177 | `dbbd60` | `ed4350` | SmallActionAroundRegion::get_input | 동치(mig060_same 확정: 동일) |  |
| 178 | `dc0800` | `ece1d0` | SmallActionAroundPosition::get_input | 동치(mig060_same 확정: 동일) |  |
| 179 | `db6ff0` | `ec3060` | SmallActionAround::get_input | ✅동치(r20) |  |
| 180 | `d851d0` | `ff62e0` | position_eval_at_uncached | ⚠변경·부분규명(전담)(r20) | 전담 · 종반 신규 항: 고체력 적 중심점 이격 보너스(cap 25 · purpose 4 ×2 · cap 60) →  |
| 181 | `d5bbf0` | `f77820` | calculate_interaction_action_score | 변경·심층(r21 §D: w10 §3) | 심층(r21) · 5지점: **§C projected_minion_damage(v2/v3 공통)** `adj = stance= |
| 182 | `dc3240` | `fddcd0` | SmallActionRunAway::get_input | ✅동치(r20) |  |
| 183 | `d57540` | `f72620` | interaction_score | ✅동치(r20) |  |
| 184 | `dbd260` | `fdb210` | SmallActionRecall::get_input | ⚠변경·한 줄(r20) | 한 줄 · v3: 최근가시 적 250000 내 `r1=ckdt(true,false,false)==0 || r2=ckdt |
| 185 | `d31f20` | `dde530` | get_input_target | 동치(mig060_same 확정: 동일) |  |
| 186 | `d59940` | `f749a0` | calculate_action_score | 변경·심층(r21 §D: w10 §1) | 심층(r21) · 인자 11→10(`_debug` 삭제) · **v3 적 챔피언 조기 반환**(Champion arm · po |
| 187 | `d84db0` | `ff5ec0` | position_eval_at | 동치(mig060_same 확정: 동일) |  |
| 188 | `cbbdb0` | `e9fe00` | BattleSubPlan::action_candidates | ✅동치(+필드/게이트/텔레메트리)(r20) | 콜리 · 본체 동치 · 콜리 resolve_fight_stake ee5e80(ee3c80 호출)·fight_parti |
| 189 | `e928f0` | `f40930` | DefenseNexusSubPlan::action_candidates | ✅동치(r20) |  |
| 190 | `e81680` | `f65030` | AttackNexusSubPlan::action_candidates | 동치(mig060_same 확정: 동일) |  |
| 191 | `cb03b0` | `e96780` | EpicCheckSubPlan::action_candidates | 변경·심층(r21 §D: w10 §2) | 심층(r21) · 인자 9→8(debug 삭제 · rs:52~53 로그 제거) · **rs:65~75 재작성 = SerpenC |
| 192 | `e8b5e0` | `e9de70` | SerpenCheckSubPlan::action_candidates | ⚠변경·다건(r20) | 다건 · L25 안 `if tp.cc0==0 {move_check=true} else { near>=REQ[tutor |
| 193 | `cc4260` | `eafb50` | JungleSubPlan::action_candidates | 변경·심층(r21 §D: w3 §1) | 심층(r21) · position_score 셀 중심 6인자(parameter 삭제) · v3 RunAway 조건(적 (12f |
| 194 | `cb7540` | `f356b0` | HideSubPlan::action_candidates | 변경·심층(r21 §D: w8 §B) | 심층(r21) · L34/L107 `!(v3 && stealth)` 게이트 · **stealth 이동 블록**(region== |
| 195 | `eb43a0` | `ead1c0` | LineWaitSubPlan::action_candidates | ✅동치(r20) |  |
| 196 | `e83390` | `f2caf0` | LineDefenseSubPlan::action_candidates | ✅동치(+필드/게이트/텔레메트리)(r20) |  |
| 197 | `cb1ce0` | `e986c0` | SerpenHuntSubPlan::action_candidates | ⚠변경·한 줄(r20) | 한 줄 · get_move: `if tp.cc7 { s=efb5b0(tp,0,team0); if dist²(me,ser |
| 198 | `eaeda0` | `f6bda0` | EpicHuntSubPlan::action_candidates | ⚠변경·한 줄(r20) | 한 줄 · get_move 전 `if tp.cc7 { s=efb5b0(tp,1,team0); if dist²(epic, |
| 199 | `cc6170` | `ea8610` | EpicPokeSubPlan::action_candidates | ✅동치(r20) |  |
| 200 | `e7caf0` | `f60480` | SerpenPokeSubPlan::action_candidates | ✅동치(r20) |  |
| 201 | `d35d10` | `de2320` | abstract_input::attack | 동치(mig060_same 확정: 동일) |  |
| 202 | `db9430` | `faa7c0` | LineGankerPlan::next_plan | 변경·심층(r21 §D: w8 §A) | 심층(r21) · 3분기(open/**response**(BattlePlan 프로브 → screened 폐기 · BattleH |
| 203 | `d2f180` | `fa47a0` | PassiveJunglePlan::next_plan | 변경·심층(r21 §D: w3 §3~4) | 심층(r21) · v3 블록 A 매복 유지/해제 · 블록 B 매복 시도(fa2670) · cj_meet 플래그군 · lead_ |
| 204 | `d8eff0` | `1000320` | calculate_score_parameter | ✅동치(r20) |  |
| 205 | `e65b10` | `d6c2b0` | LegacyPlanHandler::get_small_action | 변경·심층(r21 §D: w6 §2) | 심층(r21) · 본체 동치 + **v3 아군 대상 궁**(eff.vt[0xc0] info · defensive_crisis( |
| 206 | `d28800` | `ec7180` | PassiveLinePlan::update | ⚠변경·다건(r20) | 다건 · ①check_recall v3 undying 게이트 ②v46 bound = 1416fbbc0(특성) ③L23 |
| 207 | `e083c0` | `ee8450` | resolve_fight_uncached | ⚠변경·한 줄(r20) | 한 줄 · bias:i8 인자(judge_acc 뒤) · committed==-1: thr=unit*(1-bias) · |
| 208 | `e900b0` | `f3da50` | get_input | ⚠변경·한 줄(r20) | 한 줄 · v3 폴백: input None && 도주계(RunAway/Recall/AroundRunAway) && !우 |
| 209 | `d377a0` | `de3e80` | build_minion_wave_snapshot | 동치(mig060_same 확정: 동일) |  |
| 210 | `cba660` | `e5eb00` | StealSubPlan::action_candidates | ✅동치(r20) |  |
| 211 | `eba9b0` | `edb240` | fight_check::check_kill_die_tick_uncache | 변경·심층(r21 §D: w1 §1) | 심층(r21) · v2 = 구 본체 + bool 게이트(simple/ignore_nuke/no_noise) · **v3 = 신 |
| 212 | `e8e560` | `f3bde0` | AgentVerHamster::update_small_action | ✅동치(r20) |  |
| 213 | `d26900` | `ec4c20` | PassiveLinePlan::v46_stage1 | ⚠변경·한 줄(r20) | 한 줄 · (c,b,a′) = aggr(+0x49d)?(0,67,32) : def(+0x49e)?(240,67,32)  |
| 214 | `cc3080` | `ea6f40` | BattleSubPlan::score | 동치(r19: 동치(goal 4→3·vt +0x20)) |  |
| 215 | `d676c0` | `fe3fd0` | v16_gambler_ult_cc_bonus | ✅동치(r20) |  |
| 216 | `e8a100` | `f33fa0` | unsafe_v19_non_champion_walkup | ✅동치(r20) |  |
| 217 | `d69f80` | `fe6840` | v17_runaway_counterattack_bonus | ✅동치(r20) |  |
| 218 | `e8d6f0` | `f3adb0` | AgentVerHamster::update_state | ⚠변경·다건(r20) | 다건 · awareness_lapse 8번째 인자 m=1000+min(+0x490,100)²/20 · plan upd |
| 219 | `cc20a0` | `ea6240` | BattleSubPlan::calculate_score_parameter | ✅동치(r20) |  |
| 220 | `d27d50` | `ec62f0` | PassiveLinePlan::v46_stage2 | 동치(mig060_same 확정: 동일) |  |
| 221 | `d39ba0` | `de6280` | precompute_champion_powers | 동치(mig060_same 확정: 동일) |  |
| 222 | `d3e660` | `e76aa0` | nexus_last_stand_uncached | 동치(mig060_same 확정: 동일) |  |
| 223 | `e95ae0` | `f43b10` | DefenseNexusSubPlan::score | ✅동치(r20) |  |
| 224 | `d67060` | `fe3970` | v21_defensive_cc_score | 동치(mig060_same 확정: 동일) |  |
| 225 | `e8fd70` | `f3d660` | AgentVerHamster::item_v26 | ⚠변경·한 줄(r20) | 한 줄 · 활성템 `count>=4`(구 len>=3) && !tier → None |
| 226 | `e7b0b0` | `f27910` | build_game_finish_check_state | 동치(mig060_same 확정: 동일) |  |
| 227 | `d98ac0` | `1009e10` | v47_tower_focus_position_dangerous | 동치(mig060_same 확정: 동일) |  |
| 228 | `e35a40` | `dc0040` | TeamPlan::handle_nexus_attack | 동치(mig060_same 확정: 동일) |  |
| 229 | `e8f850` | `f3d140` | AgentVerHamster::count_nearby_enemies | 동치(mig060_same 확정: 동일) |  |
| 230 | `ccbb10` | `f3a070` | LineSafeSubPlan::score | 동치(r19: 동치) |  |
| 231 | `eb5840` | `eae650` | LineWaitSubPlan::score | 동치(r19: 동치) |  |
| 232 | `d99f60` | `100b2b0` | v30_line_champion_action_tower_aggro_ris | 동치(r19: 동치) |  |
| 233 | `d3fe50` | `e78290` | nexus_final_stand_uncached | 동치(mig060_same 확정: 동일) |  |
| 234 | `cc5a10` | `eb28e0` | JungleSubPlan::score | ✅동치(r20) |  |
| 235 | `e83080` | `f66a30` | AttackNexusSubPlan::score | ✅동치(r20) |  |
| 236 | `d69120` | `fe59e0` | v15_can_keep_support_pressure | 동치(mig060_same 확정: 동일) |  |
| 237 | `cc5fc0` | `ea8450` | RecallSubPlan::score | 동치(r19: 동치) |  |
| 238 | `d9a220` | `100b570` | v22_current_line_non_champion_action_tow | ✅동치(r20) |  |
| 239 | `dc2330` | `ecfdd0` | around::SmallActionAroundBush::update_st | 동치(r19: 동치) |  |
| 240 | `e8aeb0` | `f34d10` | LineDefenseSubPlan::calculate_score_para | ⚠변경·한 줄(r20) | 한 줄 · my = aggr?value : def?90 : value · param.+0x1501=true · ev = |
| 241 | `cc5ca0` | `ea8130` | RecallSubPlan::action_candidates | 동치(mig060_same 확정: 동일) |  |
| 242 | `cbbca0` | `e601b0` | StealSubPlan::score | 동치(mig060_same 확정: 이동만) |  |
| 243 | `cce0c0` | `fd9770` | trace::SmallActionTrace::is_end | 동치(mig060_same 확정: 동일) |  |
| 244 | `dbd130` | `fdb0e0` | move_actions::SmallActionRecall::is_end | 동치(mig060_same 확정: 동일) |  |
| 245 | `e8e160` | `f3b9e0` | AgentVerHamster::upgrade_item | 동치(mig060_same 확정: 동일) |  |
| 246 | `e388c0` | `ed3dd0` | SerpenCheckSubPlan::score | ⚠변경·다건(r20) | 다건 · 디스패처 후처리: `if param.1501 && def && !aggr { if let Some((a,b) |
| 247 | `e8fc80` | `f3d570` | AgentVerHamster::buy_item | 동치(mig060_same 확정: 동일) |  |
| 248 | `e23750` | `e5da70` | cast::SmallActionUlt::is_end | ✅동치(r20) |  |
| 249 | `cd05f0` | `eb52b0` | base_battle_action | ✅동치(r20) |  |
| 250 | `df1f50` | `fa8570` | next_plan | ✅동치(+필드/게이트/텔레메트리)(r20) |  |
| 251 | `eb6100` | `ed6300` | battle_action | 동치(mig060_same 확정: 동일) |  |
| 252 | `d68090` | `fe4950` | v16_knight_ult_zone_bonus | 동치(mig060_same 확정: 동일) |  |
| 253 | `ccacf0` | `f39240` | LineSafeSubPlan::action_candidates | 동치(r19: 동치(vt 0xe8→0x108)) |  |
| 254 | `e03360` | `e84340` | noncombat_steroid_window | ✅동치(r20) |  |
| 255 | `e7b9f0` | `f28320` | end_check | ⚠변경·한 줄(r20) | 한 줄 · 프렐류드 `if v>=3 { if let Some(r)=finish_race { margin=Aggr?tps |
| 256 | `e49a50` | `d51a70` | LegacyPlanHandler::update_on_dead | 변경(r21 w11: 프롤로그 씬 종료+lock · version 0x5e→실제 · +0x | 심층(r21) · AgentVerHamster vt 0x143a86c20 슬롯 +0x88 f4d170 경유 · **0.5.8  |
| 257 | `e0dfc0` | `dea9b0` | v30_wave_danger_chase_guard | 동치(r19: 동치(BattleSubPlanGoal 재번호)) |  |
| 258 | `e248f0` | `e936c0` | SmallActionLaneMinionPosition::choose_go | 동치(mig060_same 확정: 동일) |  |
| 259 | `eba320` | `eda1a0` | v48_projectile_profile | 동치(mig060_same 확정: 동일) |  |
| 260 | `e0b030` | `eebb40` | resolve_fight_stake_roster | 변경·소(r21 w11: bias 인자) | 심층(r21) · 소변경: 인자 +player/+debug · 동일 bias → absolute/diff · abandon 0 |
| 261 | `cd5ee0` | `ebab70` | kite_reposition_point | 동치(mig060_same 확정: 동일) |  |
| 262 | `e25030` | `e93e00` | SmallActionLaneMinionPosition::target_sc | 동치(r21 w11: 인라인만) | 심층(r21) · **동치**(상수 140/70/25/50/25/90/80/25/20/−3000 전부 · find/hp_at_ |
| 263 | `ca6700` | `e13c10` | get_small_action_score_closure | 변경(r21 w11: v3 veto f71610 · 가중치표 재번호 동치) | 심층(r21) · d6c2b0 d6d916 호출(Ghidra xref 누락) · `map(|c| (score(c),c)).co |
| 264 | `e25450` | `e944e0` | SmallActionLaneMinionPosition::push_cand | 동치(mig060_same 확정: 동일) |  |
| 265 | `e02bc0` | `e83ba0` | v54_aoe_ally_heal_value | 동치(mig060_same 확정: 동일) |  |
| 266 | `dd9f30` | `f10c60` | TeamPlan::can_near_enemies_range | 동치(r21 w11: 본체 동치 · lapse 8번째 인자 · DM 경로만 cc1) | 심층(r21) · **본체 동치** · judge_accuracy 인라인 동일식 · +0x49c 게이트는 0.5.8 에도 있음 |
| 267 | `eb5dd0` | `ed5fd0` | expected_dps | 동치(mig060_same 확정: 동일) |  |