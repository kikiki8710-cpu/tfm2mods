---

### `71` should_recall_to_shop — 상점 귀환 판정 — 에픽/세르펜 임박·can_recall 게이트 후, 다음 살 아이템(v26 슬롯 빌드경로) 가격을 골드가 넘는지

| 항목 | 값 |
|---|---|
| id | `lib__should_recall_to_shop` |
| 심볼 | `_RNvCshdEBA0ozCnw_7game_ai21should_recall_to_shop` |
| 소스 | `game-ai\src\lib.rs:1642` |
| IR | `m14.ll` 7070~7700행 |
| 경로·가시성 | `game_ai::should_recall_to_shop` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e7acd0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문에서 안 읽음. buy_item/upgrade_item 에는 `poison` 으로 전달(그쪽도 version 미사용 추정) | 5 |
| 1 | 2 | rnd | &mut StdRng | can_recall·buy_item·upgrade_item·random_item_* 로 전달 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.items/item_builds/gold | 4 |
| 3 | 4 | data | &OperationData(24B) | cache.game(get_game_mode) · context(setting.tps, item_list) | 4 |
| 4 | 5 | goal_data | &GoalData | epic/serpen 의 epic_enemy_tick·epic_ally_tick 만 읽음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn should_recall_to_shop(version, rnd, player, data, goal_data) -> bool
let tps = ctx.setting.tick_per_second;                                                  // L1647
let epic_alive   = game.get_game_mode().as_moba().map_or(false, |m| m.jungle_runner.epic.live_list.len()   != 0);   // L1648 (vtable+0x40, MobaMode+0x1a8)
let serpen_alive = game.get_game_mode().as_moba().map_or(false, |m| m.jungle_runner.serpen.live_list.len() != 0);   // L1649 (MobaMode+0x1d8)
if epic_alive   && min(goal_data.epic.epic_ally_tick,   goal_data.epic.epic_enemy_tick)   <= tps*20 { return false; }   // L1650 (ugt 면 통과)
if serpen_alive && min(goal_data.serpen.epic_ally_tick, goal_data.serpen.epic_enemy_tick) <= tps*20 { return false; }   // L1651
if !utils::can_recall(rnd, player, data) { return false; }                                 // L1656

if player.info.item_builds.len() == 0 {                                                   // L1660
    if buy_item(_, rnd, player, _, _, ctx).is_some() { return true; }                       // L1661
    return upgrade_item(_, rnd, player, _, _, ctx).is_some();                               // L1662/1666
}
let item_list = &ctx.item_list;                                                            // L1664
let (build_slot, inventory_index) = item_v26_slot(player, item_list) else { return false };   // L1671~1672
// L1674
let target = player.info.item_builds.iter().copied()
    .filter(|&i| item_list.get(i).map_or(false, |it| it.is_active()))
    .nth(build_slot) else { return false };
// L1680~1687
let next_item = if let Some(inv_idx) = inventory_index {
    let inv_item = player.info.items.get(inv_idx) else { return false };                     // L1681
    random_item_next_toward_target(item_list, inv_item.key(), item_list[target].key(), rnd) else { return false }   // L1682
} else {
    let path = random_item_build_path(item_list, target, rnd) else { return false };         // L1687
    if path.is_empty() { return false; }
    path[0]
};
// L1692
let active: Vec<usize> = active_inventory_item_indices(player.info.items, item_list);   // from_iter 인라인
if active.len() > 2 && item_list[next_item].tier() == 0 { return false; }              // [next_item] bounds 패닉 가능
// L1695
return player.info.gold >= item_list[next_item].price();
```

**`mem` 메모리 접근 24건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | L1648 | 4 | OK |
| 1 | OperationData | 0x8 | context | r | L1647/1664 | 4 | OK |
| 2 | AbstractGameWithCache | 0x0 | game.data_ptr (dyn data ptr) | r | L1648~1649 get_game_mode self | 4 | OK |
| 3 | AbstractGameWithCache | 0x8 | game.vtable_ptr vtable | r | vtable+0x40 = get_game_mode (divtable AbstractGame 0x40) · 2회 호출 | 3 | OK |
| 4 | GameContext | 0x8 | setting | r | L1647 | 4 | OK |
| 5 | GameContext | 0x30 | item_list (&Vec<Box<dyn ItemInfo>>) | r | L1664 · +0x8 ptr / +0x10 len | 4 | OK |
| 6 | GameSetting | 0x12f8 | tick_per_second | r | L1647 tps · tps*20 = 20초 | 4 | OK |
| 7 | GameMode | 0x0 | tag (0=Moba) | r | L1648~1649 · 0 이 아니면 None 취급(as_moba) | 4 | OK |
| 8 | GameMode | 0x8 | Moba.0 (&MobaMode) | r | null 이면 None | 4 | OK |
| 9 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | L1648 epic_alive = len != 0 | 4 | OK |
| 10 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | L1649 serpen_alive = len != 0 | 4 | OK |
| 11 | GoalData | 0x88 | epic.epic_enemy_tick | r | L1650 | 4 | OK |
| 12 | GoalData | 0x98 | epic.epic_ally_tick | r | L1650 · min(둘) <= tps*20 이면 귀환 금지 | 4 | OK |
| 13 | GoalData | 0xc0 | serpen.epic_enemy_tick | r | L1651 | 4 | OK |
| 14 | GoalData | 0xd0 | serpen.epic_ally_tick | r | L1651 | 4 | OK |
| 15 | PlayerState | 0x4e8 | info.item_builds.ptr | r | L1674 · 빌드 아이템 인덱스 목록(usize) | 4 | OK |
| 16 | PlayerState | 0x4f0 | info.item_builds.len | r | L1660 · 0 이면 buy_item/upgrade_item 경로 | 4 | OK |
| 17 | PlayerState | 0x4a0 | info.items.buf.ptr | r | L1681/1692 · 인벤토리(원소 16B: Box<dyn ItemInfo> fat ptr) | 4 | OK |
| 18 | PlayerState | 0x4a8 | info.items.len | r | L1681 inventory_index 범위 검사(get) / L1692 | 4 | OK |
| 19 | PlayerState | 0x998 | info.gold | r | L1695 gold >= price | 4 | OK |
| 20 | ItemInfo(vtable) | 0x50 | is_active() | r | L1674 item_builds 필터 (divtable ItemInfo 0x50, 일치율 81%) | 3 | 확인불가(★모호: 동명 def_path 40개 [('game_core::Arcan) |
| 21 | ItemInfo(vtable) | 0x58 | key() -> &String | r | L1682 random_item_next_toward_target 인자(ptr +8/len +16) | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_core::Arcan) |
| 22 | ItemInfo(vtable) | 0x68 | price() -> usize | r | L1695 | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_core::Arcan) |
| 23 | ItemInfo(vtable) | 0x70 | tier() -> usize | r | L1692 · 활성템 3개 이상일 때 tier()==0 이면 false | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_core::Arcan) |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 20 | 1650 | 계수 | tps*20 = 20초. 에픽/세르펜 생존 중이고 아군·적군 처치 예상틱 min 이 20초 이내면 귀환 안 함(L1650/1651) | 4 |
| 1 | 0 | 1648 | 태그 | GameMode 태그 0 = Moba (tcxdict --enum GameMode). L1660 item_builds.len==0 분기. L1692 tier()==0 | 3 |
| 2 | 1 | 1661 | 태그 | buy_item 반환 Option<usize> Some 태그(=1) → true. L1680 inventory_index Some 태그 | 4 |
| 3 | -1 | 1671 | 센티널 | item_v26_slot 반환 Option<(usize,Option<usize>)> 외곽 None 니치 태그(+0x8 == -1) → false | 4 |
| 4 | 2 | 1692 | 임계 | active_inventory_item_indices(...).len() > 2 (활성 아이템 3개 이상) 이면 next_item.tier()==0 검사 | 4 |
| 5 | 1152921504606846976 | 1692 | 임계 | 2^60 — Vec len 상한 llvm.assume(컴파일러 아티팩트, 판정 아님) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 오브젝트 임박 귀환 금지 창(초) | lib.rs:1650, 1651 | 20 | 올리면 에픽/세르펜 처치 예상이 더 멀어도 귀환을 막음(귀환 빈도 감소). 0 이면 게이트 무력화 | 4 | 기존 |
| 1 | 활성 아이템 보유 상한 | lib.rs:1692 | 2 | `> 2` 비교값. 올리면 활성템을 더 많이 들고도 tier0 아이템 사러 귀환 허용 | 4 | 기존 |

<details><summary>`callees` 피호출자 23건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | active_inventory_item_indices | game_ai::active_inventory_item_indices | in:game_ai | fn(&game_core::PlayerState, &[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>]) -> std::vec::Vec<usize, std::alloc::Global> | game-ai\src\lib.rs:542 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | buy_item | game_ai::buy_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> | game-ai\src\lib.rs:1477 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | can_recall | game_ai::can_recall | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\utils.rs:350 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | is_active | game_core::ItemInfo::is_active | pub | fn(&Self/#0) -> bool | game-core\src\setting\item.rs:433 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | is_active | <game_core::ModItemEntry as game_core::ItemInfo>::is_active | pub | fn(&game_core::ModItemEntry) -> bool | game-core\src\setting\item.rs:495 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 10 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | item_v26_slot | game_ai::AgentVerHamster::item_v26_slot | in:game_ai | fn(&game_core::PlayerState, &[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>]) -> std::option::Option<(usize, std::option::Option<usize>)> | game-ai\src\lib.rs:1125 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | price | game_core::ItemInfo::price | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:436 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 14 | price | <game_core::ModItemEntry as game_core::ItemInfo>::price | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:498 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 15 | price | <game_core::BaseItemInfo as game_core::ItemInfo>::price | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:616 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 16 | random_item_build_path | game_core::random_item_build_path | pub | fn(&[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>], usize, &mut R/#0) -> std::option::Option<std::vec::Vec<usize, std::alloc::Global>> | game-core\src\setting\item.rs:320 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | random_item_next_toward_target | game_core::random_item_next_toward_target | pub | fn(&[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>], &str, &str, &mut R/#0) -> std::option::Option<usize> | game-core\src\setting\item.rs:353 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | should_recall_to_shop | game_ai::should_recall_to_shop | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool | game-ai\src\lib.rs:1642 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | tier | game_core::ItemInfo::tier | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:437 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 20 | tier | <game_core::ModItemEntry as game_core::ItemInfo>::tier | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:499 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 21 | tier | <game_core::BaseItemInfo as game_core::ItemInfo>::tier | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:617 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 22 | upgrade_item | game_ai::upgrade_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1603 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 3개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `from_iter`, `map_or`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:30614) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | utils::can_recall(utils.rs:350) 본문은 범위 밖(별도 define 0xd36480) — 미독해 | 4 |  |
| 1 | 미탐색 | item_v26_slot(lib.rs:1125)·buy_item(1477)·upgrade_item(1603)·random_item_build_path·random_item_next_toward_target 내부 미독해 | 4 |  |
| 2 | 미탐색 | L1671 외곽 None 니치가 -1 로 보이는 이유(보통 2) — tcxdict --enum 으로 tuple Option 레이아웃 미확인. 로직엔 영향 없음(None→false) | 3 |  |
| 3 | 미탐색 | L1692 `tier()==0` 의 게임적 의미(tier 0 = 기본 컴포넌트?) 미확인 — ItemInfo::tier 는 item.rs:437 `-> usize` 만 확인 | 4 |  |
| 4 | 미탐색 | ItemInfo vtable 슬롯 이름은 divtable 일치율 81% 기준 — is_active(0x50)/key(0x58)/price(0x68)/tier(0x70). key 는 active_inventory_item_indices 클로저(m14:58085)에서 문자열 비교에 쓰여 &String 반환 확인, price 는 gold 비교로 정합 | 3 |  |
| 5 | 미탐색 | L1674 필터가 is_active() 인 것이 소스 의도인지(빌드 목록 중 활성템만 셈) — IR 상 vtable+0x50 호출 사실만 기록 | 4 |  |
| 6 | 미탐색 | version(p1) 은 본문에서 읽지 않음 — 하위 호출에도 poison 전달 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

