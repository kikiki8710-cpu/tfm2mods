---

### `76` EpicStanceData::update_plan — 에픽 태세(None/Check/CheckTry) 결정: 에픽 존재·시야로 hp 갱신 → 아군/적 처치시간 산출 → 준비되면 CheckTry, 적이 먹을 낌새면 인원차 확인 후 Check

| 항목 | 값 |
|---|---|
| id | `goal_data__EpicStanceData_update_plan` |
| 심볼 | `_RNvMs_NtCshdEBA0ozCnw_7game_ai9goal_dataNtB4_14EpicStanceData11update_plan` |
| 소스 | `game-ai\src\goal_data.rs:266` |
| IR | `m09.ll` 41134~42817행 |
| 경로·가시성 | `game_ai::EpicStanceData::update_plan` · **pub** |
| 계층 | 기타 |
| exe | `de1ee0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::EpicStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData)
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | self | &mut EpicStanceData(56B) = GoalData+0x78 | last_epic_seen@0 last_epic_hp@8 epic_enemy_tick@0x10 epic_enemy_killed_tick@0x18 epic_ally_tick@0x20 epic_ally_killed_tick@0x28 stance@0x30(EpicStance 1B) | 4 |
| 1 | 1 | (version) | usize | 호출자가 poison — 미사용 | 4 |
| 2 | 2 | (rnd) | ptr | readnone — 미사용 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | team@0x930 만 사용 | 4 |
| 4 | 4 | data | &OperationData(24B) | cache@0 context@8 (blackboard 미사용 — 세르펜판과 차이) | 4 |
| 5 | 5 | enemy_region | &[Option<EnemyRegionInfo>;5] (= &GoalData 오프셋 0) | closure#4 에서만 읽음 | 4 |
| 6 | 6 | (debug) | ptr | readnone — 에픽판은 디버그 출력 없음(GoalData::update 도 poison 으로 넘김) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn update_plan(&mut self, _v, _rnd, player, data, enemy_region, _debug):
  moba = get_game_mode(); if tag!=0 { self.stance=None; return }                     // :268~270
  epic_state = &moba.jungle_runner.epic (+0x198)                                       // :271
  epic = epic_state.live_list.get(0).and_then(get_entity_by_id); if None { self.stance=None; return }   // :272~274
  if self.last_epic_seen < epic_state.next_respawn_tick { self.last_epic_hp = epic.stat_cached.hp; self.last_epic_seen = tick() }   // :280~282
  if is_visible(player.team, epic.id) { self.last_epic_seen = tick(); self.last_epic_hp = epic.hp }   // :284~286
  ally_in_epic = (0..5).filter(|p| player_champion[team][p].is_some_and(|c| map.region_dist(map.regions[c.cell], 7) == 0))   // :289~299 closure#2 — 세르펜판(150k 거리)과 달리 '에픽 리전 안' 판정
  as_entity = ally_in_epic.filter_map(|p| player_champion[team][p])                     // :301 closure#3
  self.epic_ally_tick        = check_epic_kill_time_with_hp(ctx, as_entity, epic, self.last_epic_hp)   // :305
  self.epic_ally_killed_tick = check_epic_killed_time(epic, as_entity)                 // :306
  enemy_in_epic = (0..5).filter(|p| enemy_region[p].is_none_or(|e| {                    // :308~324 closure#4
        echamp = player_champion[1-team][p]? (None→false)                                // :310~311
        dist = distance(region_center(e.region), region_center(7))                        // :312~315
        move_tick = dist / echamp.stat_cached.move_speed (0 → div0 패닉)                  // :316~317
        if e.last_known + move_tick <= tick() { true } else { region_dist(e.region, 7) == 0 } }))   // :319~320
  as_entity = enemy_in_epic.filter_map(|p| player_champion[1-team][p])                  // :326 closure#5
  self.epic_enemy_tick        = check_epic_kill_time_with_hp(ctx, as_entity, epic, last_epic_hp)   // :330
  self.epic_enemy_killed_tick = check_epic_killed_time(epic, as_entity)                 // :332
  live_ally  = cache.champions(team)      // :334 (세르펜판의 라인페이즈 targets 필터 없음)
  live_enemy = cache.champions(1-team)    // :335
  live_ally_tick        = check_epic_kill_time_with_hp(ctx, live_ally, epic, last_epic_hp)   // :337
  live_ally_killed_tick = check_epic_killed_time(epic, live_ally)                       // :338
  ally_near  = iter_champions(team).filter(|c| region_dist(regions[c.cell],7) < 4 && c.hp*100/c.max_hp > 49).count()   // :340~343 closure#6
  enemy_near = iter_champions(1-team).filter(|c| region_dist(regions[c.cell],7) < 4).count()                            // :346~348 closure#7
  unseen = tick().saturating_sub(self.last_epic_seen)                                    // :351
  ready = (ally_near >= enemy_near && live_ally_tick + tps*2 <= live_ally_killed_tick)   // :352~353
       || (self.epic_ally_tick <= tps*5 && self.epic_ally_tick + tps*3 <= self.epic_ally_killed_tick)   // :356~357 (IR: ally_near<enemy_near 면 :356 만, 아니면 :353||:356)
  remain = self.epic_enemy_tick.saturating_sub(unseen)                                   // :362
  a = remain < tps*20 ; b = unseen >= tps*3 ; c = remain < tps*10                          // :362~364
  if ready { self.stance = CheckTry(2) }                                                 // :368~369
  else if self.epic_enemy_killed_tick < self.epic_enemy_tick.saturating_sub(tps*3) { self.stance = None(0) }   // :370~371 적이 에픽에게 먼저 죽음 → 위협 아님
  else if !((a && b) || c) { self.stance = None(0) }                                     // :372,:380 아직 임박 아님
  else if live_enemy.len >= live_ally.len + 2 { self.stance = None(0) }                  // :374~375 인원 2+ 열세면 견제 포기
  else { self.stance = Check(1) }                                                        // :377
  // :382 return
```

**`mem` 메모리 접근 36건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r |  | 4 | OK |  |
| 1 | OperationData | 0x8 | context | r |  | 4 | OK |  |
| 2 | AbstractGameWithCache | 0x0 | game.data_ptr(&dyn AbstractGame) | r | vtable +0x28 tick · +0x40 get_game_mode · +0xf8 is_visible · +0x1f0 get_entity_by_id | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | iter_champions(team)(:340,:346) · champions(team)(:334~335, g15.ll:109887 = Some 슬롯 전부 수집, 생사 필터 없음) · 클로저 | 4 | OK |  |
| 4 | MobaMode | 0x198 | jungle_runner.epic (epic_state) | r | GameMode::Moba 페이로드가 &MobaMode. live_list.ptr@0x1a0 len@0x1a8 | 4 | OK |  |
| 5 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | :272 live_list.get(0) | 4 | OK |  |
| 6 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 → epic None | 4 | OK |  |
| 7 | MobaMode | 0x1b0 | jungle_runner.epic.next_respawn_tick | r | :280 | 4 | OK |  |
| 8 | PlayerState | 0x930 | info.team | r |  | 4 | OK |  |
| 9 | Entity | 0x5c0 | id | r | epic.id → is_visible | 4 | OK |  |
| 10 | Entity | 0x628 | stat_cached.hp | r | epic 최대 HP(:281) · 챔프 최대 HP(closure#6 :343) | 4 | OK |  |
| 11 | Entity | 0x670 | hp | r | epic 현재 HP(:286) · champ.hp(check_epic_killed_time :571 · closure#6) | 4 | OK |  |
| 12 | Entity | 0x660 | x | r | 셀 변환 | 4 | OK |  |
| 13 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 14 | Entity | 0x640 | stat_cached.move_speed | r | closure#4 내부 :316 | 4 | OK |  |
| 15 | Entity | 0x490 | attack_effect@Some.0 | r | check_epic_killed_time :568 | 4 | OK |  |
| 16 | Entity | 0x4c0 | attack_effect@tag(-1=None) | r | None → unwrap 패닉 | 4 | OK |  |
| 17 | GameContext | 0x0 | pool(&Bump) | r | :334~335 champions() 의 할당자 | 4 | OK |  |
| 18 | GameContext | 0x8 | setting | r |  | 4 | OK |  |
| 19 | GameContext | 0x20 | map | r |  | 4 | OK |  |
| 20 | GameSetting | 0x12f8 | tick_per_second | r | tps | 4 | OK |  |
| 21 | MapDef | 0x38b8 | regions[cy][cx] | r | closure#2 :297 · closure#6 :341 · closure#7 :347 | 4 | OK |  |
| 22 | MapDef | 0x5510 | region_dist[a][7] (=region_dist@0x54d8 + 7*8) | r | gep +21776(0x5510) 후 a*27*8. closure#2 :298 ==0 · closure#4 :320 ==0 · closure#6/#7 :342,:348 <4. 리전 7 = 에픽 리전(추정) | 5 | OK |  |
| 23 | MapDef | 0x6ba0 | region_centers[e.region] / region_centers[7] | r | closure#4 내부 :312,:314 (+27552 base, +27664/+27672 = region_centers[7].x/.y) | 4 | OK |  |
| 24 | EpicStanceData | 0x0 | last_epic_seen | r | :280,:351 | 4 | OK |  |
| 25 | EpicStanceData | 0x8 | last_epic_hp | r | :305 로드 → 3회 check_epic_kill_time_with_hp 의 hp 인자 | 4 | OK |  |
| 26 | EnemyRegionInfo(enemy_region[p]) | 0x0 | @tag | r | closure#4 :309 is_none_or | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 27 | EnemyRegionInfo(enemy_region[p]) | 0x8 | region | r | :312,:320 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 28 | EnemyRegionInfo(enemy_region[p]) | 0x10 | last_known | r | :319 | 4 | 오귀속(그 오프셋에 필드가 없다(패딩/관통불가)) |  |
| 29 | EpicStanceData | 0x30 | stance | w | ★출력. :269(non-Moba→0) :274(epic 없음→0) :369(2) :371(0) :375(0) :377(1) :380(0) | 4 | OK | 0 None / 1 Check / 2 CheckTry |
| 30 | EpicStanceData | 0x0 | last_epic_seen | w | :282 리스폰 감지 / :285 가시 | 4 | OK | tick() |
| 31 | EpicStanceData | 0x8 | last_epic_hp | w |  | 4 | OK | epic.stat_cached.hp(:281) / epic.hp(:286) |
| 32 | EpicStanceData | 0x20 | epic_ally_tick | w | :305 | 4 | OK | check_epic_kill_time_with_hp(ctx, ally_in_epic 엔티티, epic, last_epic_hp) |
| 33 | EpicStanceData | 0x28 | epic_ally_killed_tick | w | :306 | 4 | OK | check_epic_killed_time(epic, ally_in_epic) |
| 34 | EpicStanceData | 0x10 | epic_enemy_tick | w | :330 | 4 | OK | check_epic_kill_time_with_hp(ctx, enemy_in_epic 엔티티, epic, last_epic_hp) |
| 35 | EpicStanceData | 0x18 | epic_enemy_killed_tick | w | :332 | 4 | OK | check_epic_killed_time(epic, enemy_in_epic) |

**`consts` 상수 15건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 0 | 268 | 태그 | GameMode 태그 0=Moba (as_moba) · region_dist(region,7)==0 (closure#2 :298 / closure#4 :320 = '에픽 리전 안') | 4 |  |
| 1 | 7 | 298 | 미상 | 리전 id 7 = 에픽 리전(추정) — region_dist(·,7)·region_center(7) 의 b 인자 (aux + 본문 closure#6/#7) | 5 |  |
| 2 | 27 | 298 | 임계 | 리전 개수 bounds | 4 |  |
| 3 | 4 | 342 | 임계 | region_dist[region][7] < 4 → '에픽 근처'(closure#6 아군 · closure#7 적) | 4 |  |
| 4 | 49 | 343 | 임계 | 근처 아군 hp% > 49 (closure#6만) | 4 |  |
| 5 | 100 | 343 | 계수 | hp 백분율 계수 | 4 |  |
| 6 | 1000 | 568 | 계수 | check_epic_killed_time 정밀도(dpt=dmg*1000/cooltime, ticks=hp*1000/max(dpt,1)) | 4 |  |
| 7 | 5 | 356 | 계수 | tps*5 — epic_ally_tick <= 5초 조건 · Range 0..5 (:289) | 4 |  |
| 8 | 3 | 356 | 계수 | tps*3 (`mul %tps, 3`) — :356/:357 epic_ally_tick+3s <= epic_ally_killed_tick · :363 unseen>=3s · :370 epic_enemy_tick−3s. (`shl nuw nsw %len, 3` 은 &Entity 배열 stride 로 별개) | 4 |  |
| 9 | 1 | 353 | 인덱스 | tps*2 — `shl i64 %tps, 1` 로 접힘. live_ally_tick + tps*2 <= live_ally_killed_tick | 4 | 2 |
| 10 | 20 | 362 | 계수 | tps*20 — (epic_enemy_tick − unseen) < 20초 | 4 |  |
| 11 | 10 | 364 | 계수 | tps*10 — (epic_enemy_tick − unseen) < 10초 | 4 |  |
| 12 | 2 | 374 | 임계 | live_enemy.len < live_ally.len + 2 (적 ≤ 아군+1) 이어야 Check — 세르펜판엔 없고 에픽판에만 있는 최종 인원 게이트 | 4 |  |
| 13 | 32000 | 297 | 계수 | 셀 크기 | 4 |  |
| 14 | 29 | 297 | 임계 | 셀 clamp 상한 | 4 |  |

**`knobs` 조정점 9건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 근처 판정 리전 거리 | goal_data.rs:342,348 | 4 | 올리면 ally_near/enemy_near 가 넓게 잡힘 → 인원 우세 조건이 쉬워져 CheckTry 증가 | 4 | 기존 |
| 1 | 근처 아군 HP 하한(%) | goal_data.rs:343 | 49 | 내리면 저체력도 인원으로 셈 | 4 | 기존 |
| 2 | 아군 선처치 여유(초) | goal_data.rs:353 | 2 | 올리면 전원(live_ally) 기준 처치가 훨씬 빨라야 ready | 4 | 기존 |
| 3 | in_epic 아군 처치시간 상한(초) | goal_data.rs:356 | 5 | 올리면 리전 안 아군만으로 느리게 잡아도 ready | 4 | 기존 |
| 4 | in_epic 생존 여유(초) | goal_data.rs:356~357 | 3 | 올리면 더 큰 여유 필요 | 4 | 기존 |
| 5 | 적 처치 임박 창(초) | goal_data.rs:362,364 | 20 / 10 | 올리면 더 이른 시점부터 Check(견제) | 4 | 기존 |
| 6 | 미관측 최소(초) | goal_data.rs:363 | 3 | 내리면 잠깐 안 보여도 Check 쪽으로 | 4 | 기존 |
| 7 | 적 처치 성립 여유(초) | goal_data.rs:370 | 3 | 올리면 적이 에픽에게 죽기 전 더 넉넉히 잡아야 위협으로 인정 | 4 | 기존 |
| 8 | Check 인원 열세 한계 | goal_data.rs:374 | 2 | 올리면 인원 열세여도 Check(견제) 유지; 1로 내리면 동수 이상일 때만 | 4 | 기존 |

<details><summary>`callees` 피호출자 23건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | check_epic_kill_time_with_hp | game_ai::goal_data::check_epic_kill_time_with_hp | in:game_ai::goal_data | fn(&game_core::OperationData, &bumpalo::collections::vec::Vec< &game_core::Entity>, &game_core::Entity, usize) -> usize | game-ai\src\goal_data.rs:535 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | check_epic_killed_time | game_ai::goal_data::check_epic_killed_time | in:game_ai::goal_data | fn(&game_core::OperationData, &bumpalo::collections::vec::Vec< &game_core::Entity>, &game_core::Entity) -> usize | game-ai\src\goal_data.rs:565 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 5 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 6 | distance | game_core::Projectile::distance | pub | fn(&game_core::Projectile, u64, u64) -> u64 | game-core\src\simulation\projectile.rs:1008 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 7 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | region_center | game_core::MapDef::region_center | pub | fn(&game_core::MapDef, usize) -> (u64, u64) | game-core\src\simulation\map_def.rs:261 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | region_dist | game_core::MapDef::region_dist | pub | fn(&game_core::MapDef, usize, usize) -> usize | game-core\src\simulation\map_def.rs:257 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | update_plan | game_ai::EpicStanceData::update_plan | pub | fn(&mut game_ai::EpicStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:266 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 21 | update_plan | game_ai::AgentVerHamster::update_plan | in:game_ai | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> | game-ai\src\lib.rs:682 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 22 | update_plan | game_ai::SerpenStanceData::update_plan | pub | fn(&mut game_ai::SerpenStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:399 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 3개**: `epic`, `is_none_or`, `move_speed`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:4711) · **형제 5개** (EpicStanceData)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::EpicStanceData as std::clone::Clone>::clone | pub | game-ai\src\goal_data.rs:145 | True | fn(&game_ai::EpicStanceData) -> game_ai::EpicStanceData |
| 1 | <game_ai::EpicStanceData as std::default::Default>::default | pub | game-ai\src\goal_data.rs:145 | True | fn() -> game_ai::EpicStanceData |
| 2 | <game_ai::EpicStanceData as std::fmt::Debug>::fmt | pub | game-ai\src\goal_data.rs:145 | True | fn(&game_ai::EpicStanceData, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::EpicStanceData::update | pub | game-ai\src\goal_data.rs:158 | False | fn(&mut game_ai::EpicStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) |
| 4 | game_ai::EpicStanceData::update_plan | pub | game-ai\src\goal_data.rs:266 | False | fn(&mut game_ai::EpicStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 리전 id 7 = 에픽 리전이라는 것은 region_center(7)/region_dist(·,7) 문맥 추정 — MapDef 값표 미확인(미탐색) | 5 |  |
| 1 | 미탐색 | closure 번호: 메인 exe aux 는 {closure#0/#1/#3} 3개인데 IR 별도 define 은 4개(s0_0~s3_0 = closure#2~#5, DWARF closure$2~$5). 메인 주소↔IR 스텁 대응 재확인 필요(closure#3(as_entity 아군)·#5(적)는 본문이 거의 같아 exe 에서 병합됐을 가능성) | 3 |  |
| 2 | 미탐색 | is_visible(team,id) team 인덱스 의미 — SerpenStance 명세 unknown 과 동일 | 4 |  |
| 3 | 미탐색 | check_epic_kill_time_with_hp 본문(0xde3d90) 은 다른 배치 — 계약만. 4번째 인자 @anon...138 미해독 | 4 |  |
| 4 | 미탐색 | ready 의 소스 표기: IR 은 ally_near<enemy_near 분기(%581)에서 :353 항을 평가하지 않는다. `(A && B) \|\| (C && D)` 로 재구성했으며 외연 동일(표기 불가 아님, 분기 순서만 컴파일러 재배치) | 4 |  |
| 5 | 미탐색 | m01.ll 54172~54315·54318~54461 (bumpalo from_iter_in 인스턴스) 는 glue 라 aux 제외 | 4 |  |
| 6 | 미탐색 | fnparts 가 보인 `EpicStanceData::<6자 메서드>`(m09.ll 42820~44624, 클로저 66469~66790) = `EpicStanceData::update`(m09.ll:42820 define 확인) 별개 함수 — 이 배치 범위 밖, 미탐색(호출자·update_plan 과의 관계 불명) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

