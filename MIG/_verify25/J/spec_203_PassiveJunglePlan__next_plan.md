---

### `203` PassiveJunglePlan::next_plan — 패시브 정글러의 다음 큰 플랜 결정: 카정 근접 교전(Battle) → 캠프 클리어 후 리드 행동(딥푸시 갱/라인커버/갱 셋업/카정 캠프 전환) → 아이템 점수 확률 귀환(ActiveRecall) → None

| 항목 | 값 |
|---|---|
| id | `passive_jungle__PassiveJunglePlan__next_plan` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passive_jungleNtB2_17PassiveJunglePlan9next_plan` |
| 소스 | `game-ai\src\plan_legacy\old\passive_jungle.rs:228` |
| IR | `m04.ll` 29370~32748행 |
| 경로·가시성 | `game_ai::plan_legacy::old::PassiveJunglePlan::next_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d2f180` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[203]/sig/tls/<키>`)**

- `direct`: 없음 — 본문(29370~32748)과 aux 12조각 어디에도 LocalKey::with / thread_local 접근이 없다 (grep LocalKey = 0)
- `indirect`: [{"name": "BattlePlan::update (L251 · m10.ll 23429)", "role": "소비자/작성자 (r14 자식 명세 소관)", "call_conditions": "is_counter_jungle && hp_ratio>49 && 근접 가시 적 존재 && (hp_ratio>=enemy_hp_ratio || my_level>enemy_level) 일 때 1회. 이 안에서 position_eval_at(POS_EVAL_CACHE)·interaction_score(INTER_CTX)·champion_hp_value(HP_VALUE_MEMO)·v47_siege_stance·v48_cast_beams 가 발화하는지는 BattlePlan::update 명세를 볼 것 — next_plan 은 그 순서에 관여하지 않는다(호출 1회, 그 전후 다른 TLS 접점 없음)"}, {"name": "MapDef::camp_pos → CAMP_POS_MEMO (game_core g07.ll 152570 · LocalKey<RefCell<(usize,[[Option<(u64,u64)>;2];8])>>)", "role": "소비자+작성자(game_core 순수 메모: MapDef 기하의 캐시)", "key": "(JungleType i8, bool) — 첫 usize 는 메모 유효성 키(추정: 맵 식별) · 미탐색", "call_conditions": "CounterJungle 분기에서만: L585(camp_a) → L592(camp_a, best 후보일 때) → L585(camp_b) → L592(camp_b) → L619(best) 순. 순수 함수 메모라 미러 순서 무관(추정 — game_core 경계, 시그니처만 확인)"}, {"name": "is_cleared(m04.ll 62404) · buy_item(m14.ll 8267) · upgrade_item(m14.ll 6620) · evaluate_gank_opportunity_with_score(m04.ll 62981)", "role": "-", "call_conditions": "각 define 본문 grep LocalKey = 0 (직접 접점 없음 · 그 안의 콜리는 미확인)"}]

<details><summary>인자 10개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<BigPlan> (384B) | 반환 슬롯. None = +0 i64 -1 (L300·L292). Some = +0 태그(tcxdict --enum BigPlan 메모리태그) + 페이로드 +8.. | 3 |
| 1 | 1 | self | &mut PassiveJunglePlan (104B) | IR 속성: noalias align 8 dereferenceable(104) · readonly 없음 = &mut · initializes 속성 없음(부분 쓰기) → writes 전수 기재 | 4 |
| 2 | 2 | version | usize | 본문 분기 없음. BattlePlan::new/update·buy_item·upgrade_item·is_cleared(poison 전달=미사용)에 그대로 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | &mut. 소비: L291 gen_range(0..=100) · lead_action 내 L422/L548 gen_range(0..1000) · BattlePlan::update · buy_item · upgrade_item · PlayerState::strategy · evaluate_gank_opportunity_with_score | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly | 4 |
| 5 | 5 | data | &OperationData (24B) | readonly. +0 cache · +8 context · +0x10 blackboard[2] | 4 |
| 6 | 6 | goal_data | &GoalData (248B) | readonly. has_deep_pushed_enemy 에서 enemy_region[5] 만 읽음 | 4 |
| 7 | 7 | _positioning_score | &PositioningScoreData (2760B) | readonly. 본문 미사용 — BattlePlan::update(L251) 로만 전달 | 4 |
| 8 | 8 | team_plan | &TeamPlan (1064B) | IR 속성 nonnull align 8 (dereferenceable 없음). 직접 읽기 = +0x378 next_respawn_tick[적팀][camp_idx] (L591). is_cleared·BattlePlan::update 에 전달 | 4 |
| 9 | 9 | debug | &mut DebugFrameData (224B) | 본문 직접 쓰기 없음. BattlePlan::update(L251) 로 전달 + s3_0 클로저 캡처(valid_gank_line 인자, 본문에서 미사용) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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
 strategy = player.strategy(rnd, game)                                   // L443
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
   //     if ms.minion_power > 3000 → false ; else true   L678
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
     pos = map.camp_pos(camp, player.team == 1)                           // L585 (bool = 내 팀이 1인가)
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

**`mem` 메모리 접근 53건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PassiveJunglePlan | 0x0 | chats.cap | r | push 시 len==cap 비교(grow_one) | 4 | OK |  |
| 1 | PassiveJunglePlan | 0x8 | chats.ptr | r | push 대상 | 4 | OK |  |
| 2 | PassiveJunglePlan | 0x10 | chats.len | r |  | 4 | OK |  |
| 3 | PassiveJunglePlan | 0x48 | team | r | L231 is_counter_jungle · L261/L456/L514 is_cleared 인자 | 4 | OK |  |
| 4 | PassiveJunglePlan | 0x50 | player_team | r | L231 | 4 | OK |  |
| 5 | PassiveJunglePlan | 0x58 | last_lead_action_tick | r | L285 → lead_action_v37 L393 쿨다운 | 4 | OK |  |
| 6 | PassiveJunglePlan | 0x60 | jungle | r | JungleType i8 · is_cleared/is_side_cleared 인자 | 4 | OK |  |
| 7 | PlayerState | 0x180 | info.parameter | r | &AthleteParameter → ego_ratio/aggressive_ratio/order_ratio | 4 | OK |  |
| 8 | PlayerState | 0x4a8 | info.items.len | r | L278 ==0 이면 item_score 10 | 4 | OK |  |
| 9 | PlayerState | 0x930 | info.team | r | L232/L400/L403 등 · 적팀 = 1-team | 4 | OK |  |
| 10 | PlayerState | 0x990 | info.level | r | L244 my_level · L246 enemy_level(적 PlayerState) | 4 | OK |  |
| 11 | PlayerState | 0x9c0 | info.position(tag i32) | r | player_champion[team][position] 인덱스 | 4 | OK |  |
| 12 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 13 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 14 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] (744B stride) | 4 | OK |  |
| 15 | AbstractGameWithCache | 0x0 | game.data_ptr | r | dyn AbstractGame | 4 | OK |  |
| 16 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 0x28 tick · 0x40 get_game_mode · 0x1f0 get_entity_by_id(aux s1_0) · 0x208 iter_player | 4 | OK |  |
| 17 | AbstractGameWithCache | 0x130 | twin_towers[team] | r | aux s1_0 L476 | 4 | OK |  |
| 18 | AbstractGameWithCache | 0x170 | nexus[team] | r | aux s1_0 L482 | 4 | OK |  |
| 19 | AbstractGameWithCache | 0x180 | top_tower[team] (+0x190 top_tower2 · +0x1a0/0x1b0 mid · +0x1c0/0x1d0 bottom) | r | has_deep_pushed_enemy L307 · s1_0 L475: line*32 stride, cache+team*8 | 4 | OK |  |
| 20 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | iter_champions 슬라이스 · 자기 챔프 lookup | 4 | OK |  |
| 21 | AbstractGameWithCache | 0x2218 | region_point[27] (i32) | r | CounterJungle L587 (team==1 이면 부호 반전) | 4 | OK |  |
| 22 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 23 | GameContext | 0x20 | map | r | &MapDef | 4 | OK |  |
| 24 | GameContext | 0x30 | item_list | r | &Vec<Box<dyn ItemInfo>> (+8 ptr · +0x10 len) · L268 | 4 | OK |  |
| 25 | GameContext | 0x38 | tutorial | r | L439 is_line_phase: {0 None,5 MidBottom,7 Line,8 Total} 만 시간 게이트 적용 | 4 | OK |  |
| 26 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | L439/703 리드행동 시간 게이트 | 4 | OK |  |
| 27 | GameSetting | 0x12c0 | height | r | L410/457/516/575 챔프가 어느 사이드인가(x > height-y → Bottom) | 4 | OK |  |
| 28 | GameSetting | 0x12f8 | tick_per_second | r | tps | 4 | OK |  |
| 29 | GoalData | 0x0 | enemy_region[i].tag (stride 0x18 · +0x8 region · +0x10 last_known) | r | has_deep_pushed_enemy L316~319 | 4 | OK |  |
| 30 | MapDef | 0x38b8 | regions[30][30] | r | CounterJungle L586 pos/32000 → region | 4 | OK |  |
| 31 | MapDef | 0x54d8 | region_dist[27][27] | r | has_deep_pushed_enemy L326 | 4 | OK |  |
| 32 | TeamPlan | 0x378 | next_respawn_tick[2][4] | r | CounterJungle L591 [적팀][camp_idx] | 4 | OK |  |
| 33 | Blackboard | 0x0 | top_minion_state (0x28 mid · 0x50 bottom): +0 front_minion.tag · +8 id · +0x10 minion_power · +0x20 minion_count(i32) | r | L465 minion_count<-2 · valid_gank_line L667/678 · s1_0 L488 | 4 | OK |  |
| 34 | Entity | 0x68 | ty.tag | r | has_deep_pushed_enemy: ==2 Tower | 4 | OK |  |
| 35 | Entity | 0x128 | ty@Tower.info.ty (TowerType) | r | <5 이면(1차 타워 생존) 딥푸시 아님 | 4 | OK |  |
| 36 | Entity | 0x5c0 | id | r | 적 id → BattlePlanGoal::TryKill · Chat::Battle | 4 | OK |  |
| 37 | Entity | 0x628 | stat_cached.hp | r | hp_ratio 분모(0 이면 div0 패닉) | 4 | OK |  |
| 38 | Entity | 0x640 | stat_cached.move_speed | r | CounterJungle L594 travel = dist/move_speed | 4 | OK |  |
| 39 | Entity | 0x660 | x | r |  | 4 | OK |  |
| 40 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 41 | Entity | 0x670 | hp | r |  | 4 | OK |  |
| 42 | BattlePlan | 0x58 | sub_goal.tag | r | L252 update 후 ==4(RunAway) 이면 폐기 | 4 | OK |  |
| 43 | Strategy | 0x11 | focused | r | FocusedAreaStrategy Top0/Bottom1/All2 · L444 | 4 | OK |  |
| 44 | Strategy | 0x12 | early_jungle | r | EarlyJungleStrategy GrowthAndCover0/Ganking1/CounterJungle2 · L445/447 | 4 | OK |  |
| 45 | JungleCampState | 0x10 | live_list.len | r | CounterJungle L597 ==0 이면 캠프 죽음 | 4 | OK |  |
| 46 | MobaMode | 0x18 | jungle_runner | r | get_game_mode(tag 0 Moba).payload+0x18 | 4 | OK |  |
| 47 | PassiveJunglePlan | 0x0/0x8/0x10 | chats (Vec<Chat>) | w | 6 사이트: L253 Chat::Battle{tag3,+8 enemy_id,+0x10 0} · L423 HideLine{tag11,+1 line,+8 0}(딥푸시) · L470 CoverLine{tag9,+1 line,+8 0} · L498 GankLineCover{tag10,+1 line,+8 0} · L549 HideLine{tag11,+1 line,+8 0}(갱) · L623 CounterJungle{tag18,+1 camp(JungleType),+8 0}. L423/L549 는 rnd.gen_range(0..1000) < order*600/1000+400 일 때만 | 4 | OK | push 1건 (grow_one 시 cap/ptr 갱신, len+=1) |
| 48 | PassiveJunglePlan | 0x58 | last_lead_action_tick | w | 5 사이트: L419(딥푸시 갱 확정) · L469(라인커버 확정) · L497(갱커버 확정) · L545(갱 셋업 확정) · L613(카정 캠프 전환 확정) | 4 | OK | game.tick() |
| 49 | PassiveJunglePlan | 0x60 | jungle | w | L625 CounterJungle 분기 종료 시(반환은 None) | 4 | OK | best camp (JungleType) |
| 50 | PassiveJunglePlan | 0x48 | team | w | L626 CounterJungle 분기 종료 시 → 이후 is_counter_jungle 이 true 가 됨 | 4 | OK | 1 - player.info.team (적팀) |
| 51 | (sret) | 0x0 | Option<BigPlan> | w | signature.returns 참조 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | -1 \| 9 \| 8 \| 10 \| 11 \| 3 |
| 52 | BattlePlan(스택 %33) | 0x107 | entry_src | w | L250 new 직후·update 전 (이후 sret 로 memcpy) | 4 | OK | 9 |

**`consts` 상수 49건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 232 | 임계 | player_champion[team] 바운드(팀 2) · L482/L357 등 반복 | 4 |  |
| 1 | 100 | 233 | 계수 | hp*100/stat.hp = hp_ratio(%). L242/L402/L452/L509/L561/aux L370/L638/L655 동일 | 4 |  |
| 2 | 49 | 235 | 임계 | 카정 교전 진입 hp_ratio > 49 (= 50% 이상) | 4 |  |
| 3 | 14400000001 | 237 | 임계 | 120000^2+1 — 근접 적 탐색 반경(제곱 <) = 3.75셀 | 4 |  |
| 4 | 1 | 246 | 태그 | enemy_player 못 찾으면 enemy_level=1 · Option 태그 1=Some(buy_item .0 / upgrade_item +0) · (shl 피연산자 1 = tps*2 접힘은 다음 항목 folded_from 2 참조) | 4 |  |
| 5 | 1 | 604 | 태그 | tps*2 (카정 리스폰 허용 = 도착 2초 전). `shl i64 %tps, 1`(IR 32279) 로 접힘 | 4 | 2 |
| 6 | 60 | 249 | 산출값 | BattlePlanGoal::TryKill(enemy_id, 60) 둘째 필드(리터럴) · L407 hp_gate 기준 60 | 4 |  |
| 7 | 0 | 249 | 태그 | BattlePlanGoal 태그 0 = TryKill · Chat 페이로드 둘째 0 · is_cleared extra 0 | 4 |  |
| 8 | 9 | 250 | 태그 | battle.entry_src = 9 · BigPlan::Battle 메모리태그 9(L254) · Chat::CoverLine 태그 9(L470) | 4 |  |
| 9 | 4 | 252 | 태그 | BattleSubPlanGoal 태그 4 = RunAway → Battle 폐기 · L269 tier==4 → 100 · L391 cooldown_sec = 4 - ego/500 · camp_idx 바운드 4 | 4 |  |
| 10 | 3 | 253 | 태그 | Chat 태그 3 = Battle · BigPlan::PassiveLine 태그 3(L471) · L271 tier==3 → 70 · L319 tps*3 · L865 Rhino↔Bee 짝 · ⚠`shl 3`(=*8 포인터 stride, aux s4_0/s1_0)는 임계 아님 | 4 |  |
| 11 | 112 | 269 | 미상 | ItemInfo vtable 슬롯 0x70 = tier() (divtable) | 3 |  |
| 12 | 70 | 271 | 산출값 | tier 3 → item_score 70 · L455/513/563 hp_gate2 = 70 - agg*20/1000 | 4 |  |
| 13 | 50 | 273 | 임계 | tier 2 → item_score 50 · L348 딥푸시 갱 hp_ratio < 50 이면 스킵 | 4 |  |
| 14 | 30 | 273 | 계수 | tier ≤1 → item_score 30 · L704 tps*30 · L537 agg*30 | 4 |  |
| 15 | 10 | 278 | 태그 | 아이템 살 수 있고 items.len==0 이면 item_score 10 · BigPlan::LineGanker 태그 10(L426/552) · Chat::GankLineCover 태그 10(L498) · L499/554 tps*10 | 4 |  |
| 16 | 8 | 292 | 태그 | BigPlan::ActiveRecall 메모리태그 8 (gen < item_score) · 빈 Vec dangling ptr 8 | 4 |  |
| 17 | -1 | 300 | 태그 | Option<BigPlan>::None (+0 i64 -1) · find 결과 없음 i8 -1 | 4 |  |
| 18 | 500 | 391 | 계수 | cooldown_sec = 4 - ego_ratio/500 | 4 |  |
| 19 | 20 | 407 | 계수 | hp_gate = 60 - aggressive*20/1000 (L455/513/563: 70 - agg*20/1000) | 4 |  |
| 20 | 1000 | 407 | 계수 | 비율 분모(aggressive/order ratio 스케일 0..1000 추정) · gen_range(0..1000) | 5 |  |
| 21 | 5 | 316 | 임계 | 적 슬롯 5 (goal_data.enemy_region[5] · (0..5)) · L865 is_cleared extra tps*5 · Tower2 타입 하한 5 · ⚠`shl i8 line, 5`(=*32 타워 슬롯 stride, L307)는 임계 아님 | 4 |  |
| 22 | 27 | 326 | 임계 | region 개수 27 바운드(region_dist[27][27] · region_point[27]) | 4 |  |
| 23 | 21720 | 326 | 미상 | MapDef+0x54d8 region_dist 오프셋(gep 상수) | 4 |  |
| 24 | 2 | 326 | 임계 | region_dist[deep_region][enemy_region] < 2 → 딥푸시 적 존재 · L324 idx 1..=2 · L465/588 minion_count/region_point < -2 | 4 |  |
| 25 | 400 | 422 | 미상 | 콜(HideLine 채팅) 확률 = order*600/1000 + 400 / 1000 (L548 동일) | 4 |  |
| 26 | 600 | 422 | 계수 | order_ratio 가중 | 4 |  |
| 27 | 12 | 429 | 계수 | 딥푸시 갱 LineGanker wait_limit = tick + tps*12 (갱 셋업은 tps*10) | 4 |  |
| 28 | 7 | 426 | 센티널 | LineGankerPhase 메모리태그 7 = Setup (니치 +6) · BigPlan PassiveJungle 태그 7(미사용) · tutorial Line 7 | 4 |  |
| 29 | 11 | 423 | 태그 | Chat 태그 11 = HideLine · BigPlan::LineGankCover 태그 11(L499) | 4 |  |
| 30 | -2 | 465 | 임계 | 라인커버 조건 bb[team].line.minion_count < -2 · L588 region_point(부호 보정) < -2 면 카정 후보 탈락 | 4 |  |
| 31 | 2216 | 703 | 미상 | GameSetting+0x8a8 epic_jungle.first_spawn_tick (gep 상수) | 4 |  |
| 32 | 30624999999 | 490 | 미상 | 175000^2 - 1 — 최전방 미니언이 내 타워/쌍둥이/넥서스에서 >= 175000 떨어져야 라인커버 (aux s1_0) | 4 |  |
| 33 | 6400000001 | 490 | 미상 | 80000^2+1 — 그 라인 아군이 최전방 미니언에 < 80000 이면 라인커버 확정 (aux s1_0) | 4 |  |
| 34 | -40 | 537 | 미상 | min_gank_score = -40 - aggressive*30/1000 (i32 · sdiv -1000) | 4 |  |
| 35 | -1000 | 537 | 계수 | agg*30 / -1000 = -(agg*30/1000) | 4 |  |
| 36 | 18 | 623 | 태그 | Chat 태그 18 = CounterJungle(camp, 0) | 4 |  |
| 37 | 90000000000 | 620 | 임계 | 300000^2 — best 캠프가 챔프에서 > 300000(9.4셀) 이면 카정 콜 채팅 생략 | 4 |  |
| 38 | 32000 | 586 | 인덱스 | 셀 크기 — camp_pos/32000 → regions[y][x] 인덱스 | 4 |  |
| 39 | 960000 | 586 | 임계 | 30셀*32000 — regions[30][30] 바운드 검사(pos < 960000) | 4 |  |
| 40 | 14520 | 586 | 미상 | MapDef+0x38b8 regions 오프셋(gep 상수) | 4 |  |
| 41 | 8728 | 587 | 미상 | AbstractGameWithCache+0x2218 region_point (gep 상수) | 4 |  |
| 42 | 888 | 591 | 미상 | TeamPlan+0x378 next_respawn_tick (gep 상수) | 4 |  |
| 43 | 19600000001 | 667 | 미상 | 140000^2+1 — (aux valid_gank_line) 내 라인 minion_count>1 이면 적이 자기 타워 140000 안이면 갱 불가 | 4 |  |
| 44 | 10000000001 | 672 | 미상 | 100000^2+1 — (aux valid_gank_line) 적이 자기 타워 100000 안이면 갱 불가 | 4 |  |
| 45 | 3000 | 678 | 미상 | (aux valid_gank_line) bb[team].line.minion_power > 3000 이면 갱 불가 | 4 |  |
| 46 | 59 | 638 | 미상 | (aux valid_gank_line closure#0) 적 hp_ratio > 59 만 갱 대상 | 4 |  |
| 47 | 40 | 655 | 산출값 | (aux s2_0/vdpg s1_0) 아군 hp_ratio > 40 만 셈 | 4 |  |
| 48 | 480 | 232 | 미상 | AbstractGameWithCache+0x1e0 player_champion (gep 상수) | 4 |  |

**`knobs` 조정점 25건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 카정 교전 진입 최소 체력 | passive_jungle.rs:235 | 49 | 올리면 카정 중 적을 만나도 더 건강할 때만 교전(Battle) 시도 | 4 | 기존 |
| 1 | 카정 근접 적 탐색 반경(제곱) | passive_jungle.rs:237 | 14400000001 | 올리면 더 먼 적까지 교전 대상 | 4 | 기존 |
| 2 | TryKill 목표 둘째 파라미터 | passive_jungle.rs:249 | 60 | BattlePlanGoal::TryKill(id, 60) 로 전달 — 의미는 BattlePlan 명세(r14) 참조 | 4 | 기존 |
| 3 | 아이템 티어별 귀환 점수 | passive_jungle.rs:269~273 | 4→100 / 3→70 / 2→50 / 그외→30 | gen_range(0..=100) < 점수 확률로 ActiveRecall. 100 이면 사실상 확정(gen 최대 100 은 <100 실패 ≈ 1/101) | 4 | 기존 |
| 4 | 첫 아이템 구매 귀환 점수 | passive_jungle.rs:278 | 10 | 살 수 있고 아이템 0개면 ~10% 귀환 | 4 | 기존 |
| 5 | 리드행동 쿨다운 초 | passive_jungle.rs:391 | 4 - ego/500 | ego 0→4초, 1000→2초. 상수 4 올리면 리드 행동 빈도 감소 | 4 | 기존 |
| 6 | 딥푸시 갱 체력 게이트 | passive_jungle.rs:407 | 60 - agg*20/1000 | 공격성 높을수록 낮은 체력에도 시도 | 4 | 기존 |
| 7 | 딥푸시 판정 적 정보 신선도 | passive_jungle.rs:319 | tps*3 | 적 위치 정보가 3초보다 오래되면 무시 | 4 | 기존 |
| 8 | 딥푸시 region 거리 | passive_jungle.rs:326 | 2 | 적이 우리 딥 라인 region 에서 <2 거리면 딥푸시 | 4 | 기존 |
| 9 | 딥푸시 갱 수적 허용 | passive_jungle.rs:379 | enemies.len <= allies.len + 1 | 적이 아군+1 이하일 때만 | 4 | 기존 |
| 10 | 콜 채팅 확률 | passive_jungle.rs:422/548 | order*600/1000 + 400 (/1000) | order_ratio 0→40%, 1000→100% | 4 | 기존 |
| 11 | LineGanker 대기 한계 | passive_jungle.rs:429/554 | tps*12 (딥푸시) / tps*10 (갱) | 올리면 갱 대기 길어짐 | 4 | 기존 |
| 12 | 리드행동 시간 게이트 | passive_jungle.rs:439/703 | first_spawn_tick - tps*30 | 에픽 첫 스폰 30초 전 이후 리드행동 전면 금지(라인 페이즈 튜토리얼 유형에서) | 4 | 기존 |
| 13 | 라인커버 체력 게이트(3분기 공통) | passive_jungle.rs:455/513/563 | 70 - agg*20/1000 |  | 4 | 기존 |
| 14 | 짝 캠프 클리어 유예 | passive_jungle.rs:865~868 | tps*5 | is_side_cleared 둘째 is_cleared 의 extra 인자 | 4 | 기존 |
| 15 | 라인커버 미니언 수 임계 | passive_jungle.rs:465 | -2 | minion_count < -2 (우리 라인이 밀림) 이면 PassiveLine 커버 | 4 | 기존 |
| 16 | 갱커버 최전방-타워 거리 | passive_jungle.rs:490 | 30624999999 | 최전방 미니언이 타워에서 ≥175000 이어야 | 4 | 기존 |
| 17 | 갱커버 아군-최전방 거리 | passive_jungle.rs:490 | 6400000001 | 아군이 최전방 80000 안에 있어야 | 4 | 기존 |
| 18 | 갱 최소 점수 | passive_jungle.rs:537 | -40 - agg*30/1000 | 낮출수록 나쁜 갱도 시도 | 4 | 기존 |
| 19 | valid_gank_line 적 체력 하한 | passive_jungle.rs:638 | 59 | 적 hp>59% 만 갱 대상(빈사 적은 오히려 제외) | 4 | 기존 |
| 20 | valid_gank_line 타워 안전 반경 | passive_jungle.rs:667/672 | 140000(우리 미니언>1) / 100000 |  | 4 | 기존 |
| 21 | valid_gank_line 미니언 파워 상한 | passive_jungle.rs:678 | 3000 |  | 4 | 기존 |
| 22 | 카정 후보 region_point 하한 | passive_jungle.rs:588 | -2 | 적 캠프 region 의 region_point(팀 부호 보정) < -2 면 제외 | 4 | 기존 |
| 23 | 카정 리스폰 허용 | passive_jungle.rs:604 | respawn <= tick + travel + tps*2 | 도착 2초 안에 리스폰될 캠프만 | 4 | 기존 |
| 24 | 카정 콜 거리 | passive_jungle.rs:620 | 90000000000 | 살아있는 best 캠프가 300000 이내일 때만 CounterJungle 채팅 | 4 | 기존 |

<details><summary>`callees` 피호출자 65건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | aggressive_ratio | game_core::AthleteParameter::aggressive_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:432 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | buy_item | game_ai::buy_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> | game-ai\src\lib.rs:1477 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | camp_idx | game_ai::plan_legacy::team_plan::camp_idx | pub | fn(game_core::JungleType) -> usize | game-ai\src\plan_legacy\team_plan.rs:34 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | ego_ratio | game_core::AthleteParameter::ego_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:439 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | evaluate_gank_opportunity_with_score | game_ai::plan_legacy::old::evaluate_gank_opportunity_with_score | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, i32) -> (bool, i32, i32) | game-ai\src\plan_legacy\old\passive_jungle.rs:693 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | find | game_ai::MinionWaveSnapshot::find | pub | fn(&game_ai::MinionWaveSnapshot, usize) -> std::option::Option<&game_ai::MinionHpTrajectory> | game-ai\src\utils.rs:84 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | get_camp_state | game_core::JungleRunner::get_camp_state | pub | fn(&game_core::JungleRunner, usize, game_core::JungleType) -> &game_core::JungleCampState | game-core\src\simulation\entity\jungle.rs:702 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 18 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | has_deep_pushed_enemy | game_ai::plan_legacy::old::PassiveJunglePlan::has_deep_pushed_enemy | in:game_ai::plan_legacy::old::passive_jungle | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::LineType) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:305 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | in_big_line | game_core::Blackboard::in_big_line | pub | fn(&game_core::Blackboard, usize, game_core::LineType) -> bool | game-core\src\simulation\game\blackboard.rs:137 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | is_cleared | game_ai::plan_legacy::old::is_cleared | pub | fn(game_core::JungleType, usize, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:841 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | is_counter_jungle | game_ai::plan_legacy::old::PassiveJunglePlan::is_counter_jungle | pub | fn(&game_ai::plan_legacy::old::PassiveJunglePlan) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:103 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 23 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 24 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 25 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | is_focused_line | game_core::FocusedAreaStrategy::is_focused_line | pub | fn(&game_core::FocusedAreaStrategy, game_core::LineType) -> bool | game-core\src\simulation\strategy.rs:162 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 27 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 28 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 29 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | is_side_cleared | game_ai::plan_legacy::old::is_side_cleared | pub | fn(game_core::JungleType, usize, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:861 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | iter_player | game_core::AbstractGame::iter_player | pub | fn(&Self/#0) -> game_core::PlayerIter | game-core\src\simulation.rs:181 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 34 | iter_player | <game_core::Game as game_core::AbstractGame>::iter_player | pub | fn(&game_core::Game) -> game_core::PlayerIter | game-core\src\simulation\game.rs:3756 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 35 | iter_player | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_player | pub | fn(&game_core::SingleLaneGame) -> game_core::PlayerIter | game-core\src\simulation\game.rs:4015 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 36 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | lead_action | game_ai::plan_legacy::old::PassiveJunglePlan::lead_action | in:game_ai::plan_legacy::old::passive_jungle | fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> | game-ai\src\plan_legacy\old\passive_jungle.rs:685 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 38 | line_region | game_core::line_region | pub | fn(game_core::LineType, usize, usize) -> usize | game-core\src\simulation\path_finder.rs:237 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | line_region | game_core::MapDef::line_region | pub | fn(&game_core::MapDef, game_core::LineType, usize, usize) -> usize | game-core\src\simulation\map_def.rs:239 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 41 | new | game_ai::plan_legacy::old::BattlePlan::new | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan | game-ai\src\plan_legacy\old\battle.rs:196 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 42 | next_plan | game_ai::plan_legacy::types::BigPlan::next_plan | pub | fn(&mut game_ai::plan_legacy::types::BigPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> | game-ai\src\plan_legacy\types.rs:297 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 43 | next_plan | game_ai::plan_legacy::old::BattlePlan::next_plan | pub | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> | game-ai\src\plan_legacy\old\battle.rs:2034 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 44 | next_plan | game_ai::plan_legacy::old::SinglePlanLine::next_plan | pub | fn(&mut game_ai::plan_legacy::old::SinglePlanLine, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> | game-ai\src\plan_legacy\old\single_line.rs:644 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 45 | order_ratio | game_core::AthleteParameter::order_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:446 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 46 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 47 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 48 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 49 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 50 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 51 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 52 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 53 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 54 | tier | game_core::ItemInfo::tier | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:437 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 55 | tier | <game_core::ModItemEntry as game_core::ItemInfo>::tier | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:499 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 56 | tier | <game_core::BaseItemInfo as game_core::ItemInfo>::tier | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:617 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 57 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 58 | update | game_ai::plan_legacy::old::BattlePlan::update | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\battle.rs:427 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 59 | upgrade_item | game_ai::upgrade_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1603 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 60 | valid_deep_push_gank | game_ai::plan_legacy::old::PassiveJunglePlan::valid_deep_push_gank | in:game_ai::plan_legacy::old::passive_jungle | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:337 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 61 | valid_gank_line | game_ai::plan_legacy::old::PassiveJunglePlan::valid_gank_line | in:game_ai::plan_legacy::old::passive_jungle | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:636 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 62 | version | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | fn(&game_ai::AgentVerHamster) -> usize | game-ai\src\lib.rs:428 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 63 | version | game_core::AiAgent::version | pub | fn(&Self/#0) -> usize | game-core\src\simulation\ai_interface.rs:503 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 64 | version | <game_core::Team as engine_core::spitz::SpitzDatable>::version | pub | fn() -> usize | game-core\src\data\team.rs:1696 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
</details>

⚠**미매칭 9개**: `dist_sq`, `early_jungle`, `focused`, `gen_range`, `grow_one`, `hp_ratio`, `pair`, `side`, `tower2`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:8625) · **형제 16개** (PassiveJunglePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::PassiveJunglePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:20 | True | fn(&game_ai::plan_legacy::old::PassiveJunglePlan) -> game_ai::plan_legacy::old::PassiveJunglePlan |
| 1 | <game_ai::plan_legacy::old::PassiveJunglePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:20 | True | fn(&game_ai::plan_legacy::old::PassiveJunglePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::PassiveJunglePlan::new | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:35 | False | fn(&mut rand::rngs::std::StdRng, usize) -> game_ai::plan_legacy::old::PassiveJunglePlan |
| 3 | game_ai::plan_legacy::old::PassiveJunglePlan::with_best | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:51 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::PassiveJunglePlan |
| 4 | game_ai::plan_legacy::old::PassiveJunglePlan::new_counter_jungle | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:63 | False | fn(&mut rand::rngs::std::StdRng, usize, game_core::FocusedAreaStrategy) -> game_ai::plan_legacy::old::PassiveJunglePlan |
| 5 | game_ai::plan_legacy::old::PassiveJunglePlan::is_counter_jungle | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:103 | True | fn(&game_ai::plan_legacy::old::PassiveJunglePlan) -> bool |
| 6 | game_ai::plan_legacy::old::PassiveJunglePlan::update | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:107 | False | fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) |
| 7 | game_ai::plan_legacy::old::PassiveJunglePlan::goal | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:130 | True | fn(&game_ai::plan_legacy::old::PassiveJunglePlan) -> game_core::BigGoal |
| 8 | game_ai::plan_legacy::old::PassiveJunglePlan::sub_plan | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:134 | False | fn(&game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |
| 9 | game_ai::plan_legacy::old::PassiveJunglePlan::check_recall | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:142 | False | fn(&game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 10 | game_ai::plan_legacy::old::PassiveJunglePlan::next_plan | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:228 | False | fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 11 | game_ai::plan_legacy::old::PassiveJunglePlan::has_deep_pushed_enemy | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:305 | False | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::LineType) -> bool |
| 12 | game_ai::plan_legacy::old::PassiveJunglePlan::valid_deep_push_gank | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:337 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::LineType, &mut game_core::DebugFrameData) -> bool |
| 13 | game_ai::plan_legacy::old::PassiveJunglePlan::lead_action_v37 | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:386 | False | fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 14 | game_ai::plan_legacy::old::PassiveJunglePlan::valid_gank_line | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:636 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, game_core::LineType, &mut game_core::DebugFrameData) -> bool |
| 15 | game_ai::plan_legacy::old::PassiveJunglePlan::lead_action | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:685 | False | fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |

**`open` 10건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L403/L510 `eplayer` 의 `find` 술어가 `p.info.team == 1-team` 인 것은 IR 로 확정이나, 원소 순서상 첫 매치(적팀 첫 선수)를 쓴다 — 의도(특정 포지션?)는 IR 로 알 수 없음 | 4 |  |
| 1 | 미탐색 | L585 `camp_pos(camp, player.team==1)` 의 bool 의미(어느 팀 캠프 좌표를 주나) — game_core 경계, 시그니처만 확인. CAMP_POS_MEMO 키의 첫 usize 도 미탐색 | 4 |  |
| 2 | 미탐색 | aggressive_ratio/ego_ratio/order_ratio 의 스케일(0..1000 추정 — 분모 1000/500 에서 역추정) | 5 |  |
| 3 | 미탐색 | in_big_line(bb, idx, line) 의 정확한 의미 — bb.big_goal[idx] 를 읽어 line 과 비교(g07.ll 156563 첫 40줄만 확인) '선수 idx 의 큰 목표가 그 라인' 으로 추정 | 4 |  |
| 4 | 미탐색 | evaluate_gank_opportunity_with_score(rnd, player, data, line, 0) -> (bool, i32, i32) 의 내부 — 계약만(.1 을 점수로 사용, 다섯째 인자 리터럴 0) | 4 |  |
| 5 | 미탐색 | buy_item/upgrade_item 내부 — 계약만(Option<usize> / Option<(usize,usize)>, .1 이 item_list 인덱스) | 4 |  |
| 6 | 미탐색 | is_cleared 의 version/rnd/debug 인자가 poison 으로 전달됨 = 그 함수 안에서 미사용(ArgumentPromotion) — r13 잎 명세 참조 | 4 |  |
| 7 | 미탐색 | BigPlan::Battle 페이로드 280B 안의 미기록 바이트는 BattlePlan::new/update 산출물에 종속 — 본 명세 범위 밖 | 4 |  |
| 8 | 미탐색 | GameSetting+0x12c0 `height` 로 사이드 판정: `x > height - y` → Bottom — 맵 좌표계(원점) 가정은 IR 밖 | 4 |  |
| 9 | 미탐색 | L1306 unwrap_failed(get_game_mode 가 Moba 아님)는 reach 판정 사장(NA · gamemode=0) — 봉인 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L408 의 `A \|\| B \|\| C` 실제 소스 순서 — column 정보 부재. IR 평가순(hp_gate → eplayer None → is_recent_visible)만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

