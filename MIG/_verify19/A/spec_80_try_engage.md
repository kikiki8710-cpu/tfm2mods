---

### `80` try_engage — target_id 상대로 BattlePlan(일반/다이브)을 만들어 첫 update 까지 돌리고, 게이트(패배직후 쿨·다이브 재진입 쿨·추격무망·다이브 불가·즉시 후퇴 서브골)에 걸리면 None 을 돌려준다

| 항목 | 값 |
|---|---|
| id | `engage__try_engage` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler6engageNtB4_17LegacyPlanHandler10try_engage` |
| 소스 | `game-ai\src\plan_legacy\handler\engage.rs:40` |
| IR | `m13.ll` 33742~34295행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage` · **in:game_ai** |
| 계층 | 플랜 핸들러 |
| exe | `e5ca10` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<BattlePlan> (280B) | +0 = support_target@tag 니치: -1 = None. Some 이면 BattlePlan 280B 통째 memcpy | 4 |
| 1 | 1 | self | &mut LegacyPlanHandler | 본문에서 self 필드 store 없음(읽기: 0x1480·0x1490·0x1498·0x517~0x519·&0x990·&0xf8). ⚠BattlePlan::update 에 &mut team_plan(0xf8, non-readonly) 을 넘기므로 콜리 내부 write 가능성은 범위 밖 | 4 |
| 2 | 2 | version | usize | L43·L63·L81·L96: `version > 1`(v2+) 게이트 4곳. new/new_dive/update/open_chase_race_hopeless/tower_dive_is_viable 에 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | 본문 직접 사용 없음. tower_dive_is_viable·update 에 전달 | 4 |
| 4 | 4 | player | &PlayerState (2528B) | +0x930 team, +0x9c0 position (L64·L82 me 조회) | 4 |
| 5 | 5 | data | &OperationData (24B) | +0x0 cache, +0x8 context(+0x8 setting → tick_per_second) | 4 |
| 6 | 6 | target_id | usize | 교전 대상 Entity id. get_entity_by_id 조회·BattlePlanGoal::TryKill(target_id, 60) 생성·last_lost_fight.0 과 비교 | 4 |
| 7 | 7 | debug | &mut DebugFrameData (224B) | tower_dive_is_viable·update 에 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
try_engage(&mut self, version, rnd, player, data, target_id, debug) -> Option<BattlePlan>
  if version > 1 && target_id == self.last_lost_fight.0 {                     // L43
    if game.tick() <= self.last_lost_fight.1 + tps*3 { return None } }        // L44~45  패배 직후 같은 상대 3초 재교전 금지
  target = game.get_entity_by_id(target_id)                                  // L52
  in_tower = target.map_or(false, |t| engage_requires_dive(player, data, t)) // L52 closure$0
  battle = if in_tower {                                                     // L53
    t = get_entity_by_id(target_id)?  (None → return None)                   // L54
    if game.tick() <= self.last_dive_abandon_tick + 1 + tps*4 { return None } // L57 (dive_rejoin_cd)
    if version > 1 {                                                         // L63
      if let Some(me) = cache.player_champion[team][pos] {                   // L64
        if open_chase_race_hopeless(version, data, player, me, t) { return None } } }   // L65
    if !tower_dive_is_viable(version, rnd, player, data, &mut self.team_plan, t, true, debug) { return None }   // L70
    bp = BattlePlan::new_dive(version, &TryKill(target_id, 60), data, player) // L73
    bp.entry_src = 2                                                         // L74
    tower = iter_towers_without_nexus(cache, 1-team).min_by_key(|tw| dist_sq(tw, t))   // L75 closure$1 (첫 최소)
    bp.dive_tower = tower.and_then(|tw| if tw.ty==Tower { Some(tw.info.ty) } else { None })   // L76~77 closure$2
    bp
  } else {
    if version > 1 {                                                         // L81
      if let (Some(me), Some(t)) = (player_champion[team][pos], get_entity_by_id(target_id)) {   // L82~83
        if open_chase_race_hopeless(version, data, player, me, t) { return None } } }   // L84
    bp = BattlePlan::new(version, &TryKill(target_id, 60), data, player)      // L89
    bp.entry_src = 1                                                         // L90
    bp }
  if version > 1 { battle.set_main_objective(self.team_plan.objective) }     // L96~97 (+0xff 3B)
  battle.update(version, rnd, player, data, &self.positioning_score, &mut self.team_plan, debug)   // L99
  if battle.sub_goal ∈ {KitingBack(3), RunAway(4), End(7)} { drop(battle); return None }   // L100~101
  return Some(battle)                                                        // L102
```

**`mem` 메모리 접근 24건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LegacyPlanHandler | 0x1490 | last_lost_fight.0 | r | L43: v2+ 이고 target_id == 이 값이면 패배직후 쿨 검사 | 4 | OK |  |
| 1 | LegacyPlanHandler | 0x1498 | last_lost_fight.1 | r | L44: tick <= 이 값 + tps*3 이면 None | 4 | OK |  |
| 2 | LegacyPlanHandler | 0x1480 | last_dive_abandon_tick | r | L57: 다이브 경로에서 tick <= 이 값 + 1 + tps*4(dive_rejoin_cd, engage.rs:973) 이면 None | 4 | OK |  |
| 3 | LegacyPlanHandler | 0x517 | team_plan.objective (3B: @tag 0x517 · phase/line 0x518 · with_battle/ready 0x519) | r | L97 v2+: BattlePlan::set_main_objective(battle.rs:350) 인라인 → battle+0xff 에 3B 복사 | 4 | OK |  |
| 4 | LegacyPlanHandler | 0xf8 | team_plan | r | &mut 로 tower_dive_is_viable(L70)·update(L99) 에 전달 | 4 | OK |  |
| 5 | LegacyPlanHandler | 0x990 | positioning_score | r | & 로 update(L99) 에 전달(readonly) | 4 | OK |  |
| 6 | OperationData | 0x0 | cache | r |  | 4 | OK |  |
| 7 | OperationData | 0x8 | context | r | +0x8 setting | 4 | OK |  |
| 8 | GameContext | 0x8 | setting | r |  | 4 | OK |  |
| 9 | GameSetting | 0x12f8 | tick_per_second | r | L44 tps*3 · L57 tps*4(shl 2) | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x0 | game.data_ptr / game.vtable_ptr | r | vtable+0x1f0 get_entity_by_id(target_id) (L52·L54·L83), vtable+0x28 tick() (L44·L57) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion | r | [team][pos] = me (L64·L82, bounds<2 panic @64/@82) | 4 | OK |  |
| 12 | PlayerState | 0x930 | info.team | r | L64·L75·L82. 1-team = 적 팀(타워 탐색) | 4 | OK |  |
| 13 | PlayerState | 0x9c0 | info.position@tag | r | as_index → me 열 | 4 | OK |  |
| 14 | Entity | 0x68 | ty@tag | r | L76 closure$2: 최근접 타워 엔티티가 Tower(2) 일 때만 dive_tower 채움 | 4 | OK |  |
| 15 | Entity | 0x128 | ty@Tower.info.ty | r | TowerType(Top0/Mid1/Bottom2/TwinA3/TwinB4/Top2 5..) → battle.dive_tower | 4 | OK |  |
| 16 | Entity | 0x660 | x | r | L75 closure$1 distance_sq(타워, target) | 4 | OK |  |
| 17 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 18 | BattlePlan(local) | 0x58 | sub_goal@tag | r | L100: update 후 3 KitingBack / 4 RunAway / 7 End 면 폐기(None) | 4 | OK |  |
| 19 | Option<BattlePlan>(sret) | 0x0 | support_target@tag(니치) | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | -1 = None (L45·L54·L57·L65·L70·L84·L101) / Some = battle 280B memcpy (L102) |
| 20 | BattlePlan(local→sret) | 0x40 | main_goal@tag | w | L73·L89. 60 은 TryKill.1(리터럴) | 4 | OK | BattlePlanGoal::TryKill(target_id, 60) — {tag 0, +8 target_id, +16 60} 을 new/new_dive 에 전달 |
| 21 | BattlePlan(local→sret) | 0x107 | entry_src | w |  | 4 | OK | 2 (다이브, L74) / 1 (일반, L90) |
| 22 | BattlePlan(local→sret) | 0xfe | dive_tower | w |  | 4 | OK | 다이브 경로만: 최근접 적 타워의 TowerType, 없거나 Tower 엔티티가 아니면 -1(None) (L75~77) |
| 23 | BattlePlan(local→sret) | 0xff | main_objective@tag (3B) | w | v1 은 new 가 준 값 유지 | 4 | OK | v2+: self.team_plan.objective 3B 복사 (L96~97, set_main_objective battle.rs:350~351) |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1 | 43 | 임계 | version > 1 ⇔ v2+ 게이트(L43·63·81·96). L57 의 `+1` 도 같은 리터럴(abandon_tick + 1 + tps*4) | 4 |  |
| 1 | 3 | 44 | 태그 | 패배직후 재교전 쿨 = tps*3 (3초). 별도로 L100 의 sub_goal 태그 3 = KitingBack | 4 |  |
| 2 | 2 | 57 | 태그 | folded: dive_rejoin_cd(engage.rs:973) = tps<<2 = tps*4 (4초) — `shl i64 %tps, 2`. 별도로 L74 entry_src=2(다이브), L76 EntityType Tower 태그 2, 팀 bounds 2 | 4 | 4 |
| 3 | 60 | 73 | 산출값 | BattlePlanGoal::TryKill.1 = 60 (new_dive·new 양쪽 동일 리터럴) | 4 |  |
| 4 | 0 | 73 | 태그 | BattlePlanGoal 태그 0 = TryKill | 4 |  |
| 5 | 4 | 100 | 태그 | BattleSubPlanGoal 태그 4 = RunAway → 폐기 | 4 |  |
| 6 | 7 | 100 | 태그 | BattleSubPlanGoal 태그 7 = End → 폐기 | 4 |  |
| 7 | -1 | 76 | 센티널 | Option 니치 None: dive_tower(u8 -1) · 반환 Option<BattlePlan>(i64 -1) · 타워 iter 상태 | 4 |  |
| 8 | 6 | 75 | 임계 | iter_towers_without_nexus 의 배열 IntoIter<Option<&Entity>, 6> 상한(assume) — 타워 슬롯 6개(1차/2차 × 3라인) 순회 후 slice(쌍둥이 타워) chain — 판정 상수 아님 | 4 |  |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 패배 직후 같은 대상 재교전 쿨 | engage.rs:44 | tps*3 | 올리면 진 상대에게 더 오래 안 덤빈다 | 4 | 기존 |
| 1 | 다이브 포기 후 재진입 쿨 | engage.rs:57 (dive_rejoin_cd engage.rs:973) | tps*4 (+1) | 올리면 타워 다이브 재시도 간격이 길어진다 | 4 | 기존 |
| 2 | TryKill 목표 시간 상수 | engage.rs:73·89 | 60 | BattlePlanGoal::TryKill.1 — 소비처는 이 함수 밖(§4 함정 7: 산술 소비처 확인 필요) | 4 | 기존 |
| 3 | 즉시 폐기 서브골 집합 | engage.rs:100 | {3,4,7} | 집합을 줄이면 update 가 후퇴로 판정한 플랜도 반환된다(교전 개시 완화) | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | engage_requires_dive | game_ai::engage_requires_dive | pub | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:697 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | new | game_ai::plan_legacy::old::BattlePlan::new | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan | game-ai\src\plan_legacy\old\battle.rs:196 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | new_dive | game_ai::plan_legacy::old::BattlePlan::new_dive | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan | game-ai\src\plan_legacy\old\battle.rs:246 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | open_chase_race_hopeless | game_ai::plan_legacy::old::fight_model::open_chase_race_hopeless | in:game_ai | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | set_main_objective | game_ai::plan_legacy::old::BattlePlan::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\battle.rs:349 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 9 | set_main_objective | game_ai::plan_legacy::old::SinglePlanBattle::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::SinglePlanBattle, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\single_battle.rs:85 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 10 | set_main_objective | game_ai::plan_legacy::old::DeathMatchBattle::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::DeathMatchBattle, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\death_battle.rs:867 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | tower_dive_is_viable | game_ai::plan_legacy::old::tower_dive_is_viable | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::Entity, bool, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:935 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | try_engage | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage | in:game_ai | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> | game-ai\src\plan_legacy\handler\engage.rs:40 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | update | game_ai::plan_legacy::old::BattlePlan::update | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\battle.rs:427 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 5개**: `dist_sq`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `drop_glue<BattlePlan>`, `map_or`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 6곳** (m13.ll:35748, m13.ll:35786, m13.ll:37172, m13.ll:37208, m13.ll:41831, m13.ll:44633) · **형제 41개** (LegacyPlanHandler)

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
| 0 | 미탐색 | engage_requires_dive / open_chase_race_hopeless / tower_dive_is_viable / BattlePlan::new·new_dive·update 내부 — 별도 함수(호출만). `in_tower` 의 실제 기준은 engage_requires_dive 에 있다 | 4 |  |
| 1 | 미탐색 | tower_dive_is_viable 의 7번째 인자 `true` 의 의미(파라미터 이름) — 콜리 시그니처 미독 | 4 |  |
| 2 | 미탐색 | self(%1) 가 &mut 인지 & 인지 — IR 속성에 readonly 가 없고 DI 타입 미확인. 본문 store 는 0건이므로 이 함수 자체의 self write 는 없음(update 콜리의 team_plan 변경 가능성만) | 4 |  |
| 3 | 미탐색 | iter_towers_without_nexus 반환 순서(1차/2차·라인 순) — game_core 본문 미독. min_by_key 는 동률 시 먼저 나온 타워 | 4 |  |
| 4 | 미탐색 | Option<BattlePlan> 니치가 support_target@tag(+0) 인지 — `store i64 -1, %0` 관측과 tcxdict support_target@tag 8B 로 추정(tcxdict --enum Option<BattlePlan> 미조회) | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

