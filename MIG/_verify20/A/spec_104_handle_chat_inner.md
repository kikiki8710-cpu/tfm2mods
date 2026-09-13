---

### `104` handle_chat_inner — 받은 팀 채팅(Chat 57종)을 종류별로 처리해 team_plan.objective/plan/chats 를 갱신하고 오브젝트 오해 상태를 마킹

| 항목 | 값 |
|---|---|
| id | `chat__LegacyPlanHandler_handle_chat_inner` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler4chatNtB4_17LegacyPlanHandler17handle_chat_inner` |
| 소스 | `game-ai\src\plan_legacy\handler\chat.rs:41` |
| IR | `m13.ll` 29692~33371행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat_inner` · **in:game_ai** |
| 계층 | 플랜 핸들러 |
| exe | `e59b20` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r11` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData)
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut LegacyPlanHandler(6168B) | plan/team_plan/chats/계측 카운터를 쓴다 | 4 |
| 1 | 2 | version | usize | AI 버전. `>1` 게이트 3종: resolve_join_stake 호출(:212), r2_fight_protected(:330/:351/:372/:390/:436/:454/:495/:513/:530/:551/:568/:583), 그 외 분기 없음 | 4 |
| 2 | 3 | rnd | &mut StdRng(320B) | :83 gen_range(0..1000) 1회 직접 사용 + 하위 호출에 전달 | 4 |
| 3 | 4 | player | &PlayerState(2528B) |  | 4 |
| 4 | 5 | data | &OperationData(24B) | cache(+0)/context(+8)/blackboard(+0x10) | 4 |
| 5 | 6 | from | Position(i32 0..5) | 보낸 선수 포지션 | 4 |
| 6 | 7 | chat | Chat(24B, 값전달 dead_on_return) | +0 태그, +1 LineType/StopReason 등, +4 Position, +8/+0x10 usize 페이로드 | 4 |
| 7 | 8 | misunderstood | bool | :598 에서만 분기(오브젝트 오해 마킹 vs 해제) | 4 |
| 8 | 9 | debug | &mut DebugFrameData(224B) | 본문에서 직접 읽기/쓰기 0건 — 하위 호출(passive_plan·BattlePlan::update·has_lead·resolve_join_stake·tower_dive_is_viable·handle_line_defense)에 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// chat.rs:41~43 진입
objective_target_before = self.team_plan.objective_target()   // objective 태그 0(Morgard)→Some(JungleType::Morgard=4), 1(Serpen)→Some(Serpen=5), 그 외→None
chat_target = objective_chat_target(&chat)   // chat 태그 25,27,28,29(SerpenSetup/EnemyHunt/Assemble/Hunt)→Serpen(5) · 34,36,37,38(Morgard 동일4종)→Morgard(4) · 그 외 None

// chat.rs:45 match chat  (처리 후 대부분 %54 = 공통 꼬리(:595) 로 합류. Battle 채팅 처리와 DefenseLine 의 handle_line_defense==false 만 %700(즉시 return))

// ---- Battle(3)/BattleDive(4)/BattleHelp(6) {target_id, _} : chat.rs:139~284 ----
target = game.get_entity_by_id(chat+8); None → 꼬리
self.ff_call_recv += 1;  is_dive = (chat==BattleDive)
if is_ignored_battle_enemy(version, player, data, target, is_dive) { self.ff_call_ignored += 1; return }   // :143~144
if self.plan is Battle {                                                            // :148 tag 9
  me = cache.player_champion[team][position]; None → (아래 else 블록 :191 로 진입: %180 null → %299)
  self.ff_call_in_battle += 1                                                       // :151
  if plan.support_target.is_none() && distance_sq(target, me) < 22500000001 { plan.support_target = Some(target.id) }   // :152~153
  if is_dive {                                                                      // :158
    dist = target.distance(me); spd = max(me.move_speed,1); hp% = me.hp*100/max(me.max_hp,1)
    hp_th = 60 - aggressive_ratio*20/1000 ; far = 4 + roaming_ratio*2/1000
    if !(dist/spd > far*tps || hp% < hp_th || plan.dive_abandoned || plan.dive_quiet) {       // :166
      if tick > self.last_dive_abandon_tick + (tps*4+1) {                                     // :174 dive_rejoin_cd
        viable = tower_dive_is_viable(version, rnd, player, data, &team_plan, target, ?, debug)
        plan.with_dive |= viable                                                              // :177
        if viable { plan.dive_join_tick = tick; plan.dive_tower = 적 타워(iter_towers_without_nexus(1-team)) 중 target 과 최단거리 .and_then(Tower→info.ty) }   // :178~180
      }
    }
  }
  if plan.with_dive { plan.sub_goal = Trace{focus: target.id} }                     // :183~184
  return
} else {                                                                            // :191
  me = player_champion[team][position]; None → 꼬리
  dist = target.distance(me); spd = me.move_speed (0이면 div0 패닉); hp% = me.hp*100/me.max_hp (0이면 패닉)   // :193~196
  hp_th = 60 - agg*20/1000 ; far = 4 + roam*2/1000                                  // :198~202
  can_help = self.plan.can_help(player, data)
  // 계측(배타적, 첫 사유만): :204 !can_help→ff_call_no_help+1 / :205 dist/spd > far*tps→ff_call_too_far+1 / :206 hp%<hp_th→ff_call_low_hp+1
  stake = if version>1 { resolve_join_stake(...) } else { None }                     // :212~213
  stake_commit = stake.is_some_and(|s| s.line==Commit(0)) ; stake_ok = stake.map_or(true, |s| s.line!=Disengage(2))   // :215~216
  too_far = dist/spd > far*tps ; low_hp = hp% < hp_th                                // :217
  join = can_help && ( (too_far || low_hp) ? stake_commit : stake_ok )              // :218  (게이트 기각을 stake Commit 이 뒤집는 구조)
  if join {
    self.chats.push(Chat::Ok(0))                                                     // :219
    b = BattlePlan::new(version, Support(target.id), data, player); b.entry_src = 4; b.ff_stake_join = (too_far||low_hp) && stake_commit ? 1 : 0 ; b.support_target = Some(target.id)   // :220~223
    if is_dive { if tick > last_dive_abandon_tick + tps*4+1 { b.with_dive = tower_dive_is_viable(..); if b.with_dive { b.dive_join_tick = tick; b.dive_tower = (위와 동일 타워 탐색) } } else { b.with_dive = false } }   // :225~231
    b.update(version, rnd, player, data, &self.positioning_score, &self.team_plan, debug)   // :234
    if b.sub_goal != RunAway(4) { self.ff_call_join += 1; self.plan = BigPlan::Battle(b) }   // :235~237 (구 plan drop)
    else { self.ff_call_bail += 1; drop(b) }                                          // :239~241
  } else if dist/spd > tps*6 {                                                       // :245 6초 이상 먼 아군에게만 글로벌 궁 지원 검토
    if can_help || self.plan.is_passive() /* PassiveLine|SinglePlanLine|PassiveJungle(team==player_team) */ {   // :246
      if me.can_ult() && self.pending_global_ult_target.is_none() {                  // :247~248
        ult = me.ult_effect(level)  /* level>4 면 me.ult_effect 아니면 NONE(casting=-1) */ ; None → skip   // :250
        range = ult.range + ult.growth_range*(level-1) + me.stat_buff_cached.range   // :251
        if range > 199999 && ult.target ∈ {Ally0, AllyChampion1, AllyNotSelf3} && ult.casting == Targeting(0) {   // :252~254
          enemies_near_me = count(적 챔피언 e: blackboard[1-team].is_recent_visible(game, player, e) && distance_sq(e, me) < 22500000001)   // :257~260 closure#4
          if enemies_near_me == 0 {                                                  // :262
            ally = if chat==BattleHelp(6) { target.team == Player(my team) ? target : skip } else { player_champion[team][from] ?: skip }   // :264~274
            self.pending_global_ult_target = Some((ally.id, tick + tps*3))            // :278
          }
        }
      }
    }
  }
  → 꼬리
}

// ---- HideLine(11)/HideLineToo(12) {line, _} : chat.rs:46~108 ----
self.team_plan.allies[from] = Some(AllyRegionInfo{last_tick: tick, region: Top0/Mid2/Bottom4 ← line})   // :47~68
if self.plan is PassiveLine(p) {                                                     // :70 tag 3
  me = player_champion[team][position]; None → :98
  hp_th = 70 - roaming_ratio*20/1000                                                 // :74~75
  if line.is_adj_line(p.line) /* Top↔Mid, Mid↔(Top|Bottom), Bottom↔Mid */ && p.has_lead(version, rnd, player, data, debug) && me.hp*100/me.max_hp >= hp_th {   // :76
    (_,_,score) = evaluate_gank_opportunity_with_score(rnd, player, data, line, 0)   // :78
    go = score >= 0 || rnd.gen_range(0..1000) >= ego_ratio*700/1000   // :80~83 (점수 음수면 ego 비례 확률로만)
    if go { self.chats.push(HideLineToo{line,0}); mf_note_swap(21,tick); self.plan = LineGanker{chats:[], setup_limit:0, wait_limit: tick+tps*10, line, phase: Setup} }   // :88~91
  }
}
if self.team_plan.objective.is_none() {                                              // :98
  my_line = position Top→Top, Mid→Mid, Bottom→Bottom, else skip                     // :99~102
  if my_line == line { objective = Some(Gank(line)); team_plan.gank_start_tick = tick }   // :106~108
}

// ---- Cancel(17) : chat.rs:113~136 ----
self.team_plan.allies[from] = None
if position==Jungle && self.plan is LineGanker && (from 의 라인 == plan.line) { plan.phase = Cancel; self.gank_cancel_tick = tick }   // :115~126
if objective == ComebackPick { comeback_pick_outcomes.last_mut() 의 .1 이 0 이면 6 }   // :131~132
if objective ∈ {ComebackPick11, Gank8, Dive9} { mf_note_obj_clear(43,tick); objective = None }   // :134~136

// ---- Press(21)/PressChange(22) {line,_} : chat.rs:287~321 ----
if self.v3_epicops_armed → 무시                                                       // :290
if objective == Some(PressEpic(line)) → 무시                                          // :291
strategy = player.strategy(game); match strategy.morgard_use {                        // :292~295
  Gather: objective = PressEpic(line); if 내 챔피언 살아있음 { chats.push(Ok(0)) }     // :297~299
  Split14{position}: if position != my position { objective = PressEpic(line); if 살아있음 push(Ok) }   // :305~308
  Split131{p1,p2}: if my position ∉ {p1,p2} { objective = PressEpic(line); if 살아있음 push(Ok) }   // :313~316
}

// ---- Repair(23) : chat.rs:324~333 ----
if objective != Some(Repair) { objective = Some(Repair); if 살아있음 push(Ok); if !(version>1 && plan is Battle) { ff_note_battle_swap(17,tick); mf_note_swap(22,tick); self.plan = ActiveRecall } }

// ---- Mia(1){pos,_} :340 team_plan.vision.mia_call_ticks[pos] = tick
// ---- BattleStop(7) :338 team_plan.ally_battle_stop_tick[from] = Some(tick)
// ---- SerpenPrepare(24) :487 obj_spawn.serpen_spawn_call_tick = tick ; MorgardPrepare(33) :490 epic_spawn_call_tick = tick

// ---- Serpen 계열(objective 가 Serpen 이 아닐 때만 처리; 이미 Serpen 이면 무시) ----
SerpenSetup(25) :345 objective = Serpen{Setup,false}; :350 if plan.goal() != BigGoal::Serpen(3) && !(version>1 && plan is Battle) { ff_note_battle_swap(18); (p,_) = self.passive_plan(..); mf_note_swap(23); self.plan = p }; :359 if 살아있음 push(Ok)
SerpenEnemyHunt(27)/SerpenAssemble(28) :366 objective = Serpen{Assemble(2),false}; :371~376 동일 교체; :380 push(Ok) (생존 체크 없음)
SerpenHunt(29) :385 objective = Serpen{Hunt(3),false}; :389~394 동일 교체; :398 if 살아있음 push(Ok)
SerpenBattle(30) (objective 가 Serpen 일 때만) :406 objective.with_battle = true; :407 push(Ok)
SerpenGiveUp(31) (objective 가 Serpen 일 때만) :413 obj_spawn.serpen_giveup_tick = Some(tick); :414 mf_note_obj_clear(44); :415 objective = None; :417 if plan.goal()==Serpen { (p,_) = passive_plan(); mf_note_swap(23); plan = p } (r2 보호 없음); :423 if 살아있음 push(Ok)
// ---- Morgard 계열(objective 가 Morgard 가 아닐 때만) ----
MorgardSetup(34)/EnemyHunt(36)/Assemble(37) :430 objective = Morgard{Setup(1),false} (3종 모두 Setup); :435 if plan.goal()!=Epic(2) && !(v>1&&Battle) { ff_note_battle_swap(18)[인라인]; passive_plan; mf_note_swap(23); plan=p }; :444 push(Ok) (생존 체크 없음)
MorgardHunt(38) :449 objective = Morgard{Hunt(3),false}; :453~458 동일 교체; :461 push(Ok)
MorgardBattle(39) (Morgard 일 때만) :467 with_battle = true; :468 push(Ok)
MorgardGiveUp(40) (Morgard 일 때만) :474 epic_giveup_tick = Some(tick); :475 mf_note_obj_clear(45); :476 objective=None; :478 if goal()==Epic { passive_plan; mf_note_swap(23); plan=p }; :483 push(Ok)

// ---- AttackNexus(42){line,_} (objective != Nexus 일 때) :494 objective = Nexus(line); :495 if !(v>1&&Battle) { ff_note_battle_swap(19); mf_note_swap(23); plan = ForcePassive }; :500 if 살아있음 push(Ok)
// ---- DefenseNexus(43) (objective != Defense 일 때) :507 n = calculate_nexus_defense_count(player,data); :509 if i_am_chosen_defender(player,data,n,&[]) { :512 objective=Defense; :513 if !(v>1&&Battle){ ff_note_battle_swap(20); mf_note_swap(23); plan=ForcePassive }; :518 if 살아있음 push(Ok) }
// ---- DefenseLine(14){line,_} :525 if objective ∈ {Defense2, DefenseLine3} → 무시; else :526 if handle_line_defense(version,rnd,player,data,line,debug) { :529 objective = DefenseLine(line); :530 if !(v>1&&Battle){ ff_note_battle_swap(20); mf_note_swap(23); plan=ForcePassive } } else { return (꼬리 생략) }
// ---- GankDive(44){line,_} :542 objective = Dive(line); :543 if 살아있음 push(Ok)
// ---- PressTower(45){line,_} :549 objective = PressTower(line); :550 team_plan.press_tower_start_tick = tick; :551 if !(v>1&&Battle){ ff_note_battle_swap(19)[인라인]; mf_note_swap(23); plan=ForcePassive }; :556 if 살아있음 push(Ok)
// ---- ComebackPick(46){line,_} :562 if objective ∈ {None, Gank8} { :563 objective = ComebackPick{line, ready:false}; :564 comeback_pick_start_tick = tick }
//   :567 match position { Jungle: :568 if !(v>1&&Battle){ ff_note_battle_swap(21); mf_note_swap(21); plan = LineGanker{chats:[],setup_limit:0, wait_limit: tick+tps*15, line, phase:Setup} }
//                         Top: line∈{Top,Mid} 이면 계속 / Mid: 항상 / Bottom(3)·Support(4): line!=Top 이면 계속 → :583 if !(v>1&&Battle){ ff_note_battle_swap(21); mf_note_swap(23); plan=ForcePassive } }

// ---- 공통 꼬리 chat.rs:595~607 (%54 경유 분기만) ----
if let Some(t) = chat_target {                                                       // :595
  after = self.team_plan.objective_target()                                          // :596
  if after == Some(t) {                                                              // :597 (objective 태그<2 && 4/5 일치)
    if misunderstood { if objective_target_before != Some(t) { team_plan.mark_objective_misunderstanding(t, tick) /* t∈{4,5} 면 objective_misunderstanding = Some{None,None,accepted_tick:tick,target:t} */ } }   // :598~600
    else { team_plan.clear_objective_misunderstanding() /* = None */ }              // :603
  }
}
return
```

**`mem` 메모리 접근 120건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LegacyPlanHandler | 0x517 | team_plan.objective | r | MainObjective 태그(-1=None). 함수 전체의 1차 분기 키 | 4 | OK |  |
| 1 | LegacyPlanHandler | 0x518 | team_plan.objective@Some.0.line/phase | r | 이 함수는 쓰기 위주. 읽기는 MainObjective::eq(:291,:324) 인자로 3B 통째 전달 + 인라인 ff_note_battle_swap 은 안 읽음 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 2 | LegacyPlanHandler | 0x5e8 | plan | r | BigPlan 메모리태그(니치, 2..17). 9=Battle, 10=LineGanker, 3=PassiveLine 판별에 사용 | 4 | OK |  |
| 3 | LegacyPlanHandler | 0x5f0 | plan@Battle.0.support_target@tag | r | :152 is_none 판정 | 4 | OK |  |
| 4 | LegacyPlanHandler | 0x638 | plan@PassiveJungle.0.team | r | is_passive 인라인(:246) | 4 | OK |  |
| 5 | LegacyPlanHandler | 0x640 | plan@PassiveJungle.0.player_team | r | is_passive 인라인(:246) — team==player_team 일 때만 passive | 4 | OK |  |
| 6 | LegacyPlanHandler | 0x618 | plan@LineGanker.0.line | r | :124 Cancel 처리 | 4 | OK |  |
| 7 | LegacyPlanHandler | 0x6e6 | plan@Battle.0.with_dive | r | :177/:183 | 4 | OK |  |
| 8 | LegacyPlanHandler | 0x6e8 | plan@Battle.0.dive_quiet | r | :166 | 4 | OK |  |
| 9 | LegacyPlanHandler | 0x6ea | plan@Battle.0.dive_abandoned | r | :166 | 4 | OK |  |
| 10 | LegacyPlanHandler | 0x706 | plan@PassiveLine.0.line | r | :76 is_adj_line(내 라인, 채팅 라인) | 4 | OK |  |
| 11 | LegacyPlanHandler | 0x6f3 | plan@Battle.0.dive_abort_src | r | ff_note_battle_swap 인라인(:437,:552) 읽기 → ff_battle_exit_latch[7] | 4 | OK |  |
| 12 | LegacyPlanHandler | 0x6f7 | plan@Battle.0.entry_src | r | ff_note_battle_swap 인라인 → ff_battle_exit.1 | 4 | OK |  |
| 13 | LegacyPlanHandler | 0x6f9 | plan@Battle.0.ff_wave_obs_open | r | ff_note_battle_swap 인라인 → ff_battle_exit.4 | 4 | OK |  |
| 14 | LegacyPlanHandler | 0x6fa | plan@Battle.0.ff_wave_open_hp | r | → ff_battle_exit_latch[1] | 4 | OK |  |
| 15 | LegacyPlanHandler | 0x6fb | plan@Battle.0.ff_wave_open_pct | r | → latch[2] | 4 | OK |  |
| 16 | LegacyPlanHandler | 0x6fc | plan@Battle.0.ff_wave_fire_hp | r | → latch[3] | 4 | OK |  |
| 17 | LegacyPlanHandler | 0x6fd | plan@Battle.0.ff_wave_fire_pct | r | → latch[4] | 4 | OK |  |
| 18 | LegacyPlanHandler | 0x6fe | plan@Battle.0.ff_wave_open_danger | r | → latch[5] | 4 | OK |  |
| 19 | LegacyPlanHandler | 0x6ff | plan@Battle.0.ff_wave_fire_danger | r | → latch[6] | 4 | OK |  |
| 20 | LegacyPlanHandler | 0x700 | plan@Battle.0.ff_exit1_cls | r | → latch[0] | 4 | OK |  |
| 21 | LegacyPlanHandler | 0x701 | plan@Battle.0.exit_sub | r | → ff_battle_exit.3 | 4 | OK |  |
| 22 | LegacyPlanHandler | 0x530 | pending_global_ult_target@tag | r | :248 is_none | 4 | OK |  |
| 23 | LegacyPlanHandler | 0x1480 | last_dive_abandon_tick | r | :174/:226 dive_rejoin_cd 비교 | 4 | OK |  |
| 24 | LegacyPlanHandler | 0x180a | v3_epicops_armed | r | :290 Press/PressChange 무시 게이트 | 4 | OK |  |
| 25 | LegacyPlanHandler | 0x1d8 | team_plan.comeback_pick_outcomes.buf.ptr | r | :132 last_mut | 4 | OK |  |
| 26 | LegacyPlanHandler | 0x1e0 | team_plan.comeback_pick_outcomes.len | r | :132 | 4 | OK |  |
| 27 | LegacyPlanHandler | 0x7c8 | chats.buf.cap | r | push 인라인(19곳) | 4 | OK |  |
| 28 | LegacyPlanHandler | 0x7d0 | chats.buf.ptr | r | push 인라인 | 4 | OK |  |
| 29 | LegacyPlanHandler | 0x7d8 | chats.len | r | push 인라인 | 4 | OK |  |
| 30 | LegacyPlanHandler | 0x15b8 | ff_call_recv | r | +=1 (읽고 씀) | 4 | OK |  |
| 31 | LegacyPlanHandler | 0x15c0 | ff_call_ignored | r | +=1 | 4 | OK |  |
| 32 | LegacyPlanHandler | 0x15c8 | ff_call_in_battle | r | +=1 | 4 | OK |  |
| 33 | LegacyPlanHandler | 0x15d0 | ff_call_no_help | r | +=1 (phi 5584/5592/5600 로 접혀 gep 는 `%0 + phi`) | 4 | OK |  |
| 34 | LegacyPlanHandler | 0x15d8 | ff_call_too_far | r | +=1 (phi) | 4 | OK |  |
| 35 | LegacyPlanHandler | 0x15e0 | ff_call_low_hp | r | +=1 (phi) | 4 | OK |  |
| 36 | LegacyPlanHandler | 0x15e8 | ff_call_bail | r | +=1 | 4 | OK |  |
| 37 | LegacyPlanHandler | 0x15f0 | ff_call_join | r | +=1 | 4 | OK |  |
| 38 | PlayerState | 0x930 | info.team | r | player_champion 인덱스(bounds<2 패닉가드) | 4 | OK |  |
| 39 | PlayerState | 0x9c0 | info.position@tag | r | Position(i32) — as_index / 라인 환산 | 4 | OK |  |
| 40 | PlayerState | 0x180 | info.parameter | r | &AthleteParameter(744B) → aggressive_ratio/roaming_ratio/ego_ratio 호출 인자 | 4 | OK |  |
| 41 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 42 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 43 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — closure#4 캡처 | 4 | OK |  |
| 44 | AbstractGameWithCache | 0x0 | game.data_ptr | r | dyn AbstractGame 팻포인터 데이터 | 4 | OK |  |
| 45 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x28=tick, +0x1f0=get_entity_by_id | 4 | OK |  |
| 46 | AbstractGameWithCache | 0x1e0 | player_champion | r | +0x1e0 + team*40 + pos*8 (Option<&Entity>, null=None) | 4 | OK |  |
| 47 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 48 | GameSetting | 0x12f8 | tick_per_second | r | tps — 모든 초 단위 임계의 배수 | 4 | OK |  |
| 49 | Chat | 0x0 | tag | r | 57 variant 스위치 | 4 | OK |  |
| 50 | Chat | 0x1 | line/reason(1B 페이로드) | r | HideLine·DefenseLine·Press·GankDive·PressTower·ComebackPick·AttackNexus 의 LineType | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 51 | Chat | 0x4 | Mia.0 Position | r | :340 | 4 | OK |  |
| 52 | Chat | 0x8 | Battle*.0 target id | r | :139 get_entity_by_id 인자. Battle.1(+0x10)은 안 읽음 | 4 | OK |  |
| 53 | Entity | 0x5c0 | id | r | support_target / pending_global_ult_target 값 | 4 | OK |  |
| 54 | Entity | 0x5c8 | level | r | ult_effect 인라인: level>4 면 자기 ult_effect, 아니면 정적 NONE(casting=-1) | 4 | OK |  |
| 55 | Entity | 0x538 | ult_effect | r | Option<Effect>(56B). +0x10 range, +0x18 growth_range, +0x28 target(CastingTarget), +0x30 casting(CastingType, -1=None) | 4 | OK |  |
| 56 | Entity | 0x438 | stat_buff_cached.range | r | :251 궁 사거리 가산 | 4 | OK |  |
| 57 | Entity | 0x628 | stat_cached.hp | r | HP% 분모(max(.,1) 또는 div0 패닉가드) | 4 | OK |  |
| 58 | Entity | 0x640 | stat_cached.move_speed | r | dist/spd 분모 | 4 | OK |  |
| 59 | Entity | 0x660 | x | r | distance_sq 인라인(:152, closure#4) | 4 | OK |  |
| 60 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 61 | Entity | 0x670 | hp | r | HP% | 4 | OK |  |
| 62 | Entity | 0x0 | team@tag | r | :266 target.team == Player(team) (tag 0) | 4 | OK |  |
| 63 | Entity | 0x8 | team@Player.0 | r | :266 | 4 | OK |  |
| 64 | Entity | 0x68 | ty@tag | r | 타워 and_then: ==2(Tower) 일 때만 | 4 | OK |  |
| 65 | Entity | 0x128 | ty@Tower.info.ty | r | TowerType → plan.dive_tower | 4 | OK |  |
| 66 | FightPrediction | 0x38 | line | r | resolve_join_stake 결과(Option, tag -1=None at +0). 0=Commit, 2=Disengage | 4 | OK |  |
| 67 | Strategy | 0x4 | morgard_use | r | 5=Gather, 6=Split14, 그 외=Split131(position1 이 +4 에 겹침) | 4 | OK |  |
| 68 | Strategy | 0x8 | morgard_use@Split14.position / @Split131.position2 | r |  | 4 | OK |  |
| 69 | BattlePlan(local b) | 0x58 | sub_goal@tag | r | :235 != RunAway(4) 판정 | 4 | OK |  |
| 70 | LegacyPlanHandler | 0x517 | team_plan.objective | w | ★핵심 부작용 | 4 | OK | 8 Gank(:107) / -1 None(:136,:415,:476) / 5 PressEpic(:297,:306,:314) / 7 Repair(:325) / 1 Serpen(:345,:366,:385) / 0 Morgard(:430,:449) / 4 Nexus(:494) / 2 Defense(:512) / 3 DefenseLine(:529) / 9 Dive(:542) / 10 PressTower(:549) / 11 ComebackPick(:563) |
| 71 | LegacyPlanHandler | 0x518 | team_plan.objective@Some.0 페이로드1(line 또는 phase) | w |  | 4 | OK | chat.line 또는 phase(Setup=1/Assemble=2/Hunt=3) |
| 72 | LegacyPlanHandler | 0x519 | team_plan.objective@Some.0 페이로드2(with_battle / ComebackPick.ready) | w |  | 4 | OK | 0(:345,:366,:385,:430,:449,:563) / 1(:406 SerpenBattle, :467 MorgardBattle) |
| 73 | LegacyPlanHandler | 0x5e8 | plan | w | ★교체 전 항상 drop_glue<BigPlan>(&self.plan) — Vec/Box 드롭 지점(HEAP_SUBST) | 4 | OK | 9 Battle(:237) / 8 ActiveRecall(:333) / 2 ForcePassive(:498,:516,:533,:554,:586) / 10 LineGanker(:90,:571) / passive_plan 결과 384B memcpy(:355,:376,:394,:420,:440,:458,:481) |
| 74 | LegacyPlanHandler | 0x5f0 | plan@Battle.0 (280B memcpy) | w | :237 store tag 9 + memcpy 280 | 4 | OK | BattlePlan b |
| 75 | LegacyPlanHandler | 0x5f0 | plan@Battle.0.support_target@tag | w | :153 | 4 | OK | 1(Some) |
| 76 | LegacyPlanHandler | 0x5f8 | plan@Battle.0.support_target@Some.0 | w | :153 | 4 | OK | target.id |
| 77 | LegacyPlanHandler | 0x5f0 | plan@LineGanker.0.chats (cap=0, ptr=8, len=0) | w | :90/:571 — 0x5f0 cap, 0x5f8 ptr(dangling 8), 0x600 len(memset) | 4 | OK | 빈 Vec |
| 78 | LegacyPlanHandler | 0x608 | plan@LineGanker.0.setup_limit | w | :90/:571 memset 16B(0x600..0x60f) | 4 | OK | 0 |
| 79 | LegacyPlanHandler | 0x610 | plan@LineGanker.0.wait_limit | w |  | 4 | OK | tick + tps*10(:90) / tick + tps*15(:571) |
| 80 | LegacyPlanHandler | 0x618 | plan@LineGanker.0.line | w | :90/:571 | 4 | OK | chat.line |
| 81 | LegacyPlanHandler | 0x619 | plan@LineGanker.0.phase | w | LineGankerPhase 니치 태그(WaitResponse=6,Setup=7,Cancel=8) | 4 | OK | 7 Setup(:90,:571) / 8 Cancel(:125) |
| 82 | LegacyPlanHandler | 0x648 | plan@Battle.0.sub_goal@tag | w | :184 | 4 | OK | 0 Trace |
| 83 | LegacyPlanHandler | 0x650 | plan@Battle.0.sub_goal@Trace.focus | w | :184 | 4 | OK | target.id |
| 84 | LegacyPlanHandler | 0x6a8 | plan@Battle.0.dive_join_tick | w | :178 | 4 | OK | tick |
| 85 | LegacyPlanHandler | 0x6e6 | plan@Battle.0.with_dive | w | :177 | 4 | OK | with_dive \| tower_dive_is_viable(..) |
| 86 | LegacyPlanHandler | 0x6ee | plan@Battle.0.dive_tower | w | :180 | 4 | OK | Option<TowerType>(-1=None) |
| 87 | LegacyPlanHandler | 0x378 | team_plan.vision.mia_call_ticks[pos] | w | :341 Mia — 0x378 + pos*8 | 4 | OK | tick |
| 88 | LegacyPlanHandler | 0xf8 | team_plan.ally_battle_stop_tick[from] | w | :338 BattleStop | 4 | OK | Some(tick) (tag 1 @0xf8+from*16, tick @0x100+from*16) |
| 89 | LegacyPlanHandler | 0x2b8 | team_plan.allies[from] | w |  | 4 | OK | Some{last_tick: tick @0x2b8+from*16, region @0x2c0+from*16 = Top0/Mid2/Bottom4}(:47 HideLine) / None(-1 @0x2c0+from*16, :113 Cancel) |
| 90 | LegacyPlanHandler | 0x170 | team_plan.obj_spawn.serpen_spawn_call_tick | w | :487 SerpenPrepare | 4 | OK | tick |
| 91 | LegacyPlanHandler | 0x168 | team_plan.obj_spawn.epic_spawn_call_tick | w | :490 MorgardPrepare | 4 | OK | tick |
| 92 | LegacyPlanHandler | 0x148 | team_plan.obj_spawn.epic_giveup_tick | w | :474 MorgardGiveUp | 4 | OK | Some(tick) (tag 1 @0x148, tick @0x150) |
| 93 | LegacyPlanHandler | 0x158 | team_plan.obj_spawn.serpen_giveup_tick | w | :413 SerpenGiveUp | 4 | OK | Some(tick) (tag 1 @0x158, tick @0x160) |
| 94 | LegacyPlanHandler | 0x188 | team_plan.objective_misunderstanding | w | mark 는 target∈{Morgard4,Serpen5} 일 때만((t&6)==4) | 4 | OK | Some{opposite_group_start_tick: None(0 @0x188), solo_hunt_start_tick: None(0 @0x198), accepted_tick: tick @0x1a8, target: JungleType @0x1b0}(:600 mark) / None(-1 @0x188, :603 clear) |
| 95 | LegacyPlanHandler | 0x308 | team_plan.mf_obj_clear.0 | w | mf_note_obj_clear 인라인 | 4 | OK | 43(:135 Cancel) / 44(:414 SerpenGiveUp) / 45(:475 MorgardGiveUp) |
| 96 | LegacyPlanHandler | 0x310 | team_plan.mf_obj_clear.1 | w |  | 4 | OK | tick |
| 97 | LegacyPlanHandler | 0x448 | team_plan.gank_start_tick | w | :108 | 4 | OK | tick |
| 98 | LegacyPlanHandler | 0x450 | team_plan.press_tower_start_tick | w | :550 | 4 | OK | tick |
| 99 | LegacyPlanHandler | 0x458 | team_plan.comeback_pick_start_tick | w | :564 | 4 | OK | tick |
| 100 | LegacyPlanHandler | 0x1d8 | team_plan.comeback_pick_outcomes[last].1 (힙 버퍼 경유) | w | :132 — Vec 버퍼 요소 (i32,u8) 의 u8(+4). self 오프셋이 아니라 ptr 경유 쓰기 | 4 | OK | 6 (기존값 0 일 때만) |
| 101 | LegacyPlanHandler | 0x530 | pending_global_ult_target | w | :278 | 4 | OK | Some((ally.id @0x538, tick + tps*3 @0x540)) (tag 1 @0x530) |
| 102 | LegacyPlanHandler | 0x7d8 | chats.len (+ 버퍼에 Chat 24B 기록) | w | cap==len 이면 RawVec<Chat>::grow_one 호출 = 힙 재할당 지점(HEAP_SUBST) | 4 | OK | push(Chat::Ok(0)) 18곳 / push(Chat::HideLineToo{line,0}) 1곳(:88) |
| 103 | LegacyPlanHandler | 0x1470 | gank_cancel_tick | w | :126 | 4 | OK | tick |
| 104 | LegacyPlanHandler | 0x15b8 | ff_call_recv | w | :141 | 4 | OK | +1 |
| 105 | LegacyPlanHandler | 0x15c0 | ff_call_ignored | w | :144 | 4 | OK | +1 |
| 106 | LegacyPlanHandler | 0x15c8 | ff_call_in_battle | w | :151 | 4 | OK | +1 |
| 107 | LegacyPlanHandler | 0x15d0 | ff_call_no_help | w | :204 (phi 5584) | 4 | OK | +1 |
| 108 | LegacyPlanHandler | 0x15d8 | ff_call_too_far | w | :205 (phi 5592) | 4 | OK | +1 |
| 109 | LegacyPlanHandler | 0x15e0 | ff_call_low_hp | w | :206 (phi 5600) | 4 | OK | +1 |
| 110 | LegacyPlanHandler | 0x15e8 | ff_call_bail | w | :239 | 4 | OK | +1 |
| 111 | LegacyPlanHandler | 0x15f0 | ff_call_join | w | :236 | 4 | OK | +1 |
| 112 | LegacyPlanHandler | 0x15f8 | ff_battle_exit.2 (usize tick) | w | ff_note_battle_swap 인라인 본문(:437 src=18, :552 src=19) — plan 이 Battle(9) 일 때만 | 4 | OK | tick |
| 113 | LegacyPlanHandler | 0x1600 | ff_battle_exit.0 (src) | w |  | 4 | OK | 18(:437) / 19(:552) |
| 114 | LegacyPlanHandler | 0x1601 | ff_battle_exit.1 | w |  | 4 | OK | plan.Battle.entry_src |
| 115 | LegacyPlanHandler | 0x1602 | ff_battle_exit.3 | w |  | 4 | OK | plan.Battle.exit_sub |
| 116 | LegacyPlanHandler | 0x1603 | ff_battle_exit.4 | w |  | 4 | OK | plan.Battle.ff_wave_obs_open |
| 117 | LegacyPlanHandler | 0x1608 | ff_battle_exit_latch[0..8] | w | ff_note_battle_swap 인라인 | 4 | OK | [ff_exit1_cls, ff_wave_open_hp, ff_wave_open_pct, ff_wave_fire_hp, ff_wave_fire_pct, ff_wave_open_danger, ff_wave_fire_danger, dive_abort_src] (0x1608..0x160f) |
| 118 | LegacyPlanHandler | 0x1610 | mf_swap.0 | w | mf_note_swap 인라인(항상 인라인, 15곳) | 4 | OK | 21(:89 HideLine, :570 ComebackPick 정글) / 22(:332 Repair) / 23(그 외 12곳) |
| 119 | LegacyPlanHandler | 0x1618 | mf_swap.1 | w |  | 4 | OK | tick |

**`consts` 상수 24건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 22500000001 | 152 | 임계 | 150000^2 + 1 — distance_sq(target, me) < 이 값 ⟺ 거리 ≤150000 이면 support_target 채택. aux closure#4(:258) 에서도 같은 값으로 '내 근처 150000 안 적 챔피언' 카운트 | 4 |  |
| 1 | 199999 | 252 | 임계 | 궁 사거리(ult.range + growth*(level-1) + stat_buff_cached.range) > 199999 ⟺ ≥200000 이면 글로벌 궁으로 간주 | 4 |  |
| 2 | 60 | 163 | 임계 | hp_th = 60 - aggressive_ratio*20/1000 : 합류 HP% 하한(공격성이 높을수록 낮아짐). :199 에도 동일 | 4 |  |
| 3 | 20 | 163 | 계수 | aggressive_ratio 계수(×20/1000). :75 에도 roaming_ratio*20/1000 으로 사용 | 4 |  |
| 4 | 1000 | 163 | 계수 | ratio 분모(ratio 는 0..1000 스케일). :83 gen_range 상한·ego*700/1000 에도 | 4 |  |
| 5 | 4 | 165 | 임계 | far_sec = 4 + roaming_ratio*2/1000 : 합류 허용 이동시간(초). :202 도 동일 | 4 |  |
| 6 | 1 | 165 | 임계 | roaming_ratio*2 가 `shl i64 %x, 1` 로 접힘(:165, :202) | 4 | 2 |
| 7 | 2 | 174 | 임계 | dive_rejoin_cd = tps*4 + 1 — `shl i64 %tps, 2 \| 1` 로 접힘(:174, :226). tick > last_dive_abandon_tick + cd 일 때만 다이브 재합류 판정 | 4 | 4 |
| 8 | 6 | 245 | 태그 | dist/spd > tps*6 (6초 이상 걸리는 먼 아군) 일 때만 글로벌 궁 지원 경로(:246~:278) 진입 | 4 |  |
| 9 | 3 | 278 | 태그 | pending_global_ult_target 유효기한 = tick + tps*3 | 4 |  |
| 10 | 70 | 75 | 임계 | HideLine 응답 HP% 하한 = 70 - roaming_ratio*20/1000 (sdiv -1000 형태) | 4 |  |
| 11 | 700 | 83 | 계수 | 갱크 점수<0 일 때 그래도 참가할 확률 = ego_ratio*700/1000 (‰) vs gen_range(0..1000) | 4 |  |
| 12 | 10 | 91 | 태그 | HideLine 응답으로 만든 LineGanker.wait_limit = tick + tps*10 | 4 |  |
| 13 | 15 | 572 | 태그 | ComebackPick 정글러 LineGanker.wait_limit = tick + tps*15 (Chat::Ok 태그 15 와 동일 값이 본문에 다수 있음) | 4 |  |
| 14 | 100 | 161 | 계수 | HP% = hp*100/max_hp | 4 |  |
| 15 | 5 | 297 | 센티널 | MainObjective::PressEpic 태그(store i8 5) 및 morgard_use Gather 니치 태그(5). :291 Some(PressEpic(line)) 비교용 3B 임시도 5 | 4 |  |
| 16 | 9 | 237 | 태그 | BigPlan::Battle 메모리태그(idx7+2) — self.plan 교체 | 4 |  |
| 17 | 8 | 333 | 태그 | BigPlan::ActiveRecall 태그(:333) / MainObjective::Gank 태그(:107,:134,:562) / LineGankerPhase::Cancel 태그(:125) | 4 |  |
| 18 | 12 | 88 | 태그 | Chat::HideLineToo 태그 — chats.push | 4 |  |
| 19 | 43 | 135 | 태그 | mf_obj_clear 코드(Cancel 로 objective 해제) | 4 |  |
| 20 | 44 | 414 | 태그 | mf_obj_clear 코드(SerpenGiveUp) | 4 |  |
| 21 | 45 | 475 | 태그 | mf_obj_clear 코드(MorgardGiveUp) | 4 |  |
| 22 | 17 | 331 | 태그 | ff_note_battle_swap src 코드(Repair→ActiveRecall). 18=오브젝트 채팅→passive_plan(:352,:373,:391,:437,:455), 19=AttackNexus/PressTower→ForcePassive(:496,:552), 20=DefenseLine/DefenseNexus(:514,:531), 21=ComebackPick(:569,:584) | 4 |  |
| 23 | 23 | 332 | 태그 | mf_note_swap 코드 — 이 함수의 플랜 교체 대부분(23). HideLine·ComebackPick 정글 은 21(:89,:570), Repair 는 22(:332) | 4 |  |

**`knobs` 조정점 12건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 합류 거리 임계(초) | chat.rs:165 / :202 | 4 + roaming_ratio*2/1000 | 올리면 더 먼 아군 전투 호출에도 합류(BattlePlan 생성) | 4 | 기존 |
| 1 | 합류 HP% 하한 | chat.rs:163 / :199 | 60 - aggressive_ratio*20/1000 | 내리면 저체력에서도 합류. 단 stake Commit 이면 게이트 무시 | 4 | 기존 |
| 2 | support_target 채택 거리 | chat.rs:152 | 150000^2+1 (22500000001) | 올리면 더 먼 호출 대상도 지원 표적으로 잡음 | 4 | 기존 |
| 3 | 다이브 재합류 쿨다운 | chat.rs:174 / :226 | tps*4+1 | 올리면 다이브 포기 후 재다이브가 늦어짐 | 4 | 기존 |
| 4 | 글로벌 궁 지원 최소 거리(초) | chat.rs:245 | tps*6 | 내리면 더 가까운 아군에게도 글로벌 궁 예약 | 4 | 기존 |
| 5 | 글로벌 궁 사거리 기준 | chat.rs:252 | >199999 | 내리면 사거리가 짧은 궁도 글로벌 궁으로 예약 | 4 | 기존 |
| 6 | 글로벌 궁 예약 유효기간 | chat.rs:278 | tps*3 | 올리면 pending_global_ult_target 이 오래 유지 | 4 | 기존 |
| 7 | HideLine 응답 HP% 하한 | chat.rs:75 | 70 - roaming_ratio*20/1000 | 내리면 저체력 라이너도 갱크 협조 | 4 | 기존 |
| 8 | 갱크 점수 음수시 참가 확률 | chat.rs:83 | ego_ratio*700/1000 ‰ | 올리면 불리한 갱크에도 더 자주 협조 | 4 | 기존 |
| 9 | HideLine→LineGanker 대기 한도 | chat.rs:91 | tps*10 | 올리면 갱크 대기 시간 증가 | 4 | 기존 |
| 10 | ComebackPick 정글 대기 한도 | chat.rs:572 | tps*15 | 동상 | 4 | 기존 |
| 11 | r2_fight_protected 게이트 | chat.rs:330 등 12곳 | version>1 && plan is Battle | 끄면 전투 중에도 오브젝트/넥서스 채팅이 플랜을 즉시 교체 | 4 | 기존 |

<details><summary>`callees` 피호출자 41건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | aggressive_ratio | game_core::AthleteParameter::aggressive_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:432 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | calculate_nexus_defense_count | game_ai::plan_legacy::handler::calculate_nexus_defense_count | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> usize | game-ai\src\plan_legacy\handler.rs:2328 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | can_help | game_ai::plan_legacy::types::BigPlan::can_help | pub | fn(&game_ai::plan_legacy::types::BigPlan, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\types.rs:263 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | clear_objective_misunderstanding | game_ai::plan_legacy::team_plan::TeamPlan::clear_objective_misunderstanding | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan) | game-ai\src\plan_legacy\team_plan.rs:277 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | ego_ratio | game_core::AthleteParameter::ego_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:439 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | evaluate_gank_opportunity_with_score | game_ai::plan_legacy::old::evaluate_gank_opportunity_with_score | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, i32) -> (bool, i32, i32) | game-ai\src\plan_legacy\old\passive_jungle.rs:693 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | ff_note_battle_swap | game_ai::plan_legacy::handler::LegacyPlanHandler::ff_note_battle_swap | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) | game-ai\src\plan_legacy\handler.rs:400 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | goal | game_ai::plan_legacy::types::BigPlan::goal | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> game_core::BigGoal | game-ai\src\plan_legacy\types.rs:132 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | handle_line_defense | game_ai::plan_legacy::old::handle_line_defense | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:543 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | has_lead | game_ai::plan_legacy::old::PassiveLinePlan::has_lead | pub | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_line.rs:1048 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | i_am_chosen_defender | game_ai::plan_legacy::old::i_am_chosen_defender | pub | fn(&game_core::PlayerState, &game_core::OperationData, usize, &[usize]) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:297 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | is_adj_line | game_core::LineType::is_adj_line | pub | fn(&game_core::LineType, game_core::LineType) -> bool | game-core\src\simulation\state\player.rs:1005 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | is_ignored_battle_enemy | game_ai::plan_legacy::old::is_ignored_battle_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:887 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | is_passive | game_ai::plan_legacy::types::BigPlan::is_passive | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> bool | game-ai\src\plan_legacy\types.rs:253 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 22 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | mark_objective_misunderstanding | game_ai::plan_legacy::team_plan::TeamPlan::mark_objective_misunderstanding | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType, usize) | game-ai\src\plan_legacy\team_plan.rs:265 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 25 | max_hp | game_core::PlayerAiContext::<'a, 'b, 'r>::max_hp | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> std::option::Option<usize> | game-core\src\mod_ai.rs:608 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 26 | mf_note_obj_clear | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) | game-ai\src\plan_legacy\team_plan.rs:196 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 27 | mf_note_swap | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) | game-ai\src\plan_legacy\handler.rs:409 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 28 | new | game_ai::plan_legacy::old::BattlePlan::new | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan | game-ai\src\plan_legacy\old\battle.rs:196 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | objective_chat_target | game_ai::plan_legacy::handler::chat::objective_chat_target | in:game_ai | fn(game_core::Chat) -> std::option::Option<game_core::JungleType> | game-ai\src\plan_legacy\handler\chat.rs:612 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 30 | objective_target | game_ai::plan_legacy::team_plan::TeamPlan::objective_target | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> std::option::Option<game_core::JungleType> | game-ai\src\plan_legacy\team_plan.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 31 | passive_plan | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) | game-ai\src\plan_legacy\handler.rs:1855 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 33 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 34 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 35 | resolve_join_stake | game_ai::plan_legacy::old::fight_model::resolve_join_stake | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::FightPrediction> | game-ai\src\plan_legacy\old\fight_model.rs:680 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | roaming_ratio | game_core::AthleteParameter::roaming_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:425 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | tower_dive_is_viable | game_ai::plan_legacy::old::tower_dive_is_viable | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::Entity, bool, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:935 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 40 | update | game_ai::plan_legacy::old::BattlePlan::update | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\battle.rs:427 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 6개**: `chat`, `gen_range`, `grow_one`, `last_mut`, `map_or`, `move_speed`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m13.ll:29492, m13.ll:29526) · **형제 41개** (LegacyPlanHandler)

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
| 0 | 미탐색 | BattlePlan::new 에 넘기는 BattlePlanGoal 이 Support(target.id)(tag 1) 인 것은 확인. TryKill 이 아닌 이유는 소스 부재 | 4 |  |
| 1 | 미탐색 | Entity::ult_effect 의 'level>4' 판정과 정적 NONE(casting=-1) — Option<Effect> 의 니치가 casting(i32) 에 있는지 tcx 로 미확인(IR 관측만) | 3 |  |
| 2 | 미탐색 | closure#4 에서 blackboard 인덱스가 `1 - my_team` 인 이유(적 팀 블랙보드로 is_recent_visible 판정) — 의미는 game_core 소관, 미확인 | 4 |  |
| 3 | 미탐색 | :2113 등 passive_plan 반환 u8(mf 소스 코드)을 이 함수는 버린다 — 의도(소스 코드 23 고정) 는 추정 | 5 |  |
| 4 | 미탐색 | exe 0xe59b20 과의 대조는 하지 않았다(Ghidra 미사용). IR 3,679줄 ↔ 9,416B 는 크기상 모순 없음 정도만 | 4 |  |
| 5 | 미탐색 | C3 경고 0x608(setup_limit)은 memset(0x600,16B) 로 접혀 gep 가 없다 — 관측 사실 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

