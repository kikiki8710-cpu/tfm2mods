---

### `152` line_champion_trade_allowance — 라인 챔피언 대상 공격/스킬의 '교환 허용치'(allowance) — 예상피해·near_enemies 위험치·대상 hp 가치로 환산한 i64 점수

| 항목 | 값 |
|---|---|
| id | `lane_economy__line_champion_trade_allowance` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai12lane_economy29line_champion_trade_allowance` |
| 소스 | `game-ai\src\lane_economy.rs:221` |
| IR | `m11.ll` 51196~51337행 |
| 경로·가시성 | `game_ai::lane_economy::line_champion_trade_allowance` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e281f0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, &game_core::Effect) -> i64
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | data | &OperationData(24B) | +0x8 context 만 읽음(expected_damage_target 인자). 51204~51205 | 4 |
| 1 | 2 | parameter | &ScoreParameter(5384B) | near_enemies(bumpalo Vec<ChampionScoreParameter>, ptr +0x14d8 · len +0x14f0 · 원소 216B) 순회. 51210~51223 | 4 |
| 2 | 3 | champ | &Entity(1728B) | expected_damage_target 의 caster(&dyn, vtable=@anon.…29) 로만 전달. 본문 직접 필드 읽기 없음 | 4 |
| 3 | 4 | target | &Entity(1728B) | +0x5c0 id(near_enemies 매칭 키) · +0x670 hp(분모·비교) | 4 |
| 4 | 5 | effect | &Effect(56B) | expected_damage_target 의 self 로만 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L222: expected_damage = effect.expected_damage_target(data.context, champ as &dyn(vtable @anon.29), target)  (51206)
L223: p = parameter.near_enemies.iter().find(|p| p.id == target.id)   (루프 51233~51251 · 첫 일치 원소)
  ── 못 찾음 (51317, L224):
     allowance = expected_damage * 100 / target.hp.max(1)   (sdiv · 51330)  → return (L236)
  ── 찾음 (51253~, L226~L233):
     hp_value     = champion_hp_value(data, parameter, p).max(1)                       (L226)
     total_damage = p.risk_damage/2 + expected_damage + p.possible_risk(data, 75)/4    (L227~228 · 덧셈 순서 = (risk/2 + expected) + risk75/4)
     allowance    = total_damage * hp_value / target.hp.max(1)                          (L229 · sdiv)
     if expected_damage >= target.hp  (51283 slt 의 false 방향, L230):
         allowance += hp_value.min(100)                                                 (L231)
     else if total_damage >= target.hp (51291 slt 의 false 방향, L232):
         allowance += hp_value.min(100) * 3 / 4                                         (L233)
     (else 그대로)
L236: return allowance
※ 나눗셈은 i64 sdiv, 분모는 usize umax(…,1) — 분모 -1 & 분자 i64::MIN 조합만 panic_const_div_overflow 가드(51286/51334, 도달 불가 방어)
```

**`mem` 메모리 접근 7건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | 51204 — expected_damage_target 의 ctx 인자 | 4 | OK |
| 1 | ScoreParameter | 0x14d8 | near_enemies.buf.ptr | r | 51210 (gep 5336) | 4 | OK |
| 2 | ScoreParameter | 0x14f0 | near_enemies.len | r | 51213 (gep 5360) · 끝 포인터 = ptr + len*216 (51223, 원소 216B 는 getelementptr 타입에서 · stride 216 = 51244 gep) | 4 | OK |
| 3 | ChampionScoreParameter | 0x58 | id | r | 51248 — target.id 와 비교(find 술어) | 4 | OK |
| 4 | ChampionScoreParameter | 0x80 | risk_damage | r | 51260 — /2 로 total_damage 에 합산 | 4 | OK |
| 5 | Entity | 0x5c0 | id | r | 51229 target.id (루프 전 1회 로드) | 4 | OK |
| 6 | Entity | 0x670 | hp | r | 51270 / 51319 target.hp | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1 | 226 | 인덱스 | hp_value = champion_hp_value(..).max(1) (smax · 51258) · 분모 target.hp.max(1) (umax · 51280/51329) | 4 |  |
| 1 | 2 | 227 | 계수 | risk_damage / 2 (sdiv · 51262) | 4 |  |
| 2 | 75 | 227 | 미상 | possible_risk(p, data, 75) 의 셋째 인자(확률/비율 파라미터 · 51263) — 내부 의미는 possible_risk 명세 소관 | 4 |  |
| 3 | 4 | 227 | 계수 | possible_risk(..)/4 (sdiv · 51264) | 4 |  |
| 4 | 100 | 231 | 임계 | hp_value.min(100) 상한 (umin · 51297/51305) · 못 찾은 경로의 expected_damage*100 (51318) | 4 |  |
| 5 | 3 | 233 | 미상 | min(hp_value,100)*3/4 — 곱 3 (51306) · /4 는 lshr 2 로 접힘(51307) | 4 |  |
| 6 | 2 | 233 | 계수 | lshr 2 = /4 (51307). min(hp_value,100)*3/4 의 나눗셈 | 4 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | possible_risk 비율 인자 | lane_economy.rs:227 | 75 | possible_risk 의 위험 추정 파라미터. 올리면 total_damage 가 커져 allowance 증가(대상이 더 죽을 것 같다고 봄) | 5 | 기존 |
| 1 | 위험치 축소 계수 /2·/4 | lane_economy.rs:227 | 2, 4 | risk_damage/2 · possible_risk/4. 분모를 줄이면 총피해 추정↑ → allowance↑ → 공격 경제 페널티 감소 | 5 | 기존 |
| 2 | 킬 확정 보너스 상한 | lane_economy.rs:231 | 100 | expected_damage 만으로 죽는 대상이면 hp_value.min(100) 가산. 올리면 고가치 대상 킬 시 보너스 상승 | 4 | 기존 |
| 3 | 총피해 킬 보너스 비율 | lane_economy.rs:233 | 3/4 | total_damage 로 죽을 대상이면 min(hp_value,100)*3/4 가산 | 4 | 기존 |
| 4 | 미매칭 폴백 배율 | lane_economy.rs:224 | 100 | near_enemies 에 없는 대상은 expected_damage*100/hp 만 — 예상피해 비율(%) 그대로 | 4 | 기존 |

<details><summary>`callees` 피호출자 4건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | champion_hp_value | game_ai::champion_hp_value | pub | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64 | game-ai\src\utils.rs:909 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | find | game_ai::MinionWaveSnapshot::find | pub | fn(&game_ai::MinionWaveSnapshot, usize) -> std::option::Option<&game_ai::MinionHpTrajectory> | game-ai\src\utils.rs:84 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

**호출처 1곳** (m11.ll:51693) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | champ(%2)는 본문에서 필드를 읽지 않고 expected_damage_target 의 &dyn 인자로만 넘어간다 — vtable @anon.dfa1f8a0e3a880d1d1859ec099fe0b0f.29 가 어느 트레이트 impl 인지는 확인하지 않음(game_core 경계) | 4 |  |
| 1 | 미탐색 | possible_risk(p, data, 75) 내부 및 champion_hp_value(캐시 래퍼, m04.ll:49583) 내부는 계약만(같은 r14/타 배치 소관) | 4 |  |
| 2 | 표기 불가 | L230/L232 비교는 `expected_damage < target.hp` 의 false 방향을 `>=` 로 읽음 — 소스가 `>=` 인지 `>` 의 뒤집힘인지는 외연 동일이라 표기 불가 | 4 |  |
| 3 | 표기 불가 | hp_value 의 umin(…,100)은 i64 smax(…,1) 결과에 unsigned min 을 적용 — 소스가 `as usize` 캐스팅인지 usize 타입인지 표기 불가(양수라 동작 동일) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

