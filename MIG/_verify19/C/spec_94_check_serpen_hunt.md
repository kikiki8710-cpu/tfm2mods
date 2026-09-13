---

### `94` check_serpen_hunt — 세르펜 사냥(Hunt) 진입 판정 — 압도적 머릿수/라인 압박 준비면 즉시 true, 캠프 근처 아군 교전 중이면 false, 아니면 캠프·바텀 사이드 머릿수를 판단오차 노이즈로 흔든 뒤 페널티/보너스 비교

| 항목 | 값 |
|---|---|
| id | `serpen__check_serpen_hunt` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen17check_serpen_hunt` |
| 소스 | `game-ai\src\plan_legacy\old\serpen.rs:297` |
| IR | `m05.ll` 45035~47631행 |
| 경로·가시성 | `game_ai::plan_legacy::old::check_serpen_hunt` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d61330` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | version | usize | penalty/bonus/v24 헬퍼에 전달만 | 4 |
| 1 | 1 | _rnd | &mut StdRng(320B) | 미사용(readnone) — 노이즈는 자체 시드 jrng 사용 | 4 |
| 2 | 2 | player | &PlayerState(2528B) | info.team(+0x930)·info.id(+0x928)·athlete(+0x180) | 4 |
| 3 | 3 | data | &OperationData(24B) |  | 4 |
| 4 | 4 | goal_data | &GoalData(248B) | serpen.epic_ally_tick(+0xd0, 소스명 hunt_tick) 만 읽음 | 4 |
| 5 | 5 | _plan | &BigPlan(384B) | 미사용(readonly, 본문 접근 없음) | 4 |
| 6 | 6 | team_plan | &TeamPlan(1064B) | vision.last_visible_pos/last_checked_ticks · obj_spawn.epic_giveup_tick | 4 |
| 7 | 7 | _debug | &mut DebugFrameData(224B) | v24_objective_setup_lane_pressure_ready 에 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
ctx = data.context; team = player.info.team; enemy = 1-team
L298 if !serpen_exists(ctx) → return false
L306 live_enemy = count(player_champion[enemy] Some)
L307 live_ally  = player_champion[team].iter_champions().filter(|c| c.hp*100/c.max_hp > 29).count()
L308 if live_enemy < 2 && live_ally > 2 → return true
L313 (cx,cy) = map.camp_pos(Serpen, team==0)
L314 near_camp_serpen_ally  = allies.filter(|c| dist_sq(c,camp) <= 120000^2 && hp% > 49).count()
L317 near_camp_serpen_enemy = enemies.filter(|c| dist_sq(c,camp) <= 120000^2 && hp% > 49 && blackboard[enemy].is_recent_visible(game, player, c)).count()
L320 near_serpen_ally  = allies.filter(|c| is_bottom_side(ctx,c.x,c.y) && hp% > 49).count()
     (is_bottom_side, map_regions.rs:28~30: (height - y <= x) || is_near_mid_line(ctx,x,y))
L324 if (0..5).any(|pos| player_champion[team][pos] 가 Some(c) && dist_sq(c,camp) <= 250000^2 && blackboard[team].in_battle(pos))
       [in_battle = matches!(big_goal[pos].1, Some(BigGoal::Battle{focus: Some(_)}))]
L326   → return false                                              // 캠프 근처 아군이 교전 중
L330 if team_plan.v24_objective_setup_lane_pressure_ready(version, player, data, goal_data, Serpen, debug) → return true
L334 hunt_tick = goal_data.serpen.epic_ally_tick
L337 near_serpen_enemy = (0..5).filter_map(|p| player_champion[enemy][p]).filter(|c|
L339     last_pos = team_plan.vision.last_visible_pos[p];
L340     d = distance(last_pos, camp).saturating_sub(60000);
L341     move_speed = c.stat_cached.move_speed;
L342     can_move = (tick.saturating_sub(team_plan.vision.last_checked_ticks[p]) + hunt_tick) * move_speed;
L344     hp% > 49 && can_move >= d
     ).count()
L347 is_morgard_giveup = team_plan.obj_spawn.epic_giveup_tick.is_some() && mode.jungle_runner.epic.live_list.len != 0
L348 judgement_penalty = macro_judgement_penalty(version, player)   // ceil(max(400-judge,0)/125)
L349 judgement_bonus   = macro_judgement_bonus(version, player)     // max(judge-400,0)/175 (objective_helpers.rs:21~23)
L351 judge = player.info.parameter.judge_accuracy()
L352 if judge <= 999 {
L355     bucket = tick / max(tps*2, 1)
L356     seed = (bucket << 40 | 94) ^ player.info.id;  rng = SplitMix(seed)
L357     near_camp_serpen_ally  = near_camp_serpen_ally  * error_ratio_noise(rng, judge) / 100
         near_camp_serpen_enemy = near_camp_serpen_enemy * error_ratio_noise(rng, judge) / 100
L359     near_serpen_ally       = near_serpen_ally       * error_ratio_noise(rng, judge) / 100
         near_serpen_enemy      = near_serpen_enemy      * error_ratio_noise(rng, judge) / 100
         (error_ratio_noise, utils.rs:493~496: span=(1000-judge)/20; 100 + rand[0, 2*span] - span → [100-span, 100+span] %)
     }
L361 if near_camp_serpen_ally > near_camp_serpen_enemy + judgement_penalty {
L362     return is_morgard_giveup || (near_serpen_ally + judgement_bonus >= near_serpen_enemy + judgement_penalty)
     }
     return false
```

**`mem` 메모리 접근 25건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r |  | 4 | OK |  |
| 1 | OperationData | 0x10 | blackboard | r | [1-team] is_recent_visible self / [team] in_battle(big_goal) | 4 | OK |  |
| 2 | GameContext | 0x38 | tutorial | r | serpen_exists | 4 | OK |  |
| 3 | GameContext | 0x8 | setting | r |  | 4 | OK |  |
| 4 | GameContext | 0x20 | map | r | camp_pos(Serpen, team==0) | 4 | OK |  |
| 5 | GameSetting | 0x12f8 | tick_per_second | r | bucket = tick / max(tps*2,1) (L355) | 4 | OK |  |
| 6 | GameSetting | 0x12c0 | height | r | is_bottom_side | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr(dyn) | r | vtable +0x28 tick / +0x40 get_game_mode | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion[team]/[1-team] | r |  | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 9 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | is_morgard_giveup: epic 생존 여부 | 4 | OK |  |
| 10 | TeamPlan | 0x50 | obj_spawn.epic_giveup_tick@tag | r | is_morgard_giveup | 4 | OK |  |
| 11 | TeamPlan | 0x230 | vision.last_visible_pos[i] | r | L339 | 4 | OK |  |
| 12 | TeamPlan | 0x2d0 | vision.last_checked_ticks[i] | r | L342 | 4 | OK |  |
| 13 | GoalData | 0xd0 | serpen.epic_ally_tick | r | 소스 변수 hunt_tick — can_move 에 더해지는 틱(L342) | 4 | OK |  |
| 14 | PlayerState | 0x930 | info.team | r |  | 4 | OK |  |
| 15 | PlayerState | 0x928 | info.id | r | 노이즈 시드 XOR(L356) | 4 | OK |  |
| 16 | PlayerState | 0x180 | info.parameter(AthleteParameter) | r | judge_accuracy(L351) | 4 | OK |  |
| 17 | Blackboard | 0xf8 | big_goal[pos].1@tag | r | in_battle(blackboard.rs:168): tag 5 = Battle | 4 | OK |  |
| 18 | Blackboard | 0x100 | big_goal[pos].1@Some.0@Battle.focus@tag | r | != 0 = focus Some | 4 | OK |  |
| 19 | Entity | 0x660 | x | r |  | 4 | OK |  |
| 20 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 21 | Entity | 0x628 | stat_cached.hp | r | 최대 HP(0 이면 div_by_zero 패닉) | 4 | OK |  |
| 22 | Entity | 0x670 | hp | r |  | 4 | OK |  |
| 23 | Entity | 0x640 | stat_cached.move_speed | r | L341 | 4 | OK |  |
| 24 | (local) jrng | 0x0 | seed | w | 스택 로컬 SplitMix 상태(utils.rs:177). 외부 상태 변경 없음 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | (bucket<<40 \| 94) ^ player.info.id |

**`consts` 상수 15건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 0 | 298 | 태그 | TutorialType {0,5,7,8}=serpen_exists. 밖이면 false | 4 |  |
| 1 | 29 | 307 | 임계 | live_ally = hp*100/max_hp > 29 (HP 30% 이상) 아군 수 | 4 |  |
| 2 | 100 | 307 | 계수 | HP 백분율 | 4 |  |
| 3 | 2 | 308 | 임계 | live_enemy < 2 && live_ally > 2 → 즉시 true | 4 |  |
| 4 | 5 | 313 | 태그 | JungleType::Serpen (camp_pos · v24 헬퍼 인자) | 4 |  |
| 5 | 14400000001 | 314 | 임계 | 120000^2 + 1 — 캠프 근처 판정 제곱거리(near_camp_*, L314·L317) | 4 |  |
| 6 | 49 | 315 | 임계 | hp% > 49 = HP 50% 이상 (L315·318·321·344 공통) | 4 |  |
| 7 | 62500000001 | 325 | 임계 | 250000^2 + 1 — 캠프 반경 250000 안의 아군이 in_battle 이면 false | 4 |  |
| 8 | 5 | 325 | 태그 | BigGoal::Battle 태그(in_battle 인라인, blackboard.rs:168) | 4 |  |
| 9 | 60000 | 340 | 계수 | d = distance(last_visible_pos, camp).saturating_sub(60000) | 4 |  |
| 10 | 999 | 352 | 임계 | judge_accuracy <= 999 이면 노이즈 적용(1000 이상 = 오차 없음) | 4 |  |
| 11 | 1 | 355 | 길이 | shl 1 = tps*2 (노이즈 버킷 길이 2초) · max(…,1) | 4 | 2 |
| 12 | 40 | 356 | 계수 | seed = (bucket << 40 \| 94) ^ player.info.id | 4 |  |
| 13 | 94 | 356 | 태그 | 시드 태그 상수(세르펜 hunt 전용 솔트로 추정) | 5 |  |
| 14 | 100 | 357 | 계수 | count = count * error_ratio_noise(rng, judge) / 100 (sdiv) | 4 |  |

**`knobs` 조정점 8건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 즉시 true 머릿수 | serpen.rs:308 | 2 | live_enemy<2 && live_ally>2. 내리면(적 상한↑) 더 자주 즉시 사냥 | 4 | 기존 |
| 1 | live_ally HP 컷 | serpen.rs:307 | 29 | 올리면 즉시 true 조건이 엄격 | 4 | 기존 |
| 2 | 캠프 근처 반경 | serpen.rs:314/317 | 14400000001 | 120000^2. 키우면 near_camp 양쪽 증가 | 4 | 기존 |
| 3 | 교전 중 아군 탐지 반경 | serpen.rs:325 | 62500000001 | 250000^2. 키우면 더 먼 교전에도 false | 4 | 기존 |
| 4 | 비가시 적 도달 여유 | serpen.rs:340 | 60000 | 키우면 적이 도달가능으로 더 잘 잡혀 near_serpen_enemy 증가 → 사냥 보수적 | 4 | 기존 |
| 5 | 노이즈 버킷 | serpen.rs:355 | 2 | tps*2. 키우면 노이즈 갱신 주기 길어짐(같은 판정 유지) | 4 | 기존 |
| 6 | 노이즈 상한 judge | serpen.rs:352 | 999 | judge_accuracy 1000 이상이면 노이즈 0 | 4 | 기존 |
| 7 | 판단 보너스 분모 | objective_helpers.rs:22 (별도 함수) | 175 | judge 400 초과분/175 이 near_serpen_ally 에 가산 | 4 | 기존 |

<details><summary>`callees` 피호출자 13건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | error_ratio_noise | game_ai::error_ratio_noise | pub | fn(&mut game_core::NoiseRng, usize) -> usize | game-ai\src\utils.rs:492 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | in_battle | game_core::Blackboard::in_battle | pub | fn(&game_core::Blackboard, usize) -> bool | game-core\src\simulation\game\blackboard.rs:167 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | is_bottom_side | game_core::is_bottom_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:27 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | judge_accuracy | game_core::AthleteParameter::judge_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:337 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | macro_judgement_bonus | game_ai::plan_legacy::team_plan::macro_judgement_bonus | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:20 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | macro_judgement_penalty | game_ai::plan_legacy::team_plan::macro_judgement_penalty | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:15 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | v24_objective_setup_lane_pressure_ready | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `ceil`, `dist_sq`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:32180) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L356 시드 솔트 94 의 의미(다른 판정과 시드를 가르기 위한 상수로 추정) | 5 |  |
| 1 | 미탐색 | 노이즈 곱은 sdiv(부호 있는 나눗셈) — error_ratio_noise 가 음수를 낼 수 있는지(judge 0 → span 50 → 최소 50%) 확인: 범위 [100-span,100+span] 이라 음수 없음. sdiv 는 타입 추론 결과일 뿐 | 4 |  |
| 2 | 미탐색 | v24_objective_setup_lane_pressure_ready 내부(objective_discipline.rs)는 담당 밖 | 4 |  |
| 3 | 표기 불가 | `> 49`/`> 29`/`<= 999` 는 소스 표기(`>= 50` 등) 표기 불가 | 4 |  |
| 4 | 미탐색 | _plan(&BigPlan) 은 readonly 이지만 본문에 접근 없음 — 시그니처 호환용으로 추정 | 5 |  |
| 5 | 미탐색 | vtable 간접호출(tick +0x28 / get_game_mode +0x40)은 C2 대조 불가라 logic 에만 적음 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | GoalData.serpen.epic_ally_tick(+0xd0) 의 의미 — 소스 변수명은 hunt_tick, 필드명은 epic 계열 복제(SerpenStanceData 가 EpicStanceData 와 동일 필드명). '세르펜을 잡는 데 걸리는 예상 틱'으로 추정, 미확정 | 5 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

