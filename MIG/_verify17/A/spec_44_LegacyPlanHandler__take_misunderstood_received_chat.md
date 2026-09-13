---

### `44` LegacyPlanHandler::take_misunderstood_received_chat — '오해한 수신 챗' 목록에서 (tick, from, chat) 가 완전히 같은 항목을 찾아 swap_remove 하고 있었는지(bool) 돌려준다 — 판정 없음, 순수 목록 조작

| 항목 | 값 |
|---|---|
| id | `handler__take_misunderstood_received_chat` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler32take_misunderstood_received_chat` |
| 소스 | `game-ai\src\plan_legacy\handler.rs:588` |
| IR | `m13.ll` 12482~13876행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat` · **in:game_ai::plan_legacy::handler** |
| 계층 | 플랜 핸들러 |
| exe | `e4b8c0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut LegacyPlanHandler(6168B) | misunderstood_received_chats(+0x7b0 Vec<(usize, Position, Chat)>) 만 읽고 쓴다 | 4 |
| 1 | 2 | tick | usize | 항목 .0 과 == 비교 | 4 |
| 2 | 3 | from | game_core::Position(i32, range 0..5) | 항목 .1 과 == 비교 (Top0 Jungle1 Mid2 Bottom3 Support4) | 4 |
| 3 | 4 | chat | &Chat(24B) | 항목 .2 와 derive(PartialEq) 구조 비교 — 태그(+0x0) 가 같고 그 variant 의 페이로드 필드가 전부 같아야 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn take_misunderstood_received_chat(&mut self, tick, from, chat) -> bool

[L589] v = &mut self.misunderstood_received_chats(+0x7b0)      // Vec<(usize, Position, Chat)>, 원소 40B
[L590] index = v.iter().position(|(t, f, c)| *t == tick && *f == from && c == chat)
       //  IR: 인자 chat 의 바이트를 미리 로드(+0,+1,+2,+3,+4,+8,+16) 하고 원소마다
       //      ①tick(+0x0)==tick → ②from(+0x8, i32)==from → ③chat 태그(+0x10)==chat.tag
       //      → ④switch(태그 0..56) 로 그 variant 의 페이로드 필드만 비교(derive(PartialEq) 인라인, player.rs:737~779)
       //        예) 태그 0: +0x18==chat+8 / 태그 1: +0x14==chat+4 && +0x18==chat+8 / 태그 3·4: +0x18 && +0x20 / 태그 40: 페이로드 없음(즉시 일치)
       //      어느 단계든 불일치면 다음 원소(%422). 전부 일치하면 %425
[L591] if let Some(index) = index {
         v.swap_remove(index)      // memmove(v[index] ← v[len-1], 40B); len -= 1  (⚠순서 미보존)
         return true
       }
[L596] return false

※ 부작용: 일치 시 len(+0x7c0) 감소 + 버퍼 1원소 덮어쓰기. 그 외 상태 변경 없음. 호출 없음(전부 인라인).
※ 이 함수엔 AI 판정 상수가 없다 — '오해한 챗' 을 어떤 조건으로 넣는지는 push 하는 쪽(별도 함수) 소관.
```

**`mem` 메모리 접근 13건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LegacyPlanHandler | 0x7b8 | misunderstood_received_chats.buf.ptr | r | 원소 40B = { +0x0 tick usize, +0x8 from Position i32, +0x10 chat Chat 24B } | 4 | OK |  |
| 1 | LegacyPlanHandler | 0x7c0 | misunderstood_received_chats.len | r | 0 이면 즉시 false | 4 | OK |  |
| 2 | 원소(tick, from, chat) | 0x0 | tick | r | == 인자 tick | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 3 | 원소(tick, from, chat) | 0x8 | from (Position i32) | r | == 인자 from (entity.rs:563 derived PartialEq 인라인) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 4 | 원소(tick, from, chat) | 0x10 | chat@tag (i8) | r | == chat.tag. 태그 0..56 switch(57 variant), 그 밖은 unreachable | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 5 | 원소(tick, from, chat) | 0x11 | chat 페이로드 +0x1 (LineType/u8 류) | r | variant 별 비교. 인자 chat+0x1 과 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 6 | 원소(tick, from, chat) | 0x12 | chat 페이로드 +0x2 | r | variant 별 비교. 인자 chat+0x2 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 7 | 원소(tick, from, chat) | 0x13 | chat 페이로드 +0x3 | r | variant 별 비교. 인자 chat+0x3 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 8 | 원소(tick, from, chat) | 0x14 | chat 페이로드 +0x4 (i32) | r | variant 별 비교. 인자 chat+0x4 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 9 | 원소(tick, from, chat) | 0x18 | chat 페이로드 +0x8 (usize) | r | variant 별 비교. 인자 chat+0x8 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 10 | 원소(tick, from, chat) | 0x20 | chat 페이로드 +0x10 (usize) | r | variant 별 비교. 인자 chat+0x10 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 11 | LegacyPlanHandler | 0x7c0 | misunderstood_received_chats.len | w | 일치 항목이 있을 때만. Vec::swap_remove(index): 마지막 원소 40B 를 index 자리로 memmove 후 len-1 (vec.rs:2258~2269 인라인) — ★순서가 보존되지 않는다 | 4 | OK | len - 1 |
| 12 | misunderstood_received_chats 버퍼 | 0x0 | [index] (40B) | w | llvm.memmove 40B | 4 | 확인불가(tcx 사전에 타입 없음) | [len-1] 원소 복사 |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 590 | 태그 | len == 0 이면 position() 이 None → false | 4 |
| 1 | 1 | 590 | 인덱스 | position 인덱스 i += 1 | 4 |
| 2 | -1 | 591 | 미상 | new_len = len - 1 (swap_remove) | 4 |
| 3 | 40 | 591 | 태그 | swap_remove 의 memmove 크기 = 원소 (usize, Position, Chat) 40B (stride 로도 쓰임) | 4 |
| 4 | 230584300921369396 | 591 | 임계 | isize::MAX / 40 — Vec 길이 상한 assume(swap_remove 인라인 잔재). 판정 아님 | 4 |

**`knobs` 조정점 0건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|

<details><summary>`callees` 피호출자 3건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | take_misunderstood_received_chat | game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat | in:game_ai::plan_legacy::handler | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool | game-ai\src\plan_legacy\handler.rs:588 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 4개**: `derive`, `memmove`, `misunderstood_received_chats`, `swap_remove`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m13.ll:10638, m13.ll:19717) · **형제 41개** (LegacyPlanHandler)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 1 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::handler::LegacyPlanHandler::new | pub | game-ai\src\plan_legacy\handler.rs:228 | False | fn(usize, &mut rand::rngs::std::StdRng, usize, game_core::Position) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 3 | game_ai::plan_legacy::handler::LegacyPlanHandler::r2_fight_protected | in:game_ai | game-ai\src\plan_legacy\handler.rs:362 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 4 | game_ai::plan_legacy::handler::LegacyPlanHandler::ff_note_battle_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:400 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 5 | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:409 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 6 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_fall_back_to_passive | pub | game-ai\src\plan_legacy\handler.rs:416 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 7 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_cover_picks | pub | game-ai\src\plan_legacy\handler.rs:440 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 8 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_serpen_punish_issues | pub | game-ai\src\plan_legacy\handler.rs:445 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 9 | game_ai::plan_legacy::handler::LegacyPlanHandler::subplan_is_recall | pub | game-ai\src\plan_legacy\handler.rs:450 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> bool |
| 10 | game_ai::plan_legacy::handler::LegacyPlanHandler::team_objective_code | pub | game-ai\src\plan_legacy\handler.rs:456 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> u8 |
| 11 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_v2_egowave | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:473 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 12 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_obj_restore_safe | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:501 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 13 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_apply_assign_commit | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:518 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut u8, &mut game_core::DebugFrameData) |
| 14 | game_ai::plan_legacy::handler::LegacyPlanHandler::sanitize_rule_scope | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:571 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::OperationData) |
| 15 | game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:588 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool |
| 16 | game_ai::plan_legacy::handler::LegacyPlanHandler::enter_line_backfight_support | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:598 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 17 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_on_dead | pub | game-ai\src\plan_legacy\handler.rs:635 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 18 | game_ai::plan_legacy::handler::LegacyPlanHandler::update | pub | game-ai\src\plan_legacy\handler.rs:685 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 19 | game_ai::plan_legacy::handler::LegacyPlanHandler::determine_transition_reason | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1675 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &str) -> game_core::PlanTransitionReason |
| 20 | game_ai::plan_legacy::handler::LegacyPlanHandler::force_plan_update | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1709 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 21 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_assign_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1773 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 22 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_depart_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1806 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 23 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_plan_dest | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1824 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::types::BigPlan) -> std::option::Option<(u64, u64)> |
| 24 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_repair_done | in:game_ai | game-ai\src\plan_legacy\handler.rs:1847 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 25 | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1855 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) |
| 26 | game_ai::plan_legacy::handler::auction::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::get_small_action | pub | game-ai\src\plan_legacy\handler\auction.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &std::vec::Vec<(usize, game_core::SmallAction), std::alloc::Global>, &mut game_core::DebugFrameData) -> (game_ai::ScoreParameter, i64, game_ai::SmallActionPlay) |
| 27 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat | pub | game-ai\src\plan_legacy\handler\chat.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat_inner | in:game_ai | game-ai\src\plan_legacy\handler\chat.rs:41 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 29 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_track_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:35 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) |
| 30 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_fold_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:125 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8) |
| 31 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v2_response_retreat_stance | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:13 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::BattleSubPlanGoal |
| 32 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:40 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 33 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage_dive | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:109 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 34 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:136 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 35 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_interact_battle | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:233 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_single_lane | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:13 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 37 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_deathmatch | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:56 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 38 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_lane_initiate | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:196 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 39 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_try_engage | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:241 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::SinglePlanBattle> |
| 40 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:261 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | ★exe 0xe70c70 은 fnprobe 상 인자 레지스터 rcx 1개만 읽고 패닉 Location 이 handler.rs:1366:15 뿐이라 이 함수(4인자, handler.rs:588~596, 40B memmove 루프)가 아닐 가능성이 크다. 이 함수는 `internal fastcc` 라 유일 호출자에 인라인됐을 수 있다 — 미탐색 = 호출자 정체·exe 내 40B stride 루프 탐색(namebycaller) | 4 |  |
| 1 | 미탐색 | Chat 57 variant 각각의 페이로드 비교 필드는 derive(PartialEq) 의 기계적 전개라 전부 옮겨 적지 않았다(태그별 어느 오프셋을 비교하는지는 m13.ll 12570~13860 switch 케이스에 그대로 있음). 의미상 '구조적 동일' 로 요약 | 4 |  |
| 2 | 미탐색 | misunderstood_received_chats 에 항목을 push 하는 쪽과 그 의미(어떤 챗을 '오해'로 기록하나) — 이 함수 밖. _docs game_ai 에 'misunderstood' 주석 0건 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | knobs 빈 배열 = 확인된 사실(임계·계수 없음) | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

