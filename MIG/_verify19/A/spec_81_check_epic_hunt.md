---

### `81` check_epic_hunt — 에픽(모가드) 사냥을 지금 해도 되는가 — 캠프 주변 아군/적 전력·도달 가능 적을 세어 판정

| 항목 | 값 |
|---|---|
| id | `epic__check_epic_hunt` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic15check_epic_hunt` |
| 소스 | `game-ai\src\plan_legacy\old\epic.rs:168` |
| IR | `m09.ll` 54362~57079행 |
| 경로·가시성 | `game_ai::plan_legacy::old::check_epic_hunt` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `de5340` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | version | usize | 본문 분기 없음. macro_judgement_penalty/bonus(L232-233)에 전달. ⚠v24_objective_setup_lane_pressure_ready(L208)에는 poison 전달 | 4 |
| 1 | 1 | _rnd | &mut StdRng | readnone — 미사용 | 4 |
| 2 | 2 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0, 디버그만)·info.id(+0x928, 노이즈 시드)·info.parameter(+0x180, judge_accuracy) | 4 |
| 3 | 3 | data | &OperationData(24B) | +0x0 cache / +0x8 context / +0x10 blackboard[2] | 4 |
| 4 | 4 | goal_data | &GoalData(248B) | epic.epic_enemy_tick(+0x88)·epic.epic_ally_tick(+0x98) | 4 |
| 5 | 5 | _plan | &BigPlan | readonly captures(none) — 미사용 | 4 |
| 6 | 6 | team_plan | &TeamPlan | obj_spawn.epic_camp_last_visible_tick(+0x80)·vision.last_visible_pos[p](+0x230)·vision.last_checked_ticks[p](+0x2d0) | 4 |
| 7 | 7 | debug | &mut DebugFrameData(224B) | ctx.debug 일 때만 infos(+0xa0) 에 4개 계수 문자열 push | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
ctx = data.context; if !spawn_epic(ctx.tutorial) (태그∈1..6) → return false                              [L169]
enemy_epic_tick = goal_data.epic.epic_enemy_tick                                                     [L176]
if !(enemy_epic_tick > tps) && !(team_plan.obj_spawn.epic_camp_last_visible_tick + tps < game.tick()) → return true   [L177-178]
   (= 적 에픽 틱이 tps 이하이고 캠프를 1초 안에 본 상태면 바로 true)
enemy_count = |{p: player_champion[1-team][p] 존재}|                                                   [L184]
live_ally  = |{아군 e: e.hp*100/e.max_hp > 29}|                                                        [L185]
if enemy_count < 2 && live_ally > 2 → return true                                                     [L186]
camp = map.camp_pos(Morgard, team==0)                                                                 [L191]
near_camp_epic_ally  = |{아군 e: dist²(e,camp) < 120000² && HP% > 49}|                                [L192-193]
near_camp_epic_enemy = |{적 e: dist²(e,camp) < 120000² && HP% > 49 && bb[1-team].is_recent_visible(game,player,e)}|  [L195-196]
near_epic_ally = |{아군 e: (!is_top_side(ctx,e.x,e.y) [height−y < x] || is_near_mid_line(ctx,e.x,e.y)) && HP% > 49}|  [L198-199]
if any 아군 p: dist²(e,camp) < 250000² && bb[team].in_battle(p) == Some(Battle{focus}) && focus.is_some() → return false  [L202-204]
if team_plan.v24_objective_setup_lane_pressure_ready(player, data, goal_data, Morgard, debug) → return true   [L208]
ally_tick = goal_data.epic.epic_ally_tick                                                             [L212]
near_epic_enemy = Σ_{적 p, e 존재} [ last_pos = vision.last_visible_pos[p]; d = distance(last_pos,camp).sat_sub(60000)   [L215-218]
     can_move = ((tick − vision.last_checked_ticks[p]).sat_sub + ally_tick) * e.move_speed             [L219-220]
     HP% > 49 && can_move >= d ]   (⚠시야 조건 없음 — passive_plan 쪽과 다름)                            [L222]
if ctx.debug → debug.infos[my_champ.id].push("near_camp_epic_ally: A, near_camp_epic_enemy: B, near_epic_ally: C, near_epic_enemy: D")   [L225-228]
penalty = macro_judgement_penalty(version, player); bonus = macro_judgement_bonus(version, player)   [L232-233]
acc = player.info.parameter.judge_accuracy()                                                          [L237]
if acc > 999 { (A,B,C,D) 그대로 } else { seed = ((tick / max(tps*2,1)) << 40 | 233) ^ player.id; 각각 X = error_ratio_noise(&seed, acc) * X / 100 }   [L238-245]
return (A > 2 || D == 0) && (penalty + B) < A && (bonus + C) >= (penalty + D)                          [L247-249]
   (L247 두 조건의 줄 안 순서는 column 부재로 미확정; L249 는 L247 이 참일 때만 평가)
```

**`mem` 메모리 접근 30건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — [team] in_battle(L203), [1-team] is_recent_visible(L196) | 4 | OK |  |
| 3 | GameContext | 0x38 | tutorial | r | spawn_epic(runner.rs:263)→morgard_exists(rule_scope.rs:46) 인라인 — 태그∈{1..6}이면 false | 4 | OK |  |
| 4 | GameContext | 0x3b | debug | r | bool(+59). true 면 L227-228 디버그 로그 | 4 | OK |  |
| 5 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 6 | GameContext | 0x20 | map | r | &MapDef → camp_pos(Morgard, team==0) (L191) | 4 | OK |  |
| 7 | GameSetting | 0x12f8 | tick_per_second | r | L177/178 tps, L241 tps*2 | 4 | OK |  |
| 8 | GameSetting | 0x12c0 | height | r | is_top_side(map_regions.rs:22,24) 인라인: height − y < x | 4 | OK |  |
| 9 | GoalData | 0x88 | epic.epic_enemy_tick | r | L176 enemy_epic_tick | 4 | OK |  |
| 10 | GoalData | 0x98 | epic.epic_ally_tick | r | L212 — 적 can_move 계산에 더해지는 틱(L220) | 4 | OK |  |
| 11 | TeamPlan | 0x80 | obj_spawn.epic_camp_last_visible_tick | r | L178 | 4 | OK |  |
| 12 | TeamPlan | 0x230 | vision.last_visible_pos[i] | r | 적 p 마지막 관측 좌표 +16p (L217) | 4 | OK |  |
| 13 | TeamPlan | 0x2d0 | vision.last_checked_ticks[i] | r | +8p (L220) | 4 | OK |  |
| 14 | PlayerState | 0x930 | info.team | r |  | 4 | OK |  |
| 15 | PlayerState | 0x9c0 | info.position@tag | r | 디버그 로그 키(내 챔프 id 조회)만 | 4 | OK |  |
| 16 | PlayerState | 0x928 | info.id | r | L242 노이즈 시드 xor | 4 | OK |  |
| 17 | PlayerState | 0x180 | info.parameter | r | &AthleteParameter → judge_accuracy() (L237) | 4 | OK |  |
| 18 | AbstractGameWithCache | 0x1e0 | player_champion | r | [2][5] Option<&Entity> | 4 | OK |  |
| 19 | Entity | 0x660 | x | r |  | 4 | OK |  |
| 20 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 21 | Entity | 0x628 | stat_cached.hp | r | 최대 HP(0이면 div_by_zero 패닉) | 4 | OK |  |
| 22 | Entity | 0x670 | hp | r | 현재 HP | 4 | OK |  |
| 23 | Entity | 0x640 | stat_cached.move_speed | r | L219 | 4 | OK |  |
| 24 | Entity | 0x5c0 | id | r | 디버그 로그 키(L228) | 4 | OK |  |
| 25 | Blackboard | 0xf8 | big_goal[p].1@tag | r | in_battle(blackboard.rs:168) 인라인: Some(BigGoal::Battle) = 태그 5, stride 32B | 4 | OK |  |
| 26 | Blackboard | 0x100 | big_goal[p].1@Battle.focus@tag | r | Option<usize> 태그(i64) — L204 trunc→bool: focus.is_some() | 4 | OK |  |
| 27 | DebugFrameData | 0xa0 | infos | r | HashMap<usize,Vec<String>> — 쓰기 | 4 | OK |  |
| 28 | AbstractGame(vtable) | 0x28 | tick | r | divtable AbstractGame 0x28 | 3 | 오귀속(사전은 다른 필드를 준다) |  |
| 29 | DebugFrameData | 0xa0 | infos | w | ctx.debug(+0x3b) 일 때만 (L225-228). 판정에 영향 없음 | 4 | OK | push format!("near_camp_epic_ally: {}, near_camp_epic_enemy: {}, near_epic_ally: {}, near_epic_enemy: {}") |

**`consts` 상수 17건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 6 | 169 | 임계 | spawn_epic 인라인: (tutorial_tag−1) <u 6 ⇔ 태그∈{1..6} → 에픽 없음 → return false | 4 |  |
| 1 | 100 | 185 | 계수 | hp*100/max_hp — HP 퍼센트 (L185·193·196·199·222 공통) | 4 |  |
| 2 | 29 | 185 | 임계 | live_ally = 아군 중 HP% > 29(=30% 이상) 수 | 4 |  |
| 3 | 2 | 186 | 임계 | enemy_count(적 챔프 존재 수) < 2 && live_ally > 2 → return true (압도 상황) | 4 |  |
| 4 | 4 | 191 | 미상 | JungleType::Morgard(4) — camp_pos·v24_objective_setup_lane_pressure_ready(L208) 인자 | 4 |  |
| 5 | 14400000001 | 192 | 임계 | 120000^2+1 — 캠프 반경 120000 이내 판정(L192 아군·L195 적) | 4 |  |
| 6 | 49 | 193 | 임계 | HP% > 49(=50% 이상) — near_camp_epic_ally(L193)·near_camp_epic_enemy(L196)·near_epic_ally(L199)·near_epic_enemy(L222) 공통 | 4 |  |
| 7 | 62500000001 | 203 | 임계 | 250000^2+1 — 캠프 250000 이내 아군이 Battle{focus:Some} 이면 return false | 4 |  |
| 8 | 5 | 203 | 태그 | BigGoal::Battle 메모리태그 5 — blackboard[team].in_battle(p) 인라인(blackboard.rs:168) | 4 |  |
| 9 | 60000 | 218 | 계수 | 적 도달 판정: d = distance(last_pos, camp).saturating_sub(60000) — 캠프 60000 이내면 이미 도달로 봄 | 4 |  |
| 10 | 999 | 238 | 임계 | judge_accuracy > 999 이면 노이즈 없이 원값(L239), 아니면 error_ratio_noise 로 4개 계수 각각 흔듦(L243-245) | 4 |  |
| 11 | 1 | 241 | 인덱스 | tps*2 — `shl i64 %tps, 1` 로 접힘. 노이즈 시드 = tick / max(tps*2, 1) (2초마다 같은 노이즈) | 4 | 2 |
| 12 | 40 | 242 | 계수 | 시드 = ((tick/(tps*2)) << 40 \| 233) ^ player.info.id | 4 |  |
| 13 | 233 | 242 | 미상 | 시드 하위 상수(에픽 판정 식별 솔트로 추정) | 5 |  |
| 14 | 100 | 243 | 계수 | noisy = error_ratio_noise(seed, accuracy) * 값 / 100 (sdiv) | 4 |  |
| 15 | 2 | 247 | 임계 | near_camp_epic_ally > 2 \|\| near_epic_enemy == 0 (1차 조건 중 하나) | 4 |  |
| 16 | 0 | 247 | 태그 | near_epic_enemy == 0 — 도달 가능 적 없음 | 4 |  |

**`knobs` 조정점 10건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 조기 true: 적 에픽 틱 상한 | epic.rs:177 | 1 | tps*1. enemy_epic_tick ≤ tps 이고 캠프를 1초 내 봤으면 무조건 true — 두 tps 계수(리터럴 없음, tps 그대로)를 키우면 조기 true 가 넓어진다 | 4 | 기존 |
| 1 | 생존 아군 HP 하한 | epic.rs:185 | 29 | 올리면 live_ally 가 줄어 L186 조기 true 가 드물어진다 | 4 | 기존 |
| 2 | 캠프 근접 반경(아군/적 계수) | epic.rs:192,195 | 14400000001 | 120000². 올리면 A·B 가 함께 커진다(A−B 차이가 판정이라 효과는 배치 의존) | 4 | 기존 |
| 3 | 계수 HP 하한 | epic.rs:193,196,199,222 | 49 | 내리면 저체력도 세어 A·B·C·D 모두 증가 | 4 | 기존 |
| 4 | 아군 교전중 회피 반경 | epic.rs:203 | 62500000001 | 250000². 올리면 더 먼 아군 교전에도 false 로 접는다 | 4 | 기존 |
| 5 | 적 도달 여유 반경 | epic.rs:218 | 60000 | 올리면 적이 더 쉽게 '도달 가능'으로 계수(D↑) → 판정 어려워짐 | 4 | 기존 |
| 6 | 노이즈 면제 정확도 | epic.rs:238 | 999 | judge_accuracy > 999 면 노이즈 없음. 내리면 더 많은 선수가 정확 판정 | 4 | 기존 |
| 7 | 노이즈 시드 갱신 주기 | epic.rs:241 | 2 | tps*2(folded shl 1). 올리면 같은 노이즈가 더 오래 유지 | 4 | 기존 |
| 8 | 1차 조건 아군 수 | epic.rs:247 | 2 | A > 2. 내리면 아군 2명만 캠프 근처여도 (D==0 없이) 판정 가능 | 4 | 기존 |
| 9 | 최종 비교 penalty/bonus | epic.rs:247-249 | 0 | macro_judgement_penalty/bonus(다른 배치)가 B·D 에 penalty, C 에 bonus 로 더해진다 — penalty↑ = 사냥 보수적 | 4 | 기존 |

<details><summary>`callees` 피호출자 18건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | error_ratio_noise | game_ai::error_ratio_noise | pub | fn(&mut game_core::NoiseRng, usize) -> usize | game-ai\src\utils.rs:492 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | in_battle | game_core::Blackboard::in_battle | pub | fn(&game_core::Blackboard, usize) -> bool | game-core\src\simulation\game\blackboard.rs:167 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_top_side | game_core::is_top_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:21 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | judge_accuracy | game_core::AthleteParameter::judge_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:337 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | macro_judgement_bonus | game_ai::plan_legacy::team_plan::macro_judgement_bonus | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:20 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | macro_judgement_penalty | game_ai::plan_legacy::team_plan::macro_judgement_penalty | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:15 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 11 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 12 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 13 | spawn_epic | game_core::TutorialType::spawn_epic | pub | fn(&game_core::TutorialType) -> bool | game-core\src\simulation\game\runner.rs:262 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | v24_objective_setup_lane_pressure_ready | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 5개**: `format_inner`, `or_insert`, `push_mut`, `rustc_entry`, `sat_sub`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:31363) · **형제 0개** 

**`open` 9건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L177-178 의 조기 true 의 소스 표현: IR = `enemy_epic_tick > tps` 가 거짓이고 `last_visible+tps < tick` 도 거짓이면 true. 두 비교의 결합이 `if a <= tps && last+tps >= tick { return true }` 인지 `if !(a > tps \|\| ...)` 인지는 외연 동일(표기 불가) | 4 |  |
| 1 | 미탐색 | epic_enemy_tick / epic_ally_tick(GoalData+0x88/+0x98)의 생산 의미(누가 언제 갱신) — _docs 0건, GoalData 갱신처 미탐색. 본 명세는 소비 방식만(L177 비교·L220 가산) | 4 |  |
| 2 | 미탐색 | is_top_side(map_regions.rs:22-24) = `height − y < x` 로 읽힘. 좌표계(y 아래 증가?) 해석은 미확인 — 식은 IR 정본 | 3 |  |
| 3 | 표기 불가 | near_epic_ally(L198-199) 의 `!is_top_side \|\| is_near_mid_line` 결합: IR 분기 = is_top_side 참일 때만 is_near_mid_line 검사(단락). 소스가 `!(is_top_side && !is_near_mid_line)` 인지는 표기 불가 | 4 |  |
| 4 | 미탐색 | error_ratio_noise(m04.ll:49631)·macro_judgement_penalty/bonus(m15.ll:51803/51819)·judge_accuracy 내부는 담당 밖(다른 배치/‘한 번 확정해 재사용’ 대상). 반환 범위 힌트: penalty i32 range(0,12271336), bonus i32 range(-17179869,17179870) | 4 |  |
| 5 | 미탐색 | L242 상수 233·<<40 의 의도(판정 종류별 시드 솔트)는 추정 | 5 |  |
| 6 | 미탐색 | is_recent_visible 내부(120틱=2초 유예, _gcbc g07.ll:157005~157049: is_visible(team,id) \|\| last_visible[pos]+120 >= tick) — 범위 밖이라 constants 미등재 | 4 |  |
| 7 | 미탐색 | exe 대조: fnprobe 0xde5340 — 패닉 Location epic.rs:185/193/196/199/222·simulation.rs:1905 일치, 상수 0xea60(60000)·0x1e0·0x2e8(Blackboard 744) 일치. ⚠exe consts 에 0x384(900)·-300/-299(0xfffffed4/5)·0x1869f 등이 있으나 IR 본문엔 없음 → 인라인된 콜리(judge_accuracy/error_ratio_noise 추정) 상수로 봄, 미확인. 999(0x3e7) 는 fnprobe 목록에 안 보임 — 비교 형태 변형(cmp 1000; jb) 추정, 미확인 | 4 |  |
| 8 | 표기 불가 | L204 `trunc nuw i64 → i1` 은 Option<usize>::is_some() 판별(태그 0/1). in_battle 의 반환형이 Option<Option<usize>> 인지 &Option<BigGoal> 매칭인지는 표기 불가 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

