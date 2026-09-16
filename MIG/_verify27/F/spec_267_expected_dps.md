---

### `267` expected_dps — self(캐스터)가 enemy 에게 넣는 기대 DPS = 3슬롯(평타·skill·skill2) 각 expected_damage_target×1000/쿨타임 의 합(usize)

| 항목 | 값 |
|---|---|
| id | `fight_check__expected_dps` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai11fight_check12expected_dps` |
| 소스 | `game-ai\src\fight_check.rs:9` |
| IR | `m15.ll` 23520~23697행 |
| 경로·가시성 | `game_ai::expected_dps` · **pub** |
| 계층 | 점수화·술어 |
| exe | `eb5dd0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | context | &GameContext(64B) | IR 속성 `noalias readonly captures(address, read_provenance) dereferenceable(64)` · 본문에서 직접 필드 읽기 없음 — expected_damage_target ×3 에 그대로 전달만 | 4 |
| 1 | 2 | self | &Entity(1728B) | IR 속성 `noalias readonly dereferenceable(1728)` · 캐스터. DI 변수명은 `target` 과 `self` 둘 다 %1 에 붙어 있음(m15.ll:23523~23524) — 소스 인자명은 확정 못 함(tcx sig 는 이름 없이 `&Entity`) | 3 |
| 2 | 3 | enemy | &Entity(1728B) | IR 속성 `noalias readonly dereferenceable(1728)` · 피해 대상. expected_damage_target 의 5번째 인자(target)로만 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// fight_check.rs:9  pub fn expected_dps(ctx:&GameContext, self:&Entity, enemy:&Entity) -> usize
// L10: let _t = ProfTimer::new(52)  — prof::ENABLED(atomic i8)==0 이면 None(%13=-1), 아니면 Instant::now 저장. 계측 전용
res = 0
// L13~14 평타 슬롯
if self.attack_effect.tag(@0x4c0) != -1 {                       // Some
   dmg = Effect::expected_damage_target(&self.attack_effect(@0x490), ctx, self as &dyn AbstractEntity, enemy)
   cd  = Entity::attack_cooltime(self)          // ≥3
   if cd == 0 { panic div_by_zero }              // 도달 불가(range 3..)
   res = dmg * 1000 / cd                          // udiv
}
// L17~18 skill 슬롯
if self.skill_effect.tag(@0x4f8) != -1 {
   dmg = expected_damage_target(&self.skill_effect(@0x4c8), ctx, self, enemy)
   cd  = Entity::skill_cooltime(self)           // ≥3
   res += dmg * 1000 / cd
}
// L21~22 skill2 슬롯 — DI 지역변수명은 `ult` 이지만 IR 이 읽는 것은 skill2_effect(@0x500) 이고 쿨타임도 skill2_cooltime (IR 정본)
ult = if self.level(@0x5c8) > 2 { &self.skill2_effect(@0x500) } else { &NONE }   // 인라인 Entity::skill2_effect() entity.rs:1693
if ult.tag(+0x30) != -1 {
   dmg = expected_damage_target(ult, ctx, self, enemy)
   cd  = Entity::skill2_cooltime(self)          // ≥1
   res += dmg * 1000 / cd
}
// L26 return — drop(_t): ENABLED 였으면 PHASE_NANOS[52] += elapsed_ns, PHASE_CALLS[52] += 1 (atomicrmw)
return res

// 분기 순서: attack → skill → skill2 (소스 줄 13→17→21). 각 항은 독립(앞 항의 결과가 뒤 조건에 영향 없음). 궁(ult_effect @0x538)은 이 함수에서 읽지 않는다.
// rnd 인자 없음 · gen_range 0회.
```

**`mem` 메모리 접근 11건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | Entity | 0x490 | attack_effect (Option<Effect> 56B, Some 페이로드 시작) | r | m15.ll:23557 `%1+1168` → expected_damage_target 의 &Effect 인자(DI `atk`). tcxdict Entity 0x490 = attack_effect@Some.0.ty.ptr | 3 | OK |  |
| 1 | Entity | 0x4c0 | attack_effect@tag (니치 태그 i32, -1=None) | r | m15.ll:23551~23553 `load i32 … icmp eq -1` → None 이면 평타 항 건너뜀. tcxdict: attack_effect@tag (Niche, 4B) | 3 | OK |  |
| 2 | Entity | 0x4c8 | skill_effect (Option<Effect>, Some 페이로드 시작) | r | m15.ll:23597 `%1+1224` (DI `skill`) | 4 | OK |  |
| 3 | Entity | 0x4f8 | skill_effect@tag (i32, -1=None) | r | m15.ll:23565~23567 | 4 | OK |  |
| 4 | Entity | 0x5c8 | level (usize) | r | m15.ll:23605~23607 `icmp ugt level, 2` — 인라인된 `Entity::skill2_effect()`(entity.rs:1693, dloc !37105) 의 레벨 게이트 | 4 | OK |  |
| 5 | Entity | 0x500 | skill2_effect (Option<Effect>, Some 페이로드 시작) | r | m15.ll:23608~23609 level>2 이면 `%1+1280`, 아니면 상수 None(`@anon.…58` = 48B undef + tag FF FF FF FF, m15.ll:64) 을 select | 4 | OK |  |
| 6 | Entity | 0x530 | skill2_effect@tag (i32, -1=None) — select 결과 +48 로 읽음 | r | m15.ll:23610~23612 `gep %46, 48` → level≤2 이면 상수 None 의 태그(-1) 를 읽어 항상 건너뜀 | 4 | OK |  |
| 7 | static | 0x0 | game_core::simulation::prof::ENABLED (atomic i8) | r | m15.ll:23533 계측 스위치 — 0 이면 ProfTimer 미생성(%13=-1). 판정과 무관 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 8 | stack | 0x0 | _t: Option<ProfTimer>(24B alloca %4) | w | m15.ll:23541~23549 — 계측 전용 로컬. 함수 밖 &mut 쓰기 없음(전 인자 readonly) | 4 | 확인불가(★모호: 동명 def_path 3개 [('game_core::Hunter) | phase=52 @+0, Instant.secs @+8, Instant.nanos(-1=None) @+16 |
| 9 | static | 0x1a0 | prof::PHASE_NANOS[52] (atomicrmw add, ns) | w | m15.ll:23669 `PHASE_NANOS+416`(=52×8) — ENABLED 였을 때만(%13≠-1). 계측 카운터, 판정과 무관 | 4 | 확인불가(tcx 사전에 타입 없음) | elapsed ns |
| 10 | static | 0x1a0 | prof::PHASE_CALLS[52] (atomicrmw add 1) | w | m15.ll:23672 `PHASE_CALLS+416`. 계측 카운터 | 4 | 확인불가(tcx 사전에 타입 없음) | +1 |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 13 | 센티널 | Option<Effect> 니치 태그 None. attack(L13)·skill(L17)·skill2(L21) 세 곳 모두 `icmp eq i32 tag, -1` 로 None 판별(m15.ll:23553·23567·23612) | 4 |
| 1 | 1000 | 14 | 계수 | DPS 스케일 — `expected_damage_target × 1000 / cooltime`. 세 항 동일(m15.ll:23576·23616·23680). 정수 나눗셈이라 1000 을 먼저 곱해 정밀도 확보 | 4 |
| 2 | 2 | 21 | 임계 | 인라인 `Entity::skill2_effect()`(entity.rs:1693) 의 레벨 게이트 `level > 2` — 3레벨 미만이면 skill2 슬롯을 None 으로 취급(m15.ll:23607) | 4 |
| 3 | 0 | 14 | 태그 | `udiv` 0 나눗셈 가드(`icmp eq cooltime, 0` → panic_const_div_by_zero, m15.ll:23581·23621·23685). 콜리 정의가 `range(i64 3,0)`(attack/skill) · `range(i64 1,0)`(skill2) 라 실제로는 도달 불가 | 4 |
| 4 | 52 | 10 | 산출값 | prof 계측 phase 번호(ProfTimer). 판정과 무관(m15.ll:23541) | 4 |
| 5 | 416 | 10 | 미상 | PHASE_NANOS/PHASE_CALLS 배열 바이트 오프셋 = 52×8. 계측 전용(m15.ll:23669·23672) | 4 |
| 6 | 1000000000 | 10 | 임계 | Instant.elapsed → ns 환산(secs×1e9+nanos). 계측 전용(m15.ll:23661~23662) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | DPS 스케일 계수 | fight_check.rs:14/18/22 | 1000 | 세 항에 동일 적용되는 스케일이라 소비자(교전 저울)의 다른 통화와의 비율만 바뀐다. 한 항만 바꾸면 그 슬롯의 비중이 바뀜(예: 평타만 500 → 평타 DPS 절반 평가) | 4 | 기존 |
| 1 | skill2 슬롯 레벨 게이트 | entity.rs:1693 (인라인, Entity::skill2_effect) | 2 | `level > 2` — 낮추면(예 0) 1~2레벨에서도 skill2 DPS 가 합산돼 저레벨 교전 평가가 공격적으로 변함. 이 함수 안에서 바꾸려면 select 조건을 패치 | 4 | 기존 |
| 2 | 쿨타임 하한 | entity.rs:1767 Entity::attack_cooltime (콜리 내부) | 3 | attack/skill 쿨 하한 3틱 → DPS 상한 = dmg×1000/3. 하한을 내리면 초고가속 캐스터의 DPS 가 폭증. skill2 는 하한 1 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | expected_dps | game_ai::expected_dps | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:9 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 5 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | skill2 | game_ai::skill2 | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:239 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 65개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | skill2 | game_core::Entity::skill2 | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1668 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 65개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | skill2 | game_core::ChampionInfo::skill2 | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1086 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 65개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | skill2_cooltime | game_core::Entity::skill2_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1796 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 13 | skill_cooltime | game_core::Entity::skill_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1781 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `elapsed`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 9곳** (m05.ll:11663, m05.ll:11737, m10.ll:44688, m12.ll:24817, m12.ll:24885, m12.ll:24944, m12.ll:31188, m15.ll:26928, m15.ll:35862) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 소스 인자명: %1 에 DI 변수 `target` 과 `self` 가 둘 다 붙어 있고(m15.ll:23523~23524) tcx sig 는 이름 없이 `&Entity` 라 소스의 실제 인자명(target? self?)은 확정 못 함. 동작상 %1 = 캐스터(effect·cooltime 의 self), %2 = 피해 대상 — IR 정본 | 3 |  |
| 1 | 미탐색 | skill_cooltime / skill2_cooltime 의 내부 공식은 미독해(attack_cooltime 만 확인: max(3, base×100/max(1,가속+100))). skill2 하한이 1 인 이유(range(1,0))도 미독해 | 4 |  |
| 2 | 미탐색 | src_line: DISubprogram(m15.ll:100573) line 9 = 시그니처 줄, 본문 첫 줄(ProfTimer) 은 10 — src_line 은 9 로 기록 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | expected_damage_target 내부(피해 공식·방어 적용·대상 가능 여부)는 게임코어 경계로 미독해 — 반환 0 이면 그 항이 0 이 되는 것만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L21 의 DI 지역변수명이 `ult` 인데 읽는 필드는 skill2_effect(@0x500)·쿨타임은 skill2_cooltime — 소스 오기(변수명만 ult)로 보이나 소스 원문 부재라 「이름」은 표기 불가·「동작」은 IR 로 확정(skill2). 궁(ult_effect @0x538)은 이 함수가 읽지 않는다 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

