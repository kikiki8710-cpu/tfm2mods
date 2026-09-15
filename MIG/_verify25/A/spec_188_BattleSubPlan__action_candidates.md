---

### `188` BattleSubPlan::action_candidates — 전투 서브플랜 행동 후보 루트: 논타겟 궤적/캐스트라인 강제 회피(RunAway dodge) 게이트와 v48 킬확보·지는판정·카이팅 위치 계산 → base_positioning(536) 후보 → 타워/치명 후처리(537~614)

| 항목 | 값 |
|---|---|
| id | `battle__Battle__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battleNtB2_13BattleSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle.rs:358` |
| IR | `m02.ll` 27592~36306행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::BattleSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `cbbdb0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[188]/sig/tls/<키>`)**

- {"name": "CAST_BEAMS", "role": "작성자+소비자(메모)", "key": "(seed = game.seed() [vtable +0x20], tick = game.tick() [vtable +0x28], pid = player.info.id(+0x928)) — m02.ll:34724~34747 에서 %15/%14/%13 스택에 구성 후 클로저 env 에 저장", "layout": "thread_local RefCell<(usize seed, usize tick, usize pid, [Beam;6])> (battle.rs:993, DIGlobalVariable !1585, TLS 전역 328+1+7B). Beam 48B = x1@0 · y1@+0x8 · x2@+0x10 · y2@+0x18 · kind@+0x20(u8: 0=점/원, 그 외=선분) · halfwidth@+0x28. with 클로저 반환 sret 296B = (n usize @0, [Beam;6] @+0x8)", "invalidation": "키 3-튜플 불일치 시 재계산(적 챔피언 recent_visible·Champion·시전 슬롯별 논타겟 프로필로 최대 6빔) 후 키·빔 저장 — m00.ll:93237~93786 LocalKey::with<v48_cast_beams::{closure}> 본문에서 icmp eq ×3 → memcpy 288B 반환 패턴 확인", "call_conditions": "이 범위에서 직접 `LocalKey::with` 호출은 1곳: m02.ll:34749 (L1002<1058<1183<518 = v48_kite_position 인라인 안의 v48_on_cast_line 인라인 안의 v48_cast_beams). 도달 조건 = force_runaway && !(v48_hard && !final_stand) && !(kill_secure 실패 탈출) && !v48_losing && !(v48_in_attack_range && 궤적/캐스트라인 없음) && kite anchor 존재 && attack_effect 존재 && 후보점 s.on_trajectory/on_periodic 아님 — 링(sets)×16방향 루프 안에서 후보점마다 1회(키 같으면 재생). 그 밖에 out-of-line `v48_on_cast_line` 호출 L400(%1824)·L506(%2391)·L1191<518(%2729) 도 내부에서 같은 TLS 를 탄다(콜리 계약 — 본문 미독). 순서(한 틱 최초 발화 기준): L390 position_score_at_position → [L400 v48_on_cast_line] → L449/451 check_kill_die_tick → L478/474 resolve_fight_stake(_roster) → [L506 v48_on_cast_line] → L518 루프 내 1178 position_score_at_position → 1183 CAST_BEAMS.with → 1191 v48_on_cast_line"}
- {"name": "(콜리 내부 TLS — 미검증)", "role": "소비자(콜리 경유)", "key": "-", "layout": "-", "invalidation": "-", "call_conditions": "프롬프트가 지목한 SIEGE_STANCE_CACHE(v47_siege_stance)·HP_VALUE_MEMO(champion_hp_value)·POS_EVAL_CACHE(position_eval_at)·INTER_CTX(interaction_score) 는 이 범위 IR 에 `LocalKey::with` 로 나타나지 않는다(원문 27592~36306 전체에서 with 호출은 CAST_BEAMS 1건). position_score_at_position(L390·L1178<518) / check_kill_die_tick(L449·451) / resolve_fight_stake(L478) / resolve_fight_stake_roster(L474) 가 내부에서 쓸 수 있으나 그 본문은 이 배치 범위 밖(자식 명세 계약) — 호출 순서만 위 항목에 적음"}

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::Vec<SmallActionPlay> (32B) | 반환값. ptr@0 · bump@+0x8 · cap@+0x10 · len@+0x18. L364 에서 ptr=8(dangling) bump=data.context.pool cap=len=0 으로 초기화(m02.ll:27759~27765) \| (배치 B) ptr@0 · bump@+8 · cap@+0x10 · len@+0x18. 배치 B 안의 채움 지점: L532 return 경로 %2791 `memcpy %0 ← %108(res) 32B`(m02.ll:35090). L536 은 res 에 extend 만 하고 sret 채움은 배치 C(L607/L613) \| (배치 C) ptr@0 · bump@+8 · cap@+0x10 · len@+0x18 — 배치 C 에서 %108(res) 를 memcpy 32B 로 채움(L607 m02.ll:36284 / L613 m02.ll:35966) | 4 |
| 1 | 1 | self | &mut BattleSubPlan (48B) | IR 속성 initializes((43,44),(45,46)) = +0x2b v48_dodge_claim · +0x2d last_bail_gate 가 무조건 쓰기 표면(L359/360). 그 외 조건부 쓰기: +0x2b=1(L510) · +0x2d=1(L417)/2(L484) · +0x20 v48_claim_hold_until(L511). 읽기: +0x10 goal@tag · +0x18 goal.focus · +0x20 · +0x29 with_dive(closure$2 env) \| (배치 B) initializes((43,44),(45,46)) = +0x2b v48_dodge_claim · +0x2d last_bail_gate(배치 A L359/360 이 0 으로 초기화). ★배치 B 의 &mut 쓰기 = bp:110 self.support_target ← Some(near_enemy.id): +0x0 태그 store i64 1 (m02.ll:29017) · +0x8 store id (m02.ll:29018). 읽기: +0x0/+0x8 support_target · +0x10 goal@tag · +0x18 goal.focus · +0x28 avoid_unnecessary_tower_trace · +0x2c tactic@tag \| (배치 C) initializes((43,44),(45,46)) = +0x2b v48_dodge_claim · +0x2d last_bail_gate 가 쓰기 표면. 배치 C 는 +0x2d 에 3 을 씀(L604). 읽기: +0x10 goal@tag · +0x18 goal.focus | 4 |
| 2 | 2 | version | usize | i64 %2 → 스택 %109 로 복사(27711). 이 범위 분기: version<2 → final_stand=false(L363) · L472 resolve_fight_stake 경로 선택 · 콜리(nontarget_windup_perceived·engage_pair_range·check_kill_die_tick 등)에 전달 \| (배치 B) L536 진입 시 %109 alloca 에서 재로드(%255, m02.ll:28143) → base_battle_action 1번째 인자 · SmallActionAroundHide::new / SmallActionAround::new 1번째 인자 · kite_reposition_point 1번째 인자 · bp:293 `version > 1` 분기(%1470 = icmp ugt %255, 1 · reach 가 version=2 로 접어 참 확정 → bp:320~345 사장) \| (배치 C) L539 base_battle_action 인자 · L593 check_kill_die_tick 인자. 분기 %112(version<2, L363 배치 A 산출)를 L544 에서 재사용 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | 직접 읽기 없음. check_kill_die_tick(L449/451)·fight_participants(L473)·resolve_fight_stake(L478) 에 전달(가변) \| (배치 B) L526 / bp:198 SmallActionAroundPosition::new_with_radius 1번째 인자 · bp:147 SmallActionAroundHide::new 2번째 인자 · (NA) bp:332/340 SmallActionAround::new. ⚠base_battle_action 호출엔 rnd 가 전달되지 않음(fastcc 5인자: version, player, data, goal_tag, focus — tcx 시그니처의 &mut StdRng 는 데드인자 제거) \| (배치 C) 배치 C 에선 L593 check_kill_die_tick 에 그대로 전달만 | 3 |
| 4 | 4 | player | &PlayerState (2528B) | readonly. info.team(+0x930) · info.position@tag(+0x9c0) · info.parameter(+0x180 → judge_accuracy) · info.id(+0x928, CAST_BEAMS 키) \| (배치 B) +0x930 info.team(%124, 배치 A 산출) · 모든 SmallAction 생성자·v21/v48 호출에 전달 · is_recent_visible 3번째 인자 · 클로저 s7_0/se_0 캡처 \| (배치 C) L539/L593/L606 인자 · 클로저 sf_0 캡처(+0x930 info.team) | 4 |
| 5 | 5 | data | &OperationData (24B) | readonly. cache(+0) · context(+0x8) · blackboard(+0x10, [Blackboard;2] 744B stride) \| (배치 B) +0 cache(%135 &AbstractGameWithCache: +0 game dyn 팻포인터(%417 data,%419 vtable) · +0x1e0 player_champion[2][5]) · +8 context(%118 &GameContext: +0x20 map → MapDef+0x6d70 fountains[team]) · +0x10 blackboard(&[Blackboard;2], 744B stride, 인덱스 = 1-player.team) \| (배치 C) +0 cache(&AbstractGameWithCache) · +8 context(&GameContext; +0 = bump) · +0x10 blackboard(&[Blackboard;2]) | 4 |
| 6 | 6 | parameter | &ScoreParameter (5384B) | readonly. player.applyed_damage(+0x988) · player.applyed_cc(+0x990) · positioning_score(+0x9f0, PositioningScoreData 2760B → position_score_at_position) \| (배치 B) 배치 B 에서는 v21_support_pressure_too_risky(bp:113) · kite_reposition_point(bp:196) 에 그대로 전달만. 필드 직접 읽기 없음 \| (배치 C) 배치 C 범위 안에서는 읽지 않음(%6 미사용) | 4 |
| 7 | 7 | team_plan | &TeamPlan (1064B) | DI 이름 team_plan(27718). IR 에 dereferenceable 없음(ptr 만). 이 범위: closure$3 env 로 저장(L430 env+0)→ ally_battle_stop_tick[p](+0x0, 16B stride) 읽기 · fight_participants(L473) 에 전달 \| (배치 B) 배치 B 범위 안에서 미사용 \| (배치 C) 배치 C 범위 안에서는 읽지 않음 | 4 |
| 8 | 8 | _debug | &mut DebugFrameData (224B) | DI 이름 _debug(27719). 이 함수 본문 직접 읽기/쓰기 없음 — check_kill_die_tick(L449/451)·fight_participants(L473)·resolve_fight_stake(L478) 에 그대로 전달(원문 전 범위 6회 등장 = define + 5회 인자 전달) \| (배치 B) 배치 B 범위 안에서 미사용 \| (배치 C) L593 check_kill_die_tick 8번째 인자로 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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
//                          eff.casting(+0x30 → +0x4f8 값) ∈ {1 Position, 2 Direction} && Effect::is_in_range(eff, c, champ)
//             5(Skill2) => eff = (c.level(+0x5c8) > 2 ? Some(&c.skill2_effect(+0x500)) : None).unwrap() ;   // entity.rs:1693 접근자 인라인
//                          casting(+0x530) ∈ {1,2} && is_in_range(eff, c, champ)
//             _ => false }
// L381  has_non_target_ult_range = enemies.iter_champions().any(closure$1) 인라인 [28007~28094]:
//         closure$1(c): nontarget_windup_perceived(...) && c.ty==13 && c.action_state==6(Ult) &&               // L382
//           eff = (c.level > 4 ? Some(&c.ult_effect(+0x538)) : None).unwrap() (tag +0x568) ;                     // L383 (entity.rs:1701)
//           casting ∈ {1,2} && is_in_range(eff, c, champ)                                                        // L384
// L390  current_position_score = position_score_at_position(version, player, data, &parameter.positioning_score(+0x9f0), champ.x(+0x660), champ.y(+0x668), PositionEvalPurpose::General(태그 2)) → %107 (PositioningScore 56B)
// L392  dodge_risk = cps.risk(+0)
// L393  force_runaway = dodge_risk > 9 && (has_non_target_ult_range || has_non_target_action_range || cps.on_trajectory(+0x30))   [28107~28117]
// L398  if !force_runaway:  tick = data.cache.game.tick() (vtable +0x28) ; if tick < self.v48_claim_hold_until(+0x20) {           [28120~28132]
// L399      force_runaway = cps.on_trajectory || cps.on_periodic_trajectory(+0x31)                                              [28135~28139]
// L400          || v48_on_cast_line(player, data, champ.radius_adj, champ.x, champ.y)   // radius_adj = radius_mult(+0x470)==0 ? radius(+0x680) : radius*(mult+100)/100 (entity.rs:1511~1515 인라인) [32352~32377]
//        }  else → L536 (배치 B)   // tick >= hold_until 이면 force_runaway 는 393 값 그대로(=false) → %254
// L401  if !force_runaway → L536 (배치 B, %254)
// ---- 이하 403~532 = force_runaway 블록(닫힘 532 에서 near_allies/near_enemies 드롭 후 536 으로 합류) ----
// L403  res.push(SmallActionPlay::RunAway(SmallActionRunAway::new_dodge(data, player, end_delay=5)))   // 태그 3 @+0xb1 [32386~32438]
// L411  v48_hard_pre = has_non_target_ult_range || dodge_risk > 24 || self.goal@tag(+0x10)==6(AssassinReady) || parameter.player.applyed_cc(+0x990) != 0
// L415  v48_hard = v48_hard_pre ? true : (max(champ.hp(+0x670),1)*20 <= parameter.player.applyed_damage(+0x988)*100)   // IR: %1861 = applyed_damage*100 < max(hp,1)*20 이 DW_OP_not 로 v48_hard [32457~32468]
// L416  if v48_hard && !final_stand {                                                                             // 계속 조건 = final_stand || !v48_hard
// L417      self.last_bail_gate = 1  (+0x2d)                                                                       [32504]
// L418      return res   // 회피 단일 후보 (memcpy %0←%108 [32505])
//       }
// L421  near_enemies(%104) = Vec::from_iter_in(cache.iter_champions(enemy).filter(closure$2), bump)   // aux m02.ll:76003 call_mut
//         closure$2(t) [env: version, data, champ, player, &self.with_dive(+0x29)]:
//           r = engage_pair_range(version, data, champ, t)                                    // L422
//           dist²(t, champ) <= (r + 30000)²                                                    // L423
//           && data.blackboard[1 - player.team].is_recent_visible(game, player, t)             // L424
//           && !is_ignored_battle_enemy(version, player, data, t, self.with_dive)              // L425
// L427  near_allies(%102) = Vec::from_iter_in((0..5).filter(closure$3).filter_map(closure$4), bump)   // aux m01.ll:24041 / m02.ll:76099
//         closure$3(p) [env: team_plan, cache, player, champ]: team_plan.ally_battle_stop_tick[p](+0, 16B stride) is None(태그 0)   // L428
//           && cache.player_champion[player.team][p].is_some_and(|c| dist²(c, champ) < 14400000001 (=120000²+1))                // L429
//         closure$4(p): cache.player_champion[player.team][p]   (Option<&Entity>)                                                // L430
// L431  nearest_enemy_tower = cache.iter_towers_without_nexus(enemy).min_by_key(closure$5 = dist²(t, champ))   // 첫 원소 인라인 + fold 호출 [32551~32743]
// L432   .filter(closure$6) [32761~32867]:
//          attack_range = Effect::range(t.attack_effect.unwrap(), caster=t) + champ.radius_adj + t.radius_adj      // L433 ; tag +0x4c0 == -1 → unwrap_failed
//             // Effect::range(caster)(effect.rs:26 인라인) = range(+0x4a0) + 15000 + caster.stat_buff_cached.range(+0x438) + (caster.level-1)*growth_range(+0x4a8)
//          keep iff Entity::distance(t, champ) <= attack_range                                                     // L434 (ugt → None)
// L441  focus = match self.goal@tag { 4 RunAway | 7 End => None, _ => Some(self.goal.focus(+0x18)) }
// L442  t = focus.and_then(|id| game.get_entity_by_id(id))   // vtable +0x1f0
// L443   .filter(|t| t.team(+0x0 tag, +0x8 Player.0) != champ.team)   // derive PartialEq: 태그 같고 (태그!=0 || payload 같음) 이면 같음 → 제외
// L444   .is_some_and(closure$9):
// L445      close_execute_range = max_range_cached(data, champ, t) + 25000
// L446      if dist²(t, champ) > close_execute_range² → false
// L447      if ready_damage_to_target(data.context, champ, t) < t.hp(+0x670) → false
// L449      t_die  = check_kill_die_tick(version, rnd, data, player, focus=t,     enemy=near_allies.clone(),  towers=Vec::new_in(bump)(L450),                 _debug)
// L451      my_die = check_kill_die_tick(version, rnd, data, player, focus=champ, enemy=near_enemies.clone(), towers=Vec::from_iter_in(nearest_enemy_tower, bump)(L452), _debug)
// L453      v48_kill_secure = t_die > my_die
// L456      if !v48_kill_secure → 블록 탈출(→ L532 드롭 → L536 배치 B)   // 그 외(None·범위밖·비치명·확보) → L463 계속   [33093~33095]
// L463  committed_dir: i8 = match self.goal@tag { 0 Trace|2 Kiting|5 Assassin|6 AssassinReady => 1 (L465), 3 KitingBack|4 RunAway => -1 (L466), _ (1 Protect|7 End) => 0 }
// L472  if version < 2 || near_enemies.len(+0x18) == 0 {
// L478      pred = resolve_fight_stake(version, rnd, data, player, champ, &near_allies[..], &near_enemies[..], committed_dir, nearest_enemy_tower, judge_accuracy(&player.info.parameter(+0x180)) (L479), _debug)
//       } else {
// L473      roster = fight_participants(version, rnd, data, player, champ, &near_allies[..], &near_enemies[..], team_plan, _debug)   // Vec<(&Entity,i64,bool)> 24B 원소
// L474      pred = resolve_fight_stake_roster(version, data, champ, &roster[..], &near_enemies[..], committed_dir, nearest_enemy_tower, judge_accuracy(...) (L475))
// L477      drop(roster)
//       }
// L478/474 v48_losing = pred.line(+0x38) == 2 (FightLine::Disengage)                                          [33268~33278]
// L483  if v48_losing { L484 self.last_bail_gate = 2 ; return res (%2791: memcpy + 드롭) }
// L491  anchor = (goal focus → get_entity_by_id → filter team != champ.team)                                    // 441~443 과 동일 사슬
// L493   .or_else(|| enemies.iter_champions().filter(|e| blackboard[1-team].is_recent_visible(game, player, e)).min_by_key(closure$12 = dist²(e, champ)))   // L494~495 [33518~33812]
// L496  if anchor.is_some() && champ.attack_effect.is_some() (tag +0x4c0 != -1) {
// L498      r = Effect::range_adjust(&champ.attack_effect, champ, anchor) + range(+0x4a0) + (champ.level-1)*growth(+0x4a8) + champ.stat_buff.range(+0x438) + champ.radius_adj + anchor.radius_adj
// L499      v48_in_attack_range = distance(champ.x, champ.y, anchor.x, anchor.y) <= r      // IR %2366 = dist > r (DW_OP_not)
// L504      if v48_in_attack_range && !cps.on_trajectory && !cps.on_periodic_trajectory
// L506         && !v48_on_cast_line(player, data, champ.radius_adj, champ.x, champ.y) {
// L510          self.v48_dodge_claim = true  (+0x2b store i8 1) [33960]
// L511          self.v48_claim_hold_until = game.tick() + data.context.setting(+0x8).tick_per_second(+0x12f8) * 2   // shl 1 [33961~33974]
//               → 블록 탈출(L532 드롭) → L536 배치 B
//           }  else → L518
//       } else → L518
// L518  kite = v48_kite_position(version, player, data, parameter, champ, focus_id (= 441 의 focus Option<usize>), escaping (= 배치 B L519~520 계산 %2430)) 인라인(battle.rs:1132~1210):
//         1134  anchor = focus_id.and_then(get_entity_by_id).filter(team != champ.team)
//         1136     .or_else(|| enemies recent_visible(blackboard[1-team]).min_by_key(dist²))  ; None → return None
//         1139  attack_effect = champ.attack_effect.as_ref()? (tag -1 → None)
//         1140  attack_range = range_adjust(attack_effect, champ, anchor) + range + (level-1)*growth + stat_buff.range + champ.radius_adj + anchor.radius_adj
//         1143  keep_range = attack_range.saturating_sub(8000)
//         1154  sets = [(anchor.x, anchor.y, keep_range), (anchor.x, anchor.y, attack_range+30000), (anchor.x, anchor.y, attack_range+70000), 0, 0] ; len 3     // 1155~1157
//         1160  if escaping { sets.rotate_right(2) ; sets[0]=(champ.x,champ.y,24000) ; sets[1]=(champ.x,champ.y,48000) ; len 5 }   // 1161~1163
//         1168  for (cx,cy,radius) in sets {  best = None
//         1173    for (dx,dy) in DIRS16 (anon.267: 16방향 ×1000 단위, 22.5° 간격) {
//         1174      len = max(isqrt(dx²+dy²), 1)
//         1175      px = cx + dx*radius/len ; py = cy + dy*radius/len                                       // 1176
//         1177      (px,py) = Game::adjust_position(data.context.map(+0x20), data.context.setting(+0x8), px, py)
//         1178      s = position_score_at_position(version, player, data, &parameter.positioning_score, px, py, General)
//         1180      if s.on_trajectory || s.on_periodic_trajectory → continue
//         1183      if v48_on_cast_line(player, data, champ_r, px, py) 인라인 → continue :
//                       beams = CAST_BEAMS.with(v48_cast_beams(player, data)) (L1002<1058<1183 ; TLS) → (n, [Beam;6]) ; n>6 → slice_index_fail
//                       any beam: kind(+0x20)==0 ? distance(p, (x1,y1)) <= halfwidth(+0x28) + champ_r + 15000     // 1061
//                                                : dist_to_line_segment(p, (x1,y1),(x2,y2)) <= halfwidth + champ_r + 15000   // 1064
//         1189      if !escaping { mid = ((px+champ.x)/2, (py+champ.y)/2) ; if v48_on_cast_line(player, data, champ_r, mid) → continue }   // 1190~1191 (out-of-line 호출)
//         1197      range_err = |distance(p, anchor) - keep_range| ; step = dist²(p, champ)                    // 1198
//         1199      best = min by (range_err, s.risk, step) 사전순  (1200~1201: e 같으면 risk 작은 것, risk 도 같으면 step 작은 것)
//                 }
//         1205    if best.is_some() → break → Some((x,y)) → 배치 B(줄 521)
//               }
//         1210  None → return res (%2791)
//       kite == None → return res   // %2501/%2524/%2527/%2642 → %2791

// battle.rs:519~536 (배치 B)
// 표기: `L<n>` = action_candidates 소스 줄, `bp:<n>` = base_positioning(battle.rs:72~357, L536 에 통째 인라인) 소스 줄(;L 사슬의 끝에서 두 번째). dist²(a,b) = |ax-bx|²+|ay-by|² (Entity+0x660/+0x668). radius(e) = e.radius(+0x680) 을 stat_buff_cached.radius_mult(+0x470)==0 이면 그대로, 아니면 *(100+mult)/100 (entity.rs:1509). Effect::range(e, caster) = e.range(+0x10) + caster.stat_buff_cached.range(+0x438) + e.growth_range(+0x18)*(caster.level-1) (effect.rs:26). bb = data.blackboard[1 - player.team] (744B stride).
// 공통 문맥(배치 A 산출): res=%108(bumpalo Vec, L364) · champ=%139 · team=%124 · enemy_team=%142=1-team · enemy 슬롯 %143..%144 = cache.player_champion[enemy_team][0..5] · current_position_score=%107(L390 position_score_at_position) · %233 = current_position_score.on_trajectory(L393 로드) · near_allies=%102(bumpalo Vec<&Entity>, 배치 A)

// ===== L519~L532 : 강제도주 블록(L401 force_runaway) 꼬리 =====
// L518(배치 A) = `if let Some((x,y)) = v48_kite_position(version, player, data, parameter, enemy, focus_id(%2406/%2407), <L519~520 bool>)` — 아래는 그 마지막 bool 인자와 Some 가지.
L519  on_traj = current_position_score.on_trajectory(%233) || current_position_score.on_periodic_trajectory(+0x31)   // 단락: 참이면 L520 평가 안 함 → 인자 = true
L520  else 인자 = v48_on_cast_line(player, data, radius(champ), champ.x, champ.y)   // ★TLS CAST_BEAMS 접점(콜리 내부) · 이 인자 평가가 L518 본체보다 먼저
      → 배치 A(L518, %2429 이후 v48_kite_position 인라인 본체)
L521  (L518 이 Some((x,y)) 로 끝나면 %2773) res.truncate(0)                       // 기존 후보 전부 폐기
L525  res.push(SmallActionPlay::AroundPosition(                                 // untagged variant, 184B memcpy, 필요 시 reserve_internal_or_panic
L526      SmallActionAroundPosition::new_with_radius(rnd, data, x, y, /*end_delay*/5, /*around_radius*/4000)))
      → %2791: sret ← res(32B memcpy) ; return   // 배치 A 의 다른 return 사이트(%2205,%2501,%2524,%2527,%2642)도 여기로 합류
L532  = near_allies(%102, bumpalo Vec<&Entity>) 스코프 종료 드롭 — 두 복사본: ①return 경로(%2791→%2810..) ②L511 뒤 폴스루(%2090→..→%2897→%254=L536). 판정 로직 없음: <Vec<&Entity> as Drop>::drop(원소 &Entity 라 no-op) 뒤 bumpalo dealloc 인라인(cap!=0 && bump.footer.ptr == vec.ptr 이면 footer.ptr += cap*8 로 되감기) + 언와인드용 cleanuppad 사본
L534  = %107(current_position_score) lifetime.end 2건뿐(스코프 닫는 `}`) — 로직 없음

// ===== L536 : res.extend(self.base_positioning(version, rnd, player, data, parameter)) =====
// 진입 %254 ← %245(L398 `now >= self.v48_claim_hold_until`) · %1825(L401 !force_runaway) · %2897(강제도주 블록 폴스루). 나가는 곳 %2898 Extend(res ← bp 결과) → %2901 = L537(배치 C)
// --- bp:73~95 real_focus_id ---
bp:73  match self.goal { Trace(f)|Protect(f)|Kiting(f)|KitingBack(f)|Assassin(f)|AssassinReady(f) => focus=f (태그 0,1,2,3,5,6) ; RunAway(4)|End(7) => real_focus_id = 0 로 %406 직행 }
bp:74  champ = cache.player_champion[team][pos].unwrap()   (None 이면 unwrap_failed)
bp:75  focus_e = game.get_entity_by_id(focus) ; None 이면 real_focus_id = focus 그대로(%407 = %259)
bp:76  if focus_e.team == champ.team {            // TeamType derive-eq: 태그 같고 (Player 면 페이로드 같음) — Protect 처럼 아군을 focus 로 둔 경우
bp:78     nearest = iter_champions(enemy_team).filter(|x| x.is_visible_from(champ))   // x.visible_state[champ.team]@tag == 0 Visible (Neutral 팀이면 필터 없음 — 실전 도달 불가)
bp:79                .min_by_key(|x| dist²(x, focus_e))                                  // 첫 원소 인라인 + Map::fold(closure#0/#1)
bp:80     real_focus_id = nearest.unwrap_or(focus_e).id
       } else { real_focus_id = focus_e.id }
bp:95  bp_res = Vec::new_in(context.bump)  (%81)
bp:99  champ = player_champion[team][pos].unwrap() 재로드 (%412)
// --- bp:100~114 support_target 추적(Trace 후보) ---
bp:100 if let Some(st) = self.support_target { if let Some(st_e) = game.get_entity_by_id(st) {
bp:101   near_enemy = if st_e.team == champ.team {
bp:105-106   iter_champions(enemy_team).filter(|x| bb.is_recent_visible(game, player, x)).min_by_key(|x| dist²(x, st_e))   // closure#3/#4
bp:109       (None 이면 이 블록 통째 skip → bp:120)
         } else { st_e }
bp:110   ★self.support_target = Some(near_enemy.id)      // &mut self 쓰기 +0x0=1, +0x8=id
bp:111   mr = max_range_cached(data, champ, near_enemy) + 25000       // TLS MAX_RANGE_CACHE(콜리 내부)
bp:112   if dist²(near_enemy, champ) > mr² {
bp:113     if !v21_should_skip_support_trace(version, player, data, parameter, champ, near_enemy)   // 인라인(battle.rs:1292): near_enemy.team==champ.team 이면 false, 아니면 v21_support_pressure_too_risky(player,data,parameter,champ,near_enemy)
bp:114       bp_res.push(legacy_battle_trace_action(data, near_enemy.id, self.avoid_unnecessary_tower_trace))   // Trace(14): start=game.tick(), goal=target 좌표(엔티티 없으면 0,0), margin 15000, end_delay 5, escape_commit 0, 4 bool 0, last_escape None
         } } }
// --- bp:120 goal 별 `other` 생성 (switch self.goal@tag) ---
bp:120 other = match self.goal {
  // Trace(0) | Assassin(5):
bp:123   base_battle = base_battle_action(version, player, data, goal_tag, focus)     // rnd 미전달(fastcc 5인자)
bp:126   focus_far = game.get_entity_by_id(real_focus_id).map_or(false, |f| {        // closure#5
bp:127       mr = support_min_action_range(champ, f) + 25000 ;
bp:128       dist²(f, champ) > mr² })
bp:131   has_focus_action = base_battle.iter().any(|a| matches!(a.tag, 15|16|17|18 /*Attack|Skill|Skill2|Ult*/) && a.target(+0x8) == real_focus_id)   // closure#6 인라인(assume tag!=10)
bp:133   if has_focus_action && !focus_far { bp:134 Vec::new() }          // 이미 focus 를 때리는 액션이 있고 가까움 → 위치 후보 없음
bp:135   else if self.tactic == BacklineDPS(2) { bp:136-137 vec![legacy_attack_range_trace_action(data, real_focus_id, self.avoid_unnecessary_tower_trace)] }   // Trace(14) + attack_range_only=1
bp:140   else { bp:140-141 vec![legacy_battle_trace_action(data, real_focus_id, self.avoid_unnecessary_tower_trace)] }
bp:144   drop(base_battle)   // ★탐침용 — 여기서 extend 하지 않음(battle 액션은 배치 C L539 가 다시 base_battle_action 으로 붙임)
  // AssassinReady(6):
bp:146-148 vec![SmallActionPlay::AroundHide(SmallActionAroundHide::new(version, rnd, data, player, real_focus_id, 5))]   // 태그 6
  // Protect(1):
bp:152   if self.avoid_unnecessary_tower_trace { bp:153 vec![legacy_battle_trace_action(data, real_focus_id, true)] }
bp:155   else { vec![legacy_battle_trace_action(data, real_focus_id, false)] }    // = 인자 그대로 전달, 두 줄로 갈라짐
  // Kiting(2):
bp:160   base_battle = base_battle_action(version, player, data, 2, focus)
bp:162   if base_battle.is_empty() {
bp:165-167  nearest_enemy = iter_champions(enemy_team).filter(|x| bb.is_recent_visible(game,player,x) && dist²(x,champ) < 200000²+1).min_by_key(|x| dist²(x,champ))   // closure#7/#8
bp:169     if let Some(ne) = nearest_enemy {
bp:170       attack_range = Effect::range(champ.attack_effect().unwrap(), champ) + attack_effect.range_adjust(champ, ne)
bp:171                      + radius(champ) + radius(ne)
             max_range = attack_range
bp:173       if champ.can_skill() && champ.skill_effect.unwrap().target.check(champ, ne) {
bp:174-176      skill_range = Effect::range(skill_effect, champ) + skill_effect.range_adjust(champ,ne) + radius(champ) + radius(ne) ; max_range = max(skill_range, max_range) }
bp:180       if champ.can_skill2() && champ.skill2_effect().unwrap().target.check(champ, ne) {     // skill2_effect(): level<=2 면 None → unwrap 패닉 경로
bp:181-183      skill2_range = (동일식, skill2_effect) ; max_range = max(skill2_range, max_range) }
bp:186       d = dist²(ne, champ)
bp:189       if d <= max_range² && game.is_visible(team, ne.id) {          // vtable +0xf8
bp:190-191     other = vec![SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5))]
bp:196         (kx,ky) = kite_reposition_point(version, player, data, parameter, champ, ne.x, ne.y, attack_range /*max_range 아님*/)
bp:197-198     other.push(SmallActionPlay::AroundPosition(SmallActionAroundPosition::new_with_radius(rnd, data, kx, ky, 5, 24000)))
             } else {   // 사거리 밖이거나 비가시
bp:204         if self.avoid_unnecessary_tower_trace { bp:205 other = vec![legacy_battle_trace_action(data, ne.id, true)] } else { bp:207 vec![...(data, ne.id, false)] }   // bp:203 push
             }
           } else {   // nearest_enemy None
bp:214       f = game.get_entity_by_id(real_focus_id)
bp:215-216   chase = f.map_or(false, |t| t.team != champ.team && bb.is_recent_visible(game, player, t))   // closure#9(s7_0, aux m02.ll:4435)
bp:217       if chase { bp:219 if self.avoid_unnecessary_tower_trace { bp:220 vec![legacy_battle_trace_action(data, real_focus_id, true)] } else { bp:222 vec![...false] } }   // bp:218 push
bp:226-227   else { other = vec![SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5))] }
           }
         } else { bp:233 other = Vec::new() }     // base_battle 비어있지 않으면 위치 후보 없음
bp:235   drop(base_battle)
  // KitingBack(3):
bp:237   base_battle = base_battle_action(version, player, data, 3, focus)
bp:239   other = if base_battle.is_empty() { bp:240-241 vec![RunAway(SmallActionRunAway::new(data, player, 5))] } else { bp:245 Vec::new() }
bp:247   drop(base_battle)
  // RunAway(4):
bp:250   towers = cache.iter_towers_without_nexus(team)      // Chain<Flatten<[Option<&Entity>;6]>, Copied<Iter<&Entity>>> 120B 이터레이터
bp:251-252 nearest_tower = towers.filter(|t| t.can_target())   // closure#10: +0x6b9 can_target && +0x6a0 block_target_tick==0
                                .min_by_key(|t| dist²(t, champ))  // closure#11
bp:254   bp_res2 = Vec::new()  (%57)
bp:259-260 has_move_ult = champ.ult_effect().map_or(false, |u| u.ty.expected_move_distance().is_some() && matches!(u.casting, Position(1)|Direction(2)))   // ult_effect(): level>4 아니면 None · EffectType vtable +0x58
bp:262   if has_move_ult && champ.can_ult() {
bp:263     hp_ratio = champ.hp*100 / champ.stat_cached.hp     (max 0 → div_by_zero 패닉)
bp:265-266 visible_enemies = iter_champions(enemy_team).filter(|e| bb.is_recent_visible(game,player,e) && dist²(e,champ) < 150000²+1).count()   // closure#13
bp:269-270 nearby_allies = iter_champions(team).filter(|a| a.id != champ.id && dist²(a,champ) < 150000²+1).count()   // closure#14 (5슬롯 언롤)
bp:273     is_desperate = visible_enemies < nearby_allies + 2        // DI 이름 그대로(극성은 IR 기준)
           if !(hp_ratio > 40 && is_desperate) {
bp:276       bp_res2.push(RunAway(SmallActionRunAway::new_with_ult(data, player, 5))) } }
bp:281   (lx,ly,rx,ry) = context.map.fountains[team] ; bp:282 cx=(lx+rx)/2, cy=(ly+ry)/2
bp:285   to_heal_area = dist²(champ, (cx,cy))
bp:286   if to_heal_area < 300000²+1 { bp:287 bp_res2.push(RunAway(SmallActionRunAway::new(data, player, 5))) }
bp:290   if let Some(tw) = nearest_tower {
bp:291     d_t = dist²(tw, champ) ; if d_t >= 150000² || tw.hp*100/tw.stat_cached.hp < 21 || to_heal_area < 300000²+1 {
bp:292       bp_res2.push(RunAway(SmallActionRunAway::new(data, player, 5))) }
bp:293     else if version > 1 {          // ★reach 접힘 참(version=2)
bp:300       bp_res2.push(RunAway(SmallActionRunAway::new(data, player, 5)))
bp:301       if d_t < 50000²+1 {
bp:302-303     tower_attack_range = tw.attack_effect().map(|e| Effect::range(e, tw) + radius(tw)).unwrap_or(0)    // closure#17
bp:305-309     enemy_in_tower_range = iter_champions(enemy_team).any(|c| bb.is_recent_visible(game,player,c) && dist²(tw,c) <= (tower_attack_range + radius(c))²)   // closure#16 aux m02.ll:3931
bp:312         if enemy_in_tower_range { bp:314 bp_res2.extend(base_battle_action(version, player, data, 3 /*KitingBack*/, 0)) } } }
bp:320     else { // NA(version<2 전용) bp:320~345: d_t<50000² 이면 타워 사거리 계산 후 AroundRunAway(SmallActionAround::new(..tw.id,5), escape_mode=1) push · 적 사거리 안이면 base_battle_action(3,0) extend · 아니면 bp:340~342 AroundRunAway push }
         } else { bp:346 bp_res2.push(RunAway(SmallActionRunAway::new(data, player, 5))) }
bp:349   other = bp_res2
  // End(7):
bp:351   todo!() → panic("not yet implemented")
         }
bp:354 bp_res.extend(other)   (%627 → %2898)
bp:356 return bp_res  → L536 res.extend(bp_res) → 배치 C(L537)

// battle.rs:537~614 (배치 C)
// 진입: 배치 B(L536 거대 인라인 문)의 마지막 블록 %2898 → %2901. 이 시점의 값: res=%108(bumpalo Vec<SmallActionPlay>, L364 에서 new_in(bump)) · champ=%139=cache.player_champion[team][pos](L367, None 이면 배치 A 에서 이미 unwrap 패닉) · team=%124=player.info.team · enemy_team=%142=1-team · final_stand=%116=(version>=2 && nexus_final_stand(player,data)) (L363) · %112=(version<2) (L363)

// L537~539 기본 전투 행동
if self.goal@tag != 6 (AssassinReady) {                                   // m02.ll:35463
    // L539: goal 은 (tag %2902, focus %2907) 두 스칼라로 전달. rnd 는 _rnd(미사용, ArgumentPromotion 으로 IR 인자에서 제거)
    let v: Vec<SmallActionPlay> = base_battle_action(version, player, data, self.goal);   // sret %94 · m02.ll:35473
    res.extend(v);                                                         // Extend::extend(res, v.ptr, v.len) m02.ll:35480
}

// L544~554 적 타워 버스트 후보 (version>=2 전용)
if !(version < 2) /*%112*/ && !matches!(self.goal, RunAway(4) | AssassinReady(6)) {   // m02.ll:35467 → 35500 switch
    // L546: 적 타워(넥서스 제외) 이터레이터 = Chain<Flatten<IntoIter<Option<&Entity>,6>>, Copied<Iter<&Entity>>> (120B sret %93)
    let towers = data.cache.iter_towers_without_nexus(enemy_team /*%142*/);
    // L547 filter closure$13(sb_0): t.can_target()  ≡  t.+0x6b9 can_target && t.+0x6a0 block_target_tick == 0   (entity.rs:1478 인라인 m02.ll:35641~35647 · aux 76137)
    // L548 min_by_key closure$14(sc_0): key = dx²+dy², dx=|t.x-champ.x|, dy=|t.y-champ.y|  (m02.ll:35734~35746 · fold 본체 aux m06 28077)
    let nearest: Option<(u64,&Entity)> = towers.filter(sb_0).min_by_key(sc_0);          // {i64,ptr} %2986 · ptr==null ⇒ None (m02.ll:35762)
    if let Some(tower) = nearest {                                          // L549
        // L549 champ.attack_effect.as_ref().is_some_and(closure$15):  +0x4c0 tag == -1 ⇒ None ⇒ false (m02.ll:35770)
        //   closure$15 (L550~551):
        //     r = attack.range + 30000 + champ.stat_buff_cached.range + (champ.level - 1) * attack.growth_range        // effect.rs:26 인라인 m02.ll:35874~35878
        //         + Effect::range_adjust(attack, champ, tower)                                                        // m02.ll:35800 (game_core 호출)
        //         + radius(champ) + radius(tower)      // radius(e) = e.radius_mult==0 ? e.radius : e.radius*(100+e.radius_mult)/100  (entity.rs:1511~1515)
        //     in_my_reach = !( dist²(tower, champ) > r*r )   // dbg: in_my_reach = DW_OP_not(%3063) · 분기: dist²>r² 이면 L560 으로 skip (m02.ll:35889~35891)
        if in_my_reach && v3_tower_burst_feasible(player /*team %124*/, data /*cache %135, context %118*/, tower) {   // L553 m02.ll:35894 · && 단락: in_my_reach 거짓이면 feasible 미호출
            res.push(SmallActionPlay::Attack(SmallActionAttack::new(data, tower.id /*+0x5c0*/)));   // L554 · 태그 15 @+0xb1 (m02.ll:35905~35913)
        }
    }
}

// L560 조기 종료
if matches!(self.goal, RunAway(4) | AssassinReady(6)) { return res; }      // m02.ll:35493 switch → %3087(L613 memcpy sret) · version<2 경로(%2913)도 같은 switch 를 탄다

// L561~564 이미 전투 행동이 있으면 종료
let has_battle_action = res.iter().any(|a| matches!(a.tag@+0xb1, 15|16|17|18 /*Attack|Skill|Skill2|Ult*/));   // (tag-15) <u 4 · 원소 stride 184 (m02.ll:35952~35962) · llvm.assume(tag != 10) 은 니치 미사용값 힌트(판정 아님)
if has_battle_action { return res; }                                       // L564 → %3087

// L566~570 근접 적 챔피언 수집 (aux m01 35649 from_iter_in · 술어 aux m02 76161 sf_0)
let near_enemies: Vec<&Entity> = cache.iter_champions(enemy_team)  // 슬라이스 = player_champion[enemy_team][0..5] (%143..%144) · filter_map(Option 풀기)
    .filter(|e| data.blackboard[1 - player.info.team].is_recent_visible(cache.game.data, cache.game.vtable, player, e)   // L568 · 패닉가드: 1-team <u 2 아니면 bounds_check
             && dist²(e, champ) < 40000000001 /*200000²+1*/)                                                              // L569 · && 단락(visible 거짓이면 거리 미계산)
    .collect_in(bump);                                                     // L570 sret %90
if near_enemies.len() == 0 { drop(near_enemies); return res; }             // L572 (m02.ll:36012) → L610 drop → %3087

// L574~603 모든 근접 적이 '치명' 인가
let mut all_targets_lethal = true;                                         // L574 dbg i8 1
for target in near_enemies.iter() {                                        // L574 (%3149 루프)
    let my_range = max_range_cached(data, champ, target);                  // L575 m02.ll:36177 (TLS MaxRangeCache)
    // L578~585 attackers = near_enemies 중 target 에게 '합산 사거리' 안에 있는 적 (aux sg_0 m02 76238 · collect aux m01 29447 · 캡처 {data, champ, &my_range, target})
    let attackers: Vec<&Entity> = near_enemies.iter().filter(|e| {
            let e_range = max_range_cached(data, e, champ);                // L581 (TLS MaxRangeCache)
            let combined_range = my_range + e_range;                       // L582
            dist²(e, target) <= combined_range * combined_range            // L583 (icmp ule)
        }).copied().collect_in(bump);                                      // L585 sret %87
    if attackers.len() == 0 { all_targets_lethal = false; drop(attackers); break; }   // L588 → L601 dbg all_targets_lethal=0 → L603
    // L593~595: attackers 를 값으로 넘김(memcpy %85) · towers = Vec::new_in(bump) 빈 Vec(%84, L595 · ptr=8 dangling, cap=len=0)
    let die_tick = check_kill_die_tick(version, rnd, data, player /*judger*/, champ /*focus*/, attackers /*enemy*/, Vec::new_in(bump) /*towers*/, _debug);   // m02.ll:36242 (TLS DieTickCache)
    if die_tick != 0 { /*all_targets_lethal = false — dbg 없음, 분기 효과 동일*/ break; }   // L597 (m02.ll:36254) → %3170 → L603
    // die_tick == 0 ⇒ 다음 target                                          // %3173 → %3149
}
// L603
if all_targets_lethal && !final_stand /*%116*/ {                            // m02.ll:36181 — all_targets_lethal 거짓 경로는 컴파일러가 %3103(drop→return) 으로 직결
    self.last_bail_gate = 3;                                               // L604 +0x2d (m02.ll:36263)
    res.truncate(0);                                                       // L605 (m02.ll:36265) — 기존 후보 전부 폐기
    res.push(SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5 /*end_delay*/)));   // L606 · 태그 3 (m02.ll:36271~36279)
    // L607 return res (memcpy sret m02.ll:36284) → L610 drop(near_enemies) → ret
}
// L610 drop(near_enemies) → L613 return res → L614 ret
// 언와인드(%127 → L614): res 가 sret 로 이동된 뒤(%3178)면 drop 생략, 그 외 경로면 res drop_glue(%3183)
```

**`mem` 메모리 접근 111건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | BattleSubPlan | 0x10 | goal@tag | r | L411(==6 AssassinReady) · L441/L491/L518(4 RunAway\|7 End → focus 없음) · L463(committed_dir) \| (배치 B) bp:73 switch(0..7, m02.ll:28186) · bp:120 switch(m02.ll:28802) · bp:123 base_battle_action goal_tag 인자 \| (배치 C) L537 ==6(AssassinReady) · L544/L560 switch {4 RunAway, 6 AssassinReady} (m02.ll:35462, 35499, 35488) | 4 | OK |  |
| 1 | BattleSubPlan | 0x18 | goal.focus | r | L441/491/518 Some(focus) 페이로드(usize id) \| (배치 B) bp:73 focus(변종 0,1,2,3,5,6 공통 페이로드) · base_battle_action 5번째 인자 | 4 | OK |  |
| 2 | BattleSubPlan | 0x20 | v48_claim_hold_until | r | L398 tick < hold_until | 4 | OK |  |
| 3 | BattleSubPlan | 0x29 | with_dive | r | L421 closure$2 env+32 로 &self.with_dive 저장 → L425 is_ignored_battle_enemy 마지막 인자(aux m02.ll:76086~76089) | 4 | OK |  |
| 4 | PlayerState | 0x930 | info.team | r | L367 champ 인덱스 · L370 enemy=1-team · closure$2 L424 · closure$3/4 L429/430 \| (배치 B) 클로저 s7_0/se_0 안에서 재로드(1-team 으로 blackboard 인덱스, 배열길이 2 bounds check) \| (배치 C) aux sf_0 L568(m02.ll:76174) blackboard[1-team] 인덱스 · 본체 L367(%124, 배치 A) 재사용 | 4 | OK |  |
| 5 | PlayerState | 0x9c0 | info.position@tag | r | L367 i32 → zext 인덱스 | 4 | OK |  |
| 6 | PlayerState | 0x180 | info.parameter | r | L475/L479 judge_accuracy(&player.info.parameter) | 4 | OK |  |
| 7 | PlayerState | 0x928 | info.id | r | L518→1001 CAST_BEAMS 키 pid | 4 | OK |  |
| 8 | OperationData | 0x0 | cache | r | &AbstractGameWithCache \| (배치 B) %135. bp:74/99 player_champion 재로드, get_entity_by_id/tick/is_visible vtable 호출 수신자 | 4 | OK |  |
| 9 | OperationData | 0x8 | context | r | &GameContext \| (배치 B) %118. bp:95 bump(+0) · bp:281 map(+0x20) | 4 | OK |  |
| 10 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — [1 - player.team] (744B stride gep) 을 is_recent_visible 에 전달(L494·L1137<518·closure$2 L424) \| (배치 B) bp:105/165/265/306 등 `&blackboard[1-team]`(744B stride) | 4 | OK |  |
| 11 | GameContext | 0x0 | pool | r | L364/L450/L452 bumpalo Bump | 4 | OK |  |
| 12 | GameContext | 0x8 | setting | r | L511 tick_per_second · L1177<518 adjust_position 인자 | 4 | OK |  |
| 13 | GameContext | 0x20 | map | r | L1177<518 Game::adjust_position 첫 인자(추정: GameContext 레이아웃상 +0x20=map) \| (배치 B) bp:281 (m02.ll:31289) | 4 | OK |  |
| 14 | GameSetting | 0x12f8 | tick_per_second | r | L511 hold_until = tick + tps*2 | 4 | OK |  |
| 15 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame: data@0, vtable@+0x8) | r | vtable +0x20 seed(L999<518) · +0x28 tick(L398·L511·L1000<518) · +0x1f0 get_entity_by_id(L442·L491·L1134<518) | 4 | OK |  |
| 16 | AbstractGameWithCache | 0x1e0 | player_champion[[Option<&Entity>;5];2] | r | L367 [team][pos] · L370/381/494/1137 적 5슬롯 슬라이스 · closure$3/4 [team][p] | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 17 | Entity | 0x0 | team@tag | r | L443/492/1135 team != champ.team (derive PartialEq) | 4 | OK |  |
| 18 | Entity | 0x8 | team@Player.0 | r | 위 비교 페이로드 \| (배치 B) 팀 인덱스. bp:78 visible_state 인덱스로도 사용(bounds check <2) | 4 | OK |  |
| 19 | Entity | 0x68 | ty@tag | r | ==13 Champion (L371/382) | 4 | OK |  |
| 20 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | 4 Skill/5 Skill2 (L372) · 6 Ult (L382) | 4 | OK |  |
| 21 | Entity | 0x438 | stat_buff_cached.range | r | Effect::range/range 합산 (L433·L498·L1140) \| (배치 B) Effect::range(effect.rs:26) 합산항 \| (배치 C) L550 사거리 가산 | 4 | OK |  |
| 22 | Entity | 0x470 | stat_buff_cached.radius_mult | r | radius_adj = mult==0 ? radius : radius*(mult+100)/100 (entity.rs:1511~1515 인라인, L400/433/498/506/1141) | 4 | OK |  |
| 23 | Entity | 0x4a0 | attack_effect@Some.0.range | r | L433/498/1140 | 4 | OK |  |
| 24 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | (level-1)*growth | 4 | OK |  |
| 25 | Entity | 0x4c0 | attack_effect@tag (Option 니치 -1=None) | r | L433 unwrap(패닉) · L496/L1139 is_some | 4 | OK |  |
| 26 | Entity | 0x4c8 | skill_effect@Some.0 | r | L372 Effect 시작(is_in_range 인자) | 4 | OK |  |
| 27 | Entity | 0x4f8 | skill_effect@tag / Some.0.casting | r | i32: -1=None → unwrap_failed · 1 Position/2 Direction = 논타겟 | 4 | OK |  |
| 28 | Entity | 0x500 | skill2_effect@Some.0 | r | L374 level>2 일 때만 | 4 | OK |  |
| 29 | Entity | 0x530 | skill2_effect@tag / casting | r | L374 — gep +0x500 뒤 +48(0x30) 2단 접근이라 본문에 1328 리터럴 없음(C3 경고 사유) | 4 | OK |  |
| 30 | Entity | 0x538 | ult_effect@Some.0 | r | L383 level>4 일 때만 | 4 | OK |  |
| 31 | Entity | 0x568 | ult_effect@tag / casting (0x538+0x30) | r | L383 — gep +0x538 뒤 +48 2단 접근이라 본문에 1384 리터럴 없음(C3 경고 사유) | 4 | OK |  |
| 32 | Entity | 0x5c8 | level | r | skill2/ult 접근자 게이트(>2/>4) · (level-1)*growth \| (배치 B) Effect::range 의 growth×(level-1) · skill2/ult 접근자 게이트(>2, >4) \| (배치 C) L550 | 4 | OK |  |
| 33 | Entity | 0x660 | x | r | 거리 계산 전반 \| (배치 B) dist² 계산 전역 \| (배치 C) L548 키 dist²(tower,champ) · L551 · sf_0 L569 · sg_0 L583 | 4 | OK |  |
| 34 | Entity | 0x668 | y | r | 동상 | 4 | OK |  |
| 35 | Entity | 0x670 | hp | r | L415 max(hp,1)*20 · L447 ready_damage < t.hp \| (배치 B) bp:263/291 hp*100/max | 4 | OK |  |
| 36 | Entity | 0x680 | radius | r | radius_adj \| (배치 B) Entity::radius() 기본값 \| (배치 C) L550 champ·tower 양쪽 | 4 | OK |  |
| 37 | Effect | 0x10 | range | r | Entity+0x4a0 과 동일 지점(Entity 기준으로도 적음) | 4 | OK |  |
| 38 | Effect | 0x18 | growth_range | r |  | 4 | OK |  |
| 39 | Effect | 0x30 | casting (CastingType) | r | 0 Targeting/1 Position/2 Direction/3 None; Option<Effect> 니치 -1 | 4 | OK |  |
| 40 | ScoreParameter | 0x988 | player.applyed_damage | r | L415 v48_hard: applyed_damage*100 >= max(hp,1)*20 | 4 | OK |  |
| 41 | ScoreParameter | 0x990 | player.applyed_cc | r | L411 != 0 → v48_hard | 4 | OK |  |
| 42 | ScoreParameter | 0x9f0 | positioning_score (PositioningScoreData 2760B) | r | position_score_at_position 4번째 인자(L390·L1178<518) | 4 | OK |  |
| 43 | PositioningScore(sret 56B, %107/%19) | 0x0 | risk | r | dodge_risk(L392) · kite best 비교(L1200) | 4 | OK |  |
| 44 | PositioningScore(sret 56B, %107/%19) | 0x30 | on_trajectory | r | L393/399/504/1180 (gep 48) | 4 | OK |  |
| 45 | PositioningScore(sret 56B, %107/%19) | 0x31 | on_periodic_trajectory | r | L399/504/1180 (gep 49) | 4 | OK |  |
| 46 | FightPrediction(sret 64B, %97/%98) | 0x38 | line (FightLine) | r | ==2 Disengage → v48_losing (L478/474) | 4 | OK |  |
| 47 | TeamPlan | 0x0 | ally_battle_stop_tick[p]@tag (16B stride) | r | closure$3 L428: 태그 0(None) 이면 통과 (aux m01.ll:24141~24146) | 4 | OK |  |
| 48 | bumpalo Vec<&Entity> (near_allies %102 / near_enemies %104 / roster %99) | 0x0 | ptr | r | 슬라이스 전달(L473/474/478) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 49 | bumpalo Vec<&Entity> (near_allies %102 / near_enemies %104 / roster %99) | 0x18 | len | r | L472 near_enemies.len()==0 · 슬라이스 len | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 50 | CAST_BEAMS Beam(48B, with sret 296B 의 +0x8 부터) | 0x20 | kind | r | 0=점 else 선분 (L1060<1183<518) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 51 | CAST_BEAMS Beam(48B, with sret 296B 의 +0x8 부터) | 0x28 | halfwidth | r | + champ_r + 15000 과 거리 비교 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 52 | CAST_BEAMS Beam(48B, with sret 296B 의 +0x8 부터) | 0x18 | y2 (x1@0,y1@8,x2@16) | r | 선분 끝점 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 53 | BattleSubPlan | 0x0 | support_target@tag | r | bp:100 `if let Some(x)=self.support_target`(trunc i64→i1, m02.ll:28775/28784) | 4 | OK |  |
| 54 | BattleSubPlan | 0x8 | support_target@Some.0 | r | bp:100 get_entity_by_id 인자(m02.ll:28792) | 4 | OK |  |
| 55 | BattleSubPlan | 0x28 | avoid_unnecessary_tower_trace | r | bp:114/135~141/152/204/219 Trace 의 avoid_unnecessary_tower 값·분기(gep 40) | 4 | OK |  |
| 56 | BattleSubPlan | 0x2c | tactic@tag | r | bp:135 `== BacklineDPS(2)`(m02.ll:32191~32196) | 4 | OK |  |
| 57 | MapDef | 0x6d70 | fountains[team] (lx,ly,rx,ry) 32B stride | r | bp:281 gep 28016 + team*32 (m02.ll:31292~31303) | 4 | OK |  |
| 58 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame 팻포인터) | r | vtable 슬롯: +0x28 tick(bp:114 등 start_tick) · +0xf8 is_visible(team,id)(bp:189) · +0x1f0 get_entity_by_id(bp:75/100/114/126/214) | 4 | OK |  |
| 59 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | %138 슬롯 재로드(bp:74 %262, bp:99 %412) · [1-team] 5슬롯 = 적 챔프 순회(%143..%144) · [team] = bp:270 아군 순회(%137) | 4 | OK |  |
| 60 | Entity | 0x0 | team@tag (TeamType: 0 Player(usize)/1 Neutral) | r | bp:76/101/113/215 derive PartialEq: 태그 같고 (태그 0 이면 +0x8 페이로드 같음) | 4 | OK |  |
| 61 | Entity | 0x38 | visible_state[team]@tag (24B stride, 0=Visible) | r | bp:78 Entity::is_visible_from(champ)(entity.rs:1481): x.visible_state[champ.team]==Visible | 4 | OK |  |
| 62 | Entity | 0x470 | stat_buff_cached.radius_mult(i32) | r | Entity::radius()(entity.rs:1509~1515): 0 이면 radius 그대로, 아니면 radius*(100+mult)/100 \| (배치 C) L550 Entity::radius()(entity.rs:1511~1515) 인라인: 0 이면 radius 그대로, 아니면 radius*(100+mult)/100 — champ·tower 양쪽 | 4 | OK |  |
| 63 | Entity | 0x490 | attack_effect (Option<Effect>, 태그 @+0x4c0 i32 -1=None) | r | bp:170 unwrap(None 이면 unwrap_failed 패닉) · bp:302/322 map | 4 | OK |  |
| 64 | Entity | 0x4a0 | attack_effect.range | r | Effect::range 항 \| (배치 C) L550 (effect.rs:26 range() 인라인) | 4 | OK |  |
| 65 | Entity | 0x4a8 | attack_effect.growth_range | r | × (level-1) \| (배치 C) L550 (level-1)*growth_range | 4 | OK |  |
| 66 | Entity | 0x4c8 | skill_effect (태그 @+0x4f8 -1=None · target@+0x4f0 · range +0x4d8 · growth +0x4e0) | r | bp:173~176 can_skill && target.check(champ,ne) → skill_range | 4 | OK |  |
| 67 | Entity | 0x500 | skill2_effect (Entity::skill2_effect(): level>2 일 때만 &self.skill2_effect, 아니면 정적 None @anon.19) | r | bp:180~183 (태그 @+0x530 · target @+0x528 · range +0x510 · growth +0x518) | 4 | OK |  |
| 68 | Entity | 0x538 | ult_effect (Entity::ult_effect(): level>4 일 때만, 아니면 정적 None) | r | bp:259~260 ty.expected_move_distance().is_some() && casting@+0x568 ∈ {1 Position, 2 Direction} | 4 | OK |  |
| 69 | Entity | 0x5c0 | id | r | real_focus_id / near_enemy.id / nearest_enemy.id / tower.id / 아군 자기제외(bp:269) \| (배치 C) L554 SmallActionAttack::new(data, tower.id) | 4 | OK |  |
| 70 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | bp:263 champ hp% · bp:291 tower hp% — 0 이면 panic_const_div_by_zero | 4 | OK |  |
| 71 | Entity | 0x6a0 | block_target_tick | r | closure#10 can_target(): ==0 | 4 | OK |  |
| 72 | Entity | 0x6b9 | can_target(bool) | r | closure#10 \| (배치 C) L547 필터 = Entity::can_target()(entity.rs:1478) 인라인: can_target && block_target_tick==0 (m02.ll:35641 · aux 76137) | 4 | OK |  |
| 73 | Effect | 0x30 | casting@tag (CastingType) | r | bp:260 `(tag-1) <u 2` = Position\|Direction. Effect+0x28 target(CastingTarget) 은 check() 인자 | 4 | OK |  |
| 74 | PositioningScore | 0x30 | on_trajectory | r | L519 (%233, 배치 A L393 에서 로드된 값 재사용) | 4 | OK |  |
| 75 | PositioningScore | 0x31 | on_periodic_trajectory | r | L519 (m02.ll:33980~33983) | 4 | OK |  |
| 76 | bumpalo Vec<SmallActionPlay> | 0x18 | len | r | bp:131 base_battle.iter() · bp:162/239 is_empty · bp:354 extend | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 77 | SmallActionPlay | 0xb1 | 태그 | r | bp:131 `tag-15 <u 4` = Attack(15)\|Skill(16)\|Skill2(17)\|Ult(18) (assume tag != 10) | 4 | OK |  |
| 78 | SmallActionPlay | 0x8 | Attack/Skill/Skill2/Ult.target | r | bp:131 == real_focus_id (SmallActionAttack 등 +0x8 target) | 4 | OK |  |
| 79 | BattleSubPlan | 0x18 | goal.focus(usize) | r | L539 base_battle_action 6번째 인자(goal 을 tag+focus 두 스칼라로 전달, m02.ll:35472) | 4 | OK |  |
| 80 | OperationData | 0x0 | cache(&AbstractGameWithCache) | r | L546 iter_towers_without_nexus self · L553/L568 캡처 | 4 | OK |  |
| 81 | OperationData | 0x8 | context(&GameContext) | r | L553 v3_tower_burst_feasible 3번째 인자(%118) · context+0 = bump(%119, L595 Vec::new_in) | 4 | OK |  |
| 82 | OperationData | 0x10 | blackboard(&[Blackboard;2]) | r | L568 클로저 캡처(m02.ll:35984) → sf_0 안 blackboard[1-team].is_recent_visible | 4 | OK |  |
| 83 | AbstractGameWithCache | 0x0 | game.data_ptr(&dyn AbstractGame) | r | L568 캡처(m02.ll:35981) → is_recent_visible 2번째 인자 | 4 | OK |  |
| 84 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | L568 캡처(%418 = cache+8 · m02.ll:35982) → is_recent_visible 3번째 인자 | 4 | OK |  |
| 85 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (5×2 Option<&Entity>) | r | L566 iter_champions 대상 슬라이스 = [enemy_team] 5칸(%143~%144, 배치 A L370 산출) · 내 챔피언 %139 = [team][position] | 4 | OK |  |
| 86 | Entity | 0x6a0 | block_target_tick(usize) | r | L547 ==0 | 4 | OK |  |
| 87 | Entity | 0x4c0 | attack_effect@tag(i32 니치, -1=None) | r | L549 champ.attack_effect.as_ref() (m02.ll:35769) | 4 | OK |  |
| 88 | Entity | 0x490 | attack_effect@Some.0 (&Effect 로 넘김) | r | L550 range_adjust 1번째 인자(%2996) | 4 | OK |  |
| 89 | Vec<SmallActionPlay>(res %108) | 0x18 | len | r | L561 any() 순회 상한(m02.ll:35925) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 90 | SmallActionPlay(원소 184B) | 0xb1 | tag | r | L561 tag-15 <u 4 ⇒ {15 Attack,16 Skill,17 Skill2,18 Ult} (m02.ll:35957~35962) | 4 | OK |  |
| 91 | Vec<&Entity>(near_enemies %90 / attackers %87) | 0x18 | len | r | L572 near_enemies.len()==0 · L588 attackers.len()==0 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 92 | GameContext | 0x0 | bump(&Bump) | r | %119(배치 A L364) — L585/L595 Vec::new_in/from_iter_in 할당자 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 93 | BattleSubPlan | 0x2b | v48_dodge_claim | w | L359 무조건 (m02.ll:27738~27739) — initializes((43,44)) \| (배치 A) L510 v48_in_attack_range && 궤적/캐스트라인 없음 (33960) | 4 | OK | 0 |
| 94 | BattleSubPlan | 0x2d | last_bail_gate | w | L360 무조건 (27740~27741) — initializes((45,46)) \| (배치 A) L417 v48_hard && !final_stand 일 때, 직후 return (32504). 코드표(_docs game_ai.txt L589): 1=회피 하드(궁/risk25/CC/연타피격) \| (배치 A) L484 v48_losing(FightLine::Disengage) 일 때, 직후 return (33448). 코드표: 2=지는 판정. (3=전대상 치명 은 배치 C L604) | 4 | OK | 0 |
| 95 | BattleSubPlan | 0x20 | v48_claim_hold_until | w | L511 (33961~33974). initializes 에 없음 = 조건부 쓰기 | 4 | OK | game.tick() + tick_per_second*2 |
| 96 | sret Vec (res %108) | 0x0 | ptr/bump/cap/len 초기화 | w | L364 (27759~27765) | 4 | 확인불가(tcx 사전에 타입 없음) | ptr=8, bump=context.pool, cap=0, len=0 |
| 97 | sret Vec (res %108) | 0x18 | len | w | L403 push 후 (32437~32438); cap==len 이면 reserve_internal_or_panic(res, len, 1, true) 선행(32415) | 4 | 확인불가(tcx 사전에 타입 없음) | len+1 |
| 98 | sret Vec 원소 [len] (184B) | 0x0 | SmallActionPlay::RunAway 페이로드 | w | L403 (32390, 32436). live = new_dodge initializes((0,56),(125,126),(128,132)) | 4 | 확인불가(tcx 사전에 타입 없음) | SmallActionRunAway::new_dodge(data, player, 5) 136B memcpy |
| 99 | sret Vec 원소 [len] (184B) | 0xb1 | 태그 | w | L403 store i8 3 @ %106+177 (32391~32392) | 4 | 확인불가(tcx 사전에 타입 없음) | 3 (RunAway) |
| 100 | sret %0 | 0x0 | 반환 memcpy 32B ← res | w | L418 (32505) · %2791 (35090: L484·kite None·배치 B 일부 반환 경로 공용) | 4 | 확인불가(tcx 사전에 타입 없음) | res |
| 101 | BattleSubPlan | 0x0 | support_target@tag | w | bp:110 (m02.ll:29017). 조건: support_target Some & 엔티티 존재 & (아군이면 가시 적 min 이 Some) | 4 | OK | 1 (Some) |
| 102 | BattleSubPlan | 0x8 | support_target@Some.0 | w | bp:110 (m02.ll:29018) — 아군 support_target 를 가장 가까운 가시 적으로 교체 | 4 | OK | near_enemy.id (Entity+0x5c0) |
| 103 | (sret) Vec | 0x0 | ptr·bump·cap·len | w | L532 return 경로 %2791 (m02.ll:35090). L521 truncate(0) 후 L525 push 1개 → len=1 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | res(%108) 32B memcpy |
| 104 | res(%108) Vec<SmallActionPlay> | 0x18 | len | w | L525 push: cap==len 이면 reserve_internal_or_panic(res, used_cap, 1, true) 후 memcpy 184B (m02.ll:35062~35085) | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | 0 → 1 (L521 truncate / L525 push) · L536 extend(base_positioning res) |
| 105 | BattleSubPlan(self) | 0x2d | last_bail_gate | w | L604 store i8 3 (m02.ll:36263) — all_targets_lethal && !final_stand 일 때만. initializes((45,46)) 의 배치 C 쪽 쓰기(배치 A L360 에서 0 초기화) | 4 | OK | 3 |
| 106 | sret %0 | 0x0 | Vec<SmallActionPlay> 32B | w | L607(m02.ll:36284, RunAway 경로) · L613(m02.ll:35966, 그 외 전 경로) | 4 | 확인불가(tcx 사전에 타입 없음) | res(%108) memcpy |
| 107 | res(%108) | 0x0 -> Vec 원소 | extend(base_battle_action 결과) | w | goal≠AssassinReady 일 때 base_battle_action Vec 를 통째로 append(m02.ll:35480) | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | L539 |
| 108 | res(%108) | 0x0 -> Vec 원소[len] | push Attack(tag 15) | w | L554 (m02.ll:35911 태그 store · 35913 push) | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | SmallActionAttack::new(data, tower.id) |
| 109 | res(%108) | 0x18 | truncate(0) → len=0 | w | L605 (m02.ll:36265) | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | 0 |
| 110 | res(%108) | 0x0 -> Vec 원소[0] | push RunAway(tag 3) | w | L606 (m02.ll:36277 태그 store · 36279 push) | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | SmallActionRunAway::new(data, player, 5) |

**`consts` 상수 61건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 363 | 임계 | version < 2 → final_stand 단락(false) · L472 version<2 → resolve_fight_stake 경로. (같은 리터럴 2 가 CastingType Direction·level>2·FightLine Disengage(2)·rotate_right(2) 로도 쓰임) | 4 |  |
| 1 | 13 | 371 | 태그 | EntityType::Champion 태그 (L371/382) | 4 |  |
| 2 | 4 | 372 | 태그 | ChampionActionState::Skill 태그 (switch case). 리터럴 4 는 level>4(ult 접근자 L383)·BattleSubPlanGoal::RunAway(4)·committed_dir -1 case 로도 쓰임 | 4 |  |
| 3 | 5 | 372 | 태그 | ChampionActionState::Skill2 태그. 리터럴 5 는 new_dodge end_delay(L403)·Assassin(5)·슬롯수 5(stride) 로도 | 4 |  |
| 4 | 6 | 382 | 태그 | ChampionActionState::Ult 태그 (L382). L411 goal==6 AssassinReady. 빔 배열 상한 6 (slice_index_fail) | 4 |  |
| 5 | -1 | 372 | 센티널 | Option<Effect> 니치 None 판별(casting i32 == -1) — unwrap_failed 또는 is_none | 4 |  |
| 6 | 1 | 372 | 태그 | CastingType::Position 태그(switch case 1)·Direction(2) = 논타겟. 동시에 L511 `shl i64 %tps, 1` = tick_per_second*2 (2초 홀드) 접힘 — folded_from=2 는 이 shl 을 뜻함 | 4 | 2 |
| 7 | 9 | 393 | 임계 | dodge_risk > 9 (>=10) 이면 강제 도주 후보 (sgt 9) | 4 |  |
| 8 | 24 | 411 | 임계 | dodge_risk > 24 (>=25) → v48_hard (문서 표현 risk25) | 4 |  |
| 9 | 20 | 415 | 계수 | max(hp,1)*20 <= applyed_damage*100 ⟺ hp <= applyed_damage*5 → v48_hard (hp 가 받은 피해의 5배 이하) | 4 |  |
| 10 | 100 | 415 | 계수 | 위 식의 applyed_damage*100 · radius_adj 의 (mult+100)/100 백분율 | 4 |  |
| 11 | 3 | 403 | 태그 | SmallActionPlay::RunAway 메모리 태그(store i8 3 @+0xb1). committed_dir case 3 KitingBack → -1. 본문의 shl i64 %n, 3 (예 33144·33201·35137) 은 Vec<&Entity> 드롭 코드의 len×8 바이트 산술이라 이 상수와 무관 — shl 경고 사유 | 4 |  |
| 12 | 30000 | 423 | 계수 | closure$2: dist² <= (engage_pair_range+30000)² 근접 적 판정 여유(aux m02.ll:76055). L1156<518 카이팅 링 2 = attack_range+30000 | 4 |  |
| 13 | 14400000001 | 429 | 미상 | 120000²+1 — closure$3: 아군 챔피언 dist² < 이 값(=거리 <= 120000) (aux m01.ll:24218) | 4 |  |
| 14 | 15000 | 433 | 계수 | Effect::range(caster)(effect.rs:26 인라인) 의 고정 가산 15000 (타워 사거리) · L1061/1064<518 빔 폭 여유 champ_r+15000 | 4 |  |
| 15 | 25000 | 445 | 계수 | close_execute_range = max_range_cached + 25000 (처형 거리 여유) | 4 |  |
| 16 | 7 | 441 | 태그 | BattleSubPlanGoal::End 태그 (4 RunAway 와 함께 focus 없음 case) L441/491/518 | 4 |  |
| 17 | 0 | 463 | 태그 | committed_dir case 0 Trace → 1 (0·2·5·6 → 1); Protect(1)/End(7) → 0 | 4 |  |
| 18 | 8000 | 518 | 계수 | v48_kite_position:1143 keep_range = attack_range.saturating_sub(8000) (usub.sat) | 4 |  |
| 19 | 70000 | 518 | 계수 | 1157 카이팅 링 3 반경 = attack_range+70000 | 4 |  |
| 20 | 24000 | 518 | 산출값 | 1162 escaping 시 앞에 끼우는 자기중심 링 1 반경 | 4 |  |
| 21 | 48000 | 518 | 산출값 | 1163 escaping 시 자기중심 링 2 반경 | 4 |  |
| 22 | 16 | 518 | 태그 | 1173 방향 후보 수(anon.267 256B = 16×(dx,dy) i64, 단위 1000 = 22.5° 간격) | 4 |  |
| 23 | 40 | 370 | 임계 | 적 5슬롯 슬라이스 끝(5×8B) — icmp eq %148, 40 루프 종료 (stride 이지만 루프 상한 근거로 등록) | 4 |  |
| 24 | 5 | 536 | 길이 | end_delay 인자(SmallActionRunAway::new/new_with_ult · AroundPosition::new_with_radius · AroundHide::new · Trace end_delay 스토어 @+0x80). L526 도 5. 배열길이 5(플레이어 슬롯)와 겹침 | 4 |  |
| 25 | 4000 | 526 | 미상 | L526 `SmallActionAroundPosition::new_with_radius(rnd, data, x, y, 5, 4000)` around_radius(m02.ll:35035) — 카이팅 지점 도착 반경(1/8셀) | 4 |  |
| 26 | 24000 | 536 | 산출값 | bp:198 `new_with_radius(rnd, data, kx, ky, 5, 24000)` around_radius(m02.ll:29965) — kite_reposition_point 반경(0.75셀) | 4 |  |
| 27 | 15000 | 536 | 계수 | Trace.attack_range_margin(legacy_battle_trace_action 인라인, 5회 스토어 @+0x78) | 4 |  |
| 28 | 25000 | 536 | 계수 | bp:111 `mr = max_range_cached(data,champ,near_enemy) + 25000`(m02.ll:29030) · bp:127 `support_min_action_range(champ,f) + 25000`(m02.ll:31983) — 사거리 여유(≈0.78셀) | 4 |  |
| 29 | 40000000001 | 536 | 임계 | bp:166 closure#7 필터 `dist²(x,champ) < 200000²+1`(=≤200000, 6.25셀) — Kiting 최근접 적 후보 반경(m02.ll:29494) | 4 |  |
| 30 | 22500000001 | 536 | 임계 | bp:266/269 `dist² < 150000²+1`(≤150000, 4.69셀) — visible_enemies / nearby_allies 카운트 반경(m02.ll:30869, 30981) | 4 |  |
| 31 | 90000000001 | 536 | 임계 | bp:286 `to_heal_area < 300000²+1`(분수 중심까지 ≤300000, 9.4셀) → RunAway 추가 · bp:291 재사용(m02.ll:31349) | 4 |  |
| 32 | 22499999999 | 536 | 임계 | bp:291 `dist²(tower,champ) > 22499999999` = `>= 150000²` — 최근접 타워가 150000(4.69셀) 이상 멀면 RunAway(m02.ll:31451) | 4 |  |
| 33 | 2500000001 | 536 | 임계 | bp:301 `dist²(tower,champ) < 50000²+1`(≤50000, 1.56셀) — 타워 코앞이면 enemy_in_tower_range 판정 진입(m02.ll:31678). bp:320 동일값은 NA(version<2) | 4 |  |
| 34 | 21 | 536 | 임계 | bp:291 `tower.hp*100/tower.max_hp < 21` — 타워 HP 21% 미만이면 RunAway(m02.ll:31471) | 4 |  |
| 35 | 40 | 536 | 임계 | bp:273 `hp_ratio > 40` (champ hp%) — 40% 초과이고 is_desperate 면 궁 도주(new_with_ult) 생략(m02.ll:31262) | 4 |  |
| 36 | 2 | 536 | 태그 | ①bp:273 `visible_enemies < nearby_allies + 2`(m02.ll:31263) ②bp:180 Entity::skill2_effect 게이트 `level > 2`(entity.rs:1693, m02.ll:29736) ③bp:260 `(casting-1) <u 2`(Position\|Direction) ④bp:135 tactic==2 BacklineDPS ⑤goal 태그 2 Kiting(switch) | 4 |  |
| 37 | 100 | 536 | 계수 | Entity::radius() `radius*(100+mult)/100` · hp% `hp*100/max` | 4 |  |
| 38 | 4 | 536 | 태그 | ①bp:259 Entity::ult_effect 게이트 `level > 4`(entity.rs:1701, m02.ll:30672) ②bp:131 `tag-15 <u 4`(Attack..Ult 4종) ③goal 태그 4 RunAway | 4 |  |
| 39 | -15 | 536 | 미상 | bp:131 has_focus_action: `add tag,-15; icmp ult 4` = matches!(Attack15\|Skill16\|Skill2 17\|Ult18)(m02.ll:32037) | 4 |  |
| 40 | 1 | 536 | 태그 | bp:293 `version > 1`(reach 접힘: version=2 → 참 · else 가지 bp:320~345 사장) · support_target 태그 Some=1 스토어 · goal 태그 1 Protect · `level-1`(growth_range 배수, add -1) · (본문의 `shl i64 %x, 1` 은 배치 A L511 의 ×2 로 이 항목과 무관 — 시프트량 아님) | 4 |  |
| 41 | -1 | 536 | 센티널 | Option<Effect> None 니치 태그(i32 -1 @ effect+0x30): attack/skill/skill2/ult_effect 유무 검사 · iter_towers_without_nexus 이터레이터 상태 -1(소진) | 4 |  |
| 42 | 14 | 536 | 태그 | SmallActionPlay::Trace 메모리태그(idx 11 + 3) 스토어 @+0xb1 (5회) | 4 |  |
| 43 | 3 | 536 | 태그 | SmallActionPlay::RunAway 메모리태그(idx 0 + 3) 스토어 @+0xb1 · goal 태그 3 KitingBack · base_battle_action goal_tag=3 인자(bp:237/314/337) · (본문의 `shl i64 %cap, 3` 은 L532 bumpalo dealloc 의 cap×8(&Entity 크기) 접힘으로 이 항목과 무관 — 시프트량 아님) | 4 |  |
| 44 | 6 | 536 | 태그 | SmallActionPlay::AroundHide 메모리태그(idx 3 + 3) @+0xb1 (bp:146) · goal 태그 6 AssassinReady · 타워 배열 길이 6 | 4 |  |
| 45 | 8 | 536 | 태그 | (NA version<2) SmallActionPlay::AroundRunAway 메모리태그(idx 5 + 3) bp:334/342 · 포인터 stride | 4 |  |
| 46 | 0 | 521 | 태그 | L521 `res.truncate(0)`(m02.ll:35029) — 강제도주 블록에서 카이팅 지점이 나오면 기존 후보 전부 버림 · bp:314/337 base_battle_action focus=0 · 각종 None/0 스토어 | 4 |  |
| 47 | 7 | 536 | 태그 | goal 태그 7 End → bp:351 `todo!()` 패닉("not yet implemented" 19B, m02.ll:29240) | 4 |  |
| 48 | 19 | 536 | 길이 | panic 메시지 길이("not yet implemented") | 4 |  |
| 49 | 6 | 537 | 태그 | BattleSubPlanGoal::AssassinReady 메모리태그(tcxdict --enum: idx=태그 6). L537 ==6 이면 base_battle_action 생략 · L544/L560 switch case | 3 |  |
| 50 | 4 | 544 | 태그 | BattleSubPlanGoal::RunAway 태그 4 — L544/L560 switch case(RunAway\|AssassinReady 면 타워·치명 판정 모두 생략). 또한 L561 `add -15; icmp ult 4` 의 4 = 태그 구간 폭(15..=18 Attack/Skill/Skill2/Ult) | 4 |  |
| 51 | -15 | 561 | 미상 | L561 has_battle_action: tag-15 <u 4 — 구간 시작 = Attack(15). 접힌 `matches!(a, Attack\|Skill\|Skill2\|Ult)` | 4 |  |
| 52 | -1 | 549 | 센티널 | Entity+0x4c0 attack_effect 니치 태그 -1 = None → champ.attack_effect.as_ref() 가 None 이면 is_some_and=false → 타워 공격 후보 없음 | 4 |  |
| 53 | 30000 | 550 | 오프셋가감 | effect.rs:26 Effect::range 인라인의 사거리 여유 가산(range + 30000 + …) — 셀 32000 미만의 고정 마진(약 0.94셀) | 4 |  |
| 54 | 100 | 550 | 계수 | entity.rs:1515 Entity::radius(): radius*(100+radius_mult)/100 (mult 가 0 이면 radius 그대로, 1513) | 4 |  |
| 55 | 15 | 554 | 태그 | SmallActionPlay::Attack 메모리태그 15 — L554 원소+0xb1 store | 4 |  |
| 56 | 0 | 597 | 태그 | die_tick == 0 이면 그 target 은 '치명' 으로 보고 다음 target 계속, ≠0 이면 all_targets_lethal=false 로 루프 종료. 또한 L572/L588 len==0 판정·L547 block_target_tick==0 | 4 |  |
| 57 | 3 | 604 | 태그 | self.last_bail_gate = 3 (L604) · SmallActionPlay::RunAway 메모리태그 3 (L606 원소+0xb1). 본문의 `shl i64 %3107, 3`(m02.ll:36062) 은 L610 near_enemies drop 글루의 len*8 바이트 계산이라 이 상수와 무관(시프트량 아님) | 4 |  |
| 58 | 5 | 606 | 태그 | SmallActionRunAway::new(data, player, end_delay=5) 3번째 인자 | 4 |  |
| 59 | 40000000001 | 569 | 임계 | aux sf_0(L569): dist²(e, champ) < 200000²+1 ⇒ 거리 ≤ 200000 (=6.25셀) 인 적 챔피언만 near_enemies | 4 |  |
| 60 | 1 | 568 | 임계 | aux sf_0: blackboard[1 - player.info.team] (관측 대상 = 적 팀 판. _docs game_core L21: 「Blackboard[team]은 team 자체 정보, 관측은 1-team」) · 본체 %142 = 1-team(적 팀, 배치 A L370). 본문의 `shl … 1` 은 배치 A/B 영역(537~614 슬라이스에는 shl 1 없음)이라 이 상수와 무관(시프트량 아님) | 4 |  |

**`knobs` 조정점 30건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 강제 도주 dodge_risk 임계 | battle.rs:393 | 9 | 올리면(예 14) 논타겟 궤적/캐스트라인이 있어도 위험도가 더 높아야 회피 후보를 만든다 → 회피 빈도↓ | 4 | 기존 |
| 1 | v48_hard 위험 임계 | battle.rs:411 | 24 | 올리면 회피 후 '단일 후보 조기 종료(gate 1)' 가 덜 걸려 다른 후보(421~) 로 진행 — 결사 중이 아닐 때 한정 | 4 | 기존 |
| 2 | v48_hard HP 배수 | battle.rs:415 | 20 | hp*20 <= applyed_damage*100 (hp <= 5×받은피해). 20 을 올리면 더 낮은 hp 에서만 hard → gate 1 조기종료↓ | 4 | 기존 |
| 3 | 근접 적 판정 여유 | battle.rs:423 (closure$2) | 30000 | 올리면 near_enemies 에 더 먼 적이 들어와 resolve_fight_stake/check_kill_die_tick 입력이 넓어짐 | 4 | 기존 |
| 4 | 근접 아군 반경² | battle.rs:429 (closure$3) | 14400000001 | 120000. 올리면 near_allies 가 늘어 t_die(적 처형 예측)·fight_stake 가 낙관적으로 | 4 | 기존 |
| 5 | 처형 거리 여유 | battle.rs:445 | 25000 | 올리면 더 먼 focus 도 킬확보 검사 대상 → kill_secure 실패 시 블록 탈출(정상 후보) 빈도↑ | 4 | 기존 |
| 6 | 회피 claim 홀드 | battle.rs:511 | tps*2 | 2초. 늘리면 claim 후 캐스트라인 재확인(L398~400) 창이 길어짐 | 4 | 기존 |
| 7 | 카이팅 keep_range 감산 | battle.rs:1143 (v48_kite_position 인라인 @518) | 8000 | 올리면 사거리보다 더 안쪽 링을 1순위로 → 더 붙는 카이팅 | 4 | 기존 |
| 8 | 카이팅 링 2·3 가산 | battle.rs:1156~1157 | 30000 / 70000 | 링 1(keep_range) 실패 시 후퇴 링 반경. 올리면 더 멀리 빠짐 | 4 | 기존 |
| 9 | 탈출 링(자기중심) | battle.rs:1162~1163 | 24000 / 48000 | escaping 일 때 링 목록 앞에 삽입 — 올리면 첫 탈출 스텝이 길어짐 | 4 | 기존 |
| 10 | 빔 회피 폭 여유 | battle.rs:1061/1064 (v48_on_cast_line 인라인) | 15000 | halfwidth+champ_r+15000. 올리면 궤적 근처 후보점을 더 넓게 버림 | 4 | 기존 |
| 11 | dodge end_delay | battle.rs:403 | 5 | new_dodge 3번째 인자(DI end_delay). 의미는 콜리 몫 | 4 | 기존 |
| 12 | 카이팅 지점 도착 반경(강제도주 블록) | battle.rs:526 | 4000 | 올리면 AroundPosition 이 더 넓은 원 안에서 도착 판정 → 덜 정밀한 카이팅 위치 | 4 | 기존 |
| 13 | kite_reposition_point 도착 반경 | battle.rs:198(base_positioning) | 24000 | Kiting 사거리 안 재배치 지점의 허용 반경 | 4 | 기존 |
| 14 | Trace 사거리 여유 | battle.rs:972(legacy_battle_trace_action) attack_range_margin | 15000 | 올리면 추적 시 더 멀리서 멈춤 | 4 | 기존 |
| 15 | support/focus 추적 개시 거리 여유 | battle.rs:111 / 127 | 25000 | 올리면 max_range+여유 밖일 때만 Trace 를 붙이므로 추적 후보가 덜 생김 | 4 | 기존 |
| 16 | Kiting 최근접 적 탐색 반경 | battle.rs:166 | 40000000001 | 200000²+1. 올리면 더 먼 적도 카이팅 대상 | 4 | 기존 |
| 17 | 궁 도주 판정용 적/아군 카운트 반경 | battle.rs:266, 269 | 22500000001 | 150000²+1 | 4 | 기존 |
| 18 | 궁 도주 생략 HP% | battle.rs:273 | 40 | hp% > 40 이고 가시 적 < 근처 아군+2 면 new_with_ult 생략. 내리면 궁 도주가 더 자주 후보에 듦 | 4 | 기존 |
| 19 | 궁 도주 생략 아군 여유 | battle.rs:273 | 2 | visible_enemies < nearby_allies + 2 | 4 | 기존 |
| 20 | 분수 근접 RunAway 반경 | battle.rs:286/291 | 90000000001 | 300000²+1 | 4 | 기존 |
| 21 | 타워 원거리 판정 | battle.rs:291 | 22499999999 | dist² > 이 값(=≥150000²) 이면 타워 무시하고 RunAway | 4 | 기존 |
| 22 | 타워 HP% 하한 | battle.rs:291 | 21 | 타워 HP<21% 면 타워 밑 방어 대신 RunAway | 4 | 기존 |
| 23 | 타워 코앞 판정 | battle.rs:301 | 2500000001 | 50000²+1. 이 안이면 적이 타워 사거리 안인지 보고 base_battle_action(KitingBack) 추가 | 4 | 기존 |
| 24 | 타워 공격 후보 사거리 마진 | battle.rs:550 (effect.rs:26 인라인) | 30000 | 올리면 더 먼 타워도 '사거리 안' 으로 보고 v3_tower_burst_feasible 검사에 올린다(공격 후보 생성 증가) · 내리면 타워 공격 후보가 줄어든다 | 4 | 기존 |
| 25 | 근접 적 챔피언 반경(제곱) | battle.rs:569 (aux sf_0 m02.ll:76229) | 40000000001 | 올리면(=반경 200000 확대) 더 먼 적까지 near_enemies 에 들어가 치명 판정 대상이 늘고 RunAway 대체가 더 자주 발동 · 내리면 반대 | 4 | 기존 |
| 26 | 치명 판정 기준 die_tick | battle.rs:597 | 0 | die_tick==0 만 치명으로 본다. 임계를 N 으로 올리면(die_tick<=N) 곧 죽는 상황도 치명으로 봐 RunAway 대체가 늘어난다 | 4 | 기존 |
| 27 | RunAway 대체 행동의 end_delay | battle.rs:606 | 5 | SmallActionRunAway::new 3번째 인자 — 올리면 도주 행동 종료 지연이 길어진다(의미는 SmallActionRunAway 명세 참조) | 4 | 기존 |
| 28 | bail gate 코드 | battle.rs:604 | 3 | self.last_bail_gate 에 기록되는 사유 코드(3 = 전원 치명 도주). 소비처는 이 함수 밖(미확인) — 값 변경은 관측 코드에만 영향할 가능성 | 4 | 기존 |
| 29 | 타워 공격 후보 생략 goal 집합 | battle.rs:544 / 560 | RunAway(4) \| AssassinReady(6) | 이 집합에 goal 을 더하면 그 goal 에서 타워 공격·치명 판정이 모두 생략된다(그대로 res 반환) | 4 | 기존 |

<details><summary>`callees` 피호출자 85건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::BattleSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\battle.rs:358 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | base_battle_action | game_ai::plan_legacy::sub_plan::battle::base_battle_action | in:game_ai::plan_legacy::sub_plan::battle | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::old::BattleSubPlanGoal) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\battle.rs:1305 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | base_positioning | game_ai::plan_legacy::sub_plan::JungleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::jungle | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> game_ai::SmallActionPlay | game-ai\src\plan_legacy\sub_plan\jungle.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 5 | base_positioning | game_ai::plan_legacy::sub_plan::BattleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::battle | fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\battle.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 6 | base_positioning | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_safe | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_safe.rs:15 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 7 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 11 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | dist_to_line_segment | game_core::utils::dist_to_line_segment | pub | fn(i64, i64, i64, i64, i64, i64) -> u64 | game-core\src\utils.rs:200 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | engage_pair_range | game_ai::plan_legacy::old::engage_pair_range | pub | fn(usize, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2389 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | expected_move_distance | game_core::EffectType::expected_move_distance | pub | fn(&Self/#0) -> std::option::Option<(usize, u64)> | game-core\src\simulation\effect\type.rs:289 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 19 | expected_move_distance | <game_core::RushEffect as game_core::EffectType>::expected_move_distance | pub | fn(&game_core::RushEffect) -> std::option::Option<(usize, u64)> | game-core\src\simulation\effect\type\rush.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 20 | expected_move_distance | <game_core::MoveToEffect as game_core::EffectType>::expected_move_distance | pub | fn(&game_core::MoveToEffect) -> std::option::Option<(usize, u64)> | game-core\src\simulation\effect\type\move_to.rs:54 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 21 | fight_participants | game_ai::plan_legacy::old::fight_model::fight_participants | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< (&game_core::Entity, i64, bool)> | game-ai\src\plan_legacy\old\fight_model.rs:610 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | focus | game_ai::plan_legacy::old::BattleSubPlanGoal::focus | pub | fn(&game_ai::plan_legacy::old::BattleSubPlanGoal) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\battle.rs:29 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 23 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 24 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 25 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 26 | growth | game_core::ChampionInfo::growth | pub | fn(&Self/#0) -> game_core::EntityStat | game-core\src\setting.rs:1083 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 27 | growth | <game_core::DataChampionInfo as game_core::ChampionInfo>::growth | pub | fn(&game_core::DataChampionInfo) -> game_core::EntityStat | game-core\src\setting\champion\data_driven.rs:2573 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 28 | growth | <game_core::MonkChampionInfo as game_core::ChampionInfo>::growth | pub | fn(&game_core::MonkChampionInfo) -> game_core::EntityStat | game-core\src\setting\champion\monk.rs:62 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 29 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 30 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 31 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 32 | is_ignored_battle_enemy | game_ai::plan_legacy::old::is_ignored_battle_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:887 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 36 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 37 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 38 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 39 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 42 | judge_accuracy | game_core::AthleteParameter::judge_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:337 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 43 | kind | game_view::ui::database_edit_ui::DbEditAppearanceTarget::kind | in:game_view::ui::database_edit_ui | fn(game_view::ui::database_edit_ui::DbEditAppearanceTarget) -> game_view::ui::athlete_appearance_popup::AppearanceTargetKind | game-view\src\ui\database_edit_ui.rs:100 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 44 | kite_reposition_point | game_ai::plan_legacy::sub_plan::battle::kite_reposition_point | in:game_ai::plan_legacy::sub_plan::battle | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, u64) -> (u64, u64) | game-ai\src\plan_legacy\sub_plan\battle.rs:1217 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 45 | legacy_attack_range_trace_action | game_ai::plan_legacy::sub_plan::battle::legacy_attack_range_trace_action | in:game_ai::plan_legacy::sub_plan::battle | fn(&game_core::OperationData, usize, bool) -> game_ai::SmallActionPlay | game-ai\src\plan_legacy\sub_plan\battle.rs:979 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 46 | legacy_battle_trace_action | game_ai::plan_legacy::sub_plan::battle::legacy_battle_trace_action | in:game_ai::plan_legacy::sub_plan::battle | fn(&game_core::OperationData, usize, bool) -> game_ai::SmallActionPlay | game-ai\src\plan_legacy\sub_plan\battle.rs:971 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 47 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 48 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 49 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 50 | line | game_ai::plan_legacy::types::BigPlan::debug_label::line | in:game_ai::plan_legacy::types | fn(game_core::LineType) -> &str | game-ai\src\plan_legacy\types.rs:83 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 51 | line | game_core::TowerType::line | pub | fn(&game_core::TowerType) -> std::option::Option<game_core::LineType> | game-core\src\simulation\entity\tower.rs:90 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 52 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 53 | new | game_ai::SmallActionAround::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 54 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 55 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | new | game_ai::SmallActionAroundHide::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAroundHide | game-ai\src\small_action\around.rs:312 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 57 | new_dodge | game_ai::SmallActionRunAway::new_dodge | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:77 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 58 | new_with_radius | game_ai::SmallActionAroundPosition::new_with_radius | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize, u64) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:837 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 59 | new_with_ult | game_ai::SmallActionRunAway::new_with_ult | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:53 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 60 | nexus_final_stand | game_ai::plan_legacy::old::nexus_final_stand | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:190 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 61 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 62 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 63 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 64 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 65 | ready_damage_to_target | game_ai::plan_legacy::old::fight_model::ready_damage_to_target | in:game_ai | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\plan_legacy\old\fight_model.rs:759 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 66 | resolve_fight_stake | game_ai::plan_legacy::old::fight_model::resolve_fight_stake | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:571 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 67 | resolve_fight_stake_roster | game_ai::plan_legacy::old::fight_model::resolve_fight_stake_roster | in:game_ai | fn(usize, &game_core::OperationData, &game_core::Entity, &[(&game_core::Entity, i64, bool)], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:645 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 68 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 69 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 70 | support_min_action_range | game_ai::plan_legacy::sub_plan::support_min_action_range | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\sub_plan\battle.rs:1257 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 71 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 72 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 73 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 74 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 75 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 76 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 77 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 78 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 79 | v21_should_skip_support_trace | game_ai::plan_legacy::sub_plan::battle::v21_should_skip_support_trace | in:game_ai::plan_legacy::sub_plan::battle | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\sub_plan\battle.rs:1292 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 80 | v21_support_pressure_too_risky | game_ai::plan_legacy::sub_plan::battle_common::v21_support_pressure_too_risky | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\sub_plan\battle_common.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 81 | v3_tower_burst_feasible | game_ai::plan_legacy::sub_plan::battle::v3_tower_burst_feasible | in:game_ai::plan_legacy::sub_plan::battle | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\plan_legacy\sub_plan\battle.rs:32 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 82 | v48_cast_beams | game_ai::plan_legacy::sub_plan::battle::v48_cast_beams | in:game_ai::plan_legacy::sub_plan::battle | fn(&game_core::PlayerState, &game_core::OperationData) -> (usize, [(u8, i64, i64, i64, i64, u64); 6_usize]) | game-ai\src\plan_legacy\sub_plan\battle.rs:998 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 83 | v48_kite_position | game_ai::plan_legacy::sub_plan::battle::v48_kite_position | in:game_ai::plan_legacy::sub_plan::battle | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, std::option::Option<usize>, bool) -> std::option::Option<(u64, u64)> | game-ai\src\plan_legacy\sub_plan\battle.rs:1132 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 84 | v48_on_cast_line | game_ai::plan_legacy::sub_plan::battle::v48_on_cast_line | in:game_ai::plan_legacy::sub_plan::battle | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64) -> bool | game-ai\src\plan_legacy\sub_plan\battle.rs:1057 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 33개**: `applyed_cc`, `applyed_damage`, `assume`, `casting`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `escaping`, `extend`, `focus_id`, `halfwidth`, `map_or`, `near_allies`, `near_enemies`, `on_periodic_trajectory`, `on_trajectory`, `or_else`, `panic`, `parameter`, `positioning_score`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`, `range  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `recent_visible`, `reserve_internal_or_panic`, `risk`, `rotate_right`, `setting`, `skip`, `slice_index_fail`, `target`, `truncate`, `try_fold`, `v48_claim_hold_until`, `with  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 2개는 **전부 다른 함수**라 싣지 않는다`, `with_dive`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35214) · **형제 8개** (BattleSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::BattleSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan) -> game_ai::plan_legacy::sub_plan::BattleSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::BattleSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::BattleSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:46 | True | fn(game_ai::plan_legacy::old::BattleSubPlanGoal, std::option::Option<usize>, game_ai::plan_legacy::old::BattleTactic, bool, usize, bool) -> game_ai::plan_legacy::sub_plan::BattleSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::BattleSubPlan::is_dive_local | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:68 | True | fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan) -> bool |
| 4 | game_ai::plan_legacy::sub_plan::BattleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::battle | game-ai\src\plan_legacy\sub_plan\battle.rs:72 | False | fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::BattleSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:358 | False | fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::BattleSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:616 | False | fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 7 | game_ai::plan_legacy::sub_plan::BattleSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:839 | False | fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |

**`open` 28건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | (배치 A) L519~520 의 `escaping` 표현식(%2430 phi: [true,%2405]·[%2428,%2426]) 과 L521~535(kite 사용·AroundPosition push 등)는 배치 B 범위 — 이 명세는 kite 인자·반환만 적음 | 4 |  |
| 1 | 미탐색 | (배치 A) v48_kite_position 6번째 인자(Option<usize>)는 DI 이름이 없다(`?`). L441 의 focus(goal 4/7 → None, 그 외 goal.focus) 와 같은 값이 %2406/%2407 로 전달됨 — 이름은 추정 focus_id | 5 |  |
| 2 | 미탐색 | (배치 A) DIRS16(anon.94acafa22d01e083ca1cc62f01598c8f.267, m02.ll:275) 방향표 값(1000,0)(924,383)(707,707)(383,924)… 는 전역 상수라 C1 대상이 아니어서 constants 에 넣지 않음(본문엔 memcpy 256B 참조만) | 4 |  |
| 3 | 미탐색 | (배치 A) Effect::range(caster)(effect.rs:26) 의 고정 가산 15000 과 L1061/1064 의 15000 이 같은 의미(표적 반경 기본값)인지는 확인 못 함 — 값 일치만 관측 | 4 |  |
| 4 | 미탐색 | (배치 A) `data.blackboard[1 - player.team]`(적 팀 인덱스) 를 is_recent_visible 첫 인자로 넘기는 이유(블랙보드 배열의 인덱스 규약)는 game_core 몫 — IR 사실만 기록 | 4 |  |
| 5 | 미탐색 | (배치 A) Game::adjust_position(ptr %2662=context+0x20, ptr %2663=context+0x8, x, y) 의 인자 의미(map, setting)는 GameContext 레이아웃(tcxdict)으로 추정 — _gcbc define 미독 | 3 |  |
| 6 | 미탐색 | (배치 A) nexus_final_stand 의 의미는 _docs game_ai.txt L497/L617 '최후 단계(쌍둥이 전멸 + 넥서스 피격)' 과 이름 일치로 추정 — 본문 미독(m04.ll) | 4 |  |
| 7 | 미탐색 | (배치 A) position_score_at_position 내부 TLS(POS_EVAL_CACHE)·check_kill_die_tick 내부 메모·resolve_fight_stake(ResolveFightCache 80B/ResolveFightKey 184B 존재) 의 캐시 동작은 이 범위 밖 — signature.tls 두 번째 항목에 호출 순서만 | 4 |  |
| 8 | 미탐색 | (배치 A) out-of-line v48_on_cast_line(L400·L506·L1191) 본문(m02.ll:66030~)은 미독 — 인라인된 1183 사본(빔 점/선분 거리 ≤ halfwidth+champ_r+15000)과 동일하다고 가정하지 않았음(계약만) | 4 |  |
| 9 | 미탐색 | (배치 A) closure$0/1 의 `Effect::is_in_range(eff, c, champ)` 인자 순서 = (effect, caster=c, target=champ) 로 읽음(IR 인자 순서 그대로) — is_in_range 정의(_gcbc) 미독 | 4 |  |
| 10 | 미탐색 | (배치 A) L1200 best 갱신 3순위 step(dist² to champ) 비교 `%2762 = step < best.d` 의 DI 이름 `better` — 1순위 range_err·2순위 risk 는 확정, 동률 시 더 낮은 인덱스 유지 | 4 |  |
| 11 | 미탐색 | (배치 B) min_by_key/count 폴드의 나머지 원소 처리 본체(m12.ll 12852~13060 closure#0/#1, 13063~13270 closure#3/#4, 13273~13512 closure#7/#8, m06.ll 27822~28074 closure#10/#11)는 aux 로 넣지 않았다 — 첫 원소 술어·키가 본문에 인라인돼 있어 동일 술어의 반복으로 판단(줄수 200+ 라 미독). 술어가 첫 원소와 다를 가능성은 검토 안 함(미탐색) | 4 |  |
| 12 | 미탐색 | (배치 B) EffectType vtable +0x58(슬롯 11) = expected_move_distance 판정 근거는 DI 이름 `is_some<tuple$<usize,u64>>` 와 trait 선언 순서(tcx: expected_move_distance -> Option<(usize,u64)>)뿐 — divtable 은 Arc<dyn> 이라 못 씀. 슬롯 수 산술(3 헤더 + 8번째 = 11)과 일치 | 3 |  |
| 13 | 표기 불가 | (배치 B) L153/L155·L205/L207·L220/L222 가 두 줄로 갈라진 이유(같은 legacy_battle_trace_action 호출이 avoid true/false 로 분리 · true 쪽만 `L43<101<..` 한 단계 더 인라인) — 소스 표기(if/else vs 인자식) 표기 불가(column 없음) | 4 |  |
| 14 | 미탐색 | (배치 B) is_desperate(bp:273) 의 DI 이름과 IR 극성(`visible_enemies < nearby_allies + 2` 가 참일 때 궁 도주 생략)이 직관과 반대로 보이나 IR 정본 그대로 적음. DW_OP_not 여부는 보지 않음 | 3 |  |
| 15 | 미탐색 | (배치 B) blackboard 인덱스가 `1 - player.team`(적 팀 슬롯)인 이유(Blackboard 의미론) 미확인 — is_recent_visible 자식 명세(game_core) 소관 | 4 |  |
| 16 | 미탐색 | (배치 B) Neutral 팀 분기(bp:76 두 팀 모두 Neutral → 필터 없이 min_by_key %293)는 champ.team 이 항상 Player 라 실전 도달 불가로 판단(런타임 미검증) | 4 |  |
| 17 | 미탐색 | (배치 B) base_battle_action(7,900줄)·kite_reposition_point·v21_support_pressure_too_risky 내부의 2단계 이상 TLS 접점은 미확인(1단계 grep 만: kite→position_score_at_position 2회, v21→max_range_cached 6회, base_battle_action 0건) | 4 |  |
| 18 | 표기 불가 | (배치 B) L532 의 정확한 소스 문장(`}` 인지 `return res` 인지)은 column 부재로 표기 불가 — 두 드롭 사본(return 경로 %2791 / 폴스루 %2090)이 모두 L532 를 가리키는 것만 확정 | 4 |  |
| 19 | 미탐색 | (배치 B) reach 사장(NA) 처리: bp:320~345(version<2) 의 SmallActionAround::new ×2 · push ×2 · base_battle_action(3,0) · Extend · closure#18 any — logic 에 한 줄로만 봉인, 상수·오프셋은 미전수 | 4 |  |
| 20 | 미탐색 | (배치 C) max_range_cached 의 TLS MaxRangeCache 키 구성(slots[10][10] 인덱스가 챔피언 슬롯 쌍인지)·무효화 조건 — m10.ll:50689~51083 본문 미독해(추정으로 표기) | 4 |  |
| 21 | 미탐색 | (배치 C) v3_tower_burst_feasible 의 IR 인자 (i64 team, cache, context, tower) ↔ 소스 (player, data, tower) 대응은 ArgumentPromotion 추정 — DI arg 이름(player/data/tower)과 전달값(%124 team, %135 cache, %118 context)으로 유추. 본문(m02.ll:74530~74724) 미독해 | 4 |  |
| 22 | 미탐색 | (배치 C) base_battle_action 은 r14 계약만 인용: (version, _rnd, player, data, sub_goal: BattleSubPlanGoal) -> Vec<SmallActionPlay>. _rnd 는 IR 인자에서 제거됨(DI 만 존재) | 4 |  |
| 23 | 표기 불가 | (배치 C) L597 die_tick!=0 경로에 `all_targets_lethal = false` dbg_value 가 없다 — 소스에 대입이 있었는지(컴파일러가 제거)·아니면 break 만 있고 L603 조건이 다른 형태인지 표기 불가. 분기 효과(RunAway 미생성)는 동일 | 4 |  |
| 24 | 표기 불가 | (배치 C) L561 `llvm.assume(tag != 10)` — 니치 미사용값 힌트로 읽었으나 소스 표현(`matches!` 의 정확한 variant 나열 순서)은 표기 불가 | 4 |  |
| 25 | 미탐색 | (배치 C) Effect::range_adjust(effect, champ, tower) 의 반환 의미(투사체/대상 크기 보정 추정) — game_core 본문 미독해, 시그니처만 | 5 |  |
| 26 | 미탐색 | (배치 C) self+0x2b(v48_dodge_claim) 쓰기는 배치 A(L359) 범위 — 이 배치 범위에는 없음(initializes 표면 중 배치 C 쪽은 +0x2d 만) | 4 |  |
| 27 | 미탐색 | (배치 C) iter_towers_without_nexus 의 6칸 고정 슬롯 + 슬라이스 절반의 구성(어떤 타워가 어느 절반인지) — _gcbc g15.ll:108971 본문 미독해 | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 B) dist² 계산의 인라인 사슬 `L2158 → L3147 → L9` 의 함수 이름(game_core utils.rs:2158?)은 tcx 최근접 매칭이 다른 utils.rs(transfer) 를 골라 미확정 — 연산 자체(\|dx\|²+\|dy\|²)는 IR 로 확정 | 3 | 사실 서술 |
| 1 | (배치 C) check_kill_die_tick 반환값의 정확한 의미(0 이 '즉사'인지 '처치 불가 센티널'인지) — fight_check 본문(m15.ll:27042~28034) 미독해. 이 배치는 분기 효과(0 → 계속, ≠0 → all_targets_lethal=false)만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

