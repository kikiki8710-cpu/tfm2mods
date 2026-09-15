---

### `246` SerpenCheckSubPlan::score — SerpenCheck(정글러 스틸 Lurk) 서브플랜의 후보 액션 점수: interaction_score + (Around 계열 대상이 살아있는 첫 세르펜이고 내 팀 시야에 안 보이면 +10), 그 외 0

| 항목 | 값 |
|---|---|
| id | `serpen_check__SerpenCheck__score` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12serpen_checkNtB2_18SerpenCheckSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\serpen_check.rs:123` |
| IR | `m14.ll` 31556~31675행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `e388c0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[246]/sig/tls/<키>`)**

없음 — LocalKey::with / @anon…call_once fn-포인터 참조 0 (31556~31675 전수)

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &SerpenCheckSubPlan(1B) — {move_check: bool@0x0} |  | 4 |
| 1 | 2 | version | usize |  | 4 |
| 2 | 3 | parameter | &ScoreParameter(5384B) |  | 4 |
| 3 | 4 | rnd | &mut StdRng(320B, align16) |  | 4 |
| 4 | 5 | player | &PlayerState(2528B) |  | 4 |
| 5 | 6 | data | &OperationData(24B) |  | 4 |
| 6 | 7 | action | &SmallActionPlay(184B) |  | 4 |
| 7 | 8 | debug | &mut DebugFrameData(224B) |  | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // serpen_check.rs:123 · self(move_check) 미사용
let base = interaction_score(version, rnd, player, data, parameter, action, debug);   // L124
let add = match action.get_action() {   // L125 · SmallActionPlay 태그 switch
    Around{target_id} /*Play idx 2 Around / 3 AroundHide / 10 LaneMinionPosition · payload+0x8*/ => {
        // L127: 살아있는 첫 세르펜 엔티티
        let game = data.cache.game;   // &dyn AbstractGame (+0x0 data, +0x8 vtable)
        let serpen: Option<&Entity> = game.get_game_mode() /*vtable+0x40 → GameMode{tag,ptr}*/
            .as_moba() /*game.rs:231 · tag≠0(Moba) 이면 unwrap_failed — reach: 접힌 분기(gamemode=0), 사장 호출부 NA*/ .unwrap()
            .jungle_runner.serpen.live_list /*MobaMode+0x1d0 ptr / +0x1d8 len · Vec<usize>*/
            .get(0) /*len==0 → None*/
            .and_then(|id| game.get_entity_by_id(*id) /*vtable+0x1f0*/);
        // L128
        if let Some(serpen) = serpen {
            if serpen.id /*+0x5c0*/ == target_id && !game.is_visible(player.info.team /*+0x930*/, target_id) /*vtable+0xf8*/ { 10 } else { 0 }
        } else { 0 }
    }
    _ => 0,   // Attack/Skill/Skill2 포함 전부 0 — 형제 플랜과 달리 공격 액션 가산이 전혀 없음
};
return base + add;   // L124 합산 · L141

★형제 대조(복붙 의심): 형제 4개(LineSafe/LineWait/Jungle/AttackNexus)는 champ 조회+Attack/Skill/Skill2 arm+calculate_(jungle_)action_score 골격인데 SerpenCheck 는 그 골격이 통째로 없고 Around arm 만 있다(_docs: 「commit=false(Lurk): 시야 확인 주기 기반 대기 … stale 이면 잠깐 캠프로 접근해 시야 갱신」과 부합 — 공격은 SerpenHunt 몫). is_visible 인자 순서 (team, id) 는 _gcbc g08.ll:200059 DI(self, team, id) 로 확인. `serpen.id == target_id` 뒤 `is_visible(team, target_id)` 는 `&&` 단락 — IR 상 id 비교가 먼저(블록 43→47).
rnd gen_range 사이트: 0(interaction_score 1회 전달).
version 분기 0 · debug 직접 쓰기 0 · self 읽기 0.
★JT tail-jump 디스패처(exe 0xe388c0) 주의: 이 IR 은 SDK 판이고 exe 측 진입부는 별도 확인 필요 — 명세 범위 밖.
```

**`mem` 메모리 접근 13건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | SmallActionPlay | 0xb1 | tag(u8) | r | L125(small_action.rs:309 get_action 인라인) · idx 2(Around)/3(AroundHide)/10(LaneMinionPosition) → Around arm, 나머지 전부 0 (Attack/Skill/Skill2 arm 없음!) | 4 | OK |
| 1 | SmallActionPlay | 0x8 | payload.target | r | L125(DI line 0) · SmallActionAround/AroundHide/LaneMinionPosition 의 target 필드 → target_id | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | L127 | 4 | OK |
| 3 | AbstractGameWithCache | 0x0 | game.data_ptr | r | L127 · get_game_mode / get_entity_by_id / is_visible 의 self | 4 | OK |
| 4 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | L127/L128 · 슬롯 +0x40 get_game_mode · +0x1f0 get_entity_by_id · +0xf8 is_visible (divtable ExpectedGame 기준) | 3 | OK |
| 5 | AbstractGame vtable | 0x40 | get_game_mode(&self) -> GameMode(16B: {i64 tag, ptr}) | r | L127 · 반환값 태그 0=Moba(payload=&MobaMode) · 1=SingleLane · 2=DeathMatch | 4 | 확인불가(vtable 슬롯) |
| 6 | GameMode(반환값) | 0x0 | tag | r | L127(game.rs:231 as_moba 인라인) · ≠0 이면 unwrap_failed — reach(gamemode=0) 에서 접힘 @17 %28→1, 사장 호출부 1(NA) | 4 | OK |
| 7 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | L127(vec as_slice/get(0) 인라인) · 0 이면 serpen None → 가산 0 | 4 | OK |
| 8 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.buf.ptr (Vec<usize>) | r | L127 · [0] 원소 = 첫 살아있는 세르펜 entity id | 4 | OK |
| 9 | AbstractGame vtable | 0x1f0 | get_entity_by_id(&self, id:usize)->Option<&Entity> | r | L127(option.rs:1543 and_then 클로저 closure_env$0 인라인 · f 캡처 = game 팻포인터) · null → 가산 0 | 4 | 확인불가(vtable 슬롯) |
| 10 | Entity(serpen) | 0x5c0 | id (usize) | r | L128 · serpen.id == target_id | 4 | OK |
| 11 | PlayerState | 0x930 | info.team | r | L128 · is_visible 의 team 인자 | 4 | OK |
| 12 | AbstractGame vtable | 0xf8 | is_visible(&self, team:usize, id:usize)->bool | r | L128 · (player.info.team, target_id) · true→0, false→10 | 4 | 확인불가(vtable 슬롯) |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 10 | 125 | 센티널 | SmallActionPlay 니치 구멍 assume(tag≠10). 판정 아님. ★같은 값 10 이 L128 가산으로도 쓰임(별 항목) | 4 |
| 1 | 3 | 125 | 태그 | 태그→idx 환산(tag-3). 판정 아님 | 4 |
| 2 | 7 | 125 | 태그 | tag≤2 → idx 7(AroundPosition). 판정 아님 | 4 |
| 3 | 0 | 127 | 태그 | GameMode::Moba 태그(0) — as_moba().unwrap() 검사. 그리고 live_list.len == 0 검사(get(0) None) | 4 |
| 4 | 10 | 128 | 태그 | Around 대상 == 첫 살아있는 세르펜 && !is_visible(내 팀, 그 id) 일 때 가산 10 (시야 없는 세르펜 쪽으로 가서 확인하도록) | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 미시야 세르펜 접근 가산 | serpen_check.rs:128 | 10 | 올리면 Lurk 중 세르펜 시야가 없을 때 세르펜 주변 대기(Around) 후보가 다른 대기 후보를 더 강하게 이김 → 더 적극적으로 시야 확인. 0 이면 시야 확인 유인이 사라져 안전 대기만 남음 | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

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
| 10 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 15 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 16 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
</details>

⚠**미매칭 1개**: `calculate_`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:37294) · **형제 7개** (SerpenCheckSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:8 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan) -> game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:8 | True | fn() -> game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan |
| 2 | <game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:8 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:14 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:106 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 5 | game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:123 | False | fn(&game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 6 | game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:143 | True | fn(&mut game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan) |

**`open` 2건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | as_moba().unwrap() 이 소스에서 unwrap 인지 expect 인지 — unwrap_failed(@anon.143) 만 보임. Moba 외 모드에서 이 플랜이 오는지는 이 함수 밖(reach 는 gamemode=0 봉인). | 4 |  |
| 1 | 미탐색 | self.move_check 를 안 읽는 이유 — 소스 부재(action_candidates 쪽에서 쓰는 것으로 추정 · r16 범위). | 5 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L127 의 소스 표기 — `get(0)`/`first()`/`iter().next()` 중 무엇인지(전부 len==0 검사+ptr[0] 로 접힘). 동작 확정. | 4 | 사실 서술 |
| 1 | vtable 슬롯 이름(0x40 get_game_mode · 0xf8 is_visible · 0x1f0 get_entity_by_id)은 ExpectedGame 정적 vtable(divtable) 기준 — 런타임 구현체 미확정. | 3 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

