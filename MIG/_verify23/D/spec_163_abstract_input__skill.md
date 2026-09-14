---

### `163` abstract_input::skill — 스킬1 추상 입력: 시전 가능·대상 유효하면 get_input_target 로 Skill 입력, 아니면 사거리-여유(2k/15k) 지점으로 접근 이동

| 항목 | 값 |
|---|---|
| id | `abstract_input__skill` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai14abstract_input5skill` |
| 소스 | `game-ai\src\abstract_input.rs:199` |
| IR | `m04.ll` 44368~44654행 |
| 경로·가시성 | `game_ai::skill` · **pub** |
| 계층 | 입력 생성 |
| exe | `d35930` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<Input>(32B) | 반환 슬롯 | 4 |
| 1 | 1 | version | usize | 본문 분기 없음 — get_input_target·safe_move 에 전달만 | 4 |
| 2 | 2 | rnd | &mut StdRng(320B) | get_input_target 에 전달만 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0) | 4 |
| 4 | 4 | data | &OperationData(24B) | +0x0 cache(player_champion) · +0x8 context(map·setting → adjust_position) | 4 |
| 5 | 5 | positioning_score | &PositioningScoreData(2760B) | get_input_target 에 전달만 | 4 |
| 6 | 6 | target | &Entity(1728B) | 스킬 대상. team/visible_state/ty/x/y/radius 읽음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn skill(version, rnd, player, data, positioning_score, target) -> Option<Input>   [abstract_input.rs:199~237]
  L200: champ = data.cache.player_champion[player.team][player.position]?      // None → None
  L201: if !champ.can_skill() { L202: return None }
  L205: effect = champ.skill_effect.as_ref()?                                   // None → None
  L207: if !effect.target.check(champ, target) { L208: return None }            // CastingTarget::check (game_core)
  L211: input_target = get_input_target(version, rnd, player, data, positioning_score, target, effect, champ.cooldown_reduce(false))   // Option<InputTarget> 24B
  L213: if let Some(t) = input_target { L214: return Some(Input::Skill(t)) }    // sret +0x0=3, +0x8..=memcpy 24B
  L216: if target.is_visible_from(champ) {   // 인라인 entity.rs:1482~1483: champ.player_team()==None(Neutral) || target.visible_state[champ_team]==Visible
  L217:   dx = champ.x - target.x;  L218: dy = champ.y - target.y;  L219: sz = isqrt(dx²+dy²)
  L222:   margin = if matches!(target.ty, Tower|Nexus) { 2000 } else { 15000 }      // (tag & 14) == 2
  L227:   from_distance = effect_range_with_radii(effect, champ, target).saturating_sub(margin)
          //  effect_range_with_radii(abstract_input.rs:191~192, 인라인) =
          //    (effect.casting==Targeting ? champ.radius() : 0)
          //    + effect.range + champ.stat_buff_cached.range + effect.growth_range*(champ.level-1)   // Effect::range 인라인 effect.rs:26
          //    + effect.range_adjust(champ, target) + target.radius()
  L228:   x = target.x + (from_distance as i64 * dx) / sz      // sdiv · sz==0 → 패닉(div_by_zero) · i64::MIN/-1 → 패닉(div_overflow)
  L229:   y = target.y + (from_distance as i64 * dy) / sz
  L230:   (x, y) = Game::adjust_position(ctx.map, ctx.setting, x, y)
  L232:   return safe_move_avoiding_enemy_well(version, player, data, champ, x, y)
        } else {
  L234:   return safe_move_avoiding_enemy_well(version, player, data, champ, target.x, target.y)   // 비가시 대상: 대상 좌표로
        }
```

**`mem` 메모리 접근 24건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | ≥2 → panic_bounds_check (m04.ll:44381~44385) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 → zext 인덱스 (m04.ll:44392~44394) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m04.ll:44395) | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext → +0x20 map(&MapDef 28112B)·+0x8 setting(&GameSetting 5432B) 를 adjust_position 에 (m04.ll:44637~44643) | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | None → return None (m04.ll:44396~44401, `?` option.rs:2775) | 4 | OK |  |
| 5 | Entity | 0x4f8 | skill_effect@tag / Some.0.casting | r | -1 → None(m04.ll:44424~44426). 같은 4B 가 CastingType(0 Targeting) — effect_range_with_radii 에서 ==0 이면 caster 반지름 가산 (m04.ll:44533~44534) | 4 | OK |  |
| 6 | Entity | 0x4c8 | skill_effect@Some.0 | r | &Effect(56B) → get_input_target·range_adjust 인자 (m04.ll:44423) | 4 | OK |  |
| 7 | Entity | 0x4f0 | skill_effect@Some.0.target | r | CastingTarget(4B) → CastingTarget::check(&target_kind, champ, target) (m04.ll:44431~44432) | 4 | OK |  |
| 8 | Entity | 0x4d8 | skill_effect@Some.0.range | r | effect_range_with_radii (m04.ll:44562) | 4 | OK |  |
| 9 | Entity | 0x4e0 | skill_effect@Some.0.growth_range | r | × (level-1) (m04.ll:44564) | 4 | OK |  |
| 10 | Entity | 0x5c8 | level | r | champ.level (m04.ll:44566) | 4 | OK |  |
| 11 | Entity | 0x438 | stat_buff_cached.range | r | champ 사거리 버프 (m04.ll:44568) | 4 | OK |  |
| 12 | Entity | 0x470 | stat_buff_cached.radius_mult | r | champ(m04.ll:44538)·target(44571) Entity::radius 인라인 entity.rs:1511~1515 | 4 | OK |  |
| 13 | Entity | 0x680 | radius | r | champ·target 반지름 | 4 | OK |  |
| 14 | Entity | 0x0 | team@tag | r | champ.team: 1=Neutral 이면 player_team()=None → is_visible_from 항상 true (m04.ll:44459~44460, entity.rs:1136) | 4 | OK |  |
| 15 | Entity | 0x8 | team@Player.0 | r | champ 팀 인덱스 → target.visible_state[team] (m04.ll:44468~44470) | 4 | OK |  |
| 16 | Entity | 0x38 | visible_state[team]@tag | r | target 의 팀별 VisibleState(24B stride) 태그 0=Visible (m04.ll:44477~44480, data.rs:122 is_visible) | 4 | OK |  |
| 17 | Entity | 0x68 | ty@tag | r | target.ty & 14 == 2 ⟺ Tower(2)\|Nexus(3) → margin 2000 (m04.ll:44514~44518) | 4 | OK |  |
| 18 | Entity | 0x660 | x | r | champ.x - target.x = dx (L217) · target.x 접근 기준점 (L228·L234) | 4 | OK |  |
| 19 | Entity | 0x668 | y | r | dy (L218) · target.y | 4 | OK |  |
| 20 | (sret) | 0x0 | Option<Input>@tag | w | L200 챔피언 없음(m04.ll:44411) · L202 !can_skill(44418) · L205 skill_effect 없음(44436) · L208 CastingTarget::check 실패(44440). 8B 만 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | -1 (None) |
| 21 | (sret) | 0x0 | Option<Input>@tag | w | L214 · m04.ll:44454 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 3 (Some(Input::Skill)) |
| 22 | (sret) | 0x8 | Input::Skill.target (InputTarget 24B) | w | L213~214 · m04.ll:44453 (llvm.memcpy 24B) — InputTarget 태그 i32 +0x8, 페이로드 +0x10/+0x18 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | get_input_target 의 sret 24B memcpy |
| 23 | (sret) | 0x0 | Option<Input> 전체 | w | L234(m04.ll:44492 · 대상 비가시) / L232(m04.ll:44648 · 접근 지점) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | safe_move_avoiding_enemy_well 이 채움 |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 200 | 센티널 | Option<Input>::None 니치 태그(store i64 -1 ×4: m04.ll:44411·44418·44436·44440). 같은 -1 이 Entity.skill_effect None(i32, 44426)·get_input_target 결과 Option<InputTarget>::None(i32, 44448)·level-1(add -1, 44594) 에도 | 4 |
| 1 | 2 | 200 | 임계 | team 인덱스 bounds(len 2). 같은 2 가 L222 `(ty & 14) == 2`(Tower) 비교값(44517) | 4 |
| 2 | 3 | 214 | 태그 | Input::Skill 메모리태그 3 (m04.ll:44454) | 4 |
| 3 | 14 | 222 | 계수 | target.ty 태그 마스크 0b1110: `tag & 14 == 2` ⟺ tag ∈ {2 Tower, 3 Nexus} — 구조물 판별이 접힘 (m04.ll:44516~44517) | 4 |
| 4 | 2000 | 222 | 산출값 | 구조물(타워/넥서스) 대상일 때 사거리 여유 margin (m04.ll:44518 select). 사거리에서 이만큼 안쪽 지점으로 접근 | 4 |
| 5 | 15000 | 222 | 산출값 | 비구조물 대상일 때 사거리 여유 margin(≈0.47셀) (m04.ll:44518 select) | 4 |
| 6 | 0 | 216 | 태그 | VisibleState::Visible 태그 0 (44480) · CastingType::Targeting 태그 0(L191 effect_range_with_radii, 44534: Targeting 이면 caster_r=champ.radius() 아니면 0) · isqrt==0 → div_by_zero 패닉(44615) | 4 |
| 7 | 100 | 227 | 계수 | Entity::radius 인라인(entity.rs:1515): radius*(radius_mult+100)/100 — champ(44555~44556)·target(44588~44589) 두 번 | 4 |
| 8 | 1 | 227 | 미상 | growth_range*(level-1): `add i64 %106, -1` (m04.ll:44594) — 상수 1 리터럴 없음, -1 가산으로 접힘 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 구조물 대상 접근 사거리 여유 | abstract_input.rs:222 | 2000 | 올리면 타워/넥서스에 더 안쪽까지 붙어서 시전 지점을 잡는다(사거리 끝에서 margin 만큼 안쪽) | 4 | 기존 |
| 1 | 일반 대상 접근 사거리 여유 | abstract_input.rs:222 | 15000 | 올리면 대상에 더 가까이 접근한 뒤 시전(스킬 사거리 - 15k 지점). 내리면 사거리 끝에서 시전 시도 → 빗나감/캔슬 증가 가능 | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | cooldown_reduce | game_core::Entity::cooldown_reduce | pub | fn(&game_core::Entity, bool) -> usize | game-core\src\simulation\entity.rs:2437 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | effect_range_with_radii | game_ai::abstract_input::effect_range_with_radii | in:game_ai::abstract_input | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\abstract_input.rs:190 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 5 | get_input_target | game_ai::abstract_input::get_input_target | in:game_ai::abstract_input | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity, &game_core::Effect, usize) -> std::option::Option<game_core::InputTarget> | game-ai\src\abstract_input.rs:345 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 12 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | safe_move_avoiding_enemy_well | game_ai::safe_move_avoiding_enemy_well | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:122 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | skill | game_ai::skill | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:199 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 65개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 15 | skill | game_core::Entity::skill | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1664 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 65개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 16 | skill | game_core::ChampionInfo::skill | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1085 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 65개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
</details>

**호출처 1곳** (m07.ll:11922) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | get_input_target(sret Option<InputTarget> 24B, version, &mut StdRng, &PlayerState, &OperationData, &PositioningScoreData, &Entity target, &Effect, cooldown_reduce: u64) — internal fastcc(m04.ll:33111) · 이 배치 담당 아님. 시그니처·반환(태그 i32 -1=None) 만 | 4 |  |
| 1 | 미탐색 | safe_move_avoiding_enemy_well(sret 32B, version, &PlayerState, &OperationData, &Entity champ, x, y) — 잎(계약만): 호출 m04.ll:44492·44648 | 4 |  |
| 2 | 미탐색 | CastingTarget::check(&CastingTarget(4B), &Entity caster, &Entity target)->bool · Entity::can_skill(&Entity)->bool · Entity::cooldown_reduce(&Entity, bool)->u64(range 1..2^31) · Effect::range_adjust(&Effect,&Entity,&Entity)->u64 · utils::isqrt(u64)->u64 · Game::adjust_position(&MapDef,&GameSetting,x,y)->(x,y) — game_core 경계, 시그니처만 | 4 |  |
| 3 | 미탐색 | L216 is_visible_from 의 Neutral 분기가 실제 도달 가능한지(챔피언 team 은 항상 Player 로 추정) — 이 함수 범위 밖 | 5 |  |
| 4 | 표기 불가 | L228 `from_distance as i64 * dx` 의 소스 캐스팅 위치(usize→i64) — 값 동일(표기 불가) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

