---

### `37` i_am_chosen_defender — 아군 5명을 (푸셔 감당 가능, 웨이브 클리어 점수, 넥서스 거리, id) 키로 줄 세워 내가 상위 need_count 안에 드는지

| 항목 | 값 |
|---|---|
| id | `defense_nexus__i_am_chosen_defender` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus20i_am_chosen_defender` |
| 소스 | `game-ai\src\plan_legacy\old\defense_nexus.rs:297` |
| IR | `m04.ll` 58647~59462행 |
| 경로·가시성 | `game_ai::plan_legacy::old::i_am_chosen_defender` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `13882720` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, usize, &[usize]) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | 판단 주체. info.team(+0x930)·info.position 태그(+0x9c0) 로 내 챔피언을 찾고, is_recent_visible 에 넘긴다 | 4 |
| 1 | 2 | data | &OperationData(24B) | cache(+0x0)·context(+0x8)·blackboard(+0x10) 모두 사용 | 4 |
| 2 | 3 | need_count | usize | 필요 수비 인원. 0 이면 즉시 false | 4 |
| 3 | 4 | exclude | &[usize] (ptr %3, len %4) | 제외할 챔피언 id 목록. 내 id 가 있으면 false, 아군 id 가 있으면 순위 비교에서 제외 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn i_am_chosen_defender(player, data, need_count, exclude) -> bool

[L298] if need_count == 0 { return false }
[L301] team = player.info.team(+0x930); nexus = data.cache.nexus(+0x170)[team]?   // None → false
[L304] my_champ = data.cache.player_champion(+0x1e0)[team][player.info.position 태그(+0x9c0)]?   // None → false
[L308] if exclude.contains(&my_champ.id(+0x5c0)) { return false }   // 8개 청크 unrolled + 나머지 루프(and !7 / and 7 는 contains 최적화)

// ── 우리 구조물(넥서스/쌍둥이 타워)을 치는 적 미니언 웨이브 ────────────
[L313] rep_minion: Option<&Entity> = None; wave_size = 0
[L315] for m in data.cache.iter_minions(1 - team) {                 // Chain 이터레이터, next() 3구간
[L316]   if m.ty(+0x68) != 1 /*Minion*/ { continue }
[L317]   let Some(id) = m.ty.Minion.info.nearest_enemy(+0x88/+0x90) else { continue }
         let Some(s) = game.get_entity_by_id(id) /*vtable+0x1f0*/ else { continue }
[L318]   if !(s.team(+0x0) == TeamType::Player(team)) { continue }          // 우리 편 구조물
         if !(s.ty == 3 /*Nexus*/ || (s.ty == 2 /*Tower*/ && s.ty.Tower.info.ty(+0x128) ∈ {TwinA,TwinB})) { continue }
[L323]   wave_size += 1
[L324]   if rep_minion.is_none() { [L325] rep_minion = Some(m) }   // 첫 번째 해당 미니언이 대표
       }
[L329] wave_size = max(wave_size, 1)

// ── 푸셔(넥서스 근처 최근접 적 챔프) ────────────────────────────────
[L332] pusher = None; pusher_dist = u64::MAX
[L334] for c in data.cache.iter_champions(1 - team) {               // player_champion[enemy] 의 Some 만
[L335]   if !data.blackboard(+0x10)[1 - team].is_recent_visible(game, player, c) { continue }
[L338]   d = distance_sq(c, nexus)
[L339]   if d < 90000000001 /*300000²*/ && d < pusher_dist { [L341] pusher = Some(c); pusher_dist = d }
       }

// ── 키 = (cant_handle_pusher, -clear, dist², id)  낮을수록 적합 ──────
[L345] key = |ally: &Entity| {                                       // ★closure#0 = aux m04.ll 15665~15883
[L346]   cant_handle_pusher = pusher.map_or(false, |p| battle::expected_trade_net_hp(data, ally, p) < 0)
[L347]   clear = rep_minion.map_or(0, |minion| defense_wave_clear_score(ally, ctx, minion, wave_size))
           // defense_nexus.rs:455 인라인:
           //   score  = attack_effect.is_some() ? Effect::expected_damage_target(&ally.attack_effect(+0x490), ctx, ally as &dyn AbstractEntity, minion) : 0
           //   score += area_skill_clear(skill_effect(+0x4c8))                       // 469: Some && dmg>0 && is_area ? dmg*wave_size : 0
           //   score += area_skill_clear(level(+0x5c8)>2 ? skill2_effect(+0x500) : None)
           //   score += area_skill_clear(level>4 ? ult_effect(+0x538) : None)
[L348]   (cant_handle_pusher, -(clear as i64), distance_sq(ally, nexus), ally.id)
       }
[L351] my_key = key(my_champ)
[L353] better = 0
[L354] for p in 0..5 { let Some(ally) = data.cache.player_champion[team][p] else { continue }
[L357]   if ally.id == my_champ.id { continue }
[L360]   if exclude.contains(&ally.id) { continue }
[L363]   if key(ally) < my_key { [L364] better += 1 }     // 튜플 사전순: bool(false<true) → i64 signed → u64 → u64
       }
[L367] return better < need_count

※ 부작용 없음. 순위는 '푸셔를 감당할 수 있는가 → 웨이브 클리어 기대치 높은 순 → 넥서스에 가까운 순 → id 작은 순'.
```

**`mem` 메모리 접근 26건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. >=2 면 panic_bounds_check | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 태그를 그대로 [5] 배열 인덱스로 씀(zext) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 3 | OperationData | 0x8 | context | r | &GameContext — key 클로저에서 expected_damage_target 인자 | 4 | OK |
| 4 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. [1-team](적팀) 블랙보드의 is_recent_visible 을 부른다 | 4 | OK |
| 5 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame data) | r | is_recent_visible 인자 + vtable+0x1f0 get_entity_by_id(L317) | 4 | OK |
| 6 | AbstractGameWithCache | 0x8 | game vtable | r |  | 4 | OK |
| 7 | AbstractGameWithCache | 0x170 | nexus[team] | r | [Option<&Entity>;2]. None → false | 4 | OK |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [[Option<&Entity>;5];2] stride 40/8. 내 챔프 None → false. [1-team] 은 iter_champions(적 챔프) 원본, [team][0..5] 는 아군 순위 루프 | 4 | OK |
| 9 | Entity | 0x0 | team@tag (TeamType) | r | L318: 미니언 대상 엔티티의 팀 태그 0=Player 이어야 | 4 | OK |
| 10 | Entity | 0x8 | team@Player.0 | r | L318: == 내 team 이어야(우리 구조물을 치는 미니언) | 4 | OK |
| 11 | Entity | 0x68 | ty@tag (EntityType) | r | L316: 1=Minion / L318: 대상 3=Nexus·2=Tower | 4 | OK |
| 12 | Entity(적 미니언) | 0x88 | ty@Minion.info.nearest_enemy@tag | r | Option<usize>. 1=Some | 4 | OK |
| 13 | Entity(적 미니언) | 0x90 | ty@Minion.info.nearest_enemy@Some.0 | r | 대상 id → get_entity_by_id | 4 | OK |
| 14 | Entity(대상 타워) | 0x128 | ty@Tower.info.ty (TowerType) | r | (v-3) <u 2 ⇒ TwinA/TwinB 만 | 4 | OK |
| 15 | Entity | 0x5c0 | id | r | exclude 대조·자기 제외·키 4번째 성분 | 4 | OK |
| 16 | Entity | 0x660 | x | r | distance_sq(적 챔프↔넥서스 / 아군↔넥서스) | 4 | OK |
| 17 | Entity | 0x668 | y | r |  | 4 | OK |
| 18 | Entity(aux key 클로저: ally) | 0x4c0 | attack_effect@tag | r | i32 니치. -1 이면 None → 평타 점수 0 | 4 | OK |
| 19 | Entity(aux) | 0x490 | attack_effect (Effect 56B) | r | expected_damage_target 의 &Effect | 4 | OK |
| 20 | Entity(aux) | 0x4f8 | skill_effect@tag | r | -1 이면 None | 4 | OK |
| 21 | Entity(aux) | 0x4c8 | skill_effect | r |  | 4 | OK |
| 22 | Entity(aux) | 0x500 | skill2_effect | r | level > 2 일 때만, 아니면 정적 None(@anon.16, +0x30 == -1) | 4 | OK |
| 23 | Entity(aux) | 0x538 | ult_effect | r | level > 4 일 때만 | 4 | OK |
| 24 | Entity(aux) | 0x5c8 | level | r | usize. 슬롯 개방 게이트 | 4 | OK |
| 25 | Effect(aux) | 0x30 | casting@tag (Option<Effect> 니치) | r | -1 이면 None (슬롯2·3) | 4 | OK |

**`consts` 상수 19건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 298 | 태그 | need_count == 0 → false | 4 |
| 1 | 2 | 301 | 태그 | team 인덱스 상한(panic_bounds_check len=2) / L318 대상 태그 2=Tower | 4 |
| 2 | 1 | 315 | 인덱스 | enemy = 1 - team (iter_minions·iter_champions·blackboard 모두 적팀 인덱스) | 4 |
| 3 | 1 | 316 | 태그 | 미니언 ty 태그 1=Minion | 4 |
| 4 | 1 | 317 | 태그 | nearest_enemy Option 태그 1=Some (trunc i1) | 4 |
| 5 | 0 | 318 | 태그 | 대상 엔티티 team 태그 0=Player (trunc i1 false) 이고 team 값 == 내 team | 4 |
| 6 | 3 | 318 | 태그 | 대상 ty 태그 3=Nexus → 웨이브에 산입 | 4 |
| 7 | -3 | 318 | 미상 | (TowerType-3) <u 2 ⇒ TwinA(3)/TwinB(4) 쌍둥이 타워를 치는 미니언만 산입(tower.rs:91 인라인) | 4 |
| 8 | 1 | 323 | 태그 | wave_size += 1 | 4 |
| 9 | 1 | 329 | 태그 | wave_size = max(wave_size, 1) — 미니언이 없어도 클리어 점수 계산은 1 웨이브 기준 | 4 |
| 10 | -1 | 332 | 센티널 | pusher_dist 초기값 u64::MAX (Option<usize> 대신 센티넬) | 4 |
| 11 | 90000000001 | 339 | 임계 | 300000²+1 — 넥서스 300000 이내 + 최근 가시인 적 챔프 중 최근접 = pusher | 4 |
| 12 | 4 | 353 | 임계 | 아군 슬롯 루프 0..5 의 종료 비교 `icmp ult %172, 4` | 4 |
| 13 | 1 | 364 | 태그 | better += 1 (키가 나보다 작은 아군) | 4 |
| 14 | 63 | 348 | 미상 | aux: cant_handle_pusher = expected_trade_net_hp(data, ally, pusher) < 0 → `lshr 63` 부호비트 추출 | 4 |
| 15 | -1 | 460 | 센티널 | aux: Option<Effect> 니치 None (attack/skill/skill2/ult 슬롯의 casting 태그) | 4 |
| 16 | 2 | 464 | 임계 | aux: level > 2 여야 skill2 슬롯 열림(entity.rs:1693 접근자) | 4 |
| 17 | 4 | 465 | 임계 | aux: level > 4 여야 ult 슬롯 열림(entity.rs:1701) | 4 |
| 18 | 0 | 474 | 태그 | aux: area_skill_clear — 기대 피해 > 0 이고 is_area 일 때만 dmg*wave_size 가산(`icmp sgt i64 %40, 0`) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 푸셔 탐색 반경(적 챔프↔우리 넥서스) | defense_nexus.rs:339 (IR m04.ll:59375 `icmp ult i64 %272, 90000000001`) | 90000000001 | 300000². 올리면 더 먼 적 챔프도 '푸셔'로 잡혀 cant_handle_pusher 가 순위 1순위 기준으로 자주 개입하고, 내리면 넥서스 바로 근처에서만 | 4 | 기존 |
| 1 | 웨이브에 산입하는 구조물 종류 | defense_nexus.rs:318 (IR m04.ll:59431 `add nsw i8 %307, -3` / `icmp ult i8 %308, 2`) | Nexus / TwinA / TwinB | 외곽 타워를 치는 미니언은 wave_size·rep_minion 에 안 들어간다. 넓히면 라인 푸시에도 광역기 챔프가 수비수로 뽑힌다 | 4 | 기존 |
| 2 | wave_size 하한 | defense_nexus.rs:329 (IR m04.ll:58991 `llvm.umax.i64(%99, 1)`) | 1 | 광역 스킬 클리어 점수 = dmg × wave_size. 하한을 올리면 미니언이 적어도 광역기 챔프가 우선 선발된다 | 4 | 기존 |
| 3 | 스킬 슬롯 개방 레벨 | defense_nexus.rs:464~465 (aux m04.ll:15733/15771 `icmp ugt %50, 2` / `icmp ugt %50, 4`) | 2 / 4 | level ≤2 면 skill2, ≤4 면 ult 이 클리어 점수에서 빠진다(Entity 접근자 인라인 — 게임 규칙이라 사실상 고정) | 4 | 기존 |
| 4 | 키 우선순위 | defense_nexus.rs:348 | (cant_handle_pusher, -clear, dist², id) | 성분 순서를 바꾸면 선발 기준이 바뀐다(예: 거리 우선). 현재는 '푸셔 감당 가능' 이 절대 우선 | 4 | 기존 |

<details><summary>`callees` 피호출자 31건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | area_skill_clear | game_ai::plan_legacy::old::defense_nexus::area_skill_clear | in:game_ai::plan_legacy::old::defense_nexus | fn(&game_core::OperationData, &game_core::Entity, std::option::Option<&game_core::Effect>, &game_core::Entity, usize) -> i64 | game-ai\src\plan_legacy\old\defense_nexus.rs:469 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | contains | game_core::RectU64::contains | pub | fn(&game_core::RectU64, u64, u64) -> bool | game-core\src\setting.rs:40 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | contains | game_core::RectI64::contains | pub | fn(&game_core::RectI64, i64, i64) -> bool | game-core\src\setting.rs:55 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | contains | game_core::setting::champion::nightmare::NightmareWellRect::contains | in:game_core::setting::champion::nightmare | fn(game_core::setting::champion::nightmare::NightmareWellRect, u64, u64) -> bool | game-core\src\setting\champion\nightmare.rs:28 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | defense_wave_clear_score | game_ai::plan_legacy::old::defense_nexus::defense_wave_clear_score | in:game_ai::plan_legacy::old::defense_nexus | fn(&game_core::OperationData, &game_core::Entity, std::option::Option<&game_core::Entity>, usize) -> i64 | game-ai\src\plan_legacy\old\defense_nexus.rs:455 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | expected_damage_target | game_core::Projectile::expected_damage_target | pub | fn(&game_core::Projectile, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\projectile.rs:1287 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | expected_trade_net_hp | game_ai::plan_legacy::old::expected_trade_net_hp | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> i64 | game-ai\src\plan_legacy\old\battle.rs:2493 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | i_am_chosen_defender | game_ai::plan_legacy::old::i_am_chosen_defender | pub | fn(&game_core::PlayerState, &game_core::OperationData, usize, &[usize]) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:297 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | is_area | game_core::Effect::is_area | pub | fn(&game_core::Effect) -> bool | game-core\src\simulation\effect.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 10개 중 상위 3개 |
| 16 | is_area | game_core::EffectType::is_area | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:340 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 10개 중 상위 3개 |
| 17 | is_area | <game_core::RangeEffect as game_core::EffectType>::is_area | pub | fn(&game_core::RangeEffect) -> bool | game-core\src\simulation\effect\type\range_effect.rs:34 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 10개 중 상위 3개 |
| 18 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 22 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 23 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 24 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 25 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 26 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 27 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 29 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 30 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 4개**: `bool`, `map_or`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `player_champion`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 14곳** (m04.ll:59580, m04.ll:59728, m04.ll:59761, m04.ll:59883, m04.ll:60200, m09.ll:8594, m09.ll:9181, m09.ll:32574, m09.ll:33156, m09.ll:34190, m09.ll:34620, m09.ll:35517, m09.ll:36325, m13.ll:32811) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | expected_trade_net_hp(battle.rs:2493)·Effect::expected_damage_target·is_area·Blackboard::is_recent_visible 내부는 안 읽었다(별도 함수·game_core) — 여기선 호출 계약(인자·부호 판정)만 확정 | 4 |  |
| 1 | 미탐색 | L363 키 비교의 튜플 성분 순서(bool@16 → i64@0 → u64@8 → u64@24)는 IR 비교 순서로 복원한 것 — 메모리 오프셋은 rustc 재배치 순이라 소스 튜플 선언 순과 다를 수 있으나 비교 순서 = 선언 순이라 논리는 확정 | 4 |  |
| 2 | 미탐색 | L308/L360 exclude.contains 의 `and 1152921504606846968`(= !7)·`and 7`·8 단위 unroll 은 slice::contains 의 청크 최적화 — 판정 상수가 아니라 constants 에서 뺐다 | 4 |  |
| 3 | 미탐색 | iter_minions 의 Chain 3구간(next() 호출 3개)이 어떤 미니언 집합(라인별?)을 잇는지는 game_core 미독 — 이 함수는 전 구간을 균일하게 훑는다 | 4 |  |
| 4 | 미탐색 | player.info.position 태그(i32)를 인덱스로 쓰므로 position 열거형의 태그 0..4 가 슬롯 순서라는 가정 — tcxdict --enum 으로 확인 안 함(범위 밖) | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

