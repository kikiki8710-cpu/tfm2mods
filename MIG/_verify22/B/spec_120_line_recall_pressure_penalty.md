---

### `120` line_recall_pressure_penalty — 귀환 압박 페널티 — 처벌피해를 맞은 뒤 남는 HP% 가 45%/25% 미만이거나 처벌피해가 최대HP의 11% 초과면 hp_value 비례 페널티(i64) 산출

| 항목 | 값 |
|---|---|
| id | `lane_economy__line_recall_pressure_penalty` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai12lane_economy28line_recall_pressure_penalty` |
| 소스 | `game-ai\src\lane_economy.rs:238` |
| IR | `m11.ll` 51097~51193행 |
| 경로·가시성 | `game_ai::lane_economy::line_recall_pressure_penalty` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e28060` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::Entity, i64, i64) -> i64
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | champ | &Entity(1728B) | 읽기 전용. undying / stat_cached.hp / hp 3필드만 읽음 | 4 |
| 1 | 2 | punish_damage | i64 | 귀환 시 맞을 것으로 보는 처벌 피해량. 부호 있는 산술(sub/mul/sdiv)로만 쓰임 | 4 |
| 2 | 3 | hp_value | i64 | HP 1 단위의 가치(통화). 페널티 스케일 인자 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn line_recall_pressure_penalty(champ:&Entity, punish_damage:i64(≥1), hp_value:i64) -> i64
  if champ.stat_buff_cached.undying { return 0 }                         // L239  (@0x488)
  let max_hp = max(champ.stat_cached.hp, 1)                                // L242  (@0x628, umax)
  let hp_after = max(champ.hp as i64 - punish_damage, 0)                   // L243  (@0x670, smax)
  let hp_after_pct = hp_after*100 / max_hp                                 // L244  (sdiv; -1/MIN 오버플로 패닉 가드는 컴파일러 삽입)
  let mut penalty = 0
  if hp_after_pct < 45 {                                                   // L246
     penalty = (45 - hp_after_pct) * hp_value / 60                         // L247
     if hp_after_pct < 25 { penalty += hp_value / 5 }                      // L249~250 (45 미만 블록 안에서만)
  }
  if punish_damage*100 / max_hp > 11 {                                     // L252
     penalty += (hp_value * punish_damage / max(champ.hp,1)) / 2           // L253 (분모는 max_hp 가 아니라 현재 hp!)
  }
  return penalty                                                           // L256
분기 극성: 51104 br %6(undying) → true=%51(ret 0) / 51131 br %21(pct<45) → true=%23 / 51143 br %27(pct<25) → true=%33 / 51162 br %38(ratio>11) → true=%40. 세 조건은 독립 가산(첫 undying 만 조기 종료). 세 panic_const_div_overflow 블록(51134·51165·51186)은 i64::MIN/-1 가드로 도달 불가(max_hp≥1, hp 는 usize).
```

**`mem` 메모리 접근 3건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Entity | 0x488 | stat_buff_cached.undying | r | bool(i8 trunc → i1). true 면 즉시 0 반환 (m11.ll:51101~51104, lane_economy.rs:239) | 4 | OK |
| 1 | Entity | 0x628 | stat_cached.hp | r | usize 최대HP. umax(.,1) 로 0 나눗셈 방지 → max_hp (m11.ll:51107~51111, L242) | 4 | OK |
| 2 | Entity | 0x670 | hp | r | usize 현재HP. hp - punish_damage 를 smax 0 → hp_after (m11.ll:51113~51118, L243). L253 에서 max(hp,1) 로 한 번 더 읽음(같은 로드 %12 재사용, m11.ll:51178) | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 100 | 244 | 계수 | 퍼센트 스케일 — hp_after*100/max_hp = 처벌 후 HP%. L252 에서도 punish_damage*100/max_hp 에 재사용(m11.ll:51120, 51148) | 4 |
| 1 | 45 | 246 | 임계 | 1차 임계 HP%. hp_after_pct < 45 이면 페널티 발생. L247 에서 (45 - hp_after_pct) 의 피감수로도 쓰임(m11.ll:51130, 51138) | 4 |
| 2 | 60 | 247 | 계수 | 1차 페널티 분모 — penalty = (45-hp_after_pct)*hp_value/60 (m11.ll:51140) | 4 |
| 3 | 25 | 249 | 임계 | 2차(치명) 임계 HP%. hp_after_pct < 25 이면 추가 페널티. 45 미만 블록 안에서만 검사(m11.ll:51142) | 4 |
| 4 | 5 | 250 | 계수 | 치명 추가 페널티 = hp_value/5 (sdiv, m11.ll:51154) | 4 |
| 5 | 11 | 252 | 임계 | 처벌피해 비율 임계 — punish_damage*100/max_hp > 11 (즉 ≥12%) 이면 추가 페널티(m11.ll:51161) | 4 |
| 6 | 2 | 253 | 계수 | 추가 페널티 = (hp_value*punish_damage / max(hp,1)) / 2 — 절반(sdiv 2, m11.ll:51180). 비교 임계와 무관한 분모 | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 처벌 후 HP% 1차 임계 | lane_economy.rs:246 (m11.ll:51130) | 45 | 올리면 더 높은 HP 에서도 귀환 페널티가 붙어 귀환을 더 꺼리게(라인 압박 아래 귀환 억제). L247 의 (45-pct) 기울기 기준점도 같이 이동 | 4 | 기존 |
| 1 | 1차 페널티 분모 | lane_economy.rs:247 (m11.ll:51140) | 60 | 내리면 HP% 부족 1%당 페널티가 커진다(hp_value/60 per %) | 4 | 기존 |
| 2 | 치명 HP% 임계 | lane_economy.rs:249 (m11.ll:51142) | 25 | 올리면 hp_value/5 정액 추가 페널티가 더 넓은 HP 구간에서 붙는다 | 4 | 기존 |
| 3 | 치명 정액 페널티 분모 | lane_economy.rs:250 (m11.ll:51154) | 5 | 내리면 치명 구간 정액 페널티(hp_value/5) 증가 | 4 | 기존 |
| 4 | 처벌피해/최대HP 비율 임계(%) | lane_economy.rs:252 (m11.ll:51161) | 11 | 올리면 큰 처벌피해에서만 비례 페널티가 붙는다(>11 이므로 12% 이상) | 4 | 기존 |
| 5 | 비례 페널티 반감 분모 | lane_economy.rs:253 (m11.ll:51180) | 2 | 내리면(1) 비례 페널티 2배 | 4 | 기존 |

<details><summary>`callees` 피호출자 1건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | line_recall_pressure_penalty | game_ai::lane_economy::line_recall_pressure_penalty | in:game_ai | fn(&game_core::Entity, i64, i64) -> i64 | game-ai\src\lane_economy.rs:238 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `llvm.smax.i64`, `llvm.umax.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:51683) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 호출자(1곳) 및 penalty 의 소비 방향(감산인지 가산인지)은 본 함수 범위 밖 — 미독(범위: m11.ll 51097~51193 만). '귀환 페널티' 해석은 이름·부호(양수 누적)에 근거한 추정 | 5 |  |
| 1 | 미탐색 | punish_damage 의 range(≥1) 는 호출자 유래 힌트 — 어떤 값(타워/미니언 기대피해 등)인지 미확인 | 4 |  |
| 2 | 표기 불가 | hp_after_pct/ratio 의 == 경계(<45 vs <=44 등)는 외연 동일이라 표기 불가(IR 은 slt 45 / slt 25 / sgt 11) | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L253 분모가 max_hp(stat_cached.hp) 가 아닌 현재 hp(%12=Entity+0x670) 인 것은 IR 확정(m11.ll:51172~51179) — 소스 의도(버그 vs 의도)는 판단 불가 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

