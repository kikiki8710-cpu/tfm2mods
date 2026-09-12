---

### `32` serpen_giveup_chat_reason — 서펜 포기 채팅 사유: 서펜이 살아 있으면 아군/적 서펜 스택 수를 비교해 StackAhead/Outnumbered 를 고른다

| 항목 | 값 |
|---|---|
| id | `serpen__serpen_giveup_chat_reason` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen25serpen_giveup_chat_reason` |
| 소스 | `game-ai\src\plan_legacy\old\serpen.rs:217` |
| IR | `m05.ll` 52986~53100행 |
| 경로·가시성 | `game_ai::plan_legacy::old::serpen_giveup_chat_reason` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `14050784` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::SerpenGiveUpReason>
```

<details><summary>인자 2개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽는다 | 4 |
| 1 | 2 | data | &OperationData(24B) | +0x0 cache(→ dyn AbstractGame get_game_mode), +0x8 context(→ tutorial) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn serpen_giveup_chat_reason(player: &PlayerState, data: &OperationData) -> Option<SerpenGiveUpReason>

[L218] if !rule_scope::serpen_exists(data.context) { return None }
        // 인라인: context.tutorial(+0x38).spawn_serpen()  (runner.rs:267)
        //   = matches!(tutorial, None(0) | MidBottom(5) | Line(7) | Total(8))

[L222] let moba = data.cache.game.get_game_mode().as_moba().unwrap();   // ⚠Moba 아니면 패닉
        if moba.jungle_runner.serpen.live_list(+0x1d8 len).is_empty() { return None }   // 서펜 미생존 → None

[L225] let team = player.info.team (+0x930)                              // >=2 → panic_bounds_check
        let my_serpen_count    = get_game_mode().as_moba().unwrap().serpen_count(+0x260)[team]      // game.rs:211 serpen_count(team) 인라인
[L226] let enemy_serpen_count = get_game_mode().as_moba().unwrap().serpen_count[1 - team]

[L227] return Some( if my_serpen_count <= enemy_serpen_count { Outnumbered(1) } else { StackAhead(0) } )
        // IR: %51 = icmp ule my, enemy ; zext → i8 (m05.ll:53081~53082)

[L232] ret

※ 의미: 서펜 스택에서 아군이 앞서면(my > enemy) 포기 사유 = StackAhead(이미 앞서니 굳이 안 싸움),
   같거나 뒤지면 Outnumbered(열세). 동률(my == enemy)은 Outnumbered 쪽.
※ 부작용 없음. get_game_mode 를 3회 호출하지만 순수 조회.
```

**`mem` 메모리 접근 10건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext (m05.ll:52989) | 4 | OK |
| 1 | GameContext | 0x38 | tutorial: TutorialType(1B) | r | rule_scope::serpen_exists → TutorialType::spawn_serpen(runner.rs:267) 인라인. switch 0/5/7/8 만 통과 (m05.ll:52994~53000) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m05.ll:53003) | 4 | OK |
| 3 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data ptr) | r | get_game_mode 의 self | 4 | OK |
| 4 | AbstractGameWithCache | 0x8 | game (dyn AbstractGame vtable ptr) | r | vtable+0x40 = get_game_mode (divtable AbstractGame). 이 함수는 get_game_mode 를 3번 호출한다(L222·L225·L226 각각 as_moba().unwrap()) | 3 | OK |
| 5 | GameMode(16B enum, 반환값) | 0x0 | 판별자 | r | 0=Moba 아니면 unwrap_failed 패닉 (as_moba 인라인 game.rs:231) — 3곳 모두 | 4 | OK |
| 6 | GameMode(16B enum, 반환값) | 0x8 | Moba 페이로드 &MobaMode | r |  | 4 | OK |
| 7 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | gep 472. 0(is_empty) 이면 None 반환 (m05.ll:53020~53024) | 4 | OK |
| 8 | MobaMode | 0x260 | serpen_count [usize;2] | r | gep 608 + team*8. MobaMode::serpen_count(team) 헬퍼(game.rs:211) 인라인. [team] = my, [1-team] = enemy. team>=2 → panic_bounds_check(len 2) | 4 | OK |
| 9 | PlayerState | 0x930 | info.team | r | gep 2352 (m05.ll:53042) | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 218 | 태그 | TutorialType::None 태그 0 — spawn_serpen 허용. 또 GameMode 판별자 0=Moba, live_list.len==0 검사, SerpenGiveUpReason::StackAhead 태그 0 | 4 |
| 1 | 5 | 218 | 태그 | TutorialType::MidBottom 태그 5 — spawn_serpen 허용 | 4 |
| 2 | 7 | 218 | 태그 | TutorialType::Line 태그 7 — spawn_serpen 허용 | 4 |
| 3 | 8 | 218 | 태그 | TutorialType::Total 태그 8 — spawn_serpen 허용. 그 외(First/TopSolo/Bottom/MidSolo/JungleOnly)는 서펜 규칙 없음 → None | 4 |
| 4 | 2 | 232 | 센티널 | Option<SerpenGiveUpReason> None 니치 태그 2 (반환) / serpen_count 배열 길이 2 (bounds check) | 4 |
| 5 | 1 | 226 | 태그 | enemy team = 1 - team (sub nuw nsw 1, %team). 또 Outnumbered 태그 1 (zext i1) | 4 |
| 6 | 1152921504606846976 | 222 | 임계 | 2^60 — Vec::len 의 llvm.assume 상한(isize::MAX/8). 판정 아님, 컴파일러 힌트 | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 포기 사유 분기 비교 | serpen.rs:227 (IR m05.ll:53081 `icmp ule %39, %50`) | my_serpen_count <= enemy_serpen_count → Outnumbered | 채팅 사유 선택에만 영향(StackAhead ↔ Outnumbered). ule 를 ult 로 바꾸면 동률일 때 StackAhead 로 표시된다. 판정 자체(포기 여부)는 이 함수가 아니라 호출부가 한다 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | serpen_count | game_core::MobaMode::serpen_count | pub | fn(&game_core::MobaMode, usize) -> usize | game-core\src\simulation\game.rs:211 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | serpen_giveup_chat_reason | game_ai::plan_legacy::old::serpen_giveup_chat_reason | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::SerpenGiveUpReason> | game-ai\src\plan_legacy\old\serpen.rs:217 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | spawn_serpen | game_core::TutorialType::spawn_serpen | pub | fn(&game_core::TutorialType) -> bool | game-core\src\simulation\game\runner.rs:266 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 1개**: `live_list`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:32295) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 이 사유가 실제로 어떤 채팅 문구/행동으로 이어지는지(호출부 serpen.rs 의 give-up 판정)는 담당 범위 밖 | 4 |  |
| 1 | 표기 불가 | SerpenGiveUpReason 변형 이름(StackAhead/Outnumbered)과 my<=enemy 의 대응은 IR zext(0/1)+tcxdict 태그로 확정했으나, 소스가 `if my > enemy {StackAhead} else {Outnumbered}` 인지 `if my <= enemy {Outnumbered} else {StackAhead}` 인지는 표기 불가(외연 동일) | 3 |  |
| 2 | 미탐색 | serpen_count 가 '처치 스택 수'인지 '버프 스택'인지는 game_core 쪽 정의(MobaMode.serpen_count 갱신처) 를 안 읽어 확정 못 함 — 필드명 기준 추정 | 5 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

