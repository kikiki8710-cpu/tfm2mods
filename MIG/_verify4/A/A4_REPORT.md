# 4차 반증검증 — 배치 A (인덱스 00~04) 보고

게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
산출물 = `MIG\_verify4\A\` (`A4_o01.rs` · `A4_o01b.rs`/`.tsv` · `A4_o01c.rs`/`.tsv` · `edt.ll` · 이 문서)

## 0. 요약

| 항목 | 결과 |
|---|---|
| **실오류**(값·구조·시그니처·오프셋·분기극성) | **0건** |
| `open[]` 오분류(이미 닫힌 것 / 질문이 아닌 것) | **11건** |
| 새로 닫은 `open` (실질 규명) | **6건** (01:3 · 03:1 · 04:2) |
| `ev` 강등 제안 | **1건** (`/specs[1]/open[0]` ev5 → **ev2**, 오라클 26/26) |
| `ev<=3` 표본 재확인 | 오프셋 20건 + 함수 5종 → **뒤집힘 0** |
| G5/G6 사각지대 `logic`↔정본 불일치 | **0건**(점검 내역 §5) |
| `callees_unmatched` 판정 술어 누락 | **0건**(§6, 단 3차 제안 재확인) |
| **브리핑 결함** | **1건**(§7 — §5 기준선 수치가 실측과 다르다) |

★이 라운드의 최대 성과는 **`_spec` 이 `open`(ev5)으로 남겨둔 마지막 미탐색 1건을 오라클 실행으로 닫은 것**이다.

---

## 1. 실오류 — 0건

값·구조·시그니처·오프셋·분기극성에서 정정할 것을 찾지 못했다. 억지 정정을 만들지 않았다.
특히 다음 3건은 「오류처럼 보였으나 아니었다」로 기록한다(다음 라운드가 다시 파지 않도록):

| 의심한 것 | 왜 오류가 아닌가 |
|---|---|
| `/specs[1]/logic` 이 `value = effect.expected_damage_target(...)` 에 `// i64` 라 적었는데 `sig`·`callees` 는 `-> usize` | 소스 **L523 = 76(개행 포함) → 내용 75자** = `  let value = effect.expected_damage_target(data.context, champ, t) as i64;` **±0**. `as i64` 가 소스에 명시돼 있어 둘 다 맞다 |
| `/specs[1]/logic` 이 `team =` / `pos =` / `champ =` 3줄로 쓴 것 | L522 내용 101자 = `  let champ = data.cache.player_champion[player.info.team][player.info.position.as_index()].unwrap();` **±0**. 02 와 같은 공용 이디엄이고 `logic_note` 가 의사코드임을 이미 선언 |
| `/specs[3]/logic` 의 모듈 경로(`fight_check::effect_cc_time` · `fight_model::is_ignored_well_enemy` · `plan_legacy::old::battle::max_range`) | tcx 전수 대조 **4/4 일치**(`fight_check.rs:379` / `plan_legacy\old\fight_model.rs:754` / `plan_legacy\old\battle.rs:2234` / `path_finder.rs:1032`) |

---

## 2. ★새로 닫은 `open` — 6건

### 2-1. `/specs[1]/open[0]` (ev5, 이 배치의 유일한 ev5) — **닫힘 + ev5→2**

**질문**: `Effect::expected_damage_target` 내부 미확인, `value` 의 단위(절대 피해량인지 스케일된 값인지) 불명.

**본체 = `_gcbc/g06.ll:52355~52846`**(`game-core\src\simulation\effect.rs:91`, `pub`, mir=False).
DWARF 지역변수까지 붙어 복원됐다(`ad` L93 · `ap` L93 · `thr` L100 · `ad` 재대입 L101):

```rust
// effect.rs:91   pub fn expected_damage_target(&self, context: &GameContext,
//                    caster: &dyn AbstractEntity, target: &Entity) -> usize
let _t = ProfTimer::new(53);                                       // L92, prof::ENABLED 게이트
let (ad, ap) = if target.ty.is_structure() {                       // L93  (entity.rs:1391 인라인)
        self.ty.expected_damage_structure(context, caster)         //      vt+0x30 → Option<(usize,usize)>
            .unwrap_or_else(|| self.ty.expected_damage(context, caster))   // vt+0x28
    } else {
        self.ty.expected_damage(context, caster)                   //      vt+0x28
    };
let thr = self.ty.expected_target_hp_ratio(context, caster);       // L100, vt+0x38 → usize
let ad  = if thr != 0 { ad + target.stat_cached.hp * thr / 100 } else { ad };   // L101
if ad | ap == 0 { return 0 }                                       // (`ad==0 && ap==0` 의 접힘)
  get_damage(caster, target, ad, self.attack_type, DamageType::AD)
+ get_damage(caster, target, ap, self.attack_type, DamageType::AP)
```

`get_damage` 는 **`game_core::utils::get_damage`(utils.rs:97, `pub`, mir=True)** 이고 MIR 전문이 있어 ±0 복원된다:

```rust
// utils.rs:97  pub fn get_damage<C: AbstractEntity, T: AbstractEntity>(
//                caster: &C, target: &T, mut damage: usize, attack_type: AttackType, damage_type: DamageType) -> usize
match damage_type {                                                       // L98
  DamageType::AD => {
    let target_stat = target.stat_ref();                                  // L100
    let caster_stat = caster.stat_ref();                                   // L101
    let mut defence = target_stat.defence;                                 // L102
    let buff_state  = caster.buff_state();                                 // L103
    if attack_type == AttackType::BaseAttack {                             // L106 (cols 10-47 = 37자)
      if buff_state.base_attack_enemy_max_hp_damage != 0 {                 // L107
        damage += target_stat.hp * buff_state.base_attack_enemy_max_hp_damage / 100;   // L108
      }
    } else if !matches!(attack_type, AttackType::Dot | AttackType::DotIgnoreShield) {  // L110
      if buff_state.skill_enemy_max_hp_damage != 0 {                       // L111
        damage += target_stat.hp * buff_state.skill_enemy_max_hp_damage / 100;         // L112
      }
    }
    if attack_type == AttackType::Dot || attack_type == AttackType::DotIgnoreShield {  // L116 (30자 || 42자)
      if buff_state.dot_amplify != 0 {                                     // L117
        damage = damage * (100 + buff_state.dot_amplify) / 100;            // L118
      }
    } else if buff_state.self_max_hp_damage != 0 {                          // L120
      damage += caster_stat.hp * buff_state.self_max_hp_damage / 100;       // L121
    }
    if buff_state.defence_penetration != 0 {                               // L124
      defence = defence * 100usize.saturating_sub(buff_state.defence_penetration) / 100;  // L125
    }
    (damage * 100 / (100 + defence)).max(1)                                // L129
  }
  DamageType::AP => { /* L132~L160, 완전 대칭. defence→magic_resistance,
                         defence_penetration→magic_resistance_penetration */ }
  DamageType::Fixed => damage                                              // L163 (bb2: _0 = copy _3)
}
```

**⟹ 반환값은 HP 와 같은 축의 절대 피해량**(방어·마저 경감까지 끝낸 값, 각 성분 최소 1).
`hp * thr / 100` 로 %성분까지 절대값으로 환산하므로 스케일된 값이 아니다.
`/specs[1]/logic` 의 `min(coef, coef*value/hp)` 는 그래서 그대로 「최대체력 대비 비율」이 된다.

**★오라클 실행 확증 — 26/26 MATCH, mismatch 0** (`A4_o01c.rs`/`.tsv`)

`AttackEffect`(72B, **전 필드 pub**)를 손으로 만들어 꽂고, IR 에서 읽은 예측식과 칸 단위로 대조했다.

| 스윕 | 케이스 | 결과 |
|---|---|---|
| (A) `defence` 0/25/50/100/200/400/900/1900 | dmg=500 | 8/8 = `500*100/(100+d) + 1` |
| (B) `attack_ratio × caster_stat.attack` | (100,300)(50,1000)(200,777)(0,1000) | 4/4 |
| (C) `hp_ratio × caster_stat.hp` | (10,3000)(3,1234)(0,3000) | 3/3 |
| (D) `target_hp_ratio × target.stat_cached.hp` | (10,2000)(7,3333)(25,1000)(0,2000) | 4/4 |
| (E) `magic_resistance` 0/100/500 (ap=0) | — | 3/3, 값 불변 = **ap 성분이 항상 정확히 +1** |
| (F) `ad\|ap==0` 조기반환 | dmg=ar=hr=thr=0 | 1/1 = **0** |
| (G) 구조물(타워) 대상 | def 0/100 | 2/2, `is_structure()`=**true** 확인 |
| (H) 구조물 + thr | thr=50 | 1/1 |

부수 확정: `AttackEffect::expected_damage`(`_gcbc/g13.ll:144578`) =
`damage + attack_ratio*caster_stat.attack/100 + hp_ratio*caster_stat.hp/100` 이고 **ap 는 항상 0**.
`AttackEffect::expected_target_hp_ratio` = `self.target_hp_ratio` 한 줄(`g13.ll` 바로 아래).
`AttackEffect` 에는 `expected_damage_structure` 가 **None** 이라 (G)(H) 가 일반 경로와 같은 값을 낸다.

⚠**오라클 함정 1건 추가**(`/shared/오라클_레시피_함정` 후보): **실전 `ChampionInfo::default()` 는 스탯만 0 이 아니라 액션 파라미터까지 0 이다.** 챔피언 6종 × 이펙트를 `expected_damage_target` 에 넣으면 **26/26 이 0**(`A4_o01b.tsv`) — `ad|ap==0` 조기반환에 걸린다. **`AttackEffect` 를 직접 구성**해야 판별력이 생긴다(`Effect` 도 전 필드 pub: `ty`/`range`/`growth_range`/`start_timing`/`target`/`attack_type`/`casting`).

### 2-2. `/specs[1]/open[1]` — **닫힘**

**질문**: "effect(56B) 내부 필드를 이 함수는 한 번도 읽지 않아 어떤 이펙트인지 여기서는 알 수 없다. … 이펙트 종류별 차등은 expected_damage_target 안에 있다."

**확정**: 차등의 소재가 정확히 둘이다 —
① `Effect+0x0 ty : Arc<dyn EffectType>` 의 **vtable 슬롯 3개**(`+0x28 expected_damage` / `+0x30 expected_damage_structure` / `+0x38 expected_target_hp_ratio`, `/shared/EffectType_vtable` 과 정합)
② `Effect+0x2c attack_type : AttackType` — `get_damage` 안에서 `BaseAttack(0)` / `Dot(2)·DotIgnoreShield(3)` / 그 외로 3분기해 **어떤 버프 항이 붙는지**를 가른다.
`Effect` 의 나머지 필드(`range`/`growth_range`/`start_timing`/`target`/`casting`)는 이 경로에서 **읽히지 않는다**(IR gep 0건).

### 2-3. `/specs[1]/open[3]` — **닫힘**

**질문**: "호출부를 안 봐서 어떤 액션 후보 루프에서 `t` 가 공급되는지는 미확인."

**확정**: 유일 호출자 = **`game_ai::plan_legacy::sub_plan::jungle::JungleSubPlan::score`**(`jungle.rs:152`).
`callers` 의 3개 사이트는 **같은 함수 안의 세 줄** = `jungle.rs` **L159 / L171 / L183**(`_gaibc/m02.ll:40037 / 40081 / 40124`, dbg `!46605`/`!46610`/`!46615`).
`t` 는 후보 루프가 아니라 **goal 의 타깃 id 를 엔티티로 바꾼 것**이다:
`t = (*cache.game).<vt+0x1f0 get_entity_by_id>(goal+0x8)` (m02.ll:39986/40029/40074 세 곳 동형, null 이면 그 arm 은 점수 계산을 건너뛴다).
점수 합성도 확정: `score = interaction_score(..) + calculate_jungle_action_score(..)` 이고,
`t.ty == Jungle(4) && t.ty.Jungle.info.camp_type.0 < 2` 일 때만 `max(합, 1)` 로 **하한 1** 이 걸린다(m02.ll:40040~40047).
`effect` 는 `champ+0x4b0`(=1168), `_action` 은 `champ+0x570`(=1392)에서 나온다.

### 2-4. `/specs[3]/open[2]` — **닫힘(실질) + 표기만 불가**

**질문**: "`near_enemies.len()==0` 일 때 `(false,false)` 를 즉시 반환하는 블록(%43)의 `!dbg` 는 line 30 인데, 소스가 **명시적 is_empty 검사**인지 아니면 LLVM 이 두 계산을 false 로 접은 결과인지 구분 못 했다."

**확정 = 소스의 명시적 `is_empty()` 다.** 근거 2개:
1. **`inlinedAt` 이 직접 증거다** — `_gaibc/m10.ll` 의 `!40753 = !DILocation(line: 1635, scope: !40751, inlinedAt: !40754)` 이고 `!40751` 은 `bumpalo::collections::vec::Vec::is_empty`(vec.rs:1635), `!40754 = !DILocation(line: 30, …)`. 즉 **`is_empty` 가 buff_value.rs:30 에 인라인돼 있다.**
2. **접힘으로는 설명이 안 된다** — `%43`(len==0) 경로는 `player_by_champion_id`(L32)와 **`check_kill_die_tick`(L33, `invoke`, `&mut StdRng`·`&mut DebugFrameData`)** 을 건너뛴다. 언와인드 가능 + `&mut` 인자를 가진 호출은 LLVM 이 제거할 수 없다 ⟹ 분기는 소스에서 온 것이다.
분기 결과도 확정: 종단 `phi` 가 `%43` 에서 **두 값 모두 `false`**(`%150`/`%151`, m10.ll:34273~34274).

**남은 것 = 표기뿐**: L30 은 개행 포함 50 → 내용 49 = indent 2 + **47자**. 최유력 후보 `let mut die_imminent = !near_enemies.is_empty();` 는 **48자**로 **잔차 1자**.
⚠**적용 범위** = rmeta 줄길이 + IR/DWARF(`inlinedAt`·`DILocalVariable`) + tcx 결합으로도 못 가름. `defensive_crisis` 는 `mir=False`(tcx)라 MIR 칸이 없다 — 02 `attack_nexus.rs:41`, 00 `entity.rs:1511` 과 **같은 부류의 잔차**다.
들여쓰기 보정은 이미 맞다: 같은 함수 L19(내용 49 = `  let tps = data.context.setting.tick_per_second;`) · L52(내용 45 = `  DefensiveCrisis { die_imminent, cc_threat }`) 두 줄이 indent 2 로 **±0**.
부수 확정(DWARF 스코프): `tps` L19 / `near_enemies` L21 / `die_imminent` L30(bool, 가변 — L35 에서 재대입) / `tp` L32 / `die` L33 / `cc_threat` L41 / 꼬리표현식 L52 / `ret` L53.

### 2-5. `/specs[4]/open[0]` — **닫힘(구현체 전수) · 런타임 선택은 범위 한정**

**질문**: "vtable 슬롯 `0x40` → `get_game_mode` … 런타임에 어느 구현체(ExpectedGame 외)가 꽂히는지는 확인 불가."

**확정**: `AbstractGame::get_game_mode` 구현은 **정확히 4개**(tcx 전수) —

| 구현체 | 위치 | 반환 |
|---|---|---|
| `Game` | game.rs:1707 (mir=True) | `GameMode::Moba(&self.2)` |
| `SingleLaneGame` | game.rs:4248 (mir=True) | `GameMode::SingleLane(&self.1)` |
| `DeathMatchGame` | game.rs:5052 (mir=True) | `GameMode::DeathMatch(&self.1)` |
| `ExpectedGame<'a>` | expected_game.rs:46 (mir=False) | **순수 위임** — `self.game.get_game_mode()` (`_gcbc/g08.ll:200245`: `self+0x0` 데이터 · `self+0x8` vtable · 슬롯 `+0x40` 재호출, 그 외 명령 0개) |

⟹ **`ExpectedGame` 은 종단 구현체가 아니다.** 실효 종단은 3개이고 **`Game` 만 `Moba`** 다.
`as_moba()`(simulation.rs:120 → game.rs:231~234)의 MIR: `switchInt [0→Some, 1→None(L233), 2→None(L234), otherwise→unreachable]`.
⟹ `handle_line_defense` 의 `.as_moba().unwrap()` 은 **`SingleLaneGame`/`DeathMatchGame` 에서 패닉**한다.
⚠**적용 범위**: L544 `line_exists` 는 `GameContext+0x38 tutorial` 만 보고 게임모드를 보지 않으며(`/shared/rule_scope_게이트`), L549 는 `!morgard_exists(..) || <as_moba 항>` 이라 일반 경기(tutorial=None)에서는 **둘째 항이 반드시 평가된다** ⟹ 정적으로는 패닉 경로가 열려 있다. 「legacy 플랜 계층이 그 두 모드에서 아예 안 돌아간다」는 이 배치 범위 밖(미탐색)이고, 2·3차 오라클이 `Game` 으로 통과한 것은 **`Game` 이 꽂혔음의 실행 확증**이다.

### 2-6. `/specs[4]/open[3]` — **닫힘**

**질문**: "반환 true 의 최종 의미는 호출측에서 플랜 태그를 store 하는 것으로 확인했으나, 그 핸들러 `+1055`/`+1056` 필드가 무엇인지는 담당 범위 밖이라 안 봤다."

**확정**: 그 「핸들러」는 **`TeamPlan`(1064B)** 이다 — 호출부 `_gaibc/m09.ll:9043` =
`game_ai::plan_legacy::team_plan::objective_handlers::TeamPlan::handle_none_or_gank_objective`
의 `%0`(`dereferenceable(1064)`).

- `TeamPlan+0x41f`(=1055) = `objective : Option<MainObjective>` 의 **판별자**(니치, tcx)
- `TeamPlan+0x420`(=1056) = 같은 필드의 **페이로드 태그**

`MainObjective`(3B, 12 variant, team_plan.rs:1030, 태그=선언순 0~11)와 맞추면 IR 의 store 두 쌍이 이렇게 읽힌다(m09.ll:9195~9214):

| IR | 의미 |
|---|---|
| `store i8 2, ptr %0+1055` | `objective = Some(MainObjective::Defense)` — `line_exists` 또는 `handle_line_defense` 가 false 일 때 |
| `store i8 3, ptr %0+1055` + `store i8 1, ptr %0+1056` | `objective = Some(MainObjective::DefenseLine(LineType::Mid))` — **true 일 때** |

⟹ **`handle_line_defense == true` 의 최종 의미 = 팀 목표를 「그 라인 방어(DefenseLine(line))」로 확정하는 것**이고, false 면 라인 지정 없는 `Defense` 로 떨어진다.
`MainObjective::DefenseLine` 의 페이로드는 `enum+0x1 : LineType` 1필드(tcx) — `+0x420` 이 정확히 그 자리다.
`/shared/objective_매핑표` 가 적어 둔 `TeamPlan+0x41f objective`(= `LegacyPlanHandler+0x517`)와 **같은 필드**다.

---

## 3. `open[]` 오분류 — 11건 (3차가 5건 잡은 것과 같은 부류)

닫으면 되고 값·구조는 건드릴 것이 없다. 두 갈래로 나뉜다.

### (a) 이미 다른 필드가 답을 갖고 있다 — 3건

| 경로 | 답이 있는 곳 |
|---|---|
| `/specs[3]/open[0]` | `/shared/is_recent_visible/blackboard_인덱스_의미` = ★확정("`Blackboard[T].last_visible[pos]` = 팀 (1−T) 가 팀 T 의 pos 챔프를 마지막으로 본 틱"). `blackboard[1−my_team]` 을 대입하면 **「우리 팀이 적 챔프를 본 판」** = open 이 묻는 두 후보 중 「적팀 엔티티에 대한 관측판」. ⚠**3차 §2 가 이미 지적했는데(`/specs[3]/open[1]`) 반영이 안 됐다** — 같은 shared 항목으로 닫힌 `/specs[4]/closed` 쪽만 적용됐다 |
| `/specs[3]/open[3]` | `/specs[3]/history[2]` = "★확정. `die_tick = ((revive_hp + focus.hp) ⊖sat enemy_nuke) * 60 / max(enemy_dps,1)`, 본체 m15.ll:31120~33053". open 은 아직 "추정"이라고 적혀 있다 |
| `/specs[4]/open[1]` | `/specs[4]/history[0]` = "★완전히 동일. 확인 항목 5개가 바이트 단위로 같다. m04.ll:58373~58403 ↔ m12.ll:30621~30651" |

### (b) 질문이 아니라 **확정 서술**이 `open[]` 에 주차돼 있다 — 8건

| 경로 | 본문이 스스로 하는 말 |
|---|---|
| `/specs[1]/open[2]` | "…쓰기 없음이 **확정**이다. '조사 못 함'이 아니다" |
| `/specs[2]/open[3]` | "…**DWARF !14558 로 확정한 값**" (tcx 재확인: `OperationData+0x0 = cache`, 24B 3필드 ✓) |
| `/specs[3]/open[1]` | "…constants 에는 실제 존재하는 1(shift 량)로 등록하고 meaning 에 명시했다" = 설계 결정 기록 |
| `/specs[3]/open[4]` | "…**미조사가 아니라 확인된 사실**" |
| `/specs[3]/open[5]` | "…나머지 20개는 전부 인라인이라 개별 조각으로 **존재하지 않는다**" |
| `/specs[2]/open[5]` | "…vtable 디스패치를 하나도 하지 않아 divtable 로 확인할 슬롯이 **없었다**" |
| `/specs[4]/open[2]` · `/specs[4]/open[4]` | "…아무것도 쓰지 않는다 / 애초에 참조가 없다" |

⟹ 제안: `closelist.py` 에 「`open[].q` 본문이 `확정`·`아니다`·`존재하지 않는다` 류로 끝나는데 `class=미탐색`」 규칙을 넣으면 이 8건이 기계로 잡힌다. (3차가 제안한 「`open[].q` ↔ `history[].was` 유사도 게이트」는 (a) 3건을 잡는다.)

---

## 4. `ev<=3` 표본 재확인 — 뒤집힘 0

3차가 본 30건과 겹치지 않도록 **새 항목 중심**으로 골랐다(한 행 = 한 오프셋).

| 항목 | 검증 | 결과 |
|---|---|---|
| `Effect+0x2c` = `attack_type` (AttackType 4B) | tcxdict | OK ★신규 |
| `Entity+0x618` = `stat_cached.attack` (= `stat_cached` 시작) | tcxdict + IR `%4+1560` | OK ★신규 |
| `Entity+0x630` = `stat_cached.defence` | tcxdict + IR `%4+1584` | OK ★신규 |
| `Entity+0x638` = `stat_cached.magic_resistance` | tcxdict + IR `%4+1592` | OK ★신규 |
| `EntityStat+0x10` = `hp` | tcxdict + MIR `(*_7).2` | OK |
| `EntityStat+0x18` = `defence` | tcxdict + MIR `(*_7).3` | OK ★신규 |
| `EntityStat+0x20` = `magic_resistance` | tcxdict + MIR `(*_37).4` | OK ★신규 |
| `BuffState+0xa8` = `defence_penetration` | tcxdict + MIR `(*_10).19` | OK ★신규 |
| `BuffState+0xb0` = `magic_resistance_penetration` | tcxdict + MIR `(*_40).20` | OK ★신규 |
| `BuffState+0xd0` = `base_attack_enemy_max_hp_damage` | tcxdict + MIR `(*_10).24` | OK |
| `BuffState+0xd8` = `self_max_hp_damage` | tcxdict + MIR `(*_10).25` | OK ★신규 |
| `BuffState+0xe0` = `skill_enemy_max_hp_damage` | tcxdict + MIR `(*_10).26` | OK ★신규 |
| `BuffState+0xf0` = `dot_amplify` | tcxdict + MIR `(*_10).28` | OK ★신규 |
| `AttackEffect+0x10` = `damage` | tcxdict + IR `%0+16` + 오라클 | OK ★신규 |
| `AttackEffect+0x18` = `attack_ratio` | tcxdict + IR `%0+24` + 오라클 | OK ★신규 |
| `AttackEffect+0x20` = `hp_ratio` | tcxdict + IR `%0+32` + 오라클 | OK ★신규 |
| `AttackEffect+0x28` = `target_hp_ratio` | tcxdict + IR + 오라클 | OK ★신규 |
| `TeamPlan+0x41f` = `objective` 판별자 | tcxdict + IR m09.ll:9203 | OK ★신규 |
| `TeamPlan+0x420` = `objective` 페이로드 태그 | tcxdict + IR m09.ll:9205 | OK ★신규 |
| `OperationData+0x0` = `cache` | tcxdict(24B·3필드) | OK |
| `Entity+0x98` = `ty.Jungle.info.camp_type.0` | tcxdict | OK |
| `Entity+0x5c0` = `id` | tcxdict | OK |
| `Entity+0x5c8` = `level` | tcxdict | OK |
| `Entity+0x4c8` = `skill_effect` | tcxdict | OK |
| `Entity+0x500` = `skill2_effect` | tcxdict | OK |
| `Entity+0x538` = `ult_effect` | tcxdict | OK |
| `Entity+0xb8` = `ty.Champion.0.skill_cooldown` | tcxdict | OK |
| `Entity+0xc0` = `ty.Champion.0.skill2_cooldown` | tcxdict | OK |
| `Entity+0xc8` = `ty.Champion.0.ult_cooldown` | tcxdict | OK |
| `Entity+0x680` = `radius` | tcxdict | OK |
| `Entity+0x470` = `stat_buff_cached.radius_mult` (i32) | tcxdict | OK |
| `Entity+0x438` = `stat_buff_cached.range` | tcxdict | OK |
| `Entity+0x488` = `stat_buff_cached.undying` | tcxdict | OK |
| `MobaMode+0x240` = `epic_minion_buff_time[0]` | tcxdict | OK |
| `Minion+0x18` = `nearest_enemy` (Option<usize> 16B) | tcxdict, `EntityType::Minion` 페이로드 +0x8 ⟹ Entity 기준 0x88/0x90 | OK |
| 5개 함수 `sig`·`src_line`·`vis` | `spec3lib.py fn` | 5/5 OK (280 / 520 / 31 / 17 / 543, 전부 `pub`·mir=False) |
| `AttackType` 태그 0~5 / `DamageType` 0~2 | `tcxdict --enum` | OK (BaseAttack·Skill·Dot·DotIgnoreShield·Item·Well / AD·AP·Fixed) |
| `MainObjective` 태그 0~11 | `tcxdict --enum` | OK (Direct 인코딩, 니치 밀림 없음) |

⟹ **ev<=3 사고 0건.**

---

## 5. G5/G6 사각지대 — `logic` ↔ 정본 눈대조 결과 0건

`logic` 의 모든 **오프셋 리터럴 · 상수 · 함수 시그니처 · 분기 극성**을 `mem`/`consts`/`callees`/`sig.tcx` 와 손으로 대조했다.

- 00: 오프셋 12개(0x5c8/0x538/0x28/0x30/0x10/0x18/0x438/0x470/0x680/0x660/0x668/0x1e0) 전부 일치. `vtable+0x118`·`+0xf8` 도 `/shared/EffectType_vtable`(`on_caster`·`linear_cast_range_margin`) 과 정합.
- 01: 상수 12개(2/3/200/2/80/4/20/40/1/−10/0/5) · 오프셋 8개 전부 `consts`/`mem` 과 일치. `~~< 2 임계~~` 정정이 `logic` 본문·`mem` note·`knobs` 셋 다에 반영돼 있다(G6 부류 재발 없음).
- 02: 3차 E1/E2 정정(arm 반전 + `src_line` 46→50)이 `logic`·`consts`·`knobs` **셋 다**에 들어가 있다. `logic` 의 `// L50, tag 16` 과 `consts[4].src_line=50` 일치.
- 03: 3차 E3 정정(`is_near_line` 1번 인자 = `&GameContext`)은 04 쪽 항목이라 03 에는 영향 없음. `logic` 의 모듈 경로 4/4 · 임계 2개(`die < tps<<1`, `cool <= tps`) · 레벨 게이트 2개(`>2`,`>4`) 전부 `consts`/`knobs` 와 일치.
- 04: `logic` 의 `40000000001` ↔ `consts[2]`, `(near−1) <u 2` / `near > 1` ↔ `knobs[1]`/`knobs[2]`, vtable `+0x40` ↔ `/shared/AbstractGame_vtable`, `Strategy+0xe morgard_defense` ↔ `mem` 전부 일치. 3차가 지적한 `Blackboard::minion_state` **메서드** 사실도 `history[5]` 에 들어가 있다.

⟹ **G5/G6 가 놓친 같은 부류 0건.** (다만 §3(b) 가 보여주듯, `logic` 이 아니라 **`open[]` 쪽이 정정에 안 따라오는** 부류가 남아 있다 — 그게 이번 라운드의 사각지대다.)

---

## 6. `callees_unmatched` — 판정 술어 누락 0건

3차 판정(00 `is_null` / 01 `llvm.smin.i64`·`vtable` / 02 없음 / 03 `player_champion`·`pool`·`setting` / 04 `wrapping_sub`)을 재확인했고 **전부 그대로**다.

⚠**3차 제안이 이번 라운드에서 값을 증명했다**(재확인 목적으로 기록): 01 은 `history` 에 `expected_damage_target` 본문을 실었는데, 그 본문의 술어
`game_core::utils::get_damage`(utils.rs:97, pub, mir=True) · `EntityType::is_structure`(entity.rs:1391, pub, mir=True) ·
`AbstractEntity::stat_ref`(vt+0x38) · `AbstractEntity::buff_state`(vt+0x40) ·
`EffectType::{expected_damage, expected_damage_structure, expected_target_hp_ratio}`(vt+0x28/0x30/0x38)
는 `callees` 에 **없다**(자동생성 범위가 「본 함수 `calls_raw`」뿐이라 설계상 정상). 이 중 `get_damage` 는 **mir=True** 라서 넣어 두면 다음 라운드가 MIR 칸 복원을 바로 쓸 수 있다.

---

## 7. ★브리핑 결함 — 1건

**§5 「현재 기준선」 수치가 실측과 다르다.** 방금 재측정(v3 반영 상태, 내 문서 넣기 전):

```
python -X utf8 tcxaudit.py            → 총 550건 · 오귀속 0 · 밀림 0 · 부분일치 0 · 확인불가 18 · OK 532
python -X utf8 tcxaudit.py --prose    → 총 680건 · 오귀속 0 · 밀림 0 · 부분일치 1 · 확인불가 18 · OK 661
python -X utf8 specgate.py            → 총 0건 (G1~G6 전부 0)   ← 이건 브리핑과 일치
```

브리핑 §5 는 `--prose` 를 「인자 없이 돌리면 **766건 · 부분일치 2**」라고 적었다. 어느 모드에서도 766·2 가 나오지 않는다(550·0 / 680·1). **부분일치는 1건**이고 출처는 3차 배치A 가 특정한 `_spec\specs20.json` 의 `cache` 팻포인터(오프셋 8) 산문 행이다. ⚠그 토큰을 이 문서에 그대로 쓰면 `tcxaudit --prose` 가 내 문서에서도 같은 부분일치를 1건 세니 표기를 피했다.
⟹ 다음 배치가 「내가 부분일치를 하나 늘렸다」로 오판할 수 있으니 기준선을 **680 / 부분일치 1 / 확인불가 18** 로 고쳐야 한다.
(브리핑 §5 의 다른 두 경고 — 표 형식만 쓰면 스캔 0건 · 없는 경로는 `[skip]` — 은 맞았고 그대로 따랐다. 위 오프셋들은 전부 `Type+0xNNN` 산문 형태 · 한 행 = 한 오프셋으로 적었다.)

---

## 8. 남은 `open` 과 그 상태 (닫지 못한 것 = **5건** + 표기불가 잔차 3건)

내 배치 `open` 22건 = **닫음 6 + 오분류 11 + 남음 5**(01·03·04 는 남음 0).

| 경로 | ev | 상태 | 이유·적용 범위 |
|---|---|---|---|
| `/specs[0]/open[0]` | 4 | **범위 좁힘(미탐색 유지)** | `sz==0` 은 `champ.(x,y) == target.(x,y)` 와 동치. `game_ai` 쪽 전 호출경로를 전수 확인했다 — 유일 호출자 `SmallActionUlt::get_input`(`small_action\cast.rs:261`, 호출 L273)은 `target = (*cache.game).get_entity_by_id(self.target_id)` 로 얻고 **좌표 비교를 하지 않는다**(L264 에서 `target.team == Player(1−my_team)` 만 본다). ⟹ **`game_ai` 계층에는 상위 가드가 없다**(전 경로 1/1 확인). 남은 것은 「시뮬레이션이 두 엔티티를 완전히 같은 좌표에 둘 수 있는가」로, 그건 `game_core` 충돌·분리 처리 쪽이고 이 배치에서 보지 않았다 |
| `/specs[0]` `entity.rs:1511` 잔차 4자 | — | **표기 불가(3차 판정 유지)** | 범위 표기 적절함을 확인했다: rmeta 줄길이 + MIR 칸 + tcx 필드명 3종 결합으로도 못 가름, 남은 미탐색 = 매크로 전개 여부, 문자열 자체는 재료 부재(`span_to_snippet` = SourceNotAvailable). **동작은 고정**(`stat_buff_cached.radius_mult as usize`, sext) |
| `/specs[1]` `action_score.rs:531` 잔차 | — | **표기 불가(3차 판정 유지)** | 범위 표기 적절. L531 내용 20자(재측정 일치)로 `if hp > value {`(15자)는 배제 ⟹ **arm 반전 확정**, `hp <= value` vs `value >= hp`(둘 다 11자)만 불가 |
| `/specs[2]/open[0]` | 4 | 미탐색 유지 | `version`(p2)이 무엇을 게이트하는지 — 이 함수에 참조 0건인 것은 확정. 게이트 소재는 다른 함수 소관 |
| `/specs[2]/open[1]` | 4 | 미탐색 유지 | `DW_OP_not` 이중 NOT — `/shared/DW_OP_not_아티팩트` + `history[2]` 로 **극성은 확정**됐고 남은 것은 「왜 컴파일러가 그렇게 냈나」뿐 |
| `/specs[2]/open[2]` | 4 | **사실상 `history[1]` 이 답했다** | `history[1]` = "★★다르다. 항상 반대다"(`passive_plan` 이 `1 − player.info.team` 을 넣는다). open 문구가 "확정 불가"로 남아 있다. ⟠(a) 부류에 넣을지 애매해 §3 에서 세지 않았다 — open 은 "이 함수만으론"이라는 **범위 한정**이 붙어 있어 그 범위 안에서는 여전히 참이다 |
| `/specs[2]/open[4]` | 4 | 미탐색 유지 | `twin_towers` 원소 앞 24B(ptr/ptr/cap) 필드명 |
| `/specs[2]` `attack_nexus.rs:41` 잔차 1자 | — | **표기 불가(3차 판정 유지)** | 범위 표기 적절. 재측정으로 잔차 1자 재확증(L41 개행 포함 59 → 내용 58 = indent 4 + 54자, 후보 55자). `mir=False` 확인 — MIR 칸 경로도 실제로 막혔다(`mirdump_game_ai.txt` 에 `attack_nexus.rs:41`·쌍둥이 `single_line.rs:286`·`passive_line.rs:1073` **전부 0건**) |

---

## 9. 방법론 소득

- ★**`pub` + `mir=True` 인 헬퍼를 찾으면 상위 함수의 잔여 미탐색이 한 번에 닫힌다.** `expected_damage_target`(mir=False)은 IR 로만 읽히지만, 그 안의 `get_damage` 는 **pub·mir=True** 라 `mirdump_game_core.txt` 에서 **칸까지 있는 전문**이 나왔다. ev5 한 건이 이 조합으로 ev2 까지 갔다.
- ★**「데이터 판별력이 없다」의 우회는 데이터 구조를 직접 만드는 것**이다. `ChampionInfo::default()` 가 액션 파라미터까지 0 이라 26/26 이 0 이었는데(`A4_o01b`), `AttackEffect`(전 필드 pub)를 손으로 구성하자 26/26 MATCH 가 됐다(`A4_o01c`). **"pub 구조체를 직접 조립"은 "실전 데이터 로딩"보다 값이 싸고 판별력이 높다.**
- ★**`inlinedAt` 이 「소스에 그 호출이 있었다」의 직접 증거다.** `/specs[3]/open[2]` 는 1~3차가 "LLVM 접힘인지 소스인지 구분 못 함"으로 남겼는데, `!DILocation(… inlinedAt: <line 30>)` 한 줄이 그걸 끝냈다. **「접힘인지 소스인지」류 질문은 먼저 `inlinedAt` 루트를 봐라.**
- ★**부작용 있는 호출이 건너뛰어졌으면 그 분기는 소스다.** `&mut` 인자 + `invoke`(언와인드 가능) 호출은 LLVM 이 지울 수 없다 — 「이 조기반환이 소스인가」를 가르는 **일반 논증**으로 쓸 수 있다.
- ⚠**rmeta `bytes`/`chars` 는 개행 1자를 포함한다.** 내용 길이 = 보고값 − 1. 이걸 놓치면 모든 줄에 잔차 1자가 생겨 「표기 불가」를 과다 생산한다(이번에 한 번 밟았다). 검산은 같은 함수에서 **내용이 자명한 줄**(`  }` / 꼬리표현식)로 하면 즉시 잡힌다.
