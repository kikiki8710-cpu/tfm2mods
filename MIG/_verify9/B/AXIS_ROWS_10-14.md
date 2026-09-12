# 9차 배치 B — 축 ``sig.params[].role` — **G16 승격**` · 함수 `10`~`14` (31행)

> `mkdossier.py --axis` 생성물. **칸을 하나도 줄이지 않았다** — 7차까지 지시 오류는 전부 「줄여 적은 자리」에서 났다.

| # | 함수 | idx | i | name | ty | role | ev |
|---|---|---|---|---|---|---|---|
| `10` | should_end_object_finish_kill_priority_battle | 0 | 1 | version |  | ★AI 버전 게이트가 **아니다** — **죽은 인자**(3차 배치C, ev2). `is_enemy_well_danger` 는 `%0`(version)을 한 번도 로드하지 않는다(`_gaibc/m03.ll:144500~144535` 전문) + 오라클 ver 0~5 × team 0/1 × 17×17 격자 전수 동일. ~~「버전 분기는 전적으로 이 함수 몫」~~ 은 거짓 ⟹ **10 전체에 버전 분기가 없다.** | 2 |
| `10` | should_end_object_finish_kill_priority_battle | 1 | 2 | rnd |  | 본문에서 직접 읽지/쓰지 않음. PlayerState::strategy 에 그대로 넘기기만 한다 | 4 |
| `10` | should_end_object_finish_kill_priority_battle | 2 | 3 | player |  | info.team(0x930) 을 읽어 적 팀 인덱스(1-team) 를 만든다 | 4 |
| `10` | should_end_object_finish_kill_priority_battle | 3 | 4 | data |  | {cache:&AbstractGameWithCache, context:&GameContext, blackboard:&[Blackboard;2]}. context(+0x8)는 이 함수에서 안 쓴다 | 4 |
| `10` | should_end_object_finish_kill_priority_battle | 4 | 5 | main_objective |  | byte0=태그 / byte1=phase(ObjectPhase) / byte2=with_battle(bool) — dienum MainObjective 0 출력 기준 | 3 |
| `11` | v3_fall_back_to_passive | 0 | 1 | self |  | 플랜·서브플랜·카운터 기록 대상 | 4 |
| `11` | v3_fall_back_to_passive | 1 | 2 | version |  | AI 버전 게이트. version < 2 면 아무것도 안 하고 즉시 return (handler.rs:417) | 4 |
| `11` | v3_fall_back_to_passive | 2 | 3 | rnd |  | 본문에서 직접 안 씀 — passive_plan / BigPlan::sub_plan 으로 전달만 | 4 |
| `11` | v3_fall_back_to_passive | 3 | 4 | player |  | 본문에서 직접 안 읽음 — 하위 호출로 전달만 | 4 |
| `11` | v3_fall_back_to_passive | 4 | 5 | data |  | data.cache(=&AbstractGameWithCache) 와 data.context(=&GameContext) 를 직접 읽는다 | 4 |
| `11` | v3_fall_back_to_passive | 5 | 6 | debug |  | 본문에서 직접 안 씀 — 하위 호출로 전달만 | 4 |
| `12` | handle_chat | 0 | 1 | self |  | plan/team_plan.objective 를 읽고 pending_trace_events 에 push | 4 |
| `12` | handle_chat | 1 | 2 | version |  | 이 본문에선 분기에 쓰이지 않음 — handle_chat_inner 로 그대로 전달만 | 4 |
| `12` | handle_chat | 2 | 3 | rnd |  | 이 본문에선 안 씀 — 전달만 | 4 |
| `12` | handle_chat | 3 | 4 | player |  | 수신자(이 AI가 조종하는 선수). +0x9c0 position 만 읽음 | 4 |
| `12` | handle_chat | 4 | 5 | data |  | cache(+0x0)=&dyn AbstractGameWithCache, context(+0x8)=&GameContext | 4 |
| `12` | handle_chat | 5 | 6 | from |  | 발화자 포지션. 0=Top 1=Jungle 2=Mid 3=Bottom 4=Support | 4 |
| `12` | handle_chat | 6 | 7 | chat |  | 수신한 콜. 이 본문에선 chat_allowed 인자 + Debug 포맷에만 씀 ★근거: tcx sig `(…, game_core::Position, game_core::Chat, bool, &mut DebugFrameData)` + **rustc 실컴파일 거부**(expected `Chat`, found `&Chat`; `_verify2/C/C_o12.rs` 1차 시도). 24B 라 Win64 ABI 가 간접전달해 IR 에 포인터로 보인 것. `handle_chat_inner` 도 값 전달(2차배치C) | 3 |
| `12` | handle_chat | 7 | 8 | misunderstood |  | 오해(잘못 알아들음) 플래그. 이 본문에선 분기 안 하고 트레이스 이벤트에만 기록 후 inner 로 전달 | 4 |
| `12` | handle_chat | 8 | 9 | debug |  | 이 본문에선 안 씀 — 전달만 | 4 |
| `13` | target_bush_v30 | 0 | 1 | self |  | IR에는 self.line(LineType, 0x20) 만 %0:i8 로 인자승격돼 전달. chats/wait_limit 는 이 함수에서 안 씀 | 4 |
| `13` | target_bush_v30 | 1 | 2 | player |  | IR에서 %1:i64 = player.info.team(PlayerState+0x930), %2:i32 = player.info.position(PlayerState+0x9c0) 두 스칼라로 승격. team>=2 면 panic_bounds_check(len=2) | 4 |
| `13` | target_bush_v30 | 2 | 3 | data |  | IR에서 %3:ptr = data.cache(&AbstractGameWithCache, +0x0), %4:ptr = data.context(&GameContext, +0x8) 로 승격. data.blackboard(+0x10)은 안 씀 | 4 |
| `14` | update | 0 | 1 | self |  | chats(0x0)/line(0x28) 읽고 chats/phase(0x29) 를 쓴다 = 상태변경 | 4 |
| `14` | update | 1 | 2 | _version |  | 본문에서 전혀 안 읽힘 — AI 버전 게이트 없음(dbg_value 만 존재) | 4 |
| `14` | update | 2 | 3 | _rnd |  | readnone — 난수 안 씀 | 4 |
| `14` | update | 3 | 4 | player |  | info.team(0x930), info.position(0x9c0) 만 읽음 | 4 |
| `14` | update | 4 | 5 | data |  | cache(+0x0), context(+0x8) 를 읽음. blackboard(+0x10) 는 안 씀 | 4 |
| `14` | update | 5 | 6 | goal_data |  | has_near_line_enemy 의 수신자로만 넘어감(본문에서 필드 직접 로드 없음) | 4 |
| `14` | update | 6 | 7 | _positioning_score |  | 본문에서 안 읽힘(readonly 로 표기됐지만 사용 0회) | 4 |
| `14` | update | 7 | 8 | _debug |  | readnone — 안 씀 | 4 |
