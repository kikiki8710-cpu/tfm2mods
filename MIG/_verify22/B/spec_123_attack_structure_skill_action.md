---

### `123` attack_structure_skill_action — 구조물(적 타워 6·쌍둥이·넥서스)에 스킬/스킬2 를 시전하는 후보 생성 — expected_damage_structure 를 보고하는(구조물 특화) 스킬만 대상으로, 표적가능·사거리(+이속×30)·target_constraint·can_activate 통과 시 Skill/Skill2 후보 push

| 항목 | 값 |
|---|---|
| id | `fight_check__attack_structure_skill_action` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai11fight_check29attack_structure_skill_action` |
| 소스 | `game-ai\src\fight_check.rs:794` |
| IR | `m15.ll` 33783~34619행 |
| 경로·가시성 | `game_ai::attack_structure_skill_action` · **pub** |
| 계층 | 점수화·술어 |
| exe | `ebc140` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut bumpalo Vec<SmallActionPlay>(32B) | +0 ptr / +8 bump / +16 cap / +24 len — L833 memcpy(m15.ll:34290) | 4 |
| 1 | 1 | player | &PlayerState(2528B) | info.team / info.position 만 | 4 |
| 2 | 2 | data | &OperationData(24B) | cache(+0)·context(+8). context 는 expected_damage_structure 와 pool 에, cache.game 은 Action vtable 호출에 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn attack_structure_skill_action(player, data) -> bumpalo Vec<SmallActionPlay>
  let champ = cache.player_champion[team][pos].unwrap()                                        // L795 (33803~33824; None → 패닉 33888)
  let mut ret = Vec::new_in(data.context.pool)                                                    // L796 (33839~33846)
  // L798~799: 구조물 특화 스킬만 — EffectType::expected_damage_structure(ctx, champ as &dyn AbstractEntity).is_some()
  let skill  = champ.skill_effect.as_ref().filter(|e| e.ty.expected_damage_structure(data.context, champ).is_some())    // closure#0 (33845~33885; vtable+0x30, Arc deref 33868~33873)
  // L800~801
  let skill2 = champ.skill2_effect() /*level>2*/.as_ref().filter(|e| e.ty.expected_damage_structure(data.context, champ).is_some())   // closure#1 (33899~33946)
  if skill.is_none() && skill2.is_none() { return ret }                                           // L802 (33950~33954)
  let leeway = champ.stat_cached.move_speed * 30                                                  // L806 (33957~33958, 33984)
  for tower in cache.iter_towers(1 - team).filter(|t| t.can_target()) {                           // L807 (33962; closure#2 = can_target(0x6b9) && block_target_tick(0x6a0)==0; Chain: 타워6 → 쌍둥이 slice(aux m11 33849 try_fold) → 넥서스)
    let d2 = distance_sq(tower.xy, champ.xy)                                                      // L808 (34254~34281)
    if let Some(sk) = skill {                                                                      // L810 (34282: %86)
      let r = sk.range + leeway + (level-1)*sk.growth_range + stat.range + range_adjust(sk, champ, tower) + radius(champ) + radius(tower)   // L812~813 (34297~34357)
      if champ.can_skill() && sk.target.check(champ, tower) && d2 <= r*r {                         // L814 (34359~34373; 순서 can_skill → check → 거리)
        if champ.skill.target_constraint(game, champ, tower) {                                     // L815 vtable+0xc8 (34376~34386)
          if champ.skill.can_activate(game, champ) {                                               // L816 vtable+0xc0 (34389~34395)
            ret.push(Skill(SmallActionSkill::new(data, tower.id))) } } } }                          // L817 (34400~34452; 태그 16)
    if let Some(sk2) = skill2 {                                                                    // L821 (34304: %87)
      let r = sk2.range + leeway + (level-1)*sk2.growth_range + stat.range + range_adjust(sk2,..) + radius(champ) + radius(tower)   // L822~823 (34459~34519)
      if champ.can_skill2() && sk2.target.check(champ, tower) && d2 <= r*r {                       // L824 (34521~34535)
        let act = if level > 2 { &champ.skill2 } else { &champ.empty }                             // (33993~33995)
        if act.target_constraint(game, champ, tower) {                                             // L825 (34538~34548)
          if act.can_activate(game, champ) {                                                       // L826 (34551~34559)
            ret.push(Skill2(SmallActionSkill2::new(data, tower.id))) } } } }                        // L827 (34564~34616; 태그 17)
  }
  ret                                                                                              // L833 (34290~34292)
극성: 33854/33912 None → 필터 결과 null / 33882·33940 tag==0 → null(제외) / 33954 둘 다 null → 빈 반환 / 34282 skill null → %205(skill2 로) / 34363 can_skill false → %205 / 34373 select false → %205 / 34386·34395 false → %205 / 34304 skill2 null → %285(다음 타워) / 34525·34535·34548·34559 false → %285.
```

**`mem` 메모리 접근 32건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | <2 아니면 panic(33803~33809). 1-team = 적(33961) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | as_index()(33814~33816) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | 33817. iter_towers 의 self(33962), game 팻포인터 로드(34378~34379) | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext(64B) → pool(+0) 및 expected_damage_structure 인자(33876, 33934) | 4 | OK |  |
| 4 | GameContext | 0x0 | pool | r | Vec 할당자(33837, 33841) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | None → unwrap_failed 패닉(33823~33824, 33888) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | Action::target_constraint/can_activate 인자(34378, 34540) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | dr816 (34379, 34541) | 4 | OK |  |
| 8 | Entity | 0x4f8 | skill_effect@tag(니치) | r | champ. -1=None → skill 후보 없음(33845~33854, L798) | 4 | OK |  |
| 9 | Entity | 0x4c8 | skill_effect@Some.0 | r | = skill(&Effect). +0 ty(Arc<dyn EffectType> 데이터 ptr 33865) / +8 vtable(33866~33867) / +0x10 range(33978) / +0x18 growth_range(33979) / +0x28 target(33985) | 4 | OK |  |
| 10 | Entity | 0x5c8 | level | r | champ. level>2 → skill2_effect/skill2 action 선택(33899~33904, 33993), level-1 → growth 배수(33981) | 4 | OK |  |
| 11 | Entity | 0x500 | skill2_effect@Some.0 | r | level>2 일 때 base, 아니면 정적 None @anon…58(33902~33904). +0x30 태그(33905~33907) / +0x10 range(33990) / +0x18 growth(33991) / +0x28 target(33992) | 4 | OK |  |
| 12 | Entity | 0x530 | skill2_effect@tag(니치) | r | = base+48. -1 → skill2 후보 없음(L800) | 4 | OK |  |
| 13 | Entity | 0x640 | stat_cached.move_speed | r | champ ×30 = 사거리 여유(33957~33958, 33984, L806) | 4 | OK |  |
| 14 | Entity | 0x660 | x | r | champ(33976)·tower(34254) → distance_sq L808 | 4 | OK |  |
| 15 | Entity | 0x668 | y | r | champ(33977)·tower(34258) | 4 | OK |  |
| 16 | Entity | 0x438 | stat_buff_cached.range | r | champ(33980, 34299, 34461) | 4 | OK |  |
| 17 | Entity | 0x470 | stat_buff_cached.radius_mult | r | champ(33982)·tower 반경 배율 | 4 | OK |  |
| 18 | Entity | 0x680 | radius | r | champ(33983)·tower | 4 | OK |  |
| 19 | Entity | 0x580 | skill(Box<dyn Action>).data_ptr | r | champ. Action vtable 호출 self(33986, 34376) | 4 | OK |  |
| 20 | Entity | 0x588 | skill(Box<dyn Action>).vtable_ptr | r | vtable+0xc8 target_constraint(34380~34382) / +0xc0 can_activate(34389~34391) | 4 | OK |  |
| 21 | Entity | 0x590 | skill2(Box<dyn Action>) | r | level>2 면 이것, 아니면 0x5b0 empty (select 33993~33995). target_constraint(34542~34544)/can_activate(34553~34555) | 4 | OK |  |
| 22 | Entity | 0x5b0 | empty(Box<dyn Action>) | r | level<=2 의 skill2 action 대리 — 그 경우 skill2_effect 가 None 이라 실전 도달 불가(추정) | 5 | OK |  |
| 23 | Entity | 0x5c0 | id | r | tower id → SmallActionSkill/Skill2::new(34400~34402, 34564~34566) | 4 | OK |  |
| 24 | Entity | 0x6b9 | can_target | r | tower. bool(aux m15.ll:58674~58677 / 인라인 34127~34129·34234~34236) — Entity::can_target(entity.rs:1478) = can_target && block_target_tick==0 | 4 | OK |  |
| 25 | Entity | 0x6a0 | block_target_tick | r | tower. ==0 이어야 표적 가능(58680~58682 / 34130~34133 / 34237~34239) | 4 | OK |  |
| 26 | (sret) | 0x0 | Vec.ptr | w | 33839 초기화, L833 memcpy 34290 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 8(dangling) 또는 reserve 결과 |
| 27 | (sret) | 0x8 | Vec.bump | w | 33841 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | data.context.pool |
| 28 | (sret) | 0x10 | Vec.cap | w | memset 33846; 34429, 34593 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 0 → reserve_internal_or_panic 갱신 |
| 29 | (sret) | 0x18 | Vec.len | w | 34452, 34616 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | push 횟수 |
| 30 | Vec<SmallActionPlay>[len] | 0x0 | SmallActionSkill/Skill2::new 24B | w | 184B memcpy(34450, 34614). +0x18..+0xb0 은 alloca 잔여 | 4 | 확인불가(tcx 사전에 타입 없음) | new(data, tower.id) |
| 31 | Vec<SmallActionPlay>[len] | 0xb1 | SmallActionPlay@tag | w | store i8 (34407, 34571) | 4 | 확인불가(tcx 사전에 타입 없음) | 16(Skill) \| 17(Skill2) |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 30 | 806 | 계수 | 이속 × 30틱(0.5초@60tps) 사거리 여유 — skill·skill2 사거리 모두에 가산(33984 mul, 34352, 34514) | 4 |
| 1 | 100 | 812 | 계수 | 반경 배율 radius*(mult+100)/100 (entity.rs:1515 인라인) | 4 |
| 2 | -1 | 798 | 센티널 | Option<Effect> 니치 None(i32): skill_effect(33848) / skill2_effect(33907). 34025·34146 의 -1 과 34011·34186 의 -2 는 Chain 이터레이터 상태 니치(판정 아님) | 4 |
| 3 | 2 | 800 | 임계 | skill2 존재/action 선택 조건 level > 2 (entity.rs:1693, 33901·33993). 33805 의 2 는 팀 bounds | 4 |
| 4 | 0 | 799 | 태그 | Option<(usize,usize)> 태그 0=None — expected_damage_structure 가 None 이면 그 스킬 제외(33882, 33940). block_target_tick==0 (58682) | 4 |
| 5 | 16 | 817 | 태그 | SmallActionPlay 메모리태그 16 = Skill (34407) | 4 |
| 6 | 17 | 827 | 태그 | SmallActionPlay 메모리태그 17 = Skill2 (34571) | 4 |
| 7 | 1 | 807 | 인덱스 | 1 - team = 적 팀 인덱스(33961); (level-1)(33981); push len+1 | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 사거리 여유 = 이속 × N틱 | fight_check.rs:806 (m15.ll:33984) | 30 | 올리면 아직 사거리 밖인 구조물에도 스킬 후보가 서서 접근 중 시전 판단이 앞당겨짐 | 4 | 기존 |

<details><summary>`callees` 피호출자 23건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_structure_skill_action | game_ai::attack_structure_skill_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:794 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | can_activate | game_core::Action::can_activate | pub | fn(&Self/#0, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity) -> bool | game-core\src\setting\action.rs:30 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 2 | can_activate | <game_core::HunterSkill2Action as game_core::Action>::can_activate | pub | fn(&game_core::HunterSkill2Action, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity) -> bool | game-core\src\setting\champion\hunter.rs:427 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 3 | can_activate | <game_core::NecromancerUltAction as game_core::Action>::can_activate | pub | fn(&game_core::NecromancerUltAction, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity) -> bool | game-core\src\setting\champion\necromancer.rs:417 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 4 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 11 | expected_damage_structure | game_core::EffectType::expected_damage_structure | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<(usize, usize)> | game-core\src\simulation\effect\type.rs:277 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 12 | expected_damage_structure | <game_core::SiegeBreakerSkillEffect as game_core::EffectType>::expected_damage_structure | pub | fn(&game_core::SiegeBreakerSkillEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<(usize, usize)> | game-core\src\setting\champion\siege_breaker.rs:272 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 13 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 16 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 17 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 18 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | target_constraint | game_core::Action::target_constraint | pub | fn(&Self/#0, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity, &game_core::Entity) -> bool | game-core\src\setting\action.rs:34 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 22 | target_constraint | <game_core::HunterSkill2Action as game_core::Action>::target_constraint | pub | fn(&game_core::HunterSkill2Action, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity, &game_core::Entity) -> bool | game-core\src\setting\champion\hunter.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
</details>

⚠**미매칭 8개**: `block_target_tick`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `llvm.memset.p0.i64`, `null`, `reserve_internal_or_panic`, `slice`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m02.ll:48398, m14.ll:20909, m14.ll:23577, m14.ll:48964, m15.ll:22488) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | EffectType::expected_damage_structure(&self, &GameContext, &dyn AbstractEntity) -> Option<(usize,usize)> (vtable+0x30, divtable EffectType 0x30) 의 각 구현 내부 및 (usize,usize) 의미는 game_core — 여기선 is_some 만 사용. @anon.e5f9a016….56(88B) 은 Entity 의 AbstractEntity vtable(추정: 팻포인터 뒤 절반으로 전달) | 3 |  |
| 1 | 미탐색 | Action::target_constraint(vtable+0xc8)·can_activate(vtable+0xc0) 의 구현별 내부 미독(divtable Action 일치율 58% — 슬롯 이름은 tcx 트레이트 메서드 순서로 교차확인: action.rs:30 can_activate / :34 target_constraint) | 3 |  |
| 2 | 미탐색 | iter_towers(&cache, team)->Chain 이터레이터(136B, _gcbc g15.ll:102521) 의 원소 순서(타워 6 → 쌍둥이 → 넥서스)는 IR 의 Chain 타입 이름으로 추정 — 내부 미독 | 4 |  |
| 3 | 미탐색 | SmallActionSkill/Skill2::new 의 24B 내용 미독. 원소 +0x18..+0xb0 은 alloca 잔여(런타임 비교 제외) | 4 |  |
| 4 | 미탐색 | level<=2 이면 skill2_effect 가 정적 None 이라 empty action(0x5b0) 경로는 도달 불가(추정) | 5 |  |
| 5 | 재료 부재 | L814/L824 의 `&&` 세 항 소스 순서는 IR 호출 순서(can_skill → check → 거리 select)로만 확정 — 한 줄 안 표기는 복원 불가 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

