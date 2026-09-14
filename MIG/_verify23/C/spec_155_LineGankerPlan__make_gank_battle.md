---

### `155` LineGankerPlan::make_gank_battle — 갱 대상에 대한 TryKill BattlePlan 을 만들어 BigPlan::Battle 로 반환 — v2+ 는 추격 가망 없음(open_chase_race_hopeless) 또는 첫 update 후 sub_goal 이 KitingBack/RunAway/End 면 None

| 항목 | 값 |
|---|---|
| id | `LineGankerPlan__make_gank_battle` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank6gankerNtB2_14LineGankerPlan16make_gank_battle` |
| 소스 | `game-ai\src\plan_legacy\old\line_gank\ganker.rs:76` |
| IR | `m08.ll` 94354~94516행 |
| 경로·가시성 | `game_ai::plan_legacy::old::LineGankerPlan::make_gank_battle` · **in:game_ai::plan_legacy::old::line_gank::ganker** |
| 계층 | 레거시 플랜 |
| exe | `db8e60` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan>
```

<details><summary>인자 11개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<BigPlan>(384B) | writes 참조 | 4 |
| 1 | 1 | self | &LineGankerPlan | ★IR 에서 제거됨(미사용) | 4 |
| 2 | 2 | version | usize (IR %1) | 분기 1곳: `version > 1` (94368). BattlePlan::new/update·open_chase_race_hopeless 에 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) (IR %2) | ★&mut — 본문 직접 store 0. BattlePlan::update 에만 전달(v2+ 경로) | 4 |
| 4 | 4 | player | &PlayerState(2528B, readonly) (IR %3) | +0x930 team · +0x9c0 position (v2+) | 4 |
| 5 | 5 | data | &OperationData(24B, readonly) (IR %4) | +0x0 cache → player_champion · game 팻포인터(get_entity_by_id) | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B, readonly) (IR %5) | 본문 읽기 0. update 에 전달 | 4 |
| 7 | 7 | team_plan | &TeamPlan(1064B) (IR %6, readonly 속성 없음) | +0x41f objective(Option<MainObjective> 3B) 읽어 BattlePlan.main_objective 에 복사. update 에 전달. 이 함수는 쓰지 않음 | 4 |
| 8 | 8 | target | usize (IR %7) | 갱 대상 엔티티 id — BattlePlanGoal::TryKill.0 · get_entity_by_id 인자 | 4 |
| 9 | 9 | (usize) | usize (IR %8) | BattlePlanGoal::TryKill.1 에 그대로 저장(94377·94447). 이름은 재료 부재(dbg 없음) — TryKill 2번째 필드 | 4 |
| 10 | 10 | debug | &mut DebugFrameData(224B) (IR %9) | ★&mut — 본문 직접 store 0. BattlePlan::update 에만 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
LineGankerPlan::make_gank_battle(&self /*IR 에서 제거*/, version, rnd, player, data, ps, team_plan, target, arg8, debug) -> Option<BigPlan>   [ganker.rs:76]
 if version > 1:                                                        (L79, 94368)
     me  = data.cache.player_champion[player.team][player.position]     (L80, 94404~94411)
     tgt = game.get_entity_by_id(target)                                (L81, 94412~94417, vtable+0x1f0)
     if me.is_some() && tgt.is_some() && open_chase_race_hopeless(version, data, player, me, tgt):   (L82, 94418~94427)
         return None                                                    (L83, 94430)   # 추격 경주 가망 없음
 plan = BattlePlan::new(version, BattlePlanGoal::TryKill(target, arg8), data, player)   (L87, 94379/94449)
 plan.entry_src = 7                                                     (L88, 94382/94452)
 if version > 1:                                                        (L89~91 — IR 은 L79 분기 결과로 블록 분리)
     plan.set_main_objective(team_plan.objective)   # battle.rs:350 인라인, Option<MainObjective> 3B 복사   (L90, 94453~94458)
     plan.update(version, rnd, player, data, ps, team_plan, debug)      (L92, 94459)
     if plan.sub_goal is KitingBack(3) | RunAway(4) | End(7):           (L92~93, switch 94465~94469)
         drop(plan) ; return None                                       (L93, 94472~94513: chats Vec<Chat> +0x68 · v54_reentry_ticks Vec<usize> +0x80 drop)
 return Some(BigPlan::Battle(plan))                                     (L96, 94392~94394)

요지: 갱 대상에 TryKill 전투플랜을 세우되, v2+ 에선 (a) 추격 경주가 가망 없으면 세우지 않고 (b) 한 틱 update 를 돌려 플랜이 즉시 후퇴/종료로 판정되면 폐기한다. `me`/`tgt` 가 None 이면 (a) 검사를 건너뛰고 플랜을 세운다(94421 → %49).
```

**`mem` 메모리 접근 15건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 94387 (v2+), <2 아니면 panic_bounds_check 94399 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 94405 | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | 94407 | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | Option<&Entity>, null=None (94408~94411) | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x0 | game.data_ptr / +0x8 vtable_ptr | r | 94412~94414 | 4 | OK |  |
| 5 | AbstractGame vtable | 0x1f0 | get_entity_by_id | r | (game, target) → Option<&Entity> (94415~94417, divtable 정적 vtable 기준) | 3 | 확인불가(vtable 슬롯) |  |
| 6 | TeamPlan | 0x41f | objective (Option<MainObjective> 3B: 태그·페이로드) | r | i24 로드 94454 → BattlePlan+0xff 에 저장 | 4 | OK |  |
| 7 | BattlePlan(지역 %12) | 0x58 | sub_goal@tag | r | update 뒤 로드 94464 → switch 3/4/7 | 4 | OK |  |
| 8 | Option<BigPlan>(sret) | 0x0 | tag | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | 9 = BigPlan::Battle (94394) / -1 = None (94430·94472) |
| 9 | Option<BigPlan>(sret) | 0x8 | Battle.0 (BattlePlan 280B) | w | Some 경로만 | 4 | 확인불가(tcx 사전에 타입 없음) | 지역 plan memcpy (94393) |
| 10 | BattlePlanGoal(지역 %11, 24B) | 0x0 | tag=0 TryKill / +0x8 .0=target / +0x10 .1=인자 %8 | w | BattlePlan::new 2번째 인자(값 전달, 포인터로) | 4 | OK | 94374~94378 (v≤1) · 94444~94448 (v2+) |
| 11 | BattlePlan(지역 %12) | 0x107 | entry_src | w | 94382·94452 — BattlePlan::new 직후 덮어씀(진입귀속 코드 7 = 이 함수(LineGanker 갱 전투) 진입) | 4 | OK | 7 |
| 12 | BattlePlan(지역 %12) | 0xff | main_objective (3B) | w | v2+ 경로만, update 호출 전 | 4 | OK | team_plan.objective 그대로 복사(i24 store 94458) |
| 13 | BattlePlan(지역 %12) | 0x0 | (전체) BattlePlan::update(&mut) 내부 갱신 | w | v2+ 경로. 이후 sub_goal 태그로 판정 | 4 | OK | 콜리 소관 |
| 14 | StdRng(rnd)/DebugFrameData(debug) | 0x0 | 콜리 update 경유 쓰기 | w | 본문 직접 store 없음 | 4 | 오귀속(사전은 다른 필드를 준다) | 미탐색(plan_legacy battle 계층) |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 79 | 임계 | `version > 1` 분기(94368): v≤1 = 옛 경로(update 없이 즉시 Battle), v2+ = 가망 검사 + update + sub_goal 검사 | 4 |
| 1 | 0 | 87 | 태그 | BattlePlanGoal::TryKill 태그 0 (94378·94448) | 4 |
| 2 | 7 | 88 | 태그 | entry_src = 7 (94382·94452, L88). 또한 BattleSubPlanGoal::End 태그 7 (switch 94468, L92~93) | 4 |
| 3 | 9 | 96 | 센티널 | BigPlan::Battle 메모리 태그 9(니치, 논리 idx 7) (94394) | 4 |
| 4 | -1 | 83 | 태그 | Option<BigPlan>::None 태그 (94430 L83 · 94472 L93) | 4 |
| 5 | 3 | 93 | 태그 | BattleSubPlanGoal::KitingBack 태그 3 → None (switch 94466) | 4 |
| 6 | 4 | 93 | 태그 | BattleSubPlanGoal::RunAway 태그 4 → None (94467) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 갱 전투 진입 차단 sub_goal 집합 | ganker.rs:92~93 (switch 94465~94469) | {KitingBack, RunAway, End} | Kiting(2) 을 추가하면 카이팅 판정된 갱도 폐기 — 갱 개시 빈도↓. 집합을 비우면 update 결과와 무관하게 항상 개전 | 4 | 기존 |
| 1 | v2+ 사전 가망 검사 | ganker.rs:82 (94426) | open_chase_race_hopeless(version, data, player, me, tgt) (fight_model.rs, 이 배치 밖 콜리) | 끄면 도망치는 대상에도 갱 전투플랜을 세워 헛추격↑ | 4 | 기존 |
| 2 | entry_src 코드 | ganker.rs:88 (94382·94452) | 7 | 계측 라벨(진입귀속 코드표) — 판정 영향 없음, 바꾸면 통계 분류만 바뀜 | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 1 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | make_gank_battle | game_ai::plan_legacy::old::LineGankerPlan::make_gank_battle | in:game_ai::plan_legacy::old::line_gank::ganker | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> | game-ai\src\plan_legacy\old\line_gank\ganker.rs:76 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | new | game_ai::plan_legacy::old::BattlePlan::new | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan | game-ai\src\plan_legacy\old\battle.rs:196 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | open_chase_race_hopeless | game_ai::plan_legacy::old::fight_model::open_chase_race_hopeless | in:game_ai | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | set_main_objective | game_ai::plan_legacy::old::BattlePlan::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\battle.rs:349 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | set_main_objective | game_ai::plan_legacy::old::SinglePlanBattle::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::SinglePlanBattle, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\single_battle.rs:85 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 8 | set_main_objective | game_ai::plan_legacy::old::DeathMatchBattle::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::DeathMatchBattle, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\death_battle.rs:867 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 9 | update | game_ai::plan_legacy::old::BattlePlan::update | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\battle.rs:427 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 1개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 7곳** (m08.ll:97043, m08.ll:97799, m08.ll:97823, m08.ll:97891, m08.ll:97897, m08.ll:98181, m08.ll:98316) · **형제 14개** (LineGankerPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::LineGankerPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:9 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan) -> game_ai::plan_legacy::old::LineGankerPlan |
| 1 | <game_ai::plan_legacy::old::LineGankerPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:9 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::LineGankerPlan::new | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:27 | True | fn(game_core::LineType, usize, usize) -> game_ai::plan_legacy::old::LineGankerPlan |
| 3 | game_ai::plan_legacy::old::LineGankerPlan::new_with_phase | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:31 | True | fn(game_core::LineType, usize, usize, game_ai::plan_legacy::old::LineGankerPhase) -> game_ai::plan_legacy::old::LineGankerPlan |
| 4 | game_ai::plan_legacy::old::LineGankerPlan::goal | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:35 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan) -> game_core::BigGoal |
| 5 | game_ai::plan_legacy::old::LineGankerPlan::update | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:39 | False | fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 6 | game_ai::plan_legacy::old::LineGankerPlan::is_end | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:64 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 7 | game_ai::plan_legacy::old::LineGankerPlan::is_cancel | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:69 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan) -> bool |
| 8 | game_ai::plan_legacy::old::LineGankerPlan::make_gank_battle | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:76 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 9 | game_ai::plan_legacy::old::LineGankerPlan::next_plan | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:99 | False | fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 10 | game_ai::plan_legacy::old::LineGankerPlan::target_bush_v30 | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:248 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize |
| 11 | game_ai::plan_legacy::old::LineGankerPlan::target_bush_v41 | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:310 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize |
| 12 | game_ai::plan_legacy::old::LineGankerPlan::target_bush | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:351 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> (u64, u64) |
| 13 | game_ai::plan_legacy::old::LineGankerPlan::sub_plan | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:379 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 인자 %8(usize, TryKill.1)의 이름/의미 — dbg_value 없음(internal fastcc 로 이름 소실). BattlePlanGoal::TryKill 2번째 필드 의미는 battle.rs 소관 | 4 |  |
| 1 | 미탐색 | entry_src=7 이 '진입귀속 코드표'에서 어떤 라벨인지 — _docs 에 표 본문 없음(코드표 위치 미탐색: battle.rs 주석 또는 계측 문서) | 4 |  |
| 2 | 미탐색 | BattlePlan::new(m10.ll:23214) 이 280B 중 어느 바이트를 초기화하는지(sret 에 initializes 없음) — 사용자가 sweep 비교 시 Battle 페이로드 live 범위는 battle 계층 명세 참조 | 4 |  |
| 3 | 미탐색 | exe 측 인자 배치 — internal fastcc + DeadArgElim(self 제거) 이라 exe 의 실제 레지스터/스택 배치는 argscan 으로 확인 필요(이 배치 범위 밖) | 4 |  |
| 4 | 미탐색 | open_chase_race_hopeless / BattlePlan::update 내부 — 같은 r14 중간·battle 계층, 시그니처만 기록 | 4 |  |
| 5 | 미탐색 | BattlePlan::set_main_objective(battle.rs:350) 은 인라인돼 call 이 없다(94454~94458 i24 복사가 그 본문). calls 엔 미등록 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L89·L91 의 정확한 텍스트(두 번째 `if version > 1` 인지 블록 구조인지) — IR 이 L79 분기로 두 경로를 통째 분리해 재구성 불가(column 부재). 동작은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

