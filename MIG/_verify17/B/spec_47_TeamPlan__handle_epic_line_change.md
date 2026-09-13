---

### `47` TeamPlan::handle_epic_line_change — 에픽(모르가드) 버프 압박 라인을 적 포탑 잔존 상태로 재선택 — 더 멀쩡한 라인이 있으면 PressChange 챗 + objective=PressEpic(그 라인)

| 항목 | 값 |
|---|---|
| id | `epic__handle_epic_line_change` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epicNtNtB6_9team_plan8TeamPlan23handle_epic_line_change` |
| 소스 | `game-ai\src\plan_legacy\old\epic.rs:684` |
| IR | `m09.ll` 7267~8155행 |
| 경로·가시성 | `game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `dce520` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType)
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut TeamPlan(1064B) | chats(+0xc0 Vec<Chat>) 에 push, objective(+0x41f Option<MainObjective>) 에 store | 4 |
| 1 | 2 | _version | usize | 미사용 | 4 |
| 2 | 3 | rnd | &mut StdRng(320B) | PlayerState::strategy 에만 전달 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | strategy 호출 self + info.team(+0x930) | 4 |
| 4 | 5 | data | &OperationData(24B) | +0x0 cache(포탑 슬롯·dyn game), +0x10 blackboard(&[Blackboard;2]) | 4 |
| 5 | 6 | _debug | &mut DebugFrameData | 미사용(readnone) | 4 |
| 6 | 7 | line | LineType(i8, 0=Top 1=Mid 2=Bottom) | 현재 압박 중인 라인. range(0,3) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn handle_epic_line_change(&mut self, _version, rnd, player, data, _debug, line)

[L688] strategy = player.strategy(rnd, game)             // Strategy 24B (스택)
       enemy = 1 - player.info.team(+0x930)
// 헬퍼(전부 인라인): outer(l) = cache.tower(l,enemy).or(cache.tower2(l,enemy))   // 바깥쪽 생존 포탑
//   first(l)  = outer(l).is_some_and(|t| !t.is_tower2())   // 1차 포탑 생존
//   second(l) = outer(l).is_some_and(|t|  t.is_tower2())   // 1차 파괴·2차 생존
//   score(l)  = (first(l), second(l))                        // bool 튜플, 사전식 비교(tuple.rs:195 gt 인라인)
//   change(l) = { self.chats.push(Chat::PressChange(l, 0)); self.objective = Some(PressEpic(l)) }

[L689] if line != Mid {
[L691~694]   // 현재 사이드 라인 vs Mid
[L696]   if score(Mid) > score(line) {           // = (mid_first && !line_first) || (mid_first==line_first && mid_second && !line_second)
[L697]     change(Mid) }
         return
       }

// ── line == Mid ──
[L702] if strategy.morgard_use(+0x4) 태그 < 5 /*Split131*/ { return }
[L706~712] top_first=first(Top) bottom_first=first(Bottom) top_second=second(Top) bottom_second=second(Bottom) mid_first=first(Mid) mid_second=second(Mid)
[L714] if strategy.morgard_use == 6 /*Split14*/ {
[L715]   epic_tick   = game.get_game_mode().moba().unwrap().jungle_runner.epic.next_respawn_tick(+0x1b0)
[L716]   serpen_tick = …serpen.next_respawn_tick(+0x1e0)
[L717]   if epic_tick > serpen_tick {           // 서펜이 먼저 뜬다
[L719]     object_far_line = Top
[L724]     if score(Bottom) > score(Mid) { change(Bottom) [L725] }
         } else {                                // 에픽이 먼저(또는 동시)
[L721]     object_far_line = Bottom
[L729]     if score(Top) > score(Mid) { change(Top) [L730] }
         }
         return
       }
// morgard_use == Gather(5)
[L735~736] if score(Top) > score(Mid) && score(Bottom) > score(Mid) {
[L738]   top_wave    = data.blackboard(+0x10)[team].top_minion_state.minion_count(+0x20)
[L739]   bottom_wave = …bottom_minion_state.minion_count(+0x70)
[L741]   if top_wave > bottom_wave (signed) { change(Top) [L742] } else { change(Bottom) [L745] }
[L748] } else if score(Top) > score(Mid) { change(Top) [L749] }
[L751] else if score(Bottom) > score(Mid) { change(Bottom) [L752] }
[L757] return

※ change() 의 objective store 는 공통 블록 %116 (phi 로 라인값 0/1/2) — 챗 push 뒤에 항상 같이 실행된다.
※ object_far_line 은 dbg_value 로만 남고 분기에 직접 쓰이지 않는다(후보 라인 선택으로 접힘).
```

**`mem` 메모리 접근 23건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | enemy = 1 - team (포탑 슬롯 인덱스, >=2 면 panic_bounds_check) / 우리 team 은 blackboard 인덱스 | 4 | OK |  |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache — 선두 16B = &dyn AbstractGame(strategy·get_game_mode 인자) | 4 | OK |  |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. [team] 의 미니언 수 비교(Gather 경로) | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x180 | top_tower[] | r | 라인별 1차 포탑 = cache + 0x180 + line*32 + team*8 (Top 0x180 / Mid 0x1a0 / Bottom 0x1c0). Option<&Entity> 니치 | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x190 | top_tower2[] | r | 라인별 2차 포탑 = cache + 0x190 + line*32 + team*8 (0x190/0x1b0/0x1d0). `tower.or(tower2)` = 바깥쪽 생존 포탑 | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1a0 | mid_tower[enemy] | r | 직접 상수 오프셋 416 으로도 읽는다(line 인자와 무관한 Mid 고정 조회) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x1b0 | mid_tower2[enemy] | r | 432 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x1c0 | bottom_tower[enemy] | r | 448 (Mid 분기에서 Bottom 고정 조회) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x1d0 | bottom_tower2[enemy] | r | 464 | 4 | OK |  |
| 9 | Entity(포탑) | 0x68 | ty@tag | r | ==2 → EntityType::Tower 일 때만 is_tower2 를 본다(아니면 is_tower2=false) | 4 | OK |  |
| 10 | Entity(포탑) | 0x128 | ty@Tower.info.ty (TowerType) | r | Tower::is_second_tower = 태그 >= 5 (Top2/Mid2/Bottom2). entity.rs:1309 Entity::is_tower2 인라인 | 4 | OK |  |
| 11 | Strategy(스택 24B, PlayerState::strategy 반환) | 0x4 | morgard_use (MorgardUseStrategy, i32 니치) | r | <5 = Split131(암묵) → 즉시 return / 5 = Gather / 6 = Split14 | 4 | OK |  |
| 12 | GameMode(get_game_mode 반환 {i64,ptr}) | 0x0 | tag | r | 0 = Moba(&MobaMode). 아니면 unwrap_failed(epic.rs:715/716) | 4 | OK |  |
| 13 | MobaMode | 0x1b0 | jungle_runner.epic.next_respawn_tick | r | dbg epic_tick | 4 | OK |  |
| 14 | MobaMode | 0x1e0 | jungle_runner.serpen.next_respawn_tick | r | dbg serpen_tick | 4 | OK |  |
| 15 | Blackboard[team] | 0x20 | top_minion_state.minion_count | r | i32, dbg top_wave | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 16 | Blackboard[team] | 0x70 | bottom_minion_state.minion_count | r | i32, dbg bottom_wave | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 17 | TeamPlan | 0xc0 | chats.cap | r | Vec<Chat> push 인라인: len==cap 이면 grow_one | 4 | OK |  |
| 18 | TeamPlan | 0xc8 | chats.ptr | r | elem 24B | 4 | OK |  |
| 19 | TeamPlan | 0xd0 | chats.len | r |  | 4 | OK |  |
| 20 | TeamPlan | 0xc0 | chats.cap | w | len(+0xd0) += 1. 라인을 바꿀 때만 | 4 | OK | push(Chat::PressChange(new_line, 0)) — 태그 22, +0x1 LineType, +0x8 usize 0 |
| 21 | TeamPlan | 0x41f | objective | w | 라인을 바꿀 때만 | 4 | OK | 5 = MainObjective::PressEpic |
| 22 | TeamPlan | 0x420 | objective@PressEpic.0 (LineType) | w |  | 4 | OK | new_line (0 Top / 1 Mid / 2 Bottom) |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 689 | 태그 | line == Mid(1) 로 두 분기(현재 Mid 압박 / 사이드 압박) 갈림 | 4 |
| 1 | 1 | 691 | 태그 | enemy = 1 - team | 4 |
| 2 | 2 | 691 | 태그 | 포탑 슬롯 배열 길이 2 (enemy>=2 면 panic_bounds_check) · EntityType 태그 2 = Tower | 4 |
| 3 | 5 | 691 | 임계 | TowerType < 5 = 1차 포탑(Top/Mid/Bottom/TwinA/TwinB) ⟺ !is_second_tower (tower.rs:100 인라인) ⚠`shl … , 5` 는 포탑 배열 stride(line*32) 접힘이지 이 임계가 아님 | 4 |
| 4 | 4 | 692 | 임계 | TowerType > 4 = 2차 포탑(Top2/Mid2/Bottom2) ⟺ is_second_tower | 4 |
| 5 | 22 | 697 | 태그 | Chat 메모리태그 22 = PressChange(LineType, usize) | 4 |
| 6 | 0 | 697 | 태그 | PressChange.1 (usize) 은 항상 0 | 4 |
| 7 | 5 | 702 | 센티널 | MorgardUseStrategy 니치: 태그 < 5 = Split131(암묵 variant) → 아무것도 안 하고 return. (5=Gather 6=Split14, tcxdict --enum) | 3 |
| 8 | 6 | 714 | 태그 | MorgardUseStrategy 메모리태그 6 = Split14 → 에픽/서펜 리스폰 시각으로 후보 라인 결정. 아니면(5 Gather) 양 사이드 다 비교 ⚠본문의 `shl i8 %6, 5` 는 line 레지스터(%6)*32 stride 접힘 — 이 상수와 무관 | 4 |
| 9 | 0 | 715 | 태그 | GameMode 태그 0 = Moba — 아니면 unwrap_failed | 4 |
| 10 | 0 | 719 | 태그 | LineType Top = 0 (object_far_line / PressChange 라인값) | 4 |
| 11 | 2 | 721 | 임계 | LineType Bottom = 2 | 4 |
| 12 | 5 | 697 | 태그 | objective 태그 5 = MainObjective::PressEpic (store i8 5, ptr self+0x41f) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 라인 교체 판정식 = 포탑 잔존 점수 사전식 비교 (1차 생존, 2차만 생존) | epic.rs:696·724·729·735·748·751 (m09.ll:7443 `xor i1 %39, %66` 등 tuple gt 인라인) | (first, second) 튜플 > | 후보 라인이 현재/Mid 보다 포탑이 더 남아 있어야 바꾼다. 동점이면 안 바꾼다. 비교 방향을 뒤집으면 더 뚫린 라인으로 간다 | 4 | 기존 |
| 1 | Split131 전략이면 Mid 재선택 자체를 안 함 | epic.rs:702 (m09.ll:7328 `icmp samesign ult i32 %32, 5`) | 5 | morgard_use 태그 <5 (Split131) 에서 즉시 return. 이 게이트를 없애면 131 분할 전략에서도 Mid→사이드 전환이 생긴다 | 4 | 기존 |
| 2 | Split14 에서 후보 라인 = 먼저 뜨는 오브젝트 반대편 | epic.rs:717 (m09.ll:7755 `icmp ugt i64 %220, %228`) | epic_tick > serpen_tick | 서펜이 먼저면 Bottom 후보, 에픽이 먼저(동시 포함)면 Top 후보. 부등호를 >= 로 바꾸면 동시 리스폰 때 Bottom 을 고른다 | 4 | 기존 |
| 3 | Gather 에서 양 사이드 동률 시 웨이브 큰 쪽 | epic.rs:741 (m09.ll:8066 `icmp sgt i32 %334, %336`) | top_wave > bottom_wave | 아군 블랙보드 미니언 수가 큰 라인을 고른다. 같으면 Bottom | 4 | 기존 |
| 4 | PressChange 챗 두 번째 값 | epic.rs:697 등 7곳 (`store i64 0, ptr %114`) | 0 | 항상 0 으로 push. 소비처는 이 함수 밖(미탐색) | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 1 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | handle_epic_line_change | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType) | game-ai\src\plan_legacy\old\epic.rs:684 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | is_tower2 | game_core::EntityType::is_tower2 | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | moba | game_core::MapDef::moba | pub | fn(&game_core::GameSetting) -> game_core::MapDef | game-core\src\simulation\map_def.rs:67 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 7 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 8 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 9 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 10 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 11 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 12 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 16 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 10개**: `bottom_wave`, `change`, `first`, `grow_one`, `minion_count`, `morgard_use`, `next_respawn_tick`, `outer`, `second`, `tower2`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:33961) · **형제 55개** (TeamPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic | pub | game-ai\src\plan_legacy\old\epic.rs:502 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 1 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window | in:game_ai | game-ai\src\plan_legacy\old\epic.rs:634 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool |
| 2 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change | pub | game-ai\src\plan_legacy\old\epic.rs:684 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType) |
| 3 | <game_ai::plan_legacy::team_plan::TeamPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> game_ai::plan_legacy::team_plan::TeamPlan |
| 4 | <game_ai::plan_legacy::team_plan::TeamPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 5 | <game_ai::plan_legacy::team_plan::TeamPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn() -> game_ai::plan_legacy::team_plan::TeamPlan |
| 6 | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | game-ai\src\plan_legacy\team_plan.rs:196 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) |
| 7 | game_ai::plan_legacy::team_plan::TeamPlan::sanitize_rule_scope | pub | game-ai\src\plan_legacy\team_plan.rs:200 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::GameContext) |
| 8 | game_ai::plan_legacy::team_plan::TeamPlan::objective_target | pub | game-ai\src\plan_legacy\team_plan.rs:230 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> std::option::Option<game_core::JungleType> |
| 9 | game_ai::plan_legacy::team_plan::TeamPlan::take_active | pub | game-ai\src\plan_legacy\team_plan.rs:243 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 10 | game_ai::plan_legacy::team_plan::TeamPlan::take_hunt_commit | pub | game-ai\src\plan_legacy\team_plan.rs:248 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 11 | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | game-ai\src\plan_legacy\team_plan.rs:257 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 12 | game_ai::plan_legacy::team_plan::TeamPlan::mark_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:265 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType, usize) |
| 13 | game_ai::plan_legacy::team_plan::TeamPlan::clear_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:277 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan) |
| 14 | game_ai::plan_legacy::team_plan::TeamPlan::init | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:281 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 15 | game_ai::plan_legacy::team_plan::TeamPlan::update | pub | game-ai\src\plan_legacy\team_plan.rs:294 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies | pub | game-ai\src\plan_legacy\team_plan.rs:444 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 17 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | game-ai\src\plan_legacy\team_plan.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 18 | game_ai::plan_legacy::team_plan::TeamPlan::update_wave_priority_clear_line | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:540 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 19 | game_ai::plan_legacy::team_plan::TeamPlan::should_player_clear_wave_priority_line | pub | game-ai\src\plan_legacy\team_plan.rs:561 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> |
| 20 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_morgard_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:570 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 21 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_serpen_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:574 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 22 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_object_setup_for_wave_priority | pub | game-ai\src\plan_legacy\team_plan.rs:578 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 23 | game_ai::plan_legacy::team_plan::TeamPlan::should_keep_object_for_contested_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:586 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool |
| 24 | game_ai::plan_legacy::team_plan::TeamPlan::repair_misunderstood_objective | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:603 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 25 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective | pub | game-ai\src\plan_legacy\team_plan.rs:722 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 26 | game_ai::plan_legacy::team_plan::TeamPlan::update_steal | pub | game-ai\src\plan_legacy\team_plan.rs:732 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) |
| 27 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective_after_steal | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:910 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool |
| 29 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool |
| 30 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_release_to_passive | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:177 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 31 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_relevant_lanes_ready | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:199 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 32 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> |
| 33 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_wait_pos | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:316 | False | fn(game_core::JungleType, usize, &game_core::MapDef) -> (u64, u64) |
| 34 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_entity | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:324 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData, game_core::JungleType) -> std::option::Option<&game_core::Entity> |
| 35 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::update_v27_objective_discipline | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_active_objective_discipline | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectiveDisciplineState> |
| 37 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_blocks_battle | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:497 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData) -> bool |
| 38 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> |
| 39 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_repair_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:7 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) |
| 40 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:16 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 41 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:29 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 42 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_line_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:46 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 43 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_dive_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:88 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 44 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_tower_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:141 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 45 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:253 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 46 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_split_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 47 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_comeback_pick_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:393 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType, bool) -> bool |
| 48 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_serpen_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:558 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 49 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_morgard_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:685 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 50 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_none_or_gank_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 51 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_gank_preprocess | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1133 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, game_core::LineType) -> bool |
| 52 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_defense | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1227 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 53 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_attack | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1237 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> |
| 54 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_sub_objective | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1258 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | Chat::PressChange 의 두 번째 usize(항상 0) 의 의미와 소비처 — 이 함수 밖. 미탐색 | 4 |  |
| 1 | 미탐색 | Blackboard.top_minion_state.minion_count 가 '우리 미니언 수'인지 '적 미니언 수'인지 — 이름만 근거. blackboard[team](우리 팀 판) 인 것은 IR 확정 | 4 |  |
| 2 | 표기 불가 | object_far_line(Top/Bottom) 이 dbg 로만 남아 원래 소스에서 어떤 식으로 후보 라인을 골랐는지(예: opposite(object_far_line)) 는 표기 불가. 동작(Top 후보/Bottom 후보)은 확정 | 4 |  |
| 3 | 표기 불가 | score 튜플의 두 번째 성분이 (second(l), second(Mid)) 인지 (second(l) && !second(Mid), false) 인지 — 외연 동일(둘 다 second(l) && !second(Mid))이라 표기 불가 | 4 |  |
| 4 | 미탐색 | Gather 경로에서 L748 `else if score(Top) > score(Mid)` 는 L735 가 참이고 L736 이 거짓일 때만 도달하므로 항상 참 — IR 이 %215 를 재사용하는 것으로 확인. 소스 구조(if a&&b / else if a / else if b) 는 추정 | 4 |  |
| 5 | 미탐색 | PlayerState::strategy 내부(_gcbc) 안 봄 — rnd 를 받으므로 확률적일 수 있음 | 4 |  |
| 6 | 미탐색 | exe 0xdce520 은 이번에 대조하지 않음(지시에 확정으로 적혀 있어 재검증 생략) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

