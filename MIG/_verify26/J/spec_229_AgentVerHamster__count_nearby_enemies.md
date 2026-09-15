---

### `229` AgentVerHamster::count_nearby_enemies — 적 챔피언 5슬롯 중 '최근 시야에 있고 champ 로부터 200000 이내'인 슬롯을 비트마스크(u16, bit i = 적 포지션 i)로 반환

| 항목 | 값 |
|---|---|
| id | `AgentVerHamster__count_nearby_enemies` |
| 심볼 | `_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster20count_nearby_enemies` |
| 소스 | `game-ai\src\lib.rs:830` |
| IR | `m14.ll` 37842~38171행 |
| 경로·가시성 | `game_ai::AgentVerHamster::count_nearby_enemies` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e8f850` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::Entity, &game_core::PlayerState, &game_core::OperationData) -> u16
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | champ.x | i64 (%0) | DI: x2/other = %0 (entity.rs:2157 distance_sq 인라인) | 4 |
| 1 | 1 | champ.y | i64 (%1) |  | 4 |
| 2 | 2 | player | &PlayerState(2528B) (%2) | DI !50899 player | 4 |
| 3 | 3 | data.cache | &AbstractGameWithCache(8840B) (%3) | null 검사는 llvm.assume(nonnull) 뿐 | 4 |
| 4 | 3 | data.blackboard | &[Blackboard;2] (%4) | 원소 stride 744B(Blackboard 크기) 로 인덱싱 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn count_nearby_enemies(champ, player, data) -> u16   // lib.rs:830~841
  count: u16 = 0                                        // L831
  enemy_team = 1 - player.info.team                     // L832 (+0x930). 0..1 아니면 panic_bounds_check(idx, len=2) — 도달 불가(팀은 항상 0/1)
  champs = &data.cache.player_champion[enemy_team]      // +0x1e0 + 40*enemy_team, [Option<&Entity>;5]
  for i in 0..5:                                        // L833 — 5회 완전 언롤(블록 %134/%25/%53/%81/%109)
    c = champs[i]; if c.is_none(): continue             // null 검사
    if !data.blackboard[enemy_team].is_recent_visible(data.cache.game, player, c): continue   // L834 (blackboard stride 744)
    dx = |c.x - champ.x|; dy = |c.y - champ.y|         // L835 → entity.rs:2158 distance_sq 인라인(abs_diff → common vec :3147)
    if dx*dx + dy*dy < 40000000001:                     // = dist ≤ 200000
      count |= 1 << i                                   // 슬롯0: zext(i1) · 슬롯1: or disjoint 2 · 슬롯2~4: or 4/8/16
  return count                                          // L841 · range 0..31

★의미: 반환값은 '개수'가 아니라 '어느 적 포지션이 근접·가시인가' 5비트 마스크다(popcount 하면 개수). `|=` 와 `+=` 는 비트가 서로소라 외연 동일 → 표기 불가(IR 는 or). 호출자 두 곳(update_state · update_small_action)은 이 값을 self.last_eval_nearby_enemies 와 비교하는 데 쓴다(그쪽 명세 참조).
```

**`mem` 메모리 접근 7건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | enemy_team = 1 - team (L832 · `sub i64 1, %7` · 결과가 0..1 아니면 panic_bounds_check) | 4 | OK |
| 1 | AbstractGameWithCache | 0x0 | game.data_ptr | r | dyn AbstractGame 팻포인터 앞절반 — is_recent_visible 인자로 전달(L834) | 4 | OK |
| 2 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 팻포인터 뒷절반(AbstractGame vtable 816B) — is_recent_visible 인자로 전달 | 4 | OK |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[enemy_team][0..5] | r | [[Option<&Entity>;5];2] · 팀 stride 40B · 슬롯 stride 8B. null=None. 5 슬롯 완전 언롤(L833) | 4 | OK |
| 4 | Entity | 0x660 | x (적 챔피언) | r | distance_sq 인라인(entity.rs:2158) | 4 | OK |
| 5 | Entity | 0x668 | y (적 챔피언) | r |  | 4 | OK |
| 6 | [Blackboard;2] | 0x0 | blackboard[enemy_team] | r | stride 744B 로 enemy_team 인덱싱 후 is_recent_visible 의 &self 로 전달(값은 직접 안 읽음) | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 40000000001 | 835 | 임계 | 200000² + 1 — 제곱거리 `ult` 비교 ⟹ dist ≤ 200000(=6.25셀) 이면 근접. 스칼라 200000 은 본문에 없음(제곱상수만) | 4 |
| 1 | 1 | 832 | 계수 | enemy_team = 1 - player.info.team (2팀 반전) | 4 |
| 2 | 2 | 832 | 임계 | 팀 인덱스 bounds(ult 2) · 동시에 슬롯1 마스크 비트값(or 2) | 4 |
| 3 | 4 | 835 | 계수 | 슬롯2 마스크 비트(1<<2) | 4 |
| 4 | 8 | 835 | 계수 | 슬롯3 마스크 비트(1<<3) | 4 |
| 5 | 16 | 835 | 계수 | 슬롯4 마스크 비트(1<<4) | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 근접 적 판정 반경(제곱) | lib.rs:835 · m14.ll:37935/37995/38055/38115/38168 | 40000000001 | 올리면 더 먼 적도 '근처 적'으로 집계돼 재평가 트리거(nearby 변화)가 잦아짐 · 내리면 둔감해짐. 5 언롤 사이트 전부 같은 값을 바꿔야 함 | 4 | 기존 |

<details><summary>`callees` 피호출자 2건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | count_nearby_enemies | game_ai::AgentVerHamster::count_nearby_enemies | in:game_ai | fn(&game_core::Entity, &game_core::PlayerState, &game_core::OperationData) -> u16 | game-ai\src\lib.rs:830 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 1 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

**호출처 2곳** (m14.ll:34843, m14.ll:37822) · **형제 90개** (AgentVerHamster)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::AgentVerHamster as std::clone::Clone>::clone | pub | game-ai\src\lib.rs:121 | True | fn(&game_ai::AgentVerHamster) -> game_ai::AgentVerHamster |
| 1 | <game_ai::AgentVerHamster as std::fmt::Debug>::fmt | pub | game-ai\src\lib.rs:121 | True | fn(&game_ai::AgentVerHamster, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::AgentVerHamster::plan_chats_drain | pub | game-ai\src\lib.rs:197 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::Chat, std::alloc::Global> |
| 3 | game_ai::AgentVerHamster::push_pending_trace_event | pub | game-ai\src\lib.rs:208 | False | fn(&mut game_ai::AgentVerHamster, usize, game_core::TraceEventType) |
| 4 | game_ai::AgentVerHamster::plan_goal | pub | game-ai\src\lib.rs:211 | False | fn(&game_ai::AgentVerHamster) -> std::option::Option<game_core::BigGoal> |
| 5 | game_ai::AgentVerHamster::plan_name | pub | game-ai\src\lib.rs:214 | False | fn(&game_ai::AgentVerHamster) -> std::string::String |
| 6 | game_ai::AgentVerHamster::battle_sub_goal_dir | pub | game-ai\src\lib.rs:218 | False | fn(&game_ai::AgentVerHamster) -> std::option::Option<i8> |
| 7 | game_ai::AgentVerHamster::battle_sub_goal_is_full_runaway | pub | game-ai\src\lib.rs:222 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 8 | game_ai::AgentVerHamster::plan_v3_cand_src | pub | game-ai\src\lib.rs:227 | True | fn(&game_ai::AgentVerHamster) -> u8 |
| 9 | game_ai::AgentVerHamster::plan_v3_last_stand | pub | game-ai\src\lib.rs:231 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 10 | game_ai::AgentVerHamster::plan_v3_final_stand | pub | game-ai\src\lib.rs:235 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 11 | game_ai::AgentVerHamster::plan_v3_bail_goal | pub | game-ai\src\lib.rs:239 | True | fn(&game_ai::AgentVerHamster) -> u8 |
| 12 | game_ai::AgentVerHamster::plan_ff_line_counts | pub | game-ai\src\lib.rs:243 | True | fn(&game_ai::AgentVerHamster) -> [usize; 4_usize] |
| 13 | game_ai::AgentVerHamster::plan_ff_retreat_stats | pub | game-ai\src\lib.rs:247 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize) |
| 14 | game_ai::AgentVerHamster::team_objective_code | pub | game-ai\src\lib.rs:254 | True | fn(&game_ai::AgentVerHamster) -> u8 |
| 15 | game_ai::AgentVerHamster::eo_cover_picks | pub | game-ai\src\lib.rs:258 | False | fn(&game_ai::AgentVerHamster) -> usize |
| 16 | game_ai::AgentVerHamster::eo_serpen_punish_issues | pub | game-ai\src\lib.rs:262 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 17 | game_ai::AgentVerHamster::subplan_is_recall | pub | game-ai\src\lib.rs:266 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 18 | game_ai::AgentVerHamster::plan_ff_call_stats | pub | game-ai\src\lib.rs:270 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize, usize, usize) |
| 19 | game_ai::AgentVerHamster::plan_gank_attempt_count | pub | game-ai\src\lib.rs:275 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 20 | game_ai::AgentVerHamster::plan_gank_periods | pub | game-ai\src\lib.rs:278 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, usize), std::alloc::Global> |
| 21 | game_ai::AgentVerHamster::plan_gank_score_attempts | pub | game-ai\src\lib.rs:281 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(i32, usize, usize), std::alloc::Global> |
| 22 | game_ai::AgentVerHamster::plan_gank_request_count | pub | game-ai\src\lib.rs:284 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 23 | game_ai::AgentVerHamster::plan_v46_lane_recall_stats | pub | game-ai\src\lib.rs:289 | True | fn(&game_ai::AgentVerHamster) -> (&std::vec::Vec<(usize, game_core::LineType), std::alloc::Global>, usize, usize, usize, usize, usize, usize, usize, usize) |
| 24 | game_ai::AgentVerHamster::plan_flee_death_retrospects | pub | game-ai\src\lib.rs:297 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, u8, u8, u8), std::alloc::Global> |
| 25 | game_ai::AgentVerHamster::plan_v48_dodge_stats | pub | game-ai\src\lib.rs:302 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) |
| 26 | game_ai::AgentVerHamster::plan_v46_flee_stats | pub | game-ai\src\lib.rs:310 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize) |
| 27 | game_ai::AgentVerHamster::plan_v46_flee_episodes | pub | game-ai\src\lib.rs:317 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, game_core::LineType, u8), std::alloc::Global> |
| 28 | game_ai::AgentVerHamster::plan_v50_dive_episodes | pub | game-ai\src\lib.rs:321 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<game_core::V50DiveEpisode, std::alloc::Global> |
| 29 | game_ai::AgentVerHamster::plan_epic_steal_attempt_count | pub | game-ai\src\lib.rs:324 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 30 | game_ai::AgentVerHamster::plan_epic_steal_success_count | pub | game-ai\src\lib.rs:327 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 31 | game_ai::AgentVerHamster::plan_serpen_steal_attempt_count | pub | game-ai\src\lib.rs:330 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 32 | game_ai::AgentVerHamster::plan_serpen_steal_success_count | pub | game-ai\src\lib.rs:333 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 33 | game_ai::AgentVerHamster::plan_steal_sessions | pub | game-ai\src\lib.rs:336 | False | fn(&game_ai::AgentVerHamster) -> std::vec::Vec<game_core::StealSession, std::alloc::Global> |
| 34 | game_ai::AgentVerHamster::plan_pending_trace_events_drain | pub | game-ai\src\lib.rs:339 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::PendingTraceEvent, std::alloc::Global> |
| 35 | game_ai::AgentVerHamster::plan_comeback_pick_stats | pub | game-ai\src\lib.rs:342 | False | fn(&game_ai::AgentVerHamster) -> (usize, usize, std::vec::Vec<(i32, u8), std::alloc::Global>) |
| 36 | game_ai::AgentVerHamster::plan_v54_cj_call_ticks | pub | game-ai\src\lib.rs:346 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<usize, std::alloc::Global> |
| 37 | game_ai::AgentVerHamster::plan_is_counter_jungle | pub | game-ai\src\lib.rs:350 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 38 | game_ai::AgentVerHamster::plan_v54_fs_fog_stats | pub | game-ai\src\lib.rs:357 | False | fn(&game_ai::AgentVerHamster) -> (usize, usize) |
| 39 | game_ai::AgentVerHamster::plan_v54_reentry_ticks | pub | game-ai\src\lib.rs:362 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<usize, std::alloc::Global> |
| 40 | game_ai::AgentVerHamster::new | pub | game-ai\src\lib.rs:368 | False | fn(&mut rand::rngs::std::StdRng, usize, usize, game_core::Position) -> game_ai::AgentVerHamster |
| 41 | game_ai::AgentVerHamster::init | pub | game-ai\src\lib.rs:413 | True | fn(&mut game_ai::AgentVerHamster) |
| 42 | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any | pub | game-ai\src\lib.rs:425 | True | fn(&game_ai::AgentVerHamster) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static |
| 43 | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any_mut | pub | game-ai\src\lib.rs:426 | True | fn(&mut game_ai::AgentVerHamster) -> &mut dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static |
| 44 | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | game-ai\src\lib.rs:428 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 45 | <game_ai::AgentVerHamster as game_core::AiAgent>::set_version | pub | game-ai\src\lib.rs:429 | True | fn(&mut game_ai::AgentVerHamster, usize) |
| 46 | <game_ai::AgentVerHamster as game_core::AiAgent>::debug_mut | pub | game-ai\src\lib.rs:430 | True | fn(&mut game_ai::AgentVerHamster) -> &mut game_core::DebugFrameData |
| 47 | <game_ai::AgentVerHamster as game_core::AiAgent>::small_action_current | pub | game-ai\src\lib.rs:431 | False | fn(&game_ai::AgentVerHamster) -> game_core::SmallAction |
| 48 | <game_ai::AgentVerHamster as game_core::AiAgent>::get_input | pub | game-ai\src\lib.rs:433 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> (std::option::Option<game_core::Input>, bumpalo::collections::vec::Vec< game_core::TurnEvent>) |
| 49 | <game_ai::AgentVerHamster as game_core::AiAgent>::buy_item | pub | game-ai\src\lib.rs:437 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> |
| 50 | <game_ai::AgentVerHamster as game_core::AiAgent>::upgrade_item | pub | game-ai\src\lib.rs:440 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> |
| 51 | <game_ai::AgentVerHamster as game_core::AiAgent>::update_on_dead | pub | game-ai\src\lib.rs:443 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 52 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_goal | pub | game-ai\src\lib.rs:447 | False | fn(&game_ai::AgentVerHamster) -> std::option::Option<game_core::BigGoal> |
| 53 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_debug_label | pub | game-ai\src\lib.rs:448 | False | fn(&game_ai::AgentVerHamster) -> std::string::String |
| 54 | <game_ai::AgentVerHamster as game_core::AiAgent>::push_game_event | pub | game-ai\src\lib.rs:449 | False | fn(&mut game_ai::AgentVerHamster, game_core::TurnEvent) |
| 55 | <game_ai::AgentVerHamster as game_core::AiAgent>::push_pending_trace_event | pub | game-ai\src\lib.rs:450 | False | fn(&mut game_ai::AgentVerHamster, usize, game_core::TraceEventType) |
| 56 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_chats_drain | pub | game-ai\src\lib.rs:453 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::Chat, std::alloc::Global> |
| 57 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_pending_trace_events_drain | pub | game-ai\src\lib.rs:454 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::PendingTraceEvent, std::alloc::Global> |
| 58 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_epic_steal_attempt_count | pub | game-ai\src\lib.rs:458 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 59 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_epic_steal_success_count | pub | game-ai\src\lib.rs:459 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 60 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_serpen_steal_attempt_count | pub | game-ai\src\lib.rs:460 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 61 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_serpen_steal_success_count | pub | game-ai\src\lib.rs:461 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 62 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_steal_sessions | pub | game-ai\src\lib.rs:462 | False | fn(&game_ai::AgentVerHamster) -> std::vec::Vec<game_core::StealSession, std::alloc::Global> |
| 63 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_attempt_count | pub | game-ai\src\lib.rs:463 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 64 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_request_count | pub | game-ai\src\lib.rs:464 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 65 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_periods | pub | game-ai\src\lib.rs:465 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, usize), std::alloc::Global> |
| 66 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_score_attempts | pub | game-ai\src\lib.rs:466 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(i32, usize, usize), std::alloc::Global> |
| 67 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_comeback_pick_stats | pub | game-ai\src\lib.rs:467 | False | fn(&game_ai::AgentVerHamster) -> (usize, usize, std::vec::Vec<(i32, u8), std::alloc::Global>) |
| 68 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_flee_death_retrospects | pub | game-ai\src\lib.rs:468 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, u8, u8, u8), std::alloc::Global> |
| 69 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v46_flee_stats | pub | game-ai\src\lib.rs:469 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize) |
| 70 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v46_flee_episodes | pub | game-ai\src\lib.rs:470 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, game_core::LineType, u8), std::alloc::Global> |
| 71 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v46_lane_recall_stats | pub | game-ai\src\lib.rs:471 | True | fn(&game_ai::AgentVerHamster) -> (&std::vec::Vec<(usize, game_core::LineType), std::alloc::Global>, usize, usize, usize, usize, usize, usize, usize, usize) |
| 72 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v48_dodge_stats | pub | game-ai\src\lib.rs:474 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) |
| 73 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v50_dive_episodes | pub | game-ai\src\lib.rs:477 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<game_core::V50DiveEpisode, std::alloc::Global> |
| 74 | game_ai::AgentVerHamster::get_play_type | pub | game-ai\src\lib.rs:552 | True | fn(&game_ai::AgentVerHamster) -> game_core::PlayType |
| 75 | game_ai::AgentVerHamster::update_state | pub | game-ai\src\lib.rs:556 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> |
| 76 | game_ai::AgentVerHamster::update_event | in:game_ai | game-ai\src\lib.rs:612 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 77 | game_ai::AgentVerHamster::update_plan | in:game_ai | game-ai\src\lib.rs:682 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> |
| 78 | game_ai::AgentVerHamster::update_plan_lapse | in:game_ai | game-ai\src\lib.rs:688 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, bool) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> |
| 79 | game_ai::AgentVerHamster::update_small_action | in:game_ai | game-ai\src\lib.rs:693 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 80 | game_ai::AgentVerHamster::can_skip_eval | in:game_ai | game-ai\src\lib.rs:796 | False | fn(&game_ai::AgentVerHamster, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 81 | game_ai::AgentVerHamster::count_nearby_enemies | in:game_ai | game-ai\src\lib.rs:830 | False | fn(&game_core::Entity, &game_core::PlayerState, &game_core::OperationData) -> u16 |
| 82 | game_ai::AgentVerHamster::get_input | pub | game-ai\src\lib.rs:843 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> (std::option::Option<game_core::Input>, bumpalo::collections::vec::Vec< game_core::TurnEvent>) |
| 83 | game_ai::AgentVerHamster::update_trace_escape_abandon | in:game_ai | game-ai\src\lib.rs:1088 | False | fn(&mut game_ai::AgentVerHamster, &game_core::OperationData) |
| 84 | game_ai::AgentVerHamster::update_on_dead | pub | game-ai\src\lib.rs:1113 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 85 | game_ai::AgentVerHamster::push_game_event | pub | game-ai\src\lib.rs:1120 | False | fn(&mut game_ai::AgentVerHamster, game_core::TurnEvent) |
| 86 | game_ai::AgentVerHamster::item_v26_slot | in:game_ai | game-ai\src\lib.rs:1125 | False | fn(&game_core::PlayerState, &[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>]) -> std::option::Option<(usize, std::option::Option<usize>)> |
| 87 | game_ai::AgentVerHamster::item_v26 | in:game_ai | game-ai\src\lib.rs:1149 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::GameContext, usize, std::option::Option<usize>) -> std::option::Option<usize> |
| 88 | game_ai::AgentVerHamster::buy_item | pub | game-ai\src\lib.rs:1173 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> |
| 89 | game_ai::AgentVerHamster::upgrade_item | pub | game-ai\src\lib.rs:1193 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> |

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L835 의 마스크 갱신이 소스에서 `count \|= 1 << i` 인지 `count += 1 << i` 인지 — 비트 서로소라 외연 동일, IR 는 or(슬롯1 만 disjoint 플래그). 표기 불가(동작은 확정) | 4 |  |
| 1 | 미탐색 | is_recent_visible 내부(g07.ll:157005 · game_core 경계) — 시그니처만: (self:&Blackboard, game:&dyn AbstractGame, viewer:&PlayerState, target:&Entity)->bool · 본문 관찰: vtable+248 슬롯(팀,핸들)→true 이면 즉시 true / 아니면 vtable+336 슬롯(핸들)→플레이어 없으면 false / 있으면 blackboard+0x1e0[target.position(+0x9c0)] + 120 ≥ 현재 tick(vtable+40). 즉 '지금 보이거나 최근 120틱(2초) 내 보임'. 여기까지만 관찰, 명세 대상 아님 | 4 |  |
| 2 | 미탐색 | enumerate 인덱스 i 가 exe 에서 실제로 포지션 순(0=Top…)인지는 player_champion 배열 채움 규칙(game_core) 소관 — 이 함수는 배열 인덱스 그대로 비트에 쓴다 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

