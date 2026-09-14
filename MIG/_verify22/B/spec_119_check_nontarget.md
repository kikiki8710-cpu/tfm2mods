---

### `119` check_nontarget — 논타겟(위치/방향) 스킬이 이 대상에 맞을 사거리인가 — 대상이 못 움직이면 정적 사거리, 움직이면 투사체 도달시간·대상 이속·선수 스킬명중 오차 롤을 반영한 유효사거리로 판정

| 항목 | 값 |
|---|---|
| id | `cast__check_nontarget` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai12small_action4cast15check_nontarget` |
| 소스 | `game-ai\src\small_action\cast.rs:38` |
| IR | `m07.ll` 60191~60473행 |
| 경로·가시성 | `game_ai::small_action::cast::check_nontarget` · **in:game_ai::small_action::cast** |
| 계층 | 기타 |
| exe | `d9f3c0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Effect, &game_core::Entity) -> bool
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize | 본문 미사용(이름부터 `_`). DILocalVariable arg:1 만 남음 | 4 |
| 1 | 2 | rnd | &mut StdRng(320B) | ★&mut — 롤 경로에서만 gen_range 1회로 상태 소비(writes 참조) | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team·info.position·info.parameter(AthleteParameter) | 4 |
| 3 | 4 | data | &OperationData(24B) | cache.player_champion[team][pos] 만 읽음. context/blackboard 미사용 | 4 |
| 4 | 5 | action | &Effect(56B) | casting 종류·ty(Arc<dyn EffectType>)·range·growth_range·start_timing | 4 |
| 5 | 6 | target | &Entity(1728B) | 대상 엔티티(ty 태그로 이동가능 여부·x/y·move_speed·radius) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn check_nontarget(_version, rnd: &mut StdRng, player, data, action: &Effect, target: &Entity) -> bool {
  // L39 — 논타겟(Position/Direction) 이고 투사체 속도가 정의된 스킬만 검사
  if !(action.is_nontarget() && action.ty.linear_move_speed().is_some()) { return true; }   // casting∈{1,2} · vtable+0xf8 Some
  // L43
  let champ = data.cache.player_champion[player.info.team][player.info.position].unwrap();   // 캐스터
  // L45 — 유효 사거리
  let range = action.range(champ)                        // = action.range + (champ.level-1)*action.growth_range + champ.stat_buff_cached.range   (effect.rs:26)
            + action.range_adjust(champ, target)         // game_core g06.ll:51785
            + target.radius();                           // radius_mult==0 ? radius : radius*(mult+100)/100   (entity.rs:1511)
  // L47, L49
  let target_speed = target.stat_cached.move_speed;
  let apply_time = action.start_timing;
  // L54 — 대상이 움직일 수 있나 (can_move entity.rs:1490 + is_in_action entity.rs:1549 인라인)
  let movable = match target.ty.tag {
      None|Tower|Nexus|Epic|Serpen (0,3,2,5,6) => target.rush_state == Rush,   // +0x308 == Rush 니치
      Champion (13)                            => target.action_state.tag < 3,   // Idle/Return/Move
      _ (Minion,Jungle,Ghoul,SmallJiangshi,Bear,Eagle,Revenant,Illusion) => true,
  };
  if !movable {
      // L68 — 정적: 제곱거리 비교 (rnd 미소비)
      return dist2(champ.xy, target.xy) <= range*range;
  }
  // L56 — 투사체 선형 이동속도
  let linear_move_speed = max(action.ty.linear_move_speed().unwrap(), 1);
  // L58 — 도달까지 걸리는 틱(10틱 공제)을 발동시점에 더함
  let apply_time = apply_time + (champ.distance(target) / linear_move_speed).saturating_sub(10);
  // L60~62 — 선수 스킬 명중 오차 롤
  let skill_hit_accuracy = player.info.parameter.skill_hit_accuracy();   // 150..=1000
  let range_error = max(1000 - skill_hit_accuracy, 1);
  let error = rnd.gen_range((1000 - range_error)..=(1000 + range_error));   // ★rnd 1회 소비
  // L63~64 — 대상이 도달시간 동안 도망가는 거리를 빼고 오차 배율
  let v = apply_time * target_speed;
  let r = error * range.saturating_sub(v) / 1000;
  // L66
  dist2(champ.xy, target.xy) <= r*r
}

분기 순서(IR): %10(casting∈{1,2}) → %25(linear_move_speed Some) → %29(team bounds) → %40(champ null→panic) → %55(radius_mult==0) → switch(ty) → %83(rush==Rush)/%109(action_state<3) → 정적 %84 / 롤 %110~%115. dist2 = |dx|²+|dy|² (Entity.x/y u64, abs-diff).
```

**`mem` 메모리 접근 23건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | Effect(action) | 0x30 | casting@tag | r | i32 CastingType: (tag-1)<2 ⇔ tag∈{1 Position, 2 Direction} = is_nontarget (type.rs:149 인라인) (m07:60207~60211, L39) | 4 | OK |  |
| 1 | Effect(action) | 0x0 | ty.ptr (Arc<dyn EffectType> data) | r | ArcInner: (cap-1)&-16 정렬 + 16 = 페이로드 (m07:60216~60224) | 4 | OK |  |
| 2 | Effect(action) | 0x8 | ty.vtable | r | vtable+0xf8 = EffectType::linear_move_speed(&self)->Option<usize> ({i64 tag,i64 v}) — divtable EffectType 0xf8 (m07:60225~60229 L39 · m07:60385 L56) | 3 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 3 | Effect(action) | 0x10 | range | r | Effect::range(caster) 인라인(effect.rs:26) (m07:60262~60263, L45) | 4 | OK |  |
| 4 | Effect(action) | 0x18 | growth_range | r | *(caster.level-1) (m07:60264~60269, L45) | 4 | OK |  |
| 5 | Effect(action) | 0x20 | start_timing | r | dbg `apply_time` 초기값 (m07:60309~60310, L49) | 4 | OK |  |
| 6 | PlayerState | 0x930 | info.team | r | <2 아니면 panic_bounds_check (m07:60233~60239, L43) | 4 | OK |  |
| 7 | PlayerState | 0x9c0 | info.position@tag | r | [5 x ptr] 인덱스 (m07:60244~60246, L43) | 4 | OK |  |
| 8 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B) | r | AthleteParameter::skill_hit_accuracy(&self) 인자 (m07:60409~60410, L60) | 4 | OK |  |
| 9 | AbstractGameWithCache(승격 %2) | 0x1e0 | player_champion[team][position]@Some.0 | r | null → unwrap_failed 패닉(L43 `.unwrap()`) — 캐스터 champ (m07:60249~60255,60280) | 4 | OK |  |
| 10 | Entity(champ=캐스터) | 0x5c8 | level | r | (level-1)*growth_range (m07:60266~60269) | 4 | OK |  |
| 11 | Entity(champ=캐스터) | 0x438 | stat_buff_cached.range | r | 사거리 가산 (m07:60270~60271) | 4 | OK |  |
| 12 | Entity(champ=캐스터) | 0x660 | x | r | dist² 계산 (m07:60342~60343 L68 · 60437~60438 L66) | 4 | OK |  |
| 13 | Entity(champ=캐스터) | 0x668 | y | r | (m07:60346~60347 · 60441~60442) | 4 | OK |  |
| 14 | Entity(target) | 0x470 | stat_buff_cached.radius_mult | r | i32 · Entity::radius() 인라인(entity.rs:1511~1515): 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 (m07:60273~60296, L45) | 4 | OK |  |
| 15 | Entity(target) | 0x680 | radius | r | usize (m07:60284~60285, 60291~60292) | 4 | OK |  |
| 16 | Entity(target) | 0x640 | stat_cached.move_speed | r | dbg `target_speed` (m07:60306~60307, L47) | 4 | OK |  |
| 17 | Entity(target) | 0x68 | ty@tag | r | EntityType 태그 switch (can_move 인라인 entity.rs:1490 · m07:60313~60330, L54) | 4 | OK |  |
| 18 | Entity(target) | 0x308 | rush_state@tag (니치) | r | None/Tower/Nexus/Epic/Serpen(태그 0,3,2,5,6) 일 때 == -9223372036854775805(=u64 9223372036854775811 = RushState::Rush) 이면 '이동 가능' (m07:60336~60339, L54 · can_move) | 4 | OK |  |
| 19 | Entity(target) | 0x70 | ty@Champion.0.action_state@tag | r | Champion(13) 일 때 <3(Idle/Return/Move) 이면 이동 가능, ≥3(Attack/Skill/Skill2/Ult) 이면 정지 (is_in_action 인라인 entity.rs:1549 · m07:60377~60380, L54) | 4 | OK |  |
| 20 | Entity(target) | 0x660 | x | r | (m07:60350~60351 · 60445~60446) | 4 | OK |  |
| 21 | Entity(target) | 0x668 | y | r | (m07:60354~60355 · 60449~60450) | 4 | OK |  |
| 22 | StdRng(rnd, &mut 320B) | 0x0 -> rand 내부 상태(ChaCha 블록 버퍼·인덱스) | rnd | w | ★유일한 부작용. 롤 경로(%110, target 이동 가능)에서만 정확히 1회(m07:60427, L62). 정적 경로(%84)·조기 true(L39) 에선 rnd 를 건드리지 않는다 → rnd 스트림 비트동일 재현 시 이 분기 조건이 결정적 | 1 | 오귀속(사전은 다른 필드를 준다) | gen_range::<usize, RangeInclusive<usize>>(rnd, (1000-range_error)..=(1000+range_error)) 1회 호출로 소비 |

**`consts` 상수 11건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 39 | 임계 | `(casting-1) <u 2` = CastingType∈{Position(1),Direction(2)} — is_nontarget(type.rs:149) 접힘. 아니면(Targeting/None) true 반환 (m07:60209~60210) | 4 |
| 1 | -1 | 39 | 계수 | 위 범위검사의 `add nsw i32 %8, -1` 및 ArcInner 정렬 `(cap-1)&-16` (m07:60209,60221) — 판정값 아님 | 4 |
| 2 | 1 | 39 | 태그 | linear_move_speed() 반환 Option<usize> 태그 1=Some (`icmp eq i64 %24, 1` m07:60229). None 이면 true 반환 | 4 |
| 3 | 2 | 43 | 임계 | info.team bounds-check len 2 (m07:60235,60239) — 판정값 아님 | 4 |
| 4 | 100 | 45 | 계수 | Entity::radius 인라인: radius*(radius_mult+100)/100 (m07:60293~60295, entity.rs:1515) | 4 |
| 5 | -9223372036854775805 | 54 | 센티널 | RushState 니치 태그 = Rush(idx3, u64 9223372036854775811). can_move(entity.rs:1490): None/Tower/Nexus/Epic/Serpen 는 Rush 중일 때만 이동 가능 (m07:60338) | 4 |
| 6 | 3 | 54 | 태그 | ChampionActionState 태그 <3 (Idle0/Return1/Move2) = 행동 중 아님 → 이동 가능. is_in_action(entity.rs:1549) (m07:60379) | 4 |
| 7 | 10 | 58 | 태그 | ★투사체 도달시간 보정: apply_time += (distance/linear_move_speed).saturating_sub(10) — 비행 틱에서 10틱 공제 (m07:60406~60407) | 4 |
| 8 | 1 | 56 | 태그 | 0 나눗셈 바닥 umax(linear_move_speed,1)(m07:60401) · umax(range_error,1)(m07:60415 L61) | 4 |
| 9 | 1000 | 61 | 계수 | ★명중 스케일: range_error = 1000 - skill_hit_accuracy(150..1000); error = rnd.gen_range((1000-range_error)..=(1000+range_error)); r = error * (range - apply_time*target_speed).saturating_sub / 1000 (m07:60412,60418,60420,60435, L61~64) | 4 |
| 10 | 0 | 62 | 태그 | RangeInclusive.exhausted=false 초기화 `store i8 0` (m07:60426) — 판정값 아님 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 투사체 비행 틱 공제 | cast.rs:58 (m07:60406) | 10 | 올리면 도달시간 보정이 줄어 멀리 있는 이동 대상에게도 시전 허용(빗나감↑). 내리면(0) 보수적 | 4 | 기존 |
| 1 | 명중 오차 스케일 | cast.rs:61~64 (m07:60412,60418,60420,60435) | 1000 | skill_hit_accuracy(150..1000) 를 밀리 단위로 해석. 1000 이 곧 '오차 0' 기준 — 바꾸면 accuracy 스탯 의미가 통째로 바뀜(스케일 상수, 노브로 부적합) | 4 | 기존 |
| 2 | 행동 중 판정 임계 | entity.rs:1549 is_in_action 인라인 (m07:60379) | 3 | ChampionActionState 태그 ≥3(Attack/Skill/Skill2/Ult) 은 정지로 봐 정적 사거리 — 내리면(2) Move 중인 챔피언도 정지 취급 → 롤 없이 판정 | 4 | 기존 |
| 3 | Rush 니치 태그 | entity.rs:1490 can_move 인라인 (m07:60338) | -9223372036854775805 | 구조물/에픽/세르펜은 Rush 상태에서만 이동 취급 — 상수 자체는 열거형 레이아웃이라 노브 아님 | 4 | 기존 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | check_nontarget | game_ai::small_action::cast::check_nontarget | in:game_ai::small_action::cast | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Effect, &game_core::Entity) -> bool | game-ai\src\small_action\cast.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | is_nontarget | game_core::CastingType::is_nontarget | pub | fn(&game_core::CastingType) -> bool | game-core\src\simulation\effect\type.rs:148 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | linear_move_speed | game_core::EffectType::linear_move_speed | pub | fn(&Self/#0) -> std::option::Option<usize> | game-core\src\simulation\effect\type.rs:347 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 4 | linear_move_speed | <game_core::RushEffect as game_core::EffectType>::linear_move_speed | pub | fn(&game_core::RushEffect) -> std::option::Option<usize> | game-core\src\simulation\effect\type\rush.rs:63 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 5 | linear_move_speed | <game_core::MoveToEffect as game_core::EffectType>::linear_move_speed | pub | fn(&game_core::MoveToEffect) -> std::option::Option<usize> | game-core\src\simulation\effect\type\move_to.rs:70 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 6 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | skill_hit_accuracy | game_core::AthleteParameter::skill_hit_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:330 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 6개**: `dist2`, `gen_range`, `llvm.assume`, `llvm.umax.i64`, `llvm.usub.sat.i64`, `range  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m07.ll:11905, m07.ll:12420, m07.ll:12759) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | `Effect::range_adjust(&Effect, caster:&Entity, target:&Entity)->usize`(g06.ll:51785) · `Entity::distance(&Entity,&Entity)->usize range(0,i64::MIN)`(g06.ll:84197) · `AthleteParameter::skill_hit_accuracy(&self 744B)->usize range(150,1001)`(g15.ll:125941) 내부 미독해(경계 콜리 — 시그니처만) | 4 |  |
| 1 | 미탐색 | EffectType::linear_move_speed 는 Arc<dyn> 런타임 vtable — divtable 은 CombineEffect impl 기준 슬롯 0xf8 이름만 확정, 실제 어느 구현체가 꽂히는지·None 을 돌려주는 효과 종류는 미확인 | 3 |  |
| 2 | 미탐색 | exe 측 인자 배치(internal fastcc + ArgumentPromotion) 는 argscan 미실행 — sweep 편입 시 `argscan.py 0xd9f3c0` 필요(EXE_ABI 는 IR 과 다를 수 있음) | 4 |  |
| 3 | 미탐색 | `_version` 이 IR 에서 제거됐으므로 버전 게이트 없음 — 모든 버전 동일 동작(IR 기준 확정, 소스에 `_version` 이라 의도적) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L54 의 정확한 소스 표기(`target.can_move() && !target.is_in_action()` 인지 단일 헬퍼인지) — dloc 은 1490 can_move 와 1549 is_in_action 둘 다 inlinedAt 54 를 가리킨다. 동작(위 movable 표)은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

