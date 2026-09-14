---

### `140` SmallActionAroundBush::new_with_target — 수풀 id 에 속한 하드코딩 71셀 중 대상 엔티티에 가장 가까운 셀을 골라 SmallActionAroundBush 를 생성

| 항목 | 값 |
|---|---|
| id | `SmallActionAroundBush__new_with_target` |
| 심볼 | `_RNvMs6_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_21SmallActionAroundBush15new_with_target` |
| 소스 | `game-ai\src\small_action\around.rs:1178` |
| IR | `m08.ll` 104917~105128행 |
| 경로·가시성 | `game_ai::SmallActionAroundBush::new_with_target` · **pub** |
| 계층 | 기타 |
| exe | `dc2550` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::OperationData, &game_core::Entity, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut SmallActionAroundBush(120B) | writes 참조. live 바이트만 기록됨 | 4 |
| 1 | 1 | data | &OperationData(24B, readonly) | +0x0 cache(&AbstractGameWithCache → +0x0 game &dyn AbstractGame 팻포인터: data_ptr +0 / vtable_ptr +8) · +0x8 context → map | 4 |
| 2 | 2 | target | &Entity(1728B, readonly) | +0x660 x · +0x668 y 만 읽음(min_by_key 키). 소유권 아님(참조) | 4 |
| 3 | 3 | bush | usize | 수풀 id. 필터 술어에서 map.bushes[y][x] 와 비교, 그대로 self.bush 에 저장 | 4 |
| 4 | 4 | out_line | AroundBushOutlineType(i8 0..3) | 그대로 self.out_line(+0x70) 에 저장. 분기 없음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
SmallActionAroundBush::new_with_target(data, target, bush, out_line) -> Self   [around.rs:1178]
 map = data.context.map
 candidates: Vec<(usize,usize)> = BUSH_CELLS(71개 하드코딩 (x,y), @anon…94)
      .into_iter().filter(|(x,y)| map.bushes[*y][*x] == bush).collect()        (L1180; 술어 = aux m08 113374, y<30·x<30 아니면 panic)
 best = candidates.iter().min_by_key(|(x,y)| {                                (L1181~1182, fold = aux m12 32541)
      dx = |target.x - (x*32000+16000)| ; dy = |target.y - (y*32000+16000)| ; dx*dx + dy*dy })
   — 동률이면 앞 원소 유지(compare<=Equal → 기존 유지, 32663 `icmp slt %49, 1`)
   — candidates 가 비면 first=None → `unwrap_failed` 패닉(L1182, 105081). 즉 bush id 에 해당하는 셀이 표에 없으면 패닉
 self = SmallActionAroundBush {                                                  (L1184)
   start_tick:  data.game.tick(),          (L1185, vtable+0x28)
   change_tick: data.game.tick(),          (L1186, 재호출 — 같은 틱)
   bush,
   target_x: best.x*32000+16000,           (L1188)
   target_y: best.y*32000+16000,           (L1189)
   path_finder: None,                      (+0x6d 니치 2)
   out_line }
 drop(candidates)  (L1193)

분기·판정은 없다(생성자). 유일한 실패 경로 = 후보 0개 → 패닉.
```

**`mem` 메모리 접근 15건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | 105-? IR 104937 (`gep %1, 8`) | 4 | OK |  |
| 1 | GameContext | 0x20 | map | r | &MapDef — 필터 클로저 환경 [0] (104939) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (105071 `load ptr, ptr %1`) | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 팻포인터 데이터 (105072 `load ptr, ptr %59`) | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 105074~105076; vtable+0x28 = AbstractGame::tick (divtable) — 2회 호출(L1185·L1186) | 3 | OK |  |
| 5 | Entity | 0x660 | x | r | target.x — 본체 첫 원소(105020) + fold 조각(m12 32616) | 4 | OK |  |
| 6 | Entity | 0x668 | y | r | target.y (105024 / m12 32620) | 4 | OK |  |
| 7 | MapDef | 0x1c98 | bushes[30][30] | r | aux call_mut 심 113407 `gep %4, 7320` → bushes[y][x] == bush | 4 | OK |  |
| 8 | SmallActionAroundBush(sret) | 0x0 | start_tick | w | 105099 | 4 | OK | game.tick() (1회차 vtable 호출 105077, L1185) |
| 9 | SmallActionAroundBush(sret) | 0x8 | change_tick | w | 105101 | 4 | OK | game.tick() (2회차 호출 105088, L1186 — 같은 값) |
| 10 | SmallActionAroundBush(sret) | 0x10 | bush | w | 105104 | 4 | OK | 인자 bush 그대로 |
| 11 | SmallActionAroundBush(sret) | 0x18 | target_x | w | 105106 | 4 | OK | best.x*32000+16000 (선택 셀 중심, L1188) |
| 12 | SmallActionAroundBush(sret) | 0x20 | target_y | w | 105108 | 4 | OK | best.y*32000+16000 (L1189) |
| 13 | SmallActionAroundBush(sret) | 0x6d | path_finder@tag | w | 105110 `store i8 2` | 4 | OK | 2 = None (Option<PathFinder> 니치, bool unconstrained_fallback 자리) |
| 14 | SmallActionAroundBush(sret) | 0x70 | out_line | w | 105112 | 4 | OK | 인자 out_line 그대로 |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 71 | 1180 | 산출값 | 하드코딩 후보 셀 표의 원소 수 — [(usize,usize);71] IntoIter 끝 인덱스 (104945, aux m06 44128). 표 내용은 @anon…94 (1136B=71×16): (0,0)(1,0)(2,0)(19,0)(20,0)(21,0)(0,1)(0,2)(7,4)(8,4)(9,4)(20,4)(20,5)(12,6)(4,7)(4,8)(29,8)(4,9)(14,9)(29,9)(29,10)(26,11)(6,12)(12,12)(13,12)(26,12)(12,13)(26,13)(9,14)(26,14)(20,15)(21,15)(17,16)(21,16)(16,17)(17,17)(25,18)(0,19)(25,19)(0,20)(4,20)(5,20)(15,20)(25,20)(0,21)(15,21)(16,21)(25,21)(25,22)(29,24)(18,25)(19,25)(20,25)(21,25)(22,25)(29,25)(11,26)(12,26)(13,26)(14,26)(29,26)(28,28)(29,28)(8,29)(9,29)(10,29)(24,29)(25,29)(26,29)(28,29)(29,29) — (x,y) 순 | 4 |
| 1 | 32000 | 1182 | 계수 | 셀→월드 좌표(셀 크기). 키 계산과 최종 target_x/y 저장 모두 | 4 |
| 2 | 16000 | 1182 | 계수 | 셀 중심 오프셋 | 4 |
| 3 | 2 | 1184 | 센티널 | Option<PathFinder>::None 의 니치 태그(105110). 판정값 아님(구조체 초기화) | 4 |
| 4 | 4 | 1180 | 미상 | aux from_iter: std Vec 초기 cap 4(원소 16B) — RawVecInner::try_allocate_in(4,false,8,16). 판정값 아님. ⚠본체 104968 `shl i64 %21, 4` 의 4 는 별개(len×16 = 원소 stride 접힘, 등록 대상 아님) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 수풀 후보 셀 표(71개 하드코딩) | around.rs:1180 (@anon…94, m08.ll:103) | 71 (x,y) | 표에 없는 수풀 셀은 절대 후보가 안 된다 — 맵 수풀 배치를 바꾸면 이 표도 바꿔야 하며, 어긋나면 unwrap 패닉(후보 0개) 위험 | 4 | 기존 |
| 1 | 셀 선택 기준 | around.rs:1181~1182 | 대상과의 제곱거리 최소 | 키를 바꾸면(예: 자기 위치 기준) 같은 수풀 안에서 다른 셀을 목표로 잡는다 | 4 | 기존 |

<details><summary>`callees` 피호출자 4건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | new_with_target | game_ai::SmallActionAroundBush::new_with_target | pub | fn(&game_core::OperationData, &game_core::Entity, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush | game-ai\src\small_action\around.rs:1178 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 2 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 3 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 5개**: `collect`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `from_iter`, `handle_error`, `try_allocate_in`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m02.ll:21543, m02.ll:27158) · **형제 10개** (SmallActionAroundBush)

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

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | change_tick 이 소스에서 `tick()` 재호출인지 `start_tick` 복사인지 — IR 은 vtable 슬롯을 2회 호출(105077·105088)하므로 재호출 확정. 같은 틱이라 값은 동일(순수 관측 함수 가정 — AbstractGame::tick 내부는 game_core 경계, 미탐색) | 4 |  |
| 1 | 재료 부재 | 71셀 표가 어느 맵 기준인지(맵 1종 가정) — IR 엔 맵 id 분기가 없어 모든 맵에 같은 표 적용. 소스 상수명은 재료 부재(rmeta 에 소스 원문 없음) | 4 |  |
| 2 | 미탐색 | AbstractGame::tick 의 정확한 콜리: divtable 이 `ExpectedGame::AbstractGame::tick` 로 슬롯 0x28 을 준다(정적 vtable 기준). 런타임 dyn 객체가 ExpectedGame 이 아닐 수 있으나 슬롯 의미(tick)는 트레이트 공통 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

