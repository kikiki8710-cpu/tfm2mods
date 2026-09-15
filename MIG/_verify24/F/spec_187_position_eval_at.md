---

### `187` position_eval_at — 위치 평가(PositioningScore 56B)의 TLS 캐시 래퍼 — (id,x,y,purpose,version) 키를 512슬롯 해시로 조회, miss 면 position_eval_at_uncached 계산 후 삽입

| 항목 | 값 |
|---|---|
| id | `position_eval__position_eval_at` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai13position_eval16position_eval_at` |
| 소스 | `game-ai\src\position_eval.rs:291` |
| IR | `m07.ll` 24507~24711행 |
| 경로·가시성 | `game_ai::position_eval_at` · **pub** |
| 계층 | 기타 |
| exe | `d84db0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r15` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut PositioningScore(56B) | 반환 슬롯. hit 면 캐시 슬롯에서 memcpy 56B(24645), miss 면 uncached 의 sret %9 에서 memcpy 56B(24700) | 4 |
| 1 | 1 | version | usize | AI 버전. 이 함수에선 분기 없음 — 캐시 키의 5번째 성분(key+0x20)으로만 쓰임(24554). uncached 에 그대로 전달 | 4 |
| 2 | 2 | player | &PlayerState(2528B, readonly) | +0x928 info.id 만 읽음(24544) — 캐시 키 0번 성분. uncached 에 그대로 전달 | 4 |
| 3 | 3 | data | &OperationData(24B, readonly) | +0x0 cache → AbstractGameWithCache+0x0 game(&dyn AbstractGame 팻포인터: +0 data_ptr, +8 vtable_ptr) → vtable+0x20 seed() / +0x28 tick() 호출(24530~24541) | 4 |
| 4 | 4 | x | u64 | 평가 좌표 x(원단위). 캐시 키 1번 성분 + 해시 xor 항 | 4 |
| 5 | 5 | y | u64 | 평가 좌표 y. 캐시 키 2번 성분 + 해시 (y<<21) 항 | 4 |
| 6 | 6 | purpose | PositionEvalPurpose(1B, i8 로 전달) | 니치 인코딩 enum(General=2 … AttackStance=12, LineStyle 은 암묵 0/1=LineStyle(Aggressive/Defensive), 9 는 무효). 캐시 키 3번 성분 + pe_purpose_ord 로 해시 항 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn position_eval_at(version, player, data, x, y, purpose) -> PositioningScore {
  // L293~294: 게임 seed/tick (dyn AbstractGame vtable+0x20 / +0x28)
  let seed = data.cache.game.seed();
  let tick = data.cache.game.tick();
  // L295: 키 5-튜플
  let key = (player.info.id /*+0x928*/, x, y, purpose, version);
  // L296 = pe_slot_index(id, x, y, purpose) 인라인(L100~102)
  //   pe_purpose_ord(purpose) (L82~95): General 0 | RunAway 512 | Recall 1024 | Around 1536 | Positioning 2048 | Trace 2560 | Lane 3072 | LineStyle(Aggr) 4096 | LineStyle(Def) 4608 | LaneSafe 5120 | Objective 5632 | AttackStance 6144
  let h = x ^ (y << 21) ^ (id << 42) ^ ord;          // 저 21비트에 ord 가 겹치므로 xor==or(disjoint)
  let idx = h.wrapping_mul(0x9E3779B97F4A7C15) >> 55;   // 0..511
  // L298~307: 조회 (aux 클로저 #0)
  let cached: Option<PositioningScore> = POS_EVAL_CACHE.with(|c| {
    let mut c = c.borrow_mut();                          // borrow!=0 → panic_already_borrowed
    if c.seed != seed { c.seed = seed; for s in c.slots.iter_mut() { *s = None; } return None; }  // L300~303
    match &c.slots[idx] {                                // L305 (bounds check <512)
      Some(s) if s.tick == tick && s.key == key => Some(s.val),   // key: id,x,y 는 ==, purpose 는 discr 같고 LineStyle 이면 내부값도 같아야, version ==
      _ => None,
    }
  });
  if let Some(v) = cached {                              // L308: sret+49 != 2
    prof::phase_call(90);  // ENABLED 일 때 PHASE_CALLS[90] += 1 (텔레메트리)
    return v;                                            // memcpy 56
  }
  prof::phase_call(91);                                  // L312
  let v = position_eval_at_uncached(version, player, data, x, y, purpose);   // L314
  POS_EVAL_CACHE.with(|c| {                              // L316~321 (aux 클로저 #1)
    let mut c = c.borrow_mut();
    if c.seed == seed { c.slots[idx] = Some(PeSlot { tick, key, val: v }); }  // seed 다르면 저장 안 함
  });
  v                                                      // L323 memcpy 56
}
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x928 | info.id | r | 캐시 키 성분 0 + 해시 (id<<42) 항 (24544~24546) | 4 | OK |  |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (24530) | 4 | OK |  |
| 2 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 포인터 (24531) | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x20 = AbstractGame::seed, +0x28 = AbstractGame::tick (divtable · 24534~24541) | 3 | OK |  |
| 4 | RefCell<PosEvalCache> | 0x0 | borrow | r | aux: 0 이면 -1 로 잠금, 아니면 panic_already_borrowed (75411~75416) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 5 | RefCell<PosEvalCache> | 0x8 | slots(ptr) | r | aux: Box<[Option<PeSlot>;512]> (75419·75445) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 6 | RefCell<PosEvalCache> | 0x10 | seed | r | aux: game.seed() 와 비교 (75422~75426 · 75660~75664) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 7 | PeSlot | 0x0 | tick | r | aux: == game.tick() (75447·75481) | 4 | OK |  |
| 8 | PeSlot | 0x8 | key.0(id) | r | aux 75494~75503 | 4 | OK |  |
| 9 | PeSlot | 0x10 | key.1(x) | r | aux 75490~75506 | 4 | OK |  |
| 10 | PeSlot | 0x18 | key.2(y) | r | aux 75486~75510 | 4 | OK |  |
| 11 | PeSlot | 0x20 | key.3(purpose) | r | aux 75452~75453 · discr 정규화 후 비교(75519~75546) | 4 | OK |  |
| 12 | PeSlot | 0x28 | key.4(version) | r | aux 75455~75456 · 75540~75544 | 4 | OK |  |
| 13 | PeSlot | 0x30 | val | r | aux: hit 시 49B 복사(75588) | 4 | OK |  |
| 14 | PeSlot | 0x61 | val.on_periodic_trajectory = Option<PeSlot> 니치 태그 | r | aux: 2=None (75458~75460·75471) | 4 | OK |  |
| 15 | (sret) | 0x0 | PositioningScore 56B | w | 24645 / 24700 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | hit: 캐시 슬롯 val / miss: uncached 결과 |
| 16 | TLS RefCell<PosEvalCache> | 0x0 | borrow | w | aux 두 클로저 모두 진입 시 -1, 탈출 시 0 (75416·75584·75593 / 75654·75725) | 4 | 확인불가(tcx 사전에 타입 없음) | -1 → 0 |
| 17 | TLS PosEvalCache | 0x10 | seed | w | aux 조회 클로저 · seed 불일치 시만 (75440) | 4 | 확인불가(tcx 사전에 타입 없음) | game.seed() |
| 18 | TLS PosEvalCache.slots[0..512] | 0x61 | Option<PeSlot> 태그 | w | aux 조회 클로저 · seed 불일치 시 512 슬롯 전부 (75565~75572) | 4 | 확인불가(tcx 사전에 타입 없음) | 2 (None) |
| 19 | TLS PosEvalCache.slots[idx] | 0x0 | PeSlot 통째(tick 8B + key 40B + val 56B) | w | aux 삽입 클로저 · c.seed == game.seed() 일 때만 (75688~75692) | 4 | 확인불가(tcx 사전에 타입 없음) | Some(PeSlot{tick: game.tick(), key, val: uncached 결과}) |
| 20 | static game_core::prof::PHASE_CALLS | 0x2d0 | [90] (hit 카운터) | w | prof::ENABLED != 0 일 때만 — 텔레메트리 (24662) | 4 | 확인불가(tcx 사전에 타입 없음) | +1 atomic |
| 21 | static game_core::prof::PHASE_CALLS | 0x2d8 | [91] (miss 카운터) | w | prof::ENABLED != 0 일 때만 — 텔레메트리 (24682) | 4 | 확인불가(tcx 사전에 타입 없음) | +1 atomic |

**`consts` 상수 26건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 9 | 83 | 센티널 | PositionEvalPurpose 니치 무효값(assume ne 9) — 판정 아님, 인코딩 정합 (24562) | 4 |
| 1 | -2 | 83 | 태그 | 태그→논리 discr 변환 `purpose-2`(태그 2..=12 → 0..=10) (24564) | 4 |
| 2 | 1 | 83 | 센티널 | `purpose >u 1` 이면 니치 variant, 아니면 암묵 LineStyle(discr 7) (24565) | 4 |
| 3 | 7 | 83 | 태그 | untagged variant LineStyle 의 논리 discr (24566) | 4 |
| 4 | 512 | 85 | 산출값 | pe_purpose_ord: RunAway (= 1*512) — 슬롯 수 512 와 같은 수지만 여기선 ord 값 (24617) | 4 |
| 5 | 1024 | 86 | 산출값 | pe_purpose_ord: Recall | 4 |
| 6 | 1536 | 87 | 산출값 | pe_purpose_ord: Around | 4 |
| 7 | 2048 | 88 | 산출값 | pe_purpose_ord: Positioning | 4 |
| 8 | 2560 | 89 | 산출값 | pe_purpose_ord: Trace | 4 |
| 9 | 3072 | 90 | 산출값 | pe_purpose_ord: Lane | 4 |
| 10 | 4096 | 91 | 태그 | pe_purpose_ord: LineStyle(Aggressive=태그0) (24604 select) | 4 |
| 11 | 4608 | 91 | 태그 | pe_purpose_ord: LineStyle(Defensive=태그1) — 소스 91~92 줄 어느 쪽인지는 L0 로 소실 | 4 |
| 12 | 5120 | 93 | 산출값 | pe_purpose_ord: LaneSafe | 4 |
| 13 | 5632 | 94 | 산출값 | pe_purpose_ord: Objective | 4 |
| 14 | 6144 | 95 | 산출값 | pe_purpose_ord: AttackStance | 4 |
| 15 | 0 | 84 | 태그 | pe_purpose_ord: General (phi 기본 · 24617) | 4 |
| 16 | 21 | 101 | 계수 | 해시 y<<21 (24618) | 4 |
| 17 | 42 | 101 | 계수 | 해시 id<<42 (24619) | 4 |
| 18 | -7046029254386353131 | 102 | 계수 | 0x9E3779B97F4A7C15 피보나치 해시 승수 (wrapping_mul · 24625) | 4 |
| 19 | 55 | 102 | 인덱스 | lshr 55 → 상위 9비트 = 슬롯 인덱스 0..511 (24626) | 4 |
| 20 | 2 | 308 | 태그 | Option<PositioningScore> None 태그(sret+49 = on_periodic_trajectory 자리) (24641) | 4 |
| 21 | 720 | 309 | 미상 | prof PHASE_CALLS[90] 바이트 오프셋(90*8) — hit 텔레메트리 (24662) | 4 |
| 22 | 728 | 312 | 미상 | prof PHASE_CALLS[91] 바이트 오프셋 — miss 텔레메트리 (24682) | 4 |
| 23 | 512 | 305 | 산출값 | aux: 슬롯 배열 길이(bounds check ult 512 · 초기화 루프 상한) (75436·75571·75682) | 4 |
| 24 | -1 | 299 | 미상 | aux: RefCell borrow_mut 잠금값 (75416·75654) | 4 |
| 25 | -1 | 426 | 센티널 | aux: try_with 의 Option<Option<..>> 외곽 니치(태그 255) 검사 — 컴파일러 아티팩트, 유효 데이터에선 불발 (75594) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 캐시 슬롯 수 | position_eval.rs:305,319 (배열 [Option<PeSlot>;512] · pe_slot_index 의 >>55) | 512 | 늘리면 같은 틱 안 충돌(덮어쓰기)이 줄어 uncached 재계산이 준다 — 판정값 자체는 불변(캐시는 순수 메모) | 4 | 기존 |
| 1 | 해시 승수 | position_eval.rs:102 | -7046029254386353131 | 바꾸면 슬롯 분포만 바뀜 — 결과 불변 | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | pe_purpose_ord | game_ai::position_eval::pe_purpose_ord | in:game_ai::position_eval | fn(game_ai::PositionEvalPurpose) -> u64 | game-ai\src\position_eval.rs:82 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | pe_slot_index | game_ai::position_eval::pe_slot_index | in:game_ai::position_eval | fn(usize, u64, u64, game_ai::PositionEvalPurpose) -> usize | game-ai\src\position_eval.rs:100 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | position_eval_at | game_ai::position_eval_at | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:291 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | position_eval_at_uncached | game_ai::position_eval::position_eval_at_uncached | in:game_ai::position_eval | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:371 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | seed | game_core::AbstractGame::seed | pub | fn(&Self/#0) -> u64 | game-core\src\simulation.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | seed | <game_core::Game as game_core::AbstractGame>::seed | pub | fn(&game_core::Game) -> u64 | game-core\src\simulation\game.rs:1564 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | seed | <game_core::SingleLaneGame as game_core::AbstractGame>::seed | pub | fn(&game_core::SingleLaneGame) -> u64 | game-core\src\simulation\game.rs:3781 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 9 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 8개**: `borrow_mut`, `panic_access_error`, `panic_already_borrowed`, `phase_call`, `with  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 2개는 **전부 다른 함수**라 싣지 않는다`, `wrapping_mul`, `{closure#0}, Option<PositioningScore>>`, `{closure#1},`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 6곳** (m05.ll:35372, m05.ll:35983, m07.ll:24741, m07.ll:34263, m07.ll:37813, m11.ll:53313) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | pe_purpose_ord 의 값이 소스에서 `idx*512` 인지 `ord<<9` 인지 — IR 은 상수 phi 로 접혀 있어 표기 불가(동작은 확정: 위 표) | 4 |  |
| 1 | 미탐색 | LineStyle(Aggressive)=4096 / LineStyle(Defensive)=4608 이 소스 91·92 어느 줄인지 — 24604 select 가 `;L0` 로 줄 소실. 값 대응(태그0→4096, 태그1→4608 · trunc nuw i8→i1)은 확정 | 4 |  |
| 2 | 미탐색 | 다른 함수가 POS_EVAL_CACHE 를 읽거나 쓰는지 — 이 명세 범위(본체+두 클로저) 밖. 미탐색(grep …PosEvalCacheEE4with… 를 전 IR 에 돌리면 됨) | 4 |  |
| 3 | 미탐색 | 지시문이 말한 EPC_CACHE(EpcCache) 는 이 함수가 안 만짐 — entity_positioning_cache_cached 의 캐시로 보이나 그 함수는 담당 밖(미탐색) | 4 |  |
| 4 | 미탐색 | position_eval_at_uncached(0xd851d0 · internal fastcc · 7인자 동일 배치) 내부 — 계약만: (sret 56B, version, &PlayerState, &OperationData, x, y, purpose) → PositioningScore (24687) | 4 |  |
| 5 | 미탐색 | 패딩 +0x32..+0x38 6B 의 실제 값 — uncached 가 무엇을 남기는지에 달림. 런타임 대조는 이 6B 마스크 필요 | 4 |  |
| 6 | 미탐색 | prof::ENABLED / PHASE_CALLS 는 game_core 정적 — ENABLED 가 런타임에 켜지는 조건은 안 봄(판정 무관 텔레메트리) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

