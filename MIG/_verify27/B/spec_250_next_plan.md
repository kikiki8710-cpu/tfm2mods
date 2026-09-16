---

### `250` next_plan — 갱 커버 플랜의 BigPlan 전이: 내 챔프가 목표 부시(target_bush_v30)에 서 있을 때만(champ_bush==bush) 근처(150k) 적·아군 인덱스, 최근접 적 타워, 양측 die_tick, 타워 이동 틱을 재어 Battle(태그 9)로 전이(Some) 또는 None(유지); 부시 미도착이면 즉시 None. 배치 G = 머리(L34~65: target_bush 인라인 사장값·부시 판정·근처 목록·최근접 적 타워) / 배치 H = L68~132 (die_tick·타워 이동·Battle 생성 판정).

| 항목 | 값 |
|---|---|
| id | `LineGankCoverPlan__next_plan` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank5coverNtB2_17LineGankCoverPlan9next_plan` |
| 소스 | `game-ai\src\plan_legacy\old\line_gank\cover.rs:34` |
| IR | `m10.ll` 11810~13615행 |
| 경로·가시성 | `game_ai::plan_legacy::old::LineGankCoverPlan::next_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `df1f50` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::old::LineGankCoverPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[250]/sig/tls/<키>`)**

- `name`: (없음)
- `role`: 본 함수 본문(m10.ll 11810~13615)에 TLS 접점 0 — `LocalKey`/`threadlocal` 문자열 0건, @anon 참조 12종은 전부 panic Location(.83~.88·.96~.100)·format 문자열(.89·.94·.95·.101…)이고 `constant ptr @…call_once` fn-포인터 상수 아님. 본 범위 aux 9조각도 동일(closure call_mut·from_iter_in·fold). ⚠콜리 get_die_tick_player(배치 H)·target_bush_v30(anon .127~.129·.88~.91 = Location/문자열, call_once 0) 내부 TLS 는 각 자식 명세/후속 확인 사항.
- `key`: -
- `layout`: -
- `invalidation`: -
- `call_conditions`: -

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<BigPlan> (384B) | 반환값 슬롯 · 본 범위 쓰기 = L49 `store i64 -1`(None) 1곳(m10.ll:12448) · (배치 H) (배치 H) 이 범위의 쓰기 3종: None = +0 i64 -1 (L131 · 13446) / Some(Battle) = +0 i64 9 (BigPlan::Battle 메모리태그 · 13317·13570) + +8..+0x120 BattlePlan 280B memcpy(13313·13344·13569). +0x120..+0x180 는 미기록 | 4 |
| 1 | 1 | self | &mut LineGankCoverPlan (40B) | IR 속성상 가변 · 동작상 읽기만: 본 범위 읽기 = +0x20 line (m10.ll:11919 L198 인라인, 12291 L38) · 클로저 env 에 &self 저장(11978, target_bush 키가 self.line 만 읽음) · 함수 전체(11810~13615) %1 파생 포인터 store 0건(grep: %62/%384 는 load 만 · 배치 H 는 +0x18 wait_limit 읽기 12717) ⟹ writes 표면 없음 · (배치 H) (배치 H) 읽기 = +0x18 wait_limit (L110 · 13206). 이 범위에 self 쓰기 0 (전 함수 grep 으로도 `store …, ptr %1\|%62\|%384` 0건) | 4 |
| 2 | 2 | version | usize | 본 범위 분기 0 · 함수 전체에서도 분기 없이 BattlePlan::new 인자로만 전달 ×3(m10.ll:13229·13281·13561, 배치 H) · (배치 H) (배치 H) 분기 없음 · BattlePlan::new 1번째 인자로 3회 전달(13229·13281·13561) | 4 |
| 3 | 3 | _rnd | &mut StdRng (320B) | 미사용(readnone) · gen_range 호출 사이트 0 · (배치 H) (배치 H) 미사용 · gen_range 호출 0 | 4 |
| 4 | 4 | player | &PlayerState (2528B) | 읽기: +0x930 info.team · +0x9c0 info.position@tag · 클로저 env(closure#2·target_bush 키)로 전달 · 콜리 인자(get_die_tick_player·is_recent_visible·BattlePlan::new) · (배치 H) (배치 H) team(+0x930)·position(+0x9c0 tag) 은 배치 G 진입부에서 로드된 %48/%54 를 사용 · get_die_tick_player/BattlePlan::new/is_recent_visible/클로저에 전달 | 4 |
| 5 | 5 | data | &OperationData (24B) | 읽기: +0 cache · +8 context · +0x10 blackboard · (배치 H) (배치 H) cache(%55)·context(%83)·blackboard(%141) 포인터는 배치 G 에서 로드 · 이 범위에서 context.setting(+0x8) 을 다시 로드(13235) | 4 |
| 6 | 6 | _positioning_score | &PositioningScoreData (2760B) | IR 속성 readonly 이나 본문 사용 0건(define 줄 외 %6 등장 없음 · 11810~13615 grep) · (배치 H) (배치 H) 미사용 | 4 |
| 7 | 7 | _team_plan | &TeamPlan | 미사용(readnone) · (배치 H) (배치 H) 미사용 | 4 |
| 8 | 8 | debug | &mut DebugFrameData (224B) | 본 범위: L45 add_text(&mut) 1곳(12369, context.debug 참일 때만) · 배치 H: +0xa0 접근(12698) · (배치 H) (배치 H) ctx.debug 일 때만 infos(+0xa0)[me.id].push(format!(..)) (L94~95 · 13032~13182) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// cover.rs:0~65 (배치 G)
// fn next_plan(&mut self, version, _rnd, player, data, _positioning_score, _team_plan, debug) -> Option<BigPlan>   [cover.rs:34~35 · m10.ll:11810]
//
// ── L36: self.target_bush(player, data) 197줄 인라인 (cover.rs:196~222, 반환 (u64,u64)) ── ★반환값 사용처 0 = 사장값. 남은 효과는 unwrap/bounds 패닉 4곳뿐(정상 데이터면 무해)
team = player.info.team (+0x930);  if team >= 2 → panic_bounds_check(cover.rs:197:17)            [11893~11899]
pos  = player.info.position@tag (+0x9c0) as usize
champ = cache.player_champion[team][pos] (+0x1e0);  None → unwrap_failed(197:95)                    [11907~11913, 12062]
line = self.line (+0x20)                                                                                 [11918 L198]
tower  = cache.<line>_tower[team]  (0x180 + line*32 + team*8)                                             [11924~11929]
tower2 = cache.<line>_tower2[team] (0x190 + line*32 + team*8)                                             [11930~11932]
optb = tower.or(tower2)   (select tower==null ? tower2 : tower)                                          [11933~11934 L199]
twins = cache.twin_towers[team] (+0x130: ptr, len)                                                       [11939~11946]
nearest_twin = twins.iter().min_by_key(|t| dist_sq(self.line.get_start_position(context.setting, team), (t.x, t.y)))   [11995~12059 L200~202 · 첫 원소 인라인 12021~12053 · 나머지 fold aux m12.ll:29368 · get_start_position 은 원소마다 호출]
   twins.len == 0 → nearest_twin = None                                                                 [11995 → %92]
nearest_tower = optb.or(nearest_twin)                                                                    [12065~12084 L204]
nexus = cache.nexus[team] (+0x170)                                                                       [12090~12092 L205 · 12105~12107]
if nearest_tower.is_none() { nearest_tower = nexus.unwrap() (206:8) }                                   [12096~12100 L206]
nexus = nexus.unwrap() (208:52)                                                                          [12113~12117 L208]
ms = match line { Top → blackboard[team].top_minion_state(+0) · Mid → mid(+0x28) · Bottom → bottom(+0x50) }   [12124~12143 L210 · switch 12128]
front_minion: Option<&Entity> = ms.front_minion(+0 태그, +8 id).and_then(|id| cache.game.get_entity_by_id(id))   [12149~12181 L211 · vtable+0x1f0 간접호출 12174]
// L213 filter — 극성: dist_sq(m, nexus) < dist_sq(nearest_tower, nexus) 이면(미니언이 타워보다 넥서스에 가까움 = 우리 타워 안쪽) 탈락
front = front_minion.filter(|m| dist_sq(m, nexus) >= dist_sq(nearest_tower, nexus))                       [12186~12247 · icmp ult 12246 → 참이면 %198(탈락)]
center = if let Some(m) = front { (m.x, m.y) }                                                            [12270~12274 L218]
         else { context.map.lane_path(line, team)[3]  (sret 112B, +48/+56) }                              [12251~12259 L214]
_ = near_jungle_bush(context, champ.x, champ.y, center.x, center.y).unwrap() (219:7)                     [12265/12274 L215·L218 · 12278~12283 L219 · 값 미사용]
//
// ── L38 ──
bush = self.target_bush_v30(player, data)   // usize 2..=21 · 인자 승격 (line, team, pos, cache, context) · 본체는 계약만   [12291~12293]
// ── L40 ──
champ = cache.player_champion[team][pos].unwrap()  (40:95 · 같은 슬롯 재로드)                              [12294~12297, 12331]
// ── L42 ──
champ_bush = context.map.bushes[(champ.y / 32000).clamp(0, 29)][(champ.x / 32000).clamp(0, 29)]   (+0x1c98 · [30][30] usize · 행=y셀, 열=x셀)   [12304~12324]
// ── L44~46 ──
if context.debug (+0x3b) {                                                                                [12325~12328]
   debug.add_text(champ.x, champ.y + 20000, format!("bush: {:?}, champ_bush: {:?}", bush, champ_bush), Color{r:1.0,g:0.0,b:0.0,a:1.0}, 1)   [12341~12369 L45 · anon.94]
}
// ── L48~50 ── ★부시 도착 게이트
if champ_bush != bush { return None }   // sret+0 = -1                                                     [12335~12338 L48 · 12448 L49]
// ── L52 ──
dist_cut = 150000                                                                                         [12377]
// ── L54~57 ── (결과 사용처 0 · 드롭만 · 자기 자신 포함 — 제외 조건 없음)
_near_allies_p: bumpalo::Vec<usize> = from_iter_in(
   cache.player_champion[team].iter().enumerate()
     .filter(|(_, c)| c.is_some_and(|x| x.distance_sq(champ) <= dist_cut * dist_cut))   // closure#0 aux m10.ll:54512 · is_some_and(option.rs 659~661) · distance_sq(entity.rs:2158)
     .map(|(i, _)| i),                                                                  // closure#1 인라인(from_iter_in 안에서 enumerate 카운트 push · m01.ll:52590)
   context.pool)                                                                                          [12386~12402 L54~57]
// ── L59~62 ──
near_enemies_p: bumpalo::Vec<usize> = from_iter_in(
   cache.player_champion[1 - team].iter().enumerate()                                                    [12406~12414 L59]
     .filter(|(_, c)| c.is_some_and(|x| blackboard[1 - team].is_recent_visible(cache.game, player, x)   // closure#2 aux m10.ll:54582 · 순서: 가시성 먼저, 거짓이면 거리 미평가(54635 → %51 false)
                                        && x.distance_sq(champ) <= dist_cut * dist_cut))                  //   (1-team) bounds<2 검사 있음(54629~54638)
     .map(|(i, _)| i),                                                                                   // closure#3 인라인
   context.pool)                                                                                          [12415~12445 L60~62 · invoke → unwind 시 _near_allies_p drop(12451~12454)]
// ── L64~65 ──
nearest_enemy_tower: Option<&Entity> = cache.iter_towers_without_nexus(1 - team)                          [12462 L64 · sret 120B]
   .min_by_key(|t| t.distance_sq(champ))                                                                  // closure#4 · 첫 원소 인라인 12490~12652(6칸 Option 배열 순회 → 없으면 twin 슬라이스 Copied::next 12588) · 나머지 fold aux m06/m11/m12 · 동률 = 먼저 나온 것(compare<=0 유지)
   // 순회 순서 = [top_tower, mid_tower, bottom_tower, top_tower2, mid_tower2, bottom_tower2][1-team] → twin_towers[1-team] (g15.ll:108971)
   // 적 타워 0개 → None (%345 = null)                                                                    [12661~12662]
→ 배치 H (cover.rs:68 · m10.ll:12668~) : near_enemies_p 순회·die_tick·타워 이동 틱·Battle 전이 판정
// 본 범위 콜리 순서(분기 사슬): panic_bounds_check?(197) → unwrap?(197) → [twins 비어있지 않으면 get_start_position×len + Map::fold] → unwrap?(206|208) → get_entity_by_id(211, front_minion Some 일 때) → lane_path(214, front None 일 때) → near_jungle_bush(215|218) → unwrap?(219) → target_bush_v30(38) → unwrap?(40) → [debug: format_inner + add_text](45) → [None 반환](49) → from_iter_in(54) → from_iter_in(59) → iter_towers_without_nexus(64) → [Copied::next | Map<Chain>::fold](65)
// rnd: gen_range 호출 사이트 0 (_rnd readnone)

// cover.rs:68~132 (배치 H)
// 진입 전제(배치 G 산출): me=%221 (cache.player_champion[team][pos].unwrap) · dist_cut=%41=150000(L52) · near_enemies_p=%38 (L59~62 · 적 포지션 인덱스 Vec<usize>) · nearest_enemy_tower=%345 (L65 phi 12662 · None 가능) · 적 슬라이스 %264=&cache.player_champion[1-team] · 아군 슬라이스 %57=&cache.player_champion[team]
// %344(12661~12719): iter = near_enemies_p.ptr .. ptr+len (12668~12680) · %363 = (nearest_enemy_tower == None) (12696)

// ---- L68 for e in near_enemies_p.iter() { ---- (루프 머리 %386 12721~12731 · e=&usize %387)
//   L70~75 near_allies_p: bumpalo::Vec<usize> = cache.player_champion[player.team].iter().enumerate()
//        .filter(|(_, c)| c.is_some() && (c.id == me.id || dist²(c, E) <= dist_cut²))   // aux m10.ll 54701~54808 · E = cache.player_champion[1-player.team][*e].unwrap() (54739~54761 · bounds/unwrap 패닉 가능)
//        .map(|(i, _)| i).collect_in(bump=ctx.pool)                                        // aux m01.ll 52794~52972 · 12752~12760 invoke from_iter_in(sret %35)
//   L77~80 near_enemies_p(루프 내부 · 바깥 %38 과 동명 · %33) = cache.player_champion[1-player.team].iter().enumerate()
//        .filter(|(_, c)| c.is_some() && dist²(c, E) <= dist_cut²)                        // aux m10.ll 54828~54923 · 자기 자신(E) 포함
//        .map(|(i, _)| i).collect_in(..)                                                  // 12939~12946 (sret %33)
//   L82 enemy_die_tick = get_die_tick_player(data, 1 - player.team, *e, &near_allies_p, None, &None)   // 12957~12961 · action=Option<SmallAction>::None(store i64 -1 12958 → memcpy 24B)
//   L83 me_die_tick    = get_die_tick_player(data, player.team, player.info.position, &near_enemies_p, None, &None)   // 12973 · %54 = PlayerState+0x9c0 tag
//   L85 let e = cache.player_champion[1 - player.team][*e].unwrap()   // 12978 bounds(e<5) · 12983 로드 · 12985 null→unwrap_failed(.98 Location 85:68)
//   L86 move_to_tower_tick = if let Some(tower) = nearest_enemy_tower {   // 12995 br %363
//   L87     let dist = e.distance(tower)                                   // 13003 Entity::distance(e, tower)
//   L88     let move_speed = e.stat_cached.move_speed (+0x640)             // 13008~13009
//   L89     dist / move_speed                                              // 13011 ==0 → panic_const_div_by_zero(Location 89:9) · 13015 udiv
//   L91 } else { 99999999 }                                                // phi 13023 · store %28 13024
//   L94 if ctx.debug {                                                     // 13025 br %240 (GameContext+0x3b)
//   L95     debug.infos.entry(me.id).or_default().push(format!("target: {:?}, near_allies_p: {:?}, near_enemies_p: {:?}, move_to_tower_tick: {}, me_die_tick: {}, enemy_die_tick: {}", e.id, near_allies_p, near_enemies_p, move_to_tower_tick, me_die_tick, enemy_die_tick))
//         // 13034 key=me.id(+0x5c0) · 13037 rustc_entry(debug+0xa0) · 13041 null=Vacant → 13053~13071 빈 Vec insert_no_grow / Occupied 13079 · 13122~13141 format_inner(@anon.95) · 13151~13181 len==cap→grow_one, memcpy 24B, len+1
//   L97 }
//   L100 if enemy_die_tick > 30 && near_allies_p.len() <= near_enemies_p.len() { continue }   // 13028~13030 ugt 30 → %501: 13191~13197 (allies.len <= enemies.len) || (mtt<241) → %573(continue) · 줄 길이 산술 L100=77자 = 6+71 일치
//   L104 if move_to_tower_tick <= 240 { continue }                        // 13185~13187 `ult 241` → %573 · (enemy_die_tick<=30 경로 %498 도 동일 검사)
//   L108 if me_die_tick > 120 && near_allies_p.len() >= near_enemies_p.len() {   // 13201~13203 ugt 120 → %515: 13214~13218 (allies < enemies) 이면 else 로
//   L109     let mut b = BattlePlan::new(version, BattlePlanGoal::TryKill(e.id, 60), data, player); b.entry_src = 8; return Some(BigPlan::Battle(b))   // 13222~13229 goal{0,e.id,60} · invoke new(sret %24) · 13341~13344 +0x107=8, memcpy sret+8 · 13317 sret+0=9
//   L110 } else if self.wait_limit.saturating_sub(game.tick())              // 13206 self+0x18 · 13208~13209 vtable+0x28 tick(%266) · 13234 usub.sat
//   L111        <= ctx.setting.tick_per_second * 3                          // 13235~13239: (remain > tps*3) 이면 %535 = continue (드롭 후 %550→%386)
//   L112        && near_allies_p.len() > near_enemies_p.len() {             // 13244~13248 ugt → %540 / 아니면 %535 continue · 줄 길이 산술 L111=53자·L112=54자 일치
//   L113     let mut b = BattlePlan::new(version, TryKill(e.id, 60), data, player); b.entry_src = 8; return Some(BigPlan::Battle(b))   // 13274~13281 · 13310~13313 · 13317
//   L115 } (drop near_enemies_p, near_allies_p: 13250~13302 / 13390~13430 → %550 → 다음 e)
// L116 }

// ---- 루프 소진(%391 12763~12828) ----
// L119 nearest_enemy: Option<&Entity> = cache.iter_champions(1 - player.team)   // FilterMap(Option→&Entity · iter_champions0) over %264 5칸
//        .filter(|c| data.blackboard[1 - player.team].is_recent_visible(game, player, c))   // 본체 12832~12870: 첫 통과 원소 탐색(12855 invoke · %398 = blackboard[%263=1-team]) · 나머지는 aux m12.ll 14358~14398
//        .min_by_key(|c| dist²(c, me))                                         // 첫 원소 키 = (c.x-me.x)²+(c.y-me.y)² 12891~12917 → 12920 fold(첫 (key,ptr), 잔여 iter) · aux m12.ll 14415~14461: 새 key < 현재 key 일 때만 교체(cmp<1 → 유지 · 동률은 앞 원소)
//        // 통과 원소 0 → %411 nearest_enemy=None (12872~12875) → L131
// L121 let Some(nearest_enemy) = nearest_enemy else { → L131 None }             // 13433~13437 (fold 반환 { i64 key, ptr } 의 ptr null 검사)
// L122 let dist = nearest_enemy.distance(me)                                 // 13442 Entity::distance(nearest_enemy, me)
// L123 let range = { let ef = me.attack_effect.as_ref().unwrap();            // 13468~13470 tag(+0x4c0)==-1 → unwrap_failed(.100 Location 123:57)
//          ef.range(me) [= ef.range + ef.growth_range*(me.level-1) + me.stat_buff_cached.range  · effect.rs:26 인라인 · 13477~13484·13495~13496·13543~13544]
//        + ef.range_adjust(me, nearest_enemy)                                  // 13486 Effect::range_adjust(&ef=+0x490, me, nearest_enemy) → 13545
//        + me.radius() + nearest_enemy.radius() }                              // entity.rs:1511~1515 인라인: radius_mult==0 ? radius : radius*(radius_mult+100)/100 · 13497~13519 / 13520~13542 · 합 13546
// L126 if dist > range { → L131 None }                                      // 13549 `icmp ugt %587, %646` → %588
// L127 let mut b = BattlePlan::new(version, TryKill(nearest_enemy.id, 60), data, player); b.entry_src = 8; return Some(BigPlan::Battle(b))   // 13554~13561 · 13566~13570 (sret+8 memcpy, +0=9)
// L131 None                                                                    // 13446 store i64 -1 → sret+0
// L132 } (drop near_enemies_p %38, _near_allies_p %40 · 13448~13463 · 13371~13388 · 13573~13609 · ret 13614)

// 분기 사슬 요약(적 e 하나당): [30틱 후 사망 & 아군<=적 → skip] → [타워복귀 <=240틱 → skip] → [me_die_tick>120 & 아군>=적 → Battle(e)] → [잔여대기 <= tps*3 & 아군>적 → Battle(e)] → skip. 전부 skip 이면 최근접 가시 적이 평타 사거리 안 → Battle(nearest), 아니면 None.
// rnd: gen_range 호출 0 (_rnd readnone)
// 사장 코드: reach.py 사장 호출부 0건 · NA 봉인 없음
```

**`mem` 메모리 접근 48건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | team(usize) · L197(인라인) bounds<2 · 12352 로 표기됨 \| (배치 H) 클로저 aux(54739·54858·14382) 에서 재로드 · 1-team 으로 적 슬라이스 색인 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 태그 → zext usize = 슬롯 인덱스 pos (2496) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 4 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] (744B stride) · L210 [team] · closure#2 [1-team] | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x0 | game (data_ptr) | r | &dyn AbstractGame 팻포인터 앞 절반 · L211 get_entity_by_id self · closure#2 is_recent_visible 인자 | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x8 | game (vtable_ptr) | r | 팻포인터 뒤 절반 · vtable+0x1f0(496) 슬롯 = get_entity_by_id (divtable AbstractGame 0x1f0) | 3 | OK |  |
| 7 | AbstractGameWithCache | 0x130 | twin_towers[team] | r | bumpalo Vec<&Entity> 32B stride(304+team*32) · +0 ptr · +0x18(24) len · L199~202 최근접 쌍둥이타워 탐색 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x170 | nexus[team] | r | Option<&Entity> (368+team*8) · L205/206/208 unwrap | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x180 | top_tower[team] (line=Top) / +line*32 → mid_tower 0x1a0 · bottom_tower 0x1c0 | r | L198: 384 + line<<5 + team*8 (shl 5 = LineType 별 32B 스트라이드) | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x190 | top_tower2[team] (line=Top) / +line*32 → mid_tower2 0x1b0 · bottom_tower2 0x1d0 | r | L198: 400 + line<<5 + team*8 · tower 가 None 일 때 대체(Option::or) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [[Option<&Entity>;5];2] · 480 + team*40 + pos*8 · L197(인라인)·L40 unwrap · L54 [team] 슬라이스 · L59 [1-team] 슬라이스 \| (배치 H) 아군 슬라이스 %57=[player.team] · 적 슬라이스 %264=[1-team] (둘 다 배치 G 계산) · L85 [e] 색인(12982) · L119 5칸 순회(12832) · 클로저 aux 안에서도 +480 으로 직접 색인 | 4 | OK |  |
| 12 | GameContext | 0x0 | pool | r | &Bump · from_iter_in bump 인자 ×2 (12401) | 4 | OK |  |
| 13 | GameContext | 0x8 | setting | r | &GameSetting · get_start_position 인자 \| (배치 H) L111 · 13235 (ctx=%83 은 배치 G 로드) | 4 | OK |  |
| 14 | GameContext | 0x20 | map | r | &MapDef · L214 lane_path · L42 bushes | 4 | OK |  |
| 15 | GameContext | 0x3b | debug | r | bool(59) · L44 디버그 텍스트 게이트 · (배치 H L94 재사용) \| (배치 H) L94 분기 %240 (로드 자체는 배치 G 12326) | 4 | OK |  |
| 16 | MapDef | 0x1c98 | bushes[y_cell][x_cell] | r | [[usize;30];30] (7320) · 행=champ.y/32000 · 열=champ.x/32000 · 각 clamp(0,29) | 4 | OK |  |
| 17 | Blackboard | 0x0 | top_minion_state (line=Top) / 0x28 mid_minion_state / 0x50 bottom_minion_state | r | BrainMinionParameter 40B · switch line {0→+0, 1→+40, 2→+80} (12128) | 4 | OK |  |
| 18 | BrainMinionParameter | 0x0 | front_minion@tag | r | Option<usize> 태그(i64, trunc→i1) | 4 | OK |  |
| 19 | BrainMinionParameter | 0x8 | front_minion@Some.0 | r | 엔티티 id → get_entity_by_id | 4 | OK |  |
| 20 | Entity | 0x660 | x | r | 1632 · champ/타워/미니언/넥서스 좌표 \| (배치 H) L119 dist² 키(12891) · aux 클로저 dist² | 4 | OK |  |
| 21 | Entity | 0x668 | y | r | 1640 \| (배치 H) L119 dist² 키(12895) · aux 클로저 dist² | 4 | OK |  |
| 22 | LineGankCoverPlan | 0x20 | line | r | LineType 태그 1B (Top0/Mid1/Bottom2) · L198·L38 | 4 | OK |  |
| 23 | LineGankCoverPlan | 0x18 | wait_limit | r | L110 · 13206 · saturating_sub(game.tick()) 의 피감수 | 4 | OK |  |
| 24 | GameSetting | 0x12f8 | tick_per_second | r | L111 · 13237 · ×3 과 비교 | 4 | OK |  |
| 25 | DebugFrameData | 0xa0 | infos | r | L95 · HashMap<usize,Vec<String>> entry(me.id) | 4 | OK |  |
| 26 | AbstractGameWithCache | 0x0 | game (dyn 데이터ptr %266 / vtable %268) | r | 로드는 배치 G(12415·12417) · 이 범위에서 vtable+0x28 tick 호출(13208~13209) 및 is_recent_visible 인자 | 4 | OK |  |
| 27 | AbstractGame vtable | 0x28 | tick | r | 13208 load / 13209 invoke i64 %513(ptr %266) · divtable AbstractGame 0x28 → tick | 3 | 확인불가(vtable 슬롯) |  |
| 28 | OperationData | 0x10 | blackboard → [1 - player.team] | r | L119 · 12827 gep(%141, i64 %263) · is_recent_visible 의 self | 4 | OK |  |
| 29 | Entity | 0x5c0 | id | r | me(13034 · debug 키) · e(13114 fmt · 13223/13275 TryKill.0) · nearest_enemy(13555 TryKill.0) · 클로저 L73 me.id==c.id | 4 | OK |  |
| 30 | Entity | 0x640 | stat_cached.move_speed | r | L88 · 13008 (적 e) · 0 이면 div0 패닉 | 4 | OK |  |
| 31 | Entity | 0x4c0 | attack_effect@tag | r | L123 · 13468 · -1(None)이면 unwrap 패닉(Location cover.rs:123:57) | 4 | OK |  |
| 32 | Entity | 0x490 | attack_effect@Some.0 (Effect 56B 선두) | r | L123 · 13474 · Effect::range_adjust 의 self | 4 | OK |  |
| 33 | Entity | 0x4a0 | attack_effect.range | r | L123 · effect.rs:26 인라인 Effect::range | 4 | OK |  |
| 34 | Entity | 0x4a8 | attack_effect.growth_range | r | L123 · ×(level-1) | 4 | OK |  |
| 35 | Entity | 0x5c8 | level | r | L123 · 13481 | 4 | OK |  |
| 36 | Entity | 0x438 | stat_buff_cached.range | r | L123 · 13483 | 4 | OK |  |
| 37 | Entity | 0x470 | stat_buff_cached.radius_mult | r | L123 · entity.rs:1511~1515 인라인 Entity::radius (me 13497 · nearest_enemy 13520) | 4 | OK |  |
| 38 | Entity | 0x680 | radius | r | L123 · me 13504/13511 · nearest_enemy 13527/13534 | 4 | OK |  |
| 39 | bumpalo Vec<usize> | 0x0 | ptr | r | near_enemies_p(%38) 순회 시작 12668 · near_allies_p/near_enemies_p 는 fmt 인자 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 40 | bumpalo Vec<usize> | 0x18 | len | r | near_enemies_p 12671 · L100/L108/L112 near_allies_p(%35+24=%382) vs near_enemies_p(%33+24=%383) 비교 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 41 | Option<BigPlan> (sret %0) | 0x0 | tag | w | L49 `return None` (m10.ll:12448) · 본 범위 유일한 sret 쓰기 | 4 | 확인불가(tcx 사전에 타입 없음) | -1 (None) |
| 42 | DebugFrameData (debug %8) | 0x0 | texts (add_text 경유) | w | L45 · context.debug 참일 때만 · 콜리 add_text 가 Vec push(본문 직접 store 아님) | 4 | OK | (champ.x, champ.y+20000, "bush: {:?}, champ_bush: {:?}", Color{1,0,0,1}, 1) |
| 43 | (sret) Option<BigPlan> | 0x0 | tag | w | 이 범위의 리턴 3경로 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 9 (Some(Battle)) 13317·13570 / -1 (None) 13446 |
| 44 | (sret) Option<BigPlan> | 0x8 | Battle 페이로드 BattlePlan 280B | w | target = e(L109·L113) 또는 nearest_enemy(L127) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | BattlePlan::new(version, TryKill(target.id, 60), data, player) 산출을 memcpy (13313·13344·13569) |
| 45 | BattlePlan(로컬 %24/%22/%20 → sret+8 로 복사) | 0x107 | entry_src | w | BattlePlan::new 직후 덮어씀 · 코드표는 BattlePlan 명세 소관 | 4 | OK | 8 (store i8 8 · 13342·13311·13567) |
| 46 | DebugFrameData | 0xa0 | infos[me.id] | w | L95 · ctx.debug 일 때만 · rustc_entry → or_default(빈 Vec 13053~13070 insert_no_grow) → grow_one/push(13151~13181) · 관측 전용 | 4 | OK | push(format!("target: {:?}, near_allies_p: {:?}, near_enemies_p: {:?}, move_to_tower_tick: {}, me_die_tick: {}, enemy_die_tick: {}", e.id, near_allies_p, near_enemies_p, move_to_tower_tick, me_die_tick, enemy_die_tick)) |
| 47 | BattlePlanGoal(로컬 %23/%21/%19 24B) | 0x0 | tag/TryKill.0/TryKill.1 | w | BattlePlan::new 의 goal 인자(dead_on_return) | 4 | OK | 0 / target.id / 60 |

**`consts` 상수 18건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 32000 | 42 | 인덱스 | 셀 크기 — champ.x/32000, champ.y/32000 = 격자 셀 인덱스(좌표 변환, 임계 아님) | 4 |  |
| 1 | 29 | 42 | 인덱스 | clamp(0, 29) 상한 = 30×30 bushes 격자의 마지막 인덱스(llvm.umin 으로 접힘, 하한 0 은 unsigned 라 소거) | 4 |  |
| 2 | 20000 | 45 | 계수 | 디버그 텍스트 y 오프셋(champ.y + 20000) — 판정 무관 | 4 |  |
| 3 | 150000 | 52 | 산출값 | dist_cut — 근처 아군/적 판정 반경(제곱비교 dist_sq <= 150000²) · 150000 = 4.69셀 | 4 |  |
| 4 | -1 | 49 | 태그 | Option<BigPlan>::None 메모리 태그(IR 관측) — champ_bush != bush 이면 즉시 None | 4 |  |
| 5 | 1 | 59 | 인덱스 | 1 - player.info.team = 상대팀 인덱스(shl 접힘 아님 · sub nuw nsw i64 1, %48) — L59 적 챔프 슬라이스 · L64 적 타워 · closure#2 blackboard[1-team] | 4 |  |
| 6 | 48 | 214 | 미상 | lane_path(line, team)[3] — [(u64,u64);7] 원소 16B × 3 = gep 48 로 접힘 · 라인 경로 4번째 웨이포인트를 부시 탐색 기준점(center)으로 씀(L36 인라인 target_bush 내부 · 결과는 사장값) | 4 | 3 |
| 7 | 30 | 100 | 임계 | enemy_die_tick > 30 이면(그리고 아군수 <= 적수) 이 적은 건너뜀 · `icmp ugt %462, 30` 13029 · 틱 단위(60tps=0.5초) | 4 |  |
| 8 | 241 | 104 | 임계 | move_to_tower_tick <= 240 이면 건너뜀 — `icmp ult %499, 241` 13186(·13196) 로 접힘. 줄 길이 산술: L104=36자 = 6+`if move_to_tower_tick <= 240 {`(30) 일치 · 60tps 기준 4초 | 3 | 240 |
| 9 | 120 | 108 | 임계 | me_die_tick > 120 (2초) 이고 아군수 >= 적수 → Battle 전환 · `icmp ugt %509, 120` 13202 | 4 |  |
| 10 | 3 | 111 | 계수 | tick_per_second * 3 (=3초) — `mul i64 %528, 3` 13238 (shl 아님·곱셈 리터럴). wait_limit.saturating_sub(tick) 가 이보다 크면 건너뜀 | 4 |  |
| 11 | 60 | 109 | 산출값 | BattlePlanGoal::TryKill.1 = 60 (13227·13279·13559) · L113·L127 동일 · 산술 소비처는 이 함수 밖(미확인) | 4 |  |
| 12 | 0 | 109 | 태그 | BattlePlanGoal 메모리태그 0 = TryKill (tcxdict --enum BattlePlanGoal · Direct) 13228·13280·13560 | 3 |  |
| 13 | 9 | 109 | 센티널 | BigPlan::Battle 메모리태그 9 (tcxdict --enum BigPlan: idx7 → 니치 태그 9) · Option<BigPlan>::Some 은 같은 값 그대로 · 13317·13570 | 3 |  |
| 14 | 8 | 109 | 산출값 | BattlePlan.entry_src(+0x107) = 8 (store i8 8 · 13342·13311·13567) · 진입귀속 코드 | 4 |  |
| 15 | -1 | 131 | 태그 | Option<BigPlan>::None 판별자 (13446 · DI DISCR_EXACT -1) · 같은 값이 L82~83 get_die_tick_player 의 action 인자 Option<SmallAction>::None 에도(12958) | 4 |  |
| 16 | 99999999 | 91 | 산출값 | nearest_enemy_tower 가 None 일 때 move_to_tower_tick 센티넬(phi 13023) → 항상 > 240 이라 타워복귀 컷을 통과 | 4 |  |
| 17 | 100 | 123 | 계수 | Entity::radius 인라인(entity.rs:1515): radius_mult != 0 이면 radius*(radius_mult+100)/100 (13513~13515 · 13536~13538) | 4 |  |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 근처 아군/적 판정 반경 dist_cut | cover.rs:52 | 150000 | 올리면 더 먼 적 챔프가 near_enemies_p 에 들어가(가시성 조건은 그대로) 배치 H 의 전투 전이 판정 입력이 늘어난다 · 내리면 붙어 있는 적만 센다 · _near_allies_p 는 사장값이라 아군 쪽 효과 없음 | 4 | 기존 |
| 1 | 부시 도착 게이트 (champ_bush == bush) | cover.rs:48 · 42 (bushes 격자 조회) · 38 (target_bush_v30) | 동일성 비교 | 이 함수의 나머지 전부(적 탐색·Battle 전이)가 이 게이트 뒤에 있다 — 챔프가 목표 부시 셀에 서 있지 않으면 무조건 None(플랜 유지). target_bush_v30 의 부시 선택을 바꾸면 커버 대기 위치가 바뀐다 | 4 | 기존 |
| 2 | 적 사망 여유 임계(틱) | cover.rs:100 | 30 | 올리면 '금방 죽는 적'의 범위가 넓어져 아군 열세여도 후보로 남는 적이 늘어난다(교전 전환 증가). 내리면 열세 시 더 자주 건너뛴다 | 4 | 기존 |
| 3 | 적 타워복귀 시간 컷(틱) | cover.rs:104 | 240 | 올리면 타워에서 더 먼 적만 후보가 돼 교전이 줄고, 내리면 타워 근처 적도 물어 다이브성 교전이 는다(60tps=4초) | 4 | 기존 |
| 4 | 내 생존 여유 임계(틱) | cover.rs:108 | 120 | 내리면 더 위험한 상황(빨리 죽는 상황)에서도 아군수>=적수면 교전으로 전환 · 올리면 보수적 | 4 | 기존 |
| 5 | 대기 잔여 임계 배수(tps×N) | cover.rs:111 | 3 | wait_limit 까지 남은 시간이 N초 이하이고 아군수>적수면 교전. N 을 올리면 더 일찍(대기 중에도) 교전 전환, 0 이면 사실상 wait_limit 도달 후에만 | 4 | 기존 |
| 6 | 타워 없음 센티넬 | cover.rs:91 | 99999999 | None 이면 타워복귀 컷을 항상 통과 · 판정 노브 아님(값 바꿔도 240 초과면 동일) | 4 | 기존 |

<details><summary>`callees` 피호출자 36건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_text | game_core::DebugFrameData::add_text | pub | fn(&mut game_core::DebugFrameData, u64, u64, std::string::String, common::color::Color, usize) | game-core\src\simulation\game\frame.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 2 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 6 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | get_die_tick_player | game_ai::get_die_tick_player | pub | fn(&game_core::OperationData, usize, usize, &bumpalo::collections::vec::Vec< usize>, std::option::Option<&game_core::Entity>, std::option::Option<game_core::SmallAction>) -> u64 | game-ai\src\goal_data.rs:577 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | lane_path | game_core::MapDef::lane_path | pub | fn(&game_core::MapDef, game_core::LineType, usize) -> [(u64, u64); 7_usize] | game-core\src\simulation\map_def.rs:174 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | line | game_ai::plan_legacy::types::BigPlan::debug_label::line | in:game_ai::plan_legacy::types | fn(game_core::LineType) -> &str | game-ai\src\plan_legacy\types.rs:83 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | line | game_core::TowerType::line | pub | fn(&game_core::TowerType) -> std::option::Option<game_core::LineType> | game-core\src\simulation\entity\tower.rs:90 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 18 | near_jungle_bush | game_core::near_jungle_bush | pub | fn(&game_core::GameContext, u64, u64, u64, u64) -> std::option::Option<(u64, u64)> | game-core\src\simulation\map_regions.rs:84 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | new | game_ai::plan_legacy::old::BattlePlan::new | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan | game-ai\src\plan_legacy\old\battle.rs:196 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | next_plan | game_ai::plan_legacy::old::LineGankCoverPlan::next_plan | pub | fn(&mut game_ai::plan_legacy::old::LineGankCoverPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> | game-ai\src\plan_legacy\old\line_gank\cover.rs:34 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 22 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 23 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 24 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 25 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 26 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | target_bush | game_ai::plan_legacy::old::LineGankerPlan::target_bush | in:game_ai::plan_legacy::old::line_gank::ganker | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> (u64, u64) | game-ai\src\plan_legacy\old\line_gank\ganker.rs:351 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 28 | target_bush | game_ai::plan_legacy::old::LineGankCoverPlan::target_bush | in:game_ai::plan_legacy::old::line_gank::cover | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> (u64, u64) | game-ai\src\plan_legacy\old\line_gank\cover.rs:196 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 29 | target_bush_v30 | game_ai::plan_legacy::old::LineGankCoverPlan::target_bush_v30 | in:game_ai::plan_legacy::old::line_gank::cover | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> usize | game-ai\src\plan_legacy\old\line_gank\cover.rs:134 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 31 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 32 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 33 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 34 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 35 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 19개**: `bottom`, `bounds`, `clamp`, `continue`, `dist_sq`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `entry`, `enumerate`, `format_inner`, `front_minion`, `grow_one`, `insert_no_grow`, `move_speed`, `near_enemies_p`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `or_default`, `reserve_internal_or_panic`, `rustc_entry`, `top_minion_state`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:8661) · **형제 10개** (LineGankCoverPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::LineGankCoverPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:9 | True | fn(&game_ai::plan_legacy::old::LineGankCoverPlan) -> game_ai::plan_legacy::old::LineGankCoverPlan |
| 1 | <game_ai::plan_legacy::old::LineGankCoverPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:9 | True | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::LineGankCoverPlan::new | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:17 | True | fn(game_core::LineType, usize) -> game_ai::plan_legacy::old::LineGankCoverPlan |
| 3 | game_ai::plan_legacy::old::LineGankCoverPlan::goal | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:21 | True | fn(&game_ai::plan_legacy::old::LineGankCoverPlan) -> game_core::BigGoal |
| 4 | game_ai::plan_legacy::old::LineGankCoverPlan::update | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:25 | True | fn(&mut game_ai::plan_legacy::old::LineGankCoverPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 5 | game_ai::plan_legacy::old::LineGankCoverPlan::is_end | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:30 | False | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 6 | game_ai::plan_legacy::old::LineGankCoverPlan::next_plan | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:34 | False | fn(&mut game_ai::plan_legacy::old::LineGankCoverPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 7 | game_ai::plan_legacy::old::LineGankCoverPlan::target_bush_v30 | in:game_ai::plan_legacy::old::line_gank::cover | game-ai\src\plan_legacy\old\line_gank\cover.rs:134 | False | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> usize |
| 8 | game_ai::plan_legacy::old::LineGankCoverPlan::target_bush | in:game_ai::plan_legacy::old::line_gank::cover | game-ai\src\plan_legacy\old\line_gank\cover.rs:196 | False | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> (u64, u64) |
| 9 | game_ai::plan_legacy::old::LineGankCoverPlan::sub_plan | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:224 | False | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |

**`open` 13건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | (배치 G) cover.rs:36 원문 표기 — rmeta 줄 길이 52자 · target_bush(player, data) 반환 (u64,u64) 의 사용처가 IR 에 0(사장값). `    let _<이름> = self.target_bush(player, data);` 꼴이면 이름이 10자(예: `_old_bush_` 류)여야 52자가 맞고, `let (_a, _b) =` 패턴 등 동일 길이 후보가 여럿이라 표기 불가(동작은 확정: 부수효과=패닉 4곳뿐) | 3 |  |
| 1 | 미탐색 | (배치 G) closure#2 의 `data.blackboard[1 - team]` — 상대팀 인덱스 보드를 쓰는 의미(보드[t] 가 '팀 t 에 대한 관측' 인지 '팀 t 의 관측' 인지)는 Blackboard 정본 미독 · 인덱스 사실만 기록 | 3 |  |
| 2 | 미탐색 | (배치 G) target_bush_v30 본체(m10.ll:11483~11731, 249줄) 미독 — 본 배치 범위 밖 · 반환 range 2..=21 과 인자 승격만 IR define 줄에서 확인 | 4 |  |
| 3 | 미탐색 | (배치 G) game_core 콜리(get_start_position·lane_path·near_jungle_bush·is_recent_visible·add_text·get_entity_by_id) 본체 미독 — 시그니처·반환 의미만(_tcx/game_core.json) · lane_path()[3] 가 라인의 어느 지점인지는 map_def.rs 미독 | 3 |  |
| 4 | 미탐색 | (배치 G) GameContext.debug(+0x3b) 참 경로의 add_text 는 관측 전용 — 판정에 영향 없음(가정 아님: 반환값·sret 에 닿지 않음 확인) | 4 |  |
| 5 | 표기 불가 | (배치 H) L110 원문 표기: IR 은 (wait_limit.saturating_sub(tick) > tps*3) 이면 continue, 아니면 L112 검사 — 소스가 `else if remain <= tps*3 && allies > enemies` 한 조건인지 두 단계 if 인지는 표기 불가(외연 동일 · L110=72자·L111=53자·L112=54자 줄 길이 산술은 3줄 한 조건과 정합) | 3 |  |
| 6 | 재료 부재 | (배치 H) L119 blackboard 색인이 `1 - player.team`(적 팀 인덱스, 12827·m12.ll 14384) 인 의도 — Blackboard.last_visible[+0x1e0][target.position] 을 읽는 구조라 '관측 대상 팀' 인덱스일 수 있으나 소스 없이 확정 불가(IR 사실만 기록) | 4 |  |
| 7 | 미탐색 | (배치 H) BattlePlanGoal::TryKill.1 = 60 의 산술 소비처는 이 함수 밖 — BattlePlan 명세 소관(본 범위에선 리터럴 전달만) | 4 |  |
| 8 | 미탐색 | (배치 H) entry_src=8 의 코드표(진입귀속) 의미 — _docs 에 '진입귀속 코드표' 언급만 있고 값 목록 없음 · BattlePlan/텔레메트리 명세 소관 | 4 |  |
| 9 | 미탐색 | (배치 H) Option<SmallAction>::None 이 i64 -1 로 인코딩되는 이유(rustc 니치 선택) — tcxdict 에 Option<SmallAction> 레이아웃 없음 · IR store 값(12958)만 기록 | 3 |  |
| 10 | 미탐색 | (배치 H) get_die_tick_player 의 내부(TLS 메모 여부·die_tick 의미) 는 r13 잎 명세 소관 — 여기선 계약(data, team, pos, &Vec<usize> enemies, Option<&Entity> tower=None, &Option<SmallAction>=None) -> usize 만 | 4 |  |
| 11 | 미탐색 | (배치 H) Effect::range_adjust(&Effect, caster, target) -> i64 의 의미(경로 계층/game_core) 는 시그니처만 · L123 합산에 그대로 더해짐 | 4 |  |
| 12 | 표기 불가 | (배치 H) L85 색인식 문자 수(53자)와 `data.cache.player_champion[1 - player.team][*e]`(47~52자) 불일치 — 실제 필드 경로 표기(player.info.team 등)는 표기 불가 | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 G) Option<BigPlan>::None 이 왜 -1 인가 — tcxdict 에 Option<BigPlan> 레이아웃 없음 · BigPlan 태그 2..=17 에서 rustc Niche::reserve 산술상 18 이 예상되나 IR 은 -1 을 store(12448·13446). IR 정본으로 -1 채택, 니치 배치 규칙 차이는 미확정(tcx `layout_of Option<BigPlan>` 덤프가 있으면 확정 가능) | 3 | 사실 서술 |
| 1 | (배치 H) L123 의 소스 표기(헬퍼 이름 attack_range 류) — 전부 인라인돼 이름 소실 · 계산식은 IR 로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 2건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | (배치 G) cover.rs:40 원문 표기 — unwrap Location 40:95 ⟹ 앞 94자. 후보 `let champ = data.cache.player_champion[player.info.team][player.info.position as usize].` 는 92자로 2자 잔차 · 동작은 확정(같은 슬롯 재로드 후 unwrap) | 본문에 해소 표기가 있다 |
| 1 | (배치 G) dist_sq 헬퍼 인라인 사슬 `;L9<202`·`;L3147<7<202`(L200~202·L213) 의 소유 파일(7~9줄짜리 좌표 유틸) 미확인 — 계산은 \|dx\|²+\|dy\|² 로 확정(u64::abs_diff = core 3147) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

