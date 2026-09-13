---

### `65` check_epic_kill_time_with_hp — 챔피언들의 (평타+스킬+스킬2) 기대 DPS 합으로 HP 만큼의 에픽을 잡는 데 걸리는 틱 수를 계산

| 항목 | 값 |
|---|---|
| id | `goal_data__check_epic_kill_time_with_hp` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai9goal_data28check_epic_kill_time_with_hp` |
| 소스 | `game-ai\src\goal_data.rs:535` |
| IR | `m09.ll` 52666~52833행 |
| 경로·가시성 | `game_ai::goal_data::check_epic_kill_time_with_hp` · **in:game_ai::goal_data** |
| 계층 | 기타 |
| exe | `de3d90` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::OperationData, &bumpalo::collections::vec::Vec< &game_core::Entity>, &game_core::Entity, usize) -> usize
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | data | &OperationData → 인자승격되어 IR %0 = data.context(&GameContext 64B) | dbg_value 가 poison(m09.ll:52668)이고 %0 은 expected_damage_target 의 context 인자(dereferenceable(64))로만 쓰임 = LLVM ArgumentPromotion. 소스 시그니처는 `data: &OperationData`(DILocalVariable !47139) | 4 |
| 1 | 2 | champions | &bumpalo Vec<&Entity> → 인자승격되어 IR %1=buf ptr, %2=len | 순회 대상 챔피언 목록(DPS 기여자). len 0 이면 dps 합 = 0 | 4 |
| 2 | 3 | epic | &Entity(1728B) | 피격 대상. expected_damage_target 의 target · CastingTarget::check 의 target | 4 |
| 3 | 4 | hp | usize | 잡아야 할 HP 량(현재 HP 가 아니라 호출자가 주는 값) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn check_epic_kill_time_with_hp(data, champions: &Vec<&Entity>, epic: &Entity, hp: usize) -> usize {
  let ctx = data.context;              // IR 에선 인자승격으로 %0 이 곧 ctx
  let mut tick = 0;                    // 536 (!47143)
  let mut dpt  = 0;                    // 538 (!47145) — 누적 DPS×1000 (phi %13)
  for champ in champions.iter() {      // 540 — len==0 이면 루프 없이 559 로
    // 541~542: 평타
    let atk = champ.attack_effect.as_ref().unwrap();   // 541 — tag(+0x4c0) == -1 → unwrap_failed 패닉 (m09.ll:52690~52701)
    dpt += expected_damage_target(atk, ctx, champ as &dyn AbstractEntity, epic) * 1000 / champ.attack_cooltime();   // 541~542 — cooltime 0 → div0 패닉 (m09.ll:52694~52713)
    // 544~547: 스킬 (타겟 조건 통과 시)
    if let Some(skill) = champ.skill_effect.as_ref() {                 // 544 — tag(+0x4f8) != -1
      if skill.target.check(champ, epic) {                             // 545 — CastingTarget(+0x4f0)::check(caster=champ, target=epic)
        dpt += expected_damage_target(skill, ctx, champ, epic) * 1000 / champ.skill_cooltime();   // 546~547
      }
    }
    // 551~554: 스킬2 (레벨 게이트 + 타겟 조건)
    let skill2 = champ.skill2_effect();   // 551 — entity.rs:1693 인라인: if champ.level(+0x5c8) > 2 { &champ.skill2_effect(+0x500) } else { &NONE(@anon.…288) }  (m09.ll:52745~52749)
    if let Some(s2) = skill2 {                                         // 551 — 선택된 Option 의 +48 태그 != -1
      if s2.target.check(champ, epic) {                                // 552 — +40 CastingTarget
        dpt += expected_damage_target(s2, ctx, champ, epic) * 1000 / champ.skill2_cooltime();   // 553~554
      }
    }
  }
  tick = hp * 1000 / max(dpt, 1);      // 559 (m09.ll:52829~52832) — dpt 0 이면 hp*1000
  tick                                 // 561
}

주: expected_damage_target 의 caster 인자는 &dyn AbstractEntity(fat ptr: champ + Entity vtable @anon.…138) 로 넘어간다(m09.ll:144). 결과 단위: hp*1000 / Σ(dmg*1000/cd) = hp / Σ(dmg/cd) = 틱.
```

**`mem` 메모리 접근 9건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Entity | 0x4c0 | attack_effect@tag | r | i32 니치: -1 = None → unwrap_failed(패닉). 챔피언은 항상 Some 전제 | 4 | OK |
| 1 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect(56B) → expected_damage_target 의 self | 4 | OK |
| 2 | Entity | 0x4f8 | skill_effect@tag | r | -1 = None → 스킬 기여 생략 | 4 | OK |
| 3 | Entity | 0x4c8 | skill_effect | r | &Effect | 4 | OK |
| 4 | Entity | 0x4f0 | skill_effect.target | r | CastingTarget(4B) → check(champ, epic) | 4 | OK |
| 5 | Entity | 0x5c8 | level | r | skill2_effect()(entity.rs:1693) 인라인: level >u 2 이면 &skill2_effect, 아니면 정적 None(@anon.…288, tag -1) | 4 | OK |
| 6 | Entity | 0x500 | skill2_effect | r | &Effect | 4 | OK |
| 7 | Entity | 0x530 | skill2_effect@tag | r | 선택된 Option<Effect> 의 +48 = 태그(-1=None) | 4 | OK |
| 8 | Entity | 0x528 | skill2_effect@Some.0.target | r | 선택된 Option<Effect> 의 +40 = CastingTarget → check(champ, epic) | 4 | OK |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1000 | 541 | 계수 | DPS 정수화 배율: dmg*1000/cooltime (541·546·553) 및 hp*1000/dps (559). 분자·분모 양쪽에 곱해 단위가 상쇄되고 결과는 틱 | 4 |
| 1 | -1 | 541 | 센티널 | Option<Effect> None 니치 태그(i32) — attack(541, None→unwrap 패닉)·skill(544)·skill2(551) 존재 판정 | 4 |
| 2 | 2 | 551 | 임계 | skill2_effect() 인라인(entity.rs:1693): `icmp ugt i64 level, 2` → level ≥3 일 때만 스킬2 효과를 봄 (m09.ll:52746). ⚠본문의 `shl 3` 은 슬라이스 stride 8 이지 이 2 와 무관 | 4 |
| 3 | 1 | 559 | 인덱스 | 0 나눗셈 방지: max(dps_sum, 1) (llvm.umax; champions 비어 있으면 상수 1 로 접힘 m09.ll:52829) | 4 |
| 4 | 0 | 536 | 태그 | 누적 초기값(tick/dpt = 0) · 쿨타임 0 검사(0 이면 panic_const_div_by_zero) · len 0 검사 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 스킬2 반영 레벨 하한 | goal_data.rs:551 (entity.rs:1693 skill2_effect 인라인) | 2 | game_core 쪽 규칙이라 이 함수만 바꿔선 안 바뀜. 값이 크면 저레벨 챔피언의 스킬2 DPS 가 빠져 처치 시간이 길게 계산됨 | 4 | 기존 |
| 1 | DPS 정수화 배율 | goal_data.rs:541/546/553/559 | 1000 | 정밀도만 바꾼다(분자·분모 상쇄). 너무 작으면 정수 절삭으로 dps 가 0 이 되어 hp*배율 그대로 반환 | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | check_epic_kill_time_with_hp | game_ai::goal_data::check_epic_kill_time_with_hp | in:game_ai::goal_data | fn(&game_core::OperationData, &bumpalo::collections::vec::Vec< &game_core::Entity>, &game_core::Entity, usize) -> usize | game-ai\src\goal_data.rs:535 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | skill2_cooltime | game_core::Entity::skill2_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1796 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 9 | skill_cooltime | game_core::Entity::skill_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1781 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

**호출처 9곳** (m09.ll:22402, m09.ll:22582, m09.ll:22781, m09.ll:41333, m09.ll:41507, m09.ll:41638, m09.ll:43038, m09.ll:43213, m09.ll:43334) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 지역변수 `tick`(536)과 `dpt`(538)의 정확한 역할 배분 — IR 은 phi 하나(%13, 초기 0)만 남기고 최종 나눗셈만 하므로 `tick` 이 559 에서 대입되는 결과 변수이고 `dpt` 가 누적자라는 것은 이름·줄번호로 추정(동작은 확정) | 4 |  |
| 1 | 미탐색 | expected_damage_target(g06.ll !63700, 4인자 self/context/caster/target)·attack_cooltime·skill_cooltime·skill2_cooltime·CastingTarget::check 내부 — _gcbc 에 define 있음, 미독해. 특히 cooltime 이 0 을 돌려줄 수 있는지(div0 패닉 경로 도달 가능성) | 4 |  |
| 2 | 표기 불가 | 541 의 `unwrap()` 인지 `expect()` 인지 — unwrap_failed 호출이라 unwrap 유력(표기 불가) | 4 |  |
| 3 | 표기 불가 | 545/552 의 `check` 가 `if` 인지 `filter` 류 콤비네이터인지 — 표기 불가 | 4 |  |
| 4 | 미탐색 | reads 의 0x530/0x528 은 본문에 절대 오프셋으로 안 나온다(C3 경고) — skill2_effect() 가 `select(level>2, champ+0x500, &NONE)` 로 고른 포인터 %44 에 +48/+40 을 더하는 상대 gep(m09.ll:52750~52751, 52774)이라 접힘. tcxdict Entity 0x530/0x528 = skill2_effect@tag / @Some.0.target 로 일치 확인 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

