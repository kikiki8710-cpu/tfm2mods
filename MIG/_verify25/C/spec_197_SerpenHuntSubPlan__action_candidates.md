---

### `197` SerpenHuntSubPlan::action_candidates — 세르펜 사냥 서브플랜의 행동 후보 목록 생성 — need_recall 판정→Recall 단독 / v27 규율 행동 단독 / 공격 후보(battle·summon·jungle)를 타워·대상·아군스킬·score 필터로 거른 뒤 get_move_action(추적·도주·대기)과 합쳐(431~514, 배치 H·I) 반환

| 항목 | 값 |
|---|---|
| id | `serpen_hunt__SerpenHunt__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan11serpen_huntNtB2_17SerpenHuntSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:201` |
| IR | `m02.ll` 11075~17795행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `cb1ce0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[197]/sig/tls/<키>`)**

- `direct`: 없음 — 이 배치 범위(본체 11261~14415 + aux 9조각) 안에 LocalKey::with / thread_local 접근 0건(grep 실측)
- `indirect_call_order`: ["L226 team_plan.v27_objective_discipline_action(version, rnd, player, data, Serpen)", "L236 fight_check::battle_action(version, rnd, player, data, 5) → attack_summon_action(player, data) → self.attack_jungle_action(team, pos, data)", "L237 PlayerState::strategy(player, rnd, game)", "L243~312 F1: should_ignore_object_finish_kill_priority_target · Effect::is_in_range · is_in_range_ex", "L314 F2: EffectType::expected_heal/expected_shield/expected_buff · buff_value::aoe_heal_covers_low_ally", "L423 self.score(version, parameter, rnd, player, data, a, debug) — 후보마다", "L523 utils::nontarget_windup_perceived(version, player, data, c) — 적 챔피언 ≤5회", "L535 position_eval::position_score_at_position(version, player, data, positioning_score, champ.x, champ.y, Objective)", "L543 team_plan.v25_objective_posture(version, player, data, Serpen)", "L568 / L649 v23_should_break_objective_hunt_anchor(player, data, champ, serpen, camp)", "L597 fight_check::check_kill_die_tick(version, rnd, data, player, champ, near_enemies.clone(), Vec::new_in, debug)", "L605~608 utils::range_misjudge_rng / range_misjudge_roll · battle::max_range_can_use / max_range_nearly_can_use — near_enemies 마다"]
- `note`: SIEGE_STANCE_CACHE·CAST_BEAMS·HP_VALUE_MEMO·POS_EVAL_CACHE·INTER_CTX 중 어느 것을 어느 콜리가 만지는지는 이 배치가 독립 재현 규칙상 확인하지 않았다(콜리 명세 = r13~r15 계약). 미러 설계는 위 호출 순서를 그대로 따르면 된다

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::Vec<SmallActionPlay> (32B) | 반환 Vec. ptr@+0x0 · bump@+0x8 · cap@+0x10 · len@+0x18 전부 기록됨(new_in 이 ptr=8(dangling)/cap=0/len=0 으로 초기화 후 push 가 ptr·cap·len 갱신). 이 배치 안의 반환 경로 3곳: L223(Recall 1개) · L227(from_iter_in [action;1]) — 둘 다 32B 통째 memcpy \| (배치 H) ptr@0 · bump@+8 · cap@+0x10 · len@+0x18 (m02.ll:16008/17482 의 from_iter_in 이 32B 전부 기록). 배치 H 에서 %0 을 쓰는 지점 2: 449(m02.ll:16008 = vec![AroundPosition]) · 472(17482 = vec![RunAway]). 511(배치 I)의 act_actions 반환은 이 범위 밖 \| (배치 I) 이 범위의 5개 return 지점 전부가 여기에 쓴다 — 498/500/503/506 은 from_iter_in([1개]) 결과, 511 은 act_actions(%91) 를 32B memcpy | 4 |
| 1 | 1 | self | &mut SerpenHuntSubPlan (1B: need_recall bool @+0x0) | initializes 속성 없음. 쓰기 표면 = +0x0 need_recall 1바이트뿐(writes 참조) \| (배치 H) 배치 H 범위 안 store 0건. 475 의 max_by_key 클로저(sa_0)가 %1 을 캡처해 SerpenHuntSubPlan::score(self,…) 에 &self 로만 넘긴다(m12.ll:22829). IR 속성 noalias·dereferenceable(1) 이고 readonly 아님 → 다른 배치 범위 쓰기 여부는 그쪽 담당 \| (배치 I) 이 범위(489~514)에서 읽기·쓰기 0건 (IR 속성 noalias·dereferenceable(1) · initializes 없음) | 4 |
| 2 | 2 | version | usize | 이 배치 범위에선 분기 없음 — 콜리 인자로만 전달(v27·battle_action·get_move_action 내부 콜리·F2 aoe_heal_covers_low_ally·score). 스택 %102 에 저장돼 클로저가 &version 으로 캡처 \| (배치 H) 본체 진입부에서 %102(alloca 8B)에 store(m02.ll:11208). 배치 H 안 분기 없음 — 전달만: v27 helper(1389→position_score_at_*) · range_misjudge_rng(16476) · serpen_action_score(16679) · get_input(17007) \| (배치 I) %102 슬롯에서 재로드해 position_score_at_position 1번째 인자로 전달(m02.ll:17034) — 이 범위 자체 분기 없음 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | 직접 쓰기 없음. 콜리로만 전달(v27·battle_action·strategy·AroundPosition::new·range_misjudge_roll·check_kill_die_tick·L656 gen_range(12000..=inner)·score) \| (배치 H) 배치 H 안 직접 read/write 0 · 소비 콜리: range_misjudge_roll(16488, 적 챔프 슬롯순 · any 단락) → new_with_radius(15640) → serpen_action_score/score(16679 + fold) → get_input(17007). PRNG 미러는 이 호출 순서를 지켜야 함 \| (배치 I) 이 범위 미사용 (485 get_input 인자 = 배치 H) | 4 |
| 4 | 4 | player | &PlayerState (2528B) | info.team(+0x930)·info.position(+0x9c0)·info.parameter(+0x180) 읽음 \| (배치 H) readonly · 0x930 info.team(블랙보드 인덱스, 클로저 안) · 0x180 info.parameter(positioning_effective 15445/15784 · positioning_accuracy 16271) \| (배치 I) position_score_at_position 2번째 · new_with_skill 2번째 인자 | 4 |
| 5 | 5 | data | &OperationData (24B: cache@+0x0 &AbstractGameWithCache · context@+0x8 &GameContext · blackboard@+0x10) | cache.game 팻포인터(+0x0 data_ptr / +0x8 vtable_ptr) 로 get_game_mode(+0x40)·get_entity_by_id(+0x1f0)·get_tick(+0x28) 디스패치 \| (배치 H) readonly · +0 cache(&AbstractGameWithCache: +0/+8 = &dyn AbstractGame 팻포인터, +0x1e0 player_champion[2][5]) · +8 context(&GameContext: +0 pool(bump) · +0x20 map · +0x3b debug) · +0x10 blackboard(&[Blackboard;2]) \| (배치 I) position_score_at_position 3번째 · new_with_skill 1번째 인자 · data.context.pool(%326, L312 계산) 이 from_iter_in 의 bump | 4 |
| 6 | 6 | parameter | &ScoreParameter (5384B) | 이 배치에선 &parameter.positioning_score(+0x9f0) 를 get_move_action 에 넘기고, F2(L423 score) 에 통째로 캡처 \| (배치 H) readonly · +0x9f0 positioning_score(PositioningScoreData 2760B; %348, position_score_at_* 인자) · +0x14a8 cx · +0x14b0 cy(7×7 탐색창 중심 셀, 15428/15432) · 475 클로저에 통째 전달 \| (배치 I) &parameter.positioning_score(+0x9f0 = %348, L428 계산) 를 position_score_at_position 4번째 인자로 | 4 |
| 7 | 7 | team_plan | &TeamPlan (1064B · DI: ref$<TeamPlan>) | v27_objective_discipline_action(L226)·v25_objective_posture(L543) 의 self 로만 전달. 본문에서 직접 필드 읽기 없음 \| (배치 H) 배치 H 범위 안 사용 0건(dbg_value 만) \| (배치 I) 이 범위 미사용 | 4 |
| 8 | 8 | debug | &mut DebugFrameData (224B) | L551 debug.infos(+0xa0 HashMap<usize,Vec<String>>) entry(champ.id).or_insert.push(String) — data.context.debug(+0x3b) 참일 때만. 그 외 check_kill_die_tick·score 에 전달 \| (배치 H) ★쓰기: 447 에서 +0xa0 infos(HashMap<usize,Vec<String>>).entry(champ.id).or_insert(vec![]).push("v27 serpen safe attack position") — data.context.debug 참일 때만(m02.ll:15995~16131). 그 외는 콜리 전달(serpen_action_score · get_input) \| (배치 I) 이 범위 미사용 (485 get_input 인자 = 배치 H) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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
//                         dmg = Effect::expected_damage_target(&serpen.attack_effect, data.context(+0x8), serpen as &dyn(vtable @anon.54 = Entity), champ)
// L207 (11365~11369):     if !(champ.hp(+0x670) > dmg*3) { self.need_recall = true }   // 즉 champ.hp <= 3*dmg
//                       }
//        }
//        (%162: phi need_recall = [1 ← L207 경로, 0 ← L213 경로] → store self+0x0)
// L220 (11382~11384): if self.need_recall {
// L221 (11413~11422):   res = bumpalo Vec::new_in(data.context.pool(+0x0 of GameContext))   // ptr=8(dangling)·bump·cap=0·len=0
// L222 (11425 · 17751~17790):   res.push(SmallActionPlay::Recall(SmallActionRecall::new(data, player, end_delay=5)))   // 136B → 184B 슬롯, tag@+0xb1 = 4(Recall)
// L223 (17792~17794):   return res   (sret 32B memcpy) → %286 = 함수 끝(L514, 배치 I 의 합류 라벨)
//        }
// L226 (11405~11409): if let Some(action) = team_plan.v27_objective_discipline_action(version, rnd, player, data, target=JungleType::Serpen(i8 5)) {   // sret 184B · tag@+0xb1 == -1 → None
// L227 (11430~11437):   return bumpalo::vec![in data.context.pool; action]   (from_iter_in<[SmallActionPlay;1]>) → %286(L514, 배치 I)
//        }
// L231~233 (11444~11685): nearest_enemy_tower: Option<&Entity>
//        = data.cache.iter_towers_without_nexus(1 - team)        // sret 120B = Chain<Flatten<IntoIter<[Option<&Entity>;6]>>, Copied<slice::Iter<&Entity>>>
//          .filter(|t| t.can_target(+0x6b9) && t.block_target_tick(+0x6a0) == 0)     // 클로저 s_0 (첫 6칸은 인라인 루프 11533~11599, 나머지 슬라이스는 try_fold<find::check<s_0>> 11624)
//          .min_by_key(|t| dist²(t, champ))   // (11649~11682: dx=|t.x-champ.x|, dy=|t.y-champ.y|, dx²+dy² ; Map<Filter<..>>::fold(min_by) 11682) ; 결과 ptr → %96
// L236 (11702~11750): act_actions(%94) = { 인라인 헬퍼(L668~673, 파일 미상 — 아래 unknown):
//          res = Vec::new_in(pool);
//          res.extend(fight_check::battle_action(version, rnd, player, data, _end_delay=5));      // L670
//          res.extend(fight_check::attack_summon_action(player, data));                           // L671
//          res.extend(self.attack_jungle_action(team, position(i32), data));                      // L672
//          res }
// L237 (11762~11779): strategy = PlayerState::strategy(player, rnd, data.cache.game)  (24B Strategy)
//        object_finish_objective(%93): Option<&Entity> =
//          if strategy.object_finish(+0xf) == KillPriority(0) {
// L238 (11784~11810):   get_game_mode() ; Moba 아님 → unwrap_failed(%327) ; live_list.len==0 → None
// L239 (11813~11826):   live_list.first().and_then(get_entity_by_id)
//          } else { None }
// L243~312 (11835~11873 · aux m01.ll:564~742): act_actions(%91) = act_actions.iter().filter(F1).map(|a| a.clone()).collect_in(pool)
//        F1 = 클로저 s2_0 (call_mut aux m02.ll:75123~75783) · 캡처 = (game data_ptr, game vtable, player, &object_finish_objective, champ, &nearest_enemy_tower)
//        F1(a) → keep(true) 판정 (IR 극성 = 분기 방향, %313 phi):
//          L245: if let Some(id) = a 의 대상 id(small_action.rs:309 헬퍼 — tag∈{15,16,17,18}=Attack/Skill/Skill2/Ult 이면 Some(+0x8)) {
//          L246:   if let Some(target) = game.get_entity_by_id(id) {
//          L247:     if fight_model::should_ignore_object_finish_kill_priority_target(player, target, *object_finish_objective) { return false }
//                  }}
//          L252: if let Some(id) = a 의 대상 id { if let Some(t) = get_entity_by_id(id) { if t.ty(+0x68) != Champion(13) { return true } } }   // 비-챔피언 대상은 무조건 유지
//          L253: match a {   // 니치 디코드: tag>2 ? tag-3 : 7(AroundPosition)
//            RunAway·Recall·Around·AroundHide·AroundRegion·AroundRunAway·Positioning·AroundPosition·AroundPositionBush·AroundBush·LaneMinionPosition·Trace·Stop → false
//            Attack(15):  L256: target = get_entity_by_id(a.target(+0x8)) else false
//                         L257: eff = champ.attack_effect(+0x490).unwrap()   (tag -1 → unwrap_failed)
//                         L258: if !eff.is_in_range(champ, target) → false
//                               nearest_enemy_tower None → true
//                               Some(t): if !(t.attack_effect.unwrap().is_in_range(t, champ) && target.ty != Tower(2)) → true
//                         L259:          else → return (target.team == champ.team)   // TeamType PartialEq(entity.rs:1127): tag 다르면 false, 둘 다 Neutral → true, Player 면 id 비교
//                                        (해석: 적 타워가 나를 때릴 수 있는 위치에서 타워가 아닌 상대 진영 대상 평타 후보는 버린다)
//            Skill(16):   L266: target = get_entity_by_id(a.target(+0x8)) else false
//                         L267: eff = champ.skill_effect(+0x4c8).unwrap()   (tag +0x4f8 == -1 → unwrap_failed)
//                         L268: if !eff.is_in_range(champ, target) → false ; 타워 게이트 = Attack 과 동일(L269 팀 비교, 불일치 → false; 통과 시 계속)
//                         L269: if !(eff.ty.<EffectType vtable+0x68 expected_move_on_hit>() || eff.ty.<+0x60 expected_rush_effect>()) → false
//                         L270: nearest_enemy_tower None → true ; Some(t) → return s2_0s4_0(target, t) = !t.attack_effect.unwrap().is_in_range_ex(t, target, t.x, t.y, target.x, target.y, 15000)
//            Skill2(17):  L281~285: 위와 동일, eff = (champ.level(+0x5c8) > 2 ? &champ.skill2_effect(+0x500) : &None).unwrap(), 중첩 클로저 s2_0s6_0
//            Ult(18):     L296~300: 위와 동일, eff = (champ.level > 4 ? &champ.ult_effect(+0x538) : &None).unwrap(), 중첩 클로저 s2_0s8_0
//          }
// L314 (11885~11893 · aux m01.ll:2147~5070): act_actions.retain(F2)   · 캡처 = (data, champ, player, &version)
//        F2(a) → keep 판정 (%1260 = 유지, %1269 = 제거):
//          L315: match a { Skill(16) → L317.., Skill2(17) → L351.., Ult(18) → L385.., 그 외(RunAway~Trace·Attack·Stop) → 유지 }
//          [Skill 갈래 L317~335 · Skill2 L351~369 · Ult L385~403 는 effect 만 다르고 동일 구조]
//          L317: target = get_entity_by_id(a.target(+0x8)) ; None → 제거
//          L318: if target.team != champ.team → 유지   (적 대상 스킬은 그대로)
//          L320: eff = champ.skill_effect.unwrap()  (None → 제거)   [Skill2: level>2 ? skill2_effect : None · Ult: level>4 ? ult_effect : None]
//          L321: has_heal   = eff.ty.<vtable+0x40 expected_heal>(data.context, champ as &dyn) != 0
//          L322: has_shield = eff.ty.<+0x48 expected_shield>(ctx, champ) != 0
//          L323: has_buff   = eff.ty.<+0x50 expected_buff>(ctx, champ).is_some()   (sret 288B, tag@+0x48 != -1)
//          L324~327: near_enemy = [player_champion[1-team] · iter_towers(1-team) · cache.jungles(+0xd0) · cache.others(+0xf0)[1-team]]
//                    .any(|e| dist²(e, target) < 14400000001 && e.is_visible_from(&champ.team))   // 120000²+1 · is_visible_from(entity.rs:1481): Neutral→true, Player(t)→visible_state(+0x38)[t].tag == Visible(0)
//          L328: hp_ratio = target.hp(+0x670)*100 / target.stat_cached.hp(+0x628)   (0 → div_by_zero panic)
//          L329: if has_heal && !has_buff && hp_ratio > 79 {
//          L331:    keep = (!has_shield || near_enemy) && buff_value::aoe_heal_covers_low_ally(version, eff, data, player, target)
//                } else {
//          L335:    keep = !has_shield || has_buff || near_enemy   (has_heal&&has_buff 는 무조건 유지)
//                }
// L423 (11903~11917 · aux m01.ll:5073~5262): act_actions.retain(|a| { L424: s = self.score(version, parameter, rnd, player, data, a, debug); L425: !(s < -30) })   // score < -30 → 제거
// L428 (11930~14415): move_actions(%87) = get_move_action(version, rnd, player, data, &parameter.positioning_score(+0x9f0), team_plan, debug)   // serpen_hunt.rs:517 인라인, 아래 전개
//   L517~519: res = Vec::new_in(pool) ; champ = player_champion[team][pos].unwrap()(%408)
//   L522~528: has_non_target_action_range = player_champion[1-team].iter().flatten().any(|c|
//       L523:   utils::nontarget_windup_perceived(version, player, data, c) && c.ty == Champion(13)
//       L524:   && match c.champion.action_state(+0x70) {
//                 Skill(4)  → e = c.skill_effect.unwrap() (tag +0x4f8: -1 → unwrap_failed) ; e.casting(+0x30) ∈ {Position(1), Direction(2)} && Effect::is_in_range(e, c, champ)
//       L526:    Skill2(5) → e = (c.level>2 ? skill2_effect : None).unwrap() ; 동일
//       L528:    Ult(6)    → e = (c.level>4 ? ult_effect : None).unwrap() ; 동일
//                 _ → false })
//   L535: position_score = position_eval::position_score_at_position(version, player, data, positioning_score, champ.x(+0x660), champ.y(+0x668), purpose=Objective(i8 11))   // sret 56B PositioningScore
//   L537: if position_score.on_trajectory(+0x30) || has_non_target_action_range || position_score.on_periodic_trajectory(+0x31) {
//   L539:   res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, end_delay=5, with_skill=true)))   // tag 3
//   L540:   return res → move_actions → L431(배치 H)
//         }
//   L543: posture = team_plan.v25_objective_posture(version, player, data, target=Serpen(i8 5))   // sret 88B Option<ObjectivePosture>, None = focus_enemy tag(+0)== -1
//   L544: if let Some(posture) = posture {
//   L545:   if posture.kind(+0x50) > 2 {   // WaitGroup(3)|SoftDisengage(4)
//   L546:     if kind == SoftDisengage(4) && posture.near_enemy_count(+0x38) != 0 {
//   L547:       res.push(RunAway(new_with_skill(data, player, 5, false)))  }
//   L549:     res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, posture.wait_pos.0(+0x20), .1(+0x28), end_delay=5)))   // 니치 untagged(태그 store 없음)
//   L550:     if data.context.debug(+0x3b) {
//   L551:       debug.infos(+0xa0 HashMap<usize,Vec<String>>).entry(champ.id(+0x5c0)).or_insert(vec![]).push(format!(@anon.126 …{:?}, posture.kind))  }
//   L553:     return res → L431(배치 H)
//           }
//   L556:   if kind == Screen(2) && posture.focus_enemy(+0x0 tag)==Some {
//   L557:     focus_enemy = posture.focus_enemy(+0x8)
//   L558:     res.push(Trace(SmallActionTrace::new_attack_range(data, focus_enemy, end_delay=5)))   // = new_attack_range_margin(…, 15000): attack_range_only=true
//           }  (그 외 Commit/HoldCamp/Screen-무초점 → 통과)
//         }
//   L564: serpen = get_game_mode() Moba(m) → m.…live_list.first().and_then(get_entity_by_id) ; Moba 아님 → unwrap_failed(%1293)
//   L565: objective_in_attack_range = false ; if let Some(serpen) = serpen {
//   L566:   mr = max_range(champ, serpen) [인라인 헬퍼 L16~26]: eff = champ.attack_effect.unwrap()(None → unwrap_failed)
//             = eff.range(+0x4a0) + champ.stat_buff_cached.range(+0x438) + eff.growth_range(+0x4a8)*(champ.level(+0x5c8)-1)
//               + Effect::range_adjust(eff, champ, serpen) + radius(champ) + radius(serpen)
//             radius(e) = e.stat_buff_cached.radius_mult(+0x470)==0 ? e.radius(+0x680) : e.radius*(mult+100)/100
//   L567:   if dist²(champ, serpen) <= mr² { objective_in_attack_range = true }
//           else {
//   L568:     camp = MapDef::camp_pos(data.context.map(+0x20), Serpen(i8 5), team==0)
//             if !objective_helpers::v23_should_break_objective_hunt_anchor(player, data, champ, serpen, camp.x, camp.y) {
//   L569:       if !(posture.is_some() && posture.kind ∈ {WaitGroup(3), SoftDisengage(4)}) {   // `kind-3 <u 2`
//   L570:         res.push(Trace(new_attack_range(data, serpen.id(+0x5c0), 5)))   (SmallActionPlay::push 호출)
//   L572:         return res → L431(배치 H)
//         } } } }
//   L577: strategy = player.strategy(rnd, game) ; not_kill_priority = strategy.object_finish(+0xf) != KillPriority(0)
//   L592: positioning_accuracy = AthleteParameter::positioning_accuracy(&player.info.parameter(+0x180)) ; min_v = positioning_accuracy
//   L594: max_v = 2000 - positioning_accuracy
//   L596: near_enemies: bumpalo Vec<&Entity> = data.cache.iter_champions().filter_map(..).filter(F3(champ)).collect   // F3 본문은 m01/m06 의 iter_champions 클로저 심(이 배치 미독해 — unknown)
//   L597: me_die_tick = fight_check::check_kill_die_tick(version, rnd, data, player, champ, near_enemies.clone(), Vec::new_in(pool), debug)
//   L600: runaway = false ; force_runaway = false ; for enemy in near_enemies {
//   L601:   if !(serpen.is_none() || not_kill_priority || fight_model::can_enemy_hit_objective(enemy, serpen, 25000)) { continue }
//   L605:   jrng = utils::range_misjudge_rng(version, data, player, enemy.id)   (16B)
//   L606:   mr       = battle::max_range_can_use(champ, enemy)  * utils::range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000
//   L607:   emr      = max_range_can_use(enemy, champ)          * roll / 1000
//   L608:   emr_near = battle::max_range_nearly_can_use(enemy, champ, 40) * roll / 1000
//   L609:   dist = dist²(champ, enemy)
//   L612:   if me_die_tick < data.context.setting(+0x8).tick_per_second(+0x12f8) {
//   L625:     emr = max_range_nearly_can_use(enemy, champ, 60) * roll / 1000
//   L627:     if dist > mr² && emr < mr { L629: res.push(Trace(SmallActionTrace::new(data, enemy.id, 5)))  }   // attack_range_only=false
//   L630:     else if dist <= emr² { runaway = true; force_runaway = true }
//           } else {
//   L613:     if Entity::remain_action_time(enemy) > 10 && (max_range_can_use(champ,enemy)*roll) > 999 {   // ≡ mr >= 1 (표기 불가: 소스는 `mr > 0` 추정)
//   L614:       if dist > mr² { L615: res.push(Trace(new(data, enemy.id, 5))) }
//   L617:     } else if emr < mr && dist > mr² { L619: res.push(Trace(new(data, enemy.id, 5))) }
//   L620:     else if dist <= emr_near² { runaway = true }
//           } }
//   L638: for e in data.cache.others(+0xf0)[1-team] { L639: if let Some(atk)=e.attack_effect { L640: range = max_range(e, champ); L641: if dist²(champ,e) <= range² { runaway=true; force_runaway=true; break } } }
//   L649: break_objective_anchor = serpen.map_or(false, |s| v23_should_break_objective_hunt_anchor(player, data, champ, s, camp_pos(Serpen, team==0)))
//   L651: if runaway && (break_objective_anchor || !objective_in_attack_range || force_runaway) {
//   L652:   res.push(RunAway(new_with_skill(data, player, 5, false)))
//   L653: } else if objective_in_attack_range {
//   L654:   res.truncate(0)
//   L655:   if let Some(serpen) = serpen {
//   L656:     margin = 안전 공격 여유 [인라인 헬퍼 L16~29, 문자열 "v27 serpen safe attack position"(@anon.129) 의 소유 함수 추정]:
//               range = max_range(champ, serpen) ; inner = clamp(range.saturating_sub(10000), 12000, 45000) ; dist = Entity::distance(champ, serpen)
//               if dist + 15000 < range { if inner + dist > range { rnd.gen_range(12000..=inner) } else { 12000 } } else { inner }
//   L657:     res.push(Trace(new_attack_range_margin(data, serpen.id, 5, margin)))   // attack_range_only=true
//   L659:   } else { res.push(Stop) }   // tag 19
//   L661: } else if res.is_empty() {
//   L662:   res.push(RunAway(new_with_skill(data, player, 5, false)))
//         }
//   L665: return res → move_actions(%87) ; near_enemies drop → %1311 = L431(배치 H)
// ── 이 배치 범위 끝. L431 이후(act_actions·move_actions 병합·평가) = 배치 H, L489~514 = 배치 I

// serpen_hunt.rs:431~486 (배치 H)
// 진입: 배치 G 의 428 거대 문이 끝난 뒤 블록 %1311(m02.ll:14417). 로컬: nearest_enemy_tower=%96(Option<&Entity>, 233 기록) · act_actions=%91 · move_actions=%87 · champ=%115 · team=%104 · pos=%110

431: if let Some(nearest_tower) = nearest_enemy_tower {                       // 14421 null 검사
432:   if let EntityType::Tower(info) = &nearest_tower.ty {                     // 14427 +0x68 == 2
433:     if info.nearest_enemy.map(|(_, id)| id) == Some(champ.id) {          // 14442 태그 trunc→i1, 14456 (+0x98) == champ.id(+0x5c0) · option.rs:1161/2440 인라인
434:       act_actions.truncate(0);                                            // 14473
435:       move_actions.truncate(0);                                           // 14478
436:       move_actions.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)));  // 14484 생성 · 14490 tag=3 · 14492 push
       } } }

441: if !act_actions.is_empty() {                                              // 14437 len(+0x18)==0 → 1994(=456 경로로 직행)
442:   let moba = game.get_game_mode().moba().unwrap();                          // 14502 vtable+0x40 · 14516 tag!=0 → 17736 unwrap_failed(패닉)
       let serpen = moba.jungle_runner.serpen.live_list.get(0).and_then(|id| game.get_entity_by_id(*id));   // 14534 len==0 → None · 14550 vtable+0x1f0 · null → None. (.first() 와 표기 불가)
       if let Some(serpen) = serpen {                                            // None → 455
443:     let attacking_objective = act_actions.iter().any(|a| a.get_action().target() == Some(serpen.id));
           // 클로저$10 인라인: 태그(+0xb1)-15 < 4 (Attack/Skill/Skill2/Ult) && +0x8 target == serpen.id (14601~14618). 빈 슬라이스면 1994(456) 직행(컴파일러 단축)
444:     if attacking_objective {
445:       if let Some(action) = self.v27_objective_safe_attack_position(version, rnd, player, data, &parameter.positioning_score, serpen) {   // ▼ 전량 인라인(helper 43~145)
446:         if data.context.debug {                                          // 15995 ctx+0x3b
447:           debug.infos.entry(champ.id).or_insert(vec![]).push("v27 serpen safe attack position".to_string());   // 16019~16131
             }
449:         return bumpalo::vec![in ctx.pool; SmallActionPlay::AroundPosition(action)];   // 16007 memcpy 184B → 16008 from_iter_in(sret %0) → 함수 종료(배치 G 에필로그 %1956)
450:         (unwind 시 action drop 16266)
           }
         }
       }
455:   // if !act_actions.is_empty() → 1997 = 511(배치 I): act_actions 를 그대로 반환(16275 memcpy %0←%91) · 비면 456 으로
     }

// ───── helper v27_objective_safe_attack_position(serpen_hunt.rs:43~145) — 445 에 인라인. objective = serpen ─────
 43: let champ = data.cache.player_champion[team][pos]?;                         // 14678 재로드 · null → None(1917)
 44: let objective_hp_ratio = objective.hp(0x670)*100 / max(objective.stat_cached.hp(0x628), 1);   // 14686~14694
 45: if objective_hp_ratio < 36 { return None }                                  // 14696
 49: let attack_range = objective_attack_range(champ, objective);
       // = [16] champ.attack_effect.as_ref().unwrap()(0x4c0 tag==-1 → unwrap_failed 14737)
       //   [17] Effect::range(champ)(effect.rs:26 = 0x438 + 0x4a0 + (level(0x5c8)-1)*0x4a8, 14717~14723, 14783~14786) + range_adjust(atk, champ, objective)(14725) + champ.radius()(14729~14755) + objective.radius()(14760~14778)
 52: if distance_sq(champ, objective) > attack_range*attack_range { return None }   // 14816~14831
 56: let threats: bumpalo Vec<&Entity> = game.iter_champions(player_champion[1-team]).filter(|t|                     // 14854~14870 (m01.ll:34222 수집)
 58:      blackboard[player.team].is_recent_visible(game, player, t)             // aux 75824~75839
 59:      && (distance_sq(t, champ) <= 260000² || distance_sq(t, objective) <= 220000²)   // 75879 / 75911
     ).collect_in(ctx.pool);
 61: if threats.is_empty() { return None }                                      // 14879 → 1494 drop → 1917
 65: let hp_ratio = champ.hp*100 / max(champ.stat_cached.hp, 1);               // 15012~15020
 66-68: let current_nearest_enemy = threats.iter().map(|t| distance(champ.x, champ.y, t.x, t.y)).min().unwrap();   // 15071 첫 원소 + m12.ll:31322 fold(min_by Ord::cmp, 동률 시 앞 원소)
 70-72: let threat_contact = threats.iter().any(|t| {
 71:        let range = max(max_range_nearly_can_use(t, champ, 50), 100000) + (if hp_ratio < 50 { 70000 } else { 40000 });   // 15125/15132 · 15103~15104(루프 밖 호이스트) · 15157
 72:        distance_sq(champ, t) <= range*range });                            // 15140~15165 (초과면 다음 원소)
 74: if !threat_contact && !(hp_ratio < 45 && current_nearest_enemy <= 220000) { return None }   // 15169~15172 (`< 220001`)
 78: let keep_attack_range = attack_range.saturating_sub(30000);                 // 15175
 83-85: let allies: Vec<&Entity> = game.iter_champions(player_champion[team]).filter(|a| a.id != champ.id && distance_sq(a, objective) <= 220000²).collect_in(pool);   // 15186~15198 · aux 75929~75970 (m01.ll:34400)
 87-91: let ally_centroid: Option<(u64,u64)> = if allies.is_empty() { None } else { Some((Σa.x / n, Σa.y / n)) };   // 15207 · 15239~15264(x 합) · 15391~15416(y 합) · 15419~15420 udiv n
 93: drop(allies)
 98-99: let (xi, yi) = (positioning_score.cx as i32, positioning_score.cy as i32);   // 15428~15435 (7×7 탐색창 중심 셀)
100-101: let here_value = v27_positioning_value(version, player, &position_score_at_position(version, player, data, positioning_score, champ.x, champ.y, Objective));   // 15437 · 15445~15486
     // v27_positioning_value(34~38): positioning = player.info.parameter.positioning_effective();
     //   gain_weight = (100 + (50-positioning)/10).clamp(95,105); risk_weight = (100 + (positioning-50)/2).clamp(75,125);
     //   trajectory_penalty = if score.on_trajectory || score.on_periodic_trajectory { max(positioning-50, 0) } else { 0 };
     //   value = score.gain*gain_weight/100 - score.risk*risk_weight/100 - trajectory_penalty   (IR: sdiv 100 · sdiv -100)
102: let here_score = here_value + min(current_nearest_enemy, 320000)/10000 + 8;   // 15491 · 15519 · 15609(+8 은 현위치에만) · 15612~15613
103:   + cohesion(champ.x, champ.y)   // closure$6(95): ally_centroid.map_or(0, |(ax,ay)| 320000.saturating_sub(distance(x,y,ax,ay))/10000)  15501~15521 · 15614
105: let mut best: Option<(x, y, score, nearest_enemy)> = None;                 // %1732 tag
106: for dx in 0..7 {  let xb = xi - 3 + dx;                                    // 15570 · 15587 (xb>29 면 내부 루프 통째 skip 15588/15961)
107:   for dy in 0..7 {
109:     let yb = yi - 3 + dy;                                                   // 15664
110:     if xb < 0 || yb < 0 || yb > 29 || map.walls[yb][xb] != 0 { continue }   // 15666~15680 (MapDef+0x78 [30][30])
114-115: let (x, y) = (xb*32000+16000, yb*32000+16000);                         // 15684~15689
116:     if distance_sq((x,y), objective) > keep_attack_range² { continue }      // 15691~15699 (dx² 는 외부루프 15593~15597)
120-123: let nearest_enemy = threats.iter().map(|t| distance(x, y, t.x, t.y)).min().unwrap_or(u64::MAX);   // 15705~15774 · m12.ll:31218 fold
124-125: let cell_value = v27_positioning_value(version, player, &position_score_at_cell(version, player, data, positioning_score, xb, yb, Objective));   // 15777~15822
126:     let enemy_spacing = min(nearest_enemy, 320000)/10000;                   // 15827~15830
127:     let range_slack = min(keep_attack_range.saturating_sub(distance(x, y, objective.x, objective.y)), 80000)/10000;   // 15834~15842 · 15882~15884
128:     let score = cell_value + enemy_spacing + range_slack + cohesion(x, y);  // 15853~15890
130:     if best.map_or(true, |b| score > b.score) {                              // 15899~15902 (score <= best.score 면 유지 → 동률 시 먼저 찾은 셀)
131:        best = Some((x, y, score, nearest_enemy)) }                          // 15905~15916
     } }
136: let (bx, by, best_score, best_nearest) = best?;                             // 15574 tag false → None
137: if best_score < here_score && best_nearest < current_nearest_enemy.saturating_add(30000) { return None }   // 15615~15619
140: if distance_sq(champ, (bx,by)) <= 12000² { return None }                    // 15622~15636 (`< 144000001`)
144: Some(SmallActionAroundPosition::new_with_radius(rnd, data, bx, by, 3, 25000))   // 15640 · 15644~15648 (177B+tag+6B 분리 복사) · None 판정 = byte0xb1 == 0xff(15985)
145: drop(threats)                                                              // 15650
// ───── helper 끝 ─────

455: if act_actions.is_empty() {                                               // 14507 (비지 않으면 → 511 배치 I: act_actions 반환)
456:   let positioning_accuracy = player.info.parameter.positioning_accuracy();   // 16271
458:   let (min_v, max_v) = (positioning_accuracy, 2000 - positioning_accuracy);   // 16395
462:   let danger = game.iter_champions(player_champion[1-team]).any(|c| {      // 16440~16575 (슬롯 0..5 순, any 단락)
463:      let jrng = range_misjudge_rng(version, data, player, c.id);           // 16476
464:      let emr = max_range_can_use(c, champ) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000 + 10000;   // 16484 → 16488 → 16492~16494 (호출 순서 = rnd 미러 순서)
465:      let mr = max_range_can_use(champ, c);                                 // 16496
467:      blackboard[1-team].is_recent_visible(game, player, c)                // 16503 (★적팀 블랙보드)
468:      && distance_sq(c, champ) <= emr*emr                                   // 16511~16540
469:      && !c.is_in_action() && !c.block_input()                             // 16543~16553 (is_in_action: ty==13 && action_state.tag>=3)
471:      && mr == 0 });                                                        // 16557~16560 (IR: (mr!=0 || block_input) → 다음 원소; 469/471 의 && 결합 순서는 표기 불가)
472:   if danger { return bumpalo::vec![in pool; SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))] }   // 16691 · 17478~17482 → 함수 종료(%2368)
475:   let best_move_action = move_actions.iter().max_by_key(|a| self.score(version, parameter, rnd, player, data, a, debug)).unwrap();   // 16580~16685 (첫 원소는 serpen_action_score 16679 로 인라인 · fold m12.ll:22735, 동률 시 뒤 원소) · 빈 목록이면 unwrap_failed 16710(패닉)
479:   let incoming = game.iter_projectile().any(|p|                             // 16706 vtable+0x210 · 16728 next
480:        p.team != TeamType::Player(team) && !p.is_targeting()               // 16748~16763 (tag0==0 && +8==team → skip) · projectile.rs:134: Target/TargetSplash/BouncingTarget{target_id:Some} → skip (16767~16782)
481:        && distance_sq(champ, (p.x, p.y)) < 420000² );                       // 16789~16811 (+0x100/+0x108)
482-483: let enemy_rushing = game.iter_champions(player_champion[1-team]).any(|c| matches!(c.rush_state, RushState::Rush{..} | RushState::RushPenetrate{..}));   // 16832~16974 5슬롯 언롤 · +0x308 태그 sgt -1(암묵 RushPenetrate) || == 0x8000000000000003(Rush)
484:   if incoming || enemy_rushing {                                            // 2224/2225
485:     let input = best_move_action.clone().get_input(version, rnd, player, data, &parameter.positioning_score, debug);   // 16996 clone → 17007 get_input(sret %74 32B)
486:     drop(clone)                                                            // 17016 (unwind 17012)
         → 489(배치 I): switch input.tag(+0 i64: -1 / 0 / 기타)                 // 17018~17022 · 조건 거짓이면 2223(489, 배치 I)
       }
     }
// 이 범위에서 다른 배치로 넘어가는 지점: 449·472 → 함수 에필로그(배치 G 루트줄 1) · 455 비어있지 않음 → 511(배치 I) · 484 이후 → 489(배치 I) · 모든 unwind → %1326(513, 배치 I)

// serpen_hunt.rs:489~514 (배치 I)
// 진입 문맥(배치 H 소관, 계약만): L455 `if act_actions.is_empty()`(%91.len==0, m02.ll:14506~14510) 의 참 가지 안에서 L475 best_move_action = move_actions.iter().max_by_key(serpen_action_score).unwrap() (%2127, &SmallActionPlay) · L479~483 trajectory_possible: bool · L484~488 `let move_action_input: Option<Input> = if trajectory_possible { best_move_action.clone().get_input(version, rnd, player, data, &parameter.positioning_score, debug) } else { None };` (DI: !20720 line484 type Option<Input> · get_input 호출 m02.ll:17007 · 임시 clone %73 드롭 L486).
//   ⚠IR 상 trajectory_possible==false 경로(%2223, L482 any 루프 소진)는 get_input 을 호출하지 않고 곧장 %2226(L506 경로)으로 점프한다(m02.ll:16986 `br label %2226 ;L489`) = None 케이스와 같은 코드. 또한 L455 거짓 가지(act_actions 비어있지 않음)는 %1997 → L511.

// ── L489 (DILexicalBlock !20723 · 변수 move_action_input: Input 재바인딩 !20722)
// 줄 길이 산술(rmeta_srcmap: L489 = 60자 = 들여쓰기 8 + 52) 와 정합하는 형태: `if let Some(move_action_input) = move_action_input {`
tag = *(move_action_input as *i64)                          // %2232 = load %74 (m02.ll:17018)
switch tag {
  -1 (None)              => goto L506                        // %2226
  0  (Some(Input::Move)) => goto L490                        // %2233
  _  (Some(그 외 5종))    => goto L503                        // %2239
}

// ── L490 (DILexicalBlock !20725 · x,y !20724/!20726 line 490 · 줄 59자 = 10+49 ⇔ `if let Input::Move { x, y } = move_action_input {`)
// IR 은 x,y 로드에 L489 를 붙였다(let-chain 인라인 아티팩트). L490 자체 분기 명령 없음(태그 0 케이스 = 이미 Move 확정).
x = move_action_input.+0x8 ; y = move_action_input.+0x10     // %2237, %2235

// ── L491 한글 주석(28자 mb) · L492 `let on_trajectory = {` (DILexicalBlock !20728) · L493 `let position_score = position_score_at_position(` (!20730)
position_score: PositioningScore(56B) = position_score_at_position(version, player, data, &parameter.positioning_score /*+0x9f0*/, x, y, PositionEvalPurpose::Objective /*i8 11*/)   // m02.ll:17035
// ── L495 (83자 = 14+69 ⇔ `position_score.on_trajectory || position_score.on_periodic_trajectory`)
on_trajectory = position_score.on_trajectory /*+0x30*/ || position_score.on_periodic_trajectory /*+0x31*/   // 단락: +0x30 참이면 +0x31 미로드 (17045~17056)
// L496 `};` — position_score 수명 종료(lifetime.end !26785)

// ── L497 `if on_trajectory {`
if on_trajectory {
  // L498: return bumpalo::collections::Vec::from_iter_in([SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5 /*end_delay*/, false /*with_skill*/))], data.context.pool)
  ra: SmallActionRunAway(136B) = new_with_skill(data, player, 5, false)   // m02.ll:17072 sret %70
  elem(%71): memcpy 136B ← ra ; elem.+0xb1 = 3 (RunAway 태그)               // 17086~17089
  *sret = from_iter_in([elem], data.context.pool)                            // 17090 → cap1 len1
} else {
  // L500: return from_iter_in([best_move_action.clone()], data.context.pool)
  elem(%68) = Clone::clone(best_move_action /*%2127*/)                       // 17066 ; memcpy 184 → %69 ; 17078 from_iter_in
}
// L502 `} else {` — Some(Move) 가 아닌 Some(_)
// L503: return from_iter_in([best_move_action.clone()], data.context.pool)   // %66 clone 17041 · 17100 from_iter_in
// L505 `} else {` — None (및 trajectory_possible==false 의 스레딩 경로)
// L506: return from_iter_in([best_move_action.clone()], data.context.pool)   // %64 clone 17002 · 17228 from_iter_in
// ⟹ 500/503/506 은 바이트 단위로 동일한 결과(같은 %2127 clone · 같은 bump). 차이는 오직 498 RunAway 로 갈 수 있느냐.

// ── L508 `} else {` (L455 act_actions.is_empty() 의 거짓 가지) · L509~510 한글 주석 · L511 `act_actions`
// %1997 (m02.ll:16275): *sret = memcpy32(act_actions %91) — 이동이므로 %91 은 드롭하지 않음.

// ── L513 `    }` (L455 if 의 닫는 괄호 자리 · 지역 Vec 드롭 · 255 IR줄) — 반환 경로별 순서:
//   489~506 경로: %2258 drop(move_actions %87) → 인라인 dealloc(%87) → %2295 drop(act_actions %91) → dealloc → %2330 drop(act_actions %94) → dealloc → %2365 → L514
//   511 경로:     %1997 drop(%87) → dealloc → %2440 → %2330 drop(%94) → dealloc → L514        (%91 은 이동됨)
//   472 경로(배치 H, %2366→%2368): %1957 drop(%87) → %2369 drop(%91) → %2404 drop(%94) → %2439 → L514
//   각 drop = <Vec<SmallActionPlay> as Drop>::drop(아웃오브라인 · 원소 drop_in_place) + 인라인 RawVec/Bump::dealloc:
//     if cap != 0 { footer = *(buf.a + 0x10); if footer.ptr(+0x20) == buf.ptr { footer.ptr = buf.ptr + cap*184 } }   // 마지막 할당만 되돌림(bumpalo 는 하향 할당)
//   언와인드 정리: %1326 drop_glue(%87)→%333→(%334 phi 로 %91 드롭 여부 분기: 2015 경로만 false)→%2443 drop_glue(%91)→%287 drop_glue(%94)→caller.
// ── L514 `  }` ret void (%286).

// 사장 코드: reach.txt 「사장 호출부」 = L254 unwrap_failed(범위 밖). 이 범위(489~514) 에 NA 콜리 없음.
// 이 범위에서 push 되는 variant 집합: {RunAway(태그3 · new_with_skill(data, player, end_delay=5, with_skill=false)), best_move_action 의 variant(clone, 3사이트)}. self 쓰기 0 · debug 쓰기 0 · TLS 0.
```

**`mem` 메모리 접근 123건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | SerpenHuntSubPlan | 0x0 | need_recall | r | L204·L220 읽기(bool) | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | L203 · 2 미만 bounds check \| (배치 H) aux m02.ll:75825 (closure$0, blackboard[team] 인덱스) | 4 | OK |  |
| 2 | PlayerState | 0x9c0 | info.position(tag i32) | r | L203 player_champion 두번째 인덱스 · attack_jungle_action 인자 | 4 | OK |  |
| 3 | PlayerState | 0x180 | info.parameter (AthleteParameter) | r | L592 positioning_accuracy(&…) | 4 | OK |  |
| 4 | OperationData | 0x0 | cache | r | &AbstractGameWithCache \| (배치 H) %111 (배치 G 11275) — 배치 H 는 14844/16501 재로드 | 4 | OK |  |
| 5 | OperationData | 0x8 | context | r | &GameContext \| (배치 H) %262 (배치 G 11703) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 팻포인터 앞 8B | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x28 get_tick · +0x40 get_game_mode · +0x1f0 get_entity_by_id \| (배치 H) %285/14845/16502 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | [team][pos] Option<&Entity> (stride 40/8) | 4 | OK |  |
| 9 | AbstractGameWithCache | 0xd0 | jungles (bumpalo Vec<&Entity>: ptr@+0xd0, len@+0xe8) | r | F2 L326 | 4 | OK |  |
| 10 | AbstractGameWithCache | 0xf0 | others[2] (32B stride: ptr@+0, len@+0x18) | r | L638 · F2 L327 — others[1-team] | 4 | OK |  |
| 11 | GameContext | 0x0 | pool (&Bump) | r | Vec::new_in | 4 | OK |  |
| 12 | GameContext | 0x8 | setting (&GameSetting) | r | L612 | 4 | OK |  |
| 13 | GameContext | 0x20 | map (&MapDef) | r | L568·L649 camp_pos | 4 | OK |  |
| 14 | GameContext | 0x3b | debug (bool) | r | L550 | 4 | OK |  |
| 15 | GameSetting | 0x12f8 | tick_per_second | r | L612 me_die_tick < tps | 4 | OK |  |
| 16 | GameMode | 0x0 | tag (0=Moba) | r | get_game_mode 반환 {i64,ptr} | 4 | OK |  |
| 17 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r | L205·L239·L564 \| (배치 H) 14538 (L442) | 4 | OK |  |
| 18 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | 0 → None \| (배치 H) 14528 (L442) get(0) | 4 | OK |  |
| 19 | Entity | 0x0 | team (TeamType tag) | r | F1 L259 · F2 L318·is_visible_from | 4 | OK |  |
| 20 | Entity | 0x8 | team@Player.0 | r | 팀 id 비교 · visible_state 인덱스 | 4 | OK |  |
| 21 | Entity | 0x38 | visible_state[2] (24B each, tag@+0 · 0=Visible) | r | F2 is_visible_from | 4 | OK |  |
| 22 | Entity | 0x68 | ty (EntityType tag) | r | 13=Champion · 2=Tower | 4 | OK |  |
| 23 | Entity | 0x70 | ty@Champion.0.action_state tag | r | L524 4/5/6 = Skill/Skill2/Ult | 4 | OK |  |
| 24 | Entity | 0x438 | stat_buff_cached.range | r | max_range 헬퍼 \| (배치 H) 14723 | 4 | OK |  |
| 25 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | radius 보정 | 4 | OK |  |
| 26 | Entity | 0x490 | attack_effect (Option<Effect> 56B) | r | L206·L257·L566·L639 | 4 | OK |  |
| 27 | Entity | 0x4a0 | attack_effect.range | r | max_range | 4 | OK |  |
| 28 | Entity | 0x4a8 | attack_effect.growth_range | r | max_range | 4 | OK |  |
| 29 | Entity | 0x4c0 | attack_effect@tag (-1=None) | r | unwrap | 4 | OK |  |
| 30 | Entity | 0x4c8 | skill_effect | r | F1 L267 · F2 L320 · L524 | 4 | OK |  |
| 31 | Entity | 0x4d0 | skill_effect.ty.vtable | r | EffectType vtable(+0x40 heal · +0x48 shield · +0x50 buff · +0x60 rush · +0x68 move_on_hit) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 32 | Entity | 0x4f8 | skill_effect@tag / casting | r | -1=None · 1=Position · 2=Direction | 4 | OK |  |
| 33 | Entity | 0x500 | skill2_effect | r | level>2 게이트 | 4 | OK |  |
| 34 | Entity | 0x530 | skill2_effect@tag | r | IR 에선 (0x500 선택 ptr)+48 로 접근해 절대값 1328 은 본문에 없음(qcspec 경고 사유) | 4 | OK |  |
| 35 | Entity | 0x538 | ult_effect | r | level>4 게이트 | 4 | OK |  |
| 36 | Entity | 0x568 | ult_effect@tag | r | IR 에선 (0x538 선택 ptr)+48 로 접근해 절대값 1384 는 본문에 없음(qcspec 경고 사유) | 4 | OK |  |
| 37 | Entity | 0x5c0 | id | r | Trace target · debug key \| (배치 H) 14461/14588/16014/16474 등 (champ.id · serpen.id · 클로저 x.id) | 4 | OK |  |
| 38 | Entity | 0x5c8 | level | r | >2 / >4 \| (배치 H) 14721 (level-1)*growth_range | 4 | OK |  |
| 39 | Entity | 0x628 | stat_cached.hp (max hp) | r | hp_ratio 분모 | 4 | OK |  |
| 40 | Entity | 0x660 | x | r | 14792/14804/15067/15140/15253/16511 등 | 4 | OK |  |
| 41 | Entity | 0x668 | y | r | 14798/14810/15069/15144/15405/16515 등 | 4 | OK |  |
| 42 | Entity | 0x670 | hp | r | 14686(objective) / 15012(champ) | 4 | OK |  |
| 43 | Entity | 0x680 | radius | r | 14744/14751/14767/14774 | 4 | OK |  |
| 44 | Entity | 0x6a0 | block_target_tick | r | L233 타워 필터 | 4 | OK |  |
| 45 | Entity | 0x6b9 | can_target | r | L233 타워 필터 | 4 | OK |  |
| 46 | Strategy | 0xf | object_finish (0=KillPriority) | r | L237·L577 | 4 | OK |  |
| 47 | ObjectivePosture | 0x0 | focus_enemy@tag (-1 = Option<ObjectivePosture> None 니치) | r | L544·L556 | 4 | OK |  |
| 48 | ObjectivePosture | 0x8 | focus_enemy@Some.0 | r | L557 | 4 | OK |  |
| 49 | ObjectivePosture | 0x20 | wait_pos.0 | r | L549 | 4 | OK |  |
| 50 | ObjectivePosture | 0x28 | wait_pos.1 | r | L549 | 4 | OK |  |
| 51 | ObjectivePosture | 0x38 | near_enemy_count | r | L546 | 4 | OK |  |
| 52 | ObjectivePosture | 0x50 | kind (2=Screen 3=WaitGroup 4=SoftDisengage) | r | L545·L546·L556·L569 | 4 | OK |  |
| 53 | PositioningScore | 0x30 | on_trajectory | r | L537 \| (배치 H) 15470/15809 | 4 | OK |  |
| 54 | PositioningScore | 0x31 | on_periodic_trajectory | r | L537 \| (배치 H) 15473/15811 | 4 | OK |  |
| 55 | SmallActionPlay | 0xb1 | tag | r | F1·F2·L226·L423 \| (배치 H) 14601 (L443 get_action) · 15645/15991 (AroundPosition 변환 시 outline_type 바이트) · 14489/17480 store | 4 | OK |  |
| 56 | SmallActionPlay | 0x8 | Attack/Skill/Skill2/Ult payload .target (id) | r | F1·F2 | 4 | OK |  |
| 57 | ScoreParameter | 0x9f0 | positioning_score | r | L428 인자 \| (배치 H) %348(배치 G 11930) → position_score_at_* · get_input 인자 | 4 | OK |  |
| 58 | DebugFrameData | 0xa0 | infos (HashMap<usize, Vec<String>>) | r | L551 (읽고 씀) | 4 | OK |  |
| 59 | Option<&Entity> nearest_enemy_tower(로컬 %96, 배치 G 233 에서 기록) | 0x0 | ptr(null=None) | r | m02.ll:14421 (L431) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 60 | Entity | 0x68 | ty@tag | r | 14427: ==2(Tower) (L432) / 16543: !=13(Champion) (L469 is_in_action) | 4 | OK |  |
| 61 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag | r | 14442 (L433) Option<(usize,usize)> 태그 trunc→i1 | 4 | OK |  |
| 62 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 | r | 14456 (L433) champ.id 와 비교 → 타워의 현재 표적 id 로 읽힘 | 4 | OK |  |
| 63 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | 16546 (L469 Entity::is_in_action 인라인) <3 이면 비행동 | 4 | OK |  |
| 64 | Entity | 0x4c0 | attack_effect@tag(4B) | r | 14707 (helper 16 objective_attack_range) == -1 → None → unwrap_failed | 4 | OK |  |
| 65 | Entity | 0x4a0 | attack_effect@Some.0.range | r | 14717 (Effect::range effect.rs:26) | 4 | OK |  |
| 66 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | 14719 | 4 | OK |  |
| 67 | Entity | 0x470 | stat_buff_cached.radius_mult(i32) | r | 14729/14760 (Entity::radius entity.rs:1511) | 4 | OK |  |
| 68 | Entity | 0x628 | stat_cached.hp(최대 hp) | r | 14688(objective) / 15015(champ) — umax(…,1) 로 0 나눗셈 방지 | 4 | OK |  |
| 69 | Entity | 0x308 | rush_state@tag | r | 16841/16872/16903/16934/16965 (L483) 적 5슬롯 언롤 | 4 | OK |  |
| 70 | PlayerState | 0x180 | info.parameter(AthleteParameter) | r | 15444/16270 → positioning_effective / positioning_accuracy 의 &self | 4 | OK |  |
| 71 | OperationData | 0x10 | blackboard(&[Blackboard;2]) | r | 14847/16436 (L57·L462) | 4 | OK |  |
| 72 | AbstractGameWithCache | 0x0 | game(&dyn AbstractGame).data_ptr | r | %283/14844/16501 | 4 | OK |  |
| 73 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] / [1-team][0..5] | r | %114·%115(champ, 배치 G 11279) · 14678 재로드(helper 43) · %357/%358 = 적팀 5슬롯 슬라이스(15186·16442·16832) | 4 | OK |  |
| 74 | AbstractGame vtable | 0x40 | get_game_mode | r | 14500 (L442) → {i64 tag, ptr} | 4 | 확인불가(vtable 슬롯) |  |
| 75 | AbstractGame vtable | 0x1f0 | get_entity_by_id | r | 14546 (L442) (game, id) → ptr\|null | 4 | 확인불가(vtable 슬롯) |  |
| 76 | AbstractGame vtable | 0x210 | iter_projectile | r | 16704 (L479) → sret 40B ProjectileIter | 4 | 확인불가(vtable 슬롯) |  |
| 77 | GameMode | 0x0 | tag(0=Moba) | r | 14516 (L442) | 4 | OK |  |
| 78 | GameContext | 0x0 | pool(&Bump) | r | %326(배치 G 11871) · 14869 → from_iter_in bump 인자 | 4 | OK |  |
| 79 | GameContext | 0x20 | map(&MapDef) | r | 15535/15674 (helper 110) | 4 | OK |  |
| 80 | GameContext | 0x3b | debug(bool) | r | 15995 (L446) | 4 | OK |  |
| 81 | MapDef | 0x78 | walls[30][30](usize) | r | 15675~15678 walls[yb][xb] != 0 → 셀 제외 | 4 | OK |  |
| 82 | ScoreParameter | 0x14a8 | positioning_score.cx | r | 15428 (helper 98) xi | 4 | OK |  |
| 83 | ScoreParameter | 0x14b0 | positioning_score.cy | r | 15432 (helper 99) yi | 4 | OK |  |
| 84 | PositioningScore(sret 56B 로컬 %17/%13) | 0x0 | risk | r | 15484/15820 (v27_positioning_value 38) | 4 | OK |  |
| 85 | PositioningScore | 0x10 | gain | r | 15480/15817 | 4 | OK |  |
| 86 | SmallActionPlay | 0x8 | Attack/Skill/Skill2/Ult 페이로드 .target | r | 14610 (L443) 태그 15~18 일 때만 | 4 | OK |  |
| 87 | bumpalo Vec<SmallActionPlay>(로컬 act_actions %91 / move_actions %87) | 0x18 | len | r | 14435/14506/14563/16582 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 88 | bumpalo Vec<SmallActionPlay> | 0x0 | ptr | r | 14561/16580 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 89 | bumpalo Vec<&Entity>(threats %22 / allies %19) | 0x18 | len | r | 14877/15088/15205 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 90 | Projectile | 0x0 | team@tag | r | 16748 (L480) | 4 | OK |  |
| 91 | Projectile | 0x8 | team@Player.0 | r | 16761 (L480) == team | 4 | OK |  |
| 92 | Projectile | 0x40 | move_type@tag | r | 16767 (projectile.rs:134 is_targeting 인라인) | 4 | OK |  |
| 93 | Projectile | 0x100 | x | r | 16789 (L481) | 4 | OK |  |
| 94 | Projectile | 0x108 | y | r | 16793 (L481) | 4 | OK |  |
| 95 | DebugFrameData | 0xa0 | infos(HashMap<usize,Vec<String>>) | r | 16018 (L447) rustc_entry | 4 | OK |  |
| 96 | Option<Input> (지역 move_action_input=%74, get_input sret 32B) | 0x0 | tag(i64 · 니치) | r | switch: -1=None(tg_game_ai nstart=2^64-1) · 0=Some(Move) · 그 외=Some(Return1/Attack2/Skill3/Skill2 4/Ult5) — m02.ll:17018~17022 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 97 | Option<Input> (지역 %74) | 0x8 | Move.x (u64) | r | m02.ll:17028 — Some(Move) 케이스에서만 로드 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 98 | Option<Input> (지역 %74) | 0x10 | Move.y (u64) | r | m02.ll:17025 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 99 | ScoreParameter | 0x9f0 | positioning_score (PositioningScoreData 2760B) | r | %348 = gep %6, 2544 (L428, 배치 G 에서 계산) → position_score_at_position 4번째 인자로만 전달 | 4 | OK |  |
| 100 | OperationData | 0x8 | context (&GameContext) | r | %262 (L236/312, 배치 G 계산) → %326 = context.pool(+0x0) = &Bump — from_iter_in 3번째 인자 | 4 | OK |  |
| 101 | PositioningScore (지역 position_score=%72, sret 56B) | 0x30 | on_trajectory (bool) | r | m02.ll:17045~17048 — true 면 +0x31 은 읽지 않음(단락) | 4 | OK |  |
| 102 | PositioningScore (지역 %72) | 0x31 | on_periodic_trajectory (bool) | r | m02.ll:17051~17053 — +0x30 이 false 일 때만 | 4 | OK |  |
| 103 | Vec<SmallActionPlay> (지역 %87 move_actions / %91 act_actions / %94 act_actions) | 0x0 | buf.ptr | r | L513 인라인 Bump::dealloc — footer.ptr 와 비교(m02.ll:17134·17151) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 104 | Vec<SmallActionPlay> (지역 3개) | 0x8 | buf.a (&Bump) | r | m02.ll:17132~17133 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 105 | Vec<SmallActionPlay> (지역 3개) | 0x10 | buf.cap | r | m02.ll:17124~17126 — 0 이면 dealloc 생략 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 106 | bumpalo::Bump | 0x10 | current_chunk_footer (Cell<NonNull<ChunkFooter>>) | r | m02.ll:17135~17136 | 4 | OK |  |
| 107 | bumpalo::ChunkFooter (distruct 폴백 · tcx 에 없음) | 0x20 | ptr (Cell<NonNull<u8>>) | r | m02.ll:17146~17147 — 마지막 할당이면 되돌림 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 108 | SerpenHuntSubPlan | 0x0 | need_recall | w | &mut self 의 유일한 쓰기 표면. m02.ll:11377~11378 phi+store. 두 조건 모두 아니면 값 유지 | 4 | OK | 1 (L207: champ.hp <= 3*dmg) / 0 (L213: hp_ratio > 29) |
| 109 | (sret Vec) | 0x0 | ptr/bump/cap/len | w | bumpalo Vec 원소 184B, 태그 +0xb1 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | L221~223 Recall 1개 · L227 from_iter_in 1개 · L428 get_move_action 결과(%87, 배치 H 로 전달) |
| 110 | DebugFrameData | 0xa0 | infos[champ.id].push(String) | w | data.context.debug 참일 때만(L550~551). rustc_entry/or_insert/grow_one/push_mut | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | format!(@anon.126 … {:?} posture.kind) |
| 111 | rnd | - | StdRng 상태 | w | 직접 쓰기 없음 | 4 | 확인불가(오프셋 파싱 실패) | 콜리 경유(L656 gen_range 12000..=inner 등) |
| 112 | (sret) Vec<SmallActionPlay> | 0x0 | ptr·bump·cap·len (32B 전부) | w | m02.ll:16008 (L449) from_iter_in(%78=[action;1], %326=pool). 이후 함수 종료(→ 블록 %1956 = 배치 G 에필로그, 루트줄 1) \| (배치 H) 17482 (L472) 태그 3 store 17481. 이후 함수 종료(→ %2368 배치 G 에필로그) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | vec![in pool; SmallActionPlay::AroundPosition(v27 결과)] (len 1) |
| 113 | DebugFrameData(debug %8) | 0xa0 | infos[champ.id].push(String) | w | 16019 rustc_entry(champ.id) → 16058 or_insert(Vec::new) → 16086 try_allocate_in(31) → 16123 memcpy → 16131 push_mut. data.context.debug(+0x3b)==true 일 때만(15998) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | "v27 serpen safe attack position"(31B, @anon…129) |
| 114 | 로컬 act_actions(%91) | 0x18 | len | w | 14473 (L434) — 타워가 나를 표적일 때. 로컬이지만 511(배치 I)에서 그대로 반환값이 되므로 사실상 sret 내용 | 4 | 확인불가(tcx 사전에 타입 없음) | 0 (truncate(0)) |
| 115 | 로컬 move_actions(%87) | 0x0 | truncate(0) 후 push(RunAway) | w | 14478 truncate (L435) · 14484 생성 · 14490 태그 store · 14492 push (L436) | 4 | 확인불가(tcx 사전에 타입 없음) | SmallActionPlay::RunAway(new_with_skill(data, player, end_delay=5, with_skill=true)) 태그 3 |
| 116 | rnd(&mut StdRng %3) | 0x0 | (콜리 경유 소비) | w | 배치 H 본체 직접 store 0. 호출 순서 = 462 루프(적 슬롯순, any 단락) → 445 helper 144 → 475 fold → 485 | 4 | OK | range_misjudge_roll(16488) · new_with_radius(15640) · serpen_action_score/score(16679, m12.ll:22829) · get_input(17007) |
| 117 | (sret) Vec<SmallActionPlay> | 0x0 | buf.ptr/buf.a/buf.cap/len 32B 전부 | w | 498 (m02.ll:17090) · 500 (17078) · 503 (17100) · 506 (17228) — 아웃오브라인 from_iter_in 이 %0 에 직접 sret | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | from_iter_in([1개], data.context.pool) → {bump alloc 184B, %326, 1, 1} |
| 118 | (sret) Vec<SmallActionPlay> | 0x0 | 32B memcpy | w | 511 (m02.ll:16275) — act_actions.is_empty()==false 경로(L455 분기, 배치 H) 의 반환. 이후 %91 은 드롭하지 않음(이동됨) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | act_actions(%91) 그대로 |
| 119 | SmallActionPlay (지역 %71, 498 경로) | 0xb1 | tag | w | m02.ll:17088~17089 — new_with_skill sret 136B 를 %71 에 memcpy 한 뒤 태그만 기록 | 4 | OK | 3 (=RunAway) |
| 120 | bumpalo::ChunkFooter | 0x20 | ptr | w | L513 지역 Vec 드롭의 인라인 Bump::dealloc — `footer.ptr == buf.ptr`(내 버퍼가 마지막 할당)일 때만 되돌림(m02.ll:17155~17165 등 %87·%91·%94 각 1회 + 언와인드 경로). 원소 stride 184 는 `mul i64 %cap, 184` | 4 | 확인불가(tcx 사전에 타입 없음) | buf.ptr + cap*184 |
| 121 | heap(Bump) | - | 184B 할당 | w | footer.ptr 를 184B(정렬 8) 내림 — 아웃오브라인이라 이 범위 IR 에 직접 store 없음 | 4 | 확인불가(오프셋 파싱 실패) | from_iter_in 내부(reserve_internal_or_panic, m02.ll:49585~) |
| 122 | SerpenHuntSubPlan (&mut self %1) | - | 없음 | w | 이 범위 489~514 에서 self 쓰기 0건 · debug(%8) 쓰기 0건 | 4 | 확인불가(오프셋 파싱 실패) | - |

**`consts` 상수 80건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 203 | 길이 | player_champion 1차 배열 길이(team < 2 bounds check) · L545 kind > 2 · L569 kind-3 <u 2 · L524 casting Direction · F1 Tower ty | 4 |  |
| 1 | 3 | 207 | 임계 | need_recall 판정: champ.hp <= dmg*3 (세르펜 평타 3방, `mul i64 %156, 3`). 본문의 `shl …, 3` 은 8B 포인터 stride 라 이 상수와 무관 | 4 |  |
| 2 | 100 | 212 | 계수 | hp_ratio = hp*100/max_hp (L212·L328·radius mult 보정) | 4 |  |
| 3 | 29 | 213 | 임계 | hp_ratio > 29 이면 need_recall 해제 (=30% 이상) | 4 |  |
| 4 | 5 | 222 | 태그 | end_delay=5 (Recall·RunAway·AroundPosition·Trace 전부) · JungleType::Serpen 태그(v27/v25/camp_pos 의 i8 5) · ChampionActionState::Skill2 | 4 |  |
| 5 | 4 | 222 | 태그 | SmallActionPlay::Recall 메모리 태그(+0xb1) · ObjectivePostureKind::SoftDisengage · ChampionActionState::Skill · level > 4 (Ult 해금) | 4 |  |
| 6 | -1 | 226 | 센티널 | Option 니치 None(태그 i8/i32 -1): v27 반환 tag@+0xb1 · attack/skill effect tag · posture tag · expected_buff tag | 4 |  |
| 7 | 0 | 233 | 임계 | block_target_tick == 0 · GameMode::Moba · ObjectFinishStrategy::KillPriority · VisibleState::Visible · radius_mult==0 · Vec::new_in 초기값 | 4 |  |
| 8 | 13 | 252 | 태그 | EntityType::Champion 메모리 태그 (F1 L252 · L523) | 4 |  |
| 9 | 15 | 245 | 태그 | SmallActionPlay::Attack 태그; F1 은 `tag-15 <u 4` 로 Attack/Skill/Skill2/Ult(15~18) 판별 | 4 |  |
| 10 | 7 | 253 | 센티널 | 니치 디코드: tag>2 ? tag-3 : 7 — 7=AroundPosition(untagged) 논리 인덱스 | 4 |  |
| 11 | 12 | 253 | 태그 | F1 switch 논리 인덱스 12=Attack(태그15) · 13=Skill · 14=Skill2 · 15=Ult (=tag-3) | 4 |  |
| 12 | 15000 | 270 | 계수 | is_in_range_ex 여유(적 타워 사거리+15000 안이면 스킬 후보 제거) · Trace attack_range_margin 기본값 · L656 dist+15000 < range | 4 |  |
| 13 | 14400000001 | 324 | 미상 | 120000²+1 — dist² < 이 값 ⟺ 거리 <= 120000(3.75셀): 대상 근처 가시 적 판정(F2) | 4 |  |
| 14 | 79 | 329 | 미상 | hp_ratio > 79 (=80% 이상) 이면 힐 후보는 aoe_heal_covers_low_ally 게이트를 추가로 통과해야 유지 | 4 |  |
| 15 | -30 | 425 | 미상 | self.score(...) < -30 인 공격 후보 제거 | 4 |  |
| 16 | 11 | 535 | 센티널 | PositionEvalPurpose::Objective 메모리 태그(니치) — position_score_at_position purpose 인자 | 4 |  |
| 17 | 6 | 528 | 태그 | ChampionActionState::Ult 태그 · iter_towers 배열 길이 6 | 4 |  |
| 18 | 1 | 524 | 임계 | CastingType::Position(1)/Direction(2) 만 논타겟으로 봄 · 1-team(적 팀) · TeamType::Neutral | 4 |  |
| 19 | 14 | 558 | 태그 | SmallActionPlay::Trace 메모리 태그(+0xb1) | 4 |  |
| 20 | 3 | 539 | 태그 | SmallActionPlay::RunAway 메모리 태그(+0xb1, `store i8 3`) · ObjectivePostureKind::WaitGroup. `shl …, 3` 은 stride 라 무관 | 4 |  |
| 21 | 19 | 659 | 태그 | SmallActionPlay::Stop 메모리 태그(+0xb1) | 4 |  |
| 22 | 25000 | 601 | 미상 | can_enemy_hit_objective(enemy, serpen, 25000) 여유 | 4 |  |
| 23 | 2000 | 594 | 계수 | max_v = 2000 - positioning_accuracy (range_misjudge_roll 상한) | 4 |  |
| 24 | 1000 | 606 | 계수 | misjudge roll 은 천분율: 사거리 * roll / 1000 | 4 |  |
| 25 | 40 | 608 | 태그 | max_range_nearly_can_use(enemy, champ, 40) — emr_near | 4 |  |
| 26 | 60 | 625 | 미상 | max_range_nearly_can_use(enemy, champ, 60) — me_die_tick < tps 일 때 emr | 4 |  |
| 27 | 10 | 613 | 임계 | enemy.remain_action_time() > 10 틱 | 4 |  |
| 28 | 999 | 613 | 임계 | (max_range_can_use*roll) > 999 ⟺ mr(/1000) >= 1 — 소스는 `mr > 0` 추정(표기 불가) | 5 |  |
| 29 | 10000 | 656 | 계수 | inner = range.saturating_sub(10000) | 4 |  |
| 30 | 12000 | 656 | 임계 | inner 하한 · 여유 최소값 · gen_range(12000..=inner) 시작 | 4 |  |
| 31 | 45000 | 656 | 임계 | inner 상한(umin) | 4 |  |
| 32 | 2 | 432 | 태그 | EntityType::Tower 메모리태그(tcxdict --enum EntityType) — nearest_enemy_tower.ty == Tower (14429). 같은 값 2 가 helper 36 의 (positioning-50)/2 제수(15461/15800)에도 쓰임 | 3 |  |
| 33 | 5 | 436 | 태그 | SmallActionRunAway::new_with_skill 의 end_delay 인자(14484 · 17482 두 지점 모두 5) | 4 |  |
| 34 | 3 | 436 | 태그 | SmallActionPlay::RunAway 메모리태그(store i8 3 @+0xb1 · 14490/17481). 같은 값 3 이 SmallActionAroundPosition::new_with_radius 의 end_delay(15640) · 탐색창 반폭 xi-3/yi-3(15533/15534) · ChampionActionState 태그 <3(=Idle/Return/Move, 16548)에도 쓰임. ⚠본문의 `shl i64 %n, 3`(14929/15034 등)은 &Entity 슬라이스 stride ×8 포인터 산술이지 이 상수의 접힘이 아니다(shl 경고 사유) | 4 |  |
| 35 | 15 | 443 | 태그 | SmallActionPlay 태그-15 < 4 ⟹ 태그 15~18 = Attack/Skill/Skill2/Ult(대상 있는 행동)만 get_action().target 을 본다(14605) | 4 |  |
| 36 | 4 | 443 | 태그 | 위 태그 범위 폭(4종). 또 is_targeting switch 의 논리 idx 4 = ProjectileMoveType::Target(16775) | 4 |  |
| 37 | 36 | 445 | 임계 | helper 45: objective_hp_ratio(세르펜 hp%) < 36 이면 v27 위치 조정 안 함(None) (14696) | 4 |  |
| 38 | 100 | 445 | 계수 | 백분율 스케일(hp*100/max_hp 14693/15014 · radius*(mult+100)/100 14753/14776 · gain*w/100 15483 · risk*w/(-100) 15486) | 4 |  |
| 39 | 1 | 445 | 태그 | umax(max_hp,1) 0-나눗셈 가드(14692/15019) · (level-1)*growth_range(14783) · Option::Some 태그(15940 best) · BouncingTarget.target_id Some 태그(16781) | 4 |  |
| 40 | -1 | 445 | 센티널 | Option<Effect> None 니치(attack_effect@tag i32 == -1 → unwrap_failed, 14709) · Option<SmallActionAroundPosition> None(byte 0xb1 == 0xff, 15985) · u64::MAX(빈 threats 의 nearest 기본값 15774) · RushState 태그 sgt -1(16847) | 4 |  |
| 41 | 50 | 445 | 임계 | helper 71: max_range_nearly_can_use(t, champ, tick=50) 인자(15125) · helper 71: hp_ratio<50 이면 접촉 여유 70000 아니면 40000(15103) · helper 35/36: 50-positioning / positioning-50 기준점(15450/15460) | 4 |  |
| 42 | 100000 | 445 | 인덱스 | helper 71: 위협 접촉 사거리 바닥 = max(max_range_nearly_can_use, 100000) (15132) | 4 |  |
| 43 | 70000 | 445 | 산출값 | helper 71: 내 hp_ratio < 50 일 때 접촉 판정에 더하는 여유 거리(15104) | 4 |  |
| 44 | 40000 | 445 | 산출값 | helper 71: 내 hp_ratio >= 50 일 때 접촉 여유 거리(15104) | 4 |  |
| 45 | 45 | 445 | 임계 | helper 74: 접촉 없어도 hp_ratio < 45 && 최근접 적 ≤ 220000 이면 위치 조정 계속(15169) | 4 |  |
| 46 | 220001 | 445 | 임계 | helper 74: current_nearest_enemy < 220001 (= ≤ 220000, 셀 6.9칸) (15170) | 4 |  |
| 47 | 30000 | 445 | 계수 | helper 78: keep_attack_range = attack_range.saturating_sub(30000) (15175) / helper 137: best.nearest_enemy < current_nearest.saturating_add(30000) 이면 이득 없음(15616) | 4 |  |
| 48 | 67600000001 | 445 | 미상 | = 260000²+1: closure$0(helper 59) dist_sq(t, champ) < 이 값 ⟺ ≤ 260000 → 위협 (aux m02.ll:75879) | 4 |  |
| 49 | 48400000001 | 445 | 미상 | = 220000²+1: closure$0(helper 59) dist_sq(t, objective) ≤ 220000 → 위협 (75911) / closure s1_0(helper 85) dist_sq(ally, objective) ≤ 220000 → 아군 집계(75970) | 4 |  |
| 50 | 11 | 445 | 태그 | PositionEvalPurpose::Objective 메모리태그(tcxdict --enum: idx 9 → 태그 11) — position_score_at_position(15437)/at_cell(15777) 의 purpose 인자 | 3 |  |
| 51 | 10 | 445 | 임계 | helper 35: gain_weight = 100 + (50-positioning)/10 (15451/15790) | 4 |  |
| 52 | 95 | 445 | 산출값 | helper 35: gain_weight clamp 하한(15458) — IR 은 (50-positioning) < -59 로 접힘 | 4 |  |
| 53 | 105 | 445 | 임계 | helper 35: gain_weight clamp 상한 umin(15457) | 4 |  |
| 54 | 75 | 445 | 산출값 | helper 36: risk_weight = 100 + (positioning-50)/2 의 clamp 하한(15468) — IR 은 (positioning-50) < -51 로 접힘 | 4 |  |
| 55 | 125 | 445 | 임계 | helper 36: risk_weight clamp 상한(15467) | 4 |  |
| 56 | -59 | 445 | 임계 | clamp(95,105) 하한 판정이 `(50-positioning) < -59` 로 접힘(15456) — 소스 값은 95 | 4 | 95 |
| 57 | -51 | 445 | 임계 | clamp(75,125) 하한 판정이 `(positioning-50) < -51` 로 접힘(15466) — 소스 값은 75 | 4 | 75 |
| 58 | -100 | 445 | 계수 | helper 38: `- risk*risk_weight/100` 이 sdiv -100 으로 접힘(15486/15822) | 4 | 100 |
| 59 | 320000 | 445 | 인덱스 | helper 102/126: enemy_spacing = min(nearest_enemy, 320000)/10000 (15491/15827) · closure$6(95): cohesion = 320000.saturating_sub(dist(pos, ally_centroid))/10000 (15518/15874) | 4 |  |
| 60 | 10000 | 445 | 계수 | helper 95/102/126/127: 거리→점수 스케일 제수(15520/15527/15829/15883) · 462 클로저 464: emr = …/1000 + 10000 (16494) | 4 |  |
| 61 | 8 | 445 | 미상 | 현위치 점수(here_score)에만 더해지는 상수(15609 `%1719+8`, dbg 는 helper 38 로 찍히나 후보 셀 점수 124~128 에는 없음 → 현위치 유지 보정(히스테리시스). 정확한 소스 줄은 재결합으로 표기 불가(38 또는 102) | 4 |  |
| 62 | 7 | 445 | 임계 | helper 106/107: dx,dy in 0..7 — 7×7 탐색창(15570/15960/15980 `<7`/`>6`) | 4 |  |
| 63 | 29 | 445 | 임계 | helper 110: 셀 좌표 상한(xb>29 \|\| yb>29 제외, 15588/15668) = 30×30 맵 | 4 |  |
| 64 | 32000 | 445 | 미상 | 셀 크기(좌표 변환: x = xb*32000+16000, 15590/15686) | 4 |  |
| 65 | 16000 | 445 | 미상 | 셀 중심 오프셋(15591/15687) | 4 |  |
| 66 | 144000001 | 445 | 임계 | = 12000²+1: helper 140: dist_sq(champ, best) < 이 값(≤12000) 이면 이미 그 자리 → None(15635) | 4 |  |
| 67 | 25000 | 445 | 미상 | helper 144: SmallActionAroundPosition::new_with_radius 의 around_radius(15640) | 4 |  |
| 68 | 80000 | 445 | 임계 | helper 127: range_slack = min(keep_attack_range.saturating_sub(dist(cell, objective)), 80000)/10000 상한(15842) | 4 |  |
| 69 | 2000 | 458 | 계수 | max_v = 2000 - positioning_accuracy (range_misjudge_roll 상한, 16395) | 4 |  |
| 70 | 1000 | 462 | 계수 | 클로저 464: emr = max_range_can_use(c, champ) * roll / 1000 + 10000 (16493) — roll 이 천분율 | 4 |  |
| 71 | 13 | 462 | 태그 | EntityType::Champion 메모리태그 — Entity::is_in_action 인라인(16545): 챔피언이 아니거나 action_state 태그 < 3 이면 비행동 | 4 |  |
| 72 | 176400000000 | 479 | 임계 | = 420000²: closure$13(481) dist_sq(champ, projectile) < 420000² 이면 비표적 적 투사체 근접(16810) | 4 |  |
| 73 | 9 | 479 | 센티널 | ProjectileMoveType 니치 구멍(assume tag != 9, 16769) — 컴파일러 아티팩트. is_targeting 의 idx 환산 `tag>1 ? tag-2 : 7`(16771~16773) | 4 |  |
| 74 | -9223372036854775805 | 482 | 센티널 | = 0x8000000000000003 = RushState::Rush 메모리태그(tcxdict --enum RushState: niche_start 2^63, idx 3). closure$14(483): 태그 sgt -1(= 니치 밖 = 암묵 RushPenetrate) \|\| == Rush 태그 → 적이 돌진 중(16847~16849) | 3 |  |
| 75 | -1 | 489 | 센티널 | Option<Input>::None 의 니치 태그(2^64-1 · _tcx tg_game_ai.jsonl:1125 nstart=18446744073709551615). switch case → 506 경로 | 3 |  |
| 76 | 0 | 489 | 태그 | Input::Move 의 메모리 태그(tcxdict --enum Input idx0=tag0). switch case → 490 분기 진입(x,y 로드) | 3 |  |
| 77 | 11 | 493 | 센티널 | PositionEvalPurpose::Objective 의 메모리 태그(니치 niche_start=2 · idx9 → 11). position_score_at_position 7번째 인자 i8 | 4 |  |
| 78 | 5 | 498 | 태그 | SmallActionRunAway::new_with_skill 3번째 인자 end_delay(usize · m08.ll DILocalVariable !53154). 궤적 회피 도주의 종료 지연 | 4 |  |
| 79 | 3 | 498 | 센티널 | SmallActionPlay::RunAway 의 메모리 태그(니치 niche_start=3 · idx0 → 3) — store i8 3 @+0xb1 (m02.ll:17089). 본체 다른 곳의 `shl … 3`(배치 G/H 범위 stride 접힘)과는 무관 — 시프트량 아님 | 4 |  |

**`knobs` 조정점 30건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 귀환 진입 임계(세르펜 평타 몇 방에 죽나) | serpen_hunt.rs:207 | 3 | 올리면(예 4) 더 높은 HP 에서도 need_recall=true 가 되어 사냥을 포기하고 귀환한다; 내리면 더 낮은 HP 까지 버틴다 | 4 | 기존 |
| 1 | 귀환 해제 HP% | serpen_hunt.rs:213 | 29 | hp_ratio 가 이 값을 넘으면 need_recall 해제. 올리면 귀환 상태가 더 오래 유지된다 | 4 | 기존 |
| 2 | 아군 대상 스킬 근처 적 판정 거리 | serpen_hunt.rs:324~327 | 14400000001 | 120000² +1. 올리면 더 먼 적도 '근처'로 쳐서 실드/힐 후보가 더 자주 유지된다 | 4 | 기존 |
| 3 | 힐 후보 대상 HP% 하드 게이트 | serpen_hunt.rs:329 | 79 | 대상 HP 가 이보다 높으면 aoe_heal_covers_low_ally 가 참이어야 유지. 내리면 힐 후보가 더 빨리 걸러진다 | 4 | 기존 |
| 4 | 공격 후보 score 하한 | serpen_hunt.rs:425 | -30 | 내리면(-50) 점수 낮은 공격 후보도 남는다; 올리면(0) 더 공격적으로 걸러진다 | 4 | 기존 |
| 5 | 적 타워 사거리 여유(스킬 후보 제거) | serpen_hunt.rs:270·285·300 | 15000 | 올리면 타워에서 더 먼 대상에 대한 스킬도 버린다 | 4 | 기존 |
| 6 | 세르펜 안전 공격 여유 범위 | serpen_hunt.rs:656 헬퍼 L22 | 12000..=45000, range-10000 | Trace 의 attack_range_margin. 하한을 올리면 세르펜에서 더 떨어져 공격 위치를 잡는다 | 4 | 기존 |
| 7 | 적 챔피언 도주 판정 사거리 계수 | serpen_hunt.rs:608·625 | 40 / 60 | max_range_nearly_can_use 의 여유 인자. 올리면 적 사거리를 더 넓게 봐 runaway 가 더 자주 켜진다 | 4 | 기존 |
| 8 | 적 근접 판정 tps 게이트 | serpen_hunt.rs:612 | me_die_tick < tick_per_second | 1초 안에 죽는 상황이면 60 계수·강제도주 경로, 아니면 40 계수·remain_action_time>10 경로 | 4 | 기존 |
| 9 | 세르펜 타격 가능 적 여유 | serpen_hunt.rs:601 | 25000 | KillPriority 전략일 때 세르펜을 이 여유 안에서 때릴 수 있는 적만 추적/도주 계산 대상 | 4 | 기존 |
| 10 | 타워 표적 시 도주 end_delay / with_skill | serpen_hunt.rs:436 | 5, true | 타워가 나를 노릴 때 기존 후보 전부 버리고 스킬 허용 도주 1개로 대체. end_delay 를 올리면 도주 유지 틱 증가(new_with_skill 내부 해석은 미독) | 4 | 기존 |
| 11 | v27 위치 조정 발동 세르펜 hp% 하한 | serpen_hunt.rs:45 (445 인라인) | 36 | 올리면 세르펜 체력이 더 많을 때만 안전 위치로 이동(막타 구간엔 그 자리 유지) | 4 | 기존 |
| 12 | 위협 수집 반경(대 챔프 / 대 세르펜) | serpen_hunt.rs:59 (closure$0) | 260000 / 220000 | 올리면 더 먼 적도 위협으로 세어 위치 조정이 자주 발동 | 4 | 기존 |
| 13 | 접촉 판정 사거리 바닥 + hp 별 여유 | serpen_hunt.rs:71 | max(nearly_range,100000) + (hp<50 ? 70000 : 40000) | 여유를 키우면 적이 멀어도 접촉으로 판정 → 재배치 빈도↑ | 4 | 기존 |
| 14 | 비접촉 저체력 발동 조건 | serpen_hunt.rs:74 | hp_ratio < 45 && nearest ≤ 220000 | 낮추면 저체력 재배치가 드물어짐 | 4 | 기존 |
| 15 | 공격 사거리 유지 마진 | serpen_hunt.rs:78 | 30000 | keep_attack_range = attack_range - 30000. 키우면 세르펜에 더 붙은 셀만 후보 | 4 | 기존 |
| 16 | 포지셔닝 가중치 clamp | serpen_hunt.rs:35~36 | gain 95~105 · risk 75~125 | positioning 스탯이 gain/risk 항에 미치는 폭. 좁히면 스탯 무관 | 4 | 기존 |
| 17 | 현위치 히스테리시스 | serpen_hunt.rs:102(±, 재결합) | 8 | 현위치 점수에만 +8 → 후보 셀이 8 이상 좋아야 이동(137 과 결합). 올리면 제자리 고수 | 4 | 기존 |
| 18 | 적 간격 / 응집 / 사거리 여유 점수 스케일 | serpen_hunt.rs:95·102·126·127 | min(·,320000)/10000 · 80000 상한 | 제수를 줄이면 거리 항 비중↑(포지셔닝 값 대비) | 4 | 기존 |
| 19 | 이동 이득 판정: 적 간격 개선 기준 | serpen_hunt.rs:137 | 30000 | 점수가 낮아도 최근접 적과 30000 이상 벌어지면 이동 채택. 키우면 더 큰 이격만 인정 | 4 | 기존 |
| 20 | 이미 도착 판정 거리 | serpen_hunt.rs:140 | 12000 | best 셀이 12000 이내면 None(현위치 유지). 키우면 미세 이동 억제 | 4 | 기존 |
| 21 | AroundPosition 파라미터 | serpen_hunt.rs:144 | end_delay 3 · around_radius 25000 | around_radius 를 줄이면 목표점에 더 정확히 정지 | 4 | 기존 |
| 22 | 사거리 오판 롤 범위 | serpen_hunt.rs:458 | min_v = accuracy, max_v = 2000 - accuracy | 정확도가 높을수록 롤 구간이 좁아져 emr 이 실제 사거리에 수렴 | 4 | 기존 |
| 23 | 위협 도주 판정 기본 여유 | serpen_hunt.rs:464 | +10000 (emr 에 가산) | 올리면 적 사거리 밖에서도 도주 발동 | 4 | 기존 |
| 24 | 위협 도주 end_delay / with_skill | serpen_hunt.rs:472 | 5, false | 436 과 달리 스킬 도주 미허용 | 4 | 기존 |
| 25 | 투사체 경계 반경 | serpen_hunt.rs:481 | 420000 | 비표적 적 투사체가 이 거리 안이면 best 이동 후보의 get_input 을 미리 산출(489 분기). 줄이면 덜 민감 | 4 | 기존 |
| 26 | 궤적 회피 도주의 end_delay | serpen_hunt.rs:498 (new_with_skill 3번째 인자) | 5 | 올리면 투사체 궤적 위에 있을 때 만든 RunAway 가 더 오래 유지(종료 지연 ↑) — 의미는 SmallActionRunAway 명세(r14) 계약에 따름. 내리면 더 빨리 도주 종료 | 4 | 기존 |
| 27 | 궤적 회피 도주의 with_skill | serpen_hunt.rs:498 (new_with_skill 4번째 인자) | False | true 로 바꾸면 도주 중 이동기 사용 허용(SmallActionRunAway.with_skill @0x80). 현재는 스킬 없는 도주 | 4 | 기존 |
| 28 | 위치 평가 목적 | serpen_hunt.rs:493~494 (PositionEvalPurpose::Objective, 태그 11) | 11 | 다른 purpose 로 바꾸면 position_score_at_position 의 on_trajectory 산출 문맥이 달라짐(내부는 그 함수 명세 소관) | 4 | 기존 |
| 29 | 궤적 판정 OR 결합 | serpen_hunt.rs:495 | on_trajectory \|\| on_periodic_trajectory | 둘 중 하나만 참이어도 RunAway. `&&` 로 바꾸면 주기 궤적만 있는 경우 최적 이동을 유지 | 4 | 기존 |

<details><summary>`callees` 피호출자 98건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | aoe_heal_covers_low_ally | game_ai::aoe_heal_covers_low_ally | pub | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\buff_value.rs:543 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | attack_jungle_action | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::attack_jungle_action | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:147 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | block_input | game_core::Entity::block_input | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1501 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_enemy_hit_objective | game_ai::plan_legacy::old::can_enemy_hit_objective | pub | fn(&game_core::Entity, &game_core::Entity, u64) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1188 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 18 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 20 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 21 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 22 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 23 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 24 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 25 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 26 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 27 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 28 | get_input | game_ai::SmallActionPlay::get_input | pub | fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action.rs:157 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | get_move_action | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:517 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 30 | get_move_action | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::epic_poke | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:278 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 31 | get_move_action | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:516 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 32 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 33 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 34 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 35 | is_in_action | game_core::Entity::is_in_action | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1547 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 36 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | is_in_range_ex | game_core::Effect::is_in_range_ex | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity, u64, u64, u64, u64, u64) -> bool | game-core\src\simulation\effect.rs:78 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | is_targeting | game_core::ProjectileMoveType::is_targeting | pub | fn(&game_core::ProjectileMoveType) -> bool | game-core\src\simulation\projectile.rs:133 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 40 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 41 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 42 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 43 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 44 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 45 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 46 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 47 | keep | game_view::pixel_fx::Dither::keep | in:game_view::pixel_fx | fn(game_view::pixel_fx::Dither, i32, i32) -> bool | game-view\src\view\pixel_fx.rs:49 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 48 | keep | game_view::view::effect::alchemist::Dither::keep | in:game_view::view::effect::alchemist | fn(game_view::view::effect::alchemist::Dither, i32, i32) -> bool | game-view\src\view\effect\alchemist.rs:372 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 49 | keep | game_view::view::projectile::sand_mage::Dither::keep | in:game_view::view::projectile::sand_mage | fn(game_view::view::projectile::sand_mage::Dither, i32, i32) -> bool | game-view\src\view\projectile\sand_mage.rs:240 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 50 | kind | game_view::ui::database_edit_ui::DbEditAppearanceTarget::kind | in:game_view::ui::database_edit_ui | fn(game_view::ui::database_edit_ui::DbEditAppearanceTarget) -> game_view::ui::athlete_appearance_popup::AppearanceTargetKind | game-view\src\ui\database_edit_ui.rs:100 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 51 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 52 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 53 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 54 | max_range | game_ai::max_range | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 55 | max_range_can_use | game_ai::plan_legacy::old::max_range_can_use | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2431 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | max_range_nearly_can_use | game_ai::plan_legacy::old::max_range_nearly_can_use | pub | fn(&game_core::Entity, &game_core::Entity, usize) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2397 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 57 | moba | game_core::MapDef::moba | pub | fn(&game_core::GameSetting) -> game_core::MapDef | game-core\src\simulation\map_def.rs:67 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 58 | new | game_ai::SmallActionRecall::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRecall | game-ai\src\small_action\move_actions.rs:659 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 59 | new | game_ai::SmallActionAroundPosition::new | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:831 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 60 | new_attack_range | game_ai::SmallActionTrace::new_attack_range | pub | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:75 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 61 | new_attack_range_margin | game_ai::SmallActionTrace::new_attack_range_margin | pub | fn(&game_core::OperationData, usize, usize, u64) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:79 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 62 | new_with_radius | game_ai::SmallActionAroundPosition::new_with_radius | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize, u64) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:837 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 63 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 64 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 65 | objective_attack_range | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::objective_attack_range | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:15 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 66 | objective_attack_range | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::objective_attack_range | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:15 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 67 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 68 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 69 | position_score_at_cell | game_ai::position_score_at_cell | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, i64, i64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1183 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 70 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 71 | positioning_accuracy | game_core::AthleteParameter::positioning_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:285 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 72 | positioning_effective | game_core::AthleteParameter::positioning_effective | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:291 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 73 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 74 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 75 | range_misjudge_rng | game_ai::range_misjudge_rng | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, usize) -> std::option::Option<game_core::NoiseRng> | game-ai\src\utils.rs:502 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 76 | range_misjudge_roll | game_ai::range_misjudge_roll | pub | fn(&mut rand::rngs::std::StdRng, &mut std::option::Option<game_core::NoiseRng>, u64, u64) -> u64 | game-ai\src\utils.rs:516 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 77 | remain_action_time | game_core::Entity::remain_action_time | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 78 | score | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:833 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 79 | serpen_action_score | game_ai::plan_legacy::sub_plan::serpen_action_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:843 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 80 | should_ignore_object_finish_kill_priority_target | game_ai::plan_legacy::old::should_ignore_object_finish_kill_priority_target | pub | fn(&game_core::PlayerState, &game_core::Entity, std::option::Option<&game_core::Entity>) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1208 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 81 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 82 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 83 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 84 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 85 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 86 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 87 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 88 | to_string | game_core::Position::to_string | pub | fn(&game_core::Position) -> std::string::String | game-core\src\simulation\entity.rs:665 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 89 | to_string | game_core::LineType::to_string | pub | fn(&game_core::LineType) -> std::string::String | game-core\src\simulation\state\player.rs:997 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 90 | to_string | game_core::JungleType::to_string | pub | fn(&game_core::JungleType) -> std::string::String | game-core\src\simulation\entity\jungle.rs:394 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 91 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 92 | v23_should_break_objective_hunt_anchor | game_ai::plan_legacy::team_plan::v23_should_break_objective_hunt_anchor | pub | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, (u64, u64)) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:59 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 93 | v25_objective_posture | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 94 | v27_objective_discipline_action | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 95 | v27_objective_safe_attack_position | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::v27_objective_safe_attack_position | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:41 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 96 | v27_positioning_value | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::v27_positioning_value | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(usize, &game_core::PlayerState, game_core::PositioningScore) -> i64 | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 97 | v27_positioning_value | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::v27_positioning_value | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(usize, &game_core::PlayerState, game_core::PositioningScore) -> i64 | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
</details>

⚠**미매칭 60개**: `act_actions`, `action_state`, `block_target_tick`, `cache`, `casting`, `clamp`, `cohesion`, `dealloc`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `elem`, `entry`, `extend`, `first`, `focus_enemy`, `format_inner`, `gen_range`, `grow_one`, `handle_error`, `infos`, `insert_no_grow`, `jungles`, `live_list`, `llvm.memcpy`, `map_or`, `memcpy32`, `move_actions`, `near_enemy`, `near_enemy_count`, `need_recall`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `object_finish`, `object_finish_objective`, `on_periodic_trajectory`, `on_trajectory`, `or_insert`, `others`, `parameter`, `payload`, `player_champion`, `pool`, `positioning_score`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`, `push_mut`, `range  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `reserve_internal_or_panic`, `retain`, `rustc_entry`, `s2_0`, `s2_0s4_0`, `setting`, `skip`, `target`, `trajectory_penalty`, `truncate`, `try_allocate_in`, `try_fold`, `untagged`, `variant`, `visible_state`, `void`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35254) · **형제 15개** (SerpenHuntSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan) -> game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:9 | True | fn() -> game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::objective_attack_range | in:game_ai::plan_legacy::sub_plan::serpen_hunt | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:15 | False | fn(&game_core::Entity, &game_core::Entity) -> u64 |
| 4 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::objective_wiggle_margin | in:game_ai::plan_legacy::sub_plan::serpen_hunt | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:20 | False | fn(&mut rand::rngs::std::StdRng, &game_core::Entity, &game_core::Entity, u64) -> u64 |
| 5 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::v27_positioning_value | in:game_ai::plan_legacy::sub_plan::serpen_hunt | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:33 | False | fn(usize, &game_core::PlayerState, game_core::PositioningScore) -> i64 |
| 6 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::v27_objective_safe_attack_position | in:game_ai::plan_legacy::sub_plan::serpen_hunt | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:41 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_ai::SmallActionPlay> |
| 7 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::attack_jungle_action | in:game_ai::plan_legacy::sub_plan::serpen_hunt | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:147 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 8 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:201 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 9 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::serpen_hunt | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:516 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 10 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::get_act_action | in:game_ai::plan_legacy::sub_plan::serpen_hunt | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:668 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 11 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::action_candidates_old | pub | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:677 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 12 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:816 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 13 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:833 | False | fn(&game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 14 | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:837 | True | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan) |

**`open` 30건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | (배치 G) L316/L566/L640/L656 의 `max_range` 헬퍼(파일 L16~26)와 L656 의 안전여유 헬퍼(L16~29) 소유 파일 — 같은 파일의 두 함수로 보이며 문자열 @anon.129 "v27 serpen safe attack position" 이 근처에 있으나 이 배치 범위 IR 에서 그 문자열의 사용처(store/format)는 나타나지 않았다(전역 상수만 확인) | 4 |  |
| 1 | 미탐색 | (배치 G) %7 team_plan 이 `noalias/readonly/dereferenceable` 없이 `nonnull align 8` 만 가진 이유 — DI 는 ref$<TeamPlan>(불변 참조). TeamPlan 에 UnsafeCell 계열 필드가 있는지 tcxdict 38필드 중 앞부분만 확인(추정: 내부 가변성). 이 배치 동작엔 영향 없음 | 3 |  |
| 2 | 표기 불가 | (배치 G) F1 L258/L268/L283/L298 타워 게이트의 소스 표기 — IR 상 `!is_some_and(\|t\| t.attack_effect.is_in_range(t,champ) && target.ty!=Tower && target.team!=champ.team)` 로 읽히나 `map_or`/`is_none_or` 등 어느 조합인지는 표기 불가(외연 동일) | 4 |  |
| 3 | 미탐색 | (배치 G) F1 Skill 갈래 L269 `expected_move_on_hit \|\| expected_rush_effect` 게이트 — divtable EffectType 일치율 94% 로 슬롯 0x68/0x60 이름을 얻었고 런타임 구현체는 Arc<dyn> 이라 도구로 못 본다. 이 게이트 때문에 이동 효과 없는 논타겟 스킬 후보가 전부 제거되는 것으로 읽히는데(IR 극성 확정) 의도(버그 vs 설계)는 미확인 | 3 |  |
| 4 | 미탐색 | (배치 G) L524 Skill2/Ult 갈래에서 level 부족(<=2 / <=4)이면 &None 을 unwrap 해 패닉하는 코드(unwrap_failed %396/%406) — 시전 중이면 해금돼 있어 도달 불가로 추정, 실측 없음 | 5 |  |
| 5 | 미탐색 | (배치 G) L537 `%421`(+0x31 on_periodic_trajectory)에 DI 이름 `on_trajectory` 가 붙어 있음 — 소스 지역변수명과 필드명 불일치(tcxdict 는 +0x30 on_trajectory · +0x31 on_periodic_trajectory). 판정식은 IR 대로 `+0x30 \|\| has_non_target \|\| +0x31` | 3 |  |
| 6 | 미탐색 | (배치 G) L596 near_enemies 의 filter_map/filter 클로저(iter_champions 심, m01/m06)는 이 배치가 읽지 않았다 — 캡처는 champ 하나(%38: begin,end,champ). 술어 미독해(미탐색) | 4 |  |
| 7 | 표기 불가 | (배치 G) L613 `%812 > 999` 의 소스 표기(`mr > 0` 추정) — 나눗셈 전 값과 비교하는 형태로 접혀 있어 표기 불가 | 5 |  |
| 8 | 미탐색 | (배치 G) L227·L223 의 `return` 이 %286(L514)로 합류하는데 %286 블록 자체는 배치 I 범위 — 여기선 '함수 끝'으로만 적음 | 4 |  |
| 9 | 미탐색 | (배치 G) constants 의 2·5·1 등 소형 값은 여러 의미로 재사용된다(meaning 에 열거). qcspec 의 shl 경고는 `shl i64 %n, 3`(포인터 stride 8) 때문이며 판정 상수 아님 | 4 |  |
| 10 | 미탐색 | (배치 G) s2_0 필터의 `%36 == -1` 조기 종료(fi_s2s3.ll 34번 블록: clone 결과 tag -1) — 니치 아티팩트로 보이며 실제 도달 여부 미확인 | 4 |  |
| 11 | 표기 불가 | (배치 H) 433 의 정확한 소스 표기: `info.nearest_enemy.map(\|(_,id)\| id) == Some(champ.id)` 로 읽었으나 `is_some_and(\|(_,id)\| id == champ.id)` 와 IR 외연이 같아 표기 불가(option.rs:1161·2440 인라인 체인만 근거). 동작(타워의 nearest_enemy.1 == 내 id)은 확정 | 4 |  |
| 12 | 미탐색 | (배치 H) Tower.nearest_enemy 튜플 (usize,usize) 의 .0 의미 — 이 범위는 .1 만 champ.id 와 비교. .0 은 미독(tcxdict 는 이름 없음) | 3 |  |
| 13 | 표기 불가 | (배치 H) 442 `live_list.get(0)` vs `.first()` — 표기 불가(동일 인라인). `moba().unwrap()` 의 헬퍼 이름도 미확인(GameMode tag!=0 → option::unwrap_failed 만 관측) | 4 |  |
| 14 | 표기 불가 | (배치 H) helper 102 의 `+8`: dbg 가 helper 38 로 찍히나 후보 셀 계산(124~128)에는 같은 항이 없어 현위치 전용으로 판정. 소스가 38 의 리턴식에 있는지 102 에 있는지는 재결합 때문에 표기 불가 — 동작은 확정 | 4 |  |
| 15 | 표기 불가 | (배치 H) 462 클로저 469/471 의 && 결합 순서(!is_in_action / !block_input / mr==0): block_input 호출 뒤 (mr!=0 \|\| block_input) 로 접혀 있어 소스상 mr==0 의 위치(469 뒤 471)는 dbg 줄로만 추정. 세 조건 모두 필요하다는 동작은 확정 | 4 |  |
| 16 | 미탐색 | (배치 H) range_misjudge_roll 이 rnd 를 몇 번 소비하는지·max_range_can_use / range_adjust / is_recent_visible / positioning_effective 내부 — 계약만 읽음(경로·game_core 경계). rnd 미러는 호출 순서(462 슬롯순 → 445 helper 144 → 475 → 485)만 이 명세가 보장 | 4 |  |
| 17 | 미탐색 | (배치 H) PositioningScoreData.cx/cy 가 누가 언제 채우는지(챔프 현재 셀로 추정되나 이 함수는 readonly parameter 로 받기만 함) — score_parameter 생성부 미독 | 5 |  |
| 18 | 미탐색 | (배치 H) positioning_score.value[7][7] 을 position_score_at_cell 이 그대로 조회하는지(7×7 창과 xb=cx-3+dx 가 정확히 대응) — position_eval 명세 소관 | 4 |  |
| 19 | 미탐색 | (배치 H) TLS: 이 함수 본체엔 접점 0. position_score_at_cell(셀당 1회, 최대 49회) · serpen_action_score/score · get_input 내부의 TLS 캐시 여부는 그 명세에서 확인해야 함(미탐색 = 콜리 내부) | 4 |  |
| 20 | 미탐색 | (배치 H) Option<SmallActionAroundPosition> 의 None 니치가 0xb1 == 0xff(-1)인 것은 IR 15985 근거. tcxdict --enum 으로 Option<…> 자체는 조회 불가(제네릭) | 3 |  |
| 21 | 미탐색 | (배치 H) GameContext.pool 로 만든 sret Vec 의 bump 필드(+8)가 pool 포인터인지 from_iter_in 내부 규약 — bumpalo 라이브러리 소관, 미독 | 4 |  |
| 22 | 미탐색 | (배치 H) reach.txt 사장 호출부(L254 unwrap_failed, 소스 205)는 배치 G 범위 — 배치 H 범위에 NA(version<2 전용) 콜리 없음(reach version=2·gamemode=0 접기에서 이 범위 블록 전부 live) | 4 |  |
| 23 | 미탐색 | (배치 I) L491 · L509 · L510 의 한글 주석 본문(rmeta 에 원문 없음 — 줄 길이 69/22/35자, mb 28/8/18 만 확인). 행동엔 영향 없음. | 3 |  |
| 24 | 표기 불가 | (배치 I) 소스 문장 형태(`if let Some(..)` / `if let Input::Move{x,y}`)는 rmeta_srcmap 줄 길이 산술(60=8+52 · 59=10+49 · 83=14+69 · 146=14+132 등 전 줄 정합)과 DILexicalBlock/DILocalVariable(!20722 type Input @489 · x,y @490)으로 복원한 것 — 컬럼 정보가 없어 한 줄 안 순서는 원리적으로 불가하나 이 범위는 줄마다 식이 하나라 동작 판정에는 영향 없음. | 3 |  |
| 25 | 표기 불가 | (배치 I) trajectory_possible==false 경로가 get_input 을 부르지 않고 L506 코드로 직행하는 것은 IR 사실(m02.ll:16986)이며, 소스가 `else { None }`(L487 = 14자 = 10+4 `None` 정합) 이라 컴파일러 점프-스레딩으로 설명된다. 소스 표기는 추정, 동작은 확정. | 4 |  |
| 26 | 미탐색 | (배치 I) position_score_at_position 내부(TLS 사용 여부 · on_trajectory 산출식)는 아웃오브라인 콜리(position_eval.rs:1192, pub) — 이 배치 범위 밖. 호출 계약만 기록. | 4 |  |
| 27 | 미탐색 | (배치 I) best_move_action(%2127) 이 가리키는 move_actions 원소의 variant 집합 및 SmallActionPlay::clone 의 variant 별 live 바이트는 배치 H(L475) · r14 action_candidates_old 표 소관. | 4 |  |
| 28 | 미탐색 | (배치 I) bumpalo::ChunkFooter 는 tcxdict 에 없어 distruct(DWARF 역산) 로 +0x20=ptr 을 확인 — 2차 폴백 근거. | 3 |  |
| 29 | 미탐색 | (배치 I) 언와인드 정리 phi %334(m02.ll:11896)의 `false` 유입(%2015)이 어느 반환 경로인지(511 경로에서 %91 이동 후 예외 시 %91 드롭 생략) — 정상 경로엔 영향 없음, 미추적. | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 G) L236 인라인 헬퍼(L668~673: Vec::new_in→battle_action→attack_summon_action→attack_jungle_action 을 extend)의 소유 파일·이름 — DISubprogram 을 추적하지 않았다(사슬 `L669<236` 의 scope 만 확인). 동작은 IR 로 확정 | 4 | 사실 서술 |
| 1 | (배치 G) F1 의 `L309` 헬퍼 이름(small_action.rs:309, DISubprogram 은 get_action(308) 만 잡힘) — Attack/Skill/Skill2/Ult 의 +0x8 을 Option<usize> 로 돌려주는 메서드. 동작 확정, 이름 미확정 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | (배치 H) helper 137 의 조건이 `best_score < here_score` 인지 `<=` 인지: IR `icmp slt %1736(best), %1760(here)` → best < here 면 None 쪽. 즉 best == here 는 통과. 확정(표기 불가 아님) — 다만 소스가 `>=` 로 썼는지 `!(<)` 인지는 무의미 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

