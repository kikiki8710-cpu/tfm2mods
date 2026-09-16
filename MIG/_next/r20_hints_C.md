# 배치 C — 변경 99 중 티어 2/3 14 (0.5.8 RVA → 0.6.0 RVA · exe 정규화 정렬 힌트 · 0.5.8 명세 요지)

정규화 정렬(`--norm`)이 이미 흡수한 것: PlayerState +0xd0 · Blackboard 0x2e8→0x5c8 · SmallActionPlay 태그/idx 재번호 · BattleSubPlanGoal 재번호 · Effect vt 슬롯(4 삽입) · LPH 오프셋 6. 아래 「잔여」 가 정렬로 못 닫은 차이.

## `d8eff0` → `1000320` calculate_score_parameter  (25381→25381B · Δ+0)
- 명령 5045→5045 · 정렬 5045 · exe 판정 ⚠정렬 불가(미확정) · 정규화 17 · 콜리 주의 7
- 잔여 구조: d9414c 피연산자 수
- 잔여 변위: rcx+0x78→0x1d0
- 콜리 주의: 12857f0→1643790 변경 ; 31a01a3→381e0b3 미지 ; d31bb0→d72be0 미지 ; d70900→e7c5b0 불일치(mig060 는 e7c1e0) ; d709f0→e7c6a0 불일치(mig060 는 e7c1e0) ; d70ae0→e7c790 불일치(mig060 는 e7c1e0)
- 정규화 흡수: 오프셋표 [r9+930→a00] · 오프셋표 [r9+9c0→a90] · 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 2e8→5c8
- 0.5.8 명세 #204 `score_parameter__calculate_score_parameter` src game-ai\src\score_parameter.rs:1461 · one_line: 선수 1명 관점의 ScoreParameter(5384B) 를 통째로 계산: 근처 미니언·아군·적(행동 포함)·타워 목록을 뽑고, 적의 현재 행동(평타/스킬/궁)과 비행 중 투사체를 대조해 나·아군·적 각각의 applyed/risk 피해·CC 를 누적한 뒤(배치 A~C), 타워·미니언·에픽 위험과 격자 좌표를 채워(배치 C~D) sret 으로 반환한다. 직접 TLS 접점 없음(콜리 precompute_champion_powers 의 CHAMP_POWERS_MEMO 를 간접 소비 · rnd/_debug 미사용). ★L2004~2100(자기 행동 확정피해·내 챔프 위협 적재)은 player.action 이 L1463 RunAway 고정이라 이 함수 단독 실행에선 도달 불가.
- 0.5.8 logic(앞 1500자):
```
// score_parameter.rs:0~1740 (배치 A)
// 승계: sret 5384B = ScoreParameter(tcxdict --deep) · 로컬 %47 에 조립 → 배치 D L2417 에서 sret 으로 memcpy.
// 콜리 계약(자식/경계 — 내부 미독): iter_minions(cache,team) -> Chain<Chain<Copied<Iter<&Entity>>,Copied<..>>,Copied<..>>(56B, 세 라인 미니언) / iter_towers_without_nexus(cache,team) -> Chain<Flatten<array::IntoIter<Option<&Entity>,6>>,Copied<Iter<&Entity>>>(120B) / precompute_champion_powers(version:usize, data:&OperationData, p:&mut ChampionScoreParameter 216B) — p.team(+0x60)·p.pos(+0x68) 읽고 +0xb8 attack_power·+0xc0 util_power_base·+0xc8 cc_time_x_inv_cd·+0xd0 buff_inv_cd_count 씀(m04.ll:53234, utils.rs:792) / Effect::expected_damage_target(&Effect 56B, ctx:&GameContext 64B, caster:&dyn(Entity ptr, vtable @anon.11 88B), target:&Entity 1728B) -> i64 / Projectile::expected_damage_target(&Projectile, ctx, caster:&Entity, target:&Entity) -> i64 / CastingTarget::check_projectile(&CastingTarget(applyed_target), &Projectile, &Entity) -> bool / Projectile::is_in_orbit(&Projectile, x, y, radius) -> bool / Projectile::has_cc(&Projectile) -> bool / Entity::remain_action_time(&Entity) -> usize / Blackboard::is_recent_visible(&Blackboard, game:&dyn AbstractGame, player:&PlayerState, e:&Entity) -> bool (_gcbc g07.ll:157005 — game.is_visible(player.team, e.id) || last_visible[e.pos]+120 >= game.tick(); 상수 120 은 game_core 본문이라 constants 에 미등록) / AbstractGame vtable +0x1f0 get_entity_by_id(id)->Option<&Entity> · +0x210 iter_projectile()->ProjectileIter(40B) / ProjectileIter::n
```

## `db6ff0` → `ec3060` SmallActionAround::get_input  (6900→6916B · Δ+16)
- 명령 1400→1396 · 정렬 1321 · exe 판정 ⚠정렬 불가(미확정) · 정규화 14 · 블록이동 386 · 스택슬롯 112 · 콜리 주의 2
- 잔여 구조: 구 79명령(mov|r14, qword ptr [r11 + I]…) / 신 75명령(mov|rdi, qword ptr [r11 + I]…) 짝 없음 ; db766f 피연산자 수 ; db83ad 피연산자 형 ; db83b0 피연산자 형 ; db8538 피연산자 형
- 잔여 분기: db734a → db747f/ec34ef ; db7367 → db747f/ec34ef ; db7442 → db747f/ec34ef ; db74e5 → db7572/ec35ea ; db7583 → db7ff4/ec43ac
- 잔여 즉치: 0xea5ff→0x22550 · 0xe2900→0x7d00
- 잔여 변위: r11+0x660→0x668 · r12+0x10→0x18 · rax+0x660→0x668
- 콜리 주의: 12857f0→1643790 변경 ; dc3240→fddcd0 변경
- 정규화 흡수: 오프셋표 [rdx+930→a00] · 오프셋표 [rdx+9c0→a90] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [r13+1e0→3e8] · 오프셋표 [rax+9c0→a90] · 오프셋표 [r13+1e0→3e8] · 오프셋표 [rax+9c0→a90]
- 0.5.8 명세 #179 `SmallActionAround__get_input` src game-ai\src\small_action\around.rs:61 · one_line: 엔티티(target, 보통 타워/오브젝트) 주변 range 안에서 가치 높은 셀을 goal 로 잡고 PathFinder 로 이동; escape_mode 면 적 반대편이면 분수로 goal 을 바꾸고, 타워 탈출이 필요하면 RunAway 로 위임
- 0.5.8 logic(앞 1500자):
```
fn get_input(&mut self, version, rnd, player, data, ps, debug /*미사용*/) -> Option<Input>   [around.rs:61~264]
// 63: target = data.cache.game.get_entity_by_id(self.target)?          (vtable +0x1f0 · null → None 92726)
// 65: champ = data.cache.player_champion[player.team][player.position.as_index()]?   (None → 92758)
// 67: skip_around = false
// 69: if self.escape_mode {
//   70-72: nearest_enemy = data.cache.iter_champions(1 - team)                                 (적 팀 5칸, 92884-93004 언롤)
//            .filter(|e| data.blackboard[1 - team].is_recent_visible(game, player, e))          (closure#0, 71)
//            .min_by_key(|e| e.distance_sq(champ))                                              (closure#1, 72 · 첫 원소 인라인 후 Map::fold 93083)
//   74: if let Some(enemy) = nearest_enemy {
//     75-79: to_tower = target.pos - champ.pos; away_enemy = champ.pos - enemy.pos; dot = to_tower.x*away_enemy.x + to_tower.y*away_enemy.y   (i64)
//     81: if dot <= 0 {                                                                        (icmp slt 1 — 적이 타워 반대편/직각)
//       82: f = map.fountains[team]  (lx,ly,rx,ry)
//       83-84: self.goal_x = (lx+rx)/2; self.goal_y = (ly+ry)/2;  skip_around = true          (lshr 1)
//     }
//   }
// }
// 90: dist = distance_sq(champ.pos, self.goal);  91: goal_to_target = distance_sq(self.goal, target.pos)
// 93: pve_hazard = PveHazardContext::new(version, player, data)
// 94: if !skip_around && (goal_to_target > 9215999999 /*≥96000²*/ || dist <= 25000
```

## `de5340` → `fb1b70` check_epic_hunt  (6556→6508B · Δ-48)
- 명령 1589→1587 · 정렬 1491 · exe 판정 ⚠정렬 불가(미확정) · 정규화 10 · 블록이동 73 · 스택슬롯 68
- 잔여 구조: 구 98명령(mov|rdi, qword ptr [r9 + I]…) / 신 96명령(mov|rax, qword ptr [r9 + I]…) 짝 없음 ; de55d4 피연산자 형 ; de55d4 피연산자 형 ; de624f 피연산자 형
- 잔여 분기: de535f → de5368/fb1ba0 ; de559f → de55ad/fb1dea ; de5a4e → de5f1f/fb3426 ; de5a83 → de5efb/fb26fe ; de5ac5 → de5b77/fb26f9
- 잔여 즉치: 0x118→0x128 · 0x118→0x128
- 잔여 변위: rcx+0x80→0xa0 · rsi+0x230→0x520 · rsi+0x238→0x528 · rsi+0x2d0→0x5c0 · r13+0x2f0→0x5e0 · r14+0x450→0x480
- 정규화 흡수: 오프셋표 [r14+930→a00] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 2e8→5c8
- 0.5.8 명세 #81 `epic__check_epic_hunt` src game-ai\src\plan_legacy\old\epic.rs:168 · one_line: 에픽(모가드) 사냥을 지금 해도 되는가 — 캠프 주변 아군/적 전력·도달 가능 적을 세어 판정
- 0.5.8 logic(앞 1500자):
```
ctx = data.context; if !spawn_epic(ctx.tutorial) (태그∈1..6) → return false                              [L169]
enemy_epic_tick = goal_data.epic.epic_enemy_tick                                                     [L176]
if !(enemy_epic_tick > tps) && !(team_plan.obj_spawn.epic_camp_last_visible_tick + tps < game.tick()) → return true   [L177-178]
   (= 적 에픽 틱이 tps 이하이고 캠프를 1초 안에 본 상태면 바로 true)
enemy_count = |{p: player_champion[1-team][p] 존재}|                                                   [L184]
live_ally  = |{아군 e: e.hp*100/e.max_hp > 29}|                                                        [L185]
if enemy_count < 2 && live_ally > 2 → return true                                                     [L186]
camp = map.camp_pos(Morgard, team==0)                                                                 [L191]
near_camp_epic_ally  = |{아군 e: dist²(e,camp) < 120000² && HP% > 49}|                                [L192-193]
near_camp_epic_enemy = |{적 e: dist²(e,camp) < 120000² && HP% > 49 && bb[1-team].is_recent_visible(game,player,e)}|  [L195-196]
near_epic_ally = |{아군 e: (is_top_side(ctx,e.x,e.y) [!(height−y <u x) · m09.ll:55697~55700: (height−y) <u x 참일 때만 is_near_mid_line 검사 = `A||B` 의 B 평가 조건 ⟹ A=is_top_side · 19차 C 정정(옛 문면 `!is_top_side`)] || is_near_mid_line(ctx,e.x,e.y)) && HP% > 49}|  [L198-199]
if any 아군 p: dist²(e,camp) < 250000² && bb[team].in_battle(p) == Some(Battle{focus}) && focus.is_some() → return false  [L202-204]
if team_plan.v24_objective_setup_lane_pre
```

## `ec8ba0` → `f87600` v3_epicops_defer_serpen  (1507→1579B · Δ+72)
- 명령 385→406 · 정렬 350 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 블록이동 72 · 스택슬롯 1 · 콜리 주의 1
- 잔여 구조: 구 35명령(mov|rsi, rdi…) / 신 56명령(mov|qword ptr [rsp + I], rdi…) 짝 없음
- 잔여 분기: ec8c19 → ec8d03/f87767 ; ec8c27 → ec8d03/f87767 ; ec8c87 → ec8ce9/f8774a ; ec8ca0 → ec8ce9/f8774a ; ec8cbb → ec8ce9/f8774a
- 잔여 변위: r9+0x18→0x10 · r9+0x20→0x18
- 콜리 주의: d666c0→1012f30 변경
- 정규화 흡수: 오프셋표 [r13+930→a00] · 오프셋표 2e8→5c8
- 0.5.8 명세 #90 `objective_helpers__v3_epicops_defer_serpen` src game-ai\src\plan_legacy\team_plan\objective_helpers.rs:275 · one_line: [v3 EPIC-OPS] 우리 에픽 버프 창 동안 세르펜 진입을 '미룰지' 판정 — 적이 세르펜을 치는 중이면 안 미룸, 압도적 우세(clear_win)면 안 미룸, 라인 미정리면 미룸, 그 외엔 세르펜 도달가능 적이 있을 때만 미룸
- 0.5.8 logic(앞 1500자):
```
fn v3_epicops_defer_serpen(version, player, data, goal_data, team_plan) -> bool
  team = player.info.team; enemy = 1 - team; tps = context.setting.tick_per_second
  // L276: 우리 팀 에픽 버프가 남아 있어야 판정 자체가 성립
  remain = game.get_game_mode()(vt+0x40) 가 Moba(tag 0)이면 mob.epic_minion_buff_time[team] (game.rs:210 remain_epic_time) 아니면 0
  if remain == 0 { return false }
  // L279: is_object_being_taken_by_enemy(player, data, goal_data, team_plan) (objective_helpers.rs:326~332, 인라인)
  //   L326: if !serpen_exists(context) → false  [tutorial 태그 ∉ {0,5,7,8}]
  //   L327: recently_visible = team_plan.obj_spawn.serpen_camp_last_visible_tick + tps >= game.tick()(vt+0x28)
  //   L328: enemy_kill_tick = goal_data.serpen.epic_enemy_tick
  //   L329: mob = game.as_moba().unwrap()  (Moba 가 아니면 unwrap 패닉 — 329:63)
  //   L330: serpen = mob.jungle_runner.serpen.live_list.get(0).and_then(|id| game.get_entity_by_id(*id)(vt+0x1f0))
  //   L331: damaged = serpen.is_some_and(|s| game.is_visible(team, s.id)(vt+0xf8) && s.hp < s.stat_cached.hp)
  //   L332: return recently_visible && damaged && enemy_kill_tick <= tps*20   [epic_enemy_tick 은 절대 틱이 아니라 소요 틱 — SerpenStanceData::update_plan(goal_data.rs:462) 이 check_epic_kill_time_with_hp(…, serpen_hp) = hp*1000 / max(Σ_챔피언 expected_damage*1000/attack_cooltime, 1) 로 계산한 「적 챔피언들이 세르펜을 잡는 데 걸리는 예상 틱」. 그래서 현재 tick 과 빼지 않고 tps*20 과 직접 비교 = 20초 안에 잡을 수 있으면]
  if being_taken { return false }   // 적이 지금 세르펜을 치고 있으면 미루지 않는다
  // L283~284: 적에게 밀 수 있는 타워가 하나라도 있어야 함
  
```

## `e8e560` → `f3bde0` AgentVerHamster::update_small_action  (4430→4530B · Δ+100)
- 명령 922→933 · 정렬 914 · exe 판정 ⚠정렬 불가(미확정) · 정규화 16 · 블록이동 56 · 콜리 주의 4
- 잔여 구조: 구 8명령(movdqa|xmm6, xmmword ptr [r12 …) / 신 19명령(movdqu|xmm6, xmmword ptr [r12 …) 짝 없음 ; e8ed5a 피연산자 형 ; e8edd0 피연산자 형
- 잔여 분기: e8e7e2 점프테이블 75/80 항목 불일치 ; e8e808 → e8e80f/f3c3bc ; e8e832 → e8e8ab/f3c0d0 ; e8e860 → e8e8ab/f3c3c1 ; e8e89f → e8e8ab/f3c0d0
- 잔여 소형 즉치: 0xe8e7fc:0x2→0x3 · 0xe8e80a:0xa→0x3 · 0xe8e83e:0x4→0x2 · 0xe8e84f:0x6→0x9 · 0xe8e86c:0x9→0x1 · 0xe8e87d:0x1→0x6 · 0xe8e8ab:0x3→0xa · 0xe8ea87:0x1→0x6 · 0xe8eb60:0x3→0x4 · 0xe8eb75:0x3→0x1
- 잔여 즉치: 0x2→0x3 · 0xa→0x3 · 0x4→0x2 · 0x6→0x9 · 0x9→0x1 · 0x1→0x6 · 0x3→0xa · 0x1→0x6 · 0x3→0x4 · 0x3→0x1
- 잔여 변위: rcx+0x1d58→0x2a38 · rax+0x1d50→0x2a30 · r12+0x1d48→0x2a28 · r12+0x2909→0x3601 · r12+0x28ea→0x35e2 · r12+0x28b8→0x35b0 · r12+0x1d58→0x2a38 · r12+0x1d50→0x2a30 · r12+0x1d58→0x2a38 · r12+0x1d48→0x2a28
- 콜리 주의: 31a01a3→381e0b3 미지 ; e23450→e5d7d0 미지(-81B) ; e444c0→d33900 미지 ; e65b10→d6c2b0 변경
- 정규화 흡수: SmallActionPlay 태그 e→d · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 9→8 · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 e→d
- 0.5.8 명세 #212 `AgentVerHamster__update_small_action` src game-ai\src\lib.rs:693 · one_line: 소액션 재평가: failed_action 쿨다운 정리·실패 기록(포기 Trace/무진전 도주·귀환/미발동 캐스트 교체/Trace 이탈) → plan_system.get_small_action 경매 → small_action.merge → last_eval_* 스냅샷 갱신
- 0.5.8 logic(앞 1500자):
```
fn update_small_action(&mut self, rnd, player, data)   // lib.rs:693~794
  game = data.cache.game (dyn AbstractGame · tick()=vtable+0x28 · get_game_mode()=vtable+0x40)
  // ── 1. 실패 기록 정리 (L695 · aux m06.ll retain)
  self.failed_action.retain(|(t, _)| !(t + 60 < game.tick()))      // 60틱(1초) 지난 실패는 제거
  // ── 2. 실패 기록 A: 포기된 추격 (L699~705)
  if let Trace(t) = &self.small_action && t.abandoned (+0x92):
    act = SmallAction::Trace{ target_id: t.target (+0x60) }         // discr 4
    if let Some(x) = failed_action.iter_mut().find(|(_,a)| a == act): x.0 = game.tick()   // L703
    else: failed_action.push((game.tick(), act))                     // L705
  // ── 3. 실패 기록 B: 무진전 도주/귀환 (L713~723)
  stalled = match &self.small_action {
    RunAway(a) => a.prog_best_tick != 0 && game.tick().saturating_sub(a.prog_best_tick) > 119   // L714 (+0x30)
    Recall(a)  => a.prog_best_tick != 0 && game.tick().saturating_sub(a.prog_best_tick) > 119   // L715 (+0x78)
    _ => false }
  if stalled:
    act = self.small_action.to_small_action()    // L719 · 이 문맥에선 항상 SmallAction::RunAway(discr 0 · 페이로드 없음)
    find(a == act) ? x.0 = tick (L721) : push((tick, act)) (L723)
  // ── 4. 경매 (L728~730)
  self.small_debug = DebugFrameData::default()
  (score_parameter, next_score, next_action) = self.plan_system.get_small_action(self.version, rnd, player, data, &self.small_action, &self.failed_action, &mut self.small_debug)
  // ── 5. DM 계측 (L734~752 · 관측 전용)
  if game.get_game_mode() is DeathMatch(tag 2)
```

## `e95ae0` → `f43b10` DefenseNexusSubPlan::score  (1537→1413B · Δ-124)
- 명령 353→334 · 정렬 281 · exe 판정 ⚠정렬 불가(미확정) · 정규화 4 · 블록이동 17 · 스택슬롯 23 · 패닉스텁 재배열 2 · 콜리 주의 2
- 잔여 구조: 구 72명령(mov|r15, r9…) / 신 53명령(lea|rbp, [rsp + I]…) 짝 없음 ; e95be3 피연산자 형 ; e95c14 피연산자 형 ; e95e49 피연산자 형
- 잔여 분기: e95d69 → e95e6b/f43e28 ; e95dc7 → e95e06/f43ffc ; e95e5f → e95e68/f43e88 ; e95f62 → e96023/f43ff6 ; e9603f → e96046/f43ff8
- 잔여 즉치: 0xa8→0xb8 · 0xa8→0xb8
- 잔여 변위: rbx+-0xf→-0xe · rsi+0x8→0x668
- 콜리 주의: d57540→f72620 변경 ; d59940→f749a0 변경
- 정규화 흡수: 오프셋표 [r9+9c0→a90] · SmallActionPlay 태그 7→6 · 오프셋표 2e8→5c8 · SmallActionPlay 태그 7→6
- 0.5.8 명세 #223 `defense_nexus__DefenseNexus__score` src game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:311 · one_line: 넥서스 방어 서브플랜 액션 점수: interaction_score 기저 + (v≥2·공격류 대상이 본진구조물 때리는 미니언이면 +100) + 액션별 가산(Attack/Skill/Skill2=calculate_action_score(Push) · 대상 없으면 -99999 / Around·AroundHide·LaneMinionPosition=대상이 적 미니언이고 넥서스 최근접 전방미니언이면 +5)
- 0.5.8 logic(앞 1500자):
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   [defense_nexus.rs:311]
  team = player.info.team (bounds <2)                                                         [L312 · m14.ll:49000~49006]
  champ = data.cache.player_champion[team][player.info.position].unwrap()                     [L312 · 49011~49020 · None→unwrap_failed 49037]
  base = action_score::interaction_score(version, rnd, player, data, parameter, action, debug)   [L313 · 49029]
  if version > 1 {                                                                            [L318 · 49031 ugt · reach: 항상 true]
    // L319: 공격류 4종만 target 꺼냄 (tag 15..18 = Attack/Skill/Skill2/Ult)
    target_is_base_attacker = match action { Attack(a)|Skill(a)|Skill2(a)|Ult(a) => Some(a.target), _ => None }   [49042~49046, 49081~49082]
        .and_then(|id| data.cache.game.get_entity_by_id(id))        // closure#0 L320 · vtable+0x1f0 · 49084~49097
        .is_some_and(|e| old::is_base_attacking_minion(player, data, e))   // closure#1 L321 · 49106
    if target_is_base_attacker { base += 100 }                                                [L322 · 49108~49109 select]
  }
  idx = 논리 variant idx(action 태그 +0xb1; tag>2 ? tag-3 : 7)                              [L329 · 49052~49075 switch]
  extra = match action {
    Attack(12)  => match game.get_entity_by_id(a.target) {                                    [L334 · 49128~49138]
                     None => -99999,
                     Some(t) => calculate_acti
```

## `dd26e0` → `f09860` TeamPlan::v25_objective_posture  (10749→10920B · Δ+171)
- 명령 2477→2515 · 정렬 2301 · exe 판정 ⚠정렬 불가(미확정) · 정규화 13 · 블록이동 186 · 스택슬롯 206
- 잔여 구조: 구 176명령(movzx|r10d, byte ptr [rsp + I]…) / 신 214명령(cmp|byte ptr [rdx + I], I…) 짝 없음 ; dd30ee 피연산자 형 ; dd30f3 피연산자 형 ; dd30f3 피연산자 형 ; dd3230 피연산자 형 ; dd3e79 피연산자 형
- 잔여 분기: dd2716 → dd272b/f09bb5 ; dd2724 → dd2733/f09e6b ; dd2726 → dd5018/f09ab7 ; dd2835 → dd28c5/f09884 ; dd2968 → dd29ad/f09bad
- 잔여 소형 즉치: 0xdd2712:0x4→0x5
- 잔여 즉치: 0x4→0x5 · 0x490404401→0x78b30c401 · 0x78b30c401→0x490404401
- 잔여 변위: rdx+0x41f→0xcd5 · rdx+0x420→0xcd6 · rax+0x10→0x8 · r12+0x230→0x520 · r12+0x238→0x528 · r12+0x2d0→0x5c0 · r12+0x240→0x530 · r12+0x248→0x538 · r12+0x2d8→0x5c8 · r12+0x250→0x540
- 정규화 흡수: SmallActionPlay idx 5→4 · 오프셋표 [r9+930→a00] · 오프셋표 [r9+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rdi+1e0→3e8] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rdi+1e0→3e8] · 오프셋표 [rax+9c0→a90]
- 0.5.8 명세 #106 `objective_discipline__TeamPlan_v25_objective_posture` src game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 · one_line: 에픽/세르펜 오브젝트 태세 판정 — 근접/캠프 아군·적 수, 적 압력, 체력창을 재서 Commit/HoldCamp/Screen/WaitGroup/SoftDisengage 중 하나(또는 None)를 낸다
- 0.5.8 logic(앞 1500자):
```
// objective_discipline.rs:207~214 — 진입 게이트
// self.objective(+0x41f 태그, +0x420 phase) 를 target 과 대조
match (target, self.objective) {
  (Morgard(4), Some(Morgard{phase,..}(tag0))) | (Serpen(5), Some(Serpen{phase,..}(tag1))) => phase = *(self+0x420)   // L210~212
  _ => return None                                                                                      // L213 (sret+0 = -1)
}

// L216 — 내 챔프
team = player.info.team (bounds <2, 아니면 panic_bounds_check)
champ = data.cache.player_champion[team][player.info.position as usize]?   // null 이면 return None (from_residual)

// L217~220 — 좌표
camp_pos = map.camp_pos(target, team == 0)                       // MapDef::camp_pos(&self, JungleType, bool) — bool 은 (team==0)
wait_pos = if target == Morgard(4) { map.camp_pos(Stump(2), team==0) }   // L219
           else                    { map.camp_pos(Rhino(0), team==0) }   // L220

// L224~228 — 오브젝트 엔티티 (GameMode::as_moba 인라인 game.rs:231, Moba 가 아니면 unwrap_failed 패닉)
moba = game.get_game_mode()[vtable+0x40].as_moba().unwrap()
objective_entity = if target == Morgard { moba.jungle_runner.epic.live_list.get(0) }     // L225  (+0x1a8 len, +0x1a0 ptr)
                   else                 { moba.jungle_runner.serpen.live_list.get(0) }   // L227  (+0x1d8 len, +0x1d0 ptr)
                   .and_then(|id| game.get_entity_by_id(*id)[vtable+0x1f0])            // closure#0 L226 / closure#1 L228 — null=None

// L231~232, L283
object_hp_ratio = objective_entity.map_or(100, |t| t.hp*100 / 
```

## `dccc60` → `ff11f0` GoalData::update  (2888→3144B · Δ+256)
- 명령 620→676 · 정렬 603 · exe 판정 ⚠정렬 불가(미확정) · 정규화 8 · 블록이동 65 · 스택슬롯 49 · 콜리 주의 5
- 잔여 구조: 구 17명령(mov|rax, qword ptr [rcx + rax*…) / 신 73명령(mov|r12, qword ptr [rcx + rax*…) 짝 없음 ; dccd35 피연산자 형 ; dccd35 피연산자 형 ; dccdd2 피연산자 형 ; dccee2 피연산자 형 ; dccf52 scale
- 잔여 분기: dcccd9 → dccda4/ff1380 ; dcce06 → dcceff/ff1598 ; dcce75 → dcce87/ff1d10 ; dccef9 → dcd035/ff1380 ; dccf8f → dccf98/ff1593
- 잔여 즉치: 0x9e0→0xab0
- 잔여 변위: r12+0x9c8→0xa98
- 콜리 주의: 2a300→2a300 미지(pdata 밖 thunk) ; dd73b0→ff2450 변경 ; de1ee0→ff4940 변경 ; decf00→101b7f0 미지(pdata 밖 thunk) ; decf30→101bb80 미지(pdata 밖 thunk)
- 정규화 흡수: 오프셋표 [rbx+930→a00] · 오프셋표 [rbx+9c0→a90] · 오프셋표 [r10+9c0→a90] · 오프셋표 [rbx+930→a00] · 오프셋표 [r12+930→a00] · 오프셋표 [r12+9c0→a90] · 오프셋표 [rbx+930→a00] · 오프셋표 [rbx+9c0→a90]
- 0.5.8 명세 #74 `goal_data__GoalData_update` src game-ai\src\goal_data.rs:82 · one_line: 매 평가마다 GoalData 상태 갱신: 힐 커밋(v2+) · 본진수비 틱 · 적 5명 리전 추적(5초 만료) · 에픽/세르펜 태세 갱신
- 0.5.8 logic(앞 1500자):
```
fn update(&mut self, version, _rnd, player, data, debug):
  // ---- (A) update_heal_commit (goal_data.rs:27~57, 인라인) ----
  if version >= 2 {                                   // :28  version<2 → 이 블록 전체 스킵
    champ = cache.player_champion[player.team][player.pos]   // :31 (team>=2 → bounds panic)
    if champ == None { self.heal_commit = false; }   // :32~33
    else {
      before = self.heal_commit                       // :35
      if nexus_final_stand(player,data)      { new = false }   // :36
      else if nexus_is_critical(player,data) { new = false }   // :37
      else if before {                                          // :39
        if champ.hp < champ.stat_cached.hp { (store 없음 = true 유지) } else { new = false }   // :40 풀피 도달 → 해제
      } else {                                                  // :44
        (lx,ly,rx,ry) = map.fountains[player.team]
        in_heal_area = lx<=champ.x<=rx && ly<=champ.y<=ry      // :45
        hp_ratio = champ.hp*100 / max(champ.stat_cached.hp,1)  // :46
        if !in_heal_area && hp_ratio < 36 && base_defense_focus(player,data) { new = true }   // :47 ★우물 밖에서만(IR %80/%89 분기) (&& 단락: base_defense_focus 는 앞 둘 통과 시만 호출)
        else (store 없음 = false 유지)
      }
      if store 됐고 before != new && ctx.debug { debug.add_log(format!("HEALCOMMIT T{team} {pos:?} {start|end} hp={hp*100/max_hp}%")) }  // :51~55
    }
  }
  // ---- (B) 본진수비 틱 ----
  if base_defense_focus(player,data) { self.last_base_defense_tick = tick() }   // :84~85
 
```

## `e2a830` → `eb3470` lane_minion_position_action  (911→1225B · Δ+314)
- 명령 181→236 · 정렬 147 · exe 판정 ⚠정렬 불가(미확정) · 정규화 3 · 블록이동 16 · 스택슬롯 61 · 콜리 주의 2
- 잔여 구조: 구 34명령(mov|rsi, qword ptr [rbp + I]…) / 신 89명령(mov|rsi, rcx…) 짝 없음
- 잔여 분기: e2a9a6 → e2aa5a/eb3908 ; e2aa3a → e2aa5a/eb3908 ; e2aa58 → e2aa6b/eb372c
- 잔여 즉치: 0x208→0x1d8 · 0x208→0x1d8
- 잔여 변위: r15+0x90→0x68
- 콜리 주의: 31a01a3→e6bf60 미지 ; d6fc40→e6bd90 미지(-737B)
- 정규화 흡수: 오프셋표 [rsi+9c0→a90] · 오프셋표 [rsi+930→a00] · SmallActionPlay 태그 d→c
- 0.5.8 명세 #161 `lane_minion__lane_minion_position_action` src game-ai\src\small_action\lane_minion.rs:20 · one_line: 서포터 제외, 팔로우 사거리 안 적 라인 미니언 중 target_score 최대를 골라 LaneMinionPosition 소액션(목표는 choose_goal)을 만든다
- 0.5.8 logic(앞 1500자):
```
fn lane_minion_position_action(version, rnd:&mut StdRng, data, player, line, positioning_score, wave_snapshot, end_delay, position_eval_purpose) -> Option<SmallActionPlay>
[L23] if player.info.position == Support(4) { return None }              ; 54278
[L27] champ = data.cache.player_champion[player.info.team][player.info.position]?   ; null → None(54319)
[L28] atk = champ.attack_effect.as_ref()?                                 ; 태그 0x4c0 == -1 → None(54438)
[L30] (target, target_score) = data.cache.iter_minions(1 - player.info.team)   ; 적 미니언 체인 이터레이터(56B)
  [L32~45] .filter(|target| closure$0):                                   ; aux 40688~40855
     L32: champ.team==Player(t) ⇒ target.visible_state[t]==Visible 아니면 false (Neutral 이면 통과)
     L35: target.ty is Minion(1) 아니면 false
     L38: target.ty.Minion.info.line == line 아니면 false
     L42: follow_range = atk.range + 64000 + champ.stat_buff_cached.range + (champ.level-1)*atk.growth_range + atk.range_adjust(champ,target) + Entity::radius(champ)
     L43: follow_range += Entity::radius(target)
     L44: dist_sq(target,champ) <= follow_range^2
  [L46] .filter_map(|target| Some((target, SmallActionLaneMinionPosition::target_score(version, data, player, champ, atk, target, wave_snapshot, rnd).1)))   ; 항상 Some(ptr 니치 non-null · 55505~55507). version·player 는 콜리가 안 읽어 poison
  [L47] .max_by_key(|(_, score)| *score)                                     ; 동점이면 뒤 원소(compare>0 일 때만 앞 유지 · 15117~15118)
  ?                             
```

## `d2e500` → `fa3980` PassiveJunglePlan::sub_plan  (2993→3411B · Δ+418)
- 명령 699→832 · 정렬 475 · exe 판정 ⚠정렬 불가(미확정) · 정규화 1 · 블록이동 306 · 스택슬롯 49 · 패닉스텁 재배열 1 · 콜리 주의 6
- 잔여 구조: 구 224명령(mov|rax, qword ptr [r8]…) / 신 357명령(mov|rsi, rcx…) 짝 없음 ; d2e91b 피연산자 형 ; d2e96e 피연산자 형 ; d2e977 피연산자 형 ; d2ec26 피연산자 형 ; d2eca1 피연산자 형
- 잔여 분기: d2e58f → d2e5d1/fa3a67 ; d2e599 → d2e5d1/fa3a67 ; d2e5b0 → d2e5d1/fa3a67 ; d2e5b6 → d2e5d1/fa4361 ; d2e5cb → d2ed85/fa40e1
- 잔여 소형 즉치: 0xd2ed4d:0x5→0x3 · 0xd2f00d:0x5→0x1
- 잔여 즉치: 0x1c8→0xf8 · 0x6d70→0x120 · 0x5→0x3 · 0x29→0x1 · 0x1c8→0xf8 · 0x120→0xf0 · 0x60→0x30 · 0x150→0x120 · 0x90→0x60 · 0xf0→0x150
- 잔여 변위: rdx+0x48→0x8 · rcx+0x668→0x8 · rcx+0x5c0→0x12f8 · r14+0x8→0x660
- 콜리 주의: 12857f0→1643790 변경 ; 12857f0→381e0b3 불일치(mig060 는 1643790) ; 31a01a3→381e0b3 미지 ; 31a01a3→fb12a0 미지 ; 31a0cbf→38219f0 미지(+18B) ; 31a3b40→381ebcf 미지(-3B)
- 정규화 흡수: 오프셋표 [r9+930→a00]
- 0.5.8 명세 #91 `passive_jungle__PassiveJunglePlan_sub_plan` src game-ai\src\plan_legacy\old\passive_jungle.rs:134 · one_line: 패시브 정글 플랜의 서브플랜 — check_recall(인라인)이 참이면 Recall, 아니면 Jungle{team,camp,check_move:false}
- 0.5.8 logic(앞 1500자):
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
  L176~178: || champ.skil
```

## `d851d0` → `ff62e0` position_eval_at_uncached  (30055→30607B · Δ+552)
- 명령 6071→6208 · 정렬 5768 · exe 판정 ⚠정렬 불가(미확정) · 정규화 20 · 블록이동 1058 · 스택슬롯 1245 · 패닉스텁 재배열 5 · 콜리 주의 4
- 잔여 구조: 구 303명령(mov|rax, qword ptr [r11]…) / 신 440명령(mov|qword ptr [rbp + I], rax…) 짝 없음 ; d851f6 피연산자 형 ; d86302 피연산자 수 ; d86ffe 피연산자 형 ; d87010 피연산자 형 ; d872cb 피연산자 형
- 잔여 분기: d85300 → d853aa/ff64b4 ; d853a5 → d8c52d/ffd85a ; d85801 → d85b7b/ff6c6b ; d858d6 → d85b7b/ff6c6b ; d859a8 → d85b7b/ff6c6b
- 잔여 소형 즉치: 0xd871f5:0x6→0x1 · 0xd8a8ee:0x2→0x3 · 0xd8a9c2:0x2→0x3 · 0xd8af4e:0x1→0x9 · 0xd8b607:0x0→0x4
- 잔여 즉치: 0x7b8→0x7a8 · 0xfa0→0x1090 · 0x320→0x350 · 0xfa0→0x1090 · 0x320→0x350 · 0xfa0→0x1090 · 0x320→0x350 · 0x6→0x1 · 0x0→0x96 · 0x2→0x3
- 잔여 변위: rsi+0x1a8→0x1b0 · rsi+0x1b0→0x1a8 · r15+0x20→0x1b0 · rcx+0xe8→0x108 · rcx+0xe8→0x108 · rsi+0x660→0x668 · rdx+0xe8→0x108 · rcx+0xe8→0x108 · rcx+0xe8→0x108 · r14+0x5c0→0x28
- 콜리 주의: 12857f0→1643790 변경 ; 31a01a3→381e0b3 미지 ; c7ebc0→da7370 변경 ; d96d00→1008030 변경
- 정규화 흡수: 오프셋표 [r12+930→a00] · 오프셋표 [r12+9c0→a90] · 오프셋표 [r12+930→a00] · 오프셋표 [rdi+9c0→a90] · SmallActionPlay 태그 0→1 · SmallActionPlay 태그 1→0 · 오프셋표 [rdi+930→a00] · 오프셋표 [rdi+9c0→a90]
- 0.5.8 명세 #180 `position_eval__position_eval_at_uncached` src game-ai\src\position_eval.rs:371 · one_line: 셀 (x,y) 에 내 챔피언이 서 있을 때의 위험/이득 56B PositioningScore 를 계산 — 벽 셀=9999 즉시 반환, 적 우물 위험 셀=risk·tower_risk 9999 에서 시작해 계속 누적(우물 사각 안이면 +well_damage HP% 까지: 오라클 hp=1 → 10149), 정글·에픽 몹 기대피해(HP% ×1/½/⅓ 가중 — ×2 없음)·타워(사거리·미니언 수)·미니언 웨이브·투사체 궤도·적 챔피언 위협(가시/비가시)·아군 교전 이득·purpose 별 보정을 순서대로 누적. POS_EVAL_CACHE miss 시에만 호출자 position_eval_at 이 부른다.
- 0.5.8 logic(앞 1500자):
```
// position_eval.rs:0~507 (배치 A)
// 시그니처: (sret score, version, player, data, x, y, purpose) — 호출자 position_eval_at(293~324)가 POS_EVAL_CACHE miss 때만 부름(tls 절).
373: _t = ProfTimer::start(48)   // prof::ENABLED 가 0 이면 Instant::now 생략(phase 48, nanos 슬롯 PHASE_NANOS+384)
374: champ = data.cache.player_champion[player.info.team][player.info.position.as_index()]   // team<2 bounds 패닉 가드
375: if champ is None → return PositioningScore::default()   // memset(%0, 0, 50)
378: xi = clamp(x/32000, 0, 29);  379: yi = clamp(y/32000, 0, 29)
380: if data.context.map.walls[yi][xi] != 0 →
381:     return PositioningScore{ risk: 9999, ..default }   // store 9999 @0 + memset(@8, 0, 42)
384: inv_hp_q32 = 2^32 / max(champ.hp, 1)
390: score = default (SSA: 8필드 전부 0)   // 400: tower_well_risk = 0 (초기)
393: enemy_team = 1 - team
394: visible = game.is_visible_cell(enemy_team, xi, yi)   // vtable+0x100 · 배치 A 범위에선 미사용(뒤 배치 소비)
395: cell_dist_sq = |ex, ey| axis_distance_sq(x, ex).saturating_add(axis_distance_sq(y, ey))   // closure#0, axis_distance_sq(score_parameter.rs:8~9) = abs_diff(a,b) 의 saturating 제곱
397: tower_well_risk = if path_finder::is_enemy_well_danger(version, player, x, y) { 9999 } else { 0 }
398: score.risk = tower_well_risk;  399: score.tower_risk = tower_well_risk
403: champ_cache = &cache.player_champion_cache[team][pos]
405: (enemy_mask, ally_mask) = pe_cand_masks(seed=game.seed(), tick=game.tick(), team, cache, data.blackboard, enemy_team, player)   // PE_CAND_MASKS TLS · bit i = 
```

## `d28800` → `ec7180` PassiveLinePlan::update  (14751→15601B · Δ+850)
- 명령 3089→3282 · 정렬 2735 · exe 판정 ⚠정렬 불가(미확정) · 정규화 19 · 블록이동 359 · 스택슬롯 579 · 패닉스텁 재배열 5 · 콜리 주의 10
- 잔여 구조: 구 354명령(mov|r14, qword ptr [rdx]…) / 신 547명령(mov|rcx, qword ptr [rdx]…) 짝 없음 ; d28bef 피연산자 형 ; d28f65 피연산자 형 ; d2933f 피연산자 형 ; d293ba 피연산자 형 ; d29665 피연산자 수
- 잔여 분기: d288ed → d28949/ec72dd ; d288f7 → d28949/ec72dd ; d28917 → d28949/ec72dd ; d2891d → d28949/ec72dd ; d2892f → d29efc/ec8994
- 잔여 즉치: 0x348→0x338 · 0x1e0→0x28 · 0x28→0x130 · 0x1f→0x1 · 0x64→0x18 · 0x348→0x338
- 잔여 변위: r15+0x116→0x11b · rax+0x660→0x668 · r15+0x116→0x11b · rcx+0x10→0x628 · r12+0x670→0x628 · rcx+0xa0→0xc0 · r12+0x670→0x628 · rcx+0xa0→0xc0 · rax+0x20→0x68 · rcx+0xa0→0xc0
- 콜리 주의: 12857f0→1643790 변경 ; 31a3863→3821af0 미지(-32B) ; 31a3b40→3821813 미지(+32B) ; 6d4d0→6ed50 미지(pdata 밖 thunk) ; d26900→ec4c20 변경 ; d31bb0→d72be0 미지
- 정규화 흡수: 오프셋표 [r9+930→a00] · 오프셋표 [r9+9c0→a90] · 오프셋표 [r9+930→a00] · 오프셋표 [r9+9c0→a90] · SmallActionPlay idx 5→4 · SmallActionPlay 태그 1→2 · SmallActionPlay 태그 1→2 · 오프셋표 2e8→5c8
- 0.5.8 명세 #206 `passive_line__PassiveLinePlan__update` src game-ai\src\plan_legacy\old\passive_line.rs:219 · one_line: 라인 패시브 플랜 매 틱 갱신 — L222 check_recall 로 self.in_recall(귀환 여부) 결정(배치 H) → 팀 목표 Gank 라인 보정·v46 라인귀환 stage1/2·도주 게이트(update_v46_lane_recall/update_v46_flee 인라인, L232~248 · 배치 I) → 라인 위치·채팅 push(L254~304 · 배치 J). 반환 없음(unit) — 출력은 &mut self 필드뿐.
- 0.5.8 logic(앞 1500자):
```
// passive_line.rs:0~222 (배치 H)
// 루트 L222 = `self.in_recall = self.check_recall(version, rnd, player, data, debug)` 가 통째로 인라인(passive_line.rs:1068~1449, 893 IR줄). 루트 0 으로 잡힌 2줄(24144~24145, team_plan+0x420 로드)은 줄번호 없는 DILocation 이고 배치 J 의 L260 블록 소속 → J 가 다룬다.
// 함수 프롤로그 19039~19104 = 지역 alloca 54개(bumpalo Vec 헤더 32B ×2 = near_allies %58 / near_enemies %56, BuffState 288B ×4 %45~%48, 포맷 인자 등). check_recall 결과(i8)는 21201 에서 self+0x110 에 store → 블록 %904 나머지(L232~)는 배치 I.

// ---- check_recall(&mut self, version, rnd, player, data, debug) -> bool  [passive_line.rs:1068~1449] ----
// L1069  team = player.info.team (+0x930); team<2 bounds(19167). champ = data.cache.player_champion[team][player.info.position.as_index()].unwrap() (19172~19183, 실패 19218)
// L1071  (lx,ly,rx,ry) = ctx.map.fountains[team] (map_def.rs:235, +0x6d70+team*32)
// L1072  in_fountain = lx<=champ.x<=rx && ly<=champ.y<=ry (19212~19215, 19230~19234)
// L1073  if in_fountain && champ.hp(+0x670) < champ.stat_cached.hp(+0x628): return true            (19240~19246 → %904 phi=1)
// L1079  else if in_fountain && version > 1: return false                                            (19237~19238 → %904 phi=0)  ★reach: version=2 로 항상 참 ⟹ 아래 %112(in_fountain=true) 경로는 NA(version<2 전용)
// L1084  hp_ratio = hp*100 / max_hp  (max_hp==0 → panic_const_div_by_zero 19291)                    (19252~19262, %62 에 store — 포맷 인자로도 씀)
// L1085  can_buy_item = buy_item(version, rnd, player, cache.game.{data,vtable}, ctx) -> {i64,i64}, 
```

## `de81b0` → `fb49b0` check_epic_giveup  (4216→5523B · Δ+1307)
- 명령 1087→1258 · 정렬 762 · exe 판정 ⚠정렬 불가(미확정) · 정규화 20 · 블록이동 151 · 스택슬롯 93
- 잔여 구조: 구 325명령(mov|r13, qword ptr [r15]…) / 신 496명령(mov|qword ptr [rbp + I], rcx…) 짝 없음 ; de8265 피연산자 형 ; de82ce 피연산자 형 ; de86de 피연산자 형 ; de8857 피연산자 형 ; de8980 피연산자 형
- 잔여 분기: de822c → de82e2/fb4bdf ; de8239 → de82e2/fb4a74 ; de8255 → de82e2/fb4bdf ; de8275 → de82e2/fb4bdf ; de82a0 → de82a6/fb4af9
- 잔여 소형 즉치: 0xde82a6:0x14→0x15 · 0xde8cc3:0x31→0x32 · 0xde8dba:0x32→0x31
- 잔여 즉치: 0xc8→0x1a8 · 0x14→0x15 · 0x31→0x32 · 0x32→0x31 · 0xc8→0x1a8
- 잔여 변위: rax+0x41f→0x3e4 · rax+0x420→0x404 · r13+0x230→0x520 · r13+0x238→0x528 · r13+0x2d0→0x5c0 · rax+0x28→0x20 · r13+0x2d8→0x5c8 · r13+0x150→0x28 · r13+0x2e0→0x5d0 · r13+0x150→0x28
- 정규화 흡수: SmallActionPlay 태그 0→2 · BattleSubPlanGoal 3→2 · 오프셋표 [r14+930→a00] · 오프셋표 [r14+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8]
- 0.5.8 명세 #50 `epic__check_epic_giveup` src game-ai\src\plan_legacy\old\epic.rs:109 · one_line: Morgard(에픽) 목표를 포기할지 — 생존 머릿수·캠프 근처 머릿수(+미시야 도달가능 적)로 열세면 true
- 0.5.8 logic(앞 1500자):
```
fn check_epic_giveup(version, _rnd, player, data, team_plan, _debug) -> bool

[L110] if !rule_scope::morgard_exists(data.context) { return true }
         // 인라인 = runner.rs:263 spawn_epic: context.tutorial(+0x38) 태그가 1..=6 (First~JungleOnly) 이면 Morgard 없음
[L116] moba = data.cache.game.get_game_mode()/*vtable+0x40*/ .as_moba().unwrap()   // 태그≠0 → unwrap_failed
       if moba.jungle_runner.epic.live_list.len(+0x1a8) == 0 { return true }         // 살아있는 에픽 없음

[L122] if team_plan.objective(+0x41f) == Some(Morgard{phase: Hunt(3), ..}) {
[L123]   epic_id = moba.live_list[0]   // len==0 이면 이 블록 통째로 skip → L138
         epic = game.get_entity_by_id(epic_id)/*vtable+0x1f0*/ ; None 이면 skip → L138
[L125]   hp_ratio = epic.hp(+0x670) * 100 / epic.stat_cached.hp(+0x628)   // max_hp 0 → div_by_zero 패닉
[L126]   if hp_ratio <= 20 { return false }                                 // 거의 잡았으면 포기 안 함
[L127]   if v23_visible_objective_overload(player, data, MapDef::camp_pos(map, Morgard(4), team==0)) { return true }
       }

[L138] team = player.info.team ; live_ally_count  = cache.champions(team, pool).len()
[L140] live_enemy_count = cache.champions(1-team, pool).len()
[L143] camp = MapDef::camp_pos(map, Morgard(4), team==0)
[L144~146] ally_near = player_champion[team].iter_champions().filter(|e| distance_sq(e, camp) < 22500000001).count()
[L147~154] enemy_possible = player_champion[1-team].iter_champions().filter(|(i,e)| {
             need   = sat_sub(utils::distance(team_plan.vision.last_v
```

## `e8b5e0` → `e9de70` SerpenCheckSubPlan::action_candidates  (5512→7178B · Δ+1666)
- 명령 1057→1460 · 정렬 1037 · exe 판정 ⚠정렬 불가(미확정) · 정규화 7 · 블록이동 82 · 스택슬롯 235 · 패닉스텁 재배열 2 · 콜리 주의 4
- 잔여 구조: 구 20명령(mov|rax, qword ptr [r9 + I]…) / 신 423명령(mov|r10, qword ptr [r8 + I]…) 짝 없음
- 잔여 분기: e8b660 → e8cb54/e9f4b5 ; e8b79e → e8b7aa/e9e14e ; e8ba44 → e8ba80/e9e1b8 ; e8ba64 → e8cacf/e9f479 ; e8ba87 → e8b804/e9fa78
- 잔여 변위: r9+0x668→0x660 · rax+0x3b→0xcc0
- 콜리 주의: 31a01a3→381e0b3 미지 ; 31a37c0→1660a00 미지(+354B) ; dd26e0→f09860 변경 ; ea65c0→ebde60 미지(pdata 밖 thunk)
- 정규화 흡수: 오프셋표 [r8+930→a00] · SmallActionPlay 태그 2→1 · SmallActionPlay 태그 e→d · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [rdi+1e0→3e8]
- 0.5.8 명세 #192 `serpen_check__SerpenCheck__action_candidates` src game-ai\src\plan_legacy\sub_plan\serpen_check.rs:14 · one_line: 서펜 스틸 Lurk(SerpenCheck) 서브플랜의 후보 생성 — 위험(논타겟 윈드업·궤적)이면 도주 단일, 아니면 v25 태세→서펜 캠프/(아군 Rhino) 경유지 대기 + 근접 적(최근 시야 챔프·others 150000 이내)이면 도주 추가 + (적에게 보이면) 전투 후보 + 소환물 공격 후보
- 0.5.8 logic(앞 1500자):
```
fn action_candidates(&mut self, version, rnd, player, data, parameter, team_plan, debug) -> Vec<SmallActionPlay>
L15  bump = data.context.pool; res = Vec::new_in(bump)
L17  team = player.info.team (bounds<2); champ = data.cache.player_champion[team][player.info.position as usize].unwrap()
L19  if !self.move_check {
L20    if !is_enemy_side(team, champ.x, champ.y)   // 아군 진영일 때만 Rhino 검사. is_enemy_side = (team==0) XOR is_blue_side(x,y), is_blue_side = !((x - y + setting.height) > setting.width)  (map_regions.rs:58, 7~8) · IR %79 = (team==0) XOR ugt = !is_enemy_side (m14.ll:30378~30380)
L23      camp = context.map.camp_pos(JungleType::Rhino, is_blue_side=team==0)
L25      if |champ.x-camp.x|² + |champ.y-camp.y|² < 4900000001 (70000²+1) { self.move_check = true }
       else { self.move_check = true }   // 적 진영이면 즉시 통과 (store 는 dbg L0 블록 %99 공유)
     }
     mc = self.move_check (phi %101)
L35  enemy_team = 1 - team
     has_non_target_action_range = cache.player_champion[enemy_team].iter().flatten().any(|c|
L36    nontarget_windup_perceived(version, player, data, c) && c.ty==Champion(13) &&
L37    match c.action_state { Skill(4) => e=c.skill_effect.unwrap()(casting -1 → panic) ,
L39                           Skill2(5) => e = if c.level>2 {c.skill2_effect} else {DEFAULT_EFFECT(@anon…22)} ,
L41                           Ult(6)    => e = if c.level>4 {c.ult_effect} else {DEFAULT_EFFECT} , _ => false }
       && matches!(e.casting, Position(1)|Direction(2)) && e.is_in_range(caster=c
```
