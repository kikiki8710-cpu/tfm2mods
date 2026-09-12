---

### `18` v3_epicops_buff_window — 에픽(오브젝트) 국면에서 수리·세르펜징벌 목표를 선점하고, 아니면 압박 라인 변경 채팅을 대표 1명이 발화

| 항목 | 값 |
|---|---|
| id | `old_epic__v3_epicops_buff_window` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epicNtNtB6_9team_plan8TeamPlan22v3_epicops_buff_window` |
| 소스 | `game-ai\src\plan_legacy\old\epic.rs:634` |
| IR | `m09.ll` 6879~7264행 |
| 경로·가시성 | `game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | `dce220` (epic) · 764바이트 · 195명령 |
| 라운드 | 기준 `r6` · 통과 6회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 1 | self |  | objective/chats/eo_serpen_punish_issues/v3_press_chat_line 을 갱신하는 쓰기 대상 |
| 1 | 2 | version |  | AI 버전 게이트. 이 함수 본문엔 version 분기가 없고 v3_serpen_contest_clear_win 으로 그대로 전달만 한다 |
| 2 | 3 | rnd |  | PlayerState::strategy 호출에만 넘긴다(전략 샘플링용). 본문에서 직접 안 쓴다 |
| 3 | 4 | player |  | info.team(+0x930)·info.position(+0x9c0) 두 필드만 직접 읽는다 |
| 4 | 5 | data |  | cache(+0x0) 만 직접 역참조. cache.game(&dyn AbstractGame) 과 cache.player_champion(+0x1e0) 을 쓴다 |
| 5 | 6 | goal_data |  | is_object_being_taken_by_enemy 로 그대로 전달만. 이 함수는 필드를 안 읽는다 |
| 6 | 7 | plan |  | v3_epicops_repair_need 로 그대로 전달만. 이 함수는 필드를 안 읽는다 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v3_epicops_buff_window(&mut self, version, rnd, player, data, goal_data, plan) -> bool {

// [1] 수리(Repair) 선점 — epic.rs:635
let team = player.info.team; // PlayerState+0x930
match v3_epicops_repair_need(player, data, plan) { // i8 반환. IR 인자 `(team, data.cache, plan)` 은 **ArgumentPromotion 아티팩트**라 소스 인자와 다르다. tcx sig = fn(&PlayerState, &OperationData, &BigPlan). promotion 이 일어났다는 것 자체가 'player 에서 info.team 만, data 에서 cache 만 읽는다'의 증명
 1 => { // 637~638
 self.objective = Some(MainObjective::Repair); // TeamPlan+0x41f = 7
 self.chats.push(Chat::Repair(0)); // 태그 23, usize 필드 0
 return true;
 }
 2 => { // 642
 self.objective = Some(MainObjective::Repair); // 태그 7 만, 채팅 없음
 return true;
 }
 _ => {}
}

// [2] 적이 오브젝트를 먹는 중 + 세르펜 교전에서 확실히 이긴다 → 세르펜 징벌 — 651~654
if is_object_being_taken_by_enemy(player, data, goal_data, self, WavePriorityObject::Serpen /* ★IR 의 5번째 인자 i1 은 1B 열거형의 ABI 표현이다(불리언 아님). tcx sig = fn(&PlayerState,&OperationData,&GoalData,&TeamPlan,WavePriorityObject) -> bool, 0=Morgard/1=Serpen */) // 651
 && v3_serpen_contest_clear_win(version, player, data, self) { // 652
 self.eo_serpen_punish_issues += 1; // 653, +0x410
 self.objective = Some(MainObjective::Serpen{ phase: ObjectPhase::Setup, with_battle: true });
 // 654, +0x41f=1 +0x420=1 +0x421=1
 self.chats.push(Chat::SerpenSetup(0)); // 태그 25
 return true;
}
// ★분기 순서 주의: is_object_being_taken_by_enemy 가 false 면 곧장 [3] 으로 간다.
// true 인데 v3_serpen_contest_clear_win 이 false 여도 [3] 으로 간다(단락 아님, 같은 합류 블록).

// [3] 압박 라인(group_line) 판정과 채팅 — 664~679
let strategy = player.strategy(rnd, data.cache.game); // 664, sret 24B 스택 로컬
let mu = strategy.morgard_use; // Strategy+0x4 (8B enum, i64 로 통째)
let group_line: Option<LineType> = v3_epic_group_line(mu, player, data); // 665
if group_line == None /* -1 */ { return false; } // 665

if self.v3_press_chat_line != group_line { // 666, Option<LineType>::ne

 // 발표자(announcer) 고르기 — 667~670, (0..5).position(closure) 가 통째로 인라인됨
 let announcer: Option<usize> = (0..5).position(|x| {
 data.cache.player_champion[team][x].is_some() // 668, null=None
 && v3_epic_formation_role(mu, Position::from_index(x), player, data)
 .is_some_and(|f| !f.is_split) // 669~670
 });
 // IR 은 슬롯 0..4 를 완전 언롤했다. 각 슬롯에서
 // ptr==null → 다음 슬롯
 // role==None(byte0==2) → 다음 슬롯
 // role.is_split==true → 다음 슬롯
 // 그 외(뭉치는 역할) → 그 인덱스가 announcer
 // 판정식 원문: xor(byte0 != 2, byte0 & 1) == is_some && !is_split

 if announcer == Some(player.info.position.as_index()) { // 671~678, PlayerState+0x9c0
 let chat = if self.v3_press_chat_line.is_none() { // epic.rs:673 의 is_none 이 인라인 (IR 에 섞여 보이는 682 는 `core/src/option.rs` 줄번호다). Chat 구성은 674/676
 Chat::Press(group_line_unwrapped, 0) // 태그 21
 } else {
 Chat::PressChange(group_line_unwrapped, 0) // 태그 22
 };
 self.chats.push(chat); // 태그@+0, LineType@+1, usize 0 @+8
 }

 self.v3_press_chat_line = group_line; // 679 — 발화 여부와 무관하게 항상 갱신
}
return false;
}
```

**`mem` 메모리 접근 18건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. v3_epicops_repair_need 의 1번 인자이자 cache.player_champion[team] 의 팀 인덱스. 사용 직전 team<2 경계검사(panic_bounds_check)를 받는다 · tcx 정본 대조( offset_of!(PlayerState, info.team)=+0x930 MISMATCH 0 (o1.txt)) | 3 |
| 1 | PlayerState | 0x9c0 | info.position | r | Position(i32, range 0..5). Position::as_index() 로 usize 화해서 announcer 와 비교 — '내가 발표자인가' 판정 · tcx 정본 대조( offset_of!(PlayerState, info.position)=+0x9c0 MISMATCH 0 (o1.txt)) | 3 |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache. gep 없이 바로 load 된다 · tcx 정본 대조( offset_of!(OperationData, cache)=+0x0 (o1.txt)) | 3 |
| 3 | AbstractGameWithCache | 0x0 | game(데이터 포인터) | r | ref$<dyn AbstractGame> 의 첫 워드. PlayerState::strategy 의 3번 인자로 전달 · tcx 정본 대조( offset_of!(AbstractGameWithCache, game)=+0x0 (o1.txt)) | 3 |
| 4 | AbstractGameWithCache | 0x8 | game(vtable 포인터) | r | dyn 의 둘째 워드. IR 이 dereferenceable(816) 로 표시하는데 divtable.py 가 보고한 AbstractGame vtable 총 크기 816B(슬롯 100개)와 정확히 일치해 vtable 로 확정 | 3 |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion | r | array 총 80B = [5 x ptr] x 2팀. player_champion[team][x] 를 x=0..4 로 훑으며 null(=None) 여부를 본다 · tcx 정본 대조( offset_of!(AbstractGameWithCache, player_champion)=+0x1e0 (o1.txt) + 실제 [[Option<&Entity>;5];2] 인덱싱 동작(o3/o4/o5 HP 행)) | 3 |
| 6 | Strategy(스택 로컬 24B) | 0x4 | morgard_use | r | MorgardUseStrategy(8B enum). i64 로 통째 읽어 v3_epic_group_line·v3_epic_formation_role 의 1번 인자로 넘긴다 · tcx 정본 대조( offset_of!(Strategy, morgard_use)=+0x4 (o1.txt) + 태그 직독 MUTAG 5/6/암묵 (o2.txt)) | 3 |
| 7 | TeamPlan | 0xc0 | chats.cap | r | Vec<Chat> 의 용량. len==cap 이면 RawVec::grow_one · tcx 정본 대조( offset_of!(TeamPlan, chats)=+0xc0 + Vec 3워드 순서 실측(w0=cap=4, w2=len=1) (o1.txt VEC 행)) | 3 |
| 8 | TeamPlan | 0xc8 | chats.ptr | r | Vec<Chat> 데이터 포인터. 원소 stride 24B({i8,[23 x i8]}) · tcx 정본 대조( chats+0x8=ptr — Vec 워드 순서 실측(o1.txt VEC) + tcxaudit chats.buf.inner.ptr) | 3 |
| 9 | TeamPlan | 0xd0 | chats.len | r | Vec<Chat> 길이. 읽고 +1 해서 되쓴다 · tcx 정본 대조( chats+0x10=len — Vec 워드 순서 실측(o1.txt VEC, len_api 일치)) | 3 |
| 10 | TeamPlan | 0x410 | eo_serpen_punish_issues | r | usize. 세르펜 징벌 경로에서 읽어 +1 · tcx 정본 대조( offset_of!(TeamPlan, eo_serpen_punish_issues)=+0x410 (o1.txt)) | 3 |
| 11 | TeamPlan | 0x41e | v3_press_chat_line | r | Option<LineType>(1B, None=0xFF). ① group_line 과 같으면 즉시 종료 ② None 이면 Chat::Press, Some 이면 Chat::PressChange 선택 · tcx 정본 대조( tcx adt TeamPlan v3_press_chat_line=+0x41e(Option<LineType>) — private 필드라 offset_of! 불가, 대신 +0x41e 에 raw 기록/판독이 일관(o4/o5) · Option<LineType>::None 니치=0xff 실측(o1.txt)) | 3 |
| 12 | TeamPlan | 0x41f | objective | w | repair_need 가 1 또는 2 일 때. dienum MainObjective 7 = Repair(페이로드 없음)이라 태그 1바이트만 쓴다 | 3 |
| 13 | TeamPlan | 0x41f | objective | w | dienum MainObjective 1 = Serpen, 페이로드 enum+0x1 phase(ObjectPhase), enum+0x2 with_battle(bool). 저장값 1/1 = phase=Setup, with_battle=true | 3 |
| 14 | TeamPlan | 0x410 | eo_serpen_punish_issues | w | 세르펜 징벌 경로 카운터 · tcx 정본 대조( offset_of!(TeamPlan, eo_serpen_punish_issues)=+0x410 (쓰기 대상 동일 오프셋, o1.txt)) | 3 |
| 15 | Chat(chats 버퍼 원소, 24B) | 0x0 | Chat 태그 (Repair 23 / SerpenSetup 25 / Press 21 / PressChange 22) | w | 원소 24B. 태그 @+0, Press/PressChange 는 LineType @+1, 공통 usize 필드 @+8 에 항상 0 을 넣는다. ★**베이스는 TeamPlan 이 아니라 힙 버퍼다** — TeamPlan+0xc8(chats.ptr)은 이 함수에서 **읽기 전용**이고(쓰기 0건) 그 읽은 포인터가 가리키는 원소에 쓴다. 필드 쪽 읽기는 mem[8] 이 이미 담고 있다 ★**행 분리 **: 한 행에 오프셋을 묶으면 `tcxaudit` 기계 검사가 무력화된다. 14 는 3차에 같은 이유로 분리됐는데 **18 만 남아 있었다.** | 3 |
| 16 | TeamPlan | 0xd0 | chats.len(push 로 +1) | w | 원소 24B. 태그 @+0, Press/PressChange 는 LineType @+1, 공통 usize 필드 @+8 에 항상 0 을 넣는다 ★**행 분리 **: 한 행에 오프셋을 묶으면 `tcxaudit` 기계 검사가 무력화된다. 14 는 3차에 같은 이유로 분리됐는데 **18 만 남아 있었다.** | 3 |
| 17 | TeamPlan | 0x41e | v3_press_chat_line | w | 압박 경로 마지막에 무조건 갱신 — 채팅을 실제로 뿌렸든(내가 발표자) 안 뿌렸든 갱신한다 · tcx 정본 대조( tcx adt TeamPlan v3_press_chat_line=+0x41e (쓰기 대상 동일 오프셋)) | 3 |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 635 | 태그 | v3_epicops_repair_need 반환 태그 1 — '수리 필요 + 채팅까지'. objective=Repair 로 놓고 Chat::Repair 를 발화한다 | 4 |
| 1 | 2 | 635 | 태그 | v3_epicops_repair_need 반환 태그 2 — '수리 필요하지만 채팅은 없음'. objective=Repair 만 놓는다 | 4 |
| 2 | 7 | 637 | 태그 | MainObjective::Repair 의 태그값(dienum: 태그=variant 인덱스, 밀림 없음). 637줄·642줄 두 곳에서 저장 | 3 |
| 3 | 23 | 638 | 태그 | Chat::Repair 의 태그값(24B 열거형판 Chat) · 오라클 실행 확증( Chat::Repair(0) 실값의 태그바이트 직독 = 23 (o1.txt)) | 2 |
| 4 | 25 | 658 | 태그 | Chat::SerpenSetup 의 태그값 · 오라클 실행 확증( Chat::SerpenSetup(0) 태그바이트 직독 = 25 (o1.txt)) | 2 |
| 5 | 21 | 674 | 태그 | Chat::Press 의 태그값. v3_press_chat_line 이 None 일 때(최초 압박 선언) 선택. 줄번호 = is_none 판정 **epic.rs:673**, Chat 구성 **674/676**. ⚠인라인 체인에 섞여 보이는 682 는 **`core/src/option.rs`** 줄번호이고, epic.rs:682 자체는 함수 꼬리(lifetime.end + 공통 출구)다 · 오라클 실행 확증( Chat::Press(LineType::Top,0) 태그바이트 직독 = 21 (o1.txt)) | 2 |
| 6 | 22 | 676 | 태그 | Chat::PressChange 의 태그값. v3_press_chat_line 이 이미 Some 일 때(압박 라인 변경) 선택. 줄번호는 is_none 판정 **epic.rs:673** 이고 Chat 구성은 **674/676**. ⚠인라인 체인에 보이는 682 는 **`core/src/option.rs`** 줄번호이며, epic.rs:682 자체는 함수 꼬리(lifetime.end + 공통 출구)다 · 오라클 실행 확증( Chat::PressChange(LineType::Top,0) 태그바이트 직독 = 22 (o1.txt)) | 2 |
| 7 | -1 | 665 | 센티널 | Option<LineType> 의 None 니치값(0xFF). ① group_line==-1 이면 즉시 false 반환 ② v3_press_chat_line==-1 이면 Press vs PressChange 를 가른다. v3_epic_group_line 반환의 IR range 는 [-1,3) = {None, Top, Mid, Bottom} · 오라클 실행 확증( Option<LineType>::None 니치바이트 직독 = 0xff (o1.txt) + v3_epic_group_line 이 실제로 None 을 내는 케이스 관측(JungleOnly 전 mu, First/Bottom+Split14) (o2.txt)) | 2 |

**`knobs` 조정점 27건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | 수리 선점의 우선순위 | epic.rs:635 (v3_epicops_repair_need 반환값 해석) | 1 / 2 | 반환 1은 '목표=Repair + 팀채팅', 2는 '목표=Repair, 조용히'. 둘 다 true 를 반환해 이후 세르펜·압박 판정을 통째로 건너뛴다. 2를 1로 바꾸면 수리 상황에서 채팅이 늘고, 이 두 case 를 지우면 수리보다 세르펜·압박이 우선하게 된다 | 4 |
| 1 | 세르펜 징벌 진입 조건(2중 AND) | epic.rs:651~652 | is_object_being_taken_by_enemy(…, WavePriorityObject::Serpen /*=1*/) && v3_serpen_contest_clear_win(version, …) ※5번째 인자는 `WavePriorityObject`(1B C-enum, 0=Morgard/1=Serpen)이고 IR 의 i1 은 그 ABI 표현이다 | 둘 다 참이어야 목표를 세르펜(Setup, 전투 포함)으로 확정한다. clear_win 쪽을 완화하면 불리해도 오브젝트 뺏김에 달려들고, 강화하면 상대에게 그냥 내준다 | 4 |
| 2 | 세르펜 진입 시의 phase/with_battle | epic.rs:654 | phase=ObjectPhase::Setup(1), with_battle=true(1) | phase 를 Assemble(2)/Hunt(3) 로 올리면 준비 단계를 건너뛰고 더 공격적으로 붙는다. with_battle=false 로 두면 교전 의사 없이 오브젝트만 노린다 | 4 |
| 3 | 발표자 선정 규칙 | epic.rs:667~670 | 슬롯 0..4 중 '챔피언이 존재하고 formation_role 이 Some 이며 is_split=false' 인 최초 인덱스 | 내 포지션 인덱스가 그 인덱스와 같을 때만 압박 채팅을 발화한다. 조건을 !is_split 대신 is_split 으로 뒤집으면 스플릿 담당자가 발표자가 되고, 최초 인덱스 대신 특정 슬롯을 고정하면 항상 같은 포지션이 콜을 한다 · 오라클 실행 확증( v3_epic_formation_role(pub) 진리표 450칸 + 발표자 산출 90행 — Gather→슬롯0, Split14(Top)/Split131(Top,*)→슬롯1 (o2.txt GL/ANN)) | 2 |
| 4 | 압박 채팅 중복 억제 | epic.rs:666 · 679 | self.v3_press_chat_line != group_line 일 때만 진입, 진입하면 무조건 group_line 으로 갱신 | 같은 라인을 연속 압박하는 동안은 채팅이 안 나간다. 이 비교를 지우면 매 판단마다 Press/PressChange 채팅이 도배된다. 반대로 679 갱신을 지우면 매번 PressChange 가 반복된다 | 4 |
| 5 | 첫 압박 vs 라인 변경 문구 | epic.rs:673 (`Option::is_none` 인라인) -> 674/676 에서 Chat 구성. ⚠인라인 체인에 보이는 682 는 core/src/option.rs 줄번호다 | None → Chat::Press(21), Some → Chat::PressChange(22) | 문구 선택만 바꾼다. 판정 자체에는 영향이 없다 | 4 |
| 6 | 수리 우선순위1 부상 HP% 임계 | m09.ll:65185 (icmp ult .., 41 — 슬롯 5개 언롤 사본 65185·65219·65254·65289·65324 중 첫 번째) | 41 | 올리면 더 멀쩡한 팀에도 '수리' 선점이 걸린다 | 4 |
| 7 | 수리 우선순위1 부상자 수 임계 | m09.ll:65338 (icmp samesign ugt i64 %141, 1) | > 1 (2명 이상) | 0 으로 내리면 1명만 다쳐도 수리 | 4 |
| 8 | 수리 우선순위2 '건강' HP% 임계 | m09.ll:65390 (ugt .., 29 — 슬롯 5개 언롤 사본 65390·65424·65459·65494·65529 중 첫 번째) | 29 | 올리면 우선순위2 가 훨씬 자주 발동 | 4 |
| 9 | 우선순위1 봉쇄 게이트 | m09.ll:65042 (icmp eq i8 %58, 5) | BigGoal::Battle | 이 비교를 죽이면 전투 중에도 수리 선점이 가능해진다 | 4 |
| 10 | 세르펜 아군 전력 산입 HP% | m05.ll:62483 (ugt .., 39) | 39 | 내리면 빈사 아군까지 전력에 포함 → 징벌 진입 증가 | 4 |
| 11 | 오브젝트 피탈 판정 시간창 | objective_helpers.rs / m15.ll (mul .., 20) | tps*20 | 올리면 '적이 가져가는 중'으로 더 오래 본다 | 4 |
| 12 | 압박라인 타워 점수표 | m09.ll:64526 (ugt i8 .., 4) | TowerType>4 → score 1 | 뒤집으면 '안쪽 타워 남은 라인' 을 우선하게 된다 | 4 |
| 13 | 압박라인 정렬 키 | m09.ll:64620~64650 | (score, L==Mid, L==obj_line) | `L==Mid` 항을 빼면 미드 편향이 사라진다 | 4 |
| 14 | 오브젝트 쪽 라인 판정식 | m09.ll:64451 / 64330 / 64611 | epic.next_respawn > serpen.next_respawn | 극성을 뒤집으면 스플릿 담당자가 반대 라인으로 간다 | 4 |
| 15 | 후보 라인 집합(튜토리얼별) | @anon….291/.290/.35/.36/.31 + line_exists(m13.ll:53138) | 정적표 | 튜토리얼 모드에서 압박 대상 라인을 강제 변경 · 오라클 실행 확증( v3_epic_group_line(pub) 을 TutorialType 9종 × morgard_use 5종 × 팀2 로 전수 실행 — {None,Line,Total}→Mid / TopSolo→Top / {First,Bottom}→Bottom(Split14 는 None) / MidSolo·MidBottom→Mid / JungleOnly→전부 None (o2.txt GL)) | 2 |
| 16 | 발표자 선정식 | m09.ll:7156~7159 | announcer == player.info.position | 이 비교를 지우면 전원이 채팅(스팸). 고정 인덱스로 바꾸면 특정 포지션 전담 | 4 |
| 17 | 대표 자격 술어 | m09.ll:7080~7083 | Some(f) && !f.is_split | `!f.is_split` 을 빼면 스플릿 담당자도 대표가 될 수 있다 · 오라클 실행 확증( role 바이트0 과 `is_some_and(\|f\| !f.is_split)` 이 450/450 에서 b0==0 ⟺ ok==true 로 일치 (o2.txt GL b0/ok 열)) | 2 |
| 18 | Strategy.morgard_use 직접 강제 | World+0xb3b0 의 [Strategy;2], 팀별 24B stride, +0x4 | 5=Gather / 6=Split14 / 0~4=Split131(값=첫 Position) | 모르가드 교전 대형을 직접 고정. v3_epic_group_line·v3_epic_formation_role 의 3분기가 결정된다 · 오라클 실행 확증( MorgardUseStrategy 5종 실값의 워드 직독 — Gather low32=5 / Split14 low32=6·high32=Position / Split131 low32=position1·high32=position2 (o2.txt MUTAG)) | 2 |
| 19 | Game::set_strategy(team, &Strategy) | _gcbc/g15.ll:176782 | 24B memcpy | 한 방으로 팀 전략 13개 필드 전부 교체 — 개별 필드 패치보다 이 진입점이 싸다 | 4 |
| 20 | 팀 성향 난수 분포 | _gcbc/g15.ll:5094 (gen_range(0,3) / gen_range(0,5)) | 균등 | 팀 생성 시 대형 분포를 편향시킨다. Split131 은 p1≠p2 재추첨 루프가 있다 | 4 |
| 21 | 팀 성향을 난수 대신 지정 | _gcbc/g01.ll:106699·106716 (TeamSetting::create) | — | **유일한 단일 개입점** | 4 |
| 22 | 세르펜 "도달 가능" 거리 여유 | serpen.rs:26 | 150000 | 캠프 기준 도달 판정 완화/강화 | 4 |
| 23 | 적 건강 하한 | serpen.rs:30·37 | hp*100/max > 49 | 전력 산입 기준(50% 초과) | 4 |
| 24 | 미드라인 근접 밴드 | map_regions.rs is_near_mid_line | 192001 / 192000 / 64000 | 미드 근접 판정 폭 | 4 |
| 25 | resolve_fight 히스테리시스 | fight_model.rs:510 | hyst = mean(ally_values) | Commit/Disengage 사이 Hold 구간의 폭 | 4 |
| 26 | 팀 색깔전략 활성 개수 | team.rs:2450 | gen_range(5..=9) | 필드당 Some 확률 7/12 를 직접 좌우 | 4 |

<details><summary>`callees` 피호출자 19건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 |
| 2 | from_index | game_core::Position::from_index | pub | fn(usize) -> game_core::Position | game-core\src\simulation\entity.rs:622 |
| 3 | from_index | game_core::ChampionTier::from_index | pub | fn(usize) -> game_core::ChampionTier | game-core\src\data\team.rs:59 |
| 4 | from_index | game_core::GamingHouseLevel::from_index | pub | fn(usize) -> game_core::GamingHouseLevel | game-core\src\data\team.rs:108 |
| 5 | is_object_being_taken_by_enemy | game_ai::plan_legacy::team_plan::objective_helpers::is_object_being_taken_by_enemy | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:313 |
| 6 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 |
| 7 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 |
| 8 | push | <game_core::AthleteStat as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::setting::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\setting\athlete.rs:167 |
| 9 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 |
| 10 | push | <game_core::TrainingExp as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:1137 |
| 11 | strategy | <game_core::Game as game_core::AbstractGame>::strategy | pub | fn(&game_core::Game, usize) -> game_core::Strategy | game-core\src\simulation\game.rs:1830 |
| 12 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 |
| 13 | strategy | <game_core::ExpectedGame<'a> as game_core::AbstractGame>::strategy | pub | fn(&game_core::ExpectedGame<'a/#0>, usize) -> game_core::Strategy | game-core\src\simulation\expected_game.rs:57 |
| 14 | v3_epic_formation_role | game_ai::plan_legacy::old::v3_epic_formation_role | pub | fn(game_core::MorgardUseStrategy, game_core::Position, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::old::V3EpicFormation> | game-ai\src\plan_legacy\old\epic.rs:781 |
| 15 | v3_epic_group_line | game_ai::plan_legacy::old::v3_epic_group_line | pub | fn(game_core::MorgardUseStrategy, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\old\epic.rs:814 |
| 16 | v3_epicops_buff_window | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool | game-ai\src\plan_legacy\old\epic.rs:634 |
| 17 | v3_epicops_repair_need | game_ai::plan_legacy::old::epic::v3_epicops_repair_need | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::types::BigPlan) -> u8 | game-ai\src\plan_legacy\old\epic.rs:833 |
| 18 | v3_serpen_contest_clear_win | game_ai::plan_legacy::old::v3_serpen_contest_clear_win | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan) -> bool | game-ai\src\plan_legacy\old\serpen.rs:52 |
</details>

⚠**미매칭 1개**: `grow_one`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:14828) · **형제 55개** (TeamPlan)

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
| 41 |  |  |  |
| 42 |  |  |  |
| 43 |  |  |  |
| 44 |  |  |  |
| 45 |  |  |  |
| 46 |  |  |  |
| 47 |  |  |  |
| 48 |  |  |  |
| 49 |  |  |  |
| 50 |  |  |  |
| 51 |  |  |  |
| 52 |  |  |  |
| 53 |  |  |  |
| 54 |  |  |  |

**`open` 1건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | version(2번 인자)은 v3_serpen_contest_clear_win 에만 전달된다. IR range(i64 2,0) 이 '2 이상'을 뜻하는 것 외에 이 함수 안에서의 버전 분기는 없다. ★**반증 시도 2회 모두 실패 = 유지**(6차 배치D): ①본문 IR 전량(m09.ll 6879~7264)에서 인자 `%1` 의 출현은 `define` 줄과 `v3_serpen_contest_clear_win` 호출 **딱 2회**뿐이다. ②`range(i64 2,0)` 의 **출처가 확정**됐다 — 유일 호출부 `objective_handlers.rs:1119`(m09.ll:14815 `icmp ugt i64 %1, 1`)가 **version > 1 일 때만** 이 함수를 부르기 때문이다(version <= 1 이면 같은 자리에서 `handle_press_epic` 이 불린다). 즉 '2 이상'은 이 함수의 가정이 아니라 **호출부 게이트의 그림자**다 | 4 |  |

<details><summary>`closed` 10건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | goal_data(6번 인자, GoalData 248B) 와 plan(7번 인자, BigPlan 384B) 은 이 함수가 필드를 하나도 안 읽는다 — 각각 is_object_being_taken_by_enemy / v3_epicops_repair_need 로 그대로 넘길 뿐이라 어떤 필드가 판정에 쓰이는지는 이 범위에서 확정 불가 |  |  |
| 1 | is_object_being_taken_by_enemy 의 5번째 인자 리터럴 true 의 의미(파라미터 이름) — 피호출 함수 밖이라 확정 못 함 |  |  |
| 2 | Chat::Repair / SerpenSetup / Press / PressChange 가 공통으로 갖는 usize 필드(enum+0x8)에 항상 0 을 넣는데, dienum 이 준 이름이 __0/__1 뿐이라 그 0 의 의미(틱? 대상 id?) 확정 불가 |  |  |
| 3 | epic.rs 655~663 줄에 해당하는 IR 이 없다. 세르펜 경로의 chat push 는 !DILexicalBlockFile(file:!13457, line 1) 로 찍혀 소스 줄이 소실됐다 — 채팅 push 가 매크로/헬퍼를 거쳐 인라인된 것으로 보이나 원 소스 형태는 미확정 |  |  |
| 4 | epic.rs 671~678 의 원 소스 구조(if 문인지 match 인지)는 !dbg 가 671~678 을 하나도 안 남겨 복원 불가. IR 은 '비교 후 push' 한 덩어리만 남았다 |  |  |
| 5 | player.strategy(...) 는 이 IR 파일에 define 이 없다(다른 크레이트). 내부에서 rnd 를 어떻게 쓰는지 미확인 — 여기선 strategy.morgard_use(+0x4) 만 소비한다 |  |  |
| 6 | AbstractGameWithCache.player_champion 의 원소 타입을 distruct 가 array$<?> 로만 알려준다. IR 상 [5 x ptr] 이고 null 비교로 Option 니치를 쓰므로 Option<&엔티티류> 인 것은 확실하나 정확한 T 는 미확정 |  |  |
| 7 | v3_epic_group_line / v3_epic_formation_role 내부는 안 봄 — group_line 이 어떤 근거로 Top/Mid/Bottom 을 고르는지, formation_role 의 is_split 이 무엇으로 결정되는지는 이 명세 범위 밖 |  |  |
| 8 | `epic.rs` 655~678 — ★**대부분 확정(2026-09-11). 직전 「재료 부재로 종결, 남은 건 exe 디스어셈뿐」 판정은 과했다.** **막힌 것은 MIR 경로뿐**이었고, 「**줄 길이 산술**(rmeta_srcmap) + tcx ADT 필드명·태그 + IR 분기 유무」 조합으로 뚫렸다. ★전제도 틀렸었다: DILocation 전수 집계 결과 **`659`·`674`·`676` 은 사실 IR 이 있다**. 진짜 미해명은 `655·656·670·675·681` 이었고 **그중 4개 확정**:   L654~657 = `self.objective = Some(MainObjective::Serpen {` / `  phase: ObjectPhase::Setup,` / `  with_battle: true,` / `});`     (근거: 세 스토어가 전부 `!dbg epic.rs:654` 단일 + 사이에 분기 없음 ⟹ 655 는 `if` 가 될 수 없는 **계속행**. `MainObjective::Serpen{phase, with_battle}` 가 tcx 상 네임드 필드라 표기가 강제되고 `ObjectPhase` 태그 1 = `Setup` 이 길이를 ±0 으로 맞춘다)   L675 = `} else {` / L681 = `false`   L674/676 = `self.chats.push(Chat::Press(epic_line, 0));` / `…PressChange…` (IR `select(v3_press_chat_line.is_none(), 21, 22)`. 두 줄 길이차 6 = `len("PressChange")−len("Press")` 로 자기정합) **남은 것**: ①674/676 의 **1자 잔차**(양쪽 동일) ②`667~670` 이터레이터/클로저 체인 본문(**미탐색** — IR 은 `player_champion[team][i].is_some()` 필터 + `v3_epic_formation_role(...)` 5회 언롤까지 확인) ③`648~650`·`662~663`·`685~686` 의 한글 `//` 주석 원문(**재료 부재** — 라인주석은 rmeta 미포함). 전문 = RE\2026-09-11_IR정밀독해-exit_src16_23-region_point-battle잔여-epic줄복원.md |  |  |
| 9 | `FightLine::CommitAfterJoin`(1) 의 생산 지점 — ★**"죽은 variant" 로 닫힘(2026-09-11)**. `FightPrediction.line`(+0x38)/`line_absolute`(+0x39) 로의 store 를 `_gaibc` 전량 gep 체인 누적 스캔한 결과 **값 1 을 쓰는 코드 0건**. 유일한 진짜 생산자 `fight_model.rs:520`(m10.ll:47092/47105)의 phi 값역이 **{0,2,3}**. 후보였던 `resolve_join_stake`·`battle::update_v32` 는 **둘 다 복사만** 한다(탈락). 소비 측 switch 에는 케이스가 살아 있으나 **Hold 와 같은 블록으로 접혀** 있다. ⚠적용 범위: `_gaibc`·`_gcbc`·`_gvbc` IR 한정. 미탐색 = 미추출 rlib(`libengine_*`) · rmeta MIR · exe. 전문 = RE6-09-11_fight_model-심층부4건-join_stake-tower_dive-net_value.md |  |  |
</details>

<details><summary>`history` 정정 이력 16건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | v3_epic_group_line / v3_epic_formation_role 내부 — group_line 이 어떤 근거로 라인을 고르는지, is_split 이 무엇으로 결정되는지 | ★전부 확정. |  |
| 1 | v3_epicops_repair_need 내부 — 수리 선점의 우선순위 1/2 가 어떤 조건에서 갈리는지 | ★확정(m09.ll:64975~65551). `fn(team, cache, plan: &BigPlan) -> u8 /*0..3*/`:   n_mine/n_enemy = 생존 챔프 수   **if plan.goal() 이 BigGoal::Battle(태그 5)이 아니면**: hurt = hp*100/max < 41 인 아군 수; `if n_mine <= n_enemy && hurt > 1 { return 1 }`   healthy = hp*100/max > 29 인 아군 수; `if healthy < n_enemy && healthy < n_mine { return 2 }`   return 0  ⟹ **1/2 를 가르는 것 3가지**: ①plan.goal()==Battle 이면 **1 자체가 불가**(그 블록을 통째로 건너뜀, m09.ll:65043) ②1 = 인원 열세(≤) + HP41% 미만 2명 초과 ③2 = HP29% 초과 인원이 적팀 생존수보다도 우리 생존수보다도 적다. |  |
| 2 | v3_serpen_contest_clear_win 내부 — version 이 여기서만 쓰이므로 버전별 분기표를 복원하라 | ★확정, **단 버전별 분기표는 존재하지 않는다.** (m05.ll:53103~53408)   me = cache.player_champion[team][pos]; None 이면 false   enemies = serpen_reachable_enemies(...); 비었으면 **true**(serpen.rs:57)   allies = iter_champions(team).filter(hp*100/max > 39)   acc = player.info(+0x180 AthleteParameter).judge_accuracy()   pred = fight_model::resolve_fight(version, data, me, allies, enemies, 0u8, None, acc)   return pred.line == FightLine::Commit  (FightPrediction+0x38 == 0)  ★`version` 은 전 범위에서 **정의 줄(53103)과 resolve_fight 호출 줄(53286) 2회뿐**. 버전 게이트는 `fight_model::resolve_fight`(fight_model.rs:310) 안에 있고 **다른 함수의 담당**이다. |  |
| 3 | is_object_being_taken_by_enemy 의 5번째 인자 리터럴 true 의 의미 | ★확정 — **bool 이 아니라 `target: WavePriorityObject` 2값 C-enum 이 i1 로 전달된 것**. DWARF `m15.ll !58647`(arg:5, type !9496 = DW_TAG_enumeration_type "WavePriorityObject", DIFlagEnumClass): **Morgard=0, Serpen=1** ⟹ 리터럴 `true` = **Serpen**.  극성 교차검증: `%4==true` 분기(m15.ll:53841)의 !dbg 가 `rule_scope::serpen_exists`(objective_helpers.rs:326), false 분기가 `morgard_exists`(:317). 읽는 필드도 serpen_camp_* / jungle_runner.serpen.* 계열. |  |
| 4 | goal_data(248B)와 plan(384B) 중 실제 판정에 쓰이는 필드 | ★확정. `goal_data` 는 **`serpen.epic_enemy_tick`(+0xc0) 오직 1개**(m15.ll:53994, Serpen 분기). `epic.epic_enemy_tick`(+0x88)은 Morgard 분기라 이 호출부에선 **죽은 경로**. `plan` 은 **`BigPlan::goal()` 의 판별자 태그 하나**(== BigGoal::Battle(5) 인가, m09.ll:65038~65044). 이 함수 본문 자체는 둘 다 통과만 시킨다(gep 0건). |  |
| 5 | player.strategy(...) 내부 — rnd 를 어떻게 쓰는지(RNG 상태 전진 부작용이 있는지) | ★확정 — **RNG 전진 없음.** `_gcbc/g15.ll:130480~130505` 에 define 이 있다.   `fn PlayerState::strategy(&self, rnd: &mut StdRng, game: &dyn AbstractGame) -> Strategy { if game.is_solorank()(vt+0xe8) { self.info.solorank_strategy(PlayerState+0x4f8, 24B memcpy) } else { game.strategy(self.info.team)(vt+0x108) } }`  ★**`rnd` 파라미터에 `readnone` 이 붙어 있다**(g15.ll:130480) — LLVM 의 readnone = '이 포인터로 읽지도 쓰지도 않는다' ⟹ StdRng 상태를 전혀 안 건드린다. 인자로만 받고 버리는 잔재다.  그리고 `v3_epicops_buff_window` 전 범위에서 rnd 는 이 호출 1회에만 등장 ⟹ **이 함수 전체가 RNG 를 전진시키지 않는다**(재현 시 시드 정합성 걱정 불필요). |  |
| 6 | epic.rs 655~663 / 671~678 의 원 소스 구조 — !dbg 소실로 복원 불가 | ★★**'원리적 불가' 판정을 뒤집는다.** `!dbg` 는 소실되지 않았다 — `!DILocation → scope/inlinedAt` 사슬을 **`inlinedAt` 루트까지** 타면 줄번호가 나온다.  복원된 줄: 653(eo_serpen_punish_issues += 1) · 654(objective = Some(Serpen{Setup,true})) · **658**(chats.push(Chat::SerpenSetup(0))) · 664(strategy = player.strategy(rnd, game)) · 665(group_line = v3_epic_group_line(strategy.morgard_use, ...)) · 666(if group_line != self.v3_press_chat_line) · 667(announcer = (0..5).position(...)) · 668(player_champion[team][i].is_some()) · 669(v3_epic_formation_role(...)) · 670(.is_some_and(\|f\| !f.is_split)) · 672(if announcer == Some(player.info.position.as_index())) · 673(self.v3_press_chat_line.is_none()) · **676**(chats.push(Chat::Press/PressChange(group_line, 0))) · 679(self.v3_press_chat_line = group_line)  **남은 미복원**: 655~657 · 659~663 · 671 · 674~675 · 677~678 — 이 줄들엔 어떤 !DILocation 도 매핑되지 않는다(중괄호·else·완전 상수접힘 추정). |  |
| 7 | MorgardUseStrategy 를 누가 언제 정하는가 — divtable 은 런타임 구현체를 특정 못 한다(도구 한계) | ★★**확정 — 전황 판단이 아니라 팀 생성 시 난수다.** 조회 사슬: `PlayerState::strategy`(_gcbc/g15.ll:130480) → solorank 면 `PlayerState+0x4f8`, 아니면 `AbstractGame::strategy(team)`(vt+0x108) = `Game::strategy` (g15.ll:215305) = **`World.strategy[team]`**(`distruct World 0xb3b0` = `[Strategy; 2]`, Game+46000). 값 생성: `MorgardUseStrategy::random_from<ThreadRng>`(_gcbc/g15.ll:5094) = `match rng.gen_range(0..3) { 0 => Gather(태그5), 1 => Split14{Position::from_index(gen_range(0..5))}(태그6), 2 => Split131{p1, p2 를 p1≠p2 될 때까지 재추첨} }`. ★Split131 은 **태그 자리(하위32b)가 p1(0..4) 자체** — 니치가 판별자를 먹는다. 반환 i64 = `(payload << 32) \| tag`. 호출 사슬: `TeamColorStrategy::random`(g09.ll:155494) / `Strategy::random`(g15.ll:115365) ← **`TeamSetting::create`**(g01.ll:101423, 호출 g01.ll:106699·106716). ⟹ **`morgard_use` 는 팀(구단) 고유 성향이고 경기 중 전황으로 바뀌지 않는다.** 기본값: `World::new`(g07.ll:175103, 초기화 176608~)이 `strategy[0].morgard_use = 5(Gather)` 로 채운다. 쓰기 API: `Game::set_strategy(team, &Strategy)`(g15.ll:176782) = `world.strategy[team]` 에 24B memcpy. 부수: `Strategy::from(a,b)`(g15.ll:115357)는 **두 인자를 모두 무시하고 `Strategy::random()` 을 반환**하는 사실상 죽은 함수(IR 내 호출자 0). |  |
| 8 | switch default 가 unreachable 인 이유 — LLVM 이 morgard_use 태그 ≤7 임을 어디서 증명했는지 | ★확정 — **`!range` 때문이 아니다.** 태그 로드 지점(_gaibc/m09.ll:6968)에 `!range` 메타데이터가 **붙어 있지 않다**(그 가설은 반증됐다). `unreachable` 은 **Rust 프론트엔드의 enum 매치 완전성**이 낸 것이고, 유효 태그 범위는 DWARF 에 명시돼 있다(_gaibc/m09.ll:72843~72872): Variant0 Gather `DISCR_EXACT 5` / Variant1 Split14 `DISCR_EXACT 6` / Variant2 Split131 **`DISCR_BEGIN 0` · `DISCR_END 4`(범위 variant)** ⟹ 유효 태그 = 0..=6. |  |
| 9 | AbstractGameWithCache::iter_champions 가 훑는 슬롯 범위 — 추정 | ★확정 — `_shared.맵_좌표계.iter_champions` 참조. `player_champion[team]` **5칸 전부**, None 스킵만(그 외 필터 없음), 슬롯 0→4 오름차순. |  |
| 10 | fight_model::resolve_fight 의 버전별 분기표 — 담당 범위 밖 | ★확정. 전문 = `_shared.resolve_fight`. **버전 분기는 단 1건**(시드 파생 방식)이고, version 을 받는 나머지 3개 함수는 본문에서 아예 안 쓴다. `FightLine` 결정표와 `CommitAfterJoin` 미생산 사실도 확정. |  |
| 11 | serpen_reachable_enemies 내부 — 도달 가능 적의 정의(거리/시야 임계) 미확인 | ★확정(serpen.rs:20~39, _gaibc/m05.ll:52849~52983). 반환 = **두 개의 `Vec<&Entity>`**(sret 64B, +0x0 가시 / +0x20 비가시-도달가능). **A. 비가시 도달가능**(:23~33): 적 팀 블랙보드 슬롯 i<5 순회 — `need = distance(last_pos, camp_pos).saturating_sub(150000)`, `reach = (now - 최종목격틱) * e.move_speed`, 채택 = `hp*100/max > 49 ∧ reach >= need ∧ !is_recent_visible(e)` **B. 가시**(:35~37): `iter_champions()` 위에 `(!is_bottom_side \|\| is_near_mid_line) ∧ hp*100/max > 49 ∧ is_recent_visible(e)` `is_bottom_side(ctx,x,y) = (height - y) > x`. `is_near_mid_line`: `f = height - y`; `max(f,x) < 192001` → true / `min(f,x) < width - 192000` → `\|f-x\| < 64000` / else true. |  |
| 12 | Game::set_strategy 의 호출자 — 크레이트 바깥에서 호출된다고 추정 | ★확정 — 전문 = `_shared.set_strategy_호출자`. vtable **+0x180** 간접 호출이고, 호출 지점은 **`_gvbc`(game_view) 4곳**(일시정지 UI · 서버 워커 응답 · 훈련 UI 조합테스트 · 클라 패킷). `_gvbc` 를 안 봤던 것이 원인이다. |  |
| 13 | TeamColorStrategy::random 의 필드별 Some/None 확률 | ★**전제가 틀렸다 — 필드별 확률 차이가 없다.** 전문 = `_shared.TeamColorStrategy_random`. `n = gen_range(5..=9)` 개를 셔플로 골라 setter 를 돌리고 setter 는 무조건 Some 을 채운다 ⟹ **P(Some) = 7/12 ≈ 58.3%, 12필드 전부 동일**. ★그리고 이 경로는 `thread_rng()` — **게임 결정론 PRNG 가 아니다**(리플레이 무관). |  |
| 14 | 피호출자 시그니처 2건 / `Chat::Press`·`PressChange` 의 src_line = 682 | ★**정정 2건(검증배치 D)** ① **argpromotion 아티팩트**: `v3_epicops_repair_need(team, data.cache, plan)` 로 적었으나 tcx 정본은 **`fn(player: &PlayerState, data: &OperationData, plan: &BigPlan) -> u8`**. `v3_group_press_line(team, cache, ctx, skip)` 도 실제는 **`fn(&PlayerState, &OperationData, Option<LineType>) -> Option<LineType>`(3인자)**. LLVM 이 internal 함수의 `&Struct` 인자를 *로드하는 필드값*으로 치환한 것이다(판별 = tcx `sig` + DWARF `DILocalVariable(arg:N)`).    ➕**부수 소득**: promotion 은 *모든* 용도가 그 로드일 때만 일어나므로 — `v3_epicops_repair_need` 는 **player 에서 `info.team` 만, data 에서 `cache` 만 읽는다**(그 외 접근 0). 재구현 근거로 쓸 수 있다. ② **줄번호 오류**: `constants[21]`/`constants[22]` 의 `src_line = 682` 와 `logic` 주석 "682줄의 is_none" 은 **틀렸다**. `dloc !14971` 체인 = `option.rs:633 is_some` ← **`option.rs:682 is_none`** ← **`epic.rs:673`** ⟹ **682 는 `core/src/option.rs` 줄번호**(숫자 우연 일치)다. `Chat::Press`/`PressChange` push 는 **`epic.rs:674`/`676`**. 같은 명세의 `still_unknown` 은 674/676·673 으로 **맞게** 적혀 있어 `constants`/`logic` 만 갱신 누락 — ⚠`constants` 를 기계 소비하는 도구가 잘못 앵커링한다. 참고: `epic.rs:682` 는 **실재 줄**(`!14808`, 함수 꼬리) — "682 가 없다"가 아니라 "682 는 그 문장이 아니다". |  |
| 15 | 18 epic 헬퍼 3종의 규칙표 | ★**진리표 확보**(3차 배치D, pub). `v3_epic_formation_role` **80칸 전수 확정**(Gather→전원 Mid/false · Split14{P}→P만 Bottom/true · Split131→p1 Top/true·p2 Bottom/true). ★`v3_epic_group_line` 은 **TutorialType 의존**이고 `JungleOnly`·`First/Bottom+Split14` 에서 **None** 을 낸다 ⟹ 「group_line==None → false」 경로가 **도달 가능함이 실행 확인**됐다. 부수: `PlayerState::strategy` 실제 시그니처는 **`(&self, &mut StdRng, &dyn AbstractGame)`**(인자 3개, player.rs:1576). |  |
</details>

