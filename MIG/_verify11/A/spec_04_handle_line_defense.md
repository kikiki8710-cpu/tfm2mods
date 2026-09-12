---

### `04` handle_line_defense — 적팀 몰가드(에픽) 버프가 켜진 상태에서, 해당 라인 타워 주변 적 챔피언 수가 전략별 구간에 들면 그 라인 방어를 채택

| 항목 | 값 |
|---|---|
| id | `defense_nexus__handle_line_defense` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus19handle_line_defense` |
| 소스 | `game-ai\src\plan_legacy\old\defense_nexus.rs:543` |
| IR | `m04.ll` 58175~58644행 |
| 경로·가시성 | `game_ai::plan_legacy::old::handle_line_defense` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d3e4b0` (defense_nexus) · 337바이트 · 96명령 |
| 라운드 | 기준 `r6` · 통과 6회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize | 본문에서 전혀 안 쓰임(이름 앞 _). AI 버전 게이트 분기 없음 | 4 |
| 1 | 2 | rnd | &mut rand::rngs::std::StdRng(320B = ChaCha12Rng) | 이 함수가 직접 소비하지 않는다. PlayerState::strategy(575줄)에 그대로 넘김 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930)만 직접 읽고, 나머지는 피호출 함수로 전달 | 4 |
| 3 | 4 | data | &OperationData(24B) | cache(+0x0)/context(+0x8)/blackboard(+0x10) 세 참조의 묶음 | 4 |
| 4 | 5 | line | LineType(1B, range 0..3) | 0=Top 1=Mid 2=Bottom (dienum LineType). alloca %10에 담겨 &line 으로도 전달됨 | 3 |
| 5 | 6 | _debug | &mut DebugFrameData(224B) | readnone — 본문에서 안 씀 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn handle_line_defense(_version, rnd, player, data, line, _debug) -> bool

// --- 544줄: 그 라인이 맵에 존재하는가
if !rule_scope::line_exists(data.context /*64B*/, line) { return false } // IR blk6

// --- 549줄: rule_scope::morgard_exists(...) 가 통째로 인라인됨 (define 없음)
// (a) runner.rs:263 TutorialType::spawn_epic
let t = data.context.tutorial as u8;
if t.wrapping_sub(1) < 6 { return false } // t ∈ 1..=6(First,TopSolo,Bottom,MidSolo,MidBottom,JungleOnly) 이면 종료
 // 통과하는 값 = 0(None) · 7(Line) · 8(Total) // IR blk14
// (b) game.rs:231 game.get_game_mode().as_moba().unwrap() ← None 이면 option::unwrap_failed 패닉(IR blk174)
let moba: &MobaMode = (*data.cache.game).get_game_mode()[vtable+0x40].as_moba().unwrap(); // IR blk21
let my_team = player.info.team; // +0x930
let enemy = 1 - my_team; // 0..1 범위 밖이면 panic_bounds_check(len=2)
// (c) game.rs:210 MobaMode::remain_epic_time(enemy)
if moba.epic_minion_buff_time[enemy] == 0 { return false } // IR blk38
// ⇒ 여기까지가 '적팀에 몰가드(에픽) 미니언 버프가 살아 있다' 게이트

// --- 553줄: AbstractGameWithCache::tower(line, my_team) 인라인 (simulation.rs:1822)
// line 별로 (기본타워, 보조타워2) 배열쌍을 골라 Option::or
let (a, b) = match line { Top => (0x180, 0x190), Mid => (0x1a0, 0x1b0), Bottom => (0x1c0, 0x1d0) };
let t1: Option<&Entity> = cache[a][my_team];
let t2: Option<&Entity> = cache[b][my_team];
let tower0 = t1.or(t2); // %63 = if t1.is_none() { t2 } else { t1 }

// --- 554~559줄: 없으면 트윈타워 중 라인 시작지점에 가장 가까운 것
let start = LineType::get_start_position(&line, data.context.setting /*5432B*/, my_team); // 556줄, (i64,i64)
let twins = &cache.twin_towers[my_team]; // +0x130, Vec<&Entity>
let nearest = twins.iter()
 .min_by_key(|e| { let dx = |e.x - start.0|; let dy = |e.y - start.1|; dx*dx + dy*dy }) // 부호없는 차의 절댓값
 .map(|x| *x); // 559줄 — 빈 벡터면 None (동점이면 먼저 나온 원소 유지)
let tower = tower0.or(nearest);

// --- 565줄: 둘 다 없으면 종료 (원본은 is_none 체크 + unwrap 형태 — unwrap 패닉 경로가 제거돼 있음)
let tower: &Entity = match tower { Some(t) => t, None => return false }; // IR blk105→19

// --- 567줄: 이 라인에 실제 위협이 있는가
if !defense_nexus::has_line_defense_threat(player, data, line, tower.id /*+0x5c0*/) { return false } // IR blk109

// --- 571~573줄: 타워 주변 '최근에 보인' 적 챔피언 수
let mut near_enemy_champion = 0;
for e in data.cache.champions(enemy, data.context.pool) { // Vec<&Entity>, 끝나면 Drop 호출
 // 572줄 closure$2
 if data.blackboard[enemy].is_recent_visible(data.cache.game /*dyn*/, player, e) {
 let dx = |e.x - tower.x|; let dy = |e.y - tower.y|;
 if dx*dx + dy*dy < 40000000001 { near_enemy_champion += 1 } // 거리 ≤ 200000 (6.25셀)
 }
}

// --- 575~578줄: 전략별 인원 구간 판정
let strategy = player.strategy(rnd, data.cache.game); // Strategy(24B) sret
let defense = strategy.morgard_defense; // +0xe : 0=Gather 1=Battle
return if defense == Battle { (near_enemy_champion - 1) <u 2 } // 1명 또는 2명일 때만 true
 else { near_enemy_champion > 1 }; // Gather: 2명 이상이면 true
```

**`mem` 메모리 접근 28건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(8840B). %22 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext(64B). %12 — line_exists 의 인자 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. %123 — 571줄 필터에서 [적팀] 원소를 씀 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 3 | GameContext | 0x0 | pool | r | &bumpalo::Bump. champions() 3번째 인자(%116) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 4 | GameContext | 0x8 | setting | r | &GameSetting(5432B). LineType::get_start_position 2번째 인자(%72) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 5 | GameContext | 0x38 | tutorial | r | TutorialType(1B, range 0..9). 549줄 morgard_exists 인라인의 첫 게이트(%16) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 6 | AbstractGameWithCache | 0x0 | game | r | &dyn AbstractGame 팻포인터. +0x0=data(%23), +0x8=vtable(%25, dereferenceable(816)) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 7 | AbstractGame::vtable | 0x40 | get_game_mode | r | divtable AbstractGame 0x40 → get_game_mode (일치율 98%, vtable 816B=IR deref 와 일치). 반환 {i64,ptr}=Option<&MobaMode> | 3 | 확인불가(vtable 슬롯) |
| 8 | MobaMode | 0x240 | epic_minion_buff_time[team] | r | MobaMode::remain_epic_time(team) 이 인라인된 자리(game.rs:210). team=적팀 인덱스 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 9 | PlayerState | 0x930 | info.team | r | usize. %35=아군팀, %36=1-%35=적팀 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 10 | AbstractGameWithCache | 0x130 | twin_towers[아군팀] | r | Vec<&Entity>(32B stride). 559줄 min_by_key 대상 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 11 | AbstractGameWithCache | 0x180 | top_tower[아군팀] | r | line==Top 일 때 %54=384 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 12 | AbstractGameWithCache | 0x190 | top_tower2[아군팀] | r | line==Top 일 때 %55=400 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 13 | AbstractGameWithCache | 0x1a0 | mid_tower[아군팀] | r | line==Mid 일 때 %54=416 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 14 | AbstractGameWithCache | 0x1b0 | mid_tower2[아군팀] | r | line==Mid 일 때 %55=432 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 15 | AbstractGameWithCache | 0x1c0 | bottom_tower[아군팀] | r | line==Bottom 일 때 %54=448 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 16 | AbstractGameWithCache | 0x1d0 | bottom_tower2[아군팀] | r | line==Bottom 일 때 %55=464 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 17 | Vec<&Entity>(bumpalo) | 0x0 | ptr | r | 버퍼 시작(bumpalo RawVec.buf.ptr.pointer). ★10차 배치A 신규 — IR 이 두 곳에서 실제로 로드하는데 표에 행이 없었다: m04.ll:58300 `%66 = load ptr, ptr %65`(twin_towers[아군팀], 559줄 min_by_key 의 시작 주소) · m04.ll:58451 `%117 = load ptr, ptr %9`(champions 결과, 571줄 루프의 시작 주소). 같은 접근이 `03 defensive_crisis` 에는 mem[10] 으로 실려 있다. 필드명 = tcx 정본 대조(`tcxdict bumpalo::collections::vec::Vec<'{erased}, &'{erased} game_core::Entity>` = 0x0 buf: RawVec / 0x18 len; RawVec = 0x0 ptr.pointer / 0x8 a: &Bump / 0x10 cap) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 18 | Vec<&Entity>(bumpalo) | 0x18 | len | r | twin_towers 벡터(base %65, len %68 @m04.ll:58303)와 champions 결과 벡터(base %9, len %119 @m04.ll:58454) 둘 다 +0x18 로 읽는다. ⚠%68·%119 는 **벡터가 아니라 len 값**이다. +0x0(ptr) 은 별도 행으로 분리했다(바로 위 행 — 한 행에 오프셋을 묶으면 기계 검사가 안 된다는 이 명세의 mem[22]~[24] 규약과 같은 이유) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) ★BUMPVEC len@+0x18 판별) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 19 | Entity | 0x5c0 | id | r | usize. 567줄 has_line_defense_threat 의 4번째 인자(%112) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 20 | Entity | 0x660 | x | r | u64. 거리 제곱 계산(타워/트윈타워/적 챔피언 모두) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 21 | Entity | 0x668 | y | r | u64. 거리 제곱 계산 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 22 | Strategy | 0xe | morgard_defense | r | MorgardDefenseStrategy(1B, range 0..2): 0=Gather 1=Battle. 576줄, PlayerState::strategy 의 sret 버퍼(%8)+14 · tcx 정본 대조( A6_o4.tsv STRATEGY morgard_defense_offset=0xe 실측) | 3 | OK |
| 23 | Blackboard | 0x0 | top_minion_state | r | BrainMinionParameter(40B). `has_line_defense_threat`(defense_nexus.rs:588)가 line 으로 switch 해 고른다 — IR `data+0x10 -> gep Blackboard, team` 뒤 `switch line {0:+0, 1:+40, 2:+80}` . ★한 행에 오프셋을 묶으면 기계 검사가 안 되므로 라인별로 쪼갰다 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 24 | Blackboard | 0x28 | mid_minion_state | r | BrainMinionParameter(40B). `has_line_defense_threat`(defense_nexus.rs:588)가 line 으로 switch 해 고른다 — IR `data+0x10 -> gep Blackboard, team` 뒤 `switch line {0:+0, 1:+40, 2:+80}` . ★한 행에 오프셋을 묶으면 기계 검사가 안 되므로 라인별로 쪼갰다 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 25 | Blackboard | 0x50 | bottom_minion_state | r | BrainMinionParameter(40B). `has_line_defense_threat`(defense_nexus.rs:588)가 line 으로 switch 해 고른다 — IR `data+0x10 -> gep Blackboard, team` 뒤 `switch line {0:+0, 1:+40, 2:+80}` . ★한 행에 오프셋을 묶으면 기계 검사가 안 되므로 라인별로 쪼갰다 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 26 | BrainMinionParameter | 0x10 | from_mid | r | i64. `< -3000` 이면 라인 밀림 위협 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 27 | BrainMinionParameter | 0x20 | minion_count | r | i32. `< -2` 이면 미니언 수 열세 위협 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 549 | 계수 | runner.rs:263 TutorialType::spawn_epic 인라인 — (tutorial as u8).wrapping_sub(1). 578줄의 (near-1) 계산에도 같은 -1 이 쓰임 · 오라클 실행 확증( TutorialType 9값 spawn_epic 실행 = 0/7/8 만 true (A6_o3 SPAWN_EPIC)) | 2 |
| 1 | 6 | 549 | 임계 | 위 결과와의 부호없는 비교 상한: tutorial-1 <u 6 → 즉시 false 반환. 통과 조건은 tutorial ∈ {0=None, 7=Line, 8=Total} · 오라클 실행 확증 | 2 |
| 2 | 40000000001 | 572 | 임계 | 200000^2 + 1 — 타워~적 챔피언 제곱거리 임계(d2 < 이 값 ⟺ 거리 ≤ 200000 = 6.25셀). 셀=32000 · 오라클 실행 확증( 오라클 `_verify3` 반경 경계 d2=40000000000 통과 / 40000400001 탈락 (history[5] 지시 반영)) | 2 |
| 3 | 1 | 578 | 임계 | Gather 분기의 하한 비교값: near_enemy_champion >u 1 (= 2명 이상). 같은 리터럴이 549줄의 적팀 첨자 산출 `1 - my_team` 에도 쓰임 (★7차 배치A: `src_line` 578 의 용도는 비교 임계다 — 부수 언급 한 낱말 때문에 분류가 튀어 있었다) · 오라클 실행 확증( 오라클 인원 구간 20/20, Gather `>1` (history[5] 지시 반영)) | 2 |
| 4 | 2 | 578 | 임계 | Battle 분기의 구간 폭: (near_enemy_champion - 1) <u 2 (= 1명 또는 2명). 배열 길이 2(팀 수) 경계검사 상수와 리터럴이 겹침 · 오라클 실행 확증( 오라클 인원 구간 20/20, Battle `{1,2}` (history[5] 지시 반영)) | 2 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 타워 주변 '근처 적 챔피언' 판정 반경(제곱) | defense_nexus.rs:572 (IR m04.ll:58583 `icmp ult i64 %151, 40000000001`) | 40000000001 | 올리면 더 멀리 있는 적까지 '근처'로 세어 near_enemy_champion 이 커진다 → Gather 전략에선 라인 방어가 더 잘 발동하고, Battle 전략에선 오히려 상한 2명을 넘겨 발동이 줄 수 있다(양방향). 값은 거리의 제곱이므로 반경 R 을 원하면 R*R+1 로 넣어야 한다 · 오라클 실행 확증( 오라클 반경 경계 실측 (history[5] 지시 반영)) | 2 | 기존 |
| 1 | Battle 전략의 발동 인원 구간 폭 | defense_nexus.rs:578 (IR m04.ll:58635 `icmp ult i64 %170, 2`) | 2 | 올리면 (1..=N) 구간이 넓어져 적이 더 많이 몰려 있어도 라인 방어를 채택한다. 0 으로 내리면 Battle 전략에서 라인 방어가 사실상 죽는다 · 오라클 실행 확증( 오라클 Battle 구간 20/20 (history[5] 지시 반영)) | 2 | 기존 |
| 2 | Gather 전략의 발동 최소 인원 | defense_nexus.rs:578 (IR m04.ll:58636 `icmp ugt i64 %162, 1`) | 1 | 이 값보다 많아야(>) 발동하므로 0 으로 내리면 적 1명만 있어도 라인 방어를 채택, 올리면 더 많이 몰려야 발동한다 · 오라클 실행 확증( 오라클 Gather 하한 20/20 (history[5] 지시 반영)) | 2 | 기존 |
| 3 | 튜토리얼 게이트 상한 | defense_nexus.rs:549 ← rule_scope.rs:46 ← runner.rs:263 (IR m04.ll:58200 `icmp ult i8 %17, 6`) | 6 | 이 비교를 무력화하면(예: 0) 모든 TutorialType 에서 라인 방어 판정이 살아난다. 일반 게임은 tutorial=None(0) 이라 이미 통과하므로 실전 영향은 없고 튜토리얼 전용 노브다 · 오라클 실행 확증( A6_o3.tsv SPAWN_EPIC 9값 실행 — tutorial 게이트 전수) | 2 | 기존 |
| 4 | 라인 시작 좌표 6종 | _gcbc/g15.ll:115823 `define { i64, i64 } @…LineType18get_start_position` — 좌표 3쌍 = 115875 `%9 = phi i64 [ 880000, %7 ], [ 817000, %6 ], [ 820000, %3 ]` / 115876 `%10 = phi i64 [ 144000, %7 ], [ 144000, %6 ], [ 80000, %3 ]`, 팀 스왑 = 115891~115893 `%11 = icmp eq i64 %2, 1` + `%12 = select i1 %11, i64 %10, i64 %9` / `%13 = select i1 %11, i64 %9, i64 %10` → 115908 `ret { i64, i64 }`(%13, %12) | **team0** Top (80000,820000) · Mid (144000,817000) · Bottom (144000,880000) / **team1** Top (820000,80000) · Mid (817000,144000) · Bottom (880000,144000) (양 팀 좌표 모두 오라클 실측). ★구조 정정(10차 배치A) — **독립 좌표는 6개가 아니라 3쌍**이다: IR 은 라인별 3쌍 `(880000,144000)/(817000,144000)/(820000,80000)`(g15.ll:115875~115876 phi) 만 갖고, team 축은 `team==1 ? (x,y) : (y,x)` **x↔y 스왑**으로 만든다(115891~115893 `icmp eq i64 %2, 1` + select 2개). ⟹ team0 값을 team1 과 **독립으로는 못 바꾼다** — 개입점은 3쌍의 리터럴과 스왑 조건 둘뿐이다 | 챔피언·미니언 라인 진입 지점 이동. `setting` 미참조라 설정으로는 못 바꾸고 **바이트패치/재구현만** 가능 · 오라클 실행 확증( 오라클 실측 6좌표(history[3]⑤) — 팀별 값까지 확정) | 2 | 신규 |
| 5 | 라인 방어 위협 게이트 — 라인 밀림 임계 | defense_nexus.rs:589 (m04.ll:60449 `icmp slt i64 %22, -3000`) | -3000 | `blackboard[내팀].<line>_minion_state.from_mid`(BrainMinionParameter+0x10)가 이보다 작아야(우리 쪽으로 밀렸어야) 위협 판정에 진입한다. 0 쪽으로 올리면 조금만 밀려도 라인 방어를 검토 | 4 | 신규 |
| 6 | 라인 방어 위협 게이트 — 미니언 수 열세 임계 | defense_nexus.rs:589~590 (m04.ll:60455 `icmp slt i32 %26, -2`) | -2 | `minion_count`(+0x20)가 이보다 작아야 위협. 두 조건은 **OR** 이라 하나만 만족해도 다음 단계(적 미니언 타깃 확인)로 간다 | 4 | 신규 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev |
|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 3 |
| 1 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 |
| 2 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 3 |
| 3 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 3 |
| 4 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 3 |
| 5 | get_game_mode | <game_core::DeathMatchGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::DeathMatchGame) -> game_core::GameMode | game-core\src\simulation\game.rs:5052 | True | True | 3 |
| 6 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 |
| 7 | handle_line_defense | game_ai::plan_legacy::old::handle_line_defense | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:543 | False | False | 3 |
| 8 | has_line_defense_threat | game_ai::plan_legacy::old::has_line_defense_threat | pub | fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType, usize) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:587 | False | False | 3 |
| 9 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 |
| 10 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 |
| 11 | morgard_exists | game_ai::plan_legacy::rule_scope::morgard_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:45 | True | True | 3 |
| 12 | remain_epic_time | game_core::MobaMode::remain_epic_time | pub | fn(&game_core::MobaMode, usize) -> usize | game-core\src\simulation\game.rs:210 | True | True | 3 |
| 13 | strategy | <game_core::Game as game_core::AbstractGame>::strategy | pub | fn(&game_core::Game, usize) -> game_core::Strategy | game-core\src\simulation\game.rs:1830 | True | True | 3 |
| 14 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 |
| 15 | strategy | <game_core::ExpectedGame<'a> as game_core::AbstractGame>::strategy | pub | fn(&game_core::ExpectedGame<'a/#0>, usize) -> game_core::Strategy | game-core\src\simulation\expected_game.rs:57 | False | False | 3 |
| 16 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 3 |
</details>

⚠**미매칭 1개**: `wrapping_sub`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m09.ll:9206, m09.ll:9272, m09.ll:9353, m13.ll:31086) · **형제 0개** 

**`open` 1건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 이 함수 자체는 구조체 필드에 **아무것도 쓰지 않는다**(store 대상은 alloca %7/%10 뿐) — 그래서 writes 가 빈 배열이다. 다만 `rnd`(&mut StdRng)는 readonly 가 아니라 PlayerState::strategy 안에서 소비될 수 있다(그 함수는 안 봄). → 6차 배치A 실측: `player.strategy(&mut rnd, &game)` 을 **직접 호출**해 전후 320B 를 비교했더니 **바뀌지 않았다**(A6_o4.tsv `STRATEGY rnd_state_changed=false`). ★범위: 팀 전략이 확정돼 있는 한 상태 1종에서만 잰 값이라 '난수를 절대 안 쓴다'로 일반화하지 말 것(`TeamColorStrategy_random` 경로가 있다). | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | `_version`(p1)·`_debug`(p6)는 본문에서 전혀 쓰이지 않는다(p6 는 readnone). 버전 게이트가 상수접힘으로 사라진 게 아니라 애초에 참조가 없다. | 4 | 사실 서술 |

<details><summary>`closed` 7건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | `morgard_exists`(rule_scope.rs:45)는 별도 define 이 없이 549줄에 통째로 인라인돼 있어, 원본이 `if !morgard_exists(..)` 인지 `if morgard_exists(..)` 인지 **소스 표기는 확정 불가**. 관측 사실만 확정: (tutorial-1 <u 6) → false 반환, epic_minion_buff_time[적팀]==0 → false 반환. ★이 항목 전체가 무효다(10차 배치A) — 같은 파일 `history[3]`(2026-09-11 검증배치 A)이 세 주장을 모두 뒤집었는데 여기에 전파되지 않았다: ①**「소스 표기 확정 불가」는 과했다** — `defense_nexus.rs` L544=39자·L549=134자 복원 ±0 으로 `if !morgard_exists(data.context) \|\| …` **부정형 확정** ②`morgard_exists` 는 `pub morgard_exists(&GameContext) -> bool`(mir=True)이고 MIR 이 `switchInt(discriminant(ctx.tutorial)) -> [0:true, 7:true, 8:true, otherwise:false]` **한 줄이 전부** — (b)(c)는 `:549` 의 별개 항이다 ③**「{0,7,8} 은 추정」도 과했다** — `TutorialType::spawn_epic()` 전 9값 오라클 실행으로 확정(None0 true / First1~JungleOnly6 false / Line7 true / Total8 true). ⟹ 남는 미확정은 없다. | 본문에 해소 표기가 있다 |
| 1 | `Blackboard` 배열을 **적팀 인덱스(%36)** 로 뽑아 is_recent_visible 의 self 로 쓴다(IR %126 = gep Blackboard, %123, %36). 이 배열이 '관측하는 팀'별인지 '관측당하는 팀'별인지는 Blackboard 쪽을 안 봐서 미확정 — 오프셋/인덱스 사실만 적었다. | 1차 배치A: _shared.is_recent_visible 로 인덱스 의미 확정 |
| 2 | `AbstractGameWithCache::champions`, `Blackboard::is_recent_visible`, `PlayerState::strategy`, `LineType::get_start_position`, `rule_scope::line_exists`, `has_line_defense_threat` 중 **`has_line_defense_threat` 는 define 이 있다** — `_gaibc/m04.ll:60400`(defense_nexus.rs:587~604, pub. 같은 파일 `history[4]` 가 본문을 전부 폈다). 나머지 5개만 `_gaibc` 외부 선언이라 인자 이름 DWARF 가 없다. `champions` 의 2번째 인자 = **팀** 도 ★확정됐다(`history[3]`④: 본체 `_gcbc/g15.ll:109887~109895` 에서 `%8 = cache+480`(player_champion) 을 `[5 x ptr]` 로 잡고 **2번째 인자를 그 첨자**로 쓴다) — 「%36 이 remain_epic_time 의 DWARF 인자명 team 에 붙어 있다」는 정황 근거는 더 이상 유일 근거가 아니다. | 2차 배치A: tcx sig + 본체 확인, 2번째 인자 = 팀. 오라클 len()==5 |
| 3 | vtable 슬롯 0x40 → `get_game_mode` 는 divtable 의 정적 vtable(@anon...157, 일치율 98%)에서 나온 이름이다. 런타임에 어느 구현체(ExpectedGame 외)가 꽂히는지는 확인 불가. | 4차 배치A: 구현 정확히 4개(tcx), ExpectedGame 은 순수 위임이라 종단 3개이고 Game 만 Moba(MIR) |
| 4 | `MobaMode+0x240` 을 `epic_minion_buff_time` 으로 읽은 건 distruct 결과(MobaMode 640B, +0x240 = array<usize> 16B)와 DWARF 함수명 `remain_epic_time` 의 조합이다. MobaMode 크기·필드명은 ★해소(10차 배치A) — `tcxdict`(정본)가 `game_core::MobaMode (640B · struct · 필드 9, game.rs:182)` 의 `0x240 epic_minion_buff_time: [usize; 2]`(16B) 를 직접 돌려준다. distruct+DWARF 함수명 조합이 아니라 **컴파일러 정본 대조**로 올라간다(인접: 0x228 serpen_logs · 0x250 epic_count · 0x260 serpen_count · 0x270 missed_cs_line_phase — 전부 `[usize;2]` 라 오프셋 오귀속 시 이웃과 헷갈릴 여지가 있었는데 그것도 닫힌다). | 본문에 해소 표기가 있다 |
| 5 | `min_by_key` 의 fold 본체는 담당 범위 밖 m12.ll:30536~30687 에 있다(fnparts). 그 안의 키 계산식은 담당 범위(58390~58404)의 최초 원소 계산과 비트 동일하게 재현돼 있어 그대로 적었고, 별도 상수는 없어 knobs 에 추가한 것은 없다. | 4차 배치A: history[0] 이 '★완전히 동일'로 이미 닫음 |
| 6 | 반환 true 의 최종 의미는 호출측(m09.ll:9202~9209, m13.ll:31086)에서 플랜 태그를 store 하는 것으로 확인했으나, 그 핸들러 +1055/+1056 필드가 무엇인지는 담당 범위 밖이라 안 봤다. | 4차 배치A: 「핸들러」 = TeamPlan(1064B). +0x41f objective 판별자 / +0x420 페이로드 태그. true⟹DefenseLine(line)(3,1) / false⟹Defense(2) |
</details>

<details><summary>`history` 정정 이력 6건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | aux | 상수 |
|---|---|---|---|---|
| 0 | fold 본체의 키 계산식이 담당범위와 동일한지 | ★완전히 동일. 확인 항목 5개(get_start_position 호출·entity+0x660/+0x668 로드·dx select·dy select·dx²+dy²)가 바이트 단위로 같다. m04.ll:58373~58403 ↔ m12.ll:30621~30651. | [{"ir_file": "m12.ll", "ir_from": 30536, "ir_to": 30687}] | 판정 상수 0건. 필터 없음 — 순수 Map + min_by_key. |
| 1 | blackboard 인덱스 의미 | _shared.is_recent_visible.blackboard_인덱스_의미 참조 — 확정됨. |  |  |
| 2 | get_start_position 2번째 인자(5432B 구조체) 타입명 | ★확정 = **`&GameSetting`**(DWARF `!122698 name:"setting"` → `ref$<game_core::setting::GameSetting>`, `distruct GameSetting` = 5432B 정확히 일치). ⚠**본문에서 한 번도 로드되지 않는다** — 라인 시작 좌표가 전부 리터럴 하드코딩이다. 좌표표는 `_shared.맵_좌표계.get_start_position`. |  |  |
| 3 | `morgard_exists` 가 (b)(c) 조건을 포함한다는 구조 서술 / "소스 표기 확정 불가" / "{0,7,8} 은 추정" | ★**정정 + 범위정정(2026-09-11 검증배치 A)** **① 구조 오류**: `morgard_exists` 는 (b)(c)를 **포함하지 않는다**. tcx 시그니처가 `pub morgard_exists(&GameContext) -> bool`(mir=True) — **인자에 `game` 이 아예 없어서** 구조상 들어갈 수 없다. MIR 전문 = `_3 = discriminant(ctx.tutorial)` @runner.rs:263 → `switchInt -> [0:true, 7:true, 8:true, otherwise:false]` **튜토리얼 게이트 한 줄이 전부**다. (b)(c)는 `defense_nexus.rs:549` 의 **별개 항**이다 — `dloc` inlinedAt 루트가 그것을 가른다: `!66990` 은 `spawn_epic ← morgard_exists ← :549` 인데 `!66998`(as_moba)·`!67010`(remain_epic_time)에는 **`morgard_exists` 프레임이 없다**.   본문 복원 ±0: `pub fn morgard_exists(context: &GameContext) -> bool { context.tutorial.spawn_epic() }` (rule_scope.rs L45=54자 / **L46=31자** = `  context.tutorial.spawn_epic()` / L47=1자, MIR span `46:20-46:32` 로 칸까지 일치) **② 「소스 표기 확정 불가」는 과했다 — 확정됨**: `defense_nexus.rs` 문자수 L544=**39** = `  if !line_exists(data.context, line) {` ±0 (L545 `    return false;`=17, L546 `  }`=3) ⟹ **`if !… { return false; }` 형태 확정**. L549=**134** = `  if !morgard_exists(data.context) \|\| data.cache.game.get_game_mode().as_moba().unwrap().remain_epic_time(1 - player.info.team) == 0 {` ±0 ⟹ **`!morgard_exists(..)` 확정**(둘째 항은 검산 일치이나 유일성 미보장). **③ 「{0,7,8} 은 추정」도 과했다 — 오라클 실행으로 확정**: `TutorialType::spawn_epic()` 전 9값 실행 = None0 **true** / First1~JungleOnly6 전부 false / Line7 **true** / Total8 **true**. MIR switch 와 일치. **④ 부수 확정**: `champions(&self, usize, &Bump)` 의 2번째 인자 = **팀**(본체 `_gcbc/g15.ll:109887~109895`, `%8=cache+480(player_champion)`, `%9=[5 x ptr] gep index %2`) · `MobaMode+0x240 = epic_minion_buff_time[0]`(tcx, MobaMode 640B), `remain_epic_time(team) = self.<f2>[team]` **시간 산술 없이 배열 직독**. **⑤ 좌표표에 팀 표기 누락**: 기재된 Top 820000/80000 · Mid 817000/144000 · Bottom 880000/144000 은 **team1 값만**이다. 오라클 실측 = Top t0 (80000,820000) / t1 (820000,80000) · Mid t0 (144000,817000) / t1 (817000,144000) · Bottom t0 (144000,880000) / t1 (880000,144000). **⑥ 부수 확증**: 프로브의 `GameSetting::default()` 가 `tick_per_second=0` 인 영행렬인데도 좌표가 정상값으로 나왔다 ⟹ 「`setting` 은 본문에서 한 번도 로드되지 않는다」가 **실행으로 확증**됐다. |  |  |
| 4 | `has_line_defense_threat(player, data, line, tower.id)` 는 _gaibc 에 define 이 없는 외부 선언 | ★**define 이 있다**(2026-09-11 2차배치A): `_gaibc/m04.ll:60400`(defense_nexus.rs:587~604), pub.   let bm = data.blackboard[player.info.team].<line>_minion_state;            // :588        //  Blackboard +0x0 top / +0x28 mid / +0x50 bottom (각 BrainMinionParameter 40B)        //  IR: data+0x10 -> gep Blackboard, team -> switch line {0:+0, 1:+40, 2:+80}   if bm.from_mid(+0x10,i64) < -3000 \|\| bm.minion_count(+0x20,i32) < -2 {     // :589~590     data.cache.minions(1 - team, pool).any(\|m\|                               // :594 ★적팀 미니언          map_regions::is_near_line(ctx.map, m.x(+0x660), m.y(+0x668), line)  // :595       && m.ty(+0x68) == 1 /*Minion*/                                         // :599       && m.ty.Minion.<target: Option<usize>>(tag +0x88 / val +0x90) == Some(tower_id))  // :600   } else { false } ⟹ **'라인이 밀렸다(from_mid<-3000) 또는 미니언 수가 밀린다(minion_count<-2)' AND '그 타워를 노리는 적 미니언이 있다'**. ★오라클 항별 분해(`A2_oracle4.tsv`): line_exists=true·morgard=true·epic1=600·visible=5/5·near=0~3 인데도 **threat=false** 라 handle_line_defense 가 false 였다 ⟹ 실패 원인을 이 게이트로 실행 특정. ★정정(3차 배치A): `is_near_line` 의 1번 인자는 `ctx.map` 이 아니라 **`&GameContext`** 다 — tcx `game_core::is_near_line(&GameContext, u64, u64, LineType)`(map_regions.rs:139) + IR m04.ll:60492 가 넘기는 `%32` = `data.context`(deref 64B) |  |  |
| 5 | `has_line_defense_threat` 를 실제로 true 로 만들 수 있는가 | ★**점등 성공**(3차 배치A). `(from_mid < -3000 \|\| minion_count < -2) && ∃ 적 미니언(nearest_enemy == Some(tower_id))` 의 OR·AND 구조를 실행 확증. ★미확정이던 `<target>` 필드 이름 확정 = **`Minion::nearest_enemy`**(Entity +0x88 태그 / +0x90 값). `Blackboard::minion_state(line)` 는 필드가 아니라 **메서드**(blackboard.rs:378). ★인원 구간 20/20 + 반경 경계(d2 = 40000000000 통과 · 40000400001 탈락): Gather `>1` / Battle `{1,2}` ⟹ knobs[0]·[1]·[2]·consts[2] **ev 4→2**. |  |  |
</details>

