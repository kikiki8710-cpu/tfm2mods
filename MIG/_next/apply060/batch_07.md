# apply060 batch_07.md — 반영(logic_060) 브리핑 · 함수 9

★**version = 3 런타임 확정(09-17)** — v<3/v2 경로는 명세에서 제외(사장). 태그/오프셋/vt 슬롯 = `spec_patch_060.md` §A(dispcheck 추가 행 포함). 출력 형식·절차 = `mkapply060.py` 도크스트링. 출력 = `_next/apply060/out/<old>.md`.

RE 정본 파일: 2026-09-17_r21_심층_w10_calculate_action_score_EpicCheck_calc_interaction_Trace_get_input_원문.md · 2026-09-17_r21_심층_w1_check_kill_die_tick_uncached_check_kill_원문.md · 2026-09-17_r21_심층_w3_정글3_JungleSubPlan_PassiveJungle_역정글매복_원문.md · 2026-09-17_r21_심층_w7_TeamPlan3_f10f70_phase핸들러_handle_none_or_gank_handle_chat_원문.md

### `cc4260` → `eafb50` JungleSubPlan::action_candidates (i=193 · 변경·심층(r21 §D: w3 §1))
- 0.5.8 src: `game-ai\src\plan_legacy\sub_plan\jungle.rs:107` · one_line: 정글 캠프(self.camp/team) 사냥 서브플랜 후보 — 위험(논타겟 윈드업·궤적)이면 도주 단일, 아니면 전투·소환물 후보 → 캠프 150000 이내/적에게 보임/적 정글몹 150000 이내면 도주 추가 → 캠프 몹 공격/스킬/스킬2 후보(attack_jungle_action 인라인) → 캠프 이동 1개(move_action 인라인, 적 캠프·아군 진영·경유지 멀면 out_line)
- 0.6.0 판정: **심층(r21)** · 패치 요지: position_score 셀 중심 6인자(parameter 삭제) · v3 RunAway 조건(적 (12f0+32000)² 근접 ‖ e7f810) · v3 카정 move_action(격자 경로 ee0fe0/ee2310 · ecdb90 out_line 1 r16000)
- RE 정본: `2026-09-17_r21_심층_w3_정글3_JungleSubPlan_PassiveJungle_역정글매복_원문.md` · 절: w3 §1
- sig: `fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>`
- consts: [{"value": 22500000001, "src_line": 142, "meaning": "150000² + 1 — (a) 챔프↔캠프 위치 제곱거리 ult(L142) (b) 적 jungles 엔티티↔챔프 제곱거리 ult(L143) → 어느 하나면 RunAway 추가 · 오라클 실행 확증(25차 배치L: 오라클 실행 확인(25차 L): j_camp150k_eq/gt(캠프 dsq 2.25e10/+1) · j_jungles_eq/gt(Bee 몹 dsq 2.25e10/+1) · j_vis(is_visible) 예측 일치)", "kind": "임계", "ev": 2}, {"value": 10000000000, "src_line": 33, "meaning": "100000² — 적 캠프 침투 경유지(Morgard 캠프 L33 / Serpen 캠프 L41)와의 제곱거리 ugt(아군 진영일 때만 검사): 멀면 경유지로 out_line 이동, 아니면 check_move=true 후 캠프로. 오라클 실행 확인: dsq=1e10 → check_move=1 · 1e10+1 → out_line (j_morg_eq/gt · j_serp_eq/gt · j_t1_serp_eq/gt)
- 0.5.8 logic 전문:
```
fn action_candidates(&mut self, version, rnd, player, data, parameter) -> Vec<SmallActionPlay>
L108 bump = data.context.pool; res = Vec::new_in(bump)
L111 team = player.info.team (bounds<2); champ = cache.player_champion[team][player.info.position].unwrap()
L114 enemy_team = 1 - team
     has_non_target_action_range = cache.player_champion[enemy_team].iter().flatten().any(|c|
L115   nontarget_windup_perceived(version, player, data, c) && c.ty==Champion(13) &&
L116   match c.action_state { Skill(4) => e = c.skill_effect.unwrap()(casting==-1 → unwrap_failed) ,
L118                          Skill2(5) => e = (if c.level>2 {c.skill2_effect} else {None}).unwrap() ,
L120                          Ult(6)    => e = (if c.level>4 {c.ult_effect} else {None}).unwrap() , _ => false }
       && matches!(e.casting, Position(1)|Direction(2)) && e.is_in_range(caster=c, target=champ))
L127 ps = position_score_at_position(version, player, data, &parameter.positioning_score, champ.x, champ.y, Objective(11))
L129 if ps.on_trajectory || has_non_target_action_range || ps.on_periodic_trajectory {
L131   res.push(RunAway(new_with_skill(data, player, end_delay=5, with_skill=true)))   // 태그 3
L132   return res }
L136 res.extend(battle_action(version, rnd, player, data, 5))
L137 res.extend(attack_summon_action(player, data))
L140 camp_pos = context.map.camp_pos(self.camp, is_blue_side = self.team==0)
L142 if dist²(champ, camp_pos) < 22500000001 || cache.game.is_visible(enemy_team, champ.id)
L143    || cache.jungles.iter().any(|e| dist²(e, champ) < 22500000001) {   // 단락 평가 순서: 거리 → is_visible → jungles
L144   res.push(RunAway(SmallActionRunAway::new(data, player, 5))) }   // 태그 3
L147 res.extend(attack_jungle_action(data, champ):   // jungle.rs:55~105 인라인
  L55   champ = player_champion[team][pos].unwrap()
  L56   GameMode::Moba(mode) = cache.game.get_game_mode() else unwrap_failed; camp_state = mode.jungle_runner.get_camp_state(self.team, self.camp)
  L59   sub = Vec::new_in(bump)
  L60   move_speed = champ.stat_cached.move_speed
  L62   for id in camp_state.live_list.iter() { target = cache.game.get_entity_by_id(id) (None → continue)
  L63     if champ.can_attack() {
  L64       atk = champ.attack_effect.unwrap()
  L66       max = atk.range + move_speed*30 + champ.stat_buff_cached.range + (champ.level-1)*atk.growth_range + atk.range_adjust(champ, target) + champ.radius() 
  L70       max += target.radius()      // radius() = radius_mult==0 ? radius : radius*(100+mult)/100
  L71       if dist²(target, champ) <= max² { L72 sub.push(Attack(SmallActionAttack::new(data, target.id))) } }   // 태그 15
  L76     if let Some(skill) = &champ.skill_effect (casting != -1) {
  L77       if champ.can_skill() && skill.target.check(champ, target) {
  L78         max = skill.range + move_speed*30 + stat_buff.range + (level-1)*skill.growth_range + skill.range_adjust(champ,target) + champ.radius()
  L82         max += target.radius()
  L84         if dist² <= max² { L85 sub.push(Skill(SmallActionSkill::new(data, target.id))) } } }   // 태그 16
  L90     skill2 = if champ.level>2 {&champ.skill2_effect} else {&None}; if skill2.casting != -1 {
  L91       if champ.can_skill2() && skill2.target.check(champ, target) {
  L92         max = skill2.range + move_speed*30 + (level-1)*skill2.growth_range + stat_buff.range + range_adjust + champ.radius()
  L96         max += target.radius()
  L97         if dist² <= max² { L98 sub.push(Skill2(SmallActionSkill2::new(data, target.id))) } } }   // 태그 17
        }
  L105  sub )
L148 res.push(self.move_action(rnd, data):   // jungle.rs:18~51 인라인
  L20   camp = map.camp_pos(self.camp, self.team==0)
  L22   if player.team == self.team || self.check_move { L51 AroundPosition::new(rnd, data, camp.0, camp.1, 5) }
        else {
  L25     champ = player_champion[team][pos].unwrap()
  L26     if !is_enemy_side(team, champ.x, champ.y) {     // 아군 진영일 때만 경유지 검사 (IR %597 = (team==0) XOR ugt = !is_enemy_side, m02.ll:39720~39722)
  L29       if (self.camp as u8 - 1) < 2 {      // Mushroom(1)|Stump(2) → Morgard 쪽
  L31         (ex,ey) = map.camp_pos(Morgard(4), team==0)
  L33         if dist²(champ,(ex,ey)) > 10000000000 { L34 return AroundPosition::new_with_out_line(rnd, data, ex, ey, 5, Outline(1)) }
            } else {                            // Rhino(0)|Bee(3)|… → Serpen 쪽
  L40         (ex,ey) = map.camp_pos(Serpen(5), team==0)
  L41         if dist² > 10000000000 { L42 return new_with_out_line(rnd, data, ex, ey, 5, Outline(1)) } }
          }
          self.check_move = true   // 적 진영이거나, 아군 진영이면서 경유지 100000 이내
  L51     AroundPosition::new(rnd, data, camp.0, camp.1, 5) } )
L149 return res
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d2f180` → `fa47a0` PassiveJunglePlan::next_plan (i=203 · 변경·심층(r21 §D: w3 §3~4))
- 0.5.8 src: `game-ai\src\plan_legacy\old\passive_jungle.rs:228` · one_line: 패시브 정글러의 다음 큰 플랜 결정: 카정 근접 교전(Battle) → 캠프 클리어 후 리드 행동(딥푸시 갱/라인커버/갱 셋업/카정 캠프 전환) → 아이템 점수 확률 귀환(ActiveRecall) → None
- 0.6.0 판정: **심층(r21)** · 패치 요지: v3 블록 A 매복 유지/해제 · 블록 B 매복 시도(fa2670) · cj_meet 플래그군 · lead_action = fa3350(갱 라인 선택) · v2 는 fa1010+f1dd50 게이트
- RE 정본: `2026-09-17_r21_심층_w3_정글3_JungleSubPlan_PassiveJungle_역정글매복_원문.md` · 절: w3 §3~4
- sig: `fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game`
- consts: [{"value": 2, "src_line": 232, "meaning": "player_champion[team] 바운드(팀 2) · L482/L357 등 반복", "kind": "임계", "ev": 4}, {"value": 100, "src_line": 233, "meaning": "hp*100/stat.hp = hp_ratio(%). L242/L402/L452/L509/L561/aux L370/L638/L655 동일", "kind": "계수", "ev": 4}, {"value": 49, "src_line": 235, "meaning": "카정 교전 진입 hp_ratio > 49 (= 50% 이상) · 오라클 실행 확증(25차 배치J: 오라클 실행 확인(_verify25/J/oracle/o25j.rs · run25j.py · 케이스당 프로세스 1개 · 로그 o25j_<case>.log): bt_hp49(hp 490/1000 → tag -1) vs bt_hp50(500 → tag 9 Battle) — 경계 `> 49` 확정)", "kind": "임계", "ev": 2}, {"value": 14400000001, "src_line": 237, "meaning
- 0.5.8 logic 전문:
```
fn next_plan(&mut self, version, rnd, player, data, goal_data, _positioning_score, team_plan, debug) -> Option<BigPlan>

[절1 · L231~256 카정 근접 교전] (IR 29438~30128)
 if self.team != self.player_team {            // is_counter_jungle() (passive_jungle.rs:104 인라인) — 적 정글에 있을 때만
   champ = data.cache.player_champion[player.team][player.position].unwrap()   // L232 (team<2 바운드·null unwrap 패닉)
   hp_ratio = champ.hp*100 / champ.stat_cached.hp                              // L233 (div0 패닉)
   if hp_ratio > 49 {                                                          // L235
     team = 1 - player.team                                                    // L236 적팀
     near_enemy = cache.iter_champions(team)                                   // L237~239 (5슬롯 언롤 + aux fold)
        .filter(|x| dist_sq(x, champ) < 14400000001 && data.blackboard[team].is_recent_visible(game, player, x))
        .min_by_key(|x| dist_sq(x, champ))
     if let Some(enemy) = near_enemy {                                         // L241
       enemy_hp_ratio = enemy.hp*100/enemy.stat_cached.hp                      // L242
       my_level = player.info.level                                            // L244
       enemy_level = cache.player_by_champion_id(enemy.id).map(|p| p.info.level).unwrap_or(1)   // L245~246
       if hp_ratio >= enemy_hp_ratio || my_level > enemy_level {               // L248 (IR 평가순: hp 비교 → 레벨 비교, or)
         battle = BattlePlan::new(version, BattlePlanGoal::TryKill(enemy.id, 60), data, player)   // L249
         battle.entry_src = 9                                                  // L250
         battle.update(version, rnd, player, data, _positioning_score, team_plan, debug)   // L251
         if battle.sub_goal != RunAway(tag 4) {                                // L252
           self.chats.push(Chat::Battle(enemy.id, 0))                          // L253
           return Some(BigPlan::Battle(battle))                                // L254 (sret +0=9, +8..280B memcpy)
         }  // RunAway 면 battle drop 후 아래로
       }
     }
   }
 }

[절2 · L261~300 클리어 판정·아이템·리드행동·확률 귀환]
 if !is_cleared(self.jungle, self.team, version(poison), rnd(poison), player, data, team_plan, 0, debug(poison)) { return None }   // L261 → L300 (+0=-1)
 can_buy_item     = buy_item(version, rnd, player, cache.game, ctx)      // L263 Option<usize>
 can_upgrade_item = upgrade_item(version, rnd, player, cache.game, ctx)  // L264 Option<(usize,usize)>
 item_score = if let Some((_, item)) = can_upgrade_item {               // L265 (+0x10 = .1 = item idx)
     tier = ctx.item_list[item].tier()   // L268 (len 바운드 패닉) vtable+0x70
     if tier==4 {100} else if tier==3 {70} else if tier==2 {50} else {30}   // L269/271/273
   } else if can_buy_item.is_some() { if player.info.items.len()==0 {10} else {0} }   // L278
   else { 0 }
 if let Some(p) = self.lead_action(version, rnd, player, data, goal_data, team_plan, self.last_lead_action_tick, debug) { return Some(p) }   // L285~288 (lead_action = lead_action_v37 래퍼, 전부 인라인)
 gen = rnd.gen_range(0..=100i64)                                         // L291 RangeInclusive<i64>
 if gen < item_score { Some(BigPlan::ActiveRecall) /*+0=8*/ } else { None /*-1*/ }   // L292 (signed slt)

[lead_action_v37 · L386~634 · 인라인 (IR 30202~32683)]
 ego = player.info.parameter.ego_ratio()                                 // L390
 cooldown_sec = 4 - ego/500                                              // L391
 cooldown = tps * cooldown_sec                                            // L392
 if game.tick().saturating_sub(last_lead_action_tick) < cooldown { return None }   // L393~394
 champ = cache.player_champion[player.team][player.position]              // L400 (Option, 바운드 team<2)
 // --- 딥푸시 갱 (L401~429) ---
 if champ.is_some() {
   hp_ratio = champ.hp*100/stat.hp                                        // L402
   eplayer = game.iter_player().find(|p| p.info.team == 1-team)           // L403 (2528B stride 슬라이스)
   hp_gate = 60 - aggressive_ratio()*20/1000                              // L406~407
   if !(hp_ratio < hp_gate) && eplayer.is_some() && !blackboard[team].is_recent_visible(game, eplayer, champ) {   // L408 (IR 순: hp → eplayer → 가시)
     target_lines = [ if champ.x > height - champ.y {Bottom} else {Top}, Mid ]   // L410 (height=setting+0x12c0)
     for line in target_lines {                                            // L416 filter → find 첫 통과
       if !has_deep_pushed_enemy(player, data, goal_data, line) { continue }   // L417
       if !valid_deep_push_gank(version, rnd, player, data, goal_data, line, debug) { continue }   // L418
       self.last_lead_action_tick = tick                                   // L419
       if rnd.gen_range(0..1000usize) < order_ratio()*600/1000 + 400 { self.chats.push(Chat::HideLine(line, 0)) }   // L421~423
       return Some(BigPlan::LineGanker(LineGankerPlan{ chats: Vec::new(), setup_limit: tick+tps, wait_limit: tick+tps*12, line, phase: Setup(7) }))   // L426~429
     }
   }
 }
 // --- L439 시간 게이트 ---
 if ctx.is_line_phase() /* tutorial ∈ {None0, MidBottom5, Line7, Total8} */ && !(tick < setting.epic_jungle.first_spawn_tick.saturating_sub(tps*30)) { return None }   // L439~440 (에픽 첫 스폰 30초 전부터 리드행동 금지)
 strategy = player.strategy(rnd, game)                                   // L443 (rnd 미사용 · is_solorank ? player.info.solorank_strategy(+0x4f8) : game.strategy(team))
 focused_area = strategy.focused  (+0x11)                                // L444
 match strategy.early_jungle (+0x12) {                                   // L445/447

  GrowthAndCover(0) => {                                                 // L451~504
   champ.unwrap(); hp_ratio                                               // L451~452
   if hp_ratio < 70 - aggressive*20/1000 { return None }                  // L454~456
   if !is_side_cleared(self.jungle, self.team, .., 0) { return None }     // L456 인라인(861~868): is_cleared(jungle, team, extra 0) && is_cleared(pair(jungle), team, extra tps*5) · pair: Rhino0↔Bee3, Mushroom1↔Stump2
   target_lines = [side(champ), Mid]                                      // L457
   bb = &blackboard[team]                                                 // L463
   if let Some(line) = target_lines.iter().find(|l| !(0..5).any(|ep| bb.in_big_line(ep, l)) && bb.minion_state(l).minion_count < -2) {   // L464~465 (아군 큰목표가 그 라인에 없고 우리 미니언 수 -3 이하)
     self.last_lead_action_tick = tick                                    // L469
     self.chats.push(Chat::CoverLine(line, 0))                            // L470
     return Some(BigPlan::PassiveLine(PassiveLinePlan::new(line)))        // L471 (전 필드 0/빈 Vec · line)
   }
   if let Some(line) = target_lines.iter().find(s1_0) {                  // L474~496 (aux)
     // s1_0: t = tower(line,team).or(tower2).or(twin_towers[team] 중 line.get_start_position(setting,team) 에 가장 가까운 것).or(nexus[team]).unwrap()   L475~483
     //       front = bb[team].minion_state(line).front_minion? → game.get_entity_by_id(front)?   L488~489
     //       dist_sq(front, t) > 30624999999 (>=175000)   L490
     //       && (0..5).filter(in_big_line(ep,line)).filter_map(player_champion[team][ep]).any(|a| dist_sq(a, front) < 6400000001 (80000))   L485~486
     self.last_lead_action_tick = tick                                    // L497
     self.chats.push(Chat::GankLineCover(line, 0))                        // L498
     return Some(BigPlan::LineGankCover(LineGankCoverPlan{ chats: Vec::new(), wait_limit: tick+tps*10, line }))   // L499
   }
   return None
  }

  Ganking(1) => {                                                        // L508~556
   champ.unwrap(); hp_ratio                                               // L508~509
   eplayer = game.iter_player().find(team==1-team).unwrap()               // L510 (없으면 unwrap 패닉)
   if hp_ratio < 70 - aggressive*20/1000 { return None }                  // L512~514
   if !is_side_cleared(self.jungle, self.team, ..) { return None }        // L514
   if blackboard[team].is_recent_visible(game, eplayer, champ) { return None }   // L514 (내 챔프가 적에게 최근 가시)
   target_lines = [side(champ), Mid]                                      // L516
   lines: Vec<LineType> = target_lines.into_iter().filter(|l| focused_area.is_focused_line(l) /* Top0: l<2 · Bottom1: l!=Top · All2: 항상 */ && valid_gank_line(version, rnd, player, data, team_plan, l, debug)).collect_in(ctx.pool)   // L523~526 (aux s3_0/s4_0)
   //   valid_gank_line(L636~683, aux): enemies = iter_champions(1-team).filter(is_near_line(ctx,x,line) && x.hp_ratio>59 && bb[1-team].is_recent_visible(game,player,x))
   //     nearest = enemies.min_by_key(dist to champ)?  (없으면 false)   L643~646
   //     allies = (0..5).filter(bb[team].in_big_line(i,line)).filter_map(player_champion[team][i]).filter(hp_ratio>40)   L652~655
   //     ms = bb[team].minion_state(line)   L657
   //     if enemies.len > allies.len → false   L659
   //     tower = iter_towers_without_nexus(1-team).min_by_key(dist to champ)   L663~665
   //     if tower: (ms.minion_count>1 && dist_sq(nearest,tower) < 19600000001) → false ; dist_sq(nearest,tower) < 10000000001 → false   L667/672
   //     if ms.from_mid(+0x10, i64) > 3000 → false ; else true   L678 (★minion_power(+0x18) 아님)
   scored: Vec<(LineType,i32)> = lines.iter().map(|l| (l, evaluate_gank_opportunity_with_score(rnd, player, data, l, 0).1)).collect   // L530~532 (aux s5_0)
   min_gank_score = -40 - (aggressive as i32)*30/1000                    // L537
   best = scored.into_iter().filter(|(_,s)| !(s < min_gank_score)).max_by_key(|(_,s)| s)   // L538~540 (aux s6_0/s7_0)
   let Some((line,_)) = best else { return None }                        // L541
   self.last_lead_action_tick = tick                                      // L545
   if rnd.gen_range(0..1000) < order_ratio()*600/1000+400 { self.chats.push(Chat::HideLine(line, 0)) }   // L547~549
   return Some(BigPlan::LineGanker(LineGankerPlan{ chats: Vec::new(), setup_limit: tick+tps, wait_limit: tick+tps*10, line, phase: Setup(7) }))   // L552~554
  }

  CounterJungle(2) => {                                                  // L560~626
   champ.unwrap(); hp_ratio                                               // L560~561
   if hp_ratio < 70 - aggressive*20/1000 { return None }                  // L562~564
   tick = game.tick(); enemy_team = 1 - player.team                       // L566~567
   if !is_side_cleared(self.jungle, player.team /*★self.team 아님*/, ..) { return None }   // L569
   (camp_a, camp_b) = match focused_area { Top => (Stump2, Mushroom1), Bottom => (Bee3, Rhino0), All => if height-champ.y >= champ.x {(Stump2, Mushroom1)} else {(Bee3, Rhino0)} }   // L571~575
   best: Option<(camp, remain)> = None
   for camp in [camp_a, camp_b] {                                         // L583 (2회 언롤)
     pos = map.camp_pos(camp, player.team == 1)                           // L585 (bool = 「캠프 소유팀이 0 인가」: camp_pos 메모 키 .1 = !bool → true 면 팀0 캠프 좌표. 여기선 소유팀=적팀=1-player.team 이라 player.team==1 ⇔ 적팀==0)
     region = map.regions[pos.y/32000][pos.x/32000]                       // L586 (바운드 30·27)
     rp = cache.region_point[region]; if player.team != 0 { rp = -rp }     // L587
     if rp < -2 { continue }                                              // L588
     respawn = team_plan.next_respawn_tick[enemy_team][camp_idx(camp)]     // L590~591
     travel = distance(champ.xy, camp_pos(camp, team==1)) / champ.stat_cached.move_speed   // L592~594 (div0 패닉)
     state = game.get_game_mode()[Moba].jungle_runner.get_camp_state(enemy_team, camp)   // L596 (Moba 아니면 unwrap — 사장)
     if state.live_list.len != 0 { if best.is_none() { best = (camp, 0) } }   // L597~598 캠프 살아있음
     else if respawn - 1 < travel + tick + tps*2 {                        // L604 (respawn <= 도착시각+2초)
       remain = respawn.saturating_sub(tick)                              // L605
       if best.is_none() || remain < best.1 { best = (camp, remain) }     // L606
     }
   }
   let Some((camp, remain)) = best else { return None }                  // L612
   self.last_lead_action_tick = tick                                      // L613
   if remain == 0 && dist_sq(champ, camp_pos(camp, team==1)) <= 90000000000 { self.chats.push(Chat::CounterJungle(camp, 0)) }   // L619~623 (살아있는 캠프가 300000 이내일 때만 콜)
   self.jungle = camp; self.team = enemy_team                             // L625~626 (★플랜 자체 상태 전환 — 다음 틱부터 is_counter_jungle)
   return None                                                            // L627 (BigPlan 반환 없음)
  }
 }

[has_deep_pushed_enemy · L305~336 인라인]
 t = cache.tower(line, player.team).or(cache.tower2(line, player.team))?   // L307 (None → false)
 if !(t.ty == Tower(2) && t.tower.ty >= 5 /*Top2/Mid2/Bottom2 = 1차 타워 파괴*/) { return false }   // L308
 for ep in 0..5 {                                                        // L316
   Some(ri) = goal_data.enemy_region[ep] else continue
   if tick.saturating_sub(ri.last_known) > tps*3 { continue }            // L318~319
   for idx in 1..=2 { deep = map.line_region(line, team, idx); if map.region_dist[deep][ri.region] < 2 { return true } }   // L324~326
 }
 false

[valid_deep_push_gank · L337~385 인라인 + aux]
 champ = player_champion[team][pos]? (None → false)                     // L340~341
 if hp_ratio < 50 { return false }                                        // L347~348
 enemies = iter_champions(1-team).filter(|x| is_near_line(ctx, x.xy, line) && blackboard[1-team].is_recent_visible(game, player, x)).collect   // L355~357
 if enemies.is_empty() { return false }                                   // L362
 allies = (0..5).filter(|i| blackboard[team].in_big_line(i, line)).filter_map(|i| player_champion[team][i]).filter(|x| hp_ratio(x) > 40).collect   // L367~370
 if allies.is_empty() { return false }                                    // L374
 if enemies.len > allies.len + 1 { return false }                         // L379
 true
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e59b20` → `d5b980` handle_chat_inner (i=104 · 변경·심층(r21 §D: w7 §3))
- 0.5.8 src: `game-ai\src\plan_legacy\handler\chat.rs:41` · one_line: 받은 팀 채팅(Chat 57종)을 종류별로 처리해 team_plan.objective/plan/chats 를 갱신하고 오브젝트 오해 상태를 마킹
- 0.6.0 판정: **심층(r21)** · 패치 요지: Battle/Dive/Help 처리 → d5d700 아웃라인(5.2KB · 미독) · 종류별 무시 게이트 LPH +0x24b8~+0x24d4 · Press 조건 교체 · HideLine v3 사전검증+d56740 · Repair v3 undying
- RE 정본: `2026-09-17_r21_심층_w7_TeamPlan3_f10f70_phase핸들러_handle_none_or_gank_handle_chat_원문.md` · 절: w7 §3
- sig: `fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData)`
- consts: [{"value": 22500000001, "src_line": 152, "meaning": "150000^2 + 1 — distance_sq(target, me) < 이 값 ⟺ 거리 ≤150000 이면 support_target 채택. aux closure#4(:258) 에서도 같은 값으로 '내 근처 150000 안 적 챔피언' 카운트", "kind": "임계", "ev": 4}, {"value": 199999, "src_line": 252, "meaning": "궁 사거리(ult.range + growth*(level-1) + stat_buff_cached.range) > 199999 ⟺ ≥200000 이면 글로벌 궁으로 간주", "kind": "임계", "ev": 4}, {"value": 60, "src_line": 163, "meaning": "hp_th = 60 - aggressive_ratio*20/1000 : 합류 HP% 하한(공격성이 높을수록 낮아짐). :199 에도 동일", "kind": "임계", "ev": 4}, {"value": 20, "src_line": 163, "meaning": "aggressive_ratio 계수(×20/1000
- 0.5.8 logic 전문:
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
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `dcee40` → `f024d0` TeamPlan::handle_none_or_gank_objective (i=107 · 변경·심층(r21 §D: w7 §2))
- 0.5.8 src: `game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830` · one_line: 팀 목표가 None/Gank 일 때 다음 팀 목표(넥서스공격/넥서스수비/라인수비/에픽·세르펜 헌트/타워압박/컴백픽/귀환/에픽·세르펜 셋업/에픽버프 압박)를 우선순위 사슬로 하나 골라 self.objective·plan·chats 에 쓴다
- 0.6.0 판정: **심층(r21)** · 패치 요지: v3 chat 억제 · armed 시 압박탑 생략·L910 직행 · **V6 commit 블록**(gather/cut/take · open_scene · deadline/quorum) · 1005~1094 v≥2 삭제 · efdf40 · 미독 f02ef6..f04f8f
- RE 정본: `2026-09-17_r21_심층_w7_TeamPlan3_f10f70_phase핸들러_handle_none_or_gank_handle_chat_원문.md` · 절: w7 §2
- sig: `fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool`
- consts: [{"value": 1, "src_line": 843, "meaning": "LineType::Mid 태그(line_exists/handle_line_defense 인자). 라인 순회 순서 Mid(1)→Bottom(2)→Top(0)", "kind": "태그", "ev": 4}, {"value": 2, "src_line": 849, "meaning": "LineType::Bottom 태그. 또 BigPlan::ForcePassive 메모리태그(store i64 2, 니치 niche_start=2) · MainObjective::Defense 태그(L838) · ObjectPhase 아님", "kind": "센티널", "ev": 4}, {"value": 0, "src_line": 856, "meaning": "LineType::Top 태그 / MainObjective::Morgard 태그(L879) / Chat 페이로드 usize 0", "kind": "태그", "ev": 4}, {"value": 3, "src_line": 862, "folded_from": 8, "meaning": "tps*8 이 `shl i64 %tps, 3` 로 접힘 — 최근교전 창(8초)
- 0.5.8 logic 전문:
```
// 공통 준비: ctx=data.context(+8), setting=ctx.setting(+8), tps=setting.tick_per_second(0x12f8), game=data.cache.game(vtable 0x28=tick), tutorial=ctx.tutorial(+0x38), team=player.info.team(0x930), enemy=1-team.
// 인라인 술어(전부 tutorial switch): morgard_exists(ctx)=spawn_epic ⟺ tutorial∈{None0,Line7,Total8} / serpen_exists=spawn_serpen ⟺ tutorial∈{0,MidBottom5,7,8} / as_moba()=get_game_mode(vtable 0x40) 결과 태그0 이어야 하고 아니면 unwrap_failed 패닉.
// HP%(e) = e.hp(0x670)*100 / e.stat_cached.hp(0x628)  (max_hp==0 이면 div_by_zero 패닉).

// objective_handlers.rs:831~834  ① 넥서스 공격
if let Some(line) = handle_nexus_attack(version, rnd, player, data, debug) {        // i8, -1=None. self 인자는 poison(콜리가 self 미사용)
  self.objective = Some(Nexus(line));  self.chats.push(AttackNexus(line, 0));  return;
}
// :837~841  ② 넥서스 수비 (handle_nexus_defense 인라인 = objective_handlers.rs:1228~1234)
if need_defense_nexus(version, rnd, player, data, debug) {
  let need_count = calculate_nexus_defense_count(version, rnd, player, data, debug);
  if i_am_chosen_defender(player, data, need_count, &[] /*빈 슬라이스 ptr=8,len=0*/) {
    self.objective = Some(Defense);  *plan = ForcePassive;  self.chats.push(DefenseNexus(0));  return;
  }
}
// :843~859  ③ 라인 수비 — Mid→Bottom→Top 순으로 **세 if 가 연속**(return 없음, 뒤 라인이 앞 결과를 덮어씀)
for line in [Mid(1), Bottom(2), Top(0)] {   // 언롤 형태
  if line_exists(ctx, line) && handle_line_defense(version, rnd, player, data, line, debug) {
    self.objective = Some(DefenseLine(line));  self.chats.push(DefenseLine(line, 0));  *plan = ForcePassive;   // 계속 진행
  }
}
// :862  최근 교전 게이트
recent_battle := self.last_battle_tick(0x220) != 0 && game.tick() <= last_battle_tick + tps*8   // IR: (lbt+tps<<3 < tick || lbt==0) 이면 :910 으로 건너뜀
if recent_battle {
  // :866~869
  live_ally  = iter_champions(team).filter(|e| HP%(e) > 39).count();
  live_enemy = iter_champions(enemy).count();          // 필터 없음(존재하는 챔프 수)
  if live_ally > 2 && live_ally >= live_enemy + 2 {
    // :870  epic_available = morgard_exists(ctx) && moba.jungle_runner.epic.live_list.len(0x1a8) != 0 && self.obj_spawn.epic_giveup_tick.is_none()
    // :874~876  serpen_available = serpen_exists(ctx) && serpen.live_list.len(0x1d8) != 0 && serpen_giveup_tick.is_none() && (version <= 1 || !v3_epicops_defer_serpen(version, player, data, goal_data, self))
    //          ⚠극성은 분기 방향: defer_serpen()==true 이면 세르펜을 건너뛰고 :899 로 간다(dbg 이름 serpen_available=%337 은 아티팩트).
    // :878~884
    if epic_available { self.objective = Some(Morgard{phase:Hunt(3), with_battle:true}); chats.push(MorgardHunt(0)); *plan = ForcePassive; return; }
    // :888~894
    else if serpen_available { self.objective = Some(Serpen{phase:Hunt(3), with_battle:true}); chats.push(SerpenHunt(0)); *plan = ForcePassive; return; }
    // :899~903
    else if let Some(press_line) = check_press_tower_opportunity(version, poison, player, data, poison, None(-1), poison) {
      self.objective = Some(PressTower(press_line)); self.press_tower_start_tick = tick; chats.push(PressTower(press_line,0)); *plan = ForcePassive; return;
    }
  }
}
// :910  라인전 종료 게이트 (GameContext::is_line_phase(tick) 인라인 = runner.rs:399 + setting.rs:703~704)
//   통과 조건(IR 관측): tutorial ∈ {0,5,7,8} && tick >= setting.epic_jungle.first_spawn_tick(0x8a8).saturating_sub(tps*30)
//   (추정 소스: `if !ctx.is_line_phase(tick)` — 이름/부호는 인라인돼 표기불가, 조건 자체는 확정)
if <위 조건> {
  // :911  check_recent_kill_lane 인라인(team_plan.rs:1255~1266)
  kill_line = game.kill_logs()(vtable 0x130).iter().rev().find(|log| log.killer_team(0x20)==team && tick.saturating_sub(log.tick(0x18)) <= tps*8)
              .and_then(|log| match log.killed_position(0x2c) { Top→Some(Top0), Mid→Some(Mid1), Bottom|Support→Some(Bottom2), Jungle→None })
              .filter(|l| line_exists(ctx, l));
  if let Some(kill_line) = kill_line {
    // :912  check_opportunistic_tower_push(version, player, data, kill_line) 인라인(team_plan.rs:1273~1345) → bool (tcx sig `fn(usize, &PlayerState, &OperationData, LineType) -> bool`; 아래 '→ None' = false, 참이면 kill_line 그대로 채택)
    //   1273 if !line_exists(ctx,line) → None
    //   1278 epic_near   = morgard_exists && epic.live_list.len==0 && (epic.next_respawn_tick(0x1b0) - tick).sat <= tps*20
    //   1280 serpen_near = serpen_exists && serpen.live_list.len==0 && (serpen.next_respawn_tick(0x1e0) - tick).sat <= tps*20
    //   1283 epic_alive = morgard_exists && epic.live_list.len!=0 ; 1284 serpen_alive = serpen_exists && serpen.live_list.len!=0
    //   1285 if epic_near||serpen_near||epic_alive||serpen_alive → None
    //   1290 (t1,t2)=cache.tower(line, enemy) (top/mid/bottom_tower[enemy], _tower2[enemy]); if t1.is_none() && t2.is_none() → None
    //   1295 ms = data.blackboard[team].minion_state(line); 1296 if ms.from_mid(0x10) < 0 || ms.minion_count(0x20) < 1 → None
    //   1300 jp = macro_judgement_penalty(version, player); if ms.from_mid < jp*1000 || ms.minion_count <= jp → None
    //   1305 healthy_allies = iter_champions(team).filter(HP%>49).count()   [Iterator::count 별도 define m09.ll:67786]
    //   1307 required_healthy = min(player_count(ctx),3) → tutorial {0,5,7,8}→3 / {First1,Bottom3}→2 / {TopSolo2,MidSolo4,JungleOnly6}→1 ; if healthy_allies < required_healthy → None
    //   1312~1321 same_side_allies = (0..5).filter(|p| champ[team][p].is_some_and(|c| HP%>49 && match line {Top→(setting.height(0x12c0)-c.y) >= c.x, Mid→is_near_mid_line(ctx,c.x,c.y), Bottom→(height-c.y) <= c.x})).count()  [m09.ll:68915]; if < 2 → None
    //   1327~1332 near_enemies = iter_champions(enemy).filter(|c| blackboard[team].is_recent_visible(game,player,c) && is_near_line(ctx,c.x,c.y,line)).count() [m09.ll:67886]; if > 1 → None
    //   1337~1340 for check_line in valid_lines(ctx) { if check_line==line continue; cms=blackboard[team].minion_state(check_line); if cms.from_mid < -2999 && cms.minion_count < -3 → None }
    //       valid_lines: tutorial {0,7,8}→[Top,Mid,Bottom] / {1,3}→[Bottom] / 2→[Top] / 4→[Mid] / 5→[Mid,Bottom] / 6→[]
    //   else → true
    if check_opportunistic_tower_push(..) { let line = kill_line;
      // :913~916
      self.objective = Some(PressTower(line)); self.press_tower_start_tick = tick; chats.push(PressTower(line,0)); *plan = ForcePassive; return;
    }
    // :923~935  (v2+ 전용) 기지 수비 직후 반격 압박
    else if version > 1 {
      if tick.saturating_sub(goal_data.last_base_defense_tick(0xe8)) <= tps*8 {          // :924 (초과면 건너뜀)
        dead(t) := game.iter_player()(vtable 0x208).filter(|p| p.info.team==t && p.play_state@tag(0x9c8)==Die(0)).count();   // :927~928 closure$1
        if dead(enemy) >= dead(team) + 2 {                                                   // :929  (미만이면 건너뜀)
          if cache.tower(kill_line, enemy).is_some() {                                       // :931  ★여기만 아웃오브라인 AbstractGameWithCache::tower 호출
            // :932~935
            self.objective = Some(PressTower(kill_line)); self.press_tower_start_tick = tick; chats.push(PressTower(kill_line,0)); *plan = ForcePassive; return;
          }
        }
      }
    }
  }
}
// :941~943  컴백 픽 게이트
pick_cooldown_ok = self.comeback_pick_start_tick(0x360)==0 || tick.saturating_sub(comeback_pick_start_tick) > tps*20;
// is_team_behind(version,player,data) 인라인(team_plan.rs:1350~1373):
//   1350 tutorial∈{0,5,7,8} && tick >= first_spawn_tick.sat_sub(tps*30) 아니면 false(:910 과 같은 is_line_phase 구조)
//   1353 elapsed_sec = tick / max(tps,1); 1354 if elapsed_sec < 180 → false
//   1356~1361 my_gold = Σ iter_player().filter(team==mine).gold(0x5e8) ; enemy_gold = Σ (team!=mine).gold ; 1363 gold_behind = (enemy_gold - my_gold) as i32
//   1365 threshold = elapsed<360 ? 2000 : elapsed<540 ? 2500 : 3000 ; 1373 return gold_behind >= threshold
if pick_cooldown_ok && is_team_behind(..) {
  // :944  check_comeback_pick_opportunity(version, rnd, player, data, self, debug) 인라인(team_plan.rs:1382~1580) → Option<LineType>
  //   1382~1388 my_gold/enemy_gold(i32 절단 합)/gold_behind = enemy - my
  //   1391 required_healthy = min(player_count(ctx), gold_behind>5999 ? 4 : 3)  → {0,7,8}: 4|3 / 5: 3 / {1,3}: 2 / {2,4,6}: 1
  //   1394 healthy_allies = iter_champions(team).filter(HP%>49).count(); 1395 if < required_healthy → None
  //   best_line=None, best_score=0 ; required_near = gold_behind>5999 ? 3 : 2
  //   1402 for line in valid_lines(ctx) {
  //     1405 ms=blackboard[team].minion_state(line); if ms.from_mid < -1000 && ms.minion_count < -1 → continue
  //     1415~1427 visible_count=0; for pos in 0..5 { c=champ[enemy][pos]; if c.some && blackboard[team].is_recent_visible(game,player,c) && is_near_line(ctx,c.x,c.y,line) { visible_count+=1; if visible_count==1 { isolated_id=c.id(0x5c0); isolated_pos=pos } } } ; if visible_count != 1 → continue
  //     1431 isolated = game.get_entity_by_id(isolated_id).unwrap() (vtable 0x1f0) ; 1434 if isolated_pos >= 4 (Support) → continue
  //     1448~1452 tower = tower(line,enemy).0.or(.1); if let Some(t)=tower { nexus=cache.nexus[enemy](0x170).unwrap(); if dist_sq(isolated,nexus) < dist_sq(t,nexus) → continue }
  //     1458~1466 allies_on_line_list: bumpalo Vec<usize>(ctx.pool) = (0..5).filter(|p| champ[team][p].is_some_and(|c| HP%>49 && is_near_line(ctx,c.x,c.y,line))) ; allies_on_line=len
  //     1470~1479 near_allies = iter_champions(team).filter(|c| HP%>49 && (is_near_line(ctx,c.x,c.y,line) || match line {Top→(height-y)>=x, Mid→is_near_mid_line, Bottom→(height-y)<=x})).count()
  //     1482 if allies_on_line==0 || near_allies < required_near → continue
  //     1494~1498 enemy_list=vec![isolated_pos]; battle_score = battle_check_with_list(version,rnd,data,player,&allies_on_line_list,&enemy_list,debug); if battle_score < -20 → continue
  //     1504~1506 line_center = line.get_start_position(setting, team); mia = self.can_near_enemies_range(poison,rnd,player,data,line_center.x,line_center.y,300000,poison) (sret 32B bumpalo Vec<&Entity>); if mia.len(+0x18) != 0 → continue
  //     1512~1519 nearby_visible = (0..5).filter(|i| i!=isolated_pos && champ[enemy][i].is_some_and(|c| is_recent_visible(..c) && !is_near_line(ctx,c.x,c.y,line) && distance(c,isolated) < 250001)).count(); if != 0 → continue
  //     1525~1528 total_visible = (0..5).filter(|i| champ[enemy][i].is_some_and(is_recent_visible)).count()
  //     1531~1533 if let Some(t)=tower(line,enemy).0.or(.1) { if dist_sq(isolated,t) < 14400000001 → continue }
  //     1539 score = near_allies*100 ; 1542 += allies_on_line==1 ? 80 : 200 ; 1549 += total_visible>=4 ? 150 : total_visible==3 ? 50 : 0
  //     1556 += match isolated_pos { Top→30, Jungle→100, Mid|Bottom→150 }
  //     1564~1566 enemy_base = line.get_start_position(setting, enemy); score += distance(isolated, enemy_base)/10000
  //     1569~1570 r = HP%(isolated); score += r<61 ? 120 : r<81 ? 50 : 0
  //     1576 if score > best_score { best_line=line; best_score=score } ; 1580 drop(allies_on_line_list)
  //   }
  //   return best_line
  if let Some(pick_line) = check_comeback_pick_opportunity(..) {
    // :945~951 my_gold/enemy_gold (iter_player 합, i32) ; gold_diff = my_gold - enemy_gold
    // :954~958 already_on_line = iter_champions(team).filter(|c| HP%>39 && is_near_line(ctx,c.x,c.y,pick_line)).count()
    // :959 start_ready = already_on_line > 1
    // :960~965
    self.objective = Some(ComebackPick{line:pick_line, ready:start_ready}); self.comeback_pick_start_tick = tick; self.comeback_pick_attempt_count += 1;
    self.comeback_pick_outcomes.push((gold_diff, 0u8)); chats.push(ComebackPick(pick_line,0)); *plan = ForcePassive; return;
  }
}
// :970~980  스폰 준비 귀환   (st_time=12, ed_time=7)
near_serpen_spawn = serpen_exists && serpen.live_list.len==0 && { d=(serpen.next_respawn_tick - tick).sat ; d <= tps*12 && d >= tps*7 }   // :972~974
near_epic_spawn   = morgard_exists && epic.live_list.len==0   && { d=(epic.next_respawn_tick - tick).sat   ; d <= tps*12 && d >= tps*7 }   // :976~978
if near_serpen_spawn || near_epic_spawn {                                                                                                   // :980
  champ = cache.player_champion[team][player.info.position.as_index()].unwrap();   // :981 (None 이면 unwrap_failed)
  hp = HP%(champ);                                                                    // :982
  can_buy_or_upgrade = buy_item(version,rnd,player,game,data?,ctx).is_some() || upgrade_item(version,rnd,player,game,..,ctx).0 as bool;  // :983 (buy_item 결과 태그==1 이면 upgrade_item 미호출)
  skip_for_camp = false;
  if player.info.position == Jungle(1) {                                              // :985
    if let BigPlan::PassiveJungle(jungle) = plan (tag 7) {                            // :986
      camp_live = moba.jungle_runner(+0x18).get_jungle_live_list(jungle.jungle(plan+0x68), jungle.team(plan+0x50)==0);   // :987 Vec<usize> 힙
      if !camp_live.is_empty() {                                                       // :988
        camp_pos = ctx.map.camp_pos(jungle.jungle, jungle.team==0);                    // :989
        skip_for_camp = dist_sq(champ, camp_pos) < 14400000001;                        // :990 (캠프 120000 이내)
      }
      drop(camp_live);                                                                  // :994
    }
  }
  if !skip_for_camp {                                                                  // :999
    goal = plan.goal();
    if (goal is Battle(5) && can_buy_or_upgrade) || (goal !is Battle && (hp < 61 || can_buy_or_upgrade)) {
      *plan = ActiveRecall;  return;                                                    // :1000 (drop_glue 후 store 8)
    }
  }
}
// :1005~1046  에픽 셋업
if morgard_exists(ctx) && check_epic_setup(version, poison, player, data, self, debug) {
  if self.should_delay_morgard_for_wave_priority(player, data, goal_data) { return; }                       // :1006
  if check_epic_giveup(version, poison, player, data, self, poison) {                                          // :1010
    self.obj_spawn.epic_giveup_tick = Some(tick);                                                              // :1011
    if plan.goal() is Epic(2) { *plan = ForcePassive; }  return;                                               // :1012~1013
  }
  self.objective = Some(Morgard{phase:Setup(1), with_battle:true});                                            // :1018
  if plan.goal() is not Epic(2) {                                                                              // :1022
    skip_force = false;
    if player.info.position == Jungle && plan is PassiveJungle(jungle) {                                        // :1023~1024
      camp_live = get_jungle_live_list(jungle.jungle, jungle.team==0);                                          // :1025
      if !camp_live.is_empty() {                                                                                 // :1026
        camp_pos = map.camp_pos(jungle.jungle, jungle.team==0);                                                  // :1027
        skip_force = cache.player_champion[team][1 /*Jungle*/].is_some_and(|c| closure s5_0(c.x, c.y));          // :1028~1033 = dist_sq(c,camp_pos) < 14400000001 && camp_live.iter().any(|&id| game.get_entity_by_id(id).is_some_and(|e| e.hp < e.stat_cached.hp))
      }
      drop(camp_live);                                                                                            // :1037
    }
    if !skip_force { *plan = ForcePassive; }                                                                     // :1041~1042
  }
  self.chats.push(MorgardSetup(0));  return;                                                                     // :1045~1046
}
// :1049~1094  세르펜 셋업 (에픽과 동형)
if serpen_exists(ctx) && check_serpen_setup(version, rnd, player, data, self, debug) {
  if version > 1 && v3_epicops_defer_serpen(version, player, data, goal_data, self) { goto L1097; }            // :1052
  if self.should_delay_serpen_for_wave_priority(player, data, goal_data) { return; }                           // :1053
  if check_serpen_giveup(version, rnd, player, data, self, debug) {                                             // :1057
    self.obj_spawn.serpen_giveup_tick = Some(tick);  if plan.goal() is Serpen(3) { *plan = ForcePassive; }  return;   // :1058~1060
  }
  self.objective = Some(Serpen{phase:Setup(1), with_battle:true});                                              // :1065
  if plan.goal() is not Serpen(3) {                                                                             // :1069
    skip_force = (position==Jungle && plan is PassiveJungle && !camp_live.is_empty() && champ[team][1].is_some_and(closure s6_0));   // :1070~1080 (s5_0 과 동일 본문)
    if !skip_force { *plan = ForcePassive; }                                                                     // :1088~1089
  }
  self.chats.push(SerpenSetup(0));  return;                                                                      // :1093~1094
}
// :1097~1108  에픽 버프 없을 때 타워 압박
if moba.epic_minion_buff_time[team](0x240+team*8) == 0 {                                                        // :1097 remain_epic_time 인라인
  epic_available   = morgard_exists && epic.live_list.len!=0   && epic_giveup_tick.is_none();                   // :1098
  serpen_available = serpen_exists  && serpen.live_list.len!=0 && serpen_giveup_tick.is_none();                 // :1100
  if !(epic_available || serpen_available) {                                                                    // :1103
    if let Some(press_line) = check_press_tower_opportunity(version, poison, player, data, poison, None, poison) {   // :1104
      self.objective = Some(PressTower(press_line)); self.press_tower_start_tick = tick; chats.push(PressTower(press_line,0)); *plan = ForcePassive; return;   // :1105~1108
    }
  }
}
// :1114~1128  에픽 버프 창
live_ally_count = iter_champions(team).count(); live_enemy_count = iter_champions(enemy).count();               // :1114~1115
if moba.epic_minion_buff_time[team] > tps*10 {                                                                   // :1116
  if version > 1 { self.v3_epicops_buff_window(version, rnd, player, data, goal_data, plan); /*bool 무시*/ }     // :1119~1120
  else if live_ally_count >= live_enemy_count { self.handle_press_epic(poison, rnd, player, data, poison); }     // :1123~1124
} else if version > 1 {
  self.v3_press_chat_line = None(-1);                                                                            // :1126~1128
}
return;   // :1131
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `dda220` → `f10f70` TeamPlan::update_objective_after_steal (i=108 · 변경·심층(r21 §D: w7 §1))
- 0.5.8 src: `game-ai\src\plan_legacy\team_plan.rs:910` · one_line: 스틸 추적 분리 후의 팀 목표(MainObjective) 캐스케이드 — 규율 갱신·사망 해제·갱 전처리·오해 수리·귀환 판정·목표별 핸들러 12종을 순서대로 적용해 self.objective/plan 을 갱신
- 0.6.0 판정: **심층(r21)** · 패치 요지: armed 면 v27 discipline **비활성** · V6 track 종료 감시 · 목표 사망(v3 nexus_basis) · **V6 pre-pass**(lock 해제 · enemy_gather_since · pred_dpt/n · window_intent 0..3) · **V6KILL**(레거시 objective 우선) · gank preprocess v3 LineGanker 조건 · 귀환 억제(bb call-vec kind1 · v3 undying · finish_race) · phase switch(양 씬 None 일 때만) · **씬 핸들러 I/J**(Assemble 2↔Hunt 3↔take_born 4 · hopeless/conceded/deadline) · Repair v3 v4_repair_low · phase 핸들러 표 12 · eff930 M728 Hunt 교체
- RE 정본: `2026-09-17_r21_심층_w7_TeamPlan3_f10f70_phase핸들러_handle_none_or_gank_handle_chat_원문.md` · 절: w7 §1
- sig: `fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData)`
- consts: [{"value": 19600000001, "src_line": 911, "meaning": "140000²+1 — near_ally_count: 내 챔프 기준 아군 반경(제곱비교 ult) [objective_discipline.rs] [인라인 콜리 줄 383 → 루트 911 · 20차 G]", "kind": "임계", "ev": 4}, {"value": 28900000001, "src_line": 911, "meaning": "170000²+1 — near_enemy_count 반경 [인라인 콜리 줄 387 → 루트 911 · 20차 G]", "kind": "임계", "ev": 4}, {"value": 32400000001, "src_line": 911, "meaning": "180000²+1 — camp_ally_count: 캠프 기준 아군 반경 [인라인 콜리 줄 389 → 루트 911 · 20차 G]", "kind": "임계", "ev": 4}, {"value": 36100000001, "src_line": 911, "meaning": "190000²+1 — camp_visible_enemy_count 반경 · R667 my_near_target 반경(
- 0.5.8 logic 전문:
```
// ===== team_plan.rs:911 — self.update_v27_objective_discipline(version, player, data, goal_data, debug)  [인라인 · objective_discipline.rs:334~481 · IR 26294~29353]
// 목적: 오브젝트(Morgard/Serpen) 운영 중인 정글러(내 챔프)의 "규율 상태" self.objective_discipline(0x130, Option<ObjectiveDisciplineState>, 니치 태그 @0x149: 0=SafeWait 1=HardDisengage 2=None) 를 매 틱 갱신
// [D336] target = match self.objective(@0x41f 태그) { 0(Morgard) → JungleType 4(Morgard) · %81=true / 1(Serpen) → JungleType 5(Serpen) · %81=false / 그 외 → self.objective_discipline=None(store i8 2 @0x149, IR 26303) 후 종료 }
// [D341~350] phase = self.objective@0x420(enum+1): phase ∉ {1(Setup), 3(Hunt)} → objective_discipline=None(IR 26323) 종료   (Assemble=2·None=0 은 해제)
// [D355] player.info.position(@0x9c0 i32)==1(Jungle) && self.steal_action(@0x418 태그)==2(Commit) → objective_discipline=None(IR 26348) 종료   (스틸 커밋 중엔 규율 없음)
// [D361] team = player.info.team(@0x930); champ = data.cache.player_champion[team][position](@0x1e0+40*team+8*pos); None → objective_discipline=None(IR 26471) 종료
// [D365] objective_entity = v27_objective_entity(인라인): game_mode=cache.game.get_game_mode()(vtable 0x40) → MobaMode 요구(태그 0 아니면 unwrap_failed 패닉); Morgard 면 mode.jungle_runner.epic.live_list[0](@0x1a0 ptr/@0x1a8 len) · Serpen 이면 serpen.live_list[0](@0x1d0/@0x1d8); 비어 있으면 → None(IR 26497) 종료; id 로 cache.game.get_entity_by_id(vtable 0x1f0) → null 이면 None 종료
// [D370~371] tick = cache.game.tick()(vtable 0x28); tps = data.context.setting.tick_per_second(@0x12f8)
// [D372] hp_ratio = champ.hp(@0x670)*100 / champ.stat_cached.hp(@0x628)  (0 이면 div_by_zero 패닉)
// [D373~376] objective_hp_ratio = obj.hp*100/obj.stat_cached.hp; execute_window = objective_hp_ratio < 36 → 참이면 objective_discipline=None(IR 29041) 종료  (막타 창엔 규율 해제)
// [D380] camp_pos = MapDef::camp_pos(map=context.map(@0x20), target, team==0)   [호출]
// [D381] wait_pos = MapDef::camp_pos(map, Morgard?2(Stump):0(Rhino), team==0)   [호출 · v27_objective_wait_pos 인라인]
// [D382~383 closure#0] near_ally_count = count( player_champion[team][*] 중 hp*100/max_hp > 34 && distance_sq(ally, champ) < 19600000001(=140000²+1) )   (5칸 언롤 · 자기 자신 포함)
// [D384~387 closure#1] near_enemy_count = count( player_champion[1-team][*] 중 is_visible_from(champ.team)(=champ.team 이 Player(t) 면 enemy.visible_state[t] 태그==0 Visible 요구, Neutral 이면 통과) && !is_ignored_well_enemy(version, player, enemy)[호출] && Blackboard::is_recent_visible(&data.blackboard[team], game, ctx, player, enemy)[호출] && distance_sq(enemy, champ) < 28900000001(=170000²+1) )
// [D388~389 closure#2] camp_ally_count = count( 아군 중 hp%>34 && distance_sq(ally, camp_pos) < 32400000001(=180000²+1) )
// [D390~393 closure#3] camp_visible_enemy_count = count( 적 중 hp%>34 && is_visible_from && !is_ignored_well_enemy && distance_sq(enemy, camp_pos) < 36100000001(=190000²+1) )   (※closure#1 과 달리 is_recent_visible 없음)
// [D395~406 closure#4/#5] camp_possible_enemy_count = count over p in 0..5 ( enemy=player_champion[1-team][p] 존재 && hp% >= 50(ult 50 → 제외) && !is_recent_visible(enemy) && { d = distance(self.vision.last_visible_pos[p](@0x230+16p), camp_pos)[호출 utils::distance].saturating_sub(180000); elapsed = tick.saturating_sub(self.vision.last_checked_ticks[p](@0x2d0+8p)); elapsed * enemy.stat_cached.move_speed(@0x640) >= d } )   (안 보이는 적이 마지막 확인 이후 캠프 근처까지 걸어올 수 있었는가)
// [D407] camp_enemy_pressure = camp_visible_enemy_count + camp_possible_enemy_count
// [D408] ally_finish_tick = Morgard ? goal_data.epic.epic_ally_tick(@0x98) : goal_data.serpen.epic_ally_tick(@0xd0)
// [D413] ally_finish_soon = (ally_finish_tick - 1) <u tps*4   ⟺ 1 ≤ ally_finish_tick ≤ 4*tps   (`shl %tps,2`)
// [D415] low_contact   = near_enemy_count != 0 && near_ally_count < 2
// [D416] outnumbered   = near_enemy_count > near_ally_count && near_enemy_count > 1
// [D417] camp_out_vis  = camp_visible_enemy_count >= camp_ally_count+2 && camp_visible_enemy_count > 2
// [D418] camp_out_all  = camp_visible_enemy_count > 1 && camp_enemy_pressure >= camp_ally_count+3 && camp_enemy_pressure > 3
// [D419~428] new_kind 결정 (Option<Kind>: 2=None):
//   if hp_ratio < 35:
//     if hp_ratio < 25:                       [D420~422]
//       if near_enemy_count != 0 || ally_finish_soon → new = ally_finish_soon ? None : HardDisengage(1), release=false → [D437]
//       else → (D426 로)
//     else (25≤hp<35):                        [D422~423]
//       if near_enemy_count != 0 && !ally_finish_soon:
//         if near_ally_count < 2 || outnumbered || camp_out_vis → new = HardDisengage(1), release=false → [D437]
//         else → (D426 로)
//       else → [D425]
//   else → [D425]
//   [D425] if ally_finish_soon → new = None, → [D434]   else → (D426 로)
//   (D426) if low_contact && objective_hp_ratio > 54 → new = SafeWait(0)
//   (D427) elif near_ally_count < 2 && objective_hp_ratio > 49 && outnumbered → SafeWait(0)
//   (D428) elif camp_ally_count < 2 && camp_out_all && objective_hp_ratio > 54 → SafeWait(0)
//   else → None
// [D434~436] if hp_ratio < 40 || near_enemy_count > near_ally_count+1 → release=false  else release = (camp_ally_count+2 >= camp_enemy_pressure)
// [D437] current = self.objective_discipline(@0x130..0x14f, 태그 @0x149):
//   current==None(태그 2): if new.is_some → [D453] self.objective_discipline = Some{wait_pos(@0x130,@0x138)=wait_pos, until_tick(@0x140)=tick + (new==HardDisengage ? tps*3 : tps), target(@0x148)=target, kind(@0x149)=new}   (IR 29229~29238)   else 아무것도 안 함
//   current==Some: [D438] if ally_finish_soon || release || current.target != target || current.until_tick <= tick → [D439] self.objective_discipline = None(store i8 2 @0x149, IR 29146)
//     else if new.is_some: [D440~451] kind' = (new==HardDisengage) ? HardDisengage : current.kind; current.wait_pos = wait_pos; current.until_tick = max(tick + (kind'==HardDisengage ? tps*2 : tps/2), current.until_tick); target 재기록; 통째 store(IR 29202~29212, 패딩 6B memcpy 포함)
//     else (new None, 유지조건) → 변경 없음
// [D466~468] if context.debug(@0x3b): debug.infos(@0xa0 HashMap<usize,Vec<String>>).entry(champ.id(@0x5c0)).or_default().push(format!("v27 objective discipline: {:?} {:?} hp:{} obj:{} near_a:{} near_e:{} camp_a:{} camp_p:{}", target, kind, hp_ratio, objective_hp_ratio, near_ally_count, near_enemy_count, camp_ally_count, camp_enemy_pressure))   [호출 rustc_entry · or_insert_with · format_inner · Vec<String>::push_mut · Vec 힙 재할당 지점]
// ===== team_plan.rs:920~939 — 오브젝트 사망(장기 리스폰) 시 목표 해제  [IR 29354~29469]
// if version > 1 {                                                         // L920 (`icmp ugt %1, 1`)
//   if let Some(moba) = data.cache.game.get_game_mode().as_moba() {         // L921 (vtable 0x40 · 태그 0 = MobaMode)
//     now = game.tick(); limit = tps * 15;                                  // L922~923
//     dead = match self.objective(@0x41f) {                                 // L924
//       Some(Morgard{..}) => moba.jungle_runner.epic.live_list.len(@0x1a8)==0 && moba.jungle_runner.epic.next_respawn_tick(@0x1b0).saturating_sub(now) > limit,     // L926~927
//       Some(Serpen{..})  => moba.jungle_runner.serpen.live_list.len(@0x1d8)==0 && serpen.next_respawn_tick(@0x1e0).saturating_sub(now) > limit,                   // L930~931
//       _ => false };
//     if dead {                                                             // L935
//       self.mf_note_obj_clear(36, now) → self.mf_obj_clear = (36 @0x210, now @0x218)   // L936 (인라인)
//       self.objective = None (store i8 -1 @0x41f)                          // L937   ★None 의 메모리값 = 0xFF(IR 정본, 12 가 아님)
//       if matches!(plan.goal(), BigGoal::Epic(2) | BigGoal::Serpen(3)) { drop(*plan); *plan = BigPlan::ForcePassive (store i64 2 @plan+0) }   // L938~939 [호출 BigPlan::goal sret 24B · drop_glue BigPlan]
//     }
//   }
// }
// ===== team_plan.rs:945~946 — if let Some(Gank{line}) = self.objective { self.handle_gank_preprocess(version, player, data, plan, line) }  (태그 8 · IR 30083~30554 · 아래 §3)
// ===== team_plan.rs:949 — self.repair_misunderstood_objective(player, data, plan, debug)  [인라인 · team_plan.rs:603~720 · IR 29476~30081]
// state = self.objective_misunderstanding(@0x90 Option<ObjectiveMisunderstanding>; 태그 = @0x90 i64: -1 → None, 그 외 = opposite_group_start_tick 의 Option 태그 0/1)   // R604
//   None → return (R604)
// target = match self.objective { Morgard → JungleType 4 / Serpen → 5 / 그 외 → self.objective_misunderstanding = None(store i64 -1 @0x90) return }   // R607~609
// if state.target(@0xb8) != target → objective_misunderstanding = None, return   // R611~612
// phase = self.objective.phase(@0x420); opposite = Morgard?5:4                                 // R0
// opposite_available = (Morgard 목표 → serpen_exists: spawn_serpen(ctx.tutorial(@0x38) ∈ {0,5,7,8}) && moba.jungle_runner.serpen.live_list.len(@0x1d8)!=0 / Serpen 목표 → morgard_exists: spawn_epic(tutorial ∈ {0,7,8} — IR `(t-1) ult 6` 이면 false) && epic.live_list.len(@0x1a8)!=0)   // R634~635
// if !opposite_available → state.opposite_group_start_tick = None(@0x90=0), state.solo_hunt_start_tick = None(@0xa0=0); return   // R638~641
// tick = game.tick(); marker_age = tick.saturating_sub(state.accepted_tick(@0xb0)) / max(tps,1)   // R645~647
// target_camp = MapDef::camp_pos(map, target, team==0); opposite_camp = camp_pos(map, opposite, team==0)   // R649~650 [호출]
// target_allies   = v23_healthy_allies_near_point(player, data, target_camp, 180000, 40)     // R651 [호출]
// opposite_allies = v23_healthy_allies_near_point(player, data, opposite_camp, 200000, 40)   // R652 [호출]
// required_group = player_count(ctx.tutorial): {1,3}→2 · {2,4,6}→1 · {0,5,7,8}→3 (그 외 unreachable)   // R653
// opposite_group_start: if opposite_allies >= required_group && !(target_allies > 1 && opposite_allies < target_allies+2) → Some(기존 Some 이면 유지, 아니면 tick) else None   // R655~658
// target_started = objective_is_damaged(data, target)   // R659 [호출]
// my_near_target = player_champion[team][pos] 존재 && distance_sq(champ, target_camp) < 36100000001(=190000²+1)   // R666~667
// solo_hunt_start: if phase==3(Hunt) && target_started && my_near_target && target_allies < 2 → Some(기존 유지 or tick) else None   // R668~670
// opposite_group_elapsed = opposite_group_start.is_some_and(|s| tick.saturating_sub(s) >= tps*4)   // R678 (`shl tps,2`)
// solo_hunt_elapsed      = solo_hunt_start.is_some_and(|s| tick.saturating_sub(s) >= tps*6)         // R680
// if !(opposite_group_elapsed || solo_hunt_elapsed) → self.objective_misunderstanding = 갱신된 state(두 Option 재기록 @0x90..@0xaf); return   // R681~682
// [수리 발동] new_phase = (opposite_allies >= required_group || objective_is_damaged(data, opposite)) ? Hunt(3) : Setup(1); with_battle = true   // R686 [호출]
// self.objective = Some(opposite 오브젝트: Morgard→Serpen(store i8 1) / Serpen→Morgard(store i8 0)) with {phase=new_phase @0x420, with_battle=1 @0x421} (store i16 259|257)   // R691
// self.objective_misunderstanding = None (store i64 -1 @0x90)   // R696
// goal_matches = plan.goal() == (새 목표 Morgard ? Epic(2) : Serpen(3)); if !goal_matches → drop(*plan); *plan = ForcePassive(2)   // R698~704 [호출 BigPlan::goal · drop_glue]
// if champ 존재 && ctx.debug(@0x3b): debug.infos.entry(champ.id).or_insert(vec![]).push(format!("objective misunderstanding repaired: {:?} -> {:?} by {}", target, opposite, opposite_group_elapsed ? "opposite_group" : "solo_hunt")); …push(format!(" objective misunderstanding age: {}s", marker_age))   // R707~715 [호출 rustc_entry·or_insert·format_inner·push_mut ×2]
// ===== team_plan.rs:945~947 — if let Some(Gank{line}) = self.objective(태그 8) { if self.handle_gank_preprocess(version, player, data, plan, line) { return } }   [인라인 · objective_handlers.rs:1133~1225 · IR 30083~30559 · 반환 true 경로(1827) → ret void / false 경로(1826) → L949 로]
// G1134: if !rule_scope::line_exists(ctx, line)[호출] → mf_note_obj_clear(38, tick): mf_obj_clear=(38 @0x210, tick @0x218); self.objective=None(@0x41f=0xFF); if plan 태그(@plan+0 i64)==10(LineGanker) → drop(*plan); *plan=ForcePassive(2); return true   // G1135~1139
// G1143: if tick.saturating_sub(self.gank_start_tick(@0x350)) > tps*20 → mf_note_obj_clear(39, tick); objective=None; if plan==LineGanker → ForcePassive; return true   // G1144~1148
// G1152: target_laner_pos = match line { Top(0)→0, Mid(1)→2, Bottom(2)→3 } (그 외 unreachable)
// G1158~1160: laner = cache.player_champion[team][target_laner_pos]; cancel_reason = laner None → 2 / laner.hp*100/max_hp < 25 → 1
//   취소 시 (G1166~1170): mf_note_obj_clear(40, tick); objective=None; self.chats(@0xc0 Vec<Chat>).push(Chat{tag 17, byte1=cancel_reason}) [인라인 push · 용량 부족 시 grow_one 호출 = 힙 재할당]; if plan==LineGanker → ForcePassive; return true
// G1175: enemy_tower = cache.top/mid/bottom_tower[1-team](@0x180+0x20*line+8*(1-team)).or(tower2(@0x190+0x20*line+8*(1-team)))
// G1177~1180: tower_target_minion = enemy_tower.and_then(|t| t.ty(@0x68)==Tower(2) && t.ty.Tower.info.nearest_enemy(@0x88 Some) → cache.game.get_entity_by_id(nearest_enemy.1(@0x98)) ).filter(|m| m.ty==Minion(1) && m.team==Player(team))   — 없으면 return false(→ L949 로 계속)
// G1193: tower_attack = enemy_tower.attack_effect(@0x490 · Option 태그 @0x4c0 i32 -1=None → return false)
// G1197~1199: ally_minions_in_range = cache.iter_minions(team)[호출 sret 56B].filter(closure#1: tower_attack/enemy_tower 캡처).count()[호출 count 조각 s_0]; tower_damage = Effect::expected_damage_target(tower_attack, ctx, enemy_tower as &dyn AbstractEntity(데이터 ptr + vtable @anon.138 = <Entity as AbstractEntity>), minion)[호출]
// G1200: if ally_minions_in_range < 2 && !(tower_target_minion.hp(@0x670) > tower_damage) → return false
// G1205: near_allies  = (0..5).filter(closure#2: cache,ctx,player,line 캡처).count()[호출 count 조각 s0_0]
// G1212: near_enemies = iter_champions(1-team).filter(closure#3: game,ctx,blackboard,line,player 캡처).count()[호출 count 조각 s1_0]
// G1218: if near_allies > 1 && near_allies > near_enemies → self.objective = Some(Dive{line}) (store i8 9 @0x41f, line @0x420); self.chats.push(Chat{tag 44, byte1=line, +8=0}) [grow_one 가능]; return true   // G1219~1221
//   else return false
// ===== team_plan.rs:949~950 — if self.repair_misunderstood_objective(player, data, plan, debug) { return }   (위 §R · 수리 발동 경로(1639)만 true → ret)
// ===== team_plan.rs:956~972 — 귀환 판정
// if plan 태그 == 8(ActiveRecall) → L977 로                                           // L956
// else if plan.goal() 태그 == 5(Battle) → L977 로                                        // L957 [호출 BigPlan::goal]
// else if matches!(self.objective, Some(Dive(9)|Defense(2)|DefenseLine(3)|Nexus(4))) → L977 로   // L958
// else if should_recall_to_shop(version, rnd, player, data, goal_data) [호출] {           // L961
//   if plan 태그 == 7(PassiveJungle) {                                                   // L963
//     moba = cache.game.get_game_mode().as_moba().unwrap();                            // L964 (태그≠0 → unwrap_failed 패닉)
//     camp_live: Vec<usize> = JungleRunner::get_jungle_live_list(&moba.jungle_runner(@+0x18), jungle.jungle(@plan+0x68), jungle.team(@plan+0x50)==0) [호출 sret 24B]
//     camp_pos = MapDef::camp_pos(map, jungle.jungle, jungle.team==0)                   // L965 [invoke]
//     skip_for_camp = !camp_live.is_empty() && cache.player_champion[team][pos].is_some_and(|c| distance_sq(c, camp_pos) < 14400000001(=120000²+1))   // L966~967 (closure#0)
//     drop(camp_live)  [Vec<usize> drop_glue 호출 = 힙 해제]                             // L968
//     if skip_for_camp → L977 로                                                         // L971
//   }
//   drop(*plan); *plan = BigPlan::ActiveRecall (store i64 8 @plan+0); return              // L972
// }
// ===== team_plan.rs:977~1022 — match self.objective(@0x41f):
//   None(0xFF) | Some(Gank(8))  → self.handle_none_or_gank_objective(version, rnd, player, data, goal_data, plan, debug)[호출 · 반환 bool 무시]; return   // L1022
//   Some(Repair(7)) → team=player.info.team; if version > 1 { if v3_epicops_repair_need(player→team 승격, data→cache 승격, plan)[호출 · u8] == 0 { mf_note_obj_clear(46, tick); self.objective=None } return }   // L982~985
//                     else { self.handle_repair_objective(self, team, position, cache, context)[호출 · argpromotion] return }   // L988
//   Some(Serpen{phase,..})(1) → §handle_serpen (L991~992)   Some(Morgard{phase,..})(0) → §handle_morgard (L994~995)
//   Some(PressEpic(line))(5) → §press_epic (L997~998)   Some(SplitEpic(line))(6) → §split_epic (L1000~1001)
//   Some(Defense)(2) → §defense (L1003~1004)   Some(Nexus(line))(4) → §nexus (L1006~1007)   Some(DefenseLine(line))(3) → §defense_line (L1009~1010)
//   Some(Dive(line))(9) → §dive (L1012~1013)   Some(PressTower(line))(10) → §press_tower (L1015~1016)   Some(ComebackPick{line,ready})(11) → §comeback_pick (L1018~1019)
//   (모든 arm 은 ret void 로 끝난다 — L1025)
// ===== team_plan.rs:994~995 — Some(Morgard{phase,..}) → self.handle_morgard_objective(version, rnd, player, data, goal_data, plan, debug, phase)  [인라인 · objective_handlers.rs:685~829 · IR 30760~31671]
// 공통 매크로: clear(N) = mf_note_obj_clear(N, tick) → self.mf_obj_clear=(N @0x210, tick @0x218) 후 self.objective=None(0xFF @0x41f) / FP = drop(*plan); *plan=ForcePassive(store i64 2 @plan+0) / chat(T,..) = self.chats(@0xc0).push(Chat{태그 T,..}) [grow_one 재할당 가능]
// M686: if !morgard_exists(ctx, cache) [= spawn_epic: ctx.tutorial(@0x38) ∈ {0,7,8} … 인라인 `(t-7) ult -6`] → clear(35); FP; return   // M687~689
// M697: if version > 1 { moba = get_game_mode().as_moba().unwrap(); if moba.jungle_runner.epic.live_list.len(@0x1a8)==0 && epic.next_respawn_tick(@0x1b0).saturating_sub(tick) > tps*15 → clear(36); if plan.goal()==Epic(2) → FP; return }   // M698~704
// M710: if let Some(line) = self.handle_nexus_attack(version, rnd, player, data, debug)[호출 · i8 -1=None] → self.objective = Nexus(line)(store i8 4 @0x41f, line @0x420); chat(42 AttackNexus{line, +8=0}); return   // M711~712
// M718: if need_defense_nexus(version, rnd, player, data, debug)[호출] { need_count = calculate_nexus_defense_count(version, rnd, player, data, debug)[호출]; if objective_defense_role(version, player, data, JungleType::Morgard(4), need_count)[호출] == GoDefend(1) → self.objective=Defense(store i8 2); FP; chat(43 DefenseNexus{+8=0}); return }   // M719~723
// M728: if phase == Hunt(3) {
//   M729: if (0..5).all(|p| !Blackboard::in_epic(&data.blackboard[team], p)[호출 ×5 언롤]) → self.objective = Morgard{phase=Setup(1) @0x420, with_battle=true @0x421}(store i8 0/1/1); if plan.goal()==Epic → FP; return   // M730~736
//   M741: if min(goal_data.epic.epic_enemy_killed_tick(@0x90), goal_data.epic.epic_ally_killed_tick(@0xa0)) > tps*2 (`shl tps,1`) {   // (dbg 이름 skip_disengage 는 이 비교값 — 참일 때 이탈 판정을 "수행"한다)
//     M744~745: camp_pos = MapDef::camp_pos(map, Morgard(4), team==0); if fight_check::should_disengage_object_hunt(version, player, data, camp_pos, 150000)[호출] → self.objective = Morgard{Setup(1), with_battle=true}; if goal==Epic → FP; chat(34 MorgardSetup{+8=0}); return   // M746~753
//   }
// }
// M759: moba(unwrap); if epic.live_list.len==0 { if epic.next_respawn_tick.saturating_sub(tick) > tps*15 → clear(36); if goal==Epic → FP; return  else return }   // M759~766
// M770: if phase == Setup(1) {
//   M771: if check_epic_giveup(version, rnd, player, data, self, debug)[호출] → [GIVEUP 경로 M772~798]:
//     clear(37); self.obj_spawn.epic_giveup_tick = Some(tick)(@0x50=1, @0x58=tick)   // M772~774
//     if plan 태그==9(Battle) && !BattlePlan::is_end(&plan.battle(@plan+8))[호출] → plan.battle.sub_goal(@plan+0x60) = RunAway(태그 4 · BattleSubPlanGoal)   // M776~778
//     if plan.goal()==Epic → FP   // M782~783
//     moba(unwrap); if epic.live_list.len!=0 → chat(40 MorgardGiveUp)   // M786~787
//     if serpen_exists[spawn_serpen: tutorial ∈ {0,5,7,8} && serpen.live_list.len(@0x1d8)!=0] && self.obj_spawn.serpen_giveup_tick.is_none()(@0x60==0) → return   // M790
//     if let Some(press_line) = check_press_tower_opportunity(version, rnd, player, data, self, Some(Bottom)(i8 2), debug)[호출] → self.objective=PressTower(press_line)(store i8 10, line @0x420); self.press_tower_start_tick(@0x358)=tick; chat(45 PressTower{line,+8=0}); FP; return   // M793~797
//     return
//   M806: v24_setup_ready = self.v24_objective_setup_lane_pressure_ready(version, player, data, goal_data, Morgard(4), debug)[호출]; delay = self.should_delay_morgard_for_wave_priority(player, data, goal_data)[호출]
//   M807~810: if delay && !v24_setup_ready { if goal==Epic → FP; return }
//   M814: if v24_setup_ready || check_epic_hunt(version, rnd, player, data, goal_data, plan, self, debug)[호출 · 단락평가: ready 면 호출 안 함] → self.objective = Morgard{Hunt(3), with_battle=true}(store i8 0/3/1); if goal != Epic → FP; chat(38 MorgardHunt{+8=0}); return   // M815~822
//   return
// } else if phase == Hunt(3) { if check_epic_giveup(...)[호출] → 위 GIVEUP 경로; else return }   // M770~771 (%2136)
// else return   (Assemble/None)
// ===== team_plan.rs:991~992 — Some(Serpen{phase,..}) → self.handle_serpen_objective(version, rnd, player, data, goal_data, plan, debug, phase)  [인라인 · objective_handlers.rs:558~684 · IR 31673~32463 · Morgard 판과 대칭, 차이만 표기]
// S559: if !serpen_exists(ctx) [= spawn_serpen: tutorial ∈ {0,5,7,8} · switch] → clear(32); FP; return   // S560~562
// S566: if let Some(line) = handle_nexus_attack(...)[호출] → objective=Nexus(line)(store i8 4); chat(42 AttackNexus{line,0}); return
// S574: if need_defense_nexus(...)[호출] { need_count = calculate_nexus_defense_count(...)[호출]; if objective_defense_role(version, player, data, JungleType::Serpen(5), need_count)[호출]==GoDefend(1) → objective=Defense(2); FP; chat(43); return }
// S584: moba(unwrap); if moba.jungle_runner.serpen.live_list.len(@0x1d8)==0 { S585: if serpen.next_respawn_tick(@0x1e0).saturating_sub(tick) > tps*15 → clear(33); if goal==Serpen(3) → FP; return  else return }   // ※Morgard 판(M697)과 달리 version 게이트 없음·위치도 다름(nexus/defense 뒤)
// S595: match phase { Hunt(3) → S596.. / Setup(1) → S661.. / 그 외 → return }
// [Hunt] S596: if (0..5).all(|p| !Blackboard::in_serpen(&blackboard[team], p)[호출 ×5]) → objective = Serpen{Setup(1), with_battle=true}(store i8 1/1/1); if goal==Serpen → FP; return   // S597~602
//   S607: if min(goal_data.serpen.epic_enemy_killed_tick(@0xc8), goal_data.serpen.epic_ally_killed_tick(@0xd8)) > tps*2 { S610~611: camp_pos = camp_pos(map, Serpen(5), team==0); if should_disengage_object_hunt(version, player, data, camp_pos, 150000)[호출] → objective=Serpen{Setup,true}; if goal==Serpen → FP; chat(25 SerpenSetup{+8=0}); return }   // S612~619
//   S626: if check_serpen_giveup(version, rnd, player, data, self, debug)[호출] → GIVEUP-S; else return
// [Setup] S626: if check_serpen_giveup(...)[호출] → GIVEUP-S
//   else S661: v24_setup_ready = v24_objective_setup_lane_pressure_ready(self, version, player, data, goal_data, Serpen(5), debug)[호출]; delay = should_delay_serpen_for_wave_priority(self, player, data, goal_data)[호출]
//   S662~665: if delay && !ready { if goal==Serpen → FP; return }
//   S669~677: if ready || check_serpen_hunt(version, rnd, player, data, goal_data, plan, self, debug)[호출] → objective = Serpen{Hunt(3), with_battle=true}; if goal != Serpen → FP; chat(29 SerpenHunt{+8=0}); return   else return
// [GIVEUP-S] S627: clear(34); S629: self.obj_spawn.serpen_giveup_tick = Some(tick)(@0x60=1, @0x68=tick); S631: if goal==Serpen → FP; S635~637: if plan==Battle(9) && !BattlePlan::is_end(plan+8)[호출] → plan.battle.sub_goal(@plan+0x60)=RunAway(4)
//   S641: reason = serpen_giveup_chat_reason(player, data)[호출 · Option<SerpenGiveUpReason> i8, 2=None]; if reason != None → chat(31 SerpenGiveUp{reason})
//   S645: if morgard_exists[spawn_epic(tutorial∈{0,7,8}) && epic.live_list.len(@0x1a8)!=0] && self.obj_spawn.epic_giveup_tick.is_none()(@0x50==0) → return
//   S648~652: if let Some(press_line) = check_press_tower_opportunity(version, rnd, player, data, self, Some(Top)(i8 0), debug)[호출] → objective=PressTower(line)(store i8 10); press_tower_start_tick(@0x358)=tick; chat(45 PressTower{line,0}); FP; return   else return
// ===== team_plan.rs:1003~1004 — Some(Defense) → self.handle_defense_objective(version, rnd, player, data, plan, debug)  [인라인 · objective_handlers.rs:16~27 · IR 32478~32512]
// if need_defense_nexus(version, rnd, player, data, debug)[호출] → return; if plan 태그==9(Battle) → return; else clear(2); FP; return   // D17~23
// ===== team_plan.rs:1006~1007 — Some(Nexus(line)) → self.handle_nexus_objective(version, rnd, player, data, plan, line, debug)  [인라인 · objective_handlers.rs:29~44 · IR 33017~33072]
// if !rule_scope::line_exists(ctx, line)[호출] → clear(3); FP; return   // N30~33
// if !ai::end_check(version, rnd, player, data, line, debug)[호출] → clear(4); FP; return   else return   // N37~40
// ===== team_plan.rs:1009~1010 — Some(DefenseLine(line)) → self.handle_defense_line_objective(version, rnd, player, data, plan, line, debug)  [인라인 · objective_handlers.rs:46~87 · IR 32539~32996]
// DL47~49: if !line_exists(ctx, line)[호출] → clear(5); FP; return
// DL54 (handle_nexus_defense 인라인 1227~1235): if need_defense_nexus(...)[호출] && i_am_chosen_defender(player, data, calculate_nexus_defense_count(...)[호출], &[] (빈 슬라이스))[호출] → DL55~56: self.objective=Defense(store i8 2); chat(43 DefenseNexus{+8=0}); FP; return
// DL61: tower = cache.{top,mid,bottom}_tower[team](@0x180/0x1a0/0x1c0 + 8*team).or({..}_tower2[team](@0x190/0x1b0/0x1d0))
// DL62~67: if tower None → tower = cache.twin_towers[team](@0x130+0x20*team, bumpalo Vec: ptr @+0, len @+24).iter().min_by_key(|t| distance_sq(t, LineType::get_start_position(&line, context.setting(@0x8), team)[호출]))   (closure#0/#1 · map+reduce 인라인, Map::next 호출 1회)
// DL80~81: if tower 여전히 None → clear(7); FP; return
// DL70~71: enemies = cache.champions(1-team, context.pool)[호출 sret 32B bumpalo Vec]; has_near_enemy_champion = enemies.iter().any(|e| Blackboard::is_recent_visible(&blackboard[team], game, ctx, player, e)[invoke] && distance_sq(e, tower) < 90000000001(=300000²+1)); drop(enemies)[bumpalo Vec drop_glue + Entity Drop 호출]   (closure#2)
// DL73~75: if !has_near_enemy_champion → clear(6); FP; return   else return (DL86)
// ===== team_plan.rs:997~998 — Some(PressEpic(line)) → self.handle_press_epic_objective(version, rnd, player, data, goal_data, plan, line, debug)  [인라인 · objective_handlers.rs:253~333 · IR 33075~34112]
// PE254~257: if !morgard_exists(spawn_epic: tutorial∈{0,7,8}) || !line_exists(ctx, line)[호출] → clear(20); FP; return
// PE261~263: if ai::end_check(version, rnd, player, data, line, debug)[호출] → objective=Nexus(line)(store i8 4, line @0x420); chat(42 AttackNexus{line,0}); return
// PE267~269: if need_defense_nexus(...)[호출] && i_am_chosen_defender(player, data, calculate_nexus_defense_count(...)[호출], &[])[호출] → objective=Defense(2); FP; chat(43); return
// PE274~275: goal = plan.goal()[호출]; if goal == Line(0){now_line} && now_line != line → FP   (다른 라인 플랜이면 강제 패시브 후 계속)
// PE280: if goal != Battle(5) {
//   PE281: low_hp_allies = count(player_champion[team][*] 존재 && hp*100/max_hp < 41); live_allies = count(존재); live_enemies = count(player_champion[1-team][*] 존재)   (5칸 언롤 · closure#0)
//   PE285~287: if low_hp_allies > 1 && live_allies <= live_enemies → objective=Repair(store i8 7); chat(23 Repair{+8=0}); return
// }
// PE292~294: if serpen_exists(spawn_serpen: tutorial∈{0,5,7,8}) && check_serpen_setup(version, rnd, player, data, self, debug)[호출] { if should_delay_serpen_for_wave_priority(self, player, data, goal_data)[호출] → return
//   PE297~300: if check_serpen_giveup(version, rnd, player, data, self, debug)[호출] → self.obj_spawn.serpen_giveup_tick = Some(tick)(@0x60=1,@0x68=tick); if goal==Serpen(3) → FP; return
//   PE305~312: objective = Serpen{Setup(1), with_battle=true}(store i8 1/1/1); chat(25 SerpenSetup{+8=0}); if goal != Serpen → FP; return }
// PE317~319: moba(unwrap); if moba.epic_minion_buff_time[team](@0x240+8*team) == 0 → clear(21); return   (remain_epic_time 인라인 · 버프 시간 소진)
// PE321~323: live_ally_count = count(my champs 존재 && hp% > 29); live_enemy_count = count(enemy champs 존재)   (closure#1 · 5칸 언롤)
// PE324~325: if live_ally_count < live_enemy_count → objective=Repair(7); return
// PE329: else self.handle_epic_line_change(version, rnd, player, data, debug, line)[호출]; return
// ===== team_plan.rs:1000~1001 — Some(SplitEpic(line)) → self.handle_split_epic_objective(version, rnd, player, data, goal_data, plan, line, debug)  [인라인 · objective_handlers.rs:334~392 · IR 34114~34483 · PressEpic 의 축소판]
// SE335~338: if !morgard_exists || !line_exists(ctx, line)[호출] → clear(22); FP; return
// SE342~344: if end_check(...)[호출] → objective=Nexus(line); chat(42); return
// SE348~350: if need_defense_nexus && i_am_chosen_defender(…, &[]) → objective=Defense; FP; chat(43); return
// SE355~356: if plan.goal()==Line{now_line} && now_line != line → FP
// SE361~363: if serpen_exists && check_serpen_setup(...)[호출] { if should_delay_serpen_for_wave_priority(...)[호출] → return
//   SE366~369: if check_serpen_giveup(...)[호출] → obj_spawn.serpen_giveup_tick=Some(tick); if goal==Serpen → FP; return
//   SE374~381: objective=Serpen{Setup,true}; chat(25 SerpenSetup); if goal != Serpen → FP; return }
// SE386~388: moba(unwrap); if moba.epic_minion_buff_time[team](@0x240+8*team)==0 → clear(23); return   else return   (※low_hp Repair·handle_epic_line_change 없음)
// ===== team_plan.rs:1012~1013 — Some(Dive(line)) → self.handle_dive_objective(version, rnd, player, data, plan, line, debug)  [인라인 · objective_handlers.rs:88~140 · IR 34486~35389]
// DV89~92: if !line_exists(ctx, line)[호출] → clear(8); FP; return
// DV96~98: if let Some(nexus_line) = handle_nexus_attack(...)[호출] → objective=Nexus(nexus_line); chat(42 AttackNexus{line,0}); return
// DV101~104: if need_defense_nexus && i_am_chosen_defender(…, calculate_nexus_defense_count(…), &[]) → objective=Defense; FP; chat(43); return
// DV108~111: if tick.saturating_sub(self.gank_start_tick(@0x350)) > tps*30 → clear(9); FP; return
// DV116~120: near_allies = Σ_{p∈0..5} [ player_champion[team][p].is_some_and(|c| map_regions::is_near_line(ctx, c.x, c.y, line)[호출] && c.hp*100/max_hp > 39) ]   (closure#0 · 5칸 언롤)
// DV122~126: near_enemies = count( player_champion[1-team][*] 존재 && is_near_line(ctx, e.x, e.y, line)[호출] && Blackboard::is_recent_visible(&blackboard[team], game, ctx, player, e)[호출] )   (closure#1)
// DV128: if near_allies > near_enemies → return (Dive 유지)
// DV129~134: if tick.saturating_sub(gank_start_tick) > tps*20 → clear(10); FP; return   else objective = Gank(line)(store i8 8, line @0x420 — 다이브 → 갱 복귀); return
// ===== team_plan.rs:1015~1016 — Some(PressTower(line)) → self.handle_press_tower_objective(version, rnd, player, data, goal_data, plan, line, debug)  [인라인 · objective_handlers.rs:141~252 · IR 35392~36229]
// PT142~145: if !line_exists(ctx, line)[호출] → clear(11); FP; return
// PT149~151: if let Some(nexus_line) = handle_nexus_attack(...)[호출] → objective=Nexus(nexus_line); chat(42); return   (FP 없음)
// PT154~156: if need_defense_nexus && i_am_chosen_defender(…, &[]) → objective=Defense; FP; chat(43); return
// PT161~163: enemy_team = 1-team; if cache.{top,mid,bottom}_tower[enemy_team].is_none() && {..}_tower2[enemy_team].is_none() → clear(12); FP; return
// PT168~170: if tick.saturating_sub(self.press_tower_start_tick(@0x358)) > tps*30 → clear(13); FP; return
// PT175~176: minion_state = &blackboard[team].{top(@0x0),mid(@0x28),bottom(@0x50)}_minion_state[line]; if minion_state.from_mid(@+0x10 i64) < -1000 {
//   PT181~191: engaged_at_tower = version > 1 && cache.tower(line, enemy_team)[호출 · Option<&Entity>].is_some_and(|t| closure#0(cache, ctx, player)(t.x, t.y))[호출 closure#0 아웃오브라인 조각]; if !engaged_at_tower → clear(14); FP; return }
// PT197~202: for check_line in rule_scope::valid_lines(ctx.tutorial)(인라인 switch m09.ll:35678~35733: {0 None,7 Line,8 Total}→@anon.291=[Top,Mid,Bottom] · 5 MidBottom→@anon.290=[Mid,Bottom] · {1 First,3 Bottom}→@anon.35=[Bottom] · 2 TopSolo→@anon.36=[Top] · 4 MidSolo→@anon.31=[Mid] · 6 JungleOnly→%3773 = 배열 없음(for 루프 통째 생략, PT208 로 직행) · ≥9 unreachable) { if check_line != line { ms = minion_state(check_line); if ms.from_mid < -2999 && ms.minion_count(@+0x20 i32) < -2 → clear(15); FP; return } }
// PT208~209: healthy_allies = iter_champions(team).filter(closure s_0).count()[호출 · 아웃오브라인]; live_enemies = count(player_champion[enemy_team][*] 존재)
// PT211~213: if !(healthy_allies > live_enemies) → clear(16); FP; return
// PT219~227: near_enemies = iter_champions(enemy_team).filter(closure s0_0(cache, game, ctx, blackboard, player, line))[호출 count 조각]; if near_enemies > 1 → clear(17); FP; return
// PT232~234: if serpen_exists(spawn_serpen) && check_serpen_setup(version, rnd, player, data, self, debug)[호출] → clear(18); FP; return
// PT238~240: if morgard_exists(spawn_epic) && check_epic_setup(version, rnd, player, data, self, debug)[호출] → clear(19); FP; return
// PT245~246: if plan.goal()==Line{now_line} && now_line != line → FP;  PT251: return
// ===== team_plan.rs:1018~1019 — Some(ComebackPick{line, ready}) → self.handle_comeback_pick_objective(version, rnd, player, data, goal_data, plan, line, ready, debug)  [인라인 · objective_handlers.rs:393~557 · IR 36231~37462]
// 보조: outcome(V) = self.comeback_pick_outcomes(@0xd8 Vec<(i32,u8)>; len @0xe8, ptr @0xe0).last_mut().map(|l| l.1(@+4 u8) = V)   (1=내 킬 성공 · 2=타임아웃 · 3=적 과다/무성과 · 4=아군 부족 · 5=적 킬 · 6=넥서스로 선점 · 0=진행 중)
// CB394~396: if !line_exists(ctx, line)[호출] → clear(24); FP; return
// CB401~404: if let Some(nexus_line) = handle_nexus_attack(...)[호출] → if last.1==0 { outcome(6) }; objective=Nexus(nexus_line); chat(42); return   (FP 없음)
// CB407~411: if need_defense_nexus && i_am_chosen_defender(…, &[]) → if last.1==0 { outcome(6) }; objective=Defense; FP; chat(43); return
// CB415: pick_start = self.comeback_pick_start_tick(@0x360)
// CB416~420 (closure#0): my_kill_on_line = game.kill_logs()(vtable 0x130 · &[KillLog] 48B stride).iter().rev().any(|log| log.killer_team(@+0x20)==team && log.tick(@+0x18) >= pick_start && tick.saturating_sub(log.tick) <= tps*8(`shl 3`) && (log.killed_position(@+0x2c Position i32)==Jungle(1) || line_of(killed_position: Top(0)→Top, Mid(2)→Mid, Bottom(3)|4→Bottom) == line))
// CB427~431 (closure#1): enemy_kill_on_line = 같은 술어에 killer_team != team
// CB439~452: if my_kill_on_line → self.comeback_pick_success_count(@0x370) += 1; outcome(1); clear(26); FP; → CB454 로 이어짐(return 아님)
// CB440~443: else if enemy_kill_on_line → outcome(5); clear(25); FP; return
// CB479~484: timeout_s = ready ? 20 : 15; if tick.saturating_sub(pick_start) > tps*timeout_s → outcome(2); clear(27); FP; return
// CB488~492: elapsed = tick.saturating_sub(pick_start); grace_period = elapsed < tps*5; in_battle = plan 태그==9(Battle)
// CB493: visible_enemies_in_line = iter_champions(1-team).filter(closure s0_0(game, ctx, context, blackboard, player, line))[호출 count 조각].count()
// CB498: near_allies = (0..5).filter(closure s1_0(cache, ctx, player, line))[호출 count 조각].count()
// CB510~513: if visible_enemies_in_line > 2 → outcome(3); clear(28); FP; return
// CB518: if !(grace_period || in_battle) {
//   CB519~523: if visible_enemies_in_line == 2 && near_allies < 3 → outcome(3); clear(29); FP; return
//   CB527~531: if visible_enemies_in_line == 0 → outcome(3); clear(30); FP; return
//   CB535~539: if near_allies < 2 → outcome(4); clear(31); FP; return }
// CB544: if ready → return
// CB545: allies_on_line = (0..5).filter(closure s2_0(cache, ctx, player, line))[호출 count 조각].count()
// CB551~552: if visible_enemies_in_line != 0 && allies_on_line > 1 → self.objective = ComebackPick{line, ready=true}(store i8 11 @0x41f, line @0x420, 1 @0x421); return   else return
// [분기 CB454~474 — my_kill 성공 후 clear 대신 다음 목표 선택] ※IR 상 4281(FP) 뒤 4289 → 이어짐:
//   CB454: epic_unavailable = moba.epic.live_list.len(@0x1a8)==0 || self.obj_spawn.epic_giveup_tick.is_some()(@0x50 태그)
//   CB456~457: serpen_available = serpen.live_list.len(@0x1d8)!=0 && self.obj_spawn.serpen_giveup_tick.is_none()(@0x60==0)
//   CB459~464: if !epic_unavailable → objective = Morgard{Hunt(3), with_battle=true}(store i8 0/3/1); chat(38 MorgardHunt); return
//   CB465~470: else if serpen_available → objective = Serpen{Hunt(3), with_battle=true}(store i8 1/3/1); chat(29 SerpenHunt); return
//   CB471~474: else if let Some(press_line) = check_press_tower_opportunity(version, rnd, player, data, self, None(i8 -1), debug)[호출] → objective=PressTower(press_line)(store i8 10); press_tower_start_tick(@0x358)=tick; chat(45 PressTower{line,0}); return   else return
// ===== aux — 아웃오브라인 count/술어 조각 9개 (담당 함수에서 호출되는 인라인 핸들러의 클로저 · 전부 m09.ll)
// [gank s_0 · m09 68054~68332] ally_minions_in_range = Filter<Chain<Copied<Iter<&Entity>>×2>(iter_minions 결과), closure#1>::count → 내부 fold 는 m06.ll:7310(Chain::fold) → m11.ll:24927(Copied::fold): 술어 = Effect::is_in_range(tower_attack, enemy_tower, minion)[호출] (obj_handlers.rs:1197)
// [gank s0_0 · m09 68335~68512] near_allies = (0..5).filter(closure#2 @1205~1208: player_champion[team][p].is_some_and(|c| map_regions::is_near_line(ctx, c.x, c.y, line)[호출] && c.hp*100/max_hp > 39)).count()
// [gank s1_0 · m09 66806~66971] near_enemies = iter_champions(1-team).filter(closure#3 @1212~1214: is_near_line(ctx, e.x, e.y, line)[호출] && Blackboard::is_recent_visible(&blackboard[team], game, ctx, player, e)[호출]).count()
// [press_tower closure#0 · m09 3152~3340] engaged_at_tower(tower.x, tower.y) = cache.champions(team, pool)[호출 sret 32B].iter().any(|c| distance_sq(c, tower) < 90000000001(=300000²+1)) || cache.minions(team, pool)[호출].iter().any(|m| distance_sq(m, tower) < 40000000001(=200000²+1))   (obj_handlers.rs:182~187 · bumpalo Vec 2개 drop)
// [press_tower s_0 · m09 67142~67239] healthy_allies = iter_champions(team).filter(closure#1 @208: hp*100/max_hp > 39).count()
// [press_tower s0_0 · m09 66974~67139] near_enemies = iter_champions(enemy_team).filter(closure#2 @219~221: is_recent_visible(...)[호출] && is_near_line(ctx, e.x, e.y, line)[호출]).count()
// [comeback s0_0 · m09 67242~67407] visible_enemies_in_line = iter_champions(1-team).filter(closure#2 @493~495: is_recent_visible[호출] && is_near_line[호출]).count()
// [comeback s1_0 · m09 68515~68732] near_allies = (0..5).filter(closure#3 @498~505: player_champion[team][p].is_some_and(|c| hp%>39 && (is_near_line(ctx,c.x,c.y,line)[호출] || match line { Top(0) → is_top_side: (setting.height(@0x12c0) − c.y) >= c.x · Mid(1) → map_regions::is_near_mid_line(ctx, c.x, c.y)[호출] · Bottom(2) → is_bottom_side: (height − c.y) <= c.x }))).count()
// [comeback s2_0 · m09 68735~68912] allies_on_line = (0..5).filter(closure#4 @545~548: player_champion[team][p].is_some_and(|c| hp%>39 && is_near_line(ctx,c.x,c.y,line)[호출])).count()

```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d5bbf0` → `f77820` calculate_interaction_action_score (i=181 · 변경·심층(r21 §D: w10 §3))
- 0.5.8 src: `game-ai\src\action_score.rs:980` · one_line: 액션 후보 1개(effect, 대상 t)의 상호작용 점수 i64 — 적/아군/자기 대상별 피해·힐·버프·CC 후속·타워·이동 항을 합산
- 0.6.0 판정: **심층(r21)** · 패치 요지: 5지점: **§C projected_minion_damage(v2/v3 공통)** `adj = stance==0?0 : (hp≤stance ‖ dmg%≥50 ‖ (hp%<66&&dmg%≥30) ‖ (hp%<41&&dmg%≥18) ‖ (hp%<26&&dmg%≥10)) ? stance : 0; pmd = adj.sat_sub(current/2)` · **§E lapse_core 8번째 인자** `1000+min(judgement,100)²/20` · **§H/§I expected_shield 유지 v3** `applyed>0 ‖ epic_tank ‖ 적 소액션 tag 6..=9 가 t 조준(f5d740)`(possible 항 삭제) · defensive_crisis 6번째 인자 §H walk_ticks=`(dist−cast_range)/max(ms,1)` / §I 0 · **§H/§I 꼬리 v3** `total −= vt[0x70](ctx,champ,t)*hp_value/max(t.hp,1)` · §A/B/D/F/G 그대로(db0160 3-bool 캐시 동구조)
- RE 정본: `2026-09-17_r21_심층_w10_calculate_action_score_EpicCheck_calc_interaction_Trace_get_input_원문.md` · 절: w10 §3
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, &game_core::Entity, &mut game_core::DebugFrameDa`
- consts: [{"value": 35, "src_line": 987, "meaning": "champ.hp*100 > max(max_hp,1)*35 — HP 35% 초과일 때만 base_defense_focus 판정 (version>1)", "kind": "계수", "ev": 4}, {"value": 100, "src_line": 987, "meaning": "백분율 스케일 (HP% 비교 L987 · radius() mult+100 /100 · L1127 *100 후 min 100 · L1140 /100 · L1200 min(hp_value,100) · L1290 *100)", "kind": "인덱스", "ev": 4}, {"value": 1, "src_line": 987, "folded_from": 2, "meaning": "①umax(x,1) 0나눗셈 하한(L987 max_hp · L1223 tps · L1364 move_speed) ②`shl i64 %tps, 1`=tps*2 (L1091 window 상한) / `lshr 1`=tps/2 (L1090 하한) / `shl 1`=incoming*2 (L1340·L1464) / `lshr 1`=current_damage/
- 0.5.8 logic 전문:
```
// 표기: L = action_score.rs 루트 줄(inlinedAt 루트). 오프셋은 reads 참조. 판정 순서 = IR 블록 순서.
// ---- §A 도입·v3 도주 차단 (L980~1034) ----
team = player.info.team; team>=2 → panic_bounds_check. my_position = player.info.position 태그
champ = data.cache.player_champion[team][my_position].expect(..)  (None → unwrap_failed, 메시지 anon.159 · L1013 인라인 위치)
L985 no_self_risk = if champ.stat_buff_cached.undying { true }
L986   else if version>1 && nexus_final_stand(player,data) { true }
L987   else if version>1 && champ.hp*100 > max(champ.stat_cached.hp,1)*35 { L988 base_defense_focus(player,data) }
       else { false }                                  // version<=1 이면 undying 아니면 false
L991 if let Some(v) = v57_summon_command_score(data,player,champ,action,t) { return v }   // Option<i64> {tag,v}
L1000 if version>1 && t.ty==Champion(13) && t.team != champ.team   // TeamType::eq 인라인(태그 → Player 페이로드)
L1001    && game.get_game_mode() 태그 != DeathMatch(2) {          // reach --gamemode 0: 항상 통과
L1013   if ctx.debug && parameter.v3_turnback_hold {
L1014     _debug.add_log(format!("TBHOLD-CAST T{team} {pos:?} tgt={t.id} d={champ.distance(t)} reach={line_effect_range_with_radii(effect,champ,t)} held={d>reach}")) }   // anon.152
L1019   if champ.distance(t) > line_effect_range_with_radii(effect,champ,t) {          // 사거리 밖
L1020     if parameter.v3_turnback_hold { return -9999999 }
L1023     if !no_self_risk {
L1024       if let Some((_,_,walk_tick)) = line_action_stance(data,champ,t,effect) {   // Option<(u64,u64,usize)>
L1025         if walk_tick != 0 {
L1026~1030      die_enemies: bump Vec<&Entity> = cache.player_champion[1-team].iter_champions()
                    .filter(|e| e.distance_sq(champ) < 22500000001 && data.blackboard[1-player.info.team].is_recent_visible(game,player,e)).collect_in(ctx.pool)   // closure$0 (aux)
L1031           if !die_enemies.is_empty() {
L1032             my_die = check_kill_die_tick(version,_rnd,data,player,champ,&die_enemies,&Vec::new_in(pool),_debug)
L1034             if my_die <= action.duration() + walk_tick { return -9999999 }   // IR: my_die > dur+walk 이면 계속
                } } } } } }
// ---- §B 기본 위험 · 적 타워 집중 (L1050~1082) ----
L1050 applyed_damage = parameter.player.applyed_damage ; L1051 base_damage = parameter.player.risk_damage
L1052 possible_damage = parameter.player.possible_risk(data, action.duration()+30)
L1053 is_visible = game.is_visible(1-team, champ.id)          // 적이 나를 보는가
L1054 if !is_visible { possible_damage /= 3 }   (sdiv)
L1060~1061 near_enemy_tower = cache.iter_towers_without_nexus(1-team).filter(|tw| tw.distance_sq(champ) < 22500000000).min_by_key(|tw| tw.distance_sq(champ))
L1064 can_focused_tower = false
      if let Some(tower) = near_enemy_tower {
L1065   if tower.ty == Tower(2) && L1066 tower.ty@Tower.info.nearest_enemy.is_none() {
L1067     range = effect.range + (champ.level-1)*effect.growth_range + champ.stat_buff_cached.range + effect.range_adjust(champ,tower) + champ.radius() + t.radius()
            // radius() 인라인: if stat_buff_cached.radius_mult==0 {radius} else {radius*(mult+100)/100 (udiv)}
L1068     d = t.distance(champ) ; L1069 d = d.saturating_sub(range)
L1071     tower_range = tower.attack_effect.unwrap().range + (tower.level-1)*growth_range + tower.stat_buff_cached.range   // None → unwrap_failed anon.160
L1072     dist = utils::distance(champ.x,champ.y,tower.x,tower.y)
L1074     can_focused_tower = dist <= d + 15000 + tower_range + tower.radius() + champ.radius()   (icmp ule)
        } }
L1081~1082 if can_focused_tower || t.is_champion() { possible_damage += parameter.player.risk_possible_tower }
// ---- §C 미니언 웨이브 투사 피해 (L1086~1094) ----
L1086 tick = game.tick(); spawn_epic = ctx.tutorial ∈ {None,MidBottom,Line,Total} (인라인 switch);
      line_phase_ok = !(tick < setting.epic_jungle.first_spawn_tick.saturating_sub(tps*30))   // 인라인 scope 이름 is_line_phase — IR 극성: tick >= first_spawn-30초 일 때 진행
L1087 projected_minion_damage = 0; if spawn_epic && line_phase_ok && t.team != champ.team && t.is_champion() {
L1088   if let Some((sx,sy,walk_tick)) = line_action_stance(data,champ,t,effect) {
L1089     a = action.duration() + walk_tick
L1090     window = max(tps/2, a) ; L1091 window = min(tps*2, window)      // lshr/shl 접힘
L1092     current_damage = enemy_minion_wave_risk_damage_at(version,data,champ,champ.x,champ.y,window)
L1093     stance_damage  = enemy_minion_wave_danger_damage_at(version,data,champ,sx,sy,window)
L1094     projected_minion_damage = stance_damage.saturating_sub(current_damage/2) } }
// ---- §D 기본 점수 (L1102~1108) ----
L1102 hp_value = champion_hp_value(data, parameter, &parameter.player)
L1105 base_score = if no_self_risk { -1 }
L1108              else { !( (possible_damage + base_damage + projected_minion_damage) * hp_value / champ.hp ) }   // xor -1 = 비트 NOT = -x-1 · sdiv · champ.hp==0 → div_by_zero panic · 표기(`!x` vs `-1-x`)는 외연 동일
// ---- §E 아군 타워 · 근접 적 · 이동 점수 (L1111~1149) ----
L1111~1112 near_ally_tower = cache.iter_towers_without_nexus(team).filter(|tw| tw.distance_sq(champ) < 22500000000).min_by_key(distance_sq)
L1115 has_near_enemy_champion = cache.player_champion[1-team].iter().flatten().any(|c| c.distance_sq(champ) < 15000000000 && data.blackboard[1-team].is_recent_visible(game,player,c))   // 5슬롯 언롤
L1119 tower_score = 0
      if let Some(tower) = near_ally_tower {
L1121   tower_atk = tower.attack_effect.as_ref().unwrap()   // None → unwrap_failed anon.162
        if has_near_enemy_champion && tower_atk.is_in_range(tower, t)
L1123      && tick < setting.tower_attack_disable_tick {
L1127     tower_score = min(tower_atk.expected_damage_target(ctx, tower as &dyn, t) * 100 / t.hp, 100)   (udiv · t.hp==0 → panic anon.163)
        } }
L1131 move_score = 0
      if effect.ty.expected_move_on_hit() || effect.ty.expected_rush_effect() {        // vtable +0x68 || +0x60 (이 순서)
L1134   if champ.distance_sq(t) < 1225000001 { move_score = 0 }   // ≤35000
L1137   else { ps = position_score_at_position(version, player, data, &parameter.positioning_score, t.x, t.y, PositionEvalPurpose::AttackStance)
L1140          move_score = -(hp_value * ps.risk) / 100 }   (sdiv)
      }
L1149 in_lapse = player_awareness_lapse(player, data)
      lapse_proximity_score = |e| (150000 - min(e.distance(champ),150000)) / 1500     // closure$6 (smin·sdiv)
// ---- §F 이동 중 사용 가능 · 사거리 내 (L1155~1172) ----
L1155 if action.can_use_with_move() && effect.is_in_range(champ, t) {                 // vtable +0xb8, 이 순서
L1156   if parameter.near_enemies.iter().any(|p| p.id == t.id) && in_lapse {
L1157     return 100 + lapse_proximity_score(t) }
L1159   if let Some(tp) = parameter.near_enemies.iter().find(|p| p.id == t.id) {
L1160     base_damage = tp.risk_damage
L1161     expected_damage = effect.expected_damage_target(ctx, champ as &dyn, t)
L1162     possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1163     hp_value = champion_hp_value(data, parameter, tp)
L1166     total_damage = expected_damage + base_damage/2 + possible_damage/4   (sdiv)
L1167     if t.stat_buff_cached.undying { L1168 total_damage = min(max(t.hp-1,0), total_damage) }
L1172     return total_damage * hp_value / t.hp + 100   (sdiv · t.hp==0 → panic anon.164)
        } }   // near_enemies 에 없으면 아래로 낙하
// ---- §G 적 챔피언 대상 (L1177~1300) ----
sum3 = tower_score + base_score + move_score
L1177 if in_lapse && near_enemies.any(|p| p.id==t.id) { total = lapse_proximity_score(t) (L1178) ; goto RET }
L1179 if let Some(tp) = near_enemies.find(|p| p.id==t.id) {
L1180   base_damage = tp.risk_damage ; L1181 expected_damage = effect.expected_damage_target(ctx,champ,t)
L1182   possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1183   hp_value = champion_hp_value(data,parameter,tp)
L1186   total_damage = expected_damage + base_damage/2 + possible_damage/4
L1187   if t.undying { L1188 total_damage = min(max(t.hp-1,0), total_damage) }
L1192   target_hp = t.hp ; score = total_damage * hp_value / target_hp   (sdiv · 0 → panic anon.165)
L1195   if t.is_champion() {
L1198     if expected_damage >= target_hp               { L1200 score += min(hp_value,100) }           // icmp slt 반전
L1201     else if expected_damage + base_damage/2 >= target_hp { L1203 score += min(hp_value,100)*3/4 }
L1206     else { auto_damage = champ.attack_effect.as_ref().map(|a| a.expected_damage_target(ctx,champ,t)).unwrap_or(0)   (L1206~1207)
L1210            if auto_damage + expected_damage >= target_hp { L1211 score += min(hp_value,100)/2 } } }
L1218   if let Some(cc_duration) = effective_action_cc_time(version, action, effect) {      // {tag,val}
L1219     if let Some(target_player) = cache.player_by_champion_id(t.id) {                   // L1220 null 검사
L1222       target_pos = target_player.info.position 태그 ; L1223 tps = max(setting.tick_per_second,1)
L1226       my_attack_dps = cache.player_champion_cache[team][my_position].attack_per_sec[target_pos]
L1231       ally_dps = 0; for ap in 0..5 { L1232 if ap == my_position {continue}
L1233         if let Some(ally) = cache.player_champion[team][ap] {
L1234           if ally.distance_sq(champ) < 22500000001 { c = cache.player_champion_cache[team][ap];
L1235~1241        ally_dps += c.attack_per_sec[target_pos] + c.skill_per_sec[target_pos] + c.skill2_per_sec[target_pos] + c.ult_per_sec[target_pos] } } }
L1248       follow_up_damage = (ally_dps + my_attack_dps) * cc_duration / tps   (udiv)
L1249       cc_score = follow_up_damage * hp_value / target_hp   (sdiv · overflow panic anon.166)
L1251       score += min(cc_score, 80)
L1258       nuke_during_cc = 0
            if champ.can_attack() || champ.attack_cooldown() <= cc_duration {      // attack_cooldown 인라인: ty 태그 switch → info.attack_cooldown
L1259~1260    if let Some(atk) = champ.attack_effect.as_ref() { nuke_during_cc = atk.expected_damage_target(ctx,champ,t) } }
L1264       if champ.can_skill() || champ.skill_cooldown() <= cc_duration {         // Champion 이 아니면 쿨다운 0 취급(IR: 태그≠13 → 통과)
L1265~1267    if let Some(s) = champ.skill_effect.as_ref() { if s.target.check(champ,t) { nuke_during_cc += s.expected_damage_target(ctx,champ,t) } } }
L1271       if champ.can_skill2() || champ.skill2_cooldown() <= cc_duration {
L1272~1274    s = if champ.level > 2 { champ.skill2_effect.as_ref() } else { None }; if let Some(s) { if s.target.check(champ,t) { nuke += s.expected_damage_target(..) } } }
L1278       if champ.can_ult() || champ.ult_cooldown() <= cc_duration {
L1279~1281    s = if champ.level > 4 { champ.ult_effect.as_ref() } else { None }; (동일) }
L1286       combined_with_cc_skill = nuke_during_cc + expected_damage
L1287       if combined >= target_hp { L1289 score += min(hp_value,80) }
L1290       else if target_hp > 0 && combined*100/target_hp > 59 { L1292 score += min(hp_value,80)/3 }
          } }
L1298   total = score + v55_mark_value(version,effect,data,player,parameter,champ,t,hp_value,_debug)
L1299         + v55_seal_value(version,effect,data,champ,t,hp_value)
L1300         - v55_banish_penalty(version,effect,data,player,champ,t,hp_value)
        goto RET }
// ---- §H 아군 챔피언 대상 (L1302~1423) ----
L1302 else if let Some(tp) = parameter.near_allies.iter().find(|p| p.id==t.id) {
L1303   expected_heal = effect.expected_heal_target(ctx, champ as &dyn, t as &dyn)
L1304   expected_shield = effect.expected_shield_target(ctx, champ, t)
L1306   v55_tbuff: Option<BuffState> = v55_target_dependent_buff(effect,data,t)
L1307   has_buff = v55_tbuff.is_some() || effect_buff_target(version,effect,ctx,champ,t).is_some()   // 단락: v55 Some 이면 두 번째 미호출
L1309   v55_on_attack = effect.ty.expected_on_attack_damage(ctx, champ)          // vtable +0xb0
L1310   etc_buff = if has_buff || v55_on_attack > 0 || !effect.ty.etc_buff() { 0 }    // IR 평가 순서: on_attack>0 · !etc_buff() · has_buff 를 or 로 합침(단락 없음)
L1312             else if effect.ty.expected_mark(ctx, champ as &dyn).is_none() { 5 } else { 0 }   // vtable +0xa8 sret 48B, 태그 0=None
L1320   hp_value = champion_hp_value(data,parameter,tp)
L1322   possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1323   applyed_damage = tp.applyed_damage ; L1325 epic_tank = tp.risk_epic_damage
L1328   expected_shield = if possible_damage>0 || epic_tank!=0 || applyed_damage>0 { expected_shield } else { 0 }
L1338   incoming = applyed_damage + possible_damage + epic_tank ; epic_incoming = epic_tank
L1339   missing = max(t.stat_cached.hp - t.hp, 0)
L1340   expected_heal = min(missing + incoming*2, expected_heal) ; expected_shield = min(incoming*3, expected_shield)
L1347   if has_buff || v55_on_attack > 0 {
L1348     bs = v55_tbuff.clone().or_else(|| effect_buff_target(version,effect,ctx,champ,t))   // closure$12
          if let Some(bs) = bs {
L1350       crisis_needed = bs.cc_immune || bs.undying || bs.toughness == 0
L1351~1354  crisis: Option<(bool,bool)> = if crisis_needed { Some(defensive_crisis(version,_rnd,player,data,t,_debug)) } else { None }
L1360       v = buff_value_v54(&bs, t, data, player, parameter, crisis.as_ref(), incoming, epic_incoming, v55_on_attack, hp_value)
L1363       cast_range = effect.range + (champ.level-1)*growth + champ.stat_buff_cached.range + effect.range_adjust(champ,t) + champ.radius() + t.radius()
L1364       walk_ticks = champ.distance(t).saturating_sub(cast_range) / max(champ.stat_cached.move_speed,1)   (udiv)
L1365       delay_sec = walk_ticks / tps   (sdiv · tps==0 → panic anon.167)
L1366       v = clamp(6 - delay_sec, 0, 6) * v / 6
L1368       v_currency = area_buff_multi_scale(version,effect,data,player,t,v)
          } else if v55_on_attack > 0 {     // L1369
L1373       v = buff_value_v54(&BuffState::default(), t, data, player, parameter, None, incoming, epic_incoming, v55_on_attack, hp_value)
L1375       v_currency = area_buff_multi_scale(version,effect,data,player,t,v)
          } else { v_currency = 0 }
L1378     if has_buff && v_currency == 0 {
L1380       bs2 = v55_tbuff.clone().or_else(|| effect_buff_target(..)) ; w = noncombat_steroid_window(player,data,champ,t)   // Option<(bool,bool)>
            if bs2.is_none() || w.is_none() { L1392 expected_buff_score = 0 }
            else { L1383 v = noncombat_steroid_value(data, &bs2, t, &w) ; L1384 v = area_buff_multi_scale(version,effect,data,player,t,v)
L1386              if v > 0 && ctx.debug && tick % 6 == 0 { L1387 add_log("STEROID_WIN T{team} {pos:?} ally_target v={v} fights_back={w.fights_back}(=.0)") }   // anon.153 · w = NoncombatSteroidWindow{fights_back@0, minion_wave@1}(tcxdict 2B)
L1390              expected_buff_score = v }
          } else { L1395 expected_buff_score = v_currency }
        } else { L1398 expected_buff_score = 0 }
L1401   score = (expected_heal + expected_shield) * hp_value / t.hp   (sdiv · 0 → panic anon.168)
L1403   if expected_heal + expected_shield > 0 && score == 0 {
L1404     score = if applyed_damage > 0 || possible_damage > 0 || tp.risk_damage != 0 || t.hp < t.stat_cached.hp { 1 } else { 0 } }
L1411   aoe_ally_value = v54_aoe_ally_heal_value(version,effect,data,parameter,champ,t,t.id,action)
L1414   if has_buff && ctx.debug { L1415~1416 add_log("BUFFSCORE nm={action.action_name()} T{team} {pos:?} ally t={t.id} buff={expected_buff_score} hs={score} etc={etc_buff} aoe={aoe_ally_value}") }   // anon.154
L1419   total = expected_buff_score + score + etc_buff + aoe_ally_value
L1423   if total == 0 { total = if has_buff || expected_heal+expected_shield > 0 { -10 } else { 0 } }
        goto RET }
// ---- §I 자기 자신 대상 (L1429~1550) ----
L1429 else if t.id == champ.id {           // tp = parameter.player
L1430~1431 expected_heal/shield = effect.expected_heal_target/expected_shield_target(ctx,champ,t)
L1434~1435 v55_tbuff = v55_target_dependent_buff(effect,data,t) ; has_buff = v55_tbuff.is_some() || effect_buff_target(version,effect,ctx,champ,t).is_some()
L1437   v55_on_attack = effect.ty.expected_on_attack_damage(ctx,champ)
L1438~1440 etc_buff = (§H L1310~1312 와 동일)
L1450   epic_tank = parameter.player.risk_epic_damage
L1453   expected_shield = if possible_damage(§B 값) < 1 && (epic_tank | parameter.player.applyed_damage) == 0 { 0 } else { expected_shield }
L1462   incoming = possible_damage + applyed_damage + epic_tank ; epic_incoming = epic_tank
L1463   missing = max(champ.stat_cached.hp - champ.hp, 0)
L1464   expected_heal = min(missing + incoming*2, expected_heal) ; expected_shield = min(incoming*3, expected_shield)
L1469~1511 expected_buff_score = §H L1347~1395 와 같은 구조(대상 t=champ · hp_value=§D 의 hp_value) — 단 L1363~1366 의 사거리/도달 감쇠가 없다: v_currency = area_buff_multi_scale(version,effect,data,player,champ, buff_value_v54(&bs,champ,data,player,parameter,crisis.as_ref(),incoming,epic_incoming,v55_on_attack,hp_value)) (L1482~1484) ; bs None && on_attack>0 → default BuffState (L1489~1491) ; L1494 has_buff && v_currency==0 → L1496 bs2·w=noncombat_steroid_window(player,data,champ,champ) → L1499 noncombat_steroid_value → L1500 area_buff_multi_scale → L1502~1503 로그 "STEROID_WIN ... self v= fights_back="(anon.155) → L1506 expected_buff_score=v / L1508 0 ; L1511 else v_currency
L1514   (has_buff 아니고 on_attack<=0) expected_buff_score = 0
L1517   score = (expected_heal + expected_shield) * hp_value / champ.hp   (sdiv · 0 → panic anon.169)
L1523   ally_aura_protect = 0; for ally in cache.player_champion[team].iter().flatten() {      // 자기 자신 포함 5슬롯
L1524     if effect.ty.expected_ally_aura_heal_at(ctx, champ, ally) != 0 {                       // vtable +0x98
L1527       if let Some(atp) = near_allies.find(|p| p.id == ally.id) {
L1528         if defensive_crisis(version,_rnd,player,data,ally,_debug).0 {
L1529           ally_aura_protect += min(champion_hp_value(data,parameter,atp), 80) } } } }
L1537   v55_trigger = v55_spirit_trigger_value(effect,data,player,parameter,champ,hp_value)
L1540   aoe_ally_value = v54_aoe_ally_heal_value(version,effect,data,parameter,champ,champ,champ.id,action)
L1543   if has_buff && ctx.debug { L1544~1545 add_log("BUFFSCORE nm=.. T.. self buff= hs= etc= aura= trig= aoe=") }   // anon.156
L1548   total = score + expected_buff_score + etc_buff + ally_aura_protect + v55_trigger + aoe_ally_value
L1550   if total == 0 { total = if has_buff || expected_heal+expected_shield > 0 { -10 } else { 0 } }
        goto RET }
// ---- §J 그 밖(타워·미니언·근접목록 밖 챔피언) ----
L1558 else { if t.is_champion() && ctx.debug && t.team == champ.team { L1559 add_log("BUFFTGT_MISS T{team} {pos:?} t={t.id} near_allies") }   // anon.157
        total = 0 }
// ---- RET (L1564) ----
return sum3 + total          // sum3 = tower_score + base_score + move_score
// 반환 phi 전체: [v57 Some(v)] [§F 100+x] [sum3+total] [-9999999 L1020] [-9999999 L1034]
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d59940` → `f749a0` calculate_action_score (i=186 · 변경·심층(r21 §D: w10 §1))
- 0.5.8 src: `game-ai\src\action_score.rs:8` · one_line: 액션(평타/스킬)을 대상 t 에 쓸 때의 점수 — 미니언은 막타 타이밍·라인 스타일, 챔피언/타워/넥서스는 계수×기대피해/HP + 보너스
- 0.6.0 판정: **심층(r21)** · 패치 요지: 인자 11→10(`_debug` 삭제) · **v3 적 챔피언 조기 반환**(Champion arm · pos≠Jungle · near_enemies(+0x14d8/+0x14f0 · 0xd8B) 에 t 있으면 `hv_t*(value + risk_damage/2 + possible_risk(dur+30)/4 + risk_possible_tower)/max(t.hp,1) + bonus − hv_me*(possible_risk(me)+risk+tower)/max(champ.hp,1)` 반환 · 없으면 공통 경로) · **특성 flag(v2/v3 공통)**: `Aggr(+0x49d) → (flag,def)=(true,false)` · else `(Def ‖ pos≠Jungle, Def)` → L147 `flag ? (def ? urg+multi+5 : urg+multi) : urg+3+multi` · L168/L378/L345 `flag&&!def → (gen<err?10:-9999)` · L359 `def → (gen<err?-9999:25)` · 미니언 분기 전부 그대로 · 콜리 ff1fc0 possible_risk · db0400 hp_value 캐시(본체 de5780) · d72be0 · Action vt +0x80 duration
- RE 정본: `2026-09-17_r21_심층_w10_calculate_action_score_EpicCheck_calc_interaction_Trace_get_input_원문.md` · 절: w10 §1
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, usize, &game_core::Entity, game_ai::MinionAction`
- consts: [{"value": 2, "src_line": 10, "meaning": "team bounds(<2) · EntityType::Tower 태그(L30/L420) · MinionActionType::Push(L161/178/335/368/393/462) · Tower.ty-3 <u 2(twin)", "kind": "태그", "ev": 4}, {"value": 1000, "src_line": 12, "meaning": "base = 1000 - last_hit_accuracy · 롤 정규화 /1000 · error_prob 상한 umin 1000 · gen_range 0..=1000 · L321 1000 - gen_range(acc..=1000) · 오라클 실행 확증(24차 배치F: 오라클 실행 확증(24차 F o186.rs · run186.py 335케이스 · 케이스당 프로세스 1개 · MATCH 334/334 + 의도된 패닉 1 · StdRng 복제본 소비량 동기 334/334) — 경계 케이스(value+5 · 999 · 29999 · 3v/2v · total*3/10 · 16199/16200 틱 · e.hp≥d · in_range>1 · min(coef
- 0.5.8 logic 전문:
```
// L10  team=player.team(<2); champ = cache.player_champion[team][position].unwrap()
// L11  acc = parameter.last_hit_accuracy(); base = 1000-acc; min_v=acc; max_v=2000-acc
// L18  jrng = range_misjudge_rng(version, data, player, t.id)
// L19  value = effect.expected_damage_target(ctx, champ, t) * roll/1000        // roll = range_misjudge_roll_i64(rnd,&jrng,acc,2000-acc), 매번 새 롤
// L20  hp = t.hp*roll/1000;  L21 total = t.stat_cached.hp*roll/1000
// L22  start_timing = (effect.start_timing*100/speed_mult)*roll/1000   (speed_mult==0 → 패닉)
// L23  cooltime = (action.cooltime(champ)*100/speed_mult)*roll/1000    // %88 = roll*raw (÷1000 전) 이 뒤의 29999 비교에 쓰임
// L26  match t.ty {
//  Minion(1) =>
//   L29 if let Some(target)=t.Minion.nearest_enemy.and_then(get_entity_by_id) { match target.ty {
//     Tower(2) => { L32 if Moba && epic_minion_buff_time[1-team]!=0 → return 30;  L37 if target.Tower.ty ∈{TwinA,TwinB} { L38 if Moba && buff[1-team]!=0 → return 70 else → return 50 } }
//     Nexus(3) => { L46 if Moba && buff[1-team]!=0 → return 100 else → return 70 }
//     _ => {} } }
//   L55 if Moba && epic_minion_buff_time[1-team]!=0 → return 20        // 적 에픽버프 미니언
//   L62 if let Some(snap)=parameter.wave_snapshot { L63 if let Some(traj)=snap.minions[..count].find(id==t.id) {   // count>12 → 패닉
//     L68 error_prob = min(base*base/1000, 1000)
//     L72 if gen_range(0..=1000) < error_prob → return -9999               // 오판
//     L76 lane_phase = !(tutorial∈{None,MidBottom,Line,Total} && tick < first_spawn_tick - 30*tps ? false : true)  — 정확히: early=(tutorial∈집합 && tick<spawn-30tps); flag = !early && position!=Jungle   ⚠주의: tutorial∈집합 && tick≥경계 → flag=false 가 아니라 … IR: tutorial∈집합 이면 tick<경계 일 때만 position!=Jungle, 아니면 false; tutorial∉집합 이면 position!=Jungle
//     L89 predicted_hp = hp_at_tick(traj, start_timing)*roll/1000 ; L90 death_tick = traj.expected_death_tick
//     L93 if roll*hp_at_tick > 999 (생존) {
//        can_last_hit(DI명) = predicted_hp > value+5   // = 이번 타격으로 못 죽임
//        will_die_soon = death_tick ≤ cooltime+start_timing
//        L97 if !can_last_hit (지금 죽일 수 있음) {
//          L100 urgency = death_tick > start+5 ? (will_die_soon ? 25 : 15) : 30
//          L113 concurrent = #{other in snap: id≠t.id && 0 < hp_at_tick(other, cooltime+start) ≤ value+5}
//          L126 multi_bonus = concurrent>0 ? (L129 earlier=#{other: id≠t.id && other.death_tick < death_tick}; earlier==0 ? 5 : -5) : 0
//          L147 return flag ? urgency+multi_bonus : urgency+3+multi_bonus
//        } else { L152 if will_die_soon && predicted_hp ≤ 2*value → return ty==Pull ? -9999 : 5 }
//     }
//     L159 if predicted_hp > 3*value { L161 match ty { Pull→-9999, Push→10, Normal→ L168 if roll*cooltime_raw>29999 → (gen<err?10:-9999) else if flag → (gen<err?10:-9999) else → (gen<err?-9999:10) } }
//     else { L178 match ty { Pull→-9999, Push→10, Normal→ L184 gen<err ? 10 : -9999 } }
//   } }
//   // 스냅샷 없음/미등재 → 레거시(L195~)
//   L199 flag(%420) 위와 동일 계산
//   L211 for e in iter_entity(): match e.ty { Minion|Tower|Ghoul|SmallJiangshi|Bear|Eagle }: if e 가 t 를 조준(nearest_enemy/target_enemy == t.id) { dmg = e.attack_effect.expected_damage_target(ctx,e,t) (None→패닉); acd=attack_cooltime(e); ast = attack_effect.start_timing*100/attack_speed_mult(e); if e.state==Attack && !(Minion && is_range) && time<ast && ast(+15 if Tower)-time < start_timing && Attack.target_id==t.id → applyed += dmg; expected += max((cooltime+start_timing)/acd,1)*dmg (acd==0→패닉) }
//   L297 for p in iter_projectile(): if p.move_type==Target(6) { if target_id==t.id { caster=get_entity_by_id(p.caster_id)?; dist=distance(p,t); dmg=p.expected_damage_target(ctx,caster,t); if start_timing ≥ dist/speed+5 → applyed+=dmg; expected+=dmg } } else { caster?; if p.applyed_target.check_projectile(p,t) && p.is_in_orbit(t.x,t.y, radius*(100+radius_mult)/100) { dmg=…; applyed+=dmg; expected+=dmg } }
//   L319 applyed*=roll/1000; L320 expected*=roll/1000; L321 error_prob = 1000 - gen_range(acc..=1000)
//   L325 if value-5+applyed < hp (못 죽임) {
//      L365 if value + total*3/10 + expected < hp { L368 Pull→-9999 · Push→10 · Normal→ L378 cooltime>29999 ? (gen<err?10:-9999) : flag ? (gen<err?10:-9999) : (gen<err?-9999:10) }
//      else { L393 Pull→-9999 · Push→10 · Normal→ L402 gen<err?10:-9999 }
//   } else { L329 if gen_range(0..=1000) < error_prob → return -9999
//      L333 if expected + total*3/10 < hp { L335 Pull→-9999 · Push→10 · Normal→ L345 cooltime>29999 ? (gen<err?10:-9999) : flag ? (gen<err?10:-9999) : (gen<err?-9999:10) }
//      else { L359 gen; flag ? (gen<err ? -9999 : 15) : (gen<err ? -9999 : 20) } }
//  Champion(13) =>
//   L414 bonus=0; if t.team != champ.team { L416 if let Some(ep)=player_by_champion_id(t.id) { L417 a = blackboard[1-team].small_actions[ep.position]; if a.tag∈{Attack,Skill,Skill2} { L419 if let Some(tt)=get_entity_by_id(a.target_id) { L420 match tt.ty { Tower → bonus = twin ? 80 : 10 (L424); Nexus → bonus = 100 (L428) } } } } }
//   → L438 공통
//  _ => bonus = 0 → L438
// }
// L438 value = effect.expected_damage_target(ctx, champ, t) (롤 없음)
// L452 has_epic_buff(DI명) = !(Moba && ptr) || epic_minion_buff_time[team]==0     ⚠이름과 반대 극성처럼 보임 — IR 그대로: '아군 에픽버프 없음' 이 true
// L454 coef = match t.ty {
//   Nexus(3) → 200
//   Tower(2) → L457 in_range_minion = #{e: e.team==champ.team && e.ty==Minion && t.attack_effect.is_in_range(t, e)}   // ★t(타워) 자신의 attack_effect(사거리) — champ 것이 아니다. t.attack_effect None 이면 같은 팀 미니언을 만나는 순간 unwrap 패닉(L459)
//              L460 cond = match t.Tower.nearest_enemy { Some(id) if id!=champ.id → match get_entity_by_id(id) { Some(e) → e.hp ≥ t.attack_effect.expected_damage_target(ctx, t, e) || in_range_minion>1, None → in_range_minion>1 }, _ → false }   // ★타워가 조준 대상 e 에 주는 기대피해(t.attack_effect · None 이면 unwrap 패닉 L461) — 「타워 한 방에 안 죽는 대상을 조준 중이거나 사거리 안 아군 미니언 2+」
//              L462 if cond { ty==Push ? (has_epic_buff ? 160 : 240) : (has_epic_buff ? 80 : 160) } else { L476 t.hp > effect.expected_damage_target(ctx,champ,t) ? 0 : 30 }
//   _ → 0 }
// L516 return min(coef, coef*value / t.hp) + bonus     // t.hp==0 → 패닉
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `cb03b0` → `e96780` EpicCheckSubPlan::action_candidates (i=191 · 변경·심층(r21 §D: w10 §2))
- 0.5.8 src: `game-ai\src\plan_legacy\sub_plan\epic_check.rs:14` · one_line: 에픽(모르가드) 확인 서브플랜 후보: 위험이면 도주 단독 / v25 오브젝트 자세(Screen·WaitGroup·SoftDisengage)가 있으면 그 자세 전용 후보 / 없으면 (자기 진영일 때 Stump 캠프 경유 확인 후) 모르가드 캠프 AroundPosition + 적 근접 시 도주 + 적에게 보이면 교전 + 소환수 공격
- 0.6.0 판정: **심층(r21)** · 패치 요지: 인자 9→8(debug 삭제 · rs:52~53 로그 제거) · **rs:65~75 재작성 = SerpenCheck 동일 패턴**: `!move_check → enemy_side ? set : Stump 4.9e9 이내 ? (!tp.cc0 ? set : (Morgard 220000² 내 hp≥50 아군 ≥ REQ[tutorial]([3,2,1,2,1,3,1,3,3]) ‖ hp≥40 앵커(dist+BIAS[60000,0,40000,60000,20000] 최소·동점 앞)==나 ? set : mc_now=false)) : mc_now=false` · Trace 태그 0xe→0xd · position_score 셀 중심은 0.5.8 부터(동치) · rs:77~103 그대로
- RE 정본: `2026-09-17_r21_심층_w10_calculate_action_score_EpicCheck_calc_interaction_Trace_get_input_원문.md` · 절: w10 §2
- sig: `fn(&mut game_ai::plan_legacy::sub_plan::EpicCheckSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallAc`
- consts: [{"value": 2, "src_line": 17, "meaning": "player.team 바운즈 상한 임계(팀 2) — IR L9654 `icmp ult i64 %43, 2`(panic_bounds_check). 190 consts[0] 과 같은 상수·같은 종류. ⚠같은 리터럴 2 의 다른 뜻(ObjectivePostureKind 2=Screen rs:47/58 · JungleType 2=Stump rs:69/83 · CastingType 2=Direction · level>2 rs:27)은 별개 리터럴", "kind": "임계", "ev": 4}, {"value": 13, "src_line": 24, "meaning": "EntityType 태그 13 Champion (적 챔프 필터)", "kind": "태그", "ev": 4}, {"value": 4, "src_line": 25, "meaning": "ChampionActionState 태그 4 = Skill (switch case `i64 4, label %85` · IR L9789 — 적 챔프가 스킬 시전 중이면 skill_effect 사거리 검사). ⚠같은 리터럴 4 의 다른 뜻(JungleT
- 0.5.8 logic 전문:
```
fn action_candidates(&mut self, version, rnd, player, data, parameter, team_plan, debug) -> Vec<SmallActionPlay> {
  let bump = data.context.pool;  let mut res = Vec::new_in(bump);                        // rs:15
  let team = player.info.team; assert!(team<2);                                             // rs:17
  let champ = data.cache.player_champion[team][player.info.position].unwrap();              // rs:17
  // rs:23~31  (attack_nexus rs:125~131 과 문자 단위 동일 구조 · closure#0 rs:23)
  let has_non_target_action_range = data.cache.player_champion[1-team].iter().flatten().any(|c|
      nontarget_windup_perceived(version, player, data, c) && c.ty==Champion(13) && {
          let eff = match c.action_state.tag { 4=>skill_effect(casting 1|2 아니면 false, -1 panic), 5=>(level>2 ? skill2_effect : None)…, 6=>(level>4 ? ult_effect : None)…, _=>return false };
          Effect::is_in_range(eff, c, champ) });
  // rs:36~38
  let ps = position_score_at_position(version, player, data, &parameter.positioning_score, champ.x, champ.y, PositionEvalPurpose::Objective /*11*/);
  if ps.on_trajectory || (has_non_target_action_range || ps.on_periodic_trajectory) {         // rs:38 (뒤 둘 순서 표기불가)
      res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)));           // rs:40  tag 3
      return res;                                                                            // rs:41
  }
  // rs:45~55  v25 오브젝트 자세
  let posture = team_plan.v25_objective_posture(version, player, data, JungleType::Morgard /*4*/);   // rs:45  sret 88B
  if let Some(posture) = posture {                                                           // rs:46  (+0 != -1)
      if posture.kind as u8 > 2 {                                                            // rs:47  WaitGroup(3) | SoftDisengage(4)
          if posture.kind == SoftDisengage(4) && posture.near_enemy_count != 0 {              // rs:48
              res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)));  // rs:49  tag 3 · with_skill=false
          }
          res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, posture.wait_pos.0, posture.wait_pos.1, 5)));   // rs:51
          if data.context.debug {                                                            // rs:52 (+0x3b)
              debug.infos.entry(champ.id).or_default().push(format!("v25 morgard check posture: {:?}", posture.kind));   // rs:53
          }
          return res;                                                                        // rs:55
      } else if posture.kind == Screen(2) && posture.focus_enemy.is_some() {                 // rs:58
          let focus_enemy = posture.focus_enemy.unwrap();                                    // rs:59
          res.push(Trace(SmallActionTrace::new_attack_range(data, focus_enemy, 5)));         // rs:60  tag 14 (margin 15000 · attack_range_only=true · goal=get_entity_by_id(focus_enemy).xy or 0)
          // ⚠return 없음 — 아래 rs:65 로 계속(IR %196→%153→%133)
      }
      // Commit(0)/HoldCamp(1) 또는 Screen+focus None → 아래로
  }
  // rs:65~75  사이드 경유 검사 (self.move_check 갱신)
  let move_check_now: bool = if self.move_check { true }                                     // rs:65
      else if !is_enemy_side(data.context, team, champ.x, champ.y) {                         // rs:66 ★자기 진영이면 Stump 경유 검사. IR %290 = xor(team==0, (x−y+height) >u width)(L10349~10359) 참 → %298 Stump 검사 · 거짓 → %291 move_check=true. (x−y+height)>u width 는 !is_blue_side(레드 진영, 오라클 pub is_blue_side 대조) · is_enemy_side = is_blue_side != (team==0). 소스가 `if is_enemy_side {..true} else {Stump}` 인지 `if !is_enemy_side {Stump} else {..}` 인지는 표기 불가(분기 의미는 확정)
          let camp = data.context.map.camp_pos(JungleType::Stump /*2*/, team==0);            // rs:69
          if champ.distance_sq(camp) < 4900000001 /*70000²+1*/ { self.move_check = true; true }   // rs:71  (store L10363)
          else { false }
      } else { self.move_check = true; true };                                               // rs:66 else = 적 진영 (IR %291 store i8 1)
  // rs:77~87  모르가드 캠프 접근
  let camp = data.context.map.camp_pos(JungleType::Morgard /*4*/, team==0);                  // rs:77
  if champ.distance_sq(camp) > 22500000000 /*150000²*/ {                                     // rs:78  멀다
      if move_check_now {                                                                    // rs:79
          res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, camp.0, camp.1, 5)));           // rs:80  모르가드 캠프로
      } else {
          let c2 = data.context.map.camp_pos(JungleType::Stump /*2*/, team==0);              // rs:83
          res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, c2.0, c2.1, 5)));               // rs:84  Stump 경유지로
      }
  } else {
      res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, camp.0, camp.1, 5)));               // rs:87  캠프 근처 대기
  }
  // rs:90~95  적 근접 → 도주 후보 추가
  let enemy_near = data.cache.player_champion[1-team].iter().flatten().any(|c|                // rs:90  closure#1
          Blackboard::is_recent_visible(&data.blackboard[1-team], data.cache.game, player, c)  // ⚠blackboard 인덱스 = 1-team (IR %61)
          && c.distance_sq(champ) < 22500000001 /*150000²+1*/)                                // rs:91
      || data.cache.others[1-team].iter().any(|x| x.distance_sq(champ) < 22500000001);      // rs:92  closure#2 (any 첫 절이 참이면 둘째 미평가: IR %366→%458 직행)
  if enemy_near {
      res.push(RunAway(SmallActionRunAway::new(data, player, 5)));                           // rs:95  tag 3
  }
  // rs:98~101
  if data.cache.game.is_visible(1-team, champ.id) {                                          // rs:98  vtable+0xf8 — 적에게 보이면
      res.extend(battle_action(version, rnd, player, data, 5));                              // rs:99
  }
  res.extend(attack_summon_action(player, data));                                            // rs:101
  res                                                                                        // rs:103
}
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e6b800` → `d72e70` check_kill (i=87 · 변경·심층(r21 §D: w1 §2))
- 0.5.8 src: `game-ai\src\plan_legacy\handler.rs:2507` · one_line: 150000 안 적 중 「내가 (아군 화력으로) 먼저 죽이고, 그 적이 타워로 도망치기 전에 잡을 수 있는」 첫 적을 골라 (적 id, 예상 처치 틱)을 돌려준다
- 0.6.0 판정: **심층(r21)** · 패치 요지: v3 확장 후보 e0f970 · escape_possible eeb5b0 · (c1,c2,c3) 특성 · v3 *tps · v3 즉시수락 · lapse +0x49c/8번째 인자
- RE 정본: `2026-09-17_r21_심층_w1_check_kill_die_tick_uncached_check_kill_원문.md` · 절: w1 §2
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<(usize, usize)>`
- consts: [{"value": 22500000001, "src_line": 2461, "meaning": "150000²+1 — 타워↔적 거리(제곱) 임계(L2461) · (aux) 아군↔나(L2377)·적↔나(L2379) 후보 반경 · L2481 최근접 타워 탐색 첫 원소. 셀 32000 기준 ≈4.7셀", "kind": "임계", "ev": 4}, {"value": 100, "src_line": 2377, "meaning": "(aux closure#0) 아군 HP% = hp*100/max_hp", "kind": "미상", "ev": 4}, {"value": 39, "src_line": 2377, "meaning": "(aux closure#0) 아군 후보 = HP% > 39 (즉 40% 이상)", "kind": "미상", "ev": 4}, {"value": 1000, "src_line": 2419, "meaning": "aggressive_ratio(0..1000) 정규화 분모", "kind": "계수", "ev": 4}, {"value": 80, "src_line": 2419, "meaning": "c1 = (1000-ar)*80/1000 + 80 ∈ [80,1
- 0.5.8 logic 전문:
```
check_kill(version, rnd, player, data, _ps, debug):                 // L2507
  if player_awareness_lapse(player, data) { return None }            // L2509~2510
  return check_kill_v15(version, player, data, debug)                // L2512 (2373~2505 전부 인라인)

--- check_kill_v15 ---
me   = cache.player_champion[team][my_pos].unwrap()                            // L2374
allies  = iter_champions(team).filter(|a| dist_sq(a,me) <= 150000² && a.hp*100/a.max_hp > 39)   // L2377 (aux#0) ★나 자신 포함
enemies = iter_champions(1-team).filter(|e| dist_sq(e,me) <= 150000²
            && (me.team==Neutral || e.visible_state[my_team]==Visible)
            && !is_ignored_well_enemy(version, player, e))                       // L2379 (aux#1)
disable_tick = match player_count(tutorial) { 5v5→setting.tower_attack_disable_tick, 2v2→_2v2, 3v3→_3v3 }   // L2383~2386
towers = iter_towers_without_nexus(cache, 1-team).filter(|t| game.tick() <= disable_tick)   // L2389 (aux#2) — 적 타워

// 1) 적 전원이 나에게 주는 화력
enemy_burst = 0; enemy_dps = 0
for e in enemies:                                                              // L2393
  pe = player_by_champion_id(e.id).unwrap(); cc = champion_cache[pe.team][pe.pos]   // L2395
  b = 0
  if e.can_attack() || e.attack_cooldown() is None || cd <= tps { b = cc.attack[my_pos] }         // L2398~2399
  if e.can_skill()  || e.ty!=Champion || e.skill_cooldown  <= tps { b = max(b, cc.skill[my_pos]) } // L2402~2403
  if e.can_skill2() || e.ty!=Champion || e.skill2_cooldown <= tps { b = max(b, cc.skill2[my_pos]) }// L2406~2407
  enemy_dps  += cc.attack_per_sec[my_pos] + cc.skill_per_sec[my_pos] + cc.skill2_per_sec[my_pos]  // L2410~2412
  enemy_burst += b                                                             // L2413

ar = player.parameter.aggressive_ratio()                                       // L2417
c1 = (1000-ar)*80/1000 + 80 ; c2 = ar*45/1000 + 45 ; c3 = ar*35/1000 + 15     // L2419~2421

// 2) 적 하나씩 — 아군 전원의 화력 vs 그 적
for e in enemies:                                                              // L2424
  pe = player_by_champion_id(e.id).unwrap(); epos = pe.pos                     // L2431~2432
  ally_burst = 0; ally_dps = 0
  for a in allies:                                                             // L2434
    pa = (team,pos) of a via player_champion 10칸 id 스캔 (없으면 unwrap 패닉)   // L2435
    cc = champion_cache[pa.team][pa.pos]                                       // L2440
    ally_dps += cc.attack_per_sec[epos] + cc.skill_per_sec[epos] + cc.skill2_per_sec[epos]   // L2440~2442
    b = 0 ; (공격/스킬/스킬2 가용 판정은 1)과 동일 규칙, 대상 열 = epos)            // L2444~2453
    ally_burst += b                                                            // L2456~2457
  my_burst = enemy_burst ; my_dps = enemy_dps
  for t in towers:                                                             // L2460
    if dist_sq(t, e) <= 150000² {                                              // L2461
      d = t.attack_effect.unwrap().expected_damage_target(ctx, t, <static>, me)   // L2462
      my_dps  += tps * d / t.attack_cooltime()   (0 이면 div-by-zero 패닉)        // L2463
      my_burst += d }                                                          // L2464
  enemy_die = sat(e.hp - ally_burst) * 60 / max(ally_dps, 1)                   // L2470
  my_die    = sat(me.hp - my_burst)  * 60 / max(my_dps, 1)                     // L2471
  if ctx.debug { debug.infos[me.id].push(format!(enemy_die, my_die)) }         // L2473~2474
  if enemy_die > sat(my_die - c1) { continue }                                 // L2477  ★내가 c1 틱 이상 먼저 죽이지 못하면 탈락
  tower_ticks = match towers.min_by_key(|t| dist_sq(t,e)) { Some(t) => e.distance(t) / e.move_speed, None => 99999999 }   // L2481~2484
  if me.move_speed > e.move_speed {                                            // L2489
    ok = enemy_die < sat(tower_ticks - c3)                                     // L2498~2499
  } else {
    ok = me.attack_effect.unwrap().is_in_range(me, e) && enemy_die < c2 && enemy_die < sat(tower_ticks - c3)   // L2491~2494
  }
  if ok { return Some((e.id, max(enemy_die, 60) + game.tick())) }              // L2502
return None                                                                    // L2505
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
