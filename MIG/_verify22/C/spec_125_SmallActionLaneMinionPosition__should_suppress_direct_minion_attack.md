---

### `125` SmallActionLaneMinionPosition::should_suppress_direct_minion_attack — 사거리 밖의 같은 라인 적 미니언(target)에 대해 '직접 걸어가 때리기'를 억제할지 — 직접공격 스탠스 좌표가 있고 라인 위험이 없을 때 has_safe_minion_attack_stance 결과를 그대로 반환

| 항목 | 값 |
|---|---|
| id | `lane_minion__should_suppress_direct_minion_attack` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai12small_action11lane_minionNtB2_29SmallActionLaneMinionPosition36should_suppress_direct_minion_attack` |
| 소스 | `game-ai\src\small_action\lane_minion.rs:152` |
| IR | `m11.ll` 45771~46032행 |
| 경로·가시성 | `game_ai::SmallActionLaneMinionPosition::should_suppress_direct_minion_attack` · **pub** |
| 계층 | 기타 |
| exe | `e269a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | i64(usize) | 본 함수 본문에선 분기 없음. has_safe_minion_attack_stance 에 그대로 전달(m11.ll:46030). lane_stance_risk 에는 `i64 poison` 으로 전달(46020) = 그 콜리가 version 을 안 쓴다는 컴파일러 판단 | 4 |
| 1 | 2 | data | &OperationData(24B) | +0x0 cache(&AbstractGameWithCache) → player_champion 조회 / +0x8 context(&GameContext) → direct_attack_stance 에 전달 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team / info.position 만 읽어 내 챔피언 엔티티를 찾음 | 4 |
| 3 | 4 | line | i8 LineType 태그 range(0,3) | 0=Top 1=Mid 2=Bottom (tcxdict --enum LineType). target 미니언의 line 태그와 == 비교 | 3 |
| 4 | 5 | target | &Entity(1728B) | 후보 적 미니언. dbg 이름 target(45777) — 이후 `self`로도 여러 번 바인딩(인라인 메서드의 self) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn should_suppress_direct_minion_attack(version, data:&OperationData, player:&PlayerState, line:LineType, target:&Entity) -> bool
  let champ = data.cache.player_champion[player.info.team][player.info.position.as_index()]?  // L153: None → false (45801)
  // L156: 팀 비교 + 가시성 (entity.rs:1127 eq / 1482~1483 is_visible_from / 1136~1137 player_team 인라인)
  if target.team == champ.team { return false }            // 태그 같고 (Player 면 팀번호까지 같음 | 둘 다 Neutral) → false (45827, 45856)
  match champ.team {
    Player(t) => { if target.visible_state[t] != Visible { return false } }   // 45862~45868 (t<2 아니면 panic 45871)
    Neutral   => { /* 가시성 검사 생략 — champ 는 player_champion 이라 실전 도달 불가 경로 */ }  // 45822→45840
  }
  if target.ty.tag != Minion(1) { return false }           // L159 (45843; Player 경로에선 L156 select 에 병합 45866)
  if target.ty.Minion.info.line != line { return false }    // L162 (45884)
  let atk = champ.attack_effect.as_ref()?                    // L165: None(-1) → false (45892)
  if !atk.target.check(champ, target) { return false }       // L168 CastingTarget::check (45899)
  // L172 attack_range = champ.range_to(target) (effect.rs:26 range() 인라인)
  let attack_range = champ.stat_buff_cached.range + atk.range + (champ.level-1)*atk.growth_range
                   + Effect::range_adjust(atk, champ, target) + radius(champ) + radius(target)   // 45964~45968
     where radius(e) = if e.radius_mult==0 { e.radius } else { e.radius*(radius_mult+100)/100 }   // entity.rs:1511~1515
  // L173: 이미 사거리 안이면 억제 안 함
  if distance_sq(target.xy, champ.xy) <= attack_range^2 { return false }   // ugt 가 true 일 때만 계속 (46000~46001)
  // L177
  let Some((walk_x, walk_y)) = Self::direct_attack_stance(data.context, champ.x, champ.y, target.x, target.y, attack_range) else { return false }  // L178 (46007~46010, 46027)
  // L180: 스탠스 지점의 라인 위험이 있으면(Some) 억제 안 함
  if Self::lane_stance_risk(<unused>, player, data, champ, walk_x, walk_y).is_some() { return false }   // Option<i64> 태그 1=Some → false (46020~46023)
  // L184
  return Self::has_safe_minion_attack_stance(version, player, data, champ, target, line, attack_range)   // 46030
분기 극성 근거: 45801 null→%66(false) / 45856 팀번호 eq→%66 / 45868 select(visible&&minion) false→%66 / 45885 line ne→%66 / 45893 -1→%66 / 45900 check false→%66 / 46001 dist≤range→%66 / 46010 Option None→%149→%66 false / 46023 Some→%66 false / 46031 %151 그대로.
```

**`mem` 메모리 접근 21건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. player_champion 1차 인덱스. <2 아니면 panic_bounds_check(45783~45787) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 태그 → as_index()(player.rs:581 인라인) → player_champion 2차 인덱스(45792~45794) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache 로드(45795) | 4 | OK |
| 3 | OperationData | 0x8 | context | r | &GameContext 로드 → direct_attack_stance 인자(46005~46007) | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [2][5] x Option<&Entity>(니치 null=None). None 이면 false(45797~45801) | 4 | OK |
| 5 | Entity | 0x0 | team@tag | r | target·champ 양쪽. TeamType 태그 0=Player/1=Neutral (45811, 45814) | 4 | OK |
| 6 | Entity | 0x8 | team@Player.0 | r | 팀 번호 usize. 태그가 둘 다 Player 일 때 == 비교(45853~45855); champ 쪽은 player_team() 으로도 로드(45830) | 4 | OK |
| 7 | Entity | 0x38 | visible_state[team]@tag | r | target.visible_state[champ_team]. stride 24B(gepS 45861). 태그 0=Visible 이어야 통과(45862~45863) | 4 | OK |
| 8 | Entity | 0x68 | ty@tag | r | target 의 EntityType 태그. ==1(Minion) 이어야 통과(45841~45843 L159, 45864~45866 은 컴파일러가 L156 select 로 병합) | 4 | OK |
| 9 | Entity | 0x11a | ty@Minion.info.line@tag | r | target 미니언의 LineType 1B 태그. 인자 line 과 == (45880~45884, L162). ty==Minion 확정 문맥에서만 읽힘(switch 접힘) | 4 | OK |
| 10 | Entity | 0x4c0 | attack_effect@tag(니치) | r | champ 의 attack_effect Option 니치 태그(i32 -1 = None → false, 45890~45893, L165) | 4 | OK |
| 11 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect(56B) 시작 = atk. range_adjust 인자(45917) | 4 | OK |
| 12 | Entity | 0x4b8 | attack_effect@Some.0.target | r | CastingTarget(4B) → CastingTarget::check 의 self(45898~45899, L168) | 4 | OK |
| 13 | Entity | 0x4a0 | attack_effect@Some.0.range | r | champ. 사거리 합산 항(45907~45908, effect.rs:26 range() 인라인) | 4 | OK |
| 14 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | champ. (level-1)*growth_range(45909~45914) | 4 | OK |
| 15 | Entity | 0x5c8 | level | r | champ 레벨(45911~45913) | 4 | OK |
| 16 | Entity | 0x438 | stat_buff_cached.range | r | champ 기본 사거리 스탯(45915~45916) | 4 | OK |
| 17 | Entity | 0x470 | stat_buff_cached.radius_mult | r | champ·target 양쪽. i32, 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 (entity.rs:1511~1515 인라인, 45918~45963) | 4 | OK |
| 18 | Entity | 0x680 | radius | r | champ·target 양쪽 몸통 반경(45925, 45932, 45948, 45955) | 4 | OK |
| 19 | Entity | 0x660 | x | r | target(45970)·champ(45978) 좌표 → distance_sq | 4 | OK |
| 20 | Entity | 0x668 | y | r | target(45974)·champ(45982) 좌표 → distance_sq | 4 | OK |

**`consts` 상수 4건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 159 | 태그 | EntityType 메모리태그 1 = Minion (tcxdict --enum EntityType). target.ty==Minion 아니면 false (45843, 45866). 같은 값 1 이 45822 에선 TeamType Neutral 비트(trunc i1), 46009 에선 direct_attack_stance 반환 Option 태그 Some, 46022 에선 lane_stance_risk 반환 Option<i64> 태그 Some | 3 |
| 1 | 0 | 156 | 태그 | VisibleState 메모리태그 0 = Visible (45863) — target.visible_state[champ_team]==Visible 이어야 통과. 45826 의 0 은 TeamType Player 태그 | 4 |
| 2 | -1 | 165 | 센티널 | attack_effect Option 니치 None(i32 태그 -1, 45892). champ 에 평타 이펙트 없으면 false | 4 |
| 3 | 100 | 172 | 계수 | 반경 배율 퍼센트 — radius*(radius_mult+100)/100 (entity.rs:1515 인라인, 45934~45936·45957~45959). 임계 아님 | 4 |

**`knobs` 조정점 0건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|

<details><summary>`callees` 피호출자 12건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | direct_attack_stance | game_ai::SmallActionLaneMinionPosition::direct_attack_stance | in:game_ai | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity, u64) -> std::option::Option<(u64, u64)> | game-ai\src\small_action\lane_minion.rs:212 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 5 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 6 | has_safe_minion_attack_stance | game_ai::SmallActionLaneMinionPosition::has_safe_minion_attack_stance | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64) -> bool | game-ai\src\small_action\lane_minion.rs:226 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | lane_stance_risk | game_ai::SmallActionLaneMinionPosition::lane_stance_risk | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<i64> | game-ai\src\small_action\lane_minion.rs:380 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 9 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 10 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | should_suppress_direct_minion_attack | game_ai::SmallActionLaneMinionPosition::should_suppress_direct_minion_attack | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool | game-ai\src\small_action\lane_minion.rs:152 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `range_to`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:55132) · **형제 20개** (SmallActionLaneMinionPosition)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionLaneMinionPosition as std::clone::Clone>::clone | pub | game-ai\src\small_action\lane_minion.rs:8 | True | fn(&game_ai::SmallActionLaneMinionPosition) -> game_ai::SmallActionLaneMinionPosition |
| 1 | <game_ai::SmallActionLaneMinionPosition as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\lane_minion.rs:8 | True | fn(&game_ai::SmallActionLaneMinionPosition, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionLaneMinionPosition::new | pub | game-ai\src\small_action\lane_minion.rs:139 | False | fn(&game_core::OperationData, usize, usize, i64, game_ai::PositionEvalPurpose) -> game_ai::SmallActionLaneMinionPosition |
| 3 | game_ai::SmallActionLaneMinionPosition::should_suppress_direct_minion_attack | pub | game-ai\src\small_action\lane_minion.rs:152 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool |
| 4 | game_ai::SmallActionLaneMinionPosition::has_current_explicit_minion_action | in:game_ai | game-ai\src\small_action\lane_minion.rs:187 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool |
| 5 | game_ai::SmallActionLaneMinionPosition::can_use_minion_action_now | in:game_ai | game-ai\src\small_action\lane_minion.rs:208 | False | fn(bool, std::option::Option<&game_core::Effect>, &game_core::Entity, &game_core::Entity) -> bool |
| 6 | game_ai::SmallActionLaneMinionPosition::direct_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:212 | False | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity, u64) -> std::option::Option<(u64, u64)> |
| 7 | game_ai::SmallActionLaneMinionPosition::has_safe_minion_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:226 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64) -> bool |
| 8 | game_ai::SmallActionLaneMinionPosition::is_safe_minion_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:272 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64, u64, u64) -> bool |
| 9 | game_ai::SmallActionLaneMinionPosition::target_score | in:game_ai | game-ai\src\small_action\lane_minion.rs:294 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Effect, &game_core::Entity, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut rand::rngs::std::StdRng) -> std::option::Option<i64> |
| 10 | game_ai::SmallActionLaneMinionPosition::attack_range | in:game_ai | game-ai\src\small_action\lane_minion.rs:354 | False | fn(&game_core::Entity, &game_core::Entity) -> std::option::Option<u64> |
| 11 | game_ai::SmallActionLaneMinionPosition::local_position_score | in:game_ai | game-ai\src\small_action\lane_minion.rs:359 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> i64 |
| 12 | game_ai::SmallActionLaneMinionPosition::lane_stance_risk | in:game_ai | game-ai\src\small_action\lane_minion.rs:380 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<i64> |
| 13 | game_ai::SmallActionLaneMinionPosition::push_candidate | in:game_ai | game-ai\src\small_action\lane_minion.rs:439 | False | fn(&mut bumpalo::collections::vec::Vec< (u64, u64, i64)>, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose, u64, u64, i64, game_core::LineType, i64) |
| 14 | game_ai::SmallActionLaneMinionPosition::choose_goal | in:game_ai | game-ai\src\small_action\lane_minion.rs:493 | False | fn(&game_ai::SmallActionLaneMinionPosition, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, game_core::LineType) -> std::option::Option<(u64, u64, i64)> |
| 15 | game_ai::SmallActionLaneMinionPosition::get_input | in:game_ai | game-ai\src\small_action\lane_minion.rs:540 | False | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 16 | game_ai::SmallActionLaneMinionPosition::merge | in:game_ai | game-ai\src\small_action\lane_minion.rs:635 | False | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, game_ai::SmallActionLaneMinionPosition) |
| 17 | game_ai::SmallActionLaneMinionPosition::get_action | in:game_ai | game-ai\src\small_action\lane_minion.rs:652 | True | fn(&game_ai::SmallActionLaneMinionPosition) -> game_core::SmallAction |
| 18 | game_ai::SmallActionLaneMinionPosition::is_end | in:game_ai | game-ai\src\small_action\lane_minion.rs:656 | False | fn(&game_ai::SmallActionLaneMinionPosition, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 19 | game_ai::SmallActionLaneMinionPosition::near_move_complete | in:game_ai | game-ai\src\small_action\lane_minion.rs:685 | False | fn(&game_ai::SmallActionLaneMinionPosition, &game_core::Entity) -> bool |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | has_safe_minion_attack_stance(m11.ll:45190, internal fastcc, i1 반환)·direct_attack_stance(m11.ll:45044, sret 24B Option<(i64,i64)>)·lane_stance_risk(m11.ll:44468, {i64,i64}=Option<i64>) 의 내부는 본 명세 범위 밖 — 시그니처·반환 형태만 호출부에서 확정. 최종 true 의 의미('안전한 대체 스탠스가 있어 직접 공격을 억제')는 이름 기반 추정 | 4 |  |
| 1 | 미탐색 | lane_stance_risk 의 Option<i64> 페이로드(위험 수치?)는 본 함수에서 안 읽음(태그만 검사, 46021~46022) | 4 |  |
| 2 | 미탐색 | direct_attack_stance 첫 인자 = data.context(&GameContext) 이나 `captures(address_is_null)` 속성 — 콜리가 null 검사만 하는지 미확인 | 4 |  |
| 3 | 미탐색 | champ.team==Neutral 경로(45822 true→45840)는 소스 `player_team()` 이 Option 을 돌려주는 형태로 추정(None 이면 가시성 검사 없이 통과) — 소스 표기는 미확정. 실전에선 player_champion 이 항상 Player 팀이라 도달 불가(추정) | 5 |  |
| 4 | 미탐색 | attack_range 합산 순서·`range_adjust` 의 정확한 반환 단위(부호)는 game_core 쪽 — 여기선 i64 add 로만 관측(45917, 45966) | 4 |  |
| 5 | 미탐색 | 판정 임계 상수 없음(모든 비교가 필드 대 필드/태그) → knobs 비움. 조정 지점은 콜리 3개 내부 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

