---

### `230` LineSafeSubPlan::score — LineSafe 서브플랜의 후보 액션 점수: evaluate_action(Lane 우선순위·Lane 앵커) Some 이면 그 값, None 이면 interaction_score+경제보정+액션별 가산(Attack/Skill/Skill2=calculate_action_score, Around 아군구조물 원거리=50/100, 대상 소실=-99999)

| 항목 | 값 |
|---|---|
| id | `line_safe__LineSafe__score` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9line_safeNtB2_15LineSafeSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\line_safe.rs:104` |
| IR | `m02.ll` 48427~48748행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::LineSafeSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `ccbb10` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[230]/sig/tls/<키>`)**

없음 — 본문에 LocalKey::with / @anon…call_once fn-포인터 상수 참조 0 (48427~48748 전수)

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &LineSafeSubPlan(1B) — {line: LineType@+0x0} |  | 4 |
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
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // line_safe.rs:104
// L105: 신경로(액션 평가기) 우선
let ctx = ActionContext { priority: PriorityProfile::Lane /*tag 2*/, anchor: Anchor::Lane { line: self.line } /*+0x10=1, +0x11=line*/ };
if let Some(v) = evaluate_action(version, &ctx, parameter, rnd, player, data, action, debug) {   // 반환 {i64 tag, i64 v}, tag bit0=1 이면 Some
    return v;   // L167
}
// L108: 구경로
let champ = data.cache.player_champion[player.info.team /*+0x930, <2*/][player.info.position as usize /*+0x9c0*/].unwrap();   // None → option::unwrap_failed
let base = interaction_score(version, rnd, player, data, parameter, action, debug);   // L109
let economy_adjustment = line_action_economy_adjustment(version, player, data, parameter, action, MinionActionType::Pull /*i8 0*/);   // L113
// L114: 액션별 가산 — action.get_action() 인라인 → SmallActionPlay 태그(+0xb1) switch
let add = match action.get_action() {
    // idx 12 Attack{target_id=+0x8}  (L118)
    Attack{target_id} => if let Some(t) = data.cache.game.get_entity_by_id(target_id) /*vtable+0x1f0*/ {
            let effect = champ.attack_effect.as_ref().unwrap();   // L119 · +0x4c0 tag≠-1, payload +0x490
            calculate_action_score(version, rnd, player, data, parameter, &champ.attack /*+0x570*/, effect, champ.attack_speed_mult(), t, Pull /*i8 0*/, debug)   // L120
        } else { -99999 },
    // idx 13 Skill{target_id}  (L126)
    Skill{target_id} => if let Some(t) = get_entity_by_id(target_id) {
            let effect = champ.skill_effect.as_ref().unwrap();   // L127 · +0x4f8 / +0x4c8
            calculate_action_score(version, rnd, player, data, parameter, &champ.skill /*+0x580*/, effect, champ.cooldown_reduce(false), t, Pull, debug)   // L128
        } else { -99999 },
    // idx 14 Skill2{target_id}  (L134)
    Skill2{target_id} => if let Some(t) = get_entity_by_id(target_id) {
            let effect = (if champ.level /*+0x5c8*/ > 2 { champ.skill2_effect.as_ref() } else { None }).unwrap();   // L135 entity.rs:1693 · +0x530 / +0x500 · level≤2 또는 None 이면 unwrap_failed
            calculate_action_score(version, rnd, player, data, parameter, &champ.skill2 /*+0x590*/, effect, champ.cooldown_reduce(false), t, Pull, debug)   // L136
        } else { -99999 },
    // SmallAction::Around{target_id} ← SmallActionPlay idx 2 Around / 3 AroundHide / 10 LaneMinionPosition (모두 payload+0x8 = target)  (L142)
    Around{target_id} => if let Some(t) = get_entity_by_id(target_id) {
            if t.team == champ.team /*L143 · TeamType derived eq: 태그(+0x0) 같고, 둘 다 Player(0) 면 +0x8 도 같아야*/ {
                let dist = (|t.x-champ.x|)^2 + (|t.y-champ.y|)^2;   // L144 · +0x660/+0x668 · u64 abs-diff 후 제곱합
                if dist > 39999999999 /*L147 · ≥200000² = 6.25셀*/ {
                    match t.ty /*+0x68*/ { Minion(1) | Tower(2) => 50, Nexus(3) => 100, _ => 0 }   // L148~L150
                } else { 0 }
            } else { 0 }
        } else { 0 },   // Around 대상 소실은 0 (Attack 계열의 -99999 와 다름)
    _ => 0,   // RunAway(0) Recall(1) AroundRegion(4) AroundRunAway(5) Positioning(6) AroundPosition(7) AroundPositionBush(8) AroundBush(9) Trace(11) Ult(15) Stop(16)
};
return base + economy_adjustment + add;   // L114 add 순서: (economy+base)+add · L167

★복붙 대조: LineWaitSubPlan::score(line_wait.rs:159, m15.ll 22517~22838) 와 **IR 완전 동일**(레지스터명·attr 번호 제외 diff 0 · rmeta 줄길이 104~168 ↔ 159~223 전 줄 일치, 유일한 차이 = 주석줄 115(170B/한글37) vs 170(204B/한글40)). PriorityProfile::Lane·Anchor::Lane·200000²·50/100·Pull 전부 같은 값 — 플랜 고유값 0.
rnd gen_range 사이트: 0 (콜리 전달만 · 순서 = evaluate_action → interaction_score → calculate_action_score 1회).
version 분기: 0. debug 직접 쓰기: 0.
```

**`mem` 메모리 접근 28건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LineSafeSubPlan | 0x0 | line | r | L105 · ActionContext.anchor = Anchor::Lane{line} 의 페이로드(+0x11)로 복사 | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | L108 · player_champion[team] 1차 인덱스 · <2 bounds check(panic_bounds_check) | 4 | OK |  |
| 2 | PlayerState | 0x9c0 | info.position@tag(i32) | r | L108(player.rs:581 인라인) · zext 후 player_champion[team][pos] 2차 인덱스 | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | L108 · &AbstractGameWithCache | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x0 | game.data_ptr | r | L118/126/134/142 · dyn AbstractGame 팻포인터 데이터 | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | L118/126/134/142 · vtable+0x1f0(=496) = AbstractGame::get_entity_by_id 슬롯(divtable 확인) | 3 | OK |  |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | L108 · Option<&Entity> — null 이면 unwrap_failed(entity.rs:1011/option.rs:1013) | 4 | OK |  |
| 7 | SmallActionPlay | 0xb1 | tag(u8, niche) | r | L114(small_action.rs:309 get_action 인라인) · idx = tag>2 ? tag-3 : 7(AroundPosition 암묵) · llvm.assume(tag≠10) | 4 | OK |  |
| 8 | SmallActionPlay | 0x8 | payload.target / target_id | r | Around(idx2)/AroundHide(3)/LaneMinionPosition(10)=SmallActionAround*.target · Attack(12)=cast.rs:94 · Skill(13)=cast.rs:160 · Skill2(14)=cast.rs:222 — 전부 +0x8 | 4 | OK |  |
| 9 | Entity(champ) | 0x4c0 | attack_effect@tag(i32, -1=None) | r | L119(option.rs:742 as_ref) · None 이면 unwrap_failed | 4 | OK |  |
| 10 | Entity(champ) | 0x490 | attack_effect@Some.0 (&Effect) | r | L120 · calculate_action_score 의 effect 인자 | 4 | OK |  |
| 11 | Entity(champ) | 0x570 | attack (Box<dyn Action>) | r | L120(entity.rs:1494 인라인) · calculate_action_score 의 action 인자 | 4 | OK |  |
| 12 | Entity(champ) | 0x4f8 | skill_effect@tag | r | L127 | 4 | OK |  |
| 13 | Entity(champ) | 0x4c8 | skill_effect@Some.0 | r | L128 | 4 | OK |  |
| 14 | Entity(champ) | 0x580 | skill (Box<dyn Action>) | r | L128(entity.rs:1665 인라인) | 4 | OK |  |
| 15 | Entity(champ) | 0x5c8 | level | r | L135(entity.rs:1693 인라인) · level>2 아니면 skill2_effect 가 None 취급 → unwrap_failed | 4 | OK |  |
| 16 | Entity(champ) | 0x530 | skill2_effect@tag | r | L135 | 4 | OK |  |
| 17 | Entity(champ) | 0x500 | skill2_effect@Some.0 | r | L135(entity.rs:1694) | 4 | OK |  |
| 18 | Entity(champ) | 0x590 | skill2 (Box<dyn Action>) | r | L136(entity.rs:1670 인라인) | 4 | OK |  |
| 19 | Entity(champ, t) | 0x0 | team@tag (TeamType: 0=Player,1=Neutral) | r | L143(entity.rs:1127 derived PartialEq) · t.team == champ.team | 4 | OK |  |
| 20 | Entity(champ, t) | 0x8 | team@Player.0 (usize) | r | L143 · 태그가 둘 다 0(Player) 일 때만 비교 | 4 | OK |  |
| 21 | Entity(champ, t) | 0x660 | x (u64) | r | L144(entity.rs:2158 거리제곱 인라인) | 4 | OK |  |
| 22 | Entity(champ, t) | 0x668 | y (u64) | r | L144 | 4 | OK |  |
| 23 | Entity(t) | 0x68 | ty@tag (EntityType) | r | L148(entity.rs:1261 is_any_type_minion 인라인 · dloc 확인) · switch: 1=Minion,2=Tower→50 · 3=Nexus→100 · 그외→0. 1261 스코프는 switch 명령 하나에 붙어 있어 Tower 판정이 is_any_type_minion 안인지 별도 is_tower(entity.rs:1386) 인지 IR 로 구분 불가 | 4 | OK |  |
| 24 | AbstractGame vtable | 0x1f0 | get_entity_by_id(&self, id:usize)->Option<&Entity> | r | 간접 호출 4곳(L118/126/134/142) — 심볼 호출이 아니라 calls 에 없음 | 4 | 확인불가(vtable 슬롯) |  |
| 25 | ActionContext(스택 alloca %9, 40B) | 0x0 | priority@tag | w | L105 · evaluate_action 의 ctx 인자. &mut 인자(rnd/debug)에는 본문 직접 쓰기 0 · initializes 속성 없음 | 4 | OK | 2 (= PriorityProfile::Lane 메모리태그 · tcxdict --enum: niche_start=2, Battle 암묵) |
| 26 | ActionContext(스택 alloca %9, 40B) | 0x10 | anchor@tag | w | L105 | 4 | OK | 1 (= Anchor::Lane, Direct 인코딩) |
| 27 | ActionContext(스택 alloca %9, 40B) | 0x11 | anchor@Lane.line | w | L105 · +0x8(priority 페이로드)·+0x18~ 은 미기록(undef) — 콜리가 Lane 태그에서는 안 읽는 슬롯 | 4 | OK | self.line (LineType u8) |

**`consts` 상수 15건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 105 | 태그 | PriorityProfile::Lane 의 메모리태그(store i64 2 → ActionContext+0x0). 별도로 L108 의 `icmp ult team, 2` 는 [2] 배열 bounds check(임계 아님) | 4 |
| 1 | 1 | 105 | 태그 | Anchor::Lane 의 태그(store i8 1 → ActionContext+0x10) | 4 |
| 2 | 10 | 114 | 센티널 | SmallActionPlay 니치 구멍(AroundPosition 암묵 variant 자리) — llvm.assume(tag≠10). 판정 아님 | 4 |
| 3 | 3 | 114 | 태그 | SmallActionPlay 태그→논리 idx 환산(tag-3, niche_start=3). 판정 아님 | 4 |
| 4 | 7 | 114 | 태그 | tag≤2 이면 idx 7 = AroundPosition(untagged). 판정 아님 | 4 |
| 5 | -1 | 119 | 센티널 | Option<Effect> None 니치(casting@tag i32 = -1) — attack/skill/skill2_effect 의 as_ref().unwrap() 검사 | 4 |
| 6 | 0 | 113 | 태그 | MinionActionType::Pull(=0) — line_action_economy_adjustment 6번째 인자(i8 0) 와 calculate_action_score 10번째 인자(ty) 둘 다 Pull. DI 지역상수 action_type=i8 0 | 4 |
| 7 | -99999 | 118 | 산출값 | Attack/Skill/Skill2 의 target_id 가 get_entity_by_id 로 안 풀리면(대상 소실) 가산값 = -99999 (L118·L126·L134 세 arm 공통) | 4 |
| 8 | 39999999999 | 147 | 임계 | 거리제곱 임계: dist² > 39999999999 ⟺ dist ≥ 200000 (= 6.25셀). Around 계열 액션의 대상이 같은 팀이고 이만큼 멀 때만 구조물 가산 | 4 |
| 9 | 1 | 148 | 태그 | EntityType::Minion 태그 → 가산 50 | 4 |
| 10 | 2 | 148 | 태그 | EntityType::Tower 태그 → 가산 50 | 4 |
| 11 | 3 | 148 | 태그 | EntityType::Nexus 태그 → 가산 100 | 4 |
| 12 | 50 | 148 | 산출값 | Around 대상이 아군 Minion/Tower 이고 dist≥200000 일 때 가산 | 4 |
| 13 | 100 | 150 | 산출값 | Around 대상이 아군 Nexus 이고 dist≥200000 일 때 가산 (그 외 EntityType → 0) | 4 |
| 14 | 2 | 135 | 임계 | champ.level > 2 여야 skill2_effect 를 Some 으로 봄(entity.rs:1693 인라인) — 아니면 unwrap_failed(패닉). Skill2 후보가 level≤2 에서 생성되면 크래시 경로 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Around 구조물 가산의 거리 임계 | line_safe.rs:147 | 39999999999 | 내리면 더 가까운 아군 구조물 주변 대기도 50/100 가산을 받아 구조물 근처 배회 선호가 강해짐. 올리면 멀리 떨어진 구조물로 갈 때만 가산 | 4 | 기존 |
| 1 | 아군 Minion/Tower 주변 대기 가산 | line_safe.rs:148 | 50 | 올리면 LineSafe 중 원거리 아군 미니언/타워 쪽 Around 후보가 더 자주 선택됨 (헬퍼: is_any_type_minion — Tower 도 같은 50) | 4 | 기존 |
| 2 | 아군 Nexus 주변 대기 가산 | line_safe.rs:150 | 100 | 올리면 넥서스로 후퇴 대기 선호↑ (Minion/Tower 50 의 2배) | 4 | 기존 |
| 3 | 대상 소실 페널티 | line_safe.rs:118/126/134 | -99999 | Attack/Skill/Skill2 의 target 이 사라진 후보를 사실상 배제. 값을 완화하면 죽은 대상 공격 후보가 살아남을 수 있음 | 4 | 기존 |
| 4 | 경제 보정 미니언 액션 타입 | line_safe.rs:113 | 0 | MinionActionType::Pull 고정. Normal(1)/Push(2)로 바꾸면 line_action_economy_adjustment 의 라인 경제 판정 기준이 바뀜(AttackNexus 는 Push=2 사용) | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_speed_mult | game_core::Entity::attack_speed_mult | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:2433 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | calculate_action_score | game_ai::calculate_action_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, usize, &game_core::Entity, game_ai::MinionActionType, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:8 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | cooldown_reduce | game_core::Entity::cooldown_reduce | pub | fn(&game_core::Entity, bool) -> usize | game-core\src\simulation\entity.rs:2437 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | evaluate_action | game_ai::plan_legacy::action_eval::evaluate_action | pub | fn(usize, &game_ai::plan_legacy::action_eval::ActionContext, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> std::option::Option<i64> | game-ai\src\plan_legacy\action_eval.rs:67 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 5 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 6 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 7 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | line_action_economy_adjustment | game_ai::line_action_economy_adjustment | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, game_ai::MinionActionType) -> i64 | game-ai\src\lane_economy.rs:6 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 13 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 14 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
</details>

**호출처 1곳** (m12.ll:37237) · **형제 9개** (LineSafeSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::LineSafeSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan) -> game_ai::plan_legacy::sub_plan::LineSafeSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::LineSafeSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:11 | True | fn(game_core::LineType) -> game_ai::plan_legacy::sub_plan::LineSafeSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_safe | game-ai\src\plan_legacy\sub_plan\line_safe.rs:15 | False | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_safe | game-ai\src\plan_legacy\sub_plan\line_safe.rs:44 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_safe | game-ai\src\plan_legacy\sub_plan\line_safe.rs:48 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:71 | True | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:89 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 8 | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\line_safe.rs:104 | False | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L105 `if let Some(v)` 의 소스 표기(if let / map_or / ? 등) — IR 은 {i64,i64} 의 bit0 검사 뒤 즉시 반환. 동작 확정, 표기 불가(column 부재). | 4 |  |
| 1 | 미탐색 | L114 match arm 의 소스 순서(Attack→Skill→Skill2→Around 인지) — switch 로 접혀 순서 근거는 arm 루트 줄번호(118<126<134<142)뿐. 동작엔 무관. | 4 |  |
| 2 | 표기 불가 | L148 헬퍼 = entity.rs:1261 is_any_type_minion(dloc 확인, define 없음=전부 인라인) — 태그 1(Minion)·2(Tower) 가 같은 50 으로 접혀 있어 `is_any_type_minion()` 이 Tower 까지 포함하는지, `\|\| t.is_tower()` 가 붙은 것인지 표기 불가(줄길이 64B 는 16칸 들여쓰기 + `if t.is_any_type_minion() \|\| t.is_tower() { 50 }`(48자) 와 일치 — 추정). 100 은 Nexus(3) · 0 은 그 외 — 동작 확정. | 3 |  |
| 3 | 재료 부재 | AbstractGame vtable+0x1f0 get_entity_by_id 는 divtable(ExpectedGame 정적 vtable) 기준 — 런타임 구현체는 도구로 확정 불가(계약: (&self, usize)->Option<&Entity>). | 3 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L143 `t.team == champ.team` 이 소스에서 `==` 인지 헬퍼(예: is_same_team) 인지 — entity.rs:1127 PartialEq 인라인만 보임. 동작 확정. | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

