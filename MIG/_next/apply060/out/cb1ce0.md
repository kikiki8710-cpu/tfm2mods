# cb1ce0→e986c0 SerpenHuntSubPlan::action_candidates

## logic_060
```
// serpen_hunt.rs:0~428 (배치 G) · 0.6.0 e986c0
// ── 함수 머리 serpen_hunt.rs:201 · `%N` 레지스터 표기는 0.5.8 m02.ll 기준(구조 동일 · 0.6.0 IR 없음) · 오프셋/태그는 0.6.0 값
// ★0.6.0 SmallActionPlay 태그(+0xb1): RunAway 3 · Recall 4 · Around 5 · AroundRegion 6 · AroundRunAway 7 · Positioning 8 · AroundPosition 0/1/2(dataful · 구멍 9) · AroundPositionBush 10 · AroundBush 11 · LaneMinionPosition 12 · Trace 13 · Attack 14 · Skill 15 · Skill2 16 · Ult 17 · Stop 18 · 니치 디코드 = tag>2 ? tag-3 : 6
//
// L203: team = player.info.team(+0xa00); if team >= 2 → panic_bounds_check(2)   // ★0.6.0 +0x930→+0xa00
//        champ = data.cache(+0).player_champion(+0x1e0)[team][player.info.position(+0xa90 i32)]  ; None(null) → unwrap_failed   // ★0.6.0 +0x9c0→+0xa90
// L204: if self.need_recall(+0x0 bool) {
// L212:   max_hp = champ.stat_cached.hp(+0x628); max_hp==0 → panic_const_div_by_zero
//         hp_ratio = champ.hp(+0x670) * 100 / max_hp
// L213:   if hp_ratio > 29 { self.need_recall = false }   // IR `icmp ugt 29`
//        } else {
// L205:   gm = data.cache.game.<vtable+0x40 get_game_mode>() ; tag(+0)!=0(Moba 아님) → unwrap_failed
//                       m = gm.payload(&MobaMode) ; live = m.jungle_runner.serpen.live_list(ptr +0x1d0, len +0x1d8 · 0.5.8 값 · 0.6.0 미확인)
//                       serpen: Option<&Entity> = live.first().and_then(|id| game.<vtable+0x1f0 get_entity_by_id>(*id))   // len==0 → None ; get_entity null → None
//                       if let Some(serpen) = serpen {
// L206:     serpen.attack_effect(+0x490) 의 tag(+0x4c0)== -1(None) → unwrap_failed
//                         dmg = Effect::expected_damage_target(&serpen.attack_effect, data.context(+0x8), serpen as &dyn, champ)   // 콜리 v3: 1643790(구 12857f0)
// L207:     if !(champ.hp(+0x670) > dmg*3) { self.need_recall = true }   // 즉 champ.hp <= 3*dmg
//                       }
//        }
// L220: if self.need_recall {
// L221:   res = bumpalo Vec::new_in(data.context.pool(+0x0 of GameContext))   // ptr=8(dangling)·bump·cap=0·len=0
// L222:   res.push(SmallActionPlay::Recall(SmallActionRecall::new(data, player, end_delay=5)))   // 136B → 184B 슬롯, tag@+0xb1 = 4(Recall)
// L223:   return res   (sret 32B memcpy) → L514
//        }
// L226: if let Some(action) = team_plan.v27_objective_discipline_action(version, rnd, player, data, target=JungleType::Serpen(i8 5)) {   // sret 184B · tag@+0xb1 == -1 → None
// L227:   return bumpalo::vec![in data.context.pool; action]   (from_iter_in<[SmallActionPlay;1]>) → L514
//        }
// L231~233: nearest_enemy_tower: Option<&Entity>
//        = data.cache.iter_towers_without_nexus(1 - team)        // sret 120B = Chain<Flatten<IntoIter<[Option<&Entity>;6]>>, Copied<slice::Iter<&Entity>>> · 0.6.0 생성자 14151b8d0
//          .filter(|t| t.can_target(+0x6b9) && t.block_target_tick(+0x6a0) == 0)     // 클로저 s_0 (첫 6칸은 인라인 루프, 나머지 슬라이스는 try_fold<find::check<s_0>>)
//          .min_by_key(|t| dist²(t, champ))   // dx=|t.x-champ.x|, dy=|t.y-champ.y|, dx²+dy² ; Map<Filter<..>>::fold(min_by)
// L236: act_actions(%94) = { 인라인 헬퍼(L668~673):
//          res = Vec::new_in(pool);
//          res.extend(fight_check::battle_action(version, rnd, player, data, _end_delay=5));      // L670
//          res.extend(fight_check::attack_summon_action(player, data));                           // L671
//          res.extend(self.attack_jungle_action(team, position(i32), data));                      // L672
//          res }
// L237: strategy = PlayerState::strategy(player, rnd, data.cache.game)  (24B Strategy 로컬 · 원본 PlayerState +0x568 · ★0.6.0 구 +0x4f8)
//        object_finish_objective(%93): Option<&Entity> =
//          if strategy.object_finish(+0xf of 로컬) == KillPriority(0) {
// L238:     get_game_mode() ; Moba 아님 → unwrap_failed ; live_list.len==0 → None
// L239:     live_list.first().and_then(get_entity_by_id)
//          } else { None }
// L243~312: act_actions(%91) = act_actions.iter().filter(F1).map(|a| a.clone()).collect_in(pool)
//        F1 = 클로저 s2_0 · 캡처 = (game data_ptr, game vtable, player, &object_finish_objective, champ, &nearest_enemy_tower)
//        F1(a) → keep(true) 판정:
//          L245: if let Some(id) = a 의 대상 id(small_action.rs:309 헬퍼 — tag∈{14,15,16,17}=Attack/Skill/Skill2/Ult 이면 Some(+0x8)) {   // ★0.6.0 태그 15~18→14~17
//          L246:   if let Some(target) = game.get_entity_by_id(id) {
//          L247:     if fight_model::should_ignore_object_finish_kill_priority_target(player, target, *object_finish_objective) { return false }
//                  }}
//          L252: if let Some(id) = a 의 대상 id { if let Some(t) = get_entity_by_id(id) { if t.ty(+0x68) != Champion(13) { return true } } }   // 비-챔피언 대상은 무조건 유지
//          L253: match a {   // 니치 디코드: tag>2 ? tag-3 : 6   // ★0.6.0 기본 7→6
//            RunAway·Recall·Around·AroundRegion·AroundRunAway·Positioning·AroundPosition·AroundPositionBush·AroundBush·LaneMinionPosition·Trace·Stop → false   // ★0.6.0 AroundHide 제거
//            Attack(14):  L256: target = get_entity_by_id(a.target(+0x8)) else false
//                         L257: eff = champ.attack_effect(+0x490).unwrap()   (tag -1 → unwrap_failed)
//                         L258: if !eff.is_in_range(champ, target) → false
//                               nearest_enemy_tower None → true
//                               Some(t): if !(t.attack_effect.unwrap().is_in_range(t, champ) && target.ty != Tower(2)) → true
//                         L259:          else → return (target.team == champ.team)   // TeamType PartialEq(entity.rs:1127): tag 다르면 false, 둘 다 Neutral → true, Player 면 id 비교
//                                        (해석: 적 타워가 나를 때릴 수 있는 위치에서 타워가 아닌 상대 진영 대상 평타 후보는 버린다)
//            Skill(15):   L266: target = get_entity_by_id(a.target(+0x8)) else false
//                         L267: eff = champ.skill_effect(+0x4c8).unwrap()   (tag +0x4f8 == -1 → unwrap_failed)
//                         L268: if !eff.is_in_range(champ, target) → false ; 타워 게이트 = Attack 과 동일(L269 팀 비교, 불일치 → false; 통과 시 계속)
//                         L269: if !(eff.ty.<EffectType vtable+0x88 expected_move_on_hit>() || eff.ty.<+0x80 expected_rush_effect>()) → false   // ★0.6.0 vt +0x68/+0x60→+0x88/+0x80
//                         L270: nearest_enemy_tower None → true ; Some(t) → return s2_0s4_0(target, t) = !t.attack_effect.unwrap().is_in_range_ex(t, target, t.x, t.y, target.x, target.y, 15000)
//            Skill2(16):  L281~285: 위와 동일, eff = (champ.level(+0x5c8) > 2 ? &champ.skill2_effect(+0x500) : &None).unwrap(), 중첩 클로저 s2_0s6_0
//            Ult(17):     L296~300: 위와 동일, eff = (champ.level > 4 ? &champ.ult_effect(+0x538) : &None).unwrap(), 중첩 클로저 s2_0s8_0
//          }
// L314: act_actions.retain(F2)   · 캡처 = (data, champ, player, &version)
//        F2(a) → keep 판정:
//          L315: match a { Skill(15) → L317.., Skill2(16) → L351.., Ult(17) → L385.., 그 외(RunAway~Trace·Attack·Stop) → 유지 }   // ★0.6.0 태그 재번호
//          [Skill 갈래 L317~335 · Skill2 L351~369 · Ult L385~403 는 effect 만 다르고 동일 구조]
//          L317: target = get_entity_by_id(a.target(+0x8)) ; None → 제거
//          L318: if target.team != champ.team → 유지   (적 대상 스킬은 그대로)
//          L320: eff = champ.skill_effect.unwrap()  (None → 제거)   [Skill2: level>2 ? skill2_effect : None · Ult: level>4 ? ult_effect : None]
//          L321: has_heal   = eff.ty.<vtable+0x58 expected_heal>(data.context, champ as &dyn) != 0      // ★0.6.0 vt +0x40→+0x58
//          L322: has_shield = eff.ty.<+0x60 expected_shield>(ctx, champ) != 0                          // ★0.6.0 vt +0x48→+0x60
//          L323: has_buff   = eff.ty.<+0x68 expected_buff>(ctx, champ).is_some()   (sret 288B, tag@+0x48 != -1)   // ★0.6.0 vt +0x50→+0x68
//          L324~327: near_enemy = [player_champion[1-team] · iter_towers(1-team) · cache.jungles(+0xd0) · cache.others(+0xf0)[1-team]]   (jungles/others 오프셋 = 0.5.8 값 · 0.6.0 미확인)
//                    .any(|e| dist²(e, target) < 14400000001 && e.is_visible_from(&champ.team))   // 120000²+1 · is_visible_from(entity.rs:1481): Neutral→true, Player(t)→visible_state(+0x38)[t].tag == Visible(0)
//          L328: hp_ratio = target.hp(+0x670)*100 / target.stat_cached.hp(+0x628)   (0 → div_by_zero panic)
//          L329: if has_heal && !has_buff && hp_ratio > 79 {
//          L331:    keep = (!has_shield || near_enemy) && buff_value::aoe_heal_covers_low_ally(version, eff, data, player, target)
//                } else {
//          L335:    keep = !has_shield || has_buff || near_enemy   (has_heal&&has_buff 는 무조건 유지)
//                }
// L423: act_actions.retain(|a| { L424: s = self.score(version, parameter, rnd, player, data, a, debug); L425: !(s < -30) })   // score < -30 → 제거 · 콜리 v3: score 디스패처 ed3dd0 계열(spec_patch §B)
// L428: move_actions(%87) = get_move_action(version, rnd, player, data, &parameter.positioning_score(+0x9f0), team_plan, debug)   // serpen_hunt.rs:517 인라인, 아래 전개 · +0x9f0 0.6.0 확인
//   L517~519: res = Vec::new_in(pool) ; champ = player_champion[team][pos].unwrap()(%408)
//   L522~528: has_non_target_action_range = player_champion[1-team].iter().flatten().any(|c|
//       L523:   utils::nontarget_windup_perceived(version, player, data, c)[de6080] && c.ty == Champion(13)
//       L524:   && match c.champion.action_state(+0x70) {
//                 Skill(4)  → e = c.skill_effect.unwrap() (tag +0x4f8: -1 → unwrap_failed) ; e.casting(+0x30) ∈ {Position(1), Direction(2)} && Effect::is_in_range(e, c, champ)
//       L526:    Skill2(5) → e = (c.level>2 ? skill2_effect : None).unwrap() ; 동일
//       L528:    Ult(6)    → e = (c.level>4 ? ult_effect : None).unwrap() ; 동일
//                 _ → false })
//   L535: position_score = position_eval::position_score_at_position(version, player, data, positioning_score, champ.x(+0x660), champ.y(+0x668), purpose=Objective(i8 11))   // sret 56B PositioningScore · 콜리 v3: ff5ec0
//   L537: if position_score.on_trajectory(+0x30) || has_non_target_action_range || position_score.on_periodic_trajectory(+0x31) {
//   L539:   res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, end_delay=5, with_skill=true)))   // tag 3
//   L540:   return res → move_actions → L431(배치 H)
//         }
//   L543: posture = team_plan.v25_objective_posture(version, player, data, target=Serpen(i8 5))   // sret 88B Option<ObjectivePosture>, None = focus_enemy tag(+0)== -1 (레이아웃 = 0.5.8 값 · 0.6.0 미확인)
//   L544: if let Some(posture) = posture {
//   L545:   if posture.kind(+0x50) > 2 {   // WaitGroup(3)|SoftDisengage(4)
//   L546:     if kind == SoftDisengage(4) && posture.near_enemy_count(+0x38) != 0 {
//   L547:       res.push(RunAway(new_with_skill(data, player, 5, false)))  }
//   L549:     res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, posture.wait_pos.0(+0x20), .1(+0x28), end_delay=5)))   // 니치 untagged(태그 store 없음) · 콜리 v3: ecde50
//   L550:     if data.context.debug(+0x3b) {
//   L551:       debug.infos(+0xa0 HashMap<usize,Vec<String>>).entry(champ.id(+0x5c0)).or_insert(vec![]).push(format!("…{:?}", posture.kind))  }
//   L553:     return res → L431(배치 H)
//           }
//   L556:   if kind == Screen(2) && posture.focus_enemy(+0x0 tag)==Some {
//   L557:     focus_enemy = posture.focus_enemy(+0x8)
//   L558:     res.push(Trace(SmallActionTrace::new_attack_range(data, focus_enemy, end_delay=5)))   // = new_attack_range_margin(…, 15000): attack_range_only=true · tag 13
//           }  (그 외 Commit/HoldCamp/Screen-무초점 → 통과)
//         }
//   L564: serpen = get_game_mode() Moba(m) → m.…live_list.first().and_then(get_entity_by_id) ; Moba 아님 → unwrap_failed
//   L565: objective_in_attack_range = false ; if let Some(serpen) = serpen {
//   L566:   mr = max_range(champ, serpen) [인라인 헬퍼 L16~26]: eff = champ.attack_effect.unwrap()(None → unwrap_failed)
//             = eff.range(+0x4a0) + champ.stat_buff_cached.range(+0x438) + eff.growth_range(+0x4a8)*(champ.level(+0x5c8)-1)
//               + Effect::range_adjust(eff, champ, serpen)[vt+0x108 · ★0.6.0 구 +0xe8] + radius(champ) + radius(serpen)
//             radius(e) = e.stat_buff_cached.radius_mult(+0x470)==0 ? e.radius(+0x680) : e.radius*(mult+100)/100
//   L567:   if dist²(champ, serpen) <= mr² { objective_in_attack_range = true }
//           else {
//   L568:     camp = MapDef::camp_pos(data.context.map(+0x20), Serpen(i8 5), team==0)   // 콜리 v3: 16a7af0(구 ffa3e0)
//             if !objective_helpers::v23_should_break_objective_hunt_anchor(player, data, champ, serpen, camp.x, camp.y) {   // 콜리 v3: f89c30
//   L569:       if !(posture.is_some() && posture.kind ∈ {WaitGroup(3), SoftDisengage(4)}) {   // `kind-3 <u 2`
//               // ★0.6.0 신규(ObjContest 진입점 게이트): 세르펜보다 진입점이 더 가까우면 진입점으로 이동
//               if team_plan.v6_obj_armed(+0xcc7) != 0 {                                   // 런타임 실측 v>=2 상시 1
//                 s = contest_entry_pos(team_plan, slot=0 /*Serpen*/, team==0, data)     // efb5b0 = (camp_pos(0, !team0) + camp_pos(5, !team0)) / 2 (x,y 각각 · camp_pos 16a7af0)
//                 if dist²(champ, serpen) > dist²(champ, s) {
//                   res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, s.x, s.y, end_delay=5)))   // ecde50 → e966a0(extend)
//                   return res → L431(배치 H) } }
//   L570:         res.push(Trace(new_attack_range(data, serpen.id(+0x5c0), 5)))   (SmallActionPlay::push 호출) · tag 13
//   L572:         return res → L431(배치 H)
//         } } } }
//   L577: strategy = player.strategy(rnd, game) ; not_kill_priority = strategy.object_finish(+0xf) != KillPriority(0)
//   L592: positioning_accuracy = AthleteParameter::positioning_accuracy(&player.info.parameter(+0x180 · 0.5.8 값 · 0.6.0 미확인)) ; min_v = positioning_accuracy
//   L594: max_v = 2000 - positioning_accuracy
//   L596: near_enemies: bumpalo Vec<&Entity> = data.cache.iter_champions().filter_map(..).filter(F3(champ)).collect   // F3 = 클로저 s1_0(캡처 champ): e.is_visible_from(&champ.team)(entity.rs:1482 인라인: champ.team Neutral(tag 1)→true / Player(t)→ e.visible_state[t](+0x38+24t).tag==Visible(0), t>=2 → panic_bounds_check) && dist²(e, champ) < 25600000000 (=160000², `icmp ult`) — 아군 포함 여부는 iter_champions 심(filter_map) 소관
//   L597: me_die_tick = fight_check::check_kill_die_tick(version, data, judger=player, focus=champ, &near_enemies.clone()[e17d50 복제], &towers=Vec::new_in(pool), simple=false, ignore_nuke=false, no_noise=false, &None)   // ★0.6.0 신 ABI eda920(구 eb82d0 · 인자 (…, near_enemies.clone(), Vec::new_in(pool), debug)) · 콜사이트 +0xa27b
//   L600: runaway = false ; force_runaway = false ; for enemy in near_enemies {
//   L601:   if !(serpen.is_none() || not_kill_priority || fight_model::can_enemy_hit_objective(enemy, serpen, 25000)) { continue }
//   L605:   jrng = utils::range_misjudge_rng(version, data, player, enemy.id)   (16B)
//   L606:   mr       = battle::max_range_can_use(champ, enemy)  * utils::range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000   // `roll` 은 변수가 아니라 매번 새 range_misjudge_roll 호출(적 1명당 3회 + me_die_tick<tps 시 1회)
//   L607:   emr      = max_range_can_use(enemy, champ)          * roll / 1000
//   L608:   emr_near = battle::max_range_nearly_can_use(enemy, champ, 40) * roll / 1000
//   L609:   dist = dist²(champ, enemy)
//   L612:   if me_die_tick < data.context.setting(+0x8).tick_per_second(+0x12f8) {
//   L625:     emr = max_range_nearly_can_use(enemy, champ, 60) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000   // 4번째 roll 호출 — 이 가지(me_die_tick < tps)에서만 rnd 추가 소비
//   L627:     if dist > mr² && emr < mr { L629: res.push(Trace(SmallActionTrace::new(data, enemy.id, 5)))  }   // attack_range_only=false · tag 13
//   L630:     else if dist <= emr² { runaway = true; force_runaway = true }
//           } else {
//   L613:     if Entity::remain_action_time(enemy) > 10 && (max_range_can_use(champ,enemy)*roll) > 999 {   // ≡ mr >= 1
//   L614:       if dist > mr² { L615: res.push(Trace(new(data, enemy.id, 5))) }
//   L617:     } else if emr < mr && dist > mr² { L619: res.push(Trace(new(data, enemy.id, 5))) }
//   L620:     else if dist <= emr_near² { runaway = true }
//           } }
//   L638: for e in data.cache.others(+0xf0)[1-team] { L639: if let Some(atk)=e.attack_effect { L640: range = max_range(e, champ); L641: if dist²(champ,e) <= range² { runaway=true; force_runaway=true; break } } }
//   L649: break_objective_anchor = serpen.map_or(false, |s| v23_should_break_objective_hunt_anchor(player, data, champ, s, camp_pos(Serpen, team==0)))   // f89c30 2회째
//   L651: if runaway && (break_objective_anchor || !objective_in_attack_range || force_runaway) {
//   L652:   res.push(RunAway(new_with_skill(data, player, 5, false)))
//   L653: } else if objective_in_attack_range {
//   L654:   res.truncate(0)
//   L655:   if let Some(serpen) = serpen {
//   L656:     margin = 안전 공격 여유 [인라인 헬퍼 L16~29, "v27 serpen safe attack position" 소유 함수 추정]:
//               range = max_range(champ, serpen) ; inner = clamp(range.saturating_sub(10000), 12000, 45000) ; dist = Entity::distance(champ, serpen)
//               if dist + 15000 < range { if inner + dist > range { rnd.gen_range(12000..=inner) } else { 12000 } } else { inner }
//   L657:     res.push(Trace(new_attack_range_margin(data, serpen.id, 5, margin)))   // attack_range_only=true · tag 13
//   L659:   } else { res.push(Stop) }   // tag 18   // ★0.6.0 19→18
//   L661: } else if res.is_empty() {
//   L662:   res.push(RunAway(new_with_skill(data, player, 5, false)))
//         }
//   L665: return res → move_actions(%87) ; near_enemies drop → L431(배치 H)
// ── 이 배치 범위 끝. L431 이후(act_actions·move_actions 병합·평가) = 배치 H, L489~514 = 배치 I

// serpen_hunt.rs:431~486 (배치 H)
// 진입: 배치 G 의 428 거대 문이 끝난 뒤. 로컬: nearest_enemy_tower=%96 · act_actions=%91 · move_actions=%87 · champ=%115 · team=%104 · pos=%110

431: if let Some(nearest_tower) = nearest_enemy_tower {                       // null 검사
432:   if let EntityType::Tower(info) = &nearest_tower.ty {                     // +0x68 == 2
433:     if info.nearest_enemy.map(|(_, id)| id) == Some(champ.id) {          // 태그 trunc→i1, (+0x98) == champ.id(+0x5c0)
434:       act_actions.truncate(0);
435:       move_actions.truncate(0);
436:       move_actions.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)));  // tag=3
       } } }

441: if !act_actions.is_empty() {                                              // len(+0x18)==0 → 456 경로로 직행
442:   let moba = game.get_game_mode().moba().unwrap();                          // vtable+0x40 · tag!=0 → unwrap_failed(패닉)
       let serpen = moba.jungle_runner.serpen.live_list.get(0).and_then(|id| game.get_entity_by_id(*id));   // len==0 → None · vtable+0x1f0 · null → None
       if let Some(serpen) = serpen {                                            // None → 455
443:     let attacking_objective = act_actions.iter().any(|a| a.get_action().target() == Some(serpen.id));
           // 클로저$10 인라인: 태그(+0xb1)-14 < 4 (Attack/Skill/Skill2/Ult) && +0x8 target == serpen.id   // ★0.6.0 -15→-14
444:     if attacking_objective {
445:       if let Some(action) = self.v27_objective_safe_attack_position(version, rnd, player, data, &parameter.positioning_score, serpen) {   // ▼ 전량 인라인(helper 43~145)
446:         if data.context.debug {                                          // ctx+0x3b
447:           debug.infos.entry(champ.id).or_insert(vec![]).push("v27 serpen safe attack position".to_string());
             }
449:         return bumpalo::vec![in ctx.pool; SmallActionPlay::AroundPosition(action)];   // memcpy 184B → from_iter_in(sret) → 함수 종료
450:         (unwind 시 action drop)
           }
         }
       }
455:   // if !act_actions.is_empty() → 511(배치 I): act_actions 를 그대로 반환 · 비면 456 으로
     }

// ───── helper v27_objective_safe_attack_position(serpen_hunt.rs:43~145) — 445 에 인라인. objective = serpen ─────
 43: let champ = data.cache.player_champion[team][pos]?;                         // 재로드 · null → None
 44: let objective_hp_ratio = objective.hp(0x670)*100 / max(objective.stat_cached.hp(0x628), 1);
 45: if objective_hp_ratio < 36 { return None }
 49: let attack_range = objective_attack_range(champ, objective);
       // = [16] champ.attack_effect.as_ref().unwrap()(0x4c0 tag==-1 → unwrap_failed)
       //   [17] Effect::range(champ)(effect.rs:26 = 0x438 + 0x4a0 + (level(0x5c8)-1)*0x4a8) + range_adjust(atk, champ, objective)[vt+0x108] + champ.radius() + objective.radius()
 52: if distance_sq(champ, objective) > attack_range*attack_range { return None }
 56: let threats: bumpalo Vec<&Entity> = game.iter_champions(player_champion[1-team]).filter(|t|
 58:      blackboard[player.team].is_recent_visible(game, player, t)             // stride 0x5c8 · last_seen +0x3e8
 59:      && (distance_sq(t, champ) <= 260000² || distance_sq(t, objective) <= 220000²)
     ).collect_in(ctx.pool);
 61: if threats.is_empty() { return None }
 65: let hp_ratio = champ.hp*100 / max(champ.stat_cached.hp, 1);
 66-68: let current_nearest_enemy = threats.iter().map(|t| distance(champ.x, champ.y, t.x, t.y)).min().unwrap();   // 첫 원소 + fold(min_by Ord::cmp, 동률 시 앞 원소)
 70-72: let threat_contact = threats.iter().any(|t| {
 71:        let range = max(max_range_nearly_can_use(t, champ, 50), 100000) + (if hp_ratio < 50 { 70000 } else { 40000 });   // (루프 밖 호이스트)
 72:        distance_sq(champ, t) <= range*range });
 74: if !threat_contact && !(hp_ratio < 45 && current_nearest_enemy <= 220000) { return None }   // (`< 220001`)
 78: let keep_attack_range = attack_range.saturating_sub(30000);
 83-85: let allies: Vec<&Entity> = game.iter_champions(player_champion[team]).filter(|a| a.id != champ.id && distance_sq(a, objective) <= 220000²).collect_in(pool);
 87-91: let ally_centroid: Option<(u64,u64)> = if allies.is_empty() { None } else { Some((Σa.x / n, Σa.y / n)) };   // udiv n
 93: drop(allies)
 98-99: let (xi, yi) = (positioning_score.cx as i32, positioning_score.cy as i32);   // 7×7 탐색창 중심 셀
100-101: let here_value = v27_positioning_value(version, player, &position_score_at_position(version, player, data, positioning_score, champ.x, champ.y, Objective));   // ff5ec0
     // v27_positioning_value(34~38): positioning = player.info.parameter.positioning_effective();
     //   gain_weight = (100 + (50-positioning)/10).clamp(95,105); risk_weight = (100 + (positioning-50)/2).clamp(75,125);
     //   trajectory_penalty = if score.on_trajectory || score.on_periodic_trajectory { max(positioning-50, 0) } else { 0 };
     //   value = score.gain*gain_weight/100 - score.risk*risk_weight/100 - trajectory_penalty   (IR: sdiv 100 · sdiv -100)
102: let here_score = here_value + min(current_nearest_enemy, 320000)/10000 + 8;   // (+8 은 현위치에만)
103:   + cohesion(champ.x, champ.y)   // closure$6(95): ally_centroid.map_or(0, |(ax,ay)| 320000.saturating_sub(distance(x,y,ax,ay))/10000)
105: let mut best: Option<(x, y, score, nearest_enemy)> = None;
106: for dx in 0..7 {  let xb = xi - 3 + dx;                                    // (xb>29 면 내부 루프 통째 skip)
107:   for dy in 0..7 {
109:     let yb = yi - 3 + dy;
110:     if xb < 0 || yb < 0 || yb > 29 || map.walls[yb][xb] != 0 { continue }   // (MapDef+0x78 [30][30])
114-115: let (x, y) = (xb*32000+16000, yb*32000+16000);
116:     if distance_sq((x,y), objective) > keep_attack_range² { continue }
120-123: let nearest_enemy = threats.iter().map(|t| distance(x, y, t.x, t.y)).min().unwrap_or(u64::MAX);
124-125: let cell_value = v27_positioning_value(version, player, &position_score_at_cell(version, player, data, positioning_score, xb, yb, Objective));
126:     let enemy_spacing = min(nearest_enemy, 320000)/10000;
127:     let range_slack = min(keep_attack_range.saturating_sub(distance(x, y, objective.x, objective.y)), 80000)/10000;
128:     let score = cell_value + enemy_spacing + range_slack + cohesion(x, y);
130:     if best.map_or(true, |b| score > b.score) {                              // (score <= best.score 면 유지 → 동률 시 먼저 찾은 셀)
131:        best = Some((x, y, score, nearest_enemy)) }
     } }
136: let (bx, by, best_score, best_nearest) = best?;
137: if best_score < here_score && best_nearest < current_nearest_enemy.saturating_add(30000) { return None }
140: if distance_sq(champ, (bx,by)) <= 12000² { return None }                    // (`< 144000001`)
144: Some(SmallActionAroundPosition::new_with_radius(rnd, data, bx, by, 3, 25000))   // None 판정 = byte0xb1 == 0xff
145: drop(threats)
// ───── helper 끝 ─────

455: if act_actions.is_empty() {                                               // (비지 않으면 → 511 배치 I: act_actions 반환)
456:   let positioning_accuracy = player.info.parameter.positioning_accuracy();
458:   let (min_v, max_v) = (positioning_accuracy, 2000 - positioning_accuracy);
462:   let danger = game.iter_champions(player_champion[1-team]).any(|c| {      // (슬롯 0..5 순, any 단락)
463:      let jrng = range_misjudge_rng(version, data, player, c.id);
464:      let emr = max_range_can_use(c, champ) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000 + 10000;   // (호출 순서 = rnd 미러 순서)
465:      let mr = max_range_can_use(champ, c);
467:      blackboard[1-team].is_recent_visible(game, player, c)                // (★적팀 블랙보드 · stride 0x5c8)
468:      && distance_sq(c, champ) <= emr*emr
469:      && !c.is_in_action() && !c.block_input()                             // (is_in_action: ty==13 && action_state.tag>=3)
471:      && mr == 0 });                                                        // (IR: (mr!=0 || block_input) → 다음 원소; 469/471 의 && 결합 순서는 표기 불가)
472:   if danger { return bumpalo::vec![in pool; SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))] }   // → 함수 종료
475:   let best_move_action = move_actions.iter().max_by_key(|a| self.score(version, parameter, rnd, player, data, a, debug)).unwrap();   // (첫 원소는 serpen_action_score 로 인라인 · fold, 동률 시 뒤 원소) · 빈 목록이면 unwrap_failed(패닉)
479:   let incoming = game.iter_projectile().any(|p|                             // vtable+0x210 · next
480:        p.team != TeamType::Player(team) && !p.is_targeting()               // (tag0==0 && +8==team → skip) · projectile.rs:134: Target/TargetSplash/BouncingTarget{target_id:Some} → skip
481:        && distance_sq(champ, (p.x, p.y)) < 420000² );                       // (+0x100/+0x108)
482-483: let enemy_rushing = game.iter_champions(player_champion[1-team]).any(|c| matches!(c.rush_state, RushState::Rush{..} | RushState::RushPenetrate{..}));   // 5슬롯 언롤 · +0x308 태그 sgt -1(암묵 RushPenetrate) || == 0x8000000000000003(Rush)
484:   if incoming || enemy_rushing {
485:     let input = best_move_action.clone().get_input(version, rnd, player, data, &parameter.positioning_score, debug);   // clone → get_input(sret 32B)
486:     drop(clone)
         → 489(배치 I): switch input.tag(+0 i64: -1 / 0 / 기타)                 // 조건 거짓이면 489 (None 케이스와 같은 코드)
       }
     }

// serpen_hunt.rs:489~514 (배치 I)
// 진입 문맥: L455 `if act_actions.is_empty()` 의 참 가지 안에서 L475 best_move_action · L479~483 trajectory_possible: bool · L484~488 `let move_action_input: Option<Input> = if trajectory_possible { best_move_action.clone().get_input(...) } else { None };`
//   trajectory_possible==false 경로는 get_input 을 호출하지 않고 곧장 L506 경로로 = None 케이스와 같은 코드. L455 거짓 가지(act_actions 비어있지 않음)는 L511.

// ── L489 `if let Some(move_action_input) = move_action_input {`
tag = *(move_action_input as *i64)
switch tag {
  -1 (None)              => goto L506
  0  (Some(Input::Move)) => goto L490
  _  (Some(그 외 5종))    => goto L503
}
// ── L490 `if let Input::Move { x, y } = move_action_input {`
x = move_action_input.+0x8 ; y = move_action_input.+0x10
// ── L492~495
position_score: PositioningScore(56B) = position_score_at_position(version, player, data, &parameter.positioning_score /*+0x9f0*/, x, y, PositionEvalPurpose::Objective /*i8 11*/)   // ff5ec0
on_trajectory = position_score.on_trajectory /*+0x30*/ || position_score.on_periodic_trajectory /*+0x31*/   // 단락: +0x30 참이면 +0x31 미로드
// ── L497
if on_trajectory {
  // L498: return from_iter_in([SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))], data.context.pool)   // elem.+0xb1 = 3
} else {
  // L500: return from_iter_in([best_move_action.clone()], data.context.pool)
}
// L503: return from_iter_in([best_move_action.clone()], pool)   (Some(Move) 가 아닌 Some(_))
// L506: return from_iter_in([best_move_action.clone()], pool)   (None 및 trajectory_possible==false)
// ⟹ 500/503/506 은 바이트 단위로 동일한 결과. 차이는 오직 498 RunAway 로 갈 수 있느냐.
// ── L511: act_actions 를 그대로 반환(이동 · %91 드롭 안 함)
// ── L513 지역 Vec 드롭(경로별 순서: 489~506 경로: move_actions → act_actions(%91) → act_actions(%94) · 511 경로: %87 → %94 · 472 경로: %87 → %91 → %94) · 각 drop = <Vec<SmallActionPlay> as Drop>::drop + 인라인 bumpalo dealloc(마지막 할당만 되돌림)
// ── L514 ret void.
// 이 범위에서 push 되는 variant 집합: {RunAway(태그3 · new_with_skill(data, player, 5, false)), best_move_action 의 variant(clone) ∈ {RunAway 3, AroundPosition(암묵), Trace 13, Stop 18}}. self 쓰기 0 · debug 쓰기 0 · TLS 0.
```
// ★v2 사장: 이 함수에 version 분기 없음(version 은 콜리로만 전달).

## changes
- L569 안 신규(ObjContest 진입점 게이트): `team_plan.+0xcc7 != 0` 이면 `s = efb5b0(team_plan, 0, team==0, data)`(= (camp_pos(0,!team0)+camp_pos(5,!team0))/2) · `dist²(champ,serpen) > dist²(champ,s)` 이면 Trace 대신 `AroundPosition::new(rnd,data,s.x,s.y,5)` push 후 반환.
- SmallActionPlay 태그 재번호(AroundHide 제거): Trace 14→13 · Attack 15→14 · Skill 16→15 · Skill2 17→16 · Ult 18→17 · Stop 19→18 · 니치 기본 7→6 · L245/L443 `tag-15<4`→`tag-14<4`.
- Effect vt: expected_heal/shield/buff +0x40/+0x48/+0x50 → +0x58/+0x60/+0x68 · expected_rush/move_on_hit +0x60/+0x68 → +0x80/+0x88 · range_adjust +0xe8→+0x108.
- PlayerState team/position +0x930/+0x9c0 → +0xa00/+0xa90 · Strategy +0x4f8→+0x568 · Blackboard stride 0x2e8→0x5c8 (last_seen +0x3e8).
- L597 check_kill_die_tick 신 ABI eda920: (version, data, player, focus=champ, &enemy Vec(복제 e17d50), &towers=빈 Vec, false, false, false, &None).
- 콜리 v3: expected_damage_target 1643790 · position_score_at_position ff5ec0 · v23_should_break f89c30 · camp_pos 16a7af0 · AroundPosition::new ecde50 · push/extend e966a0 · 신규 efb5b0.
- 미확인 오프셋(0.5.8 값 유지·표기): MobaMode live_list +0x1d0/+0x1d8 · cache.jungles +0xd0 / others +0xf0 · parameter +0x180 · ObjectivePosture 레이아웃(+0x50/+0x38/+0x20/+0x28) · debug.infos +0xa0.

## verified
- capstone e986c0 140e9a2c0~140e9a3bb: f89c30 → (posture kind-3<2) → `cmp byte [tp+0xcc7],0; je Trace경로` → efb5b0(tp, 0, team==0, data) → dist² 비교 `cmp r15,rsi; jbe Trace경로` → ecde50(sret, rnd, data, x, y, 5) → memcpy 0xb8 → e966a0 → 합류.
- Ghidra 0.6.0 efb5b0 디컴: camp_pos(16a7af0) 2회(type (slot==0?0:2) / (slot==0?5:4) · side = !team0) → 합/2.
- e986c0 콜 히스토그램: eda920 1회(콜사이트 인자 0,0,0 · &None) · ff5ec0 2회(purpose 0xb · +0x9f0 확인) · f89c30 2회 · ecde50 1회 · efb5b0 1회 · 16a7af0 1회 — 0.5.8 구조와 정합.
- 미확인: 배치 G 의 F1/F2/score 필터·배치 H/I 본체는 RE §13 「동치(추정) · 콜 순서 재배치 2구간」 인용(스팟체크 범위 밖).

## confidence
A(신규 게이트) / B(나머지 = RE 동치 추정 + 태그·슬롯 기계 치환 · 미확인 오프셋 5종).
