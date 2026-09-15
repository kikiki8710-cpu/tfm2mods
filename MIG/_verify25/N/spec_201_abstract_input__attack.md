---

### `201` abstract_input::attack — 대상(target)에 대한 평타 입력을 만든다 — 사거리 안이면 Input::Attack(InputTarget), 아니면 사거리−여유 지점(비가시면 대상 좌표)으로 안전 이동(Option<Input>)

| 항목 | 값 |
|---|---|
| id | `abstract_input__attack` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai14abstract_input6attack` |
| 소스 | `game-ai\src\abstract_input.rs:148` |
| IR | `m04.ll` 44657~44931행 |
| 경로·가시성 | `game_ai::attack` · **pub** |
| 계층 | 입력 생성 |
| exe | `d35d10` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[201]/sig/tls/<키>`)**

- {"name": "(직접 접점 없음)", "role": "함수 본문 44657~44931 grep: LocalKey/with/thread_local 0건 — TLS 를 직접 읽거나 쓰지 않는다", "key": "-", "layout": "-", "invalidation": "-", "call_conditions": "TLS 접점은 전부 콜리 내부. 호출 순서(미러 재현 시 유지): ①Entity::can_attack(44699, L150) ②CastingTarget::check(44726, L156) ③Entity::attack_speed_mult(44739, L160) ④get_input_target(44740, L160 — INTER_CTX/POS_EVAL_CACHE 등이 있다면 이 안) ⑤(input_target None 이고 대상 가시) Effect::range_adjust(44817, L170) → Game::adjust_position(44920, L181) → safe_move_avoiding_enemy_well(44925, L183) / (비가시) safe_move_avoiding_enemy_well(44786, L185). v47_siege_stance·v48_cast_beams·champion_hp_value·position_eval_at·interaction_score 심볼은 이 함수 IR 에 0건"}

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<game_core::Input> 32B | IR 속성 writeonly sret dereferenceable(32). None = +0 i64 −1(니치 = u64::MAX, m04.ll:44703·44710·44730·44734) · Some(Input::Attack(t)) = +0 i64 2(태그 Attack, tcxdict --enum game_core::Input) + +8..+0x20 InputTarget 24B memcpy(44747~44748) · 그 밖의 경로는 safe_move_avoiding_enemy_well 이 같은 sret 를 직접 채운다(44786·44925) | 3 |
| 1 | 1 | version | usize | 이 함수 안에서 분기 없음 — get_input_target(44740)·safe_move_avoiding_enemy_well(44786·44925) 인자로만 전달 | 4 |
| 2 | 2 | rnd | &mut StdRng(320B, align 16) | readonly 없음 = &mut. 이 함수는 직접 읽고 쓰지 않는다 — get_input_target(44740) 에만 전달. 쓰기 표면은 그 콜리 소관 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | +0x930 info.team(44671~44672, bounds<2 검사) · +0x9c0 info.position@tag(as_index, entity.rs:581 인라인, 44682~44684). get_input_target·safe_move 인자로도 전달 | 4 |
| 4 | 4 | data | &OperationData(24B) | +0 cache(&AbstractGameWithCache; +0x1e0 player_champion[2][5] Option<&Entity>) · +8 context(&GameContext; +8 setting · +0x20 map — adjust_position 인자 44914~44920) | 4 |
| 5 | 5 | positioning_score | &PositioningScoreData(2760B) | 이 함수 안에선 읽지 않음 — get_input_target(44740) 에만 전달 | 4 |
| 6 | 6 | target | &Entity(1728B) | DI 이름 target(그리고 인라인 헬퍼의 self). 읽는 필드: +0x38 visible_state[team]@tag(44771~44773) · +0x68 ty@tag(is_tower, 44872) · +0x470 stat_buff_cached.radius_mult · +0x680 radius · +0x660 x · +0x668 y. CastingTarget::check·range_adjust·get_input_target 인자 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn attack(version, rnd:&mut StdRng, player, data, positioning_score, target:&Entity) -> Option<Input>
L149: let champ = data.cache.player_champion[player.info.team][player.info.position as usize]?   // 44671~44703; team>=2 → panic_bounds_check; None → return None
L150: if !champ.can_attack() { return None }                                     // 44699~44710 (L151)
L154: let effect = champ.attack_effect.as_ref()?                                 // 44716~44730: Entity+0x4c0 == −1 → None
L156: if !effect.target.check(champ, target) { return None }                     // 44725~44734 (L157) CastingTarget::check(&effect.target(+0x4b8), caster=champ, target)
L160: let input_target: Option<InputTarget> = get_input_target(version, rnd, player, data, positioning_score, target, effect, champ.attack_speed_mult())   // 44739~44740
L162: if let Some(t) = input_target { return Some(Input::Attack(t)) }           // 44741~44748: 태그 2 + 24B memcpy
L165: if target.is_visible_from(champ) {                                         // 44753~44775: champ.team Neutral → true; Player(t) → target.visible_state[t] == Visible(0)
L166:   let dx = champ.x − target.x; L167: let dy = champ.y − target.y;         // 44790~44800 (i64)
L168:   let sz = isqrt(dx*dx + dy*dy)                                            // 44802~44805
L170:   let full_range = effect.range(champ) + effect.range_adjust(champ, target) + champ.radius() + target.radius()
          // effect.range(champ) = effect.range(+0x4a0) + (champ.level−1)*growth_range(+0x4a8) + champ.stat_buff_cached.range(+0x438)   (effect.rs:26 인라인, 44807~44816·44864~44865)
          // radius() = if radius_mult==0 { radius } else { radius*(radius_mult+100)/100 }   (entity.rs:1511~1515 인라인, 44818~44863)
          // 합산 순서 IR: (stat.range + range) + (level−1)*growth + range_adjust + champ.radius + target.radius (44864~44868)
L173:   let attack_margin = if target.is_tower() { 2000 } else { 15000 }        // 44872~44876: (ty@tag & 14)==2
L178:   let from_distance = full_range.saturating_sub(attack_margin)            // 44879
L179:   let x = target.x + from_distance*dx / sz   (sdiv; sz==0 → panic div_by_zero)   // 44881~44909
L180:   let y = target.y + from_distance*dy / sz                                 // 44897~44912
L181:   let (x, y) = Game::adjust_position(data.context.map, data.context.setting, x, y)   // 44914~44922
L183:   return safe_move_avoiding_enemy_well(version, player, data, champ, x, y)   // 44925 (sret 직접 전달)
L185: } else { return safe_move_avoiding_enemy_well(version, player, data, champ, target.x, target.y) }   // 44782~44786
L188: 끝
```

**`mem` 메모리 접근 29건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L149 (44671~44672) player_champion 1차 인덱스 · bounds<2 아니면 panic_bounds_check(44677) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag (as_index) | r | L149 (44682~44684, entity.rs:581 인라인) player_champion 2차 인덱스(i32 zext) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | L149 (44685) | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity>, null=None) | r | L149 (44686~44691) `?` — None 이면 sret None(44703) | 4 | OK |  |
| 4 | Entity(champ) | 0x4c0 | attack_effect@tag (casting@tag 4B, 니치 None=−1) | r | L154 (44716~44718) `champ.attack_effect.as_ref()?` — None 이면 sret None(44730) | 4 | OK |  |
| 5 | Entity(champ) | 0x490 | attack_effect@Some.0 (&Effect 56B 시작) | r | L154 (44715) effect 포인터 = champ+0x490; range_adjust self(44817)·get_input_target 8번째 인자(44740) | 4 | OK |  |
| 6 | Entity(champ) | 0x4b8 | attack_effect@Some.0.target (CastingTarget 4B) | r | L156 (44725~44726) CastingTarget::check(&effect.target, champ, target) | 4 | OK |  |
| 7 | Entity(champ) | 0x0 | team@tag (TeamType: 0 Player / 1 Neutral) | r | L165 (44753~44755, entity.rs:1136 player_team 인라인) 태그 bit0=1(Neutral) 이면 is_visible_from = true 로 바로 가시 경로 | 4 | OK |  |
| 8 | Entity(champ) | 0x8 | team@Player.0 (팀 번호) | r | L165 (44762~44766, entity.rs:1137) bounds<2 아니면 panic(44778) | 4 | OK |  |
| 9 | Entity(target) | 0x38 | visible_state[team]@tag (VisibleState: 0 Visible / 1 Invisible / 2 Unknown, stride 24) | r | L165 (44771~44775, data.rs:122 is_visible 인라인) 태그==0 이면 가시 경로(L166~), 아니면 L185 | 4 | OK |  |
| 10 | Entity(target) | 0x660 | x | r | L166 (44793) dx = champ.x − target.x · L179 기준점 · L185 safe_move 인자(44783) | 4 | OK |  |
| 11 | Entity(target) | 0x668 | y | r | L167 (44798) · L180 · L185 (44785) | 4 | OK |  |
| 12 | Entity(champ) | 0x660 | x | r | L166 (44791) | 4 | OK |  |
| 13 | Entity(champ) | 0x668 | y | r | L167 (44797) | 4 | OK |  |
| 14 | Entity(champ) | 0x4a0 | attack_effect@Some.0.range (Effect+0x10) | r | L170 (44807~44808, effect.rs:26 Effect::range 인라인) | 4 | OK |  |
| 15 | Entity(champ) | 0x4a8 | attack_effect@Some.0.growth_range (Effect+0x18) | r | L170 (44809~44810) × (level−1) | 4 | OK |  |
| 16 | Entity(champ) | 0x5c8 | level | r | L170 (44811~44813) level−1 | 4 | OK |  |
| 17 | Entity(champ) | 0x438 | stat_buff_cached.range | r | L170 (44815~44816) Effect::range 합산 항 | 4 | OK |  |
| 18 | Entity(champ) | 0x470 | stat_buff_cached.radius_mult (i32) | r | L170 (44818~44821, entity.rs:1511~1515 radius 인라인) 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 | 4 | OK |  |
| 19 | Entity(champ) | 0x680 | radius | r | L170 (44826·44833) | 4 | OK |  |
| 20 | Entity(target) | 0x470 | stat_buff_cached.radius_mult (i32) | r | L170 (44841~44844) target.radius() | 4 | OK |  |
| 21 | Entity(target) | 0x680 | radius | r | L170 (44849·44856) | 4 | OK |  |
| 22 | Entity(target) | 0x68 | ty@tag (EntityType) | r | L173 (44872~44875, entity.rs:1386 is_tower 인라인) (tag & 14)==2 ⇔ tag∈{2 Tower, 3 Nexus} | 4 | OK |  |
| 23 | OperationData | 0x8 | context (&GameContext) | r | L181 (44914~44915) | 4 | OK |  |
| 24 | GameContext | 0x20 | map (&MapDef 28112B) | r | L181 (44916~44917) adjust_position 1번째 인자 | 4 | OK |  |
| 25 | GameContext | 0x8 | setting (&GameSetting 5432B) | r | L181 (44918~44919) adjust_position 2번째 인자 | 4 | OK |  |
| 26 | (sret) Option<Input> | 0x0 | 태그 | w | None 경로는 +0 8B 만 기록 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | −1(None: 44703 L149 champ None · 44710 L151 !can_attack · 44730 L154 attack_effect None · 44734 L157 casting_target 불일치) / 2(Attack: 44748 L163) |
| 27 | (sret) Option<Input> | 0x8 | Attack.target (InputTarget 24B) | w | Some 경로에서만 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | memcpy 24B ← %8 input_target(get_input_target 결과) (44746~44747 L162~163) |
| 28 | %8 (alloca 24B, 로컬 input_target: Option<InputTarget>) | 0x0 | get_input_target sret | w | Option<InputTarget> None 니치 = InputTarget 태그 4B −1(None variant 3 뒤 니치) | 4 | 확인불가(tcx 사전에 타입 없음) | 콜리가 채움(44740) · +0 i32 == −1 이면 None(44741~44742 L162) |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 149 | 태그 | player_champion 1차 배열 길이 2(팀) bounds 검사(44673) · L165 team bounds(44766) · Input::Attack 메모리태그 2(44748, tcxdict --enum game_core::Input) · L173 is_tower 비교값 (tag&14)==2(44875) | 3 |
| 1 | -1 | 149 | 센티널 | Option<Input>::None 니치 태그 u64::MAX (44703·44710·44730·44734) · attack_effect None 니치 i32 −1 @0x4c0 (44718) · Option<InputTarget>::None i32 −1 (44742) · level−1 의 add −1 (44813) · sdiv 오버플로 검사 sz==−1 (44886) | 4 |
| 2 | 14 | 173 | 계수 | is_tower 마스크 `ty@tag & 14 == 2` — EntityType 태그 2(Tower)·3(Nexus) 를 한 번에 잡는다(entity.rs:1386 인라인) | 4 |
| 3 | 2000 | 173 | 산출값 | attack_margin — 대상이 타워/넥서스면 사거리에서 2000(0.0625셀) 만 빼고 접근 | 4 |
| 4 | 15000 | 173 | 산출값 | attack_margin — 대상이 구조물이 아니면 사거리에서 15000(≈0.47셀) 빼고 접근(사거리 끝에 걸치지 않도록 여유) | 4 |
| 5 | 100 | 170 | 계수 | Entity::radius 인라인(entity.rs:1515): radius*(radius_mult+100)/100 — 퍼센트 배율 (44834~44836·44857~44859) | 4 |
| 6 | -9223372036854775808 | 179 | 태그 | i64::MIN — sdiv 오버플로 패닉 검사 (44887·44898) 컴파일러 삽입, 판정 아님 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 구조물 대상 접근 여유(attack_margin) | abstract_input.rs:173 (m04.ll:44876 select 2000) | 2000 | 올리면 타워/넥서스에 더 깊이(사거리 안쪽으로) 붙어 서고, 내리면 사거리 끝에서 멈춘다 — 경로 오차로 사거리 밖에 서는 빈도가 는다 | 4 | 기존 |
| 1 | 비구조물 대상 접근 여유(attack_margin) | abstract_input.rs:173 (m04.ll:44876 select 15000) | 15000 | 올리면 챔피언/미니언 대상에 더 가까이 붙어 평타 접근 지점을 잡고(피격 위험↑·이동 시간↑), 내리면 사거리 끝 지점을 목표로 삼는다 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack | game_ai::attack | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:148 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 82개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 2 | attack | game_core::Entity::attack | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1493 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 82개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | attack | game_core::EntityInfo::attack | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1153 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 82개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | attack_speed_mult | game_core::Entity::attack_speed_mult | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:2433 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | get_input_target | game_ai::abstract_input::get_input_target | in:game_ai::abstract_input | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity, &game_core::Effect, usize) -> std::option::Option<game_core::InputTarget> | game-ai\src\abstract_input.rs:345 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | is_tower | game_core::EntityType::is_tower | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1385 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 9 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 10 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 12 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 13 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | safe_move_avoiding_enemy_well | game_ai::safe_move_avoiding_enemy_well | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:122 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 3개**: `llvm.memcpy.p0.p0.i64`, `llvm.usub.sat.i64`, `target`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m07.ll:24480) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | get_input_target 이 None 을 돌려주는 조건(사거리 밖 · 회피 등)은 그 명세(r15) 소관 — 이 함수는 결과만 소비한다 | 4 |  |
| 1 | 미탐색 | safe_move_avoiding_enemy_well 의 sret 내용(Move/None 등)은 r13 명세 소관 — 이 함수는 sret 를 그대로 넘긴다 | 4 |  |
| 2 | 미탐색 | Effect::range_adjust(effect, champ, target)·CastingTarget::check·Entity::can_attack·attack_speed_mult·Game::adjust_position 내부는 game_core 경계 — 시그니처만 확인(_gcbc g06.ll:51785·86744·63321·66878 / g15.ll:72245) | 4 |  |
| 3 | 표기 불가 | L173 `is_tower()` 가 Nexus(태그 3)를 포함하는 것은 마스크 14 로 확정. 소스가 `matches!(ty, Tower\|Nexus)` 인지 별도 헬퍼인지는 표기 불가(외연 동일) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

