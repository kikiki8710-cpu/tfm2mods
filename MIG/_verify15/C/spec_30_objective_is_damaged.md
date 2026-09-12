---

### `30` objective_is_damaged — 에픽(Morgard)/서펜 오브젝트가 살아 있고 현재 hp 가 최대치보다 낮은지(=이미 맞았는지) 판정

| 항목 | 값 |
|---|---|
| id | `objective_helpers__objective_is_damaged` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers20objective_is_damaged` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:41` |
| IR | `m15.ll` 51666~51800행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::objective_helpers::objective_is_damaged` · **in:game_ai** |
| 계층 | 기타 |
| exe | `15502064` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::OperationData, game_core::JungleType) -> bool
```

<details><summary>인자 2개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | data | &OperationData(24B) | data+0x0 = cache(&AbstractGameWithCache) 만 읽고, cache+0x0/+0x8 = &dyn AbstractGame (data, vtable) 로 게임 모드·엔티티 조회 | 4 |
| 1 | 2 | target | JungleType(1B, range 0..6) | tcxdict --enum JungleType: 0 Rhino/1 Mushroom/2 Stump/3 Bee/4 Morgard/5 Serpen. 4·5 만 처리, 나머지는 false (m15.ll:51673 switch) | 3 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn objective_is_damaged(data: &OperationData, target: JungleType) -> bool

[L42] match target {
  JungleType::Morgard(=4) => {
[L43]   let moba: &MobaMode = data.cache.game.get_game_mode()   // vtable+0x40, 반환 GameMode{tag,ptr}
                              .as_moba()                       // game.rs:231 인라인: tag==0(Moba) 이면 Some(ptr)
                              .unwrap();                       // ⚠Moba 아니면 unwrap_failed 패닉 (m15.ll:51742)
        let list: &[usize] = &moba.jungle_runner.epic.live_list;   // MobaMode+0x1a0 ptr / +0x1a8 len
[L44]   list.get(0)                                             // len==0 → None → false
            .and_then(|id| data.cache.game.get_entity_by_id(*id))   // vtable+0x1f0, closure$0. null → false
[L45]       .is_some_and(|e| e.hp(+0x670) < e.stat_cached.hp(+0x628))   // closure$1: icmp ult (m15.ll:51739)
  }
  JungleType::Serpen(=5) => {
[L46]   let moba = ...get_game_mode().as_moba().unwrap();        // 동일, 패닉 경로 m15.ll:51799
        let list: &[usize] = &moba.jungle_runner.serpen.live_list; // MobaMode+0x1d0 ptr / +0x1d8 len
[L47]   list.get(0).and_then(|id| game.get_entity_by_id(*id))     // closure$2
[L48]       .is_some_and(|e| e.hp < e.stat_cached.hp)             // closure$3 — 두 경로가 같은 블록 %37 로 합류(phi)
  }
[L49~50] _ => false        // Rhino/Mushroom/Stump/Bee 는 switch default → %44 phi false
}
[L51] ret

※ '피해를 입었다' = hp 가 stat_cached.hp(최대 hp) 보다 엄격히 작다. 같으면(만피) false.
※ live_list 는 usize(엔티티 id) 벡터이고 [0] 하나만 본다(다수 스폰은 고려 안 함).
※ 부작용 없음(읽기 전용). GameMode 가 Moba 가 아닐 때만 패닉.
```

**`mem` 메모리 접근 11건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m15.ll:51677 `load ptr, ptr %0`) | 4 | OK |
| 1 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data ptr) | r | vtable 호출의 self 인자 (m15.ll:51678) | 4 | OK |
| 2 | AbstractGameWithCache | 0x8 | game (dyn AbstractGame vtable ptr) | r | vtable+0x40 = get_game_mode, vtable+0x1f0 = get_entity_by_id (divtable AbstractGame) | 3 | OK |
| 3 | GameMode(16B enum, 반환값) | 0x0 | 판별자 | r | tcxdict --enum GameMode: 0 Moba/1 SingleLane/2 DeathMatch. ==0 이어야 진행, 아니면 unwrap_failed 패닉 (as_moba().unwrap(), game.rs:231 인라인) | 3 | OK |
| 4 | GameMode(16B enum, 반환값) | 0x8 | Moba 페이로드 &MobaMode | r | extractvalue 1 | 4 | OK |
| 5 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.buf.inner.ptr | r | Vec<usize> 버퍼 시작. [0] 을 엔티티 id 로 읽는다 (target==Morgard 경로, m15.ll:51706) | 4 | OK |
| 6 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 이면 get(0)=None → false (m15.ll:51699~51703) | 4 | OK |
| 7 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.buf.inner.ptr | r | target==Serpen 경로 (m15.ll:51775) | 4 | OK |
| 8 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | 0 이면 false (m15.ll:51768~51772) | 4 | OK |
| 9 | Entity | 0x670 | hp | r | 현재 체력 (m15.ll:51735, gep 1648) | 4 | OK |
| 10 | Entity | 0x628 | stat_cached.hp | r | 최대 체력(캐시된 스탯) (m15.ll:51737, gep 1576) | 4 | OK |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 42 | 태그 | JungleType 메모리태그 4 = Morgard → epic.live_list 경로 (switch case, m15.ll:51674) | 4 |
| 1 | 5 | 42 | 태그 | JungleType 메모리태그 5 = Serpen → serpen.live_list 경로 (switch case, m15.ll:51675) | 4 |
| 2 | 0 | 43 | 태그 | GameMode 판별자 0 = Moba. get_game_mode() 결과가 Moba 가 아니면 as_moba().unwrap() 패닉(unwrap_failed). 또 live_list.len == 0 → None → false, 그리고 live_list.get(0) 의 인덱스 0 | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 피해 판정 비교식 | objective_helpers.rs:45 / :48 (IR m15.ll:51739 `icmp ult %40, %42`) | hp < stat_cached.hp | 임계 상수가 없는 순수 비교. 예컨대 `hp * 100 < max * N` 으로 바꾸면 'N% 이하로 깎였을 때만 피해로 인정' 하게 돼, 호출부(오브젝트 개입/합류 판단)가 더 늦게 반응한다 | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | objective_is_damaged | game_ai::plan_legacy::team_plan::objective_helpers::objective_is_damaged | in:game_ai | fn(&game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:41 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 1개**: `usize`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m09.ll:29752, m09.ll:29881) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 호출부에서 이 bool 이 어떻게 소비되는지(팀 플랜의 어떤 게이트인지)는 담당 범위 밖이라 안 봄 — objective_helpers.rs 다른 함수·team_plan 쪽 xref 필요 | 4 |  |
| 1 | 미탐색 | list.get(0) 인지 first() 인지: dbg 사슬은 `get<usize,usize>`(slice::get) 라 get(0) 로 확정, 그러나 인덱스 리터럴 0 은 상수접힘(ptr+0)이라 본문에 없음 | 4 |  |
| 2 | 미탐색 | live_list 원소가 엔티티 id(usize) 라는 것은 get_entity_by_id(i64) 에 그대로 넘기는 IR 사용 패턴으로 확정, JungleRunner.epic 의 구조체 타입명(EpicRunner 등)은 tcxdict 로 별도 조회 안 함 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

