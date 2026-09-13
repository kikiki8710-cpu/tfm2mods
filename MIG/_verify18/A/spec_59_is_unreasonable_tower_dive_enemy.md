---

### `59` is_unreasonable_tower_dive_enemy — 타워 안의 적 챔프에게 다이브(타워 진입 필요)하는 게 무리한지 — HP%·킬각·근처 머릿수 3조건 중 하나도 못 채우면 true

| 항목 | 값 |
|---|---|
| id | `fight_model__is_unreasonable_tower_dive_enemy` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model32is_unreasonable_tower_dive_enemy` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:792` |
| IR | `m10.ll` 49147~49399행 |
| 경로·가시성 | `game_ai::plan_legacy::old::is_unreasonable_tower_dive_enemy` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `e0bf70` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | _version | i64(usize) | 본문에서 미사용 (DILocalVariable `_version`, m10.ll:118677) | 4 |
| 1 | 1 | player | &PlayerState(2528B) | 판정 주체. info.team(+0x930)·info.position(+0x9c0)·info.id(+0x928) 읽음 | 4 |
| 2 | 2 | data | &OperationData(24B) | +0x0 cache · +0x8 context · +0x10 blackboard 전부 사용 | 4 |
| 3 | 3 | target | &Entity(1728B) | 다이브 대상 후보. 적 팀 챔피언이 아니면 즉시 false | 4 |
| 4 | 4 | with_declared_dive | bool | 다이브 콜이 선언된 상태인지. true 면 조건 C(HP 44%/51% + 머릿수 우위)가 추가로 '합리적' 판정을 열어줌 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn is_unreasonable_tower_dive_enemy(_version, player, data, target, with_declared_dive) -> bool
  team = player.info.team; enemy = 1 - team                                   // L799
  if !(target.team == TeamType::Player(enemy) && target.ty is Champion(13)) → return false   // L799 (TeamType::ne 인라인, entity.rs:1127)
  game = data.cache.game
  if !(game.tick() < setting.tower_attack_disable_tick) → return false        // L801 (타워 꺼진 뒤엔 다이브 아님)
  champ = data.cache.player_champion[team][player.info.position]; None → return false   // L805 (team>=2 패닉)
  approach_range = max_range_cached(data, champ, target) + 25000              // L809
  (x2,y2) = (target.x, target.y)                                              // L811
  needs_tower_entry = can_tower_focused_when_battle(ctx, cache, player, x2, y2, approach_range)   // L811 — 대상 주변이 타워권
                      && !can_trace_without_tower(ctx, cache, player.info.id, x2, y2, approach_range)   // L812 — 타워 밖 자리가 없음 (dbg: DW_OP xor -1, 분기 방향 일치)
  if !needs_tower_entry → return false                                        // L813 (타워 진입 없이 닿으면 '무리' 아님)
  my_hp_ratio     = champ.hp*100 / max(champ.stat_cached.hp,1)               // L817
  target_hp_ratio = target.hp*100 / max(target.stat_cached.hp,1)             // L818
  ready_damage    = ready_damage_to_target(ctx, champ, target)               // L819
  in_range        = dist²(champ, target) <= approach_range²                  // L820 (|dx|²+|dy|² ule range²)
  allies_near_target  = cache.player_champion[team].iter().flatten()          // L821~823 (aux1) — ★자기 자신 포함
                          .filter(|a| dist²(a,target) < 140000²+1).count()
  enemies_near_target = cache.player_champion[enemy].iter().flatten()        // L824~827 (aux2)
                          .filter(|e| dist²(e,target) < 140000²+1
                                   && data.blackboard[enemy].is_recent_visible(game, player, e)).count()   // L826 — 최근 시야에 잡힌 적만 셈
  ally_adv = allies_near_target > 1 && allies_near_target > enemies_near_target   // L828
  A = ready_damage >= target.hp && my_hp_ratio > 24                           // L830  즉시 킬각 + 내 HP 25%+
  B = target_hp_ratio < 26 && my_hp_ratio > 34 && (in_range || ally_adv)      // L831  적 빈사 + 내 HP 35%+ + (사거리 안 or 머릿수 우위)
  C = with_declared_dive && my_hp_ratio > 44 && target_hp_ratio < 51 && ally_adv   // L832  콜 선언 시: 내 45%+ · 적 50%- · 머릿수 우위
  return !(A || B || C)                                                       // L834~835

  주석 근거(_docs/game_ai.txt:279~284): 이 함수는 'HP 임계값 기반' 실제 교전 판단축이고, 콜 축(check_favorable_engage_formation)과 기준이 달라 propose/cancel 반복이 생겨 v33 에서 단일 viability 함수가 따로 도입됐다 — 즉 이 함수는 legacy(old) 경로.
```

**`mem` 메모리 접근 17건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | team, enemy_team = 1 - team (m10.ll:49166~49169). 본체·aux2(56882) 양쪽 | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | Position 태그(0..4) → player_champion[team][position] 인덱스 (m10.ll:49221~49223) | 4 | OK |
| 2 | PlayerState | 0x928 | info.id | r | can_trace_without_tower 의 id 인자 (m10.ll:49248~49249) | 4 | OK |
| 3 | Entity | 0x0 | team@tag | r | target.team 판별자: 0=Player 여야 함 (TeamType Direct, m10.ll:49170~49172) | 4 | OK |
| 4 | Entity | 0x8 | team@Player.0 | r | target 의 팀 번호 == enemy_team 비교 (m10.ll:49183~49184) | 4 | OK |
| 5 | Entity | 0x68 | ty@tag | r | EntityType 태그 == 13(Champion) (m10.ll:49185~49187) | 4 | OK |
| 6 | Entity | 0x660 | x | r | target·champ·아군/적 챔프 좌표 (본체 49236·49269, aux1 56671·56690, aux2 56845·56855) | 4 | OK |
| 7 | Entity | 0x668 | y | r | 동상 | 4 | OK |
| 8 | Entity | 0x670 | hp | r | champ.hp(49261)·target.hp(49265) — 현재 HP | 4 | OK |
| 9 | Entity | 0x628 | stat_cached.hp | r | 최대 HP (49263·49267). `max(.,1)` 로 0 나눗셈 방지 (cmp.rs max 인라인, m10.ll:49331·49335) | 4 | OK |
| 10 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m10.ll:49189) | 4 | OK |
| 11 | OperationData | 0x8 | context | r | &GameContext (m10.ll:49197~49198) | 4 | OK |
| 12 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] → aux2 에서 `[1-team]` 인덱스(744B stride, m10.ll:56898~56900). 클로저 캡처 +0x28 (m10.ll:49362) | 4 | OK |
| 13 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame (data ptr +0x0 / vtable +0x8). vtable+0x28 = tick (divtable) (m10.ll:49190~49200) | 3 | OK |
| 14 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [[Option<&Entity>;5];2]. 본체 = 내 챔프(49224~49227), aux1 = 아군 5칸 순회, aux2 = 적 5칸 순회(49341) | 4 | OK |
| 15 | GameContext | 0x8 | setting | r | &GameSetting (m10.ll:49202~49203) | 4 | OK |
| 16 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | `game.tick() < 이 값` 이어야 진행 — 타워가 공격을 멈추는(꺼지는) 틱 이후엔 다이브 개념 자체가 없어 false (m10.ll:49205~49207) | 4 | OK |

**`consts` 상수 11건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 13 | 799 | 태그 | EntityType::Champion 메모리태그(tcxdict --enum: idx13=discr13=tag13, Direct). target.ty 가 챔피언인지 (m10.ll:49187) | 3 |
| 1 | 25000 | 809 | 계수 | approach_range = max_range_cached(champ→target) + 25000 (사거리 + 여유 25k, 셀 0.78개) — 타워권 판정·in_range 판정 반경 (m10.ll:49234) | 4 |
| 2 | 100 | 817 | 계수 | HP 백분율 환산: hp*100 / max(max_hp,1) (m10.ll:49330·49334) | 4 |
| 3 | 1 | 817 | 임계 | `max(max_hp, 1)` 0 나눗셈 가드 (llvm.umax, m10.ll:49331·49335). ⚠ `allies_near_target > 1`(L828, m10.ll:49371)의 1 = '나 혼자가 아니어야' 하한도 같은 값 | 4 |
| 4 | 24 | 830 | 임계 | 조건 A: my_hp_ratio > 24 (= 내 HP 25% 이상) && ready_damage >= target.hp (지금 킬각) (m10.ll:49375~49376) | 4 |
| 5 | 26 | 831 | 임계 | 조건 B: target_hp_ratio < 26 (적 HP 25% 이하) (m10.ll:49379) | 4 |
| 6 | 34 | 831 | 임계 | 조건 B: my_hp_ratio > 34 (내 HP 35% 이상) — B 는 추가로 (in_range \|\| 아군 머릿수 우위) 필요 (m10.ll:49380) | 4 |
| 7 | 44 | 832 | 임계 | 조건 C(with_declared_dive 전용): my_hp_ratio > 44 (내 HP 45% 이상) (m10.ll:49385) | 4 |
| 8 | 51 | 832 | 임계 | 조건 C: target_hp_ratio < 51 (적 HP 50% 이하) && 아군 머릿수 우위 (m10.ll:49387) | 4 |
| 9 | 19600000001 | 822 | 미상 | 140000² + 1 — '대상 근처' 반경(거리 ≤ 140000 = 4.375셀). aux1(아군, m10.ll:56719)·aux2(적, 56891) 공통 `dist² < 이 값` | 4 |
| 10 | 2 | 805 | 임계 | team/enemy_team 이 2 미만이어야 하는 bounds 상한(팀 수). 본체 49213·49306, aux2 56897 — 배열 길이지 판정 임계 아님(기록용) | 4 |

**`knobs` 조정점 8건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 접근 사거리 여유 | fight_model.rs:809 | 25000 | 올리면 approach_range↑ → 타워권 판정 지점이 넓어져 needs_tower_entry 가 더 자주 참(=무리 판정 진입↑)이면서 in_range 도 쉬워짐(B 완화). 양방향 효과라 단독 조정 주의 | 4 | 기존 |
| 1 | 조건 A 내 HP 하한 | fight_model.rs:830 | 24 | 올리면 킬각이어도 더 건강해야 다이브 허용(보수적) | 4 | 기존 |
| 2 | 조건 B 적 HP 상한 | fight_model.rs:831 | 26 | 올리면 더 건강한 적에게도 다이브 허용(공격적) | 4 | 기존 |
| 3 | 조건 B 내 HP 하한 | fight_model.rs:831 | 34 | 올리면 보수적 | 4 | 기존 |
| 4 | 조건 C 내 HP 하한 | fight_model.rs:832 | 44 | with_declared_dive 상태에서만. 올리면 보수적 | 4 | 기존 |
| 5 | 조건 C 적 HP 상한 | fight_model.rs:832 | 51 | with_declared_dive 상태에서만. 올리면 공격적 | 4 | 기존 |
| 6 | 근처 머릿수 반경 | fight_model.rs:822·825 | 19600000001 | 140000². 올리면 아군·적 모두 더 넓게 세어 ally_adv 판정이 바뀜(방향은 상황 의존). 두 줄 동시에 바꿔야 대칭 유지 | 4 | 기존 |
| 7 | 머릿수 우위 최소 아군 수 | fight_model.rs:828 | 1 | `> 1` — 자기 자신 포함 2명 이상. 올리면 3인 이상 모여야 우위로 침 | 4 | 기존 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_tower_focused_when_battle | game_ai::can_tower_focused_when_battle | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, u64, u64, u64) -> bool | game-ai\src\tower_discipline.rs:56 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | can_trace_without_tower | game_ai::can_trace_without_tower | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, usize, u64, u64, u64) -> bool | game-ai\src\tower_discipline.rs:676 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | is_unreasonable_tower_dive_enemy | game_ai::plan_legacy::old::is_unreasonable_tower_dive_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:792 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | ready_damage_to_target | game_ai::plan_legacy::old::fight_model::ready_damage_to_target | in:game_ai | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\plan_legacy\old\fight_model.rs:759 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 7 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 1개**: `legacy`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 14곳** (m05.ll:23167, m05.ll:31261, m10.ll:5310, m10.ll:15368, m10.ll:15618, m10.ll:15934, m10.ll:17493, m10.ll:22662, m10.ll:24153, m10.ll:47696, m10.ll:55101, m10.ll:55520, m10.ll:55678, m10.ll:56361) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | max_range_cached(data, champ, target)·ready_damage_to_target(ctx, champ, target)·can_tower_focused_when_battle(...) 내부는 담당 범위 밖 — 미독해(각각 battle.rs / fight_model.rs / tower_discipline.rs 에 define 있음, fnparts 로 위치 확인 가능) | 4 |  |
| 1 | 미탐색 | Blackboard::is_recent_visible(_gcbc/g07.ll:157005) 내부: `game.is_visible?(team, id) \|\| last_seen_tick+120 >= tick` 형태로 보이나(157017·157039~157043) 본문 미독해. blackboard[1-team] 인덱스 의미는 _docs/game_core.txt:21('Blackboard[team]=team 팀 정보, 관측은 1-team') 에 따라 '내 팀이 관측한 적 팀 정보'로 읽었음 — 추정 | 4 |  |
| 2 | 표기 불가 | L799 의 소스 표기(`target.team != Player(enemy) \|\| !matches!(ty, Champion)` vs `!(a && b)`) — column 부재로 표기 불가(동작 확정: 둘 다 만족해야 진행) | 4 |  |
| 3 | 미탐색 | L813 소스가 `if !needs_tower_entry { return false }` 인지 `needs_tower_entry` 를 뒤에서 && 로 합친 것인지 — 분기 구조상 전자와 동치(동작 확정) | 4 |  |
| 4 | 표기 불가 | L830~834 의 A/B/C 결합 순서·괄호 표기 — select/and/or 로 접혀 표기 불가(외연 확정: A\|\|B\|\|C) | 4 |  |
| 5 | 미탐색 | `_version` 이 미사용인 이유(호환용 시그니처로 추정) — 미탐색 = 호출자 12곳 | 5 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

