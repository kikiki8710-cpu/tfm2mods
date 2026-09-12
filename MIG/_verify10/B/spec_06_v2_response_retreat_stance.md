---

### `06` v2_response_retreat_stance — 후퇴 태세: 가장 가까운 적을 찾아 죽기까지 1초 넘게 버티면 KitingBack, 아니면 RunAway

| 항목 | 값 |
|---|---|
| id | `engage__v2_response_retreat_stance` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler6engageNtB4_17LegacyPlanHandler26v2_response_retreat_stance` |
| 소스 | `game-ai\src\plan_legacy\handler\engage.rs:13` |
| IR | `m13.ll` 45212~45626행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v2_response_retreat_stance` · **in:game_ai** |
| 계층 | 플랜 핸들러 |
| exe | `e657a0` (engage) · 877바이트 · 190명령 |
| 라운드 | 기준 `r4` · 통과 4회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::BattleSubPlanGoal
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 1 | self |  | IR 인자로 안 넘어옴(#dbg_value(ptr poison)) — 본문에서 self 필드를 하나도 안 읽어 최적화로 소거됨 |
| 1 | 2 | version |  | IR %0. AI 버전 게이트. version<2면 즉시 RunAway. 그대로 클로저 캡처(&version)와 check_kill_die_tick 으로 전달 |
| 2 | 3 | rnd |  | IR %1. 이 본문에선 안 씀 — check_kill_die_tick 에 그대로 넘김 |
| 3 | 4 | player |  | IR %2. team/position 을 읽고, 클로저 캡처 + check_kill_die_tick 의 judger 로 전달 |
| 4 | 5 | data |  | IR %3. {cache, context, blackboard} |
| 5 | 6 | debug |  | IR %4. 이 본문에선 안 씀 — check_kill_die_tick 에 그대로 넘김 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v2_response_retreat_stance(self, version, rnd, player, data, debug) -> BattleSubPlanGoal

L14 if version < 2 { return RunAway } // IR %14: icmp ult %0, 2

L17 team = player.info.team // +0x930, bounds-check team<2
 pos = player.info.position.as_index() // +0x9c0, range [0,5)
 champ = data.cache.player_champion[team][pos] // cache+0x1e0, [5 x ptr] stride 40
 if champ == null { return RunAway } // Option<&Entity> 니치 = null

L20 enemies = data.cache.iter_champions(1 - team) // cache.player_champion[1-team] 5칸을
 // filter_map(|c| *c) 로 null 제거 (인라인됨)
L21..L23 pred(e) = // 클로저 = closure_env$0, 본체는 m12.ll:41916~41998
 data.blackboard[1 - team].is_recent_visible(data.cache.game, player, e) // L21
 && !is_ignored_well_enemy(version, player, c) // ★호출부에 `fight_model::` 접두 없음(L22=53자 ±0) // L22
 && champ.distance_sq(e) < 40000000001 // L23 (= <= 200000^2)
 // 세 조건은 && 단축평가. 하나라도 false 면 그 적은 제외.

L24 nearest = enemies.filter(pred).min_by_key(|e| champ.distance_sq(e))
 // IR 은 이걸 둘로 쪼갬:
 // (a) 담당 본문에서 5칸을 수동 언롤(%30,%52,%58,%64,%70)하며 pred 가 true 인 **첫 원소**를 찾음.
 // 하나도 없으면 nearest=None → L25 로.
 // (b) 그 첫 원소의 키(dx^2+dy^2, Entity::distance_sq)를 시드로 나머지를 fold.
 // fold 본체 = m12.ll:12391~12638 (같은 pred 를 다시 평가하고 키 최소를 고름).
 // dx = x.abs_diff(other.x) (icmp+select 로 접힘), key = dx*dx + dy*dy.

L25 if nearest == None { return RunAway } // IR %103

L26/L30 near_enemies: Vec<&Entity, &Bump> =
 data.cache.iter_champions(1 - team)
 .filter(|e| is_recent_visible && !is_ignored_well_enemy && champ.distance_sq(e) < 22500000001)
 .pipe(|it| bumpalo::collections::Vec::from_iter_in(it, data.context.pool)) // ★**2인자 `from_iter_in`** 이다 (심볼 실측 m13.ll:45590 + L26/27/30 줄길이 ±0)
 // 클로저 = closure_env$2, 본체 m12.ll:42001~42083. L20 의 pred 와 **완전히 같고 거리 임계만 다름**
 // (200000^2 → 150000^2). 수집 루프 본체는 m01.ll:33865~34040(bumpalo from_iter_in).

L31/L32 die = fight_check::check_kill_die_tick(
 version, rnd, data,
 judger = player, focus = champ,
 enemy = near_enemies,
 towers = Vec::new_in(data.context.pool), // L32 — 항상 **빈 Vec**
 debug)

L33 if die > data.context.setting.tick_per_second {
L36 return KitingBack { focus: nearest.id } // tag 3, payload = nearest+0x5c0
 }
L38 return RunAway // tag 4

// 이 함수는 게임 구조체에 아무것도 쓰지 않는다. store 는 전부 스택 로컬
// (클로저 캡처 묶음 %9/%11, 빈 Vec %10)이며, 부수효과는 rnd/debug 를
// check_kill_die_tick 에 넘기는 것뿐이다.

// 의미 요약: '적이 근처에 있는데 내가 1초 이상 버틸 수 있으면 카이팅하며 물러나고(KitingBack),
// 곧 죽을 것 같거나 볼 수 있는 적이 아예 없으면 그냥 도망(RunAway)'.
```

**`mem` 메모리 접근 13건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 1 | OperationData | 0x8 | context | r | &GameContext · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — 클로저 캡처로만 넘기고 본문에선 역참조 안 함 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 3 | PlayerState | 0x930 | info.team | r | usize. 배열 인덱스로 쓰기 전 len=2 bounds-check. 적 팀 인덱스 = 1-team · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 4 | PlayerState | 0x9c0 | info.position | r | Position(u32, !range [0,5)) → Position::as_index() (entity.rs:580) 인라인. player_champion[team] 안의 슬롯 인덱스 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 5 | AbstractGameWithCache | 0x0 | game | r | &dyn AbstractGame 팻포인터 16B — +0x0 데이터ptr, +0x8 vtable(816B=슬롯100+size/align, divtable 로 확인). 둘 다 클로저가 Blackboard::is_recent_visible 로 그대로 전달 | 3 |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] (80B). [team][position] = 내 챔피언, [1-team] 전체 5칸 = 적 챔피언 배열(이터레이션 대상) · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 7 | Entity | 0x660 | x | r | min_by_key 시드 원소의 거리 계산(Entity::distance_sq, entity.rs:2157) — 나머지 원소는 범위 밖 fold(m12.ll)에서 같은 식으로 읽음 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 8 | Entity | 0x668 | y | r | 위와 동일 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 9 | Entity | 0x5c0 | id | r | 최종 승자(nearest)의 id → KitingBack{focus} 페이로드. src_line 36 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 10 | GameContext | 0x0 | pool | r | &bumpalo::Bump — near_enemies Vec 과 빈 towers Vec 의 할당자 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 11 | GameContext | 0x8 | setting | r | &GameSetting · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 12 | GameSetting | 0x12f8 | tick_per_second | r | usize. die 틱과 비교하는 임계(=1초). src_line 33 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |

**`consts` 상수 4건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 14 | 임계 | AI 버전 비교 임계(게이트) — version<2 면 본문 전체를 건너뛰고 RunAway. (같은 리터럴 2 가 team 배열 bounds-check 길이 `icmp ult team, 2` 로도 쓰인다 — **이 행 자체는 version 비교 임계**이지 팀 배열 첨자가 아니다) | 4 |
| 1 | 1 | 20 | 인덱스 | 적 팀 인덱스 = 1 - player.info.team (2팀 고정 전제) | 4 |
| 2 | 3 | 36 | 태그 | BattleSubPlanGoal::KitingBack 의 태그값(DISCR_EXACT=3, variant 인덱스와 동일) · tcx 정본 대조( tcxdict --enum BattleSubPlanGoal **tcx 정본**: KitingBack 메모리태그 3, 페이로드 focus@+0x8) | 3 |
| 3 | 4 | 38 | 태그 | BattleSubPlanGoal::RunAway 의 태그값(DISCR_EXACT=4). 모든 조기탈출 경로의 기본 반환 · tcx 정본 대조( tcxdict --enum BattleSubPlanGoal **tcx 정본**: RunAway 메모리태그 4, fieldless) | 3 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | AI 버전 게이트 | engage.rs:14 (m13.ll:45232) | 2 | version<2 면 이 함수는 무조건 RunAway. 낮추면 구버전 AI에도 KitingBack 판정이 열리고, 올리면 이 함수가 사실상 죽는다(항상 RunAway) | 4 |
| 1 | min_by_key 대상 적 탐색 반경(제곱) | engage.rs:23 — 클로저 본체는 m12.ll:41992 (담당 줄범위 밖) | 40000000001 | 200000^2+1. `dist_sq < 이 값`이므로 실질 반경 200000(=6.25셀). 올리면 더 먼 적까지 '가장 가까운 적' 후보가 되어 RunAway 대신 KitingBack 이 나올 여지가 커지고, 내리면 적을 못 찾아 RunAway 로 떨어진다 | 3 |
| 2 | near_enemies(사망시점 계산에 넣을 적) 반경(제곱) | engage.rs:30 — 클로저 본체는 m12.ll:42077 (담당 줄범위 밖) | 22500000001 | 150000^2+1, 실질 반경 150000(=4.6875셀). 올리면 더 많은 적이 위협 계산에 들어가 die 가 작아지고 → RunAway 쪽으로, 내리면 die 가 커져 KitingBack 쪽으로 기운다 | 3 |
| 3 | 생존 시간 임계 | engage.rs:33 — GameSetting.tick_per_second(+0x12f8) | tick_per_second (1초) | die > 1초면 KitingBack. 이 비교값을 키우면 더 오래 버틸 때만 카이팅하고 대부분 RunAway, 줄이면 거의 항상 KitingBack | 4 |
| 4 | towers 인자 | engage.rs:32 (m13.ll:45596~45600) | 빈 Vec (cap=0,len=0) | check_kill_die_tick 의 towers 를 항상 비워 넘긴다 = 이 판정은 **타워 피해를 사망시점 계산에 넣지 않는다**. 여기에 실제 타워 목록을 넣으면 타워 아래 후퇴 판정이 보수적으로 바뀐다 | 4 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 |
| 2 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 |
| 4 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 |
| 5 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 |
| 6 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 |
| 7 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 |
| 8 | v2_response_retreat_stance | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v2_response_retreat_stance | in:game_ai | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\handler\engage.rs:13 |
</details>

**호출처 2곳** (m13.ll:38079, m13.ll:38761) · **형제 41개** (LegacyPlanHandler)

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

<details><summary>`closed` 7건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | 거리 임계 40000000001 / 22500000001 은 `constants` 에 못 넣었다 — 두 값 모두 담당 줄범위(m13.ll 45212~45626) 밖의 클로저 본체(m12.ll:41992, m12.ll:42077)에 있어 QC C1 이 반려한다. SPEC_GUIDE §'담당 범위 밖 상수' 규칙대로 knobs 로만 실었다. |  |  |
| 1 | is_recent_visible / is_ignored_well_enemy 도 같은 이유로 `calls` 에 없다 — 담당 본문에는 call_mut 심만 있고 실제 call 은 m12.ll 의 클로저 안에 있다. |  |  |
| 2 | self(&LegacyPlanHandler)의 용도 — IR 인자로 아예 안 넘어오고 #dbg_value 도 poison 이라, 소스에 self 를 쓰는 코드가 있었는지 없었는지 IR만으로는 확정 불가(관측 사실: 본문·클로저 어디서도 LegacyPlanHandler 필드를 안 읽는다). |  |  |
| 3 | blackboard 인덱스가 1-team(적 팀)인 이유 — 관측 사실은 확정(closure %11 = sub 1, player.info.team). Blackboard 가 last_visible/last_reveal_tick 등 [5]칸 배열을 갖는 것으로 보아 'blackboard[t] = 팀 t 챔피언 5명의 피관측 기록'으로 읽는 게 자연스럽지만, is_recent_visible 본체를 안 봐서 **추정**이다. |  |  |
| 4 | min_by_key 동점 처리 — fold(m12.ll:12611의 compare + `icmp slt i8 %91, 1`)를 끝까지 따라가지 않았다. Rust min_by_key 표준 의미(첫 최소 유지)로 적었으나 IR 수준 확정은 안 했다. |  |  |
| 5 | L20 과 L26 의 술어가 소스에서 정말 두 개의 별도 클로저인지, 아니면 임계만 파라미터로 받는 헬퍼가 두 번 인라인된 것인지 — DWARF 는 closure_env$0 / closure_env$2 로 서로 다른 클로저라고 말한다(closure_env$1 = min_by_key 키 클로저, engage.rs:24). 다만 두 본체가 명령 단위로 동일해 소스 형태는 미확정. |  |  |
| 6 | 반환 페이로드가 undef 인 경로(RunAway)에서 상위 호출자가 payload 워드를 어떻게 다루는지는 안 봤다. |  |  |
</details>

<details><summary>`history` 정정 이력 6건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | 거리 임계 40000000001 / 22500000001 이 각각 어느 술어의 것인지 | ★확정. closure$0(engage.rs:21, 판정 line 23) = 40000000001 = 200000²+1 @m12.ll:41992. closure$2(engage.rs:28, 판정 line 30) = 22500000001 = 150000²+1 @m12.ll:42077. icmp ult 라 dist <= r 이면 통과. |  |
| 1 | 두 클로저가 정말 다른 술어인가 | ★아니다. 반경 상수만 다르고 술어 구조가 완전히 동일하다: ①is_recent_visible(blackboard[1-team],...) false 면 탈락 ②is_ignored_well_enemy(version,player,c) true 면 탈락 ③dist_sq < r²+1. DWARF 상으로는 서로 다른 소스 클로저(closure$0/closure$2)가 맞다. |  |
| 2 | min_by_key 동점 처리 / 키 계산식 | 키 = dx*dx + dy*dy (dx=\|c.x-R.x\|, dy=\|c.y-R.y\|) @m12.ll:12595~12597. 키 클로저 = closure$1(engage.rs:24). ★필터의 기준 엔티티와 키의 기준 엔티티가 동일 포인터(본체 m13.ll:45316/45318 에서 같은 %28 을 저장) ⟹ '200000 이내 중 자기에게 가장 가까운 것'. fold 에 인라인된 것은 200000 쪽(m12.ll:12557). |  |
| 3 | 반환 페이로드가 undef 인 경로(RunAway)에서 상위 호출자가 payload 워드를 어떻게 다루는지 | ★확정 · 문제 없음. `dienum BattleSubPlanGoal 4` → **`RunAway` 는 fieldless variant** 이므로 페이로드 워드가 애초에 정의되지 않은 변형이고 `undef` 가 정상이다. 호출부 2곳(`handle_interact_battle` _gaibc/m13.ll:38079·38761)은 `BattlePlan+0x58/+0x60` 에 **16B 통째 이동**만 하고 해석하지 않는다. ⟹ 재구현 시 `RunAway`(페이로드 없음)로 만들면 되고 `undef` 를 따라 만들 필요 없다. |  |
| 4 | closure2 L30 의 잔차 +23자(1차 미탐색) | ★해소 — `, data.context.pool);` 가 **정확히 21자** + 들여쓰기 2 = 23. `from_iter_in` 2인자 형태였기 때문이다(2026-09-11 2차배치B). L13~L32 전 구간 잔차 0. |  |
| 5 | 오라클로 실행 검증이 가능한가 | ★**차단(실측)**. `v: in:game_ai` 라 `error[E0624]: method ... is private` (`_verify2\B\B_o4_blocked.rs`, 2026-09-11). ~~`LegacyPlanHandler` 자체는 pub 이나 pub 메서드는 `handle_chat`/`get_small_action` 2개뿐이라 이 함수로 가는 pub 경로가 없다~~ → ★**거짓이었다(6차 배치B 판정반전)**. ① 같은 JSON 의 `siblings` 자동표가 이미 **pub 메서드 12개**를 싣고 있었다(clone·fmt·new·v3_fall_back_to_passive·eo_cover_picks·eo_serpen_punish_issues·subplan_is_recall·team_objective_code·update_on_dead·**update**·get_small_action·handle_chat). ② IR 호출사슬이 있다: `LegacyPlanHandler::update`(pub, define m13.ll:14467) → `handle_interact_battle`(호출 m13.ll:20242) → `v2_response_retreat_stance`(호출 m13.ll:38079·38761). ③ 6차에 `update` 를 **실제로 실행**했다(B6_o2.tsv — 05 는 이 경로로 전량 실행검증됨). #H 에서 plan=Battle 로 돌리면 BattlePlan+0x58 의 sub_goal = **4(RunAway)** 가 관측된다. ⟹ **남은 미탐색** = 그 RunAway 가 06 의 반환인지 가르는 것(범위 = BattlePlan 상태를 주입해 KitingBack 을 유도한 뒤 경계 대조. 기본 챔피언에선 `die == tick_per_second` 라 `die > tps` 가 항상 거짓이므로 AttackEffect 조립이 선행돼야 한다). |  |
</details>

