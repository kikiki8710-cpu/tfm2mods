---

### `144` SmallActionLaneMinionPosition::has_current_explicit_minion_action — 지금 위치에서 그 라인의 적 미니언(target)에게 평타/스킬/스킬2 중 하나를 '명시적으로' 쓸 수 있는가 — 라인 근접·우물/불필요 타워 위치·스탠스 위험 게이트 후 판정

| 항목 | 값 |
|---|---|
| id | `SmallActionLaneMinionPosition__has_current_explicit_minion_action` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai12small_action11lane_minionNtB2_29SmallActionLaneMinionPosition34has_current_explicit_minion_action` |
| 소스 | `game-ai\src\small_action\lane_minion.rs:187` |
| IR | `m11.ll` 45591~45768행 |
| 경로·가시성 | `game_ai::SmallActionLaneMinionPosition::has_current_explicit_minion_action` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e266e0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 분기 없음. is_enemy_well_danger/is_unnecessary_enemy_tower_position 첫 인자로 전달. lane_stance_risk 에는 poison 전달(45721 — 콜리가 안 씀) | 4 |
| 1 | 2 | data | &OperationData(24B) | +0x0 cache(player_champion) · +0x8 context(is_near_line) | 4 |
| 2 | 3 | player | &PlayerState(2528B) | +0x930 team · +0x9c0 position 태그 · path_finder/lane_stance_risk 인자 | 4 |
| 3 | 4 | line | LineType(1B) | Top0/Mid1/Bottom2. target 미니언의 line 과 비교(45699) · is_near_line 인자 | 4 |
| 4 | 5 | target | &Entity(1728B) | +0x0/+0x8 team · +0x38 visible_state[team] · +0x68 ty · +0x11a Minion.info.line · can_use_minion_action_now 인자 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L188: champ = data.cache.player_champion[player.info.team][player.info.position]  — None → false
L191: if !( target.team != champ.team
           && target.is_visible_from(&champ.team)      [entity.rs:1482: champ.team.player_team()=Some(t) → target.visible_state[t]==Visible(0) ; Neutral → 무조건 통과]
           && lane_minion_on_line(target, line) )       [lane_minion.rs:65~66: target.ty==Minion(1) && target.ty.Minion.info.line == line]
        → false
L194: if !is_near_line(data.context, champ.x, champ.y, line) → false
L195: if is_enemy_well_danger(version, player, champ.x, champ.y) → false
L196: if is_unnecessary_enemy_tower_position(version, player, data, champ.x, champ.y) → false
L199: if lane_stance_risk(<version 미사용=poison>, player, data, champ, champ.x, champ.y).is_none() → false   (ScalarPair {tag,val}: tag==1 Some 만 통과 · 값은 안 씀)
L203: if can_use_minion_action_now(champ.can_attack(), champ.attack_effect.as_ref(), champ, target) → true
L204: if can_use_minion_action_now(champ.can_skill(),  champ.skill_effect.as_ref(),  champ, target) → true
L205: return can_use_minion_action_now(champ.can_skill2(), champ.skill2_effect().as_ref() [level>2 ? &skill2_effect : &NONE], champ, target)
(ult 은 검사하지 않음 — 45763 에서 바로 종료)
```

**`mem` 메모리 접근 18건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 45600 (gep 2352) · bounds 2 (45606) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 45611 (gep 2496) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | 45614 | 4 | OK |
| 3 | OperationData | 0x8 | context | r | 45703 → is_near_line | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | 45615~45618 (gep 480, [5 x ptr] stride) · null → false | 4 | OK |
| 5 | Entity | 0x0 | team@tag | r | 45628(target)/45631(champ) — TeamType PartialEq(entity.rs:1127) 및 player_team(1136) | 4 | OK |
| 6 | Entity | 0x8 | team@Player.0 | r | 45670/45671 팀 번호 비교 · 45647 champ 팀 번호 → visible_state 인덱스 | 4 | OK |
| 7 | Entity | 0x38 | visible_state[champ_team]@tag | r | 45677~45679 (gep 56 + {i64,[2 x i64]}*team, stride 24) == 0 Visible — is_visible_from(entity.rs:1482~1483) | 4 | OK |
| 8 | Entity | 0x68 | ty@tag | r | 45658/45682 target.ty == 1 Minion (lane_minion_on_line lane_minion.rs:65) | 4 | OK |
| 9 | Entity | 0x11a | ty@Minion.info.line@tag | r | 45695 (gep 282) == line (lane_minion.rs:66 · LineType eq player.rs:988) | 4 | OK |
| 10 | Entity | 0x660 | x | r | 45705 champ.x (gep 1632) | 4 | OK |
| 11 | Entity | 0x668 | y | r | 45707 champ.y (gep 1640) | 4 | OK |
| 12 | Entity | 0x4c0 | attack_effect@tag | r | 45731 i32 == -1 → None(null 전달) | 4 | OK |
| 13 | Entity | 0x490 | attack_effect@Some.0 | r | 45734 → can_use_minion_action_now 둘째 인자 | 4 | OK |
| 14 | Entity | 0x4f8 | skill_effect@tag | r | 45742 == -1 → null | 4 | OK |
| 15 | Entity | 0x4c8 | skill_effect@Some.0 | r | 45745 | 4 | OK |
| 16 | Entity | 0x5c8 | level | r | 45752 level>2 면 &skill2_effect(+0x500) 아니면 정적 NONE(@anon.38) (entity.rs:1693 접근자 인라인) | 4 | OK |
| 17 | Entity | 0x500 | skill2_effect | r | 45755 · +0x530 태그(gep 48, 45758) == -1 → null | 4 | OK |

**`consts` 상수 4건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 188 | 임계 | player_champion 1차원 bounds(team<2 · 45602) · visible_state bounds(45654) · level>2 skill2 가용(45754, entity.rs:1693) | 4 |
| 1 | 0 | 191 | 태그 | TeamType 태그 0 = Player(45643) · visible_state 태그 0 = Visible(45680) | 4 |
| 2 | 1 | 191 | 태그 | EntityType 태그 1 = Minion (45660/45683) · lane_stance_risk 반환 Option 태그 1 = Some (45725) | 4 |
| 3 | -1 | 203 | 센티널 | Option<Effect> 니치 None (i32 -1 · 45733/45744/45760) → as_ref() null | 4 |

**`knobs` 조정점 0건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|

<details><summary>`callees` 피호출자 14건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | can_use_minion_action_now | game_ai::SmallActionLaneMinionPosition::can_use_minion_action_now | in:game_ai | fn(bool, std::option::Option<&game_core::Effect>, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\small_action\lane_minion.rs:208 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_unnecessary_enemy_tower_position | game_ai::is_unnecessary_enemy_tower_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\path_finder.rs:1399 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 8 | lane_minion_on_line | game_ai::small_action::lane_minion::lane_minion_on_line | in:game_ai::small_action::lane_minion | fn(&game_core::Entity, game_core::LineType) -> bool | game-ai\src\small_action\lane_minion.rs:64 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 9 | lane_stance_risk | game_ai::SmallActionLaneMinionPosition::lane_stance_risk | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<i64> | game-ai\src\small_action\lane_minion.rs:380 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 12 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 13 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

**호출처 2곳** (m11.ll:42747, m11.ll:54450) · **형제 20개** (SmallActionLaneMinionPosition)

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

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L191 의 세 조건(team≠ · is_visible_from · lane_minion_on_line)의 소스 결합 순서 — column 정보 없음. IR 은 team 비교 → visible → ty/line 순으로 평가하나 이것이 소스 `&&` 순서와 같다고만 추정 | 4 |  |
| 1 | 미탐색 | can_use_minion_action_now(internal fastcc m11.ll:45096 · 인자 (bool can, Option<&Effect>, &Entity champ, &Entity target)) 내부 미탐색 | 4 |  |
| 2 | 미탐색 | is_near_line(game_core g09.ll:157890 · (ctx, x, y, line)) · is_enemy_well_danger(m03.ll:144500) · is_unnecessary_enemy_tower_position(m03.ll:146255) 내부 미탐색(경로 계층 경계) | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | lane_stance_risk 반환 { i64, i64 } 를 Option<i64>(is_none 인라인 option.rs:682/430)로 읽음 — 두 번째 i64 의 의미(위험 점수?)는 이 함수가 안 쓰므로 미확정 | 4 | 사실 서술 |
| 1 | knobs 없음 — 이 함수 안에는 튜닝 가능한 수치 상수가 없다(태그·bounds 만) | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

