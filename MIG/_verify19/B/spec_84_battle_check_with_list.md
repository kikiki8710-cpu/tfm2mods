---

### `84` battle_check_with_list — 아군/적 포지션 목록으로 양측 dps·체력을 합산해 「누가 먼저 녹는가」를 -100..100 점수로 낸다(양수=아군 유리)

| 항목 | 값 |
|---|---|
| id | `fight_check__battle_check_with_list` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai11fight_check22battle_check_with_list` |
| 소스 | `game-ai\src\fight_check.rs:425` |
| IR | `m15.ll` 29104~30224행 |
| 경로·가시성 | `game_ai::battle_check_with_list` · **pub** |
| 계층 | 점수화·술어 |
| exe | `eb9570` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &bumpalo::collections::vec::Vec< usize>, &bumpalo::collections::vec::Vec< usize>, &mut game_core::DebugFrameData) -> i32
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize | 미사용 | 4 |
| 1 | 2 | _rnd | &mut StdRng | readnone — 미사용 | 4 |
| 2 | 3 | data | &OperationData (24B) | +0x0 cache, +0x8 context(+0x8 setting → tick_per_second) | 4 |
| 3 | 4 | player | &PlayerState (2528B) | +0x930 team 만 읽음(bounds<2) | 4 |
| 4 | 5 | ally_list | &bumpalo Vec<usize> (32B: +0 ptr, +24 len) | 아군 포지션 인덱스 목록(각 <5, panic @449). player_champion[team][ap] | 4 |
| 5 | 6 | enemy_list | &bumpalo Vec<usize> (32B) | 적 포지션 인덱스 목록(각 <5, panic @461/@482). player_champion[1-team][ep] | 4 |
| 6 | 7 | _debug | &mut DebugFrameData | readnone — 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
battle_check_with_list(_version, _rnd, data, player, ally_list, enemy_list, _debug) -> i32
  tps = setting.tick_per_second; horizon = max(tps*6, 1)                                   // L439
  ult_burst = |e, cc, tgt_pos| if e.can_ult() || !(e.ty==Champion && e.ult_cooldown > horizon)   // L440~441
                 { tps * cc.ult[tgt_pos] / horizon } else { 0 }                                // L442
  my_dps = my_tanking = enemy_dps = enemy_tanking = 0; my_cnt = enemy_cnt = 0               // L430~435
  for ap in ally_list {                                                                     // L448
    c = player_champion[team][ap] else continue                                             // L449~450
    my_tanking += c.hp; my_cnt += 1                                                         // L458~459
    mdps = edps = 0; cnt = 0
    for ep in enemy_list { e = player_champion[1-team][ep] else continue                     // L460~462
      mdps += cc[team][ap].{attack,skill,skill2}_per_sec[ep] + ult_burst(c, cc[team][ap], ep)      // L466~469
      edps += cc[1-team][ep].{attack,skill,skill2}_per_sec[ap] + ult_burst(e, cc[1-team][ep], ap)  // L470~473
      cnt += 1 }                                                                            // L474
    my_dps += mdps / max(cnt,1); enemy_dps += edps / max(cnt,1) }                           // L477~478  (적 수 평균)
  for ep in enemy_list { e = player_champion[1-team][ep] else continue; enemy_tanking += e.hp; enemy_cnt += 1 }   // L481~488
  for e in cache.others[1-team] {                                                           // L493
    near = ally_list.any(|ap| player_champion[team][ap] is Some(a) && dist_sq(a, e) <= 150000²)   // L494~495
    if near { if let Some(atk) = e.attack_effect {                                          // L498
      total = 0; cnt = 0
      for ap in ally_list { a = player_champion[team][ap] else continue                     // L501~502
        total += expected_damage_target(atk, ctx, e, <static>, a) * tps / max(e.attack_cooltime(),1); cnt += 1 }   // L503~504
      enemy_dps += total / max(cnt,1); enemy_tanking += e.hp } }                            // L507~508
  }
  (L512~527: cache.others[team] × enemy_list 로 대칭 — my_dps / my_tanking 에 가산)
  if my_cnt == 0 { return -20 * enemy_cnt }                                                 // L531~532
  if enemy_cnt == 0 { return 20 * my_cnt }                                                  // L533~534
  my_ratio    = my_tanking    * 10000 / max(enemy_dps, 1)                                   // L536  (내가 버티는 시간)
  enemy_ratio = enemy_tanking * 10000 / max(my_dps, 1)                                      // L537
  if my_ratio > 100000 && enemy_ratio > 100000 { return 0 }                                 // L540
  if my_ratio < enemy_ratio { return (my_ratio*100/enemy_ratio) as i32 - 100 }              // L544~545
  if my_ratio == enemy_ratio { return 0 }                                                   // L546
  return 100 - (enemy_ratio*100/my_ratio) as i32                                            // L549  (my_ratio==0 이면 div-by-zero 패닉)
```

**`mem` 메모리 접근 19건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 0..1 (bounds<2). 1-team = 적 | 4 | OK |
| 1 | OperationData | 0x0 | cache | r |  | 4 | OK |
| 2 | OperationData | 0x8 | context | r | +0x8 setting · expected_damage_target 의 ctx 인자 | 4 | OK |
| 3 | GameContext | 0x8 | setting | r |  | 4 | OK |
| 4 | GameSetting | 0x12f8 | tick_per_second | r | L439 horizon = tps*6 · L442/L503/L522 dps 환산 계수 | 4 | OK |
| 5 | AbstractGameWithCache | 0xf0 | others[] | r | [1-team] (L493) / [team] (L512): 챔피언 외 엔티티 목록(bumpalo Vec<&Entity>: +0 ptr, +24 len) | 4 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion | r | [team][ap] 아군 챔프 / [1-team][ep] 적 챔프 (None 은 건너뜀) | 4 | OK |
| 7 | AbstractGameWithCache | 0x280 | player_champion_cache[][] | r | [team][pos] ChampionCache. 열 = 상대 포지션 | 4 | OK |
| 8 | ChampionCache | 0x78 | ult[pos] | r | L442 ult_burst: tps*ult[상대pos]/horizon (궁 1회 피해를 6초에 펴서 dps 로) (IR 에선 cache 기준 `+760` = 0x280+0x78 로 접혀 있어 C3 경고 — gep 접힘) | 4 | OK |
| 9 | ChampionCache | 0x190 | attack_per_sec[pos] | r | L466/L470 | 4 | OK |
| 10 | ChampionCache | 0x1b8 | skill_per_sec[pos] | r | L467/L471 | 4 | OK |
| 11 | ChampionCache | 0x1e0 | skill2_per_sec[pos] | r | L468/L472 | 4 | OK |
| 12 | Entity | 0x68 | ty@tag | r | L441 ult_cooldown(entity.rs:1805) 인라인: Champion(13) 만 궁 쿨 읽음 | 4 | OK |
| 13 | Entity | 0xc8 | ty@Champion.0.ult_cooldown | r | L441: can_ult() 아니고 Champion 이며 이 값 > horizon 이면 궁 dps 0 | 4 | OK |
| 14 | Entity | 0x490 | attack_effect@Some.0 | r | L498/L517 others 엔티티의 공격 Effect(56B) → expected_damage_target | 4 | OK |
| 15 | Entity | 0x4c0 | attack_effect@tag | r | i32 -1 = None → 그 others 엔티티는 dps·hp 모두 미합산 | 4 | OK |
| 16 | Entity | 0x660 | x | r | L495/L514 near_battle distance_sq | 4 | OK |
| 17 | Entity | 0x668 | y | r |  | 4 | OK |
| 18 | Entity | 0x670 | hp | r | L458 my_tanking / L487 enemy_tanking / L508·L527 others hp | 4 | OK |

**`consts` 상수 12건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 6 | 439 | 계수 | horizon = tps*6 (6초 전망 창). max(…,1) | 4 |
| 1 | 1 | 439 | 임계 | 0 나눗셈 방지 하한(max(x,1)): horizon·cnt·attack_cooltime·tanking·dps | 4 |
| 2 | 13 | 441 | 태그 | EntityType::Champion — 궁 쿨다운은 챔피언만(ult_cooldown entity.rs:1805) | 4 |
| 3 | 22500000001 | 495 | 임계 | 150000²+1 — others 엔티티가 「전장 근처」인지: 어느 아군(L495)/적(L514) 챔프와의 제곱거리 < 이 값 | 4 |
| 4 | -1 | 498 | 센티널 | attack_effect Option 니치(i32) None 검사(L498·L517) | 4 |
| 5 | -20 | 532 | 계수 | 아군 0명 → -20 × enemy_cnt | 4 |
| 6 | 20 | 534 | 계수 | 적 0명 → +20 × my_cnt | 4 |
| 7 | 10000 | 536 | 계수 | 생존비 스케일: my_ratio = my_tanking*10000/enemy_dps, enemy_ratio = enemy_tanking*10000/my_dps | 4 |
| 8 | 100000 | 540 | 임계 | 양측 생존비 모두 > 100000 (= 10초분 ×10000… dps 가 사실상 0) 이면 0 반환(판정 보류) | 4 |
| 9 | 100 | 545 | 계수 | 비율 % 환산: my<enemy → my*100/enemy − 100 ; my>enemy → 100 − enemy*100/my | 4 |
| 10 | 5 | 449 | 임계 | 포지션 bounds(panic_bounds_check) — 판정 아님 | 4 |
| 11 | 2 | 448 | 임계 | 팀 bounds — 판정 아님 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 전망 창(궁 dps 환산 분모·궁 쿨 허용) | fight_check.rs:439 | tps*6 | 올리면 궁 기여가 작아지고(ult/horizon) 더 긴 쿨의 궁도 포함된다 | 4 | 기존 |
| 1 | others 엔티티 전장 반경 | fight_check.rs:495·514 | 22500000001 | 올리면 더 먼 타워/소환수 dps·hp 가 합산된다 | 4 | 기존 |
| 2 | 한쪽 부재 시 점수 단가 | fight_check.rs:532·534 | ±20/명 | 올리면 일방 상황 점수의 절대값이 커진다(호출자의 임계와 상대적) | 4 | 기존 |
| 3 | 판정 보류 생존비 상한 | fight_check.rs:540 | 100000 | 내리면 dps 가 낮은 대치 상황도 0 이 아니라 ± 점수를 낸다 | 4 | 기존 |

<details><summary>`callees` 피호출자 4건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | battle_check_with_list | game_ai::battle_check_with_list | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &bumpalo::collections::vec::Vec< usize>, &bumpalo::collections::vec::Vec< usize>, &mut game_core::DebugFrameData) -> i32 | game-ai\src\fight_check.rs:425 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `dist_sq`, `ult_burst`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m09.ll:12334, m15.ll:23307) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 반환값을 소비하는 호출자(exe 0xdcee40 1곳)의 임계 — 범위 밖 | 4 |  |
| 1 | 미탐색 | expected_damage_target 의 4번째 인자 정적 데이터(@anon…)의 내용 — 미독 | 4 |  |
| 2 | 미탐색 | cache.others 에 어떤 엔티티(타워/소환수/정글몹 등)가 들어가는지 — game_core 캐시 구성 미독 | 4 |  |
| 3 | 미탐색 | L536~537 변수명(my_ratio/enemy_ratio)은 DI 기준. 「ratio」가 실제로는 생존 시간(tanking/dps) 스케일값이라는 해석은 산식에서 도출 | 4 |  |
| 4 | 표기 불가 | L440 클로저 인자 구성(ult_burst 가 (Entity, cache, pos) 를 받는지) — 인라인돼 형태 표기 불가, 산식은 확정 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

