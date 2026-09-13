---

### `72` SerpenHuntAndPokePlan::is_end — 세르펜 헌트/포크 플랜 종료 판정 — 목표가 Serpen 이 아니거나, Setup 국면에서 해제/적 부재/적 원거리, 또는 세르펜이 죽고 리스폰이 15초 넘게 남으면 종료

| 항목 | 값 |
|---|---|
| id | `serpen_hunt_and_poke__is_end` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen13hunt_and_pokeNtB2_21SerpenHuntAndPokePlan6is_end` |
| 소스 | `game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:162` |
| IR | `m10.ll` 9387~9873행 |
| 경로·가시성 | `game_ai::plan_legacy::old::SerpenHuntAndPokePlan::is_end` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `df0a90` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &SerpenHuntAndPokePlan | 본문에서 안 읽음 | 4 |
| 1 | 2 | version | usize | v24_objective_setup_should_release_to_passive 로 전달만. 본문 분기 없음 | 4 |
| 2 | 3 | _rnd | &mut StdRng | readnone — 미사용 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | info.team(+0x930) | 4 |
| 4 | 5 | data | &OperationData(24B) | cache/context/blackboard | 4 |
| 5 | 6 | team_plan | &TeamPlan(1064B) | objective(+0x41f 태그, +0x420 phase) | 4 |
| 6 | 7 | _debug | &mut DebugFrameData | readnone — 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn is_end(&self, version, _rnd, player, data, team_plan, _debug) -> bool
// L163  take_active(JungleType::Serpen) 인라인(team_plan.rs:244→231 objective_target)
if team_plan.objective.tag != MainObjective::Serpen(1) { return true; }
// L168
let camp_pos = ctx.map.camp_pos(JungleType::Serpen, is_blue = player.team == 0);
// L169  take_setup_like 인라인(team_plan.rs:258): objective == Serpen && phase == ObjectPhase::Setup(1)
if team_plan.objective.tag == 1 && team_plan.objective.phase(+0x420) == 1 {
    // L170
    if team_plan.v24_objective_setup_should_release_to_passive(version, player, data, JungleType::Serpen) { return true; }
    // L174  vtable+0x100
    if game.is_visible_cell(player.team, camp_pos.x / 32000, camp_pos.y / 32000) {
        // L175~176  enemy_team = 1 - player.team
        let nearest_to_camp_enemy = cache.iter_champions(enemy_team)
            .filter(|e| blackboard[enemy_team].is_recent_visible(game, ctx, player, e))   // closure#0
            .min_by_key(|e| dist_sq(e, camp_pos));                                          // closure s_0 (aux m12)
        // L178
        let Some(e) = nearest_to_camp_enemy else { return true; };
        // L179
        if dist_sq(e, camp_pos) > 22500000000 { return true; }
    }
}
// L189  vtable+0x40 get_game_mode → Moba 아니면 unwrap 패닉
let m = game.get_game_mode().as_moba().unwrap();
let serpen = m.jungle_runner.serpen.live_list.first().and_then(|id| game.get_entity_by_id(*id));   // vtable+0x1f0
// L190
if serpen.is_some() { return false; }
// L191
let m = game.get_game_mode().as_moba().unwrap();
return m.jungle_runner.serpen.next_respawn_tick.saturating_sub(game.tick()) > tps * 15;   // vtable+0x28 tick
```

**`mem` 메모리 접근 19건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | TeamPlan | 0x41f | objective (MainObjective) | r | L163 take_active(Serpen) 인라인(team_plan.rs:244/231) · ==1(Serpen) 아니면 return true · L169 재확인 | 4 | OK |
| 1 | TeamPlan | 0x420 | objective@Serpen.phase (ObjectPhase) | r | L169 take_setup_like 인라인(team_plan.rs:258) · ==1(Setup) 일 때만 Setup 블록 | 4 | OK |
| 2 | PlayerState | 0x930 | info.team | r | L168 camp_pos(is_blue = team==0) · L174 is_visible_cell(team) · L175 enemy = 1-team | 4 | OK |
| 3 | OperationData | 0x0 | cache | r | L174/L189 | 4 | OK |
| 4 | OperationData | 0x8 | context | r | L168 map / L191 setting | 4 | OK |
| 5 | OperationData | 0x10 | blackboard[2] | r | L175 blackboard[1-team].is_recent_visible | 4 | OK |
| 6 | GameContext | 0x20 | map (&MapDef) | r | L168 MapDef::camp_pos(map, JungleType::Serpen, team==0) | 4 | OK |
| 7 | GameContext | 0x8 | setting | r | L191 | 4 | OK |
| 8 | GameSetting | 0x12f8 | tick_per_second | r | L191 tps*15 | 4 | OK |
| 9 | AbstractGameWithCache | 0x0 | game.data_ptr (dyn data ptr) | r | L174/189/190/191 vtable 호출 self | 4 | OK |
| 10 | AbstractGameWithCache | 0x8 | game.vtable_ptr vtable | r | +0x100 is_visible_cell(L174) · +0x40 get_game_mode(L189/191) · +0x1f0 get_entity_by_id(L190) · +0x28 tick(L191) (divtable 확인) | 3 | OK |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | L175 iter_champions(1-team) 슬라이스 | 4 | OK |
| 12 | Entity | 0x660 | x | r | L176/179 camp 거리 | 4 | OK |
| 13 | Entity | 0x668 | y | r | 동상 | 4 | OK |
| 14 | GameMode | 0x0 | tag | r | L189/191 · 0=Moba 아니면 unwrap 패닉(L1013) | 4 | OK |
| 15 | GameMode | 0x8 | Moba.0 (&MobaMode) | r |  | 4 | OK |
| 16 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r | L189 live_list.first() → id | 4 | OK |
| 17 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | L189 ==0 이면 serpen None | 4 | OK |
| 18 | MobaMode | 0x1e0 | jungle_runner.serpen.next_respawn_tick | r | L191 (next_respawn_tick -sat tick) > tps*15 | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 163 | 태그 | MainObjective 메모리태그 1 = Serpen (tcxdict --enum MainObjective) · L169 ObjectPhase 태그 1 = Setup · L175 enemy = 1 - team | 3 |
| 1 | 5 | 168 | 태그 | JungleType 태그 5 = Serpen (tcxdict --enum JungleType) — camp_pos·take_active·should_release_to_passive 인자 | 3 |
| 2 | 32000 | 174 | 계수 | 셀 크기 — camp 좌표 → 셀 좌표 변환(is_visible_cell 인자). 임계 아님 | 4 |
| 3 | 22500000000 | 179 | 임계 | 150000^2 — 캠프에서 가장 가까운 가시 적 챔피언이 150000(4.69셀) 초과 거리면 종료(ugt) | 4 |
| 4 | 15 | 191 | 계수 | tps*15 = 15초. 세르펜 부재 시 (next_respawn_tick - tick) 가 15초 초과면 종료 | 4 |
| 5 | 0 | 168 | 태그 | team==0 → is_blue_side(camp_pos bool 인자) · L189/191 GameMode 태그 0=Moba · L189 live_list.len==0 | 4 |
| 6 | 2 | 175 | 임계 | 팀 인덱스 bounds(1-team < 2) — 판정 아님 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Setup 국면 적 원거리 종료 임계(제곱) | hunt_and_poke.rs:179 | 22500000000 | 올리면(150000→더 큼) 적이 캠프에서 더 멀어야 종료 → 플랜이 더 오래 유지됨. 내리면 적이 조금만 떨어져도 종료 | 4 | 기존 |
| 1 | 세르펜 부재 시 종료 리스폰 여유(초) | hunt_and_poke.rs:191 | 15 | 올리면 리스폰이 더 임박해야 플랜 유지(=종료 잦아짐). 내리면 리스폰 대기 중 플랜을 더 오래 유지 | 4 | 기존 |

<details><summary>`callees` 피호출자 21건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | is_end | game_ai::SmallActionUlt::is_end | in:game_ai | fn(&game_ai::SmallActionUlt, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action\cast.rs:289 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 106개 중 상위 3개 |
| 9 | is_end | game_ai::SmallActionPlay::is_end | pub | fn(&game_ai::SmallActionPlay, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action.rs:348 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 106개 중 상위 3개 |
| 10 | is_end | game_ai::SmallActionTrace::is_end | in:game_ai | fn(&game_ai::SmallActionTrace, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action\trace.rs:406 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 106개 중 상위 3개 |
| 11 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | is_visible_cell | game_core::AbstractGame::is_visible_cell | pub | fn(&Self/#0, usize, usize, usize) -> bool | game-core\src\simulation.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | is_visible_cell | <game_core::Game as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::Game, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:1804 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | is_visible_cell | <game_core::SingleLaneGame as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::SingleLaneGame, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:3869 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | take_active | game_ai::plan_legacy::team_plan::TeamPlan::take_active | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan.rs:243 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | v24_objective_setup_should_release_to_passive | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_release_to_passive | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:177 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 3개**: `dist_sq`, `first`, `phase`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:6812) · **형제 9개** (SerpenHuntAndPokePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::SerpenHuntAndPokePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:9 | True | fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan) -> game_ai::plan_legacy::old::SerpenHuntAndPokePlan |
| 1 | <game_ai::plan_legacy::old::SerpenHuntAndPokePlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:9 | True | fn() -> game_ai::plan_legacy::old::SerpenHuntAndPokePlan |
| 2 | <game_ai::plan_legacy::old::SerpenHuntAndPokePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:9 | True | fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::goal | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:22 | True | fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan) -> game_core::BigGoal |
| 4 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::update | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:26 | True | fn(&mut game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 5 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::sub_plan | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:30 | False | fn(&mut game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |
| 6 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::next_plan | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:94 | True | fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 7 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::check_recall | in:game_ai::plan_legacy::old::serpen::hunt_and_poke | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:99 | False | fn(&mut game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) -> bool |
| 8 | game_ai::plan_legacy::old::SerpenHuntAndPokePlan::is_end | pub | game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:162 | False | fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | v24_objective_setup_should_release_to_passive(objective_discipline) 본문 미독해(범위 밖) | 4 |  |
| 1 | 미탐색 | take_active(team_plan.rs:244)/objective_target(231)/take_setup_like(258) 헬퍼는 인라인 잔여로 판정(태그 비교)만 확인 — 원 시그니처 미확인 | 4 |  |
| 2 | 미탐색 | closure#0(is_recent_visible 필터)의 blackboard 인덱스 = 1-team(적 팀 보드) — IR 그대로 | 4 |  |
| 3 | 미탐색 | self·_rnd·_debug 는 본문에서 읽지 않음(readnone) · version 은 전달만 | 4 |  |
| 4 | 표기 불가 | L189 live_list.first() 해석: len==0 → None, 아니면 ptr[0] 만 읽음 — `.first()` 또는 `[0]` 표기 불가(외연 동일) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

