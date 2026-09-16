# 배치 F — 변경 99 중 티어 2/3 14 (0.5.8 RVA → 0.6.0 RVA · exe 정규화 정렬 힌트 · 0.5.8 명세 요지)

정규화 정렬(`--norm`)이 이미 흡수한 것: PlayerState +0xd0 · Blackboard 0x2e8→0x5c8 · SmallActionPlay 태그/idx 재번호 · BattleSubPlanGoal 재번호 · Effect vt 슬롯(4 삽입) · LPH 오프셋 6. 아래 「잔여」 가 정렬로 못 닫은 차이.

## `e928f0` → `f40930` DefenseNexusSubPlan::action_candidates  (11291→11294B · Δ+3)
- 명령 2330→2332 · 정렬 2290 · exe 판정 ⚠정렬 불가(미확정) · 정규화 17 · 블록이동 15 · 스택슬롯 559 · 콜리 주의 4
- 잔여 구조: 구 40명령(lea|r14, [rbp + I]…) / 신 42명령(lea|rax, [rbp + I]…) 짝 없음 ; e92d0e 피연산자 수 ; e933f5 피연산자 형 ; e933fa 피연산자 형 ; e933ff 피연산자 형 ; e936d5 피연산자 수
- 잔여 즉치: 0x338→0x358 · 0x338→0x358
- 콜리 주의: 12857f0→1643790 변경 ; 128cc90→164b370 미지 ; 31a01a3→381e0b3 미지 ; eb82d0→eda920 변경
- 정규화 흡수: 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 2e8→5c8 · SmallActionPlay 태그 e→d · SmallActionPlay 태그 f→e · SmallActionPlay 태그 10→f
- 0.5.8 명세 #189 `defense_nexus__DefenseNexus__action_candidates` src game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:245 · one_line: 넥서스 방어 서브플랜의 행동 후보 루트(vtable 진입): 결사 대상 커밋(Trace)→위험시 도주 조기귀환(스킬)→미니언 웨이브 피해 도주→본진 포지셔닝(Around)→RunAway·전투·근접 미니언 공격(Attack/Skill/Skill2)·소환수·최근접 적 타워 공격·구조물 스킬 후보를 sret Vec<SmallActionPlay> 로 돌려준다
- 0.5.8 logic(앞 1500자):
```
fn action_candidates(&mut self, version, rnd, player, data, parameter, debug) -> Vec<SmallActionPlay>   [defense_nexus.rs:245~309]
L246 let mut res = Vec::new_in(data.context.pool)                                        // 45361~45371
L247 self.last_gate = 0                                                                 // 45372 (initializes 표면)
L250 res.extend(self.commit_chase(version, rnd, player, data, debug))                    // 인라인 45392~46059, 소스 L35~95:
  L35  let mut r = Vec::new_in(bump)
  L36  if version < 2 || !base_defense_focus(player, data) { L37 self.focus = None; return r }     // 45397~45406 (version<2 는 reach 접힘)
  L40  let Some(champ) = data.cache.player_champion[team][pos] else { return r }          // 45440~45445
  L45  let minion: Option<usize> = base_attacking_minion(player, data)                    // 45449 (LAST_STAND_MEMO 게이트 → uncached)
  L51  let champ_target: Option<usize> = if minion.is_some() { None } else {              // 45459
  L52    let nexus = data.cache.nexus[team]; L53 let twins = &data.cache.twin_towers[team];
  L54~56 let threats: Vec<&Entity> = iter_champions(1−team).filter(|e| e.attack_effect.map_or(false, |atk| nexus.map_or(false,|n| atk.is_in_range(e,n)) || twins.iter().any(|t| atk.is_in_range(e,t)))).collect_in(bump)   // aux closure$0
  L59~60 threats.iter().min_by_key(|e| {
  L61~63   let attackers = iter_champions(team).filter(|a| dist_sq(a,e) <= 120000²).collect_in(bump)      // aux closure$1::closure$0
  L64~67   let tow
```

## `eb43a0` → `ead1c0` LineWaitSubPlan::action_candidates  (4704→4676B · Δ-28)
- 명령 998→997 · 정렬 989 · exe 판정 ⚠정렬 불가(미확정) · 정규화 6 · 블록이동 130 · 스택슬롯 130 · 콜리 주의 1
- 잔여 구조: 구 9명령(add|rcx, r11…) / 신 8명령(mov|rdi, qword ptr [rbp + I]…) 짝 없음
- 잔여 분기: eb4466 → eb448f/ead29b ; eb4568 → eb459d/ead29b ; eb458e → eb448f/ead39a ; eb476f → eb4791/ead7ea ; eb4d0f → eb4ebd/ead7f1
- 콜리 주의: 31a01a3→381e0b3 미지
- 정규화 흡수: 오프셋표 [r10+930→a00] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · SmallActionPlay 태그 f→e · SmallActionPlay 태그 f→e
- 0.5.8 명세 #195 `line_wait__LineWait__action_candidates` src game-ai\src\plan_legacy\sub_plan\line_wait.rs:144 · one_line: 라인 대기(LineWait) 서브플랜 후보 — 기지 포지셔닝(타워/앞미니언 뒤 180000) + 도주 + 전투/라인미니언/소환/타워공격/구조물스킬 후보를 모은 뒤 v30 타워 어그로 위험 후보를 retain 으로 제거
- 0.5.8 logic(앞 1500자):
```
fn action_candidates(&mut self, version, rnd, player, data, _parameter) -> Vec<SmallActionPlay>
L145 bump = data.context.pool; res = Vec::new_in(bump)
L147 res.extend(self.base_positioning(version, rnd, player, data)):   // line_wait.rs:17~53 인라인
  L17   sub = Vec::new_in(bump); purpose = PositionEvalPurpose::LaneSafe(10)
  L19   team = player.info.team (<2); optb: Option<&Entity> = match self.line { Top => cache.top_tower[team].or(top_tower2[team]), Mid => mid_tower.or(mid_tower2), Bottom => bottom_tower.or(bottom_tower2) }
  L20~25 optb = optb.or( cache.twin_towers[team].iter().min_by_key(|t| dist²(t, self.line.get_start_position(context.setting, team))) )   // 빈 Vec 이면 None · 동률 첫 원소
  L26   optb = optb.or(cache.nexus[team])
  L27   nearest_tower = optb.unwrap()
  L29   nexus = cache.nexus[team].unwrap()
  L31   ms = data.blackboard[team].{top|mid|bottom}_minion_state (self.line 기준)
  L32   fm = ms.front_minion.and_then(|id| cache.game.get_entity_by_id(id))
  L34   if fm.is_none() || dist²(fm, nexus) < dist²(nearest_tower, nexus) {   // 앞미니언이 타워보다 넥서스에 가까움(=타워 뒤)
  L35     sub.push(Around(SmallActionAround::new(version, rnd, data, player, target=nearest_tower.id, 5).with_position_eval_purpose(LaneSafe)))   // 태그 5
  L41   } else if dist²(fm, nearest_tower) < 32400000001 (180000²+1) {
  L42     sub.push(Around(SmallActionAround::new(…, nearest_tower.id, 5).with_position_eval_purpose(LaneSafe)))
        } else {
  L46     (bx,by) = behind(fm, 180000):   // line_wait.rs:55~95
```

## `dc3240` → `fddcd0` SmallActionRunAway::get_input  (14264→14318B · Δ+54)
- 명령 2954→2963 · 정렬 2894 · exe 판정 ⚠정렬 불가(미확정) · 정규화 15 · 블록이동 69 · 스택슬롯 259 · 콜리 주의 3
- 잔여 구조: 구 60명령(mov|r14, qword ptr [rcx + r9*I…) / 신 69명령(mov|qword ptr [rbp + I], rax…) 짝 없음 ; dc3533 피연산자 형 ; dc4fde 피연산자 수 ; dc5a7d 피연산자 수
- 잔여 분기: dc33c7 → dc33de/fdde75 ; dc33e1 → dc33f8/fddea9 ; dc33fb → dc3416/fdde8f ; dc3414 → dc3455/fe0fa9 ; dc3b72 → dc3b8f/fde646
- 잔여 변위: rcx+0x1b0→0x1d0 · r8+0x660→0x8e8 · rsi+0x660→0x628
- 데이터: dc33e6 movaps 16B 01000000000000000500000000000000→01000000000000000600000000000000 ; dc3400 movaps 16B 01000000000000000600000000000000→01000000000000000500000000000000
- 콜리 주의: 12857f0→1643790 변경 ; ceddd0→e2e5a0 불일치(mig060 는 e266e0) ; d0dae0→e50aa0 불일치(mig060 는 e4cb60)
- 정규화 흡수: 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8]
- 0.5.8 명세 #182 `move_actions__SmallActionRunAway_get_input` src game-ai\src\small_action\move_actions.rs:95 · one_line: 도주 소액션 입력: 홈(분수/타워) 방향 7x7 셀 후보를 위험·귀환거리로 점수화해 목표를 고르고(히스테리시스·커밋) PathFinder 로 경로 입력을 낸다
- 0.5.8 logic(앞 1500자):
```
== 준비 (L96~L128) ==
team = player.info.team (<2 아니면 panic); champ = data.cache.player_champion[team][player.info.position]? → None 이면 return None(-1)
nearest_ally_tower = cache.iter_towers(team).min_by_key(|t| t.distance(champ) + (t.can_target && t.block_target_tick==0 ? 0 : 100000))   // L100, 동률 첫 원소
nexus = cache.nexus[team]; enemy_team = 1-team
nearest_enemy_champion = player_champion[enemy_team].iter().flatten().filter(|e| blackboard[enemy_team].is_recent_visible(game, player, e))   // ★bb 인덱스 = 1-team (적 팀 blackboard = 적 챔피언의 최근가시 표).min_by_key(|e| dist_sq(e,champ))   // L105~106
enemy_champion_to_champ = nearest_enemy.map(|e| e.distance(champ)).unwrap_or(u64::MAX)   // L108 (-1)
(enemy_positions[5], visible_enemy_count) = collect_visible_enemy_positions(player, data)   // L111
enemy_knows = version>1 ? enemy_knows_my_position(player,data) : false   // L116
eff_risk(s) := s.risk + (enemy_knows ? s.unseen_champ_threat : 0)   // L117 지역 클로저
now_tick = game.tick()   // L123

== 진행도 추적 (L125~L135) ==
if self.path_finder.is_none() || pf.path_len==0 { prog_now = usqrt(dist_sq(champ, (goal_x,goal_y))) }   // L131
else { idx = min(pf.path_len-1, pf.index); (wx,wy) = pf.path[idx] (idx<70); prog_now = usqrt(dist_sq(champ,(wx,wy))) + (pf.path_len-1-idx)*32000 }   // L127~129 남은 경로 길이
if prog_now < self.prog_best_dist_sq { self.prog_best_dist_sq = prog_now; self.prog_best_tick = now_tick }   // L133~135

== 커밋 목표 재평가 (L138~L161) ==
stale_goal_cell = None
if self.goal_committed {
  
```

## `d3b2a0` → `eed7f0` v46_flee_gate_check  (4673→4756B · Δ+83)
- 명령 1012→1026 · 정렬 1002 · exe 판정 ⚠정렬 불가(미확정) · 정규화 7 · 블록이동 2 · 스택슬롯 199 · 콜리 주의 3
- 잔여 구조: 구 10명령(add|rsi, I…) / 신 24명령(mov|rdx, rax…) 짝 없음 ; d3b9b3 피연산자 수
- 잔여 분기: d3bad4 → d3bd80/eee300
- 잔여 즉치: 0x228→0x238 · 0x228→0x238 · 0xfa0→0x1090 · 0x320→0x350 · 0x0→0xf0 · 0xfa0→0x1090 · 0x320→0x350
- 콜리 주의: 12857f0→1643790 변경 ; 128cc90→164b370 미지 ; d31bb0→d72be0 미지
- 정규화 흡수: 오프셋표 [rdi+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+930→a00] · 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90]
- 0.5.8 명세 #92 `passive_line__v46_flee_gate_check` src game-ai\src\plan_legacy\old\passive_line.rs:38 · one_line: v46 라인 도주 게이트 — 피난처(가장 가까운 타워/넥서스) 사거리 밖에 있을 때 근처 적 중 '나를 잡을 수 있는' 위협(committers)을 뽑고, 없으면 차단 사유 코드를 돌려준다
- 0.5.8 logic(앞 1500자):
```
L40: pool = context.pool; L41: tps = setting.tick_per_second; L42: tick = cache.game.tick()[vtable 0x28]; L43: committers = Vec::new_in(pool)
L46: team = player.info.team; refuge = cache.iter_towers_without_nexus(team).min_by_key(closure$0: |t| t.distance_sq(champ))   (fold 은 별도 define — 키 = 타워↔나 distance_sq, 가장 가까운 타워)
L47: refuge = refuge.or(cache.nexus[team])   (team<2 bounds)
L48: refuge None → L49 return (5, [])
L51: under_refuge = refuge.attack_effect.as_ref().map_or(false, closure$1: |atk| champ.distance_sq(refuge) <= (atk.range(refuge) + champ.radius())^2)
     atk.range(e) 인라인(effect.rs:26) = atk.range + e.stat_buff_cached.range + (e.level-1)*atk.growth_range ; radius() 인라인(entity.rs:1511) = mult==0 ? radius : radius*(mult+100)/100
L55: if under_refuge → L56 return (5, [])   (IR 55766: dist_sq > r² 이면 %172 로 계속, 아니면 %181 코드 5 — 권역 안이면 도주 게이트 차단; attack_effect None 이면 map_or(false) → 계속)
L59~62: near_enemies = cache.iter_champions(1-team).filter(closure$2 = aux 66064: |e| e.distance_sq(champ) < 22500000001 && champ.is_visible_from(e)[champ.team Neutral→true; Player(t)→ e.visible_state[t]==Visible] && !is_ignored_well_enemy(version, player, e)).collect(pool)
L64: near_enemies.is_empty() → L65 return (1, [])
L69: my_pos = player.info.position; L70: reaction = champ.attack_duration() + 6; L71: my_ms = champ.stat_cached.move_speed; L72: escape_ticks = champ.distance(refuge) / max(my_ms,1)
L73~76: tower_disable_tick = match context.player_count()[tutorial→인원] { 2 => settin
```

## `cba660` → `e5eb00` StealSubPlan::action_candidates  (5108→5224B · Δ+116)
- 명령 1146→1162 · 정렬 1137 · exe 판정 ⚠정렬 불가(미확정) · 정규화 11 · 스택슬롯 55 · 콜리 주의 3
- 잔여 구조: 구 9명령(cmove|r8, rcx…) / 신 25명령(cmove|rdx, rcx…) 짝 없음 ; cbb741 피연산자 형
- 잔여 분기: cbb6db → cbb6e4/e5fbf2
- 콜리 주의: 128cc90→164b370 미지 ; 31a01a3→381e0b3 미지 ; cb0310→e5ea10 미지(-2B)
- 정규화 흡수: 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · SmallActionPlay 태그 f→e · SmallActionPlay 태그 c→b · SmallActionPlay 태그 10→f · SmallActionPlay 태그 11→10 · SmallActionPlay 태그 12→11
- 0.5.8 명세 #210 `steal__Steal__action_candidates` src game-ai\src\plan_legacy\sub_plan\steal.rs:33 · one_line: 정글러 막타 스틸 후보군: commit 이면 대상 오브젝트 사거리 내 공격/스킬/스킬2/궁 + 진입 지점 대기, Lurk 면 적 근접 시 도주 아니면 대기 부시
- 0.5.8 logic(앞 1500자):
```
fn action_candidates(&mut self, _version, rnd, player, data, _parameter) -> Vec<SmallActionPlay>   [steal.rs:33]
  bump = data.context.pool ; res = Vec::new_in(bump)                                              [L34 · 25393~25403]
  champ = data.cache.player_champion[player.info.team /*<2*/][player.info.position]              [L36 · 25402~25434]
  if champ is None { res.push(Recall(SmallActionRecall::new(data, player, 5))); return res }      [L37 · 25537, 27384~27422]

  // ---- 대상 오브젝트 해석 (steal.rs:27/28/30 헬퍼 인라인) ----
  target_ent = match self.target {                                                                 [L41 · 25437~25447]
      None(2)      → 폴백 →                                                                        [→ %872]
      Some(Epic=0) → moba = game.get_game_mode()/*판별자0=Moba 아니면 unwrap 패닉*/ ; moba.jungle_runner.epic.live_list.first()   [L27 · +0x198]
      Some(Serpen) → … .jungle_runner.serpen.live_list.first()                                    [L28 · +0x1c8]
  } → id → game.get_entity_by_id(id)                                                             [L30 · 25514~25525]
  if 해석 실패(live_list 비었거나 엔티티 없음 · %99) 또는 None {                                   [L43 · 25547]
      (x,y) = data.context.map.camp_pos(if self.target==Serpen { JungleType::Serpen(5) } else { Morgard(4) }, player.info.team == 0)   [L44/L45 · 27318~27329]
      res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, x, y, 5))); return res      [L47 · 27339~27379]
  }

```

## `cb1ce0` → `e986c0` SerpenHuntSubPlan::action_candidates  (20647→20494B · Δ-153)
- 명령 4030→4062 · 정렬 3726 · exe 판정 ⚠정렬 불가(미확정) · 정규화 24 · 블록이동 528 · 스택슬롯 1078 · 콜리 주의 23
- 잔여 구조: 구 304명령(mov|r8d, dword ptr [rax + I]…) / 신 336명령(mov|edx, dword ptr [rax + I]…) 짝 없음 ; cb1ff8 피연산자 형 ; cb21a8 피연산자 수 ; cb2300 피연산자 형 ; cb232b 피연산자 형 ; cb237f 피연산자 형
- 잔여 분기: cb21b8 → cb229c/e98c6c ; cb2433 → cb2b5a/e993ee ; cb245e → cb2430/e98e10 ; cb246f → cb25df/e99017 ; cb24f2 → cb251e/e98ee0
- 잔여 소형 즉치: 0xcb2aa1:0x11→0xe · 0xcb3524:0x8→0x0 · 0xcb352f:0x0→0x8 · 0xcb4ef9:0xf1→0xf2 · 0xcb54bc:0x4→0x1
- 잔여 즉치: 0x668→0x688 · 0x11→0xe · 0x8→0x0 · 0x0→0x8 · 0xf1→0xf2 · 0x4→0x1 · 0x668→0x688
- 잔여 변위: r8+0x130→0x148 · rbx+0x5c8→0x660 · rbx+0x10→0x5c0 · rdx+0x438→0x1f8 · rdx+0x4a0→0x4a8 · rdx+0x4a8→0x5c8 · rdx+0x5c8→0x438 · r15+0x4a0→0x5c8 · r15+0x5c8→0x438
- 콜리 주의: 12857f0→1643790 변경 ; 128cc90→164b370 미지 ; 128cf70→165e7a0 콜리 변경?(J0.00·+226B) ; 128f0→e725d0 미지(+155B) ; 12a0180→dc05a0 미지(-222B) ; 31a01a3→381e0b3 미지
- 정규화 흡수: 오프셋표 [rax+930→a00] · 오프셋표 [rcx+508→578] · 오프셋표 [rcx+4f8→568] · SmallActionPlay 태그 e→d · 오프셋표 [rcx+508→578] · 오프셋표 [rcx+4f8→568] · 오프셋표 [rax+928→9f8] · SmallActionPlay 태그 e→d
- 0.5.8 명세 #197 `serpen_hunt__SerpenHunt__action_candidates` src game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:201 · one_line: 세르펜 사냥 서브플랜의 행동 후보 목록 생성 — need_recall 판정→Recall 단독 / v27 규율 행동 단독 / 공격 후보(battle·summon·jungle)를 타워·대상·아군스킬·score 필터로 거른 뒤 get_move_action(추적·도주·대기)과 합쳐(431~514, 배치 H·I) 반환
- 0.5.8 logic(앞 1500자):
```
// serpen_hunt.rs:0~428 (배치 G)
// ── 함수 머리 serpen_hunt.rs:201 · 정본 = m02.ll:11075~ · 이 배치 = 루트 줄 203~428 + 그 줄에 뿌리를 둔 클로저 5개(aux)
// 표기: `L<n>` = 소스 루트 줄 · `%N` = m02.ll 레지스터 · 오프셋은 tcxdict 정본 이름 · 「→ 배치 X(줄 N)」 = 다른 배치 범위로 넘어가는 br
//
// L203 (m02.ll:11261~11283): team = player.info.team(+0x930); if team >= 2 → panic_bounds_check(2)
//        champ = data.cache(+0).player_champion(+0x1e0)[team][player.info.position(+0x9c0 i32)]  ; None(null) → unwrap_failed(%120)
// L204 (11288~11290): if self.need_recall(+0x0 bool) {
// L212 (11310~11313 · 11391~11394):   max_hp = champ.stat_cached.hp(+0x628); max_hp==0 → panic_const_div_by_zero
//                                     hp_ratio = champ.hp(+0x670) * 100 / max_hp
// L213 (11396~11397):   if hp_ratio > 29 { self.need_recall = false }   // IR `icmp ugt 29` — 소스 표기(>29 / >=30)는 표기 불가
//        } else {
// L205 (11297~11348):   gm = data.cache.game.<vtable+0x40 get_game_mode>() ; tag(+0)!=0(Moba 아님) → unwrap_failed(%167)
//                       m = gm.payload(&MobaMode) ; live = m.jungle_runner.serpen.live_list(ptr +0x1d0, len +0x1d8)
//                       serpen: Option<&Entity> = live.first().and_then(|id| game.<vtable+0x1f0 get_entity_by_id>(*id))   // len==0 → None ; get_entity null → None
//                       if let Some(serpen) = serpen {
// L206 (11353~11363):     serpen.attack_effect(+0x490) 의 tag(+0x4c0)== -1(None) → unwrap_failed(%161)
//                         dmg = Effect::expected_damage_target(&serpen.attack_e
```

## `dc2070` → `ecfa40` SmallActionAroundPositionBush::get_input  (690→898B · Δ+208)
- 명령 146→180 · 정렬 126 · exe 판정 ⚠정렬 불가(미확정) · 정규화 1 · 블록이동 10 · 스택슬롯 36 · 콜리 주의 2
- 잔여 구조: 구 20명령(mov|eax, dword ptr [r13 + I]…) / 신 54명령(mov|eax, dword ptr [rbp + I]…) 짝 없음 ; dc2267 피연산자 형 ; dc2267 피연산자 형 ; dc226b 피연산자 형 ; dc226b 피연산자 형 ; dc2270 피연산자 형
- 잔여 분기: dc211a → dc2128/ecfafa ; dc2123 → dc22ea/ecfd8a ; dc2265 → dc2270/ecfaee
- 잔여 즉치: 0x288→0x2d8 · 0x288→0x2d8
- 콜리 주의: ce0e60→e1d540 불일치(mig060 는 e1b5b0) ; ceae90→e2c6b0 불일치(mig060 는 e29770)
- 정규화 흡수: 오프셋표 [r13+930→a00]
- 0.5.8 명세 #157 `SmallActionAroundPositionBush__get_input` src game-ai\src\small_action\around.rs:1086 · one_line: 목표 수풀 셀까지 타워 회피 경로탐색(PathFinder) 입력 생성 — 목표 반경 16000 안이면 None
- 0.5.8 logic(앞 1500자):
```
SmallActionAroundPositionBush::get_input(&mut self, version, rnd, player, data, ps) -> Option<Input>   [around.rs:1086]
 entity = data.cache.player_champion[player.info.team][player.info.position].unwrap()   (L1087; team<2 아니면 panic, None 이면 unwrap 패닉)
 (ex, ey) = (entity.x, entity.y) ; (tx, ty) = (self.target_x, self.target_y)
 if |ex-tx|² + |ey-ty|² < 256000001:   # 16000²+1                                (L1089)
     return None                                                                  (L1090)
 tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, tx, ty)   # 416B 지역   (L1093)
 if self.path_finder.is_none():                                                   (L1094, 태그 +0x5d == 2)
     self.path_finder = Some(PathFinder::new_target(rnd, data.context, version, "dodge_tower_cell", ex, ey, tx, ty,
         |sx,sy,nx,ny| /*closure#0, 환경 {version, player, data, ps, &tower_dodge}*/ … -> PathVerdict))     (L1095)
     if self.path_finder.is_none(): return None      # ★사장 분기 — new_target 은 태그 2 를 쓰지 않는다(m03.ll:14343~17815, +0x45 에 0/1 만) · Option::as_mut 의 접히지 않은 match   (L1104)
 self.path_finder.as_mut().update_path(rnd, data.context, ex, ey, tx, ty, |sx,sy,nx,ny| … /*closure s_0, 같은 환경*/)   (L1106)
 return Some(self.path_finder.get_input(player, data, SafeMoveWithSkill::Safe, false))   (L1114; sret 직접 기록)

클로저 계약(관측): 두 클로저 모두 환경 {version, player, data, ps, &tower_dodge} 를 잡고 PathFinder 인스턴스(m03.ll 14343~17816 / 31379~32434) 안에 인라인돼 `dodge_tower_cell_w
```

## `cbbdb0` → `e9fe00` BattleSubPlan::action_candidates  (23506→23800B · Δ+294)
- 명령 4742→4795 · 정렬 3802 · exe 판정 ⚠정렬 불가(미확정) · 정규화 51 · 블록이동 1591 · 스택슬롯 886 · 패닉스텁 재배열 11 · 콜리 주의 34
- 잔여 구조: 구 940명령(mov|rax, r8…) / 신 993명령(mov|rsi, qword ptr [rbp + I]…) 짝 없음 ; cbc16f 피연산자 형 ; cbc918 피연산자 형 ; cbcbeb 피연산자 형 ; cbcc11 피연산자 형 ; cbcfce 피연산자 형
- 잔여 분기: cbc155 → cbc1b6/ea01c9 ; cbc198 → cbc1af/ea01c2 ; cbc1db → cbbf92/e9ffcc ; cbc5b0 → cbc6a6/ea08c5 ; cbc61d → cbc6cf/ea06ff
- 잔여 소형 즉치: 0xcbbe17:0x2→0x3 · 0xcbd0b5:0xf1→0xf2 · 0xcbd533:0x3→0x1 · 0xcbda24:0x5→0x0 · 0xcbdb1c:0x18→0x20 · 0xcbe5a5:0x20→0x18 · 0xcc0a72:0x28→0x20 · 0xcc0cab:0x6→0x3 · 0xcc1514:0xf→0xd
- 잔여 즉치: 0x638→0x628 · 0x2→0x3 · 0x19→0x5 · 0x28→0x2 · 0xf1→0xf2 · 0x3→0x1 · 0x5→0x0 · 0x18→0x20 · 0x20→0x18 · 0x101→0x1
- 잔여 변위: rbx+0x10→0x18 · r13+0x1f0→0xf8 · r13+0x1f0→0x28 · rax+0x668→0x10 · r10+0x10→0x18 · rdx+0x4a8→0x4a0 · rdx+0x5c8→0x4a8 · rdx+0x438→0x5c8 · r10+0x18→0x10 · r8+0x8→0x430
- 데이터: cbc83b movaps 16B 01000000000000000200000000000000→01000000000000000300000000000000 ; cbc86c movaps 16B 01000000000000000300000000000000→01000000000000000200000000000000 ; cbd68f movaps 16B 01000000000000000200000000000000→01000000000000000300000000000000 ; cbd703 movaps 16B 01000000000000000300000000000000→01000000000000000200000000000000
- 콜리 주의: 12a04b0→de9900 미지(+404B) ; 12a07d0→db1eb0 콜리 변경?(J0.00·+1143B) ; 12a07d0→e09ad0 미지(-81B) ; 31a01a3→381e0b3 미지 ; 31a37c0→e920f0 미지(+387B) ; 31a3863→3821770 미지(-32B)
- 정규화 흡수: 오프셋표 [rdx+930→a00] · 오프셋표 [rdx+9c0→a90] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [r13+1e0→3e8] · SmallActionPlay 태그 1→0 · SmallActionPlay 태그 e→d · SmallActionPlay 태그 e→d
- 0.5.8 명세 #188 `battle__Battle__action_candidates` src game-ai\src\plan_legacy\sub_plan\battle.rs:358 · one_line: 전투 서브플랜 행동 후보 루트: 논타겟 궤적/캐스트라인 강제 회피(RunAway dodge) 게이트와 v48 킬확보·지는판정·카이팅 위치 계산 → base_positioning(536) 후보 → 타워/치명 후처리(537~614)
- 0.5.8 logic(앞 1500자):
```
// battle.rs:0~518 (배치 A)
// 시그니처: fn action_candidates(&mut self, version, rnd, player, data, parameter, team_plan, _debug) -> bumpalo Vec<SmallActionPlay>
// 인자 DI 이름 = m02.ll:27712~27719 (#dbg_value: self=%1 version=%109(스택복사) rnd=%3 player=%4 data=%5 parameter=%6 team_plan=%7 _debug=%8)
//
// L359  self.v48_dodge_claim = false            // %1+43 (0x2b) store i8 0  [m02.ll:27739]
// L360  self.last_bail_gate  = 0                // %1+45 (0x2d) store i8 0  [27741]
// L363  final_stand = version >= 2 && nexus_final_stand(player, data)   // %112 = version<2 → false 단락 [27742~27751]
// L364  res = bumpalo Vec::new_in(data.context.pool)  // %108: ptr=8(dangling) bump=*(context+0) cap=len=0 [27754~27765]
// L367  champ = data.cache.player_champion[player.info.team(+0x930)][player.info.position@tag(+0x9c0)].unwrap()
//        // cache+0x1e0 + team*40 + pos*8 ; None → unwrap_failed(anon.171) [27764~27800, 27967]
// L370  enemy = 1 - team ; enemies = cache.player_champion[enemy][0..5] (슬라이스 %143, 끝 %144)
//       has_non_target_action_range = enemies.iter_champions().any(closure$0) 인라인 [27852~27972]:
//         closure$0(c): nontarget_windup_perceived(version, player, data, c) && c.ty@tag(+0x68)==13(Champion) &&   // L371
//           match c.action_state@tag(+0x70) {                                                                    // L372/374
//             4(Skill)  => eff = c.skill_effect(+0x4c8).as_ref().unwrap() (tag +0x4f8: -1=None→panic) ;
//                          eff.
```

## `ec9de0` → `f88880` wave_priority_clearer_position  (1045→685B · Δ-360)
- 명령 261→158 · 정렬 113 · exe 판정 ⚠정렬 불가(미확정) · 블록이동 14 · 스택슬롯 9 · 콜리 주의 3
- 잔여 구조: 구 148명령(mov|rax, rcx…) / 신 45명령(mov|rax, qword ptr [rcx + I]…) 짝 없음 ; eca1a3 피연산자 형
- 잔여 분기: ec9f14 → ec9f6d/f88a0a ; ec9f1a → ec9f3b/f889da ; ec9f23 → ec9f51/f889ef ; ec9f39 → ec9f65/f88a02 ; ec9f4f → ec9f65/f88a02
- 잔여 소형 즉치: 0xec9f95:0x5→0x0
- 잔여 즉치: 0xa8→0x88 · 0x90880→0x59d80 · 0xdea80→0xa7f80 · 0x42680→0xbb80 · 0x5→0x0 · 0x1d→0x1 · 0x2→0x90880 · 0x3→0xdea80 · 0x4→0x42680 · 0xa8→0x88
- 콜리 주의: 31a3863→f5d920 미지(+940B) ; 31a3b40→3821813 미지(+32B) ; e2f230→f5d920 불일치(mig060 는 db73c0)
- 0.5.8 명세 #73 `objective_helpers__wave_priority_clearer_position` src game-ai\src\plan_legacy\team_plan\objective_helpers.rs:245 · one_line: 웨이브 우선 정리 담당 포지션 선정 — 적 미니언 중 우리 넥서스 최근접(없으면 라인 1차 타워 위치)에 가장 가까운 HP 30%+ 아군 포지션
- 0.5.8 logic(앞 1500자):
```
fn wave_priority_clearer_position(player, data, line) -> Option<usize>
let team = player.info.team;
// L246  target = wave_priority_clear_target(player, data, line)  — objective_helpers.rs:258~266 전부 인라인
//   L258  let nexus = cache.nexus[team]                         // +0x170 · None → 폴백
//   L259  let minions = cache.line_minions(line)[1 - team]      // simulation.rs:1807 · 0x10 + line*0x40 + enemy*0x20 (bumpalo Vec<&Entity>)
//   L260  minions.iter().min_by_key(|m| dist_sq(m, nexus))      // closure wave_priority_clear_target0 (인라인) · 빈 목록 → None
//   L261  Some(m) → target = (m.x, m.y)
//   L265  None(넥서스 없음 | 적 미니언 없음) → target = first_tower_position(team, line):
//         Top: team0 (48000,272000) / team1 (272000,48000)   Mid: team0 (368000,592000)/team1 (592000,368000)   Bottom: team0 (688000,912000)/team1 (912000,688000)
// L249~253
(0..5).filter_map(|pos| {
    let c = cache.player_champion[team][pos]?;                 // L249
    if !(c.hp * 100 / c.stat_cached.hp > 29) { return None; }  // L250 · max_hp==0 이면 div_by_zero 패닉
    Some((pos, dist_sq(c, target)))                            // L251
})
.min_by_key(|&(pos, d)| (d, pos))                              // L253 · 키 = (거리², 포지션) — 동거리면 낮은 포지션
.map(|(pos, _)| pos)                                           // L254~255
```

## `dff080` → `ddd7e0` base_sub_goal  (2440→2889B · Δ+449)
- 명령 542→639 · 정렬 394 · exe 판정 ⚠정렬 불가(미확정) · 정규화 16 · 블록이동 63 · 스택슬롯 9
- 잔여 구조: 구 148명령(cmp|dword ptr [rcx], I…) / 신 245명령(mov|r10, qword ptr [rcx]…) 짝 없음
- 잔여 분기: dff123 → dff9d0/dddaa9 ; dff1eb → dff3b6/dddb1f ; dff21a → dff3b6/dddb1f ; dff240 → dff3b6/dddb1f ; dff29a → dff3b6/dddb1f
- 잔여 소형 즉치: 0xdff9c4:0x3→0x1 · 0xdff9cb:0x4→0x2
- 잔여 즉치: 0x78→0x88 · 0x9502f9000→0x35a4e9001 · 0x27101→0x2 · 0xfa01→0x27101 · 0xfa01→0x27101 · 0x27101→0xfa01 · 0x9502f9000→0x35a4e9001 · 0xfa01→0x27101 · 0xfa01→0x27101 · 0x27101→0xfa01
- 정규화 흡수: 오프셋표 [rdi+930→a00] · BattleSubPlanGoal 7→4 · 오프셋표 [r8+930→a00] · 오프셋표 [r8+9c0→a90] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8] · BattleSubPlanGoal 7→4
- 0.5.8 명세 #68 `battle__base_sub_goal` src game-ai\src\plan_legacy\old\battle.rs:69 · one_line: BattlePlanGoal(TryKill/Support/Response/Avoid) → 기본 BattleSubPlanGoal 결정: 대상 추적(Trace) / 우물 위험이면 End / 가까운 적 있으면 KitingBack 없으면 RunAway
- 0.5.8 logic(앞 1500자):
```
fn base_sub_goal(&self, version, player, data) -> BattleSubPlanGoal
  team = player.info.team; enemy_team = 1 - team
  if self.tag < 2 {                                   // L70: TryKill{0: focus, ..} | Support{0: focus}
    focus = self.payload+8                            // L71
    target = game.get_entity_by_id(focus)             // L72 (vtable+0x1f0)
    // L73: if let Some(t) = target { if is_ignored_well_enemy(version, player, t) { return End } }   (fight_model.rs:754~756 인라인)
    //   is_ignored_well_enemy(t) = t.team == TeamType::Player(enemy_team) && is_enemy_well_danger(version, player, t.x, t.y)
    if target.is_some() && target.team@tag == 0 && target.team.0 == enemy_team
       && is_enemy_well_danger(version, player, target.x, target.y) → return End(7)
    return Trace{focus}(0)                            // L72 (target None 이거나 우물 위험 아님)
  }
  // Response | Avoid
  me = cache.player_champion[team][player.info.position].unwrap()     // L80 (None → unwrap_failed 패닉)
  nearest = cache.iter_champions(enemy_team)                            // L81 player_champion[1-team] 의 Some
      .filter(|e| data.blackboard[enemy_team].is_recent_visible(game, player, e)     // L82 (⚠ 적 팀 판 blackboard)
               && dist²(e, me) < 200000² + 1                                        // L83
               && !is_ignored_well_enemy(version, player, e))                       // L84 (= 적 챔피언이 우물 위험 안이면 제외)
      .min_by_key(|e| dist²(e, me))                                         
```

## `e5ca10` → `d5f3c0` try_engage  (2148→2776B · Δ+628)
- 명령 432→564 · 정렬 363 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 블록이동 49 · 스택슬롯 89 · 패닉스텁 재배열 1 · 콜리 주의 5
- 잔여 구조: 구 69명령(mov|rsi, r9…) / 신 201명령(mov|rdi, r9…) 짝 없음 ; e5ca64 피연산자 형 ; e5ca6d 피연산자 형 ; e5cab8 피연산자 형 ; e5cab8 피연산자 형 ; e5cbf9 피연산자 형
- 잔여 분기: e5ca4e → e5ca98/d5f52d ; e5ca5b → e5ca9c/d5f45b ; e5ca91 → e5ca9c/d5f92e ; e5ca93 → e5ce3d/d5fa0d ; e5cad8 → e5cdb0/d5f523
- 잔여 소형 즉치: 0xe5cb5b:0x6→0x0
- 잔여 즉치: 0x328→0x558 · 0x6→0x0 · 0x0→0x3c · 0x118→0x228 · 0x118→0x228 · 0x7→0x3d090 · 0x118→0x228 · 0x328→0x558 · 0x118→0x228
- 잔여 변위: r15+0x8→0x0 · r15+0x8→0x0
- 콜리 주의: 31a01a3→381e0b3 미지 ; dfb5d0→dd8b60 변경 ; dfb840→dd8e70 변경 ; dfdd50→ddc9f0 불일치(mig060 는 dd8b60) ; e07430→ee6d40 변경
- 정규화 흡수: SmallActionPlay 태그 1→0 · 오프셋표 [rax+930→a00]
- 0.5.8 명세 #80 `engage__try_engage` src game-ai\src\plan_legacy\handler\engage.rs:40 · one_line: target_id 상대로 BattlePlan(일반/다이브)을 만들어 첫 update 까지 돌리고, 게이트(패배직후 쿨·다이브 재진입 쿨·추격무망·다이브 불가·즉시 후퇴 서브골)에 걸리면 None 을 돌려준다
- 0.5.8 logic(앞 1500자):
```
try_engage(&self, version, rnd, player, data, target_id, debug) -> Option<BattlePlan>
  if version > 1 && target_id == self.last_lost_fight.0 {                     // L43
    if game.tick() <= self.last_lost_fight.1 + tps*3 { return None } }        // L44~45  패배 직후 같은 상대 3초 재교전 금지
  target = game.get_entity_by_id(target_id)                                  // L52
  in_tower = target.map_or(false, |t| engage_requires_dive(player, data, t)) // L52 closure$0
  battle = if in_tower {                                                     // L53
    t = get_entity_by_id(target_id)?  (None → return None)                   // L54
    if game.tick() <= self.last_dive_abandon_tick + 1 + tps*4 { return None } // L57 (dive_rejoin_cd)
    if version > 1 {                                                         // L63
      if let Some(me) = cache.player_champion[team][pos] {                   // L64
        if open_chase_race_hopeless(version, data, player, me, t) { return None } } }   // L65
    if !tower_dive_is_viable(version, rnd, player, data, &self.team_plan, t, true, debug) { return None }   // L70
    bp = BattlePlan::new_dive(version, &TryKill(target_id, 60), data, player) // L73
    bp.entry_src = 2                                                         // L74
    tower = iter_towers_without_nexus(cache, 1-team).min_by_key(|tw| dist_sq(tw, t))   // L75 closure$1 (첫 최소)
    bp.dive_tower = tower.and_then(|tw| if tw.ty==Tower { Some(tw.info.ty) } else { None })   // L76~77 closure$
```

## `dbd260` → `fdb210` SmallActionRecall::get_input  (8891→9937B · Δ+1046)
- 명령 1894→2097 · 정렬 1764 · exe 판정 ⚠정렬 불가(미확정) · 정규화 6 · 블록이동 62 · 스택슬롯 358 · 콜리 주의 3
- 잔여 구조: 구 130명령(lea|rcx, [rbp + I]…) / 신 333명령(movaps|xmmword ptr [rbp + I], …) 짝 없음 ; dbd46d 피연산자 형 ; dbd927 피연산자 형 ; dbde5e 피연산자 수 ; dbe7d9 scale ; dbf1c3 피연산자 형
- 잔여 분기: dbd38e → dbd3e3/fdb396 ; dbd3d8 → dbd3dc/fdb398 ; dbd948 → dbd951/fdbd1f ; dbdc79 → dbdc20/fdbff0 ; dbe0bc → dbe159/fdc512
- 잔여 즉치: 0x908→0x928 · 0x690→0x6a0 · 0x6a0→0x6b0 · 0x690→0x6a0 · 0x6a0→0x6b0 · 0x908→0x928
- 잔여 변위: rax+0xac0→0x8
- 콜리 주의: ce9e90→e2a6b0 불일치(mig060 는 e266e0) ; d1ea70→e622c0 불일치(mig060 는 e61a90) ; d1f2a0→e61a90 불일치(mig060 는 e622c0)
- 정규화 흡수: 오프셋표 [r14+930→a00] · 오프셋표 [r14+9c0→a90] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [r15+1e0→3e8] · 오프셋표 [rcx+928→9f8]
- 0.5.8 명세 #184 `move_actions__SmallActionRecall_get_input` src game-ai\src\small_action\move_actions.rs:681 · one_line: 귀환 소액션의 틱 입력: 우물 안이면 None, 안전하고 우물까지 도보 시간(dist/move_speed)이 tps*3+60 틱 이상이면 Input::Return(그보다 가까우면 걸어서 간다), 아니면 우물 방향 7x7 셀 후보를 위험·거리로 채점해 goal 을 (재)확정하고 PathFinder(dodge_danger_cell) 로 Move 입력을 만든다.
- 0.5.8 logic(앞 1500자):
```
fn get_input(&mut self, version, rnd, player, data, ps, _debug) -> Option<Input>   // move_actions.rs:681
// ── 절1 L682~721: 즉시 판정 ──
team = player.info.team;  healp = team==0 ? (32000, 928000) : (928000, 32000)              // L682
champ = data.cache.player_champion[team][player.info.position].unwrap()                  // L688 (None→panic)
(lx,ly,rx,ry) = ctx.map.fountains[team]                                                    // L689
if lx<=champ.x<=rx && ly<=champ.y<=ry { return None }                                       // L691~692 이미 우물 안
direct_heal = version>=2 && distance(champ, healp) < 200001                                  // L700 (version<2: false, distance 호출 안 함)
if direct_heal { self.goal=(healp); self.goal_committed=true }                              // L702~704
(enemy_positions, visible_enemy_count) = collect_visible_enemy_positions(player,data)       // L707
enemy_knows = version>=2 && enemy_knows_my_position(player,data)                             // L710
eff_risk = |s:&PositioningScore| s.risk + (enemy_knows ? s.unseen_champ_threat : 0)          // closure#0 L711
dist_to_heal_area = distance(champ, healp)                                                   // L714
time = dist_to_heal_area / champ.move_speed                                                  // L716 (speed 0 → panic)
if is_safe_recall(version,rnd,player,data,ps) && time >= setting.tick_per_second*3 + 60 {   // L719 (is_safe_recall 먼저 평가 · ★도보 시간이 3초+60틱 **이상**일 때만 귀환 시전, 그 안쪽은 걸어서 간다 — m
```

## `e900b0` → `f3da50` get_input  (7884→9352B · Δ+1468)
- 명령 1587→1884 · 정렬 1431 · exe 판정 ⚠정렬 불가(미확정) · 정규화 22 · 블록이동 205 · 스택슬롯 333 · 콜리 주의 17
- 잔여 구조: 구 156명령(mov|rsi, rcx…) / 신 453명령(movaps|xmmword ptr [rbp + I], …) 짝 없음 ; e900d9 피연산자 형 ; e901d0 피연산자 형 ; e901fe 피연산자 형 ; e902a7 피연산자 형 ; e903a7 피연산자 형
- 잔여 분기: e90112 → e9013d/f3db03 ; e90138 → e91e64/f3fafe ; e90208 → e90224/f3dd67 ; e90222 → e9022a/f3dd6c ; e90287 → e90598/f3e145
- 잔여 소형 즉치: 0xe905ff:0xf1→0xf2 · 0xe90768:0x1→0x6 · 0xe91084:0x2→0x6 · 0xe9119d:0x4→0x2 · 0xe911b2:0x6→0x9
- 잔여 즉치: 0xffffffff→0x7d00 · 0x7d00→0xe2900 · 0xf1→0xf2 · 0x1→0x6 · 0x2→0x6 · 0xa→0x7d00 · 0x4→0x2 · 0x6→0x9 · 0x28b8→0x35b0 · 0x2860→0x3558
- 잔여 변위: rax+0x8→0xf8 · rdi+0x2940→0x3638 · rdi+0x2940→0x3638 · rdi+0x2858→0x3550 · rdi+0x2910→0x3608 · rdi+0x1d90→0x2a88 · rdi+0x2910→0x3608 · rdi+0x2910→0x3608 · rdi+0x2910→0x3608 · rdi+0x29ca→0x36c2
- 콜리 주의: 31a01a3→381e0b3 미지 ; 31a01a3→3821813 미지 ; 340e0→340e0 미지(pdata 밖 thunk) ; 6d4d0→6ed50 미지(pdata 밖 thunk) ; caeb40→ec1d20 미지(-5B) ; caf500→ec2660 미지(-4B)
- 정규화 흡수: 오프셋표 [r12+930→a00] · SmallActionPlay 태그 13→12 · SmallActionPlay 태그 7→6 · SmallActionPlay 마스크 103f3→81fb · SmallActionPlay 마스크 f40c→7a04 · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 13→12 · SmallActionPlay 태그 7→6
- 0.5.8 명세 #208 `AgentVerHamster__get_input` src game-ai\src\lib.rs:843 · one_line: AI 에이전트 루트(vtable Agent::get_input). 입력 지연 롤→update_state→small_action.get_input(최대 4회, 사이에 update_small_action/폴백/도주 강제)→진단 카운터·StayEvent(디버그)·freeze 감시 갱신 후 (Option<Input>, Vec<TurnEvent>) 반환
- 0.5.8 logic(앞 1500자):
```
// lib.rs:0~1004 (배치 M)
// 정본 = m14.ll 38800~41939 · 주석본 _next\reach\e900b0.ll · reach.py: 블록 255 전부 살아있음(사장 0) — NA 봉인 없음
// 시그니처: fn get_input(&mut self, rnd:&mut StdRng, player:&PlayerState, data:&OperationData) -> (Option<Input>, bumpalo::Vec<TurnEvent>)

// ── L844 입력 지연 게이트
let game = data.cache.game;                      // %54 data, %56 vtable
let tick = game.tick();                          // vtable+0x28 (호출 1/5)
if tick < self.next_input_tick {                 // self+0x2940 · icmp ult
  // L848  조기반환: (None, Vec::new_in(data.context.pool))
  sret+0 = -1(i64) ; sret+32 = 8(dangling ptr) ; sret+40 = context.pool ; sret+48..64 = 0 (cap,len)
  return;                                        // → %1330 ret(L1083)
}
// L851  입력 지연 롤 — 배치 M 유일의 rnd 직접 소비 사이트(1/1)
let p = &player.info.parameter;                  // player+0x180
let delay = rnd.gen_range(p.input_delay_min() ..= p.input_delay_max()) / 100;   // RangeInclusive<usize> · udiv 100
// L852
self.next_input_tick = game.tick() + delay;      // tick 재호출(2/5) · store self+0x2940
// L854
let mut turn_event: bumpalo::Vec<TurnEvent> = self.update_state(rnd, player, data);   // sret 32B %51 · &mut self 전체(계약: 배치 R/update_state 명세)
// L855
let _t_sai = ProfTimer::new(26);                 // prof::ENABLED(atomic i8)==0 → None(%50+16 = -1) / 아니면 Instant::now, %50 = {26, secs, nanos}
// L856  1차 소액션 입력
let mut input: Option<Input> = self.small_action.get_input(self.version, rnd, player, data, &self.positioning_score, &mut sel
```

## `e29b40` → `100c2f0` action_eval::evaluate_action  (3297→5277B · Δ+1980)
- 명령 719→1159 · 정렬 647 · exe 판정 ⚠정렬 불가(미확정) · 정규화 4 · 블록이동 36 · 스택슬롯 140 · 콜리 주의 2
- 잔여 구조: 구 72명령(mov|rdi, rcx…) / 신 512명령(mov|r13d, dword ptr [r15 + I]…) 짝 없음 ; e29c54 피연산자 형 ; e29c86 피연산자 형 ; e29e14 피연산자 형 ; e29e60 피연산자 형 ; e2a1eb 피연산자 형
- 잔여 분기: e29ba8 → e2a7a1/100d4c8 ; e29c52 → e29c61/100c418 ; e29d23 → e2a7a1/100d4c8 ; e29e30 → e29e93/100c622 ; e29ec7 → e29f64/100c6ee
- 잔여 즉치: 0x1318→0x1338 · 0x20→0x2 · -0x5c28f5c28f5c28f5→0x28f5c28f5c28f5c3 · 0x20→0x2 · 0xa18→0xa38 · 0x8→0xd8 · 0x8→0xd8 · 0x1318→0x1338
- 잔여 변위: r14+0x8→0x670 · r15+0x18→0x20 · r15+0x28→0x18
- 콜리 주의: 12857f0→1643790 변경 ; 31a01a3→381e0b3 미지
- 정규화 흡수: SmallActionPlay 태그 7→6 · SmallActionPlay 마스크 1f863→fc33 · 오프셋표 [r10+930→a00]
- 0.5.8 명세 #165 `action_eval__evaluate_action` src game-ai\src\plan_legacy\action_eval.rs:67 · one_line: v33 통합 행동 평가 — 라인 앵커 컨텍스트에서 Around 계열 소액션만 채점: positional_gain − danger + last_hit_gain 을 Some(score) 로, 그 외는 None(기존 경로 위임)
- 0.5.8 logic(앞 1500자):
```
L77: let Anchor::Lane{line} = ctx.anchor (태그 1) else → None
L78: if action ∉ {Around(2),AroundHide(3),AroundRegion(4),AroundPosition(7),AroundPositionBush(8),AroundBush(9),LaneMinionPosition(10)} → None   (RunAway·Recall·AroundRunAway·Positioning·Trace·Attack~Stop 은 기존 경로)
L81/L89: return Some(lane_positioning_score(version, player, data, parameter, action, line))

── lane_positioning_score (인라인 L98~130) ──
L98 : champ = player_champion[team][pos].unwrap()  (None 이면 panic)
L99 : purpose = line_phase_position_eval_purpose(player, data)  (i8 0..8)
L100: dest = action.evaluation_position(_, player, data)                     [Option<(x,y)> sret 24B]
L103: dest_score = dest.map(|(x,y)| position_eval_at(version, player, data, x, y, purpose))   [Option<PositioningScore> 56B · 니치 +0x31==2]
L104: hp_value = champion_hp_value(data, parameter, &parameter.player)
L105: hp = champ.hp.max(1)
L107~108: base_damage = dest_score.map(|s| hp * s.risk.max(0) / 100).unwrap_or(parameter.player.risk_damage)
L109~112: possible_damage = parameter.player.possible_risk(data, 9999) + dest_score.map(|s| hp * s.tower_risk.max(0) / 100).unwrap_or(parameter.player.risk_possible_tower) / 3
L113~116: danger = if champ.stat_buff_cached.undying { 0 } else { base_damage*hp_value/hp + possible_damage*hp_value/hp }   (sdiv 각각)
L124~125: positional_gain = dest_score.map(|s| (s.gain − s.gain_me).max(0) * hp_value / 200).unwrap_or(0)
L127: (ax,ay) = dest.unwrap_or((champ.x, champ.y))
L128: last_hit_gain = lane_anchor
```
