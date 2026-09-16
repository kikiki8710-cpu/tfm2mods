# r21 티어1 심층 — 웨이브 2 (2) · 0.5.8 명세 logic 전문 + exe 힌트

요구: 0.5.8 명세(아래 logic) 를 기준선으로 0.6.0 디컴을 **줄 단위로 대조**해 「어느 줄이 어떻게 바뀌었나 / 새 분기·조건·상수·필드·콜리」 를 명세 형식(줄번호 참조 · 등가 Rust)으로 낸다. version≥3 분기는 「v3 경로」 로 별도 표기(값 미확정 · v2/v3 둘 다 적는다). 대형 함수는 먼저 콜리 지도·블록 지도(0.5.8 ↔ 0.6.0 매핑)를 만들고 신규 구역을 특정한 뒤 신규 구역만 정독한다.

## `dfb840` → `dd8e70` BattlePlan::update  (9320→15066B · Δ+5746)
- 힌트: BattlePlan::update(9.3→15KB · dd8e70 · screening +0x204 플래그를 읽음 · v3 에서 try_engage 가 2회 호출)
- exe 정렬: 명령 2096→3183 · 정렬 1473 · 잔여 구조 10 · 분기 10 · 콜리 주의: 1323a00→eeb400 미지(+306B) ; 1323a00→eed240 미지(+350B) ; ca19b0→e0e6e0 불일치(mig060 는 e0bd00) ; d1fe30→dea110 미지 ; d1fe30→ee7fd0 미지 ; d1fe30→eeb400 미지 ; d73310→e7f1d0 미지(+196B) ; d8ca70→13f0790 불일치(mig060 는 ffdda0)
- 0.5.8 명세 #102 `battle__BattlePlan_update` src game-ai\src\plan_legacy\old\battle.rs:427 · one_line: Battle 플랜의 매판 갱신 — 교전 종료(sub_goal=End)/도주(RunAway) 전환 조건 사슬을 순서대로 검사하고 통과하면 update_v32 로 위임
- 0.5.8 params: self:&mut BattlePlan(280B)(noalias, readonly 없음(쓰기 가능) — writes 참조. 반환 없음(void)) · version:usize(alloca %28 에 저장 후 재로드. 분기: `>1`(v2 경로: watchdog·fight_kit_ra) · rnd:&mut StdRng(320B)(noalias·align16·readonly 없음(쓰기 가능). 직접 소비 0 — check_kill_die) · player:&PlayerState(2528B)(readonly. team(+0x930)·position(+0x9c0)·info.id(+0x928)·para) · data:&OperationData(24B)(readonly. cache/context/blackboard) · positioning_score:&PositioningScoreData(2760B)(readonly. 본문 직접 읽기 0 — L707 position_risk_all_zero_near 에 전달) · team_plan:&TeamPlan(`ptr nonnull align 8`(dereferenceable 없음=크기 미표기). objective() · debug:&mut DebugFrameData(224B)(noalias. L527(ctx.debug 일 때만) infos(+0xa0 HashMap) entry pus)
- 0.5.8 logic 전문:
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

## `df36e0` → `dcce40` BattlePlan::update_v32  (29389→42894B · Δ+13505)
- 힌트: BattlePlan::update_v32(29→43KB · dcce40 · resolve_join_stake(committed=1,6) 신규 caller · +0x1f8 with_dive/+0x204 screening 분기)
- exe 정렬: 명령 5713→8103 · 정렬 4435 · 잔여 구조 10 · 분기 10 · 콜리 주의: 16fc160→dd82a0 미지(+1022B) ; 31a37c0→3821813 미지(+32B) ; 31a37c0→ee3770 미지(+191B) ; 31a3863→17d3bc0 미지(+54B) ; 32d1730→33a50 미지(+353B) ; 33a50→dca0e0 콜리 변경?(J0.00·+207B) ; 33a50→dca750 미지(-38B) ; 33a50→ee3850 콜리 변경?(J0.00·+682B)
- 0.5.8 명세 #109 `battle__BattlePlan_update_v32` src game-ai\src\plan_legacy\old\battle.rs:748 · one_line: Battle 플랜 매틱 갱신: 교전 focus 선정→다이브/타워/웨이브 위험 판정→sub_goal(Trace/Kiting/KitingBack/RunAway/End)과 exit_src·dive 상태를 self 에 기록
- 0.5.8 params: self:&mut BattlePlan(280B)(%0 noalias dereferenceable(280), readonly 없음 → &mut. sub_goa) · version:usize(%1. AI 버전 게이트: `version > 1`(icmp ugt %1,1) 이 v2+ 분기, `versi) · rnd:&mut StdRng(320B)(%2 noalias align16 dereferenceable(320), readonly 없음 → &mut.) · player:&PlayerState(2528B)(%3 noalias readonly dereferenceable(2528) → &. info.team(+0x) · data:&OperationData(24B)(%4 noalias readonly dereferenceable(24) → &. cache(+0x0 &Abs) · _positioning_score:&PositioningScoreData(DWARF arg 6. IR 에서 dead-arg 제거(#dbg_value ptr poison !25373)) · team_plan:&TeamPlan(1064B)(%5 — noalias·readonly·dereferenceable 전부 없음. 원인 = TeamPlan 이) · debug:&mut DebugFrameData(224B)(%6 noalias dereferenceable(224), readonly 없음 → &mut. add_log)
- 0.5.8 logic 전문:
```
// 기호: champ=내 챔피언 &Entity, team=player.info.team, tps=setting.tick_per_second, cache=data.cache, bump=context.pool. 모든 Vec<&Entity> 는 bumpalo(bump) 할당. ★=self 기록. 인라인 헬퍼: engage_pair_range(v,data,a,b)= v>1 ? max(fight_kit_range_cached(data,a,b),fight_kit_range_cached(data,b,a)) : max(max_range_cached(data,a,b),max_range_cached(data,b,a)) ; is_ignored_well_enemy(e)= e.team==Player(1-team) && is_enemy_well_danger(version,player,e.x,e.y) ; is_ignored_battle_enemy(e,with_declared_dive)= (version>1 && with_declared_dive) ? is_ignored_well_enemy(e) : (is_ignored_well_enemy(e) || is_unreasonable_tower_dive_enemy(player,data,e,with_declared_dive)) ; Effect::range(caster)= eff.range(+0x4a0)+caster.stat_buff_cached.range(+0x438)+(caster.level-1)*eff.growth_range(+0x4a8) ; Entity::radius()= radius_mult(+0x470)==0 ? radius(+0x680) : radius*(mult+100)/100 ; distance_sq= dx^2+dy^2 (abs_diff).

// battle.rs:750~772
if self.sub_goal == RunAway(4) { return; }                                   // 750, %3136 = ret void 직행(아무 기록 없음)
let mut committed_dir: i8 = match self.sub_goal { Trace|Kiting|Assassin|AssassinReady => 1, KitingBack => -1, Protect|End => 0 };   // 755~766(주석) 인라인 switch
if version > 1 && cache.tick() == self.start_tick && team_plan.last_resolver_bail_tick != 0 && tick > last_resolver_bail_tick {   // 767~769
    if !(tick > last_resolver_bail_tick + 3*tps) { committed_dir = -1; }      // 771 ([v3 F6] 리졸버패배 이탈 커밋 상속)
}
// 774
let champ = cache.player_champion[team][player.info.position.as_index()].unwrap();   // team>=2 → panic_bounds_check, None → unwrap_failed
// 781~785 (version>1 만)
if version > 1 {
    let incoming = tower_discipline::v3_survival_incoming(player, data, champ);   // 782
    if !(incoming < champ.hp || self.sub_goal == RunAway) { ★exit_src=8; ★sub_goal=RunAway; return; }   // 783~785
}
// 790~794
let near_enemies: Vec<&Entity> = cache.iter_champions(1-team).filter(|e| { let r = engage_pair_range(version,data,champ,e); e.distance_sq(champ) <= (r+30000)^2 && data.blackboard[1-team].is_recent_visible(e) && !is_ignored_battle_enemy(e, self.with_dive) }).collect();
// 797~800
let near_allies: Vec<&Entity> = (0..5).filter(|&c| team_plan.ally_battle_stop_tick[c].is_none() && cache.player_champion[team][c].is_some_and(|a| a.distance_sq(champ) < 120000^2+1)).filter_map(|c| cache.player_champion[team][c]).collect();   // 본인 포함
// 811~812
let can_near_list = team_plan.can_near_enemies_range(version, rnd, player, data, champ.x, champ.y, 150000, debug); let can_near_enemies = can_near_list.len();
// 818~821
let anticipated: (u64,u64) = if sub_goal ∈ {Trace,Assassin,AssassinReady} { cache.get_entity_by_id(sub_goal.focus).map(|e|(e.x,e.y)).unwrap_or((champ.x,champ.y)) } else { (champ.x,champ.y) };
// 824~827
let threat_list = if version>1 && anticipated != (champ.x,champ.y) { can_near_enemies_range(…, anticipated.0, anticipated.1, 150000, debug) } else { can_near_list.clone() };
// 829~845
let threat_enemies: Vec<&Entity> = if version>1 { threat_list.iter().copied().filter(|e| {   // s2_0, 832~839
        if (0..5).any(|i| data.blackboard[team].big_goal[i].1 == Some(BigGoal::Battle{focus: Some(e.id)})) { return false; }   // 이미 아군 big_goal 의 Battle focus 인 적 제외
        if e.ty != Champion { return true; }                                   // 837
        if e.ty.Champion.action_state != Move { return true; }                 // 838
        utils::distance(move.x, move.y, anticipated) <= utils::distance(e.x, e.y, anticipated)   // 839 (내 예상 위치로 접근 중인 적만)
    }).collect() } else { Vec::new_in(bump) };
// 848~853
let nearest_enemy_tower: Option<&Entity> = cache.iter_towers_without_nexus(1-team).min_by_key(|t| t.distance_sq(champ)).filter(|t| t.distance(champ) <= t.attack_effect.unwrap().range(t) + 15000 + champ.radius() + t.radius());
// 856~857
let nearest_enemy_tower_type: Option<TowerType> = nearest_enemy_tower.and_then(|t| (t.ty==Tower).then(|| t.ty.Tower.info.ty));
// 861~870
let die_enemies = { let mut v = near_enemies.clone(); v.extend(threat_enemies.iter().copied()); v };
let die_tick_with_tower = check_kill_die_tick(version, rnd, data, player, champ, die_enemies.clone(), Vec::from_iter(nearest_enemy_tower), debug);   // 866
let mut die_tick = check_kill_die_tick(version, rnd, data, player, champ, die_enemies.clone(), Vec::new_in(bump), debug);          // 869~870
// 879~892
die_tick = match committed_dir { 1 => if die_tick >= tps { die_tick.saturating_add(tps) } else { die_tick }, -1 => die_tick.saturating_sub(tps), _ => die_tick };
let prev_die_eval = self.prev_die_eval; ★self.prev_die_eval = die_tick;         // 887~888
if die_tick < prev_die_eval { ★self.die_fall_evals = die_fall_evals.saturating_add(1) } else { ★self.die_fall_evals = 0 }   // 889~892
// 900~904
if version>1 && committed_dir == -1 && self.sub_goal == KitingBack && die_tick <= tps*2 && self.die_fall_evals >= 2 && self.with_runaway(data) { ★sub_goal=RunAway; return; }
// 908~920
let is_in_tower_focused = nearest_enemy_tower.is_some_and(|t| t.ty==Tower && t.ty.Tower.info.nearest_enemy.is_some_and(|(_,id)| id == champ.id));
let is_in_tower_range   = nearest_enemy_tower.is_some_and(|t| champ.distance_sq(t) <= (t.attack_effect.unwrap().range(t) + 10000 + champ.radius() + t.radius())^2);
// 923~935
if let Some(st) = self.support_target { let target = cache.get_entity_by_id(st); if target.is_none_or(|x| x.distance_sq(champ) < 60000^2+1 || !blackboard[1-team].is_recent_visible(x) || is_ignored_battle_enemy(x, with_dive)) { ★support_target = None; } }
if let Some(st) = self.support_target.and_then(get_entity_by_id) { if v21_should_defer_support_target(_v, player, data, champ, st, near_allies.len(), near_enemies.len(), can_near_enemies, die_tick, die_tick_with_tower) { ★support_target = None; } }
// 939~964
let chase_range = 250000;
let focused: Option<&Entity> = if near_enemies.is_empty() { cache.iter_champions(1-team).filter(|c| c.distance_sq(champ) <= chase_range^2 && blackboard[1-team].is_recent_visible(c) && !is_ignored_battle_enemy(c, with_dive)).min_by_key(|c| c.distance_sq(champ)) }   // 946~957
              else { near_enemies.iter().min_by_key(|e| check_kill_die_tick(version, rnd, data, player, /*champ=*/e, near_allies.clone(), Vec::new_in(bump), debug)).copied() };   // 942~944: 아군들이 가장 빨리 죽이는 적
let support_target = self.support_target.and_then(get_entity_by_id).filter(|x| !is_ignored_battle_enemy(x, with_dive));   // 960~962
let focused = support_target.or(focused);                                  // 964
// 966~1002
if focused.is_none() {
    ★exit_src = 1;                                                            // 967
    if self.ff_exit1_cls == 255 {                                             // 970~984 [S1a 계측]
        let (mut invis, mut well, mut towerf) = (0,0,0);
        for c in cache.iter_champions(1-team).filter(|c| c.distance_sq(champ) <= 250000^2) { if !blackboard[1-team].is_recent_visible(c) { invis+=1 } else if is_ignored_well_enemy(c) { well+=1 } else { towerf += is_unreasonable_tower_dive_enemy(player,data,c,with_dive) as u32 } }
        ★ff_exit1_cls = match (invis==0, well==0, towerf==0) { (t,t,t)=>0, (f,t,t)=>1, (t,f,t)=>2, (t,t,f)=>3, _=>4 };
    }
    let return_to_objective = main_objective ∈ {Nexus(4),PressTower(10)} || (version>1 && nexus_last_stand(player,data));   // 996~997
    ★sub_goal = if return_to_objective { End } else { RunAway };            // 998 no_valid_battle_target_goal
    return;
}
let mut focused = focused.unwrap();
// 1006~1022
if focused.team == champ.team {                                               // focused 가 아군(=support 대상)이면 그 주변 적으로 교체
    let near_focused_enemies = cache.iter_champions(1-team).filter(|e| e.distance_sq(focused) < 150000^2+1 && blackboard[1-team].is_recent_visible(e) && !is_ignored_battle_enemy(e, with_dive)).collect();   // sh_0
    if near_focused_enemies.is_empty() { ★exit_src = 2; ★sub_goal = End; return; }   // 1012~1014
    focused = *near_focused_enemies.iter().min_by_key(|e| check_kill_die_tick(version,rnd,data,player, e, near_allies.clone(), Vec::new_in(bump), debug)).unwrap();   // 1018~1021
}
// 1031~1062
if near_enemies.is_empty() && committed_dir == -1 && focused.team == Player(1-team) {
    let target_enemies = cache.iter_champions(1-team).filter(sk_0 /*= sh_0 기준 focused 150000*/).collect();                    // 1033~1037
    let target_tower = iter_towers_without_nexus(1-team).filter(|t| t.can_target && t.block_target_tick==0).min_by_key(|t| t.distance_sq(focused)).filter(|t| t.distance(focused) <= t.attack_effect.unwrap().range(t)+15000+focused.radius()+t.radius());   // 1039~1043
    let rejoin: FightPrediction = if version>1 { let roster = fight_participants(version,rnd,data,player,champ,&near_allies,&target_enemies,team_plan,debug); resolve_fight_stake_roster(version,data,champ,&roster,&target_enemies,committed_dir,target_tower,player.info.parameter.judge_accuracy()) }   // 1050~1053
                 else { resolve_fight_stake(version,rnd,data,player,champ,&near_allies,&target_enemies,committed_dir,target_tower,judge_accuracy,debug) };   // 1054~1055
    if rejoin.line != Commit { ★exit_src = 13; ★sub_goal = End; return; }   // 1057~1059
}
// 1073~1079
let mut model_prediction: Option<FightPrediction> = if version>1 && !near_enemies.is_empty() { let roster = fight_participants(…,&near_allies,&near_enemies,…); Some(resolve_fight_stake_roster(version,data,champ,&roster,&near_enemies,committed_dir,nearest_enemy_tower,judge_accuracy)) }
                                                   else { Some(resolve_fight_stake(version,rnd,data,player,champ,&near_allies,&near_enemies,committed_dir,nearest_enemy_tower,judge_accuracy,debug)) };
// 1085~1094
if let Some(p) = model_prediction.as_mut() { if let Some(ally_id) = p.rescue_ally {
    let ally = cache.get_entity_by_id(ally_id);
    let helping = ally.is_some_and(|a| focused.distance_sq(a) > (engage_pair_range(version,data,focused,a)+30000)^2);   // 1087~1089
    if ally.is_some() && !helping { ★self.ff_rescue_tick = cache.tick(); }   // 1094 (p.line 유지)
    else { p.line = p.line_absolute; }                                        // IR 실측(1091~1093 레지스터 전용, 소스 문장 형태 미확정)
} }
let model_commits = model_prediction.is_some_and(|x| x.line == Commit);    // 1098~1099
// 1110~1125
let model_soaker = model_prediction.and_then(|p| p.soaker);
let soaker_ok = model_soaker.is_none_or(|s| s == champ.id); let i_am_soaker = model_soaker == Some(champ.id);   // 1111 (IR 두 값 %1182/%1181)
let mut my_die_tick_with_tower;
if with_dive && let (Some(dt),Some(nt)) = (self.dive_tower, nearest_enemy_tower_type) && (v50_engine_truth_scope = (dt.is_twin() && nt.is_twin()) || dt==nt) {   // 1115~1118
    my_die_tick_with_tower = if is_in_tower_focused { die_tick_with_tower } else { die_tick };   // 1124 → 1170 으로 점프(1137~1165 생략)
} else {
    my_die_tick_with_tower = if soaker_ok || is_in_tower_focused { die_tick_with_tower } else { die_tick };   // 1125
    // 1137~1165
    let focused_closer_to_tower = version>1 && !with_dive && !is_in_tower_range && nearest_enemy_tower.is_some_and(|t| focused.distance_sq(t) <= champ.distance_sq(t));   // 1137~1138 (%1253)
    let warn_zone = is_in_tower_range || focused_closer_to_tower;              // %1252
    if !with_dive && warn_zone && focused.team == Player(1-team) && !(focused.move_speed*10 < champ.move_speed*9) {   // 1142~1145
        let my_range = max_range_cached(data, champ, focused);                // 1147
        if focused.distance_sq(champ) > my_range^2 {                          // 1148
            if version>1 { if !with_dive { ★dive_abandoned=true; ★dive_abort_src=9; }   // 1150~1152
                           ★exit_src=7;                                        // 1154
                           if my_die_tick_with_tower > tps { let kite_focus = near_enemies.iter().min_by_key(|e| e.distance_sq(champ)).map(|e| e.id).unwrap_or(focused.id); ★sub_goal=KitingBack{kite_focus}; return; }   // 1158~1161
                           else { ★sub_goal=RunAway; return; } }              // 1164
            else { ★exit_src=7; ★sub_goal=RunAway; return; }
        }
    }
}
// 1170~1185 (version < 2 만)
if version < 2 { if let Some(mo)=self.main_objective && mo ∈ {Morgard,Serpen} && mo.phase == Hunt && player.strategy(rnd, game).object_finish == KillPriority {
    match objective_entity_id_for_main_objective(data, mo).and_then(get_entity_by_id) { None => { ★exit_src=4; ★exit_sub=6; ★sub_goal=End; return; }
        Some(obj) => if focused.team == Player(1-team) && !can_enemy_hit_objective(focused, obj, 25000) { ★exit_src=4; ★exit_sub=7; ★sub_goal=End; return; } }
} }
// 1196~1206
if is_enemy_well_danger(version, player, focused.x, focused.y) {            // focused_in_enemy_well
    ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★dive_abort_src=6; ★exit_src=6;   // 1198~1203
    ★sub_goal = if main_objective ∈ {Nexus,PressTower} { End } else if champ.distance_sq(well_anchor(team)) < 300000^2+1 { RunAway } else { End };   // 1204~1206, well_anchor = team==1 ? (0,960000) : (960000,0)
    return;
}
// 1218~1219
let mut dive_viable = with_dive && tower_dive_is_viable(version, rnd, player, data, team_plan, focused, /*team_model=*/true, debug);
// 1230~1278
let dive_tower_matches = self.dive_tower.is_some() && nearest_enemy_tower_type.is_some_and(|nt| (dt.is_twin()&&nt.is_twin()) || dt==nt);
let v50_dive_episode = with_dive && dive_tower.is_some() && focused.team==Player(1-team) && (dive_tower_matches || !is_in_tower_focused);   // 1232~1235 (IR 등가)
if v50_dive_episode && is_in_tower_range { ★dive_entered = true; }        // 1236
if with_dive && (dive_tower.is_none() || v50_dive_episode || !is_in_tower_focused) { ★dive_ctx_break_ticks = 0; }   // 1242~1244
if with_dive && dive_tower.is_some() && !v50_dive_episode {
    if focused.team==Player(1-team) && is_in_tower_focused {                 // 1248
        ★dive_ctx_break_ticks += 1;                                          // 1250
        if dive_entered ? dive_ctx_break_ticks >= tps : dive_ctx_break_ticks >= 2*tps {   // 1253/1257
            if near_allies.len()>1 { ★chats.push(BattleStop(TowerDiveFail)) }   // 1259
            ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★dive_abort_src=2; ★exit_src=10; ★sub_goal=End; return;   // 1260~1265
        }
    }
    if is_unreasonable_tower_dive_enemy(player, data, focused, /*with_declared_dive=*/true) && !dive_viable && !model_commits {   // 1271
        ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★dive_abort_src=13; ★exit_src=10; ★sub_goal=End; return;   // 1272~1278
    }
} else { (1271 도 평가되나 %1486=false 로 전달; 결과는 위 조건과 동일하게 &&) }
// 1282~1334
let focused_die_tick = check_kill_die_tick(version, rnd, data, player, focused, near_allies.clone(), Vec::new_in(bump), debug);
let max_range = max_range_cached(data, champ, focused); let focused_is_in_range = focused.distance_sq(champ) <= max_range^2;   // 1286~1287
let tower_close_danger = nearest_enemy_tower.is_some_and(|t| champ.distance_sq(t) < 40000^2+1) && my_die_tick_with_tower <= 2*tps;   // 1290~1291
let tower_on_other = nearest_enemy_tower.is_some_and(|t| t.ty==Tower && t.nearest_enemy.is_some_and(|(_,id)| get_entity_by_id(id).is_some_and(|e| e.ty==Champion && e.id != champ.id)));   // 1294~1297
let mut warn_tower = tower_close_danger || my_die_tick_with_tower < 11 || (my_die_tick_with_tower <= tps && is_in_tower_focused);   // 1304~1307
if tower_on_other { warn_tower = false } else if let (Some(dt),Some(nt)) = (dive_tower, nearest_enemy_tower_type) && (dt==nt || (dt.is_twin()&&nt.is_twin())) { warn_tower = false }   // 1311~1314
if context.debug { debug.map[+0xa0].entry(champ.id).or_insert(vec![]).push(format!(…)) ×2 }   // 1317~1319
let nearest_enemy_tower = iter_towers_without_nexus(1-team).min_by_key(|t| t.distance_sq(focused));   // 1322 섀도잉(필터 없음)
let attack_range = champ.attack_effect.unwrap().range(champ) + champ.radius() + focused.radius();   // 1324
let focused_is_in_tower = nearest_enemy_tower.is_some_and(|t| t.distance_sq(focused) <= (t.attack_effect.unwrap().range(t)+focused.radius()+t.radius()).saturating_sub(attack_range) + 10000)^2);   // 1325~1331
let target_hp_ratio = focused.hp*100 / focused.stat_cached.hp;               // 1333 (0 → div_by_zero panic)
let is_dive_current_tower = dive_tower.is_some() && nearest_enemy_tower_type.is_some_and(|nt| (dt,nt 둘 다 Twin) || dt==nt);   // 1334
// 1335~1345
let close_execute_range = max_range + 25000;
let guard = with_dive && is_dive_current_tower;
if tower_close_danger { dive_viable = guard && dive_viable; }
else { let outlast = focused_die_tick.saturating_add(tps/2) < my_die_tick_with_tower;   // 1336
       let close_exec = focused.distance_sq(champ) <= close_execute_range^2 && (target_hp_ratio < 30 || ready_damage_to_target(context, champ, focused) >= focused.hp);   // 1339~1340
       let has_close_execute = guard && close_exec && my_die_tick_with_tower > tps;   // 1341
       dive_viable = guard && (dive_viable || outlast || has_close_execute); }   // 1344~1345 (current_dive_can_continue)
if context.debug && i_am_soaker { debug.add_log(…) }                        // 1348~1349
// 1358~1362
if focused.distance_sq(champ) <= close_execute_range^2 && ready_damage_to_target(context, champ, focused) >= focused.hp {
    let kill_secure = focused_die_tick > my_die_tick_with_tower;               // 1360
    if !kill_secure { ★sub_goal = Trace{focused.id}; return; }                // 1361~1362
}
// 1373
if v50_dive_episode {
    // ===== 1374~1555 (v50 다이브 에피소드 판정) =====
    let holder = is_in_tower_focused;                                         // 1374
    let soaker = dive_entry_soaker(player, focused);   // fight_model.rs:1073~1074 인라인: 아군 (0..5) 중 focused 200000 이내 → max_by(stat_cached.hp, 동률 id)
    let i_am_entry_soaker = soaker.map(|e| e.id) == Some(champ.id);           // 1376
    let aggro_secured = tower_on_other || is_in_tower_focused;                // 1377
    if context.debug { add_log }                                              // 1378~1382
    ★dive_last_model = match model_prediction.map(|p| p.line) { None|Some(CommitAfterJoin)=>0, Some(Commit)=>1, Some(Hold)=>2, Some(Disengage)=>3 };   // 1385~1386
    ★dive_last_race_adv = min(die_tick_with_tower,2000) as i32 - min(focused_die_tick,2000) as i32;   // 1388
    ★dive_last_na = min(near_allies.len(),255); ★dive_last_ne = min(near_enemies.len(),255);   // 1389~1390
    if holder || dive_chase_catchable(champ, focused, tps) { ★dive_catch_break_ticks = 0 } else { ★dive_catch_break_ticks += 1 }   // 1397~1400
    if !holder && !dive_chase_catchable(champ,focused,tps) && self.dive_entered && dive_catch_break_ticks >= tps {   // 1402~1403
        if context.debug { add_log }   // 1408~1411
        if near_allies.len()>1 { ★chats.push(BattleStop(TowerDiveFail)) }   // 1413
        ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★dive_abort_src = if focused_die_tick < die_tick_with_tower {11} else {1}; ★exit_src=10; ★sub_goal=End; return;   // 1414~1419
    }
    if model_prediction.is_some_and(|p| p.line == Disengage) {                // 1427
        ★dive_model_break_ticks += 1;                                        // 1433
        if version < 2 || dive_model_break_ticks >= tps || (version>1 && my_die_tick_with_tower <= tps) {   // 1437~1438
            if context.debug { add_log }   // 1439~1441
            if near_allies.len()>1 { ★chats.push(BattleStop(TowerDiveFail)) }   // 1443
            ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★dive_model_break_ticks=0; ★exit_src=10;   // 1444~1449
            ★sub_goal = if holder { KitingBack{focused.id} } else { End }; return;   // 1450
        }
    } else { ★dive_model_break_ticks = 0; }                                   // 1454
    if holder {                                                               // 1456
        ★tactic = Frontline;                                                  // 1459
        if !(die_tick_with_tower > tps) && focused_die_tick.saturating_add(tps/2) > die_tick_with_tower {   // 1465~1466
            if die_tick_with_tower <= tps/2 || near_allies.len() < 2 { ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★exit_src=10; ★sub_goal=End; return; }   // 1469~1474
            else { ★sub_goal = KitingBack{focused.id}; return; }              // 1477
        }
        let hold_margin = if committed_dir == -1 { tps/3 } else { tps/6 };   // 1480
        if let Some((exit, tdie)) = soaker_tower_hold_components(cache, context, team, champ) {   // 1481
            if context.debug { add_log }   // 1482~1486
            if !(die_tick_with_tower > hold_margin + exit) && focused_die_tick.saturating_add(tps/2) > die_tick_with_tower {   // 1491~1492
                if context.debug { add_log }   // 1494
                if near_allies.len()>1 && die_tick_with_tower > tps/2 { ★sub_goal = KitingBack{focused.id}; return; }   // 1497, 1505
                else { ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★exit_src=10; ★sub_goal=End; return; }   // 1498~1501
            }
            if !(tdie > hold_margin + exit) {                                 // 1508
                if near_allies.len()>1 && die_tick_with_tower > tps/2 { ★sub_goal = KitingBack{focused.id}; return; }   // 1511, 1519
                else { ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★exit_src=10; ★sub_goal=End; return; }   // 1512~1516
            }
        }
    } else {                                                                  // !holder
        if i_am_entry_soaker && !aggro_secured { ★tactic = Frontline; }      // 1524~1526
        if !(die_tick > tps) && (if dive_entered { focused_die_tick.saturating_add(tps/2) } else { focused_die_tick }) > die_tick { ★sub_goal = KitingBack{focused.id}; return; }   // 1529~1532
    }
    // 1542~1555
    if !(version<=1 || with_dive || model_commits) && v3_beyond_enemy_line(team, cache, focused) {
        ★exit_src = 7;                                                        // 1543
        ★sub_goal = if die_tick > tps && self.with_runaway(data) { End } else { RunAway }; return;   // 1547 (IR: with_runaway true → End(7), 아니면 RunAway(4))
    }
    ★sub_goal = Trace{focused.id}; return;                                    // 1554
}
// ===== 1558~1581 (비 v50) =====
if warn_tower {
    let dist = focused.distance_sq(champ);                                    // 1560
    let attack_range = champ.attack_effect.unwrap().range(champ) + champ.radius() + focused.radius();   // 1562
    if dist >= attack_range^2 && (focused_closer_to_tower || is_in_tower_range) && !dive_viable && self.with_runaway(data) && !model_commits {   // 1564
        if with_dive { if focused_is_in_tower && near_allies.len()>1 { ★chats.push(BattleStop(TowerFocused)) } ★dive_abandoned=true; ★exit_src=8; ★sub_goal=End; return; }   // 1565~1572
        else { ★exit_src=8; ★sub_goal=KitingBack{focused.id}; if focused_is_in_tower { if near_allies.len()>1 { ★chats.push(BattleStop(TowerFocused)) } ★sub_goal=RunAway; } return; }   // 1575~1581
    }
}
// 1587~1610
let (mut wave_obs, mut wave_pct, mut wave_danger) = (0u8, 0u8, false);
let can_runaway = self.with_runaway(data); let return_to_objective = main_objective ∈ {Nexus,PressTower}; let open_eval = cache.tick() == self.start_tick;   // 1591~1592
let wave_goal: Option<BattleSubPlanGoal> = v30_wave_danger_chase_guard(version, data, champ, focused, max_range, focused_die_tick, focused_is_in_range, /*my_die_tick=*/die_tick, target_hp_ratio, can_runaway, return_to_objective, open_eval, &mut wave_obs, &mut wave_pct, &mut wave_danger);   // 1590
if self.ff_wave_obs_open == 255 { ★ff_wave_obs_open=wave_obs; ★ff_wave_open_hp=min(champ.hp*100/max(champ.max_hp,1),254); ★ff_wave_open_pct=wave_pct; ★ff_wave_open_danger=wave_danger; }   // 1595~1600
if let Some(goal) = wave_goal { ★exit_src=9; if ff_wave_fire_hp==255 { ★ff_wave_fire_hp=min(hp%,254); ★ff_wave_fire_pct=wave_pct; ★ff_wave_fire_danger=wave_danger; } ★sub_goal = goal; return; }   // 1602~1610
// 1614~1629
if with_dive && is_dive_current_tower && is_in_tower_range && !dive_viable {
    let safe_low_hp = target_hp_ratio < 70 && !(focused_is_in_tower || tower_close_danger || warn_tower || is_in_tower_focused);   // 1618
    if !(safe_low_hp || model_commits) {
        if (focused_is_in_tower || is_in_tower_focused) && near_allies.len()>1 { ★chats.push(BattleStop(TowerDiveFail)) }   // 1623~1624
        ★dive_abandoned=true; ★dive_abort_src=12; ★exit_src=10; ★sub_goal=End; return;   // 1626~1629
    }
}
// 1633~1671
let situation = FightSituation::build(version, rnd, player, data, team_plan, debug, near_allies.len(), near_enemies.len(), can_near_enemies, die_tick_with_tower, die_tick, focused, focused_die_tick, focused_is_in_range, warn_tower, is_in_tower_range, is_in_tower_focused, is_dive_current_tower, focused_is_in_tower, target_hp_ratio);
★tactic = Standard;                                                            // 1641
let die_tick_mult = if near_enemies.len() > 1 {
    let m = if i_am_soaker { ★tactic=Frontline; 50 } else if situation.my_battle_role == Utility { ★tactic=Peel; 100 } else if role ∈ {BaseAttacker,SkillCaster} { ★tactic=BacklineDPS; 150 } else { 100 };   // 1647~1653
    if situation.ally_dps_sum > 0 && situation.enemy_dps_sum > 0 { clamp(enemy_dps_sum*100/ally_dps_sum, 50, 150) } else if near_enemies.len()==2 { 100 } else { m }   // 1662~1664
} else { 100 };
let tps_1 = die_tick_mult*tps/100; let tps_2 = 2*tps*die_tick_mult/100; let tps_3 = 3*tps*die_tick_mult/100;   // 1669~1671
// 1675~1716
let v50_ally_absorbs = tower_on_other && !is_in_tower_focused;
if !v50_ally_absorbs && (warn_tower || target_hp_ratio > 49) && (focused_closer_to_tower || is_in_tower_range) && !is_dive_current_tower && self.with_runaway(data) && !model_commits {   // 1676
    let dive_winnable = tower_dive_is_viable(version, rnd, player, data, team_plan, focused, /*team_model=*/false, debug);   // 1679
    if context.debug && i_am_soaker { add_log }   // 1681~1682
    let legacy = version<=1 || with_dive;
    let v3_promote_ok = if legacy || self.dive_abandoned { false } else { !open_chase_race_hopeless(version, data, player, champ, focused) };   // 1689~1690
    if dive_winnable && (if legacy||dive_abandoned { legacy } else { v3_promote_ok || with_dive }) {   // 1691
        if v3_promote_ok { ★with_dive=true; ★dive_tower = iter_towers_without_nexus(1-team).min_by_key(|t| t.distance_sq(focused)).and_then(|t| (t.ty==Tower).then(|| t.ty.Tower.info.ty)); ★dive_join_tick=cache.tick(); }   // 1708~1716 (legacy 는 아무것도 안 함)
    } else {
        if !legacy { ★dive_abandoned=true; ★dive_abort_src=9; }             // 1695~1697
        ★exit_src=15; ★sub_goal=KitingBack{focused.id};                      // 1699~1700
        if focused_is_in_tower { if near_allies.len()>1 { ★chats.push(BattleStop(TowerDiveFail)) } ★sub_goal=RunAway; }   // 1701~1705
        return;
    }
}
if context.debug { add_log }                                                  // 1721~1725
// 1734~1768
let prediction = model_prediction.unwrap_or_else(|| resolve_fight_full(version, data, champ, &near_allies, &near_enemies, committed_dir, /*tower=*/None, judge_accuracy, baseline));   // 1734~1735 (sG_0)
let resolver_join_pending = prediction.line == CommitAfterJoin;              // 1736
match prediction.line {
    Commit => { if self.v46_brace { if context.debug { add_log } ★v46_brace=false; } ★sub_goal = Trace{prediction.focus_target.unwrap_or(focused.id)}; return; }   // 1743~1751
    Disengage => { ★exit_src=11; if self.with_runaway(data) && committed_dir != 1 { if version>1 && die_tick > tps_1 { ★sub_goal=KitingBack{focused.id} } else { if near_allies.len()>1 { ★chats.push(BattleStop(Outnumbered)) } ★sub_goal=RunAway } } else { ★sub_goal=KitingBack{focused.id} } return; }   // 1756~1768
    CommitAfterJoin | Hold => { /* 1778~1861 사다리 */
        let (A,E,FIR,FDT,DT,CNE) = (near_allies.len(), near_enemies.len(), focused_is_in_range, focused_die_tick, die_tick, can_near_enemies);
        if A < E {
            if FIR && FDT <= DT && with_runaway() { ★sub_goal=KitingBack{focused.id}; return; }   // 1779~1780
            else if CNE != 0 && DT <= tps_3 && with_runaway() { if A>1 { ★chats.push(BattleStop(Outnumbered)) } ★exit_src=12; ★sub_goal=RunAway; return; }   // 1784~1789
            else if DT > tps_3 || !with_runaway() { ★sub_goal=Kiting{focused.id} }   // 1793, 1797
            else { ★exit_src=12; ★sub_goal=RunAway }                          // 1794~1795
        } else if A > E {
            if FIR && FDT <= DT { ★sub_goal=Trace{focused.id}; return; }      // 1800~1801 (1871 후처리 생략)
            else if FDT > DT { if DT < tps_3 { if DT < tps_2 { if DT < tps_1 { if with_runaway() { if A>1 { ★chats.push(BattleStop(LowHp)) } ★exit_src=12; ★sub_goal=RunAway } else { ★sub_goal=Kiting } }   // 1811~1829
                                                            else { if with_runaway() { ★sub_goal=KitingBack } else { ★sub_goal=Kiting } } }   // 1816~1819
                                              else { ★sub_goal=Kiting } }     // 1814
                               else { ★sub_goal=Trace } }                     // 1812
            else { ★sub_goal = if DT > tps_2 { Trace } else { Kiting } }     // 1806~1809
        } else { // A == E
            if FIR && FDT <= DT { ★sub_goal=Kiting{focused.id}; return; }    // 1833~1834
            else if CNE > 1 && DT <= tps_3 && with_runaway() { if A>1 { ★chats.push(BattleStop(BurstRisk)) } ★exit_src=12; ★sub_goal=RunAway; return; }   // 1838~1843
            else if FDT > DT { if DT > tps_2 { ★sub_goal = if (focused_is_in_tower || DT <= tps_3) && with_runaway() { KitingBack } else { Kiting } }   // 1848, 1858~1861
                               else { if with_runaway() { if A>1 { ★chats.push(BattleStop(LowHpMatch)) } ★exit_src=12; ★sub_goal=RunAway } else { ★sub_goal=Kiting } } }   // 1849~1856
            else { ★sub_goal=Kiting }                                          // 1847
        }
        // 1871~1895 후처리 (focus 는 전부 focused.id)
        if model_prediction.is_some_and(|p| p.line == Hold) && committed_dir != 0 {
            let new_dir = match sub_goal { Trace=>1, KitingBack|RunAway=>-1, _=>skip };   // 1873 engage_dir
            if new_dir != committed_dir { if committed_dir==1 && DT > tps_1 { ★sub_goal=Trace{focused.id} }   // 1875
                else if committed_dir==-1 { let oor = !cache.iter_champions(1-team).any(|e| e.distance_sq(champ) <= (engage_pair_range(version,data,champ,e)+30000)^2 && blackboard[1-team].is_recent_visible(e) && !is_ignored_battle_enemy(e,with_dive)); if oor { ★sub_goal=KitingBack{focused.id} } } }   // 1877~1884
        }
        if committed_dir==1 && DT > tps_1 && sub_goal == RunAway { ★sub_goal=KitingBack{focused.id} }   // 1893~1895
        // 1899~1957
        if self.tactic == BacklineDPS && A < 2 && E > 1 { if sub_goal ∈ {Trace,Kiting} { ★sub_goal@tag = KitingBack (focus 유지) } }   // 1899~1902
        else if sub_goal ∈ {Trace,Kiting} {                                   // 1914
            if self.with_runaway(data) {                                      // 1915
                if FDT > DT {                                                 // 1920
                    let refuge = iter_towers_without_nexus(team).min_by_key(|t| t.distance_sq(champ)).or(cache.nexus[team]);   // 1928~1929
                    if let Some(refuge) = refuge {
                        let reaction = champ.attack_duration() + 6; let escape_ticks = champ.distance(refuge) / max(champ.move_speed,1);   // 1931~1932
                        if !self.v46_brace { if !resolver_join_pending && DT <= escape_ticks + reaction { ★v46_brace=true; if debug { add_log } } }   // 1933~1941
                        else if DT > escape_ticks + reaction { if debug { add_log } ★v46_brace=false; }   // 1945~1953
                    }
                    if self.v46_brace { ★sub_goal=KitingBack{focused.id} }   // 1956~1957
                } else { if v46_brace && debug { add_log } ★v46_brace=false; }   // 1922~1926
            } else { ★v46_brace=false; }                                      // 1919
        }
        // 1964~2001
        if self.tactic == Frontline { if let Some(&t) = near_enemies.iter().max_by_key(|e| dps_ratio(context, e, champ)) { if t.id != focused.id { match sub_goal { Trace=>★Trace{t.id}, Kiting=>★Kiting{t.id}, KitingBack=>★KitingBack{t.id}, _=>{} } } } }   // 1965~1977
        if self.tactic == SkillBurst { if !champ.can_skill() && !champ.can_skill2() { let my_attack_dps = champ.attack_effect.map(|e| e.expected_damage_target(context,champ,focused)*100000/max(champ.attack_cooltime(),1)).unwrap_or(0?); let my_total_dps = dps_ratio(context, champ, focused); if my_total_dps != 0 && my_attack_dps*100/my_total_dps < 30 && sub_goal ∈ {Trace,Kiting} { ★sub_goal@tag=KitingBack } } }   // 1987~2000 ※이 함수는 tactic 을 SkillBurst 로 절대 안 놓음 → 함수 내부 기준 사장 코드
        // 2016~2029
        if version>1 && nexus_last_stand(player, data) { match sub_goal { RunAway => ★sub_goal=Trace{focused.id}, End => {}, _ => ★sub_goal=Trace{기존 focus} } }
    }
}
// 2032: die_enemies·threat_enemies·threat_list·can_near_list·near_allies·near_enemies drop(bump) 후 return
```
