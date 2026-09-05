# AI 계층 640함수 — 역할별 분류 (재현 우선순위용)

| 역할 | 함수 | 명령 | 비중 | 대체하려면 |
|---|---|---|---|---|
| 빌더 | 22 | 40660 | 10.6% | ★후보를 만드는 곳 = 무엇을 할 수 있는지를 결정 |
| 스코어러 | 20 | 15450 | 4.0% | ★★점수 = 무엇을 고를지를 결정. 조정의 핵심 |
| 진입점 | 38 | 29210 | 7.6% | 매 틱 흐름·재계획·경매. 구조가 커서 마지막에 |
| 리졸버 | 41 | 18161 | 4.7% | 선택된 액션 → 실제 명령. 게임 상태 조회가 많다 |
| 경로 | 172 | 115768 | 30.2% | 경로탐색. 판단이라기보다 계산 — 나중에 |
| 헬퍼 | 347 | 163870 | 42.8% | 술어·수치 산출. 위 것들을 재현하며 딸려 온다 |

**합계 640함수 / 383119명령**

## 스코어러 20개 (명령 많은 순)

| RVA | 명령 | 모듈 |
|---|---|---|
| `0xe65b10` | 3945 | `plan_legacy\handler\auction.rs` |
| `0xd57540` | 1988 | `action_score.rs` |
| `0xd59940` | 1844 | `action_score.rs` |
| `0xd31f20` | 1731 | `abstract_input.rs` |
| `0xdb6ff0` | 1400 | `small_action.rs` |
| `0xcc3080` | 948 | `plan_legacy\sub_plan\battle.rs` |
| `0xeade00` | 909 | `plan_legacy\sub_plan\death_battle.rs` |
| `0xe95ae0` | 353 | `plan_legacy\sub_plan\defense_nexus.rs` |
| `0xccf670` | 302 | `plan_legacy\sub_plan\serpen_hunt.rs` |
| `0xec7c90` | 302 | `plan_legacy\sub_plan\epic_hunt.rs` |
| `0xe388c0` | 229 | `plan_legacy\sub_plan\serpen_check.rs` |
| `0xe8b200` | 222 | `plan_legacy\sub_plan\line_defense.rs` |
| `0xccbb10` | 209 | `plan_legacy\sub_plan\line_safe.rs` |
| `0xeb5840` | 209 | `plan_legacy\sub_plan\line_wait.rs` |
| `0xe81390` | 184 | `plan_legacy\sub_plan\serpen_poke.rs` |
| `0xccaa10` | 179 | `plan_legacy\sub_plan\epic_poke.rs` |
| `0xe83080` | 175 | `plan_legacy\sub_plan\attack_nexus.rs` |
| `0xcc5a10` | 150 | `plan_legacy\sub_plan\jungle.rs` |
| `0xcc5fc0` | 101 | `plan_legacy\sub_plan\recall.rs` |
| `0xcbbca0` | 70 | `plan_legacy\sub_plan\steal.rs` |

## 빌더 22개 (명령 많은 순)

| RVA | 명령 | 모듈 |
|---|---|---|
| `0xe83390` | 4985 | `plan_legacy\sub_plan\line_defense.rs` |
| `0xcbbdb0` | 4742 | `plan_legacy\sub_plan\battle.rs` |
| `0xcb1ce0` | 4030 | `plan_legacy\sub_plan\serpen_hunt.rs` |
| `0xeaeda0` | 3960 | `plan_legacy\sub_plan\epic_hunt.rs` |
| `0xcc6170` | 2549 | `plan_legacy\sub_plan\epic_poke.rs` |
| `0xe7caf0` | 2535 | `plan_legacy\sub_plan\serpen_poke.rs` |
| `0xcb7540` | 2530 | `plan_legacy\sub_plan\hide.rs` |
| `0xea9a20` | 2386 | `plan_legacy\sub_plan\death_battle.rs` |
| `0xe928f0` | 2330 | `plan_legacy\sub_plan\defense_nexus.rs` |
| `0xe81680` | 1236 | `plan_legacy\sub_plan\attack_nexus.rs` |
| `0xcba660` | 1146 | `plan_legacy\sub_plan\steal.rs` |
| `0xcc4260` | 1139 | `plan_legacy\sub_plan\jungle.rs` |
| `0xcb03b0` | 1071 | `plan_legacy\sub_plan\epic_check.rs` |
| `0xe8b5e0` | 1057 | `plan_legacy\sub_plan\serpen_check.rs` |
| `0xeb43a0` | 998 | `plan_legacy\sub_plan\line_wait.rs` |
| `0xe80070` | 961 | `plan_legacy\sub_plan\serpen_poke.rs` |
| `0xcc9740` | 950 | `plan_legacy\sub_plan\epic_poke.rs` |
| `0xeb77a0` | 528 | `fight_check.rs` |
| `0xebc140` | 512 | `fight_check.rs` |
| `0xe2ac00` | 469 | `small_action\lane_minion.rs` |
| `0xeb8b80` | 417 | `fight_check.rs` |
| `0xcc5ca0` | 129 | `plan_legacy\sub_plan\recall.rs` |
