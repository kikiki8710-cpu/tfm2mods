---

### `161` lane_minion_position_action — 서포터 제외, 팔로우 사거리 안 적 라인 미니언 중 target_score 최대를 골라 LaneMinionPosition 소액션(목표는 choose_goal)을 만든다

| 항목 | 값 |
|---|---|
| id | `lane_minion__lane_minion_position_action` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai12small_action11lane_minion27lane_minion_position_action` |
| 소스 | `game-ai\src\small_action\lane_minion.rs:20` |
| IR | `m11.ll` 54240~54538행 |
| 경로·가시성 | `game_ai::lane_minion_position_action` · **pub** |
| 계층 | 기타 |
| exe | `e2a830` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>, usize, game_ai::PositionEvalPurpose) -> std::option::Option<game_ai::SmallActionPlay>
```

<details><summary>인자 10개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<SmallActionPlay> (184B) | sret([184 x i8]) writeonly. 태그 = +0xb1(177): None = i8 -1(=0xff · 54283/54319/54438/54457/54499/54535 6곳), Some(LaneMinionPosition) = 13(54528). Some 일 때 live 바이트 = [0x0,0x80) SmallActionLaneMinionPosition 128B memcpy(54527) + 0xb1 태그. 128B 안 live: +0 start_tick, +8 target(id), +0x10 goal_x, +0x18 goal_y, +0x20 goal_score, +0x28 end_delay, +0x75 path_finder@tag(=2 None · 54486), +0x78 position_eval_purpose. [0x30,0x75)·0x76~0x77·[0x79,0x80) = alloca 잔류(path_finder None 페이로드·패딩 미기록). [0x80,0xb1)·[0xb2,0xb8) 미기록. None 이면 0xb1 외 전부 미기록 | 4 |
| 1 | 1 | version | usize | 본 함수 분기 없음. has_current_explicit_minion_action·choose_goal 에 전달(스택 %20 경유 · 클로저 캡처 &version 은 target_score 가 안 읽어 poison 전달됨) | 4 |
| 2 | 2 | rnd | &mut StdRng (320B) | IR 속성: noalias align 16 dereferenceable(320) — readonly 없음 = &mut. 본 함수는 직접 읽지도 쓰지도 않고 클로저(+72 캡처)→target_score 의 마지막 인자로만 전달. 본 함수 자체의 writes 0 | 4 |
| 3 | 3 | data | &OperationData (24B) | readonly. cache(+0) 만 직접 사용 | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly. info.position(0x9c0)·info.team(0x930) | 4 |
| 5 | 5 | line | LineType (i8 0..3) | 필터 술어(캡처 &line)·has_current_explicit_minion_action·choose_goal 에 전달 | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData (2760B) | readonly. choose_goal 에만 전달(54491~54492) | 4 |
| 7 | 7 | wave_snapshot | Option<&MinionWaveSnapshot (2320B)> | dereferenceable_or_null · null=None. 클로저 캡처(+64 → 스택 %18)→ target_score 인자 | 4 |
| 8 | 8 | end_delay | usize | SmallActionLaneMinionPosition.end_delay(+0x28) 에 그대로 저장(54488) | 4 |
| 9 | 9 | position_eval_purpose | PositionEvalPurpose (i8 0..13) | +0x78 저장(54490) + choose_goal 인자 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn lane_minion_position_action(version, rnd:&mut StdRng, data, player, line, positioning_score, wave_snapshot, end_delay, position_eval_purpose) -> Option<SmallActionPlay>
[L23] if player.info.position == Support(4) { return None }              ; 54278
[L27] champ = data.cache.player_champion[player.info.team][player.info.position]?   ; null → None(54319)
[L28] atk = champ.attack_effect.as_ref()?                                 ; 태그 0x4c0 == -1 → None(54438)
[L30] (target, target_score) = data.cache.iter_minions(1 - player.info.team)   ; 적 미니언 체인 이터레이터(56B)
  [L32~45] .filter(|target| closure$0):                                   ; aux 40688~40855
     L32: champ.team==Player(t) ⇒ target.visible_state[t]==Visible 아니면 false (Neutral 이면 통과)
     L35: target.ty is Minion(1) 아니면 false
     L38: target.ty.Minion.info.line == line 아니면 false
     L42: follow_range = atk.range + 64000 + champ.stat_buff_cached.range + (champ.level-1)*atk.growth_range + atk.range_adjust(champ,target) + Entity::radius(champ)
     L43: follow_range += Entity::radius(target)
     L44: dist_sq(target,champ) <= follow_range^2
  [L46] .filter_map(|target| Some((target, SmallActionLaneMinionPosition::target_score(version, data, player, champ, atk, target, wave_snapshot, rnd).1)))   ; 항상 Some(ptr 니치 non-null · 55505~55507). version·player 는 콜리가 안 읽어 poison
  [L47] .max_by_key(|(_, score)| *score)                                     ; 동점이면 뒤 원소(compare>0 일 때만 앞 유지 · 15117~15118)
  ?                                                                          ; 후보 0 → None(54457)
[L50] if SmallActionLaneMinionPosition::has_current_explicit_minion_action(version, data, player, line, target) { [L51] return None }   ; 54450→54499
[L54] action = SmallActionLaneMinionPosition::new(data, target.id, end_delay, target_score, position_eval_purpose)   ; lane_minion.rs:140 인라인: start_tick = data.cache.game.tick()(vtable+0x28), goal_x=goal_y=1987654321, goal_score=target_score, path_finder=None(+0x75=2)
[L56] match SmallActionLaneMinionPosition::choose_goal(target_score, position_eval_purpose, version, player, data, champ, target, positioning_score, line) {   ; sret 32B Option<(x,y,score)>: +0 tag(1=Some), +8 x, +16 y, +24 score
  Some((gx,gy,gs)) => { [L57] action.goal_x=gx; [L58] action.goal_y=gy; [L59] action.goal_score=gs; [L61] Some(SmallActionPlay::LaneMinionPosition(action)) }   ; 태그 13
  None => None }                                                              ; 54535

```

**`mem` 메모리 접근 23건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x9c0 | info.position@tag | r | 54275. ==4(Support) 면 None(54278). player_champion 2차 인덱스 | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | 54287, bounds<2(54289). 1-team 이 iter_minions 인자(54326) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | 54300 | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | 54301~54304. null → None | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame data ptr) | r | 54470 | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x8 | game vtable ptr | r | 54471 → vtable+0x28 = AbstractGame::tick (divtable) · 54473~54474 간접호출 | 3 | OK |  |
| 6 | Entity | 0x4c0 | attack_effect@tag (champ, 니치 -1=None) | r | 54312. None → None 반환(54438) | 4 | OK |  |
| 7 | Entity | 0x490 | attack_effect (Effect 56B 선두, champ) | r | 54323 → 클로저 캡처 atk. 술어에서 +0x10 range(=0x4a0)·+0x18 growth_range(=0x4a8) 읽음(40754·40756) | 4 | OK |  |
| 8 | Entity | 0x5c0 | id (best target) | r | 54462 → action.target | 4 | OK |  |
| 9 | Entity | 0x0 | team@tag (champ) [aux closure$0] | r | 40701~40702(trunc). Neutral 이면 가시성 검사 생략 | 4 | OK |  |
| 10 | Entity | 0x8 | team@Player.0 (champ) [aux] | r | 40706 | 4 | OK |  |
| 11 | Entity | 0x38 | visible_state[team]@tag (target) [aux] | r | 40715 + team*24(gepS) · ==0 Visible(40718) | 4 | OK |  |
| 12 | Entity | 0x68 | ty@tag (target) [aux] | r | 40726, ==1 Minion(40728) | 4 | OK |  |
| 13 | Entity | 0x11a | ty@Minion.info.line@tag (target) [aux] | r | 40742, == *line(40747) | 4 | OK |  |
| 14 | Entity | 0x5c8 | level (champ) [aux] | r | 40758 (level-1)*growth_range | 4 | OK |  |
| 15 | Entity | 0x438 | stat_buff_cached.range (champ) [aux] | r | 40762 | 4 | OK |  |
| 16 | Entity | 0x470 | stat_buff_cached.radius_mult (champ·target) [aux] | r | 40766 / 40790 | 4 | OK |  |
| 17 | Entity | 0x680 | radius (champ·target) [aux] | r | 40773·40780 / 40797·40804 | 4 | OK |  |
| 18 | Entity | 0x660 | x (target·champ) [aux] | r | 40823 / 40831 | 4 | OK |  |
| 19 | Entity | 0x668 | y (target·champ) [aux] | r | 40827 / 40835 | 4 | OK |  |
| 20 | sret | 0xb1 | Option<SmallActionPlay> 태그 | w | 위 signature 참조 | 4 | 확인불가(tcx 사전에 타입 없음) | -1(None) / 13(LaneMinionPosition) |
| 21 | sret | 0x0..0x80 | SmallActionLaneMinionPosition | w | 54476~54490 스택 %16 에 조립 → 54523~54525 goal 덮어쓰기 → 54527 memcpy 128B | 4 | 확인불가(tcx 사전에 타입 없음) | start_tick=tick(), target=id, goal_x/y=choose_goal, goal_score, end_delay, path_finder=None(+0x75=2), purpose |
| 22 | rnd(&mut StdRng) | - | - | w | target_score(콜리) 가 &mut 로 받음 — 부작용은 그 명세 소관 | 4 | 확인불가(오프셋 파싱 실패) | 본 함수 직접 쓰기 없음 |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 23 | 태그 | Position 태그 4 = Support(tcxdict --enum Position). 서포터는 라인 미니언 포지셔닝 안 함 · 54278 | 3 |
| 1 | -1 | 24 | 센티널 | Option<SmallActionPlay> None 니치 태그(i8 0xff, +0xb1) · 54283 등 6곳 | 4 |
| 2 | 13 | 61 | 센티널 | SmallActionPlay 니치 태그 13 = LaneMinionPosition · 54528 | 4 |
| 3 | 1987654321 | 54 | 센티널 | goal_x/goal_y 초기 센티널(미설정 표식). choose_goal Some 이면 덮어씀 · 54480/54482 | 4 |
| 4 | 2 | 54 | 센티널 | Option<PathFinder> None 니치 태그(bool 니치, +0x75) · 54486 | 4 |
| 5 | 64000 | 42 | 미상 | [aux closure$0] follow_range 여유 = 2셀(32000*2). IR 은 재결합으로 effect.rs:26 위치에 붙였으나(dloc 62398) 인라인 뿌리는 lane_minion.rs:42 · 40814 | 4 |
| 6 | 100 | 42 | 미상 | [aux] Entity::radius() 인라인 radius*(mult+100)/100 · 40784/40808 | 4 |
| 7 | 1 | 35 | 태그 | [aux] EntityType 태그 1 = Minion · 40728 | 4 |
| 8 | 0 | 32 | 미상 | [aux] VisibleState 0 = Visible(40718) / TeamType 0 = Player(40701 trunc) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 팔로우 사거리 여유(2셀) | lane_minion.rs:42 (IR 40814, aux closure$0) | 64000 | 올리면 사거리보다 더 먼 라인 미니언도 포지셔닝 대상 후보에 들어간다(대상 선정 폭 확대). 내리면 사거리 근접 미니언만 | 4 | 기존 |
| 1 | 서포터 제외 | lane_minion.rs:23 (IR 54278) | 4 | Position 태그 비교. 다른 값으로 바꾸면 그 포지션이 라인 미니언 포지셔닝을 안 하게 된다 | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | choose_goal | game_ai::SmallActionLaneMinionPosition::choose_goal | in:game_ai | fn(&game_ai::SmallActionLaneMinionPosition, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, game_core::LineType) -> std::option::Option<(u64, u64, i64)> | game-ai\src\small_action\lane_minion.rs:493 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | has_current_explicit_minion_action | game_ai::SmallActionLaneMinionPosition::has_current_explicit_minion_action | in:game_ai | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool | game-ai\src\small_action\lane_minion.rs:187 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | lane_minion_position_action | game_ai::lane_minion_position_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>, usize, game_ai::PositionEvalPurpose) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\small_action\lane_minion.rs:20 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | target_score | game_ai::SmallActionLaneMinionPosition::target_score | in:game_ai | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Effect, &game_core::Entity, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut rand::rngs::std::StdRng) -> std::option::Option<i64> | game-ai\src\small_action\lane_minion.rs:294 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 9 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 2개**: `dist_sq`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m14.ll:22206, m14.ll:26650) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | target_score 내부(43890~44235, 이 배치 밖 중간함수). 계약만: (version:i64 [미사용→poison], data:&OperationData, player:&PlayerState [미사용→poison], champ:&Entity, atk:&Effect(56B), target:&Entity, wave_snapshot:Option<&MinionWaveSnapshot>, rnd:&mut StdRng) -> (i64, i64) — .1 이 score(max_by_key 키). .0 의 의미 미확인 | 4 |  |
| 1 | 미탐색 | has_current_explicit_minion_action 내부(45591~45768, internal fastcc). 계약: (version, data, player, line, target:&Entity) -> bool(true=이미 명시적 미니언 행동이 있어 포지셔닝 None) | 4 |  |
| 2 | 미탐색 | choose_goal 내부(43376~43887, internal fastcc). 계약: (sret Option<(u64,u64,i64)> 32B, target_score:i64, purpose:i8, version:i64, player, data, champ, target, positioning_score:&PositioningScoreData, line:i8). None 이면 소액션 전체가 None | 4 |  |
| 3 | 미탐색 | iter_minions 56B 체인 구조(슬라이스 3개)의 의미 — 콜리 내부 안 팜 | 4 |  |
| 4 | 미탐색 | L30~48 한 문장 안의 `?` 위치(라인 30 귀속은 !dbg 의 2790<30 로 확정, 표기만) | 4 |  |
| 5 | 미탐색 | PositioningScoreData(2760B)·MinionWaveSnapshot 을 본 함수는 안 읽음 — 콜리에 전달만 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

