---

### `180` position_eval_at_uncached — 셀 (x,y) 에 내 챔피언이 서 있을 때의 위험/이득 56B PositioningScore 를 계산 — 벽/적 우물=9999 즉시, 정글·에픽 몹 기대피해(HP% ×2/1/½/⅓ 가중)·타워(사거리·미니언 수)·미니언 웨이브·투사체 궤도·적 챔피언 위협(가시/비가시)·아군 교전 이득·purpose 별 보정을 순서대로 누적. POS_EVAL_CACHE miss 시에만 호출자 position_eval_at 이 부른다.

| 항목 | 값 |
|---|---|
| id | `position_eval__position_eval_at_uncached` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai13position_eval25position_eval_at_uncached` |
| 소스 | `game-ai\src\position_eval.rs:371` |
| IR | `m07.ll` 24749~34229행 |
| 경로·가시성 | `game_ai::position_eval::position_eval_at_uncached` · **in:game_ai::position_eval** |
| 계층 | 기타 |
| exe | `d851d0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r15` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) score | &mut PositioningScore(56B) | sret · writeonly · dereferenceable(56). DI 이름 `score`(m07.ll:24894 `;; score = ptr %0`). 본문에서는 SSA(dbg_value `score[0..+8]` 등)로 누적하다 끝(배치 D, m07.ll:31996~32010)에서 8필드 store; 조기반환 2곳은 배치 A 가 직접 memset(375: 50B 전부 0 / 381: risk=9999 + 8..0x32 를 0). 바이트 0x32~0x37(6B 패딩)은 어느 경로에서도 안 쓴다 \| (배치 B) risk@0x0 i64 · tower_risk@0x8 · gain@0x10 · gain_me@0x18 · adjust@0x20 · unseen_champ_threat@0x28 · on_trajectory@0x30 bool · on_periodic_trajectory@0x31 bool (tcxdict game_core::PositioningScore, ai_interface.rs:368 — 승계 표에 없던 8번째 필드 on_periodic_trajectory@0x31 추가). 배치 B 범위에서 값이 바뀌는 워드 = risk·tower_risk·gain (SSA 캐리, 최종 store 는 배치 D) · on_trajectory/on_periodic_trajectory 는 L638 에서 0 초기화만 \| (배치 C) sret. 이 배치 범위 안에는 %0 에 대한 store 가 0건 — 모든 score 필드는 SSA phi 로 운반되고(DI `score[N..+8]`) 저장은 다른 배치(끝부분) 소관 \| (배치 D) sret %0. 배치 D 가 최종 기록자 — position_eval.rs:1142 에 인라인된 apply_position_eval_purpose(29~44) 의 44 줄에서 8필드 전부 store(m07.ll:31996~32010), 1176 에서 risk(+0) 를 노이즈 적용값으로 재기록(32157). 조기반환 경로(381)는 배치 A. | 3 |
| 1 | 1 | version | i64 (usize) | AI 버전. 배치 A 범위에서는 분기 없음 — is_enemy_well_danger(397)·pe_player_ctx(442)·entity_positioning_cache_cached(420/438) 에 그대로 전달만 한다 \| (배치 B) 배치 B 범위 내 직접 분기 없음. L536 v47_siege_stance 와 L574 enemy_minion_wave_risk_damage_at 에 전달(후자는 `i64 poison` 으로 전달 = 콜리가 version 을 안 읽음) \| (배치 C) DI !39827. 이 범위에선 fight_check::slot_cc_time_cached(version, …) 의 첫 인자로만 전달, 자체 분기 없음 \| (배치 D) 배치 D 분기: 1152/1153 `version > 1`(`icmp ugt i64 %1, 1` 32017) — v2+ 는 purpose 가 RunAway/Recall 이면 노이즈 면제, 1165 `version > 1` 이면 t_salt = tick/(tps*6) 6초 버킷, 아니면 tick 그대로(32087). effect_type_cc_time(990)·get_battle_role(1105) 에 그대로 전달. | 4 |
| 2 | 2 | player | &PlayerState(2528B) | readonly · noalias. 읽는 필드 = info.id@0x928(호출자 캐시키 · pe_player_ctx 키) · info.team@0x930 · info.position@0x9c0(tag i32, as_index) \| (배치 B) 배치 B 에서는 L536 v47_siege_stance 인자로만 전달 \| (배치 C) DI !39828. 이 범위에서 직접 load 없음(팀 %112=player+0x930 은 배치 A 가 로드한 값을 사용) \| (배치 D) %2. 배치 D 읽기: +0x180 info.parameter → AthleteParameter::positioning_accuracy(1148) · +0x928 info.id → 노이즈 해시 시드(1170). | 4 |
| 3 | 3 | data | &OperationData(24B) | readonly. cache@0(&AbstractGameWithCache) · context@8(&GameContext) · blackboard@0x10(&[Blackboard;2]) \| (배치 B) +0x0 cache(&AbstractGameWithCache) = %118 · +0x8 context(&GameContext) = %135 (배치 A 정의, 재확인) \| (배치 C) DI !39829. +0x8 context → GameContext+0x8 setting → GameSetting+0x1490 well_damage 를 이 범위에서 읽음. slot_ready_cached/slot_cc_time_cached 에 그대로 전달 \| (배치 D) %3. +0 cache(&AbstractGameWithCache · %118) → +0 game data ptr %151 / +8 vtable %153 (tick 슬롯 0x28 · get_game_mode 슬롯 0x40) · +8 context(&GameContext · %135) → +8 setting → +0x12f8 tick_per_second(1166). get_battle_role 에 cache(%118) 전달(1105). | 4 |
| 4 | 4 | x | i64 (u64 좌표) | DI `x`(m07.ll:24897 · 24892 store → alloca %100 → cell_dist_sq 클로저가 &x 캡처). 셀 = x/32000 clamp 0..29 \| (배치 B) DI 이름 `x`(스택 %100 경유) — 평가 좌표 x (셀=32000 단위) \| (배치 C) DI !39830 `x` — 평가 좌표 X(맵 단위, 셀=32000). 이 범위에선 is_in_well_damage(enemy_team, x, y) 의 인자(배치 A 가 %100 슬롯에 저장한 값을 load) \| (배치 D) 평가 셀 x 좌표(스택 %100 에 저장). 배치 D: dist_to_line_segment 의 점(982·1005·1117), 1172 노이즈 해시에 %130 = x/32000(배치 A 계산, **umin 29 클램프 전 값**). | 4 |
| 5 | 5 | y | i64 (u64 좌표) | DI `y`(24902) → alloca %99. yi = y/32000 clamp 0..29 \| (배치 B) DI 이름 `y`(스택 %99 경유) — 평가 좌표 y \| (배치 C) DI !39831 `y` — 평가 좌표 Y(%99 슬롯) \| (배치 D) 평가 셀 y(스택 %99). 1173 해시에 %132 = y/32000(클램프 전). | 4 |
| 6 | 6 | purpose | i8 = PositionEvalPurpose(1B, range 0..13, 9 제외) | DI `purpose`(24907). 메모리 태그(tcxdict --enum): General=2 RunAway=3 Recall=4 Around=5 Positioning=6 Trace=7 Lane=8 LineStyle(LineStyle)=0(Aggressive)/1(Defensive) LaneSafe=10 Objective=11 AttackStance=12 (9 = 없음 → range 제외). 배치 A 범위 안에서는 분기에 안 쓰임(호출자 캐시키 + 뒤 배치 purpose 보정에서만) \| (배치 B) DI 이름 `purpose`. tcxdict --enum: 니치 인코딩(untagged=LineStyle(LineStyle 1B: Aggressive=0/Defensive=1)) · 메모리태그 General=2 RunAway=3 Recall=4 Around=5 Positioning=6 Trace=7 Lane=8 LaneSafe=10 Objective=11 AttackStance=12 · 9 는 빈칸(IR `assume purpose != 9`). 배치 B 에서 쓰는 곳 = L24(573)·L13(576) 라인 판정: {0,1,8,10} = LineStyle(_)\|Lane\|LaneSafe \| (배치 C) DI !39832 `purpose`(score_parameter.rs:37, 1B 니치 enum: General=2 RunAway=3 Recall=4 Around=5 Positioning=6 Trace=7 Lane=8 LineStyle=암묵(0..1) LaneSafe=10 Objective=11 AttackStance=12). 이 범위(678~960)에서는 읽지 않음 \| (배치 D) 메모리 태그(tcxdict --enum): General=2 RunAway=3 Recall=4 Around=5 Positioning=6 Trace=7 Lane=8 LineStyle=untagged(바이트값 0=Aggressive/1=Defensive) LaneSafe=10 Objective=11 AttackStance=12 (9 는 무효 — `assume purpose != 9` 31948). 1142 apply_position_eval_purpose 인라인: idx=(purpose>1 ? purpose-2 : 7) → 7(LineStyle)·8(LaneSafe) 만 보정. 1152: idx ∉ {1,2}(RunAway·Recall) 가 노이즈 조건. | 3 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// position_eval.rs:0~507 (배치 A)
// 시그니처: (sret score, version, player, data, x, y, purpose) — 호출자 position_eval_at(293~324)가 POS_EVAL_CACHE miss 때만 부름(tls 절).
373: _t = ProfTimer::start(48)   // prof::ENABLED 가 0 이면 Instant::now 생략(phase 48, nanos 슬롯 PHASE_NANOS+384)
374: champ = data.cache.player_champion[player.info.team][player.info.position.as_index()]   // team<2 bounds 패닉 가드
375: if champ is None → return PositioningScore::default()   // memset(%0, 0, 50)
378: xi = clamp(x/32000, 0, 29);  379: yi = clamp(y/32000, 0, 29)
380: if data.context.map.walls[yi][xi] != 0 →
381:     return PositioningScore{ risk: 9999, ..default }   // store 9999 @0 + memset(@8, 0, 42)
384: inv_hp_q32 = 2^32 / max(champ.hp, 1)
390: score = default (SSA: 8필드 전부 0)   // 400: tower_well_risk = 0 (초기)
393: enemy_team = 1 - team
394: visible = game.is_visible_cell(enemy_team, xi, yi)   // vtable+0x100 · 배치 A 범위에선 미사용(뒤 배치 소비)
395: cell_dist_sq = |ex, ey| axis_distance_sq(x, ex).saturating_add(axis_distance_sq(y, ey))   // closure#0, axis_distance_sq(score_parameter.rs:8~9) = abs_diff(a,b) 의 saturating 제곱
397: tower_well_risk = if path_finder::is_enemy_well_danger(version, player, x, y) { 9999 } else { 0 }
398: score.risk = tower_well_risk;  399: score.tower_risk = tower_well_risk
403: champ_cache = &cache.player_champion_cache[team][pos]
405: (enemy_mask, ally_mask) = pe_cand_masks(seed=game.seed(), tick=game.tick(), team, cache, data.blackboard, enemy_team, player)   // PE_CAND_MASKS TLS · bit i = 적 i 가 small_action 있음 && 챔프 존재 && is_recent_visible / 아군 i 가 small_action 있음 && 챔프 존재
406: _t_peu1 = ProfTimer::start(112)
407: near_enemies_with_action = Vec::new_in(ctx.pool)   // 원소 448B = (EntityPositioningCache 424, &PlayerState, &Entity, dist)
408: for i in 0..5 {
409:     if enemy_mask & (1<<i) == 0 { continue }
412:     e = cache.player_champion[enemy_team][i].unwrap()
413:     dist = cell_dist_sq(e.x, e.y)
414:     if dist > 40000000000 (=200000²) { continue }
418:     eplayer = cache.player_state[enemy_team][i].unwrap()
419:     e_cache = &cache.player_champion_cache[eplayer.team][eplayer.pos]   // eplayer.team<2 bounds 가드
420:     epc = entity_positioning_cache_cached(version, data, champ, e, player, eplayer, champ_cache, e_cache, has_near_enemy=false)   // EPC_CACHE[team][pos][eteam][epos][0]
421:     near_enemies_with_action.push((epc, eplayer, e, dist))
     }
424: has_near_enemy = !near_enemies_with_action.is_empty()
425: near_allies_with_action = Vec::new_in(ctx.pool)
426: for i in 0..5 {
427:     if ally_mask & (1<<i) == 0 { continue }
430:     e = cache.player_champion[team][i].unwrap()
431:     dist = cell_dist_sq(e.x, e.y)
432:     if dist > 40000000000 || e.id == champ.id { continue }   // 자기 자신 제외(or 의 두 항 순서는 IR 상 dist 비교가 먼저 계산되나 단락 없음 — 둘 다 순수)
436:     eplayer = cache.player_state[team][i].unwrap()
437:     e_cache = &cache.player_champion_cache[eplayer.team][eplayer.pos]
438:     epc = entity_positioning_cache_cached(version, data, champ, e, player, eplayer, champ_cache, e_cache, has_near_enemy)   // EPC_CACHE[..][has_near_enemy as usize]
439:     near_allies_with_action.push((epc, eplayer, e, dist))
     }
442: pe_ctx = pe_player_ctx(version, player, data, champ)   // PE_PLAYER_CTX TLS (seed,tick,player.id 키) → PePlayerCtx 40B
443: skill_linear_move = pe_ctx.skill_linear_move(+0x21);  444: skill2_linear_move(+0x22);  445: ult_linear_move(+0x23)   // 배치 A 범위에선 미사용(뒤 배치 소비)
447: drop(_t_peu1)   // phase 112 nanos/calls 누적(ENABLED 일 때만)
448: _t_peu2 = ProfTimer::start(113)
449: for j in cache.jungles.iter() {   // Vec<&Entity> @0xd0/0xe8
450:     jd = cell_dist_sq(j.x, j.y)
451:     if jd > 22499999999 { continue }   // 150000² 이상 제외
454:     match j.ty.tag {
456:       Jungle(4) => if info.focused == Some(champ.id) {   // +0x88 tag==1 && +0x90 == champ.id
457:            damage = expected_attack_damage_cached(attacker=j, target=champ)   // ATTACK_DMG_CACHE[(j.id, champ.id)]
458:            ratio = min((inv_hp_q32*100*damage) >> 32, 150)   // = min(100*damage/hp, 150), i128 산술
460:            atk = j.attack_effect.as_ref().unwrap()   // +0x4c0 == -1 → unwrap 패닉
461:            if atk.is_in_range_pos(j, champ, x, y) {
462:                score.risk += ratio
            } else {
464:                r = atk.range(j, champ) + j.radius() + champ.radius()   // Effect::range 인라인 = attack_effect.range + 32000 + j.stat_buff_cached.range + (j.level-1)*growth_range + Effect::range_adjust(atk, j, champ); Entity::radius = mult==0 ? radius : radius*(mult+100)/100
466:                if jd <= r*r {   // IR: jd > r*r 이면 skip
467:                    score.risk += ratio >> 1   // 반값
                }
            }
          }
473:       Epic(5) => if info.focused.is_some() {
474:            damage = expected_attack_damage_cached(j, champ);  475: ratio = min(100*damage/hp, 150)
476:            atk = j.attack_effect.as_ref().unwrap(); if atk.is_in_range_pos(j, champ, x, y) {
477:                if info.focused == Some(champ.id) { 478: score.risk += ratio }
480:                else { score.risk += (ratio as u8) / 3 }   // trunc i128→i8 후 udiv 3 (ratio ≤150 이라 손실 없음)
            }
          }
       _ => {}   // Minion/Tower/Serpen/Ghoul/… 무시
     }
     }
489: disable_tick = pe_ctx.disable_tick   // = GameSetting.tower_attack_disable_tick(/2v2/3v3 튜토리얼 분기)
490: if game.tick() > disable_tick { → 병합점 %548: score.gain = 0 유지, 줄 573 으로 (배치 B) }   // ⚠ 방향 주의: tick 이 disable_tick 을 **넘으면** 491~572 타워 절을 통째로 건너뛴다(IR m07.ll:25967~25968 `icmp ugt tick, disable_tick → %548`)
     else {
491:   player_team = champ.player_team().unwrap()   // Entity+0 tag==0(Player) 아니면 unwrap 패닉, 페이로드 +8
492~494: enemy_towers = cache.iter_towers_without_nexus(enemy_team)   // 120B 이터레이터 = [top,top2,mid,mid2,bottom,bottom2] Option 6개 flatten + twin_towers 슬라이스 chain
            .filter(closure#1: |t| cell_dist_sq(t.x,t.y) < 22500000000 || near_enemies_with_action.iter().any(|(_,_,e,_)| e.distance_sq(t) < 4900000000))
495~497: ally_towers = cache.iter_towers_without_nexus(team).filter(closure#2: 동형, near_allies_with_action)
499~500: near_towers = enemy_towers.chain(ally_towers).map(|t| (t, cnt = if t.team != champ.team { tower_minion_in_range_count_cached(data, minion_team=player_team, t) } else { 0 }))   // TOWER_MINION_CNT_CACHE[(t.id, player_team)]
507:   for (t, cnt) in near_towers { → 배치 B(줄 510~) 본문 · 루프 백엣지 %3672→%637 은 score.risk/tower_risk/gain/tower_well_risk 4 phi 를 들고 돈다 }
       루프 종료(%738→%867) → %548 병합 → 줄 573 (배치 B)
     }
// 배치 A 밖: %2619(1001)·%3487(639)·%3635(510)·%3672(507 백엣지)·%4102(1181 반환) 블록은 phi 태그(390/399/507)만 내 범위 줄이고 로직은 B~D 것.

// position_eval.rs:510~675 (배치 B)
// ── 공통(배치 A 정의, 재확인만): champ=%122=cache.player_champion[team][pos] · ctx=%135=data.context · game=%118=cache.game(dyn, +0 data/+8 vtable) · my_id=%231=champ+0x5c0
//   inv_hp_q32=%149=2^32/max(champ.hp,1) · pct_q32(x)[L386~387] = min((inv_hp_q32*100*x)>>32, 150) = 피해/내 현재HP % (상한 150)
//   cell_dist_sq = closure#0(L395) = |x−ex|²+|y−ey|² (saturating) · distance_sq(a,b) = 같은 식(포화 없음)
//   루프 캐리(배치 A L507 `for t in near_towers` phi): score.risk=%640 · tower_risk=%639 · gain=%638 · 지역 tower_well_risk=%641 · cnt=%3636(배치 A closure#3 L498 산출 — 의미는 배치 A)
//
// ===== [A] 타워 루프 본문 L510~565 (진입: 배치 A L507 %841/%862/%863 → %3635 · 탈출: %3672 → 배치 A %637 L507) =====
// L510: let t_atk = t.attack_effect.as_ref().unwrap();                 // Entity+0x490, 태그 +0x4c0 == −1 → unwrap_failed 패닉
// L511: if t.team == champ.team {                                       // team 태그 0(Player) && 팀번호 동일
// L512:   if t.ty == Tower(2) {
// L513:     let any_enemy_in_tower_range = near_enemies_with_action.iter()
// L514:         .find(|e| t_atk.is_in_range(t, e.2));                    // closure#4 · e.2 = 원소+0x1b0 &Entity, stride 448
//           None → continue
// L518:     let damage = expected_attack_damage_cached(data, ctx, t, target);        // TLS AttackDamageCache 키 (t.id, target.id)
// L519:     let damage = damage + tps*damage / t.attack_cooltime();                   // cooltime 0 → div_by_zero 패닉
// L520:     let ratio = pct_q32(damage);
// L521:     if t_atk.is_in_range_pos(t, champ, x, y) {
// L522:       score.gain += (ratio as u8)/3; }
//           continue
//         } else continue
// L525: } else if t.ty == Tower(2) {                                     // 적(또는 비-Player) 타워
// L526:   let damage = expected_attack_damage_cached(data, ctx, t, champ);          // 키 (t.id, my_id)
// L527:   let damage = damage + tps*damage / t.attack_cooltime();
// L528:   let ratio = pct_q32(damage) as i64;
// L529:   let range = t_atk.range(t) + t.radius() + champ.radius();
//         //   Effect::range [effect.rs:26 인라인] = t.stat_buff_cached.range(0x438) + eff.range(0x4a0) + eff.growth_range(0x4a8)*(t.level(0x5c8)−1) + Effect::range_adjust(eff, t, champ)
//         //   Entity::radius [entity.rs:1511~1515 인라인] = if stat_buff_cached.radius_mult(0x470)==0 { radius(0x680) } else { radius*(100+mult)/100 }
// L532:   if !t_atk.is_in_range_ex(t, champ, t.x, t.y, x, y, 18000) { continue }
// L536:   let v47_soaker: (bool, usize) = v47_siege_stance(version, data, player, t);   // {i64,i64}
// L537:   let soaker_is_me = v47_soaker.0 && v47_soaker.1 == my_id;
// L539:   if t.info.nearest_enemy.map(|(_, id)| id == my_id) == Some(true) {        // Tower.nearest_enemy Option<(usize,usize)>: 태그 0x88 · .1 = 0x98 · closure#5
// L540:     let r = if soaker_is_me { ratio/3 } else { ratio };
//           score.risk += r; tower_well_risk += r;                                   // ★tower_risk 불변
//   } else {
// L544:     let dist = cell_dist_sq(t.x, t.y);
// L545:     let coef = if dist < (range/2)² { 100 } else { 50 };
// L547:     let risk = if soaker_is_me                                       { L548: ratio/3 }
// L549:       else if v47_soaker.0 && v47_tower_covered_for_me(data, t, my_id)  { L550: coef*3/100 }   // 3 또는 1
//                //  v47_tower_covered_for_me [tower_discipline.rs:597~613 인라인] = t.ty==Tower && t.nearest_enemy.is_some() && (tid = nearest_enemy.1) != my_id
//                //     && game.get_entity_by_id(tid) is Some(tgt) && t.attack_effect.is_some() && tgt.hp(0x670) > Effect::expected_damage_target(t_atk, ctx, t, @anon.11, tgt)
// L551:       else if !near_enemies_with_action.is_empty()  { L552: if t.nearest_enemy.is_some() { L555: ratio/2 } else { ratio } }
// L557:       else if cnt < 2                                { ratio }
// L559:       else if cnt == 2                               { L560: ((ratio*2)/3 * coef) / 100 }   // i16 산술
//             else                                           { L562: coef*3/100 };
// L565:     score.risk += risk; score.tower_risk += risk; tower_well_risk += risk;
//   }
//
// ===== [B] 라인 미니언 위험 L573~605 (진입: 배치 A L490 %542 · 타워 루프 종료 %867 → %548) =====
// L573: if position_eval_use_legacy_line_minion_risk(version, data, champ, purpose) [position_eval.rs:21~25 인라인, AND 3조건 이 순서]
//   L23: is_line_phase(ctx, tick) [runner.rs:399 / spawn_epic:263 / setting.rs:703~704] = !(ctx.tutorial(0x38) ∈ {0 None,5 MidBottom,7 Line,8 Total}) || tick < sat_sub(setting.epic_jungle.first_spawn_tick(0x8a8), tps*30)
//   L24: purpose ∈ {LineStyle(_)=태그0·1, Lane=8, LaneSafe=10}
//   L25: !enemy_minion_wave_has_epic_buff(data, champ) [minion_wave_risk.rs:92~96] = !(champ.player_team().is_some() && game.get_game_mode().as_moba().map_or(false, |m| m.remain_epic_time(1−team)(MobaMode+0x240) != 0))
//   → 참 = [B-2] legacy(L580) / 거짓 = [B-1](L574)   (if/else 표기 순서는 표기 불가 — 동작만 확정)
// [B-1] L574: let minion_wave_damage = enemy_minion_wave_risk_damage_at(version(poison), data, champ, x, y, L575: tps/2);
//       L576: score.risk += position_eval_minion_wave_risk_score(ctx, purpose, damage, champ.hp, champ.stat_cached.hp) [position_eval.rs:7~15 인라인]
//             L7: pct = position_eval_pct(damage, hp)[score_parameter.rs:53~57] = damage==0 ? 0 : min(damage*100/max(hp,1), 150); risk = min(pct, 140)
//             L8: if pct == 0 { return 0 }
//             L12: if is_line_phase(ctx, tick) L13: && purpose ∈ {LineStyle(_), Lane, LaneSafe} L14: && !enemy_minion_wave_is_dangerous(damage, hp, max_hp)
//                  [minion_wave_risk.rs:69~75] hp_pct=hp*100/max(max_hp,1) · damage_pct=damage*100/max(hp,1) · dangerous = damage>=hp || damage_pct>49 || (hp_pct<66 && damage_pct>29) || (hp_pct<41 && damage_pct>17) || (hp_pct<26 && damage_pct>9)
//             L15: { risk = min(risk/4, 18) }  → risk
// [B-2] L579: let mut range_minion_attack: Option<i64> = None; let mut melee_minion_attack: Option<i64> = None;   // 종류별 ratio 1회 계산 캐시
//       L580: for m in cache.iter_minions(enemy_team).filter(|m| cell_dist_sq(m.x, m.y) < 22500000000) {   // closure#6 = aux · 150000²
//       L582:   let (is_range, target_none, targets_other) = if m.ty == Minion(1) { (L584: m.info.is_range(0x118), L585: m.info.nearest_enemy(0x88).is_none(), L586: nearest_enemy.is_some_and(|id| id != my_id)(closure#7, id 0x90)) } else { (false,false,false) };
//       L590:   let ratio = if is_range { L591: if range_minion_attack.is_none() { L592: r = pct_q32(expected_attack_damage_cached(m, champ)); range_minion_attack = Some(r); r } else { L597: unwrap } }
//                           else       { L594: if melee_minion_attack.is_none() { L595: r = pct_q32(expected_attack_damage_cached(m, champ)); melee_minion_attack = Some(r); r } else { L597: unwrap } };
//       L598:   let ratio = if target_none { ratio/2 } else if targets_other { ratio/3 } else { ratio };
//       L599:   if cell_dist_sq(m.x, m.y) < 4096000001 /*64000²+1 = 2셀*/ { score.risk += ratio } }
// L605: drop(_t_peu2) — ProfTimer 텔레메트리(PHASE_NANOS/PHASE_CALLS fetch_add, prof::ENABLED 게이트)
//
// ===== [C] 기타 적 엔티티 L606~631 =====
// L606: let _t_peu3 = ProfTimer::start(114);
// L607: for e in cache.others[enemy_team](0xf0).iter() {
// L608:   if cell_dist_sq(e.x, e.y) > 22500000000 { continue }
// L611:   let Some(atk) = e.attack_effect.as_ref() else { continue };
// L612:   let damage = expected_attack_damage_cached(data, ctx, e, champ);
// L613:   let ratio = pct_q32(damage);
// L614:   let range = atk.range(e) + e.radius() + champ.radius();
// L615:   let range_ext = range + 32000;
// L616:   let nearest_enemy: Option<usize> = match e.ty { Ghoul(7)|Bear(9) => info.nearest_enemy(0x88/0x90), Eagle(10) => L618: info.nearest_enemy(0x70/0x78), _ => None };
// L622:   let targeting_me = nearest_enemy == Some(my_id);
// L623:   let dist_sq = cell_dist_sq(e.x, e.y);
// L624:   if dist_sq <= range² { L625: if targeting_me { L626: score.risk += ratio } else { L628: score.risk += ratio/2 } }
// L630:   else if dist_sq <= range_ext² { L631: score.risk += (ratio as u8)/3 } }
//
// ===== [D] 투사체 L638~675 (L678 가산 · L679 비-LinearDist · L684 · L687 루프 종료 = 배치 C) =====
// L638: let _t_proj = ProfTimer::start(73); let mut has_pcc = false;   (score.on_trajectory / on_periodic_trajectory = false 초기값)
// L639: for p in game.iter_projectile() (vtable+0x210 → ProjectileIter 40B, ::next) {
// L640:   if distance_sq(x, y, p.x, p.y) > 62499999999 { continue }                         // 250000 이상 = 무시
// L641:   if !(p.is_visible || (p.team == champ.team && p.name == "knight_ult")) { continue }
// L642:   if p.move_type.is_targeting() { continue }                                          // [projectile.rs:134] Target | TargetSplash | BouncingTarget{target_id: Some}
// L645:   let Some(caster) = game.get_entity_by_id(p.caster_id) else { continue };
// L648:   if !p.applyed_target.check_projectile(p, champ) { continue }
// L651:   if !p.is_in_orbit(x, y, champ.radius() + 18000) { continue }
// L655:   if p.team == champ.team {
// L656:     let ratio = position_eval_pct(L657: p.expected_heal_target(ctx, caster, champ) + L658: p.expected_shield_target(ctx, caster, champ), champ.hp);
// L661:     score.risk -= ratio;                                                              // 아군 힐/실드 투사체 = 위험 감산
//   } else {
// L663:     let ratio = pct_q32(p.expected_damage_target(ctx, caster, champ));
// L665:     let mut nearest_other_distance: Option<u64> = None;
//           for a in near_allies_with_action.iter() {
// L666:       if p.applyed_target.check_projectile(p, a.2) && p.is_in_orbit(a.2.x, a.2.y, a.2.radius()) {
// L669:         let dist = distance_sq(p.x, p.y, a.2.x, a.2.y);
// L670:         if nearest_other_distance.is_none_or(|d| dist < d) { nearest_other_distance = Some(dist) } } }   // closure#8 (IR: umin)
// L674:     let d = distance_sq(p.x, p.y, x, y);
// L675:     if is_linear_dist_no_penetrate(p) [projectile.rs:121~122 = LinearDist && !penetrate] {
//               if nearest_other_distance.is_some_and(|nd| nd < d) { continue }             // closure#9 · 아군이 먼저 맞아 막힘
//               → 배치 C(줄684): risk += ratio · on_trajectory = true · has_pcc
//           } else → 배치 C(줄679)
//   } }  루프 latch %3487→%1259 · 종료 %1287 → 배치 C(줄687)

// position_eval.rs:678~960 (배치 C)
// ── 투사체 루프 꼬리 (배치 B L639~677 루프 본문 안, 한 투사체 p=%1265 에 대해 ratio=%3398 이 계산된 뒤) ──
L678: score.risk += ratio;                                   // %3481 (m07.ll:32614)
L679: if p.move_type 논리idx == 2 (Periodic, 메모리태그 4) { score.on_periodic_trajectory = true; }   // %3331==2 → sel (32603~32605)
      else                                                  { score.on_trajectory = true; }
      // 진입 경로가 %3454(배치 B, 투사체 ratio 0 경로?) 이면 on_trajectory=true 만 (32611~32613 phi). 다른 필드는 유지
L684: has_pcc = has_pcc || p.has_cc();                      // 이미 true 면 호출 생략 (32619~32629). has_pcc 는 배치 D 소비
      → 루프 헤더 %1259(배치 B L639) 로 복귀
// ── 투사체 루프 종료 후 ──
L687: drop(timer %79 /*Option<ProfTimer>*/);                // prof 텔레메트리: tag!=-1 이면 PHASE_NANOS[phase]+=elapsed, PHASE_CALLS[phase]+=1 (27935~27991). 언와인드 정리패드 %1254 도 같은 drop
L689: if is_in_well_damage(enemy_team, x, y) {              // game.rs:5654 인라인. enemy_team=1-team(%150) 을 `%112==1` 로 접음
      //   enemy_team==0: (x≤64000 ∧ 800000≤y≤960000) ∨ (x≤160000 ∧ 896000≤y≤960000)
      //   enemy_team==1: (800000≤x≤960000 ∧ y≤64000) ∨ (896000≤x≤960000 ∧ y≤160000)
L690:     score.risk += pct_q32(setting.well_damage, inv_hp_q32)  // = min((well_damage*100*inv_hp_q32)>>32, 150) — 우물 데미지의 HP%, 150 캡 (28056~28069)
      }
L693: let mut max_ratio: i64 = 0;
// ── gain 루프 3종: 내가 상대에게 줄 수 있는 최대 ratio (캐시 튜플 = (EntityPositioningCache, &PlayerState, &Entity, dist_sq)) ──
L694: if !champ.is_block_attack() {                         // %1336 (28052)
L695:   for (cache, _, _, dist) in enemies(%95) {           // 5명, stride 448
L696:     ratio = cache.attacked_ratio;                      // +0x20
L698:     if dist > cache.attacked_range_sq {                // +0xc8
L700:       if dist <= cache.attacked_range_ext_sq {         // +0xd0
L701:         max_ratio = max(max_ratio, ratio/2); } }
L699:     else { max_ratio = max(max_ratio, ratio); }
        }
      }
L706: if champ.skill_effect.is_some() /*0x4f8!=-1*/ && champ.skill_cooldown() < 181 /*Champion 이면 +0xb8, 아니면 0*/ {
L707:   for (cache, _, e, dist) in enemies(%95) {
L708~710: ratio=cache.skilled_ratio(+0x28); range=cache.skilled_range(+0x58);
L711:     if pe_ctx.skill_linear_move {                     // %286
L712:       if dist <= cache.skilled_range_sq(+0xd8) && !(range > e.radius() + 80000 + champ.radius()) {   // 짧은 대시: 사거리 안이면 풀
L713:         max_ratio = max(max_ratio, ratio); }
L714:       else if dist <= cache.skilled_range_half_sq(+0xe8) { L715: max_ratio = max(max_ratio, ratio); }
            else if dist <= skilled_range_sq                { L717: max_ratio = max(max_ratio, ratio/2); }
L718:       else if dist <= cache.skilled_range_ext_sq(+0xe0){ L719: max_ratio = max(max_ratio, ratio/3); }
          } else {
L721:       if dist <= skilled_range_sq                     { L722: max_ratio = max(max_ratio, ratio); }
L723:       else if dist <= skilled_range_ext_sq            { L724: max_ratio = max(max_ratio, ratio/2); }
          }
        }
L728:   for (cache, _, a, dist) in allies(%93) {            // 아군에게도 같은 사다리(대상 아군 스킬 = 힐/버프 추정, 필드명은 동일 skilled_*)
L729~733: ratio=skilled_ratio; range=skilled_range; r_sq/r_half_sq/r_ext_sq = 0xd8/0xe8/0xe0
L734:     if skill_linear_move { L735: (dist≤r_sq ∧ !(range > a.radius()+80000+champ.radius())) → L736 풀 / L737 dist≤half → L738 풀 / dist≤sq → L740 /2 / L741 dist≤ext → L742 /3 }
L744:     else { L745 dist≤sq → 풀 / L746 dist≤ext → L747 /2 }
        }
      }
L752: if champ.skill2_effect().is_some() /*level>2 이고 0x530!=-1*/ && champ.skill2_cooldown() < 181 /*+0xc0*/ {
L753:   for enemies: ratio=skilled2_ratio(+0x30) range=skilled2_range(+0x60) sq/ext/half=0xf0/0xf8/0x100; L757 skill2_linear_move(%289): L758(짧은대시)→L759 풀 / L760 half→L761 풀 / sq→L763 /2 / L764 ext→L765 /3 ; 비직선 L767 sq→L768 풀 / L769 ext→L770 /2
L774:   for allies : 동일 (L778 분기, L779~L786 직선 / L788~L791 비직선)
      }
L796: if champ.ult_effect().is_some() /*level>4 이고 0x568!=-1*/ && champ.ult_cooldown() < 181 /*+0xc8*/ {
L797:   for enemies: ratio=ulted_ratio(+0x38) range=ulted_range(+0x68) sq/ext/half=0x108/0x110/0x118; L801 ult_linear_move(%292): L802→L803 풀 / L804 half→L805 풀 / sq→L807 /2 / L808 ext→L809 /3 ; 비직선 L811 sq→L812 풀 / L813 ext→L814 /2
L818:   for allies : 동일 (L822 분기, L823~L830 직선 / L832~L835 비직선)
      }
L840: score.gain += max_ratio;  score.gain_me = max_ratio;   // gain 은 배치 B 까지의 값(%549)에 가산 (29002~29006)
L846: my_role = pe_ctx.my_role;                              // PePlayerCtx+0x20
L847: is_attacker_role = (my_role & 6) == 2;                 // BaseAttacker|SkillCaster → %1787, 배치 D L1001 소비
L850: drop(timer %80→%76);                                  // prof
L851: timer %75 = ProfTimer::start(phase 115);              // prof::ENABLED 일 때만 Instant::now
// ── 적 챔피언별 위협 루프 (배치 D 로 이어짐) ──
L853: champ_threat_risk = 0; /*score.unseen_champ_threat 루프 phi 초기 0*/
      for (cache, _, e, dist) in enemies(%95) {             // 루프 헤더 %2004, 재진입 %2688(배치 D L1018) 에서 score.risk/unseen_champ_threat/champ_threat_risk 누적값 phi
        attack_ratio=skill_ratio=skill2_ratio=ult_ratio=rush_ratio=0; has_cc=false;
L860:   dist = cache.dist;
L862:   if !(dist < 10000000000 /*100000²*/ || game.is_visible(enemy_team, champ.id) /*vtable+0xf8*/) { continue; }
L866:   ratio = cache.attack_ratio(+0x0);
L867:   if dist > cache.attack_range_sq(+0x70) {
L870:     if dist <= cache.attack_range_ext_sq(+0x78) {
L871:       attack_ratio = max(ratio/2, 0);
L872:       has_cc = slot_cc_time_cached(version, data, e, 0).is_some(); } // else attack_ratio=0, has_cc=false
        } else {
L868:     attack_ratio = max(ratio, 0);
L869:     has_cc = slot_cc_time_cached(version, data, e, 0).is_some();
        }
L874:   if e.is_block_attack() { attack_ratio /= 3; }          // udiv (29590~29591)
L878:   if slot_ready_cached(data, e, 1) {                       // 적 skill 준비됨
L879~881: ratio=cache.skill_ratio(+0x8); range=cache.skill_range(+0x40);
          if cache.is_skill_linear_move(+0x1a0) {
L882:       if dist <= cache.skill_range_ext_sq(+0x88) && !has_cc { L883: has_cc = slot_cc_time_cached(version,data,e,1).is_some(); }
L885:       if dist <= cache.skill_range_sq(+0x80) && !(range > e.radius() + 80000 + champ.radius()) { L886: skill_ratio = max(ratio,0); }
L887:       else if dist <= cache.skill_range_half_sq(+0x90)      { L888: skill_ratio = max(ratio,0); }
            else if dist <= skill_range_sq                        { L890: skill_ratio = max(ratio/2,0); }
L891:       else if dist <= skill_range_ext_sq                    { L892: skill_ratio = max(ratio/3,0); }
          } else {
L894:       if dist <= skill_range_sq { L895: skill_ratio = max(ratio,0); L896: if !has_cc { has_cc = cc(e,1).is_some(); } }
L897:       else if dist <= skill_range_ext_sq { L898: skill_ratio = max(ratio/2,0); L899: if !has_cc { has_cc = cc(e,1).is_some(); } }
          }
        }
L903:   if slot_ready_cached(data, e, 2) {                       // skill2: 필드 +0x10/+0x48/+0x1a1, sq/ext/half=0x98/0xa0/0xa8
L904~927: 위와 동일 사다리 (직선: L907 cc게이트·L908 cc / L910→911 풀 / L912→913 풀 / →915 /2 / L916→917 /3 ; 비직선: L919→920 풀·L922 cc / L923→924 /2·L925 cc)
        }
L929:   if slot_ready_cached(data, e, 3) {                       // ult: 필드 +0x18/+0x50/+0x1a2, sq/ext/half=0xb0/0xb8/0xc0
L930~953: 동일 사다리 (직선: L933 cc게이트·L934 cc / L936→937 풀 / L938→939 풀 / →941 /2 / L942→943 /3 ; 비직선: L945→946 풀·L948 cc / L949→950 /2·L951 cc)
        }
L955:   if e.is_block_skill() { L956: skill_ratio /= 3; L957: skill2_ratio /= 3; L958: ult_ratio /= 3; }   // udiv
L960:   if e.is_block_move_skill() → 배치 D(줄 961, %2393) / else → 배치 D(줄 961, %2388)   // 루프 본문 나머지(rush_ratio·added·score.risk/unseen_champ_threat 누적, %2688 에서 헤더 %1994 로 복귀)는 배치 D
      }

// position_eval.rs:961~1653 (배치 D)  — 실효 961~1181 (1181 = `}`; 1183~ 는 별개 함수)
// 문맥(배치 C 승계): 853 `for cache in near_enemies_with_action(%95)` 루프 안. cache=(EPC .0, &PlayerState .1, ent: &Entity .2, dist: u64 .3 = 셀↔ent 거리²). 루프 진입 조건 862(dist<1e10 || 슬롯 시야) 통과 후 attack_ratio(%2056)·skill_ratio(%2380)·skill2_ratio(%2379)·ult_ratio(%2381)·has_cc(%2273, 872~) 가 배치 C 에서 계산돼 들어온다. 루프 캐리: score.risk(%1997)·score.unseen_champ_threat(%1996)·score.on_trajectory(%1995)·champ_threat_risk(%1999, 초기 0).

// ── 961~968: 이동 가능 스킬 감쇠 (m07.ll:30217~30334)
961: if ent.skill_effect.as_ref().map_or(false, |e| e.ty.can_move())   // Option tag@0x4f8 != -1 · vtable 0x120
       { skill_ratio /= 3 }          // udiv 3 (30247)
964: if (level>2 ? ent.skill2_effect : None).map_or(false, |e| e.ty.can_move()) { skill2_ratio /= 3 }   // 30256~30290
967: if (level>4 ? ent.ult_effect : None).map_or(false, |e| e.ty.can_move()) {
968:   ult_ratio /= 3 }                                                     // 30296~30332

// ── 971~975: 입력 차단 상태 반감 (30212·30359~30366)
971: if Entity::block_input(ent) {
972:   attack_ratio >>= 1;  973: skill_ratio >>= 1;  974: skill2_ratio >>= 1;  975: ult_ratio >>= 1 }

// ── 978~990: 돌진 궤도 위험 rush_ratio (30348~30662)
     rush_ratio = 0;   // on_trajectory 는 기존값 유지
978: match ent.rush_state {   // tag: raw@0x308 <0 ? raw^0x8000… : 4
       RushState::Rush{applyed_effect@0x310, x@0x330 ex, y@0x338 ey, range@0x340, start_tick@0x348, casting_target@0x35c, ..}
979:   | RushState::RushPenetrate{applyed_effect@0x308, x@0x340, y@0x348, range@0x350, start_tick@0x358, casting_target@0x36c, ..} => {
980:     if CastingTarget::check(&casting_target, ent, champ)                          // 30431
981:        && game.tick() >= start_tick                                             // vtable 0x28 · `tick < start_tick` 이면 skip (30443)
982:        && dist_to_line_segment(x, y, ent.x, ent.y, ex, ey)                       // 점=평가 셀, 선분=ent 현위치→돌진 목적지 (30455)
983:             <= range + 20000 + champ.radius()   {                                // radius()=radius_mult 보정 (30460~30481)
984:       let (ad, ap) = applyed_effect.iter().fold((0,0), |(ad,ap),(e,_ct)| {
985:            let (ad2, ap2) = e.ty.expected_damage(ctx, ent as &dyn AbstractEntity);   // vtable 0x28 (30547)
986:            (ad+ad2, ap+ap2) });                                                       // closure#14 인라인 (30513~30570)
987:       let dmg = get_damage::<Entity>(ent, champ, ad, AttackType::Skill(1), DamageType::AD(0))   // 30577
988:               + get_damage::<Entity>(ent, champ, ap, AttackType::Skill(1), DamageType::AP(1));  // 30582
989:       rush_ratio = min(pct_q32(inv_hp_q32, dmg), 150);   // = min((inv_hp_q32*100*dmg)>>32, 150) ≈ 내 HP 대비 % (30590~30596)
990:       if !has_cc { has_cc = applyed_effect.iter().any(|(e,_)| effect_type_cc_time(version, &*e.ty).is_some()) }   // closure#15, .0==1 = Some (30600~30658)
           score.on_trajectory = true;   // 30660 (980 root)
         }   // 980~983 중 하나라도 실패 → rush_ratio=0, on_trajectory 유지 (%2480)
       }
       _ => {}   // None/Move/MoveToTarget
     }

// ── 995~998: 이 적의 risk 합산 규칙 (30377~30688)
995: let risk = if has_cc || has_pcc {          // has_pcc=%1263 = 배치 B 639 투사체 루프 산출
996:     skill_ratio + skill2_ratio + attack_ratio + ult_ratio + rush_ratio          // 전부 합
     } else {
998:     rush_ratio + max(attack_ratio, skill_ratio, skill2_ratio, ult_ratio) };    // 최대 1개 + 돌진

// ── 1001~1008: 후방 딜러의 아군 몸빵 반감 (30697~30829)
1001: if matches!(my_role, BaseAttacker|SkillCaster) && risk > 0 {     // (my_role&6)==2 · my_role = PePlayerCtx+0x20 (배치 C 847)
1003:   if near_allies_with_action.iter().any(|a| {                       // closure#16 인라인, 원소 448B
1004:        dist_sq(a.entity.pos, ent.pos) < cache.dist                   // 아군이 나(셀)보다 적에 가깝고 (30768~30781)
1005:     && dist_to_line_segment(a.entity.x, a.entity.y, x, y, ent.x, ent.y)   // 아군이 셀→적 선분 근처 (30787)
1006:          <= a.entity.radius() + 28000 })                              // 30791~30815
1008:   { risk >>= 1 } }

// ── 1012~1018: 미시야 분할·누적 (30835~30856)
1012: let added = if visible { risk } else { risk / 2 };    // visible=%156 = game.is_visible_cell(enemy_team, xi, yi) (배치 A 394) · sdiv 2
1013~1015: if !visible { score.unseen_champ_threat += risk - added }
1017: score.risk += added;
1018: champ_threat_risk += added;
     // → 853 루프 다음 적

// ── 1024~1030: 루프 종료 후 고립 가산 (29507·30860~31080)
1024: if champ_threat_risk > 0 {
1025:   let cell_a = near_allies_with_action.iter().filter(|a| a.dist <= 120000²).count();   // ult 14400000001 (closure#17)
1026:   let cell_e = near_enemies_with_action.iter().filter(|e| e.dist <= 120000²).count();  // closure#18
1029:   if cell_a == 0 && cell_e > 1 {
1030:     score.risk += champ_threat_risk * 25 * min(cell_e as i64, 4) / 100 } }   // +25%/적, 최대 4명 (31073~31077)
1034: drop(_t)   // ProfTimer phase 115 (배치 C 851) — PHASE_NANOS/CALLS[115] atomic add, prof 전용

// ── 1035~1096: 아군 스킬 지원 gain (31087~31610)
1035: let _t = ProfTimer::new(116)   // prof::ENABLED 일 때만 Instant::now
1036: for a in near_allies_with_action {           // score.gain 캐리 %2779 (초기 %1783 = 배치 C 840)
1038:   let dist = a.dist;   1039: let ally = a.entity;  let mut max_ratio = 0;
1039:   if ally.skill_effect.is_some() {
1040:     let ratio = a.skill_ratio;  1041: let range = a.skill_range;
1042:     if a.is_skill_linear_move {
1043:       if dist <= a.skill_range_sq && !(range > ally.radius() + 80000 + champ.radius())
1044:            { max_ratio = max(ratio, 0) }                       // 31294
1045:       else if dist <= a.skill_range_half_sq { 1046: max_ratio = max(ratio,0) }   // 31212·31233 (⚠1043 조건 실패 후에만)
1047:       else if dist <= a.skill_range_sq  { 1048: max_ratio = max(ratio/2, 0) }    // 1047 은 IR 없음(접힘) · 31298
1049:       else if dist <= a.skill_range_ext_sq { 1050: max_ratio = max(ratio/3, 0) } // 31225·31237
            // 그 밖 → 0
1052:     } else if dist <= a.skill_range_sq { 1053: max_ratio = max(ratio, 0) }    // 31182·31196
1054:       else if dist <= a.skill_range_ext_sq { 1055: max_ratio = max(ratio/2, 0) }   // 31188·31200
          }
1058:   if level>2 && ally.skill2_effect.is_some() {   // 동형: ratio=a.skill2_ratio(0x10) range=a.skill2_range(0x48) linear=0x1a1 sq/ext/half=0x98/0xa0/0xa8
1059~1074: 1062 `dist<=sq && !(range > r_a+80000+r_c)` →1063 ratio / 1064 half →1065 ratio / 1067 ratio/2 / 1068 ext →1069 ratio/3 // 비선형 1071 sq→1072 ratio / 1073 ext→1074 ratio/2
            max_ratio = max(max_ratio, 그 값) }                          // smax 누적 (31380·31348·31355·31441·31448·31387)
1077:   if level>4 && ally.ult_effect.is_some() {   // ratio=ult_ratio(0x18) range=ult_range(0x50) linear=0x1a2 sq/ext/half=0xb0/0xb8/0xc0
1078~1093: 1081/1082/1083/1084/1086/1087/1088 (선형) · 1090/1091/1092/1093 (비선형) 동형 → max_ratio = max(max_ratio, …) }
1096:   score.gain += max_ratio }                                          // 31599

// ── 1101~1127: 탱커/이니시에이터의 후방 딜러 '진입가치' (31163~31639)
1101: if matches!(my_role, Tanker|Initiator) {                              // my_role < 2
1103:   let mut enabled = 0;
       for ap in near_allies_with_action {
1105:     if matches!(get_battle_role(version, ctx, cache, ap.player), BaseAttacker|SkillCaster)   // (role&6)==2 (31632~31643)
1109:       && dist_sq_sat(셀, ap.entity.pos) <= 120000²   {     // umul.with.overflow→-1 · uadd.sat (31659~31723)
1113:       if let Some(e) = near_enemies_with_action.iter().find(|e| {
1114~1116:        let a_to_e_sq = dist_sq(ap.entity.pos, e.entity.pos);  e.dist < a_to_e_sq     // 셀이 아군보다 적에 가깝고 (31790)
1117~1118:     && dist_to_line_segment(x, y, ap.entity.x, ap.entity.y, e.entity.x, e.entity.y) <= champ.radius() + 28000 })  // 셀이 아군→적 선분 위 (31799~31824)
            {
1119:         if let Some(atk) = ap.entity.attack_effect {            // tag@0x4c0 != -1
1120:           let dmg = atk.expected_damage_target(ctx, ap.entity as &dyn AbstractEntity, e.entity);   // 31837
1121:           enabled += dmg * 100 / max(e.entity.stat_cached.hp, 1) } } } }   // 아군 평타의 적 스탯HP% (31842~31849)
1127:   score.gain += enabled }

// ── 1134~1136: 교환 순이익 (31870~31878)
1134: let non_tower_risk = max(score.risk - tower_well_risk, 0);      // %552 = 배치 B 400 줄
1135: let trade_net = gain_me(max_ratio %1782) - non_tower_risk;
1136: score.gain += max(trade_net, 0);
1141: drop(_t)   // phase 116

// ── 1142: score = apply_position_eval_purpose(score, version, player, data, x, y, purpose)  — 인라인(position_eval.rs:29~44) (31947~32010)
       match purpose { LineStyle(Defensive) => risk = risk*120/100 (33)  |  LineStyle(Aggressive) => gain = gain*120/100 (36)
                       LaneSafe => { risk = risk*3/2 (39); tower_risk = tower_risk*3/2 (40) }  |  _ => {} }
       *sret = PositioningScore{ risk, tower_risk, gain, gain_me, adjust: 0, unseen_champ_threat, on_trajectory, on_periodic_trajectory }   // 44줄 store 8건

// ── 1148~1176: 선수 포지셔닝 정확도 노이즈 (32011~32157)
1148: let acc = AthleteParameter::positioning_accuracy(&player.info.parameter);
1152: let noisy = if version > 1 { !matches!(purpose, RunAway|Recall) && acc < 1000 }
1153:             else { acc < 1000 };
       if !noisy { return }   // → 1181 drop 경로 (%3223)
1154: let base_risk = risk - tower_risk;
1155: if base_risk != 0 {                                           // risk == tower_risk 면 return
1157:   let spread = if game.get_game_mode() is DeathMatch(tag 2)    // vtable 0x40
1158:                { 1000 - acc }
1160:                else { (2000 - 2*acc) / 3 };
1165:   let t_salt = if version > 1 { 1166: game.tick() / max(tps*6, 1) }   // 6초 버킷 (tps = setting.tick_per_second)
1168:                else { game.tick() };
1170:   let mut h = player.info.id;
1171:   h = (h ^ t_salt) * 0x9E3779B97F4A7C15;
1172:   h = (h ^ (x/32000)) * K;      // %130 = 클램프 전 x 셀
1173:   h = (h ^ (y/32000)) * K;      // %132
1174:   h ^= h >> 31;
1175:   let factor = (h % (2*spread + 1)) as i64 - spread + 1000;   // 1000 ± spread (‰)
1176:   sret.risk = tower_risk + base_risk * factor / 1000 }   // sdiv, 32154~32157
1181: }   // near_enemies(%95)·near_allies(%93) bumpalo Vec drop · %98(phase 배치 A) ProfTimer drop · ret
```

**`mem` 메모리 접근 209건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | team(0/1) · m07.ll:25000 · 적 루프 eplayer(34029)·아군 eplayer(33763) 도 같은 필드 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 태그 → as_index (entity.rs:581) · 25008 | 4 | OK |  |
| 2 | PlayerState | 0x928 | info.id | r | 호출자 캐시키(24544)·pe_player_ctx 키(m00.ll:212). 본체 직접 load 는 아님(tls 절 근거) \| (배치 D) 1170 노이즈 해시 시드 h0 | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache · 25010 \| (배치 B) &AbstractGameWithCache = %118 (배치 A 정의, 재확인) | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | &GameContext · 25071 \| (배치 B) &GameContext = %135 (배치 A 정의, 재확인) | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] → pe_cand_masks 캡처 · 25146 | 4 | OK |  |
| 6 | GameContext | 0x0 | pool | r | &Bump — near_enemies/near_allies Vec::new_in(bump) · 25221 | 4 | OK |  |
| 7 | GameContext | 0x20 | map | r | &MapDef · 25073 | 4 | OK |  |
| 8 | MapDef | 0x78 | walls[30][30] | r | walls[yi][xi] != 0 → 벽 · 25075~25079 | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame: ptr@0, vtable@8) | r | vtable +0x20 seed() · +0x28 tick() · +0x100 is_visible_cell(team,xi,yi) (divtable 확인) · 25115~25120 | 3 | OK |  |
| 10 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | Option<&Entity> (null=None) · 25011 (내 챔프) · 33924 (적 i) · 33653 (아군 i) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x230 | player_state[2][5] | r | Option<&PlayerState> · 25232/34021(적) · 25284/33755(아군) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x280 | player_champion_cache[2][5] | r | ChampionCache 800B(20×[5 x i64]) 참조만 만들어 EPC 헬퍼에 넘김 · 25142~25144(champ_cache) · 34047(e_cache) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0xd0 | jungles.ptr | r | bumpalo Vec<&Entity> · 25483 | 4 | OK |  |
| 14 | AbstractGameWithCache | 0xe8 | jungles.len | r | 25486 | 4 | OK |  |
| 15 | Entity | 0x0 | team@tag | r | champ.player_team() (491) · 507 map 클로저 t.team != champ.team 비교(26878~26884) \| (배치 B) 0 = Player. L511/641/655 팀 비교(파생 PartialEq entity.rs:1127) · L573 player_team() 니치 | 4 | OK |  |
| 16 | Entity | 0x8 | team@Player.0 | r | player_team 페이로드(usize) · 25973~26098 · 26954 \| (배치 B) 팀 번호 — 타워·투사체와 챔피언 비교 | 4 | OK |  |
| 17 | Entity | 0x68 | ty@tag (EntityType) | r | 정글 루프 switch: 4=Jungle · 5=Epic · 그 외 무시 · 25642~25647 | 4 | OK |  |
| 18 | Entity | 0x88 | ty@Jungle/Epic.info.focused@tag | r | Option<usize> 태그(0=None) · 25667(Jungle) · 25675(Epic) | 4 | OK |  |
| 19 | Entity | 0x90 | ty@Jungle/Epic.info.focused@Some.0 | r | == champ.id 비교 · 25688 · 25947 | 4 | OK |  |
| 20 | Entity | 0x438 | stat_buff_cached.range | r | Effect::range 인라인(effect.rs:26) 항 · 25783 \| (배치 B) Effect::range(effect.rs:26 인라인) 합산항 | 4 | OK |  |
| 21 | Entity | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius 인라인(entity.rs:1511~1515) · 25797(j) · 25820(champ) \| (배치 B) Entity::radius(entity.rs:1511~1515 인라인): 0 이면 radius, 아니면 radius*(100+mult)/100 | 4 | OK |  |
| 22 | Entity | 0x490 | attack_effect@Some.0 (Effect) | r | as_ref().unwrap() → is_in_range_pos/range_adjust 의 self · 25753 · 25924 \| (배치 D) 1119(ally): atk = &ally.attack_effect → Effect::expected_damage_target | 4 | OK |  |
| 23 | Entity | 0x4a0 | attack_effect.range | r | 25777 \| (배치 B) Effect::range 합산항 | 4 | OK |  |
| 24 | Entity | 0x4a8 | attack_effect.growth_range | r | ×(level−1) · 25779 \| (배치 B) × (level−1) | 4 | OK |  |
| 25 | Entity | 0x4c0 | attack_effect@tag (i32, -1=None) | r | unwrap 가드 · 25755 · 25919 | 4 | OK |  |
| 26 | Entity | 0x5c0 | id | r | champ.id(25283) · 아군 루프 e.id==champ.id 자기제외(33746) · focused 비교 · 캐시키(공격자/타워 id) \| (배치 B) my_id=%231 · TLS 키 (attacker.id,target.id) · nearest_enemy 비교 | 4 | OK |  |
| 27 | Entity | 0x5c8 | level | r | growth_range 계수 · 25781 \| (배치 B) Effect::range: growth_range*(level−1) \| (배치 D) 964/967(ent) · 1058/1077(ally): skill2 는 level>2, ult 는 level>4 게이트(entity.rs:1693/1701 인라인) | 4 | OK |  |
| 28 | Entity | 0x660 | x | r | cell_dist_sq/distance_sq 피연산자 · 25546 등 다수 \| (배치 B) 타워·미니언·others·아군 좌표 \| (배치 D) 982(ent) · 1004/1109(ally) · 1114(e) | 4 | OK |  |
| 29 | Entity | 0x668 | y | r | 25548 등 \| (배치 D) 982 · 1004/1109 · 1114 | 4 | OK |  |
| 30 | Entity | 0x670 | hp | r | champ.hp → inv_hp_q32 = 2^32 / max(hp,1) · 25089 | 4 | OK |  |
| 31 | Entity | 0x680 | radius | r | Entity::radius 인라인 · 25805/25826 \| (배치 B) Entity::radius() | 4 | OK |  |
| 32 | Blackboard | 0x78 | small_actions[5] | r | pe_cand_masks 클로저(m00.ll:75297·75331) is_some 검사 — tls 절 근거, 본체 직접 load 아님 | 4 | OK |  |
| 33 | GameSetting | 0x13f8 | tower_attack_disable_tick (/_2v2@0x1400 /_3v3@0x1408) | r | pe_player_ctx 클로저(m00.ll:256) — disable_tick 출처 · 본체 직접 load 아님 | 4 | OK |  |
| 34 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data ptr) | r | %151/%556 등 — vtable 호출 self | 4 | OK |  |
| 35 | AbstractGameWithCache | 0x8 | game.vtable | r | 슬롯 0x20 seed · 0x28 tick · 0x40 get_game_mode · 0x1f0 get_entity_by_id · 0x210 iter_projectile (divtable AbstractGame) | 3 | OK |  |
| 36 | AbstractGameWithCache | 0xf0 | others[enemy_team] | r | L607 bumpalo Vec<&Entity> 32B stride(팀 인덱스) · ptr @+0 · len @+24 — 기타 적 엔티티 루프 | 4 | OK |  |
| 37 | GameContext | 0x8 | setting | r | &GameSetting \| (배치 C) → &GameSetting (L690, m07.ll:28056) | 4 | OK |  |
| 38 | GameContext | 0x38 | tutorial@tag | r | TutorialType 1B — is_line_phase 의 spawn_epic 게이트 {0 None, 5 MidBottom, 7 Line, 8 Total} | 4 | OK |  |
| 39 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | is_line_phase: tick < first_spawn_tick − 30*tps | 4 | OK |  |
| 40 | GameSetting | 0x12f8 | tick_per_second | r | L519/527 tps*damage/cooltime · L575 tps/2 · is_line_phase tps*30 \| (배치 D) 1166 t_salt = tick / max(tps*6, 1) | 4 | OK |  |
| 41 | MobaMode | 0x240 | epic_minion_buff_time[team] | r | remain_epic_time(team) [game.rs:210] — enemy_minion_wave_has_epic_buff 의 != 0 판정(L25<573) | 4 | OK |  |
| 42 | Entity | 0x68 | ty@tag | r | EntityType: 1 Minion · 2 Tower · 7 Ghoul · 9 Bear · 10 Eagle (L512/525/582/616) | 4 | OK |  |
| 43 | Entity | 0x70 | ty@Eagle.info.nearest_enemy@tag | r | L618 Eagle 만 이 오프셋(phi 112/120) | 4 | OK |  |
| 44 | Entity | 0x78 | ty@Eagle.info.nearest_enemy@Some.0 | r | L622 == my_id | 4 | OK |  |
| 45 | Entity | 0x88 | nearest_enemy@tag (Tower Option<(usize,usize)> / Minion·Ghoul·Bear Option<usize>) | r | L539/549/552(Tower) · L585/586(Minion) · L616(Ghoul\|Bear) | 4 | OK |  |
| 46 | Entity | 0x90 | nearest_enemy@Some.0 (Minion·Ghoul·Bear id) | r | L586 id != my_id · L622 == my_id | 4 | OK |  |
| 47 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 (id) | r | L539 == my_id · v47_tower_covered_for_me:601 target_id | 4 | OK |  |
| 48 | Entity | 0x118 | ty@Minion.info.is_range | r | L584 원거리/근접 미니언 분기 | 4 | OK |  |
| 49 | Entity | 0x490 | attack_effect@Some.0 (Effect 시작) | r | &Effect 로 넘김(t_atk/atk). +0x10=range(0x4a0) · +0x18=growth_range(0x4a8) | 4 | OK |  |
| 50 | Entity | 0x4c0 | attack_effect@tag (i32, −1 = None) | r | L510 unwrap(패닉) · L611 as_ref(None→continue) · v47_tower_covered_for_me:607 | 4 | OK |  |
| 51 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | L576 enemy_minion_wave_is_dangerous hp_pct 분모 | 4 | OK |  |
| 52 | Entity | 0x670 | hp (현재) | r | inv_hp_q32 분모(배치 A) · L576/656 position_eval_pct 분모 · v47_tower_covered_for_me:613 tgt.hp > 예상피해 | 4 | OK |  |
| 53 | (EntityPositioningCache,&PlayerState,&Entity,u64) 원소 | 0x1b0 | .2 = &Entity | r | near_enemies_with_action(%95, len %200) / near_allies_with_action(%93, len %230) 원소 stride 0x1c0(448) — L514/551/666 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 54 | Projectile | 0x0 | team@tag | r | L641/655 팀 비교 | 4 | OK |  |
| 55 | Projectile | 0x8 | team@Player.0 | r |  | 4 | OK |  |
| 56 | Projectile | 0x40 | move_type@tag (니치: 0/1 = BouncingTarget.target_id None/Some · 2 LinearDist · 6 Target · 7 TargetSplash …) | r | L642 is_targeting · L675 is_linear_dist_no_penetrate | 4 | OK |  |
| 57 | Projectile | 0x78 | move_type@LinearDist.penetrate | r | L675 !penetrate | 4 | OK |  |
| 58 | Projectile | 0x98 | name.ptr | r | L641 memcmp "knight_ult" 10B | 4 | OK |  |
| 59 | Projectile | 0xa0 | name.len | r | L641 == 10 | 4 | OK |  |
| 60 | Projectile | 0xf8 | caster_id | r | L645 get_entity_by_id | 4 | OK |  |
| 61 | Projectile | 0x100 | x | r | L640/669/674 distance_sq | 4 | OK |  |
| 62 | Projectile | 0x108 | y | r |  | 4 | OK |  |
| 63 | Projectile | 0x12c | applyed_target@tag (CastingTarget) | r | L648/666 CastingTarget::check_projectile(&self, p, ent) | 4 | OK |  |
| 64 | Projectile | 0x131 | is_visible | r | L641 | 4 | OK |  |
| 65 | RefCell<AttackDamageCache> (TLS) | 0x40 | seed / +0x48 tick (RefCell 헤더 8B 뒤 = 절대 0x48/0x50) | r | 에포크 비교 — 클로저 m00.ll:75776~75792 (aux 아님, 계약만) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 66 | OperationData(%3) | 0x8 | context | r | → &GameContext (m07.ll:25071 은 배치 A gep, 이 범위는 L690 에서 그 ptr 을 사용) | 4 | OK |  |
| 67 | GameSetting | 0x1490 | well_damage | r | L690 (m07.ll:28058~28059). 적 우물 장판 틱 데미지(usize) | 4 | OK |  |
| 68 | Entity(champ %122) | 0x4f8 | skill_effect@tag(i32 니치, -1=None) | r | L706 `champ.skill_effect().is_some()` 인라인(effect Option::is_some effect.rs:633) m07.ll:28109 | 4 | OK |  |
| 69 | Entity(champ) | 0x68 | ty@tag (EntityType, 13=Champion) | r | L706/752/796 skill_cooldown()/skill2_cooldown()/ult_cooldown() 인라인(entity.rs:1774/1790/1805) m07.ll:28166/28563/28959 | 4 | OK |  |
| 70 | Entity(champ) | 0xb8 | ty@Champion.0.skill_cooldown | r | L706 (entity.rs:1776) m07.ll:28173 | 4 | OK |  |
| 71 | Entity(champ) | 0xc0 | ty@Champion.0.skill2_cooldown | r | L752 (entity.rs:1791) m07.ll:28570 | 4 | OK |  |
| 72 | Entity(champ) | 0xc8 | ty@Champion.0.ult_cooldown | r | L796 (entity.rs:1806) m07.ll:28966 | 4 | OK |  |
| 73 | Entity(champ) | 0x5c8 | level | r | L752 skill2_effect()(entity.rs:1692: level>2 아니면 None) m07.ll:28209~28213 · L796 ult_effect()(entity.rs:1701: level>4 아니면 None) m07.ll:28606~28608 | 4 | OK |  |
| 74 | Entity(champ) | 0x530 | skill2_effect@tag(i32 니치, -1=None) | r | L752: select(level>2, champ+0x500, @anon.31=None 정적) 후 +0x30 → 0x530 (m07.ll:28215~28217) | 4 | OK |  |
| 75 | Entity(champ) | 0x568 | ult_effect@tag(i32 니치, -1=None) | r | L796: select(level>4, champ+0x538, @anon.31) 후 +0x30 → 0x568 (m07.ll:28610~28612) | 4 | OK |  |
| 76 | Entity(champ) | 0x470 | stat_buff_cached.radius_mult(i32) | r | Entity::radius() 인라인(entity.rs:1509~1515): mult==0 ? radius : radius*(100+mult)/100. gep 은 배치 A(%342, m07.ll:25513), load 는 이 범위(L712/735/758/779/802/823/885/910/936) | 4 | OK |  |
| 77 | Entity(champ) | 0x680 | radius(usize) | r | 위와 짝(%343, m07.ll:25514). load 는 이 범위 | 4 | OK |  |
| 78 | Entity(champ) | 0x5c0 | id | r | L862 is_visible(enemy_team, champ.id) 인자. gep 은 배치 A(%231, m07.ll:25283), load 는 L862(m07.ll:29501) | 4 | OK |  |
| 79 | Entity(캐시 튜플의 .2 = 적/아군 엔티티) | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius() 인라인 (m07.ll:28320 등) | 4 | OK |  |
| 80 | Entity(캐시 튜플의 .2) | 0x680 | radius | r | Entity::radius() 인라인 (m07.ll:28346 등) | 4 | OK |  |
| 81 | bumpalo Vec<(EntityPositioningCache,&PlayerState,&Entity,u64)> (%95=적 5명 캐시 L407 / %93=아군 5명 캐시 L425, 배치 A 생성) | 0x0 | ptr | r | bumpalo Vec 헤더: +0 ptr · +8 bump · +16 cap · +24 len (m07.ll:28080) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 82 | bumpalo Vec<…> | 0x18 | len | r | 원소 stride 448 (= 424 + 8 + 8 + 8, `mul nuw nsw %len, 448`) m07.ll:28082/28091 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 83 | 튜플 (EntityPositioningCache,&PlayerState,&Entity,u64) 448B | 0x0 | .0.attack_ratio | r | L866 (적 위협 루프) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 84 | 튜플 448B | 0x8 | .0.skill_ratio | r | L879 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 85 | 튜플 448B | 0x10 | .0.skill2_ratio | r | L904 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 86 | 튜플 448B | 0x18 | .0.ult_ratio | r | L930 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 87 | 튜플 448B | 0x20 | .0.attacked_ratio | r | L696 (gain 루프: 적이 나에게 공격당할 때 ratio — 필드명은 tcxdict, 방향 의미는 score_parameter 소관) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 88 | 튜플 448B | 0x28 | .0.skilled_ratio | r | L708(적)/L729(아군) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 89 | 튜플 448B | 0x30 | .0.skilled2_ratio | r | L754(적)/L775(아군) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 90 | 튜플 448B | 0x38 | .0.ulted_ratio | r | L798(적)/L819(아군) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 91 | 튜플 448B | 0x40 | .0.skill_range | r | L880 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 92 | 튜플 448B | 0x48 | .0.skill2_range | r | L905 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 93 | 튜플 448B | 0x50 | .0.ult_range | r | L931 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 94 | 튜플 448B | 0x58 | .0.skilled_range | r | L710/L733 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 95 | 튜플 448B | 0x60 | .0.skilled2_range | r | L756/L777 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 96 | 튜플 448B | 0x68 | .0.ulted_range | r | L800/L821 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 97 | 튜플 448B | 0x70 | .0.attack_range_sq | r | L867 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 98 | 튜플 448B | 0x78 | .0.attack_range_ext_sq | r | L870 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 99 | 튜플 448B | 0x80 | .0.skill_range_sq | r | L885/L894 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 100 | 튜플 448B | 0x88 | .0.skill_range_ext_sq | r | L882/L891/L897 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 101 | 튜플 448B | 0x90 | .0.skill_range_half_sq | r | L887 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 102 | 튜플 448B | 0x98 | .0.skill2_range_sq | r | L910/L919 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 103 | 튜플 448B | 0xa0 | .0.skill2_range_ext_sq | r | L907/L916/L923 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 104 | 튜플 448B | 0xa8 | .0.skill2_range_half_sq | r | L912 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 105 | 튜플 448B | 0xb0 | .0.ult_range_sq | r | L936/L945 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 106 | 튜플 448B | 0xb8 | .0.ult_range_ext_sq | r | L933/L942/L949 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 107 | 튜플 448B | 0xc0 | .0.ult_range_half_sq | r | L938 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 108 | 튜플 448B | 0xc8 | .0.attacked_range_sq | r | L698 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 109 | 튜플 448B | 0xd0 | .0.attacked_range_ext_sq | r | L700 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 110 | 튜플 448B | 0xd8 | .0.skilled_range_sq | r | L711/721/734/744 (hoist 된 비교 %1419/%1512) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 111 | 튜플 448B | 0xe0 | .0.skilled_range_ext_sq | r | L718/723/741/746 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 112 | 튜플 448B | 0xe8 | .0.skilled_range_half_sq | r | L714/737 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 113 | 튜플 448B | 0xf0 | .0.skilled2_range_sq | r | L757/767/778/788 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 114 | 튜플 448B | 0xf8 | .0.skilled2_range_ext_sq | r | L764/769/785/790 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 115 | 튜플 448B | 0x100 | .0.skilled2_range_half_sq | r | L760/781 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 116 | 튜플 448B | 0x108 | .0.ulted_range_sq | r | L801/811/822/832 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 117 | 튜플 448B | 0x110 | .0.ulted_range_ext_sq | r | L808/813/829/834 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 118 | 튜플 448B | 0x118 | .0.ulted_range_half_sq | r | L804/825 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 119 | 튜플 448B | 0x1a0 | .0.is_skill_linear_move(bool) | r | L881 (적 캐시의 플래그 = 그 적 스킬이 직선이동형) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 120 | 튜플 448B | 0x1a1 | .0.is_skill2_linear_move | r | L906 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 121 | 튜플 448B | 0x1a2 | .0.is_ult_linear_move | r | L932 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 122 | 튜플 448B | 0x1b0 | .2 = &Entity(그 캐시의 엔티티) | r | IR 타입 `{ {…424B…}, ptr, ptr, i64 }` (m07.ll:29445) · .1(+0x1a8)=&PlayerState 는 이 범위에서 안 읽음 · +0x1b0 이 &Entity 인 근거 = 그 ptr 에서 Entity::radius(+0x680) 인라인 · slot_*_cached 인자 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 123 | 튜플 448B | 0x1b8 | .3 = dist(u64, 제곱거리) | r | DI `dist`. *_range_sq 들과 비교되므로 제곱거리 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 124 | PePlayerCtx(%91 지역 40B 사본, 배치 A 가 TLS pe_player_ctx 에서 채움) | 0x20 | my_role (BattleRole 1B: Tanker0 Initiator1 BaseAttacker2 SkillCaster3 Utility4 Assassin5) | r | L846 (m07.ll:29007~29008) | 4 | OK |  |
| 125 | PePlayerCtx | 0x21 | skill_linear_move | r | %286 (배치 A L443 load) — 이 범위 L711/734 분기에 사용 | 4 | OK |  |
| 126 | PePlayerCtx | 0x22 | skill2_linear_move | r | %289 — L757/778 | 4 | OK |  |
| 127 | PePlayerCtx | 0x23 | ult_linear_move | r | %292 — L801/822 | 4 | OK |  |
| 128 | Projectile(%1265, ProjectileIter::next 결과 — 배치 B) | 0x40 | move_type@tag (ProjectileMoveType 니치: 태그=idx+2, BouncingTarget 암묵) | r | load 는 배치 B(L642, m07.ll:32284~32290 이 `tag-2 / 7` 로 논리 idx 복원). 이 범위 L679 는 그 논리 idx `%3331 == 2`(=Periodic, 메모리태그 4) 만 비교 | 4 | OK |  |
| 129 | dyn AbstractGame vtable(%153 = OperationData.cache.game vtable, 배치 A) | 0xf8 | is_visible(self, team, entity_id) -> bool | r | divtable AbstractGame 0xf8 = ExpectedGame::AbstractGame::is_visible(일치율 98%). L862 (m07.ll:29448, 29502~29503) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 130 | game_core::simulation::prof (전역) | 0x0 | ENABLED(atomic u8) | r | L851 ProfTimer::start(prof.rs:176) — 프로파일링 게이트, 판정과 무관 | 4 | 확인불가(★모호: 동명 def_path 10개 [('game_ai::fight_c) |  |
| 131 | Option<ProfTimer> 지역(%79 / %76 / %75, 24B: +0 phase idx, +8 Instant{i64,i32}, +16 i32 tag(-1=None)) | 0x10 | tag | r | L687(%79 drop) · L850(%80→%76 memcpy 후 drop) · L851(%75 start, phase 115) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 132 | Entity(적 챔피언 ent=%2044·아군 ally=%2784/%3056·적 e=%3095) | 0x4c8 | skill_effect@Some.0 (Effect, Arc<dyn EffectType> ptr@+0/vtable@+8) | r | 961(ent) · 1039(ally, tag 만). tag@0x4f8 == -1 → None | 4 | OK |  |
| 133 | Entity | 0x4f8 | skill_effect@tag(i32, -1=None) | r | 961 · 1039 | 4 | OK |  |
| 134 | Entity | 0x500 | skill2_effect@Some.0 | r | 964(ent)·1058(ally): `level > 2` 일 때만 Some 후보(아니면 @anon…31 = tag -1 상수) | 4 | OK |  |
| 135 | Entity | 0x530 | skill2_effect@tag | r | 2단 gep: Entity+1280(0x500) → +48 (m07.ll:30257·30260 / 31310·31313) — 본문에 1328 리터럴 없음(C3 경고 사유). 964 · 1058 | 4 | OK |  |
| 136 | Entity | 0x538 | ult_effect@Some.0 | r | 967(ent)·1077(ally): `level > 4` 일 때만 | 4 | OK |  |
| 137 | Entity | 0x568 | ult_effect@tag | r | 2단 gep: Entity+1336(0x538) → +48 (30297·30300 / 31455·31458) — 리터럴 1384 없음(C3 경고 사유). 967 · 1077 | 4 | OK |  |
| 138 | Entity | 0x308 | rush_state@tag(니치 8B) / RushPenetrate.applyed_effect(Vec 헤더) | r | 978~979: raw<0 이면 tag=raw^0x8000000000000000 (None=0·Move=1·MoveToTarget=2·Rush=3) 아니면 untagged=4(RushPenetrate). Rush·RushPenetrate 두 arm 만 진입 | 4 | OK |  |
| 139 | Entity | 0x310 | rush_state@Rush.applyed_effect (Vec<(Arc<dyn EffectType>,CastingType)> 헤더 · ptr@+8=0x318 · len@+16=0x320) | r | 978 Rush arm. RushPenetrate arm 은 0x308 헤더(ptr 0x310·len 0x318). 원소 stride 24 | 4 | OK |  |
| 140 | Entity | 0x330 | rush_state@Rush.x (ex) | r | 978 | 4 | OK |  |
| 141 | Entity | 0x338 | rush_state@Rush.y (ey) | r | 978 | 4 | OK |  |
| 142 | Entity | 0x340 | rush_state@Rush.range / @RushPenetrate.x | r | 978 range · 979 ex | 4 | OK |  |
| 143 | Entity | 0x348 | rush_state@Rush.start_tick / @RushPenetrate.y | r | 978 start_tick · 979 ey | 4 | OK |  |
| 144 | Entity | 0x350 | rush_state@RushPenetrate.range | r | 979 | 4 | OK |  |
| 145 | Entity | 0x358 | rush_state@RushPenetrate.start_tick | r | 979 | 4 | OK |  |
| 146 | Entity | 0x35c | rush_state@Rush.casting_target (CastingTarget 4B) | r | 978 → CastingTarget::check 의 self | 4 | OK |  |
| 147 | Entity | 0x36c | rush_state@RushPenetrate.casting_target | r | 979 | 4 | OK |  |
| 148 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | Entity::radius() 인라인(entity.rs:1511~1515): mult==0 ? radius : radius*(mult+100)/100. champ(%122 → %342) 983·1043·1062·1081·1118 / ally 1006·1043·1062·1081 | 4 | OK |  |
| 149 | Entity | 0x680 | radius (usize) | r | radius() 인라인의 기준값(%343 = champ+0x680) | 4 | OK |  |
| 150 | Entity | 0x4c0 | attack_effect@tag(-1=None) | r | 1119 | 4 | OK |  |
| 151 | Entity | 0x628 | stat_cached.hp | r | 1121: 적 e 의 스탯 HP(현재 hp 0x670 아님) — dmg*100/max(hp,1) | 4 | OK |  |
| 152 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B) | r | 1148 positioning_accuracy(&player.info.parameter) | 4 | OK |  |
| 153 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | %118(배치 A 로드) → 1105 get_battle_role 인자 · game data/vtable(%151/%153) | 4 | OK |  |
| 154 | OperationData | 0x8 | context (&GameContext) | r | %135(배치 A 로드) → expected_damage(985)·expected_damage_target(1120)·get_battle_role(1105) 인자 · 1166 setting | 4 | OK |  |
| 155 | GameContext | 0x8 | setting (&GameSetting) | r | 1166 | 4 | OK |  |
| 156 | PePlayerCtx(%91, thread_local PE_PLAYER_CTX 복사본 — 배치 A/C) | 0x20 | my_role (BattleRole i8) | r | 847(배치 C)에서 %1785 로드. 배치 D 사용: 1001 `(my_role & 6)==2` = BaseAttacker(2)\|SkillCaster(3) · 1101 `my_role < 2` = Tanker(0)\|Initiator(1). 태그=idx(Direct) | 4 | OK |  |
| 157 | Vec<(EntityPositioningCache, &PlayerState, &Entity, u64)> 원소(448B) — near_allies_with_action(%93 bumpalo Vec, ptr@+0·len@+24=%230)·near_enemies_with_action(%95, len %200) | 0x8 | .0.skill_ratio | r | 1040 ally skill ratio | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 158 | EPC 원소 | 0x10 | .0.skill2_ratio | r | 1059 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 159 | EPC 원소 | 0x18 | .0.ult_ratio | r | 1078 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 160 | EPC 원소 | 0x40 | .0.skill_range | r | 1041 `range` | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 161 | EPC 원소 | 0x48 | .0.skill2_range | r | 1060 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 162 | EPC 원소 | 0x50 | .0.ult_range | r | 1079 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 163 | EPC 원소 | 0x80 | .0.skill_range_sq | r | 1043/1052 `dist > range_sq`(호이스트 L0, 31178) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 164 | EPC 원소 | 0x88 | .0.skill_range_ext_sq | r | 1049/1054 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 165 | EPC 원소 | 0x90 | .0.skill_range_half_sq | r | 1045 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 166 | EPC 원소 | 0x98 | .0.skill2_range_sq | r | 1062/1071 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 167 | EPC 원소 | 0xa0 | .0.skill2_range_ext_sq | r | 1068/1073 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 168 | EPC 원소 | 0xa8 | .0.skill2_range_half_sq | r | 1064 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 169 | EPC 원소 | 0xb0 | .0.ult_range_sq | r | 1081/1090 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 170 | EPC 원소 | 0xb8 | .0.ult_range_ext_sq | r | 1087/1092 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 171 | EPC 원소 | 0xc0 | .0.ult_range_half_sq | r | 1083 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 172 | EPC 원소 | 0x1a0 | .0.is_skill_linear_move (bool) | r | 1042 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 173 | EPC 원소 | 0x1a1 | .0.is_skill2_linear_move | r | 1061 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 174 | EPC 원소 | 0x1a2 | .0.is_ult_linear_move | r | 1080 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 175 | EPC 원소 | 0x1a8 | .1 &PlayerState (DI: ap) | r | 1105 get_battle_role 의 4번째 인자. 튜플 레이아웃 근거 = DICompositeType `Vec<tuple$<EntityPositioningCache,ref$<PlayerState>,ref$<Entity>,u64>>`(m07.ll:67789) · EntityPositioningCache 424B(tcxdict) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 176 | EPC 원소 | 0x1b0 | .2 &Entity | r | 874(배치 C, ent) · 1004/1039/1109 ally · 1114 e | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 177 | EPC 원소 | 0x1b8 | .3 dist (u64, 셀↔엔티티 거리²; DI `dist`/`cell_to_e_sq`) | r | 860(배치 C) · 1004 비교 기준 · 1025/1026/1038/1115 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 178 | AbstractGame vtable(%153) | 0x28 | tick (divtable) | r | %382(배치 A 490 줄 로드) → 981·1166·1168 호출 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 179 | AbstractGame vtable | 0x40 | get_game_mode (divtable) → GameMode{tag i64, ptr} | r | 1157: tag 2 = DeathMatch | 4 | 확인불가(vtable 슬롯) |  |
| 180 | EffectType vtable(Arc<dyn EffectType> +8) | 0x120 | can_move (divtable 94% 일치 · 런타임 구현체 불명) | r | 961/964/967 `e.ty.can_move()` | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 181 | EffectType vtable | 0x28 | expected_damage(&self, &GameContext, &dyn AbstractEntity) -> (usize,usize) | r | 985: 돌진 applyed_effect 각 (Arc,CastingType) 의 (ad, ap) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 182 | Arc<dyn EffectType> ArcInner | 0x10 | 데이터 시작(align 계산: (vtable.align-1)&~15 + 16) | r | sync.rs:2445 deref 인라인 — Arc 데이터 포인터 산술(2427/2445 줄) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 183 | @gc::simulation::prof::ENABLED | 0x0 | prof 계측 on/off (atomic i8) | r | 1035 `_t = ProfTimer(116)` 생성 게이트. 관측 전용 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 184 | PositioningScore(sret %0) | 0x0 | risk | w | 조기반환 직접 store. 그 밖의 경로는 SSA 누적 후 배치 D 가 store \| (배치 B) SSA 캐리(%640→%3675 · %551→%948/%3634 · %1126→… · %1262→%3490) — sret 실제 store 는 배치 D \| (배치 D) m07.ll:31996 · 32157 | 4 | OK | 9999 (381, walls 셀) / 0 (375 default memset 50B) |
| 185 | PositioningScore(sret %0) | 0x8 | tower_risk…on_periodic_trajectory (0x8~0x31) | w | 0x32~0x37 은 미기록(패딩) | 4 | 오귀속(사전은 다른 필드를 준다) | 0 (memset 42B @381 · memset 50B @375) |
| 186 | bumpalo Vec near_enemies_with_action (alloca %95, 32B: ptr@0 alloc@8 cap@0x10 len@0x18) | 0x0 -> elem[len] 448B |  | w | push 34146~34183 · 뒤 배치(타워 필터·507 map)가 읽음 | 4 | 확인불가(tcx 사전에 타입 없음) | (EntityPositioningCache 424B @0, &PlayerState eplayer @424, &Entity e @432, dist u64 @440) |
| 187 | bumpalo Vec near_allies_with_action (alloca %93) | 0x0 -> elem[len] 448B |  | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | 동형 (33880~33917) |
| 188 | TLS (tls 절 참조) | 0x0 | PE_CAND_MASKS / EPC_CACHE / PE_PLAYER_CTX / ATTACK_DMG_CACHE / TOWER_MINION_CNT_CACHE | w | 이 함수 본체의 직접 store 가 아니라 인라인 헬퍼의 `LocalKey::with` 클로저 안. POS_EVAL_CACHE 저장은 호출자 | 4 | 확인불가(tcx 사전에 타입 없음) | miss 시 각 헬퍼 클로저가 채움 |
| 189 | PositioningScore(sret %0) | 0x8 | tower_risk | w | L540 가지(타워가 나를 물고 있음)에서는 불변(%3866 phi [%639,%3788]) | 4 | OK | += risk (L565 적 타워, nearest_enemy≠나 가지에서만) |
| 190 | PositioningScore(sret %0) | 0x10 | gain | w | SSA %638→%3925 | 4 | OK | += (ratio as u8)/3 (L522 아군 타워 사거리 안 적 존재 · 내가 그 타워 사거리 안) |
| 191 | PositioningScore(sret %0) | 0x30 | on_trajectory | w | score[48] | 4 | OK | 0 (L638 초기화, 설정은 배치 C L678/684) |
| 192 | PositioningScore(sret %0) | 0x31 | on_periodic_trajectory | w | score[49] | 4 | OK | 0 (L638 초기화, 설정은 배치 C) |
| 193 | 지역 tower_well_risk (i64, 배치 D 소비) | - | tower_well_risk | w | sret 아님 — DI 지역변수 | 4 | 확인불가(오프셋 파싱 실패) | += 타워 risk (L540/L565) |
| 194 | RefCell<AttackDamageCache> (TLS) | 0x8 | map insert / clear | w | 인라인 expected_attack_damage_cached 경유(클로저 m00.ll:75730 본체) — 이 함수가 작성자 역할도 함 | 4 | 확인불가(tcx 사전에 타입 없음) | miss 시 (attacker.id,target.id)→damage insert · 에포크 불일치 시 clear + seed/tick 갱신 |
| 195 | prof 전역(원자) | - | PHASE_NANOS[phase] / PHASE_CALLS[phase] | w | prof::ENABLED 게이트 텔레메트리 — 판정 무관 | 4 | 확인불가(오프셋 파싱 실패) | ProfTimer drop 시 fetch_add (L605 _t_peu2 · phase 114 = L606 _t_peu3 · 73 = L638 _t_proj) |
| 196 | (sret %0) | 0x0 | — | w | ★이 범위에는 %0 로의 store 가 0건. score 필드는 SSA 로만 갱신(아래 logic 의 score.* 는 DI `score[N..+8]` 에 붙은 phi/add 를 뜻함). 실제 메모리 store 는 다른 배치 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 없음 |
| 197 | Option<ProfTimer> 지역 %75 | 0x0 | phase | w | L851 (m07.ll:29463) prof::ENABLED 일 때만 · +8 Instant::now() 초 · +16 nanos(또는 -1=None) | 4 | 확인불가(tcx 사전에 타입 없음) | 115 |
| 198 | prof::PHASE_NANOS[132] (전역 atomic) | 0x0 | [phase] | w | L687(m07.ll:27986) · L850(m07.ll:29064) ProfTimer drop 인라인 — 텔레메트리, 판정 무관. idx<132 아니면 panic_bounds_check | 4 | 확인불가(tcx 사전에 타입 없음) | += elapsed ns |
| 199 | prof::PHASE_CALLS[132] (전역 atomic) | 0x0 | [phase] | w | L687(m07.ll:27990) · L850(m07.ll:29068) | 4 | 확인불가(tcx 사전에 타입 없음) | += 1 |
| 200 | PositioningScore | 0x8 | tower_risk | w | 31998 | 4 | OK | %3203 (=%550 배치 B 399 줄값, LaneSafe 면 *3/2) |
| 201 | PositioningScore | 0x10 | gain | w | 32000 | 4 | OK | %3202 (아군 스킬 max_ratio 합 + 탱커 enabled + trade_net, Aggressive 면 *120/100) |
| 202 | PositioningScore | 0x18 | gain_me | w | 32002 | 4 | OK | %1782 (배치 C 693/840 max_ratio 그대로) |
| 203 | PositioningScore | 0x20 | adjust | w | 32004 — 이 함수는 adjust 를 절대 채우지 않는다 | 4 | OK | 0 (상수) |
| 204 | PositioningScore | 0x28 | unseen_champ_threat | w | 32006 | 4 | OK | %1996 (적 루프 1015 누적: 미시야 셀이면 risk - risk/2) |
| 205 | PositioningScore | 0x30 | on_trajectory (i8) | w | 32008 | 4 | OK | %1995 (적 루프 980 에서 돌진 궤도 판정 통과 시 1) |
| 206 | PositioningScore | 0x31 | on_periodic_trajectory (i8) | w | 32010 | 4 | OK | %1260 (배치 B 639 루프 산출) |
| 207 | @gc::simulation::prof::PHASE_NANOS[phase] | 0x0 -> [phase 115·116]+0 (stride 8, bounds 132) | prof 누적 ns (atomicrmw add) | w | 1034(phase 115 `_t` drop)·1141(phase 116)·1181(%98 phase = 배치 A 373 줄). ENABLED 일 때만. 관측 전용 | 4 | 확인불가(tcx 사전에 타입 없음) | Instant::elapsed 의 ns |
| 208 | @gc::simulation::prof::PHASE_CALLS[phase] | 0x0 -> [phase]+0 | prof 호출 수 (atomicrmw add 1) | w | 동상 | 4 | 확인불가(tcx 사전에 타입 없음) | 1 |

**`consts` 상수 108건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 48 | 373 | 산출값 | ProfTimer phase id(함수 전체 _t) — 계측용, 판정 아님 | 4 |  |
| 1 | 112 | 406 | 산출값 | ProfTimer phase 112 (_t_peu1: 406~447 후보 수집 구간) | 4 |  |
| 2 | 113 | 448 | 산출값 | ProfTimer phase 113 (_t_peu2: 448~ 정글/타워 구간) | 4 |  |
| 3 | 32000 | 378 | 계수 | 셀 크기 — xi = x/32000, yi = y/32000 (좌표변환). effect.rs:26 Effect::range 의 +32000(1셀 여유)도 같은 값(464) | 4 |  |
| 4 | 29 | 378 | 인덱스 | clamp 상한 = 30×30 그리드 마지막 인덱스(umin) — 좌표변환 | 4 |  |
| 5 | 9999 | 381 | 산출값 | 벽 셀(walls[yi][xi]!=0) 위험 = 9999 즉시 반환 / 397: is_enemy_well_danger 면 risk=tower_risk=tower_well_risk=9999 (아니면 0) | 4 |  |
| 6 | 4294967296 | 384 | 계수 | 2^32 — inv_hp_q32 = 2^32 / max(champ.hp,1) (Q32 역수, 이후 ratio = (inv_hp_q32*100*damage)>>32 = 100*damage/hp) | 4 |  |
| 7 | 1 | 384 | 임계 | max(hp,1) 0 나눗셈 가드(umax) · 393: enemy_team = 1 − team · 409/427 의 `shl i8 1, i` 는 마스크 비트 1<<i (시프트량 아님, 접힌 배수 없음 — qcspec shl 경고 사유) | 4 |  |
| 8 | 5 | 408 | 임계 | 팀당 플레이어 수 — 적 루프(408)·아군 루프(426) 상한 · mask 비트 i | 4 |  |
| 9 | 40000000000 | 414 | 임계 | 200000² (6.25셀) — cell_dist_sq(e) > 이 값이면 적(414)/아군(432) 후보 제외 | 4 |  |
| 10 | 22499999999 | 451 | 임계 | 150000²−1 — 정글몹 jd > 이 값(즉 jd ≥ 150000², 4.69셀 이상)이면 건너뜀 | 4 |  |
| 11 | 4 | 454 | 태그 | EntityType 메모리태그 4 = Jungle (tcxdict --enum EntityType, Direct) | 3 |  |
| 12 | 100 | 458 | 계수 | ratio = min(100*damage/hp, 150) 의 백분율 계수 (mul nuw %149,100 은 25505 에 호이스팅) · entity.rs:1515 radius*(mult+100)/100 도 100 | 4 |  |
| 13 | 150 | 458 | 임계 | HP% 위험 비율 상한(umin i128 150) — 정글(458)·에픽(475) 공통 | 4 |  |
| 14 | 3 | 480 | 태그 | 에픽이 남을 노릴 때 위험 = ratio/3 (udiv i8 3) | 4 |  |
| 15 | 22500000000 | 493 | 임계 | 150000² (4.69셀) — 타워 필터: cell_dist_sq(t) < 이 값이면 근접 타워(493 적 / 496 아군) | 4 |  |
| 16 | 4900000000 | 494 | 임계 | 70000² (2.19셀) — 근접 적/아군 챔피언 e 가 타워에서 e.distance_sq(t) < 이 값이면 그 타워도 포함(494/497). 비포화 mul | 4 |  |
| 17 | 2 | 374 | 센티널 | 팀 수 bounds(team<2) · Option 니치 태그 2(None) 다수 | 4 |  |
| 18 | 50 | 375 | 길이 | PositioningScore::default() memset 길이 0x32 (패딩 6B 제외) | 4 |  |
| 19 | 42 | 381 | 길이 | risk=9999 뒤 0x8~0x32 를 0 으로 memset 하는 길이 | 4 |  |
| 20 | 6 | 507 | 임계 | iter_towers_without_nexus 의 앞 절 = [Option<&Entity>;6] (top/top2/mid/mid2/bottom/bottom2) 배열 이터레이터 상한(index_range 0..6, 26274·26318) | 4 |  |
| 21 | 150 | 387 | 임계 | pct_q32 상한 — 예상 피해가 내 현재 HP 의 150% 를 넘어도 150 으로 절단(umin i128). position_eval_pct(score_parameter.rs:56, L656) 도 같은 150 상한 | 4 |  |
| 22 | 32 | 387 | 계수 | pct_q32 의 Q32 고정소수 시프트(lshr 32) — inv_hp_q32=2^32/hp 와 짝, 임계 아님 | 4 |  |
| 23 | 3 | 522 | 계수 | 아군 타워 gain 가산 = ratio/3 (udiv i8) · L540/548 소커=나 → ratio/3 · L598 다른 대상을 문 미니언 ratio/3 · L631 range_ext 밴드 ratio/3 · L550/562 coef*3/100 계수 | 4 |  |
| 24 | 18000 | 532 | 오프셋가감 | 적 타워 is_in_range_ex 마진(좌표 단위, 0.56셀) · L651 투사체 is_in_orbit 반경 = champ.radius()+18000 | 4 |  |
| 25 | 100 | 545 | 계수 | coef: 타워 사거리 절반 안이면 100 (밖 50) · radius()*(100+mult)/100 · pct 계산 ×100 | 4 |  |
| 26 | 50 | 545 | 산출값 | coef: dist ≥ (range/2)² 이면 50 | 4 |  |
| 27 | 2 | 557 | 임계 | cnt < 2 → ratio 그대로 · cnt == 2 → 2/3 배 (L560 `ratio*2/3*coef/100`, shl 1 로 접힘: 실제 `*2`) · L598/555/628 ratio/2 · L575 tps/2(lshr 1) | 4 |  |
| 28 | 30 | 573 | 계수 | is_line_phase(runner.rs:399·setting.rs:703): tick < first_spawn_tick − tps*30 (에픽 스폰 30초 전까지 라인 페이즈) | 4 |  |
| 29 | 9 | 573 | 센티널 | PositionEvalPurpose 태그 9 = 니치 빈칸 (llvm.assume purpose≠9) · enemy_minion_wave_is_dangerous: hp_pct<26 && damage_pct>9 | 4 |  |
| 30 | 10 | 573 | 태그 | purpose 태그 10 = LaneSafe (L24/L13 switch) · L641 "knight_ult" 길이 10 · Eagle 태그 10 | 4 |  |
| 31 | 8 | 573 | 태그 | purpose 태그 8 = Lane (L13 은 `(purpose & 14) == 8` 로 8·10 을 한 번에) · Total 튜토리얼 8 | 4 |  |
| 32 | 14 | 576 | 계수 | L13 마스크 `purpose & 14 == 8` = {8 Lane, 10 LaneSafe} (9 는 assume 로 배제) | 4 |  |
| 33 | 140 | 576 | 임계 | position_eval_minion_wave_risk_score(L7): risk = min(pct,140) — 미니언 웨이브 위험 상한(pct 자체 150 상한은 접혀 사라짐) | 4 |  |
| 34 | 49 | 576 | 임계 | enemy_minion_wave_is_dangerous(minion_wave_risk.rs:71): damage_pct > 49 → 위험 | 4 |  |
| 35 | 66 | 576 | 임계 | is_dangerous:73: hp_pct < 66 && damage_pct > 29 | 4 |  |
| 36 | 29 | 576 | 임계 | is_dangerous:73 짝 | 4 |  |
| 37 | 41 | 576 | 임계 | is_dangerous:74: hp_pct < 41 && damage_pct > 17 | 4 |  |
| 38 | 17 | 576 | 임계 | is_dangerous:74 짝 | 4 |  |
| 39 | 26 | 576 | 임계 | is_dangerous:75: hp_pct < 26 && damage_pct > 9 | 4 |  |
| 40 | 4 | 576 | 임계 | L15: 라인 페이즈·라인 목적·비위험이면 risk = min(risk/4, 18) (sdiv 4) | 4 |  |
| 41 | 18 | 576 | 임계 | L15 상한 18 | 4 |  |
| 42 | 22500000000 | 581 | 임계 | 150000² — L581 미니언 필터(< , aux 클로저) · L608 others 거리 게이트(> 면 continue) = 4.69셀 | 4 |  |
| 43 | 4096000001 | 599 | 임계 | 64000²+1 — 미니언 dist_sq < 이면 risk 가산 (=2셀 이내) | 4 |  |
| 44 | 1 | 582 | 태그 | EntityType::Minion 태그 1 · Option Some 태그 1 · v47_soaker.0 == 1(bool) · umax(hp,1). ⚠본문의 `shl i16 %3849, 1`(L560 ratio*2)·`lshr 1`(L575 tps/2, L555/598/628 ratio/2) 은 시프트량 — 소스값 2 는 value 2 항목에 folded 기록 | 4 |  |
| 45 | 7 | 616 | 태그 | EntityType::Ghoul 태그 7 (Bear 9 와 같은 nearest_enemy 오프셋 0x88/0x90) · TutorialType::Line 7 · BouncingTarget 논리 idx 7 | 4 |  |
| 46 | 32000 | 615 | 계수 | range_ext = range + 1셀(32000) — others 의 바깥 밴드 | 4 |  |
| 47 | 62499999999 | 640 | 임계 | 250000²−1 — 투사체 dist_sq > 이면 continue (7.8셀 밖 투사체 무시) | 4 |  |
| 48 | 5 | 642 | 임계 | ProjectileMoveType 논리 idx 4 Target / 5 TargetSplash = is_targeting → continue · TutorialType::MidBottom 5 | 4 |  |
| 49 | 114 | 606 | 산출값 | ProfTimer phase id (_t_peu3, L606~) — 텔레메트리, 판정 무관 | 4 |  |
| 50 | 73 | 638 | 산출값 | ProfTimer phase id (_t_proj, L638~) — 텔레메트리 | 4 |  |
| 51 | 448 | 514 | 미상 | (EntityPositioningCache,&PlayerState,&Entity,u64) 원소 stride (0x1c0) — 배열 stride, 임계 아님 | 4 |  |
| 52 | 2 | 679 | 태그 | ProjectileMoveType 논리 idx 2 = Periodic(메모리태그 4). `icmp eq %3331, 2`(m07.ll:32603). 주기형(장판) 궤도면 on_periodic_trajectory, 아니면 on_trajectory | 4 |  |
| 53 | -800000 | 689 | 계수 | is_in_well_damage(game.rs:5654) 인라인: 팀0 우물 사각 y∈[800000,960000] / 팀1 x∈[800000,960000] 을 `x-800000 <u 160001` 로 접음(m07.ll:28020,28028) | 4 |  |
| 54 | 160001 | 689 | 임계 | 우물 사각 변 길이 160000 + 1(포함비교) (m07.ll:28021,28029,28041) | 4 |  |
| 55 | 64001 | 689 | 임계 | 우물 사각 좁은 변 64000 + 1 (m07.ll:28022,28027,28035,28043) | 4 |  |
| 56 | -896000 | 689 | 계수 | 두 번째 우물 사각 [896000,960000] (m07.ll:28034,28042). 두 사각의 합 = 맵 구석 L자 우물 영역(팀0: x≤64000∧y≥800000 ∪ x≤160000∧y≥896000 / 팀1: 대칭) | 4 |  |
| 57 | 150 | 690 | 임계 | pct_q32(position_eval.rs:386) 상한: min(well_damage*100*inv_hp_q32 >> 32, 150) = 우물 데미지의 HP% 를 150 에서 캡 (m07.ll:28067) | 4 |  |
| 58 | 32 | 690 | 계수 | `lshr i128 …, 32` — inv_hp_q32(=2^32/max(hp,1), 배치 A L384) 의 Q32 고정소수 정규화. 100 은 배치 A 가 %334=inv_hp_q32*100 로 호이스트(m07.ll:25505) | 4 |  |
| 59 | 0 | 693 | 임계 | `let mut max_ratio = 0`(i64) | 4 |  |
| 60 | -1 | 706 | 센티널 | Option 니치 태그 None: champ.skill_effect@0x4f8 (i32) == -1 이면 스킬 없음 → 스킬 gain 루프 건너뜀 (m07.ll:28111). 752/796 도 동일(0x530/0x568) | 4 |  |
| 61 | 13 | 706 | 태그 | EntityType::Champion 태그(tcxdict --enum EntityType). Entity::skill_cooldown() 인라인: Champion 이면 +0xb8, 아니면 0 (m07.ll:28168) | 3 |  |
| 62 | 181 | 706 | 임계 | 쿨타임 게이트: skill_cooldown < 181 틱(60tps 기준 ≈3.0초) 일 때만 그 슬롯의 gain 을 센다 (m07.ll:28175). skill2(L752, +0xc0)·ult(L796, +0xc8) 도 같은 181 | 4 |  |
| 63 | 448 | 695 | 미상 | 캐시 튜플 stride(424+8+8+8) — 배열 stride 이지만 원소 타입 확정 근거로 등록(m07.ll:28091) | 4 |  |
| 64 | 80000 | 712 | 계수 | 직선이동 스킬 사거리 여유: `range > e.radius() + 80000 + champ.radius()` (2.5셀). 스킬 사거리가 두 반지름+80000 을 넘지 않으면(짧은 대시) 사거리 안 = 풀 ratio (m07.ll:28362). L735/758/779/802/823(gain)·L885/910/936(위협) 동일 | 4 |  |
| 65 | 100 | 712 | 계수 | Entity::radius() 인라인: radius*(100+radius_mult)/100 (entity.rs:1515) m07.ll:28355~28357 | 4 |  |
| 66 | 2 | 717 | 임계 | ratio/2 — 직선이동형: half_sq < dist ≤ range_sq 구간 (sdiv m07.ll:28400). 비직선형(L724/747/770/791/814/835): range_sq < dist ≤ ext_sq 구간도 /2. 위협 루프 L871(attack ext)·L890/915/941(linear sq)·L898/924/950(nonlinear ext) 동일 | 4 |  |
| 67 | 3 | 719 | 태그 | ratio/3 — 직선이동형: range_sq < dist ≤ ext_sq 구간 (sdiv m07.ll:28339). L742/765/786/809/830 · 위협 L892/917/943 동일. L874 `attack_ratio/3`(udiv) 는 적 is_block_attack 페널티, L956~958 `/3`(udiv) 은 적 is_block_skill 페널티 | 4 |  |
| 68 | 4 | 796 | 임계 | Entity::ult_effect()(entity.rs:1701) 인라인: level > 4 아니면 None (m07.ll:28606). skill2_effect 는 level > 2 (L752, m07.ll:28211) | 4 |  |
| 69 | 6 | 847 | 임계 | `(my_role & 6) == 2` ⇔ my_role ∈ {BaseAttacker(2), SkillCaster(3)} — 결과 %1787 은 배치 D L1001 에서 소비 (m07.ll:29010~29011) | 4 |  |
| 70 | 115 | 851 | 산출값 | ProfTimer phase 번호(prof 텔레메트리, ENABLED 일 때만) m07.ll:29463 | 4 |  |
| 71 | 132 | 687 | 길이 | PHASE_NANOS/PHASE_CALLS 배열 길이(bounds check) m07.ll:27956/29034 | 4 |  |
| 72 | 1000000000 | 687 | 임계 | Instant→ns 변환(secs*1e9+nanos) m07.ll:27978 | 4 |  |
| 73 | 10000000000 | 862 | 임계 | 100000² — 적 위협 고려 조건 `dist < 100000²(≈3.125셀) \|\| game.is_visible(enemy_team, champ.id)`: 아주 가깝거나 적 팀 시야에 내가 보일 때만 그 적을 위협원으로 계산 (m07.ll:29517~29518) | 4 |  |
| 74 | 1 | 869 | 태그 | Option<usize> Some 태그: `slot_cc_time_cached(...).0 == 1` ⇔ is_some → has_cc (m07.ll:29551 등 11곳). shl 시프트량 아님(본문의 shl 은 배치 A/B 범위) | 4 |  |
| 75 | 0 | 868 | 임계 | `max(ratio, 0)` 하한 — 위협 ratio 는 음수 절단(smax …, 0). gain 루프의 max 는 0 초기 max_ratio 와 비교 | 4 |  |
| 76 | 288 | 961 | 인덱스 | EffectType vtable 슬롯 오프셋 0x120 = can_move (배열 인덱스류지만 판정 게이트라 기록). 961/964/967 공통 | 4 |  |
| 77 | 3 | 961 | 태그 | `udiv i64 ratio, 3` — 해당 스킬 effect.ty.can_move() 이면 skill/skill2/ult ratio 를 1/3 로(961·964·968). 같은 리터럴 3 이 978 switch case(Rush 태그)·1050/1069/1088 `sdiv 3`·1160 `sdiv 3`·1142 `mul 3`(LaneSafe 3/2) 에도 쓰인다 | 4 |  |
| 78 | 2 | 964 | 태그 | `icmp ugt level, 2` — level>2 여야 skill2_effect 를 본다(entity.rs:1693 인라인, 964·1058). 같은 2 가 1012/1055/1048/1067/1074/1086/1093 `sdiv 2`, 1142 `sdiv 2`(3/2), 1157 GameMode::DeathMatch 태그, 1001/1105 `(role&6)==2`, 1101 `role<2`, 1152 `version>1`(ugt 1) 의 짝 등에 재사용 | 4 |  |
| 79 | 4 | 967 | 임계 | `icmp ugt level, 4` — level>4 여야 ult_effect(entity.rs:1701, 967·1077). 978 switch: RushState untagged=4(RushPenetrate). 1030 `min(cell_e, 4)` 고립 가산 상한 | 4 |  |
| 80 | -1 | 961 | 센티널 | Option<Effect> 니치 태그 None (i32 -1 @+48): 961/964/967/1039/1058/1077/1119. Option<ProfTimer> None 도 i32 -1(1034/1141/1181) | 4 |  |
| 81 | 1 | 972 | 임계 | 972~975 `lshr i64 ratio, 1` = ratio/2 — Entity::block_input(ent) 이면 attack/skill/skill2/ult ratio 전부 반감(비부호 시프트). 1008 `lshr risk, 1` = risk/2 (아군 몸빵). 그 밖에 1175 `or 1`(2*spread+1) · 1121/1166 `umax 1` · 1153/1152 `ugt %1, 1` | 4 | 2 |
| 82 | -9223372036854775808 | 979 | 센티널 | RushState 니치 시작(0x8000000000000000): raw ^ niche_start = 논리 태그(None0/Move1/MoveToTarget2/Rush3). raw≥0 이면 untagged RushPenetrate(=4). tcxdict --enum RushState | 3 |  |
| 83 | 20000 | 983 | 계수 | 돌진 궤도 허용폭: dist_to_line_segment(셀, ent→돌진목적지) <= rush.range + 20000 + champ.radius() 이면 궤도 위(30480) | 4 |  |
| 84 | 100 | 983 | 계수 | Entity::radius() 인라인 `radius*(mult+100)/100`(983·1006·1043·1062·1081·1118). 1030 `/100`(25%·cell_e), 1121 `dmg*100/hp`(HP% 환산), 1142 `/100`(120%). 989 는 %335 = inv_hp_q32*100 (배치 A 사전계산) | 4 |  |
| 85 | 150 | 989 | 임계 | rush_ratio 상한: min((inv_hp_q32*100*dmg)>>32, 150) = 돌진 피해의 내 HP% 를 150 으로 클램프(pct_q32 386~387 인라인, umin.i128) | 4 |  |
| 86 | 32 | 989 | 계수 | pct_q32 `lshr i128, 32` — Q32 고정소수 역수(inv_hp_q32 = 2^32/max(hp,1), 배치 A 384) 환산 | 4 |  |
| 87 | 28000 | 1006 | 계수 | 몸빵 판정 폭: 아군이 셀→적 선분에서 ally.radius()+28000 이내(1006) / 탱커 진입가치: 셀이 아군→적 선분에서 champ.radius()+28000 이내(1118) | 4 |  |
| 88 | 14400000001 | 1025 | 임계 | 120000² + 1 (제곱거리 `ult` 비교 = dist ≤ 120000 = 3.75셀): 1025 cell_a(아군 with action 수) · 1026 cell_e(적 with action 수) 필터 | 4 |  |
| 89 | 14400000000 | 1109 | 임계 | 120000² — 탱커 진입가치: 셀↔후방 아군 거리²(uadd.sat) > 120000² 이면 제외 | 4 |  |
| 90 | 25 | 1030 | 계수 | 고립 가산: score.risk += champ_threat_risk * 25 * min(cell_e,4) / 100 — 120000 안에 행동 중 아군 0·적 ≥2 일 때 적 1명당 +25% (최대 +100%) | 4 |  |
| 91 | 80000 | 1043 | 계수 | 선형이동 스킬 근접 판정: skill_range > ally.radius()+80000+champ.radius() 이면 half_sq 구간 분기, 아니면 dist≤range_sq 만으로 ratio 전액(1043·1062·1081) | 4 |  |
| 92 | 6 | 1001 | 임계 | `and i8 role, 6` == 2 → BattleRole ∈ {BaseAttacker(2), SkillCaster(3)} (1001 my_role · 1105 아군 role). 1166 `tps*6` 6초 버킷 | 4 |  |
| 93 | 132 | 1034 | 길이 | prof PHASE_NANOS/PHASE_CALLS 배열 길이(bounds check) — 관측 전용 | 4 |  |
| 94 | 1000000000 | 1034 | 임계 | Instant::elapsed → ns 환산(secs*1e9+nanos) — 관측 전용 | 4 |  |
| 95 | 116 | 1035 | 산출값 | ProfTimer phase id: 1035 `_t = ProfTimer::new(116)`(아군 gain 루프 구간). 115 는 배치 C 851 의 적 루프 구간 | 4 |  |
| 96 | 9 | 1142 | 센티널 | `assume purpose != 9` — PositionEvalPurpose 태그 9 는 무효(니치 2..12 중 빈 값) | 4 |  |
| 97 | -2 | 1142 | 임계 | idx = purpose - 2 (purpose>1 일 때) = 논리 인덱스. 1152 `icmp ult (idx-3), -2`(=254) → idx ∉ {1 RunAway, 2 Recall} | 4 |  |
| 98 | 7 | 1142 | 태그 | switch case 7 = LineStyle(untagged, purpose 바이트 0/1 → idx 7 로 사상) → 33/36 줄 보정. 8 = LaneSafe → 39/40 줄 | 4 |  |
| 99 | 8 | 1142 | 태그 | switch case 8 = LaneSafe: risk = risk*3/2, tower_risk = tower_risk*3/2 (position_eval.rs:39~40) | 4 |  |
| 100 | 120 | 1142 | 계수 | LineStyle 보정 120%: Defensive(1) → risk*120/100 (33줄) · Aggressive(0) → gain*120/100 (36줄) | 4 |  |
| 101 | -3 | 1152 | 미상 | idx-3 (RunAway/Recall 면제 판정용 wrap 비교) | 4 |  |
| 102 | 1000 | 1152 | 임계 | positioning_accuracy < 1000 이면 노이즈(1152/1153). 1158 spread = 1000-acc(DeathMatch). 1175 factor = h%(2s+1) - s + 1000 (=1000±spread ‰). 1176 risk = tower_risk + base_risk*factor/1000 | 4 |  |
| 103 | 2000 | 1160 | 계수 | 비-데스매치 spread = (2000 - 2*acc)/3 = 2/3·(1000-acc) | 4 |  |
| 104 | -7046029254386353131 | 1171 | 계수 | 0x9E3779B97F4A7C15 (황금비 곱, splitmix/fxhash 류) — h = ((((id^t_salt)*K ^ xi)*K ^ yi)*K); 결정론 노이즈 해시(1171~1173) | 4 |  |
| 105 | 31 | 1174 | 계수 | h ^= h >> 31 최종 믹스 | 4 |  |
| 106 | 448 | 1003 | 미상 | EPC 튜플 원소 stride (424+8+8+8) — 이터레이션 산술, 판정 아님 | 4 |  |
| 107 | 24 | 990 | 미상 | applyed_effect 원소 (Arc<dyn EffectType>16B, CastingType 4B+pad) stride — 이터레이션 산술 | 4 |  |

**`knobs` 조정점 49건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 벽/적 우물 셀 위험 상수 | position_eval.rs:381 · 397 | 9999 | 내리면 벽·적 우물 셀이 다른 위험과 비교 가능해져 선택될 수 있다(현재는 사실상 금지값). 두 곳 같은 리터럴이라 각각 바꿔야 한다 | 4 | 기존 |
| 1 | 후보 챔피언 거리 컷 | position_eval.rs:414(적) · 432(아군) | 40000000000 | 올리면(예 250000²) 더 먼 챔피언까지 EPC 계산·타워 필터·후속 위협 계산에 들어와 비용↑/판단 범위↑, 내리면 근접전만 본다 | 4 | 기존 |
| 2 | 정글/에픽 몹 위험 반경 | position_eval.rs:451 | 22499999999 | 올리면 더 먼 몹의 기대피해가 risk 에 들어온다(몹이 노리는 경우만). 4.69셀 | 4 | 기존 |
| 3 | 몹 위험 HP% 상한 | position_eval.rs:458 · 475 | 150 | 내리면 저HP 시 몹 위험 포화가 빨라져 몹 앞에서 더 보수적 | 4 | 기존 |
| 4 | 사거리 밖(사거리+반경 안) 정글몹 위험 가중 | position_eval.rs:467 | ratio>>1 (½) | shl/lshr 로 접힘 — 1/2 를 키우면 몹 사거리 가장자리 셀 회피↑ | 4 | 기존 |
| 5 | 에픽이 남 노릴 때 가중 | position_eval.rs:480 | ratio/3 | 3 을 키우면 남을 노리는 에픽 근처 셀을 덜 피함 | 4 | 기존 |
| 6 | 근접 타워 반경 | position_eval.rs:493 · 496 | 22500000000 | 올리면 더 먼 타워까지 배치 B 타워 위험/이득 루프에 들어온다 | 4 | 기존 |
| 7 | 챔피언-타워 근접 반경(간접 포함) | position_eval.rs:494 · 497 | 4900000000 | 올리면 근처 챔피언이 붙어 있는 타워가 더 넓게 포함된다(70000=2.19셀) | 4 | 기존 |
| 8 | 타워 절 게이트 | position_eval.rs:490 (값 출처 GameSetting.tower_attack_disable_tick@0x13f8) | tick > disable_tick 이면 491~572 스킵 | GameSetting 값을 키우면 타워 절이 더 오래 실행된다 — 의미(사후/사전 어느 쪽이 '비활성'인지)는 미탐색(unknown 참조) | 4 | 기존 |
| 9 | pct_q32 피해% 상한 | position_eval.rs:387 | 150 | 내리면 한 방에 죽는 위치도 덜 위험하게 봄(타워·미니언·others·적 투사체 전부 공통 상한) | 4 | 기존 |
| 10 | 아군 타워 gain 배율 (ratio/3) | position_eval.rs:522 | 3 | 나누는 수를 줄이면 아군 타워 사거리 안(적이 물릴 위치)에 서는 이득이 커짐 | 4 | 기존 |
| 11 | 적 타워 판정 마진 | position_eval.rs:532 | 18000 | 올리면 타워 사거리 밖 더 먼 위치까지 타워 위험이 잡힘 | 4 | 기존 |
| 12 | 타워 근접 계수 coef (사거리 절반 안 100 / 밖 50) | position_eval.rs:545 | 100 | 커버·다인 상황(L550/560/562)의 미소 위험이 이 계수에 비례 — 50 을 올리면 타워 외곽도 위험 증가 | 4 | 기존 |
| 13 | 소커(v47 시즈 스탠스)가 나일 때 타워 위험 할인 (ratio/3) | position_eval.rs:540,548 | 3 | 나누는 수를 키우면 시즈 스탠스 소커가 타워를 더 두려워하지 않음 | 4 | 기존 |
| 14 | 타워가 다른 대상을 물고 있을 때 위험 (ratio/2) | position_eval.rs:555 | 2 | 커지면(예: /3) 타워 어그로가 남에게 있을 때 더 과감히 진입 | 4 | 기존 |
| 15 | 다인 진입 시 타워 위험 (cnt==2: 2/3·coef/100, cnt≥3: coef*3/100) | position_eval.rs:560,562 | 3 | 인원이 많을수록 타워 위험이 급감하는 정책의 계수 — 3 을 키우면 3인 이상 다이브 위험이 커짐 | 4 | 기존 |
| 16 | 라인 페이즈 종료 = 에픽 첫 스폰 N초 전 | runner.rs:399 / setting.rs:703 (position_eval.rs:23·576 인라인) | 30 | 올리면 legacy 미니언 위험·라인 할인(risk/4) 구간이 일찍 끝남 | 4 | 기존 |
| 17 | 미니언 웨이브 위험 상한 | position_eval.rs:7 | 140 | 내리면 웨이브 피해가 커도 위험이 덜 반영 | 4 | 기존 |
| 18 | 라인 페이즈 미니언 위험 할인 (risk/4, 상한 18) | position_eval.rs:15 | 4 | 나누는 수를 줄이거나 18 을 올리면 라인전 중 미니언 웨이브를 더 피함(CS 손해) | 4 | 기존 |
| 19 | enemy_minion_wave_is_dangerous 임계 (damage_pct>49 / hp<66&dmg>29 / hp<41&dmg>17 / hp<26&dmg>9) | minion_wave_risk.rs:71~75 | 49 | 내리면 저체력에서 웨이브를 '위험' 으로 더 자주 판정 → 할인 없이 전액 반영 | 4 | 기존 |
| 20 | 미니언/others 탐색 반경 150000 (제곱 22500000000) | position_eval.rs:581,608 | 22500000000 | 키우면 더 먼 미니언·기타 적까지 계산(비용↑) | 4 | 기존 |
| 21 | legacy 미니언 위험 가산 반경 64000 (2셀, 제곱+1) | position_eval.rs:599 | 4096000001 | 키우면 미니언에서 더 멀어도 risk 가산 | 4 | 기존 |
| 22 | others 외곽 밴드 폭 | position_eval.rs:615 | 32000 | 키우면 사거리 밖 1셀 이상까지 ratio/3 위험 | 4 | 기존 |
| 23 | 투사체 고려 반경 250000 | position_eval.rs:640 | 62499999999 | 키우면 더 먼 투사체까지 궤도 검사(비용↑) | 4 | 기존 |
| 24 | 투사체 궤도 판정 여유 | position_eval.rs:651 | 18000 | 키우면 스치는 투사체도 on_trajectory/risk 로 잡음 | 4 | 기존 |
| 25 | 비가시 아군 투사체 예외 이름 | position_eval.rs:641 | knight_ult | 이 이름의 아군 투사체만 비가시여도 평가(힐/실드 감산 후보) | 4 | 기존 |
| 26 | 우물 위험 HP% 상한 | position_eval.rs:690 (pct_q32 386) | 150 | 올리면 적 우물 사각 안 좌표의 risk 가 well_damage/hp 비례로 더 커진다(hp 가 낮을수록). 내리면 우물 진입 회피가 약해진다 | 4 | 기존 |
| 27 | gain 산정 슬롯 쿨타임 게이트(틱) | position_eval.rs:706/752/796 | 181 | 올리면 쿨타임이 더 많이 남은 스킬도 gain 후보로 세어 '스킬 사거리 안 자리' 를 더 선호. 내리면(예: 1) 지금 당장 쓸 수 있는 슬롯만 | 4 | 기존 |
| 28 | 직선이동(대시) 스킬 짧은-사거리 판정 여유 | position_eval.rs:712/735/758/779/802/823/885/910/936 | 80000 | range ≤ e.radius+champ.radius+80000 이면 '짧은 대시' 로 보고 사거리 안 어디서나 풀 ratio. 올리면 더 긴 대시도 풀 ratio(half_sq 구간 무시), 내리면 half/sq/ext 3단 감쇠 적용 범위가 넓어짐 | 4 | 기존 |
| 29 | 사거리 밖(ext) 감쇠 분모 | position_eval.rs:717/719/724 등(gain) · 871/890/892/898 등(위협) | 2 | /2(직선: half~sq 구간, 비직선: sq~ext 구간) /3(직선: sq~ext 구간). 올리면 사거리 가장자리 자리의 가치/위협이 줄어 더 안쪽으로 붙는다 | 4 | 기존 |
| 30 | 적 위협 근접 하한(제곱거리) | position_eval.rs:862 | 10000000000 | 100000²(3.125셀). 이 안이면 적 시야에 안 보여도 그 적을 위협원으로 계산. 올리면 비가시 적도 더 멀리서부터 위협에 산입 | 4 | 기존 |
| 31 | 공격 봉인 적 페널티 | position_eval.rs:874 | 3 | is_block_attack 인 적의 attack_ratio/3. 올리면 봉인된 적을 더 무시 | 4 | 기존 |
| 32 | 스킬 봉인 적 페널티 | position_eval.rs:956~958 | 3 | is_block_skill 인 적의 skill/skill2/ult ratio 전부 /3 | 4 | 기존 |
| 33 | 이동가능 스킬 위협 감쇠 계수 | position_eval.rs:961/964/968 | 3 | can_move 스킬의 ratio 를 1/3 로. 값을 올리면 이동기 보유 적을 덜 위험하게 본다 | 4 | 기존 |
| 34 | block_input 반감 | position_eval.rs:972~975 | lshr 1 (=/2) | 입력 차단(CC 등) 중인 적의 모든 ratio 반감 — 비트 시프트라 계수 조절은 소스 변경 필요 | 4 | 기존 |
| 35 | 돌진 궤도 허용폭 | position_eval.rs:983 | 20000 | 올리면 돌진 경로에서 더 먼 셀도 on_trajectory·rush_ratio 대상 | 4 | 기존 |
| 36 | rush_ratio 상한 | position_eval.rs:989 | 150 | 돌진 피해 HP% 클램프. 내리면 돌진 위협 최대치 감소 | 4 | 기존 |
| 37 | risk 합산 방식 (CC 유무) | position_eval.rs:995~998 | has_cc\|\|has_pcc ? 전부 합 : 최대 1 + 돌진 | CC 가 있는 적은 모든 스킬 위협이 합산돼 훨씬 크게 잡힌다 | 4 | 기존 |
| 38 | 아군 몸빵 반감 폭 | position_eval.rs:1006 | 28000 | 딜러 역할 시 아군이 셀→적 선분 ±(radius+28000) 안이면 risk 반감. 올리면 더 쉽게 반감 | 4 | 기존 |
| 39 | 미시야 셀 위험 분할 | position_eval.rs:1012 | sdiv 2 | 적 시야 밖 셀은 risk 절반만 risk 로, 나머지는 unseen_champ_threat 로 | 4 | 기존 |
| 40 | 고립 가산 반경 | position_eval.rs:1025/1026 | 14400000001 | 120000²+1. 아군/적 count 반경. 올리면 고립 판정이 넓어진다 | 4 | 기존 |
| 41 | 고립 가산 계수/상한 | position_eval.rs:1030 | 25 · min(cell_e,4) | 행동 중 아군 0·적 ≥2 일 때 적 수×25% 만큼 champ_threat_risk 를 risk 에 추가(최대 +100%) | 4 | 기존 |
| 42 | 선형 스킬 근접 임계 | position_eval.rs:1043/1062/1081 | 80000 | range ≤ r_a+80000+r_c 인 단거리 선형 스킬은 range_sq 안이면 전액; 넘으면 half_sq 구간 규칙 | 4 | 기존 |
| 43 | 아군 스킬 gain 감쇠 | position_eval.rs:1048/1050/1055 등 | ratio/2, ratio/3 | 거리 구간별 지원 가치 감쇠(sq 안 전액 · sq~ext 1/2 · 선형은 half~sq 1/2, sq~ext 1/3) | 4 | 기존 |
| 44 | 탱커 진입가치 반경/폭 | position_eval.rs:1109/1118 | 14400000000 / 28000 | 후방 딜러 120000 이내 + 셀이 딜러→적 선분 champ.radius()+28000 이내면 딜러 평타 HP% 를 gain 에 가산 | 4 | 기존 |
| 45 | purpose 보정 | position_eval.rs:33/36/39/40 (1142 인라인) | 120/100 · 3/2 | LineStyle Defensive: risk×1.2 / Aggressive: gain×1.2 / LaneSafe: risk·tower_risk×1.5 | 4 | 기존 |
| 46 | 노이즈 발동 정확도 임계 | position_eval.rs:1152/1153 | 1000 | positioning_accuracy ≥1000 이면 노이즈 없음. v2+ 는 RunAway/Recall 면제 | 4 | 기존 |
| 47 | 노이즈 폭 | position_eval.rs:1158/1160 | 1000-acc / (2000-2acc)/3 | 데스매치는 전폭, 일반 모드는 2/3 폭. risk = tower_risk + base_risk×(1000±spread)/1000 | 4 | 기존 |
| 48 | 노이즈 재롤 버킷 | position_eval.rs:1166 | tps*6 | v2+ 6초마다 factor 재롤(같은 버킷·같은 셀·같은 선수는 동일 factor). 늘리면 포지셔닝이 더 안정 | 4 | 기존 |

<details><summary>`callees` 피호출자 115건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | apply_position_eval_purpose | game_ai::position_eval::apply_position_eval_purpose | in:game_ai | fn(game_core::PositioningScore, usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:29 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | axis_distance_sq | game_ai::score_parameter::axis_distance_sq | in:game_ai | fn(u64, u64) -> u64 | game-ai\src\score_parameter.rs:7 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | block_input | game_core::Entity::block_input | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1501 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_move | game_core::Entity::can_move | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1489 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 8 | can_move | game_core::Champion::can_move | pub | fn(&game_core::Champion) -> bool | game-core\src\simulation\entity\champion.rs:48 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 9 | can_move | game_core::EffectType::can_move | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:358 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 10 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | check_projectile | game_core::CastingTarget::check_projectile | pub | fn(&game_core::CastingTarget, &game_core::Projectile, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:248 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | default | <game_ai::GoalData as std::default::Default>::default | pub | fn() -> game_ai::GoalData | game-ai\src\goal_data.rs:10 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 988개 중 상위 3개 |
| 13 | default | <game_ai::EpicStance as std::default::Default>::default | pub | fn() -> game_ai::EpicStance | game-ai\src\goal_data.rs:133 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 988개 중 상위 3개 |
| 14 | default | <game_ai::EpicStanceData as std::default::Default>::default | pub | fn() -> game_ai::EpicStanceData | game-ai\src\goal_data.rs:145 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 988개 중 상위 3개 |
| 15 | dist_to_line_segment | game_core::utils::dist_to_line_segment | pub | fn(i64, i64, i64, i64, i64, i64) -> u64 | game-core\src\utils.rs:200 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 18 | effect_type_cc_time | game_ai::effect_type_cc_time | pub | fn(usize, &dyn [Binder { value: Trait(game_core::EffectType), bound_vars: [] }] + ) -> std::option::Option<usize> | game-ai\src\fight_check.rs:384 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | enemy_minion_wave_has_epic_buff | game_ai::minion_wave_risk::enemy_minion_wave_has_epic_buff | in:game_ai | fn(&game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\minion_wave_risk.rs:92 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | enemy_minion_wave_is_dangerous | game_ai::minion_wave_risk::enemy_minion_wave_is_dangerous | in:game_ai | fn(&game_core::Entity, usize) -> bool | game-ai\src\minion_wave_risk.rs:64 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | enemy_minion_wave_risk_damage_at | game_ai::enemy_minion_wave_risk_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize) -> usize | game-ai\src\minion_wave_risk.rs:6 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | entity_positioning_cache_cached | game_ai::position_eval::entity_positioning_cache_cached | in:game_ai | fn(usize, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PlayerState, &game_core::PlayerState, &game_core::ChampionCache, &game_core::ChampionCache, bool) -> game_ai::score_parameter::EntityPositioningCache | game-ai\src\position_eval.rs:209 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 23 | expected_attack_damage_cached | game_ai::position_eval::expected_attack_damage_cached | in:game_ai | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\position_eval.rs:130 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 24 | expected_damage | game_core::EffectType::expected_damage | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> (usize, usize) | game-core\src\simulation\effect\type.rs:274 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 25 | expected_damage | <game_core::RushEffect as game_core::EffectType>::expected_damage | pub | fn(&game_core::RushEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> (usize, usize) | game-core\src\simulation\effect\type\rush.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 26 | expected_damage | <game_core::RangeEffect as game_core::EffectType>::expected_damage | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> (usize, usize) | game-core\src\simulation\effect\type\range_effect.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 27 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | expected_damage_target | game_core::Projectile::expected_damage_target | pub | fn(&game_core::Projectile, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\projectile.rs:1287 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | expected_heal_target | game_core::Projectile::expected_heal_target | pub | fn(&game_core::Projectile, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\projectile.rs:1457 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | expected_shield_target | game_core::Projectile::expected_shield_target | pub | fn(&game_core::Projectile, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\projectile.rs:1531 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | get_battle_role | game_ai::get_battle_role | pub | fn(usize, &game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState) -> game_ai::BattleRole | game-ai\src\utils.rs:446 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | get_damage | game_core::utils::get_damage | pub | fn(&C/#0, &T/#1, usize, game_core::AttackType, game_core::DamageType) -> usize | game-core\src\utils.rs:97 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 34 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 35 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 36 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 37 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 38 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 39 | has_cc | game_core::Projectile::has_cc | pub | fn(&game_core::Projectile) -> bool | game-core\src\simulation\projectile.rs:1272 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | is_block_attack | game_core::Entity::is_block_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1505 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | is_block_move_skill | game_core::Entity::is_block_move_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1523 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 42 | is_block_skill | game_core::Entity::is_block_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1519 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 43 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 44 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 45 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 46 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 47 | is_in_orbit | game_core::Projectile::is_in_orbit | pub | fn(&game_core::Projectile, u64, u64, u64) -> bool | game-core\src\simulation\projectile.rs:973 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 48 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 49 | is_in_range_ex | game_core::Effect::is_in_range_ex | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity, u64, u64, u64, u64, u64) -> bool | game-core\src\simulation\effect.rs:78 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 50 | is_in_range_pos | game_core::Effect::is_in_range_pos | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity, u64, u64) -> bool | game-core\src\simulation\effect.rs:67 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 51 | is_in_well_damage | game_core::is_in_well_damage | pub | fn(usize, u64, u64) -> bool | game-core\src\simulation\game.rs:5654 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 52 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 53 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 54 | is_linear_dist_no_penetrate | game_core::ProjectileMoveType::is_linear_dist_no_penetrate | pub | fn(&game_core::ProjectileMoveType) -> bool | game-core\src\simulation\projectile.rs:120 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 55 | is_targeting | game_core::ProjectileMoveType::is_targeting | pub | fn(&game_core::ProjectileMoveType) -> bool | game-core\src\simulation\projectile.rs:133 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 56 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 57 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 58 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 59 | is_visible_cell | game_core::AbstractGame::is_visible_cell | pub | fn(&Self/#0, usize, usize, usize) -> bool | game-core\src\simulation.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 60 | is_visible_cell | <game_core::Game as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::Game, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:1804 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 61 | is_visible_cell | <game_core::SingleLaneGame as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::SingleLaneGame, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:3869 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 62 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 63 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 64 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 65 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 66 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 67 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 68 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 69 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 70 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 71 | pct_q32 | game_ai::position_eval::position_eval_at_uncached::pct_q32 | in:game_ai::position_eval | fn(usize, u64) -> i64 | game-ai\src\position_eval.rs:386 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 72 | pct_q32 | game_ai::ScoreParameter::<'a>::calculate_positioning_score::pct_q32 | in:game_ai::score_parameter | fn(usize, u64) -> i64 | game-ai\src\score_parameter.rs:399 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 73 | pe_cand_masks | game_ai::position_eval::pe_cand_masks | in:game_ai::position_eval | fn(&game_core::OperationData, &game_core::PlayerState) -> (u8, u8) | game-ai\src\position_eval.rs:255 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 74 | pe_player_ctx | game_ai::position_eval::pe_player_ctx | in:game_ai::position_eval | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> game_ai::position_eval::PePlayerCtx | game-ai\src\position_eval.rs:343 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 75 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 76 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 77 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 78 | position_eval_at | game_ai::position_eval_at | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:291 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 79 | position_eval_minion_wave_risk_score | game_ai::position_eval::position_eval_minion_wave_risk_score | in:game_ai | fn(&game_core::OperationData, &game_core::Entity, game_ai::PositionEvalPurpose, usize) -> i64 | game-ai\src\position_eval.rs:6 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 80 | position_eval_pct | game_ai::score_parameter::position_eval_pct | in:game_ai | fn(usize, usize) -> i64 | game-ai\src\score_parameter.rs:52 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 81 | position_eval_use_legacy_line_minion_risk | game_ai::position_eval::position_eval_use_legacy_line_minion_risk | in:game_ai | fn(usize, &game_core::OperationData, &game_core::Entity, game_ai::PositionEvalPurpose) -> bool | game-ai\src\position_eval.rs:21 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 82 | positioning_accuracy | game_core::AthleteParameter::positioning_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:285 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 83 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 84 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 85 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 86 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 87 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 88 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 89 | remain_epic_time | game_core::MobaMode::remain_epic_time | pub | fn(&game_core::MobaMode, usize) -> usize | game-core\src\simulation\game.rs:210 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 90 | seed | game_core::AbstractGame::seed | pub | fn(&Self/#0) -> u64 | game-core\src\simulation.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 91 | seed | <game_core::Game as game_core::AbstractGame>::seed | pub | fn(&game_core::Game) -> u64 | game-core\src\simulation\game.rs:1564 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 92 | seed | <game_core::SingleLaneGame as game_core::AbstractGame>::seed | pub | fn(&game_core::SingleLaneGame) -> u64 | game-core\src\simulation\game.rs:3781 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 93 | skill2_cooldown | game_core::Entity::skill2_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1789 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 94 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 95 | skill_cooldown | game_core::Entity::skill_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1774 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 96 | slot_cc_time_cached | game_ai::slot_cc_time_cached | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u8) -> std::option::Option<usize> | game-ai\src\fight_check.rs:357 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 97 | slot_ready_cached | game_ai::slot_ready_cached | pub | fn(&game_core::OperationData, &game_core::Entity, u8) -> bool | game-ai\src\fight_check.rs:338 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 98 | start | game_core::prof::start | pub | fn(usize) -> std::option::Option<game_core::prof::ProfTimer> | game-core\src\simulation\prof.rs:175 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 99 | start | game_view::UIPhaseEffect::start | pub | fn(&mut game_view::UIPhaseEffect) | game-view\src\ui\match_ui\phase_effect.rs:30 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 100 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 101 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 102 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 103 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 104 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 105 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 106 | tower_minion_in_range_count_cached | game_ai::position_eval::tower_minion_in_range_count_cached | in:game_ai | fn(&game_core::OperationData, &game_core::Entity, usize) -> usize | game-ai\src\position_eval.rs:184 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 107 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 108 | ult_cooldown | game_core::Entity::ult_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1804 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 109 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 110 | v47_siege_stance | game_ai::v47_siege_stance | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> std::option::Option<usize> | game-ai\src\tower_discipline.rs:508 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 111 | v47_tower_covered_for_me | game_ai::v47_tower_covered_for_me | pub | fn(&game_core::OperationData, &game_core::Entity, usize) -> bool | game-ai\src\tower_discipline.rs:597 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 112 | version | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | fn(&game_ai::AgentVerHamster) -> usize | game-ai\src\lib.rs:428 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 113 | version | game_core::AiAgent::version | pub | fn(&Self/#0) -> usize | game-core\src\simulation\ai_interface.rs:503 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 114 | version | <game_core::Team as engine_core::spitz::SpitzDatable>::version | pub | fn() -> usize | game-core\src\data\team.rs:1696 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
</details>

⚠**미매칭 65개**: `allies`, `attack_range_ext_sq`, `attack_range_sq`, `attack_ratio`, `cell_dist_sq`, `chain`, `champ_threat_risk`, `clamp`, `dist_sq`, `dist_sq_sat`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `drop_glue<Option<ProfTimer>>`, `elapsed`, `elapsed — 계측)`, `enemies`, `find  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `first_spawn_tick`, `gain`, `gain_me`, `is_none_or`, `is_range`, `is_skill_linear_move`, `legacy`, `map_or`, `memcmp`, `near_allies`, `near_enemies`, `near_enemies_with_action`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `on_trajectory`, `ratio`, `reserve_internal_or_panic`, `risk`, `rush_ratio`, `sat_sub`, `skill2_linear_move`, `skill2_range`, `skill2_ratio`, `skill_linear_move`, `skill_range`, `skill_range_ext_sq`, `skill_range_half_sq`, `skill_range_sq`, `skill_ratio`, `skilled2_range`, `skilled2_ratio`, `skilled_range`, `skilled_range_ext_sq`, `skilled_range_half_sq`, `skilled_range_sq`, `skilled_ratio`, `skip`, `spread`, `tower_attack_disable_tick`, `try_fold`, `ult_linear_move`, `ult_range`, `ult_ratio`, `ulted_range`, `ulted_ratio`, `unseen_champ_threat`, `with  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 2개는 **전부 다른 함수**라 싣지 않는다`, `{closure#0}>`, `{closure#1}>>`, `{closure#2}>>`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m07.ll:24687) · **형제 0개** 

**`open` 33건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | (배치 A) 394 visible(is_visible_cell(enemy_team,xi,yi)) 와 443~445 의 3개 *_linear_move 는 배치 A 범위 안에서 소비처 없음 — 뒤 배치(B~D) 가 쓰는 값이라 여기선 계산 사실만 기록 | 4 |  |
| 1 | 미탐색 | (배치 A) closure#3~#5(s1_0·s2_0·s3_0)의 정체 — 별도 define 이 없어(fnparts: 서브프로그램 166 vs define 11) 494/497 의 `any` 클로저·507 map 클로저로 추정(전부 본문에 인라인). 번호 대응은 추정 | 5 |  |
| 2 | 미탐색 | (배치 A) iter_towers_without_nexus 의 120B 이터레이터 내부 구조(앞 6개 Option 배열 + Copied<slice::Iter<&Entity>>)는 game_core 콜리라 시그니처·관측(6개 index_range · twin_towers 슬라이스) 만 적음 — 정확한 필드 순서 미탐색(_gcbc 본체) | 4 |  |
| 3 | 미탐색 | (배치 A) pe_cand_masks 의 is_recent_visible 인자(&blackboard[enemy_team], game, player, e) 의 self 가 적 팀 블랙보드인 이유는 소스 없이는 서술 불가(관측: last_visible[5] 가 챔피언 자기 팀 보드에 위치 인덱스로 저장돼 있을 가능성) — 배치 A 판정에는 영향 없음(마스크 값만 소비) | 4 |  |
| 4 | 미탐색 | (배치 A) ProfTimer phase 번호(48/112/113) 와 PHASE_* 배열 크기 132 는 계측 전용 — 판정 상수 아님(constants 에 실은 것은 C1 대조 목적) | 4 |  |
| 5 | 미탐색 | (배치 A) prompt 의 「EPC_CACHE miss 경로에서만 불린다」는 이름 오기 — 호출자가 보는 캐시는 POS_EVAL_CACHE(PosEvalCache) 이고 EPC_CACHE 는 이 함수 내부 헬퍼(엔티티 쌍 캐시)다(tls 절에 둘 다 기록) | 4 |  |
| 6 | 미탐색 | (배치 A) src_line=371(DISubprogram line/scopeLine) · blocks.json per 와 내 루트 히스토그램은 root 0 (line:0 DILocation 113줄, 대부분 drop/cleanup) 을 포함해 1149줄로 일치(±3) | 4 |  |
| 7 | 미탐색 | (배치 B) cnt(%3636, 배치 A L498 closure#3 산출)의 의미 — 배치 B 범위 밖. L557/559 분기(cnt<2 / ==2 / ≥3)의 '무엇의 개수'인지는 배치 A 명세 참조 | 4 |  |
| 8 | 표기 불가 | (배치 B) L573 소스 표기: `if !use_legacy {L574..} else {L580..}` 인지 `if use_legacy {..} else {..}` 인지 — 표기 불가(IR 은 참→L580(legacy) / 거짓→L574 만 확정) | 4 |  |
| 9 | 미탐색 | (배치 B) L552 소스 표기(`is_none()` 로 썼는지 `is_some()` 인지 — 인라인 체인은 is_none<is_some): 동작은 nearest_enemy Some → ratio/2, None → ratio 로 확정 | 4 |  |
| 10 | 표기 불가 | (배치 B) L616 match 팔 표기 순서(Ghoul\|Bear 와 Eagle): 표기 불가 — 태그 7·9 → 0x88/0x90, 10 → 0x70/0x78 만 확정 | 4 |  |
| 11 | 미탐색 | (배치 B) L575 `tps/2` 인자의 콜리 측 의미(enemy_minion_wave_risk_damage_at 6번째 인자, 창 길이 추정) — r14 중간 명세 범위 밖, 시그니처만: (usize version[poison 전달], &OperationData, &Entity champ, u64 x, u64 y, u64 window) -> u64 | 5 |  |
| 12 | 미탐색 | (배치 B) game_core 경계 콜리 내부 미열람(시그니처·반환 의미만): Effect::is_in_range(&Effect,&Entity caster,&Entity target)->bool · is_in_range_pos(&Effect,&Entity,&Entity,u64 x,u64 y)->bool · is_in_range_ex(&Effect,&Entity,&Entity,u64 cx,u64 cy,u64 x,u64 y,u64 margin)->bool · range_adjust(&Effect,&Entity,&Entity)->u64 · expected_damage_target(&Effect,&GameContext,&Entity,&Location?,&Entity)->u64 · Entity::attack_cooltime(&Entity)->u64 · CastingTarget::check_projectile(&CastingTarget,&Projectile,&Entity)->bool · Projectile::is_in_orbit(&Projectile,u64 x,u64 y,u64 r)->bool · expected_{heal,shield,damage}_target(&Projectile,&GameContext,&Entity caster,&Entity target)->u64 · AbstractGameWithCache::iter_minions(&self,team)->Chain 56B | 4 |  |
| 13 | 미탐색 | (배치 B) v47_siege_stance(version,&OperationData,&PlayerState,&Entity tower)->(bool,usize) 의 .1 이 소커 엔티티 id 라는 것은 L537 `== my_id` 비교로 확정, 내부는 r13/r14 명세 밖(tower_discipline.rs:508, TLS 캐시 v47_siege_stance_uncached 존재) | 4 |  |
| 14 | 미탐색 | (배치 B) Effect::expected_damage_target 4번째 인자 @anon.11(v47_tower_covered_for_me:613) / @anon.72(TLS 클로저) — panic Location 상수로 추정(전달 후 콜리에서만 소비), 미확인 | 5 |  |
| 15 | 미탐색 | (배치 B) ProfTimer phase id 114·73 의 이름(prof.rs 열거) — 텔레메트리라 미조사 | 4 |  |
| 16 | 미탐색 | (배치 B) L641 "knight_ult" 예외의 게임적 의미(왜 아군 비가시 투사체를 평가하는가) — 코드 사실만 기록 | 4 |  |
| 17 | 미탐색 | (배치 B) L582 else 가지(iter_minions 가 Minion 태그 아닌 엔티티를 낼 때) 의 실제 발생 여부 — 미확인(IR 상 (false,false,false) 로 처리, 근접 미니언 캐시 경로) | 4 |  |
| 18 | 미탐색 | (배치 C) attacked_/skilled_/skilled2_/ulted_ 필드의 방향 의미(누가 누구를)는 tcxdict 필드명으로만 적었다. 이 범위의 게이트(내 skill_effect/cooldown, 내 is_block_attack)와 결과가 score.gain 에 들어가는 점에서 '내가 그 엔티티에게 가하는' 쪽으로 읽히지만 확정은 캐시 생성자(score_parameter::EntityPositioningCache, 배치 A 의 entity_positioning_cache_cached) 소관 — 미확인 | 3 |  |
| 19 | 미탐색 | (배치 C) 아군 루프(L728/774/818)가 skilled_* 를 읽는 이유(힐/버프 스킬 대상이 아군이라 그 사거리 안이 gain 인지)는 캐시 생성 쪽을 안 봐서 추정 | 5 |  |
| 20 | 미탐색 | (배치 C) L679 phi 의 %3454 진입(배치 B)에서 on_trajectory=true 로 고정되는 조건 — 그 블록(줄 674)은 배치 B 범위라 안 읽음 | 4 |  |
| 21 | 미탐색 | (배치 C) L687/L850 의 ProfTimer 가 어느 phase 번호로 시작됐는지(배치 A/B 가 %79/%80 을 채움)는 미확인 — 텔레메트리라 판정엔 무관 | 4 |  |
| 22 | 미탐색 | (배치 C) slot_cc_time_cached 의 Option<usize> 값(.1)은 이 범위에서 미사용 — 무엇인지(cc 지속틱 추정)는 callee 명세 소관 | 5 |  |
| 23 | 미탐색 | (배치 C) PositioningScore+0x32..0x37 패딩 6B 는 배치 A memset 외 기록 없음(이 범위) — 최종 store 시점은 다른 배치 | 4 |  |
| 24 | 미탐색 | (배치 C) exe fastcc 레지스터 배정: argscan 이 7=7 만 확정했고 어느 인자가 어느 레지스터/스택 슬롯인지는 이 배치가 확인하지 않음 | 4 |  |
| 25 | 표기 불가 | (배치 C) L712 등 `!(range > a+b+80000)` 극성은 분기 방향으로 확정했으나 소스가 `range <= …` 인지 `!(range > …)` 인지는 표기 불가(외연 동일) | 4 |  |
| 26 | 재료 부재 | (배치 D) EffectType vtable 0x120 이름 `can_move` 는 divtable 94% 일치(정적 vtable 기준)이고 런타임 Arc<dyn EffectType> 에 어느 구현체가 꽂히는지는 IR 로 알 수 없다 — 슬롯 번호(0x120=36번째)·시그니처 fn(&self)->bool 만 확정. 0x28 expected_damage 도 동일. | 3 |  |
| 27 | 미탐색 | (배치 D) dist_to_line_segment(i64×6)->u64 의 인자 순서(점, 선분 A, 선분 B)는 982/1005/1117 세 호출부의 의미 정합(982: 점=평가셀·선분=ent→돌진목적지)으로 추정 — 본체(game_core utils.rs:200, mir=0)는 이 배치에서 안 읽음. | 5 |  |
| 28 | 미탐색 | (배치 D) 1043~1050 선형 스킬 구간에서 `dist > range_sq` 인데 `dist <= half_sq` 인 1046 경로는 half_sq < range_sq 라면 실행 불가 — skill_range_half_sq 의 실제 정의(EntityPositioningCache::compute_range_sq, score_parameter.rs)는 안 읽었으므로 도달 가능성 미판정(재료 부재 아님·미탐색). | 4 |  |
| 29 | 미탐색 | (배치 D) get_damage::<Entity>(ent, champ, dmg, AttackType 1, DamageType 0/1) 의 4번째 인자 1 = AttackType::Skill(tcxdict --enum AttackType 태그 1) 로 읽었다 — 돌진 applyed_effect 가 스킬 계열이라는 소스 의도와 부합하나 본체 미확인. | 3 |  |
| 30 | 미탐색 | (배치 D) EPC 튜플 원소 +0x1a8 이 `&PlayerState`(.1) 라는 것은 DICompositeType 이름(`tuple$<EntityPositioningCache,ref$<PlayerState>,ref$<Entity>,u64>`)과 get_battle_role 인자 타입으로 확정했지만 tcxdict 는 튜플 레이아웃을 주지 않아 필드 순서는 DI 이름 순서 가정(424+8+8+8=448 로 크기 정합). | 3 |  |
| 31 | 미탐색 | (배치 D) %130/%132(x/32000, y/32000) 계산은 배치 A(378/379) 소관 — 배치 D 는 1172/1173 에서 소비만. 클램프(umin 29)는 %131/%133 에만 적용되고 해시엔 미클램프 값이 들어간다(관측 사실). | 4 |  |
| 32 | 미탐색 | (배치 D) 1181 의 %93/%95 drop 이 `bumpalo Vec::drop` + `RawVec::drop` 두 심볼(B11_/B18_)로 나뉘어 불리는 이유(Drop 체인)는 표준 drop 순서로 보이나 본체 미확인 — 판정 무관. | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 D) 구조체 오프셋 0x0 항목(OperationData.cache · prof ENABLED 등)은 gep 0 이 접혀 C3 에서 검사되지 않는다. | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | (배치 A) 490 게이트의 의미 — IR 은 `tick > pe_ctx.disable_tick` 이면 타워 절(491~572)을 건너뛴다고 확정(m07.ll:25967). GameSetting.tower_attack_disable_tick 이 '이 틱까지 타워 공격 비활성'인지 '이 틱부터'인지는 IR 로는 표기 불가(값 소비처만 보임) — 미탐색 = _gcbc 에서 tower_attack_disable_tick 소비처(타워 공격 게이트) 독해 또는 GameSetting 기본값 실측. 배치 B 가 510~572 를 읽으면 의미가 좁혀질 수 있다 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

