---

### `64` v25_objective_splitter_can_stay — 에픽 오브젝트 타임에 스플리터(반대편 라인 담당)가 그 라인에 남아도 되는가 — 먼 라인 근처·오브젝트와 ≥250000·적 압박 없음·아군 인원 충족

| 항목 | 값 |
|---|---|
| id | `objective_helpers__v25_objective_splitter_can_stay` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers31v25_objective_splitter_can_stay` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:143` |
| IR | `m15.ll` 54635~54772행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::v25_objective_splitter_can_stay` · **pub** |
| 계층 | 기타 |
| exe | `eca200` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문에서 읽지 않음(분기 없음) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0) 읽음 | 4 |
| 2 | 3 | data | &OperationData(24B) | +0 cache(player_champion) / +8 context(map·tutorial) | 4 |
| 3 | 4 | target | JungleType(i8, range 0..6) | 4=Morgard→Bottom 라인, 5=Serpen→Top 라인, 그 외 즉시 false | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v25_objective_splitter_can_stay(version, player, data, target: JungleType) -> bool {
  let ctx = data.context;
  // 145: 오브젝트 반대편 '먼 라인' + 그 라인이 이 판에 존재하는가
  let far = v25_objective_far_line(target);        // 95~96 인라인 (m15.ll:54637~54640): Morgard(4)→Some(Bottom=2), Serpen(5)→Some(Top=0), 그 외→None(-1 니치)
  let Some(line) = far.filter(|&l| rule_scope::line_exists(ctx, l)) else { return false };   // 145 closure$0 = line_exists 호출(m15.ll:54646~54647). target<4 → None → false(m15.ll:54643~54644)
  // 148: 내 챔피언 엔티티
  let team = player.info.team;                                                       // <2 아니면 panic_bounds_check
  let Some(me) = data.cache.player_champion[team][player.info.position.as_index()] else { return false };   // 148 (m15.ll:54662~54670)
  // 151: 오브젝트 위치
  let (ox, oy) = ctx.map.camp_pos(target, team == 0);                                // 151
  // 152: 내가 그 먼 라인 근처인가
  if !map_regions::is_near_line(ctx, me.x, me.y, line) { return false }             // 152 (m15.ll:54677~54678)
  // 153: 오브젝트에서 충분히 먼가
  if distance_sq(me.x, me.y, ox, oy) < 62500000000 { return false }                 // 153 — utils.rs:7~9 인라인, 250000² (m15.ll:54680~54693)
  // 154: 먼 스플릿에 대한 압박 조건(별도 함수)
  if !v25_objective_far_split_pressure(player, data, target) { return false }        // 154 (m15.ll:54696~54697)
  // 158~162: 오브젝트 주변 인원
  let allies  = v23_healthy_allies_near_point(player, data, ox, oy, 180000, 40);         // 158
  let enemies = v23_recent_visible_enemies_near_point(player, data, ox, oy, 180000, 40); // 159
  let need = /* player_count 인라인(보정 접힘 추정) */ match ctx.tutorial {   // 160 — runner.rs:295~301 인라인 switch (m15.ll:54741~54762)
    None(0) | Line(7) | Total(8) => 4,  First(1) | Bottom(3) => 2,  TopSolo(2) | MidSolo(4) | JungleOnly(6) => 1,  MidBottom(5) => 3 };
  enemies == 0 && allies >= need                                                     // 162 (m15.ll:54763~54765: `%57 uge %66` and `%58 eq 0`)
}

극성 근거: 각 조건은 실패 시 %70 로 가는 phi 가 전부 false 이고, 유일한 비상수 진입 = %65 의 %69(m15.ll:54768).
```

**`mem` 메모리 접근 9건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 2 | GameContext | 0x20 | map | r | &MapDef → camp_pos | 4 | OK |
| 3 | GameContext | 0x38 | tutorial | r | TutorialType(i8) — player_count(runner.rs:295) 인라인 switch 의 입력 | 4 | OK |
| 4 | PlayerState | 0x930 | info.team | r | <2 bounds check; ==0 → camp_pos is_blue_side | 4 | OK |
| 5 | PlayerState | 0x9c0 | info.position@tag | r | i32 태그 → as_index(entity.rs:581) zext → player_champion[team][pos] | 4 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[0][0] | r | [[Option<&Entity>;5];2]; [team][pos] = 내 챔피언. None(null) → false | 4 | OK |
| 7 | Entity | 0x660 | x | r | 내 챔피언 위치 x | 4 | OK |
| 8 | Entity | 0x668 | y | r | 내 챔피언 위치 y | 4 | OK |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 145 | 태그 | JungleType::Morgard 태그(4) → far_line = Some(Bottom); 또한 `target <u 4` → 에픽 아님 → false. (또한 player_count 결과 4: tutorial None0/Line7/Total8) | 4 |
| 1 | 5 | 145 | 태그 | JungleType::Serpen 태그(5) → far_line = Some(Top). (또한 TutorialType::MidBottom 태그 5 → player_count 3) | 4 |
| 2 | 2 | 96 | 태그 | LineType::Bottom 메모리태그(tcxdict --enum LineType: Top0/Mid1/Bottom2, Direct). v25_objective_far_line(objective_helpers.rs:95~96) 인라인: Morgard→Bottom. (또한 team bounds check 상한 2·player_count 2) | 3 |
| 3 | 62500000000 | 153 | 임계 | 250000² — 내 위치와 오브젝트(camp_pos) 의 제곱거리 하한. 이보다 가까우면(<) false = 오브젝트에 이미 가까우면 남을 이유 없음 | 4 |
| 4 | 180000 | 158 | 미상 | 오브젝트 주변 인원 셈 반경(range). 158·159 동일 | 4 |
| 5 | 40 | 158 | 임계 | min_hp_ratio 인자(HP% 하한). 158·159 동일 | 4 |
| 6 | 3 | 160 | 태그 | player_count 인라인 결과: tutorial MidBottom(5) → 3 | 4 |
| 7 | 1 | 160 | 태그 | player_count 인라인 결과: tutorial TopSolo2/MidSolo4/JungleOnly6 → 1. (First1/Bottom3 → 2) | 4 |
| 8 | 0 | 162 | 태그 | enemies == 0 비교값 (또한 LineType::Top 태그 0 = Serpen 의 far_line, TutorialType::None 0) | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 오브젝트와의 최소 제곱거리 | objective_helpers.rs:153 | 62500000000 | 내리면(예 200000²) 오브젝트에 더 가까운 스플리터도 라인에 남게 판정, 올리면 더 멀리 있어야 남음 | 4 | 기존 |
| 1 | 인원 셈 반경 | objective_helpers.rs:158~159 | 180000 | 올리면 오브젝트에서 더 먼 아군·적까지 셈에 포함 — 적 셈이 늘면 enemies==0 을 만족하기 어려워져 남기 판정이 줄어듦 | 4 | 기존 |
| 2 | 건강 판정 HP% 하한 | objective_helpers.rs:158~159 | 40 | 올리면 저체력 아군이 셈에서 빠져 allies>=need 를 만족하기 어려워짐 | 4 | 기존 |
| 3 | 필요 아군 수(튜토리얼별 상수표) | objective_helpers.rs:160 (runner.rs:295~301 player_count 인라인) | 4 | 일반 경기(None) 는 4 — 오브젝트 주변에 건강한 아군 4명(=스플리터 제외 전원 추정)이 있어야 남을 수 있다. 내리면 더 적은 인원으로도 남음 | 5 | 기존 |

<details><summary>`callees` 피호출자 12건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 5 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | v23_healthy_allies_near_point | game_ai::plan_legacy::team_plan::v23_healthy_allies_near_point | pub | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | v23_recent_visible_enemies_near_point | game_ai::plan_legacy::team_plan::v23_recent_visible_enemies_near_point | pub | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:31 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | v25_objective_far_line | game_ai::plan_legacy::team_plan::v25_objective_far_line | pub | fn(game_core::JungleType) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:95 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 10 | v25_objective_far_split_pressure | game_ai::plan_legacy::team_plan::v25_objective_far_split_pressure | pub | fn(&game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:103 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | v25_objective_splitter_can_stay | game_ai::plan_legacy::team_plan::v25_objective_splitter_can_stay | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:143 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

**호출처 2곳** (m10.ll:9242, m10.ll:11377) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | 160 의 소스가 `player_count()` 그대로인지 `player_count() - 1` 이 접힌 것인지 — 상수접힘으로 표기 불가. 관측: IR phi = None→4·First→2·TopSolo→1·Bottom→2·MidSolo→1·MidBottom→3·JungleOnly→1·Line→4·Total→4. rmeta 주석 `player_count: 팀 선수 수`(game_core.txt:462) 와 일반경기 5명을 맞추면 `-1`(자기 자신 제외) 접힘이 유력(추정). m09.ll:10566 다른 사이트는 {0,7,8}→3 이라 사이트마다 보정치가 다름 | 4 |  |
| 1 | 표기 불가 | 162 의 `enemies == 0 && allies >= need` 두 항의 소스 순서 — 같은 줄·둘 다 순수식이라 표기 불가 | 4 |  |
| 2 | 미탐색 | 153 의 비교 표기(`< 62500000000` 인지 `<= 62499999999`·`< 250000*250000` 인지) — 외연 동일 | 4 |  |
| 3 | 미탐색 | line_exists(m13.ll:53138)·is_near_line(_gcbc g09.ll)·v25_objective_far_split_pressure·v23_*_near_point 내부 — 담당 범위 밖, 미독해 | 4 |  |
| 4 | 미탐색 | version(p1) 의 용도 — 본문에서 안 읽음 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

