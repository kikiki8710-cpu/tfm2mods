---

### `05` v50_fold_dive_episode — 진행중 다이브 에피소드를 종료 확정(take)해 통계 레코드로 접어 Vec에 push하고, 접촉 없이 끝난 건 다이브 포기 시각으로 기록

| 항목 | 값 |
|---|---|
| id | `dive_episode__v50_fold_dive_episode` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler12dive_episodeNtB4_17LegacyPlanHandler21v50_fold_dive_episode` |
| 소스 | `game-ai\src\plan_legacy\handler\dive_episode.rs:125` |
| IR | `m13.ll` 28946~29380행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_fold_dive_episode` · **in:game_ai** |
| 계층 | 플랜 핸들러 |
| exe | **없음** — 「exe 에 독립 함수가 없다(인라인·`define internal fastcc`)」인지 **「조인 실패」**인지는 이 칸만으로 못 가른다. `dllmatch.py`·`name2rva.py` 로 확인하라 |
| 라운드 | 기준 `r4` · 통과 4회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8)
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut LegacyPlanHandler(6168B) | 상태변경 대상 | 4 |
| 1 | 2 | _version | usize | DWARF !35908 에만 존재. 최적화로 인자에서 제거됨(#dbg_value(i64 poison)) — 본문에서 안 씀 | 3 |
| 2 | 3 | _tps | usize | DWARF !35909 에만 존재. 마찬가지로 poison — 본문에서 안 씀 | 3 |
| 3 | 4 | aborted | bool | IR %1. 레코드 aborted 필드로 그대로 저장되고, abort_src 기록 여부를 게이트 | 4 |
| 4 | 5 | end_reason | u8 | IR %2. 레코드 end_reason 으로 저장. 값 7 은 '포기 집계 제외' 특례 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v50_fold_dive_episode(&mut self, _version, _tps, aborted: bool, end_reason: u8)

// ── A) 접촉 없이 끝난 다이브면 '포기 시각' 갱신 (dive_episode.rs:131~140)
if self.v50_dive_ep_live != None { // self+0x570 != -1 (:131, 들여쓰기 6)
 let no_contact = live.in_range_ticks == 0 // :138 첫항, self+0x598
 && live.team_holder_ticks == 0; // :138 뒷항, self+0x5a8
 if no_contact && end_reason != 7 { // :139
 self.last_dive_abandon_tick = // :140, self+0x1480
 self.last_dive_abandon_tick.max(live.last_tick); // umax(self+0x1480, self+0x588)
 }
}
// ★A 블록은 self 를 소비하지 않는다 — 게이트가 거짓이어도 아래 B/C 는 항상 실행된다.
// ★소스 구조(11차 배치B — 10차의 「중첩 if 둘」은 **판정반전**됐다. 본문길이 = bytes−1):
//   L130 = 5자 `    {` — 들여쓰기 4 에서 블록을 여는 줄(코드 생성 0 — IR 에 126~130 줄의 명령이 없다)
//   L131 = 58자 = 들여쓰기 **6** + `if let Some(live) = self.v50_dive_ep_live.as_ref() {`(52) ±0
//   L132~137 = 한국어 주석 6줄(코드 없음 · 132~137 에 !dbg 달린 명령 0건)
//   L138 = 81자 = 8 + `let no_contact = live.in_range_ticks == 0 && live.team_holder_ticks == 0;`(73) ±0
//   L139 = 42 = 8 + `if no_contact && end_reason != 7 {`(34) ±0 → L140 = 88 = 10 + 78 ±0
//   L141/142/143 = 9/7/5자 = `}` 들여쓰기 8/6/4 ⟹ 열린 블록 = L130(4)·L131(6)·L139(8) 셋으로 완전히 설명
//   L156 = 54 = 4 + `if let Some(done) = self.v50_dive_ep_live.take() {`(50) ±0 ⟹ fn 본문 들여쓰기 4 를 독립 고정
//   ⟹ `in_range_ticks == 0` 은 독립 중첩 if 가 **아니라** `no_contact` 의 첫 항이고,
//     단축평가로 그 첫 항이 `if let` 게이트와 합쳐져 m13.ll:28992 `select i1 %7, i1 %10, i1 false` 가 됐다.
//     의미는 동일하므로 재구현은 이대로 두어도 된다.
//   ⚠L130 이 맨몸 `{`(스코프 블록)인지 다른 구문의 여는 줄인지는 **미탐색** — 5자로 들여쓰기 4 의 블록을 여는 ASCII 후보는 `    {` 뿐이다.
//   ⚠rustc 는 변수가 없는 scope 에 DILexicalBlock 을 안 만든다(`!35915`(line 138)의 부모가 `!35913`(line 131)인 것은 양 독해에서 모두 성립) ⟹ DWARF 스코프 사슬은 판별력이 없고, 판정 근거는 줄 길이 산술이다.

// ── B) 현재 BigPlan 이름 → end_plan 코드 (dive_episode.rs:144~153)
let n: String = self.plan.get_name(); // :145, self+0x5e8
let end_plan: u8 =
 if n.starts_with("PassiveLine") { 1 } // :146
 else if n.starts_with("PassiveJungle") { 2 } // :147
 else if n.starts_with("LineGank") { 3 } // :148
 else if n.contains("Epic") { 4 } // :149
 else if n.contains("Serpen") { 5 } // :150
 else if n.starts_with("ActiveRecall") { 6 } // :151
 else if n.starts_with("Recall") { 6 } // :151 (같은 값)
 else if n.starts_with("Battle") { 7 } // :152
 else if n.contains("Nexus") { 8 } // :153
 else { 9 }; // :153
drop(n);

// ── C) 진행중 에피소드를 take 해서 통계 레코드로 접기 (dive_episode.rs:156~169)
let taken = self.v50_dive_ep_live.take(); // 값 복사 후 self+0x570 = -1 을 무조건 store
if let Some(done) = taken { // 원래 판별자 != -1 일 때만
 self.v50_dive_episodes.push(V50DiveEpisode { // self+0x888 Vec, elem 104B
 max_catch_break: done.max_catch_break, // +0x00 <- live+0x58 (self+0x5c8)
 uncatch_total: done.uncatch_total, // +0x08 <- live+0x60 (self+0x5d0)
 start_tick: done.start_tick, // +0x10 <- live+0x10 (self+0x580)
 end_tick: done.last_tick, // +0x18 <- live+0x18 (self+0x588)
 ep_ticks: done.ep_ticks, // +0x20 <- self+0x590
 in_range_ticks: done.in_range_ticks, // +0x28 <- self+0x598
 holder_ticks: done.holder_ticks, // +0x30 <- self+0x5a0
 team_holder_ticks: done.team_holder_ticks, // +0x38 <- self+0x5a8
 minion_cover_ticks:done.minion_cover_ticks, // +0x40 <- self+0x5b0
 soaked_hp: done.soaked_hp, // +0x48 <- self+0x5b8
 start_race_adv: done.start_race_adv, // +0x50 (i32) <- self+0x5dc
 target_pos: done.target_pos, // +0x54 (i32 Position) <- self+0x5d8
 abort_src: if aborted { self.v50_dive_ep_abort_src } else { 0 }, // +0x58, :167, self+0x1811
 start_model: done.start_model, // +0x59 <- self+0x5e2
 start_na: done.start_na, // +0x5a <- self+0x5e3
 start_ne: done.start_ne, // +0x5b <- self+0x5e4
 start_tgt_hp: done.start_tgt_hp, // +0x5c <- self+0x5e5
 end_reason, // +0x5d <- 인자 %2 그대로
 end_plan, // +0x5e <- B 의 결과
 tower: done.tower, // +0x5f <- self+0x5e1
 start_in_range: done.start_in_range & 1, // +0x60 <- self+0x5e0
 aborted, // +0x61 <- 인자 %1 (zext i1->i8)
 });
 // push 구현: len(self+0x898)==cap(self+0x888) 이면 RawVec::grow_one,
 // ptr(self+0x890) + len*104 에 위 22필드 store, len += 1
}
// live 의 prev_holder_hp 페이로드(self+0x578)와 gap_ticks(self+0x5c0)는 읽히지 않는다(#dbg_value poison).
return; // :170

// ★★호출 맥락 — **B 블록이 읽는 `self.plan` 은 이번 틱에 update 가 재플래닝한 뒤의 plan 이다.**
// ( 실행 확인) 이 함수는 `LegacyPlanHandler::update`(pub, m13.ll:14467) 안에서
// `v50_track_dive_episode`(call 0건 = 전량 인라인)를 통해 불린다(invoke m13.ll:23653·23856).
// 실측(B6_o2.tsv #B, 16/16): 호출 전에 `plan = ForcePassive` 를 넣어도 update 가 PassiveLine 으로
// 갈아 끼운 뒤 fold 가 돌아 end_plan 이 **1**(PassiveLine)로 나온다. 주입한 plan 이 아니다.
// ⟹ 재구현에서 fold 를 재플래닝 **앞**에 두면 end_plan 분포가 통째로 달라진다.
```

**`mem` 메모리 접근 30건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LegacyPlanHandler | 0x570 | v50_dive_ep_live (Option<V50DiveEpLive> 판별자 = prev_holder_hp 태그 슬롯) | r | -1 = None. !range !14067 = {-1,0,1}. 진입 게이트와 take 양쪽에서 읽음 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 1 | LegacyPlanHandler | 0x580 | v50_dive_ep_live.start_tick | r | = V50DiveEpLive+0x10 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 2 | LegacyPlanHandler | 0x588 | v50_dive_ep_live.last_tick | r | = +0x18. 포기시각 max() 입력이자 레코드 end_tick · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 3 | LegacyPlanHandler | 0x590 | v50_dive_ep_live.ep_ticks | r | = +0x20 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 4 | LegacyPlanHandler | 0x598 | v50_dive_ep_live.in_range_ticks | r | = +0x28. ==0 이어야 포기집계 블록으로 진입 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 5 | LegacyPlanHandler | 0x5a0 | v50_dive_ep_live.holder_ticks | r | = +0x30 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 6 | LegacyPlanHandler | 0x5a8 | v50_dive_ep_live.team_holder_ticks | r | = +0x38. ==0 이면 no_contact(dive_episode.rs:138) · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 7 | LegacyPlanHandler | 0x5b0 | v50_dive_ep_live.minion_cover_ticks | r | = +0x40 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 8 | LegacyPlanHandler | 0x5b8 | v50_dive_ep_live.soaked_hp | r | = +0x48 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 9 | LegacyPlanHandler | 0x5c8 | v50_dive_ep_live.max_catch_break | r | = +0x58. 레코드의 첫 필드(+0x0)로 감 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 10 | LegacyPlanHandler | 0x5d0 | v50_dive_ep_live.uncatch_total | r | = +0x60 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 11 | LegacyPlanHandler | 0x5d8 | v50_dive_ep_live.target_pos | r | = +0x68, i32 Position · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 12 | LegacyPlanHandler | 0x5dc | v50_dive_ep_live.start_race_adv | r | = +0x6c, i32 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 13 | LegacyPlanHandler | 0x5e0 | v50_dive_ep_live.start_in_range | r | = +0x70, bool. 저장 시 &1 로 정규화 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 14 | LegacyPlanHandler | 0x5e1 | v50_dive_ep_live.tower | r | = +0x71, TowerType · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 15 | LegacyPlanHandler | 0x5e2 | v50_dive_ep_live.start_model | r | = +0x72 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 16 | LegacyPlanHandler | 0x5e3 | v50_dive_ep_live.start_na | r | = +0x73 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 17 | LegacyPlanHandler | 0x5e4 | v50_dive_ep_live.start_ne | r | = +0x74 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 18 | LegacyPlanHandler | 0x5e5 | v50_dive_ep_live.start_tgt_hp | r | = +0x75 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 19 | LegacyPlanHandler | 0x5e8 | plan (BigPlan, 384B) | r | BigPlan::get_name 인자. 이름 문자열로 end_plan 코드를 분류(dive_episode.rs:145~153) · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 | OK |  |
| 20 | LegacyPlanHandler | 0x1480 | last_dive_abandon_tick | r | max() 의 좌변(self). 읽고 다시 씀 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 | OK |  |
| 21 | LegacyPlanHandler | 0x1811 | v50_dive_ep_abort_src | r | aborted==true 일 때만 레코드 abort_src 로 복사(dive_episode.rs:167) · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 22 | LegacyPlanHandler | 0x888 | v50_dive_episodes.cap (RawVec cap) | r | len==cap 이면 grow_one · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 | OK |  |
| 23 | LegacyPlanHandler | 0x890 | v50_dive_episodes.ptr | r | push 대상 버퍼 시작 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 24 | LegacyPlanHandler | 0x898 | v50_dive_episodes.len | r | push 인덱스 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK |  |
| 25 | LegacyPlanHandler | 0x1480 | last_dive_abandon_tick | w | dive_episode.rs:140. 조건부 — in_range_ticks==0 && team_holder_ticks==0 && end_reason!=7 일 때만. llvm.umax.i64 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK | max(last_dive_abandon_tick, live.last_tick) |
| 26 | LegacyPlanHandler | 0x570 | v50_dive_ep_live 판별자 | w | dive_episode.rs:156 의 Option::take. 무조건 실행(성공/실패 무관) — 진행중 에피소드가 항상 소멸한다 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK | -1 (None) |
| 27 | V50DiveEpisode(힙 원소) | 0x0 | v50_dive_episodes[len] (V50DiveEpisode 104B) | w | dive_episode.rs:157~. 필드 대응은 logic 참조. ★쓰기 대상은 `LegacyPlanHandler+0x890` **자체가 아니라** 거기서 읽은 힙 포인터 + len×104 다. `+0x888/+0x890` 에 store 가 생기는 경우는 `RawVec::grow_one` 이 도는 때뿐이고 그건 mem[29] 로 따로 실려 있다 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | 오귀속(사전은 다른 필드를 준다) | live 스냅샷 + end_reason/end_plan/aborted/abort_src |
| 28 | LegacyPlanHandler | 0x898 | v50_dive_episodes.len | w | push 완료 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK | len + 1 |
| 29 | LegacyPlanHandler | 0x888 | v50_dive_episodes.cap(+0x888) · .ptr(+0x890) | w | len==cap 일 때만 간접적으로 갱신(grow_one 이 cap+ptr 를 다시 씀) · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 | OK | RawVec::grow_one 결과 |

**`consts` 상수 20건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 131 | 센티널 | Option<V50DiveEpLive> 의 None 니치값. 진입 게이트(icmp ne -1)와 take 의 store 양쪽에 등장. DISCR_EXACT=-1 이 None, 나머지 전부 Some · 오라클 실행 확증( 오라클 실행 확증: `LegacyPlanHandler+0x570` 초기값 −1 = None, fold 후 −1 로 되돌아감(take 무조건 실행) (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| 1 | 7 | 139 | 태그 | end_reason 코드(판별자) 7 = 다이브 포기시각(last_dive_abandon_tick) 집계에서 제외되는 종료사유. 이 값일 땐 no_contact 여도 갱신 안 함 | 4 |
| 2 | 1 | 146 | 태그 | end_plan 코드(판별자) 1 = 플랜 이름이 "PassiveLine" 으로 시작. (같은 리터럴 1 이 start_in_range 저장 시 `and i8 %103, 1` 불리언 정규화 마스크로도 쓰임) ★src_line=146 을 IR 로 못 박았다(9차 배치A): 1 은 m13.ll:29154 `%74 = phi i8 [ %72, %71 ], [ 1, %33 ], …` 의 인입이라 그 자리에 `!dbg` 가 없고, 인입 블록 `%33` 의 종결자 `br i1 %18, label %73, label %34, !dbg !35929` 이 `!DILocation(line: 146)` 이다 · 오라클 실행 확증( 오라클 실행 확증: end_plan=1 (plan 이름 "PassiveLine") #A·#B·#G 26케이스 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| 3 | 2 | 147 | 태그 | end_plan 코드(판별자) 2 = 플랜 이름이 "PassiveJungle" 로 시작 · 오라클 실행 확증( 오라클 실행 확증: end_plan=2 ("PassiveJungle") #G 4케이스 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| 4 | 3 | 148 | 태그 | end_plan 코드(판별자) 3 = 플랜 이름이 "LineGank" 로 시작 | 4 |
| 5 | 4 | 149 | 태그 | end_plan 코드(판별자) 4 = 플랜 이름에 "Epic" 포함 | 4 |
| 6 | 5 | 150 | 태그 | end_plan 코드(판별자) 5 = 플랜 이름에 "Serpen" 포함 | 4 |
| 7 | 6 | 151 | 태그 | end_plan 코드(판별자) 6 = 플랜 이름이 "ActiveRecall" 또는 "Recall" 로 시작(두 술어가 같은 phi 값으로 합류) | 4 |
| 8 | 7 | 152 | 태그 | end_plan 코드(판별자) 7 = 플랜 이름이 "Battle" 로 시작. (end_reason 의 7 과 값만 같고 의미는 무관) ★src_line=152 를 IR 로 못 박았다(9차 배치A): 7 은 m13.ll:29154 phi 의 인입 `[ 7, %66 ]` 이고 블록 `%66` 의 종결자 `br i1 %65, label %73, label %67, !dbg !35963` 이 `!DILocation(line: 152)` 이다. 본문에 남은 리터럴 7 (`%23 = icmp eq i8 %2, 7`, m13.ll:29017 → L139)은 **end_reason 쪽**이라 무관 · 오라클 실행 확증( 오라클 실행 확증: end_plan=7 ("Battle …") #B2 tag9 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| 9 | 8 | 153 | 태그 | end_plan 코드(판별자) 8 = 플랜 이름에 "Nexus" 포함 · 오라클 실행 확증( 오라클 실행 확증: end_plan=8 ("AttackNexus"/"DefenseNexus") #B2 tag16·17 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| 10 | 9 | 153 | 태그 | end_plan 코드(판별자) 9 = 위 어디에도 안 걸림(그 외 전부) · 오라클 실행 확증( 오라클 실행 확증: end_plan=9 ("DeathMatchBattle …"/"SinglePlanBattle …") #B tag0·5 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| 11 | 11 | 146 | 길이 | 패턴 "PassiveLine" 의 바이트 길이(starts_with 인자) · 오라클 실행 확증( 오라클 실행 확증: "PassiveLine"(11자) 접두 일치 경로가 실제로 1 을 낸다 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| 12 | 13 | 147 | 길이 | 패턴 "PassiveJungle" 의 길이 · 오라클 실행 확증( 오라클 실행 확증: "PassiveJungle"(13자) 접두 일치 경로가 실제로 2 를 낸다 #G (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 |
| 13 | 12 | 151 | 길이 | 패턴 "ActiveRecall" 의 길이 | 4 |
| 14 | 8 | 148 | 산출값 | 패턴 "LineGank" 의 바이트 길이(starts_with 인자). ★10차 배치B 신설 — 표가 11(PassiveLine)·13(PassiveJungle)·12(ActiveRecall) 셋만 싣고 나머지 6개 패턴 길이를 빠뜨리고 있었다 | 4 |
| 15 | 4 | 149 | 산출값 | 패턴 "Epic" 의 바이트 길이(str::Pattern::is_contained_in 인자 — starts_with 가 아니라 contains 다) | 4 |
| 16 | 6 | 150 | 산출값 | 패턴 "Serpen" 의 바이트 길이(is_contained_in 인자) | 4 |
| 17 | 6 | 151 | 산출값 | 패턴 "Recall" 의 바이트 길이(starts_with 인자). ActiveRecall(12)과 **별개의 두 번째 술어**이고 같은 phi 값 6 으로 합류한다 | 4 |
| 18 | 6 | 152 | 산출값 | 패턴 "Battle" 의 바이트 길이(starts_with 인자) | 4 |
| 19 | 5 | 153 | 산출값 | 패턴 "Nexus" 의 바이트 길이(is_contained_in 인자). 이 술어 결과가 select(8,9) 로 마지막 두 코드를 가른다 | 4 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 포기 집계 제외 종료사유 | dive_episode.rs:139 | 7 | end_reason==7 이면 last_dive_abandon_tick 을 안 올린다. 이 값을 바꾸면(또는 조건을 제거하면) 해당 사유의 다이브도 '포기'로 집계돼, last_dive_abandon_tick 을 읽는 쪽(다이브 재시도 쿨다운류)이 더 오래 억제된다 | 4 | 기존 |
| 1 | 포기 판정 게이트 — 사거리 진입 틱 | dive_episode.rs:138 (`let no_contact` 의 **첫 항**) — ★정정(10차 배치B, 판정반전): ~~131 (확정 — 132~137 은 한국어 주석 6줄이라 코드가 없다)~~ 는 반증됐다. ①`rmeta_srcmap game_ai dive_episode.rs`: L125=112B 가 `pub(crate) fn v50_fold_dive_episode(&mut self, _version: usize, _tps: usize, aborted: bool, end_reason: u8) {`(들여쓰기 2 + 109자 = 111자, 종결자 1B 포함 112) 와 ±0 ⟹ 들여쓰기 단위 2 · fn 본문 4 확정. ②L131=59B(58자)는 `if let Some(live) = self.v50_dive_ep_live.as_ref() {` 이지만 들여쓰기가 **4 가 아니라 6** 이다(4+52=56 ≠ 58 · 6+52=58 ±0 — L156=54=4+50 ±0 이 fn 본문 들여쓰기 4 를 독립 고정하므로 L131 은 한 단 깊다). 그리고 `&& live.in_range_ticks == 0` 을 붙이면 공백을 다 지워도 67자라 **물리적으로 못 들어간다**(`self.v50_dive_ep_live` 21자·`in_range_ticks` 14자는 tcx 로 고정). ③L139=43B(42자)=들여쓰기 **8** + `if no_contact && end_reason != 7 {`(34자) ±0, L141/142/143=10/8/6B = 닫는 중괄호 3개(들여쓰기 8/6/4) ⟹ 열린 블록 세 개는 L130(4)·L131(6)·L139(8) 로 전부 설명된다 = 132~137 에는 블록을 여는 코드가 없다. ④L138=81자 = 들여쓰기 8 + `let no_contact = live.in_range_ticks == 0 && live.team_holder_ticks == 0;`(73) ±0 ⟹ **`in_range_ticks == 0` 은 `no_contact` 의 첫 항**이다. 값 0 과 의미는 그대로(단축평가 AND 라 재구현 동작 동일) | 0 | live.in_range_ticks==0(한 번도 사거리에 못 들어감)일 때만 포기로 본다. >0 을 허용하도록 풀면 '붙었다가 물러난' 다이브까지 포기 시각이 갱신돼 다이브 시도가 더 보수적으로 된다 · 오라클 실행 확증( 오라클 실행 확증: #C 경계 — in_range_ticks 0 → last_dive_abandon_tick 갱신 / 1 → 갱신 없음 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 | 기존 |
| 2 | 포기 판정 게이트 — 접촉 여부(no_contact) | dive_episode.rs:138 | 0 | live.team_holder_ticks==0(아군이 타겟을 한 번도 붙잡지 못함)일 때만 포기. 임계를 0 대신 N 으로 올리면 잠깐 붙은 다이브도 포기로 기록된다 · 오라클 실행 확증( 오라클 실행 확증: #C 경계 — team_holder_ticks 0 → 갱신 / 1 → 갱신 없음 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 | 기존 |
| 3 | 종료 플랜 분류표(end_plan) | dive_episode.rs:146~153 | PassiveLine/PassiveJungle/LineGank/Epic/Serpen/ActiveRecall\|Recall/Battle/Nexus/기타 → 1..9 | 판정 순서가 곧 우선순위다. 예를 들어 이름에 "Epic"과 "Nexus"가 다 들어가면 4 가 이긴다. 순서를 바꾸거나 문자열을 바꾸면 에피소드 통계의 end_plan 분포가 통째로 바뀐다(통계 소비 측의 학습·튜닝 입력이 달라짐) · 오라클 실행 확증( 오라클 실행 확증: end_plan 분류 사슬 #B 16/16 + #G 20/20 MATCH(독립 재구현 대조) (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 | 기존 |
| 4 | abort_src 기록 게이트 | dive_episode.rs:167 | aborted ? self.v50_dive_ep_abort_src : 0 | 정상 종료(aborted=false)면 abort_src 가 무조건 0 으로 지워진다. 게이트를 없애면 직전 중단 사유가 정상 종료 레코드에도 남아 통계가 오염된다 · 오라클 실행 확증( 오라클 실행 확증(범위 = aborted=false 가지만): 주입 abort_src 0/0x5A/0xFF 3종 전부 레코드 abort_src=0 으로 지워짐 #F. aborted=true 가지는 미도달 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 | 기존 |
| 5 | 다이브 포기 집계 게이트 | m13.ll:29017 — end_reason != 7 && live.team_holder_ticks == 0 && live.in_range_ticks == 0 | end_reason != 7 && live.team_holder_ticks == 0 && live.in_range_ticks == 0  (세 조건 AND 게이트) | end_reason 판정 7 을 넓히면 'last_dive_abandon_tick(LegacyPlanHandler+0x1480)' 페널티 집계를 통째로 죽일 수 있다 · 오라클 실행 확증( 오라클 실행 확증: #C 4조합 — 두 게이트가 AND 로 걸린다 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 | 신규 |
| 6 | end_plan 분류가 문자열 의존 | BigPlan::get_name (m02.ll:7050) | BigPlan::get_name() 의 반환 문자열 | ★get_name 문자열 하나만 바꿔도 end_plan 분류가 바뀐다. 진단·집계 전용이지만 로그 분석 시 함정 · 오라클 실행 확증( 오라클 실행 확증: `update` 가 plan 을 바꾸자 end_plan 이 **바뀐 이름**을 따라갔다(#B 16/16) ⟹ 분류가 문자열 의존임이 실행으로 확인 (B6_o2.tsv — 05 를 상위 pub 래퍼 `LegacyPlanHandler::update` 로 최초 실행)) | 2 | 신규 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | contains | game_core::RectU64::contains | pub | fn(&game_core::RectU64, u64, u64) -> bool | game-core\src\setting.rs:40 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 3개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 1 | contains | game_core::RectI64::contains | pub | fn(&game_core::RectI64, i64, i64) -> bool | game-core\src\setting.rs:55 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 3개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 2 | contains | game_core::setting::champion::nightmare::NightmareWellRect::contains | in:game_core::setting::champion::nightmare | fn(game_core::setting::champion::nightmare::NightmareWellRect, u64, u64) -> bool | game-core\src\setting\champion\nightmare.rs:28 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 3개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 3 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 4 | get_name | game_ai::plan_legacy::types::BigPlan::get_name | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> std::string::String | game-ai\src\plan_legacy\types.rs:44 | False | False | 3 | IR 호출 심볼 일치 |
| 5 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 6개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 6 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 6개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 7 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 6개 중 상위 3개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
| 8 | v50_fold_dive_episode | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_fold_dive_episode | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8) | game-ai\src\plan_legacy\handler\dive_episode.rs:125 | False | False | 4 | ⚠미확정 — 같은 leaf 후보 1개 중 상위 1개만 실음(IR 범위에 이 호출이 없다 = `logic` 산문에서 긁혔거나 접힘) |
</details>

⚠**미매칭 6개**: `gap_ticks`, `grow_one`, `is_contained_in`, `llvm.umax.i64`, `starts_with`, `take`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m13.ll:23653, m13.ll:23856) · **형제 41개** (LegacyPlanHandler)

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

**`open` 1건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | RawVec::grow_one 내부(재할당 정책) — ★**해소(11차 배치B)**: `define` 이 `_gaibc/m03.ll:142039` 에 있다(`RawVec<V50DiveEpisode>::grow_one`). 본문 = `%3 = load i64, ptr %0`(cap) → `%4 = shl nuw i64 %3, 1` → `%5 = tail call i64 @llvm.umax.i64(i64 %4, i64 4)` → `RawVecInner::finish_grow` ⟹ **cap' = max(2×cap, 4)** (4 = `MIN_NON_ZERO_CAP`, 원소 104B ≤ 1024 이므로). len==cap 일 때만 불리므로 2×cap ≥ cap+1 은 항상 성립하고, 할당 실패는 `raw_vec::handle_error` 로 중단된다. ⟹ 재구현에서 표준 `Vec::push` 를 그대로 써도 동일하다 | 4 |  |

<details><summary>`closed` 7건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | in_range_ticks==0 (self+0x598) 체크의 정확한 소스 줄 — 이 icmp 에는 !dbg 가 안 붙고 뒤의 select 만 !35976(option.rs:742 **as_ref**, inlinedAt dive_episode.rs:131 — ~~is_some~~ 는 오기)을 물고 있다. 조건이 존재한다는 사실은 확실하나 131 줄 자체인지 132~137 어딘가의 중첩 if 인지 확정 못 함(2회 시도 후 중단). ★10차 배치B — **131 이 아님이 확정**되고 132~137 로 좁혀졌다(닫히진 않음): 줄길이 산술로 ⓐ들여쓰기 단위 2(L125 사인 ±0) ⓑL131=58자라 두 조건을 같이 담을 수 없음(최소 67자) ⓒL139 의 if 가 들여쓰기 **8**(42자 ±0)이고 L141/142/143 이 닫는 중괄호 3개(8/6/4) ⟹ 131 과 139 사이에 들여쓰기 6 블록이 하나 더 있다 ⟹ **in_range 검사는 독립 중첩 if**. ★★**11차 배치B 판정반전 · 해소** — 10차의 「독립 중첩 if, 132~137 중 한 줄」이 틀렸다. 10차는 L131 을 들여쓰기 4 로 가정했는데 그럼 56자여야 하고 실측은 58자다. 들여쓰기 **6** 이면 ±0 이고, L156=54(=4+`if let Some(done) = self.v50_dive_ep_live.take() {`50) ±0 이 fn 본문 들여쓰기를 4 로 독립 고정하므로 L131 은 한 단 깊다(L130=5자가 들여쓰기 4 의 블록을 열고 있다). 그러면 L138=81자 = 8 + `let no_contact = live.in_range_ticks == 0 && live.team_holder_ticks == 0;`(73) ±0 으로 떨어지고, L141/142/143(9/7/5자 = `}` 들여쓰기 8/6/4)이 L130(4)·L131(6)·L139(8) 으로 전수 설명된다. ⟹ **132~137 은 주석 6줄이고 `in_range_ticks == 0` 은 L138 `no_contact` 의 첫 항**이다(1차가 「132~137 은 주석 6줄」이라 한 관측 자체는 옳았고, 「따라서 조건이 131 에 있다」는 추론이 틀렸던 것이다). 10차 판이 틀린 자리는 두 곳 — L131 에서 2자, L138 에서 29자. 남은 미탐색 = **L130(5자, 들여쓰기 4 의 블록 여는 줄)의 정체**뿐이다(맨몸 `    {` 가 유일한 ASCII 후보 · 그 줄은 IR 에 명령을 하나도 안 남긴다) | 1차 배치B: L131 확정(132~137 은 한국어 주석 6줄) |
| 1 | end_reason 코드 체계 — u8 원시값이고 DWARF 에 열거형이 없다(dienum.py 조회 실패). 값 7 이 무엇을 뜻하는지는 이 함수 밖(호출자)에서만 알 수 있음 | 3차 배치B: 호출자 IR(v50_track_dive_episode)에서 0~8 의미 전량 확정 |
| 2 | end_plan 코드 1..9 도 마찬가지로 V50DiveEpisode.end_plan: u8 원시값 — 명명된 enum 이 없어 의미는 위 문자열 매칭으로만 역추정 | 3차 배치B: 문자열 매칭이 원문 + 오라클 get_name 8종 |
| 3 | _version(p2)/_tps(p3) — DWARF 에는 인자로 있으나 IR define 은 3인자뿐이고 #dbg_value(i64 poison) 이라 완전히 미사용. 원본 소스에 어떤 게이트가 있었는지는 알 수 없음 | 3차 배치B: 호출자 IR(v50_track_dive_episode)에서 0~8 의미 전량 확정 |
| 4 | live.prev_holder_hp 페이로드(self+0x578)와 live.gap_ticks(self+0x5c0)는 이 함수에서 읽히지도 레코드로 복사되지도 않는다(둘 다 fragment poison) — 접는 과정에서 버려지는 이유는 이 함수만으로는 불명 | 3차 배치B: tcx 에 그 필드가 아예 없다 |
| 5 | BigPlan::get_name 내부(어떤 variant 가 어떤 이름 문자열을 내는지)는 안 봄 — 담당 범위 밖 | 1차 배치B resolved[1] 전수 표 + 2차 오라클 8종 실행 |
| 6 | self+0x1480 last_dive_abandon_tick 을 읽어 실제로 무엇을 억제하는지는 이 함수 밖 — 소비 측 미확인 | 3차 배치B: 소비 게이트 8곳 전부 tick > 값+1+4×tps |
</details>

<details><summary>`history` 정정 이력 9건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 표 | 7이_제외되는_이유 | 진입_게이트 | 부수 | 판정_사슬 | 함정_2건 |
|---|---|---|---|---|---|---|---|---|
| 0 | end_reason: u8 코드 체계 — DWARF 에 열거형이 없어 값 7 의 의미만 관측 | ★0~8 전부 확정. 생산처 = v50_track_dive_episode::closure$0 (dive_episode.rs 37~50행, m13.ll:100726)의 `let ... else { return N }` 사슬. phi = m13.ll:23531 / 23538. | [{"code": 0, "src": 66, "meaning": "이전 에피소드를 타워가 바뀌어 강제 마감(live.tower != 현재 dive_tower)", "ir": "m13.ll:23653"}, {"code": 8, "src": 38, "meaning": "내 챔피언 엔티티를 못 얻음", "ir": "m13.ll:23240"}, {"code": 1, "src": 39, "meaning": "현재 BigPlan 이 Battle(tag 9) 이 아님", "ir": "m13.ll:23251"}, {"code": 2, "src": 40, "meaning": "BattlePlan.with_dive(+0xf6) == false", "ir": "m13.ll:23258"}, {"code": 3, "src": 41, "meaning": "BattlePlan.dive_tower(+0xfe) == None(0xff)", "ir": "m13.ll:23264"}, {"code": 4, "src": 42, "meaning": "BattleSubPlanGoal::focus() == None — 즉 sub_goal 이 RunAway(4) 또는 End(7)", "ir": "m13.ll:23272 switch(case 4,7)"}, {"code": 5, "src": 43, "meaning": "get_entity_by_id(focus)(vt+0x1f0) == None", "ir": "m13.ll:23298"}, {"code": 6, "src": "44~45", "meaning": "타깃이 적 팀 Player 챔피언이 아님 또는 그 챔피언의 PlayerState 를 못 찾음", "ir": "m13.ll:23310/23322/23329"}, {"code": 7, "src": "47·50", "meaning": "다이브 대상 타워가 없음(iter_towers_without_nexus().min_by_key()==None) 또는 그 타워에 attack_effect 가 없음", "ir": "m13.ll:23350/23360"}] | ★확정 — m13.ll:29017 의 게이트 `if end_reason == 7 \|\| live.team_holder_ticks != 0 { skip }`. 즉 '애초에 다이브할 타워 자체가 없던 케이스' = 다이브 시도가 아니므로 포기 집계에서 뺀다. 통과 시 last_dive_abandon_tick(+0x1480) = max(prev, live.end_tick). | live.is_some() && live.in_range_ticks == 0 (m13.ll:28992~28995) | 두 번째 호출의 aborted 인자 = plan.tag==9 && BattlePlan.dive_abandoned(+0xfa) (m13.ll:23846). dive_abort_src(+0x103) → self+0x1811 (m13.ll:23855). |  |  |
| 1 | end_plan: u8 코드 1~9 — 플랜 이름 문자열 매칭으로 역추정만 | ★확정. 문자열 매칭이 곧 원본 로직이라 '역추정'이 아니라 원문이다. BigPlan::get_name(m02.ll:7050) 스위치(m02.ll:7282)로 variant→문자열 전수 확정. 스위치 인덱스 = `select(tag>1, tag-2, 4)` 로 tag≤1 이 DeathMatchBattle 로 가는 것이 dienum BigPlan 의 니치와 정확히 일치(교차검증 통과). | [{"variant": "ForcePassive(2)", "name": "ForcePasive (게임 원문 오타)", "end_plan": 9}, {"variant": "PassiveLine(3)", "name": "PassiveLine", "end_plan": 1}, {"variant": "SinglePlanLine(4)", "name": "SinglePlanLine", "end_plan": 9}, {"variant": "SinglePlanBattle(5)", "name": "SinglePlanBattle goal: …", "end_plan": 9}, {"variant": "DeathMatchBattle(니치 0/1)", "name": "DeathMatchBattle goal: …", "end_plan": 9}, {"variant": "PassiveJungle(7)", "name": "PassiveJungle", "end_plan": 2}, {"variant": "ActiveRecall(8)", "name": "ActiveRecall", "end_plan": 6}, {"variant": "Battle(9)", "name": "Battle support: …", "end_plan": 7}, {"variant": "LineGanker(10)", "name": "LineGanker", "end_plan": 3}, {"variant": "LineGankCover(11)", "name": "LineGankCover", "end_plan": 3}, {"variant": "EpicHuntAndPoke(12)", "name": "EpicHuntAndPoke", "end_plan": 4}, {"variant": "EpicHuntAndBattle(13)", "name": "EpicHuntAndBattle", "end_plan": 4}, {"variant": "SerpenHuntAndPoke(14)", "name": "SerpenHuntAndPoke", "end_plan": 5}, {"variant": "SerpenHuntAndBattle(15)", "name": "SerpenHuntAndBattle", "end_plan": 5}, {"variant": "AttackNexus(16)", "name": "AttackNexus", "end_plan": 8}, {"variant": "DefenseNexus(17)", "name": "DefenseNexus", "end_plan": 8}] |  |  |  | starts_with("PassiveLine")→1 / starts_with("PassiveJungle")→2 / starts_with("LineGank")→3 / contains("Epic")→4 / contains("Serpen")→5 / starts_with("ActiveRecall")→6 / starts_with("Recall")→6 / starts_with("Battle")→7 / contains("Nexus")?8:9  (m13.ll:28998→29154 phi, 리터럴 m13.ll:55~102) | ["★starts_with(\"Recall\")→6 분기는 **도달 불가능한 죽은 가지**다 — get_name 이 \"Recall\" 로 시작하는 이름을 내는 variant 가 없다(ActiveRecall 은 앞 분기가 먼저 먹는다).", "★\"SinglePlanBattle …\"·\"DeathMatchBattle …\" 은 contains 가 아니라 starts_with(\"Battle\") 이라 **7이 아니라 9** 로 떨어진다."] |
| 2 | end_reason 6 이 소스상 44행·45행 두 else 가지에 같은 값을 쓰는지, 한 개의 and_then 체인인지 | ★**확정 — 두 개의 별도 실패 가지이고 둘 다 같은 값 6 을 쓴다.** "하나의 and_then 체인"이 아니다.   6 을 쓰는 사이트 3개: `%3397`(eq:1127 ← inlined at closure$0:44, `==` 태그 비교 단계) · `%3401`(closure$0:44, `==` 값 비교 단계) · `%3405`(closure$0:45, `player_by_champion_id(...) == null`).   44행이 IR 블록 2개인 것은 `==` 가 태그/값 2단계로 쪼개진 것일 뿐 소스 가지가 둘인 게 아니다. ⟹ **연속된 guard 두 개가 코드를 공유**한다. ★부수: `end_reason` 표가 rmeta 주석에는 **없다**(전량 grep 0건) — 우리 IR 표가 유일 출처다. |  |  |  |  |  |  |
| 3 | end_reason 0~8 표의 제3 독립 출처 | ★부분 확보(2026-09-11 2차배치B). ①**카디널리티 독립 확인**: `game_core::GankStatistics.dive_ep_end_reason` 이 **`[usize; 9]`** (game_view DWARF `_gvbc/v00.ll:!16729` size 576bit + `!16731` DISubrange count 9) ⟹ 코드 개수가 정확히 9(0~8). ②`V50DiveEpisode` 22필드 오프셋 0x00~0x61 = tcxdict 전건 일치, 선언줄 = `ai_interface.rs:444~465`. ③rmeta 주석 `_docs\game_ai.txt:510`. ④MIR = **재료 부재**(`mirdump_game_ai` 에 fold/track 둘 다 없다 — 실측) ⑤오라클 = **E0624 차단**(실측) ⟹ 각 코드의 *의미* 제3출처는 **exe 디스어셈만 남음(미탐색)**. |  |  |  |  |  |  |
| 4 | BigPlan::get_name 표가 IR 근거뿐 | ★오라클 실행 검증 8종(2차배치B, `B_o1.tsv`): ForcePassive→"ForcePasive"(**원문 오타 실행 확인**)→9 / ActiveRecall→6 / EpicHuntAndPoke·EpicHuntAndBattle→4 / SerpenHuntAndPoke·SerpenHuntAndBattle→5 / AttackNexus·DefenseNexus→8. 미탐색 = 나머지 8 variant(private 필드라 구성 불가). |  |  |  |  |  |  |
| 5 | 05 `end_reason` 0~8 의 *의미* — 제3출처가 exe 디스어셈만 남았다고 판단했다 | ★**거짓이었다 — 호출자 IR 에 전부 있었다**(3차 배치B). `m13.ll` 의 `update` 에 인라인된 `v50_track_dive_episode` 에 코드 산출 지점이 다 있다. 코드표:   0 = 다이브 타워 변경 / 1 = BigPlan≠Battle / 2 = with_dive false / 3 = dive_tower None /   4 = sub_goal 이 RunAway·End / 5 = 대상 엔티티 소멸 / 6 = 대상이 적팀 챔피언 아님 /   7 = **다이브할 타워가 없음** / 8 = 내 챔피언 캐시 없음 ⟹ 「end_reason==7 = 포기 집계 제외」가 **의미까지** 확정. ⚠이건 메인 세션 브리핑의 범위 판정 오류였다 — 「가진 재료의 한계를 문제의 한계로 착각」 재발. |  |  |  |  |  |  |
| 6 | 05 `last_dive_abandon_tick` 의 소비 게이트 | ★확정(3차 배치B). 읽는 곳 **8군데 전부** `tick > 값 + 1 + 4×tick_per_second` (≈4초 재시도 쿨다운). 두 번째 writer 도 발견(handler.rs:1205). |  |  |  |  |  |  |
| 7 | `V50DiveEpisode` 에 `prev_holder_hp`/`gap_ticks` 가 있는가 | ★**필드가 아예 없다**(tcx 확정, 3차 배치B). |  |  |  |  |  |  |
| 8 | 05·06 은 오라클이 `E0624`(private) 로 막힌다 — 2·3차 판정 | ⚠**범위 정정(4차 배치B)** — 그건 **직접 호출만** 참이다. `AgentVerHamster::plan_v50_dive_episodes`(pub, lib.rs:321)로 **05 의 레코드 Vec 을 꺼낼 수 있다.** (4차엔 미실행 — 05 의 남은 미탐색 2건은 그 경로로도 답이 안 나와서. 범위를 명시해 둔다.) |  |  |  |  |  |  |
</details>

