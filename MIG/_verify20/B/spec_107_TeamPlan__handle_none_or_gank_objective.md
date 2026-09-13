---

### `107` TeamPlan::handle_none_or_gank_objective — 팀 목표가 None/Gank 일 때 다음 팀 목표(넥서스공격/넥서스수비/라인수비/에픽·세르펜 헌트/타워압박/컴백픽/귀환/에픽·세르펜 셋업/에픽버프 압박)를 우선순위 사슬로 하나 골라 self.objective·plan·chats 에 쓴다

| 항목 | 값 |
|---|---|
| id | `objective_handlers__TeamPlan_handle_none_or_gank_objective` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan18objective_handlersNtB4_8TeamPlan29handle_none_or_gank_objective` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830` |
| IR | `m09.ll` 9043~15165행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_none_or_gank_objective` · **in:game_ai** |
| 계층 | 기타 |
| exe | `dcee40` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r11` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut TeamPlan(1064B) | %0. noalias·align8·dereferenceable(1064), readonly 없음 → &mut. objective/chats/obj_spawn/press_tower_start_tick/comeback_* 등 쓰기 | 4 |
| 1 | 2 | version | usize | %1. AI 버전 게이트. 본문 분기 4곳: L876·L923·L1052·L1119/1126 전부 `version > 1`(=v2+) 판정. 그 외는 콜리에 그대로 전달 | 4 |
| 2 | 3 | rnd | &mut StdRng(320B) | %2. align16·readonly 없음 → &mut. 본문 직접 사용 0, 콜리 전달만 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | %3. readonly. info.team(0x930)·info.position(0x9c0) 읽음 | 4 |
| 4 | 5 | data | &OperationData(24B) | %4. readonly. cache(+0)/context(+8)/blackboard(+0x10) | 4 |
| 5 | 6 | goal_data | &GoalData(248B) | %5. readonly. last_base_defense_tick(0xe8) 읽음 + 콜리 전달 | 4 |
| 6 | 7 | plan | &mut BigPlan(384B) | %6. readonly 없음 → &mut. 본문에서 drop_glue 후 태그 store(ForcePassive=2 / ActiveRecall=8) 로 통째 교체 | 4 |
| 7 | 8 | debug | &mut DebugFrameData(224B) | %7. readonly 없음 → &mut. 본문 직접 사용 0, 콜리 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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
    // :912  check_opportunistic_tower_push(version, player, data, &kill_line) 인라인(team_plan.rs:1273~1345) → Option<LineType>
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
    //   else → Some(line)
    if let Some(line) = check_opportunistic_tower_push(..) {
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

**`mem` 메모리 접근 70건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | TeamPlan | 0x220 | last_battle_tick | r | L862 최근 교전 판정 | 4 | OK |  |
| 1 | TeamPlan | 0x50 | obj_spawn.epic_giveup_tick@tag | r | L870·L1098 is_none 판정(i64 load→trunc i1) | 4 | OK |  |
| 2 | TeamPlan | 0x60 | obj_spawn.serpen_giveup_tick | r | L874·L1100 is_none 판정(==0) | 4 | OK |  |
| 3 | TeamPlan | 0x360 | comeback_pick_start_tick | r | L941~942 쿨다운 | 4 | OK |  |
| 4 | TeamPlan | 0x368 | comeback_pick_attempt_count | r | L962 +1 (read-modify-write) | 4 | OK |  |
| 5 | TeamPlan | 0xc0 | chats.buf.cap | r | push 인라인(용량 비교) | 4 | OK |  |
| 6 | TeamPlan | 0xc8 | chats.buf.ptr | r | push 인라인 | 4 | OK |  |
| 7 | TeamPlan | 0xd0 | chats.len | r | push 인라인 | 4 | OK |  |
| 8 | TeamPlan | 0xd8 | comeback_pick_outcomes.buf.cap | r | L963 push | 4 | OK |  |
| 9 | TeamPlan | 0xe0 | comeback_pick_outcomes.buf.ptr | r | L963 push | 4 | OK |  |
| 10 | TeamPlan | 0xe8 | comeback_pick_outcomes.len | r | L963 push | 4 | OK |  |
| 11 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 12 | OperationData | 0x8 | context | r | &GameContext(64B) | 4 | OK |  |
| 13 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] | 4 | OK |  |
| 14 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 15 | GameContext | 0x20 | map | r | &MapDef(28112B) — camp_pos 인자 | 4 | OK |  |
| 16 | GameContext | 0x38 | tutorial | r | TutorialType. morgard_exists/serpen_exists/is_line_phase/valid_lines/player_count 전부 이 값의 switch | 4 | OK |  |
| 17 | GameSetting | 0x12f8 | tick_per_second | r | tps | 4 | OK |  |
| 18 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | L910/L943 is_line_phase | 4 | OK |  |
| 19 | GameSetting | 0x12c0 | height | r | is_top_side/is_bottom_side (height - y) vs x | 4 | OK |  |
| 20 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 | 4 | OK |  |
| 21 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 0x28 tick / 0x40 get_game_mode / 0x130 kill_logs / 0x1f0 get_entity_by_id / 0x208 iter_player (divtable AbstractGame) | 3 | OK |  |
| 22 | AbstractGameWithCache | 0x170 | nexus[team] | r | L1450 nexus[enemy] | 4 | OK |  |
| 23 | AbstractGameWithCache | 0x180 | top_tower[] | r | tower(line,team) 인라인(L1290·L1448·L1531) | 4 | OK |  |
| 24 | AbstractGameWithCache | 0x190 | top_tower2[] | r |  | 4 | OK |  |
| 25 | AbstractGameWithCache | 0x1a0 | mid_tower[아군팀] | r |  | 4 | OK |  |
| 26 | AbstractGameWithCache | 0x1b0 | mid_tower2[아군팀] | r |  | 4 | OK |  |
| 27 | AbstractGameWithCache | 0x1c0 | bottom_tower[아군팀] | r |  | 4 | OK |  |
| 28 | AbstractGameWithCache | 0x1d0 | bottom_tower2[아군팀] | r |  | 4 | OK |  |
| 29 | AbstractGameWithCache | 0x1e0 | player_champion | r | iter_champions(team) = +0x1e0 + team*40, 5칸 ptr; [team][1]=정글 포지션 챔프(0x1e8) | 4 | OK |  |
| 30 | PlayerState | 0x930 | info.team | r |  | 4 | OK |  |
| 31 | PlayerState | 0x9c0 | info.position@tag | r | i32 Position(1=Jungle). L981 as_index / L985·L1023·L1070 ==Jungle | 4 | OK |  |
| 32 | PlayerState | 0x5e8 | info.statistics.gold | r | is_team_behind·comeback·L947/950 골드 합 | 4 | OK |  |
| 33 | PlayerState | 0x9c8 | play_state@tag | r | 0=Die. L928 dead(team) 카운트 | 4 | OK |  |
| 34 | Entity | 0x5c0 | id | r | 고립 적 id (get_entity_by_id 인자) | 4 | OK |  |
| 35 | Entity | 0x628 | stat_cached.hp | r | 최대 HP (hp*100/이 값 = HP%). 0 이면 div_by_zero 패닉 | 4 | OK |  |
| 36 | Entity | 0x660 | x | r |  | 4 | OK |  |
| 37 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 38 | Entity | 0x670 | hp | r | 현재 HP | 4 | OK |  |
| 39 | MobaMode | 0x18 | jungle_runner | r | get_jungle_live_list 인자(480B) | 4 | OK |  |
| 40 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 에픽 생존 여부 | 4 | OK |  |
| 41 | MobaMode | 0x1b0 | jungle_runner.epic.next_respawn_tick | r |  | 4 | OK |  |
| 42 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r |  | 4 | OK |  |
| 43 | MobaMode | 0x1e0 | jungle_runner.serpen.next_respawn_tick | r |  | 4 | OK |  |
| 44 | MobaMode | 0x240 | epic_minion_buff_time[team] | r | remain_epic_time(team) 인라인 = [team] | 4 | OK |  |
| 45 | KillLog | 0x18 | tick | r | check_recent_kill_lane 인라인. 원소 48B stride, rfind(뒤에서부터) | 4 | OK |  |
| 46 | KillLog | 0x20 | killer_team | r |  | 4 | OK |  |
| 47 | KillLog | 0x2c | killed_position | r | i32 Position → LineType 매핑 — 본문에는 원소 끝 포인터 기준 `gep %402, -4`(=원소+0x2c) 로만 나타남(C3 경고 사유) | 4 | OK |  |
| 48 | Blackboard | 0x0 | top_minion_state | r | minion_state(line) 인라인: Top +0 / Mid +0x28 / Bottom +0x50 (BrainMinionParameter 40B) | 4 | OK |  |
| 49 | Blackboard | 0x28 | mid_minion_state | r |  | 4 | OK |  |
| 50 | Blackboard | 0x50 | bottom_minion_state | r |  | 4 | OK |  |
| 51 | BrainMinionParameter | 0x10 | from_mid | r | i64 | 4 | OK |  |
| 52 | BrainMinionParameter | 0x20 | minion_count | r | i32 | 4 | OK |  |
| 53 | GoalData | 0xe8 | last_base_defense_tick | r | L924 | 4 | OK |  |
| 54 | BigPlan | 0x0 | @tag | r | L986·L1024·L1071 ==7(PassiveJungle) 판정 | 4 | OK |  |
| 55 | BigPlan | 0x50 | @PassiveJungle.0.team | r | ==0 → camp_pos/get_jungle_live_list 의 bool 인자 | 4 | OK |  |
| 56 | BigPlan | 0x68 | @PassiveJungle.0.jungle@tag | r | JungleType i8 | 4 | OK |  |
| 57 | TeamPlan | 0x41f | objective | w | Option<MainObjective> 3B. 태그 = tcxdict --enum MainObjective 메모리태그(Direct) | 3 | OK | 4 Nexus(L832) / 2 Defense(L838) / 3 DefenseLine(L844·850·857) / 0 Morgard(L879·L1018) / 1 Serpen(L889·L1065) / 10 PressTower(L900·913·932·1105) / 11 ComebackPick(L960) |
| 58 | TeamPlan | 0x420 | objective.payload+1 | w |  | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | LineType(Nexus/DefenseLine/PressTower/ComebackPick.line) 또는 ObjectPhase(Morgard/Serpen.phase: 3=Hunt(L879·889) / 1=Setup(L1018·1065)) |
| 59 | TeamPlan | 0x421 | objective.payload+2 | w |  | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | Morgard/Serpen.with_battle=1(true) / ComebackPick.ready = (already_on_line > 1) |
| 60 | TeamPlan | 0x41e | v3_press_chat_line | w | L1128, version>1 && 에픽버프 잔여 <= 10s 일 때만 | 4 | OK | -1 (None) |
| 61 | TeamPlan | 0x358 | press_tower_start_tick | w | L901·L914·L933·L1106 (PressTower 채택 시) | 4 | OK | tick |
| 62 | TeamPlan | 0x360 | comeback_pick_start_tick | w | L961 | 4 | OK | tick |
| 63 | TeamPlan | 0x368 | comeback_pick_attempt_count | w | L962 | 4 | OK | +1 |
| 64 | TeamPlan | 0x50 | obj_spawn.epic_giveup_tick@tag | w | L1011 check_epic_giveup 참일 때 | 4 | OK | Some(tick) (tag=1 @0x50, 값 @0x58) |
| 65 | TeamPlan | 0x60 | obj_spawn.serpen_giveup_tick | w | L1058 check_serpen_giveup 참일 때 | 4 | OK | Some(tick) (tag=1 @0x60, 값 @0x68) |
| 66 | TeamPlan | 0xc0 -> chats.ptr[len] | chats.push(Chat) | w | Vec<Chat>(24B 원소) push 인라인 — len==cap 이면 RawVec::grow_one 호출(힙 재할당 지점), len+1 store @0xd0 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | AttackNexus{line,0}=42(L833) / DefenseNexus(0)=43(L840) / DefenseLine{line,0}=14(L845·851·858) / MorgardHunt(0)=38(L883) / SerpenHunt(0)=29(L893) / PressTower{line,0}=45(L902·915·934·1107) / ComebackPick{line,0}=46(L964) / MorgardSetup(0)=34(L1045) / SerpenSetup(0)=25(L1093) |
| 67 | TeamPlan | 0xd8 -> comeback_pick_outcomes.ptr[len] | comeback_pick_outcomes.push((gold_diff:i32, 0u8)) | w | L963. len==cap 이면 grow_one(힙). 원소 8B(i32@+0,u8@+4) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | (my_gold - enemy_gold, 0) |
| 68 | BigPlan | 0x0 | *plan = ForcePassive | w | L839·846·852·859·884·894·903·916·935·965·1013·1042·1060·1089·1108 — 15개 사이트 전부 태그 2. 옛 plan 의 힙(Vec 등)은 drop_glue 가 해제 | 4 | 오귀속(사전은 다른 필드를 준다) | drop_glue(plan) 후 store i64 2 |
| 69 | BigPlan | 0x0 | *plan = ActiveRecall | w | L1000. ActiveRecallPlan 은 0B 페이로드라 태그만 store | 4 | 오귀속(사전은 다른 필드를 준다) | drop_glue(plan) 후 store i64 8 |

**`consts` 상수 53건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1 | 843 | 태그 | LineType::Mid 태그(line_exists/handle_line_defense 인자). 라인 순회 순서 Mid(1)→Bottom(2)→Top(0) | 4 |  |
| 1 | 2 | 849 | 센티널 | LineType::Bottom 태그. 또 BigPlan::ForcePassive 메모리태그(store i64 2, 니치 niche_start=2) · MainObjective::Defense 태그(L838) · ObjectPhase 아님 | 4 |  |
| 2 | 0 | 856 | 태그 | LineType::Top 태그 / MainObjective::Morgard 태그(L879) / Chat 페이로드 usize 0 | 4 |  |
| 3 | 3 | 862 | 임계 | tps*8 이 `shl i64 %tps, 3` 로 접힘 — 최근교전 창(8초). L911 check_recent_kill_lane 의 tps*8 도 같은 shl 3. 또 ObjectPhase::Hunt=3(L879·889)·MainObjective::DefenseLine=3·BigGoal::Serpen=3(L1059·1069) | 4 | 8 |
| 4 | 100 | 866 | 계수 | HP% 산출 hp*100/max_hp (전 필터 공통). 컴백 점수 near_allies*100(L1539) | 4 |  |
| 5 | 39 | 866 | 임계 | HP% > 39 (=40% 이상) 을 생존 아군으로 셈(live_ally). L955 already_on_line 도 동일 | 4 |  |
| 6 | 4 | 832 | 태그 | MainObjective::Nexus 태그. 또 L1391 gold_behind>5999 일 때 required_healthy=4 · L1434 target_position<4(=Support 제외) · L1549 total_visible>=4 | 4 |  |
| 7 | 42 | 833 | 태그 | Chat::AttackNexus 태그 | 4 |  |
| 8 | 14 | 845 | 태그 | Chat::DefenseLine 태그 | 4 |  |
| 9 | 43 | 840 | 태그 | Chat::DefenseNexus 태그 | 4 |  |
| 10 | 38 | 883 | 태그 | Chat::MorgardHunt 태그 | 4 |  |
| 11 | 29 | 893 | 태그 | Chat::SerpenHunt 태그 | 4 |  |
| 12 | 45 | 902 | 태그 | Chat::PressTower 태그 | 4 |  |
| 13 | 46 | 964 | 태그 | Chat::ComebackPick 태그 | 4 |  |
| 14 | 34 | 1045 | 태그 | Chat::MorgardSetup 태그 | 4 |  |
| 15 | 25 | 1093 | 태그 | Chat::SerpenSetup 태그 | 4 |  |
| 16 | 10 | 900 | 태그 | MainObjective::PressTower 태그. 또 L1116 tps*10 (에픽버프 잔여 10초 임계, `mul %tps, 10`) | 4 |  |
| 17 | 11 | 960 | 태그 | MainObjective::ComebackPick 태그 | 4 |  |
| 18 | 8 | 1000 | 센티널 | BigPlan::ActiveRecall 메모리태그(idx6 → 니치 +2) | 4 |  |
| 19 | 7 | 986 | 태그 | BigPlan::PassiveJungle 메모리태그(idx5 → +2). 또 morgard_exists 의 `add i8 t,-7` 오프셋 · L974/978 ed_time=7 (tps*7 스폰 근접 하한) | 4 |  |
| 20 | -6 | 870 | 임계 | morgard_exists=spawn_epic 접힘: (tutorial-7) as u8 < 250(-6) ⟺ tutorial ∈ {None0, Line7, Total8}. L912 안에서는 `add -1; ult 6` 형태(tutorial ∈ 1..=6 이면 false) | 4 |  |
| 21 | 5 | 874 | 임계 | serpen_exists=spawn_serpen: tutorial switch {0,5,7,8} (MidBottom=5 포함). 또 BigGoal::Battle=5(L999) · 5칸 포지션 루프 상한. ⚠`shl i8 %line, 5`(L1448 tower 인라인의 line*32 stride)도 있으나 그건 판정값 아님(stride) | 4 |  |
| 22 | 12 | 970 | 계수 | st_time=12: 스폰 근접 상한 tps*12 (L973·977 `mul %tps, 12`) | 4 |  |
| 23 | 30 | 910 | 계수 | is_line_phase(setting.rs:703~704): tick < first_spawn_tick.saturating_sub(tps*30). 또 L1556 target_position==Top 가산 +30 | 4 |  |
| 24 | 48 | 911 | 미상 | KillLog stride 48B (rfind 포인터 -48) | 4 |  |
| 25 | 20 | 912 | 계수 | check_opportunistic_tower_push 1279/1281: 오브젝트 리스폰까지 <= tps*20 이면 epic_near/serpen_near. 또 L942 컴백픽 쿨다운 tps*20 | 4 |  |
| 26 | 1000 | 912 | 미상 | 1300: ms.from_mid < judgement_penalty*1000 이면 None | 4 |  |
| 27 | -2999 | 912 | 임계 | 1340: 다른 라인 from_mid < -2999 이면서 | 4 |  |
| 28 | -3 | 912 | 임계 | 1340: 다른 라인 minion_count < -3 이면 None (다른 라인 붕괴 중) | 4 |  |
| 29 | 49 | 944 | 임계 | HP% > 49 (=50% 이상) 건강 아군 (1305·1314·1394·1461·1471) [인라인 콜리 줄 912 → 루트 944 · 20차 G] | 4 |  |
| 30 | 180 | 943 | 임계 | is_team_behind 1354: 경과 초 < 180 이면 false | 4 |  |
| 31 | 360 | 943 | 임계 | 1365: 경과 초 <360 → threshold 2000 | 4 |  |
| 32 | 540 | 943 | 임계 | 1365: 경과 초 <540 → 2500, 이상 → 3000 | 4 |  |
| 33 | 2000 | 943 | 산출값 | gold_behind 임계(<6분) | 4 |  |
| 34 | 2500 | 943 | 산출값 | gold_behind 임계(6~9분) | 4 |  |
| 35 | 3000 | 943 | 산출값 | gold_behind 임계(9분+) | 4 |  |
| 36 | 5999 | 944 | 임계 | 1391: gold_behind > 5999 → required_healthy 4 (else 3), required_near 3 (else 2) | 4 |  |
| 37 | -1000 | 944 | 임계 | 1405: ms.from_mid < -1000 && | 4 |  |
| 38 | -1 | 944 | 임계 | 1405: ms.minion_count < -1 이면 그 라인 continue. 또 Option<LineType>::None 표현(-1) | 4 |  |
| 39 | 300000 | 944 | 미상 | 1505: can_near_enemies_range(line_center, range=300000) — MIA 적 탐색 반경 | 4 |  |
| 40 | -20 | 944 | 임계 | 1498: battle_check_with_list 점수 < -20 이면 continue | 4 |  |
| 41 | 250001 | 944 | 임계 | 1516: distance(적, 고립적) < 250001 (=<=250000) 인 비라인 가시 적이 1명이라도 있으면 continue | 4 |  |
| 42 | 14400000001 | 944 | 임계 | 120000^2+1 — 제곱거리 < 이 값 = 거리 <= 120000. 1533 고립적↔적타워 / L990·L1029·L1076 챔프↔정글캠프 | 4 |  |
| 43 | 80 | 944 | 산출값 | 1542: allies_on_line==1 → +80 (else +200) | 4 |  |
| 44 | 200 | 944 | 산출값 | 1542: allies_on_line>=2 → +200 | 4 |  |
| 45 | 150 | 944 | 산출값 | 1549: total_visible_enemies>=4 → +150 / 1556: target Mid·Bottom → +150 | 4 |  |
| 46 | 50 | 944 | 계수 | 1549: total_visible==3 → +50 / 1570: enemy_hp 61..80% → +50 | 4 |  |
| 47 | 10000 | 944 | 계수 | 1566: score += enemy_dist_from_base / 10000 | 4 |  |
| 48 | 61 | 944 | 임계 | 1570: enemy_hp% < 61 → +120. L999 자기 HP% < 61 이면 귀환 | 4 |  |
| 49 | 81 | 944 | 임계 | 1570: enemy_hp% < 81 → +50 | 4 |  |
| 50 | 120 | 944 | 계수 | 1570: enemy_hp% < 61 → +120 | 4 |  |
| 51 | 6 | 958 | 임계 | count < 6 assume(5칸 상한 힌트, 판정 아님). BigGoal 아님 | 4 |  |
| 52 | 1152921504606846976 | 870 | 임계 | 2^60 Vec len 상한 assume(판정 아님) | 4 |  |

**`knobs` 조정점 20건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 라인수비 후보 순서 및 덮어쓰기 | objective_handlers.rs:843~859 | Mid→Bottom→Top | return 없이 세 if 가 연속이라 여러 라인이 동시에 참이면 **마지막(Top)** 이 최종 objective 가 되고 chats 는 라인마다 push 된다. 순서를 바꾸면 우선 라인이 바뀐다 | 4 | 기존 |
| 1 | 최근 교전 창 | objective_handlers.rs:862 | tps*8 | 올리면 교전 후 더 오래 '전과 확대(에픽/세르펜 헌트/타워압박)' 분기를 시도한다 | 4 | 기존 |
| 2 | 전과 확대 인원 조건 | objective_handlers.rs:866~869 | live_ally(HP%>39) > 2 && live_ally >= live_enemy+2 | HP 임계 39 를 올리면 저체력 아군을 인원에서 빼서 확대가 줄고, +2 를 낮추면 더 자주 확대 | 4 | 기존 |
| 3 | 스폰 준비 귀환 창 | objective_handlers.rs:970~978 | st_time=12, ed_time=7 (초) | 오브젝트 리스폰 7~12초 전에만 귀환 시도. 창을 넓히면 귀환이 잦아진다 | 4 | 기존 |
| 4 | 귀환 HP 임계 | objective_handlers.rs:999 | 61 (%) | goal 이 Battle 이 아닐 때 HP%<61 이면 귀환. 올리면 더 자주 귀환 | 4 | 기존 |
| 5 | 정글 캠프 근접 시 귀환/강제패시브 생략 반경 | objective_handlers.rs:990 · 1029 · 1076 | 120000 (제곱 14400000001) | 정글러가 캠프 120000 이내(+캠프 몹 피해 중)면 플랜을 안 바꾼다. 줄이면 더 자주 끊고 목표로 이동 | 4 | 기존 |
| 6 | is_line_phase 종료 여유 | setting.rs:703~704 (인라인 @ :910 · :943 · team_plan.rs:1350) | first_spawn_tick - tps*30 | 첫 에픽 스폰 30초 전부터 타워압박/컴백픽 판정이 열린다. 30 을 키우면 더 일찍 열림 | 4 | 기존 |
| 7 | 최근 킬 창(check_recent_kill_lane) | team_plan.rs:1258 | tps*8 | 8초 이내 아군 킬이 있어야 그 라인으로 타워압박 검토 | 4 | 기존 |
| 8 | 타워압박 오브젝트 리스폰 회피 | team_plan.rs:1279·1281 | tps*20 | 에픽/세르펜 리스폰 20초 이내면 타워압박 안 함 | 4 | 기존 |
| 9 | 타워압박 미니언 조건 | team_plan.rs:1296·1300 | from_mid>=0, minion_count>=1, from_mid >= penalty*1000, minion_count > penalty | macro_judgement_penalty 가 클수록(판단 페널티) 더 유리한 웨이브에서만 압박 | 4 | 기존 |
| 10 | 타워압박 인원 | team_plan.rs:1307·1321·1332 | healthy(HP%>49) >= min(player_count,3), same_side >= 2, near_enemies <= 1 | 숫자를 낮추면 더 공격적으로 압박 | 4 | 기존 |
| 11 | 다른 라인 붕괴 차단 | team_plan.rs:1340 | from_mid < -2999 && minion_count < -3 | 다른 라인이 이 정도로 밀리면 압박 포기 | 4 | 기존 |
| 12 | 반격 압박 조건(v2+) | objective_handlers.rs:924~931 | 기지수비 후 tps*8 이내, dead(enemy) >= dead(mine)+2, 적 타워 존재 | +2 를 낮추면 수비 직후 반격이 잦아진다 | 4 | 기존 |
| 13 | 컴백픽 쿨다운 | objective_handlers.rs:942 | tps*20 | 직전 컴백픽 후 20초 재시도 금지 | 4 | 기존 |
| 14 | is_team_behind 임계 | team_plan.rs:1354·1365 | 180초 이후, gold_behind >= 2000(<6분)/2500(<9분)/3000 | 낮추면 덜 뒤져도 컴백픽 시도 | 4 | 기존 |
| 15 | 컴백픽 인원 | team_plan.rs:1391·1395·1482 | required_healthy=min(pc, gold_behind>5999?4:3), required_near=gold_behind>5999?3:2, allies_on_line>=1 | 많이 뒤질수록 더 많은 인원을 요구 | 4 | 기존 |
| 16 | 컴백픽 안전 조건 | team_plan.rs:1498·1506·1519·1533 | battle_score >= -20, MIA(300000 반경) 0, 고립적 250000 이내 비라인 가시적 0, 고립적↔적타워 > 120000 | 완화하면 위험한 픽도 시도 | 4 | 기존 |
| 17 | 컴백픽 점수표 | team_plan.rs:1539~1570 | near_allies*100 + (on_line==1?80:200) + (visible>=4?150: ==3?50:0) + pos(Top30/Jg100/Mid,Bot150) + dist_from_enemy_base/10000 + (hp<61?120: hp<81?50:0) | 가중치 조정으로 어느 라인의 고립적을 고를지 바뀜 | 4 | 기존 |
| 18 | ComebackPick.ready | objective_handlers.rs:955~959 | already_on_line(HP%>39 && is_near_line) > 1 | 2명 이상 이미 라인에 있으면 즉시 시작 | 4 | 기존 |
| 19 | 에픽 버프 창 | objective_handlers.rs:1116 | epic_minion_buff_time[team] > tps*10 | 버프 잔여 10초 초과일 때만 버프 창 압박(v2+: v3_epicops_buff_window / v1: 인원 우세 시 handle_press_epic) | 4 | 기존 |

<details><summary>`callees` 피호출자 77건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | battle_check_with_list | game_ai::battle_check_with_list | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &bumpalo::collections::vec::Vec< usize>, &bumpalo::collections::vec::Vec< usize>, &mut game_core::DebugFrameData) -> i32 | game-ai\src\fight_check.rs:425 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | buy_item | game_ai::buy_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> | game-ai\src\lib.rs:1477 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | calculate_nexus_defense_count | game_ai::plan_legacy::handler::calculate_nexus_defense_count | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> usize | game-ai\src\plan_legacy\handler.rs:2328 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_near_enemies_range | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-ai\src\plan_legacy\team_plan.rs:483 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | check_comeback_pick_opportunity | game_ai::plan_legacy::team_plan::check_comeback_pick_opportunity | in:game_ai::plan_legacy::team_plan | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan.rs:1377 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | check_epic_giveup | game_ai::plan_legacy::old::check_epic_giveup | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\epic.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | check_epic_setup | game_ai::plan_legacy::old::check_epic_setup | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\epic.rs:16 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | check_opportunistic_tower_push | game_ai::plan_legacy::team_plan::check_opportunistic_tower_push | in:game_ai::plan_legacy::team_plan | fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> bool | game-ai\src\plan_legacy\team_plan.rs:1270 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | check_press_tower_opportunity | game_ai::plan_legacy::team_plan::check_press_tower_opportunity | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<game_core::LineType>, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan.rs:1129 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | check_serpen_giveup | game_ai::plan_legacy::old::check_serpen_giveup | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\serpen.rs:234 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | check_serpen_setup | game_ai::plan_legacy::old::check_serpen_setup | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\serpen.rs:68 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | find | game_ai::MinionWaveSnapshot::find | pub | fn(&game_ai::MinionWaveSnapshot, usize) -> std::option::Option<&game_ai::MinionHpTrajectory> | game-ai\src\utils.rs:84 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 18 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 19 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 20 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 21 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 22 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 23 | get_jungle_live_list | game_core::JungleRunner::get_jungle_live_list | pub | fn(&game_core::JungleRunner, game_core::JungleType, bool) -> std::vec::Vec<usize, std::alloc::Global> | game-core\src\simulation\entity\jungle.rs:743 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | goal | game_ai::plan_legacy::types::BigPlan::goal | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> game_core::BigGoal | game-ai\src\plan_legacy\types.rs:132 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | gold | game_core::EntityInfo::gold | pub | fn(&Self/#0) -> usize | game-core\src\setting.rs:1155 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 27 | gold | <game_core::NexusInfo as game_core::EntityInfo>::gold | pub | fn(&game_core::NexusInfo) -> usize | game-core\src\setting\entity\nexus.rs:28 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 28 | gold | <game_core::TowerInfo as game_core::EntityInfo>::gold | pub | fn(&game_core::TowerInfo) -> usize | game-core\src\setting\entity\tower.rs:44 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 29 | handle_line_defense | game_ai::plan_legacy::old::handle_line_defense | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:543 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | handle_nexus_attack | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_attack | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1237 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | handle_press_epic | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\epic.rs:502 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | i_am_chosen_defender | game_ai::plan_legacy::old::i_am_chosen_defender | pub | fn(&game_core::PlayerState, &game_core::OperationData, usize, &[usize]) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:297 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 34 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 35 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 36 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 37 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 38 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | is_team_behind | game_ai::plan_legacy::team_plan::is_team_behind | in:game_ai::plan_legacy::team_plan | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\team_plan.rs:1349 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 42 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 43 | iter_player | game_core::AbstractGame::iter_player | pub | fn(&Self/#0) -> game_core::PlayerIter | game-core\src\simulation.rs:181 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 44 | iter_player | <game_core::Game as game_core::AbstractGame>::iter_player | pub | fn(&game_core::Game) -> game_core::PlayerIter | game-core\src\simulation\game.rs:3756 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 45 | iter_player | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_player | pub | fn(&game_core::SingleLaneGame) -> game_core::PlayerIter | game-core\src\simulation\game.rs:4015 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 46 | jungle_runner | game_core::MobaMode::jungle_runner | pub | fn(&game_core::MobaMode) -> &game_core::JungleRunner | game-core\src\simulation\game.rs:213 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 47 | jungle_runner | game_core::AbstractGame::jungle_runner | pub | fn(&Self/#0) -> &game_core::JungleRunner | game-core\src\simulation.rs:119 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 48 | kill_logs | game_core::AbstractGame::kill_logs | pub | fn(&Self/#0) -> &std::vec::Vec<game_core::KillLog, std::alloc::Global> | game-core\src\simulation.rs:141 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 49 | kill_logs | <game_core::Game as game_core::AbstractGame>::kill_logs | pub | fn(&game_core::Game) -> &std::vec::Vec<game_core::KillLog, std::alloc::Global> | game-core\src\simulation\game.rs:3768 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 50 | kill_logs | <game_core::SingleLaneGame as game_core::AbstractGame>::kill_logs | pub | fn(&game_core::SingleLaneGame) -> &std::vec::Vec<game_core::KillLog, std::alloc::Global> | game-core\src\simulation\game.rs:4027 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 51 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 52 | macro_judgement_penalty | game_ai::plan_legacy::team_plan::macro_judgement_penalty | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:15 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 53 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 54 | morgard_exists | game_ai::plan_legacy::rule_scope::morgard_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 55 | need_defense_nexus | game_ai::plan_legacy::old::need_defense_nexus | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:481 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 57 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 58 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 59 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 60 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 61 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 62 | should_delay_morgard_for_wave_priority | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_morgard_for_wave_priority | in:game_ai::plan_legacy::team_plan | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool | game-ai\src\plan_legacy\team_plan.rs:570 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 63 | should_delay_serpen_for_wave_priority | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_serpen_for_wave_priority | in:game_ai::plan_legacy::team_plan | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool | game-ai\src\plan_legacy\team_plan.rs:574 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 64 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 65 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 66 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 67 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 68 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 69 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 70 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 71 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 72 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 73 | upgrade_item | game_ai::upgrade_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1603 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 74 | v3_epicops_buff_window | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool | game-ai\src\plan_legacy\old\epic.rs:634 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 75 | v3_epicops_defer_serpen | game_ai::plan_legacy::team_plan::objective_helpers::v3_epicops_defer_serpen | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:275 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 76 | valid_lines | game_ai::plan_legacy::rule_scope::valid_lines | pub | fn(&game_core::GameContext) -> &[game_core::LineType] | game-ai\src\plan_legacy\rule_scope.rs:13 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 23개**: `comeback_pick_start_tick`, `dead`, `defer_serpen`, `dist_sq`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `enemy_gold`, `first_spawn_tick`, `from_mid`, `grow_one`, `height`, `jungle`, `killed_position`, `killer_team`, `last_base_defense_tick`, `last_battle_tick`, `minion_count`, `next_respawn_tick`, `plan`, `poison`, `reserve_internal_or_panic`, `s5_0`, `sat_sub`, `setting`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:30757) · **형제 55개** (TeamPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic | pub | game-ai\src\plan_legacy\old\epic.rs:502 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 1 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window | in:game_ai | game-ai\src\plan_legacy\old\epic.rs:634 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool |
| 2 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change | pub | game-ai\src\plan_legacy\old\epic.rs:684 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType) |
| 3 | <game_ai::plan_legacy::team_plan::TeamPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> game_ai::plan_legacy::team_plan::TeamPlan |
| 4 | <game_ai::plan_legacy::team_plan::TeamPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 5 | <game_ai::plan_legacy::team_plan::TeamPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn() -> game_ai::plan_legacy::team_plan::TeamPlan |
| 6 | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | game-ai\src\plan_legacy\team_plan.rs:196 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) |
| 7 | game_ai::plan_legacy::team_plan::TeamPlan::sanitize_rule_scope | pub | game-ai\src\plan_legacy\team_plan.rs:200 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::GameContext) |
| 8 | game_ai::plan_legacy::team_plan::TeamPlan::objective_target | pub | game-ai\src\plan_legacy\team_plan.rs:230 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> std::option::Option<game_core::JungleType> |
| 9 | game_ai::plan_legacy::team_plan::TeamPlan::take_active | pub | game-ai\src\plan_legacy\team_plan.rs:243 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 10 | game_ai::plan_legacy::team_plan::TeamPlan::take_hunt_commit | pub | game-ai\src\plan_legacy\team_plan.rs:248 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 11 | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | game-ai\src\plan_legacy\team_plan.rs:257 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 12 | game_ai::plan_legacy::team_plan::TeamPlan::mark_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:265 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType, usize) |
| 13 | game_ai::plan_legacy::team_plan::TeamPlan::clear_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:277 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan) |
| 14 | game_ai::plan_legacy::team_plan::TeamPlan::init | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:281 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 15 | game_ai::plan_legacy::team_plan::TeamPlan::update | pub | game-ai\src\plan_legacy\team_plan.rs:294 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies | pub | game-ai\src\plan_legacy\team_plan.rs:444 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 17 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | game-ai\src\plan_legacy\team_plan.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 18 | game_ai::plan_legacy::team_plan::TeamPlan::update_wave_priority_clear_line | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:540 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 19 | game_ai::plan_legacy::team_plan::TeamPlan::should_player_clear_wave_priority_line | pub | game-ai\src\plan_legacy\team_plan.rs:561 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> |
| 20 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_morgard_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:570 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 21 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_serpen_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:574 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 22 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_object_setup_for_wave_priority | pub | game-ai\src\plan_legacy\team_plan.rs:578 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 23 | game_ai::plan_legacy::team_plan::TeamPlan::should_keep_object_for_contested_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:586 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool |
| 24 | game_ai::plan_legacy::team_plan::TeamPlan::repair_misunderstood_objective | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:603 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 25 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective | pub | game-ai\src\plan_legacy\team_plan.rs:722 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 26 | game_ai::plan_legacy::team_plan::TeamPlan::update_steal | pub | game-ai\src\plan_legacy\team_plan.rs:732 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) |
| 27 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective_after_steal | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:910 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool |
| 29 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool |
| 30 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_release_to_passive | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:177 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 31 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_relevant_lanes_ready | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:199 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 32 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> |
| 33 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_wait_pos | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:316 | False | fn(game_core::JungleType, usize, &game_core::MapDef) -> (u64, u64) |
| 34 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_entity | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:324 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData, game_core::JungleType) -> std::option::Option<&game_core::Entity> |
| 35 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::update_v27_objective_discipline | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_active_objective_discipline | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectiveDisciplineState> |
| 37 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_blocks_battle | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:497 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData) -> bool |
| 38 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> |
| 39 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_repair_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:7 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) |
| 40 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:16 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 41 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:29 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 42 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_line_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:46 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 43 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_dive_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:88 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 44 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_tower_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:141 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 45 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:253 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 46 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_split_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 47 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_comeback_pick_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:393 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType, bool) -> bool |
| 48 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_serpen_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:558 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 49 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_morgard_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:685 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 50 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_none_or_gank_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 51 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_gank_preprocess | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1133 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, game_core::LineType) -> bool |
| 52 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_defense | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1227 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 53 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_attack | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1237 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> |
| 54 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_sub_objective | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1258 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |

**`open` 11건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L910·L943·team_plan.rs:1350 의 is_line_phase 는 완전 인라인이라 소스 표기(`if !ctx.is_line_phase(tick)` 인지, is_line_phase 내부가 `(spawn_epic\|\|spawn_serpen) && setting.is_line_phase` 인지)는 표기 불가. 통과 조건 자체(tutorial∈{0,5,7,8} && tick >= first_spawn_tick.sat_sub(tps*30))는 IR 로 확정. `_gcbc`/`_gaibc` 에 define 없음(grep 0건) | 4 |  |
| 1 | 표기 불가 | player_count(ctx)(runner.rs:294~301) 는 인라인돼 원 반환값 표기 불가 — 관측은 min() 접힘 후 값뿐(우리 사이트 3/2/1, check_press_tower_opportunity 1142 사이트 4/2/1, 1198 사이트 3/2/1). 추정: player_count = {0,7,8}→5 / 5→3 / {1,3}→2 / {2,4,6}→1 이고 소비처가 .min(3)/.min(4)/.min(4\|3) — 세 사이트와 모순 없음. 확정하려면 runner.rs:295~301 MIR 또는 오라클(GameContext 는 pub) 필요(미탐색) | 2 |  |
| 2 | 표기 불가 | L876 v3_epicops_defer_serpen 의 부호: dbg 이름은 serpen_available=%337 이지만 분기는 %337==true → 세르펜 건너뜀. IR 분기 방향을 정본으로 기록(소스가 `!defer` 인지 `serpen_available = !defer` 인지는 표기 불가) | 3 |  |
| 3 | 미탐색 | check_press_tower_opportunity(version, poison, player, data, poison, i8 -1, poison) — poison 3개(2번째=rnd?, 5번째=goal_data?, 7번째=debug?)는 콜리가 그 인자를 안 읽는다는 뜻. 실제 파라미터 이름은 콜리 define(m09.ll 53000대) 미열람 | 4 |  |
| 4 | 미탐색 | handle_nexus_attack 의 self 인자가 poison — self 미사용. 반환 i8 = Option<LineType>(-1=None) 은 icmp -1 과 +0x420 store 로 확정, 콜리 본문 미열람 | 4 |  |
| 5 | 미탐색 | buy_item 반환 {i64,i64} 의 태그 1 = Some 으로 읽음(Option<...> 관례). upgrade_item 은 sret 24B 첫 8B 를 trunc i1 → bool 로 읽음(반환 타입 미확인). 둘 다 콜리 미열람 | 4 |  |
| 6 | 미탐색 | AbstractGame vtable 0x40(get_game_mode) 반환 {i64,ptr} 태그 0 이 Moba 인지는 unwrap_failed 로의 분기(비0→패닉)로 추정. as_moba 이름은 dbg scope 에서 | 4 |  |
| 7 | 미탐색 | check_opportunistic_tower_push 의 세 Iterator::count 조각(m09.ll 67786·67886·68915)은 그 함수 소유라 aux 에 넣지 않고 logic 에 술어만 옮겨 적음(각 상수 49·is_near_line 등은 본문 사이트에도 있어 C1 통과) | 4 |  |
| 8 | 미탐색 | KillLog.assist(Vec<Position>) 등 kill_logs 의 나머지 필드·rfind 가 잘못된 정렬을 가정하는지 여부 미확인(뒤에서 첫 매치 = 가장 최근 로그라는 전제) | 4 |  |
| 9 | 미탐색 | L1119 v3_epicops_buff_window 의 bool 반환은 버려진다(%53 phi 죽음). 왜 bool 인지·호출자가 쓰는지 미확인 | 4 |  |
| 10 | 미탐색 | battle_check_with_list 의 인자 순서 (version, rnd, data, player, &allies(Vec<usize>), &enemies(Vec<usize>), debug) — 4·5번째가 (data, player) 순인 것은 dereferenceable(24)/(2528) 로 확정, 슬라이스 의미(포지션 인덱스)는 push 값(pos idx)으로 추정 | 5 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L862 의 소스 변수명 recent_battle 의 정확한 식 — dbg 는 `recent_battle = (lbt==0)` 조각만 남김. 분기 방향으로 '866~903 블록은 lbt!=0 && tick <= lbt+tps*8 일 때 실행' 은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

