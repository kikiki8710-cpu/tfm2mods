---

### `102` BattlePlan::update — Battle 플랜의 매판 갱신 — 교전 종료(sub_goal=End)/도주(RunAway) 전환 조건 사슬을 순서대로 검사하고 통과하면 update_v32 로 위임

| 항목 | 값 |
|---|---|
| id | `battle__BattlePlan_update` |
| 심볼 | `_RNvMs0_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battleNtB5_10BattlePlan6update` |
| 소스 | `game-ai\src\plan_legacy\old\battle.rs:427` |
| IR | `m10.ll` 23429~27459행 |
| 경로·가시성 | `game_ai::plan_legacy::old::BattlePlan::update` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `dfb840` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r11` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData)
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut BattlePlan(280B) | noalias, readonly 아님 — writes 참조. 반환 없음(void) | 4 |
| 1 | 2 | version | usize | alloca %28 에 저장 후 재로드. 분기: `>1`(v2 경로: watchdog·fight_kit_range·anchor·L549) · `<2`(v1: v27 discipline·L591). 여러 callee 에 `i64 poison` 으로 전달(v25_scoped_battle_objective·is_unreasonable_tower_dive_enemy) = 그 callee 는 version 을 안 읽음 | 4 |
| 2 | 3 | rnd | &mut StdRng(320B) | noalias·align16·readonly 아님. 직접 소비 0 — check_kill_die_tick·should_end_object_finish_kill_priority_battle·update_v32 에 전달만 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | readonly. team(+0x930)·position(+0x9c0)·info.id(+0x928)·parameter(+0x180) | 4 |
| 4 | 5 | data | &OperationData(24B) | readonly. cache/context/blackboard | 4 |
| 5 | 6 | positioning_score | &PositioningScoreData(2760B) | readonly. 본문 직접 읽기 0 — L707 position_risk_all_zero_near 에 전달만 | 4 |
| 6 | 7 | team_plan | &TeamPlan | `ptr nonnull align 8`(dereferenceable 없음=크기 미표기). objective(+0x41f i24) 읽기 · v27_active_objective_discipline·update_v32 전달 | 4 |
| 7 | 8 | debug | &mut DebugFrameData(224B) | noalias. L527(ctx.debug 일 때만) infos(+0xa0 HashMap) entry push · check_kill_die_tick·update_v32 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// battle.rs:430~431 — 준비 + main_objective 갱신 (무조건)
champ = cache.player_champion[player.team][player.position].unwrap();
self.main_objective = fight_model::v25_scoped_battle_objective(<version 미사용>, team_plan.objective(i24), player, data, self.current_focus());   // 431 ★write 0xff
//   current_focus() = sub_goal.focus()[Trace/Protect/Kiting/KitingBack/Assassin/AssassinReady→Some(+0x60); RunAway/End→None] .or(support_target) .or(main_goal TryKill/Support 의 +0x48)

// 439~442 — v2 교전 정지 워치독 (no_enemy_idle_watchdog battle.rs:406~425 인라인)
if version > 1 && self.sub_goal != End {
  tick = game.tick();
  enemy_near = iter_champions(enemy_team).any(|e| dist_sq(e, champ) <= 250000²);
  moved = utils::distance(champ.pos, self.idle_anchor) > 8000;
  active = matches!(champ.ty, Champion(ci)) && !matches!(ci.action_state, Idle|Move);   // 413~414
  if active || enemy_near || moved || self.idle_since == 0 { self.idle_since = tick; self.idle_anchor = champ.pos; }   // 418~420 ★write
  else if tick.saturating_sub(self.idle_since) >= max(input_delay_max(player.parameter)/100*2, 2) {
    self.exit_src = 2; self.sub_goal = End; return;   // 441~442
  }
}

// 452~469 — v54 재진입 계측 (prev_eval)
(any_in_range, dir_now, oor) = match self.sub_goal.engage_dir()[battle.rs:49~50] {
  Trace(0) => (true, +1, 0),
  KitingBack(3)|RunAway(4) => {   // 453~462
    found = iter_champions(enemy_team).any(|c| blackboard[enemy_team].is_recent_visible(game, player, c)
        && !is_ignored_battle_enemy(c)[= is_ignored_well_enemy(c.team==Player(enemy_team) && is_enemy_well_danger(version, player, c.pos)) || (!(version>1 && self.with_dive) && is_unreasonable_tower_dive_enemy(player, data, c, self.with_dive))]
        && { r = if version>1 { max(fight_kit_range_cached(data,champ,c), fight_kit_range_cached(data,c,champ)) } else { max(max_range_cached(data,champ,c), max_range_cached(data,c,champ)) }; dist_sq(c, champ) <= (r+30000)² });
    (false, -1, if found {0} else {1})
  },
  _ => (false, 0, 0)
};
if let Some((prev_oor, prev_dir)) = self.v54_prev_eval { if prev_dir == -1 && prev_oor && any_in_range(=Trace) { self.v54_reentry_ticks.push(game.tick()); } }   // 464~466 ★write(힙 grow_one)
self.v54_prev_eval = Some((oor!=0, dir_now));   // 469 ★write

// 473~491 — 적 우물
if is_enemy_well_danger(version, player, champ.x, champ.y) {   // 473~474
  self.with_dive=false; self.dive_tower=None; self.dive_abandoned=true; self.dive_abort_src=3; self.exit_src=6; self.sub_goal=RunAway;
  self.well_runaway = Some(game.tick() + tps*5); return;   // 475~482
}
if let Some(wr) = self.well_runaway { if game.tick() < wr { self.exit_src=6; self.sub_goal=RunAway; return; } else { self.well_runaway = None; } }   // 486~491

// 496~505 — 다이브 통과 abort
if self.with_dive {
  near_towers = cache.iter_towers_without_nexus(enemy_team).filter(|t| dist_sq(t, champ) <= 120000²);   // 497 (aux m06/m11)
  nexus = cache.nexus[enemy_team].unwrap();   // 499
  if near_towers.any(|t| t.distance(nexus) > champ.distance(nexus) + 60000) { self.with_dive=false; self.dive_abandoned=true; self.dive_abort_src=4; }   // 502~505
}

// 509~540 — 도움 콜 (플랜 나이 ≥30틱 && !help_called)
if !(game.tick().saturating_sub(self.start_tick) < 30 || self.help_called) {
  near_enemies: bumpalo Vec<&Entity> = iter_champions(enemy_team).filter(|c| blackboard[enemy_team].is_recent_visible(game,player,c) && !is_ignored_well_enemy(version,player,c) && dist_sq(c,champ) <= (max_range_cached(data,c,champ)+30000)²).collect_in(ctx.pool);   // 511~519 (aux 55740/37075)
  near_wide_allies = iter_champions(team).filter(|c| c.id != champ.id && dist_sq(c,champ) <= 108100000000).count();   // 521~522
  if ctx.debug { near_allies = iter_champions(team).filter(dist_sq <= 120000²).count(); debug.infos.entry(champ.id).or_insert(vec![]).push(format!(..near_enemies.len, near_wide_allies, near_allies..)); }   // 524~528 (행동 무영향)
  me_die_tick = fight_check::check_kill_die_tick(version, rnd, data, player, champ, &near_enemies.clone(), &Vec::new_in(pool), debug);   // 531~532
  if near_wide_allies != 0 && !(me_die_tick > tps || self.help_called || self.sub_goal == RunAway) {   // 534~536
    self.help_called = true; self.chats.push(Chat::BattleHelp(champ.id, 0));   // 537~538 ★write(힙 grow_one)
  }
  drop(near_enemies);   // 540
}

// 549~556 — 앵커 (v2: focus 대상 위치)
(anchor_x, anchor_y, anchor_margin) = (champ.x, champ.y, 0);
if version > 1 { if let Some(t) = self.current_focus().and_then(|id| game.get_entity_by_id(id)) { anchor_margin = if game.tick() == self.start_tick { t.move_speed * tps } else { 0 }; anchor = t.pos; } }   // 550~556

// 562~579 — v1 오브젝트 규율 (version < 2 에서만 v27 호출)
target = match self.main_objective { Some(Morgard)=>Some(JungleType::Morgard(4)), Some(Serpen)=>Some(Serpen(5)), _=>None };   // 562
if version < 2 && target.is_some() {
  if let Some(state) = team_plan.v27_active_objective_discipline(version, data, target) {   // 571~572 (sret+25 == 2 → None)
    has_near_threat = iter_champions(enemy_team).any(|e| e.is_visible_from(champ) && !is_ignored_well_enemy(version,player,e) && blackboard[enemy_team].is_recent_visible(game,player,e) && dist_sq(e,champ) <= 160000²);   // 573~576
    self.exit_src=4; self.exit_sub=1; self.sub_goal = if state.kind != SafeWait || has_near_threat { RunAway } else { End }; return;   // 577~579
  }
}

// 589~643 — main_objective 별 종료
if let Some(mo) = self.main_objective {
  if version < 2 && fight_model::should_end_object_finish_kill_priority_battle(version, rnd, data?, ..., mo) { self.exit_src=4; self.exit_sub=2; self.sub_goal=End; return; }   // 591~594 (★version≥2 는 이 검사 없이 598 로)
  match mo {   // 598
    Morgard{phase,..} if phase == 3 => { camp = ctx.map.camp_pos(Morgard(4), team==0); eff = 200000.sat_sub(anchor_margin);   // 602~606
        if dist_sq(anchor, camp) >= eff² && !map_region::is_enemy_side(ctx, anchor, team==0)[is_blue_side: anchor_x - anchor_y + height > width, xor team==0] { self.exit_src=4; self.exit_sub=3; self.sub_goal=End; return; } }   // 607~610
    Serpen{phase,..} if phase == 3 => { 동일(camp Serpen(5), eff 200000) → exit_sub=4 }   // 618~626
    Defense => { nexus = cache.nexus[team].unwrap(); self.with_dive=false; self.dive_abandoned=true; self.dive_abort_src=5;   // 632~638 ★write(무조건)
        eff = 300000.sat_sub(anchor_margin); if dist_sq(anchor, nexus) >= eff² { self.exit_src=4; self.exit_sub=5; self.sub_goal=End; return; } }   // 639~643
    _ => {}
  }
}

// 675~680 — region 이탈
if let Some(region) = self.region { if dist_sq(champ.pos, region.center) >= region.range² { self.exit_src=5; self.sub_goal=End; return; } }

// 686~701 — battle_recency
(in_battle, bt) = game_core::simulation::battle_recency(game, player.id);   // 686
kiting_like = matches!(self.sub_goal, KitingBack|RunAway);   // 690
recent_start = self.start_tick >= game.tick().saturating_sub(120);
if in_battle { if bt < game.tick().sat_sub(120) && !recent_start && !kiting_like && !matches!(self.sub_goal, Kiting|Trace) { self.exit_src=3; self.sub_goal=End; return; } }   // 691~695
else if !recent_start && !kiting_like && !matches!(self.sub_goal, Kiting|Trace) { self.exit_src=3; self.sub_goal=End; return; }   // 698~701

// 705~737 — KitingBack/RunAway 종료 판정
if kiting_like {
  risk_all_zero = position_risk_all_zero_near(version, player, data, positioning_score, PositionEvalPurpose::RunAway);   // 707
  has_near_visible_enemies = iter_champions(enemy_team).any(|c| dist_sq(c,champ) <= 150000² && !is_ignored_well_enemy(version,player,c) && blackboard[enemy_team].is_recent_visible(game,player,c));   // 708~709
  has_runaway_threat = iter_champions(enemy_team).any(|c| fight_model::v22_visible_enemy_is_runaway_threat(version, player, data, champ, c));   // 712
  if self.sub_goal == RunAway {   // 716
    if !has_runaway_threat { self.sub_goal = End; return; }   // 720
    safe = iter_champions(enemy_team).enumerate().any(|(p,c)| c.is_some_and(|c| v22_visible_enemy_is_runaway_threat(version,player,data,champ,c) && blackboard[enemy_team].in_battle(p)[big_goal[p].1 == Some(Battle{focus: Some})]));   // 725~730 (aux 5330)
  } else {   // KitingBack
    safe = if risk_all_zero { <End 조건이 !recent_start 만으로 결정> } else { has_near_visible_enemies };   // 733 (IR: %1567/%1572 분기)
  }
  if !safe && !recent_start { self.sub_goal = End; return; }   // 736~737  (risk_all_zero 경로: !recent_start 이면 End)
}
self.update_v32(version, rnd, player, data, team_plan, debug);   // 745 (위 어느 return 에도 안 걸린 경우)

```

**`mem` 메모리 접근 65건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | team | r | L430 (bounds<2) · enemy_team=1-team 전역 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | position | r | L430 as_index | 4 | OK |  |
| 2 | PlayerState | 0x928 | info.id | r | L686 battle_recency(game, player.id) | 4 | OK |  |
| 3 | PlayerState | 0x180 | info.parameter | r | L423(watchdog) AthleteParameter::input_delay_max(&parameter) | 4 | OK |  |
| 4 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 5 | OperationData | 0x8 | context | r | &GameContext — setting·map·debug·pool | 4 | OK |  |
| 6 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — 전부 [enemy_team] 인덱스(L453/513/573/708/729) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr | r |  | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | +0x28 tick(L440/466/482/487/509/552/692/698/736) · +0x1f0 get_entity_by_id(L550) | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion / [enemy_team][0..5] | r | L430 champ unwrap · iter_champions(enemy_team) 5칸 언롤(L440/453/511/573/708/712/726) · iter_champions(team)(L521/526) | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x170 | nexus[team] / nexus[team] | r | L499 적 넥서스 unwrap(gep 368+team*8) · L632 내 넥서스 unwrap | 4 | OK |  |
| 11 | GameContext | 0x0 | pool | r | L519/531/532 bumpalo Vec 할당 bump | 4 | OK |  |
| 12 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 13 | GameContext | 0x20 | map | r | &MapDef(28112B) — L603/619 camp_pos | 4 | OK |  |
| 14 | GameContext | 0x3b | debug | r | L524 true 이면 near_allies 계수 + debug.infos 문자열 push (행동 무영향) | 4 | OK |  |
| 15 | GameSetting | 0x12f8 | tick_per_second | r | gep 4856 — L482 tps*5 · L535 me_die_tick > tps · L553 anchor_margin = move_speed*tps | 4 | OK |  |
| 16 | GameSetting | 0x12c0 | height | r | gep 4800 — is_blue_side 인라인(map_region.rs:7~8): x - y + height > width | 4 | OK |  |
| 17 | GameSetting | 0x12b8 | width | r | gep 4792 | 4 | OK |  |
| 18 | TeamPlan | 0x41f | objective | r | i24(Option<MainObjective> 3B, gep 1055) — L431 v25_scoped_battle_objective 인자 | 4 | OK |  |
| 19 | BattlePlan | 0x0 | support_target | r | current_focus() 인라인(battle.rs:359~361): sub_goal.focus().or(support_target).or(main_goal.focus()) | 4 | OK |  |
| 20 | BattlePlan | 0x10 | region@tag | r | L675 Option<BattleRegion> (center_x +0x18, center_y +0x20, range +0x28) | 4 | OK |  |
| 21 | BattlePlan | 0x30 | well_runaway@tag | r | L486 Option<usize> (값 +0x38) | 4 | OK |  |
| 22 | BattlePlan | 0x40 | main_goal@tag | r | current_focus: TryKill(0)/Support(1) 이면 +0x48 focus | 4 | OK |  |
| 23 | BattlePlan | 0x58 | sub_goal@tag | r | 매 분기 게이트: 0 Trace/1 Protect/2 Kiting/3 KitingBack/4 RunAway/5 Assassin/6 AssassinReady/7 End (focus 페이로드 +0x60) | 4 | OK |  |
| 24 | BattlePlan | 0x68 | chats (cap/ptr +0x70/len +0x78) | r | L538 push | 4 | OK |  |
| 25 | BattlePlan | 0x80 | v54_reentry_ticks (cap/ptr +0x88/len +0x90) | r | L466 push | 4 | OK |  |
| 26 | BattlePlan | 0xc0 | start_tick | r | L509 tick-start<30 · L552 tick==start · L692/698/736 start >= tick-120 | 4 | OK |  |
| 27 | BattlePlan | 0xd0 | idle_since | r | watchdog L418/423 | 4 | OK |  |
| 28 | BattlePlan | 0xd8 | idle_anchor | r | watchdog L410 distance(champ, anchor) > 8000 | 4 | OK |  |
| 29 | BattlePlan | 0xf4 | v54_prev_eval | r | L464 Option<(bool oor, i8 dir)>: byte0 2=None | 4 | OK |  |
| 30 | BattlePlan | 0xf6 | with_dive | r | L453 closure(with_declared_dive) · L496 게이트 | 4 | OK |  |
| 31 | BattlePlan | 0xf9 | help_called | r | L509/535 | 4 | OK |  |
| 32 | BattlePlan | 0xff | main_objective@tag | r | L431 에서 갱신된 값을 L562/589/598 에서 다시 읽음(3B: +0xff tag, +0x100 phase, +0x101) | 4 | OK |  |
| 33 | Entity | 0x0 | team@tag | r | is_ignored_well_enemy 인라인(fight_model.rs:755): c.team == Player(enemy_team) | 4 | OK |  |
| 34 | Entity | 0x8 | team@Player.0 | r |  | 4 | OK |  |
| 35 | Entity | 0x38 | visible_state | r | L573 closure$7 is_visible_from(champ)(entity.rs:1482~1483): champ.team Player(t) 이면 enemy.visible_state[t]==Visible(0), Neutral 이면 true | 4 | OK |  |
| 36 | Entity | 0x68 | ty@tag | r | watchdog L413: 13=Champion 일 때만 action_state 검사 | 4 | OK |  |
| 37 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | watchdog L414: Idle(0)/Move(2) 만 '정지 후보', 그 외는 anchor 리셋 | 4 | OK |  |
| 38 | Entity | 0x5c0 | id | r | L521 c.id != champ.id · L538 Chat 페이로드 | 4 | OK |  |
| 39 | Entity | 0x640 | stat_cached.move_speed | r | L553 anchor_margin = t.move_speed * tps (tick==start_tick 일 때) | 4 | OK |  |
| 40 | Entity | 0x660 | x | r | champ·적·앵커 좌표 | 4 | OK |  |
| 41 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 42 | Blackboard | 0xf8 | big_goal[p].1@tag | r | aux 5330: in_battle(p)(blackboard.rs:168) = big_goal[p].1 == Some(Battle(5)) && focus.is_some(+0x100) | 4 | OK |  |
| 43 | Blackboard | 0x100 | big_goal[p].1@Some.0@Battle.focus@tag | r |  | 4 | OK |  |
| 44 | DebugFrameData | 0xa0 | infos | r | L527 HashMap<usize, Vec<String>> entry(champ.id) push(format!) — ctx.debug 일 때만 | 4 | OK |  |
| 45 | ObjectiveDisciplineState | 0x19 | kind | r | L572 sret+25: 2=None 니치 / 0 SafeWait / 1 HardDisengage (L579) | 4 | OK |  |
| 46 | BattlePlan | 0xff | main_objective@tag | w | L431 — 무조건(champ unwrap 직후). 이 함수의 첫 부작용 | 4 | OK | v25_scoped_battle_objective(...) (i24) |
| 47 | BattlePlan | 0xd0 | idle_since | w | L419(watchdog, version>1 && sub_goal!=End) — enemy_near\|\|moved\|\|idle_since==0 이거나 챔프 action_state∉{Idle,Move} 일 때 리셋 | 4 | OK | game.tick() |
| 48 | BattlePlan | 0xd8 | idle_anchor.0 | w | L420 동상 | 4 | OK | champ.x |
| 49 | BattlePlan | 0xe0 | idle_anchor.1 | w | L420 동상 | 4 | OK | champ.y |
| 50 | BattlePlan | 0x108 | exit_src | w | 2=L441 watchdog 정지 / 6=L480·490 적 우물 / 4=L577·592·608·624·641 오브젝트 규율·종료 / 5=L679 region 이탈 / 3=L694·700 battle_recency 만료 | 4 | OK | 2 \| 6 \| 4 \| 5 \| 3 |
| 51 | BattlePlan | 0x58 | sub_goal@tag | w | End: L442·579(조건부)·594·610·626·643·680·695·701·720·737 / RunAway: L481·491·579(조건부). 페이로드는 안 씀 | 4 | OK | 7(End) \| 4(RunAway) |
| 52 | BattlePlan | 0xf4 | v54_prev_eval.0 (oor) | w | L469 — 매판(watchdog 조기 return 제외) Some 으로 덮어씀 | 4 | OK | 1 iff KitingBack/RunAway 이고 사거리 내 적 0 else 0 |
| 53 | BattlePlan | 0xf5 | v54_prev_eval.1 (dir) | w | L469 | 4 | OK | 1(Trace) \| -1(KitingBack/RunAway) \| 0 |
| 54 | BattlePlan | 0x90 | v54_reentry_ticks.len (+push) | w | L466 — prev==(true,-1) && 지금 Trace 일 때. ★힙: RawVec<usize>::grow_one 재할당 가능(cap==len) | 4 | OK | len+1, buf[len]=game.tick() |
| 55 | BattlePlan | 0xf6 | with_dive | w | L475(적 우물) · L503(다이브 통과 abort) · L636(Defense) | 4 | OK | 0 |
| 56 | BattlePlan | 0xfe | dive_tower | w | L476 | 4 | OK | -1(None) |
| 57 | BattlePlan | 0xfa | dive_abandoned | w | L478 · L504 · L637 | 4 | OK | 1 |
| 58 | BattlePlan | 0x103 | dive_abort_src | w | 3=L479 적 우물 / 4=L505 타워 통과 / 5=L638 Defense 오브젝트 | 4 | OK | 3 \| 4 \| 5 |
| 59 | BattlePlan | 0x30 | well_runaway@tag | w |  | 4 | OK | 1(Some) L482 \| 0(None) L488 |
| 60 | BattlePlan | 0x38 | well_runaway@Some.0 | w | L482 — 5초 우물 도주 | 4 | OK | game.tick() + tps*5 |
| 61 | BattlePlan | 0xf9 | help_called | w | L537 — near_wide_allies>0 && me_die_tick<=tps && !help_called && sub_goal!=RunAway | 4 | OK | 1 |
| 62 | BattlePlan | 0x78 | chats.len (+push) | w | L538 ★힙: RawVec<Chat>::grow_one 재할당 가능 | 4 | OK | len+1, buf[len]=Chat::BattleHelp(tag 6){champ.id, 0} |
| 63 | BattlePlan | 0x111 | exit_sub | w | exit_src=4 일 때만: 1=L578 규율 / 2=L593 finish_kill_priority / 3=L609 Morgard / 4=L625 Serpen / 5=L642 Defense | 4 | OK | 1\|2\|3\|4\|5 |
| 64 | DebugFrameData | 0xa0 | infos[champ.id].push(String) | w | L527 ctx.debug 일 때만 — 힙 String/Vec | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | format!(near_enemies.len, near_wide_allies, near_allies) |

**`consts` 상수 23건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 430 | 센티널 | team bounds · L439/549 `version>1`(ugt 1) · L570 `version<2` · watchdog L423 max(.,2) · L441 exit_src=2 · MainObjective 2=Defense · action_state 2=Move · Option None 니치(v54_prev_eval byte0=2·discipline kind=2) | 4 |  |
| 1 | 7 | 439 | 태그 | BattleSubPlanGoal End 태그 — L439 게이트(sub_goal!=End) 및 모든 종료 store | 4 |  |
| 2 | 4 | 452 | 태그 | sub_goal RunAway 태그 · exit_src/exit_sub 4 · dive_abort_src 4 · JungleType Morgard=4 · TowerType/그 외 | 4 |  |
| 3 | 3 | 452 | 센티널 | sub_goal KitingBack 태그(3..4 = `tag-3 <2` 접힘) · exit_sub 3 · dive_abort_src 3 · exit_src 3 · MainObjective phase==3 · purpose RunAway 니치 3 | 4 |  |
| 4 | 62500000001 | 409 | 임계 | 250000²+1 — watchdog enemy_near: 적 챔프 dist_sq ≤ 250000² | 4 |  |
| 5 | 8000 | 410 | 임계 | watchdog moved: distance(champ, idle_anchor) > 8000 | 4 |  |
| 6 | 13 | 413 | 태그 | EntityType Champion 태그 | 4 |  |
| 7 | 100 | 423 | 계수 | watchdog 정지 창 = max(input_delay_max/100 * 2, 2) 틱 (`shl 1` = ×2) | 4 |  |
| 8 | 1 | 423 | 태그 | `shl nuw nsw i64 %222, 1` — input_delay_max/100 의 ×2(입력지연 2회분) 접힘 · 또한 enemy_team=1-team · Some 태그 1 · dir=+1 | 4 | 2 |
| 9 | 30000 | 462 | 계수 | 교전 페어 사거리 여유: dist_sq(c,champ) ≤ (engage_pair_range+30000)² (L462) · near_enemies 필터 (max_range_cached+30000)² (aux 55844, L518) | 4 |  |
| 10 | -1 | 465 | 센티널 | dir=-1(KitingBack/RunAway) · v54_prev_eval.1==-1 비교 · dive_tower None 니치 · main_objective None 태그(i8 -1) · Chain 종료 마커 | 4 |  |
| 11 | 5 | 482 | 태그 | well_runaway = tick + tps*5 (5초) · MainObjective Serpen→JungleType 5 · BigGoal Battle 5 · player_champion 5칸 · dive_abort_src/exit_sub 5 | 4 |  |
| 12 | 6 | 480 | 태그 | exit_src 6(적 우물 도주) · Chat::BattleHelp 태그 6 | 4 |  |
| 13 | 30 | 509 | 임계 | 플랜 나이 30틱(0.5초) 미만이거나 help_called 이면 L511~540(도움 콜 판정) 생략 | 4 |  |
| 14 | 14400000001 | 497 | 임계 | 120000²+1 — near_towers(적 타워 dist≤120000², aux m06/m11) · L526 near_allies(디버그 계수) | 4 |  |
| 15 | 60000 | 502 | 미상 | aux: 어떤 near_tower 가 t.distance(enemy_nexus) > champ.distance(enemy_nexus)+60000 이면(타워보다 60k 더 깊이 들어감) 다이브 abort(src 4) | 4 |  |
| 16 | 108100000001 | 521 | 임계 | near_wide_allies: 아군(id≠champ) dist_sq ≤ 108100000000 (=1.081e11, 정수 제곱 아님 — 유래 unknown) | 4 |  |
| 17 | 0 | 534 | 태그 | near_wide_allies==0 이면 도움 콜 생략 · idle_since==0 · anchor_margin 0 | 4 |  |
| 18 | 25600000001 | 576 | 임계 | 160000²+1 — 오브젝트 규율 has_near_threat: 가시·최근가시·비우물 적 챔프 dist_sq ≤ 160000² | 4 |  |
| 19 | 200000 | 606 | 계수 | Morgard/Serpen phase3: eff = 200000.saturating_sub(anchor_margin); dist_sq(anchor, camp) ≥ eff² 이면 종료 후보 | 4 |  |
| 20 | 300000 | 639 | 계수 | Defense: eff = 300000.saturating_sub(anchor_margin); dist_sq(anchor, my_nexus) ≥ eff² 이면 End(exit_sub 5) | 4 |  |
| 21 | 120 | 692 | 계수 | 2초(60tps): battle_recency tick < now-120 · start_tick ≥ now-120 (플랜 시작 2초 이내 면제) · L736 | 4 |  |
| 22 | 22500000001 | 708 | 임계 | 150000²+1 — has_near_visible_enemies: dist_sq ≤ 150000² | 4 |  |

**`knobs` 조정점 15건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 워치독 적 탐지 거리 | battle.rs:409 (62500000001) | 250000 | 내리면 더 가까운 적이 있어야 '교전 중'으로 봐서 정지 판정(exit_src 2)이 잦아진다 | 4 | 기존 |
| 1 | 워치독 이동 판정 거리 | battle.rs:410 | 8000 | 올리면 잔이동을 정지로 봐서 End 가 잦아진다 | 4 | 기존 |
| 2 | 워치독 정지 창 배수/하한 | battle.rs:423 | 2 | input_delay_max/100*2 틱(하한 2). 올리면 정지를 더 오래 허용 | 4 | 기존 |
| 3 | 교전 페어 사거리 여유 | battle.rs:462·518 | 30000 | 올리면 더 먼 적도 '사거리 내'로 봐서 oor=0·near_enemies 증가 | 4 | 기존 |
| 4 | 우물 도주 지속 | battle.rs:482 | 5 | tps*5 = 5초. 올리면 적 우물 탈출 후 RunAway 유지 시간 증가 | 4 | 기존 |
| 5 | 다이브 타워 탐색 거리 | battle.rs:497 | 120000 | 올리면 더 먼 타워까지 통과 판정 대상 | 4 | 기존 |
| 6 | 다이브 통과 깊이 | battle.rs:502 | 60000 | 타워보다 이만큼 넥서스 쪽으로 더 들어가면 다이브 abort(src 4). 올리면 abort 가 늦어진다 | 4 | 기존 |
| 7 | 도움 콜 유예(플랜 나이) | battle.rs:509 | 30 | 틱. 올리면 교전 시작 후 도움 콜이 늦어진다 | 4 | 기존 |
| 8 | 도움 콜 아군 탐색 거리² | battle.rs:521 | 108100000001 | 올리면 더 먼 아군도 near_wide_allies 로 세어 콜 발화 조건 충족이 쉬워진다 | 4 | 기존 |
| 9 | 도움 콜 사망 예측 상한 | battle.rs:535 (tps) | 1 | me_die_tick > tps(1초) 이면 콜 안 함. tps 배수를 올리면 더 여유 있어도 콜 | 4 | 기존 |
| 10 | 오브젝트 규율 위협 거리 | battle.rs:576 | 160000 | 올리면 규율 종료 시 RunAway 가 늘고 End 가 준다 | 4 | 기존 |
| 11 | Morgard/Serpen 캠프 유지 반경 | battle.rs:606·622 | 200000 | 내리면 캠프에서 조금만 멀어져도(적 진영 아닐 때) 교전 End(exit_sub 3/4) | 4 | 기존 |
| 12 | Defense 넥서스 유지 반경 | battle.rs:639 | 300000 | 내리면 넥서스에서 조금만 멀어져도 End(exit_sub 5) | 4 | 기존 |
| 13 | 교전 최근성/시작 유예 | battle.rs:692·698·736 | 120 | 2초. 올리면 교전 없는 상태를 더 오래 유지(End 지연) | 4 | 기존 |
| 14 | KitingBack 근접 가시 적 거리 | battle.rs:708 | 150000 | 올리면 KitingBack 유지가 쉬워진다(End 감소) | 4 | 기존 |

<details><summary>`callees` 피호출자 38건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | battle_recency | game_core::battle_recency | pub | fn(&dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , usize) -> (std::option::Option<usize>, std::option::Option<usize>) | game-core\src\simulation.rs:937 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | current_focus | game_ai::plan_legacy::old::BattlePlan::current_focus | pub | fn(&game_ai::plan_legacy::old::BattlePlan) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\battle.rs:358 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | current_focus | game_ai::plan_legacy::old::SinglePlanBattle::current_focus | in:game_ai::plan_legacy::old::single_battle | fn(&game_ai::plan_legacy::old::SinglePlanBattle) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\single_battle.rs:94 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | current_focus | game_ai::plan_legacy::old::DeathMatchBattle::current_focus | in:game_ai::plan_legacy::old::death_battle | fn(&game_ai::plan_legacy::old::DeathMatchBattle) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\death_battle.rs:876 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | engage_dir | game_ai::plan_legacy::old::BattleSubPlanGoal::engage_dir | pub | fn(&game_ai::plan_legacy::old::BattleSubPlanGoal) -> i8 | game-ai\src\plan_legacy\old\battle.rs:48 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | fight_kit_range_cached | game_ai::plan_legacy::old::fight_kit_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2364 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | focus | game_ai::plan_legacy::old::BattleSubPlanGoal::focus | pub | fn(&game_ai::plan_legacy::old::BattleSubPlanGoal) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\battle.rs:29 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | in_battle | game_core::Blackboard::in_battle | pub | fn(&game_core::Blackboard, usize) -> bool | game-core\src\simulation\game\blackboard.rs:167 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | input_delay_max | game_core::AthleteParameter::input_delay_max | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:239 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | is_enemy_side | game_core::is_enemy_side | pub | fn(&game_core::GameContext, usize, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:57 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | is_ignored_battle_enemy | game_ai::plan_legacy::old::is_ignored_battle_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:887 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | is_unreasonable_tower_dive_enemy | game_ai::plan_legacy::old::is_unreasonable_tower_dive_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:792 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 23 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 24 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | position_risk_all_zero_near | game_ai::position_risk_all_zero_near | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, game_ai::PositionEvalPurpose) -> bool | game-ai\src\position_eval.rs:1268 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 28 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 29 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 30 | should_end_object_finish_kill_priority_battle | game_ai::plan_legacy::old::fight_model::should_end_object_finish_kill_priority_battle | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_ai::plan_legacy::team_plan::MainObjective) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1214 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 32 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 33 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 34 | update_v32 | game_ai::plan_legacy::old::BattlePlan::update_v32 | in:game_ai::plan_legacy::old::battle | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\battle.rs:748 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | v22_visible_enemy_is_runaway_threat | game_ai::plan_legacy::old::fight_model::v22_visible_enemy_is_runaway_threat | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1168 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | v25_scoped_battle_objective | game_ai::plan_legacy::old::fight_model::v25_scoped_battle_objective | in:game_ai | fn(usize, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>, &game_core::PlayerState, &game_core::OperationData, std::option::Option<usize>) -> std::option::Option<game_ai::plan_legacy::team_plan::MainObjective> | game-ai\src\plan_legacy\old\fight_model.rs:1138 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | v27_active_objective_discipline | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_active_objective_discipline | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectiveDisciplineState> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:483 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 14개**: `any_in_range`, `dist_sq`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `entry`, `enumerate`, `format_inner`, `grow_one`, `insert_no_grow`, `objective`, `or_insert`, `rustc_entry`, `sat_sub`, `try_fold`, `write`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 13곳** (m02.ll:6978, m04.ll:30019, m08.ll:94459, m13.ll:22761, m13.ll:30694, m13.ll:33865, m13.ll:34366, m13.ll:37661, m13.ll:37809, m13.ll:38754, m13.ll:38889, m13.ll:43209, m13.ll:45022) · **형제 20개** (BattlePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::BattlePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\battle.rs:96 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_ai::plan_legacy::old::BattlePlan |
| 1 | <game_ai::plan_legacy::old::BattlePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\battle.rs:96 | True | fn(&game_ai::plan_legacy::old::BattlePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::BattlePlan::sub_goal | pub | game-ai\src\plan_legacy\old\battle.rs:178 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_ai::plan_legacy::old::BattleSubPlanGoal |
| 3 | game_ai::plan_legacy::old::BattlePlan::ff_birth_src | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:184 | False | fn(&game_ai::plan_legacy::old::BattleSubPlanGoal) -> u8 |
| 4 | game_ai::plan_legacy::old::BattlePlan::in_active_fight | pub | game-ai\src\plan_legacy\old\battle.rs:190 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> bool |
| 5 | game_ai::plan_legacy::old::BattlePlan::new | pub | game-ai\src\plan_legacy\old\battle.rs:196 | False | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan |
| 6 | game_ai::plan_legacy::old::BattlePlan::new_dive | pub | game-ai\src\plan_legacy\old\battle.rs:246 | False | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan |
| 7 | game_ai::plan_legacy::old::BattlePlan::new_region | pub | game-ai\src\plan_legacy\old\battle.rs:296 | False | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState, u64, u64, u64) -> game_ai::plan_legacy::old::BattlePlan |
| 8 | game_ai::plan_legacy::old::BattlePlan::goal | pub | game-ai\src\plan_legacy\old\battle.rs:341 | False | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_core::BigGoal |
| 9 | game_ai::plan_legacy::old::BattlePlan::set_main_objective | pub | game-ai\src\plan_legacy\old\battle.rs:349 | True | fn(&mut game_ai::plan_legacy::old::BattlePlan, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) |
| 10 | game_ai::plan_legacy::old::BattlePlan::return_to_objective | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:354 | False | fn(&game_ai::plan_legacy::old::BattlePlan) -> bool |
| 11 | game_ai::plan_legacy::old::BattlePlan::current_focus | pub | game-ai\src\plan_legacy\old\battle.rs:358 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> std::option::Option<usize> |
| 12 | game_ai::plan_legacy::old::BattlePlan::with_runaway | pub | game-ai\src\plan_legacy\old\battle.rs:367 | False | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &game_core::OperationData) -> bool |
| 13 | game_ai::plan_legacy::old::BattlePlan::v3_beyond_enemy_line | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:387 | False | fn(&game_ai::plan_legacy::old::BattlePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool |
| 14 | game_ai::plan_legacy::old::BattlePlan::no_enemy_idle_watchdog | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:406 | False | fn(&mut game_ai::plan_legacy::old::BattlePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool |
| 15 | game_ai::plan_legacy::old::BattlePlan::update | pub | game-ai\src\plan_legacy\old\battle.rs:427 | False | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::old::BattlePlan::update_v32 | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:748 | False | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) |
| 17 | game_ai::plan_legacy::old::BattlePlan::next_plan | pub | game-ai\src\plan_legacy\old\battle.rs:2034 | True | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 18 | game_ai::plan_legacy::old::BattlePlan::is_end | pub | game-ai\src\plan_legacy\old\battle.rs:2039 | False | fn(&game_ai::plan_legacy::old::BattlePlan) -> bool |
| 19 | game_ai::plan_legacy::old::BattlePlan::sub_plan | pub | game-ai\src\plan_legacy\old\battle.rs:2043 | False | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | 108100000001 (L521 near_wide_allies 거리²) 의 유래 — 정수 제곱이 아니어서 r² 접힘으로 설명 안 됨(1.081e11). 소스 표기 표기 불가 | 4 |  |
| 1 | 미탐색 | L591 should_end_object_finish_kill_priority_battle 이 version≥2 에서 건너뛰어지는 것이 의도인지 — CFG(%849/%853 의 version<2 분기)로 확정, 설계 의도 미확인 | 4 |  |
| 2 | 미탐색 | v25_scoped_battle_objective·is_unreasonable_tower_dive_enemy 에 version 이 poison 으로 전달됨 — 두 callee 가 version 을 안 읽는다는 LLVM 판정. callee 본문 미독 | 4 |  |
| 3 | 미탐색 | L466 v54_reentry_ticks 와 L538 chats 의 push 가 힙 재할당(grow_one)을 일으킬 수 있음 — HEAP_SUBST 재료. 그 외 힙: near_enemies(bumpalo pool, L540 drop) · near_enemies.clone(L531) · Vec::new_in(L532) · debug String/Vec(L527, ctx.debug 만) | 4 |  |
| 4 | 미탐색 | watchdog 의 champ.ty 가 Champion 이 아닌 경우(action_state 검사 생략) 가 실제로 있는지 — player_champion 은 챔프 엔티티라 사실상 항상 13 | 4 |  |
| 5 | 미탐색 | engage_dir(battle.rs:49~50) 의 Protect/Kiting/Assassin 류 반환값 — IR 에서 dir=0·oor=0 으로 접혀 있어 정확한 enum 표기 unknown | 4 |  |
| 6 | 미탐색 | L1580 종료 전 update_v32 내부 — 별도 define(fastcc), 이번 범위 밖 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | Entity+0x38 visible_state[t] 인덱스 stride — IR gep 가 24B 구조로 인덱싱(VisibleState 24B) 확인, `is_visible@data:122` 가 `== Visible(0)` 인 것 확정 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L733 `safe` 변수의 소스 형태 — IR 은 risk_all_zero 참일 때 `safe = i8 1` dbg 를 달면서도 종료 조건이 `!recent_start` 만으로 결정되고(%1567), 거짓일 때는 safe=has_near_visible_enemies(%1572). 종료 조건은 IR 대로 확정(logic 참조), 소스 표현은 unknown | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

