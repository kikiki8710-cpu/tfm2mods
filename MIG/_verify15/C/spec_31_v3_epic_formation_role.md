---

### `31` v3_epic_formation_role — 모르가드(에픽) 운용 전략(Gather/Split14/Split131)과 내 포지션으로 '분할 담당 라인'인지 '본대 압박 라인'인지 배정

| 항목 | 값 |
|---|---|
| id | `epic__v3_epic_formation_role` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epic_formation_role` |
| 소스 | `game-ai\src\plan_legacy\old\epic.rs:781` |
| IR | `m09.ll` 64777~64972행 |
| 경로·가시성 | `game_ai::plan_legacy::old::v3_epic_formation_role` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `14592000` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(game_core::MorgardUseStrategy, game_core::Position, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::old::V3EpicFormation>
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | morgard_use | MorgardUseStrategy(8B, i64 로 전달) | tcxdict --enum: 니치 인코딩. low32 태그 5=Gather / 6=Split14(+0x4 position) / 0..4(=untagged)=Split131{+0x0 position1, +0x4 position2}. IR: %5=trunc low32, %6=lshr 32 (m09.ll:64778~64779) | 3 |
| 1 | 2 | position | Position(i32, range 0..5) | 판단 주체 선수의 포지션. tcxdict --enum Position: 0 Top/1 Jungle/2 Mid/3 Bottom/4 Support | 3 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽는다 | 4 |
| 3 | 4 | data | &OperationData(24B) | +0x0 cache, +0x8 context 를 읽어 하위 호출에 넘긴다 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v3_epic_formation_role(morgard_use: MorgardUseStrategy, position: Position, player: &PlayerState, data: &OperationData) -> Option<V3EpicFormation>
// V3EpicFormation { is_split: bool, line: LineType }

[L782] match morgard_use {

  // ── Gather (태그 5) ────────────────────────────────
  MorgardUseStrategy::Gather => {
[L784]  v3_group_press_line(player /*team=+0x930*/, data /*cache,context*/, exclude=None(-1))
            .map(|line| V3EpicFormation{ is_split:false, line })      // closure$0, None(-1) 이면 None
  }

  // ── Split14 (태그 6, position=split_position) ──────
  MorgardUseStrategy::Split14 { position: split_position } => {
[L787]  far_line: LineType = v3_split14_far_line(data) 인라인 (epic.rs:854~857):
            match data.cache.game.get_game_mode() {           // vtable+0x40
              GameMode::Moba(moba) => {                        // tag==0
[L855]          epic_tick   = moba.jungle_runner.epic.next_respawn_tick   (+0x1b0)
[L856]          serpen_tick = moba.jungle_runner.serpen.next_respawn_tick (+0x1e0)
[L857]          if epic_tick > serpen_tick { Top } else { Bottom }      // icmp ugt → select 0 : 2
              }
              _ => Top                                          // phi [0, %20]
            }
[L788]  if split_position == position && line_exists(data.context, far_line) {
            return Some(V3EpicFormation{ is_split:true, line: far_line })
        }
[L791]  v3_group_press_line(player, data, exclude=Some(far_line))
            .map(|line| V3EpicFormation{ is_split:false, line })      // closure$1
  }

  // ── Split131 (untagged, position1/position2) ───────
  MorgardUseStrategy::Split131 { position1, position2 } => {
[L796]  team = player.info.team (+0x930)          // team>=2 → panic_bounds_check(len 2)
        position1_absent = data.cache.player_champion(+0x1e0)[team][position1 as usize].is_none()   // pos>=5 → panic(len 5)
[L797]  position2_absent = data.cache.player_champion[team][position2 as usize].is_none()
[L798]  if line_exists(data.context, Top) {
[L799]      if position1 == position || (position2 == position && position1_absent) {
                return Some(V3EpicFormation{ is_split:true, line: Top })
            }
        }
[L801]  if line_exists(data.context, Bottom) {
[L802]      if position2 == position || (position1 == position && position2_absent) {
                return Some(V3EpicFormation{ is_split:true, line: Bottom })
            }
        }
[L804]  if line_exists(data.context, Mid) {
            return Some(V3EpicFormation{ is_split:false, line: Mid })
        }
[L807]  v3_group_press_line(player, data, exclude=None(-1))
            .map(|line| V3EpicFormation{ is_split:false, line })      // closure$2
  }
}
[L811] ret

※ 의미: Split14 = 1명이 '먼 라인'(에픽 리스폰이 서펜보다 늦으면 Top, 아니면 Bottom)을 홀로 맡고 나머지는 본대 압박.
   Split131 = position1 이 Top, position2 가 Bottom 을 맡되, 한쪽 담당 챔프가 없으면(None) 남은 사람이 대신. 셋째 라인 Mid 가 있으면 본대는 Mid.
※ line_exists 가 false 인 라인은 배정하지 않고 다음 후보로 넘어간다. 모든 폴백은 v3_group_press_line 이 정한다.
※ 부작용 없음. 패닉: team>=2, position1/2>=5 (bounds check).
```

**`mem` 메모리 접근 9건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | gep 2352. v3_group_press_line 의 1인자·player_champion[team] 인덱스(범위밖이면 panic_bounds_check len=2, m09.ll:64878) | 4 | OK |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache. get_game_mode 호출·player_champion 조회·v3_group_press_line 2인자 | 4 | OK |
| 2 | OperationData | 0x8 | context | r | &GameContext(64B). line_exists 1인자·v3_group_press_line 3인자 | 4 | OK |
| 3 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data ptr) | r | Split14 경로 far_line 계산 시 get_game_mode 의 self (m09.ll:64809) | 4 | OK |
| 4 | AbstractGameWithCache | 0x8 | game (dyn AbstractGame vtable ptr) | r | vtable+0x40 = get_game_mode (divtable AbstractGame 0x40) | 3 | OK |
| 5 | GameMode(16B enum, 반환값) | 0x0 | 판별자 | r | 0=Moba 면 페이로드 &MobaMode 사용, 아니면 far_line=Top (as_moba 인라인, game.rs:231; 여기선 unwrap 이 아니라 None 분기 있음) | 4 | OK |
| 6 | MobaMode | 0x1b0 | jungle_runner.epic.next_respawn_tick | r | gep 432 = epic_tick (v3_split14_far_line 인라인, epic.rs:855) | 4 | OK |
| 7 | MobaMode | 0x1e0 | jungle_runner.serpen.next_respawn_tick | r | gep 480 = serpen_tick (epic.rs:856) | 4 | OK |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion [[Option<&Entity>;5];2] | r | gep 480 → [team][position1] / [team][position2] 가 null(None) 인지 = position1_absent / position2_absent (Split131 경로, m09.ll:64887~64906) | 4 | OK |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 4 | 782 | 임계 | MorgardUseStrategy 니치 판별: low32 > 4 이면 태그(5 Gather / 6 Split14), 아니면(0..4 = 유효 Position) untagged Split131 (m09.ll:64785) | 4 |  |
| 1 | -5 | 782 | 계수 | 태그 5/6 → switch 인덱스 0(Gather)/1(Split14) 로 정규화 (niche_start=5) | 4 |  |
| 2 | 2 | 782 | 센티널 | 니치 밖(low32<=4) 이면 switch 인덱스 2 = Split131. 또 LineType::Bottom 태그 2 / Option<V3EpicFormation> None 니치 2 로도 쓰임 | 4 |  |
| 3 | -1 | 784 | 태그 | Option<LineType> None. v3_group_press_line 의 exclude 인자(None) 및 그 반환값 None 판정 (Gather·Split131 폴백 경로) | 4 |  |
| 4 | 0 | 857 | 태그 | LineType::Top 태그 0 (far_line 기본값·Split131 1순위 라인) / is_split=false | 4 |  |
| 5 | 1 | 804 | 태그 | LineType::Mid 태그 1 (Split131 3순위) / is_split=true | 4 |  |
| 6 | 5 | 796 | 임계 | player_champion[team][pos] 의 pos 상한(Position 5종) — panic_bounds_check len=5 (m09.ll:64897·64920) | 4 |  |
| 7 | 21474836480 | 797 | 임계 | = 5<<32. morgard_use 전체 i64 < 5<<32 ⇔ high32(position2) < 5 의 접힌 비교 (m09.ll:64898) | 4 | 5 |
| 8 | 4294967295 | 796 | 계수 | 0xFFFFFFFF 마스크 — i64 에서 low32(position1) 추출 (m09.ll:64881) | 4 |  |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Split14 먼 라인 선택 기준 | epic.rs:857 (v3_split14_far_line 인라인, IR m09.ll:64821 `icmp ugt %34, %36`) | epic.next_respawn_tick > serpen.next_respawn_tick ? Top : Bottom | 비교를 뒤집으면 분할 담당자가 반대 라인으로 간다. Moba 모드가 아니면 항상 Top | 4 | 기존 |
| 1 | Split131 라인 우선순위 | epic.rs:798~804 (line_exists Top → Bottom → Mid 순) | Top(0) → Bottom(2) → Mid(1) | line_exists 호출 순서를 바꾸면 position1/position2 가 맡는 라인이 바뀐다. 현재 position1=Top, position2=Bottom, 본대=Mid | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 1 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | v3_epic_formation_role | game_ai::plan_legacy::old::v3_epic_formation_role | pub | fn(game_core::MorgardUseStrategy, game_core::Position, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::old::V3EpicFormation> | game-ai\src\plan_legacy\old\epic.rs:781 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | v3_group_press_line | game_ai::plan_legacy::old::epic::v3_group_press_line | in:game_ai::plan_legacy::old::epic | fn(&game_core::PlayerState, &game_core::OperationData, std::option::Option<game_core::LineType>) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\old\epic.rs:863 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | v3_split14_far_line | game_ai::plan_legacy::old::epic::v3_split14_far_line | in:game_ai::plan_legacy::old::epic | fn(&game_core::OperationData) -> game_core::LineType | game-ai\src\plan_legacy\old\epic.rs:853 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 3개**: `next_respawn_tick`, `panic`, `player_champion`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 6곳** (m09.ll:7066, m09.ll:7094, m09.ll:7122, m09.ll:7150, m09.ll:7178, m09.ll:64331) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | v3_group_press_line(m09.ll:64423~) 본문은 담당 범위 밖이라 안 읽음 — '본대 압박 라인' 선정 기준은 그 함수 명세 참조 | 4 |  |
| 1 | 미탐색 | rule_scope::line_exists(m13.ll:53138~) 내부(어떤 맵/컨텍스트 조건으로 라인 존재를 판정하는지)는 안 읽음 | 4 |  |
| 2 | 미탐색 | morgard_use 가 어디서 결정되는지(팀 전략 설정 → MorgardUseStrategy) 는 호출부 범위 밖 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | epic.rs:799 / :802 의 `\|\|`·`&&` 결합 순서(소스 표기)는 column 정보 부재로 복원 불가 — 외연은 IR 의 and/or 로 확정(위 logic 대로) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

