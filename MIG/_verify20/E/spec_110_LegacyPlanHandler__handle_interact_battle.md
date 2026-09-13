---

### `110` LegacyPlanHandler::handle_interact_battle — 매 틱 교전 개시 판정 — 오브젝트(모가드/세르펜) 교전·아군 교전 합류(Support)·피격 응전(Response)·라인 다이브·정글캠프 교전·넥서스 공/방 교전·솔로킬 순으로 검사해 self.plan 을 BigPlan::Battle 로 바꾼다

| 항목 | 값 |
|---|---|
| id | `engage__LegacyPlanHandler_handle_interact_battle` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler6engageNtB4_17LegacyPlanHandler22handle_interact_battle` |
| 소스 | `game-ai\src\plan_legacy\handler\engage.rs:233` |
| IR | `m13.ll` 34496~45209행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_interact_battle` · **in:game_ai** |
| 계층 | 플랜 핸들러 |
| exe | `e5d5d0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r11` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut LegacyPlanHandler(6168B) | IR %0: noalias·readonly 없음·dereferenceable(6168) = &mut. plan(0x5e8)·chats(0x7c8) 를 쓴다 | 4 |
| 1 | 2 | version | usize | IR %1 → alloca %112 로 스필. 게이트: >=2(ugt 1) 분기 411·423·535·(solokill 내부) | 4 |
| 2 | 3 | rnd | &mut StdRng(320B) | IR %2 noalias·readonly 없음 = &mut. 본문에서 직접 안 읽고 콜리(strategy/try_engage/update/resolve_join_stake/…)에 전달만 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | IR %3 readonly. info.team(0x930)·info.position 태그(0x9c0)·info.parameter(+0x180) 사용 | 4 |
| 4 | 5 | data | &OperationData(24B) | IR %4 readonly. cache(+0)·context(+8)·blackboard(+0x10) | 4 |
| 5 | 6 | debug | &mut DebugFrameData(224B) | IR %5 noalias·readonly 없음 = &mut. ctx.debug 일 때 infos(+0xa0) HashMap entry push (349·solokill 내부), add_log 는 solokill 내부 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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
//         // is_passive 인라인(types.rs:254~257): 태그→idx = tag-2 (DeathMatch 는 4): PassiveLine(1)|SinglePlanLine(2) → true; PassiveJungle(5) → is_counter_jungle = plan.team[0x638] == plan.player_team[0x640]; 그 외 false
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
// (466~467) objective_waiting_posture = objective_posture.is_some_and(|p| p.should_wait_before_commit())   // 인라인: p.kind[+0x50] 태그 ∈ {3,4}
//                                       || self.team_plan.v27_objective_discipline_blocks_battle(version, data)   // 단락평가(앞이 true 면 호출 안 함)
// (468~484) battle_area_filter: Option<(x,y,range_sq)> = match objective {
//             Some(Morgard{phase,..}) => match phase { Hunt(3) => (470) Some((camp_pos(Morgard), 140000²)), Setup(1)|Assemble(2) => (474) Some((camp_pos(Morgard), 240000²)), None(0) => None, _ => unreachable },
//             Some(Serpen{phase,..})  => match phase { Hunt => (478) Some((camp_pos(Serpen), 140000²)), Setup|Assemble => (482) Some((camp_pos(Serpen), 240000²)), None => None },
//             _ => None }
// (487) steal_session_active = my_pos_idx == 1 && self.team_plan.current_steal_session.is_some() [0x2b2 != 2]
// (488) if !objective_waiting_posture && can_battle_triggered_filtered(version, player, data, battle_area_filter) {
// (489)   if !steal_session_active && !matches!(self.plan, Battle) {
// (490)     if cache.tick() > self.last_dive_abandon_tick [0x1480] + 1 + tps*4 {   // dive_rejoin_cd 인라인(…:973)
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
// (673)   if is_line_phase(ctx, cache.tick()) { → 스킵 }   // 인라인: ctx.tutorial ∈ {0,5,7,8} && tick < setting.epic_jungle.first_spawn_tick.saturating_sub(tps*30)  ⟹ 라인 국면이면 캠프 교전 안 함
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
// (794)         b.main_objective [0xff] = self.team_plan.objective;   // set_main_objective 인라인, 3B 복사
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
// (173)   let Some((e, t)) = check_kill(version, rnd/*IR: poison — 콜리가 안 씀*/, player, data, &self.positioning_score, debug) else { return };
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

**`mem` 메모리 접근 77건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L234 champ 조회·전 절에서 아군/적 팀 인덱스. `ult 2` 바운드체크(panic_bounds_check) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | L234 `as_index()` → my_pos_idx(i32→i64). champ = cache.champions[team][my_pos_idx].unwrap() | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B) | r | L303 aggressive_ratio / L392 roaming_ratio / L802·847 order_ratio 호출 인자 | 4 | OK |  |
| 3 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | champions(+0x1e0=480) 배열 [2][5] Option<&Entity>, game vtable(+0/+8) | 4 | OK |  |
| 4 | OperationData | 0x8 | context (&GameContext) | r | map(+0x20)·setting(+0x8, tick_per_second=+0x12f8)·debug(+0x3b) | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | L402 blackboard[team].big_goal[pos_idx] (+0xf0, stride 32) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x1e0 | champions[2][5] | r | IR gep +480 → [5 x ptr] × team. 이터레이터 iter_champions 가 전부 5회 언롤 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 7 | AbstractGame vtable | 0x28 | tick() | r | L412·490·493·537·(solokill) — 현재 틱 | 4 | 확인불가(vtable 슬롯) |  |
| 8 | AbstractGame vtable | 0x40 | as_moba() → Option<&MobaMode> | r | L256·267·358·515·568 (`as_moba@231` 인라인 표시) | 4 | 확인불가(vtable 슬롯) |  |
| 9 | AbstractGame vtable | 0x1f0 | get_entity_by_id(id) → Option<&Entity> | r | L257·267·359·424 (+496) | 4 | 확인불가(vtable 슬롯) |  |
| 10 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | L256·267 첫 원소의 id(+0) 읽음 | 4 | OK |  |
| 11 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | L256·267 `get(0)` → len==0 이면 None | 4 | OK |  |
| 12 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r | L358 | 4 | OK |  |
| 13 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | L358·515(is_empty) | 4 | OK |  |
| 14 | GameContext | 0x20 | map (&MapDef 28112B) | r | camp_pos(map, JungleType, team==0) 호출 L241·339·470·474·478·482·691·695·699·703 | 4 | OK |  |
| 15 | GameContext | 0x8 | setting (&GameSetting) | r | L412·444·490·493·537·541·554 tick_per_second(+0x12f8=4856) | 4 | OK |  |
| 16 | GameSetting | 0x12f8 | tick_per_second | r | ×3(L412·493·537) ×2(shl1, L444·541·554) ×4(shl2, L490) | 4 | OK |  |
| 17 | GameContext | 0x3b | debug (bool) | r | L348 참일 때만 계측 문자열 push | 4 | OK |  |
| 18 | Entity | 0x0 | team@tag (TeamType) | r | champ.team — is_visible_from 인라인: tag 1(Neutral) 이면 무조건 가시 | 4 | OK |  |
| 19 | Entity | 0x8 | team@Player.0 (usize) | r | visible_state 인덱스(`ult 2` 바운드체크) | 4 | OK |  |
| 20 | Entity | 0x38 | visible_state (24B stride) | r | tag 0=Visible 만 통과 (is_visible_from) | 4 | OK |  |
| 21 | Entity | 0x5c0 | id | r | try_engage(target_id)·나머지 콜리 인자 | 4 | OK |  |
| 22 | Entity | 0x660 | x | r | distance_sq 전부 | 4 | OK |  |
| 23 | Entity | 0x668 | y | r | distance_sq 전부 | 4 | OK |  |
| 24 | LegacyPlanHandler | 0x98 | data.epic.epic_ally_tick | r | L266 remain_time | 4 | OK |  |
| 25 | LegacyPlanHandler | 0xf8 | team_plan (&TeamPlan 1064B) | r | can_near_enemies_range/resolve_join_stake/v25_objective_posture/v27_…/BattlePlan::update/tower_dive_is_viable 인자(전부 & — 콜리 시그니처 tcx `& TeamPlan`) | 3 | OK |  |
| 26 | LegacyPlanHandler | 0xf8 | team_plan.ally_battle_stop_tick[0..5]@tag (stride 16: 0xf8,0x108,0x118,0x128,0x138) | r | L505·684·(solokill) tag 0=None 인 아군만 near_allies 로 셈 | 4 | OK |  |
| 27 | LegacyPlanHandler | 0x2b2 | team_plan.current_steal_session@tag (니치 바이트 = enemy_jungler_alive_at_start) | r | L487 `!= 2` = is_some | 4 | OK |  |
| 28 | LegacyPlanHandler | 0x318 | team_plan.last_battle_tick | r | L493 + tps*3 > tick 이면 exit_src=14·후퇴 stance | 4 | OK |  |
| 29 | LegacyPlanHandler | 0x517 | team_plan.objective | r | 0=Morgard 1=Serpen 5=PressEpic … 255=None | 4 | OK |  |
| 30 | LegacyPlanHandler | 0x518 | team_plan.objective 페이로드 +1 (phase / line) | r | Morgard/Serpen.phase(ObjectPhase 0 None 1 Setup 2 Assemble 3 Hunt) 또는 *.line(LineType) | 4 | OK |  |
| 31 | LegacyPlanHandler | 0x519 | team_plan.objective 페이로드 +2 (with_battle / ready) | r | Morgard/Serpen.with_battle bool | 4 | OK |  |
| 32 | LegacyPlanHandler | 0x5e8 | plan | r | 9=Battle 검사 L239·337·391·489·580·627·672·732·785·820·(solokill). `assume ne 6`(니치 무효값) | 4 | OK |  |
| 33 | LegacyPlanHandler | 0x638 | plan@PassiveJungle.0.team | r | L391 is_passive→is_counter_jungle: team == player_team | 4 | OK |  |
| 34 | LegacyPlanHandler | 0x640 | plan@PassiveJungle.0.player_team | r | L391 | 4 | OK |  |
| 35 | LegacyPlanHandler | 0x1480 | last_dive_abandon_tick | r | L490·581·628·825 재진입 쿨다운 | 4 | OK |  |
| 36 | LegacyPlanHandler | 0x1488 | last_response_bail_tick | r | L537 + tps*3 | 4 | OK |  |
| 37 | LegacyPlanHandler | 0x1490 | last_lost_fight.0 (focus id) | r | L411 r3_hold | 4 | OK |  |
| 38 | LegacyPlanHandler | 0x1498 | last_lost_fight.1 (tick) | r | L412 + tps*3 | 4 | OK |  |
| 39 | LegacyPlanHandler | 0x180a | v3_epicops_armed | r | L567 press_line 분기 | 4 | OK |  |
| 40 | LegacyPlanHandler | 0x990 | positioning_score (&PositioningScoreData 2760B) | r | BattlePlan::update 인자(& — tcx `& PositioningScoreData`) | 3 | OK |  |
| 41 | Blackboard | 0xf8 | big_goal[i].1@tag (i=0: 0xf8, stride 32 → 0x118,0x138,0x158,0x178) | r | L402: 태그 5=Battle, +8 focus Option 태그(0x100…), +16 focus 값(0x108…) | 4 | OK |  |
| 42 | Strategy | 0xc | object_battle@tag | r | L309·370 == 0 이면 poke 분기(아군 요구 수 ↑, can_near==0 요구) | 4 | OK |  |
| 43 | Strategy | 0xf | object_finish | r | L255·357 == 0 이면 스틸위험(steal_risk) 검사 | 4 | OK |  |
| 44 | Strategy | 0x11 | focused (FocusedAreaStrategy 1B) | r | L512 is_in_focused_area | 4 | OK |  |
| 45 | BattlePlan | 0x58 | sub_goal@tag | r | 7=End 검사 / 3=KitingBack 4=RunAway (`tag-3 ult 2`) | 4 | OK |  |
| 46 | BattlePlan | 0xa8 | prev_die_eval | r | L444·541·554 > tps*2 | 4 | OK |  |
| 47 | BattlePlan | 0x108 | exit_src | r | L437·451 == 7 검사 | 4 | OK |  |
| 48 | FightPrediction | 0x38 | line (FightLine) | r | L427 ==0 Commit → stake_commit / L428 ==2 Disengage → stake_veto | 4 | OK |  |
| 49 | ObjectivePosture | 0x50 | kind@tag | r | L466 should_wait_before_commit 인라인: tag∈{3,4} → true (variant 이름 = unknown 참조) | 4 | OK |  |
| 50 | AbstractGameWithCache | 0x130 | twin_towers[team] (bumpalo Vec 32B, +0x18 len) | r | L584·630·734 min_by_key 후보 / L784·819 is_empty (+0x148 = [0].len) | 4 | OK |  |
| 51 | AbstractGameWithCache | 0x170 | nexus[team] | r | L590·636·740 or() 폴백 / L783·818 unwrap | 4 | OK |  |
| 52 | AbstractGameWithCache | 0x180 | top_tower[] / +0x1a0 mid_tower / +0x1c0 bottom_tower (line*32) | r | `tower(line)` 인라인(…:1823): 0x180 + line*32 → [team]; .or(+0x190 + line*32 = *_tower2) | 4 | OK |  |
| 53 | AbstractGame vtable | 0xf8 | is_visible(team, id) | r | solokill L153 hidden = !game.is_visible(1-team, jungler.id) (divtable ExpectedGame::is_visible) | 3 | 확인불가(vtable 슬롯) |  |
| 54 | MobaMode | 0x240 | epic_minion_buff_time[team] (remain_epic_time(team) 인라인 …:210) | r | L568 != 0 이면 v3_epic_formation 으로 press_line 결정 | 4 | OK |  |
| 55 | GameContext | 0x38 | tutorial (TutorialType) | r | L673 is_line_phase 인라인(spawn_epic …:263): ∈{None0,MidBottom5,Line7,Total8} 만 라인 국면 가능 | 4 | OK |  |
| 56 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | L673 tick >= first_spawn_tick.saturating_sub(tps*30) 이면 라인 국면 아님(=스킵) | 4 | OK |  |
| 57 | MapDef | 0x1c98 | bushes (usize) | r | solokill L152 in_bush = bushes[clamp(y/32000,29)][clamp(x/32000,29)] != 0 | 4 | OK |  |
| 58 | Entity | 0x68 | ty@tag (EntityType) | r | == 2 Tower 일 때 L613·656·764 dive_tower = ty@Tower.info.ty (+0x128) | 4 | OK |  |
| 59 | Entity | 0x128 | ty@Tower.info.ty (TowerType 1B) | r | dive_tower 값(Option<TowerType>, None=-1) | 4 | OK |  |
| 60 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | hp*100/stat_cached.hp — L677·684 >59(=>=60%), solokill L161 <35. 0 이면 panic_const_div_by_zero | 4 | OK |  |
| 61 | Entity | 0x640 | stat_cached.move_speed | r | closure#6 L273~274 distance(epic)/move_speed = 도달틱 (aux) | 4 | OK |  |
| 62 | BattlePlan | 0x78 | chats.len | r | clear() 사이트 L418·446·524·556·797·842 | 4 | OK |  |
| 63 | BattlePlan | 0xff | main_objective@tag (Option<MainObjective> 3B) | r | L794·839 set_main_objective 인라인(…:350): self.team_plan.objective 3B(i24) 복사 | 4 | OK |  |
| 64 | Strategy | 0xd | tower_press@tag | r | L580 == 0 이면 press-line 다이브 허용 | 4 | OK |  |
| 65 | Strategy | 0xe | morgard_defense | r | L732 `trunc i8→i1` == 1 이면 DefenseLine 다이브 허용 | 4 | OK |  |
| 66 | BigGoal | 0x0 | tag (BigPlan::goal() 반환) | r | solokill L217 `< 2` = Line(0)\|Jungle(1) 일 때만 try_engage | 4 | OK |  |
| 67 | BattlePlan | 0xb8 | dive_join_tick | r | L832 = cache.tick() 저장(스택 battle, 이후 self.plan 으로 memcpy) | 4 | OK |  |
| 68 | BattlePlan | 0xf6 | with_dive | r | L831 = true (넥서스 공격) | 4 | OK |  |
| 69 | BattlePlan | 0x107 | entry_src | r | 쓰기: 5(L415) 3(L492) 10(L788) 11(L834) | 4 | OK |  |
| 70 | BattlePlan | 0xfc | ff_stake_join | r | L451 쓰기(§C) | 4 | OK |  |
| 71 | BattlePlan | 0x0 | support_target | r | L416 = Some(focus_id) | 4 | OK |  |
| 72 | LegacyPlanHandler | 0x5e8 | plan | w | ★전 사이트 동일 패턴: drop_glue(BigPlan)(구 plan 드롭) → store 9 → memcpy 280B. 사이트(소스줄): 314·323·373·379·453·562·615·658·720·766·806·851·858(solokill 인라인). IR: 35762/35768·35800/35806·37186/37192·37222/37228·37851/37857·38905/38911·39993/39999·40976/40982·41845/41851·42895/42901·43299/43305·44647/44653·45112/45118 (각 쌍 = cleanuppad 경로+정상 경로) | 4 | OK | i64 9 (BigPlan::Battle) |
| 73 | LegacyPlanHandler | 0x5f0 | plan@Battle.0 (BattlePlan 280B) | w | 위와 같은 사이트. ★HEAP_SUBST: 구 plan 의 drop_glue 가 먼저 돌아 구 BattlePlan 의 chats/v54_reentry_ticks Vec 등을 해제한다 | 4 | OK | memcpy 280B ← 스택 BattlePlan |
| 74 | LegacyPlanHandler | 0x7d8 | chats.len | w | L804·849 self.chats.push(Chat) — grow_one(0x7c8 cap 비교) 후 원소 24B 저장, len 갱신 (IR 43294·45107). ★HEAP_SUBST: 재할당 지점 grow_one | 4 | OK | len+1 |
| 75 | LegacyPlanHandler | 0x7d0 | chats.ptr[len] | w | push 원소 저장 (IR 43283~43293 · 45096~45106) | 4 | OK | Chat 24B |
| 76 | LegacyPlanHandler | 0x5e8 | plan (solokill 인라인 L225) | w | IR 44647/44653 — handle_solokill 본체가 이 함수에 통째 인라인(별도 define 없음) | 4 | OK | BigPlan::Battle(try_engage 결과) |

**`consts` 상수 70건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 4 | 241 | 태그 | JungleType::Morgard 태그(camp_pos 인자 i8 4). L470·474 도 동일 | 4 |  |
| 1 | 5 | 339 | 태그 | JungleType::Serpen 태그(camp_pos 인자 i8 5). L478·482 도 동일 | 4 |  |
| 2 | 19600000001 | 242 | 임계 | 140000²+1 — `ult` 로 dist_sq <= 140000² (캠프 반경 140k). L340 in_camp_ally 도 동일 | 4 |  |
| 3 | 19600000000 | 291 | 임계 | 140000² — champ_is_in_camp = !(dist_sq > 140000²). L355·470·478 도 동일 | 4 |  |
| 4 | 150000 | 250 | 계수 | can_near_enemies_range 반경 인자(L346·509·858 도 150000) | 4 |  |
| 5 | 25000 | 259 | 미상 | can_enemy_hit_objective 여유 사거리 인자(L361 도 동일, closure#3 은 aux) | 4 |  |
| 6 | 3 | 265 | 태그 | ObjectPhase::Hunt 태그 — Hunt 일 때만 remain_time/epic 도달틱 검사 | 4 |  |
| 7 | 200000 | 298 | 미상 | check_favorable_engage_formation 반경 인자(L368·605·654·716·757 도 200000) | 4 |  |
| 8 | 699 | 311 | 임계 | aggressive_ratio > 699 (=>=700) 이면 요구 아군 수 -1 (poke 2/3, engage 1/2) | 4 |  |
| 9 | 2 | 311 | 임계 | min_allies_poke = aggressive>699 ? 2 : 3 / min_allies_engage(L320) = aggressive>699 ? 1 : 2 | 4 |  |
| 10 | 9 | 239 | 태그 | BigPlan::Battle 메모리 태그 (matches!(self.plan, Battle) 검사·plan 저장) | 4 |  |
| 11 | -1 | 313 | 센티널 | Option<BattlePlan> None 니치(try_engage 반환 +0 == -1) | 4 |  |
| 12 | 255 | 672 | 계수 | Option<MainObjective> None 태그(objective.is_none → 392~455 블록 스킵) [인라인 콜리 줄 391 → 루트 672 · 20차 G] | 4 |  |
| 13 | 100 | 393 | 계수 | range = 150000 + roaming_ratio*100 | 4 |  |
| 14 | 5 | 402 | 태그 | BigGoal::Battle 태그 — 아군 blackboard big_goal 이 Battle{focus:Some} 이면 trigger_focus | 4 |  |
| 15 | 1 | 411 | 임계 | version > 1 (=v2+) 게이트 — r3_hold·stake 계산 | 4 |  |
| 16 | 3 | 412 | 태그 | tps*3 — 같은 focus 에게 진 뒤 3초 홀드(r3_hold) | 4 |  |
| 17 | 5 | 415 | 태그 | battle.entry_src = 5 (Support 합류 진입 코드) | 4 |  |
| 18 | 1 | 414 | 태그 | BattlePlanGoal::Support 태그(+0)와 support_target Some 태그(L416) | 4 |  |
| 19 | 7 | 436 | 태그 | BattleSubPlanGoal::End 태그 (PartialEq::eq 인자) / L437·451 exit_src == 7 | 4 |  |
| 20 | 0 | 427 | 태그 | FightLine::Commit 태그 → stake_commit | 4 |  |
| 21 | 2 | 428 | 태그 | FightLine::Disengage 태그 → stake_veto | 4 |  |
| 22 | -3 | 443 | 계수 | sub_goal 태그-3 `ult 2` = KitingBack(3)\|RunAway(4). L452·536·551 도 동일 | 4 |  |
| 23 | 2 | 447 | 임계 | prev_die_eval > tps*2 (`shl 1`) — L444·541·554 | 4 |  |
| 24 | 57600000000 | 474 | 산출값 | 240000² — Setup/Assemble 국면 battle_area_filter 반경 | 4 |  |
| 25 | 4 | 490 | 태그 | dive_rejoin_cd: tps<<2 = tps*4 (+1) — last_dive_abandon_tick 이후 4초+1틱 | 4 |  |
| 26 | 2 | 491 | 태그 | BattlePlanGoal::Response 태그 | 4 |  |
| 27 | 3 | 492 | 태그 | battle.entry_src = 3 (Response 진입 코드) | 4 |  |
| 28 | 14 | 494 | 산출값 | battle.exit_src = 14 (직전 팀 교전 3초 내 / 고립 후퇴 stance 표식). L517 도 동일 | 4 |  |
| 29 | 14400000001 | 505 | 임계 | 120000²+1 — near_allies/near_enemies 반경 120k (L508 도 동일) | 4 |  |
| 30 | 1152921504606846976 | 515 | 임계 | 2^60 — Vec len assume 상한(컴파일러 아티팩트, 판정 아님) | 4 |  |
| 31 | 5 | 573 | 태그 | MainObjective::PressEpic 태그 → press_line = 그 line | 4 |  |
| 32 | 1 | 569 | 태그 | v3_epic_formation 결과 필터: 태그-1 `ult 2`(=Option None(2)·is_split=true(1)) 이면 버림 → is_split==false 인 formation.line 만 press_line | 4 |  |
| 33 | 9 | 626 | 태그 | MainObjective::Dive 태그 | 4 |  |
| 34 | 3 | 730 | 태그 | MainObjective::DefenseLine 태그 | 4 |  |
| 35 | 2 | 777 | 태그 | MainObjective::Defense 태그(넥서스 방어 절 L779~808) | 4 |  |
| 36 | 4 | 812 | 태그 | MainObjective::Nexus 태그(넥서스 공격 절 L814~853) | 4 |  |
| 37 | 8 | 858 | 태그 | MainObjective::Gank 태그 — handle_solokill 진입 조건(solokill L138) | 4 |  |
| 38 | 40000000001 | 746 | 임계 | 200000²+1 — DefenseLine in_tower_enemy 반경(L752 closure#44 도 동일) / solokill L151 jungler nearby | 4 |  |
| 39 | 25600000001 | 743 | 임계 | 160000²+1 — DefenseLine in_tower_ally 반경 | 4 |  |
| 40 | 22500000001 | 684 | 임계 | 150000²+1 — 캠프 교전 near_allies 반경(L781·816 closure#48/#50 도 150000²) | 4 |  |
| 41 | 59 | 677 | 임계 | hp*100/max_hp > 59 (=HP 60% 이상) — champ(L677)·아군(L684) | 4 |  |
| 42 | 250000 | 679 | 미상 | 캠프 교전 can_near_enemies_range 반경(L679) / closure#38 dist<=250000²(62500000001, aux) | 4 |  |
| 43 | 230000 | 695 | 산출값 | camp_initiate_filter 반경(Setup/Assemble) — 모가드 L695·세르펜 L703 | 4 |  |
| 44 | 180000 | 691 | 산출값 | camp_initiate_filter 반경(Hunt) — 모가드 L691·세르펜 L699 | 4 |  |
| 45 | 30 | 673 | 계수 | tps*30 — 에픽 첫 스폰 30초 전부터 라인 국면 종료(is_line_phase 인라인) | 4 |  |
| 46 | 7 | 673 | 태그 | TutorialType 스위치 값 {0,5,7,8} 만 라인 국면 판정 진행 | 4 |  |
| 47 | 2 | 613 | 태그 | EntityType::Tower 태그 — nearest_tower 가 타워일 때만 dive_tower Some | 4 |  |
| 48 | 0 | 787 | 태그 | BattlePlanGoal::TryKill 태그(넥서스 방/공 L787·830) | 4 |  |
| 49 | 60 | 787 | 산출값 | TryKill.1 = 60 (L830 도 동일) — 소비처 미확인(unknown 참조) | 4 |  |
| 50 | 10 | 788 | 태그 | battle.entry_src = 10 (넥서스 방어 TryKill 진입 코드) | 4 |  |
| 51 | 11 | 834 | 태그 | battle.entry_src = 11 (넥서스 공격 다이브 진입 코드) | 4 |  |
| 52 | 3 | 833 | 태그 | battle.dive_tower = Some(TowerType 태그 3 = TwinA) (넥서스 공격) | 4 |  |
| 53 | 700 | 803 | 계수 | 채팅 확률: gen_range(0..1000) < order_ratio*700/1000 + 300 (L848 도 동일) | 4 |  |
| 54 | 300 | 803 | 미상 | 채팅 확률 기본 30% | 4 |  |
| 55 | 1000 | 803 | 계수 | gen_range 상한 / 나눗셈 | 4 |  |
| 56 | 3 | 804 | 태그 | Chat::Battle 태그 — self.chats.push(Chat::Battle(nearest_enemy.id, 0)) | 4 |  |
| 57 | 35 | 858 | 임계 | solokill L161: 내 라인 근처 최근가시 적 hp% < 35 이면 solokill 판정 중단(return) | 4 |  |
| 58 | 32000 | 858 | 인덱스 | solokill L152 셀 크기(좌표/32000 → bushes 인덱스, clamp 29) | 4 |  |
| 59 | 29 | 858 | 인덱스 | solokill L152 bushes 인덱스 clamp 상한(30×30 격자) | 4 |  |
| 60 | 5 | 858 | 태그 | solokill L186 t(킬 도달틱 − 현재틱) >= tps*5 이면 중단 | 4 |  |
| 61 | 30000 | 196 | 미상 | solokill L196 near_enemies retain: dist <= (max_range_cached+30000)² (aux m01.ll 45456~) [인라인 콜리 줄 858 → 루트 196 · 20차 G] | 4 |  |
| 62 | 40000 | 307 | 미상 | closure#8 L307: 아군 dist(nearest_enemy) <= (max_range_cached(data, ally, nearest_enemy)+40000)² (aux) [인라인 콜리 줄 305 → 루트 307 · 20차 G] | 4 |  |
| 63 | 2 | 858 | 태그 | solokill L217 BigPlan::goal() 태그 < 2 (Line\|Jungle) 일 때만 교전 / L215 can_near_enemies.len()>>1 | 4 |  |
| 64 | 1 | 444 | 임계 | tps*2 — `shl i64 %tps, 1` 로 접힘 (L444·541·554 prev_die_eval > tps*2) | 4 | 2 |
| 65 | 2 | 490 | 임계 | tps*4 — `shl i64 %tps, 2` 로 접힘 (dive_rejoin_cd 인라인 …:973; L490·581·628·825 는 last_dive_abandon_tick + 1 + tps*4) | 4 | 4 |
| 66 | 5 | 583 | 태그 | LineType 태그 × 32 — `shl nuw nsw i8 %line, 5`: cache.top_tower(0x180)/mid(0x1a0)/bottom(0x1c0) 선택 스트라이드(L583·629·733). 판정값 아님 | 4 | 32 |
| 67 | 3 | 584 | 태그 | 포인터 스트라이드 ×8 — `shl nuw nsw i64 %len, 3`(twin_towers 슬라이스 끝 계산). 판정값 아님 | 4 | 8 |
| 68 | 3 | 412 | 태그 | tps*3 — `mul i64 %tps, 3` (L412 r3_hold / L493 last_battle_tick / L537 last_response_bail_tick) | 4 |  |
| 69 | 1 | 490 | 임계 | last_dive_abandon_tick + 1 (+ tps*4) — dive_rejoin_cd 인라인의 +1 | 4 |  |

**`knobs` 조정점 23건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 오브젝트(모가드/세르펜) 캠프 반경 | engage.rs:242·247·291·340·344·355 (140000²=19600000000) | 19600000000 | 올리면 더 먼 아군/적도 '캠프 안'으로 세어 with_battle 오브젝트 교전 개시가 쉬워지고 champ_is_in_camp 게이트도 넓어진다 | 4 | 기존 |
| 1 | 오브젝트 교전 can_near_enemies 탐색 반경 | engage.rs:250·346·509·858(L200) can_near_enemies_range 인자 | 150000 | 올리면 '근접 가능 적'이 늘어 in_camp_ally > can_near+in_camp_enemy 조건이 어려워져 교전 개시 억제 | 4 | 기존 |
| 2 | 캠프 교전(§G) can_near_enemies 반경 | engage.rs:679 | 250000 | 올리면 near_allies > can_near+1 조건이 어려워져 캠프 주변 선제 교전 감소 | 4 | 기존 |
| 3 | 스틸 위험 판정 여유 사거리 | engage.rs:259·361 can_enemy_hit_objective 3번째 인자 | 25000 | 올리면 적이 에픽/세르펜을 칠 수 있다고 더 쉽게 판단 → steal_risk true → (object_finish 태그0 전략일 때) 교전 진행 허용 | 4 | 기존 |
| 4 | 진형 유리 판정 반경 | engage.rs:298·368·605·654·716·757 check_favorable_engage_formation 5번째 인자 | 200000 | 콜리 내부 의미는 콜리 명세 참조 — 여기서는 전달값 | 4 | 기존 |
| 5 | 공격성(aggressive_ratio) 임계 | engage.rs:311·320 | 699 | aggressive_ratio > 699 이면 모가드 교전 요구 아군 수가 poke 3→2 / engage 2→1 로 줄어 더 적은 아군으로 개시 | 4 | 기존 |
| 6 | Support 합류 탐색 반경 | engage.rs:393 150000 + roaming_ratio*100 | 150000 | 올리면(또는 roaming 계수 100 ↑) 더 먼 아군의 Battle 목표에 합류(trigger_focus) — 로밍 성향이 곧 합류 반경 | 4 | 기존 |
| 7 | 같은 focus 재합류 홀드(r3_hold) | engage.rs:412 tps*3 | 3 | 올리면 같은 상대에게 진 뒤 더 오래 합류를 안 한다(v2+) | 4 | 기존 |
| 8 | 다이브 재진입 쿨다운 | engage.rs:490·581·628·825 tps*4 (+1) | 4 | 올리면 다이브 포기 후 Response/press-line/Dive/넥서스 공격 개시가 더 오래 막힌다 | 4 | 기존 |
| 9 | 직전 팀 교전 후 후퇴 stance 창 | engage.rs:493 last_battle_tick + tps*3 | 3 | 올리면 Response 교전이 exit_src=14·후퇴 stance 로 시작되는 창이 길어진다 | 4 | 기존 |
| 10 | Response 근접 아군/적 반경 | engage.rs:505·508 120000² / 858(L206) 동일 | 14400000000 | 올리면 near_allies·near_enemies 둘 다 늘어 near_allies <= can_near+near_enemies(→후퇴 stance) 판정 방향은 데이터 의존 | 4 | 기존 |
| 11 | battle_area_filter 반경 | engage.rs:470·474·478·482 (Hunt 140000² / Setup·Assemble 240000²) | 57600000000 | 올리면 오브젝트 국면에 더 먼 적도 can_battle_triggered 트리거 대상 (docs L402) | 4 | 기존 |
| 12 | 캠프 선제 교전 필터 반경 | engage.rs:691·695·699·703 (Hunt 180000 / Setup·Assemble 230000) | 230000 | 올리면 캠프에서 더 먼 적도 nearest_enemy 후보 | 4 | 기존 |
| 13 | 캠프 교전 HP 하한 | engage.rs:677·684 hp% > 59 | 59 | 올리면 본인/아군이 더 건강해야 캠프 교전 | 4 | 기존 |
| 14 | 라인 국면 종료 선행 | engage.rs:673 first_spawn_tick - tps*30 | 30 | 올리면 에픽 스폰 더 이전부터 캠프 교전(§G) 허용 | 4 | 기존 |
| 15 | DefenseLine 타워권 반경 | engage.rs:743 160000² / 746·752 200000² | 40000000000 | 올리면 방어 다이브 시 아군/적 머릿수 집계 범위 확대 | 4 | 기존 |
| 16 | 넥서스 방/공 적 탐색·넥서스 근접 반경 | engage.rs:781·816 150000² / 784·819 120000² | 22500000000 | 올리면 넥서스 주변 TryKill 개시 범위 확대 | 4 | 기존 |
| 17 | 교전 콜 채팅 확률 | engage.rs:803·848 gen_range(0..1000) < order_ratio*700/1000+300 | 300 | 300↑이면 order_ratio 무관 기본 확률 상승(rnd 소비 1회 — 시드 스트림 영향) | 4 | 기존 |
| 18 | solokill 저체력 중단 임계 | engage.rs:161(inlined) hp% < 35 | 35 | 올리면 라인의 저체력 적이 있을 때 솔킬 판정을 더 자주 포기 | 4 | 기존 |
| 19 | solokill 킬 도달 상한 | engage.rs:186(inlined) tps*5 | 5 | 올리면 check_kill 도달틱이 더 멀어도 솔킬 시도 | 4 | 기존 |
| 20 | solokill 정글러 매복 반경 | engage.rs:151(inlined) 200000² | 40000000000 | 올리면 부시 은신 정글러가 더 멀리 있어도 jungler_ready → 솔킬 스킵 | 4 | 기존 |
| 21 | solokill near_enemies 사거리 여유 | engage.rs:196(inlined, aux m01 45456) max_range_cached+30000 | 30000 | 올리면 더 먼 적도 near_enemies 에 남아 near_allies < can_near/2+near_enemies 로 중단되기 쉬움 | 4 | 기존 |
| 22 | near_ally 사거리 여유(모가드) | engage.rs:307(closure#8, aux m13 59321) max_range_cached+40000 | 40000 | 올리면 near_ally 가 늘어 min_allies 요구 충족이 쉬움 | 4 | 기존 |

<details><summary>`callees` 피호출자 71건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_log | game_core::DebugFrameData::add_log | pub | fn(&mut game_core::DebugFrameData, &game_core::OperationData, &game_core::PlayerState, std::string::String) | game-core\src\simulation\game\frame.rs:54 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | aggressive_ratio | game_core::AthleteParameter::aggressive_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:432 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | can_battle | game_ai::plan_legacy::team_plan::MainObjective::can_battle | pub | fn(&game_ai::plan_legacy::team_plan::MainObjective) -> bool | game-ai\src\plan_legacy\team_plan.rs:1053 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | can_battle_triggered_filtered | game_ai::plan_legacy::handler::can_battle_triggered_filtered | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, std::option::Option<(u64, u64, u64)>) -> bool | game-ai\src\plan_legacy\handler\engage.rs:903 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | can_enemy_hit_objective | game_ai::plan_legacy::old::can_enemy_hit_objective | pub | fn(&game_core::Entity, &game_core::Entity, u64) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1188 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | can_near_enemies_range | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-ai\src\plan_legacy\team_plan.rs:483 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | check_favorable_engage_formation | game_ai::check_favorable_engage_formation | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64) -> bool | game-ai\src\fight_check.rs:1196 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | check_kill | game_ai::plan_legacy::handler::check_kill | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<(usize, usize)> | game-ai\src\plan_legacy\handler.rs:2507 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | clear | game_core::DataTable::<T>::clear | pub | fn(&mut game_core::DataTable<T/#0>) | game-core\src\data.rs:2259 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | dist | game_view::view::projectile::crossbowman::dist | in:game_view::view::projectile::crossbowman | fn(f32, f32, f32, f32) -> f32 | game-view\src\view\projectile\crossbowman.rs:1923 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 18 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 19 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 20 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 21 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | goal | game_ai::plan_legacy::types::BigPlan::goal | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> game_core::BigGoal | game-ai\src\plan_legacy\types.rs:132 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | handle_solokill | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_solokill | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler\engage.rs:136 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 25 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | is_in_focused_area | game_core::FocusedAreaStrategy::is_in_focused_area | pub | fn(&game_core::FocusedAreaStrategy, &game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\strategy.rs:154 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 30 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 31 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | is_passive | game_ai::plan_legacy::types::BigPlan::is_passive | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> bool | game-ai\src\plan_legacy\types.rs:253 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 33 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 35 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 36 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 37 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 38 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | line | game_ai::plan_legacy::types::BigPlan::debug_label::line | in:game_ai::plan_legacy::types | fn(game_core::LineType) -> &str | game-ai\src\plan_legacy\types.rs:83 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 41 | line | game_core::TowerType::line | pub | fn(&game_core::TowerType) -> std::option::Option<game_core::LineType> | game-core\src\simulation\entity\tower.rs:90 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 42 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 43 | new | game_ai::plan_legacy::old::BattlePlan::new | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan | game-ai\src\plan_legacy\old\battle.rs:196 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 44 | open_chase_race_hopeless | game_ai::plan_legacy::old::fight_model::open_chase_race_hopeless | in:game_ai | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 45 | order_ratio | game_core::AthleteParameter::order_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:446 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 46 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 47 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 48 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 49 | remain_epic_time | game_core::MobaMode::remain_epic_time | pub | fn(&game_core::MobaMode, usize) -> usize | game-core\src\simulation\game.rs:210 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 50 | resolve_join_stake | game_ai::plan_legacy::old::fight_model::resolve_join_stake | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::FightPrediction> | game-ai\src\plan_legacy\old\fight_model.rs:680 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 51 | roaming_ratio | game_core::AthleteParameter::roaming_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:425 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 52 | should_wait_before_commit | game_ai::plan_legacy::team_plan::ObjectivePosture::should_wait_before_commit | pub | fn(&game_ai::plan_legacy::team_plan::ObjectivePosture) -> bool | game-ai\src\plan_legacy\team_plan.rs:185 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 53 | start | game_core::prof::start | pub | fn(usize) -> std::option::Option<game_core::prof::ProfTimer> | game-core\src\simulation\prof.rs:175 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 54 | start | game_view::UIPhaseEffect::start | pub | fn(&mut game_view::UIPhaseEffect) | game-view\src\ui\match_ui\phase_effect.rs:30 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 55 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 57 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 58 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 59 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 60 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 61 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 62 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 63 | tower_dive_is_viable | game_ai::plan_legacy::old::tower_dive_is_viable | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::Entity, bool, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:935 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 64 | try_engage | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage | in:game_ai | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> | game-ai\src\plan_legacy\handler\engage.rs:40 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 65 | try_engage_dive | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage_dive | in:game_ai | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> | game-ai\src\plan_legacy\handler\engage.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 66 | update | game_ai::plan_legacy::old::BattlePlan::update | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\battle.rs:427 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 67 | v25_objective_posture | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 68 | v27_objective_discipline_blocks_battle | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_blocks_battle | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:497 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 69 | v2_response_retreat_stance | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v2_response_retreat_stance | in:game_ai | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\handler\engage.rs:13 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 70 | v3_epic_formation | game_ai::plan_legacy::old::v3_epic_formation | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::old::V3EpicFormation> | game-ai\src\plan_legacy\old\epic.rs:774 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 18개**: `champ`, `clamp`, `dist_sq`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `elapsed`, `entry`, `format_inner`, `gen_range`, `grow_one`, `insert_no_grow`, `is_none_or`, `map_or`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `objective_waiting_posture`, `or_insert`, `retain`, `rustc_entry`, `tower2`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m13.ll:20242) · **형제 41개** (LegacyPlanHandler)

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

**`open` 13건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L437 게이트의 소스 표기: IR 은 version<2 경로(%1319)에서 `stake_veto \|\| (!stake_commit && exit_src==7)` 검사를 아예 하지 않는다. `if version>=2 && (…)` 인지 `if let Some(stake)` 블록 안인지는 컬럼 정보 부재로 표기 불가(동작은 확정: v1 은 sub_goal==End 만 reject) | 4 |  |
| 1 | 미탐색 | L568 remain_epic_time(team) 이 MobaMode+0x240 = tcxdict `epic_minion_buff_time[team]` 를 읽는다 — 헬퍼 이름과 필드 이름이 다르다(…:210 인라인). 필드 의미(에픽 버프 잔여 시간?)는 game_core 명세 몫 | 3 |  |
| 2 | 미탐색 | L613·656·764 dive_tower 값 = Entity+0x128 `ty@Tower.info.ty@tag`(TowerType) 로 판정. §J L833 은 리터럴 3(TwinA) 고정 — 왜 TwinA 인지는 소스 주석 부재 | 4 |  |
| 3 | 미탐색 | TryKill.1 = 60 (L787·830) 의 소비처 — 이 함수 안에 없음(BattlePlan::new 내부). knob 등록 보류 | 4 |  |
| 4 | 미탐색 | camp_initiate_filter 의 3번째 원소는 반경 r(제곱 아님, closure#38 이 r*r 계산) / battle_area_filter 는 제곱값 — 두 필터의 타입이 다른 이유는 소스 부재 | 4 |  |
| 5 | 미탐색 | closure#9(L340)·#18/#19(L505·508)·#26/#27(L591·594)·#32/#33(L639·642)·#37(L684)·#42/#43(L743·746)·#12(L360)·#14(L425)·#17(L466)·#21(L568)·#24/#25·#30/#31·#40/#41(min_by_key 타워 키)·#47·#29/#35/#45/#49/#51(min_by_key 키) 는 본체에 인라인돼 별도 define 없음(fnparts: 서브프로그램 575 vs define 35) — 본문 IR 로 읽었고 aux 미등록 | 4 |  |
| 6 | 미탐색 | handle_solokill(engage.rs:136~231)은 이 함수에 통째 인라인이며 `_gaibc` 전체에 독립 define 없음(grep `^define.*handle_solokill` = m01 retain 4건뿐, 그중 2건은 modes.rs single_handle_solokill 소유) ⟹ 별도 명세 대상이 아니라 이 명세가 정본 | 3 |  |
| 7 | 미탐색 | check_kill(L173) 호출에 rnd 자리가 `ptr poison` — 콜리가 rnd 를 안 읽는다는 뜻(IR 확정). check_kill 명세와 대조 필요 | 4 |  |
| 8 | 미탐색 | v2_response_retreat_stance(L498·518) 호출에 self 인자가 없다(tcx 시그니처는 &self) — 내부 fastcc dead-arg 제거. 콜리가 self 필드를 안 읽는다는 뜻 | 3 |  |
| 9 | 미탐색 | ProfTimer phase 86(L235 _t_ob)·85(L669 _t_mg) 의 이름 — prof::PHASE_* 표는 game_core 소관. 계측 전용이라 판정 무관 | 4 |  |
| 10 | 미탐색 | L349·L209~210 debug.infos 에 push 하는 format! 문자열 원문 — @anon 상수 미독(계측 전용) | 4 |  |
| 11 | 미탐색 | L784·819 `twin_towers[t].is_empty()` 는 +0x148+t*32 = twin_towers[t].len 으로 읽음(bumpalo Vec 레이아웃 {bump, ptr, cap, len}=+0x18 len 관례; tcxdict 가 bumpalo 내부를 안 뚫어 관례로 판정 — 검증 방법: 동일 함수 L248 `len@1617` 인라인이 +24 를 len 으로 읽음) | 3 |  |
| 12 | 미탐색 | exit_src=14(L494·517)·entry_src 3/5/10/11 코드표 원문 = battle.rs [ff 계측] 코드표(_docs L652~653) — 이 명세에선 값만 | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L466 ObjectivePosture.kind(+0x50) 태그 3·4 의 variant 이름 — tcxdict 에 `kind` 의 열거형 이름을 안 물었음(should_wait_before_commit 인라인 …:186). 동작(태그∈{3,4}→대기)은 확정 | 3 | 사실 서술 |
| 1 | MainObjective 12 variant 중 L672 can_battle 의 `unreachable`(태그 12+) 은 니치 방어. Repair(7)→false 외 Defense/DefenseLine/Nexus/PressEpic/SplitEpic/Gank/Dive/PressTower → true 는 switch 표에서 확정 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | solokill L193 blackboard 인덱스가 `1 - player.team`(적 팀 blackboard) 인 이유 — IR 확정(aux m01 45456 %28~%30). L158 은 blackboard[team]. 의도(적 시야 기준?)는 주석 부재 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

