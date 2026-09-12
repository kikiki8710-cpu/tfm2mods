---

### `26` v23_objective_setup_pressure_line — 후보 라인 중 이미 강하게 밀린 라인을 빼고 (from_mid + lead*10000 + minion_count*1000) 최소 라인을 고른다

| 항목 | 값 |
|---|---|
| id | `objective_helpers__v23_objective_setup_pressure_line` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers33v23_objective_setup_pressure_line` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:73` |
| IR | `m15.ll` 55350~55545행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::v23_objective_setup_pressure_line` · **pub** |
| 계층 | 기타 |
| exe | `15509920` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, &[game_core::LineType]) -> std::option::Option<game_core::LineType>
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | 판단 주체. info.team(+0x930)만 읽는다 — blackboard·line_lead 의 팀 인덱스 | 4 |
| 1 | 2 | data | &OperationData(24B) | cache(+0x0)·context(+0x8)·blackboard(+0x10) 셋 다 읽는다 | 4 |
| 2 | 3 | lines | &[LineType] (ptr %2, len %3) | 후보 라인 슬라이스(원소 1B, LineType 태그 Top0/Mid1/Bottom2). 순서대로 순회 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v23_objective_setup_pressure_line(player, data, lines: &[LineType]) -> Option<LineType>

[L75] best: Option<(LineType, i64)> = None          // IR: i8 -1 / i64 0
      team = player.info.team(+0x930)
      for line in lines.iter().copied() {           // 슬라이스 순서대로
[L76]   if !rule_scope::line_exists(data.context(+0x8), line) { continue }
        //  line_exists(m13.ll:53138, rule_scope.rs:25) = context.tutorial(+0x38) 태그와 line 조합의 허용표(switch) — 맵/튜토리얼 종류에 따라 없는 라인을 거른다
[L80]   ms = &data.blackboard(+0x10)[team].{Top:top(+0x0) | Mid:mid(+0x28) | Bottom:bottom(+0x50)}_minion_state   // Blackboard::minion_state 인라인 (blackboard.rs:379~382)
        //  team >= 2 면 panic_bounds_check(len=2)
[L81]   lead = data.cache(+0x0).line_lead(line, team)    // = {top|mid|bottom}_lead[team] (+0x21c0 + line*16 + team*8)
[L82]   if lead > 2 && ms.from_mid(+0x10) > 1999 && ms.minion_count(+0x20) > 1 { continue }
        //  IR 분기 순서: lead>2 → from_mid>1999 → minion_count>1, 셋 다 참일 때만 skip(블록 %52)
        //  = '이미 충분히 밀어 놓은 라인'은 압박 후보에서 제외
[L86]   score = ms.from_mid + lead*10000 + (ms.minion_count as i64)*1000
[L87]   if best.is_none_or(|(_, s)| score < s) { best = Some((line, score)) }   // ★최솟값 선택(icmp slt), 동점이면 먼저 온 라인 유지
      }
[L93] return best.map(|(l, _)| l)                     // IR: best 의 LineType 바이트(-1=None) 그대로 반환

※ 부작용 없음. lines 가 비면 즉시 None.
```

**`mem` 메모리 접근 10건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. blackboard[team]·line_lead[team] 인덱스. >=2 이면 첫 존재 라인에서 panic_bounds_check(len=2) | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext → rule_scope::line_exists(context, line) 인자 | 4 | OK |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2](744B stride). [team] 의 minion_state 를 읽는다 | 4 | OK |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache → line_lead(simulation.rs:1929) 인라인 | 4 | OK |
| 4 | Blackboard | 0x0 | top_minion_state | r | BrainMinionParameter(40B). line==Top(0) 일 때 (blackboard.rs:379 minion_state 인라인, phi 기본값 %16) | 4 | OK |
| 5 | Blackboard | 0x28 | mid_minion_state | r | line==Mid(1) 일 때 (blackboard.rs:381, gep %16,40) | 4 | OK |
| 6 | Blackboard | 0x50 | bottom_minion_state | r | line==Bottom(2) 일 때 (blackboard.rs:382, gep %16,80) | 4 | OK |
| 7 | BrainMinionParameter | 0x10 | from_mid | r | i64. 제외 판정(>1999)과 score 의 첫 항 | 4 | OK |
| 8 | BrainMinionParameter | 0x20 | minion_count | r | i32. 제외 판정(>1)과 score 의 *1000 항. (_gcbc g11.ll BrainMinionParameter::update ai_interface.rs:47 = 아군 라인미니언 수 − 적 라인미니언 수) | 4 | OK |
| 9 | AbstractGameWithCache | 0x21c0 | top_lead / mid_lead / bottom_lead ([usize;2] × 3, 16B stride) | r | IR: cache + (line<<4) + 8640 + team*8. line_lead(simulation.rs:1929) 인라인 = {top,mid,bottom}_lead[team] | 4 | OK |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 80 | 임계 | panic_bounds_check 의 배열 길이(blackboard [Blackboard;2]) — team 인덱스 가드. 판정값 아님 | 4 |
| 1 | 0 | 80 | 태그 | LineType 태그 0 = Top → top_minion_state (switch i8 %27) | 4 |
| 2 | 1 | 80 | 태그 | LineType 태그 1 = Mid → mid_minion_state | 4 |
| 3 | 2 | 80 | 태그 | LineType 태그 2 = Bottom → bottom_minion_state | 4 |
| 4 | 2 | 82 | 임계 | 제외 조건 1: lead > 2 (icmp ugt). 라인 진출 단계가 3 이상 | 4 |
| 5 | 1999 | 82 | 임계 | 제외 조건 2: minion_state.from_mid > 1999 (icmp sgt, 즉 >= 2000). 미니언 전선이 중앙에서 충분히 전진 | 4 |
| 6 | 1 | 82 | 임계 | 제외 조건 3: minion_state.minion_count > 1 (i32 sgt). 아군 미니언 수 우위 2 이상 | 4 |
| 7 | 10000 | 86 | 계수 | score = from_mid + lead*10000 + minion_count*1000 — lead 가중치 | 4 |
| 8 | 1000 | 86 | 미상 | score 의 minion_count 가중치 | 4 |
| 9 | -1 | 87 | 센티널 | Option<(LineType,i64)> 의 None 니치(LineType 바이트 = 0xFF). best.is_none_or(\|b\| score < b.1) 의 is_none 판정 및 반환 None | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 제외 조건 — 라인 진출 단계 | objective_helpers.rs:82 (IR m15.ll `icmp ugt i64 %39, 2`) | 2 | lead 가 이 값을 넘어야(3+) 제외 후보. 내리면 덜 밀린 라인도 '이미 밀렸다'고 보아 제외가 늘고, 올리면 거의 제외되지 않는다 | 4 | 기존 |
| 1 | 제외 조건 — 미니언 전선 전진 임계 | objective_helpers.rs:82 (IR `icmp sgt i64 %46, 1999`) | 1999 | from_mid 가 2000 이상이어야 제외. 올리면 더 깊이 밀린 라인만 제외되어 압박 후보가 늘어난다 | 4 | 기존 |
| 2 | 제외 조건 — 미니언 수 우위 임계 | objective_helpers.rs:82 (IR `icmp sgt i32 %50, 1`) | 1 | 아군−적 미니언 수가 2 이상이어야 제외 | 4 | 기존 |
| 3 | score 의 lead 가중치 | objective_helpers.rs:86 (IR `mul i64 %39, 10000`) | 10000 | lead 1단계 = from_mid 10000 만큼. 올리면 라인 선택이 lead 에 더 지배되고, 내리면 미니언 전선 위치가 더 중요해진다 | 4 | 기존 |
| 4 | score 의 minion_count 가중치 | objective_helpers.rs:86 (IR `mul nsw i64 %60, 1000`) | 1000 | 미니언 수 차이 1 = from_mid 1000 만큼. 최솟값 선택이므로 아군 미니언이 적을수록(음수) 그 라인이 선택되기 쉽다 | 4 | 기존 |

<details><summary>`callees` 피호출자 7건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | line_lead | game_core::AbstractGameWithCache::<'a, 'b>::line_lead | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, game_core::LineType) -> usize | game-core\src\simulation.rs:1928 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 6 | v23_objective_setup_pressure_line | game_ai::plan_legacy::team_plan::v23_objective_setup_pressure_line | pub | fn(&game_core::PlayerState, &game_core::OperationData, &[game_core::LineType]) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:73 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 6개**: `bottom`, `cache`, `from_mid`, `is_none_or`, `minion_count`, `skip`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m05.ll:51946, m09.ll:19896, m09.ll:21945, m09.ll:22168, m09.ll:62541) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | line_lead(simulation.rs:1929) 가 돌려주는 {top,mid,bottom}_lead[team](usize) 의 정확한 의미 — _gcbc g15.ll:103500~ 부근에서 region_point(27개 i32, >2 비교)를 순차 검사해 세는 것까지만 봤고 '라인 진출 단계 수(0~N)'는 추정. _docs 에 lead 주석 없음 | 4 |  |
| 1 | 미탐색 | BrainMinionParameter.from_mid 의 정확한 정의 — _gcbc g11.ll:49926 BrainMinionParameter::update(ai_interface.rs:117/133/154/162)에서 (…)/2, (20000−x)/2, (−10000−x)/2, 0 으로 저장되는 것까지만 확인. 부호 방향(양수=적 쪽으로 전진?)은 추정이며 미확인 | 4 |  |
| 2 | 표기 불가 | 반환값이 Option<LineType> 이라는 것은 IR range(i8 -1,3) 과 best 초기값 -1 로 확정. 소스에서 `.map(\|b\| b.0)` 인지 다른 표기인지는 표기 불가(column 부재) | 4 |  |
| 3 | 표기 불가 | L82 조건 셋이 하나의 `&&` 식인지 중첩 if 인지는 표기 불가 — 평가 순서(lead → from_mid → minion_count)와 동작은 확정 | 4 |  |
| 4 | 미탐색 | team >= 2 경로(%70~%79): line_exists 가 참인 첫 라인에서 panic. 실전에서 team 은 0/1 이므로 도달 불가 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

