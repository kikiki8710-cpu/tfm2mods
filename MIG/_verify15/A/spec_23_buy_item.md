---

### `23` buy_item — 저티어(<4) 보유템이 없고 보유 ≤2개일 때, 활성·구매가능·tier0 아이템 중 챔피언 카테고리(근접/원거리/마법/유틸/암살)와 기본 HP 에 따라 허용 카테고리를 골라 후보를 만들고 무작위 1개 item_list 인덱스를 돌려준다

| 항목 | 값 |
|---|---|
| id | `lib__buy_item` |
| 심볼 | `_RNvCshdEBA0ozCnw_7game_ai8buy_item` |
| 소스 | `game-ai\src\lib.rs:1477` |
| IR | `m14.ll` 8267~8721행 |
| 경로·가시성 | `<game_ai::AgentVerHamster as game_core::AiAgent>::buy_item` · **pub** |
| 계층 | 기타 |
| exe | `15185472` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize>
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | version | usize | 본문에서 안 쓰임(dbg_value 없음, 분기 없음) | 4 |
| 1 | 1 | rnd | &mut StdRng | gen_range(0..candidate.len()) 로 최종 1개 선택 | 4 |
| 2 | 2 | player | &PlayerState(2528B) | info.items(+0x4a0/+0x4a8) · info.champion(+0x510 Arc<dyn ChampionInfo>) · info.gold(+0x998) | 4 |
| 3 | 3 | game(data ptr) | &dyn AbstractGame data 절반 | readnone — 안 씀 | 4 |
| 4 | 4 | game(vtable ptr) | &dyn AbstractGame vtable 절반 | 안 씀 | 4 |
| 5 | 5 | context | &GameContext(64B) | pool(+0x0) · item_list(+0x30) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn buy_item(_version, rnd, player, _game, context) -> Option<usize>
// L1480
let has_low_tier_item = player.info.items.iter().any(|x| x.tier() < 4);   // vtable+0x70
if has_low_tier_item { return None; }
// L1487~1491
let pool = context.pool; let item_list = context.item_list;
let mut candidate: bumpalo Vec<usize> = Vec::new_in(pool);
// L1493
let tag = player.info.champion.category();                      // ChampionCategory (vtable+0x20)
// L1495
for (i, item) in item_list.iter().enumerate() {
  if player.info.items.len() > 2 { continue; }                   // L1496 (루프 불변)
  if !item.is_active() { continue; }                             // L1500 vtable+0x50
  if item.price() > player.info.gold { continue; }               // L1504 vtable+0x68 vs +0x998
  if item.tier() != 0 { continue; }                              // L1508
  let category = item.category();                                // L1512 vtable+0xa0 (ItemCategory)
  let is_defensive = matches!(category, Defense(2) | MagicResistance(3) | Hp(5));   // L1514
  let is_magic = category == Magic(4);                           // L1515
  let n = player.info.items.len();
  let ok = match tag {                                           // L1518
    Melee(0) => {                                                // L1523
      let hp = player.info.champion.stat().hp;                   // vtable+0x30, EntityStat+0x10
      if hp < 1550 { match n { 0 | 2 => category < 2, _ => is_defensive } }        // L1524~1531
      else if player.info.champion.stat().hp < 1800 {                              // L1536 (stat() 재호출)
        if n == 2 { category < 2 } else { is_defensive } }                          // L1537~1544
      else { is_defensive }                                                         // L1551
    }
    Assassin(4) => match n { 0 | 2 => category < 2, _ => is_defensive },           // L1557~1564
    Range(1) => category < 2,                                                       // L1570
    Magician(2) => is_magic,                                                        // L1575
    Util(3) => { let hp = player.info.champion.stat().hp;                           // L1582
                 if hp < 1550 { is_magic } else { is_defensive } }                  // L1583/1586
    _ => unreachable,
  };
  if ok { candidate.push(i); }                                   // L1592
}
// L1595
if candidate.is_empty() { return None; }
// L1596
Some(candidate[rnd.gen_range(0..candidate.len())])
```

**`mem` 메모리 접근 20건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x4a0 | info.items.buf.ptr | r | Box<dyn ItemInfo> 16B 원소. has_low_tier_item 스캔 | 4 | OK |  |
| 1 | PlayerState | 0x4a8 | info.items.len | r | ①>2 면 후보 0 ②==2 / {0,2} vs 1 로 카테고리 규칙 분기 | 4 | OK |  |
| 2 | PlayerState | 0x510 | info.champion (Arc<dyn ChampionInfo> data ptr) | r | ArcInner 헤더(16B, vtable align 정렬) 건너뛰어 내부 참조를 만든다 | 4 | OK |  |
| 3 | PlayerState | 0x518 | info.champion vtable ptr | r | 슬롯 +0x10 align / +0x20 category() / +0x30 stat() (divtable ChampionInfo 93%) | 3 | OK |  |
| 4 | PlayerState | 0x998 | info.gold | r | price > gold 면 제외 | 4 | OK |  |
| 5 | GameContext | 0x0 | pool | r | candidate Vec<usize>(bumpalo) 할당자 | 4 | OK |  |
| 6 | GameContext | 0x30 | item_list | r | &Vec<Box<dyn ItemInfo>> 전체 아이템 사전 — 후보 인덱스의 기준 | 4 | OK |  |
| 7 | Vec<Box<dyn ItemInfo>>(item_list) | 0x8 | ptr | r |  | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 8 | Vec<Box<dyn ItemInfo>>(item_list) | 0x10 | len | r |  | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 9 | dyn ItemInfo vtable | 0x50 | is_active | r | false → 제외 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 10 | dyn ItemInfo vtable | 0x68 | price | r | > gold → 제외 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 11 | dyn ItemInfo vtable | 0x70 | tier | r | 보유템 스캔(<4) 및 후보 조건(==0) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 12 | dyn ItemInfo vtable | 0xa0 | category | r | -> ItemCategory(i32): 0 AD/1 AttackSpeed/2 Defense/3 MagicResistance/4 Magic/5 Hp/6 Support | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 13 | dyn ChampionInfo vtable | 0x20 | category | r | -> ChampionCategory(i32): 0 Melee/1 Range/2 Magician/3 Util/4 Assassin. 5 이상은 unreachable | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 14 | dyn ChampionInfo vtable | 0x30 | stat | r | -> EntityStat(72B, sret). Melee/Util 규칙에서만 호출(최대 2회) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 15 | EntityStat(stat() 반환) | 0x10 | hp | r | 기본 HP. 1550 / 1800 임계와 비교 | 4 | OK |  |
| 16 | bumpalo Vec<usize>(candidate, 32B 지역) | 0x10 | cap | r | len==cap → reserve_internal_or_panic | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 17 | bumpalo Vec<usize>(candidate) | 0x18 | len | r | 0 → None. gen_range 상한 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 18 | bumpalo Vec<usize>(candidate) | 0x0 | ptr | r | candidate[k] 로드 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 19 | bumpalo Vec<usize>(candidate, 지역) | 0x0 | ptr/bump/cap/len 초기화 · push(i) | w | 지역 변수, 외부 상태 변경 없음. 종료 시 Drop | 4 | 확인불가(tcx 사전에 타입 없음) | item_list 인덱스 i 누적 |

**`consts` 상수 20건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 1480 | 임계 | has_low_tier_item = 보유템 중 tier() < 4 가 하나라도 있으면 → 즉시 None(구매 대신 업그레이드 대상이 있다는 뜻) | 4 |
| 1 | 2 | 1496 | 임계 | 보유템 수 > 2 이면 모든 아이템 skip(=후보 0 → None). 즉 보유 0~2개일 때만 신규 구매 | 4 |
| 2 | 0 | 1508 | 태그 | tier() == 0 인 아이템만 구매 후보(기초 아이템) | 4 |
| 3 | 2 | 1514 | 임계 | ItemCategory 2 = Defense → is_defensive | 4 |
| 4 | 3 | 1514 | 태그 | ItemCategory 3 = MagicResistance → is_defensive | 4 |
| 5 | 5 | 1514 | 태그 | ItemCategory 5 = Hp → is_defensive | 4 |
| 6 | 4 | 1515 | 임계 | ItemCategory 4 = Magic → is_magic | 4 |
| 7 | 2 | 1526 | 임계 | category < 2 ⇔ AD(0) 또는 AttackSpeed(1) = 공격형 아이템. L1539/L1559/L1570 도 같은 비교 | 4 |
| 8 | 0 | 1518 | 태그 | ChampionCategory 0 = Melee 분기 | 4 |
| 9 | 1 | 1518 | 태그 | ChampionCategory 1 = Range 분기 | 4 |
| 10 | 2 | 1518 | 임계 | ChampionCategory 2 = Magician 분기 | 4 |
| 11 | 3 | 1518 | 태그 | ChampionCategory 3 = Util 분기 | 4 |
| 12 | 4 | 1518 | 임계 | ChampionCategory 4 = Assassin 분기 | 4 |
| 13 | 1550 | 1523 | 임계 | Melee: 기본 HP(stat().hp) < 1550 이면 '저HP 근접' 규칙. Util(L1582)도 같은 1550 임계 | 4 |
| 14 | 1800 | 1536 | 임계 | Melee: HP ≥1550 이고 < 1800 이면 '중간HP 근접' 규칙, ≥1800 이면 방어템만 | 4 |
| 15 | 2 | 1537 | 임계 | Melee 중간HP: 보유템이 정확히 2개면 공격형, 아니면 방어형 | 4 |
| 16 | 0 | 1524 | 태그 | 보유템 수 match {0 \| 2 => 공격형, _ => 방어형} (Melee 저HP L1524, Assassin L1557 동일 패턴) | 4 |
| 17 | 0 | 1600 | 태그 | Option 태그 0 = None | 4 |
| 18 | 1 | 1600 | 태그 | Option 태그 1 = Some | 4 |
| 19 | 0 | 1596 | 태그 | gen_range 하한 0 | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 구매 대신 업그레이드로 넘기는 보유템 티어 상한 | lib.rs:1480 | 4 | tier<4 보유템이 하나라도 있으면 신규 구매 안 함. 값을 내리면(예: 3) 3티어 보유 중에도 기초템을 더 산다 | 4 | 기존 |
| 1 | 신규 구매 허용 보유템 수 상한 | lib.rs:1496 | 2 | 보유 3개 이상이면 구매 없음. 슬롯 확장 모드(4칸)와 맞물려 3 으로 올리면 4번째 기초템을 산다 | 4 | 기존 |
| 2 | 근접 챔피언 저HP/중HP 경계 | lib.rs:1523 / 1536 | 1550 / 1800 | 기본 HP 가 낮은 근접은 공격템 우선(0·2개째), 높으면 방어템만. 올리면 더 많은 근접이 공격템을 산다 | 4 | 기존 |
| 3 | 유틸 챔피언 HP 경계 | lib.rs:1582 | 1550 | HP<1550 유틸은 마법템, 아니면 방어템 | 4 | 기존 |
| 4 | '공격형' 카테고리 판정 | lib.rs:1526 등 | category < 2 (AD·AttackSpeed) | Support(6)·Magic(4) 은 근접/원거리/암살 후보에 절대 안 들어감 | 4 | 기존 |
| 5 | 후보 선택 | lib.rs:1596 | 균등 무작위 | 가중치 도입 시 rnd 소비 변화로 시드 재현 깨짐 | 4 | 기존 |

<details><summary>`callees` 피호출자 23건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | buy_item | game_ai::buy_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> | game-ai\src\lib.rs:1477 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 1 | buy_item | game_ai::AgentVerHamster::buy_item | pub | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> | game-ai\src\lib.rs:1173 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 2 | buy_item | <game_ai::AgentVerHamster as game_core::AiAgent>::buy_item | pub | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> | game-ai\src\lib.rs:437 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 3 | category | game_core::News::category | pub | fn(&game_core::News) -> game_core::NewsCategory | game-core\src\data\news.rs:1066 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 119개 중 상위 3개 |
| 4 | category | game_core::ItemInfo::category | pub | fn(&Self/#0) -> game_core::ItemCategory | game-core\src\setting\item.rs:443 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 119개 중 상위 3개 |
| 5 | category | game_core::ChampionInfo::category | pub | fn(&Self/#0) -> game_core::ChampionCategory | game-core\src\setting.rs:1080 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 119개 중 상위 3개 |
| 6 | is_active | game_core::ItemInfo::is_active | pub | fn(&Self/#0) -> bool | game-core\src\setting\item.rs:433 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | is_active | <game_core::ModItemEntry as game_core::ItemInfo>::is_active | pub | fn(&game_core::ModItemEntry) -> bool | game-core\src\setting\item.rs:495 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 9 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 10 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | price | game_core::ItemInfo::price | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:436 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 12 | price | <game_core::ModItemEntry as game_core::ItemInfo>::price | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:498 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 13 | price | <game_core::BaseItemInfo as game_core::ItemInfo>::price | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:616 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 14 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 15 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 16 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 17 | stat | game_core::ItemInfo::stat | pub | fn(&Self/#0) -> game_core::BuffState | game-core\src\setting\item.rs:438 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 122개 중 상위 3개 |
| 18 | stat | game_core::EntityInfo::stat | pub | fn(&Self/#0) -> game_core::EntityStat | game-core\src\setting.rs:1152 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 122개 중 상위 3개 |
| 19 | stat | game_core::ChampionInfo::stat | pub | fn(&Self/#0) -> game_core::EntityStat | game-core\src\setting.rs:1082 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 122개 중 상위 3개 |
| 20 | tier | game_core::ItemInfo::tier | pub | fn(&Self/#0) -> usize | game-core\src\setting\item.rs:437 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 21 | tier | <game_core::ModItemEntry as game_core::ItemInfo>::tier | pub | fn(&game_core::ModItemEntry) -> usize | game-core\src\setting\item.rs:499 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
| 22 | tier | <game_core::BaseItemInfo as game_core::ItemInfo>::tier | pub | fn(&game_core::BaseItemInfo) -> usize | game-core\src\setting\item.rs:617 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 54개 중 상위 3개 |
</details>

⚠**미매칭 6개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `enumerate`, `gen_range`, `llvm.assume`, `llvm.memset.p0.i64`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 8곳** (m04.ll:19270, m04.ll:30144, m06.ll:37071, m09.ll:14053, m14.ll:7197, m14.ll:38195, m14.ll:39379, m14.ll:40141) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L1480 의 소스 표기(`any(\|x\| x.tier() < 4)` vs `iter().find(..).is_some()` 등)는 표기 불가(외연 동일). IR 은 첫 매치에서 즉시 None 반환 | 4 |  |
| 1 | 표기 불가 | L1536 에서 stat() 을 다시 호출하는 것(sret 72B 두 번째 alloca)은 IR 사실 — 소스가 `player.info.champion.stat().hp` 를 두 번 쓴 것으로 보이나 지역변수 재사용 여부는 표기 불가. stat() 이 결정적이면 결과 동일 | 4 |  |
| 2 | 미탐색 | version(%0)·game(%3,%4) 미사용 — AiAgent 트레이트 규격 추정(lib.rs:437/1173 래퍼 미확인, 미탐색) | 5 |  |
| 3 | 미탐색 | ChampionCategory 5 이상은 `unreachable` — 열거형이 5 variant 라 발생 불가 | 4 |  |
| 4 | 미탐색 | ItemCategory Support(6) 는 어떤 챔피언 카테고리에서도 후보가 되지 않는다(is_defensive·is_magic·<2 어디에도 없음) — 의도인지는 소스 주석 부재로 미확인 | 4 |  |
| 5 | 미탐색 | candidate.push 앞 `icmp ult len, 2^59` assume 는 slice 길이 상한 가정(컴파일러 삽입)이라 상수 목록에서 제외 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

