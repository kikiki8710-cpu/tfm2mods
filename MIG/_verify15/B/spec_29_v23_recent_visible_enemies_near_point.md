---

### `29` v23_recent_visible_enemies_near_point — 점 (x,y) 반경 range 안의 적 챔프 중 HP비율>=min 이고 최근 가시인 수를 센다

| 항목 | 값 |
|---|---|
| id | `objective_helpers__v23_recent_visible_enemies_near_point` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers37v23_recent_visible_enemies_near_point` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:31` |
| IR | `m15.ll` 55548~56099행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::v23_recent_visible_enemies_near_point` · **pub** |
| 계층 | 기타 |
| exe | `15510720` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | 판단 주체. info.team(+0x930) 으로 적팀 인덱스를 만들고 is_recent_visible 에 그대로 넘긴다 | 4 |
| 1 | 2 | data | &OperationData(24B) | cache(+0x0)·blackboard(+0x10) 만 읽음(context 는 안 읽음) | 4 |
| 2 | 3 | x | u64 | 기준점 x (월드 좌표) | 4 |
| 3 | 4 | y | u64 | 기준점 y | 4 |
| 4 | 5 | range | u64 | 반경. 본문에서 range*range 로 제곱해 distance_sq 와 비교(경계 포함) | 4 |
| 5 | 6 | min_hp_ratio | usize | HP 백분율 하한. hp*100/max_hp < min 이면 제외 (v23_healthy 인라인, objective_helpers.rs:12) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v23_recent_visible_enemies_near_point(player, data, x, y, range, min_hp_ratio) -> usize

[L32] enemy_ix = 1 - player.info.team(+0x930)           // >=2 → panic_bounds_check(len=2)
[L33] range_sq = range * range                            // IR %20 (루프 밖 1회)
      count = data.cache(+0x0).iter_champions(enemy_ix)   // simulation.rs:1905 인라인 = player_champion(+0x1e0)[enemy_ix] 의 Some(&Entity) 만
        .filter(|e| {                                     // closure$0 (objective_helpers.rs:34~36), 5칸 완전 언롤
[L34]       v23_healthy(e, min_hp_ratio)                  // objective_helpers.rs:12 인라인:
            //   = e.hp(+0x670) * 100 / e.stat_cached.hp(+0x628) >= min_hp_ratio
            //   (IR: icmp ult ratio, min → 작으면 탈락 / stat_cached.hp==0 → panic_const_div_by_zero)
[L35]       && Entity::distance_sq(e, (x, y)) <= range_sq // abs_diff(e.x,x)^2 + abs_diff(e.y,y)^2 (icmp ugt 면 탈락)
[L36]       && data.blackboard(+0x10)[enemy_ix].is_recent_visible(cache.game(+0x0/+0x8), player, e)
            //   = game.is_visible(player.team, e.id) || last_visible[e.position]+120 >= tick (blackboard.rs:348~350)
        })
        .count()                                          // IR: zext i1 → add 누적(accum.rs:55 fold)
[L39] return count

※ 세 조건의 평가 순서(IR 분기): healthy → 거리 → is_recent_visible. 앞에서 탈락하면 뒤는 평가 안 함(단락).
※ 부작용 없음. 호출 비용: 슬롯당 최대 1회 is_recent_visible(vtable 호출 2~3회).
```

**`mem` 메모리 접근 10건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. enemy_ix = 1 - team. >=2 면 panic_bounds_check(len=2, iter_champions 인라인 simulation.rs:1905) | 4 | OK |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2](744B stride). [enemy_ix] 가 is_recent_visible 의 self | 4 | OK |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[enemy_ix] | r | [[Option<&Entity>;5];2] (IR gep %12,480 → [5 x ptr] × enemy_ix). null = None 은 건너뜀. 5칸이 완전히 언롤됨 | 4 | OK |
| 4 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data ptr) | r | is_recent_visible 인자(루프 밖 1회 로드) | 4 | OK |
| 5 | AbstractGameWithCache | 0x8 | game (dyn AbstractGame vtable ptr) | r |  | 4 | OK |
| 6 | Entity(적 챔프) | 0x628 | stat_cached.hp | r | usize 최대 HP. ==0 이면 panic_const_div_by_zero(가드 없음 — v21_should_defer_support_target 의 max(.,1) 과 다름) | 4 | OK |
| 7 | Entity(적 챔프) | 0x670 | hp | r | usize 현재 HP. hp*100/stat_cached.hp | 4 | OK |
| 8 | Entity(적 챔프) | 0x660 | x | r | distance_sq(e, (x,y)) | 4 | OK |
| 9 | Entity(적 챔프) | 0x668 | y | r |  | 4 | OK |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 32 | 인덱스 | enemy_ix = 1 - player.info.team — 적팀 인덱스 | 4 |
| 1 | 2 | 32 | 임계 | panic_bounds_check 배열 길이(player_champion / blackboard [;2]) — 판정값 아님 | 4 |
| 2 | 100 | 12 | 계수 | v23_healthy: hp_ratio = hp*100 / stat_cached.hp — 백분율 변환(objective_helpers.rs:12, 이 함수에 인라인) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | HP 백분율 하한 (min_hp_ratio) | 인자 6 — 호출자가 정함. 비교식은 objective_helpers.rs:12 (IR m15.ll `icmp ult i64 %32, %5`) | 인자 | 올리면 저체력 적은 '위협'으로 안 세어 오브젝트 압박/셋업이 공격적으로 되고, 0 이면 HP 무관하게 전부 센다 | 4 | 기존 |
| 1 | 반경 (range) | 인자 5 — 호출자가 정함. 비교식은 objective_helpers.rs:35 (IR `icmp ugt i64 %50, %20`) | 인자 | 제곱 비교·경계 포함. 올리면 더 먼 적까지 센다 | 4 | 기존 |
| 2 | 최근 가시 시간창 (is_recent_visible 내부, 담당 범위 밖) | game-core blackboard.rs:350 (_gcbc g07.ll:157005~ `add i64 %24, 120`) | 120 | 120틱(2초@60tps) 안에 본 적까지 센다. 별도 define 이라 constants 에 없음 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 12 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 13 | v23_healthy | game_ai::plan_legacy::team_plan::objective_helpers::v23_healthy | in:game_ai | fn(&game_core::Entity, usize) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:11 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 14 | v23_recent_visible_enemies_near_point | game_ai::plan_legacy::team_plan::v23_recent_visible_enemies_near_point | pub | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:31 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 2개**: `cache`, `player_champion`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 8곳** (m09.ll:21339, m09.ll:22144, m15.ll:52772, m15.ll:54076, m15.ll:54732, m15.ll:56713, m15.ll:56727, m15.ll:56764) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 이 함수 본문에는 판정 리터럴이 100(백분율) 하나뿐이고 임계는 전부 인자(range, min_hp_ratio)다 — 실제 노브 값은 호출자(objective_helpers 의 다른 v23_* 함수들로 추정)에 있으며 미탐색 | 5 |  |
| 1 | 표기 불가 | v23_healthy(objective_helpers.rs:12) 가 `>=` 인지 `!(<)` 인지 소스 표기는 표기 불가 — 동작(ratio < min 이면 탈락)은 IR icmp ult 로 확정 | 4 |  |
| 2 | 미탐색 | iter_champions(simulation.rs:1905) 가 player_champion 의 Some 만 돌려주는 것은 IR(null 검사 → skip)로 확정. 죽은 챔프(hp 0)가 Some 으로 남는지 None 이 되는지는 game_core 소관이라 미확인 — hp 0 이면 healthy 가 0 < min 으로 걸러지므로(min>0 일 때) 결과엔 영향 없음 | 4 |  |
| 3 | 표기 불가 | distance_sq 의 두 번째 인자가 (x,y) 튜플인지 별도 두 인자인지는 표기 불가(utils.rs:7~9 abs_diff 두 번 + 제곱합으로만 보임) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

