---

### `236` v15_can_keep_support_pressure — 서포트 대상(support_target)에 대한 압박을 유지해도 되는가 — 내 HP·피해·CC 상태 게이트 후, 대상이 아군이면 근처 가시 적 존재로, 적이면 가시성+180000 거리로 판정

| 항목 | 값 |
|---|---|
| id | `battle_common__v15_can_keep_support_pressure` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common29v15_can_keep_support_pressure` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle_common.rs:7` |
| IR | `m05.ll` 57757~57979행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::battle_common::v15_can_keep_support_pressure` · **in:game_ai** |
| 계층 | 기타 |
| exe | `d69120` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, std::option::Option<usize>) -> bool
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[236]/sig/tls/<키>`)**

- `name`: (직접 접점 없음)
- `role`: 없음
- `key`: 본체·aux 클로저 모두 `LocalKey`/`call_once` 상수 참조 0건(grep) — TLS 메모를 직접 읽거나 쓰지 않는다
- `layout`: -
- `invalidation`: -
- `call_conditions`: 콜리 `v21_support_pressure_too_risky`(1,022줄 · 내부 미독해)·`is_recent_visible`(TLS 참조 0 확인) 경유 여부는 그 콜리 명세 소관

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize | IR %0 · 본문에서 안 읽음(밑줄 이름 그대로 · DI `_version`) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | IR %1 · readonly · info.team(+0x930)·info.position 태그(+0x9c0) 읽음 · is_recent_visible/v21_support_pressure_too_risky 에 전달 | 4 |
| 2 | 3 | data | &OperationData(24B) | IR %2 · readonly · cache(+0)·blackboard(+0x10) 읽음 | 4 |
| 3 | 4 | parameter | &ScoreParameter(5384B) | IR %3 · readonly · player.applyed_cc(+0x990)·player.applyed_damage(+0x988) 읽음 | 4 |
| 4 | 5 | support_target | Option<usize> | IR 스칼라 2개: %4 = 태그(range(0,2) · 1=Some) · %5 = 페이로드 target_id. None 이면 즉시 false(L8) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L8   let Some(target_id) = support_target else { return false };
L12  let Some(champ) = data.cache.player_champion[player.info.team][player.info.position as usize] else { return false };   // team 은 <2 bounds check(panic) · position 은 i32 태그 그대로
L16  if champ.hp*100 / max(champ.stat_cached.hp,1) < 45 || parameter.player.applyed_cc != 0 { return false }   // IR 은 select(hp<45, true, cc≠0) — 두 항 모두 순수 load 라 소스 순서 판별 불가·외연 동일
L18  if !(parameter.player.applyed_damage*100 < max(champ.hp,1)*20) { return false }   // 받은 피해가 현재 HP 의 20% 미만일 때만 계속
L22  let Some(target) = data.cache.game.get_entity_by_id(target_id) else { return false };   // vtable +0x1f0
L26  if v21_support_pressure_too_risky(player, data, parameter, champ, target) { return false }
L30  if target.team == champ.team {                       // TeamType: 태그 같고 (Neutral 이거나 Player 페이로드 같음)
L35     let team = 1 - player.info.team;                 // 적팀
L36     return data.cache.iter_champions(team).any(|enemy|   // aux m05.ll 9441~9595 · Option 널 건너뜀
L37         data.blackboard[1 - player.info.team].is_recent_visible(game, player, enemy)
L38         && dist2(enemy, target) < 120000²+1
L39         && dist2(enemy, champ)  < 180000²+1);
     } else {
L31     return data.blackboard[1 - player.info.team].is_recent_visible(game, player, target)
L32         && dist2(target, champ) < 180000²+1;
     }
L41  (phi: false ×7 / %81(any) / %108)

해석: 대상이 아군(같은 팀)이면 '그 아군 근처(120000)에 내 사거리권(180000) 안의 가시 적이 있을 때' 압박 유지, 대상이 적이면 '가시 상태이고 180000 이내'일 때 유지.
dist2 = |dx|²+|dy|² (u64, entity.rs:2158 인라인 · nuw sub 로 절댓값).
```

**`mem` 메모리 접근 15건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L12 player_champion 1차 인덱스(<2 bounds check 있음) · L35/L37/L31 `1 - team` 으로 적팀 인덱스 | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | L12 i32 태그를 zext 해 player_champion[team][position] 2차 인덱스로 사용(bounds check 없음 — 태그 0..4 전제) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 3 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] · 744B stride · 인덱스 = 1 - player.info.team | 4 | OK |
| 4 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data ptr) | r | L22 get_entity_by_id 의 self · is_recent_visible 인자 | 4 | OK |
| 5 | AbstractGameWithCache | 0x8 | game (vtable ptr) | r | +0x1f0 슬롯 = get_entity_by_id (divtable) | 3 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] (Option<&Entity> = 널 니치) | r | L12 [team][position] · L35 [1-team] 슬라이스 5개(끝 = +40) | 4 | OK |
| 7 | Entity | 0x670 | hp | r | champ · L16 HP% 분자 · L18 20% 기준 | 4 | OK |
| 8 | Entity | 0x628 | stat_cached.hp | r | champ · L16 max(…,1) 분모 | 4 | OK |
| 9 | Entity | 0x0 | team@tag | r | L30 target.team == champ.team (TeamType derived PartialEq) | 4 | OK |
| 10 | Entity | 0x8 | team@Player.0 | r | L30 태그가 Player(0)일 때만 페이로드 비교 | 4 | OK |
| 11 | Entity | 0x660 | x | r | L32/L38/L39 거리² | 4 | OK |
| 12 | Entity | 0x668 | y | r | L32/L38/L39 거리² | 4 | OK |
| 13 | ScoreParameter | 0x990 | player.applyed_cc | r | L16 ≠0 이면 false | 4 | OK |
| 14 | ScoreParameter | 0x988 | player.applyed_damage | r | L18 *100 < max(champ.hp,1)*20 이어야 계속 | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 12 | 길이 | player_champion 1차 배열 길이(team<2 bounds check · panic_bounds_check 인자) — 판정값 아님 | 4 |
| 1 | 100 | 16 | 계수 | HP% 환산 배수(L16 champ.hp*100 / max(stat_cached.hp,1)) · L18 applyed_damage*100 | 4 |
| 2 | 45 | 16 | 임계 | 내 HP% 하한 — 45% 미만이면 false | 4 |
| 3 | 20 | 18 | 계수 | 받은 피해 허용 상한 = 현재 HP 의 20% (applyed_damage*100 < max(hp,1)*20) | 4 |
| 4 | 1 | 16 | 인덱스 | max(…,1) 0-나눗셈 가드(llvm.umax) · L18 도 동일 | 4 |
| 5 | 32400000001 | 32 | 임계 | 180000²+1 — (적 대상) target↔champ 거리² 상한 / (aux L39) enemy↔champ 거리² 상한 | 4 |
| 6 | 14400000001 | 38 | 임계 | 120000²+1 — (aux L38) enemy↔target 거리² 상한 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 내 HP% 하한 | battle_common.rs:16 | 45 | 올리면 더 건강할 때만 압박 유지 → 서포터가 더 빨리 빠진다 | 4 | 기존 |
| 1 | 받은 피해 상한(현재 HP 대비 %) | battle_common.rs:18 | 20 | 올리면 피해를 더 받고도 압박 유지 | 4 | 기존 |
| 2 | 적↔아군대상 근접 거리(아군 대상일 때) | battle_common.rs:38 | 14400000001 | 120000² — 올리면 더 먼 적도 '아군을 위협' 으로 봐 압박 유지가 잦아짐 | 4 | 기존 |
| 3 | 적↔나 / 적대상↔나 거리 상한 | battle_common.rs:32 / :39 | 32400000001 | 180000² — 올리면 더 먼 대상에도 압박 유지 true | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | check | game_core::transfer::SellGuard::check | pub | fn(&game_core::Database, usize, game_core::Position) -> game_core::transfer::SellGuardResult | game-core\src\transfer\roster_blueprint.rs:564 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | v21_support_pressure_too_risky | game_ai::plan_legacy::sub_plan::battle_common::v21_support_pressure_too_risky | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\sub_plan\battle_common.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `dist2`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m02.ll:36739, m02.ll:37235, m15.ll:11093, m15.ll:11584) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L16 `\|\|` 두 항의 소스 순서(hp<45 vs applyed_cc≠0) — column 정보 부재 + 둘 다 순수 load 라 IR 에 흔적 없음(표기 불가·외연 동일) | 4 |  |
| 1 | 미탐색 | blackboard 인덱스가 `1 - player.info.team`(적팀 번호)인 의미 — Blackboard 배열이 '팀별 관측 보드'인지 '상대팀에 대한 보드'인지는 game_core 소관(is_recent_visible 내부 미독해) | 4 |  |
| 2 | 미탐색 | `_version`(i64 %0) 은 본문에서 미사용 — 버전 분기 없음 | 4 |  |
| 3 | 미탐색 | `v21_support_pressure_too_risky` 내부(m05.ll 58219~59241 · 1,022줄)는 이 배치 범위 밖 — 계약만: fn(&PlayerState,&OperationData,&ScoreParameter,&Entity champ,&Entity target)->bool, true=위험 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

