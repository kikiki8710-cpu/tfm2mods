---

### `117` camp_idx — 정글 캠프 타입(JungleType) → 팀플랜 캠프 배열 인덱스 0..4 순수 매핑(Rhino0 Mushroom1 Bee2 Stump3 · Morgard/Serpen 은 unreachable 패닉)

| 항목 | 값 |
|---|---|
| id | `team_plan__camp_idx` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan8camp_idx` |
| 소스 | `game-ai\src\plan_legacy\team_plan.rs:34` |
| IR | `m09.ll` 54334~54359행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::camp_idx` · **pub** |
| 계층 | 기타 |
| exe | `de5300` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(game_core::JungleType) -> usize
```

<details><summary>인자 1개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | jungle | JungleType(1B enum, 태그 Direct) | tcxdict --enum JungleType: 0 Rhino / 1 Mushroom / 2 Stump / 3 Bee / 4 Morgard / 5 Serpen (idx==태그) | 3 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn camp_idx(jungle: JungleType) -> usize {
  match jungle {            // L35 switch i8
    JungleType::Rhino    => 0,   // 태그 0 (L35 default-through, phi [0,%1])
    JungleType::Mushroom => 1,   // 태그 1 (L37)
    JungleType::Bee      => 2,   // 태그 3 (L38)
    JungleType::Stump    => 3,   // 태그 2 (L39)
    JungleType::Morgard | JungleType::Serpen => unreachable!(),   // 태그 4·5 → core::panicking::panic (L40, team_plan.rs:40:10)
  }
}

★선언 순서(Rhino,Mushroom,Stump,Bee)와 배열 인덱스 순서(Rhino,Mushroom,Bee,Stump)가 다르다 — Stump↔Bee 가 뒤바뀐 매핑이며 이 함수가 그 변환 전담. 호출자 1곳(exe 0xde5300 · 54B).
```

**`mem` 메모리 접근 0건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 35 | 태그 | switch case JungleType 태그 0=Rhino → 인덱스 0 (m09:54337 → phi %1 값 0, m09:54357) | 4 |
| 1 | 1 | 37 | 태그 | 태그 1=Mushroom → 인덱스 1 (m09:54338 → %3 → phi 1) | 4 |
| 2 | 2 | 39 | 태그 | 태그 2=Stump → 인덱스 **3** (m09:54339 → %4 → phi 3, L39) — 개발자 주석 '(Rhino=0, Mushroom=1, Bee=2, Stump=3)' 과 일치 | 4 |
| 3 | 3 | 38 | 태그 | 태그 3=Bee → 인덱스 **2** (m09:54340 → %5 → phi 2, L38) | 4 |
| 4 | 40 | 40 | 길이 | panic 메시지 길이 40 = "internal error: entered unreachable code"(@anon…304) · Location @anon…305 = team_plan.rs:40:10 (m09:54344) — 판정값 아님 | 4 |

**`knobs` 조정점 0건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|

<details><summary>`callees` 피호출자 1건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_idx | game_ai::plan_legacy::team_plan::camp_idx | pub | fn(game_core::JungleType) -> usize | game-ai\src\plan_legacy\team_plan.rs:34 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m04.ll:32448, m04.ll:32575, m04.ll:62447) · **형제 0개** 

**`open` 1건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 이 인덱스를 소비하는 캠프 배열(길이 4)의 정체·호출자는 이 명세 범위 밖(호출자 1 — 잎 라운드라 미추적) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | 소스 L36 이 무엇인지(IR 에 줄 36 DILocation 없음 — `match jungle {` 다음 빈 줄이거나 Rhino arm 이 35 에 접힘 추정). 동작은 4 arm 전부 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

