---

### `247` AgentVerHamster::buy_item — 아이템 구매 판단 진입점: 아이템빌드가 있으면 v26 경로(슬롯→item_v26→tier 0 만 채택), 없으면 레거시 game_ai::buy_item 에 위임

| 항목 | 값 |
|---|---|
| id | `AgentVerHamster__buy_item` |
| 심볼 | `_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster8buy_item` |
| 소스 | `game-ai\src\lib.rs:1173` |
| IR | `m14.ll` 38174~38280행 |
| 경로·가시성 | `game_ai::AgentVerHamster::buy_item` · **pub** |
| 계층 | 기타 |
| exe | `e8fc80` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize>
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut AgentVerHamster(10704B) (%0) | DI self = %0 | 4 |
| 1 | 2 | rnd | &mut StdRng(320B) (%1) | DI rnd = %1 | 4 |
| 2 | 3 | player | &PlayerState(2528B) (%2) | DI player = %2 | 4 |
| 3 | 4 | game.data_ptr | *const dyn AbstractGame (%3) | tcx sig arg4 = &dyn AbstractGame | 3 |
| 4 | 4 | game.vtable_ptr | AbstractGame vtable ptr (%4) |  | 4 |
| 5 | 5 | context | &GameContext(64B) (%5) | DI context = %5 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn buy_item(&mut self, rnd, player, game:&dyn AbstractGame, context) -> Option<usize>   // lib.rs:1173~1190
  // L1174
  if player.info.item_builds.is_empty() {                         // +0x4f0 == 0
      // L1188 — 레거시(v26 이전) 위임
      return game_ai::buy_item(self.version, rnd, player, game, context)   // lib.rs:1477 · IR 은 version/game 을 poison(콜리 미사용) · 반환 그대로
  }
  // L1175
  item_list = &context.item_list                                   // +0x30
  (build_slot, inventory_index) = Self::item_v26_slot(player, item_list.as_slice())?   // sret 24B: +0 build_slot · +8 내부 Option 태그(usize::MAX=외부 None) · +16 payload
  // L1176
  if inventory_index.is_some() { return None }                   // 태그==1 — 슬롯의 아이템을 이미 보유 → 구매가 아니라 승급 소관
  // L1180
  item = self.item_v26(rnd, player, context, build_slot, None)?  // tag 상수 0 · payload 는 sret+16 값 그대로(무의미)
  // L1182
  if item_list[item].tier() == 0 { Some(item) } else { None }    // vtable+0x70 · 색인 bounds 패닉
  // L1190 ret

★역할 분담: buy_item 은 **tier 0(기본템) 만** 구매 후보로 돌려주고, tier≥1 은 upgrade_item(같은 item_v26 결과를 tier!=0 조건으로 채택)이 담당한다. 두 함수의 차이는 (inventory_index 유무, tier 조건) 뿐.
★item_build 훅 접점: 이 함수가 player.info.item_builds(Vec<usize> · item_list 인덱스) 를 읽어 비었으면 레거시로, 있으면 v26 로 가르므로 item_builds 를 채우는 쪽(바닐라 빌드 훅/개인전술)이 곧 이 분기의 스위치다.
```

**`mem` 메모리 접근 6건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x4f0 | info.item_builds.len | r | L1174 (gep 1264) — ==0 이면 레거시 경로. llvm.assume(len < 2^60)=Vec<usize> 상한 | 4 | OK |
| 1 | AgentVerHamster | 0x2910 | version | r | L1188 (gep 10512) load 되지만 호출 인자는 poison — 죽은 읽기(레거시 buy_item 이 첫 인자를 안 씀) | 4 | OK |
| 2 | GameContext | 0x30 | item_list | r | L1175 (gep 48) &Vec<Box<dyn ItemInfo>> | 4 | OK |
| 3 | Vec<Box<dyn ItemInfo>> | 0x8 | item_list.ptr | r | L1175 슬라이스 data | 4 | 확인불가(tcx 사전에 타입 없음) |
| 4 | Vec<Box<dyn ItemInfo>> | 0x10 | item_list.len | r | L1175 슬라이스 len · L1182 색인 bounds(panic_bounds_check) | 4 | 확인불가(tcx 사전에 타입 없음) |
| 5 | ItemInfo vtable | 0x70 | tier | r | L1182 fn(&self)->usize · ==0 이면 구매 후보 채택 | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 1174 | 태그 | item_builds.len()==0 → 레거시 / L1180 item_v26 에 inventory_index=None(tag 0) 전달 / L1182 tier()==0 채택 | 4 |
| 1 | -1 | 1175 | 센티널 | item_v26_slot 반환 Option<(usize,Option<usize>)> 의 외부 None 니치 = sret+8(내부 Option 태그 자리)==usize::MAX | 4 |
| 2 | 1 | 1176 | 태그 | 내부 Option<usize> 태그 Some — inventory_index 가 Some 이면(이미 들고 있는 슬롯 = 승급 대상) 구매 아님 → None | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 레거시/v26 경로 스위치 | lib.rs:1174 · m14.ll:38189 | item_builds.len()==0 | item_builds 가 비어 있으면 레거시 buy_item(lib.rs:1477) 만 돌고 v26 경로(빌드 추종)는 전혀 안 탄다. 데이터(빌드 목록) 로 제어되는 스위치 | 4 | 기존 |
| 1 | 구매 채택 tier 조건 | lib.rs:1182 · m14.ll:38265 | tier()==0 | 조건을 넓히면(예 tier<=1) 상위 티어 아이템도 '구매' 로 돌려줌 — upgrade_item 과 중복 채택 위험 | 4 | 기존 |

<details><summary>`callees` 피호출자 12건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | buy_item | game_ai::buy_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> | game-ai\src\lib.rs:1477 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 2 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | item_v26 | game_ai::AgentVerHamster::item_v26 | in:game_ai | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::GameContext, usize, std::option::Option<usize>) -> std::option::Option<usize> | game-ai\src\lib.rs:1149 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | item_v26_slot | game_ai::AgentVerHamster::item_v26_slot | in:game_ai | fn(&game_core::PlayerState, &[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>]) -> std::option::Option<(usize, std::option::Option<usize>)> | game-ai\src\lib.rs:1125 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | tier | game_core::ItemInfo::tier | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:437 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 7 | tier | <game_core::ModItemEntry as game_core::ItemInfo>::tier | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:499 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 8 | tier | <game_core::BaseItemInfo as game_core::ItemInfo>::tier | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:617 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 9 | upgrade_item | game_ai::upgrade_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1603 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 10 | upgrade_item | game_ai::AgentVerHamster::upgrade_item | pub | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1193 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 11 | upgrade_item | <game_ai::AgentVerHamster as game_core::AiAgent>::upgrade_item | pub | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:440 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
</details>

⚠**미매칭 3개**: `as_slice`, `item_builds`, `poison`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m14.ll:56755) · **형제 90개** (AgentVerHamster)

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
| 0 | 미탐색 | item_v26_slot(lib.rs:1125 · m14.ll:35237 · internal) 내부 — 어느 빌드 슬롯을 고르고 inventory_index 를 어떻게 매기는지: 계약만 = fn(player:&PlayerState, item_list:&[Box<dyn ItemInfo>]) -> Option<(build_slot:usize, inventory_index:Option<usize>)> (sret 24B · +0 build_slot · +8 내부 태그 0/1 · usize::MAX=외부 None · +16 payload) | 4 |  |
| 1 | 미탐색 | game_ai::buy_item(lib.rs:1477 · m14.ll:8267 · 레거시) 내부 — 계약만 = fn(version:usize, rnd, player, game:&dyn AbstractGame, context) -> Option<usize>. IR 이 version·game 을 poison 으로 넘기므로 콜리는 그 둘을 안 읽는다(콜리 정의 쪽 dead-arg) | 4 |  |
| 2 | 미탐색 | item_v26 에 넘기는 payload(sret+16) 가 None 일 때 어떤 값인지 — item_v26_slot 이 None 자리에 무엇을 쓰는지 안 봄(콜리는 tag 0 이면 안 읽음) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

