# apply060 batch_08.md — 반영(logic_060) 브리핑 · 함수 9

★**version = 3 런타임 확정(09-17)** — v<3/v2 경로는 명세에서 제외(사장). 태그/오프셋/vt 슬롯 = `spec_patch_060.md` §A(dispcheck 추가 행 포함). 출력 형식·절차 = `mkapply060.py` 도크스트링. 출력 = `_next/apply060/out/<old>.md`.

RE 정본 파일: 2026-09-17_r21_심층_w1_check_kill_die_tick_uncached_check_kill_원문.md · 2026-09-17_r21_심층_w2_BattlePlan_update_update_v32_원문.md · 2026-09-17_r21_심층_w4_LegacyPlanHandler_update_원문.md · 2026-09-17_r21_심층_w5_handle_interact_battle_원문.md · 2026-09-17_r21_심층_w6_passive_plan_get_small_action_원문.md · 2026-09-17_r21_심층_w8_LineGanker_next_plan_Hide_action_candidates_원문.md

### `eba9b0` → `edb240` fight_check::check_kill_die_tick_uncached (i=211 · 변경·심층(r21 §D: w1 §1))
- 0.5.8 src: `game-ai\src\fight_check.rs:970` · one_line: focus 챔피언이 지금 교전을 시작하면 몇 틱 뒤에 죽는가(die tick) — 적 챔피언·타워·기타 유닛의 nuke(선제 버스트)와 dps 를 판단력 노이즈 곱으로 합산해 (hp+부활보너스−nuke)*60/dps
- 0.6.0 판정: **심층(r21)** · 패치 요지: v2 = 구 본체 + bool 게이트(simple/ignore_nuke/no_noise) · **v3 = 신규 타임라인 알고리즘**(arrival·궁 즉시분/지연분·pool=hp*tps·undying 하한 · tick 버킷 없음) · 캐시 +0x46..+0x4b
- RE 정본: `2026-09-17_r21_심층_w1_check_kill_die_tick_uncached_check_kill_원문.md` · 절: w1 §1
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize`
- consts: [{"value": 9223372036854775807, "src_line": 976, "meaning": "i64::MAX(9223372036854775807) — focus 가 undying 이면 반환하는 검열 die_tick(_docs game_ai L364 가 「지평 밖 검열 큰 값」이라 부르는 값 · i64::MAX 하나뿐이고 다른 센티널 없음. 오라클 26E C01 und=1 → 9223372036854775807) · 오라클 실행 확증(26차 배치E: 오라클 실행 확인(26E o26E.rs · 케이스당 프로세스 1개 · 명세 logic 독립 재구현 predict ↔ 실행 대조 · run26E.py 100/100 MATCH) — C01 und=1 → 9223372036854775807)", "kind": "센티널", "ev": 2}, {"value": 1, "folded_from": 2, "src_line": 987, "meaning": "bucket = tick / (tps*2) — `shl i64 %41, 1` 로 접힘(2초 단위 버킷). 별도로 1 은 umax(tps,1)·umax(cooltime,1)·umax(dps,1) 바닥값·`or 1`
- 0.5.8 logic 전문:
```
fn check_kill_die_tick_uncached(version, _rnd, data, judger, focus, enemy: Vec<&Entity>, towers: Vec<&Entity>, _debug) -> usize   // fight_check.rs:970
 if focus.stat_buff_cached.undying { return i64::MAX }                                   L976
 player = cache.player_by_champion_id(focus.id).unwrap()   // focus 의 PlayerState        L979
 tps_raw = setting.tick_per_second ; tps = max(tps_raw, 1)                                L986
 bucket = game.tick() / (tps*2)   // 2초 버킷 (tps*2==0 이면 div_by_zero 패닉 경로)        L987
 rng = NoiseRng( (judger.info.id * 0x9E3779B97F4A7C15) ^ (focus.id << 24) ^ bucket )      L989~990
 ja = judge_accuracy(&judger.info.parameter)   // 100..=1000                               L994
 d = (1000 - ja) >> 1 ; noise() := rng.range_usize(1000 - d, 1000 + d)   // [lo,hi] 양끝 포함 · splitmix64 → hi64(z * (hi-lo+1)) + lo   L995
 enemy_dps = 0 ; enemy_nuke = 0                                                            L998
 cc(p) := cache.player_champion_cache[p.team][p.pos] ; fpos := player.pos
 for pchamp in enemy {                                                                     L1000
   p = cache.player_by_champion_id(pchamp.id).unwrap()                                     L1002
   nuke = 0
   if pchamp.can_attack() || pchamp.attack_cooldown() <= tps_raw { nuke = cc(p).attack[fpos] * noise() / 1000 }               L1005~1007  ←rng#1
   if pchamp.can_skill()  || pchamp.skill_cooldown()  <= tps_raw { nuke = max(nuke, cc(p).skill[fpos]  * noise() / 1000) }   L1010~1012  ←rng#2
   if pchamp.can_skill2() || pchamp.skill2_cooldown() <= tps_raw { nuke = max(nuke, cc(p).skill2[fpos] * noise() / 1000) }   L1015~1017  ←rng#3
   if pchamp.can_ult()    || pchamp.ult_cooldown()    <= tps_raw { nuke = max(nuke, cc(p).ult[fpos]    * noise() / 1000) }   L1021~1023  ←rng#4
     // *_cooldown() 은 Champion(ty 13) 이면 필드, 아니면 0 → 항상 통과
   if !pchamp.is_block_attack() { enemy_dps += cc(p).attack_per_sec[fpos] * noise() / 1000 }                                   L1026~1028  ←rng#5
   if !pchamp.is_block_skill() && !(pchamp.is_block_move_skill() && pchamp.skill_effect.map_or(false, |e| e.can_move()))
        { enemy_dps += cc(p).skill_per_sec[fpos] * noise() / 1000 }                                                               L1031~1033  ←rng#6
   if !pchamp.is_block_skill() && !(pchamp.is_block_move_skill() && pchamp.skill2_effect().map_or(false, |e| e.can_move()))
        { enemy_dps += cc(p).skill2_per_sec[fpos] * noise() / 1000 }      // skill2_effect() = level>2 ? Some(..) : None    L1036~1038  ←rng#7
   enemy_nuke += nuke                                                                      L1040
 }   // ult_per_sec 는 합산하지 않음
 for tower in towers {                                                                     L1049
   damage = tower.attack_effect.unwrap().expected_damage_target(ctx, tower, focus)          L1050
   enemy_dps  += (noise() * tps_raw * damage / 1000) / tower.attack_cooltime()   // cooltime 0 이면 div_by_zero 패닉   L1051~1053 ←rng#8
   enemy_nuke += damage * noise() / 1000                                                    L1054  ←rng#9
 }
 for e in cache.others[1 - player.team] {                                                  L1058
   if dist²(e, focus) > 22500000000 { continue }          // > 150,000                     L1059
   if !game.is_visible(judger.team, e.id) { continue }                                     L1063
   if let Some(atk) = e.attack_effect {                                                    L1066
     damage = atk.expected_damage_target(ctx, e, focus)                                    L1067
     enemy_dps  += damage * tps_raw / max(e.attack_cooltime(), 1)                          L1068
     enemy_nuke += damage                                   // 노이즈 없음                 L1069
   }
 }
 for pchamp in enemy { for b in pchamp.effect_buffs { enemy_dps += b.expected_aura_dps_at(ctx, pchamp, focus) } }          L1075~1077
 ally_heal = 0 ; for ally in cache.iter_champions(player.team) { for b in ally.effect_buffs { ally_heal += b.expected_aura_heal_at(ctx, ally, focus) } }   L1084~1087
 enemy_dps = enemy_dps.saturating_sub(ally_heal)                                           L1090
 enemy_epic_buff := (game.get_game_mode() is Moba(m)) && m.epic_minion_buff_time[1 - player.team] != 0   // IR %653 은 이 값의 부정(dbg_value 에 DW_OP_not) — 소스 변수명·극성 일치   L1095
 low_enough_to_care_minions := focus.hp*100 <= max(focus.stat_cached.hp,1)*75   // IR %661 은 이 값의 부정(dbg_value DW_OP_not) · 오라클 C31(751/1000 → 제외)·C32(750/1000 → 포함) 경계   L1096
 line_phase := if tutorial ∈ {None,MidBottom,Line,Total} { game.tick() < setting.epic_jungle.first_spawn_tick.saturating_sub(tps_raw*30) } else { true }   L1097  (오라클 C15 fst=1900·tick=100 → 100<100 거짓 / C16 fst=1901 → 참 경계 실측)
 if enemy_epic_buff || (low_enough_to_care_minions && !line_phase) { enemy_dps += enemy_minion_wave_risk_dps_at(version, data, focus, focus.x, focus.y) }   L1098~1099  (IR: %653 참 && (%661 || %673) 이면 건너뜀 — 동치 · 오라클 C13/C14/C17/C18/C19/C33/C34 로 4분기 전부 실행 확인)
 revive_hp = Σ focus.effect_buffs.revive_bonus_hp(ctx, focus)                              L1104
 return ((revive_hp + focus.hp).saturating_sub(enemy_nuke) * 60) / max(enemy_dps, 1)       L1107
 // 드롭: enemy·towers Vec (bumpalo, L1110)
 // NoiseRng 추첨 순서(상태는 한 스트림): 적 챔피언마다 최대 7회(attack·skill·skill2·ult·attack_ps·skill_ps·skill2_ps, 각 조건부) → 타워마다 2회(dps, nuke) → others/버프 0회. 총 사이트 9곳. StdRng(_rnd) gen_range 0회.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `dfb840` → `dd8e70` BattlePlan::update (i=102 · 변경·심층(r21 §D: w2 §1))
- 0.5.8 src: `game-ai\src\plan_legacy\old\battle.rs:427` · one_line: Battle 플랜의 매판 갱신 — 교전 종료(sub_goal=End)/도주(RunAway) 전환 조건 사슬을 순서대로 검사하고 통과하면 update_v32 로 위임
- 0.6.0 판정: **심층(r21)** · 패치 요지: 본체 동치(재번호·이중모드) · Defense 넥서스 거리 End 는 v<3 만 · battle_recency End 사장 · **v3 에필로그 REBAR**(이탈 4초 내 Trace 복귀 억제 · engage_stamp +0x20d)
- RE 정본: `2026-09-17_r21_심층_w2_BattlePlan_update_update_v32_원문.md` · 절: w2 §1
- sig: `fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData)`
- consts: [{"value": 2, "src_line": 430, "meaning": "team bounds · L439/549 `version>1`(ugt 1) · L570 `version<2` · watchdog L423 max(.,2) · L441 exit_src=2 · MainObjective 2=Defense · action_state 2=Move · Option None 니치(v54_prev_eval byte0=2·discipline kind=2)", "kind": "센티널", "ev": 4}, {"value": 7, "src_line": 439, "meaning": "BattleSubPlanGoal End 태그 — L439 게이트(sub_goal!=End) 및 모든 종료 store", "kind": "태그", "ev": 4}, {"value": 4, "src_line": 452, "meaning": "sub_goal RunAway 태그 · exit_src/exit_sub 4 · dive_abort_src 4 · JungleType Morgard=4 · TowerType/그 외", "kind": "태그", "ev": 4}, {"value": 3, "src_l
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
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `df36e0` → `dcce40` BattlePlan::update_v32 (i=109 · 변경·심층(r21 §D: w2 §0·§2))
- 0.5.8 src: `game-ai\src\plan_legacy\old\battle.rs:748` · one_line: Battle 플랜 매틱 갱신: 교전 focus 선정→다이브/타워/웨이브 위험 판정→sub_goal(Trace/Kiting/KitingBack/RunAway/End)과 exit_src·dive 상태를 self 에 기록
- 0.6.0 판정: **심층(r21)** · 패치 요지: 신규 구역 8: 플립플롭 계측/브레이커(exit 12) · TANKAVAIL(die_avail) · TWBIN(bb 공유 스탬프 · exit 15) · 솔로듀얼(7/9) · 추격진척(7/10) · TBFLEE(16) · JOINGATE(13) · line 히스테리시스 · dd82a0→Kiting · team_dive 게이트 · v3 latch · **BattlePlan 0.6.0 필드표**
- RE 정본: `2026-09-17_r21_심층_w2_BattlePlan_update_update_v32_원문.md` · 절: w2 §0·§2
- sig: `fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData)`
- consts: [{"value": 1, "src_line": 767, "meaning": "`version > 1`(icmp ugt %1,1) = v2+ 게이트. 같은 값 1 이 `shl i64 tps,1`(=tps*2: 902/1253/1290/1438/1670) 와 `lshr tps,1`(=tps/2: 1336/1466/1492/1511/1531) 의 시프트량으로도 쓰임", "folded_from": 2, "kind": "임계", "ev": 4}, {"value": 3, "src_line": 771, "meaning": "last_resolver_bail_tick + tps*3 (`mul %177,3`) · 1671 tps_3 = 3*tps*mult/100 · 1480 hold_margin = tps/3(committed_dir=-1) · TowerType Twin 판정 `add -3`. ※본문의 `shl … , 3` 은 &Entity 슬라이스 stride(×8) 산술이지 임계 아님(시프트량 아님)", "kind": "태그", "ev": 4}, {"value": 150000, "src_line": 811, "meaning": "can_near_enemies_range 
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
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e46bc0` → `d38180` passive_plan (i=105 · 변경·심층(r21 §D: w6 §1))
- 0.5.8 src: `game-ai\src\plan_legacy\handler.rs:1855` · one_line: 판단 없이도 하는 기본 행동 플랜(패시브) 선택 — 스틸/오브젝트/에고웨이브/버프창/라인 리드 순으로 BigPlan 과 발원코드(u8) 반환
- 0.6.0 판정: **심층(r21)** · 패치 요지: 반환 (BigPlan 0x228,code) · objective JT 재배치 · 에고웨이브 `roam>49 skip`·p=(500-10r)(10e)/1000 · PressTower roaming≥40 · ComebackPick 정글 LineGanker(+0x24c5/ff0c10/f1dd50) · 웨이브우선 재작성(f18b10·6b/6c) · legacy_objective 인라인 · ⑧ cc7==0 폐기 · **⑧′ ambient(fb8cc0 · 코드 29)** · ⑩ 정글 +0x1ed0==2?fafe50:PassiveLine(+0x1ed1) · ⑪ Repair v3 undying · **⑪′ cc2 플래너 d56ac0(32/33/34)** · ⑫ +0x24d6 B′(d753e0&&d750f0) · **후처리 E**(Support +0x24b6 · 라이너 +0x24d5 내 라인 복귀 18 · 코드 30/31) · ⚠+0x24d5 = Support 슬롯 제외
- RE 정본: `2026-09-17_r21_심층_w6_passive_plan_get_small_action_원문.md` · 절: w6 §1
- sig: `fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8)`
- consts: [{"value": -2000, "src_line": 1915, "meaning": "에고웨이브 라인 조건: minion_state(fallback_line).from_mid < -2000 (우리 웨이브가 밀림)", "kind": "임계", "ev": 4}, {"value": -2, "src_line": 1915, "meaning": "&& minion_count < -2", "kind": "임계", "ev": 4}, {"value": 500, "src_line": 1916, "meaning": "base = 500 - roaming_ratio ; p = base*ego_ratio/500 (‰). base<1 이면 불참", "kind": "계수", "ev": 4}, {"value": 1000, "src_line": 1919, "meaning": "‰ 분모 계수: gen_range(0..1000) < p 이면 PassiveLine(fallback_line) 채택(발원 6). :2096 점수 가중 계수 = -minion_count*1000 - from_mid 에도 사용", "kind": "계수", "ev": 4}, {"value": 399, "src_line":
- 0.5.8 logic 전문:
```
// handler.rs:1858~1872 ① 정글러 스틸
is_jungle = (position == Jungle)
if is_jungle {
  match self.team_plan.steal_action {                                                   // :1861 (0x510/0x511)
    Some(Lurk(target)):  Serpen && serpen_exists(tutorial∈{None0,MidBottom5,Line7,Total8}) → return (SerpenHuntAndPoke{focus:false, vision_only:true, flee:false}, 1)   // :1871~1872
                         Epic && morgard_exists(tutorial==0 || tutorial>=7)             → return (EpicHuntAndPoke{focus:false, vision_only:true, flee:false}, 1)     // :1865~1866
    Some(Commit(target)): Serpen && serpen_exists → return (SerpenHuntAndPoke{focus_serpen_only:true, vision_only:false}, 1)   // :1868~1869
                          Epic && morgard_exists  → return (EpicHuntAndPoke{focus_epic_only:true, vision_only:false}, 1)      // :1862~1863
    _ (None / None(0) / exists 실패) → 계속
  }
}
// :1878~1892 ② 팀 목표 직결 플랜
match self.team_plan.objective {
  Nexus(line)      → return (AttackNexus{team: 1-my_team, line}, 2)           // :1879
  Defense          → return (DefenseNexus{team: my_team}, 3)                  // :1883
  DefenseLine(line)→ return (PassiveLine::new(line), 4)                       // :1886~1887
  Dive(line)       → return (PassiveLine::new(line), 5)                       // :1891~1892
  _ → 계속
}
// :1897~1920 ③ 에고웨이브(v2)
match self.v2_egowave {                                                                  // 0x1815
  2 → return (PassiveLine::new(self.v2_egowave_line), 6)                               // :1903
  0 → if !is_jungle {                                                                    // :1905
        roam = roaming_ratio; ego = ego_ratio; my_line = Top→Top, Mid→Mid, else Bottom  // :1906~1908
        line = fallback_line(ctx, my_line)                                             // :1913
        ms = blackboard[team].minion_state(line)                                       // :1914
        if ms.from_mid < -2000 && ms.minion_count < -2 {                               // :1915 (우리 웨이브가 크게 밀림)
          base = 500 - roam; if base >= 1 { p = base*ego/500; if rnd.gen_range(0..1000) < p { return (PassiveLine::new(line), 6) } }   // :1916~1920
        }
      }
  _ → 계속
}
// :1928~1939 ④ PressTower 목표
if objective == PressTower(line) {
  if let Some((x,y)) = self.v3_assign_anchor(player, data) {                              // :1929
    if is_near_line(ctx,x,y,line) || (roaming_ratio > 399 && side(line,x,y) /* Top: height-y>=x · Mid: is_near_mid_line · Bottom: height-y<=x */) { return (PassiveLine::new(line), 7) }   // :1931~:1939
  }
}
// :1946~1960 ⑤ ComebackPick 목표
if objective == ComebackPick(line) {
  if is_jungle { return (LineGanker{chats:[], setup_limit:0, wait_limit: tick+tps*15, line, phase:Setup}, 8) }   // :1948~1950
  if let Some((x,y)) = v3_assign_anchor(..) { if is_near_line(..) || side(line,x,y) { return (PassiveLine::new(line), 8) } }   // :1953~1960 (roam 조건 없음)
}
// :1967~1970 ⑥ 웨이브 우선 정리
if team_plan.should_delay_object_setup_for_wave_priority(player,data) {
  if let Some(l) = team_plan.should_player_clear_wave_priority_line(player,data) { return (PassiveLine::new(l), 9) } else → ⑨(:2040) 로 점프(⑦⑧ 건너뜀)   // :1969
} else if game.as_moba().map_or(false,|m| m.epic_minion_buff_time[team]!=0) {            // :1968 우리 에픽 버프 창
  if let Some(l) = should_player_clear_wave_priority_line(..) { return (PassiveLine::new(l), 9) }
}
// :1974~1995 ⑦ 정글러: 오브젝트 셋업 중 손상된 캠프 마무리
if is_jungle && objective ∈ {Morgard,Serpen} && objective.phase == Setup(1) && self.plan is PassiveJungle(pj) {   // :1974~1981
  me = player_champion[team][Jungle]; if None → ⑧
  camp_live = as_moba().map(|m| m.jungle_runner.get_jungle_live_list(pj.jungle, pj.team==0)).unwrap_or_default()   // :1983
  if !camp_live.is_empty() {                                                            // :1984
    (cx,cy) = map.camp_pos(pj.jungle, pj.team==0)                                       // :1985
    if distance_sq(me,(cx,cy)) < 14400000001 && camp_live.iter().any(|id| get_entity_by_id(id).map_or(false,|e| e.hp < e.stat_cached.hp)) { return (PassiveJungle(pj.clone()), 10) }   // :1986~1991
  }
}
// :2000~2034 ⑧ 오브젝트 전용 패시브(serpen/epic)
if player_champion[team][position].is_some() {                                           // :2000
  if objective is Serpen{phase} {                                                       // :2001
    anchor = v3_depart_anchor(v3_armed, team, position, ctx)                            // :2002
    match serpen_passive_plan(version,rnd,player,data,&team_plan,phase,anchor,debug) {
      Some(p) → if v2_armed && (p.tag&30)==14 /*SerpenHuntAndPoke|Battle*/ && !v2_obj_restore_safe(..) { drop(p) → ⑨ } else { return (p, 11) }   // :2007~2012
      None → if v2_armed && v2_obj_part == Some(0x50|phase) && serpen_exists && v2_obj_restore_safe(..) { return (SerpenHuntAndPoke{false,false,false}, 11) }   // :2014~2017
    }
  }
  if objective is Morgard{phase} {                                                      // :2021
    anchor = v3_depart_anchor(..); match epic_passive_plan(..) {                          // :2022
      Some(p) → if v2_armed && (p.tag&30)==12 /*EpicHuntAndPoke|Battle*/ && !v2_obj_restore_safe(..) { drop → ⑨ } else { return (p, 12) }   // :2024~2029
      None → if v2_armed && v2_obj_part == Some(0x60|phase) && morgard_exists && v2_obj_restore_safe(..) { return (EpicHuntAndPoke{false,false,false}, 12) }   // :2031~2034
    }
  }
}
// :2040~2107 ⑨ 에픽옵스 커버 라인 (objective None 일 때만)
if objective.is_none() {
  enemy_buff = as_moba().map_or(false,|m| m.epic_minion_buff_time[1-team]!=0)          // :2041
  recent_giveup = epic_giveup_tick.is_some_and(|t| tick.saturating_sub(t) <= tps*10)    // :2042~2043
  our_buff = v3_epicops_armed && as_moba().map_or(false,|m| m.epic_minion_buff_time[team]!=0)   // :2048~2049
  if (enemy_buff || recent_giveup || our_buff) && player_champion[team][position].is_some() {   // :2051~2052
    best=None; best_score=0
    for line in valid_lines(tutorial) /* {0,7,8}:[T,M,B] 5:[M,B] 4:[M] 2:[T] {1,3}:[B] 6:[] */ {   // :2057
      ms = blackboard[team].minion_state(line); if ms.from_mid > -3001 || ms.minion_count > -4 → continue   // :2058~2060
      if count(아군 i≠me: player_champion[team][i] 살아있고 is_near_line(ctx, e.pos, line)) != 0 → continue   // :2065~2071 (closure#4)
      if enemy_buff || recent_giveup || !our_buff {                                      // (루프 언스위치) 일반 모드
        (ax,ay) = v3_assign_anchor(..).unwrap_or(me.pos)                                // :2087
        ok = is_near_line(ctx,ax,ay,line) || side(line,ax,ay)                            // :2088~2093
      } else {                                                                            // our_buff 전용 모드
        ok = (line 의 포지션 == my position)                                             // :2077~2082
      }
      if ok { score = -ms.minion_count*1000 - ms.from_mid; if score > best_score { best_score=score; best=line } }   // :2096~2097
    }
    if let Some(l) = best {                                                              // :2103
      if !(enemy_buff || recent_giveup || !our_buff) { team_plan.eo_cover_picks.fetch_add(1) }   // :2104~2105 (우리 버프 창 커버 픽 계측)
      return (PassiveLine::new(l), 13)                                                   // :2107
    }
  }
}
// :2113~2126 ⑩ 초반 라인 페이즈 기본
if is_line_phase(ctx,tick) /* tutorial∉{0,5,7,8} || tick < epic.first_spawn_tick - tps*30 */ {   // :2113
  match position { Top→(PassiveLine(Top),14) · Mid→(PassiveLine(Mid),14) · Bottom→(PassiveLine(Bottom),14) · Support→(PassiveLine(Bottom),14)   // :2116~2119,:2126
                   Jungle→ pj = PassiveJunglePlan::with_best(..); pj.last_lead_action_tick = self.last_jungle_lead_action_tick; return (PassiveJungle(pj), 27) }   // :2121~2124
}
// :2128~2186 ⑪ 우리 에픽 버프 창(중반 이후)
if let Some(m) = as_moba() {
  if m.epic_minion_buff_time[team] != 0 {                                                // :2128
    if v3_epicops_armed {                                                                // :2132
      if objective != Repair { if let Some(f) = v3_epic_formation(rnd,player,data) { return (PassiveLine::new(f.line), 15) } }   // :2133~2134
    }
    if objective ∈ {PressEpic(l), SplitEpic(l)} { return (PassiveLine::new(l), 15) }     // :2137~2139
    if objective == Repair && !v3_repair_done(version,team,position,cache) { return (ActiveRecall, 16) }   // :2142~2150
    cand = [Top if line_exists(Top)/*tutorial∈{0,2,7,8}*/ && enemy(1-team) top_tower|top_tower2 살아있음, Mid if line_exists(Mid)/*{0,4,5,7,8}*/ && mid_tower*, Bottom if line_exists(Bottom)/*{0,1,3,5,7,8}*/ && bottom_tower*]   // :2154~2170
    if !cand.is_empty() && player_champion[team][position].is_some() {                   // :2173~2174
      l = cand.iter().min_by_key(|line| distance_sq(me.pos, map.region_centers[map.line_region(line, team, lead[team])])).unwrap()   // :2175~2181
      return (PassiveLine::new(l), 17)                                                   // :2183
    }
  }
}
// :2189~2260 ⑫ 라인 리드 기반 최종 폴백
top_lead/mid_lead/bottom_lead = cache.*_lead[team]                                       // :2189~2191
top_cnt/mid_cnt/bottom_cnt = count(i in 0..5: blackboard[team].in_big_line(i, Top/Mid/Bottom))   // :2192~2194
A_x = x_lead < 3 ; B_x = line_exists(x) && x_lead < 3 && x_cnt == 0 && player_champion[team][x 포지션].is_none()   // :2195~2197 (x∈{Top,Mid,Bottom}; 포지션 Top=0,Mid=2,Bottom=3)
match position {                                                                        // :2199
  Top:    A_top → (Line Top,18) ; else B_mid → (Mid,19) ; else B_bottom && champion[Mid] 없음 → (Bottom,19) ; else (Top,18)   // :2202~2209
  Jungle: if champion[Top] 없음 && champion[Mid] 없음 && (살아있는 아군 캠프 수 < 3 /*aux 정글 closure*/) && champion[Bottom] 없음 {   // :2217~2221
            B_mid → (Mid,19) ; else B_top → (Top,19) ; else B_bottom → (Bottom,19) ; else (Mid,19)    // :2222~2229
          } else { pj = with_best(..); pj.last_lead_action_tick = self.last_jungle_lead_action_tick; (PassiveJungle(pj), 27) }   // :2232~2234
  Mid:    A_mid → (Mid,18) ; else B_top → (Top,19) ; else B_bottom → (Bottom,19) ; else (Mid,18)   // :2238~2245
  Bottom: A_bottom → (Bottom,18) ; else B_top && champion[Mid] 없음 → (Top,19) ; else B_mid && champion[Top] 없음 → (Mid,19) ; else (Bottom,18)   // :2249~2256
  Support: (PassiveLine(fallback_line(tutorial, Mid)), 18)                             // :2260 (tutorial {0,4,5,7,8}→Mid, {1,3}→Bottom, 2→Top, 6→Bottom)
}

// 발원 코드 표(ret.1): 1 스틸 포크 · 2 AttackNexus · 3 DefenseNexus · 4 DefenseLine · 5 Dive 라인 · 6 에고웨이브 · 7 PressTower 앵커 · 8 ComebackPick · 9 웨이브우선 라인 · 10 손상캠프 유지 · 11 serpen 패시브 · 12 epic 패시브 · 13 에픽옵스 커버 · 14 초반 기본 라인 · 15 버프창 라인 · 16 Repair 귀환 · 17 버프창 최근접 라인 · 18 리드 폴백(자기/기본 라인) · 19 리드 폴백(다른 라인) · 27 with_best 정글
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e65b10` → `d6c2b0` LegacyPlanHandler::get_small_action (i=205 · 변경·심층(r21 §D: w6 §2))
- 0.5.8 src: `game-ai\src\plan_legacy\handler\auction.rs:8` · one_line: SmallActionPlay 실행기 — ScoreParameter 를 만들어 self 에 반사하고(positioning_score·wave_snapshot·회두홀드 래치), v2+ 는 생존 절대규칙(RunAway 99999)/글로벌 궁 지시(Ult 99999)를 선반환, 아니면 sub_plan.action_candidates 를 pre_action 병합 입력검사로 거른 뒤(데스매치만) 후보마다 SubPlan::score 를 judge_noise_ratio[SmallAction 종류] 로 스케일해 (점수,액션) 목록을 만들고(배치 E 끝) → 이후 ignore_action 제외·최고점 선택·동률 rnd.choose·RunAway 폴백·디버그 로그를 거쳐 (parameter, score, action) 을 반환한다(배치 F·G).
- 0.6.0 판정: **심층(r21)** · 패치 요지: 본체 동치 + **v3 아군 대상 궁**(eff.vt[0xc0] info · defensive_crisis(ally) → Ult{ally} 99999) + 계측 + TBPICK 로그 삭제 · 재번호(Ult 17 · 갈래 B {2,3,6,7,8,9}) · sub_plan +0x13a0 · +0xde0 · +0x24d3 · +0x14c8
- RE 정본: `2026-09-17_r21_심층_w6_passive_plan_get_small_action_원문.md` · 절: w6 §2
- sig: `fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &std::vec::Vec<(usize, game_core::SmallAction), std::alloc::Global>, &mut game_core::DebugFrameData) -> (game_ai::ScoreParamet`
- consts: [{"value": 33, "src_line": 9, "meaning": "ProfTimer phase id (_t_csp = calculate_score_parameter 구간)", "kind": "산출값", "ev": 4}, {"value": 35, "src_line": 16, "meaning": "ProfTimer phase id (_t_ws = wave_snapshot 구간)", "kind": "산출값", "ev": 4}, {"value": 36, "src_line": 92, "meaning": "ProfTimer phase id (_t_ac = action_candidates 구간)", "kind": "산출값", "ev": 4}, {"value": 37, "src_line": 174, "meaning": "ProfTimer phase id (_t_sl = 채점 구간)", "kind": "산출값", "ev": 4}, {"value": 1000000000, "src_line": 13, "meaning": "ProfTimer drop: secs*1e9+nanos (ns 환산, 판정 아님)", "kind": "임계", "ev": 4}, {"value": 1
- 0.5.8 logic 전문:
```
// auction.rs:0~201 (배치 E)
// 표기: L<n> = auction.rs 루트 줄 · (m13.ll:NNNNN) = 원문 IR 줄 · vt[0xNN] = &dyn AbstractGame vtable 슬롯 · 간접호출 없음(이 범위의 sub_plan 호출은 JT 호스트 SubPlan::* 직접 call)

L9  _t_csp = ProfTimer::start(33)            // prof::ENABLED(atomic i8)==0 이면 nanos=-1 (비활성) (45832~45848)
L10 parameter: ScoreParameter = calculate_score_parameter(version, rnd, player, data, debug)   // sret 5384B, rnd·debug 는 콜리 define 이 readnone (45850)
L11 self.sub_plan.calculate_score_parameter_value(version, rnd, player, data, &mut parameter)   // (&mut self.sub_plan 0x768) (45861)
L12 self.positioning_score(0x990) = parameter.positioning_score(0x9f0)   // memcpy 2760B (45873)
L13 drop(_t_csp)   // nanos!=-1 → PHASE_NANOS[33]+=elapsed_ns, PHASE_CALLS[33]+=1 (45878~45930)
L16 _t_ws = ProfTimer::start(35)
L17 prediction_depth = player.info.parameter(0x180).last_hit_prediction_depth()
L18 source_quality  = player.info.parameter.last_hit_source_quality()
L19 parameter.wave_snapshot = Some(build_minion_wave_snapshot(player, data, prediction_depth, source_quality))   // 태그 1 @+0, 2320B @+8 (45980~45982)
L20 drop(_t_ws)
L22 team = player.info.team(0x930) (bounds<2, 46046) ; champ = data.cache.player_champion[team][player.info.position(0x9c0) as usize].unwrap()   // 0x1e0 + team*40 + pos*8 (46051~46066)

L29 if version > 1 {                                                     // (46081) v1 이하는 L56 으로 직행
      if game.get_game_mode()(vt[0x40]) 태그 != 2 /*DeathMatch*/ {     // (46094~46100) 데스매치면 L30~38 건너뛰고 L50 으로
L30     base_exempt = champ.stat_buff_cached.undying(0x488)
L31        || nexus_final_stand(player, data)
L32        || data.cache.nexus[team].is_some_and(|n| champ.distance(n) < 180001)   // (46117~46134)
L33     sh: SoloHuntObs = solo_hunt_obs(game.data, game.vtable, team, champ)     // sret 16B
L34     scene_now = if base_exempt || sh.ally_ctx > 1 { false }
                    else if sh.al120 + 1 < sh.en150 && sh.chasers != 0 { sh.aa_safe }   // SoloHuntObs::is_target_scene 인라인 (46146~46176)
                    else { false }
L35     parameter.v3_turnback_hold(0x1500) = scene_now || self.v3_turnback_scene_prev(0x1809)   // 직전 경매 래치 (46178~46183)
L36     self.v3_turnback_scene_prev = scene_now
L37     if data.context.debug(0x3b) {
L38       debug.add_log(data, player, format!("TBHOLD-EVAL T{} {:?} hold={} now={} ex={} al={} ctx={} en={} ch={} aa={}", team, position, parameter.v3_turnback_hold, scene_now, base_exempt, sh.al120, sh.ally_ctx, sh.en150, sh.chasers, sh.aa_safe)) }
      }
L50   if v3_survival_incoming(player, data, champ) >= champ.hp(0x670)          // (46275~46281) incoming < hp 이면 통과
L51      && !nexus_final_stand(player, data) {
L52     return (parameter, 99999, SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5)))   // sret +0x1508=99999, +0x15c1=3 (46293~46313) → ret
      }
    }

L56 if let Some((target_id, until_tick)) = self.pending_global_ult_target(0x530) {   // (46269~46272)
L58   if game.tick()(vt[0x28]) > until_tick → L88                                    // (46322~46332)
L61   if !champ.can_ult() → L88
L64   target = game.get_entity_by_id(target_id)(vt[0x1f0]) ; None → L88               // (46342~46349)
L66   eff = champ.ult_effect()  // 인라인: level(0x5c8) > 4 ? &champ.ult_effect(0x538) : &NONE(@anon.76)
L67   if eff 태그(+0x30) == -1 (None) → L88
L69   enemy = 1 - team
L70~72 visible_enemies = Σ_{i<5} data.cache.player_champion[enemy][i].is_some_and(|e| data.blackboard[enemy].is_recent_visible(game, player, e) && distance_sq(champ, e) < 22500000001)   // 루프 5회 (46427~46511)
L73   if visible_enemies == 0 && CastingTarget::check(&eff.target(+0x28), champ, target) {
L75     return (parameter, 99999, SmallActionPlay::Ult(SmallActionUlt::new(data, target_id)))   // +0x15c1=18 (46528~46548) → ret
      }
L88   self.pending_global_ult_target = None   // 위 게이트 중 하나라도 실패 (46551)
    }

L92 _t_ac = ProfTimer::start(36)
L93 candidates: bumpalo Vec<SmallActionPlay> = self.sub_plan.action_candidates(version, rnd, player, data, &parameter, &self.team_plan(0xf8), debug)   // JT 호스트 → *SubPlan::action_candidates(r16) 분기 (46573)
L94 drop(_t_ac)
L99 if game.get_game_mode() 태그 == 2 /*DeathMatch*/ {                       // (46659~46670) 아니면 candidates 그대로 L125 로
L100  move_speed = champ.stat_cached.move_speed(0x640)
L101~115 filtered = candidates.iter().filter(closure$2).cloned().collect_in(data.context.pool)   // aux m14.ll:58208 (58237~58351)
        closure$2(a):  if a.태그 ∈ 15..=18 (Attack/Skill/Skill2/Ult) → true                      // L103
                       let mut merged = pre_action.clone();                                       // L108
                       merged.merge(version, champ, a.clone());                                   // L109
                       match merged.get_input(version, &mut rnd.clone()/*Array64::clone·실스트림 미소비*/, player, data, &parameter.positioning_score, debug) {   // L110
                         None(-1) → false                                                         // L113
                         Some(Input::Move{x,y})(0) → max(|x-champ.x|, |y-champ.y|) >= move_speed   // L112 체비셰프 ≥ 이속 이면 유지
                         Some(_) → true }
L116  if filtered.len() == 0 {
L119    combat = self.sub_plan.combat_fallback(version, rnd, player, data)
L120    candidates = if combat.len() == 0 { candidates } else { combat }   // (46766~46790)
L121  } else { candidates = filtered }
L122 }
L125 self.v3_last_stand(0x1804)  = nexus_last_stand(player, data)
L126 self.v3_final_stand(0x1805) = nexus_final_stand(player, data)
L127 self.v3_bail_goal(0x1813) = if self.plan 태그(0x5e8)==9 /*Battle*/ { match plan.sub_goal 태그(0x648) { 0 Trace→0, 2 Kiting→1(L130), 3 KitingBack→2(L131), 4 RunAway→3(L132), 5|6 Assassin/Ready→4(L133), 1 Protect→5(L134), 7 End→6(L135) } } else { 7 }   // (46843~46888)
L139 self.v3_cand_src(0x1812) = match self.sub_plan {                                  // switch 태그-2 (46889~46933)
L140   DefenseNexus(idx15) → match last_gate(0x780) { 1→1, 2→2, _→3 }
L141   Battle(idx5)        → if last_bail_gate(0x79d) ∈ 1..=3 { +3 } else if candidates.len()==1 { 7 } else { 8 }
       _ → 0 }
L146 if self.sub_plan 은 Battle(태그7) && candidates.len()==1 {                          // (46934~46940)
L147   match candidates[0].get_action() { SmallAction::RunAway → self.v48_dodge_flee_picks(0x1548) += 1   // 태그 3,4,8 (RunAway/Recall/AroundRunAway)
L148                                     SmallAction::AroundPosition → self.v48_dodge_step_picks(0x1540) += 1   // 태그 7,11,12 + untagged AroundPosition
                                     _ → {} } }                                          // (46958~46991)
L154 judge_accuracy = player.info.parameter.judge_accuracy()
L157 judge_noise_ratio_now = if game.get_game_mode() 태그 == 2 { 0 } else { (1000 - judge_accuracy) >> 1 }   // (46995~47003)
L158 lo = 1000 - ratio ; L159 hi = 1000 + ratio
L165 disc = discriminant(self.plan)  // 태그>1 ? 태그-2 : 4(DeathMatchBattle untagged) (47012~47017)
L166 if self.judge_noise_plan(0x560) != Some(disc) {                                     // (47023~47030)
L167   self.judge_noise_plan = Some(disc)
L168   for i in 0..11 { self.judge_noise_ratio[i](0x17a0+8i) = rnd.gen_range(lo..=hi) }  // ★rnd 소비 11회, i 순서 (47041~47069)
     }
L174 _t_sl = ProfTimer::start(37)
L175 judge_noise_ratio = self.judge_noise_ratio(0x17a0)   // [i64;11] 지역 복사
L177~185 with_score: bumpalo Vec<(i64, SmallActionPlay)> = candidates.into_iter().map(closure$3).collect_in(pool)   // aux m01.ll:48954 (49060~49311)
        closure$3(a):  score = self.sub_plan.score(version, &parameter, rnd/*실스트림*/, player, data, &a, debug)   // L178 (49093)
                       k = a.get_action() 판별자 (SmallAction: RunAway0 · Positioning1 · Around2 · AroundPosition3 · Trace4 · Attack6 · Skill7 · Skill2 8 · Ult9 · Stop10 ; Dodge5 는 생성 안 됨)   // L179
                       if |score| < 6 { score } else { judge_noise_ratio[k] * score / 1000 }   // L180~L183 (49164~49173, sdiv)
                       (score, a)                                                                 // L184
L186 drop(_t_sl)
L189 if data.context.trace_level(0x39) != Off(0) {                                       // (47217~47220) Off 면 → 배치 F(줄 254, %688)
L190   if with_score.iter().any(|(_, a)| a.is_ult_escape())  /* a 태그==3 RunAway && with_ult(+0x81) */ {   // (47250~47276)
L192     hp_ratio = champ.hp * 100 / champ.stat_cached.hp(0x628)   // 분모 0 → panic_const_div_by_zero (47293~47302)
L193~196 visible_enemies = Σ 적팀 5명: is_some && blackboard[1-team].is_recent_visible(game, player, e) && distance_sq < 22500000001   // L72 와 동형 (47364~47449)
L199     nearby_allies = Σ 아군 5명(언롤): is_some && e.id(0x5c0) != champ.id && distance_sq(champ, e) < 22500000001   // 가시성 조건 없음 (47455~47821)
L201     self.pending_trace_events(0x858).push(PendingTraceEvent{ event: UltEscape{hp_ratio, visible_enemies, nearby_allies, ult_used:false}, tick: game.tick()(vt[0x28]) })   // L202 의 tick 호출 포함 (47823~47877)
       }                                                                                  // → 배치 F(줄 215, %710)
     }

// rnd(StdRng) 소비 순서(이 범위): L11 calculate_score_parameter_value(계약) → L93 action_candidates(계약) → [L119 combat_fallback(계약, 데스매치·필터 전멸 시)] → [L169 gen_range ×11 (플랜 판별자 변경 시)] → L178 SubPlan::score × candidates.len() (후보 순서). L110 필터의 get_input 은 rnd 복제본이라 비소비.
// 반환 경로(이 범위): L52 RunAway 99999 · L75 Ult 99999 — 둘 다 parameter 이동 + 즉시 ret(%2439). 나머지는 배치 F·G.
// 사장 코드: reach.py(version=2, gamemode=0) 기준 사장 호출부 0 — NA 봉인 없음.

// auction.rs:202~339 (배치 F)
// 진입: ①%710(L215) ← %697(L190 루프 종료)/%940(L201 push 후) — 즉 E 의 `if context.trace_level(0x39) != Off {…}`(L189, 47217~47220) 블록 안 ②%688(L254) ← %684(L189 trace_level==Off) · %710(L215 불일치) · L221/L223/L239 종료.
// L202: `tick: game.tick()`(vtable+0x28, 47823~47825 %924) — 배치 E 의 L199~201 PendingTraceEvent push(47832 에서 +176 에 저장) 의 마지막 필드. 판정 없음 → 배치 E(줄 201).

// ── [L215~242] TowerEngage 트레이스 (trace_level != Off 안에서만 · 관측 전용) ──
L215 (47280~47289): if self.plan@tag(0x5e8)==9(Battle) && self.plan.Battle.0.sub_goal@tag(0x648)==4(RunAway)  [select 로 접힘 — 0x648 은 태그 무관 로드]
  L217 (47883): let towers = data.cache.iter_towers_without_nexus(player.info.team)   // sret 120B Chain<Flatten<[Option<&Entity>;6]>, Copied<Iter<&Entity>>>
  L218~219: let nearest = towers.filter(closure#7 |t| t.can_target())            // can_target(0x6b9) && block_target_tick(0x6a0)==0 (aux m14.ll:58362~58370)
                           .min_by_key(closure#8 |t| t.distance_sq(champ))          // 인라인 min_by_key: 첫 통과 원소를 find(47914~48088)로 뽑아 dist²(48104~48132) 계산 → 나머지는 Map<Filter,..>::fold(48137, m12.ll:16938) 로 reduce
  L221 (48150): if let Some(tower) = nearest {                                  // null → L254
    L222 (48156~48184): let d2 = tower.distance_sq(champ)                        // |dx|²+|dy|² (u64, 0x660/0x668)
    L223 (48186): if d2 < 2500000001 {                                           // 50000² 이내. 아니면 → L254
      L224 (48191~48194): let tower_range = tower.attack_effect.as_ref()         // 0x4c0 == -1 → None
      L225 (48200~48240): .map(closure#9 |e| e.range + e.growth_range*(tower.level-1) + tower.stat_buff_cached.range + tower.radius())   // radius() = radius_mult==0 ? radius : radius*(radius_mult+100)/100
      L226 (48244): .unwrap_or(0)
      L227 (48248~48251): let enemies = &cache.player_champion[1 - team]         // [Option<&Entity>;5]
      L228~231 (48287~48401): let enemy_in_tower_range = enemies.iter().flatten().any(closure#10 |e|
            data.blackboard[1 - team].is_recent_visible(game, player, e)          // L231 앞 항(48362 호출은 무조건, 결과는 select)
            && e.distance_sq(tower) <= (tower_range + e.radius())²)               // L229~231 (48366~48386)
      L234 (48403~48412): let hp_ratio = champ.hp*100 / champ.stat_cached.hp     // 분모 0 → div_by_zero panic(Location 234:30)
      L235~237 (48463~48506): let enemy_count = enemies.iter().flatten().filter(closure#11 |e| blackboard[1-team].is_recent_visible(game, player, e)).count()
      L239~242 (48516~48575): self.pending_trace_events.push(PendingTraceEvent{ event: TowerEngage{ tower_distance: (d2 as f64).sqrt() as u64 /*L242 fptoui.sat*/, hp_ratio, enemy_count, enemy_in_tower_range }, tick: game.tick() /*L240*/ })
    } }

// ── [L254~256] 디버그 후보 덤프 ──
L254 (47223~47226): if data.context.debug(0x3b) {
  L255~256 (48623~48967): for (score, play) in with_score.iter() {   // 원소 192B: +0 score · +8 play · +0xb9 태그
      debug.infos.entry(champ.id).or_insert_with(Vec::new).push(format!("{:?}: {}", play.get_action(), score))   // get_action = small_action.rs:308~326 인라인(태그→game_core::SmallAction 변환표는 아래 ※)
  } }

// ── [L260~261] 후보 0 개 경고(디버그 게이트 없음 — stdout) ──
L260 (48582~48585): if with_score.len()==0 {
  L261 (48974~48985): println!("!no action candidates, team_plan: {:?}, plan: {:?}, sub_plan: {:?}", self.team_plan, self.plan, self.sub_plan) }

// ── [L269~290] 「음수 점수 + 아군 대상 캐스트」 후보 배제 ──
L269 (48590~48594 / 48993~48997): let bad = closure#12 |&(score, play)| {   // env = {game.data, game.vtable, champ}
    L270: score < 0 (`icmp sgt %1322, -1` 거짓)
    L271: && matches!(play, Skill|Skill2|Ult)  (태그 16..=18)
    L272~273: && game.get_entity_by_id(play.target /*play+8*/).is_some_and(|e| e.team == champ.team)  // TeamType eq 인라인(entity.rs:1127)
};
L277 (49025~49113): let any_bad = with_score.iter().any(closure#13 = bad 인라인);   // 루프 1319~1348, 통과 시 1350
L278 (49144): if !any_bad { with_score 그대로(%58 ← %70, 플래그 %1370=0) }
else {
  L280~281 (49135~49140): let filtered: bumpalo Vec<(i64,Play)> = with_score.iter().filter(closure#14 |x| !bad(x)).cloned().collect()   // aux m14.ll:58379~58462: bad 의 부정 (score≥0 · 비캐스트 · 대상 없음 · 타팀 → true)
  L282 (49151~49154): if filtered.is_empty() { L287: with_score 유지; L289: drop(filtered) }
  L288 (49177): else { with_score = filtered (플래그 %1368=1) }
}

// ── [L291] 최고점 ──
L291 (49192~49274): let max_score = with_score.iter().max_by_key(closure#15 |x| x.0).unwrap().0;   // 비어 있으면 unwrap_failed(Location 291:59). 인라인 max_by_key: 첫 원소를 초기값으로 fold(m12.ll:33083 — 동률은 뒤 원소, 점수만 쓰므로 무영향)

// ── [L300~307] v2 도주 대체 후보 ──
L300 (49275~49277): if version > 1 (%206 · E L29) && max_score < -8999999 {
  L301 (49312~49318): if !matches!(game.get_game_mode() /*vtable+0x40*/, DeathMatch /*태그 2*/) {
    L302 (49323~49329): let run = SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5 /*end_delay*/));   // 태그 3 store +177
    L303 (49331~49333): let score = self.sub_plan.score(version, &parameter, rnd, player, data, &run, debug);   // ★rnd 소비 가능(콜리 내부)
    L304 (49342~49344): if score > max_score {
      L305 (49352~49356): return (parameter, score, run);   // sret +0/+5384/+5392 → %1931(L416 에필로그)
    }
    L307 (49347): drop(run)  } }

// ── [L309~326] 최고점 후보 중 하나 선택 (rnd) ──
L309~310 (49288~49309): let max_score_actions: bumpalo Vec<SmallActionPlay> = with_score.iter().filter(closure#16 |x| x.0 == max_score).map(closure#17 |x| x.1.clone()).collect();
L313 (49363~49368): if ignore_action.len() == 0 {
  L314 (49374~49433): result = (max_score, max_score_actions.choose(rnd).unwrap().clone())   // ★rnd ①(비어 있으면 unwrap_failed 314:49 — L291 통과 시 불가)
} else {
  L316~319 (49386~49408): let without_ignore_action: bumpalo Vec<SmallActionPlay> = max_score_actions.iter()
        .filter(closure#18 |a| !ignore_action.iter().any(|(_, ia)| *ia == a.get_action()))   // aux m14.ll:58480~58773 (usize 첫 필드 무시 · SmallAction 태그/페이로드 완전 일치)
        .map(closure#19 clone).collect();
  L321 (49445~49448): if without_ignore_action.is_empty() {
    L322 (49453~49457): result = (max_score, max_score_actions.choose(rnd).unwrap().clone())   // ★rnd ② (unwrap_failed 322:39)
  } else {
    L324 (49462~49464): result = (max_score, without_ignore_action.choose(rnd).unwrap().clone())   // ★rnd ③ (unwrap_failed 324:43)
  }
  L326 (49491~49506): drop(without_ignore_action) }
// choose = len==0 ? None : &v[gen_range(0..len as u32)] — StdRng u32 워드 ≥1 소비(거부 루프, len==1 도 소비)

// ── [L329~339] 디버그 로그 (data.context.debug) ──
L329 (49438): if debug(%691) {
  L330 (49536~49542): if matches!(result.1, Skill|Skill2|Ult) {
    L331 (49557~49599): if game.get_entity_by_id(result.1.target).is_some_and(closure#20 |e| e.team == champ.team && e.is_champion() /*ty@tag==13*/) {
      L332~333 (49608~49817): debug.add_log(data, player, format!("\nBUFFPICK T{} {:?} act={:?} score={} sub={:?}", team, player.info.position, result.1.get_action(), result.0, self.sub_plan)) } }
  L337 (49551~49554): if result.0 < -8999999 {
    L338~339 (49826~50075): debug.add_log(data, player, format!("\rTBHOLD-PICK T{} {:?} act={:?} score={} sub={:?} n_cand={}", team, position, result.1.get_action(), result.0, self.sub_plan, with_score.len())) }
  → 배치 G(줄 343)  [%1701→%1679 · %1490→%1679]
} else → 배치 G(줄 356) [%1444→%1474]

// ※ get_action(small_action.rs:308~326) 인라인 변환표(3회 등장: L256·L333·L339 — 48762~48908 / 49614~49778 / 49831~50026): 스위치값 = tag>2 ? tag-3 : 7 →
//   RunAway·Recall·AroundRunAway → SmallAction::RunAway(0) · Around/AroundHide/LaneMinionPosition → Around{target=play+8}(2) · AroundRegion → AroundPosition{x=+16,y=+24}(3) · Positioning → Positioning{+8,+16}(1) · AroundPosition → AroundPosition{+48,+56}(3) · AroundPositionBush → AroundPosition{+8,+16}(3) · AroundBush → AroundPosition{+24,+32}(3) · Trace → Trace{target=+96}(4) · Attack → Attack{+8}(6) · Skill → Skill{+8}(7) · Skill2 → Skill2{+8}(8) · Ult → Ult{+8}(9) · Stop → Stop(10)
// 언와인드: 모든 invoke 는 %679/%1381/%1434/%1486(L416 cleanup) 로 — 판정 무관.

// auction.rs:342~416 (배치 G)
// 진입: 배치 F 의 L337 `%1490` (score < -8999999 거짓 엣지 → L343 %1679) · L338 `%1701` → %1679 · L329 `%1444`(%691 거짓) → L356 %1474 직행. 이 시점의 상태: %49 = (score: i64, action: SmallActionPlay) 192B 로컬(선택 완료) · %95 = ScoreParameter(배치 E) · %58 = bumpalo Vec<(i64, SmallActionPlay)> 채점 후보(배치 F) · %200 = champ(&Entity) · %454/%456 = data.cache.game 팻포인터 · %655 = data.context.pool.

// ── L342~352: 적 챔피언을 겨눈 액션이면 TBPICK 로그 ─────────────────────────
L342~343  let target_id = match &action.1 {            // switch on tag-3 (m13.ll 50002)
              Trace(t)            => t.target,          // 케이스 11 · %49+104 (Trace+0x60)
              Attack(a)|Skill(a)|Skill2(a)|Ult(a) => a.target,   // 케이스 12~15 · %49+16 (+0x8)
              _ => goto L356 };                          // RunAway·Recall·Around*·Positioning·LaneMinionPosition·Stop
L344      if let Some(t) = game.get_entity_by_id(target_id)      // 간접호출(vtable +0x1f0) · null=None → L356
              .filter(|t| t.team != champ.team              // TeamType: 판별자 다르면 ≠ · 둘 다 Player 면 +0x8 비교 · 둘 다 Neutral 이면 = (closure#21 · IR 순서: team 먼저, 다음 is_champion)
                       && matches!(t.ty, EntityType::Champion{..}))   // +0x68 == 13
          {
L345        let dive = if let Trace(t) = &action.1 { t.dive_ignore_tower_escape /* +0x94 */ } else { false };
L347        let s = format!("{:?}", self.sub_plan);       // anon.62 = "{:?}" · SubPlan Debug (self+0x768)
L348        let sub = s[..첫 ' ' 또는 '(' 의 위치(없으면 끝)].to_string();   // closure#22 `|c| c==' '||c=='('` · UTF-8 디코드 루프 후 `(ch & 0x1FFFF7)==32` · to_owned (try_allocate_in + memcpy)
L349        drop(s);
L350        debug.add_log(data, player, format!("TBPICK T{} {:?} act={:?} score={} dive={} sub={}",   // anon.143
                player.info.team(+0x930), player.info.position(+0x9c0), SmallAction::from(&action.1) /*L351 변환 표 ↓*/, action.0, dive, sub));
L352        drop(sub);
          }
// SmallActionPlay → SmallAction 변환(small_action.rs:309~326 인라인 · L351/L385/L407 세 곳 동일): RunAway|Recall|AroundRunAway → RunAway(0) · Positioning → Positioning{x:+0x8,y:+0x10}(1) · Around|AroundHide|LaneMinionPosition → Around{target:+0x8}(2) · AroundRegion → AroundPosition{+0x10,+0x18}(3) · AroundPosition → AroundPosition{+0x30,+0x38}(3) · AroundPositionBush → AroundPosition{+0x8,+0x10}(3) · AroundBush → AroundPosition{+0x18,+0x20}(3) · Trace → Trace{+0x60}(4) · Attack → Attack{+0x8}(6) · Skill → Skill(7) · Skill2 → Skill2(8) · Ult → Ult(9) · Stop → Stop(10). (SmallAction 태그 Direct 8B · Dodge(5) 는 생성 안 됨)

// ── L356~357: LaneMinionPosition 은 손대지 않는다 ─────────────────────────
L356      if action.1 은 LaneMinionPosition(태그 13) {
L357        return (score_param, action.0, action.1); }      // sret +0 memcpy 5384 · +0x1508 · +0x1510 memcpy 184

// ── L360: 액션 종류별 3갈래 ───────────────────────────────────────────────
L360      match &action.1 {
            Trace(t)  => { /* 갈래 A: L360~391 */ }                                              // 케이스 11
            Around|AroundHide|AroundRegion|AroundPosition|AroundPositionBush|AroundBush|LaneMinionPosition => { /* 갈래 B: L393~413 */ }   // 케이스 2,3,4,7,8,9,10 (LaneMinionPosition 은 L357 에서 이미 반환돼 실제 도달 불가)
            _ /* RunAway·Recall·AroundRunAway·Positioning·Attack·Skill·Skill2·Ult·Stop */ =>
L414          return (score_param, action.0, action.1) }                                          // 케이스 0,1,5,6,12,13,14,15,16 → %2211

// ── 갈래 A (Trace): 같은 대상을 겨눈 Attack/Skill/Skill2/Ult 후보로 갈아타기 ──
L360        let target = t.target;                          // %29 ← %49+104
L361        if matches!(self.sub_plan, SubPlan::Battle(..)) {          // self+0x768 == 7
L362          if let Some(t) = game.get_entity_by_id(target) {          // 간접호출 · None 이면 L370 으로
L363            let r = support_min_action_range(champ, t) + 25000;     // u64
L364            if dist2(t.pos(+0x660,+0x668), champ.pos) > r*r {       // |dx|²+|dy|² · ugt(엄격)
L365              return (score_param, action.0, action.1); } } }       // 사거리 밖: 그대로
L370~371    let cast: Vec<&(i64,SmallActionPlay)> = candidates(%58).iter()
                .filter(|a| match &a.1 { Attack(x)|Skill(x)|Skill2(x)|Ult(x) => x.target == target, _ => false })   // closure#23 sl_0 (aux m14:58776 · L372~375 4 arm)
                .collect_in(pool);
L378        if cast.is_empty() {                                        // %28+0x18 == 0
L379          return (score_param, action.0, action.1); }
L382        let max_score = cast.iter().max_by_key(|a| a.0).unwrap().0;   // closure#24 (aux m12:32331) · unwrap_failed anon.152 (비어있지 않으므로 도달 불가) · 동률이면 마지막 원소지만 점수만 쓴다
L383~384    let best: Vec<SmallActionPlay> = cast.iter().filter(|a| a.0 == max_score).map(|a| a.1.clone()).collect_in(pool);   // closure#25 sn_0 · closure#26 so_0 (aux m01:1469)
L385        if max_score >= 0                                                              // icmp sgt -1
               || (SmallAction::from(&action.1) == SmallAction::from(pre_action)          // blackboard.rs:81 PartialEq: 판별자 같고 Positioning/AroundPosition 은 (x,y) · Around/Trace/Attack/Skill/Skill2/Ult 는 target · RunAway/Dodge/Stop 은 무조건 같음
                   && pre_action.move_near_complete(champ)) {                            // IR 순서: 판별자 → 페이로드 → move_near_complete(단락)
L386          let picked = best.choose(rnd).unwrap().clone();           // ★rnd 소비 1회: gen_range<u32>(0..len) — len==1 이어도 소비(zone 거부 루프 ≈ 50% 재시도) · unwrap_failed anon.153 은 best 비어있을 때(도달 불가: max 원소는 반드시 포함)
L387          return (score_param, max_score, picked);                   // ★점수도 max_score 로 교체
L389        } else { return (score_param, action.0, action.1); }         // 음수 최고점 + 이전 액션과 다르거나 아직 도착 전 → 원래 Trace 유지
L391        // drop(best) · drop(cast) (bumpalo Vec 원소 drop) 

// ── 갈래 B (Around 계열): 대상 무관 Attack/Skill/Skill2 양수 후보로 갈아타기 ──
L393~394    let atk: Vec<&(i64,SmallActionPlay)> = candidates(%58).iter()
                .filter(|(score, act)| matches!(act, Attack|Skill|Skill2) && *score > 0)   // closure#27 sp_0 (aux m14:58989) · 태그-15 ult 3 · Ult 제외 · 대상 조건 없음
                .collect_in(pool);
L399        if atk.is_empty() {
L400          return (score_param, action.0, action.1); }
L403        let max_score = atk.iter().max_by_key(|a| a.0).unwrap().0;   // closure#28 sq_0 (aux m12:32436) · anon.154
L404~405    let best: Vec<SmallActionPlay> = atk.iter().filter(|a| a.0 == max_score).map(|a| a.1.clone()).collect_in(pool);   // closure#29 sr_0 · closure#30 ss_0 (aux m01:1653)
L407        if max_score >= 0 || (SmallAction::from(&action.1) == SmallAction::from(pre_action) && pre_action.move_near_complete(champ)) {   // L385 와 동일 구조(max_score>0 이 보장되므로 첫 항이 항상 참 → 뒤 항은 사실상 사장이나 IR 에는 남아 있음)
L408          let picked = best.choose(rnd).unwrap().clone();           // ★rnd 소비 1회 (L386 과 상호 배타)
L409          return (score_param, max_score, picked);
L411        } else { return (score_param, action.0, action.1); }
L413        // drop(best) · drop(atk)

// ── L416: 함수 끝 공통 드롭 ───────────────────────────────────────────────
// 반환 9곳 전부 → %51(SmallActionPlay 로컬, 배치 F) · %58(채점 후보 Vec) · %70(드롭플래그 %1370 시) · %80(드롭플래그 %513 시) 순서로 drop 후 ret void. 갈래 A/B 에서 clone 을 반환한 경로(L387/L409)는 %1475(원 action) 도 drop_glue(%2210) · 원 action 을 memcpy 로 반환한 경로는 move 라 drop 없음. 언와인드 cleanuppad(%108/%114/%157/%421/%448/%515/%634/%679/%1381/%1434/%1486/%2005/%2085/%2240/%2321) 는 같은 로컬들의 예외 경로 드롭.

// rnd: gen_range 사이트 = L386(갈래 A) · L408(갈래 B) 각 1회 · 상호 배타 · 조건부(후보 비었거나 L365/L389/L411 조기반환이면 0회). 그 외 본 범위에 rnd 접점 없음.
// 사장 코드: reach.txt(version=2 · gamemode=0) 사장 0 — NA 봉인 대상 없음.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `cb7540` → `f356b0` HideSubPlan::action_candidates (i=194 · 변경·심층(r21 §D: w8 §B))
- 0.5.8 src: `game-ai\src\plan_legacy\sub_plan\hide.rs:19` · one_line: [배치 L · hide.rs:0~101] Hide 서브플랜 행동 후보 루트: 적 시야 노출 갱신 → check_move 전엔 부시 위치 기준 Morgard/Serpen 캠프로 이동(AroundPosition) → check_move 후엔 부시 근처 최근접 적을 찾아 노출 시 아군/적 수·1v1 판정으로 전투(battle_action+Trace)/도주(RunAway), 비노출 시 부시 대기(AroundBush)+정글 스틸(Attack/Skill/Skill2). hide.rs:103~139 = `is_visible_to_enemy && !check_move` 분기(enemy_spotted_me 는 쓰기만 하고 읽지 않음)·반환
- 0.6.0 판정: **심층(r21)** · 패치 요지: L34/L107 `!(v3 && stealth)` 게이트 · **stealth 이동 블록**(region==bush → check_move · ee0dd0 경로 → AroundPosition::new_ex(out_line 1,r16000) · 폴백 ee1820 · Stop 0x12) · 후보 비면 Stop · direct/ambush_cell 미소비 · 생성자 = PassiveJungle 매복 1곳(구 캠프접근 사장 추정)
- RE 정본: `2026-09-17_r21_심층_w8_LineGanker_next_plan_Hide_action_candidates_원문.md` · 절: w8 §B
- sig: `fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>`
- consts: [{"value": 248, "src_line": 23, "meaning": "AbstractGame vtable 슬롯 +0xf8 = is_visible(team, id) (divtable). is_visible_to_enemy = game.is_visible(enemy_team, champ.id)", "kind": "미상", "ev": 3}, {"value": 71, "src_line": 35, "meaning": "game_core::BUSH_POSITIONS 길이(BUSH_NONZERO_COUNT, path_finder.rs:77/100) — 71개 (x,y) 부시 셀 상수 배열이 @anon.148(1136B) 로 인라인돼 L35/L58/L147 세 번 스택 복사(memcpy 20484/20818/21664)", "kind": "길이", "ev": 4}, {"value": 30, "src_line": 35, "meaning": "MapDef.bushes[30][30] 그리드 폭 = 배열 인덱스 상한 — x,y 각각 `icmp ult %idx, 30` bounds check(20547/20551 · 20881/20885 · 21727/21731), 실패 
- 0.5.8 logic 전문:
```
// hide.rs:0~101 (배치 L)
// 진입: 함수 머리. 인자 = (&mut self HideSubPlan16B, version, rnd, player, data, _parameter, _debug) -> Vec<SmallActionPlay>

// L20  res = Vec::new_in(data.context.pool)                                   // m02.ll:20378~20389
// L22  team = player.info.team(+0x930); assert team<2 (panic_bounds_check 20397)
//      champ = data.cache.player_champion[team][player.info.position@tag(+0x9c0)].unwrap()   // 20410~20423 · None→unwrap_failed 20446
// L23  is_visible_to_enemy = game.is_visible(enemy_team=1-team, champ.id)      // vtable+0xf8 · 20430~20442
// L26  if is_visible_to_enemy {
// L27      self.enemy_spotted_me = true;                                       // store i8 1 → self+0xa 20465
//      } else if self.check_move {                                             // 20455~20457 (표기: `else if` 인지 `else { if }` 인지 표기 불가 — 블록 구조는 이와 동치)
// L30      self.enemy_spotted_me = false;                                      // 20589
//      }
// L34  if !self.check_move {                                                   // 20467~20470 / 20457
// L35      let (x,y) = BUSH_POSITIONS.into_iter().filter(|&(x,y)| map.bushes[y][x] == self.bush).next().unwrap();
//                 // map = data.context.map(+0x20) · bushes @MapDef+0x1c98 [y][x] · 71개 상수 셀 @anon.148 · unwrap 실패 20597
// L37      bx = x*32000+16000; by = y*32000+16000;                             // 20607~20612 (by 는 −(…) 로 접힘)
// L39      if is_top_side(context, bx, by) {   // = (setting.height(+0x12c0) − by) >= bx (MIR map_regions.rs:22~24) · IR %147 = !is_top (20621) → 149(L48) / 151(L40)
// L40          (ex,ey) = map.camp_pos(JungleType::Morgard(4), blue_side = team==0)   // 20630
// L42          if champ.distance_sq(ex,ey) > 100000² {                         // 20726~20747 (dx,dy=|Δ| · dx²+dy² ugt 10000000000)
// L43              res.push(AroundPosition(SmallActionAroundPosition::new_with_out_line(rnd, data, ex, ey, 5, Outline(1))))   // 20757 · push 20778~20797
//              } else {
// L45              self.check_move = true;                                     // 20751
//              }
// L48      } else {
//              (ex,ey) = map.camp_pos(JungleType::Serpen(5), team==0)          // 20626
// L49          if champ.distance_sq(ex,ey) > 100000² {                         // 20642~20663
// L50              res.push(AroundPosition(new_with_out_line(rnd, data, ex, ey, 5, Outline)))   // 20673 · 20694~20713
//              } else {
// L52              self.check_move = true;                                     // 20667
//              }
//          }
//      }
// L57  if self.check_move {                                                    // 재로드 20578 (123) / 20583 (128: visible&&check_move) / 20591 (131: !visible&&check_move)
//      // ---- 거짓 분기: → 배치 M(줄 107) — 123→%211(107) · 128→%244(108/110, enemy_spotted_me=true 상수접힘) · 131→%243(136, enemy_spotted_me=false 접힘)
// L58      let (x,y) = BUSH_POSITIONS…filter(map.bushes[y][x]==self.bush).next().unwrap();   // 20811~20907 · 실패 21288
// L59      bx = x*32000+16000; by = y*32000+16000;                             // 21298~21305 (스택 %39/%38 — 클로저가 참조 캡처)
// L60      enemies = data.cache.player_champion[enemy_team]   (5칸 Option<&Entity>)   // 21306~21313
// L61      nearest_enemy: Option<&Entity> = iter_champions(enemies)
//              .filter(|e| data.blackboard[enemy_team].is_recent_visible(game, player, e)   // 21414 (aux m12.ll:7823 는 blackboard[1-player.info.team] 로 재계산)
// L62                   && e.distance_sq(bx,by) < 250000²+1)                    // 21430~21457 / aux 7839~7866
// L63          .min_by_key(|e| e.distance_sq(champ))                            // 첫 원소 key 21488~21508 → fold 21511(aux) · 동점 시 앞 원소 유지(aux 7920~7924)
//              // 결과 { i64 key, ptr } — ptr null = None (21515~21518)
// L66      if is_visible_to_enemy {                                            // 21519 / 21524
// L67          if let Some(enemy) = nearest_enemy {                            // 21535 (None → L91)
// L70              nearby_allies = iter_champions(player_champion[team]).filter(|e| e.id != champ.id && e.distance_sq(champ) < 150000²+1).count()   // closure$5(L69) · 5칸 unrolled 22596~22919 · 결과 %1035
// L73              nearby_enemies = iter_champions(player_champion[enemy_team]).filter(|e| e.is_visible_from(champ) && e.distance_sq(champ) < 150000²+1).count()
//                      // closure$6(L72) · is_visible_from 인라인(entity.rs:1481): champ.team Neutral(태그1)→항상 참(1043 경로 22973~22919) / Player(t)→ t<2 검사(23271, 실패 panic 23651) 후 e.visible_state[t]@tag==0(23309~23313) · 결과 %1332(23710)
// L75              dominated = nearby_allies < nearby_enemies                   // 23712 icmp ult
// L76              can_fight = !dominated && can1v1win(rnd, player, data, champ, enemy)   // 23714 (dominated 면 호출 생략) · 23717
// L78              if can_fight {                                              // 23728
// L80                  res.extend(battle_action(version, rnd, player, data, 5))   // 23732 · extend 23808
// L81                  res.push(Trace(SmallActionTrace::new(data, enemy.id, 5)))   // 인라인: t=game.get_entity_by_id(enemy.id)(23827) · start_tick=game.tick()(23835) · goal=(t.x,t.y) or (0,0)(23839~23860) · margin 15000 · end_delay 5 · path_finder/last_escape None · 태그 14 (23861~23881) · push 23903~23926
//                  } else {
// L84                  res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)))   // 23723 · 태그 3 23738 · push 23760~23784
// L86                  if !dominated {                                         // 23786
// L87                      res.extend(battle_action(version, rnd, player, data, 5))   // 23790 · 23797
//                      }
//                  }
//                  → 배치 M(줄 107) %1424
// L91          } else {   // nearest_enemy None
//                  res.push(RunAway(new_with_skill(data, player, 5, false)))     // 22970 · 태그 3 23933 · push 23955~23979 → 배치 M(107) %1424
//              }
// L93      } else {   // !is_visible_to_enemy
//              if let Some(enemy) = nearest_enemy {                            // 21532
// L94              res.push(AroundBush(SmallActionAroundBush::new_with_target(data, enemy, self.bush, self.out_line)))   // 21543 · 태그 12 21558 · push 21580~21604
//              } else {
// L96              res.push(AroundBush(SmallActionAroundBush::new_with_out_line(rnd, data, player, self.bush, self.out_line)))   // 21552 · 태그 12 22506 · push 22528~22552
//              }
// L101         res.extend(self.steal_jungle_action(rnd, player, data));        // 전부 인라인(hide.rs:142~, define 없음) · extend 23992 → 배치 M(107) %1428
//              // ===== steal_jungle_action(&self, rnd, player, data) 인라인 본체 (루트 줄 101, IR 21619~22502 + 23990~23993) =====
//              // L143 steal_res = Vec::new_in(bump); steal_range = 150000       // 21632~21639
//              // L144 champ = cache.player_champion[team][pos].unwrap()         // 21640~21643 (재로드 · 21756 unwrap_failed)
//              // L147 bush = BUSH_POSITIONS.into_iter().filter(|&(x,y)| map.bushes[y][x]==self.bush).next()   // closure#0 · 21660~21753
//              // L149     .map(|(x,y)| (x*32000+16000, y*32000+16000))         // closure#1 · 21767~21770
//              // L150     .unwrap_or((champ.x, champ.y)) → (bx,by)              // 21774~21808 (phi %505/%506)
//              // L156 for jungle in cache.jungles (Vec<&Entity> +0xd0 ptr / +0xe8 len) {   // 21821~21871 · 끝나면 %1427 extend
//              // L158     if !jungle.ty.is_jungle(enemy_team) { continue }     // ty@tag(+0x68)==4 && ty.Jungle.info.camp_type.0(+0x98)==enemy_team (21879~21888)
//              // L163     if jungle.distance_sq(bx,by) > steal_range² { continue }   // 21891~21913 ugt 22500000000
//              // L168     move_speed = champ.stat_cached.move_speed(+0x640)    // 21916
//              // L171     if champ.can_attack() {                                // 21918
//              // L172         if let Some(atk) = &champ.attack_effect {         // +0x4c0 != -1 (21934~21936)
//              // L173             expected_dmg = atk.expected_damage_target(context, champ as &dyn AbstractEntity, jungle)   // 21942 (vtable @anon.54)
//              // L174             range = atk.range(champ) + atk.range_adjust(champ, jungle) + champ.radius() + jungle.radius()
//              //                      // Effect::range 인라인 = range(+0x4a0) + growth(+0x4a8)*(level(+0x5c8)−1) + stat_buff_cached.range(+0x438) (21947~21957) · range_adjust 21952 · radius 인라인 21958~22002
//              // L175             dist_sq = jungle.distance_sq(champ)             // 22005~22030 (champ.x/y, 부시 아님)
//              // L176             max_dist = range + move_speed*20                // 22031~22038
//              // L179             if jungle.hp(+0x670) <= expected_dmg && dist_sq <= max_dist² {   // 22039~22045 (select = 양쪽 다 계산, 소스 순서 표기 불가)
//              // L180                 steal_res.push(Attack(SmallActionAttack::new(data, jungle.id)))   // 22052 · 태그 15 22057 · push 22079~22105
//              //                  }
//              //              }
//              //          }
//              // L186     if champ.can_skill() {                                 // 21929
//              // L187         if let Some(skill) = &champ.skill_effect {        // +0x4f8 != -1 (22122~22124)
//              // L188             if skill.target.check(champ, jungle) {         // CastingTarget::check(+0x4f0, champ, jungle) 22130
//              // L189                 expected_dmg = skill.expected_damage_target(context, champ, jungle)   // 22138
//              // L190                 range = skill.range(champ) + skill.range_adjust(champ, jungle) + champ.radius() + jungle.radius()   // 22143~22198
//              // L191                 dist_sq = jungle.distance_sq(champ)         // 22201~22226
//              // L192                 max_dist = range + move_speed*20            // 22227~22234
//              // L194                 if jungle.hp <= expected_dmg && dist_sq <= max_dist² {   // 22235~22241
//              // L195                     steal_res.push(Skill(SmallActionSkill::new(data, jungle.id)))   // 22248 · 태그 16 22253 · push 22275~22300
//              //                      }
//              //                  }
//              //              }
//              //          }
//              // L202     if champ.can_skill2() {                                // 22117
//              // L203         if let Some(skill2) = champ.skill2_effect() {     // 인라인(entity.rs:1692): level>2 ? &skill2_effect(+0x500) : &NONE · tag +0x530 != -1 (22315~22322)
//              // L204             if skill2.target.check(champ, jungle) {        // +0x528 · 22329
//              // L205                 expected_dmg = skill2.expected_damage_target(context, champ, jungle)   // 22337
//              // L206                 range = skill2.range(champ) + range_adjust + champ.radius() + jungle.radius()   // 22342~22398
//              // L207                 dist_sq = jungle.distance_sq(champ)         // ~22426
//              // L208                 max_dist = range + move_speed*20            // 22427~22434
//              // L210                 if jungle.hp <= expected_dmg && dist_sq <= max_dist² {   // 22435~22441
//              // L211                     steal_res.push(Skill2(SmallActionSkill2::new(data, jungle.id)))   // 22448 · 태그 17 22453 · push 22475~22499
//              //                      }
//              //                  }
//              //              }
//              //          }
//              //      }   // loop 22312 → 21857
//              // L219 (steal_res 는 언와인드 시 drop_glue 21610 · 정상 경로엔 extend 로 소유권 이전)
//              // ===== steal_jungle_action 끝 =====
//          }
//      } else { → 배치 M (hide.rs:107~) }
// 이후 반환(L136~139: res → sret memcpy · 언와인드 cleanup %57 은 L139 drop_glue) = 배치 M

// 사장 코드: reach.py(version=2 · gamemode=0) 결과 블록 351 전부 live · NA 봉인 대상 0

// hide.rs:103~139 (배치 M)
// 승계(배치 L 정의, IR 로 재확인): team=%54=player+0x930 · champ=%68=data.cache.player_champion[team][pos].unwrap() · is_visible_to_enemy=%79=data.cache.game.vtable[0xf8](AbstractGame::is_visible)(1-team, champ.id) (hide.rs:23) · %74 = 1-team(적 팀) · res=%46 빈 bumpalo Vec(ptr=8,bump=data.context.bump,cap=0,len=0)

// 진입: 배치 L 의 line 57 분기(%123 check_move==0 → %211 / %128·%131 → 직접) 와 line 58~101 블록 끝(%1349·%1359·%1404·%1419 → %1424) · line 101 끝(%1427 → %1428 → line 136)

// hide.rs:107  if is_visible_to_enemy && !self.check_move {        // 두 조건 모두 필요. %211: br %79 → %244/%243 · %1424: check_move 재로드(IR 23986) → 1 이면 %243(136), 0 이면 %244(108)
//   근거: CFG 에서 %79=0 으로 접으면 %1424·%244 도달 불가(cfg79.py BFS: 156블록 중 없음) ⟹ %1424 경로는 %79=1 이 정적으로 확정된 경로(jump-threading). 두 조건의 소스 표기 순서는 표기 불가(column 없음).

// hide.rs:108  data.iter_champions(1-team)                          // %245 = &player_champion[1-team] (5 x Option<&Entity>)
// hide.rs:109    .filter(|x| x.is_visible_from(champ))              // closure#7 인라인: champ.team==Neutral(tag 1) → true / Player(t): t<2 bounds(아니면 panic_bounds_check 2) → x.visible_state[t].tag==0(Visible)
// hide.rs:110    .min_by_key(|x| x.distance_sq(champ))              // closure#8: |x.x-champ.x|²+|x.y-champ.y|² (utils.rs:2158). 슬롯 0~4 를 순서대로 훑어 필터 통과 첫 원소를 %328 에서 잡고(first=(dist²,ptr)), 잔여는 aux fold(m12.ll:12641)로: 같은 필터·같은 키·min_by(cmp(acc,new)>Greater 일 때만 교체 = 동률이면 앞 원소 유지). 5슬롯 전부 실패 → %327: nearest_enemy=None
//   let nearest_enemy: Option<&Entity>

// hide.rs:112  if let Some(enemy) = nearest_enemy {                 // %1429: null 검사 → None 이면 %1587(line 132)

// hide.rs:114/115  let nearby_allies = data.iter_champions(team).filter(|x| x.id != champ.id && x.distance_sq(champ) < 22500000001).count();
//     // %66 = player_champion[team] 5슬롯 완전 언롤(%1432~%1580). null skip · id==champ.id 는 0 · 아니면 zext(dist² ult 150000²+1) 누적. ⚠거리 기준점은 enemy 가 아니라 **champ 자신**(%1434/%1435 = champ.x/y 재로드 IR 24043~24044)

// hide.rs:117/118  let nearby_enemies = data.iter_champions(1-team).filter(|x| x.is_visible_from(champ) && x.distance_sq(champ) < 22500000001).count();
//     // %245 5슬롯 완전 언롤. champ.team 으로 루프 언스위치(IR 24410~24415 freeze): Neutral → %1588 경로(가시 검사 생략) / Player(t) → %1705: t<2 이면 %1710 경로(visible_state[t].tag==0 && dist²) / t>=2 이면 %1709 경로(첫 non-null 슬롯에서 panic_bounds_check(t,2))

// hide.rs:120  let dominated = nearby_allies < nearby_enemies;       // icmp ult %1581,%1877 (IR 25162, samesign)
// hide.rs:121  let can_fight = !dominated && can1v1win(rnd, player, data, champ, enemy);   // dominated 면 can1v1win 호출 자체를 안 함(IR 25164 → %1881). DI: can_fight=%1880 는 %1882(호출 뒤)에서만

// hide.rs:123  if can_fight {
// hide.rs:124      res.extend(battle_action(version, rnd, player, data, 5));      // sret %26 → extend(res, vec.ptr, vec.len) → %1909 → line 136
//              } else {
// hide.rs:126      res.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)));   // sret %24 136B → %25 memcpy · tag 3 @+0xb1 · len==cap 이면 reserve_internal_or_panic(res,len,1,true) · ptr[len]=184B memcpy · len+=1
// hide.rs:127      if !dominated {                                                // IR 25236: br %1878 → %243(136) / %1899(128). 즉 「1v1 승산 없음이지만 수적 열세는 아님」일 때만
// hide.rs:128          res.extend(battle_action(version, rnd, player, data, 5));  // sret %23
//                  }
//              }
//   → line 136
// } else {   // nearest_enemy == None (%1587)
// hide.rs:132      res.push(RunAway(new_with_skill(data, player, 5, false)));    // sret %21 → %22 · tag 3 · push 동일 수순(IR 25266~25314)
// hide.rs:133      res.extend(battle_action(version, rnd, player, data, 5));      // sret %20 (IR 25317~25325)
// }
// }  // end if 107

// hide.rs:136  res.extend(attack_summon_action(player, data));       // %243: sret %19 → extend(IR 25335). 107 이 거짓인 경로(%211·%1424·%1428) 도 전부 여기로 합류
// hide.rs:138  res                                                  // memcpy 32B → sret %0 (IR 25340)
// hide.rs:139  }                                                    // ret (IR 25342). 언와인드 %57: drop_glue(bumpalo Vec<SmallActionPlay>)(res) 뒤 caller 로 (IR 20402)

// 배치 M 이 push 하는 variant 집합 = {RunAway(태그 3)} 2사이트(126·132), 생성자 인자 (data, player, end_delay=5, with_skill=false) 동일. 나머지 원소는 battle_action ×3 · attack_summon_action ×1 의 반환 Vec 을 extend.
// 배치 M 범위 내 self(%1) 쓰기: 없음. TLS(LocalKey::with) 접점: 없음(함수 전체 0건, 콜리 can1v1win/battle_action/attack_summon_action 내부는 그 명세 소관).
// 사장 코드: reach.txt 사장 호출부 0 — NA 봉인 대상 없음.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `db9430` → `faa7c0` LineGankerPlan::next_plan (i=202 · 변경·심층(r21 §D: w8 §A))
- 0.5.8 src: `game-ai\src\plan_legacy\old\line_gank\ganker.rs:99` · one_line: 라인 갱커 BigPlan 전이 판정: 노출(적에게 보임+정글 영역) 시 광역 인원우위(아군>적 && 적≠0)면 최근접 가시 적에 make_gank_battle, 아니면(열세·동수·근접 적 0 포함) 가시 적이 하나라도 있으면 즉시 Battle(Response, entry_src 3)+BattleHelp 챗 → 목표 부시 도착 여부 → 근처 적별 킬가능(die_tick)·타워복귀시간·인원비 게이트로 make_gank_battle → check_kill → 사거리 내 최근접 적
- 0.6.0 판정: **심층(r21)** · 패치 요지: 3분기(open/**response**(BattlePlan 프로브 → screened 폐기 · BattleHelp)/None) · L117 내 라인 존 적 제외(lane_zone 19903f0 · own_line +0xac · exposed +0xa7 — LPH [TRAIT_ROAM] 강제 생성만 세팅) · arrived_tick · make_gank_battle fa9d60 **site 인자(1~7)** · version 분기 0
- RE 정본: `2026-09-17_r21_심층_w8_LineGanker_next_plan_Hide_action_candidates_원문.md` · 절: w8 §A
- sig: `fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types`
- consts: [{"value": 6, "src_line": 101, "meaning": "LineGankerPhase 니치 시작(WaitResponse=6/Setup=7/Cancel=8; tag<6 = ChangeJungle 암묵 variant, tcxdict --enum LineGankerPhase). `icmp ult i8 tag, 6` → ChangeJungle 이면 None(95054). ⚠같은 값 6 이 L135 Chat::BattleHelp 메모리태그로도 쓰인다(97021, tcxdict --enum Chat 6) · 오라클 실행 확증(25차 배치I: 오라클 실행 확인: A1(ChangeJungle 태그 0<6 → None) / A2(Setup 태그 7 → 진행) (o202 · 프로세스 분리 · run202_all.txt))", "kind": "센티널", "ev": 2}, {"value": 2, "src_line": 106, "meaning": "team 배열 bounds(<2, 95066) · L136 BattlePlanGoal::Response 태그 2(97029, tcxdict --enum BattlePlanGoal idx2→2, Direct) · 오라클
- 0.5.8 logic 전문:
```
// ganker.rs:99~246 (fn 루트 99). reach.py(version=2·gamemode=0): 블록 258 전부 살아있음 · 사장 0 · 접힌 분기 0 — NA 봉인 없음.
// 극성은 분기 방향으로 읽음. 인라인 헬퍼는 `<파일:줄>` 로 표시.
//
// L101  if let LineGankerPhase::ChangeJungle(_) = self.phase { return None }      // tag(+0x29) < 6 (95052~95055) → L102 store -1
// L106  let team = player.info.team;  let pos = player.info.position as usize;    // bounds<2
//       let champ = data.cache.player_champion[team][pos].unwrap();            // 95083~95086, unwrap_failed 95118
// L107  if game.is_visible(1 - team, champ.id)                                  // vtable +0xf8, 적팀에 내가 보이는가 (95112~95115)
//        && is_in_jungle_area(context, champ.x, champ.y) {                      // 95540~95547 (`&&`: 둘째는 첫째 참일 때만 호출)
//   L113  let near_wide_allies_len = cache.player_champion[team].iter().flatten()      // 5슬롯 언롤(자기 자신 포함)
//            .filter(|x| x.distance2(champ) < 62500000001                            // L111 ≤250000
//                     && x.hp*100 / x.stat_cached.hp > 39).count();                  // L112 HP≥40% (max_hp 0 → div0 패닉)
//   L115  let near_wide_enemies_len = cache.player_champion[1-team].iter().flatten()
//   L117     .filter(|x| data.blackboard[1-team].is_recent_visible(game, player, x)    // ★블랙보드 인덱스 = 1-team (96020)
//   L118            && x.distance2(champ) < 62500000001
//   L119            && x.hp*100 / x.stat_cached.hp > 39).count();                     // L120
//   L122  if near_wide_allies_len > near_wide_enemies_len && near_wide_enemies_len != 0 {   // 96437~96440
//   L124    let nearest_enemy = cache.iter_champions(1-team)                           // 적 5슬롯 언롤 + m12 14487 fold
//                .filter(|x| blackboard[1-team].is_recent_visible(game, player, x))    // closure#2
//                .min_by_key(|x| x.distance2(champ));                                  // closure#3 (첫 최소 유지)
//   L126    if let Some(e) = nearest_enemy {
//   L127      if let Some(p) = self.make_gank_battle(version, rnd, player, data, positioning_score, team_plan, e.id, 60, debug) { return Some(p) }   // 97043~97046, None 이면 L142 로
//           }                                                                            // None 이어도 L142 로(96929)
//         } else {
//   L132    let nearest_enemy = cache.iter_champions(1-team).filter(is_recent_visible).min_by_key(dist² to champ);   // closure#4/#5, 동일 fold
//   L134    if let Some(e) = nearest_enemy {
//   L135      self.chats.push(Chat::BattleHelp(e.id, 0));                                 // 태그 6, 96987~97027
//   L136      let mut b = BattlePlan::new(version, BattlePlanGoal::Response, data, player); b.entry_src = 3;
//             return Some(BigPlan::Battle(b));                                            // 태그 9 + memcpy 280B
//           }                                                                            // None 이면 L142 로(96658)
//         }
//       }
// L142  let _ = self.target_bush(player, data);          // ★인라인(ganker.rs:351~377). 값은 어디에도 안 쓰임(%18 재로드 0건) — unwrap 패닉 부작용만 남음:
//         //  352 champ = player_champion[team][pos].unwrap()  (재로드 95129, unwrap 95280)
//         //  353 optb = tower[line][team].or(tower2[line][team])                          (+0x180+line*32 / +0x190+line*32, 95142~95152)
//         //  354 t = cache.twin_towers[team].iter().min_by_key(|t| t.distance2(self.line.get_start_position(context.setting, team)))   (95239~95272)
//         //  359 nearest_tower = optb.or(t)  (closure#1: 95287~95302)
//         //  360 nearest_tower = nearest_tower.unwrap_or(cache.nexus[team].unwrap())   (361 unwrap 95318)
//         //  363 nexus = cache.nexus[team].unwrap()                                    (95335)
//         //  365 ms = blackboard[team].minion_state(self.line)  (<blackboard.rs:379~382> switch line → +0/+0x28/+0x50, 95346~95361)
//         //  366 front_minion = ms.front_minion.and_then(|m| game.get_entity_by_id(m))  (closure#2, vtable +0x1f0, 95389~95391)
//         //  368 reference = front_minion.filter(|x| x.distance2(nexus) >= nearest_tower.distance2(nexus))  (closure#3; 95463 `ult` 참이면 버림)
//         //        Some(x) → 373 near_jungle_bush(context, champ.x, champ.y, x.x, x.y)
//         //        None    → 369 center = context.map.lane_path(self.line, team)[3];  370 near_jungle_bush(context, champ.x, champ.y, center.x, center.y)
//         //  374 .unwrap()   (Option<(u64,u64)> 태그만 검사 95495~95500)
// L144  let bush = self.target_bush_v41(player, data);                                   // fastcc(line, team, pos, cache, context) → usize (95508~95510)
// L148  let champ_bush = context.map.bushes[min(champ.y/32000, 29)][min(champ.x/32000, 29)];   // 95513~95533 ([y][x] 순)
// L150  if context.debug {  L151 debug.add_text(champ.x, champ.y + 20000, format!("bush: {:?}, champ_bush: {:?}", bush, champ_bush), Color(1,0,0,1), 1) }
// L154  if champ_bush != bush { L155 return None }                                       // 97049~97052 (eq → 진행)
// L158  let dist_cut: u64 = 150000;
// L160  let near_allies_p: bumpalo Vec<usize> = cache.player_champion[team].iter().enumerate()
//           .filter(|(_, x)| x.is_some_and(|x| x.distance2(champ) <= dist_cut*dist_cut))   // closure#6 (m08 112913), 자기 자신 포함(거리 0)
// L163      .map(|(i, _)| i).collect_in(context.pool);                                      // 97106~97116 (bump = context+0 %803)
// L165  let near_enemies_p: bumpalo Vec<usize> = cache.player_champion[1-team].iter().enumerate()
//           .filter(|(_, x)| x.is_some_and(|c| blackboard[1-team].is_recent_visible(game, player, c) && c.distance2(champ) <= dist_cut²))   // closure#8 (112983)
// L168      .map(|(i, _)| i).collect_in(pool);                                              // 97137~97155
// L170  let nearest_enemy_tower = cache.iter_towers_without_nexus(1-team)                   // 97172 (120B 이터레이터)
// L171      .min_by_key(|t| t.distance2(champ));                                             // closure#10 (m12 39469) · fold m06 23823 / m11 17703 · 97189~97372
// L173  for e in near_enemies_p.iter() {                                                    // 97430~97440 (ptr 순회, e: &usize)
//   L175  let near_allies_p = cache.player_champion[team].iter().enumerate()                // 루프 지역 재바인딩(%32)
//             .filter(|(_, x)| x.is_some_and(|c| c.id == champ.id || c.distance2(player_champion[1-team][*e].unwrap()) <= dist_cut²))   // closure#11 (113084)
//   L180      .map(|(i,_)| i).collect_in(pool);
//   L182  let near_enemies_p = cache.player_champion[1-team].iter().enumerate()             // (%30)
//             .filter(|(_, x)| x.is_some_and(|x| x.distance2(player_champion[1-team][*e].unwrap()) <= dist_cut²))   // closure#13 (113214)
//   L185      .map(|(i,_)| i).collect_in(pool);
//   L187  let enemy_die_tick = get_die_tick_player(data, 1-team, *e, &near_allies_p, None, None);     // 97511~97516 (Option<SmallAction>::None = -1)
//   L188  let me_die_tick    = get_die_tick_player(data, team, pos, &near_enemies_p, None, None);     // 97528~97532
//   L190  let e_ent = cache.player_champion[1-team][*e].unwrap();                          // bounds<5 97533, unwrap 97553
//   L191  let move_to_tower_tick = if let Some(tower) = nearest_enemy_tower {
//   L192~194   e_ent.distance(tower) / e_ent.stat_cached.move_speed                        // 97558, 97563~97570 (speed 0 → div0 패닉)
//           } else { 99999999 };                                                             // 97578
//   L199  if context.debug { L200 debug.infos.entry(e_ent.id).or_default().push(format!("target: {:?}, near_allies_p: {:?}, near_enemies_p: {:?}, move_to_tower_tick: {}, me_die_tick: {}, enemy_die_tick: {}", e_ent.id, near_allies_p, near_enemies_p, move_to_tower_tick, me_die_tick, enemy_die_tick)) }
//   L205  if enemy_die_tick > 30 && near_allies_p.len() <= near_enemies_p.len() { continue }   // 97583~97585 + 97744~97748 (ugt 30 참 & !(allies>enemies) → 1001 drop → 다음 e)
//   L209  if move_to_tower_tick <= 120 {                                                   // ult 121 (97739)
//   L211    if enemy_die_tick <= 60 {                                                       // ult 61 (97778)
//   L212      if let Some(p) = self.make_gank_battle(…, e_ent.id, 60, debug) { return Some(p) }   // 97887~97908
//   L213    } else if near_allies_p.len() > near_enemies_p.len() {                          // 97882~97886
//   L214      if let Some(p) = self.make_gank_battle(…, e_ent.id, 60, debug) { return Some(p) }   // 97894~97903
//           }
//   L216  } else if me_die_tick > 120 && near_allies_p.len() >= near_enemies_p.len() {      // 97773~97775 · 97790~97794(`allies < enemies` 참이면 다음 else-if 로)
//   L217    if let Some(p) = self.make_gank_battle(…, e_ent.id, 60, debug) { return Some(p) }   // 97797~97834
//   L218  } else if self.wait_limit.saturating_sub(game.tick())                            // vtable +0x28 · usub.sat 97804
//   L219             > context.setting.tick_per_second * 3 {                                // 97805~97809 → 참이면 아무것도 안 함(1047: drop → 다음 e)
//   L220  } else if near_allies_p.len() > near_enemies_p.len() {                            // 97814~97818
//   L221    if let Some(p) = self.make_gank_battle(…, e_ent.id, 60, debug) { return Some(p) }   // 97820~97829
//         }
// L223  }  // 루프 지역 Vec 2개 drop 후 다음 e
// L225  if let Some((e, t)) = check_kill(version, rnd, player, data, positioning_score, debug) {   // 97474 · Option<(usize,usize)> +8 e, +0x10 t (98175~98179)
// L226    if near_allies_p.len() > near_enemies_p.len() {                                   // 바깥 Vec(%37 vs %35) 98006~98011
// L227      if let Some(p) = self.make_gank_battle(…, e, t, debug) { return Some(p) }        // ★min_trace = check_kill 의 t (98181~98187)
//         }
//       }
// L232  let nearest_enemy = cache.iter_champions(1-team).filter(|x| blackboard[1-team].is_recent_visible(game, player, x))   // closure#15, 98081~98121 언롤
// L233      .min_by_key(|x| x.distance2(champ));                                             // closure#16, fold m12 14907
// L235  if let Some(ne) = nearest_enemy {                                                    // 98197~98198
// L236    let d = ne.distance(champ);                                                        // 98203 (Entity::distance, 제곱 아님)
// L237    let attack_range = champ.attack_effect.as_ref().unwrap().range(champ)             // <effect.rs:26> = champ.stat_buff_cached.range + eff.range + (champ.level-1)*eff.growth_range (98238~98257, 98304~98305)
//                          + eff.range_adjust(champ, ne)                                     // 98247 (게임 경계)
//                          + champ.radius() + ne.radius();                                    // <entity.rs:1511~1515>: radius_mult==0 ? radius : radius*(mult+100)/100 (98258~98303)
// L240    if d > attack_range { return None }                                                // 98310~98311 (ugt → 1152 None)
// L241    return self.make_gank_battle(…, ne.id, 60, debug);                                 // 98314~98322 — 콜리 결과가 None(-1)이어도 그대로 None 반환
//       }
// L245  None                                                                                // 98207
// L246  } // 바깥 near_allies_p/near_enemies_p drop
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e5d5d0` → `d606b0` LegacyPlanHandler::handle_interact_battle (i=110 · 변경·심층(r21 §D: w5 §4))
- 0.5.8 src: `game-ai\src\plan_legacy\handler\engage.rs:233` · one_line: 매 틱 교전 개시 판정 — 오브젝트(모가드/세르펜) 교전·아군 교전 합류(Support)·피격 응전(Response)·라인 다이브·정글캠프 교전·넥서스 공/방 교전·솔로킬 순으로 검사해 self.plan 을 BigPlan::Battle 로 바꾼다
- 0.6.0 판정: **심층(r21)** · 패치 요지: 신규 프롤로그(flee_latch/latch_v3 · pending tower_binary +0x1ef8 동기·유효성·다이브 콜 · one_shot d601f0) · §C 합류 대폭(gank=bb Vec · joiner(FightJoiner/합류창/정글) · range 450000/250000 · 트리거 1→2→3 · v3 게이트 A/bb · commit=line≠Disengage · reject 기록) · §A aggr=+0x230≥70‖특성 · 유효 objective 인라인 · §D isolated/chased · solokill Gank 조건 삭제
- RE 정본: `2026-09-17_r21_심층_w5_handle_interact_battle_원문.md` · 절: w5 §4
- sig: `fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)`
- consts: [{"value": 4, "src_line": 241, "meaning": "JungleType::Morgard 태그(camp_pos 인자 i8 4). L470·474 도 동일", "kind": "태그", "ev": 4}, {"value": 5, "src_line": 339, "meaning": "JungleType::Serpen 태그(camp_pos 인자 i8 5). L478·482 도 동일", "kind": "태그", "ev": 4}, {"value": 19600000001, "src_line": 242, "meaning": "140000²+1 — `ult` 로 dist_sq <= 140000² (캠프 반경 140k). L340 in_camp_ally 도 동일", "kind": "임계", "ev": 4}, {"value": 19600000000, "src_line": 291, "meaning": "140000² — champ_is_in_camp = !(dist_sq > 140000²). L355·470·478 도 동일", "kind": "임계", "ev": 4}, {"value": 150000, "src_line": 250, "meaning": "can_
- 0.5.8 logic 전문:
```
// ===== engage.rs:233~235 진입 =====
// (234) let champ = data.cache.champions[player.info.team][player.info.position.as_index()].unwrap();   // team ult 2 바운드체크 · None 이면 unwrap_failed
// (235) let _t_ob = ProfTimer::start(86)  // prof::ENABLED 원자 로드 — 계측, 판정 무관. 함수 끝(859)에서 drop 시 PHASE_NANOS[phase]/PHASE_CALLS 누적

// ===== §A engage.rs:238~333  모가드 with_battle 오브젝트 교전 =====
// (238) if let Some(MainObjective::Morgard{phase, with_battle}) = &self.team_plan.objective  [self+0x517 == 0]
// (239)   if *with_battle [0x519] && !matches!(self.plan, BigPlan::Battle) [0x5e8 != 9] {
// (241)     camp_pos = ctx.map.camp_pos(JungleType::Morgard(4), team == 0)
// (242)     in_camp_ally = cache.iter_champions(team).filter(|x| x.pos.distance_sq(camp_pos) <= 140000²).count()      // closure#0 인라인, 5회 언롤
// (244~247) in_camp_enemy_list: bumpalo Vec<&Entity> = iter_champions(1-team).filter(closure#1).collect_in(ctx.pool)
//             closure#1(x) = x.is_visible_from(champ.team) && !is_ignored_well_enemy(version, player, x) && x.pos.distance_sq(camp_pos) <= 140000²   // aux m12.ll 41648~41735
// (248)     in_camp_enemy = in_camp_enemy_list.len()
// (250)     can_near_enemies = self.team_plan.can_near_enemies_range(version, rnd, player, data, camp_pos.x, camp_pos.y, 150000, debug).len()   // bumpalo Vec 반환 후 즉시 drop
// (252)     let mut skip = false;
// (255)     let strategy = player.strategy(rnd, cache.game);
//           if strategy.object_finish [+0xf] == <태그 0> {
// (256)       steal_risk = cache.as_moba().unwrap().jungle_runner.epic.live_list.get(0)        // len==0 → None
// (257)                     .and_then(|e| cache.get_entity_by_id(e.id))                          // closure#4 (vtable+0x1f0)
// (258~259)                .is_some_and(|epic| iter_champions(1-team).any(|c| c.is_visible_from(champ.team) && !is_ignored_well_enemy(version,player,c) && can_enemy_hit_objective(c, epic, 25000)))   // closure#3 = aux m13.ll 5709~6089
// (260)       if !steal_risk { skip = true }
//           }
// (265)     if *phase == ObjectPhase::Hunt(3) {
// (266)       remain_time = self.data.epic.epic_ally_tick [self+0x98]
// (267~268)   if let Some(epic) = as_moba().unwrap().epic.live_list.get(0).and_then(|e| cache.get_entity_by_id(e.id)) {   // closure#4
// (270~276)     min_tick: Option<usize> = iter_champions(1-team).filter(closure#5{champ,version,player,epic}).map(closure#6).min()   // aux: m12.ll 10615~10796(fold) / m13.ll 4210~4337(reduce) — 내용은 unknown 참조
// (279)         if let Some(min_tick) = min_tick { if min_tick > remain_time { skip = true } }   // IR: skip = skip || (tag && min_tick ugt remain_time)
//             }
//           }
// (288)     if skip { → 블록 탈출(§B 로) }
// (291)     champ_is_in_camp = champ.pos.distance_sq(camp_pos) <= 140000²   (DW_OP_not 확인: 값은 !(dist>140000²))
// (292)     if in_camp_ally <= can_near_enemies + in_camp_enemy || !champ_is_in_camp { → 탈출 }
// (293)     nearest_enemy = in_camp_enemy_list.iter().copied().min_by_key(|e| e.pos.distance_sq(champ.pos))   // closure#7, 동률이면 앞 원소 (aux m11.ll 16958~17104)
// (294)     let Some(nearest_enemy) = nearest_enemy else { 탈출 }
// (295)     mr = max_range_cached(data, champ, nearest_enemy)
// (297)     if nearest_enemy.pos.distance_sq(champ.pos) > mr*mr { 탈출 }
// (298)     formation_ok = check_favorable_engage_formation(version, player, data, nearest_enemy, 200000)
// (301)     strategy = player.strategy(rnd, cache.game)   // 재호출(rnd 소비 2회째)
// (303)     aggressive = player.info.parameter.aggressive_ratio()
// (305~308) near_ally = iter_champions(team).filter(closure#8{data, nearest_enemy, champ}).count()   // aux m13.ll 59321~59489 — 내용은 unknown 참조
// (309)     if strategy.object_battle [+0xc] == <태그 0> {
// (311)       min_allies_poke = if aggressive > 699 {2} else {3}
// (312)       if formation_ok && can_near_enemies == 0 && near_ally >= min_allies_poke {
// (313)         if let Some(b) = self.try_engage(version, rnd, player, data, nearest_enemy.id, debug) {
// (314)           self.plan = BigPlan::Battle(b);   (317) return;
//               }
//             }
//           } else {
// (320)       min_allies_engage = if aggressive > 699 {1} else {2}
// (321)       if formation_ok && near_ally >= min_allies_engage {
// (322~326)     if let Some(b) = self.try_engage(…, nearest_enemy.id, debug) { self.plan = Battle(b); return; }
//             }
//           }
// (333)     drop(in_camp_enemy_list)   // bumpalo Vec drop_glue
//   }
// }

// ===== §B engage.rs:336~386  세르펜 with_battle 오브젝트 교전 (§A 와 동형, 차이만) =====
// (336) if let Some(MainObjective::Serpen{phase:_, with_battle}) = objective [0x517 == 1] && *with_battle [0x519]
// (337)   && !matches!(self.plan, Battle) {
// (339)   camp_pos = ctx.map.camp_pos(JungleType::Serpen(5), team==0)
// (340)   in_camp_ally = iter_champions(team).filter(dist_sq(camp_pos) <= 140000²).count()      // closure#9 인라인
// (341~344) in_camp_enemy_list = iter_champions(1-team).filter(closure#10).collect_in(pool)     // closure#10 = aux m12.ll 41146~41233 (내용 §A closure#1 과 동형인지 = unknown 참조)
// (345)   in_camp_enemy = len;  (346) can_near_enemies = team_plan.can_near_enemies_range(…, camp_pos, 150000, debug).len()
// (348)   if ctx.debug [+0x3b] { (349) debug.infos.entry(champ.id).or_insert(vec![]).push(format!(…)) }   // 계측만
// (355)   champ_is_in_camp = dist_sq(champ, camp_pos) <= 140000²
// (357)   kill_priority_skip = strategy(=player.strategy(rnd,game)).object_finish == <태그0>
// (358~361)   && !( as_moba().unwrap().serpen.live_list.get(0).and_then(get_entity_by_id /*closure#11*/)
//                   .is_some_and(|serpen| iter_champions(1-team).any(|c| c.is_visible_from(champ.team) && !is_ignored_well_enemy(version,player,c) && can_enemy_hit_objective(c, serpen, 25000))) )   // closure#12 인라인
// (362)   if kill_priority_skip || !champ_is_in_camp || in_camp_ally <= can_near_enemies + in_camp_enemy { → 탈출 }
// (363)   strategy = player.strategy(rnd, game)   // 재호출
// (364)   nearest_enemy = in_camp_enemy_list.min_by_key(dist_sq to champ)   // closure#13
// (365)   let Some(nearest_enemy) else 탈출;  (366) max_range = max_range_cached(data, champ, nearest_enemy)
// (367)   if dist_sq(nearest_enemy, champ) > max_range² { 탈출 }
// (368)   formation_ok = check_favorable_engage_formation(version, player, data, nearest_enemy, 200000)
// (370)   if strategy.object_battle == <태그0> {
// (371)     if can_near_enemies == 0 && formation_ok && in_camp_ally > in_camp_enemy + 1 {
// (372~373)   if let Some(b) = self.try_engage(…, nearest_enemy.id, debug) { self.plan = Battle(b); return }   // ★§A 와 달리 near_ally/aggressive 를 안 쓰고 in_camp_ally > in_camp_enemy+1
//           }
//         } else {
// (377)     if formation_ok {
// (378~379)   if let Some(b) = self.try_engage(…) { self.plan = Battle(b); return }
//           }
//         }
// (386)   drop(in_camp_enemy_list)
// }
// (389) drop(_t_ob)  // ProfTimer 계측

// ===== §C engage.rs:391~455  아군 교전 합류(Support) =====
// (391) if self.plan.is_passive() && !matches!(self.plan, Battle) && self.team_plan.objective.is_some() [0x517 != 255] {
//         // is_passive 인라인(types.rs:254~257): 태그→idx = tag-2 (DeathMatch 는 4): PassiveLine(1)|SinglePlanLine(2) → true; PassiveJungle(5) → is_counter_jungle() 인라인(passive_jungle.rs:103~104) = plan.team[0x638] == plan.player_team[0x640]; 그 외 false
// (392)   roaming = player.info.parameter.roaming_ratio()
// (393)   range = roaming*100 + 150000;  (394) range_sq = range*range
// (397)   trigger_focus: Option<usize> = None
// (398~406) for pos_idx in 0..5 {   // 5회 언롤
// (399)     if pos_idx == my_pos_idx { continue }
// (400)     let Some(ally) = cache.champions[team][pos_idx] else continue;
// (401)     if ally.pos.distance_sq(champ.pos) > range_sq { continue }
// (402)     if let (_, Some(BigGoal::Battle{focus: Some(focus_id)})) = data.blackboard[team].big_goal[pos_idx] { trigger_focus = Some(focus_id); break }
//         }
// (408)   if let Some(focus_id) = trigger_focus {
// (411)     r3_hold = version >= 2 && focus_id == self.last_lost_fight.0 [0x1490]
// (412)               && cache.tick() <= self.last_lost_fight.1 [0x1498] + tps*3      // DW_OP_not: 값 = !(tick > …)
// (413)     if !r3_hold {
// (414)       let mut battle = BattlePlan::new(version, BattlePlanGoal::Support(focus_id), data, player);
// (415)       battle.entry_src [0x107] = 5;
// (416)       battle.support_target [0x0] = Some(focus_id);
// (417)       battle.update(version, rnd, player, data, &self.positioning_score, &self.team_plan, debug);
// (418)       battle.chats.clear();   // len[0x78] = 0
// (423~433)   stake: Option<FightPrediction> = if version >= 2 { cache.get_entity_by_id(focus_id).and_then(|t| resolve_join_stake(version, rnd, data, player, champ, t, &self.team_plan, debug)) /*closure#14*/ } else { None };
// (427)       stake_commit = stake.is_some_and(|s| s.line == FightLine::Commit(0));
// (428)       stake_veto   = stake.is_some_and(|s| s.line == FightLine::Disengage(2));
// (436)       let mut reject = battle.sub_goal == BattleSubPlanGoal::End;   // PartialEq::eq 호출
// (437)       if version >= 2 && (stake_veto || (!stake_commit && battle.exit_src [0x108] == 7)) { reject = true }   // ★IR: version<2 경로(%1319)는 이 검사를 아예 안 함 — 소스 표기(게이트 위치)는 unknown
// (442)       if !reject {
//               if stake_commit {
// (451~452)       battle.ff_stake_join [0xfc] = battle.exit_src == 7 || matches!(battle.sub_goal, KitingBack|RunAway)
//               } else {
// (443~444)       if matches!(battle.sub_goal, KitingBack|RunAway) && battle.prev_die_eval [0xa8] > tps*2 {
// (445~446)         battle.update(…same args…); battle.chats.clear();
// (447~449)         if battle.sub_goal == End { reject = true /* drop battle, plan 유지 */ }
//                 }
//                 battle.ff_stake_join = false
//               }
// (453)         if !reject { self.plan = BigPlan::Battle(battle) }   // ★return 하지 않고 461 로 계속
//             }
//           }
//         }
//       }

// ===== §D engage.rs:461~563  피격/트리거 응전(Response) =====
// (461~464) objective_posture: Option<ObjectivePosture(88B)> = match objective { Some(Morgard{..}) => self.team_plan.v25_objective_posture(version, player, data, JungleType::Morgard(4)), Some(Serpen{..}) => …(Serpen(5)), _ => None }
// (466~467) objective_waiting_posture = objective_posture.is_some_and(|p| p.should_wait_before_commit())   // 인라인: p.kind[+0x50] 태그 ∈ {3 WaitGroup, 4 SoftDisengage}
//                                       || self.team_plan.v27_objective_discipline_blocks_battle(version, data)   // 단락평가(앞이 true 면 호출 안 함)
// (468~484) battle_area_filter: Option<(x,y,range_sq)> = match objective {
//             Some(Morgard{phase,..}) => match phase { Hunt(3) => (470) Some((camp_pos(Morgard), 140000²)), Setup(1)|Assemble(2) => (474) Some((camp_pos(Morgard), 240000²)), None(0) => None, _ => unreachable },
//             Some(Serpen{phase,..})  => match phase { Hunt => (478) Some((camp_pos(Serpen), 140000²)), Setup|Assemble => (482) Some((camp_pos(Serpen), 240000²)), None => None },
//             _ => None }
// (487) steal_session_active = my_pos_idx == 1 && self.team_plan.current_steal_session.is_some() [0x2b2 != 2]
// (488) if !objective_waiting_posture && can_battle_triggered_filtered(version, player, data, battle_area_filter) {
// (489)   if !steal_session_active && !matches!(self.plan, Battle) {
// (490)     if cache.tick() > self.last_dive_abandon_tick [0x1480] + 1 + tps*4 {   // dive_rejoin_cd(version, tps) 인라인(engage.rs:971~973 · fn(usize,usize)->usize) = tps*4 + 1 — version 인자는 `#dbg_value(i64 poison)` = 미사용
// (491)       let mut battle = BattlePlan::new(version, BattlePlanGoal::Response(2), data, player);
// (492)       battle.entry_src = 3;
// (493)       if self.team_plan.last_battle_tick [0x318] + tps*3 > cache.tick() {
// (494)         battle.exit_src [0x108] = 14;
// (498)         battle.sub_goal [0x58] = self.v2_response_retreat_stance(version, rnd, player, data, debug);   // IR 은 self 를 안 넘김(dead-arg 제거)
//             }
// (503)       champ (재바인딩)
// (504~505)   near_allies = (0..5).filter(|i| self.team_plan.ally_battle_stop_tick[i].is_none() [tag==0] && cache.champions[team][i].is_some_and(|c| c.pos.distance_sq(champ.pos) <= 120000²)).count()   // closure#18 인라인
// (506~508)   near_enemies = iter_champions(1-team).filter(|c| c.is_visible_from(champ.team) && !is_ignored_well_enemy(version,player,c) && c.pos.distance_sq(champ.pos) <= 120000²).count()   // closure#19 인라인(루프)
// (509)       can_near_enemies = team_plan.can_near_enemies_range(version, rnd, player, data, champ.x, champ.y, 150000, debug).len()
// (511~513)   strategy = player.strategy(rnd, game); focused_area = strategy.focused[+0x11]; is_focused = focused_area.is_in_focused_area(ctx, champ.x, champ.y)
// (515)       serpen_alive = cache.as_moba().map_or(false, |m| !m.jungle_runner.serpen.live_list.is_empty())
//             if !serpen_alive && near_allies <= can_near_enemies + near_enemies && !is_focused {
// (517)         battle.exit_src = 14;
// (518)         battle.sub_goal = self.v2_response_retreat_stance(version, rnd, player, data, debug);
//             }
// (523)       battle.update(version, rnd, player, data, &self.positioning_score, &self.team_plan, debug);
// (524)       battle.chats.clear();
// (526)       if battle.sub_goal == End { drop(battle); return; }   // ★함수 종료(plan 안 바꿈)
// (535)       if version >= 2 {
// (536)         if matches!(battle.sub_goal, KitingBack|RunAway) {
// (537)           if !(cache.tick() > self.last_response_bail_tick [0x1488] + tps*3) {
// (541)             if battle.prev_die_eval > tps*2 { drop(battle); return; }
//                 }
//               }
// (551)         if matches!(battle.sub_goal, KitingBack|RunAway) {
// (554)           if battle.prev_die_eval > tps*2 {
// (555~556)         battle.update(…); battle.chats.clear();
// (557)             if battle.sub_goal == End { drop(battle); return; }
//                 }
//               }
//             }
// (562)       self.plan = BigPlan::Battle(battle);   // ★return 하지 않고 567 로 계속
//           }
//         }
//       }

// ===== §E engage.rs:567~618  press-line(에픽 압박 라인) 타워 다이브 =====
// (567~575) press_line: Option<LineType> =
//   if self.v3_epicops_armed [0x180a] {
// (568)   cache.as_moba().and_then(|m| if m.remain_epic_time(team) [epic_minion_buff_time[team] 0x240+team*8] != 0 {
// (569)       v3_epic_formation(rnd, player, data).filter(|f| !f.is_split).map(|f| f.line) } else { None })   // (i8,i8) 반환: .0 = Option<V3EpicFormation> 니치(0=is_split false,1=true,2=None), .1 = line
//   } else if let Some(MainObjective::PressEpic(line)) = objective [0x517 == 5] { Some(line [0x518]) } else { None };
// (578) if let Some(line) = press_line {
// (579)   strategy = player.strategy(rnd, game)
// (580)   if !matches!(self.plan, Battle) && strategy.tower_press [+0xd] == <태그 0> {
// (581)     if cache.tick() > self.last_dive_abandon_tick + 1 + tps*4 {
// (583~590)   nearest_tower = cache.tower(line)[enemy].or(tower2(line)[enemy])                              // 0x180+line*32 / 0x190+line*32
//                 .or(cache.twin_towers[enemy].iter().min_by_key(|t| t.pos.distance_sq(line.get_start_position(setting, team))).copied())   // closure#24/#25
//                 .or(cache.nexus[enemy]).unwrap()
// (591)       in_tower_ally  = iter_champions(team).filter(|c| c.pos.distance_sq(nearest_tower.pos) <= 140000²).count()   // closure#26
// (592~594)   in_tower_enemy = iter_champions(enemy).filter(|c| c.is_visible_from(champ.team) && !is_ignored_well_enemy(version,player,c) && c.dist_sq(nearest_tower) <= 140000²).count()   // closure#27
// (597)       if in_tower_ally > in_tower_enemy {
// (598~601)     nearest_enemy = iter_champions(enemy).filter(closure#28: 가시 && !ignored && dist_sq(x, nearest_tower) <= 140000²).min_by_key(dist_sq to champ)   // closure#29
// (602)         if let Some(nearest_enemy) {
// (603)           max_range = max_range_cached(data, champ, nearest_enemy)
// (604)           if dist_sq(nearest_enemy, champ) <= max_range² {
// (605)             formation_ok = check_favorable_engage_formation(version, player, data, nearest_enemy, 200000)
// (610)             dive_viable  = tower_dive_is_viable(version, rnd, player, data, &self.team_plan, nearest_enemy, true, debug)
// (611)             if formation_ok && dive_viable {
// (613)               dive_tower = if let EntityType::Tower(info) = nearest_tower.ty [0x68==2] { Some(info.ty [0x128]) } else { None }
// (614)               if let Some(b) = self.try_engage_dive(version, rnd, player, data, nearest_enemy.id, dive_tower, debug) {
// (615)                 self.plan = BigPlan::Battle(b);  (618) return;
//   }}}}}}}}

// ===== §F engage.rs:626~661  팀 Dive 목표 라인 타워 다이브 =====
// (626) if let Some(MainObjective::Dive{line}) = objective [0x517 == 9] {
// (627)   if !matches!(self.plan, Battle) && (628) cache.tick() > self.last_dive_abandon_tick + 1 + tps*4 {
// (629~636) nearest_tower = tower(line)[enemy].or(tower2).or(twin_towers[enemy] min_by_key dist(line.get_start_position(setting, team))).or(nexus[enemy])   // closure#30/#31
// (637)   if let Some(nearest_tower) {     // ★§E 와 달리 unwrap 이 아니라 let-else
// (639)     in_tower_ally  = iter_champions(team).filter(dist_sq(c, nearest_tower) <= 140000²).count()   // closure#32
// (640~642) in_tower_enemy = iter_champions(enemy).filter(가시 && !ignored && dist_sq(c, nearest_tower) <= 140000²).count()   // closure#33 (루프)
// (644)     if in_tower_ally > 1 && in_tower_ally > in_tower_enemy {
// (645~648)   nearest_enemy = enemies.filter(closure#34: 가시·!ignored·dist(nearest_tower)<=140000²).min_by_key(dist to champ)   // closure#35
// (649)       if let Some(nearest_enemy) { (650) max_range = max_range_cached(data, champ, nearest_enemy);
// (651)         if dist_sq(nearest_enemy, champ) <= max_range² {
// (653)           dive_viable  = tower_dive_is_viable(version, rnd, player, data, &team_plan, nearest_enemy, true, debug)
// (654)           formation_ok = check_favorable_engage_formation(version, player, data, nearest_enemy, 200000)   // ★§E 와 호출 순서 반대(dive_viable 먼저)
//                 if dive_viable && formation_ok {
// (656)             dive_tower = nearest_tower.ty 가 Tower 면 Some(info.ty) else None
// (657~661)         if let Some(b) = self.try_engage_dive(…, nearest_enemy.id, dive_tower, debug) { self.plan = Battle(b); return }
//   }}}}}}}

// ===== §G engage.rs:669~723  정글 캠프(모가드/세르펜) 주변 교전 =====
// (669) let _t_mg = ProfTimer::start(85)   // 계측
// (672) if !objective_waiting_posture(§D 의 %1394) && self.team_plan.objective.is_none_or(|o| o.can_battle()) && !matches!(self.plan, Battle) {
//         // can_battle 인라인(team_plan.rs:1054~1066): Morgard{phase,..}|Serpen{phase,..} => phase == Setup(1); Repair(7) => false; ComebackPick{ready,..} => ready; 그 외 => true
// (673)   if is_line_phase(ctx, cache.tick()) { → 스킵 }   // 인라인(GameContext::is_line_phase(tick) runner.rs:397~399 → tutorial.spawn_epic() runner.rs:262~263 + GameSetting::is_line_phase(tick) setting.rs:702~703): ctx.tutorial ∈ {0 None,5 MidBottom,7 Line,8 Total} && tick < setting.epic_jungle.first_spawn_tick.saturating_sub(tps*30)  ⟹ 라인 국면이면 캠프 교전 안 함
// (676)   champ; (677) if champ.hp*100 / champ.stat_cached.hp <= 59 { 스킵 }   // HP 60% 미만 제외
// (678~679) can_near_enemies = team_plan.can_near_enemies_range(version, rnd, player, data, champ.x, champ.y, 250000, debug).len()
// (681~684) near_allies = (0..5).filter(|i| ally_battle_stop_tick[i].is_none() && champions[team][i].is_some_and(|c| dist_sq(c,champ) <= 150000² && c.hp*100/c.stat_cached.hp > 59)).count()   // closure#37 인라인
// (687)   if near_allies > can_near_enemies + 1 {
// (689~705)  camp_initiate_filter: Option<(x,y,r)> = match objective { Some(Morgard{phase}) => { Hunt => (691) (camp_pos(Morgard),180000), Setup|Assemble => (695) (camp_pos(Morgard),230000), None => None }, Some(Serpen{phase}) => { Hunt => (699) (camp_pos(Serpen),180000), Setup|Assemble => (703) (camp_pos(Serpen),230000), None => None }, _ => None }
// (708~712)  nearest_enemy = iter_champions(enemy).filter(closure#38: x.is_visible_from(champ.team) && !is_ignored_well_enemy(version,player,x) && dist_sq(x,champ) <= 250000² && camp_initiate_filter.is_none_or(|(cx,cy,r)| dist_sq(x,(cx,cy)) <= r*r)).min_by_key(dist_sq to champ)   // closure#39
// (713)      if let Some(nearest_enemy) { (714) max_range = max_range_cached(data, champ, nearest_enemy)
// (715)        if dist_sq(nearest_enemy, champ) <= max_range² {
// (716)          formation_ok = check_favorable_engage_formation(version, player, data, nearest_enemy, 200000)
// (718)          if formation_ok { (719) if let Some(b) = self.try_engage(…, nearest_enemy.id, debug) { (720) self.plan = Battle(b); (723) return } }
//   }}}}
// }

// ===== §H engage.rs:730~769  DefenseLine 타워 방어 다이브 =====
// (730) if let Some(MainObjective::DefenseLine(line)) = objective [0x517 == 3] {
// (731)   strategy = player.strategy(rnd, game)
// (732)   if !matches!(self.plan, Battle) && strategy.morgard_defense [+0xe] == <태그 1> {
// (733~740) nearest_tower = tower(line)[team].or(tower2(line)[team]).or(twin_towers[team] min_by_key dist(line.get_start_position(setting, team))).or(nexus[team])   // ★아군 측(team) 타워. closure#40/#41
// (733)   if let Some(nearest_tower) {
// (743)     in_tower_ally  = iter_champions(team).filter(dist_sq(c, nearest_tower) <= 160000²).count()   // closure#42
// (744~746) in_tower_enemy = iter_champions(enemy).filter(가시 && !ignored && dist_sq(c, nearest_tower) <= 200000²).count()   // closure#43
// (749)     if in_tower_ally > in_tower_enemy {
// (750~753)   nearest_enemy = enemies.filter(closure#44: 가시·!ignored·dist_sq(x, nearest_tower) <= 200000²).min_by_key(dist to champ)   // closure#45
// (754)       if let Some(nearest_enemy) { (755) max_range = max_range_cached(data, champ, nearest_enemy)
// (756)         if dist_sq(nearest_enemy, champ) <= max_range² {
// (757)           formation_ok = check_favorable_engage_formation(version, player, data, nearest_enemy, 200000)
// (760)           dive_viable  = tower_dive_is_viable(version, rnd, player, data, &team_plan, nearest_enemy, true, debug)
// (761)           if formation_ok && dive_viable {
// (761~764)         dive_tower = cache.iter_towers_without_nexus(enemy).min_by_key(|t| t.pos.distance_sq(nearest_enemy.pos)) /*closure#46*/ .and_then(|t| if let Tower(info) = t.ty { Some(info.ty) } else { None }) /*closure#47*/
// (765~769)         if let Some(b) = self.try_engage_dive(…, nearest_enemy.id, dive_tower, debug) { self.plan = Battle(b); return }
//   }}}}}}}
// (777) drop(_t_mg)

// ===== §I engage.rs:777~808  Defense(넥서스 방어) TryKill =====
// (777) if objective == Some(MainObjective::Defense) [0x517 == 2] {
// (779~782) nearest_enemy = iter_champions(enemy).filter(closure#48: 가시 && !ignored && dist_sq(x, champ) <= 150000²).min_by_key(dist to champ)   // closure#49
//         if let Some(nearest_enemy) {
// (783)   nexus = cache.nexus[team].unwrap()
// (784)   if cache.twin_towers[team].is_empty() [0x148+team*32] && nearest_enemy.dist_sq(nexus) <= 120000² {
// (785)     if !matches!(self.plan, Battle) {
// (786)       if version >= 2 && open_chase_race_hopeless(version, data, player, champ, nearest_enemy) { 스킵 }
// (787)       let mut b = BattlePlan::new(version, BattlePlanGoal::TryKill(nearest_enemy.id, 60), data, player);
// (788)       b.entry_src = 10;
// (792)       if version >= 2 {
// (794)         b.main_objective [0xff] = self.team_plan.objective;   // set_main_objective(objective) 인라인(battle.rs:349~350), 3B 복사
// (796)         b.update(version, rnd, player, data, &self.positioning_score, &self.team_plan, debug);
// (797)         b.chats.clear();
// (798)         if matches!(b.sub_goal, KitingBack(3)|RunAway(4)|End(7)) { drop(b); return }
//             }
// (802)       order = player.info.parameter.order_ratio();
// (803)       if rnd.gen_range(0..1000) < order*700/1000 + 300 {
// (804)         self.chats.push(Chat::Battle(nearest_enemy.id, 0));   // ★self 쓰기 0x7c8/0x7d0/0x7d8, grow_one 재할당 가능
//             }
// (806)       self.plan = BigPlan::Battle(b);  (808) return;
//   }}}
// }

// ===== §J engage.rs:812~853  Nexus(넥서스 공격) 다이브 TryKill =====
// (812) if let Some(MainObjective::Nexus(_line)) = objective [0x517 == 4] {   // line 페이로드는 안 읽음
// (814~817) nearest_enemy = iter_champions(enemy).filter(closure#50: 가시·!ignored·dist_sq(x,champ) <= 150000²).min_by_key(dist to champ)   // closure#51
//         if let Some(nearest_enemy) {
// (818)   nexus = cache.nexus[enemy].unwrap()
// (819)   if cache.twin_towers[enemy].is_empty() && nearest_enemy.dist_sq(nexus) <= 120000² {
// (820)     if !matches!(self.plan, Battle) {
// (825)       if cache.tick() > self.last_dive_abandon_tick + 1 + tps*4 {
// (828)         if version >= 2 && open_chase_race_hopeless(version, data, player, champ, nearest_enemy) { 스킵 }
// (829)         dive_viable = tower_dive_is_viable(version, rnd, player, data, &team_plan, nearest_enemy, true, debug); if !dive_viable { 스킵 }
// (830)         let mut plan = BattlePlan::new(version, TryKill(nearest_enemy.id, 60), data, player);
// (831)         plan.with_dive [0xf6] = true;
// (832)         plan.dive_join_tick [0xb8] = cache.tick();
// (833)         plan.dive_tower [0xfe] = Some(TowerType::TwinA(3));
// (834)         plan.entry_src = 11;
// (837)         if version >= 2 { (839) plan.main_objective = objective; (841) plan.update(…); (842) plan.chats.clear(); (843) if matches!(plan.sub_goal, KitingBack|RunAway|End) { drop; return } }
// (847~849)     order = order_ratio(); if rnd.gen_range(0..1000) < order*700/1000+300 { self.chats.push(Chat::Battle(nearest_enemy.id, 0)) }
// (851)         self.plan = BigPlan::Battle(plan);  (853) return;
//   }}}}
// }

// ===== engage.rs:858  self.handle_solokill(version, rnd, player, data, debug)  — 통째 인라인(engage.rs:136~231) =====
// (138) if player.info.position != Jungle(1) && let Some(MainObjective::Gank{line}) = objective [0x517 == 8] {   // 둘 다 아니면 return
// (140~143) my_line = match position { Top(0) => Top(0), Mid(2) => Mid(1), Bottom(3) => Bottom(2), _ => return }
// (146)   if my_line != line { return }
// (148)   jungler = cache.champions[team][1]
// (149~155) jungler_ready = jungler.is_some_and(|j| { champ = champions[team][my_pos].unwrap();
//             nearby = j.dist_sq(champ) <= 200000²; in_bush = map.bushes[clamp(j.y/32000,29)][clamp(j.x/32000,29)] != 0;
//             hidden = !cache.game.is_visible(1-team, j.id) [vtable+0xf8];  nearby && in_bush && hidden })
// (156)   if jungler_ready { return }   // 정글러가 부시 매복 중이면 솔킬 판단 안 함
// (158~161) if iter_champions(enemy).any(|c| is_near_line(ctx, c.x, c.y, line) && data.blackboard[team].is_recent_visible(game, player, c) && c.hp*100/c.stat_cached.hp < 35) { return }   // 5회 언롤
// (171)   champ = champions[team][my_pos].unwrap();  (172) if matches!(self.plan, Battle) { return }
// (173)   let Some((e, t)) = check_kill(version, rnd/*IR: poison — 콜리가 안 씀*/, player, data, &self.positioning_score/*IR: poison — 콜리가 안 씀*/, debug) else { return };
// (175)   target = cache.get_entity_by_id(e).unwrap();  (177) max_r = max_range_cached(data, champ, target);  (178) t = t.saturating_sub(cache.tick());
// (180~181) if ctx.debug { debug.add_log(data, player, format!(.. max_r, t ..)) }
// (186)   if t >= tps*5 { return }
// (190)   near_enemies = cache.champions(enemy, ctx.pool)   // bumpalo Vec<&Entity> 복사
// (192~197) near_enemies.retain(|c| data.blackboard[1 - team].is_recent_visible(game, player, c) && c.dist_sq(champ) <= (max_range_cached(data, champ, c) + 30000)²)   // aux m01 45456 — ★인덱스가 1-team(적 blackboard)
// (200~201) can_near_enemies = team_plan.can_near_enemies_range(version, rnd, player, data, champ.x, champ.y, 150000, debug)
// (202)   can_near_enemies.retain(|c| near_enemies.iter().any(|e| e.id == c.id))   // aux m01 45712
// (204~206) near_allies = (0..5).filter(|i| ally_battle_stop_tick[i].is_none() && champions[team][i].is_some_and(dist_sq(c,champ) <= 120000²)).count()
// (208~210) if ctx.debug { debug.infos.entry(champ.id).or_insert(vec![]).push(format!(near_allies, near_enemies.len(), can_near_enemies.len(), e, t)) }
// (215)   if near_allies < can_near_enemies.len()/2 + near_enemies.len() { return }
// (217)   if matches!(self.plan.goal(), BigGoal::Line(..)|Jungle(..)) [tag < 2] {
// (224)     if let Some(b) = self.try_engage(version, rnd, player, data, e, debug) { (225) self.plan = BigPlan::Battle(b) }
//         }
// (229)   drop(can_near_enemies, near_enemies)
// }
// (859) drop(_t_ob) → return

```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e4c5c0` → `d3d210` LegacyPlanHandler::update (i=111 · 변경·심층(r21 §D: w4 §3~§5))
- 0.5.8 src: `game-ai\src\plan_legacy\handler.rs:685` · one_line: AI 플랜 계층 최상위 틱 진입점 — 게임모드 분기(DeathMatch/SingleLane 전용 경로는 Moba 에서 사장) 뒤 Moba 본류: 룰스코프 정화·팀플랜/GoalData 갱신·채팅 수신/발신·플랜 전환(페이즈게이트 handle_interact_battle·패시브·전투 BattlePlan)·v3 귀환→패시브 사다리(1263~1289: Recall+만피면 passive_plan/PassiveLine 으로 플랜 교체)·서브플랜 병합·계측 기록(트레이스/SIM_STATS/DIEWIN)
- 0.6.0 판정: **심층(r21)** · 패치 요지: 호출자 = 래퍼 d5a940 · 구 본체 오프셋만(egowave 상수 스케일 1건) · 신규 17.5KB 목표 오케스트레이션: v3 위치 링 · 앵커/이름 계산 · armed 플래그군 · bb Vec +0x250 레코드 → 채팅(0xf)/넥서스방어(objective=2+ForcePassive)/kind4(objective=4) · ObjContest 표결(tp 모드/ETA +0x778~) · 강제 플랜 배정(7KB 미독) · 강제 LineGanker(4.6KB 미독) · 서포터 라인슬롯 · v3 정글 귀환 상태기 · 리콜커밋 +0x24aa/+0x24ac → 강제 ActiveRecall · Battle 이탈 +0x2060/68/70=(target,tick,**bp.exit_src**) · 미독 6 구역 → r22
- RE 정본: `2026-09-17_r21_심층_w4_LegacyPlanHandler_update_원문.md` · 절: w4 §3~§5
- sig: `fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool)`
- consts: [{"value": 2, "src_line": 688, "meaning": "GameMode::DeathMatch 메모리태그(tcxdict --enum GameMode: idx2·discr2·태그2 · 인코딩 Direct). `%219 = icmp eq i64 %218, 2`(m13.ll:15214) 참 → L689 update_deathmatch 인라인. ★reach(version=2·gamemode=Moba) 접힘 @entry %219→0 = 사장", "kind": "태그", "ev": 3}, {"value": 1, "src_line": 694, "meaning": "GameMode::SingleLane 메모리태그(tcxdict --enum GameMode: 태그1). `%678 = icmp eq i64 %677, 1`(m13.ll:16485) 참 → L695 update_single_lane 인라인. ★reach 접힘 @675 %678→0 = 사장. (qcspec 경고 사유: 이 1 은 `icmp eq` 의 비교값=열거형 태그이지 shl 시프트량이 아님 — folded_from 해당 없음)", "kind": "태그", "ev": 3}, {"value":
- 0.5.8 logic 전문:
```
// handler.rs:685~695 (배치 A)
// L685: fn update(&mut self, version, rnd, player, data, debug, lapse) — 진입. version 을 스택 %210 에 스필(store, m13.ll:14711) 외 명령 없음(alloca 200여 개는 함수 전체 지역변수).
// L688: mode = data.cache.game.get_game_mode()  // vtable+0x40 간접호출, {tag,ptr}
//   if tag == 2 (GameMode::DeathMatch) → 블록 %220 … (L689)
// L689: self.update_deathmatch(version, rnd, player, data, debug)  // modes.rs 인라인 517줄(주석본 L220~L674)
//   // NA(gamemode≠Moba 전용: DeathMatch) — reach version=2·gamemode=Moba 접힘 @entry %219→0 · 블록 220~674 전부 사장(배치 A 236블록(220~674 85·675·679·680~1626 149) 중 234 사장 = 85+149) · reads/writes/consts 미등재
// L690: return;  → 블록 %674 `br label %679`(m13.ll:16480) → %679 `ret void`(m13.ll:16489) = 조기 반환 · L691: `}`  (rmeta_srcmap handler.rs L690=13자 `      return;` · L691=5자 `    }` · 파일은 2칸 들여쓰기)
// L694: if tag == 1 (GameMode::SingleLane) → 블록 %680 … (L695)   // ★L688 과 별개의 `if`(else-if 아님 — L691 이 `    }` 단독 줄) · get_game_mode() 재호출 %676(m13.ll:16483) · L696 `return;`(블록 %1626 `br label %679`, m13.ll:18969) · L697 `}`
// L695: self.update_single_lane(version, rnd, player, data, debug)  // modes.rs 인라인 1,060줄(주석본 L680~L1626)
//   // NA(gamemode≠Moba 전용: SingleLane) — reach 접힘 @675 %678→0 · 블록 680~1626 전부 사장 · reads/writes/consts 미등재
// else → 블록 %1627 = Moba 본류 → 배치 B(줄 698, `_t_head` ProfTimer 시작)
//
// self writes(살아있는 범위): 0건. HEAP 재료(살아있는 범위): 0건. rnd/debug 부작용: 0건.
// 함수 공용 EH: 살아있는 invoke 318개의 unwind(목표 64종: %3303 62·%2270 28·%4122 27·%2216 26·%1640 24 …)는 직접 또는 중간 cleanuppad 의 `cleanupret … unwind label` 사슬을 거쳐 **전부** 블록 %1640 cleanuppad 로 합류(살아있는 `unwind to caller` 는 %5375 하나뿐 — 사장 영역의 %571·%771·%946·%1570·%1589·%1609 은 DeathMatch/SingleLane 인라인 내부)(m13.ll:18998~19001 — `%1641 = phi i1 [...]` · `%1642 = cleanuppad within none []` · `br i1 %1641, label %5376, label %5375`) → phi %1641 참이면 %5376 에서 drop_glue<Option<ProfTimer>>(%209 = L698 `_t_head`) 후 %5375 `cleanupret unwind to caller`(m13.ll:28938, L685 귀속) — 패닉 전파 경로에서 self 쓰기 없음.
// 함수 공용 스필: %210 = version(i64) 스택 사본 · 배치 B~E 의 `load i64, ptr %210` 23곳이 전부 version.

// handler.rs:696~976 (배치 B)
// 진입: 배치 A 블록 %675(695) → %1627. 예외 cleanup 블록(1640·2039·2216·2270 등)은 언와인딩 전용이라 생략.
// 표기: game = data.cache.game(&dyn AbstractGame, vtable+0x28=tick, +0x130=kill_logs) · ctx = data.context · bb = data.blackboard[player.team]

698 _t_head = ProfTimer::start(77)                       // prof::ENABLED 원자 로드; 계측 전용(판정 무관, 이하 ProfTimer 전부 동일)
699 self.sanitize_rule_scope(data)           // 콜리 계약(tcx callees[86]) = (&mut self, &OperationData). IR 의 3인자 (self, cache=%211, ctx=%1639) 는 internal fastcc 함수의 &OperationData 인자가 필드로 갈라진 것(ArgPromotion, 추정 — define m13.ll:11163 인자 %1 `readonly captures(address_is_null)` %2 `captures(address, read_provenance)`). 714·761 도 동일 · &mut self = 본 범위 self 쓰기 재료(내부 미탐색)
702 { _t = ProfTimer::start(57); 703 self.team_plan.update(version, rnd, player, data, debug); 704 drop(_t) }
705 self.chats.extend(self.team_plan.chats.drain(..))    // ★HEAP chats@0x7c8 grow / team_plan.chats@0x1b8 비움
707 { _t = ProfTimer::start(58); 708 GoalData::update(&mut self.data, version, rnd, player, data, debug); 709 drop(_t) }

711 if version >= 2 { self.update_v2_egowave(player, data.blackboard, ctx, rnd) }   // 인라인 handler.rs:474~488
  474 (version<2 → return)
  477 self.v2_armed = true; 478 self.v3_armed = true; 479 self.v3_epicops_armed = true
  480 match egowave_check(player, bb, ctx) {                // 인라인 handler.rs:2269(정의)~2285 (자유함수 handler::egowave_check)
    2270 if player.info.position == Jungle → None
    2273 roaming = player.info.parameter.roaming_ratio() as i32; 2274 ego = ego_ratio() as i32
    2275 my_line = match position { Top→Top(0), Mid→Mid(1), _→Bottom(2) }
    2280 my_line = rule_scope::fallback_line(ctx, my_line)   // 2인자(tcx callees[20]) — 「default Bottom」은 인자가 아니라 본체 rule_scope.rs:34(JungleOnly → %1749 → Bottom=2, m13.ll:19315/19318). 879 의 아웃오브라인 call 과 같은 함수 · 인라인 rule_scope.rs:29~34: line_exists(ctx,line)(tutorial.spawn_line_minion(line)) 이면 그대로, 아니면 valid_lines(tutorial).last() (First/Bottom→[Bottom], TopSolo→[Top], MidSolo→[Mid], MidBottom→[Mid,Bottom]), JungleOnly→default Bottom
    2281 ms = &bb[player.team].{top|mid|bottom}_minion_state (my_line 별 +0/+0x28/+0x50; team<2 bounds check)
    2282 if !(ms.from_mid < -2000 && ms.minion_count < -2) → None
    2283 wave_concern = 500 - roaming; 2284 if wave_concern < 1 → None
    2285 Some((my_line, follow_own = wave_concern*ego/500))
  }
  None → 488 self.v2_egowave = 0
  Some((line, follow_own)) → 482 if self.v2_egowave != 0 && self.v2_egowave_line == line { /*유지*/ }
                             else { 483 self.v2_egowave = if rnd.gen_range(0..1000) < follow_own {2} else {1}; 484 self.v2_egowave_line = line }

714 self.sanitize_rule_scope(data.cache, ctx)
716 if !self.counter_jungle_route_init && player.info.position == Jungle && player_count(ctx)==full /*인라인: !(tutorial∈1..=7)*/ {
  717 self.counter_jungle_route_init = true
  718 strategy = player.strategy(rnd, game)               // PlayerState::strategy(sret 24B, &player, &mut rnd, game 팻포인터)
  719 if strategy.early_jungle == CounterJungle(2) {
    720 mf_note_swap(28, game.tick())                    // 인라인 handler.rs:410: mf_swap = (28, tick)
    722 p = PassiveJunglePlan::new_counter_jungle(rnd, player.info.team, strategy.focused)   // sret 104B
    721 self.plan = BigPlan::PassiveJungle(p)             // ★HEAP: drop_glue(&self.plan) → 태그 7 store + 104B memcpy@+0x5f0 (723)
  }
}

728 if !lapse && (!self.received_chats.is_empty() || !self.misunderstood_received_chats.is_empty()) {   // ★lapse 게이트는 745 블록까지 덮는다: lapse=true 면 %1802 `br i1 %6, %1929, %1832`(m13.ll:19423) 로 758(%1929) 직행 — 745 의 chats_wait 검사(%1923)는 preds %1910(742 뒤)·%1838(수신 둘 다 빔)뿐
  729 handled_chats: Vec<(usize,Position,Chat)> = Vec::new()   // 로컬 힙(741 에서 drop)
  730 for (tick, from, chat) in self.received_chats.iter() { 731 if *tick <= game.tick() { 732 handled_chats.push((tick, from, chat.clone())) } }
  735 self.received_chats.retain(|(tick,..)| *tick > game.tick())      // aux m06.ll:2048
  736 for (tick, from, c) in handled_chats {
    737 if rule_scope::chat_allowed(ctx, &c) {
      738 misunderstood = self.take_misunderstood_received_chat(tick, from, &c)     // fastcc, &mut self
      739 self.handle_chat(version, rnd, player, data, from, &c, misunderstood, debug)   // 자식 명세(계약만)
    }
  }
  742 self.misunderstood_received_chats.retain(|(tick,..)| *tick > game.tick())   // aux m06.ll:2184
}
745 if !self.chats_wait.is_empty() {   // ← 728 과 같은 `!lapse` 블록 안(위 주석). lapse=true 면 이 블록 통째 생략(chats_wait 발행·retain 둘 다)
  746 for (tick, chat) in self.chats_wait.iter() { 747 if *tick <= game.tick() { 748 if chat_allowed(ctx, chat) { 749 self.chats.push(chat.clone()) /*★HEAP*/ } } }
  754 self.chats_wait.retain(|(tick,_)| *tick > game.tick())   // aux m06.ll:2727
}
758 if let Some(chats) = self.plan.chats() /*인라인 types.rs:185~189, 태그별 Vec 위치*/ { 759 self.chats.extend(chats.drain(..)) }   // ★HEAP
761 self.sanitize_rule_scope(data.cache, ctx)
763 drop(_t_head)
764 _t_np = ProfTimer::start(59)
765 next: Option<BigPlan> = self.plan.next_plan(version, rnd, player, data, &self.data, &self.positioning_score, &self.team_plan, debug)   // sret 384B · &mut 는 self.plan(%1930) 뿐, team_plan 은 `&TeamPlan`(tcx callees[69]). ⚠IR 인자 %1652 에 noalias/readonly/dereferenceable 이 없는 사유는 미탐색(정의 m02.ll:8528 %8 도 동일 — 본체는 %8 을 PassiveJungle/LineGanker/LineGankCover ::next_plan 에 넘길 뿐)
767 drop(_t_np); 768 _t_tail = ProfTimer::start(78)   // (_t_tail 의 drop 은 배치 C/D)

769 if let Some(plan) = next {                                // next.tag == -1 → None → 947 로
  770 if rule_scope::plan_allowed(ctx, &plan) {               // 인라인 rule_scope.rs:101 → goal_allowed(ctx, plan.goal()) :91~95
        // goal.tag: Line(0)→line_exists(ctx, goal.line) · Jungle(1)→tutorial∈{None,JungleOnly,Total} · Epic(2)→morgard_exists=tutorial∈{None,Line,Total} · Serpen(3)→tutorial∈{None,MidBottom,Line,Total} · Nexus/Battle/Recall(4·5·6)→true
    772 if plan.tag==LineGanker(10) && plan.LineGanker.phase == WaitResponse(6) {
      775 can_transition = game.tick().saturating_sub(self.gank_cancel_tick) > ctx.setting.tick_per_second*10
      783 if !can_transition → 945 drop(plan) → 947   // 전이 거부(갱크 취소 후 10초 쿨다운)
    }
    784 if let Some(chats) = self.plan.chats() { 785 self.chats.extend(chats.drain(..)) }   // ★HEAP (전이 직전 현재 플랜의 chats 회수)
    789 if player.info.position == Jungle {                    // %1800 거짓일 때만; 아니면 938 로
      789 if self.plan.tag == PassiveJungle(7) { 790 self.last_jungle_lead_action_tick = game.tick() }
      797 if self.plan.tag == LineGanker(10) {
        799 is_transition_to_battle = plan.tag == Battle(9)
        801 if is_transition_to_battle { 803 self.active_gank_line = Some(self.plan.LineGanker.line) }
        else {
          806 self.active_gank_line = None
          807 if let Some(last) = self.gank_periods.last_mut() { 808 if last.1 == 0 {
            809 start_tick = last.0; 810 end_tick = game.tick(); 811 last.1 = end_tick
            813 if ctx.trace_level != Off {
              815 duration = end_tick.saturating_sub(start_tick); 816 line = self.plan.LineGanker.line
              818 kills  = game.kill_logs().iter().filter(|k| start<=k.tick<=end && k.killer_team==player.team && (k.killer_position==Jungle || k.assist.contains(&Jungle))).count()   // aux m13.ll:60162
              824 deaths = game.kill_logs().iter().filter(|k| start<=k.tick<=end && k.killer_team!=player.team && k.killed_position==Jungle).count()   // aux m13.ll:60317
              830 success = kills != 0
              833 self.pending_trace_events.push(PendingTraceEvent{ event: GankResult{kills,deaths,duration,line,success}, tick: end_tick })   // ★HEAP
            }
          } }
        }
      }
      850 if self.plan.tag == Battle(9) {
        851 if plan.tag == Battle(9) → 938   // Battle→Battle 은 장부 생략
        853 if self.team_plan.objective.tag == ComebackPick(11) {
          854 pick_start = self.team_plan.comeback_pick_start_tick
          855 got_kill = game.kill_logs().iter().rev().any(|k| k.killer_team==player.team && k.tick >= pick_start)   // 인라인 closure$5(:856)
          if got_kill { 859 self.team_plan.comeback_pick_success_count += 1; 860 if let Some(last)=self.team_plan.comeback_pick_outcomes.last_mut() { if last.1==0 { last.1 = 1 } } }
          else       { 863 if let Some(last)=...last_mut() { if last.1==0 { last.1 = 6 } } }
        }
        866 if matches!(self.team_plan.objective.tag, ComebackPick(11)|Gank(8)|Dive(9)) { 867 self.team_plan.mf_note_obj_clear(41, game.tick()) /*mf_obj_clear=(41,tick)*/; 868 self.team_plan.objective = None }
        870 if let Some(last) = self.gank_periods.last_mut() { 871 if last.1 == 0 {
          872 start_tick = last.0; 873 end_tick = game.tick(); 874 last.1 = end_tick
          876 if ctx.trace_level != Off {
            878 duration = end_tick.saturating_sub(start_tick)
            879 line = rule_scope::fallback_line(ctx, self.active_gank_line.unwrap_or(Mid))   // 아웃오브라인 call(2인자판, default 없음)
            882~899 kills/deaths/success/GankResult push — 818~833 과 동일(aux m13.ll:60462/60617 본문 동일)
          }
        } }
        909 self.active_gank_line = None
      }
      916 if plan.tag == LineGanker(10) {
        918 last_start = self.gank_periods.last().map(|(s,_)| *s).unwrap_or(0)
        919 if game.tick().saturating_sub(last_start) > tps*10 {
          920 self.gank_attempt_count += 1
          922 self.gank_periods.push((game.tick(), 0))   // ★HEAP
          925 actual_score = evaluate_gank_opportunity_with_score(rnd, player, data, plan.LineGanker.line, 0).1   // sret 12B 의 +8 i32
          927 self.gank_score_attempts.push((game.tick(), actual_score, 0))   // ★HEAP
        }
        931 if self.team_plan.objective.is_none() { 932 self.team_plan.objective = Some(Gank{line: plan.LineGanker.line}); 933 self.team_plan.gank_start_tick = game.tick() }
      }
    }
    938 mf_note_swap(20, game.tick()); 939 self.plan = plan   // ★HEAP drop_glue(&self.plan) 후 memcpy 384
  } else {   // plan_allowed 거짓
    942 mf_note_swap(26, game.tick()); 943 self.plan = BigPlan::ForcePassive(태그 2)   // ★HEAP drop_glue 후 store
    945 drop(plan)
  }
}

947 self.force_plan_update(version, rnd, player, data, debug, lapse)   // 인라인 handler.rs:1713~1766
  1713 _t = ProfTimer::start(79)
  1714 if !lapse { 1715 self.handle_interact_battle(version, rnd, player, data, debug) }   // 자식 명세(전환엔진·페이즈게이트 소재 — 본 범위엔 그 식 없음). 1717 drop(_t)
  1719 was_battle = self.plan.tag == Battle(9); entry_src = self.plan.Battle.entry_src(0x6f7, 선읽기)
  1724 r2_saved: Option<BigPlan> = if version < 2 || !was_battle { None } else { Some(self.plan.clone()) }   // ★HEAP: BigPlan::clone(m13.ll:20310) = 페이로드 소유 Vec 복제(alloc) · 1753 이동 또는 1755 drop
  1726 _t = ProfTimer::start(80)
  1727 if lapse {
    1737 before_obj = self.team_plan.objective (3B); 1738 before_plan = self.plan.clone() /*★HEAP clone(m13.ll:20364) · 1744 이동 또는 1747 drop*/; 1739 before_chats = self.team_plan.chats.len()
    1740 self.team_plan.update_objective(version, rnd, player, data, self, &self.plan, debug)
    1741 cleared = before_obj.is_some() && self.team_plan.objective.is_none()
    if cleared { 1747 drop(before_plan) }   // 목표 해제(종료)만 통과
    else { 1743 self.team_plan.objective = before_obj; 1744 self.plan = before_plan /*★HEAP*/; 1745 self.team_plan.chats.truncate(before_chats) }   // 발행/전환 롤백
  } else { 1748 self.team_plan.update_objective(version, rnd, player, data, self, &self.plan, debug) }
  1750 drop(_t)
  1751 if let Some(saved) = r2_saved { 1752 if self.plan.tag == Battle(9) { 1755 drop(saved) } else { 1753 self.plan = saved /*★HEAP: v2+ 에서 update_objective 가 Battle 을 못 걷어내게 복원*/ } }
  1756 if was_battle && self.plan.tag != Battle(9) {   // v2+ 에선 위 복원 때문에 도달 불가 → 사실상 version<2 전용
    1758 self.ff_battle_exit = (22, entry_src, game.tick(), 0, 255); 1759 self.ff_battle_exit_latch = [0xff;8]
  }
  1763 goal = self.plan.goal()   // sret %31 24B — ★소비처 없음: %31 은 20510 호출 직후 21494 lifetime.end 뿐(로드 0건). 결과 미사용 호출(BigPlan::goal 은 &self 순수 accessor)

948 if self.plan.tag == ForcePassive(2) {
  949 (mf_p, mf_src) = self.passive_plan(version, rnd, player, data, debug)   // sret 392B = BigPlan + u8
  950 self.v2_apply_assign_commit(version, rnd, player, data, &mut mf_p, &mut mf_src, debug)   // 자식 명세(계약만) · ★둘 다 &mut(tcx): 952/953 의 mf_src 와 955 의 mf_p 는 커밋 뒤 값(IR %175 재로드 21542 · %176 → %173 memcpy 21548)
  952 if self.mf_swap.1 != game.tick() { 953 mf_note_swap(mf_src, game.tick()) }
  955 self.plan = mf_p   // ★HEAP
}
960 if !lapse && self.plan.is_passive() /*인라인 types.rs:254~257: PassiveLine(3)|SinglePlanLine(4) → true · PassiveJungle(7) → !is_counter_jungle ⇔ p.team == p.player_team · 그 외 false*/ {
  961 (new_plan, mf_src) = self.passive_plan(version, rnd, player, data, debug)
  969 keep_journey = false; if self.v3_armed {
    970 obj_key = self.team_plan.objective
    976 if self.v3_dest_obj == obj_key /*None==None 참, 둘 다 Some 이면 MainObjective::eq*/ && let Some((dx,dy)) = self.v3_dest { → 배치 C(줄 977, 블록 %2683) } else { → 배치 C(줄 982, 블록 %2725) }
  } else { → 배치 C(줄 988, 블록 %2671: v2_apply_assign_commit) }
} else { → 배치 C(줄 1005, 블록 %2768) }
// 다른 배치 경계: 2610(lapse 참) → %2768(1005) · 2639 switch default/2644 거짓 → %2768(1005)

// handler.rs:977~1251 (배치 C)
// 진입 ①: 배치B L976 `v3_armed && v3_dest_obj == team_plan.objective && v3_dest.is_some()` → L977(%2683). ②: L976 거짓/`arrived` 경로 → L982(%2725). ③: L969 v3_armed 거짓 → L988(%2671, 배치B 21622). ④: 배치B L948(plan!=SinglePlanLine)·L960(`lapse`==true 또는 !passive) → L1005(%2768) 직행(새 플랜 계산·커밋 전부 생략).
// vtable: game.tick = vtable+0x28 · get_entity_by_id = +0x1f0 · kill_logs = +0x130 (divtable AbstractGame). champ = cache.player_champion[player.team][player.position] (0x1e0+team*40+pos*8, team<2 bounds check).

// ── L977~1002: v3 목적지 게이트 + 새 플랜 커밋 (new=%172 384B, src=%171 u8: 배치B L961 passive_plan 결과)
L977: let (dx,dy) = self.v3_dest.unwrap()  // 값은 B 에서 로드(0x550/0x558)
L978: arrived = champ.is_some_and(|x| (|x.x-dx|² + |x.y-dy|²) < 64000²+1)   // Entity 0x660/0x668 · 4096000001
L979: if !arrived && closure$9(&self.plan) && closure$9(&new) {  // closure$9(L974, 배치B) = idx ∈ {PassiveLine, SinglePlanLine, PassiveJungle}
L985:   drop(new); → L1002 → L1005   // 목적지로 이동 중 + 구/신 모두 소극 플랜 = 교체 생략(구 플랜 유지). ⚠ champ 가 None 이면 arrived=false 로 취급(21694)
}
L982: self.v3_dest = self.v3_plan_dest(player, data, &new)   // tcx (&self, &PlayerState, &OperationData, &BigPlan) → fastcc 판(m13.ll:9990)은 self 제거·player→info.team(i64)·data→context 로 인자승격되어 (sret 24, team, &GameContext, &BigPlan) · 내부 미탐색
L983: self.v3_dest_obj = obj   // %169 (B L970 에서 team_plan.objective 복사)
L988: self.v2_apply_assign_commit(version, rnd, player, data, &mut new, &src, debug)   // 진입③ 합류점 %2671
L991: if self.plan is PassiveLine(3) && new is PassiveLine(3) {
L992:   if old.line(0x706) == new.line(+0x11e) {
L993:     new.0.v46_carry_over(&mut old.0)   // PassiveLinePlan::v46_carry_over(&mut new, &mut old)
}}
L997: if self.mf_swap.1(0x1618) != game.tick() {
L998:   self.mf_note_swap(src, game.tick())   // mf_swap = (src, tick) (handler.rs:410 인라인)
}
L1000: self.plan = new   // HEAP: 구 플랜 drop_glue 후 memcpy 384 (언와인드 시 원복 memcpy)
L1002: (스코프 끝 — 언와인드 경로 %2680: new drop)

// ── L1005~1013: 플랜 update (진입④ 합류점 %2768)
L1005: phase = match self.plan idx { Battle(7)→68, PassiveLine(1)→69, _→70 }; timer = ProfTimer::start(phase)  // prof::ENABLED 원자 로드 0 이면 None
L1010: self.plan.update(version, rnd, player, data, &self.data(0x0), &self.team_plan(0xf8) /*tcx `&TeamPlan` — Atomic 3개 외 불변*/, &self.positioning_score(0x990), debug)   // BigPlan::update m02.ll:6887 — 자식 명세 없음, 내부 미탐색
L1013: drop(timer)  // PHASE_NANOS[phase] += elapsed, PHASE_CALLS[phase] += 1 (원자)

// ── L1015~1016: 플랜 채팅 회수
L1015: if let Some(chats) = self.plan.chats() {   // types.rs:185~189: PassiveLine→+0x18(0x608) · SinglePlanLine/PassiveJungle/LineGanker/LineGankCover→+0x0(0x5f0) · SinglePlanBattle/Battle→+0x68(0x658) · DeathMatchBattle→+0xf8(0x6e0) · 그 외(ForcePassive·ActiveRecall·Epic*/Serpen*/AttackNexus/DefenseNexus) None
L1016:   self.chats(0x7c8).extend(chats.drain(..))   // HEAP grow
}

// ── L1023~1053: v48 회피 claim 창 계측
L1023: if let SubPlan::Battle(sb) = &self.sub_plan(0x768 tag 7) && sb.v48_dodge_claim(0x79b) {
L1025:   self.v48_claim_until_tick = max(self.v48_claim_until_tick, sb.v48_claim_hold_until(0x790))
}
L1029: now = game.tick()
L1030: if now > self.v48_claim_until_tick {
L1033:   nth = player.info.statistics.non_target_hit(0x678)
L1034:   if nth > self.v48_last_non_target_hit { L1035: d = nth-last; L1050: self.v48_other_hits += d }
} else {
L1031:   self.v48_claim_window_ticks += 1
L1033:   nth = …
L1034:   if nth > last { L1035: d = nth-last; L1037: self.v48_claim_window_hits += d
L1039:     if let Some(c) = champ { L1040: if c.ty is Champion(13) {
L1041:       match c.action_state@tag(0x70) { Move(2) → L1044 v48_claim_hit_moving += d ; Attack(3)|Skill(4)|Skill2(5)|Ult(6) → L1043 v48_claim_hit_locked += d ; _(Idle/Return) → L1045 v48_claim_hit_idle += d }
}}}}
L1053: self.v48_last_non_target_hit = nth   // 양 분기 공통(phi)

// ── L1059~1092: v48 시전 분포 계측 (자기 논타겟 스킬 시전 시 가장 가까운 적의 상태)
L1059: if let Some(c) = champ { L1060: if c.ty is Champion {
L1061:   cur = match action_state { Skill(4)→1, Skill2(5)→2, Ult(6)→3, _→0 }   // 0 이면 L1092 로
L1067:   if cur != 0 && cur != self.v48_last_cast_state(0x1814) {
L1069~1071: effect = match cur { 1 → c.skill_effect.as_ref()(tag 0x4f8) ; 2 → c.skill2_effect().as_ref() [level(0x5c8)>2 ? &skill2_effect(0x500) : &NONE] ; 3 → c.ult_effect().as_ref() [level>4 ? &ult_effect(0x538) : &NONE] }
L1074:   if let Some(e) = effect && e.casting(+0x30).is_nontarget() [casting ∈ {Position 1, Direction 2}] {
L1075:     nearest = cache.iter_champions(1-team).filter(|x| x.is_visible_from(c)).min_by_key(|x| distance_sq(x, c))   // aux m13.ll:5116 · is_visible_from(entity.rs:1482): c.team 이 Player 아니면 true, 아니면 x.visible_state[c.team]@tag(0x38+16*team)==0
L1077:     if let Some(t) = nearest {
L1078:       windup = e.start_timing(+0x20)
L1079:       if t.block_move() { self.v48_cast_cc += 1 }
L1081:       else if !(t.remain_action_time() < windup) { self.v48_cast_locked += 1 }   // 극성: remain < windup 이면 아래로, 아니면 locked (22332~22333)
L1083:       else { d = t.distance(c); r = max(e.range(c), 1) [range + (level-1)*growth + c.stat_buff_cached.range(0x438)]; if d*2 < r { v48_cast_free_near += 1 } else { v48_cast_free_far += 1 } }
}}}
L1092:   self.v48_last_cast_state = cur   // Champion 이면 항상(cur 가 0 이어도) 저장
}}

// ── L1098~1115: PassiveLine v46 계측 접기 (플랜 → 핸들러 누적)
L1098: if let BigPlan::PassiveLine(p) = &mut self.plan {
L1100:   self.v46_lane_recall_trigger_ticks(0x870).append(&mut p.v46_pending.trigger_ticks(0x650))   // HEAP: reserve+memcpy, 소스 len=0
L1101~1114: self.v46_lane_recall_{danger_ticks,veto_wave,veto_crash,veto_heal,stage2_saves,commit_clears,wave_enemy_half,wave_my_half} / v46_lane_flee_{triggers,hold_ticks,hold_acute_ticks,hold_refuge_ticks,hold_cover_ticks,nohit_episodes} (0x14d0~0x1538) += mem::take(&mut p.v46_pending.<동명>) (0x680~0x6e8 → 0)
L1115:   self.v46_lane_flee_episodes(0x8a0).append(&mut p.v46_pending.flee_episodes(0x668))
}
// ── L1118~1119
L1118: if let BigPlan::Battle(b) /*별도 if — PassiveLine 블록 끝 %3125 가 %3032 로 가서 태그를 다시 읽는다(22590·22406). 태그가 배타라 else-if 와 외연 동일*/ = &mut self.plan { L1119: self.v54_reentry_ticks(0x900).append(&mut b.v54_reentry_ticks(0x670)) }
// ── L1121
L1121: self.sanitize_rule_scope(data)   // tcx (&mut self, &OperationData)(handler.rs:571) → fastcc 판(m13.ll:11163)은 data→(cache, context) 인자승격 · 내부 미탐색

// ── L1124~1126: 라인 백파이트 지원 진입 (enter_line_backfight_support, handler.rs:599~633 인라인)
L1124: timer = ProfTimer::start(76)
L1125: {
  599: if let BigPlan::PassiveLine(p) = &self.plan {
  600:   line = p.line(0x706)
  603:   if let Some((ally_id, focus_id)) = utils::line_backfight_support_focus(version, player, data, line) {   // sret 24 · m04.ll:53979 미탐색
  606:     if let Some(focus) = game.get_entity_by_id(focus_id) {
  609:       if !fight_model::is_ignored_battle_enemy(version, player, data, focus, false) {
  613:         let mut battle = BattlePlan::new(version, BattlePlanGoal::Support(focus_id) /*24B by-ptr %25: store i64 1(+0)=Support 태그, +8=focus_id(22743~22744) — tcxdict --enum BattlePlanGoal: 0 TryKill·1 Support·2 Response·3 Avoid*/, data, player)
  614:         battle.entry_src(+0x107) = 6
  615:         battle.support_target(+0x0) = Some(focus_id)
  616:         battle.set_main_objective(self.team_plan.objective(0x517))  // → +0xff
  617:         battle.update(version, rnd, player, data, &self.positioning_score, &self.team_plan /*tcx `&`*/, debug)
  618:         battle.chats.clear()   // +0x78 len=0
  620:         if battle.sub_goal(+0x58)@tag ∈ {RunAway 4, End 7} { drop(battle); → 633 }   // 진입 취소
  624:         else { if context.debug(0x3b) { 625: if let Some(c) = champ { 626~627: debug.infos(+0xa0).entry(c.id).or_insert(vec![]).push(format!("!v26 line backfight support: line {:?}, ally {}, focus {}", line, ally_id, focus_id)) } }
  631:           self.plan = BigPlan::Battle(battle)   // HEAP: 구 플랜 drop · tag 9 + memcpy 280 → 0x5f0
  }}}}}
}
L1126: drop(timer)

// ── L1128~1131: 종료 판정
L1128: timer = ProfTimer::start(74)
L1129: plan_ended = !self.plan.is_passive() && self.plan.is_end(version, rnd, player, data, &self.data, &self.team_plan /*tcx `&`*/, debug)
       // is_passive(types.rs:254~257): PassiveLine(idx1)·SinglePlanLine(idx2) → true ; PassiveJungle(idx5) → !is_counter_jungle = (team(0x638) == player_team(0x640)) ; 그 외 false
L1130: drop(timer)
L1131: if !plan_ended → L1251

// ── L1133~1173: (plan_ended) 정글러 갱크 결과 트레이스
L1133: if player.position(0x9c0) == Jungle(1) && self.plan is LineGanker(10) {
L1135:   if let Some(last) = self.gank_periods(0x810).last_mut() {   // (start,end) 16B, len 0x820
L1136:     if last.1 == 0 {
L1137:       start = last.0 ; L1138: now = game.tick() ; L1139: last.1 = now   // 힙 원소 in-place
L1141:       if context.trace_level(0x39) != Off(0) {
L1143:         duration = now.saturating_sub(start)
L1144:         line = gank_plan.line(0x618)
L1147~1151:    kills = game.kill_logs().iter().filter(|k| start <= k.tick(+0x18) <= now && k.killer_team(+0x20) == player.team && (k.killer_position(+0x28) == Jungle || k.assist(+0x0).contains(&Jungle))).count()   // aux m13.ll:60762
L1153~1157:    deaths = kill_logs().iter().filter(|k| start <= k.tick <= now && k.killer_team != player.team && k.killed_position(+0x2c) == Jungle).count()   // aux m13.ll:60917
L1159:         success = kills != 0
L1162:         self.pending_trace_events(0x858).push(PendingTraceEvent{ event: GankResult{kills, deaths, duration, line, success}, tick: now })   // HEAP
}}}}
// ── L1178~1192
L1178: if self.plan.is_cancel() { L1179: if self.plan is LineGanker(10) { L1181: self.gank_cancel_tick(0x1470) = game.tick() } }
L1189: if self.plan tag ∈ {Battle 9, LineGanker 10} {
L1190:   if self.team_plan.objective(0x517) tag ∈ {Gank 8, Dive 9} {
L1191:     self.team_plan.mf_note_obj_clear(42, game.tick())   // (0x308,0x310) = (42, tick)
L1192:     self.team_plan.objective = None (0xff)
}}
// ── L1200~1237: Battle 이탈 계측
L1200: if let BigPlan::Battle(b) = &self.plan {
L1204:   if (b.dive_abandoned(0x6ea) || b.with_dive(0x6e6)) && !b.dive_entered(0x6e7) { L1205: self.last_dive_abandon_tick(0x1480) = tick }   // 극성: !(…) || dive_entered 이면 건너뜀 (24115~24121)
L1211:   if b.main_goal(0x630)@tag == Response(2) && b.exit_src(0x6f8) ∈ 11..=14 { L1212: self.last_response_bail_tick(0x1488) = tick }
L1215:   if b.exit_src == 11 { L1216: self.team_plan.last_resolver_bail_tick(0x320) = tick }
L1225:   if b.exit_src ∈ {11,12} {
L1226:     if b.main_goal@tag < 2 (TryKill 0 | Support 1) { t = main_goal.0(0x638); L1227: self.last_lost_fight(0x1490) = (t, tick) }
L1228:     else if let Some(focus) = b.focus() [sub_goal@tag(0x648) ∈ {0,1,2,3,5,6} → 0x650] { L1231: self.last_lost_fight = (focus, tick) }
  }
L1235:   self.ff_battle_exit(0x15f8) = (b.exit_src, b.entry_src(0x6f7), tick, b.exit_sub(0x701), b.ff_wave_obs_open(0x6f9))
L1236~1237: self.ff_battle_exit_latch(0x1608..) = (b.ff_exit1_cls(0x700), ff_wave_open_hp(0x6fa), ff_wave_open_pct(0x6fb), ff_wave_fire_hp(0x6fc), ff_wave_fire_pct(0x6fd), ff_wave_open_danger(0x6fe), ff_wave_fire_danger(0x6ff), dive_abort_src(0x6f3))
}
// ── L1240~1249: 종료 플랜 교체
L1240: let (new, src) = self.passive_plan(version, rnd, player, data, debug)   // sret 392 = BigPlan 384 + u8 · tcx `&self`(self 쓰기 없음)
L1241: self.mf_ret25_src(0x1658)[min(src,31)] += 1
L1246: self.v2_apply_assign_commit(version, rnd, player, data, &mut new, &src, debug)
L1247: self.mf_note_swap(25, game.tick())   // mf_swap = (25, tick) — L997 의 tick 동일성 검사 없이 무조건
L1248: self.plan = new   // HEAP: 구 플랜 drop · 언와인드 시 원복
L1249: (언와인드 %3873: new drop)

// ── L1251: self.v50_track_dive_episode(version, player, data)  (dive_episode.rs:36~118 인라인 · 346 IR줄)
  36: tick = game.tick()
  37~59: snap = closure$0() → Ok(active) | Err(reason):
    38: champ = cache.player_champion[team][pos] else Err(8)
    39: b = self.plan as Battle(9) else Err(1)
    40: b.with_dive(0x6e6) else Err(2)
    41: tower_ty = b.dive_tower(0x6ee) (Some) else Err(3)
    42: focus = b.focus() [sub_goal 0x648 tag ∈ {0,1,2,3,5,6} → 0x650] else Err(4)   // ⚠ tag 기본(default) 케이스는 %1736(L711, 배치A 의 unreachable/패닉 블록)로 점프
    43: target = game.get_entity_by_id(focus) else Err(5)
    44: target.team(0x0 tag 0 Player, 0x8 == 1-team) 아니면 Err(6)
    45: tp = cache.player_by_champion_id(target.id(0x5c0)) else Err(6)
    46: target_pos = tp.info.position@tag(0x9c0)
    47~49: tower = cache.iter_towers_without_nexus(1-team).filter(|t| t.tower_type == tower_ty).min_by_key(closure s_0)  else Err(7)   // 필터·키 본체 = dive_episode 소유 별도 define(m13.ll:4340~4531 `v50_track_dive_episode` 클로저) — 본 배치 범위 밖·미탐색
    50: atk = tower.attack_effect.as_ref()(tag 0x4c0 != -1) else Err(7)
    51: reach = atk.range(tower) [range(0x4a0) + (tower.level(0x5c8)-1)*growth(0x4a8) + tower.stat_buff_cached.range(0x438)] + champ.radius() + tower.radius()   // radius(entity.rs:1511~1515): mult=radius_mult(0x470); mult==0 ? radius(0x680) : radius*(mult+100)/100
    52: in_range = tower.distance(champ) <= reach
    53~56: if tower.ty is Tower(2) && let Some((_, id)) = tower.info.nearest_enemy(0x88 tag, 0x98 id) { tgt = get_entity_by_id(id); holder = (id == champ.id); tower_on_champ = tgt.is_some_and(|e| e.ty is Champion(13)); team_holder = tower_on_champ && e.team == Player(team) } else { holder=false, team_holder=false, tower_on_champ=false }
    58: tgt_hp_pct = min(target.hp(0x670)*100 / max(target.stat_cached.hp(0x628),1), 255) as u8
    59: Ok(active{ hp: champ.hp, catch_break: b.dive_catch_break_ticks(0x688), target_pos, in_range, holder, team_holder, tower_on_champ, tower_ty, snap: (race_adv: b.dive_last_race_adv(0x6e0), model 0x6f4, na 0x6f5, ne 0x6f6, tgt_hp_pct) })
  Ok(active):
    65: if let Some(live) = &self.v50_dive_ep_live(0x570) && live.tower(0x5e1) != active.tower_ty { 66: self.v50_fold_dive_episode(false, 0) }   // tps(0x12f8) 로드는 dead
    68: if let Some(live) = self.v50_dive_ep_live.as_mut() {
      70: live.gap_ticks = 0 ; 71: live.max_catch_break = max(catch_break, live.max_catch_break) ; 72: live.uncatch_total += (catch_break != 0)
      73: if live.start_model == 0 && snap.model != 0 { 75: live.start_model = model; live.start_race_adv = race_adv; 76: live.start_na = na; live.start_ne = ne }
      78: live.last_tick = tick ; 79: live.ep_ticks += 1
      80: if in_range { live.in_range_ticks += 1 }
      81: if team_holder { live.team_holder_ticks += 1 }
      82: if in_range && !tower_on_champ { live.minion_cover_ticks += 1 }   // 23755~23777 분기 재구성
      83: if holder { 84: live.holder_ticks += 1 ; 85: if let Some(ph) = live.prev_holder_hp { live.soaked_hp += ph.saturating_sub(hp) } ; 86: live.prev_holder_hp = Some(hp) } else { 88: live.prev_holder_hp = None }
    } else { 91~92: self.v50_dive_ep_live = Some(V50DiveEpLive{…}) (writes 참조) }
  Err(reason):
    108: if let Some(live) = self.v50_dive_ep_live.as_mut() {
      110: if reason ∈ {4,6} && live.gap_ticks < setting.tick_per_second(0x12f8) { 111: live.gap_ticks += 1; 112: live.ep_ticks += 1; 113: live.last_tick = tick }
      else { 115: dive_abandoned = self.plan is Battle && b.dive_abandoned(0x6ea) ; 116: abort_src = Battle ? b.dive_abort_src(0x6f3) : 0 ; 117: self.v50_dive_ep_abort_src(0x1811) = abort_src ; 118: self.v50_fold_dive_episode(dive_abandoned, reason) }
    }
  → 모든 경로 %3897 = L1253 (배치 D)

// ── 다른 배치로 넘어가는 지점
// %2725/%2683 ← 배치B L976 · %2671 ← 배치B L969 · %2768 ← 배치B L948/L960(lapse) · %1736 ← dive_episode.rs:42 focus() default → 배치A L711 · 언와인드 %2270→L769(배치B) · %3303→L1672(배치E) · 정상 종료 %3897 → L1253(배치D)

// handler.rs:1252~1672 (배치 D)
// 진입: 배치 C 의 블록들(%3507/%3573/%3623/%3636/%3649/%3664)이 %3897(24343) 로 합류. 레지스터: %0=self %210=&version %2=rnd %3=player %4=data %211=cache %212=game.data_ptr %214=game.vtable_ptr %1639=context %1930=&self.plan %2824=&self.sub_plan %2913=&cache.player_champion[team][pos] %2915=(내 챔피언 None) %1799=player.info.position %2867=player.info.team %4057=context.debug

// ── [1253~1291] 서브플랜 산출 + v3 귀환→패시브 사다리 ──
1253 let _t = ProfTimer::start(75)                      // prof::ENABLED 원자 로드가 0 이면 타이머 None(%151 tag -1)
1254 let mut sub: SubPlan(72B) = BigPlan::sub_plan(&mut self.plan(0x5e8, %1930 readonly 없음), version, rnd, player, data, &self.data(GoalData 0x0 = %0 readonly), &self.team_plan(0xf8), &self.positioning_score(0x990), debug)   // 자식 명세, 계약만 — self.plan 은 &mut 로 넘어가 내부 쓰기 가능(1270/1282 도 동일)
1263 if version > 1 /*항상 true*/ && sub.tag == 5(Recall) && self.v3_repair_done(1264 인라인 handler.rs:1851~1852: cache.player_champion[team][pos].is_some_and(|c| c.hp(0x670) >= c.stat_cached.hp(0x628)))  {
1265     self.v3_home_ladder[0](0x1640) += 1
1266     let plan = self.passive_plan(version, rnd, player, data, debug)          // sret 392B 중 앞 384B 만 BigPlan 으로 memcpy
1267     let mut swapped_ok = false
         if plan_allowed(context, &plan) {   // rule_scope.rs:101 인라인 → goal = plan.goal()(BigGoal 24B) → goal_allowed(rule_scope.rs:90):
                                            //   tag0 Line(line@+1): line_exists = tutorial(ctx+0x38) ∈ Top{0,2,7,8} / Mid{0,4,5,7,8} / Bottom{0,1,3,5,7,8}
                                            //   tag1 Jungle: player_count → tutorial ∈ {0,6,8}
                                            //   tag2 Epic: morgard_exists → tutorial ∉ {1..6}   tag3 Serpen: serpen_exists → tutorial ∈ {0,5,7,8}
                                            //   tag4 Nexus / 5 Battle / 6 Recall → 항상 허용
1268         self.mf_note_swap(29, game.tick())   // self.mf_swap = (29, tick)
1269         drop(self.plan); self.plan = plan                     // ★HEAP plan@0x5e8 교체 #1
1270         sub = self.plan.sub_plan(…같은 인자…)
1272         swapped_ok = (sub.tag != 5)          // 여전히 Recall 이면 아래 사다리 계속
         }
1272     if !swapped_ok {
1273         self.v3_home_ladder[1](0x1648) += 1
1274         let line = fallback_line(context, match player.info.position { Top(0)→Top(0), Bottom(3)|Support(4)→Bottom(2), _→Mid(1) })
1279         let plan = BigPlan::PassiveLine(PassiveLinePlan::new(line))   // tag 3(store i64 3 @+0) · 페이로드: v46_flee_entry tag@+8=0(None; 페이로드 +0x10~+0x20 미기록) · 5개 빈 Vec(cap 0·ptr=8) · +0x48~+0x120 memset 0 · line@+0x11e. ⚠+0x10~+0x20 과 +0x120~+0x180(enum 꼬리) 은 스택 잔재 그대로 self.plan 으로 memcpy 됨 → 0x5f8~0x608·0x708~0x768 은 비결정
1280         if plan_allowed(context, &plan) {
1281             drop(self.plan); self.plan = plan                 // ★HEAP plan@0x5e8 교체 #2
1282             sub = self.plan.sub_plan(…)
1284             if sub.tag == 5(Recall) { 1285 self.v3_home_ladder[2](0x1650) += 1; 1286 sub = SubPlan::LineWait(line) /*tag4, line@+8*/ }
         } else { 1285 self.v3_home_ladder[2] += 1; 1286 sub = SubPlan::LineWait(line); drop(plan) }
     }
     // 1289 임시 plan(%150) drop — passive_plan 결과가 self 로 이동되지 않은 경우만(%3982==1)
 }
1290 SubPlan::merge(&mut self.sub_plan(0x768), &sub)        // 자식 명세, 계약만
1291 drop(_t)   // ENABLED 였으면 PHASE_NANOS[75] += elapsed_ns, PHASE_CALLS[75] += 1

// ── [1294~1505] trace (context.trace_level(0x39) != Off) — 계측, 판단 없음 ──
1294 if context.trace_level != 0 {
1295     let name: String = self.plan.get_name()
1296     if name != self.prev_plan_name(0x840)  /*len 비교 후 memcmp*/ {
1298         let reason = self.determine_transition_reason(&name)   // &self 메서드(prev = self.prev_plan_name 0x840 을 내부에서 읽음) · 인라인 handler.rs:1675~1707, 순서대로:
             //  1679 name.starts_with("LineGanker") → 3 GankInitiated
             //  1682 prev.starts_with("LineGanker") && !name.starts_with("LineGanker") → 4 GankCompleted
             //  1687 name.starts_with("Battle") → 2 BattleTriggered
             //  1692 name.starts_with("Passive") && !prev.starts_with("Passive") → 0 PlanEnded
             //  1697 name.starts_with("ActiveRecall") → 6 LowHp
             //  1702 name.contains("Epic")||contains("Serpen")||contains("Nexus") → 5 ObjectiveChange
             //  1706 else → 9 Other(format!("{prev} -> {name}"))
1301~1305 my_champ = cache.player_champion[team][pos]; hp_ratio = my_champ.map(|c| c.hp*100/c.stat_cached.hp /*max 0 이면 div_by_zero panic*/).unwrap_or(0)
             ally_count = player_champion[team] 의 Some 개수; enemy_count = player_champion[1-team] 의 Some 개수 (1304/1305 count, 필터 없음)
1307 tick = game.tick()
1310 self.pending_trace_events.push(PendingTraceEvent{ event: PlanTransition{reason, from: prev.clone(), to: name.clone(), context: PlanContext{hp_ratio, ally_count, enemy_count, tick, gold_diff: 0}}, tick: game.tick()(1311) })
1321 if name.starts_with("Battle") && !prev.starts_with("Battle") {
1323     self.battle_start_tick = Some(game.tick())
1325     if self.plan.tag == 9(Battle) {
1326         focus = match plan.Battle.sub_goal.tag(0x648) { 0,1,2,3,5,6 → sub_goal.focus(0x650), 4 RunAway|7 End → 0 }   // BattlePlan::focus 인라인 battle.rs:30
1332         (my_hp%, my_max) = my_champ.map(|c| (if c.max==0 {0} else {c.hp*100/c.max}, c.max)).unwrap_or((0,0))   // 1303 과 달리 max 0 이어도 panic 없음(icmp eq 0 → 0)
1337         target = game.get_entity_by_id(focus); 1339 (t_hp%, t_max) = target.map(..).unwrap_or((0,0))
1344         engage_reason = match plan.Battle.main_goal.tag(0x630) {
                 0 TryKill → 1347 if prev.starts_with("LineGanker") {"gank_kill"} else 1349 if prev.contains("Hunt")||prev.contains("Epic") {"objective_kill"} else {"try_kill"}
                 1 Support → "support_ally"(1355)   2 Response → "forced_response"(1356)   3 Avoid → "avoid_fight"(1357) }
1361         initial_sub_goal = format!("{:?}", plan.Battle.sub_goal)
1367         ally_count = (0..5).filter(|i| player_champion[team][i].is_some_and(|a| a.distance_sq(my_champ) < 150000²)).count()   // aux sg_0 (my_champ.unwrap() — None 이면 panic 경로 %4390)
1372         enemy_count = player_champion[1-team].iter().flatten().filter(|e| e 가 내 팀에 가시(entity.rs:1482) && !is_ignored_well_enemy(version, player, e) && e.distance_sq(my_champ) < 150000²).count()   // aux sh_0
1375         drop(self.battle_start_state); self.battle_start_state = Some(BattleStartState{engage_reason, initial_sub_goal, tick: game.tick()(1376), my_hp: my_hp%, my_max_hp, target_id: focus, target_hp: t_hp%, target_max_hp, ally_count, enemy_count})
1388         self.pending_trace_events.push({EngageDecision{target_id: focus, my_die_tick:0, enemy_die_tick:0, expected_win:false}, tick: game.tick()(1389)})
1397         drop(engage_reason 원본)
         }
     }
1401 if prev.starts_with("Battle") && !name.starts_with("Battle") {
1402     tick = game.tick(); 1403 start = self.battle_start_tick.unwrap_or(tick); duration = tick.saturating_sub(start)
1407     state = self.battle_start_state.take().unwrap_or_default()   // Default: 빈 String 2개 + 0
1413     (my_hp_end%, my_hp_now) = my_champ.map(|c| (if c.max==0 {0} else {c.hp*100/c.max}, c.hp)).unwrap_or((0,0))   // max 0 → (0, c.hp), panic 없음. 1339/1420 의 target 쪽도 같은 checked 형
1418     target = game.get_entity_by_id(state.target_id(+0x48)); 1420 (t_hp_end%, t_hp_now) = target.map(..).unwrap_or((0,0))
1425     damage_taken = (state.my_hp(+0x38) * state.my_max_hp(+0x40) / 100).saturating_sub(my_hp_now)
1428     damage_dealt = (state.target_hp(+0x50) * state.target_max_hp(+0x58) / 100).saturating_sub(t_hp_now); 1429 target None(=표적 소멸) 이면 = 시작 hp 전부(select %4486)
1437     kills  = game.kill_logs().iter().filter(|k| start<=k.tick<=tick && k.killer_team==team && (k.killer_position==pos || k.assist.contains(&pos))).count()
1443     deaths = kill_logs.filter(|k| start<=k.tick<=tick && k.killer_team!=team && k.killed_position==pos).count()
1450     (disengage_reason, deaths_field, target_escaped) =
             if deaths>0 {("died", deaths, false)} else if kills>0 {("kill_secured",0,false)} else if my_hp_end% < 30 {("low_hp_retreat",0, target.is_some())} else if target.is_some() /*get_entity_by_id Some = 표적 생존*/ {("target_escaped",0,true)} else /*None = 표적 소멸*/ {("target_died",0,false)}
1472     won = kills != 0 && deaths_field == 0
1475     self.pending_trace_events.push({BattleResult{engage_reason: state.engage_reason, disengage_reason, initial_sub_goal: state.initial_sub_goal, kills, deaths: deaths_field, duration, ally_count: state.ally_count, enemy_count: state.enemy_count, my_hp_start: state.my_hp, my_hp_end, target_hp_start: state.target_hp, target_hp_end, damage_dealt, damage_taken, target_id: state.target_id, won, prediction_correct: won, target_escaped}, tick: game.tick()(1476)})
     }
1501 self.battle_start_tick = None      // ⚠ %4436 — 1321/1401 어느 쪽이든 name!=prev 블록 끝에서 무조건
     }
1504 drop(self.prev_plan_name); self.prev_plan_name = name     // name==prev 여도 실행(%4069→%4114)
 }

// ── [1507~1512] 디버그 라벨 (context.debug(0x3b)) ──
1507 if context.debug { champ = my_champ.unwrap() /*None 이면 panic %4730*/;
1509 debug.infos(+0xa0).entry(champ.id(0x5c0)).or_insert(vec![]).push(format!("speed : {}", champ.stat_cached.move_speed(0x640)))
1510 …push(format!("Main Objective: {:?}", self.team_plan.objective(0x517)))   1511 …push(format!("Plan: {:?}", self.plan.get_name()))   1512 …push(format!("SubPlan: {:?}", self.sub_plan)) }

// ── [1520~1622] SIM_STATS 도주 ring 계측 (prof::SIM_STATS != 0 && position != Jungle(1)) ──
1521 if setting.is_line_phase(game.tick())  /*setting.rs:703: tick < epic_jungle.first_spawn_tick(0x8a8).saturating_sub(tick_per_second(0x12f8)*30)*/ {
1522   death_count = player.info.statistics.death(0x5f8)
1523   if death_count > self.flee_prev_death_count(0x15b0) {
1524     self.flee_prev_death_count = death_count
1530     match self.flee_ring.last() {
         None → 1565 (tick=game.tick(), plan_tag=-1(255), gate=-1, label=4)
         Some((t_last, _, ptag_last, _)) → 1531 win = t_last.saturating_sub(tps*10)
1536       for (t, block, ptag, gate) in ring.iter().rev() { 1537 if t < win break;
1540         if block&1 != 0 { any_danger=true; 1542 A |= block&4!=0; B |= (block&6)==6;
1548           if (block&6)==2 && !found { found=true; rec=(ptag, gate) } } }
1553       if found → (t_last, rec.ptag, rec.gate, label 0)
1555       else → (t_last, ptag_last, -1, label = if B {1} else if A {2} else if any_danger {3} else {4}) }
1567     self.flee_death_retrospects.push((tick, label, ptag, gate))
1568     if context.debug { 1569 debug.add_log(data, player, format!("V46DEATH T{team} {pos:?} tick~{tick} label={LABELS[label]} plan_tag={ptag} block={gate}")) }   // LABELS = @anon.85 &str 표(이름 미해독)
1574     self.flee_ring.clear()
1575   } else if game.tick() % 6 == 0 && my_champ.is_some() {
1577     champ = my_champ
1579     enemies: bumpalo Vec = iter_champions(1-team).filter(sm_0: e.distance_sq(champ) < 150000² && (champ.team==Neutral || e.visible_state[champ.team]==Visible) && !is_ignored_well_enemy(version, player, e)).collect_in(context.pool)
1582     if enemies.is_empty() { drop; → 1629 } else {
1583       dis = match player_count(context)/*tutorial*/ { First(1)|Bottom(3) → setting.tower_attack_disable_tick_2v2(0x1400), MidBottom(5) → _3v3(0x1408), _ → tower_attack_disable_tick(0x13f8) }
1588       towers = iter_towers_without_nexus(cache, 1-team).filter(su_0: game.tick() <= dis && t.distance_sq(champ) < 150000²).collect_in(pool)
1593       mdt = check_kill_die_tick(version, rnd, data, player, champ, enemies(복사 32B), &towers, debug)
1594       my_tower = iter_towers_without_nexus(cache, team).min_by_key(so_0 /*미독해*/).or(cache.nexus[team](0x170))
1596       nrst = my_tower.map(|t| champ.distance(t) / max(champ.move_speed, 1)).unwrap_or(0)
1597 a = mdt < 600;  1598 b = mdt > nrst
1599       fleeing = self.sub_plan.tag==5(Recall) || self.plan.battle_engage_dir() == Some(-1) || (1600 self.plan.tag==3 && plan.PassiveLine.v46_flee(0x702))
1601       plan_tag_code = match self.plan.tag { 3 PassiveLine→0, 9 Battle→1, 8 ActiveRecall→2, 12|13|14|15 Epic/Serpen→3, _→4 }
1609       block = (a as u8) | (b?2:0) | (fleeing?4:0)
1612       gate = if plan.tag==3 && !fleeing && a && b { 1613 v46_flee_gate_check(version, player, data, champ).0 /*sret 40B 첫 u8, 내부 bumpalo Vec drop*/ } else { 255 }
1617       self.flee_ring.push((game.tick(), block, plan_tag_code, gate))
1618       if len_before > 127 { 1619 self.flee_ring.remove(0) }
1622       drop(enemies) } } }

// ── [1629~1670] DIEWIN 디버그 로그 (position ∈ {Bottom(3),Support(4)} && context.debug) ──
1630 if game.tick()%6==0 && my_champ.is_some() { 1631 champ;
1633 enemies = iter_champions(1-team).filter(sq_0 ≡ sm_0).collect_in(pool); 1636 if !empty {
1637 ne = enemies.len; 1639 nearest = enemies.min_by_key(|e| e.distance_sq(champ)) (sr_0);
1640 (myrng, erng, nrst, ehp) = nearest.map(|e| (1641 champ.attack_effect.map_or(false,|f| f.is_in_range(champ,e)), 1642 e.attack_effect.map_or(false,|f| f.is_in_range(e,champ)), 1643 e.distance(champ)/1000, e.hp*100/max(e.max,1))).unwrap_or((0,0,0,0))
1645 dis = (1583 과 동일 선택); 1650 towers = 적 타워 filter(sn_0≡su_0); 1654 mdt = check_kill_die_tick(…); 1655 hp = champ.hp*100/max(champ.max,1)
1656 sub = match self.sub_plan.tag { 2→"LineDefense",3→"LineSafe",4→"LineWait",5→"Recall",7→"Battle",9→"Hide",_→"Other" }
1665 debug.add_log(data, player, format!("DIEWIN T{team} {pos:?} hp={hp}% mdt={mdt} ne={ne} nrst={nrst}k myrng={myrng} erng={erng} ehp={ehp}% plan={get_name()} sub={sub}")) } 1670 drop(enemies) }

// ── 꼬리 ──
5069(28058) drop(_t_tail %191 — 배치 A L768 의 ProfTimer, 인덱스 동적·bounds 132) → 5362 → 679(16488) ret void  ;L1672
// unwind 정리 블록(1640/2013/2039/2270/3303/… L1672 cleanuppad)은 본 범위에 귀속되나 논리 없음(BigPlan/ProfTimer/String drop_glue)
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
