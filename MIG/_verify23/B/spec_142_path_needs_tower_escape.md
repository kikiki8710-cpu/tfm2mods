---

### `142` path_needs_tower_escape — v38 타워 회피 후퇴 필요 판정 — 현재 경로의 다음 웨이포인트가 '불필요한 적 타워 위치'면 true (HP비율·타워 표적·최근 타워 피격 게이트)

| 항목 | 값 |
|---|---|
| id | `small_action__path_needs_tower_escape` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai12small_action23path_needs_tower_escape` |
| 소스 | `game-ai\src\small_action.rs:109` |
| IR | `m11.ll` 52695~52915행 |
| 경로·가시성 | `game_ai::small_action::path_needs_tower_escape` · **in:game_ai::small_action** |
| 계층 | 기타 |
| exe | `e29520` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::PathFinder) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 분기 없음 — is_unnecessary_enemy_tower_position 첫 인자로만 전달(52759/52909) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | +0x930 team · +0x9c0 position 태그 · path_finder 콜리 인자 | 4 |
| 2 | 3 | data | &OperationData(24B) | +0x0 cache 만(player_champion · iter_towers_without_nexus · game.get_entity_by_id) | 4 |
| 3 | 4 | path_finder | &PathFinder(72B) | captures(none) readonly. +0x10 path_len · +0x18 index · +0x28 path(Box<[(u64,u64);70]>) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L110: if path_finder.path_len == 0 → false
L114: champ = data.cache.player_champion[team][position] ; None → false
L119: hp_ratio = champ.hp * 100 / champ.stat_cached.hp.max(1)
L120: if hp_ratio > 45 {
  L121~122: tower_targets_champ = data.cache.iter_towers_without_nexus(1-team)   [sret 120B Chain 이터레이터]
                 .any(|t| t.ty==Tower(2) && t.tower.nearest_enemy.is_some() && nearest_enemy.1 == champ.id)   [aux 클로저 L123~124]
  L129: if !tower_targets_champ {
    L130~133: recent_tower_hit = hp_ratio < 66
                 && champ.last_attacked_from.and_then(|id| game.get_entity_by_id(id))                       [L131]
                       .is_some_and(|e| e.team == Player(1-team) && e.is_tower())                          [L132]
    if !recent_tower_hit → false
  }
}
L139: if is_unnecessary_enemy_tower_position(version, player, data, champ.x, champ.y) → false   (이미 그 위치면 회피 불필요)
L143: move_speed = champ.stat_cached.move_speed
L144: index = path_finder.index.min(path_len-1)
L145: (x,y) = path[index]
L146: if distance_sq((x,y),(champ.x,champ.y)) < (move_speed*10)² {   [abs_diff² 합]
  L147: next_index = (index+1).min(path_len-1)
  L151: (x,y) = path[next_index]
}
L152: return is_unnecessary_enemy_tower_position(version, player, data, x, y)
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PathFinder | 0x10 | path_len | r | 52703 (gep 16) == 0 → false · 52840 path_len-1 상한 | 4 | OK |
| 1 | PathFinder | 0x18 | index | r | 52838 (gep 24) → min(index, path_len-1) | 4 | OK |
| 2 | PathFinder | 0x28 | path | r | 52853 (gep 40) Box<[(u64,u64);70]> · 원소 16B(gepS {i64,i64}) · bounds 70 (52884/52913) | 4 | OK |
| 3 | PlayerState | 0x930 | info.team | r | 52709 (gep 2352) · bounds 2 · 적 팀 = 1-team (52764) | 4 | OK |
| 4 | PlayerState | 0x9c0 | info.position@tag | r | 52724 (gep 2496) | 4 | OK |
| 5 | OperationData | 0x0 | cache | r | 52727 | 4 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | 52728~52731 · null → false | 4 | OK |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr | r | 52791 | 4 | OK |
| 8 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 52789 · vtable+0x1f0(496) get_entity_by_id (52798~52800) | 4 | OK |
| 9 | Entity | 0x670 | hp | r | 52737 champ.hp | 4 | OK |
| 10 | Entity | 0x628 | stat_cached.hp | r | 52739 (gep 1576) 최대 HP · max(…,1) | 4 | OK |
| 11 | Entity | 0x660 | x | r | 52751 champ.x | 4 | OK |
| 12 | Entity | 0x668 | y | r | 52755 champ.y | 4 | OK |
| 13 | Entity | 0x640 | stat_cached.move_speed | r | 52850 (gep 1600) — 도달 판정 반경 (move_speed*10)² | 4 | OK |
| 14 | Entity | 0x28 | last_attacked_from@tag | r | 52778 (gep 40) trunc→i1 Some 판정 (option.rs:1542 and_then 인라인) | 4 | OK |
| 15 | Entity | 0x30 | last_attacked_from@Some.0 | r | 52793 (gep 48) 공격자 id → get_entity_by_id | 4 | OK |
| 16 | Entity | 0x0 | team@tag | r | 52811 공격자 e.team 태그 == 0 Player | 4 | OK |
| 17 | Entity | 0x8 | team@Player.0 | r | 52824 == 1-team (적 팀) | 4 | OK |
| 18 | Entity | 0x68 | ty@tag | r | 52830 e.is_tower(entity.rs:1386) == 2 Tower · aux 클로저 31039/34952 t.ty == 2 | 4 | OK |
| 19 | Entity | 0x5c0 | id | r | aux 31020/34909 champ.id (클로저 캡처) ↔ 타워 nearest_enemy.1 | 4 | OK |
| 20 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag | r | aux 31043/34956 (gep 136) trunc→i1 Some | 4 | OK |
| 21 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 | r | aux 31056/34969 (gep 152) == champ.id | 4 | OK |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 110 | 태그 | path_len == 0 → false (52705) · 공격자 team 태그 0 Player (52813) | 4 |
| 1 | 2 | 114 | 태그 | player_champion bounds team<2 (52711) · EntityType 태그 2 Tower (52832 · aux 31041) | 4 |
| 2 | 100 | 119 | 계수 | hp_ratio = hp*100/max_hp (52744) | 4 |
| 3 | 1 | 119 | 인덱스 | max_hp.max(1) 분모 가드(52743) · 적 팀 = 1-team (52764) · next_index = index+1 (52888) | 4 |
| 4 | 45 | 120 | 임계 | hp_ratio > 45 이면 타워 표적/최근 타워 피격 게이트를 거침. 45 이하면 게이트 없이 바로 경로 검사 (52747 ugt) | 4 |
| 5 | 66 | 130 | 임계 | hp_ratio < 66 일 때만 '최근 타워 피격' 경로로 게이트 통과 가능 (52774 ult) | 4 |
| 6 | 10 | 146 | 계수 | 도달 반경 = move_speed*10 (52878) — 제곱비교 dist² < (move_speed*10)² | 4 |
| 7 | 70 | 145 | 임계 | path 배열 길이 [(u64,u64);70] bounds (52846/52894) | 4 |
| 8 | -1 | 121 | 계수 | aux m06 34852 — Chain 어댑터 a-side 소진 니치 태그(핵심 판정 아님) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 게이트 면제 HP% | small_action.rs:120 | 45 | hp_ratio ≤ 45 면 타워표적/피격 조건 없이 항상 경로 검사. 올리면 더 건강한 챔프도 무조건 타워 회피를 검사(후퇴 빈도↑) | 4 | 기존 |
| 1 | 최근 타워 피격 인정 HP% | small_action.rs:130 | 66 | 45<hp<66 구간에서만 '적 타워에게 마지막 피격' 으로 게이트 통과. 올리면 고HP 에서도 피격 이력으로 회피 검사 | 4 | 기존 |
| 2 | 웨이포인트 도달 반경 배율 | small_action.rs:146 | 10 | move_speed*10 안이면 다음 웨이포인트를 검사. 올리면 더 일찍 다음 지점의 타워 위험을 봄 | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | is_tower | game_core::EntityType::is_tower | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1385 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | is_unnecessary_enemy_tower_position | game_ai::is_unnecessary_enemy_tower_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\path_finder.rs:1399 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 1개**: `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m02.ll:64216, m08.ll:93968, m11.ll:46523) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L120~133 의 정확한 소스 중첩 형태(if/&&/\|\| 조합) — IR 흐름(hp>45 → any → hp<66&&recent)만 확정, 소스 표기는 외연 동일 후보 다수라 표기 불가 | 4 |  |
| 1 | 미탐색 | iter_towers_without_nexus(cache, team) sret 120B = Chain<Flatten<IntoIter<Option<&Entity>,6>>,Copied<Iter<&Entity>>> — 앞 6칸 고정 배열 + 슬라이스 의 정체(어느 타워 집합인지)는 game_core 경계로 미탐색 | 4 |  |
| 2 | 미탐색 | is_unnecessary_enemy_tower_position(m03.ll:146255 · (version,player,data,x,y)->bool) 내부 미탐색(경로 계층) | 4 |  |
| 3 | 미탐색 | closure_env$1/$2(and_then · is_some_and)는 본문에 인라인(fnparts: DWARF 33 vs define 3) — 별도 define 없음 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

