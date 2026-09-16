# r21 티어1 심층 — 웨이브 3 (3) · 0.5.8 명세 logic 전문 + exe 힌트

요구: 0.5.8 명세(아래 logic) 를 기준선으로 0.6.0 디컴을 **줄 단위로 대조**해 「어느 줄이 어떻게 바뀌었나 / 새 분기·조건·상수·필드·콜리」 를 명세 형식(줄번호 참조 · 등가 Rust)으로 낸다. version≥3 분기는 「v3 경로」 로 별도 표기(값 미확정 · v2/v3 둘 다 적는다). 대형 함수는 먼저 콜리 지도·블록 지도(0.5.8 ↔ 0.6.0 매핑)를 만들고 신규 구역을 특정한 뒤 신규 구역만 정독한다.

## `cc4260` → `eafb50` JungleSubPlan::action_candidates  (5349→10734B · Δ+5385)
- 힌트: JungleSubPlan::action_candidates(5.3→10.7KB · eafb50 · JungleSubPlan 0.6.0 확장 +0x48 team/+0x58 check_move/+0x59 camp)
- exe 정렬: 명령 1139→2140 · 정렬 979 · 잔여 구조 10 · 분기 10 · 콜리 주의: 128cc90→164b370 미지 ; 31a01a3→16a7af0 미지 ; 31a01a3→381e0b3 미지 ; 31a37c0→ecde50 미지(+570B) ; cada80→e920f0 미지(+2B) ; ffa3e0→e8e7c0 미지(-359B)
- 0.5.8 명세 #193 `jungle__Jungle__action_candidates` src game-ai\src\plan_legacy\sub_plan\jungle.rs:107 · one_line: 정글 캠프(self.camp/team) 사냥 서브플랜 후보 — 위험(논타겟 윈드업·궤적)이면 도주 단일, 아니면 전투·소환물 후보 → 캠프 150000 이내/적에게 보임/적 정글몹 150000 이내면 도주 추가 → 캠프 몹 공격/스킬/스킬2 후보(attack_jungle_action 인라인) → 캠프 이동 1개(move_action 인라인, 적 캠프·아군 진영·경유지 멀면 out_line)
- 0.5.8 params: (sret):bumpalo::Vec<SmallActionPlay(writeonly sret. +0 ptr · +8 bump(=data.context.pool) · +0x10) · self:&mut JungleSubPlan (16B)(noalias captures(none), readonly 없음 = &mut. team:usize@0 · c) · version:usize(본문 분기 없음. nontarget_windup_perceived · position_score_at_pos) · rnd:&mut StdRng (320B)(본문 직접 읽기 없음. battle_action(_rnd) · AroundPosition::new / new) · player:&PlayerState (2528B)(readonly) · data:&OperationData (24B)(cache@0 · context@8. blackboard(+0x10)는 이 함수에서 읽지 않음) · parameter:&ScoreParameter (5384B)(+0x9f0 positioning_score 주소만 position_score_at_position 에 전달)
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

## `d2f180` → `fa47a0` PassiveJunglePlan::next_plan  (10479→14676B · Δ+4197)
- 힌트: PassiveJunglePlan::next_plan(10.5→14.7KB · fa47a0)
- exe 정렬: 명령 2225→2966 · 정렬 1272 · 잔여 구조 10 · 분기 10 · 콜리 주의: 1323a00→fa3350 미지(+1454B) ; 31a01a3→381e0b3 미지 ; 31a01a3→d35450 미지 ; 31a04c0→3821813 미지(-43B) ; 31a04c0→dd8e70 미지(+14959B) ; 31a37c0→38219f0 미지(+15B) ; 31a3b40→3821813 미지(+32B) ; c984e0→e05040 미지(+143B)
- 0.5.8 명세 #203 `passive_jungle__PassiveJunglePlan__next_plan` src game-ai\src\plan_legacy\old\passive_jungle.rs:228 · one_line: 패시브 정글러의 다음 큰 플랜 결정: 카정 근접 교전(Battle) → 캠프 클리어 후 리드 행동(딥푸시 갱/라인커버/갱 셋업/카정 캠프 전환) → 아이템 점수 확률 귀환(ActiveRecall) → None
- 0.5.8 params: (sret):*mut Option<BigPlan> (384B)(반환 슬롯. None = +0 i64 -1 (L300·L292). Some = +0 태그(tcxdict --) · self:&mut PassiveJunglePlan (104B(IR 속성: noalias align 8 dereferenceable(104) · readonly 없음 = ) · version:usize(본문 분기 없음. BattlePlan::new/update·buy_item·upgrade_item·is_cl) · rnd:&mut StdRng (320B)(&mut. 소비: L291 gen_range(0..=100) · lead_action 내 L422/L548 ) · player:&PlayerState (2528B)(readonly) · data:&OperationData (24B)(readonly. +0 cache · +8 context · +0x10 blackboard[2]) · goal_data:&GoalData (248B)(readonly. has_deep_pushed_enemy 에서 enemy_region[5] 만 읽음) · _positioning_score:&PositioningScoreData (2760B(readonly. 본문 미사용 — BattlePlan::update(L251) 로만 전달) · team_plan:&TeamPlan (1064B)(IR 속성 nonnull align 8 (dereferenceable 없음). 직접 읽기 = +0x378 n) · debug:&mut DebugFrameData (224B)(본문 직접 쓰기 없음. BattlePlan::update(L251) 로 전달 + s3_0 클로저 캡처(val)
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

## `d2e500` → `fa3980` PassiveJunglePlan::sub_plan  (2993→3411B · Δ+418)
- 힌트: PassiveJunglePlan::sub_plan(이관 · 배치 C §2 가 골격 규명: v3 역정글 매복 Hide 블록 · 헬퍼 fb12a0/db75f0/de7750/fafc10 — 남은 것 = 헬퍼 본체·매복 셀 +0x48/+0x50 세팅처)
- exe 정렬: 명령 699→832 · 정렬 475 · 잔여 구조 9 · 분기 10 · 콜리 주의: 12857f0→1643790 변경 ; 12857f0→381e0b3 불일치(mig060 는 1643790) ; 31a01a3→381e0b3 미지 ; 31a01a3→fb12a0 미지 ; 31a0cbf→38219f0 미지(+18B) ; 31a3b40→381ebcf 미지(-3B)
- 0.5.8 명세 #91 `passive_jungle__PassiveJunglePlan_sub_plan` src game-ai\src\plan_legacy\old\passive_jungle.rs:134 · one_line: 패시브 정글 플랜의 서브플랜 — check_recall(인라인)이 참이면 Recall, 아니면 Jungle{team,camp,check_move:false}
- 0.5.8 params: (sret):SubPlan(72B)(%0. 태그 5=Recall / 6=Jungle(payload +8 team, +0x10 camp, +0x1) · self:&PassiveJunglePlan(104B)(%1. team(+0x48)·jungle(+0x60) 만 읽음) · version:usize(%2. effect_buff_target 에 그대로 전달만 — 본문 분기 없음) · rnd:&mut StdRng(320B)(%3. readnone — 미사용) · player:&PlayerState(2528B)(%4) · data:&OperationData(24B)(%5) · debug:&mut DebugFrameData(224B)(%6. readnone — 미사용)
- 0.5.8 logic 전문:
```
sub_plan(L135~140) = if self.check_recall(version,rnd,player,data,debug) { Recall } else { Jungle{team:self.team, camp:self.jungle, check_move:false} }
check_recall 전체가 인라인(passive_jungle.rs:143~206):
L143: team = player.info.team; champ = cache.player_champion[team][player.info.position].unwrap()
L145: (lx,ly,rx,ry) = context.map.fountains[team]
L146: is_in_heal_area = lx<=champ.x<=rx && ly<=champ.y<=ry
L147: if is_in_heal_area && champ.hp < champ.stat_cached.hp { return true(Recall) }   // 분수 안에서 회복 중이면 계속 귀환
L154: moba = cache.game.get_game_mode().as_moba().unwrap(); camp_live = moba.jungle_runner.get_jungle_live_list(self.jungle, self.team == 0)
L155: if !camp_live.is_empty() {
  L156: camp_pos = map.camp_pos(self.jungle, self.team == 0)
  L157: if champ.distance_sq(camp_pos) < 14400000001 {
    L159~162: edpt = camp_live.iter().filter_map(|id| get_entity_by_id(id)).map(|e| e.attack_effect.as_ref().map(|ae| ae.expected_damage_target(ctx, e, champ) * 1000 / e.attack_cooltime()).unwrap_or(0)).sum()   (cooltime 0 → div_by_zero panic)
    L164: die_tick = if edpt == 0 { usize::MAX } else { champ.hp * 1000 / edpt }
    L165: if die_tick > tps { return false(Jungle) }   // 캠프 옆에서 1초 안에 안 죽으면 계속 정글
  } }
L172: can_heal = champ.stat_buff_cached.vamp > 0
  L173~175: || champ.skill_effect.as_ref().map(|e| e.expected_heal_target(ctx, champ, champ) > 0 || effect_buff_target(version, e, ctx, champ, champ).map(|b| b.vamp > 0).unwrap_or(false)).unwrap_or(false)
  L176~178: || champ.skill2_effect()[level>2 게이트].as_ref().map(같은 식).unwrap_or(false)
  (IR 순서: vamp → skill1 heal → skill1 buff → skill2 heal → skill2 buff, 단락 평가)
L180: is_in_fight = get_jungle_live_list(self.jungle, self.team==0).iter().any(|id| L181 get_entity_by_id(id).is_some_and(|e| L182 e.ty == Jungle && L184 e.ty.info.focused == Some(champ.id)))
L191: if is_in_fight {
  L192: camp_live = get_jungle_live_list(…)  (3번째 호출)
  L193~195: edpt = max(camp_live.iter().filter_map(get_entity_by_id).map(|e| e.attack_effect.as_ref().unwrap().expected_damage_target(ctx,e,champ)*1000 / e.attack_cooltime()).sum(), 1)   (★attack_effect None 이면 unwrap 패닉 — L162 와 달리 unwrap_or 없음)
  L197: die_tick = champ.hp * 1000 / edpt
  L199: return die_tick <= tps   // 1초 안에 죽으면 Recall, 아니면 Jungle
} else {
  L205: hp_ratio = champ.hp * 100 / champ.stat_cached.hp   (max 0 → div_by_zero panic)
  L206: return if can_heal { hp_ratio < 21 } else { hp_ratio < 41 }
}
```
