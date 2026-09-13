---

### `69` try_engage_dive — 다이브 재합류 쿨다운·(v2+) 추격 레이스 가망 검사 후 TryKill 다이브 BattlePlan 을 생성·1틱 update 해 이탈 태세면 폐기, 아니면 반환

| 항목 | 값 |
|---|---|
| id | `engage__try_engage_dive` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler6engageNtB4_17LegacyPlanHandler15try_engage_dive` |
| 소스 | `game-ai\src\plan_legacy\handler\engage.rs:109` |
| IR | `m13.ll` 34298~34493행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage_dive` · **in:game_ai** |
| 계층 | 플랜 핸들러 |
| exe | `e5d300` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan>
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | sret | *mut Option<BattlePlan>(280B) | None = 첫 8B 에 -1 (니치, m13.ll:34338/34419/34446). Some = BattlePlan 280B memcpy | 4 |
| 1 | 1 | self | &LegacyPlanHandler | last_dive_abandon_tick(0x1480)·team_plan(0xf8)·team_plan.objective(0x517)·positioning_score(0x990) | 4 |
| 2 | 2 | version | usize | L116 `version > 1` 분기(v2+ 에서만 추격 가망 검사·main_objective 복사) | 4 |
| 3 | 3 | rnd | &mut StdRng | BattlePlan::update 에 전달만 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | info.team(0x930)·info.position@tag(0x9c0) — v2+ 경로에서만 | 4 |
| 5 | 5 | data | &OperationData(24B) | cache.game(tick·get_entity_by_id)·context.setting.tick_per_second | 4 |
| 6 | 6 | target_id | usize | 다이브 대상 엔티티 id → TryKill.0 · get_entity_by_id | 4 |
| 7 | 7 | dive_tower | Option<TowerType>(i8) | BattlePlan.dive_tower(0xfe) 에 그대로 저장 | 4 |
| 8 | 8 | debug | &mut DebugFrameData | BattlePlan::update 에 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn try_engage_dive(&self, version, rnd, player, data, target_id, dive_tower, debug) -> Option<BattlePlan>
  tick = game.tick()                                                                  // L112 (vtable+0x28)
  tps  = context.setting.tick_per_second
  // dive_rejoin_cd(self) 인라인 (engage.rs:973) = self.last_dive_abandon_tick + 1 + tps*4
  if !(tick > self.last_dive_abandon_tick + 1 + tps*4) → return None                 // L112~113 (어보트 후 4초 쿨다운)
  if version > 1 {                                                                    // L116
    champ  = cache.player_champion[player.team][player.position]                     // L117
    target = game.get_entity_by_id(target_id)                                          // L118 (vtable+0x1f0)
    if let (Some(champ), Some(target)) = (champ, target) {
      if open_chase_race_hopeless(version, data, player, champ, target) → return None   // L119~120
    }
  }
  goal = BattlePlanGoal::TryKill(target_id, 60)                                       // L124
  plan = BattlePlan::new_dive(version, &goal, data, player)
  plan.entry_src = 2                                                                   // L125
  plan.dive_tower = dive_tower                                                         // L126
  if version > 1 { plan.set_main_objective(self.team_plan.objective) }                // L128 (battle.rs:350 인라인, 0x517 → 0xff 3B)
  plan.update(version, rnd, player, data, &self.positioning_score, &self.team_plan, debug)   // L130
  dive_committed = !matches!(plan.sub_goal, KitingBack|RunAway|End)                   // L132 (tag 3/4/7)
  if dive_committed { Some(plan) } else { None }                                       // L133
```

**`mem` 메모리 접근 19건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LegacyPlanHandler | 0x1480 | last_dive_abandon_tick | r | gep +5248 (m13.ll:34317). dive_rejoin_cd(engage.rs:973) = 이 값 + 1 + tps*4 | 4 | OK |  |
| 1 | LegacyPlanHandler | 0x517 | team_plan.objective (Option<MainObjective>, 3B i24) | r | gep +1303 (m13.ll:34404). v2+ 에서 BattlePlan.main_objective(0xff) 로 복사 (set_main_objective, battle.rs:350 인라인) | 4 | OK |  |
| 2 | LegacyPlanHandler | 0x990 | positioning_score (&PositioningScoreData) | r | gep +2448 (m13.ll:34364). BattlePlan::update 6번째 인자 | 4 | OK |  |
| 3 | LegacyPlanHandler | 0xf8 | team_plan (&TeamPlan) | r | gep +248 (m13.ll:34365). BattlePlan::update 7번째 인자 | 4 | OK |  |
| 4 | OperationData | 0x0 | cache | r | game(+0/+8 vtable)·player_champion(0x1e0) | 4 | OK |  |
| 5 | OperationData | 0x8 | context | r | setting(+8) → tick_per_second | 4 | OK |  |
| 6 | GameSetting | 0x12f8 | tick_per_second | r | gep +4856 (m13.ll:34323). 쿨다운 4초 환산 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr data/vtable | r | vtable+0x28 = tick() (m13.ll:34314) / vtable+0x1f0 = get_entity_by_id(target_id) (m13.ll:34386, v2+) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | gep +480 (m13.ll:34382). v2+ 에서 내 챔피언 | 4 | OK |  |
| 9 | PlayerState | 0x930 | info.team | r | v2+ (L117) | 4 | OK |  |
| 10 | PlayerState | 0x9c0 | info.position@tag | r | v2+ (L117) | 4 | OK |  |
| 11 | BattlePlan | 0x58 | sub_goal@tag (BattleSubPlanGoal) | r | L132 update 후 판독 (gep +88, m13.ll:34431): 3 KitingBack / 4 RunAway / 7 End 면 폐기 | 4 | OK |  |
| 12 | BattlePlanGoal(스택 24B) | 0x0 | tag = 0 (TryKill) | w | m13.ll:34348 — new_dive 의 goal 인자 | 4 | OK | 0 |
| 13 | BattlePlanGoal(스택 24B) | 0x8 | TryKill.0 | w | m13.ll:34345 | 4 | OK | target_id |
| 14 | BattlePlanGoal(스택 24B) | 0x10 | TryKill.1 | w | m13.ll:34347. 리터럴 60 (BattlePlanGoal.__1 — 소비처는 이 본문에 없음) | 4 | OK | 60 |
| 15 | BattlePlan(스택 280B) | 0x107 | entry_src | w | L125 (m13.ll:34352/34401) — 진입귀속 코드 2 = 다이브 경유 | 4 | OK | 2 |
| 16 | BattlePlan(스택 280B) | 0xfe | dive_tower | w | L126 (m13.ll:34353/34402) | 4 | OK | 인자 dive_tower 그대로 |
| 17 | BattlePlan(스택 280B) | 0xff | main_objective@tag (3B) | w | L128, v2+ 경로만 (m13.ll:34408). v1 경로는 new_dive 초기값 유지 | 4 | OK | self.team_plan.objective (0x517) 복사 |
| 18 | sret Option<BattlePlan> | 0x0 | None 니치 / Some(BattlePlan 280B) | w | m13.ll:34338·34419·34446 / 34441 | 4 | 확인불가(tcx 사전에 타입 없음) | -1 또는 memcpy 280 |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 112 | 임계 | `shl i64 %tps, 2` = tps*4 (dive_rejoin_cd, engage.rs:973). 소스 수준 값은 folded_from 참조 | 4 | 4 |
| 1 | 1 | 112 | 임계 | dive_rejoin_cd = last_dive_abandon_tick + 1 + tps*4 — 재합류 허용 조건 tick > cd (즉 어보트 후 4초+1틱 경과). L116 의 `version > 1` 리터럴도 1 | 4 |  |
| 2 | 60 | 124 | 산출값 | BattlePlanGoal::TryKill.1 에 넣는 리터럴 60 (의미는 이 본문에서 소비 안 됨 — new_dive/BattlePlan 내부 미탐색) | 4 |  |
| 3 | 0 | 124 | 태그 | BattlePlanGoal::TryKill 메모리태그 0 | 4 |  |
| 4 | 3 | 132 | 태그 | BattleSubPlanGoal::KitingBack 태그 — update 후 이 태세면 dive 미커밋 → None | 4 |  |
| 5 | 4 | 132 | 태그 | BattleSubPlanGoal::RunAway 태그 — 동상 | 4 |  |
| 6 | 7 | 132 | 태그 | BattleSubPlanGoal::End 태그 — 동상 | 4 |  |
| 7 | -1 | 113 | 센티널 | Option<BattlePlan>::None 니치 값(첫 8B) | 4 |  |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 다이브 재합류 쿨다운 | engage.rs:973 (dive_rejoin_cd, 인라인) | tps*4 + 1 틱 (= 4초) | shl 2 를 바꾸면 어보트 후 재다이브까지 대기 시간 변경. 줄이면 propose/abort 반복 증가 가능 | 4 | 기존 |
| 1 | v2+ 추격 가망 게이트 | engage.rs:116~120 | version > 1 && open_chase_race_hopeless | 게이트 제거 시 가망 없는 추격 다이브도 플랜 생성 | 4 | 기존 |
| 2 | dive 미커밋 판정 태세 집합 | engage.rs:132 | {KitingBack(3), RunAway(4), End(7)} | 집합을 줄이면(예: KitingBack 허용) 이탈 태세로 시작하는 다이브도 반환됨 | 4 | 기존 |
| 3 | TryKill.1 리터럴 | engage.rs:124 | 60 | 이 본문엔 소비처 없음 — 효과 미확정 | 4 | 기존 |

<details><summary>`callees` 피호출자 14건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | dive_rejoin_cd | game_ai::plan_legacy::handler::engage::dive_rejoin_cd | in:game_ai | fn(usize, usize) -> usize | game-ai\src\plan_legacy\handler\engage.rs:971 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | new_dive | game_ai::plan_legacy::old::BattlePlan::new_dive | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan | game-ai\src\plan_legacy\old\battle.rs:246 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | open_chase_race_hopeless | game_ai::plan_legacy::old::fight_model::open_chase_race_hopeless | in:game_ai | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | set_main_objective | game_ai::plan_legacy::old::BattlePlan::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\battle.rs:349 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | set_main_objective | game_ai::plan_legacy::old::SinglePlanBattle::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::SinglePlanBattle, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\single_battle.rs:85 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 8 | set_main_objective | game_ai::plan_legacy::old::DeathMatchBattle::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::DeathMatchBattle, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\death_battle.rs:867 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 9 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 10 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 11 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | try_engage_dive | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage_dive | in:game_ai | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> | game-ai\src\plan_legacy\handler\engage.rs:109 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | update | game_ai::plan_legacy::old::BattlePlan::update | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\battle.rs:427 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 1개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m13.ll:39979, m13.ll:40962, m13.ll:42881) · **형제 41개** (LegacyPlanHandler)

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

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | BattlePlanGoal::TryKill.1 = 60 의 의미 — new_dive/BattlePlan 내부(m10.ll:27462) 미탐색. 이름으로 성격 단정 금지 원칙에 따라 '리터럴 60' 으로만 기록 | 4 |  |
| 1 | 미탐색 | open_chase_race_hopeless(fight_model.rs, m10.ll:47705) 의 판정 내용 미탐색 | 4 |  |
| 2 | 미탐색 | BattlePlan::new_dive / update 내부 미탐색 — update 가 sub_goal 을 어떻게 정하는지(base_sub_goal 경유 여부 포함)는 별도 명세 | 4 |  |
| 3 | 미탐색 | entry_src=2 의 코드표(진입귀속) 전체는 _docs 708행 '진입귀속 코드표' 로만 확인, 2 의 의미('다이브 경유')는 이 함수가 유일한 store 인지 미확인 — 추정 | 5 |  |
| 4 | 표기 불가 | L112 비교가 `tick > cd` 인지 `tick >= cd+1` 인지 표기 불가(외연 동일: `+1` 이 cd 계산에 접혀 있음) | 4 |  |
| 5 | 미탐색 | v1 경로(version<=1)에서 main_objective 를 복사하지 않는 것이 의도인지(소스가 if version>1 블록 안에 set_main_objective 를 둔 것)는 IR 구조상 확정 — 단 v1 이 실전에서 쓰이는지는 미탐색 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

