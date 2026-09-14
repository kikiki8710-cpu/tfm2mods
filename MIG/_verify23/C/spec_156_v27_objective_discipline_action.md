---

### `156` v27_objective_discipline_action — 오브젝트 규율 상태(SafeWait/HardDisengage)와 근접 가시 적 유무로 대기위치 배회 / 스킬 도주 / 없음 중 하나의 소액션을 만든다

| 항목 | 값 |
|---|---|
| id | `TeamPlan__v27_objective_discipline_action` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan20objective_disciplineNtB4_8TeamPlan31v27_objective_discipline_action` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505` |
| IR | `m09.ll` 19231~19840행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action` · **pub** |
| 계층 | 기타 |
| exe | `dd5270` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<SmallActionPlay>(184B) | None = +0xb1(177) 에 i8 -1 저장(19276·19816·19824). RunAway 는 136B 페이로드를 +0 에 쓰고 +0xb1 에 태그 3(19836~19838). AroundPosition::new 는 sret 184B 를 통째로 채움(19828·19832) | 4 |
| 1 | 1 | self | &TeamPlan | 본문에서 직접 읽지 않음 — v27_active_objective_discipline 에 전달 | 4 |
| 2 | 2 | version | usize | is_ignored_well_enemy 1번째 인자로만 전달(19394 등 5회). 콜리 v27_active_objective_discipline 에는 poison | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | 본문에서 직접 읽지 않음 — SmallActionAroundPosition::new 에 &mut 전달(19828·19832). 이 함수 자체의 writes 없음(콜리 내부 rng 진행은 콜리 소관) | 4 |
| 4 | 4 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0) 읽음; is_ignored_well_enemy·is_recent_visible·new_with_skill 에 전달 | 4 |
| 5 | 5 | data | &OperationData(24B) | +0 cache(player_champion·game) / +0x10 blackboard | 4 |
| 6 | 6 | target | JungleType(i8, 0..5) | v27_active_objective_discipline 에 그대로 전달. 본문 분기 없음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v27_objective_discipline_action(&self, version, rnd, player, data, target:JungleType) -> Option<SmallActionPlay>
// L507
state = self.v27_active_objective_discipline(version /*poison*/, data, target)?   // None(kind 태그 2) → return None
// state = ObjectiveDisciplineState { wait_pos:(x,y) +0/+8, until_tick +0x10(미사용), target +0x18(미사용), kind +0x19 }
hard = (state.kind == HardDisengage /*1*/)
// L508
team = player.info.team (bounds 2); champ = cache.player_champion[team][player.info.position]?   // None → return None
// L509~512 (5칸 언롤, 첫 true 단락)
has_near_threat = cache.player_champion[1-team].iter().flatten().any(|enemy|
       enemy.is_visible_from(champ)          // champ.team Neutral → true; Player t → enemy.visible_state[t] == Visible(0)
    && !is_ignored_well_enemy(version, player, enemy)                     // L510
    && blackboard[1-team].is_recent_visible(cache.game, player, enemy)    // L511
    && (|dx|²+|dy|²) < 25600000001 )                                       // L512 dist<=160000
// L514
if hard {
    if has_near_threat { Some(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true))) }   // L516
    else               { SmallActionAroundPosition::new(rnd, data, x, y, 5) /*sret 그대로 = Some(AroundPosition)*/ }   // L519
} else /*SafeWait*/ {
    if has_near_threat { SmallActionAroundPosition::new(rnd, data, x, y, 5) }   // L522
    else               { None }                                                // L524
}
```

**`mem` 메모리 접근 16건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | ObjectiveDisciplineState(Option, 32B 스택 %8) | 0x19 | kind@tag | r | 19252~19254: ==2 → None(Option 니치) → 반환 None. 0=SafeWait / 1=HardDisengage (19263 trunc → %16) | 4 | OK |  |
| 1 | ObjectiveDisciplineState | 0x0 | wait_pos.0 | r | 19258 → AroundPosition x | 4 | OK |  |
| 2 | ObjectiveDisciplineState | 0x8 | wait_pos.1 | r | 19259~19260 → AroundPosition y | 4 | OK |  |
| 3 | PlayerState | 0x930 | info.team | r | 19268~19270 bounds<2; 적 팀 = 1-team(19301) | 4 | OK |  |
| 4 | PlayerState | 0x9c0 | info.position@tag | r | i32 → 내 챔피언 슬롯 인덱스(19285~19287) | 4 | OK |  |
| 5 | OperationData | 0x0 | cache | r | 19288 | 4 | OK |  |
| 6 | OperationData | 0x10 | blackboard | r | 19309~19310 → blackboard[1-team](19342, Blackboard 744B stride) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] / [1-team][0..5] | r | 내 챔피언(19290~19293, null → None 반환 19814~19816) · 적 5칸 언롤(19345·…·19718, null 건너뜀) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x0 | game(dyn AbstractGame 팻포인터 +0/+8) | r | 19306~19308 → is_recent_visible 인자 | 4 | OK |  |
| 9 | Entity(champ) | 0x0 | team@tag | r | Entity::is_visible_from 인라인(entity.rs:1482): 태그 1(Neutral) 이면 가시 검사 생략(19369~19371) | 4 | OK |  |
| 10 | Entity(champ) | 0x8 | team@Player.0 | r | 팀 idx(19375, bounds<2 → 19390 panic) | 4 | OK |  |
| 11 | Entity(enemy) | 0x38 | visible_state[team]@tag | r | VisibleState 24B stride(gepS 19383): 태그 0=Visible 만 통과(data.rs:122 is_visible) | 4 | OK |  |
| 12 | Entity(enemy / champ) | 0x660 | x | r | 19403~19404 · 19411 거리 | 4 | OK |  |
| 13 | Entity(enemy / champ) | 0x668 | y | r | 19407~19408 · 19414 | 4 | OK |  |
| 14 | sret Option<SmallActionPlay> | 0xb1 |  | w | None (19276 state None / 19816 champ None / 19824 SafeWait&무위협) | 4 | 확인불가(tcx 사전에 타입 없음) | -1 |
| 15 | sret Option<SmallActionPlay> | 0xb1 |  | w | RunAway 태그(SmallActionPlay 니치 idx0→태그3), 19838. 페이로드 +0..136 은 new_with_skill 이 씀 | 4 | 확인불가(tcx 사전에 타입 없음) | 3 |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 507 | 센티널 | Option<ObjectiveDisciplineState> None 니치 = kind 태그 2 (19254). 또 팀 bounds 2 (19270·19377) | 4 |
| 1 | -1 | 524 | 태그 | Option<SmallActionPlay> None 태그(+0xb1) | 4 |
| 2 | 3 | 516 | 센티널 | SmallActionPlay::RunAway 메모리 태그(니치 start 3) | 4 |
| 3 | 25600000001 | 512 | 임계 | 적 거리² < 160000²+1 ⟺ dist <= 160000 (5셀) — '근접 위협' 반경 | 4 |
| 4 | 5 | 516 | 미상 | SmallActionRunAway::new_with_skill 의 3번째 usize 인자(19836) 및 SmallActionAroundPosition::new 의 5번째 usize 인자(19828·19832) — 액션 지속/우선 파라미터(이름은 콜리 시그니처에 없음) | 4 |
| 5 | 1 | 509 | 미상 | 적 팀 = 1 - team (19301) · new_with_skill 마지막 bool true(19836) | 4 |
| 6 | 0 | 509 | 태그 | VisibleState::Visible 태그 0 (19385 · 19758) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 근접 위협 반경(제곱) | objective_discipline.rs:512 (m09.ll:19430) | 25600000001 | 올리면 더 먼 적도 위협으로 봐 HardDisengage 는 도주, SafeWait 는 배회로 더 자주 전환 | 4 | 기존 |
| 1 | 액션 파라미터 5 (RunAway/AroundPosition 공통 usize) | objective_discipline.rs:516·519·522 (m09.ll:19828·19832·19836) | 5 | 콜리 시그니처상 usize 1개 — 의미(지속틱/우선도)는 콜리 내부 소관이라 효과 미확정 | 4 | 기존 |
| 2 | RunAway 스킬 허용 플래그 | objective_discipline.rs:516 (m09.ll:19836 마지막 i1 true) | 1 | false 로 바꾸면 스킬 없는 도주(문서 주석: '도주 입력(스킬 미사용)') | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | new | game_ai::SmallActionAroundPosition::new | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:831 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | v27_active_objective_discipline | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_active_objective_discipline | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectiveDisciplineState> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:483 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | v27_objective_discipline_action | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

**호출처 4곳** (m02.ll:11405, m02.ll:40621, m14.ll:13400, m15.ll:13163) · **형제 55개** (TeamPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic | pub | game-ai\src\plan_legacy\old\epic.rs:502 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 1 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window | in:game_ai | game-ai\src\plan_legacy\old\epic.rs:634 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool |
| 2 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change | pub | game-ai\src\plan_legacy\old\epic.rs:684 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType) |
| 3 | <game_ai::plan_legacy::team_plan::TeamPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> game_ai::plan_legacy::team_plan::TeamPlan |
| 4 | <game_ai::plan_legacy::team_plan::TeamPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 5 | <game_ai::plan_legacy::team_plan::TeamPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn() -> game_ai::plan_legacy::team_plan::TeamPlan |
| 6 | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | game-ai\src\plan_legacy\team_plan.rs:196 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) |
| 7 | game_ai::plan_legacy::team_plan::TeamPlan::sanitize_rule_scope | pub | game-ai\src\plan_legacy\team_plan.rs:200 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::GameContext) |
| 8 | game_ai::plan_legacy::team_plan::TeamPlan::objective_target | pub | game-ai\src\plan_legacy\team_plan.rs:230 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> std::option::Option<game_core::JungleType> |
| 9 | game_ai::plan_legacy::team_plan::TeamPlan::take_active | pub | game-ai\src\plan_legacy\team_plan.rs:243 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 10 | game_ai::plan_legacy::team_plan::TeamPlan::take_hunt_commit | pub | game-ai\src\plan_legacy\team_plan.rs:248 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 11 | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | game-ai\src\plan_legacy\team_plan.rs:257 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 12 | game_ai::plan_legacy::team_plan::TeamPlan::mark_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:265 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType, usize) |
| 13 | game_ai::plan_legacy::team_plan::TeamPlan::clear_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:277 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan) |
| 14 | game_ai::plan_legacy::team_plan::TeamPlan::init | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:281 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 15 | game_ai::plan_legacy::team_plan::TeamPlan::update | pub | game-ai\src\plan_legacy\team_plan.rs:294 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies | pub | game-ai\src\plan_legacy\team_plan.rs:444 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 17 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | game-ai\src\plan_legacy\team_plan.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 18 | game_ai::plan_legacy::team_plan::TeamPlan::update_wave_priority_clear_line | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:540 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 19 | game_ai::plan_legacy::team_plan::TeamPlan::should_player_clear_wave_priority_line | pub | game-ai\src\plan_legacy\team_plan.rs:561 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> |
| 20 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_morgard_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:570 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 21 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_serpen_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:574 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 22 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_object_setup_for_wave_priority | pub | game-ai\src\plan_legacy\team_plan.rs:578 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 23 | game_ai::plan_legacy::team_plan::TeamPlan::should_keep_object_for_contested_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:586 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool |
| 24 | game_ai::plan_legacy::team_plan::TeamPlan::repair_misunderstood_objective | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:603 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 25 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective | pub | game-ai\src\plan_legacy\team_plan.rs:722 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 26 | game_ai::plan_legacy::team_plan::TeamPlan::update_steal | pub | game-ai\src\plan_legacy\team_plan.rs:732 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) |
| 27 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective_after_steal | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:910 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool |
| 29 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool |
| 30 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_release_to_passive | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:177 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 31 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_relevant_lanes_ready | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:199 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 32 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> |
| 33 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_wait_pos | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:316 | False | fn(game_core::JungleType, usize, &game_core::MapDef) -> (u64, u64) |
| 34 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_entity | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:324 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData, game_core::JungleType) -> std::option::Option<&game_core::Entity> |
| 35 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::update_v27_objective_discipline | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_active_objective_discipline | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectiveDisciplineState> |
| 37 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_blocks_battle | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:497 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData) -> bool |
| 38 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> |
| 39 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_repair_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:7 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) |
| 40 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:16 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 41 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:29 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 42 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_line_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:46 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 43 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_dive_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:88 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 44 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_tower_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:141 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 45 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:253 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 46 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_split_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 47 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_comeback_pick_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:393 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType, bool) -> bool |
| 48 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_serpen_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:558 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 49 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_morgard_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:685 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 50 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_none_or_gank_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 51 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_gank_preprocess | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1133 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, game_core::LineType) -> bool |
| 52 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_defense | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1227 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 53 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_attack | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1237 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> |
| 54 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_sub_objective | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1258 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | AroundPosition 경로의 sret: SmallActionAroundPosition::new 의 tcx 반환형은 SmallActionAroundPosition(184B) 인데 IR 은 Option<SmallActionPlay>(184B) sret 을 그대로 넘긴다 — SmallActionPlay::AroundPosition 이 untagged(암묵) variant 라 레이아웃 동일해 접힌 것으로 읽음(tcxdict --enum SmallActionPlay: untagged=7 AroundPosition). +0xb1 이 니치 밖 값이라는 전제 | 3 |  |
| 1 | 미탐색 | state.until_tick(+0x10)·target(+0x18) 은 이 함수에서 안 읽음 — 콜리 v27_active_objective_discipline 소관(같은 r14 중간 · 내부 미독) | 4 |  |
| 2 | 미탐색 | is_ignored_well_enemy(usize,&PlayerState,&Entity)->bool (fight_model.rs:754) · Blackboard::is_recent_visible · new_with_skill(&OperationData,&PlayerState,usize,bool)->SmallActionRunAway · SmallActionAroundPosition::new(&mut StdRng,&OperationData,u64,u64,usize) — 시그니처만 | 4 |  |
| 3 | 미탐색 | Entity::is_visible_from(&Entity,&Entity)->bool (entity.rs:1481, mir=1) 인라인 — Neutral 관측자는 항상 true 로 읽힘(19369~19371 분기 방향) | 4 |  |
| 4 | 미탐색 | rnd(&mut StdRng) 는 본문에 직접 write 없음 — AroundPosition::new 내부 rng 소비 여부는 콜리 소관 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

