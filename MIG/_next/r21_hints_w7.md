# r21 티어1 심층 — 웨이브 7 (3) · 0.5.8 명세 logic 전문 + exe 힌트

요구: 0.5.8 명세(아래 logic) 를 기준선으로 0.6.0 디컴을 **줄 단위로 대조**해 「어느 줄이 어떻게 바뀌었나 / 새 분기·조건·상수·필드·콜리」 를 명세 형식(줄번호 참조 · 등가 Rust)으로 낸다. version≥3 분기는 「v3 경로」 로 별도 표기(값 미확정 · v2/v3 둘 다 적는다). 대형 함수는 먼저 콜리 지도·블록 지도(0.5.8 ↔ 0.6.0 매핑)를 만들고 신규 구역을 특정한 뒤 신규 구역만 정독한다.

## `dcee40` → `f024d0` TeamPlan::handle_none_or_gank_objective  (14041→23078B · Δ+9037)
- 힌트: TeamPlan::handle_none_or_gank_objective(14→23KB · f024d0 = objective phase 8 핸들러 · +0xcd5 전이 4/2/3/0xa/0xb/0 · +0xcc7 3회)
- exe 정렬: 명령 3112→4806 · 정렬 2608 · 잔여 구조 10 · 분기 10 · 콜리 주의: 12920→f22420 미지(+81B) ; 31a01a3→381e0b3 미지 ; 31a04c0→157b720 미지(-20B) ; 31a04c0→2f33b70 미지(+79B) ; 31a04c0→efc390 미지(+17B) ; 31a04c0→f27ea0 미지(+962B) ; 31a05a0→381e3d0 미지(+0B) ; 31a0cbf→3821813 미지(+35B)
- 0.5.8 명세 #107 `objective_handlers__TeamPlan_handle_none_or_gank_objective` src game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830 · one_line: 팀 목표가 None/Gank 일 때 다음 팀 목표(넥서스공격/넥서스수비/라인수비/에픽·세르펜 헌트/타워압박/컴백픽/귀환/에픽·세르펜 셋업/에픽버프 압박)를 우선순위 사슬로 하나 골라 self.objective·plan·chats 에 쓴다
- 0.5.8 params: self:&mut TeamPlan(1064B)(%0. noalias·align8·dereferenceable(1064), readonly 없음 → &mut) · version:usize(%1. AI 버전 게이트. 본문 분기 4곳: L876·L923·L1052·L1119/1126 전부 `vers) · rnd:&mut StdRng(320B)(%2. align16·readonly 없음 → &mut. 본문 직접 사용 0, 콜리 전달만) · player:&PlayerState(2528B)(%3. readonly. info.team(0x930)·info.position(0x9c0) 읽음) · data:&OperationData(24B)(%4. readonly. cache(+0)/context(+8)/blackboard(+0x10)) · goal_data:&GoalData(248B)(%5. readonly. last_base_defense_tick(0xe8) 읽음 + 콜리 전달) · plan:&mut BigPlan(384B)(%6. readonly 없음 → &mut. 본문에서 drop_glue 후 태그 store(ForcePassi) · debug:&mut DebugFrameData(224B)(%7. readonly 없음 → &mut. 본문 직접 사용 0, 콜리 전달만)
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

## `dda220` → `f10f70` TeamPlan::update_objective_after_steal  (24674→30871B · Δ+6197)
- 힌트: TeamPlan::update_objective_after_steal(24.7→30.9KB · f10f70 · 신규플랜 RE 가 phase switch 표 확보(0:eff930 1:efeeb0 2:eff8b0 3:f01e40 4:efea40 5:f00450 6:f00e80 8:f024d0 9:efe020 a:f01380 b:f082d0) · 남은 것 = 각 phase 핸들러 본체와 ObjContest 진입/이탈 조건 · +0xcc7 세팅처)
- exe 정렬: 명령 5330→6315 · 정렬 2311 · 잔여 구조 10 · 분기 10 · 콜리 주의: 12857f0→13f0790 불일치(mig060 는 1643790) ; 12920→ec21b0 미지 ; 12a07d0→ef5a30 미지(+719B) ; 12a07d0→efeeb0 콜리 변경?(J0.00·+2169B) ; 1323a00→12920 미지(-73B) ; 1323a00→ef7350 미지(+334B) ; 1323a00→efafc0 미지(+127B) ; 1323a00→efb0c0 미지(+274B)
- 0.5.8 명세 #108 `team_plan__TeamPlan_update_objective_after_steal` src game-ai\src\plan_legacy\team_plan.rs:910 · one_line: 스틸 추적 분리 후의 팀 목표(MainObjective) 캐스케이드 — 규율 갱신·사망 해제·갱 전처리·오해 수리·귀환 판정·목표별 핸들러 12종을 순서대로 적용해 self.objective/plan 을 갱신
- 0.5.8 params: self:&mut TeamPlan(1064B)(%0 · noalias · readonly 없음 → &mut. objective/objective_disci) · version:usize(%1 · AI 버전. `version > 1`(L920·L982·PT181) 게이트 3곳 + 콜리 인자로 전) · rnd:&mut StdRng(320B)(%2 · noalias·readonly 없음 → &mut. 본문에서 직접 읽지 않고 콜리(should_rec) · player:&PlayerState(2528B)(%3 · readonly. info.team(@0x930)·info.position(@0x9c0) 만 직접 ) · data:&OperationData(24B)(%4 · readonly. cache(@0x0)·context(@0x8)·blackboard(@0x10) 3) · goal_data:&GoalData(248B)(%5 · readonly. epic/serpen 스탠스 틱 6종만 읽음) · plan:&mut BigPlan(384B)(%6 · noalias·readonly 없음 → &mut. 태그 읽기 + drop_glue 후 ForcePa) · debug:&mut DebugFrameData(224B)(%7 · noalias·readonly 없음 → &mut. context.debug 일 때만 infos(@0)
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

## `e59b20` → `d5b980` handle_chat_inner  (9416→7100B · Δ-2316)
- 힌트: handle_chat_inner(9.4→7.1KB −2.3KB · d5b980 · 축소)
- exe 정렬: 명령 1912→1442 · 정렬 942 · 잔여 구조 10 · 분기 10 · 콜리 주의: 12a07d0→381e0b3 미지 ; 31a01a3→381e0b3 미지 ; 31a04c0→d35010 미지(+335B) ; 31a04c0→d37c50 미지(+85B) ; 31a04c0→ec21b0 미지 ; 31a3863→d75ad0 미지(+1088B) ; caefe0→d38180 미지 ; caefe0→ec21b0 미지
- 0.5.8 명세 #104 `chat__LegacyPlanHandler_handle_chat_inner` src game-ai\src\plan_legacy\handler\chat.rs:41 · one_line: 받은 팀 채팅(Chat 57종)을 종류별로 처리해 team_plan.objective/plan/chats 를 갱신하고 오브젝트 오해 상태를 마킹
- 0.5.8 params: self:&mut LegacyPlanHandler(6168B(%0 (noalias, readonly 없음 → &mut) · plan/team_plan/chats/계측 카) · version:usize(%1 · AI 버전. `>1` 게이트 3종: resolve_join_stake 호출(:212), r2_fig) · rnd:&mut StdRng(320B)(%2 (noalias, readonly 없음) · :83 gen_range(0..1000) 1회 직접 사용 ) · player:&PlayerState(2528B)(%3 (readonly)) · data:&OperationData(24B)(%4 (readonly) · cache(+0)/context(+8)/blackboard(+0x10)) · from:Position(i32 0..5)(%5 · 보낸 선수 포지션) · chat:Chat(24B, 값전달 dead_on_return(%6 (readonly) · +0 태그, +1 LineType/StopReason 등, +4 Position) · misunderstood:bool(%7 · :598 에서만 분기(오브젝트 오해 마킹 vs 해제)) · debug:&mut DebugFrameData(224B)(%8 (noalias, readonly 없음) · 본문에서 직접 읽기/쓰기 0건 — 하위 호출(passive)
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
