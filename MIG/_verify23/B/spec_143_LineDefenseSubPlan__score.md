---

### `143` LineDefenseSubPlan::score — 라인 수비 서브플랜 점수 — evaluate_action(Lane 프로필) 이 Some 이면 그 값, 아니면 interaction_score + 경제 보정 + 행동별 보너스(Around 아군 원거리 50/100/5 · Attack/Skill/Skill2 는 calculate_action_score, 대상 없으면 -99999)

| 항목 | 값 |
|---|---|
| id | `LineDefenseSubPlan__score` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12line_defenseNtB2_18LineDefenseSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\line_defense.rs:940` |
| IR | `m14.ll` 29906~30234행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `e8b200` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &LineDefenseSubPlan(3B: style@0 LineStyle, line@1 LineType, minion_action_type@2 MinionActionType) | line(+1)·minion_action_type(+2) 읽음. style 은 안 읽음 | 4 |
| 1 | 2 | version | usize | 콜리 4곳에 전달만 | 4 |
| 2 | 3 | parameter | &ScoreParameter(5384B) | 콜리 전달만 | 4 |
| 3 | 4 | rnd | &mut StdRng(320B) | evaluate_action·interaction_score·calculate_action_score 에 전달 — 이 함수 자체 write 0건 | 4 |
| 4 | 5 | player | &PlayerState(2528B) | info.team(+0x930)·info.position 태그(+0x9c0) | 4 |
| 5 | 6 | data | &OperationData(24B) | cache→player_champion / game.get_entity_by_id | 4 |
| 6 | 7 | action | &SmallActionPlay(184B) | get_action() 인라인(태그 +0xb1) · target_id(+0x8) | 4 |
| 7 | 8 | debug | &mut DebugFrameData(224B) | 콜리 전달만 — 이 함수 자체 write 0건 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // line_defense.rs:940
  // L941: let ctx = ActionContext{ priority: PriorityProfile::Lane, anchor: Anchor::Lane{ line: self.line } }  (alloca 40B: +0=2, +16=1, +17=self.line)
  if let Some(v) = evaluate_action(version, &ctx, parameter, rnd, player, data, action, debug) { return v }   // L941 ScalarPair {tag i64, v i64}: tag&1 → Some (29924~29927)
  let champ = data.cache.player_champion[player.info.team][player.info.position].unwrap();   // L944
  let base = interaction_score(version, rnd, player, data, parameter, action, debug);          // L945
  let action_type = self.minion_action_type;                                                    // L948 (+0x2)
  let economy_adjustment = line_action_economy_adjustment(version, player, data, parameter, action, action_type);   // L949
  let bonus = match action.get_action() {   // L950 (small_action.rs:309 인라인)
    SmallAction::Around{target_id} /*Play idx 2·3·10*/ => {                     // L978
      let Some(t) = game.get_entity_by_id(target_id) else { 0 };
      if t.team != champ.team { 0 }                                             // L979 (TeamType derive eq: 태그 같고, Player 면 번호까지)
      else if t.distance_sq(champ) > 39999999999 /*≥200000*/ {                  // L980~983
         match t.ty tag { 1 Minion | 2 Tower => 50 /*L984 is_any_type_minion 인라인*/, 3 Nexus => 100 /*L986*/, 13 Champion => 5 /*L988*/, _ => 0 }
      } else { 0 }
    }
    SmallAction::Attack{target_id} /*Play idx 12*/ => {                         // L954 (cast.rs:94)
      let Some(t) = game.get_entity_by_id(target_id) else { -99999 };
      let effect = champ.attack_effect.as_ref().unwrap();                        // L955 (+0x4c0 == -1 → 패닉)
      calculate_action_score(version, rnd, player, data, parameter, &champ.attack /*+0x570*/, effect /*+0x490*/, champ.attack_speed_mult(), t, action_type, debug)   // L956
    }
    SmallAction::Skill{target_id} /*Play idx 13*/ => {                          // L962 (cast.rs:160)
      let Some(t) = … else { -99999 };
      let effect = champ.skill_effect.as_ref().unwrap();                         // L963 (+0x4f8)
      calculate_action_score(…, &champ.skill /*+0x580*/, effect /*+0x4c8*/, champ.cooldown_reduce(false), t, action_type, debug)   // L964
    }
    SmallAction::Skill2{target_id} /*Play idx 14*/ => {                         // L970 (cast.rs:222)
      let Some(t) = … else { -99999 };
      let effect = champ.skill2_effect().unwrap();                               // L971 (entity.rs:1693: level(+0x5c8)>2 아니면 None → 패닉 ; +0x530 == -1 → 패닉)
      calculate_action_score(…, &champ.skill2 /*+0x590*/, effect /*+0x500*/, champ.cooldown_reduce(false), t, action_type, debug)   // L972
    }
    _ /*RunAway·Recall·AroundRegion·AroundRunAway·Positioning·AroundPosition·AroundPositionBush·AroundBush·Trace·Ult·Stop*/ => 0
  };
  base + economy_adjustment + bonus   // L950 (30074~30075) → L1005 반환
```

**`mem` 메모리 접근 29건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LineDefenseSubPlan | 0x1 | line (LineType) | r | m14.ll:29917 → ActionContext.anchor Lane{line} 로 복사 | 4 | OK |  |
| 1 | LineDefenseSubPlan | 0x2 | minion_action_type (MinionActionType 0=Pull 1=Normal 2=Push) | r | m14.ll:29974 → economy/calculate_action_score 인자 | 4 | OK |  |
| 2 | PlayerState | 0x930 | info.team | r | m14.ll:29937 | 4 | OK |  |
| 3 | PlayerState | 0x9c0 | info.position@tag | r | m14.ll:29953 | 4 | OK |  |
| 4 | OperationData | 0x0 | cache | r | m14.ll:29956 | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | m14.ll:29957~29960 None→unwrap_failed(30011) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | m14.ll:30018 등 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | m14.ll:30019 등 | 4 | OK |  |
| 8 | vtable(AbstractGame) | 0x1f0 | get_entity_by_id | r | m14.ll:30021~30023, 30036~30038, 30051~30053, 30066~30068 (4 사이트) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 9 | SmallActionPlay | 0xb1 | @tag (니치) | r | m14.ll:29980 | 4 | OK |  |
| 10 | SmallActionPlay | 0x8 | target_id (Around/AroundHide/LaneMinionPosition → SmallAction::Around ; Attack/Skill/Skill2 → cast.rs:94/160/222 target_id) | r | m14.ll:30015, 30030, 30045, 30060 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 11 | Entity | 0x0 | team@tag | r | t·champ, TeamType eq (m14.ll:30084, 30087) | 4 | OK |  |
| 12 | Entity | 0x8 | team@Player.0 | r | m14.ll:30104~30105 (태그 0=Player 일 때만 비교) | 4 | OK |  |
| 13 | Entity | 0x660 | x | r | t·champ distance_sq (m14.ll:30110, 30118) | 4 | OK |  |
| 14 | Entity | 0x668 | y | r | m14.ll:30114, 30122 | 4 | OK |  |
| 15 | Entity | 0x68 | ty@tag (EntityType) | r | t (m14.ll:30145) 1=Minion 2=Tower 3=Nexus 13=Champion | 4 | OK |  |
| 16 | Entity | 0x4c0 | attack_effect@tag (니치 i32, -1=None) | r | champ (m14.ll:30166) None→unwrap_failed | 4 | OK |  |
| 17 | Entity | 0x490 | attack_effect@Some.0 (&Effect 56B) | r | champ (m14.ll:30172) | 4 | OK |  |
| 18 | Entity | 0x570 | attack (Box<dyn Action>) | r | champ (m14.ll:30175) entity.rs:1494 | 4 | OK |  |
| 19 | Entity | 0x4f8 | skill_effect@tag | r | champ (m14.ll:30188) | 4 | OK |  |
| 20 | Entity | 0x4c8 | skill_effect@Some.0 | r | champ (m14.ll:30194) | 4 | OK |  |
| 21 | Entity | 0x580 | skill (Box<dyn Action>) | r | champ (m14.ll:30197) entity.rs:1665 | 4 | OK |  |
| 22 | Entity | 0x5c8 | level | r | champ (m14.ll:30209) skill2_effect(): level>2 아니면 None→unwrap_failed | 4 | OK |  |
| 23 | Entity | 0x530 | skill2_effect@tag | r | champ (m14.ll:30216) | 4 | OK |  |
| 24 | Entity | 0x500 | skill2_effect@Some.0 | r | champ (m14.ll:30226) | 4 | OK |  |
| 25 | Entity | 0x590 | skill2 (Box<dyn Action>) | r | champ (m14.ll:30230) entity.rs:1670 | 4 | OK |  |
| 26 | ActionContext(로컬 alloca 40B, evaluate_action 인자) | 0x0 | priority@tag = 2 (PriorityProfile::Lane, 니치 태그 2) | w | m14.ll:29923 — 스택 로컬. 힙/인자 write 아님 | 4 | OK | 2 |
| 27 | ActionContext(로컬 alloca 40B) | 0x10 | anchor@tag = 1 (Anchor::Lane) | w | m14.ll:29920 | 4 | OK | 1 |
| 28 | ActionContext(로컬 alloca 40B) | 0x11 | anchor@Lane.line = self.line | w | m14.ll:29922 | 4 | OK | self+0x1 |

**`consts` 상수 12건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 941 | 센티널 | PriorityProfile::Lane 메모리 태그(니치 start 2, tcxdict --enum PriorityProfile) — ActionContext.priority (m14.ll:29923). ⚠29939 의 2 는 팀 경계검사, 30211 의 2 는 level>2 게이트 | 3 |
| 1 | 1 | 941 | 태그 | Anchor::Lane 태그 (m14.ll:29920) | 4 |
| 2 | 39999999999 | 983 | 임계 | 200000²−1 — Around 대상(아군)과의 distance_sq > 이것(=거리 ≥200000) 일 때만 타입별 보너스 (m14.ll:30140) | 4 |
| 3 | 50 | 984 | 태그 | Around 아군 대상이 EntityType 태그 1(Minion)·2(Tower) 이면 보너스 (m14.ll:30073 phi ←%126) | 4 |
| 4 | 100 | 986 | 태그 | 대상 태그 3(Nexus) 보너스 (m14.ll:30073 ←%129) | 4 |
| 5 | 5 | 988 | 태그 | 대상 태그 13(Champion) 보너스 (m14.ll:30073 ←%130) | 4 |
| 6 | -99999 | 954 | 산출값 | Attack/Skill/Skill2 의 target_id 가 get_entity_by_id None 이면 보너스 -99999(사실상 배제) (m14.ll:30073 ←%59/%69/%79) | 4 |
| 7 | 3 | 984 | 센티널 | EntityType::Nexus 태그 (switch m14.ll:30147). ⚠29984 의 3 은 SmallActionPlay 니치 시작 | 4 |
| 8 | 13 | 984 | 태그 | EntityType::Champion 태그 (switch m14.ll:30147) | 4 |
| 9 | -1 | 955 | 센티널 | Option<Effect> 니치 None 판별값(i32) — attack_effect/skill_effect/skill2_effect 가 None 이면 unwrap_failed 패닉 (m14.ll:30168, 30190, 30218) | 4 |
| 10 | 0 | 979 | 태그 | TeamType 태그 0=Player → 팀 번호 비교 (m14.ll:30094) · bonus 기본 0 · Option 태그 None(29926 trunc) | 4 |
| 11 | 10 | 950 | 태그 | llvm.assume(tag != 10) get_action 인라인 아티팩트 (m14.ll:29982) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Around 아군 원거리 보너스 — 미니언/타워 | line_defense.rs:984 (m14.ll:30073) | 50 | 올리면 200k 밖 아군 미니언·타워 주변으로 가는 Around 행동이 더 자주 선택(라인 복귀 성향↑) | 4 | 기존 |
| 1 | Around 아군 원거리 보너스 — 넥서스 | line_defense.rs:986 | 100 | 올리면 넥서스 귀환형 Around 우선 | 4 | 기존 |
| 2 | Around 아군 원거리 보너스 — 챔피언 | line_defense.rs:988 | 5 | 올리면 멀리 있는 아군 챔피언 합류 Around 우선 | 4 | 기존 |
| 3 | 보너스 적용 거리 게이트 | line_defense.rs:983 (m14.ll:30140) | 39999999999 | 200000²−1. 내리면 더 가까운 아군 대상에도 보너스 | 4 | 기존 |
| 4 | 대상 없는 공격/스킬 배제값 | line_defense.rs:954/962/970 | -99999 | 덜 음수로 하면 대상이 사라진 Attack/Skill/Skill2 가 선택될 여지 생김(비권장) | 4 | 기존 |

<details><summary>`callees` 피호출자 21건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_speed_mult | game_core::Entity::attack_speed_mult | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:2433 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | calculate_action_score | game_ai::calculate_action_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, usize, &game_core::Entity, game_ai::MinionActionType, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:8 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | cooldown_reduce | game_core::Entity::cooldown_reduce | pub | fn(&game_core::Entity, bool) -> usize | game-core\src\simulation\entity.rs:2437 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | evaluate_action | game_ai::plan_legacy::action_eval::evaluate_action | pub | fn(usize, &game_ai::plan_legacy::action_eval::ActionContext, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> std::option::Option<i64> | game-ai\src\plan_legacy\action_eval.rs:67 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 7 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 8 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 9 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 16 | line_action_economy_adjustment | game_ai::line_action_economy_adjustment | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, game_ai::MinionActionType) -> i64 | game-ai\src\lane_economy.rs:6 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 18 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 19 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 20 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 1개**: `llvm.assume`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m01.ll:13373, m01.ll:13381, m12.ll:23095, m12.ll:37231, m14.ll:27726) · **형제 17개** (LineDefenseSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::LineDefenseSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::LineDefenseSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:15 | True | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, game_ai::MinionActionType, game_core::LineStyle) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::new_for_gank | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:20 | True | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, game_ai::MinionActionType, game_core::LineStyle) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 4 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_effect | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:24 | False | fn(&game_core::Entity, game_core::SmallAction) -> std::option::Option<&game_core::Effect> |
| 5 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_is_currently_in_range | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:34 | False | fn(&game_core::Entity, game_core::SmallAction, &game_core::Entity) -> bool |
| 6 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_walkup_position | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:38 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &game_core::Entity, &game_core::Entity, game_core::SmallAction, &game_core::Effect, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 7 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::positioning_score_at | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:68 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64) -> game_core::PositioningScore |
| 8 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::unsafe_v19_non_champion_walkup | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:72 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, bool, &mut game_core::DebugFrameData) -> bool |
| 9 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:154 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 10 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::get_move_action_v46 | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:200 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 11 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:368 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 12 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:372 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> |
| 13 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:396 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 14 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:440 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 15 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_candidates_old | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:881 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 16 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:940 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L984 에서 EntityType 태그 1(Minion)·2(Tower) 가 모두 50 인데, 소스가 `is_any_type_minion()` 하나(Minion\|Tower 포함)인지 `is_any_type_minion() \|\| is_tower()` 인지 — dbg 는 1261(is_any_type_minion)만 남고 switch 는 L984 루트라 표기 불가. 동작(1·2→50)은 확정 | 4 |  |
| 1 | 미탐색 | evaluate_action 의 ScalarPair 반환에서 None 경로의 두 번째 i64 는 미기록(undef) — 이 함수는 tag 비트만 보고 분기 | 4 |  |
| 2 | 미탐색 | self.style(+0x0) 미사용 — 의도 여부 불명 | 4 |  |
| 3 | 미탐색 | Ult(Play idx 15) 가 bonus 0 인 이유(match 팔 부재)는 소스 없이는 의도 판정 불가 — IR 정본은 0 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

