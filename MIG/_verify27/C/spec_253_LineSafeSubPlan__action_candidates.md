---

### `253` LineSafeSubPlan::action_candidates — 라인 안전(LineSafe) 서브플랜의 소행동 후보 생성: 아군 최전방(타워/최전방 미니언) 주변 대기 + 도주 + 전투 + 미니언 + 소환수 + 사거리 안 적 타워 평타 + 구조물 스킬을 모은 뒤 v30 타워 어그로 위험 액션을 제거

| 항목 | 값 |
|---|---|
| id | `LineSafe__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9line_safeNtB2_15LineSafeSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\line_safe.rs:89` |
| IR | `m02.ll` 47101~48424행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::LineSafeSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `ccacf0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[253]/sig/tls/<키>`)**

없음 — 본문 47101~48424 및 aux 6곳에 LocalKey/call_once/llvm.threadlocal.address/__getit 참조 0건(grep) · @anon 상수 5건은 전부 panic Location(.218/.219 unwrap · .243/.244/.245 bounds) · reach 콜리 요약에도 TLS 접점 콜리 없음(콜리 내부 TLS 는 각 콜리 명세 소관)

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::vec::Vec<SmallActionPlay> (32B) | 반환 슬롯 · IR 속성 sret([32 x i8]) writeonly captures(none) dereferenceable(32) · initializes 속성 없음 · L101 에서 지역 res(%27) 32B 를 memcpy | 4 |
| 1 | 1 | self | &LineSafeSubPlan (1B: +0x0 line: LineType u8 Top=0/Mid=1/Bottom=2) | IR 속성 dereferenceable(1) (readonly 없음) · 동작상 읽기만: L18/L30 switch(line)·L21 get_start_position(&self.line,..)·L45 line_minion_action_candidates(.., self.line) · store 0회 | 4 |
| 2 | 2 | version | usize (i64) | 본문 직접 분기 0회 · %28 alloca 에 스필해 retain 클로저 env(+0)에 &version 으로 캡처(L99 → v30 에 *version 전달) · SmallActionAround::new(L34/L37 · 콜리 이름 `_version` 미사용) · battle_action(L94) · line_minion_action_candidates(L45) · v22(L64) 에 값 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B, align 16) | IR 속성 없음(readnone/readonly 모두 없음) · 본문 직접 접근 0회 · gen_range 호출 사이트 0개 · 전달처 2곳: SmallActionAround::new(L34 또는 L37 중 하나만 실행 · 콜리 define 이 `_rnd` readnone) · fight_check::battle_action(L94 · 콜리 define 이 `_rnd` readnone) ⟹ 이 함수 서브트리에서 rnd 상태 변화 0 (콜리 속성 근거) | 4 |
| 4 | 4 | player | &PlayerState (2528B) | IR 속성 readonly dereferenceable(2528) · info.team(+0x930) L18 · info.position@tag(+0x9c0) L49 · 콜리 7곳에 전달 · retain 클로저 env(+0x10) | 4 |
| 5 | 5 | data | &OperationData (24B: +0 cache &AbstractGameWithCache · +8 context &GameContext · +0x10 blackboard &[Blackboard;2]) | IR 속성 readonly dereferenceable(24) · context.pool(bump) L90/L16 · cache.tower/twin_towers/nexus/player_champion/game(vtable) · blackboard[team].minion_state · 콜리 전부에 전달 · retain 클로저 env(+8) | 4 |
| 6 | 6 | _parameter | &ScoreParameter (5384B) | IR 속성 readonly captures(none) dereferenceable(5384) · 본문 사용 0회(grep) · 이름도 `_parameter` | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates(&self, version, rnd, player, data, _parameter) -> Vec<SmallActionPlay>  [line_safe.rs:89]
  L90  bump = data.context.pool; res = Vec::new_in(bump)   // sret 32B: ptr=dangling8, a=bump, cap=len=0

  L92  res.extend(self.base_positioning(version, rnd, player, data))   — 인라인(line_safe.rs:15~42), 항상 원소 1개:
    L16  vec 는 같은 bump 에 new_in
    L18  team = player.info.team;  tower = cache.tower(self.line, team)   // simulation.rs 인라인: team<2 bounds check(line 별 panic 3곳) → match line { Top: top_tower[team].or(top_tower2[team]) · Mid: mid_tower.or(mid_tower2) · Bottom: bottom_tower.or(bottom_tower2) } (Option::or, 첫 Some)
    L19~24  twin = cache.twin_towers[team].iter()
              .min_by_key(|t| { L21 (x,y) = self.line.get_start_position(context.setting, team); L22 utils::distance_sq(t.x, t.y, x, y) })   // 첫 원소는 본문 인라인·2번째부터 aux m12 fold(원소마다 get_start_position 재호출) · 동률이면 앞 원소 유지 · len==0 → None
              .map(|t| *t)  (L24 closure#1)
         nearest_tower = tower.or(twin)                     // ★eager: twin 의 min_by_key 는 tower 가 Some 이어도 계산된다(IR 이 %66 select 전에 %104 fold 호출)
    L25  nearest_tower = nearest_tower.or(cache.nexus[team])
    L26  nearest_tower = nearest_tower.unwrap()             // 셋 다 None 이면 unwrap_failed 패닉
    L28  nexus = cache.nexus[team].unwrap()
    L30  ms = data.blackboard[team].minion_state(self.line)   // Top→+0x0 · Mid→+0x28 · Bottom→+0x50 (BrainMinionParameter 40B)
    L31  front_minion: Option<&Entity> = ms.front_minion.and_then(|id| cache.game.get_entity_by_id(id))   // vtable +0x1f0
    L33  if front_minion.is_none_or(|m| utils::distance_sq(m, nexus) < utils::distance_sq(nearest_tower, nexus)) {   // None 이거나, 최전방 미니언이 우리 넥서스에 타워보다 더 가까우면(웨이브가 타워 안쪽까지 밀림)
    L34~35   [ SmallActionPlay::Around( SmallActionAround::new(version, rnd, data, player, nearest_tower.id, 5).with_position_eval_purpose(LaneSafe/*10*/) ) ]
    L37~38 } else {
           [ SmallActionPlay::Around( SmallActionAround::new(version, rnd, data, player, front_minion.id, 5).with_position_eval_purpose(LaneSafe) ) ]   // 미니언이 타워보다 앞(적 쪽)이면 미니언 주변
         }

  L93  res.push(SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5)))       // 태그 3 · 무조건
  L94  res.extend(fight_check::battle_action(version, rnd, player, data, 5))                 // 배치 E~F 명세
  L95  res.extend(self.attack_minion_action(version, player, data))  == L45 line_minion_action_candidates(version, data, player, self.line)   // r14 명세
  L96  res.extend(fight_check::attack_summon_action(player, data))                            // r13 명세
  L97  res.extend(self.attack_tower_action(version, player, data))  — 인라인(line_safe.rs:48~69), Option<SmallActionPlay>(0 또는 1개):
    L49  champ = cache.player_champion[team][player.info.position.as_index()]?    // None → 아무것도 안 넣음
    L50  nearest_enemy_tower = cache.iter_towers(1 − team)      // 열거 순서: [top_tower, mid_tower, bottom_tower, top_tower2, mid_tower2, bottom_tower2] 의 Some 만 → twin_towers[1−team] 슬라이스 → nexus[1−team]  (★적 넥서스도 후보)
    L51      .filter(|t| t.can_target())                         // Entity::can_target(): can_target(+0x6b9) && block_target_tick(+0x6a0) == 0 (첫 원소 본문 인라인 · 이후 aux m06/m11/m02-77166)
    L52      .min_by_key(|t| utils::distance_sq(t, champ))?     // 첫 최소 우선 · 없으면 None
    L54  move_speed = champ.stat_cached.move_speed
    L55  atk = champ.attack_effect.as_ref()?                    // +0x4c0 == −1 → None
    L57  dist = utils::distance_sq(nearest_enemy_tower, champ)   // 제곱 거리
    L58  range = atk.range(champ, nearest_enemy_tower) + champ.radius() + nearest_enemy_tower.radius()
           Effect::range(effect.rs:25~26 인라인) = atk.range(+0x4a0) + champ.stat_buff_cached.range(+0x438) + (champ.level(+0x5c8) − 1) × atk.growth_range(+0x4a8) + atk.range_adjust(champ, tower)(콜)
           Entity::radius()(entity.rs:1511~1515) = radius_mult(+0x470)==0 ? radius(+0x680) : radius × (mult+100) / 100
           ※7항 덧셈은 LLVM 이 재결합(move_speed×30 이 range 항에 먼저 더해짐) — 합은 확정, 항의 소스 줄 소속(58 vs 59)은 추정
    L59  max_dist = range + move_speed × 30
    L60  if dist > max_dist × max_dist { return None }            // 사거리+30틱 이동 밖이면 제외
    L64  if !tower_discipline::v22_lane_tower_pressure_attack_allowed(version, data, player, nearest_enemy_tower) { return None }
    L65  Some(SmallActionPlay::Attack(SmallActionAttack::new(data, nearest_enemy_tower.id)))   // 태그 15
  L98  res.extend(fight_check::attack_structure_skill_action(player, data))                 // r13 명세
  L99  res.retain(|a| !tower_discipline::v30_line_champion_action_tower_aggro_risk(version, data, player, a))   // aux m01: v30==true 인 원소 제거(DrainFilter · drop_glue) · 순서 보존
  L101 res   (sret memcpy 32)

분기 요약: 무조건 경로 = Around 1 + RunAway 1 + battle_action + minion + summon + structure_skill 후 v30 필터. 조건부 = Attack(적 타워/넥서스) 1개 — champ 有 ∧ can_target 적 구조물 有 ∧ attack_effect 有 ∧ dist² ≤ (range+radius₂+ms×30)² ∧ v22 허용.
rnd: 본문 gen_range 0회 · 전달 콜리 2곳(Around::new · battle_action) 모두 define 에 readnone → 서브트리 rnd 소비 0.
사장 코드: reach(version=2, gamemode=0) 103 블록 전부 live · NA 0.
```

**`mem` 메모리 접근 43건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LineSafeSubPlan(self) | 0x0 | line | r | L18 switch(0/1/2 · 그 밖 unreachable)·L30 switch·L21 &self 전달·L45 i8 값 전달 | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | L18 team · <2 bounds check(panic_bounds_check ×3 — line 별) · 1−team 은 L50 적 팀 | 4 | OK |  |
| 2 | PlayerState | 0x9c0 | info.position@tag | r | L49 Position::as_index()(entity.rs:581) i32 zext → player_champion[team][pos] | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache — L18/19/25/28/31/49/50 | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | &GameContext — L90/L16 pool · L21 setting | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — L30 [team] | 4 | OK |  |
| 6 | GameContext | 0x0 | pool | r | bump — L90 res=Vec::new_in(bump) · L16 base_positioning 의 vec! 도 같은 bump | 4 | OK |  |
| 7 | GameContext | 0x8 | setting | r | &GameSetting(5432B) — L21 get_start_position 2번째 인자 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame 팻포인터: +0 data_ptr · +8 vtable_ptr) | r | L31 vtable+0x1f0 get_entity_by_id(data_ptr, id) 간접호출(divtable: ExpectedGame::AbstractGame::get_entity_by_id) | 3 | OK |  |
| 9 | AbstractGameWithCache | 0x130 | twin_towers[team] (bumpalo Vec<&Entity> 32B: +0 ptr · +0x18 len) | r | L19 .iter() — stride 32 · len==0 이면 min_by_key=None | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x170 | nexus[team] (Option<&Entity> 니치 null=None) | r | L25 .or(nexus) · L28 nexus.unwrap() | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x180 | top_tower[team] | r | L18 tower(): line==Top 1순위(gep 384) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x190 | top_tower2[team] | r | L18 tower(): line==Top 2순위(gep 400) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x1a0 | mid_tower[team] | r | L18 line==Mid 1순위(gep 416) | 4 | OK |  |
| 14 | AbstractGameWithCache | 0x1b0 | mid_tower2[team] | r | L18 line==Mid 2순위(gep 432) | 4 | OK |  |
| 15 | AbstractGameWithCache | 0x1c0 | bottom_tower[team] | r | L18 line==Bottom 1순위(gep 448) | 4 | OK |  |
| 16 | AbstractGameWithCache | 0x1d0 | bottom_tower2[team] | r | L18 line==Bottom 2순위(gep 464) | 4 | OK |  |
| 17 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | L49 champ(Option<&Entity> null=None → 빈 반환) · [5 x ptr] 행 stride 40 | 4 | OK |  |
| 18 | Blackboard | 0x0 | top_minion_state (BrainMinionParameter 40B) | r | L30 minion_state(line): Top→+0x0 · Mid→+0x28 · Bottom→+0x50 (blackboard.rs:378~382) · blackboard[team] stride 744 | 4 | OK |  |
| 19 | BrainMinionParameter | 0x0 | front_minion@tag (i64 Direct · trunc→bool Some) | r | L31 and_then — 태그 0 이면 front_minion=None | 4 | OK |  |
| 20 | BrainMinionParameter | 0x8 | front_minion@Some.0 (usize id) | r | L31 get_entity_by_id(id) 인자 | 4 | OK |  |
| 21 | Entity | 0x5c0 | id | r | L34 nearest_tower.id / L37 front_minion.id → Around target · L65 nearest_enemy_tower.id → Attack target | 4 | OK |  |
| 22 | Entity | 0x660 | x | r | utils::distance_sq 인자 — L22(타워↔라인 시작점)·L33(front_minion/nearest_tower↔nexus)·L52·L57(타워↔champ) | 4 | OK |  |
| 23 | Entity | 0x668 | y | r | 위와 같은 4곳 | 4 | OK |  |
| 24 | Entity | 0x6b9 | can_target (bool) | r | L51 Entity::can_target()(entity.rs:1478) 1항 — 적 타워 필터 | 4 | OK |  |
| 25 | Entity | 0x6a0 | block_target_tick | r | L51 Entity::can_target() 2항 ==0 | 4 | OK |  |
| 26 | Entity | 0x640 | stat_cached.move_speed | r | L54 move_speed → L59 ×30 | 4 | OK |  |
| 27 | Entity | 0x490 | attack_effect@Some.0 (Effect 56B) | r | L55 as_ref() → atk · L58 Effect::range(atk, champ, tower)·range_adjust 의 self | 4 | OK |  |
| 28 | Entity | 0x4a0 | attack_effect.range (Effect+0x10) | r | L58 Effect::range 항 | 4 | OK |  |
| 29 | Entity | 0x4a8 | attack_effect.growth_range (Effect+0x18) | r | L58 (level−1)×growth_range | 4 | OK |  |
| 30 | Entity | 0x4c0 | attack_effect@tag (i32 니치 · −1=None) | r | L55 None 이면 attack_tower_action=None(빈 반환) | 4 | OK |  |
| 31 | Entity | 0x5c8 | level | r | L58 (level−1)×growth_range | 4 | OK |  |
| 32 | Entity | 0x438 | stat_buff_cached.range | r | L58 Effect::range 항(caster 사거리 버프) | 4 | OK |  |
| 33 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | L58 Entity::radius()(entity.rs:1511~1515) — champ·tower 각각 | 4 | OK |  |
| 34 | Entity | 0x680 | radius | r | L58 Entity::radius() — champ·tower 각각 | 4 | OK |  |
| 35 | sret res (bumpalo Vec<SmallActionPlay> 32B · 지역 %27 → L101 memcpy 32) | 0x0 | buf.ptr | w | L90 Vec::new_in(bump) · push 시 len==cap 이면 reserve_internal_or_panic(used_cap,1,true) | 4 | 확인불가(tcx 사전에 타입 없음) | inttoptr 8 (dangling) → reserve_internal_or_panic 후 bump 힙 ptr |
| 36 | sret res | 0x8 | buf.a | w | L90 | 4 | 확인불가(tcx 사전에 타입 없음) | bump (= data.context.pool) |
| 37 | sret res | 0x10 | cap | w | L90 memset 16B(cap·len=0) | 4 | 확인불가(tcx 사전에 타입 없음) | 0 → reserve 갱신 |
| 38 | sret res | 0x18 | len | w | L93·L65 는 memcpy 184 후 len+1 · extend 5회는 콜리 sret Vec (ptr,len) 를 그대로 extend | 4 | 확인불가(tcx 사전에 타입 없음) | 0 → push/extend 마다 +1·+n → L99 retain 후 len−del |
| 39 | res 원소[len] (bump 힙 184B) | 0xb1 | SmallActionPlay@tag | w | alloca 에 태그 store 후 원소 통째 memcpy | 4 | 확인불가(tcx 사전에 타입 없음) | 5(Around, L34/L37) · 3(RunAway, L93) · 15(Attack, L65) |
| 40 | res 원소 Around 페이로드 | 0x80 | position_eval_purpose@tag | w | L35/L38 with_position_eval_purpose(around.rs:56~57) — new 이 넣은 값을 덮어씀 | 4 | 확인불가(tcx 사전에 타입 없음) | 10 (PositionEvalPurpose::LaneSafe) |
| 41 | 지역 base_positioning vec(%18, bump) | 0x0 | buf.ptr/buf.a/cap/len | w | L16/L34/L37 · L92 extend 로 res 에 복사되고 %18 자체는 drop(bump 라 해제 없음) | 4 | 확인불가(tcx 사전에 타입 없음) | new_in(bump) → 원소 1 push |
| 42 | 스택 %28 (version 스필 8B) | 0x0 | version | w | L99 retain 클로저 env{+0 &version, +8 data, +0x10 player} 가 참조 | 4 | 확인불가(tcx 사전에 타입 없음) | %2 |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 18 | 임계 | 팀 인덱스 상한(team<2 · panic_bounds_check ×3, line 별) — cache.tower(line, team) 인라인(simulation.rs) | 4 |
| 1 | 5 | 34 | 산출값 | end_delay=5 — SmallActionAround::new(.., target, end_delay=5)(L34·L37) · SmallActionRunAway::new(data, player, end_delay=5)(L93) · fight_check::battle_action(.., 5)(L94 · 콜리는 `_end_delay` 미사용). ※단위(틱)는 콜리 소관 | 4 |
| 2 | 5 | 34 | 태그 | SmallActionPlay::Around 메모리 태그(+0xb1 = 5 · tcxdict --enum: idx2 → 태그 5) — L34/L37 store i8 5 | 3 |
| 3 | 10 | 35 | 센티널 | PositionEvalPurpose::LaneSafe 메모리 태그(+0x80 = 10 · idx8 → 니치 태그 10) — with_position_eval_purpose 인라인 store i8 10 (L35·L38) | 4 |
| 4 | 3 | 93 | 태그 | SmallActionPlay::RunAway 메모리 태그(+0xb1 = 3) — L93 `store i8 3`. ※경고의 `shl i64 %71, 3`(m02.ll 47251)은 twin_towers len×8(&Entity 스트라이드) 포인터 산술이지 판정 상수가 아님 — 이 항목과 무관 | 4 |
| 5 | 15 | 65 | 태그 | SmallActionPlay::Attack 메모리 태그(+0xb1 = 15) — L65 | 4 |
| 6 | -1 | 55 | 센티널 | Option<Effect> None 니치(attack_effect@tag +0x4c0 i32 == −1) — L55 as_ref()? 조기 반환 · L58 `level + (−1)` 의 (level−1) 계산에도 같은 리터럴 · 이터레이터 내부 Chain 상태값 −1/−2 는 라이브러리 구현(판정 아님) | 4 |
| 7 | 0 | 51 | 태그 | block_target_tick == 0 (Entity::can_target 2항) · L58 radius_mult == 0 이면 radius 그대로 · L30 front_minion@tag == 0 ⇔ None | 4 |
| 8 | 30 | 59 | 계수 | max_dist = range + move_speed × 30 — 30틱(0.5초@60tps 추정) 이동 여유 | 5 |
| 9 | 100 | 58 | 계수 | Entity::radius() 인라인(entity.rs:1511~1515): radius_mult≠0 이면 radius × (mult + 100) / 100 (udiv) — champ·tower 각 1회 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 적 타워 평타 후보 이동 여유(틱) | line_safe.rs:59 (m02.ll 48222 `mul i64 %349, 30`) | 30 | 올리면 사거리에서 더 먼(30틱 이동분 이상) 적 타워/넥서스도 Attack 후보에 들어간다(이후 v22·v30·평가가 걸러야 함) · 내리면 사거리 근처 타워만 | 4 | 기존 |
| 1 | Around/RunAway/battle_action 의 end_delay | line_safe.rs:34·37·93·94 | 5 | 소행동 종료 지연(단위·소비처는 SmallActionAround/RunAway 명세) — 올리면 대기/도주 액션이 더 오래 유지 | 4 | 기존 |
| 2 | 라인 안전 대기 위치 결정 규칙(타워 vs 최전방 미니언) | line_safe.rs:33 (m02.ll 47530 `icmp ult`) | d²(front_minion,nexus) < d²(nearest_tower,nexus) → 타워 주변, 아니면 미니언 주변 | 부등호를 뒤집거나 여유(예: 타워 거리 + 마진)를 넣으면 미니언이 타워 근처일 때 어느 쪽에 서는지가 바뀐다 · 미니언 없음은 항상 타워 | 4 | 기존 |
| 3 | Around 의 position_eval_purpose | line_safe.rs:35·38 | 10 | LaneSafe(10) → 다른 purpose(예: Lane=8·General=2)로 바꾸면 SmallActionAround::get_input 의 위치 평가 스타일이 바뀐다(소비처는 position_eval 명세) | 4 | 기존 |
| 4 | 적 구조물 후보에 넥서스 포함 | AbstractGameWithCache::iter_towers (g15.ll 102521 · 마지막 Chain 원소 nexus[team]) | 포함 | iter_towers 를 바꾸지 않고 이 함수에서 nexus 를 제외하려면 L51 filter 에 조건 추가 — 현재는 가장 가까운 can_target 구조물이 넥서스면 넥서스 평타 후보가 나온다 | 4 | 기존 |

<details><summary>`callees` 피호출자 43건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::SubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:118 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 1 | action_candidates | game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\hide.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 2 | action_candidates | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 3 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | attack_minion_action | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_safe | fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_safe.rs:44 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | attack_minion_action | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_wait | fn(&mut game_ai::plan_legacy::sub_plan::LineWaitSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_wait.rs:99 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | attack_minion_action | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_defense | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_defense.rs:368 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | attack_structure_skill_action | game_ai::attack_structure_skill_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:794 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | attack_tower_action | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_safe | fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_safe.rs:48 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | base_positioning | game_ai::plan_legacy::sub_plan::JungleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::jungle | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> game_ai::SmallActionPlay | game-ai\src\plan_legacy\sub_plan\jungle.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | base_positioning | game_ai::plan_legacy::sub_plan::BattleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::battle | fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\battle.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | base_positioning | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_safe | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_safe.rs:15 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 18 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 19 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 20 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 21 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 22 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 25 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | line_minion_action_candidates | game_ai::line_minion_action_candidates | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\small_action\lane_minion.rs:72 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 29 | new | game_ai::SmallActionAround::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 33 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 34 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 35 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 36 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 37 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 39 | v22_lane_tower_pressure_attack_allowed | game_ai::v22_lane_tower_pressure_attack_allowed | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:394 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | v30_line_champion_action_tower_aggro_risk | game_ai::v30_line_champion_action_tower_aggro_risk | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_ai::SmallActionPlay) -> bool | game-ai\src\tower_discipline.rs:158 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | with_position_eval_purpose | game_ai::SmallActionAround::with_position_eval_purpose | pub | fn(game_ai::SmallActionAround, game_ai::PositionEvalPurpose) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:56 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 42 | with_position_eval_purpose | game_ai::SmallActionAroundPosition::with_position_eval_purpose | pub | fn(game_ai::SmallActionAroundPosition, game_ai::PositionEvalPurpose) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:857 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
</details>

⚠**미매칭 8개**: `block_target_tick`, `check  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 2개는 **전부 다른 함수**라 싣지 않는다`, `extend`, `is_none_or`, `reach`, `reserve_internal_or_panic`, `retain`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35190) · **형제 9개** (LineSafeSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::LineSafeSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan) -> game_ai::plan_legacy::sub_plan::LineSafeSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::LineSafeSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:11 | True | fn(game_core::LineType) -> game_ai::plan_legacy::sub_plan::LineSafeSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_safe | game-ai\src\plan_legacy\sub_plan\line_safe.rs:15 | False | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_safe | game-ai\src\plan_legacy\sub_plan\line_safe.rs:44 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_safe | game-ai\src\plan_legacy\sub_plan\line_safe.rs:48 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:71 | True | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:89 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 8 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:104 | False | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L60 `dist > max²` 조기반환 vs `dist <= max²` 진행, L64 `!v22 → None` vs `v22 → Some` — 외연 동일(표기 불가) | 4 |  |
| 1 | 표기 불가 | L33 은 DI 가 Option::is_none_or 로 확정 — 단 `is_none_or(\|m\| a < b)` vs `!is_some_and(\|m\| a >= b)` 는 외연 동일(표기 불가) | 4 |  |
| 2 | 표기 불가 | base_positioning L19 `.or(...)` 가 eager 인 것은 IR 순서(min_by_key fold 가 tower select 앞)로 확정이나, 소스가 `.or(iter…)` 인지 `let twin = …; tower.or(twin)` 인지는 표기 불가 | 4 |  |
| 3 | 미탐색 | L22 closure 가 get_start_position 을 원소마다 재호출하는 것은 IR(aux m12 fold 루프 안 call)로 확정 — 소스가 클로저 안에 호출을 둔 것인지 컴파일러가 못 끌어올린 것인지 구분 불필요(동작 동일) | 4 |  |
| 4 | 미탐색 | SmallActionAround::new 인자 5 의 이름은 콜리 DI(`end_delay`, arg 6)로 확정 · 단위(틱 여부)·소비처는 SmallActionAround 명세 소관 — 여기선 미확인 | 4 |  |
| 5 | 미탐색 | v30_line_champion_action_tower_aggro_risk / v22_lane_tower_pressure_attack_allowed / battle_action / line_minion_action_candidates / attack_summon_action / attack_structure_skill_action / Effect::range_adjust / get_start_position 내부는 안 봄(계약만) — _docs 에 LineSafe·v30·v22 관련 개발자 주석 0건(grep: line_safe/LineSafe/v30/v22/어그로) | 4 |  |
| 6 | 미탐색 | LineSafeSubPlan 을 누가 어느 조건에서 고르는지(SubPlan 선택·score)는 이 함수 밖(LineSafeSubPlan::score line_safe.rs:104 · r17 명세) | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L58/L59 의 항 소속: IR 이 7항 덧셈을 재결합(%417 = atk.range + move_speed×30 이 L26<58 로 찍힘)해 `range + champ.radius + tower.radius` 가 L58, `+ move_speed×30` 이 L59 라는 배정은 dbg 줄 우세로 추정 — 합(max_dist)은 확정 | 4 | 사실 서술 |
| 1 | iter_towers 의 136B 이터레이터 내부 상태(+0/+8 outer-b Option<IntoIter>, +0x10 −2/−1 상태값, +0x18/+0x20 array 범위, +0x28.. [Option<&Entity>;6], +0x78/+0x80 슬라이스)는 라이브러리 구현 — 원소 열거 순서만 g15.ll:102521 store 순서로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

