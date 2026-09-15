---

### `206` PassiveLinePlan::update — 라인 패시브 플랜 매 틱 갱신 — L222 check_recall 로 self.in_recall(귀환 여부) 결정(배치 H) → 팀 목표 Gank 라인 보정·v46 라인귀환 stage1/2·도주 게이트(update_v46_lane_recall/update_v46_flee 인라인, L232~248 · 배치 I) → 라인 위치·채팅 push(L254~304 · 배치 J). 반환 없음(unit) — 출력은 &mut self 필드뿐.

| 항목 | 값 |
|---|---|
| id | `passive_line__PassiveLinePlan__update` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12passive_lineNtB2_15PassiveLinePlan6update` |
| 소스 | `game-ai\src\plan_legacy\old\passive_line.rs:219` |
| IR | `m04.ll` 19038~24726행 |
| 경로·가시성 | `game_ai::plan_legacy::old::PassiveLinePlan::update` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d28800` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData)
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[206]/sig/tls/<키>`)**

- `name`: 없음
- `role`: -
- `key`: -
- `layout`: -
- `invalidation`: -
- `call_conditions`: 함수 본체 19038~24726 전체에서 TLS fn-포인터 상수 참조 0건 확인(m04.ll 의 `@anon.* = constant ptr @…call_once` 3개 = HP_VALUE_MEMO(.221)·CHAMP_POWERS_MEMO(.235)·LAST_STAND_MEMO(.248) 를 본문에서 grep → 0). H 범위·전 범위 모두 직접 TLS 접점 없음. 콜리(buy_item·upgrade_item·has_line_lead·v46_stage1/2 등) 내부 TLS 는 각 명세 소관.

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut PassiveLinePlan (280B, align 8) | 쓰기 표면 전수는 각 배치 writes 합본으로 — H 는 in_recall 만 | 4 |
| 1 | 2 | version | usize | IR 속성: noundef 만 | 4 |
| 2 | 3 | rnd | &mut StdRng (320B, align 16) | IR 속성 noalias·dereferenceable(320) | 4 |
| 3 | 4 | player | &PlayerState (2528B) |  | 4 |
| 4 | 5 | data | &OperationData (24B) |  | 4 |
| 5 | 6 | team_plan | &TeamPlan (1064B) | TeamPlan 크기는 tcxdict 값(1064B); IR 속성엔 없음 | 3 |
| 6 | 7 | _positioning_score | &PositioningScoreData (2760B) |  | 4 |
| 7 | 8 | debug | &mut DebugFrameData (224B) | 직접 store 0 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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
// L1090    fm = blackboard[team].minion_state(self.line).front_minion.and_then(|id| game.get_entity_by_id(id))  (19439~19479; minion_state 스위치 ai_interface.rs:379~382 top+0/mid+0x28/bottom+0x50, vtable +0x1f0)
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

**`mem` 메모리 접근 160건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 19164~19165. team<2 bounds check(19167, 실패=panic_bounds_check 19186) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag(i32) | r | 19172~19174 as_index 로 player_champion 슬롯 인덱스 · 20688 `==1`(Position::Jungle 태그 1 · tcxdict --enum Position) = line_strategy 분기 | 3 | OK |  |
| 2 | PlayerState | 0x4a8 | info.items.len | r | 20990~20994 L1405 items.is_empty() → item_score 5 | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | 19175 &AbstractGameWithCache | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | 19194~19195 &GameContext | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard | r | 19439~19440 · 19574~19575 &[Blackboard;2] (팀 인덱스 stride 744) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr / +0x8 game.vtable_ptr | r | 19263~19265 (&dyn AbstractGame 팻포인터) → buy_item/upgrade_item 인자, vtable 슬롯 호출 +0x28 tick(19883~19885·21144~21146) · +0x40 get_game_mode(21051~21053) · +0x1f0 get_entity_by_id(19474~19477·19618~19620) [divtable AbstractGame] | 3 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | 19176~19179 (+480 + team*40 + pos*8) champ unwrap(실패 19218). 19834 [1-team] 5슬롯 = near_enemies 원천, 19706 [team] = near_allies 원천 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x180 | top_tower/mid_tower/bottom_tower[team] (line*0x20 + 0x180 + team*8) | r | 19301~19306 `tower(line,team)` 1차 · 19897 · 20372~20374 (적 팀) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 9 | AbstractGameWithCache | 0x190 | top_tower2/mid_tower2/bottom_tower2[team] (line*0x20 + 0x190 + team*8) | r | 19307~19311 `.or(2차 타워)` · 19898 · 20375~20377 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 10 | AbstractGameWithCache | 0x130 | twin_towers[team] (bumpalo Vec: +0 ptr, +0x18 len) | r | 19316~19323 (team) L1102 min_by_key · 19930~19932 (closure$13) · 20380~20385 (1-team) L1243 `.iter().next()` | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x170 | nexus[team] | r | 19543~19547 · 19558~19560 (team) unwrap(실패 19553/19585) · 20424~20426 (1-team) | 4 | OK |  |
| 12 | GameContext | 0x0 | pool(&Bump) | r | 19721 → from_iter_in 할당자(19722·19871) | 4 | OK |  |
| 13 | GameContext | 0x8 | setting(&GameSetting) | r | 19334~19335 · 19502~19503 | 4 | OK |  |
| 14 | GameContext | 0x20 | map(&MapDef) | r | 19197~19198 fountain | 4 | OK |  |
| 15 | GameContext | 0x30 | item_list(&Vec<Box<dyn ItemInfo>>: +0x8 ptr, +0x10 len) | r | 20972~20984 · 20999~21000 L1395 index(bounds 실패 21014) | 4 | OK |  |
| 16 | GameContext | 0x3b | debug(bool) | r | 19996~19998 L1160 디버그 로그 게이트 | 4 | OK |  |
| 17 | GameSetting | 0x12f8 | tick_per_second | r | 19504~19506 tps*2(shl 1) · 20006~20008 tps*30 · 20630 remain>tps | 4 | OK |  |
| 18 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | 20003~20004 is_line_phase | 4 | OK |  |
| 19 | GameSetting | 0x1368 | return_tick | r | 20449~20450 L1248 귀환 소요 틱 | 4 | OK |  |
| 20 | MapDef | 0x6d70 | fountains[team] (+0 lx, +8 ly, +0x10 rx, +0x18 ry) | r | 19200~19226 map_def.rs:235 fountain(team) | 4 | OK |  |
| 21 | Entity | 0x660 | x | r | 19209 champ · 다수(거리 계산 전부) | 4 | OK |  |
| 22 | Entity | 0x668 | y | r | 19228 champ · 다수 \| (배치 J) 24276~24277 등 5회 → is_near_line 3번 인자 | 4 | OK |  |
| 23 | Entity | 0x670 | hp | r | 19241~19242 · 19252~19253 champ hp · 20282 콤보뎀 비교 · 20753 적 hp% · 20828 아군 hp% | 4 | OK |  |
| 24 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | 19243~19244 · 19254~19256 (0 이면 panic_const_div_by_zero 19291) · 20740 · 20815 \| (배치 I) 21234(L237 분모) · 21823(:593) · 21876(:618 full_hp) · 23566(update_v46_flee:471) \| (배치 J) 24196~24197(L274) · 24649~24650(L263) · 0 이면 panic_const_div_by_zero(24497·24668) | 4 | OK |  |
| 25 | Entity | 0x3f0 | stat_buff_cached.vamp(i32) | r | 19724~19726 champ >0 = has_self_heal · 20505~20507 아군 | 4 | OK |  |
| 26 | Entity | 0x4c8 | skill_effect Option<Effect> (None 니치 = +0x4f8 casting i32 == -1) | r | 19731~19737 champ · 20213~20219 적(closure$15) · 20512~20518 아군(closure$19) | 4 | OK |  |
| 27 | Entity | 0x500 | skill2_effect Option<Effect> (레벨>2 일 때만 참조, 아니면 정적 None @anon.16 · None 니치 = +0x530 == -1) | r | 19764~19773 entity.rs:1693 skill2_effect() · 20233~20241 · 20558~20566 | 4 | OK |  |
| 28 | Entity | 0x490 | attack_effect Option<Effect> (None 니치 = +0x4c0 == -1) | r | 20198~20204 (closure$15 콤보뎀) · aux 13049~13050 | 4 | OK |  |
| 29 | Entity | 0x538 | ult_effect Option<Effect> (레벨>4 일 때만, 아니면 @anon.16 · None 니치 = +0x568 == -1) | r | 20260~20266 entity.rs:1701 ult_effect() | 4 | OK |  |
| 30 | Entity | 0x5c8 | level | r | 19764~19766 >2 · 20252~20260 >4 | 4 | OK |  |
| 31 | Entity | 0x68 | ty@tag (EntityType: 2=Tower · 13=Champion) | r | 20438~20440 적 타워 ==2 · 20452~20454 champ ==13 | 4 | OK |  |
| 32 | Entity | 0x70 | ty@Champion.0.action_state@tag (ChampionActionState: 1=Return) | r | 20618~20620 entity.rs:1656 return_time() | 4 | OK |  |
| 33 | Entity | 0x78 | ty@Champion.0.action_state@Return.time | r | 20624~20627 귀환 경과 틱 | 4 | OK |  |
| 34 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag (Option<(usize,usize)>, 0=None) | r | 20635~20637 L1255 적 타워가 표적을 잡고 있나 | 4 | OK |  |
| 35 | Blackboard | 0x0 | top_minion_state / +0x28 mid / +0x50 bottom (BrainMinionParameter 40B): +0 front_minion@tag, +8 front_minion id, +0x20 minion_count(i32) | r | 19446~19462 · 19578~19598 · 20650~20667 ai_interface.rs:379~382 minion_state(line) 스위치(LineType 0/1/2, default 19375 unreachable) | 4 | OK |  |
| 36 | PassiveLinePlan | 0x116 | line(LineType 태그 0 Top/1 Mid/2 Bottom) | r | 19286~19287 · 19443~19444 · 19383(get_start_position 인자) · aux 23506 | 4 | OK |  |
| 37 | Box<dyn ItemInfo> | 0x70 | vtable slot tier() -> i64 | r | 21005~21010 · 21022 · 21030 (divtable ItemInfo 0x70 = tier). 4→100 · 3→70 · 2→60 · 그외→30 | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 38 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 21071~21078 L1437 is_epic = get_game_mode().as_moba().map_or(false, \|m\| len!=0) | 4 | OK |  |
| 39 | upgrade_item sret(24B) | 0x0 | Option 태그(i64 0/1, trunc nuw) / +0x10 item index | r | 19276~19279 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 40 | buy_item ret {i64,i64} | 0x0 | .0 == 1 → Some | r | 19270~19272 · 20894 L1392 is_none() | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 41 | effect_buff_target sret(288B = Option<BuffState>) | 0x48 | duration@tag (i32 -1 = None) / +0x80 vamp(i32) | r | 19781~19787 · 19818~19824 · 20548~20552 · 20599~20603 → Some && vamp>0 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 42 | TeamPlan | 0x41f | objective@tag (Option<MainObjective> 니치 태그 1B) | r | 21202~21205 `%907 = gep %5, 1055` · `and i8 -2` 후 `icmp eq 8` → 태그 8(Gank)\|9(Dive) · tcxdict --enum MainObjective: Gank=8 Dive=9 (메모리태그=선언discr) | 3 | OK |  |
| 43 | TeamPlan | 0x420 | objective@Some.0@Gank.line / @Dive.line (LineType 1B) | r | 21209~21210 `%912 = gep %5, 1056` · DI 이름 gank_line · Gank·Dive 둘 다 line 이 enum+0x1 | 4 | OK |  |
| 44 | PassiveLinePlan | 0x116 | line (LineType) | r | 21213(L236 gank_line 비교) · 21362(:503 cache.tower) · 21519(:513 minion_state) · 21775(:588) · 22092(:638 push) · 22130(:639) · 22643(v46_flee_end :325 push 원소) · 23228(update_v46_flee :374) 등 | 4 | OK |  |
| 45 | PassiveLinePlan | 0x111 | v46_commit (bool) | r | 21620~21622(:528 커밋중 분기) · 22414(v46_clear:477 completed=false 판) · 22469~22471(L244) · 22602(update_v46_flee:345) | 4 | OK |  |
| 46 | PassiveLinePlan | 0x38 | v46_committers.buf.ptr | r | 22367(:534 only=Some(&self.v46_committers[..]) 슬라이스 ptr) | 4 | OK |  |
| 47 | PassiveLinePlan | 0x40 | v46_committers.len | r | 22369(:534 only 슬라이스 len) | 4 | OK |  |
| 48 | PassiveLinePlan | 0x30 | v46_committers (Vec<usize> 24B 통째) | r | 22079(:637 대입 전 drop_in_place<Vec<usize>>) · 22295(:647 로그 `{:?}`) | 4 | OK |  |
| 49 | PassiveLinePlan | 0x0 | v46_flee_entry@tag (Option<(usize,u8)> 판별자 8B · 0=None 1=Some) | r | 22488(L241 take) · 22625(:346) · 22927(:350) · 23130(:357) · 23976(:393) — v46_flee_end 인라인 5곳 전부 `take()` = load 후 store 0 | 4 | OK |  |
| 50 | PassiveLinePlan | 0x8 | v46_flee_entry@Some.0.0 entry_tick (usize) | r | 22490 등 take 5곳 | 4 | OK |  |
| 51 | PassiveLinePlan | 0x10 | v46_flee_entry@Some.0.1 wave_flags (u8) | r | 22492 등 take 5곳 | 4 | OK |  |
| 52 | PassiveLinePlan | 0x114 | v46_flee_approached (bool) | r | 22501(v46_flee_end:324 flags \|= approached as u8) · 24060(:390 로그 `approached={}`) | 4 | OK |  |
| 53 | PassiveLinePlan | 0x100 | v46_flee_hp_entry (usize) | r | 22548(v46_flee_end:326 hp_min < hp_entry 비교) 외 4곳 | 4 | OK |  |
| 54 | PassiveLinePlan | 0x108 | v46_flee_hp_min (usize) | r | 22546(v46_flee_end:326) · 23221(update_v46_flee:366 min 갱신) | 4 | OK |  |
| 55 | PassiveLinePlan | 0x112 | v46_flee (bool) | r | 23115·23122(update_v46_flee:365 도주 진행중 분기) | 4 | OK |  |
| 56 | PassiveLinePlan | 0x50 | v46_flee_threats.buf.ptr | r | 23687·23810(update_v46_flee:385·:400 threats.iter()) | 4 | OK |  |
| 57 | PassiveLinePlan | 0x58 | v46_flee_threats.len | r | 23689·23811(:385·:400) · 24052 등 clear 시 store | 4 | OK |  |
| 58 | PassiveLinePlan | 0x48 | v46_flee_threats (Vec<usize> 24B 통째) | r | 23513(:459 대입 전 drop_in_place<Vec<usize>>) · 23592(:468 로그 `threats={:?}`) | 4 | OK |  |
| 59 | PassiveLinePlan | 0x60 | v46_pending.trigger_ticks.cap | r | 22106(push 용량 비교 · :638) | 4 | OK |  |
| 60 | PassiveLinePlan | 0x68 | v46_pending.trigger_ticks.ptr | r | 22115~22116(:638 push 쓰기 위치) | 4 | OK |  |
| 61 | PassiveLinePlan | 0x70 | v46_pending.trigger_ticks.len | r | 22101~22102(:638) | 4 | OK |  |
| 62 | PassiveLinePlan | 0x78 | v46_pending.flee_episodes.cap | r | 22523 등(v46_flee_end:325 push ×5 사이트) | 4 | OK |  |
| 63 | PassiveLinePlan | 0x80 | v46_pending.flee_episodes.ptr | r | 22532~22533 등 | 4 | OK |  |
| 64 | PassiveLinePlan | 0x88 | v46_pending.flee_episodes.len | r | 22518~22519 등 | 4 | OK |  |
| 65 | PlayerState | 0x930 | info.team (usize) | r | 배치 H 로드(%64) 재사용 · aux m01.ll:23845~23846 에서 재로드(blackboard[1-team] 인덱스) | 4 | OK |  |
| 66 | PlayerState | 0x9c0 | info.position@tag (i32) | r | 배치 H 로드(%68/%69) 재사용 — v46_stage1 의 pos 인자·로그 `{:?}` | 4 | OK |  |
| 67 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B) | r | 22022 `gep %1219, 384` — player_by_champion_id(best_id) 로 얻은 상대 PlayerState 의 parameter → aggressive_ratio() | 4 | OK |  |
| 68 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | %70(배치 H 로드) 전역 재사용 | 4 | OK |  |
| 69 | OperationData | 0x8 | context (&GameContext) | r | %79(배치 H 로드) 재사용 · 21336·22607 에서 +8 setting 로드 | 4 | OK |  |
| 70 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | 21515~21517(:513) · 23380~23382(update_v46_flee:446) · 23632~23634(:382) · 인덱스 = team(%64) | 4 | OK |  |
| 71 | GameContext | 0x0 | pool (&Bump) | r | 21617(:526 from_iter_in 의 bump) | 4 | OK |  |
| 72 | GameContext | 0x8 | setting (&GameSetting) | r | 21336~21337(:498 is_line_phase) · 21418(:505 get_start_position 인자) · 22607~22608(update_v46_flee:345) | 4 | OK |  |
| 73 | GameContext | 0x3b | debug (bool) | r | 22157~22160(:645) · 22408~22411(:537) · 23550~23553(update_v46_flee:467) · 23793~23796(:389) — 로그 게이트 | 4 | OK |  |
| 74 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick (usize) | r | 21339~21340 · 22610~22611 (is_line_phase setting.rs:703) | 4 | OK |  |
| 75 | GameSetting | 0x12f8 | tick_per_second (usize) | r | 21342~21344 · 22613~22615 (is_line_phase setting.rs:704 · ×30) | 4 | OK |  |
| 76 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame 팻포인터: data_ptr +0 · vtable_ptr +8) | r | 21281~21283(:489) · 22592~22593(update_v46_flee:344) · vtable+0x28 = tick() (21286~21288 · 22598~22600) · vtable+0x1f0 = get_entity_by_id (21569~21571 · 23423~23425 · 23666~23668 · 23723~23724·23738 · aux m04.ll:12106~12107·12128) — divtable AbstractGame 0x28→tick · 0x1f0→get_entity_by_id | 3 | OK |  |
| 77 | AbstractGameWithCache | 0x130 | twin_towers[team] (bumpalo Vec<&Entity> 32B: ptr +0 · len +24) | r | 21381~21389(:504) · 23246~23253(update_v46_flee:375) · `getelementptr {{ptr,ptr,i64},i64}, %70+304, team` | 4 | OK |  |
| 78 | AbstractGameWithCache | 0x170 | nexus[team] (Option<&Entity>) | r | 21535~21537(:510 최후 폴백) · 22908~22910(update_v46_flee:355) | 4 | OK |  |
| 79 | AbstractGameWithCache | 0x180 | top_tower[team] / +0x190 top_tower2 (line*32 stride: mid 0x1a0/0x1b0 · bottom 0x1c0/0x1d0) | r | 21367~21377(:503 cache.tower(line,team) 인라인 = <line>_tower[team].or(<line>_tower2[team])) · 23233~23243(update_v46_flee:374) · aux m04.ll:15380~15389(적 타워 = [1-team]) · 아웃오브라인 `AbstractGameWithCache::tower` 호출 21806·21854 도 같은 필드 | 4 | OK |  |
| 80 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] / [1-team][0..5] (Option<&Entity>) | r | %73(배치 H) 재로드 21226·21291·22710 = 내 챔피언 · 21579~21580(:518 `[5 x ptr]` 인덱스 1-team = 적 5칸 순회) | 4 | OK |  |
| 81 | Blackboard | 0x0 | top_minion_state / +0x28 mid / +0x50 bottom (BrainMinionParameter 40B · minion_state(line) switch) | r | 21521~21556(:513) · 21777~21802(:588) · 22132~22147(:639) · 23387~23409(update_v46_flee:446) · 23638~23653(:382) · switch line 0→+0 1→+40 2→+80 | 4 | OK |  |
| 82 | BrainMinionParameter | 0x0 | front_minion@tag (Option<usize> 8B) / +0x8 front_minion id | r | 21557~21565 · 23411~23419 · 23654~23662 → get_entity_by_id(id) | 4 | OK |  |
| 83 | BrainMinionParameter | 0x10 | from_mid (i64) | r | 22148~22151(:639~640 >0 → 상대반쪽) · 23461~23464(update_v46_flee:456 >0 → wave_flags\|4) | 4 | OK |  |
| 84 | BrainMinionParameter | 0x18 | minion_power (i64) | r | 21803~21805(:588) · 21818·21830·21841(:594·:602 부호 비교) · 23431~23433·23443~23445(update_v46_flee:448 >0) | 4 | OK |  |
| 85 | Entity | 0x670 | hp (usize) | r | 21236(L237 gank_hold 분자) · 21660(:555 my_hp) · 22362(:535) · 23222(update_v46_flee:366) · 23530(:461) · 23565(:471) — 위치 산술 없이 직접 | 4 | OK |  |
| 86 | Entity | 0x660 | x (u64) | r | 21303(:497 in_heal) · 21466·21725 등 distance_sq 인라인 다수 | 4 | OK |  |
| 87 | Entity | 0x668 | y (u64) | r | 21326(:497) · 21470 등 | 4 | OK |  |
| 88 | Entity | 0x4c0 | attack_effect@tag (Option<Effect> 니치 i32 · -1=None) | r | 21766~21770(:584) · 21866~21870(:603) · 23018~23021(update_v46_flee:360) · 23874~23877(:402) · aux 12151~12154 · 15399~15401 | 4 | OK |  |
| 89 | Entity | 0x490 | attack_effect@Some.0 (Effect · range +0x4a0 · growth_range +0x4a8) | r | Effect::range(caster) 인라인 = caster.stat_buff_cached.range(0x438) + effect.range(0x4a0) + (caster.level(0x5c8)-1)*effect.growth_range(0x4a8) — 23033~23036·23046~23049·23101~23104(update_v46_flee:361) · 23885~23896(:402) · aux 594~597·607~610·662~665 등 | 4 | OK |  |
| 90 | Entity | 0x438 | stat_buff_cached.range (usize) | r | Effect::range 항 (effect.rs:26) | 4 | OK |  |
| 91 | Entity | 0x5c8 | level (usize) | r | Effect::range 항 (level-1)*growth_range | 4 | OK |  |
| 92 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | Entity::radius() 인라인(entity.rs:1511~1515): mult==0 → radius(0x680) / else radius*(mult+100)/100 — 23052~23070 · 23903~23921 · 23927~23942 · aux 613~631 등 | 4 | OK |  |
| 93 | Entity | 0x680 | radius (usize) | r | Entity::radius() 본체 | 4 | OK |  |
| 94 | Entity | 0x640 | stat_cached.move_speed (usize) | r | 23951~23953(update_v46_flee:404 speed*reaction) · aux 12228~12230(:574 speed*cs_lock) | 4 | OK |  |
| 95 | Entity | 0x68 | ty@tag (EntityType 8B · 13=Champion) | r | aux m01.ll:23867~23869 (Entity::is_in_return entity.rs:1648 인라인) | 4 | OK |  |
| 96 | Entity | 0x70 | ty@Champion.0.action_state@tag (ChampionActionState 8B · 1=Return) | r | aux m01.ll:23870~23873 (is_in_return = Champion && action_state==Return) | 4 | OK |  |
| 97 | TeamPlan | 0x41f | objective@tag (Option<MainObjective> 니치 태그) | r | m04.ll:24143~24145 `%2051 = gep %5, 1055` → 배치 H 의 %907 재사용(22571) · `and i8 -2` 후 `== 8` = 태그 8(Gank) 또는 9(Dive) · tcxdict --enum MainObjective: Gank=8 Dive=9 (메모리태그=선언discr) | 3 | OK |  |
| 98 | TeamPlan | 0x420 | objective@Some.0@Gank.line / @Dive.line (LineType) | r | 24144~24145 `%2052` · DI 이름 gank_line · Gank·Dive 둘 다 line 이 enum+0x1 이라 태그 무관 같은 바이트 | 4 | OK |  |
| 99 | PassiveLinePlan | 0x116 | line (LineType 태그) | r | 24155~24156 %2054 · 24320/24363/24406/24449 재로드(closure$1 안) · 24470 %2155(L283 타워 조회) | 4 | OK |  |
| 100 | PassiveLinePlan | 0x110 | in_recall (bool) | r | 24177 · 24183 (L271 게이트: 인접라인 && in_recall) | 4 | OK |  |
| 101 | PassiveLinePlan | 0x28 | chats.len | r | 24612~24613 %2212(L299) · 24683~24684 %2241(L269) — Vec::push 인라인 | 4 | OK |  |
| 102 | PassiveLinePlan | 0x18 | chats.buf.inner.cap | r | 24617 %2213 · 24688 %2242 — len==cap 이면 grow_one | 4 | OK |  |
| 103 | PassiveLinePlan | 0x20 | chats.buf.inner.ptr | r | 24626~24627 %2218 · 24697~24698 %2247 — 원소 쓰기 목적지 | 4 | OK |  |
| 104 | PlayerState | 0x930 | info.team (%64) | r | 정의는 배치 H 19164~19165(L222) — 배치 J 에서 `1 - team`(24212)·TeamType::Player(team) 비교(24568)·player_champion[team] 인덱스에 사용 | 4 | OK |  |
| 105 | PlayerState | 0x9c0 | info.position (%69, i32 태그) | r | 정의는 배치 H 19172~19174 — %73 = &cache.player_champion[team][position] 의 인덱스 | 4 | OK |  |
| 106 | OperationData | 0x0 | cache (&AbstractGameWithCache) (%70) | r | 정의는 배치 H 19175 — 배치 J 에서 player_champion·tower·game 팻포인터 읽기의 베이스 | 4 | OK |  |
| 107 | OperationData | 0x8 | context (&GameContext 64B) (%79) | r | 정의는 배치 H 19195 — is_near_line 의 1번 인자(24278 등 5회) | 4 | OK |  |
| 108 | OperationData | 0x10 | blackboard (&[Blackboard;2]) (%2088) | r | 24220~24221 · `%2089 = gep <Blackboard>, %2088, i64 %2082` = &blackboard[1 - team](적 팀 인덱스) → is_recent_visible 의 &self | 4 | OK |  |
| 109 | AbstractGameWithCache | 0x0 | game.data_ptr (&dyn AbstractGame 앞 8B) (%2084) | r | 24217 · is_recent_visible 2번 인자 · get_entity_by_id 의 self | 4 | OK |  |
| 110 | AbstractGameWithCache | 0x8 | game.vtable_ptr (%2086) | r | 24218~24219 · is_recent_visible 3번 인자 · 24536~24538 vtable+0x1f0(=496) 슬롯 로드 → divtable AbstractGame 0x1f0 = get_entity_by_id | 3 | OK |  |
| 111 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] (Option<&Entity>, 니치 null=None) | r | %73(배치 H 19176~19178 = 0x1e0 + team*40 + position*8) 를 24189(L273)·24642(L262)에서 load → null 이면 unwrap_failed 패닉 · 적 팀 슬롯 5개 = `%2083 = gep [5 x ptr], %71, %2082`(24214) 를 +0/8/16/24/32 로 언롤 순회(24254·24296·24339·24382·24425) | 4 | OK |  |
| 112 | AbstractGameWithCache | 0x180 | top_tower/mid_tower/bottom_tower[team] (Option<&Entity>) — 0x180 + line*0x20 + team*8 | r | 24474~24481 `shl line,5` + 384 + team*8 · L283 `cache.tower(line, team)` 인라인(dloc: scope `tower` simulation.rs line 0 → option.rs:1622 `or`) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 113 | AbstractGameWithCache | 0x190 | top_tower2/mid_tower2/bottom_tower2[team] — 0x190 + line*0x20 + team*8 | r | 24482~24485 (400 = 0x190) · `tower.or(tower2)`: 첫 타워 null 이면 둘째 타워(24486~24487 select) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 114 | Entity | 0x670 | hp (현재 HP) | r | 24206~24207(L274) · 24659~24660(L263) · hp_ratio = hp*100/stat_cached.hp | 4 | OK |  |
| 115 | Entity | 0x660 | x (적 챔피언 좌표) | r | 24274~24275 등 5회(closure$1 L278) → is_near_line 2번 인자 | 4 | OK |  |
| 116 | Entity | 0x68 | ty@tag (EntityType) | r | 24506~24508 타워 엔티티 `== 2`(Tower) L284 · 24546~24548 nearest_enemy 엔티티 `== 1`(Minion) L287 — tcxdict --enum EntityType: Tower=2 Minion=1 | 3 | OK |  |
| 117 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag (Option<(usize,usize)>) | r | 24513~24514 %2178 · trunc i1 → Some 판정 (L285) · = 0x70(Tower 페이로드 시작) + 0x18 | 4 | OK |  |
| 118 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 (entity id) | r | 24530~24531 %2182 (DI id) → get_entity_by_id(id) | 4 | OK |  |
| 119 | Entity | 0x0 | team@tag (TeamType) | r | 24554~24556 `== 0`(Player) L287 (entity.rs:1127 derive PartialEq 인라인) | 4 | OK |  |
| 120 | Entity | 0x8 | team@Player.0 (usize) | r | 24567~24568 `== %64`(내 팀) L287 | 4 | OK |  |
| 121 | PassiveLinePlan | 0x110 | in_recall | w | ★루트 L222 · store 21201(블록 %904 첫 3줄 — 블록 자체는 배치 I 가 잇는다). phi 21198: 분수 안&hp<최대 → 1 · 분수 안&만피&version>1 → 0 · 그 외 %887(21136) = 웨이브 위험 → 1 / 조기 return true 경로(%901) → 1 / 점수 경로 → score>49 \| (배치 I) 24140 (L245 · `if self.v46_commit { self.in_recall = true }`) — 배치 H 가 L222 에서 check_recall 결과를 저장한 것(21201)을 커밋 시 덮어쓴다 \| (배치 J) 24711 (L265 · 갱 라인==내 라인 && hp_ratio>24) · 24587 (L297 · 인접라인 && in_recall && hp_ratio>69 && (pushed_to_tower \|\| !my_line_enemy_visible)) | 4 | OK | check_recall 결과 i8 (0/1) |
| 122 | DebugFrameData(&mut debug) | 0x48 | logs (간접: add_log 호출) | w | 21150~21182 L1161~1162 · ctx.debug(+0x3b)==true && L1156 조건 성립 시 1회. 직접 store 없음(오프셋 0x48 은 tcxdict DebugFrameData.logs — 본문엔 gep 없음) | 3 | OK | "V46BASEHEAL T{team} {position:?} tick={tick} hp={hp_ratio}%" |
| 123 | StdRng(&mut rnd) | 0x0 | (간접) buy_item·upgrade_item 에 &mut 전달 | w | 19270·19275 · H 범위 직접 쓰기 0 | 4 | OK | - |
| 124 | PassiveLinePlan | 0x111 | v46_commit | w | 0: 21313(:491 champ None) · 21352(:499 in_heal \|\| !line_phase) · 22450(:541 커밋 유지 실패 still_vec 비었음) · 22478(L240 gank_hold) — 전부 v46_clear 인라인(:480) · 1: 22052(:636 커밋 확정) \| (배치 J) 24715 (L266) · 24591 (L298) — `self.v46_clear(true)` 인라인(passive_line.rs:476~481 · 477 주소계산 · 480 store) | 4 | OK | 0 / 1 |
| 125 | PassiveLinePlan | 0x40 | v46_committers.len | w | 21319·21358·22455·22484 — v46_clear:481 `Vec::clear` 인라인(len=0 · usize 라 drop 루프 없음) \| (배치 J) 24721 (L266) · 24597 (L298) — v46_clear 481 줄 `Vec::clear`(alloc vec/mod.rs:3035) 인라인 · cap/ptr 은 불변 | 4 | OK | 0 |
| 126 | PassiveLinePlan | 0x30 | v46_committers (Vec<usize> 24B 통째) | w | 22079(구 Vec drop_in_place) → 22088 memcpy 24B(:637) · 원소 = 커밋 위협 엔티티 id(c.0 · aux m12.ll:32998~33080 fold 가 elem+0 을 복사) | 4 | OK | committers.iter().map(\|c\| c.0).collect() |
| 127 | PassiveLinePlan | 0x68 | v46_pending.trigger_ticks 원소 (ptr[len] · 16B {tick i64 @0, line u8 @8}) | w | 22123~22125(:638 push) · 용량 부족 시 RawVec::grow_one(22111) 이 cap/ptr(0x60/0x68) 갱신 | 4 | OK | (tick, self.line) |
| 128 | PassiveLinePlan | 0x70 | v46_pending.trigger_ticks.len | w | 22126~22127(:638) | 4 | OK | len+1 |
| 129 | PassiveLinePlan | 0x80 | v46_pending.flee_episodes 원소 (ptr[len] · 16B {entry_tick i64 @0, line u8 @8, flags u8 @9}) | w | v46_flee_end:325 push — 22538~22542(L241) · 22677~22681(:346) · 22979~22983(:350) · 23182~23186(:357) · 24027~24031(:393) · grow_one 22528 등 | 4 | OK | (entry_tick, line, approached\|wave_flags) |
| 130 | PassiveLinePlan | 0x88 | v46_pending.flee_episodes.len | w | 22543~22544 등 5곳 | 4 | OK | len+1 |
| 131 | PassiveLinePlan | 0x90 | v46_pending.danger_ticks | w | 21784~21787(:578 exposed 일 때) | 4 | OK | +1 |
| 132 | PassiveLinePlan | 0x98 | v46_pending.veto_wave | w | 21834~21837(:595) | 4 | OK | +1 |
| 133 | PassiveLinePlan | 0xa0 | v46_pending.veto_crash | w | 21887~21890(:610) | 4 | OK | +1 |
| 134 | PassiveLinePlan | 0xa8 | v46_pending.veto_heal | w | 22042~22045(:631) | 4 | OK | +1 |
| 135 | PassiveLinePlan | 0xb0 | v46_pending.stage2_saves | w | 21719~21722(:563 stage2 생존) | 4 | OK | +1 |
| 136 | PassiveLinePlan | 0xb8 | v46_pending.commit_clears | w | 22459~22462 — v46_clear(completed=false):477~478 `if self.v46_commit { commit_clears += 1 }` (:541 커밋 유지 실패 경로만 · completed=true 인스턴스 4곳은 이 증가가 접혀 없음) | 4 | OK | +1 |
| 137 | PassiveLinePlan | 0xc0 | v46_pending.trigger_wave_enemy_half | w | 22152~22156(:640 from_mid>0 → gep %0, 192) | 4 | OK | +1 |
| 138 | PassiveLinePlan | 0xc8 | v46_pending.trigger_wave_my_half | w | 22152~22156(:640 from_mid<=0 → gep %0, 200) — select 192/200 으로 한 store | 4 | OK | +1 |
| 139 | PassiveLinePlan | 0xd0 | v46_pending.flee_triggers | w | 23538~23541(update_v46_flee:464 도주 진입) | 4 | OK | +1 |
| 140 | PassiveLinePlan | 0xd8 | v46_pending.flee_hold_ticks | w | 23542~23545(:465 진입 틱) · 23800~23803(:395 유지 틱) | 4 | OK | +1 |
| 141 | PassiveLinePlan | 0xe0 | v46_pending.flee_hold_acute_ticks | w | 23546~23549(:466 진입) · 24098~24101(:409 acute 유지) | 4 | OK | +1 |
| 142 | PassiveLinePlan | 0xe8 | v46_pending.flee_hold_refuge_ticks | w | 24105~24108(:412 under_refuge 유지) | 4 | OK | +1 |
| 143 | PassiveLinePlan | 0xf0 | v46_pending.flee_hold_cover_ticks | w | 24127~24130(:427 cover 유지) | 4 | OK | +1 |
| 144 | PassiveLinePlan | 0xf8 | v46_pending.flee_nohit_episodes | w | v46_flee_end:327 (hp_min >= hp_entry 일 때) — 22553~22556 · 22692~22695 · 22994~22997 · 23197~23200 · 24041~24044 | 4 | OK | +1 |
| 145 | PassiveLinePlan | 0x100 | v46_flee_hp_entry | w | 23532~23533(update_v46_flee:461) | 4 | OK | champ.hp |
| 146 | PassiveLinePlan | 0x108 | v46_flee_hp_min | w | 23534~23535(:462 진입) · 23226~23227(:366 유지 갱신 umin) | 4 | OK | champ.hp / min(champ.hp, hp_min) |
| 147 | PassiveLinePlan | 0x0 | v46_flee_entry@tag | w | 0: 22493·22630·22932·23135·23981 (v46_flee_end:323 take) · 1: 23525(update_v46_flee:460) | 4 | OK | 0(take) / 1(Some) |
| 148 | PassiveLinePlan | 0x8 | v46_flee_entry@Some.0.0 entry_tick | w | 23526~23527(:460) | 4 | OK | tick |
| 149 | PassiveLinePlan | 0x10 | v46_flee_entry@Some.0.1 wave_flags | w | 23528~23529(:460) | 4 | OK | (wave_crashing?2:0)\|(from_mid>0?4:0) |
| 150 | PassiveLinePlan | 0x112 | v46_flee | w | 1: 23467(:457 진입) · 0: v46_flee_end:331 `store i32 0, %0+274` 가 0x112~0x115 4바이트를 한 번에 0 (22567·22706·23008·23211·24053) | 4 | OK | 1 / 0 |
| 151 | PassiveLinePlan | 0x113 | v46_flee_acute | w | 23469(:458 진입=1) · 23968~23969(:400 acute=false) · 24094~24095(:400 acute=true) · v46_flee_end i32 store 로 0 | 4 | OK | 1 / acute / 0 |
| 152 | PassiveLinePlan | 0x114 | v46_flee_approached | w | 23536~23537(:463 진입=1) · 24096~24097(:408 acute 유지 시 1) · v46_flee_end i32 store 로 0 | 4 | OK | 1 / 0 |
| 153 | PassiveLinePlan | 0x115 | v46_flee_cover | w | 24116~24118(:420 under_refuge 일 때 zext) · 24122~24123(:420 아니면 0) · v46_flee_end i32 store 로 0 | 4 | OK | cover(bool) / 0 |
| 154 | PassiveLinePlan | 0x48 | v46_flee_threats (Vec<usize> 24B 통째) | w | 23513(구 Vec drop_in_place) → 23522 memcpy 24B(update_v46_flee:459) | 4 | OK | committers(게이트 반환).iter().copied().collect() |
| 155 | PassiveLinePlan | 0x58 | v46_flee_threats.len | w | v46_flee_end:332 `Vec::clear` 인라인 — 22566·22705·23007·23210·24052 | 4 | OK | 0 |
| 156 | PassiveLinePlan | 0x28 | chats.len | w | 24637~24638 (L299) · 24706~24707 (L269) — Vec<Chat>::push 인라인 | 4 | OK | len+1 |
| 157 | PassiveLinePlan | 0x18 -> chats.cap / 0x20 -> chats.ptr | chats.buf (grow_one 재할당 시) | w | 24622 · 24693 `RawVec<Chat>::grow_one(&mut self.chats)` — len==cap 일 때만 호출 · 재할당 뒤 24627/24698 에서 ptr 재로드 | 4 | OK | 콜리가 갱신 |
| 158 | PassiveLinePlan | 0x20 -> chats.ptr[len]+0 (Chat 24B 원소) | Chat::HideLineToo(gank_line, 0) | w | 24632~24636 (L299) · tcxdict --enum Chat 12 = HideLineToo{enum+0x1 LineType, enum+0x8 usize} · 원소 나머지 바이트(+2..+7, +16..+23)는 미기록(패딩·live 아님) | 3 | 오귀속(사전은 다른 필드를 준다) | +0 = i8 12 · +1 = gank_line(i8) · +8 = i64 0 |
| 159 | PassiveLinePlan | 0x20 -> chats.ptr[len]+0 (Chat 24B 원소) | Chat::Cancel(CancelReason::LowHpSelf) | w | 24703~24705 (L269) · tcxdict --enum Chat 17 = Cancel{enum+0x1 CancelReason} · CancelReason 0 = LowHpSelf · +2..+23 미기록 | 3 | 오귀속(사전은 다른 필드를 준다) | +0 = i8 17 · +1 = i8 0 |

**`consts` 상수 62건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 100 | 1084 | 계수 | hp_ratio = hp*100/max_hp (19260) · 아군/적 hp% 합(20755·20830) · item_score 100(4티어, 21039) | 4 |  |
| 1 | 1 | 1095 | 임계 | tps*2 (= 2초) 가 `shl i64 %220, 1`(19506) 로 접힘 — enemy_minion_line_action_danger_damage_at 의 예측 틱 인자 | 4 | 2 |
| 2 | 46 | 1096 | 임계 | hp_ratio < 46 이면 웨이브 피해(≠0)만으로 귀환 확정 (20513) | 4 |  |
| 3 | 62500000001 | 1157 | 임계 | 250000²+1 — 쌍둥이 타워↔프론트 미니언 근접 판정(19991 `ult`) · aux 23561 L1145 적↔(front_minion or target) 근접 | 4 |  |
| 4 | 22500000001 | 1174 | 임계 | 150000²+1 — 적↔front_minion 근접 카운트(20121) · aux 23227 L1125 아군↔target · aux 23601 L1146 적↔target · aux 13108 L1202 킬 후보 필터 | 4 |  |
| 5 | 22500000000 | 1180 | 임계 | 150000² — closure$15 `dist_sq > 22500000000` 이면 제외(20192) = 150000 이내만 콤보뎀 계산(위 +1 과 동치 외연) | 4 |  |
| 6 | 70 | 1154 | 임계 | hp_ratio > 70 이면 타워없음·라인페이즈 검사 생략(19879) · L1255 적타워 표적 검사 게이트(20645 재사용) · heal_score 70(L1342·1356·1373·1380 등) | 4 |  |
| 7 | 30 | 704 | 계수 | is_line_phase: tick < first_spawn_tick − tps*30(20008) · item_score 30(기타 티어, 21035) | 4 |  |
| 8 | 51 | 1171 | 임계 | hp_ratio < 51 && 라인 페이즈 → 프론트 미니언 근처 적 수 검사 진입(20012) | 4 |  |
| 9 | 2 | 1247 | 태그 | EntityType::Tower 태그(20440) · level > 2 → skill2_effect 유효(19766·20235·20560·aux 13153) · item tier 2 → 60(21034) · [team<2 bounds 19167 는 인덱스 가드] | 4 |  |
| 10 | 13 | 1655 | 태그 | EntityType::Champion 태그(20454) return_time() 진입 | 4 |  |
| 11 | 4 | 1701 | 임계 | level > 4 → ult_effect 유효(20260·aux 13174) · item tier 4 → 100(21018) | 4 |  |
| 12 | 3 | 1398 | 태그 | item tier 3 → item_score 70(21026) (본문의 `shl …, 3` 은 포인터 8B 스트라이드 — 이 상수와 무관) | 4 |  |
| 13 | 60 | 1400 | 임계 | item tier 2 → 60(21035) · heal_score 60(L1303·1327·1342·1358·1382) · hp_ratio > 60 (L1382, 20920) | 4 |  |
| 14 | 80 | 1368 | 임계 | hp_ratio > 80 이면 0 아니면 80 (20906~20907) · heal_score 80(L1297·1317·1340·1374) | 4 |  |
| 15 | 31 | 1256 | 임계 | hp_ratio < 31 — 적타워 표적중 귀환(20670·20676) · heal_score 분기(L1301·1327·1340·1358·1374) | 4 |  |
| 16 | 21 | 1299 | 임계 | hp_ratio < 21 → heal 70 (20931·20951·20881) | 4 |  |
| 17 | 41 | 1303 | 임계 | hp_ratio < 41 → heal 60 / 70 (20939·20911) | 4 |  |
| 18 | 10 | 1286 | 계수 | enemy_hp_ratio_sum = Σ(적 hp%) + 10 (20769·20773 초기값 10) | 4 |  |
| 19 | 5 | 1405 | 산출값 | 아이템 0개 & 구매가능 → item_score 5(20995) (본문의 `shl …, 5` 는 line*32 타워 스트라이드 — 이 상수와 무관) | 4 |  |
| 20 | -10 | 1422 | 산출값 | panelty_score: 적이 라인 리드 → −10 (21043) | 4 |  |
| 21 | -5 | 1422 | 산출값 | panelty_score: 양쪽 리드 없음 → −5 (21043) | 4 |  |
| 22 | 49 | 1447 | 임계 | return score > 49 (21122) | 4 |  |
| 23 | 61 | 1439 | 임계 | hp_ratio < 61 && is_epic(에픽 생존) → 귀환 (21084) | 4 |  |
| 24 | 11 | 1439 | 임계 | hp_ratio < 11 → 무조건 귀환 (21086) | 4 |  |
| 25 | -1 | 742 | 센티널 | Option<Effect>/Option<BuffState> None 니치 태그 (19733 등 다수) | 4 |  |
| 26 | 1 | 1079 | 태그 | version > 1 (19237, reach 접힘 → 항상 참) · ChampionActionState::Return 태그(20620) · buy_item .0==1 Some(20894) (shl 시프트량 1 = tps*2 접힘은 위 folded_from 항목) | 4 |  |
| 27 | -2 | 232 | 계수 | `and i8 tag, -2` = 최하위 비트 마스크 — Gank(8)\|Dive(9) 를 한 비교로 접음 (21204) | 4 |  |
| 28 | 8 | 232 | 태그 | MainObjective 메모리태그 8 = Gank (마스크 후 ==8 이면 Gank 또는 Dive=9) — tcxdict --enum MainObjective (21205) | 3 |  |
| 29 | 100 | 237 | 계수 | HP 백분율 분자 `hp*100` (21242) · 같은 값이 :650(22265)·update_v46_flee:471(23565) hp% 로그, Entity::radius() 의 `(mult+100)/100`(23068·23070 등) 에도 쓰임 | 4 |  |
| 30 | 24 | 237 | 임계 | gank_hold HP% 임계 — `hp*100/max(max_hp,1) > 24` (21244) · 참이면 v46 귀환/도주 상태를 접고(L240~241) L243/248 을 건너뛴다 | 4 |  |
| 31 | 64000 | 496 | 산출값 | Game::heal_area(team) (game.rs:319) 인라인: team0 → rx=64000, team1 → ry=64000 (21298·21323 select) | 4 |  |
| 32 | 960000 | 496 | 산출값 | heal_area: team0 → ry=960000, team1 → rx=960000 (21298·21323) — 맵 대각 끝(30셀×32000) | 4 |  |
| 33 | 891999 | 497 | 임계 | in_heal x 하한(team1): `x > 891999` = x >= 892000 = 960000-68000 (21305) — team0 쪽 하한(64000-68000<0)은 saturating 으로 접혀 `team==0 \|\|` 로 남음 | 4 |  |
| 34 | 896000 | 497 | 임계 | in_heal y 하한(team0): `y < 896000` 이면 밖 = y >= 896000 = 960000-64000 (21328) — team1 쪽 하한(64000-64000=0)은 접힘 | 4 |  |
| 35 | 30 | 498 | 계수 | GameSetting::is_line_phase(tick) (setting.rs:703~704) = tick < first_spawn_tick(0x8a8).saturating_sub(tps*30) — 에픽 첫 스폰 30초 전까지가 라인 국면 (21344 · update_v46_flee:345 22615) | 4 |  |
| 36 | 62500000001 | 386 | 임계 | 250000²+1 — `distance_sq < 62500000001` = 거리 <= 250000(7.8셀) · update_v46_flee:386 위협 still 판정(23779) · aux :523 front_minion 근접(m01.ll:23923) · aux :524 nearest_tower 근접(23953) | 4 |  |
| 37 | 60 | 624 | 임계 | heal_die_gain = (full_hp-my_hp).saturating_sub *60 / max(best_dps,1) (22009) — dps 가 초당이면 60 = 틱/초 환산(추정 · tps 상수 하드코딩 여부는 소스 부재) | 5 |  |
| 38 | 1000 | 628 | 계수 | aggressive_ratio 의 천분율 스케일 — best_diff_bound = (1000-aggr)*80/1000 + 80 (22032·22034) | 4 |  |
| 39 | 80 | 628 | 임계 | best_diff_bound 기울기와 바닥 — 공격성 0 → 160, 1000 → 80 (22033·22035) | 4 |  |
| 40 | 13 | 521 | 태그 | EntityType 메모리태그 13 = Champion (aux m01.ll:23869 · Entity::is_in_return 인라인) | 4 |  |
| 41 | 1 | 521 | 태그 | ChampionActionState 메모리태그 1 = Return (aux m01.ll:23872 `icmp eq %62, 1`) — is_in_return = Champion && action_state==Return · 리터럴 그대로(shl 시프트량 아님 · 접힘 없음)(접힘 아님) | 4 |  |
| 42 | 2 | 456 | 임계 | wave_flags bit1 = wave_crashing(front 미니언이 적 타워 사거리 안) (23508 select 2/0) · 리터럴 그대로(shl 시프트량 아님 · 접힘 없음) | 4 |  |
| 43 | 4 | 456 | 임계 | wave_flags bit2 = from_mid>0(웨이브 상대반쪽) (23464 select 4/0) · 리터럴 그대로(shl 시프트량 아님 · 접힘 없음) · bit0 = approached 는 v46_flee_end:324 에서 OR | 4 |  |
| 44 | 6 | 399 | 임계 | reaction = champ.attack_duration() + 6 틱 (23805) — acute 판정의 `speed*reaction` 여유 · 리터럴 그대로(shl 시프트량 아님 · 접힘 없음) | 4 |  |
| 45 | -1 | 584 | 센티널 | Option<Effect> 니치 태그 None = i32 -1 (attack_effect@tag 0x4c0 · 21768 등) · Effect::range 의 `level-1`(23101 등)도 -1 리터럴 | 4 |  |
| 46 | -2 | 254 | 계수 | `and i8 tag, -2` = 최하위 비트 마스크 — Gank(8)\|Dive(9) 를 한 비교로 접음 (22572) | 4 |  |
| 47 | 8 | 254 | 태그 | MainObjective 메모리태그 8 = Gank (마스크 후 == 8 이면 Gank 또는 Dive=9) — tcxdict --enum MainObjective (22573) | 3 |  |
| 48 | 1 | 271 | 태그 | LineType 태그 1 = Mid — is_adj_line(player.rs:1006) 인라인: self.line == Mid 분기 (24167) · phi 상수 1 = `gank_line == Mid` 비교값 (24175) · (qcspec 경고 사유: 이 함수의 다른 배치 범위에 `shl …, 1` 이 있으나 배치 J 의 1 은 열거형 태그값이지 시프트량이 아님) | 4 |  |
| 49 | 0 | 271 | 태그 | LineType 태그 0 = Top — is_adj_line(player.rs:1008): self.line==Mid 일 때 gank_line == Top (24171) | 4 |  |
| 50 | 2 | 271 | 태그 | LineType 태그 2 = Bottom — phi 상수(24175): self.line==Mid && gank_line!=Top 일 때 gank_line == Bottom | 4 |  |
| 51 | 100 | 274 | 계수 | hp_ratio = hp*100/max_hp (백분율) — 24208 · L263 24661 도 동일 | 4 |  |
| 52 | 69 | 296 | 임계 | `icmp ugt hp_ratio, 69` = HP 70% 이상이어야 인접 갱라인 귀환 취소(HideLineToo) — 소스가 `>69` 인지 `>=70` 인지는 표기 불가(외연 동일) (24574) | 4 |  |
| 53 | 70 | 296 | 임계 | 타워 None 경로(@2204)의 같은 임계를 `icmp ult hp_ratio, 70` 로 접은 것 (24582) — 69 와 동일 임계 | 4 |  |
| 54 | 24 | 264 | 임계 | `icmp ugt hp_ratio, 24` = 갱 라인==내 라인일 때 HP 25% 이상이면 귀환 취소, 이하면 Cancel(LowHpSelf) 채팅 (24664) — `>24`/`>=25` 표기 불가 | 4 |  |
| 55 | 5 | 283 | 산출값 | `shl nuw nsw i8 line, 5` = line*32 — AbstractGameWithCache 의 tower/tower2 라인별 stride 0x20 (top 0x180 · mid 0x1a0 · bottom 0x1c0) 로 접힘 (24474·24476) | 4 | 32 |
| 56 | 12 | 299 | 태그 | Chat 메모리태그 12 = HideLineToo (24632) — tcxdict --enum Chat | 3 |  |
| 57 | 17 | 269 | 태그 | Chat 메모리태그 17 = Cancel (24703) | 4 |  |
| 58 | 0 | 269 | 태그 | CancelReason 태그 0 = LowHpSelf (24705 `store i8 0` at +1) · L299 HideLineToo 두 번째 필드 usize 0 (24636) | 4 |  |
| 59 | 2 | 284 | 태그 | EntityType 메모리태그 2 = Tower — closure$2 `if let EntityType::Tower(info) = tower.ty` (24508) | 4 |  |
| 60 | 1 | 287 | 태그 | EntityType 메모리태그 1 = Minion — nearest_enemy 엔티티가 미니언인가 (24548) · (qcspec 경고 사유: 이 함수의 다른 배치 범위에 `shl …, 1` 이 있으나 배치 J 의 1 은 열거형 태그값이지 시프트량이 아님) | 4 |  |
| 61 | 0 | 287 | 태그 | TeamType 메모리태그 0 = Player — `e.team == TeamType::Player(my_team)` derive PartialEq (24556) | 4 |  |

**`knobs` 조정점 23건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 웨이브 피해 귀환 HP 임계 | passive_line.rs:1096 (m04.ll:20513) | 46 | 올리면 프론트 미니언 위치에서 2초 웨이브 피해가 있을 때 더 높은 체력에서도 귀환 | 4 | 기존 |
| 1 | 고체력 면제 임계 | passive_line.rs:1154 (19879) | 70 | hp_ratio 가 이 값 초과면 타워없음·라인페이즈 위험 검사를 건너뜀. 내리면 더 낮은 체력만 검사 | 4 | 기존 |
| 2 | 라인 페이즈 위험 검사 HP 임계 | passive_line.rs:1171 (20012) | 51 | 올리면 라인 페이즈 중 적 수 우세·킬각 검사가 더 자주 발동(귀환 증가) | 4 | 기존 |
| 3 | 라인 페이즈 종료 여유 | player.rs:704 is_line_phase (20008) | 30 | 에픽 첫 스폰 30초 전까지를 라인 페이즈로 봄. 올리면 라인 페이즈가 일찍 끝남 | 4 | 기존 |
| 4 | 위협 거리(프론트 미니언 기준) | passive_line.rs:1174·1180·1202·1125·1146 (22500000001/22500000000) | 150000 | 제곱비교. 올리면 더 먼 적/아군까지 near 집계에 포함 | 4 | 기존 |
| 5 | 쌍둥이 타워 근접 거리 | passive_line.rs:1157·1145 (62500000001) | 250000 | 제곱비교. 라인 타워가 없을 때 프론트 미니언이 쌍둥이 타워 이 거리 안이면 즉시 귀환 | 4 | 기존 |
| 6 | 적 타워 표적중 귀환 HP 임계 | passive_line.rs:1256·1261 (20670·20676) | 31 | 올리면 적 타워 근처에서 더 높은 체력에서 귀환 | 4 | 기존 |
| 7 | 귀환 확정 점수 | passive_line.rs:1447 (21122) | 49 | score > 49 → 귀환. 내리면 귀환이 잦아짐 | 4 | 기존 |
| 8 | 에픽 생존 시 귀환 HP 임계 | passive_line.rs:1439 (21084) | 61 | 에픽이 살아있으면 hp_ratio<61 로 귀환. 내리면 에픽 국면 귀환 감소 | 4 | 기존 |
| 9 | 무조건 귀환 HP 임계 | passive_line.rs:1439 (21086) | 11 | hp_ratio<11 이면 점수 무관 귀환 | 4 | 기존 |
| 10 | 라인 리드 페널티 | passive_line.rs:1422 (21043) | -10 / -5 | 적 리드 −10, 리드 없음 −5, 아군 리드 0. 절댓값 키우면 리드 열세에서 귀환 억제 | 4 | 기존 |
| 11 | 아이템 티어 점수 | passive_line.rs:1396~1400 (21018~21039) | 4→100, 3→70, 2→60, 그외 30, 무아이템&구매가능 5 | 업그레이드 가능 아이템 티어가 높을수록 귀환 점수 증가 | 4 | 기존 |
| 12 | heal_score 표 | passive_line.rs:1293~1392 (20857~20957) | 0/60/70/80 (hp 임계 21/31/41/51/60/70/80) | 힐 보유·정글 여부·수 우세에 따른 귀환 점수. 표 값을 올리면 그 상황에서 귀환 증가 | 4 | 기존 |
| 13 | 갱/다이브 라인 홀드 HP% 임계(gank_hold) | passive_line.rs:237 (m04.ll:21244 `icmp ugt %927, 24`) | 24 | 올리면(예: 40) HP 가 더 높아야 v46 귀환/도주 상태를 접고 갱 라인에 남는다(=L243/248 을 더 자주 실행 → 귀환·도주 판단이 더 자주 개입) · 내리면 저체력에도 갱을 위해 v46 판단을 건너뛴다 | 4 | 기존 |
| 14 | 라인 국면 종료 여유(초) | setting.rs:703~704 인라인 (m04.ll:21344 · 22615 `mul %964, 30`) | 30 | is_line_phase = tick < first_spawn_tick - tps*30 · 올리면 에픽 스폰 더 전에 v46 귀환 커밋/도주 판단이 꺼진다(둘 다 v46_clear/v46_flee_end 로 종료) | 4 | 기존 |
| 15 | 본진 회복 지역 박스(in_heal) | passive_line.rs:497 (m04.ll:21305 891999 · 21328 896000 · heal_area game.rs:319 64000/960000) | x∈[rx-68000,rx] · y∈[ry-64000,ry] | 박스를 키우면 본진 근처에서 귀환 커밋 판단이 더 넓게 꺼진다(v46_clear) · 값은 게임 상수 접힘이라 소스 수준에서는 heal_area+마진 | 4 | 기존 |
| 16 | 근접 반경(near_enemies 후보 · 도주 위협 유지 still) | passive_line.rs:523·524·386 (m01.ll:23923·23953 · m04.ll:23779 `< 62500000001`) | 62500000001 | = 250000² + 1 · 올리면 더 먼 적도 near_enemies 에 들어가 stage1 위협 후보가 늘고, 도주 중 위협이 더 멀리서도 '남아있음'으로 잡혀 도주가 길어진다 | 4 | 기존 |
| 17 | 회복 이득 환산 계수 | passive_line.rs:624 (m04.ll:22009 `mul %1215, 60`) | 60 | heal_die_gain = (full_hp-my_hp)*60/best_dps · 올리면 같은 HP 결손에서 heal_die_gain 이 커져 `< best_diff_bound` 거부(veto_heal)가 줄고 귀환 커밋이 늘어난다 | 4 | 기존 |
| 18 | 회복 이득 임계 바닥/기울기(best_diff_bound) | passive_line.rs:628 (m04.ll:22033 `*80` · 22035 `+80`) | 80 | best_diff_bound = 80 + 80*(1000-aggr)/1000 (80..160) · 바닥 80 을 올리면 모든 선수가 귀환 커밋에 더 큰 회복 이득을 요구(veto_heal 증가) · 기울기 80 을 올리면 소극적 선수일수록 더 까다로워진다 | 4 | 기존 |
| 19 | 도주 acute 반응 여유 틱 | passive_line.rs:399 (m04.ll:23805 `add %1910, 6`) | 6 | reaction = attack_duration + 6 · 올리면 위협의 `speed*reaction` 도달 여유가 커져 acute(급박) 판정이 더 자주 참 → approached 표시·flee_hold_acute_ticks 증가(행동 효과는 v46_flee_acute 소비처 = 다른 배치/함수) | 4 | 기존 |
| 20 | v46 귀환 커밋 트리거 조건(순서) | passive_line.rs:548~630 | front Some && near_enemies≠∅ → stage1≠∅ → !stage2 → !(tower_protects\|\|mp<0 && !base_heal) → !(mp>0 && wave_crashing) → heal_flips && heal_gain>=bound | 각 veto 카운터(0x98/0xa0/0xa8/0xb0)가 어느 게이트에서 거부됐는지 계측 · 게이트 하나를 빼면 그만큼 in_recall 승격(L245)이 늘어난다 | 4 | 기존 |
| 21 | 갱/다이브 라인==내 라인일 때 귀환 취소 HP 임계 | passive_line.rs:264 (m04.ll:24664 `icmp ugt %hp_ratio, 24`) | 24 | 올리면(예: 40) HP 가 더 높아야 귀환을 접고 갱 라인에 남는다(=더 자주 Cancel(LowHpSelf) 채팅으로 갱 거부) · 내리면 저체력에도 귀환을 취소하고 갱에 합류한다 | 4 | 기존 |
| 22 | 인접 갱라인 귀환 취소(HideLineToo) HP 임계 | passive_line.rs:296 (m04.ll:24574 `ugt 69` · 24582 `ult 70`) | 69 | 올리면 인접 라인 갱 때 귀환을 유지하는 경우가 늘고(HideLineToo 감소) · 내리면 낮은 HP 로도 귀환을 접고 라인에 숨는다 — 두 사이트(69/70) 를 같이 바꿔야 타워 유무 경로가 일치한다 | 4 | 기존 |

<details><summary>`callees` 피호출자 84건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_log | game_core::DebugFrameData::add_log | pub | fn(&mut game_core::DebugFrameData, &game_core::OperationData, &game_core::PlayerState, std::string::String) | game-core\src\simulation\game\frame.rs:54 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | aggressive_ratio | game_core::AthleteParameter::aggressive_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:432 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | attack_duration | game_core::Entity::attack_duration | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1770 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | buy_item | game_ai::buy_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> | game-ai\src\lib.rs:1477 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | check_recall | game_ai::plan_legacy::old::SinglePlanLine::check_recall | in:game_ai::plan_legacy::old::single_line | fn(&game_ai::plan_legacy::old::SinglePlanLine, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\single_line.rs:281 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | check_recall | game_ai::plan_legacy::old::PassiveLinePlan::check_recall | in:game_ai::plan_legacy::old::passive_line | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_line.rs:1068 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | check_recall | game_ai::plan_legacy::old::PassiveJunglePlan::check_recall | in:game_ai::plan_legacy::old::passive_jungle | fn(&game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:142 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | clear | game_core::DataTable::<T>::clear | pub | fn(&mut game_core::DataTable<T/#0>) | game-core\src\data.rs:2259 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | effect_buff_target | game_ai::effect_buff_target | pub | fn(usize, &game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-ai\src\fight_check.rs:390 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | enemy_minion_line_action_danger_damage_at | game_ai::enemy_minion_line_action_danger_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize, bool, bool) -> usize | game-ai\src\minion_wave_risk.rs:233 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | expected_heal_target | game_core::Effect::expected_heal_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 22 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 23 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 24 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 25 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 26 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 27 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | has_line_lead | game_ai::plan_legacy::sub_plan::has_line_lead | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, usize, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\sub_plan\line_defense.rs:1399 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | heal_area | game_core::Game::heal_area | pub | fn(usize) -> (u64, u64, u64, u64) | game-core\src\simulation\game.rs:318 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 30 | in_big_line | game_core::Blackboard::in_big_line | pub | fn(&game_core::Blackboard, usize, game_core::LineType) -> bool | game-core\src\simulation\game\blackboard.rs:137 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | in_recall | game_core::Blackboard::in_recall | pub | fn(&game_core::Blackboard, usize) -> bool | game-core\src\simulation\game\blackboard.rs:175 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | is_adj_line | game_core::LineType::is_adj_line | pub | fn(&game_core::LineType, game_core::LineType) -> bool | game-core\src\simulation\state\player.rs:1005 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 33 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 34 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 35 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 36 | is_in_return | game_core::Entity::is_in_return | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1647 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 37 | is_jungle | game_core::EntityType::is_jungle | pub | fn(&game_core::EntityType, usize) -> bool | game-core\src\simulation\entity.rs:1377 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 38 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 39 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 40 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 42 | item_score | game_core::ItemNetwork::item_score | in:game_core::simulation::item_network | fn(&game_core::ItemNetwork, &std::vec::Vec<std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>, std::alloc::Global>, std::vec::Vec<(game_core::Position, usize), std::alloc::Global>, std::vec::Vec<(game_core::Position, usize), std::alloc::Global>, game_core::Position, usize, bool) -> i64 | game-core\src\simulation\item_network.rs:55 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 43 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 44 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 45 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 46 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 47 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 48 | line | game_ai::plan_legacy::types::BigPlan::debug_label::line | in:game_ai::plan_legacy::types | fn(game_core::LineType) -> &str | game-ai\src\plan_legacy\types.rs:83 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 49 | line | game_core::TowerType::line | pub | fn(&game_core::TowerType) -> std::option::Option<game_core::LineType> | game-core\src\simulation\entity\tower.rs:90 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 50 | max_hp | game_core::PlayerAiContext::<'a, 'b, 'r>::max_hp | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> std::option::Option<usize> | game-core\src\mod_ai.rs:608 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 51 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 52 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 53 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 54 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 55 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 56 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 57 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 58 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 59 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 60 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 61 | return_time | game_core::Entity::return_time | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1654 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 62 | skill2 | game_ai::skill2 | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:239 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 63 | skill2 | game_core::Entity::skill2 | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1668 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 64 | skill2 | game_core::ChampionInfo::skill2 | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1086 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 65 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 66 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 67 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 68 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 69 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 70 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 71 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 72 | tier | game_core::ItemInfo::tier | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:437 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 73 | tier | <game_core::ModItemEntry as game_core::ItemInfo>::tier | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:499 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 74 | tier | <game_core::BaseItemInfo as game_core::ItemInfo>::tier | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:617 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 75 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 76 | update_v46_flee | game_ai::plan_legacy::old::PassiveLinePlan::update_v46_flee | in:game_ai::plan_legacy::old::passive_line | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\passive_line.rs:343 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 77 | update_v46_lane_recall | game_ai::plan_legacy::old::PassiveLinePlan::update_v46_lane_recall | in:game_ai::plan_legacy::old::passive_line | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\passive_line.rs:488 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 78 | upgrade_item | game_ai::upgrade_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1603 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 79 | v46_clear | game_ai::plan_legacy::old::PassiveLinePlan::v46_clear | in:game_ai::plan_legacy::old::passive_line | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, bool) | game-ai\src\plan_legacy\old\passive_line.rs:476 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 80 | v46_flee_end | game_ai::plan_legacy::old::PassiveLinePlan::v46_flee_end | in:game_ai::plan_legacy::old::passive_line | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan) | game-ai\src\plan_legacy\old\passive_line.rs:322 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 81 | v46_flee_gate_check | game_ai::plan_legacy::old::passive_line::v46_flee_gate_check | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> (u8, bumpalo::collections::vec::Vec< usize>) | game-ai\src\plan_legacy\old\passive_line.rs:38 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 82 | v46_stage1 | game_ai::plan_legacy::old::PassiveLinePlan::v46_stage1 | in:game_ai::plan_legacy::old::passive_line | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::Entity, usize, bool, std::option::Option<&[usize]>, &bumpalo::collections::vec::Vec< &game_core::Entity>) -> bumpalo::collections::vec::Vec< (usize, usize, usize)> | game-ai\src\plan_legacy\old\passive_line.rs:663 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 83 | v46_stage2 | game_ai::plan_legacy::old::PassiveLinePlan::v46_stage2 | in:game_ai::plan_legacy::old::passive_line | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &bumpalo::collections::vec::Vec< (usize, usize, usize)>) -> bool | game-ai\src\plan_legacy\old\passive_line.rs:783 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 48개**: `bounds`, `combo_dmg`, `commit_clears`, `danger_ticks`, `dist_sq`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `else`, `enumerate`, `first_spawn_tick`, `flee_hold_acute_ticks`, `flee_hold_cover_ticks`, `flee_hold_refuge_ticks`, `flee_hold_ticks`, `flee_nohit_episodes`, `flee_triggers`, `format_inner`, `from_mid`, `grow_one`, `has_ally_heal`, `heal_score`, `hp_ratio`, `is_more_enemy`, `is_none_or`, `llvm.assume`, `llvm.usub.sat.i64`, `map_or`, `minion_power`, `move_speed`, `my_pos_index`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `parameter`, `reserve_internal_or_panic`, `sat_sub`, `stage2_saves`, `take`, `trigger_wave_enemy_half`, `trigger_wave_my_half`, `try_fold`, `v46_commit`, `v46_flee`, `v46_flee_acute`, `v46_flee_approached`, `v46_flee_cover`, `v46_flee_hp_entry`, `v46_flee_hp_min`, `veto_crash`, `veto_heal`, `veto_wave`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:6931) · **형제 18개** (PassiveLinePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::PassiveLinePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\passive_line.rs:176 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan) -> game_ai::plan_legacy::old::PassiveLinePlan |
| 1 | <game_ai::plan_legacy::old::PassiveLinePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\passive_line.rs:176 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::PassiveLinePlan::new | pub | game-ai\src\plan_legacy\old\passive_line.rs:197 | True | fn(game_core::LineType) -> game_ai::plan_legacy::old::PassiveLinePlan |
| 3 | game_ai::plan_legacy::old::PassiveLinePlan::v46_fleeing | pub | game-ai\src\plan_legacy\old\passive_line.rs:207 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan) -> bool |
| 4 | game_ai::plan_legacy::old::PassiveLinePlan::goal | pub | game-ai\src\plan_legacy\old\passive_line.rs:211 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan) -> game_core::BigGoal |
| 5 | game_ai::plan_legacy::old::PassiveLinePlan::update | pub | game-ai\src\plan_legacy\old\passive_line.rs:219 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 6 | game_ai::plan_legacy::old::PassiveLinePlan::v46_carry_over | pub | game-ai\src\plan_legacy\old\passive_line.rs:307 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, &mut game_ai::plan_legacy::old::PassiveLinePlan) |
| 7 | game_ai::plan_legacy::old::PassiveLinePlan::v46_flee_end | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:322 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan) |
| 8 | game_ai::plan_legacy::old::PassiveLinePlan::update_v46_flee | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:343 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 9 | game_ai::plan_legacy::old::PassiveLinePlan::v46_clear | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:476 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, bool) |
| 10 | game_ai::plan_legacy::old::PassiveLinePlan::update_v46_lane_recall | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:488 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 11 | game_ai::plan_legacy::old::PassiveLinePlan::v46_stage1 | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:663 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::Entity, usize, bool, std::option::Option<&[usize]>, &bumpalo::collections::vec::Vec< &game_core::Entity>) -> bumpalo::collections::vec::Vec< (usize, usize, usize)> |
| 12 | game_ai::plan_legacy::old::PassiveLinePlan::v46_stage2 | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:783 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &bumpalo::collections::vec::Vec< (usize, usize, usize)>) -> bool |
| 13 | game_ai::plan_legacy::old::PassiveLinePlan::sub_plan | pub | game-ai\src\plan_legacy\old\passive_line.rs:848 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |
| 14 | game_ai::plan_legacy::old::PassiveLinePlan::check_bot_lane_2v1 | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:1018 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::sub_plan::SubPlan> |
| 15 | game_ai::plan_legacy::old::PassiveLinePlan::has_lead | pub | game-ai\src\plan_legacy\old\passive_line.rs:1048 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 16 | game_ai::plan_legacy::old::PassiveLinePlan::check_recall | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:1068 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 17 | game_ai::plan_legacy::old::PassiveLinePlan::next_plan | pub | game-ai\src\plan_legacy\old\passive_line.rs:1451 | True | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |

**`open` 23건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | (배치 H) L1392~1405 item_score 의 소스 분기 순서(can_buy.is_none 을 먼저 보는지 upgrade 를 먼저 보는지): IR 은 %775(L1392)→%127(L1392/1394) 순이나 column 없음 → 표기 불가. 결과 표는 4조합 전부 확정 | 4 |  |
| 1 | 미탐색 | (배치 H) buy_item 반환 {i64,i64} 의 .1(페이로드) 의미 — H 범위에서 .0==1 (Some) 만 씀. upgrade_item sret 24B 의 +8 바이트 의미 — 미사용(+0 태그·+0x10 idx 만) | 4 |  |
| 2 | 미탐색 | (배치 H) expected_heal_target/expected_damage_target 의 4번째·6번째 인자 @anon.6(정적 상수: drop_glue<Entity> fn-ptr + 헤더 = 빈 bumpalo Vec<&Entity> 로 추정) — 시그니처 의미는 game_core 경계라 미명세 | 5 |  |
| 3 | 미탐색 | (배치 H) get_game_mode()(vtable+0x40) 반환 {i64,ptr} 의 .0 의미: as_moba(L231) 가 `.0 != 0 \|\| .1 == null` 이면 None 으로 봄 — 열거형 이름은 미탐색(game_core) | 4 |  |
| 4 | 표기 불가 | (배치 H) L1140 `!(ty==Champion && action_state==Return)` 이 소스에서 return_time().is_none() 인지 직접 매치인지 — 외연 동일, 표기 불가 | 4 |  |
| 5 | 미탐색 | (배치 H) NA(version<2 전용) 표기: %112=true(분수 안·만피·version≤1) 경로와 %130 블록(L1087 웨이브 검사 생략) — reach.py version=2 접힘으로 사장. 명세는 version≥2 기준 | 4 |  |
| 6 | 미탐색 | (배치 H) 블록 %904(21197~) 는 루트 L222 3줄 + L232 5줄 혼합 — in_recall store 까지만 H, 나머지는 배치 I 가 잇는다. 블록 %2050(24143~) 의 루트 0 2줄도 배치 J 소속 | 4 |  |
| 7 | 미탐색 | (배치 I) committers 원소 (usize,usize,usize) 의 .1/.2 정확한 의미 — 본 범위에서는 .0 = 엔티티 id(get_entity_by_id·player_by_champion_id 인자로 확정) · .1 = min_by_key 키(로그 라벨 my_die → 사망예측값 추정) · .2 = best_dps(로컬 DI 이름) 까지만 확정. 정의는 v46_stage1(배치 T) | 5 |  |
| 8 | 표기 불가 | (배치 I) heal_area(team)(game.rs:318~319) 의 원 표현 — IR 엔 team==0 select 64000/960000 만 남아 있고 in_heal 의 마진 68000(x)/64000(y) 이 saturating 접힘으로 비대칭 상수(891999·896000)로만 관측된다. 소스가 `abs_diff <= 마진` 인지 `rx-마진..=rx` 인지는 표기 불가(동작은 확정 · logic 에 접힌 식 기재) | 4 |  |
| 9 | 미탐색 | (배치 I) iter_towers_without_nexus 의 6칸 배열이 정확히 어떤 타워 슬롯 6개인지 — 반환 타입(IntoIter<Option<&Entity>,6> + Copied<Iter<&Entity>>)으로 top/top2/mid/mid2/bottom/bottom2 + twin_towers 로 추정 · _gcbc 본문 미독해(계약만 · 경로 계층 아님이나 r18 범위 밖이라 열지 않음) | 5 |  |
| 10 | 표기 불가 | (배치 I) L237 `>24` vs `>=25` · :594 `<0` vs `<=-1` 등 외연 동일 비교의 소스 표기 — 표기 불가(동작 확정) | 4 |  |
| 11 | 미탐색 | (배치 I) constants 의 상수 60(:624) 이 tps 하드코딩인지 다른 단위 환산인지 — setting.tick_per_second(0x12f8) 로드가 이 식에 없으므로 리터럴 60 확정, 의미는 추정 | 5 |  |
| 12 | 표기 불가 | (배치 I) v46_flee_end 의 line 인자 — L241 호출은 gank_line(%913 · TeamPlan+0x420)을 넘기지만 L236 에서 self.line 과 같음이 확정된 값이라 컴파일러가 통합했다(22540 store %913). 소스가 `self.v46_flee_end(gank_line)` 인지 `(self.line)` 인지는 표기 불가 | 4 |  |
| 13 | 미탐색 | (배치 I) has_lead / v46_carry_over / has_line_lead · v46_stage1 내부 · SubPlan/next_plan 생성 — 전부 배치 I 범위 밖(H/J/T) | 4 |  |
| 14 | 표기 불가 | (배치 J) v46_clear(passive_line.rs:476~481) 의 478~479 줄 내용 — completed=true 상수로 인라인돼 IR 에 흔적이 없다(관측: 477 주소계산·480 store v46_commit=0·481 Vec::clear 만). `!completed` 게이트 아래 계측 증가(v46_pending.commit_clears 0xb8 등)일 가능성은 추정 — IR 로는 표기 불가, 미탐색 = 다른 호출처(completed=false)에서 v46_clear 인라인을 읽는 것 | 4 |  |
| 15 | 표기 불가 | (배치 J) L264 `>24` vs `>=25`, L296 `>69` vs `>=70` — 외연 동일(표기 불가 · 동작은 확정) | 4 |  |
| 16 | 표기 불가 | (배치 J) L271 `is_adj_line(..) && self.in_recall` 의 소스 순서(A && B 의 A/B) — column 0 이라 표기 불가 · IR 은 인접판정 뒤 in_recall(select 접힘) · 둘 다 순수 load 라 동작 차이 없음. 수신자 순서(gank_line.is_adj_line(self.line) 추정 — DI __self_discr=gank_line 근거)도 대칭 관계라 동작 무관 | 4 |  |
| 17 | 표기 불가 | (배치 J) L296 `pushed_to_tower \|\| !my_line_enemy_visible` 의 소스 순서 — 둘 다 이미 계산된 bool 이라 표기 불가·동작 무관 | 4 |  |
| 18 | 미탐색 | (배치 J) data.blackboard[1 - team](적 팀 인덱스)를 is_recent_visible 의 &self 로 쓰는 것이 소스 의도인지(콜리 본문은 self.last_visible[target.position] 을 읽으므로 '그 팀 챔피언의 최근 가시 틱' 테이블로 보임) — 계약만 기록, 의미 해석은 추정 | 5 |  |
| 19 | 미탐색 | (배치 J) is_near_line 본문(_gcbc g09.ll:157890~158197, 경로/맵 계층) — 시그니처·반환 의미만(r18 범위) · 내부 상수 미등록 | 4 |  |
| 20 | 미탐색 | (배치 J) is_recent_visible 의 120 틱 상수는 콜리 소유라 constants 미등록(callee_contracts 에만) | 4 |  |
| 21 | 미탐색 | (배치 J) get_entity_by_id 는 vtable 간접 호출(`call ptr %2184`)이라 qcspec C2 대상이 아니어서 calls 에 넣지 않음 — callee_contracts 에 등재 | 4 |  |
| 22 | 미탐색 | (배치 J) 지도 `per` 는 루트 277=82 · 283=45 (합 235)인데 주석본 슬라이스는 277=72 · 283=44 (합 224) — 주석본에서 `;L` 사슬이 비어(빈 chain) 루트를 못 단 줄 11개(dbg_value 류로 추정) · 본문 명령엔 영향 없음(원문 24143~24726 을 직접 대조함) | 4 |  |

<details><summary>`closed` 3건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | (배치 H) line_strategy(L1276) 의 열거형 이름·값: DI 가 i8 poison 으로만 남기고 비교 %702(position==Jungle) 만 생존 — 값 표기는 재료 부재(IR 상수접힘). 동작은 확정(정글이면 한 갈래, 아니면 다른 갈래) | 본문에 해소 표기가 있다 |
| 1 | (배치 I) v46_flee_gate_check 의 verdict(u8) 값 표 — 본 범위에서는 0=통과(도주 진입)만 확정(23371 `icmp eq 0`). 비0 값들의 의미(docs 의 '도주가능 시점 게이트 차단사유')는 m04.ll:55362 본문 미독해(미탐색 · 이 배치 범위 밖) | 본문에 해소 표기가 있다 |
| 2 | (배치 I) blackboard[1 - player.team] 인덱스의 의미(적 팀 인덱스인데 '내가 본 최근 시야'로 쓰임) — is_recent_visible/in_big_line 본문(_gcbc g07.ll:157005·156563) 미독해. IR 인덱스 산술은 확정(m01.ll:23845~23852) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

