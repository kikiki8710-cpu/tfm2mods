---

### `40` v3_assign_anchor — v3_armed 이면 현 BigPlan 의 목적지(라인 1차타워 중점/정글캠프)를, 2칸 안이면 현 위치를 앵커로 낸다

| 항목 | 값 |
|---|---|
| id | `handler__v3_assign_anchor` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler16v3_assign_anchor` |
| 소스 | `game-ai\src\plan_legacy\handler.rs:1773` |
| IR | `m13.ll` 10721~10921행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::LegacyPlanHandler::v3_assign_anchor` · **in:game_ai::plan_legacy::handler** |
| 계층 | 플랜 핸들러 |
| exe | `e4a780` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)>
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<(u64,u64)>(24B) — +0x0 tag(0 None/1 Some), +0x8 x, +0x10 y | IR %0. 챔프 부재일 때만 None | 4 |
| 1 | 1 | self | &LegacyPlanHandler | IR %1. v3_armed(+0x1808)·plan 태그(+0x5e8)·plan 페이로드(+0x5f0~) 를 읽는다 | 4 |
| 2 | 2 | player | &PlayerState(2528B) | 인자 승격: IR %2 = player.info.team(i64, <2 아니면 panic_bounds_check), IR %3 = player.info.position(i32, Position::as_index 인라인 entity.rs:581) | 4 |
| 3 | 3 | data | &OperationData(24B) | 인자 승격: IR %4 = data.cache(+0x0), IR %5 = data.context(+0x8). blackboard 는 안 읽음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v3_assign_anchor(&self, player, data) -> Option<(u64,u64)>   // handler.rs:1773~1799
[L1774] team = player.info.team (<2)   pos = player.info.position.as_index()
        champ = data.cache.player_champion(+0x1e0)[team][pos] else { return None }
[L1775] here = (champ.x(+0x660), champ.y(+0x668))
[L1776] if !self.v3_armed(+0x1808) { return Some(here) }
[L1779] line_anchor = |line: LineType| {
[L1780]   mine   = first_tower_position(line, team)        // m15.ll:51623 — 상수표: Top (272000,48000)/(48000,272000), Mid (592000,368000)/(368000,592000), Bottom (912000,688000)/(688000,912000); team==0 이면 앞쪽
[L1781]   theirs = first_tower_position(line, 1 - team)
[L1782]   ((mine.x + theirs.x) >> 1, (mine.y + theirs.y) >> 1) }   // 양 팀 1차 타워의 중점 = 라인 중앙
[L1784] dest = match self.plan(+0x5e8 태그) {
[L1785]   PassiveLine(p)                          => line_anchor(p.line(+0x706)),
[L1786]   LineGanker(p)                           => line_anchor(p.line(+0x618)),
[L1787]   PassiveJungle(p)                        => data.context.map(+0x20).camp_pos(p.jungle(+0x650), p.team(+0x638) == 0),
[L1789]   EpicHuntAndPoke | EpicHuntAndBattle     => map.camp_pos(JungleType::Morgard(4), team == 0),
[L1791]   SerpenHuntAndPoke | SerpenHuntAndBattle => map.camp_pos(JungleType::Serpen(5), team == 0),
          _ /*ForcePassive·SinglePlanLine·SinglePlanBattle·DeathMatchBattle·ActiveRecall·Battle·LineGankCover·AttackNexus·DefenseNexus*/ => return Some(here) }
[L1796] d2 = abs_diff(here.x, dest.x)² + abs_diff(here.y, dest.y)²
        return Some( if d2 > 4096000000 /*64000²=2셀*/ { dest } else { here } )

※ 부작용 없음(sret 외 store 0). rnd 없음. 이 함수는 v33+ ActionContext 의 anchor("어디 근처에 있어야 하는가") 공급원으로 보이나 호출측(m13.ll 7002/7166/8193)은 안 봤다.
```

**`mem` 메모리 접근 10건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] stride 40. null → return None (handler.rs:1774 let-else) | 4 | OK |
| 1 | Entity(내 챔프) | 0x660 | x | r | here.0 | 4 | OK |
| 2 | Entity(내 챔프) | 0x668 | y | r | here.1 | 4 | OK |
| 3 | LegacyPlanHandler | 0x1808 | v3_armed | r | bool(i8 range 0..2). false → 즉시 Some(here). true 여야 plan 목적지를 본다 (handler.rs:1776) | 4 | OK |
| 4 | LegacyPlanHandler | 0x5e8 | plan (BigPlan 니치 태그) | r | i64 range[0,18), 6 은 아님(assume). 태그 ≥2 → 논리idx = 태그-2, 태그 0/1 → idx 4(DeathMatchBattle) | 4 | OK |
| 5 | LegacyPlanHandler | 0x706 | plan@PassiveLine.0.line (LineType i8 0..3) | r | idx1 PassiveLine 가지 (handler.rs:1785) | 4 | OK |
| 6 | LegacyPlanHandler | 0x618 | plan@LineGanker.0.line (LineType i8 0..3) | r | idx8 LineGanker 가지 (handler.rs:1786) | 4 | OK |
| 7 | LegacyPlanHandler | 0x650 | plan@PassiveJungle.0.jungle (JungleType i8 0..6) | r | idx5 PassiveJungle 가지 — camp_pos 의 캠프 인자 (handler.rs:1787) | 4 | OK |
| 8 | LegacyPlanHandler | 0x638 | plan@PassiveJungle.0.team (usize) | r | == 0 이 camp_pos 의 세 번째 인자(bool). ⚠player 팀이 아니라 플랜 페이로드의 team | 4 | OK |
| 9 | GameContext | 0x20 | map: &MapDef(28112B) | r | camp_pos 의 self | 4 | OK |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 1774 | 임계 | team 인덱스 상한 (player_champion 의 바깥 배열 길이 2, panic_bounds_check) | 4 |
| 1 | 6 | 1784 | 센티널 | BigPlan 태그 6 은 존재하지 않음(llvm.assume ne 6) — 니치 구멍 | 4 |
| 2 | -2 | 1784 | 태그 | BigPlan 태그 → 논리 인덱스 (niche_start=2). 1 PassiveLine / 5 PassiveJungle / 8 LineGanker / 10·11 EpicHuntAndPoke·EpicHuntAndBattle / 12·13 SerpenHuntAndPoke·SerpenHuntAndBattle | 4 |
| 3 | 4 | 1784 | 센티널 | 태그 0/1(DeathMatchBattle 니치)의 논리 인덱스 4 — switch 에 없으므로 default(=Some(here)) | 4 |
| 4 | 1 | 1781 | 인덱스 | theirs = first_tower_position(line, 1 - team) — 상대 팀 인덱스 | 4 |
| 5 | 1 | 1782 | 임계 | (mine + theirs) >> 1 — 두 1차 타워의 중점(x,y 각각). 나눗셈 2 가 lshr 1 로 접힘 | 4 |
| 6 | 4 | 1789 | 태그 | JungleType 태그 4 = Morgard — Epic 플랜 두 종의 목적지 캠프 | 4 |
| 7 | 5 | 1791 | 태그 | JungleType 태그 5 = Serpen — Serpen 플랜 두 종의 목적지 캠프 | 4 |
| 8 | 0 | 1789 | 태그 | camp_pos(…, team == 0) — Epic/Serpen 가지의 bool 인자 = 내 팀이 0번 팀인가(캠프 좌표 대칭 선택으로 추정) | 5 |
| 9 | 4096000000 | 1796 | 임계 | 64000² = (2셀)² — here↔dest 제곱거리가 이보다 크면 dest 를, 아니면 here 를 앵커로 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 앵커 스냅 반경 (플랜 목적지 vs 현 위치) | handler.rs:1796 (IR m13.ll:10917 `icmp ugt i64 %108, 4096000000`) | 4096000000 | 64000²(2셀). 목적지에서 이보다 멀면 앵커=목적지(그쪽으로 끌림), 가까우면 앵커=현 위치(자유 배회). 올리면 목적지 근처에서 더 넓게 배회하고, 내리면 목적지에 더 바짝 붙는다 | 4 | 기존 |
| 1 | v3_armed 게이트 | handler.rs:1776 (IR m13.ll:10762 load self+6152) | 1 | false 면 플랜과 무관하게 앵커=현 위치. 강제로 true 면 모든 플랜에서 목적지 앵커가 켜진다 | 4 | 기존 |
| 2 | Epic 플랜 목적지 캠프 | handler.rs:1789 (IR m13.ll:10873 `i8 noundef 4`) | 4 | JungleType 4=Morgard. 바꾸면 에픽 사냥 플랜 중 앵커가 다른 캠프로 간다 | 4 | 기존 |
| 3 | Serpen 플랜 목적지 캠프 | handler.rs:1791 (IR m13.ll:10887 `i8 noundef 5`) | 5 | JungleType 5=Serpen | 4 | 기존 |
| 4 | 라인 앵커 = 양 팀 1차 타워 중점 | handler.rs:1782 (IR m13.ll:10815/10817 `lshr i64 …, 1`) | 1 | lshr 1 = /2 (중점). 가중치를 바꾸면(예: 3:1) 아군 타워 쪽으로 앵커가 당겨진다 — first_tower_position 상수표(m15.ll:51643~51654)도 함께 보라 | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | first_tower_position | game_ai::plan_legacy::team_plan::first_tower_position | pub | fn(game_core::LineType, usize) -> (u64, u64) | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:237 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | line | game_ai::plan_legacy::types::BigPlan::debug_label::line | in:game_ai::plan_legacy::types | fn(game_core::LineType) -> &str | game-ai\src\plan_legacy\types.rs:83 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 5 | line | game_core::TowerType::line | pub | fn(&game_core::TowerType) -> std::option::Option<game_core::LineType> | game-core\src\simulation\entity\tower.rs:90 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 6 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | v3_assign_anchor | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_assign_anchor | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\plan_legacy\handler.rs:1773 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 6개**: `anchor`, `jungle`, `line_anchor`, `plan`, `player_champion`, `v3_armed`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m13.ll:7002, m13.ll:7166, m13.ll:8193) · **형제 41개** (LegacyPlanHandler)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 1 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::handler::LegacyPlanHandler::new | pub | game-ai\src\plan_legacy\handler.rs:228 | False | fn(usize, &mut rand::rngs::std::StdRng, usize, game_core::Position) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 3 | game_ai::plan_legacy::handler::LegacyPlanHandler::r2_fight_protected | in:game_ai | game-ai\src\plan_legacy\handler.rs:362 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 4 | game_ai::plan_legacy::handler::LegacyPlanHandler::ff_note_battle_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:400 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 5 | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:409 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 6 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_fall_back_to_passive | pub | game-ai\src\plan_legacy\handler.rs:416 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 7 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_cover_picks | pub | game-ai\src\plan_legacy\handler.rs:440 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 8 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_serpen_punish_issues | pub | game-ai\src\plan_legacy\handler.rs:445 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 9 | game_ai::plan_legacy::handler::LegacyPlanHandler::subplan_is_recall | pub | game-ai\src\plan_legacy\handler.rs:450 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> bool |
| 10 | game_ai::plan_legacy::handler::LegacyPlanHandler::team_objective_code | pub | game-ai\src\plan_legacy\handler.rs:456 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> u8 |
| 11 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_v2_egowave | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:473 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 12 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_obj_restore_safe | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:501 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 13 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_apply_assign_commit | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:518 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut u8, &mut game_core::DebugFrameData) |
| 14 | game_ai::plan_legacy::handler::LegacyPlanHandler::sanitize_rule_scope | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:571 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::OperationData) |
| 15 | game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:588 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool |
| 16 | game_ai::plan_legacy::handler::LegacyPlanHandler::enter_line_backfight_support | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:598 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 17 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_on_dead | pub | game-ai\src\plan_legacy\handler.rs:635 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 18 | game_ai::plan_legacy::handler::LegacyPlanHandler::update | pub | game-ai\src\plan_legacy\handler.rs:685 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 19 | game_ai::plan_legacy::handler::LegacyPlanHandler::determine_transition_reason | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1675 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &str) -> game_core::PlanTransitionReason |
| 20 | game_ai::plan_legacy::handler::LegacyPlanHandler::force_plan_update | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1709 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 21 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_assign_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1773 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 22 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_depart_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1806 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 23 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_plan_dest | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1824 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::types::BigPlan) -> std::option::Option<(u64, u64)> |
| 24 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_repair_done | in:game_ai | game-ai\src\plan_legacy\handler.rs:1847 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 25 | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1855 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) |
| 26 | game_ai::plan_legacy::handler::auction::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::get_small_action | pub | game-ai\src\plan_legacy\handler\auction.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &std::vec::Vec<(usize, game_core::SmallAction), std::alloc::Global>, &mut game_core::DebugFrameData) -> (game_ai::ScoreParameter, i64, game_ai::SmallActionPlay) |
| 27 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat | pub | game-ai\src\plan_legacy\handler\chat.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat_inner | in:game_ai | game-ai\src\plan_legacy\handler\chat.rs:41 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 29 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_track_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:35 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) |
| 30 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_fold_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:125 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8) |
| 31 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v2_response_retreat_stance | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:13 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::BattleSubPlanGoal |
| 32 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:40 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 33 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage_dive | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:109 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 34 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:136 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 35 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_interact_battle | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:233 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_single_lane | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:13 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 37 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_deathmatch | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:56 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 38 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_lane_initiate | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:196 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 39 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_try_engage | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:241 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::SinglePlanBattle> |
| 40 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:261 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | MapDef::camp_pos(&MapDef, JungleType, bool) 의 bool 인자 의미 — 호출측은 `team == 0`(Epic/Serpen 가지) 또는 `p.team == 0`(PassiveJungle 가지)을 넘긴다. '0번 팀 기준 좌표 대칭' 으로 추정. 본문은 _gcbc/g07.ll:152570 에 있으나 안 읽음(미탐색) | 4 |  |
| 1 | 표기 불가 | default 가지(9개 variant)에서 dest 가 '없음'인지 'here' 인지는 소스 표기 구분 불가 — IR 상 phi 로 곧장 Some(here) 에 합류하므로 관측 동작은 동일(표기 불가) | 4 |  |
| 2 | 미탐색 | PassiveJungle 가지가 플레이어 팀이 아니라 플랜 페이로드의 `p.team`(+0x638) == 0 을 쓰는 이유 — PassiveJunglePlan 에 team(+0x48)/player_team(+0x50) 두 필드가 있고 여기선 +0x48(team) 을 읽는다. 둘의 의미 차이는 미탐색 | 4 |  |
| 3 | 미탐색 | 호출자 3곳(m13.ll:7002 / 7166 / 8193)이 어느 함수인지·반환 앵커가 어떤 채점에 쓰이는지는 안 봤다(범위 밖) | 4 |  |
| 4 | 미탐색 | v3_armed 가 언제 true 로 세팅되는지(쓰기 지점)는 이 함수 밖 — 미탐색 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

