---

### `237` RecallSubPlan::score — 귀환 서브플랜 액션 점수: interaction_score 를 기저로 도주/귀환류 +50, 그 외는 양수면 CC 스킬만 +50·나머지 /3, 음수면 ×3

| 항목 | 값 |
|---|---|
| id | `recall__Recall__score` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6recallNtB2_13RecallSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\recall.rs:42` |
| IR | `m02.ll` 40317~40506행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::RecallSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `cc5fc0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::RecallSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[237]/sig/tls/<키>`)**

없음 — 본문에 `@anon.* = constant ptr @<KEY…call_once>` 참조 0 · LocalKey::with 0 (interaction_score 내부는 콜리 계약 밖)

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &RecallSubPlan (0B ZST) | IR `readonly nonnull` · 본문에서 읽지 않음 | 4 |
| 1 | 2 | version | usize | 본 함수 분기 없음 · interaction_score 1번 인자로 통과(m02.ll:40326) | 4 |
| 2 | 3 | parameter | &ScoreParameter (5384B) | IR readonly · 본문 직접 읽기 없음 · interaction_score 5번 인자로 통과 | 4 |
| 3 | 4 | rnd | &mut StdRng (320B) | IR 속성 없음(=&mut). 본 함수 gen_range 사이트 0 · interaction_score 2번 인자로 통과만 | 4 |
| 4 | 5 | player | &PlayerState (2528B) | IR readonly | 4 |
| 5 | 6 | data | &OperationData (24B) | IR readonly | 4 |
| 6 | 7 | action | &SmallActionPlay (184B) | IR readonly · 태그 +0xb1 만 직접 읽음 | 4 |
| 7 | 8 | debug | &mut DebugFrameData (224B) | IR 속성 없음(=&mut). 본 함수 직접 store 0 · interaction_score 7번 인자로 통과만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   [recall.rs:42]
  base = action_score::interaction_score(version, rnd, player, data, parameter, action, debug)   [L43 · m02.ll:40326]
  idx = 논리 variant idx(action 태그 +0xb1)                                                        [L46 · 40329~40354]
  match action {
    RunAway(0) | Recall(1) | AroundRunAway(5) => return base + 50                                  [L47 · 40364]
    _ => {                                                                                          [L48]
      if base > 0 {                                                                                 [L48 · 40360 sgt]
        champ = data.cache.player_champion[player.info.team][player.info.position].unwrap()         [L49 · 40372~40393 · team<2 바운드체크]
        match action {
          Skill(13)  => if champ.skill_effect.as_ref().unwrap().ty.expected_cc_time().is_some()  { base + 50 } else { base / 3 }   [L51 · 40439~40463 → 40471 / 40500]
          Skill2(14) => if champ.skill2_effect()/*level>2 ? &skill2_effect : &None*/.as_ref().unwrap().ty.expected_cc_time().is_some() { base + 50 } else { base / 3 }   [L52 · 40422~40431, 40478~40492]
          _ (Around·AroundHide·AroundRegion·Positioning·AroundPosition·AroundPositionBush·AroundBush·LaneMinionPosition·Trace·Attack·Ult·Stop) => base / 3   [L55 · 40500]
        }
      } else { base * 3 }                                                                           [L58 · 40368]
    }
  }                                                                                                 [L60 ret · 40505]

주의: Ult(15)·Attack(12) 은 CC 검사 없이 base/3. Skill2 가지는 champ.level<=2 이면 정적 None → unwrap 패닉 경로(40496 · 실전에선 Skill2 후보 자체가 안 생기므로 도달 조건은 콜러 쪽). gen_range 사이트 0. self/parameter 직접 읽기 0.
```

**`mem` 메모리 접근 12건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | SmallActionPlay | 0xb1 | 태그(니치 1B) | r | m02.ll:40329~40335 · 논리 idx = tag>2 ? tag-3 : 7(AroundPosition 암묵) · assume(tag!=10) | 4 | OK |
| 1 | PlayerState | 0x930 | info.team | r | player_champion 1차 인덱스 · `<2` 바운드체크 (m02.ll:40372~40378) | 4 | OK |
| 2 | PlayerState | 0x9c0 | info.position@tag | r | i32 zext · 2차 인덱스 (m02.ll:40383~40385, player.rs:581 인라인) | 4 | OK |
| 3 | OperationData | 0x0 | cache | r | m02.ll:40386 | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [2][5] Option<&Entity> null=None → unwrap_failed(40434) (m02.ll:40387~40393) | 4 | OK |
| 5 | Entity | 0x4f8 | skill_effect@tag(casting 니치 i32) | r | -1 = None → unwrap_failed(40467) (m02.ll:40439~40442) — Skill 가지 | 4 | OK |
| 6 | Entity | 0x4c8 | skill_effect.ty (Arc<dyn EffectType> data ptr) | r | m02.ll:40445~40449 — Skill 가지 | 4 | OK |
| 7 | Entity | 0x4d0 | skill_effect.ty vtable ptr | r | vtable+0x10 align 으로 ArcInner 데이터 오프셋 계산(40452~40457) · vtable+0x80 = EffectType::expected_cc_time (g02.ll:1311 정적 vtable 슬롯 순서로 확정) (40458~40460) | 4 | OK |
| 8 | Entity | 0x5c8 | level | r | Entity::skill2_effect(entity.rs:1692) 인라인: level>2 면 &skill2_effect 아니면 정적 None(@anon.19 = 48B undef + i32 -1) (m02.ll:40422~40426) — Skill2 가지 | 4 | OK |
| 9 | Entity | 0x500 | skill2_effect.ty (Arc data ptr) | r | m02.ll:40478 — Skill2 가지 (level>2 일 때) | 4 | OK |
| 10 | Entity | 0x508 | skill2_effect.ty vtable ptr | r | m02.ll:40479~40480 · +0x80 = expected_cc_time (40487~40489) | 4 | OK |
| 11 | Entity | 0x530 | skill2_effect@tag(casting 니치 i32) | r | 선택된 &Option<Effect> +0x30(48) 을 -1 과 비교 → None 이면 unwrap_failed(40496) (m02.ll:40428~40431) | 4 | OK |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 50 | 47 | 계수 | RunAway/Recall/AroundRunAway 가산 (m02.ll:40364 `add %9, 50`) | 4 |
| 1 | 50 | 53 | 계수 | base>0 이고 Skill/Skill2 의 스킬 효과가 expected_cc_time().is_some() 이면 가산 (m02.ll:40471 `add nuw %9, 50`) | 4 |
| 2 | 3 | 55 | 태그 | base>0 · 그 밖의 액션(또는 CC 없는 Skill/Skill2) → base/3 (m02.ll:40500 `udiv %9, 3` · base>0 보장이라 부호 무관) | 4 |
| 3 | 3 | 58 | 태그 | base<=0 → base*3 (m02.ll:40368 `mul %9, 3`) — 음수 기저를 3배로 벌린다 | 4 |
| 4 | 0 | 48 | 임계 | `base > 0` 분기 (m02.ll:40360 icmp sgt) | 4 |
| 5 | 2 | 49 | 길이 | player_champion 1차 길이 바운드체크 team<2 (m02.ll:40374) | 4 |
| 6 | 2 | 52 | 임계 | Entity::skill2_effect 인라인 `level > 2` — 2레벨 이하면 skill2_effect 를 None 으로 본다(m02.ll:40424, entity.rs:1693) | 4 |
| 7 | -1 | 51 | 센티널 | Option<Effect> 니치 None 판별값(casting 태그 i32 = -1) (m02.ll:40430, 40441) | 4 |
| 8 | 128 | 51 | 미상 | dyn EffectType vtable 슬롯 바이트 오프셋 0x80 = expected_cc_time (drop·size·align·Debug::fmt 뒤 13번째 메서드 — g02.ll:1311 정적 vtable 실측) (m02.ll:40458, 40487) | 4 |
| 9 | 1 | 51 | 태그 | expected_cc_time() -> Option<usize> 의 {i64,i64} 첫 워드 == 1 → Some (m02.ll:40462, 40491) | 4 |
| 10 | 10 | 46 | 센티널 | SmallActionPlay 니치 태그 10(암묵 AroundPosition 자리) 은 절대 안 나옴 → llvm.assume (m02.ll:40331) | 4 |
| 11 | 7 | 46 | 태그 | 태그 ≤2 이면 논리 idx 7 = AroundPosition(untagged) (m02.ll:40335 select) | 4 |
| 12 | -3 | 46 | 센티널 | 니치 태그 → 논리 idx 변환 `tag-3` (niche_start=3) (m02.ll:40333) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 도주·귀환류 가산 | recall.rs:47 | 50 | 올리면 귀환 서브플랜 중 RunAway/Recall/AroundRunAway 가 다른 후보(스킬·추적 등) 대비 더 잘 뽑힌다 | 4 | 기존 |
| 1 | CC 스킬 가산 | recall.rs:53 | 50 | 올리면 귀환 중 CC 스킬(expected_cc_time Some) 사용 후보가 귀환 자체와 대등하게 경쟁한다(둘 다 +50) · 내리면 귀환 우선 | 4 | 기존 |
| 2 | 그 외 액션 감쇠 제수 | recall.rs:55 | 3 | 키우면 귀환 중 비-CC 액션(공격·궁·추적·자리잡기)이 더 억제된다 | 4 | 기존 |
| 3 | 음수 기저 배수 | recall.rs:58 | 3 | 키우면 interaction_score 가 음수인 후보가 더 강하게 배제된다 | 4 | 기존 |
| 4 | skill2 해금 레벨 | entity.rs:1693 (Entity::skill2_effect · game_core) | 2 | game_core 쪽 규칙 — 여기서 바꿀 수 없음(참조만) | 4 | 기존 |

<details><summary>`callees` 피호출자 6건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | expected_cc_time | game_core::EffectType::expected_cc_time | pub | fn(&Self/#0) -> std::option::Option<usize> | game-core\src\simulation\effect\type.rs:299 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 37개 중 상위 3개 |
| 1 | expected_cc_time | game_core::EffectBuff::expected_cc_time | pub | fn(&Self/#0) -> std::option::Option<usize> | game-core\src\simulation\effect.rs:170 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 37개 중 상위 3개 |
| 2 | expected_cc_time | <game_core::GrabEffect as game_core::EffectType>::expected_cc_time | pub | fn(&game_core::GrabEffect) -> std::option::Option<usize> | game-core\src\simulation\effect\type\grab.rs:31 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 37개 중 상위 3개 |
| 3 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | score | game_ai::plan_legacy::sub_plan::RecallSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::RecallSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\recall.rs:42 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

**호출처 1곳** (m12.ll:37248) · **형제 6개** (RecallSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::RecallSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::RecallSubPlan) -> game_ai::plan_legacy::sub_plan::RecallSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::RecallSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::RecallSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::sub_plan::RecallSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:5 | True | fn() -> game_ai::plan_legacy::sub_plan::RecallSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::RecallSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:10 | False | fn(&mut game_ai::plan_legacy::sub_plan::RecallSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::RecallSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:30 | True | fn(&game_ai::plan_legacy::sub_plan::RecallSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 5 | game_ai::plan_legacy::sub_plan::RecallSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:42 | False | fn(&game_ai::plan_legacy::sub_plan::RecallSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | interaction_score 내부(정본 r15) — 여기서는 (version,rnd,player,data,parameter,action,debug)->i64 계약만 | 3 |  |
| 1 | 표기 불가 | recall.rs:51/52 의 `is_some()` 이 소스에서 `.is_some()` 인지 `matches!(…, Some(_))` 인지 — 표기 불가(외연 동일) | 4 |  |
| 2 | 미탐색 | Skill2 가지에서 champ.level<=2 일 때 unwrap 패닉(40496)이 실전에서 도달 가능한지 — 콜러(SmallActionSkill2 후보 생성 조건) 범위 밖 · 미탐색 | 4 |  |
| 3 | 미탐색 | expected_cc_time 의 어느 구현체가 꽂히는지 — 런타임 vtable 이라 IR 로 불가(슬롯 이름만 확정) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

