---

### `202` LineGankerPlan::next_plan — 라인 갱커 BigPlan 전이 판정: 노출·인원우위/열세 대응 → 목표 부시 도착 여부 → 근처 적별 킬가능(die_tick)·타워복귀시간·인원비 게이트로 make_gank_battle → check_kill → 사거리 내 최근접 적

| 항목 | 값 |
|---|---|
| id | `line_gank__ganker__LineGankerPlan__next_plan` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank6gankerNtB2_14LineGankerPlan9next_plan` |
| 소스 | `game-ai\src\plan_legacy\old\line_gank\ganker.rs:99` |
| IR | `m08.ll` 94975~98368행 |
| 경로·가시성 | `game_ai::plan_legacy::old::LineGankerPlan::next_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `db9430` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[202]/sig/tls/<키>`)**

- `name`: (없음)
- `role`: 직접 접점 0
- `key`: -
- `layout`: -
- `invalidation`: -
- `call_conditions`: 본 함수 본체(94975~98368) 및 aux 15 조각 전부에서 LocalKey::with · thread_local 심볼 0건(grep). v47_siege_stance(SIEGE_STANCE_CACHE)·v48_cast_beams(CAST_BEAMS)·champion_hp_value(HP_VALUE_MEMO)·position_eval_at(POS_EVAL_CACHE)·interaction_score(INTER_CTX) 는 이 함수가 직접 부르지 않는다. TLS 가 있다면 콜리 내부(make_gank_battle · check_kill · get_die_tick_player · BattlePlan::new) — 호출 순서는 logic 의 줄 순서(L127→L136→L212/214/217/221→L225→L227→L241) 그대로이며 한 호출에서 Some 이 나오면 즉시 반환하므로 그 뒤 콜리는 실행되지 않는다

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<BigPlan> 384B | None = +0 i64 -1 (m08.ll:95059 L102 · 97158 L155 · 98207 L245), 나머지 376B 미기록. Some(BigPlan::Battle) = +0 i64 9(메모리태그, tcxdict --enum BigPlan idx7→9) + +8..+0x120 BattlePlan 280B memcpy(97034~97036 L136), +0x120..+0x180 미기록. make_gank_battle 경로(7곳)는 sret %0 를 그대로 넘겨 콜리가 채운다(살아있는 바이트 = make_gank_battle 명세) — 호출 후 +0 == -1 이면 None 으로 판정해 계속 진행(97044~97046 등) | 3 |
| 1 | 1 | self | &mut LineGankerPlan(48B: +0 chats Vec<Chat> · +0x18 setup_limit · +0x20 wait_limit · +0x28 line LineType · +0x29 phase LineGankerPhase) | IR 속성 noalias dereferenceable(48), readonly 없음 · initializes 없음 ⇒ &mut. 쓰기 표면 = chats push 1곳뿐(L135: grow_one(%1) 97011 · 원소 store 97021~97025 · len(+0x10) store 97027). phase(+0x29)·line(+0x28)·wait_limit(+0x20) 는 읽기만. make_gank_battle 에는 self 가 ArgumentPromotion 으로 탈락(포이즌) — 콜리는 self 를 안 읽는다 | 4 |
| 2 | 2 | version | usize | 본 함수 안 version 분기 0건. BattlePlan::new(97030) · make_gank_battle(7곳) · check_kill(97474) 의 첫 인자로 전달만 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B, ChaCha12) | 직접 소비 0건. make_gank_battle · check_kill 에 전달만(check_kill 쪽 define 은 %2 readnone) | 4 |
| 4 | 4 | player | &PlayerState(2528B) | +0x930 info.team(95064, bounds<2 95093) · +0x9c0 info.position@tag(95075, i32→zext, player_champion 인덱스) 직접 읽음. is_recent_visible · get_die_tick_player · 클로저 캡처로 전달 | 4 |
| 5 | 5 | data | &OperationData(24B) | +0 cache(&AbstractGameWithCache 8840B) · +8 context(&GameContext 64B) · +0x10 blackboard(&[Blackboard;2]) | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B) | 본 함수에서 미읽음 — make_gank_battle(7곳) · check_kill 에 전달만 | 4 |
| 7 | 7 | team_plan | &TeamPlan(1064B; IR 는 `ptr nonnull` 만 — tcx sig 는 공유참조) | 본 함수에서 미읽음 — make_gank_battle 에 전달만 | 4 |
| 8 | 8 | debug | &mut DebugFrameData(224B) | context.debug 일 때만 씀: L151 add_text(97083) · L200 infos(+0xa0) entry(e.id).or_default().push(String)(97590~97734) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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

**`mem` 메모리 접근 46건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LineGankerPlan(self) | 0x29 | phase@tag (니치: 6 WaitResponse/7 Setup/8 Cancel/0..5 = ChangeJungle(JungleType)) | r | L101 95052~95055 `icmp ult i8 tag, 6` → ChangeJungle 이면 즉시 None | 4 | OK |  |
| 1 | LineGankerPlan(self) | 0x28 | line (LineType 0 Top/1 Mid/2 Bottom) | r | L353<142 95136~95137(타워 배열 인덱스 line<<5) · get_start_position self 인자(&self.line, 95239) · minion_state switch(95346) · lane_path 인자(95470) · L144 target_bush_v41 인자(95508~95509) | 4 | OK |  |
| 2 | LineGankerPlan(self) | 0x20 | wait_limit | r | L218 97782 `wait_limit.saturating_sub(tick)` | 4 | OK |  |
| 3 | LineGankerPlan(self) | 0x10 | chats.len | r | L135 97001~97002 push 전 len 읽기(cap 과 비교 97006~97007) | 4 | OK |  |
| 4 | LineGankerPlan(self) | 0x0 | chats.cap | r | L135 97006 | 4 | OK |  |
| 5 | LineGankerPlan(self) | 0x8 | chats.ptr | r | L135 97015~97016 | 4 | OK |  |
| 6 | PlayerState | 0x930 | info.team | r | L106 95063~95064 (bounds<2) · 적팀 = 1-team(95104) | 4 | OK |  |
| 7 | PlayerState | 0x9c0 | info.position@tag (i32, as_index) | r | L106 95074~95076 · L188 get_die_tick_player pos 인자(97528) | 4 | OK |  |
| 8 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | 95077 | 4 | OK |  |
| 9 | OperationData | 0x8 | context (&GameContext) | r | 95176 · 95540 | 4 | OK |  |
| 10 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | 95343 · 95969 | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame: +0 data · +8 vtable) | r | 95101~95103 · vtable 슬롯 +0xf8 is_visible(95112~95114) · +0x1f0 get_entity_by_id(95389~95391) · +0x28 tick(97784~97785) — divtable AbstractGame | 3 | OK |  |
| 12 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] ([[Option<&Entity>;5];2], 원소 8B) | r | L106 95083~95086 (unwrap 95118) · L113 아군 5슬롯 언롤(95584 …) · L115 적 5슬롯(95960) · L190 97537~97538 · 클로저 캡처 | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x180 | top_tower/mid_tower/bottom_tower[team] (line<<5 로 선택, +0x180+line*32) | r | L353<142 95142~95147 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 14 | AbstractGameWithCache | 0x190 | top_tower2/mid_tower2/bottom_tower2[team] (+0x190+line*32) | r | L353<142 95148~95152 tower.or(tower2) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 15 | AbstractGameWithCache | 0x130 | twin_towers[team] (bumpalo Vec: ptr@0 · len@+0x18) | r | L354<142 95157~95164 min_by_key(dist² to line 시작점) | 4 | OK |  |
| 16 | AbstractGameWithCache | 0x170 | nexus[team] | r | L360/363<142 95308~95310 · 95323~95325 | 4 | OK |  |
| 17 | GameContext | 0x8 | setting (&GameSetting) | r | get_start_position 인자 95177~95178 | 4 | OK |  |
| 18 | GameContext | 0x20 | map (&MapDef) | r | lane_path 95468~95470 · L148 bushes 95527~95528 | 4 | OK |  |
| 19 | GameContext | 0x3b | debug (bool) | r | L150 95534~95536 · L199 97580 재사용 | 4 | OK |  |
| 20 | GameSetting | 0x12f8 | tick_per_second | r | L219 97805~97807 ×3 | 4 | OK |  |
| 21 | MapDef | 0x1c98 | bushes[30][30] (usize, [y][x] 순, stride 240B/8B) | r | L148 95529~95532 champ_bush = bushes[min(y/32000,29)][min(x/32000,29)] | 4 | OK |  |
| 22 | Blackboard[team] | 0x0 | top/mid/bottom_minion_state.front_minion (Option<usize>; line 별 +0/+0x28/+0x50, tag +0 · 값 +8) | r | L365~366<142 95344~95368, 95384~95385 (인덱스 %52 = 자기 팀) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 23 | Blackboard[1-team] | 0x0 | (&self 로 is_recent_visible 에 전달) | r | L117 96020 · L132 · L124 · L232 · closure#8 — 전부 blackboard[1-team](적팀 슬롯)이 self | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 24 | Entity | 0x5c0 | id | r | champ.id is_visible 키(95109~95110) · 적 id → make_gank_battle target_id(97041, 97797, 97821, 97889, 97895, 98314) · Chat 페이로드(96987~96988) · closure#11 자기판정 | 4 | OK |  |
| 25 | Entity | 0x660 | x | r | dist² 계산 전역(95249, 95542, 95604 …) · L148 x/32000 · L151 add_text x | 4 | OK |  |
| 26 | Entity | 0x668 | y | r | dist² · L148 y/32000 · L151 y+20000 | 4 | OK |  |
| 27 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | L112/L119 hp*100/max_hp (95633, 96078) — 0 이면 div_by_zero 패닉(95648, 96093) | 4 | OK |  |
| 28 | Entity | 0x670 | hp | r | L112/L119 95639~95641, 96084 | 4 | OK |  |
| 29 | Entity | 0x640 | stat_cached.move_speed | r | L193 97563~97564 (0 이면 div_by_zero 97574) | 4 | OK |  |
| 30 | Entity | 0x4c0 | attack_effect@tag (i32 -1 = None) | r | L237 98229~98231 unwrap(98252) | 4 | OK |  |
| 31 | Entity | 0x490 | attack_effect@Some.0 (Effect, &self → range_adjust) | r | L237 98235, 98247 | 4 | OK |  |
| 32 | Entity | 0x4a0 | attack_effect.range | r | Effect::range 인라인(effect.rs:26) 98238~98239 | 4 | OK |  |
| 33 | Entity | 0x4a8 | attack_effect.growth_range | r | 98240~98241 ×(level-1) | 4 | OK |  |
| 34 | Entity | 0x5c8 | level | r | 98242~98243 | 4 | OK |  |
| 35 | Entity | 0x438 | stat_buff_cached.range | r | 98244~98245 | 4 | OK |  |
| 36 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | Entity::radius 인라인(entity.rs:1511~1515) 98258~98261 · 98281~98284 (champ·적 각각) | 4 | OK |  |
| 37 | Entity | 0x680 | radius | r | 98265~98266 · 98272~98276 (mult==0 이면 그대로, 아니면 radius*(mult+100)/100) | 4 | OK |  |
| 38 | DebugFrameData | 0xa0 | infos (HashMap<usize,Vec<String>>) | r | L200 97407(%901) rustc_entry 97590 | 4 | OK |  |
| 39 | LanePath [(u64,u64);7] | 0x30 | [3] (lane_path 결과의 4번째 점, DI 이름 center) | r | L369<142 95471~95475 (+48 x, +56 y) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 40 | check_kill 반환 Option<(usize,usize)> 24B | 0x0 | tag(+0 bit0) · +8 e(target id) · +0x10 t(min_trace) | r | L225 97998~97999 · 98175~98179 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 41 | LineGankerPlan(self) | 0x0 | chats (Vec<Chat> 24B 원소) | w | L135 만: cap==len 이면 grow_one(%1)(97011, ptr/cap 갱신) → 원소 store(97021~97025) → len(+0x10) = len+1 store(97027). initializes 속성 없음 · self 의 다른 필드 쓰기 0건(전 본문 grep) | 4 | OK | push(Chat::BattleHelp(nearest_enemy.id, 0)) — 태그 i8 6 @+0 · +8 = enemy.id · +0x10 = 0 |
| 42 | sret Option<BigPlan> | 0x0 | tag / 페이로드 | w | L136 은 BattlePlan::new sret %45 → +263(0x107 entry_src)=3 store(97032~97033) 후 memcpy | 4 | 확인불가(tcx 사전에 타입 없음) | -1(None: L102 95059 · L155 97158 · L245 98207) · 9 + BattlePlan 280B(L136 97034~97036) · make_gank_battle 결과(콜리가 채움, 7곳) |
| 43 | DebugFrameData(debug) | 0x0 | texts (add_text) | w | context.debug 일 때만(97054~97083) | 4 | OK | L151: add_text(champ.x, champ.y+20000, format!("bush: {:?}, champ_bush: {:?}"), Color(1,0,0,1), 1) |
| 44 | DebugFrameData(debug) | 0xa0 | infos[e_ent.id] push | w | context.debug 일 때만(97590~97734) · entry(id).or_default() 로 없으면 빈 Vec 삽입(97606~97624) | 4 | OK | L200: format!("target: {:?}, near_allies_p: {:?}, near_enemies_p: {:?}, move_to_tower_tick: {}, me_die_tick: {}, enemy_die_tick: {}") |
| 45 | 로컬 bumpalo Vec<usize> ×4 (%37 near_allies_p · %35 near_enemies_p · %32/%30 루프 내부 동명) | 0x0 | from_iter_in 결과(32B 헤더: ptr@0 · bump@+8 · cap@+0x10 · len@+0x18) | w | 루프 내부 것은 매 반복 drop(97841 …), 바깥 것은 반환 직전 drop(97936, 98327) | 4 | 확인불가(tcx 사전에 타입 없음) | L163/L168/L175/L182 — 원소 = 포지션 인덱스(usize) |

**`consts` 상수 19건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 6 | 101 | 센티널 | LineGankerPhase 니치 시작(WaitResponse=6/Setup=7/Cancel=8; tag<6 = ChangeJungle 암묵 variant, tcxdict --enum LineGankerPhase). `icmp ult i8 tag, 6` → ChangeJungle 이면 None(95054). ⚠같은 값 6 이 L135 Chat::BattleHelp 메모리태그로도 쓰인다(97021, tcxdict --enum Chat 6) | 3 |
| 1 | 2 | 106 | 태그 | team 배열 bounds(<2, 95066) · L136 BattlePlanGoal::Response 태그 2(97029, tcxdict --enum BattlePlanGoal idx2→2, Direct) | 3 |
| 2 | 62500000001 | 111 | 임계 | 250000² + 1 — `dist² < 62500000001` ⇔ 거리 ≤ 250000(7.8셀). L111 아군(95629) · L118 적(96074) 광역 인원 집계 반경 | 4 |
| 3 | 100 | 112 | 계수 | hp*100/max_hp 백분율(95641, 96086). L237 radius*(mult+100)/100 에도 100(98274~98276) | 4 |
| 4 | 39 | 112 | 임계 | `hp% > 39` ⇔ HP 40% 이상만 인원 집계(95643 아군 · 96088 적) | 4 |
| 5 | 3 | 136 | 계수 | BattlePlan.entry_src(+0x107) = 3 (97033) — 갱커 노출·열세 대응 진입 표식. L219 tps*3(97808)에도 3 ⚠QC shl 경고 사유: 본문 `shl i64 %98, 3`(95173)은 슬라이스 원소 8B stride(ptr 산술)이고 이 상수와 무관 — 시프트량 아님 | 4 |
| 6 | 9 | 136 | 태그 | BigPlan::Battle 메모리태그(tcxdict --enum BigPlan idx7→9, 97034) | 3 |
| 7 | 60 | 127 | 미상 | make_gank_battle 의 min_trace 인자(DI 이름, m08 94354 정의) — L127·212·214·217·221·241 여섯 곳 리터럴 60(=1초@60tps 추정, 콜리 명세 참조). L227 만 check_kill 이 준 t 를 넘긴다 | 4 |
| 8 | 32000 | 148 | 계수 | 셀 크기 — champ.x/32000, y/32000 으로 bushes 그리드 셀 좌표(95515, 95521) | 4 |
| 9 | 29 | 148 | 인덱스 | bushes[30][30] 인덱스 clamp(umin 29, 95519, 95526) | 4 |
| 10 | 20000 | 151 | 계수 | 디버그 텍스트 y 오프셋(champ.y+20000, 97055) | 4 |
| 11 | 150000 | 158 | 산출값 | dist_cut — 근접 인원 집계 반경(4.7셀). 클로저#6/#8/#11/#13 에서 `dist² <= dist_cut²` 로 소비(캡처 &dist_cut). 본문엔 store i64 150000(97091) | 4 |
| 12 | -1 | 187 | 태그 | Option<SmallAction>::None 태그(97512 · L188 도 같은 -1 alloca %27) → get_die_tick_player 6번째 인자. Option<BigPlan>::None(-1) 판정에도 같은 값(97045, 97828, 97833, 97902, 97907, 98186, 98321) | 4 |
| 13 | 99999999 | 195 | 산출값 | 적 타워가 없을 때 move_to_tower_tick 기본값(97578) — 사실상 무한(L209 `<=120` 게이트 항상 실패 → L216 분기로) | 4 |
| 14 | 30 | 205 | 임계 | `enemy_die_tick > 30`(97584) — 적을 30틱(0.5초@60) 안에 못 죽이면 인원우위(allies.len > enemies.len) 없을 때 이 적은 건너뜀 | 4 |
| 15 | 121 | 209 | 임계 | `move_to_tower_tick < 121` ⇔ ≤120틱(2초@60): 적이 자기 타워까지 2초 안이면 빠른 킬 분기(L211~215), 아니면 L216 분기(97739). 표기(`<=120` vs `<121`)는 외연 동일이라 표기 불가 | 4 |
| 16 | 61 | 211 | 임계 | `enemy_die_tick < 61` ⇔ ≤60틱(1초) 안에 죽일 수 있으면 인원비 무관 갱 시도(97778) | 4 |
| 17 | 120 | 216 | 임계 | `me_die_tick > 120`(97774) — 내가 2초 이상 버티고 아군≥적 이면 갱 시도(L217) | 4 |
| 18 | 5 | 190 | 임계 | player_champion[1-team][*e] bounds(<5, 97533) · closure#11/#13 동일 ⚠QC shl 경고 사유: `shl i8 %82, 5`(95142)는 line*32(top_tower..bottom_tower2 배열 stride)이며 이 상수와 무관 | 4 |

**`knobs` 조정점 10건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 광역 인원 집계 반경(노출 시 우위 판정) | ganker.rs:111·118 (m08.ll:95629, 96074) | 62500000001 | 올리면(예: 300000²+1) 더 먼 아군·적까지 세어 인원우위/열세 판정이 넓은 범위로 바뀐다 · 내리면 바로 옆 인원만 센다. 아군·적 두 곳을 같이 바꿔야 대칭이 유지된다 | 4 | 기존 |
| 1 | 인원 집계 HP 하한(%) | ganker.rs:112·119 (95643, 96088) | 39 | 올리면(예: 59) 저체력 챔피언을 인원에서 더 많이 제외해 우위 판정이 보수적으로 · 내리면(0) 빈사도 인원으로 센다 | 4 | 기존 |
| 2 | 근접 인원·킬계산 반경 dist_cut | ganker.rs:158 (97091) | 150000 | 올리면 near_allies_p/near_enemies_p 에 더 먼 챔피언이 들어가 get_die_tick_player 의 공격자/피격자 집합이 커지고(die_tick 감소) 인원비 게이트도 넓어진다 · 내리면 갱 대상 후보(바깥 near_enemies_p) 자체가 줄어 루프가 짧아진다 | 4 | 기존 |
| 3 | 적 처치 예상 틱 상한(인원우위 없을 때) | ganker.rs:205 (97584) | 30 | 올리면 더 오래 걸리는 킬도 인원비 무관하게 후보로 남는다(공격적) · 내리면 즉사급만 후보(소극적) | 4 | 기존 |
| 4 | 적의 타워 복귀 시간 임계(틱) | ganker.rs:209 (97739, `<121`) | 121 | 올리면 타워에서 먼 적도 '빠른 킬' 분기(L211~215)로 들어가 60틱 킬 조건이 적용 · 내리면 대부분 L216 분기(내 생존 120틱 + 아군≥적) 로 흘러간다 | 4 | 기존 |
| 5 | 빠른 킬 판정 틱(인원비 무관 갱) | ganker.rs:211 (97778, `<61`) | 61 | 올리면 인원 열세여도 갱을 시도하는 범위가 넓어진다 · 내리면 인원우위(L213) 조건으로만 갱 | 4 | 기존 |
| 6 | 내 생존 예상 틱 하한 | ganker.rs:216 (97774) | 120 | 올리면 내가 더 안전할 때만(적 타워 멀 때) 갱 · 내리면 위험해도 갱 | 4 | 기존 |
| 7 | 대기 잔여시간 유예(초) | ganker.rs:219 (97808, tps*3) | 3 | 올리면 wait_limit 까지 여유가 많을 때 '아군우위 갱'(L220~221) 을 더 오래 미룬다 · 0 이면 항상 L220 판정으로 | 4 | 기존 |
| 8 | make_gank_battle min_trace | ganker.rs:127·212·214·217·221·241 (리터럴 60, 6곳) | 60 | 콜리 make_gank_battle 명세의 min_trace 소비 방식에 따름(추정: 추적 최소 틱). L227 만 check_kill 이 준 t 를 쓰므로 여기 값을 바꿔도 그 경로는 불변 | 5 | 기존 |
| 9 | 노출·열세 시 응답 플랜 진입 표식 | ganker.rs:136 (97033, entry_src=3) | 3 | BattlePlan.entry_src 는 텔레메트리/후속 판정 표식 — 값 변경은 BattlePlan 소비처(battle.rs)에 따라 효과가 달라진다(이 함수에선 저장만) | 4 | 기존 |

<details><summary>`callees` 피호출자 35건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_text | game_core::DebugFrameData::add_text | pub | fn(&mut game_core::DebugFrameData, u64, u64, std::string::String, common::color::Color, usize) | game-core\src\simulation\game\frame.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check_kill | game_ai::plan_legacy::handler::check_kill | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<(usize, usize)> | game-ai\src\plan_legacy\handler.rs:2507 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | get_die_tick_player | game_ai::get_die_tick_player | pub | fn(&game_core::OperationData, usize, usize, &bumpalo::collections::vec::Vec< usize>, std::option::Option<&game_core::Entity>, std::option::Option<game_core::SmallAction>) -> u64 | game-ai\src\goal_data.rs:577 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | is_in_jungle_area | game_core::is_in_jungle_area | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:66 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 11 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | lane_path | game_core::MapDef::lane_path | pub | fn(&game_core::MapDef, game_core::LineType, usize) -> [(u64, u64); 7_usize] | game-core\src\simulation\map_def.rs:174 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | make_gank_battle | game_ai::plan_legacy::old::LineGankerPlan::make_gank_battle | in:game_ai::plan_legacy::old::line_gank::ganker | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> | game-ai\src\plan_legacy\old\line_gank\ganker.rs:76 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | near_jungle_bush | game_core::near_jungle_bush | pub | fn(&game_core::GameContext, u64, u64, u64, u64) -> std::option::Option<(u64, u64)> | game-core\src\simulation\map_regions.rs:84 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | new | game_ai::plan_legacy::old::BattlePlan::new | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan | game-ai\src\plan_legacy\old\battle.rs:196 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 21 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 22 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 23 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 24 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 25 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | target_bush | game_ai::plan_legacy::old::LineGankerPlan::target_bush | in:game_ai::plan_legacy::old::line_gank::ganker | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> (u64, u64) | game-ai\src\plan_legacy\old\line_gank\ganker.rs:351 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 27 | target_bush | game_ai::plan_legacy::old::LineGankCoverPlan::target_bush | in:game_ai::plan_legacy::old::line_gank::cover | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> (u64, u64) | game-ai\src\plan_legacy\old\line_gank\cover.rs:196 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 28 | target_bush_v41 | game_ai::plan_legacy::old::LineGankerPlan::target_bush_v41 | in:game_ai::plan_legacy::old::line_gank::ganker | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize | game-ai\src\plan_legacy\old\line_gank\ganker.rs:310 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 30 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 31 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 32 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 33 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 34 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 12개**: `distance2`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `entry`, `enumerate`, `fastcc`, `format_inner`, `grow_one`, `insert_no_grow`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `or_default`, `rustc_entry`, `usize`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:8655) · **형제 14개** (LineGankerPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::LineGankerPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:9 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan) -> game_ai::plan_legacy::old::LineGankerPlan |
| 1 | <game_ai::plan_legacy::old::LineGankerPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:9 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::LineGankerPlan::new | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:27 | True | fn(game_core::LineType, usize, usize) -> game_ai::plan_legacy::old::LineGankerPlan |
| 3 | game_ai::plan_legacy::old::LineGankerPlan::new_with_phase | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:31 | True | fn(game_core::LineType, usize, usize, game_ai::plan_legacy::old::LineGankerPhase) -> game_ai::plan_legacy::old::LineGankerPlan |
| 4 | game_ai::plan_legacy::old::LineGankerPlan::goal | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:35 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan) -> game_core::BigGoal |
| 5 | game_ai::plan_legacy::old::LineGankerPlan::update | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:39 | False | fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 6 | game_ai::plan_legacy::old::LineGankerPlan::is_end | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:64 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 7 | game_ai::plan_legacy::old::LineGankerPlan::is_cancel | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:69 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan) -> bool |
| 8 | game_ai::plan_legacy::old::LineGankerPlan::make_gank_battle | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:76 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 9 | game_ai::plan_legacy::old::LineGankerPlan::next_plan | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:99 | False | fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 10 | game_ai::plan_legacy::old::LineGankerPlan::target_bush_v30 | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:248 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize |
| 11 | game_ai::plan_legacy::old::LineGankerPlan::target_bush_v41 | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:310 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize |
| 12 | game_ai::plan_legacy::old::LineGankerPlan::target_bush | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:351 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> (u64, u64) |
| 13 | game_ai::plan_legacy::old::LineGankerPlan::sub_plan | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:379 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |

**`open` 9건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L142 `self.target_bush(player, data)` 인라인 결과(near_jungle_bush 의 (u64,u64))가 어떤 변수에도 안 쓰인다(%18 재로드 0건) — 소스가 `let _ = …` 인지, 결과를 쓰는 코드가 dead-store 제거된 것인지 IR 로는 구분 불가(표기 불가). 동작상으로는 unwrap 패닉(부시 못 찾음) 부작용만 남는다 | 4 |  |
| 1 | 미탐색 | is_recent_visible 의 self 가 왜 blackboard[1-team](적팀 슬롯)인지 — IR 사실(96020·113034·14604 전부 1-team). Blackboard.last_visible[pos] 가 '그 팀 챔피언의 피관측 기록'이라면 자연스럽지만 근거는 game_core blackboard.rs:346 본문 미독(범위 밖). 재현 시 인덱스만 그대로 따르면 됨 | 4 |  |
| 2 | 미탐색 | L154 `champ_bush != bush → None` 의 bush 값 의미(bushes 그리드의 id 체계, 0 이 '부시 아님'인지) — MapDef.bushes 채움 규칙은 map_def 계층 미독. target_bush_v41 내부(94136~)도 본 배치 범위 밖(시그니처만) | 4 |  |
| 3 | 표기 불가 | L191 `if let Some(tower)` 의 Some/None 판정은 %900(=nearest_enemy_tower==null) 로 루프 밖에서 한 번 계산돼 재사용된다(97406) — 소스 표기(`match`/`map_or`)는 표기 불가 | 4 |  |
| 4 | 표기 불가 | L218~219 조건의 소스 표기: IR 은 `usub.sat(wait_limit, tick) > tps*3` 참이면 아무 것도 안 하고 다음 e 로 간다. `else if cond {}` 인지 `else if !cond && allies>enemies` 인지 외연 동일 — 표기 불가 | 4 |  |
| 5 | 미탐색 | column 정보 부재로 같은 줄 안의 `&&` 좌우 순서(L107·L122·L205·L216)는 분기 방향으로만 복원했다(첫 항 거짓이면 둘째 항 미평가) | 4 |  |
| 6 | 미탐색 | L60 Color 생성(1.0,0.0,0.0,1.0)은 common::color::Color::new 인라인 추정 — 필드 순서 RGBA 는 float 4개 store 순서로만 판단 | 5 |  |
| 7 | 미탐색 | make_gank_battle · check_kill · get_die_tick_player · BattlePlan::new 내부의 TLS 사용 여부는 각 자식 명세 소관(본 함수 직접 접점 0) | 4 |  |
| 8 | 미탐색 | L151 Color(1.0,0.0,0.0,1.0)(97076~97082, float store)은 qcspec 이 실수 리터럴을 못 잡아 constants 에서 뺐다(writes 에 기재) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

