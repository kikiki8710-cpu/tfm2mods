---

### `114` attack_summon_action — 적 소환수(others 목록: 곰/독수리/구울 등) 중 표적 가능·가시·80k 이내인 것마다 평타/스킬/스킬2 사거리(+이속×30 여유) 안이면 Attack/Skill/Skill2 후보를 bumpalo Vec 에 push 해 반환

| 항목 | 값 |
|---|---|
| id | `fight_check__attack_summon_action` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai11fight_check20attack_summon_action` |
| 소스 | `game-ai\src\fight_check.rs:836` |
| IR | `m15.ll` 28194~28874행 |
| 경로·가시성 | `game_ai::attack_summon_action` · **pub** |
| 계층 | 점수화·술어 |
| exe | `eb8b80` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut bumpalo Vec<SmallActionPlay>(32B) | +0 ptr / +8 bump(&Bump) / +16 cap / +24 len. L880 에서 지역 %10 을 memcpy 32B(m15.ll:28345) | 4 |
| 1 | 1 | player | &PlayerState(2528B) | info.team(0x930)·info.position(0x9c0) → 내 챔피언; can_target 인자로도 전달(28341) | 4 |
| 2 | 2 | data | &OperationData(24B) | +0 cache / +8 context(→+0 pool = Vec 할당자). can_target·SmallAction*::new 의 self/인자 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn attack_summon_action(player:&PlayerState, data:&OperationData) -> bumpalo Vec<SmallActionPlay>
  let champ = cache.player_champion[player.info.team][player.info.position.as_index()].unwrap()   // L837 (None → unwrap_failed 패닉 28303)
  let mut ret = Vec::new_in(data.context.pool)                                                       // L838 (28249~28255)
  let leeway = champ.stat_cached.move_speed * 30                                                     // L839 (28254~28256, 28290)
  for target in cache.others[1 - team].iter() {                                                      // L841 적 팀 others(소환수) 순회 (28258~28275, 28323)
    // L842
    if !data.can_target(cache.game, player, target) { continue }                                     // 28341~28350 false → %255 next
    if !target.is_visible_from(champ.player_team()) { continue }                                     // 28354~28371 (champ Neutral 이면 생략)
    let d2 = distance_sq(target.xy, champ.xy)                                                        // L845 (28378~28428)
    if d2 > 80000² { continue }                                                                       // 28429~28430
    // ── 평타 후보 L849~854
    if champ.can_attack() {                                                                           // 28433~28437
      if let Some(atk) = champ.attack_effect {                                                        // L850 (28447~28449)
        let r = atk.range + leeway + champ.stat_buff_cached.range + (level-1)*atk.growth_range
              + range_adjust(atk, champ, target) + radius(champ) + radius(target)                     // L851~852 (28508~28513)
        if d2 <= r*r { ret.push(SmallActionPlay::Attack(SmallActionAttack::new(data, target.id))) } } }   // L853~854 (28516~28576; 태그 15)
    // ── 스킬 후보 L859~864
    if let Some(sk) = champ.skill_effect {                                                            // L859 (28441~28443)
      if champ.can_skill() && sk.target.check(champ, target) {                                        // L860 (28581~28602)
        let r = sk.range + leeway + stat.range + (level-1)*sk.growth_range + range_adjust(sk,..) + radius(champ) + radius(target)   // L861~862 (28659~28664)
        if d2 <= r*r { ret.push(Skill(SmallActionSkill::new(data, target.id))) } } }                   // L863~864 (28667~28727; 태그 16)
    // ── 스킬2 후보 L869~874 (level>2 일 때만 skill2_effect 가 Some 가능)
    if let Some(sk2) = champ.skill2_effect() {                                                        // L869 (28585~28592)
      if champ.can_skill2() && sk2.target.check(champ, target) {                                      // L870 (28732~28747)
        let r = sk2.range + leeway + (level-1)*sk2.growth_range + stat.range + range_adjust(sk2,..) + radius(champ) + radius(target)   // L871~872 (28805~28810)
        if d2 <= r*r { ret.push(Skill2(SmallActionSkill2::new(data, target.id))) } } }                 // L873~874 (28813~28873; 태그 17)
  }
  ret                                                                                                  // L880~881 (28345~28347)
극성: 28350 can_target false→%255(continue) / 28371 Visible(0) 아니면 %255 / 28430 d2>80000² → %255 / 28437 can_attack false→%112(스킬로) / 28449 atk None→%112 / 28517 d2>r² → %112 / 28443 skill None→%181(스킬2로) / 28595·28602 false→%181 / 28668 d2>r²→%181 / 28592 sk2 None→%255 / 28739·28747 false→%255 / 28814 d2>r²→%255. 세 후보는 독립(한 대상에 최대 3개 push).
```

**`mem` 메모리 접근 41건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | <2 아니면 panic(28207~28213). 1-team = 적 인덱스(28258) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | as_index()(28218~28220) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(28221) | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext → +0x0 pool(&Bump) 을 Vec 할당자로(28245~28251, L838) | 4 | OK |  |
| 4 | GameContext | 0x0 | pool | r | &bumpalo::Bump → Vec+8 에 저장(28247, 28251) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | None 이면 unwrap_failed 패닉(28224~28228, 28303) — Option 아님, unwrap | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | dyn AbstractGame 팻포인터 앞 절반 → can_target 인자(28339) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 팻포인터 뒤 절반(dr816 = vtable 100슬롯) → can_target 인자(28340) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0xf0 | others[1-team].buf.ptr | r | 적 팀 '기타' 엔티티(소환수) bumpalo Vec 시작(stride 32B, 28259~28263, L841) | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x108 | others[1-team].len | r | = Vec+24 (28265~28266). ptr..ptr+len 순회(28275, 28323) | 4 | OK |  |
| 10 | Entity | 0x640 | stat_cached.move_speed | r | champ. ×30 이 세 사거리 모두에 가산되는 여유(28254~28256, 28290, L839) | 4 | OK |  |
| 11 | Entity | 0x0 | team@tag | r | champ. Neutral 이면 가시성 검사 생략(28354~28356, entity.rs:1136) | 4 | OK |  |
| 12 | Entity | 0x8 | team@Player.0 | r | champ 팀번호 → target.visible_state 인덱스(28360~28363) | 4 | OK |  |
| 13 | Entity | 0x38 | visible_state[team]@tag | r | target. 0=Visible 아니면 continue(28367~28371, L842) | 4 | OK |  |
| 14 | Entity | 0x660 | x | r | target(28378)·champ(28398) → distance_sq(L845) | 4 | OK |  |
| 15 | Entity | 0x668 | y | r | target(28388)·champ(28407) | 4 | OK |  |
| 16 | Entity | 0x4c0 | attack_effect@tag(니치) | r | champ. -1=None → 평타 후보 생략(28447~28449, L850) | 4 | OK |  |
| 17 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect → range_adjust(28458) | 4 | OK |  |
| 18 | Entity | 0x4a0 | attack_effect@Some.0.range | r | 28454 (L851) | 4 | OK |  |
| 19 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | ×(level-1) 28455·28462~28463 | 4 | OK |  |
| 20 | Entity | 0x5c8 | level | r | champ(28456). level>2 가 skill2 존재 조건(28585~28587) | 4 | OK |  |
| 21 | Entity | 0x438 | stat_buff_cached.range | r | champ 기본 사거리(28457, 28608, 28754) | 4 | OK |  |
| 22 | Entity | 0x470 | stat_buff_cached.radius_mult | r | champ(28464)·target(28484) 반경 배율(entity.rs:1511~1515) | 4 | OK |  |
| 23 | Entity | 0x680 | radius | r | champ·target 몸통 반경(28470·28476·28491·28498 등) | 4 | OK |  |
| 24 | Entity | 0x5c0 | id | r | target id → SmallActionAttack/Skill/Skill2::new 인자(28522~28524, 28673~28675, 28819~28821) | 4 | OK |  |
| 25 | Entity | 0x4f8 | skill_effect@tag(니치) | r | champ. -1 → 스킬 후보 생략(28441~28443, L859) | 4 | OK |  |
| 26 | Entity | 0x4c8 | skill_effect@Some.0 | r | &Effect → range_adjust(28609) | 4 | OK |  |
| 27 | Entity | 0x4f0 | skill_effect@Some.0.target | r | CastingTarget → check(28598, L860) | 4 | OK |  |
| 28 | Entity | 0x4d8 | skill_effect@Some.0.range | r | 28605 (L861) | 4 | OK |  |
| 29 | Entity | 0x4e0 | skill_effect@Some.0.growth_range | r | 28606 | 4 | OK |  |
| 30 | Entity | 0x500 | skill2_effect@Some.0 | r | level>2 일 때만 이 주소, 아니면 정적 None @anon…58 (entity.rs:1693, 28585~28587). 이하 +0x30 태그/+0x28 target/+0x10 range/+0x18 growth 는 이 base 상대(28589·28742·28750·28752) | 4 | OK |  |
| 31 | Entity | 0x530 | skill2_effect@tag(니치) | r | = base+48. -1 → 다음 대상(28589~28592, L869) | 4 | OK |  |
| 32 | Entity | 0x528 | skill2_effect@Some.0.target | r | = base+40 → check(28742~28743, L870) | 4 | OK |  |
| 33 | Entity | 0x510 | skill2_effect@Some.0.range | r | = base+16 (28750~28751, L871) | 4 | OK |  |
| 34 | Entity | 0x518 | skill2_effect@Some.0.growth_range | r | = base+24 (28752~28753) | 4 | OK |  |
| 35 | (sret) | 0x0 | Vec.ptr | w | L880 memcpy 32B(28345). 지역 %10 초기화: 28249 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 지역 Vec 의 ptr(초기 8 dangling, 첫 push 시 reserve 로 교체) |
| 36 | (sret) | 0x8 | Vec.bump | w | 28251 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | data.context.pool |
| 37 | (sret) | 0x10 | Vec.cap | w | memset 0 (28255); push 시 len==cap 이면 reserve(28542~28551 등) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 0 → reserve_internal_or_panic 이 갱신 |
| 38 | (sret) | 0x18 | Vec.len | w | 28574·28725·28871 에서 +1 store | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | push 횟수 |
| 39 | Vec<SmallActionPlay>[len] | 0x0 | SmallActionPlay 페이로드(24B SmallActionAttack/Skill/Skill2::new 결과) | w | 184B memcpy(28572·28723·28869). 24B 이후 바이트는 지역 alloca 의 미초기화 잔여(런타임 대조 시 +0x18..+0xb1 은 비교 제외) | 4 | 확인불가(tcx 사전에 타입 없음) | new(data, target.id) |
| 40 | Vec<SmallActionPlay>[len] | 0xb1 | SmallActionPlay@tag | w | store i8 15/16/17 (28529, 28680, 28826) | 4 | 확인불가(tcx 사전에 타입 없음) | 15(Attack) \| 16(Skill) \| 17(Skill2) |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 30 | 839 | 계수 | 이속 × 30틱(=0.5초@60tps) 를 사거리 여유로 가산 — 세 후보(평타·스킬·스킬2) 사거리 모두에 더함(28290 mul, 28508·28659·28805) | 4 |
| 1 | 6400000000 | 845 | 임계 | 80000² — champ↔target 거리제곱이 이보다 크면 후보 생성 자체를 건너뜀(ugt, 28429~28430) | 4 |
| 2 | 100 | 851 | 계수 | 반경 배율 퍼센트 radius*(mult+100)/100 (entity.rs:1515 인라인, 28477~28479 등 6곳) | 4 |
| 3 | -1 | 850 | 센티널 | Option<Effect> 니치 None(i32): attack_effect(28448) / skill_effect(28442) / skill2_effect(28591) | 4 |
| 4 | 2 | 869 | 임계 | skill2_effect 존재 조건 level > 2 (entity.rs:1693, 28586). 28209·28362 의 2 는 팀 배열 bounds | 4 |
| 5 | 15 | 854 | 태그 | SmallActionPlay 메모리태그 15 = Attack (store i8 28529) | 4 |
| 6 | 16 | 864 | 태그 | SmallActionPlay 메모리태그 16 = Skill (28680) | 4 |
| 7 | 17 | 874 | 태그 | SmallActionPlay 메모리태그 17 = Skill2 (28826) | 4 |
| 8 | 0 | 842 | 태그 | VisibleState 태그 0=Visible (28370); radius_mult==0 분기(28466 등) | 4 |
| 9 | 1 | 841 | 인덱스 | 1 - team = 적 팀 인덱스(28258); (level-1) 는 add -1 (28462); push len+1 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 후보 생성 최대 거리(제곱) | fight_check.rs:845 (m15.ll:28429) | 6400000000 | 80000²(2.5셀). 올리면 더 먼 소환수도 후보 검토(사거리 필터는 별도라 실제 push 는 사거리+여유 안일 때만) | 4 | 기존 |
| 1 | 사거리 여유 = 이속 × N틱 | fight_check.rs:839 (m15.ll:28290) | 30 | 올리면 아직 사거리 밖(0.5초 안에 닿을)인 소환수도 공격/스킬 후보로 올림 — 소환수 공격이 더 적극적 | 4 | 기존 |

<details><summary>`callees` 피호출자 22건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 9 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 10 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 11 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 14 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 15 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 16 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 17 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 18 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 19 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 20 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 4개**: `llvm.memcpy.p0.p0.i64`, `llvm.memset.p0.i64`, `others`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 14곳** (m02.ll:10875, m02.ll:11730, m02.ll:20911, m02.ll:38713, m02.ll:40291, m02.ll:46600, m02.ll:47725, m14.ll:18445, m14.ll:20231, m14.ll:22792, m14.ll:31469, m14.ll:48258, m15.ll:13488, m15.ll:21806) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | can_target(&OperationData, game.data, game.vtable, &PlayerState, &Entity)->bool 내부(game_core) 미독 — 시그니처만(m15.ll:28341). game 팻포인터를 두 인자로 분해해 넘김 | 4 |  |
| 1 | 미탐색 | SmallActionAttack/Skill/Skill2::new(sret 24B, &OperationData, target_id) 의 24B 내용 미독(별도 함수). 184B 원소 중 +0x18..+0xb0 은 지역 alloca 미초기화 잔여 — 런타임 비교 시 제외 대상 | 4 |  |
| 2 | 재료 부재 | L851 vs L871 에서 stat.range 와 (level-1)*growth 의 가산 순서가 IR 상 다르지만(28509~28510 / 28806~28807) 정수 덧셈이라 결과 동일 — 소스 줄 안 순서는 column 부재로 복원 불가 | 4 |  |
| 3 | 미탐색 | others[1-team] 이 정확히 어떤 엔티티 종류를 담는지(소환수 외 포함 여부)는 game_core 캐시 채우는 쪽(미독) — docs L115 '적 소환수(곰, 독수리, 구울 등)' 에 근거 | 4 |  |
| 4 | 미탐색 | champ Neutral 경로(28356 true→%90)는 실전 도달 불가로 추정(player_champion 은 Player 팀) | 5 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | Effect::range(effect.rs:26) 인라인의 정확한 인자(leeway 가 range() 인자인지 호출부 가산인지)는 소스 표기 미확정 — 합산 결과는 IR 로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

