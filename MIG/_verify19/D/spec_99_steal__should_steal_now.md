---

### `99` steal::should_steal_now — [v4] 정글러 스틸 액션 결정 — 에픽/세르펜 각각 '적 3명+ 캠프 접근 가능·우리 팀이 안 치는 중·대상 hp<95%·최근 포기 아님' 후보를 걸러 evaluate_steal_for_target 로 평가하고 Commit > Commit > Lurk(둘 다면 적 예상 처치 잔여틱 짧은 쪽) > Lurk > None 으로 합친다

| 항목 | 값 |
|---|---|
| id | `steal__should_steal_now` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai11plan_legacy5steal16should_steal_now` |
| 소스 | `game-ai\src\plan_legacy\steal.rs:279` |
| IR | `m07.ll` 54198~55131행 |
| 경로·가시성 | `game_ai::plan_legacy::steal::should_steal_now` · **pub** |
| 계층 | 기타 |
| exe | `d9ac10` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData) -> game_ai::plan_legacy::steal::StealAction
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 분기 없음 (dbg 만) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | info.position@tag·info.team | 4 |
| 2 | 3 | goal_data | &GoalData(248B) | epic/serpen 스탠스(last_epic_seen·last_epic_hp·epic_enemy_tick) → evaluate 인자 | 4 |
| 3 | 4 | team_plan | &TeamPlan(1064B) | current_steal_session·last_battle_tick·objective·obj_spawn.*_giveup_tick 읽기 + enemy_could_be_at_camp 에 전달. store 0건 | 4 |
| 4 | 5 | data | &OperationData(24B) | cache(+0)·context(+8) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn should_steal_now(version, player, goal_data, team_plan, data) -> StealAction
  // L286~287
  if player.info.position != Jungle { return None }
  let Some(my_champ) = cache.player_champion[team][position] else { return None }
  // L291~299
  in_session_target = team_plan.current_steal_session.map(|s| s.target)     // None=2 / Epic=0 / Serpen=1
  if my_champ.hp*2 < my_champ.stat_cached.hp { return None }                // L294 hp<50%
  if in_session_target.is_none() && game.tick().saturating_sub(team_plan.last_battle_tick) < tps*5 { return None }   // L299
  // L304~310: 캠프 존에 있을 수 있는 적 수 (5슬롯 count)
  epic_zone_short  = !epic_exists(ctx)   || (0..5).filter(|p| enemy_could_be_at_camp(team, cache, team_plan, p, 288000, 288000)).count() < 3   // L329 조건으로 소비
  serpen_zone_short= !serpen_exists(ctx) || (0..5).filter(|p| enemy_could_be_at_camp(team, cache, team_plan, p, 672000, 672000)).count() < 3   // L358
  // L315~316
  epic_alive   = epic_exists && mob.epic.live_list.len != 0
  serpen_alive = serpen_exists && mob.serpen.live_list.len != 0
  // L319~345: 에픽 후보
  epic_action: (StealAction, usize) =
    if in_session_target == Some(Epic) {                                         // L320~321
      if epic_alive && objective != Morgard { evaluate(Epic, in_session=true) } else { (None, MAX) }
    } else {
      epic_eligible = mob.epic.live_list.get(0) → get_entity_by_id → is_some_and(|e| e.hp*100 < e.stat_cached.hp*95)   // L323~325
      if epic_alive && objective != Morgard(0)                                     // L326~327
         && !obj_spawn.epic_giveup_tick.is_some_and(|t| t + tps*5 >= tick) && !epic_zone_short   // L328~329
         && epic_eligible                                                          // L333
      { evaluate(Epic, in_session = (in_session_target==Some(Epic)) → 여기선 false) } else { (None, MAX) }   // L334~345
    }
    where evaluate(Epic, s) = evaluate_steal_for_target(player, cache, ctx, my_champ, target=false, epic_entity, goal_data.epic.last_epic_hp, goal_data.epic.epic_enemy_tick, goal_data.epic.last_epic_seen, (288000,288000), s)
  // L348~370: 세르펜 후보 (대칭)
  serpen_action =
    if in_session_target == Some(Serpen) { if serpen_alive && objective != Serpen(1) { evaluate(Serpen, true) } else { skip } }   // L350
    else if serpen_alive && objective != Serpen && !serpen_giveup_tick.is_some_and(|t| t+tps*5 >= tick) && !serpen_zone_short && serpen_eligible(hp<95%) { evaluate(Serpen, false) } else { skip }   // L352~362
    where evaluate(Serpen, s) = evaluate_steal_for_target(..., target=true, serpen_entity, goal_data.serpen.last_epic_hp, .epic_enemy_tick, .last_epic_seen, (672000,672000), s)
  // L376~390: 합성
  if epic_action.0 == Commit { return epic_action.0 }                     // 에픽 Commit 최우선
  if serpen skipped { return if epic.0==Lurk { Lurk(Epic) } else { None } }
  if serpen_action.0 == Commit { return Commit(Serpen) }
  match (epic.0, serpen.0) {
    (Lurk, Lurk) => if in_session_target.is_none() { Lurk(if epic.1 > serpen.1 { Serpen } else { Epic }) }   // L383: 적 예상 처치 잔여틱(enemy_tick) 작은 쪽
                    else { Lurk(in_session_target) }                                                          // L380
    (_, Lurk) => Lurk(Serpen),  (Lurk, _) => Lurk(Epic),  _ => None }
```

**`mem` 메모리 접근 30건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x9c0 | info.position@tag | r | 1=Jungle 아니면 None (L286, m07.ll:54236) | 4 | OK |
| 1 | PlayerState | 0x930 | info.team | r | team (L287) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r |  | 4 | OK |
| 3 | OperationData | 0x8 | context | r |  | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion | r | [team][position] = my_champ (L287) | 4 | OK |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr | r | vtable +0x28 tick / +0x40 get_game_mode / +0x1f0 get_entity_by_id | 4 | OK |
| 6 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r |  | 4 | OK |
| 7 | GameContext | 0x8 | setting | r | tps | 4 | OK |
| 8 | GameContext | 0x38 | tutorial | r | epic_exists(rule_scope.rs:46→runner.rs:263: 태그 1..6 이면 false) · serpen_exists(rule_scope.rs:50→runner.rs:267: {0,5,7,8}) | 4 | OK |
| 9 | GameSetting | 0x12f8 | tick_per_second | r | tps (L292) | 4 | OK |
| 10 | Entity | 0x670 | hp | r | my_champ.hp*2 < stat_cached.hp → None (L294) · 오브젝트 hp*100 < max*95 (L325/L354) | 4 | OK |
| 11 | Entity | 0x628 | stat_cached.hp | r |  | 4 | OK |
| 12 | TeamPlan | 0x1ba | current_steal_session@tag | r | 2=None → in_session_target=None (L291, +442) | 4 | OK |
| 13 | TeamPlan | 0x1b9 | current_steal_session@Some.0.target@tag | r | in_session_target (0 Epic/1 Serpen, +441) | 4 | OK |
| 14 | TeamPlan | 0x220 | last_battle_tick | r | 세션 없을 때 tick - last_battle_tick < tps*5 → None (L299, +544) | 4 | OK |
| 15 | TeamPlan | 0x41f | objective | r | 0=Morgard 이면 에픽 후보 제외 / 1=Serpen 이면 세르펜 후보 제외 (team_plan.rs:244→231 인라인, +1055) | 4 | OK |
| 16 | TeamPlan | 0x50 | obj_spawn.epic_giveup_tick@tag | r | Option<usize> 태그(8B, 1=Some) (+80, L328) | 4 | OK |
| 17 | TeamPlan | 0x58 | obj_spawn.epic_giveup_tick@Some.0 | r | t + tps*5 >= tick 이면 에픽 후보 제외 (+88) | 4 | OK |
| 18 | TeamPlan | 0x60 | obj_spawn.serpen_giveup_tick | r | (+96, L357) | 4 | OK |
| 19 | TeamPlan | 0x68 | obj_spawn.serpen_giveup_tick@Some.0 | r | (+104) | 4 | OK |
| 20 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.buf.inner.ptr | r | epic 엔티티 id (+416) | 4 | OK |
| 21 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | epic_alive (+424, L315) | 4 | OK |
| 22 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.buf.inner.ptr | r | (+464) | 4 | OK |
| 23 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | serpen_alive (+472, L316) | 4 | OK |
| 24 | GoalData | 0x78 | epic.last_epic_seen | r | (+120, L341) → evaluate last_seen | 4 | OK |
| 25 | GoalData | 0x80 | epic.last_epic_hp | r | (+128, L339) → last_hp | 4 | OK |
| 26 | GoalData | 0x88 | epic.epic_enemy_tick | r | (+136, L340) → enemy_tick (last_seen 시점 예상 적 처치 소요 틱 — evaluate 명세 참조) | 4 | OK |
| 27 | GoalData | 0xb0 | serpen.last_epic_seen | r | (+176, L370) | 4 | OK |
| 28 | GoalData | 0xb8 | serpen.last_epic_hp | r | (+184, L368) | 4 | OK |
| 29 | GoalData | 0xc0 | serpen.epic_enemy_tick | r | (+192, L369) | 4 | OK |

**`consts` 상수 20건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1 | 286 | 태그 | Position 태그 1=Jungle — 정글러만 (본문 `shl 1` 은 L294 hp*2 항목) | 4 |  |
| 1 | 2 | 291 | 센티널 | Option<StealSession> None 니치 = 2 (+0x1ba) → in_session_target None. 본문 다른 2 = player_champion bounds·StealAction Commit 태그 | 4 |  |
| 2 | 1 | 294 | 태그 | my_champ.hp*2 < stat_cached.hp (=hp<50%) 이면 None — `shl i64 %29, 1` 로 접힘 | 4 | 2 |
| 3 | 5 | 299 | 태그 | tps*5: 세션 중이 아닐 때 마지막 교전(last_battle_tick) 후 5초 이내면 None. L328/L357 giveup_tick + tps*5 >= tick(최근 5초 내 포기) 에도 같은 5 | 4 |  |
| 4 | 6 | 304 | 임계 | epic_exists: (tutorial 태그-1) < 6 → 태그 1..6 이면 에픽 없음 (runner.rs:263) | 4 |  |
| 5 | 288000 | 305 | 미상 | 에픽 캠프 존 좌표 (x=y=288000 = 9셀) — enemy_could_be_at_camp(team, cache, team_plan, p, 288000, 288000) 5슬롯 | 4 |  |
| 6 | 672000 | 310 | 미상 | 세르펜 캠프 존 좌표 (x=y=672000 = 21셀) | 4 |  |
| 7 | 3 | 329 | 임계 | epic_enemies_at_zone < 3 이면 에픽 후보 제외 (즉 적 3명 이상이 캠프에 있을 수 있어야 스틸 대상). L358 세르펜도 3 | 4 |  |
| 8 | 0 | 309 | 태그 | TutorialType 태그 0=None: serpen_exists 집합 {0,5,7,8} | 4 |  |
| 9 | 5 | 309 | 태그 | TutorialType 5=MidBottom | 4 |  |
| 10 | 7 | 309 | 태그 | TutorialType 7=Line | 4 |  |
| 11 | 8 | 309 | 태그 | TutorialType 8=Total | 4 |  |
| 12 | 100 | 325 | 계수 | obj.hp*100 < obj.stat_cached.hp*95 — 대상 hp 95% 미만이어야 eligible (세션 중이 아닐 때만 검사) | 4 |  |
| 13 | 95 | 325 | 계수 | 95% 임계 (L354 세르펜 동일) | 4 |  |
| 14 | 0 | 327 | 태그 | MainObjective 태그 0=Morgard: 우리 팀 objective 가 Morgard 면 에픽 스틸 후보 제외 (우리가 치는 중) — 세션 중(L321)에도 적용 | 4 |  |
| 15 | 1 | 356 | 태그 | MainObjective 태그 1=Serpen: objective 가 Serpen 이면 세르펜 후보 제외 (L350 세션 중에도). `shl 1` 과 무관 | 4 |  |
| 16 | -1 | 345 | 센티널 | epic_action 미평가 시 (None, usize::MAX) — 비교 키(잔여틱) 최대값 | 4 |  |
| 17 | 2 | 376 | 태그 | StealAction 태그 2=Commit — 에픽 Commit 우선, 다음 세르펜 Commit | 4 |  |
| 18 | 1 | 376 | 태그 | StealAction 태그 1=Lurk. `shl 1` 과 무관 | 4 |  |
| 19 | 0 | 390 | 태그 | StealAction 태그 0=None (조기 반환 전부) | 4 |  |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 내 hp 최소 | steal.rs:294 | 50% (hp*2 < max) | 내리면 저체력에도 스틸 시도 | 4 | 기존 |
| 1 | 교전 직후 스틸 금지 창 | steal.rs:299 | tps*5 | 줄이면 교전 끝나자마자 스틸 판단 재개 | 4 | 기존 |
| 2 | 캠프 존 적 최소 인원 | steal.rs:329 / :358 | 3 | 내리면(2) 적 2명만 캠프 근처여도 스틸 후보로 본다 — 오판(적이 안 치는데 스틸 시도) 증가 | 4 | 기존 |
| 3 | 대상 손상 임계 | steal.rs:325 / :354 | 95% | 올리면(예: 100) 풀피 오브젝트도 후보 — 사실상 '적이 치기 시작했나' 게이트 | 4 | 기존 |
| 4 | 포기 후 재진입 금지 창 | steal.rs:328 / :357 | tps*5 | giveup_tick 후 5초. 줄이면 포기 직후 재시도 | 4 | 기존 |
| 5 | 에픽/세르펜 존 좌표 | steal.rs:305 / :310 | (288000,288000) / (672000,672000) | 맵 캠프 위치 하드코딩 — 맵 모드에서 캠프를 옮기면 어긋난다 | 4 | 기존 |

<details><summary>`callees` 피호출자 7건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | enemy_could_be_at_camp | game_ai::plan_legacy::steal::enemy_could_be_at_camp | in:game_ai::plan_legacy::steal | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, (u64, u64)) -> bool | game-ai\src\plan_legacy\steal.rs:241 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | evaluate_steal_for_target | game_ai::plan_legacy::steal::evaluate_steal_for_target | in:game_ai::plan_legacy::steal | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::Entity, game_core::StealTarget, std::option::Option<&game_core::Entity>, usize, usize, usize, (u64, u64), bool) -> (game_ai::plan_legacy::steal::StealAction, usize) | game-ai\src\plan_legacy\steal.rs:392 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | should_steal_now | game_ai::plan_legacy::steal::should_steal_now | pub | fn(usize, &game_core::PlayerState, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData) -> game_ai::plan_legacy::steal::StealAction | game-ai\src\plan_legacy\steal.rs:279 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 5 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 6 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 3개**: `epic_exists`, `evaluate`, `serpen_eligible`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:24533) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | enemy_could_be_at_camp(team, cache, team_plan, slot, x, y) 내부 미독해 — 이름·인자로 '적 슬롯 p 가 (x,y) 캠프에 있을 수 있는가(시야/마지막 관측 기반)' 로 읽힘. 별도 define(fastcc) 이나 이 배치 범위 밖 | 4 |  |
| 1 | 미탐색 | team_plan.rs:244→231 인라인 헬퍼 이름 — objective 태그==0/1 비교만 남음. _docs '`matches!(objective, Morgard{..}/Serpen{..})` 치환 — Take 운영이 그 캠프에 활성인가' 와 부합 (is_take_active 류) | 4 |  |
| 2 | 미탐색 | L334 evaluate 의 in_session 인자: 비세션 경로 phi %151 은 in_session_target==Some(Epic) 일 때만 1 — 그 경로는 L320 분기로 이미 갈라져 있어 실제론 항상 false. 세션 경로(%223)는 true | 4 |  |
| 3 | 미탐색 | L383 비교 키 = evaluate_steal_for_target 반환 (StealAction, usize) 의 usize = enemy_tick(적 예상 처치 소요틱) — 작은 쪽 우선. 미평가 시 usize::MAX(-1) | 4 |  |
| 4 | 미탐색 | exe 패닉 Location 4개 vs IR unwrap 6곳 — 337·366 짝이 exe 에서 안 보임(동일 Location 공유/병합 추정, Ghidra 미확인) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

