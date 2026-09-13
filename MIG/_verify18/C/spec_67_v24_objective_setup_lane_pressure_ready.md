---

### `67` v24_objective_setup_lane_pressure_ready — 오브젝트(Morgard/Serpen) Setup 단계에서 '라인 정리가 끝나 캠프로 모여도 되는가'를 아군 집결·건강·압박 라인 잔여로 판정

| 항목 | 값 |
|---|---|
| id | `objective_discipline__v24_objective_setup_lane_pressure_ready` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan20objective_disciplineNtB4_8TeamPlan39v24_objective_setup_lane_pressure_ready` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7` |
| IR | `m09.ll` 21244~22101행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready` · **pub** |
| 계층 | 기타 |
| exe | `dd6b40` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | self | &TeamPlan(1064B) | objective(0x41f tag / 0x420 phase) 만 읽음 — take_setup_like 인라인 | 4 |
| 1 | 1 | version | usize | 본문에서 미사용(분기 없음) | 4 |
| 2 | 2 | player | &PlayerState(2528B) | info.team(0x930)·info.position@tag(0x9c0, 디버그 로그용) | 4 |
| 3 | 3 | data | &OperationData(24B) | cache(+0)·context(+8) 사용, blackboard 미사용 | 4 |
| 4 | 4 | goal_data | &GoalData(248B) | is_object_being_taken_by_enemy 에 그대로 전달만 | 4 |
| 5 | 5 | camp | JungleType(u8: 4=Morgard 5=Serpen) | dbg 이름 target/camp. 4·5 외는 즉시 false | 4 |
| 6 | 6 | debug | &mut DebugFrameData | context.debug(0x3b) 일 때만 +0xa0 HashMap<usize,Vec<String>> 에 로그 push | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v24_objective_setup_lane_pressure_ready(self, version, player, data, goal_data, camp, debug) -> bool
  // L13 take_setup_like(camp) 인라인 (team_plan.rs:258)
  match camp { Morgard(4) => self.objective == Some(Morgard{phase: Setup,..})   // 0x41f==0 && 0x420==1
               Serpen(5)  => self.objective == Some(Serpen{phase: Setup,..})    // 0x41f==1 && 0x420==1
               _ => return false }  else return false
  obj = if camp==Serpen { WavePriorityObject::Serpen(1) } else { Morgard(0) }
  if is_object_being_taken_by_enemy(player, data, goal_data, self, obj) → return false     // L23
  team = player.info.team
  camp_pos = data.context.map.camp_pos(camp, team == 0)                                    // L27
  if !game.is_visible_cell(team, camp_pos.x/32000, camp_pos.y/32000) → return false       // L28 (vtable+0x100)
  if v23_recent_visible_enemies_near_point(player, data, camp_pos.x, camp_pos.y, 180000, 50) != 0 → return false   // L32
  allies = cache.player_champion[team]   (bounds team<2)
  pc = context.tutorial.player_count()   // runner.rs:295~301 인라인: TopSolo/MidSolo/JungleOnly→1, First/Bottom→2, MidBottom→3, None/Line/Total→>=4
  gathered = allies.flatten().filter(|c| c.hp*100/c.max_hp > 49 && dist²(c, camp_pos) < 220000²+1).count()   // L36~39 (5회 언롤)
  if gathered < min(2, pc) → return false                                                  // L40
  healthy = allies.flatten().filter(|c| c.hp*100/c.max_hp > 49).count()                    // L44 (aux s_0)
  if healthy < min(3, pc) → return false                                                   // L45
  need = if camp == Morgard { 4 } else { 3 }                                               // L49~52
  if gathered >= min(need, pc) {                                                           // L54 (반대 분기)
     if context.debug { debug.infos[my_champ.id].push("v24 objective setup gathered ready: {camp:?}") }   // L55~58
     return true }
  // L63~70 라인 배치 인원 (aux s0_0)
  placed = allies.flatten().filter(|c| c.hp*100/c.max_hp > 49 &&
              match camp { Morgard => is_top_side(c) || is_near_mid_line(ctx, c.x, c.y),      // L65: x <= height - y
                           Serpen  => is_bottom_side(c) || is_near_mid_line(ctx, c.x, c.y),   // L66: x >= height - y
                           _ => false }).count()
  if placed < min(3, pc) → return false                                                     // L70
  // L74 v24_objective_setup_relevant_lanes_ready 인라인(objective_discipline.rs:199~202)
  lanes: [LineType;2] = match camp { Morgard => [Top, Mid], Serpen => [Bottom, Mid], _ => return false }
  if v23_objective_setup_pressure_line(player, data, &lanes).is_some() → return false        // L75 (아직 밀어야 할 라인이 남음)
  if context.debug { debug.infos[my_champ.id].push("v24 objective setup lane pressure ready: {camp:?}") }   // L79~82
  return true                                                                                // L86
```

**`mem` 메모리 접근 19건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | TeamPlan | 0x41f | objective (MainObjective: 0=Morgard 1=Serpen) | r | gep +1055 (m09.ll:21281). take_setup_like(team_plan.rs:258) 인라인 | 4 | OK |  |
| 1 | TeamPlan | 0x420 | objective@Some.0@{Morgard,Serpen}.phase (ObjectPhase: 1=Setup) | r | gep +1056 (m09.ll:21283) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 2 | PlayerState | 0x930 | info.team | r | L27 camp_pos 의 is_blue(team==0)·L28 is_visible_cell·player_champion[team] | 4 | OK |  |
| 3 | PlayerState | 0x9c0 | info.position@tag | r | L56/L80 디버그 로그 키(자기 챔피언 id) 조회에만 | 4 | OK |  |
| 4 | OperationData | 0x0 | cache | r | game(+0 data/+8 vtable)·player_champion | 4 | OK |  |
| 5 | OperationData | 0x8 | context(&GameContext) | r | map(0x20)·setting(0x8, 클로저)·tutorial(0x38)·debug(0x3b) | 4 | OK |  |
| 6 | GameContext | 0x20 | map(&MapDef) | r | L27 camp_pos(camp, team==0) | 4 | OK |  |
| 7 | GameContext | 0x38 | tutorial(TutorialType) | r | gep +56 (m09.ll:21762). TutorialType::player_count()(runner.rs:295~301) 인라인 → 인원 임계 min() 에 사용 | 4 | OK |  |
| 8 | GameContext | 0x3b | debug(bool) | r | gep +59 (m09.ll:21911/21951). 로그 분기만, 반환값 무영향 | 4 | OK |  |
| 9 | GameContext | 0x8 | setting(&GameSetting) | r | aux s0_0: setting.height(0x12c0) 로 is_top_side/is_bottom_side 계산 | 4 | OK |  |
| 10 | GameSetting | 0x12c0 | height | r | aux s0_0 gep +4800 (m09.ll:67433). is_top_side = x <= height-y / is_bottom_side = x >= height-y (map_regions.rs:22~30 인라인) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x0 | game.data_ptr data/vtable | r | vtable+0x100 = is_visible_cell(team, x/32000, y/32000) (L28, m09.ll:21335) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x1e0 | player_champion[team][0..5] | r | gep +480 (m09.ll:21345). L36~40 언롤 5회 / 클로저 2개 / 디버그 로그 | 4 | OK |  |
| 13 | Entity | 0x628 | stat_cached.hp(최대) | r | hp% 분모 (0 → div_by_zero 패닉) | 4 | OK |  |
| 14 | Entity | 0x670 | hp | r | hp% 분자 | 4 | OK |  |
| 15 | Entity | 0x660 | x | r | camp_pos 거리²·is_top/bottom_side·is_near_mid_line | 4 | OK |  |
| 16 | Entity | 0x668 | y | r | 동상 | 4 | OK |  |
| 17 | Entity | 0x5c0 | id | r | 디버그 로그 HashMap 키 (m09.ll:21968/22052) | 4 | OK |  |
| 18 | DebugFrameData | 0xa0 | infos(HashMap<usize, Vec<String>>)[champ.id].push(String) | w | context.debug 일 때만. 판정에 무영향 (gep +160, m09.ll:21969/22053) | 4 | OK | "v24 objective setup gathered ready: {camp:?}" (L57) / "v24 objective setup lane pressure ready: {camp:?}" (L81) |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 13 | 태그 | JungleType::Morgard 태그(switch case, m09.ll:21287). L49 `camp == Morgard` 도 동일. L51 Morgard 집결 요구 인원 4 | 4 |
| 1 | 5 | 13 | 태그 | JungleType::Serpen 태그(switch case). (아군 슬롯 수 5 는 슬라이스 길이·판정 아님) | 4 |
| 2 | 0 | 13 | 태그 | MainObjective::Morgard 메모리태그 0 (0x41f) — camp==Morgard 일 때 objective 가 Morgard 여야 함 | 4 |
| 3 | 1 | 13 | 태그 | ObjectPhase::Setup 메모리태그 1 (0x420) — 두 case 공통. camp==Serpen 은 objective 태그 1(Serpen) 도 요구 | 4 |
| 4 | 32000 | 28 | 계수 | 셀 크기 — camp_pos 를 셀 좌표로 변환해 is_visible_cell 에 넘김 (좌표 변환, 임계 아님) | 4 |
| 5 | 180000 | 32 | 미상 | v23_recent_visible_enemies_near_point 반경(u64) — 캠프 180k 안 적 수가 0 이어야 함 | 4 |
| 6 | 50 | 32 | 미상 | v23_recent_visible_enemies_near_point 마지막 인자(usize) — 시그니처상 usize, 의미(최근 틱 창 추정)는 그 함수 본문 미탐색 | 5 |
| 7 | 100 | 37 | 계수 | hp% 환산 hp*100/max_hp (L37·L44·L64) | 4 |
| 8 | 49 | 37 | 임계 | hp% > 49 (=50% 이상) 만 '건강한 아군' — L37 근접 집결 수·L44 건강 수·L64 라인 배치 수 3곳 공통 | 4 |
| 9 | 48400000001 | 38 | 임계 | 220000² + 1 — 아군↔camp_pos 거리²(\|dx\|²+\|dy\|²) < 이 값 = 220k 이내 집결로 셈 | 4 |
| 10 | 2 | 40 | 임계 | L40: gathered(220k·hp50%) < min(2, player_count) → false. player_count(tutorial): TopSolo/MidSolo/JungleOnly=1, First/Bottom=2, MidBottom=3, None/Line/Total>=4 | 4 |
| 11 | 3 | 45 | 태그 | L45: healthy(hp>49, 거리무관) < min(3, player_count) → false. L50: 비-Morgard 집결 요구 3. L70: 배치 인원 < min(3, pc) → false | 4 |
| 12 | -1 | 75 | 센티널 | Option<LineType>::None 니치(255). v23_objective_setup_pressure_line 이 None(=압박할 라인 없음)일 때만 true 로 진행 | 4 |

**`knobs` 조정점 8건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 건강 임계 hp% | objective_discipline.rs:37/44/64 | 49 | 낮추면 저체력 아군도 집결/배치 인원으로 셈 → 더 빨리 ready | 4 | 기존 |
| 1 | 캠프 집결 반경 | objective_discipline.rs:38 | 48400000001 (=220000²+1) | 키우면 멀리 있는 아군도 gathered 로 셈 | 4 | 기존 |
| 2 | 캠프 주변 적 감시 반경 / 창 | objective_discipline.rs:32 | 180000 / 50 | 반경을 줄이면 적이 근처에 있어도 ready 판정 가능(위험) | 4 | 기존 |
| 3 | 최소 집결 인원 | objective_discipline.rs:40 | min(2, player_count) |  | 4 | 기존 |
| 4 | 최소 건강 인원 | objective_discipline.rs:45 | min(3, player_count) |  | 4 | 기존 |
| 5 | 완전 집결 인원(라인 검사 생략 조건) | objective_discipline.rs:49~54 | Morgard 4 / Serpen 3 | 낮추면 라인 압박 검사 없이 바로 true | 4 | 기존 |
| 6 | 라인 배치 최소 인원 | objective_discipline.rs:70 | min(3, player_count) |  | 4 | 기존 |
| 7 | 검사 대상 라인 | objective_discipline.rs:200~202 (인라인, anon.190/.191) | Morgard [Top,Mid] / Serpen [Bottom,Mid] | v23_objective_setup_pressure_line 에 넘기는 후보 | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | is_bottom_side | game_core::is_bottom_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:27 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | is_object_being_taken_by_enemy | game_ai::plan_legacy::team_plan::objective_helpers::is_object_being_taken_by_enemy | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:313 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | is_top_side | game_core::is_top_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:21 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | is_visible_cell | game_core::AbstractGame::is_visible_cell | pub | fn(&Self/#0, usize, usize, usize) -> bool | game-core\src\simulation.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | is_visible_cell | <game_core::Game as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::Game, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:1804 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | is_visible_cell | <game_core::SingleLaneGame as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::SingleLaneGame, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:3869 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 11 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 12 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 13 | take_setup_like | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan.rs:257 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | v23_objective_setup_pressure_line | game_ai::plan_legacy::team_plan::v23_objective_setup_pressure_line | pub | fn(&game_core::PlayerState, &game_core::OperationData, &[game_core::LineType]) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:73 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | v23_recent_visible_enemies_near_point | game_ai::plan_legacy::team_plan::v23_recent_visible_enemies_near_point | pub | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:31 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | v24_objective_setup_lane_pressure_ready | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 4개**: `format_inner`, `or_insert`, `push_mut`, `rustc_entry`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m05.ll:46995, m09.ll:31351, m09.ll:32168, m09.ll:56379) · **형제 55개** (TeamPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic | pub | game-ai\src\plan_legacy\old\epic.rs:502 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 1 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window | in:game_ai | game-ai\src\plan_legacy\old\epic.rs:634 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool |
| 2 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change | pub | game-ai\src\plan_legacy\old\epic.rs:684 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType) |
| 3 | <game_ai::plan_legacy::team_plan::TeamPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> game_ai::plan_legacy::team_plan::TeamPlan |
| 4 | <game_ai::plan_legacy::team_plan::TeamPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 5 | <game_ai::plan_legacy::team_plan::TeamPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn() -> game_ai::plan_legacy::team_plan::TeamPlan |
| 6 | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | game-ai\src\plan_legacy\team_plan.rs:196 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) |
| 7 | game_ai::plan_legacy::team_plan::TeamPlan::sanitize_rule_scope | pub | game-ai\src\plan_legacy\team_plan.rs:200 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::GameContext) |
| 8 | game_ai::plan_legacy::team_plan::TeamPlan::objective_target | pub | game-ai\src\plan_legacy\team_plan.rs:230 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> std::option::Option<game_core::JungleType> |
| 9 | game_ai::plan_legacy::team_plan::TeamPlan::take_active | pub | game-ai\src\plan_legacy\team_plan.rs:243 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 10 | game_ai::plan_legacy::team_plan::TeamPlan::take_hunt_commit | pub | game-ai\src\plan_legacy\team_plan.rs:248 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 11 | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | game-ai\src\plan_legacy\team_plan.rs:257 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 12 | game_ai::plan_legacy::team_plan::TeamPlan::mark_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:265 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType, usize) |
| 13 | game_ai::plan_legacy::team_plan::TeamPlan::clear_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:277 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan) |
| 14 | game_ai::plan_legacy::team_plan::TeamPlan::init | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:281 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 15 | game_ai::plan_legacy::team_plan::TeamPlan::update | pub | game-ai\src\plan_legacy\team_plan.rs:294 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies | pub | game-ai\src\plan_legacy\team_plan.rs:444 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 17 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | game-ai\src\plan_legacy\team_plan.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 18 | game_ai::plan_legacy::team_plan::TeamPlan::update_wave_priority_clear_line | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:540 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 19 | game_ai::plan_legacy::team_plan::TeamPlan::should_player_clear_wave_priority_line | pub | game-ai\src\plan_legacy\team_plan.rs:561 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> |
| 20 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_morgard_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:570 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 21 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_serpen_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:574 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 22 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_object_setup_for_wave_priority | pub | game-ai\src\plan_legacy\team_plan.rs:578 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 23 | game_ai::plan_legacy::team_plan::TeamPlan::should_keep_object_for_contested_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:586 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool |
| 24 | game_ai::plan_legacy::team_plan::TeamPlan::repair_misunderstood_objective | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:603 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 25 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective | pub | game-ai\src\plan_legacy\team_plan.rs:722 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 26 | game_ai::plan_legacy::team_plan::TeamPlan::update_steal | pub | game-ai\src\plan_legacy\team_plan.rs:732 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) |
| 27 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective_after_steal | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:910 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool |
| 29 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool |
| 30 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_release_to_passive | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:177 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 31 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_relevant_lanes_ready | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:199 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 32 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> |
| 33 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_wait_pos | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:316 | False | fn(game_core::JungleType, usize, &game_core::MapDef) -> (u64, u64) |
| 34 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_entity | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:324 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData, game_core::JungleType) -> std::option::Option<&game_core::Entity> |
| 35 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::update_v27_objective_discipline | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_active_objective_discipline | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectiveDisciplineState> |
| 37 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_blocks_battle | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:497 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData) -> bool |
| 38 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> |
| 39 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_repair_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:7 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) |
| 40 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:16 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 41 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:29 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 42 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_line_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:46 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 43 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_dive_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:88 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 44 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_tower_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:141 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 45 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:253 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 46 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_split_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 47 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_comeback_pick_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:393 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType, bool) -> bool |
| 48 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_serpen_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:558 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 49 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_morgard_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:685 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 50 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_none_or_gank_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 51 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_gank_preprocess | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1133 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, game_core::LineType) -> bool |
| 52 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_defense | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1227 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 53 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_attack | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1237 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> |
| 54 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_sub_objective | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1258 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | v23_recent_visible_enemies_near_point 의 6번째 인자 50(usize)의 의미 — 그 함수 본문(m15.ll:55548) 미탐색. '최근 N틱' 으로 추정 | 4 |  |
| 1 | 미탐색 | v23_objective_setup_pressure_line 내부(m15.ll:55350, 어떤 라인을 '압박 필요'로 보는지) 미탐색 — 이 함수는 Some/None 만 봄 | 4 |  |
| 2 | 미탐색 | is_object_being_taken_by_enemy 내부(m15.ll:53833) 미탐색 | 4 |  |
| 3 | 표기 불가 | is_top_side/is_bottom_side 의 소스 표기(`x <= h-y` vs `x+y <= h`)는 표기 불가(외연 동일) — IR 은 `(h - y) < x` 의 부정 / `(h - y) > x` 의 부정 | 4 |  |
| 4 | 표기 불가 | L54 이후 L55~57 은 gathered >= min(need,pc) 일 때 true 를 반환하는 분기 — 소스가 `if gathered >= need {return true}` 인지 `if gathered < need {…} else {return true}` 인지 표기 불가 | 4 |  |
| 5 | 미탐색 | camp_pos 의 3번째 인자 `team == 0` 의 의미(is_blue 추정)는 MapDef::camp_pos(_gcbc) 미탐색 | 5 |  |
| 6 | 미탐색 | goal_data·version 은 이 본문에서 판정에 직접 쓰이지 않음(goal_data 는 헬퍼에 전달만, version 은 미사용) | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | TutorialType::player_count 의 None/Line/Total 반환값은 본문에서 4 이상임만 확정(min(4,pc)=4). 정확한 값(5?)은 runner.rs:295~301 본체 미탐색 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

