---

### `146` SmallActionAroundBush::new_with_out_line — 수풀 id 에 속한 하드코딩 71셀 중 하나를 rng 로 무작위 선택해 SmallActionAroundBush 를 생성(후보 0이면 메시지 출력 후 패닉)

| 항목 | 값 |
|---|---|
| id | `SmallActionAroundBush__new_with_out_line` |
| 심볼 | `_RNvMs6_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_21SmallActionAroundBush17new_with_out_line` |
| 소스 | `game-ai\src\small_action\around.rs:1154` |
| IR | `m08.ll` 105131~105277행 |
| 경로·가시성 | `game_ai::SmallActionAroundBush::new_with_out_line` · **pub** |
| 계층 | 기타 |
| exe | `dc27a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut SmallActionAroundBush(120B) | writes 참조 | 4 |
| 1 | 1 | rnd | &mut StdRng(320B, align 16) | ★&mut. 이 본문 직접 store 0건 — 쓰기는 콜리 SliceRandom::choose → StdRng::gen_range::<u32>(0..len) 1회(m12.ll:4092 choose 본문: len<2^32 이면 u32 경로)로 ChaCha 상태 전진. 후보 0개면 choose 가 gen_range 를 호출하지 않아 rng 무변경(그리고 직후 패닉) | 4 |
| 2 | 2 | data | &OperationData(24B, readonly) | +0x0 cache → +0x0 game 팻포인터(tick 호출) · +0x8 context → map | 4 |
| 3 | 3 | player | &PlayerState (IR: `ptr noalias readonly captures(none) %3`, noundef 없음) | ★본문에서 전혀 읽지 않음(dead 인자). tcx 시그니처 3번째 = &PlayerState | 3 |
| 4 | 4 | bush | usize | 수풀 id — 필터 비교 + self.bush 저장 + 후보 0 메시지의 Display 인자 | 4 |
| 5 | 5 | out_line | AroundBushOutlineType(i8 0..3) | 그대로 저장, 분기 없음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
SmallActionAroundBush::new_with_out_line(rnd, data, player, bush, out_line) -> Self   [around.rs:1154]
 map = data.context.map
 candidates: Vec<(usize,usize)> = BUSH_CELLS(71).into_iter().filter(|(x,y)| map.bushes[*y][*x] == bush).collect()   (L1155)
 if candidates.len() == 0:                                                       (L1157)
     print!("no candidates for bush: {}\n", bush)      # stdout, @anon…99      (L1158)
 chosen = candidates.choose(rnd).unwrap()      # 비면 None → unwrap_failed 패닉     (L1161)
     # choose: len==0 → None ; len<2^32 → &v[rnd.gen_range::<u32>(0..len)]
 target_x = chosen.x*32000+16000 ; target_y = chosen.y*32000+16000               (L1162~1163)
 self = { start_tick: game.tick() (L1166), change_tick: game.tick() (L1167), bush, target_x, target_y, path_finder: None, out_line }   (L1165)
 drop(candidates)                                                                  (L1176)

`player` 인자는 미사용. 판정 없음(생성자). new_with_target 과의 차이 = 최근접 선택 → **rng 무작위 선택** + 후보 0 경고 출력.
```

**`mem` 메모리 접근 15건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | 105152 | 4 | OK |  |
| 1 | GameContext | 0x20 | map | r | &MapDef → 필터 클로저 환경 (105154) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (105219) | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x0 | game.data_ptr / vtable_ptr(+0x8) | r | 105220~105225; vtable+0x28 = AbstractGame::tick, 2회 호출(L1166·L1167) | 4 | OK |  |
| 4 | MapDef | 0x1c98 | bushes[30][30] | r | aux 113452 `gep 7320` → bushes[y][x] | 4 | OK |  |
| 5 | Vec<(usize,usize)>(candidates, 지역) | 0x10 | len | r | 105171 (+0x8 ptr 105202) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 6 | SmallActionAroundBush(sret) | 0x0 | start_tick | w | 105248 | 4 | OK | game.tick() (L1166) |
| 7 | SmallActionAroundBush(sret) | 0x8 | change_tick | w | 105250 | 4 | OK | game.tick() (L1167, 재호출) |
| 8 | SmallActionAroundBush(sret) | 0x10 | bush | w | 105253 | 4 | OK | 인자 bush |
| 9 | SmallActionAroundBush(sret) | 0x18 | target_x | w | 105255 | 4 | OK | chosen.x*32000+16000 (L1162) |
| 10 | SmallActionAroundBush(sret) | 0x20 | target_y | w | 105257 | 4 | OK | chosen.y*32000+16000 (L1163) |
| 11 | SmallActionAroundBush(sret) | 0x6d | path_finder@tag | w | 105259 | 4 | OK | 2 = None |
| 12 | SmallActionAroundBush(sret) | 0x70 | out_line | w | 105261 | 4 | OK | 인자 out_line |
| 13 | StdRng(rnd, &mut) | 0x0 | ChaCha12 상태(320B 전체 중 gen_range 가 건드리는 부분) | w | 콜리 choose(m12.ll:4092) 내부에서만 쓰기. 이 본문엔 store 없음. 정확한 바이트 범위는 rand 크레이트 경계 — 미탐색 | 4 | OK | gen_range::<u32>(0..len) 1회 소비 |
| 14 | stdout | 0x0 | (부작용) print! | w | 후보 0개일 때만. 오프셋 없음(서술) | 4 | 확인불가(tcx 사전에 타입 없음) | "no candidates for bush: {bush}\n" (@anon…99, 105183) |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 71 | 1155 | 산출값 | 후보 셀 표 원소 수([(usize,usize);71], @anon…94 — 표 내용은 new_with_target 명세 constants 참조, 동일 전역) | 4 |
| 1 | 0 | 1157 | 태그 | candidates.len() == 0 → 경고 출력 분기 (105174) | 4 |
| 2 | 32000 | 1162 | 계수 | 셀→월드 변환 | 4 |
| 3 | 16000 | 1162 | 계수 | 셀 중심 오프셋 | 4 |
| 4 | 2 | 1165 | 센티널 | Option<PathFinder>::None 니치 태그(105259) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 수풀 후보 셀 표 | around.rs:1155 (@anon…94) | 71 (x,y) | new_with_target 과 공유. 표와 맵 bushes 가 어긋나면 후보 0 → 경고 후 패닉 | 4 | 기존 |
| 1 | 셀 선택 방식 | around.rs:1161 | rng 균등 무작위(choose) | 결정적 선택(예: 최근접)으로 바꾸면 rng 소비가 1회 줄어 이후 시드 스트림이 전부 어긋난다(리플레이 비트동일 깨짐) | 1 | 기존 |

<details><summary>`callees` 피호출자 4건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | new_with_out_line | game_ai::SmallActionAroundBush::new_with_out_line | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush | game-ai\src\small_action\around.rs:1154 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 2 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 3 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 4개**: `_RNvNtNtCs9ec1k27omRZ_3std2io5stdio6__print`, `collect`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `from_iter`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m02.ll:21552, m08.ll:105285) · **형제 10개** (SmallActionAroundBush)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionAroundBush as std::clone::Clone>::clone | pub | game-ai\src\small_action\around.rs:1138 | True | fn(&game_ai::SmallActionAroundBush) -> game_ai::SmallActionAroundBush |
| 1 | <game_ai::SmallActionAroundBush as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\around.rs:1138 | True | fn(&game_ai::SmallActionAroundBush, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionAroundBush::new | pub | game-ai\src\small_action\around.rs:1150 | False | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionAroundBush |
| 3 | game_ai::SmallActionAroundBush::new_with_out_line | pub | game-ai\src\small_action\around.rs:1154 | False | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush |
| 4 | game_ai::SmallActionAroundBush::new_with_target | pub | game-ai\src\small_action\around.rs:1178 | False | fn(&game_core::OperationData, &game_core::Entity, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush |
| 5 | game_ai::SmallActionAroundBush::get_input | in:game_ai | game-ai\src\small_action\around.rs:1195 | False | fn(&mut game_ai::SmallActionAroundBush, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 6 | game_ai::SmallActionAroundBush::update_state | in:game_ai | game-ai\src\small_action\around.rs:1221 | False | fn(&mut game_ai::SmallActionAroundBush, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 7 | game_ai::SmallActionAroundBush::get_action | in:game_ai | game-ai\src\small_action\around.rs:1240 | True | fn(&game_ai::SmallActionAroundBush) -> game_core::SmallAction |
| 8 | game_ai::SmallActionAroundBush::merge | in:game_ai | game-ai\src\small_action\around.rs:1244 | False | fn(&mut game_ai::SmallActionAroundBush, game_ai::SmallActionAroundBush) |
| 9 | game_ai::SmallActionAroundBush::is_end | in:game_ai | game-ai\src\small_action\around.rs:1251 | False | fn(&game_ai::SmallActionAroundBush, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 재료 부재 | `player`(%3) 가 시그니처에 있으나 본문 사용 0건 — 소스에서 `_player` 인지, 향후 확장 자리인지 재료 부재(rmeta 에 소스 원문 없음) | 4 |  |
| 1 | 미탐색 | StdRng::gen_range::<u32> 가 320B 상태 중 어느 바이트를 갱신하는지 — rand 크레이트 경계(ChaCha12 블록 버퍼/인덱스). 이 배치 범위 밖, 미탐색 | 4 |  |
| 2 | 미탐색 | calls 의 `_RNvNtNtCs9ec1k27omRZ_3std2io5stdio6__print` = std::io::stdio::_print (print! 매크로). 망글 표기인 이유: qcspec 의 v0 파서가 `_` 접두 식별자(`6__print`)를 `__prin` 으로 잘라 짧은 이름으로는 C2 를 못 통과(도구 한계, 사실 왜곡 아님) | 4 |  |
| 3 | 미탐색 | @anon…100(unwrap_failed Location) = around.rs:1161 col 56 로 추정(바이트 `\89\04`=1161, `8`=56) — 패닉 위치 확인용, 판정과 무관 | 5 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

