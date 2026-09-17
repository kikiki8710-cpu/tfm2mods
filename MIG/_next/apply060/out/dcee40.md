# dcee40→f024d0 TeamPlan::handle_none_or_gank_objective

## logic_060
```
// team_plan/objective_handlers.rs:830 · 0.6.0 f024d0(23KB) · sig 동일: fn(&mut TeamPlan, version, rnd, player, data, goal_data, plan:&mut BigPlan, debug) -> bool
// ★0.6.0 TeamPlan 오프셋: objective +0xcd5(태그 · 0xff None) · +0xcd6 phase/line · +0xcd7 with_battle/ready · chats +0x2e8/+0x2f0/+0x2f8(0x18B · tag@0 · byte1 · u64@8) · last_battle_tick +0x510 · press_tower_start +0x648 · comeback_pick_start +0x650 · attempt_count +0x658 · comeback_pick_outcomes +0x300 · obj_spawn epic_giveup +0x70/+0x78 · serpen_giveup +0x80/+0x88 · v3_press_chat_line +0xcd4 · armed: v4_obj +0xcc5 · v5_obj +0xcc6 · v6_obj +0xcc7(v≥2 상시 1) · v6_sync_follower +0xcd2 · locks +0xc0/+0xf0 · 씬 +0x3c8/+0x3e8 · v6_window_intent +0xc9e[2] · v6_obj_deadline +0x778 · arrive_at +0x788 · quorum_k +0x798 · assemble_arrived +0xca0 · departed +0xca2 · commits +0x730[3] · cut_no +0x748(+0x750/+0x758/+0x768/+0x770) · commit_at +0xc40 · last_commit +0xc50 · v6_audit[4] +0x868
//   BigPlan(plan+0): ForcePassive 2 · ActiveRecall 8 · PassiveJungle 7(jungle +0x83 · team +0x60 · 0.6.0 PassiveJunglePlan 레이아웃) · Battle 니치 0/1 · goal() = ec21b0 sret(Line 0 · Epic 2 · Serpen 3 · Battle 5)
//   PlayerState team +0xa00 · pos +0xa90 · gold +0xa68 · play_state 태그 +0xa98(0 = Die/리스폰 대기) · iter_player stride 0xab0 · Entity hp +0x670 · max +0x628 · xy +0x660/+0x668 · id +0x5c0
//   Blackboard stride 0x5c8 · minion_state top +0x0 · mid +0x28 · bottom +0x50 {from_mid +0x10 (i64) · minion_count +0x20 (i32)} · call-vec ptr +0x250 / len +0x258 (0x50 stride · kind +0x46 · pos +0x20) · big_goal +0x2f8+p*0x20 {update_tick@0 · tag@+8}
//   MobaMode epic live len +0x1a8 · respawn +0x1b0 · serpen +0x1d8/+0x1e0 · epic_minion_buff_time +0x240+team*8 · jungle_runner +0x18 · setting tps +0x12f8 · first_spawn +0x8a8 · height +0x12c0 · ctx.tutorial +0x38 · game vt +0x28 tick · +0x40 get_game_mode · +0x130 kill_logs · +0x1f0 get_entity_by_id · +0x208 iter_player
//   Chat 태그: DefenseLine 0xe · SerpenHunt 0x1d · SerpenSetup 0x19 · MorgardSetup 0x22 · MorgardHunt 0x26 · AttackNexus 0x2a · DefenseNexus 0x2b · PressTower 0x2d · ComebackPick 0x2e
// 콜리 v3: handle_nexus_attack efdbf0 · need_defense_nexus e74b40 · calculate_nexus_defense_count d75ad0 · i_am_chosen_defender e759a0(player,data,n,&[]) · handle_line_defense e753e0 · is_near_line 1419903f0 · is_near_mid_line 14137cbc0 · check_press_tower_opportunity f1fce0(…,Option<line>) · buy_item f27ea0 · upgrade_item f27140 · get_jungle_live_list · camp_pos 157b720/16a7af0 · BigPlan::goal ec21b0 · v3_epicops_defer_serpen f87600 · v3_epicops_buff_window ef4690 · handle_press_epic ef3da0 · ★신규 efdf40(PressTower 챗 · v3 억제) · eda830(undying 잔여) · V6: ef6ac0 enemy_committed · ef5a30 hunt_feasible · efbb70 (ally_mass, enemy_mass) · efb980 도착틱 정렬 Vec · efb0c0 t_full · f1e3c0 (dead_allies, near_enemies(240000), obj_hp%) · efc4f0 V6COMMIT take · ef56d0 commit 감사 · f0ff30/f10550 씬 열기(serpen/morgard) · f10a20/f10b40 lock 허용 · 컴백픽 내부 ed8f40 · f10c60 · 1417d3bc0 · v1 전용(사장): fb34e0 · f18ad0 · fb49b0 · 100f210 · f18a80 · 1010060 · ef3c30 · 16e49e0

// 공통 준비: ctx=data.context(+8), setting=ctx.setting(+8), tps=setting+0x12f8, game=data.cache.game, tutorial=ctx+0x38, team=player.team(+0xa00), enemy=1-team.
// 인라인 술어: morgard_exists(ctx) ⟺ tutorial∈{0,7,8} / serpen_exists ⟺ tutorial∈{0,5,7,8} / as_moba()=get_game_mode(vt+0x40) 태그0 요구(아니면 unwrap_failed).
// HP%(e) = e.hp*100 / e.stat_cached.hp  (max_hp==0 이면 div_by_zero 패닉). armed = self.v6_obj_armed(+0xcc7) (★version=3 이라 항상 1).

// objective_handlers.rs:831~834  ① 넥서스 공격
if let Some(line) = handle_nexus_attack(efdbf0)(version, rnd, player, data, debug) {        // i8, −1=None
  self.objective = Some(Nexus(4){line});
  // ★0.6.0 v3 챗 억제: if version>=3 && bb[team].call_vec.iter().any(|e| e.kind(+0x46)==4 && e.pos(+0x20) != my_position) { return }   (챗 생략)
  self.chats.push(AttackNexus(0x2a){line, 0});  return;
}
// :837~841  ② 넥서스 수비 (handle_nexus_defense 인라인)
if need_defense_nexus(e74b40)(version, rnd, player, data, debug) {
  let need_count = calculate_nexus_defense_count(d75ad0)(version, rnd, player, data, debug);
  if i_am_chosen_defender(e759a0)(player, data, need_count, &[]) {
    self.objective = Some(Defense(2));  *plan = ForcePassive;  (★0.6.0 v3: call_vec kind==3 타인 존재 시 챗 생략)  self.chats.push(DefenseNexus(0x2b){0});  return;
  }
}
// :843~859  ③ 라인 수비 — Mid→Bottom→Top 순으로 세 if 연속(return 없음 · 뒤 라인이 앞 결과를 덮어씀)
for line in [Mid(1), Bottom(2), Top(0)] {   // line_exists 마스크 Mid 0x1b1 · Bottom 0x1ab · Top 0x185 (tutorial 비트)
  if line_exists(ctx, line) && handle_line_defense(e753e0)(version, rnd, player, data, line, debug) {
    self.objective = Some(DefenseLine(3){line});  self.chats.push(DefenseLine(0xe){line, 0});  *plan = ForcePassive;
  }
}
// :862  최근 교전 게이트
recent_battle := self.last_battle_tick(+0x510) != 0 && game.tick() <= last_battle_tick + tps*8
if recent_battle {
  // :866~869
  live_ally  = iter_champions(team).filter(|e| HP%(e) > 39).count();
  live_enemy = iter_champions(enemy).count();
  if live_ally > 2 && live_ally >= live_enemy + 2 {
    epic_available   = morgard_exists(ctx) && moba.epic.live_list.len(+0x1a8) != 0 && self.obj_spawn.epic_giveup_tick(+0x70).is_none()
    serpen_available = serpen_exists(ctx) && serpen.live_list.len(+0x1d8) != 0 && serpen_giveup_tick(+0x80).is_none() && (version <= 1 || !v3_epicops_defer_serpen(f87600)(version, player, data, goal_data, self))
    // ★0.6.0 armed 분기 (★v2 사장: `!armed` 가지 = v1 경로 → Morgard{Hunt,1}/Serpen{Hunt,1} + chat 0x26/0x1d + FP, 또는 v5/v4_obj_armed 이면 f10b40/f10550(3) · f10a20/f0ff30(3) 씬 열기)
    if armed {
      if epic_available || serpen_available { goto L910 }        // 압박탑 생략 · 목표 선점은 아래 V6 commit 이 담당
      else if let Some(press_line) = check_press_tower_opportunity(f1fce0)(version, rnd, player, data, self, None(−1), debug) {
        self.objective = Some(PressTower(10){press_line}); self.press_tower_start_tick(+0x648) = tick; efdf40(self, version, team, pos, bb, press_line) /*PressTower 챗 0x2d · v3 억제*/; *plan = ForcePassive; return;
      }
    }
  }
}
// :910  라인전 종료 게이트 (is_line_phase 인라인)  — ★미독 구역 시작(f02ef6..f04f8f · Ghidra 실패 · 콜 시퀀스만 확인: 1419903f0×6 · 14137cbc0×2 · ed8f40 · f10c60 · 1417d3bc0 · f27ea0 · f27140 · get_jungle_live_list · 157b720). 아래는 0.5.8 본문 그대로(상수·조건 미검증 · 오프셋만 0.6.0)
//   통과 조건: tutorial ∈ {0,5,7,8} && tick >= setting.first_spawn_tick(+0x8a8).saturating_sub(tps*30)
if <위 조건> {
  // :911  check_recent_kill_lane 인라인 — JT f02ef4 복원: killed_position Top→f02ff9 · Jungle→None · Mid→f02fad · Bottom/Support→f02ef6
  kill_line = game.kill_logs()(vt+0x130).iter().rev().find(|log| log.killer_team(+0x20)==team && tick.saturating_sub(log.tick(+0x18)) <= tps*8)
              .and_then(|log| match log.killed_position(+0x2c) { Top→Some(Top0), Mid→Some(Mid1), Bottom|Support→Some(Bottom2), Jungle→None })
              .filter(|l| line_exists(ctx, l));
  if let Some(kill_line) = kill_line {
    // :912  check_opportunistic_tower_push(version, player, data, kill_line) 인라인 → bool   // ★미독(0.5.8 그대로)
    //   1273 if !line_exists(ctx,line) → false
    //   1278 epic_near   = morgard_exists && epic.live_list.len==0 && (epic.next_respawn_tick(+0x1b0) − tick).sat <= tps*20
    //   1280 serpen_near = serpen_exists && serpen.live_list.len==0 && (serpen.next_respawn_tick(+0x1e0) − tick).sat <= tps*20
    //   1283 epic_alive = morgard_exists && epic.live_list.len!=0 ; 1284 serpen_alive = serpen_exists && serpen.live_list.len!=0
    //   1285 if epic_near||serpen_near||epic_alive||serpen_alive → false
    //   1290 (t1,t2)=cache.tower(line, enemy); if t1.is_none() && t2.is_none() → false
    //   1295 ms = bb[team].minion_state(line); 1296 if ms.from_mid(+0x10) < 0 || ms.minion_count(+0x20) < 1 → false
    //   1300 jp = macro_judgement_penalty(version, player); if ms.from_mid < jp*1000 || ms.minion_count <= jp → false
    //   1305 healthy_allies = iter_champions(team).filter(HP%>49).count()
    //   1307 required_healthy = min(player_count(ctx),3) → tutorial {0,5,7,8}→3 / {1,3}→2 / {2,4,6}→1 ; if healthy_allies < required_healthy → false
    //   1312~1321 same_side_allies = (0..5).filter(|p| champ[team][p].is_some_and(|c| HP%>49 && match line {Top→(height−c.y) >= c.x, Mid→is_near_mid_line(ctx,c.x,c.y), Bottom→(height−c.y) <= c.x})).count(); if < 2 → false
    //   1327~1332 near_enemies = iter_champions(enemy).filter(|c| bb[team].is_recent_visible(game,player,c) && is_near_line(ctx,c.x,c.y,line)).count(); if > 1 → false
    //   1337~1340 for check_line in valid_lines(ctx) { if check_line==line continue; cms=bb[team].minion_state(check_line); if cms.from_mid < −2999 && cms.minion_count < −3 → false }
    //   else → true
    if check_opportunistic_tower_push(..) { let line = kill_line;
      self.objective = Some(PressTower(line)); self.press_tower_start_tick = tick; efdf40(…PressTower 챗…); *plan = ForcePassive; return;   // :913~916
    }
    else if version > 1 {                                                                    // :923~935 기지 수비 직후 반격 압박
      if tick.saturating_sub(goal_data.last_base_defense_tick(0.5.8 +0xe8 · 0.6.0 미확인)) <= tps*8 {
        dead(t) := game.iter_player()(vt+0x208).filter(|p| p.team==t && p.play_state 태그(+0xa98)==Die(0)).count();
        if dead(enemy) >= dead(team) + 2 {
          if cache.tower(kill_line, enemy).is_some() {
            self.objective = Some(PressTower(kill_line)); self.press_tower_start_tick = tick; efdf40(…); *plan = ForcePassive; return;
          }
        }
      }
    }
  }
}
// :941~943  컴백 픽 게이트   // ★미독(0.5.8 그대로 · 콜 시퀀스 일치)
pick_cooldown_ok = self.comeback_pick_start_tick(+0x650)==0 || tick.saturating_sub(comeback_pick_start_tick) > tps*20;
// is_team_behind 인라인: tutorial∈{0,5,7,8} && tick >= first_spawn.sat_sub(tps*30) 아니면 false ; elapsed_sec = tick/max(tps,1); if < 180 → false ; my_gold = Σ iter_player().filter(team==mine).gold(+0xa68) ; enemy_gold = Σ(team!=mine) ; gold_behind = (enemy−my) as i32 ; threshold = elapsed<360 ? 2000 : elapsed<540 ? 2500 : 3000 ; gold_behind >= threshold
if pick_cooldown_ok && is_team_behind(..) {
  // :944 check_comeback_pick_opportunity(version, rnd, player, data, self, debug) 인라인 → Option<LineType>   // ★미독(0.5.8 1382~1580 그대로: required_healthy=min(player_count, gold_behind>5999 ? 4 : 3) · required_near = gold_behind>5999 ? 3 : 2 · 라인별 ms.from_mid<−1000&&count<−1 skip · 가시 적 정확히 1(isolated · Support 제외) · 넥서스보다 타워 뒤면 skip · allies_on_line/near_allies · battle_check_with_list(ed8f40 추정) < −20 skip · can_near_enemies_range(f10c60)(…, line_center, 300000) 비어야 · 근처 가시 적 250001 미만 없음 · 타워 120000 내 skip · score = near_allies*100 + (allies_on_line==1 ? 80 : 200) + (total_visible>=4 ? 150 : ==3 ? 50 : 0) + {Top 30 · Jungle 100 · Mid|Bottom 150} + distance(isolated, enemy_base)/10000 + (HP%<61 ? 120 : <81 ? 50 : 0) → 최고점)
  if let Some(pick_line) = check_comeback_pick_opportunity(..) {
    // :945~958 my_gold/enemy_gold(i32) ; gold_diff = my − enemy ; already_on_line = iter_champions(team).filter(|c| HP%>39 && is_near_line(ctx,c.x,c.y,pick_line)).count() ; start_ready = already_on_line > 1
    self.objective = Some(ComebackPick(11){line:pick_line, ready:start_ready}); self.comeback_pick_start_tick(+0x650) = tick; self.comeback_pick_attempt_count(+0x658) += 1;
    self.comeback_pick_outcomes(+0x300).push((gold_diff, 0u8)); chats.push(ComebackPick(0x2e){pick_line,0}); *plan = ForcePassive; return;
  }
}
// :970~980  스폰 준비 귀환 (st_time=12, ed_time=7)   — 여기부터 디컴 건전(f04b67 f27ea0 · f04bb5 f27140 · f04eef eda830)
near_serpen_spawn = serpen_exists && serpen.live_list.len==0 && { d=(serpen.next_respawn_tick(+0x1e0) − tick).sat ; d <= tps*12 && d >= tps*7 }
near_epic_spawn   = morgard_exists && epic.live_list.len==0   && { d=(epic.next_respawn_tick(+0x1b0) − tick).sat ; d <= tps*12 && d >= tps*7 }
if near_serpen_spawn || near_epic_spawn {
  champ = cache.player_champion[team][player.position].unwrap();
  hp = HP%(champ);
  can_buy_or_upgrade = buy_item(f27ea0)(version,rnd,player,game,ctx).is_some() || upgrade_item(f27140)(…).is_some();   // buy 가 Some 이면 upgrade 미호출
  skip_for_camp = false;
  if player.position == Jungle(1) {
    if let BigPlan::PassiveJungle(jungle) = plan (tag 7) {
      camp_live = moba.jungle_runner(+0x18).get_jungle_live_list(jungle.jungle(plan+0x83), jungle.team(plan+0x60)==0);   // ★0.6.0 필드 오프셋
      if !camp_live.is_empty() { camp_pos = ctx.map.camp_pos(jungle.jungle, jungle.team==0); skip_for_camp = dist_sq(champ, camp_pos) < 14400000001; }
      drop(camp_live);
    }
  }
  // ★0.6.0 v3: if version>=3 && champ.is_some() && eda830(champ) > tps { 귀환 생략(아래 if 건너뜀) }   (f04eef)
  if !skip_for_camp && !(v3 undying 잔여 > tps) {
    goal = plan.goal()(ec21b0);
    if (goal is Battle(5) && can_buy_or_upgrade) || (goal !is Battle && (hp < 61 || can_buy_or_upgrade)) { drop(plan); *plan = ActiveRecall(8); return; }
  }
}
// ★0.6.0 신규 V6 commit 블록 (armed(+0xcc7)==1 && !v6_sync_follower(+0xcd2)) — 씬 생성 = ObjContest 진입의 TeamPlan 측 (f04fc3~f06e2a)
for idx in [1 /*epic · kind 4*/, 0 /*serpen · kind 5*/] {
  if !(exists(idx) && moba && live_list[idx].len != 0) { continue }
  n = ef6ac0(self, kind, player, data)                                                  // 적 committed 수
  lock_ok = !( (idx==1 ? v5_obj_armed(+0xcc6) : v4_obj_armed(+0xcc5)) && lock(+0xf0/+0xc0).is_some() && moba && live && { s = f1e3c0(kind); s.dead >= lock.dead && s.near >= lock.near && !(s.obj_hp > 99 && lock.obj_hp < 100) } )
  feasible = ef5a30(self, idx, version, player, data, goal_data)
  gate = n > 2 && (armed ? window_intent[idx](+0xc9e+idx) != 2 : true)
  if lock_ok && gate {
    if !feasible && { (a,e) = efbb70(kind, team, data); a < e && (bb[team].top.minion_count < −2 || mid < −2 || bottom < −2) } { /*skip*/ }
    else {
      arr = efb980(self, kind, version, player, data); t = efb0c0(self, idx, player, cache, ctx)
      if arr.len > 2 && t <= arr[2] + tps*6 {                                            // GATHER
        open_scene(idx, phase=2) /* f0ff30/f10550 인라인 동형: lock Some 이면 lock_opens++ · lock=None · released[idx]=0 · track_seen 처리 · scene={0,0,MAX, phase 2, with_battle 1, follower 0, stance_grind 0, take_born 0} */ ; chat(idx==0 ? SerpenHunt 0x1d : MorgardHunt 0x26)
        arrive_at[idx](+0x788) = tick + arr[2]; deadline[idx](+0x778) = tick + arr[2]*2 + tps*4; quorum_k[idx](+0x798) = 3; assemble_arrived/departed[idx] = 0; ef56d0(감사); commits[1](+0x738) += 1;
        if last_commit[idx](+0xc50) != 0 && tick − last_commit < tps*2 { v6_audit[4] += 1 }; last_commit[idx] = commit_at[idx](+0xc40) = tick; (debug: V6COMMIT gather 로그); *plan = ForcePassive; return
      }
    }
  }
  if armed && window_intent[idx] == 2 { cut_no(+0x770) += 1; continue }
  t = efb0c0(self, idx, …); if t <= tps*3 { (+0x750) += 1; continue }
  arr = efb980(…); if arr.len < 3 { (+0x758) += 1; continue }
  lock 재검사(+ `tps*12 < t` 조건) → 막히면 v4_lock_blocks(+0xc70) += 1; continue
  enemy_can = #적 챔프(HP≥30 && 가시 → dist(anchor)/speed <= t ; 사망자는 리스폰+이동) ; ally_can = #(arr[i] + tps*2 <= t)   // ★미독 세부(RE 요지)
  if ally_can > 2 {
    k = idx==1 ? min(ally_can, 5) : min(ally_can, max(enemy_can, 3))
    if (t − arr[k−1]).sat <= tps*4 {                                                     // CUT
      open_scene(idx, phase 2); arrive_at[idx] = tick + arr[k−1]; deadline[idx] = tick + 2*arr[k−1] + tps*4; quorum_k[idx] = k; … (gather 와 동일 부기 · V6COMMIT cut 로그); *plan = ForcePassive; return
    } else { (+0x768) += 1 /*stall*/ }
  }
}
if efc4f0(self, 1, 0, …, version, player, data, goal_data, plan, debug) { return }      // TAKE(epic): 씬 phase 3 즉시 + hunt_call_pending[1]=1(v3) + take_born=1 · arrive_at=tick · deadline=tick+tps*4+arr[2]*2 · quorum 3 · FP
if efc4f0(self, 0, 0, …) { return }                                                        // TAKE(serpen)
// :1005~1094  에픽/세르펜 셋업 — ★0.6.0 삭제(v≥2): armed 면 L1097 로 직행.  ★v2 사장(v1 전용): 구 본문(check_epic_setup fb34e0 · should_delay f18ad0 · check_epic_giveup fb49b0 · Morgard{Setup,1} · 정글 캠프 예외 · MorgardSetup 0x22 / check_serpen_setup 100f210 · f87600 · f18a80 · 1010060 · Serpen{Setup,1} · SerpenSetup 0x19) + v4/v5 lock·씬 대안(f10a20/f0ff30 · f10b40/f10550)
// :1097~1108  에픽 버프 없을 때 타워 압박
if moba.epic_minion_buff_time[team](+0x240+team*8) == 0 {
  epic_available   = morgard_exists && epic.live_list.len!=0   && epic_giveup_tick.is_none();
  serpen_available = serpen_exists  && serpen.live_list.len!=0 && serpen_giveup_tick.is_none();
  if !(epic_available || serpen_available) {
    if let Some(press_line) = check_press_tower_opportunity(f1fce0)(version, rnd, player, data, self, None, debug) {
      self.objective = Some(PressTower(press_line)); self.press_tower_start_tick = tick;
      efdf40(self, version, team, pos, bb, press_line);   // ★0.6.0 콜리 교체: PressTower 챗(0x2d) push — v3: bb[team].call_vec 에 kind(+0x46)==5 && pos(+0x20)!=나 있으면 생략
      *plan = ForcePassive; return;
    }
  }
}
// :1114~1128  에픽 버프 창
live_ally_count = iter_champions(team).count(); live_enemy_count = iter_champions(enemy).count();
if moba.epic_minion_buff_time[team] > tps*10 {
  if version > 1 { self.v3_epicops_buff_window(ef4690)(version, rnd, player, data, goal_data, plan); /*bool 무시*/ }
  else { if live_ally_count >= live_enemy_count { self.handle_press_epic(ef3da0)(…); } }   // ★v2 사장(v1)
} else if version > 1 {
  self.v3_press_chat_line(+0xcd4) = None(0xff);
}
return;   // :1131
```

## changes
- :831 넥서스 공격 챗 · :837 수비 챗: v3 는 bb[team].call_vec(+0x250/+0x258 · 0x50 stride) 에 같은 kind(4/3) 를 타인이 이미 올렸으면 챗 생략.
- :862~903 최근교전: armed(+0xcc7 · v≥2 상시) 면 `epic_av ‖ serpen_av → :910 직행(압박탑 생략)` · 아니면 f1fce0 → PressTower. 구 Morgard/Serpen{Hunt} 선점 + v4/v5 lock·씬 열기 가지는 `!armed` 전용(사장).
- :970~1000 스폰 귀환 앞에 `version>=3 && eda830(me) > tps → 귀환 생략`.
- 신규 V6 commit 블록(armed && !sync_follower): idx [1,0] 순회 gather(ef6ac0 n>2 · intent≠2 · lock_ok · efb980 arr[2]+6tps ≥ efb0c0) → open_scene(phase 2) / cut(ally_can>2 · quorum k · t−arr[k−1] ≤ 4tps) / 루프 후 efc4f0 take ×2(phase 3 즉시).
- :1005~1094 에픽/세르펜 Setup 선택 전체 삭제(armed → :1097 직행).
- :1097 PressTower 챗 push → `efdf40(self,version,team,pos,bb,line)`(v3 억제).
- PassiveJungle 필드 jungle +0x68→+0x83 · team +0x50→+0x60 · 나머지 오프셋 헤더 표.
- ★미독: f02ef6..f04f8f(:910~:1000 앞부분 = kill-lane · opportunistic push · v2 반격 · 컴백픽 · 스폰준비 앞) — 콜 시퀀스만 일치, 상수·조건은 0.5.8 본문 그대로 둠. V6 commit 의 enemy_can/ally_can 산식 세부 · goal_data.last_base_defense_tick 0.6.0 오프셋.
- ★v2 사장: 최근교전 `!armed` 가지 · :1005~1094 v1 본문 · :1123 handle_press_epic.

## verified
- capstone 0.6.0 f024d0 선형 디스어셈: efdbf0 → +0xcd5=4/+0xcd6 · `version>2` → call_vec 순회(kind +0x46==4 · pos +0x20 != mine → 꼬리) · chats push 0x2a(+0x2f8/+0x2e8/+0x2f0) · e74b40→d75ad0→e759a0 · e753e0 ×3(Mid 0x1b1 · Bottom 0x1ab · Top 0x185) · f87600 뒤 `cl = !cc7; test cl, epic_av` → cc6/cc5 → f10b40/f10550 · f10a20/f0ff30(!armed 가지) · f1fce0(f02dda) · JT f02ef4 · f27ea0/f27140(f04b67/f04bb5) · eda830(f04eef: `> [rbp+0x1e8]=tps → f04fbc`) · `cc7==1 && cd2==0 → V6 블록` else f05f62 · V6 블록 콜 순서(ef6ac0 ×2 · cc6/cc5 · ef5a30 · efbb70 · efb980 ×2 · efb0c0 ×2 · f1e3c0 ×3 · efc4f0 ×3 · ef56d0 · f0ff30) · f05f62 `cc7!=0 → f05f70(:1097 블록: vt+0x40 …)` / `==0 → f06645(v1 fb34e0 셋업)` · ef4690/ef3da0 · +0xcd4=0xff · f1fce0(f061e6) → efdf40(f06247).
- 미확인: 위 ★미독 구역(f02ef6..f04f8f) · V6 블록 내부 산식(RE §2-c 요지 채택).

## confidence
B — 선두(①~③·최근교전 armed 분기)·꼬리(귀환 v3·V6 commit 골격·:1005~1094 삭제·efdf40·:1114~) 는 asm 확인. 중간 :910~:1000 은 Ghidra 실패로 0.5.8 본문 유지(상수 미검증).
