---

### `77` DefenseNexusPlan::sub_plan — 넥서스 방어 플랜의 서브플랜 결정 — DefenseNexus(계속 방어) vs Recall(귀환) 을 넥서스 위기·본진 위험·자기 HP·분수 여부로 판정

| 항목 | 값 |
|---|---|
| id | `defense_nexus__DefenseNexusPlan_sub_plan` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexusNtB2_16DefenseNexusPlan8sub_plan` |
| 소스 | `game-ai\src\plan_legacy\old\defense_nexus.rs:32` |
| IR | `m04.ll` 27345~28107행 |
| 경로·가시성 | `game_ai::plan_legacy::old::DefenseNexusPlan::sub_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d2da10` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::DefenseNexusPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | SubPlan(72B) | %0. 태그만 쓴다: 17=DefenseNexus(payload focus=None,last_gate=0) / 5=Recall | 4 |
| 1 | 1 | self | &DefenseNexusPlan(8B) | %1. 본문에서 읽지 않음(team 필드 미사용 — player.info.team 을 대신 씀) | 4 |
| 2 | 2 | version | usize | %2. `version > 1` 분기 (m04.ll:27981) | 4 |
| 3 | 3 | _rnd | &mut StdRng(320B) | %3. readnone — 미사용 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | %4 | 4 |
| 5 | 5 | data | &OperationData(24B) | %5. cache/context/blackboard | 4 |
| 6 | 6 | goal_data | &GoalData(248B) | %6. heal_commit 만 읽음 | 4 |
| 7 | 7 | _debug | &mut DebugFrameData(224B) | %7. readnone — 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L38: team = player.info.team (<2 아니면 bounds panic); champ = data.cache.player_champion[team][player.info.position].unwrap()
L40: hp_ratio = champ.hp*100 / champ.stat_cached.hp  (max=0 이면 div_by_zero panic — max(.,1) 없음)
L41: (lx,ly,rx,ry) = data.context.map.fountains[team]
L42: in_fountain = lx<=champ.x<=rx && ly<=champ.y<=ry   (x 먼저, 실패 시 y 는 안 봄 — 단락)
L44: nexus = data.cache.nexus[team].unwrap()
L45~47: enemy_bb = data.blackboard[1-team]; top/mid/bottom_front_minion = enemy_bb.{top,mid,bottom}_minion_state.front_minion.and_then(|id| cache.game.get_entity_by_id(id))   [vtable 0x1f0]
L48~50: ally_top/mid/bottom_front_minion = data.blackboard[team].* 동일 방식 — ★결과가 이후 어디에도 안 쓰임(호출만 남음, 관측 무영향)
L51~53: top_near = top_front_minion.is_some_and(|m| m.distance_sq(nexus) < 14400000001); mid_near, bottom_near 동일
L54: existing_lines_weak = valid_lines(context.tutorial).iter().all(|line| match line { Top=>top_near, Mid=>mid_near, Bottom=>bottom_near })
     valid_lines 인라인 표(27764~27816): None/Line/Total→[Top,Mid,Bottom] · First/Bottom→[Bottom] · TopSolo→[Top] · MidSolo→[Mid] · MidBottom→[Mid,Bottom] · JungleOnly→[] (빈 슬라이스면 all()=true)
L59: enemies = data.cache.champions(1-team, context.pool)   (bumpalo Vec<&Entity>, L60 뒤 drop)
L60: nexus_near_enemy_champion = enemies.iter().any(|e| e.distance_sq(nexus) < 14400000001)
     real_danger = nexus_near_enemy_champion  (dbg_value 가 같은 %245 의 not — 추가 조건 없음)
L82: if version > 1 {
       if goal_data.heal_commit { return Recall }            // 28011~28014 → phi 5
       if nexus_is_critical(player,data)[인라인: nexus 존재 && nexus.hp*100/max(nexus.max_hp,1) < 51 && nexus_under_direct_attack(player,data)] { return DefenseNexus }
       if nexus_final_stand(player,data)[인라인: last_stand_flags(player,data) 를 LAST_STAND_MEMO(키 seed,tick) 경유 → .1 (bit 256)] { return DefenseNexus }
     } else {
       if nexus_is_critical(player,data)[동일 인라인] { return DefenseNexus }
     }
L87: danger = (existing_lines_weak && real_danger) || nexus_under_direct_attack(player,data)
     (IR: %292 = (!existing_lines_weak || !real_danger) && !under; 참이면 L95, 거짓이면 L92)
L92: if danger { if hp_ratio > 20 || in_fountain { DefenseNexus } else { Recall } }
L95: else     { if hp_ratio < 31 || (champ.hp < champ.stat_cached.hp && in_fountain) { Recall } else { DefenseNexus } }
L102: DefenseNexus 반환 시 payload: focus=None(+8 tag 0), last_gate=0(+0x18)
```

**`mem` 메모리 접근 24건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | team (m04.ll:27373). <2 아니면 bounds panic | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | as_index → player_champion 인덱스 (27385) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (27388) | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext (27414) | 4 | OK |  |
| 4 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] (27483) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion | r | Option<&Entity> — None 이면 unwrap panic (27389~27394). stride [5 x ptr] | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x170 | nexus[team] | r | Option<&Entity> unwrap (27458~27462); 같은 슬롯을 nexus_is_critical 인라인이 재로드(27987, 28033) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr | r | dyn AbstractGame 데이터 포인터 (27489) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable (27491); 슬롯 0x1f0=get_entity_by_id(27517 등 6회), 0x20=seed, 0x28=tick (28066, 28071 — last_stand_flags 인라인) | 4 | OK |  |
| 9 | Entity | 0x670 | hp | r | champ.hp (27399) / nexus.hp (27993, 28039) | 4 | OK |  |
| 10 | Entity | 0x628 | stat_cached.hp | r | 최대 HP. champ 는 0 이면 div_by_zero panic(27403) / nexus 는 max(.,1) (27999) | 4 | OK |  |
| 11 | Entity | 0x660 | x | r | champ.x(27430) / 미니언·넥서스·적 챔프 distance_sq | 4 | OK |  |
| 12 | Entity | 0x668 | y | r | champ.y(27448) / distance_sq | 4 | OK |  |
| 13 | GameContext | 0x0 | pool | r | &Bump — champions() 인자 (27890) | 4 | OK |  |
| 14 | GameContext | 0x20 | map | r | &MapDef (27418) | 4 | OK |  |
| 15 | GameContext | 0x38 | tutorial | r | TutorialType 1B — valid_lines 인라인 switch (27762) | 4 | OK |  |
| 16 | MapDef | 0x6d70 | fountains | r | (lx,ly,rx,ry) 4×u64 stride 32B: +0 lx, +8 ly, +16 rx, +24 ry (27421~27446) | 4 | OK |  |
| 17 | Blackboard | 0x0 | top_minion_state.front_minion | r | Option<usize> (tag +0, 값 +8). blackboard[1-team] 과 blackboard[team] 둘 다 읽음 (27485, 27570) | 4 | OK |  |
| 18 | Blackboard | 0x28 | mid_minion_state.front_minion | r | Option<usize> (+40/+48) | 4 | OK |  |
| 19 | Blackboard | 0x50 | bottom_minion_state.front_minion | r | Option<usize> (+80/+88) | 4 | OK |  |
| 20 | GoalData | 0xf0 | heal_commit | r | bool — version>1 에서만 (28011) | 4 | OK |  |
| 21 | SubPlan(sret) | 0x0 | tag | w | 28105. phi: %293 경로→17, 나머지(%325/%284/%330)→5 | 4 | OK | 17(DefenseNexus) \| 5(Recall) |
| 22 | SubPlan(sret) | 0x8 | DefenseNexus.focus@tag | w | 28025. 태그 17 경로에서만 | 4 | OK | 0 (None) |
| 23 | SubPlan(sret) | 0x18 | DefenseNexus.last_gate | w | 28027. u8 | 4 | OK | 0 |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 100 | 40 | 계수 | hp_ratio = hp*100 / stat_cached.hp (백분율) | 4 |
| 1 | 14400000001 | 51 | 임계 | 120000^2 + 1 — `distance_sq < 14400000001` = 거리 ≤ 120000(3.75셀). 적 전선 미니언↔넥서스(L51~53) 및 적 챔피언↔넥서스(L60) 근접 판정 공용 | 4 |
| 2 | 1 | 82 | 임계 | `version > 1` 게이트 — v2+ 에서만 heal_commit 조기 Recall 과 nexus_final_stand 검사 | 4 |
| 3 | 51 | 86 | 임계 | nexus_is_critical 인라인: 넥서스 hp_ratio < 51 (=≤50%) | 4 |
| 4 | 256 | 86 | 계수 | last_stand_flags i24 의 bit8 = 튜플 .1 = nexus_final_stand 결과 (nexus_final_stand 본체 m04.ll:57082 와 동일 패턴) | 4 |
| 5 | 20 | 92 | 임계 | 위험 상황: hp_ratio > 20 이면 계속 방어 | 4 |
| 6 | 31 | 95 | 임계 | 비위험 상황: hp_ratio < 31 이면 귀환 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 넥서스 근접 반경(적 전선 미니언·적 챔피언) | defense_nexus.rs:51~53,60 | 14400000001 | 올리면 더 먼 미니언/챔피언도 '넥서스 위협'으로 봐서 existing_lines_weak·real_danger 가 쉽게 참 → danger 분기(L92, 낮은 HP 에도 방어 유지) 진입 증가 | 4 | 기존 |
| 1 | 넥서스 위기 HP% | defense_nexus.rs:86 (nexus_is_critical 인라인) | 51 | 올리면 넥서스가 덜 깎여도 '위기'로 판정 → 직접 피격 시 무조건 DefenseNexus | 4 | 기존 |
| 2 | 위험 시 귀환 HP% 하한 | defense_nexus.rs:92 | 20 | 올리면 위험 상황에서 더 높은 HP 에서도 귀환(단 분수 안이면 계속 방어) | 4 | 기존 |
| 3 | 비위험 시 귀환 HP% | defense_nexus.rs:95 | 31 | 올리면 한가할 때 더 일찍 귀환해 회복 | 4 | 기존 |
| 4 | version 게이트 | defense_nexus.rs:82 | 1 | version≤1 이면 heal_commit 조기 귀환·nexus_final_stand 검사가 없다 | 4 | 기존 |

<details><summary>`callees` 피호출자 14건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | last_stand_flags | game_ai::plan_legacy::old::defense_nexus::last_stand_flags | in:game_ai::plan_legacy::old::defense_nexus | fn(&game_core::PlayerState, &game_core::OperationData) -> (bool, bool, bool) | game-ai\src\plan_legacy\old\defense_nexus.rs:166 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | nexus_final_stand | game_ai::plan_legacy::old::nexus_final_stand | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:190 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | nexus_is_critical | game_ai::plan_legacy::old::nexus_is_critical | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:139 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | nexus_under_direct_attack | game_ai::plan_legacy::old::nexus_under_direct_attack | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:121 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | valid_lines | game_ai::plan_legacy::rule_scope::valid_lines | pub | fn(&game_core::GameContext) -> &[game_core::LineType] | game-ai\src\plan_legacy\rule_scope.rs:13 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 4개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `llvm.umax.i64`, `nexus_near_enemy_champion`, `with  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 2개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:8490) · **형제 9개** (DefenseNexusPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::DefenseNexusPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\defense_nexus.rs:9 | True | fn(&game_ai::plan_legacy::old::DefenseNexusPlan) -> game_ai::plan_legacy::old::DefenseNexusPlan |
| 1 | <game_ai::plan_legacy::old::DefenseNexusPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\old\defense_nexus.rs:9 | True | fn() -> game_ai::plan_legacy::old::DefenseNexusPlan |
| 2 | <game_ai::plan_legacy::old::DefenseNexusPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\defense_nexus.rs:9 | True | fn(&game_ai::plan_legacy::old::DefenseNexusPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::old::DefenseNexusPlan::new | pub | game-ai\src\plan_legacy\old\defense_nexus.rs:15 | True | fn(usize) -> game_ai::plan_legacy::old::DefenseNexusPlan |
| 4 | game_ai::plan_legacy::old::DefenseNexusPlan::goal | pub | game-ai\src\plan_legacy\old\defense_nexus.rs:19 | True | fn(&game_ai::plan_legacy::old::DefenseNexusPlan) -> game_core::BigGoal |
| 5 | game_ai::plan_legacy::old::DefenseNexusPlan::is_end | pub | game-ai\src\plan_legacy\old\defense_nexus.rs:23 | True | fn(&game_ai::plan_legacy::old::DefenseNexusPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 6 | game_ai::plan_legacy::old::DefenseNexusPlan::update | pub | game-ai\src\plan_legacy\old\defense_nexus.rs:27 | True | fn(&mut game_ai::plan_legacy::old::DefenseNexusPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 7 | game_ai::plan_legacy::old::DefenseNexusPlan::sub_plan | pub | game-ai\src\plan_legacy\old\defense_nexus.rs:32 | False | fn(&game_ai::plan_legacy::old::DefenseNexusPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |
| 8 | game_ai::plan_legacy::old::DefenseNexusPlan::next_plan | pub | game-ai\src\plan_legacy\old\defense_nexus.rs:106 | True | fn(&game_ai::plan_legacy::old::DefenseNexusPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L87 한 줄 안의 `(existing_lines_weak && real_danger) \|\| under_direct_attack` 결합 순서/괄호는 IR 극성으로 재구성한 것 — column 정보 부재로 소스 표기(예: `!(a&&b) && !c` 형태일 가능성)는 표기 불가(동작은 확정) | 4 |  |
| 1 | 미탐색 | L48~50 ally_*_front_minion 3개는 get_entity_by_id 호출만 남고 결과 미사용 — 소스에서 미사용 변수(let _ 또는 데드코드)인지, 아니면 debug 출력용이었는지 IR 로는 알 수 없음(행동 무영향은 확정) | 4 |  |
| 2 | 미탐색 | real_danger 가 nexus_near_enemy_champion 과 같은 %245 를 가리키는데 L61~81 사이(소스 20줄)에 IR 명령이 전혀 없음 — 주석/제거된 코드로 추정, rmeta_srcmap 미조회 | 3 |  |
| 3 | 미탐색 | nexus_under_direct_attack(m04.ll:61399) 본문 미독 — 계약만: (&PlayerState,&OperationData)->bool | 4 |  |
| 4 | 미탐색 | last_stand_flags 의 .0/.2 (nexus_last_stand, 세 번째 bool) 의미는 이 함수 범위 밖 — 여기서는 .1 만 소비 | 4 |  |
| 5 | 미탐색 | calls 의 `LocalKey::with` 는 망글 `_RINvMs2_...LocalKey...4with<last_stand_flags::{closure#0}>` 의 짧은 이름 — 실제 내용은 last_stand_flags 메모(nexus_final_stand 인라인) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

