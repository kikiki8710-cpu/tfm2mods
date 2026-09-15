# r17 vtable 진입점 잔여 편성표 (생성 2026-09-16 · rvaname 실명 · 원장 §3-C 정정 · 진입점 45 = 본체 43 + JT 2)

> 출처: 지도 미명세 plan/act 195 → 클로저/인스턴스 132 제외 · NA 6 · 이미 편입/사장/기각/호스트 11 → 진입점 47. 서브트리 합집합 266 중 비경로 미명세 콜리 32 는 **전부 클로저/인스턴스**(부모 aux) · 경로 계층 172(path_finder 102 · path_field 48 · free_dist 22 = 520KB) 는 **r18** 로 분리(`_next\subtree_r17.md`).
> 합계 IR 줄 53,745 · exe 152,556 B · 거대(≥3,000줄) 5 = rootcut 분책 대상 · [D] 4 = ai_adjust judge 계층 DIFF=0 기록 있음(명세만 · ev1 인용)

| # | RVA | 함수 | IR | define~끝 | IR 줄수 | exe 크기 | internal | 계층 | 비고 |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `0xd8eff0` | `score_parameter::calculate_score_parameter` | m07.ll | 37862~46714 | 8853 | 25,381B |  | act | DONE 보류(미포팅 상위 5) — 25KB 거대 · rootcut 분책 대상 |
| 2 | `0xe65b10` | `auction::LegacyPlanHandler::get_small_action` | m13.ll | 45629~52585 | 6957 | 19,492B |  | act | LegacyPlanHandler::get_small_action 19KB — SmallActionPlay 실행기 루트 · 분책 대상 |
| 3 | `0xd28800` | `passive_line::PassiveLinePlan::update` | m04.ll | 19038~24726 | 5689 | 14,751B |  | plan(추정) |  |
| 4 | `0xe083c0` | `fight_model::resolve_fight_uncached` | m10.ll | 43974~47451 | 3478 | 8,841B | ✔ | plan(추정) | [D] ai_adjust judge DIFF=0 2,426,997(09-13) → 명세만 |
| 5 | `0xe900b0` | `AgentVerHamster::get_input` | m14.ll | 38800~41939 | 3140 | 7,884B |  | act |  |
| 6 | `0xd377a0` | `utils::build_minion_wave_snapshot` | m04.ll | 49922~52222 | 2301 | 6,286B |  | act |  |
| 7 | `0xcba660` | `steal::StealSubPlan::action_candidates` | m02.ll | 25360~27425 | 2066 | 5,108B |  | act(추정) |  |
| 8 | `0xeba9b0` | `fight_check::check_kill_die_tick_uncached` | m15.ll | 31120~33053 | 1934 | 4,429B | ✔ | act(추정) | [D] ai_adjust judge DIFF=0 5,182,148(09-06) → 명세만 |
| 9 | `0xe8e560` | `AgentVerHamster::update_small_action` | m14.ll | 35955~37839 | 1885 | 4,430B | ✔ | act | 지도 라벨 update_small_action 은 오라벨 — DONE: AgentVerHamster::new(팩토리 아님) · rvaname lib.rs:774 |
| 10 | `0xd26900` | `passive_line::PassiveLinePlan::v46_stage1` | m04.ll | 16364~18034 | 1671 | 5,047B | ✔ | plan(추정) |  |
| 11 | `0xcc3080` | `battle::BattleSubPlan::score` | m02.ll | 37119~38402 | 1284 | 4,491B |  | act |  |
| 12 | `0xd676c0` | `battle_common::v16_gambler_ult_cc_bonus` | m05.ll | 54567~55828 | 1262 | 2,502B |  | act |  |
| 13 | `0xe8a100` | `line_defense::LineDefenseSubPlan::unsafe_v19_non_champion_walkup` | m14.ll | 28602~29733 | 1132 | 3,432B |  | act(추정) |  |
| 14 | `0xd69f80` | `battle_common::v17_runaway_counterattack_bonus` | m05.ll | 59244~60293 | 1050 | 2,271B |  | act |  |
| 15 | `0xe8d6f0` | `AgentVerHamster::update_state` | m14.ll | 34283~35106 | 824 | 2,159B |  | act |  |
| 16 | `0xcc20a0` | `battle::score_paramet` | m02.ll | 36309~37116 | 808 | 4,059B |  | plan(추정) |  |
| 17 | `0xd27d50` | `passive_line::PassiveLinePlan::v46_stage2` | m04.ll | 18037~18818 | 782 | 2,190B | ✔ | plan(추정) |  |
| 18 | `0xd39ba0` | `utils::precompute_champion_powers` | m04.ll | 53234~53976 | 743 | 2,039B |  | act |  |
| 19 | `0xd3e660` | `defense_nexus::nexus_last_stand_uncached` | m04.ll | 60704~61396 | 693 | 5,151B |  | act(추정) |  |
| 20 | `0xe95ae0` | `defense_nexus::DefenseNexusSubPlan::score` | m14.ll | 48981~49570 | 590 | 1,537B |  | act |  |
| 21 | `0xd67060` | `battle_common::v21_defensive_cc_score` | m05.ll | 54048~54564 | 517 | 1,619B | ✔ | act |  |
| 22 | `0xe8fd70` | `AgentVerHamster::item_v26` | m14.ll | 38283~38797 | 515 | 818B | ✔ | plan(추정) |  |
| 23 | `0xe7b0b0` | `build_game_finish_check_state` | m14.ll | 7703~8214 | 512 | 1,244B | ✔ | plan(추정) |  |
| 24 | `0xd98ac0` | `tower_discipline::v47_tower_focus_position_dangerous` | m07.ll | 51323~51760 | 438 | 1,390B |  | act(추정) |  |
| 25 | `0xe35a40` | `objective_handlers::TeamPlan::handle_nexus_attack` | m09.ll | 8158~8578 | 421 | 393B |  | plan(추정) |  |
| 26 | `0xe8f850` | `AgentVerHamster::count_nearby_enemies` | m14.ll | 37842~38171 | 330 | 1,064B | ✔ | act |  |
| 27 | `0xccbb10` | `line_safe::LineSafeSubPlan::score` | m02.ll | 48427~48748 | 322 | 926B |  | act |  |
| 28 | `0xeb5840` | `line_wait::LineWaitSubPlan::score` | m15.ll | 22517~22838 | 322 | 926B |  | act |  |
| 29 | `0xd99f60` | `tower_discipline::v30_line_champion_action_tower_aggro_risk` | m07.ll | 52737~53029 | 293 | 689B |  | act(추정) |  |
| 30 | `0xd3fe50` | `defense_nexus::nexus_final_stand_uncached` | m04.ll | 61661~61940 | 280 | 1,906B |  | act(추정) | [D] ai_adjust judge dn_reach DONE(09-06 · 캐시시점 0.6%) → 명세만 |
| 31 | `0xcc5a10` | `jungle::JungleSubPlan::score` | m02.ll | 39903~40149 | 247 | 641B |  | act |  |
| 32 | `0xe83080` | `attack_nexus::AttackNexusSubPlan::score` | m14.ll | 20984~21208 | 225 | 778B |  | act |  |
| 33 | `0xd69120` | `battle_common::v15_can_keep_support_pressure` | m05.ll | 57757~57979 | 223 | 602B |  | act |  |
| 34 | `0xcc5fc0` | `recall::RecallSubPlan::score` | m02.ll | 40317~40506 | 190 | 432B |  | act |  |
| 35 | `0xd9a220` | `tower_discipline::v22_current_line_non_champion_action_tower_risk` | m07.ll | 53032~53218 | 187 | 563B |  | act(추정) |  |
| 36 | `0xdc2330` | `_NtNt7::around::SmallActionAroundstat` | m08.ll | 104740~104914 | 175 | 531B |  | act |  |
| 37 | `0xe8aeb0` | `line_defense::LineDefenseSubPlan::calculate_score_parameter_value` | m14.ll | 29736~29903 | 168 | 836B |  | act(추정) |  |
| 38 | `0xcc5ca0` | `recall::RecallSubPlan::action_candidates` | m02.ll | 40152~40314 | 163 | 571B |  | act(추정) |  |
| 39 | `0xcbbca0` | `steal::StealSubPlan::score` | m02.ll | 27428~27589 | 162 | 269B |  | act | [D] ai_adjust judge steal_score DIFF=0 7판(09-06) → 명세만 |
| 40 | `0xcce0c0` | `trace::SmallActionTrace::is_end` | m02.ll | 63183~63322 | 140 | 333B |  | act |  |
| 41 | `0xdbd130` | `move_actions::SmallActionRecall::is_end` | m08.ll | 99606~99740 | 135 | 291B |  | act |  |
| 42 | `0xe8e160` | `AgentVerHamster::upgrade_item` | m14.ll | 35109~35234 | 126 | 254B |  | plan(추정) |  |
| 43 | `0xe388c0` | `serpen_check::SerpenCheckSubPlan::score` | m14.ll | 31556~31675 | 120 | 966B |  |  | JT tail-jump 디스패처(DONE 보류) — 명세는 하고 ev1 은 진입부 확인 후(#150 부류 가능) |
| 44 | `0xe8fc80` | `AgentVerHamster::buy_item` | m14.ll | 38174~38280 | 107 | 230B |  | plan(추정) |  |
| 45 | `0xe23750` | `cast::SmallActionUlt::is_end` | m07.ll | 12554~12654 | 101 | 2,118B |  |  | JT tail-jump 디스패처(DONE 보류) — 명세는 하고 ev1 은 진입부 확인 후(#150 부류 가능) |

## 제외(진입점 후보였으나)

- `0xe33570` — resolve_fight_uncached 폴드 인스턴스(391B · 본체 0xe083c0 에 흡수)
- `0xe33700` — resolve_fight_uncached 폴드 인스턴스(391B)
- `0xd57420` — hp_at_tick 클로저(r13 판정 · 지문 본체 없음)
- `0xcaf9f0` — BigPlan::sub_plan JT 호스트 — #02 arm 이미 ev1 · 미편입
- `0xccbeb0` / `0xccc260` — Serpen/EpicHuntAndBattlePlan::update: 플랜 생성 코드 없음(#07 범위 밖 · 09-13) → 사장 제외