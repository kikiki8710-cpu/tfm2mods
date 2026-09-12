---

### `27` nexus_under_direct_attack — 아군 넥서스가 직접 위협받는가: 최근가시 적 챔프가 240000 안 OR 적 미니언이 넥서스를 타겟 중

| 항목 | 값 |
|---|---|
| id | `defense_nexus__nexus_under_direct_attack` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus25nexus_under_direct_attack` |
| 소스 | `game-ai\src\plan_legacy\old\defense_nexus.rs:121` |
| IR | `m04.ll` 61399~61658행 |
| 경로·가시성 | `game_ai::plan_legacy::old::nexus_under_direct_attack` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `13892224` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData) -> bool
```

<details><summary>인자 2개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | 판단 주체. info.team(+0x930) 을 읽고 is_recent_visible 에 그대로 넘긴다 | 4 |
| 1 | 2 | data | &OperationData(24B) | cache(+0x0)·context(+0x8, pool 만)·blackboard(+0x10) 전부 읽는다 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn nexus_under_direct_attack(player, data) -> bool

[L122] team = player.info.team(+0x930)                       // >=2 → panic_bounds_check(len=2)
       nexus = data.cache(+0x0).nexus(+0x170)[team]           // Option<&Entity>, null → return false
       if nexus.is_none() { return false }

[L126] enemy_ix = 1 - team
       enemies: Vec<&Entity,&Bump> = cache.champions(enemy_ix, data.context(+0x8).pool(+0x0))
       //  (_gcbc g15.ll:109546, simulation.rs:1898~) = player_champion(+0x1e0)[enemy_ix] 의 Some 만 모은 벡터

[L127] hit = enemies.iter().any(|e| {                        // closure$0 (defense_nexus.rs:127~128) 인라인
           data.blackboard(+0x10)[enemy_ix].is_recent_visible(cache.game(+0x0/+0x8), player, e)
           //  = game.is_visible(player.team, e.id) || last_visible[e의 position] + 120 >= tick (blackboard.rs:348~350)
[L128]     && Entity::distance_sq(e, nexus) < 57600000001    // abs_diff(x)^2 + abs_diff(y)^2 <= 240000^2
       })
       drop(enemies)
[L129] if hit { return true }                                 // '적 챔프가 본진 코앞'

[L133] minions: Vec<&Entity,&Bump> = cache.minions(enemy_ix, pool)
       //  (_gcbc g15.ll:109887) = top_minions(+0x10)/mid_minions(+0x50)/bottom_minions(+0x90)[enemy_ix] 를 이어붙인 벡터
[L134] hit2 = minions.iter().any(|m| {                       // closure$1 (defense_nexus.rs:134) 인라인
           m.ty(+0x68) == EntityType::Minion /*tag 1*/
           && m.ty.Minion.info.nearest_enemy(+0x88 tag ==1 Some, +0x90 값) == Some(nexus.id(+0x5c0))
           //  Option<usize>::eq 인라인 (option.rs:2439~2440, cmp.rs:1878). None 이면 false
       })
       drop(minions)
[L135] return hit2                                            // '적 미니언이 넥서스를 직접 노리는 중'

※ 부작용 없음. 두 벡터는 bump pool 에 할당됐다가 각 단계 끝에 drop.
※ 분기 순서: 넥서스 없음 → 챔프 검사(참이면 미니언 검사 생략) → 미니언 검사.
```

**`mem` 메모리 접근 18건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. nexus[team] 인덱스, enemy_ix = 1 - team. >=2 면 panic_bounds_check(len=2) | 4 | OK |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(8840B) | 4 | OK |
| 2 | OperationData | 0x8 | context | r | &GameContext → pool(+0x0) 만 읽어 champions/minions 의 할당자로 넘김 | 4 | OK |
| 3 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. [enemy_ix](744B stride) 를 is_recent_visible 의 self 로 | 4 | OK |
| 4 | GameContext | 0x0 | pool | r | &bumpalo::Bump | 4 | OK |
| 5 | AbstractGameWithCache | 0x170 | nexus[team] | r | [Option<&Entity>;2] 니치(null=None). IR: gep %9,368 + team*8. None 이면 즉시 false | 4 | OK |
| 6 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data ptr) | r | is_recent_visible 인자 | 4 | OK |
| 7 | AbstractGameWithCache | 0x8 | game (dyn AbstractGame vtable ptr) | r | is_recent_visible 인자 | 4 | OK |
| 8 | Entity(nexus) | 0x660 | x | r | u64. distance_sq 의 other 쪽 | 4 | OK |
| 9 | Entity(nexus) | 0x668 | y | r | u64 | 4 | OK |
| 10 | Entity(nexus) | 0x5c0 | id | r | usize. 적 미니언의 nearest_enemy 와 비교(IR %78, 루프 밖에서 1회 로드) | 4 | OK |
| 11 | Entity(적 챔프, champions 원소) | 0x660 | x | r | distance_sq(e, nexus) | 4 | OK |
| 12 | Entity(적 챔프, champions 원소) | 0x668 | y | r |  | 4 | OK |
| 13 | Entity(적 미니언, minions 원소) | 0x68 | ty (EntityType 태그) | r | i64. == 1 (Minion) 일 때만 아래 두 필드를 읽는다 (tcxdict --enum EntityType 1: idx1/discr1/tag1 = Minion) | 3 | OK |
| 14 | Entity(적 미니언) | 0x88 | ty@Minion.info.nearest_enemy@tag | r | Option<usize> 판별자 8B. trunc nuw i64→i1: 1 = Some | 4 | OK |
| 15 | Entity(적 미니언) | 0x90 | ty@Minion.info.nearest_enemy@Some.0 | r | usize. == nexus.id 면 '넥서스를 직접 노리는 미니언' | 4 | OK |
| 16 | bumpalo Vec<&Entity>(32B) | 0x0 | ptr | r | champions/minions 결과 버퍼 시작 (sret alloca %4/%3) | 4 | 확인불가(tcx 사전에 타입 없음) |
| 17 | bumpalo Vec<&Entity>(32B) | 0x18 | len | r | 0 이면 any() 가 false. 루프 끝 = ptr + len*8 | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 4건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 122 | 임계 | panic_bounds_check 배열 길이(nexus [Option<&Entity>;2]) — team 인덱스 가드 | 4 |
| 1 | 1 | 126 | 태그 | enemy_ix = 1 - player.info.team (sub nuw nsw). champions/minions/blackboard 의 적팀 인덱스 | 4 |
| 2 | 57600000001 | 128 | 임계 | 240000^2 + 1 — 적 챔프↔넥서스 제곱거리 임계. `icmp ult d2, 240000^2+1` = d2 <= 240000^2 (= 7.5칸, 셀 32000 기준) | 4 |
| 3 | 1 | 134 | 태그 | EntityType 메모리태그 1 = Minion. 적 미니언 목록 원소가 Minion variant 인지(다른 EntityType 은 nearest_enemy 오프셋이 달라 건너뜀) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 적 챔프 '본진 코앞' 반경 | defense_nexus.rs:128 (IR m04.ll `icmp ult i64 %58, 57600000001`) | 57600000001 | 240000(7.5칸) 이내의 최근가시 적 챔프가 있으면 넥서스 직접 위협. 올리면 더 먼 적에도 넥서스 방어 모드(전원 방어, _docs:314)가 켜져 과민해지고, 내리면 실제로 넥서스에 붙어야 반응한다. value 는 제곱+1 이므로 반경 r 로 바꾸려면 r^2+1 | 4 | 기존 |
| 1 | 최근 가시 시간창 (is_recent_visible 내부, 담당 범위 밖) | game-core blackboard.rs:350 (_gcbc g07.ll:157005~157049 `add i64 %24, 120`) | 120 | 120틱(2초@60tps) 안에 본 적 챔프까지 위협으로 센다. 별도 define 이라 constants 에 없음 | 4 | 기존 |

<details><summary>`callees` 피호출자 16건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | minions | game_core::AbstractGameWithCache::<'a, 'b>::minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1853 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 9 | nexus_under_direct_attack | game_ai::plan_legacy::old::nexus_under_direct_attack | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:121 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 10 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 12 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 13 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 14 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 15 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
</details>

⚠**미매칭 7개**: `bottom_minions`, `cache`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `mid_minions`, `player_champion`, `pool`, `top_minions`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m04.ll:28007, m04.ll:28017, m04.ll:28053, m04.ll:57124, m04.ll:59568) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | champions()/minions() 내부(_gcbc g15.ll:109546/109887)는 읽는 캐시 필드(+0x1e0 / +0x10·+0x50·+0x90)만 확인했고, 죽은 엔티티·Illusion 등을 거르는지(필터 조건)는 미확인 | 4 |  |
| 1 | 미탐색 | Minion.info.nearest_enemy 의 갱신 규칙(누가 언제 Some(nexus.id) 로 세팅하는지 — 실제 공격 중인지 단순 최근접인지)은 game_core 쪽이라 안 읽었다. _docs:293 의 '직접 때리는 중' 은 개발자 주석 기준 | 4 |  |
| 2 | 표기 불가 | L127 의 && 두 항 순서(is_recent_visible 먼저, distance 나중)는 IR 분기 순서로 확정. 소스 표기가 한 줄 && 인지 두 줄인지는 표기 불가 | 4 |  |
| 3 | 미탐색 | 적 챔프 검사에서 is_ignored_well_enemy 류의 샘 위험구역 제외는 없다(본문에 호출·인라인 흔적 없음) — 즉 적 샘에 서 있어도 240000 안이면 위협으로 센다(넥서스와 적 샘은 멀어 실전 영향은 없을 것으로 추정) | 5 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

