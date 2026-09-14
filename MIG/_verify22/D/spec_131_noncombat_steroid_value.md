---

### `131` noncombat_steroid_value — [v54 비교전 스테로이드] 버프 1개가 비교전 창(구조물/오브젝트/웨이브 공격)에서 갖는 가치 — 스탯별 항목 가치(각 상한 15~40)를 합산해 /2 후 40 으로 캡한 i64

| 항목 | 값 |
|---|---|
| id | `buff_value__noncombat_steroid_value` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai10buff_value23noncombat_steroid_value` |
| 소스 | `game-ai\src\buff_value.rs:458` |
| IR | `m10.ll` 34945~35856행 |
| 경로·가시성 | `game_ai::buff_value::noncombat_steroid_value` · **in:game_ai** |
| 계층 | 점수화·술어 |
| exe | `e02540` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::OperationData, &game_core::BuffState, &game_core::Entity, &game_ai::NoncombatSteroidWindow) -> i64
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | data | &OperationData(24B) | +0x0 cache 만 읽음 → player_by_champion_id / player_champion_cache / player_champion | 4 |
| 1 | 2 | buff | &BuffState(288B) | game_core::BuffState(entity.rs:933). 읽는 필드 12개(reads 참조) | 4 |
| 2 | 3 | target | &Entity(1728B) | 버프를 받을 챔피언. stat_cached(attack/magic_power/hp)·id·team·x/y·hp 읽음 | 4 |
| 3 | 4 | window | &NoncombatSteroidWindow(2B) | +0x0 fights_back 만 읽음(m10.ll:35837). +0x1 minion_wave 는 본 함수에서 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn noncombat_steroid_value(data, buff:&BuffState, target:&Entity, window:&NoncombatSteroidWindow) -> i64
  let stat = target.get_stat()   // L460 (entity.rs:789 인라인) → attack=0x618, magic_power=0x620, hp(max)=0x628
  // L461 attack_share_permille(cache, target.id)  (buff_value.rs:59~71 인라인, 34966~35305)
  let share = match cache.player_by_champion_id(target.id) { None => 500,
      Some(p) => { let c = &cache.player_champion_cache[p.info.team][p.info.position.as_index()];
                   let a = sum(c.attack_per_sec[0..5]);                                              // L63
                   let total = a + sum(c.skill_per_sec) + sum(c.skill2_per_sec) + sum(c.ult_per_sec); // L64~67
                   if total == 0 { 500 } else { a*1000/total } } }                                    // L67~70
  let mut v = 0
  if buff.attack_mult > 0        { v += min(stat.attack * attack_mult / 100, 40) }                  // L463~464 (35308~35329)
  if buff.attack_speed_mult > 0  { v += min((stat.attack * as_mult / 200) * share / 500, 40) }      // L466~467 (35316~35364)
  v += clamp(buff.attack, 0, 30)                                                                    // L469 (35334~35339)
  v += clamp(buff.magic_power, 0, 30)                                                              // L472 (35341~35346)
  if buff.magic_power_mult > 0   { v += min(stat.magic_power * mp_mult / 100, 40) }                 // L475~476 (35348~35383)
  if buff.crit_chance > 0        { v += min(share * (crit/5) / 500, 20) }                           // L478~479 (35369~35403)
  if buff.defence_penetration != 0           { v += min(def_pen / 3, 20) }                          // L481~482 (35388~35420)
  if buff.magic_resistance_penetration != 0  { v += min(mr_pen / 3, 20) }                           // L484~485 (35408~35437)
  if buff.range != 0             { v += min(max(range/3000,1) * max(stat.attack/50,1), 25) }        // L487~488 (35425~35465)
  if buff.skill_cooldown_mult >= 1 && let Player(t) = target.team {                                // L494 (35442~35448; <1 || Neutral → 생략)
     let n = cache.player_champion[t].iter().flatten()                                              // L495~496 iter_champions 인라인, 5명 전개
              .filter(|c| distance_sq(c, target) < 60000²+1).count()                               // closure#0 L497 (target 자신 포함)
     if n > 1 { v += min((1000 - share) * (cd_mult/5) / 500, 15) } }                                // L502~503 (35806~35825)
  if buff.vamp > 0 && (window.fights_back || target.hp < stat.hp) {                                 // L506 (35470~35473, 35837~35843)
     v += min(share * (vamp/5) / 500, 15) }                                                          // L507 (35846~35855)
  return min(v / 2, 40)                                                                             // L509~510 (35830~35834)
극성: 35311 attack_mult>0 → %106 가산 / 35319 → %128 / 35351 → %141 / 35372 → %152 / 35391 ==0 → %159 건너뜀 / 35411 ==0 → %168 건너뜀 / 35428 ==0 → %177 건너뜀 / 35448 select(cd<1 || Neutral) true → %193 건너뜀 / 35808 n>1 → %329 가산 / 35843 select(fights_back || hp<max) true → %348 가산.
```

**`mem` 메모리 접근 30건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Entity | 0x618 | stat_cached.attack | r | target. get_stat()(entity.rs:789 인라인) 조각 stat[0..8] (34951~34952). attack_mult·attack_speed_mult·range 항의 곱수 | 4 | OK |
| 1 | Entity | 0x620 | stat_cached.magic_power | r | target. stat[8..16] (34954~34955). magic_power_mult 항 | 4 | OK |
| 2 | Entity | 0x628 | stat_cached.hp | r | target 최대HP. stat[16..24] (34957~34958). L506 에서 현재 hp 와 비교 | 4 | OK |
| 3 | Entity | 0x5c0 | id | r | target 챔피언 id → player_by_champion_id (34967~34978, L461) | 4 | OK |
| 4 | Entity | 0x0 | team@tag | r | target. trunc i1: 1=Neutral 이면 쿨감 항 생략(35445~35448, L494) | 4 | OK |
| 5 | Entity | 0x8 | team@Player.0 | r | target 팀번호 → player_champion[t] (35476~35482, L495~496). <2 아니면 panic(35811) | 4 | OK |
| 6 | Entity | 0x660 | x | r | target(35503)·아군 5명(35528 등) → distance_sq (closure buff_value.rs:497 인라인) | 4 | OK |
| 7 | Entity | 0x668 | y | r | target(35505)·아군 5명(35530 등) | 4 | OK |
| 8 | Entity | 0x670 | hp | r | target 현재HP. hp < stat_cached.hp (풀피 아님) 이 vamp 항 조건의 한 축(35839~35842, L506) | 4 | OK |
| 9 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(34966) | 4 | OK |
| 10 | PlayerState | 0x930 | info.team | r | player_by_champion_id 결과 p. player_champion_cache 1차 인덱스(34984~34987, buff_value.rs:62 인라인) | 4 | OK |
| 11 | PlayerState | 0x9c0 | info.position@tag | r | as_index() → 2차 인덱스(34995~34997) | 4 | OK |
| 12 | AbstractGameWithCache | 0x280 | player_champion_cache[team][pos] | r | [2][5] x 800B(20 x [5 x usize]). 34998~35000. 아래 4 배열은 이 원소 기준 상대 오프셋 | 4 | OK |
| 13 | AbstractGameWithCache | 0x410 | player_champion_cache[..].attack_per_sec[0..5] | r | 원소+400(35002~35061). 합 = 평타 DPS 몫 A (buff_value.rs:63) | 4 | OK |
| 14 | AbstractGameWithCache | 0x438 | player_champion_cache[..].skill_per_sec[0..5] | r | 원소+440(35068~35128). (L64) | 4 | OK |
| 15 | AbstractGameWithCache | 0x460 | player_champion_cache[..].skill2_per_sec[0..5] | r | 원소+480(35135~35194). (L65) | 4 | OK |
| 16 | AbstractGameWithCache | 0x488 | player_champion_cache[..].ult_per_sec[0..5] | r | 원소+520(35201~35259). (L66) | 4 | OK |
| 17 | AbstractGameWithCache | 0x1e0 | player_champion[t][0..5] | r | [2][5] Option<&Entity>. iter_champions(simulation.rs:1905 인라인) → 5명 null 검사 후 거리 카운트(35486~35802, L496) | 4 | OK |
| 18 | BuffState | 0x5c | attack_mult | r | i32 >0 게이트(35308~35311, L463) | 4 | OK |
| 19 | BuffState | 0x8c | attack_speed_mult | r | i32 >0 게이트(35316~35319, L466) | 4 | OK |
| 20 | BuffState | 0x58 | attack | r | i32 → clamp(0,30) 무조건 가산(35334~35339, L469) ⚠dbg 없음: 오프셋 0x58 = BuffState.attack(평타 고정치) | 4 | OK |
| 21 | BuffState | 0x60 | magic_power | r | i32 → clamp(0,30) 무조건 가산(35341~35346, L472) | 4 | OK |
| 22 | BuffState | 0x64 | magic_power_mult | r | i32 >0 게이트(35348~35351, L475) | 4 | OK |
| 23 | BuffState | 0x104 | crit_chance | r | i32 >0 게이트(35369~35372, L478) | 4 | OK |
| 24 | BuffState | 0xa8 | defence_penetration | r | usize !=0 게이트(35388~35391, L481) | 4 | OK |
| 25 | BuffState | 0xb0 | magic_resistance_penetration | r | usize !=0 게이트(35408~35411, L484) | 4 | OK |
| 26 | BuffState | 0xc8 | range | r | usize !=0 게이트(35425~35428, L487) | 4 | OK |
| 27 | BuffState | 0x90 | skill_cooldown_mult | r | i32. <1 이면 쿨감 항 생략(35442~35444, L494) | 4 | OK |
| 28 | BuffState | 0x80 | vamp | r | i32 >0 게이트(35470~35473, L506) | 4 | OK |
| 29 | NoncombatSteroidWindow | 0x0 | fights_back | r | bool. true 면 hp 조건 없이 vamp 항 유효(35837~35842, L506) — docs L38: 반격형 대상(에픽/세르펜/정글캠프)이라 회복 소요 발생 | 4 | OK |

**`consts` 상수 17건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 500 | 461 | 계수 | ① attack_share_permille 기본값(플레이어 못 찾음/총합 0 → 500‰, 35305 phi) ② permille 정규화 분모 — share*x/500 = share 가 500‰ 일 때 x 그대로(35357·35396·35818·35849) | 4 |
| 1 | 1000 | 461 | 계수 | permille 스케일: share = A*1000/total (buff_value.rs:70 인라인, 35290). L503 에선 (1000 - share) = 스킬 DPS 몫(35817) | 4 |
| 2 | 100 | 464 | 계수 | 퍼센트 분모 — stat.attack*attack_mult/100 (35324) · stat.magic_power*magic_power_mult/100 (35377) | 4 |
| 3 | 40 | 464 | 임계 | 항목 상한(smin): attack_mult(35327)·attack_speed_mult(35361)·magic_power_mult(35380) 항 각각 40, 최종 결과도 40 캡(35833) | 4 |
| 4 | 200 | 467 | 계수 | 공속 항 분모 — stat.attack*attack_speed_mult/200 (공격력 항의 절반 가중, 35356) 뒤 *share/500 | 4 |
| 5 | 30 | 469 | 임계 | attack(0x58)·magic_power(0x60) 고정치 clamp 상한 (umin i32 30, 35337·35344) | 4 |
| 6 | 20 | 479 | 임계 | 항목 상한: crit(35400)·defence_penetration(35417)·magic_resistance_penetration(35434) 각 20 | 4 |
| 7 | 5 | 479 | 계수 | 치명타/쿨감/흡혈 % 를 5% 단위로 환산 — crit/5(udiv 35394), cd_mult/5(35815), vamp/5(35846) | 4 |
| 8 | 3 | 482 | 계수 | 관통 가치 = 관통값/3 (sdiv 35414·35431) | 4 |
| 9 | 3000 | 488 | 계수 | 사거리 단위 — max(range/3000, 1) (35451~35454). 좌표 단위(셀 32000)라 3000 ≈ 0.09셀 | 4 |
| 10 | 50 | 488 | 계수 | 사거리 항 공격력 가중 — max(stat.attack/50, 1) (35455~35458) | 4 |
| 11 | 25 | 488 | 임계 | 사거리 항 상한(smin, 35462) | 4 |
| 12 | 1 | 488 | 임계 | ① max(.,1) 바닥(35454·35458) ② L494 skill_cooldown_mult < 1 이면 쿨감 항 생략(35444) ③ 35481 ult 2 는 팀 bounds | 4 |
| 13 | 3600000001 | 497 | 임계 | 60000² + 1 — 아군 챔피언이 target 반경 60000(≈1.9셀) 안이면 카운트(dist_sq < 60000²+1 ⇔ dist ≤ 60000; closure#0 buff_value.rs:497 인라인, 35562·35619·35677·35735·35793) | 4 |
| 14 | 15 | 503 | 임계 | 항목 상한: 쿨감 항(35822)·흡혈 항(35852) 각 15 | 4 |
| 15 | 2 | 509 | 임계 | 최종 합계 /2 (sdiv 35830). 34986·35481 의 2 는 팀 배열 bounds(임계 아님). L502 `count > 1` 은 samesign ugt 1(35806) = 자기 포함 2명 이상 | 4 |
| 16 | 0 | 463 | 임계 | 각 게이트의 비교 기준(>0 / !=0) 및 clamp 하한(smax 0). 35286 총합==0 → share 500 | 4 |

**`knobs` 조정점 9건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 공격력 배율 항 상한 | buff_value.rs:464 (m10.ll:35327) | 40 | 내리면 대형 공격력 스테로이드의 비교전 가치가 깎여 오브젝트/타워 창에서 덜 씀 | 4 | 기존 |
| 1 | 공속 항 분모(공격력 항 대비 가중) | buff_value.rs:467 (m10.ll:35356) | 200 | 올리면 공속 버프의 비교전 가치 감소. 평타 DPS 몫(share)이 낮은 스킬형 챔프는 추가로 share/500 배 감쇠 | 4 | 기존 |
| 2 | 고정 공격력/주문력 clamp 상한 | buff_value.rs:469·472 (m10.ll:35337·35344) | 30 | 고정치 버프의 기여 상한 | 4 | 기존 |
| 3 | 치명/관통 항 상한 | buff_value.rs:479·482·485 (m10.ll:35400·35417·35434) | 20 | 각 항 기여 상한 | 4 | 기존 |
| 4 | 관통 환산 분모 | buff_value.rs:482·485 (m10.ll:35414·35431) | 3 | 내리면 관통 버프 가치 증가(상한 20 까지) | 4 | 기존 |
| 5 | 사거리 항 단위/가중/상한 | buff_value.rs:488 (m10.ll:35451·35455·35462) | 3000 | range/3000 × attack/50, 상한 25. 3000 을 올리면 사거리 버프 가치 감소 | 4 | 기존 |
| 6 | 쿨감 항 아군 근접 반경(제곱) | buff_value.rs:497 closure (m10.ll:35562 등 5곳) | 3600000001 | 60000²+1. 올리면 더 먼 아군도 '함께 있음'으로 세어 쿨감 버프가 비교전 창에서 가치를 얻기 쉬움(n>1 조건) | 4 | 기존 |
| 7 | 쿨감/흡혈 항 상한 | buff_value.rs:503·507 (m10.ll:35822·35852) | 15 | 각 항 기여 상한 | 4 | 기존 |
| 8 | 최종 반감·캡 | buff_value.rs:509 (m10.ll:35830·35833) | 40 | 합계/2 후 40 캡. 캡을 올리면 다항목 버프가 더 높은 가치 | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | attack_share_permille | game_ai::buff_value::attack_share_permille | in:game_ai | fn(&game_core::OperationData, &game_core::Entity) -> i64 | game-ai\src\buff_value.rs:58 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 5 | get_stat | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::get_stat | pub | fn(&game_ai::WindowStatView</#0>) -> game_core::EntityStat | game-ai\src\fight_check.rs:93 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 4개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | get_stat | game_core::AbstractEntity::get_stat | pub | fn(&Self/#0) -> game_core::EntityStat | game-core\src\simulation\entity.rs:713 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 4개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | get_stat | game_view::DatabaseEditUIRunner::get_stat | in:game_view::ui::database_edit_ui | fn(&game_core::AthleteStat, &str) -> usize | game-view\src\ui\database_edit_ui.rs:5331 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 4개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | noncombat_steroid_value | game_ai::buff_value::noncombat_steroid_value | in:game_ai | fn(&game_core::OperationData, &game_core::BuffState, &game_core::Entity, &game_ai::NoncombatSteroidWindow) -> i64 | game-ai\src\buff_value.rs:458 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 9 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 5개**: `clamp`, `llvm.smax.i32`, `llvm.smax.i64`, `llvm.smin.i64`, `llvm.umin.i32`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m05.ll:43329, m05.ll:43910) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 호출자(1곳)와 반환값이 어느 임계와 비교되는지(사용 결정)는 범위 밖 — docs L37 '구조물/오브젝트 공격 창 판정 결과' 에 근거해 비교전 창 가치로 해석 | 4 |  |
| 1 | 미탐색 | window.minion_wave(+0x1) 는 본 함수에서 안 읽음 — docs L39 '파밍 보조만 유효' 는 호출자 쪽 필터로 추정(미확인) | 5 |  |
| 2 | 미탐색 | get_stat()(entity.rs:789) 이 stat_cached 를 그대로 돌려주는지, 버프 포함 스탯인지 — IR 은 0x618/0x620/0x628 직접 로드(stat_cached) 로 확정. docs L92~93 '창 스탯 뷰' 와의 관계는 미확인 | 4 |  |
| 3 | 미탐색 | player_by_champion_id(m10.ll:28070, internal fastcc, game_core 함수의 game_ai 내 인스턴스) 내부 미독 — 계약: (cache, champion_id) → Option<&PlayerState>(null=None) | 4 |  |
| 4 | 미탐색 | L502 count 에 target 자신이 포함되는지는 player_champion[t] 에 target 이 있으면 dist 0 이라 포함(추정: target 은 챔피언이므로 항상 포함 → n>1 ⇔ 다른 아군 1명 이상 60k 내) | 5 |  |
| 5 | 미탐색 | share 가 500 기본값일 때와 실측값의 분포는 런타임 재료 — 미확인 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | BuffState+0x58(attack) 로드(35334)에 dbg_value 가 없어 필드 확정은 tcxdict 오프셋 대조로만(0x58=attack i32). L469 소스가 attack 인지 다른 동명 필드인지 소스 표기는 미확정 | 3 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

