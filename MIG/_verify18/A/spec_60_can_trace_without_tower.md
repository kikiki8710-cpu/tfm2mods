---

### `60` can_trace_without_tower — 대상 id 주위 반경 range 의 12방향 원주점 중 하나라도 타워 피격권 밖이면 true(타워 없이 추격 가능)

| 항목 | 값 |
|---|---|
| id | `tower_discipline__can_trace_without_tower` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline23can_trace_without_tower` |
| 소스 | `game-ai\src\tower_discipline.rs:676` |
| IR | `m07.ll` 50443~50611행 |
| 경로·가시성 | `game_ai::can_trace_without_tower` · **pub** |
| 계층 | 기타 |
| exe | `d98210` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::GameContext, &game_core::AbstractGameWithCache, usize, u64, u64, u64) -> bool
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | context | &GameContext(64B) | +0x8 setting(&GameSetting)·+0x20 map(&MapDef) 만 읽어 adjust_position 에 넘김 (m07.ll:50540~50543) | 4 |
| 1 | 1 | cache | &AbstractGameWithCache(8840B) | +0x0 game(&dyn AbstractGame 팻포인터: data·vtable) 만 읽음 (m07.ll:50451~50453) | 4 |
| 2 | 2 | id | i64 | 플레이어 id → AbstractGame::get_player(id) 로 &PlayerState 획득. None 이면 unwrap 패닉(Location tower_discipline.rs:678:36, @anon.81aa…175) | 4 |
| 3 | 3 | x | i64 | 원 중심 x (월드 좌표) | 4 |
| 4 | 4 | y | i64 | 원 중심 y | 4 |
| 5 | 5 | range | i64 | 원 반경. 원주점 = 중심 + 단위벡터(×1000)·range/1000 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn can_trace_without_tower(context, cache, id, x, y, range) -> bool
  game = cache.game (dyn AbstractGame)                              // L677
  player = game.get_player(id).unwrap()   // vtable+0x140, None → 패닉 (L678, Location 678:36)
  circle: [(dx,dy); 12] = [(0,1000),(500,866),(866,500),(1000,0),(866,-500),(500,-866),(0,-1000),(-500,-866),(-866,-500),(-1000,0),(-866,500),(-500,866)]  // L681 (30° 간격, ×1000)
  for (dx,dy) in circle:                                            // L681 iter (12회 상한)
    px = x + dx*range/1000                                          // L682 (sdiv, 내림)
    py = y + dy*range/1000                                          // L683
    (px,py) = Game::adjust_position(context.map, context.setting, px, py)   // L685 — 맵 경계·벽 보정(본문 미독해)
    hit = can_tower_focused(context, cache, player, px, py)         // L687
    if !hit: break                                                  // 루프 조건 = hit && idx!=12
  return !hit   // L693 — 마지막 평가값의 부정: 어떤 점이 안 맞으면 true, 12점 전부 맞으면 false

  ⟹ 의미상 `!circle.iter().all(|p| can_tower_focused(player, adjust(x+p*range)))` = `circle.iter().any(|p| !can_tower_focused(...))`.
  ⚠ 판정 주체(피격 여부를 묻는 챔프)는 `player`(id 의 PlayerState) — x,y 는 player 의 현재 좌표가 아니라 호출자가 넘긴 임의 중심(대상 위치 등). can_tower_focused 내부는 미독해.
```

**`mem` 메모리 접근 4건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 포인터 (m07.ll:50451) | 4 | OK |
| 1 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable 포인터; 슬롯 +0x140 = AbstractGame::get_player (divtable 확정) (m07.ll:50452~50458) | 3 | OK |
| 2 | GameContext | 0x20 | map | r | &MapDef(28112B) → adjust_position 1인자 (m07.ll:50540) | 4 | OK |
| 3 | GameContext | 0x8 | setting | r | &GameSetting(5432B) → adjust_position 2인자 (m07.ll:50542) | 4 | OK |

**`consts` 상수 4건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1000 | 681 | 계수 | 단위원 스케일(cos·sin ×1000). 원주 12점 표 = (0,1000)(500,866)(866,500)(1000,0)(866,-500)(500,-866)(0,-1000)(-500,-866)(-866,-500)(-1000,0)(-866,500)(-500,866) — 30° 간격. 682~683 줄에서 `dx*range/1000` 으로 나눠 반경으로 환산 (sdiv 1000, m07.ll:50584·50588) | 4 |
| 1 | 866 | 681 | 산출값 | sin60°·1000(=0.866). 원주점 표의 성분 (m07.ll:50498 등) | 4 |
| 2 | 500 | 681 | 산출값 | sin30°·1000. 원주점 표의 성분 (m07.ll:50496 등) | 4 |
| 3 | 12 | 681 | 태그 | 원주 표본 점 개수(배열 길이). 루프 상한 `icmp ne %47, 12` (m07.ll:50603). 배열 12×(i64,i64)=192B 스택 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 원주 표본 점 개수 | tower_discipline.rs:681 | 12 | 늘리면(더 촘촘한 각도) '타워 밖 자리'를 더 잘 찾아 true 가 잦아짐(추격 허용↑). 줄이면 보수적. 단 배열 리터럴이라 값·개수 둘 다 바꿔야 함 | 4 | 기존 |
| 1 | 반경 환산 분모 | tower_discipline.rs:682~683 | 1000 | 표 성분 스케일과 짝. 분모를 키우면 실효 반경이 줄어(중심 가까이 검사) true 가 어려워지고, 줄이면 반경이 커져 true 가 쉬워짐. 표와 함께 바꿔야 의미 유지 | 4 | 기존 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust | game_ai::ScoreParameter::<'a>::adjust | pub | fn(&mut game_ai::ScoreParameter<'a/#0>, &mut rand::rngs::std::StdRng) | game-ai\src\score_parameter.rs:375 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 1 | adjust | game_ai::ChampionScoreParameter::<'_>::adjust | pub | fn(&mut game_ai::ChampionScoreParameter</#0>, &mut rand::rngs::std::StdRng) | game-ai\src\score_parameter.rs:1441 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 2 | adjust | game_core::InputTarget::adjust | pub | fn(&game_core::InputTarget, u64, u64, u64) -> game_core::InputTarget | game-core\src\simulation\state\player.rs:1465 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | can_tower_focused | game_ai::can_tower_focused | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\tower_discipline.rs:9 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_trace_without_tower | game_ai::can_trace_without_tower | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, usize, u64, u64, u64) -> bool | game-ai\src\tower_discipline.rs:676 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | get_player | game_core::AbstractGame::get_player | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation.rs:143 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_player | <game_core::Game as game_core::AbstractGame>::get_player | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation\game.rs:1866 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_player | <game_core::SingleLaneGame as game_core::AbstractGame>::get_player | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation\game.rs:3914 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
</details>

**호출처 3곳** (m02.ll:37404, m10.ll:49250, m15.ll:11761) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | can_tower_focused(context, cache, player, x, y) 내부(피격권 판정식·타워 선택·시즈 스탠스 예외)는 담당 범위 밖이라 미독해 — `_docs/game_ai.txt:83~85` 의 v47 주석('시즈 스탠스가 선 타워는 소커 본인·커버를 위험으로 안 침')이 그 함수로 추정되나 이름 대응 미확인 | 5 |  |
| 1 | 미탐색 | 호출자가 x,y 로 무엇을 넣는지(대상 적 좌표인지, 자기 좌표인지)는 이 함수 본문만으로는 알 수 없음 — 미탐색 = 호출자 그래프(`grep can_trace_without_tower` 전 .ll) | 4 |  |
| 2 | 표기 불가 | 루프의 소스 표기(`iter().all` vs `any(!…)` vs 수동 for) — 외연 동일·column 정보 부재로 표기 불가(동작은 확정) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | Game::adjust_position(&MapDef,&GameSetting,x,y) 내부(_gcbc/g15.ll:72245~)는 `icmp ult 30`·`ult 960000` 등 맵 경계 클램프로 보이나 본문 전체 미독해 — 이 명세는 '좌표 보정 후 그 자리를 검사한다'까지만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

