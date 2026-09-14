---

### `158` line_minion_action_candidates — 적 미니언 전수 스캔 → 사거리(+이동 30틱 여유) 안이면 Attack/Skill/Skill2 후보를 bumpalo Vec 으로 모은다

| 항목 | 값 |
|---|---|
| id | `lane_minion__line_minion_action_candidates` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai12small_action11lane_minion29line_minion_action_candidates` |
| 소스 | `game-ai\src\small_action\lane_minion.rs:72` |
| IR | `m11.ll` 54541~55418행 |
| 경로·가시성 | `game_ai::line_minion_action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `e2ac00` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::Vec<SmallActionPlay> (32B) | sret([32 x i8]) writeonly. 레이아웃: +0x0 ptr(빈 Vec 은 dangling 8 · m11.ll:54592) / +0x8 &Bump(=data.context.pool · 54589~54590 로드, 54593~54594 store) / +0x10 cap / +0x18 len(둘 다 memset 0 · 54598). 원소 = SmallActionPlay 184B, 태그 = 원소+0xb1(1B, 니치 · tcxdict --enum SmallActionPlay): 15=Attack 16=Skill 17=Skill2. 원소 안 live 바이트 = [0x0,0x18) 페이로드(SmallActionAttack/Skill/Skill2 24B: +0 start_tick, +8 target, +0x10 is_act 1B — `::new` 의 sret 24B memcpy · 55147/55258/55369) + 0xb1 태그 1B. [0x18,0xb1)·[0xb2,0xb8)·0x11~0x17 은 alloca 잔류(미기록) | 3 |
| 1 | 1 | version | usize | 본 함수에선 분기 없음. should_suppress_direct_minion_attack 에 그대로 전달(55132) | 4 |
| 2 | 2 | data | &OperationData (24B) | readonly. +0 cache(&AbstractGameWithCache 8840B), +8 context(&GameContext) → context+0 = pool(&Bump) | 4 |
| 3 | 3 | player | &PlayerState (2528B) | readonly. info.team(0x930)·info.position(0x9c0) 만 읽음 | 4 |
| 4 | 4 | line | LineType (i8, range 0..3) | 0=Top 1=Mid 2=Bottom (tcxdict --enum LineType). 스캔 반경 밖 미니언의 라인 필터(55077) + should_suppress 전달 | 3 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn line_minion_action_candidates(version, data, player, line) -> Vec<SmallActionPlay, &Bump>
[L74] _t = ProfTimer::new(65)  (prof::ENABLED 일 때만 Instant::now)
[L75] res = Vec::new_in(data.context.pool)          ; 32B: ptr=8(dangling) bump cap=0 len=0
[L76] champ = data.cache.player_champion[player.info.team][player.info.position]  ; team bounds<2
[L77] if champ.is_none() { return res }              ; 빈 Vec(54651)
[L79] move_speed = champ.stat_cached.move_speed (0x640)
[L81] champ_radius = Entity::radius(champ) = if radius_mult==0 { radius } else { radius*(radius_mult+100)/100 }
[L82] can_attack = champ.can_attack()
[L84] atk_base: Option<i64> = champ.attack_effect.map(|e| e.range + champ_radius + champ.stat_buff_cached.range + (champ.level-1)*e.growth_range)   ; effect.rs:1162 헬퍼 인라인, None 이면 undef
[L85] can_skill = champ.can_skill()
[L87] skill_base = champ.skill_effect.map(같은 식 · 0x4d8/0x4e0)
[L88] can_skill2 = champ.can_skill2()
[L89] skill2_eff = if champ.level > 2 { &champ.skill2_effect } else { &None }   ; entity.rs:1693
[L90] skill2_base = skill2_eff.map(같은 식 · +16/+24)
[L92] for target in data.cache.iter_minions(1 - player.info.team)   ; 적 팀 미니언 · 56B 체인 이터레이터(슬라이스 3개 순차)
  [L93]  if champ.team is Player(t) && target.visible_state[t] != Visible { continue }   ; entity.rs:1482 헬퍼 인라인. champ Neutral 이면 검사 생략
  [L97]  dist_sq = |dx|^2 + |dy|^2  (champ.x/y vs target.x/y, 0x660/0x668)
  [L98]  near_default_scan = dist_sq < 6400000000 (80000^2)
  [L99]  if !near_default_scan && !(target.ty is Minion && target.ty.Minion.info.line == line) { continue }   ; lane_minion.rs:65 헬퍼 인라인
  [L103] if can_attack {
  [L104]   atk = champ.attack_effect.as_ref().unwrap()          ; None 이면 unwrap_failed 패닉
  [L106]   max_dist = atk_base + move_speed*30 + atk.range_adjust(champ, target)
  [L107]   max_dist += Entity::radius(target)
  [L108]   if dist_sq <= max_dist*max_dist {
  [L109]     if !SmallActionLaneMinionPosition::should_suppress_direct_minion_attack(version, data, player, line, target) {
  [L110]       res.push(SmallActionPlay::Attack(SmallActionAttack::new(data, target.id)))   ; 태그 15
  } } }
  [L114] if can_skill && champ.skill_effect.is_some() {
  [L115]   if CastingTarget::check(&skill.target, champ, target) {
  [L116]     max_dist = skill_base + move_speed*30 + skill.range_adjust(champ, target)
  [L117]     max_dist += Entity::radius(target)
  [L118]     if dist_sq <= max_dist^2 {
  [L119]       res.push(SmallActionPlay::Skill(SmallActionSkill::new(data, target.id)))   ; 태그 16
  } } }
  [L124] if can_skill2 && skill2_eff.is_some() {
  [L125]   if CastingTarget::check(&skill2.target, champ, target) {
  [L126]     max_dist = skill2_base + move_speed*30 + skill2_eff.unwrap().range_adjust(champ, target)   ; None 이면 unwrap_failed(도달 불가: is_some 확인 후)
  [L127]     max_dist += Entity::radius(target)
  [L128]     if dist_sq <= max_dist^2 {
  [L129]       res.push(SmallActionPlay::Skill2(SmallActionSkill2::new(data, target.id)))   ; 태그 17
  } } }
}
[L135] res(54981 memcpy 32B)   ; [L136] ProfTimer drop → PHASE_NANOS[65] += elapsed, PHASE_CALLS[65] += 1

★공격 후보에만 should_suppress 게이트가 있고 Skill/Skill2 에는 없다. ★Skill/Skill2 는 is_some 을 먼저 보지만 Attack 은 can_attack 만 보고 unwrap 한다(attack_effect None+can_attack true 면 패닉 — 게임 데이터상 도달 여부 미확인).
```

**`mem` 메모리 접근 32건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 54597. bounds<2 (54600) 아니면 panic_bounds_check | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 54618, i32 zext. player_champion 2차 인덱스 | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | 54621 → AbstractGameWithCache | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | 54589 → GameContext+0x0 pool(&Bump, 54590) 을 Vec 할당자로 | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | 54622~54625 [2][5] x 8B · null=None(54626) → 빈 Vec 반환 | 4 | OK |  |
| 5 | Entity | 0x640 | stat_cached.move_speed (champ) | r | 54641. move_speed*30 이 사거리 여유로 가산(54839) | 4 | OK |  |
| 6 | Entity | 0x470 | stat_buff_cached.radius_mult (champ·target) | r | 54644 champ / 55100·55218·55329 target. Entity::radius() 인라인(entity.rs:1511~1515) | 4 | OK |  |
| 7 | Entity | 0x680 | radius (champ·target) | r | 54658·54665 champ / 55107·55114 등 target. mult!=0 이면 radius*(mult+100)/100 | 4 | OK |  |
| 8 | Entity | 0x4c0 | attack_effect@tag (니치 -1=None) | r | 54687. None 이면 atk_base undef, can_attack 이면 unwrap_failed(55088) | 4 | OK |  |
| 9 | Entity | 0x4a0 | attack_effect.range | r | 54700 | 4 | OK |  |
| 10 | Entity | 0x4a8 | attack_effect.growth_range | r | 54702. (level-1)*growth_range 가산 | 4 | OK |  |
| 11 | Entity | 0x5c8 | level (champ) | r | 54704·54747·54769. skill2_effect 는 level>2 일 때만 Some(54771) | 4 | OK |  |
| 12 | Entity | 0x438 | stat_buff_cached.range (champ) | r | 54708·54751·54795. 기본 사거리에 가산 | 4 | OK |  |
| 13 | Entity | 0x4f8 | skill_effect@tag (니치 -1=None) | r | 54730 | 4 | OK |  |
| 14 | Entity | 0x4d8 | skill_effect.range | r | 54743 | 4 | OK |  |
| 15 | Entity | 0x4e0 | skill_effect.growth_range | r | 54745 | 4 | OK |  |
| 16 | Entity | 0x4f0 | skill_effect.target (CastingTarget) | r | 54843 → CastingTarget::check 의 self(55205) | 4 | OK |  |
| 17 | Entity | 0x500 | skill2_effect (Option<Effect> 56B 선두) | r | 54772. level>2 아니면 @anon.38(정적 None, 태그 -1) 로 대체(54773) | 4 | OK |  |
| 18 | Entity | 0x510 | skill2_effect.range | r | 54789 (%118+16) | 4 | OK |  |
| 19 | Entity | 0x518 | skill2_effect.growth_range | r | 54791 (%118+24) | 4 | OK |  |
| 20 | Entity | 0x528 | skill2_effect.target (CastingTarget) | r | 54846 (%118+40) → CastingTarget::check(55309) | 4 | OK |  |
| 21 | Entity | 0x530 | skill2_effect@tag (니치 -1=None) | r | 54776 (%118+48) | 4 | OK |  |
| 22 | Entity | 0x0 | team@tag (champ, TeamType) | r | 54975. 0=Player 1=Neutral. Neutral 이면 가시성 검사 생략 | 4 | OK |  |
| 23 | Entity | 0x8 | team@Player.0 (champ) | r | 54836/55007. target.visible_state 인덱스(bounds<2 · 55009) | 4 | OK |  |
| 24 | Entity | 0x38 | visible_state[team]@tag (target) | r | 55014 + team*24(gep stride). 0=Visible 만 통과(55017) | 4 | OK |  |
| 25 | Entity | 0x660 | x (champ·target) | r | 54837 / 55033 | 4 | OK |  |
| 26 | Entity | 0x668 | y (champ·target) | r | 54838 / 55036 | 4 | OK |  |
| 27 | Entity | 0x68 | ty@tag (target) | r | 55061. ==1(Minion) 검사 — 스캔 반경 밖일 때만 | 4 | OK |  |
| 28 | Entity | 0x11a | ty@Minion.info.line@tag (target) | r | 55073. == line 인자(55077) | 4 | OK |  |
| 29 | Entity | 0x5c0 | id (target) | r | 55141·55252·55363 → SmallAction*::new 의 target | 4 | OK |  |
| 30 | sret Vec | 0x0..0x20 | res | w | sret 외 힙 쓰기 없음. 인자 전부 readonly | 4 | 확인불가(tcx 사전에 타입 없음) | 빈 Vec 초기화(54592~54598) → push 시 +0x18 len 갱신(55193·55304·55415), 원소 memcpy 184B(55191·55302·55413) · 용량 부족 시 reserve_internal_or_panic(55170·55281·55392) |
| 31 | prof::PHASE_NANOS / PHASE_CALLS | +520 (=65*8) | phase 65 계측 | w | 54997~54998. prof::ENABLED!=0 일 때만(54570~54571) | 4 | 확인불가(tcx 사전에 타입 없음) | atomicrmw add nanos / +1 |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 65 | 74 | 산출값 | ProfTimer phase id(계측 전용, 판정 무관) · 54578 | 4 |
| 1 | 100 | 81 | 계수 | Entity::radius() 인라인: radius*(radius_mult+100)/100 (entity.rs:1515) · 54667/54669 등 4곳 | 4 |
| 2 | 2 | 89 | 임계 | level > 2 이어야 skill2_effect 유효(entity.rs:1693) · 54771 | 4 |
| 3 | 30 | 92 | 계수 | ★ 사거리 여유 = move_speed*30 (틱 단위 추정: 60tps 면 0.5초 이동분). Attack/Skill/Skill2 세 판정에 공통 가산 · 54839 | 5 |
| 4 | 6400000000 | 98 | 임계 | 80000^2 — near_default_scan 반경(제곱비교). 이 안이면 라인 무관하게 후보, 밖이면 `line` 라인 미니언만 · 55053 | 4 |
| 5 | 1 | 99 | 태그 | EntityType 태그 1 = Minion (tcxdict --enum EntityType) · 55063 | 3 |
| 6 | 0 | 93 | 태그 | VisibleState 태그 0 = Visible · 55017 / TeamType 태그 0 = Player(trunc→i1 · 54976) | 4 |
| 7 | 15 | 110 | 센티널 | SmallActionPlay 니치 태그 15 = Attack(원소+0xb1) · 55148 | 4 |
| 8 | 16 | 119 | 센티널 | SmallActionPlay 니치 태그 16 = Skill · 55259 | 4 |
| 9 | 17 | 129 | 센티널 | SmallActionPlay 니치 태그 17 = Skill2 · 55370 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 사거리 여유(이동분) 계수 | lane_minion.rs:92 (IR 54839 `mul i64 %50, 30`) | 30 | 올리면 더 먼 미니언까지 Attack/Skill/Skill2 후보에 들어간다(세 판정 공통). 0 이면 순수 사거리+반지름만 | 4 | 기존 |
| 1 | 근접 스캔 반경(제곱) | lane_minion.rs:98 (IR 55053) | 6400000000 | 80000 이내 미니언은 라인과 무관하게 후보. 올리면 다른 라인 미니언도 근처면 후보에 포함, 내리면 담당 라인 미니언만 남는다 | 4 | 기존 |
| 2 | skill2 해금 레벨 | entity.rs:1693 인라인 (IR 54771) | 2 | level > 2 조건. game_core 규칙이라 AI 쪽에서 바꿀 값이 아님(참고용) | 4 | 기존 |

<details><summary>`callees` 피호출자 14건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | line_minion_action_candidates | game_ai::line_minion_action_candidates | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\small_action\lane_minion.rs:72 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 6 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 12 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | should_suppress_direct_minion_attack | game_ai::SmallActionLaneMinionPosition::should_suppress_direct_minion_attack | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool | game-ai\src\small_action\lane_minion.rs:152 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 3개**: `elapsed`, `move_speed`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m02.ll:47712, m14.ll:22764, m15.ll:21793) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | iter_minions 의 56B 반환 구조 — 슬라이스 3개(cur/end 쌍 3 + i64 상태) 체인으로 읽히나(54811~54833) 세 슬라이스가 라인별(Top/Mid/Bot)인지는 _gcbc 본문 미확인(콜리 내부 안 팜 · 지시 범위 밖) | 4 |  |
| 1 | 미탐색 | should_suppress_direct_minion_attack 내부 — 잎 22, 계약만: (version:i64, data:&OperationData, player:&PlayerState, line:i8, target:&Entity) -> bool(true=Attack 후보 억제) · 55132 | 4 |  |
| 2 | 미탐색 | L84/87/90 의 사거리 합산 헬퍼가 effect.rs:1162 의 어떤 이름인지(인라인·define 없음). 식 자체는 IR 로 확정: range + champ_radius + stat_buff_cached.range + (level-1)*growth_range | 4 |  |
| 3 | 미탐색 | 상수 30 의 단위 — 틱이면 60tps 기준 0.5초. tps 나눗셈 없이 곧바로 곱하므로 '틱' 은 추정 | 5 |  |
| 4 | 미탐색 | SmallActionAttack/Skill/Skill2::new 가 24B 안에 무엇을 쓰는지(start_tick·target·is_act 로 tcxdict 확정, 값은 콜리 내부 · 안 팜) | 3 |  |
| 5 | 미탐색 | entity.rs:1482 가시성 헬퍼의 이름(인라인). 동작은 IR 로 확정: champ.team==Player(t) 이면 target.visible_state[t]==Visible 요구 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

