---

### `238` v22_current_line_non_champion_action_tower_risk — 라인전 구간에서 비챔피언(미니언/타워 등) 대상 행동 중 적 타워에 물릴 위험이 있는가 — 사거리 안 적 타워가 나를 조준 중이거나, 최근 적 타워에 맞았고 커버가 즉시 끊기면 true

| 항목 | 값 |
|---|---|
| id | `tower_discipline__v22_current_line_non_champion_action_tower_risk` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline47v22_current_line_non_champion_action_tower_risk` |
| 소스 | `game-ai\src\tower_discipline.rs:225` |
| IR | `m07.ll` 53032~53218행 |
| 경로·가시성 | `game_ai::v22_current_line_non_champion_action_tower_risk` · **pub** |
| 계층 | 기타 |
| exe | `d9a220` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, &game_core::Entity) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | %0. 분기 없음 | 4 |
| 1 | 2 | context | &GameContext(64B) | %1. +0x38 tutorial · +0x8 setting 만 읽음 | 4 |
| 2 | 3 | cache | &AbstractGameWithCache(8840B) | %2. game dyn 팻포인터(+0 data · +8 vtable) · player_champion(+0x1e0) · iter_towers_without_nexus 호출 self | 4 |
| 3 | 4 | player | &PlayerState(2528B) | %3. info.team(+0x930) · info.position(+0x9c0) 만 읽음. v22_enemy_tower_cover_breaks_immediately 에 그대로 전달 | 4 |
| 4 | 5 | target | &Entity(1728B) | %4. 현재 행동 대상(비챔피언). ty 태그(+0x68)==Tower 만 판정에 쓰임 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v22_current_line_non_champion_action_tower_risk(version, context, cache, player, target) -> bool
// version(%0) 은 본문에서 한 번도 읽지 않는다.
L225: tick = cache.game.tick()
      // context.is_line_phase(tick) 인라인(runner.rs:399 → spawn_epic runner.rs:263 → setting.rs:703)
      spawn_epic = context.tutorial ∈ {None(0), MidBottom(5), Line(7), Total(8)}
      if spawn_epic && !(tick < setting.epic_jungle.first_spawn_tick.saturating_sub(setting.tick_per_second*30)) → return false
      // (tutorial 이 그 외 값이면 시간 판정 없이 통과)
L226: if !(cache.game.tick() < setting.tower_attack_disable_tick) → return false   // tick 재호출
L230: team = player.info.team (bounds <2 · panic_bounds_check)
      champ = cache.player_champion[team][player.info.position.as_index()]  ; None(null) → return false
L233~235: recently_hit_by_enemy_tower =
      champ.last_attacked_from                       // Option<usize> (+0x28 tag / +0x30 id)
        .and_then(|id| cache.game.get_entity_by_id(id))   // L234 closure$0
        .is_some_and(|e| e.team == TeamType::Player(1 - team) && e.is_tower())   // L235 closure$1 : +0x0==0 && +0x8==1-team && +0x68==2
L237: iter = cache.iter_towers_without_nexus(1 - team)
L238: .filter(|t| t.can_target())        // closure$2 = entity.rs:1478 : t.can_target(+0x6b9) && t.block_target_tick(+0x6a0)==0
L239: .any(|tower| {                     // closure$3 (aux m06/m11)
  L240:   let Some(attack) = tower.attack_effect.as_ref() else { return false }   // +0x4c0 == -1 → false
  L244:   if !attack.is_in_range(tower, champ) { return false }
  L248:   if tower.ty == Tower(2) && tower.info.nearest_enemy.is_some()(+0x88)
  L249:       && nearest_enemy.0(+0x98) == champ.id(+0x5c0) { return true }   // 타워가 나를 조준 중
  L254:   if target.is_tower() {                                  // target +0x68 == 2
  L255:       recently_hit_by_enemy_tower && v22_enemy_tower_cover_breaks_immediately(context, cache, player, tower)
          } else {
  L258:       v22_enemy_tower_cover_breaks_immediately(context, cache, player, tower)
  L262~263:   && recently_hit_by_enemy_tower
          }   // ★두 가지의 외연이 동일(피연산자 순서만 다름 · 부작용 없음) — target.is_tower() 분기는 결과에 영향 없음
      })
L264: return any 결과

반환 phi(%82): L225 false / L226 false / L230 None false / any 결과
```

**`mem` 메모리 접근 24건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | GameContext | 0x38 | tutorial | r | runner.rs:263 spawn_epic 인라인 — switch {0 None,5 MidBottom,7 Line,8 Total} 이면 에픽 스폰 모드 → is_line_phase 시간 판정 수행. 그 외(1~4,6) 는 is_line_phase 무조건 true | 4 | OK |
| 1 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |
| 2 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | setting.rs:703 is_line_phase | 4 | OK |
| 3 | GameSetting | 0x12f8 | tick_per_second | r | setting.rs:704 — ×30 | 4 | OK |
| 4 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | tower_discipline.rs:226 — tick 이 이 값 이상이면 false | 4 | OK |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 | 4 | OK |
| 6 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 +0x28 tick · +0x1f0 get_entity_by_id (divtable AbstractGame) | 3 | OK |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [[Option<&Entity>;5];2] · gep [5 x ptr] stride 40B · null=None | 4 | OK |
| 8 | PlayerState | 0x930 | info.team | r | usize · bounds check <2 (panic_bounds_check) | 4 | OK |
| 9 | PlayerState | 0x9c0 | info.position@tag | r | i32 → entity.rs:581 as_index → zext 그대로 인덱스 | 4 | OK |
| 10 | Entity(champ) | 0x28 | last_attacked_from@tag | r | Option<usize> — 하위 1비트 trunc 로 Some 판정 | 4 | OK |
| 11 | Entity(champ) | 0x30 | last_attacked_from@Some.0 | r | 엔티티 id → get_entity_by_id | 4 | OK |
| 12 | Entity(champ) | 0x5c0 | id | r | aux(m06/m11): tower.nearest_enemy.0 과 비교 | 4 | OK |
| 13 | Entity(last_attacker) | 0x0 | team@tag | r | TeamType 판별자 0=Player (entity.rs:1127 PartialEq) | 4 | OK |
| 14 | Entity(last_attacker) | 0x8 | team@Player.0 | r | == 1 - player.info.team (적 팀) | 4 | OK |
| 15 | Entity(last_attacker) | 0x68 | ty@tag | r | == 2 Tower (entity.rs:1386 is_tower) | 4 | OK |
| 16 | Entity(target) | 0x68 | ty@tag | r | aux: == 2 Tower (tower_discipline.rs:254) — 분기만 가르고 결과식은 양쪽 동일 | 4 | OK |
| 17 | Entity(tower) | 0x6b9 | can_target | r | aux: entity.rs:1478 can_target() 1항 | 4 | OK |
| 18 | Entity(tower) | 0x6a0 | block_target_tick | r | aux: entity.rs:1478 can_target() 2항 (==0) | 4 | OK |
| 19 | Entity(tower) | 0x4c0 | attack_effect@tag(niche i32) | r | aux: -1 = None → 이 타워 skip | 4 | OK |
| 20 | Entity(tower) | 0x490 | attack_effect@Some.0 | r | aux: &Effect → Effect::is_in_range(effect, tower, champ) | 4 | OK |
| 21 | Entity(tower) | 0x68 | ty@tag | r | aux: == 2 Tower (tower_discipline.rs:248) | 4 | OK |
| 22 | Entity(tower) | 0x88 | ty@Tower.info.nearest_enemy@tag | r | aux: Option<(usize,usize)> Some 판정(하위 1비트) | 4 | OK |
| 23 | Entity(tower) | 0x98 | ty@Tower.info.nearest_enemy@Some.0 | r | aux: 조준 대상 엔티티 id == champ.id 이면 즉시 true | 4 | OK |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 225 | 태그 | TutorialType::None(태그 0) — switch case. runner.rs:263 spawn_epic | 4 |
| 1 | 5 | 225 | 태그 | TutorialType::MidBottom(태그 5) — switch case | 4 |
| 2 | 7 | 225 | 태그 | TutorialType::Line(태그 7) — switch case | 4 |
| 3 | 8 | 225 | 태그 | TutorialType::Total(태그 8) — switch case | 4 |
| 4 | 30 | 225 | 계수 | setting.rs:704 — tick_per_second×30 = 30초. 에픽 첫 스폰 30초 전까지가 라인전 구간(saturating_sub) | 4 |
| 5 | 2 | 230 | 태그 | player.info.team 경계검사 상한(팀 2). 또한 entity.rs:1386 is_tower 의 EntityType 태그 2=Tower(235·248·254), TeamType 판별자 비교 | 4 |
| 6 | 1 | 235 | 인덱스 | 1 - team = 적 팀 인덱스 (235 · 237 iter_towers_without_nexus 인자) | 4 |
| 7 | 0 | 235 | 태그 | TeamType 태그 0 = Player (entity.rs:1127 PartialEq) | 4 |
| 8 | -1 | 240 | 센티널 | aux: Option<Effect> 니치 태그 -1 = None (attack_effect 없음 → skip) | 4 |
| 9 | 0 | 238 | 태그 | aux: entity.rs:1478 can_target(): block_target_tick == 0 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 라인전 구간 종료 여유(에픽 첫 스폰 전 초) | game-core setting.rs:704 (is_line_phase, 인라인) | 30 | 올리면 라인전 구간이 더 일찍 끝나 이 위험 판정이 더 빨리 항상 false(비활성)가 된다. game_core 상수라 game_ai 재현에서는 GameSetting 값으로 재현 | 4 | 기존 |
| 1 | 타워 공격 비활성 시각 | GameSetting.tower_attack_disable_tick (+0x13f8) · tower_discipline.rs:226 | 설정값 | tick 이 이 값 이상이면 무조건 false. 값을 올리면 판정 활성 구간이 길어진다 | 4 | 기존 |

<details><summary>`callees` 피호출자 22건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | is_tower | game_core::EntityType::is_tower | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1385 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | v22_enemy_tower_cover_breaks_immediately | game_ai::tower_discipline::v22_enemy_tower_cover_breaks_immediately | in:game_ai | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:198 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | version | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | fn(&game_ai::AgentVerHamster) -> usize | game-ai\src\lib.rs:428 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 20 | version | game_core::AiAgent::version | pub | fn(&Self/#0) -> usize | game-core\src\simulation\ai_interface.rs:503 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 21 | version | <game_core::Team as engine_core::spitz::SpitzDatable>::version | pub | fn() -> usize | game-core\src\data\team.rs:1696 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
</details>

⚠**미매칭 1개**: `block_target_tick`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m14.ll:28912) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L226 의 `tick < tower_attack_disable_tick` 이어야 통과하는 극성이 필드명(타워 공격 '비활성' 시각)과 직관이 반대로 보이나 IR 이 정본 — 필드 의미(비활성 시작 시각인지 종료 시각인지)는 game_core setting.rs 미독으로 미확정(미탐색 = `_gcbc` 에서 tower_attack_disable_tick 소비처 grep) | 3 |  |
| 1 | 미탐색 | L254~263: target.is_tower() 분기의 두 결과식이 외연 동일(recently_hit && cover_breaks). 소스에 259~261 줄이 존재(주석/빈 줄 추정)하나 IR 에 명령이 없어 내용 미확정 — 동작에는 영향 없음 | 4 |  |
| 2 | 미탐색 | v22_enemy_tower_cover_breaks_immediately 본문(m07.ll:52389~) 은 담당 범위 밖 — 반환 의미는 이름 기반 추정 | 4 |  |
| 3 | 미탐색 | iter_towers_without_nexus 의 배열 6칸(Flatten<array::IntoIter<Option<&Entity>,6>>) + 슬라이스 부분이 각각 무엇(외곽/내곽 타워?)인지는 game_core 본문 미독 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | Tower.nearest_enemy: Option<(usize,usize)> 의 두 번째 usize 의미 미확정(본문에서 첫 원소만 사용) | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

