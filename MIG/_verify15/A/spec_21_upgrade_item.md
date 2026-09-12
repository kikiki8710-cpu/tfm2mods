---

### `21` upgrade_item — 보유 아이템의 next_tier 후보 중 (활성 · 내 최대티어(4미만 기준)보다 높은 티어 · 가격≤골드) 인 것을 모아 무작위 1개를 (내 슬롯 i, item_list 인덱스) 로 돌려준다

| 항목 | 값 |
|---|---|
| id | `lib__upgrade_item` |
| 심볼 | `_RNvCshdEBA0ozCnw_7game_ai12upgrade_item` |
| 소스 | `game-ai\src\lib.rs:1603` |
| IR | `m14.ll` 6620~7067행 |
| 경로·가시성 | `<game_ai::AgentVerHamster as game_core::AiAgent>::upgrade_item` · **pub** |
| 계층 | 기타 |
| exe | `15182016` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<(usize,usize)>(24B) | +0x0 태그(0=None,1=Some) / +0x8 내 아이템 슬롯 인덱스 i / +0x10 context.item_list 인덱스 | 4 |
| 1 | 1 | version | usize | 본문에서 안 쓰임(dbg_value 없음, 분기 없음) | 4 |
| 2 | 2 | rnd | &mut StdRng | 후보가 2개 이상일 때 gen_range(0..len) 로 1개 선택 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | info.items(+0x4a0/0x4a8) 와 info.gold(+0x998) 만 읽음 | 4 |
| 4 | 4 | game(data ptr) | &dyn AbstractGame (data 절반) | readnone — 본문에서 전혀 안 씀 | 4 |
| 5 | 5 | game(vtable ptr) | &dyn AbstractGame (vtable 절반) | 본문에서 안 씀 | 4 |
| 6 | 6 | context | &GameContext(64B) | pool(+0x0, 후보 Vec 할당자) 와 item_list(+0x30) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn upgrade_item(_version, rnd, player, _game, context) -> Option<(usize, usize)>
// L1604~1605
let pool = context.pool; let item_list = context.item_list;   // +0x0 / +0x30
// L1610 (클로저 L1614): 내 아이템 중 tier<4 인 것의 최대 tier, 없으면 0
let my_max_tier = player.info.items.iter().filter(|it| it.tier() < 4).map(|it| it.tier()).max().unwrap_or(0);
   // IR: 1차 루프로 tier<4 인 첫 원소를 찾고(없으면 0), 2차 루프로 first 를 잡은 뒤 fold(max_by Ord::cmp)
// L1621
let mut candidate: bumpalo Vec<(usize,usize)> = Vec::new_in(pool);
// L1622
for (i, my_item) in player.info.items.iter().enumerate() {
  // L1623
  for nxt in my_item.next_tier().iter() {            // vtable+0x80 -> &Vec<String>
    // L1625
    if let Some(index) = item_index_by_key(item_list, nxt) {   // Option<usize> {tag, idx}
      let it = &item_list[index];                    // bounds check
      // L1626 (단락 평가 순서 = IR 순서)
      if it.is_active()                              // vtable+0x50
         && it.tier() > my_max_tier                  // vtable+0x70, ugt
         && !(it.price() > player.info.gold)         // vtable+0x68 vs +0x998  ⇔ price <= gold
      {
        candidate.push((i, index));                  // L1627
      }
    }
  }
}
// L1633
if candidate.is_empty() { return None; }             // L1637
// L1634
let k = rnd.gen_range(0..candidate.len());
Some(candidate[k])
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | GameContext | 0x0 | pool | r | &bumpalo::Bump — candidate Vec<(usize,usize)> 의 할당자 | 4 | OK |  |
| 1 | GameContext | 0x30 | item_list | r | &Vec<Box<dyn ItemInfo>> — 전체 아이템 사전 | 4 | OK |  |
| 2 | Vec<Box<dyn ItemInfo>>(item_list) | 0x8 | ptr | r | 원소 16B = Box<dyn ItemInfo> 팻포인터 (data +0, vtable +8) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 3 | Vec<Box<dyn ItemInfo>>(item_list) | 0x10 | len | r | item_index_by_key 의 슬라이스 길이·bounds check | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 4 | PlayerState | 0x4a0 | info.items.buf.ptr | r | 내 아이템 Vec<Box<dyn ItemInfo>> 버퍼(원소 16B) | 4 | OK |  |
| 5 | PlayerState | 0x4a8 | info.items.len | r | 내 아이템 수. 0 이면 my_max_tier=0, 후보 0 → None | 4 | OK |  |
| 6 | PlayerState | 0x998 | info.gold | r | usize. 후보 price 가 이보다 크면 제외 | 4 | OK |  |
| 7 | dyn ItemInfo vtable | 0x50 | is_active | r | divtable ItemInfo 0x50 (g02.ll vtable, 일치율 81%). false 면 후보 제외 | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 8 | dyn ItemInfo vtable | 0x68 | price | r | usize. > gold 면 제외 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 9 | dyn ItemInfo vtable | 0x70 | tier | r | usize. 내 아이템에는 <4 필터+max, 후보에는 > my_max_tier 조건 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 10 | dyn ItemInfo vtable | 0x80 | next_tier | r | -> &Vec<String> (다음 티어 아이템 키 목록) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 11 | Vec<String>(next_tier 반환) | 0x8 | ptr | r | 원소 24B String | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 12 | Vec<String>(next_tier 반환) | 0x10 | len | r |  | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 13 | String(next_tier 원소) | 0x8 | ptr | r | item_index_by_key 의 &str 인자 | 4 | OK |  |
| 14 | String(next_tier 원소) | 0x10 | len | r |  | 4 | OK |  |
| 15 | bumpalo Vec<(usize,usize)>(candidate, 32B 지역) | 0x0 | ptr | r | 초기 dangling(8). 원소 16B (i, index) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 16 | bumpalo Vec<(usize,usize)>(candidate) | 0x10 | cap | r | len==cap 이면 reserve_internal_or_panic | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 17 | bumpalo Vec<(usize,usize)>(candidate) | 0x18 | len | r | 0 이면 None. gen_range(0..len) 의 상한 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 18 | Option<(usize,usize)>(sret) | 0x0 | 태그 | w | L1637 None / L1634 Some | 4 | 확인불가(tcx 사전에 타입 없음) | 0 (None) / 1 (Some) |
| 19 | Option<(usize,usize)>(sret) | 0x8 | Some.0 = 내 슬롯 i | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | candidate[rnd].0 |
| 20 | Option<(usize,usize)>(sret) | 0x10 | Some.1 = item_list 인덱스 | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | candidate[rnd].1 |
| 21 | bumpalo Vec<(usize,usize)>(candidate, 지역) | 0x0 | ptr/bump/cap/len 초기화 및 push | w | 지역 변수라 외부 상태 변경 없음. 함수 끝에 Drop | 4 | 확인불가(tcx 사전에 타입 없음) | (i, index) 누적 |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 1610 | 임계 | my_max_tier 계산 시 tier() < 4 인 내 아이템만 센다(티어 4 이상 아이템은 최대티어 계산에서 제외). 두 루프(첫 매치 탐색·max fold) 모두 같은 4 | 4 |
| 1 | 0 | 1610 | 태그 | tier<4 인 아이템이 하나도 없으면 my_max_tier = 0 (phi 기본값 — unwrap_or(0) 또는 동등 표현) | 4 |
| 2 | 0 | 1637 | 태그 | Option 태그 0 = None (후보 비었을 때) | 4 |
| 3 | 1 | 1634 | 태그 | Option 태그 1 = Some | 4 |
| 4 | 0 | 1634 | 태그 | gen_range 하한 0 — 후보 인덱스 0..len | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 최대티어 산정에서 제외할 티어 하한 | lib.rs:1610 | 4 | tier<4 만 센다. 내리면(예: 3) 3티어 아이템도 my_max_tier 에서 빠져 더 낮은 티어로의 '업그레이드'도 후보가 된다. 올리면 고티어 보유 시 후보가 줄어든다(후보 조건이 tier > my_max_tier 이므로) | 4 | 기존 |
| 1 | 후보 조건 tier > my_max_tier | lib.rs:1626 | strict > | >= 로 바꾸면 같은 티어 간 교체도 업그레이드 후보가 된다 | 4 | 기존 |
| 2 | 가격 조건 price <= gold | lib.rs:1626 | price > gold 면 제외 | 골드 여유분(예: gold - 예비금)을 두면 업그레이드가 늦어진다 | 4 | 기존 |
| 3 | 후보 선택 방식 | lib.rs:1634 | 균등 무작위 | 가격/티어 가중 선택으로 바꾸면 rnd 소비가 달라져 시드 재현이 깨진다 | 4 | 기존 |

<details><summary>`callees` 피호출자 19건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | is_active | game_core::ItemInfo::is_active | pub | fn(&Self/#0) -> bool | game-core\src\setting\item.rs:433 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | is_active | <game_core::ModItemEntry as game_core::ItemInfo>::is_active | pub | fn(&game_core::ModItemEntry) -> bool | game-core\src\setting\item.rs:495 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | item_index_by_key | game_core::item_index_by_key | pub | fn(&[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>], &str) -> std::option::Option<usize> | game-core\src\setting\item.rs:367 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | next_tier | game_core::ItemInfo::next_tier | pub | fn(&Self/#0) -> &std::vec::Vec<std::string::String, std::alloc::Global> | game-core\src\setting\item.rs:439 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 7 | next_tier | <game_core::ModItemEntry as game_core::ItemInfo>::next_tier | pub | fn(&game_core::ModItemEntry) -> &std::vec::Vec<std::string::String, std::alloc::Global> | game-core\src\setting\item.rs:507 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 8 | next_tier | <game_core::BaseItemInfo as game_core::ItemInfo>::next_tier | pub | fn(&game_core::BaseItemInfo) -> &std::vec::Vec<std::string::String, std::alloc::Global> | game-core\src\setting\item.rs:625 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 9 | price | game_core::ItemInfo::price | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:436 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 10 | price | <game_core::ModItemEntry as game_core::ItemInfo>::price | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:498 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 11 | price | <game_core::BaseItemInfo as game_core::ItemInfo>::price | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:616 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 12 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 13 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 14 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 15 | tier | game_core::ItemInfo::tier | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:437 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 16 | tier | <game_core::ModItemEntry as game_core::ItemInfo>::tier | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:499 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 17 | tier | <game_core::BaseItemInfo as game_core::ItemInfo>::tier | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:617 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 18 | upgrade_item | game_ai::upgrade_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1603 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 5개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `enumerate`, `gen_range`, `llvm.memset.p0.i64`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 13곳** (m02.ll:49067, m02.ll:49456, m04.ll:19275, m04.ll:30148, m06.ll:37076, m09.ll:14065, m10.ll:8004, m10.ll:10062, m10.ll:10086, m14.ll:7690, m14.ll:35133, m14.ll:39409, m14.ll:40171) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | my_max_tier 의 소스 표기(`filter(..).map(..).max().unwrap_or(0)` vs `filter_map(..).max()` 등)는 외연이 같아 표기 불가 — IR 은 1차 탐색 루프 + max_by fold(심볼에 upgrade_item 클로저 s_0·s0_0) 로 나온다 | 4 |  |
| 1 | 미탐색 | L1626 세 조건의 소스 순서는 IR 평가 순서(is_active → tier → price)로 적었다. column 정보가 없어 한 줄 안 순서는 IR 단락 순서를 근거로 한다 | 4 |  |
| 2 | 미탐색 | version(%1)·game(%4,%5) 은 본문에서 사용 흔적 0 — 왜 시그니처에 있는지는 트레이트 AiAgent::upgrade_item 규격 때문으로 추정(lib.rs:440/1193 의 래퍼 미확인, 미탐색) | 5 |  |
| 3 | 미탐색 | item_index_by_key(game_core item.rs:367) 는 TLS 메모(ITEM_INDEX_MEMO)로 키→인덱스를 찾는다(g02.ll:157331 본문 앞부분만 확인). 키 불일치 시 None 인 것 외 세부는 미탐색 | 4 |  |
| 4 | 미탐색 | ItemInfo vtable 슬롯 이름은 divtable(g02.ll 정적 vtable, 일치율 81%) 기준. 런타임에 꽂히는 구현체별 tier/price 값은 이 함수 밖 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

