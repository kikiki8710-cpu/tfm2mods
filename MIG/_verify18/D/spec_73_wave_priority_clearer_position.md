---

### `73` wave_priority_clearer_position — 웨이브 우선 정리 담당 포지션 선정 — 적 미니언 중 우리 넥서스 최근접(없으면 라인 1차 타워 위치)에 가장 가까운 HP 30%+ 아군 포지션

| 항목 | 값 |
|---|---|
| id | `objective_helpers__wave_priority_clearer_position` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers30wave_priority_clearer_position` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:245` |
| IR | `m15.ll` 54086~54632행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::objective_helpers::wave_priority_clearer_position` · **in:game_ai** |
| 계층 | 기타 |
| exe | `ec9de0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> std::option::Option<usize>
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽음 | 4 |
| 1 | 2 | data | &OperationData(24B) | cache(+0x0) 만 읽음 | 4 |
| 2 | 3 | line | LineType(i8) | 0=Top 1=Mid 2=Bottom · line_minions 인덱스 및 폴백 타워 위치 선택. 3 이상이면 unreachable(L238 switch default) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn wave_priority_clearer_position(player, data, line) -> Option<usize>
let team = player.info.team;
// L246  target = wave_priority_clear_target(player, data, line)  — objective_helpers.rs:258~266 전부 인라인
//   L258  let nexus = cache.nexus[team]                         // +0x170 · None → 폴백
//   L259  let minions = cache.line_minions(line)[1 - team]      // simulation.rs:1807 · 0x10 + line*0x40 + enemy*0x20 (bumpalo Vec<&Entity>)
//   L260  minions.iter().min_by_key(|m| dist_sq(m, nexus))      // closure wave_priority_clear_target0 (인라인) · 빈 목록 → None
//   L261  Some(m) → target = (m.x, m.y)
//   L265  None(넥서스 없음 | 적 미니언 없음) → target = first_tower_position(team, line):
//         Top: team0 (48000,272000) / team1 (272000,48000)   Mid: team0 (368000,592000)/team1 (592000,368000)   Bottom: team0 (688000,912000)/team1 (912000,688000)
// L249~253
(0..5).filter_map(|pos| {
    let c = cache.player_champion[team][pos]?;                 // L249
    if !(c.hp * 100 / c.stat_cached.hp > 29) { return None; }  // L250 · max_hp==0 이면 div_by_zero 패닉
    Some((pos, dist_sq(c, target)))                            // L251
})
.min_by_key(|&(pos, d)| (d, pos))                              // L253 · 키 = (거리², 포지션) — 동거리면 낮은 포지션
.map(|(pos, _)| pos)                                           // L254~255
```

**`mem` 메모리 접근 11건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L246/258 · nexus[team] · enemy = 1-team · 폴백 타워 위치 팀 선택 · player_champion[team] | 4 | OK |
| 1 | OperationData | 0x0 | cache | r |  | 4 | OK |
| 2 | AbstractGameWithCache | 0x170 | nexus[2]: Option<&Entity> | r | L258 (wave_priority_clear_target 인라인) · nexus[team] None 이면 폴백 위치 | 4 | OK |
| 3 | AbstractGameWithCache | 0x10 | top_minions[2] (bumpalo Vec<&Entity>: ptr+0x0 / len+0x18) | r | L259 line_minions(simulation.rs:1807) 인라인 — 오프셋 = 0x10 + line*0x40 + team*0x20 (line<<6\|16 로 접힘) · [enemy_team] | 4 | OK |
| 4 | AbstractGameWithCache | 0x50 | mid_minions[2] | r | line=1 | 4 | OK |
| 5 | AbstractGameWithCache | 0x90 | bottom_minions[2] | r | line=2 (tcxdict 로 0x10/0x50 확인, 0x90 은 stride 산술) | 3 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[2][5]: Option<&Entity> | r | L249 [team][0..5] | 4 | OK |
| 7 | Entity | 0x660 | x | r | L260 미니언↔넥서스 거리 · L261 target 좌표 · L251 아군↔target 거리 | 4 | OK |
| 8 | Entity | 0x668 | y | r | 동상 | 4 | OK |
| 9 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | L250 hp*100/stat_cached.hp · 0 이면 div_by_zero 패닉 | 4 | OK |
| 10 | Entity | 0x670 | hp (현재) | r | L250 | 4 | OK |

**`consts` 상수 12건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 29 | 250 | 임계 | HP% 임계 — hp*100/max_hp > 29 (즉 30% 이상)인 아군 챔피언만 후보 | 4 |
| 1 | 100 | 250 | 계수 | HP 백분율 계산 계수 | 4 |
| 2 | 5 | 249 | 산출값 | 포지션 수 0..5 — 판정 아님 | 4 |
| 3 | 2 | 258 | 태그 | 팀 인덱스 bounds — 판정 아님. L238 switch 의 LineType 태그 2=Bottom. (QC C1 shl 경고는 `shl i8 %2, 6` 의 레지스터 %2 오매칭 — 시프트량 6 = line*0x40 stride 접힘, 판정 아님) | 4 |
| 4 | 1 | 259 | 태그 | enemy_team = 1 - team · L238 switch LineType 1=Mid · 반환 Some 태그 1 | 4 |
| 5 | 0 | 265 | 태그 | team==0 (블루) 폴백 좌표 선택 · L238 switch LineType 0=Top · 반환 None 태그 0 | 4 |
| 6 | 48000 | 239 | 산출값 | 폴백 Top 1차 타워 위치(first_tower_position→tower.rs:105 get_position 인라인): team0 (x=48000,y=272000) / team1 (x=272000,y=48000) — 1.5셀/8.5셀 | 4 |
| 7 | 272000 | 239 | 산출값 | 동상(8.5셀) | 4 |
| 8 | 368000 | 240 | 산출값 | 폴백 Mid 1차 타워(tower.rs:110): team0 (368000,592000) / team1 (592000,368000) — 11.5셀/18.5셀 | 4 |
| 9 | 592000 | 240 | 산출값 | 동상 | 4 |
| 10 | 688000 | 241 | 산출값 | 폴백 Bottom 1차 타워(tower.rs:115): team0 (688000,912000) / team1 (912000,688000) — 21.5셀/28.5셀 | 4 |
| 11 | 912000 | 241 | 산출값 | 동상 | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 정리 담당 후보 HP% 하한 | objective_helpers.rs:250 | 29 | 올리면 더 건강한 아군만 웨이브 정리에 배정(후보 줄어 None 잦아짐). 내리면 저체력 아군도 배정 | 4 | 기존 |

<details><summary>`callees` 피호출자 4건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | first_tower_position | game_ai::plan_legacy::team_plan::first_tower_position | pub | fn(game_core::LineType, usize) -> (u64, u64) | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:237 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 1 | line_minions | game_core::AbstractGameWithCache::<'a, 'b>::line_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> &bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1806 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | wave_priority_clear_target | game_ai::plan_legacy::team_plan::objective_helpers::wave_priority_clear_target | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> (u64, u64) | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:257 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | wave_priority_clearer_position | game_ai::plan_legacy::team_plan::objective_helpers::wave_priority_clearer_position | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> std::option::Option<usize> | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:245 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 4개**: `dist_sq`, `team0`, `team1`, `wave_priority_clear_target0`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:37594) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | line_minions(simulation.rs:1807)·first_tower_position(objective_helpers.rs:238~241)·tower.rs get_position(105/110/115) 은 인라인 잔여로만 확인 — 원 본문 미독해 | 4 |  |
| 1 | 미탐색 | bottom_minions 오프셋 0x90 은 stride 산술(0x10+2*0x40) — tcxdict 로는 top(0x10)/mid(0x50) 만 직접 확인 | 3 |  |
| 2 | 미탐색 | L253 min_by_key 키가 (dist, pos) 튜플인 것은 망글 심볼 `keyTjyETyjE` + 폴드 초기상태 [dist,pos,pos,dist] 배치로 판정 — 동거리 타이브레이크가 pos 오름차순인 것은 튜플 비교 규칙 추론 | 4 |  |
| 3 | 미탐색 | line 인자 3 이상은 switch default → unreachable(UB) — LineType 이 3 variant 라 실제 도달 없음 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

