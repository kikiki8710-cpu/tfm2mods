---

### `75` SerpenStanceData::update_plan — 세르펜 태세(None/Check/CheckTry) 결정: 세르펜 존재·시야로 hp 갱신 → 아군/적 처치시간 6종 산출 → 준비되면 CheckTry, 적이 먹을 낌새면 Check

| 항목 | 값 |
|---|---|
| id | `goal_data__SerpenStanceData_update_plan` |
| 심볼 | `_RNvMs0_NtCshdEBA0ozCnw_7game_ai9goal_dataNtB5_16SerpenStanceData11update_plan` |
| 소스 | `game-ai\src\goal_data.rs:399` |
| IR | `m09.ll` 22178~24407행 |
| 경로·가시성 | `game_ai::SerpenStanceData::update_plan` · **pub** |
| 계층 | 기타 |
| exe | `dd73b0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SerpenStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData)
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | self | &mut SerpenStanceData(56B) = GoalData+0xb0 | last_epic_seen@0 last_epic_hp@8 epic_enemy_tick@0x10 epic_enemy_killed_tick@0x18 epic_ally_tick@0x20 epic_ally_killed_tick@0x28 stance@0x30(EpicStance 1B) | 4 |
| 1 | 1 | (version) | usize | 호출자가 poison 으로 넘김 — 본문 미사용 | 4 |
| 2 | 2 | (rnd) | ptr | readnone — 미사용 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | team@0x930 position@0x9c0 | 4 |
| 4 | 4 | data | &OperationData(24B) | cache@0 context@8 blackboard@0x10 | 4 |
| 5 | 5 | enemy_region | &[Option<EnemyRegionInfo>;5] (= &GoalData, 오프셋 0) | stride 24: tag@0 region@8 last_known@16 — closure#4 에서만 읽음 | 4 |
| 6 | 6 | debug | &mut DebugFrameData(224B) | ctx.debug 일 때 +0xa0 HashMap 에 3줄 push (:481~489) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn update_plan(&mut self, _v, _rnd, player, data, enemy_region, debug):
  moba = cache.game.get_game_mode(); if tag!=0(Moba) { self.stance=None; return }        // :401~403
  serpen_state = &moba.jungle_runner.serpen                                              // :404 (+0x1c8)
  serpen = serpen_state.live_list.get(0).and_then(|id| game.get_entity_by_id(*id))       // :405
  if serpen==None { self.stance=None; return }                                            // :406~407
  if self.last_epic_seen < serpen_state.next_respawn_tick { self.last_epic_hp = serpen.stat_cached.hp; self.last_epic_seen = tick() }   // :413~415 리스폰 후 첫 관측: 풀피 가정
  if is_visible(player.team, serpen.id) { self.last_epic_seen = tick(); self.last_epic_hp = serpen.hp }   // :417~419
  ally_in_epic: Vec<usize> = (0..5).filter(|p| player_champion[team][p].is_some_and(|c| c.distance_sq(serpen) < 150000²))   // :422~431 closure#2
  as_entity = ally_in_epic.filter_map(|p| player_champion[team][p])                         // :433 closure#3
  self.epic_ally_tick        = check_epic_kill_time_with_hp(ctx, as_entity, serpen, self.last_epic_hp)   // :437
  self.epic_ally_killed_tick = check_epic_killed_time(serpen, as_entity)                     // :438
  enemy_in_epic: Vec<usize> = (0..5).filter(|p| enemy_region[p].is_none_or(|e| {           // :440~456 closure#4 — 위치 모르면(None) 포함
        echamp = player_champion[1-team][p]?  (None→false)                                   // :442~443
        dist = distance(region_center(e.region), region_center(2))                           // :444~447
        move_tick = dist / echamp.stat_cached.move_speed  (speed 0 → div0 패닉)               // :448~449
        if e.last_known + move_tick <= tick() { true }                                        // :451 이미 도달 가능 시간
        else { echamp.distance_sq(serpen) < 150000² } }))                                     // :452
  as_entity = enemy_in_epic.filter_map(|p| player_champion[1-team][p])                       // :458 closure#5
  self.epic_enemy_tick        = check_epic_kill_time_with_hp(ctx, as_entity, serpen, last_epic_hp)   // :462
  self.epic_enemy_killed_tick = check_epic_killed_time(serpen, as_entity)                    // :464
  targets = if is_line_phase(ctx, tick()) { 1..5 } else { 0..5 }                            // :467  is_line_phase = !spawn_epic(tutorial∉{0,5,7,8}) || tick < first_spawn_tick.saturating_sub(tps*30)
  live_ally  = targets.filter_map(|p| player_champion[team][p])                              // :473 closure#6
  live_enemy = targets.filter_map(|p| player_champion[1-team][p])                            // :475 closure#7
  live_ally_tick        = check_epic_kill_time_with_hp(ctx, live_ally, serpen, last_epic_hp)  // :478 (지역변수)
  live_ally_killed_tick = check_epic_killed_time(serpen, live_ally)                          // :479
  if ctx.debug && player_champion[team][my_pos].is_some() { debug 3줄 push }                  // :481~489
  ally_near  = iter_champions(team).filter(|c| region_dist(regions[c.cell], 2) < 4 && c.hp*100/c.max_hp > 49).count()   // :492~495 closure#8 (max_hp 0 → div0 패닉)
  enemy_near = iter_champions(1-team).filter(|c| region_dist(regions[c.cell], 2) < 4).count()                            // :498~501 closure#9
  unseen = tick().saturating_sub(self.last_epic_seen)                                        // :503
  ally_ok  = ally_near >= enemy_near && live_ally_tick + tps*2 <= live_ally_killed_tick       // :504~505 (단락: ally_near<enemy_near → false)
  epic_ok  = self.epic_ally_tick <= tps*5 && self.epic_ally_tick + tps*3 <= self.epic_ally_killed_tick   // :508~509
  gate     = if live_ally.len <= live_enemy.len { bb[team].mid.minion_count > 2 || bb[team].bottom.minion_count > 2 } else { true }   // :511~512
  ready    = gate && (ally_ok || epic_ok)                                                    // :514
  remain   = self.epic_enemy_tick.saturating_sub(unseen)                                     // :516
  a = remain < tps*20 ; b = unseen >= tps*3 ; c = remain < tps*10                             // :516~518
  self.stance = if ready { CheckTry(2) }                                                     // :522
     else if self.epic_enemy_killed_tick >= self.epic_enemy_tick.saturating_sub(tps*3) && ((a && b) || c) { Check(1) }   // :524 (적이 죽기 전에 잡을 수 있고, 곧 끝날 낌새)
     else { None(0) }                                                                        // :526
  // :531~532 (bumpalo Vec drop) return
```

**`mem` 메모리 접근 48건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r |  | 4 | OK |  |
| 1 | OperationData | 0x8 | context | r |  | 4 | OK |  |
| 2 | OperationData | 0x10 | blackboard(&[Blackboard;2]) | r | :511~512 bb[player.team] | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x0 | game.data_ptr(&dyn AbstractGame) | r | vtable +0x28 tick · +0x40 get_game_mode · +0xf8 is_visible · +0x1f0 get_entity_by_id | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | iter_champions(team) = 이 5칸 슬라이스(simulation.rs:1905) · 클로저 전부 | 4 | OK |  |
| 5 | MobaMode | 0x1c8 | jungle_runner.serpen (serpen_state) | r | GameMode::Moba 페이로드(+8) 가 &MobaMode. live_list.ptr@0x1d0 len@0x1d8 | 4 | OK |  |
| 6 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r | :405 live_list.get(0) → id | 4 | OK |  |
| 7 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | 0 이면 serpen None | 4 | OK |  |
| 8 | MobaMode | 0x1e0 | jungle_runner.serpen.next_respawn_tick | r | :413 last_epic_seen < next_respawn_tick → 리스폰 후 첫 관측 | 4 | OK |  |
| 9 | PlayerState | 0x930 | info.team | r |  | 4 | OK |  |
| 10 | PlayerState | 0x9c0 | info.position@tag | r | :482 디버그 키용 | 4 | OK |  |
| 11 | Entity | 0x5c0 | id | r | serpen.id → is_visible / 내 champ id → 디버그 키 | 4 | OK |  |
| 12 | Entity | 0x628 | stat_cached.hp | r | serpen 최대 HP(:414) · 챔프 최대 HP(closure#8 hp%) | 4 | OK |  |
| 13 | Entity | 0x670 | hp | r | serpen 현재 HP(:419) · check_epic_killed_time 의 champ.hp(:571) · closure#8 | 4 | OK |  |
| 14 | Entity | 0x660 | x | r | distance_sq / 셀 변환 | 4 | OK |  |
| 15 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 16 | Entity | 0x640 | stat_cached.move_speed | r | closure#4 내부 :448 (dist/speed = move_tick) | 4 | OK |  |
| 17 | Entity | 0x490 | attack_effect@Some.0 (Effect) | r | check_epic_killed_time :568 serpen.attack_effect.as_ref().unwrap() | 4 | OK |  |
| 18 | Entity | 0x4c0 | attack_effect@tag(i32, -1=None) | r | None 이면 unwrap_failed 패닉 | 4 | OK |  |
| 19 | GameContext | 0x8 | setting | r |  | 4 | OK |  |
| 20 | GameContext | 0x20 | map | r |  | 4 | OK |  |
| 21 | GameContext | 0x38 | tutorial(TutorialType) | r | spawn_epic(runner.rs:263) = tag ∈ {0 None,5 MidBottom,7 Line,8 Total} | 4 | OK |  |
| 22 | GameContext | 0x3b | debug | r |  | 4 | OK |  |
| 23 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | setting.is_line_phase(tick) (setting.rs:703~704): tick < first_spawn_tick.saturating_sub(tps*30) | 4 | OK |  |
| 24 | GameSetting | 0x12f8 | tick_per_second | r | tps — 모든 초 단위 임계의 배수 | 4 | OK |  |
| 25 | MapDef | 0x38b8 | regions[cy][cx] | r | closure#8/#9 :493,:499 | 4 | OK |  |
| 26 | MapDef | 0x54e8 | region_dist[a][2] (=region_dist@0x54d8 + 2*8) | r | map_def.rs:258 region_dist(a, b=2) → gep +21736(0x54e8) 후 a*27*8. 리전 2 = 세르펜 리전(추정) | 5 | OK |  |
| 27 | MapDef | 0x6ba0 | region_centers[region] / region_centers[2] | r | closure#4 내부 :444,:446 (+27552 = region_centers[e.region], +27584/+27592 = region_centers[2].x/.y) | 4 | OK |  |
| 28 | Blackboard | 0x48 | mid_minion_state.minion_count(i32) | r | :511 bb[team] (Blackboard 744B stride) | 4 | OK |  |
| 29 | Blackboard | 0x70 | bottom_minion_state.minion_count(i32) | r | :512 | 4 | OK |  |
| 30 | SerpenStanceData | 0x0 | last_epic_seen | r | :413,:503 | 4 | OK |  |
| 31 | SerpenStanceData | 0x8 | last_epic_hp | r | check_epic_kill_time_with_hp 의 hp 인자(:437,:462,:478) | 4 | OK |  |
| 32 | SerpenStanceData | 0x10 | epic_enemy_tick | r | :516 (읽기: 방금 쓴 값) | 4 | OK |  |
| 33 | SerpenStanceData | 0x18 | epic_enemy_killed_tick | r | :524 | 4 | OK |  |
| 34 | SerpenStanceData | 0x20 | epic_ally_tick | r | :508~509 | 4 | OK |  |
| 35 | SerpenStanceData | 0x28 | epic_ally_killed_tick | r | :509 | 4 | OK |  |
| 36 | SerpenStanceData | 0x30 | stance | r | :488 디버그 출력(갱신 전 값) | 4 | OK |  |
| 37 | EnemyRegionInfo(enemy_region[p]) | 0x0 | @tag(0=None) | r | closure#4 :441 is_none_or | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 38 | EnemyRegionInfo(enemy_region[p]) | 0x8 | region | r | :444 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 39 | EnemyRegionInfo(enemy_region[p]) | 0x10 | last_known | r | :451 | 4 | 오귀속(그 오프셋에 필드가 없다(패딩/관통불가)) |  |
| 40 | SerpenStanceData | 0x30 | stance | w | ★출력. :402(non-Moba→0) :407(serpen 없음→0) :522~526 결정(값 2/1/0). 정상 경로에선 항상 덮어씀 | 4 | OK | 0 None / 1 Check / 2 CheckTry |
| 41 | SerpenStanceData | 0x0 | last_epic_seen | w | :415 리스폰 감지 시 / :418 가시 시 | 4 | OK | tick() |
| 42 | SerpenStanceData | 0x8 | last_epic_hp | w |  | 4 | OK | serpen.stat_cached.hp(:414 리스폰) / serpen.hp(:419 가시) |
| 43 | SerpenStanceData | 0x20 | epic_ally_tick | w | :437 — 150k 안 아군이 세르펜(last_epic_hp)을 잡는 틱 | 4 | OK | check_epic_kill_time_with_hp(ctx, ally_in_epic 엔티티, serpen, last_epic_hp) |
| 44 | SerpenStanceData | 0x28 | epic_ally_killed_tick | w | :438 — 세르펜이 그 아군 전원을 잡는 틱 합 | 4 | OK | check_epic_killed_time(serpen, ally_in_epic) |
| 45 | SerpenStanceData | 0x10 | epic_enemy_tick | w | :462 | 4 | OK | check_epic_kill_time_with_hp(ctx, enemy_in_epic 엔티티, serpen, last_epic_hp) |
| 46 | SerpenStanceData | 0x18 | epic_enemy_killed_tick | w | :464 | 4 | OK | check_epic_killed_time(serpen, enemy_in_epic) |
| 47 | DebugFrameData | 0xa0 | HashMap[champ.id] Vec<String> | w | :484,:486,:488 — ctx.debug && 내 champ Some 일 때 | 4 | 오귀속(사전은 다른 필드를 준다) | "enemy in serpen: {n}, last_serpen_seen: {}, last_serpen_hp: {}" · "live_ally_tick: {}, live_ally_killed_tick: {}" · "now_serpen_stance: {구 stance}" |

**`consts` 상수 22건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 0 | 401 | 태그 | GameMode 태그 0 = Moba (as_moba, game.rs:231). 아니면 stance=None 후 return | 4 |  |
| 1 | 22500000001 | 430 | 미상 | 150000² + 1 — 아군(closure#2 :430)·적(closure#4 내부 :452) 이 세르펜 150k(≈4.7셀) 안이면 in_epic (aux) | 4 |  |
| 2 | 1 | 441 | 인덱스 | enemy_region[p] tag 1=Some (trunc to i1) — is_none_or 분기 (aux). shl 피연산자 아님 | 4 |  |
| 3 | 2 | 446 | 임계 | 리전 id 2 = 세르펜 리전(추정) — region_center(2)(closure#4 :446) / region_dist(·,2)(closure#8/#9 :494,:500) | 5 |  |
| 4 | 27 | 494 | 임계 | 리전 개수 — region_dist/region_centers 인덱스 bounds | 4 |  |
| 5 | 4 | 494 | 임계 | region_dist[region][2] < 4 이면 '세르펜 근처'(closure#8 아군 · closure#9 적) | 4 |  |
| 6 | 49 | 495 | 임계 | 아군 hp% > 49 (즉 ≥50%) 여야 근처 아군으로 셈 (closure#8만; 적은 hp 조건 없음) | 4 |  |
| 7 | 100 | 495 | 계수 | hp 백분율 계수 | 4 |  |
| 8 | 1000 | 568 | 계수 | check_epic_killed_time 정밀도: dpt = dmg*1000/cooltime, ticks = hp*1000/max(dpt,1) | 4 |  |
| 9 | 30 | 467 | 계수 | is_line_phase: tick < first_spawn_tick − tps*30 (에픽 첫 스폰 30초 전까지 라인 페이즈) — setting.rs:704 인라인 | 4 |  |
| 10 | 5 | 467 | 태그 | Range end(포지션 5칸) · tps*5(:508 epic_ally_tick 상한 5초) | 4 |  |
| 11 | 1 | 467 | 인덱스 | 라인 페이즈면 targets = 1..5 (Top(0) 제외) — `phi [1,%236],[0,%226]`. shl 피연산자 아님 | 4 |  |
| 12 | 7 | 467 | 태그 | TutorialType 태그 {0 None,5 MidBottom,7 Line,8 Total} 이면 spawn_epic=true (runner.rs:263 switch) | 4 |  |
| 13 | 8 | 467 | 태그 | TutorialType::Total (위 switch) | 4 |  |
| 14 | 1 | 505 | 인덱스 | tps*2 — `shl i64 %tps, 1` 로 접힘. live_ally_tick + tps*2 <= live_ally_killed_tick (2초 여유로 아군이 먼저 잡음) | 4 | 2 |
| 15 | 3 | 509 | 계수 | tps*3 (`mul %tps, 3`) — :509 epic_ally_tick+3s <= epic_ally_killed_tick · :517 unseen >= 3s · :524 epic_enemy_tick−3s. (`shl nuw nsw %len, 3` 은 &Entity 배열 stride ×8 로 별개) | 4 |  |
| 16 | 2 | 511 | 임계 | bb[team].mid/bottom minion_count > 2 (미니언 웨이브 밀려있음) — 라인 게이트 | 4 |  |
| 17 | 1 | 511 | 인덱스 | live_ally.len < live_enemy.len + 1 (= 아군 ≤ 적) 이면 미니언 게이트 적용. shl 피연산자 아님 | 4 |  |
| 18 | 20 | 516 | 계수 | tps*20 — (epic_enemy_tick − unseen) < 20초 | 4 |  |
| 19 | 10 | 518 | 계수 | tps*10 — (epic_enemy_tick − unseen) < 10초 | 4 |  |
| 20 | 32000 | 493 | 계수 | 셀 크기(좌표→셀) | 4 |  |
| 21 | 29 | 493 | 임계 | 셀 clamp 상한 | 4 |  |

**`knobs` 조정점 11건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | in_epic 거리(세르펜 150k) | goal_data.rs:430,452 (aux closure#2/#4) | 22500000001 | 올리면 더 먼 아군/적도 '세르펜 교전 인원'으로 계산돼 epic_*_tick 이 짧아진다(더 공격적 CheckTry/Check) | 4 | 기존 |
| 1 | 근처 판정 리전 거리 | goal_data.rs:494,500 | 4 | 올리면 ally_near/enemy_near 가 넓게 잡혀 인원 비교(ally_ok) 성립이 쉬워진다 | 4 | 기존 |
| 2 | 근처 아군 HP 하한(%) | goal_data.rs:495 | 49 | 내리면 저체력 아군도 인원으로 세어 ally_ok 가 잘 성립 | 4 | 기존 |
| 3 | 아군 선처치 여유(초) | goal_data.rs:505 | 2 | 올리면 live_ally 가 세르펜을 훨씬 빨리 잡을 수 있어야 ally_ok | 4 | 기존 |
| 4 | in_epic 아군 처치시간 상한(초) | goal_data.rs:508 | 5 | 올리면 느린 처치도 epic_ok 인정 → CheckTry 증가 | 4 | 기존 |
| 5 | in_epic 아군 생존 여유(초) | goal_data.rs:509 | 3 | 올리면 더 큰 여유 필요 → epic_ok 감소 | 4 | 기존 |
| 6 | 미니언 게이트 수 | goal_data.rs:511~512 | 2 | 아군≤적 일 때 미드/바텀 미니언이 이 값 초과여야 ready. 올리면 인원 열세 시 CheckTry 억제 | 4 | 기존 |
| 7 | 적 처치 임박 창(초) | goal_data.rs:516,518 | 20 / 10 | 올리면 적이 세르펜을 먹기까지 남은 시간이 더 길어도 Check 발동(일찍 견제) | 4 | 기존 |
| 8 | 미관측 최소(초) | goal_data.rs:517 | 3 | 내리면 잠깐만 안 보여도 (remain<20s 조건과 함께) Check | 4 | 기존 |
| 9 | 적 처치 성립 여유(초) | goal_data.rs:524 | 3 | 올리면 적이 세르펜에게 죽기 전에 더 넉넉히 잡아야 '적이 먹을 낌새'로 인정 | 4 | 기존 |
| 10 | 라인 페이즈 경계(초) | setting.rs:704 (goal_data.rs:467 인라인) | 30 | 에픽 첫 스폰 N초 전까지 Top 을 live 집계에서 제외 | 4 | 기존 |

<details><summary>`callees` 피호출자 30건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check_epic_kill_time_with_hp | game_ai::goal_data::check_epic_kill_time_with_hp | in:game_ai::goal_data | fn(&game_core::OperationData, &bumpalo::collections::vec::Vec< &game_core::Entity>, &game_core::Entity, usize) -> usize | game-ai\src\goal_data.rs:535 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | check_epic_killed_time | game_ai::goal_data::check_epic_killed_time | in:game_ai::goal_data | fn(&game_core::OperationData, &bumpalo::collections::vec::Vec< &game_core::Entity>, &game_core::Entity) -> usize | game-ai\src\goal_data.rs:565 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 4 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 5 | distance | game_core::Projectile::distance | pub | fn(&game_core::Projectile, u64, u64) -> u64 | game-core\src\simulation\projectile.rs:1008 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 6 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | region_center | game_core::MapDef::region_center | pub | fn(&game_core::MapDef, usize) -> (u64, u64) | game-core\src\simulation\map_def.rs:261 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 22 | region_dist | game_core::MapDef::region_dist | pub | fn(&game_core::MapDef, usize, usize) -> usize | game-core\src\simulation\map_def.rs:257 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 23 | spawn_epic | game_core::TutorialType::spawn_epic | pub | fn(&game_core::TutorialType) -> bool | game-core\src\simulation\game\runner.rs:262 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 24 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 25 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 26 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 27 | update_plan | game_ai::EpicStanceData::update_plan | pub | fn(&mut game_ai::EpicStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:266 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | update_plan | game_ai::AgentVerHamster::update_plan | in:game_ai | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> | game-ai\src\lib.rs:682 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 29 | update_plan | game_ai::SerpenStanceData::update_plan | pub | fn(&mut game_ai::SerpenStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:399 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 5개**: `format_inner`, `insert_no_grow`, `is_none_or`, `move_speed`, `rustc_entry`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:4713) · **형제 4개** (SerpenStanceData)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SerpenStanceData as std::clone::Clone>::clone | pub | game-ai\src\goal_data.rs:386 | True | fn(&game_ai::SerpenStanceData) -> game_ai::SerpenStanceData |
| 1 | <game_ai::SerpenStanceData as std::default::Default>::default | pub | game-ai\src\goal_data.rs:386 | True | fn() -> game_ai::SerpenStanceData |
| 2 | <game_ai::SerpenStanceData as std::fmt::Debug>::fmt | pub | game-ai\src\goal_data.rs:386 | True | fn(&game_ai::SerpenStanceData, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::SerpenStanceData::update_plan | pub | game-ai\src\goal_data.rs:399 | False | fn(&mut game_ai::SerpenStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) |

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 리전 id 2 가 세르펜 리전이라는 것은 region_center(2)·region_dist(·,2) 사용 문맥에서의 추정 — MapDef.regions 값표 미확인(미탐색: map 데이터 덤프) | 5 |  |
| 1 | 미탐색 | EpicStance 태그 의미(0 None/1 Check/2 CheckTry)는 tcxdict --enum EpicStance 로 확정. 그러나 Check vs CheckTry 의 소비처(플랜 선택) 는 이 함수 밖(미탐색) | 3 |  |
| 2 | 미탐색 | closure 번호: 메인이 준 exe aux 목록은 {closure#0/#1/#3/#4/#5} 5개인데 IR 의 별도 define 은 6개(v0 망글 s0_0~s5_0 = closure#2~#7, DWARF closure$2~$7). closure#0/#1 은 인라인(as_moba/and_then). 메인 exe 주소↔IR 스텁 대응은 재확인 필요 | 3 |  |
| 3 | 미탐색 | is_visible(player.team, serpen.id) 의 team 인덱스 의미 — GoalData::update 는 (적 팀, 적 id) 로 부르고 여기선 (내 팀, 세르펜 id). World::is_visible = entity.visible_state[team]==Visible 까지만 확인(g07.ll:171567~171640) | 4 |  |
| 4 | 미탐색 | check_epic_kill_time_with_hp 본문은 담당 밖(다른 배치) — 호출 계약만 기재. 4번째 인자 @anon...138 (16B 상수 {0x6c0, 8}) 은 expected_damage_target 의 옵션 구조체로 보이나 내용 미해독 | 4 |  |
| 5 | 표기 불가 | is_line_phase 의 극성 표기: IR phi([1,%236 default/true],[0,%226 fallthrough]) 로 '라인 페이즈면 1..5' 는 확정. 소스가 `if is_line_phase {1..5} else {0..5}` 인지 부정형인지는 표기 불가(동작 동일) | 4 |  |
| 6 | 미탐색 | :516 remain = epic_enemy_tick.saturating_sub(unseen) 의 설계 의도(적이 마지막 관측 시점부터 계속 때렸다고 가정한 잔여 시간?) 는 주석 부재로 추정 | 5 |  |
| 7 | 미탐색 | m01.ll 53880~54169 (bumpalo from_iter_in 인스턴스 2개) 는 이터레이터 glue 라 aux 에서 제외 — 판정 상수 없음(call_mut 호출만) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

