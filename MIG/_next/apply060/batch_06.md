# apply060 batch_06.md — 반영(logic_060) 브리핑 · 함수 9

★**version = 3 런타임 확정(09-17)** — v<3/v2 경로는 명세에서 제외(사장). 태그/오프셋/vt 슬롯 = `spec_patch_060.md` §A(dispcheck 추가 행 포함). 출력 형식·절차 = `mkapply060.py` 도크스트링. 출력 = `_next/apply060/out/<old>.md`.

RE 정본 파일: 2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md · 2026-09-17_r20_변경99_배치G_누락7_소폭오프셋만_디컴대조_원문.md · 2026-09-17_r21_심층_w3_정글3_JungleSubPlan_PassiveJungle_역정글매복_원문.md · 2026-09-17_r21_심층_w9_serpen_epic_passive_plan_HuntAndPoke_sub_plan_ObjContest헬퍼_원문.md

### `d28800` → `ec7180` PassiveLinePlan::update (i=206 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\passive_line.rs:219` · one_line: 라인 패시브 플랜 매 틱 갱신 — L222 check_recall 로 self.in_recall(귀환 여부) 결정(배치 H) → 팀 목표 Gank 라인 보정·v46 라인귀환 stage1/2·도주 게이트(update_v46_lane_recall/update_v46_flee 인라인, L232~248 · 배치 I) → 라인 위치·채팅 push(L254~304 · 배치 J). 반환 없음(unit) — 출력은 &mut self 필드뿐.
- 0.6.0 판정: **다건** · 패치 요지: ①check_recall v3 undying 게이트 ②v46 bound = 1416fbbc0(특성) ③L232 Gank 게이트 v≥2 bb Vec(+0x250) 조회 → 없으면 cd5==8 && cd6==line
- RE 정본: `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` §4
- sig: `fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData)`
- consts: [{"value": 100, "src_line": 1084, "meaning": "hp_ratio = hp*100/max_hp (19260) · 아군/적 hp% 합(20755·20830) · item_score 100(4티어, 21039)", "kind": "계수", "ev": 4}, {"value": 1, "folded_from": 2, "src_line": 1095, "meaning": "tps*2 (= 2초) 가 `shl i64 %220, 1`(19506) 로 접힘 — enemy_minion_line_action_danger_damage_at 의 예측 틱 인자", "kind": "임계", "ev": 4}, {"value": 46, "src_line": 1096, "meaning": "hp_ratio < 46 이면 웨이브 피해(≠0)만으로 귀환 확정 (19513 `icmp ult i64 %120, 46` · 19514 `icmp uge %222, %114` 와 or)", "kind": "임계", "ev": 4}, {"value": 62500000001, "src_line": 1157, "meaning": "250000²+1 — 쌍둥이 타워↔프론트 미니언 
- 0.5.8 logic 전문:
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
// L1085  can_buy_item = buy_item(version, rnd, player, cache.game.{data,vtable}, ctx) -> {i64,i64}, .0==1 이 Some   (19263~19272)
// L1086  upgrade_item(sret 24B: +0 Option 태그 i64, +0x10 idx)(version, rnd, player, game, ctx); 이후 L1161 map 으로 (Some 여부 %127, idx %129) 만 씀  (19275~19279)
// L1087  if !in_fountain {                        // NA(version<2 전용): in_fountain=true 이면 %130 으로 웨이브 검사 생략(19285~19288)
// L1090    fm = blackboard[team].minion_state(self.line).front_minion.and_then(|id| game.get_entity_by_id(id))  (19439~19479; minion_state 스위치 blackboard.rs:378~382 top+0/mid+0x28/bottom+0x50, vtable +0x1f0)
// L1092    (mx,my) = fm.map(|e| e.pos).unwrap_or(champ.pos)                                          (19484~19499)
// L1094    minion_wave_damage = enemy_minion_line_action_danger_damage_at(version, data, champ, mx, my, tps*2, false, false)   (19502~19507)
// L1096    if minion_wave_damage != 0 && (hp_ratio < 46 || minion_wave_damage >= champ.hp): return true   (19509~19516 → %886 phi=1)
//        }
// L1101  tower = cache.<line>_tower[team].or(cache.<line>_tower2[team])   (simulation.rs:1823 tower(line,team); 19301~19311)
// L1102~1109 nearest_tower = tower.or( twin_towers[team].iter().min_by_key(|t| dist_sq(LineType::get_start_position(&self.line, ctx.setting, team), t.pos)).map(|t| *t) ).or(cache.nexus[team]).unwrap()   (19316~19436 · aux m12 27076~ · unwrap 실패 19553)
// L1111  nexus = cache.nexus[team].unwrap()   (19558~19567, 실패 19585)
// L1113  front_minion = blackboard[team].minion_state(line).front_minion.and_then(get_entity_by_id)  → %60   (19574~19621)
// L1116  target = if front_minion.is_none_or(|fm| dist_sq(fm,nexus) < dist_sq(nearest_tower,nexus)) { nearest_tower } else { fm }  → %59   (19626~19697: 미니언이 타워보다 넥서스에 가까우면 타워, 아니면 미니언)
// L1122~1126 near_allies: Vec<&Entity> = cache.player_champion[team].iter().enumerate()
//            .filter(|(i,e)| *i != my_pos_index(L1124) && target.is_some() && e.is_some() && dist_sq(e, target) < 150000²+1 (L1125))
//            .filter_map(|(_,e)| *e).collect_in(ctx.pool)     (19706~19722 · aux m01 23056~23313)
// L1128~1134 has_self_heal = champ.stat_buff_cached.vamp > 0
//            || champ.skill_effect.as_ref().map(|e| expected_heal_target(e,ctx,champ,_,champ,_) != 0 || effect_buff_target(version,e,ctx,champ,_,champ,_).map(|b| b.vamp>0)==Some(true)).unwrap_or(false)
//            || champ.skill2_effect()(level>2) 동일 검사     (19724~19830, phi %360)
// L1136~1147 near_enemies: Vec<&Entity> = cache.player_champion[1-team].iter().enumerate()
//            .filter(|(i,e)| e.is_some_and(|e| blackboard[1-team].is_recent_visible(game, player, e) (L1139)
//                     && !(e.ty is Champion && action_state is Return) (L1140 — 귀환 중인 적 제외)
//                     && ( blackboard[1-team].in_big_line(*i, self.line) (L1141)
//                        || front_minion.or(target).is_some_and(|p| dist_sq(e,p) < 250000²+1) (L1145)
//                        || target.is_some_and(|t| dist_sq(e,t) < 150000²+1) (L1146) )))
//            .filter_map(|(_,e)| *e).collect_in(ctx.pool)     (19833~19872 · aux m01 23316~23686)
// L1154  if hp_ratio <= 70 {                                                                       (19879: `ugt 70` 참이면 L1171 로)
// L1155    if tower(line,team) is None(1·2차 모두 없음) {                                            (19897~19905)
// L1156      if front_minion.is_some_and(|fm| twin_towers[team].iter().any(|t| dist_sq(fm,t) < 250000²+1)) {   (19908~19992, closure$13)
// L1160        if ctx.debug { debug.add_log(data, player, format!("V46BASEHEAL T{team} {position:?} tick={game.tick()} hp={hp_ratio}%")) }   (19996~19999 · 21140~21183)
// L1162        return true   (→ %567 → %901 → phi %885=1)
//    } } }
// L1171  tick = game.tick()(vtable+0x28). if hp_ratio < 51 && is_line_phase(tick) [= tick < setting.epic_jungle.first_spawn_tick.saturating_sub(tps*30)] {   (19883~20014)
// L1172    if let Some(fm) = front_minion {                                                          (20017~20019)
// L1174      lane_enemy_count = near_enemies.iter().filter(|e| dist_sq(e,fm) < 150000²+1).count()   (20063~20141, closure$14)
// L1177      if lane_enemy_count != 0 && near_allies.len()+1 <= lane_enemy_count {                    (20142~20151: `len+1 > count` 면 탈출)
// L1179        if near_enemies.iter().any(|e| dist_sq(e,fm) <= 150000² && combo_dmg(e→champ) >= champ.hp) {   (20153~20293, closure$15: combo = attack_effect + skill_effect + skill2(level>2) + ult(level>4) 각 expected_damage_target(eff,ctx,e,_,champ), L1184~1196)
// L1201          can_we_kill_any = near_enemies.iter().filter(|e| dist_sq(e,fm) < 150000²+1).any(|e| [champ 콤보뎀(e 대상) + Σ near_allies 콤보뎀(e 대상)] >= e.hp)   (20300~20341 · aux m04 13030~13349, L1202~1231)
// L1234          if !can_we_kill_any { return true }                                                  (20347: 참이면 %572=L1242, 거짓이면 %567→true)
//    } } } }
// L1242  nearest_enemy_tower = tower(line, 1-team).or(twin_towers[1-team].iter().next().map(|t| *t)).or(cache.nexus[1-team]).unwrap()   (20372~20434, 실패 20444 → unreachable %373)
// L1247  if nearest_enemy_tower.ty is Tower(태그 2) {                                                 (20438~20441)
// L1248    if let Some(t) = champ.return_time() [ty Champion(13) && action_state Return(1) → Return.time] { remain_return_time = setting.return_tick.saturating_sub(t)   (20449~20627)
// L1251      if remain_return_time <= tps { return true } }                                          (20630~20631: `remain > tps` 면 계속)
// L1255    if tower.nearest_enemy.is_some() (Entity+0x88 태그≠0) {                                    (20635~20638)
//            if hp_ratio > 70 || near_enemies.is_empty() { (L1261) if hp_ratio < 31 { return true } }   (20642~20646 · 20676~20677)
//            else { (L1256) if blackboard[team].minion_state(line).minion_count > 1 || hp_ratio < 31 { return true } }   (20650~20672)
//    } }
// L1266  has_ally_heal = has_self_heal || near_allies.iter().any(|a| a.vamp>0 || a.skill_effect 힐/뱀프버프 || a.skill2_effect() 힐/뱀프버프)   (20460~20686, closure$19 — 자기 검사와 같은 식, 대상 champ)
// L1276  line_strategy = f(player.info.position == Jungle)  — i8 값 자체는 DI poison, 비교 %702(20688)만 생존. 아래 트리에서 `is_jungle` 로 표기
// L1284  is_more_enemy = near_enemies.len() > near_allies.len()+1   (20694~20695)
// L1285  is_more_ally  = near_allies.len()+1 > near_enemies.len()   (20698 = `ule` 의 부정 — dbg_value DW_OP_not 확인 · 분기도 그 극성)
// L1286  enemy_hp_ratio_sum = Σ_{e∈near_enemies} e.hp*100/e.max_hp + 10   (20723~20773, closure$20; max_hp 0 → panic 20746)
// L1287  ally_hp_ratio_sum  = Σ_{a∈near_allies} a.hp*100/a.max_hp + hp_ratio   (20798~20845, closure$21)
// L1289  ally_has_line_lead  = has_line_lead(version, rnd, data, team,   line, debug)   (20847)
// L1290  enemy_has_line_lead = has_line_lead(version, rnd, data, 1-team, line, debug)   (20852)
// L1293~1392 heal_score (phi 20890) =
//   if has_ally_heal (20857):
//     if is_jungle (20860): if is_more_enemy (20925): L1317 hp<51 ? 80 : 0   else (L1322) if !is_more_ally: L1325 hp<21 ? 70 : (L1327 hp<31 ? 60 : 0)  else 0
//     else (L1297): if is_more_enemy: L1299 hp<21 ? 80 : (L1301 hp<31 ? 70 : (L1303 hp<41 ? 60 : 0))  else 0
//   else (L1336):
//     if is_jungle (20863): if is_more_enemy (L1367): L1368 hp>80 ? 0 : 80   else (L1373) if !is_more_ally: L1380 hp<41 ? 70 : (L1382 hp>60 ? 0 : 60)  else L1374 hp<31 ? 80 : 0
//     else (L1338): if is_more_enemy: L1340 hp<31 ? 80 : (L1342 hp<51 ? 70 : (hp>70 ? 0 : 60))
//                  else (L1349) if !is_more_ally && ally_hp_ratio_sum < enemy_hp_ratio_sum: L1356 hp<21 ? 70 : (L1358 hp<31 ? 60 : 0)  else 0
// L1392~1405 item_score (phi 21039) = if can_buy_item.is_none() && upgrade_item.is_none(): 0
//     else if let Some(idx)=upgrade_item: match ctx.item_list[idx].tier() { 4→100, 3→70, 2→60, _→30 }  (L1395~1400, bounds 실패 21014)
//     else (can_buy Some, upgrade None): player.info.items.is_empty() ? 5 : 0  (L1405)
// L1411  item_score = if ally_has_line_lead { item_score } else if enemy_has_line_lead { 0 } else { item_score/2 }   (21041~21044)
// L1422  panelty_score = if ally_has_line_lead { 0 } else if enemy_has_line_lead { -10 } else { -5 }   (21043·21045)
// L1434  score = heal_score + panelty_score + item_score   (21048~21049)
// L1437  is_epic = game.get_game_mode()(vtable+0x40).as_moba().map_or(false, |m| m.jungle_runner.epic.live_list.len() != 0)   (21051~21082)
// L1439  if hp_ratio < 11 || (hp_ratio < 61 && is_epic): return true   (21084~21088)
// L1447  return score > 49   (21122~21124)
// (drop: near_enemies %56 → 21092/21106, near_allies %58 → 21112/21129; 언와인드 정리 %336/%380)
// → 결과 i8 는 %886(21136)→%904(21197) 로 합류: self.in_recall = 결과 (21200~21201) → 이후 L232 부터 배치 I(%904 나머지)

// passive_line.rs:232~248 (배치 I)
// 진입 = 블록 @904(21197) — 배치 H 의 L222 `self.in_recall = check_recall(..)`(21200~21201) 직후.
// 공통: %64=team %68/%69=position %70=cache %73=&cache.player_champion[team][pos] %79=ctx (배치 H 로드)

// ── L232~241 (update 본문) ──
L232: tag = team_plan.objective@tag(+0x41f);  if (tag & ~1) == 8 {           // Some(Gank{line}) | Some(Dive{line})   (21202~21206)
L233:   gank_line = team_plan+0x420
L236:   if gank_line == self.line(+0x116) {                                   // LineType 1B 비교 (21213~21223)
L237:     champ = *%73;  gank_hold = champ.is_some_and(|x| x.hp(0x670)*100 / max(x.stat_cached.hp(0x628), 1) > 24)   // closure$0 (21226~21245)
L239:     if gank_hold { → @1365 }                                             // 아니면 아래 L243 로
        }
      }
// gank_hold == false (또는 Gank/Dive 아님 / 라인 불일치 / champ None) → @929 = L243

// ── L243  self.update_v46_lane_recall(version, player, data, debug)  (passive_line.rs:488~653 인라인 · @929~@1361) ──
:489 tick = cache.game.tick()                                                  // vtable+0x28 → %44
:490 champ = *%73;  if None → :491 v46_clear(completed=true) → :492 return    // v46_clear = { v46_commit(0x111)=0; v46_committers.len(0x40)=0 } (completed=true 라 commit_clears 증가 없음)
:496 (rx, ry) = heal_area(team) = team==0 ? (64000, 960000) : (960000, 64000)
:497 in_heal = x∈[rx-68000, rx] && y∈[ry-64000, ry]   (saturating · 접힌 형태: team0: x<=64000 && 896000<=y<=960000 / team1: 891999<x<=960000 && y<=64000)
     if in_heal → :499 v46_clear(true); return
:498 if !setting.is_line_phase(tick)  [= tick < sat_sub(first_spawn_tick(0x8a8), tps(0x12f8)*30)] → :499 v46_clear(true); return
:503 tower = cache.tower(line, team)                         // <line>_tower[team].or(<line>_tower2[team]) 인라인
:504~509 twin = cache.twin_towers[team].iter().min_by_key(|t| t.distance_sq(line.get_start_position(setting, team))).map(|t| *t)   // closure$0 :506~507 · closure$1 :509
:510~511 nearest_tower = tower.or(twin).or(cache.nexus[team]).unwrap()        // unwrap_failed anon.75 (Location :511:8)
:513~514 front_minion = blackboard[team].minion_state(line).front_minion.and_then(|id| cache.game.get_entity_by_id(id))   // vtable+0x1f0 → %43
:517~526 near_enemies = Vec::from_iter_in(cache.player_champion[1-team].iter().enumerate()
           .filter(|(idx, c)| c.is_some_and(|c|                                // ★aux m01.ll:23689~24038 (closure s1_0 · DI closure$3)
:520            blackboard[1 - team].is_recent_visible(game, player, c)         //   ⚠인덱스 = 1 - player.team (IR 그대로)
:521            && !c.is_in_return()                                             //   entity.rs:1648 인라인 = !(ty==Champion(13) && action_state==Return(1))
:522            && ( blackboard[1 - team].in_big_line(idx, line)
:523              || front_minion.is_some_and(|f| c.distance_sq(f) < 62500000001)   //   <= 250000
:524              || c.distance_sq(nearest_tower) < 62500000001 ) ))
           .filter_map(|(_, c)| *c), ctx.pool)                                  // closure s2_0 = 단순 역참조(m04.ll:64249~64255)
:528 if self.v46_commit(0x111) {                                               // ── 커밋 유지 경로 ──
:533   if front_minion.is_none() → return (@1080: drop near_enemies)
:534   still_vec = v46_stage1(version, team, pos, cache, ctx, champ, front, nearest_tower, my_hp=champ.hp, range_gate=false, only=Some(&self.v46_committers[..]), near_enemies)
:535   still = still_vec.is_empty()   (still_vec drop)
:536   if still {                                                               // 커밋했던 위협이 전부 사라짐
:537     if ctx.debug(0x3b) { :538 debug.add_log(format!("\x08V46CLR T{team} {position:?} tick={tick} 상황해소(커밋 적 이탈/사망/억제역전)")) }   // anon.73
:541     v46_clear(completed=false)   // = { if v46_commit { commit_clears(0xb8) += 1 }; v46_commit=0; committers.len=0 }
       }
       return (@1080)
     }
:548 if front_minion.is_none() || near_enemies.is_empty() → return (@1080)
:554~555 committers = v46_stage1(version, team, pos, cache, ctx, champ, front, nearest_tower, my_hp=champ.hp, range_gate=true, only=None, near_enemies)   // Vec<(id, die_key, dps)>
:556 if committers.is_empty() → return (drop · @1097→@1322→@1080)
:561 survive = v46_stage2(team, cache, ctx, champ, front, &committers)
:562 if survive { :563 stage2_saves(0xb0) += 1; return }
:569 cs_lock = champ.attack_duration()
:570 exposed = committers.iter().any(|c| game.get_entity_by_id(c.0).is_some_and(|e|         // ★aux m04.ll:12086~12247
:572~574     e.distance(champ) <= e.attack_effect.map_or(0, |a| a.range(e)) + e.radius() + champ.radius() + e.move_speed(0x640)*cs_lock ))
:577 if exposed { :578 danger_ticks(0x90) += 1 }
:584 my_tower_protects = nearest_tower.attack_effect.map_or(false, |t| front.distance_sq(nearest_tower) <= (t.range(nearest_tower) + front.radius())²)   // ★aux m04.ll:584~679
:588 minion_power = blackboard[team].minion_state(line).minion_power(+0x18)
:592 lane_tower_gone = cache.tower(line, team).is_none()                       // 아웃오브라인 호출 21806
:593 base_heal_dominates = lane_tower_gone && champ.hp < champ.stat_cached.hp   // 타워 없고 피가 깎여 있으면 귀환 회복이 우선
:594 if (my_tower_protects || minion_power < 0) && !base_heal_dominates { :595 veto_wave(0x98) += 1; return }
:602 if minion_power > 0 {
:603~606 wave_crashing_enemy_tower = cache.tower(line, 1-team).is_some_and(|et| et.attack_effect.map_or(false, |t| front.distance_sq(et) <= (t.range(et) + front.radius())²))   // ★aux m04.ll:486~581 · 아웃오브라인 tower 21854
:609   if wave_crashing_enemy_tower { :610 veto_crash(0xa0) += 1; return }
     }
:618 full_hp = champ.stat_cached.hp
:619 full_committers = v46_stage1(.., my_hp=full_hp, range_gate=true, only=None, near_enemies)
:621~622 heal_flips = full_committers.is_empty() || v46_stage2(team, cache, ctx, champ, front, &full_committers)   // 풀피였다면 살아남는가
:623 best = committers.iter().min_by_key(|c| c.1).unwrap();  best_id = best.0;  best_dps = best.2     // unwrap_failed anon.76 (:623:81)
:624 heal_die_gain = full_hp.saturating_sub(my_hp) * 60 / max(best_dps, 1)
:626 ep = cache.player_by_champion_id(best_id).unwrap()                          // anon.77 (:626:58)
:627 aggr = ep.info.parameter(+0x180).aggressive_ratio()
:628 best_diff_bound = (1000 - aggr) * 80 / 1000 + 80                            // 80..160
:630 if !heal_flips || heal_die_gain < best_diff_bound { :631 veto_heal(0xa8) += 1; return }
:636 self.v46_commit = true
:637 self.v46_committers = committers.iter().map(|c| c.0).collect::<Vec<usize>>()   // 구 Vec drop 후 memcpy
:638 self.v46_pending.trigger_ticks.push((tick, line))
:639 from_mid = minion_state(line).from_mid(+0x10)
:640 if from_mid > 0 { trigger_wave_enemy_half(0xc0) += 1 } else { trigger_wave_my_half(0xc8) += 1 }
:645 if ctx.debug {
:646   my_die = committers.iter().min_by_key(|c| c.1).copied().unwrap().1          // anon.78 (:646:83)
:650   hp_pct = my_hp*100 / max(full_hp, 1)
:647   debug.add_log(format!("\tV46TRIG T{team} {position:?} tick={tick} hp={hp_pct}% committers={self.v46_committers:?} my_die={my_die} best={best_id} heal_gain={heal_die_gain} from_mid={from_mid} minion_power={minion_power}"))   // anon.74
     }
:653 drop(full_committers, committers, near_enemies) → @1361

// ── L244~245 (@1361 · 22469~22472 · @2049) ──
L244: if self.v46_commit(0x111) { L245: self.in_recall(0x110) = true }   // 커밋 중이면 귀환 강제
→ @1407 = L248

// ── L240~241 (@1365 · gank_hold == true) ──
L240: v46_clear(completed=true)              // v46_commit=0 · committers.len=0 (22477~22484)
L241: v46_flee_end(gank_line)  [passive_line.rs:323~332 인라인 · line 값은 self.line 과 동일(L236 에서 같음이 확정돼 %913 사용)]
   :323 if let Some((entry_tick, wave_flags)) = self.v46_flee_entry.take() {
   :324   flags = (self.v46_flee_approached(0x114) as u8) | wave_flags
   :325   self.v46_pending.flee_episodes.push((entry_tick, line, flags))         // 16B {tick, line@8, flags@9}
   :326   if !(self.v46_flee_hp_min(0x108) < self.v46_flee_hp_entry(0x100)) { :327 flee_nohit_episodes(0xf8) += 1 }   // 도주 중 피해 0
        }
   :330~331 v46_flee = v46_flee_acute = v46_flee_approached = v46_flee_cover = false   (i32 store 0 @0x112)
   :332 self.v46_flee_threats.clear()
→ @1403 = L254 (배치 J)  ※ L243/L248 은 건너뛴다

// ── L248  self.update_v46_flee(version, player, data, debug)  (passive_line.rs:343~474 인라인 · @1407~@2048) ──
:344 tick = cache.game.tick()   (→ %22)
:345 if self.v46_commit || !setting.is_line_phase(tick) { :346 v46_flee_end(self.line); return }   // 귀환 커밋 중엔 도주 에피소드 종료
:349 champ = *%73;  if None { :350 v46_flee_end(self.line); return }
:354 refuge = cache.iter_towers_without_nexus(team).min_by_key(|t| t.distance_sq(champ))    // closure$0 :354 · fold 콜리 m06.ll:21667~
:355 refuge = refuge.or(cache.nexus[team])
:356 if refuge.is_none() { :357 v46_flee_end(self.line); return }
:360~362 under_refuge = refuge.attack_effect.map_or(false, |t| champ.distance_sq(refuge) <= (t.range(refuge) + champ.radius())²)   // 내가 피난 타워 사거리 안
:365 if !self.v46_flee(0x112) {                                                  // ── 도주 진입 판정 ──
:434   if under_refuge → return                                                  // (@1671 → @2048)
:441   (verdict, committers) = v46_flee_gate_check(version, player, data, champ)  // sret 40B
:442   if verdict != 0 → drop committers; return
:446   ms = blackboard[team].minion_state(line)
:447   front_minion = ms.front_minion.and_then(get_entity_by_id)                 // closure$8
:448   wave_crashing = ms.minion_power > 0 && front_minion.is_some_and(|f|      // ★aux m04.ll:15328~15498 (closure s7_0)
:449~452     cache.tower(line, 1 - player.team).is_some_and(|t| t.attack_effect.map_or(false, |a| f.distance_sq(t) <= (a.range(t) + f.radius())²)))
:456   wave_flags = (wave_crashing ? 2 : 0) | (ms.from_mid > 0 ? 4 : 0)
:457   self.v46_flee = true;  :458 self.v46_flee_acute = true
:459   self.v46_flee_threats = committers.iter().copied().collect::<Vec<usize>>()   (구 Vec drop 후 memcpy)
:460   self.v46_flee_entry = Some((tick, wave_flags))
:461   self.v46_flee_hp_entry = champ.hp;  :462 self.v46_flee_hp_min = champ.hp
:463   self.v46_flee_approached = true
:464   flee_triggers(0xd0) += 1;  :465 flee_hold_ticks(0xd8) += 1;  :466 flee_hold_acute_ticks(0xe0) += 1
:467   if ctx.debug { :471 hp_pct = champ.hp*100/max(champ.stat_cached.hp,1); :468 debug.add_log(format!("\tV46FLEE T{team} {position:?} tick={tick} hp={hp_pct}% threats={self.v46_flee_threats:?}")) }   // anon.68
:474   drop committers; return
     } else {                                                                     // ── 도주 유지 ──
:366   self.v46_flee_hp_min = min(champ.hp, self.v46_flee_hp_min)
:374   tower = cache.tower(line, team)  (인라인)
:375~380 twin = twin_towers[team].iter().min_by_key(|t| t.distance_sq(line.get_start_position(setting, team))).map(|t| *t)   // closure$2/$3
:381   nearest_tower = tower.or(twin).or(cache.nexus[team])                     // Option (unwrap 없음)
:382~383 front_minion = blackboard[team].minion_state(line).front_minion.and_then(get_entity_by_id)   // closure$4
:384   anchor = front_minion.or(nearest_tower).unwrap_or(refuge)
:385~386 still = self.v46_flee_threats.iter().any(|id| game.get_entity_by_id(id).is_some_and(|e| e.distance_sq(anchor) < 62500000001))   // 위협이 앵커 250000 안에 남아있나 · threats 비면 false
:389   if !still {
:390     if ctx.debug { debug.add_log(format!("\x0cV46FLEECLR T{team} {position:?} tick={tick} 위협해소(front앵커) approached={self.v46_flee_approached}")) }   // anon.67
:393     v46_flee_end(self.line); return
       }
:395   flee_hold_ticks(0xd8) += 1
:399   reaction = champ.attack_duration() + 6
:400~404 acute = threats.iter().any(|id| get_entity_by_id(id).is_some_and(|e| e.distance(champ) <= e.attack_effect.map_or(0, |a| a.range(e)) + e.radius() + champ.radius() + e.move_speed*reaction))
:400   self.v46_flee_acute(0x113) = acute
:407~409 if acute { self.v46_flee_approached(0x114) = true; flee_hold_acute_ticks(0xe0) += 1 }
:411   if under_refuge {
:412     flee_hold_refuge_ticks(0xe8) += 1
:420~423 self.v46_flee_cover(0x115) = front_minion.is_some_and(|f| refuge.attack_effect.map_or(false, |t| f.distance_sq(refuge) <= (t.range(refuge) + f.radius())²))   // ★aux m04.ll:388~483
:426     if cover { :427 flee_hold_cover_ticks(0xf0) += 1 }
       } else { self.v46_flee_cover = false }
       return
     }
→ @2048 → @1403 = L254 (배치 J)

// 인라인 헬퍼(공통): Effect::range(caster) [effect.rs:26] = caster.stat_buff_cached.range(0x438) + effect.range(0x4a0) + (caster.level(0x5c8)-1)*effect.growth_range(0x4a8)
//                 Entity::radius() [entity.rs:1511~1515] = radius_mult(0x470)==0 ? radius(0x680) : radius*(mult+100)/100
//                 Entity::distance_sq [entity.rs:2158 → utils.rs:7~9] = dx²+dy² (abs_diff)
// rnd(StdRng): 배치 I 범위 gen_range 호출 사이트 0개.

// passive_line.rs:254~304 (배치 J)
// 진입: @1403(22570) ← 배치 I 의 @1400(L241 · 22568 `br %1403`) 및 @2048(L248 · 24137 `br %1403`) 두 갈래. 이 절 전체의 종착은 @2254 `ret void`(L304) 하나뿐.
// 선행 값(배치 H, L222): team=%64=player.info.team(0x930) · %69=player.info.position(0x9c0) · %70=data.cache · %73=&cache.player_champion[team][position] · %79=data.context

// L254  @1403
if (team_plan.objective@tag(0x41f) & !1) == 8 {           // = Some(MainObjective::Gank{..} | MainObjective::Dive{..})  (Gank=8 · Dive=9 · tcxdict)
  // @2050  L0/L260
  gank_line = team_plan+0x420 (i8 LineType)                  // Gank.line == Dive.line 오프셋 동일
  // L260
  if gank_line == self.line(0x116) {                          // player.rs:988 derive PartialEq
    // L262  @2223
    champ = *%73 (cache.player_champion[team][position])  ; null → core::option::unwrap_failed(@anon…82) 패닉(option.rs:1011/1013)
    // L263  @2226/@2231
    max = champ.stat_cached.hp(0x628) ; if max == 0 → panic_const_div_by_zero(@anon…83)
    hp_ratio = champ.hp(0x670) * 100 / max                    // udiv
    // L264
    if hp_ratio > 24 {                                        // HP 25% 이상 (표기: >24 ≡ >=25)
      // L265  @2251
      self.in_recall(0x110) = false
      // L266  v46_clear(true) 인라인
      self.v46_commit(0x111) = false ; self.v46_committers.len(0x40) = 0
      → L304
    } else {
      // L269  @2238/@2244/@2245
      self.chats.push(Chat::Cancel(CancelReason::LowHpSelf))  // len==cap → grow_one ; ptr[len] = {+0: 17, +1: 0} ; len += 1
      → L304
    }
  }
  // L271  @2056/@2058/@2060/@2066   (else-if)
  else if is_adj_line(gank_line, self.line) && self.in_recall(0x110) {
    //   is_adj_line 인라인(player.rs:1006/1008): self.line==Mid(1) ? gank_line∈{Top(0),Bottom(2)} : gank_line==Mid(1)
    //   ⚠ in_recall 은 인접판정과 무관하게 항상 load 됨(select 접힘) — 순수 읽기라 의미 차이 없음
    // L273  @2069
    champ = *%73 ; null → unwrap_failed(@anon…80)
    // L274  @2072/@2077
    max = champ.stat_cached.hp ; max == 0 → panic_const_div_by_zero(@anon…81)(@2171)
    hp_ratio = champ.hp * 100 / max
    // L277  @2077~@2152  (적 챔피언 5슬롯 완전 언롤 · any 단락)
    team' = 1 - team                                          // 적 팀 (DI `team` 재바인딩)
    my_line_enemy_visible = cache.iter_champions(team').any(|c| {   // player_champion[team'][0..5], null 슬롯 skip
      // L278 closure$1
      is_near_line(data.context, c.x(0x660), c.y(0x668), self.line)
      // L279
      && data.blackboard[team'].is_recent_visible(cache.game /*data_ptr, vtable*/, player, c)
    })                                                         // @2153 phi: 어느 슬롯이든 true → true, 5슬롯 소진 → false
    // L283  @2153~@2198
    tower = cache.tower(self.line, team')                     // [top|mid|bottom]_tower[team'].or(…_tower2[team'])  (0x180+line*0x20+team'*8 · 없으면 0x190+…)
    pushed_to_tower = tower.is_some_and(|tower| {             // option.rs:659/661 · closure$2
      // L284
      if tower.ty@tag(0x68) == 2 (Tower) {
        // L285
        info.nearest_enemy(0x88 tag).is_some_and(|(_, id /*0x98*/)| {
          // L286
          cache.game.get_entity_by_id(id) /*vtable+0x1f0*/ .is_some_and(|e| {
            // L287
            e.ty@tag(0x68) == 1 (Minion) && e.team(0x0 tag)==0 (Player) && e.team.0(0x8) == team /*내 팀*/
          })
        })
      } else { false }
    })
    // L296  @2198 (타워 Some) / @2204 (타워 None ⇒ pushed_to_tower=false 로 접힘)
    if hp_ratio > 69 && (pushed_to_tower || !my_line_enemy_visible) {   // HP 70% 이상 && (내 미니언이 적 타워를 치고 있거나 · 내 라인에 최근 보인 적 챔피언이 없음)
      // L297  @2207
      self.in_recall(0x110) = false
      // L298  v46_clear(true) 인라인
      self.v46_commit(0x111) = false ; self.v46_committers.len(0x40) = 0
      // L299  @2207/@2215/@2216
      self.chats.push(Chat::HideLineToo(gank_line, 0))        // len==cap → grow_one ; ptr[len] = {+0: 12, +1: gank_line, +8: 0} ; len += 1
    }
    → L304
  }
  // (인접 아님 또는 !in_recall) → L304
}
// L304  @2254
ret void

// 배치 J 안 rnd(StdRng) gen_range 호출 사이트: 0개.
// 배치 J 안 debug(DebugFrameData) 접근: 0건.
// 사장 코드: reach.py(version=2 · gamemode=0) 결과 이 함수 사장 호출부 0 · 접힌 분기 1(@103, 배치 H 범위) — 배치 J 에 NA 봉인 대상 없음.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `de92d0` → `fb5ff0` epic_passive_plan (i=82 · 변경·심층(r21 §D: w9 §3))
- 0.5.8 src: `game-ai\src\plan_legacy\old\epic.rs:317` · one_line: 에픽(모가드) 목표의 수동 국면에서 내 BigPlan(PassiveLine 라인 / EpicHuntAndPoke / None)을 고른다
- 0.6.0 판정: **심층(r21)** · 패치 요지: serpen 동형 · cc0 대체 Setup = remain(epic +0x1b0) → `since=tick-tp.+0xa0` · top_lead(+0x23a0)<3 ? (since≤3tps ? f904b0 : F) : (since≤3tps ? mid_lead>2‖f904b0 : mid_lead>2 ? EHP : F) · F = f10c60(camp,150000).len()>2 · Split → PassiveLine{Bottom, mode 1} · CONTEST 치환: slot 1 · ca5/ca3 · +0x404/+0x403 · n=fbb470 · ETA +0x790 · **k-랭킹 없음** · phase 2: `ca3 비트 && dist²>220000² → EHP else None`
- RE 정본: `2026-09-17_r21_심층_w9_serpen_epic_passive_plan_HuntAndPoke_sub_plan_ObjContest헬퍼_원문.md` · 절: w9 §3
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_ai::plan_legacy::team_plan::ObjectPhase, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<(u64, u64)>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::B`
- consts: [{"value": 6, "src_line": 318, "meaning": "spawn_epic 인라인: (tutorial_tag-1) <u 6 ⇔ 태그∈{1 First..6 JungleOnly} → 에픽 없음 → None. 통과 = {0 None,7 Line,8 Total}", "kind": "임계", "ev": 4}, {"value": -1, "src_line": 319, "meaning": "Option<BigPlan>::None 니치 태그(store i64 -1). L331에선 Option<LineType>::None(icmp eq i8 -1)", "kind": "센티널", "ev": 4}, {"value": 3, "src_line": 324, "meaning": "ObjectPhase::Hunt(3) — phase switch → EpicHuntAndPoke 즉시 반환(L325)", "kind": "임계", "ev": 4}, {"value": 1, "src_line": 324, "meaning": "ObjectPhase::Setup(1) — 본문 진입. 그 외 phase → None(L497) (shl 시프트량 아님 — 열거형 태그 리터럴)", "k
- 0.5.8 logic 전문:
```
ctx = data.context; if !spawn_epic(ctx.tutorial) (태그∈1..6) → return None                                   [L318-319]
champ = cache.player_champion[team][position].unwrap()                                                  [L322]
match phase { Hunt(3) → return EpicHuntAndPoke(default) [L325]; Setup(1) → 아래; _ → return None [L497] } [L324]
strategy = player.strategy(rnd, game)                                                                     [L327]
opposite_object_pressure = v23_enemy_object_pressure(player, data, Serpen)                                [L329]
if opposite_object_pressure { if let Some(line) = v23_objective_setup_pressure_line(player,data,&[Top,Mid]) → return PassiveLine{line} } [L330-332]
match strategy.object_buildup {                                                                            [L337]
  Split(pos) [L339]: if pos == my position { if !v25_objective_splitter_should_join_contest(version,player,data,Morgard) → return PassiveLine{Bottom} [L340-341] } → Gather 로 진행
  Flexible [L347]: camp = map.camp_pos(Morgard, team==0)
    can_reach = Σ_{p∈0..5, 적 챔프 e=player_champion[1-team][p] 존재} [                                    [L350-359]
        last_pos = team_plan.vision.last_visible_pos[p]; d = distance(last_pos, camp).sat_sub(150000)      [L352-353]
        can_move = (game.tick() − vision.last_checked_ticks[p]).sat_sub * e.move_speed                    [L354-355]
        e.hp*100/e.max_hp > 49 && can_move >= d && !blackboard[1-team].is_recent_visible(game, player, e) ] [L357]
    if can_reach < 2 [L362]: if blackboard[team].bottom_minion_state.minion_count < -3 && count{p≠me && blackboard[team].in_big_line(p,Bottom)}==0 → return PassiveLine{Bottom} [L361-366]
    if can_reach <= 2 (L362 count<2 에서 바텀 조건 실패한 경우 + L370 count==2) && dist²(champ, camp) < 320000² [L370] && is_enemy_side(ctx, team, champ.x, champ.y) [L371 = is_blue_side(x−y+height > width) XOR team==0]:
        top=bb[team].top_minion_state.minion_count; mid=bb[team].mid_minion_state.minion_count             [L373-374]
        if top_lead[team] < 3 { if mid_lead[team] < 3 { return PassiveLine{ if top<mid Top else Mid } [L377-380] } else return PassiveLine{Top} [L383] }
        else if mid_lead[team] < 3 → return PassiveLine{Mid} [L385]  (그 외 → Gather 로 진행) [L376-386]
    그 외 → Gather 로 진행
  Gather (및 위 폴백) [L394]: camp = map.camp_pos(Morgard, team==0)
    moba = game.get_game_mode().as_moba().unwrap(); remain = moba.jungle_runner.epic.next_respawn_tick.sat_sub(tick) [L397]
    pos = depart_anchor.unwrap_or((champ.x,champ.y)); travel = distance(pos,camp) / champ.move_speed (0이면 패닉) [L399-402]
    if remain > tps*2 + travel → return None                                                               [L403-404]
    if tick − team_plan.obj_spawn.epic_camp_last_visible_tick > tps*3 [L407]:
        near = team_plan.can_near_enemies_range(rnd, player, data, camp, 150000)                            [L461]
        if near.len() > 2 → return EpicHuntAndPoke                                                          [L462-463]
        if top_lead>2 { if mid_lead>2 → EpicHuntAndPoke [L470-471] else → PassiveLine{Mid} [L490] }
        else if mid_lead<3 → PassiveLine{ if top<mid Top else Mid } [L478-485] else → PassiveLine{Top} [L488]
    else:
        nearest = player_champion[1-team].filter(|e| bb[1-team].is_recent_visible(game,player,e)).min_by_key(dist²(e,camp)) [L408-409, aux m12]
        if top_lead>2 && mid_lead>2 → return EpicHuntAndPoke                                               [L412-413]
        if let Some(e)=nearest [L420]: if dist²(e,camp) < 150000² → return EpicHuntAndPoke [L421-422]
            top/mid = bb[team] minion_count [L424-425]; if top_lead<3 { if mid_lead<3 → PassiveLine{top<mid?Top:Mid} [L428-431] else PassiveLine{Top} [L434] } else if mid_lead<3 → PassiveLine{Mid} [L436] else → None [L439]
        else (적 시야 없음) [L442-457]: 같은 lead/minion 표 → Top/Mid(L447-454) 또는 None [L457]
}
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `defcd0` → `fd7f70` EpicHuntAndPokePlan::sub_plan (i=83 · 변경·심층(r21 §D: w9 §5))
- 0.5.8 src: `game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:30` · one_line: 에픽(Morgard) 사냥/견제 빅플랜의 서브플랜 선택 — Steal/EpicCheck/Recall/LineDefense/EpicHunt/EpicPoke 중 하나를 sret 로 반환
- 0.6.0 판정: **심층(r21)** · 패치 요지: dcbe00 동형: slot 1 · ef7590(1) · C `3e4!=2 → Recall; 404==2 && cd5!=0 → Recall` · ca5 · `phase=(404==2)?cd6:+0x400` · ObjContest{1, poke: epic.hp<max} · Hunt Split && cc2==0 && f88b30(4) → LineDefense{Bottom} · v≤2 `min(gd.88,gd.98)>5tps` / v≥3 `!ef6da0(1)` · `!cc7 ‖ 404==2 ‖ 403&1 → EpicHunt{false} else ObjContest{1,true}` · D(hp<21·regions==7·200000²)/E(f0cfe0(4)·22500000001) 그대로
- RE 정본: `2026-09-17_r21_심층_w9_serpen_epic_passive_plan_HuntAndPoke_sub_plan_ObjContest헬퍼_원문.md` · 절: w9 §5
- sig: `fn(&mut game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan`
- consts: [{"value": 0, "src_line": 36, "meaning": "get_game_mode(vtable+0x40) 반환 {tag,ptr} 의 tag 0 = Moba (as_moba, game.rs:231). 아니면 unwrap_failed", "kind": "태그", "ev": 4}, {"value": 100, "src_line": 47, "meaning": "hp_ratio = hp*100/max_hp (max_hp==0 이면 div_by_zero 패닉 — build 와 달리 max(.,1) 없음)", "kind": "계수", "ev": 4}, {"value": 51, "src_line": 58, "meaning": "hp_ratio < 51 (=HP 50% 이하) 이면 Recall 후보. `<51` vs `<=50` 표기 불가", "kind": "임계", "ev": 4}, {"value": 10, "src_line": 51, "meaning": "SubPlan 메모리태그 10 = EpicCheck (idx 8 + 2)", "kind": "태그", "ev": 4}, {"value": 5, "src_line": 59, "meaning": "SubPl
- 0.5.8 logic 전문:
```
// hunt_and_poke.rs:30~96 (+ check_recall :103~160 인라인). 분기 순서대로.
t = player.info.team; pos = player.info.position
champ = cache.player_champion[t][pos].unwrap()                                   // :32

// ── A. 스틸 모드 (:35~44)
if self.focus_epic_only || self.vision_only {
   moba = game.get_game_mode() (vtable+0x40) → tag 0 = Moba 아니면 unwrap 패닉      // :36
   epic = moba.jungle_runner.epic.live_list.first().and_then(|id| game.get_entity_by_id(id))   // :37 closure$0
   if epic.is_some() → return Steal{ last_vision_tick:0, target:Some(Epic), commit:self.focus_epic_only }   // :39
   else → return EpicCheck{ move_check:false }                                        // :44
}

// ── B. 일반 (:47~)
hp_ratio = champ.hp*100 / champ.stat_cached.hp                                    // :47 (max_hp 0 → 패닉)
can_upgrade_item = upgrade_item(version, rnd, player, game, ctx).is_some()          // :48
epic = moba.jungle_runner.epic.live_list.first().and_then(get_entity_by_id)         // :49 closure$1
if epic.is_none() → return EpicCheck{false}                                        // :50~51
(x0,y0,x1,y1) = map.fountains[t]                                                    // :56
is_in_heal_area = x0<=champ.x<=x1 && y0<=champ.y<=y1                                // :57
if epic.hp == epic.stat_cached.hp /*에픽 풀피*/ && (hp_ratio < 51 || can_upgrade_item || (is_in_heal_area && champ.hp < champ.stat_cached.hp))   // :58 (줄 안 순서 표기 불가)
   → return Recall                                                                   // :59

// ── C. 팀 오브젝트 게이트 (:63~64)
if team_plan.objective != Some(Morgard{..}) → return Recall                         // :63 objective_target / :94
if phase == Hunt {                                                                   // :64 take_hunt_commit
   strategy = player.strategy(rnd, game)                                             // :65
   if strategy.object_buildup == Split(position) && position == pos                  // :66
      && v25_objective_splitter_can_stay(version, player, data, Morgard)             // :68
      → return LineDefense{ style:Aggressive, line:fallback_line(ctx, Bottom), minion_action_type:Push }   // :70~73
   else → return EpicHunt{ need_recall:false }                                       // :77
}

// ── D. check_recall(self, version, player, data, goal_data, debug) (:79, 본문 :103~160)
   champ = player_champion[t][pos].unwrap()                                          // :104
   if heal_area(t).contains(champ) && champ.hp < champ.max_hp → true                 // :107~108 (game_core heal_area 인라인: 팀0 x<=64000&&y>=896000&&y<=960000 / 팀1 x>=892000&&x<=960000&&y<=64000)
   hp_ratio = champ.hp*100/champ.max_hp                                              // :112
   if min(goal_data.epic.epic_enemy_tick, epic_ally_tick) <= tps*5 → false           // :115
   if self.v46_flee {                                                                // :124
      if self.v46_flee_threats.iter().any(|id| game.get_entity_by_id(id).is_some_and(|e| dist_sq(e,champ) < 40000000001)) → true   // :125~126
      self.v46_flee=false; self.v46_flee_threats.clear()                              // :131
   } else {
      (code, threats) = v46_flee_gate_check(version, player, data, champ)             // :134
      if code == 0 { self.v46_flee=true; self.v46_flee_threats = threats.to_vec();    // :136~137
                     if ctx.debug { debug.add_log(data, player, format!(.. tick, position, .. threats)) }   // :138~139
                     return true }                                                    // :140
   }
   enemy_cnt = player_champion[1-t].iter().flatten().filter(|e| map.regions[e.y/32000][e.x/32000]==7 && blackboard[1-t].is_recent_visible(game, player, e)).count()   // :147~149
   ally_cnt  = player_champion[t].iter().flatten().filter(|a| regions==7 && blackboard[t].is_recent_visible(game, player, a)).count()                 // :154~155
   return hp_ratio < 21 && ally_cnt < enemy_cnt                                      // :160
if check_recall(..) → return Recall                                                  // :79~80

// ── E. 캠프 확인/견제 (:82~89)
if team_plan.v24_objective_setup_should_check_camp(version, player, data, goal_data, Morgard) → return EpicCheck{false}   // :82~83
if dist_sq(champ, epic) < 22500000001 → return EpicPoke                             // :86 → :87
else → return EpicCheck{false}                                                       // :89
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d65620` → `1011d20` serpen_passive_plan (i=96 · 변경·심층(r21 §D: w9 §2))
- 0.5.8 src: `game-ai\src\plan_legacy\old\serpen.rs:416` · one_line: 세르펜 목표(phase)에 맞는 개인 BigPlan 을 고른다 — Hunt→SerpenHuntAndPoke, Setup→전략(object_buildup)·적 압박·라인 상태·이동시간으로 PassiveLine/SerpenHuntAndPoke/None
- 0.6.0 판정: **심층(r21)** · 패치 요지: 진입 match 에 `phase>2 && cc7 → CONTEST` · `phase==2 && cc7 → CONTEST` 추가 · **cc0(v≥2) 면 구 Setup/Flexible/Gather L429~L496 전체 사장** → `dist(anchor‖champ,camp)/ms + 2tps < remain(serpen 리스폰 +0x1e0) ? None : SHP` · Split → PassiveLine{Top, mode +0x120=1} · CONTEST = v3 블록(ca4 비트 · contest_eligible ef5f20 · contest_can_hold ef6da0 · hp<50/phase 3/건강 아군 n>2(10167f0) → ActiveRecall(recall_fits_window ef57f0, c38++) 또는 None) + 공통(phase 2: ca2 비트 && dist²≥220000² · else tick+이동+2tps ≥ +0x788) → 정글 즉시 SHP · k=clamp(efb690(5,12tps),2,4) · e89b70 (eta,pos) 랭킹 내 index ≤ k → SHP
- RE 정본: `2026-09-17_r21_심층_w9_serpen_epic_passive_plan_HuntAndPoke_sub_plan_ObjContest헬퍼_원문.md` · 절: w9 §2
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, game_ai::plan_legacy::team_plan::ObjectPhase, std::option::Option<(u64, u64)>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::B`
- consts: [{"value": 0, "src_line": 417, "meaning": "TutorialType {0,5,7,8}=serpen_exists. 밖이면 None(L418)", "kind": "태그", "ev": 4}, {"value": 30, "src_line": 421, "meaning": "is_line_phase: tick < epic first_spawn_tick.saturating_sub(tps*30). tutorial 가 위 집합 밖이면 is_line_phase=true 로 접힘(%35 phi)", "kind": "계수", "ev": 4}, {"value": 3, "src_line": 426, "meaning": "ObjectPhase::Hunt → SerpenHuntAndPoke · 오라클 실행 확증(19차 배치C: 오라클 실행 확인: phase=Hunt(3) → Option<BigPlan> tag 14(SerpenHuntAndPoke) 4/4 (v19C_o1.tsv case3))", "kind": "임계", "ev": 2}, {"value": 1, "src_line": 426, "meaning": "ObjectPhase::Setup → 이하 분
- 0.5.8 logic 전문:
```
ctx = data.context; team = player.info.team; my_pos = player.info.position
L417 if !serpen_exists(ctx) → return None (L418)
L421 is_line_phase = tutorial∈{0,5,7,8} ? tick < epic_jungle.first_spawn_tick.saturating_sub(tps*30) : true
L422 champ = cache.player_champion[team][my_pos].unwrap()
L423 skip = is_skip_serpen(version, rnd, player, data);  L425 if skip → return None (L506)
L426 match phase {
  Hunt(3)  → L427 return Some(BigPlan::SerpenHuntAndPoke(SerpenHuntAndPokePlan{v46_flee_threats: vec![], focus_serpen_only:false, vision_only:false, v46_flee:false}))
  Setup(1) → 아래
  _        → return None (L503)
}
L429 strategy = player.strategy(rnd, game)
L431 opposite_object_pressure = v23_enemy_object_pressure(player, data, JungleType::Morgard)
     (objective_helpers.rs:181~207 요지: 해당 오브젝트 생존 && 근처(180000, hp40%) 최근가시 적 != 0 && 적 >= 건강 아군)
L432 if opposite_object_pressure {
L433     if let Some(line) = v23_objective_setup_pressure_line(player, data, &[Bottom, Mid]) {
L434         return Some(PassiveLine(PassiveLinePlan{line, ..default}))
         }
     }
L438 object_buildup = strategy.object_buildup
L439 match object_buildup {
  Split(position) → L440 if !is_line_phase && position == my_pos {
                      L441 if !v25_objective_splitter_should_join_contest(version, player, data, JungleType::Serpen) {
                      L443     return Some(PassiveLine{line: fallback_line(ctx, Top)})
                      } }
                    // 그 외 → L496
  Flexible(6)     → L449 camp = map.camp_pos(Serpen, team==0)
                    L452 near_serpen_enemy = (0..5).filter_map(|p| player_champion[1-team][p]).filter(|e|
                    L454     last_pos = team_plan.vision.last_visible_pos[p];
                    L455     d = distance(last_pos, camp).saturating_sub(150000);
                    L456     move_speed = e.stat_cached.move_speed;
                    L457     can_move = (tick.saturating_sub(team_plan.vision.last_checked_ticks[p])) * move_speed;
                    L459     e.hp*100/e.max_hp > 49 && can_move >= d && !blackboard[1-team].is_recent_visible(game, player, e)
                         ).count()
                    L463 if near_serpen_enemy < 3 && dist_sq(champ, camp) <= 320000^2
                    L464    && is_enemy_side(ctx, team, champ.x, champ.y)   [= (team==0) XOR ((x - y + height) > width)] {
                    L466     bottom_count = blackboard[team].bottom_minion_state.minion_count; L467 mid_count = ...mid...
                    L469     if cache.bottom_lead[team] < 3 {
                                 if cache.mid_lead[team] < 3 { L470 return Some(PassiveLine{line: bottom_count < mid_count ? Bottom(L471) : Mid(L473)}) }
                    L476         else return Some(PassiveLine{line: Bottom})
                             }
                    L477     else if cache.mid_lead[team] < 3 { L478 return Some(PassiveLine{line: Mid}) }
                         }
                    L482 remain_spawn_tick = mode.jungle_runner.serpen.next_respawn_tick.saturating_sub(tick)
                    L483 camp_pos = map.camp_pos(Serpen, team==0)
                    L485 (dpx,dpy) = depart_anchor.unwrap_or((champ.x, champ.y))
                    L486 dist_to_camp = distance(dpx,dpy, camp_pos)
                    L487 move_speed = champ.stat_cached.move_speed;  L488 move_tick = dist_to_camp / move_speed
                    L489 if remain_spawn_tick > move_tick + tps*2 → L490 return None
                    // 아니면 L496
  Gather(5)       → L496
}
L496 return Some(BigPlan::SerpenHuntAndPoke(SerpenHuntAndPokePlan{빈 Vec, 플래그 false}))   // camp_pos 호출 결과는 IR 상 미사용
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `df0e90` → `dcbe00` SerpenHuntAndPokePlan::sub_plan (i=97 · 변경·심층(r21 §D: w9 §4))
- 0.5.8 src: `game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:30` · one_line: 세르펜 사냥/견제 빅플랜의 서브플랜 선택 — Epic 판과 동형(Steal/SerpenCheck/Recall/LineDefense/SerpenHunt/SerpenPoke), 차이 5곳은 unknown/logic 에 명시
- 0.6.0 판정: **심층(r21)** · 패치 요지: B v≥3: `serpen.hp==max && (can_upgrade ‖ (in_heal && hp<max))`(hp_ratio<51 삭제) → `!cc7 ‖ !finish_before_enemy(ef7590,0) ? Recall : {c30++; 계속}` · C: `3e4==2 && !(404==2 && cd5==1) → Recall` · v≥3 `cc7 && bt(ca4,pos) → {c38++; Recall}` · `phase=(3e4==2)?cd6:+0x3e0` · **`cc7 && phase==2 → ObjContest{0, poke: serpen.hp<max}`** · Hunt: Split && cc2==0 && f88b30(5) → LineDefense{Aggr,Top,Push} · v≤2 `cc7 && hp<50 && min(gd.c0,gd.d0)>5tps → Recall` / v≥3 `cc7 && !ef6da0(0) && min>5tps → Recall` · `!cc7 ‖ 3e4==2 ‖ 3e3&1 → SerpenHunt{false} else ObjContest{0,true}` · D/E 그대로
- RE 정본: `2026-09-17_r21_심층_w9_serpen_epic_passive_plan_HuntAndPoke_sub_plan_ObjContest헬퍼_원문.md` · 절: w9 §4
- sig: `fn(&mut game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan`
- consts: [{"value": 0, "src_line": 36, "meaning": "get_game_mode 반환 tag 0 = Moba(as_moba). / :70 fallback_line(ctx, LineType 0 = Top) ★Epic 판은 Bottom(2). / :130 v46 gate 코드 0 = 도주 발동", "kind": "태그", "ev": 4}, {"value": 100, "src_line": 47, "meaning": "hp_ratio = hp*100/max_hp", "kind": "계수", "ev": 4}, {"value": 13, "src_line": 50, "meaning": "SubPlan 메모리태그 13 = SerpenCheck · 오라클 실행 확증(19차 배치C: 오라클 실행 확인: 세르펜 live_list 비어 있음 → SubPlan tag 13 = SerpenCheck{move_check:false} 10/10 (v19C_o1.tsv case4, L48~50 경로))", "kind": "태그", "ev": 2}, {"value": 51, "src_line": 58, "meaning": "hp_ratio < 51 → Recall 후보(
- 0.5.8 logic 전문:
```
// serpen/hunt_and_poke.rs:30~92 (+ check_recall :99~159 인라인). Epic 판(epic_hunt_and_poke__sub_plan.json)과 같은 골격 — ★표시가 차이.
t, pos, champ = … (:32)
// A. 스틸 (:35~44)
if self.focus_serpen_only || self.vision_only {
   serpen = as_moba().jungle_runner.serpen.live_list.first().and_then(get_entity_by_id)   // :36~37 (MobaMode+0x1d0/+0x1d8)
   Some → Steal{ last_vision_tick:0, target:Some(Serpen) ★, commit:focus_serpen_only }   // :39
   None → SerpenCheck{false}                                                            // :44
}
// B. 일반
hp_ratio = hp*100/max_hp                                                                 // :47
serpen = live_list.first().and_then(get_entity_by_id); None → SerpenCheck{false}         // :48~50
(x0,y0,x1,y1) = map.fountains[t]; is_in_heal_area = …                                    // :55~56
can_upgrade_item = upgrade_item(version, rnd, player, game, ctx).is_some()               // :57 (★Epic 은 :48 에서 먼저 계산 — 순서만 다르고 값 동일)
if serpen.hp == serpen.max_hp && (hp_ratio<51 || can_upgrade_item || (is_in_heal_area && hp<max_hp)) → Recall   // :58~59
// C. 오브젝트 게이트
if team_plan.objective != Some(Serpen{..}) → Recall                                       // :63 / :90 ★태그 1
if phase == Hunt {                                                                        // :64
   strategy = player.strategy(rnd, game)                                                  // :65
   if object_buildup == Split(pos) && v25_objective_splitter_can_stay(version, player, data, Serpen ★5)   // :66~68
      → LineDefense{ Aggressive, fallback_line(ctx, Top ★0), Push }                       // :70~73
   else → SerpenHunt{ need_recall:false }                                                 // :77
}
// D. check_recall (:99~159)
   heal_area(t).contains(champ) && hp<max → true                                          // :103~104
   hp_ratio = hp*100/max                                                                  // :108
   min(goal_data.serpen.epic_enemy_tick, epic_ally_tick) <= tps*5 → false                 // :111
   v46 도주 블록 — Epic 판과 동일(:119~135, 위협 반경 200000, gate 코드 0, 로그)
   ★serpen = game.jungle_runner()(vtable+0xe0).serpen.live_list.first().and_then(get_entity_by_id)   // :142~143 (JungleRunner+0x1b8/+0x1c0)
   ★if serpen.is_none() → true(귀환)                                                       // :143 (Epic 판엔 이 조회 없음)
   ★enemy_cnt = player_champion[1-t] 중 blackboard[1-t].is_recent_visible && dist_sq(e, serpen) < 22500000001 인 수   // :148~151 closure$2
   ★ally_cnt  = player_champion[t]   중 blackboard[t].is_recent_visible   && dist_sq(a, serpen) < 22500000001 인 수   // :153~156 closure$3  (Epic 은 regions==7 셀)
   return hp_ratio < 21 && ally_cnt < enemy_cnt                                            // :159
if check_recall → Recall                                                                  // :79~80
// E. 꼬리
if team_plan.v24_objective_setup_should_check_camp(version, player, data, goal_data, Serpen ★5) → SerpenCheck{false}   // :82~83
★else → SerpenPoke (거리 게이트 없음 — Epic 판은 dist<150000 일 때만 Poke, 아니면 Check)   // :86
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `db90f0` → `faa440` update (i=14 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\line_gank\ganker.rs:39` · one_line: 갱커의 갱크 계속/취소 판정 — 저HP거나 목표 부시 도착+적 없음이면 Cancel
- 0.6.0 판정: **심층** · 패치 요지: 저HP 취소 시 `+0xaf=1` · 부시 도착 `if +0x38==0 {+0x38=tick}`
- RE 정본: `2026-09-17_r20_변경99_배치G_누락7_소폭오프셋만_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치G_누락7_소폭오프셋만_디컴대조_원문.md` §3
- sig: `fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData)`
- consts: [{"value": 100, "src_line": 44, "meaning": "hp_ratio = hp * 100 / max_hp — 백분율 환산 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410)", "kind": "계수", "ev": 2}, {"value": 41, "src_line": 47, "meaning": "★HP 취소 임계. hp_ratio < 41 이면 갱크 포기(41% 미만) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410)", "kind": "임계", "ev": 2}, {"value": 2, "src_line": 261, "meaning": "EntityType 판별자 2 = Tower (is_tower2 의 앞단 비교). 같은 리터럴 2 는 이 함수에서 역할이 넷이다 — ①EntityType::Tower 판별자(m08.ll:94675 `icmp eq i64 %60, 2`; `!dbg !55641
- 0.5.8 logic 전문:
```
fn update(&mut self, _version, _rnd, player, data, goal_data, _positioning_score, _debug)
// ganker.rs:39. 반환값 없음 — self.chats / self.phase 만 바꾼다.

team = player.info.team // PlayerState+0x930, team<2 아니면 panic_bounds_check(ganker.rs:43)
pos = player.info.position.as_index() // PlayerState+0x9c0, entity.rs:580, range 0..5
cache = data.cache // OperationData+0x0
champ = cache.player_champion[team][pos].unwrap() // cache+0x1e0 + team*40 + pos*8, None 이면 panic(ganker.rs:43)

// ganker.rs:44
max_hp = champ.stat_cached.hp // Entity+0x628 (0 이면 div_by_zero panic)
hp_ratio = champ.hp * 100 / max_hp // Entity+0x670

// ganker.rs:47 — 취소 사유 ①
if hp_ratio < 41 {
 self.chats.push(Chat::Cancel(CancelReason::LowHpSelf)) // ganker.rs:48, 태그17/사유0
 self.phase = LineGankerPhase::Cancel // self+0x29 = 8
 return
}

// ganker.rs:54 — bush = self.target_bush_v30(player, data) [전량 인라인]
line = self.line // self+0x28, 0=Top/1=Mid/2=Bottom
context = data.context // OperationData+0x8
tower = cache.<line>_tower[team].or(cache.<line>_tower2[team]) // (cache+384+line*32)[team] .or( (cache+400+line*32)[team] ) ganker.rs:249

if tower.is_none() { // ganker.rs:250
 bush = match line { // ganker.rs:251
 Top => if team==0 {2} else {16} // 252
 Mid => if team==0 {4} else {17} // 253
 Bottom => if team==0 {9} else {21} // 254
 }
} else {
 t = tower.unwrap() // ganker.rs:258
 // champ 은 위에서 구한 것 재사용 (ganker.rs:259)
 is_tower_variant = (t.ty 판별자 == 2) // Entity+0x68 == EntityType::Tower
 match line { // ganker.rs:261
 Top => { // ganker.rs:263 info = t.ty.Tower 페이로드(Entity+0x70)
 if !is_tower_variant { unreachable!() } // ganker.rs:274
 if t.ty.is_tower2() { // ganker.rs:264 → Entity+0x128 > 4
 bush = if team==0 {3} else {6} // 265
 } else if info.nearest_enemy.is_some() { // 267, Entity+0x88 != 0
 bush = if team==0 {6} else {3} // 268 ← 좌우 반전
 } else {
 bush = if team==0 {3} else {6} // 270
 }
 }
 Mid => {
 if t.ty.is_tower2() { // ganker.rs:278 (판별자==2 && Entity+0x128>4)
 // ganker.rs:279 — map_regions::is_top_side(context, champ.x, champ.y)
 // ry = context.setting.height - champ.y ; IR 술어 = (ry < champ.x) = **!is_top_side**
 // (오라클 확증 — `is_top_side` 원식 = x + y <= height)
 if ry_lt_x /* = !is_top_side */ {
 bush = if team==0 {13} else {18} // 282
 } else {
 bush = if team==0 {8} else {12} // 280
 }
 } else {
 // ganker.rs:285 — 여기만 team 에 무관
 bush = if ry_lt_x /* = !is_top_side */ {14} else {11}
 }
 }
 Bottom => { // ganker.rs:293
 if !is_tower_variant { unreachable!() } // ganker.rs:304
 if t.ty.is_tower2() { // 294
 bush = if team==0 {15} else {20} // 295
 } else if info.nearest_enemy.is_some() { // 297
 bush = if team==0 {20} else {15} // 298 ← 좌우 반전
 } else {
 bush = if team==0 {15} else {20} // 300
 }
 }
 }
}

// ganker.rs:55
map = context.map // GameContext+0x20 (MapDef 28112B)
cy = min(champ.y / 32000, 29)
cx = min(champ.x / 32000, 29)
champ_bush = map.bushes[cy][cx] // MapDef+0x1c98, [30][30] usize

// ganker.rs:56 — 취소 사유 ②
if champ_bush == bush {
 // ganker.rs:57
 if !goal_data.has_near_line_enemy(self.line, map) {
 self.chats.push(Chat::Cancel(CancelReason::TargetMissing)) // ganker.rs:58, 태그17/사유2
 self.phase = LineGankerPhase::Cancel // self+0x29 = 8
 }
}
// ganker.rs:62 return
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `dd6b40` → `f0dc20` v24_objective_setup_lane_pressure_ready (i=67 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7` · one_line: 오브젝트(Morgard/Serpen) Setup 단계에서 '라인 정리가 끝나 캠프로 모여도 되는가'를 아군 집결·건강·압박 라인 잔여로 판정
- 0.6.0 판정: **심층** · 패치 요지: L13 take_setup_like 게이트 이중모드(dd5db0 L91 과 동일 패턴)
- RE 정본: `2026-09-17_r20_변경99_배치G_누락7_소폭오프셋만_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치G_누락7_소폭오프셋만_디컴대조_원문.md` §5
- sig: `fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool`
- consts: [{"value": 4, "src_line": 13, "meaning": "JungleType::Morgard 태그(switch case, m09.ll:21287). L49 `camp == Morgard` 도 동일. L51 Morgard 집결 요구 인원 4", "kind": "태그", "ev": 4}, {"value": 5, "src_line": 13, "meaning": "JungleType::Serpen 태그(switch case). (아군 슬롯 수 5 는 슬라이스 길이·판정 아님)", "kind": "태그", "ev": 4}, {"value": 0, "src_line": 13, "meaning": "MainObjective::Morgard 메모리태그 0 (0x41f) — camp==Morgard 일 때 objective 가 Morgard 여야 함", "kind": "태그", "ev": 4}, {"value": 1, "src_line": 13, "meaning": "ObjectPhase::Setup 메모리태그 1 (0x420) — 두 case 공통. camp==Serpen 은 objective 태그 1(Serpen) 도 요구", "kind": "태그", 
- 0.5.8 logic 전문:
```
fn v24_objective_setup_lane_pressure_ready(self, version, player, data, goal_data, camp, debug) -> bool
  // L13 take_setup_like(camp) 인라인 (team_plan.rs:258)
  match camp { Morgard(4) => self.objective == Some(Morgard{phase: Setup,..})   // 0x41f==0 && 0x420==1
               Serpen(5)  => self.objective == Some(Serpen{phase: Setup,..})    // 0x41f==1 && 0x420==1
               _ => return false }  else return false
  obj = if camp==Serpen { WavePriorityObject::Serpen(1) } else { Morgard(0) }
  if is_object_being_taken_by_enemy(player, data, goal_data, self, obj) → return false     // L23
  team = player.info.team
  camp_pos = data.context.map.camp_pos(camp, team == 0)                                    // L27
  if !game.is_visible_cell(team, camp_pos.x/32000, camp_pos.y/32000) → return false       // L28 (vtable+0x100)
  if v23_recent_visible_enemies_near_point(player, data, camp_pos.x, camp_pos.y, 180000, 50) != 0 → return false   // L32
  allies = cache.player_champion[team]   (bounds team<2)
  pc = context.tutorial.player_count()   // runner.rs:295~301 인라인: TopSolo/MidSolo/JungleOnly→1, First/Bottom→2, MidBottom→3, None/Line/Total→>=4
  gathered = allies.flatten().filter(|c| c.hp*100/c.max_hp > 49 && dist²(c, camp_pos) < 220000²+1).count()   // L36~39 (5회 언롤)
  if gathered < min(2, pc) → return false                                                  // L40
  healthy = allies.flatten().filter(|c| c.hp*100/c.max_hp > 49).count()                    // L44 (aux s_0)
  if healthy < min(3, pc) → return false                                                   // L45
  need = if camp == Morgard { 4 } else { 3 }                                               // L49~52
  if gathered >= min(need, pc) {                                                           // L54 (반대 분기)
     if context.debug { debug.infos[my_champ.id].push("v24 objective setup gathered ready: {camp:?}") }   // L55~58
     return true }
  // L63~70 라인 배치 인원 (aux s0_0)
  placed = allies.flatten().filter(|c| c.hp*100/c.max_hp > 49 &&
              match camp { Morgard => is_top_side(c) || is_near_mid_line(ctx, c.x, c.y),      // L65: x <= height - y
                           Serpen  => is_bottom_side(c) || is_near_mid_line(ctx, c.x, c.y),   // L66: x >= height - y
                           _ => false }).count()
  if placed < min(3, pc) → return false                                                     // L70
  // L74 line_ready = self.v24_objective_setup_relevant_lanes_ready(player, data, camp)   — 인라인(objective_discipline.rs:199~202):
  //   L200 lanes: [LineType;2] = match camp { Morgard(4) => [Top, Mid], Serpen(5) => [Bottom, Mid], _ => return false }   (anon.190/.191)
  //   L202 return v23_objective_setup_pressure_line(player, data, &lanes).is_none()      (m09.ll:21945~21946, -1 = None 니치)
  if !line_ready → return false                                                              // L75 (아직 밀어야 할 라인이 남음)
  if context.debug { debug.infos[my_champ.id].push("v24 objective setup lane pressure ready: {camp:?}") }   // L79~82
  return true                                                                                // L86
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d9ac10` → `e85f00` steal::should_steal_now (i=99 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\steal.rs:279` · one_line: [v4] 정글러 스틸 액션 결정 — 에픽/세르펜 각각 '적 3명+ 캠프 접근 가능·우리 팀이 안 치는 중·대상 hp<95%·최근 포기 아님' 후보를 걸러 evaluate_steal_for_target 로 평가하고 Commit > Commit > Lurk(둘 다면 적 예상 처치 잔여틱 짧은 쪽) > Lurk > None 으로 합친다
- 0.6.0 판정: **심층** · 패치 요지: `objective!=Morgard/Serpen` 4곳 → `not_morgard = 3e4!=2 || (404==2&&cd5!=0)` · `not_serpen = 3e4==2 && !(404==2&&cd5==1)`
- RE 정본: `2026-09-17_r20_변경99_배치G_누락7_소폭오프셋만_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치G_누락7_소폭오프셋만_디컴대조_원문.md` §4
- sig: `fn(usize, &game_core::PlayerState, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData) -> game_ai::plan_legacy::steal::StealAction`
- consts: [{"value": 1, "src_line": 286, "meaning": "Position 태그 1=Jungle — 정글러만 (본문 `shl 1` 은 L294 hp*2 항목)", "kind": "태그", "ev": 4}, {"value": 2, "src_line": 291, "meaning": "Option<StealSession> None 니치 = 2 (+0x1ba) → in_session_target None. 본문 다른 2 = player_champion bounds·StealAction Commit 태그 · tcx 정본 대조(19차 배치D: DWARF m09.ll:75569 `!4971 = DISCR_EXACT … extraData: i64 2`(Option<StealSession> None) · tcxdict TeamPlan 0x1ba Niche)", "kind": "센티널", "ev": 3}, {"value": 1, "src_line": 294, "folded_from": 2, "meaning": "my_champ.hp*2 < stat_cached.hp (=hp<50%) 이면 None — 계수 2 가 `shl i64 %29, 1` 로 접힘", "
- 0.5.8 logic 전문:
```
fn should_steal_now(version, player, goal_data, team_plan, data) -> StealAction
  // L286~287
  if player.info.position != Jungle { return None }
  let Some(my_champ) = cache.player_champion[team][position] else { return None }
  // L291~299
  in_session_target = team_plan.current_steal_session.map(|s| s.target)     // None=2 / Epic=0 / Serpen=1
  if my_champ.hp*2 < my_champ.stat_cached.hp { return None }                // L294 hp<50%
  if in_session_target.is_none() && game.tick().saturating_sub(team_plan.last_battle_tick) < tps*5 { return None }   // L299
  // L304~310: 캠프 존에 있을 수 있는 적 수 (5슬롯 count)
  epic_zone_short  = !epic_exists(ctx)   || (0..5).filter(|p| enemy_could_be_at_camp(team, cache, team_plan, p, 288000, 288000)).count() < 3   // L329 조건으로 소비
  serpen_zone_short= !serpen_exists(ctx) || (0..5).filter(|p| enemy_could_be_at_camp(team, cache, team_plan, p, 672000, 672000)).count() < 3   // L358
  // L315~316
  epic_alive   = epic_exists && mob.epic.live_list.len != 0
  serpen_alive = serpen_exists && mob.serpen.live_list.len != 0
  // L319~345: 에픽 후보
  epic_action: (StealAction, usize) =
    if in_session_target == Some(Epic) {                                         // L320~321
      if epic_alive && objective != Morgard { evaluate(Epic, in_session=true) } else { (None, MAX) }
    } else {
      epic_eligible = mob.epic.live_list.get(0) → get_entity_by_id → is_some_and(|e| e.hp*100 < e.stat_cached.hp*95)   // L323~325
      if epic_alive && objective != Morgard(0)                                     // L326~327
         && !obj_spawn.epic_giveup_tick.is_some_and(|t| t + tps*5 >= tick) && !epic_zone_short   // L328~329
         && epic_eligible                                                          // L333
      { evaluate(Epic, in_session = (in_session_target==Some(Epic)) → 여기선 false) } else { (None, MAX) }   // L334~345
    }
    where evaluate(Epic, s) = evaluate_steal_for_target(player, cache, ctx, my_champ, target=false, epic_entity, goal_data.epic.last_epic_hp, goal_data.epic.epic_enemy_tick, goal_data.epic.last_epic_seen, (288000,288000), s)
  // L348~370: 세르펜 후보 (대칭)
  serpen_action =
    if in_session_target == Some(Serpen) { if serpen_alive && objective != Serpen(1) { evaluate(Serpen, true) } else { skip } }   // L350
    else if serpen_alive && objective != Serpen && !serpen_giveup_tick.is_some_and(|t| t+tps*5 >= tick) && !serpen_zone_short && serpen_eligible(hp<95%) { evaluate(Serpen, false) } else { skip }   // L352~362
    where evaluate(Serpen, s) = evaluate_steal_for_target(..., target=true, serpen_entity, goal_data.serpen.last_epic_hp, .epic_enemy_tick, .last_epic_seen, (672000,672000), s)
  // L376~390: 합성
  if epic_action.0 == Commit { return epic_action.0 }                     // 에픽 Commit 최우선
  if serpen skipped { return if epic.0==Lurk { Lurk(Epic) } else { None } }
  if serpen_action.0 == Commit { return Commit(Serpen) }
  match (epic.0, serpen.0) {
    (Lurk, Lurk) => if in_session_target.is_none() { Lurk(if epic.1 > serpen.1 { Serpen } else { Epic }) }   // L383: 적 예상 처치 잔여틱(enemy_tick) 작은 쪽
                    else { Lurk(in_session_target) }                                                          // L380
    (_, Lurk) => Lurk(Serpen),  (Lurk, _) => Lurk(Epic),  _ => None }
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d2e500` → `fa3980` PassiveJunglePlan::sub_plan (i=91 · 변경·심층(r21 §D: w3 §2))
- 0.5.8 src: `game-ai\src\plan_legacy\old\passive_jungle.rs:134` · one_line: 패시브 정글 플랜의 서브플랜 — check_recall(인라인)이 참이면 Recall, 아니면 Jungle{team,camp,check_move:false}
- 0.6.0 판정: **심층(r21)** · 패치 요지: v3 undying → 매복블록 직행 · fb12a0 recall_need · **cj_ambush Some 이면 적 정글러 kill 시간 vs 내 이동+tps/2 → Jungle/Hide{bush,stealth,out_line 1}**
- RE 정본: `2026-09-17_r21_심층_w3_정글3_JungleSubPlan_PassiveJungle_역정글매복_원문.md` · 절: w3 §2
- sig: `fn(&game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan`
- consts: [{"value": 14400000001, "src_line": 157, "meaning": "120000^2 + 1 — champ↔캠프 좌표 distance_sq < 14400000001 = 거리 ≤ 120000(3.75셀) 이면 '캠프에 붙어 있음'", "kind": "임계", "ev": 4}, {"value": 1000, "src_line": 162, "meaning": "틱당 기대피해 = expected_damage*1000 / attack_cooltime (×1000 스케일). die_tick = hp*1000 / 합산 (L164, L197)", "kind": "계수", "ev": 4}, {"value": 0, "src_line": 164, "meaning": "합산 피해 0 이면 die_tick = usize::MAX(-1) (L164) / vamp > 0, heal > 0 판정 (L172, L174, L177)", "kind": "센티널", "ev": 4}, {"value": -1, "src_line": 164, "meaning": "usize::MAX (die_tick 무한) · Option 니치 None 태그(skill_effect/attac
- 0.5.8 logic 전문:
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
  L176~178: || champ.skill2_effect()[level>2 게이트].as_ref().map(같은 식).unwrap_or(false)
  (IR 순서: vamp → skill1 heal → skill1 buff → skill2 heal → skill2 buff, 단락 평가)
L180: is_in_fight = get_jungle_live_list(self.jungle, self.team==0).iter().any(|id| L181 get_entity_by_id(id).is_some_and(|e| L182 e.ty == Jungle && L184 e.ty.info.focused == Some(champ.id)))
L191: if is_in_fight {
  L192: camp_live = get_jungle_live_list(…)  (3번째 호출)
  L193~195: edpt = max(camp_live.iter().filter_map(get_entity_by_id).map(|e| e.attack_effect.as_ref().unwrap().expected_damage_target(ctx,e,champ)*1000 / e.attack_cooltime()).sum(), 1)   (★attack_effect None 이면 unwrap 패닉 — L162 와 달리 unwrap_or 없음)
  L197: die_tick = champ.hp * 1000 / edpt
  L199: return die_tick <= tps   // 1초 안에 죽으면 Recall, 아니면 Jungle
} else {
  L205: hp_ratio = champ.hp * 100 / champ.stat_cached.hp   (max 0 → div_by_zero panic)
  L206: return if can_heal { hp_ratio < 21 } else { hp_ratio < 41 }
}
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
