---

### `235` AttackNexusSubPlan::score — AttackNexus 서브플랜의 후보 액션 점수: interaction_score + Attack/Skill/Skill2 는 calculate_action_score(Push) 가산(Skill/Skill2 는 대상이 타워면 +3), 대상 소실=-99999, Around 는 0(조회만), 그 외 0

| 항목 | 값 |
|---|---|
| id | `attack_nexus__AttackNexus__score` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12attack_nexusNtB2_18AttackNexusSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:158` |
| IR | `m14.ll` 20984~21208행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `e83080` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[235]/sig/tls/<키>`)**

없음 — LocalKey::with / @anon…call_once fn-포인터 참조 0 (20984~21208 전수)

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &AttackNexusSubPlan(0B, ZST) |  | 4 |
| 1 | 2 | version | usize |  | 4 |
| 2 | 3 | parameter | &ScoreParameter(5384B) |  | 4 |
| 3 | 4 | rnd | &mut StdRng(320B, align16) |  | 4 |
| 4 | 5 | player | &PlayerState(2528B) |  | 4 |
| 5 | 6 | data | &OperationData(24B) |  | 4 |
| 6 | 7 | action | &SmallActionPlay(184B) |  | 4 |
| 7 | 8 | debug | &mut DebugFrameData(224B) |  | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // attack_nexus.rs:158 · self=ZST
let champ = data.cache.player_champion[player.info.team][player.info.position as usize].unwrap();   // L159
let base = interaction_score(version, rnd, player, data, parameter, action, debug);   // L160
let action_type = MinionActionType::Push;   // DI 상수 i8 2
let add = match action.get_action() {   // L164 · SmallActionPlay 태그 switch
    Attack{target_id} => if let Some(t) = data.cache.game.get_entity_by_id(target_id) /*L169*/ {
            let effect = champ.attack_effect.as_ref().unwrap();   // L170
            calculate_action_score(version, rnd, player, data, parameter, &champ.attack, effect, champ.attack_speed_mult(), t, Push, debug)   // L171 · ★tower_bonus 없음
        } else { -99999 },
    Skill{target_id} => if let Some(t) = get_entity_by_id(target_id) /*L177*/ {
            let tower_bonus = if t.is_tower() /*L178 · ty@tag==2*/ { 3 } else { 0 };
            let effect = champ.skill_effect.as_ref().unwrap();   // L184
            calculate_action_score(version, rnd, player, data, parameter, &champ.skill, effect, champ.cooldown_reduce(false), t, Push, debug) + tower_bonus   // L185
        } else { -99999 },
    Skill2{target_id} => if let Some(t) = get_entity_by_id(target_id) /*L191*/ {
            let tower_bonus = if t.is_tower() /*L193*/ { 3 } else { 0 };
            let effect = (if champ.level > 2 { champ.skill2_effect.as_ref() } else { None }).unwrap();   // L194 entity.rs:1693
            calculate_action_score(version, rnd, player, data, parameter, &champ.skill2, effect, champ.cooldown_reduce(false), t, Push, debug) + tower_bonus   // L195
        } else { -99999 },
    Around{target_id} /*Play idx 2/3/10*/ => { let _ = get_entity_by_id(target_id) /*L201 · 호출은 남음(간접 호출이라 제거 불가), 결과 미사용*/; 0 }   // L201~L210 → 0
    _ => 0,
};
return base + add;   // L164 합산 · L213 반환

★형제 대조(복붙 의심): LineSafe/LineWait 와 골격 동일(interaction_score → get_action switch → calculate_action_score) 이나 ①evaluate_action(ActionContext) 신경로 없음 ②line_action_economy_adjustment 없음 ③MinionActionType Push(2) vs Pull(0) ④tower_bonus 3 이 Skill/Skill2 에만 있고 Attack 에는 없음(비대칭 — 의도인지 누락인지 소스 부재로 판정 불가) ⑤Around arm 이 LineSafe 의 아군구조물 50/100 가산 대신 0 (get_entity_by_id 호출 잔존 = 본문이 t 를 안 읽는 형태로 남은 복붙 흔적 의심 · L201~L210 줄길이 71/36/14/19/14/12/17/12/10/8 ↔ LineSafe L142·L157~L164 의 71/…/14/19/14/12/17/12/10/8 과 꼬리 일치).
rnd gen_range 사이트: 0(전달만: interaction_score → calculate_action_score).
version 분기 0 · debug 직접 쓰기 0 · self 접근 0.
```

**`mem` 메모리 접근 20건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L159 · <2 bounds check | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag(i32) | r | L159(player.rs:581) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | L159 | 4 | OK |
| 3 | AbstractGameWithCache | 0x0 | game.data_ptr | r | L169/177/191/201 | 4 | OK |
| 4 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x1f0(=496) get_entity_by_id 슬롯 4곳 | 4 | OK |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | L159 · null → unwrap_failed | 4 | OK |
| 6 | SmallActionPlay | 0xb1 | tag(u8) | r | L164(small_action.rs:309 get_action) · idx 2/3/10 → Around arm, 12/13/14 → Attack/Skill/Skill2, 나머지 0 | 4 | OK |
| 7 | SmallActionPlay | 0x8 | target / target_id | r | Around 계열(+0x8 target) · Attack(cast.rs:94)/Skill(:160)/Skill2(:222) | 4 | OK |
| 8 | Entity(champ) | 0x4c0 | attack_effect@tag(-1=None) | r | L170(option.rs:742) · None → unwrap_failed | 4 | OK |
| 9 | Entity(champ) | 0x490 | attack_effect@Some.0 | r | L171 | 4 | OK |
| 10 | Entity(champ) | 0x570 | attack (Box<dyn Action>) | r | L171(entity.rs:1494) | 4 | OK |
| 11 | Entity(champ) | 0x4f8 | skill_effect@tag | r | L184 | 4 | OK |
| 12 | Entity(champ) | 0x4c8 | skill_effect@Some.0 | r | L185 | 4 | OK |
| 13 | Entity(champ) | 0x580 | skill (Box<dyn Action>) | r | L185(entity.rs:1665) | 4 | OK |
| 14 | Entity(champ) | 0x5c8 | level | r | L194(entity.rs:1693) · level>2 아니면 unwrap_failed | 4 | OK |
| 15 | Entity(champ) | 0x530 | skill2_effect@tag | r | L194 | 4 | OK |
| 16 | Entity(champ) | 0x500 | skill2_effect@Some.0 | r | L194(entity.rs:1694) | 4 | OK |
| 17 | Entity(champ) | 0x590 | skill2 (Box<dyn Action>) | r | L195(entity.rs:1670) | 4 | OK |
| 18 | Entity(t) | 0x68 | ty@tag (EntityType) | r | L178/L193(entity.rs:1386 is_tower 인라인) · ==2(Tower) → tower_bonus 3. ★Attack arm 에는 이 검사가 없음 | 4 | OK |
| 19 | AbstractGame vtable | 0x1f0 | get_entity_by_id(&self, usize)->Option<&Entity> | r | 간접 호출 4곳(Around arm 포함) — 심볼 호출 아님 | 4 | 확인불가(vtable 슬롯) |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 10 | 164 | 센티널 | SmallActionPlay 니치 구멍 assume(tag≠10). 판정 아님 | 4 |
| 1 | 3 | 164 | 태그 | 태그→idx 환산(tag-3). 판정 아님. ★같은 값 3 이 L178/L193 tower_bonus 로도 쓰임(별 항목) | 4 |
| 2 | 7 | 164 | 태그 | tag≤2 → idx 7(AroundPosition). 판정 아님 | 4 |
| 3 | -1 | 170 | 센티널 | Option<Effect> None 니치 — attack/skill/skill2_effect unwrap 검사 | 4 |
| 4 | 2 | 171 | 임계 | MinionActionType::Push(=2) — calculate_action_score 10번째 인자(i8 2) 세 arm 공통. DI 지역상수 action_type=i8 2. (LineSafe/LineWait 는 Pull=0) | 4 |
| 5 | -99999 | 169 | 산출값 | Attack/Skill/Skill2 의 target 이 안 풀리면 가산 -99999 (base 는 더해짐) | 4 |
| 6 | 2 | 178 | 태그 | EntityType::Tower 메모리태그 — t.is_tower()(entity.rs:1386) ⟹ tower_bonus. L193 도 동일 | 4 |
| 7 | 3 | 178 | 태그 | tower_bonus: Skill/Skill2 대상이 타워면 +3, 아니면 0 (select). L193 동일. ★Attack arm 은 미적용 | 4 |
| 8 | 2 | 194 | 임계 | champ.level > 2 여야 skill2_effect Some(entity.rs:1693) — 아니면 unwrap_failed | 4 |
| 9 | 0 | 201 | 태그 | Around arm 값 0 — get_entity_by_id 를 호출하지만 결과(t)를 읽지 않고 0 (Some/None 양쪽 0) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 타워 대상 스킬 가산 | attack_nexus.rs:178 (Skill) · :193 (Skill2) | 3 | 올리면 넥서스 공략 중 타워에 스킬을 쓰는 후보가 평타/다른 대상보다 우선. Attack arm 엔 적용되지 않으므로 평타 타워 딜엔 영향 없음 | 4 | 기존 |
| 1 | 미니언 액션 타입 | attack_nexus.rs:171/185/195 | 2 | Push 고정 → calculate_action_score 내부 미니언 관련 분기가 푸시 기준. Pull(0)/Normal(1)로 바꾸면 라인 관리 성향이 바뀜 | 4 | 기존 |
| 2 | 대상 소실 페널티 | attack_nexus.rs:169/177/191 | -99999 | 완화하면 사라진 대상 후보가 남음 | 4 | 기존 |

<details><summary>`callees` 피호출자 14건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_speed_mult | game_core::Entity::attack_speed_mult | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:2433 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | calculate_action_score | game_ai::calculate_action_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, usize, &game_core::Entity, game_ai::MinionActionType, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:8 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | cooldown_reduce | game_core::Entity::cooldown_reduce | pub | fn(&game_core::Entity, bool) -> usize | game-core\src\simulation\entity.rs:2437 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 4 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 5 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 6 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_tower | game_core::EntityType::is_tower | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1385 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 12 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 13 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
</details>

**호출처 1곳** (m12.ll:37310) · **형제 9개** (AttackNexusSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::AttackNexusSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan) -> game_ai::plan_legacy::sub_plan::AttackNexusSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::AttackNexusSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:5 | True | fn() -> game_ai::plan_legacy::sub_plan::AttackNexusSubPlan |
| 2 | <game_ai::plan_legacy::sub_plan::AttackNexusSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::attack_nexus | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:11 | False | fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::attack_nexus | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:21 | False | fn(&mut game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::attack_nexus | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:76 | False | fn(&mut game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:102 | True | fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:118 | False | fn(&mut game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 8 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:158 | False | fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | Around arm(L201~L210) 본문: get_entity_by_id 호출 뒤 t 를 전혀 읽지 않고 0 — 소스가 `if let Some(t) = … { 0 } else { 0 }` 인지, t 를 읽는 식이 상수 접힘으로 사라진 것인지 표기 불가(줄길이 L202=36B·한글0 만 관측). 동작 확정: 값 0. | 3 |  |
| 1 | 미탐색 | Attack arm 에 tower_bonus 가 없는 것이 의도인지 — 소스 부재. IR 상 L169~L171 에 ty@tag 읽기 없음(동작 확정). | 4 |  |
| 2 | 미탐색 | entity.rs:1386 is_tower 가 `ty@tag==2` 만 보는지(추가 조건 없음) — IR 상 그 비교 하나뿐(동작 확정, 이름은 dloc 로 확인). | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | AbstractGame vtable+0x1f0 get_entity_by_id 는 ExpectedGame 정적 vtable 기준 — 런타임 구현체 미확정. | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

