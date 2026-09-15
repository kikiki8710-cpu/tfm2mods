---

### `234` JungleSubPlan::score — Jungle 서브플랜의 후보 액션 점수: interaction_score 기반 + Attack/Skill/Skill2 는 calculate_jungle_action_score 가산, 대상이 (팀)정글몹이면 최소 1 보장, 대상 소실=-99999, 그 외 액션은 base 그대로

| 항목 | 값 |
|---|---|
| id | `jungle__Jungle__score` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6jungleNtB2_13JungleSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\jungle.rs:152` |
| IR | `m02.ll` 39903~40149행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::JungleSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `cc5a10` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[234]/sig/tls/<키>`)**

없음 — LocalKey::with / @anon…call_once fn-포인터 참조 0 (39903~40149 전수)

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &JungleSubPlan(16B) — {team:usize@0x0, camp:JungleType@0x8, check_move:bool@0x9} |  | 4 |
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
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // jungle.rs:152 · self 미사용
let champ = data.cache.player_champion[player.info.team][player.info.position as usize].unwrap();   // L153
let mut base = interaction_score(version, rnd, player, data, parameter, action, debug);   // L154
match action.get_action() {   // L156 · SmallActionPlay 태그 switch, 12/13/14 만 arm
    Attack{target_id} => {   // idx 12
        if let Some(t) = data.cache.game.get_entity_by_id(target_id) {   // L158
            let score = base + calculate_jungle_action_score(rnd, player, data, parameter, &champ.attack /*+0x570*/, champ.attack_effect.as_ref().unwrap() /*+0x4c0/+0x490*/, t);   // L159
            if t.is_jungle() /*L160 · ty@tag==4 && ty.info.camp_type.0(+0x98) < 2*/ { base = max(score, 1); /*L161*/ } else { base = score; }
        } else { return -99999; }   // L158 else — phi 직행, base 미가산
    }
    Skill{target_id} => {   // idx 13 · L170~L173 동형: &champ.skill(+0x580) · skill_effect(+0x4f8/+0x4c8)
        Some(t) → score = base + calculate_jungle_action_score(..., &champ.skill, skill_effect.unwrap(), t) /*L171*/; base = t.is_jungle() ? max(score,1) /*L173*/ : score
        None → return -99999
    }
    Skill2{target_id} => {   // idx 14 · L182~L185
        Some(t) → let (act, eff) = champ.skill2() /*entity.rs:1669: level(+0x5c8)>2 ? (&skill2 +0x590, &skill2_effect +0x500) : (&empty +0x5b0, &None)*/;
                  score = base + calculate_jungle_action_score(..., act, eff.as_ref().unwrap() /*level≤2 면 여기서 unwrap_failed*/, t) /*L183*/; base = t.is_jungle() ? max(score,1) /*L185*/ : score
        None → return -99999
    }
    _ => {}   // RunAway·Recall·Around·AroundHide·AroundRegion·AroundRunAway·Positioning·AroundPosition·AroundPositionBush·AroundBush·LaneMinionPosition·Trace·Ult·Stop → base 유지
}
return base;   // L195

★형제 대조(복붙 의심): LineSafe/LineWait/AttackNexus 는 calculate_action_score(version, rnd, … speed_mult/cooldown_reduce, ty) 를 쓰는데 Jungle 은 calculate_jungle_action_score(rnd, player, data, parameter, action, effect, t) 로 콜리가 다르고 attack_speed_mult/cooldown_reduce 호출이 없다. 대상 소실 시 형제들은 base+(-99999) 인데 여기만 -99999 단독 반환(phi 에 %24 미포함 — IR 40065). Around 계열 arm 이 없다(형제는 Around 에 가산/호출이 있음). evaluate_action(ActionContext) 신경로 없음.
rnd gen_range 사이트: 0(interaction_score 1회 전달이 유일한 소비 가능 지점 · calculate_jungle_action_score 는 readnone).
version 분기 0 · debug 직접 쓰기 0 · self 읽기 0.
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L153 · player_champion[team] · <2 bounds check | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag(i32) | r | L153(player.rs:581 인라인) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | L153 | 4 | OK |
| 3 | AbstractGameWithCache | 0x0 | game.data_ptr | r | L158/170/182 | 4 | OK |
| 4 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x1f0(=496) get_entity_by_id 슬롯 3곳 | 4 | OK |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | L153 · null → unwrap_failed | 4 | OK |
| 6 | SmallActionPlay | 0xb1 | tag(u8) | r | L156(small_action.rs:309 get_action 인라인) · idx 12/13/14 만 분기, 나머지 → base | 4 | OK |
| 7 | SmallActionPlay | 0x8 | target_id | r | Attack(cast.rs:94)/Skill(:160)/Skill2(:222) | 4 | 오귀속(사전은 다른 필드를 준다) |
| 8 | Entity(champ) | 0x4c0 | attack_effect@tag(-1=None) | r | L159(option.rs:742) · None → unwrap_failed | 4 | OK |
| 9 | Entity(champ) | 0x490 | attack_effect@Some.0 | r | L159 effect 인자 | 4 | OK |
| 10 | Entity(champ) | 0x570 | attack (Box<dyn Action>) | r | L159(entity.rs:1494) · calculate_jungle_action_score 의 `_action`(콜리 쪽 미사용 이름) | 4 | OK |
| 11 | Entity(champ) | 0x4f8 | skill_effect@tag | r | L171 | 4 | OK |
| 12 | Entity(champ) | 0x4c8 | skill_effect@Some.0 | r | L171 | 4 | OK |
| 13 | Entity(champ) | 0x580 | skill (Box<dyn Action>) | r | L171(entity.rs:1665) | 4 | OK |
| 14 | Entity(champ) | 0x5c8 | level | r | L183(entity.rs:1669 Entity::skill2 인라인) · level>2 ? (&skill2, &skill2_effect) : (&empty, &None) | 4 | OK |
| 15 | Entity(champ) | 0x530 | skill2_effect@tag | r | L183 · level>2 경로. ★C3 경고 사유: 본문엔 1328 리터럴이 없고 `select(level>2, champ+1280, @anon.19)` 뒤 `+48` 로 접혀 있음(0x500+0x30=0x530). level≤2 면 @anon.19(정적 None, +48=-1) 를 읽어 항상 unwrap_failed | 4 | OK |
| 16 | Entity(champ) | 0x500 | skill2_effect@Some.0 | r | L183 | 4 | OK |
| 17 | Entity(champ) | 0x590 | skill2 (Box<dyn Action>) | r | L183 · select(level>2, 0x590, 0x5b0) | 4 | OK |
| 18 | Entity(champ) | 0x5b0 | empty (Box<dyn Action>) | r | L183 · level≤2 대체 액션 — 그 경로는 effect None 으로 먼저 패닉하므로 실질 도달 불가(가드 아님) | 4 | OK |
| 19 | Entity(t) | 0x68 | ty@tag (EntityType) | r | L160/172/184(entity.rs:1378 is_jungle 인라인) · ==4(Jungle) | 4 | OK |
| 20 | Entity(t) | 0x98 | ty@Jungle.info.camp_type.0 (usize) | r | L160/172/184 · is_jungle 의 두 번째 조건 `< 2` (Jungle+0x28 (usize, JungleType) 의 .0) | 4 | OK |
| 21 | AbstractGame vtable | 0x1f0 | get_entity_by_id(&self, usize)->Option<&Entity> | r | 간접 호출 3곳 — 심볼 호출 아님 | 4 | 확인불가(vtable 슬롯) |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 10 | 156 | 센티널 | SmallActionPlay 니치 구멍 assume(tag≠10). 판정 아님 | 4 |
| 1 | 3 | 156 | 태그 | 태그→idx 환산(tag-3). 판정 아님 | 4 |
| 2 | 7 | 156 | 태그 | tag≤2 → idx 7(AroundPosition). 판정 아님 | 4 |
| 3 | -1 | 159 | 센티널 | Option<Effect> None 니치(i32 -1) — attack/skill/skill2_effect unwrap 검사 | 4 |
| 4 | -99999 | 158 | 산출값 | Attack/Skill/Skill2 의 target_id 가 안 풀리면 반환값 자체가 -99999 (★base 를 더하지 않음 — LineSafe/AttackNexus 는 base+(-99999)) | 4 |
| 5 | 4 | 160 | 태그 | EntityType::Jungle 메모리태그 — t.is_jungle() 첫 조건(entity.rs:1378) | 4 |
| 6 | 2 | 160 | 임계 | is_jungle 둘째 조건: t.ty.info.camp_type.0 < 2 (entity.rs:1378 인라인). 별도로 L153 `ult team,2` 는 bounds check, L183 `level > 2` 는 skill2 해금 레벨 | 4 |
| 7 | 1 | 161 | 태그 | 대상이 is_jungle 이면 base = max(score, 1) — llvm.smax(score,1): 정글몹 상대 액션은 최소 1점 보장(음수 억제) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 정글몹 대상 액션 최소 점수 | jungle.rs:161 (Skill 173 · Skill2 185) | 1 | 올리면 정글몹 공격/스킬 후보가 음수·0 점수여도 이 값으로 바닥이 올라가 다른 0점 후보(대기 등)보다 항상 우선. 내리거나(0) 없애면 정글몹 상대 손해 액션이 밀릴 수 있음 | 4 | 기존 |
| 1 | is_jungle 캠프 분류 임계 | entity.rs:1378 (jungle.rs:160 인라인) | 2 | camp_type.0 < 2 인 정글몹만 최소점 보장 대상. 올리면 더 많은 캠프 종류(추정: 상위 캠프/오브젝트)가 보장 대상에 포함 — 단 이 값은 game_core 헬퍼 안이라 이 함수에서 못 바꿈 | 5 | 기존 |
| 2 | 대상 소실 반환값 | jungle.rs:158/170/182 | -99999 | 완화하면 죽은/사라진 대상 후보가 살아남음. base 미가산이라 형제 플랜보다 절대값 기준이 다름 | 4 | 기존 |

<details><summary>`callees` 피호출자 24건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | calculate_action_score | game_ai::calculate_action_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, usize, &game_core::Entity, game_ai::MinionActionType, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:8 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | calculate_jungle_action_score | game_ai::calculate_jungle_action_score | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, &game_core::Entity) -> i64 | game-ai\src\action_score.rs:520 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | evaluate_action | game_ai::plan_legacy::action_eval::evaluate_action | pub | fn(usize, &game_ai::plan_legacy::action_eval::ActionContext, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> std::option::Option<i64> | game-ai\src\plan_legacy\action_eval.rs:67 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 4 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 5 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 6 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_jungle | game_core::EntityType::is_jungle | pub | fn(&game_core::EntityType, usize) -> bool | game-core\src\simulation\entity.rs:1377 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 15 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 16 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 17 | skill | game_ai::skill | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:199 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 18 | skill | game_core::Entity::skill | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1664 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 19 | skill | game_core::ChampionInfo::skill | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1085 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 20 | skill2 | game_ai::skill2 | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:239 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 21 | skill2 | game_core::Entity::skill2 | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1668 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 22 | skill2 | game_core::ChampionInfo::skill2 | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1086 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 23 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 1개**: `llvm.smax.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:37254) · **형제 9개** (JungleSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::JungleSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::JungleSubPlan) -> game_ai::plan_legacy::sub_plan::JungleSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::JungleSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::JungleSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::JungleSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:15 | True | fn(usize, game_core::JungleType) -> game_ai::plan_legacy::sub_plan::JungleSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::JungleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::jungle | game-ai\src\plan_legacy\sub_plan\jungle.rs:19 | False | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> game_ai::SmallActionPlay |
| 4 | game_ai::plan_legacy::sub_plan::JungleSubPlan::attack_jungle_action | in:game_ai::plan_legacy::sub_plan::jungle | game-ai\src\plan_legacy\sub_plan\jungle.rs:54 | False | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::JungleSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:107 | False | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::JungleSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:152 | False | fn(&game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 7 | game_ai::plan_legacy::sub_plan::JungleSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:197 | True | fn(&game_ai::plan_legacy::sub_plan::JungleSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 8 | game_ai::plan_legacy::sub_plan::JungleSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:214 | True | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, game_ai::plan_legacy::sub_plan::JungleSubPlan) |

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | entity.rs:1378 is_jungle 의 `camp_type.0 < 2` 가 무슨 분류인지(팀 진영 0/1 vs 중립 2? · 캠프 등급?) — game_core 헬퍼 인라인만 보임. 미탐색 = _gcbc 의 is_jungle define/주석 · Jungle::camp_type 생성처. | 4 |  |
| 1 | 표기 불가 | L158/170/182 `else { -99999 }` 가 소스에서 `return -99999` 인지 `base = -99999` 후 return 인지 — 외연 동일(반환 -99999). 표기 불가. | 4 |  |
| 2 | 미탐색 | Skill2 arm 에서 level≤2 경로(empty+0x5b0 / None) 가 실제 도달하는지 — 정적으로는 unwrap_failed 확정. 후보 생성기(action_candidates)가 level≤2 에서 Skill2 를 안 만드는지는 이 함수 밖(r16 명세 범위). | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | AbstractGame vtable+0x1f0 = get_entity_by_id 는 ExpectedGame 정적 vtable 기준(divtable) — 런타임 구현체 미확정. | 3 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | self(JungleSubPlan: team/camp/check_move) 를 전혀 읽지 않는 이유 — 소스 부재. 동작 확정(load 0). | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

