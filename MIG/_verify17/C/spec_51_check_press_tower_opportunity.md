---

### `51` check_press_tower_opportunity — 라인전 이후 머릿수 우위일 때 세 라인 중 '적 타워를 압박할' 라인을 점수로 골라 Option<LineType> 을 낸다

| 항목 | 값 |
|---|---|
| id | `team_plan__check_press_tower_opportunity` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan29check_press_tower_opportunity` |
| 소스 | `game-ai\src\plan_legacy\team_plan.rs:1130` |
| IR | `m09.ll` 52836~54331행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::check_press_tower_opportunity` · **pub** |
| 계층 | 기타 |
| exe | `de40c0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<game_core::LineType>, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 분기 없음. macro_judgement_penalty/bonus 에만 전달(54290·54293) | 4 |
| 1 | 2 | _rnd | &mut StdRng | readnone — 미사용 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930) 읽기 + is_recent_visible·penalty/bonus 인자 | 4 |
| 3 | 4 | data | &OperationData(24B) | +0 cache / +8 context / +0x10 blackboard | 4 |
| 4 | 5 | _team_plan | &TeamPlan | readnone — 미사용 | 4 |
| 5 | 6 | preferred_line | Option<LineType>(i8, None=255) | == 후보 라인이면 점수 +20(54240) | 4 |
| 6 | 7 | _debug | &mut DebugFrameData | readnone — 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn check_press_tower_opportunity(version, _rnd, player, data, _team_plan, preferred_line, _debug) -> Option<LineType>

[L1136] tick = cache.game.tick()/*vtable+0x28*/
        if context.is_line_phase(tick) { return None }
          // 인라인: tutorial ∉ {None,MidBottom,Line,Total} → 라인전 취급(None) ; 아니면 tick < sat_sub(setting.first_spawn_tick(+0x8a8), tps*30) → None
[L1141] team = player.info.team ; healthy = player_champion[team].iter_champions().filter(|a| a.hp*100/a.max_hp > 49).count()   // ★count 클로저 #1
[L1142] need = context.player_count()   // tutorial 별 4/3/2/1 (constants 참조)
[L1143] if healthy < need { return None }
[L1148] enemy_alive = player_champion[1-team] 의 Some 개수
[L1149] if !(healthy > enemy_alive) { return None }          // 건강한 아군이 생존 적보다 많아야
[L1155] lines = rule_scope::valid_lines(tutorial)   // None/Line/Total [Top,Mid,Bottom] · MidBottom [Mid,Bottom] · MidSolo [Mid] · First/Bottom [Bottom] · TopSolo [Top] · JungleOnly []
[L1156~1158] for line in lines {                          // 사전 게이트
           ms = blackboard[team].{top,mid,bottom}_minion_state[line]
           if ms.from_mid < -1999 && (ms.from_mid < -4999 || ms.minion_count < -2) { return None }
        }
        best_score = 0 ; best_line = None
[L1167] for line in lines {
[L1173]   if cache.tower[1-team][line].is_none() && cache.tower2[1-team][line].is_none() { continue }   // 적 타워 다 깨진 라인
[L1178]   ms = blackboard[team].minion_state[line]
[L1179]   if ms.from_mid < 2000 { continue }
[L1184]   if ms.minion_count < 1 { continue }
[L1190]   ally_on = player_champion[team].iter_champions().filter(|a| a.hp*100/a.max_hp > 49 && on_line(a)).count()   // ★count 클로저 #2
            // on_line: Top → is_top_side = (setting.height - a.y) >= a.x ; Bottom → is_bottom_side = (height - y) <= x ;
            //          Mid → is_near_mid_line(context,x,y) || is_near_line(context,x,y,Mid)
[L1198]   if ally_on < min(player_count, 3) { continue }
[L1206]   enemy_on = player_champion[1-team].iter_champions().filter(|e| blackboard[1-team].is_recent_visible(game, player, e) && is_near_line(context, e.x, e.y, line)).count()   // ★count 클로저 #3
[L1210]   if enemy_on > 1 { continue }
[L1217]   push = min((ms.from_mid as i32) / 500, 60)
[L1220]   score = (enemy_on == 0 ? 140 : 100) + ally_on*10 + push
[L1228]   if Some(line) == preferred_line { score += 20 }
[L1233]   t = cache.tower[1-team][line].or(cache.tower2[1-team][line])
          if t.is_some_and(|t| !t.is_tower2())   // is_tower2 = ty==Tower(2) && TowerType > 4
[L1235]      { score += 30 }
[L1238]   if score > best_score { best_score = score ; best_line = Some(line) }
        }
[L1244] threshold = macro_judgement_penalty(version, player)*20 + 130 - macro_judgement_bonus(version, player)*5
[L1246] if best_score < threshold { return None }
[L1251] return best_line

※ 부작용 없음(readnone 인자 3개, 게임 구조체 쓰기 없음). 세 카운트는 IR 에서 5슬롯 언롤.
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext — tutorial·setting·is_near_line 인자 | 4 | OK |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. [team] 은 라인 미니언 상태, [1-team] 은 is_recent_visible | 4 | OK |
| 3 | GameContext | 0x38 | tutorial | r | TutorialType i8. 4곳의 switch(52886·53133·53233·53990) 의 키 — is_line_phase / player_count / valid_lines 인라인 | 4 | OK |
| 4 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |
| 5 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | IR 2216. is_line_phase: tick < sat_sub(first_spawn_tick, tps*30) 면 라인전 중 → None | 4 | OK |
| 6 | GameSetting | 0x12f8 | tick_per_second | r | IR 4856 | 4 | OK |
| 7 | GameSetting | 0x12c0 | height | r | IR 4800. map_regions is_top_side/is_bottom_side: (height - y) >= x / <= x | 4 | OK |
| 8 | AbstractGameWithCache | 0x0 | game.data_ptr (&dyn AbstractGame) | r | vtable+0x28 tick() 호출(52880) + is_recent_visible 인자 | 4 | OK |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2]. [team] 아군 3종 카운트, [1-team] 적 생존·라인 카운트(전부 5슬롯 언롤) | 4 | OK |
| 10 | AbstractGameWithCache | 0x180 | top_tower[] | r | cache + 0x180 + 32*line + 8*enemy_team (53406~53411·54246~54251). 1차 타워 | 4 | OK |
| 11 | AbstractGameWithCache | 0x190 | top_tower2[] | r | +0x10. 둘 다 None 이면 그 라인 skip | 4 | OK |
| 12 | PlayerState | 0x930 | info.team | r | usize. enemy = 1 - team | 4 | OK |
| 13 | Blackboard | 0x0 | top_minion_state / +0x28 mid / +0x50 bottom (BrainMinionParameter 40B) | r | line 태그 0/1/2 → +0/+40/+80 (53270~53273) | 4 | OK |
| 14 | BrainMinionParameter | 0x10 | from_mid | r | i64. 사전 게이트(< -1999 && (< -4999 \|\| count < -2) → None), 라인 게이트(< 2000 skip), 점수(min(from_mid/500, 60)) | 4 | OK |
| 15 | BrainMinionParameter | 0x20 | minion_count | r | i32. 사전 게이트(< -2) / 라인 게이트(< 1 skip) | 4 | OK |
| 16 | Entity | 0x670 | hp | r | usize. hp*100/max_hp > 49 | 4 | OK |
| 17 | Entity | 0x628 | stat_cached.hp | r | max_hp. 0 → div_by_zero 패닉 | 4 | OK |
| 18 | Entity | 0x660 | x | r | u64 | 4 | OK |
| 19 | Entity | 0x668 | y | r | u64 | 4 | OK |
| 20 | Entity | 0x68 | ty@tag | r | EntityType i64. ==2 Tower 일 때만 아래 TowerType 검사(Entity::is_tower2 인라인 54258~54266) | 4 | OK |
| 21 | Entity | 0x128 | ty@Tower.info.ty | r | TowerType i8. >4 (Top2/Mid2/Bottom2) = 2차 타워 → +30 보너스 없음 | 4 | OK |

**`consts` 상수 25건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 30 | 1136 | 계수 | GameSetting::is_line_phase(setting.rs:704) 인라인: tick < first_spawn_tick - tps*30 (첫 에픽 스폰 30초 전까지) 면 라인전 → None. tutorial 이 {0 None,5 MidBottom,7 Line,8 Total} 이 아니면 tick 무관 None(runner.rs:399 → spawn_epic 263) | 4 |
| 1 | 49 | 1141 | 임계 | 아군 hp% > 49 만 '건강한 아군'(healthy). L1190 라인 위 아군 카운트도 같은 조건 | 4 |
| 2 | 100 | 1141 | 계수 | hp*100/max_hp 백분율 | 4 |
| 3 | 4 | 1142 | 임계 | GameContext::player_count(runner.rs:295) 인라인의 tutorial 별 값: None/Line/Total→4, MidBottom(5)→3, First(1)/Bottom(3)→2, TopSolo(2)/MidSolo(4)/JungleOnly(6)→1. healthy < 이 값 → None | 4 |
| 4 | 1 | 1148 | 임계 | enemy_team = 1 - team ; 적 생존수 = player_champion[enemy] 의 Some 개수 | 4 |
| 5 | -1999 | 1158 | 임계 | 사전 게이트: 어느 유효 라인이든 from_mid < -1999 이고 (from_mid < -4999 또는 minion_count < -2) 면 None — 우리 쪽으로 깊게 밀린 라인이 있으면 압박 금지 | 4 |
| 6 | -4999 | 1158 | 임계 | 위 게이트의 깊은 밀림 임계 | 4 |
| 7 | -2 | 1158 | 임계 | 위 게이트의 minion_count 임계(음수 = 적 미니언 우세로 추정) | 5 |
| 8 | 2000 | 1179 | 임계 | 후보 라인 조건: from_mid >= 2000 (적 쪽으로 밀린 웨이브) | 4 |
| 9 | 1 | 1184 | 임계 | 후보 라인 조건: minion_count >= 1 | 4 |
| 10 | 3 | 1198 | 태그 | player_count 를 3 으로 캡한 값(None/Line/Total/MidBottom→3, First/Bottom→2, Solo류→1). 라인 위 건강한 아군 < 이 값 → skip | 4 |
| 11 | 1 | 1210 | 임계 | 라인 위 시야 적 > 1 (2명 이상) 이면 skip | 4 |
| 12 | 500 | 1217 | 계수 | push = min(from_mid / 500, 60) — 밀림 정도 점수(i32 sdiv) | 4 |
| 13 | 60 | 1217 | 임계 | push 점수 상한 | 4 |
| 14 | 140 | 1220 | 산출값 | 라인 위 적 0명이면 기본점 140 | 4 |
| 15 | 100 | 1220 | 계수 | 라인 위 적 1명이면 기본점 100 | 4 |
| 16 | 10 | 1225 | 미상 | 라인 위 건강한 아군 1명당 +10 | 4 |
| 17 | 20 | 1228 | 계수 | line == preferred_line 이면 +20 | 4 |
| 18 | 2 | 1233 | 태그 | EntityType 태그 2 = Tower (is_tower2 인라인) | 4 |
| 19 | 4 | 1233 | 태그 | TowerType 태그 > 4 (Top2/Mid2/Bottom2) = 2차 타워. 남은 적 타워(1차 우선, 없으면 2차)가 2차 타워가 아니면(=1차) +30 | 4 |
| 20 | 30 | 1235 | 계수 | 1차 타워 보너스 +30 | 4 |
| 21 | 20 | 1244 | 계수 | threshold = macro_judgement_penalty*20 + 130 - macro_judgement_bonus*5 | 4 |
| 22 | 130 | 1244 | 계수 | threshold 기본값 | 4 |
| 23 | -5 | 1245 | 계수 | bonus 1당 threshold -5 | 4 |
| 24 | -1 | 1246 | 산출값 | Option<LineType>::None 표식(i8 -1 = 255). best_score < threshold 면 None | 4 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 라인전 종료 시점 | team_plan.rs:1136 ← setting.rs:704 (IR m09.ll:52902 `mul i64 %25, 30`) | 30 | 첫 에픽 스폰 30초 전부터 압박 판정 시작. 올리면 더 이른 시점부터 타워 압박을 시도 | 4 | 기존 |
| 1 | 후보 라인의 웨이브 밀림 하한 | team_plan.rs:1179 (IR 53455 `icmp slt i64 %242, 2000`) | 2000 | from_mid ≥ 2000 인 라인만 후보. 내리면 덜 밀린 라인도 후보가 되어 압박이 잦아진다 | 4 | 기존 |
| 2 | 라인 위 적 허용 수 | team_plan.rs:1210 (IR 54193 `icmp samesign ugt i64 %516, 1`) | 1 | 시야에 잡힌 적이 라인에 2명 이상이면 그 라인 포기. 올리면 다수 적 앞에서도 압박 | 4 | 기존 |
| 3 | 라인 위 아군 최소 | team_plan.rs:1198~1199 (IR 54026 `icmp samesign ult i64 %433, %437`, 일반 게임 3) | 3 | 건강한 아군 3명이 라인에 있어야 후보. 튜토리얼은 player_count 로 자동 축소 | 4 | 기존 |
| 4 | 채택 임계(threshold) | team_plan.rs:1244~1246 (IR 54291~54297) | 130 | best_score ≥ penalty*20 + 130 - bonus*5 여야 Some. 130 을 내리면 압박이 잦아지고, penalty(판단력 낮음) 가 크면 더 어려워진다 | 4 | 기존 |
| 5 | 점수 구성 | team_plan.rs:1217~1235 | 140/100 + 10*ally + min(from_mid/500,60) + 20(preferred) + 30(1차 타워) | 적 0명 기본 140 은 그 자체로 threshold 130 을 넘어 아군 3명·밀림 0 이어도 채택. 적 1명(100)이면 아군 3명(+30)+push 로 넘겨야 한다 | 4 | 기존 |
| 6 | 사전 게이트(우리 라인이 밀렸을 때 압박 금지) | team_plan.rs:1158 (IR 53364~53371) | -1999 | 어떤 라인이든 from_mid < -1999 이고 (< -4999 또는 minion_count < -2) 면 전체 None. 값을 더 음수로 내리면 우리 라인이 꽤 밀려도 압박한다 | 4 | 기존 |

<details><summary>`callees` 피호출자 16건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | check_press_tower_opportunity | game_ai::plan_legacy::team_plan::check_press_tower_opportunity | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<game_core::LineType>, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan.rs:1129 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_tower2 | game_core::EntityType::is_tower2 | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | macro_judgement_bonus | game_ai::plan_legacy::team_plan::macro_judgement_bonus | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:20 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | macro_judgement_penalty | game_ai::plan_legacy::team_plan::macro_judgement_penalty | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:15 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 11 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 12 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | valid_lines | game_ai::plan_legacy::rule_scope::valid_lines | pub | fn(&game_core::GameContext) -> &[game_core::LineType] | game-ai\src\plan_legacy\rule_scope.rs:13 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 3개**: `first_spawn_tick`, `on_line`, `sat_sub`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m09.ll:9864, m09.ll:14595, m09.ll:31580, m09.ll:32380, m09.ll:37283) · **형제 0개** 

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 지시의 exe 별도 인스턴스 3개(0xdeb970/0xdeb9f0/0xdec010 = Iterator::count 클로저)는 IR 에서는 전부 본체에 인라인(fnparts: define 1개, 5슬롯 언롤)이라 aux 로 줄 범위를 선언할 수 없다. 세 클로저의 술어는 logic 의 ★#1(L1141 healthy)·#2(L1190 ally_on_line)·#3(L1206 enemy_on_line) 에 그대로 적었다 — exe 함수 3개 ↔ 이 세 술어 대응은 순서 추정(미검증) | 4 |  |
| 1 | 표기 불가 | GameContext::player_count(runner.rs:295) 의 실제 반환값은 인라인·상수접힘 후 비교식만 남았다. 관측값(일반 4 / MidBottom 3 / First·Bottom 2 / Solo류 1)이 'player_count()-1' 인지 'player_count()' 자체인지 IR 로 구분 불가(표기 불가, 동작은 확정). L1198 은 같은 값을 3 으로 캡(일반 3) | 4 |  |
| 2 | 미탐색 | GameContext::is_line_phase(runner.rs:399) 의 tutorial 분기: {0,5,7,8} 만 tick 검사로 가고 나머지는 None — 소스가 `!spawn_epic → true` 인지 다른 술어인지는 인라인이라 확정 못 함. check_epic_giveup 의 morgard_exists(1..=6 false) 와 5(MidBottom) 취급이 다른 것은 관측 사실 | 4 |  |
| 3 | 미탐색 | BrainMinionParameter.from_mid 의 단위·부호 의미(양수 = 적 쪽으로 밀림)는 게이트 방향(≥2000 이 후보, ≤-2000 이 위험)에서 추정. minion_count 가 음수가 되는 의미(적 미니언 우세?)도 추정 | 5 |  |
| 4 | 미탐색 | map_regions::is_near_line(context, x, y, LineType) / is_near_mid_line 내부(반경)는 game_core 미열람 | 4 |  |
| 5 | 미탐색 | macro_judgement_penalty / macro_judgement_bonus 내부·범위 미열람(i32) | 4 |  |
| 6 | 표기 불가 | L1233 의 `t = tower.or(tower2)` 는 `select null? tower2 : tower` 로 읽음(54252). Option::or 인지 다른 표현인지는 표기 불가 | 4 |  |
| 7 | 미탐색 | Option<LineType> None 이 255 인 것은 DWARF Variant0.DISCR_EXACT=255 로 확정. tcxdict 가 TeamPlan.wave_priority_clear_line 을 '니치 1B' 로만 표시해 값은 안 주므로 DWARF 를 근거로 삼았다 | 3 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | constants 의 100·20·1·4 는 서로 다른 의미로 여러 줄에 겹친다 — 각각 별도 항목으로 적었다 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

