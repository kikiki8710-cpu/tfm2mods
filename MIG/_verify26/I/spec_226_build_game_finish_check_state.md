---

### `226` build_game_finish_check_state — 게임 종료(넥서스 마무리) 판정용 상태 빌더 — 아군/적군 생존·체력·근접 집계 + 라인 타워·미니언 푸시 상태를 Option<GameFinishCheckState>(72B) 로 조립

| 항목 | 값 |
|---|---|
| id | `build_game_finish_check_state` |
| 심볼 | `_RNvCshdEBA0ozCnw_7game_ai29build_game_finish_check_state` |
| 소스 | `game-ai\src\lib.rs:1390` |
| IR | `m14.ll` 7703~8214행 |
| 경로·가시성 | `game_ai::build_game_finish_check_state` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e7b0b0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> std::option::Option<game_ai::GameFinishCheckState>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[226]/sig/tls/<키>`)**

- `name`: FINISH_AGG_CACHE
- `role`: 작성자+소비자(읽고 없으면 계산해 쓰는 메모). 이 함수 밖의 다른 소비자는 이 배치 범위에서 미확인
- `key`: thread_local! RefCell<FinishAggCache>(672B = borrow flag i64 @+0 + FinishAggCache 656B @+8). 접점 = `@anon.b0108feec1ab8ff62b7a37c1a95c251f.71 = constant ptr @<FINISH_AGG_CACHE…call_once>` (m14.ll:77) 를 인자로 `LocalKey::with` 호출(m14.ll:7814, lib.rs:1398). 캐시 키 = (seed, tick) 전역 + 슬롯 인덱스 [team][position]
- `layout`: FinishAggCache(656B, lib.rs:1376): +0x000 slots [[Option<FinishAggregates>;5];2] (원소 64B · Some 페이로드 FinishAggregates 0x00~0x39 · 니치 태그 = +0x39 has_epic_buff 바이트, 2=None) · +0x280 seed u64 · +0x288 tick usize. RefCell 안에서는 전부 +8 (슬롯[t][p] 태그 = 8+(t*5+p)*64+57, seed=+648, tick=+656)
- `invalidation`: with 진입 시 (cache.seed != game.seed() || cache.tick != game.tick()) 이면 seed·tick 갱신 + 10 슬롯 전부 태그 2(None) 로 리셋(m00.ll 72490~72513, lib.rs:1400~1402). 같은 (seed,tick) 안에서는 슬롯이 채워지면 그 값을 재생 — 같은 틱 안에서 입력(hp 등)이 바뀌어도 재계산 안 함
- `call_conditions`: champ=player_champion[t][p] 와 enemy_nexus=nexus[1-t] 둘 다 Some 일 때만 with 호출(그 전에 None 반환). 슬롯 [t][p] 가 None(태그 2)일 때만 클로저 본체(집계 계산)가 돌고 Some 이면 64B 복사만. RefCell 이 이미 대여 중이면 panic_already_borrowed(m00.ll 72473)

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<GameFinishCheckState> (72B) | None 이면 +0x44 에 i8 2 만 기록(니치 태그 = has_epic_buff 바이트). Some 이면 +0x00~+0x44 69B 전부 기록, +0x45~+0x47 3B 패딩 미기록 | 4 |
| 1 | 1 | player | &PlayerState (2528B) | 읽는 필드 = info.team(+0x930) · info.position 태그(+0x9c0, i32) | 4 |
| 2 | 2 | data | &OperationData (24B) | cache(+0) · context(+8) · blackboard(+0x10) 세 포인터 전부 읽음 | 4 |
| 3 | 3 | line | LineType (i8, range 0..3) | 0=Top 1=Mid 2=Bottom (tcxdict --enum LineType · Direct). 미니언 상태 슬롯·타워 슬롯 선택과 is_near_line 인자로 씀 | 3 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn build_game_finish_check_state(player, data, line) -> Option<GameFinishCheckState>   // lib.rs:1390
 t = player.info.team (bounds <2) ; p = player.info.position as usize
 champ = data.cache.player_champion[t][p]?        // None → return None (sret+0x44 = 2)   L1391
 enemy_nexus = data.cache.nexus[1-t]?             // None → return None                    L1392
 seed = game.seed(); tick = game.tick()           // vtable +0x20 / +0x28                   L1395~1396
 agg: FinishAggregates = FINISH_AGG_CACHE.with(|c| {                                       L1398 (aux m00.ll 72405~73350)
   c = c.borrow_mut()   // flag 0→-1, 아니면 panic_already_borrowed
   if c.seed != seed || c.tick != tick { c.seed=seed; c.tick=tick; 10 슬롯 전부 None }     L1400~1402
   if let Some(a) = c.slots[t][p] { return a }   // 태그 != 2 → 64B 복사 반환             L1405
   live_allies  = cache.champions(t, pool)     // player_champion[t] 중 Some 만 모은 Vec   L1408
   live_enemies = cache.champions(1-t, pool)                                                L1409
   live_ally_count = live_allies.len ; live_enemy_count = live_enemies.len                  L1411~1412
   healthy_ally_50_count = count(a in allies: a.hp*100/a.stat_cached.hp > 49)              L1414
   healthy_ally_55_count = count(… > 54)                                                    L1417
   near_player_ally_count = count(a: dist²(a, champ) < 1440000000001)   // ≤1,200,000     L1420
   near_enemy_nexus_healthy_ally_50_count = count(a: hp%>49 && dist²(a, enemy_nexus) < 422500000001)  // ≤650,000  L1423
   near_enemy_nexus_healthy_ally_55_count = count(a: hp%>54 && dist²(a, enemy_nexus) < 360000000001)  // ≤600,000  L1426
   has_enemy_twin_tower = if game.tick() > setting.tower_attack_disable_tick { false } else { cache.twin_towers[1-t].len != 0 }   L1428~1429
   has_epic_buff = if tutorial ∈ 1..=6 { false } else { m = game.get_game_mode(); v = if m is Moba(mm) { mm.epic_minion_buff_time[t] } else { 0 }; v >= setting.tick_per_second*10 }   L1433~1434
   c.slots[t][p] = Some(agg); return agg                                                    L1436
 })
 line_tower_alive = if tick > setting.tower_attack_disable_tick { false }                  L1441
                    else { cache.<line>_tower[1-t].is_some() || cache.<line>_tower2[1-t].is_some() }   L1442 (0x180+line*32 / 0x190+line*32)
 ms = data.blackboard[t].{top|mid|bottom}_minion_state (line 별)                           L1444
 front_near_nexus = match ms.front_minion { Some(id) => match game.get_entity_by_id(id) { Some(fm) => dist²(fm, enemy_nexus) < 202500000001 /*≤450,000*/, None => false }, None => false }   L1445~1446
 stable_pushed_line     = ms.minion_count > 3 && (ms.from_mid > 2999 || front_near_nexus)  L1447
 aggressive_pushed_line = ms.minion_count > 3 || (ms.minion_count > 1 && (ms.from_mid > 999 || front_near_nexus))   L1448 (phi 복원: cnt>3 이면 무조건 true)
 live_allies = cache.champions(t, pool)                                                    L1451
 near_finish_line_healthy_ally_50_count = count(a in live_allies: hp%>49 && (is_near_line(ctx, a.x, a.y, line) || dist²(a, enemy_nexus) < 490000000001 /*≤700,000*/))   L1454~1457
 return Some(GameFinishCheckState{ agg 7 usize, near_finish_line_…, agg.has_enemy_twin_tower, line_tower_alive, stable_pushed_line, aggressive_pushed_line, agg.has_epic_buff })   L1459~1472
 // dist² = |dx|²+|dy|² (u64, entity +0x660/+0x668) · hp% 는 정수 나눗셈(max_hp 0 이면 div_by_zero 패닉)
 // 극성: 모든 count 는 조건 true 시 +1 · `A || B` 의 B 는 A 거짓일 때만 평가(IR 분기 그대로)
```

**`mem` 메모리 접근 37건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | t. 2 미만 bounds check(m14.ll 7730) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | p (i32 → zext). 슬롯 인덱스 [t][p] | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 4 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — [t] (stride 744B, 구조체 gep m14.ll 7866) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data ptr) | r | vtable 호출 self | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x8 | game (vtable ptr) | r | 슬롯 +0x20 seed · +0x28 tick · +0x1f0 get_entity_by_id (divtable AbstractGame) | 3 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[t][p]@Some.0 | r | 480 + t*40 + p*8. null(None) → 전체 None 반환 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x170 | nexus[1-t]@Some.0 | r | 368 + (1-t)*8. null → 전체 None 반환 | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x180 | {top,mid,bottom}_tower[1-t] | r | 384 + line*32 + (1-t)*8 — line 별 1차 타워(적) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 10 | AbstractGameWithCache | 0x190 | {top,mid,bottom}_tower2[1-t] | r | 400 + line*32 + (1-t)*8 — line 별 2차 타워(적) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 11 | GameContext | 0x0 | pool (&Bump) | r | champions() 의 alloc 인자 | 4 | OK |  |
| 12 | GameContext | 0x8 | setting (&GameSetting) | r |  | 4 | OK |  |
| 13 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | 5112. tick 이 이보다 크면 line_tower_alive=false(본체)·has_enemy_twin_tower=false(클로저) | 4 | OK |  |
| 14 | Blackboard | 0x0 | top_minion_state / mid(+0x28) / bottom(+0x50) | r | line switch: 0→+0, 1→+40, 2→+80 (BrainMinionParameter 40B) | 4 | OK |  |
| 15 | BrainMinionParameter | 0x0 | front_minion@tag | r | i64 trunc→i1: Some 이면 +8 의 entity id 로 get_entity_by_id | 4 | OK |  |
| 16 | BrainMinionParameter | 0x8 | front_minion@Some.0 (entity id) | r |  | 4 | OK |  |
| 17 | BrainMinionParameter | 0x10 | from_mid (i64) | r | > 2999 / > 999 비교 (단위 미확인 — unknown 참조) | 4 | OK |  |
| 18 | BrainMinionParameter | 0x20 | minion_count (i32) | r | > 3 / > 1 비교 | 4 | OK |  |
| 19 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | hp*100/max — 0 이면 div_by_zero 패닉 | 4 | OK |  |
| 20 | Entity | 0x670 | hp | r |  | 4 | OK |  |
| 21 | Entity | 0x660 | x | r | 거리² 계산 | 4 | OK |  |
| 22 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 23 | GameContext | 0x38 | tutorial (TutorialType 태그) | r | aux 클로저: 1..=6(First~JungleOnly) 이면 has_epic_buff=false | 4 | OK |  |
| 24 | AbstractGameWithCache | 0x148 | twin_towers[1-t].len | r | aux 클로저: 328 + (1-t)*32 (Vec 32B, len +0x18) != 0 | 4 | OK |  |
| 25 | MobaMode | 0x240 | epic_minion_buff_time[t] | r | aux 클로저: 576 + t*8. get_game_mode() 가 Moba(태그 0) 일 때만 | 4 | OK |  |
| 26 | GameSetting | 0x12f8 | tick_per_second | r | aux 클로저: 4856 → *10 | 4 | OK |  |
| 27 | RefCell<FinishAggCache> | 0x0 | borrow flag | r | aux 클로저: 0 이어야 함(-1 로 borrow_mut) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 28 | RefCell<FinishAggCache> | 0x288 | FinishAggCache.seed (+8 보정 = 648) | r | aux 클로저 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 29 | RefCell<FinishAggCache> | 0x290 | FinishAggCache.tick (+8 보정 = 656) | r | aux 클로저 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 30 | RefCell<FinishAggCache> | 0x8 | FinishAggCache.slots[t][p] (64B, 태그 +57) | r | aux 클로저: 8+(t*5+p)*64 · 태그 2=None | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 31 | sret Option<GameFinishCheckState> | 0x44 | has_epic_buff / Option 니치 태그 | w | None 경로에서 유일하게 기록되는 바이트 | 4 | 확인불가(tcx 사전에 타입 없음) | None 경로: i8 2 (m14.ll 7763, 7830) · Some 경로: agg.has_epic_buff (8197) |
| 32 | sret Option<GameFinishCheckState> | 0x0 | live_ally_count..near_finish_line_healthy_ally_50_count (8 usize) + 4 bool | w | m14.ll 8173~8195 (lib.rs:1459) | 4 | 확인불가(tcx 사전에 타입 없음) | +0x00~+0x30 = agg 0x00~0x30 복사 · +0x38 = 본체 계산 near_finish_line · +0x40 = agg.has_enemy_twin_tower · +0x41 line_tower_alive · +0x42 stable_pushed_line · +0x43 aggressive_pushed_line |
| 33 | TLS RefCell<FinishAggCache> | 0x0 | borrow flag | w | aux m00.ll 72458 / 73329 | 4 | 확인불가(tcx 사전에 타입 없음) | -1 (borrow_mut) → 종료 시 +1 복원 |
| 34 | TLS RefCell<FinishAggCache> | 0x288 | seed(+648) · tick(+656) | w | aux m00.ll 72490~72493 (lib.rs:1401~1402) | 4 | 확인불가(tcx 사전에 타입 없음) | game.seed() / game.tick() — 키 불일치 시만 |
| 35 | TLS RefCell<FinishAggCache> | 0x8 | slots[*][*] 태그 10개 (+65,+129,…,+641) | w | aux m00.ll 72494~72513 | 4 | 확인불가(tcx 사전에 타입 없음) | i8 2 (None) — 키 불일치 시 전부 리셋 |
| 36 | TLS RefCell<FinishAggCache> | 0x8 | slots[t][p] (8+(t*5+p)*64, 0x00~0x39) | w | aux m00.ll 73233~73248 (lib.rs:1436) — 슬롯이 None 이었을 때만 | 4 | 확인불가(tcx 사전에 타입 없음) | 계산된 FinishAggregates 9 필드(태그 = has_epic_buff 0/1 = Some) |

**`consts` 상수 17건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 1391 | 센티널 | ①team bounds(<2) ②Option 니치 None 태그(store i8 2 @+68) ③aux: 슬롯 None 태그 비교/리셋 | 4 |
| 1 | 202500000001 | 1446 | 임계 | 450000²+1 — front_minion↔enemy_nexus 거리² < 이 값 ⇔ 거리 ≤ 450,000(=14.06셀) → front_near_nexus | 4 |
| 2 | 3 | 1447 | 임계 | minion_count > 3 (stable/aggressive 1차 조건) (`shl nuw nsw i8 %3, 5` 의 %3 은 인자 레지스터명 — 이 상수와 무관, 접힘 아님) | 4 |
| 3 | 2999 | 1447 | 임계 | from_mid > 2999 (stable_pushed_line 의 OR 항) | 4 |
| 4 | 1 | 1448 | 임계 | minion_count > 1 (aggressive 2차 조건) · aux: 1-t 팀 반전 · 루프 증가 | 4 |
| 5 | 999 | 1448 | 임계 | from_mid > 999 (aggressive_pushed_line 의 OR 항) | 4 |
| 6 | 100 | 1454 | 계수 | hp*100/max_hp = HP 백분율 (aux 4곳도 동일) | 4 |
| 7 | 49 | 1454 | 임계 | HP% > 49 ⇔ ≥50% 건강 (aux: healthy_ally_50 · near_enemy_nexus_50 도 동일) | 4 |
| 8 | 490000000001 | 1455 | 임계 | 700000²+1 — 아군↔enemy_nexus 거리² < 이 값 ⇔ 거리 ≤ 700,000(=21.9셀) → near_finish_line 대체 조건 | 4 |
| 9 | 54 | 1417 | 미상 | aux: HP% > 54 ⇔ ≥55% (healthy_ally_55 · near_enemy_nexus_55) | 4 |
| 10 | 1440000000001 | 1420 | 미상 | aux: 1200000²+1 — 아군↔내 챔피언 거리² < 이 값 ⇔ ≤1,200,000(=37.5셀) → near_player_ally_count | 4 |
| 11 | 422500000001 | 1423 | 미상 | aux: 650000²+1 — HP≥50% 아군↔enemy_nexus 거리 ≤ 650,000(=20.3셀) → near_enemy_nexus_healthy_ally_50_count | 4 |
| 12 | 360000000001 | 1426 | 미상 | aux: 600000²+1 — HP≥55% 아군↔enemy_nexus 거리 ≤ 600,000(=18.75셀) → near_enemy_nexus_healthy_ally_55_count | 4 |
| 13 | 10 | 1434 | 미상 | aux: epic_minion_buff_time[t] >= tick_per_second*10 (10초 이상 남음) → has_epic_buff | 4 |
| 14 | 6 | 1433 | 미상 | aux: (tutorial-1) <u 6 ⇔ tutorial ∈ {1..6}=First,TopSolo,Bottom,MidSolo,MidBottom,JungleOnly → has_epic_buff=false | 4 |
| 15 | -1 | 1433 | 미상 | aux: ①RefCell borrow_mut 플래그 값 ②tutorial-1 (add nsw i8 %291, -1) 범위검사 | 4 |
| 16 | 5 | 1405 | 미상 | aux: position bounds(<5) (`shl i8 %3, 5` 는 line*32 stride 시프트량 — 이 bounds 상수와 별개, 접힘 아님) | 4 |

**`knobs` 조정점 9건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | front_minion 근접 넥서스 반경 | lib.rs:1446 | 202500000001 | 올리면 선두 미니언이 더 멀리 있어도 front_near_nexus=true → stable/aggressive_pushed_line 이 더 쉽게 켜짐 | 4 | 기존 |
| 1 | stable_pushed_line 미니언 수 임계 | lib.rs:1447 | 3 | 낮추면 적은 미니언으로도 안정 푸시로 판정(aggressive 도 같은 3 공유) | 4 | 기존 |
| 2 | stable_pushed_line from_mid 임계 | lib.rs:1447 | 2999 | 낮추면 from_mid 가 작아도 안정 푸시 | 4 | 기존 |
| 3 | aggressive_pushed_line 미니언 수·from_mid 임계 | lib.rs:1448 | 1 / 999 | 낮추면 공격적 푸시 판정이 늘어남 | 4 | 기존 |
| 4 | 마무리 라인 근접 반경(넥서스 기준) | lib.rs:1455 | 490000000001 | 올리면 near_finish_line_healthy_ally_50_count 증가 | 4 | 기존 |
| 5 | 건강 HP% 임계 50/55 | lib.rs:1414/1417/1423/1426/1454 | 49 / 54 | 낮추면 healthy·near_nexus 카운트 증가(마무리 판단이 공격적으로) | 4 | 기존 |
| 6 | 아군 근접 반경(내 챔피언 기준) | lib.rs:1420 | 1440000000001 | 올리면 near_player_ally_count 증가 | 4 | 기존 |
| 7 | 적 넥서스 근접 반경 50/55 | lib.rs:1423/1426 | 422500000001 / 360000000001 | 올리면 near_enemy_nexus_healthy_* 증가 | 4 | 기존 |
| 8 | 에픽 버프 잔여 시간 임계 | lib.rs:1434 | 10 | tps*10 = 10초. 내리면 버프 잔여가 짧아도 has_epic_buff=true | 4 | 기존 |

<details><summary>`callees` 피호출자 18건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | build_game_finish_check_state | game_ai::build_game_finish_check_state | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> std::option::Option<game_ai::GameFinishCheckState> | game-ai\src\lib.rs:1390 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | seed | game_core::AbstractGame::seed | pub | fn(&Self/#0) -> u64 | game-core\src\simulation.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | seed | <game_core::Game as game_core::AbstractGame>::seed | pub | fn(&game_core::Game) -> u64 | game-core\src\simulation\game.rs:1564 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | seed | <game_core::SingleLaneGame as game_core::AbstractGame>::seed | pub | fn(&game_core::SingleLaneGame) -> u64 | game-core\src\simulation\game.rs:3781 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 5개**: `borrow_mut`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `panic_access_error`, `panic_already_borrowed`, `with  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 2개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m14.ll:8769, m14.ll:10088) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | AbstractGameWithCache::player_champion[t][p] 가 None 이 되는 정확한 조건(사망/미스폰) — champions() 는 Some 만 모으므로 '생존 챔피언' 으로 읽었으나 game_core 캐시 갱신 코드는 이 배치에서 안 읽음(DI 변수명 live_allies/live_enemies 가 근거) | 4 |  |
| 1 | 미탐색 | is_near_line(ctx, x, y, line) 내부 기하(192000/704000/96000 상수 관측) — game_core 경계라 시그니처·bool 반환만 기록 | 4 |  |
| 2 | 미탐색 | FINISH_AGG_CACHE 의 다른 소비자 유무 — @anon…71 상수 참조는 m14.ll 에서 이 함수 1곳만 확인, 다른 모듈은 미탐색 | 4 |  |
| 3 | 미탐색 | exe argscan 대응(internal fastcc · sret+3인자) 은 지시대로 열람 금지 — IR 정본 기준으로만 기록 | 3 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | BrainMinionParameter.from_mid(+0x10, i64) 의 단위·의미 — tcxdict 이름만 확보. 2999/999 임계가 좌표 단위인지 별도 스케일인지 이 배치 재료(IR·_docs)로는 미확정 (_docs grep 0건) | 3 | 사실 서술 |
| 1 | TutorialType 1..=6 제외의 소스 표기(matches! 인지 헬퍼 함수인지) — L263<46<1433 inlinedAt 사슬은 runner.rs:263 헬퍼로 보이나 이름 미확인. 동작(태그 1~6 이면 false)은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

