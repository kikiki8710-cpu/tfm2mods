---

### `159` battle_ally_action — 전투 중 아군 대상 스킬/스킬2 후보 목록 — 150000 안 아군 중 타겟 규칙·사거리(+걸음 허용치) 안이면 Skill/Skill2 액션, 자기 대상은 별도 판정으로 추가

| 항목 | 값 |
|---|---|
| id | `fight_check__battle_ally_action` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai11fight_check18battle_ally_action` |
| 소스 | `game-ai\src\fight_check.rs:1112` |
| IR | `m15.ll` 25950~26895행 |
| 경로·가시성 | `game_ai::battle_ally_action` · **pub** |
| 계층 | 점수화·술어 |
| exe | `eb77a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo Vec<SmallActionPlay>(32B) | candidates(%18) 를 memcpy 32B 로 반환(26445). new_in(bump)=+0 ptr 8(dangling) / +8 bump / +0x10..0x18 0 (26076~26083) | 4 |
| 1 | 1 | _version | usize | 미사용(본문에 %1 참조 0건) | 4 |
| 2 | 2 | _rnd | &mut StdRng(320B) | readnone — 미사용. writes 없음 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0); should_add_self_etc_buff_action 인자 | 4 |
| 4 | 4 | data | &OperationData(24B) | +0 cache(player_champion·game 팻포인터) / +8 context(→ +0 pool bump, expected_buff_deep 인자) | 4 |
| 5 | 5 | _end_delay | usize | 미사용(본문에 %5 참조 0건) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn battle_ally_action(_version, _rnd, player, data, _end_delay) -> Vec<SmallActionPlay>   // 인자 1·2·5 미사용
// L1113 ProfTimer(88) (계측)
// L1114
team = player.info.team (bounds 2); champ = cache.player_champion[team][player.info.position].unwrap()
// L1116~1117  [aux m01.ll:37788 from_iter_in + m15.ll:58228 call_mut 심]
near_allies: Vec<&Entity> = cache.iter_champions(team) /*player_champion[team] 5칸 flatten*/.filter(|e| e.id != champ.id && dist²(e, champ) < 22500000000 /*<150000*/).collect_in(ctx.pool)
// L1119
candidates = Vec::new_in(ctx.pool)
// L1121~1124
skill  = champ.skill_effect (Option)
skill2 = champ.skill2_effect() (level>2 만 Some)
move_speed = champ.stat_cached.move_speed
// L1126
if let Some(skill) = skill && champ.can_skill() {
    // L1129 ally_buff_walk_allowance(_version, data, champ, skill) 인라인 (fight_check.rs:1181~1183)
    walk = if skill.ty.expected_buff_deep(ctx, champ as &dyn AbstractEntity).is_none() { 30 } else { 90 }
    // L1130~1144
    for e in near_allies {
        if !skill.target.check(champ, e) { continue }                                   // L1131
        max_dist = skill.range(champ)/*range + growth*(level-1) + stat_buff.range*/ + walk*move_speed + skill.range_adjust(champ, e) + champ.radius() + e.radius()   // L1135~1139
        dist_sq = dist²(e, champ)                                                       // L1136
        if dist_sq > max_dist² { continue }                                             // L1140
        candidates.push(SmallActionPlay::Skill(SmallActionSkill::new(data, e.id)))     // L1144 태그 16
    }
    // L1147~1148 자기 대상
    if skill.target.check(champ, champ) && should_add_self_etc_buff_action(player, data, champ, skill) {
        candidates.push(Skill(SmallActionSkill::new(data, champ.id)))
    }
}
// L1152
if let Some(skill2) = skill2 && champ.can_skill2() {
    walk = if skill2.ty.expected_buff_deep(ctx, champ).is_none() { 30 } else { 90 }   // L1153
    for e in near_allies {                                                            // L1154~1168
        if !skill2.target.check(champ, e) { continue }                                 // L1155
        max_dist = skill2.range(champ) + walk*move_speed + skill2.range_adjust(champ, e) + champ.radius() + e.radius()   // L1159~1163
        if dist²(e, champ) > max_dist² { continue }                                    // L1164
        candidates.push(SmallActionPlay::Skill2(SmallActionSkill2::new(data, e.id)))  // L1168 태그 17
    }
    // L1171~1172 자기 대상 = should_add_self_skill2_action(player, data, champ, skill2) 인라인 (fight_check.rs:613~617)
    if skill2.target.check(champ, champ) {
        act = champ.skill2() /*level>2 → skill2 액션, 아니면 empty*/
        ok = if let Some(p) = act.as_any().downcast_ref::<PrisonerSkill2Action>() { p.has_enemy_champion_target_or_action_threat(cache.game, champ) }   // L615
             else { should_add_self_etc_buff_action(player, data, champ, skill2) }                                                                  // L617
        if ok { candidates.push(Skill2(SmallActionSkill2::new(data, champ.id))) }
    }
}
// L1176~1177
return candidates   (ProfTimer drop: 계측 누적)
```

**`mem` 메모리 접근 37건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | prof::ENABLED(정적) | 0x0 | atomic i8 | r | 25986~25988 계측 게이트(phase 88, ProfTimer). 판정 무관 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 1 | PlayerState | 0x930 | info.team | r | 26003~26006 bounds 2 | 4 | OK |  |
| 2 | PlayerState | 0x9c0 | info.position@tag | r | 26023~26025 i32 → 슬롯 | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | 26026 | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | 26062~26063; context+0 = pool(26064, bump 할당자) · expected_buff_deep 의 ctx 인자(26137·26483) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] / [team][0..5] | r | 내 챔피언 unwrap(26028~26033, None→unwrap_failed 26070) · iter_champions(team) 5칸 슬라이스(26053~26059) → near_allies 필터 | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game(dyn AbstractGame 팻포인터 +0/+8) | r | 26772~26774 → PrisonerSkill2Action::has_enemy_champion_target_or_action_threat 인자 | 4 | OK |  |
| 7 | Entity(champ) | 0x4f8 | skill_effect@tag | r | i32 -1=None (26082~26085, L1121) | 4 | OK |  |
| 8 | Entity(champ) | 0x4c8 | skill_effect@Some.0 (Effect: +0 ty.data / +8 ty.vtable) | r | 26086 · 26120~26122; ArcInner 데이터 = ptr + ((vtable.align-1)&-16) + 16 (26129~26134) | 4 | OK |  |
| 9 | Entity(champ) | 0x4d8 | skill_effect.range | r | 26167·26204 (Effect::range 인라인 effect.rs:26) | 4 | OK |  |
| 10 | Entity(champ) | 0x4e0 | skill_effect.growth_range | r | 26168·26205 × (level-1) | 4 | OK |  |
| 11 | Entity(champ) | 0x4f0 | skill_effect.target (CastingTarget) | r | 26166 → CastingTarget::check(self, champ, e)(26196) · 자기대상 check(champ,champ)(26362) | 4 | OK |  |
| 12 | Entity(champ) | 0x500 | skill2_effect@Some.0 (Effect) | r | Entity::skill2_effect 인라인(entity.rs:1693): level>2 → &skill2_effect 아니면 정적 None(@anon…58) (26088~26092) | 4 | OK |  |
| 13 | Entity(champ) | 0x30 | skill2_effect@tag (선택 ptr+0x30 · 절대 0x530) | r | 26094~26096 i32 -1=None | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 14 | Entity(champ) | 0x10 | skill2_effect.range (선택 ptr+0x10 · 절대 0x510) | r | 26513·26550 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 15 | Entity(champ) | 0x18 | skill2_effect.growth_range (선택 ptr+0x18 · 절대 0x518) | r | 26514·26551 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 16 | Entity(champ) | 0x28 | skill2_effect.target (선택 ptr+0x28 · 절대 0x528) | r | 26512 → CastingTarget::check (26542·26708) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 17 | Entity(champ) | 0x5c8 | level | r | 26088~26090 skill2 게이트(>2) · growth 배수(26170·26516) · skill2 액션 선택(26725~26727) | 4 | OK |  |
| 18 | Entity(champ) | 0x640 | stat_cached.move_speed | r | 26098~26099 → walk*move_speed 가산(26175·26521) | 4 | OK |  |
| 19 | Entity(champ) | 0x438 | stat_buff_cached.range | r | 26169·26206 Effect::range 가산항 | 4 | OK |  |
| 20 | Entity(champ / e) | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius 인라인(entity.rs:1511~1515) 26213~26228 · 26235~26253 | 4 | OK |  |
| 21 | Entity(champ / e) | 0x680 | radius | r | 26219 · 26242 | 4 | OK |  |
| 22 | Entity(champ / e) | 0x660 | x | r | 거리² (26260~26261 · 26268) · 필터 심 58246~58255 | 4 | OK |  |
| 23 | Entity(champ / e) | 0x668 | y | r | 26264~26265 · 26271 | 4 | OK |  |
| 24 | Entity(e / champ) | 0x5c0 | id | r | SmallActionSkill::new(data, e.id)(26302~26304) · 자기 id(26380~26382 · 26785~26787) · 필터 심 e.id != champ.id(58236~58240) | 4 | OK |  |
| 25 | Entity(champ) | 0x590 | skill2 (Box<dyn Action> 팻포인터) | r | should_add_self_skill2_action 인라인(fight_check.rs:613, entity.rs:1669 Entity::skill2): level>2 → +0x590 아니면 +0x5b0 empty (26725~26728) | 4 | OK |  |
| 26 | Entity(champ) | 0x5b0 | empty (Box<dyn Action>) | r | level<=2 대체 액션 | 4 | OK |  |
| 27 | vtable(dyn EffectType) | 0xa0 | expected_buff_deep | r | 26135~26137 · 26481~26483 (divtable EffectType 0xa0) → sret 288B Option<BuffState> | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 28 | vtable(dyn EffectType) | 0x10 | align | r | ArcInner 데이터 오프셋 계산(26129~26132 · 26475~26478) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 29 | vtable(dyn Action) | 0x68 | as_any | r | 26733~26735 → &dyn Any; 그 vtable+0x18 type_id(26746~26748) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 30 | Option<BuffState>(sret 288B) | 0x48 | duration@tag | r | i32 -1 = None (26142~26146 · 26488~26492): None → walk 30, Some → 90 (ally_buff_walk_allowance 인라인 fight_check.rs:1181~1183) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 31 | bumpalo Vec<&Entity>(near_allies) | 0x18 | len | r | 26153~26154 · 26499~26500 (ptr +0) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 32 | sret Vec<SmallActionPlay>(candidates %18 → sret) | 0x0 |  | w | new_in 26076 · reserve 시 갱신 · 26445 memcpy 32B 로 sret 전체(+0 ptr/+8 bump/+0x10 cap/+0x18 len) 기록 | 4 | 확인불가(tcx 사전에 타입 없음) | ptr(초기 8 dangling → bump 할당) |
| 33 | candidates 원소[len] | 0x0 -> ptr[len]+0 |  | w | memcpy 184B (26352 · 26431 · 26698 · 26836) — 원소 stride 184 | 4 | 확인불가(tcx 사전에 타입 없음) | SmallActionSkill(24B) 또는 SmallActionSkill2(24B) |
| 34 | candidates 원소[len] | 0xb1 |  | w | SmallActionPlay 니치 태그(idx13 Skill→16, idx14 Skill2→17) 26309 · 26388 · 26655 · 26793 | 4 | 확인불가(tcx 사전에 타입 없음) | 16(Skill) / 17(Skill2) |
| 35 | candidates | 0x18 |  | w | push 마다 26354 · 26433 · 26700 · 26838 | 4 | 확인불가(tcx 사전에 타입 없음) | len+1 |
| 36 | prof::PHASE_NANOS[88] / PHASE_CALLS[88](정적) | 0x0 |  | w | 26885 · 26889 — ENABLED 일 때만. 계측 부작용(판정 무관) | 4 | 확인불가(tcx 사전에 타입 없음) | atomicrmw add 경과 ns / +1 |

**`consts` 상수 16건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 88 | 1113 | 산출값 | ProfTimer phase id (계측) | 4 |
| 1 | 132 | 1177 | 임계 | PHASE_NANOS/PHASE_CALLS 배열 bounds (계측) | 4 |
| 2 | 1000000000 | 1177 | 임계 | Duration → ns 환산 (계측) | 4 |
| 3 | 2 | 1114 | 임계 | 팀 bounds. 또 skill2 레벨 게이트 level > 2 (26090 · 26726) | 4 |
| 4 | 5 | 1116 | 길이 | iter_champions 슬라이스 길이 5 (26049, gep +40 = 5 ptr) | 4 |
| 5 | 22500000000 | 1117 | 미상 | near_allies 필터: dist² < 150000² (엄격 <, +1 없음) — aux m15.ll:58275 | 4 |
| 6 | -1 | 1121 | 센티널 | Option<Effect> None 니치(i32) / Option<BuffState> None(duration 태그 -1) / level-1 | 4 |
| 7 | 30 | 1183 | 산출값 | ally_buff_walk_allowance: expected_buff_deep(ctx, champ) 가 None 이면 walk=30 (틱 — move_speed 곱해 사거리 가산) | 4 |
| 8 | 90 | 1183 | 산출값 | ally_buff_walk_allowance: Some 이면 walk=90 | 4 |
| 9 | 100 | 1135 | 계수 | Entity::radius 인라인 (radius*(mult+100)/100) | 4 |
| 10 | 16 | 1144 | 태그 | SmallActionPlay::Skill 메모리 태그 (tcxdict --enum SmallActionPlay idx13→16) | 3 |
| 11 | 17 | 1168 | 태그 | SmallActionPlay::Skill2 메모리 태그 (idx14→17) | 4 |
| 12 | 168406848281932906149591046147716593956 | 614 | 태그 | TypeId::of::<PrisonerSkill2Action>() — should_add_self_skill2_action 인라인(fight_check.rs:613~617)의 downcast_ref 판별(26756, dloc: is<…prisoner::PrisonerSkill2Action>) | 4 |
| 13 | 1424 | 613 | 산출값 | select 로 접힌 Entity 필드 오프셋: level>2 → 0x590 skill2 / 아니면 1456=0x5b0 empty (Entity::skill2 인라인 entity.rs:1669) — 오프셋이 값으로 나타난 특수 케이스 | 4 |
| 14 | 1456 | 613 | 산출값 | 위 select 의 else 값(0x5b0 empty 액션) | 4 |
| 15 | 184 | 1144 | 미상 | SmallActionPlay 원소 크기(memcpy) — stride | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 아군 후보 반경(제곱) | fight_check.rs:1117 (aux m15.ll:58275) | 22500000000 | 150000². 올리면 더 먼 아군도 near_allies 에 들어 사거리 판정을 받는다(사거리 판정이 최종이라 효과는 사거리+걸음치 이내에서만) | 4 | 기존 |
| 1 | 버프 없는 스킬의 걸음 허용치(틱) | fight_check.rs:1183 (m15.ll:26146·26492 select 30) | 30 | walk*move_speed 만큼 사거리를 넓힘. 올리면 더 먼 아군에게 걸어가 스킬을 쓰려 함 | 4 | 기존 |
| 2 | 버프 스킬의 걸음 허용치(틱) | fight_check.rs:1183 (select 90) | 90 | expected_buff_deep 이 Some 인 스킬(버프류)은 3배 멀리까지 접근 허용 | 4 | 기존 |
| 3 | 스킬2 레벨 게이트 | entity.rs:1693 / 1669 (m15.ll:26090·26726) | 2 | game_core 소관 — level>2 부터 skill2 후보 생성 | 4 | 기존 |

<details><summary>`callees` 피호출자 30건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | ally_buff_walk_allowance | game_ai::fight_check::ally_buff_walk_allowance | in:game_ai | fn(usize, &game_core::OperationData, &game_core::Entity, &game_core::Effect) -> u64 | game-ai\src\fight_check.rs:1181 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | as_any | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any | pub | fn(&game_ai::AgentVerHamster) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-ai\src\lib.rs:425 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 2 | as_any | game_core::Action::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\setting\action.rs:12 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 3 | as_any | game_core::AiAgent::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\simulation\ai_interface.rs:499 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 4 | battle_ally_action | game_ai::battle_ally_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:1112 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | expected_buff_deep | game_core::EffectType::expected_buff_deep | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type.rs:309 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 27개 중 상위 3개 |
| 9 | expected_buff_deep | <game_core::RushEffect as game_core::EffectType>::expected_buff_deep | pub | fn(&game_core::RushEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\rush.rs:59 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 27개 중 상위 3개 |
| 10 | expected_buff_deep | <game_core::RangeEffect as game_core::EffectType>::expected_buff_deep | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\range_effect.rs:110 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 27개 중 상위 3개 |
| 11 | has_enemy_champion_target_or_action_threat | game_core::PrisonerSkill2Action::has_enemy_champion_target_or_action_threat | pub | fn(&game_core::PrisonerSkill2Action, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity) -> bool | game-core\src\setting\champion\prisoner.rs:241 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 15 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 16 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 17 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | should_add_self_etc_buff_action | game_ai::fight_check::should_add_self_etc_buff_action | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Effect) -> bool | game-ai\src\fight_check.rs:608 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | should_add_self_skill2_action | game_ai::fight_check::should_add_self_skill2_action | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Effect) -> bool | game-ai\src\fight_check.rs:612 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 22 | skill2 | game_ai::skill2 | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:239 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 23 | skill2 | game_core::Entity::skill2 | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1668 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 24 | skill2 | game_core::ChampionInfo::skill2 | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1086 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 25 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 26 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 27 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 29 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 5개**: `_RNvXs1_NtNtNtCsjihNppCmMEE_4core3ops8function5implsQNCNvNtCshdEBA0ozCnw_7game_ai11fight_check18battle_ally_action0INtB7_5FnMutTRRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEE8call_mutBU_`, `candidates`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `elapsed`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m02.ll:19252, m02.ll:46673, m14.ll:18517, m14.ll:23702, m15.ll:19789) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | should_add_self_etc_buff_action(&PlayerState,&OperationData,&Entity,&Effect)->bool (fight_check.rs:608, 잎 22 계약만): 이 함수에서 2회 호출 — L1147 skill 자기대상(26371, 팻포인터 = skill.ty data/vtable) · L617 skill2 자기대상 비-Prisoner 경로(26765) | 4 |  |
| 1 | 미탐색 | expected_buff_deep 의 실제 구현체(dyn EffectType) 는 런타임 결정 — divtable 은 CombineEffect 정적 vtable 기준으로 0xa0 슬롯명을 준 것. 반환 Option<BuffState>(288B) 의 None 판별은 duration 태그(+0x48) -1 로 읽음(tcxdict BuffState 0x48 = duration@tag) | 3 |  |
| 2 | 미탐색 | CastingTarget::check(&CastingTarget,&Entity,&Entity)->bool · Entity::can_skill/can_skill2(&Entity)->bool · range_adjust(&Effect,&Entity,&Entity)->u64 · PrisonerSkill2Action::has_enemy_champion_target_or_action_threat(&Self,&dyn AbstractGame,&Entity)->bool · SmallActionSkill::new / SmallActionSkill2::new(&OperationData,usize)->24B — game_core 경계·같은 r14 콜리는 시그니처만 | 4 |  |
| 3 | 미탐색 | iter_champions(team)(simulation.rs:1904, mir=1) 이 player_champion[team] 5칸 flatten 인 것은 IR(26053~26059 슬라이스 [5]ptr + aux 37850 null 스킵)로 읽음 | 4 |  |
| 4 | 미탐색 | L1116 near_allies 의 반경이 `< 22500000000`(엄격) 인 반면 다른 함수들은 `< N²+1`(<=) — 소스 표기 `dist_sq < 150000*150000` 추정, 외연은 IR 대로 | 4 |  |
| 5 | 미탐색 | SmallActionPlay 원소의 +0x18..+0xb1 구간이 미초기화 스택 복사인 점 — sweep 대조 시 태그 16/17 원소는 +0..+0x18 과 +0xb1 만 비교해야 함(ELEM_LIVE) | 4 |  |
| 6 | 미탐색 | calls 마지막 항목 = near_allies 필터 클로저의 call_mut 심(aux m15.ll:58228) 망글 심볼 — qcspec 매칭용 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

