---

### `248` cast::SmallActionUlt::is_end — 궁 소액션 종료 판정: 시작 후 5틱 경과 · 이미 시전(is_act) · 대상 엔티티 소멸 · 대상이 내 챔피언 팀 시야에 없음 — 넷 중 하나면 true

| 항목 | 값 |
|---|---|
| id | `cast__SmallActionUlt__is_end` |
| 심볼 | `_RNvMs5_NtNtCshdEBA0ozCnw_7game_ai12small_action4castNtB5_14SmallActionUlt6is_end` |
| 소스 | `game-ai\src\small_action\cast.rs:289` |
| IR | `m07.ll` 12554~12654행 |
| 경로·가시성 | `game_ai::SmallActionUlt::is_end` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e23750` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::SmallActionUlt, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &SmallActionUlt(24B) (%0) | DI self = %0 | 4 |
| 1 | 2 | _rnd | &mut StdRng(320B) (%1) | DI _rnd | 4 |
| 2 | 3 | _version | usize (%2) | DI _version | 4 |
| 3 | 4 | player | &PlayerState(2528B) (%3) | DI player = %3 | 4 |
| 4 | 5 | data | &OperationData(24B) (%4) | DI data = %4 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn is_end(&self, _rnd, _version, player, data) -> bool   // cast.rs:289~296
  game = data.cache.game                                           // OperationData+0x0 → cache+0x0/+0x8 팻포인터
  // L291 — tick() 가상호출이 무조건 먼저 실행되므로 소스 순서 = (틱 조건) || is_act
  if self.start_tick + 5 <= game.tick() || self.is_act { return true }   // tick = vtable+0x28 · select(%15, true, is_act)
  // L292
  game.get_entity_by_id(self.target)                               // vtable+0x1f0 → Option<&Entity>(null=None)
      .is_none_or(|t| {                                            // None → true(대상 소멸 = 종료)
          // L293
          champ = data.cache.player_champion[player.info.team][player.info.position.as_index()].unwrap()   // +0x1e0 · team ult 2 아니면 panic_bounds_check · None 이면 unwrap_failed
          // L294
          !t.is_visible_from(champ)                                // entity.rs:1482: match champ.player_team() { None(Neutral) => true(보임 취급) → 클로저 false, Some(team) => t.visible_state[team].is_visible() }
      })
  // L296

★해석: 클로저 결과 phi = [Neutral → false] [Player → visible_state[team].tag != 0]. 즉 대상이 내 팀 시야에 없으면(Invisible/Unknown) 종료. is_visible_from 이 Neutral 관측자에 대해 true 를 돌려주는 것은 IR 분기 방향에서 확정(태그 1 → 클로저 false).
★rnd/version 미사용 · 쓰기 0.
```

**`mem` 메모리 접근 12건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | SmallActionUlt | 0x0 | start_tick | r | L291 · +5 후 game.tick() 과 ule 비교 | 4 | OK |
| 1 | SmallActionUlt | 0x8 | target | r | L292 · get_entity_by_id 인자(엔티티 id) | 4 | OK |
| 2 | SmallActionUlt | 0x10 | is_act | r | L291 · bool(i8 trunc) — true 면 즉시 종료 | 4 | OK |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 4 | AbstractGameWithCache | 0x0 | game.data_ptr | r | dyn AbstractGame 팻포인터 앞절반 | 4 | OK |
| 5 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 뒷절반 — +0x28 tick · +0x1f0 get_entity_by_id 슬롯 호출(divtable AbstractGame) | 3 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | L293 (gep 480) · [[Option<&Entity>;5];2] 팀 stride 40B · 슬롯 8B · null=None → unwrap_failed 패닉. team 은 ult 2 bounds 패닉 | 4 | OK |
| 7 | PlayerState | 0x930 | info.team | r | L293 (gep 2352) | 4 | OK |
| 8 | PlayerState | 0x9c0 | info.position | r | L293 (gep 2496) i32 태그 → as_index(entity.rs:581) zext | 4 | OK |
| 9 | Entity | 0x0 | team@tag (champ) | r | L294 → player_team(entity.rs:1136): TeamType 태그 0=Player · 1=Neutral(trunc nuw i1) | 4 | OK |
| 10 | Entity | 0x8 | team@Player.0 (champ) | r | L294 · 내 챔피언 팀 인덱스(ult 2 bounds 패닉) | 4 | OK |
| 11 | Entity | 0x38 | visible_state[team]@tag (target) | r | L294 → is_visible_from(entity.rs:1483) → is_visible(data.rs:122): [VisibleState;2] stride 24B({i64,[2 x i64]}) · 태그 0=Visible/1=Invisible/2=Unknown · !=0 면 안 보임 | 4 | OK |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 5 | 291 | 계수 | start_tick + 5 <= game.tick() — 궁 액션 수명 5틱(60tps 기준 약 0.083초) 지나면 종료 | 4 |
| 1 | 2 | 293 | 임계 | player_champion 팀 축 bounds(ult 2) 및 visible_state 팀 축 bounds(entity.rs:1483) | 4 |
| 2 | 0 | 294 | 태그 | VisibleState 태그 0 = Visible — `!= 0` 이면 시야 밖 | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 궁 액션 수명(틱) | cast.rs:291 · m07.ll:12561 | 5 | 올리면 시전 못 한 채 더 오래 궁 액션을 붙들고 있음(재평가 지연) · 내리면 빨리 포기 | 4 | 기존 |

<details><summary>`callees` 피호출자 18건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | is_end | game_ai::SmallActionUlt::is_end | in:game_ai | fn(&game_ai::SmallActionUlt, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action\cast.rs:289 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 106개 중 상위 3개 |
| 6 | is_end | game_ai::SmallActionPlay::is_end | pub | fn(&game_ai::SmallActionPlay, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action.rs:348 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 106개 중 상위 3개 |
| 7 | is_end | game_ai::SmallActionTrace::is_end | in:game_ai | fn(&game_ai::SmallActionTrace, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action\trace.rs:406 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 106개 중 상위 3개 |
| 8 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 9 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 10 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 11 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 1개**: `is_none_or`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:42805) · **형제 9개** (SmallActionUlt)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionUlt as std::clone::Clone>::clone | pub | game-ai\src\small_action\cast.rs:239 | True | fn(&game_ai::SmallActionUlt) -> game_ai::SmallActionUlt |
| 1 | <game_ai::SmallActionUlt as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\cast.rs:239 | True | fn(&game_ai::SmallActionUlt, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionUlt::new | pub | game-ai\src\small_action\cast.rs:247 | False | fn(&game_core::OperationData, usize) -> game_ai::SmallActionUlt |
| 3 | game_ai::SmallActionUlt::movement_landing_position | in:game_ai | game-ai\src\small_action\cast.rs:255 | False | fn(&game_ai::SmallActionUlt, &game_core::Entity, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 4 | game_ai::SmallActionUlt::get_input | in:game_ai | game-ai\src\small_action\cast.rs:261 | False | fn(&mut game_ai::SmallActionUlt, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> |
| 5 | game_ai::SmallActionUlt::update_state | in:game_ai | game-ai\src\small_action\cast.rs:283 | False | fn(&mut game_ai::SmallActionUlt, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 6 | game_ai::SmallActionUlt::get_action | in:game_ai | game-ai\src\small_action\cast.rs:286 | True | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction |
| 7 | game_ai::SmallActionUlt::is_end | in:game_ai | game-ai\src\small_action\cast.rs:289 | False | fn(&game_ai::SmallActionUlt, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 8 | game_ai::SmallActionUlt::is_complete | in:game_ai | game-ai\src\small_action\cast.rs:298 | True | fn(&game_ai::SmallActionUlt) -> bool |

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L291 의 `\|\|` 좌우가 소스에서 `is_act \|\| tick조건` 순서일 가능성 — IR 은 tick() 가상호출을 무조건 실행하므로 `tick조건 \|\| is_act` 로 판정(가상호출을 조건 앞으로 호이스트할 수 없음). `column` 부재로 문면 확인은 불가 | 4 |  |
| 1 | 미탐색 | is_none_or 클로저가 소스에서 `!t.is_visible_from(champ)` 인지 `t.visible_state[..]` 직접 접근인지 — dloc 사슬(entity.rs:1482 is_visible_from → data.rs:122 is_visible)로 is_visible_from 경유 확정, 부정(!)은 phi 극성에서 확정 | 4 |  |
| 2 | 미탐색 | exe 0xe23750(2,118B) 진입부의 JT tail-jump 디스패처와 이 IR 본문의 대응(어느 케이스 분기가 이 함수인지) — IR 로만 명세, 진입부 확인은 ev1 단계 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

