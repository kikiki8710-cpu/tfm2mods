---

### `136` v23_should_break_objective_hunt_anchor — 오브젝트 사냥 앵커를 풀어야 하는가 — 오브젝트 HP<21% 면 유지, 아니면 내 주변/캠프 주변 적 우세로 판정

| 항목 | 값 |
|---|---|
| id | `objective_helpers__v23_should_break_objective_hunt_anchor` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers38v23_should_break_objective_hunt_anchor` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:59` |
| IR | `m15.ll` 56682~56738행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::v23_should_break_objective_hunt_anchor` · **pub** |
| 계층 | 기타 |
| exe | `ecb300` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, (u64, u64)) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | 본문에서 직접 읽지 않음 — 헬퍼 2종에 그대로 전달(m15.ll:56711·56713·56725·56727) | 4 |
| 1 | 2 | data | &OperationData(24B: 0x0 cache/0x8 context/0x10 blackboard) | 본문에서 직접 읽지 않음 — 헬퍼에 전달 | 4 |
| 2 | 3 | champ | &Entity(1728B) | 내 챔피언. x/y 만 읽음(56707~56710) | 4 |
| 3 | 4 | objective | &Entity(1728B) | 사냥 대상 오브젝트. stat_cached.hp(max)·hp 만 읽음(56689~56696) | 4 |
| 4 | 5 | camp_pos | (u64,u64) | IR 에선 %4=x, %5=y 두 스칼라(#dbg_value fragment 0..8 / 8..16, 56687~56688). 오브젝트 캠프 좌표 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v23_should_break_objective_hunt_anchor(player, data, champ, objective, camp_pos:(x,y)) -> bool
// L60
max = objective.stat_cached.hp (+0x628); hp = objective.hp (+0x670)
if max == 0 { panic(div by zero) }            // 56691~56703, 가드 없음
if hp*100/max < 21 { return false }           // 오브젝트가 거의 죽음(<=20%) → 앵커 유지
// L64~66 개인 주변
personal_allies  = v23_healthy_allies_near_point(player, data, champ.x, champ.y, 180000, 40)
personal_enemies = v23_recent_visible_enemies_near_point(player, data, champ.x, champ.y, 160000, 40)
if personal_enemies > 1 && personal_enemies > personal_allies { return true }   // 56715~56718
// L70 → v23_visible_objective_overload(player, data, camp_pos) 인라인 (objective_helpers.rs:54~56)
allies  = v23_healthy_allies_near_point(player, data, camp_x, camp_y, 180000, 40)
enemies = v23_recent_visible_enemies_near_point(player, data, camp_x, camp_y, 180000, 40)
return enemies > 2 && enemies >= allies + 2   // 56729~56732 select(%30, %32, false)

분기 순서(줄번호 확정): L60 저혈 조기 false → L66 개인 우세열세 true → L70 캠프 과부하. 헬퍼 반환은 range(i64 0,6) = 0..=5 명 계수.
```

**`mem` 메모리 접근 4건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Entity(objective) | 0x628 | stat_cached.hp | r | 최대 HP. 0 이면 udiv 0 패닉(panic_const_div_by_zero, 56703) — 가드 없음. objective_helpers.rs:60 | 4 | OK |
| 1 | Entity(objective) | 0x670 | hp | r | 현재 HP. hp*100/max_hp 로 % 계산. objective_helpers.rs:60 | 4 | OK |
| 2 | Entity(champ) | 0x660 | x | r | 내 좌표 x — 개인 반경 판정 중심. objective_helpers.rs:64 | 4 | OK |
| 3 | Entity(champ) | 0x668 | y | r | 내 좌표 y. objective_helpers.rs:64 | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 100 | 60 | 계수 | hp*100/max_hp — 백분율 환산 | 4 |
| 1 | 21 | 60 | 임계 | 오브젝트 HP% 임계: `icmp ult %pct, 21` 이 true(=HP%<=20) 면 즉시 false(앵커 유지). 소스는 `< 21` 또는 `<= 20` — 외연 동일(표기 불가) | 4 |
| 2 | 180000 | 64 | 계수 | 내 주변 건강한 아군 계수 반경(v23_healthy_allies_near_point 의 radius 인자). 캠프 주변(L54·55, v23_visible_objective_overload 인라인) 에서도 아군·적 둘 다 180000 | 4 |
| 3 | 160000 | 65 | 계수 | 내 주변 최근가시 적 계수 반경(v23_recent_visible_enemies_near_point radius). 아군 180000 보다 좁음 | 4 |
| 4 | 40 | 64 | 임계 | 헬퍼 4종 호출 전부의 마지막 인자(usize). 헬퍼 시그니처 fn(&PlayerState,&OperationData,u64,u64,u64,usize)->usize 의 6번째 — 이름상 '건강' 기준(HP% 하한 40)으로 추정, 헬퍼 내부는 이 배치 범위 밖 | 5 |
| 5 | 1 | 66 | 임계 | 개인 판정: personal_enemies > 1 (적 2명 이상) AND personal_enemies > personal_allies → true | 4 |
| 6 | 2 | 56 | 임계 | 캠프 과부하(v23_visible_objective_overload 인라인, L56<70): enemies > 2 (적 3명 이상) AND enemies >= allies + 2 → true. `add nuw nsw %allies, 2`(56730) 의 2 와 `ugt %enemies, 2`(56729) 의 2 둘 다 | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 오브젝트 저혈 앵커 유지 임계(HP%) | objective_helpers.rs:60 (m15.ll:56699) | 21 | 올리면 오브젝트 HP 가 더 높을 때부터 '끝까지 친다'(적이 몰려도 앵커 유지) — 내리면 더 낮은 HP 에서만 유지 | 4 | 기존 |
| 1 | 개인 주변 아군 계수 반경 | objective_helpers.rs:64 (m15.ll:56711) | 180000 | 올리면 먼 아군도 세어 personal_allies 가 커져 앵커를 덜 깬다 | 4 | 기존 |
| 2 | 개인 주변 적 계수 반경 | objective_helpers.rs:65 (m15.ll:56713) | 160000 | 올리면 더 먼 적까지 세어 앵커를 더 자주 깬다 | 4 | 기존 |
| 3 | 개인 판정 적 최소 수 | objective_helpers.rs:66 (m15.ll:56715) | 1 | 적 > 1 (2명 이상). 0 으로 내리면 적 1명이 아군보다 많아도(내가 혼자면 아군 계수 포함 여부에 따라) 깸 | 4 | 기존 |
| 4 | 캠프 과부하 적 최소 수 / 아군 대비 마진 | objective_helpers.rs:56 (m15.ll:56729~56731) | 2 | enemies>2 && enemies>=allies+2. 마진을 내리면 캠프 주변 소폭 열세에도 앵커를 깬다 | 4 | 기존 |
| 5 | 헬퍼 HP% 하한(추정) | objective_helpers.rs:54·55·64·65 (마지막 인자 40) | 40 | 추정: 올리면 '건강한' 아군/적으로 세는 기준이 엄격해져 계수가 줄어든다 — 헬퍼 내부 미독 | 5 | 기존 |

<details><summary>`callees` 피호출자 5건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 1 | v23_healthy_allies_near_point | game_ai::plan_legacy::team_plan::v23_healthy_allies_near_point | pub | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | v23_recent_visible_enemies_near_point | game_ai::plan_legacy::team_plan::v23_recent_visible_enemies_near_point | pub | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:31 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | v23_should_break_objective_hunt_anchor | game_ai::plan_legacy::team_plan::v23_should_break_objective_hunt_anchor | pub | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, (u64, u64)) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:59 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 4 | v23_visible_objective_overload | game_ai::plan_legacy::team_plan::v23_visible_objective_overload | pub | fn(&game_core::PlayerState, &game_core::OperationData, (u64, u64)) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:53 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m02.ll:12860, m02.ll:13748, m15.ll:14544, m15.ll:15187) · **형제 0개** 

**`open` 2건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L60 `< 21` vs `<= 20` 표기 — 외연 동일(표기 불가) | 4 |  |
| 1 | 미탐색 | player·data 는 본문에서 직접 읽지 않아 이 함수 자체의 reads 에 없음(헬퍼 경유) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | 헬퍼 v23_healthy_allies_near_point / v23_recent_visible_enemies_near_point 의 6번째 인자 40 의 정확한 의미(HP% 하한으로 추정) — 헬퍼 본체(m15.ll:53341·55548)는 이 배치 범위 밖(시그니처만 기록). 반환 range(i64 0,6) 으로 계수(0..=5)임은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

