---

### `12` handle_chat — 아군 채팅 수신 게이트 — 자기발화/튜토리얼 미존재 포지션/규칙범위 밖이면 무시하고, 통과하면 handle_chat_inner 호출(트레이스 켜져 있으면 전후 플랜 스냅샷을 CallHandled 이벤트로 적재)

| 항목 | 값 |
|---|---|
| id | `handler_chat__handle_chat` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler4chatNtB4_17LegacyPlanHandler11handle_chat` |
| 소스 | `game-ai\src\plan_legacy\handler\chat.rs:8` |
| IR | `m13.ll` 29383~29689행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat` · **pub** |
| 계층 | 플랜 핸들러 |
| exe | `e70330` (chat) · 379바이트 · 100명령 |
| 라운드 | 기준 `r4` · 통과 4회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData)
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 1 | self |  | plan/team_plan.objective 를 읽고 pending_trace_events 에 push |
| 1 | 2 | version |  | 이 본문에선 분기에 쓰이지 않음 — handle_chat_inner 로 그대로 전달만 |
| 2 | 3 | rnd |  | 이 본문에선 안 씀 — 전달만 |
| 3 | 4 | player |  | 수신자(이 AI가 조종하는 선수). +0x9c0 position 만 읽음 |
| 4 | 5 | data |  | cache(+0x0)=&dyn AbstractGameWithCache, context(+0x8)=&GameContext |
| 5 | 6 | from |  | 발화자 포지션. 0=Top 1=Jungle 2=Mid 3=Bottom 4=Support |
| 6 | 7 | chat |  | 수신한 콜. 이 본문에선 chat_allowed 인자 + Debug 포맷에만 씀 ★근거: tcx sig `(…, game_core::Position, game_core::Chat, bool, &mut DebugFrameData)` + **rustc 실컴파일 거부**(expected `Chat`, found `&Chat`; `_verify2/C/C_o12.rs` 1차 시도). 24B 라 Win64 ABI 가 간접전달해 IR 에 포인터로 보인 것. `handle_chat_inner` 도 값 전달(2차배치C) |
| 7 | 8 | misunderstood |  | 오해(잘못 알아들음) 플래그. 이 본문에선 분기 안 하고 트레이스 이벤트에만 기록 후 inner 로 전달 |
| 8 | 9 | debug |  | 이 본문에선 안 씀 — 전달만 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn handle_chat(self, version, rnd, player, data, from: Position, chat: Chat, misunderstood: bool, debug)

// ---- 게이트 1 : 자기 자신의 발화는 무시 (chat.rs:9)
if from == player.info.position { // PlayerState +0x9c0, Position::eq 인라인(entity.rs:563)
 return;
}

// ---- 게이트 2 : 튜토리얼에서 그 포지션이 존재하는가 (chat.rs:12)
// rule_scope::position_exists(&GameContext, Position) // ★이 호출이 통째로 인라인돼
// from 별 switch + tutorial 태그 switch 로 접혔다. tut = data.context.tutorial (GameContext +0x38)
let ok = match from {
 Top => tut in {None(0), TopSolo(2), Line(7), Total(8)}, // rule_scope.rs:38 ← TutorialType::spawn_top_minion
 Jungle => tut in {None(0), JungleOnly(6), Total(8)}, // rule_scope.rs:39 ← TutorialType::player_count
 Mid => tut in {None(0), MidSolo(4), MidBottom(5), Line(7), Total(8)}, // rule_scope.rs:40 ← TutorialType::spawn_mid_minion
 Bottom | Support => tut in {None(0), First(1), Bottom(3), MidBottom(5), Line(7), Total(8)}, // rule_scope.rs:41 ← TutorialType::spawn_bottom_minion
};
if !ok { return; }

// ---- 게이트 3 : 규칙 범위(같은 chat.rs:12 줄)
if !rule_scope::chat_allowed(data.context, &chat) { return; } // 내부 미조사

// ---- 트레이스 분기 (chat.rs:18)
if data.context.trace_level == TraceLevel::Off { // GameContext +0x39, TraceLevel::is_enabled 인라인
 self.handle_chat_inner(version, rnd, player, data, from, chat, misunderstood, debug); // chat.rs:19
 return;
}

// ---- 트레이스 ON 경로 : inner 를 전후로 감싸 스냅샷 (chat.rs:22~37)
let plan_before = self.plan.get_name(); // +0x5e8, chat.rs:22
let objective_before = format!("{:?}", self.team_plan.objective); // +0x517, chat.rs:23
self.handle_chat_inner(version, rnd, player, data, from, chat, misunderstood, debug); // chat.rs:24
let plan_after = self.plan.get_name(); // chat.rs:25
let objective_after = format!("{:?}", self.team_plan.objective); // chat.rs:26
self.pending_trace_events.push(PendingTraceEvent { // +0x858 Vec, chat.rs:27
 event: TraceEventType::CallHandled {
 chat: format!("{:?}", chat), // chat.rs:31 → 필드 +0x8
 plan_before, // +0x20
 plan_after, // +0x38
 objective_before, // +0x50
 objective_after, // +0x68
 from, // +0x80
 misunderstood, // +0x84
 },
 tick: data.cache.game.tick(), // 간접호출: vtable +0x28 → +0xb0
});
// push 는 인라인: cap(+0x858)==len(+0x868) 이면 RawVec::grow_one, 그 뒤 ptr(+0x860)+len*184 에 184B memcpy, len += 1
return;

// 언와인딩 경로(50/54/60/72/100~107)는 만들어둔 String 들을 drop_glue 로 해제하는 정리 코드뿐 — 판정 로직 없음.
// 주의: 판정에 실제로 쓰이는 인자는 player.position / from / data.context 뿐이고,
// version·rnd·debug 는 이 함수에서 한 번도 읽히지 않고 그대로 inner 로 넘어간다.
```

**`mem` 메모리 접근 25건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x9c0 | info.position | r | 수신자 자신의 포지션(Position i32). from 과 같으면 즉시 반환 = 자기 발화 무시 (chat.rs:9, PartialEq 인라인) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=0)) | 3 |
| 1 | OperationData | 0x0 | cache | r | &dyn AbstractGameWithCache 팻포인터의 데이터 절반(%65 에서 다시 로드) — tick 조회용, 트레이스 경로에서만 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=1)) | 3 |
| 2 | OperationData | 0x8 | context | r | &GameContext(64B) — 튜토리얼/트레이스레벨 게이트의 원천이자 chat_allowed 의 1번 인자 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=2)) | 3 |
| 3 | GameContext | 0x38 | tutorial | r | TutorialType(i8). rule_scope::position_exists 가 인라인돼 from 별 switch 로 접혔다 (chat.rs:12) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=3)) | 3 |
| 4 | GameContext | 0x39 | trace_level | r | TraceLevel(i8). TraceLevel::is_enabled 인라인 = (trace_level != Off) (trace.rs:14~15 → chat.rs:18) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=4)) | 3 |
| 5 | AbstractGameWithCache | 0x8 | game(vtable 절반) | r | cache.game 은 &dyn AbstractGame 팻포인터 — +0x0=데이터, +0x8=vtable · tcx 정본 대조( tcx: `AbstractGameWithCache.game` 은 +0x0 의 16B `&dyn` 팻포인터라 **+0x8 = vtable 절반**(Rust 팻포인터 ABI). `tcxdict` 는 팻포인터 뒤 절반을 필드로 세지 않는다(METHOD_MAP ⑦ 한계 명시)) | 3 |
| 6 | AbstractGame vtable | 0x28 | tick | r | divtable.py 확인: 슬롯 0x28 = ExpectedGame::AbstractGame::tick. 간접 호출로 i64 tick 을 얻어 PendingTraceEvent.tick 에 넣는다 | 3 |
| 7 | LegacyPlanHandler | 0x5e8 | plan | r | BigPlan(384B). BigPlan::get_name 을 inner 호출 전(plan_before)·후(plan_after) 두 번 부른다 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=7)) | 3 |
| 8 | LegacyPlanHandler | 0x517 | team_plan.objective | r | Option<MainObjective>(3B). Debug 포맷을 inner 호출 전·후 두 번(objective_before/after) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=8)) | 3 |
| 9 | LegacyPlanHandler | 0x858 | pending_trace_events.cap | r | Vec 의 RawVec.cap. len 과 같으면 grow_one · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=9)) | 3 |
| 10 | LegacyPlanHandler | 0x860 | pending_trace_events.ptr | r | Vec 버퍼 시작 주소. 원소 stride 184B · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=10)) | 3 |
| 11 | LegacyPlanHandler | 0x868 | pending_trace_events.len | r | push 위치이자 push 후 +1 되는 대상 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=11)) | 3 |
| 12 | LegacyPlanHandler | 0x868 | pending_trace_events.len | w | 트레이스 경로에서만. 게임 판정에는 영향 없는 관측용 버퍼 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=12)) | 3 |
| 13 | LegacyPlanHandler | 0x860[len] | pending_trace_events[len] | w | 스택에 조립한 184B 를 memcpy. 원소 = { event: TraceEventType(176B), tick: usize(+0xb0) } · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=13)) | 3 |
| 14 | TraceEventType | 0x0 | discriminant | w | dienum 확인: TraceEventType::CallHandled | 3 |
| 15 | TraceEventType::CallHandled | 0x8 | chat | w | chat.rs:29 대입 / 포맷 생성은 :31 · tcx 정본 대조( `tcxdict --enum game_core::TraceEventType`: 페이로드 `CallHandled.chat: String` = **enum+0x8** ) | 3 |
| 16 | TraceEventType::CallHandled | 0x20 | plan_before | w | chat.rs:22 생성 → :33 대입 · tcx 정본 대조( `tcxdict --enum game_core::TraceEventType`: `CallHandled.plan_before` = **enum+0x20** ) | 3 |
| 17 | TraceEventType::CallHandled | 0x38 | plan_after | w | chat.rs:25 생성 → :34 대입 · tcx 정본 대조( `tcxdict --enum game_core::TraceEventType`: `CallHandled.plan_after` = **enum+0x38** ) | 3 |
| 18 | TraceEventType::CallHandled | 0x50 | objective_before | w | chat.rs:23 생성 → :35 대입 · tcx 정본 대조( `tcxdict --enum game_core::TraceEventType`: `CallHandled.objective_before` = **enum+0x50** ) | 3 |
| 19 | TraceEventType::CallHandled | 0x68 | objective_after | w | chat.rs:26 생성 → :36 대입 · tcx 정본 대조( `tcxdict --enum game_core::TraceEventType`: `CallHandled.objective_after` = **enum+0x68** ) | 3 |
| 20 | TraceEventType::CallHandled | 0x80 | from | w | · tcx 정본 대조( `tcxdict --enum game_core::TraceEventType`: `CallHandled.from: Position` = **enum+0x80** ) | 3 |
| 21 | TraceEventType::CallHandled | 0x84 | misunderstood | w | · tcx 정본 대조( `tcxdict --enum game_core::TraceEventType`: `CallHandled.misunderstood: bool` = **enum+0x84** ) | 3 |
| 22 | PendingTraceEvent | 0xb0 | tick | w | · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=22)) | 3 |
| 23 | LegacyPlanHandler | 0x858 | pending_trace_events.cap | w | 트레이스 ON 첫 push 에서 재할당. logic 에는 적혀 있었으나 writes 표에 빠져 있었다. 실측 diff 0x858..0x859 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=23)) | 3 |
| 24 | LegacyPlanHandler | 0x860 | pending_trace_events.ptr | w | 동상. 실측 diff 0x860..0x866 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=12 idx=24)) | 3 |

**`consts` 상수 12건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 12 | 태그 | TutorialType::None 태그 — 4개 포지션 arm 전부에서 통과값(튜토리얼이 아니면 항상 채팅 수신). 같은 값 0 은 chat.rs:18 의 TraceLevel::Off 비교(icmp eq i8 %42, 0)와 Position::Top 케이스에도 쓰인다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 1 | 1 | 12 | 태그 | TutorialType::First 태그 — Bottom/Support arm 에서만 통과. (같은 값이 switch 의 Position::Jungle 케이스로도 등장) · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 2 | 2 | 12 | 태그 | TutorialType::TopSolo 태그 — Top arm 에서만 통과. (Position::Mid 케이스 값이기도 함) · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 3 | 3 | 12 | 태그 | TutorialType::Bottom 태그 — Bottom/Support arm 통과. (Position::Bottom 케이스 값이기도 함) · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 4 | 4 | 12 | 태그 | TutorialType::MidSolo 태그 — Mid arm 통과. (Position::Support 케이스 값이기도 함) · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 5 | 5 | 12 | 태그 | TutorialType::MidBottom 태그 — Mid arm 과 Bottom/Support arm 둘 다 통과 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 6 | 6 | 12 | 태그 | TutorialType::JungleOnly 태그 — Jungle arm 에서 icmp eq i8 %28, 6 으로 단독 비교 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 7 | 7 | 12 | 태그 | TutorialType::Line 태그 — Top/Mid/Bottom·Support arm 통과(정글만 제외) · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 8 | 8 | 12 | 태그 | TutorialType::Total 태그 — 4개 arm 전부 통과(풀 5:5 튜토리얼) · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 9 | -8 | 12 | 임계 | Jungle arm 의 접힌 범위검사 add i8 %28, -8 — TutorialType::Total(8) 을 0 으로 옮기는 오프셋 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270** ⟹ 집합은 실행 확정(리터럴 인코딩은 외연 동일 표기라 실행으로 안 갈린다 — 범위 명시)) | 2 |
| 10 | -7 | 12 | 임계 | 같은 검사의 상한 icmp ult i8 %34, -7 (=u8 249). (%28-8) <u 249 ⟺ %28 ∈ {0, 8} = {None, Total}. 여기에 ==6(JungleOnly) 을 or 한 것이 Jungle 통과집합 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270** ⟹ 집합은 실행 확정(리터럴 인코딩은 외연 동일 표기라 실행으로 안 갈린다 — 범위 명시)) | 2 |
| 11 | -9223372036854775793 | 27 | 태그 | TraceEventType 판별자 = CallHandled (기준 -9223372036854775808 + 15). 니치 최적화라 variant 인덱스와 다르며 dienum.py DISCR_EXACT 로 확인 · 오라클 실행 확증( 오라클 `_verify5/C/o11b.out §S12d` — `pending_trace_events[0]` 선두 i64 날바이트가 정확히 -9223372036854775793 · 6차 `tcxdict --enum TraceEventType` 로 niche_start(-2^63)+15 재확인) | 2 |

**`knobs` 조정점 21건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | 자기 발화 무시 게이트 | game-ai\src\plan_legacy\handler\chat.rs:9 | from == player.info.position → return | 이 비교를 없애면 자기가 낸 콜에도 자기 AI가 반응하게 된다(자기 콜에 대한 자문자답). 유지가 정상 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 1 | 튜토리얼 포지션 존재 게이트(position_exists) | game-ai\src\plan_legacy\rule_scope.rs:36~42 (chat.rs:12 에 인라인) | from 별 허용 TutorialType 태그 집합 {Top:0,2,7,8 / Jungle:0,6,8 / Mid:0,4,5,7,8 / Bottom·Support:0,1,3,5,7,8} | 특정 튜토리얼 태그를 arm 에 추가하면 그 튜토리얼에서 해당 포지션이 콜을 받아 반응하기 시작하고, 빼면 그 포지션이 콜을 전부 무시한다. 일반 경기(tutorial=None(0))는 모든 arm 에 0 이 있어 항상 통과하므로 이 노브는 튜토리얼 전용 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 2 | 트레이스 레벨 게이트 | GameContext +0x39 trace_level (chat.rs:18) | Off(0) / Summary(1) / Detailed(2) | Off 가 아니면 콜 처리 전후 플랜·오브젝티브 문자열을 만들어 pending_trace_events 에 CallHandled 로 쌓는다. 판정 결과는 바뀌지 않고 관측/디버그용 비용(포맷 + Vec 증가)만 는다. 모드에서 콜 처리 로그를 뽑고 싶으면 여기를 켜는 게 최소 개입 지점 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 3 | 정글 콜 수신 튜토리얼 게이트 | rule_scope.rs:39 / m13.ll:29471~29474, 53283~53286 | tut==JungleOnly(6) \|\| player_count()==5 → {0,6,8} | `==5` 를 `>=4` 로 낮추면 **Line(7) 튜토리얼에서도 정글러가 콜을 받기 시작**한다. 일반 경기(0)는 영향 없음 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 4 | 에픽(모르가드) 콜 허용 집합 | TutorialType::spawn_epic (runner.rs:262 / m13.ll:53356) | {0,7,8} | 태그를 더하면 해당 튜토리얼에서 Morgard 계열 콜 33~41 과 Split/Press/PressChange 가 전부 열린다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 5 | 세르펜 콜 허용 집합 | TutorialType::spawn_serpen (runner.rs:266) | {0,5,7,8} | Serpen 계열 콜 24~32 개폐 | 4 |
| 6 | chat_allowed default=true | rule_scope.rs:129 switch default / m13.ll:53207 | 미열거 15개 태그는 무조건 허용 | 태그를 추가/제거하면 그 콜 종류가 튜토리얼에서 통째로 막히거나 열린다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1`(포지션 5열 × tutorial 9행 45/45) + `o11b.out §S12b` 게이트 실동작 **270/270**) | 2 |
| 7 | PlayCall/PlayPropose 라인코드 우회 임계 | rule_scope.rs:113 push_line_code_allowed | code>=3 → 무조건 허용 | 3 을 올리면 더 많은 코드가 라인 존재검사를 우회 | 4 |
| 8 | ★v3_epicops_armed 로 Press 콜 봉인 | handler +0x180a / m13.ll:29790 부근 블록 %781 | true 면 Press/PressChange 콜을 통째로 무시 | 이 플래그를 강제로 끄면 v3 에서도 레거시 Press 처리가 되살아난다 | 4 |
| 9 | 오브젝티브 중복 가드 | 각 arm 진입 icmp | objective == 목표면 skip | 제거하면 같은 콜이 반복 도착할 때마다 plan 이 매번 passive_plan() 으로 리셋된다 | 4 |
| 10 | ComebackPick 수용 조건 | m13.ll:29981 switch %43 [-1, 8] | objective ∈ {None, Gank} | 태그를 추가하면 다른 오브젝티브 중에도 컴백픽 콜을 받는다 | 4 |
| 11 | 오해(misunderstanding) 기록 | chat.rs:598 / m13.ll:33329 | misunderstood bool | 항상 false 로 두면 objective_misunderstanding 이 절대 세워지지 않는다(= 그 기록을 읽는 상위 판정이 죽는다) | 4 |
| 12 | ★HideLine 콜 무시 확률 | chat.rs:83 / m13.ll:31003~31006 | gen_range(0..1000) < ego_ratio*700/1000 | `700` 을 0 으로 하면 **이기심 무시가 사라져 콜 반응률이 급증**한다(간크 스코어가 음수여도 합류) | 4 |
| 13 | HideLine 수락 HP 게이트 | chat.rs:75 / m13.ll:30869~30871 | 70 - roaming*20/1000 (70~50) | 낮추면 저체력에도 간크 콜에 붙는다 | 4 |
| 14 | Battle 합류 HP 게이트 | chat.rs:163·199 | 60 - aggressive*20/1000 (60~40) | 교전 콜 수용 하한 | 4 |
| 15 | Battle 합류 이동거리 한도 | chat.rs:165·202 | roaming*2/1000 + 4 초 × tps | 4 를 올리면 원거리 콜도 받는다(ff_call_too_far 감소) | 4 |
| 16 | 다이브 재시도 쿨 | chat.rs:174 / m13.ll:30198~30201 | last_dive_abandon_tick + tps*4 + 1 | 줄이면 포기 직후 다이브 재합류 허용 | 4 |
| 17 | ★저울 합류식 | chat.rs:218 / m13.ll:30389~30419 | can_help && ((old_pass && !stake_veto) \|\| stake_commit) | `stake_commit` 항을 빼면 저울이 죽고 v1 동작으로 회귀한다 | 4 |
| 18 | 즉발불발 폐기 조건 | chat.rs:235 / m13.ll:30737 | 새 플랜 sub_goal == RunAway → 폐기 | 제거하면 "합류하자마자 도주" 플랜도 채택된다 | 4 |
| 19 | ★ff_call_* 8칸 계측표 — 「콜을 받고도 왜 안 갔는가」 분해기 | LegacyPlanHandler +0x15b8 recv / +0x15c0 ignored / +0x15c8 in_battle / +0x15d0 no_help / +0x15d8 too_far / +0x15e0 low_hp / +0x15e8 bail / +0x15f0 join (chat.rs:141/144/151/204/205/206/239/236) | usize × 8 | `handle_chat` 이 게이트를 통과시킨 뒤 inner 가 콜을 어떻게 처리했는지가 이 8칸에 전부 남는다 — **인게임 검증 지표로 즉시 쓸 수 있다.** 실행 확인 : Battle 계열 콜이 inner 에 닿을 때마다 `recv` +1, 동반해 `too_far` 또는 `bail` 이 +1 로 실제 증가 | 2 |
| 20 | ★합류 거절 후 「글로벌 궁 예약」 (명세 본문에 통째로 없던 구간) | chat.rs:245~290 (`_gaibc/m13.ll:30495~30620`) → LegacyPlanHandler+0x530 `pending_global_ult_target: Option<(usize,usize)>`(24B, tcx 확정) | 대상 `Entity.id`(+0x538) + 만료틱 `tick + tps*3`(+0x540) | 콜을 거절한다고 아무것도 안 하는 게 아니라 **거절 대신 글로벌 궁을 3초짜리로 예약한다** (조건: chats 포화 ∧ `Entity::can_ult` ∧ `+0x530==None` ∧ 거리>199999 ∧ `iter_champions(..).count()==0`). 소비처 = `auction::get_small_action` 1곳. 재구현에서 빠지면 궁 사용 타이밍이 통째로 달라진다. ⚠오라클 미검증 — 기본 챔피언(이펙트 공백)은 `can_ult` 가 안 서서 14,950행 전수에서 한 번도 안 바뀌었다(입력 판별력 부재) | 2 |

<details><summary>`callees` 피호출자 14건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | chat_allowed | game_ai::plan_legacy::rule_scope::chat_allowed | pub | fn(&game_core::GameContext, &game_core::Chat) -> bool | game-ai\src\plan_legacy\rule_scope.rs:128 |
| 1 | get_name | game_ai::plan_legacy::sub_plan::SubPlan::get_name | pub | fn(&game_ai::plan_legacy::sub_plan::SubPlan) -> &str | game-ai\src\plan_legacy\sub_plan\mod.rs:74 |
| 2 | get_name | game_ai::plan_legacy::types::BigPlan::get_name | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> std::string::String | game-ai\src\plan_legacy\types.rs:44 |
| 3 | get_name | game_core::JungleType::get_name | pub | fn(&game_core::JungleType) -> std::string::String | game-core\src\simulation\entity\jungle.rs:416 |
| 4 | handle_chat | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat | pub | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler\chat.rs:8 |
| 5 | handle_chat_inner | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat_inner | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler\chat.rs:41 |
| 6 | position_exists | game_ai::plan_legacy::rule_scope::position_exists | pub | fn(&game_core::GameContext, game_core::Position) -> bool | game-ai\src\plan_legacy\rule_scope.rs:36 |
| 7 | push | <game_core::AthleteStat as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::setting::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\setting\athlete.rs:167 |
| 8 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 |
| 9 | push | <game_core::TrainingExp as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:1137 |
| 10 | tick | <game_core::Game as game_core::AbstractGame>::tick | pub | fn(&game_core::Game) -> usize | game-core\src\simulation\game.rs:1796 |
| 11 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 |
| 12 | tick | <game_core::ExpectedGame<'a> as game_core::AbstractGame>::tick | pub | fn(&game_core::ExpectedGame<'a/#0>) -> usize | game-core\src\simulation\expected_game.rs:53 |
| 13 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 |
</details>

⚠**미매칭 2개**: `format_inner`, `grow_one`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m13.ll:10643, m13.ll:19723) · **형제 41개** (LegacyPlanHandler)

| # | 이름 | 심볼 | 비고 |
|---|---|---|---|
| 0 |  |  |  |
| 1 |  |  |  |
| 2 |  |  |  |
| 3 |  |  |  |
| 4 |  |  |  |
| 5 |  |  |  |
| 6 |  |  |  |
| 7 |  |  |  |
| 8 |  |  |  |
| 9 |  |  |  |
| 10 |  |  |  |
| 11 |  |  |  |
| 12 |  |  |  |
| 13 |  |  |  |
| 14 |  |  |  |
| 15 |  |  |  |
| 16 |  |  |  |
| 17 |  |  |  |
| 18 |  |  |  |
| 19 |  |  |  |
| 20 |  |  |  |
| 21 |  |  |  |
| 22 |  |  |  |
| 23 |  |  |  |
| 24 |  |  |  |
| 25 |  |  |  |
| 26 |  |  |  |
| 27 |  |  |  |
| 28 |  |  |  |
| 29 |  |  |  |
| 30 |  |  |  |
| 31 |  |  |  |
| 32 |  |  |  |
| 33 |  |  |  |
| 34 |  |  |  |
| 35 |  |  |  |
| 36 |  |  |  |
| 37 |  |  |  |
| 38 |  |  |  |
| 39 |  |  |  |
| 40 |  |  |  |

**`open` 0건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**

(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)

<details><summary>`closed` 8건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | Jungle arm(rule_scope.rs:39)의 원래 비교식 — TutorialType::player_count(runner.rs:294)가 인라인 전용이라 _gaibc 24개 .ll 어디에도 define 이 없다(grep '^define.*TutorialType12player_count' 무결과). 결과 집합 {None(0), JungleOnly(6), Total(8)} 은 add/icmp 로 확정했지만, 원래 소스가 player_count() 를 어떤 상수와 비교했는지는 상수접힘으로 소실 — 2회 시도 후 포기 |  |  |
| 1 | rule_scope::chat_allowed(GameContext, &Chat) 내부는 안 봄 — 어떤 Chat 종류를 어떤 조건으로 막는지는 별도 명세 필요. 담당 줄범위 밖 |  |  |
| 2 | handle_chat_inner(m13.ll 29692~33371) 내부는 안 봄 — 실제 콜 반응(플랜 전환) 로직은 전부 거기 있다. 이 함수는 게이트 + 트레이스 래퍼일 뿐 |  |  |
| 3 | format! 템플릿 @anon.282069a2ed2ad3a275929b639963fb55.62 는 2바이트 압축 constant(c"\C0\00")라 문자열 조각이 없다 — Debug 포맷 인자 1개짜리("{:?}")로 추정. 리터럴 조각을 직접 확인하지는 못함 |  |  |
| 4 | chat.rs:18 의 소스 형태가 `if !is_enabled { inner(); return; }` 인지 `if is_enabled { ... } else { inner(); }` 인지 IR(br + 두 진입 블록)만으로는 확정 불가. 동작은 동일 |  |  |
| 5 | misunderstood(p8) 가 이 함수에서 분기에 안 쓰이는 것은 확정이지만, inner 안에서 어떻게 쓰이는지는 미조사 |  |  |
| 6 | PendingTraceEvent 원소 크기 184(= gep 타입 {{i64,[21 x i64]},i64})와 dbg_value i64 184 는 stride 라서 SPEC_GUIDE §3 표에 따라 constants 에 넣지 않았다 |  |  |
| 7 | `rule_scope.rs:39` 의 `== 5` vs `>= 5` — ★**표기 불가로 종결 · 재조사 금지(2026-09-11)**. `\|\|` 좌우 순서는 확정(좌 `player_count()==5` / 우 `tut == TutorialType::JungleOnly`) — MIR 단락 구조로 직접 증명됐고 오라클 실행으로도 재확인. `> 4` 는 컬럼 산술(56~64열이 정확히 9자)로 배제. 남은 두 표현은 `player_count()` 최댓값이 5라 **진리값 집합이 완전히 동일** ⟹ MIR·LLVM IR·기계어 어디에도 차이가 안 남는다. 오라클로도 못 갈린다(**외연이 같은 표현은 실행으로 절대 안 갈린다**). 전문 = RE6-09-11_rustc-dev-커스텀드라이버-tcx덤프-MIR가용성판정-0.5.8.md |  |  |
</details>

<details><summary>`history` 정정 이력 15건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | TutorialType::player_count 의 원래 비교식 — 인라인 전용이라 상수접힘으로 소실, 2회 시도 후 포기 | ★확정. `_gcbc` 에도 define 은 없지만(전 호출부 인라인) **DWARF DISubprogram(_gcbc/g09.ll:491581, runner.rs:294)** 이 있어 인라인 잔해를 역추적할 수 있었다. 반환형 usize. 값은 **접힘 안 된 호출부 2곳**에서 리터럴로 확정됐다: `_gaibc/m04.ll:16516~16545`(match player_count(){2=>tick_2v2, 3=>tick_3v3, _=>tick}) 와 `_gaibc/m09.ll:53135~53162`(`.min(4)` phi). |  |
| 1 | rule_scope::chat_allowed(GameContext, &Chat) 내부 — 어떤 Chat 을 어떤 조건으로 막는지 | ★전체 표 확정. `_gaibc/m13.ll:53199~53946`(rule_scope.rs:128, match 는 :129). 인자 = (&GameContext(64B), &Chat(24B)), 튜토리얼은 GameContext+0x38. |  |
| 2 | handle_chat_inner 내부는 안 봄 — 실제 콜 반응(플랜 전환) 로직이 전부 거기 있다 | ★Chat 태그별 플랜 전환표 확정. m13.ll:29692~33371, 메인 디스패치 = m13.ll:29802(chat.rs:45)의 31-case switch. 필드: objective = handler +0x517(태그)/+0x518/+0x519, plan(BigPlan 태그) = +0x5e8. |  |
| 3 | format! 템플릿 @anon...62 (2바이트 압축 상수) 의 실제 포맷 문자열 | ★확정 = **`"{:?}"`**. m13.ll:69 = `[2 x i8] c"\C0\00"` 는 문자열이 아니라 `core::fmt::Arguments` 의 **압축 템플릿 프로그램**(최신 rustc 인코딩). 호출부(m13.ll:29500~29514)가 `[&self.team_plan.objective, <Option<MainObjective> as Debug>::fmt]` 로 **인자 1개 + Debug 포맷터**를 만들고, 템플릿 2바이트 중 1바이트가 종결자라 **리터럴 문자 조각이 0개** ⟹ 포맷 문자열은 정확히 `"{:?}"`. handle_chat 의 format! 3곳이 이 상수를 공유한다. |  |
| 4 | misunderstood(p8) 가 handle_chat_inner 안에서 어떻게 쓰이는지 | ★확정 — **플랜 판정에 전혀 개입하지 않는다.** 딱 한 곳(m13.ll:33329, chat.rs:598), 함수 에필로그에서만 쓰이고 **Serpen/Morgard 계열 8개 태그(25,27,28,29,34,36,37,38)** 로 들어온 경우에만 도달한다.   `if let Some(t) = objective_chat_target(chat) { if after == Some(t) { if misunderstood { if before != Some(t) { mark_objective_misunderstanding(t, tick) } } else { clear_objective_misunderstanding() } } }`   쓰기 대상 = `TeamPlan+0x90 objective_misunderstanding: Option<ObjectiveMisunderstanding>(48B)`(= handler +0x188). clear → +0x188 = -1(니치 None). mark → +0x188=0, +0x198=0, +0x1a8=tick, +0x1b0=target(JungleType). ⟹ **오브젝티브 '오해' 기록부만 세우고 지운다.** |  |
| 5 | chat.rs:18 의 소스 형태가 `if !enabled { inner(); return; }` 인지 `if enabled {…} else { inner(); }` 인지 IR 만으로는 확정 불가 | ★**확정 — 원리적 불가가 아니었다(r4 명세의 오판정).** !dbg 줄번호로 결정된다: 조건 분기 !36385=**18**, trace-OFF 경로의 handle_chat_inner 호출 !36388=**19**, trace 본문 시작 !36391=**22**, 함수 끝 !36387=**39**. `if enabled {…} else { inner() }` 였다면 else 본문이 then-블록(22~37) **뒤** 줄번호를 가져야 한다. **19 < 22 이므로 조기반환형 확정**: `if !trace_level.is_enabled() { self.handle_chat_inner(...); return; }` |  |
| 6 | PendingTraceEvent 원소 크기 184 를 constants 에서 뺀 것이 맞는지 | ★맞다. `distruct PendingTraceEvent` = **184B · 필드 2**(`0x0 event: TraceEventType(176B)`, `0xb0 tick: usize(8B)`). 184 는 순수 원소 크기 ⟹ SPEC_GUIDE §3 '배열 인덱스·stride 는 적지 않는다' 에 정확히 해당. |  |
| 7 | handler +0x148 / +0x158 (Morgard/Serpen GiveUp 이 기록하는 Option<usize> 2개)의 정확한 필드명 | ★확정 — `distruct LegacyPlanHandler 0x148 / 0x158` 로 중첩 관통. **`+0x148 = team_plan.obj_spawn.epic_giveup_tick : Option<usize>(16B)`**, **`+0x158 = ...serpen_giveup_tick`**. `team_plan` @+0xf8(TeamPlan 1064B), `obj_spawn` @+0x148(`ObjectiveSpawnState` 64B). 그 구조체 전량(절대 오프셋): +0x148 epic_giveup_tick / +0x158 serpen_giveup_tick / +0x168 epic_spawn_call_tick / +0x170 serpen_spawn_call_tick / +0x178 epic_camp_last_visible_tick / +0x180 serpen_camp_last_visible_tick. "Morgard" 가 `epic_*` 로 불리는 것은 `JungleType::Morgard` ↔ `JungleRunner.epic(+0x180)` 대응으로 교차검증된다. |  |
| 8 | `\C0` 옵코드의 비트 정의 — rustc 내부 인코딩이라 IR 에 명세가 없다(원리적 불가) | ★**뒤집혔다 — 문법 전체를 역산했다.** 전문 = `_shared.포맷템플릿_문법`. 전 크레이트 템플릿 상수 6,127개를 전수 디코드해 리터럴런/플레이스홀더/옵션비트 체계를 확정했고, 리터럴 길이가 전부 맞는 것으로 교차검증했다. `"{:?}"` 결론이 옵코드로도 뒷받침된다. |  |
| 9 | GankPlan(50) arm 의 소스 줄번호 — 테일머지되어 !DILocation(line: 0) | ★**확정 = `rule_scope.rs:131`. 별개 arm 이 아니라 같은 or-패턴이다.** 근거: `!DILocalVariable(name:"line", line:131)` 과 `!DILexicalBlock(line:131)` 이 **각 1개뿐**이고, chat_allowed 안의 `line` 바인딩은 131·145 둘뿐이다(145 = 태그 20/21/22). 태그 50 전용 바인딩이 없다. 테일머지는 `DILocalVariable` 을 합치지 않으므로 테일머지가 아니다. GankPlan 만 `line` 필드가 `+0x2`(다른 13개는 `+0x1`)라 phi 로 갈린 것뿐이다. |  |
| 10 | handle_chat_inner 각 arm 의 세부 계산식(HideLine gen_range 확률식 / Battle arm 의 resolve_join_stake·tower_dive_is_viable 조건) | ★대부분 확정. **HideLine(11·12), chat.rs:46~107**: `gank_hp_gate = 70 - roaming_ratio*20/1000`(70~50). `gank_score >= 0` 이면 무조건 수락, **음수일 때만 롤**: ★`should_join = !(rnd.gen_range(0..1000) < ego_ratio*700/1000)` — ego 1000 이면 **70% 확률로 콜 무시**, 0 이면 절대 무시 안 함. **Battle(3·4·6), chat.rs:139~241**: `battle_hp_gate = 60 - aggressive_ratio*20/1000`(60~40), `max_move_sec = roaming_ratio*2/1000 + 4`(4~6초). ★`tower_dive_is_viable` 은 **BattleDive(태그 4) 콜에 한해**, ①거리 ②HP 게이트 ③`dive_abandoned`·`dive_quiet` 없음 ④**직전 다이브 포기 후 `tps*4+1` 틱 경과** 4개를 전부 통과했을 때만 호출된다(앞 4개가 게이트). ★`resolve_join_stake`(_gaibc/m10.ll:41001~42354) 반환 = `Option<FightPrediction>`(64B, None = +0x0 == -1 니치). 구조 = focus_target@+0x0 / soaker@+0x10 / rescue_ally@+0x20 / net_value@+0x30 / **line@+0x38** / line_absolute@+0x39. `FightLine` = **Commit(0)/CommitAfterJoin(1)/Disengage(2)/Hold(3)**(rmeta 주석과 일치). **저울의 역할**: `Commit` 이면 거리·체력 게이트 기각을 **뒤집어 합류**시키고, `Disengage` 면 게이트를 통과했어도 **거부**한다. `can_help()` 만은 절대 우회 불가(AND 바깥). 최종식 = `can_help && ((old_pass && !stake_veto) \|\| stake_commit)`. |  |
| 11 | `\C0` 비트 3~6 — 이 빌드 템플릿 6,127개가 전부 `0b11xxxxxx` 라 관측 불가(재료 한정) | ★**해소(2026-09-11)** — `\C0` 템플릿 flags 비트는 **관측 대상이 아니라 rustc 소스에 정의**돼 있었다. `rust-src`/`rustc-src` 설치 후 `rustc_ast_lowering\srcormat.rs:336~425`(인코더)와 `core\srcmt\mod.rs:316~331`(flags)로 **전량 확정**. 「비트 3~6 미확정」은 **질문 자체가 틀렸다** — 0~20 이 통째로 fill 문자다. 부수로 우리 표의 오류 2건도 잡혔다: ~~비트25=alternate~~ → **23**(25/26은 DEBUG_HEX), ~~0x80~0xFF = 플레이스홀더~~ → **0x80 은 긴 리터럴 런**(u16 길이 뒤따름). 누락 3건(옵션 bit3 명시 인자인덱스 / bit4·5 간접 width·precision)도 보강. 전문 = `shared.포맷템플릿_문법` |  |
| 12 | (미커버) handle_chat_inner 4건 | ➕**보강(2026-09-11 검증배치 C)** — 명세에 없던 4건: ① **`chat.rs:245~290` = 합류 거절 후 글로벌 궁 예약** 통째 누락 (`+0x530 pending_global_ult_target: Option<(usize,usize)>` = 대상 id + `tick + tps*3`) ② **`exit_src` 17~21 의 기록 조건** = `version <= 1 && plan == Battle(9)`(m13.ll:32256·32313) ⟹ **현행 v3 에서 이 계측이 0 인 게 정상**이라는 뜻 ③ `stake`/`stake_commit`/`stake_veto` 줄번호(chat.rs:212/215/216) ④ `ff_call_*` 8칸 계측표 ⚠**부수 함정**: `m13.ll:33188` 의 `store i8 21` 은 `ff_battle_exit` 가 아니라 **`mf_swap.0`(+0x1610)** 이다. |  |
| 13 | 1차의 「오라클 인자 구성 불가」 + chat_allowed/position_exists 표의 신뢰도 | ★**실행으로 전수 검증**(2026-09-11 2차배치C, `C_o12.rs` → **14,950행**). ①자기발화 게이트: `recv==from` 인 **전 행에서 diff 0바이트**(위반 0건) ②`position_exists`: tutorial 9종 × from 5종 **45칸 전수 일치**(Top{0,2,7,8}/Jungle{0,6,8}/Mid{0,4,5,7,8}/Bottom·Support{0,1,3,5,7,8}) ③트레이스 게이트: Off 는 push 0, **Summary·Detailed 둘 다** len +1 ⟹ `is_enabled() = (trace_level != Off)` 확정 ④`misunderstood` 는 이 본문에서 분기하지 않는다 ⑤stake 줄번호 = chat.rs:212/215/216. ⚠부수 함정 재확인: m13.ll:33188 의 `store i8 21` 은 `ff_battle_exit` 가 아니라 **`mf_swap.0`(+0x1610)** 이다(tcx 레이아웃으로 재확인). |  |
| 14 | 12 포맷 템플릿의 `{:?}` 추정(ev5) | ★확정(3차 배치C). `shared.포맷템플릿_문법.결론` 에 이미 답이 있었고(정정이 shared 에만 반영된 사례), IR 직접 근거도 있다 — 인자 배열에 `<Option<MainObjective> as Debug>::fmt` ×2, `<Chat as Debug>::fmt` ×1 store(m13.ll rel 104/137/171). ev 5→4. |  |
</details>

