---

### `227` v47_tower_focus_position_dangerous — (x,y)가 적 타워에 피격될 위험 위치인가 — 사거리(+반지름·15000 여유) 안이면서 시즈 스탠스(소커/커버)로 면책되지 않는 타워가 하나라도 있으면 true

| 항목 | 값 |
|---|---|
| id | `tower_discipline__v47_tower_focus_position_dangerous` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline34v47_tower_focus_position_dangerous` |
| 소스 | `game-ai\src\tower_discipline.rs:620` |
| IR | `m07.ll` 51323~51760행 |
| 경로·가시성 | `game_ai::v47_tower_focus_position_dangerous` · **pub** |
| 계층 | 기타 |
| exe | `d98ac0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &game_core::PlayerState, u64, u64) -> bool
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[227]/sig/tls/<키>`)**

- `name`: SIEGE_STANCE_CACHE
- `role`: 소비자(간접) — 본 함수 본문에는 LocalKey/@anon fn-포인터 상수 참조가 0건. 콜리 v47_siege_stance(m07.ll:48393) 가 @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.173 = `SIEGE_STANCE_CACHE…call_once` 경유로 읽고(closure$0, tower_discipline.rs:516~527) 미스 시 계산 후 쓴다(closure s_0, 532~537). 따라서 본 함수 호출 1회 = 후보 타워마다 TLS 접근 1~2회
- `key`: (player.info.team: usize, tower.id: usize) — v47_siege_stance L515 (m07.ll:48449~48455)
- `layout`: thread_local RefCell<SiegeStanceCache>: +0x0 RefCell borrow flag(i64) · +0x8 map: HashMap<(usize,usize), Option<usize>, ahash::RandomState>(64B) · +0x48 seed: u64 · +0x50 tick: usize (tcxdict SiegeStanceCache 80B: map@0x0 seed@0x40 tick@0x48, RefCell 헤더 8B 가산). 값 = Option<usize> 소커 챔피언 entity id
- `invalidation`: reader closure$0(L518~521): c.seed != game.seed() || c.tick != game.tick() 이면 c.seed/c.tick 갱신 + map.clear() 후 미스. 즉 **틱 단위 캐시** — 같은 (seed, tick) 안에서만 유효. writer closure(L534~535): seed·tick 이 일치할 때만 insert(key, v)
- `call_conditions`: v47_siege_stance 는 tower.ty==Tower(+0x68==2) && tower.can_target() 일 때만 TLS 접근(L509 · 아니면 None 즉시). 본 함수는 iter_towers(1-team) 의 각 타워 중 ty==Tower && attack_effect.is_some() 인 것마다 in_range 계산 직후(L637) 호출 — in_range 여부와 무관하게 호출된다
- `order`: 본 함수 안에서 TLS 접점은 v47_siege_stance 1곳뿐(L637). 같은 틱에 다른 팀원/다른 호출자가 먼저 채워두면 여기서는 읽기만 발생

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | %0 | 4 |
| 1 | 2 | data | &OperationData(24B) | %1. +0 cache · +8 context 읽음. v47_siege_stance 에 전달 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | %2. info.team(+0x930) · info.position(+0x9c0). v47_siege_stance 에 전달 | 4 |
| 3 | 4 | x | u64 | %3. 판정 위치 x (distance_sq 의 abs_diff 피연산자) | 4 |
| 4 | 5 | y | u64 | %4. 판정 위치 y | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v47_tower_focus_position_dangerous(version, data, player, x, y) -> bool
L620: if !(cache.game.tick() < context.setting.tower_attack_disable_tick) → return false
L623: champ = cache.player_champion[player.team][player.position.as_index()] ; None → return false
L627: for t in cache.iter_towers(1 - team) {            // 적 타워(넥서스 포함 여부는 game_core 미독)
L628:   let EntityType::Tower(info) = &t.ty else { continue }        // +0x68 == 2
L631:   let Some(attack) = t.attack_effect.as_ref() else { continue }  // +0x4c0 == -1 → continue
L634:   range = attack.range(t, 15000) + t.radius() + champ.radius()      // 인자 형태는 추정(unknown 참조)
          // Effect::range(effect.rs:26) 인라인 = attack.range + 15000 + t.stat_buff_cached.range + attack.growth_range*(t.level-1)
          // radius() = radius_mult==0 ? radius : radius*(radius_mult+100)/100
L635:   in_range = dx² + dy² <= range²   (dx=|x - t.x|, dy=|y - t.y| · utils.rs:7~9 distance_sq)
L637:   match v47_siege_stance(version, data, player, t) {
        Some(soaker) => {
L638:     if soaker == champ.id { continue }                          // 내가 소커 → 이 타워는 위험 아님
L641:     if let Some((id, _)) = info.nearest_enemy {
L642:       if id == champ.id { return true }                       // 타워가 나를 조준 중 → in_range 무관 true
L645:       covered = v47_tower_covered_for_me(data, t, champ.id)   // 인라인 L598~614: t.ty==Tower && get_entity_by_id(id) 존재 && attack 존재 && target.hp > expected_damage_target(attack, ctx, t, target)
            if covered || !in_range { continue } else { return true }
          } else {
L650:       if in_range { return true } else { continue }
          }
        }
        None => {
L657:     if let Some((id, _)) = info.nearest_enemy {
L658:       if id == champ.id { return true }                       // in_range 무관 true
L661:       cnt = cache.iter_minions(team).filter(|m| attack.is_in_range(t, m)).count()   // 내 팀 미니언 중 타워 사거리 안 수(aux)
L664:       if cnt < 2 && in_range { return true } else { continue }
          } else {
L667:       if in_range { return true } else { continue }
          }
        }
      }
    }
L673: return false

※ `dangerous = in_range`(dbg 이름) 이 기본이고, 스탠스/커버/미니언 수는 그것을 면책하는 쪽으로만 작용한다. 예외 2곳(L642·L658: 타워가 이미 나를 조준)은 위치와 무관하게 true.
```

**`mem` 메모리 접근 26건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext → +0x8 setting | 4 | OK |
| 2 | GameContext | 0x8 | setting | r |  | 4 | OK |
| 3 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | L620: tick 이 이 값 이상이면 false | 4 | OK |
| 4 | AbstractGameWithCache | 0x0 | game.data_ptr | r |  | 4 | OK |
| 5 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 +0x28 tick(L620) · +0x1f0 get_entity_by_id(L607 인라인 v47_tower_covered_for_me) | 4 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [[Option<&Entity>;5];2] · stride 40B · null=None | 4 | OK |
| 7 | PlayerState | 0x930 | info.team | r | bounds <2 | 4 | OK |
| 8 | PlayerState | 0x9c0 | info.position@tag | r | as_index (entity.rs:581) | 4 | OK |
| 9 | Entity(champ) | 0x5c0 | id | r | L638 소커 비교 · L642/L658 타워 조준 대상 비교 | 4 | OK |
| 10 | Entity(champ) | 0x470 | stat_buff_cached.radius_mult | r | L634 champ.radius() (entity.rs:1511~1515) | 4 | OK |
| 11 | Entity(champ) | 0x680 | radius | r | L634 champ.radius() | 4 | OK |
| 12 | Entity(tower) | 0x68 | ty@tag | r | L628 == 2 Tower (아니면 continue) · L598 재확인 | 4 | OK |
| 13 | Entity(tower) | 0x4c0 | attack_effect@tag(niche i32) | r | L631 -1 = None → continue | 4 | OK |
| 14 | Entity(tower) | 0x490 | attack_effect@Some.0 | r | &Effect attack → Effect::range 인라인 · expected_damage_target · is_in_range(aux) | 4 | OK |
| 15 | Entity(tower) | 0x4a0 | attack_effect@Some.0.range | r | effect.rs:26 Effect::range 인라인 | 4 | OK |
| 16 | Entity(tower) | 0x4a8 | attack_effect@Some.0.growth_range | r | × (level-1) | 4 | OK |
| 17 | Entity(tower) | 0x5c8 | level | r | growth_range 계수 | 4 | OK |
| 18 | Entity(tower) | 0x438 | stat_buff_cached.range | r | 사거리 버프 가산 | 4 | OK |
| 19 | Entity(tower) | 0x470 | stat_buff_cached.radius_mult | r | tower.radius() | 4 | OK |
| 20 | Entity(tower) | 0x680 | radius | r | tower.radius() | 4 | OK |
| 21 | Entity(tower) | 0x660 | x | r | L635 distance_sq | 4 | OK |
| 22 | Entity(tower) | 0x668 | y | r | L635 distance_sq | 4 | OK |
| 23 | Entity(tower) | 0x88 | ty@Tower.info.nearest_enemy@tag | r | L641/L657 조준 대상 유무 | 4 | OK |
| 24 | Entity(tower) | 0x98 | ty@Tower.info.nearest_enemy@Some.0 | r | 조준 대상 entity id (L642·L658 == champ.id / L607 get_entity_by_id) | 4 | OK |
| 25 | Entity(tower_target) | 0x670 | hp | r | L613: hp > expected_damage_target → 커버가 한 방에 안 끊김 | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 623 | 태그 | team 경계검사 상한(2팀). 또한 EntityType 태그 2=Tower(L628·L598), L664 `cnt < 2`(커버 미니언 2마리 미만) | 4 |
| 1 | 1 | 627 | 미상 | 1 - team = 적 팀 (iter_towers 인자). 또한 L634 growth_range × (level - 1) 의 -1(add -1) | 4 |
| 2 | -1 | 631 | 센티널 | Option<Effect> 니치 태그 -1 = None (attack_effect 없음 → continue). 이터레이터 Chain 상태 -1/-2 는 내부값(판정 아님) | 4 |
| 3 | 15000 | 634 | 계수 | 사거리 여유 가산(셀 32000 의 약 0.47). add 명령의 DILocation 이 L26<634(effect.rs:26 Effect::range 인라인) — 같은 Effect::range 인라인이 line_defense.rs:59(unsafe_v19_non_champion_walkup)에선 15000 대신 caster_radius 를 같은 자리에 더하므로 `Effect::range(&self, caster, extra)` 의 extra 인자로 L634 에서 15000 을 넘긴 것으로 추정(add 는 nsw 없는 재결합 가능 연산) | 5 |
| 4 | 100 | 634 | 계수 | entity.rs:1515 radius(): radius × (radius_mult + 100) / 100 — 퍼센트 배율 | 4 |
| 5 | 0 | 634 | 태그 | entity.rs:1512 radius_mult == 0 이면 radius 그대로(곱셈 생략) | 4 |
| 6 | 6 | 627 | 길이 | iter_towers 전반부 배열 [Option<&Entity>;6] 길이(assume) — 판정 아님 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 타워 사거리 여유 | game-core effect.rs:26 Effect::range (인라인 · 본 함수 L634) | 15000 | 올리면 더 먼 위치까지 위험으로 본다(타워 접근 보수화). game_core 상수라 game_ai 재현 시 그대로 복제 필요 | 4 | 기존 |
| 1 | 무스탠스 시 커버 미니언 최소 수 | tower_discipline.rs:664 | 2 | 올리면(예: 3) 미니언 2마리가 사거리 안에 있어도 위험으로 판정 → 더 보수적. 내리면(1) 미니언 1마리만 있어도 안전으로 본다 | 4 | 기존 |
| 2 | 커버 생존 판정 | tower_discipline.rs:613 (v47_tower_covered_for_me 인라인) | target.hp > expected_damage_target (1샷) | '2샷' 등으로 바꾸면 커버 인정이 엄격해져 위험 판정이 늘어난다 | 4 | 기존 |
| 3 | 타워 공격 비활성 시각 | GameSetting +0x13f8 · L620 | 설정값 | tick 이 이 값 이상이면 무조건 false | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | v47_siege_stance | game_ai::v47_siege_stance | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> std::option::Option<usize> | game-ai\src\tower_discipline.rs:508 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | v47_tower_covered_for_me | game_ai::v47_tower_covered_for_me | pub | fn(&game_core::OperationData, &game_core::Entity, usize) -> bool | game-ai\src\tower_discipline.rs:597 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | v47_tower_focus_position_dangerous | game_ai::v47_tower_focus_position_dangerous | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\tower_discipline.rs:619 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

**호출처 1곳** (m14.ll:29309) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L620 극성(tick < tower_attack_disable_tick 일 때만 판정 진행)이 필드명과 직관이 반대로 보이나 IR 정본 — 필드 의미는 game_core 미독(v22 명세와 동일 항목) | 3 |  |
| 1 | 미탐색 | iter_towers 가 넥서스를 포함하는지(iter_towers_without_nexus 와 대비) — game_core 본문 미독. 본 함수는 ty==Tower 로 다시 거르므로 넥서스(ty 3)는 어차피 continue | 4 |  |
| 2 | 미탐색 | iter_towers 후반부 `Iterator::next` 외부 콜리(gc::…Entity…Iterator4next)의 구체 이터레이터 타입 — 계약(Option<&Entity>) 만 확인 | 4 |  |
| 3 | 미탐색 | v47_siege_stance 본문(소커 선정 규칙·resolve_fight 연동)은 담당 밖 — 반환 계약만 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | Effect::range(effect.rs:26) 의 시그니처 — 15000 add 가 L26 귀속이고, line_defense.rs:59 의 같은 인라인은 그 자리에 caster_radius 를 더하므로 `range(&self, caster, extra: u64)` 로 추정. 인자 순서·이름은 game_core 소스 부재로 미확정(add 재결합 가능) | 5 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

