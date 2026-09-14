---

### `157` SmallActionAroundPositionBush::get_input — 목표 수풀 셀까지 타워 회피 경로탐색(PathFinder) 입력 생성 — 목표 반경 16000 안이면 None

| 항목 | 값 |
|---|---|
| id | `SmallActionAroundPositionBush__get_input` |
| 심볼 | `_RNvMs5_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_29SmallActionAroundPositionBush9get_input` |
| 소스 | `game-ai\src\small_action\around.rs:1086` |
| IR | `m08.ll` 104592~104737행 |
| 경로·가시성 | `game_ai::SmallActionAroundPositionBush::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `dc2070` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionAroundPositionBush, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<Input>(32B) | None = tag i64 -1 @+0x0(니치) · Some = Input 그대로(tag 0..5 @+0x0, 페이로드 +0x8~) | 4 |
| 1 | 1 | self | &mut SmallActionAroundPositionBush(96B) | ★&mut — writes 참조. +0x8 target_x · +0x10 target_y 읽음, +0x18 path_finder(Option<PathFinder> 72B, 태그 +0x5d) 읽고 씀 | 4 |
| 2 | 2 | version | usize | 본문 분기 없음. new_tower_avoid_v3 · new_target(3번째) · 클로저 환경[0] 에 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | ★&mut — 본문 직접 store 0. PathFinder::new_target / update_path 에 전달(path_finder 계층 내부 소비, 미탐색) | 4 |
| 4 | 4 | player | &PlayerState(2528B, readonly) | +0x930 info.team · +0x9c0 info.position 태그 읽음 | 4 |
| 5 | 5 | data | &OperationData(24B, readonly) | +0x0 cache → player_champion · +0x8 context | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B, readonly) | 본문 읽기 0. 클로저 환경[3] 으로 전달(콜리 dodge_tower_cell_with_context 호출부에선 poison = 미사용) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
SmallActionAroundPositionBush::get_input(&mut self, version, rnd, player, data, ps) -> Option<Input>   [around.rs:1086]
 entity = data.cache.player_champion[player.info.team][player.info.position].unwrap()   (L1087; team<2 아니면 panic, None 이면 unwrap 패닉)
 (ex, ey) = (entity.x, entity.y) ; (tx, ty) = (self.target_x, self.target_y)
 if |ex-tx|² + |ey-ty|² < 256000001:   # 16000²+1                                (L1089)
     return None                                                                  (L1090)
 tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, tx, ty)   # 416B 지역   (L1093)
 if self.path_finder.is_none():                                                   (L1094, 태그 +0x5d == 2)
     self.path_finder = Some(PathFinder::new_target(rnd, data.context, version, "dodge_tower_cell", ex, ey, tx, ty,
         |sx,sy,nx,ny| /*closure#0, 환경 {version, player, data, ps, &tower_dodge}*/ … -> PathVerdict))     (L1095)
     if self.path_finder.is_none(): return None      # new_target 결과가 None 일 수 있음(Option 그대로 memcpy)   (L1104)
 self.path_finder.as_mut().update_path(rnd, data.context, ex, ey, tx, ty, |sx,sy,nx,ny| … /*closure s_0, 같은 환경*/)   (L1106)
 return Some(self.path_finder.get_input(player, data, SafeMoveWithSkill::Safe, false))   (L1114; sret 직접 기록)

클로저 계약(관측): 두 클로저 모두 환경 {version, player, data, ps, &tower_dodge} 를 잡고 PathFinder 인스턴스(m03.ll 14343~17816 / 31379~32434) 안에 인라인돼 `dodge_tower_cell_with_context(version, player, data, ps, tower_dodge, sx,sy,nx,ny)` 를 부른다(m03 17654·31641 — 호출부에서 version/data/ps 가 `poison` = 콜리가 그 인자를 안 읽는다는 LLVM IPO 증거). check_cell 은 이 함수 경로에서 호출되지 않는다(Bush 계열만).
```

**`mem` 메모리 접근 14건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 104608; <2 아니면 panic_bounds_check(104613) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 → zext, [5] 인덱스 (104618~104620, 인라인 player.rs:581) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (104621) | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | Option<&Entity> 니치(null=None) → None 이면 unwrap_failed 패닉 (104622~104626, L1087) | 4 | OK |  |
| 4 | Entity | 0x660 | x | r | 내 챔피언 위치 (104633) | 4 | OK |  |
| 5 | Entity | 0x668 | y | r | 104637 | 4 | OK |  |
| 6 | SmallActionAroundPositionBush | 0x8 | target_x | r | 104641 | 4 | OK |  |
| 7 | SmallActionAroundPositionBush | 0x10 | target_y | r | 104645 | 4 | OK |  |
| 8 | SmallActionAroundPositionBush | 0x5d | path_finder@tag | r | +0x18+0x45: Option<PathFinder> 니치(unconstrained_fallback bool 자리), 2 = None (104676·104708) | 4 | OK |  |
| 9 | OperationData | 0x8 | context | r | 104681·104691 → new_target/update_path 2번째 인자 | 4 | OK |  |
| 10 | SmallActionAroundPositionBush(self) | 0x18 | path_finder | w | path_finder 가 None 일 때만(L1095). PathFinder 72B 전체(+0x18~+0x5f) 덮어씀 | 4 | OK | Some(PathFinder::new_target(rnd, ctx, version, "dodge_tower_cell"(16B), ex, ey, target_x, target_y, closure)) — 72B memcpy(104704) |
| 11 | SmallActionAroundPositionBush(self) | 0x18 | path_finder(콜리 경유) | w | path_finder 계층 내부 — 갱신 오프셋 전수는 미탐색(경계 규칙) | 4 | OK | PathFinder::update_path(&mut, …)(104724) 와 PathFinder::get_input(&mut, …)(104726) 가 내부 갱신(index/path/danger_violations 등) |
| 12 | Option<Input>(sret) | 0x0 | tag | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | -1(None) 104685·104731 / Input (PathFinder::get_input sret 104726) |
| 13 | StdRng(rnd) | 0x0 | rng 상태 | w | 본문 직접 store 없음 | 4 | OK | new_target/update_path 내부 소비(양 미상) |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 256000001 | 1089 | 임계 | 16000² + 1 — 내 위치↔목표 제곱거리 < 이 값(= 거리 ≤ 16000, 반 셀) 이면 도착으로 보고 None 반환 (104661 `icmp ult`) | 4 |
| 1 | -1 | 1090 | 센티널 | Option<Input>::None 니치 태그 (104685, 104731) | 4 |
| 2 | 2 | 1094 | 센티널 | Option<PathFinder>::None 니치 태그 비교 (104676·104708 `icmp eq i8 %52, 2`). 또한 player_champion 팀 인덱스 상한 2(panic_bounds_check 104613 — 배열 크기, 판정 아님) | 4 |
| 3 | 16 | 1095 | 길이 | &'static str "dodge_tower_cell" 의 길이(new_target 4번째 인자 key, @anon…87) | 4 |
| 4 | 1 | 1114 | 태그 | SafeMoveWithSkill::Safe 태그 1 — PathFinder::get_input 4번째 인자 (104726 `i8 1`); 5번째 bool = false | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 도착 판정 반경 | around.rs:1089 | 256000001 (16000²+1) | 올리면 목표에서 더 멀리서도 '도착'으로 보고 입력을 안 낸다(제자리) — 수풀 안 정지 위치가 느슨해짐 | 4 | 기존 |
| 1 | 경로 이동의 스킬 사용 정책 | around.rs:1114 | SafeMoveWithSkill::Safe(1), flag=false | Must/MustWithUlt 로 올리면 이동 스킬을 더 적극 사용(path_finder 계층 해석) | 4 | 기존 |

<details><summary>`callees` 피호출자 5건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | dodge_tower_cell_with_context | game_ai::dodge_tower_cell_with_context | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::TowerDodgeContext, usize, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1322 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 1 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | new_target | game_ai::PathFinder::new_target | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:66 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | new_tower_avoid_v3 | game_ai::TowerDodgeContext::new_tower_avoid_v3 | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1205 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

**호출처 1곳** (m11.ll:43052) · **형제 7개** (SmallActionAroundPositionBush)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionAroundPositionBush as std::clone::Clone>::clone | pub | game-ai\src\small_action\around.rs:1066 | True | fn(&game_ai::SmallActionAroundPositionBush) -> game_ai::SmallActionAroundPositionBush |
| 1 | <game_ai::SmallActionAroundPositionBush as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\around.rs:1066 | True | fn(&game_ai::SmallActionAroundPositionBush, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionAroundPositionBush::new | pub | game-ai\src\small_action\around.rs:1075 | False | fn(&game_core::OperationData, u64, u64) -> game_ai::SmallActionAroundPositionBush |
| 3 | game_ai::SmallActionAroundPositionBush::get_input | in:game_ai | game-ai\src\small_action\around.rs:1086 | False | fn(&mut game_ai::SmallActionAroundPositionBush, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> |
| 4 | game_ai::SmallActionAroundPositionBush::update_state | in:game_ai | game-ai\src\small_action\around.rs:1119 | False | fn(&mut game_ai::SmallActionAroundPositionBush, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 5 | game_ai::SmallActionAroundPositionBush::get_action | in:game_ai | game-ai\src\small_action\around.rs:1122 | True | fn(&game_ai::SmallActionAroundPositionBush) -> game_core::SmallAction |
| 6 | game_ai::SmallActionAroundPositionBush::is_end | in:game_ai | game-ai\src\small_action\around.rs:1125 | False | fn(&game_ai::SmallActionAroundPositionBush, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | PathFinder::new_target 이 Option<PathFinder> 를 반환하는지(72B sret 을 self.path_finder 에 통째로 memcpy 후 태그 재검사 L1104) — tcx 시그니처는 `-> PathFinder` 인데 IR 은 memcpy 직후 니치 태그 2 를 다시 검사한다. 사전 시그니처와 IR 이 어긋남: IR 우선 = 실제로는 None 가능(또는 컴파일러가 접지 못한 불변 검사). path_finder 계층 미탐색 | 3 |  |
| 1 | 미탐색 | PathFinder::get_input 5번째 bool(false) 의 의미 — path_finder 계층 미탐색 | 4 |  |
| 2 | 재료 부재 | closure#0 / closure s_0 두 개가 왜 따로 있는지(같은 환경·같은 본문) — 소스에서 두 클로저 리터럴로 쓰였다는 것 외엔 재료 부재(col 정보 없음) | 4 |  |
| 3 | 미탐색 | rnd 의 소비량 — new_target/update_path 내부 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

