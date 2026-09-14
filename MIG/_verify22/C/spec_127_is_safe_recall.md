---

### `127` is_safe_recall — 지금 귀환(채널링 recall_time 틱)을 시작해도 안전한가 — 정글/에픽 어그로·세르펜 근접·적 타워 사거리·(v2+) 날아오는 적 투사체/DoT 도트·적 미니언/적 챔피언/기타 적 유닛의 도달 시간을 전부 통과해야 true

| 항목 | 값 |
|---|---|
| id | `cast__is_safe_recall` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai12small_action4cast14is_safe_recall` |
| 소스 | `game-ai\src\small_action\cast.rs:304` |
| IR | `m07.ll` 58406~60188행 |
| 경로·가시성 | `game_ai::small_action::cast::is_safe_recall` · **in:game_ai** |
| 계층 | 기타 |
| exe | `d9ddc0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | L350 `icmp ugt %0, 1` — version≥2 면 투사체·DoT 검사(L351~394) 추가 후 공통 경로로 합류 | 4 |
| 1 | 2 | _rnd | &mut StdRng(320B) | ★미사용(readnone) — 이 함수는 rnd 스트림을 소비하지 않는다 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team·info.position + is_recent_visible_big_action 인자 | 4 |
| 3 | 4 | data | &OperationData(24B) | +0 cache · +8 context(setting.return_tick · pool) · +0x10 blackboard[2] | 4 |
| 4 | 5 | _positioning_score | &PositioningScoreData(2760B) | ★미사용(본문 참조 0회) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn is_safe_recall(version, _rnd, player, data, _positioning_score) -> bool {
  let team = player.info.team; let cache = data.cache; let game = cache.game;
  let champ = cache.player_champion[team][player.info.position].unwrap();                       // L305
  // L307~310 — 정글몹/에픽이 나를 조준(focused) 중이면 불가
  if cache.jungles.iter().any(|e| match e.ty { Jungle{info}|Epic{info} => info.focused == Some(champ.id), _ => false }) { return false; }
  // L319~320 — 세르펜(모바 모드 live_list 첫 id)이 120000 이내면 불가
  if let GameMode::Moba(m) = game.get_game_mode() {
     if let Some(serpen) = m.jungle_runner.serpen.live_list.first().and_then(|&id| game.get_entity_by_id(id)) {
        if dist2(serpen, champ) < 14400000001 { return false; } } }
  // L325~328 — 적 타워(넥서스 제외) 공격 사거리 안이면 불가
  let towers = cache.towers_without_nexus(1 - team, data.context.pool);
  if towers.iter().any(|t| t.attack_effect.as_ref().unwrap().is_in_range(t, champ)) { return false; }
  // L332~336 — 귀환 완료까지 남은 시간
  let mut recall_time = data.context.setting.return_tick;
  if let Champion(info) = &champ.ty { if let Return{time} = info.action_state { recall_time = recall_time.saturating_sub(time); } }
  // L350~394 — v2+: 날아오는 적 투사체 / 내게 걸린 DoT
  if version > 1 {
     for p in game.iter_projectile() {                                                        // L351
        if p.team == champ.team { continue; }                                                // L352
        if !p.applyed_target.check_projectile(p, champ) { continue; }
        if !p.is_visible { continue; }                                                       // L355
        if !p.is_in_orbit(champ.x, champ.y, champ.radius()) { continue; }                    // L358
        let hit_in = match p.move_type {                                                     // L361
           Delayed{tick, applyed, apply_done} => (!apply_done && applyed <= tick).then_some(applyed),   // L376 (관측: t = applyed)
           Periodic{tick, period, cool, first_delay} => {                                    // L363~373
              let rp = max(period,1);
              let next_in = if cool < first_delay { first_delay - cool }                     // L364~365
                            else if first_delay == 0 && cool < rp { rp - cool }              // L366~368
                            else { let base = cool - first_delay; (rp - base % rp) % rp };   // L370~371
              (next_in <= tick).then_some(next_in) }                                         // L373
           _ => None };
        if let Some(h) = hit_in { if h < recall_time { return false; } }                     // L380
     }
     for ce in champ.casted_effects.iter() {                                                 // L388
        if ce.casted_type == Heal { continue; }                                              // L389
        let period = max(ce.period,1); let next_in = (period - ce.tick % period) % period;   // L392~393
        if next_in + ce.tick < ce.duration && next_in < recall_time { return false; }        // L394
     }
  }
  // L400~402 — 최근접 아군 타워 / 적 미니언 / 아군 미니언 (min_by_key dist², 첫 최솟값)
  let nearest_tower = cache.iter_towers_without_nexus(team).min_by_key(|t| dist2(t, champ));
  let nearest_enemy_minion = cache.iter_minions(1 - team).min_by_key(|m| dist2(m, champ));
  let nearest_ally_minion = cache.iter_minions(team).min_by_key(|m| dist2(m, champ));
  // L404~427 — 최근접 적 미니언이 내 자리에 닿는 시간
  if let Some(minion) = nearest_enemy_minion {
     let range = minion.attack_effect.unwrap().range(minion) + minion.radius() + champ.radius();   // range_adjust 없음
     let dist = minion.distance(champ).saturating_sub(range);                                  // L405
     let mut bonus_time = 0;
     if let Some(tower) = nearest_tower {                                                      // L407
        if !(dist2(tower, minion) > dist2(champ, minion)) {                                    // 타워가 나보다 미니언에 가깝다(미니언이 타워를 지나야 함)
           bonus_time = if tower.hp*100 / tower.stat_cached.hp > 9 { return_tick } else { 30 }; } }   // L410
     if let Some(ally) = nearest_ally_minion {                                                 // L418
        if !(dist2(ally, minion) > dist2(champ, minion)) { bonus_time += 20; } }                // L419
     let move_time = dist / minion.stat_cached.move_speed + bonus_time;                        // L424 (speed 0 → panic)
     if move_time < recall_time { return false; }                                              // L427
  }
  // L433~459 — 최근 가시(big_action) 적 챔피언 5명: 최대 사거리(평타/스킬/스킬2 중 대상 가능한 것) 뺀 거리 / 이속
  for enemy in cache.player_champion[1-team].iter().flatten() {
     if !data.blackboard[1 - team].is_recent_visible_big_action(game, player, enemy) { continue; }   // L435 ★적팀 인덱스 블랙보드
     let atk = enemy.attack_effect.as_ref().unwrap();                                                // L439
     let mut max_range = atk.range(enemy) + atk.range_adjust(enemy, champ) + enemy.radius() + champ.radius();   // L439~441
     if let Some(s) = &enemy.skill_effect { if s.target.check(enemy, champ) {                       // L443~444
        max_range = max(max_range, s.range(enemy) + s.range_adjust(enemy, champ) + enemy.radius() + champ.radius()); } }   // L445
     let skill2 = if enemy.level > 2 { enemy.skill2_effect.as_ref() } else { None };                // L449 (entity.rs:1693)
     if let Some(s2) = skill2 { if s2.target.check(enemy, champ) { max_range = max(max_range, s2.range(enemy)+s2.range_adjust(enemy,champ)+enemy.radius()+champ.radius()); } }   // L450~451
     let d = enemy.distance(champ).saturating_sub(max_range);                                       // L455
     let move_time = d / enemy.stat_cached.move_speed;                                              // L457 (0 → panic)
     if move_time < recall_time { return false; }                                                   // L459
  }
  // L466~471 — 기타 적 유닛(cache.others[1-team]) 중 공격효과 있는 것
  for e in cache.others[1 - team].iter() {
     let Some(atk) = e.attack_effect.as_ref() else { continue; };                                   // L467
     let range = atk.range(e) + atk.range_adjust(e, champ) + e.radius() + champ.radius();            // L468
     let d = e.distance(champ).saturating_sub(range);                                                // L469
     let move_time = d / max(e.stat_cached.move_speed, 1);                                          // L470
     if move_time < recall_time { return false; }                                                   // L471
  }
  true                                                                                              // L478
}

분기 순서(IR 블록): %20 team bounds → %31 champ null(panic) → jungles 루프(%43~%59) → %72 Moba → 세르펜(%73~%107) → towers any(%117~%131) → recall_time(%138~%154) → %156 version>1 → [v2: 투사체(%330~%816) → DoT(%346~%368)] → 공통 %157(L400) → nearest 3종 → %321 enemy minion → … → %528 적 챔피언 5슬롯 → %534 others → ret.
Effect::range(caster) 인라인(effect.rs:26) = range + (caster.level-1)*growth_range + caster.stat_buff_cached.range. Entity::radius() 인라인(entity.rs:1511) = radius_mult==0 ? radius : radius*(mult+100)/100. dist2 = |Δx|²+|Δy|².
```

**`mem` 메모리 접근 61건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | <2 아니면 panic_bounds_check (m07:58442~58448, L305) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | (m07:58453~58455, L305) | 4 | OK |
| 2 | PlayerState | 0x0 | (전체) is_recent_visible_big_action 의 player 인자 | r | m07:59812 | 4 | OK |
| 3 | OperationData | 0x0 | cache | r | m07:58456 | 4 | OK |
| 4 | OperationData | 0x8 | context | r | m07:58654~58655 · context+0 pool(&Bump) → towers_without_nexus 인자(m07:58656) · context+8 setting → return_tick(m07:58742~58745) | 4 | OK |
| 5 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | ★blackboard[1-team](적 팀 인덱스, 744B stride) 를 is_recent_visible_big_action 의 self 로 (m07:59391~59393, 59812, L435) | 4 | OK |
| 6 | GameContext | 0x0 | pool | r | towers_without_nexus(cache, 1-team, pool) (m07:58656~58657, L325) | 4 | OK |
| 7 | GameContext | 0x8 | setting | r | m07:58742~58743 | 4 | OK |
| 8 | GameSetting | 0x1368 | return_tick | r | recall_time 기본값 (m07:58744~58745, L332) · L410 bonus_time 로도 재사용(m07:59551) | 4 | OK |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion[team][position]@Some.0 | r | None → unwrap_failed(L305 `.unwrap()`) (m07:58457~58463,58560) · L433 [1-team][0..5] 루프(m07:59385,59648~59651) | 4 | OK |
| 10 | AbstractGameWithCache | 0xd0 | jungles.buf.ptr | r | Vec<&Entity> (m07:58478~58479, L307) | 4 | OK |
| 11 | AbstractGameWithCache | 0xe8 | jungles.len | r | (m07:58481~58482) | 4 | OK |
| 12 | AbstractGameWithCache | 0x0 | game.data_ptr | r | (m07:58565) get_game_mode/get_entity_by_id/iter_projectile 수신자 | 4 | OK |
| 13 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x40 get_game_mode(m07:58568~58570) · +0x1f0 get_entity_by_id(m07:58610~58612) · +0x210 iter_projectile(m07:59267~59269) — divtable AbstractGame | 3 | OK |
| 14 | AbstractGameWithCache | 0xf0 | others[1-team].buf.ptr / +0x18 len | r | {ptr,ptr,i64},i64 stride 32B × (1-team) (m07:59654~59661, L466) | 4 | OK |
| 15 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.buf.ptr | r | (m07:58598~58603, L319) first() → 세르펜 id | 4 | OK |
| 16 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | 0 이면 skip (m07:58588~58595) | 4 | OK |
| 17 | Entity(jungle/epic) | 0x68 | ty@tag | r | switch 4=Jungle / 5=Epic (m07:58519~58524, L307) | 4 | OK |
| 18 | Entity(jungle/epic) | 0x88 | ty@Jungle\|Epic.info.focused@tag | r | i64 trunc→i1 Some (m07:58530~58533, 58554~58557, L308/L310) | 4 | OK |
| 19 | Entity(jungle/epic) | 0x90 | ty@Jungle\|Epic.info.focused@Some.0 | r | == champ.id → false (m07:58536~58539) | 4 | OK |
| 20 | Entity(champ) | 0x5c0 | id | r | (m07:58507~58508) | 4 | OK |
| 21 | Entity(champ) | 0x0 | team@tag | r | L352 투사체 팀 비교 (m07:59291) | 4 | OK |
| 22 | Entity(champ) | 0x8 | team@Player.0 | r | (m07:59272, 60046) | 4 | OK |
| 23 | Entity(champ) | 0x68 | ty@tag == 13(Champion) | r | (m07:58748~58750, L334) | 4 | OK |
| 24 | Entity(champ) | 0x70 | ty@Champion.0.action_state@tag == 1(Return) | r | (m07:58755~58757, L335) | 4 | OK |
| 25 | Entity(champ) | 0x78 | ty@Champion.0.action_state@Return.time | r | recall_time = return_tick.saturating_sub(time) — 이미 귀환 중이면 남은 시간 (m07:58761~58765, L335~336) | 4 | OK |
| 26 | Entity(champ) | 0x298 | casted_effects.buf.ptr | r | Vec<CastedEffect 56B> (m07:59301~59302, L388) | 4 | OK |
| 27 | Entity(champ) | 0x2a0 | casted_effects.len | r | (m07:59303~59304) | 4 | OK |
| 28 | CastedEffect(ce) | 0x34 | casted_type@tag | r | i8 == 3(Heal) 이면 skip (m07:59331~59334, L389) | 4 | OK |
| 29 | CastedEffect(ce) | 0x28 | period | r | max(period,1) (m07:59348~59352, L392) | 4 | OK |
| 30 | CastedEffect(ce) | 0x10 | tick | r | next_in = (period - tick%period)%period (m07:59354~59359, L393) | 4 | OK |
| 31 | CastedEffect(ce) | 0x18 | duration | r | next_in+tick < duration (m07:59361~59363, L394) | 4 | OK |
| 32 | Entity(champ/minion/enemy/e) | 0x660 | x | r | dist² 전역(L320/400~402/408/419) · is_in_orbit 인자(m07:59273,60057) | 4 | OK |
| 33 | Entity(champ/minion/enemy/e) | 0x668 | y | r | 동상 | 4 | OK |
| 34 | Entity(champ/minion/enemy/e) | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius() 인라인(entity.rs:1511~1515) 도처 (m07:59275,59379,59411,59439,59712,59839,59863,60059) | 4 | OK |
| 35 | Entity(champ/minion/enemy/e) | 0x680 | radius | r | 동상 (m07:59276,59380,59423,59446,59730,59846,59869,60065) | 4 | OK |
| 36 | Entity(tower/minion/enemy/e) | 0x4c0 | attack_effect@tag (니치) | r | -1=None → unwrap_failed(L326 m07:58696~58703 · L405 m07:59377~59380,59419 · L439 m07:59817~59820,59852) / L467 others 는 None 이면 skip(m07:59692~59695) | 4 | OK |
| 37 | Entity(tower/minion/enemy/e) | 0x490 | attack_effect@Some.0 (&Effect) | r | is_in_range self(m07:58710~58712) · range_adjust self(m07:59711,59838) | 4 | OK |
| 38 | Entity(minion/enemy/e) | 0x4a0 | attack_effect@Some.0.range | r | Effect::range(caster) 인라인 effect.rs:26 (m07:59401,59701,59826) | 4 | OK |
| 39 | Entity(minion/enemy/e) | 0x4a8 | attack_effect@Some.0.growth_range | r | *(level-1) (m07:59403,59703,59828) | 4 | OK |
| 40 | Entity(minion/enemy/e) | 0x5c8 | level | r | (m07:59405,59705,59830) · L449 level>2 → skill2 가용(m07:59905) | 4 | OK |
| 41 | Entity(minion/enemy/e) | 0x438 | stat_buff_cached.range | r | (m07:59409,59709,59834) | 4 | OK |
| 42 | Entity(minion/enemy/e) | 0x640 | stat_cached.move_speed | r | L424 minion(0→panic div0 m07:59623~59636) · L457 enemy(0→panic m07:59970~59973,60026) · L470 others max(,1)(m07:59771~59775) | 4 | OK |
| 43 | Entity(tower) | 0x670 | hp | r | L410 tower.hp*100/stat_cached.hp > 9 (m07:59546~59550) | 4 | OK |
| 44 | Entity(tower) | 0x628 | stat_cached.hp (max) | r | 0 → panic div0 (m07:59540~59543,59555) | 4 | OK |
| 45 | Entity(enemy) | 0x4f8 | skill_effect@tag (니치) | r | -1 이면 스킬 없음 (m07:59889~59892, L443) | 4 | OK |
| 46 | Entity(enemy) | 0x4f0 | skill_effect@Some.0.target (CastingTarget) | r | CastingTarget::check(&target, enemy, champ) (m07:59897~59898, L444) | 4 | OK |
| 47 | Entity(enemy) | 0x4c8 | skill_effect@Some.0 (&Effect) | r | range_adjust self (m07:59888,59920) · +0x4d8 range(59915) · +0x4e0 growth_range(59917) | 4 | OK |
| 48 | Entity(enemy) | 0x500 | skill2_effect@Some.0 (&Effect) | r | level>2 일 때만; 아니면 정적 None(@anon.31, 태그 -1) (m07:59905~59907, entity.rs:1693 skill2_effect 인라인) · +0x28 target → check(59959~59960) · +0x30 tag(59909~59911) · +0x10 range/+0x18 growth(59976~59979) | 4 | OK |
| 49 | Projectile(p) | 0x0 | team@tag | r | L352 팀 비교: 태그 다르거나(태그==0 이고 .0 다르면) 진행, 같으면 skip (m07:59288~59294, 60030~60031, 60045~60048) | 4 | OK |
| 50 | Projectile(p) | 0x8 | team@Player.0 | r | (m07:59289,60045) | 4 | OK |
| 51 | Projectile(p) | 0x12c | applyed_target (CastingTarget) | r | CastingTarget::check_projectile(&self, p, champ) (m07:60034~60035, L352) | 4 | OK |
| 52 | Projectile(p) | 0x131 | is_visible | r | false 면 skip (m07:60051~60054, L355) | 4 | OK |
| 53 | Projectile(p) | 0x40 | move_type@tag (니치, 2..11) | r | 3=Delayed → L376 / 4=Periodic → L363 / 그 외 skip (m07:60083~60093, L361) | 4 | OK |
| 54 | Projectile(p) | 0x48 | move_type@Delayed.tick | r | (m07:60125~60127) | 4 | OK |
| 55 | Projectile(p) | 0x50 | move_type@Delayed.applyed | r | applyed<=tick 이면 hit_in=applyed (m07:60123~60133, L376) | 4 | OK |
| 56 | Projectile(p) | 0x58 | move_type@Delayed.apply_done | r | true 면 skip (m07:60099~60102) | 4 | OK |
| 57 | Projectile(p) | 0x60 | move_type@Periodic.tick | r | next_in > tick 이면 skip (m07:60176~60180, L373) | 4 | OK |
| 58 | Projectile(p) | 0x68 | move_type@Periodic.period | r | rp=max(period,1) (m07:60109~60113, L363) | 4 | OK |
| 59 | Projectile(p) | 0x70 | move_type@Periodic.cool | r | (m07:60115~60116, L364~371) | 4 | OK |
| 60 | Projectile(p) | 0x78 | move_type@Periodic.first_delay | r | (m07:60117~60118) | 4 | OK |

**`consts` 상수 16건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 305 | 임계 | info.team bounds-check(len 2) (m07:58444,58448) — 판정값 아님 | 4 |
| 1 | 4 | 307 | 태그 | EntityType 태그 4=Jungle (m07:58522) — jungles 순회 중 정글몹 arm | 4 |
| 2 | 5 | 307 | 태그 | EntityType 태그 5=Epic (m07:58523) — 에픽 arm(같은 focused 검사) | 4 |
| 3 | 0 | 319 | 태그 | GameMode 태그 0=Moba (`icmp eq %71, 0` m07:58574) — 모바 모드가 아니면 세르펜 검사 skip · L352 TeamType 태그 0=Player(m07:60030) | 4 |
| 4 | 14400000001 | 320 | 임계 | ★120000²+1 — 세르펜과 제곱거리 < 이 값(=거리 ≤120000, 3.75셀) 이면 false (m07:58648) | 4 |
| 5 | 1 | 325 | 태그 | `1 - team` 적팀 인덱스(m07:58653) · L350 `version >u 1`(m07:58772) · L335 ChampionActionState 태그 1=Return(m07:58757) · umax 바닥(L392 m07:59352, L363 m07:60113, L470 m07:59775) | 4 |
| 6 | 13 | 334 | 태그 | EntityType 태그 13=Champion — 내 엔티티가 챔피언이고 Return 상태면 recall_time 을 남은 시간으로 (m07:58750) | 4 |
| 7 | 3 | 389 | 태그 | CastedType 태그 3=Heal — 힐 도트는 위협 아님, skip (m07:59333). (본문의 `shl nuw nsw i64 %len, 3`(m07:58491,59670) 은 &Entity 슬라이스 바이트 stride(×8) 이지 이 상수와 무관 — 접힘 아님) | 4 |
| 8 | 2 | 361 | 센티널 | ProjectileMoveType 니치 태그 환산 `tag-2`(niche_start=2): 결과 1=Delayed(태그3) / 2=Periodic(태그4) (m07:60087~60093) · L449 `level >u 2` skill2 가용(m07:59905) | 4 |
| 9 | 7 | 361 | 산출값 | switch 기본값(tag<=1 이면 7 → default skip) (m07:60089) — 판정값 아님 | 4 |
| 10 | 9 | 361 | 임계 | `assume tag != 9`(BouncingTarget 은 untagged 라 9 는 미사용) (m07:60085~60086) · L410 `hp*100/max_hp >u 9` = 타워 HP 10% 이상(m07:59550) | 4 |
| 11 | 100 | 410 | 계수 | 타워 HP 백분율 `hp*100/stat_cached.hp`(m07:59548) · Entity::radius 인라인 `radius*(mult+100)/100` 도처 | 4 |
| 12 | 30 | 410 | 산출값 | ★타워가 미니언보다 나에게 가깝고 타워 HP ≤9% 일 때 bonus_time=30틱; HP>9% 면 bonus_time=return_tick(사실상 무한) (m07:59551 select) | 4 |
| 13 | 20 | 419 | 계수 | ★아군 미니언이 적 미니언에 나보다 가깝거나 같으면 bonus_time += 20틱 (m07:59616~59617) | 4 |
| 14 | 5 | 433 | 태그 | player_champion[1-team][0..5] 슬롯 수 (m07:59800) — 판정값 아님 | 4 |
| 15 | -1 | 326 | 센티널 | Option<Effect> None 니치(attack_effect/skill_effect/skill2 +0x30) · Chain 소진 태그(aux m06:24813) · level-1 산술(m07:59407 등) | 4 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 세르펜 근접 임계(제곱) | cast.rs:320 (m07:58648) | 14400000001 | =120000². 올리면 세르펜에서 더 멀어도 귀환 불가로 판정(보수적). 내리면 세르펜 옆에서도 귀환 | 4 | 기존 |
| 1 | 타워 보호 HP 임계(%) | cast.rs:410 (m07:59550) | 9 | 타워 HP>9% 면 미니언 도달시간에 return_tick 을 통째로 더해 사실상 무시. 올리면(예 30) 저체력 타워 뒤 귀환이 덜 안전 판정 | 4 | 기존 |
| 2 | 저체력 타워 보너스 틱 | cast.rs:410 (m07:59551) | 30 | 타워 HP≤9% 일 때 미니언이 타워를 뚫는 데 드는 가정 틱. 올리면 귀환 허용↑ | 4 | 기존 |
| 3 | 아군 미니언 차단 보너스 틱 | cast.rs:419 (m07:59616) | 20 | 아군 미니언이 적 미니언과 나 사이에 있으면 +20틱 여유. 올리면 미니언 라인 뒤 귀환 허용↑ | 4 | 기존 |
| 4 | 스킬2 가용 레벨 | entity.rs:1693 skill2_effect 인라인 (m07:59905) | 2 | level>2 부터 적 스킬2 사거리를 위협 반경에 포함 — game_core 규칙, 노브 아님 | 4 | 기존 |
| 5 | 귀환 채널링 시간 | GameSetting+0x1368 return_tick (m07:58744) | 설정값 | 모든 '도달시간 < recall_time' 비교의 기준. 설정 필드(코드 상수 아님) | 4 | 기존 |
| 6 | 버전 게이트 | cast.rs:350 (m07:58772) | 1 | version>1 에서만 투사체·DoT 검사. 내리면 v0/v1 도 검사 | 4 | 기존 |

<details><summary>`callees` 피호출자 22건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check_projectile | game_core::CastingTarget::check_projectile | pub | fn(&game_core::CastingTarget, &game_core::Projectile, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:248 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | is_in_orbit | game_core::Projectile::is_in_orbit | pub | fn(&game_core::Projectile, u64, u64, u64) -> bool | game-core\src\simulation\projectile.rs:973 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | is_recent_visible_big_action | game_core::Blackboard::is_recent_visible_big_action | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:367 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | is_safe_recall | game_ai::small_action::cast::is_safe_recall | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> bool | game-ai\src\small_action\cast.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1841 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 11개**: `dist2`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `first`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `llvm.umax.i64`, `llvm.usub.sat.i64`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `null`, `recall_time`, `then_some`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 8곳** (m08.ll:93989, m08.ll:99365, m08.ll:99998, m08.ll:102953, m08.ll:104368, m08.ll:111035, m11.ll:46536, m15.ll:56937) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L376 Delayed 경로의 hit_in = `applyed`(+0x50) 인데 조건은 `applyed <= tick`(+0x48) 이다 — 필드 의미(tick 이 경과인지 총지연인지)에 따라 '남은 틱'이 아니라 '경과 틱'일 수 있음. IR 관측만 기록, Projectile 틱 갱신(game_core projectile.rs) 미독해 | 4 |  |
| 1 | 미탐색 | L352 팀 비교는 TeamType PartialEq 인라인(entity.rs:1127) — 태그≠ 또는 (Player 이고 .0≠) 이면 진행. Neutral 투사체는 champ(Player) 와 항상 진행 | 4 |  |
| 2 | 미탐색 | 경계 콜리 내부 미독해(시그니처만): towers_without_nexus(&cache,team,&Bump)->bumpalo Vec<&Entity>(32B sret) · iter_towers_without_nexus(&cache,team)->Chain(120B) · iter_minions(&cache,team)->Chain(56B) · Effect::is_in_range(&Effect,&Entity,&Entity)->bool · Effect::range_adjust(&Effect,&Entity,&Entity)->usize · Entity::distance(&Entity,&Entity)->usize · CastingTarget::check(&CastingTarget,caster,target)->bool · check_projectile(&CastingTarget,&Projectile,&Entity)->bool · Projectile::is_in_orbit(&Projectile,x,y,r)->bool · vtable AbstractGame get_game_mode(0x40)->GameMode{tag,ptr} · get_entity_by_id(0x1f0)->Option<&Entity> · iter_projectile(0x210)->ProjectileIter(40B sret) | 4 |  |
| 3 | 미탐색 | L400 min_by_key 순회 순서(Chain: 고정 6슬롯 → 슬라이스 / 3 슬라이스)는 동률 시 어느 타워/미니언이 뽑히는지에 영향 — 배열 순서는 game_core 미독해 | 4 |  |
| 4 | 미탐색 | L305 `.unwrap()`·L326/L405/L439 attack_effect `.unwrap()`·L424/L457 `/ move_speed`(0 → div0 panic) 는 실전 도달 여부 미확인(추정: 도달 안 함) | 5 |  |
| 5 | 미탐색 | `_positioning_score`(2760B) 는 본문에서 완전 미사용(IR 참조 0회) — 시그니처 호환용 인자로 추정 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L435 블랙보드 인덱스가 `1-team`(적 팀) 인 이유 — IR 확정(m07:58653→59393) 이지만 Blackboard 가 '관측 대상 팀' 키인지 '소유 팀' 키인지는 Blackboard 구조 미독해. is_recent_visible_big_action(&Blackboard,&dyn AbstractGame,&PlayerState,&Entity)->bool 내부(g07.ll:157514) 미독해 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

