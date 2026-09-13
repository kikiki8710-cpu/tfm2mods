---

### `96` serpen_passive_plan — 세르펜 목표(phase)에 맞는 개인 BigPlan 을 고른다 — Hunt→SerpenHuntAndPoke, Setup→전략(object_buildup)·적 압박·라인 상태·이동시간으로 PassiveLine/SerpenHuntAndPoke/None

| 항목 | 값 |
|---|---|
| id | `serpen__serpen_passive_plan` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen19serpen_passive_plan` |
| 소스 | `game-ai\src\plan_legacy\old\serpen.rs:416` |
| IR | `m05.ll` 51795~52846행 |
| 경로·가시성 | `game_ai::plan_legacy::old::serpen_passive_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d65620` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, game_ai::plan_legacy::team_plan::ObjectPhase, std::option::Option<(u64, u64)>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan>
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<BigPlan>(384B) | tag -1(0xFFFF..)=None / 14=SerpenHuntAndPoke / 3=PassiveLine(line @+0x11e) | 4 |
| 1 | 1 | version | usize | v25_objective_splitter_should_join_contest 에만 전달 | 4 |
| 2 | 2 | rnd | &mut StdRng(320B) | is_skip_serpen·PlayerState::strategy 에 전달 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0) | 4 |
| 4 | 4 | data | &OperationData(24B) |  | 4 |
| 5 | 5 | team_plan | &TeamPlan(1064B) | vision.last_visible_pos[]·last_checked_ticks[] (Flexible 분기) | 4 |
| 6 | 6 | phase | ObjectPhase(i8 0..4) | 0 None / 1 Setup / 2 Assemble / 3 Hunt | 4 |
| 7 | 7 | depart_anchor | &Option<(u64,u64)>(24B) | 출발 기준점. Some 이면 챔피언 위치 대신 사용(L485) | 4 |
| 8 | 8 | _debug | &mut DebugFrameData(224B) | 미사용(readnone) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
ctx = data.context; team = player.info.team; my_pos = player.info.position
L417 if !serpen_exists(ctx) → return None (L418)
L421 is_line_phase = tutorial∈{0,5,7,8} ? tick < epic_jungle.first_spawn_tick.saturating_sub(tps*30) : true
L422 champ = cache.player_champion[team][my_pos].unwrap()
L423 skip = is_skip_serpen(version, rnd, player, data);  L425 if skip → return None (L506)
L426 match phase {
  Hunt(3)  → L427 return Some(BigPlan::SerpenHuntAndPoke(SerpenHuntAndPokePlan{v46_flee_threats: vec![], focus_serpen_only:false, vision_only:false, v46_flee:false}))
  Setup(1) → 아래
  _        → return None (L503)
}
L429 strategy = player.strategy(rnd, game)
L431 opposite_object_pressure = v23_enemy_object_pressure(player, data, JungleType::Morgard)
     (objective_helpers.rs:181~207 요지: 해당 오브젝트 생존 && 근처(180000, hp40%) 최근가시 적 != 0 && 적 >= 건강 아군)
L432 if opposite_object_pressure {
L433     if let Some(line) = v23_objective_setup_pressure_line(player, data, &[Bottom, Mid]) {
L434         return Some(PassiveLine(PassiveLinePlan{line, ..default}))
         }
     }
L438 object_buildup = strategy.object_buildup
L439 match object_buildup {
  Split(position) → L440 if !is_line_phase && position == my_pos {
                      L441 if !v25_objective_splitter_should_join_contest(version, player, data, JungleType::Serpen) {
                      L443     return Some(PassiveLine{line: fallback_line(ctx, Top)})
                      } }
                    // 그 외 → L496
  Flexible(6)     → L449 camp = map.camp_pos(Serpen, team==0)
                    L452 near_serpen_enemy = (0..5).filter_map(|p| player_champion[1-team][p]).filter(|e|
                    L454     last_pos = team_plan.vision.last_visible_pos[p];
                    L455     d = distance(last_pos, camp).saturating_sub(150000);
                    L456     move_speed = e.stat_cached.move_speed;
                    L457     can_move = (tick.saturating_sub(team_plan.vision.last_checked_ticks[p])) * move_speed;
                    L459     e.hp*100/e.max_hp > 49 && can_move >= d && !blackboard[1-team].is_recent_visible(game, player, e)
                         ).count()
                    L463 if near_serpen_enemy < 3 && dist_sq(champ, camp) <= 320000^2
                    L464    && is_enemy_side(ctx, team, champ.x, champ.y)   [= (team==0) XOR ((x - y + height) > width)] {
                    L466     bottom_count = blackboard[team].bottom_minion_state.minion_count; L467 mid_count = ...mid...
                    L469     if cache.bottom_lead[team] < 3 {
                                 if cache.mid_lead[team] < 3 { L470 return Some(PassiveLine{line: bottom_count < mid_count ? Bottom(L471) : Mid(L473)}) }
                    L476         else return Some(PassiveLine{line: Bottom})
                             }
                    L477     else if cache.mid_lead[team] < 3 { L478 return Some(PassiveLine{line: Mid}) }
                         }
                    L482 remain_spawn_tick = mode.jungle_runner.serpen.next_respawn_tick.saturating_sub(tick)
                    L483 camp_pos = map.camp_pos(Serpen, team==0)
                    L485 (dpx,dpy) = depart_anchor.unwrap_or((champ.x, champ.y))
                    L486 dist_to_camp = distance(dpx,dpy, camp_pos)
                    L487 move_speed = champ.stat_cached.move_speed;  L488 move_tick = dist_to_camp / move_speed
                    L489 if remain_spawn_tick > move_tick + tps*2 → L490 return None
                    // 아니면 L496
  Gather(5)       → L496
}
L496 return Some(BigPlan::SerpenHuntAndPoke(SerpenHuntAndPokePlan{빈 Vec, 플래그 false}))   // camp_pos 호출 결과는 IR 상 미사용
```

**`mem` 메모리 접근 29건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r |  | 4 | OK |  |
| 1 | OperationData | 0x10 | blackboard | r | [team] minion_count / [1-team] is_recent_visible | 4 | OK |  |
| 2 | GameContext | 0x38 | tutorial | r | serpen_exists / is_line_phase | 4 | OK |  |
| 3 | GameContext | 0x8 | setting | r |  | 4 | OK |  |
| 4 | GameContext | 0x20 | map | r | camp_pos(Serpen, team==0) | 4 | OK |  |
| 5 | GameSetting | 0x12f8 | tick_per_second | r | tps*30(is_line_phase) · tps*2(L489, shl 1) | 4 | OK |  |
| 6 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | is_line_phase | 4 | OK |  |
| 7 | GameSetting | 0x12c0 | height | r | is_blue_side(map_regions.rs:7~8) | 4 | OK |  |
| 8 | GameSetting | 0x12b8 | width | r | is_blue_side: (x - y + height) > width | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x0 | game.data_ptr(dyn) | r | vtable +0x28 tick / +0x40 get_game_mode | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x1e0 | player_champion / [1-team][0..5] | r | 내 챔피언(None 이면 unwrap 패닉) · 적 5명 순회 | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x21d0 | mid_lead[team] | r | < 3 판정(L469~477) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x21e0 | bottom_lead[team] | r | < 3 판정(L469) | 4 | OK |  |
| 13 | MobaMode | 0x1e0 | jungle_runner.serpen.next_respawn_tick | r | remain_spawn_tick(L482) | 4 | OK |  |
| 14 | TeamPlan | 0x230 | vision.last_visible_pos[i] | r | (u64,u64) 적 p 마지막 목격 위치(L454) | 4 | OK |  |
| 15 | TeamPlan | 0x2d0 | vision.last_checked_ticks[i] | r | can_move = (tick - last_checked)*move_speed(L457) | 4 | OK |  |
| 16 | PlayerState | 0x930 | info.team | r |  | 4 | OK |  |
| 17 | PlayerState | 0x9c0 | info.position@tag | r | 내 챔피언 슬롯 · Split(position) 비교 | 4 | OK |  |
| 18 | Strategy | 0x0 | object_buildup@tag | r | ObjectBuildupStrategy: <5 = Split(position=값) / 5 Gather / 6 Flexible | 4 | OK |  |
| 19 | Blackboard | 0x48 | mid_minion_state.minion_count | r | L467 | 4 | OK |  |
| 20 | Blackboard | 0x70 | bottom_minion_state.minion_count | r | L466 | 4 | OK |  |
| 21 | Entity | 0x660 | x | r |  | 4 | OK |  |
| 22 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 23 | Entity | 0x640 | stat_cached.move_speed | r | L456 적 / L487 내 챔피언(0 이면 div_by_zero 패닉) | 4 | OK |  |
| 24 | Entity | 0x628 | stat_cached.hp | r | 최대 HP | 4 | OK |  |
| 25 | Entity | 0x670 | hp | r |  | 4 | OK |  |
| 26 | Option<BigPlan>(sret) | 0x0 | @tag | w | BigPlan 니치 태그(tcxdict: SerpenHuntAndPoke idx12→14, PassiveLine idx1→3) | 3 | 확인불가(tcx 사전에 타입 없음) | -1 None / 14 SerpenHuntAndPoke / 3 PassiveLine |
| 27 | Option<BigPlan>(sret) | 0x8 | SerpenHuntAndPokePlan{v46_flee_threats: Vec::new(), focus_serpen_only:false, vision_only:false, v46_flee:false} | w | L427·L496 | 4 | 확인불가(tcx 사전에 타입 없음) | cap 0 / ptr 8(dangling) / len 0 / 3 bool = 0 |
| 28 | Option<BigPlan>(sret) | 0x11e | PassiveLinePlan.line | w | +0x8 부터 PassiveLinePlan 280B 를 0/빈 Vec 으로 초기화 후 line 만 기록(L434·443·471·473·476·478) | 4 | 확인불가(tcx 사전에 타입 없음) | LineType (0 Top / 1 Mid / 2 Bottom) |

**`consts` 상수 24건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 0 | 417 | 태그 | TutorialType {0,5,7,8}=serpen_exists. 밖이면 None(L418) | 4 |  |
| 1 | 30 | 421 | 계수 | is_line_phase: tick < epic first_spawn_tick.saturating_sub(tps*30). tutorial 가 위 집합 밖이면 is_line_phase=true 로 접힘(%35 phi) | 4 |  |
| 2 | 3 | 426 | 임계 | ObjectPhase::Hunt → SerpenHuntAndPoke | 4 |  |
| 3 | 1 | 426 | 태그 | ObjectPhase::Setup → 이하 분기. 그 외(None/Assemble) → None(L503) | 4 |  |
| 4 | 14 | 427 | 태그 | BigPlan::SerpenHuntAndPoke 메모리 태그 | 4 |  |
| 5 | 4 | 431 | 미상 | JungleType::Morgard — v23_enemy_object_pressure 로 '반대 오브젝트(에픽) 압박' 판정 | 4 |  |
| 6 | 2 | 433 | 임계 | v23_objective_setup_pressure_line 후보 라인 slice 길이 2 = [Bottom(2), Mid(1)] (@anon.189) | 4 |  |
| 7 | 3 | 434 | 태그 | BigPlan::PassiveLine 메모리 태그 | 4 |  |
| 8 | 5 | 439 | 센티널 | object_buildup 태그 < 5 = Split(position) (니치: Gather=5, Flexible=6) | 4 |  |
| 9 | 5 | 441 | 임계 | JungleType::Serpen — v25_objective_splitter_should_join_contest 인자 | 4 |  |
| 10 | 0 | 443 | 태그 | fallback_line(ctx, LineType::Top) → PassiveLine(line) | 4 |  |
| 11 | 6 | 447 | 태그 | object_buildup == 6 = Flexible → L449~490. 5(Gather) → SerpenHuntAndPoke(L496) | 4 |  |
| 12 | 150000 | 455 | 계수 | d = distance(last_visible_pos, camp).saturating_sub(150000) — 캠프 반경 150000 안이면 0 | 4 |  |
| 13 | 49 | 459 | 임계 | 적 hp*100/max_hp > 49 && can_move >= d && !is_recent_visible → near_serpen_enemy 로 셈(비가시 도달가능 적) | 4 |  |
| 14 | 100 | 459 | 계수 | HP 백분율 | 4 |  |
| 15 | 3 | 463 | 임계 | near_serpen_enemy < 3 이어야 라인 배정 분기 | 4 |  |
| 16 | 102400000001 | 463 | 임계 | 320000^2 + 1 — 내 챔피언↔세르펜 캠프 제곱거리 < 이 값(= 10셀 이내) | 4 |  |
| 17 | 3 | 469 | 임계 | bottom_lead[team] < 3 / mid_lead[team] < 3 | 4 |  |
| 18 | 2 | 471 | 임계 | LineType::Bottom (bottom_count < mid_count 일 때) | 4 |  |
| 19 | 1 | 473 | 태그 | LineType::Mid | 4 |  |
| 20 | 2 | 476 | 임계 | LineType::Bottom (bottom_lead<3 && mid_lead>=3) | 4 |  |
| 21 | 1 | 478 | 태그 | LineType::Mid (bottom_lead>=3 && mid_lead<3) | 4 |  |
| 22 | 1 | 489 | 태그 | shl 1 = tps*2 로 접힘: remain_spawn_tick > move_tick + tps*2 이면 None | 4 | 2 |
| 23 | -1 | 490 | 태그 | Option<BigPlan>::None 태그(0xFFFFFFFFFFFFFFFF) | 4 |  |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 비가시 적 캠프 반경 | serpen.rs:455 | 150000 | 키우면 목격 위치가 더 멀어도 '캠프 안'으로 봐 도달가능 적이 늘어 라인 배정(PassiveLine) 분기 억제 | 4 | 기존 |
| 1 | 비가시 도달가능 적 상한 | serpen.rs:463 | 3 | 올리면 적이 더 있어도 라인 배정 분기로 진입 | 4 | 기존 |
| 2 | 캠프 근접 거리 | serpen.rs:463 | 102400000001 | 320000^2+1. 키우면 더 먼 챔피언도 라인 배정 분기 대상 | 4 | 기존 |
| 3 | 라인 리드 상한 | serpen.rs:469/477 | 3 | bottom_lead/mid_lead < 3 일 때만 그 라인 PassiveLine. 올리면 라인 배정 잦아짐 | 4 | 기존 |
| 4 | 스폰 대기 여유 | serpen.rs:489 | 2 | tps*2. 올리면 스폰이 더 멀어도 None 대신 SerpenHuntAndPoke(미리 이동) | 4 | 기존 |
| 5 | 적 건강 기준 | serpen.rs:459 | 49 | 올리면 도달가능 적 수 감소 | 4 | 기존 |

<details><summary>`callees` 피호출자 11건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | fallback_line | game_ai::plan_legacy::rule_scope::fallback_line | pub | fn(&game_core::GameContext, game_core::LineType) -> game_core::LineType | game-ai\src\plan_legacy\rule_scope.rs:28 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | is_enemy_side | game_core::is_enemy_side | pub | fn(&game_core::GameContext, usize, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:57 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | is_skip_serpen | game_ai::plan_legacy::old::is_skip_serpen | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\serpen.rs:365 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | v23_enemy_object_pressure | game_ai::plan_legacy::team_plan::v23_enemy_object_pressure | pub | fn(&game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:178 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | v23_objective_setup_pressure_line | game_ai::plan_legacy::team_plan::v23_objective_setup_pressure_line | pub | fn(&game_core::PlayerState, &game_core::OperationData, &[game_core::LineType]) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:73 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | v25_objective_splitter_should_join_contest | game_ai::plan_legacy::team_plan::v25_objective_splitter_should_join_contest | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:165 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 1개**: `dist_sq`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m13.ll:7637) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 재료 부재 | top_lead/mid_lead/bottom_lead(AbstractGameWithCache+0x21c0~) 의 의미(타워 진행도? 미니언 리드?)는 이 함수 안에서 확정 불가 — '< 3' 비교만 관측 | 4 |  |
| 1 | 미탐색 | v23_objective_setup_pressure_line / v25_objective_splitter_should_join_contest / fallback_line 내부는 담당 밖(m15.ll 55350 / 56741, m13.ll 54181) | 4 |  |
| 2 | 표기 불가 | L496 에서 camp_pos 를 호출하고 결과를 버림 — 소스가 SerpenHuntAndPokePlan::new(camp) 류인지 표기 불가(IR 상 dead value) | 4 |  |
| 3 | 표기 불가 | L463 `< 102400000001` 은 소스가 `<= 320000^2` 인지 `< 320000^2+1` 인지 표기 불가 | 4 |  |
| 4 | 표기 불가 | L440 `!is_line_phase && position == my_pos` 의 한 줄 내 평가 순서 표기 불가(column 부재) | 4 |  |
| 5 | 미탐색 | vtable 간접호출(tick +0x28 / get_game_mode +0x40)은 C2 대조 불가라 logic 에만 적음 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

