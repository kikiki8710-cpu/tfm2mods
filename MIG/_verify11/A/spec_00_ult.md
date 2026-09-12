---

### `00` ult — 궁극기 사용 입력을 만들거나, 못 쓰면 사거리 확보용 이동 입력을 만든다

| 항목 | 값 |
|---|---|
| id | `abstract_input__ult` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai14abstract_input3ult` |
| 소스 | `game-ai\src\abstract_input.rs:280` |
| IR | `m04.ll` 43967~44365행 |
| 경로·가시성 | `game_ai::ult` · **pub** |
| 계층 | 입력 생성 |
| exe | `d354c0` (abstract_input) · 1135바이트 · 269명령 |
| 라운드 | 기준 `r6` · 통과 6회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | &mut Option<Input> (32B) | 반환값 출력 슬롯. +0x0=태그(-1=None, 5=Input::Ult), +0x8=InputTarget(24B) | 4 |
| 1 | 1 | version | usize | AI 버전 게이트. 이 함수 본문에선 분기 없음 — get_input_target / safe_move_avoiding_enemy_well 로 그대로 전달만 한다 | 4 |
| 2 | 2 | rnd | &mut rand::rngs::std::StdRng (320B) | 본문에서 직접 안 씀. get_input_target 으로 전달만 | 4 |
| 3 | 3 | player | &PlayerState (2528B) | info.team(0x930) / info.position(0x9c0) 을 읽어 자기 챔피언을 찾는 데 씀 | 4 |
| 4 | 4 | data | &OperationData (24B) | cache(0x0)=AbstractGameWithCache, context(0x8)=GameContext | 4 |
| 5 | 5 | positioning_score | &PositioningScoreData (2760B) | 본문에서 직접 안 씀. get_input_target 으로 전달만 | 4 |
| 6 | 6 | target | &Entity (1728B) | 궁극기를 쓸 상대(적) 엔티티. 거리·가시성·반경 계산의 기준 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn ult(version, rnd, player, data, positioning_score, target: &Entity) -> Option<Input>

[281] team = player.info.team; assert team < 2
 pos = player.info.position.as_index() // Position::as_index, entity.rs:580-581 (0..4)
 champ = data.cache.player_champion[team][pos] // AbstractGameWithCache+0x1e0, [[Option<&Entity>;5];2]
 if champ.is_null() { return None } // Option::branch, option.rs:2775 → from_residual 2790

[282] if !champ.can_ult() { return None } // [283]

[286] effect = champ.ult_effect().as_ref() // entity.rs:1700-1701 인라인:
 // ult_effect() = if champ.level(0x5c8) > 4 { &champ.ult_effect(0x538) } else { &정적 None }
 // as_ref() = Option<Effect> 니치 판별자 = effect+0x30(i32); -1 이면 None
 if effect.is_none() { return None }

[288] if !CastingTarget::check(&effect.target /*+0x28*/, champ, target) { return None } // [289]

[293] input_target: Option<InputTarget> = // 24B, 니치 -1 = None
 match effect.casting /* +0x30, 0..3 */ {
 1 /*Position*/ if (*effect.ty)[vtable+0x118]() == true => { // [296]
 range = linear_cast_range_with_margin(effect, champ, target) // [297]
 if distance_sq(champ.x, champ.y, target.x, target.y) > range*range { None /*→ 이동 경로*/ } // [299]
 else { GET_INPUT_TARGET }
 }
 2 /*Direction*/ if (*effect.ty)[vtable+0xf8]().0 == 1 => { // [310]
 range = linear_cast_range_with_margin(effect, champ, target) // [311]
 if distance_sq(champ.x, champ.y, target.x, target.y) > range*range { None } // [313]
 else { GET_INPUT_TARGET }
 }
 _ /*Targeting(0), None(3), 위 가드 실패*/ => GET_INPUT_TARGET
 }
 where GET_INPUT_TARGET =
 cr = champ.cooldown_reduce(true)
 get_input_target(version, rnd, player, data, positioning_score, target, effect, cr)
 // ※ distance_sq(utils.rs:6-9) = abs_diff(x1,x2)^2 + abs_diff(y1,y2)^2 (양쪽 다 인라인)

[325] if let Some(t) = input_target { return Some(Input::Ult { target: t }) } // [326] 태그 5, 페이로드 +0x8

// --- 여기부터는 '지금은 궁을 못 쓴다' → 자리잡기 이동 ---
[328] visible = target.is_visible_from(champ) // entity.rs:1481-1483 인라인:
 // champ.team 태그(0x0)가 Neutral(1)이면 → true 취급
 // Player(0)면 team=champ.team(0x8); assert team<2;
 // target.visible_state[team](0x38, 24B stride).is_visible() // data.rs:121-122, 태그 0(Visible)==true
 if !visible {
[340] return safe_move_avoiding_enemy_well(version, player, data, champ, target.x, target.y)
 }

[329] dx = champ.x - target.x // wrapping sub (i64)
[330] dy = champ.y - target.y
[331] sz = isqrt(dx*dx + dy*dy) // game_core::utils::isqrt; sz==0 이면 아래 나눗셈이 div-by-zero 패닉
[333] total = effect_range_with_radii(effect, champ, target) // abstract_input.rs:190-192 인라인:
 // caster_r = if effect.casting == Targeting(0) { champ.radius() } else { 0 } // 191
 // Effect::range(effect, champ) = effect.range(0x10) // ★**caster_r 는 여기 없다** — abstract_input.rs:191 에서 밖에서 더해진다
 // + champ.stat_buff_cached.range(0x438)
 // + (champ.level - 1) * effect.growth_range(0x18) // effect.rs:26
 // total = Effect::range(..) + **caster_r** // ★caster_r 는 `effect_range_with_radii`(abstract_input.rs:191) 에서 **밖에서** 더해진다
// + Effect::range_adjust(effect, champ, target)
 // + target.radius() // 192
 // Entity::radius() (entity.rs:1509-1515):
 // let mult = <stat_buff_cached.radius_mult(0x470)> as usize; // ★`as usize` = i32 -> usize 부호확장(sext)
 // if mult == 0 { e.radius(0x680) } else { e.radius * (100 + mult) / 100 } // wrapping usize 산술
// ⚠`as usize` 를 빼면 **음수 radius_mult 에서 동작이 갈린다**(거대 usize 로 접혀 반경 오버플로)
 from_distance = total.saturating_sub(150000)
[334] x = target.x + (from_distance * dx) / sz // sdiv (부호 있음)
[335] y = target.y + (from_distance * dy) / sz
[336] (x, y) = Game::adjust_position(data.context.map, data.context.setting, x, y)
[338] return safe_move_avoiding_enemy_well(version, player, data, champ, x, y)

// 요약: champ 에서 target 을 향한 방향으로, target 으로부터 (총사거리-150000) 만큼
// 떨어진 지점을 잡아 그리로 이동한다 = '궁 사거리 안쪽으로 파고들기'.
```

**`mem` 메모리 접근 29건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState(=GamePlayer info) | 0x930 | info.team | r | usize. player_champion 배열의 1차 인덱스. 2 미만인지 bounds-check(len=2) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 1 | PlayerState(=GamePlayer info) | 0x9c0 | info.position | r | Position(4B, range 0..4). Position::as_index() 로 2차 인덱스(0..4) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext. adjust_position 인자 뽑는 데만 씀 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] (80B). [team][position] 로 자기 챔피언 champ 를 얻는다. null=None · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 5 | GameContext | 0x8 | setting | r | &GameSetting(5432B). Game::adjust_position 2번째 인자 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 6 | GameContext | 0x20 | map | r | &MapDef(28112B). Game::adjust_position 1번째 인자 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 7 | Entity(champ) | 0x0 | team(태그) | r | TeamType 판별자(i64, range 0..1). 0=Player, 1=Neutral. TeamType::player_team() 인라인 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 8 | Entity(champ) | 0x8 | team(페이로드) | r | Player 일 때의 팀 인덱스 usize. visible_state 인덱스로 씀(bounds-check len=2) · tcx 정본 대조( _verify5/A/A5_o2.tsv TeamType::Player payload@+0x8=1 실측 + A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 9 | Entity(target) | 0x38 | visible_state | r | [VisibleState;2] (원소 24B: {i64 태그,[2 x i64]}). target.visible_state[champ.team] 의 태그를 읽는다 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 10 | Entity(champ) | 0x438 | stat_buff_cached.range | r | usize. stat_buff_cached(0x370)+0xc8. Effect::range 합산항 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 11 | Entity(champ/target) | 0x470 | stat_buff_cached.radius_mult | r | i32. stat_buff_cached(0x370)+0x100. Entity::radius() 에서 %보정 ★`as usize` 로 **부호확장**돼 쓰인다(MIR entity.rs:1511 `_2 = move _3 as usize (IntToInt)`, IR `sext i32 %9 to i64` @m04.ll:43306 · `%33` @43339). 이후 `100 + mult` 는 wrapping usize 산술 | 3 | OK |  |
| 12 | Entity(champ) | 0x538 | ult_effect | r | Option<Effect>(56B). Entity::ult_effect() 인라인 — level>4 일 때만 이 필드, 아니면 정적 None · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 13 | Entity(champ) | 0x5c8 | level | r | usize. level>4 게이트 + Effect::range 의 (level-1)*growth_range · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 14 | Entity(champ/target) | 0x660 | x | r | u64 좌표 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 15 | Entity(champ/target) | 0x668 | y | r | u64 좌표 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 16 | Entity(champ/target) | 0x680 | radius | r | usize. Entity::radius() 원본값 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 17 | Effect | 0x0 | ty(Arc 데이터 포인터) | r | Arc<dyn EffectType> 의 앞 워드(ArcInner*) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 18 | Effect | 0x8 | ty(vtable 포인터) | r | Arc<dyn EffectType> 의 뒤 워드. 여기서 +0x10=align, +0xf8/+0x118=메서드 슬롯을 읽는다 · tcx 정본 대조( A6_o6.tsv(실주소 차 보충, 10/10 MATCH) EFFECT_ARC word1 == dyn 팻포인터 vtable 주소 일치) | 3 | OK |  |
| 19 | Effect | 0x10 | range | r | u64. 기본 사거리 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 20 | Effect | 0x18 | growth_range | r | u64. 레벨당 사거리 증가분 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 21 | Effect | 0x28 | target | r | CastingTarget(4B). CastingTarget::check 의 self · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 22 | Effect | 0x30 | casting | r | CastingType(i32). ★Option<Effect> 의 니치도 같은 자리 — range -1..3 로 읽으면 -1=None, range 0..3 로 읽으면 0=Targeting/1=Position/2=Direction/3=None · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |  |
| 23 | EffectType::vtable | 0x10 | align | r | Arc 페이로드 오프셋 계산용(ABI). 슬롯 배치 = 0x0 drop / 0x8 size / 0x10 align / 0x18~ 메서드 · tcx 정본 대조(8차 배치A: `divtable` 정본 대조 — `python -X utf8 divtable.py EffectType` 가 `EffectType` vtable 전역(`@anon.75300de3978c94cc946f88c448be65c0.1131`, g02.ll · 슬롯 35개 · 296B)을 찾아 `0x0 = drop_glue`, 메서드는 `0x18` 부터임을 실측했고(같은 실행에서 `0xf8 = linear_move_speed`·`0x118 = on_caster` 도 재확인 — mem[24]/mem[25] 와 일치), `divtable.py` 가 문서화한 Rust vtable 레이아웃이 `0x00 drop / 0x08 size / 0x10 align / 0x18~ 메서드` 다. IR 이 그 값을 실제로 **정렬값으로** 쓰는 것도 확인했다: m04.ll:44062 `%48 = getelementptr inbounds nuw i8, ptr %47, i64 16` → 44063 `%49 = load i64, ptr %48, align 8, !range !12998, !invariant.load !8` → 44064 `%50 = add nsw i64 %49, -1` → 44065 `%51 = and i64 %50, -16` → 44066 `%52 = getelementptr inbounds nuw i8, ptr %45, i64 %51`(Arc 페이로드 주소). 곧 `+0x10` 이 align 이라는 주장은 IR 단독(ev4)이 아니라 vtable 전역 덤프로 교차검증된다. ⚠`chk` 칸은 파생이라 여기서 못 고친다 — 현재 `확인불가(vtable 슬롯)` 인데 이는 `tcxaudit` 이 ADT 가 아닌 vtable 을 못 보기 때문이고(METHOD_MAP ⑦ 한계), 근거 부재가 아니다.) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 24 | EffectType::vtable | 0xf8 | linear_move_speed (EffectType vtable +0xf8) | r | fn(&self)->(i64,i64). Direction 캐스팅일 때 .0==1 인지 검사 · tcx 정본 대조(7차 배치A: shared.EffectType_vtable +0xf8 = linear_move_speed · IR m04.ll:44085 gep +248 → 44087 `tail call { i64, i64 }` (abstract_input.rs:310)) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 25 | EffectType::vtable | 0x118 | on_caster (EffectType vtable +0x118) | r | fn(&self)->bool. Position 캐스팅일 때 참이어야 사거리 검사로 진행 · tcx 정본 대조(7차 배치A: shared.EffectType_vtable +0x118 = on_caster · IR m04.ll:44068 gep +280 → 44070 `tail call .. i1` (abstract_input.rs:296)) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 26 | Option<Input> (sret 반환슬롯) | 0x0 | 판별자 | w | -1 은 4곳(챔피언 없음 281 / can_ult 실패 283 / ult_effect None 286 / CastingTarget::check 실패 289)에서 저장 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK | -1 (None) 또는 5 (Input::Ult) |
| 27 | Option<Input> (sret 반환슬롯) | 0x8 | Input::Ult.target | w | memcpy 24B. Input::Ult 페이로드는 enum+0x8 에 위치(dienum Input 5) | 3 | OK | get_input_target 이 채운 InputTarget 24B |
| 28 | Option<Input> (sret 반환슬롯) | 0x0 | 전체 32B | w | 궁을 못 쓰는 경우 이동 입력으로 덮어씀(338행/340행) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK | safe_move_avoiding_enemy_well 의 sret 출력 |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 281 | 임계 | player_champion 1차 배열 길이(팀 수 2) — `icmp ult i64 %10, 2`(m04.ll:43983) 의 비교 상한 = 팀 첨자 bounds-check. entity.rs:1483 의 visible_state 첨자에도 같은 len=2 (★7차 배치A: 이 값은 첨자가 아니라 **비교 상한** 이라 kind=임계) · tcx 정본 대조(10차 배치A: tcxdict AbstractGameWithCache → `0x1e0 player_champion [[std::option::Option<&Entity>; 5_usize]; 2_usize] (80B)` ⟹ 1차 축 길이 = 2(팀 수)가 컴파일러 정본으로 확정된다. IR `icmp ult i64 %10, 2`(m04.ll:43983)는 그 길이의 bounds-check) | 3 |
| 1 | 4 | 286 | 임계 | ★궁극기 해금 레벨 게이트. Entity::ult_effect(entity.rs:1701) 인라인 — champ.level > 4 (즉 레벨 5부터)여야 ult_effect 를 반환, 아니면 정적 None · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — level 4 → ult_eff_some=false / level 5 → true (A6_o2 visible 4/5)) | 2 |
| 2 | -1 | 281 | 센티널 | Option 의 None 센티넬(니치값). 반환 Option<Input> 의 None 저장값이자, Option<Effect>(effect+0x30) / Option<InputTarget> 의 니치 None 값 · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — Option<Input>::None tag@+0x0 = -1 실측) | 2 |
| 3 | 1 | 293 | 태그 | CastingType::Position 태그(switch case). 또한 310행 vtable(+0xf8) 반환 튜플 .0 == 1 비교값 · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — CastingType::Position = 1 실측) | 2 |
| 4 | 2 | 293 | 태그 | CastingType::Direction 태그(switch case) · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — CastingType::Direction = 2 실측) | 2 |
| 5 | 0 | 191 | 태그 | CastingType::Targeting 태그. effect_range_with_radii(abstract_input.rs:191)에서 casting==Targeting 일 때만 시전자 반경을 사거리에 더한다 · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — casting=Targeting 만 caster_r_ON 이 MATCH_ULT=YES, 나머지 3값은 caster_r_OFF 가 YES) | 2 |
| 6 | 0 | 328 | 태그 | VisibleState::Visible 태그. target.visible_state[champ.team] 태그가 0이면 '보인다' · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — VisibleState::Visible = 0 / Unknown = 2 실측 + invisible 케이스가 L340 경로로 갈림) | 2 |
| 7 | 100 | 333 | 계수 | Entity::radius(entity.rs:1515) 의 퍼센트 기준 — radius*(radius_mult+100)/100. radius_mult=0 이면 원본 radius 그대로 · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — radius_mult = 0/50/-50/100/-100/-99 스윕 — radius()=raw*(100+mult)/100 이 6/6 일치, 기준 1000 가설 기각) | 2 |
| 8 | 5 | 326 | 태그 | Input::Ult 의 열거형 태그값(dienum Input 5) · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — Some(Input::Ult) tag@+0x0 = 5 실측) | 2 |
| 9 | 150000 | 333 | 미상 | ★이동 목표 산정 시 총사거리에서 빼는 여유분(saturating_sub). 셀=32000 기준 약 4.7셀. 사거리 끝보다 이만큼 안쪽에 서려고 접근한다 · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — margin 149999/150000/150001 중 150000 만 MATCH_ULT=YES) | 2 |

**`knobs` 조정점 9건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 궁극기 해금 레벨 | abstract_input.rs:286 에서 인라인된 entity.rs:1701 (IR m04.ll 44025행 `icmp ugt i64 %30, 4`) | 4 | champ.level > 4 여야 ult_effect 가 Some. 낮추면 더 이른 레벨부터 AI 가 궁 입력을 만들고, 높이면 늦어진다. 실패 시 이 함수는 즉시 None(이동조차 안 함) · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — level 4 → None / 5 → 이동입력 (A6_o2)) | 2 | 기존 |
| 1 | 궁 사거리 확보 이동의 여유분 | abstract_input.rs:333 (IR m04.ll 44313행 `llvm.usub.sat.i64(i64 %205, i64 150000)`) | 150000 | 이동 목표는 target 에서 (총사거리 - 150000) 거리. 올리면 target 에 더 바짝 붙고(= 더 깊이 들어감), 내리면 사거리 끝자락에 머문다. 총사거리 ≤ 150000 이면 saturating 으로 0 → target 좌표 자체가 목표 · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — margin ±1 로 좌표가 1 씩 움직이고 150000 만 일치) | 2 | 기존 |
| 2 | 시전자 반경 가산 조건 | abstract_input.rs:191 (IR m04.ll 44245~44247행 `icmp eq i32 %158, 0` = casting==Targeting · 그 결과가 모이는 caster_r phi 는 44272 `phi i64 [ 0, %143 ], [ %166, %164 ], [ %173, %167 ]`) | 0 | CastingType::Targeting 일 때만 champ.radius() 가 총사거리에 더해진다. 조건을 없애면 Position/Direction 궁도 자기 반경만큼 더 멀리서 자리잡는다 · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — casting==Targeting 일 때만 champ.radius() 가 총사거리에 더해짐(caster_r_ON/OFF 대조)) | 2 | 기존 |
| 3 | 반경 배율 기준값 | abstract_input.rs:333 에서 인라인된 entity.rs:1515 (IR m04.ll 44266·44268행 `add nsw i64 %168, 100` / `udiv i64 %172, 100` — champ 반경. target 반경의 같은 식은 44298·44300) | 100 | radius*(radius_mult+100)/100. 기준을 낮추면 같은 radius_mult 로도 반경이 크게 잡혀 총사거리가 늘고 더 멀리서 자리잡는다 · 오라클 실행 확증( A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv — radius_mult 스윕으로 기준값 100 판별(1000 가설 기각), 음수 mult 에서 sext 확정) | 2 | 기존 |
| 4 | 궁 봉인 CC 집합 | can_ult _gcbc/g06.ll:81816~81818 ((tag-6) u< -4) | ∉{2,3,4,5} | Bind·BlockAttack 만 궁 허용. 여기를 바꾸면 예컨대 Taunt 중 궁 허용 가능 | 4 | 신규 |
| 5 | cooltime_use_count(충전 수) | Action vtable +0xa8, 디폴트 1 (_gcbc/g02.ll:306879) | 1 | 2 이상으로 만들면 그 스킬이 충전형이 됨(쿨 절반만 차도 사용 가능) | 4 | 신규 |
| 6 | 궁 캐스팅 게이트 술어 | abstract_input.rs:296(on_caster) / 310(linear_move_speed) — m04.ll:44068 / 44085 | Position → on_caster()(vt+0x118) · Direction → linear_move_speed().is_some()(vt+0xf8) | Position 캐스팅은 on_caster, Direction 캐스팅은 linear_move_speed.is_some(). 이 둘을 뒤집으면 논타깃/방향형 궁의 사거리 검사 유무를 바꾸는 개입점 | 4 | 신규 |
| 7 | v2+ 우물회피 거리제곱 임계 | abstract_input.rs:134 | 4,000,000 | ⚠v1 은 무조건 None(이동 포기), v2+ 만 이 임계로 갈린다 | 4 | 신규 |
| 8 | approach_dodge_steer 임계 | abstract_input.rs:61·85 | 2000 / 1500 | 투사체 회피 조향의 근접·조향 임계 | 4 | 신규 |

<details><summary>`callees` 피호출자 30건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev |
|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 3 |
| 2 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 3 |
| 3 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 |
| 4 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 |
| 5 | check | game_core::transfer::SellGuard::check | pub | fn(&game_core::Database, usize, game_core::Position) -> game_core::transfer::SellGuardResult | game-core\src\transfer\roster_blueprint.rs:564 | False | False | 3 |
| 6 | cooldown_reduce | game_core::Entity::cooldown_reduce | pub | fn(&game_core::Entity, bool) -> usize | game-core\src\simulation\entity.rs:2437 | False | False | 3 |
| 7 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 3 |
| 8 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 3 |
| 9 | effect_range_with_radii | game_ai::abstract_input::effect_range_with_radii | in:game_ai::abstract_input | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\abstract_input.rs:190 | False | False | 3 |
| 10 | get_input_target | game_ai::abstract_input::get_input_target | in:game_ai::abstract_input | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity, &game_core::Effect, usize) -> std::option::Option<game_core::InputTarget> | game-ai\src\abstract_input.rs:345 | False | False | 3 |
| 11 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 3 |
| 12 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 3 |
| 13 | is_visible | <game_core::Game as game_core::AbstractGame>::is_visible | pub | fn(&game_core::Game, usize, usize) -> bool | game-core\src\simulation\game.rs:1800 | False | False | 3 |
| 14 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 3 |
| 15 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 |
| 16 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 3 |
| 17 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 3 |
| 18 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 3 |
| 19 | linear_cast_range_with_margin | game_ai::abstract_input::linear_cast_range_with_margin | in:game_ai::abstract_input | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\abstract_input.rs:195 | False | False | 3 |
| 20 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 3 |
| 21 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 3 |
| 22 | range_adjust | <game_core::CombineEffect as game_core::EffectType>::range_adjust | pub | fn(&game_core::CombineEffect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect\type\combine.rs:108 | False | False | 3 |
| 23 | range_adjust | game_core::EffectType::range_adjust | pub | fn(&Self/#0, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect\type.rs:342 | True | True | 3 |
| 24 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 |
| 25 | safe_move_avoiding_enemy_well | game_ai::safe_move_avoiding_enemy_well | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:122 | False | False | 3 |
| 26 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 3 |
| 27 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 3 |
| 28 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 3 |
| 29 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 3 |
</details>

⚠**미매칭 1개**: `is_null`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m07.ll:12776) · **형제 0개** 

**`open` 0건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**

(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)

<details><summary>`closed` 10건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | dyn EffectType vtable 슬롯 +0x118(fn(&self)->bool, 296행)·+0xf8(fn(&self)->(i64,i64), 310행)의 메서드 이름 — `divtable.py EffectType 0x118` 이 '없음: EffectType' 을 반환(1회 시도). Arc<dyn Trait> 라 vtable 전역이 _gaibc 에 없다(SPEC_GUIDE §1 알려진 한계 그대로). 관측 사실만: Position 캐스팅에선 bool 이 true 여야, Direction 캐스팅에선 (i64,i64).0 이 1이어야 사거리 검사 단계로 진행한다. vtable 레이아웃은 0x0 drop/0x8 size/0x10 align/0x18~ 메서드 이므로 각각 32번째·28번째 메서드 슬롯 | 1차 배치A: vt+0x110 = linear_cast_range_margin 을 L196 줄길이 ±0 으로 독립 재확증 |
| 1 | linear_cast_range_with_margin(effect, champ, target) 내부 — 담당 범위 밖(game_ai::abstract_input 의 별도 define). 299/313행 사거리 게이트의 실제 임계는 이 함수가 정한다 | 3차 배치A: history[4](a) 에 이미 답이 있었다(과열림) |
| 2 | get_input_target(...) 내부 — 담당 범위 밖. 실제 InputTarget(대상/방향/좌표) 선정과 None 반환 조건이 여기 있다. 마지막 인자로 champ.cooldown_reduce(true) 결과를 받는다는 것만 확인 | 3차 배치A: abstract_input.rs 345~655 확정, None 반환 정확히 3곳 |
| 3 | Effect::range_adjust(effect, champ, target) 내부 — game_core 쪽, 담당 범위 밖. 총사거리에 더해지는 보정치라는 것만 확인 | 3차 배치D: vtable +0xe8 위임 + 반환 u64 확정 |
| 4 | Entity::can_ult / CastingTarget::check 내부 조건 — game_core 호출로 남아 있어 본문에서 판정식을 볼 수 없다 | 3차 배치A: history[4](c) 에 이미 답이 있었다(과열림) |
| 5 | safe_move_avoiding_enemy_well 이 sret 에 어떤 Input variant 를 채우는지 — 담당 범위 밖. 반환 Option<Input> 32B 를 통째로 쓴다는 사실만 확인 | 3차 배치A: history[0]+[1] 에 이미 답이 있었다(과열림) |
| 6 | param2 rnd(StdRng) 와 param6 positioning_score 는 이 함수 본문에서 한 번도 역참조되지 않는다 — get_input_target 으로 전달만 하므로 여기서의 역할은 '통과' | 1차 배치A: 통과 인자임이 확정(본문 역참조 0). 사실 서술이라 미확정이 아니다 |
| 7 | param1 version 도 본문 분기에 쓰이지 않는다 — get_input_target / safe_move_avoiding_enemy_well 로 전달만. 버전 게이트가 그 하위에 있는지는 미확인 | 3차 배치A: abstract_input.rs 345~655 확정, None 반환 정확히 3곳 |
| 8 | ★해소(6차 배치A) — **sz==0 으로 334행에 도달하는 것은 이 함수 구조상 불가능하다**(상위가 막는 게 아니라 자기가 막는다). 근거 ①`Effect::is_in_range`(_gcbc/g06.ll:51634~51774) = `dist_sq <= total^2` 이고 total 은 비음수 항의 합뿐 — **하한 사거리가 없다** ⟹ 거리 0 이면 항상 in-range. ②`get_input_target` 의 None 반환은 정확히 3곳(m04.ll 블록43 champ null / 블록45 !is_in_range / 블록70 `!visible && !is_nontarget`)이고, 거리 0 에서는 ①때문에 블록45 가 못 뜨며 블록43 은 `ult` 가 이미 champ 를 얻은 뒤라 못 뜬다. ③남은 블록70 은 `!visible` 을 전제하므로, 그 경우 `ult` 는 L328 에서 `!visible` 로 갈라 **L340 `safe_move_avoiding_enemy_well`** 로 나간다 — L329~L335(isqrt·나눗셈)에 못 온다. ④오라클 실행 확증: champ 와 target 좌표를 완전히 같게 두고 casting 4값 × eff_range {0, 200000} = **8/8 전부 `Ult(..)` 반환, Move 0건**(A6_o2_samepos.tsv). ⟹ IR 의 `panic_const_div_by_zero` 는 LLVM 이 남긴 **죽은 가드**이고, 재현 때 이 패닉을 모사할 필요가 없다. ★범위: `ult` 안에서의 판정이다. `get_input_target` 내부가 다른 경로로 None 을 낼 수 있게 바뀌면 다시 봐야 한다. | 본문에 해소 표기가 있다 |
| 9 | Arc 페이로드 오프셋 계산의 `(align-1) & -16` 은 컴파일러가 접은 ABI 계산이라 원식 복원 불가 — 상수 -16 은 판정값이 아니라고 보아 constants 에서 제외했다 | 1차 배치A: ABI 접힘이라 constants 제외 결정 = 판정 완료 |
</details>

<details><summary>`history` 정정 이력 8건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 부가 | Option_니치 | 분기_확정 | Option_인코딩 |
|---|---|---|---|---|---|---|
| 0 | Entity::can_ult 내부 조건 | ★판정식 전량 확정. 본체 _gcbc/g06.ll:81679~81898.  1) cc 에 BlockSkill(4) 있음 → false  2) cc 에 BlockMoveSkill(5) 있음: e = level>4 ? ult_effect(+0x538) : NONE; ok = e.is_none() \|\| !e.ty.can_move()(vt+0x120); if !(ok && ty==Champion) → false.  else: ty != Champion 이면 false  3) cooltime = umax(ult.cooltime(self)*100 / smax(100 + ult_cooldown_mult(+0x46c) + skill_cooldown_mult(+0x400), 1), 1)  4) n = (level>4 ? ult : empty).cooltime_use_count(self); n==0 이면 panic  5) ty.Champion.ult_cooldown(+0xc8) > cooltime - cooltime/n → false  6) cc 에 태그 ∉ {2 Bind, 3 BlockAttack, 4 BlockSkill, 5 BlockMoveSkill} 인 것이 있으면 → false (= Airborne/Stun/ForceMove/Taunt/Fear/Charm/Animation 이 막는다)  7) e2 = level>4 ? ult_effect : NONE; is_none 이면 false  8) return level > 4 | cooltime_use_count 디폴트 = 1 (_gcbc/g02.ll:306878~306880), override 는 GhostSkill2Action 1개뿐 ⟹ 보통 궁은 ult_cooldown == 0 일 때만 사용 가능. EffectType vt+0x120 = can_move, 전 구현 117개 중 true 는 8개(teleport/dash 계열)뿐 ⟹ 2번 규칙은 'BlockMoveSkill CC 중엔 돌진형 궁만 봉인'이다. | Option<Effect> 는 Effect+0x30 casting: CastingType 이 -1 이면 None (range {-1,4}). |  |  |
| 1 | CastingTarget::check 내부 | max_range_nearly_can_use 항목의 14종 표 참조 — 전량 확정. |  |  |  |  |
| 2 | Effect::range_adjust 내부 | max_range_nearly_can_use 항목의 분기표 참조 — 전량 확정. |  |  |  |  |
| 3 | dyn EffectType vtable +0x118(fn->bool) / +0xf8(fn->(i64,i64)) 슬롯 이름 — 1~6차는 'Arc<dyn> 이라 원리적으로 불가'로 포기 | ★확정. **+0x118 = EffectType::on_caster(&self) -> bool**, **+0xf8 = EffectType::linear_move_speed(&self) -> Option<usize>**. 전체 슬롯표는 _shared.EffectType_vtable 참조. |  |  | ★분기 판별자는 CastingTarget 이 아니라 **CastingType** 이다(m04.ll:44051 switch). dienum CastingType: 0 Targeting / 1 Position / 2 Direction / 3 None.   1 Position  → on_caster() == true 여야 사거리 검사로 진행 (m04.ll:44068, abstract_input.rs:296)   2 Direction → linear_move_speed().is_some() 여야 진행 (m04.ll:44085~44088, abstract_input.rs:310) | linear_move_speed 반환 Option<usize> 는 DISCR_EXACT 0=None / 1=Some, tag +0x0 · payload +0x8 (_gcbc/g05.ll:389409~389413). m04.ll:44087~44088 의 `icmp eq %70, 1` 이 곧 is_some(). |
| 4 | linear_cast_range_with_margin / get_input_target / safe_move_avoiding_enemy_well 내부 — 담당 범위 밖 | ★요지 확정(_gaibc/m04.ll). **(a) `linear_cast_range_with_margin`**(abstract_input.rs:191~197, m04.ll:43272~43375):   `radius(e) = radius_mult==0 ? e.radius(+0x680) : e.radius*(radius_mult+100)/100`   `total = effect.range(+0x10) + (casting==Targeting ? radius(champ) : 0) + champ.stat_buff_cached.range(+0x438) + (champ.level−1)*effect.growth_range(+0x18) + Effect::range_adjust(...) + radius(target)`   `반환 = total.saturating_sub(effect.ty.linear_cast_range_margin())`(EffectType vt **+0x110**) **(b) `get_input_target`**(abstract_input.rs:345~, m04.ll:33111~35159): DWARF 지역변수가 계약을 그대로 노출 — `champ, residual, skill_hit_accuracy, caster_skill_hit, timing_error, min_range, max_range, start_timing, lead_timing, flight, best_pos, dx, dy, score, x, y`. Position 경로는 대상 주변 격자를 `position_eval::position_score_at_cell` 로 채점해 best_pos 선정 → `apply_aim_offset_pos`; Direction 경로는 `apply_aim_offset_dir`. ★계측 `entity::nt_trace_record_aim` 을 4곳에서 호출 — rmeta 주석 "조준 시점 스냅샷(get_input_target에서 기록)"의 실체가 이것이다. **(c) `safe_move_avoiding_enemy_well`**(abstract_input.rs:122~145, m04.ll:43378~43964): 목적지가 위험권이 아니면 `Some(approach_dodge_steer(...))`; 현재 위치도 위험권이면 통과; `!is_recent_enemy_well_damage_danger` → `Some((x,y))`; ★`version <= 1` → **None**, `version > 1` → `dist_sq > 4,000,000 ? Some(..) : None`. |  |  |  |  |
| 5 | `get_input_target` 내부(2,049줄) — 미탐색 | ★확정(3차 배치A). abstract_input.rs **345~655**, **None 반환 정확히 3곳**: L346 `?` / L349 `!Effect::is_in_range` / L354 `!is_visible_from && !casting.is_nontarget()`. `CastingType::is_nontarget()`(type.rs:148) = {Position, Direction} 이고 IR 의 `(casting - 1) <u 2` 가 그 접힘이다. 격자 채점 = `position_score_at_cell(..., PositionEvalPurpose::AttackStance)`(태그 12, 니치 밀림), best_pos = `Option<(i64,(i32,i32))>` + `Option::is_none_or`, 인덱스 `clamp(0,29)`, `gen_range(0..=1000)`. 부수: PlayerState +0x180 = `AthleteParameter`(기존 확정 재확인). |  |  |  |  |
| 6 | `entity.rs:1511` 잔차 4자 — 들여쓰기 문제인가 | ★**들여쓰기 문제가 아니다 → 표기 불가(4자)**(3차 배치A). **MIR 칸**으로 `Entity::radius` 나머지 6줄은 전부 ±0 복원됐다(`if mult == 0` / `self.radius * (100 + mult) / 100` 등). L1511 은 `let <4자> = <29자> as usize;` 로 칸까지 고정되는데 의미상 `self.stat_buff_cached.radius_mult` 는 33자다 ⟹ 표기 불가. 남은 미탐색 = 매크로 전개 여부. **동작·sext 는 확정.** |  |  |  |  |
| 7 | `caster_r`(시전자 반경)가 `Effect::range` **안**에서 더해진다 | ★**틀렸다 — 밖에서 더해진다**(5차 배치A, 오라클). `Effect::range(&ef, champ)` 은 casting 4값(Targeting/Position/Direction/None) **전부 200000**(champ.radius=10000) 인데, 같은 실행의 실제 total 은 Targeting **220000** / 나머지 **210000** ⟹ 여분 10000 = `champ.radius` 가 `effect_range_with_radii`(abstract_input.rs:191)에서 **밖에서** 더해진다. IR `m04.ll:44245~44255` 의 caster_r phi 도 `effect+16` 과 나란한 **별개 항**이다. ⚠같은 파일 `constants`(CastingType::Targeting 태그)는 **이미 올바르게** 적혀 있었다 = **자기모순**인데 `specgate G1` 이 못 잡았다(G1 은 `logic` 산문의 **함수 경계**를 안 본다). |  |  |  |  |
</details>

