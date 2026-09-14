---

### `164` abstract_input::skill2 — 스킬2 추상 입력: 레벨>2·시전 가능·대상 유효하면 get_input_target 로 Skill2 입력, 아니면 사거리-여유(2k/15k) 지점으로 접근 이동

| 항목 | 값 |
|---|---|
| id | `abstract_input__skill2` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai14abstract_input6skill2` |
| 소스 | `game-ai\src\abstract_input.rs:239` |
| IR | `m04.ll` 44934~45224행 |
| 경로·가시성 | `game_ai::skill2` · **pub** |
| 계층 | 입력 생성 |
| exe | `d360a0` (None) · None바이트 · None명령 |
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
fn skill2(version, rnd, player, data, positioning_score, target) -> Option<Input>   [abstract_input.rs:239~277] — skill() 과 동형(차이: can_skill2 · skill2_effect(레벨>2 게이트) · 태그 4)
  L240: champ = data.cache.player_champion[player.team][player.position]?      // None → None
  L241: if !champ.can_skill2() { L242: return None }
  L245: effect = champ.skill2_effect()?      // 인라인 entity.rs:1693: if level > 2 { &skill2_effect(+0x500) } else { &None } → None → None
  L247: if !effect.target.check(champ, target) { L248: return None }
  L251: input_target = get_input_target(version, rnd, player, data, positioning_score, target, effect, champ.cooldown_reduce(false))
  L253: if let Some(t) = input_target { L254: return Some(Input::Skill2(t)) }    // sret +0x0=4, +0x8..=memcpy 24B
  L256: if target.is_visible_from(champ) {   // champ.player_team()==None || target.visible_state[champ_team]==Visible
  L257:   dx = champ.x - target.x;  L258: dy = champ.y - target.y;  L259: sz = isqrt(dx²+dy²)
  L262:   margin = if matches!(target.ty, Tower|Nexus) { 2000 } else { 15000 }
  L267:   from_distance = effect_range_with_radii(effect, champ, target).saturating_sub(margin)
          //  = (effect.casting==Targeting ? champ.radius() : 0) + effect.range + champ.stat_buff_cached.range + effect.growth_range*(level-1) + effect.range_adjust(champ,target) + target.radius()
  L268:   x = target.x + (from_distance as i64 * dx) / sz      // sdiv · sz==0 → 패닉
  L269:   y = target.y + (from_distance as i64 * dy) / sz
  L270:   (x, y) = Game::adjust_position(ctx.map, ctx.setting, x, y)
  L272:   return safe_move_avoiding_enemy_well(version, player, data, champ, x, y)
        } else {
  L274:   return safe_move_avoiding_enemy_well(version, player, data, champ, target.x, target.y)
        }
```

**`mem` 메모리 접근 24건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | ≥2 → panic_bounds_check (m04.ll:44947~44951) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 → zext 인덱스 (m04.ll:44958~44960) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m04.ll:44961) | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext → +0x20 map(&MapDef 28112B)·+0x8 setting(&GameSetting 5432B) 를 adjust_position 에 (m04.ll:45207~45213) | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | None → return None (m04.ll:44962~44967, `?` option.rs:2775) | 4 | OK |  |
| 5 | Entity | 0x530 | skill2_effect@tag / Some.0.casting | r | eff+0x30 (m04.ll:44995~44997 · eff 는 select 결과라 절대 오프셋 gep 없음): -1 → None. 같은 4B 가 CastingType(0 Targeting) — effect_range_with_radii 에서 ==0 이면 caster 반지름 가산 (m04.ll:45105) | 4 | OK |  |
| 6 | Entity | 0x500 | skill2_effect@Some.0 | r | &Effect(56B). L245 인라인 Entity::skill2_effect(entity.rs:1693): level>2 ? &champ.skill2_effect : &정적 None(@anon.16) (m04.ll:44989~44993 select) | 4 | OK |  |
| 7 | Entity | 0x528 | skill2_effect@Some.0.target | r | eff+0x28 CastingTarget(4B) → CastingTarget::check (m04.ll:45002~45003) | 4 | OK |  |
| 8 | Entity | 0x510 | skill2_effect@Some.0.range | r | eff+0x10 (m04.ll:45133) | 4 | OK |  |
| 9 | Entity | 0x518 | skill2_effect@Some.0.growth_range | r | eff+0x18 × (level-1) (m04.ll:45135) | 4 | OK |  |
| 10 | Entity | 0x5c8 | level | r | champ.level — L245 `> 2` 게이트(m04.ll:44989~44991) · growth 곱(45137) | 4 | OK |  |
| 11 | Entity | 0x438 | stat_buff_cached.range | r | champ 사거리 버프 (m04.ll:45138) | 4 | OK |  |
| 12 | Entity | 0x470 | stat_buff_cached.radius_mult | r | champ(m04.ll:45109)·target(45141) Entity::radius 인라인 entity.rs:1511~1515 | 4 | OK |  |
| 13 | Entity | 0x680 | radius | r | champ·target 반지름 | 4 | OK |  |
| 14 | Entity | 0x0 | team@tag | r | champ.team: 1=Neutral 이면 player_team()=None → is_visible_from 항상 true (m04.ll:45030~45031, entity.rs:1136) | 4 | OK |  |
| 15 | Entity | 0x8 | team@Player.0 | r | champ 팀 인덱스 → target.visible_state[team] (m04.ll:45039~45041) | 4 | OK |  |
| 16 | Entity | 0x38 | visible_state[team]@tag | r | target 의 팀별 VisibleState(24B stride) 태그 0=Visible (m04.ll:45048~45051, data.rs:122 is_visible) | 4 | OK |  |
| 17 | Entity | 0x68 | ty@tag | r | target.ty & 14 == 2 ⟺ Tower(2)\|Nexus(3) → margin 2000 (m04.ll:45085~45089) | 4 | OK |  |
| 18 | Entity | 0x660 | x | r | champ.x - target.x = dx (L257) · target.x 접근 기준점 (L268·L274) | 4 | OK |  |
| 19 | Entity | 0x668 | y | r | dy (L258) · target.y | 4 | OK |  |
| 20 | (sret) | 0x0 | Option<Input>@tag | w | L240 챔피언 없음(m04.ll:44978) · L242 !can_skill2(44985) · L245 skill2_effect 없음/레벨≤2(45007) · L248 CastingTarget::check 실패(45011). 8B 만 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | -1 (None) |
| 21 | (sret) | 0x0 | Option<Input>@tag | w | L254 · m04.ll:45025 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 4 (Some(Input::Skill2)) |
| 22 | (sret) | 0x8 | Input::Skill2.target (InputTarget 24B) | w | L253~254 · m04.ll:45024 (llvm.memcpy 24B) — InputTarget 태그 i32 +0x8, 페이로드 +0x10/+0x18 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | get_input_target 의 sret 24B memcpy |
| 23 | (sret) | 0x0 | Option<Input> 전체 | w | L274(m04.ll:45063 · 대상 비가시) / L272(m04.ll:45218 · 접근 지점) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | safe_move_avoiding_enemy_well 이 채움 |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 240 | 센티널 | Option<Input>::None 니치 태그(store i64 -1 ×4: m04.ll:44978·44985·45007·45011). 같은 -1 이 skill2_effect None(i32, 44997)·get_input_target 결과 Option<InputTarget>::None(i32, 45019)·level-1(add -1, 45164) 에도 | 4 |
| 1 | 2 | 245 | 임계 | `champ.level > 2` — Entity::skill2_effect 인라인(entity.rs:1693) 스킬2 해금 레벨 게이트 (m04.ll:44991). 같은 2 가 team bounds(len 2)·L262 `(ty & 14) == 2`(Tower) 비교값(45088) | 4 |
| 2 | 4 | 254 | 태그 | Input::Skill2 메모리태그 4 (m04.ll:45025) | 4 |
| 3 | 14 | 262 | 계수 | target.ty 태그 마스크 0b1110: `tag & 14 == 2` ⟺ tag ∈ {2 Tower, 3 Nexus} — 구조물 판별이 접힘 (m04.ll:45087~45088) | 4 |
| 4 | 2000 | 262 | 산출값 | 구조물(타워/넥서스) 대상일 때 사거리 여유 margin (m04.ll:45089 select) | 4 |
| 5 | 15000 | 262 | 산출값 | 비구조물 대상일 때 사거리 여유 margin(≈0.47셀) (m04.ll:45089 select) | 4 |
| 6 | 0 | 256 | 태그 | VisibleState::Visible 태그 0 (45051) · CastingType::Targeting 태그 0(L191 effect_range_with_radii, 45105) · isqrt==0 → div_by_zero 패닉(45185) | 4 |
| 7 | 100 | 267 | 계수 | Entity::radius 인라인(entity.rs:1515): radius*(radius_mult+100)/100 — champ(45126~45127)·target(45158~45159) | 4 |
| 8 | 1 | 267 | 미상 | growth_range*(level-1): `add i64 %109, -1` (m04.ll:45164) — 상수 1 리터럴 없음, -1 가산으로 접힘 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 구조물 대상 접근 사거리 여유 | abstract_input.rs:262 | 2000 | 올리면 타워/넥서스에 더 안쪽까지 붙어서 시전 지점을 잡는다(사거리 끝에서 margin 만큼 안쪽) | 4 | 기존 |
| 1 | 일반 대상 접근 사거리 여유 | abstract_input.rs:262 | 15000 | 올리면 대상에 더 가까이 접근한 뒤 시전(스킬 사거리 - 15k 지점). 내리면 사거리 끝에서 시전 시도 → 빗나감/캔슬 증가 가능 | 4 | 기존 |
| 2 | 스킬2 해금 레벨(인라인된 game_core 게이트) | entity.rs:1693 (Entity::skill2_effect, m04.ll:44991 인라인) | 2 | game_core 상수 — 레벨 ≤2 면 이 함수는 항상 None. 정보성(여기서 바꾸는 노브는 아님) | 4 | 기존 |

<details><summary>`callees` 피호출자 21건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
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
| 17 | skill2 | game_ai::skill2 | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:239 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 65개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 18 | skill2 | game_core::Entity::skill2 | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1668 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 65개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 19 | skill2 | game_core::ChampionInfo::skill2 | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1086 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 65개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 20 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

**호출처 1곳** (m07.ll:12437) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | get_input_target(sret Option<InputTarget> 24B, version, &mut StdRng, &PlayerState, &OperationData, &PositioningScoreData, &Entity target, &Effect, cooldown_reduce: u64) — internal fastcc(m04.ll:33111) · 이 배치 담당 아님. 시그니처·반환(태그 i32 -1=None) 만 | 4 |  |
| 1 | 미탐색 | safe_move_avoiding_enemy_well(sret 32B, version, &PlayerState, &OperationData, &Entity champ, x, y) — 잎(계약만): 호출 m04.ll:45063·45218 | 4 |  |
| 2 | 미탐색 | CastingTarget::check(&CastingTarget(4B), &Entity caster, &Entity target)->bool · Entity::can_skill2(&Entity)->bool · Entity::cooldown_reduce(&Entity, bool)->u64(range 1..2^31) · Effect::range_adjust(&Effect,&Entity,&Entity)->u64 · utils::isqrt(u64)->u64 · Game::adjust_position(&MapDef,&GameSetting,x,y)->(x,y) — game_core 경계, 시그니처만 | 4 |  |
| 3 | 미탐색 | L256 is_visible_from 의 Neutral 분기가 실제 도달 가능한지(챔피언 team 은 항상 Player 로 추정) — 이 함수 범위 밖 | 5 |  |
| 4 | 표기 불가 | L268 `from_distance as i64 * dx` 의 소스 캐스팅 위치(usize→i64) — 값 동일(표기 불가) | 4 |  |
| 5 | 미탐색 | skill2_effect 필드 절대 오프셋(0x500/0x510/0x518/0x528/0x530)은 tcxdict 로 확정했으나 본문 gep 는 select 된 eff 포인터 기준 상대(+16/+24/+40/+48)라 C3 경고가 날 수 있음 — 값 자체는 정합 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

