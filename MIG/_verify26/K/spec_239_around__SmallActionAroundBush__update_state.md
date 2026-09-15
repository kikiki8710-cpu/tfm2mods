---

### `239` around::SmallActionAroundBush::update_state — 부쉬 주변대기 상태 갱신: 목표점 16000 이내 도달 && change_tick 경과면 같은 부쉬(id) 셀 71 후보 중 무작위 셀로 목표를 옮기고 change_tick = tick + rand(60..=120); debug 면 선 그리기

| 항목 | 값 |
|---|---|
| id | `around__SmallActionAroundBush__update_state` |
| 심볼 | `_RNvMs6_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_21SmallActionAroundBush12update_state` |
| 소스 | `game-ai\src\small_action\around.rs:1221` |
| IR | `m08.ll` 104740~104914행 |
| 경로·가시성 | `game_ai::SmallActionAroundBush::update_state` · **in:game_ai** |
| 계층 | 기타 |
| exe | `dc2330` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionAroundBush, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut SmallActionAroundBush(120B) (%0) | DI self = %0 | 4 |
| 1 | 2 | rnd | &mut StdRng(320B) (%1) | DI rnd = %1 | 4 |
| 2 | 3 | player | &PlayerState(2528B) (%2) |  | 4 |
| 3 | 4 | data | &OperationData(24B) (%3) |  | 4 |
| 4 | 5 | debug | &mut DebugFrameData(224B) (%4) | DI debug = %4 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn update_state(&mut self, rnd, player, data, debug)   // around.rs:1221~1238
  // L1222
  champ = data.cache.player_champion[player.info.team][player.info.position.as_index()].unwrap()
  // L1224 — 도달 && 교체 시각 경과 (단락: 거리 먼저, tick() 가상호출은 거리 조건 참일 때만)
  if champ.distance_sq((self.target_x, self.target_y)) < 256000001 && game.tick() >= self.change_tick {
      // L1225 — 71 개 고정 셀 후보(@anon…94) 중 self.bush 와 같은 부쉬 id 인 셀만
      const CANDIDATES: [(usize,usize);71] = [(0,0),(1,0),(2,0),(19,0),(20,0),(21,0),(0,1),(0,2),(7,4),(8,4),(9,4),(20,4),(20,5),(12,6),(4,7),(4,8),(29,8),(4,9),(14,9),(29,9),(29,10),(26,11),(6,12),(12,12),(13,12),(26,12),(12,13),(26,13),(9,14),(26,14),(20,15),(21,15),(17,16),(21,16),(16,17),(17,17),(25,18),(0,19),(25,19),(0,20),(4,20),(5,20),(15,20),(25,20),(0,21),(15,21),(16,21),(25,21),(25,22),(29,24),(18,25),(19,25),(20,25),(21,25),(22,25),(29,25),(11,26),(12,26),(13,26),(14,26),(29,26),(28,28),(29,28),(8,29),(9,29),(10,29),(24,29),(25,29),(26,29),(28,29),(29,29)]
      candidates = CANDIDATES.into_iter().filter(|(x, y)| data.context.map.bushes[*y][*x] == self.bush)   // aux call_mut: bushes = MapDef+0x1c98 [[usize;30];30] · 튜플 .0=x(+0) .1=y(+8)
      // L1226
      (cx, cy) = candidates.choose(rnd).unwrap()          // aux choose: reservoir — 매칭 원소 k 번째마다 gen_range(0..k)==0 이면 교체 · 매칭 0개 → None → unwrap_failed 패닉
      // L1227~1228
      target_x = cx*32000 + 16000; target_y = cy*32000 + 16000
      // L1230~1231
      self.target_x = target_x; self.target_y = target_y
      // L1232
      self.change_tick = game.tick() + rnd.gen_range(60..=120)   // RangeInclusive{60,120,exhausted=false} · tick 재호출
  }
  // L1235
  if data.context.debug {
      // L1236
      debug.add_line(champ.x, champ.y, self.target_x, self.target_y, &[1.0,1.0,0.0,1.0])   // 갱신 후 값(재선정됐으면 새 목표)
  }
  // L1238

★rnd 순서(재선정 분기): [choose: 매칭 후보 수 N 만큼 gen_range(0..1), gen_range(0..2), …, gen_range(0..N) (u32 경로)] → gen_range(60..=120). N 은 self.bush 에 속한 후보 셀 수(맵 데이터 소관).
★self.bush 가 후보 71 셀 어느 것과도 안 맞으면 choose=None → unwrap 패닉(생성자가 bush 를 후보 안 셀에서 잡는다는 전제).
```

**`mem` 메모리 접근 20건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | SmallActionAroundBush | 0x8 | change_tick | r | L1224 (gep 8) · tick < change_tick 이면 재선정 건너뜀 | 4 | OK |  |
| 1 | SmallActionAroundBush | 0x10 | bush | r | L1225 (gep 16) · 술어 클로저가 &self.bush 를 캡처(aux call_mut 에서 load 후 bushes[y][x] 와 eq) | 4 | OK |  |
| 2 | SmallActionAroundBush | 0x18 | target_x | r | L1224 (gep 24) x2 · L1236 add_line | 4 | OK |  |
| 3 | SmallActionAroundBush | 0x20 | target_y | r | L1224 (gep 32) y2 · L1236 | 4 | OK |  |
| 4 | PlayerState | 0x930 | info.team | r | L1222 (gep 2352) ult 2 bounds | 4 | OK |  |
| 5 | PlayerState | 0x9c0 | info.position | r | L1222 (gep 2496) → as_index | 4 | OK |  |
| 6 | OperationData | 0x0 | cache | r | L1222/L1224/L1232 | 4 | OK |  |
| 7 | OperationData | 0x8 | context | r | L1225 map · L1235 debug | 4 | OK |  |
| 8 | GameContext | 0x20 | map | r | L1225 (gep 32) &MapDef → 술어 캡처 | 4 | OK |  |
| 9 | GameContext | 0x3b | debug | r | L1235 (gep 59) bool — true 면 add_line | 4 | OK |  |
| 10 | MapDef | 0x1c98 | bushes[y][x] | r | aux call_mut (gep 7320) [[usize;30];30] · y,x 각각 ult 30 bounds 패닉 · == self.bush | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x0 | game.data_ptr | r | tick 호출 | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | +0x28 tick (L1224 · L1232 두 번 호출) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | L1222 (gep 480) · null → unwrap_failed | 4 | OK |  |
| 14 | Entity | 0x660 | x (champ) | r | L1224 distance_sq (gep 1632) · L1236 add_line 시점 | 4 | OK |  |
| 15 | Entity | 0x668 | y (champ) | r | L1224 (gep 1640) · L1236 | 4 | OK |  |
| 16 | SmallActionAroundBush | 0x18 | target_x | w | L1230 · m08.ll:104877 · 재선정 분기만 | 4 | OK | cx*32000+16000 (cx = 선택 셀 .0) |
| 17 | SmallActionAroundBush | 0x20 | target_y | w | L1231 · m08.ll:104878 · 재선정 분기만 | 4 | OK | cy*32000+16000 (cy = 선택 셀 .1) |
| 18 | SmallActionAroundBush | 0x8 | change_tick | w | L1232 · m08.ll:104889 · 재선정 분기만 · tick 은 L1224 와 별도로 재호출(%83) | 4 | OK | game.tick() + rnd.gen_range(60..=120) |
| 19 | DebugFrameData | (콜리) | add_line(champ.x, champ.y, self.target_x, self.target_y, color(1,1,0,1)) | w | L1236 · context.debug 일 때만 · game_core 경계(frame.rs) — 본문 직접 store 0 | 4 | 확인불가(오프셋 파싱 실패) | 노란 선 |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 256000001 | 1224 | 임계 | 16000² + 1 — dist_sq ult ⟹ 챔프↔target 거리 ≤ 16000(반 셀) 이면 '도달' → 재선정 후보 | 4 |
| 1 | 71 | 1225 | 길이 | 후보 셀 상수 배열 [(usize,usize);71] 길이(@anon…94 · 1136B = 71×16). 내용 = 부쉬 셀 좌표 목록(아래 logic) | 4 |
| 2 | 32000 | 1227 | 계수 | 셀 → 월드 좌표 변환(셀 폭) | 4 |
| 3 | 16000 | 1227 | 계수 | 셀 중심 오프셋(반 셀) | 4 |
| 4 | 60 | 1232 | 산출값 | gen_range(60..=120) 하한 — 다음 목표 변경까지 최소 60틱(1초@60tps) | 4 |
| 5 | 120 | 1232 | 산출값 | gen_range(60..=120) 상한 — 최대 120틱(2초) | 4 |
| 6 | 30 | 1225 | 미상 | (aux) bushes 격자 30×30 bounds — 후보는 전부 0..29 라 패닉 도달 불가 | 4 |
| 7 | 2 | 1222 | 임계 | player_champion 팀 축 bounds | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 목표 도달 반경(제곱) | around.rs:1224 · m08.ll:104810 | 256000001 | 올리면 목표점에 덜 붙어도 다음 셀 재선정 → 부쉬 안 배회가 잦아짐 · 내리면 정확히 붙어야 재선정 | 4 | 기존 |
| 1 | 재선정 대기 틱 범위 | around.rs:1232 · m08.ll:104881/104883 | 60..=120 | 넓히면 한 지점에 더 오래 서 있음(잠복 시간↑) · 좁히면 자주 움직임 | 4 | 기존 |
| 2 | 후보 셀 목록 | around.rs:1225 · m08.ll:103 (@anon…94) | [(usize,usize);71] | 부쉬 셀 좌표 하드코딩 — 맵 bushes 격자와 어긋나면 그 부쉬는 후보 0 → unwrap 패닉 위험. 셀을 추가/삭제하면 잠복 지점 분포가 바뀜 | 4 | 기존 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_line | game_core::DebugFrameData::add_line | pub | fn(&mut game_core::DebugFrameData, u64, u64, u64, u64, common::color::Color) | game-core\src\simulation\game\frame.rs:67 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 6 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 7 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | update_state | game_ai::SmallActionAroundBush::update_state | in:game_ai | fn(&mut game_ai::SmallActionAroundBush, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\small_action\around.rs:1221 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 1개**: `gen_range`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:41313) · **형제 10개** (SmallActionAroundBush)

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

**`open` 2건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 71 후보 셀이 맵 bushes 격자의 부쉬 셀 전체와 일치하는지(하드코딩 목록 vs 맵 데이터) — MapDef 인스턴스 값은 IR 밖(런타임/에셋) | 4 |  |
| 1 | 미탐색 | L1224 `&&` 좌우 순서는 분기 구조(거리 → tick 가상호출)로 확정 · 문면 확인은 column 부재 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | rand::Rng::gen_range(60..=120) 및 choose 내부 gen_range(0..k) 의 실제 RNG 워드 소비 수(rejection 루프) — rand 크레이트 본체(gen_range 정의 m*.ll 별도) · 사이트 수와 순서만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

