# r21 티어1 심층 — 웨이브 5 (1) · 0.5.8 명세 logic 전문 + exe 힌트

요구: 0.5.8 명세(아래 logic) 를 기준선으로 0.6.0 디컴을 **줄 단위로 대조**해 「어느 줄이 어떻게 바뀌었나 / 새 분기·조건·상수·필드·콜리」 를 명세 형식(줄번호 참조 · 등가 Rust)으로 낸다. version≥3 분기는 「v3 경로」 로 별도 표기(값 미확정 · v2/v3 둘 다 적는다). 대형 함수는 먼저 콜리 지도·블록 지도(0.5.8 ↔ 0.6.0 매핑)를 만들고 신규 구역을 특정한 뒤 신규 구역만 정독한다.

## `e5d5d0` → `d606b0` LegacyPlanHandler::handle_interact_battle  (32177→45755B · Δ+13578)
- 힌트: LegacyPlanHandler::handle_interact_battle(32→46KB · d606b0 · ObjContest 태그 검사 · resolve_join_stake(0,6) · 특성 감사 카운터 +0x1f98)
- exe 정렬: 명령 6763→9415 · 정렬 5159 · 잔여 구조 10 · 분기 10 · 콜리 주의: 2b072a0→d35fb0 미지(-234B) ; 2b074c0→16a7af0 콜리 변경?(J0.00·+138B) ; 31a01a3→381e0b3 미지 ; 31a01a3→d35010 미지 ; 31a3863→3821af0 미지(-32B) ; caefe0→ec21b0 미지 ; d230d0→3821813 미지(-27B) ; d25ea0→d35450 미지(-134B)
- 0.5.8 명세 #110 `engage__LegacyPlanHandler_handle_interact_battle` src game-ai\src\plan_legacy\handler\engage.rs:233 · one_line: 매 틱 교전 개시 판정 — 오브젝트(모가드/세르펜) 교전·아군 교전 합류(Support)·피격 응전(Response)·라인 다이브·정글캠프 교전·넥서스 공/방 교전·솔로킬 순으로 검사해 self.plan 을 BigPlan::Battle 로 바꾼다
- 0.5.8 params: self:&mut LegacyPlanHandler(6168B(IR %0: noalias·readonly 없음·dereferenceable(6168) = &mut. pla) · version:usize(IR %1 → alloca %112 로 스필. 게이트: >=2(ugt 1) 분기 411(423 은 %1258) · rnd:&mut StdRng(320B)(IR %2 noalias·readonly 없음 = &mut. 본문에서 직접 안 읽고 콜리(strategy/t) · player:&PlayerState(2528B)(IR %3 readonly. info.team(0x930)·info.position 태그(0x9c0)·inf) · data:&OperationData(24B)(IR %4 readonly. cache(+0)·context(+8)·blackboard(+0x10)) · debug:&mut DebugFrameData(224B)(IR %5 noalias·readonly 없음 = &mut. ctx.debug 일 때 infos(+0xa0))
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
