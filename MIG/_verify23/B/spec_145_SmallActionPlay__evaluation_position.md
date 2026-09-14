---

### `145` SmallActionPlay::evaluation_position — 소액션의 '평가 위치' — 액션 종류별 목표점을 구해, 공격/스킬계는 착지점 그대로, 그 외는 그 목표를 향해 1초(move_speed×tps)만큼 이동한 좌표를 Option<(x,y)> 로

| 항목 | 값 |
|---|---|
| id | `SmallActionPlay__evaluation_position` |
| 심볼 | `_RNvMNtCshdEBA0ozCnw_7game_ai12small_actionNtB2_15SmallActionPlay19evaluation_position` |
| 소스 | `game-ai\src\small_action.rs:245` |
| IR | `m11.ll` 41846~42097행 |
| 경로·가시성 | `game_ai::SmallActionPlay::evaluation_position` · **pub** |
| 계층 | 기타 |
| exe | `e23170` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::SmallActionPlay, usize, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)>
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<(u64,u64)>(24B) | +0 태그 i64(0 None/1 Some) · +8 x · +16 y. writeonly | 4 |
| 1 | 1 | self | &SmallActionPlay(184B) | +0xb1 태그 · 종류별 goal/target 좌표 필드 · Trace/Attack/Skill/Skill2/Ult 는 콜리에 통째 전달 | 4 |
| 2 | 2 | _version | usize | dbg 이름 없음(익명/미사용) — 본문에서 읽지 않음 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | +0x930 team · +0x9c0 position 태그 · expected_goal_position 인자 | 4 |
| 4 | 4 | data | &OperationData(24B) | +0x0 cache(player_champion) · +0x8 context(setting/map_setting/map → move_to) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L246: champ = data.cache.player_champion[player.team][player.position]?   — None → return None
L247: target: Option<(u64,u64)> = match self (idx=tag−3) {
  L248 Trace(11):  self.expected_goal_position(player, data).or_else(|| Some((trace.goal_x, trace.goal_y)))   [잎22 · sret 24B Option<(u64,u64)> · None 이면 +0x68/+0x70 폴백]
  L249 Attack(12): SmallActionAttack::movement_landing_position(self, champ, data)      [sret 24B Option<(u64,u64)>]
  L250 Skill(13):  SmallActionSkill::movement_landing_position(self, champ, data)
  L251 Skill2(14): SmallActionSkill2::movement_landing_position(self, champ, data)
  L252 Ult(15):    SmallActionUlt::movement_landing_position(self, champ, data)
  L253 Stop(16):   Some((champ.x, champ.y))   → L257 분기 없이 바로 move_to 경로(42020)
  L254 그 외:      self.target_position()  [인라인 small_action.rs:230~238]
      RunAway(0)/Positioning(6)/AroundPosition(7)/AroundPositionBush(8) → Some((+0x8, +0x10))
      Recall(1)                                                          → Some((+0x50, +0x58))
      Around(2)/AroundHide(3)/LaneMinionPosition(10)                    → Some((+0x10, +0x18))
      AroundBush(9)                                                      → Some((+0x18, +0x20))
      AroundRegion(4)/AroundRunAway(5)                                   → None
}?   — None → return None (42048)
L257: if idx ∈ {Attack,Skill,Skill2,Ult} (idx & !3 == 12) → return Some(target)   (L258 · 착지점 그대로)
L260~261: x = champ.x ; y = champ.y
L262~264: Entity::move_to(ctx.setting, ctx.map_setting, ctx.map, champ.id, &mut x, &mut y, champ.stat_cached.move_speed * setting.tick_per_second, target.x, target.y, &mut None(ptr null))
L265: return Some((x, y))   — 1초 뒤 예상 위치
```

**`mem` 메모리 접근 25건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 41861 (gep 2352) bounds 2 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 41872 (gep 2496) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | 41875 | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | 42060 → GameContext | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | 41876~41879 · null → None (option.rs:2790 `?`) | 4 | OK |  |
| 5 | GameContext | 0x8 | setting | r | 42062 → move_to 첫 인자 · +0x12f8 tick_per_second | 4 | OK |  |
| 6 | GameContext | 0x18 | map_setting | r | 42064 → move_to 둘째 인자 | 4 | OK |  |
| 7 | GameContext | 0x20 | map | r | 42066 → move_to 셋째 인자 | 4 | OK |  |
| 8 | GameSetting | 0x12f8 | tick_per_second | r | 42072 (gep 4856) × move_speed = 1초 이동량 | 4 | OK |  |
| 9 | SmallActionPlay | 0xb1 | @tag | r | 41887 (gep 177) → idx = tag>2 ? tag-3 : 7(AroundPosition 암묵) | 4 | OK |  |
| 10 | SmallActionPlay | 0x8 | RunAway/Positioning/AroundPosition.goal_x · AroundPositionBush.target_x | r | 41936 phi 8 (target_position 인라인 small_action.rs:230~238) | 4 | OK |  |
| 11 | SmallActionPlay | 0x10 | 위 variant goal_y · Around/AroundHide/LaneMinionPosition.goal_x | r | 41936/41937 phi 16 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 12 | SmallActionPlay | 0x18 | Around/AroundHide/LaneMinionPosition.goal_y · AroundBush.target_x | r | phi 24 | 4 | OK |  |
| 13 | SmallActionPlay | 0x20 | AroundBush.target_y | r | phi 32 | 4 | OK |  |
| 14 | SmallActionPlay | 0x50 | Recall.goal_x | r | phi 80 | 4 | OK |  |
| 15 | SmallActionPlay | 0x58 | Recall.goal_y | r | phi 88 | 4 | OK |  |
| 16 | SmallActionPlay | 0x68 | Trace.goal_x | r | 41977 (gep 104) — expected_goal_position 이 None 일 때 폴백(or_else 클로저) | 4 | OK |  |
| 17 | SmallActionPlay | 0x70 | Trace.goal_y | r | 41979 (gep 112) | 4 | OK |  |
| 18 | Entity | 0x660 | x | r | 42013/42040 champ.x (Stop 의 목표 · move_to 시작점) | 4 | OK |  |
| 19 | Entity | 0x668 | y | r | 42015/42042 champ.y | 4 | OK |  |
| 20 | Entity | 0x5c0 | id | r | 42068 → move_to id 인자 | 4 | OK |  |
| 21 | Entity | 0x640 | stat_cached.move_speed | r | 42070 (gep 1600) | 4 | OK |  |
| 22 | sret Option<(u64,u64)> | 0x0 | tag | w | &mut 인자 없음 — 쓰기는 sret 뿐. move_to 의 &mut x/&mut y 는 지역 alloca(%8/%7) | 4 | 확인불가(tcx 사전에 타입 없음) | 0 (41904 champ 없음 · 42048 target None) / 1 (42085 · 42095) |
| 23 | sret Option<(u64,u64)> | 0x8 | x | w | Some 경로만 | 4 | 확인불가(tcx 사전에 타입 없음) | 42082 move_to 후 x / 42092 착지점 x |
| 24 | sret Option<(u64,u64)> | 0x10 | y | w | Some 경로만 | 4 | 확인불가(tcx 사전에 타입 없음) | 42084 / 42094 |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 246 | 임계 | player_champion bounds team<2 (41863) | 4 |
| 1 | 12 | 257 | 태그 | idx & !3 == 12 ⇔ idx ∈ {12 Attack,13 Skill,14 Skill2,15 Ult} — 착지점을 그대로 반환하는 공격/스킬계 판정 (42035~42036, `and i8 -4`) | 4 |
| 2 | -4 | 257 | 계수 | idx & 0xFC 마스크 (42035) — 12..15 묶음 비교용 | 4 |
| 3 | 7 | 247 | 센티널 | 니치 untagged variant AroundPosition 의 논리 idx(41893 select) | 4 |
| 4 | 10 | 247 | 태그 | 태그 10 은 존재하지 않음(assume · 41889) — AroundPosition 은 암묵(태그 없음) | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 이동 투영 시간 | small_action.rs:263 | move_speed × tick_per_second (=1초) | 리터럴 없음(tps 곱). 곱을 늘리면 더 먼 미래 위치로 평가 — 코드 변경 없이 노브 없음 | 4 | 기존 |

<details><summary>`callees` 피호출자 6건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | expected_goal_position | game_ai::SmallActionTrace::expected_goal_position | in:game_ai | fn(&game_ai::SmallActionTrace, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\small_action\trace.rs:116 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | move_to | game_core::Entity::move_to | pub | fn(&game_core::GameSetting, &game_core::MapSetting, &game_core::MapDef, usize, &mut u64, &mut u64, u64, u64, u64, &mut std::option::Option<&mut game_core::GameFrameData>) | game-core\src\simulation\entity.rs:3427 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | movement_landing_position | game_ai::SmallActionUlt::movement_landing_position | in:game_ai | fn(&game_ai::SmallActionUlt, &game_core::Entity, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\small_action\cast.rs:255 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | movement_landing_position | game_ai::SmallActionSkill::movement_landing_position | in:game_ai | fn(&game_ai::SmallActionSkill, &game_core::Entity, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\small_action\cast.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | movement_landing_position | game_ai::SmallActionAttack::movement_landing_position | in:game_ai | fn(&game_ai::SmallActionAttack, &game_core::Entity, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\small_action\cast.rs:31 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | target_position | game_ai::SmallActionPlay::target_position | pub | fn(&game_ai::SmallActionPlay) -> std::option::Option<(u64, u64)> | game-ai\src\small_action.rs:229 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `or_else`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m05.ll:35320, m05.ll:35776, m11.ll:53284) · **형제 18개** (SmallActionPlay)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionPlay as std::clone::Clone>::clone | pub | game-ai\src\small_action.rs:8 | True | fn(&game_ai::SmallActionPlay) -> game_ai::SmallActionPlay |
| 1 | <game_ai::SmallActionPlay as std::fmt::Debug>::fmt | pub | game-ai\src\small_action.rs:8 | True | fn(&game_ai::SmallActionPlay, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionPlay::get_input | pub | game-ai\src\small_action.rs:157 | False | fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 3 | game_ai::SmallActionPlay::target_position | pub | game-ai\src\small_action.rs:229 | True | fn(&game_ai::SmallActionPlay) -> std::option::Option<(u64, u64)> |
| 4 | game_ai::SmallActionPlay::evaluation_position | pub | game-ai\src\small_action.rs:245 | False | fn(&game_ai::SmallActionPlay, usize, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 5 | game_ai::SmallActionPlay::position_eval_purpose | pub | game-ai\src\small_action.rs:270 | True | fn(&game_ai::SmallActionPlay) -> game_ai::PositionEvalPurpose |
| 6 | game_ai::SmallActionPlay::avoid_unnecessary_tower | pub | game-ai\src\small_action.rs:284 | True | fn(&game_ai::SmallActionPlay) -> bool |
| 7 | game_ai::SmallActionPlay::update_state | pub | game-ai\src\small_action.rs:291 | False | fn(&mut game_ai::SmallActionPlay, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 8 | game_ai::SmallActionPlay::get_action | pub | game-ai\src\small_action.rs:308 | True | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction |
| 9 | game_ai::SmallActionPlay::path_finder | pub | game-ai\src\small_action.rs:330 | False | fn(&game_ai::SmallActionPlay) -> std::option::Option<&game_ai::PathFinder> |
| 10 | game_ai::SmallActionPlay::is_end | pub | game-ai\src\small_action.rs:348 | False | fn(&game_ai::SmallActionPlay, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 11 | game_ai::SmallActionPlay::is_premise_lost | pub | game-ai\src\small_action.rs:376 | False | fn(&game_ai::SmallActionPlay, &game_core::OperationData) -> bool |
| 12 | game_ai::SmallActionPlay::is_complete | pub | game-ai\src\small_action.rs:386 | True | fn(&game_ai::SmallActionPlay) -> bool |
| 13 | game_ai::SmallActionPlay::merge | pub | game-ai\src\small_action.rs:397 | False | fn(&mut game_ai::SmallActionPlay, usize, &game_core::Entity, game_ai::SmallActionPlay) |
| 14 | game_ai::SmallActionPlay::move_near_complete | pub | game-ai\src\small_action.rs:413 | False | fn(&game_ai::SmallActionPlay, &game_core::Entity) -> bool |
| 15 | game_ai::SmallActionPlay::is_ult_escape | pub | game-ai\src\small_action.rs:437 | True | fn(&game_ai::SmallActionPlay) -> bool |
| 16 | game_ai::SmallActionPlay::extend_action | pub | game-ai\src\small_action.rs:444 | True | fn(&mut game_ai::SmallActionPlay, usize) |
| 17 | game_ai::SmallActionPlay::is_action_complete | pub | game-ai\src\small_action.rs:462 | True | fn(&game_ai::SmallActionPlay) -> bool |

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L253 Stop 경로는 target=(champ.x,champ.y) 로 move_to 를 호출한 뒤 Some 을 반환하는데, L257 의 Attack~Ult 조기반환 검사를 거치지 않고 42020 에서 직접 %86 으로 간다 — 소스가 match arm 안에서 별도 처리하는지 컴파일러 접힘인지 표기 불가(동작은 동일: 제자리 이동 = 현 좌표) | 4 |  |
| 1 | 미탐색 | Entity::move_to(g06.ll:82444 · (setting, map_setting, map, id, &mut x, &mut y, speed, tx, ty, &mut Option<…>(8B)))의 마지막 &mut 인자 타입(ptr null 초기화 · 지역 %6)은 미확정 — game_core 경계 | 4 |  |
| 2 | 미탐색 | movement_landing_position 4종(m07.ll:7051/7567/11937/12452 · (self, champ, data) → sret Option<(u64,u64)>) 내부는 r14 타 배치/미탐색 | 4 |  |
| 3 | 미탐색 | expected_goal_position(잎22 · m02.ll:8995 · (self:&SmallActionTrace(152B), player, data) → sret 24B Option<(u64,u64)>) 계약만 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | version(%2)은 dbg 이름이 없어 소스 인자명 미확정(`_version` 추정) — 본문 미사용은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

