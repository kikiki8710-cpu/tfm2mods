---

### `244` move_actions::SmallActionRecall::is_end — 귀환 소액션 종료 판정: 수명 만료(start+end_delay) 또는 goal 10000 이내 도달이면 true, 아니면 (version>1 && 내 팀 우물 사각형 안) 여부

| 항목 | 값 |
|---|---|
| id | `move_actions__SmallActionRecall__is_end` |
| 심볼 | `_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action12move_actionsNtB5_17SmallActionRecall6is_end` |
| 소스 | `game-ai\src\small_action\move_actions.rs:1032` |
| IR | `m08.ll` 99606~99740행 |
| 경로·가시성 | `game_ai::SmallActionRecall::is_end` · **in:game_ai** |
| 계층 | 기타 |
| exe | `dbd130` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::SmallActionRecall, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &SmallActionRecall(136B) (%0) | DI self = %0 | 4 |
| 1 | 2 | _rnd | &mut StdRng(320B) (%1) |  | 4 |
| 2 | 3 | version | usize (%2) | DI version = %2 | 4 |
| 3 | 4 | player | &PlayerState(2528B) (%3) |  | 4 |
| 4 | 5 | data | &OperationData(24B) (%4) |  | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn is_end(&self, _rnd, version, player, data) -> bool   // move_actions.rs:1032~1044
  // L1033
  champ = data.cache.player_champion[player.info.team][player.info.position.as_index()].unwrap()
  // L1039
  (lx, ly, rx, ry) = data.context.map.fountain(player.info.team)     // MapDef+0x6d70 + 32*team (map_def.rs:235 인라인)
  // L1040
  in_fountain = version > 1 && champ.x >= lx && champ.x <= rx && champ.y >= ly && champ.y <= ry   // 단락: version 거짓 → false(x/y 로드 안 함) · x 범위 밖 → false(y 로드 안 함)
  // L1041
  if game.tick() < self.start_tick + self.end_delay {               // tick = vtable+0x28 · add(end_delay,start_tick)
      // L1042
      if champ.distance_sq((self.goal_x, self.goal_y)) < 100000001 { in_fountain = true }   // DI: in_fountain = 1 (%85)
  } else { in_fountain = true }                                     // 수명 만료
  // L1044
  in_fountain

★의미: 귀환 액션은 ①시간 만료 ②goal(우물 목표점) 10000 이내 도달 ③(v2+) 내 팀 우물 사각형 안 진입 — 셋 중 하나면 끝. 사각형 판정은 version 게이트 뒤에만 있어 version<=1 이면 ①②만.
★rnd 미사용 · 쓰기 0. 소스가 `let mut in_fountain … ; if … { in_fountain = true }` 형태인지 `return true` 형태인지는 외연 동일(DI 가 %85 에서 `in_fountain = 1` 을 찍어 전자에 가까움).
```

**`mem` 메모리 접근 15건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | SmallActionRecall | 0x48 | start_tick | r | L1041 (gep 72) | 4 | OK |
| 1 | SmallActionRecall | 0x60 | end_delay | r | L1041 (gep 96) — start_tick+end_delay 수명 | 4 | OK |
| 2 | SmallActionRecall | 0x50 | goal_x | r | L1042 (gep 80) x2 | 4 | OK |
| 3 | SmallActionRecall | 0x58 | goal_y | r | L1042 (gep 88) y2 | 4 | OK |
| 4 | PlayerState | 0x930 | info.team | r | L1033 (gep 2352) · ult 2 bounds · fountains 인덱스로도 재사용(L1039) | 4 | OK |
| 5 | PlayerState | 0x9c0 | info.position | r | L1033 (gep 2496) → as_index | 4 | OK |
| 6 | OperationData | 0x0 | cache | r | L1033/L1041 | 4 | OK |
| 7 | OperationData | 0x8 | context | r | L1039 → GameContext | 4 | OK |
| 8 | GameContext | 0x20 | map | r | L1039 (gep 32) &MapDef | 4 | OK |
| 9 | MapDef | 0x6d70 | fountains[team] | r | L1039 (gep 28016) → map_def.rs:235 fountain(team) 인라인 · [(lx,ly,rx,ry);2] stride 32B ({i64,i64,i64,i64}) · +0 lx · +8 ly · +16 rx · +24 ry | 4 | OK |
| 10 | AbstractGameWithCache | 0x0 | game.data_ptr | r | L1041 tick 호출 | 4 | OK |
| 11 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | +0x28 tick | 4 | OK |
| 12 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | L1033 (gep 480) · null → unwrap_failed | 4 | OK |
| 13 | Entity | 0x660 | x (champ) | r | L1040 사각형 검사 · L1042 distance_sq (gep 1632) | 4 | OK |
| 14 | Entity | 0x668 | y (champ) | r | L1040 · L1042 (gep 1640) | 4 | OK |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 1040 | 임계 | version > 1 (ugt 1) — 우물 사각형 판정 게이트(v2+). 현행 2 → 참 | 4 |
| 1 | 100000001 | 1042 | 임계 | 10000² + 1 — dist_sq ult ⟹ goal 까지 ≤ 10000 이면 도달 | 4 |
| 2 | 2 | 1033 | 임계 | player_champion 팀 축 bounds(ult 2) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | goal 도달 반경(제곱) | move_actions.rs:1042 · m08.ll:99729 | 100000001 | 올리면 우물 목표점에서 더 멀어도 귀환 액션이 끝남(우물 진입 전 종료 가능) · 내리면 더 정확히 붙어야 끝남 | 4 | 기존 |
| 1 | 우물 사각형 판정 게이트 | move_actions.rs:1040 · m08.ll:99658 | version > 1 | version 을 1 이하로 주면 사각형 진입으로는 안 끝나고 도달/만료만 남음 | 4 | 기존 |

<details><summary>`callees` 피호출자 12건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | fountain | game_core::MapDef::fountain | pub | fn(&game_core::MapDef, usize) -> (u64, u64, u64, u64) | game-core\src\simulation\map_def.rs:234 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | is_end | game_ai::SmallActionRecall::is_end | in:game_ai | fn(&game_ai::SmallActionRecall, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action\move_actions.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 8 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 9 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 10 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 11 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

**호출처 1곳** (m11.ll:42546) · **형제 11개** (SmallActionRecall)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionRecall as std::clone::Clone>::clone | pub | game-ai\src\small_action\move_actions.rs:645 | True | fn(&game_ai::SmallActionRecall) -> game_ai::SmallActionRecall |
| 1 | <game_ai::SmallActionRecall as std::default::Default>::default | pub | game-ai\src\small_action\move_actions.rs:645 | True | fn() -> game_ai::SmallActionRecall |
| 2 | <game_ai::SmallActionRecall as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\move_actions.rs:645 | True | fn(&game_ai::SmallActionRecall, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::SmallActionRecall::new | pub | game-ai\src\small_action\move_actions.rs:659 | False | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRecall |
| 4 | game_ai::SmallActionRecall::get_input | in:game_ai | game-ai\src\small_action\move_actions.rs:681 | False | fn(&mut game_ai::SmallActionRecall, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 5 | game_ai::SmallActionRecall::is_stalled | pub | game-ai\src\small_action\move_actions.rs:1019 | True | fn(&game_ai::SmallActionRecall, usize) -> bool |
| 6 | game_ai::SmallActionRecall::merge | in:game_ai | game-ai\src\small_action\move_actions.rs:1023 | False | fn(&mut game_ai::SmallActionRecall, game_ai::SmallActionRecall) |
| 7 | game_ai::SmallActionRecall::update_state | in:game_ai | game-ai\src\small_action\move_actions.rs:1026 | False | fn(&mut game_ai::SmallActionRecall, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 8 | game_ai::SmallActionRecall::get_action | in:game_ai | game-ai\src\small_action\move_actions.rs:1029 | True | fn(&game_ai::SmallActionRecall) -> game_core::SmallAction |
| 9 | game_ai::SmallActionRecall::is_end | in:game_ai | game-ai\src\small_action\move_actions.rs:1032 | False | fn(&game_ai::SmallActionRecall, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 10 | game_ai::SmallActionRecall::near_move_complete | in:game_ai | game-ai\src\small_action\move_actions.rs:1046 | False | fn(&game_ai::SmallActionRecall, &game_core::Entity) -> bool |

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | start_tick/end_delay/goal_x/goal_y 의 세팅처(생성자·update_state move_actions.rs:1026) — 이 함수는 읽기만 | 4 |  |
| 1 | 미탐색 | L1040 의 `&&` 4항 순서(x 범위 → y 범위)는 분기 방향(%33→%42)으로 확정 · lx/ly/rx/ry 의 튜플 위치는 DI 이름(lx@+0 · ly@+8 · rx@+16 · ry@+24)과 tcxdict fountains[0].0~.3 대응 | 3 |  |
| 2 | 표기 불가 | L1041~L1044 의 소스 문형(mut 변수 갱신 vs 조기 return) — 외연 동일 · 표기 불가 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

