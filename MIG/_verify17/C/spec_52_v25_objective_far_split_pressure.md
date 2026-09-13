---

### `52` v25_objective_far_split_pressure — 에픽 목표(Morgard/Serpen)의 반대편 '먼 라인'에 아군 스플릿이 있고 그 라인이 밀리고 있으면 true(스플릿 압박 중)

| 항목 | 값 |
|---|---|
| id | `objective_helpers__v25_objective_far_split_pressure` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers32v25_objective_far_split_pressure` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:103` |
| IR | `m15.ll` 54775~55347행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::v25_objective_far_split_pressure` · **pub** |
| 계층 | 기타 |
| exe | `eca430` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽는다 | 4 |
| 1 | 2 | data | &OperationData(24B) | +0 cache / +8 context / +0x10 blackboard | 4 |
| 2 | 3 | target | JungleType(i8) | 목표 캠프. 4 Morgard → 먼 라인 Bottom(2), 5 Serpen → Top(0), 그 외(<4) → 즉시 false (v25_objective_far_line 인라인, objective_helpers.rs:96) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v25_objective_far_split_pressure(player, data, target: JungleType) -> bool

[L104] far_line = v25_objective_far_line(target)          // 인라인(objective_helpers.rs:96): Serpen(5)→Top(0), Morgard(4)→Bottom(2), else None
       far_line = far_line.filter(|l| rule_scope::line_exists(data.context, *l))
       if far_line.is_none() { return false }              // target<4 → 54784 즉시 false ; line_exists false → false
[L108] team = player.info.team ; camp = MapDef::camp_pos(map, target, team==0)
[L109~115] far_count = cache.player_champion[team].iter_champions().filter(|a|
[L111]        v23_healthy(a)                                  // a.hp*100/a.max_hp > 39
[L112]     && map_regions::is_near_line(context, a.x, a.y, far_line)
[L113]     && distance_sq(a, camp) > 62499999999             // 캠프에서 250000 이상 떨어짐
           ).count()
[L116] if far_count == 0 { return false }
[L120] ms = data.blackboard[team].minion_state(far_line)      // Top→+0x0, Bottom→+0x50 (Mid 는 올 수 없음)
[L121] if cache.{top,bottom}_lead[team](+0x21c0 + 16*far_line + 8*team) > 1 { return true }
[L122] if ms.from_mid > 2999 { return true }
[L123] if ms.from_mid > 999 { return ms.minion_count > 1 }
[L124] return ms.minion_count > 3

※ 부작용 없음. far_count 의 실제 값은 0 여부만 쓰인다(1명이어도 통과).
```

**`mem` 메모리 접근 14건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache — player_champion·top/bottom_lead | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext — line_exists·is_near_line 인자, map | 4 | OK |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. [team].minion_state(far_line) | 4 | OK |
| 3 | GameContext | 0x20 | map | r | &MapDef — camp_pos(map, target, team==0) | 4 | OK |
| 4 | PlayerState | 0x930 | info.team | r | usize. camp_pos side·player_champion/blackboard/lead 인덱스(≥2 면 panic_bounds_check) | 4 | OK |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion | r | [Option<&Entity>;5] — 5슬롯 언롤 카운트 | 4 | OK |
| 6 | AbstractGameWithCache | 0x21c0 | top_lead[team] / bottom_lead[team] | r | cache + 0x21c0 + 16*far_line + 8*team (55311~55316). usize > 1 → true | 4 | OK |
| 7 | Blackboard | 0x0 | top_minion_state (far_line Top) / +0x50 bottom_minion_state (far_line Bottom) | r | Blackboard::minion_state 인라인(blackboard.rs:379) — far_line==0 ? +0 : +80 | 4 | OK |
| 8 | BrainMinionParameter | 0x10 | from_mid | r | i64. > 2999 → true / > 999 → count>1 / 그 외 → count>3 | 4 | OK |
| 9 | BrainMinionParameter | 0x20 | minion_count | r | i32 | 4 | OK |
| 10 | Entity | 0x670 | hp | r | v23_healthy(objective_helpers.rs:12): hp*100/max_hp > 39 | 4 | OK |
| 11 | Entity | 0x628 | stat_cached.hp | r | max_hp. 0 → div_by_zero 패닉 | 4 | OK |
| 12 | Entity | 0x660 | x | r | u64 — is_near_line 인자·camp 거리 | 4 | OK |
| 13 | Entity | 0x668 | y | r | u64 | 4 | OK |

**`consts` 상수 11건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 5 | 104 | 태그 | v25_objective_far_line 인라인: target == 5(Serpen) → far_line = Top(0) | 4 |
| 1 | 4 | 104 | 임계 | target == 4(Morgard) → far_line = Bottom(2). target < 4 (일반 정글캠프) → far_line None → 즉시 false. (본문의 `shl i8 %7, 4` 는 lead 배열 stride 16 의 접힘이지 이 상수와 무관 — 규칙대로 stride 는 미등록) | 4 |
| 2 | 2 | 104 | 태그 | LineType 태그 2 = Bottom (Morgard 의 먼 라인) | 4 |
| 3 | 39 | 111 | 임계 | v23_healthy: 아군 hp% > 39 (≥40%) 여야 스플릿 인원으로 센다 | 4 |
| 4 | 100 | 111 | 계수 | hp*100/max_hp 백분율 | 4 |
| 5 | 62499999999 | 113 | 임계 | 250000² - 1 — 아군이 목표 캠프에서 dist² > 이 값(즉 ≥250000, ≈7.8칸) 만큼 떨어져 있어야 '먼 라인 스플릿' | 4 |
| 6 | 0 | 116 | 태그 | far_count == 0 → false (먼 라인에 건강한 아군 없음) | 4 |
| 7 | 1 | 121 | 임계 | lead[team][far_line] > 1 (타워 리드 2 이상) → true | 4 |
| 8 | 2999 | 122 | 임계 | from_mid > 2999 → true (웨이브가 적 쪽으로 깊게 밀림) | 4 |
| 9 | 999 | 123 | 임계 | from_mid > 999 이면 minion_count > 1 로 판정, 아니면 (L124) minion_count > 3 | 4 |
| 10 | 3 | 124 | 임계 | 웨이브가 안 밀렸을 때(from_mid ≤ 999) 필요한 미니언 우세 minion_count > 3 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 스플릿 인원 HP 하한 | objective_helpers.rs:111 ← :12 v23_healthy (IR m15.ll:54910 `icmp ugt i64 %37, 39`) | 39 | HP ≥40% 아군만 스플릿으로 인정. 내리면 빈사 아군의 스플릿도 압박으로 친다 | 4 | 기존 |
| 1 | 캠프-스플릿 최소 거리 | objective_helpers.rs:113 (IR 54843 `icmp ugt i64 %57, 62499999999`) | 62499999999 | 250000(≈7.8칸) 이상 떨어져야 '먼 라인'. 내리면 캠프 근처에 있는 아군도 스플릿으로 오인 | 4 | 기존 |
| 2 | 타워 리드 임계 | objective_helpers.rs:121 (IR 55318 `icmp ugt i64 %230, 1`) | 1 | 그 라인 lead ≥2 면 웨이브 상태와 무관하게 true. 올리면 리드만으로는 압박 인정이 어려워진다 | 4 | 기존 |
| 3 | 웨이브 밀림 임계 2단 | objective_helpers.rs:122~124 (IR 55323 `> 2999`, 55327 `> 999`, 55334 `> 3`, 55337 `> 1`) | 2999 / 999 | from_mid > 2999 무조건 true ; 999 < from_mid ≤ 2999 는 minion_count > 1 ; from_mid ≤ 999 는 minion_count > 3. 값을 내리면 덜 밀린 라인도 압박으로 인정 | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 5 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | v23_healthy | game_ai::plan_legacy::team_plan::objective_helpers::v23_healthy | in:game_ai | fn(&game_core::Entity, usize) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:11 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 8 | v25_objective_far_line | game_ai::plan_legacy::team_plan::v25_objective_far_line | pub | fn(game_core::JungleType) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:95 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 9 | v25_objective_far_split_pressure | game_ai::plan_legacy::team_plan::v25_objective_far_split_pressure | pub | fn(&game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:103 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

**호출처 2곳** (m09.ll:18842, m15.ll:54726) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | rule_scope::line_exists(context, LineType) 내부 미열람 — 튜토리얼 모드에서 라인이 있는지 검사로 추정 | 5 |  |
| 1 | 미탐색 | map_regions::is_near_line(context, x, y, LineType) 의 반경 미열람(game_core) | 4 |  |
| 2 | 미탐색 | BrainMinionParameter.from_mid 의 부호·단위(양수 = 적 쪽으로 밀림)는 게이트 방향에서 추정. minion_count 의 의미(아군-적 미니언 수 차?)도 추정 | 5 |  |
| 3 | 미탐색 | top_lead/bottom_lead 의 정의(파괴한 타워 수 차이?)는 game_core 미열람 — '> 1' 이 리드 2 이상이라는 것만 확정 | 4 |  |
| 4 | 미탐색 | v23_healthy(objective_helpers.rs:12) 는 인라인이라 별도 define 없음 — 본문의 `hp*100/max_hp > 39` 가 그 전부라고 읽음 | 4 |  |
| 5 | 미탐색 | far_line 이 Mid(1) 가 되는 경로는 없다(Serpen→Top, Morgard→Bottom, 그 외 None). 따라서 L120 의 minion_state 선택이 Top/Bottom 2분기(select 0/80) 로 접힌 것 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

