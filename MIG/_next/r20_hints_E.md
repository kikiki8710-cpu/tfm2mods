# 배치 E — 변경 99 중 티어 2/3 14 (0.5.8 RVA → 0.6.0 RVA · exe 정규화 정렬 힌트 · 0.5.8 명세 요지)

정규화 정렬(`--norm`)이 이미 흡수한 것: PlayerState +0xd0 · Blackboard 0x2e8→0x5c8 · SmallActionPlay 태그/idx 재번호 · BattleSubPlanGoal 재번호 · Effect vt 슬롯(4 삽입) · LPH 오프셋 6. 아래 「잔여」 가 정렬로 못 닫은 차이.

## `e59190` → `d5aff0` v50_fold_dive_episode  (889→886B · Δ-3)
- 명령 202→201 · 정렬 197 · exe 판정 ⚠정렬 불가(미확정) · 정규화 1 · 스택슬롯 14 · 콜리 주의 1
- 잔여 구조: 구 5명령(movzx|eax, byte ptr [rsi + I]…) / 신 4명령(xor|r13d, r13d…) 짝 없음 ; e591d7 피연산자 형
- 잔여 변위: rcx+0x570→0xe30 · rsi+0x598→0xe58 · rsi+0x5a8→0xe68 · rsi+0x1480→0x1f50 · rsi+0x588→0xe48 · rsi+0x1480→0x1f50 · rsi+0x570→0xe30 · rsi+0x580→0xe40 · rsi+0x590→0xe50 · rsi+0x5a0→0xe60
- 콜리 주의: caf500→ec2660 미지(-4B)
- 정규화 흡수: 오프셋표 [rsi+5e8→f58]
- 0.5.8 명세 #5 `dive_episode__v50_fold_dive_episode` src game-ai\src\plan_legacy\handler\dive_episode.rs:125 · one_line: 진행중 다이브 에피소드를 종료 확정(take)해 통계 레코드로 접어 Vec에 push하고, 접촉 없이 끝난 건 다이브 포기 시각으로 기록
- 0.5.8 logic(앞 1500자):
```
fn v50_fold_dive_episode(&mut self, _version, _tps, aborted: bool, end_reason: u8)

// ── A) 접촉 없이 끝난 다이브면 '포기 시각' 갱신 (dive_episode.rs:131~140)
if self.v50_dive_ep_live != None { // self+0x570 != -1 (:131, 들여쓰기 6)
 let no_contact = live.in_range_ticks == 0 // :138 첫항, self+0x598
 && live.team_holder_ticks == 0; // :138 뒷항, self+0x5a8
 if no_contact && end_reason != 7 { // :139
 self.last_dive_abandon_tick = // :140, self+0x1480
 self.last_dive_abandon_tick.max(live.last_tick); // umax(self+0x1480, self+0x588)
 }
}
// ★A 블록은 self 를 소비하지 않는다 — 게이트가 거짓이어도 아래 B/C 는 항상 실행된다.
// ★소스 구조(11차 배치B — 10차의 「중첩 if 둘」은 **판정반전**됐다. 본문길이 = bytes−1):
//   L130 = 5자 `    {` — 들여쓰기 4 에서 블록을 여는 줄(코드 생성 0 — IR 에 126~130 줄의 명령이 없다)
//   L131 = 58자 = 들여쓰기 **6** + `if let Some(live) = self.v50_dive_ep_live.as_ref() {`(52) ±0
//   L132~137 = 한국어 주석 6줄(코드 없음 · 132~137 에 !dbg 달린 명령 0건)
//   L138 = 81자 = 8 + `let no_contact = live.in_range_ticks == 0 && live.team_holder_ticks == 0;`(73) ±0
//   L139 = 42 = 8 + `if no_contact && end_reason != 7 {`(34) ±0 → L140 = 88 = 10 + 78 ±0
//   L141/142/143 = 9/7/5자 = `}` 들여쓰기 8/6/4 ⟹ 열린 블록 = L130(4)·L131(6)·L139(8) 셋으로 완전히 설명
//   L156 = 54 = 4 + `if let Some(done) = self.v50_dive_ep_live.take() {`(50) ±0 ⟹ fn 본문 들여쓰기 4 를 독립 고정
//   ⟹ `in_range_ticks == 0` 은 독립 중첩 if 가 **아니라** `no_contact` 의 첫 항이고,
//     단축평가로 그 첫 항이 `if let` 게이트와 합쳐져 m13.ll:28992 `select i1 %7, i1 %10, i1 false` 가 됐다.
//     의미는 동일하므로 재구현은 이대로 두어도 된다.
//   ⚠L130 이 맨몸 `{`(스코프 블록)인지 다른 구문의 여는 줄인지는
```

## `e5c1f0` → `d5ebb0` single_try_engage  (1791→1765B · Δ-26)
- 명령 348→343 · 정렬 336 · exe 판정 ⚠정렬 불가(미확정) · 정규화 1 · 블록이동 2 · 스택슬롯 1 · 콜리 주의 4
- 잔여 구조: 구 12명령(mov|rsi, qword ptr [rbp + I]…) / 신 7명령(mov|rdx, r12…) 짝 없음
- 잔여 분기: e5c25e → e5c4e6/d5eea5 ; e5c354 → e5c4e6/d5eea5
- 잔여 변위: rcx+0x990→0x1408
- 콜리 주의: 31a01a3→381e0b3 미지 ; d52030→f9bfe0 변경 ; d60920→fae830 변경 ; dff080→ddd7e0 변경
- 정규화 흡수: 오프셋표 [rsi+930→a00]
- 0.5.8 명세 #15 `modes__single_try_engage` src game-ai\src\plan_legacy\handler\modes.rs:241 · one_line: target_id 를 노리는 1인 교전 플랜(SinglePlanBattle)을 만들어 1틱 돌려보고, 회피/종료로 귀결되면 None 을 돌려주는 시도 함수
- 0.5.8 logic(앞 1500자):
```
241 fn single_try_engage(&self, version, rnd, player, data, target_id, debug) -> Option<SinglePlanBattle>

242 let game = data.cache.game; // &dyn AbstractGame (data=cache+0x0, vtable=cache+0x8)
242 let in_tower = game.get_entity_by_id(target_id) // vtable +0x1f0 간접호출
 .is_some_and(|e| tower_discipline::engage_requires_dive(player, data, e));
 // 대상 엔티티가 없으면 in_tower = false (블록 %26)

243 let mut battle = if in_tower {
244 let target = game.get_entity_by_id(target_id); // 같은 슬롯 재호출
244 if target.is_none() { return None; } // 반환 니치에 -1 저장
245 if !single_battle::single_tower_dive_is_viable(version, rnd, player, data,
 &self.team_plan /*+0xf8, `&TeamPlan` 공유 참조*/, target, debug) {
245 return None;
245 }
248 let mut b = SinglePlanBattle::new_dive(version, BattlePlanGoal::TryKill(target_id, 60), data, player) // ★`&` 없음(by-value) — tcx `fn(usize, BattlePlanGoal, &OperationData, &PlayerState)`;
249 b.dive_tower = data.cache.iter_towers_without_nexus(1 - player.info.team /*+0x930*/)
249 // 반환 이터레이터 = Chain<Flatten<IntoIter<Option<&Entity>,6>>, Copied<Iter<&Entity>>>
249 // 앞 6칸 = [top_tower, mid_tower, bottom_tower, top_tower2, mid_tower2, bottom_tower2][team]
 // (+0x180/0x1a0/0x1c0/0x190/0x1b0/0x1d0, 이 순서 그대로) .flatten()
 // 뒤 = twin_towers[team](+0x130, bumpalo Vec<&Entity>).iter().copied()
 // nexus(+0x170)는 별도 필드라 실제 제외. 실측 **팀당 8개**(이름있는 6칸 + twin_towers 2개), **좌표 중복 0** (⚠하네스가 `init_tower` 를 재호출하면 타워가 2배로 늘어 팀당 10개(6+4, twin 2쌍 좌표 중복)로 관측된다) 
249 .min_by_key(|t| t.distance_sq
```

## `cd05f0` → `eb52b0` base_battle_action  (21149→21099B · Δ-50)
- 명령 4574→4562 · 정렬 4549 · exe 판정 ⚠정렬 불가(미확정) · 정규화 78 · 블록이동 15 · 스택슬롯 79 · 패닉스텁 재배열 4 · TypeId 2 · 콜리 주의 4
- 잔여 구조: 구 25명령(mov|r15, qword ptr [rbp + I]…) / 신 13명령(mov|rcx, qword ptr [rax]…) 짝 없음 ; cd175a 피연산자 수
- 잔여 분기: cd0daf → cd0dc7/eb5a87 ; cd0ebd → cd5786/eb59fe ; cd1544 → cd15b5/eb7537 ; cd1f26 → cd1f2f/eb6bdf ; cd2830 → cd2892/eb96bd
- 잔여 소형 즉치: 0xcd132e:-0x3→-0x2 · 0xcd1536:0x5→0x2
- 잔여 즉치: -0x3→-0x2 · 0x5→0x2
- 잔여 변위: rcx+0xa0→0xc0 · rcx+0xa0→0xc0 · rcx+-0x3→-0x2 · rcx+0xa0→0xc0 · rcx+0xa0→0xc0 · rcx+-0x3→-0x2 · rcx+0xa0→0xc0 · rcx+0xa0→0xc0
- 콜리 주의: 12857f0→1643790 변경 ; 128cc90→164b370 미지 ; 31a01a3→381e0b3 미지 ; cb0310→e966a0 미지(-2B)
- 정규화 흡수: 오프셋표 [r8+930→a00] · 오프셋표 [r8+9c0→a90] · BattleSubPlanGoal 3→2 · SmallActionPlay 태그 f→e · SmallActionPlay 태그 f→e · SmallActionPlay 태그 f→e · SmallActionPlay 태그 f→e · SmallActionPlay 태그 10→f
- 0.5.8 명세 #249 `battle__base_battle_action` src game-ai\src\plan_legacy\sub_plan\battle.rs:1305 · one_line: BattleSubPlan 기본 전투 후보 생성: 사거리(+이동 max_tick) 안 적 챔프·타워·본진공격 미니언에 Attack, 대상제약·대시가치·힐/실드/버프 조건 통과한 적/아군에 Skill·Skill2, 궁 후보 Ult 를 bumpalo Vec<SmallActionPlay> 로 반환 (battle.rs:1305~2039 전체 · 4분책 병합본)
- 0.5.8 logic(앞 1500자):
```
// battle.rs:0~1511 (배치 A)
// ─── 머리 (1305~1343) ───
// 1306  team = player.info.team(0x930); pos = player.info.position(0x9c0)  [team<2 아니면 panic_bounds_check]
//       champ = data.cache.player_champion[team][pos].unwrap()  [None ⟹ unwrap_failed]
// 1312  kb_hold: bool = (sub_goal.tag == 3 KitingBack) && version > 1 && base_defense_focus(player, data)
//       base_defense_focus(defense_nexus.rs:200 · m04.ll:57133) = last_stand_flags(TLS 메모 · 키 player.info.id · (seed,tick) 바뀌면 clear).0 || .2 = nexus_last_stand_uncached || base_attacking_minion_uncached.is_some()   // exe 에선 인라인(0xcd05f0 cd0687~cd06ef)
//       (IR phi %97 = max_tick: !KitingBack → 30 / KitingBack&&version<=1 → 0 / KitingBack&&version>1 → base_defense_focus ? 30 : 0)
// 1361  max_tick = if KitingBack && !kb_hold { 0 } else { 30 }   // 위 phi 와 동치
// 1316  near_allies: Vec<&Entity> = cache.player_champion[team].iter().filter_map(iter_champions closure: Some 만).filter(closure$0: a.id != champ.id && dist_sq(a,champ) < 150000²).collect_in(data.context.pool)
// 1318~1325 near_enemies_with_action: Vec<(SmallAction,&Entity)> = cache.player_champion[1-team].iter().enumerate()
//         .map(closure$1: |(i,e)| (data.blackboard[1-team].small_actions[i], e))          // Option<SmallAction> 24B · bounds i<5
//         .filter(closure$2: act.is_some() && e.is_some() && data.can_target(cache.game, player, e) && !is_enemy_well_danger(version, player, e.x, e.y))
//         .map(closure$3: (act.unwrap(), e.unwrap())).collect
```

## `d676c0` → `fe3fd0` v16_gambler_ult_cc_bonus  (2502→2422B · Δ-80)
- 명령 629→604 · 정렬 577 · exe 판정 ⚠정렬 불가(미확정) · 정규화 3 · 블록이동 7 · 스택슬롯 23 · TypeId 1 · 콜리 주의 2
- 잔여 구조: 구 52명령(mov|rdi, qword ptr [rdx]…) / 신 27명령(cmp|rax, qword ptr [rdx]…) 짝 없음
- 잔여 분기: d677a3 → d677c4/fe40d4 ; d67e28 → d67eb2/fe474a ; d67e35 → d67e3b/fe4757 ; d67ea6 → d67eaa/fe47c8 ; d67fa0 → d67fc5/fe4905
- 잔여 즉치: 0x78→0x88 · 0x78→0x88
- 콜리 주의: 12857f0→1643790 변경 ; d433f0→fd47a0 ptr:미지(-587B)
- 정규화 흡수: 오프셋표 [rbx+930→a00] · 오프셋표 2e8→5c8
- 0.5.8 명세 #215 `battle_common__v16_gambler_ult_cc_bonus` src game-ai\src\plan_legacy\sub_plan\battle_common.rs:109 · one_line: 겜블러 궁(GamblerUltAction, 매혹 CC)을 적 챔피언 target 에 쓸 때의 가산점(0~70) — CC 시간·궁 피해·대상 HP가치·유입피해·근접 아군/가시 적 수·포커스/서포트 일치로 합산
- 0.5.8 logic(앞 1500자):
```
L120  let Some(gambler_ult) = action.as_any().downcast_ref::<GamblerUltAction>() else { return 0 };   // TypeId i128 비교
L124  if target.team == champ.team { return 0 }                    // TeamType derived PartialEq
L125  if target.ty 태그 != 13(Champion) { return 0 }                // L124·L125 는 IR 에서 한 select 로 접힘
L126  if target.stat_buff_cached.cc_immune { return 0 }
L127  if target.cc.iter().any(|c| c.is_cc()) { return 0 }           // entity.rs:534 is_cc = 태그∈{Airborne,Stun,Bind,ForceMove,Fear,Charm}
L131  let cc_time = effect.ty.expected_cc_time().map_or(gambler_ult.charm_duration, |t| max(t, gambler_ult.charm_duration));   // (외연 동일: max(charm, cc.unwrap_or(0)))
L132  if cc_time == 0 { return 0 }
L136  let mut bonus = ((cc_time as i64) / 3).clamp(12, 30);         // IR: cc_time<36 → 12, else min(cc_time/3, 30)
L138  if let Some(tp) = parameter.near_enemies.iter().find(|p| p.id == target.id) {
L139      let hp_value = min(champion_hp_value(data, parameter, tp), 100);
L140      let ult_damage = effect.expected_damage_target(data.context, champ as &dyn AbstractEntity, target);
L141      let incoming = tp.applyed_damage + tp.risk_damage                                     // L141~L143
L143                   + tp.possible_risk(data, cc_time + 30);
L144      bonus += min(ult_damage * hp_value / max(target.hp,1), 35);       // sdiv(i64)
L145      bonus += hp_value / 6;
L146      if incoming > 0 {                                                    // signed
L147          bonu
```

## `de1ee0` → `ff4940` EpicStanceData::update_plan  (4642→4756B · Δ+114)
- 명령 1073→1108 · 정렬 995 · exe 판정 ⚠정렬 불가(미확정) · 정규화 1 · 블록이동 67 · 스택슬롯 144 · 콜리 주의 5
- 잔여 구조: 구 78명령(mov|r15, qword ptr [rbp + I]…) / 신 113명령(mov|qword ptr [rbp + I], rcx…) 짝 없음
- 잔여 분기: de232d → de2440/ff4da8 ; de233a → de3099/ff4b41 ; de234e → de2364/ff4dae ; de235e → de2443/ff4ef0 ; de2423 → de2350/ff4df0
- 잔여 즉치: 0x1a8→0x1b8 · 0x1a8→0x1b8 · 0x1e0→0x2
- 콜리 주의: 10ea8e0→100b8d0 미지(+177B) ; 12857f0→1643790 변경 ; c9c160→e080d0 불일치(mig060 는 e07e10) ; c9c2c0→e08230 불일치(mig060 는 e07f70) ; de3d90→137cbc0 불일치(mig060 는 100b8d0)
- 정규화 흡수: 오프셋표 [rax+930→a00]
- 0.5.8 명세 #76 `goal_data__EpicStanceData_update_plan` src game-ai\src\goal_data.rs:266 · one_line: 에픽 태세(None/Check/CheckTry) 결정: 에픽 존재·시야로 hp 갱신 → 아군/적 처치시간 산출 → 준비되면 CheckTry, 적이 먹을 낌새면 인원차 확인 후 Check
- 0.5.8 logic(앞 1500자):
```
fn update_plan(&mut self, _v, _rnd, player, data, enemy_region, _debug):
  moba = get_game_mode(); if tag!=0 { self.stance=None; return }                     // :268~270
  epic_state = &moba.jungle_runner.epic (+0x198)                                       // :271
  epic = epic_state.live_list.get(0).and_then(get_entity_by_id); if None { self.stance=None; return }   // :272~274
  if self.last_epic_seen < epic_state.next_respawn_tick { self.last_epic_hp = epic.stat_cached.hp; self.last_epic_seen = tick() }   // :280~282
  if is_visible(player.team, epic.id) { self.last_epic_seen = tick(); self.last_epic_hp = epic.hp }   // :284~286
  ally_in_epic = (0..5).filter(|p| player_champion[team][p].is_some_and(|c| map.region_dist(map.regions[c.cell], 7) == 0))   // :289~299 closure#2 — 세르펜판(150k 거리)과 달리 '에픽 리전 안' 판정
  as_entity = ally_in_epic.filter_map(|p| player_champion[team][p])                     // :301 closure#3
  self.epic_ally_tick        = check_epic_kill_time_with_hp(ctx, as_entity, epic, self.last_epic_hp)   // :305
  self.epic_ally_killed_tick = check_epic_killed_time(epic, as_entity)                 // :306
  enemy_in_epic = (0..5).filter(|p| enemy_region[p].is_none_or(|e| {                    // :308~324 closure#4
        echamp = player_champion[1-team][p]? (None→false)                                // :310~311
        dist = distance(region_center(e.region), region_center(7))                        // :312~315
        move_tick = dist / echamp.stat_cached.move_speed
```

## `e05e70` → `ee4e70` resolve_join_stake  (3571→3721B · Δ+150)
- 명령 742→770 · 정렬 669 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 블록이동 24 · 스택슬롯 189 · 콜리 주의 3
- 잔여 구조: 구 73명령(sub|rcx, rax…) / 신 101명령(sub|rcx, r13…) 짝 없음 ; e06073 피연산자 형 ; e060c0 피연산자 형 ; e062ea 피연산자 형 ; e06a45 피연산자 형 ; e06a48 피연산자 형
- 잔여 분기: e06071 → e06050/ee50d0 ; e0623d → e0627b/ee52da ; e06332 → e06351/ee53ac ; e0659c → e065cc/ee5620 ; e065a5 → e065cc/ee5620
- 잔여 즉치: 0x288→0x298 · 0x288→0x298
- 잔여 변위: rsi+0xf8→0x300 · rax+0x100→0x308 · rax+0x108→0x310 · rdx+0x438→0x4a0 · rdx+0x4a0→0x438 · rbx+0x680→0x438 · rcx+0x450→0x480
- 데이터: e065f1 movaps 16B 01000000000000000400000000000000→01000000000000000500000000000000 ; e0661e movaps 16B 01000000000000000500000000000000→01000000000000000400000000000000
- 콜리 주의: ca2960→e0fad0 불일치(mig060 는 e0b8b0) ; e04c60→ee3000 미지(+102B) ; e05450→ee4420 변경
- 정규화 흡수: 오프셋표 [r14+930→a00] · 오프셋표 2e8→5c8
- 0.5.8 명세 #49 `fight_model__resolve_join_stake` src game-ai\src\plan_legacy\old\fight_model.rs:680 · one_line: anchor(적) 주변 교전에 내가 6초 내 합류할 때의 '저울' — 아군만/아군+나 두 판을 resolve_fight_full 로 돌려 차분 FightPrediction 을 낸다
- 0.5.8 logic(앞 1500자):
```
fn resolve_join_stake(version, rnd, data, player, champ, anchor, team_plan, debug) -> Option<FightPrediction>

[L682] if version < 2 { return None }                      // 41062
[L685] horizon = data.context.setting.tick_per_second * 6    // 6초
[L688] cache = data.cache ; my_team = player.info.team(+0x930) ; enemy_team = 1 - my_team (≥2 면 panic_bounds_check)
[L687~691] enemies: Vec<&Entity,&Bump> = cache.player_champion[enemy_team] (iter_champions, Some 만)
      .filter(|e| {                                   // ★클로저0 = aux m10.ll 56030~56141 (Entity::call_mut 심)
         [L689] e.id == anchor.id                      // anchor 자신은 무조건 포함
              || ( distance_sq(e, anchor) < 22500000001   // ≤150000
         [L690]   && data.blackboard[enemy_team].is_recent_visible(cache.game, player, e)
         [L691]   && !is_ignored_well_enemy(version, player, e) )
                  //  is_ignored_well_enemy 인라인 = (e.team(+0x0) == TeamType::Player(enemy_team)) && path_finder::is_enemy_well_danger(version, player, e.x, e.y)
      }).collect_in(pool)
[L692~693] if enemies.is_empty() { return None }

[L696] allies: Vec<&Entity> = new_in(pool) ; [L697] arrivals: Vec<usize> = new_in(pool)
      nearest_bound: Option<&Entity> = None
      my_pos = player.info.position 태그(+0x9c0)
[L699] for i in 0..5 {
[L700]   if i == my_pos { continue }                          // 나 자신 슬롯 제외
[L701]   if team_plan.ally_battle_stop_tick[i].is_some() { continue }   // 태그(+0x0+16i) != 0
[L702]   a = cache.p
```

## `e7b9f0` → `f28320` end_check  (2859→3059B · Δ+200)
- 명령 698→749 · 정렬 692 · exe 판정 ⚠정렬 불가(미확정) · 정규화 7 · 블록이동 5 · 스택슬롯 39
- 잔여 구조: 구 6명령(mov|dword ptr [rsp + I], edx…) / 신 57명령(cmp|rcx, I…) 짝 없음
- 잔여 분기: e7ba39 → e7ba6f/f283a7 ; e7ba51 → e7ba6f/f283a7 ; e7c435 → e7c2ec/f28485 ; e7c43b → e7c32c/f28464
- 정규화 흡수: 오프셋표 [r14+508→578] · 오프셋표 [r14+4f8→568] · 오프셋표 [r14+930→a00] · 오프셋표 [r14+9c0→a90] · 오프셋표 [r14+930→a00] · 오프셋표 [r14+930→a00] · 오프셋표 2e8→5c8
- 0.5.8 명세 #255 `end_check` src game-ai\src\lib.rs:1211 · one_line: 판 종료(넥서스 마무리 진입) 판정 — 선수의 game_finish 전략(Stable/Flexible/Aggressive)별로 라인 타워·생존/건강 아군 수·적 넥서스 근접 인원을 세어 bool 반환
- 0.5.8 logic(앞 1500자):
```
fn end_check(version, rnd, player, data, line, debug) -> bool   // lib.rs:1211
ctx = data.context; cache = data.cache; game = cache.game(&dyn AbstractGame)
L1212: if !rule_scope::line_exists(ctx, line) { return false }           // m14.ll:8736~8737, %20 phi false
L1216: strat = player.strategy(rnd, game)  (sret Strategy 24B)  ; match strat.game_finish(+0x15) {

== Stable (tag 0) → end_check_stable(player, data, line)  lib.rs:1279~1302 ==
L1279: state = build_game_finish_check_state(player, data, line)?   // None(+68==2) → return false (m14.ll:8772→%47)
L1280: required_allies = min(ctx.tutorial.player_count(), 4)   // 외연 동일 표기(표기 불가) · player_count 원값: None/Total=5 · Line=4 · First/Bottom=2 · TopSolo/MidSolo/JungleOnly=1 · MidBottom=3 (MIR runner.rs:296~304) → 접힘값 4/4/2/1/3 (phi m14.ll:8836)
L1282: if (state.line_tower_alive(raw u8 +65) != state.has_enemy_twin_tower(raw u8 +64)) || state.line_tower_alive { return false }
        // IR 문면 `icmp ne %40,%38 ; or ..., %41`(m14.ll:8838~8840). 불 대수상 line_tower_alive || has_enemy_twin_tower 와 동치. 소스 표기는 미확정(표기 불가)
L1286: if !(state.live_ally_count >= required_allies && state.healthy_ally_55_count >= state.live_ally_count && state.near_player_ally_count == state.live_ally_count && state.stable_pushed_line) { return false }   // m14.ll:8843~8849
L1298: match state.live_enemy_count { 0 => return true,                                            // m14.ll:8853
L1302:   1 => return state.near_enemy_nexus_healthy_ally_55_count >= required_a
```

## `d9a220` → `100b570` v22_current_line_non_champion_action_tower_risk  (563→856B · Δ+293)
- 명령 124→185 · 정렬 107 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 블록이동 3 · 스택슬롯 24 · 콜리 주의 1
- 잔여 구조: 구 17명령(movzx|ecx, byte ptr [rbx + I]…) / 신 78명령(mov|rcx, rax…) 짝 없음
- 잔여 분기: d9a291 → d9a427/100b89e ; d9a2a5 → d9a313/100b89c ; d9a2d1 → d9a313/100b89c ; d9a315 → d9a427/100b89e
- 잔여 즉치: 0xd8→0x118 · 0xd8→0x118
- 잔여 변위: rdi+0x1d0→0x148
- 콜리 주의: d74e50→e6e4e0 미지(-17B)
- 정규화 흡수: 오프셋표 [rsi+930→a00] · 오프셋표 [rsi+9c0→a90]
- 0.5.8 명세 #238 `tower_discipline__v22_current_line_non_champion_action_tower_risk` src game-ai\src\tower_discipline.rs:225 · one_line: 라인전 구간에서 비챔피언(미니언/타워 등) 대상 행동 중 적 타워에 물릴 위험이 있는가 — 사거리 안 적 타워가 나를 조준 중이거나, 최근 적 타워에 맞았고 커버가 즉시 끊기면 true
- 0.5.8 logic(앞 1500자):
```
fn v22_current_line_non_champion_action_tower_risk(version, context, cache, player, target) -> bool
// version(%0) 은 본문에서 한 번도 읽지 않는다.
L225: tick = cache.game.tick()
      // context.is_line_phase(tick) 인라인(runner.rs:399 → spawn_epic runner.rs:263 → setting.rs:703)
      spawn_epic = context.tutorial ∈ {None(0), MidBottom(5), Line(7), Total(8)}
      if spawn_epic && !(tick < setting.epic_jungle.first_spawn_tick.saturating_sub(setting.tick_per_second*30)) → return false
      // (tutorial 이 그 외 값이면 시간 판정 없이 통과)
L226: if !(cache.game.tick() < setting.tower_attack_disable_tick) → return false   // tick 재호출
L230: team = player.info.team (bounds <2 · panic_bounds_check)
      champ = cache.player_champion[team][player.info.position.as_index()]  ; None(null) → return false
L233~235: recently_hit_by_enemy_tower =
      champ.last_attacked_from                       // Option<usize> (+0x28 tag / +0x30 id)
        .and_then(|id| cache.game.get_entity_by_id(id))   // L234 closure$0
        .is_some_and(|e| e.team == TeamType::Player(1 - team) && e.is_tower())   // L235 closure$1 : +0x0==0 && +0x8==1-team && +0x68==2
L237: iter = cache.iter_towers_without_nexus(1 - team)
L238: .filter(|t| t.can_target())        // closure$2 = entity.rs:1478 : t.can_target(+0x6b9) && t.block_target_tick(+0x6a0)==0
L239: .any(|tower| {                     // closure$3 (aux m06/m11)
  L240:   let Some(attack) = tower.attack_effect.as_ref() else { return false }   // +0x4c0 == -1 → false
  L244:   if !atta
```

## `dd5db0` → `f0cfe0` TeamPlan::v24_objective_setup_should_check_camp  (3467→3125B · Δ-342)
- 명령 736→693 · 정렬 627 · exe 판정 ⚠정렬 불가(미확정) · 정규화 4 · 블록이동 107 · 스택슬롯 64 · 콜리 주의 1
- 잔여 구조: 구 109명령(movzx|r12d, byte ptr [rsp + I]…) / 신 66명령(movzx|r15d, byte ptr [rsp + I]…) 짝 없음 ; dd65d3 피연산자 형
- 잔여 분기: dd5de1 → dd5e06/f0d121 ; dd5ea6 → dd5fb6/f0d16a ; dd5eb5 → dd5fc5/f0d263 ; dd5f0f → dd5fc8/f0da50 ; dd5f4e → dd6073/f0d1bf
- 잔여 소형 즉치: 0xdd672f:0x10→0x8 · 0xdd6769:0x18→0x10 · 0xdd67aa:0x20→0x18
- 잔여 즉치: 0xfa00→0x2ee01 · 0x2ee01→0xfa00 · 0x27→0x2ee01 · 0x10→0x8 · 0x18→0x10 · 0x20→0x18 · 0x8→0x20 · 0x80→0xa0 · 0x88→0xa8
- 잔여 변위: rcx+0x41f→0xcd6 · rcx+0x420→0xcd5 · r14+0x0→0x208 · r13+0x10→0x8 · r13+0x18→0x10 · r13+0x20→0x18
- 콜리 주의: 31a3b40→dc3de0 미지(+241B)
- 정규화 흡수: 오프셋표 [rbx+930→a00] · SmallActionPlay 태그 2→1 · 오프셋표 [rbx+9c0→a90] · SmallActionPlay 태그 1→0
- 0.5.8 명세 #89 `objective_discipline__v24_objective_setup_should_check_camp` src game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 · one_line: [v24] Morgard/Serpen Setup 태세에서 '내가 캠프를 직접 확인하러 가야 하는가' — 라인 압박 완료·건강한 같은-편 아군 충분·내가 checker 로 뽑힘일 때, 적이 치는 중이거나 캠프가 최근에 안 보였거나 숨은 건강 적이 닿을 수 있으면 true
- 0.5.8 logic(앞 1500자):
```
fn v24_objective_setup_should_check_camp(&self, _version, player, data, goal_data, camp: JungleType) -> bool
  team = player.info.team; enemy = 1 - team; ctx = data.context; cache = data.cache
  // L91: 태세 게이트 — take_setup_like(camp) (team_plan.rs:258)
  //   camp==Morgard(4): self.objective 태그==0(Morgard) && phase(+0x420)==1(Setup)
  //   camp==Serpen(5) : self.objective 태그==1(Serpen)  && phase==1(Setup)
  //   그 외 camp: false
  if !take_setup_like { return false }
  // L95: v24_objective_setup_relevant_lanes_ready(player, data, camp) (인라인)
  //   lanes = camp==Morgard ? [Top(0),Mid(1)] : [Bottom(2),Mid(1)]   (anon.190 = \00\01 / anon.191 = \02\01)
  //   ready = v23_objective_setup_pressure_line(player, data, &lanes, 2).is_none()   (반환 i8 == -1)
  if !ready { return false }
  // L99~106: 캠프 쪽 절반에 있는 건강한 아군 수
  side(c) = camp==Morgard ? is_top_side(ctx,c.x,c.y) [= height - y < x] : is_bottom_side [= height - y > x]   (map_regions.rs:24 / :30)
  healthy_side(c) = c.hp*100/c.stat_cached.hp > 39 && side(c) && is_near_mid_line(ctx, c.x, c.y)
  healthy_side_allies = cache.iter_champions(team).filter(healthy_side).count()      // L100~101, [team] 5칸
  N = player_count(ctx) 인라인 → tutorial TopSolo/MidSolo/JungleOnly: 1, 그 외: 2
  if healthy_side_allies < N { return false }     // L106 (samesign ult)
  // L110~111
  camp_pos = ctx.map.camp_pos(camp, team == 0)
  my_idx = player.info.position 태그(i32)
  if cache.player_champion[team][my_idx].is_none() { return false }
  // L123~132: che
```

## `e388c0` → `ed3dd0` SerpenCheckSubPlan::score  (966→1408B · Δ+442)
- 명령 229→317 · 정렬 150 · exe 판정 ⚠정렬 불가(미확정) · 정규화 1 · 블록이동 66 · 스택슬롯 29 · 콜리 주의 12
- 잔여 구조: 구 79명령(mov|r10, qword ptr [rsp + I]…) / 신 167명령(push|r13…) 짝 없음
- 잔여 분기: e3892a → e38c5e/ed4207 ; e3893e → cbbca0/ed4207 ; e3895c → e38c5e/ed4207 ; e389aa → e38c5b/ed41c8 ; e38a2b → e38c5e/ed4207
- 잔여 즉치: 0x40→0x58 · 0x40→0x58 · 0x7→0xfdfb · 0x1fbf3→0x4 · 0x7→0xfdfb
- 잔여 변위: rdx+0x1d8→0x1a8 · rdx+0x1d0→0x1a0 · rdx+0x1a8→0x1d8 · rdx+0x1a0→0x1d0
- 콜리 주의: cc5a10→f64d30 불일치(mig060 는 eb28e0) ; cc5fc0→f66a30 불일치(mig060 는 ea8450) ; ccbb10→f352d0 불일치(mig060 는 f3a070) ; ccf670→f72620 불일치(mig060 는 eb4320) ; d57540→eb28e0 불일치(mig060 는 f72620) ; d57540→f43b10 불일치(mig060 는 f72620)
- 정규화 흡수: 오프셋표 [r14+930→a00]
- 0.5.8 명세 #246 `serpen_check__SerpenCheck__score` src game-ai\src\plan_legacy\sub_plan\serpen_check.rs:123 · one_line: SerpenCheck(정글러 스틸 Lurk) 서브플랜의 후보 액션 점수: interaction_score + (Around 계열 대상이 살아있는 첫 세르펜이고 내 팀 시야에 안 보이면 +10), 그 외 0
- 0.5.8 logic(앞 1500자):
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // serpen_check.rs:123 · self(move_check) 미사용
let base = interaction_score(version, rnd, player, data, parameter, action, debug);   // L124
let add = match action.get_action() {   // L125 · SmallActionPlay 태그 switch
    Around{target_id} /*Play idx 2 Around / 3 AroundHide / 10 LaneMinionPosition · payload+0x8*/ => {
        // L127: 살아있는 첫 세르펜 엔티티
        let game = data.cache.game;   // &dyn AbstractGame (+0x0 data, +0x8 vtable)
        let serpen: Option<&Entity> = game.get_game_mode() /*vtable+0x40 → GameMode{tag,ptr}*/
            .as_moba() /*game.rs:231 · tag≠0(Moba) 이면 unwrap_failed — reach: 접힌 분기(gamemode=0), 사장 호출부 NA*/ .unwrap()
            .jungle_runner.serpen.live_list /*MobaMode+0x1d0 ptr / +0x1d8 len · Vec<usize>*/
            .get(0) /*len==0 → None*/
            .and_then(|id| game.get_entity_by_id(*id) /*vtable+0x1f0*/);
        // L128
        if let Some(serpen) = serpen {
            if serpen.id /*+0x5c0*/ == target_id && !game.is_visible(player.info.team /*+0x930*/, target_id) /*vtable+0xf8*/ { 10 } else { 0 }
        } else { 0 }
    }
    _ => 0,   // Attack/Skill/Skill2 포함 전부 0 — 형제 플랜과 달리 공격 액션 가산이 전혀 없음
};
return base + add;   // L124 합산 · L141

★형제 대조(복붙 의심): 형제 4개(LineSafe/LineWait/Jungle/AttackNexus)는 champ 조회+Attack/Skill/Skill2 arm+calculate_(jungle_)action_score 골격인데 SerpenCheck 는 그 골격이 통째로 없고 Around arm 만 있다(_docs: 「commit=false(Lurk): 시야 확인 주기 기반 대기 … stale 이면 잠깐
```

## `e8aeb0` → `f34d10` LineDefenseSubPlan::calculate_score_parameter_value  (836→1461B · Δ+625)
- 명령 153→235 · 정렬 115 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 블록이동 1
- 잔여 구조: 구 38명령(mov|qword ptr [r8 + I], rax…) / 신 120명령(movzx|ecx, byte ptr [r8 + I]…) 짝 없음 ; e8afa8 피연산자 수
- 잔여 분기: e8b127 → e8b110/f34fc0 ; e8b1b9 → e8b140/f35210
- 정규화 흡수: 오프셋표 [r8+930→a00] · 오프셋표 [r8+9c0→a90]
- 0.5.8 명세 #240 `line_defense__LineDefense__calculate_score_parameter_value` src game-ai\src\plan_legacy\sub_plan\line_defense.rs:396 · one_line: 라인수비 서브플랜의 ScoreParameter 가중치 세팅: 자기 attack/util_value 를 라인 스타일(공격/수비)과 HP% 로 30/50/70 중 택일, 아군 전원 50, 적 전원 50(수비)/100(공격)
- 0.5.8 logic(앞 1500자):
```
fn calculate_score_parameter_value(&self, _rnd, player, data, parameter: &mut ScoreParameter)   [line_defense.rs:396]
  line_style: bool = (self.style == LineStyle::Defensive)        // 태그 1 → true          [L397 · m14.ll:29746~29747]
  champ = data.cache.player_champion[player.info.team][player.info.position].unwrap()      [L399 · 29749~29770 · team<2 체크]
  hp_ratio = champ.hp * 100 / champ.stat_cached.hp                 // usize udiv · max_hp==0 이면 패닉   [L400 · 29774~29787]
  value = if line_style /*Defensive*/ { if hp_ratio > 50 { 50 } else { 70 } }
          else          /*Aggressive*/ { if hp_ratio < 50 { 50 } else { 30 } }             [L401 · 29789~29793]
  parameter.player.attack_value = value                                                    [L418 · 29795~29796]
  parameter.player.util_value   = value                                                    [L419 · 29797~29798]
  for p in parameter.near_allies.iter_mut()  { p.attack_value = 50; p.util_value = 50; }   [L421~423 · 29801~29847]
  for p in parameter.near_enemies.iter_mut() { let v = if line_style { 50 } else { 100 }; p.attack_value = v; p.util_value = v; }   [L426, 435~436 · 29852~29899]
  return                                                                                    [L438 · 29902]

해석: 수비 스타일은 HP 가 절반 이하로 떨어졌을 때 자기 가치를 70 으로 올리고(더 조심/보호) 적 가치는 50 으로 낮춰 본다 · 공격 스타일은 HP 가 절반 이상이면 자기 가치 30 (희생 허용) 이고 적 가치는 100 으로 본다. L402~417 · 424~425 · 427~434 는 IR 에 흔적 없음(빈 줄/주석/닫는 괄호 추정 — 미확인). gen_range 사이트 0.
```

## `d2c5d0` → `ecb2b0` PassiveLinePlan::sub_plan  (5182→6198B · Δ+1016)
- 명령 1194→1396 · 정렬 835 · exe 판정 ⚠정렬 불가(미확정) · 정규화 9 · 블록이동 115 · 스택슬롯 95 · 패닉스텁 재배열 7 · 콜리 주의 1
- 잔여 구조: 구 359명령(mov|rsi, rcx…) / 신 561명령(movabs|rax, I…) 짝 없음 ; d2c7a5 피연산자 형 ; d2c823 피연산자 형 ; d2c82c 피연산자 형 ; d2c84e 피연산자 형 ; d2c881 피연산자 형
- 잔여 분기: d2c5ed → d2c5fb/ecb2db ; d2c5f6 → d2d744/ecc6a2 ; d2c602 → d2c630/ecb331 ; d2c60b → d2c630/ecb331 ; d2c61e → d2c83a/ecb405
- 잔여 소형 즉치: 0xd2ca4a:0x3→0x1
- 잔여 즉치: 0x3→0x1 · 0x1e0→0x28 · 0x78→0x28 · 0x3f→0x8 · 0x28→0x50 · 0x7d1→0x7d0
- 잔여 변위: rdx+0x116→0x11b · r15+0x420→0xcd6 · rdx+0x116→0x11b · rdx+0x116→0x11b · rax+0x8→0x628 · rax+0x18→0x660 · rax+0x668→0x660 · rdx+0x1a8→0x1d8 · rdx+0x1a0→0x1d0 · rdx+0x1d8→0x1a8
- 콜리 주의: dd9f30→f10c60 미지(+39B)
- 정규화 흡수: 오프셋표 [r14+930→a00] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 2e8→5c8
- 0.5.8 명세 #56 `passive_line__sub_plan` src game-ai\src\plan_legacy\old\passive_line.rs:848 · one_line: 수동 라인전 서브플랜 선택 — Recall / LineSafe / LineWait / LineDefense{style,line,action_type} 중 하나를 낸다
- 0.5.8 logic(앞 1500자):
```
fn sub_plan(&self, version, rnd, player, data, team_plan, debug) -> SubPlan

[L849] if self.in_recall(+0x110) { [L850] return Recall }
[L851] if self.v46_flee(+0x112) && !self.v46_flee_cover(+0x115) {
[L852]   if self.v46_flee_acute(+0x113) { [L854] return LineWait{line: self.line} } else { [L858] return LineSafe{line: self.line} } }

[L863] gank_or_dive_here = team_plan.objective(+0x41f) ∈ {Gank(8), Dive(9)} && objective.line(+0x420) == self.line
[L864] line = self.line
[L867] if !gank_or_dive_here {
[L868]   if let Some(p) = self.check_bot_lane_2v1(player, data) { return p } }   // ── 인라인 (passive_line.rs:1018~1041)
       //  [L1019] if line != Bottom(2) → None
       //  [L1020] tick = game.tick()/*vtable+0x28*/; if !context.is_line_phase(tick) → None   // 초반(첫 에픽 스폰 30초 전)에만
       //   ★인라인 GameContext::is_line_phase(&self, tick)[runner.rs:397~399, pub] = !self.tutorial(+0x38).spawn_epic() || self.setting(+0x8).is_line_phase(tick)
       //     TutorialType::spawn_epic()[runner.rs:262~263] 자리의 switch = 태그 ∈{0 None,5 MidBottom,7 Line,8 Total} → 시간 게이트, 그 외(1,2,3,4,6) → 게이트 없이 통과
       //     GameSetting::is_line_phase(&self, tick)[setting.rs:702~704, pub] = tick < epic_jungle.first_spawn_tick(+0x8a8).saturating_sub(tick_per_second(+0x12f8)*30)
       //  [L1021] if position(+0x9c0) <= 2 → None      (Bottom/Support 만)
       //  [L1023] partner_pos = position==3 ? 4 : 3
       //  [L1024] (lx,ly,rx,ry) = map.fountains[team](+0x6d70)
       //  [L1025] partner = cache.pla
```

## `dfe2a0` → `ed5720` FightSituation::build  (3474→2157B · Δ-1317)
- 명령 789→503 · 정렬 443 · exe 판정 ⚠정렬 불가(미확정) · 정규화 8 · 블록이동 15 · 스택슬롯 86
- 잔여 구조: 구 346명령(movaps|xmmword ptr [rbp + I], …) / 신 60명령(mov|rbx, r9…) 짝 없음 ; dfecc1 피연산자 형
- 잔여 분기: dfe3fa → dfe4db/ed592b ; dfe44a → dfe319/ed5788 ; dfe488 → dfe4e2/ed5925 ; dfe4ba → dfe4e2/ed592b ; dfe9cb → dfecd6/ed5c97
- 잔여 즉치: 0x1a8→0x128 · 0x230→0x28 · 0x4→0x28 · 0x1a8→0x128
- 잔여 변위: rdi+0x510→0x580 · rdi+0x518→0x588 · rax+0x660→0x628 · rax+0x10→0x670 · rax+0x3f8→0x710 · rax+0x400→0x718 · rsi+0x10→0x0 · rsi+0x18→0x8 · rsi+0x20→0x10 · rsi+0x28→0x18
- 정규화 흡수: 오프셋표 [rdi+930→a00] · 오프셋표 [rdi+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8]
- 0.5.8 명세 #86 `fight_model__FightSituation_build` src game-ai\src\plan_legacy\old\fight_model.rs:129 · one_line: 교전 상황 요약 구조체(FightSituation 128B) 조립 — 내 HP%/스킬·궁 준비, 전위 아군 유무, 위기 캐리, 아군/적 DPS 합과 우위를 계산
- 0.5.8 logic(앞 1500자):
```
// fight_model.rs:129~238. 판정 없이 FightSituation 을 '조립'하는 함수. prof 타이머(phase 49) 는 prof::ENABLED 일 때만.
t = player.info.team (0x930); pos = player.info.position 태그 (0x9c0)
champ = cache.player_champion[t][pos].unwrap()                         // :140 (None → unwrap_failed 패닉)
my_battle_role = get_battle_role(version, ctx, cache, player)           // :143
my_hp_ratio = champ.hp*100 / max(champ.stat_cached.hp, 1)             // :144
my_skills_ready = champ.skill_cooldown() < 31 || champ.skill2_cooldown() < 31   // :145 (비챔프 엔티티면 cooldown()=0 → true)
my_ult_ready = champ.can_ult()                                         // :146
my_dist_to_focused = dist_sq(champ, focused)                           // :149

// :150~163 has_frontline_ally
has_frontline_ally = (0..5).filter(p != pos)
   .filter_map(p → (player_champion[t][p]?, player_state[t][p]?))
   .any(|(p, ally)| get_battle_role(version, ctx, cache, ally_state) < 2 /*Tanker|Initiator*/ && dist_sq(ally, focused) < my_dist_to_focused)

// :165~191 endangered_carry — 첫 매치에서 중단(find_map)
endangered_carry = None
for p in 0..5 { if p == pos continue;
   ally = player_champion[t][p]?; ally_state = player_state[t][p]?;
   role = get_battle_role(version, ctx, cache, ally_state); if (role & 6) != 2 continue;   // BaseAttacker|SkillCaster 만
   enemies: bumpalo Vec<&Entity> = player_champion[1-t].iter().flatten().filter(|e|          // closure$0 (aux m10:54399)
        dist_sq(e, ally) <= (max_range_cached(data, e, ally) + 30000)^2         
```

## `e083c0` → `ee8450` resolve_fight_uncached  (8841→10775B · Δ+1934)
- 명령 1891→2299 · 정렬 1476 · exe 판정 ⚠정렬 불가(미확정) · 정규화 4 · 블록이동 234 · 스택슬롯 273 · 패닉스텁 재배열 17 · 콜리 주의 2
- 잔여 구조: 구 415명령(test|r14, r14…) / 신 823명령(mov|rbx, qword ptr [rsp + I]…) 짝 없음 ; e08a2f 피연산자 수 ; e08bd9 피연산자 형 ; e08bd9 피연산자 형 ; e08be5 피연산자 형 ; e08cf8 피연산자 형
- 잔여 분기: e084bf → e08518/ee85a4 ; e084d7 → e08518/ee85a4 ; e084ef → e08518/ee85a4 ; e08507 → e08518/ee85a4 ; e0871b → e0896d/ee89f7
- 잔여 소형 즉치: 0xe08a0b:0x5→0x1 · 0xe08e6e:0x4→0x5 · 0xe08eb8:0x5→0x1 · 0xe09626:0x2→0x3 · 0xe09a04:0x5→0x3 · 0xe09a9d:0x2→0x3 · 0xe09ad1:0x3→0x4 · 0xe09b01:0x4→0x5 · 0xe09b31:0x5→0x2 · 0xe09fb3:0x1→0x3
- 잔여 즉치: 0x278→0x308 · 0x5→0x1 · 0x4→0x5 · 0x5→0x1 · 0x2→0x3 · 0x5→0x3 · 0x2→0x3 · 0x3→0x4 · 0x4→0x5 · 0x5→0x2
- 잔여 변위: rbx+0x0→0x378
- 콜리 주의: e33570→dbcec0 불일치(mig060 는 dbd050) ; e33700→dbd050 불일치(mig060 는 dbd1e0)
- 정규화 흡수: SmallActionPlay 태그 0→1 · SmallActionPlay 태그 1→2 · SmallActionPlay 태그 1→0 · SmallActionPlay 태그 0→1
- 0.5.8 명세 #207 `fight_model__resolve_fight_uncached` src game-ai\src\plan_legacy\old\fight_model.rs:378 · one_line: 교전 예측 순수 코어: 아군·적 각 ≤5명의 EHP·DPS·CC·도착틱을 만들어(오판 노이즈 포함) 6초 창을 틱 단위로 전개하고 승패 라인(Commit/CommitAfterJoin/Disengage/Hold)·net_value·focus/soaker/rescue 를 FightPrediction(64B) 으로 낸다
- 0.5.8 logic(앞 1500자):
```
// fight_model.rs:0~464 (배치 K)
// 인자: version, data(&OperationData → %2=data.cache · %3=data.context), champ, near_allies(&[&Entity]), near_enemies(&[&Entity]), committed_dir(i8), tower(Option<&Entity>), judge_accuracy, arrivals(&[i64]), baseline(i64) → sret FightPrediction(64B)
// 콜리 계약(자식 명세 별도): self_sustain_in_window(version,&GameContext,&Entity,horizon)->i64(창 내 자기 힐+실드 1회분) · available_cc_in_window(version,&Entity,horizon)->i64(창 내 CC 틱 합) · fight_dps(version,&GameContext,att,tgt)->i64 · expected_dps(&GameContext,tower,tgt)->i64 · error_ratio_noise(&mut NoiseRng,j)->i64 (m04.ll:49631 본문 확인: e=(1000-j)/20(udiv) · state+=0x9E3779B97F4A7C15 · splitmix64 믹스 · mulhi(mix, 2e+1) - e + 100 ⇒ [100-e, 100+e] 균등 정수 백분율)

L379: let context = data.context;                                             // %3 (승격 슬롯)
L380: if near_allies.is_empty() || near_enemies.is_empty() {                // %40 = len_a==0, %41 = len_e==0, or
L381:   return FightPrediction { focus_target: None, soaker: None, rescue_ally: None, net_value: 0, line: Hold(3), line_absolute: Hold(3) };  // 0x0/0x10/0x20 ← 0, 0x30 ← 0, 0x38/0x39 ← 3 · 페이로드/패딩 미기록 → 배치 L(줄 521) ret
}

// ── 시드 (L392~403) ──
L392: let seed: u64 = if version > 1 {                                       // %58 = icmp ugt %1, 1 (reach: 항상 true 로 접힘)
L394:   let mut set_h: u64 = 0;
L394:   for a in near_allies.iter().take(5) {                                // %97 루프(카운트 5↓ · end 포인터)
L395:     set_h ^= a.id.wrapping_mul(0x9E3779B97F4A7C15);     
```
