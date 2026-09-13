---

### `50` check_epic_giveup — Morgard(에픽) 목표를 포기할지 — 생존 머릿수·캠프 근처 머릿수(+미시야 도달가능 적)로 열세면 true

| 항목 | 값 |
|---|---|
| id | `epic__check_epic_giveup` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic17check_epic_giveup` |
| 소스 | `game-ai\src\plan_legacy\old\epic.rs:109` |
| IR | `m09.ll` 59019~60555행 |
| 경로·가시성 | `game_ai::plan_legacy::old::check_epic_giveup` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `de81b0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 분기 없음. macro_judgement_penalty 에만 전달(60538) | 4 |
| 1 | 2 | _rnd | &mut StdRng | readnone — 미사용 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930) 읽기 + v23_visible_objective_overload·is_recent_visible·macro_judgement_penalty 인자 | 4 |
| 3 | 4 | data | &OperationData(24B) | +0 cache / +8 context / +0x10 blackboard | 4 |
| 4 | 5 | team_plan | &TeamPlan(1064B) | objective(+0x41f)·vision.last_visible_pos(+0x230)·vision.last_checked_ticks(+0x2d0) 읽기 | 4 |
| 5 | 6 | _debug | &mut DebugFrameData | readnone — 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn check_epic_giveup(version, _rnd, player, data, team_plan, _debug) -> bool

[L110] if !rule_scope::morgard_exists(data.context) { return true }
         // 인라인 = runner.rs:263 spawn_epic: context.tutorial(+0x38) 태그가 1..=6 (First~JungleOnly) 이면 Morgard 없음
[L116] moba = data.cache.game.get_game_mode()/*vtable+0x40*/ .as_moba().unwrap()   // 태그≠0 → unwrap_failed
       if moba.jungle_runner.epic.live_list.len(+0x1a8) == 0 { return true }         // 살아있는 에픽 없음

[L122] if team_plan.objective(+0x41f) == Some(Morgard{phase: Hunt(3), ..}) {
[L123]   epic_id = moba.live_list[0]   // len==0 이면 이 블록 통째로 skip → L138
         epic = game.get_entity_by_id(epic_id)/*vtable+0x1f0*/ ; None 이면 skip → L138
[L125]   hp_ratio = epic.hp(+0x670) * 100 / epic.stat_cached.hp(+0x628)   // max_hp 0 → div_by_zero 패닉
[L126]   if hp_ratio <= 20 { return false }                                 // 거의 잡았으면 포기 안 함
[L127]   if v23_visible_objective_overload(player, data, MapDef::camp_pos(map, Morgard(4), team==0)) { return true }
       }

[L138] team = player.info.team ; live_ally_count  = cache.champions(team, pool).len()
[L140] live_enemy_count = cache.champions(1-team, pool).len()
[L143] camp = MapDef::camp_pos(map, Morgard(4), team==0)
[L144~146] ally_near = player_champion[team].iter_champions().filter(|e| distance_sq(e, camp) < 22500000001).count()
[L147~154] enemy_possible = player_champion[1-team].iter_champions().filter(|(i,e)| {
             need   = sat_sub(utils::distance(team_plan.vision.last_visible_pos[i], camp), 150000)
             reach  = sat_sub(game.tick(), team_plan.vision.last_checked_ticks[i]) * e.move_speed(+0x640)
             e.hp*100/e.max_hp > 49  &&  reach >= need  &&  !blackboard[1-team].is_recent_visible(game, player, e)
           }).count()                       // '안 보이는데 이미 캠프 근처까지 왔을 수 있는' 건강한 적
[L158~159] enemy_near = player_champion[1-team].iter_champions().filter(|e|
             distance_sq(e, camp) < 22500000001 && e.hp*100/e.max_hp > 49 && blackboard[1-team].is_recent_visible(game, player, e)
           ).count()
[L163] penalty = macro_judgement_penalty(version, player)   // i32
[L164] return (penalty + 1 + live_ally_count) < live_enemy_count            // signed i32 비교
           && (ally_near + 1 + penalty) < (enemy_near + enemy_possible)

※ 두 카운트 루프는 IR 에서 5슬롯 언롤(59240~60530). 부작용 없음(champions Vec 두 개는 즉시 drop)
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache — game(dyn)·player_champion·champions | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — [enemy_team] 을 is_recent_visible 에 넘김(59613) | 4 | OK |
| 3 | GameContext | 0x38 | tutorial | r | TutorialType i8. (tag-1) <u 6 즉 1 First~6 JungleOnly 면 morgard_exists=false → 즉시 true (rule_scope.rs:46 ← runner.rs:263 spawn_epic 인라인) | 4 | OK |
| 4 | GameContext | 0x0 | pool | r | &Bump — champions() Vec 할당자(59403) | 4 | OK |
| 5 | GameContext | 0x20 | map | r | &MapDef — camp_pos 인자 | 4 | OK |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr (&dyn AbstractGame data+vtable) | r | vtable+0x40 get_game_mode / +0x1f0 get_entity_by_id / +0x28 tick | 4 | OK |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2]. [team] 은 L144 아군 근접 카운트, [1-team] 은 L147·L158 적 카운트(5회 언롤) | 4 | OK |
| 8 | GameMode | 0x0 | tag | r | get_game_mode 반환 {i64,ptr}. 0=Moba 여야 하고 아니면 unwrap_failed(60553) | 4 | OK |
| 9 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 이면 살아있는 에픽 없음 → true(59072~59078) | 4 | OK |
| 10 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | [0] 을 에픽 entity id 로 읽어 get_entity_by_id(59144~59148) | 4 | OK |
| 11 | PlayerState | 0x930 | info.team | r | usize. camp_pos 의 side 인자(team==0)·champions 인자·player_champion 인덱스 | 4 | OK |
| 12 | TeamPlan | 0x41f | objective | r | Option<MainObjective> 니치 1B. ==0 → Some(Morgard) | 4 | OK |
| 13 | TeamPlan | 0x420 | objective@Morgard.phase | r | ObjectPhase 1B. ==3 → Hunt. Morgard·Hunt 일 때만 L123~127 에픽 HP/과밀 검사 | 4 | OK |
| 14 | TeamPlan | 0x230 | vision.last_visible_pos[i] | r | (u64,u64) stride 16. 적 i 의 마지막 관측 좌표 → camp 까지 utils::distance | 4 | OK |
| 15 | TeamPlan | 0x2d0 | vision.last_checked_ticks[i] | r | usize stride 8. tick - 이 값 = 미관측 경과 틱 | 4 | OK |
| 16 | Entity | 0x670 | hp | r | usize. hp_ratio = hp*100/stat_cached.hp | 4 | OK |
| 17 | Entity | 0x628 | stat_cached.hp | r | usize 최대체력. 0 이면 panic_const_div_by_zero | 4 | OK |
| 18 | Entity | 0x660 | x | r | u64 — camp 와의 distance_sq | 4 | OK |
| 19 | Entity | 0x668 | y | r | u64 | 4 | OK |
| 20 | Entity | 0x640 | stat_cached.move_speed | r | usize. 미관측 적의 도달가능 거리 = 경과틱 * move_speed | 4 | OK |
| 21 | Vec<&Entity>(bumpalo,32B) | 0x18 | len | r | champions(team).len() = live_ally_count / champions(1-team).len() = live_enemy_count | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 110 | 태그 | tutorial 태그 -1 후 <u 6 비교: TutorialType 1(First)~6(JungleOnly) 범위 검사(morgard_exists 인라인). 그 범위면 Morgard 가 안 나오는 모드 → 즉시 true | 4 |
| 1 | 6 | 110 | 임계 | 위 범위 폭(1..=6). None(0)/Line(7)/Total(8) 만 Morgard 존재 | 4 |
| 2 | 0 | 116 | 태그 | GameMode 태그 0 = Moba 강제(아니면 unwrap 패닉) / live_list.len == 0 → true | 4 |
| 3 | 0 | 122 | 센티널 | Option<MainObjective> 니치 태그 0 = Some(Morgard) | 4 |
| 4 | 3 | 122 | 태그 | ObjectPhase 태그 3 = Hunt. Morgard+Hunt 일 때만 에픽 HP·과밀 검사 | 4 |
| 5 | 100 | 125 | 계수 | hp_ratio = hp*100/max_hp (백분율). L154·L159 의 적 HP 비율도 같은 식 | 4 |
| 6 | 20 | 126 | 임계 | 에픽 HP% > 20 이 아니면(≤20) 포기 안 함(false) — 거의 잡은 에픽은 놓지 않는다 | 4 |
| 7 | 4 | 127 | 태그 | JungleType 태그 4 = Morgard. MapDef::camp_pos(map, Morgard, team==0) 의 camp 인자(L143 도 동일) | 4 |
| 8 | 1 | 140 | 계수 | enemy_team = 1 - team | 4 |
| 9 | 22500000001 | 144 | 임계 | 150000²+1 — camp 로부터 dist² < 이 값(≤150000, ≈4.7칸) 이면 '캠프 근처'. L158 적 근접 카운트도 동일 | 4 |
| 10 | 150000 | 150 | 계수 | 미관측 적: sat_sub(distance(last_visible_pos, camp), 150000) — 캠프 근처 반경만큼 빼서 '근처까지 오는 데 필요한 거리' | 4 |
| 11 | 49 | 154 | 임계 | 적 HP% > 49 (즉 ≥50%) 여야 위협 머릿수로 센다(L154 미관측·L159 근접 둘 다) | 4 |
| 12 | 1 | 164 | 계수 | 아군 쪽에 +1 여유: (penalty+1+live_ally < live_enemy) && (near_ally+1+penalty < near_enemy+possible_enemy) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 에픽 HP 하한(포기 금지) | epic.rs:126 (IR m09.ll:59161 `icmp ugt i64 %68, 20`) | 20 | Morgard·Hunt 중 에픽 HP% ≤ 20 이면 무조건 유지. 올리면 더 일찍부터 '끝까지 간다'가 되고, 0 이면 이 보호가 사라져 머릿수 판정만 남는다 | 4 | 기존 |
| 1 | 적 위협 HP 하한 | epic.rs:154·159 (IR 59314 `icmp ugt i64 %257, 49` 등 10곳) | 49 | HP% ≥50 인 적만 근접/도달가능 머릿수로 센다. 내리면 빈사 적도 위협으로 세어 포기가 잦아진다 | 4 | 기존 |
| 2 | 캠프 근처 반경 | epic.rs:144·158 (IR 59259 `icmp ult i64 %124, 22500000001` 등 10곳) + epic.rs:150 (`usub.sat 150000`) | 150000 | ≈4.7칸. 올리면 아군/적 모두 더 넓게 '근처'로 세어지며, 미관측 적의 필요 이동거리도 그만큼 줄어 possible 카운트가 늘어난다 | 4 | 기존 |
| 3 | 아군 여유 머릿수 | epic.rs:164 (IR 60540 `add i32 %604, 1` / 60545 `add nuw nsw i32 %224, 1`) | 1 | 두 비교식 모두 아군 쪽에 +1. 즉 적이 아군보다 2명 이상 많아야(penalty 0 기준) 포기. 0 으로 내리면 1명 열세에도 포기, 올리면 잘 포기 안 함 | 4 | 기존 |
| 4 | 판단 페널티 | epic.rs:163 macro_judgement_penalty(version, player) — 내부는 범위 밖 | 함수값 | 아군 쪽 가산항. 값이 크면(판단력 낮음) 더 쉽게 열세로 보아 포기. 음수면 반대 | 4 | 기존 |

<details><summary>`callees` 피호출자 22건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | check_epic_giveup | game_ai::plan_legacy::old::check_epic_giveup | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\epic.rs:109 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 6 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | macro_judgement_penalty | game_ai::plan_legacy::team_plan::macro_judgement_penalty | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:15 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | morgard_exists | game_ai::plan_legacy::rule_scope::morgard_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | v23_visible_objective_overload | game_ai::plan_legacy::team_plan::v23_visible_objective_overload | pub | fn(&game_core::PlayerState, &game_core::OperationData, (u64, u64)) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:53 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 4개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `move_speed`, `objective`, `sat_sub`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m09.ll:14844, m09.ll:31316, m09.ll:31320) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | v23_visible_objective_overload(player, data, x, y) 내부는 안 봄 — 이름상 '목표 지점 시야 과밀'이고 true 면 포기. 인자 (x,y)=camp_pos 만 확정 | 4 |  |
| 1 | 미탐색 | AbstractGameWithCache::champions(cache, team, pool) 가 '살아있는' 챔피언만 돌려주는지는 game_core 미열람 — dbg 변수명 live_ally_count/live_enemy_count 로 추정 | 4 |  |
| 2 | 미탐색 | Blackboard::is_recent_visible 의 정확한 의미(최근 N틱 관측)는 game_core 미열람. 인덱스가 적팀 판(1-team)인 것만 확정 | 4 |  |
| 3 | 미탐색 | MapDef::camp_pos(map, JungleType, bool) 의 bool 이 team==0 인 것은 확정, 그 뜻(아군 진영 기준 좌우 반전)은 추정 | 5 |  |
| 4 | 미탐색 | L123 의 live_list[0] 이 '지금 살아있는 Morgard' 라는 것은 live_list 원소가 entity id(i64) 이고 get_entity_by_id 로 넘기는 것으로 읽음. 에픽이 2종(Morgard/Serpen) 이상 동시에 살아 있을 때 [0] 이 Morgard 라는 보장은 본문에 없음 | 4 |  |
| 5 | 미탐색 | L164 두 비교는 signed(icmp slt) 인데 카운트는 usize→i32 trunc — 5 이하라 실질 무해 | 4 |  |
| 6 | 미탐색 | constants 의 `0`(L116/L122) 은 태그값이자 len 비교값으로 여러 자리 겹침 — 대표 줄만 기재 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | macro_judgement_penalty(version, player) -> i32 내부는 안 봄. 부호·범위 미확정(i32 signed 비교이므로 음수 가능성 열림) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

