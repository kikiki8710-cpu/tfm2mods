---

### `249` base_battle_action — BattleSubPlan 기본 전투 후보 생성: 사거리(+이동 max_tick) 안 적 챔프·타워·본진공격 미니언에 Attack, 대상제약·대시가치·힐/실드/버프 조건 통과한 적/아군에 Skill·Skill2, 궁 후보 Ult 를 bumpalo Vec<SmallActionPlay> 로 반환 (배치 A = 1305~1511: 머리·Attack 3종·적 챔프 Skill·아군 Skill 전반부)

| 항목 | 값 |
|---|---|
| id | `battle__base_battle_action` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battle18base_battle_action` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle.rs:1305` |
| IR | `m02.ll` 66196~74096행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::battle::base_battle_action` · **in:game_ai::plan_legacy::sub_plan::battle** |
| 계층 | 기타 |
| exe | `cd05f0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::old::BattleSubPlanGoal) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[249]/sig/tls/<키>`)**

- `name`: null
- `role`: 접점 없음
- `key`: null
- `layout`: null
- `invalidation`: null
- `call_conditions`: null
- `evidence`: m02.ll 66196~74096 에 LocalKey/call_once/__getit/llvm.threadlocal.address 0건. 본문이 참조하는 @anon 상수 8종(94aca…의 19·26·54·268~272)은 각각 None-Effect 상수(19)·panic Location(26,268~272)·`Entity as AbstractEntity` vtable(54) 이고 fn-포인터 TLS 키 상수는 없다. ⟹ 승계 메모의 `last_stand_flags LocalKey::with · v3_beyond_enemy_line · estimate_damage_to` 는 **이 define(m02.ll:66196) 에는 없다**(적용 범위: IR 본문 grep · 형제 death_battle m15 는 미확인)

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | ptr → bumpalo::collections::vec::Vec<SmallActionPlay> (32B) | 출력 전용 · IR 속성 `dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(32)` · 함수 끝(m02.ll:72349 · battle.rs:2038 · 배치 D)에서 지역 `candidates`(%65) 를 memcpy 32B 로 1회 복사 — 그 전엔 %0 접근 0건(m02.ll 66196~74096 grep) · (배치 B) 배치 B 범위에서 ret 에 3곳 push: 1562(Skill·아군대상) · 1587(Skill·자기대상) · 1647(Skill2·적대상) \| (배치 D) 레이아웃(tcxdict 정본) +0x0 buf.ptr · +0x8 buf.a(&Bump) · +0x10 buf.cap · +0x18 len — 4 필드 전부 live | 3 |
| 1 | 1 | version | usize | AI 버전 게이트. 배치 A 안 직접 분기 1곳: battle.rs:1313 `version > 1`(icmp ugt %1,1 · m02.ll:66414). 그 외 alloca %73 에 store(66296) 되어 closure$2 환경(+16) 으로 전달 → is_enemy_well_danger(version,..) 인자(aux m02.ll:77075~) · 1506 effect_buff_target 첫 인자(load %73 · m02.ll:68026) · (배치 B) 배치 B 범위에서 직접 분기 없음. 스택 슬롯 %73(m02.ll:66296 `store i64 %1, ptr %73`)에서 다시 load 해 aoe_heal_covers_low_ally(1515·1671)·effect_buff_target(1569·1662) 의 1번째 인자로 전달 \| (배치 D) 배치 D 에선 L1949/L2009 effect_buff_target 의 첫 인자로만 전달(%73 스택 재로드 m02.ll:73031·73586). 분기 없음 | 4 |
| 2 | 2 | _rnd | &mut StdRng (DI !15700) | 미사용 — IR 인자에서 제거됨(포이즌 dbg_value 1건). gen_range 호출 사이트 0건(m02.ll 66196~74096 에 rand 콜리 없음) · (배치 B) IR define 에 없음 — 미사용으로 소거(배치 A 가 확정). 배치 B 범위에 gen_range 호출 사이트 0 \| (배치 D) 배치 D 범위 gen_range 호출 사이트 = 0 개 | 4 |
| 3 | 3 | player | &PlayerState (2528B) | IR 속성 `noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528)` — 읽기 전용. 읽는 필드 = info.team(0x930), info.position@tag(0x9c0). 클로저 환경(closure$1/$2 · effect_buff_target 등)에 포인터로 전달 · (배치 B) define 줄 속성: readonly. 배치 B 에서는 직접 필드 읽기 없음 — aoe_heal_covers_low_ally(1515·1671)·is_dash_worth(1638) 에 그대로 전달 \| (배치 D) 배치 D 에선 L1926 is_dash_worth 의 2번째 인자로만 전달 | 4 |
| 4 | 4 | data | &OperationData (24B = {cache:&AbstractGameWithCache +0, context:&GameContext +8, blackboard:&[Blackboard;2] +16}) | IR 속성 `readonly ... dereferenceable(24)` — 읽기 전용. cache→player_champion/others/game(vtable) · context→pool(bump)+expected_damage/buff 인자 · blackboard→closure$1 · (배치 B) define 줄 속성: readonly. +0 cache(&AbstractGameWithCache, %82) · +8 context(&GameContext, %102). SmallActionSkill::new/Skill2::new·is_dash_worth·aoe_heal_covers_low_ally 에 전달 \| (배치 D) +0x0 cache(&AbstractGameWithCache · %82) → cache+0x0/+0x8 = game &dyn AbstractGame 팻포인터(data %2959 / vtable %2960) · +0x8 context(&GameContext · %102). SmallActionUlt/Skill2::new · is_dash_worth 에 그대로 전달 | 4 |
| 5 | 5 | sub_goal | &BattleSubPlanGoal (16B enum · 판별자 enum+0x0 8B Direct · Trace0/Protect1/Kiting2/KitingBack3/RunAway4/Assassin5/AssassinReady6/End7 · 페이로드 focus:usize enum+0x8) | 값 전달(ArgumentPromotion). 배치 A 사용: 1312 `%4 == 3`(KitingBack), 1467 `switch %4 {3,4 → 1469 / 0,5 → 1483 / 그 외 → 1476}`. %5(focus) 는 배치 A 범위에서 참조 0건 · (배치 B) 배치 B 에서 %4 만 사용: 1627 switch(3 KitingBack·4 RunAway → 대시 금지 사전검사 / 0 Trace·5 Assassin → 검사 생략 / 그 외 → 기본 검사). 배치 A 의 %89(=%4==3 KitingBack)는 max_tick(%97) 결정에 씀. %5(focus) 는 배치 B 범위에서 미사용 \| (배치 D) 태그표(tcxdict --enum): 0 Trace·1 Protect·2 Kiting·3 KitingBack·4 RunAway·5 Assassin·6 AssassinReady·7 End. 배치 D 의 switch 2곳(L1887·L1915) \| (배치 D) L1887 get_entity_by_id(focus) 인자. RunAway/End 에선 안 읽음 | 3 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// battle.rs:0~1511 (배치 A)
// ─── 머리 (1305~1343) ───
// 1306  team = player.info.team(0x930); pos = player.info.position(0x9c0)  [team<2 아니면 panic_bounds_check]
//       champ = data.cache.player_champion[team][pos].unwrap()  [None ⟹ unwrap_failed]
// 1312  kb_hold: bool = (sub_goal.tag == 3 KitingBack) && version > 1 && base_defense_focus(player, data)
//       (IR phi %97 = max_tick: !KitingBack → 30 / KitingBack&&version<=1 → 0 / KitingBack&&version>1 → base_defense_focus ? 30 : 0)
// 1361  max_tick = if KitingBack && !kb_hold { 0 } else { 30 }   // 위 phi 와 동치
// 1316  near_allies: Vec<&Entity> = cache.player_champion[team].iter().filter_map(iter_champions closure: Some 만).filter(closure$0: a.id != champ.id && dist_sq(a,champ) < 150000²).collect_in(data.context.pool)
// 1318~1325 near_enemies_with_action: Vec<(SmallAction,&Entity)> = cache.player_champion[1-team].iter().enumerate()
//         .map(closure$1: |(i,e)| (data.blackboard[1-team].small_actions[i], e))          // Option<SmallAction> 24B · bounds i<5
//         .filter(closure$2: act.is_some() && e.is_some() && data.can_target(cache.game, player, e) && !is_enemy_well_danger(version, player, e.x, e.y))
//         .map(closure$3: (act.unwrap(), e.unwrap())).collect_in(pool)
// 1328~1329 enemy_towers: Vec<&Entity> = cache.iter_towers(1-team)[Chain<Flatten<[Option<&Entity>;6]>, Copied<Iter<&Entity>>>].filter(closure$4: t.can_target(0x6b9) && t.block_target_tick(0x6a0) == 0).collect_in(pool)
// 1331  candidates = Vec::new_in(pool)   // {ptr 8, a=pool, cap 0, len 0}
// 1333  atk_eff  = champ.attack_effect()  (0x4c0 태그 != -1)
// 1334  skl_eff  = champ.skill_effect()   (0x4f8 != -1)
// 1335  skl2_eff = if champ.level > 2 { champ.skill2_effect() } else { None }  (0x500 / 0x530 != -1)
// 1336  ult_eff  = if champ.level > 4 { champ.ult_effect() } else { None }     (0x538 / 0x568 != -1)  → 배치 D 소비
// 1339~1341 atk_base/skill_base/skill2_base: Option<u64> = X_eff.map(closure$5/6/7: |e| e.range(champ))
//       Effect::range(effect.rs:26) = e.range + e.growth_range*(champ.level-1) + champ.stat_buff_cached.range(0x438) + champ.radius()
//       Entity::radius(entity.rs:1511~1515) = if radius_mult(0x470)==0 { radius(0x680) } else { radius*(100+radius_mult)/100 }
// 1343  move_speed = champ.stat_cached.move_speed(0x640)
// ─── Attack 후보 (1345~1428) ───
// 1345  if atk_eff.is_some() && champ.can_attack() {
// 1346    for (act, e) in near_enemies_with_action {
// 1347      if !e.is_visible_from(champ) continue   // 인라인 entity.rs:1482: champ.team 태그 비트=1(비-Player) ⟹ true; 아니면 e.visible_state[champ.team.0](0x38) 태그==0
// 1351      range = Effect::range_adjust(&atk_eff, champ, e) + atk_base + e.radius()
// 1352      dist_sq = |e.x-champ.x|² + |e.y-champ.y|²
// 1355      ms = if act == SmallAction::RunAway(0) && !e.block_move() { move_speed.saturating_sub(e.move_speed) } else { move_speed }   // 1356
// 1367      max_dist = range + ms*max_tick
// 1368      if dist_sq > max_dist² continue
// 1372      candidates.push(SmallActionPlay::Attack(SmallActionAttack::new(data, e.id)))   // new = {start_tick: game.tick(), target, is_act:false}
//         }
// 1375    for t in enemy_towers {
// 1376      base = range_adjust(&atk_eff, champ, t) + atk_base + move_speed*max_tick
// 1383      max_dist = base + t.radius()
// 1385      if dist_sq(t,champ) > max_dist² continue
// 1390      if t.ty == Nexus(3) { 1391 push Attack(t.id) }
// 1396      else { remain_attack = t.hp(0x670) / expected_damage_target(&atk_eff, data.context, champ, &t as &dyn AbstractEntity)   [0 이면 div_by_zero 패닉]
// 1397             if remain_attack > 2 continue
// 1401             push Attack(t.id) }
//         }
//       }
// 1409  if atk_eff.is_some() && champ.can_attack() {      // 두 번째 독립 게이트(첫 블록에서 can_attack 거짓이면 여기서도 거짓 — 재호출)
// 1411    for m in cache.iter_minions(1-team) {            // 56B 체인 이터레이터 · Copied<Iter>::next ×3 단
// 1412      if m.ty != Minion(1) continue
// 1413      let Some(eid) = m.ty.Minion.info.nearest_enemy else continue;  e = cache.game.get_entity_by_id(eid)  [vtable +0x1f0]
// 1414      if !e.is_some_and(closure$9: e.team == Team::Player(team) && (e.ty == Nexus(3) || e.is_twin_tower()))  continue   // is_twin_tower(entity.rs:1313) = ty==Tower(2) && tower.ty(0x128) ∈ {TwinA 3, TwinB 4}
// 1418      if !m.is_visible_from(champ) continue
// 1422      max_dist = range_adjust(&atk_eff, champ, m) + atk_base + move_speed*max_tick    // 1423 + m.radius()
// 1424      if dist_sq(m,champ) > max_dist² continue
// 1428      push Attack(m.id)
//         }
//       }
// ─── Skill 후보: 적 챔프 (1432~1491) ───
// 1432  if skl_eff.is_some() && champ.can_skill() {     // skl_eff None 이면 → 배치 B(줄 1592) 로 점프(%592)
// 1433    for (act, e) in near_enemies_with_action {
// 1434      if !e.is_visible_from(champ) continue
// 1438      if !CastingTarget::check(&skl_eff.target(0x4f0), champ, e) continue
// 1442      if !champ.skill(0x580).target_constraint(cache.game, champ, e) continue     // dyn Action 슬롯 +0xc8 (divtable 58% · 추정)
// 1446      ms = if act == RunAway && !e.block_move() { move_speed.saturating_sub(e.move_speed) } else { move_speed }   // 1447
// 1452      range = range_adjust(&skl_eff, champ, e) + skill_base + e.radius()
// 1462      max_dist = range + ms*max_tick
// 1463      if dist_sq(e,champ) > max_dist² continue
// 1467      match sub_goal.tag {
//             3 KitingBack | 4 RunAway => {
// 1469          if skl_eff.ty.expected_move_on_hit() continue            // EffectType +0x68
//               if skl_eff.ty.expected_move_distance().is_some() continue // +0x58 · sret 24B 태그+0 != 0
//               → 1476 으로 진행 }
//             0 Trace | 5 Assassin => { → 1483 직행 }
//             _ (Protect1/Kiting2/AssassinReady6/End7) => { → 1476 }
//           }
// 1476      if skl_eff.ty.can_move() [+0x120] && e.ty == Champion(13) {
// 1477        skill_damage = expected_damage_target(&skl_eff, data.context, champ, &e as &dyn AbstractEntity)
// 1478        if !is_dash_worth(data, player, champ, e, skill_damage) continue
//           }
// 1483      if !champ.skill.can_activate(cache.game, champ) continue     // dyn Action +0xc0 (58% · 추정)
// 1491      push SmallActionPlay::Skill(SmallActionSkill::new(data, e.id))   // 태그 16
//         }
// ─── Skill 후보: 아군 (1494~1511, 배치 A 는 1511 까지) ───
// 1494    for a in near_allies {
// 1495      if !CastingTarget::check(&skl_eff.target, champ, a) continue
// 1499      if !champ.skill.target_constraint(cache.game, champ, a) continue
// 1504      has_heal   = skl_eff.ty.expected_heal(data.context, champ, &champ as &dyn)   != 0   // +0x40 · IR %808 = (==0), dbg DW_OP_not 1회 · 1513 분기 `br %808 → 1519(거짓) / 1513(참)` 로 극성 확정
// 1505      has_shield = skl_eff.ty.expected_shield(data.context, champ, &champ as &dyn) != 0  // +0x48 · %821 동일 형태
// 1506      buff(288B) = effect_buff_target(version, &skl_eff, data.context, champ, &champ dyn, champ, &champ dyn)
// 1507      has_buff = buff+0x48 (i32) != -1
// 1508      has_near_enemy = near_enemies_with_action.iter().any(closure$10: |(_,e)| e.is_visible_from(champ) && dist_sq(e, a) < 120000²+1)
// 1509                    || cache.others[1-team].iter().any(closure$11: |o| dist_sq(o, a) < 120000²+1)
// 1511      hp_ratio = a.hp(0x670) * 100 / a.stat_cached.hp(0x628)   [분모 0 ⟹ div_by_zero 패닉]
//           → 배치 B(줄 1513): br has_heal ? %961(1513) : %958(1519)   // has_heal/has_shield/has_buff/has_near_enemy/hp_ratio 를 넘김; push 는 1562/1587(배치 B) · 루프 되돌이 %1154 → 1494
//         }
//       }
// ─── 흐름 요약 ───  1432 게이트 거짓/1494 루프 종료 → 배치 B(1565·1592) · 모든 invoke unwind → %178(배치 D 2039 cleanup) · panic 경로 → %118 unreachable
// gen_range: 0회(_rnd 미사용)

// battle.rs:1513~1685 (배치 B)
// 문맥(배치 A 승계): champ=%86(self 챔피언 Entity) · enemies=%70 Vec<(SmallAction,&Entity)> · candidates=%72 Vec<&Entity>(스킬 아군/대상 후보) · ret=%65 · skill_effect=&champ.skill_effect(%134) · skill2_effect=%139(level>2 ? &champ.skill2_effect : None상수) · max_tick=%97(30 / KitingBack 이면 0, version>1 && base_defense_focus 면 30) · skill_base.1=%208(1340) · skill2_base.1=%237(1341) · atk_base.1=%177(1339) · game=(cache.game ptr,vtable) · ctx=data.context(%102)
// 인라인 헬퍼(이름은 tcx sp 로 확정): Effect::range(caster)=range+growth_range*(level-1)+caster.stat_buff_cached.range (effect.rs:25) · Entity::radius()=radius_mult==0 ? radius : radius*(100+radius_mult)/100 (entity.rs:1509) · Entity::is_visible_from(team)= team Neutral ? true : visible_state[team]==Visible (entity.rs:1481, 팀 인덱스 ≥2 면 panic) · Entity::distance_sq (entity.rs:2157) · EntityType::is_champion(1403)
// 「x 가 e 의 평타 사거리 안」 클로저(1530~1531 / 1575~1576 / 1686~1687 문자 단위 동형): attack_range = target_atk.range(e) + target_atk.range_adjust(e, x) + e.radius() + x.radius(); x.is_visible_from(champ.team) && e.distance_sq(x) <= attack_range²  (IR 분기 순서: 가시성 먼저, 거리 나중)

// ── [1494 루프 계속] for e in candidates(%72) — 배치 A 가 1495 CastingTarget::check · 1499 skill.target_constraint · 1504 has_heal=expected_heal!=0(%808 은 부정형) · 1505 has_shield=expected_shield!=0(%821 부정형) · 1506~1507 buff=effect_buff_target(version,&skill_effect,ctx,champ,champ) has_buff=buff.is_some()(%825) · 1508 enemy_near(%948) · 1511 hp_ratio=e.hp*100/e.max_hp(%956) 를 계산
1513: if has_heal && !has_buff && hp_ratio > 79 {                       // m02.ll:68323~68339 (%808 → %961 → %962 `icmp ugt %956, 79`)
1515:   if !aoe_heal_covers_low_ally(version, &champ.skill_effect, data, player, e) { continue }   // 68346~68353 (%968) — 낭비 힐 컷: 대상이 80% 이상이면 AOE 힐이 저체력 아군을 덮을 때만 허용
      }
1519: if has_shield && !has_buff && !enemy_near { continue }             // 68330~68343 (%960 = !has_shield||has_buff||enemy_near 가 거짓이면 %972 → %1154 continue)
1525: is_non_movespeed_buff = has_buff && buff.move_speed_mult == 0    // 68363·68381~68382 (%627=%51+0x88; closure_env$12 map_or)
1526: is_etc_buff = skill_effect.ty.etc_buff()                            // 68367~68377 (vtable+0x90)
1527: if is_non_movespeed_buff || is_etc_buff {                          // 68385~68386
1528:   let Some(target_atk) = &e.attack_effect else { continue }         // 68389~68394 (e+0x4c0 == -1 → %972)
1529:   if enemies.is_empty() || !enemies.iter().any(|(_, x)| {           // 68431~68432 (len==0 → continue) · 68445~68590 루프
1530:       attack_range = target_atk.range(e) + target_atk.range_adjust(e, x) + e.radius() + x.radius()   // 68459~68520 (%1058)
1531:       x.is_visible_from(champ.team) && e.distance_sq(x) <= attack_range²   // 68524~68581 (%1090 ugt → 다음 x / 아니면 %995 통과)
        }) { continue }                                                    // 68589~68590 (%1092 루프 끝 → %972)
      }                                                                    // 의미: 버프(이속 아닌 버프·etc 버프)는 대상 e 가 지금 때릴 수 있는 적이 있을 때만
1543: max_dist = skill_base.1 + champ.move_speed*max_tick (=%629, 배치 A 67653~67654) + skill_effect.range_adjust(champ, e)   // 68398~68400·68649 (%997·%1127)
1544: dist_sq = e.distance_sq(champ)                                       // 68620~68646 (%1126, champ x/y=%604/%605)
1553: max_dist += e.radius()                                               // 68592~68617·68650 (%1109·%1128)
1554: if dist_sq > max_dist² { continue }                                  // 68652~68654 (%1130 → %1154)
1558: if !champ.skill.can_activate(game, champ) { continue }               // 68657~68663 (vtable+0xc0, %1134)
1562: ret.push(SmallActionPlay::Skill(SmallActionSkill::new(data, e.id)))   // 68668~68722 (sret %49 → memcpy %50, store i8 16 @+0xb1, len==cap → reserve_internal_or_panic, memcpy 184B, len+=1)
    } // for e (→ %1154 → %777 루프 머리, 배치 A 1494)

// ── 자기 대상 스킬1 (루프 종료 %1155 진입)
1565: if CastingTarget::check(&skill_effect.target, champ, champ) && champ.skill.target_constraint(game, champ, champ) {   // 68730~68747 (%1156 → %1165 vtable+0xc8; 둘 중 하나라도 거짓 → %592 = 1592)
1569:   buff = effect_buff_target(version, &skill_effect, ctx, champ as &dyn AbstractEntity, champ as &dyn AbstractEntity)   // 68752~68754 sret %48 (DI: skip_self_buff 초기값 false)
1570:   is_non_movespeed_buff = buff.is_some() && buff.move_speed_mult == 0   // 68760~68763·68781~68783 (%48+72 != -1 · %48+136 == 0)
1571:   is_etc_buff = skill_effect.ty.etc_buff()                            // 68767~68777
1572:   skip_self_buff = if is_non_movespeed_buff || is_etc_buff {          // 68786~68787
1573:       if champ.attack_effect.is_none() (배치 A %130) { true }          // 68791 (→ %1430 skip_self_buff=1)
1574:       else { let target_atk = &champ.attack_effect(%127); !enemies.iter().any(|(_, x)| {   // 68804~68830 (len==0 → %1430 skip) · 루프 4변종(%1233/%1278/%1326/%1376 = champ.team Neutral/Player × champ.radius_mult==0/≠0 호이스팅, 68861~69249)
1575:           attack_range = target_atk.range(champ) + target_atk.range_adjust(champ, x) + champ.radius() + x.radius()   // %1256/%1301/%1354/%1404
1576:           x.is_visible_from(champ.team) && champ.distance_sq(x) <= attack_range²   // 어느 x 라도 참 → %1191(1586) / 전부 거짓 → %1430(skip)
            }) }
        } else { false };                                                   // 1572 거짓 → 곧장 %1191
1586:   if !skip_self_buff && champ.skill.can_activate(game, champ) {       // 69279~69284 (%1430: skip → %592 / %1191 vtable+0xc0 %1194 → %1431)
1587:       ret.push(SmallActionPlay::Skill(SmallActionSkill::new(data, champ.id)))   // 69289~69343 (태그 16)
        }
      }

// ── 스킬2 (%592: 배치 A %142 = skill2_effect None 이면 %1452 = 배치 C 줄 1761 로 점프)
1592: if skill2_effect.is_some() && champ.can_skill2() {                  // 67566(%142) · 69348~69388 (%1451 거짓 → %1452 배치 C 1761)
1593:   for (a, e) in enemies(%70).iter() {                                // 69394~69432 (원소 32B: a=SmallAction@+0, e=&Entity@+0x18)
1594:     if !e.is_visible_from(champ.team) { continue }                    // 69445~69491 (%1493 Neutral → 검사 생략 / %1514 visible_state[team]==0)
1598:     if !CastingTarget::check(&skill2_effect.target, champ, e) { continue }   // 69498~69502 (%1475=%139+40)
1602:     if !skill2_action.target_constraint(game, champ, e) { continue }   // 69505~69515 (%1477: level>2 ? champ.skill2(0x590) : champ.empty(0x5b0); vtable+0xc8)
1606:     move_speed = if *a == SmallAction::RunAway && !e.block_move() {   // 69518~69531 (a@+0 == 0 · Entity::block_move)
1607:                      champ.move_speed(%239).saturating_sub(e.stat_cached.move_speed) } else { champ.move_speed }   // 69534~69543 (llvm.usub.sat)
1612:     max_dist = skill2_effect.range_adjust(champ, e) + skill2_base.1(%237)   // 69544~69548 (%1540·%1542)
1613:     dist_sq = e.distance_sq(champ)                                     // 69573~69599 (%1574)
1622:     max_dist += move_speed*max_tick + e.radius()                       // 69549~69571·69602~69604 (%1575·%1557·%1577)
1623:     if dist_sq > max_dist² { continue }                                // 69606~69608 (%1579 → %1654)
1627:     match sub_goal(%4) {                                               // 69611~69616 switch
            KitingBack(3) | RunAway(4) => {
1629:         if skill2_effect.ty.expected_move_on_hit() || skill2_effect.ty.expected_move_distance().is_some() { continue }   // 69621~69659 (vtable+0x68 → 참이면 %1654 / vtable+0x58 sret %45 태그!=0 → %1654) → 통과하면 1636 으로
            }
            Trace(0) | Assassin(5) => { /* 1636 검사 생략 → 1643 */ }        // → %1608
            _ (Protect 1 · Kiting 2 · AssassinReady 6 · End 7) => { /* 1636 */ }   // → %1614
          }
1636:     if skill2_effect.ty.can_move() && e.is_champion() {                // 69672~69693 (vtable+0x120 %1625 · e+0x68==13) — 둘 중 하나 거짓 → 1643
1637:       skill_damage = skill2_effect.expected_damage_target(ctx, champ as &dyn AbstractEntity, e)   // 69696~69697 (%1632)
1638:       if !is_dash_worth(data, player, champ, e, skill_damage) { continue }   // 69701~69705 (%1634 거짓 → %1654)
          }
1643:     if !skill2_action.can_activate(game, champ) { continue }           // 69662~69667·69708 (vtable+0xc0 %1613)
1647:     ret.push(SmallActionPlay::Skill2(SmallActionSkill2::new(data, e.id)))   // 69713~69766 (태그 17)
        } // for (a,e) (→ %1654 → %1485)

// ── 스킬2 아군/대상 후보 루프 (전반부 — 1699 이후는 배치 C)
1650:   for e in candidates(%72).iter() {                                   // 69453~69476·69774~69783 (%1494 진입 블록에서 %1501=cache.others[상대팀]·%1505=%239*%97+%237(=skill2_base.1+move_speed*max_tick, 배치 C 가 소비) 호이스팅)
1651:     if !CastingTarget::check(&skill2_effect.target, champ, e) { continue }   // 69790~69794 (→ %2034 → %1655)
1655:     if !skill2_action.target_constraint(game, champ, e) { continue }   // 69797~69808
1660:     has_heal = skill2_effect.ty.expected_heal(ctx, champ as &dyn AbstractEntity) != 0     // 69813~69828 (%1686 = ==0 = !has_heal, DW_OP_not 명명)
1661:     has_shield = skill2_effect.ty.expected_shield(ctx, champ) != 0                        // 69831~69846 (%1699 부정형)
1662:     buff = effect_buff_target(version, skill2_effect, ctx, champ, champ)   // 69848~69850 sret %42 ⚠caster·target 둘 다 champ(e 아님)
1663:     has_buff = buff.is_some()                                          // 69854~69856 (%42+72 != -1, %1703)
1664:     enemy_near = enemies.iter().any(|(_, x)| x.is_visible_from(champ.team) && x.distance_sq(e) <= 120000²)   // 69859~70042 (len==0 → 바로 1665; Neutral 변종 %1723 / Player 변종 %1767; `icmp ult dist_sq, 14400000001`)
1665:               || cache.others[상대팀].iter().any(|x| x.distance_sq(e) <= 120000²)   // 70062~70129 (가시성 검사 없음) → %1826 phi
1667:     hp_ratio = e.hp*100 / e.stat_cached.hp                              // 70134~70144 (max_hp==0 → panic_const_div_by_zero)
1669:     if has_heal && !has_buff && hp_ratio > 79 {                         // 70145~70161
1671:       if !aoe_heal_covers_low_ally(version, skill2_effect, data, player, e) { continue }   // 70168~70175 (%1846)
          }
1675:     if has_shield && !has_buff && !enemy_near { continue }             // 70152~70154·70164~70165 (→ %1850 → %2034 continue)
1681:     is_non_movespeed_buff = has_buff && buff.move_speed_mult == 0       // 70185·70203~70204 (%1503=%42+136)
1682:     is_etc_buff = skill2_effect.ty.etc_buff()                           // 70189~70199
1683:     if is_non_movespeed_buff || is_etc_buff {                          // 70207~70208 (거짓 → %1873 배치 C 줄 1699)
1684:       let Some(target_atk) = &e.attack_effect else { continue }         // 70211~70216 (e+0x4c0 == -1 → %1850)
1685:       if enemies.is_empty() || !enemies.iter().any(|(_, x)| {           // 70225~70254·70267~70412
1686:           attack_range = target_atk.range(e) + target_atk.range_adjust(e, x) + e.radius() + x.radius()   // %1936
1687:           x.is_visible_from(champ.team) && e.distance_sq(x) <= attack_range²   // 참 → %1873(배치 C 1699) / 전부 거짓 → %1850 continue
            }) { continue }
          }
          → 배치 C(줄 1699: skill2_effect.range_adjust(champ,e) … 1718 push Skill2 · %2029 는 1650 루프 back-edge)
        } // for e → 루프 종료 %2035 = 배치 C 줄 1721(자기 대상 스킬2)
      } // if can_skill2 — 거짓 경로 %1452 = 배치 C 줄 1761

// 다른 배치로 넘어가는 지점: %972/%1154(→배치 A 1494 루프 머리 %777) · %118(패닉 후 unreachable, 배치 A) · %178(unwind cleanup, 배치 D 2039) · %1452(배치 C 1761) · %1873(배치 C 1699) · %2035(배치 C 1721) · %1850/%2034(1650 루프 continue, 배치 A/C 소유 블록이나 실체는 back-edge)
// gen_range 호출 사이트: 0 (_rnd 는 define 에서 소거)

// battle.rs:1697~1862 (배치 C)
// 문맥(다른 배치 정의 값): champ=%86(내 챔피언 Entity, 배치 A 1306) · data=%3 · cache=%82 · game=(cache+0, cache+8) · ctx=%102 · max_tick=%97(배치 A 1312~1361: 기본 30, sub_goal==3 이고 (version<=1 || !base_defense_focus) 이면 0)
//   atk_base=%177 · skill_base=%208 · skill2_base=%237 (배치 A 1339~1341: effect.range + growth_range*(level-1) + champ.stat_buff_cached.range + champ.radius(); None 이면 undef 이나 각 게이트 %130/%133/%142 로 보호)
//   move_speed=%239(stat_cached.move_speed 0x640, 배치 A 1343) · MOVE=%1461=move_speed*max_tick · %1505=MOVE+skill2_base · %1462=MOVE+atk_base
//   play=%65 (로컬 bumpalo Vec<SmallActionPlay>, 배치 A 1331) · nearby=%72 (배치 A 1315: 자기 제외·dist²<150000² 챔피언 Vec<&Entity>) · enemy_acts=%70 (배치 A 1318: 적팀 챔피언 (SmallAction,&Entity) Vec)
//   dist_sq(a,b) = |a.x-b.x|² + |a.y-b.y|² (Entity +0x660/+0x668) · e.radius() = radius_mult==0 ? radius : radius*(radius_mult+100)/100 (entity.rs:1509~1515 인라인)

// ===== [A] 1699~1718 : 배치 B 1650 루프(for e in nearby) 본체의 꼬리 — 게이트 1592(배치 B: skill2_effect.is_some && champ.can_skill2()) 안 =====
// 배치 B(줄 1696 이전) → 1873(m02.ll:70218)
1699: adj = skill2_effect.range_adjust(champ, e)                                  // %1875
1700: dist_sq = dist_sq(e, champ)                                                  // %2004 (Entity::x/y)
1709: max_dist = (MOVE + skill2_base) + adj + e.radius()                          // %2006 = %1505 + %1875 + %1987
1710: if dist_sq > max_dist*max_dist  → continue(→2034→1655, 배치 B 루프 헤드)
1714: if !champ.skill2_action().can_activate(game, champ)  → continue            // vtable+0xc0 · skill2_action = level>2 ? &*skill2(0x590) : &*empty(0x5b0)
1718: play.push(SmallActionPlay::Skill2(SmallActionSkill2::new(data, e.id)))      // 태그 17 · 루프 계속(break 없음 → 사거리 내 적 챔피언마다 1개)
// 루프 소진(1655 %1657) → 2035(70553)

// ===== [B] 1721~1754 : 스킬2 자기 대상(자기 버프) 후보 =====
1721: if skill2_effect.target.check(champ, champ)                                  // CastingTarget::check(self=%139+0x28)
      && champ.skill2_action().target_constraint(game, champ, champ) {            // vtable+0xc8 (5인자, i1) — 둘 다 거짓이면 → 1452(1761)
1725:   buff: Option<BuffState> = effect_buff_target(version, skill2_effect, ctx, champ as &dyn(vt anon.54), champ as &dyn)   // sret 288B %39
1726:   has_shield = skill2_effect.ty.expected_shield(ctx, champ as &dyn) != 0    // vtable+0x48 → i64 %2060; IR %2062=(==0), DI has_shield=NOT(%2062)
1727:   is_non_movespeed_buff = buff.map_or(false, |b| b.move_speed_mult == 0)    // closure#20 인라인: tag(+0x48)!=-1 && (+0x88)==0
1728:   is_etc_buff = skill2_effect.ty.etc_buff()                                  // vtable+0x90 → i1 %2077
        skip_self_buff = false
1729:   if is_non_movespeed_buff || is_etc_buff {                                  // %2082 = or (한 줄이라 IR 상 동시 평가: etc_buff 호출은 1728 에서 무조건 실행됨)
1731:     // 분기 방향: %2062(shield==0) → 2096(1740) / 아니면(shield!=0) → 2090(1732)
          has_enemy_in_attack_range =
1732:       if has_shield && let Some(pa) = champ.skill2_action().as_any().downcast_ref::<PrisonerSkill2Action>()   // vtable+0x68 as_any → Any vtable+0x18 type_id == TypeId 상수
1734:            { pa.has_enemy_champion_target_or_action_threat(game, champ) }   // closure#21 자리(1734:20) · game_core prisoner.rs
1740:       else if let Some(target_atk) = champ.attack_effect                      // %130 None → skip_self_buff=true (2114)
1741:            { enemy_acts.iter().any(|(_, e)| {                                 // closure#22 = aux m02.ll:3170~3628
1742:                 attack_range = target_atk.base_range(champ) + target_atk.range_adjust(champ, e) + e.radius();   // base_range = range + growth*(level-1) + champ.buff.range + champ.radius() (aux 3188~3209)
1743:                 e.is_visible_from(champ.team) && dist_sq(champ, e) <= attack_range²   // aux: 둘 다 필요. IR 순서 = range_adjust 호출(3425) → 가시성 분기(3457) → 거리 분기(3492) — 소스 &&, 좌우 순서는 표기 불가; Neutral 팀이면 가시성 검사 자체가 없음(3220)
                   }) }
            else { skip_self_buff = true; → 1452 }
1748:     if !has_enemy_in_attack_range { skip_self_buff = true → 1452 }           // 2115/2116 → 2114
        }
1753:   if !skip_self_buff && champ.skill2_action().can_activate(game, champ)      // vtable+0xc0 (2084)
1754:      play.push(SmallActionPlay::Skill2(SmallActionSkill2::new(data, champ.id)))   // 자기 자신 대상 · 태그 17
      }
// → 1452(m02.ll:69351): 배치 B 블록도 여기로 직접 진입한다 — 592(줄 1592 `%142` skill2_effect None → 1452) · 1468(1592 `%1451` !can_skill2 → 1452) — 즉 [A][B] 는 통째로 건너뛰어질 수 있다(preds: 2131·2117·2114·2046·2037·1468·592)

// ===== [C] 1761~1792 : for e in cache.jungles (Vec<&Entity> 0xd0) — 중립 에픽 막타 =====
1761: for e in cache.jungles.iter() {                                              // %2136 루프
1762:   if !e.can_target(champ 팀 기준) continue     // entity.rs:1477 can_target 인라인(인자 형태는 &Entity 인지 &TeamType 인지 표기 불가): e.can_target(bool 0x6b9) && e.block_target_tick(0x6a0)==0 && (champ.team==Neutral || e.visible_state[champ.team.0].tag==0 Visible)
1765:   if !e.ty.is_epic() continue          // ty.tag ∈ {5 Epic, 6 Serpen}
1768:   hp_ratio = e.hp*100 / max(e.stat_cached.hp, 1)
1769:   if hp_ratio > 20 continue            // ★20% 이하 에픽만
1773:   if attack_effect.is_some(%130) && champ.can_attack() {
1774:     max_dist = (MOVE + atk_base) + attack_effect.range_adjust(champ, e) + e.radius()   // %2224
1778:     if dist_sq(e, champ) <= max_dist² {
1779:        play.push(Attack(e.id)) } }     // 태그 15
1783:   if skill_effect.is_some(%133) && champ.can_skill() && skill_effect.target.check(champ, e) {
1784:     range = skill_base + skill_effect.range_adjust(champ, e) + e.radius()     // ⚠이동 항 없음(%2256 = %2254 + %208)
1785:     if dist_sq(e, champ) <= range² {
1786:        play.push(Skill(e.id)) } }      // 태그 16
1789:   if skill2_effect.is_some(%142) && champ.can_skill2() && skill2_effect.target.check(champ, e) {
1790:     range = skill2_base + skill2_effect.range_adjust(champ, e) + e.radius()   // 이동 항 없음
1791:     if dist_sq(e, champ) <= range² {
1792:        play.push(Skill2(e.id)) } }     // 태그 17
      }

// ===== [D] 1798~1831 : for e in cache.jungles (같은 벡터 재순회) — 적 정글캠프 한 방 처치(스틸) =====
// 루프 진입 전(2139): MOVE15 = move_speed*15 + atk_base (%2141)
1798: for e in cache.jungles.iter() {
1799:   if !e.can_target(champ) continue     // 1762 와 동일 인라인
1803:   if !e.ty.is_jungle(enemy_team) continue   // ty.tag==4 && ty@Jungle.info.camp_type.0(+0x98) == %104(적팀)
1807:   if attack_effect.is_some && champ.can_attack() {
1808:     expected_dmg = attack_effect.expected_damage_target(ctx, champ as &dyn(anon.54), e)
1809:     if e.hp <= expected_dmg {         // IR: hp > expected_dmg 면 건너뜀 → 한 방에 죽는 캠프만
1810:       max_dist = MOVE15 + attack_effect.range_adjust(champ, e) + e.radius()
1812:       if dist_sq(e, champ) <= max_dist² {
1813:          play.push(Attack(e.id)) } } }
1817:   if skill_effect.is_some && champ.can_skill() && skill_effect.target.check(champ, e) {
1818:     expected_dmg = skill_effect.expected_damage_target(ctx, champ as &dyn, e)
1819:     if e.hp <= expected_dmg {
1820:       range = skill_base + skill_effect.range_adjust(champ, e) + e.radius()   // 이동 항 없음
1821:       if dist_sq <= range² {
1822:          play.push(Skill(e.id)) } } }
1826:   if skill2_effect.is_some && champ.can_skill2() && skill2_effect.target.check(champ, e) {
1827:     expected_dmg = skill2_effect.expected_damage_target(ctx, champ as &dyn, e)
1828:     if e.hp <= expected_dmg {
1829:       range = skill2_base + skill2_effect.range_adjust(champ, e) + e.radius()
1830:       if dist_sq <= range² {
1831:          play.push(Skill2(e.id)) } } }
      }

// ===== [E] 1838~1862 : for e in cache.others[enemy_team] (0xf0 + 32*%104) — 적팀 기타 엔티티 =====
// 루프 진입 전(2625): MOVE_S = MOVE + skill_base (%2634) · MOVE_S2 = MOVE + skill2_base (%2636)
1838: for e in cache.others[enemy_team].iter() {
1839:   if !(data.can_target(game, player, e) && e.is_visible_from(champ.team)) continue   // OperationData::can_target(self=%3, game data, game vtable, player %2, e) 먼저, 그 다음 가시성 인라인(1482)
1845:   if attack_effect.is_some && champ.can_attack() {
1846:     max_dist = (MOVE + atk_base) + attack_effect.range_adjust(champ, e) + e.radius()
1848:     if dist_sq(e, champ) <= max_dist² {
1849:        play.push(Attack(e.id)) } }
1853:   if skill_effect.is_some && champ.can_skill() && skill_effect.target.check(champ, e) {
1854:     max_dist = MOVE_S + skill_effect.range_adjust(champ, e) + e.radius()      // ★여기서는 스킬에도 이동 항 포함([C][D] 와 다름)
1856:     if dist_sq <= max_dist² {
1857:        play.push(Skill(e.id)) } }
1861:   if skill2_effect.is_some && champ.can_skill2() && skill2_effect.target.check(champ, e) {
1862:     max_dist = MOVE_S2 + skill2_effect.range_adjust(champ, e) + e.radius()    // %2810 · e.radius() 합산 %2811 은 줄 1863 → 배치 D(줄 1863~)
           → 배치 D(줄 1864 dist 판정 · 1865 push Skill2) — 블록 2808(m02.ll:72247) MIXED
      }
// 루프 소진 → 2647 (배치 D, 줄 1863 이후)

// 사장 코드: reach(version=2, gamemode=0) 결과 이 함수 사장 0 · 접힌 분기 0 — 배치 C 범위에 NA 없음
// rnd: gen_range 호출 사이트 0 (_rnd 는 IR define 에서 제거됨)

// battle.rs:1863~2039 (배치 D)
// 전제(배치 A~C 가 만든 값을 재사용 · 값 정의는 그쪽 명세): champ=%86(player 의 챔피언 &Entity) · candidates=%65(bumpalo Vec<SmallActionPlay>) · near_allies=%72(Vec<&Entity>) · near_enemies_with_action=%70(Vec<(SmallAction<&Entity>, &Entity)>) · move_speed=%239=champ.stat_cached.move_speed · max_tick=%97 · ult_effect=%145(level>4 ? &champ.ult_effect : 빈 Effect 상수) · ult_none=%148(ult_effect.casting@tag==-1) · champ_attack_none=%130 · game=(cache.game.data, cache.game.vtable) · ctx=data.context.
// ★max_tick: 소스는 L1904·L1987 에 `let max_tick` 을 다시 선언하지만(DILocalVariable 11곳: 1361·1378·…·1904·1987) IR 은 전부 %97 하나로 CSE 됨 = `if sub_goal==KitingBack(3) { if version>1 && base_defense_focus(player,data) {30} else {0} } else {30}` (m02.ll:66406~66423 · 배치 A 범위 · 여기선 값만 씀).
// 헬퍼 인라인 표기: dist_sq(a,b)=|ax-bx|²+|ay-by|²(entity.rs:2158→utils.rs:7~9) · radius(e)= e.radius_mult==0 ? e.radius : e.radius*(100+e.radius_mult)/100 (entity.rs:1511~1515) · eff_range(eff,caster)= eff.range + eff.growth_range*(caster.level-1) + caster.stat_buff_cached.range (effect.rs:26) · is_visible_from(t, viewer)= viewer.team 이 Neutral ? true : t.visible_state[viewer.team_idx]==Visible(0) (entity.rs:1482~1483 · data.rs:122).

// ---- L1862~1865: 배치 C 의 L1838 루프(`for e in <리스트> { if !data.can_target(game, player, e) {continue} … }` · skill2 후보) 꼬리 — 블록 %2808 은 루트 L1862 로 시작해 배치 C 에 귀속되지만 L1863·L1864 IR 이 그 안에 있다.
L1863: max_dist = (배치 C 부분합 %2810 = (move_speed*max_tick + skill2_base%237) + range_adjust(champ.skill2_effect, champ, e)) + radius(e)      // m02.ll:72251 (배치 D 귀속 IR 1줄)
L1864: if dist_sq(e, champ) > max_dist*max_dist { continue → %2649 (배치 C 루프 헤더 L1838) }      // m02.ll:72280~72282
L1865: candidates.push(SmallActionPlay::Skill2(SmallActionSkill2::new(data, e.id)))   // 태그 17 @+0xb1 · cap==len 이면 reserve_internal_or_panic(…,1,true) · memcpy 184B · len+=1 → %2649 (배치 C)

// ---- L1870: 궁극기 게이트
L1870: if ult_none(%148) { return candidates }          // %2647→%2850 (L2038 sret memcpy)
       if !champ.can_ult() { return candidates }         // %2848/%2885 → %2850

// ---- L1871~1872: 갬블러 궁 판정
L1871: ult_action: &dyn Action = if champ.level>4 { &*champ.ult } else { &*champ.empty }   // select %143 ? +0x5a0 : +0x5b0 (entity.rs:1677 Entity::ult 인라인)
       any = ult_action.as_any()(vtable+0x68) ; tid = any.type_id()(Any vtable+0x18 · sret 16B)
L1872: is_v16_gambler_cc_ult = (tid == TypeId::<GamblerUltAction>) && (gambler_ult.charm_duration(+0x10) != 0)   // closure$23 인라인 · Option::is_some_and(option.rs:661)
L1888(호이스팅): focus_radius = if is_v16_gambler_cc_ult { 8100000000 (90000²) } else { 36000000 (6000²) }   // %2910 phi: 비-갬블러 → 36000000 · 갬블러 → select(charm==0 ? 36000000 : 8100000000)

// ---- L1874~1935: 적 대상 궁 후보 (for (a, e) in near_enemies_with_action.iter()  // a=SmallAction<&Entity>(원소+0x0) · e=&Entity(원소+0x18) · 원소 32B)
L1875: if !e.is_visible_from(champ) { continue }                       // champ Neutral 이면 검사 생략(항상 통과)
L1879: if !CastingTarget::check(&ult_effect.target, champ, e) { continue }
L1883: if !ult_action.target_constraint(game, champ, e) { continue }    // Action vtable+0xc8 · (self, game.data, game.vtable, caster, target) -> bool
L1887: match sub_goal { Trace|Protect|Kiting|KitingBack|Assassin|AssassinReady (0·1·2·3·5·6) => {
           if let Some(focus) = game.get_entity_by_id(sub_goal.focus)(vtable+0x1f0) {
L1889:         if !(dist_sq(e, focus) < focus_radius) { continue }      // `icmp ult %2992, %2910` — focus 에서 focus_radius(제곱) 미만이어야 통과 (경계 = 같으면 탈락)
           } }
         RunAway|End (4·7) => {} }
L1894: move_speed_e = if a.tag == RunAway(0) && !e.block_move() { move_speed.saturating_sub(e.stat_cached.move_speed) } else { move_speed }   // 도망치는 적은 상대 속도로 추격 여력 계산 (block_move 면 적이 못 움직이니 내 이속 그대로)
L1900: range = eff_range(ult_effect, champ) + range_adjust(ult_effect, champ, e) + radius(champ) + radius(e)
L1901: dist_sq = dist_sq(e, champ)
L1904: max_tick = %97 (위 ★)
L1910: max_dist = range + move_speed_e * max_tick
L1911: if dist_sq > max_dist*max_dist { continue }
L1915: match sub_goal {
         KitingBack|RunAway (3·4) => {
L1917:       if ult_effect.ty.expected_move_on_hit()(EffectType vtable+0x68) { continue }
             if ult_effect.ty.expected_move_distance()(vtable+0x58 · sret 24B) 의 첫 8B != 0 { continue }   // 이동을 유발하는 궁은 후퇴 중엔 배제
             → L1924 로 }
         Trace|Assassin (0·5) => → L1931 로 (대시 가치 검사 생략)
         _ (1·2·6·7) => → L1924 로 }
L1924: if ult_effect.ty.can_move()(vtable+0x120) && e.ty.tag == Champion(13) {
L1925:     skill_damage = Effect::expected_damage_target(ult_effect, ctx, (champ as &dyn AbstractEntity), e)   // caster 팻포인터 = (champ, @anon…54) · target = e
L1926:     if !is_dash_worth(data, player, champ, e, skill_damage) { continue } }
L1931: if !ult_action.can_activate(game, champ)(Action vtable+0xc0) { continue }
L1935: candidates.push(SmallActionPlay::Ult(SmallActionUlt::new(data, e.id)))   // 태그 18 · 인라인 push
// 루프 끝 → L1938

// ---- L1938~2002: 아군 대상 궁 후보 (for e in near_allies.iter()  // Vec<&Entity> · 원소 8B)
// 루프 진입 전 호이스팅: caster_extra = move_speed*60 (%2940 · dbg L1964) · champ.attack_effect.range/growth 주소(%2941/%2942)
L1939: if !CastingTarget::check(&ult_effect.target, champ, e) { continue }
L1943: if !ult_action.target_constraint(game, champ, e) { continue }
L1949: buff: Option<BuffState> = effect_buff_target(version, ult_effect, ctx, (champ as dyn), (champ as dyn))   // ⚠caster·target 둘 다 champ(e 아님) — IR m02.ll:73032 그대로
L1950: is_etc_buff = ult_effect.ty.etc_buff()(vtable+0x90)
L1954: if is_etc_buff || buff.is_some() { → L1963 }                      // `or i1`(비단락) · 소스 순서는 컬럼 부재로 불명, 의미상 무관
       else {
L1955:   gate_applies = (ult_effect.ty.expected_on_attack_damage(ctx, champ)(vtable+0xb0) == 0)
L1956:   if gate_applies { → L1983 (교전권 검사 없이 사거리 검사로) } else { → L1963 } }
L1963: engage_extra = e.stat_cached.move_speed * 60
L1964: caster_extra = champ.stat_cached.move_speed * 60 (%2940)
L1965: target_atk = e.attack_effect ; if None(casting@tag==-1) { continue }   // %3196→%3351→%3432
L1966: has_enemy_in_attack_range = near_enemies_with_action.iter().any(|(_, x)| {   // 인라인 클로저(iterator.rs:331~332) · 빈 리스트면 false → continue
L1967:     attack_range = eff_range(target_atk, e) + engage_extra + range_adjust(target_atk, e, x) + radius(e) + radius(x)
L1968~1970: caster_range = (champ.attack_effect.map(|atk| eff_range(atk, champ) + range_adjust(atk, champ, x)).unwrap_or(0)) + caster_extra + radius(champ) + radius(x)
L1971:     x.is_visible_from(champ) && ( dist_sq(x, e) <= attack_range²
L1972:                                 || dist_sq(x, champ) <= caster_range² ) })   // 각각 `icmp ugt` 의 거짓 방향 = 통과
       if !has_enemy_in_attack_range { continue }      // %3351(dbg line 1 아티팩트 · lifetime.end L1981) → %3432
L1983: range = eff_range(ult_effect, champ) + range_adjust(ult_effect, champ, e) + radius(champ) + radius(e)
L1984: dist_sq = dist_sq(e, champ)
L1987: max_tick = %97
L1993: max_dist = range + move_speed * max_tick        // %1461 = %239*%97 (호이스팅 · 적 루프와 달리 상대 속도 보정 없음)
L1994: if dist_sq > max_dist*max_dist { continue }
L1998: if !ult_action.can_activate(game, champ) { continue }
L2002: candidates.push(SmallActionPlay::Ult(SmallActionUlt::new(data, e.id)))   // 태그 18
// 루프 끝 → L2005

// ---- L2005~2033: 자기 대상 궁 후보
L2005: if !(CastingTarget::check(&ult_effect.target, champ, champ) && ult_action.target_constraint(game, champ, champ)) { return candidates }   // 각각 거짓이면 %2850
L2007: skip_self_buff = false
L2009: buff = effect_buff_target(version, ult_effect, ctx, (champ as dyn), (champ as dyn))
L2010: is_etc_buff = ult_effect.ty.etc_buff()
L2014: if is_etc_buff || buff.is_some() { → L2018 } else {
L2015:   gate_applies = (ult_effect.ty.expected_on_attack_damage(ctx, champ) == 0)
L2016:   if gate_applies { → L2032 } else { → L2018 } }
L2018: engage_extra = move_speed * 60 (%2940 재사용)
L2019: target_atk = champ.attack_effect ; if None(%130) { skip_self_buff = true → return candidates }
L2020: has_enemy_in_attack_range = near_enemies_with_action.iter().any(|(_, x)| {   // 빈 리스트면 false
L2021:     attack_range = eff_range(target_atk, champ) + engage_extra + range_adjust(target_atk, champ, x) + radius(champ) + radius(x)
L2022:     x.is_visible_from(champ) && dist_sq(x, champ) <= attack_range² })
       if !has_enemy_in_attack_range { skip_self_buff = true → return candidates }   // %3571 → %2850
L2032: if !ult_action.can_activate(game, champ) { return candidates }     // %3478/%3572 → %2850
L2033: candidates.push(SmallActionPlay::Ult(SmallActionUlt::new(data, champ.id)))   // 태그 18 · out-of-line Vec::push(m02.ll:73850)
L2038: return candidates   // %2850: memcpy(sret, candidates, 32)
L2039: 스코프 종료 — near_enemies_with_action(%70)·near_allies(%72)·enemy_towers(%68) 의 bumpalo Vec drop(원소 drop → 최상단 청크 반환 경로: footer+0x20 == ptr 이면 ptr += len*elem) · 언와인드 cleanuppad %116/%120/%178 는 candidates(%65)·%68·%70·%72 drop_glue.
// 배치 D 의 gen_range 호출 사이트: 0 개(_rnd 는 IR define 에서 제거됨).
```

**`mem` 메모리 접근 188건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState(player) | 0x930 | info.team | r | 1306 champ 인덱스 · bounds<2 (panic_bounds_check) | 4 | OK |  |
| 1 | PlayerState(player) | 0x9c0 | info.position@tag (i32→zext) | r | 1306 player.rs:581 getter 인라인 · champ 인덱스 | 4 | OK |  |
| 2 | OperationData(data) | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 3 | OperationData(data) | 0x8 | context | r | &GameContext · expected_damage_target/effect_buff_target/expected_heal/shield 인자 | 4 | OK |  |
| 4 | OperationData(data) | 0x10 | blackboard | r | &[Blackboard;2] · closure$1 환경(+24) | 4 | OK |  |
| 5 | GameContext | 0x0 | pool (&Bump) | r | 1316/1331 bumpalo Vec 할당자 | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 팻포인터 앞 절반 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 뒤 절반 · 1413 슬롯 +0x1f0 get_entity_by_id(divtable) · 1442/1483/1499 Action 슬롯 인자 | 3 | OK |  |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] Option<&Entity> | r | 1306 [team][pos] · 1316 [team] 슬라이스(5·stride 8=0x28) · 1319 [1-team] 슬라이스 | 4 | OK |  |
| 9 | AbstractGameWithCache | 0xf0 | others[2] Vec<&Entity> | r | 1509 others[1-team].iter() (원소 32B stride · +0 ptr · +0x18 len) | 4 | OK |  |
| 10 | Blackboard[1-team] | 0x78 | small_actions[5] Option<SmallAction> (24B · 태그 +0 · -1=None) | r | aux m12.ll:22263~ closure$1: enumerate idx i 로 [i] 읽어 (act, &Option<&Entity>) 쌍 생성 · bounds<5 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 11 | Entity(champ) | 0x0 | team@tag | r | is_visible_from 인라인(1347/1418/1434/1508): 태그 하위비트 1(비-Player)이면 가시로 간주 | 4 | OK |  |
| 12 | Entity(champ) | 0x8 | team.Player.0 | r | visible_state 인덱스 · bounds<2 | 4 | OK |  |
| 13 | Entity(대상) | 0x38 | visible_state[team]@tag | r | == 0 이면 champ 팀에 가시(data.rs:122 is_visible) · [team] stride 8 | 4 | OK |  |
| 14 | Entity(대상) | 0x68 | ty@tag (EntityType) | r | 1390 ==3 Nexus · 1412 ==1 Minion · 1414 switch 3 Nexus/2 Tower · 1476 ==13 Champion | 4 | OK |  |
| 15 | Entity(미니언) | 0x88 | ty.Minion.info.nearest_enemy@tag | r | 1413 Option<usize> is_some (Minion 0x70 + 0x18) | 4 | OK |  |
| 16 | Entity(미니언) | 0x90 | ty.Minion.info.nearest_enemy.0 | r | 1413 → get_entity_by_id 인자 | 4 | OK |  |
| 17 | Entity(타워) | 0x128 | ty.Tower.info.ty@tag (TowerType 1B) | r | 1414 is_twin_tower 인라인: (v-3) <u 2 ⟹ TwinA(3)\|TwinB(4) | 4 | OK |  |
| 18 | Entity(champ) | 0x438 | stat_buff_cached.range | r | Effect::range 인라인(1339~1341) | 4 | OK |  |
| 19 | Entity(any) | 0x470 | stat_buff_cached.radius_mult (i32) | r | Entity::radius 인라인: 0 이면 radius 그대로 | 4 | OK |  |
| 20 | Entity(champ) | 0x490 | attack_effect.Some.0 (Effect 56B 시작) | r | range_adjust/expected_damage_target 첫 인자 %127 | 4 | OK |  |
| 21 | Entity(champ) | 0x4a0 | attack_effect.range | r | 1339 | 4 | OK |  |
| 22 | Entity(champ) | 0x4a8 | attack_effect.growth_range | r | 1339 ×(level-1) | 4 | OK |  |
| 23 | Entity(champ) | 0x4c0 | attack_effect@tag (니치 i32 · -1=None) | r | 1333 · %130 | 4 | OK |  |
| 24 | Entity(champ) | 0x4c8 | skill_effect.Some.0 (Effect) | r | %134 · 1452 range_adjust · 1477 expected_damage_target · 1506 effect_buff_target | 4 | OK |  |
| 25 | Entity(champ) | 0x4d0 | skill_effect.ty.vtable_ptr (Arc<dyn EffectType> 뒤 절반) | r | 1469/1476/1504/1505 슬롯 호출 · 데이터 = ArcInner 헤더 16B 뒤 align 정렬(`(align-1)&-16 + 16`) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 26 | Entity(champ) | 0x4d8 | skill_effect.range | r | 1340 | 4 | OK |  |
| 27 | Entity(champ) | 0x4e0 | skill_effect.growth_range | r | 1340 | 4 | OK |  |
| 28 | Entity(champ) | 0x4f0 | skill_effect.target (CastingTarget) | r | %600 · 1438/1495 CastingTarget::check 첫 인자 | 4 | OK |  |
| 29 | Entity(champ) | 0x4f8 | skill_effect@tag (-1=None) | r | 1334 · %133 · 1432 게이트 | 4 | OK |  |
| 30 | Entity(champ) | 0x500 | skill2_effect.Some.0 (Effect) | r | 1335 level>2 일 때만, 아니면 @anon.19(None 상수) | 4 | OK |  |
| 31 | Entity(champ) | 0x510 | skill2_effect.range | r | 1341 (%139+16) · (C3 경고 사유) 절대 오프셋 gep 가 아니라 `select(level>N, champ+0x500\|0x538, @anon.19)` 결과 포인터에 +16/+24/+48 을 더하는 상대 gep 라 본문에 10진수 리터럴이 없다(m02.ll:66536~66552 · 66668~66672) | 4 | OK |  |
| 32 | Entity(champ) | 0x518 | skill2_effect.growth_range | r | 1341 (%139+24) · (C3 경고 사유) 절대 오프셋 gep 가 아니라 `select(level>N, champ+0x500\|0x538, @anon.19)` 결과 포인터에 +16/+24/+48 을 더하는 상대 gep 라 본문에 10진수 리터럴이 없다(m02.ll:66536~66552 · 66668~66672) | 4 | OK |  |
| 33 | Entity(champ) | 0x530 | skill2_effect@tag | r | 1335 (%139+48) · %142 · 배치 B 1592 게이트에 씀 · (C3 경고 사유) 절대 오프셋 gep 가 아니라 `select(level>N, champ+0x500\|0x538, @anon.19)` 결과 포인터에 +16/+24/+48 을 더하는 상대 gep 라 본문에 10진수 리터럴이 없다(m02.ll:66536~66552 · 66668~66672) | 4 | OK |  |
| 34 | Entity(champ) | 0x538 | ult_effect.Some.0 | r | 1336 level>4 일 때만 | 4 | OK |  |
| 35 | Entity(champ) | 0x568 | ult_effect@tag | r | 1336 (%145+48) · %148 · 배치 D 소비 · (C3 경고 사유) 절대 오프셋 gep 가 아니라 `select(level>N, champ+0x500\|0x538, @anon.19)` 결과 포인터에 +16/+24/+48 을 더하는 상대 gep 라 본문에 10진수 리터럴이 없다(m02.ll:66536~66552 · 66668~66672) | 4 | OK |  |
| 36 | Entity(champ) | 0x580 | skill.data_ptr (Arc<dyn Action> 앞 절반) | r | %601 · 1442/1483/1499 self 인자 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 37 | Entity(champ) | 0x588 | skill.vtable_ptr | r | %602 · 슬롯 +0xc8 / +0xc0 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 38 | Entity(any) | 0x5c0 | id | r | SmallActionAttack/Skill::new target 인자 · closure$0 id 비교 | 4 | OK |  |
| 39 | Entity(champ) | 0x5c8 | level | r | 1335 >2 · 1336 >4 · Effect::range (level-1) | 4 | OK |  |
| 40 | Entity(아군) | 0x628 | stat_cached.hp (최대 HP) | r | 1511 hp_ratio 분모 · 0 이면 div_by_zero 패닉 | 4 | OK |  |
| 41 | Entity(any) | 0x640 | stat_cached.move_speed | r | 1343 champ · 1356/1447 대상(RunAway 보정) | 4 | OK |  |
| 42 | Entity(any) | 0x660 | x | r | dist_sq | 4 | OK |  |
| 43 | Entity(any) | 0x668 | y | r | dist_sq | 4 | OK |  |
| 44 | Entity(any) | 0x670 | hp | r | 1396 타워 hp · 1511 아군 hp | 4 | OK |  |
| 45 | Entity(any) | 0x680 | radius | r | Entity::radius 인라인 | 4 | OK |  |
| 46 | Entity(타워) | 0x6a0 | block_target_tick | r | aux closure$4(1329): == 0 | 4 | OK |  |
| 47 | Entity(타워) | 0x6b9 | can_target (bool) | r | aux closure$4(1329) | 4 | OK |  |
| 48 | vtable dyn AbstractGame | 0x1f0 | get_entity_by_id (divtable) | r | 1413 · fn(game, id) -> Option<&Entity>(null=None) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 49 | vtable dyn Action(champ.skill) | 0xc8 | target_constraint (divtable 일치율 58% → 추정 · 인자 형태 (self,game.data,game.vt,caster,target)->bool 일치) | r | 1442/1499 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 50 | vtable dyn Action(champ.skill) | 0xc0 | can_activate (divtable 58% → 추정 · (self,game.data,game.vt,caster)->bool) | r | 1483 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 51 | vtable dyn EffectType(skill_effect.ty) | 0x68 | expected_move_on_hit (divtable 94%) | r | 1469 (self)->bool | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 52 | vtable dyn EffectType(skill_effect.ty) | 0x58 | expected_move_distance (94%) -> Option<(usize,u64)> sret 24B · 태그 +0 · 0=None(tcx sig · 기본 impl store 0) | r | 1469 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 53 | vtable dyn EffectType(skill_effect.ty) | 0x120 | can_move (94%) | r | 1476 (self)->bool | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 54 | vtable dyn EffectType(skill_effect.ty) | 0x40 | expected_heal (94%) -> i64 | r | 1504 (self,context,caster,&dyn target) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 55 | vtable dyn EffectType(skill_effect.ty) | 0x48 | expected_shield (94%) -> i64 | r | 1505 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 56 | Entity | 0x0 | team@tag (TeamType: 0 Player / 1 Neutral) | r | trunc i64→i1 로 bit0 검사. Neutral(1) 이면 is_visible_from 검사 생략(=보임 취급), Player(0) 면 +0x8 팀 인덱스로 visible_state 검사. 1531·1576·1594·1664·1687 (m02.ll:68524·68855·69445·69897·70346) | 4 | OK |  |
| 57 | Entity | 0x8 | team@Player.0 (팀 인덱스) | r | visible_state 배열 인덱스. <2 아니면 panic_bounds_check(m02.ll:68544·69272·69494·70055·70366) | 4 | OK |  |
| 58 | Entity | 0x38 | visible_state[team]@tag (VisibleState: 0 Visible) | r | gep 56 + team*24. ==0 이어야 '보임'. 상대 엔티티 x/e 에 대해 읽음(챔프 팀 기준). 1531·1576·1594·1664·1687 | 4 | OK |  |
| 59 | Entity | 0x68 | ty@tag (EntityType) | r | ==13 Champion 검사(EntityType::is_champion 인라인 entity.rs:1403). 1636 (m02.ll:69691~69692) | 4 | OK |  |
| 60 | Entity | 0x438 | stat_buff_cached.range | r | Effect::range(caster) 인라인(effect.rs:25~26) 의 가산항. 1530(대상 e 기준)·1575(챔프 기준, 루프 밖 호이스팅 %1210)·1686 | 4 | OK |  |
| 61 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | Entity::radius() 인라인(entity.rs:1509~1515): mult==0 → radius, 아니면 radius*(100+mult)/100 (udiv). 1530·1543·1575·1612·1686 (m02.ll:68473·68494·68596·68843·69550·70295·70316) | 4 | OK |  |
| 62 | Entity | 0x490 | attack_effect@Some.0 (&Effect 페이로드 시작) | r | target_atk. e 기준 1528(%999)·1684(%1877) / 챔프 기준 1574(%127, 배치 A 66524 산출) | 4 | OK |  |
| 63 | Entity | 0x4a0 | attack_effect.range | r | Effect::range 인라인. 1530·1575·1686 | 4 | OK |  |
| 64 | Entity | 0x4a8 | attack_effect.growth_range | r | growth_range*(level-1). 1530·1575·1686 | 4 | OK |  |
| 65 | Entity | 0x4c0 | attack_effect@tag (Option 니치 -1=None) | r | e.attack_effect 유무. ==-1 → continue. 1528(m02.ll:68393)·1684(70215). 챔프 것은 배치 A %130(66528) | 4 | OK |  |
| 66 | Entity | 0x4c8 | skill_effect@Some.0 (&Effect · Arc<dyn EffectType> ptr) | r | %134=champ+0x4c8. etc_buff(1526·1571) 호출용 Arc 데이터 포인터 및 range_adjust/aoe_heal/effect_buff_target 의 &Effect 인자 | 4 | OK |  |
| 67 | Entity | 0x4d0 | skill_effect.ty.vtable (dyn EffectType) | r | %606/%607. ArcInner 데이터 오프셋 = ((align-1)&-16)+16 (vtable+16 = align). 1526·1571 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 68 | Entity | 0x4f0 | skill_effect.target (CastingTarget) | r | %600(배치 A 67591 산출). CastingTarget::check(1565 자기대상) | 4 | OK |  |
| 69 | Entity | 0x4f8 | skill_effect@tag | r | 배치 A %133(66533) — 배치 B 에서는 직접 안 읽음(문맥상 skill_effect Some 확정) | 4 | OK |  |
| 70 | Entity | 0x500 | skill2_effect@Some.0 (&Effect) | r | %139 = level>2 ? champ+0x500 : @anon.19(None 상수) — Entity::skill2_effect 인라인(entity.rs:1692~1693, 배치 A 66536~66540). 배치 B 의 skill2 전 호출이 %139 사용 | 4 | OK |  |
| 71 | Entity | 0x508 | skill2_effect.ty.vtable | r | %1482/%1483 = %139+8. expected_move_on_hit(1629)·expected_move_distance(1629)·can_move(1636)·expected_heal(1660)·expected_shield(1661)·etc_buff(1682) ⚠C3: IR 은 Entity 절대 gep 이 아니라 %139(=champ+0x500 또는 None 상수) 기준 상대 gep(+8/+40/+48, m02.ll:66542·69410·69417)로 접근 — 절대 오프셋 1288/1320/1328 문자열은 본문에 없음 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 72 | Entity | 0x528 | skill2_effect.target (CastingTarget) | r | %1475 = %139+40. CastingTarget::check 1598·1651 ⚠C3: IR 은 Entity 절대 gep 이 아니라 %139(=champ+0x500 또는 None 상수) 기준 상대 gep(+8/+40/+48, m02.ll:66542·69410·69417)로 접근 — 절대 오프셋 1288/1320/1328 문자열은 본문에 없음 | 4 | OK |  |
| 73 | Entity | 0x530 | skill2_effect@tag | r | 배치 A %142(66544: ==-1 None). 배치 B 진입 %592 블록의 `br %142` 로만 소비(1592: None 이면 스킬2 절 전체 건너뜀 → %1452 배치 C 줄 1761) ⚠C3: IR 은 Entity 절대 gep 이 아니라 %139(=champ+0x500 또는 None 상수) 기준 상대 gep(+8/+40/+48, m02.ll:66542·69410·69417)로 접근 — 절대 오프셋 1288/1320/1328 문자열은 본문에 없음 | 4 | OK |  |
| 74 | Entity | 0x580 | skill (Box<dyn Action> 데이터 ptr) | r | %601. can_activate(1558·1586)·target_constraint(1565) | 4 | OK |  |
| 75 | Entity | 0x588 | skill.vtable (dyn Action) | r | %602. +0xc0 can_activate · +0xc8 target_constraint | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 76 | Entity | 0x590 | skill2 (Box<dyn Action>) | r | %1476 = select(level>2, 0x590, 0x5b0) — Entity::skill2_action 류 인라인(1593 진입 블록 m02.ll:69411). 데이터 %1477, vtable %1478 | 4 | OK |  |
| 77 | Entity | 0x5b0 | empty (Box<dyn Action>) — level<=2 일 때 skill2 대신 | r | 위와 같은 select | 4 | OK |  |
| 78 | Entity | 0x5c0 | id | r | SmallActionSkill::new/Skill2::new 의 target 인자. 1562(e)·1587(champ)·1647(e) (m02.ll:68669·69289·69713) | 4 | OK |  |
| 79 | Entity | 0x5c8 | level | r | Effect::range 의 growth 곱(level-1). 1530·1575(%135 배치 A)·1686 | 4 | OK |  |
| 80 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | hp_ratio 분모. ==0 이면 panic_const_div_by_zero. 1511(배치 A %949~%956)·1667(m02.ll:70134~70143) | 4 | OK |  |
| 81 | Entity | 0x640 | stat_cached.move_speed (적 e) | r | 1607: champ.move_speed.saturating_sub(e.move_speed) (llvm.usub.sat m02.ll:69537) | 4 | OK |  |
| 82 | Entity | 0x660 | x | r | Entity::distance_sq 인라인(entity.rs:2157~2158): \|dx\|²+\|dy\|² | 4 | OK |  |
| 83 | Entity | 0x668 | y | r | 동상 | 4 | OK |  |
| 84 | Entity | 0x670 | hp | r | hp_ratio = hp*100/stat_cached.hp. 1667 | 4 | OK |  |
| 85 | Entity | 0x680 | radius | r | Entity::radius() 인라인 가산항 | 4 | OK |  |
| 86 | BuffState(Option, effect_buff_target sret 288B) | 0x48 | duration@tag (Option 니치: -1=None) | r | has_buff = tag != -1. %51+72(배치 A 1507 %825) · %48+72(1570 m02.ll:68760~68761) · %42+72(1663 70185~70203 ; %1499) | 4 | OK |  |
| 87 | BuffState(Option, effect_buff_target sret 288B) | 0x88 | move_speed_mult (i32) | r | closure_env$12: `\|b\| b.move_speed_mult == 0` → is_non_movespeed_buff = buff.map_or(false, …). 1525(%627=%51+136)·1570(%48+136)·1681(%1503=%42+136) | 4 | OK |  |
| 88 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | %82(배치 A 66354). Action 호출의 game 인자(%82 → +0 dyn ptr, +8 vtable) | 4 | OK |  |
| 89 | OperationData | 0x8 | context (&GameContext) | r | %102(배치 A 66443). expected_heal/expected_shield/effect_buff_target/expected_damage_target 인자 | 4 | OK |  |
| 90 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame: +0 data ptr %788/%1161/%1522, +8 vtable %789/%1162/%1523) | r | Action::can_activate/target_constraint 의 2·3번째 인자 | 4 | OK |  |
| 91 | AbstractGameWithCache | 0xf0 | others[team] (bumpalo Vec<&Entity>; +0 ptr · +0x18 len · stride 32) | r | 1665: cache.others[%104] (%104 = 1 - player.info.team(+0x930, %75) — DI 변수명은 team, 배치 A 66450; 값은 상대 팀 인덱스). %1501 = gep {{ptr,ptr,i64},i64} %624, %104 (m02.ll:70066~70077) | 4 | OK |  |
| 92 | vtable(dyn EffectType) @anon.75300de3978c94cc946f88c448be65c0.1131(g02.ll:1149) | 0x40 | expected_heal(&self, &GameContext, &dyn AbstractEntity) -> usize | r | 1660 (m02.ll:69821~69823). 배치 A 1504 도 동일 슬롯 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 93 | vtable(dyn EffectType) | 0x48 | expected_shield(…) -> usize | r | 1661 (69839~69841) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 94 | vtable(dyn EffectType) | 0x58 | expected_move_distance(&self) -> Option<(usize,u64)> (sret 24B, +0 태그 0=None) | r | 1629 (69649~69657) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 95 | vtable(dyn EffectType) | 0x68 | expected_move_on_hit(&self) -> bool | r | 1629 (69629~69632) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 96 | vtable(dyn EffectType) | 0x90 | etc_buff(&self) -> bool | r | 1526(68375~68377)·1571(68775~68777)·1682(70197~70199) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 97 | vtable(dyn EffectType) | 0x120 | can_move(&self) -> bool | r | 1636 (69680~69682) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 98 | vtable(dyn Action) @anon.fa20fec81c56e69076f4627196dc604e.609(g04.ll) | 0xc0 | can_activate(&self, &dyn AbstractGame, &Entity caster) -> bool | r | 1558(68657~68659)·1586(68795~68797)·1643(69664~69666) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 99 | vtable(dyn Action) | 0xc8 | target_constraint(&self, &dyn AbstractGame, &Entity caster, &Entity target) -> bool | r | 1565(68741~68743)·1602(69509~69511)·1655(69802~69804) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 100 | vtable(dyn EffectType/Action 공통 머리) | 0x10 | align (ArcInner 데이터 오프셋 계산: ((align-1) & -16) + 16) | r | Arc<dyn EffectType>::deref 인라인(sync.rs:2445). 1526·1571·1629·1636·1660·1661·1682 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 101 | local %70 enemies: bumpalo Vec<(SmallAction, &Entity)> | 0x0 | ptr | r | +0x18 len(%596/%1471). 원소 32B: +0 SmallAction 태그(1606 에서 ==0 RunAway 비교) · +0x18 &Entity(x/e). 1529·1574·1593·1664·1685 루프 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 102 | local %72 candidates: bumpalo Vec<&Entity> | 0x0 | ptr | r | +0x18 len. 원소 8B. 1494(배치 A 진입)·1650 루프의 아군/대상 후보 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 103 | local %65 ret: bumpalo Vec<SmallActionPlay> | 0x10 | cap (%125) | r | push: len==cap → reserve_internal_or_panic(%65, len, 1, true) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 104 | local %65 ret | 0x18 | len (%126) | r | push 후 len+1 store | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 105 | Entity(champ %86) | 0x0 | team@tag | r | 1762/1799/1839 is_visible_from 인라인(entity.rs:1482→1135 player_team): 1=Neutral 이면 가시성 검사 생략 | 4 | OK |  |
| 106 | Entity(champ %86) | 0x8 | team@Player.0 | r | 팀 인덱스 → 대상 visible_state[team] 색인(<2 아니면 panic_bounds_check anon.26 = entity.rs:1483:7) | 4 | OK |  |
| 107 | Entity(champ %86) | 0x490 | attack_effect (Option<Effect>) | r | %127. range_adjust/expected_damage_target self. 1773/1807/1845 는 %130(=attack_effect.casting==-1 None, 배치 A 1333)으로 게이트 | 4 | OK |  |
| 108 | Entity(champ %86) | 0x4c8 | skill_effect (Option<Effect>) | r | %134. 1783/1817/1853 는 %133(None 게이트, 배치 A 1334) | 4 | OK |  |
| 109 | Entity(champ %86) | 0x4f0 | skill_effect.target (CastingTarget) | r | %1464 = 0x4c8+0x28. CastingTarget::check(champ, e) | 4 | OK |  |
| 110 | Entity(champ %86) | 0x500 | skill2_effect (Option<Effect>) — %139 = level>2 ? &skill2_effect : &None(anon.19) | r | 배치 A 1335 Entity::skill2_effect()(entity.rs:1692) 결과. 1789/1826/1861 는 %142(None 게이트) | 4 | OK |  |
| 111 | Effect(skill2 %139) | 0x0 | ty (Arc<dyn EffectType> 데이터/vtable) | r | 1726 expected_shield(vtable+0x48) · 1728 etc_buff(vtable+0x90) — ArcInner 데이터 오프셋 = (align-1)&-16 +16 계산 | 4 | OK |  |
| 112 | Effect(skill2 %139) | 0x28 | target (CastingTarget) | r | %1475/%1466 = %139+40. 1721 check(champ,champ) · 1789/1826/1861 check(champ,e) | 4 | OK |  |
| 113 | Entity(champ %86) | 0x590 | skill2 (Box<dyn Action>) / 0x5b0 empty | r | %1477 = champ + select(level>2, 0x590, 0x5b0) (배치 B 1469 %1476) = Entity::skill2_action(). vtable 슬롯 0xc0 can_activate · 0xc8 target_constraint · 0x68 as_any | 4 | OK |  |
| 114 | Entity(champ %86) | 0x660 | x | r | %1480/%1459. dist_sq 계산(\|dx\|²+\|dy\|²) | 4 | OK |  |
| 115 | Entity(champ %86) | 0x668 | y | r | %1481/%1460 | 4 | OK |  |
| 116 | Entity(대상 e) | 0x38 | visible_state[team]@tag | r | +0x38+0x18*team == 0(Visible) 이어야 통과 (1762·1799·1839·closure#22 1743) | 4 | OK |  |
| 117 | Entity(대상 e) | 0x68 | ty@tag | r | 1765 is_epic: (tag-5)<2 ⟹ Epic(5)\|Serpen(6) · 1803 is_jungle: tag==4 | 4 | OK |  |
| 118 | Entity(대상 e) | 0x98 | ty@Jungle.info.camp_type.0 (usize 팀) | r | 1803: == 적팀 인덱스 %104(=1-내팀, 배치 A 1319) | 4 | OK |  |
| 119 | Entity(대상 e) | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius()(entity.rs:1509) 인라인: mult==0 ? radius : radius*(mult+100)/100 | 4 | OK |  |
| 120 | Entity(대상 e) | 0x5c0 | id | r | SmallAction*::new(data, e.id) 의 target | 4 | OK |  |
| 121 | Entity(대상 e) | 0x628 | stat_cached.hp | r | 1768 hp_ratio 분모(umax(·,1)) | 4 | OK |  |
| 122 | Entity(대상 e) | 0x660 | x | r |  | 4 | OK |  |
| 123 | Entity(대상 e) | 0x668 | y | r |  | 4 | OK |  |
| 124 | Entity(대상 e) | 0x670 | hp | r | 1768 hp_ratio 분자 · 1809/1819/1828 `hp > expected_dmg` 면 제외 | 4 | OK |  |
| 125 | Entity(대상 e) | 0x680 | radius | r | Entity::radius() 인라인 | 4 | OK |  |
| 126 | Entity(대상 e) | 0x6a0 | block_target_tick | r | 1762/1799 can_target 인라인: ==0 필요 | 4 | OK |  |
| 127 | Entity(대상 e) | 0x6b9 | can_target (bool) | r | 1762/1799 can_target 인라인(entity.rs:1477) | 4 | OK |  |
| 128 | AbstractGameWithCache(%82) | 0x0 | game (&dyn AbstractGame: +0 데이터, +8 vtable) | r | %1666/%1667·%2041/%2042·%2644/%2645 — can_activate/target_constraint/has_enemy_champion_target_or_action_threat/can_target 에 (data,vtable) 쌍으로 전달 | 4 | OK |  |
| 129 | AbstractGameWithCache(%82) | 0xd0 | jungles (bumpalo Vec<&Entity>: +0 ptr, +0x18 len) | r | 1761 루프·1798 루프 둘 다 이 벡터를 처음부터 순회 | 4 | OK |  |
| 130 | AbstractGameWithCache(%82) | 0xf0 | others[적팀] (bumpalo Vec<&Entity> 32B stride, +0x18 len) | r | 1838 루프. getelementptr {{ptr,ptr,i64},i64} 인덱스 %104=적팀 | 4 | OK |  |
| 131 | Option<BuffState>(%39, effect_buff_target sret 288B) | 0x48 | duration@tag (BuffType) — -1 이면 None | r | 1727 closure#20 map_or(false): tag!=-1 && move_speed_mult==0 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 132 | Option<BuffState>(%39) | 0x88 | move_speed_mult (i32) | r | ==0 이어야 is_non_movespeed_buff | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 133 | bumpalo::Vec<SmallActionPlay>(%65 로컬) | 0x10 | cap | r | %125. push 전 len==cap 이면 reserve_internal_or_panic(self,len,1,true) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 134 | bumpalo::Vec<SmallActionPlay>(%65 로컬) | 0x18 | len | r | %126 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 135 | bumpalo::Vec<(SmallAction,&Entity)>(%70 로컬, 배치 A 1318) | 0x0 | ptr / 0x18 len | r | 1741 any 이터레이터 원소 32B · 원소+0x18 = &Entity(closure#22 3235/3416 등) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 136 | Entity(champ %86) — closure#22 안 | 0x438 | stat_buff_cached.range | r | aux 3196: base_range = attack.range(+0x10) + growth_range(+0x18)*(level(+0x5c8)-1) + buff.range + champ.radius() | 4 | OK |  |
| 137 | Effect(attack %127) — closure#22 안 | 0x10 | range / 0x18 growth_range | r | aux 3188~3191 | 4 | OK |  |
| 138 | Entity(champ %86 / e / x) | 0x0 | team@tag | r | TeamType 판별자(8B · Direct · 0 Player/1 Neutral). `trunc i64→i1` 로 Player 여부 판정(is_visible_from 인라인 · entity.rs:1136 player_team). champ 가 Neutral 이면 가시성 검사 생략 | 4 | OK |  |
| 139 | Entity(champ) | 0x8 | team@Player.0 | r | 팀 인덱스(usize · <2 bounds check · %1458). visible_state 첨자 | 4 | OK |  |
| 140 | Entity(e / x) | 0x38 | visible_state[team]@tag | r | [VisibleState;2] · 원소 24B(+0x38 + 24*team · gepS) · 태그 0 = Visible 이어야 통과(entity.rs:1482~1483 is_visible_from → data.rs:122 is_visible) | 4 | OK |  |
| 141 | Entity(e) | 0x68 | ty@tag | r | EntityType 판별자 · L1924 `== 13`(Champion) | 4 | OK |  |
| 142 | Entity(champ / e) | 0x438 | stat_buff_cached.range | r | 사거리 합산항(effect.rs:26 Effect::range 인라인) · champ %2918 · e %3215 | 4 | OK |  |
| 143 | Entity(champ / e / x) | 0x470 | stat_buff_cached.radius_mult | r | i32 · 0 이면 radius 그대로, 아니면 radius*(100+mult)/100 (entity.rs:1511~1515 Entity::radius 인라인) | 4 | OK |  |
| 144 | Entity(champ / e) | 0x4a0 | attack_effect@Some.0.range | r | L1966/L2020 클로저 attack_range 항(%2941 · %3212) | 4 | OK |  |
| 145 | Entity(champ / e) | 0x4a8 | attack_effect@Some.0.growth_range | r | ×(level-1) | 4 | OK |  |
| 146 | Entity(champ / e) | 0x4c0 | attack_effect@tag(casting@tag) | r | i32 == -1 이면 attack_effect None(니치). champ = %130(배치 A 계산 재사용) · e = %3203(L1965) | 4 | OK |  |
| 147 | Entity(champ) | 0x538 | ult_effect@Some.0.ty (Arc<dyn EffectType> 팻포인터 +0x0 ptr · +0x8 vtable) | r | level>4 일 때만(배치 A %143=`level ugt 4` · entity.rs:1701) 실체, 아니면 상수 @anon…19(빈 Effect) — `%145 = select`. vtable+0x10 align 으로 ArcInner 데이터 오프셋 산출(sync.rs:2445) | 4 | OK |  |
| 148 | Entity(champ) | 0x548 | ult_effect@Some.0.range | r | %2916(=%145+0x10 · m02.ll:72538). C3 경고 사유: 본문 gep 는 Entity 절대 오프셋이 아니라 select 된 Effect 포인터 %145 기준 +16 으로 접혀 있다 | 4 | OK |  |
| 149 | Entity(champ) | 0x550 | ult_effect@Some.0.growth_range | r | %2917(=%145+0x18 · 72539) ×(level-1)(%2919). C3 경고 사유 동일(%145 기준 +24) | 4 | OK |  |
| 150 | Entity(champ) | 0x560 | ult_effect@Some.0.target@tag | r | %2915(=%145+0x28 · 72537) · CastingTarget::check 의 &self. C3 경고 사유 동일(%145 기준 +40) | 4 | OK |  |
| 151 | Entity(champ) | 0x568 | ult_effect@tag(casting@tag) | r | 배치 A 의 %148(`== -1` = 궁 없음/레벨 미달 · m02.ll:66550~66552 = %145+48) 을 L1870 에서 분기 조건으로 재사용. C3 경고 사유 동일(%145 기준 +48) | 4 | OK |  |
| 152 | Entity(champ) | 0x5a0 | ult (Box<dyn Action> 팻포인터) | r | level>4 면 +0x5a0(1440) 아니면 +0x5b0(1456) empty — `select %143`(entity.rs:1677 Entity::ult 인라인 · L1871). Action vtable 슬롯 +0x68 as_any · +0xc0 can_activate · +0xc8 target_constraint | 4 | OK |  |
| 153 | Entity(champ) | 0x5b0 | empty (Box<dyn Action>) | r | 위 select 의 else 쪽 | 4 | OK |  |
| 154 | Entity(e / champ) | 0x5c0 | id | r | SmallActionSkill2/Ult::new 의 target 인자(L1865 e · L1935 e · L2002 e · L2033 champ) | 4 | OK |  |
| 155 | Entity(champ / e) | 0x5c8 | level | r | champ %136(배치 A 로드 재사용) · e %3214 — growth_range*(level-1) | 4 | OK |  |
| 156 | Entity(champ / e / x) | 0x640 | stat_cached.move_speed | r | champ %239(배치 A 로드) → L1894 추격 이속 · L1963/L2018 engage_extra(×60). e: L1894 saturating_sub 피감수 · L1963 engage_extra | 4 | OK |  |
| 157 | Entity(champ / e / x / focus) | 0x660 | x | r | distance_sq(entity.rs:2158 → utils.rs:7~9 abs_diff²+abs_diff²) | 4 | OK |  |
| 158 | Entity(champ / e / x / focus) | 0x668 | y | r | 위와 같음 | 4 | OK |  |
| 159 | Entity(champ / e / x) | 0x680 | radius | r | Entity::radius 인라인(0x470 참조) | 4 | OK |  |
| 160 | OperationData(data %3) | 0x0 | cache | r | &AbstractGameWithCache(%82) | 4 | OK |  |
| 161 | OperationData(data %3) | 0x8 | context | r | &GameContext(%102) — effect_buff_target · expected_damage_target · expected_on_attack_damage 인자 | 4 | OK |  |
| 162 | AbstractGameWithCache(%82) | 0x0 | game(&dyn AbstractGame).data | r | %2959 등 — vtable 슬롯 +0x1f0 get_entity_by_id · target_constraint/can_activate 의 game 인자 | 4 | OK |  |
| 163 | AbstractGameWithCache(%82) | 0x8 | game(&dyn AbstractGame).vtable | r | %2632 = %82+8 | 4 | OK |  |
| 164 | GamblerUltAction(다운캐스트 결과) | 0x10 | charm_duration | r | L1872 `!= 0` → is_v16_gambler_cc_ult (dbg 는 DW_OP_not 1개 · 극성은 select 방향으로 확정) | 4 | OK |  |
| 165 | Option<BuffState>(buff · 스택 %14/%11 · effect_buff_target sret 288B) | 0x48 | duration@tag (니치) | r | i32 == -1(0xFFFFFFFF) 이면 None — `!= -1` = buff.is_some() (L1954 · L2014) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 166 | (SmallAction<&Entity>, &Entity) 튜플(near_enemies_with_action 원소 32B) | 0x0 | .0 SmallAction@tag | r | L1894 `== 0`(RunAway) — 적이 도망 중이면 추격 이속 상대화 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 167 | (SmallAction<&Entity>, &Entity) 튜플 | 0x18 | .1 &Entity | r | e(L1874 루프) / x(L1966·L2020 any 클로저) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 168 | bumpalo Vec near_enemies_with_action(%70) / near_allies(%72) | 0x0 | buf.ptr | r | 루프 시작 포인터 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 169 | bumpalo Vec near_enemies_with_action(%70) / near_allies(%72) | 0x18 | len | r | 루프 끝 = ptr + len*32(%70) / len*8(%72) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 170 | sret Vec<SmallActionPlay>(%0) | 0x0 | buf.ptr/buf.a/buf.cap/len (32B 전체) | w | 배치 D 범위(battle.rs:2038 · m02.ll:72349) — 배치 A 에는 %0 쓰기 없음. 여기 적는 것은 sret 표면 선언(배치 A 가 시그니처 담당) | 4 | 확인불가(tcx 사전에 타입 없음) | candidates(%65) memcpy 32B |
| 171 | 지역 candidates(%65) bumpalo Vec | 0x0 -> ptr | buf.ptr | w | 1331 초기화 m02.ll:66518 | 4 | 확인불가(tcx 사전에 타입 없음) | 8(dangling) → reserve_internal_or_panic 후 bump 할당 포인터 |
| 172 | 지역 candidates(%65) | 0x8 | buf.a | w | m02.ll:66520 | 4 | 확인불가(tcx 사전에 타입 없음) | data.context.pool |
| 173 | 지역 candidates(%65) | 0x10 | buf.cap | w | memset 16B m02.ll:66526 · push 때 len==cap 이면 reserve_internal_or_panic(vec,len,1,true) | 4 | 확인불가(tcx 사전에 타입 없음) | 0 → reserve 가 갱신 |
| 174 | 지역 candidates(%65) | 0x18 | len | w | 배치 A push 5곳: 1372·1391·1401·1428(Attack 15) · 1491(Skill 16). 원소 쓰기 = ptr[len] 에 184B memcpy(live = 0x0~0x11 + 0xb1) | 4 | 확인불가(tcx 사전에 타입 없음) | +1 per push |
| 175 | 지역 version 슬롯(%73) | 0x0 | version 복사 | w | m02.ll:66296 · &version 을 closure$2 환경에 넣기 위한 spill | 4 | 확인불가(tcx 사전에 타입 없음) | %1 |
| 176 | local %65 ret (→ sret %0, 배치 D memcpy m02.ll:72349) | 0x0 | ret[len] (184B 원소) · +0x18 len | w | 1562 (m02.ll:68671~68722): 아군/대상 후보 e 에 스킬1. reserve 실패 unwind 시 %50 drop_glue | 4 | 확인불가(tcx 사전에 타입 없음) | SmallActionPlay::Skill(SmallActionSkill{start_tick=cache.game.tick(), target=e.id, is_act=false}) — 태그 16 store i8 at +0xb1 |
| 177 | local %65 ret | 0x0 | ret[len] · len | w | 1587 (69291~69343): 자기 대상 스킬1 \| (배치 B) 1647 (69715~69766): 적 e 에 스킬2 | 4 | 확인불가(tcx 사전에 타입 없음) | SmallActionPlay::Skill(target=champ.id) 태그 16 |
| 178 | sret 인자 없음(&mut 참조 인자 0) | 0x0 | (부작용 표면 = 위 Vec push 뿐) | w | define 줄 참조 인자 %2·%3 은 readonly · %0 은 writeonly sret. 배치 B 는 게임 상태를 쓰지 않는다 | 4 | 확인불가(tcx 사전에 타입 없음) | - |
| 179 | bumpalo::Vec<SmallActionPlay>(%65 로컬 · 배치 D 가 sret 으로 이관) | len*184 + 0x00..0x11 / +0xb1 | push SmallActionPlay::Skill2 (태그 17) | w | 각 사이트: new → memcpy 24B → store i8 17 @+177 → (len==cap ? reserve) → memcpy 184B → len+=1 (%126 store) | 4 | 확인불가(tcx 사전에 타입 없음) | SmallActionSkill2::new(data, e.id) · 사이트 5곳: 1718(m02.ll:70495~70546 · 적 챔피언 e) · 1754(70736~70788 · 자기 자신 champ.id) · 1792(71252~71304 · 정글 e) · 1831(71784~71836 · 적정글캠프 e) · 1862→1865(배치 D · others e) |
| 180 | bumpalo::Vec<SmallActionPlay>(%65) | len*184 + 0x00..0x11 / +0xb1 | push SmallActionPlay::Skill (태그 16) | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | SmallActionSkill::new(data, e.id) · 사이트 3곳: 1786(71108~71160 · 정글) · 1822(71628~71680 · 적정글캠프) · 1857(72152~72204 · others) |
| 181 | bumpalo::Vec<SmallActionPlay>(%65) | len*184 + 0x00..0x11 / +0xb1 | push SmallActionPlay::Attack (태그 15) | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | SmallActionAttack::new(data, e.id) · 사이트 3곳: 1779(70964~71016 · 정글) · 1813(71472~71524 · 적정글캠프) · 1849(72012~72064 · others) |
| 182 | bumpalo::Vec<SmallActionPlay>(%65) | 0x18 | len | w | reserve 경로에선 cap/ptr 도 reserve_internal_or_panic 내부에서 갱신(콜리 소관) | 4 | 확인불가(tcx 사전에 타입 없음) | len+1 (push 마다) |
| 183 | 이터레이터 상태 %38(alloca 16B) | 0x0/0x8 | slice::Iter<(SmallAction,&Entity)> {ptr,end} | w | closure#22 any 호출용 로컬 — 외부 부작용 아님 | 4 | 확인불가(tcx 사전에 타입 없음) | 1741: %70 벡터 [ptr, ptr+len*32) |
| 184 | (sret %0) Vec<SmallActionPlay> | 0x0 | buf.ptr·buf.a·buf.cap·len (32B 통째) | w | 배치 D 의 반환 지점은 %2850(L2038 · m02.ll:72349) 하나. 도달 경로: L1870(궁 없음/can_ult 거짓) · L2005(자기 대상 조건 실패) · L2032(skip_self_buff 또는 can_activate 거짓) · L2033(push 후) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 지역 candidates(%65) 32B memcpy |
| 185 | candidates(지역 bumpalo Vec %65) | 0x18 | len | w | push 4곳: L1865(m02.ll:72340) · L1935(72985) · L2002(73556) · L2033(out-of-line push · 73850) | 4 | 확인불가(tcx 사전에 타입 없음) | len+1 |
| 186 | candidates(지역 %65) | 0x0 -> buf.ptr[len] (184B 원소) | 원소 memcpy + 태그 | w | cap==len 이면 먼저 reserve_internal_or_panic(self, used_cap, 1, true) | 4 | 확인불가(tcx 사전에 타입 없음) | Skill2: 페이로드 24B(SmallActionSkill2::new(data, e.id)) + 태그 17 @+0xb1 (L1865) / Ult: 페이로드 24B(SmallActionUlt::new(data, target.id)) + 태그 18 @+0xb1 (L1935 e · L2002 e(아군) · L2033 champ) |
| 187 | (참고) 스택 alloca 태그 바이트 | 0xb1 | SmallActionPlay 태그(177) | w | %2637(=%19+177) · %2924(=%16+177) · %2943(=%13+177) · %3577(=%10+177) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 17 / 18 |

**`consts` 상수 58건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 1306 | 태그 | player_champion 1차원 길이(팀 수) — bounds check. 같은 값이 1347/1418/1434/1508 visible_state bounds, 1414 Tower 태그, 1397 remain_attack 임계(>2)에도 등장 | 4 |
| 1 | 30 | 1312 | 산출값 | max_tick(kb_hold 파생) — 후보 사거리에 더하는 이동 예산 틱: max_dist = range + move_speed*30. KitingBack 이고 (version<=1 이거나 base_defense_focus 거짓) 이면 0 | 4 |
| 2 | 1 | 1313 | 태그 | version > 1 게이트(KitingBack 일 때만 base_defense_focus 호출) · 1319 `1-team` 적 팀 · 1412 EntityType::Minion 태그 | 4 |
| 3 | 3 | 1312 | 태그 | BattleSubPlanGoal::KitingBack 메모리태그(tcxdict --enum · Direct) · 1390/1414 EntityType::Nexus 태그 3 · 1467 switch case 3 · 1414 TowerType TwinA=3 (`add -3; icmp ult 2` = {3,4}) · (경고 사유) 본문의 `shl 3`(×8 stride) 과 무관 — 이 항목은 태그값 등재 | 3 |
| 4 | -1 | 1333 | 센티널 | Option<Effect> 니치 None 태그(i32 · attack/skill/skill2/ult_effect@tag == -1 ⟹ None) · 1507 buff+0x48 != -1 ⟹ has_buff · aux closure$2: small_actions 태그 -1 = None | 4 |
| 5 | 100 | 1339 | 계수 | Entity::radius(entity.rs:1511~1515) 퍼센트: radius*(100+radius_mult)/100 (udiv) | 4 |
| 6 | 15 | 1372 | 센티널 | SmallActionPlay::Attack 메모리태그(니치 · idx12+3) — enum+0xb1 에 store i8 15 (1372·1391·1401·1428) | 4 |
| 7 | 16 | 1491 | 태그 | SmallActionPlay::Skill 메모리태그(idx13+3) — store i8 16 | 4 |
| 8 | 0 | 1355 | 태그 | SmallAction::RunAway 태그(blackboard.rs:82 Direct 0) — 대상의 현재 행동이 도주면 move_speed 를 saturating_sub 로 보정(1355/1446) · 1467 switch case 0 = BattleSubPlanGoal::Trace · 1347 visible_state 태그 0 = 가시 | 4 |
| 9 | 13 | 1476 | 태그 | EntityType::Champion 태그 — 대상이 챔피언일 때만 skill_damage/is_dash_worth 검사 | 4 |
| 10 | 4 | 1467 | 태그 | BattleSubPlanGoal::RunAway 태그(switch case 4 → 1469 경로) · 1336 level>4 ⟹ ult 보유 · TowerType TwinB=4 | 4 |
| 11 | 5 | 1467 | 태그 | BattleSubPlanGoal::Assassin 태그(case 5 → 1483 직행) · 1316/1319 슬라이스 길이 5 · aux closure$1 idx<5 · (경고 사유) 본문의 `shl 5`(×32 = (SmallAction,&Entity) 원소 stride, m02.ll:68048) 과 무관 — 이 항목은 태그값/길이 등재 | 4 |
| 12 | 14400000001 | 1508 | 임계 | 120000^2 + 1 — 아군 근처 적(챔프 또는 others) 판정: dist_sq < 14400000001 ⟺ 거리 ≤ 120000 (=3.75셀·셀 32000) · 1508 적 챔프 / 1509 others 동일 임계 | 4 |
| 13 | 22500000000 | 1316 | 미상 | 150000^2 — near_allies 필터(aux closure$0 m02.ll:77019~): 자기 자신 제외 && dist_sq < 150000^2(strict · +1 없음). 등록 근거 = aux 범위 | 4 |
| 14 | 40 | 1316 | 미상 | 슬라이스 끝 = 시작 + 5*8 (stride 8 · Option<&Entity> 5개) — 임계 아님(수집 범위 표기용) | 4 |
| 15 | 480 | 1306 | 미상 | = 0x1e0 player_champion 오프셋(reads 참조) — gep 상수 그대로. 임계 아님 | 4 |
| 16 | 79 | 1513 | 임계 | hp_ratio(대상 e 의 hp*100/최대hp) > 79 이면 힐 스킬은 낭비 → AOE 힐이 저체력 아군을 덮지 않는 한 continue. 1669 에도 동일(스킬2) | 4 |
| 17 | 14400000001 | 1664 | 임계 | 120000² + 1 — `icmp ult dist_sq, 14400000001` = dist ≤ 120000(=3.75셀). 대상 e 근방에 적(enemies 또는 cache.others[상대팀]) 존재 판정(enemy_near). 배치 A 1508 도 동일 값 | 4 |
| 18 | 100 | 1667 | 계수 | 백분율 스케일: hp_ratio = hp*100/max_hp(1667) · radius*(100+radius_mult)/100(Entity::radius 인라인, 1530·1543·1575·1612·1686) | 4 |
| 19 | 16 | 1562 | 센티널 | SmallActionPlay 메모리 태그 16 = Skill (tcxdict --enum: 니치 niche_start=3, idx13). store i8 16 at +0xb1 (1562·1587) | 3 |
| 20 | 17 | 1647 | 태그 | SmallActionPlay 태그 17 = Skill2. store i8 17 at +0xb1 | 4 |
| 21 | 13 | 1636 | 태그 | EntityType 태그 13 = Champion (e.ty@+0x68 == 13 → is_champion) | 4 |
| 22 | 3 | 1627 | 태그 | BattleSubPlanGoal 태그 3 = KitingBack (switch %4). 4 = RunAway 와 같은 arm(1629 대시 금지 사전검사) — ⚠shl 피연산자 아님(switch case 값; `shl … 5` 는 Vec 원소 32B stride 와 무관한 별개 명령) | 4 |
| 23 | 4 | 1627 | 태그 | BattleSubPlanGoal 태그 4 = RunAway | 4 |
| 24 | 5 | 1627 | 태그 | BattleSubPlanGoal 태그 5 = Assassin — 0(Trace) 과 같은 arm(1636 can_move/is_dash_worth 검사 생략 → 곧장 1643) — ⚠shl 피연산자 아님(switch case 값; 본문의 `shl nuw nsw i64 %len, 5` 는 (SmallAction,&Entity) 32B stride 계산이며 판정 상수가 아님) | 4 |
| 25 | 0 | 1606 | 태그 | 다의: (a) SmallAction 태그 0 = RunAway (`__arg1_discr = 0`, 1606) (b) VisibleState 태그 0 = Visible (c) TeamType 태그 0 = Player (d) BattleSubPlanGoal 0 = Trace (1627) (e) expected_move_distance 태그 0 = None (1629) (f) BuffState.move_speed_mult == 0 (1525·1570·1681) (g) expected_heal/shield == 0 → has_* 부정 | 4 |
| 26 | -1 | 1528 | 센티널 | Option 니치 None: attack_effect@tag(+0x4c0) == -1 (1528·1684) / BuffState duration@tag(+0x48) == -1 (has_buff 부정, 1570·1663) | 4 |
| 27 | 2 | 1594 | 임계 | visible_state 배열 길이 2 — 팀 인덱스 <2 bounds check(1531·1576·1594·1664·1687). ⚠판정 상수 아님(가드) | 4 |
| 28 | -16 | 1526 | 계수 | ArcInner 데이터 오프셋 정렬 마스크 `(align-1) & -16` (Arc<dyn EffectType>::deref 인라인). 판정 상수 아님 | 4 |
| 29 | 100 | 1699 | 계수 | Entity::radius() 인라인(entity.rs:1515): radius*(radius_mult+100)/100 — 대상마다 반복(1699·1774·1784·1790·1810·1820·1829·1846·1854·1862·closure#22) | 4 |
| 30 | 17 | 1718 | 센티널 | SmallActionPlay 메모리 태그 17 = Skill2 (tcxdict --enum: idx14 · 니치 start 3) · store i8 @+177(0xb1) | 3 |
| 31 | 16 | 1786 | 태그 | SmallActionPlay 태그 16 = Skill | 4 |
| 32 | 15 | 1779 | 태그 | SmallActionPlay 태그 15 = Attack. ⚠같은 리터럴 15 가 1798 `mul %239, 15`(move_speed×15틱)에도 쓰임 — 별개 의미 | 4 |
| 33 | 177 | 1718 | 태그 | SmallActionPlay 태그 바이트 오프셋 0xb1(gep i64 177) — 레이아웃 상수(임계 아님) | 4 |
| 34 | 168406848281932906149591046147716593956 | 1733 | 태그 | TypeId::of::<PrisonerSkill2Action>() 16B 상수 — skill2_action.as_any().downcast_ref::<PrisonerSkill2Action>() (any.rs:229 인라인). 뒤이은 1734 호출 self 가 PrisonerSkill2Action 이라 타입 확정 | 4 |
| 35 | -5 | 1765 | 미상 | EntityType::is_epic()(entity.rs:1369) 인라인: (ty.tag - 5) <u 2 ⟹ tag∈{5 Epic, 6 Serpen} | 4 |
| 36 | 2 | 1765 | 임계 | 위 is_epic 의 `icmp ult, 2`. (1762/1799/1839 의 `team < 2` 는 visible_state[2] 배열 경계) | 4 |
| 37 | 20 | 1769 | 임계 | ★에픽 HP% 임계: hp_ratio = hp*100/max(max_hp,1) 가 20 초과면 그 에픽/세르펜은 후보 제외(20 이하만 공격/스킬 후보) | 4 |
| 38 | 1 | 1768 | 임계 | umax(stat_cached.hp, 1) — 0 나눗셈 보호(core cmp::max 인라인 ;L1039). push 의 reserve additional=1 도 리터럴 1 | 4 |
| 39 | 4 | 1803 | 태그 | EntityType 태그 4 = Jungle — is_jungle(team)(entity.rs:1377) 인라인: tag==4 && camp_type.0 == 적팀 | 4 |
| 40 | 15 | 1798 | 계수 | ★적 정글캠프 루프 전용 이동 틱 = 15 (max_dist = move_speed*15 + atk_base + …). 정글/others 루프는 max_tick(%97: 30 또는 0, 배치 A) 사용 | 4 |
| 41 | 0 | 1726 | 태그 | has_shield = expected_shield(...) != 0 (IR: icmp eq 0 + DI DW_OP_not). 1762 등 block_target_tick==0 · visible tag==0 · 1727 move_speed_mult==0 도 0 비교 | 4 |
| 42 | -1 | 1727 | 센티널 | Option<BuffState> None 니치 = duration(BuffType) tag -1 (effect_buff_target m15.ll:26907 store i32 -1 로 교차확인) | 4 |
| 43 | 36000000 | 1888 | 산출값 | 6000² — focus_radius(제곱비교): sub_goal 의 focus 엔티티에서 6000 안에 있는 적만 궁 대상(비-갬블러 궁 또는 갬블러 궁 charm_duration==0). `%2910 = phi [%2908, 36000000]` | 4 |
| 44 | 8100000000 | 1888 | 산출값 | 90000² — 갬블러(GamblerUltAction) 궁이면서 charm_duration != 0(is_v16_gambler_cc_ult) 일 때의 focus_radius. select %2907(charm==0) ? 36000000 : 8100000000 | 4 |
| 45 | 129962296932191015333459514643667840978 | 1871 | 태그 | TypeId(GamblerUltAction) 상수(i128) — ult_action.as_any().downcast_ref::<GamblerUltAction>() 인라인의 type_id 비교(m05.ll:54021 등 4곳도 같은 값 · 그쪽 dbg 변수명 gambler_ult) | 4 |
| 46 | 60 | 1963 | 계수 | engage_extra = move_speed*60 (틱 60 = 1초 이동분 · L1963 e · L1964 caster_extra %2940 · L2018 champ). 사거리 합산에 더해 「1초 안에 붙을 수 있는」 교전권 판정 | 4 |
| 47 | 100 | 1900 | 계수 | Entity::radius 인라인(entity.rs:1515): radius*(100+radius_mult)/100. 판정 임계가 아니라 퍼센트 환산(L1900·1966~1970·1983·2021 반복) | 4 |
| 48 | 13 | 1924 | 태그 | EntityType 메모리태그 13 = Champion — 대시형 궁(can_move) 의 대상이 챔피언일 때만 is_dash_worth 검사 | 4 |
| 49 | 0 | 1894 | 태그 | SmallAction<&Entity> 태그 0 = RunAway(적이 도망 중) — L1894 · 또한 L1872 charm_duration==0 · L1955/L2015 expected_on_attack_damage()==0(gate_applies) · L1917 expected_move_distance 첫 8B==0 · L1875 visible_state 태그 0=Visible 비교값 | 4 |
| 50 | -1 | 1965 | 센티널 | Option<Effect> 니치(casting@tag=-1 → None): champ.attack_effect(%130) · e.attack_effect(L1965) · champ.ult_effect(%148, L1870) / Option<BuffState> 니치(duration@tag=-1 → None · L1954/L2014) — 전부 「없음」 판정 | 4 |
| 51 | 17 | 1865 | 태그 | SmallActionPlay 메모리태그 17 = Skill2 (store i8 17 @ 원소+0xb1) | 4 |
| 52 | 18 | 1935 | 태그 | SmallActionPlay 메모리태그 18 = Ult (L1935 · L2002 · L2033) | 4 |
| 53 | 3 | 1915 | 태그 | BattleSubPlanGoal 태그 3 = KitingBack — L1915 switch: 3·4 → L1917 이동형 궁 배제 검사. (qcspec 경고 사유: 본문의 `shl … 3` 은 Vec<&Entity> stride 8 접힘(drop 경로)이지 이 상수와 무관 — 여기 3 은 switch case 값) | 4 |
| 54 | 4 | 1887 | 태그 | BattleSubPlanGoal 태그 4 = RunAway — L1887 switch: 4·7 은 focus 없음(거리 제한 없이 통과) · L1915: 3·4 → L1917 | 4 |
| 55 | 5 | 1915 | 태그 | BattleSubPlanGoal 태그 5 = Assassin — L1915 switch: 0(Trace)·5 → L1924 대시 검사 생략, 곧장 L1931 can_activate. (qcspec 경고 사유: 본문의 `shl … 5` 는 (SmallAction,&Entity) stride 32 접힘(near_enemies len*32)이지 이 상수와 무관 — 여기 5 는 switch case 값) | 4 |
| 56 | 7 | 1887 | 태그 | BattleSubPlanGoal 태그 7 = End — L1887 switch 에서 focus 없음 | 4 |
| 57 | 2 | 1875 | 임계 | visible_state[team] 의 bounds check 상한(팀 2개) — 판정값 아님(panic_bounds_check 인자) | 4 |

**`knobs` 조정점 21건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 이동 예산 틱 max_tick | battle.rs:1312 (phi %97 · m02.ll:66419/66423) | 30 | 올리면 사거리 밖(멀리) 적·타워·미니언까지 Attack/Skill 후보에 들어간다(내리면 사거리 안 대상만). KitingBack+kb_hold 거짓이면 0 으로 고정 | 4 | 기존 |
| 1 | 타워 공격 잔여타 임계 | battle.rs:1397 | 2 | 비-넥서스 타워는 (hp / 평타 기대피해) ≤ 2 일 때만 후보. 올리면 체력 많은 타워도 때리기 시작 | 4 | 기존 |
| 2 | 아군 근처 적 판정 거리 | battle.rs:1508~1509 | 14400000001 | 아군 반경 120000 안에 가시 적 챔프/others 가 있으면 has_near_enemy — 배치 B 의 힐/실드 분기 입력. 올리면 더 넓게 '위험'으로 본다 | 4 | 기존 |
| 3 | near_allies 반경 | battle.rs:1316 (aux m02.ll:77019~) | 22500000000 | champ 반경 150000 안 아군만 Skill 대상 후보. 올리면 먼 아군에게도 스킬 후보 생성 | 4 | 기존 |
| 4 | skill2/ult 해금 레벨 | battle.rs:1335/1336 | 2 | level>2 ⟹ skill2, level>4 ⟹ ult 를 후보 계산에 포함(Entity 메서드 인라인 — 게임 규칙과 결합돼 있어 단독 조정 위험) | 4 | 기존 |
| 5 | 힐 낭비 컷 HP% 임계 | battle.rs:1513 (스킬1) · 1669 (스킬2) | 79 | 올리면(예: 89) 더 높은 체력의 아군에게도 힐 스킬 후보가 남는다(AOE 저체력 커버 검사 없이). 내리면 힐이 더 낮은 체력에서만 후보가 된다. 부등호 `hp_ratio > 79` 라 80% 부터 컷 | 4 | 기존 |
| 6 | 실드/버프 판단용 '적 근접' 반경 (enemy_near) | battle.rs:1664~1665 (스킬2) · 배치 A 1508 (스킬1) | 14400000001 | 120000²+1. 올리면 더 먼 적이 있어도 실드 스킬을 아군에게 걸어 준다(낭비↑) · 내리면 적이 바짝 붙었을 때만. 값은 제곱+1 이므로 반경 R 로 바꾸려면 R²+1 | 4 | 기존 |
| 7 | 버프 스킬 사용 조건 — 대상이 때릴 수 있는 적 존재 | battle.rs:1529~1531 · 1574~1576 · 1685~1687 | attack_range 공식(가시성 && 평타 사거리) | 이 조건을 없애면 적이 사거리 밖이어도 (이속 아닌) 버프/etc 버프 스킬을 미리 건다. 사거리에 여유(예: +radius) 를 더하면 조금 더 이르게 버프 | 4 | 기존 |
| 8 | KitingBack/RunAway 시 대시 스킬2 금지 | battle.rs:1627~1629 | switch %4 ∈ {3,4} | arm 을 비우면 후퇴 중에도 이동형(expected_move_on_hit / expected_move_distance Some) 스킬2 를 적에게 쓴다 | 4 | 기존 |
| 9 | Trace/Assassin 시 대시 가치 검사 생략 | battle.rs:1627 (0·5 arm) → 1643 | switch %4 ∈ {0,5} | 이 arm 을 기본 arm 으로 돌리면 추적/암살 중에도 챔피언 대상 이동 스킬2 에 is_dash_worth 검사가 붙어 후보가 줄어든다 | 4 | 기존 |
| 10 | RunAway 적에 대한 접근 속도 보정 | battle.rs:1606~1607 | champ.move_speed − e.stat_cached.move_speed (saturating) | 도망치는 적(SmallAction::RunAway, block_move 아님)에겐 상대 속도로 max_dist 를 계산해 스킬2 후보가 줄어든다. 보정을 빼면 도망치는 적에게도 스킬2 후보가 남음 | 4 | 기존 |
| 11 | 스킬2 해금 레벨 | entity.rs:1692~1693 인라인 (배치 A %137 `level > 2`) | level>2 | 배치 B 의 스킬2 절 전체가 %139/%1476 select 에 걸려 있음. 배치 A 소관 | 4 | 기존 |
| 12 | 에픽/세르펜 막타 후보 HP% 임계 | battle.rs:1769 (m02.ll:70874 `icmp ugt %2179, 20`) | 20 | 올리면 더 높은 HP 의 에픽/세르펜도 평타·스킬·스킬2 후보에 오른다(막타 시도 조기화). 내리면 더 낮을 때만 시도 | 4 | 기존 |
| 13 | 적 정글캠프 스틸 루프의 도달 틱 | battle.rs:1798 (m02.ll:70805 `mul %239, 15`) | 15 | 올리면 더 먼 적 정글캠프(한 방 처치 가능한 것)에 평타 후보를 낸다. 스킬/스킬2 는 이동 항 없음이라 무관 | 4 | 기존 |
| 14 | 정글/others 루프 도달 틱(max_tick) | battle.rs:1312~1361 (배치 A %97: 30/0) | 30 | 배치 A 소관 — [A][C][E] 의 평타·[A][E] 의 스킬 도달 사거리에 move_speed 배로 들어간다 | 4 | 기존 |
| 15 | 적 정글캠프 한 방 처치 조건 | battle.rs:1809/1819/1828 (`icmp ugt hp, expected_dmg`) | hp <= expected_damage_target | `<=` 를 완화(예: hp <= dmg*k)하면 못 죽이는 캠프에도 후보를 냄 — 상수 아님(구조 노브) | 4 | 기존 |
| 16 | focus_radius(기본) — sub_goal focus 로부터 궁 대상 허용 거리 | battle.rs:1888 | 36000000 | 6000² 제곱비교. 올리면 focus(추적/보호/카이팅 대상)에서 더 먼 적에게도 궁을 후보로 올린다 · 내리면 focus 근처 적에게만 | 4 | 기존 |
| 17 | focus_radius(갬블러 CC 궁) | battle.rs:1888 | 8100000000 | 90000². is_v16_gambler_cc_ult(GamblerUltAction && charm_duration!=0) 일 때 사실상 거리 제한 해제. 내리면 갬블러 궁도 focus 근처로 제한 | 4 | 기존 |
| 18 | engage_extra 배수(교전권 = 1초 이동분) | battle.rs:1963 · 1964 · 2018 | 60 | 아군/자기 대상 버프 궁의 「근처에 적이 있어야」 조건에서 사거리에 더하는 이속×틱. 올리면 더 멀리 있는 적도 교전 중으로 간주해 버프 궁이 더 자주 후보에 오른다 | 4 | 기존 |
| 19 | max_tick(추격 허용 틱) — 궁 대상 max_dist 에 곱하는 틱 | battle.rs:1904 · 1987 (IR 은 배치 A L1361 %97 로 CSE) | 30 | KitingBack 이면서 (version<=1 또는 !base_defense_focus) 일 땐 0(제자리 사거리만). 올리면 더 먼 적/아군에게도 궁 후보 등록(이동 포함 사거리 확대) | 4 | 기존 |
| 20 | 대시형 궁 가치 검사 대상 = 챔피언만 | battle.rs:1924 | 13 | EntityType Champion 태그. 바꾸면 미니언/정글 대상 대시 궁도 is_dash_worth 를 거친다(현재는 비챔피언엔 검사 없이 통과) | 4 | 기존 |

<details><summary>`callees` 피호출자 87건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | aoe_heal_covers_low_ally | game_ai::aoe_heal_covers_low_ally | pub | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\buff_value.rs:543 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | as_any | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any | pub | fn(&game_ai::AgentVerHamster) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-ai\src\lib.rs:425 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 2 | as_any | game_core::Action::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\setting\action.rs:12 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 3 | as_any | game_core::AiAgent::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\simulation\ai_interface.rs:499 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 4 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | base_defense_focus | game_ai::plan_legacy::old::base_defense_focus | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:200 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | block_move | game_core::Entity::block_move | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1497 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_activate | game_core::Action::can_activate | pub | fn(&Self/#0, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity) -> bool | game-core\src\setting\action.rs:30 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 8 | can_activate | <game_core::HunterSkill2Action as game_core::Action>::can_activate | pub | fn(&game_core::HunterSkill2Action, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity) -> bool | game-core\src\setting\champion\hunter.rs:427 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 9 | can_activate | <game_core::NecromancerUltAction as game_core::Action>::can_activate | pub | fn(&game_core::NecromancerUltAction, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity) -> bool | game-core\src\setting\champion\necromancer.rs:417 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 10 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | can_move | game_core::Entity::can_move | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1489 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 12 | can_move | game_core::Champion::can_move | pub | fn(&game_core::Champion) -> bool | game-core\src\simulation\entity\champion.rs:48 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 13 | can_move | game_core::EffectType::can_move | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:358 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 14 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 20 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 21 | effect_buff_target | game_ai::effect_buff_target | pub | fn(usize, &game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-ai\src\fight_check.rs:390 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | etc_buff | game_core::EffectType::etc_buff | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:304 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 23 | etc_buff | <game_core::BanishEffect as game_core::EffectType>::etc_buff | pub | fn(&game_core::BanishEffect) -> bool | game-core\src\simulation\effect\type\banish.rs:84 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 24 | etc_buff | <game_core::CombineEffect as game_core::EffectType>::etc_buff | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:54 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 25 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | expected_heal | game_ai::expected_heal | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:266 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 27 | expected_heal | game_core::EffectType::expected_heal | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type.rs:283 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 28 | expected_heal | <game_core::HealEffect as game_core::EffectType>::expected_heal | pub | fn(&game_core::HealEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type\heal.rs:186 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 29 | expected_move_distance | game_core::EffectType::expected_move_distance | pub | fn(&Self/#0) -> std::option::Option<(usize, u64)> | game-core\src\simulation\effect\type.rs:289 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 30 | expected_move_distance | <game_core::RushEffect as game_core::EffectType>::expected_move_distance | pub | fn(&game_core::RushEffect) -> std::option::Option<(usize, u64)> | game-core\src\simulation\effect\type\rush.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 31 | expected_move_distance | <game_core::MoveToEffect as game_core::EffectType>::expected_move_distance | pub | fn(&game_core::MoveToEffect) -> std::option::Option<(usize, u64)> | game-core\src\simulation\effect\type\move_to.rs:54 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 32 | expected_move_on_hit | game_core::EffectType::expected_move_on_hit | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:293 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 33 | expected_move_on_hit | <game_core::CombineEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:96 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 34 | expected_move_on_hit | <game_core::DelayedEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::DelayedEffect) -> bool | game-core\src\simulation\effect\type\delayed.rs:45 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 35 | expected_on_attack_damage | game_core::EffectType::expected_on_attack_damage | pub | fn(&Self/#0, &game_core::GameContext, &game_core::Entity) -> usize | game-core\src\simulation\effect\type.rs:319 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 36 | expected_on_attack_damage | game_core::EffectBuff::expected_on_attack_damage | pub | fn(&Self/#0, &game_core::GameContext, &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:159 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 37 | expected_on_attack_damage | <game_core::CombineEffect as game_core::EffectType>::expected_on_attack_damage | pub | fn(&game_core::CombineEffect, &game_core::GameContext, &game_core::Entity) -> usize | game-core\src\simulation\effect\type\combine.rs:80 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 38 | expected_shield | game_ai::expected_shield | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:284 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 39 | expected_shield | game_core::EffectType::expected_shield | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type.rs:285 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 40 | expected_shield | <game_core::RangeEffect as game_core::EffectType>::expected_shield | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type\range_effect.rs:90 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 41 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 42 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 43 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 44 | has_enemy_champion_target_or_action_threat | game_core::PrisonerSkill2Action::has_enemy_champion_target_or_action_threat | pub | fn(&game_core::PrisonerSkill2Action, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity) -> bool | game-core\src\setting\champion\prisoner.rs:241 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 45 | is_champion | game_core::EntityType::is_champion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 46 | is_dash_worth | game_ai::is_dash_worth | pub | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, usize) -> bool | game-ai\src\utils.rs:98 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 47 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 48 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 49 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 50 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 51 | is_epic | game_core::EntityType::is_epic | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1369 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 52 | is_jungle | game_core::EntityType::is_jungle | pub | fn(&game_core::EntityType, usize) -> bool | game-core\src\simulation\entity.rs:1377 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 53 | is_twin_tower | game_core::EntityType::is_twin_tower | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1312 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 54 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 55 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 57 | max_hp | game_core::PlayerAiContext::<'a, 'b, 'r>::max_hp | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> std::option::Option<usize> | game-core\src\mod_ai.rs:608 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 58 | new | game_ai::SmallActionUlt::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionUlt | game-ai\src\small_action\cast.rs:247 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 59 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 60 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 61 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 62 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 63 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 64 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 65 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 66 | skill | game_ai::skill | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:199 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 67 | skill | game_core::Entity::skill | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1664 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 68 | skill | game_core::ChampionInfo::skill | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1085 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 69 | skill2 | game_ai::skill2 | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:239 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 70 | skill2 | game_core::Entity::skill2 | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1668 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 71 | skill2 | game_core::ChampionInfo::skill2 | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1086 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 72 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 73 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 74 | sub_goal | game_ai::plan_legacy::old::BattlePlan::sub_goal | pub | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\battle.rs:178 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 75 | sub_goal | game_ai::plan_legacy::old::SinglePlanBattle::sub_goal | pub | fn(&game_ai::plan_legacy::old::SinglePlanBattle) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\single_battle.rs:27 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 76 | sub_goal | game_ai::plan_legacy::old::DeathMatchBattle::sub_goal | pub | fn(&game_ai::plan_legacy::old::DeathMatchBattle) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\death_battle.rs:114 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 77 | target_constraint | game_core::Action::target_constraint | pub | fn(&Self/#0, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity, &game_core::Entity) -> bool | game-core\src\setting\action.rs:34 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 78 | target_constraint | <game_core::HunterSkill2Action as game_core::Action>::target_constraint | pub | fn(&game_core::HunterSkill2Action, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity, &game_core::Entity) -> bool | game-core\src\setting\champion\hunter.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 79 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 80 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 81 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 82 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 83 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 84 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 85 | type_id | game_view::worker::WorkerMessage::type_id | pub | fn(&game_view::worker::WorkerMessage) -> &str | game-view\src\logic\server\worker.rs:445 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 86 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 36개**: `atk_base`, `base_range`, `block_target_tick`, `buff`, `candidates`, `champ`, `charm_duration`, `continue`, `dist_sq`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `eff_range`, `empty`, `enemies`, `enemy_near`, `enemy_towers`, `enumerate`, `focus_radius`, `jungles`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `llvm.umax.i64`, `map_or`, `max_tick`, `move_speed`, `near_allies`, `near_enemies_with_action`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`, `reach`, `reserve_internal_or_panic`, `skill2_action`, `skill2_base`, `skill_base`, `target`, `try_fold`, `ult_none`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 6곳** (m02.ll:29209, m02.ll:29225, m02.ll:29230, m02.ll:31654, m02.ll:31799, m02.ll:35473) · **형제 0개** 

**`open` 35건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | (배치 A) dyn Action 슬롯 +0xc8/+0xc0 의 이름(target_constraint/can_activate)은 divtable 일치율 58% 기반 추정 — 인자 형태(5/4 인자·bool 반환)는 IR 과 일치하나 이름 확정은 런타임 vtable 덤프 또는 _gcbc 정적 vtable 재대조 필요(적용 범위: divtable 정적 매칭) | 3 |  |
| 1 | 미탐색 | (배치 A) 1469 `expected_move_distance().is_some()` 의 태그 값 의미(0=None)는 tcx 시그니처 Option<(usize,u64)> 와 기본 impl `store i64 0` 로 추정 — 니치 없는 Direct 태그라면 0=None 이 맞으나 tcxdict --enum 으로 Option<(usize,u64)> 를 직접 조회하지 않았다 | 3 |  |
| 2 | 미탐색 | (배치 A) closure$1 이 읽는 Blackboard.small_actions 의 의미(적 챔프의 '현재 소행동')는 이름·타입(SmallAction)에서 온 해석 — 누가 쓰는지(작성자)는 미탐색 | 4 |  |
| 3 | 미탐색 | (배치 A) 1409 두 번째 `can_attack()` 재호출이 첫 호출과 다른 값을 낼 수 있는지(부작용 여부)는 can_attack 본문 미확인(_gcbc) — IR 상 두 게이트는 독립 호출 | 4 |  |
| 4 | 미탐색 | (배치 A) 승계 메모의 last_stand_flags LocalKey::with · v3_beyond_enemy_line · estimate_damage_to 는 이 define 에 없다(grep 0). 형제 death_battle(m15) 또는 다른 버전 잔재로 추정 — 적용 범위: m02.ll:66196~74096 본문 | 4 |  |
| 5 | 미탐색 | (배치 A) exe 0xcd05f0 의 실제 인자 배치(argscan 스택 인자)와 IR 6인자(sret,version,player,data,tag,focus) 대응은 미확인(IR 만 읽음) — sweep 편입 전 argscan.py 로 대조 필요 | 4 |  |
| 6 | 미탐색 | (배치 A) is_visible_from 인라인에서 champ.team 태그 하위비트가 1 인 케이스(Team 비-Player variant)의 실제 의미는 Team 열거형을 조회하지 않아 미확정 — IR 상 그 경로는 무조건 '가시' | 4 |  |
| 7 | 미탐색 | (배치 A) 1316 filter_map 의 iter_champions closure 본체(Option<&Entity> → &Entity 로 추정)는 m01.ll:41528~ 안에 인라인돼 개별 확인 안 함 | 4 |  |
| 8 | 미탐색 | (배치 A) constants 의 40/480 은 stride·오프셋이라 규격상 등재 대상이 아니나 C1 검증 겸 남겼다 — 병합 시 제거 가능 | 4 |  |
| 9 | 미탐색 | (배치 B) 배치 A 산출값의 정확한 구성은 재확인하지 않음(계약만 인용): %948(1508 enemy_near — phi true/false 5경로) · %629(=%208 skill_base.1 + move_speed*max_tick) · %237(skill2_base.1) · %97(max_tick) · %102(ctx) · %72/%70 Vec 의 채움 규칙(어느 엔티티가 candidates/enemies 에 들어가는가) | 4 |  |
| 10 | 표기 불가 | (배치 B) 1606: `*a == SmallAction::RunAway`(PartialEq derive, DI __self_discr/__arg1_discr) 와 `matches!(a, RunAway)` 는 외연이 같아 표기 불가 — 동작은 태그==0 비교로 확정 | 4 |  |
| 11 | 미탐색 | (배치 B) 1662(및 배치 A 1506): effect_buff_target 의 caster·target 두 인자가 모두 champ(%86) 이고 루프 변수 e 가 아님 — IR 사실. 의도(버프 정의만 보면 대상 무관인지)는 effect_buff_target 본문(fight_check.rs:390, m15.ll:26898)을 안 읽어 미확인 | 4 |  |
| 12 | 미탐색 | (배치 B) 콜리 본문 미독(계약만): Effect::range_adjust(effect.rs:29 → usize) · CastingTarget::check(type.rs:227 → bool) · Effect::expected_damage_target(effect.rs:91 → usize) · aoe_heal_covers_low_ally(buff_value.rs:543, (usize,&Effect,&OperationData,&PlayerState,&Entity)->bool) · is_dash_worth(utils.rs:98, (&OperationData,&PlayerState,&Entity,&Entity,usize)->bool) · effect_buff_target(fight_check.rs:390 → Option<BuffState> 288B) · Entity::can_skill2(entity.rs:1721) · Entity::block_move(1497) · SmallActionSkill::new(cast.rs:119: start_tick=cache.game.tick()(vtable+0x28) · target · is_act=false — m07.ll:7645 확인) | 4 |  |
| 13 | 미탐색 | (배치 B) 1574 루프의 4변종(%1233/%1278/%1326/%1376)은 컴파일러의 루프 불변 호이스팅(champ.team 태그 · champ.radius_mult==0) — 소스는 단일 클로저(1575~1576)로 판단. 4변종 본문이 문자 단위 동형인 것은 확인 | 4 |  |
| 14 | 미탐색 | (배치 B) Entity::radius() 인라인의 `radius*(100+radius_mult)/100` 은 udiv(무부호) 이고 radius_mult 는 i32 sext — 음수 mult 시 (100+mult) 가 0 이하일 때의 동작은 소스 미확인(0 이면 udiv 0 → 반지름 0, 음수면 wrap) | 4 |  |
| 15 | 미탐색 | (배치 B) vtable 슬롯 이름은 정적 vtable 전역(EffectType: g02.ll:1149 DokkaebiUltExplosionEffect 판 · Action: g04.ll anon.609 TargetAttackAction 판, 일치율 58%)의 순서로 확정 — 런타임 구현체는 알 수 없음(같은 트레이트라 슬롯 순서는 동일) | 4 |  |
| 16 | 미탐색 | (배치 B) reach.txt(version=2·gamemode=0): 이 함수 사장 호출부 0 · 접힌 분기 0 — 배치 B 범위에 NA 봉인 대상 없음 | 4 |  |
| 17 | 표기 불가 | (배치 C) 1731 has_shield 분기의 소스 표기: IR 은 %2062(shield==0)→1740, 아니면→1732 downcast 이고 downcast 실패도 1740 으로 합류한다. `if has_shield && let Some(pa)=… {…} else if …` 형태로 읽었지만 정확한 소스 문형(if-let 체인 vs 중첩 if)은 column 부재로 표기 불가 — 동작은 확정 | 4 |  |
| 18 | 표기 불가 | (배치 C) 1729 `is_non_movespeed_buff \|\| is_etc_buff` 의 좌우 순서: IR 은 `or i1` 로 접혀 있고 etc_buff 호출(1728)이 항상 실행되므로 단락 평가 순서는 표기 불가(동작엔 무관) | 4 |  |
| 19 | 미탐색 | (배치 C) EffectType vtable 슬롯 이름(0x48 expected_shield · 0x90 etc_buff)과 Action vtable(0xc0 can_activate · 0xc8 target_constraint · 0x68 as_any)은 divtable 로 game_core 정적 vtable(일치율 94%/58%)에서 읽은 것 — 런타임에 꽂히는 구현체는 챔피언별로 달라 본 명세 범위 밖 | 3 |  |
| 20 | 미탐색 | (배치 C) PrisonerSkill2Action::has_enemy_champion_target_or_action_threat(&self(64B), game, &Entity) -> bool: game_core g13.ll:98656 define 존재 확인만, 내부 미독해(자식 명세 없음 · 시그니처만 기록) | 4 |  |
| 21 | 미탐색 | (배치 C) 1839 OperationData::can_target(&self, game: &dyn, player: &PlayerState, e: &Entity) -> bool 내부 미독해(game_core, 시그니처만) | 4 |  |
| 22 | 미탐색 | (배치 C) Effect::range_adjust / expected_damage_target / CastingTarget::check / Entity::can_attack·can_skill·can_skill2 내부는 game_core 경계 — 시그니처·반환 의미만: range_adjust(&Effect, caster, target)->i64 사거리 보정치, expected_damage_target(&Effect, ctx, caster as &dyn, target)->usize 기대 데미지, check(&CastingTarget, caster, target)->bool 대상 종류 허용 | 4 |  |
| 23 | 미탐색 | (배치 C) nearby(%72)/enemy_acts(%70)/max_tick(%97)/atk·skill·skill2_base 의 정확한 정의는 배치 A(1312~1343)·배치 B(1650 루프 필터) 소관 — 여기선 closure#0(1316: 자기 제외·150000² 미만)과 closure#1/#2(1320/1321: 적팀 player_champion 슬롯) 심만 확인 | 4 |  |
| 24 | 미탐색 | (배치 C) [C]/[D] 루프에서 스킬·스킬2 사거리에 이동 항(move_speed*tick)이 빠지고 [A]/[E] 에는 들어가는 이유 — 의도인지 누락인지 IR 로는 판정 불가(관측 사실만 기록) | 4 |  |
| 25 | 미탐색 | (배치 C) 1761 루프 헤드 블록 1452 의 preds 에 배치 B 블록 592/1468/1445… 가 있어 [A][B] 전체가 건너뛰어지는 조건은 배치 B(1592 게이트) 소관 | 4 |  |
| 26 | 미탐색 | (배치 C) SmallActionPlay 원소 184B 중 +0x11..+0xb1 / +0xb2..+0xb8 는 alloca 잔여(미기록) — sweep 대조 시 이 구간은 비교 제외 대상(추정: 이 배치가 push 하는 3 variant 의 페이로드가 전부 24B struct 라서) | 5 |  |
| 27 | 미탐색 | (배치 D) L1917 expected_move_distance(EffectType vtable+0x58) 의 sret 24B 타입 — 첫 8B(i64)==0 이면 통과만 확인. Option<(u64,u64)> 인지 (usize,…) 인지 tcxdict 로 미확인(vtable 슬롯은 divtable EffectType 일치율 94% 기준) | 3 |  |
| 28 | 미탐색 | (배치 D) Action vtable 슬롯 이름(+0x68 as_any · +0xc0 can_activate · +0xc8 target_constraint)은 divtable `Action` 일치율 58% 의 두 정적 vtable(TargetProjectileAction/TargetAttackAction)에서 읽은 것 — 런타임 구현체는 불명이나 슬롯↔이름은 두 vtable 이 일치 | 3 |  |
| 29 | 미탐색 | (배치 D) L1954/L2014 `is_etc_buff \|\| buff.is_some()` 의 소스 순서(컬럼 부재) — IR 은 `or i1`(비단락) 이라 순서 무관 | 4 |  |
| 30 | 표기 불가 | (배치 D) L1889 비교가 소스에서 `<` 인지 `>=` 의 부정인지(표기 불가) — IR `icmp ult dist_sq, focus_radius` 참이면 통과로 동작 확정 | 4 |  |
| 31 | 미탐색 | (배치 D) L1949/L2009 effect_buff_target 에 아군 루프에서도 caster·target 이 둘 다 champ 인 이유(설계 의도) — IR 사실만 기록 | 4 |  |
| 32 | 미탐색 | (배치 D) 블록 %2808(m02.ll:72247~72282) 은 루트 L1862 로 시작해 블록 단위론 배치 C 이지만 L1863(1줄)·L1864(21줄) IR 이 그 안에 있어 여기 logic 에 포함 — mergespec 후 C 와 중복 서술 가능(내용은 동일해야 함) | 4 |  |
| 33 | 미탐색 | (배치 D) reach.txt(version=2·gamemode=0): 사장 블록 0 · 배치 D 범위에 NA 봉인 대상 없음 | 4 |  |
| 34 | 미탐색 | (배치 D) L1875 is_visible_from 의 정확한 소스 표기(메서드명은 DIScope 로 확정 · 인자 순서 (e, champ) 는 인라인 형태에서 추정 — champ.team 을 읽고 e.visible_state 를 첨자) | 5 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 B) 1543 의 max_dist 에 챔피언 자신의 radius 가 포함되는지는 %208(skill_base.1, 배치 A 1340) 의 구성에 달려 있어 배치 B 범위로는 미확정 | 4 | 사실 서술 |
| 1 | (배치 C) 1714/1753 `can_activate` 및 1721 `target_constraint` 의 의미(무엇을 검사하는지)는 구현체(챔피언별 Action)마다 다름 — 시그니처 (self, game: &dyn AbstractGame(data, vtable 816B), caster: &Entity[, target: &Entity]) -> bool 만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

