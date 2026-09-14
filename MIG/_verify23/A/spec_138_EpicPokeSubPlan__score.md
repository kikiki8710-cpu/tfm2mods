---

### `138` EpicPokeSubPlan::score — 에픽 포킹 서브플랜 행동 점수 = interaction_score ± (에픽 미가시·도주계열이면 /2) + 에픽 지향 보너스 5

| 항목 | 값 |
|---|---|
| id | `EpicPokeSubPlan__score` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9epic_pokeNtB2_15EpicPokeSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\epic_poke.rs:521` |
| IR | `m02.ll` 46765~47098행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `ccaa10` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &EpicPokeSubPlan | captures(none) — 본문에서 안 읽음 | 4 |
| 1 | 2 | version | usize | interaction_score 로 전달만. 이 함수 안 분기 없음 | 4 |
| 2 | 3 | parameter | &ScoreParameter(5384B) | interaction_score 로 전달만 | 4 |
| 3 | 4 | rnd | &mut StdRng(320B) | interaction_score 로 전달만 — 이 함수 자체의 write 0건(호출 안에서의 소비는 콜리 몫) | 4 |
| 4 | 5 | player | &PlayerState(2528B) | info.team(+0x930)·info.position 태그(+0x9c0) 로 내 챔피언 조회 | 4 |
| 5 | 6 | data | &OperationData(24B) | cache(+0)→player_champion / game.get_game_mode·get_entity_by_id ; context(+8)→map(+0x20)→regions | 4 |
| 6 | 7 | action | &SmallActionPlay(184B) | get_action()(small_action.rs:309 인라인, 태그 +0xb1 니치) 로 SmallAction 변환해 매칭 | 4 |
| 7 | 8 | debug | &mut DebugFrameData(224B) | interaction_score 로 전달만 — 이 함수 자체의 write 0건 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // epic_poke.rs:521
  let s = interaction_score(version, rnd, player, data, parameter, action, debug);   // L522 (m02.ll:46781)
  let champ = data.cache.player_champion[player.info.team][player.info.position].unwrap();   // L524
  // L525: let epic = match data.cache.game.get_game_mode() { Moba(m) => m.jungle_runner.epic.live_list.first().and_then(|id| game.get_entity_by_id(*id)), _ => unwrap_failed(패닉) }
  //   live_list.len==0 → None (46839) ; 아니면 get_entity_by_id(live_list[0]) (46855, Option<&Entity> — null=None)
  let champ_region = map.regions[min(champ.y/32000,29)][min(champ.x/32000,29)];   // L526 (46866~46885)
  let bonus = if champ_region == 7 /*Epic*/ { 0 }   // L528 — 이미 에픽 지역이면 보너스 없음
    else match action.get_action() {                 // L532 (small_action.rs:309 인라인, 니치 태그→idx)
      SmallAction::Around{target_id} /*Play idx 2 Around·3 AroundHide·10 LaneMinionPosition*/ => {   // L534
         // get_game_mode() 재호출(46950) → Moba 아니면 패닉 ; live_list.first() == Some(&target_id) ? 5 : 0 (47013~47015; len==0 이면 0)
      }
      SmallAction::AroundPosition{x,y} /*Play idx 4 AroundRegion(x@16,y@24)·7 AroundPosition(48,56)·8 AroundPositionBush(8,16)·9 AroundBush(24,32)*/ => {   // L541
         let region = map.regions[min(y/32000,29)][min(x/32000,29)]; if region == 7 {5} else {0}   // L541~542 (46966~46981)
      }
      _ /*RunAway·Recall·AroundRunAway·Positioning·Trace·Attack·Skill·Skill2·Ult·Stop*/ => 0
    };
  // L556: let visible = epic.is_some_and(|e| e.is_visible_from(champ))
  //   is_visible_from(entity.rs:1482) = match champ.player_team() { None(team 태그 Neutral=1, 47033) => true, Some(t) => e.visible_state[t] is Visible(태그 0, data.rs:122, 47050) }
  let objective_score = if visible { s }                                       // 47089 phi: %115(Neutral)·%123(Visible) → s
    else if matches!(action.get_action(), SmallAction::RunAway) /*Play idx 0 RunAway·1 Recall·5 AroundRunAway*/ { s / 2 }   // L557 (47086 sdiv)
    else { s };
  objective_score + bonus   // L568~569 (47092)
```

**`mem` 메모리 접근 27건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m02.ll:46783 (<2 경계검사 46785) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | m02.ll:46794 i32 zext → player_champion 2차 인덱스 | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | m02.ll:46797 | 4 | OK |
| 3 | OperationData | 0x8 | context | r | m02.ll:46878 | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | m02.ll:46798~46801 Option<&Entity>, None 이면 unwrap_failed(46821) | 4 | OK |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr | r | m02.ll:46808 | 4 | OK |
| 6 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | m02.ll:46810 | 4 | OK |
| 7 | vtable(AbstractGame) | 0x40 | get_game_mode | r | m02.ll:46811~46813, 46950 → GameMode{tag i64, ptr} (divtable AbstractGame 0x40). 2회 호출(L525, L534) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 8 | vtable(AbstractGame) | 0x1f0 | get_entity_by_id | r | m02.ll:46851~46855 fn(id)->Option<&Entity> (divtable AbstractGame 0x1f0) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 9 | GameMode | 0x0 | tag | r | 0=Moba 만 허용, 그 외 unwrap_failed (m02.ll:46817, 46954) | 4 | OK |
| 10 | GameMode | 0x8 | Moba.0 (&MobaMode) | r | extractvalue 1 (m02.ll:46825) | 4 | OK |
| 11 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.buf.ptr | r | m02.ll:46843, 47003 (tcxdict MobaMode 0x1a0) | 3 | OK |
| 12 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | m02.ll:46833, 46993 — 0 이면 epic=None / target 비교 0 | 4 | OK |
| 13 | Entity | 0x660 | x | r | champ (m02.ll:46871) | 4 | OK |
| 14 | Entity | 0x668 | y | r | champ (m02.ll:46864) | 4 | OK |
| 15 | Entity | 0x0 | team@tag | r | champ, is_visible_from→player_team (m02.ll:47024) 0=Player 1=Neutral (tcxdict TeamType) | 3 | OK |
| 16 | Entity | 0x8 | team@Player.0 | r | champ 팀 번호 (m02.ll:47025) | 4 | OK |
| 17 | Entity | 0x38 | visible_state[team]@tag | r | epic, stride 24B (gepS m02.ll:47048) 0=Visible/1=Invisible/2=Unknown (tcxdict VisibleState) | 3 | OK |
| 18 | GameContext | 0x20 | map (&MapDef) | r | m02.ll:46880 | 4 | OK |
| 19 | MapDef | 0x38b8 | regions[30][30] (usize) | r | m02.ll:46882 [y/32000][x/32000], 각 축 min(…,29). 값은 RegionName 인덱스(7=Epic) | 4 | OK |
| 20 | SmallActionPlay | 0xb1 | @tag (니치) | r | m02.ll:46892, 47055 — tag>2 ? tag-3 : 7 = 논리 idx (untagged AroundPosition=idx7) | 4 | OK |
| 21 | SmallActionPlay | 0x8 | Around/AroundHide/LaneMinionPosition.target_id (get_action→SmallAction::Around{target_id}) | r | m02.ll:46945 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |
| 22 | SmallActionPlay | 0x10 | AroundRegion x (idx4) / AroundPositionBush y (idx8) | r | m02.ll:46958~46963 phi 오프셋 쌍: idx4→(16,24) idx7→(48,56) idx8→(8,16) idx9→(24,32) | 4 | OK |
| 23 | SmallActionPlay | 0x18 | AroundRegion y / AroundBush x | r | get_action→SmallAction::AroundPosition{x,y} | 4 | OK |
| 24 | SmallActionPlay | 0x20 | AroundBush y | r |  | 4 | OK |
| 25 | SmallActionPlay | 0x30 | AroundPosition x | r |  | 4 | OK |
| 26 | SmallActionPlay | 0x38 | AroundPosition y | r |  | 4 | OK |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 7 | 528 | 센티널 | RegionName::Epic 의 usize 인덱스(tcxdict --enum game_core::RegionName idx7). champ_region==7 → bonus 0 (m02.ll:46887) / 목표 region==7 → +5 (m02.ll:46980). ⚠m02.ll:46898·47061 의 7 은 니치 untagged idx(AroundPosition) — 다른 뜻 | 3 |
| 1 | 5 | 534 | 태그 | 에픽 지향 보너스(L534 target_id==epic id / L542 목표 지점 region==Epic) (m02.ll:46981, 47015) | 4 |
| 2 | 2 | 557 | 임계 | objective_score = s / 2 (sdiv, L557) — 에픽 미가시 ∧ RunAway 계열 (m02.ll:47086). ⚠46785·47038 의 2 는 팀 배열 경계검사 | 4 |
| 3 | 32000 | 526 | 인덱스 | 셀 크기 — 좌표→regions 인덱스 변환 (m02.ll:46866, 46873, 46966, 46971) | 4 |
| 4 | 29 | 526 | 인덱스 | regions 인덱스 상한 clamp(min(…,29), 30x30 격자) (m02.ll:46870 등) | 4 |
| 5 | 0 | 525 | 태그 | GameMode 태그 0=Moba(m02.ll:46817) · live_list.len==0 (46839) · visible_state 태그 0=Visible(47050) · bonus 기본 0 | 4 |
| 6 | 3 | 532 | 센티널 | SmallActionPlay 니치 시작(niche_start=3): idx = tag-3 (m02.ll:46896) — get_action 인라인 아티팩트 | 4 |
| 7 | 10 | 532 | 태그 | llvm.assume(tag != 10): 태그 10 은 SmallActionPlay 에 없는 값(idx7 은 untagged) (m02.ll:46894) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 에픽 지향 보너스 | epic_poke.rs:534 / :542 (m02.ll:47015, 46981) | 5 | 올리면 에픽을 목표로 하는 Around·AroundPosition(에픽 region) 행동이 다른 행동보다 더 자주 선택됨. 챔피언이 이미 Epic region 안이면 무효 | 4 | 기존 |
| 1 | 에픽 미가시 시 도주계열 감점 비율 | epic_poke.rs:557 (m02.ll:47086) | 2 | s/2 → 나눗수를 키우면 에픽이 안 보일 때 RunAway·Recall·AroundRunAway 를 더 강하게 억제(음수 s 면 반대로 완화됨 — sdiv 0방향 절사) | 4 | 기존 |
| 2 | 에픽 region 코드 | epic_poke.rs:528 / :542 | 7 | RegionName::Epic 인덱스 — 바꾸면 다른 지역을 '에픽 지역'으로 취급(맵 정의 종속, 노브로 부적합) | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 1 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 2 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 3 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 15 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 16 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
</details>

⚠**미매칭 3개**: `first`, `llvm.assume`, `llvm.umin.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m01.ll:20231, m01.ll:20262, m02.ll:43896, m12.ll:23361, m12.ll:37288) · **형제 8개** (EpicPokeSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::EpicPokeSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::EpicPokeSubPlan) -> game_ai::plan_legacy::sub_plan::EpicPokeSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::EpicPokeSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::sub_plan::EpicPokeSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:9 | True | fn() -> game_ai::plan_legacy::sub_plan::EpicPokeSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:14 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::epic_poke | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:278 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::action_candidates_old | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:426 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:504 | True | fn(&game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:521 | False | fn(&game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | get_game_mode 가 Moba 가 아닐 때 unwrap_failed(m02.ll:47096·47019)로 가는 소스 표기(`unwrap()` vs `expect`/`let else` — 표기 불가, 패닉 동작은 확정) | 4 |  |
| 1 | 미탐색 | L556 `is_some_and` 클로저(closure$1)와 L525 `and_then` 클로저(closure$0)는 전부 인라인(fnparts 별도 define 없음) — 본문 관측으로 복원 | 4 |  |
| 2 | 미탐색 | bonus 계산(L532 match)에서 SmallAction::Positioning{x,y}(Play idx6) 이 AroundPosition 과 달리 0 으로 가는 것은 switch 관측(m02.ll:46906) — get_action 의 Positioning 매핑이 SmallAction::Positioning 이라 match 팔에 없기 때문으로 추정(get_action MIR 미열람) | 3 |  |
| 3 | 미탐색 | self(EpicPokeSubPlan) 필드는 본문에서 전혀 안 읽힘(captures(none)) — 의도인지 여부는 소스 없이는 불명 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

