---

### `134` around::check_cell — 수풀 주변대기 경로 셀 판정 — out_line 모드별 지역(region) 규칙으로 Soft, 통과 시 타워 회피 판정으로 Allow/Danger

| 항목 | 값 |
|---|---|
| id | `around__check_cell` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai12small_action6around10check_cell` |
| 소스 | `game-ai\src\small_action\around.rs:1256` |
| IR | `m08.ll` 112301~112665행 |
| 경로·가시성 | `game_ai::small_action::around::check_cell` · **in:game_ai::small_action::around** |
| 계층 | 기타 |
| exe | `dc8550` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::PositioningScoreData, &game_core::OperationData, &game_ai::TowerDodgeContext, game_ai::AroundBushOutlineType, usize, usize, usize, usize, usize, usize) -> game_ai::PathVerdict
```

<details><summary>인자 12개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 분기 없음. dodge_tower_cell_with_context 에 그대로 전달(112656) | 4 |
| 1 | 2 | player | &PlayerState(2528B, readonly) | 본문에서 읽지 않음. 콜리에 전달만 | 4 |
| 2 | 3 | positioning_score | &PositioningScoreData(2760B, readonly) | 본문에서 읽지 않음. 콜리에 전달만 | 4 |
| 3 | 4 | data | &OperationData(24B, readonly) | +0x8 context → GameContext(+0x8 setting · +0x20 map) 만 읽음 | 4 |
| 4 | 5 | tower_dodge | &TowerDodgeContext(416B) | IR 속성에 readonly/noalias 없음 = 내부 Cell 필드(memo_s_key +0x10 · memo_inside_mask +0x198) 때문(tcxdict). 이 함수 자신은 읽기·쓰기 0, 콜리에 전달만 | 3 |
| 5 | 6 | out_line | AroundBushOutlineType(i8, range 0..3) | 태그 0=None 1=Outline 2=Inline (tcxdict --enum, Direct 인코딩). switch 112316 | 3 |
| 6 | 7 | sx | usize | 시작 셀 x (regions 열 인덱스, <30 아니면 panic_bounds_check) | 4 |
| 7 | 8 | sy | usize | 시작 셀 y (regions 행 인덱스) | 4 |
| 8 | 9 | nx | usize | 판정 대상(다음) 셀 x | 4 |
| 9 | 10 | ny | usize | 판정 대상(다음) 셀 y | 4 |
| 10 | 11 | tx | usize | 목표 셀 x — Outline/Inline 경로에서만 사용(None 경로에선 미사용) | 4 |
| 11 | 12 | ty | usize | 목표 셀 y — 위와 같음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
check_cell(version, player, ps, data, tower_dodge, out_line, sx,sy, nx,ny, tx,ty) -> PathVerdict   [around.rs:1256]
 map = data.context.map ; setting = data.context.setting
 match out_line (L1257, switch 112316):
  None(0)    → goto FINAL
  Outline(1) →                                                                   (L1260~1277)
     start_region = map.regions[sy][sx]     (L1260, 각 인덱스 <30 아니면 panic_bounds_check)
     end_region   = map.regions[ty][tx]     (L1261)
     nxt_region   = map.regions[ny][nx]     (L1262)
     start_in_top = is_top_side(ctx, sx*32000+16000, sy)   (L1264 · 인라인 map_regions.rs:22~24)
                  = ((setting.height - 16000) - sy*32000) >= (sx*32000+16000)      [112402~112407 icmp uge]
     end_in_top   = ((setting.height - 16000) - ty*32000) >= (tx*32000+16000)      (L1265; IR 는 부정형 `icmp ult` 112418)
     if start_in_top == end_in_top:                          (L1267; `xor %58,%63` 112419 = start_in_top ^ !end_in_top)
        # 같은 쪽(대각선 기준)                                 (L1268, 블록 %106)
        ok = (nxt_region == end_region) || (nxt_region == start_region) || !map.is_line_region[nxt_region]
        if ok → goto FINAL  else → return Soft(1)
     else:                                                    (L1276, 블록 %69)
        if !map.is_line_region[nxt_region] → goto FINAL
        seq = map.lane_seq(LineType::Mid(1), team=1)   # [usize;7] sret 56B (112446)
        if seq[0..7] 중 어느 것 == nxt_region → goto FINAL   (112465~112540, 7개 select 체인)
        if nxt_region == start_region || nxt_region == end_region → goto FINAL   (L1277)
        return Soft(1)
  Inline(2) →                                                                    (L1284~1288, 블록 %120~%149)
     start_region = regions[sy][sx]; end_region = regions[ty][tx]; nxt_region = regions[ny][nx]
     ok = (nxt_region == start_region) || map.is_line_region[nxt_region] || (nxt_region == end_region)
     if ok → goto FINAL  else → return Soft(1)
 FINAL: (L1300, 블록 %159)
   if dodge_tower_cell_with_context(version, player, data, ps, tower_dodge, sx, sy, nx, ny) → return Allow(0)
   else → return Danger(2)
 return 시 payload i32 = undef (L1305)

요지: Outline 은 '라인 지역을 피해 돌아가라'(같은 쪽이면 라인 지역 진입 = Soft, 대각선을 넘을 땐 미드(팀1 시퀀스) 라인 지역만 허용), Inline 은 '라인 지역 안에서만 움직여라'(비라인 지역 = Soft). 시작·목표 지역은 항상 면제. 통과한 셀은 타워 회피 판정이 최종 Allow/Danger 를 정한다.
```

**`mem` 메모리 접근 6건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext (112343·112582) | 4 | OK |
| 1 | GameContext | 0x20 | map | r | &MapDef (112346·112584) | 4 | OK |
| 2 | GameContext | 0x8 | setting | r | &GameSetting — is_top_side 인라인(map_regions.rs:22) 안에서 (112402~112403) | 4 | OK |
| 3 | GameSetting | 0x12c0 | height | r | 맵 세로 크기(월드 단위). ry = height-16000 - y*32000 (112402~112405) | 4 | OK |
| 4 | MapDef | 0x38b8 | regions[30][30] | r | regions[y][x] usize — start/end/nxt 지역 id (112347~112353, 112372~112373, 112391~112393, Inline 경로 112582~112629) | 4 | OK |
| 5 | MapDef | 0x6db0 | is_line_region[27] | r | bool 배열, MapDef::is_line_region(map_def.rs:266) 인라인. 인덱스 nxt_region <27 아니면 panic (112434·112556·112640) | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 32000 | 1264 | 미상 | 셀→월드 좌표 변환(셀 크기). x = sx*32000+16000, y 항은 -32000*sy (좌표 변환, 임계 아님) | 4 |
| 1 | 16000 | 1264 | 미상 | 셀 중심 오프셋. is_top_side: ry = (height - 16000) - sy*32000 (112406 `add i64 %55, -16000`) | 4 |
| 2 | -32000 | 1264 | 미상 | y 축 반전 셀 변환(`mul nsw i64 %7, -32000`, 112397~112398·112409) | 4 |
| 3 | -16000 | 1264 | 계수 | height - 16000 (112406). 16000 의 부호 반전 표현 | 4 |
| 4 | 1 | 1276 | 태그 | lane_seq(map, LineType tag 1=Mid, team 1) 의 두 인자 모두 1 (112446). 또한 PathVerdict::Soft 태그값 1 (112661 phi) | 4 |
| 5 | 0 | 1300 | 태그 | PathVerdict::Allow 태그 0 (112657 select true 측) | 4 |
| 6 | 2 | 1300 | 태그 | PathVerdict::Danger 태그 2 (112657 select false 측). AroundBushOutlineType::Inline 태그도 2 (switch 112316) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Outline 대각선 횡단 시 허용되는 라인 시퀀스(lane_seq 의 LineType/team 인자) | around.rs:1276 (IR 112446 `i8 1, i64 1`) | Mid(1), team 1 | 다른 라인/팀으로 바꾸면 횡단 시 통과 허용되는 라인 지역 집합이 바뀐다(현재는 팀1 미드 시퀀스 7지역 고정 — 호출 선수 팀과 무관) | 4 | 기존 |
| 1 | 라인 지역 위반 시 판정 등급 | around.rs:1268/1277/1288 (phi 112661 상수 1) | Soft(1) | Danger(2)/Deadly(3)로 올리면 경로탐색이 그 셀을 더 강하게 기피(가격은 호출측 path_field 규칙) | 4 | 기존 |
| 2 | 타워 회피 실패 등급 | around.rs:1300 (select 112657) | Danger(2) | Soft 로 내리면 타워 사거리 셀을 완전 차단이 아닌 감점으로 취급 | 4 | 기존 |

<details><summary>`callees` 피호출자 5건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | check_cell | game_ai::small_action::around::check_cell | in:game_ai::small_action::around | fn(usize, &game_core::PlayerState, &game_core::PositioningScoreData, &game_core::OperationData, &game_ai::TowerDodgeContext, game_ai::AroundBushOutlineType, usize, usize, usize, usize, usize, usize) -> game_ai::PathVerdict | game-ai\src\small_action\around.rs:1256 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | check_cell | game_ai::SmallActionAroundHide::check_cell | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::PositioningScoreData, &game_core::OperationData, &game_ai::TowerDodgeContext, usize, usize, usize, usize, u64, u64) -> game_ai::PathVerdict | game-ai\src\small_action\around.rs:411 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | dodge_tower_cell_with_context | game_ai::dodge_tower_cell_with_context | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::TowerDodgeContext, usize, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1322 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | is_top_side | game_core::is_top_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:21 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 4 | lane_seq | game_core::MapDef::lane_seq | pub | fn(&game_core::MapDef, game_core::LineType, usize) -> [usize; 7_usize] | game-core\src\simulation\map_def.rs:250 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `out_line`, `undef`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 30곳** (m00.ll:20818, m00.ll:24066, m00.ll:28607, m00.ll:28809, m00.ll:30203, m00.ll:30405, m00.ll:61639, m00.ll:62037, m00.ll:62174, m00.ll:62641, m00.ll:63039, m00.ll:63176, m00.ll:86126, m00.ll:96370, m00.ll:96486, m03.ll:21150, m03.ll:21311, m03.ll:32711, m03.ll:64707, m03.ll:64862, m03.ll:133987, m03.ll:134693, m08.ll:23670, m08.ll:24027, m08.ll:47397, m08.ll:48585, m08.ll:76804, m08.ll:77841, m08.ll:90150, m08.ll:90323) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | lane_seq 인자 `team=1` 이 소스에서 리터럴 1 인지 상수 접힘된 식인지 — IR 엔 `i64 1` 리터럴만 있고 player 팀 필드 읽기가 없어 리터럴로 판단(적용 범위: 이 IR 본문 한정) | 4 |  |
| 1 | 미탐색 | dodge_tower_cell_with_context(path_finder.rs:1322) 내부 — 같은 라운드 경계(중간 콜리) 규칙대로 시그니처만 기록: fn(version:usize, &PlayerState, &OperationData, &PositioningScoreData, &TowerDodgeContext, sx,sy,nx,ny:usize)->bool (true=통과 가능→Allow) | 4 |  |
| 2 | 미탐색 | MapDef::lane_seq(map_def.rs:250) 내부 — game_core 경계: fn(&MapDef, LineType, usize team)->[usize;7] (56B sret). 7원소가 전부 유효 지역 id 인지(패딩/센티넬 여부)는 미확인 | 4 |  |
| 3 | 표기 불가 | is_top_side 의 정확한 소스 표현(`>=` vs `>`) — IR `icmp uge`(start) / `icmp ult` 부정(end) 로 동작은 확정, 소스 표기만 미확정 | 4 |  |
| 4 | 미탐색 | `out_line` 값 3 이상은 `unreachable`(112323) — switch default. 호출자가 보장 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

