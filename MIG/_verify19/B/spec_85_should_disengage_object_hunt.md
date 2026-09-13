---

### `85` should_disengage_object_hunt — 오브젝트 사냥 중 캠프 범위 안의 (보이는) 적이 있고 진형이 불리하며 적 수 ≥ (나 제외·HP 40%↑) 아군 수면 true(Setup 복귀)

| 항목 | 값 |
|---|---|
| id | `fight_check__should_disengage_object_hunt` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai11fight_check28should_disengage_object_hunt` |
| 소스 | `game-ai\src\fight_check.rs:1361` |
| IR | `m15.ll` 33056~33780행 |
| 경로·가시성 | `game_ai::should_disengage_object_hunt` · **pub** |
| 계층 | 점수화·술어 |
| exe | `ebbb80` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, (u64, u64), u64) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize | 미사용(이름부터 _). 본문 분기 없음 | 4 |
| 1 | 2 | player | &PlayerState (2528B) | +0x930 team, +0x9c0 position. check_favorable_engage_formation 에 전달 | 4 |
| 2 | 3 | data | &OperationData (24B) | +0x0 cache 만 직접 읽음 | 4 |
| 3 | 4 | camp_pos | (u64, u64) | SROA 승격: %3 = x, %4 = y (스택 %11 에 저장 후 클로저가 & 로 참조). 오브젝트 캠프 좌표 | 4 |
| 4 | 5 | camp_range | u64 | %5 (스택 %10). camp_range² 와 제곱거리 비교(L1377 적 · L1402 아군) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
should_disengage_object_hunt(_version, player, data, camp_pos:(x,y), camp_range) -> bool
  me = cache.player_champion[team][pos]  ; None → return false                       // L1368
  enemies: Vec<&Entity> = cache.iter_champions(1-team).filter(|e|                   // L1374~1375 (aux closure#0)
      (me.team==Neutral || e.visible_state[my_team]==Visible)                         // L1376
      && distance_sq(e, camp_pos) <= camp_range*camp_range ).collect()               // L1377
  if enemies.is_empty() { return false }                                            // L1382
  nearest = enemies.iter().min_by_key(|e| distance_sq(e, me))  ; None → return false   // L1387~1391 (closure$1)
  if check_favorable_engage_formation(0, player, data, nearest, 200000) { return false }   // L1394  유리 구도면 유지
  allies = cache.player_champion[team][0..5].filter(|a| Some(a)                      // L1401~1405 (closure$2 인라인 ×5)
      && a.id != me.id                                                                // L1401 ★나 제외
      && distance_sq(a, camp_pos) <= camp_range²                                      // L1402
      && a.hp*100/a.max_hp > 39).count()                                              // L1403
  return enemies.len() >= allies   (IR: len < allies → false, else true)             // L1407~1410
```

**`mem` 메모리 접근 12건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | bounds<2 (panic @1368). 1-team = 적 팀 | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | as_index → me = player_champion[team][pos] | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion | r | [team][pos] = me (L1368, None → false). [1-team][0..5] = iter_champions(적) (L1374). [team][0..5] = 아군 5칸 스캔 (L1405) | 4 | OK |
| 4 | Entity | 0x0 | team@tag | r | (aux) me.team Neutral(1) 이면 가시성 검사 생략 | 4 | OK |
| 5 | Entity | 0x8 | team@Player.0 | r | (aux) me 팀 인덱스 | 4 | OK |
| 6 | Entity | 0x38 | visible_state | r | (aux) e.visible_state[my_team]@tag == 0 Visible 이어야 적 후보 | 4 | OK |
| 7 | Entity | 0x5c0 | id | r | L1401: 아군 슬롯 중 me.id 와 같은 것 제외 | 4 | OK |
| 8 | Entity | 0x628 | stat_cached.hp | r | L1403 아군 최대 HP(0 이면 div-by-zero 패닉 @1403) | 4 | OK |
| 9 | Entity | 0x660 | x | r | distance_sq(utils.rs:7~9): 적↔camp_pos(aux L1377) · 적↔me(L1389) · 아군↔camp_pos(L1402) | 4 | OK |
| 10 | Entity | 0x668 | y | r |  | 4 | OK |
| 11 | Entity | 0x670 | hp | r | L1403 아군 hp*100/max_hp > 39 | 4 | OK |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 1394 | 태그 | check_favorable_engage_formation 의 1번째 인자(version) 를 0 으로 고정 호출. L1382 enemies.len()==0 → false 도 리터럴 0 | 4 |
| 1 | 200000 | 1394 | 임계 | check_favorable_engage_formation 의 마지막 인자(거리 임계, 셀 32000 기준 6.25셀) — 유리 구도면 즉시 false | 4 |
| 2 | 100 | 1403 | 계수 | 아군 HP% = hp*100/max_hp | 4 |
| 3 | 39 | 1403 | 임계 | 아군 카운트 조건 HP% > 39 (40% 이상) | 4 |
| 4 | 2 | 1368 | 임계 | 팀 인덱스 bounds(panic_bounds_check) | 4 |
| 5 | 1152921504606846976 | 1382 | 임계 | 2^60 — Vec<&Entity>.len 상한 assume(컴파일러 아티팩트, 판정 아님) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 유리 구도 판정 거리 | fight_check.rs:1394 | 200000 | check_favorable_engage_formation 의 반경. 올리면 더 넓은 진형을 보고 유리 판정(→ 유지) 가능성 변화(콜리 의미에 따름) | 4 | 기존 |
| 1 | 아군 카운트 HP% 하한 | fight_check.rs:1403 | 39 | 내리면 빈사 아군도 세어 disengage 가 덜 일어난다 | 4 | 기존 |
| 2 | 수적 비교 극성 | fight_check.rs:1410 | enemies >= allies(나 제외) | `>` 로 바꾸면 동수에서 유지, 현재는 동수면 후퇴(나를 안 세므로 실질 아군 = allies+1 대 enemies) | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | check_favorable_engage_formation | game_ai::check_favorable_engage_formation | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64) -> bool | game-ai\src\fight_check.rs:1196 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | should_disengage_object_hunt | game_ai::should_disengage_object_hunt | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, (u64, u64), u64) -> bool | game-ai\src\fight_check.rs:1361 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 3개**: `allies`, `collect`, `from_iter`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m09.ll:31154, m09.ll:32067) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | check_favorable_engage_formation 내부·200000 의 정확한 의미(콜리 시그니처의 5번째 인자 이름) — 별도 함수(호출만). 첫 인자 0 이 version 인지도 콜리 미독 | 4 |  |
| 1 | 표기 불가 | L1410 소스 표기가 `enemies.len() >= allies` 인지 `!(enemies.len() < allies)` 인지 — 표기 불가(외연 동일), 동작은 확정 | 4 |  |
| 2 | 미탐색 | camp_pos 튜플이 (x,y) 순서인지 — 클로저 env +8→%11(=%3)·+16→%12(=%4) 가 각각 Entity.x(0x660)·y(0x668) 와 비교되므로 (x,y) 로 확정. 단 소스 인자 이름(camp_pos)은 arg 4 DI 가 누락돼 _docs 주석(`camp_pos`)으로 보완 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

