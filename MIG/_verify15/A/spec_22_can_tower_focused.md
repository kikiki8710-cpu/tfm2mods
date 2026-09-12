---

### `22` can_tower_focused — (x,y) 에 서면 적 타워 중 하나가 내 챔피언을 조준할 수 있는가 — 이미 나를 조준 중 / 타깃 있어도 사거리 안 내 미니언<2 & 내가 사거리 안 / 타깃 없고 내가 사거리 안

| 항목 | 값 |
|---|---|
| id | `tower_discipline__can_tower_focused` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline17can_tower_focused` |
| 소스 | `game-ai\src\tower_discipline.rs:9` |
| IR | `m07.ll` 48968~49426행 |
| 경로·가시성 | `game_ai::can_tower_focused` · **pub** |
| 계층 | 기타 |
| exe | `14250752` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, u64, u64) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | context | &GameContext(64B) | setting(+0x8).tower_attack_disable_tick 와 pool(+0x0, towers Vec 할당자) | 4 |
| 1 | 1 | cache | &AbstractGameWithCache | game(dyn, +0x0/+0x8) · player_champion(+0x1e0) · towers() · iter_minions() | 4 |
| 2 | 2 | player | &PlayerState(2528B) | info.team(+0x930) · info.position(+0x9c0) | 4 |
| 3 | 3 | x | u64 | 판정 좌표 x (내 챔피언 현 위치가 아니라 인자 좌표) | 4 |
| 4 | 4 | y | u64 | 판정 좌표 y | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn can_tower_focused(context, cache, player, x, y) -> bool
// L10
if !(game.tick() < context.setting.tower_attack_disable_tick) { return false; }   // vtable+0x28, +0x13f8
// L14~50 closure$0 (L52 에서 호출) — 캡처: cache, player, context, x, y
// L15
let champ = cache.player_champion[player.info.team][player.info.position]?;   // None → false
// L17
let towers = cache.towers(1 - player.info.team, context.pool);   // 적 타워 목록(bumpalo Vec<&Entity>)
// L19
for t in towers.iter() {
  // L20
  if t.ty 태그 != 2 (Tower) { continue; }
  // L21
  if let Some((_, id)) = tower.nearest_enemy {            // +0x88 태그 / +0x98 = .1
    // L22
    if id == champ.id { return true; }                    // 이미 나를 조준 중
    // L25
    let range = t.attack_effect.unwrap().range(t);        // = range + 15000 + stat_buff_cached.range + (level-1)*growth_range
    // L27~30
    let cnt = cache.iter_minions(player.info.team).filter(|m| t.attack_effect.unwrap().is_in_range(t, m)).count();   // 타워 사거리 안 내 미니언 수(aux m06/m11)
    // L33
    let dist = (x - t.x)² + (y - t.y)²;                   // abs_diff 제곱합
    // L34
    if cnt < 2 && dist <= (range + t.radius() + champ.radius())² { return true; }
  } else {
    // L39
    let dist = (x - t.x)² + (y - t.y)²;
    // L40
    let range = t.attack_effect.unwrap().range(t);
    // L42
    if dist <= (range + t.radius() + champ.radius())² { return true; }
  }
}
// L47~50
false
// Entity::radius(e) = if e.radius_mult == 0 { e.radius } else { e.radius * (radius_mult + 100) / 100 }   (entity.rs:1511~1515 인라인)
```

**`mem` 메모리 접근 24건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | GameContext | 0x0 | pool | r | cache.towers() 의 bumpalo 할당자 인자 | 4 | OK |
| 1 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |
| 2 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | usize. game.tick() 이 이 값 이상이면 즉시 false(타워 공격 비활성 구간) | 4 | OK |
| 3 | AbstractGameWithCache | 0x0 | game data ptr | r | dyn AbstractGame. vtable(+0x8) 슬롯 +0x28 = tick | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | [2][5] Option<&Entity>(니치 null). [team] stride 40, [position] 8. None → false | 4 | OK |
| 5 | PlayerState | 0x930 | info.team | r | usize. >=2 면 panic_bounds_check(len 2). 적팀 = 1 - team | 4 | OK |
| 6 | PlayerState | 0x9c0 | info.position (태그 i32) | r | player_champion 2차 인덱스 | 4 | OK |
| 7 | bumpalo Vec<&Entity>(towers, 32B) | 0x0 | ptr | r | cache.towers(enemy_team, pool) 반환 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 8 | bumpalo Vec<&Entity>(towers) | 0x18 | len | r |  | 4 | 확인불가(tcx 사전에 타입 없음) |
| 9 | Entity(타워) | 0x68 | ty 태그 | r | ==2(Tower) 인 원소만 검사, 아니면 skip | 4 | OK |
| 10 | Entity(타워) | 0x88 | ty.Tower.info.nearest_enemy 태그 | r | Option<(usize,usize)> (Tower+0x18). 0=None/1=Some | 4 | OK |
| 11 | Entity(타워) | 0x98 | ty.Tower.info.nearest_enemy.Some.1 | r | usize. 내 챔피언 id(+0x5c0) 와 비교 — 사용처로 보아 엔티티 id | 4 | OK |
| 12 | Entity(타워) | 0x4c0 | attack_effect 니치 판별자(casting) | r | == -1 이면 None → unwrap_failed 패닉 (타워는 항상 Some 전제) | 4 | OK |
| 13 | Entity(타워) | 0x490 | attack_effect.Some.0 (&Effect) | r | 술어 클로저에서 Effect::is_in_range 의 self | 4 | OK |
| 14 | Entity(타워) | 0x4a0 | attack_effect.range | r | Effect::range 인라인(effect.rs:26) | 4 | OK |
| 15 | Entity(타워) | 0x4a8 | attack_effect.growth_range | r | (level-1)*growth_range | 4 | OK |
| 16 | Entity(타워) | 0x438 | stat_buff_cached.range | r | Effect::range 에 가산 | 4 | OK |
| 17 | Entity(타워) | 0x5c8 | level | r | level-1 배수 | 4 | OK |
| 18 | Entity(타워) | 0x660 | x | r | dist² 계산 | 4 | OK |
| 19 | Entity(타워) | 0x668 | y | r |  | 4 | OK |
| 20 | Entity(타워·챔피언) | 0x470 | stat_buff_cached.radius_mult | r | i32. Entity::radius 인라인(entity.rs:1511): 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 | 4 | OK |
| 21 | Entity(타워·챔피언) | 0x680 | radius | r | usize. 타워 반경 + 내 챔피언 반경이 사거리에 가산 | 4 | OK |
| 22 | Entity(챔피언) | 0x5c0 | id | r | 타워 nearest_enemy.1 과 비교 | 4 | OK |
| 23 | iter_minions 반환 Chain(56B) | 0x0 | 3단 Chain<Copied<Iter<&Entity>>> 상태 | r | aux m06/m11: 각 슬라이스 (ptr,end) 쌍 3개 + 술어 캡처(타워 ptr, +0x38) | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 20 | 태그 | EntityType 태그 2 = Tower — towers() 결과 중 타워 엔티티만 검사 | 4 |
| 1 | 2 | 34 | 임계 | 타워가 다른 대상을 조준 중일 때: 타워 사거리 안 내 미니언 수 cnt < 2 여야 '나를 조준할 수 있음' 후보 | 4 |
| 2 | 15000 | 25 | 계수 | Effect::range(effect.rs:26) 인라인 — 사거리 = range + 15000 + stat_buff_cached.range + (level-1)*growth_range. L40 경로에도 동일 | 4 |
| 3 | -1 | 25 | 센티널 | Option<Effect> None 니치(casting == -1). attack_effect 없으면 unwrap 패닉(L25·L40, 술어 클로저 L28 도 동일) | 4 |
| 4 | 100 | 34 | 계수 | Entity::radius(entity.rs:1515) 인라인 — radius*(radius_mult+100)/100. L42 경로에도 동일 | 4 |
| 5 | 0 | 34 | 태그 | Entity::radius: radius_mult == 0 이면 곱 생략(radius 그대로) | 4 |
| 6 | 1 | 25 | 미상 | (level - 1) — `add i64 %level, -1` 로 나타남(IR 리터럴 -1) | 4 |
| 7 | 1 | 21 | 태그 | nearest_enemy Option 태그 1 = Some (trunc i64→i1) | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 타워가 다른 대상 조준 중일 때 '내가 대신 맞을' 판정의 내 미니언 수 임계 | tower_discipline.rs:34 | 2 | 올리면(예: 3) 미니언 2마리가 있어도 타워를 위험으로 봐서 더 소극적으로, 내리면(1) 미니언 1마리만 있어도 안전으로 봐서 더 공격적으로 타워 사거리에 들어간다 | 4 | 기존 |
| 1 | 타워 사거리 여유 가산 | effect.rs:26 (인라인, tower_discipline.rs:25/40) | 15000 | game_core Effect::range 공용 상수 — 이 함수만 바꿀 수 없다(모든 이펙트 사거리에 공통). 여기서 바꾸려면 재현 시 별도 상수로 분리 | 4 | 기존 |
| 2 | 타워 공격 비활성 tick 게이트 | tower_discipline.rs:10 (GameSetting+0x13f8) | tick < tower_attack_disable_tick | 설정값. 이 tick 이후엔 항상 false(타워를 위험으로 안 봄) | 4 | 기존 |
| 3 | 사거리 비교에 양쪽 반경 가산 | tower_discipline.rs:34/42 | range + t.radius() + champ.radius() | 챔피언 반경을 빼면 타워 경계에서 살짝 더 안쪽까지 안전으로 오판 | 4 | 기존 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_tower_focused | game_ai::can_tower_focused | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\tower_discipline.rs:9 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 6 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 7 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | towers | game_core::AbstractGameWithCache::<'a, 'b>::towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1859 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 4개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `size_hint`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 8곳** (m02.ll:37394, m02.ll:37398, m04.ll:47316, m07.ll:50596, m07.ll:50957, m11.ll:52095, m15.ll:11751, m15.ll:11755) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | Tower.nearest_enemy: Option<(usize,usize)> 의 .0 이 무엇인지(거리? tick?) — 이 함수는 .1 만 읽고 champ.id 와 비교하므로 .1=엔티티 id 는 사용처로 확정, .0 은 미탐색(game_core tower.rs 세팅 지점 안 읽음) | 4 |  |
| 1 | 표기 불가 | closure$0(L14~50) 이 즉시호출 클로저인지 `let f = \|..\| ..; f(..)` 인지 등 소스 표기는 표기 불가(외연 동일). TLS/메모 호출은 본문에 없음 | 4 |  |
| 2 | 미탐색 | AbstractGameWithCache::towers / iter_minions 의 본문(어떤 타워·미니언 집합을 주는지 — 살아있는 것만인지, 가시성 필터가 있는지)은 _gcbc 미탐색. 시그니처상 towers(cache, team, pool) -> bumpalo Vec<&Entity>, iter_minions(cache, team) -> 3단 Chain | 4 |  |
| 3 | 미탐색 | Effect::is_in_range(effect, caster, target) 본문(effect.rs:63) 미탐색 — 반경·사거리 계산이 L34 의 수식과 같은지 확인 안 함 | 4 |  |
| 4 | 미탐색 | L34 의 `cnt < 2 && dist <= R²` 에서 소스가 `&&` 두 항의 순서(cnt 먼저)인 것은 IR 분기 순서 기준(column 정보 없음) | 4 |  |
| 5 | 표기 불가 | L10 의 tick 비교가 `>=` 조기반환인지 `if tick < X { closure() } else { false }` 인지는 표기 불가(외연 동일) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

