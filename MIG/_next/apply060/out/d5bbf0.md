# d5bbf0→f77820 calculate_interaction_action_score

## logic_060
```
// action_score.rs:980 · 0.6.0 f77820 · sig 동일: fn(version, rnd, player, data, parameter, action:&Box<dyn Action>, effect, t, debug) -> i64
// ★0.6.0 오프셋: PlayerState team +0xa00 · pos +0xa90 · id +0x9f8 · lapse 게이트 +0x49c · +0x208/+0x210/+0x478/+0x490(lapse 인자)
//   ScoreParameter: player +0x918(ChampionScoreParameter: id +0x970 · applyed +0x988 · risk +0x998 · epic +0x9a0 · tower +0x9b0 · possible_risk Vec +0x930/+0x948) · near_allies +0x14b8/+0x14d0 · near_enemies +0x14d8/+0x14f0 (원소 0xd8: possible_risk Vec +0x18/+0x30 · id +0x58 · team +0x60 · applyed +0x70 · risk +0x80 · epic +0x88 · tower +0x98) · positioning_score(구 +0x… 인라인) · v3_turnback_hold +0x1500
//   Entity hp +0x670 · max_hp +0x628 · ms +0x640 · xy +0x660/+0x668 · id +0x5c0 · level +0x5c8 · ty +0x68 · undying +0x488 · stat_buff.range +0x438 · radius +0x680 · mult +0x470 · attack_effect +0x490 · skill +0x4c8 · skill2 +0x500 · ult +0x538 · Tower.nearest_enemy +0x88
//   Blackboard stride 0x5c8 · small_actions +0x1d0+pos*0x18 {tag +0 · target +8} · last_seen +0x3e8
//   PlayerChampionCache 팀 0x1090B stride(구 0xfa0) · per-champ 0x350(구 0x320 · rel≥0x230 +0x30)
//   Effect vt(0.6.0): +0x58 heal · +0x60 shield · +0x70 ★신규(대상 역효과 · v3 꼬리 패널티) · +0x80 rush · +0x88 move_on_hit · +0x98 aura · +0xa8 mark · +0xb0 etc_buff · +0xb8 can_use_with_move(Action) · +0xc0 effect_buff_target · +0xc8 mark(sret) · +0xd0 on_attack · +0x108 range_adjust ; Action vt +0x80 duration
// 콜리 v3: last_stand_flags c87850→db0160(3-bool 캐시 · byte0=nexus_last_stand e76aa0 · byte1=nexus_final_stand e78290 · byte2=base_attacking_minion e78a10 · 0.5.8 과 동구조) · v57_summon e04400→e85460 · distance 12a07d0→1660a00 · add_log 16fc160→1413f0790 · line_action_stance e279f0→e72970 · die_enemies ca21c0→e0f1d0 · check_kill_die_tick eb82d0→eda920 · possible_risk d83230→ff1fc0 · 최근접 타워 fold d729b0→e7e540 · wave dmg at d95d00→1007030 · champion_hp_value→db0400/de3d60 · position_score d84db0→ff5ec0 · lapse core 16047b0→17d68b0 · player_by_champion_id d31bb0→d72be0 · can_attack/skill/skill2/ult 164b370/165d410/164b650/165b800 · target.check 165e7a0 · v55 mark/seal/banish e82410/e82990/e83000 · v55_target_dependent_buff e85820 · buff_value_v54 e80960(12인자) · area_buff_multi_scale e832b0 · defensive_crisis e82c00(7인자 · 6번째 extra_tick) · noncombat_steroid_window e84340 · v54_aoe_ally_heal_value e83ba0 · v55_spirit_trigger_value e84f30 · ★신규 f5d740(적 소액션이 t 조준 판정)

// ---- §A 도입·v3 도주 차단 (L980~1034) ---- 그대로
team = player.info.team; team>=2 → panic_bounds_check. my_position = player.info.position 태그
champ = data.cache.player_champion[team][my_position].expect(..)
L985 no_self_risk = if champ.stat_buff_cached.undying { true }
L986   else if version>1 && nexus_final_stand(player,data) /*db0160 byte1*/ { true }
L987   else if version>1 && champ.hp*100 > max(champ.stat_cached.hp,1)*35 { L988 base_defense_focus(player,data) /*db0160 byte0||byte2*/ }
       else { false }
L991 if let Some(v) = v57_summon_command_score(e85460)(data,player,champ,action,t) { return v }
L1000 if version>1 && t.ty==Champion(13) && t.team != champ.team
L1001    && game.get_game_mode() 태그 != DeathMatch(2) {
L1013   if ctx.debug && parameter.v3_turnback_hold(+0x1500) {
L1014     _debug.add_log(format!("TBHOLD-CAST T{team} {pos:?} tgt={t.id} d={champ.distance(t)} reach={line_effect_range_with_radii(effect,champ,t)} held={d>reach}")) }
L1019   if champ.distance(t) > line_effect_range_with_radii(effect,champ,t) {          // 사거리 밖
L1020     if parameter.v3_turnback_hold { return -9999999 }
L1023     if !no_self_risk {
L1024       if let Some((_,_,walk_tick)) = line_action_stance(e72970)(data,champ,t,effect) {
L1025         if walk_tick != 0 {
L1026~1030      die_enemies = e0f1d0(...) = iter_champions(1-team).filter(|e| e.distance_sq(champ) < 22500000001 && bb[1-team].is_recent_visible(game,player,e)).collect_in(pool)
L1031           if !die_enemies.is_empty() {
L1032             my_die = check_kill_die_tick(eda920)(version, data, player, champ, &die_enemies, &towers=Vec::new_in(pool), false, false, false, &None)   // ★0.6.0 ABI: bool×3=false · Option None
L1034             if my_die <= action.duration(vt+0x80) + walk_tick { return -9999999 }
                } } } } } }
// ---- §B 기본 위험 · 적 타워 집중 (L1050~1082) ---- 그대로
L1050 applyed_damage = parameter.player.applyed_damage(+0x988) ; L1051 base_damage = parameter.player.risk_damage(+0x998)
L1052 possible_damage = parameter.player.possible_risk(ff1fc0)(data, action.duration()+30)
L1053 is_visible = game.is_visible(vt+0xf8)(1-team, champ.id)
L1054 if !is_visible { possible_damage /= 3 }   (sdiv)
L1060~1061 near_enemy_tower = e7e540(...) = iter_towers_without_nexus(1-team).filter(|tw| tw.distance_sq(champ) < 22500000000).min_by_key(|tw| tw.distance_sq(champ))
L1064 can_focused_tower = false
      if let Some(tower) = near_enemy_tower {
L1065   if tower.ty == Tower(2) && L1066 tower.Tower.info.nearest_enemy(+0x88).is_none() {
L1067     range = effect.range + (champ.level-1)*effect.growth_range + champ.stat_buff_cached.range + effect.range_adjust(vt+0x108)(champ,tower) + champ.radius() + t.radius()
L1068     d = t.distance(champ) ; L1069 d = d.saturating_sub(range)
L1071     tower_range = tower.attack_effect.unwrap().range + (tower.level-1)*growth_range + tower.stat_buff_cached.range   // casting −1 → unwrap_failed
L1072     dist = utils::distance(champ.x,champ.y,tower.x,tower.y)
L1074     can_focused_tower = dist <= d + 15000 + tower_range + tower.radius() + champ.radius()
        } }
L1081~1082 if can_focused_tower || t.is_champion() { possible_damage += parameter.player.risk_possible_tower(+0x9b0) }
// ---- §C 미니언 웨이브 투사 피해 (L1086~1094) ---- ★0.6.0 산식 변경(v2/v3 공통)
L1086 tick = game.tick(); spawn_epic = ctx.tutorial ∈ {0,5,7,8};
      line_phase_ok = !(tick < setting.epic_jungle.first_spawn_tick(+0x8a8).saturating_sub(tps*30))
L1087 projected_minion_damage = 0; if spawn_epic && line_phase_ok && t.team != champ.team && t.is_champion() {
L1088   if let Some((sx,sy,walk_tick)) = line_action_stance(data,champ,t,effect) {
L1089     a = action.duration() + walk_tick
L1090     window = max(tps/2, a) ; L1091 window = min(tps*2, window)
L1092     current = enemy_minion_wave_risk_damage_at(1007030)(version,data,champ,champ.x,champ.y,window)
L1093     stance  = enemy_minion_wave_danger_damage_at(1007030)(version,data,champ,sx,sy,window)
          // ★0.6.0 (0.5.8: projected = stance.saturating_sub(current/2))
          hp = champ.hp; hp_pct = hp*100/max(champ.max_hp,1); dmg_pct = stance*100/max(hp,1)
          adj = if stance == 0 { 0 }
                else if hp <= stance || dmg_pct >= 50 || (hp_pct < 66 && dmg_pct >= 30)
                     || (hp_pct < 41 && dmg_pct >= 18) || (hp_pct < 26 && dmg_pct >= 10) { stance } else { 0 }
L1094     projected_minion_damage = adj.saturating_sub(current/2) } }
// ---- §D 기본 점수 (L1102~1108) ---- 그대로
L1102 hp_value = champion_hp_value(db0400)(data, parameter, &parameter.player)
L1105 base_score = if no_self_risk { -1 }
L1108              else { !( (possible_damage + base_damage + projected_minion_damage) * hp_value / champ.hp ) }   // = -x-1 · sdiv · champ.hp==0 → div_by_zero
// ---- §E 아군 타워 · 근접 적 · 이동 점수 (L1111~1149) ----
L1111~1112 near_ally_tower = iter_towers_without_nexus(team).filter(|tw| tw.distance_sq(champ) < 22500000000).min_by_key(distance_sq)
L1115 has_near_enemy_champion = player_champion[1-team].iter().flatten().any(|c| c.distance_sq(champ) < 15000000000 && bb[1-team].is_recent_visible(game,player,c))
L1119 tower_score = 0
      if let Some(tower) = near_ally_tower {
L1121   tower_atk = tower.attack_effect.as_ref().unwrap()
        if has_near_enemy_champion && tower_atk.is_in_range(tower, t)
L1123      && tick < setting.tower_attack_disable_tick {
L1127     tower_score = min(tower_atk.expected_damage_target(ctx, tower, t) * 100 / t.hp, 100)   (udiv · t.hp==0 → panic)
        } }
L1131 move_score = 0
      if effect.ty.expected_move_on_hit(vt+0x88) || effect.ty.expected_rush_effect(vt+0x80) {
L1134   if champ.distance_sq(t) < 1225000001 { move_score = 0 }
L1137   else { ps = position_score_at_position(ff5ec0)(version, player, data, cell_center(t.x), cell_center(t.y), AttackStance)   // 셀 중심(0.5.8 부터)
L1140          move_score = -(hp_value * ps.risk) / 100 }
      }
L1149 in_lapse = player.+0x49c==1 && lapse_core(17d68b0)(seed, player.id(+0x9f8), tick, tps, min(+0x208,100), min(+0x210,100), +0x478, 1000 + (min(+0x490,100)²*0xccd)>>16)   // ★0.6.0 8번째 인자 신설
      lapse_proximity_score = |e| (150000 - min(e.distance(champ),150000)) / 1500
// ---- §F 이동 중 사용 가능 · 사거리 내 (L1155~1172) ---- 그대로
L1155 if action.can_use_with_move(vt+0xb8) && effect.is_in_range(champ, t) {
L1156   if parameter.near_enemies.iter().any(|p| p.id == t.id) && in_lapse {
L1157     return 100 + lapse_proximity_score(t) }
L1159   if let Some(tp) = parameter.near_enemies.iter().find(|p| p.id == t.id) {
L1160     base_damage = tp.risk_damage(+0x80)
L1161     expected_damage = effect.expected_damage_target(ctx, champ, t)
L1162     possible_damage = tp.risk_possible_tower(+0x98) + tp.possible_risk(data, action.duration()+30)
L1163     hp_value = champion_hp_value(data, parameter, tp)
L1166     total_damage = expected_damage + base_damage/2 + possible_damage/4
L1167     if t.stat_buff_cached.undying { L1168 total_damage = min(max(t.hp-1,0), total_damage) }
L1172     return total_damage * hp_value / t.hp + 100
        } }
// ---- §G 적 챔피언 대상 (L1177~1300) ---- 그대로
sum3 = tower_score + base_score + move_score
L1177 if in_lapse && near_enemies.any(|p| p.id==t.id) { total = lapse_proximity_score(t) ; goto RET }
L1179 if let Some(tp) = near_enemies.find(|p| p.id==t.id) {
L1180   base_damage = tp.risk_damage ; L1181 expected_damage = effect.expected_damage_target(ctx,champ,t)
L1182   possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1183   hp_value = champion_hp_value(data,parameter,tp)
L1186   total_damage = expected_damage + base_damage/2 + possible_damage/4
L1187   if t.undying { L1188 total_damage = min(max(t.hp-1,0), total_damage) }
L1192   target_hp = t.hp ; score = total_damage * hp_value / target_hp
L1195   if t.is_champion() {
L1198     if expected_damage >= target_hp               { L1200 score += min(hp_value,100) }
L1201     else if expected_damage + base_damage/2 >= target_hp { L1203 score += min(hp_value,100)*3/4 }
L1206     else { auto_damage = champ.attack_effect.map(|a| a.expected_damage_target(ctx,champ,t)).unwrap_or(0)
L1210            if auto_damage + expected_damage >= target_hp { L1211 score += min(hp_value,100)/2 } } }
L1218   if let Some(cc_duration) = effective_action_cc_time(version, action, effect) {
L1219     if let Some(target_player) = cache.player_by_champion_id(d72be0)(t.id) {
L1222       target_pos = target_player.info.position ; L1223 tps = max(setting.tick_per_second,1)
L1226       my_attack_dps = cache.player_champion_cache[team][my_position].attack_per_sec[target_pos]   // ★0.6.0 per-champ 0x350 stride · *_sec 열 +0x30 시프트
L1231       ally_dps = 0; for ap in 0..5 { if ap == my_position {continue}
L1233         if let Some(ally) = cache.player_champion[team][ap] {
L1234           if ally.distance_sq(champ) < 22500000001 { c = cache.player_champion_cache[team][ap];
L1235~1241        ally_dps += c.attack_per_sec[target_pos] + c.skill_per_sec[target_pos] + c.skill2_per_sec[target_pos] + c.ult_per_sec[target_pos] } } }
L1248       follow_up_damage = (ally_dps + my_attack_dps) * cc_duration / tps
L1249       cc_score = follow_up_damage * hp_value / target_hp
L1251       score += min(cc_score, 80)
L1258       nuke_during_cc = 0
            if champ.can_attack() || champ.attack_cooldown() <= cc_duration {
L1259~1260    if let Some(atk) = champ.attack_effect { nuke_during_cc = atk.expected_damage_target(ctx,champ,t) } }
L1264       if champ.can_skill() || champ.skill_cooldown() <= cc_duration {
L1265~1267    if let Some(s) = champ.skill_effect { if s.target.check(165e7a0)(champ,t) { nuke += s.expected_damage_target(ctx,champ,t) } } }
L1271       if champ.can_skill2() || champ.skill2_cooldown() <= cc_duration { s = level>2 ? skill2 : None; (동일) }
L1278       if champ.can_ult() || champ.ult_cooldown() <= cc_duration { s = level>4 ? ult : None; (동일) }
L1286       combined = nuke_during_cc + expected_damage
L1287       if combined >= target_hp { L1289 score += min(hp_value,80) }
L1290       else if target_hp > 0 && combined*100/target_hp > 59 { L1292 score += min(hp_value,80)/3 }
          } }
L1298   total = score + v55_mark_value(e82410)(version,effect,data,player,parameter,champ,t,hp_value,_debug)
L1299         + v55_seal_value(e82990)(version,effect,data,champ,t,hp_value)
L1300         - v55_banish_penalty(e83000)(version,effect,data,player,champ,t,hp_value)
        goto RET }
// ---- §H 아군 챔피언 대상 (L1302~1423) ----
L1302 else if let Some(tp) = parameter.near_allies(+0x14b8/+0x14d0).iter().find(|p| p.id==t.id) {
L1303   expected_heal = effect.expected_heal_target(vt+0x58)(ctx, champ, t)
L1304   expected_shield = effect.expected_shield_target(vt+0x60)(ctx, champ, t)
L1306   v55_tbuff: Option<BuffState> = v55_target_dependent_buff(e85820)(effect,data,t)
L1307   has_buff = v55_tbuff.is_some() || effect_buff_target(vt+0xc0)(version,effect,ctx,champ,t).is_some()
L1309   v55_on_attack = effect.ty.expected_on_attack_damage(vt+0xd0)(ctx, champ)
L1310   etc_buff = if has_buff || v55_on_attack > 0 || !effect.ty.etc_buff(vt+0xb0) { 0 }
L1312             else if effect.ty.expected_mark(vt+0xc8)(ctx, champ).is_none() { 5 } else { 0 }
L1320   hp_value = champion_hp_value(data,parameter,tp)
L1322   possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1323   applyed_damage = tp.applyed_damage(+0x70) ; L1325 epic_tank = tp.risk_epic_damage(+0x88)
        // ★0.6.0 v3 조건 변경 (0.5.8: possible_damage>0 || epic_tank!=0 || applyed_damage>0)
        enemy_targets_t = (0..5).any(|i| { a = &bb[1-team].small_actions[i]; (6..=9).contains(a.tag) && a.target_id == t.id })   // 인라인(§H) · tag 9 = Ult(추정)
L1328   expected_shield = if applyed_damage > 0 || epic_tank != 0 || enemy_targets_t { expected_shield } else { 0 }   // ★0.6.0 possible 항 삭제 · ★v2 사장: 구 조건
L1338   incoming = applyed_damage + possible_damage + epic_tank ; epic_incoming = epic_tank
L1339   missing = max(t.stat_cached.hp - t.hp, 0)
L1340   expected_heal = min(missing + incoming*2, expected_heal) ; expected_shield = min(incoming*3, expected_shield)
L1347   if has_buff || v55_on_attack > 0 {
L1348     bs = v55_tbuff.clone().or_else(|| effect_buff_target(version,effect,ctx,champ,t))
          if let Some(bs) = bs {
L1350       crisis_needed = bs.cc_immune || bs.undying || bs.toughness == 0
L1363       cast_range = effect.range + (champ.level-1)*growth + champ.stat_buff_cached.range + effect.range_adjust(champ,t) + champ.radius() + t.radius()   // ★0.6.0 L1363~1364 를 앞으로 끌어옴
L1364       walk_ticks = champ.distance(t).saturating_sub(cast_range) / max(champ.stat_cached.move_speed(+0x640),1)
L1351~1354  crisis: Option<(bool,bool)> = if crisis_needed { Some(defensive_crisis(e82c00)(version,_rnd,player,data,t, walk_ticks, _debug)) } else { None }   // ★0.6.0 6번째 인자 walk_ticks(v3 에서 유효)
L1360       v = buff_value_v54(e80960)(version, &bs, t, data, player, parameter, crisis.as_ref(), incoming, epic_incoming, v55_on_attack, hp_value, champ_incoming)   // ★0.6.0 12인자(+version, +champ_incoming · spec §A ABI)
L1365       delay_sec = walk_ticks / tps   (sdiv · tps==0 → panic)
L1366       v = clamp(6 - delay_sec, 0, 6) * v / 6
L1368       v_currency = area_buff_multi_scale(e832b0)(version,effect,data,player,t,v)
          } else if v55_on_attack > 0 {     // L1369
L1373       v = buff_value_v54(version, &BuffState::default(), t, data, player, parameter, None, incoming, epic_incoming, v55_on_attack, hp_value, champ_incoming)
L1375       v_currency = area_buff_multi_scale(version,effect,data,player,t,v)
          } else { v_currency = 0 }
L1378     if has_buff && v_currency == 0 {
L1380       bs2 = v55_tbuff.clone().or_else(|| effect_buff_target(..)) ; w = noncombat_steroid_window(e84340)(player,data,champ,t)
            if bs2.is_none() || w.is_none() { L1392 expected_buff_score = 0 }
            else { L1383 v = noncombat_steroid_value(data, &bs2, t, &w) ; L1384 v = area_buff_multi_scale(version,effect,data,player,t,v)
L1386              if v > 0 && ctx.debug && tick % 6 == 0 { L1387 add_log("STEROID_WIN T{team} {pos:?} ally_target v={v} fights_back={w.fights_back}") }
L1390              expected_buff_score = v }
          } else { L1395 expected_buff_score = v_currency }
        } else { L1398 expected_buff_score = 0 }
L1401   score = (expected_heal + expected_shield) * hp_value / t.hp   (sdiv · 0 → panic)
L1403   if expected_heal + expected_shield > 0 && score == 0 {
L1404     score = if applyed_damage > 0 || possible_damage > 0 || tp.risk_damage != 0 || t.hp < t.stat_cached.hp { 1 } else { 0 } }
L1411   aoe_ally_value = v54_aoe_ally_heal_value(e83ba0)(version,effect,data,parameter,champ,t,t.id,action)
L1414   if has_buff && ctx.debug { L1415~1416 add_log("BUFFSCORE nm={action.action_name()} T{team} {pos:?} ally t={t.id} buff={expected_buff_score} hs={score} etc={etc_buff} aoe={aoe_ally_value}") }
        penalty = if version < 3 { 0 } else { effect.ty.vt70(ctx, champ, t) * hp_value / max(t.hp, 1) }   // ★0.6.0 v3 신규 항(vt+0x70 · 대상 역효과 추정)
L1419   total = expected_buff_score + score + etc_buff + aoe_ally_value - penalty                              // ★0.6.0
L1423   if total == 0 { total = if has_buff || expected_heal+expected_shield > 0 { -10 } else { 0 } }
        goto RET }
// ---- §I 자기 자신 대상 (L1429~1550) ----
L1429 else if t.id == champ.id {           // tp = parameter.player(+0x918)
L1430~1431 expected_heal/shield = effect.expected_heal_target/expected_shield_target(ctx,champ,t)
L1434~1435 v55_tbuff = v55_target_dependent_buff(effect,data,t) ; has_buff = v55_tbuff.is_some() || effect_buff_target(version,effect,ctx,champ,t).is_some()
L1437   v55_on_attack = effect.ty.expected_on_attack_damage(ctx,champ)
L1438~1440 etc_buff = (§H L1310~1312 와 동일)
L1450   epic_tank = parameter.player.risk_epic_damage(+0x9a0)
        enemy_targets_me = f5d740(&bb[1-team], champ.id)   // ★0.6.0 신규 콜리(§I 는 아웃라인): 적 소액션 tag 6..=9 가 champ 조준
L1453   expected_shield = if parameter.player.applyed_damage(+0x988) > 0 || epic_tank != 0 || enemy_targets_me { expected_shield } else { 0 }   // ★0.6.0 v3(possible 항 삭제) · ★v2 사장: `possible(§B)<1 && (epic|applyed)==0 → 0`
L1462   incoming = possible_damage(§B 값) + applyed_damage + epic_tank ; epic_incoming = epic_tank
L1463   missing = max(champ.stat_cached.hp - champ.hp, 0)
L1464   expected_heal = min(missing + incoming*2, expected_heal) ; expected_shield = min(incoming*3, expected_shield)
L1469~1511 expected_buff_score = §H L1347~1395 와 같은 구조(대상 t=champ · hp_value=§D 의 hp_value) — 단 L1363~1366 의 사거리/도달 감쇠가 없다:
            crisis = crisis_needed ? Some(defensive_crisis(version,_rnd,player,data,champ, 0 /*★0.6.0 6번째 인자 0*/,_debug)) : None
            v_currency = area_buff_multi_scale(version,effect,data,player,champ, buff_value_v54(version,&bs,champ,data,player,parameter,crisis.as_ref(),incoming,epic_incoming,v55_on_attack,hp_value,champ_incoming)) (L1482~1484) ; bs None && on_attack>0 → default BuffState (L1489~1491) ; L1494 has_buff && v_currency==0 → L1496 bs2·w=noncombat_steroid_window(player,data,champ,champ) → L1499 noncombat_steroid_value → L1500 area_buff_multi_scale → L1502~1503 로그 "STEROID_WIN ... self v= fights_back=" → L1506 expected_buff_score=v / L1508 0 ; L1511 else v_currency
L1514   (has_buff 아니고 on_attack<=0) expected_buff_score = 0
L1517   score = (expected_heal + expected_shield) * hp_value / champ.hp   (sdiv · 0 → panic)
L1523   ally_aura_protect = 0; for ally in cache.player_champion[team].iter().flatten() {      // 자기 자신 포함 5슬롯
L1524     if effect.ty.expected_ally_aura_heal_at(vt+0x98)(ctx, champ, ally) != 0 {
L1527       if let Some(atp) = near_allies.find(|p| p.id == ally.id) {
L1528         if defensive_crisis(version,_rnd,player,data,ally, 0, _debug).0 {                    // ★0.6.0 6번째 인자 0
L1529           ally_aura_protect += min(champion_hp_value(data,parameter,atp), 80) } } } }
L1537   v55_trigger = v55_spirit_trigger_value(e84f30)(effect,data,player,parameter,champ,hp_value)
L1540   aoe_ally_value = v54_aoe_ally_heal_value(version,effect,data,parameter,champ,champ,champ.id,action)
L1543   if has_buff && ctx.debug { L1544~1545 add_log("BUFFSCORE nm=.. T.. self buff= hs= etc= aura= trig= aoe=") }
        penalty = if version < 3 { 0 } else { effect.ty.vt70(ctx, champ, champ) * hp_value / max(champ.hp, 1) }   // ★0.6.0 v3 신규 항
L1548   total = score + expected_buff_score + etc_buff + ally_aura_protect + v55_trigger + aoe_ally_value - penalty   // ★0.6.0
L1550   if total == 0 { total = if has_buff || expected_heal+expected_shield > 0 { -10 } else { 0 } }
        goto RET }
// ---- §J 그 밖(타워·미니언·근접목록 밖 챔피언) ---- 그대로
L1558 else { if t.is_champion() && ctx.debug && t.team == champ.team { L1559 add_log("BUFFTGT_MISS T{team} {pos:?} t={t.id} near_allies") }
        total = 0 }
// ---- RET (L1564) ----
return sum3 + total          // sum3 = tower_score + base_score + move_score
// 반환 phi 전체: [v57 Some(v)] [§F 100+x] [sum3+total] [-9999999 L1020] [-9999999 L1034]
```

## changes
- §C L1092~1094(v2/v3 공통): `projected = stance.sat_sub(current/2)` → `adj = stance==0 ? 0 : (hp<=stance ‖ dmg%>=50 ‖ (hp%<66 && dmg%>=30) ‖ (hp%<41 && dmg%>=18) ‖ (hp%<26 && dmg%>=10)) ? stance : 0; projected = adj.sat_sub(current/2)` (hp% = hp*100/max(max_hp,1) · dmg% = stance*100/max(hp,1)).
- §E L1149: lapse_core(17d68b0) 8번째 인자 `1000 + (min(player.+0x490,100)²·0xccd)>>16` 신설 · 게이트 `+0x49c==1`.
- §H L1328 / §I L1453(v3): expected_shield 유지 조건 `possible>0 ‖ applyed>0 ‖ epic≠0` → `applyed>0 ‖ epic≠0 ‖ 적 소액션(tag 6..=9) 이 t 조준`(possible 항 삭제 · §I 는 f5d740 콜리 · §H 인라인).
- §H L1351: defensive_crisis 6번째 인자 = walk_ticks(L1363~1364 를 앞으로 이동) · §I L1482/L1528: 0.
- §H L1419 / §I L1548(v3): `total -= effect.ty.vt[0x70](ctx,champ,t) * hp_value / max(t.hp,1)` 패널티 항 추가.
- buff_value_v54 12인자(+version, +champ_incoming) · check_kill_die_tick ABI(bool×3=false · &None).
- §A/§B/§D/§F/§G/§J 그대로(db0160 3-bool 캐시 동구조). move_score 의 position_score 셀 중심은 0.5.8 부터.
- ★v2 사장: L1328/L1453 구 조건 · 꼬리 패널티 없음(version<3).

## verified
- capstone 0.6.0 f77820: §C 임계 블록 f7ad95~f7ae12(`cmp rcx,r9 jae` hp<=stance · dmg%>0x31 · hp%>0x41→skip/dmg%>0x1d · hp%>0x28→skip/dmg%>0x11 · hp%>0x19→0/dmg%>=0xa · `shr rbx,1; sub; cmovae` = sat(adj−current/2)) · 1007030 ×2(f78abe/f78ad9) · lapse 8번째 인자(f799e0~f79a16: 0xccd · >>0x10 · +0x3e8 · [rsp+0x38]) · f5d740 호출(f7ac83 §I) 직후 `rbx|rsi`(applyed|epic) 합성 · vt+0x70 호출 2곳(f7c0d4 §H · f7c519 §I) · e82c00 ×3(f7aa62/f7aebb/f7c2c5) · e72970/e0f1d0/eda920 §A 위치 · db0160 ×2 · e85460 · ff5ec0 · ff1fc0(+0x1e).
- 미확인: §H 의 enemy_targets_t 인라인 본문(RE 「인라인」 판정 채택 · 이번엔 f5d740 콜 1곳만 확인) · tag 9 = Ult 명칭 · vt+0x70 의미(패널티 산식은 확정) · defensive_crisis 6번째 인자가 v3 내부에서 어떻게 쓰이는지(ABI 만).

## confidence
A(§C·§E·§I 꼬리·f5d740) / B(§H expected_shield 인라인·vt+0x70 의미) — 변경 5지점 중 4지점 asm 직접 확인, 나머지 RE 원문(88KB 덤프 정독) 의존.
