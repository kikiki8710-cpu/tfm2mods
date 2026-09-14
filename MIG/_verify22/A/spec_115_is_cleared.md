---

### `115` is_cleared — 정글 캠프가 '비어 있고(리스폰 대기) 내가 도착+offset+1초 후에도 아직 안 나오는가' 판정

| 항목 | 값 |
|---|---|
| id | `passive_jungle__is_cleared` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passive_jungle10is_cleared` |
| 소스 | `game-ai\src\plan_legacy\old\passive_jungle.rs:841` |
| IR | `m04.ll` 62404~62516행 |
| 경로·가시성 | `game_ai::plan_legacy::old::is_cleared` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d408e0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(game_core::JungleType, usize, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | camp | JungleType (i8, range 0..6) | camp_idx(camp) 인덱스 + camp_pos(camp) 좌표. Morgard(4)/Serpen(5) 를 넣으면 camp_idx 가 panic(team_plan.rs:40) | 4 |
| 1 | 2 | team | usize (i64 %1) | next_respawn_tick[team] 인덱스(bounds <2, L62443/L62452) · camp_pos 의 `team == 0` 플래그(L62483). ⚠챔피언 조회는 이 team 이 아니라 player.info.team 을 씀 | 4 |
| 2 | 3 | _version | usize (i64 %2) | 미사용(dbg_value poison L62409) | 4 |
| 3 | 4 | _rnd | &mut StdRng (ptr %3, IR 속성 noalias readnone) | 미사용 — readnone 이라 읽기·쓰기 0. writes 없음 | 4 |
| 4 | 5 | player | &PlayerState (2528B) noalias readonly | info.team(0x930)·info.position(0x9c0) 만 읽음 | 4 |
| 5 | 6 | data | &OperationData (24B) noalias readonly | cache(+0)·context(+8) 읽음. blackboard(+0x10) 미사용 | 4 |
| 6 | 7 | team_plan | &TeamPlan (1064B) readonly (noalias 아님, nonnull) | next_respawn_tick(0x378) 만 읽음 | 4 |
| 7 | 8 | offset | usize (i64 %7) | 도착 예상틱에 더하는 여유틱(호출자 제공) | 4 |
| 8 | 9 | _debug | &mut DebugFrameData (ptr %8, noalias readnone) | 미사용 — readnone. writes 없음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn is_cleared(camp, team, _version, _rnd, player, data, team_plan, offset, _debug) -> bool {   // passive_jungle.rs:841
  // :842  내 챔피언(관측자 player 의 팀·포지션)
  let champ = data.cache.player_champion[player.info.team][player.info.position.as_index()];   // L62416~62434 (bounds team<2)
  if champ.is_none() { return false; }                                                      // :843 L62438~62439 → phi false
  // :849  이 캠프의 다음 리스폰 예정틱 (팀 인자 기준)
  let next_spawn_tick = team_plan.next_respawn_tick[team][camp_idx(camp)];                  // L62443~62459 (bounds team<2, idx<4; camp_idx: Rhino0 Mushroom1 Stump3 Bee2, Morgard/Serpen → panic team_plan.rs:40)
  // :851  아직 리스폰 전인가 (tick < next_spawn) — 아니면 캠프가 살아 있는 것 → false
  if !(game.tick() < next_spawn_tick) { return false; }                                     // L62461~62468 icmp ult %tick, %next → else phi false
  // :853  캠프까지 거리
  let dist = distance(champ.x, champ.y, map.camp_pos(camp, team == 0));                     // L62475~62488 (camp_pos 가 x,y 각각 1회씩 2번 call — 순수함수 CSE 미적용, 소스는 1회 호출로 추정)
  // :854  이동 소요틱
  let move_tick = dist / champ.stat_cached.move_speed;                                     // L62490~62496 udiv (speed 0 → panic)
  // :855  도착(+offset+1초) 시점에도 아직 리스폰 전이면 '클리어됨'
  next_spawn_tick > move_tick + offset + game.tick() + setting.tick_per_second               // L62498~62506 icmp ugt %34, (%62+%7+%63+%67)
}   // :859

[콜리 계약]
  camp_idx(JungleType) -> usize 0..4 (m09.ll:54334): 0→0 1→1 2(Stump)→3 3(Bee)→2, 4/5 → panic
  MapDef::camp_pos(&MapDef, JungleType, is_team0: bool) -> (u64,u64) (g07.ll:152570, TLS RefCell 메모 경유)
  utils::distance(x1,y1,x2,y2) -> u64 = isqrt(dx²+dy²) (g06.ll:87697)
  AbstractGame::tick (vtable +0x28) -> usize
```

**`mem` 메모리 접근 14건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L62416~62417, player_champion[team] 인덱스(bounds <2 → L62422 panic) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag (Position, i32) | r | L62427~62429 Position::as_index 인라인(entity.rs:581) = 태그값 그대로 0..4 (tcxdict --enum Position: Top0 Jungle1 Mid2 Bottom3 …) | 3 | OK |
| 2 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | L62430 | 4 | OK |
| 3 | OperationData | 0x8 | context (&GameContext) | r | L62479~62480 | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity>, null=None) | r | L62431~62434 gep 480 + team*40 + pos*8 (tcxdict AbstractGameWithCache 0x1e0) | 3 | OK |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr | r | L62461 | 4 | OK |
| 6 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | L62462~62465 슬롯 +0x28 = AbstractGame::tick (divtable). L62466·L62498 두 번 호출 | 3 | OK |
| 7 | TeamPlan | 0x378 | next_respawn_tick[team][camp_idx] ([[usize;4];2]) | r | L62456~62459 gep 888 + team*32 + idx*8 (tcxdict TeamPlan 0x378) | 3 | OK |
| 8 | GameContext | 0x20 | map (&MapDef 28112B) | r | L62481~62482 → camp_pos | 4 | OK |
| 9 | GameContext | 0x8 | setting (&GameSetting 5432B) | r | L62499~62500 | 4 | OK |
| 10 | GameSetting | 0x12f8 | tick_per_second | r | L62501~62502 (tcxdict GameSetting 0x12f8) — 여유 1초 | 3 | OK |
| 11 | Entity | 0x660 | x | r | L62475~62476 (내 챔피언) | 4 | OK |
| 12 | Entity | 0x668 | y | r | L62477~62478 | 4 | OK |
| 13 | Entity | 0x640 | stat_cached.move_speed | r | L62490~62491; 0 이면 L62510 panic_const_div_by_zero (passive_jungle.rs:854) | 4 | OK |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 842 | 임계 | player_champion 1차원 길이(팀 2) bounds check L62418 · next_respawn_tick 1차원 길이 bounds check L62443(:849). 판정값 아님 | 4 |
| 1 | 4 | 849 | 임계 | next_respawn_tick 2차원 길이(캠프 4종) bounds check L62448 `icmp ult %27, 4` / L62471 panic len 4. camp_idx 반환 range(0,4) 라 실제 도달 불가 | 4 |
| 2 | 0 | 853 | 태그 | `team == 0` → camp_pos 의 bool 인자(L62483~62484). 팀0 기준 캠프 좌표/대칭 선택 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 도착 여유 = tick_per_second(1초) | passive_jungle.rs:855 (L62501~62505, GameSetting+0x12f8) | tps(실전 60) | 리터럴이 아니라 세팅값. 여유를 늘리면 '곧 나올' 캠프도 클리어로 봐서 더 일찍 포기, 줄이면 캠프 앞에서 기다리는 쪽 | 4 | 기존 |
| 1 | offset 인자 | passive_jungle.rs:855 (%7) | 호출자 제공 | 값이 클수록 true 가 쉬워짐(=캠프를 클리어로 판정해 후보에서 제외) | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | camp_idx | game_ai::plan_legacy::team_plan::camp_idx | pub | fn(game_core::JungleType) -> usize | game-ai\src\plan_legacy\team_plan.rs:34 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | is_cleared | game_ai::plan_legacy::old::is_cleared | pub | fn(game_core::JungleType, usize, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:841 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 9 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

**호출처 11곳** (m04.ll:28389, m04.ll:29456, m04.ll:31315, m04.ll:31343, m04.ll:31873, m04.ll:31901, m04.ll:32198, m04.ll:32226, m04.ll:62529, m04.ll:62556, m04.ll:66338) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | team 인자와 player.info.team 이 다른 경우(상대 팀 캠프 판정?)의 의도 — 본문은 챔피언 조회만 player.info.team, 나머지는 team 인자. 호출자 3곳의 전달값은 이 명세 범위 밖(잎 라운드) | 4 |  |
| 1 | 표기 불가 | camp_pos 가 2번 호출되는 것이 소스에서 튜플 1회 호출인지 2회 호출인지 — IR 로는 구분 불가(표기 불가). 동작(x,y 동일 호출)은 확정 | 4 |  |
| 2 | 미탐색 | _version/_rnd/_debug 는 IR 상 완전 미사용(readnone/poison) — 시그니처 호환용으로 추정 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | next_respawn_tick 의 의미(리스폰 '예정틱'인지 '마지막 클리어틱+쿨'인지)는 TeamPlan 갱신 지점의 몫 — 이 함수는 tick<next 를 '비어 있음' 으로 해석한다는 것만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

