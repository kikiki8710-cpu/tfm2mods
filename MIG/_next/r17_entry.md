# r17 vtable 진입점 잔여 편성표 — 진입점(rvaname 실명 · 2026-09-16 · 원장 §3-C 정정)

> 지도 미명세 plan/act 노드 195(인스턴스·excl 제외) → rvaname → 진입점 51(루트 48 + JT 디스패처 3) · 제외 = 클로저 132 · 데스매치/SingleLane 6 · 이미 편입/사장/기각 7
> 서브트리 합집합(콜리 포함 · 이미 명세 제외) = `_next\subtree_r17.md`

| # | RVA | 판정 | 실명 | IR | 크기 | 계층 | 지도 라벨 | 호출자 | 비고 |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `0xd8eff0` | 확정 | `score_parameter::calculate_score_parameter` | m07.ll | 25,381B | act | calculate_score_parameter | 1 | DONE 보류(미포팅 상위 5) — 25KB 거대 · rootcut 분책 대상 |
| 2 | `0xe65b10` | 확정 | `auction::LegacyPlanHandler::get_small_action` | m13.ll | 19,492B | act | get_small_action | 1 | LegacyPlanHandler::get_small_action 19KB — SmallActionPlay 실행기 루트 · 분책 대상 |
| 3 | `0xd28800` | 확정 | `passive_line::PassiveLinePlan::update` | m04.ll | 14,751B | plan(추정) | update | 0 |  |
| 4 | `0xe083c0` | 확정 | `fight_model::resolve_fight_uncached` | m10.ll | 8,841B | plan(추정) | resolve_fight_uncached | 0 | [D] ai_adjust judge DIFF=0 2,426,997(09-13) → 명세만 |
| 5 | `0xe900b0` | 확정 | `AgentVerHamster::get_input` | m14.ll | 7,884B | act | get_input | 0 |  |
| 6 | `0xd377a0` | 확정 | `utils::build_minion_wave_snapshot` | m04.ll | 6,286B | act | build_minion_wave_snapshot | 1 |  |
| 7 | `0xd3e660` | 확정 | `defense_nexus::nexus_last_stand_uncached` | m04.ll | 5,151B | act(추정) | nexus_last_stand_uncached | 0 |  |
| 8 | `0xcba660` | 확정 | `steal::StealSubPlan::action_candidates` | m02.ll | 5,108B | act(추정) | action_candidates | 0 |  |
| 9 | `0xd26900` | 확정 | `passive_line::PassiveLinePlan::v46_stage1` | m04.ll | 5,047B | plan(추정) | v46_stage1 | 1 |  |
| 10 | `0xcc3080` | 확정 | `battle::BattleSubPlan::score` | m02.ll | 4,491B | act | score | 1 |  |
| 11 | `0xe8e560` | 확정 | `AgentVerHamster::update_small_action` | m14.ll | 4,430B | act | update_small_action | 2 | 지도 라벨 update_small_action 은 오라벨 — DONE: AgentVerHamster::new(팩토리 아님) · rvaname lib.rs:774 |
| 12 | `0xeba9b0` | 확정 | `fight_check::check_kill_die_tick_uncached` | m15.ll | 4,429B | act(추정) | check_kill_die_tick_uncached | 0 | [D] ai_adjust judge DIFF=0 5,182,148(09-06) → 명세만 |
| 13 | `0xcc20a0` | 확정 | `battle::score_paramet` | m02.ll | 4,059B | plan(추정) | calculate_score_parameter_value | 0 |  |
| 14 | `0xe8a100` | 확정 | `line_defense::LineDefenseSubPlan::unsafe_v19_non_champion_walkup` | m14.ll | 3,432B | act(추정) | unsafe_v19_non_champion_walkup | 0 |  |
| 15 | `0xd676c0` | 확정 | `battle_common::v16_gambler_ult_cc_bonus` | m05.ll | 2,502B | act | v16_gambler_ult_cc_bonus | 2 |  |
| 16 | `0xd69f80` | 확정 | `battle_common::v17_runaway_counterattack_bonus` | m05.ll | 2,271B | act | v21_support_pressure_too_risky | 2 |  |
| 17 | `0xd27d50` | 확정 | `passive_line::PassiveLinePlan::v46_stage2` | m04.ll | 2,190B | plan(추정) | v46_stage2 | 1 |  |
| 18 | `0xe8d6f0` | 확정 | `AgentVerHamster::update_state` | m14.ll | 2,159B | act | clone_bo | 1 |  |
| 19 | `0xe23750` | 디스패처 | `cast::SmallActionUlt::is_end` | m07.ll | 2,118B | act | is_end | 1 | JT tail-jump 디스패처(DONE 보류) — #150 과 같은 미편입 부류 · 진입부 확인 후 판단 |
| 20 | `0xd39ba0` | 확정 | `utils::precompute_champion_powers` | m04.ll | 2,039B | act | precompute_champion_powers | 1 |  |
| 21 | `0xd3fe50` | 확정 | `defense_nexus::nexus_final_stand_uncached` | m04.ll | 1,906B | act(추정) | nexus_final_stand_uncached | 0 | [D] ai_adjust judge dn_reach DONE(09-06 · 캐시시점 0.6%) → 명세만 |
| 22 | `0xd67060` | 확정 | `battle_common::v21_defensive_cc_score` | m05.ll | 1,619B | act | v17_escape_is_costly | 2 |  |
| 23 | `0xe95ae0` | 확정 | `defense_nexus::DefenseNexusSubPlan::score` | m14.ll | 1,537B | act | score | 1 |  |
| 24 | `0xd98ac0` | 확정 | `tower_discipline::v47_tower_focus_position_dangerous` | m07.ll | 1,390B | act(추정) | v47_tower_focus_position_dangerous | 1 |  |
| 25 | `0xe7b0b0` | 확정 | `build_game_finish_check_state` | m14.ll | 1,244B | plan(추정) | build_game_finish_check_state | 0 |  |
| 26 | `0xe8f850` | 확정 | `AgentVerHamster::count_nearby_enemies` | m14.ll | 1,064B | act | count_nearby_enemies | 2 |  |
| 27 | `0xcaf9f0` | 디스패처 | `line_gank::cover::LineGankCoverPlan` | m10.ll | 1,049B | plan | BigPlan::sub_plan (host) | 2 | BigPlan::sub_plan JT 호스트 — #02 인라인 arm 이미 ev1 · 미편입 부류 |
| 28 | `0xe388c0` | 디스패처 | `serpen_check::SerpenCheckSubPlan::score` | m14.ll | 966B | act | ? | 1 | JT tail-jump 디스패처(DONE 보류) — 미편입 부류 |
| 29 | `0xccbb10` | 확정 | `line_safe::LineSafeSubPlan::score` | m02.ll | 926B | act | score | 1 |  |
| 30 | `0xeb5840` | 확정 | `line_wait::LineWaitSubPlan::score` | m15.ll | 926B | act | score | 1 |  |
| 31 | `0xe8aeb0` | 확정 | `line_defense::LineDefenseSubPlan::calculate_score_parameter_value` | m14.ll | 836B | act(추정) | calculate_score_parameter_value | 0 |  |
| 32 | `0xe8fd70` | 확정 | `AgentVerHamster::item_v26` | m14.ll | 818B | plan(추정) | item_v26 | 2 |  |
| 33 | `0xe83080` | 확정 | `attack_nexus::AttackNexusSubPlan::score` | m14.ll | 778B | act | score | 1 |  |
| 34 | `0xd99f60` | 확정 | `tower_discipline::v30_line_champion_action_tower_aggro_risk` | m07.ll | 689B | act(추정) | v30_line_champion_action_tower_aggro_risk | 0 |  |
| 35 | `0xcc5a10` | 확정 | `jungle::JungleSubPlan::score` | m02.ll | 641B | act | score | 1 |  |
| 36 | `0xd69120` | 확정 | `battle_common::v15_can_keep_support_pressure` | m05.ll | 602B | act | v15_can_keep_support_pressure | 4 |  |
| 37 | `0xcc5ca0` | 확정 | `recall::RecallSubPlan::action_candidates` | m02.ll | 571B | act(추정) | action_candidates | 0 |  |
| 38 | `0xd9a220` | 확정 | `tower_discipline::v22_current_line_non_champion_action_tower_risk` | m07.ll | 563B | act(추정) | v22_current_line_non_champion_action_tower_risk | 1 |  |
| 39 | `0xdc2330` | 확정 | `_NtNt7::around::SmallActionAroundstat` | m08.ll | 531B | act | update_state | 1 |  |
| 40 | `0xcc5fc0` | 확정 | `recall::RecallSubPlan::score` | m02.ll | 432B | act | score | 1 |  |
| 41 | `0xe35a40` | 확정 | `objective_handlers::TeamPlan::handle_nexus_attack` | m09.ll | 393B | plan(추정) | handle_nexus_attack | 0 |  |
| 42 | `0xe33570` | 확정 | `fight_model::resolve_fight_uncached` | m10.ll | 391B | plan(추정) | ? | 1 |  |
| 43 | `0xe33700` | 확정 | `fight_model::resolve_fight_uncached` | m10.ll | 391B | plan(추정) | ? | 1 |  |
| 44 | `0xccbeb0` | ? | `hunt_and_battle::SerpenHuntAndBattlePlan::update` | m02.ll | 343B | plan(추정) | update | 0 | hunt_and_battle update 다후보 · 사장 계열 의심 → 편성 시 IR 확인 |
| 45 | `0xccc260` | ? | `hunt_and_battle::SerpenHuntAndBattlePlan::update` | m02.ll | 343B | plan(추정) | update | 0 | hunt_and_battle update 다후보 · 사장 계열 의심 → 편성 시 IR 확인 |
| 46 | `0xcce0c0` | 확정 | `trace::SmallActionTrace::is_end` | m02.ll | 333B | act | is_end | 1 |  |
| 47 | `0xdbd130` | 확정 | `move_actions::SmallActionRecall::is_end` | m08.ll | 291B | act | is_end | 1 |  |
| 48 | `0xd57420` | 확정 | `hp_at_tick` | ? | 279B | act | hp_at_tick | 1 |  |
| 49 | `0xcbbca0` | ? | `steal::StealSubPlan::score` | m02.ll | 269B | act | score | 1 | [D] ai_adjust judge steal_score DIFF=0 7판(09-06) → 명세만 |
| 50 | `0xe8e160` | 확정 | `AgentVerHamster::upgrade_item` | m14.ll | 254B | plan(추정) | upgrade_item | 0 |  |
| 51 | `0xe8fc80` | 확정 | `AgentVerHamster::buy_item` | m14.ll | 230B | plan(추정) | buy_item | 0 |  |

## 제외

| RVA | 사유 | 실명/인스턴스 | 크기 | 모듈 | 비고 |
|---|---|---|---|---|---|
| `0xd781e0` | SingleLane(NA) | `plan_legacy` | 5,111B | single_line |  |
| `0xd478c0` | 데스매치(NA) | `plan_legacy` | 16,861B | death_battle |  |
| `0xeade00` | 데스매치(NA) | `plan_legacy` | 3,913B | death_battle |  |
| `0xebf380` | 데스매치(NA) | `plan_legacy` | 3,191B | death_battle |  |
| `0xec7930` | 데스매치(NA) | `plan_legacy` | 859B | death_battle |  |
| `0xea96e0` | 데스매치(NA) | `인스턴스 1` | 686B | death_battle |  |
| `0xdeaa70` | 이미 편입/사장/기각 | `_ep` | 1,230B | epic | 이미 #22(원장 1-C DIFF 0) → 제외 |
| `0xe92310` | 이미 편입/사장/기각 | `_15A` | 1,083B | lib | Debug fmt(lib.rs:382) — 판단 함수 아님 → 제외 |
| `0xd666c0` | 이미 편입/사장/기각 | `_se` | 641B | serpen | 이미 #24(1-C) → 제외 |
| `0xccc3c0` | 이미 편입/사장/기각 | `_2` | 591B | hunt_and_battle | 사장(hunt_and_battle 15 · 발화 0 · 09-13 IR 확정) → 제외 |
| `0xe2f230` | 이미 편입/사장/기각 | `wave_priority_clearer_position` | 531B | objective_helpers | 독립 진입부 없음(#44 기각 09-13) → 제외 |
| `0xec9bf0` | 이미 편입/사장/기각 | `is_object_being_taken_by_enemy` | 364B | objective_helpers | 이미 #23(1-C) → 제외 |
| `0xc91770` | 클로저(부모 aux) | `인스턴스 1` | 10,217B | line_defense |  |
| `0xc8dc90` | 클로저(부모 aux) | `인스턴스 1` | 7,142B | serpen_hunt |  |
| `0xc94a00` | 클로저(부모 aux) | `인스턴스 1` | 7,142B | epic_hunt |  |
| `0xc8fac0` | 클로저(부모 aux) | `인스턴스 1` | 6,747B | serpen_poke |  |
| `0xc96830` | 클로저(부모 aux) | `인스턴스 1` | 6,747B | epic_poke |  |
| `0xca6ec0` | 클로저(부모 aux) | `인스턴스 1` | 3,173B | goal_data |  |
| `0xcd6940` | 클로저(부모 aux) | `인스턴스 1` | 2,597B | serpen_hunt |  |
| `0xecb870` | 클로저(부모 aux) | `인스턴스 1` | 2,597B | epic_hunt |  |
| `0xcd7660` | 클로저(부모 aux) | `인스턴스 1` | 2,518B | epic_poke |  |
| `0xe9fdc0` | 클로저(부모 aux) | `인스턴스 1` | 2,518B | serpen_poke |  |
| `0xe05450` | 클로저(부모 aux) | `인스턴스 1` | 2,450B | fight_model |  |
| `0xc7f640` | 클로저(부모 aux) | `인스턴스 1` | 2,332B | position_eval |  |
| `0xc7d7d0` | 클로저(부모 aux) | `인스턴스 1` | 2,008B | lib |  |
| `0xd73c30` | 클로저(부모 aux) | `인스턴스 2` | 1,999B | line_defense |  |
| `0xd74400` | 클로저(부모 aux) | `인스턴스 2` | 1,999B | battle |  |
| `0xc99fe0` | 클로저(부모 aux) | `인스턴스 2` | 1,882B | team_plan |  |
| `0xe202b0` | 클로저(부모 aux) | `인스턴스 2` | 1,861B | line_defense |  |
| `0xe20a00` | 클로저(부모 aux) | `인스턴스 2` | 1,861B | battle |  |
| `0xd41980` | 클로저(부모 aux) | `인스턴스 1` | 1,464B | passive_jungle |  |
| `0xc993e0` | 클로저(부모 aux) | `인스턴스 2` | 1,452B | battle |  |
| `0xc999e0` | 클로저(부모 aux) | `인스턴스 2` | 1,452B | battle |  |
| `0xc9a790` | 클로저(부모 aux) | `인스턴스 2` | 1,452B | fight_model |  |
| `0xd25120` | 클로저(부모 aux) | `인스턴스 1` | 1,443B | passive_jungle |  |
| `0xc809d0` | 클로저(부모 aux) | `인스턴스 1` | 1,420B | battle |  |
| `0xe34150` | 클로저(부모 aux) | `인스턴스 1` | 1,409B | score_parameter |  |
| `0xc80f80` | 클로저(부모 aux) | `인스턴스 1` | 1,390B | battle |  |
| `0xc89530` | 클로저(부모 aux) | `인스턴스 1` | 1,307B | action_eval |  |
| `0xc9dc60` | 클로저(부모 aux) | `인스턴스 2` | 1,193B | passive_jungle |  |
| `0xc9e160` | 클로저(부모 aux) | `인스턴스 2` | 1,193B | passive_jungle |  |
| `0xc87af0` | 클로저(부모 aux) | `인스턴스 1` | 1,187B | around |  |
| `0xd75cb0` | 클로저(부모 aux) | `인스턴스 0` | 1,166B | buff_value |  |
| `0xe229d0` | 클로저(부모 aux) | `인스턴스 0` | 1,166B | buff_value |  |
| `0xea07a0` | 클로저(부모 aux) | `인스턴스 1` | 1,112B | line_defense |  |
| `0xc7e150` | 클로저(부모 aux) | `인스턴스 1` | 1,102B | fight_check |  |
| `0xe21650` | 클로저(부모 aux) | `인스턴스 2` | 892B | tower_discipline |  |
| `0xe1e000` | 클로저(부모 aux) | `인스턴스 2` | 847B | position_eval |  |
| `0xe1e350` | 클로저(부모 aux) | `인스턴스 2` | 847B | tower_discipline |  |
| `0xe1e6a0` | 클로저(부모 aux) | `인스턴스 2` | 847B | tower_discipline |  |
| `0xe1ed20` | 클로저(부모 aux) | `인스턴스 2` | 847B | score_parameter |  |
| `0xe1f070` | 클로저(부모 aux) | `인스턴스 2` | 847B | score_parameter |  |
| `0xe1f3c0` | 클로저(부모 aux) | `인스턴스 2` | 847B | score_parameter |  |
| `0xe1f710` | 클로저(부모 aux) | `인스턴스 2` | 847B | score_parameter |  |
| `0xca59f0` | 클로저(부모 aux) | `인스턴스 2` | 835B | battle |  |
| `0xe70c70` | 클로저(부모 aux) | `인스턴스 1` | 809B | handler |  |
| `0xe11290` | 클로저(부모 aux) | `인스턴스 1` | 806B | battle |  |
| `0xc7e5a0` | 클로저(부모 aux) | `인스턴스 1` | 793B | fight_check |  |
| `0xca6090` | 클로저(부모 aux) | `인스턴스 2` | 746B | score_parameter |  |
| `0xe2fa70` | 클로저(부모 aux) | `인스턴스 2` | 733B | objective_discipline |  |
| `0xe12220` | 클로저(부모 aux) | `인스턴스 1` | 732B | fight_model |  |
| `0xc87fe0` | 클로저(부모 aux) | `인스턴스 1` | 724B | utils |  |
| `0xc98dc0` | 클로저(부모 aux) | `인스턴스 1` | 718B | passive_line |  |
| `0xe11710` | 클로저(부모 aux) | `인스턴스 1` | 708B | battle |  |
| `0xe119e0` | 클로저(부모 aux) | `인스턴스 1` | 708B | battle |  |
| `0xc990e0` | 클로저(부모 aux) | `인스턴스 1` | 685B | passive_line |  |
| `0xc7f370` | 클로저(부모 aux) | `인스턴스 1` | 670B | position_eval |  |
| `0xe33040` | 클로저(부모 aux) | `인스턴스 1` | 670B | battle |  |
| `0xd79da0` | 클로저(부모 aux) | `인스턴스 1` | 669B | around |  |
| `0xd7a080` | 클로저(부모 aux) | `인스턴스 1` | 669B | around |  |
| `0xd75a30` | 클로저(부모 aux) | `인스턴스 1` | 636B | around |  |
| `0xc7ebc0` | 클로저(부모 aux) | `인스턴스 1` | 630B | position_eval |  |
| `0xc875a0` | 클로저(부모 aux) | `인스턴스 1` | 626B | fight_check |  |
| `0xc872f0` | 클로저(부모 aux) | `인스턴스 1` | 625B | fight_check |  |
| `0xc7e920` | 클로저(부모 aux) | `인스턴스 1` | 610B | fight_check |  |
| `0xc7f0f0` | 클로저(부모 aux) | `인스턴스 1` | 591B | position_eval |  |
| `0xe32df0` | 클로저(부모 aux) | `인스턴스 1` | 585B | fight_model |  |
| `0xe33f00` | 클로저(부모 aux) | `인스턴스 1` | 580B | fight_check |  |
| `0xc7ee80` | 클로저(부모 aux) | `인스턴스 1` | 574B | position_eval |  |
| `0xca4a80` | 클로저(부모 aux) | `인스턴스 1` | 571B | modes |  |
| `0xe30840` | 클로저(부모 aux) | `인스턴스 1` | 522B | passive_jungle |  |
| `0xcadcd0` | 클로저(부모 aux) | `인스턴스 1` | 492B | battle |  |
| `0xcadec0` | 클로저(부모 aux) | `인스턴스 1` | 492B | battle |  |
| `0xe31af0` | 클로저(부모 aux) | `인스턴스 1` | 479B | battle |  |
| `0xe11cb0` | 클로저(부모 aux) | `인스턴스 1` | 476B | battle |  |
| `0xe2f890` | 클로저(부모 aux) | `인스턴스 1` | 476B | hide |  |
| `0xc7ff90` | 클로저(부모 aux) | `인스턴스 1` | 461B | tower_discipline |  |
| `0xd808b0` | 클로저(부모 aux) | `인스턴스 1` | 452B | tower_discipline |  |
| `0xe10da0` | 클로저(부모 aux) | `인스턴스 1` | 449B | fight_model |  |
| `0xe3b1f0` | 클로저(부모 aux) | `인스턴스 1` | 449B | engage |  |
| `0xd470f0` | 클로저(부모 aux) | `인스턴스 1` | 447B | battle_common |  |
| `0xe11e90` | 클로저(부모 aux) | `인스턴스 1` | 437B | buff_value |  |
| `0xc80160` | 클로저(부모 aux) | `인스턴스 1` | 433B | utils |  |
| `0xd6c020` | 클로저(부모 aux) | `인스턴스 1` | 426B | serpen |  |
| `0xe3b840` | 클로저(부모 aux) | `인스턴스 2` | 424B | tower_discipline |  |
| `0xe3b9f0` | 클로저(부모 aux) | `인스턴스 2` | 424B | tower_discipline |  |
| `0xe3bd30` | 클로저(부모 aux) | `인스턴스 2` | 424B | score_parameter |  |
| `0xe3bee0` | 클로저(부모 aux) | `인스턴스 2` | 424B | score_parameter |  |
| `0xe3c090` | 클로저(부모 aux) | `인스턴스 2` | 424B | score_parameter |  |
| `0xe3c240` | 클로저(부모 aux) | `인스턴스 2` | 424B | score_parameter |  |
| `0xc9c590` | 클로저(부모 aux) | `인스턴스 1` | 420B | serpen |  |
| `0xc9fed0` | 클로저(부모 aux) | `인스턴스 1` | 420B | passive_jungle |  |
| `0xe39a40` | 클로저(부모 aux) | `인스턴스 2` | 418B | position_eval |  |
| `0xe39bf0` | 클로저(부모 aux) | `인스턴스 2` | 418B | tower_discipline |  |
| `0xe39da0` | 클로저(부모 aux) | `인스턴스 2` | 418B | tower_discipline |  |
| `0xe3a0d0` | 클로저(부모 aux) | `인스턴스 2` | 418B | score_parameter |  |
| `0xe3a280` | 클로저(부모 aux) | `인스턴스 2` | 418B | score_parameter |  |
| `0xe3a430` | 클로저(부모 aux) | `인스턴스 2` | 418B | score_parameter |  |
| `0xe3a5e0` | 클로저(부모 aux) | `인스턴스 2` | 418B | score_parameter |  |
| `0xd7ac00` | 클로저(부모 aux) | `인스턴스 2` | 416B | tower_discipline |  |
| `0xcae8d0` | 클로저(부모 aux) | `인스턴스 1` | 414B | epic_poke |  |
| `0xd6c290` | 클로저(부모 aux) | `인스턴스 1` | 414B | battle_common |  |
| `0xe7a0a0` | 클로저(부모 aux) | `인스턴스 1` | 414B | serpen_poke |  |
| `0xd7a630` | 클로저(부모 aux) | `인스턴스 2` | 413B | tower_discipline |  |
| `0xe2fee0` | 클로저(부모 aux) | `인스턴스 2` | 410B | action_score |  |
| `0xd47620` | 클로저(부모 aux) | `인스턴스 1` | 407B | action_score |  |
| `0xcd74c0` | 클로저(부모 aux) | `인스턴스 1` | 405B | battle |  |
| `0xe32140` | 클로저(부모 aux) | `인스턴스 1` | 404B | hunt_and_poke |  |
| `0xe2fd50` | 클로저(부모 aux) | `인스턴스 1` | 394B | battle |  |
| `0xe30080` | 클로저(부모 aux) | `인스턴스 1` | 394B | battle |  |
| `0xe30210` | 클로저(부모 aux) | `인스턴스 1` | 394B | around |  |
| `0xe31960` | 클로저(부모 aux) | `인스턴스 1` | 394B | battle |  |
| `0xe322e0` | 클로저(부모 aux) | `인스턴스 1` | 394B | cover |  |
| `0xe32470` | 클로저(부모 aux) | `인스턴스 1` | 394B | ganker |  |
| `0xe32600` | 클로저(부모 aux) | `인스턴스 1` | 394B | ganker |  |
| `0xe32790` | 클로저(부모 aux) | `인스턴스 1` | 394B | ganker |  |
| `0xe32c60` | 클로저(부모 aux) | `인스턴스 1` | 394B | move_actions |  |
| `0xe70330` | 클로저(부모 aux) | `인스턴스 1` | 379B | chat |  |
| `0xea0d60` | 클로저(부모 aux) | `인스턴스 1` | 379B | lib |  |
| `0xe126b0` | 클로저(부모 aux) | `인스턴스 1` | 369B | fight_model |  |
| `0xd6bec0` | 클로저(부모 aux) | `인스턴스 1` | 338B | serpen |  |
| `0xcd7370` | 클로저(부모 aux) | `인스턴스 1` | 336B | serpen_hunt |  |
| `0xecc2a0` | 클로저(부모 aux) | `인스턴스 1` | 336B | epic_hunt |  |
| `0xe115c0` | 클로저(부모 aux) | `인스턴스 1` | 331B | battle |  |
| `0xca5ef0` | 클로저(부모 aux) | `인스턴스 1` | 318B | score_parameter |  |
| `0xca5170` | 클로저(부모 aux) | `인스턴스 1` | 305B | fight_check |  |
| `0xc80340` | 클로저(부모 aux) | `인스턴스 1` | 302B | utils |  |
| `0xd41fe0` | 클로저(부모 aux) | `인스턴스 1` | 288B | passive_jungle |  |
| `0xdc8b20` | 클로저(부모 aux) | `인스턴스 1` | 288B | ganker |  |
| `0xe3b570` | 클로저(부모 aux) | `인스턴스 2` | 273B | objective_discipline |  |
| `0xdc8c40` | 클로저(부모 aux) | `인스턴스 1` | 265B | ganker |  |
| `0xe12500` | 클로저(부모 aux) | `인스턴스 1` | 262B | fight_model |  |
| `0xdc8d50` | 클로저(부모 aux) | `인스턴스 1` | 239B | ganker |  |
| `0xcaea70` | 클로저(부모 aux) | `인스턴스 1` | 197B | battle |  |