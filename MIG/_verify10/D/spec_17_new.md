---

### `17` new — DeathMatchBattle(384B) 생성자 — 36개 필드를 전부 초기화하고 TryKill 목표면 Battle 채팅 1건을 실어 보낸다.

| 항목 | 값 |
|---|---|
| id | `old_death_battle__new` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12death_battleNtB2_16DeathMatchBattle3new` |
| 소스 | `game-ai\src\plan_legacy\old\death_battle.rs:746` |
| IR | `m05.ll` 17695~17825행 |
| 경로·가시성 | `game_ai::plan_legacy::old::DeathMatchBattle::new` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `None` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r4` · 통과 4회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::DeathMatchBattle
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 0 | (sret) |  | 반환값 out-ptr. m05.ll:17695 ptr dead_on_unwind noalias noundef writable writeonly sret([384 x i8]) align 8 captures(none) dereferenceable(384) %0 · tcx 정본 대조(9차 배치B: tcx 정본 대조: `fn(usize, BattlePlanGoal, &OperationData, &PlayerState) -> DeathMatchBattle` — 소스 인자 4개. m05.ll:17695 `define void @…DeathMatchBattle3new(ptr dead_on_unwind noalias noundef writable writeonly sret([384 x i8]) align 8 captures(none) dereferenceable(384) %0, i64 noundef %1, …)` — IR 인자 5개 = 4 + sret 1. 반환 DeathMatchBattle 이 384B) |
| 1 | 1 | version |  | ★IR %1 (=%0 은 반환 out-ptr). AI 버전 게이트. 이 함수 본문에서는 분기 없음 — base_sub_goal 로 그대로 전달만 한다(!dbg 753). sret out-ptr 은 m05.ll:17695 `define void @…DeathMatchBattle3new(ptr dead_on_unwind noalias noundef writable writeonly sret([384 x i8]) align 8 captures(none) dereferenceable(384) %0, i64 noundef %1, …)` 의 `%0` 이다. IR 인자 5개 ↔ params 5행으로 **자리 번호가 1:1 로 일치**한다 — 9차 시점에 `(sret)`(i=0) 행이 params[0] 로 실려 있다(8차 배치C 확정 규약 → 9차 `applypatch op:insert` 로 반영). ~~이 params 표는 sret 을 빠뜨렸다 · 표의 자리 번호가 IR 보다 한 칸 앞선다~~ 는 8차 기준 서술이라 9차에 정정 |
| 2 | 2 | goal |  | enum2$<...BattlePlanGoal>. 태그 i64@+0x0(!range 0..4), TryKill 은 (usize @+0x8, usize @+0x10). 그대로 self.main_goal 로 이동 |
| 3 | 3 | data |  | cache/context/blackboard. 여기선 cache 만 씀 |
| 4 | 4 | player |  | 본문에서 직접 로드하는 필드 없음 — base_sub_goal 인자로만 전달 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn DeathMatchBattle::new(version: usize, goal: BattlePlanGoal, data: &OperationData, player: &PlayerState) -> DeathMatchBattle

// 747
let mut chats: Vec<Chat> = Vec::new(); // ptr=dangling(8), len=0, cap=0

// 749~751 — 유일한 분기
if goal.tag == 0 { // BattlePlanGoal::TryKill(target, _)
 let target: usize = goal.TryKill.__0; // goal + 0x8
 chats.grow_one(); // cap 0 -> **4** // ★`RawVec::MIN_NON_ZERO_CAP`, `size_of::<Chat>()`=24 ⟹ 첫 push 에서 cap 이 **4**가 된다(실측 `vec_push1 w0=4 cap_api=4`, Vec::push 인라인)
 chats[0] = Chat::Battle(target, 0); // 태그 i8 3, __0=target, __1=0
 chats.len = 1;
}

// 753~784 — 구조체 리터럴. 필드 36개 전부 초기화(부분초기화 없음)
let sub = goal.base_sub_goal(version, player, data); // {i64,i64} = BattleSubPlanGoal
let tick = (*data.cache.game.vtable[0x28])(data.cache.game.data); // AbstractGame::tick()

DeathMatchBattle {
 main_goal: goal, // 0xd0, 인자 그대로 이동
 sub_goal: sub, // 0xe8
 chats, // 0xf8
 start_tick: tick, // 0x110
 flee_die: i64::MAX, // 0x118 '아직 없음' 센티널
 stance: DeathStance::Commit, // 0x176 = 0
 tactic: BattleTactic::Standard,// 0x177 = 0
 scene: DmScene::Stand, // 0x178 = 0
 scene_last_from: DmScene::Stand, // 0x179 = 0
 dive_tower: None, // 0x17a = 0xff
 main_objective: None, // 0x17b = 0xff
 // Option 계열 전부 None(판별자 0):
 support_target/region/well_runaway/death_focus/seal_basis/
 hold_scene_basis/repo_scene_basis/flee_dir/far_noout_since = None,
 // bool 6종 전부 false:
 with_dive/help_called/dive_abandoned/dodge_commit/last_stand/had_ult_ready = false,
 // 틱·누적 계열 **9필드** = memset(0x120, 0, 80) 로 일괄 0: // ★8×8B + `idle_prev_pos` **16B** = 80B 로 딱 맞는다
 trade_lean/lean_last_tick/scene_change_tick/last_act_tick/
 idle_spec_tick/last_swing_tick/ep_follow_until/idle_prev_pos/last_unseal_tick = 0,
 lean_last_sign: 0, // 0x17e
}

주의:
- 판정다운 판정은 단 하나 — 'goal 이 TryKill 인가'. 나머지는 전부 상수 초기화다.
- version 과 player 는 본문에서 한 번도 로드되지 않는다. base_sub_goal 로 넘어가므로
 실제 버전 게이트/플레이어 의존은 그 함수(m10.ll 29294~29997) 안에 있다.
- start_tick 은 dyn AbstractGame 의 vtable 간접호출이라 IR 에 심볼이 없다(divtable 로 tick 확정).
- unwind 경로(%16)는 chats Vec 을 drop_glue 로 정리만 한다 — 판정 아님.
```

**`mem` 메모리 접근 43건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | BattlePlanGoal(p2 goal) | 0x0 | tag | r | !range !20208 = [0,4) → 0=TryKill 1=Support 2=Response 3=Avoid (DISCR_EXACT 확인: tag==variant index) · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 1 | BattlePlanGoal::TryKill(p2 goal) | 0x8 | __0 (target) | r | usize. DILocalVariable name="target" (death_battle.rs:749). tag==0 일 때만 로드 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 2 | OperationData(p3 data) | 0x0 | cache | r | &AbstractGameWithCache · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 3 | AbstractGameWithCache | 0x0 | game.data_ptr | r | game: &dyn AbstractGame 팻포인터의 데이터 절반 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 4 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 팻포인터의 vtable 절반 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 5 | AbstractGame::vtable | 0x28 | tick | r | divtable.py AbstractGame 0x28 → ExpectedGame::AbstractGame::tick. 반환 i64 = 현재 틱. ★런타임 확증 : world.tick 을 0/1/4321/999999 로 바꾸면 DeathMatchBattle+0x110 start_tick 이 4/4 추종 ⟹ 슬롯 동작 확정. ⚠`chk` 는 tcxaudit 파생값이라 '확인불가(vtable 슬롯)' 로 계속 찍힌다 — **런타임 해소를 `chk` 에 반영할 채널이 파이프라인에 없다** · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 6 | DeathMatchBattle | 0x0 | support_target | w | Option<usize>, i64 판별자 0=None · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 7 | DeathMatchBattle | 0x10 | region | w | Option<BattleRegion> · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 8 | DeathMatchBattle | 0x30 | well_runaway | w | Option<usize> · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 9 | DeathMatchBattle | 0x40 | death_focus | w | Option<usize> · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 10 | DeathMatchBattle | 0x50 | seal_basis | w | Option<(usize,usize,usize)> · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 11 | DeathMatchBattle | 0x70 | hold_scene_basis | w | Option<(usize,usize,usize)> · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 12 | DeathMatchBattle | 0x90 | repo_scene_basis | w | Option<(usize,usize)> · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 13 | DeathMatchBattle | 0xa8 | flee_dir | w | Option<(i64,i64)> · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 14 | DeathMatchBattle | 0xc0 | far_noout_since | w | Option<usize> · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 15 | DeathMatchBattle | 0xd0 | main_goal | w | BattlePlanGoal. 인자를 통째로 이동. !dbg death_battle.rs:753 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 16 | DeathMatchBattle | 0xe8 | sub_goal (하위 8B) | w | BattleSubPlanGoal(16B). 반환 {i64,i64} 의 extractvalue 0 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 17 | DeathMatchBattle | 0xf0 | sub_goal (상위 8B) | w | 같은 필드의 뒤쪽 8B (extractvalue 1) · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 18 | DeathMatchBattle | 0xf8 | chats | w | TryKill 이면 원소 1개(Chat::Battle(target,0)), 아니면 빈 Vec(ptr=dangling 8, len=0, cap=0). !dbg 755 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 19 | DeathMatchBattle | 0x110 | start_tick | w | vtable+0x28 간접호출 결과. !dbg death_battle.rs:754 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 20 | DeathMatchBattle | 0x118 | flee_die | w | **`i64::MAX`** 센티널 = '아직 도주사망 판정 없음' (저장값 9223372036854775807 = 0x7FFF...FF, 타입은 `i64`) · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 21 | DeathMatchBattle | 0x120 | trade_lean | w | memset(0x120, 0, 80) 로 일괄 0 — 아래 0x168 까지 같은 memset · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 22 | DeathMatchBattle | 0x128 | lean_last_tick | w | memset 범위 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 23 | DeathMatchBattle | 0x130 | scene_change_tick | w | memset 범위 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 24 | DeathMatchBattle | 0x138 | last_act_tick | w | memset 범위 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 25 | DeathMatchBattle | 0x140 | idle_spec_tick | w | memset 범위 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 26 | DeathMatchBattle | 0x148 | last_swing_tick | w | memset 범위 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 27 | DeathMatchBattle | 0x150 | ep_follow_until | w | memset 범위 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 28 | DeathMatchBattle | 0x158 | idle_prev_pos | w | tuple<u64,u64>, memset 범위 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 29 | DeathMatchBattle | 0x168 | last_unseal_tick | w | memset 범위 끝(0x120+80=0x170) · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 30 | DeathMatchBattle | 0x170 | with_dive | w | bool · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 31 | DeathMatchBattle | 0x171 | help_called | w | bool · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 32 | DeathMatchBattle | 0x172 | dive_abandoned | w | bool · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 33 | DeathMatchBattle | 0x173 | dodge_commit | w | bool · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 34 | DeathMatchBattle | 0x174 | last_stand | w | bool · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 35 | DeathMatchBattle | 0x175 | had_ult_ready | w | bool · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 36 | DeathMatchBattle | 0x176 | stance | w | dienum DeathStance: 0=Commit 1=Reposition 2=HoldForSpawn (C 라이크 enum이라 태그=인덱스) · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 37 | DeathMatchBattle | 0x177 | tactic | w | dienum BattleTactic: 0=Standard 1=Frontline 2=BacklineDPS 3=SkillBurst 4=Peel 5=AllIn 6=Disengage · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 38 | DeathMatchBattle | 0x178 | scene | w | dienum DmScene: 0=Stand 1=Engage 2=Withdraw 3=Hold 4=Reposition 5=LastStand 6=Fallback · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 39 | DeathMatchBattle | 0x179 | scene_last_from | w | 직전 장면도 Stand 로 시작 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 40 | DeathMatchBattle | 0x17a | dive_tower | w | Option<TowerType> 1B 니치 — 0xff 가 None · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 41 | DeathMatchBattle | 0x17b | main_objective | w | **`Option<MainObjective>`(3B) 단일 Option**(tcx 정본). ⚠**이중 Option 으로 읽으면 니치가 한 겹 어긋나 재구현이 틀린다**. 태그 바이트(+0x17b)만 0xff 로 쓰고 0x17c/0x17d 는 안 씀 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 42 | DeathMatchBattle | 0x17e | lean_last_sign | w | i8 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) · tcx 정본 대조(5차 배치D: 5차 배치D 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |

**`consts` 상수 4건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 749 | 태그 | BattlePlanGoal 태그 0 = TryKill — 채팅 방송의 유일한 조건. 동시에 모든 Option 필드의 판별자 0 = None · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(5차 배치D: 5차 배치D 오라클 game==mine 236/236) | 2 |
| 1 | 3 | 750 | 태그 | Chat 태그 3 = Chat::Battle (DISCR_EXACT=3 확인). 아군에게 '전투 개시' 통보 · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(5차 배치D: 5차 배치D 오라클 game==mine 236/236) | 2 |
| 2 | 9223372036854775807 | 753 | 센티널 | i64::MAX — flee_die 초기값 센티널(도주사망 미기록) · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(5차 배치D: 5차 배치D 오라클 game==mine 236/236) | 2 |
| 3 | -1 | 753 | 센티널 | 0xff 바이트 — dive_tower / main_objective 의 Option None 니치 센티널 · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(5차 배치D: 5차 배치D 오라클 game==mine 236/236) | 2 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | 전투개시 채팅(Chat::Battle) 방송 조건 | death_battle.rs:749 (m05.ll 17709~17712, icmp eq %9, 0) | 0 | 지금은 goal 이 TryKill 일 때만 아군에게 Battle(target) 채팅을 뿌린다. 다른 태그(1=Support/2=Response/3=Avoid)까지 열면 지원·대응 상황에서도 아군이 '전투 신호'를 받아 합류 판단이 붙는다. 반대로 이 분기를 죽이면 데스매치 전투가 조용히 시작돼 아군 호응이 줄어든다 · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(5차 배치D: 5차 배치D 오라클 game==mine 236/236) | 2 |
| 1 | 채팅 페이로드의 두 번째 값 | death_battle.rs:750 (m05.ll 17754, store i64 0, ptr %21) | 0 | Chat::Battle(target, __1) 의 __1 이 항상 0. Chat 소비측이 이 값을 쓰면(예: 우선순위·시각) 여기서 조절 가능하나, 소비측을 안 봐서 효과 미확정 — unknown 참조 · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(5차 배치D: 5차 배치D 오라클 game==mine 236/236) | 2 |
| 2 | 초기 전투 태세(stance/tactic/scene) | death_battle.rs:753 (m05.ll 17773/17775/17800, store i8 0) | 0 | 전투 진입 시 항상 Commit / Standard / Stand 로 시작한다. 여기를 다른 태그로 바꾸면 첫 프레임부터 다른 태세(예: DmScene::Engage=1, BattleTactic::AllIn=5)로 시작해 초반 교전 성향이 통째로 바뀐다. 단 이후 틱에서 갱신될 수 있으니 지속성은 미확인 · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(5차 배치D: 5차 배치D 오라클 game==mine 236/236) | 2 |
| 3 | flee_die 센티널 | death_battle.rs:753 (m05.ll 17784, store i64 9223372036854775807) | 9223372036854775807 | '도주 중 사망 시각 없음'을 뜻하는 무한대. 유한한 값(예: start_tick)으로 낮추면 소비측이 '이미 도주사망함'으로 오인해 도주 관련 판정이 즉시 발화할 수 있다 — 소비측 미확인이라 위험 · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(5차 배치D: 5차 배치D 오라클 game==mine 236/236) | 2 |
| 4 | 오브젝트 스폰 예고 임계 | team_plan.rs:376·388 (m09.ll:38724·39066, mul .., 20) | tps*20 (20초) | 올리면 "모르가드/세르펜 곧 나옴" 채팅이 더 일찍 나가 팀 집결이 앞당겨진다 | 4 |

<details><summary>`callees` 피호출자 4건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | base_sub_goal | game_ai::plan_legacy::old::BattlePlanGoal::base_sub_goal | pub | fn(&game_ai::plan_legacy::old::BattlePlanGoal, usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\battle.rs:69 |
| 1 | tick | <game_core::Game as game_core::AbstractGame>::tick | pub | fn(&game_core::Game) -> usize | game-core\src\simulation\game.rs:1796 |
| 2 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 |
| 3 | tick | <game_core::ExpectedGame<'a> as game_core::AbstractGame>::tick | pub | fn(&game_core::ExpectedGame<'a/#0>) -> usize | game-core\src\simulation\expected_game.rs:53 |
</details>

⚠**미매칭 1개**: `grow_one`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m13.ll:16217, m13.ll:16237) · **형제 22개** (DeathMatchBattle)

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

**`open` 0건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**

(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)

<details><summary>`closed` 8건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | ~~base_sub_goal(m10.ll 29294~29997) 내부는 안 봄 — sub_goal 이 어떤 조건으로 정해지는지 미확정~~ → ★**축 확정 = 「적 우물 위험」**(2026-09-11 4차 배치D). `m10.ll:29294~29372`:     if game.get_entity_by_id(target).is_some_and(\|e\| is_ignored_well_enemy(version, player, e))        { End } else { Trace{focus} } `is_ignored_well_enemy`(**pub**, fight_model.rs:754) = 적팀 판정(`Entity+0x0` team) + `path_finder::is_enemy_well_danger(version, player, Entity+0x660, Entity+0x668)`. ⟹ 3차 관측 3개(타워→Trace / 가시성 6축 무관 / 스폰에서만 End)가 **전부 이 하나로 설명된다.** 남은 미탐색 = `is_enemy_well_danger` 내부(범위 명시). |  |  |
| 1 | Chat::Battle 의 두 번째 필드 __1 에 항상 0 을 넣는 의미 — Chat 소비측(채팅 처리기)을 안 봐서 이 0 이 '없음'인지 '기본값'인지 확정 불가. |  |  |
| 2 | p1 version 은 본문에서 단 한 번도 로드/비교되지 않는다(base_sub_goal 인자로만 전달). 이 함수 자체에는 버전 분기가 없다는 것까지만 확정. |  |  |
| 3 | p4 player(&PlayerState 2528B)도 본문에서 필드 로드가 전혀 없다 — base_sub_goal 로만 넘어간다. |  |  |
| 4 | 0x17c/0x17d 의 정체 = **확정**: `tcxdict --enum MainObjective` = 3B, 태그 +0x0(0..11), `Morgard`/`Serpen` 페이로드가 **`phase: ObjectPhase @+0x1` · `with_battle: bool @+0x2`**. None 니치는 태그바이트 0xff 하나 ⟹ **0x17c/0x17d 는 그 phase/with_battle 자리** |  |  |
| 5 | Vec::new 의 dangling 포인터 상수 8(=align)과 push 후 len=1 은 판정값이 아니라 Vec 내부 표현이라 constants 에 넣지 않았다. |  |  |
| 6 | AbstractGame vtable 은 game_core 소속이라 divtable.py 의 전역 상수 스캔으로만 슬롯 0x28=tick 을 확정했다(ExpectedGame 구현체 기준). 다른 AbstractGame 구현체가 런타임에 들어오면 같은 슬롯인지는 미검증. |  |  |
| 7 | `Chat::{Serpen,Morgard}Prepare.__1` — ★**"이름이 없다"로 닫힘(2026-09-11)**. `Chat` 은 game_ai 가 아니라 `game-core\src\simulation\state\player.rs` 정의이고 `SerpenPrepare` @765 / `MorgardPrepare` @776 **둘 다 튜플 variant**(rustc 진단: "is a tuple variant") ⟹ rmeta 필드 테이블의 이름이 `0`/`1` 뿐이라 **의도를 알려주는 이름이 애초에 존재할 수 없다**. 값 자체는 종전대로 **항상 리터럴 0 · 소비처 0건 = 죽은 슬롯**. 줄표 검산 L765 = 30자 = `  SerpenPrepare(usize, usize),`. 전문 = RE6-09-11_rmeta-SourceMap-rustc프로브-Span복원.md |  |  |
</details>

<details><summary>`history` 정정 이력 6건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | Chat::Battle 의 두 번째 필드 __1 에 항상 0 을 넣는 의미 | ★전제 정정 + 의미 확정. **enum+0x8 = 대상 엔티티 id 슬롯(틱 아님)**. death_battle::new(m05.ll:17745~17748)가 0 을 넣는 곳은 enum+0x8 이 아니라 **enum+0x10(Chat::Battle.__1)** 이고, enum+0x8 에는 **실제 대상 id(goal.__0)** 가 들어간다. SinglePlanBattle::new(m05.ll:26947)·BattlePlan::new(m10.ll:23214)도 동일 패턴. |  |
| 1 | Chat 의 enum+0x8 이 Battle계 이외 태그(24 SerpenPrepare, 33 MorgardPrepare)에서도 '엔티티 id' 인지 | ★**아니다 — 스폰까지 남은 "초"다.** 전문 = `_shared.Chat_스폰예고`. `__0 = udiv(next_respawn_tick - now, tps)`, 발화는 **남은 시간이 20초를 처음 밑도는 순간 1회**. `_gaibc`+`_gcbc` 전수 확인 결과 이 값을 id 로 쓰는 소비처는 없다. |  |
| 2 | `main_objective` = `Option<Option<MainObjective>>`(3B) / `flee_die` 센티널 = `usize::MAX` | ★**정정 2건(검증배치 D)** ① **타입 오류**: `+0x17b main_objective` 는 tcx 정본상 **`Option<MainObjective>`(3B) 단일 Option** 이다(`SinglePlanBattle+0x8d` 도 동일). **이중 Option 으로 읽으면 니치가 한 겹 어긋나 재구현 코드가 실제로 틀린다.**    ➕그 결과 `unknown`(0x17c/0x17d 정체)이 닫힌다: `MainObjective` 3B, 태그 `+0x0`(0..11), `Morgard`/`Serpen` 페이로드 = **`phase: ObjectPhase @+0x1` · `with_battle: bool @+0x2`**. None 니치는 태그바이트 `0xff` 하나이고 **`0x17c/0x17d` 는 그 phase/with_battle 자리**. ② **센티널 이름 오류(경미)**: `+0x118 flee_die` 의 저장값 `9223372036854775807 = 0x7FFF…FF` 는 **`i64::MAX`**(= `usize::MAX/2`)다. 같은 명세의 `logic` 블록은 `i64::MAX` 로 맞게 적혀 있어 **두 곳이 서로 모순**이었다. |  |
| 3 | `BattlePlanGoal::base_sub_goal` 내부 — 미탐색 | ★**전수 진리표 확보**(3차 배치D, pub). `Response`/`Avoid` → `RunAway` / `TryKill`·`Support` 는 산출 동일 / 타워·없는 id → **거리 무관 `Trace{focus:id}`** / 챔피언 → 거리에 따라 `Trace`/`End`. version·`__1` 무관. ★갈림은 **가시성이 아니다** — `visible_state`·`can_target`·`invisible_tick`·`hp=0`·`visible_map`·`exist_map` **6축을 실측 배제**했다. 남은 미탐색 = 그 판별 축. |  |
| 4 | `AbstractGame` vtable 슬롯 0x28=tick 이 `ExpectedGame` 기준이라 다른 구현체는 미검증(unknown[6]) | ★**해소 — 슬롯 인덱스는 impl 무관**(4차 배치C 가 10 에서 런타임 대조 4/4, `+0x28 tick` 포함). 정본 = 10 의 `resolved`. |  |
| 5 | 3차: `base_sub_goal` 의 갈림 축은 가시성이 아니다(6축 배제) — 축 자체는 미탐색 | ★**판정반전(보강) — 축은 「적 우물 위험」이다**(4차 배치D). 위 `unknown[0]` 참조. 3차의 배제 6건은 참이었지만 **축을 못 찾은 것이 아니라 IR 을 안 읽은 것**이었다. |  |
</details>

