---

### `242` StealSubPlan::score — 스틸 서브플랜 액션 점수: 대상 오브젝트(에픽/세르펜 live_list[0]) 를 때리는 공격/스킬/궁 1000 · 추적 100 · 자리잡기류 90 · 다른 대상 -99999 · 그 외 0

| 항목 | 값 |
|---|---|
| id | `steal__Steal__score` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan5stealNtB2_12StealSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\steal.rs:136` |
| IR | `m02.ll` 27428~27589행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::StealSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `cbbca0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[242]/sig/tls/<키>`)**

없음 — 본문에 `@anon.* = constant ptr @<KEY…call_once>` 참조 0 · LocalKey::with 0

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &StealSubPlan (16B) | IR readonly · +0x8 target(Option<StealTarget> 니치 1B: 0=Epic 1=Serpen 2=None) 만 읽음 · last_vision_tick(+0)·commit(+0x9) 은 읽지 않음 | 4 |
| 1 | 2 | _version | usize | 미사용(DI 이름 `_version`) | 4 |
| 2 | 3 | _parameter | &ScoreParameter (5384B) | IR readonly · 미사용(DI `_parameter`) | 4 |
| 3 | 4 | _rnd | &mut StdRng (320B) | IR `readnone` · 미사용 · gen_range 사이트 0 | 4 |
| 4 | 5 | _player | &PlayerState (2528B) | IR readonly · 미사용(DI `_player`) | 4 |
| 5 | 6 | data | &OperationData (24B) | IR readonly · cache.game 만 사용 | 4 |
| 6 | 7 | action | &SmallActionPlay (184B) | IR readonly · 태그 +0xb1, Trace.target +0x60, Attack/Skill/Skill2/Ult.target +0x8 | 4 |
| 7 | 8 | _debug | &mut DebugFrameData (224B) | IR `readnone` · 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn score(&self, _version, _parameter, _rnd, _player, data, action, _debug) -> i64   [steal.rs:136]
  target: &Entity = match self.target {                                                  [L137 · self+0x8]
    None(2)      => return 0                                                             [m02.ll:27446 → 27587 phi 0]
    Some(Epic=0) => { moba = data.cache.game.get_game_mode().as_moba().unwrap()  [L27 인라인(steal.rs:27 — 이 함수가 아니라 steal.rs 상단 헬퍼 줄이 inlinedAt 로 찍힘)]
                      moba.jungle_runner.epic.live_list.first() }                        [+0x198 · len==0 → return 0]
    Some(Serpen=1)=> { … .jungle_runner.serpen.live_list.first() }                       [L28 · +0x1c8]
  } → id → data.cache.game.get_entity_by_id(id) — None 이면 return 0                      [L30 · m02.ll:27521~27526]
  match action.get_action() {                                                             [L139 · small_action.rs:309 인라인]
    SmallAction::AroundPosition{..} /*SmallActionPlay AroundRegion|AroundPosition|AroundPositionBush|AroundBush*/ => 90   [m02.ll:27561~27562]
    SmallAction::Attack(t)|Skill(t)|Skill2(t)|Ult(t) => if t == target.id { 1000 } else { -99999 }   [L144 · 27576~27583 · t = action+0x8]
    SmallAction::Trace(t)                             => if t == target.id { 100 } else { -99999 }    [L147 · 27563~27573 · t = action+0x60]
    _ /*RunAway·Recall·Around·AroundHide·AroundRunAway·Positioning·LaneMinionPosition·Stop*/ => 0
  }                                                                                                    [L152 ret · 27588]

분기 순서: target 해석(3 단계 조기 0 반환) → 액션 종류. gen_range 0 · self 쓰기 0 · commit/last_vision_tick 미참조(commit 은 action_candidates 가 후보군 구성에 쓴다). reach.txt: unwrap_failed 2 사이트(27501, 27505 = get_game_mode 가 Moba 가 아닐 때)는 gamemode=0 접힘으로 사장(NA).
```

**`mem` 메모리 접근 12건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | StealSubPlan | 0x8 | target@tag | r | Option<StealTarget> 니치 1B: 0=Epic → steal.rs:27 가지 · 1=Serpen → :28 가지 · 2=None → 0 반환 (m02.ll:27437~27448) | 4 | OK |
| 1 | OperationData | 0x0 | cache | r | m02.ll:27439 | 4 | OK |
| 2 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 워드 (m02.ll:27453, 27468, 27514) | 4 | OK |
| 3 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x40 = get_game_mode(divtable AbstractGame 0x40) · vtable+0x1f0 = get_entity_by_id (m02.ll:27456~27458, 27522~27524) | 3 | OK |
| 4 | GameMode(get_game_mode 반환 {i64,ptr}) | 0x0 | 판별자 | r | == 0 이면 MobaMode 참조(ptr 워드) · 아니면 unwrap_failed(mode.rs:231 인라인 as_moba 류 → option.rs:1013) (m02.ll:27459~27463, 27474~27478) | 4 | OK |
| 5 | MobaMode | 0x198 | jungle_runner.epic | r | Epic 가지 · +0x8 live_list.ptr(0x1a0) · +0x10 live_list.len(0x1a8) (m02.ll:27482 phi 408 → 27486~27492, 27509~27510) | 4 | OK |
| 6 | MobaMode | 0x1c8 | jungle_runner.serpen | r | Serpen 가지 · +0x8 live_list.ptr(0x1d0) · +0x10 live_list.len(0x1d8) (phi 456) | 4 | OK |
| 7 | live_list[0] | 0x0 | 엔티티 id(usize) | r | `first()` — len==0 이면 0 반환 (m02.ll:27497~27498), 아니면 ptr[0] (27521) | 4 | 확인불가(tcx 사전에 타입 없음) |
| 8 | Entity | 0x5c0 | id | r | 대상 오브젝트 엔티티의 id 와 액션의 target 비교 (m02.ll:27570~27572, 27580~27582) | 4 | OK |
| 9 | SmallActionPlay | 0xb1 | 태그 | r | get_action() 인라인(small_action.rs:309) — 논리 idx = tag>2 ? tag-3 : 7 (m02.ll:27531~27556) | 4 | OK |
| 10 | SmallActionTrace | 0x60 | target | r | Trace 가지(idx 11) · small_action.rs:321 → trace.rs:404 인라인 (m02.ll:27567~27568) | 4 | OK |
| 11 | SmallActionAttack/Skill/Skill2/Ult | 0x8 | target | r | idx 12~15 (m02.ll:27577~27578) | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 137 | 태그 | target 없음(None) / 오브젝트 live_list 비어 있음 / get_entity_by_id None / 매치 안 되는 액션 → 점수 0 (phi m02.ll:27587) | 4 |
| 1 | 90 | 139 | 태그 | get_action() 이 SmallAction::AroundPosition 으로 접히는 variant(SmallActionPlay AroundRegion4·AroundPosition7·AroundPositionBush8·AroundBush9 · small_action.rs:314) → 90 (m02.ll:27543~27548 → %62) | 4 |
| 2 | 1000 | 144 | 산출값 | Attack/Skill/Skill2/Ult(idx 12~15) 의 target == 대상 오브젝트 id → 1000 (m02.ll:27583) | 4 |
| 3 | 100 | 147 | 산출값 | Trace(idx 11) 의 target == 대상 오브젝트 id → 100 (m02.ll:27573) | 4 |
| 4 | -99999 | 144 | 산출값 | 공격류·추적의 target 이 대상 오브젝트가 아니면 -99999 (사실상 배제) (m02.ll:27573, 27583) | 4 |
| 5 | 2 | 137 | 센티널 | Option<StealTarget> 니치 None 태그값 (m02.ll:27446) | 4 |
| 6 | 408 | 27 | 산출값 | MobaMode.jungle_runner.epic 오프셋 0x198 (Epic 가지 · phi m02.ll:27482) — 오프셋 상수라 reads 참조 | 4 |
| 7 | 456 | 28 | 산출값 | MobaMode.jungle_runner.serpen 오프셋 0x1c8 (Serpen 가지) | 4 |
| 8 | 64 | 27 | 미상 | AbstractGame vtable 슬롯 0x40 = get_game_mode (m02.ll:27456, 27471) | 4 |
| 9 | 496 | 30 | 미상 | AbstractGame vtable 슬롯 0x1f0 = get_entity_by_id (m02.ll:27522) | 4 |
| 10 | 10 | 139 | 센티널 | SmallActionPlay 니치 태그 10 불가 assume (m02.ll:27533) | 4 |
| 11 | 7 | 139 | 태그 | 태그 ≤2 → 논리 idx 7(AroundPosition 암묵) (m02.ll:27537) | 4 |
| 12 | -3 | 139 | 태그 | 태그→논리 idx `tag-3` (m02.ll:27535) | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 대상 오브젝트 직접 공격 점수 | steal.rs:144 | 1000 | 내려서 100 미만이면 추적(Trace)보다 밀려 공격 대신 접근만 하게 된다 · 자리잡기 90 보다 커야 스틸 시도가 성립 | 4 | 기존 |
| 1 | 대상 추적 점수 | steal.rs:147 | 100 | 자리잡기(90)보다 살짝 높아 시야가 있으면 추적을 고른다 · 90 아래로 내리면 접근 대신 대기 위치 선호 | 4 | 기존 |
| 2 | 자리잡기(AroundPosition류) 점수 | steal.rs:139~142(줄 표기 불가 · small_action.rs:314 접힘) | 90 | 올려 100 을 넘기면 Trace 보다 대기 위치를 선호(Lurk 가 강해짐) | 4 | 기존 |
| 3 | 오표적 배제값 | steal.rs:144/147 | -99999 | 대상 외 엔티티를 때리는 공격/추적을 사실상 금지 — 올리면 스틸 중 다른 적을 공격할 수 있게 된다 | 4 | 기존 |

<details><summary>`callees` 피호출자 13건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 2 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 3 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 4 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 11 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 12 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
</details>

⚠**미매칭 1개**: `first`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:37322) · **형제 9개** (StealSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::StealSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:12 | True | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan) -> game_ai::plan_legacy::sub_plan::StealSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::StealSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:12 | True | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::sub_plan::StealSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:12 | True | fn() -> game_ai::plan_legacy::sub_plan::StealSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::StealSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:21 | True | fn(game_core::StealTarget, bool) -> game_ai::plan_legacy::sub_plan::StealSubPlan |
| 4 | game_ai::plan_legacy::sub_plan::StealSubPlan::get_target | in:game_ai::plan_legacy::sub_plan::steal | game-ai\src\plan_legacy\sub_plan\steal.rs:25 | False | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, &game_core::OperationData) -> std::option::Option<&game_core::Entity> |
| 5 | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::StealSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:122 | True | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 8 | game_ai::plan_legacy::sub_plan::StealSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:154 | True | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, game_ai::plan_legacy::sub_plan::StealSubPlan) |

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 90 을 반환하는 arm 의 정확한 소스 줄 — phi 로 접혀 `!dbg` 가 없다(L0). 139~143 사이로 추정(140~142 중 하나) · 줄 길이 산술(rmeta_srcmap) 미실시 | 3 |  |
| 1 | 미탐색 | get_game_mode() 반환 타입(`{i64, ptr}`)의 정확한 열거형 이름과 `as_moba` 헬퍼 이름(mode.rs?:231 인라인) — game_core 쪽 · 판정에는 무관(판별자 0 = Moba 확정) | 4 |  |
| 2 | 표기 불가 | match 의 `_ => 0` arm 이 명시적 arm 인지 — 표기 불가(외연 동일) | 4 |  |
| 3 | 미탐색 | [D] ai_adjust judge steal_score DIFF=0 7판(09-06) 이라 지시대로 명세만 작성 — 재현 검증 대상 아님 | 1 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

