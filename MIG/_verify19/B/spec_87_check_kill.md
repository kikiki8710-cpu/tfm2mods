---

### `87` check_kill — 150000 안 적 중 「내가 (아군 화력으로) 먼저 죽이고, 그 적이 타워로 도망치기 전에 잡을 수 있는」 첫 적을 골라 (적 id, 예상 처치 틱)을 돌려준다

| 항목 | 값 |
|---|---|
| id | `handler__check_kill` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler10check_kill` |
| 소스 | `game-ai\src\plan_legacy\handler.rs:2507` |
| IR | `m13.ll` 54538~56187행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::check_kill` · **pub** |
| 계층 | 플랜 핸들러 |
| exe | `e6b800` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<(usize, usize)>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<(usize, usize)> (24B) | +0 태그(0=None/1=Some), +8 = 적 Entity.id, +16 = max(적 사망예상틱, 60) + 현재 tick | 4 |
| 1 | 1 | version | usize | 본문 분기 없음. 적 필터 클로저(aux) 의 is_ignored_well_enemy 에 전달 | 4 |
| 2 | 2 | rnd | &mut StdRng | readnone — 전혀 안 씀 | 4 |
| 3 | 3 | player | &PlayerState (2528B) | +0x930 team, +0x9c0 position, +0x180 parameter(aggressive_ratio). player_awareness_lapse·is_ignored_well_enemy 에 전달 | 4 |
| 4 | 4 | data | &OperationData (24B) | +0x0 cache, +0x8 context | 4 |
| 5 | 5 | _positioning_score | &PositioningScoreData | readonly·미사용(이름부터 _) | 4 |
| 6 | 6 | debug | &mut DebugFrameData (224B) | ctx.debug 일 때만 +0xa0 infos[me.id] 에 문자열 push (L2473~2474) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
check_kill(version, rnd, player, data, _ps, debug):                 // L2507
  if player_awareness_lapse(player, data) { return None }            // L2509~2510
  return check_kill_v15(version, player, data, debug)                // L2512 (2373~2505 전부 인라인)

--- check_kill_v15 ---
me   = cache.player_champion[team][my_pos].unwrap()                            // L2374
allies  = iter_champions(team).filter(|a| dist_sq(a,me) <= 150000² && a.hp*100/a.max_hp > 39)   // L2377 (aux#0) ★나 자신 포함
enemies = iter_champions(1-team).filter(|e| dist_sq(e,me) <= 150000²
            && (me.team==Neutral || e.visible_state[my_team]==Visible)
            && !is_ignored_well_enemy(version, player, e))                       // L2379 (aux#1)
disable_tick = match player_count(tutorial) { 5v5→setting.tower_attack_disable_tick, 2v2→_2v2, 3v3→_3v3 }   // L2383~2386
towers = iter_towers_without_nexus(cache, 1-team).filter(|t| game.tick() <= disable_tick)   // L2389 (aux#2) — 적 타워

// 1) 적 전원이 나에게 주는 화력
enemy_burst = 0; enemy_dps = 0
for e in enemies:                                                              // L2393
  pe = player_by_champion_id(e.id).unwrap(); cc = champion_cache[pe.team][pe.pos]   // L2395
  b = 0
  if e.can_attack() || e.attack_cooldown() is None || cd <= tps { b = cc.attack[my_pos] }         // L2398~2399
  if e.can_skill()  || e.ty!=Champion || e.skill_cooldown  <= tps { b = max(b, cc.skill[my_pos]) } // L2402~2403
  if e.can_skill2() || e.ty!=Champion || e.skill2_cooldown <= tps { b = max(b, cc.skill2[my_pos]) }// L2406~2407
  enemy_dps  += cc.attack_per_sec[my_pos] + cc.skill_per_sec[my_pos] + cc.skill2_per_sec[my_pos]  // L2410~2412
  enemy_burst += b                                                             // L2413

ar = player.parameter.aggressive_ratio()                                       // L2417
c1 = (1000-ar)*80/1000 + 80 ; c2 = ar*45/1000 + 45 ; c3 = ar*35/1000 + 15     // L2419~2421

// 2) 적 하나씩 — 아군 전원의 화력 vs 그 적
for e in enemies:                                                              // L2424
  pe = player_by_champion_id(e.id).unwrap(); epos = pe.pos                     // L2431~2432
  ally_burst = 0; ally_dps = 0
  for a in allies:                                                             // L2434
    pa = (team,pos) of a via player_champion 10칸 id 스캔 (없으면 unwrap 패닉)   // L2435
    cc = champion_cache[pa.team][pa.pos]                                       // L2440
    ally_dps += cc.attack_per_sec[epos] + cc.skill_per_sec[epos] + cc.skill2_per_sec[epos]   // L2440~2442
    b = 0 ; (공격/스킬/스킬2 가용 판정은 1)과 동일 규칙, 대상 열 = epos)            // L2444~2453
    ally_burst += b                                                            // L2456~2457
  my_burst = enemy_burst ; my_dps = enemy_dps
  for t in towers:                                                             // L2460
    if dist_sq(t, e) <= 150000² {                                              // L2461
      d = t.attack_effect.unwrap().expected_damage_target(ctx, t, <static>, me)   // L2462
      my_dps  += tps * d / t.attack_cooltime()   (0 이면 div-by-zero 패닉)        // L2463
      my_burst += d }                                                          // L2464
  enemy_die = sat(e.hp - ally_burst) * 60 / max(ally_dps, 1)                   // L2470
  my_die    = sat(me.hp - my_burst)  * 60 / max(my_dps, 1)                     // L2471
  if ctx.debug { debug.infos[me.id].push(format!(enemy_die, my_die)) }         // L2473~2474
  if enemy_die > sat(my_die - c1) { continue }                                 // L2477  ★내가 c1 틱 이상 먼저 죽이지 못하면 탈락
  tower_ticks = match towers.min_by_key(|t| dist_sq(t,e)) { Some(t) => e.distance(t) / e.move_speed, None => 99999999 }   // L2481~2484
  if me.move_speed > e.move_speed {                                            // L2489
    ok = enemy_die < sat(tower_ticks - c3)                                     // L2498~2499
  } else {
    ok = me.attack_effect.unwrap().is_in_range(me, e) && enemy_die < c2 && enemy_die < sat(tower_ticks - c3)   // L2491~2494
  }
  if ok { return Some((e.id, max(enemy_die, 60) + game.tick())) }              // L2502
return None                                                                    // L2505
```

**`mem` 메모리 접근 38건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 0..1. player_champion[team][pos] 인덱스(bounds<2, panic @2374). 1-team = 적 팀 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 Position → as_index = my_pos(캐시 열 인덱스) | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter | r | AthleteParameter(744B) → aggressive_ratio() (L2417) | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 5 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 6 | GameContext | 0x38 | tutorial | r | player_count(runner.rs:295) 인라인: {0,2,4,6,7,8}→5v5 / {1,3}→2v2 / {5}→3v3 설정 선택 | 4 | OK |  |
| 7 | GameContext | 0x3b | debug | r | bool. true 면 L2474 디버그 문자열 기록 | 4 | OK |  |
| 8 | GameSetting | 0x12f8 | tick_per_second | r | 쿨다운 임계(L2398·2402·2406·2444·2448·2452: cooldown > tps 면 그 공격수단 제외) · 타워 dps 환산(L2463: tps*dmg/attack_cooltime) | 4 | OK |  |
| 9 | GameSetting | 0x13f8 | tower_attack_disable_tick (5v5) / 0x1400 _2v2 / 0x1408 _3v3 | r | L2383 player_count 별 선택 → 타워 필터(aux closure#2): game.tick() <= 이 값인 타워만 포함 | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x0 | game.data_ptr / game.vtable_ptr | r | vtable+0x28 tick() (L2502·타워 필터) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion | r | [team][pos] Option<&Entity>. L2374 me = [team][my_pos].unwrap(). L2435 player_by_champion_id 인라인(10칸 id 스캔 → (team,pos)) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x280 | player_champion_cache[][] | r | [team][pos] ChampionCache(800B). 열 인덱스 = 대상 pos | 4 | OK |  |
| 13 | ChampionCache | 0x0 | attack[pos] | r | [대상pos] 1회 공격 피해. 적→나(L2399), 아군→적(L2445) | 4 | OK |  |
| 14 | ChampionCache | 0x28 | skill[pos] | r | L2403 / L2449 | 4 | OK |  |
| 15 | ChampionCache | 0x50 | skill2[pos] | r | L2407 / L2453 | 4 | OK |  |
| 16 | ChampionCache | 0x190 | attack_per_sec[pos] | r | L2410 / L2440 | 4 | OK |  |
| 17 | ChampionCache | 0x1b8 | skill_per_sec[pos] | r | L2411 / L2441 | 4 | OK |  |
| 18 | ChampionCache | 0x1e0 | skill2_per_sec[pos] | r | L2412 / L2442 | 4 | OK |  |
| 19 | Entity | 0x0 | team@tag | r | (aux 적 필터) me.team: Neutral(1)이면 가시성 검사 생략 | 4 | OK |  |
| 20 | Entity | 0x8 | team@Player.0 | r | (aux 적 필터) me 팀 인덱스 → e.visible_state[team] | 4 | OK |  |
| 21 | Entity | 0x38 | visible_state | r | (aux 적 필터) [my_team]@tag == 0 Visible 이어야 적 후보 | 4 | OK |  |
| 22 | Entity | 0x68 | ty@tag | r | EntityType. attack_cooldown(entity.rs:1748~1760) 인라인 매치: 13 Champion→+0xb0, 1 Minion→+0xb8, 2 Tower→+0x110, 4/7→+0xe8, 5/6→+0x1f0, 8→+0xb0, 9→+0xc8, 10→+0xf0, 11→+0xd8, 12→+0xd0, 0/3→쿨다운 없음. skill_cooldown(1775)/skill2_cooldown(1790)은 13 Champion 만 | 4 | OK |  |
| 23 | Entity | 0xb0 | ty@Champion.0.attack_cooldown | r | can_attack() false 일 때 > tps 면 공격 피해 제외 | 4 | OK |  |
| 24 | Entity | 0xb8 | ty@Champion.0.skill_cooldown | r | can_skill() false && Champion 일 때 > tps 면 skill 제외 | 4 | OK |  |
| 25 | Entity | 0xc0 | ty@Champion.0.skill2_cooldown | r | can_skill2() false && Champion 일 때 > tps 면 skill2 제외 | 4 | OK |  |
| 26 | Entity | 0x490 | attack_effect@Some.0 | r | Effect(56B). 타워: expected_damage_target(L2462) / 나: is_in_range(L2494) | 4 | OK |  |
| 27 | Entity | 0x4c0 | attack_effect@tag | r | i32 니치 -1 = None → unwrap panic(@2462 타워 / @2491 나) | 4 | OK |  |
| 28 | Entity | 0x5c0 | id | r | player_by_champion_id 키 · 반환값.0 · debug 키 | 4 | OK |  |
| 29 | Entity | 0x628 | stat_cached.hp | r | (aux 아군 필터) 최대 HP: hp*100/max_hp > 39 | 4 | OK |  |
| 30 | Entity | 0x640 | stat_cached.move_speed | r | L2484 e.distance(tower)/e.speed = 타워 도달 틱 · L2489 me.speed > e.speed 분기 | 4 | OK |  |
| 31 | Entity | 0x660 | x | r | distance_sq(utils.rs:7~9) 인라인 | 4 | OK |  |
| 32 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 33 | Entity | 0x670 | hp | r | L2470 적 잔여 HP, L2471 내 잔여 HP, (aux) 아군 필터 hp% | 4 | OK |  |
| 34 | Option<(usize,usize)>(sret) | 0x0 | tag | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | 0 (None: L2510 lapse / L2502 후보 없음) · 1 (Some) |
| 35 | Option<(usize,usize)>(sret) | 0x8 | Some.0 | w | L2502 | 4 | 확인불가(tcx 사전에 타입 없음) | e.id (Entity+0x5c0) |
| 36 | Option<(usize,usize)>(sret) | 0x10 | Some.1 | w | L2502 예상 처치 틱 | 4 | 확인불가(tcx 사전에 타입 없음) | max(enemy_die_ticks, 60) + game.tick() |
| 37 | DebugFrameData(debug) | 0xa0 | infos | w | L2473 ctx.debug 일 때만. 적 후보마다 1줄 | 4 | OK | HashMap<usize, Vec<String>>: entry(me.id).or_default().push(format!(.., enemy_die_ticks, my_die_ticks)) |

**`consts` 상수 14건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 22500000001 | 2461 | 임계 | 150000²+1 — 타워↔적 거리(제곱) 임계(L2461) · (aux) 아군↔나(L2377)·적↔나(L2379) 후보 반경 · L2481 최근접 타워 탐색 첫 원소. 셀 32000 기준 ≈4.7셀 | 4 |
| 1 | 100 | 2377 | 미상 | (aux closure#0) 아군 HP% = hp*100/max_hp | 4 |
| 2 | 39 | 2377 | 미상 | (aux closure#0) 아군 후보 = HP% > 39 (즉 40% 이상) | 4 |
| 3 | 1000 | 2419 | 계수 | aggressive_ratio(0..1000) 정규화 분모 | 4 |
| 4 | 80 | 2419 | 계수 | c1 = (1000-ar)*80/1000 + 80 ∈ [80,160] 틱 — 「내가 죽기까지」와 「적이 죽기까지」의 최소 여유(L2477). 공격성 높을수록 여유 요구가 작다 | 4 |
| 5 | 45 | 2420 | 계수 | c2 = ar*45/1000 + 45 ∈ [45,90] 틱 — 내가 더 느릴 때 적 사망예상틱 상한(L2494) | 4 |
| 6 | 35 | 2421 | 계수 | c3 = ar*35/1000 + 15 ∈ [15,50] 틱 — 적의 타워 도달 틱에서 빼는 여유(L2498·L2494) | 4 |
| 7 | 15 | 2421 | 미상 | c3 절편 | 4 |
| 8 | 60 | 2470 | 임계 | 잔여HP*60/초당피해 = 틱 환산(tps 60 리터럴 — setting.tick_per_second 가 아닌 하드코딩). L2502 max(적사망틱, 60) 하한도 같은 리터럴 | 4 |
| 9 | 1 | 2470 | 태그 | max(dps, 1) — 0 나눗셈 방지 | 4 |
| 10 | 99999999 | 2481 | 산출값 | 적 타워 없음 → 타워 도달 틱 = 무한대 대용 | 4 |
| 11 | 13 | 2402 | 태그 | EntityType::Champion 태그 — skill/skill2 쿨다운은 챔피언만 읽음(entity.rs:1775·1790) | 4 |
| 12 | -1 | 2462 | 센티널 | attack_effect Option 니치(i32) None → unwrap 실패 패닉(타워 L2462 · 나 L2491) | 4 |
| 13 | 2 | 2374 | 임계 | 팀 인덱스 bounds | 4 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 후보 반경(아군·적·타워) | handler.rs:2377·2379·2461 | 22500000001 | 올리면 더 먼 아군 화력을 합산하고 더 먼 적을 킬 후보로 본다(과대평가 위험). 내리면 근접 교전만 | 4 | 기존 |
| 1 | 아군 화력 참여 HP% 하한 | handler.rs:2377 | 39 | 내리면 빈사 아군도 화력에 포함 → 킬 판정이 후해진다 | 4 | 기존 |
| 2 | 선제 여유 c1 기본/기울기 | handler.rs:2419 | 80+80 | 올리면 「내가 훨씬 먼저 죽인다」가 확실할 때만 킬 시도(보수적). ar 높을수록 자동으로 작아짐 | 4 | 기존 |
| 3 | 느린 추격자의 적 사망틱 상한 c2 | handler.rs:2420 | 45+45 | 올리면 내가 더 느려도 오래 걸리는 킬을 시도 | 4 | 기존 |
| 4 | 타워 도달 여유 c3 | handler.rs:2421 | 15+35 | 올리면 타워 근처 적을 더 일찍 포기 | 4 | 기존 |
| 5 | 틱 환산 계수 | handler.rs:2470·2471·2502 | 60 | setting.tick_per_second 와 별개 리터럴 — tps 를 바꿔도 여기 안 따라감(불일치 지점) | 4 | 기존 |
| 6 | 쿨다운 허용 임계 | handler.rs:2398~2452 | setting.tick_per_second | 쿨다운이 1초 이내면 그 공격수단을 곧 쓸 수 있는 것으로 셈 | 4 | 기존 |

<details><summary>`callees` 피호출자 24건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | aggressive_ratio | game_core::AthleteParameter::aggressive_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:432 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_cooldown | game_core::Entity::attack_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1747 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | check_kill | game_ai::plan_legacy::handler::check_kill | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<(usize, usize)> | game-ai\src\plan_legacy\handler.rs:2507 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | check_kill_v15 | game_ai::plan_legacy::handler::check_kill_v15 | in:game_ai::plan_legacy::handler | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<(usize, usize)> | game-ai\src\plan_legacy\handler.rs:2373 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | player_awareness_lapse | game_ai::player_awareness_lapse | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\utils.rs:538 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 18 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 19 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 20 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 21 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 22 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 23 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 3개**: `dist_sq`, `format_inner`, `rustc_entry`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m08.ll:97474, m13.ll:16599, m13.ll:43887) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | expected_damage_target 의 4번째 인자 @anon...174(88B 정적) 의 내용 — 본문 밖 상수 데이터(미독) | 4 |  |
| 1 | 미탐색 | iter_towers_without_nexus 의 반환 순서·포함 범위(1차/2차 타워·쌍둥이 타워 여부) — game_core 본문 미독. min_by_key 는 첫 최소 원소(closure$3, m12.ll 30393~30533) | 4 |  |
| 2 | 미탐색 | player_awareness_lapse(utils) 내부 조건 — 별도 함수(호출만) | 4 |  |
| 3 | 미탐색 | attack_cooldown 이 None 인 ty(None/Nexus) 경로는 도달 불가로 보이나(iter_champions 만) IR 상 분기는 존재 — 기록만 | 4 |  |
| 4 | 표기 불가 | L2398 `can_attack \|\| cooldown<=tps` 의 소스 문장 형태(`if !can_attack && cd > tps { 0 } else {..}` 등) — column 부재로 표기 불가, 진리표는 확정 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | 타워 필터 `game.tick() <= tower_attack_disable_tick` 의 게임 의미(왜 이 틱 이전에만 타워를 세는지) — 극성만 IR 로 확정(ule → true=포함). 설정 필드 의미는 game-core 미독 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

