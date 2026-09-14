---

### `166` utils::can1v1win — 1v1 승산: 양쪽 (평타+스킬1+스킬2) 기대 DPS 로 사망 틱을 계산해 내가 상대보다 60틱(1초) 넘게 늦게 죽으면 true

| 항목 | 값 |
|---|---|
| id | `utils__can1v1win` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai5utils9can1v1win` |
| 소스 | `game-ai\src\utils.rs:398` |
| IR | `m04.ll` 54936~55242행 |
| 경로·가시성 | `game_ai::can1v1win` · **pub** |
| 계층 | 기타 |
| exe | `d3ab40` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _rnd | &mut StdRng(320B) | 미사용 (IR readnone) | 4 |
| 1 | 2 | _player | &PlayerState(2528B) | 미사용 (본문 load 0) | 4 |
| 2 | 3 | data | &OperationData(24B) | +0x8 context(&GameContext) 만 — expected_damage_target 인자 | 4 |
| 3 | 4 | champ | &Entity(1728B) | 나. attack/skill/skill2 effect·level·hp·team | 4 |
| 4 | 5 | enemy | &Entity(1728B) | 상대. 같은 필드 + visible_state[my_team] | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn can1v1win(_rnd, _player, data, champ, enemy) -> bool   [utils.rs:398~427]
  ctx = data.context
  L399: my_attack_damage = champ.attack_effect.as_ref().map(|e| e.expected_damage_target(ctx, champ as dyn, enemy)).unwrap_or(0)
  L400: my_skill_damage  = champ.skill_effect  … 동일
  L401: my_skill2_damage = champ.skill2_effect() … (인라인: level>2 아니면 None → 0)
  L403: target_attack_damage = enemy.attack_effect … expected_damage_target(ctx, enemy, champ)
  L404: target_skill_damage  = enemy.skill_effect …
  L405: target_skill2_damage = enemy.skill2_effect() …
  L407~409: my_attack_cooltime = max(champ.attack_cooltime(),1); my_skill_cooltime = max(champ.skill_cooltime(),1); my_skill2_cooltime = max(champ.skill2_cooltime(),1)
  L411~413: target_* 동일(enemy)
  L415: my_hp = champ.hp
  L416: enemy_hp = if enemy.is_visible_from(champ) { enemy.hp } else { enemy.stat_cached.hp(최대) }   // 인라인 entity.rs:1482~1483 · Neutral 이면 가시 취급
  L419: my_dps    = my_attack_damage*100000/my_attack_cooltime + my_skill_damage*100000/my_skill_cooltime + my_skill2_damage*100000/my_skill2_cooltime   (udiv · IR 는 *100000 을 각 phi 전에 미리 곱함)
  L420: enemy_dps = 같은 식(target_*)
  L422: my_die_tick    = my_hp*100000 / enemy_dps        // enemy_dps==0 → 패닉(div_by_zero) — 검사 순서: enemy_dps 먼저(55216), 그다음 my_dps(55221)
  L423: enemy_die_tick = enemy_hp*100000 / my_dps        // my_dps==0 → 패닉
  L426: return my_die_tick > enemy_die_tick + 60
  ※ ult 은 계산에 없음(평타·스킬1·스킬2 만). 궁·아이템 액티브·힐 미반영.
```

**`mem` 메모리 접근 13건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext → expected_damage_target 6회 (m04.ll:54973~54974 · 54990~54991) | 4 | OK |
| 1 | Entity | 0x4c0 | attack_effect@tag | r | champ(m04.ll:54965~54967)·enemy(55059~55061) -1=None → 해당 대미지 0 | 4 | OK |
| 2 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect(56B) → expected_damage_target (54971 · 55065) | 4 | OK |
| 3 | Entity | 0x4f8 | skill_effect@tag | r | champ(55005~55007)·enemy(55084~55086) -1=None → 0 | 4 | OK |
| 4 | Entity | 0x4c8 | skill_effect@Some.0 | r | &Effect → expected_damage_target (55011 · 55090) | 4 | OK |
| 5 | Entity | 0x5c8 | level | r | Entity::skill2_effect 인라인(entity.rs:1693): level > 2 ? &skill2_effect : &정적 None(@anon.16) — champ(55029~55033)·enemy(55108~55112) | 4 | OK |
| 6 | Entity | 0x500 | skill2_effect@Some.0 | r | &Effect(56B) — select 결과 포인터 (55032 · 55111) | 4 | OK |
| 7 | Entity | 0x530 | skill2_effect@tag | r | select 된 eff+0x30 (i32, 55035~55037 · 55114~55116) -1=None → 0. 절대 gep 없음(상대 +48) | 4 | OK |
| 8 | Entity | 0x670 | hp | r | my_hp = champ.hp (55167~55168) · enemy_hp (가시일 때, phi 1648 at 55200) | 4 | OK |
| 9 | Entity | 0x628 | stat_cached.hp | r | enemy_hp = 최대 HP (비가시일 때, phi 1576 at 55200) | 4 | OK |
| 10 | Entity | 0x0 | team@tag | r | champ.team: 1=Neutral → player_team()=None → is_visible_from true (55171~55172, entity.rs:1136/1482) | 4 | OK |
| 11 | Entity | 0x8 | team@Player.0 | r | champ 팀 인덱스 (55176~55178) ≥2 → panic_bounds_check | 4 | OK |
| 12 | Entity | 0x38 | visible_state[team]@tag | r | enemy.visible_state[my_team] (24B stride, 55185~55188) 0=Visible → 현재 hp, 아니면 max hp | 4 | OK |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 399 | 센티널 | Option<Effect> None 니치 태그(i32) — attack/skill/skill2 × champ/enemy 6곳 (54967·55007·55037·55061·55086·55116). None 이면 그 슬롯 대미지 0 | 4 |
| 1 | 100000 | 419 | 계수 | DPS 고정소수 스케일: damage*100000/cooltime (6회, 54985 등) · die_tick = hp*100000/dps (55229·55230) — 나눗셈 정밀도 확보용, 임계 아님 | 4 |
| 2 | 1 | 407 | 임계 | 쿨타임 하한 `max(cooltime, 1)` (llvm.umax ×6, 55140~55165) — 0 나눗셈 방지 | 4 |
| 3 | 2 | 401 | 임계 | `level > 2` — Entity::skill2_effect 인라인 스킬2 해금 게이트(55031 · 55110). 같은 2 가 team bounds(len 2, 55192) | 4 |
| 4 | 0 | 416 | 태그 | VisibleState::Visible 태그 0 (55188). L422/423 `dps == 0` 검사(55216·55221)는 0 이면 패닉으로 빠지는 나눗셈 가드 | 4 |
| 5 | 60 | 426 | 오프셋가감 | 승리 마진(틱) — my_die_tick > enemy_die_tick + 60 (55235~55236). tps 60 기준 1초. 리터럴 60 이라 tps 비례 아님 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 1v1 승리 판정 마진(틱) | utils.rs:426 | 60 | 올리면 더 확실히 유리할 때만 true(1v1 교전 개시 보수적) · 내리면(0) 사망 틱이 조금이라도 늦으면 true | 4 | 기존 |
| 1 | 스킬2 포함 레벨 게이트(game_core 인라인) | entity.rs:1693 (Entity::skill2_effect, m04.ll:55031·55110) | 2 | 정보성 — 레벨 ≤2 챔피언은 skill2 대미지 0 으로 계산 | 4 | 기존 |

<details><summary>`callees` 피호출자 7건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | can1v1win | game_ai::can1v1win | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\utils.rs:398 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 4 | skill2_cooltime | game_core::Entity::skill2_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1796 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 6 | skill_cooltime | game_core::Entity::skill_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1781 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `my_dps`, `my_skill2_cooltime`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m02.ll:23717, m02.ll:25167) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | Effect::expected_damage_target(&Effect(56B), &GameContext(64B), caster &dyn(ptr, vtable @anon.6), target &Entity)->u64 내부 — game_core 경계, 시그니처만 | 4 |  |
| 1 | 미탐색 | Entity::attack_cooltime/skill_cooltime(&Entity)->u64(range ≥3) · skill2_cooltime(&Entity)->u64(range ≥1) — game_core 경계(_gcbc/g06.ll:66510·66405·66747). 반환값 범위 메타데이터상 0 이 될 수 없으므로 umax(…,1) 은 실효 없음(방어 코드) | 4 |  |
| 2 | 미탐색 | enemy_dps==0(상대가 effect 전무·대미지 0) 이면 패닉 — 호출측이 어떻게 막는지는 이 함수 범위 밖 | 4 |  |
| 3 | 표기 불가 | L419 소스가 `(a*100000/ca) + (b*100000/cb) + (c*100000/cc)` 인지 다른 결합인지 — IR 덧셈 순서 (skill+attack)+skill2 로 접혀 있고 값 동일(표기 불가) | 4 |  |
| 4 | 미탐색 | _rnd/_player 가 인자로 남은 이유(과거 시그니처 호환으로 추정) — 본문 근거 없음 | 5 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

