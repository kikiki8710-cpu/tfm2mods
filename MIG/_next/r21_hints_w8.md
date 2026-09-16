# r21 티어1 심층 — 웨이브 8 (2) · 0.5.8 명세 logic 전문 + exe 힌트

요구: 0.5.8 명세(아래 logic) 를 기준선으로 0.6.0 디컴을 **줄 단위로 대조**해 「어느 줄이 어떻게 바뀌었나 / 새 분기·조건·상수·필드·콜리」 를 명세 형식(줄번호 참조 · 등가 Rust)으로 낸다. version≥3 분기는 「v3 경로」 로 별도 표기(값 미확정 · v2/v3 둘 다 적는다). 대형 함수는 먼저 콜리 지도·블록 지도(0.5.8 ↔ 0.6.0 매핑)를 만들고 신규 구역을 특정한 뒤 신규 구역만 정독한다.

## `db9430` → `faa7c0` LineGankerPlan::next_plan  (9919→14470B · Δ+4551)
- 힌트: LineGankerPlan::next_plan(9.9→14.5KB · faa7c0 · ganker.rs +568줄 · d72e70 호출)
- exe 정렬: 명령 2031→2940 · 정렬 1787 · 잔여 구조 10 · 분기 10 · 콜리 주의: 1323a00→19903f0 미지(+488B) ; 31a01a3→381e0b3 미지 ; ca8180→e156e0 불일치(mig060 는 e15290) ; ca82f0→e15850 불일치(mig060 는 e15400) ; ca8460→e159c0 불일치(mig060 는 e15570) ; db8e60→fa9d60 콜리 변경?(J0.50·+1119B) ; dfb5d0→d35450 불일치(mig060 는 dd8b60) ; e32600→dbb9b0 불일치(mig060 는 dbbc30)
- 0.5.8 명세 #202 `line_gank__ganker__LineGankerPlan__next_plan` src game-ai\src\plan_legacy\old\line_gank\ganker.rs:99 · one_line: 라인 갱커 BigPlan 전이 판정: 노출(적에게 보임+정글 영역) 시 광역 인원우위(아군>적 && 적≠0)면 최근접 가시 적에 make_gank_battle, 아니면(열세·동수·근접 적 0 포함) 가시 적이 하나라도 있으면 즉시 Battle(Response, entry_src 3)+BattleHelp 챗 → 목표 부시 도착 여부 → 근처 적별 킬가능(die_tick)·타워복귀시간·인원비 게이트로 make_gank_battle → check_kill → 사거리 내 최근접 적
- 0.5.8 params: (sret):Option<BigPlan> 384B(None = +0 i64 -1 (m08.ll:95059 L102 · 97158 L155 · 98207 L24) · self:&mut LineGankerPlan(48B: +0 (IR 속성 noalias dereferenceable(48), readonly 없음 · initializes) · version:usize(본 함수 안 version 분기 0건. BattlePlan::new(97030) · make_gank_bat) · rnd:&mut StdRng(320B, ChaCha12)(직접 소비 0건(본체 94975~98368 에서 %3 의 load/store 0건 · 본 함수 define ) · player:&PlayerState(2528B)(+0x930 info.team(95064, bounds<2 95093) · +0x9c0 info.positi) · data:&OperationData(24B)(+0 cache(&AbstractGameWithCache 8840B) · +8 context(&GameCon) · positioning_score:&PositioningScoreData(2760B)(본 함수에서 미읽음 — make_gank_battle(7곳) · check_kill 에 전달만) · team_plan:&TeamPlan(1064B; IR 는 `ptr n(본 함수에서 미읽음 — make_gank_battle 에 전달만) · debug:&mut DebugFrameData(224B)(context.debug 일 때만 씀: L151 add_text(97083) · L200 infos(+0xa)
- 0.5.8 logic 전문:
```
// ganker.rs:99~246 (fn 루트 99). reach.py(version=2·gamemode=0): 블록 258 전부 살아있음 · 사장 0 · 접힌 분기 0 — NA 봉인 없음.
// 극성은 분기 방향으로 읽음. 인라인 헬퍼는 `<파일:줄>` 로 표시.
//
// L101  if let LineGankerPhase::ChangeJungle(_) = self.phase { return None }      // tag(+0x29) < 6 (95052~95055) → L102 store -1
// L106  let team = player.info.team;  let pos = player.info.position as usize;    // bounds<2
//       let champ = data.cache.player_champion[team][pos].unwrap();            // 95083~95086, unwrap_failed 95118
// L107  if game.is_visible(1 - team, champ.id)                                  // vtable +0xf8, 적팀에 내가 보이는가 (95112~95115)
//        && is_in_jungle_area(context, champ.x, champ.y) {                      // 95540~95547 (`&&`: 둘째는 첫째 참일 때만 호출)
//   L113  let near_wide_allies_len = cache.player_champion[team].iter().flatten()      // 5슬롯 언롤(자기 자신 포함)
//            .filter(|x| x.distance2(champ) < 62500000001                            // L111 ≤250000
//                     && x.hp*100 / x.stat_cached.hp > 39).count();                  // L112 HP≥40% (max_hp 0 → div0 패닉)
//   L115  let near_wide_enemies_len = cache.player_champion[1-team].iter().flatten()
//   L117     .filter(|x| data.blackboard[1-team].is_recent_visible(game, player, x)    // ★블랙보드 인덱스 = 1-team (96020)
//   L118            && x.distance2(champ) < 62500000001
//   L119            && x.hp*100 / x.stat_cached.hp > 39).count();                     // L120
//   L122  if near_wide_allies_len > near_wide_enemies_len && near_wide_enemies_len != 0 {   // 96437~96440
//   L124    let nearest_enemy = cache.iter_champions(1-team)                           // 적 5슬롯 언롤 + m12 14487 fold
//                .filter(|x| blackboard[1-team].is_recent_visible(game, player, x))    // closure#2
//                .min_by_key(|x| x.distance2(champ));                                  // closure#3 (첫 최소 유지)
//   L126    if let Some(e) = nearest_enemy {
//   L127      if let Some(p) = self.make_gank_battle(version, rnd, player, data, positioning_score, team_plan, e.id, 60, debug) { return Some(p) }   // 97043~97046, None 이면 L142 로
//           }                                                                            // None 이어도 L142 로(96929)
//         } else {
//   L132    let nearest_enemy = cache.iter_champions(1-team).filter(is_recent_visible).min_by_key(dist² to champ);   // closure#4/#5, 동일 fold
//   L134    if let Some(e) = nearest_enemy {
//   L135      self.chats.push(Chat::BattleHelp(e.id, 0));                                 // 태그 6, 96987~97027
//   L136      let mut b = BattlePlan::new(version, BattlePlanGoal::Response, data, player); b.entry_src = 3;
//             return Some(BigPlan::Battle(b));                                            // 태그 9 + memcpy 280B
//           }                                                                            // None 이면 L142 로(96658)
//         }
//       }
// L142  let _ = self.target_bush(player, data);          // ★인라인(ganker.rs:351~377). 값은 어디에도 안 쓰임(%18 재로드 0건) — unwrap 패닉 부작용만 남음:
//         //  352 champ = player_champion[team][pos].unwrap()  (재로드 95129, unwrap 95280)
//         //  353 optb = tower[line][team].or(tower2[line][team])                          (+0x180+line*32 / +0x190+line*32, 95142~95152)
//         //  354 t = cache.twin_towers[team].iter().min_by_key(|t| t.distance2(self.line.get_start_position(context.setting, team)))   (95239~95272)
//         //  359 nearest_tower = optb.or(t)  (closure#1: 95287~95302)
//         //  360 nearest_tower = nearest_tower.unwrap_or(cache.nexus[team].unwrap())   (361 unwrap 95318)
//         //  363 nexus = cache.nexus[team].unwrap()                                    (95335)
//         //  365 ms = blackboard[team].minion_state(self.line)  (<blackboard.rs:379~382> switch line → +0/+0x28/+0x50, 95346~95361)
//         //  366 front_minion = ms.front_minion.and_then(|m| game.get_entity_by_id(m))  (closure#2, vtable +0x1f0, 95389~95391)
//         //  368 reference = front_minion.filter(|x| x.distance2(nexus) >= nearest_tower.distance2(nexus))  (closure#3; 95463 `ult` 참이면 버림)
//         //        Some(x) → 373 near_jungle_bush(context, champ.x, champ.y, x.x, x.y)
//         //        None    → 369 center = context.map.lane_path(self.line, team)[3];  370 near_jungle_bush(context, champ.x, champ.y, center.x, center.y)
//         //  374 .unwrap()   (Option<(u64,u64)> 태그만 검사 95495~95500)
// L144  let bush = self.target_bush_v41(player, data);                                   // fastcc(line, team, pos, cache, context) → usize (95508~95510)
// L148  let champ_bush = context.map.bushes[min(champ.y/32000, 29)][min(champ.x/32000, 29)];   // 95513~95533 ([y][x] 순)
// L150  if context.debug {  L151 debug.add_text(champ.x, champ.y + 20000, format!("bush: {:?}, champ_bush: {:?}", bush, champ_bush), Color(1,0,0,1), 1) }
// L154  if champ_bush != bush { L155 return None }                                       // 97049~97052 (eq → 진행)
// L158  let dist_cut: u64 = 150000;
// L160  let near_allies_p: bumpalo Vec<usize> = cache.player_champion[team].iter().enumerate()
//           .filter(|(_, x)| x.is_some_and(|x| x.distance2(champ) <= dist_cut*dist_cut))   // closure#6 (m08 112913), 자기 자신 포함(거리 0)
// L163      .map(|(i, _)| i).collect_in(context.pool);                                      // 97106~97116 (bump = context+0 %803)
// L165  let near_enemies_p: bumpalo Vec<usize> = cache.player_champion[1-team].iter().enumerate()
//           .filter(|(_, x)| x.is_some_and(|c| blackboard[1-team].is_recent_visible(game, player, c) && c.distance2(champ) <= dist_cut²))   // closure#8 (112983)
// L168      .map(|(i, _)| i).collect_in(pool);                                              // 97137~97155
// L170  let nearest_enemy_tower = cache.iter_towers_without_nexus(1-team)                   // 97172 (120B 이터레이터)
// L171      .min_by_key(|t| t.distance2(champ));                                             // closure#10 (m12 39469) · fold m06 23823 / m11 17703 · 97189~97372
// L173  for e in near_enemies_p.iter() {                                                    // 97430~97440 (ptr 순회, e: &usize)
//   L175  let near_allies_p = cache.player_champion[team].iter().enumerate()                // 루프 지역 재바인딩(%32)
//             .filter(|(_, x)| x.is_some_and(|c| c.id == champ.id || c.distance2(player_champion[1-team][*e].unwrap()) <= dist_cut²))   // closure#11 (113084)
//   L180      .map(|(i,_)| i).collect_in(pool);
//   L182  let near_enemies_p = cache.player_champion[1-team].iter().enumerate()             // (%30)
//             .filter(|(_, x)| x.is_some_and(|x| x.distance2(player_champion[1-team][*e].unwrap()) <= dist_cut²))   // closure#13 (113214)
//   L185      .map(|(i,_)| i).collect_in(pool);
//   L187  let enemy_die_tick = get_die_tick_player(data, 1-team, *e, &near_allies_p, None, None);     // 97511~97516 (Option<SmallAction>::None = -1)
//   L188  let me_die_tick    = get_die_tick_player(data, team, pos, &near_enemies_p, None, None);     // 97528~97532
//   L190  let e_ent = cache.player_champion[1-team][*e].unwrap();                          // bounds<5 97533, unwrap 97553
//   L191  let move_to_tower_tick = if let Some(tower) = nearest_enemy_tower {
//   L192~194   e_ent.distance(tower) / e_ent.stat_cached.move_speed                        // 97558, 97563~97570 (speed 0 → div0 패닉)
//           } else { 99999999 };                                                             // 97578
//   L199  if context.debug { L200 debug.infos.entry(e_ent.id).or_default().push(format!("target: {:?}, near_allies_p: {:?}, near_enemies_p: {:?}, move_to_tower_tick: {}, me_die_tick: {}, enemy_die_tick: {}", e_ent.id, near_allies_p, near_enemies_p, move_to_tower_tick, me_die_tick, enemy_die_tick)) }
//   L205  if enemy_die_tick > 30 && near_allies_p.len() <= near_enemies_p.len() { continue }   // 97583~97585 + 97744~97748 (ugt 30 참 & !(allies>enemies) → 1001 drop → 다음 e)
//   L209  if move_to_tower_tick <= 120 {                                                   // ult 121 (97739)
//   L211    if enemy_die_tick <= 60 {                                                       // ult 61 (97778)
//   L212      if let Some(p) = self.make_gank_battle(…, e_ent.id, 60, debug) { return Some(p) }   // 97887~97908
//   L213    } else if near_allies_p.len() > near_enemies_p.len() {                          // 97882~97886
//   L214      if let Some(p) = self.make_gank_battle(…, e_ent.id, 60, debug) { return Some(p) }   // 97894~97903
//           }
//   L216  } else if me_die_tick > 120 && near_allies_p.len() >= near_enemies_p.len() {      // 97773~97775 · 97790~97794(`allies < enemies` 참이면 다음 else-if 로)
//   L217    if let Some(p) = self.make_gank_battle(…, e_ent.id, 60, debug) { return Some(p) }   // 97797~97834
//   L218  } else if self.wait_limit.saturating_sub(game.tick())                            // vtable +0x28 · usub.sat 97804
//   L219             > context.setting.tick_per_second * 3 {                                // 97805~97809 → 참이면 아무것도 안 함(1047: drop → 다음 e)
//   L220  } else if near_allies_p.len() > near_enemies_p.len() {                            // 97814~97818
//   L221    if let Some(p) = self.make_gank_battle(…, e_ent.id, 60, debug) { return Some(p) }   // 97820~97829
//         }
// L223  }  // 루프 지역 Vec 2개 drop 후 다음 e
// L225  if let Some((e, t)) = check_kill(version, rnd, player, data, positioning_score, debug) {   // 97474 · Option<(usize,usize)> +8 e, +0x10 t (98175~98179)
// L226    if near_allies_p.len() > near_enemies_p.len() {                                   // 바깥 Vec(%37 vs %35) 98006~98011
// L227      if let Some(p) = self.make_gank_battle(…, e, t, debug) { return Some(p) }        // ★min_trace = check_kill 의 t (98181~98187)
//         }
//       }
// L232  let nearest_enemy = cache.iter_champions(1-team).filter(|x| blackboard[1-team].is_recent_visible(game, player, x))   // closure#15, 98081~98121 언롤
// L233      .min_by_key(|x| x.distance2(champ));                                             // closure#16, fold m12 14907
// L235  if let Some(ne) = nearest_enemy {                                                    // 98197~98198
// L236    let d = ne.distance(champ);                                                        // 98203 (Entity::distance, 제곱 아님)
// L237    let attack_range = champ.attack_effect.as_ref().unwrap().range(champ)             // <effect.rs:26> = champ.stat_buff_cached.range + eff.range + (champ.level-1)*eff.growth_range (98238~98257, 98304~98305)
//                          + eff.range_adjust(champ, ne)                                     // 98247 (게임 경계)
//                          + champ.radius() + ne.radius();                                    // <entity.rs:1511~1515>: radius_mult==0 ? radius : radius*(mult+100)/100 (98258~98303)
// L240    if d > attack_range { return None }                                                // 98310~98311 (ugt → 1152 None)
// L241    return self.make_gank_battle(…, ne.id, 60, debug);                                 // 98314~98322 — 콜리 결과가 None(-1)이어도 그대로 None 반환
//       }
// L245  None                                                                                // 98207
// L246  } // 바깥 near_allies_p/near_enemies_p drop
```

## `cb7540` → `f356b0` HideSubPlan::action_candidates  (11547→13814B · Δ+2267)
- 힌트: HideSubPlan::action_candidates(11.5→13.8KB · f356b0 · HideSubPlan 신규 필드 ambush_cell/direct/stealth · PassiveJungle 매복이 Hide{+0x18=ambush_cell,+0x23=1,+0x24=1} 생성)
- exe 정렬: 명령 2530→2833 · 정렬 2293 · 잔여 구조 5 · 분기 10 · 콜리 주의: 12857f0→1643790 변경 ; 128cc90→164b370 미지 ; 31a01a3→381e0b3 미지 ; cada80→ed8550 미지(+1457B) ; cada80→f26680 미지(+2B) ; eb8b80→ecdb90 불일치(mig060 는 ed8550)
- 0.5.8 명세 #194 `hide__Hide__action_candidates` src game-ai\src\plan_legacy\sub_plan\hide.rs:19 · one_line: [배치 L · hide.rs:0~101] Hide 서브플랜 행동 후보 루트: 적 시야 노출 갱신 → check_move 전엔 부시 위치 기준 Morgard/Serpen 캠프로 이동(AroundPosition) → check_move 후엔 부시 근처 최근접 적을 찾아 노출 시 아군/적 수·1v1 판정으로 전투(battle_action+Trace)/도주(RunAway), 비노출 시 부시 대기(AroundBush)+정글 스틸(Attack/Skill/Skill2). hide.rs:103~139 = `is_visible_to_enemy && !check_move` 분기(enemy_spotted_me 는 쓰기만 하고 읽지 않음)·반환
- 0.5.8 params: (sret):bumpalo Vec<SmallActionPlay>(ptr@0 · bump@+8 · cap@+0x10 · len@+0x18. 본 배치는 지역 res(%46, L) · self:&mut HideSubPlan(16B)(IR 속성 `noalias captures(none) dereferenceable(16)` — readonl) · version:usize(전 함수에서 분기 없음 — battle_action 1번째 인자로만 전달(본 배치 L80 m02.ll:237) · rnd:&mut StdRng(320B)(L43/L50 AroundPosition::new_with_out_line · L96 AroundBush::) · player:&PlayerState(2528B)(+0x930 info.team(L22) · +0x9c0 info.position@tag(L22, i32) 읽) · data:&OperationData(24B)(+0 cache(&AbstractGameWithCache) · +8 context(&GameContext; ) · _parameter:&ScoreParameter(5384B)(DI 이름 `_parameter`. 전 함수에서 미사용(define 줄 외 `%6` 출현 0) — 시그니처만) · _debug:&mut DebugFrameData(224B)(DI 이름 `_debug`. IR 속성 readnone = 전 함수에서 읽지 않음(정정: tcx 시그니처는 )
- 0.5.8 logic 전문:
```
// hide.rs:0~101 (배치 L)
// 진입: 함수 머리. 인자 = (&mut self HideSubPlan16B, version, rnd, player, data, _parameter, _debug) -> Vec<SmallActionPlay>

// L20  res = Vec::new_in(data.context.pool)                                   // m02.ll:20378~20389
// L22  team = player.info.team(+0x930); assert team<2 (panic_bounds_check 20397)
//      champ = data.cache.player_champion[team][player.info.position@tag(+0x9c0)].unwrap()   // 20410~20423 · None→unwrap_failed 20446
// L23  is_visible_to_enemy = game.is_visible(enemy_team=1-team, champ.id)      // vtable+0xf8 · 20430~20442
// L26  if is_visible_to_enemy {
// L27      self.enemy_spotted_me = true;                                       // store i8 1 → self+0xa 20465
//      } else if self.check_move {                                             // 20455~20457 (표기: `else if` 인지 `else { if }` 인지 표기 불가 — 블록 구조는 이와 동치)
// L30      self.enemy_spotted_me = false;                                      // 20589
//      }
// L34  if !self.check_move {                                                   // 20467~20470 / 20457
// L35      let (x,y) = BUSH_POSITIONS.into_iter().filter(|&(x,y)| map.bushes[y][x] == self.bush).next().unwrap();
//                 // map = data.context.map(+0x20) · bushes @MapDef+0x1c98 [y][x] · 71개 상수 셀 @anon.148 · unwrap 실패 20597
// L37      bx = x*32000+16000; by = y*32000+16000;                             // 20607~20612 (by 는 −(…) 로 접힘)
// L39      if is_top_side(context, bx, by) {   // = (setting.height(+0x12c0) − by) >= bx (MIR map_regions.rs:22~24) · IR %147 = !is_top (20621) → 149(L48) / 151(L40)
// L40          (ex,ey) = map.camp_pos(JungleType::Morgard(4), blue_side = team==0)   // 20630
// L42          if champ.distance_sq(ex,ey) > 100000² {                         // 20726~20747 (dx,dy=|Δ| · dx²+dy² ugt 10000000000)
// L43              res.push(AroundPosition(SmallActionAroundPosition::new_with_out_line(rnd, data, ex, ey, 5, Outline(1))))   // 20757 · push 20778~20797
//              } else {
// L45              self.check_move = true;                                     // 20751
//              }
// L48      } else {
//              (ex,ey) = map.camp_pos(JungleType::Serpen(5), team==0)          // 20626
// L49          if champ.distance_sq(ex,ey) > 100000² {                         // 20642~20663
// L50              res.push(AroundPosition(new_with_out_line(rnd, data, ex, ey, 5, Outline)))   // 20673 · 20694~20713
//              } else {
// L52              self.check_move = true;                                     // 20667
//              }
//          }
//      }
// L57  if self.check_move {                                                    // 재로드 20578 (123) / 20583 (128: visible&&check_move) / 20591 (131: !visible&&check_move)
//      // ---- 거짓 분기: → 배치 M(줄 107) — 123→%211(107) · 128→%244(108/110, enemy_spotted_me=true 상수접힘) · 131→%243(136, enemy_spotted_me=false 접힘)
// L58      let (x,y) = BUSH_POSITIONS…filter(map.bushes[y][x]==self.bush).next().unwrap();   // 20811~20907 · 실패 21288
// L59      bx = x*32000+16000; by = y*32000+16000;                             // 21298~21305 (스택 %39/%38 — 클로저가 참조 캡처)
// L60      enemies = data.cache.player_champion[enemy_team]   (5칸 Option<&Entity>)   // 21306~21313
// L61      nearest_enemy: Option<&Entity> = iter_champions(enemies)
//              .filter(|e| data.blackboard[enemy_team].is_recent_visible(game, player, e)   // 21414 (aux m12.ll:7823 는 blackboard[1-player.info.team] 로 재계산)
// L62                   && e.distance_sq(bx,by) < 250000²+1)                    // 21430~21457 / aux 7839~7866
// L63          .min_by_key(|e| e.distance_sq(champ))                            // 첫 원소 key 21488~21508 → fold 21511(aux) · 동점 시 앞 원소 유지(aux 7920~7924)
//              // 결과 { i64 key, ptr } — ptr null = None (21515~21518)
// L66      if is_visible_to_enemy {                                            // 21519 / 21524
// L67          if let Some(enemy) = nearest_enemy {                            // 21535 (None → L91)
// L70              nearby_allies = iter_champions(player_champion[team]).filter(|e| e.id != champ.id && e.distance_sq(champ) < 150000²+1).count()   // closure$5(L69) · 5칸 unrolled 22596~22919 · 결과 %1035
// L73              nearby_enemies = iter_champions(player_champion[enemy_team]).filter(|e| e.is_visible_from(champ) && e.distance_sq(champ) < 150000²+1).count()
//                      // closure$6(L72) · is_visible_from 인라인(entity.rs:1481): champ.team Neutral(태그1)→항상 참(1043 경로 22973~22919) / Player(t)→ t<2 검사(23271, 실패 panic 23651) 후 e.visible_state[t]@tag==0(23309~23313) · 결과 %1332(23710)
// L75              dominated = nearby_allies < nearby_enemies                   // 23712 icmp ult
// L76              can_fight = !dominated && can1v1win(rnd, player, data, champ, enemy)   // 23714 (dominated 면 호출 생략) · 23717
// L78              if can_fight {                                              // 23728
// L80                  res.extend(battle_action(version, rnd, player, data, 5))   // 23732 · extend 23808
// L81                  res.push(Trace(SmallActionTrace::new(data, enemy.id, 5)))   // 인라인: t=game.get_entity_by_id(enemy.id)(23827) · start_tick=game.tick()(23835) · goal=(t.x,t.y) or (0,0)(23839~23860) · margin 15000 · end_delay 5 · path_finder/last_escape None · 태그 14 (23861~23881) · push 23903~23926
//                  } else {
// L84                  res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)))   // 23723 · 태그 3 23738 · push 23760~23784
// L86                  if !dominated {                                         // 23786
// L87                      res.extend(battle_action(version, rnd, player, data, 5))   // 23790 · 23797
//                      }
//                  }
//                  → 배치 M(줄 107) %1424
// L91          } else {   // nearest_enemy None
//                  res.push(RunAway(new_with_skill(data, player, 5, false)))     // 22970 · 태그 3 23933 · push 23955~23979 → 배치 M(107) %1424
//              }
// L93      } else {   // !is_visible_to_enemy
//              if let Some(enemy) = nearest_enemy {                            // 21532
// L94              res.push(AroundBush(SmallActionAroundBush::new_with_target(data, enemy, self.bush, self.out_line)))   // 21543 · 태그 12 21558 · push 21580~21604
//              } else {
// L96              res.push(AroundBush(SmallActionAroundBush::new_with_out_line(rnd, data, player, self.bush, self.out_line)))   // 21552 · 태그 12 22506 · push 22528~22552
//              }
// L101         res.extend(self.steal_jungle_action(rnd, player, data));        // 전부 인라인(hide.rs:142~, define 없음) · extend 23992 → 배치 M(107) %1428
//              // ===== steal_jungle_action(&self, rnd, player, data) 인라인 본체 (루트 줄 101, IR 21619~22502 + 23990~23993) =====
//              // L143 steal_res = Vec::new_in(bump); steal_range = 150000       // 21632~21639
//              // L144 champ = cache.player_champion[team][pos].unwrap()         // 21640~21643 (재로드 · 21756 unwrap_failed)
//              // L147 bush = BUSH_POSITIONS.into_iter().filter(|&(x,y)| map.bushes[y][x]==self.bush).next()   // closure#0 · 21660~21753
//              // L149     .map(|(x,y)| (x*32000+16000, y*32000+16000))         // closure#1 · 21767~21770
//              // L150     .unwrap_or((champ.x, champ.y)) → (bx,by)              // 21774~21808 (phi %505/%506)
//              // L156 for jungle in cache.jungles (Vec<&Entity> +0xd0 ptr / +0xe8 len) {   // 21821~21871 · 끝나면 %1427 extend
//              // L158     if !jungle.ty.is_jungle(enemy_team) { continue }     // ty@tag(+0x68)==4 && ty.Jungle.info.camp_type.0(+0x98)==enemy_team (21879~21888)
//              // L163     if jungle.distance_sq(bx,by) > steal_range² { continue }   // 21891~21913 ugt 22500000000
//              // L168     move_speed = champ.stat_cached.move_speed(+0x640)    // 21916
//              // L171     if champ.can_attack() {                                // 21918
//              // L172         if let Some(atk) = &champ.attack_effect {         // +0x4c0 != -1 (21934~21936)
//              // L173             expected_dmg = atk.expected_damage_target(context, champ as &dyn AbstractEntity, jungle)   // 21942 (vtable @anon.54)
//              // L174             range = atk.range(champ) + atk.range_adjust(champ, jungle) + champ.radius() + jungle.radius()
//              //                      // Effect::range 인라인 = range(+0x4a0) + growth(+0x4a8)*(level(+0x5c8)−1) + stat_buff_cached.range(+0x438) (21947~21957) · range_adjust 21952 · radius 인라인 21958~22002
//              // L175             dist_sq = jungle.distance_sq(champ)             // 22005~22030 (champ.x/y, 부시 아님)
//              // L176             max_dist = range + move_speed*20                // 22031~22038
//              // L179             if jungle.hp(+0x670) <= expected_dmg && dist_sq <= max_dist² {   // 22039~22045 (select = 양쪽 다 계산, 소스 순서 표기 불가)
//              // L180                 steal_res.push(Attack(SmallActionAttack::new(data, jungle.id)))   // 22052 · 태그 15 22057 · push 22079~22105
//              //                  }
//              //              }
//              //          }
//              // L186     if champ.can_skill() {                                 // 21929
//              // L187         if let Some(skill) = &champ.skill_effect {        // +0x4f8 != -1 (22122~22124)
//              // L188             if skill.target.check(champ, jungle) {         // CastingTarget::check(+0x4f0, champ, jungle) 22130
//              // L189                 expected_dmg = skill.expected_damage_target(context, champ, jungle)   // 22138
//              // L190                 range = skill.range(champ) + skill.range_adjust(champ, jungle) + champ.radius() + jungle.radius()   // 22143~22198
//              // L191                 dist_sq = jungle.distance_sq(champ)         // 22201~22226
//              // L192                 max_dist = range + move_speed*20            // 22227~22234
//              // L194                 if jungle.hp <= expected_dmg && dist_sq <= max_dist² {   // 22235~22241
//              // L195                     steal_res.push(Skill(SmallActionSkill::new(data, jungle.id)))   // 22248 · 태그 16 22253 · push 22275~22300
//              //                      }
//              //                  }
//              //              }
//              //          }
//              // L202     if champ.can_skill2() {                                // 22117
//              // L203         if let Some(skill2) = champ.skill2_effect() {     // 인라인(entity.rs:1692): level>2 ? &skill2_effect(+0x500) : &NONE · tag +0x530 != -1 (22315~22322)
//              // L204             if skill2.target.check(champ, jungle) {        // +0x528 · 22329
//              // L205                 expected_dmg = skill2.expected_damage_target(context, champ, jungle)   // 22337
//              // L206                 range = skill2.range(champ) + range_adjust + champ.radius() + jungle.radius()   // 22342~22398
//              // L207                 dist_sq = jungle.distance_sq(champ)         // ~22426
//              // L208                 max_dist = range + move_speed*20            // 22427~22434
//              // L210                 if jungle.hp <= expected_dmg && dist_sq <= max_dist² {   // 22435~22441
//              // L211                     steal_res.push(Skill2(SmallActionSkill2::new(data, jungle.id)))   // 22448 · 태그 17 22453 · push 22475~22499
//              //                      }
//              //                  }
//              //              }
//              //          }
//              //      }   // loop 22312 → 21857
//              // L219 (steal_res 는 언와인드 시 drop_glue 21610 · 정상 경로엔 extend 로 소유권 이전)
//              // ===== steal_jungle_action 끝 =====
//          }
//      } else { → 배치 M (hide.rs:107~) }
// 이후 반환(L136~139: res → sret memcpy · 언와인드 cleanup %57 은 L139 drop_glue) = 배치 M

// 사장 코드: reach.py(version=2 · gamemode=0) 결과 블록 351 전부 live · NA 봉인 대상 0

// hide.rs:103~139 (배치 M)
// 승계(배치 L 정의, IR 로 재확인): team=%54=player+0x930 · champ=%68=data.cache.player_champion[team][pos].unwrap() · is_visible_to_enemy=%79=data.cache.game.vtable[0xf8](AbstractGame::is_visible)(1-team, champ.id) (hide.rs:23) · %74 = 1-team(적 팀) · res=%46 빈 bumpalo Vec(ptr=8,bump=data.context.bump,cap=0,len=0)

// 진입: 배치 L 의 line 57 분기(%123 check_move==0 → %211 / %128·%131 → 직접) 와 line 58~101 블록 끝(%1349·%1359·%1404·%1419 → %1424) · line 101 끝(%1427 → %1428 → line 136)

// hide.rs:107  if is_visible_to_enemy && !self.check_move {        // 두 조건 모두 필요. %211: br %79 → %244/%243 · %1424: check_move 재로드(IR 23986) → 1 이면 %243(136), 0 이면 %244(108)
//   근거: CFG 에서 %79=0 으로 접으면 %1424·%244 도달 불가(cfg79.py BFS: 156블록 중 없음) ⟹ %1424 경로는 %79=1 이 정적으로 확정된 경로(jump-threading). 두 조건의 소스 표기 순서는 표기 불가(column 없음).

// hide.rs:108  data.iter_champions(1-team)                          // %245 = &player_champion[1-team] (5 x Option<&Entity>)
// hide.rs:109    .filter(|x| x.is_visible_from(champ))              // closure#7 인라인: champ.team==Neutral(tag 1) → true / Player(t): t<2 bounds(아니면 panic_bounds_check 2) → x.visible_state[t].tag==0(Visible)
// hide.rs:110    .min_by_key(|x| x.distance_sq(champ))              // closure#8: |x.x-champ.x|²+|x.y-champ.y|² (utils.rs:2158). 슬롯 0~4 를 순서대로 훑어 필터 통과 첫 원소를 %328 에서 잡고(first=(dist²,ptr)), 잔여는 aux fold(m12.ll:12641)로: 같은 필터·같은 키·min_by(cmp(acc,new)>Greater 일 때만 교체 = 동률이면 앞 원소 유지). 5슬롯 전부 실패 → %327: nearest_enemy=None
//   let nearest_enemy: Option<&Entity>

// hide.rs:112  if let Some(enemy) = nearest_enemy {                 // %1429: null 검사 → None 이면 %1587(line 132)

// hide.rs:114/115  let nearby_allies = data.iter_champions(team).filter(|x| x.id != champ.id && x.distance_sq(champ) < 22500000001).count();
//     // %66 = player_champion[team] 5슬롯 완전 언롤(%1432~%1580). null skip · id==champ.id 는 0 · 아니면 zext(dist² ult 150000²+1) 누적. ⚠거리 기준점은 enemy 가 아니라 **champ 자신**(%1434/%1435 = champ.x/y 재로드 IR 24043~24044)

// hide.rs:117/118  let nearby_enemies = data.iter_champions(1-team).filter(|x| x.is_visible_from(champ) && x.distance_sq(champ) < 22500000001).count();
//     // %245 5슬롯 완전 언롤. champ.team 으로 루프 언스위치(IR 24410~24415 freeze): Neutral → %1588 경로(가시 검사 생략) / Player(t) → %1705: t<2 이면 %1710 경로(visible_state[t].tag==0 && dist²) / t>=2 이면 %1709 경로(첫 non-null 슬롯에서 panic_bounds_check(t,2))

// hide.rs:120  let dominated = nearby_allies < nearby_enemies;       // icmp ult %1581,%1877 (IR 25162, samesign)
// hide.rs:121  let can_fight = !dominated && can1v1win(rnd, player, data, champ, enemy);   // dominated 면 can1v1win 호출 자체를 안 함(IR 25164 → %1881). DI: can_fight=%1880 는 %1882(호출 뒤)에서만

// hide.rs:123  if can_fight {
// hide.rs:124      res.extend(battle_action(version, rnd, player, data, 5));      // sret %26 → extend(res, vec.ptr, vec.len) → %1909 → line 136
//              } else {
// hide.rs:126      res.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)));   // sret %24 136B → %25 memcpy · tag 3 @+0xb1 · len==cap 이면 reserve_internal_or_panic(res,len,1,true) · ptr[len]=184B memcpy · len+=1
// hide.rs:127      if !dominated {                                                // IR 25236: br %1878 → %243(136) / %1899(128). 즉 「1v1 승산 없음이지만 수적 열세는 아님」일 때만
// hide.rs:128          res.extend(battle_action(version, rnd, player, data, 5));  // sret %23
//                  }
//              }
//   → line 136
// } else {   // nearest_enemy == None (%1587)
// hide.rs:132      res.push(RunAway(new_with_skill(data, player, 5, false)));    // sret %21 → %22 · tag 3 · push 동일 수순(IR 25266~25314)
// hide.rs:133      res.extend(battle_action(version, rnd, player, data, 5));      // sret %20 (IR 25317~25325)
// }
// }  // end if 107

// hide.rs:136  res.extend(attack_summon_action(player, data));       // %243: sret %19 → extend(IR 25335). 107 이 거짓인 경로(%211·%1424·%1428) 도 전부 여기로 합류
// hide.rs:138  res                                                  // memcpy 32B → sret %0 (IR 25340)
// hide.rs:139  }                                                    // ret (IR 25342). 언와인드 %57: drop_glue(bumpalo Vec<SmallActionPlay>)(res) 뒤 caller 로 (IR 20402)

// 배치 M 이 push 하는 variant 집합 = {RunAway(태그 3)} 2사이트(126·132), 생성자 인자 (data, player, end_delay=5, with_skill=false) 동일. 나머지 원소는 battle_action ×3 · attack_summon_action ×1 의 반환 Vec 을 extend.
// 배치 M 범위 내 self(%1) 쓰기: 없음. TLS(LocalKey::with) 접점: 없음(함수 전체 0건, 콜리 can1v1win/battle_action/attack_summon_action 내부는 그 명세 소관).
// 사장 코드: reach.txt 사장 호출부 0 — NA 봉인 대상 없음.
```
