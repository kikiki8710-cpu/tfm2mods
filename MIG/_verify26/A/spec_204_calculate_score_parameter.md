---

### `204` calculate_score_parameter — 선수 1명 관점의 ScoreParameter(5384B) 를 통째로 계산: 근처 미니언·아군·적(행동 포함)·타워 목록을 뽑고, 적의 현재 행동(평타/스킬/궁)과 비행 중 투사체를 대조해 나·아군·적 각각의 applyed/risk 피해·CC 를 누적한 뒤(배치 A~C), 타워·미니언·에픽 위험과 격자 좌표를 채워(배치 C~D) sret 으로 반환한다. TLS 접점 없음(순수 계산).

| 항목 | 값 |
|---|---|
| id | `score_parameter__calculate_score_parameter` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parameter` |
| 소스 | `game-ai\src\score_parameter.rs:1461` |
| IR | `m07.ll` 37862~46714행 |
| 경로·가시성 | `game_ai::calculate_score_parameter` · **pub** |
| 계층 | 기타 |
| exe | `d8eff0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::ScoreParameter
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[204]/sig/tls/<키>`)**

- `name`: 없음 — 이 함수 자체의 TLS 접점 0
- `role`: 작성자/소비자 아님. 지시문 승계: score_parameter_cached(r15 · TLS 200엔트리 5차원 캐시)의 미스 경로에서 호출되는 순수 계산 함수
- `key`: 해당 없음(캐시 키는 score_parameter_cached 명세 몫)
- `layout`: 해당 없음
- `invalidation`: 해당 없음
- `call_conditions`: 근거: 본문 37862~46714 전체에서 `llvm.threadlocal.address`·`call_once`·`LocalKey`·`thread_local` 문자열 0건, 참조하는 @anon 상수는 .11(Entity 용 dyn vtable 88B)·.31(정적 Option<Effect> None)·.153~.170(panic Location) 뿐이며 `constant ptr @<KEY…call_once>` 형 fn-포인터 상수 참조 0건(grep 실측). 다른 배치가 TLS 접점을 발견하면 그 배치가 정정.

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) parameter | ScoreParameter (5384B, sret([5384 x i8])) | IR 속성 = `dead_on_unwind noalias writable writeonly captures(none) dereferenceable(5384)`. 본문은 로컬 alloca %47(5384B)에 조립하고 함수 끝(L2417 · 배치 D · m07.ll:46075)에서 5384B memcpy 로 sret 에 복사한다. 살아있는 바이트는 `return` 절 참조. \| (배치 B) define 줄 속성: `dead_on_unwind noalias writable writeonly sret([5384 x i8]) align 8 captures(none) dereferenceable(5384)`. 본체는 지역 alloca %47(5384B)에 조립한 뒤 마지막에 sret 으로 memcpy(배치 D 범위). 내 범위의 쓰기는 전부 %47 과 그 bumpalo Vec 힙 원소에 대한 것 \| (배치 C) (배치 C) 이 범위가 채우는 필드 = player.risk_damage(0x998) · player.risk_possible_tower(0x9b0) · near_enemies[i].risk_possible(Vec<PossibleGain> · elem+0x18) push · near_enemies[i].applyed_damage(elem+0x70) · near_enemies[i].risk_damage(elem+0x80) — writes 전수 참조 \| (배치 D) (배치 D) 함수 전체에서 단 한 번, 2417 `llvm.memcpy(%0 ← %47, 5384)`(m07.ll:46075)로만 기록된다. 본문은 로컬 `parameter`(%47 alloca 5384B)에 쌓고 마지막에 통째로 복사 — 즉 sret 의 live/패딩은 로컬 %47 의 상태 그대로다 | 4 |
| 1 | 1 | version | usize | IR 속성 없음(i64 noundef). 배치 A 범위에서는 분기 없음 — parameter.version(+0x14f8)에 그대로 저장(L1463)하고 precompute_champion_powers 의 첫 인자로 3회 전달(L1501·L1547·L1578)할 뿐. \| (배치 B) 내 범위(1744~1962)에서 사용 0회 — 버전 분기 없음 \| (배치 C) (배치 C) 이 범위에서 분기 없음(%1 참조 0건) \| (배치 D) (배치 D) 2183~2418 에서 분기·산술 사용 0건. 2338/2341 의 minion_wave_risk 콜리 첫 인자(version)는 `i64 poison` 으로 전달(콜리가 `_version` 미사용이라 ArgumentPromotion 이 poison 으로 바꿈) | 4 |
| 2 | 2 | rnd | &mut StdRng (320B) | IR 속성 = `readnone captures(none) dereferenceable(320)` — 함수 전체(37862~46714)에서 %2 참조 0건. gen_range 호출 사이트 0개 · 순서 없음. 형식상 &mut 이지만 쓰기 표면 없음. \| (배치 B) define 줄 속성 `readnone captures(none)` — 전 함수(37862~46714)에서 gen_range 호출 0회(grep gen_range/LocalKey 0건). 즉 rnd 스트림을 소비하지 않는다 \| (배치 C) (배치 C) 미사용 \| (배치 D) (배치 D) gen_range 호출 사이트 0건(내 범위·전체 모두). 캐시 미스 경로 재현 시 rnd 스트림 소비 없음 | 4 |
| 3 | 3 | player | &PlayerState (2528B) | IR 속성 = `readonly captures(address, read_provenance)`. 읽는 필드: info.team(+0x930) · info.position@tag(+0x9c0, i32→usize). 클로저 env 에도 포인터로 실려 s0_0/s3_0/s4_0 에서 같은 두 필드를 다시 읽는다. \| (배치 B) define 줄 속성 `readonly captures(address, read_provenance)`. 내 범위에서 직접 읽기 0회(배치 A 줄 1462 에서 info.team/pos 로 내 챔피언 %59 를 뽑아 둔 것을 재사용) \| (배치 C) (배치 C) info.team(0x930 · %49 · iter_minions 팀 인자 · 배치 A L1462 에서 로드) 만 간접 사용 \| (배치 D) (배치 D) 직접 읽기 없음. 배치 A 가 읽어둔 %49 = player.info.team(+0x930) 을 iter_minions(2193/2240)·others 인덱스(2265)에 씀 | 4 |
| 4 | 4 | data | &OperationData (24B) | IR 속성 = `readonly captures(address, read_provenance)`. cache(+0x0)·context(+0x8)·blackboard(+0x10) 세 포인터를 모두 읽는다. context.pool(+0x0 of GameContext) 이 bumpalo 할당자로 near_* Vec 들의 `a` 필드에 들어간다. \| (배치 B) define 줄 속성 `readonly`. 내 범위는 배치 A 가 로드한 %55(=data.cache) · %64(=data.context) · %177/%179(=cache.game dyn AbstractGame fat ptr, cache+0/+8) 를 재사용 \| (배치 C) (배치 C) data.cache(+0x0 → AbstractGameWithCache · game 팻포인터 +0x0/+0x8) · data.context(+0x8 → GameContext 64B · setting +0x8 · tutorial +0x38) · data.blackboard(+0x10 → [Blackboard;2]) 사용 \| (배치 D) (배치 D) 배치 A 가 로드한 %55 = data.cache(+0x0, &AbstractGameWithCache 8840B)·%64 = data.context(+0x8, &GameContext 64B)·%177/%179 = cache.game 팻포인터(data/vtable) 를 사용. 2338/2341 콜리에는 `data` 자체(%4)를 넘김 | 4 |
| 5 | 5 | _debug | &mut DebugFrameData (224B) | IR 속성 = `readnone captures(none) dereferenceable(224)` — 함수 전체에서 %5 참조 0건. 미사용. \| (배치 B) define 줄 속성 `readnone captures(none)` — 전 함수에서 미사용(dbg_value 만) \| (배치 C) (배치 C) 미사용 \| (배치 D) 미사용(함수 전체) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// score_parameter.rs:0~1740 (배치 A)
// 승계: sret 5384B = ScoreParameter(tcxdict --deep) · 로컬 %47 에 조립 → 배치 D L2417 에서 sret 으로 memcpy.
// 콜리 계약(자식/경계 — 내부 미독): iter_minions(cache,team) -> Chain<Chain<Copied<Iter<&Entity>>,Copied<..>>,Copied<..>>(56B, 세 라인 미니언) / iter_towers_without_nexus(cache,team) -> Chain<Flatten<array::IntoIter<Option<&Entity>,6>>,Copied<Iter<&Entity>>>(120B) / precompute_champion_powers(version:usize, data:&OperationData, p:&mut ChampionScoreParameter 216B) — p.team(+0x60)·p.pos(+0x68) 읽고 +0xb8 attack_power·+0xc0 util_power_base·+0xc8 cc_time_x_inv_cd·+0xd0 buff_inv_cd_count 씀(m04.ll:53234, utils.rs:792) / Effect::expected_damage_target(&Effect 56B, ctx:&GameContext 64B, caster:&dyn(Entity ptr, vtable @anon.11 88B), target:&Entity 1728B) -> i64 / Projectile::expected_damage_target(&Projectile, ctx, caster:&Entity, target:&Entity) -> i64 / CastingTarget::check_projectile(&CastingTarget(applyed_target), &Projectile, &Entity) -> bool / Projectile::is_in_orbit(&Projectile, x, y, radius) -> bool / Projectile::has_cc(&Projectile) -> bool / Entity::remain_action_time(&Entity) -> usize / Blackboard::is_recent_visible(&Blackboard, game:&dyn AbstractGame, player:&PlayerState, e:&Entity) -> bool (_gcbc g07.ll:157005 — game.is_visible(player.team, e.id) || last_visible[e.pos]+120 >= game.tick(); 상수 120 은 game_core 본문이라 constants 에 미등록) / AbstractGame vtable +0x1f0 get_entity_by_id(id)->Option<&Entity> · +0x210 iter_projectile()->ProjectileIter(40B) / ProjectileIter::next -> Option<&Projectile>.

L1462  let team = player.info.team;  // <2 아니면 panic_bounds_check
       let champ = data.cache.player_champion[team][player.info.position as usize].unwrap();  // null 이면 unwrap_failed
L1463  let mut parameter = ScoreParameter { wave_snapshot: None, player: ChampionScoreParameter{ action: RunAway(태그 0, 페이로드 미기록), risk_possible: Vec::new_in(pool), gain_possible: Vec::new_in(pool), id: 0, team, pos, 나머지 usize/i64 전부 0 }, positioning_score: value 49원소 ×50B 0(패딩 6B 미기록) · cx 0 · cy 0, near_allies: Vec::new_in(pool), near_enemies: Vec::new_in(pool), version, v3_turnback_hold: false };
       // pool = data.context.pool(L1466). 빈 bumpalo Vec = {ptr 8(dangling), a pool, cap 0, len 0}
L1496~1498  let near_minions: Vec<&Entity, pool> = cache.iter_minions(1-team).filter(|m| distance_sq(m,champ) < 150000²).chain(cache.iter_minions(team).filter(|m| distance_sq(m,champ) < 150000²)).collect_in(pool);   // 적 라인미니언 먼저, 아군 뒤. 배치 A 에서는 안 읽힘(뒤 배치 입력)
L1500  parameter.player.id = champ.id;
L1501  precompute_champion_powers(version, data, &mut parameter.player);
L1504~1508  let near_allies_with_action: Vec<(SmallAction, usize, &Entity), pool> = cache.player_champion[team].iter().enumerate()
       .map(|(i,c)| (data.blackboard[player.team].small_actions[i], i, c))            // L1506 (aux s0_0)
       .filter(|(a,_,c)| a.is_some() && c.is_some_and(|e| e.id != champ.id && distance_sq(e,champ) < 200000²+1))   // L1507 (aux s1_0) — 나 자신 제외
       .map(|(a,i,c)| (a.unwrap(), i, c.unwrap())).collect_in(pool);                // L1508
L1511~1517  let near_enemies_with_action = cache.player_champion[1-team].iter().enumerate()
       .map(|(i,c)| (data.blackboard[1-player.team].small_actions[i], i, c))          // L1513 (aux s3_0) — 적 팀 블랙보드의 행동을 그대로 읽음
       .filter(|(a,_,c)| a.is_some() && c.is_some_and(|e| data.blackboard[1-player.team].is_recent_visible(cache.game, player, e) && distance_sq(e,champ) < 200000²+1))   // L1514~1516 (aux s4_0)
       .map(|(a,i,c)| (a.unwrap(), i, c.unwrap())).collect_in(pool);                // L1517
L1521  for (action, pos, e) in near_allies_with_action.iter() {
L1523~1524  let act_tick = max(e.remain_action_time(), e.cc.iter().filter(|c| c.block_input()).map(|c| c.tick()).max().unwrap_or(0));
              // block_input ⟺ CCState 태그 ∉ {Bind, BlockAttack, BlockSkill, BlockMoveSkill}; tick 은 Animation 이면 +0x20 아니면 +0x8; cc 비었으면 act_tick = remain_action_time (select %255)
L1526~1541  let mut p = ChampionScoreParameter { action: *action, risk_possible: new_in(pool), gain_possible: new_in(pool), id: e.id (L1527), team, pos, applyed_damage..risk_possible_tower: 0, action_time: act_tick, attack_value..buff_inv_cd_count: 0 };
L1547  precompute_champion_powers(version, data, &mut p);
L1548  parameter.near_allies.push(p);   // reserve_internal_or_panic(cap==len 일 때) → memcpy 216 → len+1 ; L1549 언와인드 시 p drop
       }
L1551  for (action, pos, e) in near_enemies_with_action.iter() {   // 동일 절차, team = 1-team
L1554~1555  act_tick = max(e.remain_action_time(), cc.filter(block_input).map(tick).max().unwrap_or(0));
L1557~1572  p = ChampionScoreParameter{ .., id: e.id (L1558), team: 1-team, pos, action_time: act_tick, .. };
L1578  precompute_champion_powers(version, data, &mut p);
L1579  parameter.near_enemies.push(p);   // L1580 언와인드 drop
       }
L1582~1587  let near_towers: Vec<&Entity, pool> = cache.iter_towers_without_nexus(1-team)
       .filter(|t| distance_sq(t,champ) < 150000² || near_enemies_with_action.iter().any(|(_,_,e)| distance_sq(e,t) < 70000²))   // L1583~1584 (aux sa_0)
       .chain(cache.iter_towers_without_nexus(team)
         .filter(|t| distance_sq(t,champ) < 150000² || near_allies_with_action.iter().any(|(_,_,e)| distance_sq(e,t) < 70000²)))  // L1586~1587 (aux sb_0)
       .collect_in(pool);   // 배치 A 에서는 안 읽힘
L1593  for proj in cache.game.iter_projectile() {              // vtable+0x210, next() 가 null 이면 종료 → L1686
L1594  let caster = cache.game.get_entity_by_id(proj.caster_id);  // vtable+0x1f0
L1597  let Some(caster) = caster else { continue };
       let mut nearest_other_distance: Option<usize> = None;   // (tag %3440=0, val %3439=undef)
L1598  for a in parameter.near_allies.iter_mut() {           // 원소 216B, 힙 직접 갱신
L1599    let Some(ae) = game.get_entity_by_id(a.id) else { continue };
L1600    if !proj.applyed_target.check_projectile(proj, ae) { continue; }
L1604    if !proj.is_in_orbit(ae.x, ae.y, ae.radius()) { continue; }   // radius() = mult==0 ? radius : radius*(mult+100)/100
L1608    if let Some(target_id) = proj.is_targeting() {   // Target·TargetSplash → move_type+0x10 · BouncingTarget{Some} → +0x8 ; 그 외 None
L1609      if target_id == ae.id {
L1610        a.applyed_damage += proj.expected_damage_target(ctx, caster, ae);
L1611        if proj.has_cc() { a.applyed_cc += 1; }
           }
         } else {
L1616      let dist = distance_sq(proj.(x,y), ae.(x,y));      // (|x1-x2|² + |y1-y2|²)
L1617      nearest_other_distance = Some(nearest_other_distance.map_or(dist, |n| min(n, dist)));   // 비겨냥 투사체가 궤도에 태운 가장 가까운 아군 거리
L1621      a.risk_damage += proj.expected_damage_target(ctx, caster, ae);
L1622      if proj.has_cc() { a.risk_cc += 1; }
         }
       }
L1629  for a in parameter.near_enemies.iter_mut() {
L1630    let Some(ae) = game.get_entity_by_id(a.id) else { continue };
L1631    if !proj.applyed_target.check_projectile(proj, ae) { continue; }
L1635    if !proj.is_in_orbit(ae.x, ae.y, ae.radius()) { continue; }
L1639    if let Some(target_id) = proj.is_targeting() {
L1640      if target_id == ae.id {
L1641        a.applyed_damage += proj.expected_damage_target(ctx, caster, ae);
L1642        if proj.has_cc() { a.applyed_cc += 1; }
           }
         } else {                                             // 적 쪽엔 nearest 갱신·penetrate 예외 없음
L1647      a.risk_damage += proj.expected_damage_target(ctx, caster, ae);
L1648      if proj.has_cc() { a.risk_cc += 1; }
         }
       }
L1655  if !proj.applyed_target.check_projectile(proj, champ) { continue; }
L1659  if !proj.is_in_orbit(champ.x, champ.y, champ.radius()) { continue; }
L1663  if let Some(target_id) = proj.is_targeting() {
L1664    if target_id == champ.id {
L1665      parameter.player.applyed_damage += proj.expected_damage_target(ctx, caster, champ);
L1666~1667 if proj.has_cc() { parameter.player.applyed_cc += 1; }
         }   // 겨냥 대상이 내가 아니면 아무것도 안 함
       } else {
L1671~1672 if let Some(nearest) = nearest_other_distance {   // 앞 아군 루프에서 비겨냥 투사체가 태운 아군이 있을 때만
           let dist = distance_sq(proj.(x,y), champ.(x,y));
L1673      if proj.is_linear_dist_no_penetrate() && dist > nearest { continue; }   // LinearDist(idx 0)·penetrate==false 이고 나보다 가까운 아군이 먼저 맞음 → 나는 위험 아님. (IR 은 `and`(비단락) 후 penetrate 로드: (idx==0 && dist>nearest) 이면 penetrate 검사, penetrate=false 면 continue)
         }
L1678    parameter.player.risk_damage += proj.expected_damage_target(ctx, caster, champ);
L1679~1680 if proj.has_cc() { parameter.player.risk_cc += 1; }
       }
       }  // end for proj
L1686  for (action, e) in near_enemies_with_action.iter() {     // a[0]=태그, a[8]=첫 페이로드(target_id) ; 루프 종료 → %393(배치 B, L1883)
L1687    if *action == SmallAction::RunAway { continue; }        // 태그 0
L1692~1693 let act_tick = max(e.remain_action_time(), e.cc.iter().filter(block_input).map(tick).max().unwrap_or(0));   // 계산만 — 배치 A 범위에서 소비처 없음(배치 B 로 넘어가는 값 %430)
L1695    match action {
         Attack(target_id) => {                                // 태그 6
L1697      let atk = e.attack_effect.as_ref().unwrap();        // +0x4c0 == -1 이면 unwrap_failed
L1698      if target_id == champ.id {
L1699        if (atk.casting ∈ {Position, Direction}) || !e.is_in_attack() {   // is_in_attack = ty==Champion && action_state==Attack. IR 순서: casting 검사 먼저, 거짓이면 is_in_attack
L1700          parameter.player.risk_damage += atk.expected_damage_target(ctx, e, champ);
             } else {
L1702          parameter.player.applyed_damage += atk.expected_damage_target(ctx, e, champ);   // 지정형 캐스팅 && 이미 평타 모션 중 = 확정 피해
             }
           }
L1706      if let Some(target) = parameter.near_allies.iter_mut().find(|x| x.id == target_id) {
L1707        let target_entity = game.get_entity_by_id(target_id).unwrap();
L1708        if (atk.casting ∈ {1,2}) || !e.is_in_attack() {
L1709          target.risk_damage += atk.expected_damage_target(ctx, e, target_entity);
             } else {
L1711          target.applyed_damage += atk.expected_damage_target(ctx, e, target_entity);
             }
           }
           → %431(배치 B, L1781)
         }
         Skill(target_id) => {                                 // 태그 7
L1716      let skill = e.skill_effect.as_ref().unwrap();       // +0x4f8 == -1 이면 unwrap_failed
L1717      if target_id == champ.id {
L1718        if (skill.casting ∈ {1,2}) || !e.is_in_skill() {  // is_in_skill = ty==Champion && action_state==Skill(4)
L1719          parameter.player.risk_damage += skill.expected_damage_target(ctx, e, champ);
             } else {
L1721          parameter.player.applyed_damage += skill.expected_damage_target(ctx, e, champ);
             }
           }
L1725      if let Some(target) = parameter.near_allies.iter_mut().find(|x| x.id == target_id) {
L1726        let target_entity = game.get_entity_by_id(target_id).unwrap();
L1727        if (skill.casting ∈ {1,2}) || !e.is_in_skill() {
L1728          target.risk_damage += skill.expected_damage_target(ctx, e, target_entity);
             } else {
L1730          target.applyed_damage += skill.expected_damage_target(ctx, e, target_entity);
             }
           }
           → %431(배치 B, L1781)
         }
         Skill2(target_id) => {                                // 태그 8
L1735      let skill2 = e.skill2_effect().as_ref().unwrap();   // entity.rs:1693: level > 2 ? &skill2_effect : &None(@anon.31) — 레벨 2 이하면 항상 unwrap_failed(패닉)
L1736      if target_id == champ.id {
L1737        if (skill2.casting ∈ {1,2}) || !e.is_in_skill2() {   // is_in_skill2 = ty==Champion && action_state==Skill2(5)
L1738          parameter.player.risk_damage += skill2.expected_damage_target(ctx, e, champ);
             } else {
L1740          parameter.player.applyed_damage += skill2.expected_damage_target(ctx, e, champ);
             }
           }
           → %601(배치 B, L1744: near_allies find 로 이어짐)
         }
         Ult(target_id) => → %457(배치 B, L1744~)                // 태그 9
         _ (Positioning/Around/AroundPosition/Trace/Dodge/Stop) => → %431(배치 B, L1781)
       }
       }
// 사장 코드: reach.py(version=2·gamemode=0) 결과 사장 블록 0 · 사장 호출부 0 — NA 봉인 대상 없음.
// 언와인드: 모든 invoke 의 unwind 는 %146/%158/%189/%241/%365(배치 D L2418 cleanup: near_* Vec·ChampionScoreParameter drop) 로 간다.

// score_parameter.rs:1744~1962 (배치 B)
// 문맥(배치 A): 줄 1686 `for (a, e) in near_enemies_with_action` 루프 안. a=SmallAction(적 e 의 현재 소액션), e=&Entity(적). act_tick(1692~1693) = max(e.remain_action_time(), e.cc 중 block_input 인 것의 tick 최대). 줄 1695 match a: Attack(6)→1697.. Skill(7)→1716.. Skill2(8)→1735.. Ult(9)→1756.. _→{}. 각 arm 이 끝나면 1779 로 합류(continue 아님).

// ── Skill2 arm 꼬리 (1744~1749) ── (1735~1743 = 배치 A: target_id==내 id 분기)
1744: if let Some(target) = parameter.near_allies.iter_mut().find(|x| x.id == target_id) {          // near_allies buf 0x14b8/len 0x14d0, 원소 216B, x.id=+0x58 (closure#19 인라인)
1745:   let target_entity = data.cache.game.get_entity_by_id(target_id).unwrap();                 // vtable+0x1f0 간접호출, None→unwrap_failed(@anon.130)
1746:   if skill2.is_nontarget() || !e.is_in_skill2() {   // is_nontarget(type.rs:149)=casting∈{Position,Direction} 를 먼저 평가, 거짓일 때만 is_in_skill2(entity.rs:1579~1580: ty==Champion(13) && action_state==Skill2(5))
1747:     target.risk_damage(+0x80) += skill2.expected_damage_target(data.context, e as &dyn AbstractEntity, target_entity);
1748:   } else {
1749:     target.applyed_damage(+0x70) += skill2.expected_damage_target(...같은 인자);
1750~1752: } } }  (1753~1754 한글 주석 · 1755 = `SmallAction::Ult { target_id } => {` 추정(줄길이 42=8+34))

// ── Ult arm (1756~1770) ──
1756: let ult = e.ult_effect().as_ref().unwrap();          // Entity::ult_effect(entity.rs:1701) 인라인: level(0x5c8) > 4 ? Some(&ult_effect@0x538) : None ; casting@tag==-1 → unwrap_failed(@anon.131)
1757: if target_id == my_id(%157) {
1758:   if ult.is_nontarget() || !e.is_in_ult() {           // is_in_ult(entity.rs:1586~1587): ty==13 && action_state==6
1759:     parameter.player.risk_damage(0x998) += ult.expected_damage_target(ctx, e, me);
1760:   } else {
1761:     parameter.player.applyed_damage(0x988) += ult.expected_damage_target(ctx, e, me);
1762~1764: } }
1765: } else if let Some(target) = parameter.near_allies.iter_mut().find(|x| x.id == target_id) {   // closure#20
1766:   let target_entity = game.get_entity_by_id(target_id).unwrap();     // @anon.132
1767:   if ult.is_nontarget() || !e.is_in_ult() {
1768:     target.risk_damage += ult.expected_damage_target(ctx, e, target_entity);
1769:   } else {
1770:     target.applyed_damage += ult.expected_damage_target(ctx, e, target_entity);
1771~1777: 닫는 괄호들 (1778 한글 주석)

// ── 모든 적 e 에 공통 (1779~1875) — match 뒤 합류점 %431 ──
1779: let e_speed = e.stat_cached.move_speed(0x640);
1780: (IR 없음·39자) 추정 `let e_atk_eff = &e.attack_effect;` (dbg 이름 e_atk_eff=e)
1781: let atk = e_atk_eff.as_ref().unwrap();               // attack_effect@tag(0x4c0)==-1 → unwrap_failed(@anon.133) — 공격 이펙트 없는 적이 목록에 있으면 패닉
1782: let e_atk_range = atk.range(e) + e.radius() [+ e_speed*20 — 이 항은 dbg 없어 1782/1793 귀속 불가];   // Effect::range(effect.rs:26) = atk.range(0x4a0) + atk.growth_range(0x4a8)*(e.level-1) + e.stat_buff_cached.range(0x438) ; Entity::radius = radius_mult(0x470)==0 ? radius(0x680) : radius*(mult+100)/100
1783: let e_attack_tick = act_tick.max(e.attack_cooldown());   // entity.rs:1748~1760 타입별 필드(reads 0xb0 항목) · None/Nexus 는 0
1784: let e_skill_tick  = act_tick.max(e.skill_cooldown());    // 챔피언만 0xb8, 그 외 0
1785: let e_skill2_tick = act_tick.max(e.skill2_cooldown());   // 챔피언만 0xc0
1786: (IR 없음·40자) 추정 `let e_skill_eff = e.skill_effect();`류 (dbg 이름 e_skill_eff=e — as_ref 는 1807 에서 casting@tag(0x4f8)!=-1 로 판정)
1787: let e_skill2_eff = e.skill2_effect();                  // entity.rs:1693: level>2 ? Some(&skill2_effect@0x500) : None(@anon.31 상수 = casting -1)
1788: for a in parameter.near_allies.iter_mut() {           // a: &mut ChampionScoreParameter
1789:   let Some(ae) = game.get_entity_by_id(a.id) else { continue };   // null → 다음 원소
1790:   let dist = e.distance_sq(ae);                        // (|ex-aex|² + |ey-aey|²) abs_diff, entity.rs:2158→utils.rs:7~9
1793:   let atk_reach = e_atk_range + e_speed*20 + atk.range_adjust(e, ae) + ae.radius();   // 덧셈 순서: (speed*20 + e_atk_range) + range_adjust, 그 뒤 + ae.radius (add 명령은 1797 로 표기됨)
1797:   if e_attack_tick < 121 && atk_reach*atk_reach >= dist {     // %798 을 먼저 분기, 그 다음 제곱비교(`icmp ult reach², dist` 가 참이면 건너뜀)
1798:     a.risk_possible.push(PossibleGain{ from: e.id(0x5c0) /*1799*/, tick: e_attack_tick, value: atk.expected_damage_target(ctx, e, ae) /*1800*/ });
1801~1806: (닫는 괄호 · 1801/1805 IR 없음)
1807:   if let Some(skill) = e_skill_eff /* skill_effect@0x4c8, casting@0x4f8 != -1 */ {
1808:     if skill.target(0x4f0).check(e, ae) {               // CastingTarget::check(&self, caster=e, target=ae) → bool
1809:       let skill_reach = skill.range(e) + e_speed*20 + skill.range_adjust(e, ae) + e.radius();   // Effect::range 인라인: skill.range(0x4d8)+growth(0x4e0)*(level-1)+stat_buff_cached.range(0x438)
1810:       if e_skill_tick < 121 && (skill_reach + ae.radius())² >= dist {   // %809 먼저, 그 다음 제곱비교
1811:         a.risk_possible.push(PossibleGain{ from: e.id /*1812*/, tick: e_skill_tick, value: skill.expected_damage_target(ctx, e, ae) /*1813*/ });
1814~1821: 닫는 괄호(1814/1820 IR 없음)
1822:   if let Some(skill2) = e_skill2_eff /* casting(+0x30)!=-1 */ {
1823:     if skill2.target(+0x28).check(e, ae) {
1824:       let skill2_reach = skill2.range(e) + e_speed*20 + skill2.range_adjust(e, ae) + e.radius();
1825:       if e_skill2_tick < 121 && (skill2_reach + ae.radius())² >= dist {
1826:         a.risk_possible.push(PossibleGain{ from: e.id /*1827*/, tick: e_skill2_tick, value: skill2.expected_damage_target(ctx, e, ae) /*1828*/ });
1829~1836: 닫는 괄호 } // for near_allies 끝   ※궁(ult)은 이 루프에서 평가하지 않음

// ── 적 e 가 '나'에게 닿는가 (1837~1875) ──
1837: let dist = e.distance_sq(me);                          // me = cache.player_champion[team][pos] (%59), x/y 0x660/0x668
1838~1839: (IR 없음·48/51자 — 정체 미확정)
1840: let atk_reach = atk.range(e) + e_speed*20 + atk.range_adjust(e, me) + me.radius() + e.radius();   // atk.range 를 다시 계산(reload 0x4a0/0x4a8)
1842: let attack_tick = act_tick.max(e.attack_cooldown());   // 1783 과 같은 값을 재계산
1844: if attack_tick < 121 && atk_reach² >= dist {
1845:   parameter.player.risk_possible.push(PossibleGain{ from: e.id /*1846*/, tick: attack_tick, value: atk.expected_damage_target(ctx, e, me) /*1847*/ });
1848~1851: }
1852: let skill_tick = act_tick.max(e.skill_cooldown());      // 챔피언이면 0xb8, 아니면 act_tick
1854: if let Some(skill) = e.skill_effect (casting 0x4f8 != -1) {
1855:   if skill.target.check(e, me) {
1856:     let skill_reach = skill.range(e) + e_speed*20 + skill.range_adjust(e, me) + me.radius() + e.radius();
1857:     if skill_tick < 121 && skill_reach² >= dist {
1858:       parameter.player.risk_possible.push(PossibleGain{ from: e.id /*1859*/, tick: skill_tick, value: skill.expected_damage_target(ctx, e, me) /*1860*/ });
1861~1866: }}}
1867: let skill2_tick = act_tick.max(e.skill2_cooldown());
1869: if let Some(skill2) = e_skill2_eff {
1870:   if skill2.target.check(e, me) {
1871:     let skill2_reach = skill2.range(e) + e_speed*20 + skill2.range_adjust(e, me) + me.radius() + e.radius();
1872:     if skill2_tick < 121 && skill2_reach² >= dist {
1873:       parameter.player.risk_possible.push(PossibleGain{ from: e.id /*1874*/, tick: skill2_tick, value: skill2.expected_damage_target(ctx, e, me) /*1875*/ });
1876~1882: }}} → 루프 다음 원소(%1144 → 배치 A 줄 1686 헤더 %390)

// ── 아군 행동 루프 (1883~1962, 배치 C 로 이어짐) ──
1883: for (a, e) in near_allies_with_action.iter() {         // %41 (bumpalo Vec, 원소 40B: +0 SmallAction, +0x18 i64(미사용), +0x20 &Entity)
1884:   if *a == SmallAction::RunAway { continue; }          // 태그 0 (blackboard.rs:81 derived PartialEq)
1885~1887: (괄호)
1888: let act_tick = e.remain_action_time().max(            // Entity::remain_action_time(&self)->i64 (game_core 정의 g06.ll:67110)
1889:     e.cc.iter().filter(|c| c.block_input()).map(|c| c.tick()).max().unwrap_or(0));   // closure#21/#22 · 첫 원소는 인라인, 나머지는 aux fold(m12.ll:19396) · 빈 슬라이스면 remain_action_time 그대로(select)
1891: match a {    // switch 태그 6/7/8/9, 그 외 → 1944
1892:   SmallAction::Attack { target_id } => {                // (45자=8+37, IR 없음)
1893:     let atk = e.attack_effect.as_ref().unwrap();        // 0x4c0==-1 → unwrap_failed(@anon.134)
1894:     if let Some(target) = parameter.near_enemies.iter_mut().find(|x| x.id == target_id) {   // near_enemies buf 0x14d8/len 0x14f0 (closure#23)
1895:       let target_entity = game.get_entity_by_id(target_id).unwrap();   // @anon.135
1896:       if atk.is_nontarget() || !e.is_in_attack() {     // is_in_attack(entity.rs:1564~1565): ty==13 && action_state==3
1897:         target.risk_damage += atk.expected_damage_target(ctx, e, target_entity);
1898:       } else {
1899:         target.applyed_damage += atk.expected_damage_target(ctx, e, target_entity);
1900~1902: }}}
1903:   SmallAction::Skill { target_id } => {
1904:     let skill = e.skill_effect.as_ref().unwrap();       // 0x4f8==-1 → @anon.136
1905:     if let Some(target) = near_enemies.find(id==target_id) {      // closure#24
1906:       let target_entity = game.get_entity_by_id(target_id).unwrap();   // @anon.137
1907:       if skill.is_nontarget() || !e.is_in_skill() {    // entity.rs:1571~1572: ty==13 && action_state==4
1908:         target.risk_damage += skill.expected_damage_target(ctx, e, target_entity);
1909:       } else {
1910:         target.applyed_damage += skill.expected_damage_target(ctx, e, target_entity);
1911~1913: }}}
1914:   SmallAction::Skill2 { target_id } => {
1915:     let skill2 = e.skill2_effect().as_ref().unwrap();   // level>2 게이트 + casting!=-1, 실패 → @anon.138
1917:     if let Some(target) = near_enemies.find(id==target_id) {      // closure#25
1918:       let target_entity = game.get_entity_by_id(target_id).unwrap();   // @anon.139
1919:       if skill2.is_nontarget() || !e.is_in_skill2() {  // action_state==5
1920:         target.risk_damage += skill2.expected_damage_target(ctx, e, target_entity);
1921:       } else {
1922:         target.applyed_damage += skill2.expected_damage_target(ctx, e, target_entity);
1923~1926: }}} (1926 한글 주석)
1927:   SmallAction::Ult { target_id } => {
1928:     let ult = e.ult_effect().as_ref().unwrap();         // level>4 게이트, 실패 → @anon.140
1930:     if let Some(target) = near_enemies.find(id==target_id) {      // closure#26
1931:       let target_entity = game.get_entity_by_id(target_id).unwrap();   // @anon.141
1932:       if ult.is_nontarget() || !e.is_in_ult() {        // action_state==6
1933:         target.risk_damage += ult.expected_damage_target(ctx, e, target_entity);
1934:       } else {
1935:         target.applyed_damage += ult.expected_damage_target(ctx, e, target_entity);
1936~1942: }}} _ => {} }  (1943 한글 주석)
// 합류점 %1230 (모든 아군 e, RunAway 제외)
1944: let e_speed = e.stat_cached.move_speed;
1945: (IR 없음·39자) 추정 `let e_atk_eff = &e.attack_effect;`
1946: let atk = e_atk_eff.as_ref().unwrap();                // 0x4c0==-1 → @anon.142
1947: let e_atk_range = atk.range(e) + e.radius() [+ e_speed*20];   // 1782 와 동형
1948: let e_attack_tick = act_tick.max(e.attack_cooldown());
1949: let e_skill_tick  = act_tick.max(e.skill_cooldown());
1950: let e_skill2_tick = act_tick.max(e.skill2_cooldown());
1951: (IR 없음·40자) 1786 과 동형 추정
1952: let e_skill2_eff = e.skill2_effect();                  // level>2
1953: for a in parameter.near_enemies.iter_mut() {          // 원소 216B
1954:   let Some(ae) = game.get_entity_by_id(a.id) else { continue };
1955:   let dist = e.distance_sq(ae);
1956~1957: (IR 없음·29/25자 — 정체 미확정, 1791~1792 와 같은 길이)
1958:   let atk_reach = e_atk_range + e_speed*20 + atk.range_adjust(e, ae) + ae.radius();
1962:   if e_attack_tick < 121 && atk_reach² >= dist {       // %1503 먼저, 그 다음 제곱비교
1963:     → 배치 C(줄 1963): a.risk_possible(+0x18).push(PossibleGain{from: e.id /*1964*/, tick: e_attack_tick, value: atk.expected_damage_target(ctx, e, ae) /*1965*/}) — 원소 store 명령들이 1963 루트
      거짓이면 → 배치 C(%1565, 줄 1966 이후 skill/skill2 판정: 1514/1520 에 호이스트된 `e_skill_tick<121`/`e_skill2_tick<121` 사용)
// 루프 끝 → %1203 → 1883 헤더 ; 목록 소진 → %1701(배치 C, 1883 다음)

// score_parameter.rs:1963~2177 (배치 C)
// ── 문맥(배치 B 에서 이어짐) ──
// L1883 `for a in near_allies_with_action`(bumpalo Vec<SmallAction<&Entity>> 40B · e = 원소+0x20 = 아군 엔티티 %1201) 안의
// L1953 `for a in parameter.near_enemies.iter_mut()`(216B · %1522) · L1954 `if let Some(ae) = game.get_entity_by_id(a.id)` · L1955 dist = distance_sq(e, ae)(%1549)
// e_attack_tick(%1489 B L1948) · e_skill_tick(%1490 B L1949) · e_skill2_tick(%1491 B L1950) · e_skill2_eff(%1494 = e.skill2_effect() B L1952: level>2 ? &e.skill2_effect : &NONE)
// L1962(B): `if e_attack_tick < 121 && dist <= reach_atk²` 참이면 ↓ L1963, 거짓이면 L1972 로

// ── L1963~1966: 아군 e 의 평타 위협을 적 a 에 적재 ──
a.risk_possible.push(PossibleGain{ from: e.id /*L1964 0x5c0*/, tick: e_attack_tick, value: e.attack_effect(0x490).expected_damage_target(ctx, e as &dyn AbstractEntity, ae) /*L1965*/ })
//   push = cap==len 이면 reserve_internal_or_panic(vec,len,1,true) → ptr[len] = {from,tick,value} → len+=1 (m07.ll:41866~41903)

// ── L1972~1978: 아군 e 의 스킬 위협 ──
if let Some(skill) = &e.skill_effect {            // L1972: 태그 0x4f8 != -1 (m07.ll:41835~41837) · None → L1987
  if skill.target.check(e, ae) {                    // L1973: CastingTarget::check(&skill.target(0x4f0), e, ae) (41909) · false → L1987
    // L1974 reach = skill.range(e) + skill.range_adjust(e, ae) + e.radius() + ae.radius()
    //   skill.range(e)(effect.rs:26) = e.stat_buff_cached.range(0x438) + skill.range(0x4d8) + skill.growth_range(0x4e0)×(e.level-1)  [+ e.move_speed×20 는 같은 합에 포함(%1515 = 0x438 + speed×20, B 호이스트)]
    //   Entity::radius() = radius_mult(0x470)==0 ? radius(0x680) : radius×(mult+100)/100  (e: %1611, ae: %1619)
    //   (41922~41960 · range_adjust 호출 41924)
    if e_skill_tick < 121 /*41952 · 먼저 평가*/ && dist <= reach*reach /*41961~41963: reach² < dist 면 스킵*/ {   // L1975 (같은 줄 안 A/B 순서는 column 부재로 표기 불가 · IR 분기 순서는 tick→dist)
      a.risk_possible.push(PossibleGain{ from: e.id /*L1977*/, tick: e_skill_tick, value: skill.expected_damage_target(ctx, e, ae) /*L1978*/ })   // L1976 (41971~42021)
    }
  }
}
// ── L1987~1993: 아군 e 의 스킬2 위협 ──
if let Some(skill2) = e_skill2_eff {              // L1987: %1494+0x30 태그 != -1 (41914~41916) · None → L1953 다음 a (배치 B %1551)
  if skill2.target.check(e, ae) {                   // L1988 (42027) · false → 다음 a
    // L1989 reach = skill2.range(e) + skill2.range_adjust(e, ae) + e.radius() + ae.radius() [+ speed×20]  (42034~42072)
    if e_skill2_tick < 121 /*42064*/ && dist <= reach*reach /*42073~42075*/ {   // L1990
      a.risk_possible.push(PossibleGain{ from: e.id /*L1992*/, tick: e_skill2_tick, value: skill2.expected_damage_target(ctx, e, ae) /*L1993*/ })   // L1991 (42083~42134)
    }
  }
}
// → 다음 a(L1953) → 다음 아군 액션(L1883) → 모두 끝나면 L2004

// ── L2004~2052: 자기(parameter.player) 액션 대상에 대한 대미지 확정 ──
if parameter.player.action != SmallAction::RunAway {   // L2004: 0x918 태그 != 0 (42138~42144) · RunAway 면 L2111 로 직행(L2005~2100 전부 스킵)
  // L2005~2006 act_tick = max(champ.remain_action_time() /*42147*/, champ.cc.iter().filter(|c| matches!(c, Airborne|Stun|ForceMove|Taunt|Fear|Charm|Animation)).map(|c| c.tick).max().unwrap_or(0))
  //   필터 = (tag-6) <u -4 (42223~42226) · tick 은 Animation(10)만 +0x20, 나머지 +0x8 (42237~42240) · 나머지 원소는 aux fold(m12.ll 19505~) · CC 비었으면 remain_action_time 그대로(select 42253)
  match parameter.player.action {   // L2008 switch on 0x918 (42255~42261) · Attack 6 / Skill 7 / Skill2 8 / Ult 9 / 그 외 → L2060
    Attack{target_id} => {   // L2009 target_id = 0x920 (42307)
      let atk = champ.attack_effect.as_ref().unwrap();          // L2010 (0x4c0 == -1 → unwrap_failed anon.143 · 42399)
      if let Some(target) = parameter.near_enemies.iter_mut().find(|x| x.id == target_id) {   // L2011 (42359~42395) · 없으면 L2060
        let target_entity = game.get_entity_by_id(target_id).unwrap();   // L2012 (vtable+0x1f0 42405 · None → anon.144 42422)
        if atk.casting.is_nontarget() /*(casting-1)<u2 · 42416~42419*/ || !champ.is_in_attack() /*ty==13 && action_state==3 · 42426~42440*/ {   // L2013 (분기 방향으로 복원 · 줄 안 순서는 표기 불가)
          target.risk_damage(+0x80) += atk.expected_damage_target(ctx, champ, target_entity)     // L2014 (42432 · 42454~42457)
        } else {
          target.applyed_damage(+0x70) += atk.expected_damage_target(ctx, champ, target_entity)  // L2016 (42443 · 42447~42450)
        }
      }
    }
    Skill{target_id} => {   // L2020~2027: 위와 동형 · skill = champ.skill_effect.as_ref().unwrap()(L2021 · 0x4f8 · anon.145) · find L2022(closure$30) · get_entity L2023(anon.146) · L2024 is_nontarget || !is_in_skill(action_state==4) → L2025 risk_damage += / else L2027 applyed_damage +=
    }
    Skill2{target_id} => {  // L2031~2039: skill2 = champ.skill2_effect().unwrap()(L2032 · level>2 게이트 후 0x530 · anon.147) · find L2034(closure$31) · get_entity L2035(anon.148) · L2036 is_nontarget || !is_in_skill2(==5) → L2037 risk_damage += / else L2039 applyed_damage +=
    }
    Ult{target_id} => {     // L2044~2052: ult = champ.ult_effect().unwrap()(L2045 · level>4 ? &ult_effect(0x538) : &NONE · anon.149) · find L2047(closure$32) · get_entity L2048(anon.150) · L2049 is_nontarget || !is_in_ult(==6) → L2050 risk_damage += / else L2052 applyed_damage +=
    }
    _ => {}
  }

  // ── L2060~2100: 자기 챔피언의 평타/스킬/스킬2 위협을 각 적 a 에 적재 ──
  for a in parameter.near_enemies.iter_mut() {   // L2060 (42266~42304 · 42778~42788)
    if let Some(ae) = game.get_entity_by_id(a.id) {   // L2061 (42794~42802) · None → 다음 a
      let dist = distance_sq(champ, ae);             // L2062 (42810~42837 · champ.x/y = %373/%374)
      let speed = champ.stat_cached.move_speed;      // L2063 (42838)
      let atk = champ.attack_effect.as_ref().unwrap();   // L2064 (0x4c0 · anon.151 42862)
      // L2065 reach_atk = atk.range(champ) + atk.range_adjust(champ, ae) + ae.radius() + champ.radius()   (42852~42903 · 43000~43002)
      let attack_tick = max(act_tick, champ.attack_cooldown());   // L2067 (entity.rs:1748~1760 인라인 = ty 별 14-way switch 42904~42983 · Champion → 0xb0 · umax 42986)
      if attack_tick < 121 /*42988*/ && dist <= (reach_atk + speed*20)² /*42996~43005*/ {   // L2069
        a.risk_possible.push(PossibleGain{ from: champ.id, tick: attack_tick, value: atk.expected_damage_target(ctx, champ, ae) /*L2072 43008*/ })   // L2070 (43012~43062)
      }
      let skill_tick = if champ.ty==Champion { max(champ.skill_cooldown(0xb8), act_tick) } else { act_tick };   // L2077 (entity.rs:1775~1776 · 42992~42993 · 43067~43072)
      if let Some(skill) = &champ.skill_effect {      // L2079 (43077~43079)
        if skill.target.check(champ, ae) {           // L2080 (43084 · 43091)
          // L2081 reach = skill.range(champ) + skill.range_adjust(champ, ae) + ae.radius() + champ.radius()  (43094~43124)
          if skill_tick < 121 /*43125*/ && dist <= (reach + speed*20)² /*43130~43139*/ {   // L2082
            a.risk_possible.push(PossibleGain{ from: champ.id, tick: skill_tick, value: skill.expected_damage_target(ctx, champ, ae) /*L2085 43142*/ })   // L2083 (43146~43196)
          }
        }
      }
      let skill2_tick = if champ.ty==Champion { max(champ.skill2_cooldown(0xc0), act_tick) } else { act_tick };   // L2092 (entity.rs:1790~1791 · 43088 · 43201~43206)
      if let Some(skill2) = champ.skill2_effect() {   // L2094 (level>2 게이트 43210~43216 · None → 다음 a)
        if skill2.target.check(champ, ae) {           // L2095 (43222~43226)
          // L2096 reach = skill2.range(champ) + skill2.range_adjust(champ, ae) + ae.radius() + champ.radius()  (43229~43261)
          if skill2_tick < 121 /*43262*/ && dist <= (reach + speed*20)² /*43267~43276*/ {   // L2097
            a.risk_possible.push(PossibleGain{ from: champ.id, tick: skill2_tick, value: skill2.expected_damage_target(ctx, champ, ae) /*L2100 43279*/ })   // L2098 (43283~43334)
          }
        }
      }
      // ⚠ 이 루프에는 Ult 위협 적재가 없다(스킬2까지만)
    }
  }
}

// ── L2111~2117: 타워 공격 불능 시점 게이트 ──
let disable_tick = match ctx.tutorial.player_count() /*runner.rs:295 인라인 · ctx+0x38 switch 43153~43169*/ {
  // First(1)/Bottom(3) → setting.tower_attack_disable_tick_2v2(+0x1400)   (switch 직결 arm · 줄 소실 · 추정 L2112)
  // MidBottom(5)       → setting.tower_attack_disable_tick_3v3(+0x1408)   (L2113 · 43337)
  // None/TopSolo/MidSolo/JungleOnly/Line/Total → setting.tower_attack_disable_tick(+0x13f8)   (L2114 · 43340) · 태그 9 이상 → %756 (배치 B 1782 unreachable 계열)
};
if game.tick() /*vtable+0x28 · 43347~43349*/ > disable_tick {   // L2117 (43353~43354) — 참이면 L2118~2264 전부 건너뛰고 L2265 로 (→ 배치 D 줄 2265, 블록 %2822)
  → 배치 D(줄 2265)
}
// 아래는 tick <= disable_tick 일 때만

// ── L2118~2161: 적 타워가 자기 챔피언을 위협하는가 ──
for t in near_towers.iter() {   // L2118 (%31 지역 Vec<&Entity> · 43360~43395)
  if t.team == champ.team { continue }   // L2119 Entity::eq(entity.rs:1127): 태그 같고(둘 다 Player 면 +0x8 팀번호까지 같음) → 스킵 (43401~43410 · 43435~43436 · 43453~43456)
  // L2123 is_near_tower_range(t, champ.x, champ.y, r = champ.radius(), d = champ.move_speed*30)  (score_parameter.rs:1449~1453 인라인 · 43439~43558)
  //   1449: if let Some(attack) = &t.attack_effect else false   (0x4c0 == -1 → 스킵 43486~43489)
  //   1450: range = attack.range(t) = t.stat_buff_cached.range + attack.range + attack.growth_range×(t.level-1)
  //   1451: dist = distance_sq((x,y), (t.x,t.y))
  //   1452: check = d + r + range + t.radius()
  //   1453: dist <= check²   (dist > check² → 스킵 43556~43558)
  if !is_near_tower_range(...) { continue }
  if let EntityType::Tower(info) = &t.ty {   // L2129 (0x68 == 2 · 43562~43565) · 아니면 스킵
    if let Some((_, id)) = info.nearest_enemy {   // L2131 (0x88 trunc→1 · id = 0x98 · 43569~43580)
      if id == champ.id {                              // L2132 (43581~43582)
        parameter.player.risk_damage(0x998) += t.attack_effect.expected_damage_target(ctx, t, champ)   // L2133 (43811 · 43815~43817)
      } else if let Some(target) = game.get_entity_by_id(id) {   // L2134 (43585~43591)
        if target.is_champion() {                      // L2135 (entity.rs:1404 · ty==13 · 43596~43599)
          if let Some(player) = cache.player_by_champion_id(id) {   // L2144 (fastcc 43608~43610) · None → 스킵
            // L2145 team = player.info.team(0x930 · bounds<2 아니면 panic_bounds_check anon.152) · pos = player.info.position(0x9c0).as_index()
            if data.blackboard[team].small_actions[pos] == Some(SmallAction::RunAway) {   // L2146 (Blackboard+0x78 + pos×24 · 태그 0 · 43641~43642)
              parameter.player.risk_possible_tower(0x9b0) += t.attack_effect.unwrap().expected_damage_target(ctx, t, champ)   // L2147 (unwrap anon.153 · 43653 · 43662~43664)
            }
          }
        } else {
          // L2136~2139 cnt = cache.iter_minions(player.info.team /*%49 = 자기 팀*/).filter(|m| t.attack_effect.as_ref().unwrap().is_in_range(t, m)).count()   (closure$33 · 43615 · 43669~43706 · aux m11.ll 25009~)
          if cnt < 2 {   // L2141 (43707~43708)
            parameter.player.risk_possible_tower += t.attack_effect.unwrap().expected_damage_target(ctx, t, champ)   // L2142 (anon.154 · 43723 · 43732~43734)
          }
        }
      } else {
        // L2151~2154 cnt = 위와 동일 count (closure$34 · 43604 · 43739~43776 · aux m11.ll 25104~)
        if cnt < 2 {   // L2156 (43777~43778)
          parameter.player.risk_possible_tower += t.attack_effect.unwrap().expected_damage_target(ctx, t, champ)   // L2157 (anon.155 · 43793 · 43802~43804)
        }
      }
    } else {
      parameter.player.risk_possible_tower += t.attack_effect.unwrap().expected_damage_target(ctx, t, champ)   // L2161 (nearest_enemy None · 43824 · 43828~43830)
    }
  }
}

// ── L2170~2177: 적 챔피언 p 각각에 대해 타워 위협 (본체는 배치 D) ──
for p in parameter.near_enemies.iter_mut() {   // L2170 (43833~43843) · 루프 종료 → %2456 = 배치 D(줄 2217)
  if let Some(champ_e) = game.get_entity_by_id(p.id) {   // L2171 (43849~43853 · 43878~43879) · DI 이름 `champ`(적 챔피언 엔티티 %2455)
    for t in near_towers.iter() {   // L2172 (43887~43925)
      if t.team == champ_e.team { continue }   // L2173 (Entity::eq · 43931~43964)
      if is_near_tower_range(t, champ_e.x, champ_e.y, champ_e.radius(), champ_e.move_speed*30) {   // L2177 (43947~44066 · 실패 → 다음 t %2571)
        → 배치 D(줄 2183)   // 블록 %2563 · 배치 D 가 p.risk_possible_tower(+0x98 · %2475)/p.risk_damage(+0x80 · %2477)/champ_e.id(%2476) 를 준비된 포인터로 받는다
      }
    }
  }
}
// (배치 B 로 되돌아가는 간선: L1987/1988/1990 실패 → %1551 = L1953 루프 증분 · L1972/1973/1975 실패 → %1596 = L1987)

// score_parameter.rs:2183~2418 (배치 D)
// ── 전제(배치 A~C 산출 · DI 이름) ──
// parameter = 로컬 %47(ScoreParameter 5384B) · me = %59(내 챔프 &Entity, cache.player_champion[team][pos]) · my_id = %157(me.id) · team = %49(player.info.team) · %144 = 1-team
// cache = %55(data.cache) · ctx = %64(data.context) · game = (%177 data, %179 vtable) = cache.game(&dyn AbstractGame) · get_entity_by_id = vtable+0x1f0(%372) · tick() = vtable+0x28(%2232, 배치 C 2117 에서 로드)
// near_towers = %31(bumpalo Vec<&Entity>, 배치 B) · near_minions = %46 · near_allies/near_enemies = parameter+0x14b8/+0x14d8(원소 ChampionScoreParameter 216B)
// dmg(e, target) := e.attack_effect.as_ref().unwrap().expected_damage_target(ctx, e as &dyn(vtable @anon.11), target)  — attack_effect None(+0x4c0 == -1) 이면 core::option::unwrap_failed(패닉)
// find(vec, id) := vec.iter_mut().find(|x| x.id(+0x58) == id)
//
// ── [C 꼬리] 2170 `for p in parameter.near_enemies.iter_mut()`: champ = game.get_entity_by_id(p.id)? (None→다음 p) · 2172 `for t in near_towers`: 2173 t.team == champ.team → continue · 2177 !is_near_tower_range(t, champ.x, champ.y, champ.radius(), champ.move_speed*30) → continue
//   (is_near_tower_range 1449~1453 인라인: attack = t.attack_effect.as_ref()? else false · range = attack.range + t.stat_buff_cached.range + (t.level-1)*attack.growth_range(effect.rs:26) · dist = |x-t.x|²+|y-t.y|² · check = d + r + range + t.radius() · true iff !(dist > check²))
// 2183 if let Tower(info) = &t.ty {                                     // +0x68 == 2
// 2185   if let Some((_, id)) = info.nearest_enemy {                   // +0x88 tag==1 · id = +0x98(.1)
// 2186     if id == champ.id {                                          // +0x5c0
// 2187       p.risk_damage(+0x80) += dmg(t, champ)
// 2188     } else if let Some(other) = game.get_entity_by_id(id) {
// 2189       if !other.is_champion() {                                  // +0x68 != 13 (is_champion 이면 아무것도 안 함 — `if X {}` 인지 `if !X {..}` 인지는 표기 불가)
// 2190         p.risk_possible_tower(+0x98) += dmg(t, champ)
//            }
// 2193     } else {  // 타깃 엔티티 소실
// 2194       let cnt = cache.iter_minions(1 - team)                     // 적 팀(=타워의 적) 미니언 top∪mid∪bottom
// 2196                   .filter(|m| t.attack_effect.as_ref().unwrap().is_in_range(t, m)).count();   // aux closure sx_0
// 2199       if cnt < 3 { 2200 p.risk_damage += dmg(t, champ) }
// 2202       else       {      p.risk_possible_tower += dmg(t, champ) }
//          }
// 2206   } else { p.risk_possible_tower += dmg(t, champ) }             // nearest_enemy None
//      }   // (Tower 아니면 아무것도 안 함) → 다음 t
//
// 2217 for p in parameter.near_allies.iter_mut() {
// 2218   let Some(champ) = game.get_entity_by_id(p.id) else { continue };
// 2219   for t in near_towers {
// 2220     if t.team == champ.team { continue }                           // TeamType::eq
// 2224     if !is_near_tower_range(t, champ.x, champ.y, champ.radius(), champ.stat_cached.move_speed * 30) { continue }   // 위와 동일 인라인(attack_effect None → false → continue)
// 2230     if let Tower(info) = &t.ty {
// 2232       if let Some((_, id)) = info.nearest_enemy {
// 2233         if id == champ.id { 2234 p.risk_damage += dmg(t, champ) }
// 2235         else if let Some(other) = game.get_entity_by_id(id) {
// 2236           if !other.is_champion() { 2237 p.risk_possible_tower += dmg(t, champ) }
// 2240         } else {
// 2241           let cnt = cache.iter_minions(team)                       // 내 팀(=적 타워의 적) 미니언
// 2243                     .filter(|m| t.attack_effect.as_ref().unwrap().is_in_range(t, m)).count();   // aux closure sy_0
// 2246           if cnt < 3 { 2247 p.risk_damage += dmg(t, champ) } else { 2249 p.risk_possible_tower += dmg(t, champ) }
//              }
// 2253       } else { p.risk_possible_tower += dmg(t, champ) }
//          }
//      }
//    }
// (⚠ 2118~2253 타워 블록 전체는 배치 C 의 2117 게이트 `game.tick() > %2230(MapDef/설정 시각)` 이면 통째로 건너뛰고 %2822(2265)로 진입 — m07.ll:43353~43354)
//
// 2265 for e in cache.others[1 - team].iter() {                          // +0xf0 + 32*(1-team)
// 2266   if e.distance_sq(me) > 22500000000 { continue }                 // 150000²
// 2269   let Some(atk) = e.attack_effect.as_ref() else { continue };
// 2270   let nearest: Option<usize> = match &e.ty { Ghoul(i) | Bear(i) => i.nearest_enemy(+0x88/+0x90), 2272 Eagle(i) => i.nearest_enemy(+0x70/+0x78), _ => None };
// 2277   if let Some(id) = nearest {
// 2278     if id == my_id { 2279 parameter.player.risk_damage(+0x998) += atk.expected_damage_target(ctx, e, me) }
// 2282     if let Some(target) = find(near_allies, id) {
// 2283       if let Some(target_entity) = game.get_entity_by_id(id) { 2284 target.risk_damage += atk.expected_damage_target(ctx, e, target_entity) }
//          }
// 2288     if let Some(target) = find(near_enemies, id) {
// 2289       if let Some(target_entity) = game.get_entity_by_id(id) { 2290 target.risk_damage += atk.expected_damage_target(ctx, e, target_entity) }
//          }
// 2293   } else if e.distance_sq(me) < 2500000001 {                       // ≤ 50000²  (2270 의 `_` 타입도 여기로)
// 2294     parameter.player.risk_damage += atk.expected_damage_target(ctx, e, me)
//      }
//    }
//
// 2300 let mut player_direct_minion_risk_damage: usize = 0;
// 2301 for m in near_minions.iter() {
// 2302   if let Minion(info) = &m.ty {                                    // +0x68 == 1
// 2304     if let Some(id) = info.nearest_enemy {                         // +0x88/+0x90
// 2305       if id == my_id {
// 2306         let damage = dmg(m, me);
// 2307         parameter.player.risk_damage += damage;
// 2308         if m.team != me.team { 2309 player_direct_minion_risk_damage += damage }
//            }
// 2313       if let Some(target) = find(near_allies, id) { 2314 if let Some(te) = game.get_entity_by_id(id) { 2315 target.risk_damage += dmg(m, te) } }
// 2319       if let Some(target) = find(near_enemies, id) { 2320 if let Some(te) = game.get_entity_by_id(id) { 2321 target.risk_damage += dmg(m, te) } }
// 2325     } else if m.distance_sq(me) < 2500000001 && m.team != me.team {   // IR 평가순: dist → team
// 2326       let damage = dmg(m, me);
// 2327       parameter.player.risk_damage += damage;
// 2328       player_direct_minion_risk_damage += damage;
//          }
//        }
//    }
//
// 2337 let tick = game.tick();
//      let minion_wave_damage = if ctx.is_line_phase(tick) {              // runner.rs:399 → setting.rs:703 인라인: match ctx.tutorial(+0x38) { 0 None|5 MidBottom|7 Line|8 Total => tick < setting.epic_jungle.first_spawn_tick(+0x8a8).saturating_sub(setting.tick_per_second(+0x12f8)*30), _ => true }
// 2338     enemy_minion_line_action_danger_damage_at(version=poison, data, target=me, me.x, me.y, window_tick=ctx.setting.tick_per_second /*2339*/, champion_action=false, predict_retarget=false)
//      } else {
// 2341     enemy_minion_wave_danger_damage_at(version, data, me, me.x, me.y, window_tick=tps)   // minion_wave_risk.rs:102 인라인:
//            let damage = enemy_minion_wave_risk_damage_at(poison, data, me, x, y, tps);         // 아웃오브라인 호출
//            if damage == 0 { 0 } else {                                                        // enemy_minion_wave_is_dangerous(65~75) 인라인
//              hp_pct = me.hp(+0x670)*100 / max(me.stat_cached.hp(+0x628), 1); damage_pct = damage*100 / max(me.hp, 1);
//              if damage >= me.hp || damage_pct > 49 || (hp_pct < 66 && damage_pct > 29) || (hp_pct < 41 && damage_pct > 17) || (hp_pct < 26 && damage_pct > 9) { damage } else { 0 } }
//      };
// 2343 parameter.player.risk_damage += minion_wave_damage.saturating_sub(player_direct_minion_risk_damage);
//
// 2346~2349 let near_jungles: bumpalo Vec<&Entity> = cache.jungles(+0xd0/+0xe8).iter().filter(|j| j.distance_sq(me) < 22500000000).copied().collect_in(pool);   // aux m01.ll:27827 + m07.ll:61552
// 2352 for j in near_jungles.iter() {
// 2354   match &j.ty {                                                     // +0x68
// 2356     Jungle(info) => if let Some(id) = info.focused {               // +0x88/+0x90
// 2357         if id == my_id { 2358 parameter.player.risk_damage += dmg(j, me) }
// 2361         if let Some(target) = find(near_allies, id)  { 2362 target.risk_damage += dmg(j, me) }    // ⚠target 인자가 target 엔티티가 아니라 **me(%59)** (m07.ll:45642)
// 2365         if let Some(target) = find(near_enemies, id) { 2366 target.risk_damage += dmg(j, me) }    // ⚠동일 (45668)
//            }
// 2371     Epic(info) => if let Some(id) = info.focused {
// 2372         let damage = dmg(j, me);
// 2373         if id == my_id { 2374 player.risk_damage += damage; 2375 player.risk_epic_damage(+0x9a0) += damage }
// 2378         if let Some(t) = find(near_allies, id)  { 2379 t.risk_damage += damage; 2380 t.risk_epic_damage(+0x88) += damage }
// 2383         if let Some(t) = find(near_enemies, id) { 2384 t.risk_damage += damage; 2385 t.risk_epic_damage += damage }
//            }
// 2392     Serpen(info) => if let Some(id) = info.focused {
// 2393         let damage = dmg(j, me);
// 2394         if id == my_id { 2395 player.risk_epic_damage += damage }                                   // (risk_damage 에는 안 더함)
// 2398         if let Some(t) = find(near_allies, id)  { 2399 t.risk_epic_damage += damage }
// 2402         if let Some(t) = find(near_enemies, id) { 2403 t.risk_epic_damage += damage }
//            }
//          _ => {}
//        }
//    }
// 2412 parameter.positioning_score.cx(+0x14a8) = map_cell_index(me.x) = min(me.x / 32000, 29);
// 2413 parameter.positioning_score.cy(+0x14b0) = map_cell_index(me.y);
// 2415 parameter.adjust();   // ScoreParameter::adjust(376~383) → ChampionScoreParameter::adjust(1444): player.risk_damage /= 2 (376); for a in near_allies { a.risk_damage /= 2 } (378~379); for e in near_enemies { e.risk_damage /= 2 } (382~383)
// 2417 return parameter;     // memcpy(sret %0 ← %47, 5384)
// 2418 } // drop: near_jungles(%16) · near_towers(%31) · near_enemies_with_action(%39) · near_allies_with_action(%41) · near_minions(%46) — 정상 경로 46077~46177 · 언와인드 정리 패드 %146/%158/%189/%241/%365/%3197/%3659(panic 시에만)
// ★TLS 접점 0 · rnd(gen_range) 사용 0 · NA(사장) 콜리 0(reach.txt: 블록 926 전부 live)
```

**`mem` 메모리 접근 212건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L1462 team 인덱스(<2 bounds check) · L1463 parameter.player.team · L1497 1-team \| (배치 C) self: %49(배치 A L1462) → iter_minions 팀 · L2145 player_by_champion_id 결과의 team (bounds<2 · panic anon.152) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | L1462 i32 zext → player_champion[team][pos] 인덱스 · L1463 parameter.player.pos | 4 | OK |  |
| 2 | OperationData | 0x0 | cache(&AbstractGameWithCache) | r | L1462 player_champion · L1497/1498 iter_minions · L1583/1586 iter_towers_without_nexus · L1514 game | 4 | OK |  |
| 3 | OperationData | 0x8 | context(&GameContext) | r | L1466 → context.pool(+0x0) = bumpalo Bump(%65) · expected_damage_target 의 ctx 인자(%64) | 4 | OK |  |
| 4 | OperationData | 0x10 | blackboard(&[Blackboard;2]) | r | L1506/L1513 클로저 env 로 전달(aux) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | L1462 [team][pos].unwrap()=champ · L1505 [team] 슬라이스(아군) · L1512 [1-team] 슬라이스(적) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game(&dyn AbstractGame: data,vtable) | r | L1514 클로저 env · L1593 vtable+0x210 iter_projectile · L1594/1599/1630/1707/1726 vtable+0x1f0 get_entity_by_id | 4 | OK |  |
| 7 | GameContext | 0x0 | pool(&Bump) | r | 모든 bumpalo Vec 의 할당자 | 4 | OK |  |
| 8 | Blackboard | 0x78 | small_actions[5] | r | aux s0_0/s3_0: [player.team] 은 아군, [1-player.team] 은 적 행동. 24B Option<SmallAction>(니치 -1=None) 그대로 튜플 원소 0 에 복사 | 4 | OK |  |
| 9 | Entity | 0x5c0 | id | r | L1500 champ.id · L1527/L1558 e.id · L1609/1640 ae.id · aux 필터 e.id!=champ.id \| (배치 B) gep 1472 — PossibleGain.from \| (배치 C) PossibleGain.from(e.id L1964/1977/1992 · champ.id %157 L2070/2083/2098) · L2132 nearest_enemy.1 == champ.id \| (배치 D) 2186/2233 nearest_enemy.id == champ.id · %157(내 챔프 id, 배치 A 로드) 와 2278·2305·2357·2373·2394 비교 | 4 | OK |  |
| 10 | Entity | 0x2c8 | cc.buf.ptr | r | L1524/1555/1693 cc 슬라이스 시작(원소 40B CCState) \| (배치 B) gep 712 — 줄 1888 CCState(40B) 슬라이스 \| (배치 C) Vec<CCState>(원소 40B) · L2005~2006 champ 의 CC 순회 | 4 | OK |  |
| 11 | Entity | 0x2d0 | cc.len | r | L1524/1555/1693 \| (배치 B) gep 720 \| (배치 C) L2005 | 4 | OK |  |
| 12 | CCState | 0x0 | @tag(i32) | r | block_input(): tag∉{2 Bind,3 BlockAttack,4 BlockSkill,5 BlockMoveSkill} 만 통과 · tag==10(Animation) 이면 tick 이 +0x20 | 4 | OK |  |
| 13 | CCState | 0x8 | tick(Airborne/Stun/ForceMove/Taunt/Fear/Charm) | r | map(\|c\| c.tick()) | 4 | OK |  |
| 14 | CCState | 0x20 | Animation.tick | r | select tag==10 ? +0x20 : +0x8 | 4 | OK |  |
| 15 | Entity | 0x68 | ty@tag | r | is_in_attack/is_in_skill/is_in_skill2 (entity.rs:1564/1571/1579): ==13(Champion) \| (배치 C) EntityType 태그 · 13=Champion(L2013/2024/2036/2049 is_in_*, L2077/2092 *_cooldown, L2135 is_champion) · 2=Tower(L2129) · L2067 attack_cooldown 의 14-way switch \| (배치 D) 2183/2230 ==2 Tower · 2189/2236 ==13 Champion(is_champion 인라인 entity.rs:1404) · 2270 switch 7 Ghoul/9 Bear/10 Eagle · 2302 ==1 Minion · 2354 switch 4 Jungle/5 Epic/6 Serpen(tcxdict --enum EntityType 메모리태그 = idx 그대로) | 3 | OK |  |
| 16 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | ==3 Attack(L1699/1708) · ==4 Skill(L1718/1727) · ==5 Skill2(L1737) \| (배치 B) gep 112 — 3 Attack·4 Skill·5 Skill2·6 Ult (tcxdict --enum ChampionActionState) \| (배치 C) ChampionActionState · 3 Attack(L2013) / 4 Skill(L2024) / 5 Skill2(L2036) / 6 Ult(L2049) | 3 | OK |  |
| 17 | Entity | 0x490 | attack_effect@Some.0 (Effect 56B) | r | L1697 attack_effect.as_ref().unwrap() \| (배치 D) `as_ref().unwrap()` 후 Effect::expected_damage_target 의 self(+1168). 23개 호출 사이트 전부 이 오프셋 | 4 | OK |  |
| 18 | Entity | 0x4c0 | attack_effect@tag(=casting, i32, -1=None) | r | L1697 None→unwrap_failed · L1699/1708 casting∈{1,2} | 4 | OK |  |
| 19 | Entity | 0x4c8 | skill_effect@Some.0 | r | L1716 \| (배치 B) gep 1224 | 4 | OK |  |
| 20 | Entity | 0x4f8 | skill_effect@tag(=casting) | r | L1716 · L1718/1727 | 4 | OK |  |
| 21 | Entity | 0x500 | skill2_effect@Some.0 | r | L1735 skill2_effect(): level>2 ? &skill2_effect : &None(@anon.31) \| (배치 B) gep 1280 · level>2 일 때만(Entity::skill2_effect entity.rs:1693 인라인) — Effect 내부 +0x10 range/+0x18 growth/+0x28 target/+0x30 casting \| (배치 C) Entity::skill2_effect()(entity.rs:1693) = level>2 ? &skill2_effect : &NONE(@anon…31 · 태그 -1) · L1987(e · 배치 B L1952 산출 %1494) · L2032 · L2094 | 4 | OK |  |
| 22 | Entity | 0x5c8 | level | r | entity.rs:1693 skill2_effect() 게이트 level>2 \| (배치 B) gep 1480 — skill2/ult 존재 게이트, growth_range*(level-1) \| (배치 C) growth_range×(level-1) · skill2_effect(): level>2 · ult_effect(): level>4 \| (배치 D) 2224 Effect::range: (tower.level - 1) * growth_range | 4 | OK |  |
| 23 | Effect | 0x30 | casting(CastingType) | r | skill2 경로는 선택된 Option<Effect> 의 +48 을 직접 읽음(-1=None). (casting-1)<u2 ⟺ casting∈{1 Position, 2 Direction} | 4 | OK |  |
| 24 | Entity | 0x660 | x | r | distance_sq · is_in_orbit \| (배치 B) gep 1632 — distance_sq(entity.rs:2158→utils.rs:7~9) \| (배치 C) distance_sq(entity.rs:2158) · is_near_tower_range 의 x \| (배치 D) distance_sq(entity.rs:2158→utils.rs:7) 인라인 다수 · 2412 cx · 2341/2338 콜리 인자 | 4 | OK |  |
| 25 | Entity | 0x668 | y | r | gep 1640 \| (배치 D) 위와 동일 | 4 | OK |  |
| 26 | Entity | 0x470 | stat_buff_cached.radius_mult(i32) | r | Entity::radius()(entity.rs:1511~1515): mult==0 ? radius : radius*(mult+100)/100 \| (배치 C) Entity::radius()(entity.rs:1511~1515): 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 | 4 | OK |  |
| 27 | Entity | 0x680 | radius | r | gep 1664 \| (배치 C) Entity::radius() 본값 \| (배치 D) 2224 Entity::radius ×2 | 4 | OK |  |
| 28 | Projectile | 0xf8 | caster_id | r | L1594 get_entity_by_id → caster (None 이면 continue) | 4 | OK |  |
| 29 | Projectile | 0x12c | applyed_target(CastingTarget) | r | check_projectile(self=&applyed_target, proj, entity) | 4 | OK |  |
| 30 | Projectile | 0x40 | move_type@tag(니치: 2..11, untagged BouncingTarget 은 +0x0 이 Option<usize> 태그 0/1) | r | is_targeting()(projectile.rs:134): idx=(t>1?t-2:7); idx 4 Target·5 TargetSplash → target_id=+0x50, idx 7(t==1, BouncingTarget Some) → target_id=+0x48 | 4 | OK |  |
| 31 | Projectile | 0x48 | move_type@BouncingTarget.target_id@Some | r |  | 4 | OK |  |
| 32 | Projectile | 0x50 | move_type@Target/TargetSplash.target_id | r |  | 4 | OK |  |
| 33 | Projectile | 0x78 | move_type@LinearDist.penetrate(bool) | r | is_linear_dist_no_penetrate()(projectile.rs:122) — L1673 에서 idx==0 && !penetrate | 4 | OK |  |
| 34 | Projectile | 0x100 | x | r | L1616/L1671 nearest 거리 | 4 | OK |  |
| 35 | Projectile | 0x108 | y | r |  | 4 | OK |  |
| 36 | ChampionScoreParameter | 0x58 | id | r | near_allies.iter().find(\|x\| x.id==target_id)(L1706/1725) · near_*[i].id → get_entity_by_id(L1599/1630) \| (배치 D) 2218(아군 루프 p.id → get_entity_by_id) · 2282/2288/2313/2319/2361/2365/2378/2383/2398/2402 `find(\|x\| x.id == id)` 인라인(iterator.rs:348~349) | 4 | OK |  |
| 37 | ChampionScoreParameter | 0x70 | applyed_damage | r | read-modify-write += (L1610/1641/1711/1730) | 4 | OK |  |
| 38 | ChampionScoreParameter | 0x78 | applyed_cc | r | += 1 (L1611/1642) | 4 | OK |  |
| 39 | ChampionScoreParameter | 0x80 | risk_damage | r | += (L1621/1647/1709/1728) | 4 | OK |  |
| 40 | ChampionScoreParameter | 0x90 | risk_cc | r | += 1 (L1622/1648) | 4 | OK |  |
| 41 | near_*_with_action 원소 {SmallAction 24B, usize pos, &Entity} 40B | 0x18 | pos | r | L1522/L1552 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 42 | near_*_with_action 원소 | 0x20 | &Entity | r | L1523/L1554/L1686 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 43 | near_*_with_action 원소 | 0x0 | action@tag / +0x8 payload(target_id) | r | L1687 tag==0 RunAway → continue · L1695 switch 6/7/8/9 · target_id=+0x8 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 44 | ScoreParameter(%47 지역 조립본) | 0x14b8 | near_allies.buf.ptr | r | 줄 1744·1765·1788 iter_mut 시작 (gep 5304) | 4 | OK |  |
| 45 | ScoreParameter(%47) | 0x14d0 | near_allies.len | r | gep 5328 — 원소 216B stride | 4 | OK |  |
| 46 | ScoreParameter(%47) | 0x14d8 | near_enemies.buf.ptr | r | 줄 1894·1905·1917·1930·1953 (gep 5336) | 4 | OK |  |
| 47 | ScoreParameter(%47) | 0x14f0 | near_enemies.len | r | gep 5360 | 4 | OK |  |
| 48 | ScoreParameter(%47) | 0x940 | player.risk_possible.buf.cap | r | push 시 len==cap 검사(줄 1845·1858·1873) | 4 | OK |  |
| 49 | ScoreParameter(%47) | 0x948 | player.risk_possible.len | r | push 시 읽고 +1 써넣음 | 4 | OK |  |
| 50 | ScoreParameter(%47) | 0x930 | player.risk_possible.buf.ptr | r | push 대상 힙 포인터(%77) | 4 | OK |  |
| 51 | ScoreParameter(%47) | 0x988 | player.applyed_damage | r | 줄 1761 += (read-modify-write) | 4 | OK |  |
| 52 | ScoreParameter(%47) | 0x998 | player.risk_damage | r | 줄 1759 += | 4 | OK |  |
| 53 | ChampionScoreParameter(near_allies/near_enemies 힙 원소 216B) | 0x58 | id | r | find(\|x\| x.id == target_id) 술어(closure#19/20/23~26 인라인) · 줄 1789/1954 `a.id` 로 get_entity_by_id | 4 | OK |  |
| 54 | ChampionScoreParameter(힙 원소) | 0x18 | risk_possible.buf.ptr | r | push 대상(줄 1798·1811·1826, gep 24) | 4 | OK |  |
| 55 | ChampionScoreParameter(힙 원소) | 0x28 | risk_possible.buf.cap | r | gep 40 | 4 | OK |  |
| 56 | ChampionScoreParameter(힙 원소) | 0x30 | risk_possible.len | r | gep 48 | 4 | OK |  |
| 57 | ChampionScoreParameter(힙 원소) | 0x70 | applyed_damage | r | += (줄 1749·1770·1899·1910·1922·1935) | 4 | OK |  |
| 58 | ChampionScoreParameter(힙 원소) | 0x80 | risk_damage | r | += (줄 1747·1768·1897·1908·1920·1933) | 4 | OK |  |
| 59 | near_allies_with_action Vec(%41 alloca 32B, bumpalo Vec<(SmallAction,i64,&Entity)>) | 0x0 | buf.ptr | r | 줄 1883 iter 시작 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 60 | near_allies_with_action Vec(%41) | 0x18 | len | r | gep 24 · 원소 40B = `{ {i64,[2 x i64]}, i64, ptr }` | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 61 | (SmallAction,i64,&Entity) 원소 40B | 0x0 | a@tag | r | SmallAction 판별자(tcxdict --enum SmallAction: 0 RunAway · 6 Attack · 7 Skill · 8 Skill2 · 9 Ult) | 3 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 62 | (SmallAction,i64,&Entity) 원소 | 0x8 | a.target_id | r | Attack/Skill/Skill2/Ult 페이로드 enum+0x8 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 63 | (SmallAction,i64,&Entity) 원소 | 0x20 | e: &Entity | r | gep 32. +0x18 의 i64 는 내 범위에서 안 읽음 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 64 | Entity(e = 적/아군, 1728B) | 0x640 | stat_cached.move_speed | r | 줄 1779/1944 `e_speed` (gep 1600) | 4 | OK |  |
| 65 | Entity | 0x490 | attack_effect@Some.0 (Effect 56B 시작) | r | gep 1168 — atk \| (배치 C) L1963(e) · L2010/2014/2016(champ) · L2065/2072(champ) · L2133/2142/2147/2157/2161(tower) · closure$33/34 is_in_range | 4 | OK |  |
| 66 | Entity | 0x4c0 | attack_effect@tag(=casting@tag, 니치) | r | gep 1216 · i32 -1 = None → unwrap_failed(줄 1781/1946) | 4 | OK |  |
| 67 | Entity | 0x4a0 | attack_effect.range | r | gep 1184 (Effect::range 인라인, effect.rs:26) \| (배치 C) Effect+0x10 · L2065 · L2123/2177(타워) | 4 | OK |  |
| 68 | Entity | 0x4a8 | attack_effect.growth_range | r | gep 1192 \| (배치 C) Effect+0x18 · ×(level-1) | 4 | OK |  |
| 69 | Entity | 0x4d8 | skill_effect.range | r | gep 1240 \| (배치 C) L1974/2081 | 4 | OK |  |
| 70 | Entity | 0x4e0 | skill_effect.growth_range | r | gep 1248 \| (배치 C) L1974/2081 | 4 | OK |  |
| 71 | Entity | 0x4f0 | skill_effect.target@tag (CastingTarget) | r | gep 1264 — CastingTarget::check 의 &self | 4 | OK |  |
| 72 | Entity | 0x4f8 | skill_effect@tag(casting, 니치 -1=None) | r | gep 1272 | 4 | OK |  |
| 73 | Entity | 0x538 | ult_effect@Some.0 | r | gep 1336 · level>4 일 때만(Entity::ult_effect entity.rs:1701 인라인) · +0x30 casting 태그로 None 판정 \| (배치 C) Entity::ult_effect()(entity.rs:1701) = level>4 ? &ult_effect : &NONE · L2045 | 4 | OK |  |
| 74 | Entity | 0x438 | stat_buff_cached.range | r | gep 1080 — Effect::range 의 시전자 사거리 보정 \| (배치 C) Effect::range(e)(effect.rs:26) 의 한 항 · L1974/1989/2065/2081/2096/2123/2177 \| (배치 D) 2224 is_near_tower_range→Effect::range(effect.rs:26) 인라인: 타워 사거리 가산 | 4 | OK |  |
| 75 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | gep 1136 — Entity::radius (entity.rs:1511~1515) 인라인 | 4 | OK |  |
| 76 | Entity | 0x68 | ty@tag (EntityType) | r | gep 104 — 13=Champion(is_in_attack/skill/skill2/ult) · attack_cooldown switch 키 | 4 | OK |  |
| 77 | Entity | 0xb0 | ty@Champion.0.attack_cooldown (SmallJiangshi 도 0xb0) | r | gep 176 — Entity::attack_cooldown(entity.rs:1748~1760) switch: Minion 0xb8(184) · Tower 0x110(272) · Jungle/Ghoul 0xe8(232) · Epic/Serpen 0x1f0(496) · Bear 0xc8(200) · Eagle 0xf0(240) · Revenant 0xd8(216) · Illusion 0xd0(208) · None/Nexus → 0 | 4 | OK |  |
| 78 | Entity | 0xb8 | ty@Champion.0.skill_cooldown | r | gep 184 (entity.rs:1776, 비챔피언 0) \| (배치 C) L2077 skill_cooldown()(entity.rs:1775~1776) — Minion 이면 같은 오프셋이 Minion.attack_cooldown(L2067 switch case 1) | 4 | OK |  |
| 79 | Entity | 0xc0 | ty@Champion.0.skill2_cooldown | r | gep 192 (entity.rs:1791, 비챔피언 0) \| (배치 C) L2092 skill2_cooldown()(entity.rs:1790~1791) | 4 | OK |  |
| 80 | CCState(40B 원소) | 0x0 | tag (i32) | r | block_input(entity.rs:521) = tag ∉ {2 Bind,3 BlockAttack,4 BlockSkill,5 BlockMoveSkill} (`add -6; icmp ult -4`) · tick(entity.rs:546) = tag==10 Animation ? +0x20 : +0x8 | 4 | OK |  |
| 81 | dyn AbstractGame vtable(%179 = cache.game vtable) | 0x1f0 | get_entity_by_id 슬롯 | r | gep 496 (divtable AbstractGame 0x1f0 → get_entity_by_id). 간접 호출이라 calls 에 심볼 없음 | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 82 | OperationData(%4) | 0x0 | cache | r | → &AbstractGameWithCache(%55). +0x0 game.data(%177) · +0x8 game.vtable(%179): vtable+0x28=tick(L2117 · m07.ll:43347) · vtable+0x1f0=get_entity_by_id(%372 · L1954/2012/2023/2035/2048/2061/2134/2171) | 4 | OK |  |
| 83 | OperationData(%4) | 0x8 | context | r | → &GameContext(%64). expected_damage_target 의 context 인자 | 4 | OK |  |
| 84 | OperationData(%4) | 0x10 | blackboard | r | → &[Blackboard;2](%167) · L2145 | 4 | OK |  |
| 85 | GameContext(%64) | 0x8 | setting | r | → &GameSetting(%1710) · L2111~2114 | 4 | OK |  |
| 86 | GameContext(%64) | 0x38 | tutorial | r | TutorialType(i8) · L2111 switch(m07.ll:43153~43169) | 4 | OK |  |
| 87 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | L2114 기본 arm(gep 5112 · m07.ll:43343 phi) · _docs game_core.txt 에 「타워 공격 불능 시간」 계열 | 4 | OK |  |
| 88 | GameSetting | 0x1400 | tower_attack_disable_tick_2v2 | r | tutorial First(1)/Bottom(3) arm(gep 5120) | 4 | OK |  |
| 89 | GameSetting | 0x1408 | tower_attack_disable_tick_3v3 | r | tutorial MidBottom(5) arm(L2113 · gep 5128) | 4 | OK |  |
| 90 | ScoreParameter(%47 지역) | 0x918 | player.action@tag | r | SmallAction 태그(8B) · L2004(!=RunAway 0) · L2008 switch(6 Attack/7 Skill/8 Skill2/9 Ult) · %75 | 4 | OK |  |
| 91 | ScoreParameter(%47 지역) | 0x920 | player.action.target_id | r | Attack/Skill/Skill2/Ult 페이로드 +0x8 · L2009/2020/2031/2044 · %76 | 4 | OK |  |
| 92 | ScoreParameter(%47 지역) | 0x14d8 | near_enemies.buf.ptr | r | %71 · L1953/2011/2022/2034/2047/2060/2170 순회 시작 | 4 | OK |  |
| 93 | ScoreParameter(%47 지역) | 0x14f0 | near_enemies.len | r | %74 · 원소 216B | 4 | OK |  |
| 94 | ChampionScoreParameter(near_enemies 원소) | 0x58 | id | r | a.id → get_entity_by_id(L1954/2061/2171) · find(\|x\| x.id == target_id)(L2011/2022/2034/2047) | 4 | OK |  |
| 95 | ChampionScoreParameter(near_enemies 원소) | 0x28 | risk_possible.buf.cap | r | push 의 cap==len 검사(L1428 인라인) | 4 | OK |  |
| 96 | ChampionScoreParameter(near_enemies 원소) | 0x30 | risk_possible.len | r | push 후 +1 | 4 | OK |  |
| 97 | SmallAction<&Entity>(near_allies_with_action 원소 40B · 배치 B L1883) | 0x20 | entity(e) | r | %1201 = 아군 엔티티 · 배치 B 에서 로드, 배치 C 가 계속 사용 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 98 | Entity | 0x0 | team@tag | r | TeamType 태그(Player 0/Neutral 1) · L2119/2173 Entity::eq(entity.rs:1127) \| (배치 D) 2173(C)·2220·2308·2325 TeamType::eq(entity.rs:1127) 인라인 — 0=Player(payload +0x8 비교) / 1=Neutral. 2220: 타워.team == 아군챔프.team → continue · 2308/2325: 미니언.team != 내.team | 4 | OK |  |
| 99 | Entity | 0x8 | team@Player.0 | r | 팀 번호 · L2119/2173 (태그가 둘 다 Player 일 때만 비교) \| (배치 D) TeamType::eq 의 Player 페이로드(팀 번호) 비교(m07.ll:44362·45130·45279) | 4 | OK |  |
| 100 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag | r | Option<(usize,usize)> · trunc→i1 1=Some · L2131 | 4 | OK |  |
| 101 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 | r | 타워가 노리는 적의 id · L2131 `Some((_, id))` \| (배치 D) 타워는 Option<(usize,usize)> — **.1 을 id 로 읽음**(DI `id`). 2185·2232(m07.ll:44086·44486). .0(+0x90)은 내 범위에서 안 읽음 | 4 | OK |  |
| 102 | Entity | 0xb0 | ty@Champion.0.attack_cooldown | r | L2067 attack_cooldown()(entity.rs:1748~1760 인라인 · 실제 champ 는 Champion 이라 이 필드) | 4 | OK |  |
| 103 | Entity | 0x110 | ty@Tower.info.attack_cooldown | r | L2067 attack_cooldown() switch case 2 (champ 가 Tower 일 리 없어 실질 사장 arm · 다른 case: Jungle/Ghoul 0xe8 · Epic/Serpen 0x1f0 · SmallJiangshi 0xb0 · Bear 0xc8 · Eagle 0xf0 · Revenant 0xd8 · Illusion 0xd0 · None/Nexus → 0) | 4 | OK |  |
| 104 | CCState(40B 원소) | 0x0 | @tag(i32) | r | L2006 필터: (tag-6) <u -4 ⟺ tag ∈ {0 Airborne,1 Stun,6 ForceMove,7 Taunt,8 Fear,9 Charm,10 Animation} (Bind/BlockAttack/BlockSkill/BlockMoveSkill 제외) | 4 | OK |  |
| 105 | CCState(40B 원소) | 0x8 | tick | r | L2006 map: tag≠10 이면 +0x8 | 4 | OK |  |
| 106 | CCState(40B 원소) | 0x20 | Animation.tick | r | L2006 map: tag==10 이면 +0x20 (select 32/8 · m07.ll:42238) | 4 | OK |  |
| 107 | Entity | 0x4c0 | attack_effect.casting@tag = attack_effect@tag(니치) | r | -1(0xFFFFFFFF)=None · L2010/2064 unwrap · L1449(is_near_tower_range) · L2137/2142/2147/2152/2157/2161 · L2013 is_nontarget: casting ∈ {1 Position, 2 Direction} | 4 | OK |  |
| 108 | Entity | 0x4c8 | skill_effect@Some.0 (Effect 시작) | r | L1972(e 스킬) · L2021/2025/2027 · L2079/2085(champ) | 4 | OK |  |
| 109 | Entity | 0x4f0 | skill_effect.target(CastingTarget 4B) | r | L1973/2080 CastingTarget::check(&target, caster, target_entity) | 4 | OK |  |
| 110 | Entity | 0x4f8 | skill_effect.casting@tag = skill_effect@tag(니치) | r | -1=None → L1972 스킵 / L2021 unwrap / L2079 스킵 · L2024 is_nontarget | 4 | OK |  |
| 111 | Entity | 0x510 | skill2_effect.range | r | L1989/2096 · (C3 경고 사유) 본문엔 skill2 포인터(0x500 선택 결과 %1494/%1782/%2163) 기준 2단계 gep +16 으로 나타나 절대 오프셋 1296 은 없음 | 4 | OK |  |
| 112 | Entity | 0x518 | skill2_effect.growth_range | r | L1989/2096 · (C3 경고 사유) 2단계 gep +24 | 4 | OK |  |
| 113 | Entity | 0x528 | skill2_effect.target | r | L1988/2095 check · (C3 경고 사유) 2단계 gep +40 | 4 | OK |  |
| 114 | Entity | 0x530 | skill2_effect@tag(니치 · casting) | r | -1=None · L1987/2032/2094 · L2036 is_nontarget · (C3 경고 사유) 2단계 gep +48 | 4 | OK |  |
| 115 | Entity | 0x568 | ult_effect@tag(니치 · casting) | r | L2045 unwrap · L2049 is_nontarget · (C3 경고 사유) ult 포인터(0x538 · %1792) 기준 2단계 gep +48 | 4 | OK |  |
| 116 | Entity | 0x640 | stat_cached.move_speed | r | L2063 speed · ×20(L2069/2082/2097 · 배치 B %1504 e) · ×30(L2123/2177 타워 판정 d) \| (배치 D) 2224 `d = champ.move_speed * 30`(m07.ll:44381~44382) | 4 | OK |  |
| 117 | PlayerState | 0x9c0 | info.position@tag(i32) | r | L2145 as_index(entity.rs:581) → small_actions 인덱스 | 4 | OK |  |
| 118 | Blackboard(744B · [2] 배열) | 0x78 | small_actions[pos]@tag | r | Option<SmallAction> 24B 원소 · 태그 0 = Some(RunAway) · L2146 (Option::eq 인라인 · option.rs:2439) | 4 | OK |  |
| 119 | bumpalo Vec<&Entity>(near_towers %31 지역) | 0x0 | buf.ptr | r | L2118/2172 순회 · 원소 8B(&Entity) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 120 | bumpalo Vec<&Entity>(near_towers %31 지역) | 0x18 | len | r | L2118/2172 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 121 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag / Minion·Ghoul·Bear.info.nearest_enemy@tag / Jungle·Epic·Serpen.info.focused@tag | r | `trunc nuw i64 → i1`: 1=Some. 2185·2232(Tower) 2270(Ghoul/Bear) 2304(Minion) 2356/2371/2392(Jungle/Epic/Serpen) | 4 | OK |  |
| 122 | Entity | 0x90 | ty@Minion·Ghoul·Bear.info.nearest_enemy@Some.0 / Jungle·Epic·Serpen.info.focused@Some.0 | r | 타깃 엔티티 id(usize). 2270(+136/+144 phi) 2304 2356 2371 2392 | 4 | OK |  |
| 123 | Entity | 0x70 | ty@Eagle.info.nearest_enemy@tag | r | 2272 Eagle 분기(phi 112/120 → +0x70/+0x78) | 4 | OK |  |
| 124 | Entity | 0x78 | ty@Eagle.info.nearest_enemy@Some.0 | r | 2272 | 4 | OK |  |
| 125 | Entity | 0x470 | stat_buff_cached.radius_mult | r | 2224 Entity::radius(entity.rs:1511~1515) 인라인 ×2(챔프 r · 타워 radius). i32, 0 이면 radius 그대로 | 4 | OK |  |
| 126 | Entity | 0x4a0 | attack_effect@Some.0.range | r | 2224 Effect::range | 4 | OK |  |
| 127 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | 2224 Effect::range: (level-1)*growth_range | 4 | OK |  |
| 128 | Entity | 0x4c0 | attack_effect@tag (Niche · casting@tag i32) | r | `icmp eq i32 -1` = None → option::unwrap_failed(panic) 또는 2269/2224 에서 continue/false. tcxdict: attack_effect@tag = 니치 4B, -1 이 None | 3 | OK |  |
| 129 | Entity | 0x628 | stat_cached.hp(최대 HP) | r | 2341 enemy_minion_wave_is_dangerous 인라인: hp_pct = hp*100/max(max_hp,1) | 4 | OK |  |
| 130 | Entity | 0x670 | hp | r | 2341 wave_is_dangerous: damage>=hp / damage_pct = damage*100/max(hp,1) | 4 | OK |  |
| 131 | ScoreParameter(local %47) | 0x14b8 | near_allies.buf.ptr | r | 2217 이터·find 다수·2415 (len +0x14d0) | 4 | OK |  |
| 132 | ScoreParameter(local %47) | 0x14d0 | near_allies.len | r |  | 4 | OK |  |
| 133 | ScoreParameter(local %47) | 0x14d8 | near_enemies.buf.ptr | r | find 다수·2415 (len +0x14f0) | 4 | OK |  |
| 134 | ScoreParameter(local %47) | 0x14f0 | near_enemies.len | r |  | 4 | OK |  |
| 135 | AbstractGameWithCache | 0xd0 | jungles.buf.ptr (len +0xe8) | r | 2347 near_jungles 원천 | 4 | OK |  |
| 136 | AbstractGameWithCache | 0xf0 | others[1-team].buf.ptr (원소 32B · len = +24) | r | 2265 `for e in cache.others[1 - team]`(m07.ll:44656~44663 · gep stride 32 × %144) | 4 | OK |  |
| 137 | GameContext | 0x8 | setting(&GameSetting) | r | 2337 is_line_phase(runner.rs:399) 인라인 | 4 | OK |  |
| 138 | GameContext | 0x38 | tutorial(TutorialType u8) | r | %1708(배치 C 2111 에서 로드) — 2337 switch 0/5/7/8 | 4 | OK |  |
| 139 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | 2337 setting.rs:703: tick < first_spawn_tick.saturating_sub(tps*30) | 4 | OK |  |
| 140 | GameSetting | 0x12f8 | tick_per_second | r | 2337 tps*30 · 2338/2339/2341 window_tick = tps | 4 | OK |  |
| 141 | AbstractGame vtable | 0x1f0 | get_entity_by_id(&self, id) -> Option<&Entity> | r | %372(배치 A) 슬롯 — 2188·2218·2235·2283·2289·2314·2320(divtable AbstractGame 0x1f0) | 3 | 확인불가(vtable 슬롯) |  |
| 142 | AbstractGame vtable | 0x28 | tick(&self) -> usize | r | %2232(배치 C 2117 로드) 재사용 — 2337(divtable AbstractGame 0x28) | 3 | 확인불가(vtable 슬롯) |  |
| 143 | ScoreParameter(local %47 → sret memcpy L2417) | 0x0 | wave_snapshot@tag | w | L1463 m07.ll:38219. 함수 전체에서 유일한 store — 이 함수는 wave_snapshot 을 채우지 않는다 | 4 | OK | 0 (None) |
| 144 | ScoreParameter | 0x918 | player.action@tag | w | L1463 (+2328). 페이로드 +0x920..0x92f 미기록 | 4 | OK | 0 (RunAway) |
| 145 | ScoreParameter | 0x930 | player.risk_possible {ptr,a,cap,len} | w | L1463 (+2352/+2360, memset 16 @+2368) | 4 | OK | {8, context.pool, 0, 0} |
| 146 | ScoreParameter | 0x950 | player.gain_possible {ptr,a,cap,len} | w | L1463 (+2384/+2392, memset 24 @+2400 = cap,len,id) | 4 | OK | {8, context.pool, 0, 0} |
| 147 | ScoreParameter | 0x970 | player.id | w | L1463 memset 후 L1500 store (+2416) | 4 | OK | 0 → champ.id |
| 148 | ScoreParameter | 0x978 | player.team | w | L1463 (+2424) | 4 | OK | player.info.team |
| 149 | ScoreParameter | 0x980 | player.pos | w | L1463 (+2432) | 4 | OK | player.info.position as usize |
| 150 | ScoreParameter | 0x988 | player.applyed_damage | w | memset(+2440,154) 로 0 초기화 후 누적. 조건은 logic 참조 | 4 | OK | 0; += Effect::expected_damage_target(atk/skill/skill2, ctx, e, champ) [L1702/L1721/L1740]; += Projectile::expected_damage_target(proj, ctx, caster, champ) [L1665] |
| 151 | ScoreParameter | 0x990 | player.applyed_cc | w | +2448 | 4 | OK | 0; += 1 [L1667: 나를 겨냥한 투사체 && has_cc] |
| 152 | ScoreParameter | 0x998 | player.risk_damage | w | +2456 | 4 | OK | 0; += Effect::expected_damage_target [L1700/L1719/L1738]; += Projectile::expected_damage_target [L1678] |
| 153 | ScoreParameter | 0x9a0 | player.risk_epic_damage | w | 배치 A 에서는 0 초기화만(+2464). 갱신은 배치 D(L2374/L2395) | 4 | OK | 0 |
| 154 | ScoreParameter | 0x9a8 | player.risk_cc | w | +2472 | 4 | OK | 0; += 1 [L1680: 비겨냥 투사체 궤도 안 && has_cc] |
| 155 | ScoreParameter | 0x9b0 | player.risk_possible_tower | w | +2480. 갱신은 배치 C | 4 | OK | 0 |
| 156 | ScoreParameter | 0x9b8 | player.action_time | w | +2488 — 개별 gep 없음(memset(+2440,154) 범위 안이라 C3 경고는 정상). 배치 A 에서 이후 store 없음 | 4 | OK | 0 |
| 157 | ScoreParameter | 0x9c0 | player.attack_value..buff_inv_cd_count (6×i64) | w | attack_value(+0x9c0)/util_value(+0x9c8) 는 배치 A 에서 0 유지 | 4 | OK | 0 → precompute_champion_powers(version,data,&mut player)(L1501) 가 +0x9d0..0x9e8(attack_power·util_power_base·cc_time_x_inv_cd·buff_inv_cd_count) 를 채움 |
| 158 | ScoreParameter | 0x9f0 | positioning_score.value[7][7] (56B×49) | w | L1463: 첫 원소(+2544)는 memset(+2440,154) 꼬리 50B 라 개별 gep 없음(C3 경고 정상), 나머지 48개는 memset 50 ×48(+2600..+5232 stride 56). 원소 +0x32..0x37 6B 패딩 미기록 | 4 | OK | 각 원소 앞 50B = 0 (risk,tower_risk,gain,gain_me,adjust,unseen_champ_threat,on_trajectory,on_periodic_trajectory) |
| 159 | ScoreParameter | 0x14a8 | positioning_score.cx / cy | w | memset(+5288,16). 배치 D L2412~2413 이 덮음 | 4 | OK | 0 |
| 160 | ScoreParameter | 0x14b8 | near_allies {ptr,a,cap,len} | w | +5304/+5312, memset16 @+5320. push = reserve_internal_or_panic(필요 시) + memcpy 216 + len+=1 | 4 | OK | {8, context.pool, 0, 0} → push(ChampionScoreParameter 216B) ×N [L1548] |
| 161 | ScoreParameter | 0x14d8 | near_enemies {ptr,a,cap,len} | w | +5336/+5344, memset16 @+5352 | 4 | OK | {8, context.pool, 0, 0} → push ×N [L1579] |
| 162 | ScoreParameter | 0x14f8 | version | w | L1463 (+5368) | 4 | OK | %1 |
| 163 | ScoreParameter | 0x1500 | v3_turnback_hold | w | L1463 (+5376). 함수 전체에서 유일한 store | 4 | OK | false |
| 164 | ChampionScoreParameter(near_allies/near_enemies 원소, 스택 %37/%34 조립 후 216B memcpy) | 0x0 | action | w | L1541/L1572 (memcpy %36←%214 / %33←%295) | 4 | OK | near_*_with_action 원소의 SmallAction 24B memcpy |
| 165 | ChampionScoreParameter 원소 | 0x18 | risk_possible / gain_possible(+0x38) | w | L1526/L1557 | 4 | 확인불가(tcx 사전에 타입 없음) | 빈 bumpalo Vec {8, pool, 0, 0} |
| 166 | ChampionScoreParameter 원소 | 0x58 | id / team(+0x60) / pos(+0x68) | w | L1527·1526 / L1558·1557 | 4 | 확인불가(tcx 사전에 타입 없음) | e.id / 아군=player.team·적=1-team / p.1(pos) |
| 167 | ChampionScoreParameter 원소 | 0x70 | applyed_damage..risk_cc (5×usize) + risk_possible_tower(+0x98) | w | memset(+112,40) + store 0 @+152 | 4 | 확인불가(tcx 사전에 타입 없음) | 0 |
| 168 | ChampionScoreParameter 원소 | 0xa0 | action_time | w | L1523~1526 / L1554~1557 | 4 | 확인불가(tcx 사전에 타입 없음) | act_tick = max(e.remain_action_time(), e.cc.iter().filter(block_input).map(tick).max().unwrap_or(0)) |
| 169 | ChampionScoreParameter 원소 | 0xa8 | attack_value..buff_inv_cd_count (6×i64) | w | memset(+168,48) | 4 | 확인불가(tcx 사전에 타입 없음) | 0 → precompute_champion_powers(version,data,&mut p)(L1547/L1578) 가 +0xb8..0xd0 채움 |
| 170 | ChampionScoreParameter 원소(near_allies, 힙 원소 직접) | 0x70 | applyed_damage | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | += Projectile::expected_damage_target(proj,ctx,caster,ae) [L1610: proj 가 ae 를 겨냥] · += Effect::expected_damage_target(atk/skill,ctx,e,target_entity) [L1711/L1730: 적 e 의 평타/스킬 대상이 아군이고 casting∉{1,2} && e.is_in_attack()/is_in_skill()] |
| 171 | ChampionScoreParameter 원소(near_allies 힙) | 0x80 | risk_damage | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | += Projectile::expected_damage_target [L1621: 비겨냥 투사체 궤도 안] · += Effect::expected_damage_target [L1709/L1728] |
| 172 | ChampionScoreParameter 원소(near_allies 힙) | 0x78 | applyed_cc (+1 L1611) / risk_cc(+0x90, +1 L1622) | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | += 1 when proj.has_cc() |
| 173 | ChampionScoreParameter 원소(near_enemies 힙) | 0x70 | applyed_damage(+0x70, L1641) / risk_damage(+0x80, L1647) / applyed_cc(+0x78, L1642) / risk_cc(+0x90, L1648) | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | 투사체 대조 누적(적 쪽엔 nearest/penetrate 예외 없음) |
| 174 | 로컬(sret 아님) | 0x0 | near_minions(%46) · near_allies_with_action(%41) · near_enemies_with_action(%39) · near_towers(%31) · nearest_other_distance(Option<usize>) | w | sret 에 들어가지 않는다 — 뒤 배치가 읽는 입력 | 4 | 확인불가(tcx 사전에 타입 없음) | bumpalo Vec 4개(함수 끝 배치 D 에서 drop) + 투사체별 지역 변수 |
| 175 | ScoreParameter(%47) | 0x998 | player.risk_damage | w | 줄 1759 — 적 e 가 Ult(9) 로 나(target_id==my id)를 겨냥, `ult.is_nontarget() \|\| !e.is_in_ult()` 참일 때 | 4 | OK | += ult.expected_damage_target(ctx, e, me) |
| 176 | ScoreParameter(%47) | 0x988 | player.applyed_damage | w | 줄 1761 — 위 조건 거짓(비논타겟 궁을 실제 시전 중) | 4 | OK | += ult.expected_damage_target(ctx, e, me) |
| 177 | ScoreParameter(%47) | 0x948 | player.risk_possible.len | w | 줄 1845(평타)·1858(스킬)·1873(스킬2) — 적 e 가 나에게 닿을 수 있을 때. 힙 원소 [len] = PossibleGain{from: e.id(0x5c0), tick: attack/skill/skill2_tick, value: expected_damage_target(ctx,e,me)}; len==cap 이면 reserve_internal_or_panic(+1) 후 buf.ptr(0x930)·cap(0x940) 갱신 | 4 | OK | +1 (push) |
| 178 | ChampionScoreParameter(near_allies 힙 원소, ScoreParameter+0x14b8 buf 안) | 0x80 | risk_damage | w | 줄 1747(적 Skill2 → 아군 target) · 1768(적 Ult → 아군 target). 조건 `eff.is_nontarget() \|\| !e.is_in_skill2()/is_in_ult()` | 4 | OK | += skill2/ult.expected_damage_target(ctx, e, target_entity) |
| 179 | ChampionScoreParameter(near_allies 힙 원소) | 0x70 | applyed_damage | w | 줄 1749·1770 (위 조건 거짓) | 4 | OK | += 같은 expected_damage_target |
| 180 | ChampionScoreParameter(near_allies 힙 원소) | 0x30 | risk_possible.len (+0x18 Vec push) | w | 줄 1798·1811·1826 — 적 e 가 아군 ae 에 닿을 수 있을 때. len==cap 이면 reserve_internal_or_panic 후 +0x18 ptr/+0x28 cap 갱신 | 4 | OK | +1, 원소 = PossibleGain{from: e.id, tick: e_attack/e_skill/e_skill2_tick, value: atk/skill/skill2.expected_damage_target(ctx, e, ae)} |
| 181 | ChampionScoreParameter(near_enemies 힙 원소, ScoreParameter+0x14d8 buf 안) | 0x80 | risk_damage | w | 줄 1897·1908·1920·1933 — 아군 e 의 Attack/Skill/Skill2/Ult 대상이 near_enemies 에 있을 때, `eff.is_nontarget() \|\| !e.is_in_attack()/skill/skill2/ult` 참 | 4 | OK | += atk/skill/skill2/ult.expected_damage_target(ctx, e(아군), target_entity) |
| 182 | ChampionScoreParameter(near_enemies 힙 원소) | 0x70 | applyed_damage | w | 줄 1899·1910·1922·1935 (위 조건 거짓 = 아군이 실제 시전 중) | 4 | OK | += 같은 expected_damage_target |
| 183 | ChampionScoreParameter(near_enemies 원소 a · %1522) | 0x18 | risk_possible.push(PossibleGain{from: e.id, tick: e_skill_tick, value}) | w | L1976~1978 · store 3×8B m07.ll:42014~42018 · len(+0x30)+1 m07.ll:42021 · cap==len 이면 reserve_internal_or_panic(+1) m07.ll:41998 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | from=e.id(0x5c0 load m07.ll:41966) · tick=%1490(배치 B L1949 e_skill_tick) · value=e.skill_effect.expected_damage_target(ctx, e as &dyn AbstractEntity, ae)(m07.ll:41967) |
| 184 | ChampionScoreParameter(near_enemies 원소 a · %1522) | 0x18 | risk_possible.push(PossibleGain{from: e.id, tick: e_attack_tick, value}) | w | L1963~1965 · store m07.ll:41896~41900 · len+1 m07.ll:41903 · 진입 조건은 배치 B L1962 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | from=e.id(m07.ll:41848) · tick=%1489(배치 B L1948 e_attack_tick) · value=e.attack_effect.expected_damage_target(ctx, e, ae)(m07.ll:41849) |
| 185 | ChampionScoreParameter(near_enemies 원소 a · %1522) | 0x18 | risk_possible.push(PossibleGain{from: e.id, tick: e_skill2_tick, value}) | w | L1991~1993 · store m07.ll:42126~42130 · len+1 m07.ll:42133 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | from=e.id(m07.ll:42078) · tick=%1491(배치 B L1950) · value=e.skill2_effect().expected_damage_target(ctx, e, ae)(m07.ll:42079) |
| 186 | ChampionScoreParameter(near_enemies find 결과 target · %1801) | 0x80 | risk_damage += champ.attack_effect.expected_damage_target(ctx, champ, target_entity) | w | L2014 (Attack arm · 조건: atk.casting.is_nontarget() \|\| !champ.is_in_attack()) | 4 | OK | m07.ll:42454~42457 |
| 187 | ChampionScoreParameter(near_enemies find 결과 target · %1801) | 0x70 | applyed_damage += champ.attack_effect.expected_damage_target(ctx, champ, target_entity) | w | L2016 (Attack arm else · 대상지정 평타를 이미 Attack 상태로 시전 중) | 4 | OK | m07.ll:42447~42450 |
| 188 | ChampionScoreParameter(near_enemies find 결과 target · %1844) | 0x80 | risk_damage += champ.skill_effect.expected_damage_target(...) | w | L2025 (Skill arm) | 4 | OK | m07.ll:42560~42563 |
| 189 | ChampionScoreParameter(near_enemies find 결과 target · %1844) | 0x70 | applyed_damage += champ.skill_effect.expected_damage_target(...) | w | L2027 (Skill arm else · is_in_skill) | 4 | OK | m07.ll:42553~42556 |
| 190 | ChampionScoreParameter(near_enemies find 결과 target · %1887) | 0x80 | risk_damage += champ.skill2_effect().expected_damage_target(...) | w | L2037 (Skill2 arm) | 4 | OK | m07.ll:42666~42669 |
| 191 | ChampionScoreParameter(near_enemies find 결과 target · %1887) | 0x70 | applyed_damage += champ.skill2_effect().expected_damage_target(...) | w | L2039 (Skill2 arm else · is_in_skill2) | 4 | OK | m07.ll:42659~42662 |
| 192 | ChampionScoreParameter(near_enemies find 결과 target · %1930) | 0x80 | risk_damage += champ.ult_effect().expected_damage_target(...) | w | L2050 (Ult arm) | 4 | OK | m07.ll:42772~42775 |
| 193 | ChampionScoreParameter(near_enemies find 결과 target · %1930) | 0x70 | applyed_damage += champ.ult_effect().expected_damage_target(...) | w | L2052 (Ult arm else · is_in_ult) | 4 | OK | m07.ll:42765~42768 |
| 194 | ChampionScoreParameter(near_enemies 원소 a · %1969) | 0x18 | risk_possible.push(PossibleGain{from: champ.id, tick: attack_tick, value}) | w | L2070~2072 · store m07.ll:43055~43059 · len+1 m07.ll:43062 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | from=%157(champ.id · 배치 A L1500) · tick=%2060(L2067 attack_tick) · value=champ.attack_effect.expected_damage_target(ctx, champ, ae)(m07.ll:43008) |
| 195 | ChampionScoreParameter(near_enemies 원소 a · %1969) | 0x18 | risk_possible.push(PossibleGain{from: champ.id, tick: skill_tick, value}) | w | L2083~2085 · store m07.ll:43189~43193 · len+1 m07.ll:43196 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | tick=%2098(L2077) · value=champ.skill_effect.expected_damage_target(ctx, champ, ae)(m07.ll:43142) |
| 196 | ChampionScoreParameter(near_enemies 원소 a · %1969) | 0x18 | risk_possible.push(PossibleGain{from: champ.id, tick: skill2_tick, value}) | w | L2098~2100 · store m07.ll:43326~43330 · len+1 m07.ll:43333 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | tick=%2161(L2092) · value=champ.skill2_effect().expected_damage_target(ctx, champ, ae)(m07.ll:43279) |
| 197 | ScoreParameter(%47 지역) | 0x998 | player.risk_damage += tower.attack_effect.expected_damage_target(ctx, tower, champ) | w | L2133 — 사거리 안 적 타워의 nearest_enemy.1 == champ.id | 4 | OK | m07.ll:43815~43817 (%89) |
| 198 | ScoreParameter(%47 지역) | 0x9b0 | player.risk_possible_tower += tower.attack_effect.expected_damage_target(ctx, tower, champ) | w | L2142(타겟이 비챔피언 & 우리 미니언 <2) · L2147(타겟 챔피언의 블랙보드 액션이 RunAway) · L2157(타겟 id 엔티티 없음 & 우리 미니언 <2) · L2161(nearest_enemy None) | 4 | OK | m07.ll:43662~43664(L2147) · 43732~43734(L2142) · 43802~43804(L2157) · 43828~43830(L2161) (%92) |
| 199 | ChampionScoreParameter(near_enemies[p]) | 0x80 | risk_damage | w | 2187(타워 타깃==champ · m07.ll:44239) · 2200(타깃 소실 & 사거리 내 적미니언 <3 · 44226) | 4 | OK | += tower.attack_effect.expected_damage_target(ctx, tower, champ) |
| 200 | ChampionScoreParameter(near_enemies[p]) | 0x98 | risk_possible_tower | w | 2190(타깃이 비챔피언 · 44135) · 2202(타깃 소실 & 미니언 ≥3 · 44205) · 2206(nearest_enemy None · 44252) | 4 | OK | += tower.attack_effect.expected_damage_target(ctx, tower, champ) |
| 201 | ChampionScoreParameter(near_allies[p]) | 0x80 | risk_damage | w | 2234(44639) · 2247(44626) | 4 | OK | += tower.attack_effect.expected_damage_target(ctx, tower, champ) |
| 202 | ChampionScoreParameter(near_allies[p]) | 0x98 | risk_possible_tower | w | 2237(44535) · 2249(44605) · 2253(44652) | 4 | OK | += tower.attack_effect.expected_damage_target(ctx, tower, champ) |
| 203 | ScoreParameter(local %47) | 0x998 | player.risk_damage | w | 2279(44854) 2294(44949) 2307(45099) 2327(45298) 2343(45411) 2358(45587) 2374(45753) · 2415 adjust(45982, lshr 1) | 4 | OK | += 소환수/미니언/정글몹의 expected_damage_target(ctx, e, me) · 2343 += wave.saturating_sub(direct) · 2415 >>= 1 |
| 204 | ScoreParameter(local %47) | 0x9a0 | player.risk_epic_damage | w | 45756 · 45894 | 4 | OK | += damage(Epic 2375 · Serpen 2395) |
| 205 | ChampionScoreParameter(near_allies.find(id)) | 0x80 | risk_damage | w | 44917 · 45208 · 45654 · 45764 · 2415 루프 lshr 1(46018) | 4 | OK | += expected_damage_target(ctx, e, target_entity)(2284·2315) / (ctx, j, **me**)(2362·2379) |
| 206 | ChampionScoreParameter(near_allies.find(id)) | 0x88 | risk_epic_damage | w | 2380(45768 Epic) · 2399(45902 Serpen) | 4 | OK | += damage |
| 207 | ChampionScoreParameter(near_enemies.find(id)) | 0x80 | risk_damage | w | 44939 · 45244 · 45680 · 45817 · 2415 루프 lshr 1(46064) | 4 | OK | += expected_damage_target(ctx, e, target_entity)(2290·2321) / (ctx, j, **me**)(2366·2384) |
| 208 | ChampionScoreParameter(near_enemies.find(id)) | 0x88 | risk_epic_damage | w | 2385(45821) · 2403(45951) | 4 | OK | += damage |
| 209 | ScoreParameter(local %47) | 0x14a8 | positioning_score.cx | w | 2412 map_cell_index(score_parameter.rs:14) 인라인 · m07.ll:45961 | 4 | OK | min(me.x / 32000, 29) |
| 210 | ScoreParameter(local %47) | 0x14b0 | positioning_score.cy | w | 2413 · 45968 | 4 | OK | min(me.y / 32000, 29) |
| 211 | ScoreParameter(sret %0) | 0x0 | (전체 5384B) | w | 2417 m07.ll:46075 — 유일한 sret 기록 | 4 | OK | memcpy(%0 ← %47, 5384) |

**`consts` 상수 88건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 1462 | 임계 | player_champion[team] 의 team 상한(bounds check <2) — 팀 2개. 같은 값이 entity.rs:1693 skill2_effect() 의 `level > 2` 게이트(L1735)와 CastingType 검사 `(casting-1) <u 2`(L1699 등)에도 쓰임 | 4 |  |
| 1 | 5 | 1506 | 태그 | small_actions[i]/player_champion[team][i] 의 포지션 상한(bounds check <5) — aux s0_0/s3_0. 본문에선 ChampionActionState::Skill2 태그(L1737 is_in_skill2)로도 등장 | 4 |  |
| 2 | 0 | 1687 | 태그 | SmallAction::RunAway 메모리태그 0 — 적 행동이 RunAway 면 continue. (L1463 의 0 초기화들·Option None 태그도 0) | 4 |  |
| 3 | 6 | 1695 | 태그 | SmallAction::Attack 태그(tcxdict --enum SmallAction: idx=태그, Direct) → L1697 평타 분기 | 3 |  |
| 4 | 7 | 1695 | 태그 | SmallAction::Skill 태그 → L1716 스킬 분기. (Projectile is_targeting 의 idx 7 = BouncingTarget 과는 별개) | 4 |  |
| 5 | 8 | 1695 | 태그 | SmallAction::Skill2 태그 → L1735 스킬2 분기. (CCState tick 오프셋 8 과는 별개) | 4 |  |
| 6 | 9 | 1695 | 센티널 | SmallAction::Ult 태그 → %457(배치 B, L1744~) 로 이탈. (L1608/1639/1663 의 `assume t != 9` 는 ProjectileMoveType 니치 태그 9 부재 가정) | 4 |  |
| 7 | -6 | 1524 | 미상 | CCState::block_input() 접힘: (tag + -6) <u -4(=0xFFFFFFFC) ⟺ tag ∉ {2 Bind, 3 BlockAttack, 4 BlockSkill, 5 BlockMoveSkill}. 즉 Airborne·Stun·ForceMove·Taunt·Fear·Charm·Animation 만 통과 | 4 |  |
| 8 | -4 | 1524 | 임계 | 위 block_input 접힘의 비교 상수(u32 -4) | 4 |  |
| 9 | 10 | 1524 | 태그 | CCState::Animation 태그 — tick 필드가 +0x20(name String 뒤)이라 select(tag==10 ? 32 : 8) | 4 |  |
| 10 | 13 | 1699 | 태그 | EntityType::Champion 태그 — Entity::is_in_attack/is_in_skill/is_in_skill2 의 첫 조건(ty==Champion) | 4 |  |
| 11 | 3 | 1699 | 태그 | ChampionActionState::Attack 태그 — is_in_attack() 의 둘째 조건(L1699/L1708) | 4 |  |
| 12 | 4 | 1718 | 태그 | ChampionActionState::Skill 태그 — is_in_skill()(L1718/L1727). Projectile is_targeting 의 idx 4(Target) 로도 등장 | 4 |  |
| 13 | -1 | 1697 | 센티널 | Option<Effect> None 니치(casting i32 == -1) — attack/skill/skill2 effect unwrap 검사 · CastingType 검사 `add -1` 의 피연산자 | 4 |  |
| 14 | 1 | 1497 | 태그 | 1 - team(상대 팀) · CastingType::Position(1) 하한 · BouncingTarget target_id Some 태그(L1608 t==1) · 카운터 +1 (applyed_cc/risk_cc/len). shl 피연산자 아님 | 4 |  |
| 15 | 100 | 1604 | 계수 | Entity::radius() 백분율: radius*(radius_mult+100)/100 (entity.rs:1515, mult==0 이면 나눗셈 생략) | 4 |  |
| 16 | 22500000000 | 1497 | 임계 | 150000² — near_minions 필터(L1497/1498) 및 near_towers 필터(L1583/1586)의 champ 거리 임계(제곱비교, `<`). aux 클로저에 있음 | 4 |  |
| 17 | 40000000001 | 1507 | 임계 | 200000²+1 — near_allies/near_enemies_with_action 필터(L1507/L1516)의 champ 거리 임계(`< 200000²+1` ⟺ `<= 200000²`). aux 클로저 | 4 |  |
| 18 | 4900000000 | 1584 | 미상 | 70000² — near_towers 필터의 대안 조건: 근처 적/아군(with_action) 중 하나가 타워에서 70000 미만(L1584/L1587). aux 클로저 | 4 |  |
| 19 | 121 | 1797 | 임계 | tick 임계: `attack/skill/skill2_tick <u 121` 즉 ≤120틱(=2초@60tps)이어야 PossibleGain 등록. 소스가 `<= 120` 인지 `< 121` 인지는 표기 불가(외연 동일). 사이트: 1797·1810·1825(아군 루프) · 1844·1857·1872(나) · 1962(배치 C 로 넘어가는 아군→적 루프 평타) + 1949/1950 의 skill/skill2 판정은 배치 C 에서 사용 | 4 |  |
| 20 | 20 | 1782 | 계수 | `e_speed * 20` — move_speed(0x640) 20틱분(≈1/3초@60tps)을 사거리에 더하는 이동 여유(추정). dbg 없음(`mul %433, 20` 호이스트)이라 1782/1793 중 어느 줄의 항인지 귀속 불가 — 값·용처는 확정: 평타·스킬·스킬2 도달 판정 전부에 가산 | 4 |  |
| 21 | 100 | 1782 | 계수 | Entity::radius(entity.rs:1511~1515) 인라인: radius_mult==0 ? radius : radius*(mult+100)/100 (퍼센트) | 4 |  |
| 22 | 13 | 1746 | 태그 | EntityType::Champion 태그(tcxdict --enum EntityType 13=Champion) — is_in_skill2/is_in_ult/is_in_attack/is_in_skill 의 첫 조건 · attack_cooldown switch 케이스 | 3 |  |
| 23 | 3 | 1896 | 태그 | ChampionActionState::Attack 메모리태그 3 — is_in_attack(entity.rs:1564~1565) | 4 |  |
| 24 | 4 | 1907 | 태그 | ChampionActionState::Skill 태그 4 — is_in_skill(entity.rs:1571~1572). ⚠같은 값 4 가 줄 1756/1928 `level > 4`(ult_effect 게이트, entity.rs:1701 `icmp ugt level, 4`)에도 쓰임 | 4 |  |
| 25 | 5 | 1746 | 태그 | ChampionActionState::Skill2 태그 5 — is_in_skill2(entity.rs:1579~1580) | 4 |  |
| 26 | 6 | 1758 | 태그 | ChampionActionState::Ult 태그 6 — is_in_ult(entity.rs:1586~1587). ⚠같은 값 6 이 switch 키 SmallAction::Attack(6)과 CCState 판정 `add -6` 에도 쓰임 | 4 |  |
| 27 | 2 | 1787 | 임계 | `level > 2`(icmp ugt) — Entity::skill2_effect(entity.rs:1693) 존재 게이트 · 또 is_nontarget(type.rs:149) 의 `casting-1 <u 2` = casting ∈{1 Position, 2 Direction} | 4 |  |
| 28 | -1 | 1781 | 센티널 | Option<Effect> None 니치(casting@tag i32 = -1) — attack/skill/skill2/ult effect 유무 판정(option.rs:742 as_ref 인라인) | 4 |  |
| 29 | 0 | 1884 | 태그 | SmallAction::RunAway 태그 0 — `*a == SmallAction::RunAway` 이면 continue(blackboard.rs:81 derived eq 인라인) | 4 |  |
| 30 | 7 | 1891 | 태그 | switch 키 SmallAction::Skill(7) (6 Attack·8 Skill2·9 Ult) | 4 |  |
| 31 | 8 | 1891 | 태그 | switch 키 SmallAction::Skill2(8) | 4 |  |
| 32 | 9 | 1891 | 임계 | switch 키 SmallAction::Ult(9) | 4 |  |
| 33 | 10 | 1889 | 태그 | CCState::Animation 태그 10 — tick 필드가 +0x20(String name 24B 뒤), 나머지 variant 는 +0x8 | 4 |  |
| 34 | -6 | 1889 | 미상 | CCState::block_input(entity.rs:521) 접힘: `(tag-6) as u32 < 0xFFFFFFFC` ⟺ tag ∉ {2,3,4,5} (Bind/BlockAttack/BlockSkill/BlockMoveSkill 은 입력을 막지 않음) | 4 |  |
| 35 | -4 | 1889 | 임계 | 위 block_input 비교 상수(0xFFFFFFFC) | 4 |  |
| 36 | 32 | 1889 | 산출값 | CCState::tick 접힘(select 10? 32 : 8) — Animation 페이로드 tick 오프셋. 판정값 아님(레이아웃) | 4 |  |
| 37 | 121 | 1975 | 임계 | 틱 임계: e_skill_tick < 121 (≤120틱 = 2초@60tps) 일 때만 스킬 위협 적재. 리터럴은 배치 B 구간에 호이스트(m07.ll:41735 %1514) — 같은 값이 L1990(41741 %1520) · L2069(42988) · L2082(43125) · L2097(43262) 에도 | 4 |  |
| 38 | 121 | 2069 | 임계 | attack_tick < 121 (자기 평타 위협 적재 조건 · m07.ll:42988) | 4 |  |
| 39 | 121 | 2082 | 임계 | skill_tick < 121 (m07.ll:43125) | 4 |  |
| 40 | 121 | 2097 | 임계 | skill2_tick < 121 (m07.ll:43262) | 4 |  |
| 41 | 20 | 2069 | 계수 | move_speed × 20 = 20틱(1/3초) 이동 여유를 사거리에 가산(m07.ll:42996 · L2082 43130 · L2097 43267 · e 쪽은 배치 B 41725 %1504) | 4 |  |
| 42 | 30 | 2123 | 계수 | d = champ.move_speed × 30 = 30틱(0.5초) 이동 여유 — is_near_tower_range 의 d 인자(m07.ll:43474 · L2177 43982) | 4 |  |
| 43 | 100 | 1974 | 계수 | Entity::radius(): radius × (radius_mult + 100) / 100 (entity.rs:1515 인라인 · udiv) | 4 |  |
| 44 | -1 | 1972 | 센티널 | Option<Effect> 니치 태그(casting@tag == 0xFFFFFFFF) = None. skill/skill2/ult/attack 전부 이 검사 | 4 |  |
| 45 | -6 | 2006 | 미상 | CC 필터 (tag-6) <u -4 ⟺ tag ∈ {0,1,6,7,8,9,10} — 접힌 matches! 체인(m07.ll:42224) | 4 |  |
| 46 | -4 | 2006 | 임계 | 위 필터의 비교값(m07.ll:42225) | 4 |  |
| 47 | 10 | 2006 | 태그 | CCState::Animation 태그 — tick 오프셋이 +0x20(m07.ll:42237) | 4 |  |
| 48 | 0 | 2004 | 태그 | SmallAction::RunAway 태그 — parameter.player.action != RunAway 일 때만 L2005~2100 실행(m07.ll:42143) · L2146 에서는 blackboard small_action == Some(RunAway)(m07.ll:43641) | 4 |  |
| 49 | 6 | 2008 | 태그 | SmallAction::Attack 태그(switch m07.ll:42257) | 4 |  |
| 50 | 7 | 2008 | 태그 | SmallAction::Skill | 4 |  |
| 51 | 8 | 2008 | 태그 | SmallAction::Skill2 | 4 |  |
| 52 | 9 | 2008 | 임계 | SmallAction::Ult | 4 |  |
| 53 | 13 | 2013 | 태그 | EntityType::Champion 태그 (is_in_attack/is_in_skill/is_in_skill2/is_in_ult · L2077/2092 *_cooldown · L2135 is_champion · L2067 switch case 13) | 4 |  |
| 54 | 3 | 2013 | 임계 | ChampionActionState::Attack (entity.rs:1565 is_in_attack) | 4 |  |
| 55 | 4 | 2024 | 임계 | ChampionActionState::Skill (entity.rs:1572 is_in_skill) · L2045 ult_effect(): level > 4 | 4 |  |
| 56 | 5 | 2036 | 태그 | ChampionActionState::Skill2 (entity.rs:1580 is_in_skill2) · L2111 TutorialType::MidBottom 태그 | 4 |  |
| 57 | 6 | 2049 | 태그 | ChampionActionState::Ult (entity.rs:1587 is_in_ult) | 4 |  |
| 58 | 2 | 2013 | 태그 | is_nontarget: (casting-1) <u 2 ⟺ casting ∈ {1 Position, 2 Direction} (type.rs:149) · L2032 skill2_effect(): level > 2 · L2129 EntityType::Tower 태그 · L2141/2156 「사거리 안 우리 미니언 수 < 2」 임계 · L2145 팀 인덱스 bounds | 4 |  |
| 59 | 1 | 2013 | 태그 | is_nontarget 의 casting-1 은 IR 에 `add nsw i32 %x, -1` 로 나타남(리터럴 -1). 값 1 자체는 push 의 additional=1 · L2111 TutorialType::First 태그 | 4 |  |
| 60 | 5112 | 2114 | 산출값 | GameSetting+0x13f8 tower_attack_disable_tick (phi 상수 m07.ll:43343 — 오프셋이라 reads 에도) | 4 |  |
| 61 | 5120 | 2112 | 산출값 | GameSetting+0x1400 tower_attack_disable_tick_2v2 (First/Bottom) | 4 |  |
| 62 | 5128 | 2113 | 산출값 | GameSetting+0x1408 tower_attack_disable_tick_3v3 (MidBottom) | 4 |  |
| 63 | 2 | 2183 | 태그 | EntityType::Tower 태그(2183·2230) | 4 |  |
| 64 | 13 | 2189 | 태그 | EntityType::Champion 태그 — is_champion() 인라인(2189·2236) | 4 |  |
| 65 | -1 | 2190 | 센티널 | Option<Effect>(attack_effect) None 니치 = casting@tag(i32, +0x4c0) == -1. 23개 unwrap/as_ref 사이트 전부 | 4 |  |
| 66 | 3 | 2199 | 임계 | 타워 사거리 내 '타워의 적 미니언' 수 임계: count < 3 → risk_damage(타워가 곧 챔프를 침), ≥3 → risk_possible_tower(2199·2246) | 4 |  |
| 67 | 30 | 2224 | 계수 | d = champ.stat_cached.move_speed × 30 (30틱 이동 여유 · is_near_tower_range 인자 d). 같은 리터럴이 2337 setting.rs:703 `tps*30`(라인 페이즈 = 에픽 첫 스폰 30초 전) 에도 | 4 |  |
| 68 | 100 | 2224 | 계수 | Entity::radius(): radius*(radius_mult+100)/100 (entity.rs:1515) · 2341 wave_is_dangerous 백분율 ×100 | 4 |  |
| 69 | 22500000000 | 2266 | 임계 | 150000² — others 소환수 후보 반경(2266 `> 150000²` → continue) · 2348 near_jungles 필터(`< 150000²`, aux m07.ll:61592) | 4 |  |
| 70 | 2500000001 | 2293 | 임계 | 50000²+1 — `dist² < 50000²+1`(= ≤50000²): 타깃 없는 소환수(2293)/미니언(2325)이 내게 주는 피해를 risk 로 치는 반경 | 4 |  |
| 71 | 7 | 2270 | 태그 | EntityType::Ghoul 태그(switch case) | 4 |  |
| 72 | 9 | 2270 | 태그 | EntityType::Bear 태그 | 4 |  |
| 73 | 10 | 2270 | 태그 | EntityType::Eagle 태그(2272 · nearest_enemy 가 +0x70/+0x78) | 4 |  |
| 74 | 1 | 2302 | 태그 | EntityType::Minion 태그 · 2341 `max(x,1)` 분모 하한 · 2415 `lshr 1`(= /2, 표기는 folded_from 참조) | 4 |  |
| 75 | 4 | 2354 | 태그 | EntityType::Jungle 태그 | 4 |  |
| 76 | 5 | 2354 | 태그 | EntityType::Epic 태그 · 2337 TutorialType::MidBottom(5) 도 같은 리터럴 | 4 |  |
| 77 | 6 | 2354 | 태그 | EntityType::Serpen 태그 | 4 |  |
| 78 | 0 | 2337 | 태그 | TutorialType::None(0) — is_line_phase 의 tick 검사 대상 튜토리얼 집합 {0,5,7,8}. 2300 direct 초기값 0 · 2341 damage==0 조기 0 | 4 |  |
| 79 | 8 | 2337 | 태그 | TutorialType::Total(8) | 4 |  |
| 80 | 49 | 2341 | 임계 | minion_wave_risk.rs:71 (인라인 enemy_minion_wave_is_dangerous): damage_pct > 49 (또는 damage >= hp) → 위험 | 4 |  |
| 81 | 66 | 2341 | 임계 | minion_wave_risk.rs:73: hp_pct < 66 && damage_pct > 29 | 4 |  |
| 82 | 29 | 2341 | 인덱스 | minion_wave_risk.rs:73 damage_pct > 29 · 2412/2413 `min(cell, 29)` 셀 인덱스 상한(30칸 그리드) | 4 |  |
| 83 | 41 | 2341 | 임계 | minion_wave_risk.rs:74: hp_pct < 41 && damage_pct > 17 | 4 |  |
| 84 | 17 | 2341 | 임계 | minion_wave_risk.rs:74 | 4 |  |
| 85 | 26 | 2341 | 임계 | minion_wave_risk.rs:75: hp_pct < 26 && damage_pct > 9 | 4 |  |
| 86 | 32000 | 2412 | 계수 | 셀 크기(좌표→셀: coord/32000) — map_cell_index | 4 |  |
| 87 | 1 | 2415 | 임계 | ChampionScoreParameter::adjust(score_parameter.rs:1444): `risk_damage /= 2` 가 `lshr i64 %x, 1` 로 접힘 — player(45981)·near_allies 전원(46017)·near_enemies 전원(46063) | 4 | 2 |

**`knobs` 조정점 28건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 근처 미니언 판정 반경(champ 기준) | score_parameter.rs:1497~1498 (aux m07.ll:61364/61636) | 22500000000 | 150000²(≈4.7셀). 올리면 near_minions 에 더 먼 라인미니언이 들어가 뒤 배치의 미니언 위험/이득 계산 범위가 넓어진다 | 4 | 기존 |
| 1 | 근처 타워 판정 반경(champ 기준) | score_parameter.rs:1583/1586 (aux m07.ll:61683/61806) | 22500000000 | 150000². 올리면 near_towers 가 늘어 배치 C 의 risk_possible_tower 계산 대상이 는다 | 4 | 기존 |
| 2 | 근처 타워 대안 조건 — with_action 챔피언과 타워 거리 | score_parameter.rs:1584/1587 (aux m07.ll:61755/61878) | 4900000000 | 70000²(≈2.2셀). 챔프에게서 멀어도 근처 적/아군 중 누군가가 타워 70000 안이면 그 타워를 포함. 올리면 타워 포함이 늘어난다 | 4 | 기존 |
| 3 | 근처 아군/적 챔피언 판정 반경(champ 기준) | score_parameter.rs:1507/1516 (aux m07.ll:61440/61543) | 40000000001 | 200000²+1(≈6.25셀, `<=200000²`). 올리면 near_allies/near_enemies 및 *_with_action 이 늘어 이후 모든 상호작용 대조 대상이 는다 | 4 | 기존 |
| 4 | 적 챔피언 포함의 시야 조건 | score_parameter.rs:1515 → game_core Blackboard::is_recent_visible(_gcbc g07.ll:157005, 상수 120틱) | 120 | game_core 본문(constants 미등록). 현재 보이거나 최근 120틱(=2초@60tps) 내 보였던 적만 near_enemies 에 포함. 늘리면 시야 밖 적을 더 오래 위험/이득 계산에 남긴다 | 4 | 기존 |
| 5 | 행동 시간 act_tick 에 반영되는 CC 종류 | score_parameter.rs:1523~1524 / 1554~1555 / 1692~1693 (CCState::block_input entity.rs:521) | 태그 ∉ {2,3,4,5} | Airborne·Stun·ForceMove·Taunt·Fear·Charm·Animation 의 남은 tick 과 remain_action_time 의 max 가 action_time. Bind/Block* 은 무시 | 4 | 기존 |
| 6 | 확정(applyed) vs 위험(risk) 피해 분류 조건 | score_parameter.rs:1699/1708/1718/1727/1737 | casting∈{Position,Direction} \|\| !is_in_*() | 지정형(Targeting/None) 캐스팅이면서 적이 이미 해당 모션(Attack/Skill/Skill2) 중일 때만 applyed_damage, 아니면 risk_damage. 조건을 바꾸면 뒤 배치의 이득/위험 가중이 달라진다 | 4 | 기존 |
| 7 | 비겨냥 투사체의 '앞에 아군이 먼저 맞음' 면제 | score_parameter.rs:1671~1673 (Projectile::is_linear_dist_no_penetrate projectile.rs:122) | LinearDist && !penetrate && dist > nearest_other_distance | 직선·비관통 투사체는 나보다 가까운 아군이 궤도에 있으면 내 risk 에 안 들어간다. 관통이면 항상 들어간다 | 4 | 기존 |
| 8 | 스킬2 효과 게이트 | game_core entity.rs:1693 (L1735 인라인) | 2 | level>2 일 때만 skill2_effect 를 돌려주고 그 외엔 None → 이 코드는 unwrap 하므로 적이 level<=2 에 Skill2 행동을 잡고 있으면 패닉. 게임측 불변식(2레벨 이하엔 Skill2 를 고르지 않음)에 의존 | 4 | 기존 |
| 9 | 엔티티 반경 버프 백분율 | game_core entity.rs:1511~1515 (L1604/1635/1659 인라인) | 100 | radius*(radius_mult+100)/100 — is_in_orbit 판정 반경. mult 가 크면 투사체 궤도에 더 잘 걸린다 | 4 | 기존 |
| 10 | PossibleGain 등록 tick 창 | score_parameter.rs:1797/1810/1825/1844/1857/1872/1962 (`< 121`) | 121 | 내리면(예: 61) 1초 안에 행동 가능한 적만 '가능한 피해'로 잡혀 risk_possible 이 줄고, 올리면 쿨다운/CC 가 긴 적까지 위험으로 셈 — 소비자(position_eval/action_score 계열)의 위험 가격이 그만큼 커진다 | 4 | 기존 |
| 11 | 사거리 이동 여유 계수 | score_parameter.rs:1782~1793 (`e_speed * 20`, dbg 없음) | 20 | 올리면 적이 더 먼 거리에서도 '닿는다'로 판정돼 risk_possible 등록이 늘어난다(스킬·평타 전부 공통). 0 이면 현재 사거리+반경만 | 4 | 기존 |
| 12 | 논타겟 스킬은 항상 risk 로 분류 | score_parameter.rs:1746/1758/1767/1896/1907/1919/1932 (`is_nontarget() \|\|`) | casting∈{Position,Direction} | 논타겟(위치/방향) 캐스팅은 실제 시전 중이어도 applyed_damage 가 아니라 risk_damage 로 누적된다 — 빗나갈 수 있는 피해를 '확정'으로 세지 않는다. 이 조건을 빼면 논타겟 시전 중 피해가 applyed 로 옮겨간다 | 4 | 기존 |
| 13 | 궁은 아군/나 risk_possible 계산에서 제외 | score_parameter.rs:1788~1836 · 1837~1875 (ult 분기 없음) | 평타·스킬·스킬2만 | 적의 궁 사거리·쿨다운은 PossibleGain 에 안 잡힌다(궁 위협은 다른 경로). 추가하려면 e.ult_effect()+ult_cooldown(0xc8) 분기를 붙여야 한다 | 4 | 기존 |
| 14 | 위협 적재 틱 창 (e_skill/e_skill2/attack/skill/skill2 _tick < 121) | score_parameter.rs:1975/1990/2069/2082/2097 (m07.ll:41735·41741·42988·43125·43262) | 121 | 올리면 더 먼 미래(쿨다운·CC 중)의 스킬/평타까지 risk_possible 로 잡아 적 위험 평가가 비관적으로 커진다. 내리면 즉시 가능한 위협만 남는다 | 4 | 기존 |
| 15 | 사거리 이동 여유(챔피언 위협 판정) | score_parameter.rs:2069/2082/2097 및 배치 B 1955 부근 (m07.ll:42996·43130·43267·41725) | 20 | move_speed×N 틱만큼 사거리를 늘려 본다. 올리면 더 먼 적도 위협권으로 들어온다 | 4 | 기존 |
| 16 | 사거리 이동 여유(타워 판정 d) | score_parameter.rs:2123/2177 (m07.ll:43474·43982) | 30 | 타워 사거리에 챔피언 이동 0.5초분을 더해 본다. 올리면 타워 위협이 더 넓은 반경에서 잡힌다 | 4 | 기존 |
| 17 | 타워가 나를 노릴 확률 대리 조건: 사거리 안 우리 미니언 수 < 2 | score_parameter.rs:2141/2156 (m07.ll:43707·43777) | 2 | 올리면(예 3) 미니언이 2마리 있어도 risk_possible_tower 를 얹어 타워 근접을 더 꺼린다 | 4 | 기존 |
| 18 | 스킬2/궁 가용 레벨 게이트 (skill2_effect: level>2 · ult_effect: level>4) | entity.rs:1693/1701 (인라인 · m07.ll:42331·42345·43210) | 2 / 4 | 낮추면 저레벨 스킬2/궁 위협·확정대미지가 계산에 들어온다 — game_core 헬퍼라 모드에서 바꾸려면 재구현 지점 | 4 | 기존 |
| 19 | 타워 판정 게이트: game.tick() <= setting.tower_attack_disable_tick(_2v2/_3v3) | score_parameter.rs:2117 (m07.ll:43353) | GameSetting+0x13f8/0x1400/0x1408 | 타워 공격 불능 시점 이후엔 L2118~2264 타워 위협을 전혀 계산하지 않는다(설정값 · 코드 상수 아님) | 4 | 기존 |
| 20 | 타워 '직접 위험' 판정용 적 미니언 수 임계 | score_parameter.rs:2199 · 2246 | 3 | 올리면 타워 사거리 안에 미니언이 더 많아도 '타워가 곧 챔프를 친다'(risk_damage)로 분류 → 타워 밑 위험이 커져 다이브/타워 근접이 줄어든다. 내리면 risk_possible_tower 쪽으로 빠져 덜 보수적 | 4 | 기존 |
| 21 | 타워 사거리 판정 이동 여유(틱) | score_parameter.rs:2224 (배치 C 2177 도 동일) | 30 | d = move_speed×30. 올리면 더 먼 타워까지 '사거리 안'으로 보아 risk 가 붙는 챔프가 늘어난다 | 4 | 기존 |
| 22 | 소환수(others)·정글몹 탐색 반경 | score_parameter.rs:2266 · 2348 | 22500000000 | 150000²(≈4.7셀). 올리면 더 먼 소환수/정글몹의 타깃 피해까지 risk 에 합산 | 4 | 기존 |
| 23 | 타깃 없는 소환수/미니언 근접 위험 반경 | score_parameter.rs:2293 · 2325 | 2500000001 | ≤50000²(≈1.56셀). 올리면 아직 나를 안 노리는 소환수/적 미니언의 기대피해도 내 risk_damage 에 더 넓게 합산 | 4 | 기존 |
| 24 | 미니언 웨이브 위험 인정 임계(웨이브 페이즈) | minion_wave_risk.rs:71~75 (2341 에 인라인) | damage>=hp \|\| dmg%>49 \|\| (hp%<66&&dmg%>29) \|\| (hp%<41&&dmg%>17) \|\| (hp%<26&&dmg%>9) | 임계를 낮추면 낮은 웨이브 피해도 risk_damage 에 반영돼 라인 근접이 보수화 | 4 | 기존 |
| 25 | 라인 페이즈 판정 = 에픽 첫 스폰 30초 전까지 | setting.rs:703 (2337 에 인라인) | tps*30 | 30 을 키우면 line_action_danger(적 미니언 실제 행동 기반) 대신 wave_risk(웨이브 총량 기반) 경로가 더 일찍 켜진다. 튜토리얼 {First,TopSolo,Bottom,MidSolo,JungleOnly} 는 항상 라인 페이즈 | 4 | 기존 |
| 26 | risk_damage 최종 반감 | score_parameter.rs:1444 (2415 adjust) | /2 (lshr 1) | player·near_allies·near_enemies 전원의 risk_damage 만 반으로(risk_epic_damage·risk_possible_tower 는 그대로). 나누기 계수를 바꾸면 모든 risk 기반 판정의 스케일이 통째로 바뀐다 | 4 | 기존 |
| 27 | 셀 인덱스 상한 | score_parameter.rs:14 (2412/2413) | 29 | 30×30 그리드 가정. 맵이 커지면 cx/cy 가 29 로 포화 | 4 | 기존 |

<details><summary>`callees` 피호출자 79건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust | game_ai::ScoreParameter::<'a>::adjust | pub | fn(&mut game_ai::ScoreParameter<'a/#0>, &mut rand::rngs::std::StdRng) | game-ai\src\score_parameter.rs:375 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 1 | adjust | game_ai::ChampionScoreParameter::<'_>::adjust | pub | fn(&mut game_ai::ChampionScoreParameter</#0>, &mut rand::rngs::std::StdRng) | game-ai\src\score_parameter.rs:1441 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 2 | adjust | game_core::InputTarget::adjust | pub | fn(&game_core::InputTarget, u64, u64, u64) -> game_core::InputTarget | game-core\src\simulation\state\player.rs:1465 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | attack_cooldown | game_core::Entity::attack_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1747 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | block_input | game_core::Entity::block_input | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1501 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | block_input | game_core::CCState::block_input | pub | fn(&game_core::CCState) -> bool | game-core\src\simulation\entity.rs:520 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | check_projectile | game_core::CastingTarget::check_projectile | pub | fn(&game_core::CastingTarget, &game_core::Projectile, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:248 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 12 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 13 | enemy_minion_line_action_danger_damage_at | game_ai::enemy_minion_line_action_danger_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize, bool, bool) -> usize | game-ai\src\minion_wave_risk.rs:233 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | enemy_minion_wave_danger_damage_at | game_ai::enemy_minion_wave_danger_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize) -> usize | game-ai\src\minion_wave_risk.rs:99 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | enemy_minion_wave_is_dangerous | game_ai::minion_wave_risk::enemy_minion_wave_is_dangerous | in:game_ai | fn(&game_core::Entity, usize) -> bool | game-ai\src\minion_wave_risk.rs:64 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | enemy_minion_wave_risk_damage_at | game_ai::enemy_minion_wave_risk_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize) -> usize | game-ai\src\minion_wave_risk.rs:6 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | expected_damage_target | game_core::Projectile::expected_damage_target | pub | fn(&game_core::Projectile, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\projectile.rs:1287 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 20 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 21 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 22 | growth | game_core::ChampionInfo::growth | pub | fn(&Self/#0) -> game_core::EntityStat | game-core\src\setting.rs:1083 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 23 | growth | <game_core::DataChampionInfo as game_core::ChampionInfo>::growth | pub | fn(&game_core::DataChampionInfo) -> game_core::EntityStat | game-core\src\setting\champion\data_driven.rs:2573 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 24 | growth | <game_core::MonkChampionInfo as game_core::ChampionInfo>::growth | pub | fn(&game_core::MonkChampionInfo) -> game_core::EntityStat | game-core\src\setting\champion\monk.rs:62 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 25 | has_cc | game_core::Projectile::has_cc | pub | fn(&game_core::Projectile) -> bool | game-core\src\simulation\projectile.rs:1272 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | is_champion | game_core::EntityType::is_champion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 27 | is_in_attack | game_core::Entity::is_in_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1563 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 28 | is_in_orbit | game_core::Projectile::is_in_orbit | pub | fn(&game_core::Projectile, u64, u64, u64) -> bool | game-core\src\simulation\projectile.rs:973 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | is_in_skill | game_core::Entity::is_in_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1570 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 31 | is_in_skill2 | game_core::Entity::is_in_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1578 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | is_in_ult | game_core::Entity::is_in_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1585 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 33 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 34 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 35 | is_linear_dist_no_penetrate | game_core::ProjectileMoveType::is_linear_dist_no_penetrate | pub | fn(&game_core::ProjectileMoveType) -> bool | game-core\src\simulation\projectile.rs:120 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 36 | is_near_tower_range | game_ai::is_near_tower_range | pub | fn(&game_core::Entity, u64, u64, u64, u64) -> bool | game-ai\src\score_parameter.rs:1448 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 37 | is_nontarget | game_core::CastingType::is_nontarget | pub | fn(&game_core::CastingType) -> bool | game-core\src\simulation\effect\type.rs:148 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 38 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | is_targeting | game_core::ProjectileMoveType::is_targeting | pub | fn(&game_core::ProjectileMoveType) -> bool | game-core\src\simulation\projectile.rs:133 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 40 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 41 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 42 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 43 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 44 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 45 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 46 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 47 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 48 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 49 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 50 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 51 | map_cell_index | game_ai::score_parameter::map_cell_index | in:game_ai | fn(u64) -> i32 | game-ai\src\score_parameter.rs:13 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 52 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 53 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 54 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 55 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 56 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 57 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 58 | precompute_champion_powers | game_ai::precompute_champion_powers | pub | fn(usize, &game_core::OperationData, &mut game_ai::ChampionScoreParameter) | game-ai\src\utils.rs:792 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 59 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 60 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 61 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 62 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 63 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 64 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 65 | remain_action_time | game_core::Entity::remain_action_time | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 66 | skill2_cooldown | game_core::Entity::skill2_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1789 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 67 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 68 | skill_cooldown | game_core::Entity::skill_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1774 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 69 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 70 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 71 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 72 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 73 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 74 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 75 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 76 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 77 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 78 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 40개**: `act_tick`, `applyed_damage`, `bool`, `casting`, `chain`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `e_attack_tick`, `e_skill2_eff`, `e_skill2_tick`, `e_skill_tick`, `enumerate`, `find  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `first_spawn_tick`, `jungles`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `llvm.umax.i64`, `llvm.umin.i64`, `llvm.usub.sat.i64`, `map_or`, `move_speed`, `my_id`, `near_allies_with_action`, `near_enemies_with_action`, `near_jungles`, `near_minions`, `near_towers`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `pool`, `reserve_internal_or_panic`, `risk_damage`, `risk_epic_damage`, `risk_possible`, `risk_possible_tower`, `size_hint`, `target`, `tower_attack_disable_tick`, `tower_attack_disable_tick_2v2`, `tower_attack_disable_tick_3v3`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m13.ll:45850, m15.ll:23147, m15.ll:56921) · **형제 0개** 

**`open` 34건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | (배치 A) L1507/L1515 의 `is_some_and` 안 두 조건(id!=champ.id / is_recent_visible 과 distance_sq) 의 소스 표기 순서 — IR 은 각각 id 비교→거리, is_recent_visible→거리 순으로 평가하지만 column 정보가 없어 `A && B` 의 표기 순서는 관측된 평가 순서로만 적음(표기 불가, 동작은 확정). | 4 |  |
| 1 | 표기 불가 | (배치 A) L1699 계열 조건의 소스 형태 — IR 은 casting∈{1,2} 를 먼저 보고 거짓일 때만 is_in_*() 를 본다. `A \|\| !B` 인지 `!(matches!(casting, Targeting\|None) && B)` 인지는 외연이 같아 표기 불가(동작 확정). | 4 |  |
| 2 | 재료 부재 | (배치 A) L1673 의 `&&` 두 항(is_linear_dist_no_penetrate() 와 dist > nearest) 표기 순서 — IR 이 idx==0 과 dist>nearest 를 비단락 `and` 로 묶은 뒤 penetrate 를 읽어 순서 복원 불가(동작 확정). | 4 |  |
| 3 | 미탐색 | (배치 A) L1692~1693 에서 계산한 적의 act_tick(%430) 은 배치 A 범위 안에서 소비처가 없다 — Ult 분기(%457)·후속 %431(L1781~) 에서 쓰이는지는 배치 B 몫. | 4 |  |
| 4 | 미탐색 | (배치 A) Blackboard::is_recent_visible 의 내부(`game.is_visible(team,id)` vtable+0xf8 · get_player_by_champion_id vtable+0x150 · last_visible[pos]+120 >= tick)는 game_core 경계라 시그니처·반환 의미 수준만 확인(_gcbc g07.ll:157005~). 상수 120 은 본문/aux 밖이라 constants 에 넣지 않고 knobs 에만 적음. | 4 |  |
| 5 | 미탐색 | (배치 A) 왜 적 팀 블랙보드(blackboard[1-player.team])의 small_actions 를 그대로 읽는지(AI 가 상대 행동을 직접 아는 설계) — 소스 주석 부재(_docs 0건). 관측 사실만 기록. | 4 |  |
| 6 | 미탐색 | (배치 A) iter_minions(56B)·iter_towers_without_nexus(120B) 의 반환 이터레이터가 정확히 어느 Vec 들을 잇는지(top/mid/bottom 순서, twin_towers 포함 여부)는 game_core 본문 미독 — 타입 시그니처(Chain<Chain<Copied,Copied>,Copied> / Chain<Flatten<IntoIter<Option<&Entity>,6>>,Copied>)로만 기술. | 4 |  |
| 7 | 미탐색 | (배치 A) wave_snapshot(+0x0 None 고정)·v3_turnback_hold(+0x1500 false 고정)·player.action(+0x918 RunAway 고정)이 이 함수 밖(score_parameter_cached 등)에서 채워지는지는 미탐색 — 이 함수 안에서는 초기값 외 store 가 없음을 전체 grep 으로 확인. | 4 |  |
| 8 | 미탐색 | (배치 A) 블록 %3451(root `line 1`, iterator 인라인 잔류)은 아군 투사체 루프(L1598)의 continue 간선으로 귀속했다 — 소스 줄 없음. | 4 |  |
| 9 | 표기 불가 | (배치 B) `e_speed*20` 항(%799/%800/%810, dbg 없음·호이스트)이 소스 1782(`e_atk_range` 정의)에 포함되는지 1793/1809 의 도달식에 따로 더해지는지 — 값·용처는 확정, 줄 귀속만 표기 불가(column 부재) | 4 |  |
| 10 | 미탐색 | (배치 B) 1793↔1797, 1809↔1810, 1824↔1825, 1958↔1962 에서 `+ ae.radius()` 가 어느 줄 표현식에 속하는지 — radius 계산은 앞 줄 chain, add 명령은 뒷 줄 dbg. 의미 동일(제곱 전에 더함) | 4 |  |
| 11 | 미탐색 | (배치 B) IR 없는 코드형 줄의 정체: 1780/1945(39자, dbg 이름 e_atk_eff=e 로 `let e_atk_eff = &e.attack_effect;` 추정) · 1786/1951(40자, e_skill_eff) · 1791~1792/1956~1957(29/25자) · 1795(41) · 1801/1805/1814/1820/1829(31~41) · 1838~1839(48/51) · 1848(27) · 1861/1876(30/31) · 1960(41) — 영문 주석이거나 접힌 코드. rmeta 에 원문 없음 | 4 |  |
| 12 | 미탐색 | (배치 B) near_allies_with_action 원소(40B) 의 +0x18 i64 필드 의미 — 내 범위에서 load 0회(배치 A 의 채우는 곳 1521 참조 필요) | 4 |  |
| 13 | 미탐색 | (배치 B) expected_damage_target(effect.rs:91 · 인자 &Effect,&GameContext(64B),&dyn AbstractEntity(vtable @anon.11 = Entity as AbstractEntity),&Entity → i64) · range_adjust(effect.rs:29 · &Effect,&Entity,&Entity → i64) · CastingTarget::check(type.rs:227 · &CastingTarget,&Entity,&Entity → bool) · remain_action_time(&Entity → i64) 내부는 game_core 경계라 계약만(정의 g06.ll:52355/51785/86744/67110) | 4 |  |
| 14 | 재료 부재 | (배치 B) get_entity_by_id 는 dyn AbstractGame vtable+0x1f0 간접 호출(divtable 근거)이라 calls 에 심볼로 못 올림 — 런타임 구현체(ExpectedGame 등)는 이 도구로 확정 불가 | 3 |  |
| 15 | 미탐색 | (배치 B) 1842/1852/1867 에서 attack/skill/skill2_tick 을 1783~1785 값 재사용 없이 다시 계산하는 이유(소스가 별도 let 인지) — 값은 동일하므로 동작엔 영향 없음 | 4 |  |
| 16 | 미탐색 | (배치 B) aux fold(m12.ll:19396) 의 max_by 는 `acc > x ? acc : x`(같으면 새 값) — Iterator::max 의 표준 동작이라 값 영향 없음, 기록만 | 4 |  |
| 17 | 표기 불가 | (배치 C) L1975/1990/2069/2082/2097 의 `tick < 121 && dist <= reach²` 두 조건의 소스 표기 순서 — column 부재로 표기 불가. IR 분기 순서는 tick 검사가 먼저. | 4 |  |
| 18 | 표기 불가 | (배치 C) L2013/2024/2036/2049 `is_nontarget() \|\| !is_in_*()` 의 소스 표기(`\|\|` 순서·부정 위치) — 분기 방향으로 외연만 복원. 표기 불가. | 4 |  |
| 19 | 표기 불가 | (배치 C) Effect::range(&self, e)(effect.rs:26) 안에서 세 항(stat_buff range · range · growth×(level-1))의 덧셈 순서 — 표기 불가(외연 동일). | 4 |  |
| 20 | 미탐색 | (배치 C) TutorialType::player_count()(runner.rs:295) 의 반환값 자체는 접혀 사라졌다(태그→GameSetting 오프셋 phi 만 남음). 「First/Bottom=2 · MidBottom=3 · 그 외=기본」 은 이 phi 매핑에서 역추론한 추정. | 5 |  |
| 21 | 미탐색 | (배치 C) AbstractGameWithCache::iter_minions(cache, team) 의 반환 3-체인이 top/mid/bottom_minions[team] 인지는 시그니처(sret 56B · Chain<Chain<Copied,Copied>,Copied>)와 구조체 레이아웃으로 추정 — game_core 경계라 계약만. | 5 |  |
| 22 | 미탐색 | (배치 C) Effect::expected_damage_target / range_adjust / is_in_range · CastingTarget::check · Entity::remain_action_time · player_by_champion_id 내부 — game_core 경계, 계약만(시그니처는 _gcbc g06/g15 define 확인). | 4 |  |
| 23 | 미탐색 | (배치 C) 블록 %2227(m07.ll:43342~43350)은 `;L0` 만 있는 phi/gep 때문에 루트 줄 지도에서 배치 A(줄 0)로 분류되지만 L2111~2117 의 본체다 — 이 명세에 포함시켰다(배치 A 와 중복 가능). | 4 |  |
| 24 | 표기 불가 | (배치 C) risk_possible 는 `iter_mut` 가정: near_enemies 원소를 통해 쓰므로 &mut 순회로 읽었으나 소스가 index 루프인지 iter_mut 인지는 표기 불가. | 4 |  |
| 25 | 표기 불가 | (배치 D) 2189/2236 `if other.is_champion()` 의 소스 표기 — IR 은 is_champion 이면 아무 것도 안 하고 continue, 아니면 risk_possible_tower 가산. `if X {} else {..}` 와 `if !X {..}` 는 외연 동일 → 표기 불가(동작은 확정) | 4 |  |
| 26 | 미탐색 | (배치 D) 2362/2366(Jungle 분기)에서 find 한 target 에 더하는 값이 `dmg(j, me)`(target 인자 = %59 내 챔프, m07.ll:45642·45668)인 것은 IR 사실. 2284/2290/2315/2321 처럼 get_entity_by_id 로 target 엔티티를 구하지 않는다 — 의도(버그 여부)는 IR 로 판정 불가. 재현은 IR 대로 me 를 넘겨야 비트동일 | 1 |  |
| 27 | 재료 부재 | (배치 D) is_near_tower_range 1452 `check = d + r + range + t.radius()` 의 소스 항 순서 — IR 덧셈 순서는 (d+r)+attack.range+stat_buff.range+(level-1)*growth+radius (m07.ll:44458~44462). 한 줄 안 순서는 column 부재로 복원 불가(합은 가환이라 동작 무관) | 4 |  |
| 28 | 미탐색 | (배치 D) 2325 `dist < 50000²+1 && team !=` 의 소스 순서 — IR 은 dist 먼저(45030) 후 team(45252~45280). 둘 다 순수라 결과 동일 | 4 |  |
| 29 | 미탐색 | (배치 D) 2117 게이트 임계 %2230(MapDef/GameSetting +5112/5120/5128 중 하나 · phi 는 %1706 의 switch 결과) 의 정체 — 배치 C 범위. 내 범위엔 '그 게이트가 거짓이면 2118~2253 을 건너뛰고 2265 로 온다'는 진입 사실만 기록 | 4 |  |
| 30 | 미탐색 | (배치 D) cache.others[..] 에 어떤 엔티티 타입이 담기는지(Ghoul/Bear/Eagle 외 Illusion·Revenant·SmallJiangshi 등) — 2270 switch 가 7/9/10 만 잡고 나머지는 `_`(2293 경로). 원천은 game_core AbstractGameWithCache 생성자(미탐색) | 4 |  |
| 31 | 미탐색 | (배치 D) 2196/2243 count 클로저의 `t.attack_effect.as_ref().unwrap()` 은 원소마다 unwrap → 타워에 attack_effect 가 없고 해당 팀 미니언이 1개라도 있으면 패닉(aux m12.ll:42779 unwrap_failed). 실전에서 타워는 항상 Some 이라 추정(근거 없음 · '추정') | 4 |  |
| 32 | 미탐색 | (배치 D) 지시문의 '니치 +0x1a2' — ScoreParameter 5384B 레이아웃(tcxdict --deep)에서 +0x1a2 는 wave_snapshot.minions[1].checkpoint_hp 내부라 내 범위(2183~)와 무관. score_parameter_cached 의 Option<ScoreParameter> 니치는 r15 명세 소관으로 미확인 | 3 |  |
| 33 | 미탐색 | (배치 D) 콜리 계약만 확인(본문 미독): Effect::expected_damage_target(&Effect 56B, &GameContext 64B, caster: &dyn(팻포인터 data+vtable @anon.11=Entity vtable), target: &Entity) -> i64(g06.ll:52355) · Effect::is_in_range(&Effect, caster &Entity, target &Entity) -> bool(g06.ll:51634) · AbstractGameWithCache::iter_minions(sret 56B Chain<Chain<Copied<Iter<&Entity>>,Copied>,Copied>, &self, team) (g15.ll:102624) · enemy_minion_wave_risk_damage_at(_version, data &OperationData, target &Entity, x, y, window_tick) -> usize(m07.ll:47104) · enemy_minion_line_action_danger_damage_at(version, data, target, x, y, window_tick, champion_action: bool, predict_retarget: bool) -> usize(m07.ll:48226) | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 C) L2112 로 추정한 arm(First/Bottom → 2v2): switch 에서 %2227 로 직결돼 !dbg 줄이 없다. 2113=3v3 · 2114=기본 은 확정, 2v2 arm 의 정확한 줄은 미확정(줄 길이 산술 미시도). | 3 | 사실 서술 |
| 1 | (배치 C) vtable 간접 호출(AbstractGame::tick +0x28 · get_entity_by_id +0x1f0)은 `call ptr %N` 이라 calls 목록(C2)에 못 싣고 logic 에만 적었다(slot 이름은 divtable 로 확인). | 3 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | (배치 B) `< 121` 이 소스에서 `<= 120` 인지 `< 121` 인지(표기 불가·외연 동일). 120 이 `tps*2` 같은 식이 아니라 리터럴인 것은 확정(컨텍스트 tps 로드 없음) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

