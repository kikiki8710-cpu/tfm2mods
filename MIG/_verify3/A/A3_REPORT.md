# 3차 반증검증 — 배치 A (인덱스 00~04) 보고

게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
산출물 = `MIG\_verify3\A\` (A3_o03a.rs/.tsv · A3_o03b.rs/.tsv · A3_o04.rs/.tsv · s00~s04.txt)

## 0. 요약

| 항목 | 결과 |
|---|---|
| 실오류 (값·구조·시그니처·오프셋·분기극성) | **4건** (02×2 · 04×1 · shared×1) |
| open[] 오분류(이미 닫힌 것이 열려 있다) | **5건** |
| 닫은 open | **8건** (00:3 · 02:1 · 03:2 · 04:2) |
| ev<=3 표본 재확인 | 5개 함수 sig+src_line + 오프셋 20여 건 → **뒤집힘 0** |
| callees_unmatched 에 섞인 판정 술어 | **0건** |
| 오라클 최초 달성 | 03 `cc_threat=true` · 04 `has_line_defense_threat=true` |

## 1. 실오류 — JSON 경로 패치

### E1. `/specs[2]/consts[4]/src_line`

- 구: `46`
- 신: `50`
- 근거: `rmeta_srcmap game_ai 'old\attack_nexus.rs'` — L46 = 1자(빈 줄). L50 = 58자 → 내용 57 = indent 6 + `SubPlan::AttackNexus(AttackNexusSubPlan::default())`(51자) ±0.

### E2. `/specs[2]/logic` — 최종 분기 arm 반전 (구조 오류)

- 구: `if !has_enemy_twin_tower { return SubPlan::AttackNexus }` 뒤에 `return SubPlan::LineDefense(..)`
- 신:

```
L47  if has_enemy_twin_tower {
L48    SubPlan::LineDefense(LineDefenseSubPlan{line: self.line, minion_action_type: MinionActionType::Push, style: LineStyle::Aggressive})
L49  } else {
L50    SubPlan::AttackNexus(AttackNexusSubPlan::default())
L51  }
```

`return` 없는 꼬리표현식 + else 블록이고, 조건은 **부정형이 아니다**.
근거 = 줄 길이 ±0 4줄(L47=29·L48=137·L49=12·L50=57, indent 4/6 보정 후 정확히 일치) + IR m12.ll:34955~34971 분기 방향(%60 = len==0 → phi 16 AttackNexus / else → 태그 2 LineDefense).
동작은 동일하지만 이 구조 오류가 E1(src_line 46)을 만들었다.

### E3. `/specs[4]/history[4]/now` — `is_near_line` 1번 인자 타입

- 구: `map_regions::is_near_line(ctx.map, m.x(+0x660), m.y(+0x668), line)` (:595)
- 신: `game_core::is_near_line(&GameContext, u64, u64, LineType) -> bool` (game-core\src\simulation\map_regions.rs:139, tcx)
- 근거: IR m04.ll:60492 가 넘기는 `%32` = `data.context` (dereferenceable(64) = GameContext 64B). `ctx.map`(MapDef 28112B)이 아니다.

### E4. `/shared/set_strategy_호출자/슬롯` — 인자 타입명·전달방식 (배치 A 범위 밖이나 04 가 참조)

- 구: 시그니처 = `void(self, team: i64, &TeamStrategy(24B, align 4))`
- 신: `fn(&mut Self, usize, game_core::Strategy)` — `Strategy`(24B·13필드, game-core\src\simulation\strategy.rs:45)를 **값으로** 전달한다. 24B 라 Win64 가 간접전달해 IR 에 포인터로 보이는 것(BRIEF 함정 ③ 그대로). `TeamStrategy`(24B·12필드, strategy.rs:82)는 **다른 타입**이다(`dm_tactic` 이 없다).
- 근거: tcx `AbstractGame::set_strategy` + impl 4개 전부 / 오라클 실컴파일 `g.set_strategy(0, st)`(st: Strategy) 성공.

## 2. open[] 오분류 — 이미 닫힌 것이 열려 있다

| 경로 | 왜 닫힌 것인가 |
|---|---|
| `/specs[0]/open[0]` | `linear_cast_range_with_margin` 전량식이 `/specs[0]/history[4]`(a)에 있다 |
| `/specs[0]/open[3]` | `Entity::can_ult` 8규칙 = `history[0]`, `CastingTarget::check` = `history[1]` 로 전량 확정 |
| `/specs[0]/open[4]` | `safe_move_avoiding_enemy_well` 반환 계약 = `history[4]`(c) |
| `/specs[3]/open[1]` | `Blackboard::is_recent_visible` 인덱스 의미 = `/shared/is_recent_visible/blackboard_인덱스_의미` 에 ★확정 |
| `/specs[4]/open[0]` | 같은 shared 항목 + `/specs[4]/history[1]` 이 "확정됨" 이라고 이미 적어 놓았다 |

과하게 열린 쪽이라 안전한 실수. `closelist.py` 에 근거를 붙여 닫으면 된다.

## 3. 닫은 open — 신규 확정

### 3-1. `/specs[3]/open[4]` — `effect_cc_time` 내부 (닫힘)

```
game_ai::effect_cc_time(version: usize, e: &Effect) -> Option<usize>
  본문 전량 = 단 하나의 vtable 디스패치 (IR m15.ll:25822~25846)
    payload = e.ty.data + ((vt.align - 1) & -16) + 16      // Arc 페이로드
    return  vt[+0x88](payload)                             // = EffectType::expected_cc_time_deep
  ★version(%0) 은 본문에서 한 번도 쓰이지 않는다 — dbg_value 2개뿐, 분기·전달 0.
```

- vt 슬롯 이름은 `/shared/EffectType_vtable` 의 `0x88` = `expected_cc_time_deep`.
- 오라클 실행 확증: 실전 챔피언 60종 × 4슬롯 × version 1~5 = **1,150 호출, version별 결과 불일치 0**.
- ⟹ `/specs[3]/open[6]`("버전 게이트는 그 안쪽에 있을 것으로 추정", ev5)의 **effect_cc_time 분기는 반증**됐다. `check_kill_die_tick`·`is_ignored_well_enemy` 는 여전히 미탐색.

### 3-2. `/specs[3]` — `cc_threat` 진리표 (오라클 최초 점등, 44/44)

`A3_o03b.tsv` (tps=60, 적팀 챔프 교체 + level/쿨 조작):

| 적 챔프(CC 슬롯) | lvl 1 | 2 | 3 | 4 | 5 |
|---|---|---|---|---|---|
| Swordman (CC 없음) | F | F | F | F | F |
| Executioner (skill) | T | T | T | T | T |
| Werewolf (skill2 만) | F | F | **T** | T | T |
| WindMage (ult 만) | F | F | F | F | **T** |

- ⟹ `level > 2`(skill2 개방) · `level > 4`(ult 개방) **실행 확증** → `/specs[3]/consts[2]/ev`·`/specs[3]/consts[3]/ev` 4→2.
- 쿨 경계: 해당 슬롯 쿨 = **60(=tps) → 위협으로 셈 / 61 → 안 셈** ⟹ `cool <= tps` 확정 → `/specs[3]/knobs[1]/ev` 4→2.
- 슬롯 독립성: Werewolf 에 sk2=61 만 걸면 false, sk2=60 이면 true(다른 슬롯 무관) — 3슬롯 OR 구조 확증.

### 3-3. `/specs[3]` — `effect_cc_time` 실전 챔피언 전수 진리표 (신규 재료)

`A3_o03a.tsv` — ChampionInfo 60종(`DataChampionInfo` 는 `Default` 없음) × 4슬롯 = 240:

| 상태 | 수 |
|---|---|
| OK | 230 |
| NO_EFFECT (`Action::effect()` = None) | 5 |
| PANIC (스탯 0 → 0除算) | 5 |
| → `effect_cc_time` = None | 189 |
| → `Some(0)` | 37 |
| → `Some(1)` | 4 |

★`defensive_crisis` 는 `is_some()` 으로만 판정하므로 **`Some(0)` 도 CC 위협으로 센다** ⟹ CC 후보 = **41슬롯**.

### 3-4. `/specs[4]` — `has_line_defense_threat = true` (오라클 최초 점등)

`A3_o04.tsv`. 켜는 조건이 실행으로 갈렸다:

| from_mid | minion_count | 적 미니언 target | threat |
|---|---|---|---|
| 0 | 0 | X | false |
| **-5000** | 0 | X | false |
| 0 | **-3** | X | false |
| **-5000** | 0 | **O** | **true** |
| 0 | **-3** | **O** | **true** |

⟹ `(from_mid < -3000 || minion_count < -2) && ∃ 적팀 미니언(nearest_enemy == Some(tower_id))` — **OR·AND 구조 실행 확증**.

신규 오프셋(한 행 = 한 오프셋):

- `Entity` +0x88 = `ty.Minion.info.nearest_enemy` 판별자 (Option<usize>, 0=None / 1=Some)
- `Entity` +0x90 = `ty.Minion.info.nearest_enemy` 페이로드 (usize = 대상 엔티티 id)

★이름 확정: `/specs[4]/history[4]/now` 의 `<target: Option<usize>>` → **`Minion::nearest_enemy`**(pub, game-core\src\simulation\entity\minion.rs:22). `Minion` 구조체 안 +0x18, Entity 기준 `ty`(+0x68) → variant 페이로드(+0x8) → +0x18/+0x20. 비교는 `Option<usize>::PartialEq::eq` 인라인(L600) — 즉 `info.nearest_enemy == Some(tower_id)`.

지역변수·줄 확정(DWARF): L588 `minion_state` / L589 `wave_is_pushed_in` / L594 클로저 인자 `m` / L600 `info`. L595 = `is_near_line`, L599 = `ty` 판별자 == 1(Minion), L600 = nearest_enemy 비교.

★`Blackboard::minion_state(&self, LineType) -> &BrainMinionParameter` (pub, mir=True, game-core\src\simulation\game\blackboard.rs:378) — 세 필드 직독이 아니라 **메서드 호출**이고 IR 의 `switch line {0/40/80}` 은 그 인라인이다(L379/381/382). `/specs[4]/mem` 의 세 행 note 를 이 이름으로 바꾸면 좋다.

### 3-5. `/specs[4]` — 전략별 인원 구간 20/20 + 반경 경계 실측

threat=true 상태에서:

| morgard_defense | n=0 | 1 | 2 | 3 | 4 |
|---|---|---|---|---|---|
| Gather | F | F | **T** | T | T |
| Battle | F | **T** | T | F | F |

⟹ Gather `near > 1` / Battle `(near - 1) <u 2` = {1,2} — `/specs[4]/knobs[1]`·`knobs[2]` 실행 확증(ev 4→2).

반경 임계 경계(`A3_o04.tsv` bound 행, Gather·적 2명 배치):

| dx | d2 | hld |
|---|---|---|
| 199999 | 39999600001 | true |
| 200000 | 40000000000 | **true** |
| 200001 | 40000400001 | **false** |

⟹ `d2 < 40000000001` 확정(포함 최대 = 200000²) → `/specs[4]/consts[2]/ev`·`/specs[4]/knobs[0]/ev` 4→2.

### 3-6. `/specs[0]/open[1]` — `get_input_target` 요지 확정 (닫힘)

소스 범위 = `game-ai\src\abstract_input.rs` **345~655** (DISubprogram line 345, ret !dbg line 655). IR = m04.ll:33111~35159.

**None 반환은 정확히 3곳** (`store i32 -1, ptr %0` 3개 = m04.ll:33230 / 33237 / 33292):

1. **L346** `let champ = data.cache.player_champion[team][pos]?;` — `?`(Option::branch option.rs:2775 → from_residual 2790)
2. **L349** `!Effect::is_in_range(effect, champ, target)` (호출 !dbg = L348)
3. **L354** — L353 조건 = `!target.is_visible_from(champ) && !effect.casting.is_nontarget()`
   - `is_visible_from` 인라인(entity.rs:1482~1483 + `VisibleState::is_visible` data.rs:122)
   - ★`CastingType::is_nontarget()`(game-core\src\simulation\effect\type.rs:148) = casting ∈ {Position(1), Direction(2)}. IR 의 `(casting - 1) <u 2` 는 **그 술어의 접힘**이다(BRIEF 함정 ①).

그 외 모든 경로는 `apply_aim_offset_pos` / `apply_aim_offset_dir` 가 sret 을 채워 **Some** 을 낸다.

새 사실:

- `PlayerState` +0x180 = `AthleteParameter` (여기서 `skill_hit_accuracy()` / `skill_hit_effective()` 를 호출) — `/specs[3]/knobs` 의 `AthleteParameter+0x98 = PlayerState+0x218` 과 정합(0x218 − 0x98 = 0x180).
- L357 `skill_hit_accuracy` / L358 `caster_skill_hit`(= `skill_hit_effective()`) / L360 `timing_error = 1000 − skill_hit_accuracy` / L361 `min_range` / L362 `max_range`
- L364 `Rng::gen_range(0..=1000)` (RangeInclusive, dbg start=0·end=1000) — 이 함수의 난수 진입점(총 5곳)
- 격자 채점 = `position_eval::position_score_at_cell(version, player, data, positioning_score, x, y, purpose)`; 8번째 인자 `i8 12` = **`PositionEvalPurpose::AttackStance`**(니치 밀림: idx 10 → 메모리 태그 12)
- best_pos 타입 = `Option<(i64,(i32,i32))>`, 갱신 비교는 `Option::is_none_or`(core/option.rs:707) — Direction 경로 L389 / Position 경로 L544
- 격자 인덱스는 `Ord::clamp(0, 29)`(cmp.rs:2016/2025 인라인, L551 등) — **30칸**
- `%8 == 0`(= `champ.cooldown_reduce(true)` 결과) 가드가 L364 의 0除算 방지
- 계측 `nt_trace_on()` / `nt_trace_record_aim()` 4곳 — rmeta 주석 "조준 시점 스냅샷"의 실체
- 마무리 호출: `Entity::move_to` · `Game::adjust_position` · `utils::distance` · `utils::isqrt`

### 3-7. `/specs[0]` — `Entity::radius` 본문 ±0 (MIR 칸 활용)

`mirdump_game_core.txt` 의 span 에 칸이 있어 `mir=True` 함수는 소스가 ±0 으로 복원된다. `game_core::Entity::radius`(entity.rs:1509, 본문 indent 4/6):

```
1511:    let mult = <29자 place-expr> as usize;     ← 잔차 4자 (아래)
1512:    if mult == 0 {
1513:      self.radius
1514:    } else {
1515:      self.radius * (100 + mult) / 100
1516:    }
1517:  }
```

L1512·L1513·L1514·L1515·L1516·L1517 **전부 ±0**(rmeta 줄길이 18/17/12/38/5/3 ↔ MIR 칸 [8,17)/[7,18)/-/[7,39)).
★새 사실: 곱셈항은 **`(100 + mult)`** 로 괄호가 있고 **상수가 앞**이다(MIR `Add(const 100_usize, copy _2)`). `/specs[0]/consts[7]/meaning`·`/specs[0]/knobs[3]` 이 `radius*(radius_mult+100)/100` 로 적어 둔 순서는 반대다(동작 동일 — 오류로 세지 않음).

**L1511 잔차 4자 — 들여쓰기 보정으로는 안 없어진다**:

- 들여쓰기는 tcx `sp` 로 확정(impl 멤버 c=3 → indent 2, fn 본문 indent 4)이고 L1509 시그니처 줄 33자 ±0 으로 검산됐다.
- MIR 칸이 `let`(cols 5-7) · 바인딩(cols 9-12, **4자**) · `=`(col 14) · place-expr(**cols 16-44 = 29자**) · `as`(46-47) · `usize`(49-53) · `;`(54) 를 전부 고정한다. rmeta 줄길이 54자와 완전 정합.
- 그런데 의미상 그 place-expr 은 `self.stat_buff_cached.radius_mult`(**33자**)여야 한다 — Entity 필드 #18 = `stat_buff_cached`, BuffState 필드 #31 = `radius_mult` (둘 다 tcx 확정).
- ⟹ **표기 불가(4자)**. 적용 범위 = rmeta 줄길이 + MIR 칸 + tcx 필드명 3종 결합으로도 못 가름. 남은 미탐색 = 그 줄이 매크로 전개인지(전개면 MIR 칸이 호출부 인자 span 으로 좁아진다). rmeta 에 소스 원문이 없어(`span_to_snippet` = SourceNotAvailable) 문자열 자체는 **재료 부재**.
- ⚠동작은 완전히 고정돼 있다: `(*self).stat_buff_cached.radius_mult as usize`(i32 → usize **sext**, MIR `IntToInt`) ⟹ `/specs[0]/mem` 의 `Entity` +0x470 항목(ev 3)은 그대로 유효.

### 3-8. `/specs[2]` — sub_plan 본문 ±0 6줄 신규 (`/specs[2]/history[4]` 확장)

```
36:    let champ = data.cache.player_champion[player.info.team][player.info.position.as_index()].unwrap();   (99자, indent 4) ±0
37:    let (lx, ly, rx, ry) = data.context.map.fountain(player.info.team);                                  (67자) ±0
38:    let is_in_heal_area = champ.x >= lx && champ.x <= rx && champ.y >= ly && champ.y <= ry;               (87자) ±0 (재확증)
41:    if is_in_heal_area && champ.hp < champ.stat_cached.hp {   ← 잔차 1자 (아래)
42:      return SubPlan::Recall(RecallSubPlan::default());                                                  (49자, indent 6) ±0
45:    let has_enemy_twin_tower = !data.cache.twin_towers[1 - player.info.team].is_empty();                  (84자) ±0
47:    if has_enemy_twin_tower {                                                                             (25자) ±0
49:    } else {                                                                                              ( 8자) ±0
50:      SubPlan::AttackNexus(AttackNexusSubPlan::default())                                                 (51자) ±0
```

★`team` / `pos` **지역변수는 없다** — DWARF 지역변수는 `champ`·`lx`·`ly`·`rx`·`ry`·`is_in_heal_area`·`has_enemy_twin_tower` 7개뿐이고, 팀·포지션은 매번 `player.info.team` / `player.info.position.as_index()` 로 인라인 표기돼 있다(L36·L37·L45 세 줄이 그것으로 ±0 이 된다). `/specs[2]/logic` 의 `team =` / `pos =` 두 줄은 의사코드 표기다.
★L48 구조체 리터럴이 **`LineDefenseSubPlan{...}`(중괄호 앞뒤 공백 없음)** 이라야 137자 ±0 이 된다 ⟹ 이 파일은 rustfmt 기본값으로 포맷되지 않았다(L31 시그니처 167자·L48 137자).

**L41 잔차 1자**:

- 들여쓰기 보정은 이미 맞다(같은 함수 L36·37·38·42·45·47·49·50 여덟 줄이 indent 4/6 로 ±0).
- 측정 = 54자(indent 4 제외). 의미상 후보 `if is_in_heal_area && champ.hp < champ.stat_cached.hp {` = 55자.
- 고정된 조각: `is_in_heal_area`(15자, L38 ±0 으로 확정) · `champ.hp`·`champ.stat_cached.hp`(tcx 필드명; IR !dbg 에 inlinedAt 이 없어 **접근자 인라인이 아닌 직접 필드 접근** 확정) · ` {`(L47 이 ` {` 임을 25자로 증명).
- ⟹ **표기 불가(1자)**. 적용 범위 = rmeta 줄길이 + IR(m12.ll)/DWARF + tcx. `mir=False` 라 MIR 칸이 없다(= 02 에 남은 유일한 미탐색 경로가 그것).
- ★신규 정황: 이 58자 줄은 `old\passive_line.rs:1073` · `old\single_line.rs:286` 에 **바이트까지 같은 줄**로 또 있다(세 파일 모두 `fountain` 줄 72자 · `is_in_heal_area` 줄 92자 · 그 다음 줄 59자) ⟹ 복사된 공용 이디엄이며 attack_nexus 국소 오타가 아니다.

### 3-9. `/specs[1]` — L524·L526~L548 ±0 (표기 확인만, 오류 0)

`calculate_jungle_action_score` 는 자유 함수라 본문 indent 2 다.

```
524:  let hp = t.hp as i64;                                    (21자) ±0  ← ★`as i64` 가 소스에 명시돼 있다
526:  let coef = if t.ty.is_nexus() {                          (31자) ±0
527:    200                                                    ( 5자) ±0
528:  } else if t.ty.is_tower() {                              (27자) ±0
529:    80                                                     ( 4자) ±0
530:  } else if t.ty.is_jungle(0) || t.ty.is_jungle(1) {       (50자) ±0 (재확증)
531:    if hp <= value {                                       (16자) ±0
532:      40                                                   ( 2자) ±0
533:    } else {                                               ( 8자) ±0
534:      20                                                   ( 2자) ±0
535:    }                                                      ( 1자) ±0
536:  } else if t.ty.is_any_type_minion() {                    (37자) ±0 (재확증)
537:    -10   /  538:  } else {  /  539:    0  /  540:  };
542:  let based = if t.ty.is_jungle(0) || t.ty.is_jungle(1) {  (55자) ±0 (재확증)
548:  return min(coef, coef * value / hp) + based;             (44자) ±0
```

- 지시받은 대로 L531 은 다시 파지 않았다. **범위 표기 확인 결과 적절하다**: L531 = 20자(=indent 4 + 16)이므로 `if hp > value {`(15자)는 배제되고 arm 반전이 확정, `hp <= value` vs `value >= hp`(둘 다 11자)는 **표기 불가**.
- L548 이 `min(coef, ..)` 자유함수 형태(44자)로 ±0 ⟹ `/specs[1]/logic` 의 반환식 표기가 맞다(`coef.min(..)` 메서드형은 43자로 안 맞는다).

## 4. ev<=3 표본 재확인 — 뒤집힘 0

| 항목 | 검증 | 결과 |
|---|---|---|
| 5개 함수 `sig.tcx` + `src_line` | `spec3lib.py fn` | 5/5 ±0 (280 / 520 / 31 / 17 / 543) |
| `PlayerState` +0x930 `info.team` | IR gep 2352 | OK |
| `PlayerState` +0x9c0 `info.position` | IR gep 2496 | OK |
| `Entity` +0x628 `stat_cached.hp` | tcxdict + IR gep 1576 | OK |
| `Entity` +0x670 `hp` | tcxdict + IR gep 1648 | OK |
| `Entity` +0x660 `x` | IR gep 1632 | OK |
| `Entity` +0x668 `y` | IR gep 1640 | OK |
| `Entity` +0x68 `ty` 판별자 | IR gep 104, `== 1` Minion | OK |
| `Entity` +0x470 `stat_buff_cached.radius_mult` | tcxdict + MIR IntToInt | OK (sext 확정) |
| `AbstractGameWithCache` +0x1e0 `player_champion` | IR gep 480 | OK |
| `AbstractGameWithCache` +0x148 `twin_towers[0].len` | IR gep 328 + stride 32 | OK |
| `MapDef` +0x6d70 `fountains` | IR gep 28016 | OK |
| `GameContext` +0x20 `map` | IR gep 32 | OK |
| `GameSetting` +0x12f8 `tick_per_second` | tcxdict | OK |
| `Blackboard` +0x0 `top_minion_state` | tcxdict | OK |
| `Blackboard` +0x28 `mid_minion_state` | tcxdict | OK |
| `Blackboard` +0x50 `bottom_minion_state` | tcxdict | OK |
| `BrainMinionParameter` +0x10 `from_mid` | tcxdict | OK |
| `BrainMinionParameter` +0x20 `minion_count` | tcxdict | OK |
| `Strategy` +0xe `morgard_defense` | tcxdict | OK |
| `Effect` +0x10 `range` | tcxdict | OK |
| `Effect` +0x18 `growth_range` | tcxdict | OK |
| `Effect` +0x28 `target` | tcxdict | OK |
| `Effect` +0x30 `casting` | tcxdict | OK |
| `SubPlan` 태그 5 / 2 / 16 | `tcxdict --enum SubPlan` | OK (니치 start 2) |
| `LineDefenseSubPlan` +0x0 `style` | tcxdict + IR store i8 0 | OK |
| `LineDefenseSubPlan` +0x1 `line` | tcxdict + IR store | OK |
| `LineDefenseSubPlan` +0x2 `minion_action_type` | tcxdict + IR store i8 2 | OK |
| `EntityType::is_jungle(&self, usize)` | tcx sig | OK (1차 유령 노브 정정 유지) |
| `action_score.rs` L531 = 20자 | rmeta_srcmap | OK |

⟹ **ev<=3 사고 0건.**

## 5. callees_unmatched — 판정 술어 누락 0건

| spec | names | 판정 |
|---|---|---|
| 00 | `is_null` | `logic` 산문의 널검사 표현(실체는 `Option::branch` 의 `icmp eq ptr, null`). 게임 술어 아님 |
| 01 | `llvm.smin.i64`, `vtable` | intrinsic + 산문 잡음 |
| 02 | (없음) | — |
| 03 | `player_champion`, `pool`, `setting` | 전부 **필드명**(AbstractGameWithCache / GameContext). 오라클에서 pub 필드로 직접 접근돼 실증됨 |
| 04 | `wrapping_sub` | std |

제안(오류 아님): 04 는 `has_line_defense_threat` 본문을 `history` 에 실어 놓았는데 그 본문 술어 `Blackboard::minion_state` · `game_core::is_near_line` · `AbstractGameWithCache::minions` 는 `callees` 에 없다. 자동생성 범위가 "본 함수 `calls_raw`" 뿐이라 설계상 정상이지만, 본문을 실은 피호출자의 술어도 넣으면 E3 같은 인자 오류가 기계로 잡힌다.

## 6. 오라클 레시피 — 신규 함정 3개 (`/shared/오라클_레시피_함정` 추가 후보)

1. **`GameSetting::default().minion_wave_setting` 은 전 필드 0** ⟹ `run_tick` 을 1,200틱 돌려도 **미니언이 하나도 안 생긴다**. 손으로 넣어야 한다(실측: `start_tick=1 / tick_per_wave=600 / melee_count=3 / range_count=3 / tick_per_spawn=5` → **6틱**에 팀당 3+3 = 6마리). `tick_per_second = 0` 과 같은 부류다.
2. **`world.strategy[team]` 직접 대입은 `PlayerState::strategy()` 결과에 반영되지 않는다.** `AbstractGame::set_strategy(team, Strategy)`(pub) 를 써야 한다 — 실측 20/20 반영, 직접 대입은 0/20.
3. `start_game()` 만으로 `world.tower_ids.len() == 16`(팀당 8) 재확증 — BRIEF §3① 의 두 줄(`init_tower`/`init_nexus`)을 지운 상태에서 실행한 결과다. 이번 배치 프로브 3개 전부 그 두 줄이 없다.

덤으로: **`ChampionInfo` 구현 61종이 전부 pub이고 `Action::effect()` 도 pub** 이라 **Game 을 만들지 않고** 실전 챔피언 이펙트 진리표를 뽑을 수 있다(A3_o03a). BRIEF 의 "실전 챔피언 데이터 로딩은 미탐색" 은 부분 해소. 단 ①`::default()` 는 스탯 0 이라 `Effect.range` 가 0 ②`DataChampionInfo` 만 `Default` 없음 ③슬롯 5개는 스탯 0 除算으로 패닉하니 `catch_unwind` 로 감싸야 한다.

## 7. 방법론 소득

- ★**`_tcx\mirdump_game_core.txt` 의 span 칸으로 `mir=True` 함수는 소스 ±0 복원이 된다.** `Entity::radius` 는 이 한 방으로 6/7 줄이 닫혔고, 남은 1줄도 **칸 단위로 좌표가 고정**돼 잔차의 위치가 4자로 국소화됐다. 줄 길이 산술만 쓰던 이전 방식보다 강하다 — `mir=True` 인 피호출자는 먼저 여기를 볼 것.
- ★**잔차를 "들여쓰기 탓"으로 넘기지 마라.** 02 L41·00 L1511 두 잔차 모두 들여쓰기는 이미 맞았고(같은 함수의 다른 줄 8개·6개가 ±0), 잔차는 **식 표기** 쪽이었다. 들여쓰기 보정은 2차에서 이미 끝난 문제다.
- ★**`open` 에 `history` 가 이미 답한 항목이 5건 섞여 있었다.** `history` 는 v2 `resolved[]` 원문이라 `open` 과 기계 대조가 안 된다 — `specgate.py` 에 "open[].q 와 history[].was 의 유사도" 게이트를 넣으면 잡힌다(제안).

## 8. 제출 게이트 실행 결과

- `python -X utf8 specgate.py` → **총 0건** (G1~G4 전부 0)
- `python -X utf8 tcxaudit.py --prose _verify3\A\A3_REPORT.md` → 총 681건 · **오귀속=0 · 밀림=0** · 부분일치=1 · 확인불가=18

★기준선 대조(같은 `--prose` 모드로, 존재하지 않는 파일을 넘겨 측정): 총 679건 · 오귀속=0 · 밀림=0 · **부분일치=1** · 확인불가=18.
⟹ 이 문서가 더한 것은 **행 2건(전부 OK)** 이고 실패 버킷(오귀속·밀림·부분일치·확인불가)에는 **0건**을 더했다.

⚠BRIEF §5 의 "현재 오귀속 0 · 밀림 0" 은 맞지만 **`--prose` 모드에는 이미 `부분일치=1` 이 있다** — 출처는 `_spec\specs20.json` 의 `base=cache` `offset=0x8` 산문 행(v2 파일)이다. 다음 배치가 자기 문서 탓으로 오해하지 않도록 기준선을 이 값으로 적어 두는 편이 좋다. 또 `--prose` 에 없는 파일을 넘기면 오류 없이 `[skip]` 만 찍히므로, 경로를 틀리면 "내 문서는 깨끗하다"로 잘못 읽힌다.
