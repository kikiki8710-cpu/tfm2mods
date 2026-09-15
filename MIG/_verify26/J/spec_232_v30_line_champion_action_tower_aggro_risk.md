---

### `232` v30_line_champion_action_tower_aggro_risk — 라인전 구간에서 적 챔피언 대상 공격/스킬 행동이 (즉사시키지 못하면서) 시전 위치를 적 타워 사거리(+15000) 안에 두어 타워 어그로를 끌 위험이 있는가

| 항목 | 값 |
|---|---|
| id | `tower_discipline__v30_line_champion_action_tower_aggro_risk` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline41v30_line_champion_action_tower_aggro_risk` |
| 소스 | `game-ai\src\tower_discipline.rs:160` |
| IR | `m07.ll` 52737~53029행 |
| 경로·가시성 | `game_ai::v30_line_champion_action_tower_aggro_risk` · **pub** |
| 계층 | 기타 |
| exe | `d99f60` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_ai::SmallActionPlay) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | %0 | 4 |
| 1 | 2 | data | &OperationData(24B) | %1. +0 cache · +8 context | 4 |
| 2 | 3 | player | &PlayerState(2528B) | %2. info.team(+0x930) · info.position(+0x9c0) | 4 |
| 3 | 4 | action | &SmallActionPlay(184B) | %3. 니치 태그 +0xb1 → Attack/Skill/Skill2/Ult 만 통과 · 페이로드 +0x8 target(entity id) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v30_line_champion_action_tower_aggro_risk(version, data, player, action) -> bool
// version(%0) 미사용
L160: tick = cache.game.tick(); if !context.is_line_phase(tick) → return false
      // is_line_phase = !(tutorial ∈ {None,MidBottom,Line,Total}) || tick < first_spawn_tick.saturating_sub(tps*30)
L161: if !(cache.game.tick() < setting.tower_attack_disable_tick) → return false
L165: champ = cache.player_champion[team][pos]? ; None → return false
L169: small_action = action.get_action()   // SmallActionPlay 니치 태그 복원 → Attack/Skill/Skill2/Ult 만, 그 외 → return false
L170: target_id = small_action.get_action_target()  // 4종 모두 payload +0x8 target (blackboard.rs:104~106)
L173: target = cache.game.get_entity_by_id(target_id)? ; None → return false
L176: if target.team == champ.team || !target.is_champion() → return false
L180: effect = v30_line_action_effect(champ, small_action) 인라인(L123~127):
        Attack → champ.attack_effect.as_ref()  / Skill → skill_effect.as_ref()
        Skill2 → champ.skill2_effect() (level>2 아니면 None) / Ult → champ.ult_effect() (level>4 아니면 None)
      None → return false
L183: if !CastingTarget::check(&effect.target, champ, target) → return false
L187: if !(expected_damage_target(effect, context, champ, target) < target.hp) → return false   // 한 방에 죽이면 위험 아님
L191: (stance_x, stance_y) = v30_line_action_stance(Some(context), champ, target, effect)
L192: cache.iter_towers_without_nexus(1 - team)
L193:   .filter(|t| t.can_target())            // +0x6b9 && +0x6a0==0
L194:   .any(|tower| tower.attack_effect.as_ref()
L195:        .is_some_and(|ta| ta.is_in_range_ex(tower, champ, tower.x, tower.y, stance_x, stance_y, 15000)))
L196: return any 결과

반환 phi: 모든 조기 탈출 = false, any 결과만 true 가능
```

**`mem` 메모리 접근 34건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r |  | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |
| 2 | GameContext | 0x38 | tutorial | r | runner.rs:263 spawn_epic 인라인 switch {0,5,7,8} | 4 | OK |
| 3 | GameContext | 0x8 | setting | r |  | 4 | OK |
| 4 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | setting.rs:703 is_line_phase | 4 | OK |
| 5 | GameSetting | 0x12f8 | tick_per_second | r | ×30 | 4 | OK |
| 6 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | L161 | 4 | OK |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr | r |  | 4 | OK |
| 8 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 +0x28 tick(L160·L161 2회) · +0x1f0 get_entity_by_id(L173) | 4 | OK |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | stride 40B · null=None | 4 | OK |
| 10 | PlayerState | 0x930 | info.team | r | bounds <2 | 4 | OK |
| 11 | PlayerState | 0x9c0 | info.position@tag | r | as_index | 4 | OK |
| 12 | SmallActionPlay | 0xb1 | @tag(niche) | r | small_action.rs:309 get_action: 태그 15/16/17/18 → SmallAction Attack(6)/Skill(7)/Skill2(8)/Ult(9), 그 외 → return false | 4 | OK |
| 13 | SmallActionPlay | 0x8 | @Attack\|Skill\|Skill2\|Ult.0.target | r | SmallActionAttack/Skill 공통 +0x8 target: usize → blackboard.rs:104~106 get_action_target | 4 | OK |
| 14 | Entity(target) | 0x0 | team@tag | r | L176 entity.rs:1127 TeamType PartialEq | 4 | OK |
| 15 | Entity(target) | 0x8 | team@Player.0 | r | == champ 의 것이면 return false | 4 | OK |
| 16 | Entity(target) | 0x68 | ty@tag | r | L176 entity.rs:1404 is_champion == 13 | 4 | OK |
| 17 | Entity(target) | 0x670 | hp | r | L187: expected_damage < hp 여야 진행 | 4 | OK |
| 18 | Entity(champ) | 0x0 | team@tag | r | L176 비교 상대 | 4 | OK |
| 19 | Entity(champ) | 0x8 | team@Player.0 | r |  | 4 | OK |
| 20 | Entity(champ) | 0x4c0 | attack_effect@tag(niche i32) | r | L124 Attack: -1 → return false | 4 | OK |
| 21 | Entity(champ) | 0x490 | attack_effect@Some.0 | r | effect (Attack) | 4 | OK |
| 22 | Entity(champ) | 0x4f8 | skill_effect@tag(niche i32) | r | L125 Skill | 4 | OK |
| 23 | Entity(champ) | 0x4c8 | skill_effect@Some.0 | r | effect (Skill) | 4 | OK |
| 24 | Entity(champ) | 0x5c8 | level | r | entity.rs:1693 skill2_effect(): level>2 / entity.rs:1701 ult_effect(): level>4 아니면 정적 NONE(@anon.31, 태그 -1) | 4 | OK |
| 25 | Entity(champ) | 0x500 | skill2_effect | r | L126 · 태그 +0x530(=+48) | 4 | OK |
| 26 | Entity(champ) | 0x538 | ult_effect | r | L127 · 태그 +0x568(=+48) | 4 | OK |
| 27 | Effect | 0x28 | target(CastingTarget) | r | L183 CastingTarget::check(&effect.target, champ, target) | 4 | OK |
| 28 | Entity(tower) | 0x6b9 | can_target | r | aux L193 filter entity.rs:1478 | 4 | OK |
| 29 | Entity(tower) | 0x6a0 | block_target_tick | r | aux == 0 | 4 | OK |
| 30 | Entity(tower) | 0x4c0 | attack_effect@tag(niche i32) | r | aux L194 -1 → skip | 4 | OK |
| 31 | Entity(tower) | 0x490 | attack_effect@Some.0 | r | aux tower_attack | 4 | OK |
| 32 | Entity(tower) | 0x660 | x | r | aux L195 is_in_range_ex caster_x | 4 | OK |
| 33 | Entity(tower) | 0x668 | y | r | aux L195 caster_y | 4 | OK |

**`consts` 상수 18건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 160 | 태그 | TutorialType::None 태그 0 (switch case, runner.rs:263 spawn_epic) | 4 |
| 1 | 5 | 160 | 태그 | TutorialType::MidBottom 태그 5 | 4 |
| 2 | 7 | 160 | 센티널 | TutorialType::Line 태그 7. 또한 SmallActionPlay 니치 복원 select 기본값 7(=untagged AroundPosition 논리 idx) · SmallAction::Skill 태그 7 | 4 |
| 3 | 8 | 160 | 태그 | TutorialType::Total 태그 8. 또한 SmallAction::Skill2 태그 8 | 4 |
| 4 | 30 | 160 | 계수 | setting.rs:704 tick_per_second×30 = 에픽 첫 스폰 30초 전까지 라인전 | 4 |
| 5 | 2 | 165 | 임계 | team 경계검사 상한. 또한 entity.rs:1693 skill2_effect 의 level > 2 임계 | 4 |
| 6 | 10 | 169 | 센티널 | SmallActionPlay 니치 태그 10 은 불가값(llvm.assume ne 10 — 컴파일러 힌트, 판정 아님) | 4 |
| 7 | -3 | 169 | 센티널 | 니치 태그→논리 idx 복원: tag - 3 (niche_start=3) — small_action.rs:309 get_action 인라인 | 4 |
| 8 | 12 | 169 | 태그 | SmallActionPlay 논리 idx 12 = Attack → SmallAction::Attack(6) | 4 |
| 9 | 13 | 169 | 태그 | 논리 idx 13 = Skill → SmallAction::Skill(7). 또한 L176 EntityType 태그 13 = Champion(is_champion) | 4 |
| 10 | 14 | 169 | 태그 | 논리 idx 14 = Skill2 → SmallAction::Skill2(8) | 4 |
| 11 | 15 | 169 | 태그 | 논리 idx 15 = Ult → SmallAction::Ult(9) | 4 |
| 12 | 6 | 170 | 태그 | SmallAction::Attack 태그 6 (blackboard.rs:104 get_action_target 경유 phi) | 4 |
| 13 | 9 | 170 | 태그 | SmallAction::Ult 태그 9 | 4 |
| 14 | 1 | 192 | 태그 | 1 - team = 적 팀 (iter_towers_without_nexus 인자) | 4 |
| 15 | -1 | 180 | 센티널 | Option<Effect> 니치 None (4종 이펙트 as_ref) | 4 |
| 16 | 4 | 180 | 임계 | entity.rs:1701 ult_effect(): level > 4 아니면 NONE | 4 |
| 17 | 15000 | 195 | 미상 | aux: Effect::is_in_range_ex 의 마지막 인자(사거리 여유 가산 15000 — 추정: v47 의 Effect::range 여유와 같은 값) | 5 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 시전 위치의 타워 사거리 여유 | tower_discipline.rs:195 (is_in_range_ex 마지막 인자) | 15000 | 올리면 타워에서 더 먼 시전 위치도 어그로 위험으로 본다(라인전 트레이드 보수화). 0 이면 순수 사거리 | 4 | 기존 |
| 1 | 라인전 구간 종료 여유 | game-core setting.rs:704 | 30 | 올리면 판정 활성 구간이 짧아진다(game_core 상수) | 4 | 기존 |
| 2 | 킬 면책 | tower_discipline.rs:187 | expected_damage < target.hp | 없애면 처형각도 타워 위험으로 막힌다 | 4 | 기존 |

<details><summary>`callees` 피호출자 27건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 5 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 6 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 7 | get_action_target | game_core::SmallAction::get_action_target | pub | fn(&game_core::SmallAction) -> std::option::Option<usize> | game-core\src\simulation\game\blackboard.rs:101 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | is_champion | game_core::EntityType::is_champion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | is_in_range_ex | game_core::Effect::is_in_range_ex | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity, u64, u64, u64, u64, u64) -> bool | game-core\src\simulation\effect.rs:78 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 14 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 15 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | v30_line_action_effect | game_ai::tower_discipline::v30_line_action_effect | in:game_ai | fn(&game_core::Entity, game_core::SmallAction) -> std::option::Option<&game_core::Effect> | game-ai\src\tower_discipline.rs:122 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 22 | v30_line_action_stance | game_ai::tower_discipline::v30_line_action_stance | in:game_ai | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::Effect) -> (u64, u64) | game-ai\src\tower_discipline.rs:132 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | v30_line_champion_action_tower_aggro_risk | game_ai::v30_line_champion_action_tower_aggro_risk | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_ai::SmallActionPlay) -> bool | game-ai\src\tower_discipline.rs:158 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 24 | version | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | fn(&game_ai::AgentVerHamster) -> usize | game-ai\src\lib.rs:428 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 25 | version | game_core::AiAgent::version | pub | fn(&Self/#0) -> usize | game-core\src\simulation\ai_interface.rs:503 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 26 | version | <game_core::Team as engine_core::spitz::SpitzDatable>::version | pub | fn() -> usize | game-core\src\data\team.rs:1696 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
</details>

⚠**미매칭 1개**: `target`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 8곳** (m01.ll:13120, m01.ll:13145, m01.ll:13736, m01.ll:13761, m01.ll:20411, m01.ll:20436, m01.ll:20583, m01.ll:20608) · **형제 0개** 

**`open` 2건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | v30_line_action_stance 본문(m07.ll:50193~50440)은 계약만 읽음 — (x,y) 산출 규칙(사거리 안이면 현 위치 / 아니면 접근점 · usub.sat 15000) 은 스킴 수준 추정 | 4 |  |
| 1 | 미탐색 | SmallActionPlay 니치 태그 10 이 어느 variant 에 해당하는지(assume ne 10 — tcxdict 표엔 태그 10 이 없음: AroundPosition 은 암묵) — 판정에 영향 없음 | 3 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L161 tower_attack_disable_tick 극성/의미 — v22 명세와 동일 미확정 | 4 | 사실 서술 |
| 1 | is_in_range_ex 의 15000 이 소스에서 리터럴인지 상수명인지(_docs 0건) — 값만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

