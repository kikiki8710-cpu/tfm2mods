---

### `24` v23_healthy_allies_near_point — 내 팀 챔피언(최대 5) 중 HP% ≥ min_hp_ratio 이고 (x,y) 로부터 range 이내인 인원 수를 센다

| 항목 | 값 |
|---|---|
| id | `objective_helpers__v23_healthy_allies_near_point` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers29v23_healthy_allies_near_point` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:25` |
| IR | `m15.ll` 53341~53830행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::v23_healthy_allies_near_point` · **pub** |
| 계층 | 기타 |
| exe | `15505472` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽음 — 어느 팀 챔피언을 셀지 | 4 |
| 1 | 1 | data | &OperationData(24B) | cache(+0x0) 의 player_champion[team] 만 사용 | 4 |
| 2 | 2 | x | u64 | 기준점 x | 4 |
| 3 | 3 | y | u64 | 기준점 y | 4 |
| 4 | 4 | range | u64 | 반경. range² 와 dist² 를 비교(ule) | 4 |
| 5 | 5 | min_hp_ratio | usize | HP 백분율 하한(0~100 스케일). hp*100/max_hp < 이 값이면 제외 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v23_healthy_allies_near_point(player, data, x, y, range, min_hp_ratio) -> usize
// L26
let team = player.info.team;                                   // +0x930, bounds 2
// L27~28
data.cache.iter_champions(team)                                // player_champion[team] 의 Some 만 (5슬롯 언롤)
  .filter(|e| v23_healthy(e, min_hp_ratio)                     // objective_helpers.rs:12: e.hp*100 / e.stat_cached.hp >= min_hp_ratio  (ult 로 '미만이면 false')
           && (x.abs_diff(e.x))² + (y.abs_diff(e.y))² <= range*range)   // 단락: healthy 먼저, 그 다음 거리
  .count()
// max_hp == 0 인 슬롯이 있으면 div_by_zero 패닉. 사망(hp=0) 챔피언은 min_hp_ratio>0 이면 자동 제외
```

**`mem` 메모리 접근 7건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. >=2 면 panic_bounds_check(len 2) | 4 | OK |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 2 | AbstractGameWithCache | 0x1e0 | player_champion[team][0..5] | r | [2][5] Option<&Entity>(null 니치). iter_champions(team) 인라인 — 5슬롯 완전 언롤, null 은 skip | 4 | OK |
| 3 | Entity(아군 챔피언) | 0x628 | stat_cached.hp (최대 HP) | r | 0 이면 panic_const_div_by_zero | 4 | OK |
| 4 | Entity(아군 챔피언) | 0x670 | hp | r | hp*100/stat_cached.hp 로 백분율 | 4 | OK |
| 5 | Entity(아군 챔피언) | 0x660 | x | r | abs_diff 로 dx | 4 | OK |
| 6 | Entity(아군 챔피언) | 0x668 | y | r | abs_diff 로 dy | 4 | OK |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 100 | 12 | 계수 | v23_healthy(objective_helpers.rs:12) 인라인 — HP 백분율 hp*100/max_hp. 클로저 L27 에서 호출 | 4 |
| 1 | 2 | 26 | 임계 | player_champion 1차 배열 길이 2(팀). team>=2 면 panic_bounds_check | 4 |
| 2 | 0 | 28 | 태그 | count 초기값 0 (fold init) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | HP 하한·반경은 전부 인자(min_hp_ratio, range) | objective_helpers.rs:25 시그니처 | 호출자가 정함 | 이 함수 안에 고정 임계는 없다. 호출자별 값(예: _docs 의 '건강(≥40%)' 언급)은 호출처에서 확인해야 한다 | 4 | 기존 |
| 1 | 거리 비교 포함/배제 | objective_helpers.rs:27 | dist² <= range² (ule, 경계 포함) | < 로 바꾸면 정확히 range 거리인 아군이 빠진다 | 4 | 기존 |

<details><summary>`callees` 피호출자 3건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 1 | v23_healthy | game_ai::plan_legacy::team_plan::objective_helpers::v23_healthy | in:game_ai | fn(&game_core::Entity, usize) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:11 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | v23_healthy_allies_near_point | game_ai::plan_legacy::team_plan::v23_healthy_allies_near_point | pub | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:25 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `min_hp_ratio`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 8곳** (m09.ll:29701, m09.ll:29703, m15.ll:52770, m15.ll:54074, m15.ll:54730, m15.ll:56711, m15.ll:56725, m15.ll:56762) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | v23_healthy(objective_helpers.rs:12) 의 정확한 시그니처(인자 순서·반환)는 인라인돼 IR 동작(hp*100/max_hp >= min_hp_ratio)만 확정 — _gaibc 전 파일에 별도 define 없음(grep 0건, fnparts: 서브프로그램 12 vs define 1 = 전부 인라인) | 4 |  |
| 1 | 미탐색 | 사망/미스폰 챔피언이 player_champion 에 None 으로 오는지 Some(hp=0) 으로 오는지는 이 함수 밖(cache 구성, 미탐색) — 후자면 min_hp_ratio=0 호출 시 죽은 아군도 센다 | 4 |  |
| 2 | 미탐색 | 호출자가 넘기는 range·min_hp_ratio 실제 값은 이 명세 범위 밖 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L27 클로저 안에서 healthy 와 거리 조건의 소스 순서는 IR 단락 순서(healthy → 거리)로 적었다(column 없음) | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

