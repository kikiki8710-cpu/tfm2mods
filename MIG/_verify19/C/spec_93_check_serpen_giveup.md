---

### `93` check_serpen_giveup — 세르펜 목표를 포기할지 판정 — 세르펜 부재/과부하/근처·건강 머릿수 열세(판단 페널티 가산)로 결정

| 항목 | 값 |
|---|---|
| id | `serpen__check_serpen_giveup` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen19check_serpen_giveup` |
| 소스 | `game-ai\src\plan_legacy\old\serpen.rs:234` |
| IR | `m05.ll` 49223~51792행 |
| 경로·가시성 | `game_ai::plan_legacy::old::check_serpen_giveup` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d639f0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | version | usize | macro_judgement_penalty 에 전달만 | 4 |
| 1 | 1 | _rnd | &mut StdRng(320B) | 미사용(readnone) | 4 |
| 2 | 2 | player | &PlayerState(2528B) | info.team(+0x930)·is_recent_visible/헬퍼 인자 | 4 |
| 3 | 3 | data | &OperationData(24B) | +0x0 cache / +0x8 context / +0x10 blackboard | 4 |
| 4 | 4 | team_plan | &TeamPlan(1064B) | objective(+0x41f) 만 읽음(take_hunt_commit 인라인) | 4 |
| 5 | 5 | _debug | &mut DebugFrameData(224B) | 미사용(readnone) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
ctx = data.context; team = player.info.team; enemy = 1-team; camp = JungleType::Serpen(5)
L235 if !serpen_exists(ctx) → return true                       // tutorial ∉ {None,MidBottom,Line,Total}
L241 mode = game.get_game_mode() (Moba 아니면 unwrap 패닉); if mode.jungle_runner.serpen.live_list.len == 0 → return true   // 세르펜 없음 = 포기
L246 if team_plan.take_hunt_commit(camp)  [objective == Some(Serpen{phase: Hunt, ..})] {
L247     if serpen.live_list.len != 0 && let Some(serpen) = game.get_entity_by_id(live_list[0]) {
L249         hp_ratio = serpen.hp*100 / serpen.stat_cached.hp
L250         if hp_ratio <= 20 → return false                     // 다 잡은 세르펜은 포기 금지
L251         if v23_visible_objective_overload(player, data, map.camp_pos(Serpen, team==0)) → return true
             // (objective_helpers.rs:54~57: 근처(반경 180000·hp40%) 최근가시 적 > 2 && 적 >= 건강 아군 + 2)
         }
     }
L260 my_serpen_count = mode.serpen_count[team]; L261 enemy_serpen_count = mode.serpen_count[enemy]
L263 if my_serpen_count > enemy_serpen_count {
L265     near_ally  = player_champion[team].iter_champions().filter(|e| is_bottom_side(ctx,e.x,e.y) && e.hp*100/e.max_hp > 49).count()
L268     near_enemy = player_champion[enemy].iter_champions().filter(|e| is_bottom_side && hp% > 49 && blackboard[1-team].is_recent_visible(game, player, e)).count()
L271     penalty = macro_judgement_penalty(version, player)
L272     return near_ally + penalty < near_enemy
     }
L274 near_ally  = (L265 와 동일 필터).count()
L277 near_enemy = (L268 과 동일 필터).count()
L281 if is_line_phase(ctx) {
L282     healthy_ally  = (1..5).filter(|i| player_champion[team][i].map_or(false, |c| c.hp*100/c.max_hp > 49)).count()   // Top(슬롯0) 제외
L283     healthy_enemy = (1..5).filter(|i| player_champion[enemy][i] … hp% > 49).count()
L284     penalty = macro_judgement_penalty(version, player)
L285     if healthy_ally + 1 + penalty < healthy_enemy {
L286         return near_ally + penalty < near_enemy
         }
         return false
     } else {
L288     healthy_ally  = player_champion[team].iter_champions().filter(hp% > 49).count()
L289     healthy_enemy = player_champion[enemy].iter_champions().filter(hp% > 49).count()
L290     penalty = macro_judgement_penalty(version, player)
L291     if healthy_ally + 1 + penalty < healthy_enemy {
L292         return near_ally + penalty < near_enemy
         }
         return false
     }
(보조: is_bottom_side(map_regions.rs:28~30) = (setting.height - y <= x) || is_near_mid_line(ctx,x,y);
 is_recent_visible(blackboard.rs:348~354) = 현재 가시 || (해당 적의 last_visible[pos] + 120 >= tick);
 macro_judgement_penalty(objective_helpers.rs:16~18) = ceil(max(400 - judge_accuracy,0)/125) ∈ 0..=4)
```

**`mem` 메모리 접근 20건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r |  | 4 | OK |
| 1 | OperationData | 0x10 | blackboard | r | [1-team](적팀 블랙보드) 을 is_recent_visible 의 self 로 — 적 p 의 last_visible[pos] | 4 | OK |
| 2 | GameContext | 0x38 | tutorial | r | serpen_exists / is_line_phase 의 switch 키 | 4 | OK |
| 3 | GameContext | 0x8 | setting | r |  | 4 | OK |
| 4 | GameContext | 0x20 | map | r | &MapDef → camp_pos(Serpen, is_blue_side) | 4 | OK |
| 5 | GameSetting | 0x12f8 | tick_per_second | r | is_line_phase tps*30 | 4 | OK |
| 6 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | is_line_phase | 4 | OK |
| 7 | GameSetting | 0x12c0 | height | r | is_bottom_side ry = height - y | 4 | OK |
| 8 | AbstractGameWithCache | 0x0 | game.data_ptr(dyn) | r | vtable +0x40 get_game_mode / +0x28 tick / +0x1f0 get_entity_by_id | 4 | OK |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion | r | [team]·[1-team] | 4 | OK |
| 10 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | 0 = 세르펜 미생존 → 즉시 true | 4 | OK |
| 11 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r | [0] = 세르펜 엔티티 id | 4 | OK |
| 12 | MobaMode | 0x260 | serpen_count / [1-team] | r | 팀별 세르펜 처치 수 | 4 | OK |
| 13 | TeamPlan | 0x41f | objective | r | Option<MainObjective> 니치. 1 = Some(Serpen) | 4 | OK |
| 14 | TeamPlan | 0x420 | objective@Some.0@Serpen.phase@tag | r | 3 = ObjectPhase::Hunt | 4 | OK |
| 15 | PlayerState | 0x930 | info.team | r | team==0 → is_blue_side | 4 | OK |
| 16 | Entity | 0x628 | stat_cached.hp | r | 최대 HP(0 이면 div_by_zero 패닉) | 4 | OK |
| 17 | Entity | 0x670 | hp | r |  | 4 | OK |
| 18 | Entity | 0x660 | x | r | is_bottom_side | 4 | OK |
| 19 | Entity | 0x668 | y | r | is_bottom_side | 4 | OK |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 235 | 태그 | TutorialType {0,5,7,8} = serpen_exists. 밖이면 true(포기) | 4 |
| 1 | 5 | 246 | 태그 | JungleType::Serpen 태그(camp 변수 · camp_pos 인자) | 4 |
| 2 | 1 | 246 | 태그 | team_plan.objective@tag == 1 = Some(MainObjective::Serpen) (take_hunt_commit 인라인, team_plan.rs:249) | 4 |
| 3 | 3 | 246 | 태그 | Serpen.phase == 3 = ObjectPhase::Hunt | 4 |
| 4 | 100 | 249 | 계수 | hp_ratio = serpen.hp*100/max_hp | 4 |
| 5 | 20 | 250 | 임계 | Hunt 커밋 중 세르펜 hp_ratio <= 20 이면 포기 안 함(false) | 4 |
| 6 | 49 | 266 | 임계 | hp*100/max_hp > 49 = HP 50% 이상 (모든 필터 공통: L266·269·275·278·282·283·288·289) | 4 |
| 7 | 30 | 281 | 계수 | is_line_phase: tick < epic first_spawn_tick.saturating_sub(tps*30) | 4 |
| 8 | 1 | 282 | 태그 | 라인전 단계 healthy 셈은 인덱스 (1..5) — 슬롯 0(Top) 제외 | 4 |
| 9 | 1 | 285 | 태그 | healthy_ally + 1 + penalty < healthy_enemy 이어야 근처 비교로 진행 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Hunt 중 세르펜 잔여 HP 포기 금지선 | serpen.rs:250 | 20 | 올리면 더 높은 HP 에서도 포기 불가 → 끝까지 버팀 | 4 | 기존 |
| 1 | 건강 기준 HP% | serpen.rs:266 등 | 49 | 올리면 근처/건강 아군·적이 모두 줄어듦(양쪽 대칭이라 효과 상쇄 경향, 단 적은 가시 조건이 더 붙음) | 4 | 기존 |
| 2 | 건강 머릿수 열세 마진 | serpen.rs:285/291 | 1 | 올리면 더 큰 열세여야 근처 비교로 진입 → 포기 드묾 | 4 | 기존 |
| 3 | 라인전 healthy 셈 제외 슬롯 | serpen.rs:282~283 | 1 | (1..5) 의 시작. 0 으로 내리면 Top 포함 | 4 | 기존 |
| 4 | 과부하 판정 반경/최소HP% | objective_helpers.rs:54~55 (별도 함수) | 180000 | v23_visible_objective_overload 의 근처 반경. 키우면 더 먼 적까지 세어 포기 잦아짐 | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | is_bottom_side | game_core::is_bottom_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:27 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | macro_judgement_penalty | game_ai::plan_legacy::team_plan::macro_judgement_penalty | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:15 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | take_hunt_commit | game_ai::plan_legacy::team_plan::TeamPlan::take_hunt_commit | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan.rs:248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | v23_visible_objective_overload | game_ai::plan_legacy::team_plan::v23_visible_objective_overload | pub | fn(&game_core::PlayerState, &game_core::OperationData, (u64, u64)) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:53 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `ceil`, `map_or`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m09.ll:14253, m09.ll:32135, m09.ll:32139, m09.ll:33973, m09.ll:34344) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L263 우세 분기(my_serpen_count > enemy)에서 near 셈은 i32, 나머지 분기는 i64 로 나와 소스가 두 벌인지 한 벌 인라인 차이인지 표기 불가(동작 동일) | 4 |  |
| 1 | 표기 불가 | L265/L274 근처 아군 필터와 L268/L277 근처 적 필터가 완전 동일해 보이나 소스가 헬퍼 호출인지 인라인 클로저인지 표기 불가(closure$N 번호만 다름) | 4 |  |
| 2 | 미탐색 | `> 49`/`<= 20` 은 소스가 `>= 50`/`< 21` 일 수 있음(외연 동일) | 4 |  |
| 3 | 미탐색 | L282~283 의 (1..5) 범위 상한 5 는 리터럴로 IR 에 안 남음(언롤됨: 슬롯 1~4 로드로 확인) — 상수 목록에 5 미등재 | 4 |  |
| 4 | 미탐색 | v23_visible_objective_overload 내부 헬퍼(v23_healthy_allies_near_point / v23_recent_visible_enemies_near_point, m15.ll 53341/55548)의 반경 180000·40 의 정확한 의미(40 = HP% 최소치로 추정)는 담당 밖 | 5 |  |
| 5 | 미탐색 | vtable 간접호출(get_game_mode +0x40 / tick +0x28 / get_entity_by_id +0x1f0, divtable 확정)은 C2 대조 불가라 logic 에만 적음 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

