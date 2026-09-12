---

### `39` is_wave_priority_start_line — 그 라인에 적 미니언이 9+ 이고 그중 하나라도 우리 1차 타워(또는 그 기본 위치)보다 넥서스에 가까운가

| 항목 | 값 |
|---|---|
| id | `objective_helpers__is_wave_priority_start_line` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers27is_wave_priority_start_line` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:221` |
| IR | `m15.ll` 52851~53084행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::objective_helpers::is_wave_priority_start_line` · **in:game_ai** |
| 계층 | 기타 |
| exe | **없음** — 「exe 에 독립 함수가 없다(인라인·`define internal fastcc`)」인지 **「조인 실패」**인지는 이 칸만으로 못 가른다. `dllmatch.py`·`name2rva.py` 로 확인하라 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> bool
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽는다 | 4 |
| 1 | 2 | data | &OperationData(24B) | cache(+0x0) 만 읽는다 | 4 |
| 2 | 3 | line | LineType(i8: 0 Top/1 Mid/2 Bottom) | 검사할 라인. 그 외 값은 unreachable | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn is_wave_priority_start_line(player, data, line) -> bool

[L222] team = player.info.team(+0x930); enemy = 1 - team
       line_minions = match line { Top => cache.top_minions(+0x10), Mid => mid_minions(+0x50), Bottom => bottom_minions(+0x90) }
       enemy_count = line_minions[enemy].len(+0x18)
[L223] return enemy_count > 8 && is_enemy_wave_inside_ally_first_tower(player, data, line)

// ── is_enemy_wave_inside_ally_first_tower (objective_helpers.rs:227~234, 인라인) ──
[L227] let Some(nexus) = cache.nexus(+0x170)[team] else { return false }
[L228] tower = cache.first_tower(line, team)        // [top|mid|bottom]_tower[team] (0x180 + line*32 + team*8)
[L229] (tx, ty) = tower.map(|t| (t.x(+0x660), t.y(+0x668)))
[L230]   .unwrap_or_else(|| first_tower_position(line, team))   // 238~241 → TowerType::get_position 인라인:
           //   Top:    team0 (48000, 272000)  / team1 (272000, 48000)
           //   Mid:    team0 (368000, 592000) / team1 (592000, 368000)
           //   Bottom: team0 (688000, 912000) / team1 (912000, 688000)
[L231] tower_distance = utils::distance(tx, ty, nexus.x, nexus.y)   // 1차 타워 ↔ 우리 넥서스 거리
[L233] line_minions[enemy].iter()
[L234]   .any(|m| Entity::distance(m, nexus) < tower_distance)     // 타워보다 넥서스에 가까운 적 미니언이 있는가

※ 부작용 없음. 판정은 '큰 웨이브(9+)' 와 '타워 안쪽 침투' 둘 다 필요(AND, 단락 평가 — 9 이하면 넥서스/타워는 안 본다).
```

**`mem` 메모리 접근 11건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 1 | PlayerState | 0x930 | info.team | r | usize. enemy = 1 - team. 둘 다 >=2 면 panic_bounds_check | 4 | OK |
| 2 | AbstractGameWithCache | 0x10 | top_minions[team] | r | [bumpalo Vec<&Entity>;2] 32B stride. line==Top 일 때 [enemy] 의 len(+0x18)·ptr(+0x0) | 4 | OK |
| 3 | AbstractGameWithCache | 0x50 | mid_minions[team] | r | line==Mid | 4 | OK |
| 4 | AbstractGameWithCache | 0x90 | bottom_minions[team] | r | line==Bottom | 4 | OK |
| 5 | Vec<&Entity>(bumpalo,32B) line_minions[enemy] | 0x18 | len | r | enemy_count. <=8 이면 false | 4 | 확인불가(tcx 사전에 타입 없음) |
| 6 | Vec<&Entity>(bumpalo,32B) line_minions[enemy] | 0x0 | ptr | r | L233 순회 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 7 | AbstractGameWithCache | 0x170 | nexus[team] | r | [Option<&Entity>;2]. None → false | 4 | OK |
| 8 | AbstractGameWithCache | 0x180 | top_tower[team] / mid_tower(+0x1a0) / bottom_tower(+0x1c0) | r | AbstractGameWithCache::first_tower(line, team) 인라인(simulation.rs:1815): 0x180 + line*32 + team*8. None 이면 기본 좌표 사용 | 4 | OK |
| 9 | Entity(1차 타워 / 넥서스) | 0x660 | x | r | utils::distance 인자 | 4 | OK |
| 10 | Entity(1차 타워 / 넥서스) | 0x668 | y | r |  | 4 | OK |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 222 | 태그 | enemy = 1 - player.info.team | 4 |
| 1 | 2 | 222 | 임계 | team/enemy 인덱스 상한(panic_bounds_check len=2). (본문의 `shl nuw nsw i8 %2, 5` 는 레지스터 %2=line 을 32배(first_tower 배열 stride)하는 것이라 이 상수와 무관) | 4 |
| 2 | 8 | 223 | 임계 | enemy_count > 8 (적 미니언 9마리 이상) 이어야 후속 검사 (`icmp ugt i64 %21, 8`) | 4 |
| 3 | 0 | 230 | 태그 | first_tower_position: team == 0 이면 (x,y) 를 뒤집는다(TowerType::get_position 인라인, tower.rs:105/110/115) | 4 |
| 4 | 48000 | 239 | 산출값 | Top 1차 타워 기본 좌표: team0 (48000, 272000) / team1 (272000, 48000) — 타워가 파괴돼 None 일 때만 | 4 |
| 5 | 272000 | 239 | 산출값 | Top 기본 좌표 성분 | 4 |
| 6 | 368000 | 240 | 산출값 | Mid 1차 타워 기본 좌표: team0 (368000, 592000) / team1 (592000, 368000) | 4 |
| 7 | 592000 | 240 | 산출값 | Mid 기본 좌표 성분 | 4 |
| 8 | 688000 | 241 | 산출값 | Bottom 1차 타워 기본 좌표: team0 (688000, 912000) / team1 (912000, 688000) | 4 |
| 9 | 912000 | 241 | 산출값 | Bottom 기본 좌표 성분 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 웨이브 크기 임계 | objective_helpers.rs:223 (IR m15.ll:52887 `icmp ugt i64 %21, 8`) | 8 | 적 미니언이 9마리 이상일 때만 웨이브 우선. 내리면(예: 5) 작은 웨이브 침투에도 라인이 우선 처리되고, 올리면 큰 웨이브만 | 4 | 기존 |
| 1 | 침투 기준선 = 1차 타워 위치(파괴 시 기본 좌표) | objective_helpers.rs:228~231 | first_tower / first_tower_position | 타워가 없어도 원래 타워 자리를 기준으로 본다. 2차 타워로 바꾸면 더 깊이 들어온 웨이브만 잡는다 | 4 | 기존 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | first_tower | game_core::AbstractGameWithCache::<'a, 'b>::first_tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1814 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | first_tower_position | game_ai::plan_legacy::team_plan::first_tower_position | pub | fn(game_core::LineType, usize) -> (u64, u64) | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:237 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 4 | is_enemy_wave_inside_ally_first_tower | game_ai::plan_legacy::team_plan::objective_helpers::is_enemy_wave_inside_ally_first_tower | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:226 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 5 | is_wave_priority_start_line | game_ai::plan_legacy::team_plan::objective_helpers::is_wave_priority_start_line | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:221 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 6 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
</details>

⚠**미매칭 6개**: `bottom_minions`, `mid_minions`, `team0`, `team1`, `top_minions`, `unwrap_or_else`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m11.ll:14682, m15.ll:58634) · **형제 0개** 

**`open` 2건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | utils::distance / Entity::distance 의 정의(정수 sqrt 유클리드 여부)는 game_core 미독 — 두 호출이 같은 단위(실거리)라 비교 자체는 정합 | 4 |  |
| 1 | 미탐색 | top/mid/bottom_minions[team] 벡터가 어느 시점의 스냅샷인지(가시성 필터 여부)는 AbstractGameWithCache 생성부 미독 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L230 unwrap_or_else 의 closure#1 은 인라인돼 별도 define 없음 — 좌표는 본문 select 로 직접 확정(team==0 이면 (48000,272000) 순, 아니면 뒤집힘) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

