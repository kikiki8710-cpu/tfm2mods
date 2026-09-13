---

### `83` EpicHuntAndPokePlan::sub_plan — 에픽(Morgard) 사냥/견제 빅플랜의 서브플랜 선택 — Steal/EpicCheck/Recall/LineDefense/EpicHunt/EpicPoke 중 하나를 sret 로 반환

| 항목 | 값 |
|---|---|
| id | `epic_hunt_and_poke__sub_plan` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic13hunt_and_pokeNtB2_19EpicHuntAndPokePlan8sub_plan` |
| 소스 | `game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:30` |
| IR | `m10.ll` 7902~9384행 |
| 경로·가시성 | `game_ai::plan_legacy::old::EpicHuntAndPokePlan::sub_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `defcd0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut SubPlan(72B) | %0. 태그 = enum+0x0 (니치: tag = idx+2). 반환 variant 는 writes 참조 | 4 |
| 1 | 1 | self | &mut EpicHuntAndPokePlan(32B) | %1. v46_flee_threats(Vec, +0x0)·focus_epic_only(+0x18)·vision_only(+0x19)·v46_flee(+0x1a) 읽고 v46 필드는 갱신도 함 | 4 |
| 2 | 2 | version | usize | %2. 본문 분기 없음 — upgrade_item·v46_flee_gate_check·v25_objective_splitter_can_stay·v24_objective_setup_should_check_camp 에 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | %3. upgrade_item·PlayerState::strategy 에 전달 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | %4. info.team(0x930)·info.position 태그(0x9c0) | 4 |
| 5 | 5 | data | &OperationData(24B) | %5. cache(+0)·context(+8)·blackboard(+0x10) | 4 |
| 6 | 6 | goal_data | &GoalData(248B) | %6. epic.epic_enemy_tick(0x88)·epic.epic_ally_tick(0x98) — check_recall 인라인분에서만 | 4 |
| 7 | 7 | team_plan | &TeamPlan(1064B) | %7. objective(0x41f 태그, 0x420 phase) + v24_objective_setup_should_check_camp 의 self | 4 |
| 8 | 8 | debug | &mut DebugFrameData(224B) | %8. ctx.debug 일 때 v46 도주 로그 add_log 에만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// hunt_and_poke.rs:30~96 (+ check_recall :103~160 인라인). 분기 순서대로.
t = player.info.team; pos = player.info.position
champ = cache.player_champion[t][pos].unwrap()                                   // :32

// ── A. 스틸 모드 (:35~44)
if self.focus_epic_only || self.vision_only {
   moba = game.get_game_mode() (vtable+0x40) → tag 0 = Moba 아니면 unwrap 패닉      // :36
   epic = moba.jungle_runner.epic.live_list.first().and_then(|id| game.get_entity_by_id(id))   // :37 closure$0
   if epic.is_some() → return Steal{ last_vision_tick:0, target:Some(Epic), commit:self.focus_epic_only }   // :39
   else → return EpicCheck{ move_check:false }                                        // :44
}

// ── B. 일반 (:47~)
hp_ratio = champ.hp*100 / champ.stat_cached.hp                                    // :47 (max_hp 0 → 패닉)
can_upgrade_item = upgrade_item(version, rnd, player, game, ctx).is_some()          // :48
epic = moba.jungle_runner.epic.live_list.first().and_then(get_entity_by_id)         // :49 closure$1
if epic.is_none() → return EpicCheck{false}                                        // :50~51
(x0,y0,x1,y1) = map.fountains[t]                                                    // :56
is_in_heal_area = x0<=champ.x<=x1 && y0<=champ.y<=y1                                // :57
if epic.hp == epic.stat_cached.hp /*에픽 풀피*/ && (hp_ratio < 51 || can_upgrade_item || (is_in_heal_area && champ.hp < champ.stat_cached.hp))   // :58 (줄 안 순서 표기 불가)
   → return Recall                                                                   // :59

// ── C. 팀 오브젝트 게이트 (:63~64)
if team_plan.objective != Some(Morgard{..}) → return Recall                         // :63 objective_target / :94
if phase == Hunt {                                                                   // :64 take_hunt_commit
   strategy = player.strategy(rnd, game)                                             // :65
   if strategy.object_buildup == Split(position) && position == pos                  // :66
      && v25_objective_splitter_can_stay(version, player, data, Morgard)             // :68
      → return LineDefense{ style:Aggressive, line:fallback_line(ctx, Bottom), minion_action_type:Push }   // :70~73
   else → return EpicHunt{ need_recall:false }                                       // :77
}

// ── D. check_recall(self, version, player, data, goal_data, debug) (:79, 본문 :103~160)
   champ = player_champion[t][pos].unwrap()                                          // :104
   if heal_area(t).contains(champ) && champ.hp < champ.max_hp → true                 // :107~108 (game_core heal_area 인라인: 팀0 x<=64000&&y>=896000&&y<=960000 / 팀1 x>=892000&&x<=960000&&y<=64000)
   hp_ratio = champ.hp*100/champ.max_hp                                              // :112
   if min(goal_data.epic.epic_enemy_tick, epic_ally_tick) <= tps*5 → false           // :115
   if self.v46_flee {                                                                // :124
      if self.v46_flee_threats.iter().any(|id| game.get_entity_by_id(id).is_some_and(|e| dist_sq(e,champ) < 40000000001)) → true   // :125~126
      self.v46_flee=false; self.v46_flee_threats.clear()                              // :131
   } else {
      (code, threats) = v46_flee_gate_check(version, player, data, champ)             // :134
      if code == 0 { self.v46_flee=true; self.v46_flee_threats = threats.to_vec();    // :136~137
                     if ctx.debug { debug.add_log(data, player, format!(.. tick, position, .. threats)) }   // :138~139
                     return true }                                                    // :140
   }
   enemy_cnt = player_champion[1-t].iter().flatten().filter(|e| map.regions[e.y/32000][e.x/32000]==7 && blackboard[1-t].is_recent_visible(game, player, e)).count()   // :147~149
   ally_cnt  = player_champion[t].iter().flatten().filter(|a| regions==7 && blackboard[t].is_recent_visible(game, player, a)).count()                 // :154~155
   return hp_ratio < 21 && ally_cnt < enemy_cnt                                      // :160
if check_recall(..) → return Recall                                                  // :79~80

// ── E. 캠프 확인/견제 (:82~89)
if team_plan.v24_objective_setup_should_check_camp(version, player, data, goal_data, Morgard) → return EpicCheck{false}   // :82~83
if dist_sq(champ, epic) < 22500000001 → return EpicPoke                             // :86, :89
else → return EpicCheck{false}                                                       // :87
```

**`mem` 메모리 접근 41건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | EpicHuntAndPokePlan | 0x18 | focus_epic_only | r | 스틸 Commit 플래그. focus_epic_only\|\|vision_only 면 Steal 경로(:35) | 4 | OK |  |
| 1 | EpicHuntAndPokePlan | 0x19 | vision_only | r | 스틸 Lurk 플래그 | 4 | OK |  |
| 2 | EpicHuntAndPokePlan | 0x1a | v46_flee | r | 도주 에피소드 진행 중 플래그(check_recall :124) | 4 | OK |  |
| 3 | EpicHuntAndPokePlan | 0x8 | v46_flee_threats.ptr | r | Vec<usize>(위협 엔티티 id) 버퍼 — :125 순회 | 4 | OK |  |
| 4 | EpicHuntAndPokePlan | 0x10 | v46_flee_threats.len | r | :125 순회 상한 / :131 clear 로 0 씀 | 4 | OK |  |
| 5 | PlayerState | 0x930 | info.team | r | t. 적 팀 = 1-t. 2 이상이면 panic_bounds_check(:32) | 4 | OK |  |
| 6 | PlayerState | 0x9c0 | info.position@tag | r | 내 포지션(i32). player_champion[t][pos] 및 Split(position) 비교(:66) | 4 | OK |  |
| 7 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 8 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 9 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. 적 카운트는 [1-t], 아군 카운트는 [t] 로 is_recent_visible | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable(816B). 슬롯 +0x28 tick · +0x40 get_game_mode · +0x1f0 get_entity_by_id (divtable AbstractGame) | 3 | OK |  |
| 12 | AbstractGameWithCache | 0x1e0 | player_champion | r | [t][pos]=내 챔프(None→unwrap_failed), [1-t][0..5]=적, [t][0..5]=아군(check_recall 카운트, 5회 언롤) | 4 | OK |  |
| 13 | GameContext | 0x8 | setting | r | &GameSetting → tick_per_second | 4 | OK |  |
| 14 | GameContext | 0x20 | map | r | &MapDef → fountains·regions | 4 | OK |  |
| 15 | GameContext | 0x3b | debug | r | true 면 v46 도주 진입 시 add_log(:138~139) | 4 | OK |  |
| 16 | GameSetting | 0x12f8 | tick_per_second | r | tps. check_recall :115 임계 tps*5 | 4 | OK |  |
| 17 | MapDef | 0x6d70 | fountains | r | [2] x (x0,y0,x1,y1) 32B stride. is_in_heal_area(:56~57): x0<=x<=x1 && y0<=y<=y1 | 4 | OK |  |
| 18 | MapDef | 0x38b8 | regions[cy][cx] | r | [30][30] usize. clamp(…,29). 값 7 인 셀에 있는 챔프만 check_recall 카운트(:148, :154) | 4 | OK |  |
| 19 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | live_list[0] = 살아있는 에픽 id | 4 | OK |  |
| 20 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 이면 에픽 없음 → EpicCheck | 4 | OK |  |
| 21 | TeamPlan | 0x41f | objective | r | Option<MainObjective> 태그. 0 = Some(Morgard) 만 통과(:63), 아니면 Recall(:94) | 4 | OK |  |
| 22 | TeamPlan | 0x420 | objective@Some.0@Morgard.phase | r | ObjectPhase. 3=Hunt 면 take_hunt_commit 경로(:64~77) | 4 | OK |  |
| 23 | GoalData | 0x88 | epic.epic_enemy_tick | r | check_recall :115 min(enemy_tick, ally_tick) | 4 | OK |  |
| 24 | GoalData | 0x98 | epic.epic_ally_tick | r |  | 4 | OK |  |
| 25 | Entity | 0x628 | stat_cached.hp | r | 내 max HP(:47 / :112 — 0 이면 panic_const_div_by_zero) · 에픽 max HP(:58) | 4 | OK |  |
| 26 | Entity | 0x660 | x | r | 내 x(fountain·heal_area·region·dist), 에픽 x, 위협 x | 4 | OK |  |
| 27 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 28 | Entity | 0x670 | hp | r | 내 hp · 에픽 hp | 4 | OK |  |
| 29 | Strategy | 0x0 | object_buildup | r | PlayerState::strategy() 결과(24B). 니치 enum: Split(position) 은 태그 없이 Position(i32) 그대로 → 내 pos 와 == 비교(:66) | 4 | OK |  |
| 30 | SubPlan | 0x0 | tag | w | Steal=18 / EpicCheck=10 / Recall=5 / LineDefense=2 / EpicHunt=11 / EpicPoke=12 (tcxdict --enum SubPlan 메모리태그) | 3 | OK | 18\|10\|5\|2\|11\|12 |
| 31 | SubPlan | 0x8 | Steal.last_vision_tick | w | :39 | 4 | OK | 0 |
| 32 | SubPlan | 0x10 | Steal.target | w | StealTarget 태그 0 = Epic | 4 | OK | Some(Epic)=0 |
| 33 | SubPlan | 0x11 | Steal.commit | w | focus_epic_only=true → Commit(공격 포함), vision_only 만 → Lurk | 4 | OK | self.focus_epic_only |
| 34 | SubPlan | 0x8 | EpicCheck.move_check | w | :44 :51 :83 :87 전부 0 | 4 | OK | false(0) |
| 35 | SubPlan | 0x8 | EpicHunt.need_recall | w | :77 | 4 | OK | false(0) |
| 36 | SubPlan | 0x8 | LineDefense.style | w | :73 | 4 | OK | Aggressive(0) |
| 37 | SubPlan | 0x9 | LineDefense.line | w | :70 | 4 | OK | fallback_line(ctx, Bottom=2) |
| 38 | SubPlan | 0xa | LineDefense.minion_action_type | w | :73 | 4 | OK | Push(2) |
| 39 | EpicHuntAndPokePlan | 0x1a | v46_flee | w | check_recall 인라인분 | 4 | OK | true(:136) / false(:131) |
| 40 | EpicHuntAndPokePlan | 0x0 | v46_flee_threats | w | std Vec<usize> 24B 통째 교체(:137, memcpy 24B) 또는 clear | 4 | OK | gate.1.to_vec()(:137) / len=0(:131) |

**`consts` 상수 23건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 36 | 태그 | get_game_mode(vtable+0x40) 반환 {tag,ptr} 의 tag 0 = Moba (as_moba, game.rs:231). 아니면 unwrap_failed | 4 |
| 1 | 100 | 47 | 계수 | hp_ratio = hp*100/max_hp (max_hp==0 이면 div_by_zero 패닉 — build 와 달리 max(.,1) 없음) | 4 |
| 2 | 51 | 58 | 임계 | hp_ratio < 51 (=HP 50% 이하) 이면 Recall 후보. `<51` vs `<=50` 표기 불가 | 4 |
| 3 | 10 | 51 | 태그 | SubPlan 메모리태그 10 = EpicCheck (idx 8 + 2) | 4 |
| 4 | 5 | 59 | 태그 | SubPlan 메모리태그 5 = Recall (idx 3 + 2) | 4 |
| 5 | 3 | 64 | 태그 | ObjectPhase 태그 3 = Hunt (take_hunt_commit, team_plan.rs:249 인라인). ※본문의 `shl i64 %len, 3` 은 Vec<usize> 인덱스 stride(×8B)이지 이 상수가 아님 | 4 |
| 6 | 4 | 68 | 태그 | JungleType 태그 4 = Morgard — v25_objective_splitter_can_stay / v24_objective_setup_should_check_camp 의 target 인자 | 4 |
| 7 | 2 | 70 | 태그 | LineType 태그 2 = Bottom — fallback_line(ctx, Bottom) 인자. 같은 값 2 가 :73 LineDefense 태그(2)·MinionActionType Push(2) 로도 쓰임 | 4 |
| 8 | 11 | 77 | 태그 | SubPlan 메모리태그 11 = EpicHunt | 4 |
| 9 | 64000 | 107 | 산출값 | [game_core heal_area(game.rs:319) 인라인] 팀0 샘: x<=64000 && 896000<=y<=960000 / 팀1 샘: 892000<=x<=960000 && y<=64000 (2셀 = 64000) | 4 |
| 10 | 960000 | 107 | 산출값 | heal_area 인라인 — 맵 끝(30셀×32000) | 4 |
| 11 | 891999 | 107 | 임계 | heal_area 인라인 — 팀1 x > 891999 (= x>=892000 = 960000-68000). exe 는 0xd9c60(892000) 로 인코딩 | 4 |
| 12 | 896000 | 107 | 임계 | heal_area 인라인 — 팀0 y >= 896000 (= 960000-64000) | 4 |
| 13 | 5 | 115 | 계수 | check_recall: min(epic_enemy_tick, epic_ally_tick) <= tps*5 (5초) 이면 즉시 false(귀환 안 함) — 최근 5초 내 에픽 근처에 적/아군 관측이 있으면 귀환 판단 자체를 건너뜀 | 4 |
| 14 | 40000000001 | 126 | 임계 | 200000^2 + 1 — v46 도주 위협 엔티티가 200000(6.25셀) 이내면 계속 도주(Recall) | 4 |
| 15 | 0 | 135 | 태그 | v46_flee_gate_check 반환 (u8, Vec<usize>) 의 .0 == 0 이면 도주 발동. 나머지 코드 1~5 의 의미는 passive_line.rs:38 본문 미탐색 | 4 |
| 16 | 7 | 148 | 태그 | MapDef.regions 셀 값 7 — 이 리전에 서 있는 최근가시 적/아군만 센다(리전 7 의 의미(에픽 둥지 추정)는 MapDef 데이터 미확인) | 5 |
| 17 | 29 | 148 | 인덱스 | regions 인덱스 clamp 상한(30x30 그리드) | 4 |
| 18 | 32000 | 148 | 인덱스 | 셀 크기 — 좌표/32000 = 그리드 인덱스 | 4 |
| 19 | 21 | 160 | 임계 | check_recall 최종: hp_ratio < 21 (HP 20% 이하) && 리전7 아군수 < 리전7 적수 → 귀환 | 4 |
| 20 | 22500000001 | 86 | 임계 | 150000^2 + 1 — 에픽과의 거리 ≤150000 이면 EpicPoke, 아니면 EpicCheck | 4 |
| 21 | 12 | 89 | 태그 | SubPlan 메모리태그 12 = EpicPoke | 4 |
| 22 | 18 | 39 | 태그 | SubPlan 메모리태그 18 = Steal | 4 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 귀환 HP 임계(에픽 풀피일 때) | hunt_and_poke.rs:58 | 51 | 올리면 더 건강한 상태에서도 에픽이 아직 안 맞았으면 귀환. 내리면 저체력으로 버티다 사냥 진입 | 4 | 기존 |
| 1 | check_recall 저체력 임계 | hunt_and_poke.rs:160 | 21 | hp<21% 이고 리전7 적이 아군보다 많으면 귀환. 올리면 귀환이 잦아지고(사냥 포기), 내리면 끝까지 버팀 | 4 | 기존 |
| 2 | 에픽 근처 관측 유예 | hunt_and_poke.rs:115 | 5 | tps*5초. 최근 5초 내 에픽 근처 관측(적/아군)이 있으면 귀환 판단을 건너뜀. 올리면 귀환 판단이 더 오래 봉인됨 | 4 | 기존 |
| 3 | v46 도주 위협 유지 반경 | hunt_and_poke.rs:126 | 40000000001 | 200000^2+1. 올리면 위협이 더 멀어도 도주(Recall)를 유지, 내리면 도주 에피소드가 빨리 풀림 | 4 | 기존 |
| 4 | 견제(EpicPoke) 진입 거리 | hunt_and_poke.rs:86 | 22500000001 | 150000^2+1. 올리면 더 먼 거리에서도 EpicPoke 로 전환(견제 적극), 내리면 EpicCheck 로 더 오래 머묾 | 4 | 기존 |
| 5 | 스플리터 잔류 시 라인 | hunt_and_poke.rs:70 | 2 | fallback_line(ctx, Bottom). Top(0)/Mid(1) 로 바꾸면 Hunt 국면에 남는 스플리터가 가는 라인이 달라짐 | 4 | 기존 |
| 6 | 카운트 대상 리전 | hunt_and_poke.rs:148 | 7 | regions 값 7 셀의 챔프만 세어 수적 열세를 판단. 다른 리전 값으로 바꾸면 판단 위치가 달라짐 | 4 | 기존 |

<details><summary>`callees` 피호출자 22건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_log | game_core::DebugFrameData::add_log | pub | fn(&mut game_core::DebugFrameData, &game_core::OperationData, &game_core::PlayerState, std::string::String) | game-core\src\simulation\game\frame.rs:54 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check_recall | game_ai::plan_legacy::old::SinglePlanLine::check_recall | in:game_ai::plan_legacy::old::single_line | fn(&game_ai::plan_legacy::old::SinglePlanLine, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\single_line.rs:281 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | check_recall | game_ai::plan_legacy::old::PassiveLinePlan::check_recall | in:game_ai::plan_legacy::old::passive_line | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_line.rs:1068 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | check_recall | game_ai::plan_legacy::old::PassiveJunglePlan::check_recall | in:game_ai::plan_legacy::old::passive_jungle | fn(&game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:142 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | clear | game_core::DataTable::<T>::clear | pub | fn(&mut game_core::DataTable<T/#0>) | game-core\src\data.rs:2259 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | contains | game_core::RectU64::contains | pub | fn(&game_core::RectU64, u64, u64) -> bool | game-core\src\setting.rs:40 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | contains | game_core::RectI64::contains | pub | fn(&game_core::RectI64, i64, i64) -> bool | game-core\src\setting.rs:55 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | contains | game_core::setting::champion::nightmare::NightmareWellRect::contains | in:game_core::setting::champion::nightmare | fn(game_core::setting::champion::nightmare::NightmareWellRect, u64, u64) -> bool | game-core\src\setting\champion\nightmare.rs:28 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 8 | fallback_line | game_ai::plan_legacy::rule_scope::fallback_line | pub | fn(&game_core::GameContext, game_core::LineType) -> game_core::LineType | game-ai\src\plan_legacy\rule_scope.rs:28 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | heal_area | game_core::Game::heal_area | pub | fn(usize) -> (u64, u64, u64, u64) | game-core\src\simulation\game.rs:318 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | upgrade_item | game_ai::upgrade_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1603 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | v24_objective_setup_should_check_camp | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | v25_objective_splitter_can_stay | game_ai::plan_legacy::team_plan::v25_objective_splitter_can_stay | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:143 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | v46_flee_gate_check | game_ai::plan_legacy::old::passive_line::v46_flee_gate_check | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> (u8, bumpalo::collections::vec::Vec< usize>) | game-ai\src\plan_legacy\old\passive_line.rs:38 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 5개**: `dist_sq`, `first`, `format_inner`, `from_iter`, `to_vec`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:8460) · **형제 9개** (EpicHuntAndPokePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::EpicHuntAndPokePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:9 | True | fn(&game_ai::plan_legacy::old::EpicHuntAndPokePlan) -> game_ai::plan_legacy::old::EpicHuntAndPokePlan |
| 1 | <game_ai::plan_legacy::old::EpicHuntAndPokePlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:9 | True | fn() -> game_ai::plan_legacy::old::EpicHuntAndPokePlan |
| 2 | <game_ai::plan_legacy::old::EpicHuntAndPokePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:9 | True | fn(&game_ai::plan_legacy::old::EpicHuntAndPokePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::old::EpicHuntAndPokePlan::goal | pub | game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:22 | True | fn(&game_ai::plan_legacy::old::EpicHuntAndPokePlan) -> game_core::BigGoal |
| 4 | game_ai::plan_legacy::old::EpicHuntAndPokePlan::update | pub | game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:26 | True | fn(&mut game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 5 | game_ai::plan_legacy::old::EpicHuntAndPokePlan::sub_plan | pub | game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:30 | False | fn(&mut game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |
| 6 | game_ai::plan_legacy::old::EpicHuntAndPokePlan::next_plan | pub | game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:98 | True | fn(&game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 7 | game_ai::plan_legacy::old::EpicHuntAndPokePlan::check_recall | in:game_ai::plan_legacy::old::epic::hunt_and_poke | game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:103 | False | fn(&mut game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) -> bool |
| 8 | game_ai::plan_legacy::old::EpicHuntAndPokePlan::is_end | pub | game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:163 | False | fn(&game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | :58 의 `hp_ratio<51 \|\| can_upgrade_item \|\| (is_in_heal_area && hp<max)` 와 `epic.hp==epic.max_hp` 의 줄 안 결합 순서 — column 정보 없음(표기 불가, 외연은 확정) | 4 |  |
| 1 | 미탐색 | v46_flee_gate_check(passive_line.rs:38) 반환 코드 0 이 '도주 발동' 인 것은 분기 방향으로 확정. 코드 1~5 의 의미는 그 함수 본문(m04.ll:55362~57000) 미탐색 | 4 |  |
| 2 | 미탐색 | MapDef.regions 값 7 이 어느 지역인지 — 맵 데이터(_gcbc MapDef 생성부) 미탐색. 에픽 둥지 리전으로 추정 | 5 |  |
| 3 | 미탐색 | TeamPlan.objective_target(team_plan.rs:231)·take_hunt_commit(:249) 은 인라인돼 태그 비교(0x41f==0, 0x420==3)만 남음 — 두 헬퍼가 추가로 무언가를 갱신하는지(take_* 이름) 는 별도 define 없음(fnparts 인라인 86개) 으로 판정 불가. IR 상 sub_plan 본문에서 team_plan 에 store 는 없음 | 4 |  |
| 4 | 미탐색 | Steal.last_vision_tick=0 은 Steal 생성 시 초기값. 실제 시야 갱신은 다른 함수(Steal 서브플랜 평가) 소관 — 미탐색 | 4 |  |
| 5 | 미탐색 | PlayerState::strategy() 가 rnd 를 받는 이유(랜덤 요소?) — game_core 본문 미탐색 | 4 |  |
| 6 | 미탐색 | exe 0xdefcd0 대조: 상수 0x38b8·0x6d70·64000·892000(891999+1)·896000·960000·40000000001(+0x508)·22500000001(+0xbbe) 전부 exe 본문에서 확인. call 은 vtable [+0x40]/[+0x1f0] 및 직접 call 11개로 IR 과 정합. [+0xe8]/[+0x108] 간접호출 2개는 IR 에 없는 것(is_recent_visible 이 exe 에서 인라인된 흔적으로 추정). 로직 어긋남 없음 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

