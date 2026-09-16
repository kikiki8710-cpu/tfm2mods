# r21 티어1 심층 — 웨이브 1 (2) · 0.5.8 명세 logic 전문 + exe 힌트

요구: 0.5.8 명세(아래 logic) 를 기준선으로 0.6.0 디컴을 **줄 단위로 대조**해 「어느 줄이 어떻게 바뀌었나 / 새 분기·조건·상수·필드·콜리」 를 명세 형식(줄번호 참조 · 등가 Rust)으로 낸다. version≥3 분기는 「v3 경로」 로 별도 표기(값 미확정 · v2/v3 둘 다 적는다). 대형 함수는 먼저 콜리 지도·블록 지도(0.5.8 ↔ 0.6.0 매핑)를 만들고 신규 구역을 특정한 뒤 신규 구역만 정독한다.

## `eba9b0` → `edb240` fight_check::check_kill_die_tick_uncached  (4429→11569B · Δ+7140)
- 힌트: fight_check::check_kill_die_tick_uncached(4.4→11.6KB) — 신 ABI(rnd·debug 제거 · bool×3 · &Option<(u64,u64,u64)>)의 본체 · TLS 캐시 래퍼 eda920 뒤
- exe 정렬: 명령 1038→2524 · 정렬 914 · 잔여 구조 7 · 분기 10 · 콜리 주의: 12857f0→1643790 변경 ; 128cc90→165d410 미지 ; 128cf70→164b370 미지 ; d31bb0→d72be0 미지
- 0.5.8 명세 #211 `fight_check__check_kill_die_tick_uncached` src game-ai\src\fight_check.rs:970 · one_line: focus 챔피언이 지금 교전을 시작하면 몇 틱 뒤에 죽는가(die tick) — 적 챔피언·타워·기타 유닛의 nuke(선제 버스트)와 dps 를 판단력 노이즈 곱으로 합산해 (hp+부활보너스−nuke)*60/dps
- 0.5.8 params: version:usize (i64 %0)(IR 속성 없음(값 인자) · 본문 분기 없음. enemy_minion_wave_risk_dps_at 의 1) · _rnd:&mut StdRng(★IR 시그니처에서 제거됨(미사용 인자, DI !42125 만 남음) · ★IR define 인자에서 제거됨) · data:&OperationData (24B, ptr %1)(IR 속성 `readonly captures(address, read_provenance)` · cache() · judger:&PlayerState (2528B, ptr %2)(IR 속성 `readonly captures(address, read_provenance)` · 판정 주체 ) · focus:&Entity (1728B, ptr %3)(IR 속성 `readonly captures(address, read_provenance)` · 죽음 판정 ) · enemy:bumpalo Vec<&Entity> (32B by(IR 속성 `dead_on_return noalias captures(address)` — 소유권 이전, 함) · towers:bumpalo Vec<&Entity> (32B by(IR 속성 `dead_on_return noalias captures(address)` — 소유권 이전, 함) · _debug:&mut DebugFrameData(★IR 시그니처에서 제거됨(미사용 인자, DI !42131 만 남음) · ★IR define 인자에서 제거됨)
- 0.5.8 logic 전문:
```
fn check_kill_die_tick_uncached(version, _rnd, data, judger, focus, enemy: Vec<&Entity>, towers: Vec<&Entity>, _debug) -> usize   // fight_check.rs:970
 if focus.stat_buff_cached.undying { return i64::MAX }                                   L976
 player = cache.player_by_champion_id(focus.id).unwrap()   // focus 의 PlayerState        L979
 tps_raw = setting.tick_per_second ; tps = max(tps_raw, 1)                                L986
 bucket = game.tick() / (tps*2)   // 2초 버킷 (tps*2==0 이면 div_by_zero 패닉 경로)        L987
 rng = NoiseRng( (judger.info.id * 0x9E3779B97F4A7C15) ^ (focus.id << 24) ^ bucket )      L989~990
 ja = judge_accuracy(&judger.info.parameter)   // 100..=1000                               L994
 d = (1000 - ja) >> 1 ; noise() := rng.range_usize(1000 - d, 1000 + d)   // [lo,hi] 양끝 포함 · splitmix64 → hi64(z * (hi-lo+1)) + lo   L995
 enemy_dps = 0 ; enemy_nuke = 0                                                            L998
 cc(p) := cache.player_champion_cache[p.team][p.pos] ; fpos := player.pos
 for pchamp in enemy {                                                                     L1000
   p = cache.player_by_champion_id(pchamp.id).unwrap()                                     L1002
   nuke = 0
   if pchamp.can_attack() || pchamp.attack_cooldown() <= tps_raw { nuke = cc(p).attack[fpos] * noise() / 1000 }               L1005~1007  ←rng#1
   if pchamp.can_skill()  || pchamp.skill_cooldown()  <= tps_raw { nuke = max(nuke, cc(p).skill[fpos]  * noise() / 1000) }   L1010~1012  ←rng#2
   if pchamp.can_skill2() || pchamp.skill2_cooldown() <= tps_raw { nuke = max(nuke, cc(p).skill2[fpos] * noise() / 1000) }   L1015~1017  ←rng#3
   if pchamp.can_ult()    || pchamp.ult_cooldown()    <= tps_raw { nuke = max(nuke, cc(p).ult[fpos]    * noise() / 1000) }   L1021~1023  ←rng#4
     // *_cooldown() 은 Champion(ty 13) 이면 필드, 아니면 0 → 항상 통과
   if !pchamp.is_block_attack() { enemy_dps += cc(p).attack_per_sec[fpos] * noise() / 1000 }                                   L1026~1028  ←rng#5
   if !pchamp.is_block_skill() && !(pchamp.is_block_move_skill() && pchamp.skill_effect.map_or(false, |e| e.can_move()))
        { enemy_dps += cc(p).skill_per_sec[fpos] * noise() / 1000 }                                                               L1031~1033  ←rng#6
   if !pchamp.is_block_skill() && !(pchamp.is_block_move_skill() && pchamp.skill2_effect().map_or(false, |e| e.can_move()))
        { enemy_dps += cc(p).skill2_per_sec[fpos] * noise() / 1000 }      // skill2_effect() = level>2 ? Some(..) : None    L1036~1038  ←rng#7
   enemy_nuke += nuke                                                                      L1040
 }   // ult_per_sec 는 합산하지 않음
 for tower in towers {                                                                     L1049
   damage = tower.attack_effect.unwrap().expected_damage_target(ctx, tower, focus)          L1050
   enemy_dps  += (noise() * tps_raw * damage / 1000) / tower.attack_cooltime()   // cooltime 0 이면 div_by_zero 패닉   L1051~1053 ←rng#8
   enemy_nuke += damage * noise() / 1000                                                    L1054  ←rng#9
 }
 for e in cache.others[1 - player.team] {                                                  L1058
   if dist²(e, focus) > 22500000000 { continue }          // > 150,000                     L1059
   if !game.is_visible(judger.team, e.id) { continue }                                     L1063
   if let Some(atk) = e.attack_effect {                                                    L1066
     damage = atk.expected_damage_target(ctx, e, focus)                                    L1067
     enemy_dps  += damage * tps_raw / max(e.attack_cooltime(), 1)                          L1068
     enemy_nuke += damage                                   // 노이즈 없음                 L1069
   }
 }
 for pchamp in enemy { for b in pchamp.effect_buffs { enemy_dps += b.expected_aura_dps_at(ctx, pchamp, focus) } }          L1075~1077
 ally_heal = 0 ; for ally in cache.iter_champions(player.team) { for b in ally.effect_buffs { ally_heal += b.expected_aura_heal_at(ctx, ally, focus) } }   L1084~1087
 enemy_dps = enemy_dps.saturating_sub(ally_heal)                                           L1090
 enemy_epic_buff := (game.get_game_mode() is Moba(m)) && m.epic_minion_buff_time[1 - player.team] != 0   // IR %653 은 이 값의 부정(dbg_value 에 DW_OP_not) — 소스 변수명·극성 일치   L1095
 low_enough_to_care_minions := focus.hp*100 <= max(focus.stat_cached.hp,1)*75   // IR %661 은 이 값의 부정(dbg_value DW_OP_not) · 오라클 C31(751/1000 → 제외)·C32(750/1000 → 포함) 경계   L1096
 line_phase := if tutorial ∈ {None,MidBottom,Line,Total} { game.tick() < setting.epic_jungle.first_spawn_tick.saturating_sub(tps_raw*30) } else { true }   L1097  (오라클 C15 fst=1900·tick=100 → 100<100 거짓 / C16 fst=1901 → 참 경계 실측)
 if enemy_epic_buff || (low_enough_to_care_minions && !line_phase) { enemy_dps += enemy_minion_wave_risk_dps_at(version, data, focus, focus.x, focus.y) }   L1098~1099  (IR: %653 참 && (%661 || %673) 이면 건너뜀 — 동치 · 오라클 C13/C14/C17/C18/C19/C33/C34 로 4분기 전부 실행 확인)
 revive_hp = Σ focus.effect_buffs.revive_bonus_hp(ctx, focus)                              L1104
 return ((revive_hp + focus.hp).saturating_sub(enemy_nuke) * 60) / max(enemy_dps, 1)       L1107
 // 드롭: enemy·towers Vec (bumpalo, L1110)
 // NoiseRng 추첨 순서(상태는 한 스트림): 적 챔피언마다 최대 7회(attack·skill·skill2·ult·attack_ps·skill_ps·skill2_ps, 각 조건부) → 타워마다 2회(dps, nuke) → others/버프 0회. 총 사이트 9곳. StdRng(_rnd) gen_range 0회.
```

## `e6b800` → `d72e70` check_kill  (5680→8014B · Δ+2334)
- 힌트: check_kill(5.7→8KB · d72e70 = 다이브/킬 창 평가로 이미 부분 규명: 적 처치 시간 vs 내 사망 시간 · agent_ctx +0x49c/+0x49d/+0x49e 임계 · 호출자 LPH::update·handle_interact_battle·ganker)
- exe 정렬: 명령 1219→1693 · 정렬 988 · 잔여 구조 4 · 분기 10 · 콜리 주의: 12857f0→1643790 변경 ; 128cc90→164b370 미지 ; 16047b0→17d68b0 미지(+40B) ; d31bb0→d72be0 미지
- 0.5.8 명세 #87 `handler__check_kill` src game-ai\src\plan_legacy\handler.rs:2507 · one_line: 150000 안 적 중 「내가 (아군 화력으로) 먼저 죽이고, 그 적이 타워로 도망치기 전에 잡을 수 있는」 첫 적을 골라 (적 id, 예상 처치 틱)을 돌려준다
- 0.5.8 params: (sret):Option<(usize, usize)> (24B)(+0 태그(0=None/1=Some), +8 = 적 Entity.id, +16 = max(적 사망예상틱, 6) · version:usize(본문 분기 없음. 적 필터 클로저(aux) 의 is_ignored_well_enemy 에 전달) · rnd:&mut StdRng(readnone — 전혀 안 씀) · player:&PlayerState (2528B)(+0x930 team, +0x9c0 position, +0x180 parameter(aggressive_ra) · data:&OperationData (24B)(+0x0 cache, +0x8 context) · _positioning_score:&PositioningScoreData(readonly·미사용(이름부터 _)) · debug:&mut DebugFrameData (224B)(ctx.debug 일 때만 +0xa0 infos[me.id] 에 문자열 push (L2473~2474))
- 0.5.8 logic 전문:
```
check_kill(version, rnd, player, data, _ps, debug):                 // L2507
  if player_awareness_lapse(player, data) { return None }            // L2509~2510
  return check_kill_v15(version, player, data, debug)                // L2512 (2373~2505 전부 인라인)

--- check_kill_v15 ---
me   = cache.player_champion[team][my_pos].unwrap()                            // L2374
allies  = iter_champions(team).filter(|a| dist_sq(a,me) <= 150000² && a.hp*100/a.max_hp > 39)   // L2377 (aux#0) ★나 자신 포함
enemies = iter_champions(1-team).filter(|e| dist_sq(e,me) <= 150000²
            && (me.team==Neutral || e.visible_state[my_team]==Visible)
            && !is_ignored_well_enemy(version, player, e))                       // L2379 (aux#1)
disable_tick = match player_count(tutorial) { 5v5→setting.tower_attack_disable_tick, 2v2→_2v2, 3v3→_3v3 }   // L2383~2386
towers = iter_towers_without_nexus(cache, 1-team).filter(|t| game.tick() <= disable_tick)   // L2389 (aux#2) — 적 타워

// 1) 적 전원이 나에게 주는 화력
enemy_burst = 0; enemy_dps = 0
for e in enemies:                                                              // L2393
  pe = player_by_champion_id(e.id).unwrap(); cc = champion_cache[pe.team][pe.pos]   // L2395
  b = 0
  if e.can_attack() || e.attack_cooldown() is None || cd <= tps { b = cc.attack[my_pos] }         // L2398~2399
  if e.can_skill()  || e.ty!=Champion || e.skill_cooldown  <= tps { b = max(b, cc.skill[my_pos]) } // L2402~2403
  if e.can_skill2() || e.ty!=Champion || e.skill2_cooldown <= tps { b = max(b, cc.skill2[my_pos]) }// L2406~2407
  enemy_dps  += cc.attack_per_sec[my_pos] + cc.skill_per_sec[my_pos] + cc.skill2_per_sec[my_pos]  // L2410~2412
  enemy_burst += b                                                             // L2413

ar = player.parameter.aggressive_ratio()                                       // L2417
c1 = (1000-ar)*80/1000 + 80 ; c2 = ar*45/1000 + 45 ; c3 = ar*35/1000 + 15     // L2419~2421

// 2) 적 하나씩 — 아군 전원의 화력 vs 그 적
for e in enemies:                                                              // L2424
  pe = player_by_champion_id(e.id).unwrap(); epos = pe.pos                     // L2431~2432
  ally_burst = 0; ally_dps = 0
  for a in allies:                                                             // L2434
    pa = (team,pos) of a via player_champion 10칸 id 스캔 (없으면 unwrap 패닉)   // L2435
    cc = champion_cache[pa.team][pa.pos]                                       // L2440
    ally_dps += cc.attack_per_sec[epos] + cc.skill_per_sec[epos] + cc.skill2_per_sec[epos]   // L2440~2442
    b = 0 ; (공격/스킬/스킬2 가용 판정은 1)과 동일 규칙, 대상 열 = epos)            // L2444~2453
    ally_burst += b                                                            // L2456~2457
  my_burst = enemy_burst ; my_dps = enemy_dps
  for t in towers:                                                             // L2460
    if dist_sq(t, e) <= 150000² {                                              // L2461
      d = t.attack_effect.unwrap().expected_damage_target(ctx, t, <static>, me)   // L2462
      my_dps  += tps * d / t.attack_cooltime()   (0 이면 div-by-zero 패닉)        // L2463
      my_burst += d }                                                          // L2464
  enemy_die = sat(e.hp - ally_burst) * 60 / max(ally_dps, 1)                   // L2470
  my_die    = sat(me.hp - my_burst)  * 60 / max(my_dps, 1)                     // L2471
  if ctx.debug { debug.infos[me.id].push(format!(enemy_die, my_die)) }         // L2473~2474
  if enemy_die > sat(my_die - c1) { continue }                                 // L2477  ★내가 c1 틱 이상 먼저 죽이지 못하면 탈락
  tower_ticks = match towers.min_by_key(|t| dist_sq(t,e)) { Some(t) => e.distance(t) / e.move_speed, None => 99999999 }   // L2481~2484
  if me.move_speed > e.move_speed {                                            // L2489
    ok = enemy_die < sat(tower_ticks - c3)                                     // L2498~2499
  } else {
    ok = me.attack_effect.unwrap().is_in_range(me, e) && enemy_die < c2 && enemy_die < sat(tower_ticks - c3)   // L2491~2494
  }
  if ok { return Some((e.id, max(enemy_die, 60) + game.tick())) }              // L2502
return None                                                                    // L2505
```
