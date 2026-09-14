# r13 행동 계층 잎 61 편성표 (생성 2026-09-14 · rvaname 실명 · 지도 라벨은 참고만)

> argscan(09-14): `0xd9f3c0` check_nontarget exe 5 = IR 5 ✓ · `0xe28060` line_recall_pressure_penalty exe ≤4(레지스터만) IR 3 ✓ · ⚠`0xebcbd0` should_add_self_etc_buff_action **exe 6 ≠ IR 5**(스택 arg6 이 `call [rsp+0x188]` = 함수 포인터 → 팻포인터/구조체 ArgumentPromotion 의심 · #103 과 같은 유형 → sweep 은 `EXE_ABI_UNRECOVERABLE` 후보 · 호출자 2 로 간접 검증 예정)
> 합계: 확정 21 · 다후보 1 · 클로저 39 — 클로저 = 부모 명세의 aux(독립 명세 대상 아님) · 다후보 = Location 다수 본체를 1순위로 두고 배치가 IR 로 확정
| # | RVA | 판정 | IR 실명(짧게) | IR | define 줄 | 크기 | 호출자 | 지도 라벨 | Location |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `0xd399a0` | 확정 | `ozCnw_7game_ai5utils26nontarget_windup_perceived` | m04.ll | 53108 | 503B | 13 | nontarget_windup_perceived | utils.rs:557:7 |
| 2 | `0xd97700` | 확정 | `ozCnw_7game_ai16tower_discipline20v3_deadly_edge_cells` | m07.ll | 49636 | 533B | 5 | v3_deadly_edge_cells | tower_discipline.rs:308:21 |
| 3 | `0xeb8b80` | 확정 | `ozCnw_7game_ai11fight_check20attack_summon_action` | m15.ll | 28194 | 1874B | 14 | battle_action | fight_check.rs:837:93 · entity.rs:1483:7 · fight_check.rs:83 |
| 4 | `0xd408e0` | 다후보(2) | `ozCnw_7game_ai11plan_legacy3old14passive_jungle10is_cleared` | m04.ll | 62404 | 564B | 3 | is_cleared | team_plan.rs:40:10 · passive_jungle.rs:842:15 · passive_jung |
| 5 | `0xd98740` | 확정 | `ozCnw_7game_ai16tower_discipline29can_tower_focused_when_attack` | m07.ll | 50962 | 397B | 2 | can_tower_focused_when_attack | tower_discipline.rs:89:17 |
| 6 | `0xcaea70` | 클로저 | `ozCnw_7game_ai11plan_legacy8sub_plan6battleNtB4_13BattleSubPlan16base_positionings7_0Ba_` | m02.ll | — | 197B | 1 | base_positioning | battle.rs:216:22 |
| 7 | `0xde5300` | 확정 | `ozCnw_7game_ai11plan_legacy9team_plan8camp_idx` | m09.ll | 54334 | 54B | 1 | camp_idx | team_plan.rs:40:10 |
| 8 | `0xd405d0` | 확정 | `ozCnw_7game_ai11plan_legacy3old13defense_nexus30base_attacking_minion_uncached` | m04.ll | 62109 | 774B | 3 | base_attacking_minion_uncached | defense_nexus.rs:281:15 |
| 9 | `0xd9f3c0` | 확정 | `ozCnw_7game_ai12small_action4cast15check_nontarget` | m07.ll | 60191 | 804B | 3 | is_end | cast.rs:43:93 · cast.rs:56:59 · cast.rs:43:15 |
| 10 | `0xd57420` | 클로저 | `?` | ? | — | 279B | 1 | hp_at_tick | utils.rs:49:14 · utils.rs:60:19 · utils.rs:61:19 |
| 11 | `0xc872f0` | 클로저 | `_NtNtCs9ec1k27omRZ_3std6thread5localINtB6_8LocalKeyINtNtCsjihNppCmMEE_4core4cell7RefCellTyjINtNtNtNtBa` | m00.ll | — | 625B | 2 | available_cc_in_window | fight_check.rs:361:19 |
| 12 | `0xe2fd50` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9Filte` | m12.ll | — | 394B | 1 | ? | battle.rs:494:29 |
| 13 | `0xe30080` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9Filte` | m12.ll | — | 394B | 1 | ? | battle.rs:1137:19 |
| 14 | `0xe31960` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9Filte` | m12.ll | — | 394B | 1 | ? | battle.rs:105:25 |
| 15 | `0xe32470` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9Filte` | m12.ll | — | 394B | 1 | ? | ganker.rs:124:25 |
| 16 | `0xe32600` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9Filte` | m12.ll | — | 394B | 1 | ? | ganker.rs:132:25 |
| 17 | `0xe32790` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9Filte` | m12.ll | — | 394B | 1 | ? | ganker.rs:232:19 |
| 18 | `0xe30210` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9Filte` | m12.ll | — | 394B | 1 | is_end | around.rs:71:23 |
| 19 | `0xe32c60` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9Filte` | m12.ll | — | 394B | 1 | get_input | move_actions.rs:105:19 |
| 20 | `0xe28060` | 확정 | `ozCnw_7game_ai12lane_economy28line_recall_pressure_penalty` | m11.ll | 51097 | 398B | 1 | line_recall_pressure_penalty | lane_economy.rs:244:22 · lane_economy.rs:252:6 · lane_econom |
| 21 | `0xd47620` | 클로저 | `ozCnw_7game_ai12action_score17interaction_scores2_0B5_` | m05.ll | — | 407B | 1 | in_reach | action_score.rs:689:131 |
| 22 | `0xc9fed0` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core3ops8function5implsQNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passi` | m04.ll | — | 420B | 1 | ? | passive_jungle.rs:357:11 |
| 23 | `0xd34750` | 확정 | `ozCnw_7game_ai14abstract_input29safe_move_avoiding_enemy_well` | m04.ll | 43378 | 3440B | 8 | safe_move_avoiding_enemy_well | abstract_input.rs:101:21 |
| 24 | `0xca1fa0` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core3ops8function5implsQNCNvNtCshdEBA0ozCnw_7game_ai12action_score17interaction_s` | m05.ll | — | 449B | 1 | interaction_score_cl | action_score.rs:765:22 |
| 25 | `0xca21c0` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core3ops8function5implsQNCNvNtCshdEBA0ozCnw_7game_ai12action_score34calculate_int` | m05.ll | — | 449B | 1 | interaction_score_cl | action_score.rs:1029:22 |
| 26 | `0xca1020` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core3ops8function5implsQNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6b` | m02.ll | — | 450B | 1 | v48_on_cast_line | battle.rs:568:25 |
| 27 | `0xd34470` | 확정 | `ozCnw_7game_ai14abstract_input29convert_to_move_action_target` | m04.ll | 43140 | 450B | 1 | convert_to_move_action_target | abstract_input.rs:1465:10 |
| 28 | `0xd808b0` | 클로저 | `core5slice4iter4IterINtNtBa_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulat` | m07.ll | — | 452B | 1 | v3_lethal_tower_position | tower_discipline.rs:454:21 |
| 29 | `0xe2f890` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterIBY_INtNtB8_10filter_map9F` | m12.ll | — | 476B | 1 | ? | hide.rs:61:23 |
| 30 | `0xe31af0` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9Filte` | m12.ll | — | 479B | 1 | new | battle.rs:165:25 |
| 31 | `0xcadcd0` | 클로저 | `core5slice4iter4IterINtNtBa_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulat` | m02.ll | — | 492B | 1 | kite_reposition_point | battle.rs:309:19 |
| 32 | `0xcadec0` | 클로저 | `core5slice4iter4IterINtNtBa_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulat` | m02.ll | — | 492B | 1 | fight_kit_range | battle.rs:328:19 |
| 33 | `0xebc140` | 확정 | `ozCnw_7game_ai11fight_check29attack_structure_skill_action` | m15.ll | 33783 | 2498B | 5 | attack_structure_skill_action | fight_check.rs:795:93 · fight_check.rs:795:15 |
| 34 | `0xe30840` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9Filte` | m12.ll | — | 522B | 1 | new_counter_jungle | passive_jungle.rs:238:16 |
| 35 | `0xc7ee80` | 클로저 | `_NtNtCs9ec1k27omRZ_3std6thread5localINtB6_8LocalKeyINtNtCsjihNppCmMEE_4core4cell7RefCellNtNtCshdEBA0oz` | m00.ll | — | 574B | 1 | position_eval_cache_probe | position_eval.rs:299:19 · position_eval.rs:305:7 |
| 36 | `0xe019d0` | 확정 | `ozCnw_7game_ai10buff_value14v55_seal_value` | m10.ll | 33434 | 614B | 1 | v55_seal_value | buff_value.rs:525:12 · buff_value.rs:526:13 |
| 37 | `0xc875a0` | 클로저 | `_NtNtCs9ec1k27omRZ_3std6thread5localINtB6_8LocalKeyINtNtCsjihNppCmMEE_4core4cell7RefCellTyjINtNtNtNtBa` | m00.ll | — | 626B | 1 | v48_projectile_profile | fight_check.rs:342:19 |
| 38 | `0xc7ebc0` | 클로저 | `_NtNtCs9ec1k27omRZ_3std6thread5localINtB6_8LocalKeyINtNtCsjihNppCmMEE_4core4cell7RefCellNtNtCshdEBA0oz` | m00.ll | — | 630B | 1 | side_visibility_masks_cached | position_eval.rs:261:19 · position_eval.rs:271:12 · position |
| 39 | `0xe269a0` | 확정 | `ozCnw_7game_ai12small_action11lane_minionNtB2_29SmallActionLaneMinionPosition36should_suppress_direct_minion_attack` | m11.ll | 45771 | 665B | 1 | lane_stance_risk | lane_minion.rs:153:23 · entity.rs:1483:7 |
| 40 | `0xd79da0` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core3ops8function5implsQNCNvMs6_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNt` | m08.ll | — | 669B | 1 | ? | around.rs:1180:65 |
| 41 | `0xd7a080` | 클로저 | `_NtNtNtCsjihNppCmMEE_4core3ops8function5implsQNCNvMs6_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNt` | m08.ll | — | 669B | 1 | ? | around.rs:1155:65 |
| 42 | `0xc7f370` | 클로저 | `_NtNtCs9ec1k27omRZ_3std6thread5localINtB6_8LocalKeyINtNtCsjihNppCmMEE_4core4cell7RefCellNtNtCshdEBA0oz` | m00.ll | — | 670B | 1 | cached_units_in_range_of | position_eval.rs:189:19 · simulation.rs:1848:5 |
| 43 | `0xe02020` | 확정 | `ozCnw_7game_ai10buff_value18v55_banish_penalty` | m10.ll | 34297 | 686B | 1 | v55_banish_penalty | buff_value.rs:601:66 · simulation.rs:1905:5 · buff_value.rs: |
| 44 | `0xd9ddc0` | 확정 | `ozCnw_7game_ai12small_action4cast14is_safe_recall` | m07.ll | 58406 | 5533B | 8 | is_safe_recall | cast.rs:326:41 · cast.rs:439:58 · cast.rs:305:93 |
| 45 | `0xc89a90` | 클로저 | `_NtNtCs9ec1k27omRZ_3std6thread5localINtB6_8LocalKeyINtNtCsjihNppCmMEE_4core4cell7RefCellTyjjjAThxxxxyE` | m00.ll | — | 1529B | 2 | max_range | battle.rs:1003:19 · simulation.rs:1905:5 |
| 46 | `0xcaff00` | 확정 | `ozCnw_7game_ai12small_action5traceNtB2_16SmallActionTrace22expected_goal_position` | m02.ll | 8995 | 1029B | 1 | expected_goal_position | trace.rs:118:17 · entity.rs:1483:7 |
| 47 | `0xc9e160` | 클로저 | `bumpalo11collections3vecINtB3_3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6` | m01.ll | — | 1193B | 1 | update | blackboard.rs:138:5 · passive_jungle.rs:369:25 · passive_jun |
| 48 | `0xebcbd0` | 확정 | `ozCnw_7game_ai11fight_check31should_add_self_etc_buff_action` | m15.ll | 34622 | 2459B | 2 | should_add_self_etc_buff_action | fight_check.rs:593:3 |
| 49 | `0xe01450` | 확정 | `ozCnw_7game_ai10buff_value14v55_mark_value` | m10.ll | 33063 | 1394B | 1 | v55_mark_value | buff_value.rs:735:20 · simulation.rs:1905:5 · buff_value.rs: |
| 50 | `0xc993e0` | 클로저 | `bumpalo11collections3vecINtB3_3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6` | m01.ll | — | 1452B | 1 | update_v32 | battle.rs:428:25 · battle.rs:429:15 · battle.rs:430:29 |
| 51 | `0xd25120` | 클로저 | `core3ops11index_rangeNtB5_10IndexRangeNtNtNtNtB9_4iter6traits8iterator8Iterator8try` | m04.ll | — | 1443B | 1 | ? | passive_jungle.rs:483:16 · passive_jungle.rs:482:19 · simula |
| 52 | `0xe02540` | 확정 | `ozCnw_7game_ai10buff_value23noncombat_steroid_value` | m10.ll | 34945 | 1658B | 1 | noncombat_steroid_value | buff_value.rs:62:16 · simulation.rs:1905:5 · buff_value.rs:7 |
| 53 | `0xe202b0` | 클로저 | `core4iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJ` | m11.ll | — | 1861B | 1 | ? | line_defense.rs:804:19 · simulation.rs:1905:5 |
| 54 | `0xd390a0` | 확정 | `ozCnw_7game_ai5utils26champion_hp_value_uncached` | m04.ll | 52225 | 2304B | 1 | champion_hp_value_uncached | utils.rs:885:19 · utils.rs:977:5 |
| 55 | `0xca6ec0` | 클로저 | `bumpalo11collections3vecINtB3_3VecTyjEE12from_iter_inINtNtNtNtCsjihNppCmMEE_4core4ite` | m01.ll | — | 3173B | 1 | has_near_line_enemy | goal_data.rs:583:63 · goal_data.rs:584:60 · utils.rs:80:14 |
| 56 | `0xc96830` | 클로저 | `bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActi` | m01.ll | — | 6747B | 1 | ? | simulation.rs:1905:5 · entity.rs:1483:7 · epic_poke.rs:172:3 |
| 57 | `0xc8fac0` | 클로저 | `bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActi` | m01.ll | — | 6747B | 1 | ? | simulation.rs:1905:5 · entity.rs:1483:7 · serpen_poke.rs:173 |
| 58 | `0xdffa10` | 확정 | `ozCnw_7game_ai10buff_value14buff_value_v54` | m10.ll | 30000 | 6720B | 1 | buff_value_v54 | buff_value.rs:224:96 · simulation.rs:1905:5 · buff_value.rs: |
| 59 | `0xc8dc90` | 클로저 | `bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActi` | m01.ll | — | 7142B | 1 | ? | simulation.rs:1905:5 · entity.rs:1483:7 · serpen_hunt.rs:396 |
| 60 | `0xc94a00` | 클로저 | `bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActi` | m01.ll | — | 7142B | 1 | ? | simulation.rs:1905:5 · entity.rs:1483:7 · epic_hunt.rs:397:3 |
| 61 | `0xc91770` | 클로저 | `bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActi` | m01.ll | — | 10217B | 1 | ? | simulation.rs:1905:5 · entity.rs:1483:7 · line_defense.rs:68 |