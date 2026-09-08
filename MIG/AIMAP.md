# game_ai 계층 상관관계 지도 (게임 0.5.8 · 자동생성 `MIG\aimap.py`)

> ⚠호출 그래프는 **직접 call 만** 담는다(Ghidra 콜리 표). vtable·간접 호출은 빠져 있어 "도달 가능" 수치는 **하한**이다. 모듈 간 호출은 SDK IR 심볼로 따로 집계했다.

## 1. 전체

| | 함수 | 바이트 | 비중 |
|---|---|---|---|
| AI 계층 전체 | 640 | 1,751,943 | 100% |
| judge 훅 = 결정 지점 | 28 | 106,754 | 6.1% |
| 결정 지점에서 도달 = **모드가 소유해야 할 코드** | 51 | 138,421 | 7.9% |
| 도달 밖(다른 결정의 하부·미발화) | 589 | 1,613,522 | 92.1% |

## 2. 모듈별 (소유 바이트 순)

| 모듈 | 함수 | 바이트 | 훅 | 소유 | 소유B | 재현흔적 |
|---|---|---|---|---|---|---|
| `position_eval` | 12 | 47,504 | 2 | 7 | 35,693 | 3 |
| `action_score` | 8 | 39,385 | 2 | 4 | 29,654 | 5 |
| `buff_value` | 10 | 15,967 | 6 | 6 | 12,032 | 8 |
| `plan_legacy/old/passive_line` | 7 | 33,246 | 1 | 2 | 9,855 | 2 |
| `score_parameter` | 17 | 41,851 | 0 | 1 | 7,241 | 1 |
| `plan_legacy/team_plan/objective_helpers` | 12 | 9,800 | 2 | 6 | 5,604 | 6 |
| `plan_legacy/old/fight_model` | 18 | 27,773 | 1 | 3 | 5,183 | 2 |
| `plan_legacy/old/single_line` | 3 | 13,579 | 1 | 1 | 5,111 | 1 |
| `plan_legacy/team_plan/objective_discipline` | 11 | 46,101 | 2 | 4 | 5,104 | 4 |
| `plan_legacy/old/serpen/hunt_and_poke` | 3 | 4,891 | 1 | 1 | 3,471 | 1 |
| `plan_legacy/old/epic/hunt_and_poke` | 4 | 4,878 | 1 | 1 | 3,413 | 1 |
| `utils` | 14 | 21,101 | 1 | 2 | 3,028 | 2 |
| `plan_legacy/old/passive_jungle` | 15 | 25,047 | 1 | 1 | 2,993 | 1 |
| `plan_legacy/old/defense_nexus` | 9 | 16,796 | 1 | 2 | 2,900 | 5 |
| `fight_check` | 15 | 25,857 | 0 | 2 | 1,251 | 1 |
| `plan_legacy/old/attack_nexus` | 1 | 1,049 | 1 | 1 | 1,049 | 1 |
| `small_action/trace` | 3 | 6,104 | 0 | 1 | 1,029 | 1 |
| `lib` | 14 | 24,330 | 1 | 1 | 932 | 1 |
| `small_action` | 8 | 29,291 | 1 | 1 | 729 | 1 |
| `plan_legacy/old/line_gank/ganker` | 10 | 13,993 | 0 | 1 | 698 | 0 |
| `plan_legacy/old/epic/hunt_and_battle` | 2 | 934 | 1 | 1 | 591 | 1 |
| `plan_legacy/old/serpen/hunt_and_battle` | 2 | 934 | 1 | 1 | 591 | 1 |
| `plan_legacy/sub_plan/steal` | 2 | 5,377 | 1 | 1 | 269 | 1 |
| `abstract_input` | 7 | 16,748 | 0 | 0 | 0 | 0 |
| `free_dist` | 22 | 38,975 | 0 | 0 | 0 | 0 |
| `goal_data` | 16 | 24,705 | 0 | 0 | 0 | 0 |
| `lane_economy` | 4 | 5,292 | 0 | 0 | 0 | 0 |
| `path_field` | 48 | 133,350 | 0 | 0 | 0 | 0 |

## 3. 결정 지점이 소유하는 범위

| 결정 지점 | 모듈 | 자기 B | 도달 함수 | 도달 B | 걸치는 모듈 |
|---|---|---|---|---|---|
| `0xd851d0` | `position_eval` | 30,055 | 8 | 42,770 | `fight_check`, `position_eval`, `score_parameter` |
| `0xd5bbf0` | `action_score` | 19,586 | 6 | 31,952 | `action_score`, `buff_value`, `utils` |
| `0xd57540` | `action_score` | 9,212 | 21 | 87,963 | `action_score`, `buff_value`, `fight_check`, `position_eval` … |
| `0xdffa10` | `buff_value` | 6,720 | 1 | 6,720 | `buff_value` |
| `0xd2c5d0` | `plan_legacy/old/passive_line` | 5,182 | 1 | 5,182 | `plan_legacy/old/passive_line` |
| `0xd781e0` | `plan_legacy/old/single_line` | 5,111 | 1 | 5,111 | `plan_legacy/old/single_line` |
| `0xdf0e90` | `plan_legacy/old/serpen/hunt_and_poke` | 3,471 | 13 | 19,784 | `lib`, `plan_legacy/old/passive_line`, `plan_legacy/old/serpen/hunt_and_poke`, `plan_legacy/team_plan/objective_discipline` … |
| `0xdd5db0` | `plan_legacy/team_plan/objective_discipline` | 3,467 | 6 | 6,255 | `plan_legacy/team_plan/objective_discipline`, `plan_legacy/team_plan/objective_helpers` |
| `0xdefcd0` | `plan_legacy/old/epic/hunt_and_poke` | 3,413 | 13 | 19,726 | `lib`, `plan_legacy/old/epic/hunt_and_poke`, `plan_legacy/old/passive_line`, `plan_legacy/team_plan/objective_discipline` … |
| `0xd2e500` | `plan_legacy/old/passive_jungle` | 2,993 | 1 | 2,993 | `plan_legacy/old/passive_jungle` |
| `0xe07430` | `plan_legacy/old/fight_model` | 2,848 | 3 | 5,183 | `plan_legacy/old/fight_model` |
| `0xd2da10` | `plan_legacy/old/defense_nexus` | 2,104 | 2 | 2,900 | `plan_legacy/old/defense_nexus` |
| `0xe02540` | `buff_value` | 1,658 | 1 | 1,658 | `buff_value` |
| `0xe01450` | `buff_value` | 1,394 | 1 | 1,394 | `buff_value` |
| `0xcaf9f0` | `plan_legacy/old/attack_nexus` | 1,049 | 23 | 42,312 | `lib`, `plan_legacy/old/attack_nexus`, `plan_legacy/old/defense_nexus`, `plan_legacy/old/epic/hunt_and_battle` … |
| `0xe04400` | `buff_value` | 960 | 1 | 960 | `buff_value` |
| `0xe7a8c0` | `lib` | 932 | 1 | 932 | `lib` |
| `0xd84db0` | `position_eval` | 841 | 10 | 44,185 | `fight_check`, `position_eval`, `score_parameter` |
| `0xe23170` | `small_action` | 729 | 2 | 1,758 | `small_action`, `small_action/trace` |
| `0xc87fe0` | `utils` | 724 | 2 | 3,028 | `utils` |
| `0xe02020` | `buff_value` | 686 | 1 | 686 | `buff_value` |
| `0xdcc100` | `plan_legacy/team_plan/objective_discipline` | 631 | 1 | 631 | `plan_legacy/team_plan/objective_discipline` |
| `0xe019d0` | `buff_value` | 614 | 1 | 614 | `buff_value` |
| `0xccc010` | `plan_legacy/old/epic/hunt_and_battle` | 591 | 2 | 1,523 | `lib`, `plan_legacy/old/epic/hunt_and_battle` |
| `0xccc3c0` | `plan_legacy/old/serpen/hunt_and_battle` | 591 | 2 | 1,523 | `lib`, `plan_legacy/old/serpen/hunt_and_battle` |
| `0xeca200` | `plan_legacy/team_plan/objective_helpers` | 559 | 4 | 4,453 | `plan_legacy/team_plan/objective_helpers` |
| `0xec9bf0` | `plan_legacy/team_plan/objective_helpers` | 364 | 1 | 364 | `plan_legacy/team_plan/objective_helpers` |
| `0xcbbca0` | `plan_legacy/sub_plan/steal` | 269 | 1 | 269 | `plan_legacy/sub_plan/steal` |

## 4. 진입점 (아무도 안 부르는 함수 = 상위 결정 후보) — 상위 20

| RVA | 모듈 | 바이트 | 명령 | 훅? |
|---|---|---|---|---|
| `0xe5d5d0` | `plan_legacy/handler/engage` | 32,177 | 6763 |  |
| `0xe83390` | `plan_legacy/sub_plan/line_defense` | 24,820 | 4985 |  |
| `0xdda220` | `plan_legacy/team_plan/objective_discipline` | 24,674 | 5330 |  |
| `0xcbbdb0` | `plan_legacy/sub_plan/battle` | 23,506 | 4742 |  |
| `0xd4be20` | `plan_legacy/old/death_battle` | 23,311 | 4661 |  |
| `0xcb1ce0` | `plan_legacy/sub_plan/serpen_hunt` | 20,647 | 4030 |  |
| `0xd52030` | `plan_legacy/old/single_battle` | 20,218 | 4176 |  |
| `0xeaeda0` | `plan_legacy/sub_plan/epic_hunt` | 20,037 | 3960 |  |
| `0xd478c0` | `plan_legacy/old/death_battle` | 16,861 | 3803 |  |
| `0xd28800` | `plan_legacy/old/passive_line` | 14,751 | 3089 |  |
| `0xdcee40` | `plan_legacy/team_plan` | 14,041 | 3112 |  |
| `0xcc6170` | `plan_legacy/sub_plan/epic_poke` | 12,777 | 2549 |  |
| `0xe7caf0` | `plan_legacy/sub_plan/serpen_poke` | 12,629 | 2535 |  |
| `0xea9a20` | `plan_legacy/sub_plan/death_battle` | 11,686 | 2386 |  |
| `0xcb7540` | `plan_legacy/sub_plan/hide` | 11,547 | 2530 |  |
| `0xe928f0` | `plan_legacy/sub_plan/defense_nexus` | 11,291 | 2330 |  |
| `0xd2f180` | `plan_legacy/old/passive_jungle` | 10,479 | 2225 |  |
| `0xc91770` | `plan_legacy/sub_plan/line_defense` | 10,217 | 2190 |  |
| `0xdb9430` | `plan_legacy/old/line_gank/ganker` | 9,919 | 2031 |  |
| `0xe59b20` | `plan_legacy/handler/chat` | 9,416 | 1912 |  |

## 5. 미소유 허브 (여러 곳에서 불리는데 소유 밖) — 상위 20

| RVA | 모듈 | 피호출 | 바이트 |
|---|---|---|---|
| `0xd20830` | `path_finder` | 43 | 897 |
| `0xdc8090` | `free_dist` | 27 | 938 |
| `0xd214e0` | `path_finder` | 19 | 548 |
| `0xdc8550` | `small_action/around` | 17 | 1,126 |
| `0xeb8b80` | `fight_check` | 11 | 1,874 |
| `0xc8c620` | `path_finder` | 11 | 549 |
| `0xd399a0` | `utils` | 11 | 503 |
| `0xd1bda0` | `path_finder` | 10 | 5,986 |
| `0xd1faa0` | `path_finder` | 10 | 347 |
| `0xd1d9c0` | `path_finder` | 9 | 658 |
| `0xd34750` | `abstract_input` | 8 | 3,440 |
| `0xe0bf70` | `plan_legacy/old/fight_model` | 8 | 915 |
| `0xd9ddc0` | `small_action/cast` | 7 | 5,533 |
| `0xd97300` | `tower_discipline` | 7 | 1,019 |
| `0xd59940` | `action_score` | 6 | 8,506 |
| `0xe7b640` | `lib` | 6 | 856 |
| `0xdfb840` | `plan_legacy/old/battle` | 5 | 9,320 |
| `0xc8bde0` | `path_finder` | 5 | 764 |
| `0xd1f880` | `path_finder` | 5 | 529 |
| `0xdc3240` | `small_action/move_actions` | 4 | 14,264 |

## 6. 소유 범위 안인데 소스에 흔적 없음 (= 진짜 구멍 후보)

| RVA | 모듈 | 바이트 | 명령 |
|---|---|---|---|
| `0xc7f640` | `position_eval` | 2,332 | 365 |
| `0xc9a790` | `plan_legacy/old/fight_model` | 1,452 | 334 |
| `0xc9eb60` | `plan_legacy/old/fight_model` | 883 | 217 |
| `0xc7f370` | `position_eval` | 670 | 177 |
| `0xc872f0` | `fight_check` | 625 | 165 |
| `0xc7f0f0` | `position_eval` | 591 | 159 |
| `0xc875a0` | `fight_check` | 626 | 157 |
| `0xc7ee80` | `position_eval` | 574 | 140 |
| `0xdb8ba0` | `plan_legacy/old/line_gank/ganker` | 698 | 121 |
| `0xca1fa0` | `action_score` | 449 | 118 |

## 7. 모듈 간 호출 (SDK IR 심볼 기준, 상위 25)

| 호출자 모듈 | 피호출 모듈 | 건수 |
|---|---|---|
| `plan_legacy7handler7auction` | `small_action15` | 69 |
| `plan_legacy8sub_plan9epic_hunt` | `small_action15` | 52 |
| `plan_legacy8sub_plan12line_defense` | `small_action15` | 46 |
| `plan_legacy8sub_plan11serpen_poke` | `small_action15` | 42 |
| `plan_legacy8sub_plan11serpen_hunt` | `small_action15` | 38 |
| `plan_legacy3old12death_battle` | `plan_legacy3old6battle16max_range_cached` | 38 |
| `plan_legacy3old6battle` | `path_finder20is_enemy_well_danger` | 34 |
| `plan_legacy8sub_plan9epic_poke` | `small_action15` | 30 |
| `plan_legacy7handler` | `plan_legacy5types` | 25 |
| `abstract_input16get_input_target` | `position_eval22position_score_at_cell` | 24 |
| `small_action6around` | `path_finder` | 24 |
| `plan_legacy9team_plan` | `plan_legacy5types` | 24 |
| `small_action` | `small_action6around` | 24 |
| `plan_legacy8sub_plan6battle18base_battle_action` | `small_action4cast` | 22 |
| `plan_legacy8sub_plan6battle18base_battle_action` | `small_action15` | 22 |
| `plan_legacy8sub_plan12death_battle18base_battle_action` | `small_action4cast` | 22 |
| `plan_legacy8sub_plan12death_battle18base_battle_action` | `small_action15` | 22 |
| `plan_legacy7handler6engage` | `plan_legacy3old6battle` | 21 |
| `plan_legacy7handler6engage` | `plan_legacy3old11fight_model21is_ignored_well_enemy` | 20 |
| `plan_legacy8sub_plan6battle` | `small_action15` | 18 |
| `plan_legacy7handler4chat` | `plan_legacy7handler` | 17 |
| `plan_legacy8sub_plan12death_battle` | `small_action15` | 17 |
| `plan_legacy8sub_plan11serpen_hunt` | `small_action12move_actions` | 16 |
| `plan_legacy9team_plan20objective_discipline` | `plan_legacy3old11fight_model21is_ignored_well_enemy` | 16 |
| `plan_legacy9team_plan` | `plan_legacy9team_plan18objective_handlers` | 16 |

