---

### `103` position_risk_all_zero_near — positioning 격자 (cx,cy)±3셀 영역(7x7)에 어떤 적 위험원(적 챔프 사거리·러시·정글/에픽·타워·미니언·중립몹·투사체·적 우물)도 닿지 않으면 true

| 항목 | 값 |
|---|---|
| id | `position_eval__position_risk_all_zero_near` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai13position_eval27position_risk_all_zero_near` |
| 소스 | `game-ai\src\position_eval.rs:1268` |
| IR | `m07.ll` 34268~37606행 |
| 경로·가시성 | `game_ai::position_risk_all_zero_near` · **pub** |
| 계층 | 기타 |
| exe | `d8ca70` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r11` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, game_ai::PositionEvalPurpose) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 사용 0건. EntityPositioningCache::new 1번 인자로 `i64 poison` 이 전달됨(34704) = callee 도 안 읽는다는 LLVM 판정. 분기 없음 | 4 |
| 1 | 2 | player | &PlayerState(2528B) | readonly·noalias·captures(address,read_provenance). team(+0x930)·position(+0x9c0) 만 읽음 | 4 |
| 2 | 3 | data | &OperationData(24B) | readonly·captures(none). cache(+0)·context(+8)·blackboard(+0x10) 3포인터 해체 | 4 |
| 3 | 4 | positioning_score | &PositioningScoreData(2760B) | readonly·captures(none). cx(+0xab8)·cy(+0xac0) 만 읽음 — value[7][7] 점수는 안 읽음 | 4 |
| 4 | 5 | purpose | PositionEvalPurpose(i8 range 0..13) | 본문 사용 0건(%4 참조 없음). 니치 태그 2..12(tcxdict --enum PositionEvalPurpose) | 3 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// position_eval.rs:1268~1284 — 준비
let Some(champ) = data.cache.player_champion[player.team][player.position.as_index()] else { return true };   // 34331: None→true
enemy_team = 1 - player.team;
min_xi = max(cx-3, 0); max_xi = min(cx+3, 29); min_yi = max(cy-3, 0); max_yi = min(cy+3, 29);   // 1275~1278
lx = min_xi*32000+16000; rx = max_xi*32000+16000; ly = ...; ry = ...;   // 1279~1282 셀 중심 좌표
if cell_region_in(player.team, lx,rx,ly,ry) { return false }   // 1284: 적 우물 피해 rect 2개(팀별) 와 rects_intersect → false

// 1288~1295 — near_enemies_raw: Vec<(SmallAction, &Entity)> (bumpalo, ctx.pool)
//   = player_champion[enemy_team].iter().enumerate()
//       .filter(|(i,opt)| opt.is_some_and(|e| blackboard[enemy_team].is_recent_visible(game, player, e) && distance_sq(e, champ) <= 200000²))   // aux m07 61218~ (closure$0/1)
//       .map(|(i,c)| (blackboard[enemy_team].small_actions[i](24B 복사), c))   // aux m12 (closure$0 L1290)
//       .map(|(a,v)| (a.unwrap(), v.unwrap()))   // aux m01 (closure$2 L1294)
//       .collect_in(pool)

// 1297~1352 — 적 챔피언 루프
for (_, caster) in near_enemies_raw.iter() {   // caster = 튜플.1 (+24)
  eplayer = cache.player_by_champion_id(caster.id).unwrap();   // 1298
  ecache = cache.player_champion_cache[eplayer.team][eplayer.position];   // 1300 (bounds<2)
  cache_ = EntityPositioningCache::new(<version: 미사용/poison>, champ, caster, player, eplayer, champ_cache(내), ecache, !near_enemies_raw.is_empty());   // 1301
  unseen_limit = if game.is_visible(enemy_team, champ.id) { u64::MAX } else { 100000 };   // 1302 (vtable+0xf8)
  e_atk = caster.attack_effect.as_ref().unwrap();   // 1305 (None→panic)
  attack_range = Effect::range(e_atk, caster, champ)[= caster.stat.range + e_atk.range + e_atk.growth_range*(caster.level-1) + range_adjust(e_atk, caster, champ)] + 50000 + caster.radius() + champ.radius();   // 1306~1307
  if cache_.attack_ratio > 0 && circle_intersects_cell_region(caster.x, caster.y, min(unseen_limit, attack_range), lx,rx,ly,ry) { return false }   // 1308
  if let Some(skill) = caster.skill_effect.as_ref() {   // 1312
    skill_range = range(skill,caster,champ) + 50000 + caster.radius() + champ.radius();   // 1313~1314
    ratio = if caster.is_block_skill() { cache_.skill_ratio/3 } else { cache_.skill_ratio };   // 1315
    if ratio > 0 && caster.skill_cooldown() < 181 [Champion 이면 ty.0.skill_cooldown, 비챔프는 검사 생략(0)]   // 1316
       && circle_intersects(caster.pos, min(unseen_limit, skill_range)) { return false }   // 1317
  }
  if let Some(skill2) = caster.skill2_effect() [level>2 일 때만 Some] {   // 1321
    skill2_range = range(skill2..) + 50000 + 두 radius;  ratio = skill2_ratio (/3 if block);   // 1322~1324
    if ratio > 0 && caster.skill2_cooldown() < 181 && circle_intersects(caster.pos, min(unseen_limit, skill2_range)) { return false }   // 1325~1326
  }
  if let Some(ult) = caster.ult_effect() [level>4] {   // 1330
    ult_range = ...+50000+...; ratio = ult_ratio (/3 if block);   // 1331~1333
    if ratio > 0 && caster.ult_cooldown() < 181 && circle_intersects(caster.pos, min(unseen_limit, ult_range)) { return false }   // 1334~1335
  }
  // 1340~1351 러시 상태
  match caster.rush_state { Rush{applyed_effect, x:ex, y:ey, range, ..} => (0x310,0x330,0x338,0x340), RushPenetrate{applyed_effect(0x308), x(0x340), y(0x348), range(0x350), ..} => .., _ => continue }
  has_damage = applyed_effect.iter().any(|(eff_ty, _)| { (ad, ap) = eff_ty.vtable[+0x28](ctx, caster as &dyn); get_damage(caster, champ, ad, 1, 0) != -get_damage(caster, champ, ap, 1, 1) });   // 1342~1345 (ad피해+ap피해 ≠ 0)
  if !has_damage { continue }
  r = *range + 20000 + champ.radius();   // 1348
  lx1 = min(ex, caster.x) - r(sat); rx1 = max(ex, caster.x) + r; ly1 = min(ey, caster.y) - r; ry1 = max(ey, caster.y) + r;   // 1349~1351
  if rects_intersect(lx,rx,ly,ry, lx1,rx1,ly1,ry1) { return false }
}

// 1358~1376 — 정글/에픽
for j in cache.jungles.iter() {
  if distance_sq(j, champ) > 150000²-1 { continue }   // 1359
  match j.ty { Jungle(info) => { if info.focused != Some(champ.id) { continue } ... }   // 1362~1363
                Epic(info)   => { if info.focused.is_none() { continue } ... }   // 1372
                _ => continue }
  effect = j.attack_effect.as_ref().unwrap();   // 1364/1373
  if effect.expected_damage_target(ctx, j as &dyn, champ) == 0 { continue }   // 1365/1374
  range = Effect::range(effect, j, champ) + 32000 + j.radius() + champ.radius();   // 1366/1375
  if circle_intersects(j.pos, range, region) { return false }   // 1367/1376
}

// 1385~1391 — 타워 준비
player_team_minions = cache.minions(player.team, ctx);   // 1385 (bumpalo Vec<&Entity>)
disable_tick = match ctx.tutorial.player_count() { 2 => setting.tower_attack_disable_tick_2v2, 3 => ..._3v3, _(5) => setting.tower_attack_disable_tick };   // 1386~1389
if game.tick() <= disable_tick {   // 1391 (tick > disable_tick 이면 타워 루프 전체 생략)
  for tower in cache.iter_towers_without_nexus(enemy_team) {   // 1392
    if !(distance_sq(tower, champ) < 150000² || near_enemies_raw.iter().any(|(_,e)| distance_sq(e, tower) < 70000²)) { continue }   // 1393~1394
    effect = tower.attack_effect.as_ref().unwrap();   // 1397
    range = Effect::range(effect, tower, champ) + 18000 + (if effect.casting == Targeting(0) { 0 } else { tower.radius() }) + champ.radius();   // 1398~1400
    if !circle_intersects(tower.pos, range, region) { continue }   // 1401
    damage = effect.expected_damage_target(ctx, tower as &dyn, champ);   // 1404
    dmg100 = (damage * setting.tick_per_second / tower.attack_cooltime() [0→div_by_zero panic] + damage) * 100;   // 1405
    cnt = player_team_minions.iter().filter(|m| effect.is_in_range(tower, m)).count();   // 1406
    if max(champ.hp,1) <= dmg100 || cnt > 2 { return false }   // 1407
  }
}

// 1414~1417 — 적 미니언
for minion in cache.iter_minions(enemy_team) {
  if distance_sq(minion, champ) >= 150000² { continue }   // 1415
  if !minion.attack_effect.as_ref().is_some_and(|e| e.expected_damage_target(ctx, minion as &dyn, champ) != 0) { continue }   // 1416
  if point_cell_region_dist_sq(minion.pos, region) <= 120000² { return false }   // 1417 (minion_risk_radius=120000)
}

// 1422~1429 — 적 팀 기타 엔티티(others)
for enemy in cache.others[enemy_team].iter() {
  if distance_sq(enemy, champ) > 150000² { continue }   // 1423
  let Some(effect) = enemy.attack_effect.as_ref() else { continue };   // 1426
  range = Effect::range(effect, enemy, champ) + 32000 + enemy.radius() + champ.radius();   // 1427
  if effect.expected_damage_target(ctx, enemy as &dyn, champ) == 0 { continue }   // 1428
  if circle_intersects(enemy.pos, range, region) { return false }   // 1429
}

// 1435~1446 — 투사체
for projectile in game.iter_projectile() {   // vtable+0x210 → ProjectileIter
  if projectile.team == champ.team { continue }   // 1436 (TeamType 비교: 태그 같고 Player 면 payload 도 같아야)
  if projectile.is_targeting() [move_type ∈ {Target, TargetSplash, BouncingTarget{target_id: Some}}] { continue }   // 1436
  if !projectile.applyed_target.check_projectile(projectile, champ) { continue }   // 1437
  if distance_sq(champ, projectile.pos) > 250000²-1 { continue }   // 1438
  if !(projectile.is_visible || projectile.name == "knight_ult") { continue }   // 1439
  let Some(caster) = game.get_entity_by_id(projectile.caster_id) else { continue };   // 1442 (vtable+0x1f0)
  if projectile.expected_damage_target(ctx, caster, champ) == 0 { continue }   // 1445
  // 1446 projectile_intersects_cell_region(projectile, radius=champ.radius(), region)  [1238~1262]
  match projectile.move_type {
    LinearDist => match projectile.shape.tag { 0(원) => rect(min/max(pos, target)±(radius+33000+shape.r)) intersect → false,
                                              1(선분) => rect(min/max(from,to)±(radius+33000+width)) intersect → false,
                                              2(사각) => rect(pos ± (w/2 + radius+33000, h/2 + radius+33000)) intersect → false,
                                              3 => return false(무조건 위험) },
    Delayed|Periodic|ApplyIn|Parabolic => circle(pos, radius+33000) intersect → false,
    _(FollowTargetShrinkBarrier|BouncingTarget(None)|Removed) => return false(무조건 위험)
  }
}
return true;   // 1452 (near_enemies_raw·player_team_minions drop 후)

// 헬퍼(인라인) point_cell_region_dist_sq(1212~1214): dx = px<lx ? lx-px : sat(px-rx); dy 동일; dx²+dy² (saturating)
// circle_intersects_cell_region(1219): dist_sq <= r² (r² 오버플로면 true)
// rects_intersect(1224): lx<=rx1 && lx1<=rx && ly<=ry1 && ly1<=ry
```

**`mem` 메모리 접근 78건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | team | r | player.team(usize, bounds<2). L1271·1274(enemy_team=1-team)·1284·1385(minions 내 팀) | 4 | OK |
| 1 | PlayerState | 0x9c0 | position | r | position.as_index()(i32→zext). L1271 player_champion[team][pos] | 4 | OK |
| 2 | PlayerState | 0x930 | eplayer.team | r | 적 PlayerState(player_by_champion_id 반환)의 team — L1300 ecache 인덱스(bounds<2) | 4 | OK |
| 3 | PlayerState | 0x9c0 | eplayer.position | r | 적 PlayerState position.as_index() — L1300 ecache 인덱스 | 4 | 오귀속(사전은 다른 필드를 준다) |
| 4 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(8840B) | 4 | OK |
| 5 | OperationData | 0x8 | context | r | &GameContext(64B) — from_iter_in 3번째 인자·expected_damage_target·setting 경유 | 4 | OK |
| 6 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — aux 클로저에서 [enemy_team] 인덱싱 | 4 | OK |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 — vtable 호출 self | 4 | OK |
| 8 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable(816B): +0x28 tick(L1391) · +0xf8 is_visible(L1302) · +0x1f0 get_entity_by_id(L1442) · +0x210 iter_projectile(L1435) (divtable AbstractGame) | 3 | OK |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] — L1271 내 챔프(None→return true) · L1289 적 팀 5칸 순회 | 4 | OK |
| 10 | AbstractGameWithCache | 0x280 | player_champion_cache[][] | r | [[ChampionCache(800B);5];2] — L1300 내 캐시·적 캐시 → EntityPositioningCache::new 인자 | 4 | OK |
| 11 | AbstractGameWithCache | 0xd0 | jungles.ptr | r | bumpalo Vec<&Entity> ptr — L1358 정글/에픽 순회 | 4 | OK |
| 12 | AbstractGameWithCache | 0xe8 | jungles.len | r | L1358 | 4 | OK |
| 13 | AbstractGameWithCache | 0xf0 | others[enemy_team].ptr | r | [bumpalo Vec<&Entity>;2] stride 32 — L1422 적 팀 기타 엔티티 순회(ptr +0, len +0x18) | 4 | OK |
| 14 | PositioningScoreData | 0xab8 | cx | r | L1275·1276 min_xi=max(cx-3,0)·max_xi=min(cx+3,29) | 4 | OK |
| 15 | PositioningScoreData | 0xac0 | cy | r | L1277·1278 min_yi·max_yi | 4 | OK |
| 16 | Entity | 0x5c0 | id | r | champ.id(L1302 is_visible·L1363 focused 비교) · caster.id(L1298 player_by_champion_id) | 4 | OK |
| 17 | Entity | 0x5c8 | level | r | caster/j/tower.level — Effect::range 의 growth*(level-1) · skill2_effect(level>2) · ult_effect(level>4) | 4 | OK |
| 18 | Entity | 0x660 | x | r | champ.x(distance_sq 기준) · 각 위험원 x(circle 중심) | 4 | OK |
| 19 | Entity | 0x668 | y | r |  | 4 | OK |
| 20 | Entity | 0x670 | hp | r | champ.hp — L1405 max(hp,1) 타워 피해 비교 | 4 | OK |
| 21 | Entity | 0x680 | radius | r | Entity::radius() 인라인(entity.rs:1511~1515): radius_mult==0 ? radius : radius*(mult+100)/100 | 4 | OK |
| 22 | Entity | 0x470 | stat_buff_cached.radius_mult | r | i32 — radius() 인라인 | 4 | OK |
| 23 | Entity | 0x438 | stat_buff_cached.range | r | Effect::range 인라인(effect.rs:26) 항 | 4 | OK |
| 24 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect(56B) — L1305(caster 필수 unwrap)·1364·1373(정글/에픽 unwrap)·1397(타워 unwrap)·1416(미니언 is_some_and)·1426(others as_ref) | 4 | OK |
| 25 | Entity | 0x4a0 | attack_effect@Some.0.range | r | Effect::range 항 | 4 | OK |
| 26 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | ×(level-1) | 4 | OK |
| 27 | Entity | 0x4c0 | attack_effect@tag / attack_effect@Some.0.casting@tag | r | i32 -1 = None(니치). L1399 타워: casting==0(Targeting) 이면 타워 radius 항 0 | 4 | OK |
| 28 | Entity | 0x4c8 | skill_effect@Some.0 | r | L1312 (+0x4d8 range·+0x4e0 growth·+0x4f8 tag) | 4 | OK |
| 29 | Entity | 0x4d8 | skill_effect@Some.0.range | r |  | 4 | OK |
| 30 | Entity | 0x4e0 | skill_effect@Some.0.growth_range | r |  | 4 | OK |
| 31 | Entity | 0x4f8 | skill_effect@tag | r | -1 = None → skill 블록 스킵 | 4 | OK |
| 32 | Entity | 0x500 | skill2_effect@Some.0 | r | L1321 skill2_effect(): level>2 일 때만(아니면 정적 None 전역 @anon..31). tag +0x530(=0x500+48) | 4 | OK |
| 33 | Entity | 0x538 | ult_effect@Some.0 | r | L1330 ult_effect(): level>4 일 때만. tag +0x568(=0x538+48) | 4 | OK |
| 34 | Entity | 0x68 | ty@tag | r | 13=Champion(cooldown 읽기 게이트 L1316/1325/1334) · L1362 switch 4=Jungle/5=Epic | 4 | OK |
| 35 | Entity | 0xb8 | ty@Champion.0.skill_cooldown | r | L1316 <181 (Champion 일 때만; 비챔프는 0 취급→검사 생략) | 4 | OK |
| 36 | Entity | 0xc0 | ty@Champion.0.skill2_cooldown | r | L1325 <181 | 4 | OK |
| 37 | Entity | 0xc8 | ty@Champion.0.ult_cooldown | r | L1334 <181 | 4 | OK |
| 38 | Entity | 0x88 | ty@Jungle.info.focused@tag / ty@Epic.info.focused@tag | r | L1363 Jungle: focused==Some(champ.id) · L1372 Epic: focused.is_some() | 4 | OK |
| 39 | Entity | 0x90 | ty@Jungle.info.focused@Some.0 | r | L1363 == champ.id | 4 | OK |
| 40 | Entity | 0x308 | rush_state@tag | r | L1340 니치 태그(0x8000..+idx): Rush(+3)→%498 · RushPenetrate(암묵)→%503 · 그 외 다음 적 | 4 | OK |
| 41 | Entity | 0x310 | rush_state@Rush.applyed_effect (cap/ptr+0x318/len+0x320) | r | L1340 Rush 팔: Vec<(Arc<dyn EffectType>, CastingType)> 원소 24B | 4 | OK |
| 42 | Entity | 0x330 | rush_state@Rush.x | r | L1340 ex | 4 | OK |
| 43 | Entity | 0x338 | rush_state@Rush.y | r | L1340 ey | 4 | OK |
| 44 | Entity | 0x340 | rush_state@Rush.range / rush_state@RushPenetrate.x | r | Rush 팔 range · RushPenetrate 팔 ex | 4 | OK |
| 45 | Entity | 0x348 | rush_state@RushPenetrate.y | r | L1341 ey | 4 | OK |
| 46 | Entity | 0x350 | rush_state@RushPenetrate.range | r | L1341 range (applyed_effect 는 +0x308 cap/+0x310 ptr/+0x318 len) | 4 | OK |
| 47 | Entity | 0x0 | team@tag | r | L1436 champ.team == projectile.team 비교(TeamType: 0 Player(payload +0x8)/1 Neutral) | 4 | OK |
| 48 | Entity | 0x8 | team@Player.0 | r | L1436 payload 비교 | 4 | OK |
| 49 | EntityPositioningCache | 0x0 | attack_ratio | r | L1308 >0 이면 평타 사거리 검사 | 4 | OK |
| 50 | EntityPositioningCache | 0x8 | skill_ratio | r | L1315 (/3 if caster.is_block_skill()) | 4 | OK |
| 51 | EntityPositioningCache | 0x10 | skill2_ratio | r | L1324 | 4 | OK |
| 52 | EntityPositioningCache | 0x18 | ult_ratio | r | L1333 | 4 | OK |
| 53 | GameContext | 0x8 | setting | r | &GameSetting — L1386 disable_tick · L1405 tick_per_second | 4 | OK |
| 54 | GameContext | 0x38 | tutorial | r | TutorialType(i8) → player_count()(runner.rs:295) 인라인: First(1)/Bottom(3)→2v2, MidBottom(5)→3v3, 그 외→5 | 4 | OK |
| 55 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | L1386~1389 player_count 5 (gep 5112) | 4 | OK |
| 56 | GameSetting | 0x1400 | tower_attack_disable_tick_2v2 | r | gep 5120 | 4 | OK |
| 57 | GameSetting | 0x1408 | tower_attack_disable_tick_3v3 | r | gep 5128 | 4 | OK |
| 58 | GameSetting | 0x12f8 | tick_per_second | r | L1405 damage*tps/attack_cooltime (gep 4856) | 4 | OK |
| 59 | Projectile | 0x0 | team | r | TeamType(16B) L1436 | 4 | OK |
| 60 | Projectile | 0x10 | shape@tag | r | L1446 projectile_intersects_cell_region(1239): LinearDist 일 때 0/1/2/3 분기 | 4 | OK |
| 61 | Projectile | 0x18 | shape@*.0 (radius / width) | r | shape 0: 반경 · 1: width · 2: width | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |
| 62 | Projectile | 0x20 | shape@1.from_x / shape@2.height | r |  | 4 | OK |
| 63 | Projectile | 0x28 | shape@1.from_y | r |  | 4 | OK |
| 64 | Projectile | 0x30 | shape@1.to_x | r |  | 4 | OK |
| 65 | Projectile | 0x38 | shape@1.to_y | r |  | 4 | OK |
| 66 | Projectile | 0x40 | move_type@tag | r | 니치 2..11(untagged BouncingTarget 은 0/1=target_id Option 태그). L1436 is_targeting · L1446 분기 | 4 | OK |
| 67 | Projectile | 0x68 | move_type@LinearDist.target_x | r | L1446(1239) | 4 | OK |
| 68 | Projectile | 0x70 | move_type@LinearDist.target_y | r |  | 4 | OK |
| 69 | Projectile | 0x98 | name.ptr | r | L1439 == "knight_ult"(10B memcmp) | 4 | OK |
| 70 | Projectile | 0xa0 | name.len | r | L1439 len==10 | 4 | OK |
| 71 | Projectile | 0xf8 | caster_id | r | L1442 game.get_entity_by_id | 4 | OK |
| 72 | Projectile | 0x100 | x | r | L1438 distance_sq(champ, proj) · L1446 원/사각 중심 | 4 | OK |
| 73 | Projectile | 0x108 | y | r |  | 4 | OK |
| 74 | Projectile | 0x12c | applyed_target | r | CastingTarget(4B) → check_projectile(&self, proj, champ) L1437 | 4 | OK |
| 75 | Projectile | 0x131 | is_visible | r | L1439 is_visible \|\| name=="knight_ult" | 4 | OK |
| 76 | Blackboard | 0x78 | small_actions[pos]@tag | r | aux m12 21689: blackboard[enemy_team].small_actions[idx] 24B 복사 → 튜플 .0 (본체에서 .0 은 안 읽음) | 4 | OK |
| 77 | Effect | 0x0 | ty(Arc<dyn EffectType>).ptr / vtable | r | L1342 rush applyed_effect 원소의 (Arc data(+0), vtable(+8)) → vtable+0x28 슬롯 호출 → {i64,i64}(ad,ap) | 4 | OK |

**`consts` 상수 33건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 1271 | 임계 | team bounds(<2) · L1321 level>2 → skill2_effect 존재 | 4 |  |
| 1 | 3 | 1275 | 태그 | 격자 반경 ±3셀 (cx-3..cx+3, cy-3..cy+3 = 7x7) · L1315/1324/1333 is_block_skill 이면 ratio/3 | 4 |  |
| 2 | 29 | 1276 | 인덱스 | 셀 인덱스 상한(30x30 맵) min(cx+3,29) | 4 |  |
| 3 | 32000 | 1279 | 계수 | 셀 크기(좌표 변환 cell*32000+16000) · ★L1366/1375/1427 에서는 Effect::range 인라인 합에 32000 이 붙음(정글·에픽·others 사거리 여유 1셀 — 50000 과 대비, meaning 아래 50000 참조) | 4 |  |
| 4 | 16000 | 1279 | 미상 | 셀 중심 오프셋 | 4 |  |
| 5 | 64001 | 1284 | 임계 | 적 우물 피해 영역 rect(cell_region_in 1228~1233 = game.rs is_in_well_damage 와 동일 상수): team1 → x≤64000∧800000≤y≤960000 ∪ x≤160000∧896000≤y≤960000 / team0 → 800000≤x≤960000∧y≤64000 ∪ 896000≤x≤960000∧y≤160000. 7x7 영역이 겹치면 return false | 4 |  |
| 6 | 960001 | 1284 | 임계 | 맵 끝 960000(30셀) 포함 비교 | 4 |  |
| 7 | 799999 | 1284 | 임계 | >799999 = ≥800000 (우물 rect) | 4 |  |
| 8 | 160001 | 1284 | 임계 | <160001 = ≤160000 (우물 rect) | 4 |  |
| 9 | 895999 | 1284 | 임계 | >895999 = ≥896000 (우물 rect) | 4 |  |
| 10 | 40000000001 | 1293 | 미상 | 200000² + 1 — (aux 61316) near_enemies_raw 필터: distance_sq(e, champ) ≤ 200000² 인 적 챔프만 | 4 |  |
| 11 | -1 | 1302 | 센티널 | u64::MAX — is_visible(enemy_team, champ.id) 참이면 unseen_limit 무한 | 4 |  |
| 12 | 100000 | 1302 | 산출값 | unseen_limit: 적 팀에 내가 안 보이면 적 사거리를 100000 으로 캡(min(unseen_limit, range)) | 4 |  |
| 13 | 50000 | 1306 | 계수 | 적 챔프 평타/스킬/스킬2/궁 사거리 여유. IR 에선 Effect::range 인라인(effect.rs:26) 의 add 에 붙어 있으나 정글/others 루프(같은 인라인)는 32000 이라 ★caller 줄의 상수가 재결합(reassociation)된 것으로 판단 — 소스상 어느 항인지는 표기 불가 | 4 |  |
| 14 | 100 | 1307 | 계수 | Entity::radius 인라인 radius*(mult+100)/100 · L1405 (damage+damage*tps/cooltime)*100 | 4 |  |
| 15 | 181 | 1316 | 임계 | 스킬/스킬2/궁 쿨다운 <181틱(=≤3초@60tps) 이어야 사거리 위험으로 봄 | 4 |  |
| 16 | 13 | 1316 | 태그 | EntityType 태그 13=Champion — 챔피언일 때만 cooldown 필드 읽음 | 4 |  |
| 17 | 4 | 1330 | 센티널 | level>4 → ult_effect 존재 · L1340 rush_state 니치 디코딩(untagged=4) · L1362 ty 4=Jungle | 4 |  |
| 18 | 5 | 1362 | 태그 | ty 5=Epic · player_champion 5칸 | 4 |  |
| 19 | -9223372036854775808 | 1341 | 계수 | RushState 니치 시작(0x8000_0000_0000_0000) — tag^MIN 으로 논리 idx 복원 | 4 |  |
| 20 | 20000 | 1348 | 계수 | 러시 사각 여유 r = rush.range + 20000 + champ.radius() | 4 |  |
| 21 | 22499999999 | 1359 | 임계 | 150000²−1 — distance_sq > 이것(=≥150000²)이면 정글/에픽(L1359)·타워(L1393) 스킵 | 4 |  |
| 22 | 22500000000 | 1415 | 임계 | 150000² — 미니언(L1415) <150000² 만 검사 · others(L1423) >150000² 스킵 | 4 |  |
| 23 | 4900000000 | 1394 | 임계 | 70000² — 타워가 멀어도 near_enemies_raw 중 타워 70000 이내 적 챔프가 있으면 타워 검사 | 4 |  |
| 24 | 18000 | 1398 | 계수 | 타워 사거리 여유(Effect::range 인라인 합에 +18000 — 50000/32000 과 같은 성격) | 4 |  |
| 25 | 14400000001 | 1417 | 임계 | minion_risk_radius 120000 의 제곱+1 — 미니언 위치~영역 dist_sq ≤ 120000² 이면 위험(r² 접힘) | 4 | 120000 |
| 26 | 62499999999 | 1438 | 임계 | 250000²−1 — 투사체가 champ 에서 ≥250000 이면 스킵 | 4 |  |
| 27 | 10 | 1439 | 길이 | "knight_ult" 길이(비가시 투사체 예외 이름) | 4 |  |
| 28 | 9 | 1436 | 태그 | ProjectileMoveType 태그 디코딩 assume(tag≠9) · move_type 논리 idx 9=Removed | 4 |  |
| 29 | 7 | 1436 | 태그 | move_type 태그≤1 → 논리 idx 7(BouncingTarget, untagged) 복원 | 4 |  |
| 30 | 33000 | 1446 | 계수 | 투사체 반경 여유 r = champ.radius() + 33000 (+shape 반경/폭) | 4 |  |
| 31 | 1 | 1405 | 임계 | max(champ.hp, 1) 0 나눗셈 방지 · L1274 enemy_team = 1 - team | 4 |  |
| 32 | 0 | 1308 | 임계 | ratio>0 · expected_damage_target≠0 · casting==0(Targeting) 게이트 | 4 |  |

**`knobs` 조정점 13건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 격자 검사 반경(셀) | position_eval.rs:1275~1278 | 3 | 올리면 더 넓은 영역이 '전부 안전'해야 true — true 가 드물어진다(보수적) | 4 | 기존 |
| 1 | 미시야 적 사거리 캡 | position_eval.rs:1302 | 100000 | 올리면 안 보이는 적 챔프의 사거리를 더 크게 봐서 false 가 늘어난다 | 4 | 기존 |
| 2 | 적 챔프 사거리 여유 | position_eval.rs:1306(IR 상 Effect::range 인라인 add) | 50000 | 올리면 적 챔프 평타/스킬 위협 반경이 커져 false 가 늘어난다 | 4 | 기존 |
| 3 | 스킬 쿨다운 상한(틱) | position_eval.rs:1316/1325/1334 | 181 | 올리면 쿨이 더 긴 스킬도 위협으로 봐서 false 가 늘어난다 | 4 | 기존 |
| 4 | 러시 사각 여유 | position_eval.rs:1348 | 20000 | 올리면 러시 궤적 위협 폭이 커진다 | 4 | 기존 |
| 5 | 위험원 탐색 거리(정글/타워/미니언/others) | position_eval.rs:1359/1393/1415/1423 | 22500000000 | 150000². 올리면 더 먼 위험원까지 검사 | 4 | 기존 |
| 6 | 타워 검사 트리거 — 적 챔프~타워 거리 | position_eval.rs:1394 | 4900000000 | 70000². 올리면 멀리 있는 타워도 근처 적 챔프 때문에 검사 대상이 된다 | 4 | 기존 |
| 7 | 타워 사거리 여유 | position_eval.rs:1398 | 18000 | 올리면 타워 위협 반경 증가 | 4 | 기존 |
| 8 | 타워 위험 판정 미니언 수 | position_eval.rs:1407 | 2 | cnt > 2 — 내 미니언이 타워 사거리 안에 3마리 이상이면 (타워가 미니언을 때리는 상황이어도) 위험으로 봄. 올리면 타워를 덜 위험하게 본다 | 4 | 기존 |
| 9 | 미니언 위험 반경 | position_eval.rs:1417 (14400000001 = 120000²+1) | 120000 | 올리면 적 미니언 근처를 더 넓게 위험으로 본다 | 4 | 기존 |
| 10 | 투사체 탐색 거리 | position_eval.rs:1438 | 62499999999 | 250000²−1. 올리면 더 먼 투사체까지 검사 | 4 | 기존 |
| 11 | 투사체 반경 여유 | position_eval.rs:1446(1241/1247/1254/1261) | 33000 | 올리면 투사체 위협 폭 증가 | 4 | 기존 |
| 12 | 정글/에픽/others 사거리 여유 | position_eval.rs:1366/1375/1427 | 32000 | 올리면 중립몹/기타 위협 반경 증가 | 4 | 기존 |

<details><summary>`callees` 피호출자 46건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | check_projectile | game_core::CastingTarget::check_projectile | pub | fn(&game_core::CastingTarget, &game_core::Projectile, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:248 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | circle_intersects_cell_region | game_ai::position_eval::circle_intersects_cell_region | in:game_ai::position_eval | fn(u64, u64, u64, u64, u64, u64, u64) -> bool | game-ai\src\position_eval.rs:1218 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 6 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | expected_damage_target | game_core::Projectile::expected_damage_target | pub | fn(&game_core::Projectile, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\projectile.rs:1287 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | get_damage | game_core::utils::get_damage | pub | fn(&C/#0, &T/#1, usize, game_core::AttackType, game_core::DamageType) -> usize | game-core\src\utils.rs:97 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | is_block_skill | game_core::Entity::is_block_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1519 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 16 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 17 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | is_targeting | game_core::ProjectileMoveType::is_targeting | pub | fn(&game_core::ProjectileMoveType) -> bool | game-core\src\simulation\projectile.rs:133 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 21 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 22 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 23 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 25 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 26 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 27 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | minions | game_core::AbstractGameWithCache::<'a, 'b>::minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1853 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | new | game_ai::score_parameter::EntityPositioningCache::new | pub | fn(usize, &game_core::Entity, &game_core::Entity, &game_core::PlayerState, &game_core::PlayerState, &game_core::ChampionCache, &game_core::ChampionCache, bool) -> game_ai::score_parameter::EntityPositioningCache | game-ai\src\score_parameter.rs:136 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 32 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 33 | projectile_intersects_cell_region | game_ai::position_eval::projectile_intersects_cell_region | in:game_ai::position_eval | fn(&game_core::Projectile, u64, u64, u64, u64, u64) -> bool | game-ai\src\position_eval.rs:1237 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 34 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 35 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 36 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | rects_intersect | game_ai::position_eval::rects_intersect | in:game_ai::position_eval | fn(u64, u64, u64, u64, u64, u64, u64, u64) -> bool | game-ai\src\position_eval.rs:1223 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 38 | skill2_cooldown | game_core::Entity::skill2_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1789 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 39 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 40 | skill_cooldown | game_core::Entity::skill_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1774 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 41 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 42 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 43 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 44 | ult_cooldown | game_core::Entity::ult_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1804 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 45 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 12개**: `applyed_effect`, `cell_region_in`, `champ_cache`, `circle`, `circle_intersects`, `enumerate`, `memcmp`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `point_cell_region_dist_sq`, `rect`, `skill2_ratio`, `ult_ratio`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m05.ll:25877, m05.ll:33683, m10.ll:26670) · **형제 0개** 

**`open` 9건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | 50000(챔프)/32000(정글·others)/18000(타워) 이 소스상 어느 줄·항에 있는지 — IR 에선 셋 다 `range@effect:26` 인라인 사이트의 add 에 붙어 있어 Effect::range 안 상수처럼 보이지만 같은 함수 인라인에 값이 다르므로 caller 줄 상수의 reassociation 으로 판단. 표기 위치는 표기 불가(column 부재) | 4 |  |
| 1 | 미탐색 | near_enemies_raw 원소 .0 의 타입 — 망글은 `(SmallAction, &Entity)` 인데 tcx 는 blackboard.small_actions 가 `[Option<SmallAction>;5]`. aux m01 closure$2 가 `.unwrap()`(태그 -1 검사) 하므로 Option→SmallAction. 본체는 .0 을 안 읽으므로 판정 무영향 | 3 |  |
| 2 | 미탐색 | cell_region_in(1228~1233) 이 '적 우물' 인지 '내 우물' 인지 — 상수·팀 분기가 path_finder::is_enemy_well_danger(m03 144529~) 의 game.rs is_in_well_damage 인라인과 완전히 같아 적 우물로 판단(근거: 동일 rect·동일 team 분기). 소스 이름은 미확인 | 4 |  |
| 3 | 미탐색 | EffectType vtable +0x28 슬롯(L1342, {i64,i64} 반환·인자 (self, &ctx, caster, &Entity vtable @anon.11)) 의 메서드 이름 — Arc<dyn EffectType> 런타임 vtable 이라 divtable 무력(정적 @vtable 없음). 반환 (ad, ap) 로 get_damage(…,1,0)/(…,1,1) 에 들어감 | 3 |  |
| 4 | 미탐색 | Blackboard::is_recent_visible(bb, game_data, game_vtable, player, e) 내부 — _gcbc g07.ll:157005 에 define 있음, 이번 범위에선 안 읽음 | 4 |  |
| 5 | 미탐색 | EntityPositioningCache::new 내부(ratio 계산) — 별도 define, 이번 범위 밖. version 인자에 poison 이 전달되므로 new 도 version 을 안 읽는 것으로 판단(LLVM 이 poison 을 넣는 조건) | 4 |  |
| 6 | 미탐색 | projectile_intersects_cell_region 의 shape 태그 3 과 move_type 기본 팔(FollowTargetShrinkBarrier/Bouncing(None)/Removed)이 '무조건 위험(false)' 인 이유 — IR 은 확정, 설계 의도 미확인 | 4 |  |
| 7 | 미탐색 | EntityType 태그 13 이 아닌 caster(비챔프 적 팀 엔티티가 player_champion 에 들어올 수 있는지) — 게이트 %296/%380/%463 은 cooldown 검사만 생략하고 ratio>0 만 본다 | 4 |  |
| 8 | 미탐색 | GameSetting 오프셋 5112/5120/5128 (tower_attack_disable_tick*) 과 4856(tps) 은 gep 정수로만 본문에 있음(C3 경고 예상 없음) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L1405 `max(hp,1) <= (damage + damage*tps/cooltime)*100` 의 소스 표기 — ×100 이 hp 백분율인지(damage 단위) 확정 못 함. IR 산식은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

