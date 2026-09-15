---

### `225` AgentVerHamster::item_v26 — v26 빌드 경로: 활성 아이템빌드의 build_slot 번째 목표 아이템을 잡고, 인벤 현재템→목표로 가는 다음 아이템(또는 빌드경로 첫 아이템)을 골 조건까지 검사해 반환

| 항목 | 값 |
|---|---|
| id | `AgentVerHamster__item_v26` |
| 심볼 | `_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster8item_v26` |
| 소스 | `game-ai\src\lib.rs:1149` |
| IR | `m14.ll` 38283~38797행 |
| 경로·가시성 | `game_ai::AgentVerHamster::item_v26` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e8fd70` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::GameContext, usize, std::option::Option<usize>) -> std::option::Option<usize>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | (제거됨) | tcx sig 에만 존재 | 3 |
| 1 | 2 | rnd | &mut StdRng(320B) (%0) | DI rnd = %0 | 4 |
| 2 | 3 | player | &PlayerState(2528B) (%1) | DI player = %1 | 4 |
| 3 | 4 | context.item_list | &Vec<Box<dyn ItemInfo>> (%2) | DI item_list = %2. exe 에선 (ptr,len) 두 인자 | 4 |
| 4 | 5 | build_slot | usize (%3) | DI build_slot/slot/n = %3 | 4 |
| 5 | 6 | inventory_index.tag | i64 range(0,2) (%4) | DI inventory_index[0..+8] | 4 |
| 6 | 6 | inventory_index.payload | i64 (%5) | DI inventory_index[8..+8] | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn item_v26(&mut self, rnd, player, context, build_slot, inventory_index) -> Option<usize>   // lib.rs:1149~1170 (self 미사용)
  item_list = &context.item_list                                   // +0x30 (IR 는 승격된 %2)
  // L1151 — 목표 아이템: 활성 빌드 항목 중 build_slot 번째
  target: usize = *player.info.item_builds.iter()
        .filter(|&&i| i < item_list.len() && item_list[i].is_active())   // i>=len 이면 패닉 없이 술어 거짓(get 의미) · is_active = vtable+0x50
        .nth(build_slot)?                                            // 블록 %22~%40 = advance_by(build_slot: 술어 참일 때만 accum-=1), %43~%59 = next(). 소진 → None
  target_item = &item_list[target]
  // L1153
  next_item: usize = if let Some(inv) = inventory_index {
      // L1154
      current = player.info.items.get(inv)?                          // inv >= items.len → None(패닉 아님 · %64→%167)
      // L1155
      random_item_next_toward_target(item_list, current.key(), target_item.key(), rnd)?   // key = vtable+0x58 → &String → &str. game_core 경계(item.rs:353)
  } else {
      // L1157
      path: Vec<usize> = random_item_build_path(item_list, target, rnd)?   // game_core 경계(item.rs:320) · None 니치 cap==-1
      // L1158
      first = *path.first()?                                         // len==0 → drop(path), None
      // L1159
      drop(path); first
  }
  // L1161
  inv_active: Vec<usize> = active_inventory_item_indices(player, item_list)   // lib.rs:542 인라인 + from_iter(aux m06) · 술어(aux m14 call_mut): items[i] 가 item_list 의 어떤 li 와 key 동일(len 동일 && memcmp==0) && li.is_active()
  if inv_active.len() > 2 && item_list[next_item].tier() == 0 { return None }   // tier = vtable+0x70 · 색인은 panic_bounds_check(item_list.len)
  drop(inv_active)
  // L1165
  if player.info.gold >= item_list[next_item].price() { Some(next_item) } else { None }   // price = vtable+0x68 · uge

★rnd 소비: 본문 직접 gen_range 0회. 호출당 정확히 하나 — inventory_index Some → random_item_next_toward_target 1회 / None → random_item_build_path 1회 (내부 gen_range 수는 game_core 경계 · 미명세).
★None 이 되는 경로 9개(38792 phi): 빌드 소진(%22/%43/%59) · inv bounds(%64) · next_toward None(%71) · build_path None(%116) · path 빈(%120) · 3개+ && tier0(%143) · gold 부족(%153 false).
```

**`mem` 메모리 접근 11건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x4e8 | info.item_builds.buf.ptr | r | Vec<usize> 데이터 포인터(L1151 · gep 1256). 원소 = item_list 인덱스 | 4 | OK |
| 1 | PlayerState | 0x4f0 | info.item_builds.len | r | L1151 (gep 1264) — 순회 종료(end = ptr + 8*len) | 4 | OK |
| 2 | PlayerState | 0x4a0 | info.items.buf.ptr | r | Vec<Box<dyn ItemInfo>> 인벤토리 데이터(L1154 gep 1184 · L1161 재로드 38708) | 4 | OK |
| 3 | PlayerState | 0x4a8 | info.items.len | r | L1154 (gep 1192) inventory_index bounds · L1161 재로드 | 4 | OK |
| 4 | PlayerState | 0x998 | info.gold | r | L1165 (gep 2456) — price 와 uge 비교 | 4 | OK |
| 5 | Vec<Box<dyn ItemInfo>> | 0x8 | item_list.ptr | r | context.item_list 의 데이터 포인터(gep %2,8) · 원소 16B 팻포인터 {data,vtable} | 4 | 확인불가(tcx 사전에 타입 없음) |
| 6 | Vec<Box<dyn ItemInfo>> | 0x10 | item_list.len | r | gep %2,16 — L1151 술어 bounds(get) · L1161/L1165 색인 bounds(panic) | 4 | 확인불가(tcx 사전에 타입 없음) |
| 7 | ItemInfo vtable | 0x50 | is_active | r | fn(&self)->bool (divtable ItemInfo 0x50). L1151 빌드 술어 + aux 인벤 술어 | 3 | 확인불가(tcx 사전에 타입 없음) |
| 8 | ItemInfo vtable | 0x58 | key | r | fn(&self)->&String (+0x8 ptr · +0x10 len 을 &str 로 씀). L1155 current/target · aux 술어 key 비교 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 9 | ItemInfo vtable | 0x68 | price | r | fn(&self)->usize. L1165 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 10 | ItemInfo vtable | 0x70 | tier | r | fn(&self)->usize. L1161 (==0 검사) | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 1151 | 태그 | nth 카운터 종료(accum==0) · Option 태그 None · L1158 path.len()==0 · L1161 tier()==0 판정 | 4 |
| 1 | 2 | 1161 | 임계 | active_inventory_item_indices(player,item_list).len() > 2 (samesign ugt 2) ⟹ 활성 인벤 아이템이 3개 이상이면 tier 0 아이템은 거부 | 4 |
| 2 | -1 | 1157 | 센티널 | Option<Vec<usize>> None 니치 — sret+0(cap)==usize::MAX 면 random_item_build_path 가 None | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | tier-0 거부 발동 인벤 개수 임계 | lib.rs:1161 · m14.ll:38647 | 2 | `len > 2`. 올리면(예 3) 활성 아이템 4개째부터 기본템(tier 0)을 거부 → 기본템을 더 많이 삼. 내리면(1) 기본템 2개째부터 거부. 값 자체가 '기본템 슬롯 상한 = 3' 을 결정 | 4 | 기존 |
| 1 | 구매/승급 골드 조건 | lib.rs:1165 · m14.ll:38782 | gold >= price (uge) | 비교를 완화(예 gold + 여유)하면 돈이 모자라도 후보를 돌려주지만 실제 구매는 game_core 가 다시 검증할 가능성 — 여기선 후보 산출 여부만 바뀜 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | active_inventory_item_indices | game_ai::active_inventory_item_indices | in:game_ai | fn(&game_core::PlayerState, &[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>]) -> std::vec::Vec<usize, std::alloc::Global> | game-ai\src\lib.rs:542 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | is_active | game_core::ItemInfo::is_active | pub | fn(&Self/#0) -> bool | game-core\src\setting\item.rs:433 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | is_active | <game_core::ModItemEntry as game_core::ItemInfo>::is_active | pub | fn(&game_core::ModItemEntry) -> bool | game-core\src\setting\item.rs:495 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | item_v26 | game_ai::AgentVerHamster::item_v26 | in:game_ai | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::GameContext, usize, std::option::Option<usize>) -> std::option::Option<usize> | game-ai\src\lib.rs:1149 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | next | game_view::StatMode::next | in:game_view::ui::match_result_ui | fn(game_view::StatMode) -> game_view::StatMode | game-view\src\ui\match_result_ui.rs:62 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 5 | next | game_view::FlowPeriod::next | in:game_view::ui::finance_ui | fn(&game_view::FlowPeriod) -> game_view::FlowPeriod | game-view\src\ui\finance_ui.rs:180 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 6 | next | game_view::ResultsPeriod::next | pub | fn(game_view::ResultsPeriod) -> game_view::ResultsPeriod | game-view\src\ui\training_ui.rs:44 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 7 | price | game_core::ItemInfo::price | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:436 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 8 | price | <game_core::ModItemEntry as game_core::ItemInfo>::price | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:498 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 9 | price | <game_core::BaseItemInfo as game_core::ItemInfo>::price | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:616 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 10 | random_item_build_path | game_core::random_item_build_path | pub | fn(&[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>], usize, &mut R/#0) -> std::option::Option<std::vec::Vec<usize, std::alloc::Global>> | game-core\src\setting\item.rs:320 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | random_item_next_toward_target | game_core::random_item_next_toward_target | pub | fn(&[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>], &str, &str, &mut R/#0) -> std::option::Option<usize> | game-core\src\setting\item.rs:353 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | tier | game_core::ItemInfo::tier | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:437 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 13 | tier | <game_core::ModItemEntry as game_core::ItemInfo>::tier | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:499 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 14 | tier | <game_core::BaseItemInfo as game_core::ItemInfo>::tier | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:617 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
</details>

⚠**미매칭 7개**: `advance_by`, `bounds`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `first`, `from_iter`, `memcmp`, `tier0`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m14.ll:35181, m14.ll:38241) · **형제 90개** (AgentVerHamster)

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

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | random_item_build_path(item.rs:320 · m08.ll:87652) / random_item_next_toward_target(item.rs:353 · m08.ll:88180) 내부(경로 산출·gen_range 수) — game_core 경계라 시그니처만: (&[Box<dyn ItemInfo>], usize target, &mut StdRng)->Option<Vec<usize>> / (&[Box<dyn ItemInfo>], current_key:&str, target_key:&str, &mut StdRng)->Option<usize> | 4 |  |
| 1 | 표기 불가 | L1151 술어가 소스에서 `item_list.get(i).is_some_and(is_active)` 인지 `i < len && item_list[i].is_active()` 인지 — 외연 동일(bounds 실패 시 패닉 없이 거짓) · 표기 불가 | 4 |  |
| 2 | 표기 불가 | L1161 조건 `len > 2` 가 소스에서 `>= 3` 인지 — 외연 동일 · 표기 불가 | 4 |  |
| 3 | 미탐색 | exe 7 인자 중 build_slot/tag/payload 의 스택 순서는 caller(0xe8fc80) 스토어로 확정했으나 레지스터 4개의 의미(rnd/player/ptr/len)는 caller 측 mov 패턴(rcx=rbx, rdx=r14, r8=[r15+8], r9=[r15+0x10]) 으로 추정 — rbx/r14/r15 의 기원은 디스어셈 22줄 밖. 검증 = ghidra-re 진입부 대조 | 5 |  |
| 4 | 미탐색 | item_v26_slot(lib.rs:1125 · m14.ll:35237) 은 이 함수의 콜리가 아니라 형제(호출자 buy/upgrade 가 부름) — 계약은 buy_item/upgrade_item 명세 참조 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

