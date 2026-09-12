---

### `11` v3_fall_back_to_passive — AI v2 이상에서 현재 플랜을 패시브 플랜으로 되돌리는 폴백 — 튜토리얼 스코프 허용 시에만 적용 ※ 게이트는 **version >= 2** 다 — 함수명의 `v3` 가 아니다. 오라클 실행 확증: version 0·1 은 `LegacyPlanHandler` 6168B 스냅샷 diff 전무, 2 부터 `v3_lapse_passive_fallbacks=1`.

| 항목 | 값 |
|---|---|
| id | `handler__v3_fall_back_to_passive` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler23v3_fall_back_to_passive` |
| 소스 | `game-ai\src\plan_legacy\handler.rs:416` |
| IR | `m13.ll` 12238~12479행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::LegacyPlanHandler::v3_fall_back_to_passive` · **pub** |
| 계층 | 플랜 핸들러 |
| exe | `e6fe60` (handler) · 445바이트 · 110명령 |
| 라운드 | 기준 `r4` · 통과 4회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut LegacyPlanHandler(6168B) | 플랜·서브플랜·카운터 기록 대상 | 4 |
| 1 | 2 | version | usize | AI 버전 게이트. version < 2 면 아무것도 안 하고 즉시 return (handler.rs:417) | 4 |
| 2 | 3 | rnd | &mut rand::rngs::std::StdRng(320B, align16) | 본문에서 직접 안 씀 — passive_plan / BigPlan::sub_plan 으로 전달만 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | 본문에서 직접 안 읽음 — 하위 호출로 전달만 | 4 |
| 4 | 5 | data | &OperationData(24B) | data.cache(=&AbstractGameWithCache) 와 data.context(=&GameContext) 를 직접 읽는다 | 4 |
| 5 | 6 | debug | &mut DebugFrameData(224B) | 본문에서 직접 안 씀 — 하위 호출로 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v3_fall_back_to_passive(self, version, rnd, player, data, debug):
 // handler.rs:417 — 버전 게이트
 if version < 2: return // v2 미만은 이 폴백 없음

 // handler.rs:421 — 게임모드 게이트
 game = data.cache.game // &dyn AbstractGame (data ptr +0, vtable +8)
 if game.get_game_mode() /*vtable+0x40*/ == GameMode::DeathMatch(2): return

 // handler.rs:424~427 — 폴백으로 쓸 플랜 만들기
 if game.get_game_mode() == GameMode::SingleLane(1):
 plan = BigPlan::SinglePlanLine( SinglePlanLine{ chats: Vec::new(), in_recall: false, line: LineType::Mid } )
 // IR: plan+0=4(tag), plan+8/0x10/0x18=빈 Vec, plan+0x20=0, plan+0x21=1
 else:
 plan = self.passive_plan(version, rnd, player, data, debug) // 392B sret → 앞 384B 만 plan 으로 복사

 // handler.rs:429 — rule_scope::plan_allowed(context, &plan) 이 전부 인라인됨
 ctx = data.context
 goal = BigPlan::goal(&plan) // rule_scope.rs:101 → BigGoal(24B sret)
 t = ctx.tutorial // GameContext+0x38, TutorialType u8
 allowed = match goal.tag: // rule_scope::goal_allowed, rule_scope.rs:90~96 (함수 본문 L90~L98 · arm 줄 = 92 Line / 93 Epic / 94 Serpen / 95 Jungle / **96 Nexus|Battle|Recall**)
 0 Line(line) => // rule_scope.rs:92 → line_exists(ctx, line) (rule_scope.rs:24~25)
 // → TutorialType::spawn_line_minion(line) (runner.rs:274~278)
 line==Top(0) -> t in {None(0), TopSolo(2), Line(7), Total(8)} // spawn_top_minion, runner.rs:283
 line==Mid(1) -> t in {None(0), MidSolo(4), MidBottom(5), Line(7), Total(8)} // spawn_mid_minion, runner.rs:287
 line==Bottom(2) -> t in {None(0), First(1), Bottom(3), MidBottom(5), Line(7), Total(8)} // spawn_bottom_minion, runner.rs:291
 2 Epic => // rule_scope.rs:93 → morgard_exists(ctx) (rule_scope.rs:45~46)
 // → TutorialType::spawn_epic (runner.rs:262~263)
 t in {None(0), Line(7), Total(8)} // IR: (u8)(t-7) < 250
 3 Serpen => // rule_scope.rs:94 → serpen_exists(ctx) (rule_scope.rs:49~50)
 // → TutorialType::spawn_serpen (runner.rs:266~267)
 t in {None(0), MidBottom(5), Line(7), Total(8)}
 1 Jungle => // rule_scope.rs:95 (인라인 스코프는 TutorialType::player_count, runner.rs:294~295)
 t in {None(0), JungleOnly(6), Total(8)} // IR: ((u8)(t-8) < 249) || (t == 6)
 4 Nexus | 5 Battle | 6 Recall => true // 튜토리얼 스코프 검사 없음, 항상 허용

 if !allowed:
 drop(plan); return // handler.rs:437 — 아무것도 안 바꾸고 종료

 // 허용된 경우에만 상태 반영
 self.mf_note_swap(src=29, tick=game.tick() /*vtable+0x28*/) // handler.rs:432 → self.mf_swap = (29, tick)
 self.v3_lapse_passive_fallbacks += 1 // handler.rs:433
 self.plan = plan // handler.rs:434 (기존 plan drop 후 384B memcpy)
 sp = BigPlan::sub_plan(&mut self.plan, version, rnd, player, data,
 &self.data(0x0,248B), &self.team_plan(0xf8),
 &self.positioning_score(0x990), debug) // handler.rs:435
 SubPlan::merge(&mut self.sub_plan(0x768), sp) // handler.rs:436 — ★2번째 인자는 `&SubPlan` 이 아니라 **값 전달**이다(tcx: `fn(&mut SubPlan, SubPlan)`). 72B 라 Win64 ABI 가 간접전달해 IR 에 포인터로 보일 뿐이다(m13.ll:12466 `readonly captures(none) dereferenceable(72) %8`). 재구현에서 `&` 로 받으면 소유권 이동이 사라진다
 return // handler.rs:437
```

**`mem` 메모리 접근 23건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=0)) | 3 | OK |  |
| 1 | OperationData | 0x8 | context | r | &GameContext — tutorial 판정에 사용 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=1)) | 3 | OK |  |
| 2 | AbstractGameWithCache | 0x0 | game.data_ptr | r | dyn AbstractGame 팻포인터의 데이터 포인터 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 재조회: `game` = +0x0 16B `&dyn AbstractGame` ⟹ 데이터 절반이 +0x0 (`memchk.tsv` i=11 idx=2)) | 3 | OK |  |
| 3 | AbstractGameWithCache | 0x8 | game.vtable | r | dyn AbstractGame 팻포인터의 vtable 포인터 · tcx 정본 대조( tcx: `AbstractGameWithCache.game` 은 +0x0 의 16B `&dyn` 팻포인터라 **+0x8 = vtable 절반**(Rust 팻포인터 ABI). `tcxdict` 는 팻포인터 뒤 절반을 필드로 세지 않는다(METHOD_MAP ⑦ 한계 명시)) | 3 | OK |  |
| 4 | AbstractGame::vtable | 0x40 | get_game_mode | r | divtable.py AbstractGame 0x40 → AbstractGame::get_game_mode. {i64 태그, ptr} 반환. 본문에서 2회 호출(417 게이트 통과 후 421, 424) | 3 | 확인불가(vtable 슬롯) |  |
| 5 | AbstractGame::vtable | 0x28 | tick | r | divtable.py AbstractGame 0x28 → AbstractGame::tick. i64 반환. 허용 판정 통과 후에만 호출(handler.rs:432) | 3 | 확인불가(vtable 슬롯) |  |
| 6 | GameContext | 0x38 | tutorial | r | TutorialType(u8, 0..8). rule_scope 의 모든 허용 판정이 이 한 바이트만 본다 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=6)) | 3 | OK |  |
| 7 | BigGoal | 0x0 | tag | r | BigPlan::goal() 이 sret 로 채운 24B BigGoal 의 판별자(0..6). DISCR_EXACT == variant index · tcx 정본 대조( `tcxdict --enum game_core::BigGoal`: 판별자 enum+0x0(1B, Direct), 태그 0..6 = Line/Jungle/Epic/Serpen/Nexus/Battle/Recall ) | 3 | OK |  |
| 8 | BigGoal | 0x1 | Line.line | r | tag==0(Line) 일 때의 페이로드 LineType(0=Top,1=Mid,2=Bottom) · tcx 정본 대조( `tcxdict --enum game_core::BigGoal`: 페이로드 `Line.line: LineType` = **enum+0x1** ) | 3 | OK |  |
| 9 | LegacyPlanHandler | 0x0 | data | r | GoalData(248B) — BigPlan::sub_plan 의 인자로 &self(248B) 형태로 전달 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=9)) | 3 | OK |  |
| 10 | LegacyPlanHandler | 0xf8 | team_plan | r | TeamPlan(1064B) — BigPlan::sub_plan 인자 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=10)) | 3 | OK |  |
| 11 | LegacyPlanHandler | 0x990 | positioning_score | r | PositioningScoreData(2760B) — BigPlan::sub_plan 인자 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=11)) | 3 | OK |  |
| 12 | LegacyPlanHandler | 0x5e8 | plan | r | 쓰기 직전 old 값을 drop, 이후 sub_plan 계산 시 &self.plan 으로 다시 읽음 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=12)) | 3 | OK |  |
| 13 | LegacyPlanHandler | 0x1628 | v3_lapse_passive_fallbacks | r | +1 하기 위해 읽음(read-modify-write) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=13)) | 3 | OK |  |
| 14 | BigPlan(지역변수 plan) | 0x0 | tag | w | GameMode::SingleLane 경로에서만. 4 = BigPlan::SinglePlanLine (dienum BigPlan) · 지역변수 `plan` 은 BigPlan 전체 384B(`base` 에서 옮긴 크기 주석) | 3 | OK | 4 |
| 15 | BigPlan(지역변수 plan) | 0x8 | SinglePlanLine.chats | w | +0x8=0, +0x10=inttoptr(8), +0x18=0 세 워드가 Vec::new() · tcx 정본 대조( `tcxdict`: `SinglePlanLine` 은 32B struct(chats@0x0 / in_recall@0x18 / line@0x19)이고 BigPlan 페이로드가 **+0x8** 에서 시작 ⟹ chats = BigPlan+0x8 ) | 3 | OK | 빈 Vec (cap/len 0, dangling ptr 0x8) |
| 16 | BigPlan(지역변수 plan) | 0x20 | SinglePlanLine.in_recall | w | · tcx 정본 대조( `tcxdict`: `SinglePlanLine.in_recall@0x18` + 페이로드 시작 +0x8 = **BigPlan+0x20** ) | 3 | OK | 0 (false) |
| 17 | BigPlan(지역변수 plan) | 0x21 | SinglePlanLine.line | w | 싱글레인 모드라 미드 고정 · tcx 정본 대조( `tcxdict`: `SinglePlanLine.line@0x19` + 페이로드 시작 +0x8 = **BigPlan+0x21** ) | 3 | OK | 1 = LineType::Mid |
| 18 | LegacyPlanHandler | 0x1610 | mf_swap.0 | w | handler.rs:432 mf_note_swap(src=29, tick) 인라인. 29 = 이 폴백을 가리키는 src 코드 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=18)) | 3 | OK | 29 |
| 19 | LegacyPlanHandler | 0x1618 | mf_swap.1 | w | 같은 mf_note_swap 인라인(handler.rs:410) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=19)) | 3 | OK | AbstractGame::tick() 결과 |
| 20 | LegacyPlanHandler | 0x1628 | v3_lapse_passive_fallbacks | w | handler.rs:433 — 이 폴백이 실제로 적용된 횟수 카운터 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=20)) | 3 | OK | +1 |
| 21 | LegacyPlanHandler | 0x5e8 | plan | w | handler.rs:434. 기존 plan 은 drop_glue 로 먼저 파기. 예외 경로 대비로 임시 %9 에 복사본을 둔다 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=21)) | 3 | OK | 새로 만든 BigPlan(384B memcpy) |
| 22 | LegacyPlanHandler | 0x768 | sub_plan | w | handler.rs:435~436 — 새 plan 으로 BigPlan::sub_plan 을 구해 기존 sub_plan 에 merge · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=11 idx=22)) | 3 | OK | SubPlan::merge(&mut self.sub_plan, &새 SubPlan) |

**`consts` 상수 19건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 417 | 임계 | AI 버전 하한. version < 2 면 즉시 return — v2 미만에는 이 폴백 자체가 없다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 |
| 1 | 2 | 421 | 태그 | GameMode::DeathMatch 태그. 데스매치면 즉시 return · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 |
| 2 | 1 | 424 | 태그 | GameMode::SingleLane 태그. 이 모드면 passive_plan 을 부르지 않고 SinglePlanLine 을 직접 만든다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 |
| 3 | 4 | 425 | 태그 | BigPlan 판별자 4 = SinglePlanLine (DISCR_EXACT, variant2) · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 |
| 4 | 1 | 425 | 태그 | LineType::Mid 판별자 1 — 싱글레인 플랜의 라인 고정값 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 |
| 5 | 0 | 425 | 태그 | in_recall = false · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 |
| 6 | 0 | 429 | 태그 | TutorialType::None 판별자 0 — 모든 허용집합(라인/에픽/세르펜/정글)에 공통 포함 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 |
| 7 | 1 | 429 | 태그 | TutorialType::First 판별자 1 — Bottom 라인 목표 허용집합에만 포함 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 |
| 8 | 2 | 429 | 태그 | TutorialType::TopSolo 판별자 2 — Top 라인 목표 허용집합 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 |
| 9 | 3 | 429 | 태그 | TutorialType::Bottom 판별자 3 — Bottom 라인 목표 허용집합 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 |
| 10 | 4 | 429 | 태그 | TutorialType::MidSolo 판별자 4 — Mid 라인 목표 허용집합 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 |
| 11 | 5 | 429 | 태그 | TutorialType::MidBottom 판별자 5 — Mid/Bottom 라인 + Serpen 허용집합 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 |
| 12 | 6 | 429 | 태그 | TutorialType::JungleOnly 판별자 6 — Jungle 목표를 추가로 허용시키는 별도 항(icmp eq) · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 |
| 13 | 7 | 429 | 태그 | TutorialType::Line 판별자 7 — 라인 3종 + Epic + Serpen 허용집합 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 |
| 14 | 8 | 429 | 태그 | TutorialType::Total 판별자 8 — 전 목표 허용집합 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 |
| 15 | -7 | 429 | 임계 | Epic 허용 판정 바이어스: (tutorial + (-7)) as u8 < 250 → tutorial ∈ {0,7,8}. Jungle 판정에서는 상한(=u8 249)으로도 쓰임 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수) ⟹ **집합은 실행 확정**이다. 단 리터럴 인코딩(`add i8 %t, -7` 류) 자체는 외연이 같은 표기라 실행으로 갈리지 않는다(범위 명시)) | 2 |
| 16 | -6 | 429 | 임계 | Epic 판정의 부호없는 상한(i8 -6 = u8 250) — 범위비교로 접힌 집합검사 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수) ⟹ **집합은 실행 확정**이다. 단 리터럴 인코딩(`add i8 %t, -7` 류) 자체는 외연이 같은 표기라 실행으로 갈리지 않는다(범위 명시)) | 2 |
| 17 | -8 | 429 | 오프셋가감 | Jungle 허용 판정 바이어스: (tutorial + (-8)) as u8 < 249 → tutorial ∈ {0,8} · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수) ⟹ **집합은 실행 확정**이다. 단 리터럴 인코딩(`add i8 %t, -7` 류) 자체는 외연이 같은 표기라 실행으로 갈리지 않는다(범위 명시)) | 2 |
| 18 | 29 | 432 | 태그 | mf_note_swap 의 src 코드 — '패시브 폴백으로 플랜 교체' 를 뜻하는 태그 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff) — `mf_swap.0 == 29` 관측) | 2 |

**`knobs` 조정점 22건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | AI 버전 하한 | handler.rs:417 | 2 | 2 를 내리면 v1 게임에서도 패시브 폴백이 돌게 되고, 올리면 해당 버전 미만에서 이 폴백이 통째로 죽는다(플랜이 이전 값 그대로 유지됨) · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 | 기존 |
| 1 | 데스매치 제외 | handler.rs:421 | 2 | GameMode::DeathMatch(2) 비교를 없애면 데스매치에서도 패시브 폴백이 걸려 AI가 교전을 중단하고 물러난다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 | 기존 |
| 2 | 싱글레인 고정 라인 | handler.rs:425 | 1 | 1(Mid) 을 0/2 로 바꾸면 싱글레인 모드 폴백 플랜의 목표 라인이 Top/Bottom 으로 바뀐다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 | 기존 |
| 3 | 튜토리얼 스코프 허용집합 | rule_scope.rs:90~95 (goal_allowed / line_exists / morgard_exists / serpen_exists) | 0 | 각 switch 의 tutorial 케이스를 넓히면 튜토리얼/제한 모드에서도 해당 목표(라인·에픽·세르펜·정글)의 패시브 플랜이 채택된다. 반대로 케이스를 빼면 그 목표의 폴백이 통째로 버려져 플랜이 갱신되지 않는다. 일반 대전은 tutorial=None(0) 이라 전 케이스가 이미 허용이므로 실전 영향은 없다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 | 기존 |
| 4 | mf_swap src 코드 | handler.rs:432 | 29 | 진단용 태그값. 바꿔도 판정에는 영향 없고 로그/집계 구분만 달라진다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 | 기존 |
| 5 | 폴백 AI 버전 게이트 | handler.rs:417 / m13.ll:12246 (icmp ult version, 2) | 2 | ★**함수명은 v3 인데 실제 게이트는 version >= 2** — 이름만 믿으면 안 된다. 낮추면 v0/v1 에서도 폴백 발동, 올리면 폴백이 죽는다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 | 신규 |
| 6 | DeathMatch 제외 | handler.rs:421 / m13.ll:12257 | 2 | 이 비교를 빼면 데스매치에서도 패시브 폴백이 돈다(현재는 즉시 return) · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 | 신규 |
| 7 | SingleLane 고정 플랜 | handler.rs:424~425 / m13.ll:12270, 12283~12292 | tag1 → BigPlan tag4(SinglePlanLine) | SingleLane 모드에선 passive_plan 을 **아예 안 부르고** 이 플랜을 하드코딩한다. 싱글레인 AI 개입의 가장 싼 지점 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 | 신규 |
| 8 | 튜토리얼 허용 화이트리스트 6표 | rule_scope.rs:92~95 / m13.ll:53988·53996·54005·54027·54037·54045 | 대상별 상이 | case 를 추가하면 그 튜토리얼 모드에서 해당 목표의 플랜이 폐기되지 않는다. **일반 경기(TutorialType=0)는 6표 전부 통과하므로 무영향** — 튜토리얼 전용 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §R1` 171/171 (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × tutorial 9행 전수)) | 2 | 신규 |
| 9 | mf_swap 출처 코드 | handler.rs:432 / m13.ll:12437 (store i8 29) | 29 | `LegacyPlanHandler+0x1610` 에 기록되는 '왜 플랜이 바뀌었나' 코드. passive_plan 이 돌려준 _mf_src(1~19,27)를 버리고 29 로 덮는다. 살려 쓰면 폴백 사유가 세분화된다 · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 | 신규 |
| 10 | 폴백 카운터 | handler.rs:433 / m13.ll:12442~12445 (+0x1628 += 1) | — | `v3_lapse_passive_fallbacks: usize`. 인게임 검증에서 **폴백이 실제로 발화했는지 세는 무료 계측 지점** · 오라클 실행 확증( 오라클 `_verify5/C/o11.out §S11`(게임모드 게이트 — `SingleLaneGame::new`/`DeathMatchGame::new` 로 modetag 0/1/2 실제 생성, 6168B 바이트 diff)) | 2 | 신규 |
| 11 | SubPlan::merge 보존 화이트리스트 | sub_plan.rs:186~197 / m12.ll:36962~36970 switch | switch 키는 **리맵 인덱스** `(tag>1 ? tag−2 : 6)` 다 — 그 기준 case {4,7,8,9,11,12,15,16} ⟹ **SubPlan 메모리 태그로는 {6 Jungle, 9 Hide, 10 EpicCheck, 11 EpicHunt, 13 SerpenCheck, 14 SerpenHunt, 17 DefenseNexus, 18 Steal}**. ⚠원시 태그로 읽으면 `EpicPoke(12)`·`SerpenPoke(15)` 를 보존 대상으로 착각한다(실제로는 항상 덮어쓰기). 전표 정본 = `shared.SubPlan_merge` | ★case 를 빼면 그 서브플랜이 매 병합마다 통째 덮어써져 진행 상태(check_move·need_recall·focus·last_vision_tick)가 리셋된다. 추가하면 끈질기게 유지된다. **AI 의 '우유부단/고집' 성향을 직접 건드리는 노브** | 4 | 신규 |
| 12 | Steal 시야 틱 max 규칙 | sub_plan.rs:164~165(인라인) / m12.ll:37172~37177 | ugt | last_vision_tick 을 max 로만 올린다(내려가지 않음). 부등호를 뒤집으면 최신 값 우선이 된다 | 4 | 신규 |
| 13 | avoid_unnecessary_tower_trace 산식(Battle arm) | _gaibc/m02.ll:8338 | tactic != 1(Frontline) && with_dive | 상수 1 을 바꾸면 어느 전술에서 타워 추격을 허용할지가 바뀐다. DeathBattle arm 은 with_dive 직결이라 별도 | 4 | 신규 |
| 14 | `v2_assign` 강제 배정(외부 주입점) | handler.rs:551·559 (+0x1802, Option<(u8, LineType)>) | — | Some 이면 `v2_apply_assign_commit` 이 플랜과 함께 `_mf_src` 까지 덮어쓴다 = **외부 주입 지점** · tcx 정본 대조(10차 배치C: `tcxdict LegacyPlanHandler 0x1802` = `v2_assign@Some.0.0: u8` — 오프셋·타입이 tcx 정본과 일치(10차 배치C).) | 3 | 신규 |
| 15 | mf_swap 기록 게이트 | handler.rs:953·998 (m13.ll:21525·21815) | mf_swap.__1 != now_tick | 이 비교를 없애면 틱당 여러 번 사유가 갱신된다(현재는 첫 1회만 남아 뒤 사유가 유실) | 4 | 신규 |
| 16 | 로밍/에고 라인변경 확률 | handler.rs:1918~1919 (m13.ll:6899) | (500 − roaming_ratio) * ego_ratio / 500. ⚠**선행 컷오프가 있다** — `500 − roaming_ratio < 1` 이면 난수를 뽑기도 전에 분기를 건너뛴다 | 올리면 비정글 선수가 배정 라인을 벗어나 fallback_line 으로 가는 빈도가 늘어난다(_mf_src=6) | 4 | 신규 |
| 17 | 정글 캠프 채택 거리 | handler.rs:1986 (m13.ll:7513) | 14400000001 = 120000²+1 | 올리면 더 먼 정글 캠프까지 PassiveJungle(_mf_src=10)로 잡는다 | 4 | 신규 |
| 18 | 카운터정글 라우트 게이트 | handler.rs:719 | strategy+18 == 2 | 카정 채택 전략값 | 4 | 신규 |
| 19 | 라인 수용 상한 | passive_plan:2195/2196/2197 | 배정수 < 3 | 한 라인에 몰릴 수 있는 인원 | 4 | 신규 |
| 20 | 남의 라인 진입 조건 | 동상 | 그 라인 담당 슬롯 == None | 완화하면 라인 중복 커버를 허용한다 | 4 | 신규 |
| 21 | DefenseNexus 병합 유지 필드 | defense_nexus.rs:21~23 | focus 유지 | 유지를 해제하면 넥서스 방어 대상이 매 병합 재선정된다(배회 요인) | 4 | 신규 |

<details><summary>`callees` 피호출자 20건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 1 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 5개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 2 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 5개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 3 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 5개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 4 | goal | game_ai::plan_legacy::types::BigPlan::goal | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> game_core::BigGoal | game-ai\src\plan_legacy\types.rs:132 | False | False | 3 | IR 호출 심볼 일치 |
| 5 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 6 | merge | game_ai::plan_legacy::sub_plan::SubPlan::merge | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, game_ai::plan_legacy::sub_plan::SubPlan) | game-ai\src\plan_legacy\sub_plan\mod.rs:186 | False | False | 3 | IR 호출 심볼 일치 |
| 7 | mf_note_swap | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) | game-ai\src\plan_legacy\handler.rs:409 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 8 | morgard_exists | game_ai::plan_legacy::rule_scope::morgard_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:45 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 9 | passive_plan | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) | game-ai\src\plan_legacy\handler.rs:1855 | False | False | 3 | IR 호출 심볼 일치 |
| 10 | plan_allowed | game_ai::plan_legacy::rule_scope::plan_allowed | pub | fn(&game_core::GameContext, &game_ai::plan_legacy::types::BigPlan) -> bool | game-ai\src\plan_legacy\rule_scope.rs:100 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 11 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 12 | spawn_epic | game_core::TutorialType::spawn_epic | pub | fn(&game_core::TutorialType) -> bool | game-core\src\simulation\game\runner.rs:262 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 13 | spawn_line_minion | game_core::TutorialType::spawn_line_minion | pub | fn(&game_core::TutorialType, game_core::LineType) -> bool | game-core\src\simulation\game\runner.rs:274 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 14 | spawn_serpen | game_core::TutorialType::spawn_serpen | pub | fn(&game_core::TutorialType) -> bool | game-core\src\simulation\game\runner.rs:266 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 15 | sub_plan | game_ai::plan_legacy::types::BigPlan::sub_plan | pub | fn(&mut game_ai::plan_legacy::types::BigPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\types.rs:230 | False | False | 3 | IR 호출 심볼 일치 |
| 16 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 7개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 17 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 7개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 18 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 7개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 19 | v3_fall_back_to_passive | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_fall_back_to_passive | pub | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler.rs:416 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
</details>

⚠**미매칭 5개**: `captures`, `data`, `dereferenceable`, `positioning_score`, `team_plan`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m14.ll:39075) · **형제 41개** (LegacyPlanHandler)

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

**`open` 0건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**

(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)

<details><summary>`closed` 9건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | rule_scope.rs:95 (Jungle 허용)의 원래 소스 표현식 — 인라인 스코프가 TutorialType::player_count(runner.rs:294~295) 로 찍히는데, IR 에는 카운트 반환·비교가 남지 않고 tutorial 태그 집합검사 {0,6,8} 로 완전히 접혀 있다. 다른 모듈(m04.ll:16516~16528)의 player_count 인라인 사이트는 {0,2,4,6,7,8}/{1,3}/{5} 3그룹으로 갈리므로 이 집합과 맞지 않는다 → 'player_count() 를 어떤 값과 비교했는가'를 확정하지 못했다(2회 시도 후 중단). 관측 사실은 t in {None,JungleOnly,Total} 뿐. | 3차 배치C: 값표 전수 → 비교값 5 확정(==5 vs >=5 는 표기 불가) |
| 1 | goal_allowed 의 소스 줄 92~95 중 라인 92=Line, 93=Epic, 94=Serpen, 95=Jungle 은 !DILocation inlinedAt 사슬로 확정했으나, Nexus/Battle/Recall 이 어느 줄의 어떤 패턴(`_ => true` 인지 개별 arm 인지)인지는 IR 에 블록이 하나로 합쳐져(label %61) 확인 불가 | 4차 배치C: history[0] — IR switch 4·5·6 명시 case + default unreachable |
| 2 | AbstractGame::get_game_mode 가 { i64, ptr } 를 반환하는데 ptr 부분(GameMode 페이로드)은 이 함수에서 전혀 쓰지 않아 무엇인지 모름 | 4차 배치C: history[1] — 16B, tag 0/1/2 → &MobaMode/&SingleLaneMode/&DeathMatchMode |
| 3 | get_game_mode 를 handler.rs:421 과 424 에서 두 번 호출한다(CSE 안 됨). 두 번째 호출이 다른 값을 반환할 수 있는지(=trait 구현이 상태의존인지)는 game_core DWARF 가 _gaibc 에 없어 확인 불가 | 4차 배치C(ev3): history[2] — 구체 impl 3개 전부 memory(none) ⟹ 2회 호출이 다를 수 없다. 사고 아님 |
| 4 | passive_plan 이 392B sret 인데 384B 만 plan 으로 복사한다. 뒤 8B 가 무엇인지(패딩인지 별도 반환값인지) 이 범위에서는 확인 불가 | 3차 배치C: tcx sig 가 (BigPlan, u8) — 뒤 8B 는 별도 반환값 |
| 5 | rnd(%2) · player(%3) · debug(%5) 는 본문에서 직접 읽지 않고 passive_plan / BigPlan::sub_plan 으로만 전달 — 내부 사용처는 안 봄 | 3차 배치C: tcx sig 가 (BigPlan, u8) — 뒤 8B 는 별도 반환값 |
| 6 | BigPlan::sub_plan · SubPlan::merge · passive_plan 내부는 담당 범위 밖이라 안 봄 | 3차 배치C: tcx sig 가 (BigPlan, u8) — 뒤 8B 는 별도 반환값 |
| 7 | `SubPlan::merge` 빈 arm 의 줄 배정 — ★**확정(2026-09-11 tcx 독립 확증)**. L188 Hide / L189 Jungle / **L190 SerpenCheck / L191 EpicCheck / L192 EpicHunt / L193 SerpenHunt** / L194 Steal / L196 DefenseNexus. 근거 2중: ①rmeta SourceMap 줄표에서 `줄길이 − 2×len(name) = 63` 이 네 줄 모두 일치 ②tcx 상 리프 `XxxSubPlan::merge` 가 **정확히 8개**이고 이름 길이 {4,5,6,8,9,10,11,12}가 전부 상이해 8줄과 **유일 대응**(상수 61/65/59/67 은 존재하지 않는 이름 길이를 요구해 배제). ⚠**형태 정정**: "빈 4 arm" 이 아니라 **8개 arm 이 같은 RHS(리프 merge 위임)를 공유**하는 구조로 보인다. ⚠부수: `EpicPoke(12)`·`SerpenPoke(15)` 는 switch 에 없어 **항상 덮어쓰기**. 전문 = RE6-09-11_rustc-dev-커스텀드라이버-tcx덤프-MIR가용성판정-0.5.8.md | 본문에 해소 표기가 있다 |
| 8 | `_mf_src` 값에 대응하는 사람이 붙인 이름(`MF_SRC_NAMES` / `FF_BAIL_NAMES` / `MF_OBJCLR_NAMES`) — ★**부재 확정 · 재탐색 금지(2026-09-11, 범위 확장 완료)**. 탐색 범위:   ① SDK `sdk_058\mod-sdk\deps` 의 **rlib·rmeta 308개 전량 바이트 스캔**   ② IR 3크레이트(`_gaibc`/`_gcbc`/`_gvbc`) 전량 + rmeta MIR/Span 경로   ③ 게임 exe (`TeamfightManager2.exe`, `TFM2ModUploader.exe`)   ④ ★**게임 데이터·에셋 번들 전체 — 2,622파일 / 2.95GB**(`bundle_unpacked_full` 1.1GB · `TFM2.gg` 567MB · `db` · `config` · `ModData` · `bundle.game_data` · `mod-sdk-stable`). ASCII·UTF-16LE 양쪽 + **zlib/gzip 압축조각 1,625개를 풀어서** 재검색. 도구 = `MIGundlegrep.py` ⟹ **전 범위 히트 0건.** 유일한 출현은 game_ai rmeta `@0x527ff3` 의 한국어 문서주석 문장(`코드는 simulator MF_SRC_NAMES와 1:1`)뿐이고, 그 주석이 직접 **개발사 내부 시뮬레이터** 소유임을 지목한다. ⟹ 배포되는 어떤 아티팩트에도 실체가 없다. **개발사 소스 없이는 불가.** | 부재 확정 · 재탐색 금지(2026-09-11 범위 확장 완료: SDK deps rlib/rmeta + IR 3 + exe + 게임 번들 2.95GB/zlib 1,625조각 → 히트 0). 어휘는 **재료 부재(전 범위)** 가 정확하다 |
</details>

<details><summary>`history` 정정 이력 14건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 |
|---|---|---|
| 0 | goal_allowed 의 Nexus/Battle/Recall 이 어느 줄의 어떤 패턴인지 확인 불가 | ★확정 — `_ => true` 가 **아니다.** IR switch 에서 4·5·6 이 명시 case 로 열거돼 있고 default 는 unreachable 이다. ★**줄번호도 확정(2026-09-11)**: 소스에서는 `\|` 결합 arm 한 줄 **`rule_scope.rs:96`**(문자수 77 검산 일치). arm 줄표 = 91 match / 92 Line / 93 Epic / 94 Serpen / 95 Jungle / 96 Nexus·Battle·Recall. 전체 판정표와 대상별 튜토리얼 화이트리스트 6종은 `_shared.rule_scope_게이트` 참조. |
| 1 | AbstractGame::get_game_mode 가 { i64, ptr } 를 반환하는데 ptr(GameMode 페이로드)이 무엇인지 모름 | ★확정. `GameMode` = 16B enum, tag 0 `Moba` → `&MobaMode` / 1 `SingleLane` → `&SingleLaneMode` / 2 `DeathMatch` → `&DeathMatchMode`. ptr 은 각 impl 의 **self 안 서브구조체 주소**: Game `self+0xED00`(_gcbc/g15.ll:176809) / SingleLaneGame `self+0xECC8`(g15.ll:227404) / DeathMatchGame `self+0xECC8`(g15.ll:255260). ExpectedGame(g08.ll:200245)은 안쪽 `&dyn AbstractGame` 으로 tail-call 위임. 이 함수는 tag 만 쓰고 ptr 은 한 번도 안 읽는다. |
| 2 | get_game_mode 를 두 번 호출(CSE 안 됨) — 두 번째가 다른 값을 반환할 수 있는지 | ★**불가능(확정).** 구체 impl 3개가 전부 `memory(none)`(_gcbc/g15.ll:334286) — 메모리를 읽지도 않는 순수 함수이고 본문이 `insertvalue {i64 상수, gep}` 뿐이라 태그가 구체 타입으로 고정된다. ExpectedGame 판은 순수 포워더이고 vtable 슬롯 로드는 `!invariant.load`. 호출부도 함수 포인터를 한 번만 로드해 재사용하고 두 호출 사이에 store 가 없다. ⟹ CSE 가 안 된 건 간접호출이라 LLVM 이 순수성을 증명 못 한 것뿐. **재구현 시 1회 호출로 접어도 안전.** |
| 3 | passive_plan 이 392B sret 인데 384B 만 복사한다 — 뒤 8B 가 패딩인지 별도 반환값인지 | ★**패딩 아님. 별도 반환값.** DWARF 반환 타입 = `tuple$<enum2$<BigPlan>, u8>`(m13.ll:86194 !17637, 3136bit=392B). `.0` = BigPlan @+0x0(384B), `.1` = **u8 @+0x180**, 뒤 7B 는 정렬 패딩. 이 u8 은 호출부에서 **`_mf_src`** 로 바인딩된다(m13.ll:88711 !20154) — `LegacyPlanHandler+0x1610 mf_swap: (u8, usize)` 및 `ff_note_battle_swap(self, src:u8, tick:usize)` 계열의 **플랜 출처/스왑 사유 코드**. passive_plan 본문이 쓰는 값 = 1~19, 27. 죽은 필드가 아니다(m13.ll:21518~21520 에서 다른 호출부가 sret+384 를 읽어 `v2_apply_assign_commit` 로 넘긴다). ★단 **이 함수는 그 값을 버리고 리터럴 29 를 박는다**(m13.ll:12436~12439). |
| 4 | BigPlan::sub_plan · SubPlan::merge · passive_plan 내부는 안 봄 | 요지 확정. `passive_plan`(m13.ll:6495~9987, handler.rs:1855) = 3,493줄 대형 선택기, 산출 BigPlan 태그 3 PassiveLine(32경로로 압도적 다수)/7/8/10/12/14/16/17. `BigPlan::sub_plan`(m02.ll:8253~8495) = **순수 디스패처**로 `(tag>1 ? tag-2 : 4)` 로 variant 를 복원해 16 arm 으로 tail-call(특례: ForcePassive·ActiveRecall → SubPlan Recall(5) 상수, SinglePlanBattle → Battle(7) 인라인). `SubPlan::merge` 규칙 전량은 `_shared.SubPlan_merge` 참조. |
| 5 | rnd·player·debug 가 하위로만 전달 — 내부 사용처는 안 봄 | ★확정. `rnd` **사용됨**(passive_plan 이 `Rng::gen_range(rnd,0,1000)` 직접 호출, m13.ll:6902). `player` **사용됨**(passive_plan 이 45개 지점에서 필드 로드). `debug` **판정에 안 쓰임** — passive_plan 본문에 직접 필드 접근 0건, 전부 재전달이고 `BigPlan::sub_plan` 은 두 갈래에 아예 `ptr poison` 을 넘긴다(LLVM 이 죽은 인자로 판정). `DebugFrameData` = texts/lines/circles/logs/positiong_score/infos = **순수 디버그 오버레이 싱크**로 AI 판정에 되먹임 없음. |
| 6 | BigPlan::sub_plan 의 SinglePlanBattle/DeathMatchBattle 인라인 arm 의 페이로드 계산식 | ★확정(_gaibc/m02.ll:8253~8496, types.rs:230). 디스패치 `%15 = (tag>1) ? tag-2 : 4` + `assume(tag≠6)`. **(A) SinglePlanBattle → SubPlan::Battle(7)**(m02.ll:8363): support_target ← BigPlan+0x8/+0x10 그대로 / goal ← BigPlan+0x60/+0x68(sub_goal) / v48_claim_hold_until=0 / **avoid_unnecessary_tower_trace = `(tactic != Frontline) && with_dive`**(m02.ll:8338, battle.rs:55) / with_dive ← +0x90 / dive_local=0 / v48_dodge_claim=0 / tactic ← +0x93 / last_bail_gate=0. 버려지는 필드: region·well_runaway·main_goal·chats·start_tick·help_called·dive_abandoned·dive_tower·main_objective. **(B) DeathMatchBattle → SubPlan::DeathBattle(니치, DeathBattleSubPlan 72B)**(m02.ll:8399): DeathMatchBattle 이 384B = BigPlan 전체 크기라 오프셋이 1:1. tag ← BigPlan 판별자 그대로(0/1 — 그 워드가 곧 `support_target: Option<usize>` 의 discr 이라 니치가 이월) / flee_dir ← +0xa8(24B memcpy) / goal ← +0xe8/+0xf0 / lean ← +0x120 / must_dodge ← +0x173 / **avoid_unnecessary_tower_trace ← +0x170(with_dive) 직결**(★A안과 달리 tactic 조건 없음) / all_in ← +0x174 / stance ← +0x176 / tactic ← +0x177 / scene ← +0x178. |
| 7 | passive_plan 의 `_mf_src` u8 값(1~19, 27) 각각의 사유 — 담당 범위를 2배 넘어 중단 | ★**전수 확정**(47개 store 사이트 + 소비처). 전문 = `_shared.mf_src_코드표`. ★가장 중요한 것: **`mf_swap.__0` 을 load 하는 곳이 `_gaibc` 전체에 0건** — **순수 진단/텔레메트리 상태**이고 판정에 되먹이지 않는다. 그리고 **`ff_note_battle_swap` 은 mf_swap 소비처가 아니라 `ff_battle_exit`(+0x15f8)에 쓴다**(이전 서술 정정). |
| 8 | `_mf_src` 18 과 19 의 의미 차이 — column 정보가 없어 구분 불가 | ★**확정 — 원리적 불가가 아니었다.** `passive_plan` 2189~2199 에서 라인 3개 술어를 선계산한 뒤 `switch %31`(포지션) 5팔로 갈린다. 팔마다 **자기 포지션의 지정 라인**을 고를 때만 18 이고 그 외는 19다.   팔0 Top: 2203=18/Top · 2205=19/Mid · 2207=19/Bot · 2209=18/Top   팔1 Jungle: **네 사이트 전부 19**(지정 라인이 없다 — 결정적 증거)   팔2 Mid: 2239=18/Mid · 2241=19/Top · 2243=19/Bot · 2245=18/Mid   팔3 Bot: 2250=18/Bot · 2252=19/Top · 2254=19/Mid · 2256=18/Bot   팔4: 2260=18/fallback_line ⟹ **18 = 자기 포지션의 지정 라인으로 가는 PassiveLine(1순위·최종폴백), 19 = 다른 라인(빈 라인 커버).** 술어 정체도 확정: 자기 라인 = `line_exists(맵) ∧ 배정수<3`, 남의 라인 = 거기에 **그 라인 담당 슬롯이 None** 까지 요구. 즉 *내 라인은 여력만 있으면 가고, 남의 라인은 비어 있어야 간다.* |
| 9 | 코드 20·25·28 의 발화 조건 — 위치는 확정했으나 조건식은 안 읽었다 | ★확정. **28**(handler.rs:716~722): `!counter_jungle_route_init(+0x1806)` ∧ `PlayerState+0x9c0(info.position) == 1` ∧ `player_count` 이 1..=7 **밖** ∧ `strategy+18 == 2` → 래치 후 `PassiveJunglePlan::new_counter_jungle`. **판당 1회.** **20**(handler.rs:938): 공통 종착점. 도달 5경로 = ①:789 `PlayerState+0x9c0(info.position) != 1` ②:851 BigPlan tag==9 ③:916 tag!=10 ④:931 `main_objective != None` ⑤:933(tag 8 + objective 기록 직후). 직후 939 에서 memcpy(384)로 교체. **25**(handler.rs:1240~1247): :1200 BigPlan tag!=9 → `passive_plan()` → `mf_ret25_src[min(src,31)]++`(1241) → `v2_apply_assign_commit`(1246) → `mf_note_swap(25)`(1247) → 1248 교체. 즉 **25 = "본류 판단이 끝나 패시브로 되돌아가는 재선택"**이고 내부 발원은 25가 가리므로 `mf_ret25_src`(+0x1658, `array$<usize>` 32엔트리) 히스토그램으로 따로 수확한다(주석과 일치). |
| 10 | DefenseNexus merge 에 인라인된 헬퍼(!65699)의 정체 | ★**확정 — 인라인 헬퍼가 아니었다.** `!65699 = DILexicalBlock(scope: !65692, line: 21)` 이고 `!65692` = **`DefenseNexusSubPlan::merge`**(`defense_nexus.rs:20`) 자신의 본문 블록이다.   본문 전문: `fn merge(&mut self, new: &Self) { let keep = self.focus; *self = *new; self.focus = keep; }`   ⟹ **`last_gate`(+0x10)만 새 값으로 갱신하고 `focus` 는 유지한다.** |
| 11 | `_mf_src` 값에 대응하는 사람이 붙인 이름 — IR 만으로는 원리적 복원 불가 | ★**필드 주석은 찾았고, 이름표는 재료 범위를 확정했다.** 필드 주석(rmeta 원본에서 항목↔주석 정렬 복원): `[mf 계측, v3 M1] 마지막 비전투 플랜 채택의 (발원 코드, 틱) — 매크로 여정 폐기/깜빡임 발원 분해 소스.` + `코드는 simulator MF_SRC_NAMES와 1:1. 관측 전용 — 행동·RNG 무영향.` ⟹ `mf` = **매크로 여정 폐기/깜빡임(macro-flicker) 계측군**. 형제 = `mf_ret25_src`, `mf_obj_clear`(`MF_OBJCLR_NAMES`), 기록 함수 `mf_note_swap`(handler.rs:409)·`mf_note_obj_clear`(**team_plan.rs:196**; ~~handler.rs:197~~ 은 오기 — 소유 타입도 `LegacyPlanHandler` 가 아니라 **`TeamPlan`** 이다. 같은 이유로 `mf_obj_clear` 도 `TeamPlan` 필드(team_plan.rs:52)이고 `LegacyPlanHandler` 필드인 것은 `mf_ret25_src`(handler.rs:159, +0x1658)뿐이다 — 11차 배치C). ★**이름 문자열은 배포본에 없다(불가 범위 확정)**: `_docs` 3종 · 3 크레이트 rmeta 원본 바이트 전량 · `_gaibc`/`_gcbc` IR 전량 · `sdk_058\deps` 전 rlib · 게임 exe — **`MF_SRC_NAMES` 실체 0건**(주석 문장 1건만). 시뮬레이터(개발사 내부 도구)를 구하면 즉시 풀린다. 부수: `mf_note_swap` 은 `update:952` 에서 **`mf_swap.1 != now()` 일 때만** 호출 = **틱당 선착 1회**(다른 사이트엔 이 가드가 없다). |
| 12 | 1차(REPORT_C §2)의 「&PlayerState·&OperationData·&mut DebugFrameData 는 오라클 구성 불가」 | ★**거짓이었다 — 실행으로 반증**(2026-09-11 2차배치C). `C_o11.rs`·`C_o11b.rs` 가 컴파일·링크·실행 전부 성공(패닉 0). 관측은 ★**`LegacyPlanHandler` 6,168B 스냅샷 바이트 diff**(필드가 pub 이 아니어도, 반환형이 `()` 여도 된다). 실행 일치: ①`version<2` 게이트(ver 0/1 → **diff 0바이트**) ②DeathMatch 즉시 return(diff 0바이트) ③SingleLane → `BigPlan` tag 4 + `+0x8=0/+0x10=inttoptr 8/+0x18=0/+0x20=0/+0x21=1` **1바이트도 안 어긋남** ④`mf_swap=(29,tick)` ⑤`v3_lapse_passive_fallbacks=1` ⑥writes 5구역(0x5e8·0x768·0x1610·0x1618·0x1628) **밖 변화 0건** ⑦튜토리얼 화이트리스트 9×5 = **45칸 전수 일치**. ⟹ 이 명세의 신뢰도는 「IR 추론」이 아니라 **「오라클 실행」**이다. 정정 0건. |
| 13 | `passive_plan` 반환 392B 중 뒤 8B 가 패딩인지 반환값인지 | ★**별도 반환값**(3차 배치C). tcx: `passive_plan(...) -> (BigPlan, u8)`. 392 = 384 + u8 + 패딩 7 이고 **이 호출자는 u8 을 버린다**. ★근거가 이미 같은 명세의 `callees[]` 안에 있었다 — 자동 생성 필드를 안 본 사례. ev 4→3. 남은 미탐색 = 그 u8 의 의미(private 이라 오라클 불가). |
</details>

