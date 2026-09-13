---

### `97` SerpenHuntAndPokePlan::sub_plan — 세르펜 사냥/견제 빅플랜의 서브플랜 선택 — Epic 판과 동형(Steal/SerpenCheck/Recall/LineDefense/SerpenHunt/SerpenPoke), 차이 5곳은 unknown/logic 에 명시

| 항목 | 값 |
|---|---|
| id | `serpen_hunt_and_poke__sub_plan` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen13hunt_and_pokeNtB2_21SerpenHuntAndPokePlan8sub_plan` |
| 소스 | `game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:30` |
| IR | `m10.ll` 9876~11480행 |
| 경로·가시성 | `game_ai::plan_legacy::old::EpicHuntAndPokePlan::sub_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `df0e90` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut SubPlan(72B) | %0. 니치 태그(idx+2) | 4 |
| 1 | 1 | self | &mut SerpenHuntAndPokePlan(32B) | %1. v46_flee_threats(+0x0 Vec)·focus_serpen_only(+0x18)·vision_only(+0x19)·v46_flee(+0x1a). v46 필드는 갱신도 | 4 |
| 2 | 2 | version | usize | %2. 본문 분기 없음 — upgrade_item·v46_flee_gate_check·v25·v24 에 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | %3. upgrade_item·PlayerState::strategy 에 전달 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | %4. info.team(0x930)·info.position 태그(0x9c0) | 4 |
| 5 | 5 | data | &OperationData(24B) | %5. cache·context·blackboard | 4 |
| 6 | 6 | goal_data | &GoalData(248B) | %6. serpen.epic_enemy_tick(0xc0)·serpen.epic_ally_tick(0xd0) — check_recall 인라인분 | 4 |
| 7 | 7 | team_plan | &TeamPlan(1064B) | %7. objective 태그(0x41f)·phase(0x420) + v24 의 self | 4 |
| 8 | 8 | debug | &mut DebugFrameData(224B) | %8. ctx.debug 일 때 v46 도주 로그 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// serpen/hunt_and_poke.rs:30~92 (+ check_recall :99~159 인라인). Epic 판(epic_hunt_and_poke__sub_plan.json)과 같은 골격 — ★표시가 차이.
t, pos, champ = … (:32)
// A. 스틸 (:35~44)
if self.focus_serpen_only || self.vision_only {
   serpen = as_moba().jungle_runner.serpen.live_list.first().and_then(get_entity_by_id)   // :36~37 (MobaMode+0x1d0/+0x1d8)
   Some → Steal{ last_vision_tick:0, target:Some(Serpen) ★, commit:focus_serpen_only }   // :39
   None → SerpenCheck{false}                                                            // :44
}
// B. 일반
hp_ratio = hp*100/max_hp                                                                 // :47
serpen = live_list.first().and_then(get_entity_by_id); None → SerpenCheck{false}         // :48~50
(x0,y0,x1,y1) = map.fountains[t]; is_in_heal_area = …                                    // :55~56
can_upgrade_item = upgrade_item(version, rnd, player, game, ctx).is_some()               // :57 (★Epic 은 :48 에서 먼저 계산 — 순서만 다르고 값 동일)
if serpen.hp == serpen.max_hp && (hp_ratio<51 || can_upgrade_item || (is_in_heal_area && hp<max_hp)) → Recall   // :58~59
// C. 오브젝트 게이트
if team_plan.objective != Some(Serpen{..}) → Recall                                       // :63 / :90 ★태그 1
if phase == Hunt {                                                                        // :64
   strategy = player.strategy(rnd, game)                                                  // :65
   if object_buildup == Split(pos) && v25_objective_splitter_can_stay(version, player, data, Serpen ★5)   // :66~68
      → LineDefense{ Aggressive, fallback_line(ctx, Top ★0), Push }                       // :70~73
   else → SerpenHunt{ need_recall:false }                                                 // :77
}
// D. check_recall (:99~159)
   heal_area(t).contains(champ) && hp<max → true                                          // :103~104
   hp_ratio = hp*100/max                                                                  // :108
   min(goal_data.serpen.epic_enemy_tick, epic_ally_tick) <= tps*5 → false                 // :111
   v46 도주 블록 — Epic 판과 동일(:119~135, 위협 반경 200000, gate 코드 0, 로그)
   ★serpen = game.jungle_runner()(vtable+0xe0).serpen.live_list.first().and_then(get_entity_by_id)   // :142~143 (JungleRunner+0x1b8/+0x1c0)
   ★if serpen.is_none() → true(귀환)                                                       // :143 (Epic 판엔 이 조회 없음)
   ★enemy_cnt = player_champion[1-t] 중 blackboard[1-t].is_recent_visible && dist_sq(e, serpen) < 22500000001 인 수   // :148~151 closure$2
   ★ally_cnt  = player_champion[t]   중 blackboard[t].is_recent_visible   && dist_sq(a, serpen) < 22500000001 인 수   // :153~156 closure$3  (Epic 은 regions==7 셀)
   return hp_ratio < 21 && ally_cnt < enemy_cnt                                            // :159
if check_recall → Recall                                                                  // :79~80
// E. 꼬리
if team_plan.v24_objective_setup_should_check_camp(version, player, data, goal_data, Serpen ★5) → SerpenCheck{false}   // :82~83
★else → SerpenPoke (거리 게이트 없음 — Epic 판은 dist<150000 일 때만 Poke, 아니면 Check)   // :86
```

**`mem` 메모리 접근 42건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | SerpenHuntAndPokePlan | 0x18 | focus_serpen_only | r | Steal commit 플래그(:35, :39) | 4 | OK |  |
| 1 | SerpenHuntAndPokePlan | 0x19 | vision_only | r | Steal Lurk 플래그 | 4 | OK |  |
| 2 | SerpenHuntAndPokePlan | 0x1a | v46_flee | r | check_recall :119 | 4 | OK |  |
| 3 | SerpenHuntAndPokePlan | 0x8 | v46_flee_threats.ptr | r | :120 순회 | 4 | OK |  |
| 4 | SerpenHuntAndPokePlan | 0x10 | v46_flee_threats.len | r | :120 순회 / :126 clear | 4 | OK |  |
| 5 | PlayerState | 0x930 | info.team | r | t. 적 팀 1-t | 4 | OK |  |
| 6 | PlayerState | 0x9c0 | info.position@tag | r | 내 포지션(i32) — player_champion 조회·Split(position) 비교(:66) | 4 | OK |  |
| 7 | OperationData | 0x0 | cache | r |  | 4 | OK |  |
| 8 | OperationData | 0x8 | context | r |  | 4 | OK |  |
| 9 | OperationData | 0x10 | blackboard | r | [1-t] 적 카운트 · [t] 아군 카운트 | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x0 | game.data_ptr | r |  | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 +0x28 tick · +0x40 get_game_mode · +0xe0 jungle_runner(★Epic 판엔 없음) · +0x1f0 get_entity_by_id | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x1e0 | player_champion | r | [t][pos] 내 챔프 / [1-t][0..5] 적 / [t][0..5] 아군(5회 언롤) | 4 | OK |  |
| 13 | GameContext | 0x8 | setting | r | → tick_per_second | 4 | OK |  |
| 14 | GameContext | 0x20 | map | r | → fountains 만(★regions 는 안 읽음 — Epic 판과 차이) | 4 | OK |  |
| 15 | GameContext | 0x3b | debug | r | v46 로그 게이트 | 4 | OK |  |
| 16 | GameSetting | 0x12f8 | tick_per_second | r | check_recall :111 tps*5 | 4 | OK |  |
| 17 | MapDef | 0x6d70 | fountains | r | (x0,y0,x1,y1). is_in_heal_area(:55~56) | 4 | OK |  |
| 18 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r | :36/:48 — as_moba 경유(+464) | 4 | OK |  |
| 19 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | 0 이면 세르펜 없음(+472) | 4 | OK |  |
| 20 | JungleRunner | 0x1b8 | serpen.live_list.ptr | r | check_recall :142~143 — vtable+0xe0 jungle_runner() 경유(+440). MobaMode+0x18 부터가 JungleRunner 라 0x1d0-0x18 | 4 | OK |  |
| 21 | JungleRunner | 0x1c0 | serpen.live_list.len | r | (+448) | 4 | OK |  |
| 22 | TeamPlan | 0x41f | objective | r | 1 = Some(Serpen) 만 통과(:63) — Epic 판은 0(Morgard) | 4 | OK |  |
| 23 | TeamPlan | 0x420 | objective@Some.0@Serpen.phase | r | 3 = Hunt | 4 | OK |  |
| 24 | GoalData | 0xc0 | serpen.epic_enemy_tick | r | check_recall :111 (필드명은 epic_* 그대로 재사용) | 4 | OK |  |
| 25 | GoalData | 0xd0 | serpen.epic_ally_tick | r |  | 4 | OK |  |
| 26 | Entity | 0x628 | stat_cached.hp | r | 내/세르펜 max HP (0 → div_by_zero 패닉) | 4 | OK |  |
| 27 | Entity | 0x660 | x | r |  | 4 | OK |  |
| 28 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 29 | Entity | 0x670 | hp | r |  | 4 | OK |  |
| 30 | Strategy | 0x0 | object_buildup | r | Split(position) 니치 — i32 그대로 내 pos 와 비교(:66) | 4 | OK |  |
| 31 | SubPlan | 0x0 | tag | w | Steal=18 / SerpenCheck=13 / Recall=5 / LineDefense=2 / SerpenHunt=14 / SerpenPoke=15 | 4 | OK | 18\|13\|5\|2\|14\|15 |
| 32 | SubPlan | 0x8 | Steal.last_vision_tick | w | :39 | 4 | OK | 0 |
| 33 | SubPlan | 0x10 | Steal.target | w | StealTarget 태그 1 = Serpen (Epic 판은 0) | 4 | OK | Some(Serpen)=1 |
| 34 | SubPlan | 0x11 | Steal.commit | w |  | 4 | OK | self.focus_serpen_only |
| 35 | SubPlan | 0x8 | SerpenCheck.move_check | w | :44 :50 :83 | 4 | OK | false(0) |
| 36 | SubPlan | 0x8 | SerpenHunt.need_recall | w | :77 | 4 | OK | false(0) |
| 37 | SubPlan | 0x8 | LineDefense.style | w | :73 | 4 | OK | Aggressive(0) |
| 38 | SubPlan | 0x9 | LineDefense.line | w | :70 ★Epic 판은 Bottom(2) | 4 | OK | fallback_line(ctx, Top=0) |
| 39 | SubPlan | 0xa | LineDefense.minion_action_type | w |  | 4 | OK | Push(2) |
| 40 | SerpenHuntAndPokePlan | 0x1a | v46_flee | w |  | 4 | OK | true(:131) / false(:126) |
| 41 | SerpenHuntAndPokePlan | 0x0 | v46_flee_threats | w |  | 4 | OK | gate.1.to_vec()(:132) / len=0(:126) |

**`consts` 상수 18건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 36 | 태그 | get_game_mode 반환 tag 0 = Moba(as_moba). / :70 fallback_line(ctx, LineType 0 = Top) ★Epic 판은 Bottom(2). / :130 v46 gate 코드 0 = 도주 발동 | 4 |
| 1 | 100 | 47 | 계수 | hp_ratio = hp*100/max_hp | 4 |
| 2 | 13 | 50 | 태그 | SubPlan 메모리태그 13 = SerpenCheck | 4 |
| 3 | 51 | 58 | 임계 | hp_ratio < 51 → Recall 후보(세르펜 풀피 조건과 결합) | 4 |
| 4 | 5 | 59 | 태그 | SubPlan 메모리태그 5 = Recall. / :68·:82 JungleType 5 = Serpen(v25·v24 target). / :111 tps*5 유예 | 4 |
| 5 | 1 | 63 | 태그 | TeamPlan.objective 태그 1 = Some(Serpen) (MainObjective idx1). 아니면 Recall(:90). / :39 StealTarget 1 = Serpen | 4 |
| 6 | 3 | 64 | 태그 | ObjectPhase 3 = Hunt (take_hunt_commit 인라인). ※`shl …, 3` 은 Vec<usize> stride(8B) | 4 |
| 7 | 2 | 73 | 태그 | SubPlan 태그 2 = LineDefense · MinionActionType 2 = Push | 4 |
| 8 | 14 | 77 | 태그 | SubPlan 메모리태그 14 = SerpenHunt | 4 |
| 9 | 64000 | 103 | 산출값 | [game_core heal_area 인라인] 팀0: x<=64000 && 896000<=y<=960000 / 팀1: 892000<=x<=960000 && y<=64000 | 4 |
| 10 | 960000 | 103 | 산출값 | heal_area 인라인 | 4 |
| 11 | 891999 | 103 | 임계 | heal_area 인라인(x > 891999) | 4 |
| 12 | 896000 | 103 | 임계 | heal_area 인라인 | 4 |
| 13 | 40000000001 | 121 | 임계 | 200000^2+1 — v46 위협이 이 거리 안이면 도주 유지(Recall) | 4 |
| 14 | 22500000001 | 150 | 임계 | 150000^2+1 — ★check_recall 카운트 반경: 세르펜에서 150000 이내의 최근가시 적/아군을 센다(Epic 판은 regions==7 셀 기준). Epic 판의 :86 EpicPoke 거리 게이트는 Serpen 판에 없음 | 4 |
| 15 | 21 | 159 | 임계 | check_recall 최종: hp_ratio < 21 && 아군수 < 적수 → 귀환 | 4 |
| 16 | 15 | 86 | 태그 | SubPlan 메모리태그 15 = SerpenPoke — v24 가 false 면 거리 무관 무조건 Poke | 4 |
| 17 | 18 | 39 | 태그 | SubPlan 메모리태그 18 = Steal | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 귀환 HP 임계(세르펜 풀피일 때) | serpen/hunt_and_poke.rs:58 | 51 | Epic 판과 동일 | 4 | 기존 |
| 1 | check_recall 저체력 임계 | serpen/hunt_and_poke.rs:159 | 21 | Epic 판과 동일 | 4 | 기존 |
| 2 | 세르펜 근처 관측 유예 | serpen/hunt_and_poke.rs:111 | 5 | tps*5초. Epic 판과 동일 | 4 | 기존 |
| 3 | check_recall 카운트 반경 | serpen/hunt_and_poke.rs:150 | 22500000001 | 150000^2+1. 올리면 더 먼 적/아군까지 수적 비교에 들어가 귀환 판단이 바뀐다(Epic 은 리전 셀 기준이라 이 노브가 없음) | 4 | 기존 |
| 4 | v46 도주 위협 유지 반경 | serpen/hunt_and_poke.rs:121 | 40000000001 | Epic 판과 동일 | 4 | 기존 |
| 5 | 스플리터 잔류 라인 | serpen/hunt_and_poke.rs:70 | 0 | fallback_line(ctx, Top). 세르펜은 탑 쪽, 에픽은 바텀 쪽(2) — 맵 배치에 맞춘 값 | 4 | 기존 |

<details><summary>`callees` 피호출자 18건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_log | game_core::DebugFrameData::add_log | pub | fn(&mut game_core::DebugFrameData, &game_core::OperationData, &game_core::PlayerState, std::string::String) | game-core\src\simulation\game\frame.rs:54 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | check_recall | game_ai::plan_legacy::old::SinglePlanLine::check_recall | in:game_ai::plan_legacy::old::single_line | fn(&game_ai::plan_legacy::old::SinglePlanLine, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\single_line.rs:281 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | check_recall | game_ai::plan_legacy::old::PassiveLinePlan::check_recall | in:game_ai::plan_legacy::old::passive_line | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_line.rs:1068 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | check_recall | game_ai::plan_legacy::old::PassiveJunglePlan::check_recall | in:game_ai::plan_legacy::old::passive_jungle | fn(&game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:142 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | contains | game_core::RectU64::contains | pub | fn(&game_core::RectU64, u64, u64) -> bool | game-core\src\setting.rs:40 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | contains | game_core::RectI64::contains | pub | fn(&game_core::RectI64, i64, i64) -> bool | game-core\src\setting.rs:55 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | contains | game_core::setting::champion::nightmare::NightmareWellRect::contains | in:game_core::setting::champion::nightmare | fn(game_core::setting::champion::nightmare::NightmareWellRect, u64, u64) -> bool | game-core\src\setting\champion\nightmare.rs:28 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 8 | fallback_line | game_ai::plan_legacy::rule_scope::fallback_line | pub | fn(&game_core::GameContext, game_core::LineType) -> game_core::LineType | game-ai\src\plan_legacy\rule_scope.rs:28 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | heal_area | game_core::Game::heal_area | pub | fn(usize) -> (u64, u64, u64, u64) | game-core\src\simulation\game.rs:318 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | jungle_runner | game_core::MobaMode::jungle_runner | pub | fn(&game_core::MobaMode) -> &game_core::JungleRunner | game-core\src\simulation\game.rs:213 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 12 | jungle_runner | game_core::AbstractGame::jungle_runner | pub | fn(&Self/#0) -> &game_core::JungleRunner | game-core\src\simulation.rs:119 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 13 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | upgrade_item | game_ai::upgrade_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1603 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | v24_objective_setup_should_check_camp | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | v25_objective_splitter_can_stay | game_ai::plan_legacy::team_plan::v25_objective_splitter_can_stay | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:143 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | v46_flee_gate_check | game_ai::plan_legacy::old::passive_line::v46_flee_gate_check | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> (u8, bumpalo::collections::vec::Vec< usize>) | game-ai\src\plan_legacy\old\passive_line.rs:38 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 4개**: `dist_sq`, `first`, `format_inner`, `from_iter`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:8472) · **형제 9개** (SerpenHuntAndPokePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::SerpenHuntAndPokePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:9 | True | fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan) -> game_ai::plan_legacy::old::SerpenHuntAndPokePlan |
| 1 | <game_ai::plan_legacy::old::SerpenHuntAndPokePlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:9 | True | fn() -> game_ai::plan_legacy::old::SerpenHuntAndPokePlan |
| 2 | <game_ai::plan_legacy::old::SerpenHuntAndPokePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:9 | True | fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::goal | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:22 | True | fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan) -> game_core::BigGoal |
| 4 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::update | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:26 | True | fn(&mut game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 5 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::sub_plan | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:30 | False | fn(&mut game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |
| 6 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::next_plan | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:94 | True | fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 7 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::check_recall | in:game_ai::plan_legacy::old::serpen::hunt_and_poke | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:99 | False | fn(&mut game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) -> bool |
| 8 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::is_end | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:162 | False | fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | ★Epic 판과의 차이 정리(IR 확정): ① objective 태그 1(Serpen) vs 0(Morgard) ② JungleType 인자 5 vs 4 ③ fallback_line 라인 Top(0) vs Bottom(2) ④ check_recall 카운트: 세르펜 실체 재조회(vtable+0xe0 jungle_runner, 없으면 귀환 true) + dist<150000 vs Epic 의 regions==7 셀 ⑤ 꼬리: v24 false 면 무조건 SerpenPoke vs Epic 의 dist<150000 게이트. 그 외(스틸·HP·샘·v46·Split 잔류)는 동형 | 4 |  |
| 1 | 미탐색 | can_upgrade_item 계산 위치가 :57(fountain 뒤)로 Epic(:48)과 다르지만 IR 에서 두 경로에 중복 호출돼 있을 뿐 조건식은 동일 — 부작용 유무는 upgrade_item 본문 미탐색 | 4 |  |
| 2 | 표기 불가 | :58 줄 안 결합 순서 — column 없음(표기 불가) | 4 |  |
| 3 | 미탐색 | v46_flee_gate_check 코드 1~5 의미 — passive_line.rs:38 본문 미탐색 | 4 |  |
| 4 | 미탐색 | objective_target/take_hunt_commit 헬퍼의 부수효과 여부 — 인라인돼 태그 비교만 남음(fnparts 인라인 87개), 본문 내 team_plan store 없음 | 4 |  |
| 5 | 미탐색 | exe 0xdf0e90 대조: 0x6d70·64000·892000·896000·960000·40000000001(+0x519)·22500000001(+0x685) 확인, vtable [+0x40]/[+0xe0]/[+0x1f0] 호출 존재(Epic exe 엔 [+0xe0] 없음 — IR 차이 ④와 일치), regions 0x38b8 부재(차이 ④ 일치). 로직 어긋남 없음 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

