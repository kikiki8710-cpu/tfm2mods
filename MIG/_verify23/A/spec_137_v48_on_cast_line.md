---

### `137` v48_on_cast_line — 점(px,py)이 현재 시전 중인 논타겟 빔(원/선분) 폭 안에 있는가 — 빔 중 하나라도 dist <= champ_r+15000+halfwidth 이면 true

| 항목 | 값 |
|---|---|
| id | `battle__v48_on_cast_line` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battle16v48_on_cast_line` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle.rs:1057` |
| IR | `m02.ll` 66030~66193행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::battle::v48_on_cast_line` · **in:game_ai::plan_legacy::sub_plan::battle** |
| 계층 | 기타 |
| exe | `cd0480` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | info.id(+0x928) 만 읽음 — TLS 캐시 키(pid) | 4 |
| 1 | 2 | data | &OperationData(24B) | cache(+0x0)→game(dyn AbstractGame) 의 seed/tick 슬롯 호출 — TLS 캐시 키. 클로저에도 전달 | 4 |
| 2 | 3 | champ_r | usize | 챔피언 반경. 판정 반경 = champ_r + 15000 + beam.halfwidth | 4 |
| 3 | 4 | px | usize | 검사 점 x | 4 |
| 4 | 5 | py | usize | 검사 점 y | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v48_on_cast_line(player, data, champ_r, px, py) -> bool  // battle.rs:1057
  // L1058: let (n, beams) = v48_cast_beams(player, data)  — 인라인: TLS 캐시(with) 호출
  //   키 = (seed = data.cache.game.seed()  [L999], tick = data.cache.game.tick() [L1000], pid = player.info.id [L1001])
  //   클로저 env {&seed,&tick,&pid,data,player} 40B 를 LocalKey::with 에 넘김 (m02.ll:66074~66083) → sret 296B
  //   sret 296B 살아있는 바이트: +0 n:usize, +8.. [Beam;6] 중 앞 n개(Beam 48B: x1@0,y1@8,x2@16,y2@24,kind@32(i8, 33..39 패딩),halfwidth@40)
  let beams = &beams[..n]        // L1059, n>=7 이면 slice_index_fail 패닉 (m02.ll:66109/66137)
  let r = champ_r + 15000        // m02.ll:66133 (L1059 문맥)
  for b in beams {               // n==0 이면 즉시 false (m02.ll:66129)
     if b.kind == 0 {            // L1060
        if distance(px,py,b.x1,b.y1) <= r + b.halfwidth { return true }   // L1061 (icmp ugt → 초과면 다음 빔)
     } else {
        if dist_to_line_segment(px,py,b.x1,b.y1,b.x2,b.y2) <= r + b.halfwidth { return true }  // L1064
     }
  }
  false                          // L1069 (phi m02.ll:66190: %31·%61 → false, %49·%53 → true)
```

**`mem` 메모리 접근 12건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x928 | info.id | r | m02.ll:66070 — TLS 캐시 키 pid (tcxdict PlayerState 0x928) | 3 | OK |
| 1 | OperationData | 0x0 | cache | r | m02.ll:66056 → &AbstractGameWithCache | 4 | OK |
| 2 | AbstractGameWithCache | 0x0 | game.data_ptr | r | m02.ll:66057 dyn AbstractGame 데이터 포인터 | 4 | OK |
| 3 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | m02.ll:66059 | 4 | OK |
| 4 | vtable(AbstractGame) | 0x20 | seed | r | m02.ll:66060~66062 `seed()` → i64 (divtable AbstractGame 0x20) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 5 | vtable(AbstractGame) | 0x28 | tick | r | m02.ll:66065~66067 `tick()` → i64 (divtable AbstractGame 0x28) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 6 | Beam(48B, 로컬 [Beam;6] 원소) | 0x0 | x1 | r | m02.ll:66147 | 4 | 오귀속(사전은 다른 필드를 준다) |
| 7 | Beam(48B, 로컬 [Beam;6] 원소) | 0x8 | y1 | r | m02.ll:66150 | 4 | 오귀속(사전은 다른 필드를 준다) |
| 8 | Beam(48B, 로컬 [Beam;6] 원소) | 0x10 | x2 | r | m02.ll:66171 — kind!=0 일 때만 읽음 | 4 | 오귀속(사전은 다른 필드를 준다) |
| 9 | Beam(48B, 로컬 [Beam;6] 원소) | 0x18 | y2 | r | m02.ll:66168 — kind!=0 일 때만 읽음 | 4 | 오귀속(사전은 다른 필드를 준다) |
| 10 | Beam(48B, 로컬 [Beam;6] 원소) | 0x20 | kind | r | m02.ll:66145 i8. 0=점/원(중심 x1,y1) · 그 외(클로저가 1 을 store, m00.ll:93748)=선분(x1,y1)-(x2,y2) | 4 | 오귀속(사전은 다른 필드를 준다) |
| 11 | Beam(48B, 로컬 [Beam;6] 원소) | 0x28 | halfwidth | r | m02.ll:66155 빔 반폭 | 4 | 오귀속(그 오프셋에 필드가 없다(패딩/관통불가)) |

**`consts` 상수 4건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 15000 | 1059 | 계수 | 판정 여유 반경(≈0.47셀). 임계 = champ_r + 15000 + halfwidth (m02.ll:66133, 66162, 66174) | 4 |
| 1 | 0 | 1060 | 태그 | beam.kind == 0 → 원(점거리) 판정 분기 (m02.ll:66157) | 4 |
| 2 | 7 | 1059 | 임계 | `beams[..n]` 슬라이스 경계검사 n<7 (n<=6, 배열 용량 6). 판정 상수 아님 (m02.ll:66109) | 4 |
| 3 | 6 | 1059 | 미상 | 빔 배열 용량 [Beam;6] — slice_index_fail 인자 (m02.ll:66137). 판정 상수 아님 | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 빔 판정 여유 반경 | battle.rs:1059 (m02.ll:66133) | 15000 | 올리면 더 먼 점도 '시전선 위'로 판정(회피 스텝이 더 자주/넓게 발동), 내리면 빔 폭에 바짝 붙어야만 true | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | dist_to_line_segment | game_core::utils::dist_to_line_segment | pub | fn(i64, i64, i64, i64, i64, i64) -> u64 | game-core\src\utils.rs:200 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | seed | game_core::AbstractGame::seed | pub | fn(&Self/#0) -> u64 | game-core\src\simulation.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | seed | <game_core::Game as game_core::AbstractGame>::seed | pub | fn(&game_core::Game) -> u64 | game-core\src\simulation\game.rs:1564 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | seed | <game_core::SingleLaneGame as game_core::AbstractGame>::seed | pub | fn(&game_core::SingleLaneGame) -> u64 | game-core\src\simulation\game.rs:3781 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 6 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 7 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | v48_cast_beams | game_ai::plan_legacy::sub_plan::battle::v48_cast_beams | in:game_ai::plan_legacy::sub_plan::battle | fn(&game_core::PlayerState, &game_core::OperationData) -> (usize, [(u8, i64, i64, i64, i64, u64); 6_usize]) | game-ai\src\plan_legacy\sub_plan\battle.rs:998 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | v48_on_cast_line | game_ai::plan_legacy::sub_plan::battle::v48_on_cast_line | in:game_ai::plan_legacy::sub_plan::battle | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64) -> bool | game-ai\src\plan_legacy\sub_plan\battle.rs:1057 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 2개**: `slice_index_fail`, `{closure#0}>`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 6곳** (m02.ll:32376, m02.ll:33953, m02.ll:34010, m02.ll:34877, m02.ll:74264, m02.ll:74448) · **형제 0개** 

**`open` 2건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | Beam 원소 타입 = tcx sig `(u8, i64, i64, i64, i64, u64)`(v48_cast_beams 반환 `(usize, [(u8,i64,i64,i64,i64,u64);6])`, _tcx game_ai.json i=3128). 메모리 배치는 rustc 재배열: i64 x1@0,y1@8,x2@16,y2@24, u8 kind@32(+33..39 패딩), u64 halfwidth@40 — 필드 이름은 dbg_value 이름(m02.ll:66146~66156). 열거형이 아니라 u8 이며 클로저가 0/1 만 store(m00.ll:93651·93748). 남는 unknown: 소스에서 u8 값 0/1 의 상수명(표기 불가 — IR 리터럴) | 3 |  |
| 1 | 표기 불가 | 15000 이 소스에서 리터럴인지 상수명인지(표기 불가 — IR 에 리터럴로 접힘). 값·위치(L1059 문맥, r=champ_r+15000 이 루프 전 계산)는 확정 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | exe 0xcd0480 의 fastcc 레지스터 배치(argscan 미실시) — IR 인자 5개는 소스 인자와 1:1 이라 승격 없음은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

