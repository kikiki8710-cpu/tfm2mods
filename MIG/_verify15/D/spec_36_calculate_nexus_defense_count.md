---

### `36` calculate_nexus_defense_count — 넥서스 근처 적 챔프 수 + 라인 열세/에픽 가산 vs 구조물을 치는 적 미니언 수 바닥값 중 큰 쪽 = 넥서스 수비 필요 인원

| 항목 | 값 |
|---|---|
| id | `handler__calculate_nexus_defense_count` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler29calculate_nexus_defense_count` |
| 소스 | `game-ai\src\plan_legacy\handler.rs:2328` |
| IR | `m13.ll` 56190~56961행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::calculate_nexus_defense_count` · **pub** |
| 계층 | 플랜 핸들러 |
| exe | `15126528` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> usize
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | i64 %0. 본문에서 안 읽힘(분기 없음) | 4 |
| 1 | 2 | rnd | &mut StdRng | ptr %1 readnone — 본문에서 안 씀 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽는다 | 4 |
| 3 | 4 | data | &OperationData(24B) | cache(+0x0)·context(+0x8) 사용 | 4 |
| 4 | 5 | debug | &mut DebugFrameData | ptr %4 readnone — 본문에서 안 씀 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn calculate_nexus_defense_count(version, rnd, player, data, debug) -> usize

[L2329] context = data.context(+0x8)
        enemy_has_epic = if rule_scope::morgard_exists(context)          // = context.tutorial(+0x38).spawn_epic()
                                                                          //   = !(tutorial-1 <u 6) ⇔ tutorial ∈ {None0, Line7, Total8}
          {
            let GameMode::Moba(m) = data.cache.game.get_game_mode()      // vtable+0x40, 태그 0 아니면 unwrap_failed 패닉
            m.remain_epic_time(1 - player.info.team(+0x930)) != 0        // MobaMode+0x240 [enemy]
          } else { false }
[L2330] team = player.info.team ; nexus = data.cache.nexus(+0x170)[team].unwrap()   // 없으면 패닉

[L2333] lines = rule_scope::valid_lines(context.tutorial)      // 스위치(rule_scope.rs:14~19):
          // First1/Bottom3 → [Bottom] · TopSolo2 → [Top] · MidSolo4 → [Mid] · MidBottom5 → [Mid,Bottom]
          // JungleOnly6 → [] · None0/Line7/Total8 → [Top,Mid,Bottom]
[L2334] weak_lead_state = lines.iter().copied().any(|line| data.cache.line_lead(team, line) == 0)
          // = cache[0x21c0 + line*16 + team*8] == 0  ([top|mid|bottom]_lead[team])

[L2337] enemy_minions = data.cache.minions(1 - team, context.pool)
[L2338] very_weak_lead_state = enemy_minions.iter().any(|m| distance_sq(m, nexus) < 22500000001)   // <= 150000

[L2340] enemy_champs = data.cache.champions(1 - team, context.pool)
[L2341] near_nexus_enemy_count = enemy_champs.iter().filter(|c|
            distance_sq(c, nexus) < 90000000001                          // <= 300000
            && game.is_visible(team, c.id(+0x5c0))                        // vtable+0xf8
          ).count()

[L2343] champ_based = near_nexus_enemy_count +
          if enemy_has_epic { if very_weak_lead_state { 2 } else { weak_lead_state as usize } }
          else               { very_weak_lead_state as usize }
          // IR: %276 = enemy_has_epic ? (very_weak ? 2 : weak) : very_weak

[L2361] structure_minions = data.cache.iter_minions(1 - team)            // Chain 이터레이터(56B)
[L2362]   .filter(|m| {                                                  // ★closure#3 = aux m12.ll 42894~42979
            m.ty(+0x68) == 1 /*Minion*/ && m.ty.Minion.info.nearest_enemy(+0x88).is_some()
[L2363]     && match game.get_entity_by_id(m.nearest_enemy.id(+0x90)) {   // vtable+0x1f0
[L2364]          Some(e) => match e.ty(+0x68) { 3 /*Nexus*/ => true,
                                                2 /*Tower*/ => e.ty.Tower.info.ty(+0x128) ∈ {TwinA3, TwinB4},
                                                _ => false },
                 None => false }
          }).count()
[L2369] minion_floor = if structure_minions > 5 { 2 } else if structure_minions != 0 { 1 } else { 0 }
[L2370] return max(minion_floor, champ_based)   // llvm.umax

※ version/rnd/debug 는 안 쓰인다. 부작용 없음(bump Vec 2개 할당·drop 뿐).
```

**`mem` 메모리 접근 19건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |
| 2 | GameContext | 0x0 | pool | r | &Bump. minions()/champions() 의 할당자(gep 없이 ctx 선두 load) | 4 | OK |
| 3 | GameContext | 0x38 | tutorial | r | TutorialType(i8). ①rule_scope::morgard_exists = spawn_epic() 인라인(runner.rs:263): (tutorial-1) <u 6 이면 false ②rule_scope::valid_lines 스위치(L2333) | 4 | OK |
| 4 | PlayerState | 0x930 | info.team | r | usize. 아군 팀 인덱스. >=2 이면 panic_bounds_check | 4 | OK |
| 5 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame data) | r | vtable 호출: +0x40 get_game_mode / +0xf8 is_visible / +0x1f0 get_entity_by_id(클로저 내) | 4 | OK |
| 6 | AbstractGameWithCache | 0x8 | game vtable | r |  | 4 | OK |
| 7 | AbstractGameWithCache | 0x170 | nexus[team] | r | [Option<&Entity>;2] stride 8. None 이면 unwrap_failed 패닉 | 4 | OK |
| 8 | AbstractGameWithCache | 0x21c0 | top_lead/mid_lead/bottom_lead [line][team] | r | [usize;2]×3 (line*16 + team*8). AbstractGameWithCache::line_lead 인라인. == 0 이면 그 라인 '약한 상태' | 4 | 오귀속(사전은 다른 필드를 준다) |
| 9 | MobaMode | 0x240 | epic_minion_buff_time[enemy] | r | usize. MobaMode::remain_epic_time(1-team) 인라인(game.rs:210). != 0 → enemy_has_epic | 4 | OK |
| 10 | Entity(nexus / 적 미니언 / 적 챔프) | 0x660 | x | r | distance_sq 계산 | 4 | OK |
| 11 | Entity(nexus / 적 미니언 / 적 챔프) | 0x668 | y | r | distance_sq 계산 | 4 | OK |
| 12 | Entity(적 챔프) | 0x5c0 | id | r | is_visible(team, id) 인자 | 4 | OK |
| 13 | Entity(적 미니언, aux 클로저) | 0x68 | ty@tag | r | EntityType 태그. ==1(Minion) 이어야 함. 대상 엔티티 쪽은 3(Nexus)/2(Tower) 분기 | 4 | OK |
| 14 | Entity(적 미니언, aux 클로저) | 0x88 | ty@Minion.info.nearest_enemy@tag | r | Option<usize> 태그(EntityType+0x20 = Minion+0x18). 1=Some 이어야 함 | 4 | OK |
| 15 | Entity(적 미니언, aux 클로저) | 0x90 | ty@Minion.info.nearest_enemy@Some.0 | r | usize 대상 엔티티 id → get_entity_by_id | 4 | OK |
| 16 | Entity(미니언 대상, aux 클로저) | 0x128 | ty@Tower.info.ty (TowerType) | r | i8. (v-3) <u 2 ⇒ 3 TwinA / 4 TwinB 만 통과 | 4 | OK |
| 17 | Vec<&Entity>(bumpalo,32B) minions/champions | 0x0 | ptr | r |  | 4 | 확인불가(tcx 사전에 타입 없음) |
| 18 | Vec<&Entity>(bumpalo,32B) minions/champions | 0x18 | len | r |  | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 14건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 6 | 2329 | 임계 | morgard_exists = tutorial.spawn_epic(): (tutorial-1) <u 6 (=1..=6, First~JungleOnly) 이면 에픽 없음 → enemy_has_epic=false. 0 None/7 Line/8 Total 만 에픽 판 (`add nsw i8 %21, -1`) | 4 |
| 1 | -1 | 2329 | 미상 | 위 범위검사용 tutorial-1 | 4 |
| 2 | 0 | 2329 | 태그 | ①get_game_mode() 태그 0 = Moba 만 허용(아니면 unwrap_failed 패닉) ②remain_epic_time(enemy) != 0 → enemy_has_epic | 4 |
| 3 | 1 | 2329 | 태그 | enemy = 1 - team | 4 |
| 4 | 2 | 2329 | 임계 | team/enemy 인덱스 상한(panic_bounds_check len=2) | 4 |
| 5 | 0 | 2334 | 태그 | weak_lead_state = valid_lines(tutorial) 중 하나라도 line_lead[line][team] == 0 | 4 |
| 6 | 22500000001 | 2338 | 임계 | 150000²+1 — very_weak_lead_state: 적 미니언이 우리 넥서스에서 150000(≈4.7셀) 이내에 하나라도 있는가 (`icmp ult dist², 22500000001`) | 4 |
| 7 | 90000000001 | 2341 | 임계 | 300000²+1 — near_nexus_enemy_count: 우리 넥서스에서 300000(≈9.4셀) 이내 + is_visible 인 적 챔프 수 | 4 |
| 8 | 2 | 2343 | 임계 | champ_based 가산: enemy_has_epic && very_weak → +2 (그 외 weak/very_weak 는 +1, 없으면 +0) | 4 |
| 9 | 5 | 2369 | 임계 | structure_minions > 5 → minion_floor=2; 1..=5 → 1; 0 → 0 | 4 |
| 10 | 1 | 2362 | 태그 | aux 클로저: 미니언 ty 태그 1(Minion) 이고 nearest_enemy 태그 1(Some) 이어야 대상 조회 | 4 |
| 11 | 3 | 2364 | 태그 | aux 클로저: 대상 ty 태그 3 = Nexus → 구조물 미니언으로 센다 (본문의 `shl i64 %92, 3` 은 &Entity 포인터 stride ×8 이라 이 상수와 무관) | 4 |
| 12 | 2 | 2364 | 태그 | aux 클로저: 대상 ty 태그 2 = Tower 이면 TowerType 검사로 | 4 |
| 13 | -3 | 2364 | 미상 | aux 클로저: (TowerType - 3) <u 2 ⇒ TwinA(3)/TwinB(4) 쌍둥이 타워만 구조물로 인정(tower.rs:91 인라인) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 매우 약한 라인 판정 반경(적 미니언↔우리 넥서스) | handler.rs:2338 (IR m13.ll:56492 `icmp ult i64 %121, 22500000001`) | 22500000001 | 150000². 올리면 더 먼 미니언도 '넥서스 코앞'으로 봐 very_weak 가 자주 켜져 수비 인원(+1/+2)이 늘고, 내리면 진짜 넥서스에 붙었을 때만 | 4 | 기존 |
| 1 | 넥서스 근처 적 챔프 반경 | handler.rs:2341 (IR m13.ll:56592 `icmp ult i64 %156, 90000000001`) | 90000000001 | 300000². 올리면 더 먼(보이는) 적 챔프까지 1:1 대응 인원으로 세서 수비가 두터워지고, 내리면 넥서스 바로 근처만 | 4 | 기존 |
| 2 | 에픽 버프 + 넥서스 근접 미니언 가산 | handler.rs:2343 (IR m13.ll:56920 `select i1 %125, i64 2, ...`) | 2 | 적이 에픽 버프를 갖고 미니언이 넥서스에 붙어 있으면 +2. 올리면 에픽 푸시에 더 많이 수비, 1 로 내리면 일반 푸시와 같게 | 4 | 기존 |
| 3 | 구조물 공격 미니언 수 바닥값 임계 | handler.rs:2369 (IR m13.ll:56942 `icmp ugt i64 %282, 5`) | 5 | 넥서스/쌍둥이타워를 치는 적 미니언이 6+ 이면 최소 2명, 1~5 면 최소 1명. 내리면 소수 미니언에도 2명이 붙는다 | 4 | 기존 |
| 4 | 구조물로 인정하는 타워 종류 | handler.rs:2364 (aux m12.ll:42952 `add nsw i8 %30, -3` / `icmp ult i8 %31, 2`) | TwinA(3)/TwinB(4) | 외곽 타워(Top/Mid/Bottom/…2)를 치는 미니언은 안 센다. 범위를 넓히면 라인 타워 푸시에도 넥서스 수비 인원이 잡힌다 | 4 | 기존 |

<details><summary>`callees` 피호출자 25건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | calculate_nexus_defense_count | game_ai::plan_legacy::handler::calculate_nexus_defense_count | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> usize | game-ai\src\plan_legacy\handler.rs:2328 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 11 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | line_lead | game_core::AbstractGameWithCache::<'a, 'b>::line_lead | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, game_core::LineType) -> usize | game-core\src\simulation.rs:1928 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | minions | game_core::AbstractGameWithCache::<'a, 'b>::minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1853 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | morgard_exists | game_ai::plan_legacy::rule_scope::morgard_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | remain_epic_time | game_core::MobaMode::remain_epic_time | pub | fn(&game_core::MobaMode, usize) -> usize | game-core\src\simulation\game.rs:210 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | spawn_epic | game_core::TutorialType::spawn_epic | pub | fn(&game_core::TutorialType) -> bool | game-core\src\simulation\game\runner.rs:262 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 21 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 22 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 23 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 24 | valid_lines | game_ai::plan_legacy::rule_scope::valid_lines | pub | fn(&game_core::GameContext) -> &[game_core::LineType] | game-ai\src\plan_legacy\rule_scope.rs:13 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 2개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `size_hint`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 11곳** (m09.ll:8592, m09.ll:9179, m09.ll:30961, m09.ll:31803, m09.ll:32572, m09.ll:33154, m09.ll:34188, m09.ll:34618, m09.ll:35515, m09.ll:36323, m13.ll:32809) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | line_lead(cache+0x21c0…) 값의 의미 체계(0 이 '열세'인지 '푸시 안 됨'인지)는 game_core 쪽 계산을 안 읽어 미확인 — 이 함수는 `== 0` 을 '약한 라인' 으로 쓴다는 사실만 확정. 사이드 관측: 같은 모듈 v23_objective_setup_pressure_line 은 `lead > 2` 를 게이트로 씀(0~2+ 정수 척도로 추정) | 5 |  |
| 1 | 미탐색 | get_game_mode() 가 Moba 가 아닐 때(SingleLane/DeathMatch) unwrap_failed 로 패닉하는 경로가 실제로 도달 가능한지(호출자가 튜토리얼/모드 게이트를 거는지) 미확인 | 4 |  |
| 2 | 미탐색 | is_visible(vtable+0xf8)·get_entity_by_id(vtable+0x1f0)·get_game_mode(vtable+0x40) 이름은 divtable(ExpectedGame 정적 vtable) 기준 — 런타임 구현체는 다를 수 있음 | 3 |  |
| 3 | 미탐색 | L2368 근처의 size_hint 호출 6건은 Chain::count 의 상한 assume 계산(llvm.assume)일 뿐 판정에 기여하지 않는다 — calls 에는 'size_hint' 로만 등재 | 4 |  |
| 4 | 미탐색 | fnparts 조각 중 m11.ll 25471·26245·26955, m06.ll 7832 (이터레이터 fold 인스턴스)와 m12.ll 40879(동일 내용의 두 번째 call_mut 복제본)는 aux 에 넣지 않았다 — 42894 본과 명령 단위 동일 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

