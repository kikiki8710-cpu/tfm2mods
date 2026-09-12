---

### `25` v22_visible_enemy_is_runaway_threat — 최근 시야에 잡힌 적 챔프가 (적 사거리+40000, 상한 150000) 안이면 도주 위협으로 판정

| 항목 | 값 |
|---|---|
| id | `fight_model__v22_visible_enemy_is_runaway_threat` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model35v22_visible_enemy_is_runaway_threat` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:1168` |
| IR | `m10.ll` 49402~49495행 |
| 경로·가시성 | `game_ai::plan_legacy::old::fight_model::v22_visible_enemy_is_runaway_threat` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | `14730000` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | AI 버전. 본문에서 분기 없음 — is_enemy_well_danger(인라인된 is_ignored_well_enemy 내부)로만 전달 | 4 |
| 1 | 2 | player | &PlayerState(2528B) | 판단 주체(아군) 플레이어. info.team(+0x930)만 읽는다 | 4 |
| 2 | 3 | data | &OperationData(24B) | {0x0 cache: &AbstractGameWithCache, 0x8 context, 0x10 blackboard: &[Blackboard;2]}. cache(+0x0)과 blackboard(+0x10)를 읽는다 | 4 |
| 3 | 4 | champ | &Entity(1728B) | 아군 챔피언(도주 주체). x/y(+0x660/+0x668)만 읽고 max_range_cached 의 target 으로 넘긴다 | 4 |
| 4 | 5 | enemy | &Entity(1728B) | 판정 대상 적 챔피언. team(+0x0/+0x8)·x/y 를 읽고 is_recent_visible/max_range_cached 의 인자로 넘긴다 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v22_visible_enemy_is_runaway_threat(version, player, data, champ, enemy) -> bool

[L1169] enemy_ix = 1 - player.info.team(+0x930)            // >=2 면 panic_bounds_check(len=2)
        bb = &data.blackboard(+0x10)[enemy_ix]               // 744B stride. Blackboard[T] = T 팀 자체 정보(_docs game_core.txt:21)
        if !bb.is_recent_visible(data.cache.game(&dyn AbstractGame, cache+0x0/+0x8), player, enemy) { return false }
        //  is_recent_visible(_gcbc g07.ll:157005, blackboard.rs:348~354) =
        //    game.is_visible(player.team, enemy.id)  ||
        //    { ep = game.get_player_by_champion_id(enemy.id); ep != null && bb.last_visible[ep.position] + 120 >= game.tick() }
        //    (120틱 = 2초@60tps 안에 본 적이 있으면 '최근 가시')

[L1170] // is_ignored_well_enemy(version, player, enemy) 가 인라인됨 (fight_model.rs:755~756)
        ignored = (enemy.team(+0x0) == TeamType::Player /*tag 0*/ && enemy.team.0(+0x8) == enemy_ix)
                  && path_finder::is_enemy_well_danger(version, player, enemy.x(+0x660), enemy.y(+0x668))
        if ignored { return false }                          // 적 샘(well) 위험구역에 선 적은 위협으로 안 본다
        //  ⚠ enemy.team 이 Player(enemy_ix) 가 아니면(중립 등) is_enemy_well_danger 를 호출하지 않고 ignored=false

[L1174] threat_range = min( battle::max_range_cached(data, champ=enemy, target=champ) + 40000, 150000 )
        //  ⚠인자 순서: max_range_cached(data, champ, target) 에 (enemy, champ) 가 들어간다 = '적이 아군 champ 를 때릴 수 있는 최대 사거리'

[L1175] d2 = Entity::distance_sq(enemy, champ)               // utils.rs:7~9: abs_diff(x)^2 + abs_diff(y)^2 (enemy 좌표 +0x660/+0x668 vs champ 좌표)
        return d2 <= threat_range * threat_range             // icmp ule (경계 포함)

[L1176] (phi: %10 경로 false / %31 경로 false / %33 경로 = 비교결과)

※ 부작용 없음(writes 없음). version 은 분기에 안 쓰이고 is_enemy_well_danger 로만 전달.
```

**`mem` 메모리 접근 11건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. enemy_ix = 1 - team (IR: sub i64 1, %7; 2 이상이면 panic_bounds_check len=2) | 4 | OK |
| 1 | OperationData | 0x10 | blackboard | r | &[Blackboard;2](원소 744B). [enemy_ix] 를 골라 is_recent_visible 의 self 로 넘긴다 | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache. cache+0x0/+0x8 = &dyn AbstractGame (data, vtable) 을 is_recent_visible 에 전달 | 4 | OK |
| 3 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data ptr) | r | load ptr, ptr %14 | 4 | OK |
| 4 | AbstractGameWithCache | 0x8 | game (dyn AbstractGame vtable ptr) | r | gep %14, 8 | 4 | OK |
| 5 | Entity(enemy) | 0x0 | team@tag (TeamType 판별자) | r | 0 = Player. is_ignored_well_enemy 인라인(fight_model.rs:755, entity.rs:1127 eq)에서 TeamType::Player(enemy_ix) 와 비교 | 4 | OK |
| 6 | Entity(enemy) | 0x8 | team@Player.0 | r | usize 팀 번호. == enemy_ix (1 - player.team) 인지 비교 | 4 | OK |
| 7 | Entity(enemy) | 0x660 | x | r | u64. is_enemy_well_danger 인자 및 distance_sq | 4 | OK |
| 8 | Entity(enemy) | 0x668 | y | r | u64 | 4 | OK |
| 9 | Entity(champ) | 0x660 | x | r | u64. distance_sq(enemy, champ) 의 other 쪽 | 4 | OK |
| 10 | Entity(champ) | 0x668 | y | r | u64 | 4 | OK |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 1169 | 인덱스 | enemy_ix = 1 - player.info.team — 적팀 인덱스(blackboard 인덱스이자 TeamType::Player(n) 비교값) | 4 |
| 1 | 2 | 1169 | 임계 | panic_bounds_check 의 배열 길이(blackboard [Blackboard;2]). 판정값 아님, 인덱스 가드 | 4 |
| 2 | 0 | 1170 | 태그 | TeamType 메모리태그 0 = Player (tcxdict --enum TeamType: idx0/discr0/tag0). enemy.team 이 Player(enemy_ix) 인지 검사(is_ignored_well_enemy 인라인) | 3 |
| 3 | 40000 | 1174 | 오프셋가감 | 위협 사거리 여유분: threat_range = max_range_cached(enemy→champ) + 40000 (셀 32000 보다 약간 큼 ≈ 1.25칸) | 4 |
| 4 | 150000 | 1174 | 인덱스 | threat_range 상한(core::cmp::min 이 llvm.umin 으로 인라인). 사거리+40000 이 150000 을 넘어도 150000 으로 자른다(≈4.7칸) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 위협 사거리 여유분 | fight_model.rs:1174 (IR m10.ll `add i64 %34, 40000`) | 40000 | 적 최대사거리에 더하는 여유. 올리면 더 먼 적도 도주 위협으로 보아 도주 판단이 과민해지고, 내리면 사거리 바로 밖의 적은 무시한다 | 4 | 기존 |
| 1 | 위협 사거리 상한 | fight_model.rs:1174 (IR m10.ll `llvm.umin.i64(%35, 150000)`) | 150000 | 장사거리 적이라도 150000(≈4.7칸)까지만 위협으로 본다. 올리면 장거리 챔프(포킹형)에 대한 도주가 더 멀리서 발동한다 | 4 | 기존 |
| 2 | 최근 가시 시간창 (is_recent_visible 내부, 담당 범위 밖) | game-core blackboard.rs:350 (_gcbc g07.ll:157005~157049 `add i64 %24, 120`) | 120 | 마지막 목격 후 120틱(2초@60tps) 안이면 '최근 가시'. 올리면 시야에서 사라진 적을 더 오래 위협으로 유지한다. constants 에 안 넣은 이유 = 담당 함수 본문이 아니라 game_core 의 별도 define | 4 | 기존 |

<details><summary>`callees` 피호출자 19건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | get_player_by_champion_id | game_core::AbstractGame::get_player_by_champion_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation.rs:145 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | get_player_by_champion_id | <game_core::Game as game_core::AbstractGame>::get_player_by_champion_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation\game.rs:1874 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | get_player_by_champion_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_player_by_champion_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation\game.rs:3922 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 13 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 14 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 15 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 16 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 17 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 18 | v22_visible_enemy_is_runaway_threat | game_ai::plan_legacy::old::fight_model::v22_visible_enemy_is_runaway_threat | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1168 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `llvm.umin.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 18곳** (m05.ll:9679, m05.ll:9834, m05.ll:26341, m05.ll:26374, m05.ll:26407, m05.ll:26440, m05.ll:26473, m05.ll:34147, m05.ll:34180, m05.ll:34213, m05.ll:34246, m05.ll:34279, m10.ll:5411, m10.ll:27219, m10.ll:27252, m10.ll:27285, m10.ll:27318, m10.ll:27351) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | max_range_cached(data, champ, target) 의 내부(m10.ll:50689~, TLS RefCell 캐시)는 안 읽었다 — '적 enemy 가 target=champ 에 대해 갖는 최대 사거리'라는 것은 인자명(data/champ/target, battle.rs:2332 DILocalVariable)으로만 확정 | 4 |  |
| 1 | 미탐색 | path_finder::is_enemy_well_danger(version, player, x, y) 내부(m03.ll:144500)는 안 읽었다 — 좌표가 적 샘(well) 위험구역인지 판정한다는 것은 이름·인자에서만 추정 | 4 |  |
| 2 | 미탐색 | is_recent_visible 의 vtable 슬롯 +0xf8 = is_visible / +0x150 = get_player_by_champion_id / +0x28 = tick 은 divtable(AbstractGame) 로 확인했으나 ExpectedGame 구현 기준이며 실제 런타임 게임 타입의 슬롯 배치가 같은지는 미확인(정적 vtable 규약상 같아야 함) | 3 |  |
| 3 | 표기 불가 | L1169 의 `A && B` 표기(is_recent_visible 를 부정 조건으로 조기 반환하는지, 하나의 && 식인지)는 column 정보 부재로 표기 불가 — 동작(가시 아니면 false)은 확정 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

