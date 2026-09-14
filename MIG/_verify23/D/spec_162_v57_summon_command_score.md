---

### `162` v57_summon_command_score — 소환수 명령 액션(환술사 궁/네크로맨서 Q·W·R) 4종의 점수 — 근처 가시 적 유무·대상 팀·구울 수로 0..90 산출, 그 외 액션은 None

| 항목 | 값 |
|---|---|
| id | `buff_value__v57_summon_command_score` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai10buff_value24v57_summon_command_score` |
| 소스 | `game-ai\src\buff_value.rs:785` |
| IR | `m10.ll` 38354~38672행 |
| 경로·가시성 | `game_ai::buff_value::v57_summon_command_score` · **in:game_ai** |
| 계층 | 점수화·술어 |
| exe | `e04400` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Entity) -> std::option::Option<i64>
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | data | &OperationData(24B) | +0 cache(&AbstractGameWithCache) → 캡처·others / +8 context(&GameContext) → expected_damage_target 인자 / +0x10 blackboard(&[Blackboard;2]) → 캡처 | 4 |
| 1 | 2 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽음(본체 38488 · 클로저 7241) | 4 |
| 2 | 3 | champ | &Entity(1728B) | 내 챔피언. team(+0/+8) 비교 및 클로저 around 인자 | 4 |
| 3 | 4 | action | &Box<dyn Action>(16B 팻포인터: +0 data / +8 vtable) | vtable+0x68 = Action::as_any → &dyn Any 팻포인터, 그 vtable+0x18 = Any::type_id(sret 16B TypeId). is::<T>() 4회 | 4 |
| 4 | 5 | t | &Entity(1728B) | 명령 대상 엔티티. team·attack_effect·attack_cooltime 읽음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v57_summon_command_score(data, player, champ, action:&Box<dyn Action>, t:&Entity) -> Option<i64>
// L787~791 클로저 near_visible_enemy(around:&Entity) -> bool  [aux m10.ll:7228~7652]
//   enemy = 1 - player.info.team (bounds<2)
//   cache.player_champion[enemy].iter().flatten()   // 5칸, null=None 건너뜀
//     .any(|c| (|c.x-around.x|² + |c.y-around.y|²) < 40000000001   // dist<=200000
//              && blackboard[enemy].is_recent_visible(cache.game, player, c))
//   (5칸이 완전 언롤: 7292~7638, 첫 true 에서 단락)

any = action.as_any()   // vtable+0x68
// L794
if any.is::<IllusionistUltAction>() {
    // L795
    if t.team != champ.team { return Some(0) }   // TeamType eq: 태그 다름 → ne / 둘 다 Player 면 idx 비교 / 둘 다 Neutral 면 eq
    // L796~798
    dps = t.attack_effect.map(|eff| eff.expected_damage_target(data.context, t as &dyn AbstractEntity, t) * 1000 / max(t.attack_cooltime(), 1)).unwrap_or(10)
    // L800
    atk_power = clamp(dps / 40, 10, 90)     // None 경로는 상수 10 으로 접힘(phi 38667)
    return Some(if near_visible_enemy(t) { atk_power } else { 5 })
}
// L804
else if any.is::<NecromancerSkillAction>() {
    // L805
    return Some(if near_visible_enemy(champ) { 25 } else { 8 })
}
// L809
else if any.is::<NecromancerSkill2Action>() {
    // L810~811
    ghouls = cache.others[player.info.team].iter().filter(|e| e.ty tag == 7 /*Ghoul*/).count()   // team>=2 → bounds panic
    // L812
    if ghouls == 0 { return Some(-100) }
    // L813
    if t.team == champ.team { return Some(0) }
    // L814
    return Some(min(ghouls * 18, 60))
}
// L818
else if any.is::<NecromancerUltAction>() {
    // L819
    return Some(if near_visible_enemy(champ) { 90 } else { 30 })
}
// L823
else { return None }   // 값 슬롯 undef

분기 순서 = 소스 줄 794→804→809→818 (TypeId 검사 체인, 각 검사마다 as_any 재호출 38389·38409·38443·38468).
```

**`mem` 메모리 접근 19건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache — 클로저 캡처 +0 및 others 접근 (38376·38507) | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext — expected_damage_target 2번째 인자 (38643) | 4 | OK |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — 클로저 캡처 +8 (38377) | 4 | OK |
| 3 | Box<dyn Action> | 0x0 | data_ptr | r | 38384 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 4 | Box<dyn Action> | 0x8 | vtable_ptr | r | 38385~38388; vtable+0x68(104) 슬롯 = as_any (divtable Action 0x68) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 5 | vtable(dyn Any) | 0x18 | type_id | r | as_any() 결과 팻포인터의 vtable+24 → TypeId 16B sret (38397~38399 등 4회) | 4 | 확인불가(tcx 사전에 타입 없음) |
| 6 | Entity(t / champ) | 0x0 | team@tag | r | TeamType 판별자(0=Player,1=Neutral). t.team != champ.team 비교(L795·L813, entity.rs:1127 derive eq 인라인) | 4 | OK |
| 7 | Entity(t / champ) | 0x8 | team@Player.0 | r | 팀 인덱스 usize — 판별자 둘 다 Player 일 때만 비교 | 4 | OK |
| 8 | Entity(t) | 0x4c0 | attack_effect@tag | r | Option<Effect> 니치 태그 i32: -1 = None (38635~38637, L796) | 4 | OK |
| 9 | Entity(t) | 0x490 | attack_effect@Some.0 | r | &Effect 페이로드 시작 — expected_damage_target 의 self (38641) | 4 | OK |
| 10 | PlayerState | 0x930 | info.team | r | others[team] 인덱스(38488, 범위<2 아니면 bounds panic) · 클로저에선 1-team = 적 팀(7241~7243) | 4 | OK |
| 11 | AbstractGameWithCache | 0xf0 | others[team].buf.ptr | r | bumpalo Vec<&Entity> 배열 [2], stride 32 (38507~38508: gep {{ptr,ptr,i64},i64} 인덱스 team) | 4 | OK |
| 12 | AbstractGameWithCache | 0x18 | others[team].len (원소 base +0x18 · 절대 0x108) | r | Vec+0x18 = len (38513~38514, 원소 순회 상한) | 4 | 오귀속(사전은 다른 필드를 준다) |
| 13 | Entity(others 원소) | 0x68 | ty@tag | r | EntityType 태그 == 7(Ghoul) 계수 (38552~38554, L811) | 4 | OK |
| 14 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame 팻포인터 +0 data/+8 vtable) | r | 클로저 7256~7258 — is_recent_visible 의 game 인자 | 4 | OK |
| 15 | AbstractGameWithCache | 0x1e0 | player_champion[enemy_team][0..5] | r | 클로저 7252~7253: [5 x ptr] 배열 stride 40, Option<&Entity> 니치(null=None, 7297) | 4 | OK |
| 16 | Entity(around / c) | 0x660 | x | r | 클로저 거리 계산 (7287·7313) | 4 | OK |
| 17 | Entity(around / c) | 0x668 | y | r | 클로저 거리 계산 (7288·7317) | 4 | OK |
| 18 | [Blackboard;2] | 0x0 | blackboard[enemy_team] | r | 클로저 7342: Blackboard 744B stride 로 인덱스 1-team (bounds<2 검사 7245·7409) → is_recent_visible self | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 20건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -141079226136965309619829630430687205697 | 794 | 태그 | TypeId::of::<game_core::setting::champion::illusionist::IllusionistUltAction>() (i128 상수, DISubprogram is<…IllusionistUltAction> 38406 스코프 !44006) | 4 |
| 1 | -134454233013580059629836840851528089863 | 804 | 태그 | TypeId::of::<NecromancerSkillAction>() (스코프 !44023) | 4 |
| 2 | 55786046490698026004615071578502304005 | 809 | 태그 | TypeId::of::<NecromancerSkill2Action>() (스코프 !44041) | 4 |
| 3 | 148968811538501754127421805951936454270 | 818 | 태그 | TypeId::of::<NecromancerUltAction>() (스코프 !44059) | 4 |
| 4 | 0 | 795 | 태그 | 환술사 궁: t.team != champ.team → Some(0). 네크로 W(813): ghouls>0 이고 t.team == champ.team → Some(0). 판별자 0 = TeamType::Player | 4 |
| 5 | 1000 | 797 | 계수 | dps = expected_damage*1000/attack_cooltime — 밀리초 단위 초당 피해 | 4 |
| 6 | 1 | 797 | 태그 | umax(attack_cooltime, 1) — 0 나눗셈 가드. 또 Option Some 태그 | 4 |
| 7 | 40 | 800 | 계수 | atk_power = dps/40 (sdiv) 뒤 clamp — 피해 → 점수 환산 계수 | 4 |
| 8 | 10 | 800 | 임계 | atk_power 하한(smax 10) 겸 attack_effect None 일 때 dps 기본값(unwrap_or 10, L798 → 10/40 clamp = 10) | 4 |
| 9 | 90 | 800 | 임계 | atk_power 상한(umin 90). 네크로 궁(819)의 '가시 적 있음' 점수 90 과 같은 값 | 4 |
| 10 | 5 | 800 | 산출값 | 환술사 궁: 대상 t 근처(200000) 에 가시 적 없음 → 5 (있으면 atk_power) | 4 |
| 11 | 25 | 805 | 산출값 | 네크로 Q(SkillAction): 내 근처 가시 적 있음 → 25 | 4 |
| 12 | 8 | 805 | 산출값 | 네크로 Q: 가시 적 없음 → 8 | 4 |
| 13 | 2 | 810 | 임계 | others[team] 인덱스 bounds(팀 2). 클로저에서도 player_champion[1-team]·blackboard[1-team] bounds 2 | 4 |
| 14 | 7 | 811 | 태그 | EntityType 메모리태그 7 = Ghoul (tcxdict --enum EntityType idx7=tag7) — others[team] 중 구울 수 계수 | 3 |
| 15 | -100 | 812 | 산출값 | 네크로 W(Skill2Action): 아군 구울 0마리 → Some(-100) (강한 거부) | 4 |
| 16 | 18 | 814 | 계수 | 네크로 W: 적 대상 → ghouls*18 | 4 |
| 17 | 60 | 814 | 임계 | 네크로 W: smin(ghouls*18, 60) 상한 (구울 4마리부터 포화) | 4 |
| 18 | 30 | 819 | 산출값 | 네크로 궁(UltAction): 내 근처 가시 적 없음 → 30 (있으면 90) | 4 |
| 19 | 40000000001 | 789 | 미상 | 클로저: 거리² < 200000²+1 ⟺ dist <= 200000 (약 6.25셀) — 적 챔피언이 around 근처인지 | 4 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 환술사 궁 대상 근처 적 없음 점수 | buff_value.rs:800 (m10.ll:38670) | 5 | 올리면 적이 안 보여도 환술사 궁(소환 명령)을 더 자주 씀 | 4 | 기존 |
| 1 | 환술사 궁 atk_power 환산 계수 / 하한 / 상한 | buff_value.rs:800 (m10.ll:38661~38663) | 40 | 계수 40 을 내리면 대상 dps 가 같아도 점수↑(상한 90 까지). 하한 10·상한 90 | 4 | 기존 |
| 2 | 네크로 Q 점수(적 가시 / 없음) | buff_value.rs:805 (m10.ll:38464) | 25 | 25/8. 올리면 근처 적 있을 때 Q 우선순위↑ | 4 | 기존 |
| 3 | 네크로 W 구울 0마리 거부 | buff_value.rs:812 (m10.ll:38499 phi -100) | -100 | 덜 음수로 하면 구울 없이도 W 를 고려 | 4 | 기존 |
| 4 | 네크로 W 구울당 점수 / 상한 | buff_value.rs:814 (m10.ll:38599·38602) | 18 | ghouls*18 을 60 에서 포화. 계수↑ → 적은 구울로도 최대점 | 4 | 기존 |
| 5 | 네크로 궁 점수(적 가시 / 없음) | buff_value.rs:819 (m10.ll:38495) | 90 | 90/30. 없음 쪽 30 을 내리면 적 없을 때 궁 낭비 감소 | 4 | 기존 |
| 6 | 근처 가시 적 반경(제곱) | buff_value.rs:789 (m10.ll:7338 클로저) | 40000000001 | 200000²+1. 올리면 더 먼 적도 '근처'로 봐 높은 점수 경로를 더 자주 탐 | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_any | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any | pub | fn(&game_ai::AgentVerHamster) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-ai\src\lib.rs:425 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 1 | as_any | game_core::Action::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\setting\action.rs:12 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 2 | as_any | game_core::AiAgent::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\simulation\ai_interface.rs:499 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 3 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 8 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 9 | v57_summon_command_score | game_ai::buff_value::v57_summon_command_score | in:game_ai | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Entity) -> std::option::Option<i64> | game-ai\src\buff_value.rs:785 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 7개**: `_RNCNvNtCshdEBA0ozCnw_7game_ai10buff_value24v57_summon_command_score0B5_`, `clamp`, `llvm.smax.i64`, `llvm.smin.i64`, `llvm.umax.i64`, `llvm.umin.i64`, `near_visible_enemy`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m05.ll:40323) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | Action::as_any 슬롯 이름은 divtable 이 TargetAttackAction 구현체의 정적 vtable 기준으로 0x68=as_any 라고 답한 것 — 런타임 Box<dyn Action> 의 실제 구현체별 vtable 배치가 같다는 전제(트레이트 오브젝트 vtable 은 트레이트 기준이라 같아야 함) | 3 |  |
| 1 | 미탐색 | L797 인라인 클로저(closure$1)가 map 으로 dps 를 만들 때 `expected_damage_target` 4번째 인자(target)에 t 자신(%4)을 넘긴다 — 소환수의 자기 자신 대상 기대피해로 '공격력 척도'를 잡는 의도로 읽히나 소스 주석 부재(rmetadocs 0건) | 4 |  |
| 2 | 표기 불가 | L798 unwrap_or(10) 과 L800 clamp 의 정확한 소스 표기(`.map(..).unwrap_or(10)` 뒤 `(dps/40).clamp(10,90)` 으로 추정) — None 경로가 상수 10 으로 접혀 표기 불가(외연 동일) | 5 |  |
| 3 | 미탐색 | Effect::expected_damage_target / Entity::attack_cooltime / Blackboard::is_recent_visible 내부는 game_core 경계 — 시그니처만 기록(tcx: fn(&Effect,&GameContext,&dyn AbstractEntity,&Entity)->usize / fn(&Entity)->usize / fn(&Blackboard,&dyn AbstractGame,&PlayerState,&Entity)->bool) | 3 |  |
| 4 | 미탐색 | calls 첫 항목의 망글 심볼 = closure#0(near_visible_enemy, aux m10.ll:7228~7652) — qcspec 이 'closure#0' 표기를 못 잡아 심볼로 적음 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | TypeId i128 상수 4개는 컴파일러 해시라 값 자체 의미 없음 — 타입 귀속은 DISubprogram is<T> 스코프(!44006·!44023·!44041·!44059)로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

