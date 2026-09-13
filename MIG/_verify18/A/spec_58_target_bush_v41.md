---

### `58` target_bush_v41 — 갱커가 숨을 부시 id 를 (라인, 팀, 라인 리드 단계 0..6, 미드는 챔프 위치의 상/하 변) 로 룩업표에서 고른다

| 항목 | 값 |
|---|---|
| id | `ganker__target_bush_v41` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank6gankerNtB2_14LineGankerPlan15target_bush_v41` |
| 소스 | `game-ai\src\plan_legacy\old\line_gank\ganker.rs:310` |
| IR | `m08.ll` 94136~94351행 |
| 경로·가시성 | `game_ai::plan_legacy::old::LineGankerPlan::target_bush_v41` · **in:game_ai::plan_legacy::old::line_gank::ganker** |
| 계층 | 레거시 플랜 |
| exe | `db8ba0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | self.line | i8 = LineType 태그(0=Top·1=Mid·2=Bottom, tcxdict --enum: Direct 인코딩) | 소스 시그니처는 `fn target_bush_v41(&self, player:&PlayerState, data:&OperationData)->usize` (DISubprogram m08.ll:170575·170582~170584). 컴파일러가 인자를 스칼라로 쪼갬(SROA): 호출자 sub_plan(m08.ll:94953~94960) 이 `LineGankerPlan+0x28 line`·`PlayerState+0x930 info.team`·`PlayerState+0x9c0 info.position@tag`·`OperationData+0x0 cache`·`+0x8 context` 를 읽어 넘김 | 4 |
| 1 | 1 | team | i64 (usize) | player.info.team. 0 또는 1 — 2 이상이면 `lead[team]` 인덱싱에서 bounds 패닉(m08.ll:94212·94230·94339) | 4 |
| 2 | 2 | position | i32 (Position 태그) | player.info.position. Mid 분기에서만 `cache.player_champion[team][position]` 인덱스로 사용(zext, m08.ll:94235~94241) | 4 |
| 3 | 3 | cache | &AbstractGameWithCache(8840B) | data.cache | 4 |
| 4 | 4 | context | &GameContext(64B) | data.context. Mid 분기에서 setting.height 만 읽음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn target_bush_v41(line, team, position, cache, context) -> bush_id
  match line {                                                       // L311 switch(LineType 태그)
    Top(0) => {                                                       // L313
      bush_by_lead = if team==0 { [16,6,3,3,3,2,2] }                  // L314
                     else       { [2,3,6,6,6,16,16] }                 // L316
      lead = cache.top_lead[team]        // +0x21c0, team<2 아니면 패닉   L319
      return bush_by_lead[lead]          // lead<7 아니면 패닉             L319~320
    }
    Mid(1) => {                                                       // L322
      champ = cache.player_champion[team][position].unwrap()          // +0x1e0, L322 (None→패닉)
      low = (setting.height - champ.y) <u champ.x   // 인라인 is_top_side 의 IR 식. ★true = 대각선 아래쪽(x+y>height) — 이름과 반대 극성으로 추정(unknown 참조)  L323
      bush_by_lead = if team==0 { [low?21:17, low?14:11 ×4, low?9:4 ×2] }      // L323~ (select)
                     else       { [low?9:4,   low?14:11 ×4, low?21:17 ×2] }
      lead = cache.mid_lead[team]        // +0x21d0                        L337
      return bush_by_lead[lead]                                       // L337~338
    }
    Bottom(2) => {                                                    // L340
      bush_by_lead = if team==0 { [21,20,15,15,15,9,7] }              // L341
                     else       { [9,15,20,20,20,21,23] }              // L343
      lead = cache.bottom_lead[team]     // +0x21e0                     L346
      return bush_by_lead[lead]                                       // L346~347
    }
  }                                                                   // L349 ret

  표 구조: 인덱스 = 그 라인의 '리드 단계'(0..6). 팀0 표는 lead 가 커질수록 상대 진영 쪽(→ 표가 서로 거울: 팀1 표 = 팀0 표를 뒤집은 순서에 가깝다) 부시로 옮겨감. Mid 는 리드 단계 외에 미드 챔프가 대각선 어느 쪽(x+y>height 이면 21/14/9 계열, 아니면 17/11/4 계열)에 있는지로 상/하 부시 쌍을 고른다.
```

**`mem` 메모리 접근 8건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | AbstractGameWithCache | 0x21c0 | top_lead[team] | r | Top 분기. usize 0..6 (7 미만 아니면 bounds 패닉 m08.ll:94222). 값은 `AbstractGameWithCache::new_with_prev_cache`(_gcbc/g15.ll:102716, phi g15.ll:103619 값 0~6)가 계산 — 의미 미독해 | 4 | OK |
| 1 | AbstractGameWithCache | 0x21d0 | mid_lead[team] | r | Mid 분기 (m08.ll:94288~94291) | 4 | OK |
| 2 | AbstractGameWithCache | 0x21e0 | bottom_lead[team] | r | Bottom 분기 (m08.ll:94332~94335) | 4 | OK |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | Mid 분기. Option<&Entity>, None 이면 unwrap 패닉(m08.ll:94295, Location ganker.rs:322). 배열 stride: 팀 40B·포지션 8B (m08.ll:94238~94241) | 4 | OK |
| 4 | Entity | 0x660 | x | r | Mid 분기. 미드 챔프 좌표 (m08.ll:94254~94255) | 4 | OK |
| 5 | Entity | 0x668 | y | r | Mid 분기 (m08.ll:94256~94257) | 4 | OK |
| 6 | GameContext | 0x8 | setting | r | &GameSetting (m08.ll:94258~94259) | 4 | OK |
| 7 | GameSetting | 0x12c0 | height | r | 맵 높이. 인라인된 `map_regions::is_top_side(x,y)` = `(height - y) < x` (unsigned, dloc !55280/!55281 = map_regions.rs:22·24, m08.ll:94264~94267) | 4 | OK |

**`consts` 상수 14건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 16 | 314 | 산출값 | Top·팀0 표 `[16,6,3,3,3,2,2]` 의 lead=0 부시 / Top·팀1 표 `[2,3,6,6,6,16,16]` 의 lead=5,6 부시 (m08.ll:94182·94197~94198) | 4 |
| 1 | 6 | 314 | 산출값 | Top 표 부시 id (팀0 lead=1 / 팀1 lead=2,3,4) | 4 |
| 2 | 3 | 314 | 산출값 | Top 표 부시 id (팀0 lead=2,3,4 / 팀1 lead=1) | 4 |
| 3 | 2 | 314 | 임계 | Top 표 부시 id (팀0 lead=5,6 / 팀1 lead=0). ⚠같은 값 2 가 `lead[team]` bounds 상한(팀 수 2, m08.ll:94212) 으로도 등장 — 그건 배열 길이라 상수 아님 | 4 |
| 4 | 21 | 323 | 산출값 | Mid 표: IR 식 (h-y)<x 이면 21 아니면 17 — 팀0 lead=0 / 팀1 lead=5,6 슬롯 (select m08.ll:94275·94278). Bottom 표에서도 21(팀0 lead=0 / 팀1 lead=5, m08.ll:94309·94324) | 4 |
| 5 | 17 | 323 | 산출값 | Mid 표: (h-y)<x 거짓일 때의 21 짝 (select m08.ll:94275·94278) | 4 |
| 6 | 14 | 323 | 산출값 | Mid 표 lead=1..4 슬롯: IR 식 (h-y)<x 이면 14 아니면 11 (select m08.ll:94274, 팀 무관) | 4 |
| 7 | 11 | 323 | 산출값 | Mid 표 lead=1..4 슬롯의 (h-y)<x 거짓 값 | 4 |
| 8 | 9 | 323 | 산출값 | Mid 표: IR 식 (h-y)<x 이면 9 아니면 4 — 팀0 lead=5,6 / 팀1 lead=0 슬롯 (select m08.ll:94276~94277). Bottom 표에서도 9(팀0 lead=5 / 팀1 lead=0, m08.ll:94314·94319) | 4 |
| 9 | 4 | 323 | 산출값 | Mid 표: 9 의 (h-y)<x 거짓 짝 | 4 |
| 10 | 20 | 341 | 산출값 | Bottom 표 `팀0 [21,20,15,15,15,9,7]` lead=1 / `팀1 [9,15,20,20,20,21,23]` lead=2,3,4 (m08.ll:94310·94321~94323) | 4 |
| 11 | 15 | 341 | 산출값 | Bottom 표 부시 id (팀0 lead=2,3,4 / 팀1 lead=1) | 4 |
| 12 | 7 | 341 | 임계 | Bottom 표 팀0 lead=6 부시 (m08.ll:94315). ⚠값 7 은 `lead<7` bounds 상한(m08.ll:94208·94291·94335)으로도 등장 — 그쪽은 표 길이 | 4 |
| 13 | 23 | 343 | 산출값 | Bottom 표 팀1 lead=6 부시 (m08.ll:94325) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Top 부시 룩업표(팀0/팀1, lead 0..6) | ganker.rs:314·316 | [16,6,3,3,3,2,2] / [2,3,6,6,6,16,16] | 값을 바꾸면 그 리드 단계에서 갱커가 대기하는 부시가 바뀜(맵 부시 id 로 직접 지정). 유효 부시 id 범위 밖 값은 호출자 쪽 좌표 변환에서 문제 가능 — 미확인 | 4 | 기존 |
| 1 | Mid 부시 룩업표(상/하 쌍) | ganker.rs:323 | (h-y)<x ? [21,14,14,14,14,9,9] : [17,11,11,11,11,4,4] (팀0) · 팀1 은 첫/끝 슬롯 교환 | 대각선 판정은 `is_top_side` 고정. 표 값 변경 = 미드 갱 대기 부시 변경 | 4 | 기존 |
| 2 | Bottom 부시 룩업표 | ganker.rs:341·343 | [21,20,15,15,15,9,7] / [9,15,20,20,20,21,23] | 동상 | 4 | 기존 |

<details><summary>`callees` 피호출자 1건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | target_bush_v41 | game_ai::plan_legacy::old::LineGankerPlan::target_bush_v41 | in:game_ai::plan_legacy::old::line_gank::ganker | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize | game-ai\src\plan_legacy\old\line_gank\ganker.rs:310 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

**호출처 2곳** (m08.ll:94960, m08.ll:95509) · **형제 14개** (LineGankerPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::LineGankerPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:9 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan) -> game_ai::plan_legacy::old::LineGankerPlan |
| 1 | <game_ai::plan_legacy::old::LineGankerPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:9 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::LineGankerPlan::new | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:27 | True | fn(game_core::LineType, usize, usize) -> game_ai::plan_legacy::old::LineGankerPlan |
| 3 | game_ai::plan_legacy::old::LineGankerPlan::new_with_phase | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:31 | True | fn(game_core::LineType, usize, usize, game_ai::plan_legacy::old::LineGankerPhase) -> game_ai::plan_legacy::old::LineGankerPlan |
| 4 | game_ai::plan_legacy::old::LineGankerPlan::goal | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:35 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan) -> game_core::BigGoal |
| 5 | game_ai::plan_legacy::old::LineGankerPlan::update | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:39 | False | fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 6 | game_ai::plan_legacy::old::LineGankerPlan::is_end | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:64 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 7 | game_ai::plan_legacy::old::LineGankerPlan::is_cancel | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:69 | True | fn(&game_ai::plan_legacy::old::LineGankerPlan) -> bool |
| 8 | game_ai::plan_legacy::old::LineGankerPlan::make_gank_battle | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:76 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 9 | game_ai::plan_legacy::old::LineGankerPlan::next_plan | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:99 | False | fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 10 | game_ai::plan_legacy::old::LineGankerPlan::target_bush_v30 | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:248 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize |
| 11 | game_ai::plan_legacy::old::LineGankerPlan::target_bush_v41 | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:310 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize |
| 12 | game_ai::plan_legacy::old::LineGankerPlan::target_bush | in:game_ai::plan_legacy::old::line_gank::ganker | game-ai\src\plan_legacy\old\line_gank\ganker.rs:351 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> (u64, u64) |
| 13 | game_ai::plan_legacy::old::LineGankerPlan::sub_plan | pub | game-ai\src\plan_legacy\old\line_gank\ganker.rs:379 | False | fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | `*_lead[team]`(0..6) 의 정확한 의미 — `AbstractGameWithCache::new_with_prev_cache`(_gcbc/g15.ll:102716~, 27칸 라인 셀 점수 배열에서 phi 로 0~6 산출, g15.ll:103619·simulation.rs:1749) 를 훑었으나 본문 미독해. 이 명세는 '7단계 인덱스'까지만 확정. 미탐색 = g15.ll:103579~103619 손 독해 | 4 |  |
| 1 | 미탐색 | 부시 id → 맵 좌표 매핑(어느 id 가 어느 자리인지)은 MapDef 부시 목록 소관으로 이 함수 밖 — 미탐색 | 4 |  |
| 2 | 미탐색 | `is_top_side` 는 game_core 에 define 없이 인라인만 존재(grep ^define 0건). 확정된 것은 IR 식 `(height-y) <u x` 가 true 일 때 14/21/9 계열이라는 합성 사실뿐. 이름 극성은 handle_press_epic(m09.ll:5446~5448) 에서 같은 식의 true 쪽이 Bottom 라인 검사(소스 else 절 L535)로 가는 구조로 볼 때 `is_top_side == !((h-y)<x)` (컴파일러가 비교 반전+라벨 교환) 로 추정 — 즉 위 표의 `low`(=!is_top_side) 가 true 면 21/14/9. 확정하려면 오라클(is_top_side 가 pub 이면) 필요 | 2 |  |
| 3 | 표기 불가 | Mid 표의 `team==0` 조건과 `is_top_side` 조건이 소스에서 어떤 중첩 순서(`if team==0 { if top {..} }` 인지 `[if top {21} else {17}, ...]` 인지)로 쓰였는지 — column 부재 + select 로 접혀 표기 불가(동작은 확정) | 4 |  |
| 4 | 미탐색 | 리턴 타입이 소스에서 usize 인지 i64 인지 — IR 은 i64 이고 DISubroutineType 반환 !10 미해독(동작 무관) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

