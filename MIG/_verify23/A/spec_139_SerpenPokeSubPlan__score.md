---

### `139` SerpenPokeSubPlan::score — 세르펜 포킹 서브플랜 점수 = 세르펜 없으면 0 · 아니면 interaction_score(미가시·도주계열 /2) + 150k 밖에서 세르펜 지향 보너스 5

| 항목 | 값 |
|---|---|
| id | `SerpenPokeSubPlan__score` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan11serpen_pokeNtB2_17SerpenPokeSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:508` |
| IR | `m14.ll` 18618~18963행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `e81390` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &SerpenPokeSubPlan | captures(none) — 본문에서 안 읽음 | 4 |
| 1 | 2 | version | usize | interaction_score 전달만 | 4 |
| 2 | 3 | parameter | &ScoreParameter(5384B) | interaction_score 전달만 | 4 |
| 3 | 4 | rnd | &mut StdRng(320B) | interaction_score 전달만 — 이 함수 자체 write 0건 | 4 |
| 4 | 5 | player | &PlayerState(2528B) | info.team(+0x930)·info.position 태그(+0x9c0) | 4 |
| 5 | 6 | data | &OperationData(24B) | cache→player_champion / game.get_game_mode·get_entity_by_id ; context→map.regions | 4 |
| 6 | 7 | action | &SmallActionPlay(184B) | get_action() 인라인(태그 +0xb1 니치) | 4 |
| 7 | 8 | debug | &mut DebugFrameData(224B) | interaction_score 전달만 — 이 함수 자체 write 0건 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // serpen_poke.rs:508
  let s = interaction_score(version, rnd, player, data, parameter, action, debug);   // L509 (m14.ll:18636) — 세르펜 부재여도 먼저 호출됨
  let champ = data.cache.player_champion[player.info.team][player.info.position].unwrap();   // L511
  // L512: let serpen = match game.get_game_mode() { Moba(m) => m.jungle_runner.serpen.live_list.first().and_then(|id| game.get_entity_by_id(*id)), _ => 패닉 }
  if serpen.is_none() { return 0 }     // L513 (18697 len==0 → 0 ; 18718 entity None → 0)
  // L518: let bonus = if champ.distance_sq(serpen) < 22500000001 { 0 }   // 150000 이내면 보너스 없음 (entity.rs:2158 → utils.rs:9 dx²+dy², 18740~18754)
    else match action.get_action() {    // L522 (small_action.rs:309 인라인)
      SmallAction::Around{target_id} /*Play 2·3·10*/ => { get_game_mode() 재호출(18809, Moba 아니면 패닉); serpen.live_list.first()==Some(&target_id) ? 5 : 0 (18877~18879; len 0 → 0) }   // L524
      SmallAction::AroundPosition{x,y} /*Play 4(16,24)·7(48,56)·8(8,16)·9(24,32)*/ => { map.regions[min(y/32000,29)][min(x/32000,29)] == 7 ? 5 : 0 }   // L531~532
      _ => 0
    };
  // L545: let visible = serpen.is_visible_from(champ) = champ.team Neutral(태그1) ? true : serpen.visible_state[champ.team] is Visible(태그0)
  let objective_score = if visible { s }                                      // 18950 phi: %120(Neutral)·%128(Visible) → s
    else if matches!(action.get_action(), SmallAction::RunAway) /*Play 0 RunAway·1 Recall·5 AroundRunAway*/ { s / 2 }   // L546 (18947)
    else { s };
  objective_score + bonus   // L557~558 (18953)
```

**`mem` 메모리 접근 27건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m14.ll:18638 | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | m14.ll:18649 | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | m14.ll:18652 | 4 | OK |
| 3 | OperationData | 0x8 | context | r | m14.ll:18835 | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | m14.ll:18653~18656, None→unwrap_failed(18678) | 4 | OK |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr | r | m14.ll:18665 | 4 | OK |
| 6 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | m14.ll:18667 | 4 | OK |
| 7 | vtable(AbstractGame) | 0x40 | get_game_mode | r | m14.ll:18668~18670, 18809 (2회: L512, L524) | 4 | 확인불가(tcx 사전에 타입 없음) |
| 8 | vtable(AbstractGame) | 0x1f0 | get_entity_by_id | r | m14.ll:18708~18712 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 9 | GameMode | 0x0 | tag | r | 0=Moba 만, 아니면 unwrap_failed (m14.ll:18674, 18813) | 4 | OK |
| 10 | GameMode | 0x8 | Moba.0 (&MobaMode) | r | m14.ll:18682 | 4 | OK |
| 11 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.buf.ptr | r | m14.ll:18700, 18867 (tcxdict MobaMode 0x1d0) | 3 | OK |
| 12 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | m14.ll:18690, 18857 — 0 → return 0 / 보너스 0 | 4 | OK |
| 13 | Entity | 0x660 | x | r | champ·serpen (m14.ll:18724, 18732) | 4 | OK |
| 14 | Entity | 0x668 | y | r | champ·serpen (m14.ll:18728, 18736) | 4 | OK |
| 15 | Entity | 0x0 | team@tag | r | champ player_team (m14.ll:18890) 0=Player/1=Neutral | 4 | OK |
| 16 | Entity | 0x8 | team@Player.0 | r | m14.ll:18897 | 4 | OK |
| 17 | Entity | 0x38 | visible_state[team]@tag | r | serpen, stride 24B (gepS m14.ll:18905) 0=Visible | 4 | OK |
| 18 | GameContext | 0x20 | map (&MapDef) | r | m14.ll:18837 | 4 | OK |
| 19 | MapDef | 0x38b8 | regions[30][30] | r | m14.ll:18839 [y/32000][x/32000] clamp 29 | 4 | OK |
| 20 | SmallActionPlay | 0xb1 | @tag (니치) | r | m14.ll:18758, 18916 | 4 | OK |
| 21 | SmallActionPlay | 0x8 | Around/AroundHide/LaneMinionPosition.target_id | r | m14.ll:18804 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |
| 22 | SmallActionPlay | 0x10 | AroundRegion x / AroundPositionBush y | r | m14.ll:18817~18822 phi 오프셋 쌍 idx4→(16,24) idx7→(48,56) idx8→(8,16) idx9→(24,32) | 4 | OK |
| 23 | SmallActionPlay | 0x18 | AroundRegion y / AroundBush x | r |  | 4 | OK |
| 24 | SmallActionPlay | 0x20 | AroundBush y | r |  | 4 | OK |
| 25 | SmallActionPlay | 0x30 | AroundPosition x | r |  | 4 | OK |
| 26 | SmallActionPlay | 0x38 | AroundPosition y | r |  | 4 | OK |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 22500000001 | 518 | 임계 | 150000²+1 — champ↔serpen distance_sq < 이것(=거리 ≤150000) 이면 bonus=0(match 생략). 150k 밖에서만 세르펜 지향 보너스 (m14.ll:18753) | 4 |
| 1 | 7 | 532 | 센티널 | 목표 지점 regions 값 == 7 → +5 (m14.ll:18844). ⚠7 = RegionName::Epic(tcxdict --enum RegionName; Serpen 은 2). 세르펜 포크인데 에픽 region 을 본다 — IR 정본(epic_poke.rs:542 와 동일 코드). ⚠18764·18922 의 7 은 니치 untagged idx | 3 |
| 2 | 5 | 524 | 태그 | 세르펜 지향 보너스(L524 target_id==serpen live_list[0] / L532 region==7) (m14.ll:18879, 18845) | 4 |
| 3 | 2 | 546 | 임계 | objective_score = s/2 (sdiv) — 세르펜 미가시 ∧ RunAway 계열 (m14.ll:18947). ⚠18640·18899 의 2 는 팀 경계검사 | 4 |
| 4 | 32000 | 531 | 인덱스 | 셀 크기 — 좌표→regions 인덱스 (m14.ll:18825, 18830) | 4 |
| 5 | 29 | 531 | 인덱스 | regions 인덱스 clamp 상한 (m14.ll:18829, 18834) | 4 |
| 6 | 0 | 513 | 태그 | GameMode 태그 0=Moba(18674) · live_list.len==0(18696) → return 0 · visible_state 태그 0=Visible(18907) · 반환 0(18957) | 4 |
| 7 | 3 | 522 | 센티널 | SmallActionPlay 니치 시작: idx = tag-3 (m14.ll:18762) — get_action 인라인 아티팩트 | 4 |
| 8 | 10 | 522 | 태그 | llvm.assume(tag != 10) (m14.ll:18760) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 세르펜 지향 보너스 | serpen_poke.rs:524 / :532 (m14.ll:18879, 18845) | 5 | 올리면 150k 밖에서 세르펜(또는 region 7 지점)을 향하는 Around/AroundPosition 행동이 더 자주 선택 | 4 | 기존 |
| 1 | 보너스 적용 거리 게이트 | serpen_poke.rs:518 (m14.ll:18753) | 22500000001 | 150000² +1. 내리면 더 가까이서도 보너스 적용(세르펜 근처에서 접근 행동 과대), 올리면 더 멀리서만 | 4 | 기존 |
| 2 | 세르펜 미가시 도주계열 감점 나눗수 | serpen_poke.rs:546 (m14.ll:18947) | 2 | 키우면 세르펜이 안 보일 때 RunAway·Recall·AroundRunAway 억제 강화(음수 s 면 완화) | 4 | 기존 |

<details><summary>`callees` 피호출자 16건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 3 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 4 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 5 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 14 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 15 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
</details>

⚠**미매칭 3개**: `first`, `llvm.assume`, `llvm.umin.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m01.ll:8280, m01.ll:8311, m12.ll:22962, m12.ll:37305, m14.ll:16323) · **형제 8개** (SerpenPokeSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan) -> game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:9 | True | fn() -> game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:15 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::serpen_poke | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:279 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::action_candidates_old | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:418 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:491 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:508 | False | fn(&game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L532 의 region==7(Epic) 이 의도인지 복붙 결함인지 — IR 정본은 7. RegionName 은 explicit discr(`{constant#0}`)라 regions 값↔RegionName 대응은 확실하나 소스 의도는 소스 없이 판정 불가 | 3 |  |
| 1 | 표기 불가 | get_game_mode Moba 외 패닉의 소스 표기(unwrap/expect/let-else) — 표기 불가 | 4 |  |
| 2 | 미탐색 | self(SerpenPokeSubPlan) 필드 미사용(captures(none)) | 4 |  |
| 3 | 미탐색 | L513 early-return 0 에서 이미 호출된 interaction_score 의 부작용(rnd·debug write)은 콜리 명세 몫 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

