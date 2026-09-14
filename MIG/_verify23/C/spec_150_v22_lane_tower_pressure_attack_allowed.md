---

### `150` v22_lane_tower_pressure_attack_allowed — 라인 타워를 직접 칠 수 있나 — v47 시즈 스탠스/커버, 미니언 커버 수, 라인전 페이즈, 마무리 가능성(1~2방), 적 챔피언 위협·타워 1방 감당 순으로 게이트

| 항목 | 값 |
|---|---|
| id | `tower_discipline__v22_lane_tower_pressure_attack_allowed` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline38v22_lane_tower_pressure_attack_allowed` |
| 소스 | `game-ai\src\tower_discipline.rs:394` |
| IR | `m07.ll` 51763~52386행 |
| 경로·가시성 | `game_ai::v22_lane_tower_pressure_attack_allowed` · **pub** |
| 계층 | 기타 |
| exe | `d99030` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문에서 직접 분기 없음 — v47_siege_stance(version,…) 에 그대로 전달만 (m07.ll:51834) | 4 |
| 1 | 2 | data | &OperationData(24B) | readonly. +0x0 cache · +0x8 context(GameContext) · +0x10 blackboard([Blackboard;2]) | 4 |
| 2 | 3 | player | &PlayerState(2528B) | readonly. info.team(+0x930)·info.position(+0x9c0) + is_recent_visible 인자로 전달 | 4 |
| 3 | 4 | tower | &Entity(1728B) | readonly. 공격 대상 적 라인 타워. ty(+0x68)==Tower(2) 일 때 Tower.nearest_enemy(+0x88 tag/+0x98 .1) 읽음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v22_lane_tower_pressure_attack_allowed(version, data, player, tower) -> bool   [tower_discipline.rs:394~470]
  L395: team = player.info.team; champ = data.cache.player_champion[team][player.info.position]; None → return false
  L398: champ_attack = champ.attack_effect(+0x490, tag +0x4c0); None → return false
  L401: if !champ.can_attack() → return false          (game_core 콜리)
  L407: if let Some(soaker) = v47_siege_stance(version, data, player, tower) {      // {i64,i64} = Option<entity id>
  L408:   if soaker == champ.id → return true                                          // 내가 소커
          if v47_tower_covered_for_me(data, tower, champ.id) → return true            // 인라인(597~613):
            //   tower.ty==Tower(2) && tower.nearest_enemy.is_some()(+0x88) 아니면 false
            //   target_id = nearest_enemy.1(+0x98); target_id == my_id → false
            //   target = game.get_entity_by_id(target_id)(vtable+0x1f0); None → false; tower.attack_effect None → false
            //   return target.hp > expected_damage_target(tower_attack, ctx, tower as dyn, target)    // 커버가 한 방에 안 끊김
        }
  L413: tower_attack = tower.attack_effect; None → return false
  L417: targeting = tower.ty==Tower && tower.nearest_enemy.is_some(); tower_target_id = nearest_enemy.1
  L425: tower_cover_count = 0; focused_cover_will_die = false
        for m in data.cache.iter_minions(team)   // 내 팀 top/mid/bottom 미니언 (Chain 56B, sret)
                 .filter(|m| tower_attack.is_in_range(tower, m))  {      // aux s_0 (m07.ll:62029)
  L427:   tower_cover_count += 1
  L428:   if targeting && m.id == tower_target_id {
  L429:     focused_cover_will_die = m.hp <= expected_damage_target(tower_attack, ctx, tower, m) } }
        // targeting 이 false 면 카운트만 세는 루프(%101) — will_die 는 false 유지
  L433: if !(tower_cover_count == 0 || (tower_cover_count == 1 && focused_cover_will_die)) → return true   // 커버가 서 있으면 허용
        //  IR: %178 = count<2 && (count!=1 || will_die); br %178 → 계속, else → ret true (phi [true,%172])
  L437: tick = game.tick()(vtable+0x28); if !ctx.is_line_phase(tick) → return false
        //  is_line_phase(runner.rs:399→setting.rs:703~704): tutorial ∉ {None,MidBottom,Line,Total} → true; 아니면 tick < sat_sub(first_spawn_tick, tps*30)
  L441: damage_to_tower = expected_damage_target(champ_attack, ctx, champ, tower)
  L442: if damage_to_tower == 0 → return false
  L446: finish_now  = damage_to_tower >= tower.hp        (IR 는 `ult` 를 계산해 dbg 에 DW_OP_not — 분기 방향으로 극성 확정)
  L447: finish_soon = saturating(damage_to_tower*2) >= tower.hp
  L459: if !finish_soon → return false
  L452: enemy_team = 1 - team
  L453~458: has_enemy_threat = data.cache.iter_champions(enemy_team)          // aux 2362~2534 (filter_map Some → filter s0_0 → any s1_0)
        .filter(|e| data.blackboard[enemy_team].is_recent_visible(game, player, e))     // ★인덱스가 enemy_team(1-team) 임 — IR 관측
        .any(|e| dist²(e, champ) <= max(max_range(e, champ)+50000, 120000)²  ||  dist²(e, tower) < 180000²+1)
  L461: tower_damage = expected_damage_target(tower_attack, ctx, tower, champ)
  L462: hp_floor = champ.stat_cached.hp >> 2
  L463: can_absorb_one_shot = tower_damage == 0 || champ.hp > hp_floor + tower_damage
  L465: if finish_now { L466: return can_absorb_one_shot || !has_enemy_threat }
  L469: else          { return tower_cover_count == 1 && can_absorb_one_shot && !has_enemy_threat }   // 이 지점에서 count ∈ {0, 1(will_die)}
  (phi 통합: %33/%34 = [true,%227 finish_now&tower_damage==0] [%245=can_absorb||!threat, %243] [!threat, %238] [false, %236/%240])
```

**`mem` 메모리 접근 21건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m07.ll:51787~51789 · ≥2 면 panic_bounds_check(len 2) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | player_champion[team][position] 인덱스 (m07.ll:51798~51800) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m07.ll:51801) | 4 | OK |
| 3 | OperationData | 0x8 | context | r | &GameContext — expected_damage_target 인자, is_line_phase, tutorial (m07.ll:51899~51900 · 51929~51930) | 4 | OK |
| 4 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — aux 이터레이터에서 [enemy_team] 인덱스(stride 744B) 로 is_recent_visible self (m07.ll:52288~52289 → aux 2431~2439) | 4 | OK |
| 5 | AbstractGameWithCache | 0x0 | game | r | &dyn AbstractGame (data ptr +0x0, vtable +0x8) — tick(vtable+0x28)·get_entity_by_id(vtable+0x1f0) 호출 (m07.ll:51879~51886 · 52217~52223) | 4 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | Option<&Entity> 니치. None → false (m07.ll:51803~51807) | 4 | OK |
| 7 | Entity | 0x4c0 | attack_effect@tag | r | champ(L398)·tower(L413)·target(L607 인라인) 세 곳. -1=None → false | 4 | OK |
| 8 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect(56B) 페이로드 선두 — champ_attack / tower_attack 로 expected_damage_target·is_in_range 에 전달 (m07.ll:51813 · 51849) | 4 | OK |
| 9 | Entity | 0x5c0 | id | r | champ.id(L408 소커 비교·L604 타겟 비교) · minion.id(L428 타워 타겟 비교) | 4 | OK |
| 10 | Entity | 0x68 | ty@tag | r | tower.ty == 2(Tower) 확인 (L598 인라인·L417) | 4 | OK |
| 11 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag | r | i64→trunc i1: Some 이면 타워가 타겟을 물고 있음 (L598·L417) | 4 | OK |
| 12 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 | r | 타워 타겟 엔티티 id 로 사용(get_entity_by_id 인자·minion.id 비교) — 튜플 .1 이 id 인 것은 사용처로 추정 | 5 | OK |
| 13 | Entity | 0x670 | hp | r | target.hp(L613) · tower.hp(L446·447) · minion.hp(L429) · champ.hp(L463) | 4 | OK |
| 14 | Entity | 0x628 | stat_cached.hp | r | champ 최대 HP — hp_floor = >>2 (L462) | 4 | OK |
| 15 | Entity | 0x660 | x | r | (aux 2362~) enemy/champ/tower 거리 계산 | 4 | OK |
| 16 | Entity | 0x668 | y | r | (aux) 거리 계산 | 4 | OK |
| 17 | GameContext | 0x38 | tutorial | r | TutorialType(1B) switch {0 None,5 MidBottom,7 Line,8 Total} — is_line_phase 인라인 (m07.ll:52225~52232) | 4 | OK |
| 18 | GameContext | 0x8 | setting | r | &GameSetting (m07.ll:52235) | 4 | OK |
| 19 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | 라인전 종료 기준 (setting.rs:703) | 4 | OK |
| 20 | GameSetting | 0x12f8 | tick_per_second | r | ×30 (setting.rs:704) | 4 | OK |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 395 | 태그 | team 인덱스 bounds(len 2) · 같은 2 가 EntityType::Tower 메모리태그(L598/L417 `icmp eq i64 %53, 2`) · L433 `tower_cover_count < 2` 에도 쓰임 | 4 |  |
| 1 | -1 | 398 | 센티널 | Entity.attack_effect Option 니치 None(i32 -1). champ(L398)·tower(L413)·target(L607) 모두 None → false/미커버 | 4 |  |
| 2 | 1 | 452 | 태그 | `1 - team` = enemy_team (m07.ll:52275). 같은 1 이 L427 count+1 · L433/L469 `count == 1`(단일 커버) 비교에도. (본문의 `shl … 1` 은 이 상수가 아니라 아래 folded_from 2 항목) | 4 |  |
| 3 | 0 | 442 | 임계 | damage_to_tower == 0 → false (m07.ll:52255). L463 tower_damage == 0 → can_absorb_one_shot = true (m07.ll:52337) | 4 |  |
| 4 | 1 | 447 | 태그 | finish_soon = damage_to_tower*2 >= tower.hp — `shl nuw i64 %199, 1`(m07.ll:52263) 로 접힘. 부호 오버플로(%206 slt 0) 면 포화 처리 → true | 4 | 2 |
| 5 | 2 | 462 | 임계 | hp_floor = champ.stat_cached.hp / 4 — `lshr i64 %230, 2`(m07.ll:52347) 로 접힘 | 4 | 4 |
| 6 | 30 | 437 | 계수 | is_line_phase(setting.rs:703~704 인라인): tick < saturating_sub(epic_jungle.first_spawn_tick, tps*30) — 에픽 첫 스폰 30초 전까지가 라인전 페이즈. tutorial ∈ {0,5,7,8} 에서만 검사, 그 외 튜토리얼은 항상 라인전 | 4 |  |
| 7 | 5 | 437 | 태그 | switch case: TutorialType MidBottom(5) — 0 None/7 Line/8 Total 과 함께 is_line_phase 검사 대상 (m07.ll:52227~52232 switch) | 4 |  |
| 8 | 7 | 437 | 태그 | switch case: TutorialType Line(7) | 4 |  |
| 9 | 8 | 437 | 태그 | switch case: TutorialType Total(8) | 4 |  |
| 10 | 50000 | 456 | 미상 | (aux m07.ll:2454) threat_range = max(max_range(enemy,champ) + 50000, 120000) — 적 사거리 + 50k 여유(≈1.56셀) | 4 |  |
| 11 | 120000 | 456 | 임계 | (aux m07.ll:2457 umax) threat_range 하한 120k(3.75셀) | 4 |  |
| 12 | 32400000001 | 458 | 미상 | (aux m07.ll:2517) 180000² + 1 — 적 챔피언이 타워에서 180k(5.6셀) 이내면 위협 (dist² < 32400000001 ⟺ dist <= 180000) | 4 |  |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 적 챔피언 위협 사거리 여유 | tower_discipline.rs:456 (aux m07.ll:2454) | 50000 | 올리면 더 먼 적도 위협으로 잡혀 타워 공격이 더 소극적(마무리 직전·감당 가능 아니면 false) | 4 | 기존 |
| 1 | 위협 사거리 하한 | tower_discipline.rs:456 (aux m07.ll:2456) | 120000 | 올리면 단거리 킷 적도 멀리서부터 위협으로 계산 | 4 | 기존 |
| 2 | 타워 주변 위협 반경(제곱) | tower_discipline.rs:458 (aux m07.ll:2517) | 32400000001 | 180k. 올리면 타워에서 더 먼 적까지 위협 → 압박 공격 감소 | 4 | 기존 |
| 3 | 라인전 페이즈 종료 마진(초) | setting.rs:704 (인라인, m07.ll:52243) | 30 | 올리면 에픽 스폰 더 이전에 커버 없는 타워 압박이 막힘(game_core 값이라 다른 사용처에도 영향) | 4 | 기존 |
| 4 | 마무리 임박 배수 | tower_discipline.rs:447 (shl 1) | 2 | 올리면(3방 이내 등) 더 이른 시점부터 커버 없는 타워 공격 허용 | 4 | 기존 |
| 5 | 타워 1방 감당 HP 바닥 분모 | tower_discipline.rs:462 (lshr 2) | 4 | 분모를 올리면(바닥 낮아짐) 낮은 HP 로도 타워 한 방을 감당한다고 판단 → 더 공격적 | 4 | 기존 |

<details><summary>`callees` 피호출자 19건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | max_range | game_ai::max_range | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2234 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | v22_lane_tower_pressure_attack_allowed | game_ai::v22_lane_tower_pressure_attack_allowed | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:394 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | v47_siege_stance | game_ai::v47_siege_stance | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> std::option::Option<usize> | game-ai\src\tower_discipline.rs:508 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | v47_tower_covered_for_me | game_ai::v47_tower_covered_for_me | pub | fn(&game_core::OperationData, &game_core::Entity, usize) -> bool | game-ai\src\tower_discipline.rs:597 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 4개**: `enemy_team`, `sat_sub`, `saturating`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m02.ll:48235, m03.ll:139939, m14.ll:23402, m15.ll:22326) · **형제 0개** 

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | v47_siege_stance(version, data, player, tower) -> {i64,i64}(Option<usize> 소커 챔피언 id · tag 0/1) 내부는 m07.ll:48393(src 508) 별도 명세 대상 — 계약만 | 4 |  |
| 1 | 미탐색 | v47_tower_covered_for_me(data, tower, my_champion_id) (src 597~613) 는 별도 define 없이 인라인 — 위 logic 에 인라인 본문을 풀어 적음. 자체 시그니처는 IR 에 남지 않음 | 4 |  |
| 2 | 미탐색 | Blackboard::is_recent_visible(&Blackboard(744B), &dyn AbstractGame, &PlayerState, &Entity)->bool 내부(_gcbc/g07.ll:157005)는 안 봄. ★blackboard 인덱스가 enemy_team(=1-team, alloca %9 값) 인 것은 IR 관측 사실 — 의미(적 팀 블랙보드 기준 '최근 시야')는 미확인 | 4 |  |
| 3 | 미탐색 | max_range(&Entity enemy, &Entity champ)->u64 (m10.ll:52940) 내부는 안 봄 — 적 킷 최대 사거리로 추정(이름·용법) | 4 |  |
| 4 | 미탐색 | Effect::expected_damage_target(&Effect, &GameContext, caster &dyn(ptr,vtable @anon.11=Entity vtable), target &Entity)->u64 · Effect::is_in_range(&Effect, &Entity caster, &Entity target)->bool · Entity::can_attack(&Entity)->bool 은 game_core 경계 — 시그니처만 | 4 |  |
| 5 | 미탐색 | iter_minions(sret 56B = Chain<Chain<Copied<Iter<&Entity>>,Copied<Iter>>,Copied<Iter>>, &cache, team): 관측 레이아웃 +0x0 i64 외곽 Option 태그, +0x8/+0x18/+0x28 각 슬라이스 ptr(null=None) — 어느 슬라이스가 top/mid/bottom 인지 순서는 game_core 경계라 미확인(카운트 합산이라 결과 무관) | 4 |  |
| 6 | 미탐색 | Tower.nearest_enemy: Option<(usize,usize)> 의 .0 의미(거리로 추정) — 이 함수는 .1 만 쓴다 | 5 |  |
| 7 | 표기 불가 | L446/L447 소스 표기가 `>=` 인지 `tower.hp <= dmg` 인지 — 외연 동일(표기 불가) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

