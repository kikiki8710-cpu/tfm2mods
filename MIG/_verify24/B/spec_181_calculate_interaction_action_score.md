---

### `181` calculate_interaction_action_score — 액션 후보 1개(effect, 대상 t)의 상호작용 점수 i64 — 적/아군/자기 대상별 피해·힐·버프·CC 후속·타워·이동 항을 합산

| 항목 | 값 |
|---|---|
| id | `action_score__calculate_interaction_action_score` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai12action_score34calculate_interaction_action_score` |
| 소스 | `game-ai\src\action_score.rs:980` |
| IR | `m05.ll` 40106~44246행 |
| 경로·가시성 | `game_ai::action_score::calculate_interaction_action_score` · **in:game_ai** |
| 계층 | 점수화·술어 |
| exe | `d5bbf0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r15` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, &game_core::Entity, &mut game_core::DebugFrameData) -> i64
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize (i64 %0) | AI 버전. 본문 분기 3곳: L986/L987 `version>1`(넥서스 최종방어·기지방어 판정 활성), L1000 `version>1`(v3 턴백/도주 차단 블록 활성). 그 외는 콜리에 그대로 전달(check_kill_die_tick·minion_wave_risk·position_score_at_position·effective_action_cc_time·v55_*·effect_buff_target·defensive_crisis·area_buff_multi_scale·v54_aoe_ally_heal_value). alloca %75 에 저장 후 재로드. | 4 |
| 1 | 2 | _rnd | &mut StdRng (320B) %1 | 이 함수는 직접 읽지도 쓰지도 않는다. 전달만: check_kill_die_tick(콜리 속성 readnone — 실사용 없음) · defensive_crisis(L1352·L1474·L1528, 콜리 속성 RW — 롤 소비 가능). | 4 |
| 2 | 3 | player | &PlayerState (2528B) %2 | info.team(+0x930)·info.position 태그(+0x9c0) 읽음. 콜리 다수에 전달. | 4 |
| 3 | 4 | data | &OperationData (24B) %3 | cache(+0x0 &AbstractGameWithCache 8840B) · context(+0x8 &GameContext 64B) · blackboard(+0x10 &[Blackboard;2]) | 4 |
| 4 | 5 | parameter | &ScoreParameter (5384B) %4 (readonly) | DI 이름 `parameter`. player(+0x918 ChampionScoreParameter)·positioning_score(+0x9f0)·near_allies(+0x14b8/len +0x14d0)·near_enemies(+0x14d8/len +0x14f0)·v3_turnback_hold(+0x1500) 읽음. | 4 |
| 5 | 6 | action | &Box<dyn Action> (16B 팻포인터를 가리키는 포인터) %5 — data_ptr +0x0 / vtable_ptr +0x8 | DI 이름 `action`. vtable 슬롯: +0x78 action_name(sret String 24B, 디버그 로그 전용) · +0x80 duration() -> i64 · +0xb8 can_use_with_move() -> bool (divtable Action 일치율 58%). | 3 |
| 6 | 7 | effect | &Effect (56B) %6 (readonly) | DI 이름 `effect`. ty(+0x0 Arc<dyn EffectType> · ArcInner 데이터 = ptr+((vtable.align-1)&~15)+16) · range(+0x10) · growth_range(+0x18). EffectType vtable 슬롯: +0x60 expected_rush_effect · +0x68 expected_move_on_hit · +0x90 etc_buff · +0x98 expected_ally_aura_heal_at · +0xa8 expected_mark(sret 48B Option) · +0xb0 expected_on_attack_damage (g02.ll:1311 CombineEffect 정적 vtable 슬롯표 · 런타임 구현체는 Arc 라 미상). | 4 |
| 7 | 8 | t | &Entity (1728B) %7 (readonly) | DI 이름 `t` = 대상 엔티티. 적 챔피언/아군 챔피언/자기 자신/그 밖(타워·미니언 등) 으로 분기. | 4 |
| 8 | 9 | _debug | &mut DebugFrameData (224B) %8 | 직접 쓰기 0. 전달: DebugFrameData::add_log ×6 (L1014·L1387·L1415·L1503·L1544·L1559 — logs Vec push = 힙 성장) · v55_mark_value(L1298, RW) · defensive_crisis(L1352·L1474·L1528, RW) · check_kill_die_tick(readnone). | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// 표기: L = action_score.rs 루트 줄(inlinedAt 루트). 오프셋은 reads 참조. 판정 순서 = IR 블록 순서.
// ---- §A 도입·v3 도주 차단 (L980~1034) ----
team = player.info.team; team>=2 → panic_bounds_check. my_position = player.info.position 태그
champ = data.cache.player_champion[team][my_position].expect(..)  (None → unwrap_failed, 메시지 anon.159 · L1013 인라인 위치)
L985 no_self_risk = if champ.stat_buff_cached.undying { true }
L986   else if version>1 && nexus_final_stand(player,data) { true }
L987   else if version>1 && champ.hp*100 > max(champ.stat_cached.hp,1)*35 { L988 base_defense_focus(player,data) }
       else { false }                                  // version<=1 이면 undying 아니면 false
L991 if let Some(v) = v57_summon_command_score(data,player,champ,action,t) { return v }   // Option<i64> {tag,v}
L1000 if version>1 && t.ty==Champion(13) && t.team != champ.team   // TeamType::eq 인라인(태그 → Player 페이로드)
L1001    && game.get_game_mode() 태그 != DeathMatch(2) {          // reach --gamemode 0: 항상 통과
L1013   if ctx.debug && parameter.v3_turnback_hold {
L1014     _debug.add_log(format!("TBHOLD-CAST T{team} {pos:?} tgt={t.id} d={champ.distance(t)} reach={line_effect_range_with_radii(effect,champ,t)} held={d>reach}")) }   // anon.152
L1019   if champ.distance(t) > line_effect_range_with_radii(effect,champ,t) {          // 사거리 밖
L1020     if parameter.v3_turnback_hold { return -9999999 }
L1023     if !no_self_risk {
L1024       if let Some((_,_,walk_tick)) = line_action_stance(data,champ,t,effect) {   // Option<(u64,u64,usize)>
L1025         if walk_tick != 0 {
L1026~1030      die_enemies: bump Vec<&Entity> = cache.player_champion[1-team].iter_champions()
                    .filter(|e| e.distance_sq(champ) < 22500000001 && data.blackboard[1-player.info.team].is_recent_visible(game,player,e)).collect_in(ctx.pool)   // closure$0 (aux)
L1031           if !die_enemies.is_empty() {
L1032             my_die = check_kill_die_tick(version,_rnd,data,player,champ,&die_enemies,&Vec::new_in(pool),_debug)
L1034             if my_die <= action.duration() + walk_tick { return -9999999 }   // IR: my_die > dur+walk 이면 계속
                } } } } } }
// ---- §B 기본 위험 · 적 타워 집중 (L1050~1082) ----
L1050 applyed_damage = parameter.player.applyed_damage ; L1051 base_damage = parameter.player.risk_damage
L1052 possible_damage = parameter.player.possible_risk(data, action.duration()+30)
L1053 is_visible = game.is_visible(1-team, champ.id)          // 적이 나를 보는가
L1054 if !is_visible { possible_damage /= 3 }   (sdiv)
L1060~1061 near_enemy_tower = cache.iter_towers_without_nexus(1-team).filter(|tw| tw.distance_sq(champ) < 22500000000).min_by_key(|tw| tw.distance_sq(champ))
L1064 can_focused_tower = false
      if let Some(tower) = near_enemy_tower {
L1065   if tower.ty == Tower(2) && L1066 tower.ty@Tower.info.nearest_enemy.is_none() {
L1067     range = effect.range + (champ.level-1)*effect.growth_range + champ.stat_buff_cached.range + effect.range_adjust(champ,tower) + champ.radius() + t.radius()
            // radius() 인라인: if stat_buff_cached.radius_mult==0 {radius} else {radius*(mult+100)/100 (udiv)}
L1068     d = t.distance(champ) ; L1069 d = d.saturating_sub(range)
L1071     tower_range = tower.attack_effect.unwrap().range + (tower.level-1)*growth_range + tower.stat_buff_cached.range   // None → unwrap_failed anon.160
L1072     dist = utils::distance(champ.x,champ.y,tower.x,tower.y)
L1074     can_focused_tower = dist <= d + 15000 + tower_range + tower.radius() + champ.radius()   (icmp ule)
        } }
L1081~1082 if can_focused_tower || t.is_champion() { possible_damage += parameter.player.risk_possible_tower }
// ---- §C 미니언 웨이브 투사 피해 (L1086~1094) ----
L1086 tick = game.tick(); spawn_epic = ctx.tutorial ∈ {None,MidBottom,Line,Total} (인라인 switch);
      line_phase_ok = !(tick < setting.epic_jungle.first_spawn_tick.saturating_sub(tps*30))   // 인라인 scope 이름 is_line_phase — IR 극성: tick >= first_spawn-30초 일 때 진행
L1087 projected_minion_damage = 0; if spawn_epic && line_phase_ok && t.team != champ.team && t.is_champion() {
L1088   if let Some((sx,sy,walk_tick)) = line_action_stance(data,champ,t,effect) {
L1089     a = action.duration() + walk_tick
L1090     window = max(tps/2, a) ; L1091 window = min(tps*2, window)      // lshr/shl 접힘
L1092     current_damage = enemy_minion_wave_risk_damage_at(version,data,champ,champ.x,champ.y,window)
L1093     stance_damage  = enemy_minion_wave_danger_damage_at(version,data,champ,sx,sy,window)
L1094     projected_minion_damage = stance_damage.saturating_sub(current_damage/2) } }
// ---- §D 기본 점수 (L1102~1108) ----
L1102 hp_value = champion_hp_value(data, parameter, &parameter.player)
L1105 base_score = if no_self_risk { -1 }
L1108              else { !( (possible_damage + base_damage + projected_minion_damage) * hp_value / champ.hp ) }   // xor -1 = 비트 NOT = -x-1 · sdiv · champ.hp==0 → div_by_zero panic · 표기(`!x` vs `-1-x`)는 외연 동일
// ---- §E 아군 타워 · 근접 적 · 이동 점수 (L1111~1149) ----
L1111~1112 near_ally_tower = cache.iter_towers_without_nexus(team).filter(|tw| tw.distance_sq(champ) < 22500000000).min_by_key(distance_sq)
L1115 has_near_enemy_champion = cache.player_champion[1-team].iter().flatten().any(|c| c.distance_sq(champ) < 15000000000 && data.blackboard[1-team].is_recent_visible(game,player,c))   // 5슬롯 언롤
L1119 tower_score = 0
      if let Some(tower) = near_ally_tower {
L1121   tower_atk = tower.attack_effect.as_ref().unwrap()   // None → unwrap_failed anon.162
        if has_near_enemy_champion && tower_atk.is_in_range(tower, t)
L1123      && tick < setting.tower_attack_disable_tick {
L1127     tower_score = min(tower_atk.expected_damage_target(ctx, tower as &dyn, t) * 100 / t.hp, 100)   (udiv · t.hp==0 → panic anon.163)
        } }
L1131 move_score = 0
      if effect.ty.expected_move_on_hit() || effect.ty.expected_rush_effect() {        // vtable +0x68 || +0x60 (이 순서)
L1134   if champ.distance_sq(t) < 1225000001 { move_score = 0 }   // ≤35000
L1137   else { ps = position_score_at_position(version, player, data, &parameter.positioning_score, t.x, t.y, PositionEvalPurpose::AttackStance)
L1140          move_score = -(hp_value * ps.risk) / 100 }   (sdiv)
      }
L1149 in_lapse = player_awareness_lapse(player, data)
      lapse_proximity_score = |e| (150000 - min(e.distance(champ),150000)) / 1500     // closure$6 (smin·sdiv)
// ---- §F 이동 중 사용 가능 · 사거리 내 (L1155~1172) ----
L1155 if action.can_use_with_move() && effect.is_in_range(champ, t) {                 // vtable +0xb8, 이 순서
L1156   if parameter.near_enemies.iter().any(|p| p.id == t.id) && in_lapse {
L1157     return 100 + lapse_proximity_score(t) }
L1159   if let Some(tp) = parameter.near_enemies.iter().find(|p| p.id == t.id) {
L1160     base_damage = tp.risk_damage
L1161     expected_damage = effect.expected_damage_target(ctx, champ as &dyn, t)
L1162     possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1163     hp_value = champion_hp_value(data, parameter, tp)
L1166     total_damage = expected_damage + base_damage/2 + possible_damage/4   (sdiv)
L1167     if t.stat_buff_cached.undying { L1168 total_damage = min(max(t.hp-1,0), total_damage) }
L1172     return total_damage * hp_value / t.hp + 100   (sdiv · t.hp==0 → panic anon.164)
        } }   // near_enemies 에 없으면 아래로 낙하
// ---- §G 적 챔피언 대상 (L1177~1300) ----
sum3 = tower_score + base_score + move_score
L1177 if in_lapse && near_enemies.any(|p| p.id==t.id) { total = lapse_proximity_score(t) (L1178) ; goto RET }
L1179 if let Some(tp) = near_enemies.find(|p| p.id==t.id) {
L1180   base_damage = tp.risk_damage ; L1181 expected_damage = effect.expected_damage_target(ctx,champ,t)
L1182   possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1183   hp_value = champion_hp_value(data,parameter,tp)
L1186   total_damage = expected_damage + base_damage/2 + possible_damage/4
L1187   if t.undying { L1188 total_damage = min(max(t.hp-1,0), total_damage) }
L1192   target_hp = t.hp ; score = total_damage * hp_value / target_hp   (sdiv · 0 → panic anon.165)
L1195   if t.is_champion() {
L1198     if expected_damage >= target_hp               { L1200 score += min(hp_value,100) }           // icmp slt 반전
L1201     else if expected_damage + base_damage/2 >= target_hp { L1203 score += min(hp_value,100)*3/4 }
L1206     else { auto_damage = champ.attack_effect.as_ref().map(|a| a.expected_damage_target(ctx,champ,t)).unwrap_or(0)   (L1206~1207)
L1210            if auto_damage + expected_damage >= target_hp { L1211 score += min(hp_value,100)/2 } } }
L1218   if let Some(cc_duration) = effective_action_cc_time(version, action, effect) {      // {tag,val}
L1219     if let Some(target_player) = cache.player_by_champion_id(t.id) {                   // L1220 null 검사
L1222       target_pos = target_player.info.position 태그 ; L1223 tps = max(setting.tick_per_second,1)
L1226       my_attack_dps = cache.player_champion_cache[team][my_position].attack_per_sec[target_pos]
L1231       ally_dps = 0; for ap in 0..5 { L1232 if ap == my_position {continue}
L1233         if let Some(ally) = cache.player_champion[team][ap] {
L1234           if ally.distance_sq(champ) < 22500000001 { c = cache.player_champion_cache[team][ap];
L1235~1241        ally_dps += c.attack_per_sec[target_pos] + c.skill_per_sec[target_pos] + c.skill2_per_sec[target_pos] + c.ult_per_sec[target_pos] } } }
L1248       follow_up_damage = (ally_dps + my_attack_dps) * cc_duration / tps   (udiv)
L1249       cc_score = follow_up_damage * hp_value / target_hp   (sdiv · overflow panic anon.166)
L1251       score += min(cc_score, 80)
L1258       nuke_during_cc = 0
            if champ.can_attack() || champ.attack_cooldown() <= cc_duration {      // attack_cooldown 인라인: ty 태그 switch → info.attack_cooldown
L1259~1260    if let Some(atk) = champ.attack_effect.as_ref() { nuke_during_cc = atk.expected_damage_target(ctx,champ,t) } }
L1264       if champ.can_skill() || champ.skill_cooldown() <= cc_duration {         // Champion 이 아니면 쿨다운 0 취급(IR: 태그≠13 → 통과)
L1265~1267    if let Some(s) = champ.skill_effect.as_ref() { if s.target.check(champ,t) { nuke_during_cc += s.expected_damage_target(ctx,champ,t) } } }
L1271       if champ.can_skill2() || champ.skill2_cooldown() <= cc_duration {
L1272~1274    s = if champ.level > 2 { champ.skill2_effect.as_ref() } else { None }; if let Some(s) { if s.target.check(champ,t) { nuke += s.expected_damage_target(..) } } }
L1278       if champ.can_ult() || champ.ult_cooldown() <= cc_duration {
L1279~1281    s = if champ.level > 4 { champ.ult_effect.as_ref() } else { None }; (동일) }
L1286       combined_with_cc_skill = nuke_during_cc + expected_damage
L1287       if combined >= target_hp { L1289 score += min(hp_value,80) }
L1290       else if target_hp > 0 && combined*100/target_hp > 59 { L1292 score += min(hp_value,80)/3 }
          } }
L1298   total = score + v55_mark_value(version,effect,data,player,parameter,champ,t,hp_value,_debug)
L1299         + v55_seal_value(version,effect,data,champ,t,hp_value)
L1300         - v55_banish_penalty(version,effect,data,player,champ,t,hp_value)
        goto RET }
// ---- §H 아군 챔피언 대상 (L1302~1423) ----
L1302 else if let Some(tp) = parameter.near_allies.iter().find(|p| p.id==t.id) {
L1303   expected_heal = effect.expected_heal_target(ctx, champ as &dyn, t as &dyn)
L1304   expected_shield = effect.expected_shield_target(ctx, champ, t)
L1306   v55_tbuff: Option<BuffState> = v55_target_dependent_buff(effect,data,t)
L1307   has_buff = v55_tbuff.is_some() || effect_buff_target(version,effect,ctx,champ,t).is_some()   // 단락: v55 Some 이면 두 번째 미호출
L1309   v55_on_attack = effect.ty.expected_on_attack_damage(ctx, champ)          // vtable +0xb0
L1310   etc_buff = if has_buff || v55_on_attack > 0 || !effect.ty.etc_buff() { 0 }    // IR 평가 순서: on_attack>0 · !etc_buff() · has_buff 를 or 로 합침(단락 없음)
L1312             else if effect.ty.expected_mark(ctx, champ as &dyn).is_none() { 5 } else { 0 }   // vtable +0xa8 sret 48B, 태그 0=None
L1320   hp_value = champion_hp_value(data,parameter,tp)
L1322   possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1323   applyed_damage = tp.applyed_damage ; L1325 epic_tank = tp.risk_epic_damage
L1328   expected_shield = if possible_damage>0 || epic_tank!=0 || applyed_damage>0 { expected_shield } else { 0 }
L1338   incoming = applyed_damage + possible_damage + epic_tank ; epic_incoming = epic_tank
L1339   missing = max(t.stat_cached.hp - t.hp, 0)
L1340   expected_heal = min(missing + incoming*2, expected_heal) ; expected_shield = min(incoming*3, expected_shield)
L1347   if has_buff || v55_on_attack > 0 {
L1348     bs = v55_tbuff.clone().or_else(|| effect_buff_target(version,effect,ctx,champ,t))   // closure$12
          if let Some(bs) = bs {
L1350       crisis_needed = bs.cc_immune || bs.undying || bs.toughness == 0
L1351~1354  crisis: Option<(bool,bool)> = if crisis_needed { Some(defensive_crisis(version,_rnd,player,data,t,_debug)) } else { None }
L1360       v = buff_value_v54(&bs, t, data, player, parameter, crisis.as_ref(), incoming, epic_incoming, v55_on_attack, hp_value)
L1363       cast_range = effect.range + (champ.level-1)*growth + champ.stat_buff_cached.range + effect.range_adjust(champ,t) + champ.radius() + t.radius()
L1364       walk_ticks = champ.distance(t).saturating_sub(cast_range) / max(champ.stat_cached.move_speed,1)   (udiv)
L1365       delay_sec = walk_ticks / tps   (sdiv · tps==0 → panic anon.167)
L1366       v = clamp(6 - delay_sec, 0, 6) * v / 6
L1368       v_currency = area_buff_multi_scale(version,effect,data,player,t,v)
          } else if v55_on_attack > 0 {     // L1369
L1373       v = buff_value_v54(&BuffState::default(), t, data, player, parameter, None, incoming, epic_incoming, v55_on_attack, hp_value)
L1375       v_currency = area_buff_multi_scale(version,effect,data,player,t,v)
          } else { v_currency = 0 }
L1378     if has_buff && v_currency == 0 {
L1380       bs2 = v55_tbuff.clone().or_else(|| effect_buff_target(..)) ; w = noncombat_steroid_window(player,data,champ,t)   // Option<(bool,bool)>
            if bs2.is_none() || w.is_none() { L1392 expected_buff_score = 0 }
            else { L1383 v = noncombat_steroid_value(data, &bs2, t, &w) ; L1384 v = area_buff_multi_scale(version,effect,data,player,t,v)
L1386              if v > 0 && ctx.debug && tick % 6 == 0 { L1387 add_log("STEROID_WIN T{team} {pos:?} ally_target v={v} fights_back={w.1}") }   // anon.153
L1390              expected_buff_score = v }
          } else { L1395 expected_buff_score = v_currency }
        } else { L1398 expected_buff_score = 0 }
L1401   score = (expected_heal + expected_shield) * hp_value / t.hp   (sdiv · 0 → panic anon.168)
L1403   if expected_heal + expected_shield > 0 && score == 0 {
L1404     score = if applyed_damage > 0 || possible_damage > 0 || tp.risk_damage != 0 || t.hp < t.stat_cached.hp { 1 } else { 0 } }
L1411   aoe_ally_value = v54_aoe_ally_heal_value(version,effect,data,parameter,champ,t,t.id,action)
L1414   if has_buff && ctx.debug { L1415~1416 add_log("BUFFSCORE nm={action.action_name()} T{team} {pos:?} ally t={t.id} buff={expected_buff_score} hs={score} etc={etc_buff} aoe={aoe_ally_value}") }   // anon.154
L1419   total = expected_buff_score + score + etc_buff + aoe_ally_value
L1423   if total == 0 { total = if has_buff || expected_heal+expected_shield > 0 { -10 } else { 0 } }
        goto RET }
// ---- §I 자기 자신 대상 (L1429~1550) ----
L1429 else if t.id == champ.id {           // tp = parameter.player
L1430~1431 expected_heal/shield = effect.expected_heal_target/expected_shield_target(ctx,champ,t)
L1434~1435 v55_tbuff = v55_target_dependent_buff(effect,data,t) ; has_buff = v55_tbuff.is_some() || effect_buff_target(version,effect,ctx,champ,t).is_some()
L1437   v55_on_attack = effect.ty.expected_on_attack_damage(ctx,champ)
L1438~1440 etc_buff = (§H L1310~1312 와 동일)
L1450   epic_tank = parameter.player.risk_epic_damage
L1453   expected_shield = if possible_damage(§B 값) < 1 && (epic_tank | parameter.player.applyed_damage) == 0 { 0 } else { expected_shield }
L1462   incoming = possible_damage + applyed_damage + epic_tank ; epic_incoming = epic_tank
L1463   missing = max(champ.stat_cached.hp - champ.hp, 0)
L1464   expected_heal = min(missing + incoming*2, expected_heal) ; expected_shield = min(incoming*3, expected_shield)
L1469~1511 expected_buff_score = §H L1347~1395 와 같은 구조(대상 t=champ · hp_value=§D 의 hp_value) — 단 L1363~1366 의 사거리/도달 감쇠가 없다: v_currency = area_buff_multi_scale(version,effect,data,player,champ, buff_value_v54(&bs,champ,data,player,parameter,crisis.as_ref(),incoming,epic_incoming,v55_on_attack,hp_value)) (L1482~1484) ; bs None && on_attack>0 → default BuffState (L1489~1491) ; L1494 has_buff && v_currency==0 → L1496 bs2·w=noncombat_steroid_window(player,data,champ,champ) → L1499 noncombat_steroid_value → L1500 area_buff_multi_scale → L1502~1503 로그 "STEROID_WIN ... self v= fights_back="(anon.155) → L1506 expected_buff_score=v / L1508 0 ; L1511 else v_currency
L1514   (has_buff 아니고 on_attack<=0) expected_buff_score = 0
L1517   score = (expected_heal + expected_shield) * hp_value / champ.hp   (sdiv · 0 → panic anon.169)
L1523   ally_aura_protect = 0; for ally in cache.player_champion[team].iter().flatten() {      // 자기 자신 포함 5슬롯
L1524     if effect.ty.expected_ally_aura_heal_at(ctx, champ, ally) != 0 {                       // vtable +0x98
L1527       if let Some(atp) = near_allies.find(|p| p.id == ally.id) {
L1528         if defensive_crisis(version,_rnd,player,data,ally,_debug).0 {
L1529           ally_aura_protect += min(champion_hp_value(data,parameter,atp), 80) } } } }
L1537   v55_trigger = v55_spirit_trigger_value(effect,data,player,parameter,champ,hp_value)
L1540   aoe_ally_value = v54_aoe_ally_heal_value(version,effect,data,parameter,champ,champ,champ.id,action)
L1543   if has_buff && ctx.debug { L1544~1545 add_log("BUFFSCORE nm=.. T.. self buff= hs= etc= aura= trig= aoe=") }   // anon.156
L1548   total = score + expected_buff_score + etc_buff + ally_aura_protect + v55_trigger + aoe_ally_value
L1550   if total == 0 { total = if has_buff || expected_heal+expected_shield > 0 { -10 } else { 0 } }
        goto RET }
// ---- §J 그 밖(타워·미니언·근접목록 밖 챔피언) ----
L1558 else { if t.is_champion() && ctx.debug && t.team == champ.team { L1559 add_log("BUFFTGT_MISS T{team} {pos:?} t={t.id} near_allies") }   // anon.157
        total = 0 }
// ---- RET (L1564) ----
return sum3 + total          // sum3 = tower_score + base_score + move_score
// 반환 phi 전체: [v57 Some(v)] [§F 100+x] [sum3+total] [-9999999 L1020] [-9999999 L1034]
```

**`mem` 메모리 접근 80건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L982 (2352). >=2 이면 panic_bounds_check | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag (i32) | r | L581<982 my_position (2496); L1222 target_player 의 position | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 4 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — [1-team] 을 is_recent_visible 에 전달(L1028 클로저·L1115). _docs game_core:21 「Blackboard[team]=team 팀 자체 정보, 관측은 1-team」 | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data +0x0 / vtable +0x8) | r | vtable 슬롯 +0x28 tick() · +0x40 get_game_mode() -> {i64 태그, ptr} · +0xf8 is_visible(team, entity_id) -> bool (divtable AbstractGame 98%) | 3 | OK |  |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | L982 (480) champ = [team][my_position].expect; L1027/L1115 적팀 5슬롯; L1233/L1523 아군 5슬롯 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x280 | player_champion_cache[2][5] (ChampionCache 800B) | r | L1226 (640) [team][pos] | 4 | OK |  |
| 8 | ChampionCache | 0x190 | attack_per_sec[target_pos] | r | L1226 my_attack_dps · L1235 ally (400) | 4 | OK |  |
| 9 | ChampionCache | 0x1b8 | skill_per_sec[target_pos] | r | L1237 (440) | 4 | OK |  |
| 10 | ChampionCache | 0x1e0 | skill2_per_sec[target_pos] | r | L1239 (480) | 4 | OK |  |
| 11 | ChampionCache | 0x208 | ult_per_sec[target_pos] | r | L1241 (520) | 4 | OK |  |
| 12 | GameContext | 0x0 | pool (&Bump) | r | L1030 die_enemies collect · L1033 Vec::new_in | 4 | OK |  |
| 13 | GameContext | 0x8 | setting (&GameSetting) | r | L1086·L1090·L1123·L1223·L1365 | 4 | OK |  |
| 14 | GameContext | 0x38 | tutorial (TutorialType i8) | r | L1086 spawn_epic() 인라인: ∈{0 None,5 MidBottom,7 Line,8 Total} 이면 true (switch) | 4 | OK |  |
| 15 | GameContext | 0x3b | debug (bool) | r | L1013·L1386·L1414·L1502·L1543·L1558 디버그 로그 게이트 (59) | 4 | OK |  |
| 16 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | L1086 is_line_phase 인라인 (2216) | 4 | OK |  |
| 17 | GameSetting | 0x12f8 | tick_per_second | r | L1086 *30 · L1090/1091 window 하한/상한 · L1223 max(tps,1) · L1365 delay_sec (4856) | 4 | OK |  |
| 18 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | L1123 tick < 이 값일 때만 아군 타워 점수 (5112) | 4 | OK |  |
| 19 | ScoreParameter | 0x918 | player (ChampionScoreParameter 216B) | r | L1050/L1102/L1052 &parameter.player (2328) | 4 | OK |  |
| 20 | ScoreParameter | 0x988 | player.applyed_damage | r | L1050 (2440) · L1453 | 4 | OK |  |
| 21 | ScoreParameter | 0x998 | player.risk_damage | r | L1051 DI 이름 base_damage (2456) · L1108 | 4 | OK |  |
| 22 | ScoreParameter | 0x9a0 | player.risk_epic_damage | r | L1450 epic_tank (2464) | 4 | OK |  |
| 23 | ScoreParameter | 0x9b0 | player.risk_possible_tower | r | L1082 (2480) can_focused_tower\|\|t.is_champion 일 때 possible_damage 에 가산 | 4 | OK |  |
| 24 | ScoreParameter | 0x9f0 | positioning_score (PositioningScoreData 2760B) | r | L1137 position_score_at_position 인자 (2544) | 4 | OK |  |
| 25 | ScoreParameter | 0x14b8 | near_allies.buf.ptr | r | L1302·L1527 (5304) · len +0x14d0 (5328) | 4 | OK |  |
| 26 | ScoreParameter | 0x14d0 | near_allies.len | r |  | 4 | OK |  |
| 27 | ScoreParameter | 0x14d8 | near_enemies.buf.ptr | r | L1156·L1159·L1177·L1179 (5336) · 원소 stride 216 | 4 | OK |  |
| 28 | ScoreParameter | 0x14f0 | near_enemies.len | r | (5360) | 4 | OK |  |
| 29 | ScoreParameter | 0x1500 | v3_turnback_hold (bool) | r | L1013·L1020 (5376) | 4 | OK |  |
| 30 | ChampionScoreParameter | 0x58 | id | r | near_enemies/near_allies find·any 술어 `p.id == t.id` (88) | 4 | OK |  |
| 31 | ChampionScoreParameter | 0x70 | applyed_damage | r | L1323 (112) | 4 | OK |  |
| 32 | ChampionScoreParameter | 0x80 | risk_damage | r | L1160/L1180 base_damage · L1404 (128) | 4 | OK |  |
| 33 | ChampionScoreParameter | 0x88 | risk_epic_damage | r | L1325 epic_tank (136) | 4 | OK |  |
| 34 | ChampionScoreParameter | 0x98 | risk_possible_tower | r | L1162/L1182/L1322 possible_damage 에 가산 (152) | 4 | OK |  |
| 35 | Entity | 0x0 | team@tag (TeamType 0=Player/1=Neutral) | r | L1000·L1087·L1558 TeamType::eq 인라인 — 태그 비교 후 Player 면 +0x8 페이로드 비교 | 4 | OK |  |
| 36 | Entity | 0x8 | team@Player.0 (usize) | r |  | 4 | OK |  |
| 37 | Entity | 0x68 | ty@tag (EntityType) | r | 13=Champion (L1000·L1264·L1271·L1278) · 2=Tower (L1065) · attack_cooldown 인라인 switch (L1258) (104) | 4 | OK |  |
| 38 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag (Option<(usize,usize)>) | r | L1066 0=None 일 때만 타워 집중 판정 (136) | 4 | OK |  |
| 39 | Entity | 0xb0 | ty@Champion.info.attack_cooldown | r | L1258 attack_cooldown() 인라인(Champion 태그 13 → 176; 다른 EntityType 은 각자 info 오프셋 184/272/232/496/200/240/216/208) | 4 | OK |  |
| 40 | Entity | 0xb8 | ty@Champion.info.skill_cooldown | r | L1264 (184) | 4 | OK |  |
| 41 | Entity | 0xc0 | ty@Champion.info.skill2_cooldown | r | L1271 (192) | 4 | OK |  |
| 42 | Entity | 0xc8 | ty@Champion.info.ult_cooldown | r | L1278 (200) | 4 | OK |  |
| 43 | Entity | 0x438 | stat_buff_cached.range | r | L1067·L1071·L1363 range() 인라인 (1080) | 4 | OK |  |
| 44 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | radius() 인라인: 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 (1136) | 4 | OK |  |
| 45 | Entity | 0x488 | stat_buff_cached.undying (bool) | r | L985 champ · L1167/L1187 t (1160) | 4 | OK |  |
| 46 | Entity | 0x490 | attack_effect (Option<Effect>) | r | L1121 tower_atk · L1206·L1259 champ (1168) | 4 | OK |  |
| 47 | Entity | 0x4a0 | attack_effect@Some.range | r | L1071 (1184) | 4 | OK |  |
| 48 | Entity | 0x4a8 | attack_effect@Some.growth_range | r | L1071 (1192) | 4 | OK |  |
| 49 | Entity | 0x4c0 | attack_effect@tag (i32 니치 · -1=None) | r | L1071·L1121·L1206·L1259 (1216) | 4 | OK |  |
| 50 | Entity | 0x4c8 | skill_effect (Option<Effect>) | r | L1265 (1224) | 4 | OK |  |
| 51 | Entity | 0x4f0 | skill_effect@Some.target (CastingTarget) | r | L1266 check (1264) | 4 | OK |  |
| 52 | Entity | 0x4f8 | skill_effect@tag | r | L1265 (1272) | 4 | OK |  |
| 53 | Entity | 0x500 | skill2_effect (Option<Effect>) | r | L1272 level>2 일 때만 (1280); 아니면 정적 None(anon.121) | 4 | OK |  |
| 54 | Entity | 0x528 | skill2_effect@Some.target | r | L1273 (1320 = 1280+40) | 4 | OK |  |
| 55 | Entity | 0x530 | skill2_effect@tag | r | L1272 (1280+48) | 4 | OK |  |
| 56 | Entity | 0x538 | ult_effect (Option<Effect>) | r | L1279 level>4 일 때만 (1336) | 4 | OK |  |
| 57 | Entity | 0x560 | ult_effect@Some.target | r | L1280 (1336+40) | 4 | OK |  |
| 58 | Entity | 0x568 | ult_effect@tag | r | L1279 (1336+48) | 4 | OK |  |
| 59 | Entity | 0x5c0 | id | r | L1053 champ.id(is_visible) · L1014·L1156~ t.id · L1429 t.id==champ.id · L1527 ally.id (1472) | 4 | OK |  |
| 60 | Entity | 0x5c8 | level | r | range() 인라인 (level-1)*growth · L1272 >2 · L1279 >4 (1480) | 4 | OK |  |
| 61 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | L987 · L1339 · L1404 · L1463 (1576) | 4 | OK |  |
| 62 | Entity | 0x640 | stat_cached.move_speed | r | L1364 max(ms,1) (1600) | 4 | OK |  |
| 63 | Entity | 0x660 | x | r | distance_sq 인라인 다수 · L1072 · L1092 (1632) | 4 | OK |  |
| 64 | Entity | 0x668 | y | r | (1640) | 4 | OK |  |
| 65 | Entity | 0x670 | hp | r | L987 · L1108 · L1127 · L1167/L1172 · L1187/L1192 · L1339 · L1401 · L1463 · L1517 (1648) | 4 | OK |  |
| 66 | Entity | 0x680 | radius | r | radius() 인라인 (1664) | 4 | OK |  |
| 67 | Effect | 0x0 | ty (Arc<dyn EffectType> ptr +0x0 / vtable +0x8) | r | L1131·L1309·L1310·L1312·L1437·L1438·L1440·L1524 | 4 | OK |  |
| 68 | Effect | 0x10 | range | r | L1067·L1363 (16) | 4 | OK |  |
| 69 | Effect | 0x18 | growth_range | r | L1067·L1363 (24) | 4 | OK |  |
| 70 | Box<dyn Action> | 0x8 | vtable | r | action.duration()(+0x80=128) · can_use_with_move()(+0xb8=184) · action_name()(+0x78=120) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 71 | BuffState(Option, 288B) | 0x48 | Option 태그 (i32 -1=None) | r | v55_target_dependent_buff / effect_buff_target 반환 판정 (72) | 4 | OK |  |
| 72 | BuffState | 0xb8 | toughness | r | L1350/L1472 ==0 → crisis_needed (184) | 4 | OK |  |
| 73 | BuffState | 0xf8 | cc_immune (bool) | r | L1350/L1472 (248) | 4 | OK |  |
| 74 | BuffState | 0x118 | undying (bool) | r | L1350/L1472 (280) | 4 | OK |  |
| 75 | Option<(u64,u64,usize)> (line_action_stance sret 32B) | 0x18 | .2 walk_tick | r | +0x0 태그 · +0x8 sx · +0x10 sy · +0x18 walk_tick (24) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 76 | PositioningScore (sret 56B) | 0x0 | risk | r | L1140 move_score 산출에 이것만 읽음 | 4 | OK |  |
| 77 | (직접 쓰기 없음) | (간접) _rnd %1 | StdRng 320B | w | 이 함수 본문에는 %1 에 대한 store 0건 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | defensive_crisis(L1352·L1474·L1528) 가 RW 로 받음 — 롤 소비 가능. check_kill_die_tick 은 readnone(미사용) |
| 78 | (직접 쓰기 없음) | (간접) _debug %8 | DebugFrameData 224B | w | 이 함수 본문에는 %8 에 대한 store 0건. ctx.debug=false 면 add_log 6곳 전부 미도달 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | add_log ×6 → logs(+0x48 Vec<DebugLog>) push(힙 성장 = HEAP 재료) · v55_mark_value(L1298) RW · defensive_crisis RW |
| 79 | (지역 힙) | (간접) bump Vec | die_enemies: bumpalo Vec<&Entity>(L1026~1030 collect, ctx.pool) · Vec::new_in(bump)(L1033) | w | String 6개(format_inner sret 24B)는 add_log 후 drop | 4 | 확인불가(오프셋 파싱 실패) | L1038 drop_glue 로 소멸(len==0 경로) — bump 할당이라 힙 free 없음 |

**`consts` 상수 24건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 35 | 987 | 계수 | champ.hp*100 > max(max_hp,1)*35 — HP 35% 초과일 때만 base_defense_focus 판정 (version>1) | 4 |  |
| 1 | 100 | 987 | 인덱스 | 백분율 스케일 (HP% 비교 L987 · radius() mult+100 /100 · L1127 *100 후 min 100 · L1140 /100 · L1200 min(hp_value,100) · L1290 *100) | 4 |  |
| 2 | 1 | 987 | 임계 | ①umax(x,1) 0나눗셈 하한(L987 max_hp · L1223 tps · L1364 move_speed) ②`shl i64 %tps, 1`=tps*2 (L1091 window 상한) / `lshr 1`=tps/2 (L1090 하한) / `shl 1`=incoming*2 (L1340·L1464) / `lshr 1`=current_damage/2 (L1094) — 접힘 ③L1404 score 최솟값 1 ④(champ.level-1) | 4 | 2 |
| 3 | 13 | 1000 | 태그 | EntityType::Champion 메모리태그 13 (tcxdict --enum EntityType) — t.is_champion() · champ 스킬 쿨다운 접근 게이트 | 3 |  |
| 4 | 2 | 1001 | 태그 | GameMode::DeathMatch 태그 2 (L1001, reach gamemode=0 접힘으로 항상 비-DM) · EntityType::Tower 태그 2 (L1065) · L1166/L1186 base_damage/2 · L1211 /2 · Option<(bool,bool)> None 태그 2 (L1354/L1476 store i8 2 · noncombat_steroid_window 반환 .0==2) · 팀 배열 길이 2(bounds) | 4 |  |
| 5 | 22500000000 | 1061 | 임계 | 150000² — 적/아군 타워 후보 필터 distance_sq(champ) < 150000 (엄격 부등, L1061 closure$1·L1112 closure$3, aux 심에도 동일) | 4 |  |
| 6 | 22500000001 | 1234 | 임계 | 150000²+1 — distance_sq < 이 값 = 거리 ≤150000: L1234 아군 dps 합산 범위 · L1028 die_enemies 필터(aux closure$0) | 4 |  |
| 7 | 15000 | 1074 | 계수 | 타워 집중 판정 여유: dist(champ,tower) <= (d-사거리) + 15000 + 타워사거리 + 반지름들 | 4 |  |
| 8 | 3 | 1054 | 태그 | 적에게 비가시일 때 possible_damage /3 (L1054) · L1203 *3/4 · L1292 /3 · L1340·L1464 incoming*3 shield 상한 | 4 |  |
| 9 | 30 | 1052 | 계수 | possible_risk(data, action.duration()+30) — 30tick 여유(L1052·L1162·L1182·L1322) · L1086 tps*30 = 에픽 첫 스폰 30초 전 | 4 |  |
| 10 | 7 | 1086 | 태그 | TutorialType::Line(7) — spawn_epic() 인라인 switch {0 None,5 MidBottom,7 Line,8 Total} 이면 true | 4 |  |
| 11 | 8 | 1086 | 태그 | TutorialType::Total(8) (위 switch) | 4 |  |
| 12 | 5 | 1086 | 임계 | TutorialType::MidBottom(5) (위 switch) · L1312/L1440 etc_buff = 5 (expected_mark None 일 때) · 챔피언 슬롯 5 | 4 |  |
| 13 | 15000000000 | 1115 | 임계 | ≈122474² — has_near_enemy_champion: 적 챔피언 distance_sq(champ) < 이 값 && 최근 가시 | 4 |  |
| 14 | 1225000001 | 1134 | 임계 | 35000²+1 — champ.distance_sq(t) < 이 값(≤35000)이면 move_score=0 (이동형 이펙트만) | 4 |  |
| 15 | 12 | 1137 | 센티널 | PositionEvalPurpose::AttackStance 메모리태그 12 (니치 · idx 10) — position_score_at_position purpose 인자 | 4 |  |
| 16 | 150000 | 1157 | 인덱스 | lapse_proximity_score(closure$6) = (150000 - min(dist,150000)) / 1500 — 0~100 | 4 |  |
| 17 | 1500 | 1157 | 계수 | 위 나눗셈 (거리 1500 당 1점) | 4 |  |
| 18 | 4 | 1166 | 임계 | total_damage = expected + base_damage/2 + possible_damage/4 (L1166·L1186) · L1203 *3/4 분모 | 4 |  |
| 19 | 80 | 1251 | 임계 | 상한: min(cc_score,80) L1251 · min(hp_value,80) L1289·L1292 · min(champion_hp_value(atp),80) L1529 | 4 |  |
| 20 | 59 | 1290 | 임계 | combined_with_cc_skill*100/target_hp > 59 (=60% 이상) 이면 score += min(hp_value,80)/3 | 4 |  |
| 21 | 6 | 1366 | 임계 | clamp(6 - delay_sec, 0, 6) * v / 6 — 도달까지 6초 넘으면 버프 가치 0 · L1386/L1502 tick % 6 == 0 로그 샘플링 | 4 |  |
| 22 | -10 | 1423 | 산출값 | 아군/자기 대상 total==0 인데 has_buff 이거나 heal+shield>0 이면 total=-10 (L1423·L1550) | 4 |  |
| 23 | -9999999 | 1020 | 산출값 | 거부 반환값: L1020 v3_turnback_hold(사거리 밖) · L1034 my_die <= duration+walk_tick | 4 |  |

**`knobs` 조정점 16건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 기지방어 판정 HP 하한 | action_score.rs:987 | 35 | 올리면 더 높은 HP 에서도 base_defense_focus 를 물어 no_self_risk 가 될 여지가 늘어 base_score 가 -1 로 고정되는(위험 무시) 경우 증가 | 4 | 기존 |
| 1 | 타워 후보 탐색 반경(제곱) | action_score.rs:1061 / 1112 | 22500000000 | 올리면 더 먼 적/아군 타워를 집중·엄호 후보로 봄 (150000) | 4 | 기존 |
| 2 | 적 타워 집중 판정 여유거리 | action_score.rs:1074 | 15000 | 올리면 타워가 더 먼 상황도 '집중당함' 으로 판정 → risk_possible_tower 가산 빈도 증가 | 4 | 기존 |
| 3 | 비가시 시 위험 감산 분모 | action_score.rs:1054 | 3 | 올리면 적에게 안 보일 때 possible_damage 를 더 낮게 봄(공격적) | 4 | 기존 |
| 4 | possible_risk 시간 여유(tick) | action_score.rs:1052/1162/1182/1322 | 30 | 올리면 더 긴 창의 위험을 계산 → 위험 항 증가 | 4 | 기존 |
| 5 | 미니언 투사 활성 시점(에픽 첫 스폰 n초 전) | action_score.rs:1086 | 30 | 올리면(tps*30 → 더 큼) 더 이른 tick 부터 projected_minion_damage 를 셈 | 4 | 기존 |
| 6 | 근접 적 챔피언 반경(제곱) | action_score.rs:1115 | 15000000000 | 올리면 더 먼 적이 있어도 아군 타워 점수(tower_score) 가 활성 | 4 | 기존 |
| 7 | 이동형 이펙트 최소 거리(제곱+1) | action_score.rs:1134 | 1225000001 | 올리면 더 먼 대상까지 move_score=0 (위치평가 생략) | 4 | 기존 |
| 8 | 인지공백 근접 점수 기준거리/분모 | action_score.rs:1157 | 150000 | 150000/1500: 내리면 같은 거리에서 lapse 점수가 낮아짐 | 4 | 기존 |
| 9 | 킬 확정 보너스 상한 | action_score.rs:1200/1203/1211 | 100 | min(hp_value,100)·×3/4·÷2 — 내리면 즉사/근사 킬 보너스 축소 | 4 | 기존 |
| 10 | CC 후속·누킹 보너스 상한 | action_score.rs:1251/1289/1292 | 80 | 내리면 CC 콤보 가치 축소 | 4 | 기존 |
| 11 | CC 중 누킹 비율 임계(%) | action_score.rs:1290 | 59 | >59 → 60% 이상이면 부분 보너스; 올리면 보너스 조건 엄격 | 4 | 기존 |
| 12 | 아군 버프 도달 감쇠 창(초) | action_score.rs:1366 | 6 | clamp(6-delay,0,6)/6 — 올리면 먼 아군에게도 버프 가치 유지 | 4 | 기존 |
| 13 | 무효 버프 벌점 | action_score.rs:1423/1550 | -10 | total==0 인 버프/힐 액션에 -10; 0 으로 하면 무의미 버프가 중립 | 4 | 기존 |
| 14 | 표식 없는 etc 버프 가치 | action_score.rs:1312/1440 | 5 | 올리면 잡버프 선호 | 4 | 기존 |
| 15 | 거부 반환값 | action_score.rs:1020/1034 | -9999999 | 턴백홀드/처치예상 시 후보 제거 수준 | 4 | 기존 |

<details><summary>`callees` 피호출자 92건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_name | game_core::Action::action_name | pub | fn(&Self/#0, &game_core::Entity) -> std::string::String | game-core\src\setting\action.rs:15 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 193개 중 상위 3개 |
| 1 | action_name | <game_core::EmptyAction as game_core::Action>::action_name | pub | fn(&game_core::EmptyAction, &game_core::Entity) -> std::string::String | game-core\src\setting\action\common.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 193개 중 상위 3개 |
| 2 | action_name | <game_core::DataActionDef as game_core::Action>::action_name | pub | fn(&game_core::DataActionDef, &game_core::Entity) -> std::string::String | game-core\src\setting\champion\data_driven.rs:2255 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 193개 중 상위 3개 |
| 3 | add_log | game_core::DebugFrameData::add_log | pub | fn(&mut game_core::DebugFrameData, &game_core::OperationData, &game_core::PlayerState, std::string::String) | game-core\src\simulation\game\frame.rs:54 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | area_buff_multi_scale | game_ai::buff_value::area_buff_multi_scale | in:game_ai | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, i64) -> i64 | game-ai\src\buff_value.rs:771 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | attack_cooldown | game_core::Entity::attack_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1747 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | base_defense_focus | game_ai::plan_legacy::old::base_defense_focus | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:200 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | buff_value_v54 | game_ai::buff_value::buff_value_v54 | in:game_ai | fn(&game_core::BuffState, &game_core::Entity, &game_core::OperationData, &game_core::PlayerState, &game_ai::ScoreParameter, std::option::Option<&game_ai::DefensiveCrisis>, i64, i64, i64, i64) -> i64 | game-ai\src\buff_value.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | can_use_with_move | game_core::Action::can_use_with_move | pub | fn(&Self/#0) -> bool | game-core\src\setting\action.rs:28 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 11개 중 상위 3개 |
| 13 | can_use_with_move | <game_core::DataActionDef as game_core::Action>::can_use_with_move | pub | fn(&game_core::DataActionDef) -> bool | game-core\src\setting\champion\data_driven.rs:2279 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 11개 중 상위 3개 |
| 14 | can_use_with_move | <game_core::BardSkillAction as game_core::Action>::can_use_with_move | pub | fn(&game_core::BardSkillAction) -> bool | game-core\src\setting\champion\bard.rs:302 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 11개 중 상위 3개 |
| 15 | champion_hp_value | game_ai::champion_hp_value | pub | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64 | game-ai\src\utils.rs:909 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | default | <game_ai::GoalData as std::default::Default>::default | pub | fn() -> game_ai::GoalData | game-ai\src\goal_data.rs:10 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 988개 중 상위 3개 |
| 19 | default | <game_ai::EpicStance as std::default::Default>::default | pub | fn() -> game_ai::EpicStance | game-ai\src\goal_data.rs:133 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 988개 중 상위 3개 |
| 20 | default | <game_ai::EpicStanceData as std::default::Default>::default | pub | fn() -> game_ai::EpicStanceData | game-ai\src\goal_data.rs:145 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 988개 중 상위 3개 |
| 21 | defensive_crisis | game_ai::defensive_crisis | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &mut game_core::DebugFrameData) -> game_ai::DefensiveCrisis | game-ai\src\buff_value.rs:17 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 25 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 26 | duration | game_core::Action::duration | pub | fn(&Self/#0) -> usize | game-core\src\setting\action.rs:16 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 197개 중 상위 3개 |
| 27 | duration | game_view::UIPhaseEffect::duration | pub | fn(&game_view::UIPhaseEffect) -> f32 | game-view\src\ui\match_ui\phase_effect.rs:35 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 197개 중 상위 3개 |
| 28 | duration | game_view::ui::match_ui::BanpickShowcaseFx::duration | in:game_view::ui::match_ui | fn(&game_view::ui::match_ui::BanpickShowcaseFx) -> f32 | game-view\src\ui\match_ui.rs:1664 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 197개 중 상위 3개 |
| 29 | effect_buff_target | game_ai::effect_buff_target | pub | fn(usize, &game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-ai\src\fight_check.rs:390 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | effective_action_cc_time | game_ai::fight_check::effective_action_cc_time | in:game_ai | fn(usize, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect) -> std::option::Option<usize> | game-ai\src\fight_check.rs:314 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | enemy_minion_wave_danger_damage_at | game_ai::enemy_minion_wave_danger_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize) -> usize | game-ai\src\minion_wave_risk.rs:99 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | enemy_minion_wave_risk_damage_at | game_ai::enemy_minion_wave_risk_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize) -> usize | game-ai\src\minion_wave_risk.rs:6 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | etc_buff | game_core::EffectType::etc_buff | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:304 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 34 | etc_buff | <game_core::BanishEffect as game_core::EffectType>::etc_buff | pub | fn(&game_core::BanishEffect) -> bool | game-core\src\simulation\effect\type\banish.rs:84 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 35 | etc_buff | <game_core::CombineEffect as game_core::EffectType>::etc_buff | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:54 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 36 | expected_ally_aura_heal_at | game_core::EffectType::expected_ally_aura_heal_at | pub | fn(&Self/#0, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\effect\type.rs:307 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 37 | expected_ally_aura_heal_at | <game_core::CombineEffect as game_core::EffectType>::expected_ally_aura_heal_at | pub | fn(&game_core::CombineEffect, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\effect\type\combine.rs:59 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 38 | expected_ally_aura_heal_at | <game_core::AddCasterEffectBuffEffect as game_core::EffectType>::expected_ally_aura_heal_at | pub | fn(&game_core::AddCasterEffectBuffEffect, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\effect\type\add_buff.rs:244 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 39 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | expected_heal_target | game_core::Effect::expected_heal_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | expected_mark | game_core::EffectType::expected_mark | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::MarkProfile> | game-core\src\simulation\effect\type.rs:315 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 42 | expected_mark | <game_core::CombineEffect as game_core::EffectType>::expected_mark | pub | fn(&game_core::CombineEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::MarkProfile> | game-core\src\simulation\effect\type\combine.rs:76 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 43 | expected_mark | <game_core::HitmanUltEffect as game_core::EffectType>::expected_mark | pub | fn(&game_core::HitmanUltEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::MarkProfile> | game-core\src\simulation\effect\type\hitman_ult.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 44 | expected_move_on_hit | game_core::EffectType::expected_move_on_hit | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:293 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 45 | expected_move_on_hit | <game_core::CombineEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:96 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 46 | expected_move_on_hit | <game_core::DelayedEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::DelayedEffect) -> bool | game-core\src\simulation\effect\type\delayed.rs:45 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 47 | expected_on_attack_damage | game_core::EffectType::expected_on_attack_damage | pub | fn(&Self/#0, &game_core::GameContext, &game_core::Entity) -> usize | game-core\src\simulation\effect\type.rs:319 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 48 | expected_on_attack_damage | game_core::EffectBuff::expected_on_attack_damage | pub | fn(&Self/#0, &game_core::GameContext, &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:159 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 49 | expected_on_attack_damage | <game_core::CombineEffect as game_core::EffectType>::expected_on_attack_damage | pub | fn(&game_core::CombineEffect, &game_core::GameContext, &game_core::Entity) -> usize | game-core\src\simulation\effect\type\combine.rs:80 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 50 | expected_rush_effect | game_core::EffectType::expected_rush_effect | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:291 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 51 | expected_rush_effect | <game_core::RushEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::RushEffect) -> bool | game-core\src\simulation\effect\type\rush.rs:53 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 52 | expected_rush_effect | <game_core::CombineEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:42 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 53 | expected_shield_target | game_core::Effect::expected_shield_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect.rs:114 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 54 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 55 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 56 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 57 | is_champion | game_core::EntityType::is_champion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 58 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 59 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 60 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 61 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 62 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 63 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 64 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 65 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 66 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 67 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 68 | line_action_stance | game_ai::lane_economy::line_action_stance | in:game_ai | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::Effect) -> std::option::Option<(u64, u64, usize)> | game-ai\src\lane_economy.rs:77 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 69 | line_effect_range_with_radii | game_ai::lane_economy::line_effect_range_with_radii | in:game_ai | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\lane_economy.rs:72 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 70 | nexus_final_stand | game_ai::plan_legacy::old::nexus_final_stand | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:190 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 71 | noncombat_steroid_value | game_ai::buff_value::noncombat_steroid_value | in:game_ai | fn(&game_core::OperationData, &game_core::BuffState, &game_core::Entity, &game_ai::NoncombatSteroidWindow) -> i64 | game-ai\src\buff_value.rs:458 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 72 | noncombat_steroid_window | game_ai::buff_value::noncombat_steroid_window | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> std::option::Option<game_ai::NoncombatSteroidWindow> | game-ai\src\buff_value.rs:378 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 73 | player_awareness_lapse | game_ai::player_awareness_lapse | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\utils.rs:538 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 74 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 75 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 76 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 77 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 78 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 79 | skill2_cooldown | game_core::Entity::skill2_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1789 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 80 | skill_cooldown | game_core::Entity::skill_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1774 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 81 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 82 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 83 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 84 | ult_cooldown | game_core::Entity::ult_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1804 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 85 | v54_aoe_ally_heal_value | game_ai::buff_value::v54_aoe_ally_heal_value | in:game_ai | fn(usize, &game_core::Effect, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, usize, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>) -> i64 | game-ai\src\buff_value.rs:562 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 86 | v55_banish_penalty | game_ai::buff_value::v55_banish_penalty | in:game_ai | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, i64) -> i64 | game-ai\src\buff_value.rs:596 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 87 | v55_mark_value | game_ai::buff_value::v55_mark_value | in:game_ai | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, i64, &mut game_core::DebugFrameData) -> i64 | game-ai\src\buff_value.rs:724 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 88 | v55_seal_value | game_ai::buff_value::v55_seal_value | in:game_ai | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::Entity, &game_core::Entity, i64) -> i64 | game-ai\src\buff_value.rs:516 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 89 | v55_spirit_trigger_value | game_ai::buff_value::v55_spirit_trigger_value | in:game_ai | fn(&game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_ai::ScoreParameter, &game_core::Entity, i64) -> i64 | game-ai\src\buff_value.rs:682 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 90 | v55_target_dependent_buff | game_ai::buff_value::v55_target_dependent_buff | in:game_ai | fn(&game_core::Effect, &game_core::OperationData, &game_core::Entity) -> std::option::Option<game_core::BuffState> | game-ai\src\buff_value.rs:635 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 91 | v57_summon_command_score | game_ai::buff_value::v57_summon_command_score | in:game_ai | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Entity) -> std::option::Option<i64> | game-ai\src\buff_value.rs:785 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 11개**: `Filter`, `clamp`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `find  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `format_inner`, `lapse_proximity_score`, `new  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 150개는 **전부 다른 함수**라 싣지 않는다`, `or_else`, `possible_damage`, `target_hp`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m05.ll:37422, m05.ll:37443, m05.ll:37469, m05.ll:37495) · **형제 0개** 

**`open` 13건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L1108 base_score 의 소스 표기: IR 은 `xor -1`(비트 NOT) — `!(x)` 인지 `-1 - x` 인지 표기 불가(외연 동일). no_self_risk 경로의 -1 과 정합. | 4 |  |
| 1 | 표기 불가 | L1310 etc_buff 조건 3항의 소스 순서: IR 이 세 i1 을 or 로 접어(단락 없음) `A\|\|B\|\|C` 의 순서 표기 불가. 동작은 확정. | 4 |  |
| 2 | 표기 불가 | L1156 `any(..) && in_lapse` 의 소스 순서: IR 은 any 루프 안에서 매치 시 in_lapse 분기 — `in_lapse && any` 로 써도 외연 동일(any 무부작용). 표기 불가. | 4 |  |
| 3 | 미탐색 | min_by_key fold 조각(m06.ll 29654~29928 / 29931~30205)과 filter/filter_map 이터레이터 조각(m01.ll 38145~38320, m11.ll 22466~22855, m12.ll 17138~17181)은 심볼 이름·타입으로만 판독(distance_sq 키 · closure 술어) — 본문 미독. 동률 시 min_by_key 는 첫 최솟값(표준 의미). | 4 |  |
| 4 | 미탐색 | Action vtable 슬롯 이름(+0x78 action_name · +0x80 duration · +0xb8 can_use_with_move)은 divtable 일치율 58% 의 정적 vtable(TargetAttackAction/TargetProjectileAction) 기준 — 트레이트 메서드 순서는 구현체 무관이라 슬롯↔이름은 유효하나 런타임 구현체는 미상. | 3 |  |
| 5 | 미탐색 | EffectType vtable 슬롯명은 g02.ll:1311 CombineEffect 정적 vtable 을 손으로 셈(+0x60 expected_rush_effect · +0x68 expected_move_on_hit · +0x90 etc_buff · +0x98 expected_ally_aura_heal_at · +0xa8 expected_mark · +0xb0 expected_on_attack_damage). +0xa8 expected_mark 의 반환(sret 48B, 태그 i64 0=None) 페이로드 타입 미확인. | 4 |  |
| 6 | 미탐색 | `_debug` 를 DebugFrameData::add_log 에 넘길 때 실제 logs Vec push 여부·형식은 콜리(g13.ll) 미독 — 힙 성장 지점이라고만 추정. | 5 |  |
| 7 | 미탐색 | check_kill_die_tick 의 두 번째 Vec 인자(L1033 Vec::new_in(bump), 빈 벡터)의 의미(아군 목록?)는 콜리 명세 소관. | 4 |  |
| 8 | 미탐색 | Blackboard::is_recent_visible(self=&blackboard[1-team], game(dyn 2워드), player, entity) 인자 의미는 콜리 소관 — _docs game_core:21 로 [1-team]=적팀 정보를 우리 팀이 관측한 판 임을 확인. | 4 |  |
| 9 | 미탐색 | L1258 attack_cooldown() 인라인의 EntityType 별 오프셋(Minion 184·Tower 272·Jungle 232·Epic/Serpen 496·Ghoul 176·SmallJiangshi 200·Bear 240·Eagle 216·Revenant 208·Champion 176)은 IR phi 그대로이며 tcxdict 로 Champion(+0xb0)만 교차검증. None/Nexus(태그 0·3)는 쿨다운 없음 = 통과. | 3 |  |
| 10 | 미탐색 | reach: 접힌 분기 1(@246 gamemode!=DeathMatch → 항상 통과) 외 사장 블록 0 — 본 명세의 전 분기가 live. | 4 |  |
| 11 | 미탐색 | possible_risk(&ChampionScoreParameter, &OperationData, i64) 반환 부호: i64 로 sdiv 하므로 음수 가능성은 콜리 명세 소관. | 4 |  |
| 12 | 미탐색 | 체감 소요: 원문 4,141줄을 주석본(3,749줄)→루트줄 태깅→압축(3,239줄)으로 17개 절로 나눠 읽음. 막힌 곳: ①EffectType 이 Arc<dyn> 이라 divtable 이 _gcbc 를 훑어 느림(슬롯 1개 5~9분) → g02.ll 정적 vtable 한 줄을 직접 파싱해 전 슬롯 확보 ②`;L` 사슬이 잘려 루트줄을 잃는 줄이 많아 원문 !dbg 를 직접 inlinedAt 루트까지 푸는 스크립트를 먼저 만듦. | 3 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L1086 인라인 헬퍼 이름·극성: scope 명 `spawn_epic`·`is_line_phase` 만 확인. IR 극성은 「tick >= first_spawn_tick - tps*30 이면 진행」. 소스가 `is_line_phase()` 인지 `!is_line_phase()` 인지는 헬퍼 본문(game_core 인라인)을 안 봐 미확정 — 동작은 IR 대로 확정. | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

