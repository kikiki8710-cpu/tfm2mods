---

### `90` v3_epicops_defer_serpen — [v3 EPIC-OPS] 우리 에픽 버프 창 동안 세르펜 진입을 '미룰지' 판정 — 적이 세르펜을 치는 중이면 안 미룸, 압도적 우세(clear_win)면 안 미룸, 라인 미정리면 미룸, 그 외엔 세르펜 도달가능 적이 있을 때만 미룸

| 항목 | 값 |
|---|---|
| id | `objective_helpers__v3_epicops_defer_serpen` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers23v3_epicops_defer_serpen` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:275` |
| IR | `m15.ll` 51836~52544행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::objective_helpers::v3_epicops_defer_serpen` · **in:game_ai** |
| 계층 | 기타 |
| exe | `ec8ba0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | AI 버전. 본문 분기 없음 — v3_serpen_contest_clear_win 에 그대로 전달만 (m15.ll:52430) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | info.team(+0x930)만 읽음 (m15.ll:51862) | 4 |
| 2 | 3 | data | &OperationData(24B) | cache(+0)·context(+8)·blackboard(+0x10) 전부 사용 | 4 |
| 3 | 4 | goal_data | &GoalData(248B) | serpen.epic_enemy_tick(+0xc0)만 읽음 (m15.ll:51939) | 4 |
| 4 | 5 | team_plan | &TeamPlan(1064B) | 공유 참조(ref$<TeamPlan>, DILocalVariable !58483). obj_spawn.serpen_camp_last_visible_tick(+0x88) 읽기 + 두 피호출 함수에 전달. store 0건 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v3_epicops_defer_serpen(version, player, data, goal_data, team_plan) -> bool
  team = player.info.team; enemy = 1 - team; tps = context.setting.tick_per_second
  // L276: 우리 팀 에픽 버프가 남아 있어야 판정 자체가 성립
  remain = game.get_game_mode()(vt+0x40) 가 Moba(tag 0)이면 mob.epic_minion_buff_time[team] (game.rs:210 remain_epic_time) 아니면 0
  if remain == 0 { return false }
  // L279: is_object_being_taken_by_enemy(player, data, goal_data, team_plan) (objective_helpers.rs:326~332, 인라인)
  //   L326: if !serpen_exists(context) → false  [tutorial 태그 ∉ {0,5,7,8}]
  //   L327: recently_visible = team_plan.obj_spawn.serpen_camp_last_visible_tick + tps >= game.tick()(vt+0x28)
  //   L328: enemy_kill_tick = goal_data.serpen.epic_enemy_tick
  //   L329: mob = game.as_moba().unwrap()  (Moba 가 아니면 unwrap 패닉 — 329:63)
  //   L330: serpen = mob.jungle_runner.serpen.live_list.get(0).and_then(|id| game.get_entity_by_id(*id)(vt+0x1f0))
  //   L331: damaged = serpen.is_some_and(|s| game.is_visible(team, s.id)(vt+0xf8) && s.hp < s.stat_cached.hp)
  //   L332: return recently_visible && damaged && enemy_kill_tick <= tps*20   [주의: epic_enemy_tick 을 현재 tick 과 빼지 않고 그대로 tps*20 과 비교]
  if being_taken { return false }   // 적이 지금 세르펜을 치고 있으면 미루지 않는다
  // L283~284: 적에게 밀 수 있는 타워가 하나라도 있어야 함
  lines = valid_lines(context)  (rule_scope.rs:14~19: None/Line/Total→[Top,Mid,Bottom] · First/Bottom→1라인 · TopSolo→1 · MidSolo→1 · MidBottom→2 · JungleOnly→[])
  pressable_tower = lines.iter().any(|l| cache.tower(enemy, l).is_some())   // tower = tower[l][enemy].or(tower2[l][enemy])
  if !pressable_tower { return false }
  // L293~296: 머릿수 압도면 세르펜 경합 저울(clear_win)로 바로 결정
  healthy_ally_count = cache.player_champion[team].iter().flatten().filter(|c| c.hp*100/c.stat_cached.hp > 39).count()   // stat_cached.hp==0 이면 div_by_zero 패닉(293:83)
  live_enemy_count   = cache.player_champion[enemy].iter().flatten().count()
  if healthy_ally_count > 2 && healthy_ally_count >= live_enemy_count + 2 {
    if v3_serpen_contest_clear_win(version, player, data, team_plan) { return false }   // 명확 승이면 미루지 않는다 (phi %88 [false,%236], m15.ll:52431)
  }
  // L302~303: 모든 유효 라인의 우리 미니언 수가 2 이상이어야 함
  if !valid_lines(context).iter().all(|l| data.blackboard[team].minion_state(l).minion_count > 1) { return true }   // 라인 미정리 → defer
  // L305~306
  (in_vision, out_vision) = serpen_reachable_enemy_count(player, data, team_plan)
  return in_vision + out_vision != 0     // IR: in_vision != -out_vision (0 - out_vision 과 ne) — 도달가능 적이 하나라도 있으면 defer
```

**`mem` 메모리 접근 20건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | team = %17 (m15.ll:51862). enemy_team = 1 - team (m15.ll:52091 `sub nuw nsw i64 1, %18`) | 4 | OK |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m15.ll:51848) | 4 | OK |
| 2 | OperationData | 0x8 | context | r | &GameContext (m15.ll:51900) | 4 | OK |
| 3 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — [team] 로 인덱스(stride 744B, m15.ll:52488) | 4 | OK |
| 4 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 포인터 (m15.ll:51851) | 4 | OK |
| 5 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable 슬롯 +0x28 tick / +0x40 get_game_mode / +0xf8 is_visible / +0x1f0 get_entity_by_id (divtable AbstractGame) | 3 | OK |
| 6 | AbstractGameWithCache | 0x180 | top_tower[] | r | cache.tower(enemy_team, line): base 0x180 + line*32 + enemy_team*8 → line0 top_tower/0x190 top_tower2, line1 0x1a0 mid_tower/0x1b0 mid_tower2, line2 0x1c0 bottom_tower/0x1d0 bottom_tower2. IR 은 +384(tower)·+400(tower2) 로 접힘 (m15.ll:52117~52128) | 4 | OK |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] — [team] 5칸(아군 건강 수, m15.ll:52141)·[enemy_team] 5칸(적 생존 수, m15.ll:52354) | 4 | OK |
| 8 | GameContext | 0x8 | setting | r | &GameSetting (m15.ll:51902) | 4 | OK |
| 9 | GameContext | 0x38 | tutorial | r | TutorialType 태그(1B). serpen_exists(rule_scope.rs:50→runner.rs:267 spawn_serpen) 와 valid_lines(rule_scope.rs:14~19) 두 곳에서 switch (m15.ll:51919, 52022) | 4 | OK |
| 10 | GameSetting | 0x12f8 | tick_per_second | r | tps (m15.ll:51904~51906) | 4 | OK |
| 11 | MobaMode | 0x240 | epic_minion_buff_time[team] | r | remain_epic_time(team) = epic_minion_buff_time[team] (game.rs:210, [usize;2] bounds check 2). +576 (m15.ll:51889~51891) | 4 | OK |
| 12 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.buf.inner.ptr | r | live_list.get(0) 의 원소 포인터 (+464, m15.ll:51969) | 4 | OK |
| 13 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | len==0 이면 세르펜 없음 (+472, m15.ll:51964~51966) | 4 | OK |
| 14 | TeamPlan | 0x88 | obj_spawn.serpen_camp_last_visible_tick | r | +136 (m15.ll:51929) | 4 | OK |
| 15 | GoalData | 0xc0 | serpen.epic_enemy_tick | r | +192, 지역명 enemy_kill_tick (m15.ll:51939) | 4 | OK |
| 16 | Entity | 0x5c0 | id | r | 세르펜 엔티티 id → is_visible(team, id) 인자 (+1472, m15.ll:51992) | 4 | OK |
| 17 | Entity | 0x628 | stat_cached.hp | r | 최대 hp (+1576). 세르펜 damaged 판정·아군 hp% 분모 (m15.ll:52001, 52170) | 4 | OK |
| 18 | Entity | 0x670 | hp | r | 현재 hp (+1648) (m15.ll:52003, 52183) | 4 | OK |
| 19 | Blackboard | 0x20 | top_minion_state.minion_count | r | minion_state(line).minion_count: line0 +0x0/ line1 +0x28 / line2 +0x50 의 BrainMinionParameter 안 +0x20 (i32). IR 은 +40/+80 선택 후 +32 (m15.ll:52490~52527) | 4 | OK |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 276 | 태그 | GameMode 태그 0 = Moba (as_moba, game.rs:231, tcxdict --enum GameMode) — 그리고 map_or 기본값 0 / remain_epic_time == 0 이면 return false | 3 |
| 1 | 2 | 210 | 임계 | epic_minion_buff_time [usize;2] bounds check(game.rs:210) — 배열 길이, 임계 아님 | 4 |
| 2 | 0 | 326 | 태그 | TutorialType 태그 0=None: serpen_exists 참 집합 {0,5,7,8} = None/MidBottom/Line/Total (runner.rs:267 spawn_serpen) | 4 |
| 3 | 5 | 326 | 태그 | TutorialType 태그 5=MidBottom (serpen_exists 참). 본문의 `shl i8 %109, 5` 는 line*32(tower 배열 stride) 접힘으로 별개 | 4 |
| 4 | 7 | 326 | 태그 | TutorialType 태그 7=Line (serpen_exists 참) | 4 |
| 5 | 8 | 326 | 태그 | TutorialType 태그 8=Total (serpen_exists 참) | 4 |
| 6 | 20 | 332 | 계수 | tps*20 = 20초. is_object_being_taken_by_enemy: goal_data.serpen.epic_enemy_tick <= tps*20 (ule) | 4 |
| 7 | 100 | 293 | 계수 | hp*100/stat_cached.hp — 백분율 환산 | 4 |
| 8 | 39 | 293 | 임계 | hp% > 39 (= ≥40%) 인 아군만 healthy. `icmp ugt %, 39` — 소스가 `>= 40` 인지 `> 39` 인지는 표기 불가(외연 동일) | 4 |
| 9 | 2 | 295 | 임계 | healthy_ally_count > 2 (즉 ≥3) AND healthy_ally_count >= live_enemy_count + 2 — 두 자리 모두 리터럴 2 | 4 |
| 10 | 1 | 303 | 임계 | blackboard[team].minion_state(line).minion_count > 1 (sgt i32) — 라인 미니언이 2기 이상이어야 '정리됨' 아님 | 4 |
| 11 | 1 | 284 | 임계 | enemy_team = 1 - team (`sub nuw nsw i64 1, %18`, m15.ll:52091) | 4 |
| 12 | 3 | 283 | 태그 | valid_lines 기본(Tutorial None/Line/Total) 슬라이스 길이 3 = 전 라인 (rule_scope.rs:14 phi [3,%91]) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 세르펜 '적이 치는 중' 판정 시간창 | objective_helpers.rs:332 | 20 | tps*20. 올리면 epic_enemy_tick 이 더 커도 '적이 치는 중'으로 봐 defer 를 더 자주 포기(false)한다 | 4 | 기존 |
| 1 | 세르펜 캠프 최근 가시 창 | objective_helpers.rs:327 | tps (1초) | serpen_camp_last_visible_tick + tps >= tick. 리터럴이 아니라 tps 그대로(계수 1 은 접힘) — 계수를 키우면 더 오래 전 시야도 '최근'으로 친다 | 4 | 기존 |
| 2 | 건강 아군 hp% 임계 | objective_helpers.rs:293 | 39 | hp% > 39. 올리면 healthy_ally_count 가 줄어 clear_win 경로(머릿수 압도) 진입이 줄어든다 | 4 | 기존 |
| 3 | 머릿수 압도 조건 | objective_helpers.rs:295 | 2 | healthy > 2 && healthy >= enemy + 2. 낮추면 clear_win 저울을 더 자주 묻는다 | 4 | 기존 |
| 4 | 라인 정리 판정 미니언 수 | objective_helpers.rs:303 | 1 | minion_count > 1 이 전 라인에서 성립해야 통과. 올리면 '라인 미정리→defer(true)' 가 더 자주 난다 | 4 | 기존 |

<details><summary>`callees` 피호출자 21건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | is_object_being_taken_by_enemy | game_ai::plan_legacy::team_plan::objective_helpers::is_object_being_taken_by_enemy | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:313 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 9 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 10 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 11 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | serpen_reachable_enemy_count | game_ai::plan_legacy::old::serpen_reachable_enemy_count | pub | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan) -> (usize, usize) | game-ai\src\plan_legacy\old\serpen.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | v3_epicops_defer_serpen | game_ai::plan_legacy::team_plan::objective_helpers::v3_epicops_defer_serpen | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:275 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | v3_serpen_contest_clear_win | game_ai::plan_legacy::old::v3_serpen_contest_clear_win | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan) -> bool | game-ai\src\plan_legacy\old\serpen.rs:52 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | valid_lines | game_ai::plan_legacy::rule_scope::valid_lines | pub | fn(&game_core::GameContext) -> &[game_core::LineType] | game-ai\src\plan_legacy\rule_scope.rs:13 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 1개**: `out_vision`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m09.ll:9873, m09.ll:14249) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L332 `enemy_kill_tick <= tps*20` — IR 은 goal_data.serpen.epic_enemy_tick 을 현재 tick 과 빼지 않고 리터럴로 tps*20 과 비교한다(m15.ll:52008~52009). 이 필드가 '경과 틱' 저장 방식인지 '절대 틱' 인지는 이 함수만으로 확정 불가(미탐색 = goal_data.rs:387 SerpenStanceData 갱신 지점 IR). 명세는 IR 그대로 적었다 | 4 |  |
| 1 | 표기 불가 | L293 `> 39` 가 소스에서 `>= 40` 인지 `> 39` 인지 — 외연 동일, 표기 불가 | 4 |  |
| 2 | 표기 불가 | L306 `in_vision != -out_vision` — 소스 표현이 `in_vision + out_vision > 0` 인지 `!= 0` 인지 표기 불가(usize 라 외연 동일). 반환 극성만 확정: 도달가능 적 합계가 0 이 아니면 true | 4 |  |
| 3 | 미탐색 | vtable 슬롯 이름은 divtable.py(ExpectedGame 구현 기준)로 붙였다 — +0x40 get_game_mode·+0x28 tick·+0xf8 is_visible·+0x1f0 get_entity_by_id. as_moba(game.rs:231)는 get_game_mode 결과 태그==0(Moba) 검사로 인라인됨 | 3 |  |
| 4 | 미탐색 | closure$1(L284 tower 술어)·closure$2(L293 hp% 술어)·closure$3(L303 minion_state 술어)는 전부 인라인(fnparts: DWARF 서브프로그램 19 vs define 1) — 별도 aux 없음 | 3 |  |
| 5 | 미탐색 | v3_serpen_contest_clear_win / serpen_reachable_enemy_count 내부는 안 봄(다른 함수 소유). _docs 주석: 후자 = '세르펜 캠프에 닿을 수 있는 적(가시 근처, 비가시 도달가능) 집합의 인원 셈', 전자 = '[v3 EPIC-OPS B-④] 세르펜 경합 교전 저울 — 도달 가능 적 상대로 명확 승(Commit)인가' | 4 |  |
| 6 | 미탐색 | exe 상수 0x1a1 의 정체 — IR 오프셋 목록에 417 이 없음(0x1a0 mid_tower+1? capstone 디스플레이스먼트 접힘 추정). 판정 무관 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

