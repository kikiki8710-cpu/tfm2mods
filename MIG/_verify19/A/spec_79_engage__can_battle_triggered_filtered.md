---

### `79` engage::can_battle_triggered_filtered — 적 챔프 5명 중 '전투 빅골(Battle)이 판단지연 이상 지속·가시·영역내·무시대상 아님·1초 내 사거리 도달' 인 적이 하나라도 있으면 true

| 항목 | 값 |
|---|---|
| id | `engage__can_battle_triggered_filtered` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler6engage29can_battle_triggered_filtered` |
| 소스 | `game-ai\src\plan_legacy\handler\engage.rs:903` |
| IR | `m12.ll` 37713~38065행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::can_battle_triggered_filtered` · **pub** |
| 계층 | 플랜 핸들러 |
| exe | `e38c90` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, std::option::Option<(u64, u64, u64)>) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize | %0. 본문 분기 없음 — is_ignored_battle_enemy 에 전달만 | 4 |
| 1 | 2 | player | &PlayerState(2528B) | %1. info.team(0x930)·info.position(0x9c0)·info.parameter(0x180, AthleteParameter 744B) | 4 |
| 2 | 3 | data | &OperationData(24B) | %2. cache·context·blackboard | 4 |
| 3 | 4 | area_filter | Option<(u64,u64,u64)>(32B, by-ref dead_on_return) | %3. +0 태그(i1), +8 ax, +0x10 ay, +0x18 ar_sq(제곱 반경). Some 이면 적이 (ax,ay) 반경 안에 있어야 함 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// engage.rs:903~967
_t = ProfTimer::start(84) (prof::ENABLED 일 때만)                                          // :904
t = player.info.team; champ = cache.player_champion[t][pos].unwrap()                        // :905
enemy_team = 1 - t                                                                          // :909
for idx in 0..5 {                                                                           // :910
   c = player_champion[enemy_team][idx]?  (None → continue)                                 // :911
   p = player_state[enemy_team][idx]?                                                       // :912
   // :916 in_battle(blackboard.rs:168): blackboard[p.team].big_goal[p.position]
   if !(big_goal.1 == Some(BigGoal::Battle{focus: Some(_)})) continue                       // 태그 5 && focus 태그 != 0
   battle_tick = blackboard[enemy_team].big_goal[p.position].0                              // :923 big_goal_tick
   after_duration = game.tick().saturating_sub(battle_tick)                                 // :924 (vtable+0x28)
   judge_latency = player.info.parameter.judge_battle_latency()                             // :927  [_gcbc g15.ll:126049 = (480 - 4*min(judgement*judgement_mental_ratio/1000, 100)) / 10 → 8~48틱]
   if after_duration < judge_latency continue                                               // :928 — 적이 전투 빅골을 잡은 지 내 판단지연 미만이면 아직 인지 못함
   if let Some(my_team) = champ.team.player_team() {                                        // :932 (Neutral 이면 검사 생략)
      if c.visible_state[my_team] != Visible continue }                                     //   is_visible_from → 태그 0
   if let Some((ax, ay, ar_sq)) = area_filter {                                             // :936
      if dist_sq(c, (ax,ay)) > ar_sq continue }                                             // :937
   if is_ignored_battle_enemy(version, player, data, c, with_declared_dive=false) continue   // :943
   r = max_range_cached(data, c, champ)                                                     // :947 (적 c 기준 사거리)
   d = c.distance(champ)                                                                    // :948
   move_speed = c.stat_cached.move_speed                                                    // :949
   remain_dist = d.saturating_sub(r)                                                        // :950
   if remain_dist > tps * move_speed continue                                               // :953 — 적이 1초 안에 사거리에 못 들어오면 제외
   if remain_dist > move_speed*10 && champ.stat_cached.move_speed > move_speed continue     // :957 — 10틱 밖이고 내가 더 빠르면 제외
   return true                                                                              // :967 (%60 = idx<5)
}
return false
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | t. 2 이상이면 panic_bounds_check(:905) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 내 포지션(i32) — player_champion[t][pos] | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter | r | &AthleteParameter → judge_battle_latency() (:927). 적 p 의 0x930/0x9c0 도 읽음(:916) | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r |  | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | → setting.tick_per_second | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. in_battle 은 [p.team], big_goal_tick 은 [1-t] 로 인덱스(같은 팀이지만 표현이 다름) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | tick() 호출 대상 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 +0x28 = tick (divtable AbstractGame) | 3 | OK |  |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion | r | [t][pos]=내 챔프(None→unwrap_failed) / [1-t][idx]=적 챔프 c | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x230 | player_state[][] | r | [1-t][idx]=적 PlayerState p | 4 | OK |  |
| 10 | GameContext | 0x8 | setting | r |  | 4 | OK |  |
| 11 | GameSetting | 0x12f8 | tick_per_second | r | tps. :953 remain_dist > tps*move_speed | 4 | OK |  |
| 12 | Blackboard | 0xf0 | big_goal[pos].0 | r | [5] x 32B. .0 = big_goal_tick(:923, blackboard.rs:160) | 4 | OK |  |
| 13 | Blackboard | 0xf8 | big_goal[pos].1@tag | r | Option<BigGoal> 태그. 5 = Battle (in_battle, blackboard.rs:168) | 4 | OK |  |
| 14 | Blackboard | 0x100 | big_goal[pos].1@Some.0@Battle.focus@tag | r | focus Option 태그 != 0 (=Some) 이어야 in_battle | 4 | OK |  |
| 15 | Entity | 0x0 | team@tag | r | 내 챔프 team. 1(Neutral) 이면 가시성 검사 생략(player_team()=None, entity.rs:1136~1137) | 4 | OK |  |
| 16 | Entity | 0x8 | team@Player.0 | r | 내 팀 번호 → 적의 visible_state 인덱스 | 4 | OK |  |
| 17 | Entity | 0x38 | visible_state | r | [2] x 24B. 적 c 의 visible_state[내 팀] 태그 0 = Visible 이어야 통과(is_visible_from entity.rs:1483) | 4 | OK |  |
| 18 | Entity | 0x640 | stat_cached.move_speed | r | 적 c 의 이동속도(:949) 와 내 챔프 이동속도(:957 비교) | 4 | OK |  |
| 19 | Entity | 0x660 | x | r | 적 c — area_filter 거리(:937) | 4 | OK |  |
| 20 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 21 | (none) | 0x0 | - | w | 게임 상태 변경 없음. prof::PHASE_NANOS[84]/PHASE_CALLS[84] atomic 증가(계측, ENABLED 일 때만) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | - |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 5 | 916 | 태그 | BigGoal 메모리태그 5 = Battle (in_battle 인라인). 같은 5 가 :910 루프 상한(포지션 수)이기도 함 | 4 |
| 1 | 0 | 916 | 태그 | Battle.focus Option 태그 0 = None → focus 없으면 in_battle 아님. / :932 VisibleState 태그 0 = Visible | 4 |
| 2 | 1 | 932 | 태그 | TeamType 태그 1 = Neutral — 내 챔프가 Neutral 이면 가시성 검사 생략(`trunc i64 tag to i1`) | 4 |
| 3 | 10 | 957 | 계수 | remain_dist > move_speed*10 (적 이동속도 기준 10틱 거리) && 내 move_speed > 적 move_speed 이면 제외 | 4 |
| 4 | 84 | 904 | 산출값 | prof 위상 id 84 (ProfTimer::start). 판정 무관 계측 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 판단 지연 게이트 | engage.rs:928 (값은 player.rs:352~355 judge_battle_latency) | 480 | (480-4*min(judgement*mental/1000,100))/10 틱. 판단력이 높을수록 8틱까지 짧아짐. 선수 스탯 노브이지 코드 상수는 게임코어에 있음 — 이 함수에선 비교만 | 4 | 기존 |
| 1 | 사거리 도달 허용 시간 | engage.rs:953 | 1 | tps*1 (1초, 리터럴 없음 — tps 그대로 곱함). 적이 1초 이동으로 사거리에 닿아야 트리거. 계수를 올리면 더 먼 적도 트리거로 인정 | 4 | 기존 |
| 2 | 도주 가능 거리 계수 | engage.rs:957 | 10 | 적 이동속도×10 보다 멀고 내가 더 빠르면 무시. 올리면 '도주 가능' 판정이 줄어 트리거가 늘어남 | 4 | 기존 |

<details><summary>`callees` 피호출자 13건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | in_battle | game_core::Blackboard::in_battle | pub | fn(&game_core::Blackboard, usize) -> bool | game-core\src\simulation\game\blackboard.rs:167 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | is_ignored_battle_enemy | game_ai::plan_legacy::old::is_ignored_battle_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:887 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | judge_battle_latency | game_core::AthleteParameter::judge_battle_latency | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:351 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 8 | start | game_core::prof::start | pub | fn(usize) -> std::option::Option<game_core::prof::ProfTimer> | game-core\src\simulation\prof.rs:175 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | start | game_view::UIPhaseEffect::start | pub | fn(&mut game_view::UIPhaseEffect) | game-view\src\ui\match_ui\phase_effect.rs:30 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 11 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 2개**: `dist_sq`, `elapsed`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m12.ll:37707, m13.ll:38001) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | :953 의 `tps * move_speed` 에 계수 리터럴이 없어(`mul %tps, %speed`) '1초' 는 해석. 소스가 `tps*1*speed` 인지 `tps*speed` 인지 표기 불가 | 4 |  |
| 1 | 미탐색 | in_battle(:916) 은 blackboard[p.team] 로, big_goal_tick(:923) 은 blackboard[1-t] 로 인덱스 — 값은 같으나 소스 표현이 다른 것으로 보임. blackboard[적팀] 이 '적 팀 자체 정보(우리가 관측)' 라는 해석은 _docs game_core.txt:21 근거 | 5 |  |
| 2 | 미탐색 | is_ignored_battle_enemy(m10.ll:47629) 본문·max_range_cached(TLS 캐시, m00.ll:79611) 본문은 담당 밖 — 미탐색 | 4 |  |
| 3 | 미탐색 | judge_battle_latency 의 필드 +152(stat.judgement)·+720(judgement_mental_ratio) 는 tcxdict AthleteParameter 0x98/0x2d0 로 확인, 본문은 _gcbc g15.ll:126049 (담당 밖이라 참고만) | 3 |  |
| 4 | 미탐색 | exe 0xe38c90 대조: call 9개(Instant::now·[+0x28] tick·0xe0bf70·0xe0cf10·0x12a07d0·0xe2ca30·Instant::elapsed·panic 2) 로 IR 의 직접 call(judge_battle_latency·is_ignored_battle_enemy·max_range_cached·distance) 4개와 수가 맞음. 상수 0x27101/0xfa01(=160001/64001) 은 exe 가 is_ignored_battle_enemy 내부의 is_enemy_well_danger 를 인라인한 흔적으로 추정(IR 본문엔 없음), 0x199a=6554 는 정체 미확인(IR 본문에 없음 — 인라인된 피호출 함수 상수로 추정). 로직 어긋남은 발견 못 함 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

