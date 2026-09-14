---

### `135` v3_lethal_tower_position — [v3 M1b] 치명 HP(타워 2샷)일 때 셀 (x,y)가 적 타워 사거리(+20k 여유) 안이면 true — 이동 후보 하드 스킵

| 항목 | 값 |
|---|---|
| id | `tower_discipline__v3_lethal_tower_position` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline24v3_lethal_tower_position` |
| 소스 | `game-ai\src\tower_discipline.rs:282` |
| IR | `m07.ll` 50614~50670행 |
| 경로·가시성 | `game_ai::v3_lethal_tower_position` · **pub** |
| 계층 | 기타 |
| exe | `d98420` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | AI 버전 게이트. `icmp ult %0, 2` — version<2 면 즉시 false (m07.ll:50625~50626, L283) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | readonly. info.team(+0x930)·info.position(+0x9c0) 만 읽음 | 4 |
| 2 | 3 | data | &OperationData(24B) | readonly. +0x0 cache(&AbstractGameWithCache) 만 사용. v3_lethal_tower_hp 에 그대로 전달 | 4 |
| 3 | 4 | x | u64 | 후보 셀 좌표 x(월드 단위). alloca %8 에 저장돼 클로저 env f[0..8] 로 캡처 | 4 |
| 4 | 5 | y | u64 | 후보 셀 좌표 y. alloca %7 → 클로저 env f[8..16] | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v3_lethal_tower_position(version, player, data, x, y) -> bool  [tower_discipline.rs:282]
  L283: if version < 2 { return false }
  L286: team = player.info.team (+0x930); (team>=2 → panic_bounds_check)
        champ = data.cache.player_champion[team][player.info.position(+0x9c0)]   // Option<&Entity>, null=None
        if champ.is_none() { return false }
  L289: if !v3_lethal_tower_hp(data, player, champ) { return false }   // 비치명 HP 면 항상 false (콜리 내부는 안 봄)
  L292: towers = data.cache.iter_towers_without_nexus(1 - team)   // 적 팀: [top,top2,mid,mid2,bottom,bottom2] 6개 Option + twin_towers 슬라이스 (Chain)
  L293~295: return towers.any(|t| {                                   // aux 클로저 m06(6개 배열 절)·m11(twin 슬라이스 절) — 동일 술어
        // L294: r = t.attack_effect.as_ref().map(|e| e.range(t) + 20000).unwrap_or(20000) + t.radius()
        //   Effect::range(effect.rs:26) = range(+0x4a0) + stat_buff_cached.range(+0x438) + growth_range(+0x4a8)*(level(+0x5c8)-1)
        //   Entity::radius(entity.rs:1511~1515) = radius_mult(+0x470)==0 ? radius(+0x680) : radius*(mult+100)/100   (udiv)
        // L295: dx=|t.x-x|, dy=|t.y-y| (sub nuw 절대값 select)
        dx*dx + dy*dy <= r*r          // IR: `icmp ugt (dx²+dy²), r²` → true(ugt) 면 다음 타워, 아니면 any=true
     })
  극성: 어느 타워라도 (dist² <= (attack_range+20000+radius)²) 이면 true.
  ※ can_target 필터 없음 — 무적 타워도 포함(iter_towers_without_nexus 는 Option 이 Some 인 타워 전부, 파괴된 타워는 cache 에 None 으로 들어 있는지는 game_core 경계라 미확인).
```

**`mem` 메모리 접근 13건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m07.ll:50629~50630 · 값 ≥2 면 panic_bounds_check(len 2) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 → zext, player_champion[team][position] 인덱스 (m07.ll:50640~50642) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m07.ll:50643) | 4 | OK |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | Option<&Entity> 니치(null=None). None 이면 false (m07.ll:50644~50648) | 4 | OK |
| 4 | Entity | 0x4c0 | attack_effect@tag | r | (aux 클로저) -1 = None → 사거리 0 취급. m06.ll:35270~35272 | 4 | OK |
| 5 | Entity | 0x4a0 | attack_effect.range | r | (aux) Effect::range 인라인, effect.rs:26 | 4 | OK |
| 6 | Entity | 0x4a8 | attack_effect.growth_range | r | (aux) × (level-1) | 4 | OK |
| 7 | Entity | 0x5c8 | level | r | (aux) growth_range 곱셈 | 4 | OK |
| 8 | Entity | 0x438 | stat_buff_cached.range | r | (aux) 사거리 가산 | 4 | OK |
| 9 | Entity | 0x470 | stat_buff_cached.radius_mult | r | (aux) Entity::radius 인라인 entity.rs:1511~1515. 0 이면 radius 그대로 | 4 | OK |
| 10 | Entity | 0x680 | radius | r | (aux) 타워 반지름. mult≠0 이면 radius*(mult+100)/100 | 4 | OK |
| 11 | Entity | 0x660 | x | r | (aux) 타워 x | 4 | OK |
| 12 | Entity | 0x668 | y | r | (aux) 타워 y | 4 | OK |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 283 | 임계 | version 게이트: `version < 2` 면 false (m07.ll:50625). 같은 2 가 team 인덱스 bounds(len 2) 에도 나온다(L286) | 4 |
| 1 | 1 | 292 | 인덱스 | `1 - player.team` = 적 팀 인덱스 (sub nuw nsw i64 1, %12 · m07.ll:50658) | 4 |
| 2 | 20000 | 294 | 미상 | 타워 사거리 여유(+20k 월드단위 ≈ 0.625셀). attack_effect 가 None 이어도 20000 은 남는다(phi m06.ll:35299: [range+20000+buf+growth, 20000]) | 4 |
| 3 | -1 | 294 | 센티널 | Entity.attack_effect Option 니치 None 태그(i32 -1, tcxdict Entity 0x4c0). `icmp eq i32 %28, -1` m06.ll:35272 | 3 |
| 4 | 100 | 294 | 미상 | Entity::radius(entity.rs:1515) 인라인: radius*(radius_mult+100)/100 — 퍼센트 배수. m06.ll:35316~35318 | 4 |
| 5 | 6 | 292 | 미상 | iter_towers_without_nexus 의 앞부분 = [Option<&Entity>;6](top/top2/mid/mid2/bottom/bottom2) 배열 순회 상한(aux m06.ll:35241 `icmp ult i64 %20, 6`) — 임계값 아님, 배열 길이 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | version 게이트 | tower_discipline.rs:283 | 2 | 올리면 더 높은 버전까지 이 필터가 꺼진다(항상 false) | 4 | 기존 |
| 1 | 타워 사거리 여유 폭 | tower_discipline.rs:294 (aux m06.ll:35293 / m11.ll 동일) | 20000 | 올리면 타워에서 더 먼 셀까지 치명 밴드로 보고 이동 후보에서 뺀다 → 타워 근처 접근이 더 소극적. 내리면 밴드가 좁아져 사거리 경계 셀을 후보로 허용 | 4 | 기존 |

<details><summary>`callees` 피호출자 13건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 2 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 5 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 6 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 8 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | v3_lethal_tower_hp | game_ai::v3_lethal_tower_hp | pub | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:272 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | v3_lethal_tower_position | game_ai::v3_lethal_tower_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\tower_discipline.rs:282 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m08.ll:93557, m08.ll:103007, m08.ll:104463, m08.ll:109479) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | v3_lethal_tower_hp(data, player, champ)->bool 내부(치명 HP 판정)는 콜리 계약만 — m07.ll:49429~49614 · 별도 명세 대상 | 4 |  |
| 1 | 미탐색 | iter_towers_without_nexus(sret 120B Chain 이터레이터, &cache, team) 의 내부(파괴 타워 제외 여부·twin_towers 순서)는 game_core(_gcbc/g15.ll:108971) 경계라 안 봄. 관측: sret 120B = Chain<Flatten<IntoIter<Option<&Entity>,6>>, Copied<slice::Iter<&Entity>>> · +0x0 첫 이터 상태(-1=소진), +0x8/+0x10 배열 인덱스 범위, +0x18 [Option<&Entity>;6] 데이터, +0x68 두 번째 이터 데이터 ptr(null=None) | 4 |  |
| 2 | 표기 불가 | L294 의 소스 표기가 `.map(\|e\| e.range()+20000).unwrap_or(20000)` 인지 `.map_or(0,..)+20000` 인지 — IR 은 phi [range+…+20000, 20000] 로 접혀 외연 동일(표기 불가) | 4 |  |
| 3 | 표기 불가 | Effect::range 가 정확히 `range + buf.range + growth*(level-1)` 순서인지 — 덧셈 결합순서만 다르고 값 동일(표기 불가) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

