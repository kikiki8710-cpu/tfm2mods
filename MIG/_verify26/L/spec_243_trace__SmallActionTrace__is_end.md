---

### `243` trace::SmallActionTrace::is_end — 추격 소액션 종료 판정: 포기(abandoned) · 수명 만료(start+end_delay) · 대상 소멸 → true; 그 외엔 '대상이 보이는 상태에서 목표점 10000 이내 도달' 일 때만 true

| 항목 | 값 |
|---|---|
| id | `trace__SmallActionTrace__is_end` |
| 심볼 | `_RNvMs_NtNtCshdEBA0ozCnw_7game_ai12small_action5traceNtB4_16SmallActionTrace6is_end` |
| 소스 | `game-ai\src\small_action\trace.rs:406` |
| IR | `m02.ll` 63183~63322행 |
| 경로·가시성 | `game_ai::SmallActionTrace::is_end` · **in:game_ai** |
| 계층 | 기타 |
| exe | `cce0c0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::SmallActionTrace, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &SmallActionTrace(152B) (%0) | DI self = %0 | 4 |
| 1 | 2 | _rnd | &mut StdRng(320B) (%1) |  | 4 |
| 2 | 3 | player | &PlayerState(2528B) (%2) | DI player = %2 | 4 |
| 3 | 4 | data | &OperationData(24B) (%3) | DI data = %3 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn is_end(&self, _rnd, player, data) -> bool   // trace.rs:406~411
  // L407
  champ = data.cache.player_champion[player.info.team][player.info.position.as_index()].unwrap()   // +0x1e0 · 팀 bounds 패닉 · None → unwrap_failed
  // L408
  if self.abandoned { return true }                                  // +0x92
  // L409
  game = data.cache.game
  if !(self.end_delay + self.start_tick > game.tick()) { return true }   // 수명 만료(tick >= start+end_delay) · IR: add(end_delay,start_tick) ugt tick 이면 계속
  if game.get_entity_by_id(self.target).is_none() { return true }    // 대상 소멸(option.rs:682 is_none 인라인)
  // L410
  game.get_entity_by_id(self.target).is_none_or(|t| t.is_visible_from(champ))   // 재호출. Neutral 관측자→true · Player→ t.visible_state[team]==Visible
      && champ.distance_sq((self.goal_x, self.goal_y)) < 100000001     // dx=|x1-x2|, dy=|y1-y2| (abs_diff select) · dx²+dy²
  // L411

★극성: 대상이 시야 밖이면 클로저 false → `&&` 단락으로 **false**(종료 아님 — 마지막 goal 로 계속 이동). 시야 안이면 goal 도달(≤10000) 여부가 종료. 즉 '보이는 대상 옆까지 갔다' 만 정상 종료이고, 나머지 종료는 포기·만료·소멸.
★L409 의 두 조건이 한 if(`||`) 인지 별개 if 인지는 column 부재로 불명(외연 동일).
★rnd 미사용 · 쓰기 0.
```

**`mem` 메모리 접근 17건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | SmallActionTrace | 0x92 | abandoned | r | L408 (gep 146) bool — true 면 즉시 종료(v38 추격 포기 플래그 · _docs game_ai.txt:18) | 4 | OK |
| 1 | SmallActionTrace | 0x58 | start_tick | r | L409 (gep 88) | 4 | OK |
| 2 | SmallActionTrace | 0x80 | end_delay | r | L409 (gep 128) — start_tick+end_delay 가 수명 | 4 | OK |
| 3 | SmallActionTrace | 0x60 | target | r | L409/L410 (gep 96) get_entity_by_id 인자 · 두 번 호출 | 4 | OK |
| 4 | SmallActionTrace | 0x68 | goal_x | r | L410 (gep 104) x2 | 4 | OK |
| 5 | SmallActionTrace | 0x70 | goal_y | r | L410 (gep 112) y2 | 4 | OK |
| 6 | PlayerState | 0x930 | info.team | r | L407 (gep 2352) · ult 2 bounds | 4 | OK |
| 7 | PlayerState | 0x9c0 | info.position | r | L407 (gep 2496) i32 → as_index | 4 | OK |
| 8 | OperationData | 0x0 | cache | r |  | 4 | OK |
| 9 | AbstractGameWithCache | 0x0 | game.data_ptr | r | 팻포인터 앞절반 | 4 | OK |
| 10 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | +0x28 tick · +0x1f0 get_entity_by_id | 4 | OK |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | L407 (gep 480) · null → unwrap_failed | 4 | OK |
| 12 | Entity | 0x0 | team@tag (champ) | r | L410 → is_visible_from(entity.rs:1482) → player_team(1136): 1=Neutral 이면 '보임' 취급 | 4 | OK |
| 13 | Entity | 0x8 | team@Player.0 (champ) | r | L410 · ult 2 bounds | 4 | OK |
| 14 | Entity | 0x38 | visible_state[team]@tag (target) | r | L410 → is_visible(data.rs:122) stride 24B · ==0(Visible) 만 통과 | 4 | OK |
| 15 | Entity | 0x660 | x (champ) | r | L410 (gep 1632) x1 — distance_sq 인라인(common vec :3147) | 4 | OK |
| 16 | Entity | 0x668 | y (champ) | r | L410 (gep 1640) y1 | 4 | OK |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 100000001 | 410 | 임계 | 10000² + 1 — dist_sq `ult` ⟹ 챔프↔goal 거리 ≤ 10000(셀 32000 의 0.31) 이면 도달 | 4 |
| 1 | 2 | 407 | 임계 | player_champion 팀 축 bounds(ult 2) · visible_state 팀 축 bounds(entity.rs:1483) | 4 |
| 2 | 0 | 410 | 태그 | VisibleState 태그 0 = Visible | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | goal 도달 반경(제곱) | trace.rs:410 · m02.ll:63316 | 100000001 | 올리면 goal 에서 더 멀어도 '도달' 로 종료 → 추격이 일찍 끝나 재평가 빈도↑ · 내리면 더 붙어야 끝남 | 4 | 기존 |
| 1 | 수명 = start_tick + end_delay | trace.rs:409 · m02.ll:63230 (필드 +0x58/+0x80 · 값은 생성자 소관) | self.end_delay | end_delay 를 늘리면 대상이 안 보여도 추격 액션이 오래 유지 | 4 | 기존 |

<details><summary>`callees` 피호출자 14건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | is_end | game_ai::SmallActionUlt::is_end | in:game_ai | fn(&game_ai::SmallActionUlt, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action\cast.rs:289 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 106개 중 상위 3개 |
| 8 | is_end | game_ai::SmallActionPlay::is_end | pub | fn(&game_ai::SmallActionPlay, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action.rs:348 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 106개 중 상위 3개 |
| 9 | is_end | game_ai::SmallActionTrace::is_end | in:game_ai | fn(&game_ai::SmallActionTrace, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action\trace.rs:406 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 106개 중 상위 3개 |
| 10 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 1개**: `is_none_or`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:42785) · **형제 20개** (SmallActionTrace)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionTrace as std::clone::Clone>::clone | pub | game-ai\src\small_action\trace.rs:8 | True | fn(&game_ai::SmallActionTrace) -> game_ai::SmallActionTrace |
| 1 | <game_ai::SmallActionTrace as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\trace.rs:8 | True | fn(&game_ai::SmallActionTrace, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionTrace::new | pub | game-ai\src\small_action\trace.rs:42 | False | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace |
| 3 | game_ai::SmallActionTrace::with_avoid_enemy_zone | pub | game-ai\src\small_action\trace.rs:64 | True | fn(game_ai::SmallActionTrace) -> game_ai::SmallActionTrace |
| 4 | game_ai::SmallActionTrace::new_keep_range | pub | game-ai\src\small_action\trace.rs:69 | False | fn(&game_core::OperationData, usize, usize, u64) -> game_ai::SmallActionTrace |
| 5 | game_ai::SmallActionTrace::new_attack_range | pub | game-ai\src\small_action\trace.rs:75 | False | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace |
| 6 | game_ai::SmallActionTrace::new_attack_range_margin | pub | game-ai\src\small_action\trace.rs:79 | False | fn(&game_core::OperationData, usize, usize, u64) -> game_ai::SmallActionTrace |
| 7 | game_ai::SmallActionTrace::new_avoid_tower | pub | game-ai\src\small_action\trace.rs:100 | False | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace |
| 8 | game_ai::SmallActionTrace::new_attack_range_avoid_tower | pub | game-ai\src\small_action\trace.rs:106 | False | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace |
| 9 | game_ai::SmallActionTrace::avoid_unnecessary_tower | pub | game-ai\src\small_action\trace.rs:112 | True | fn(&game_ai::SmallActionTrace) -> bool |
| 10 | game_ai::SmallActionTrace::expected_goal_position | in:game_ai | game-ai\src\small_action\trace.rs:116 | False | fn(&game_ai::SmallActionTrace, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 11 | game_ai::SmallActionTrace::get_input | in:game_ai | game-ai\src\small_action\trace.rs:165 | False | fn(&mut game_ai::SmallActionTrace, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 12 | game_ai::SmallActionTrace::update_state | in:game_ai | game-ai\src\small_action\trace.rs:400 | False | fn(&mut game_ai::SmallActionTrace, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 13 | game_ai::SmallActionTrace::get_action | in:game_ai | game-ai\src\small_action\trace.rs:403 | True | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction |
| 14 | game_ai::SmallActionTrace::is_end | in:game_ai | game-ai\src\small_action\trace.rs:406 | False | fn(&game_ai::SmallActionTrace, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 15 | game_ai::SmallActionTrace::near_move_complete | in:game_ai | game-ai\src\small_action\trace.rs:413 | False | fn(&game_ai::SmallActionTrace, &game_core::Entity) -> bool |
| 16 | game_ai::SmallActionTrace::is_abandoned | pub | game-ai\src\small_action\trace.rs:418 | True | fn(&game_ai::SmallActionTrace) -> bool |
| 17 | game_ai::SmallActionTrace::applied_escape | pub | game-ai\src\small_action\trace.rs:423 | True | fn(&game_ai::SmallActionTrace) -> bool |
| 18 | game_ai::SmallActionTrace::target_id | pub | game-ai\src\small_action\trace.rs:427 | True | fn(&game_ai::SmallActionTrace) -> usize |
| 19 | game_ai::SmallActionTrace::mark_abandoned | pub | game-ai\src\small_action\trace.rs:432 | True | fn(&mut game_ai::SmallActionTrace) |

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | end_delay(+0x80)·goal_x/goal_y(+0x68/+0x70) 의 값이 어디서 세팅되는지 — 생성자/update_state(trace.rs:400) 소관 · 이 함수는 읽기만 | 4 |  |
| 1 | 표기 불가 | L409 가 소스에서 `if A \|\| B { return true }` 하나인지 두 개의 if 인지 — column 부재 · 외연 동일 · 표기 불가 | 4 |  |
| 2 | 미탐색 | get_entity_by_id 를 L409 와 L410 에서 두 번 호출하는 이유(소스가 `is_none()` 검사 후 다시 `is_none_or` 를 부르는 형태) — 동작상 두 번째 None 분기(%44→%60)는 첫 검사로 사실상 도달 불가이나 IR 에 남아 있음(사장 아님 · reach 는 live 처리) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

