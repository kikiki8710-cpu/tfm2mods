# 5차 반증검증 — 배치 A (`specs[0]`~`specs[4]`) 보고서

> 게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
> 산출물 전량 = `C:\tfm2mods\MIG\_verify5\A\` (다른 배치 폴더는 읽지도 쓰지도 않았다)
> ⛔`_spec\specs20.json` · `_spec\specs20_v3.json` · `distruct.json` · `dienum.json` **무수정**(읽기만)

---

## ① 결론 한 줄

**실오류 1 · 판정반전 0 · 새 발견 9 · 미반영 정정 적발 2건(11행) · `ev4→ev2` 상향 146행**
(배치 A `ev≥4` 비율 **94.4% → 24.0%** 로 내려간다. 상향분 전량이 이 세션의 오라클 실행 출력으로 뒷받침된다)

---

## ② 정정 목록

### E1 (실오류) `/specs[0]/logic` — `caster_r` 를 `Effect::range` **안**에 넣어 적었다

| | |
|---|---|
| 경로 | `/specs[0]/logic` (L333 주석 블록, 3~4째 줄) |
| 구 | `Effect::range(effect, champ) = effect.range(0x10) + caster_r + champ.stat_buff_cached.range(0x438) + (champ.level - 1) * effect.growth_range(0x18)` / `total = Effect::range(..) + Effect::range_adjust(..) + target.radius()` |
| 신 | `Effect::range(effect, champ) = effect.range(0x10) + champ.stat_buff_cached.range(0x438) + (champ.level - 1) * effect.growth_range(0x18)` **(effect.rs:26 — caster_r 없음)** / `total = Effect::range(..) + caster_r + Effect::range_adjust(..) + target.radius()` **(abstract_input.rs:191~192)** |

**근거 3중**

1. **오라클(가장 강함)** — `_verify5/A/A5_o3_casting.tsv` `EFFECT` 줄: `Effect::range(&ef, champ)` 이
   `casting = Targeting / Position / Direction / None` **4값 전부 200000** 이다(`effect.range=200000`,
   `stat_buff_cached.range=0`, `growth_range=0`, `champ.radius()=10000`).
   caster_r 이 `Effect::range` 안에 있었다면 Targeting 만 210000 이어야 한다.
2. **같은 실행의 `total` 역산** — `ult()` 가 실제로 쓴 total 은 Targeting **220000** / 그 외 **210000**
   (`PREDICT ... MATCH_ULT=YES` 행이 `caster_r_ON`/`caster_r_OFF` 로 정확히 갈린다).
   `Effect::range(200000) + range_adjust(0) + target.radius(10000) = 210000` 이므로 Targeting 의 여분
   10000 = `champ.radius()` 이고, 그것은 `Effect::range` **밖**에서 더해진 것이다.
3. **IR** — `_gaibc/m04.ll:44245~44255`: `%158 = load i32, effect+48`(=+0x30 casting) → `icmp eq %158, 0`
   → `%175 = phi [0, %143], [%166, %164], [%173, %167]`(시전자 반경) 이 **`%177`(effect+16 range) 과 나란한
   별개 항**으로 합산된다. `Effect::range` 는 여기 통째로 인라인돼 있고 caster_r 은 그 바깥 항이다.

**⚠자기모순이었다**: 같은 파일 `/specs[0]/consts[5].meaning` 은 이미
「`effect_range_with_radii(abstract_input.rs:191)`에서 casting==Targeting 일 때만 시전자 반경을 더한다」로
**올바르게** 적고 있다. 정본 우선순위(`consts` > `logic`)대로 `consts` 가 맞고 `logic` 이 틀렸다.
**`specgate G1`(자기모순)이 이걸 못 잡았다** — G1 은 표↔표만 보고 `logic` 산문의 함수 경계 서술은 안 본다.

**파급**: 숫자 합계는 같으므로 값 오류는 아니다. 다만 이 줄은 `Effect::range` 의 **계약**을 잘못 말한다 —
재구현이 game_core `Effect::range` 를 경계로 삼으면 `champ.radius()` 만큼(기본 10000, 셀 1/3) 적게 나온다.

### M1 (미반영 정정) `/specs[3]` — history 가 지시한 `ev 4→2` 가 표에 반영되지 않았다

`/specs[3]/history[9]` 원문: 「`cc_threat` 오라클 **44/44 실행 확증**: `level>2`(skill2)·`level>4`(ult)·
`cool<=tps`(60 통과 / 61 탈락) ⟹ **consts[2]·consts[3]·knobs[1] ev 4→2**」
그런데 현재 표 = `consts[2].ev=4`, `consts[3].ev=4`, `knobs[1].ev=4`. **지시가 실행되지 않았다.**
→ 5차에 **내가 독립 재확인**했다(아래 ③ 참조, `A5_o6.tsv`). 반영해도 안전하다.

### M2 (미반영 정정) `/specs[4]` — 같은 종류

`/specs[4]/history[5]` 원문: 「★인원 구간 20/20 + 반경 경계(d2 = 40000000000 통과 · 40000400001 탈락):
Gather `>1` / Battle `{1,2}` ⟹ **knobs[0]·[1]·[2]·consts[2] ev 4→2**」
현재 표 = 전부 `ev=4`.
⚠**나는 5차에 이 4행을 재확인하지 못했다** — `has_line_defense_threat` 를 true 로 만들려면 적팀 미니언이
필요하고, `MinionRunner::update` 는 `minion_wave_setting` 없이 **0마리**를 돌려준다(`A5_o4.tsv`
`MINION_SPAWN n=0`, TEMPLATE ⑤ 재확인). **3차 근거로 반영을 권고**하되, 내 이름으로 ev2 를 주장하지는 않는다.

### M3 (미반영 정정, 묵시) ev 상향 지시를 **문장으로 쓰지 않아** 반영이 누락된 것들

| 경로 | history/근거 원문 | 현재 ev |
|---|---|---|
| `/specs[2]/consts[1]`·`[4]`·`[5]`, `/specs[2]/knobs[0..2]`, `/specs[2]/mem[17..20]` | `/specs[2]/history[3]` 「★오라클 실행 확증(2차배치A, `_verify2\A\A2_oracle3.tsv`) ①Recall ②LineDefense ③AttackNexus」 | 3 / 4 |
| `/specs[4]/consts[0]`·`[1]`, `/specs[4]/knobs[3]` | `/specs[4]/history[3]③` 「오라클 실행으로 확정: `TutorialType::spawn_epic()` 전 9값 실행」 | 4 |
| `/specs[4]/knobs[4]` | 「오라클 실측, 2차배치A」 | 4 |
| `/specs[1]/consts[6]`·`[7]` | `/specs[1]/history`·`consts.meaning` 「2차 배치A: rmeta_srcmap … arm 반전 구조는 확정」 | 3 |

전부 5차에 **내가 재실행**해 확인했다(③ 참조).

### 부분 상향만 가능(값은 실측, 의미의 일부는 미판별) — ev2 주장 보류

| 경로 | 실측한 것 | 못 한 것 |
|---|---|---|
| `/specs[0]/consts[3]`(=1) | `CastingType::Position` 판별자 = **1** (`A5_o2.tsv`) | 「310행 vtable(+0xf8) 반환 튜플 `.0 == 1` 비교값」 쪽은 `AttackEffect` 가 `linear_move_speed()=None` 이라 가드가 안 열려 미판별 |
| `/specs[0]/consts[4]`(=2) | `CastingType::Direction` 판별자 = **2** | 위와 동일 |
| `/specs[0]/mem[23]`(vt+0x10 align)·`mem[24]`(+0xf8)·`mem[25]`(+0x118) | 트레이트 메서드 `EffectType::on_caster()=false`·`linear_move_speed()=None` 실행 확인(`A5_o3_casting.tsv`) | **슬롯 번호 자체**는 대조 못 함(`Arc<dyn>` vtable 전역 부재) |
| `/specs[2]/mem[6]`(0x148 = twin_towers+0x18 len) · `/specs[3]/mem[11]` · `/specs[4]/mem[17]` | bumpalo Vec 32B 의 **+0x0 = 버퍼 기점 CONFIRMED**(`*(ptr)` == 원소0 주소), **+0x8 = Bump 참조**(값이 Bump 주소) | `+0x10`/`+0x18` 이 둘 다 `2` 라 **len 과 cap 을 실행으로 못 가른다**. ★범위 한정: `twin_towers` 로는 불가 — `len != cap` 인 bumpalo Vec 이 필요 |
| `/specs[2]/mem[16]`(AttackNexusPlan.line 0x8) | `AttackNexusPlan::new(1, Mid)` → 반환 payload `line=Mid`, `team(=1)` 미사용(`A5_o5.tsv`) | 필드가 `in:module` 사유라 `offset_of!` 불가 ⟹ 오프셋 숫자는 미실측 |

---

## ③ `ev4 → ev2` 로 올린 행 (이번 라운드 주 산출물)

### 방법 — 5차에서 처음 쓴 두 가지

1. ★**`std::mem::offset_of!` 일괄 대조**(중첩 경로 포함). `game_core` 의 구조체 필드가 **거의 전부 `pub`**
   이라(`Entity` 41필드 전부 pub) 명세 `mem[].offset` 을 **컴파일러가 계산한 실행 시점 숫자**와 한 프로세스에서
   전수 대조할 수 있다. 열거형 variant 페이로드(`ty.Champion.*`)는 `offset_of` 가 안 되므로 **실주소 차**로 쟀다.
   → `A5_o1.rs` / `A5_o1.tsv` (오프셋 45행 + 구조체 크기 20건, **전건 MATCH, MISMATCH 0**)
   → `A5_o2.rs` / `A5_o2.tsv` (중첩·stride·열거형 태그·variant 페이로드, **전건 MATCH**)
2. ★**세계 직접 조작 + 차분 오라클**. `World::entity` 가 `engine_core::container::Container<Entity>` 이고
   `get_mut` 이 `pub` 이라 `level`/`ult_effect`/`hp`/`x`/`y`/`visible_state`/`stat_buff_cached`/`ty.Champion.*`
   를 정상 경로로 세팅할 수 있다. 임계값은 **±1 로 흔들어 판별력을 확인**했다(예: margin 149999/150000/150001).

프로브 목록(전부 `sh MIG\_verify3\build.sh` 로 빌드, `real_setting()`+`setting_ok()=true`,
`init_tower`/`init_nexus` 미호출 → `towers=16 twin=2/2` 무결성 확인):

| 프로브 | 대상 | 출력 |
|---|---|---|
| `A5_o1.rs` | 배치 A `mem` 오프셋 45 + 크기 20 | `A5_o1.tsv` |
| `A5_o2.rs` | 중첩/stride/variant/태그 | `A5_o2.tsv` |
| `A5_o3.rs` | `specs[0] game_ai::ult` (케이스당 1프로세스) | `A5_o3_level.tsv` · `A5_o3_casting.tsv` · `A5_o3_visible.tsv` · `A5_o3_samepos.tsv` |
| `A5_o4.rs` | `specs[0]` radius / `specs[1]` 계수표·최종식 | `A5_o4.tsv` · `A5_o4_hpsweep.tsv` |
| `A5_o5.rs` | `specs[2]` 3분기+sret 바이트 / `specs[4]` 튜토리얼 게이트 | `A5_o5.tsv` |
| `A5_o6.rs` | `specs[3]` `cc_threat` 축 | `A5_o6.tsv` |
| `A5_o7.rs` | 남은 `ev<=3` 표본 재확인 | `A5_o7.tsv` |
| `tmpl.rs` | `_verify3\TEMPLATE.rs` 의 `real_setting`/`setting_ok`/`mkgame` 모듈판 | — |

### 상향 목록 (총 146행)

#### `specs[0]` `ult` — 36행

* `mem[0]`(0x930) `mem[1]`(0x9c0) `mem[2]`(0x0) `mem[3]`(0x8) `mem[4]`(0x1e0) `mem[5]`(0x8) `mem[6]`(0x20)
  `mem[7]`(0x0) `mem[8]`(0x8) `mem[9]`(0x38) `mem[10]`(0x438) `mem[12]`(0x538) `mem[13]`(0x5c8)
  `mem[14]`(0x660) `mem[15]`(0x668) `mem[16]`(0x680) `mem[17]`(0x0) `mem[19]`(0x10) `mem[20]`(0x18)
  `mem[21]`(0x28) `mem[22]`(0x30) — `offset_of!` 실측 MATCH (`A5_o1.tsv`, `A5_o2.tsv`)
* `mem[11]`(0x470 `stat_buff_cached.radius_mult`) **ev3→ev2** — 오프셋 MATCH + **부호확장 판별**:
  `radius_mult=-50` → `Entity::radius()` = **5000**(=sext 예측), zext 가설은 429496734600 (`A5_o4.tsv` `RADIUS` 줄)
* `mem[26]`(sret 판별자 -1/5) — `None` = **-1**, `Some(Input::Ult)` = **5** 실측(`A5_o7.tsv`), 그리고
  `ult()` 가 실제로 `Ult(Target{target_id:23})` 를 낸 케이스 관측(`A5_o3_samepos.tsv`)
* `mem[27]`(0x8 `Input::Ult.target` 24B) **ev3→ev2** — `Some(Input::Ult{target:Pos{x:0x1111222233334444,
  y:0x5555666677778888}})` 를 +0x8 에서 `InputTarget` 으로 읽어 값이 그대로 나옴(`A5_o7.tsv`)
* `mem[28]`(sret 전체 32B = `safe_move_avoiding_enemy_well` 출력) — 가시(338행)·비가시(340행) **양쪽에서
  `ult()` 반환 == `safe_move_avoiding_enemy_well(...)` 반환**이 문자열 동일(`A5_o3_casting.tsv` `MATCH_ULT=YES`,
  `A5_o3_visible.tsv` `PREDICT_L340 ... MATCH_ULT=YES`)
* `consts[1]`(=4 궁 해금 레벨) — level 1/4 → `can_ult=false`·`ult_eff_some=false`·`ULT_RESULT=None`,
  level 5/6 → `true`·`true`·`Move(...)` (`A5_o3_level.tsv`)
* `consts[2]`(=-1 Option None) — `Option<Effect>=None` 의 비-0 i32 워드가 **`+0x30` 에서만 -1**(`A5_o2.tsv` `niche` 줄)
* `consts[5]`(=0 Targeting 태그 → 시전자 반경 가산) — Targeting 만 total 220000, Position/Direction/None 은
  210000 (`A5_o3_casting.tsv`)
* `consts[6]`(=0 `VisibleState::Visible`) **ev5→ev2** — 태그 실측 `Visible=0 / Invisible=1 / Unknown=2`
  (`A5_o2.tsv`) + 가시/비가시로 **338행 ↔ 340행 분기가 실제로 갈림**(`A5_o3_visible.tsv`)
* `consts[7]`(=100 반경 백분율 기준) — `radius_mult` 0/50/200/-50 → `radius()` 10000/15000/30000/5000 (`A5_o4.tsv`)
* `consts[8]`(=5 `Input::Ult` 태그) **ev3→ev2** — `A5_o7.tsv` `INPUT_TAG` 전표
  (`Move 0 / Return 1 / Attack 2 / Skill 3 / Skill2 4 / Ult 5`)
* `consts[9]`(=150000) — **margin ±1 판별**: 149999·150001 은 불일치, **150000 에서만** `ult()` 와 비트 동일
* `knobs[0]`(4) `knobs[1]`(150000) `knobs[2]`(0 = Targeting 만 반경 가산) `knobs[3]`(100)

★**`knobs[1]`=150000 의 game==mine 비트동일**(가장 강한 근거):
`total.saturating_sub(150000)` 부터 `Game::adjust_position` → `safe_move_avoiding_enemy_well` 까지를
**pub API 만으로 재구현**해 `ult()` 반환과 대조했다. Targeting: 예측 `Move(863503,64497)` == 실측, margin 149999→
`Move(863502,64498)`(불일치) / 150001→`Move(863504,64496)`(불일치). Position/Direction/None: 예측
`Move(741223,140490)` == 실측, 역시 margin 150000 에서만 일치. (`A5_o3_casting.tsv`)

#### `specs[1]` `calculate_jungle_action_score` — 23행

* `mem[0]`~`mem[5]`, `mem[7]`(0x670 hp) — `offset_of!` MATCH
* `mem[6]`(0x98 `ty.Jungle.info.camp_type.__0`) — 실게임 캠프에서 team0 → **0**, team1 → **1** 로 읽히고,
  그 자리에 **2 를 강제로 써 넣으면 점수가 45 → 0** 으로 바뀐다(`A5_o4.tsv`)
* `consts[1]`(3 Nexus) `consts[2]`(200) `consts[3]`(2 Tower) `consts[4]`(80) `consts[5]`(4 Jungle)
  `consts[10]`(0 = 그 외 전부) `consts[11]`(5 based) — 실엔티티 전수 `game==mine` MATCH:
  Nexus tag3→200 / Tower tag2→80 / Champion tag13→0 / Jungle tag4→coef+based (`A5_o4.tsv`)
* `consts[6]`(20) `consts[7]`(40) **ev3→ev2** — ★**hp 스윕으로 `hp <= value` 를 판별**했다:
  `value=496` 고정, `hp=496` 에서 game=**45**(coef 40) 이고 대립가설 `hp < value` 는 25 를 예측한다.
  `hp=497` → 24(coef 20). 9점 전부 `min(coef, coef*value/hp)+based` 와 일치(hp=992→15, 993→14 로
  0방향 절삭까지 일치). (`A5_o4_hpsweep.tsv`)
  ※`hp <= value` / `value >= hp` 중 어느 **표기**인지는 여전히 **표기 불가**(외연 동일) — 그 판정은 유지.
* `knobs[0]`(200) `knobs[1]`(80) `knobs[2]`(20) `knobs[3]`(40) `knobs[5]`(5)
* `knobs[6]`(정글 캠프 소유 팀 게이트) **ev3→ev2** — team0 캠프 6개·team1 캠프 6개 **전부 45**(coef 40 + based 5)
  ⟹ 「`is_jungle(0) || is_jungle(1)` = 양 팀 캠프 전부」가 **실행으로 확정**. `camp_type.__0=2` 강제 시 0
  ⟹ IR 의 `icmp ult camp_type.0, 2` 가 그 OR 의 접힘이라는 2차 정정도 재확증.

**미상향(범위 명시)**: `consts[8]`(1 Minion 태그) · `consts[9]`(-10) · `knobs[4]`(-10) —
`MinionRunner::update` 가 `minion_wave_setting` 없이 **0마리**를 돌려줘 Minion 엔티티를 못 만들었다
(TEMPLATE ⑤ 레시피 미적용). **「검증 불가」가 아니라 「이 프로브에서 미검증」**이다.

#### `specs[2]` `AttackNexusPlan::sub_plan` — 27행

* `mem[0]`~`mem[5]`, `mem[7]`(0x6d70), `mem[12]`~`mem[15]` — `offset_of!` MATCH
* `mem[8]`~`mem[11]`(lx/ly/rx/ry) — `map.fountains[0]` 실측 `(0, 896000, 64000, 960000)` 이고
  챔프를 `(lx+1000, ly+1000)` 에 두면 **샘 안 판정**이 서서 Recall 이 난다 ⟹ `(x0,y0,x1,y1)` 순서 확정(`A5_o5.tsv`)
* `mem[17]`(sret 판별자) `mem[18]`(+0x8 style) `mem[19]`(+0x9 line) `mem[20]`(+0xa minion_action_type) —
  ★**sret 바이트 직접 덤프**: LineDefense 케이스가 `tag=2 b8=0 b9=1 b10=2` 이고
  `Debug` 는 `LineDefense(LineDefenseSubPlan{ line: Mid, minion_action_type: Push, style: Aggressive })`
  ⟹ style=Aggressive(0)@+0x8 · line=Mid(1)@+0x9 · Push(2)@+0xa **전부 실측 일치**(`A5_o5.tsv`)
* `consts[1]`(5 Recall) **ev3→ev2** — 샘 안 + hp 500/999 → `tag=5`
* `consts[3]`(0) `consts[4]`(16) `consts[5]`(2) — `no_twin`(적 트윈타워 제거) → `tag=16`, LineDefense → `tag=2`
* `knobs[0]`(hp < max) — 샘 안 hp 500/999 → **Recall** / 샘 안 hp 999/999 → **LineDefense** (분기 순서까지)
* `knobs[1]`(적 트윈타워 empty) — `twin1_len=2` → LineDefense / `twin1_len=0` → **AttackNexus**
* `knobs[2]`(LineDefense{Aggressive, self.line, Push}) — 위 sret 바이트
* `knobs[3]`(샘 사각형) — 실값 + 동작

#### `specs[3]` `defensive_crisis` — 31행

* `mem[0]`~`mem[9]`, `mem[12]`, `mem[16]`~`mem[21]` — `offset_of!` MATCH
* `mem[10]`(bumpalo Vec ptr @+0x0) — `*(word0)` == 원소0 주소 (`A5_o7.tsv` `BUMPVEC`)
* `mem[13]`(0xb8) `mem[14]`(0xc0) `mem[15]`(0xc8) — Champion variant 페이로드 **실주소 차** MATCH(`A5_o2.tsv`)
  + `A5_o6.tsv` 에서 세 필드를 각각 조작해 분기가 실제로 갈림
* `consts[1]`(13 Champion) — `Entity+0x68` 실측 13(Champion)·2(Tower)·3(Nexus)·4(Jungle)
* `consts[2]`(=2, `level>2` → skill2 개방) — slot=skill2, **level 3 → cc_threat=1 / level 2 → 0**
* `consts[3]`(=4, `level>4` → ult 개방) — slot=ult, **level 5 → 1 / level 4 → 0**
* `consts[4]`(=-1 니치) — `A5_o2.tsv` niche 줄
* `consts[6]`(=1 `Some` 판별자) — `effect_cc_time(1, StunEffect)` = `Some(120)`,
  `effect_cc_time(1, AttackEffect)` = `None` 이고 그에 따라 `cc_threat` 이 갈림
* `knobs[1]`(cool <= tps) — tps=60 에서 **cool=60 → 1 / cool=61 → 0** (`<=` 경계 확정)
* `knobs[3]`(2) `knobs[4]`(4)
* `knobs[19]`(판단 능력치 원본 = `AthleteStat.judgement` = PlayerState+0x218) **ev5→ev2** —
  `mkgame` 이 `st.judgement=80` 을 넣었고 `PlayerState+0x218` 을 읽으면 **80**(`A5_o7.tsv`)
* `knobs[20]`(`judgement_mental_ratio` = PlayerState+0x450, 초기 1000) — 실측 **1000**(`A5_o7.tsv`)
  ⟹ 「초기값 1000(`AthleteParameter::new`)」이 실행으로 확증

추가 관측(명세 서술 재확증): slot=skill 은 **level 1 에서도 cc_threat=1** ⟹ `skill_effect` 는
「레벨 게이트 없이 항상 후보」가 실행으로 맞다(`A5_o6.tsv`).

#### `specs[4]` `handle_line_defense` — 29행

* `mem[0]`~`mem[6]`, `mem[9]`~`mem[16]`, `mem[18]`~`mem[20]`, `mem[21]`, `mem[22]`~`mem[24]`,
  `mem[25]`·`mem[26]` — `offset_of!` MATCH (타워 슬롯 0x180/0x190/0x1a0/0x1b0/0x1c0/0x1d0 포함)
* `mem[7]`(AbstractGame vtable **+0x40 = get_game_mode**) **ev3→ev2** — ★런타임 슬롯 대조:
  `&dyn AbstractGame` 를 transmute 해 vtable 의 `+0x40` 슬롯을 **직접 호출**한 결과가
  트레이트 호출 결과와 **같고**, 둘 다 `&game.mode` 를 가리켰다 (`A5_o7.tsv` `VT ... verdict=get_game_mode CONFIRMED`).
  덤으로 `closed[3]`(「런타임에 어느 구현체가 꽂히는지 확인 불가」)이 `Game` 에 대해 해소된다.
* `mem[8]`(MobaMode+0x240 `epic_minion_buff_time`) — `offset_of!` MATCH + **`size_of::<MobaMode>()=640` 실측**
  ⟹ `closed[4]` 의 「MobaMode 는 `dereferenceable(N)` 이 없어 **크기 교차검증을 못 했다**」가 해소된다.
  `remain_epic_time(0/1)` 이 `epic_minion_buff_time[0/1]` 을 그대로 돌려주는 것도 확인(시간 산술 없음).
* `consts[0]`(-1) `consts[1]`(6) `knobs[3]`(6) — `TutorialType::spawn_epic()` 전 9값 실행:
  `0=true / 1~6=false / 7=true / 8=true` ⟹ `(tutorial as u8).wrapping_sub(1) <u 6` 이면 false 가
  **실행으로 확정**(`A5_o5.tsv` `SPAWN_EPIC`). 덤으로 `morgard_exists(&ctx)` 가 **인자 1개**(`&GameContext`)
  로 컴파일·실행되는 것 자체가 3차의 구조 정정(「(b)(c)는 morgard_exists 밖」)을 재확증한다.

**미상향**: `consts[2]`(40000000001) `knobs[0]`·`[1]`·`[2]` — §②M2 참조(미니언 필요).
`mem[17]`(Vec len 0x18) — len==cap 이라 판별 불가.

### 상향 후 예상 분포 (배치 A, `mem`+`consts`+`knobs` 196행)

| | 현재 | 상향 후 |
|---|---|---|
| ev1 | 0 | 0 |
| ev2 | 3 | **149** |
| ev3 | 8 | **0** |
| ev4 | 183 | **47** |
| ev5 | 2 | **0** |
| **ev≥4** | **94.4%** | **24.0%** |

---

## ④ `ev<=3` 표본 재확인 — **뒤집힌 항목 0**

배치 A 의 `ev3` 8행 + `ev5` 2행을 **전부** 다시 쟀다.

| 경로 | 기존 판정 | 5차 재측정 | 결과 |
|---|---|---|---|
| `/specs[0]/mem[11]` 0x470 radius_mult (ev3, MIR 근거) | 부호확장(sext) | `mult=-50` → `radius()`=5000 (sext 예측) | **유지**(ev2 로 상향) |
| `/specs[0]/mem[27]` 0x8 Input::Ult.target (ev3) | payload @+0x8, 24B | 값 왕복 일치 | **유지** |
| `/specs[0]/consts[8]` =5 Input::Ult 태그 (ev3) | 5 | 5 | **유지** |
| `/specs[0]/consts[6]` =0 VisibleState::Visible (ev5) | Visible=0 | Visible=0, Invisible=1, Unknown=2 + 분기 갈림 | **유지** |
| `/specs[1]/consts[6]` =20 (ev3) | Jungle·처치불가 | hp=497 → 24 (=20+5-1 비례) | **유지** |
| `/specs[1]/consts[7]` =40 (ev3) | Jungle·처치가능, `hp <= value` | hp=496=value → 45 (coef 40) | **유지 + 강화**(`<=` 판별됨) |
| `/specs[1]/knobs[6]` 소유 팀 게이트 (ev3) | 양 팀 캠프 전부 | team0·team1 캠프 12개 전부 45 | **유지** |
| `/specs[2]/consts[1]` =5 Recall (ev3) | 5 | `tag=5` | **유지** |
| `/specs[3]/knobs[19]` judgement = PS+0x218 (ev5) | 선수 능력치 원본 | `PS+0x218 = 80`(= 넣은 값) | **유지** |
| `/specs[4]/mem[7]` vt+0x40 = get_game_mode (ev3) | divtable 일치율 98% | 슬롯 직접 호출 == 트레이트 호출 == `&game.mode` | **유지** |

그 밖의 `ev3` (구조화 표 밖): `/specs[3]/notes[0]`(fnparts 서브프로그램 23 vs define 3) ·
`/specs[0]/sig.ev`·`params` 계열 — 도구 상태 서술이라 재측정 대상이 아니다. 손대지 않았다.

`ev2` 3행(`/specs[3]/knobs[0]`·`[2]`·`[7]`)도 뒤집히지 않았다:
`A5_o6.tsv` 에서 `die_imminent=1` 이 tps=60 세팅에서 일관되게 나오고(knobs[0] 방향 일치),
`max_range(foe,target)=120000` + `(mr+30000)^2 = 22500000000 ≥ dist_sq 2000000` 로 필터를 통과한다(knobs[2]).

---

## ⑤ 게이트 실측 출력

```
$ cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 specgate.py
## 명세 완결 조건 검사 (게임 0.5.8)
   G1 자기모순=0  G10 class 오분류=0  G2 호출부 전수=0  G3 형제 함수=0  G4 술어 시그니처=0
   G5 sig 정본 대조=0  G6 logic 미반영=0  G7 과열림=0  G8 표에 옛 값=0  G9 callees 오염=4
### [G9 callees 오염] 4건   ← 전부 `손확인` 등급, 전부 배치 B/C/D 범위(07·13·14·15). 배치 A = 0건
총 4건
⟹ 제출 조건(G1~G8·G10 = 0) 충족
```

```
$ PYTHONIOENCODING=utf-8 python -X utf8 _spec/audit4.py
미반영 0 · STALE 0 · ev불일치 0   → 전건 반영됨
(4차 36항목 + ev상향 2건 전부 OK)
```

```
$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py
총 551건  오귀속=0  밀림=0  부분일치=0  확인불가=18  OK=533

$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose
총 686건  오귀속=0  밀림=0  부분일치=1  확인불가=18  OK=667
```

```
$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose _verify5/A/A5_REPORT.md
총 690건  오귀속=0  밀림=0  부분일치=1  확인불가=18  OK=671
※ ~~취소선~~(이미 정정된 옛 값)이라 스캔에서 제외한 오프셋 주장 4건 — resolved_2026-09-10.json 2, specs20.json 2
```

**델타 = `+4건, 전부 OK`**(686→690, OK 667→671). `오귀속 0 · 밀림 0` **유지**,
`확인불가` 18 **불변**, `부분일치` 1 은 기준선에 이미 있던 `specs20.json base=cache offset=0x8` 이고 내 것이 아니다.
내 보고서에서 스캔된 행 = `A5_REPORT.md Entity 0x68 주장=(산문) tcx=ty.tag` 등 4건.
⚠오프셋은 **한 행에 하나씩만** 적었다(4차 D-E5 함정 회피). 명세 JSON 은 수정하지 않았으므로
기준선 대비 다른 델타는 없다.

### 참고 — 내가 직접 재본 기준선 (브리핑 §1 표 검산)

```
행 집합 = mem + consts + knobs (open/notes/sig.params 제외)
배치 A : n=196  ev2=3  ev3=8  ev4=183  ev5=2  ev>=4=94.4%   ← 브리핑 표와 완전 일치
전 20  : n=858  ev2=13 ev3=55 ev4=788  ev5=2  ev>=4=92.1%   ← 「858행 중 13행」과 일치
open 총 11 (A=5) · notes 총 8 (A=3)                          ← 브리핑 §1·§3 과 일치
```

---

## ⑥ 브리핑(`_verify5\BRIEF.md`)의 오류

**값 오류는 0건이다.** 4차와 달리 §1 표·`open`/`notes` 개수·「858행 중 13행」이 전부 실측과 일치했다
(위 검산 블록). 아래는 오류가 아니라 **다음 라운드에 한 줄 추가하면 시간이 절약되는 지점**이다.

1. ★**§1 표의 「행」 정의가 안 적혀 있다.** 나는 같은 숫자를 재현하려고 6가지 집합
   (`mem+consts+knobs` / `+open+notes` / `+sig.params` / `+sig.ev` …)을 돌려봐야 했다. 정답은
   **`mem`+`consts`+`knobs` 만**이다. 브리핑 §1 표에 `(행 = mem+consts+knobs)` 한 줄만 있으면 된다.
   ⚠참고로 `sig.params` 까지 세면 A 는 237행이 되고 `sig.ev` 까지 세면 242행·`ev≥4` 92.6% 가 나온다
   — **다음 라운드가 「브리핑이 틀렸다」고 오적발할 위험**이 있는 지점이다.
2. **§5 의 게이트 목록에 `ev` 검사가 없다.** 이번 배치 최대 문서 오염(§②M1·M2·M3)은
   **「history 가 `ev 4→2` 를 지시했는데 표의 `ev` 가 그대로」** 였고, `specgate G8` 은 **값만** 보고
   `audit4.py` 는 **4차분만** 본다. 그래서 2차·3차의 ev 상향 지시가 조용히 누락돼 있었다.
   → **G11 제안**: `history[]`/`closed[]`/`knobs[].effect` 텍스트에서 `ev ?\d ?(→|->) ?\d` 패턴과
   `오라클`·`실행 확증`·`MATCH` 마커를 찾아 **지목된 행의 현재 `ev`** 와 대조. 정규식 한 줄로 잡힌다
   (내가 그렇게 찾았다: `_verify5/A/` 의 조회 스크립트 참조).
3. **§0 의 「판정반전도 오류로 센다」가 `logic` 산문에는 적용 기준이 없다.** 내 E1 은 값이 아니라
   **함수 경계 오귀속**이라 「값 오류」도 「판정반전」도 아니다. 4차가 「3차 최대 오염원 = `logic` 산문」
   이라고 결론냈으니, **`logic` 의 계약 서술 오류를 별 분류(예: `계약오류`)로 세는 칸**을 §0 에 두면
   집계가 흔들리지 않는다.
4. **§2④ 템플릿 함정 목록에 ⑥을 추가할 만하다** — 「`GameSetting::default()` 는 0」과 같은 급으로
   **「`SwordmanChampionInfo::default()` 엔티티는 `hp==1`, `stat_cached`·이펙트가 비어 있다」**.
   내 `specs[1]` 계수표에서 전 대상이 `hp=1` 로 나와 `min()` 의 **비례 구간을 못 밟았고**, 그래서 hp 를
   손으로 넣어 스윕해야 판별력이 생겼다(`A5_o4_hpsweep.tsv`). **「결과가 전부 같으면 입력이 상수인지 보라.」**
5. **§1 「우선순위: knobs → consts → mem」은 효율 측면에서 뒤집는 게 낫다.** `mem` 이 가장 많고(A 107행)
   `offset_of!` 로 **한 프로세스에 전수 대조**돼 비용이 거의 0 이다. 실제 이번 상향 146행 중 **99행이 `mem`**
   이었고 시간은 전체의 15% 도 안 걸렸다. 브리핑이 "장식적 오프셋은 뒤로"라고 해서 나는 처음에
   `mem` 을 미뤘는데, 그 판단이 손해였다. → **「`mem` 은 `offset_of!` 로 일괄 먼저 털고, 남은 시간을
   `knobs`/`consts` 의 ±1 판별에 쓰라」**가 맞다.

---

## 부록 A — 새 발견 9건

| # | 발견 | 근거 |
|---|---|---|
| N1 | **`game_core::Entity` 41필드가 전부 `pub`** 이고 `World::entity` 는 `engine_core::container::Container<Entity>`(`get_mut` pub). ⟹ 오라클에서 세계를 직접 세팅할 수 있다(level·ult_effect·hp·좌표·visible_state·stat_buff_cached·ty.Champion.*). 「입력 판별력 부재」를 쓰기 전에 이걸 먼저 보라 | `_tcx/game_core.json`, `A5_o3.rs`·`A5_o4.rs`·`A5_o6.rs` 실행 |
| N2 | ★**`std::mem::offset_of!`(중첩 경로 포함) 로 명세 `mem` 표를 기계 전수 대조**할 수 있다. 45행+크기 20건이 한 프로세스에서 MISMATCH 0. 열거형 variant 페이로드는 실주소 차로 보완 | `A5_o1.tsv`·`A5_o2.tsv` |
| N3 | `JungleCampState::spawn(&GameSetting, usize, usize)` 가 `pub` ⟹ **정글 캠프 Entity 를 월드 없이 생성** 가능(팀별 6개 확보). 반면 `MinionRunner::update` 는 `minion_wave_setting` 없이 **0마리**(TEMPLATE ⑤ 재확인) | `A5_o4.tsv` |
| N4 | `/specs[0]/open[0]`(sz==0 div-by-zero) **범위 축소**: `sz==0`(champ==target)이면 `Effect::is_in_range` 가 **eff_range=0 에서도 true** 라 `get_input_target` 이 `Some` 을 돌려주고 **L326 에서 `Some(Input::Ult)` 로 빠진다**(실측 `Ult(Target{target_id:23})`). ⟹ L334 패닉은 `sz==0` **이면서** `get_input_target` 이 **L346 `?` 로** None 인 경우만 남는다(L349 는 sz==0 과 모순, L354 는 L328 가시성 게이트와 모순) | `A5_o3_samepos.tsv` |
| N5 | `size_of::<MobaMode>() = 640` 실측 ⟹ `/specs[4]/closed[4]` 의 「크기 교차검증 못 했다」 해소. `AbstractGame` vtable **+0x40 슬롯 직접 호출**이 트레이트 호출과 동일 + `&game.mode` 반환 ⟹ `/specs[4]/closed[3]` 의 「어느 구현체가 꽂히는지 확인 불가」가 `Game` 에 대해 해소 | `A5_o1.tsv`·`A5_o7.tsv` |
| N6 | **실측 태그 전표**: `Input` Move0·Return1·Attack2·Skill3·Skill2 4·Ult5 / `Option<Input>` None=-1 / `VisibleState` Visible0·Invisible1·Unknown2 / `CastingType` Targeting0·Position1·Direction2·None3 / `TeamType` Player(tag 0, payload@+0x8)·Neutral(tag 1) / `EntityType` Champion13·Tower2·Nexus3·Jungle4 | `A5_o2.tsv`·`A5_o7.tsv` |
| N7 | `/specs[1]` 의 arm 조건은 **`hp <= value`** 다(등호 포함). `hp=496=value` 에서 coef 40 이 나와 `hp < value` 가설과 갈린다 — 기존 「arm 반전 구조는 확정, 표기는 불가」에 **등호 포함까지** 더해진다 | `A5_o4_hpsweep.tsv` |
| N8 | `/specs[3]` `cc_threat` 의 창은 **`cool <= tps`**(60 통과 / 61 탈락). 최종식 `min(coef, coef*value/hp)+based` 는 9점 전수 game==mine | `A5_o6.tsv`·`A5_o4_hpsweep.tsv` |
| N9 | `Entity::radius()` 의 `radius_mult as usize` **부호확장이 실행으로 확정**. `mult=-50` → 5000(sext), zext 가설은 429496734600 로 3자리 이상 갈린다 ⟹ 재구현에서 `as usize` 를 빼면 음수 버프에서 반경이 폭주한다 | `A5_o4.tsv` |

## 부록 B — 이번 라운드에 쓴 재료와 그 한계

* `⑥ SDK 실행 오라클` — 주력. `game_ai::ult` · `calculate_jungle_action_score` ·
  `AttackNexusPlan::sub_plan` · `defensive_crisis` · `rule_scope::morgard_exists` ·
  `TutorialType::spawn_epic` · `Effect::range` / `range_adjust` / `is_in_range` / `expected_damage_target` ·
  `Entity::radius` / `can_ult` / `is_visible_from` · `Game::adjust_position` · `utils::isqrt` ·
  `safe_move_avoiding_enemy_well` · `max_range` · `effect_cc_time` — **전부 직접 호출 성공**.
* `⑦ tcx` — `pub` 여부·시그니처·필드 목록 조회에만 사용(`_verify5/A/q.py`).
* `① IR` — E1 의 3번째 근거로만 사용(`_gaibc/m04.ll:44245~44255`, `m05.ll:39984` 서명줄).
* **알려진 한계로 남긴 것**(범위 명시):
  1. **bumpalo Vec 의 `len`(+0x18) vs `cap`(+0x10)** — `twin_towers` 는 len==cap==2 라 실행 판별 불가.
     `len != cap` 인 bumpalo Vec 을 만들면 가능하다(미탐색).
  2. **`Arc<dyn EffectType>` vtable 슬롯 번호**(+0xf8·+0x118·+0x10) — 트레이트 메서드의 **동작**은 실행으로
     확인했지만 **슬롯 번호**는 여전히 정적 vtable 전역 부재로 대조 불가. `_gcbc` 의 `EffectType` vtable
     전역(`_shared.EffectType_vtable`)이 그 자리를 메운다.
  3. **`specs[1]` Minion 계수(-10)** 와 **`specs[4]` 인원 구간·반경 임계** — 둘 다 **미니언 엔티티**가 필요하고
     `minion_wave_setting` 16필드 주입 + `run_tick` 600틱(TEMPLATE ⑤)을 이번에 적용하지 않았다.
     **「불가」가 아니라 「미적용」**이다. 다음 배치는 이걸로 두 건을 한 번에 열 수 있다.
  4. **`AttackNexusPlan.line`(+0x8)** — 필드가 `in:module` 이라 `offset_of!` 불가. `transmute` 로는 가능(미시도).
