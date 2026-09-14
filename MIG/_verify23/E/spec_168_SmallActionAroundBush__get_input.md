---

### `168` SmallActionAroundBush::get_input — 수풀 목표 셀까지 check_cell(지역 규칙+타워 회피) 술어로 PathFinder 경로를 만들고/갱신해 이동 입력 생성, 디버그면 경로선·지역 로그 기록

| 항목 | 값 |
|---|---|
| id | `SmallActionAroundBush__get_input` |
| 심볼 | `_RNvMs6_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_21SmallActionAroundBush9get_input` |
| 소스 | `game-ai\src\small_action\around.rs:1195` |
| IR | `m08.ll` 105393~105815행 |
| 경로·가시성 | `game_ai::SmallActionAroundBush::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `dc2960` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionAroundBush, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<Input>(32B) | None = i64 -1 @+0x0 · Some = Input 32B memcpy | 4 |
| 1 | 1 | self | &mut SmallActionAroundBush(120B) | ★&mut. 읽기 +0x18 target_x · +0x20 target_y · +0x6d path_finder 태그 · +0x70 out_line(클로저 환경으로 &참조). 쓰기 = writes | 4 |
| 2 | 2 | version | usize | 분기 없음. new_tower_avoid_v3 · new_target · 클로저 환경[0](&version → check_cell 1번째) | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | ★&mut — 본문 직접 store 0. new_target/update_path 로 전달(소비량 path_finder 계층 미탐색) | 4 |
| 4 | 4 | player | &PlayerState(2528B, readonly) | +0x930 team · +0x9c0 position 태그. 클로저 환경[1] | 4 |
| 5 | 5 | data | &OperationData(24B, readonly) | +0x0 cache → player_champion · +0x8 context(+0x20 map · +0x3b debug). 클로저 환경[3] | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B, readonly) | 본문 읽기 0. 클로저 환경[2] → check_cell 3번째 | 4 |
| 7 | 7 | debug | &mut DebugFrameData(224B) | ★&mut — context.debug 이고 입력이 Move 일 때만 +0xa0 infos HashMap insert/push + add_line(lines Vec push). IR 속성 noalias·align 8 dereferenceable(224) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
SmallActionAroundBush::get_input(&mut self, version, rnd, player, data, ps, debug) -> Option<Input>   [around.rs:1195]
 entity = data.cache.player_champion[player.team][player.position].unwrap()          (L1196)
 tcx = min(self.target_x/32000, 29) ; tcy = min(self.target_y/32000, 29)             (L1197~1198; 지역 변수, 클로저에 &참조)
 tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, self.target_x, self.target_y)   (L1199)
 if self.path_finder.is_none():                                                        (L1200)
     self.path_finder = PathFinder::new_target(rnd, data.context, version, "around_bush", entity.x, entity.y, self.target_x, self.target_y,
         |sx,sy,nx,ny| check_cell(version, player, ps, data, &tower_dodge, self.out_line, sx,sy,nx,ny, tcx, tcy))   (L1201; 클로저 환경 8칸: &version, player, ps, data, &tower_dodge, &self.out_line, &tcx, &tcy)
     if self.path_finder.is_none(): return None                                        (L1206 — new_target 결과 None 가능, 105492)
 pf = self.path_finder.as_mut()  (Some 확정)
 pf.update_path(rnd, data.context, entity.x, entity.y, self.target_x, self.target_y, 같은 클로저(s_0))   (L1206)
 if self.path_finder.is_none(): return None                                            (L1210, 105613 — update_path 뒤 재검사)
 input = pf.get_input(player, data, SafeMoveWithSkill::Safe, false)                    (L1210)
 if data.context.debug && input is Move{x:mx, y:my}:                                    (L1211~1212)
     debug.infos.entry(entity.id).or_insert(Vec::new()).push(format!("region: {}", map.regions[entity.y/32000][entity.x/32000]))   (L1213; ex,ey ≥ 960000 이면 panic)
     debug.add_line(entity.x, entity.y, mx, my, Color(1,0,1,1))                          (L1214)
 return Some(input)                                                                    (L1218)

클로저 계약(관측, PathFinder 인스턴스 m03.ll 17819~21356 / 32437~33531 안 인라인): check_cell 12인자를 위 순서로 호출(m03 21150·32711), 반환 PathVerdict 태그로 switch(0 Allow → 계속 / 1 Soft / 2·3·4 → 위반 처리 — path_finder 계층). out_line 은 self.out_line 을 매 호출 로드.
```

**`mem` 메모리 접근 26건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 105426, <2 아니면 panic | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 105437 (player.rs:581 인라인) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | 105439 | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | Option<&Entity> null=None → unwrap_failed (105484, L1196) | 4 | OK |  |
| 4 | SmallActionAroundBush | 0x18 | target_x | r | 105452 (L1197) · 105496(재로드 L1206) | 4 | OK |  |
| 5 | SmallActionAroundBush | 0x20 | target_y | r | 105461 (L1198) | 4 | OK |  |
| 6 | SmallActionAroundBush | 0x6d | path_finder@tag | r | +0x28+0x45 Option<PathFinder> 니치, 2=None (105475·105530·105492·105613) | 4 | OK |  |
| 7 | SmallActionAroundBush | 0x28 | path_finder 페이로드 | r | PathFinder 72B 선두 — update_path/get_input 에 &mut 로 전달 (105472 `gep %1, 40`) | 4 | OK |  |
| 8 | SmallActionAroundBush | 0x50 | path_finder.path(Box ptr) | r | +0x28+0x28. old 값 drop 경로에서만 로드(105558, 1120B dealloc) | 4 | OK |  |
| 9 | SmallActionAroundBush | 0x58 | path_finder.planned_verdict(Box ptr) | r | 105537, 70B dealloc | 4 | OK |  |
| 10 | SmallActionAroundBush | 0x70 | out_line | r | 주소만 클로저 환경[5] 에 저장(105509·105592) — 값 읽기는 콜리 인스턴스 안(m03 21147) | 4 | OK |  |
| 11 | Entity | 0x660 | x | r | 내 위치 ex (105505·105588) | 4 | OK |  |
| 12 | Entity | 0x668 | y | r | ey | 4 | OK |  |
| 13 | Entity | 0x5c0 | id | r | 디버그 infos HashMap 키 (105655, L1213) | 4 | OK |  |
| 14 | OperationData | 0x8 | context | r | 105480·105503 | 4 | OK |  |
| 15 | GameContext | 0x3b | debug | r | bool — 디버그 로그 게이트 (105623, L1211) | 4 | OK |  |
| 16 | GameContext | 0x20 | map | r | 105749 → regions | 4 | OK |  |
| 17 | MapDef | 0x38b8 | regions[30][30] | r | regions[ey/32000][ex/32000] — 디버그 문자열 (105750, 인덱스 <30 아니면 panic) | 4 | OK |  |
| 18 | DebugFrameData | 0xa0 | infos | r | HashMap<usize, Vec<String>> — rustc_entry(105659) | 4 | OK |  |
| 19 | Input(지역 %15) | 0x8 | Move.x / +0x10 Move.y | r | 105647~105651 (L1212) — add_line 끝점 | 4 | OK |  |
| 20 | SmallActionAroundBush(self) | 0x28 | path_finder | w | path_finder 가 None 일 때만. memcpy 직전 old 값이 Some 이면 Box 2개 dealloc(105558~105580 — 진입 조건상 사장 경로) | 4 | OK | Some/None(PathFinder::new_target(rnd, ctx, version, "around_bush"(11B), ex, ey, target_x, target_y, closure#0)) 72B memcpy (105488, L1201) |
| 21 | SmallActionAroundBush(self) | 0x28 | path_finder(콜리 경유) | w | path_finder 계층 미탐색 — 갱신 오프셋 전수 불가(범위: 이 본문 한정) | 4 | OK | update_path(&mut)(105608) · get_input(&mut)(105621) 내부 갱신 |
| 22 | DebugFrameData(debug) | 0xa0 | infos[entity.id] | w | context.debug && input==Move 일 때만 (L1213) | 4 | OK | Vec<String> 없으면 빈 Vec 삽입(insert_no_grow 105695) 후 format!("region: {}", map.regions[ey/32000][ex/32000]) push(grow_one 105782 · memcpy 105797 · len+1 105799) |
| 23 | DebugFrameData(debug) | 0x18 | lines | w | 콜리 DebugFrameData::add_line(game_core) 내부 push | 4 | OK | add_line(ex, ey, move.x, move.y, Color{1.0,0.0,1.0,1.0}) (105808, L1214) |
| 24 | Option<Input>(sret) | 0x0 | tag/Input | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | -1 (105617·105631) 또는 32B memcpy (105642) |
| 25 | StdRng(rnd) | 0x0 | rng 상태 | w | 본문 직접 store 없음 | 4 | OK | new_target/update_path 내부 |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 32000 | 1197 | 계수 | 월드→셀: target_x/32000, target_y/32000 (105453·105462) · 디버그 ex/32000, ey/32000 (105734·105739) | 4 |
| 1 | 29 | 1197 | 인덱스 | 셀 인덱스 상한 clamp: min(x/32000, 29) (105457·105466, `llvm.umin`) — 30×30 격자 | 4 |
| 2 | 960000 | 1213 | 임계 | 30×32000 — 디버그 regions 인덱스 범위 검사(ey<960000, ex<960000 아니면 panic_bounds_check 30) (105735·105740) | 4 |
| 3 | 2 | 1200 | 센티널 | Option<PathFinder>::None 니치 태그 비교(105475 등 4곳). player_champion 팀 인덱스 상한 2 도 동일 리터럴(배열 크기) | 4 |
| 4 | -1 | 1206 | 센티널 | Option<Input>::None 니치 태그(105617·105631) | 4 |
| 5 | 11 | 1201 | 길이 | &'static str "around_bush" 길이 — PathFinder::new_target 의 key (@anon…104, 105525) | 4 |
| 6 | 1 | 1210 | 계수 | SafeMoveWithSkill::Safe(1) — PathFinder::get_input 4번째 (105621); 5번째 bool=false | 4 |
| 7 | 0 | 1211 | 태그 | Input 태그 0 = Move 비교(105626 `icmp eq i64 %106, 0`) — 디버그 로그 게이트 | 4 |
| 8 | 1120 | 1201 | 미상 | old PathFinder.path Box 크기 70×(u64,u64)=1120B dealloc (105558). 판정값 아님 | 4 |
| 9 | 70 | 1201 | 미상 | old planned_verdict Box [u8;70] dealloc (105580). 판정값 아님 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | PathFinder key | around.rs:1201 | "around_bush" | pf_stats 통계·경로 캐시 구분 이름 — 판정 영향 없음(계측 키) | 4 | 기존 |
| 1 | 이동 스킬 정책 | around.rs:1210 | Safe(1), false | Must 계열로 올리면 수풀 진입 시 이동 스킬을 더 적극 사용 | 4 | 기존 |
| 2 | 경로 셀 규칙 | check_cell 경유(self.out_line) | 생성자 인자 out_line | None 이면 타워 회피만, Outline/Inline 이면 라인 지역 규칙 추가(check_cell 명세) | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_line | game_core::DebugFrameData::add_line | pub | fn(&mut game_core::DebugFrameData, u64, u64, u64, u64, common::color::Color) | game-core\src\simulation\game\frame.rs:67 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check_cell | game_ai::small_action::around::check_cell | in:game_ai::small_action::around | fn(usize, &game_core::PlayerState, &game_core::PositioningScoreData, &game_core::OperationData, &game_ai::TowerDodgeContext, game_ai::AroundBushOutlineType, usize, usize, usize, usize, usize, usize) -> game_ai::PathVerdict | game-ai\src\small_action\around.rs:1256 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | check_cell | game_ai::SmallActionAroundHide::check_cell | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::PositioningScoreData, &game_core::OperationData, &game_ai::TowerDodgeContext, usize, usize, usize, usize, u64, u64) -> game_ai::PathVerdict | game-ai\src\small_action\around.rs:411 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | new_target | game_ai::PathFinder::new_target | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:66 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | new_tower_avoid_v3 | game_ai::TowerDodgeContext::new_tower_avoid_v3 | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1205 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 8개**: `__rust_dealloc`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `entry`, `format_inner`, `grow_one`, `insert_no_grow`, `or_insert`, `rustc_entry`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:43057) · **형제 10개** (SmallActionAroundBush)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionAroundBush as std::clone::Clone>::clone | pub | game-ai\src\small_action\around.rs:1138 | True | fn(&game_ai::SmallActionAroundBush) -> game_ai::SmallActionAroundBush |
| 1 | <game_ai::SmallActionAroundBush as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\around.rs:1138 | True | fn(&game_ai::SmallActionAroundBush, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionAroundBush::new | pub | game-ai\src\small_action\around.rs:1150 | False | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionAroundBush |
| 3 | game_ai::SmallActionAroundBush::new_with_out_line | pub | game-ai\src\small_action\around.rs:1154 | False | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush |
| 4 | game_ai::SmallActionAroundBush::new_with_target | pub | game-ai\src\small_action\around.rs:1178 | False | fn(&game_core::OperationData, &game_core::Entity, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush |
| 5 | game_ai::SmallActionAroundBush::get_input | in:game_ai | game-ai\src\small_action\around.rs:1195 | False | fn(&mut game_ai::SmallActionAroundBush, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 6 | game_ai::SmallActionAroundBush::update_state | in:game_ai | game-ai\src\small_action\around.rs:1221 | False | fn(&mut game_ai::SmallActionAroundBush, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 7 | game_ai::SmallActionAroundBush::get_action | in:game_ai | game-ai\src\small_action\around.rs:1240 | True | fn(&game_ai::SmallActionAroundBush) -> game_core::SmallAction |
| 8 | game_ai::SmallActionAroundBush::merge | in:game_ai | game-ai\src\small_action\around.rs:1244 | False | fn(&mut game_ai::SmallActionAroundBush, game_ai::SmallActionAroundBush) |
| 9 | game_ai::SmallActionAroundBush::is_end | in:game_ai | game-ai\src\small_action\around.rs:1251 | False | fn(&game_ai::SmallActionAroundBush, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | new_target 결과의 None 가능성 — tcx 시그니처는 `-> PathFinder` 이나 IR 이 memcpy 뒤 니치 태그 2 를 두 번(105530·105492) 검사. 실제 None 반환 여부는 path_finder 계층 미탐색(PositionBush::get_input 과 같은 의문) | 3 |  |
| 1 | 미탐색 | old path_finder drop 경로(105529~105580: 태그≠2 이면 Box dealloc) 는 진입 조건(태그==2)상 도달 불가로 보이나 컴파일러가 못 접었다 — 사장 여부는 reach 도구 미적용 | 4 |  |
| 2 | 미탐색 | PathFinder::get_input 5번째 bool(false) 의미 · update_path 가 self.path_finder 를 None 으로 되돌릴 수 있는지(L1210 재검사의 존재 이유) — path_finder 계층 | 4 |  |
| 3 | 미탐색 | rnd 소비량 — path_finder 계층 | 4 |  |
| 4 | 미탐색 | 디버그 선 색 상수 `float 1.000000e+00`/`0.000000e+00`(105801~105807, Color{1,0,1,1} 마젠타)는 qcspec C1 이 정수만 대조해 constants 에 못 싣는다(도구 한계) — 판정값 아님, 여기 기록 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

