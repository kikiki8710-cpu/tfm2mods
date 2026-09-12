# 6차 반증검증 — 배치 A (`specs[0]`~`specs[4]`) 보고서

> 게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
> 산출물 전량 = `C:\tfm2mods\MIG\_verify6\A\` (다른 배치 폴더 B/C/D 는 읽지도 쓰지도 않았다)
> ⛔`_spec\specs20.json` · `_spec\specs20_v3.json` · `distruct.json` · `dienum.json` **무수정**(읽기만)

---

## ① 결론 한 줄

**실오류 8 · 판정반전 1 · 분류오류 2 · 보강 2 (= `errors` 13건) · 새 발견 6 · `ev` 상향 158행**
그리고 **그 13건 중 `reused` 가 11건**이다 — 즉 **「계측기를 새로 붙였기 때문에 오류가 보였다」는 가설은 이번 라운드 배치 A 에서 성립하지 않았다.**
(`ev≥4` 비율은 적용 시 **94.4% → 16.3%**, 185행 → 32행.)

---

## ② `patch.json` 요약

`_verify6\A\patch.json` (38,586 B, 생성기 = `mkpatch.py` — 손으로 158행을 옮겨 적다 빠뜨리는 5차 사고를 막으려고 스크립트로 만들었다)

```
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 6 --only A --dry
  [A] 정정 13/13 · ev상향 158/158
  정정 13 성공 / 0 실패 · ev상향 158 성공 / 0 실패 · **동작 변경 0건**
  발견 경위: inherited=2 · reused=11
```
**적용 실패 0.**

### `errors` 13건

| # | 경로 | 종류 | 요지 | `found_by` |
|---|---|---|---|---|
| E1 | `/specs[0]/open[0]` | 판정반전 | `sz==0` 의 div-by-zero 경로는 **이 함수 구조상 도달 불가**(상위가 막는 게 아니다) | reused |
| E2 | `/specs[2]/open[1]` | 분류오류 | 같은 파일 `history[1]` 이 이미 닫은 물음이 `open` 에 남아 있었다(과열림) | reused |
| E3 | `/specs[3]/consts[0]` | 실오류 | `src_line` ~~21~~ → **22** | reused |
| E4 | `/specs[3]/consts[1]` | 실오류 | `src_line` ~~42~~ → **43** | reused |
| E5 | `/specs[3]/consts[2]` | 실오류 | `src_line` ~~42~~ → **44** | reused |
| E6 | `/specs[3]/consts[3]` | 실오류 | `src_line` ~~42~~ → **45** | reused |
| E7 | `/specs[3]/knobs[3]` | 실오류 | `where` 의 `buff_value.rs:42` → **44** | reused |
| E8 | `/specs[3]/knobs[4]` | 실오류 | `where` 의 `buff_value.rs:42` → **45** | reused |
| E9 | `/specs[3]/logic` ×2 | 실오류 | `[L21]`→`[L22]`, `[L42]`→`[L43~45]` + `ty==13` 의 실제 소재 | reused |
| E10 | `/specs[3]/open[0]` | 분류오류 | 잔여 물음(`rnd`/`debug` 가 변하나)을 **실측으로 닫았다** — 안 변한다 | inherited |
| E11 | `/specs[4]/open[0]` | 보강 | `PlayerState::strategy` 가 `rnd` 를 소비하는지 **직접 재서** 답을 붙였다 | inherited |
| E12 | `/specs[3]/notes[0]` | 보강 | 별도 `define` 3개 중 **m01.ll 이터레이터 조각이 `aux` 에 빠져 있다** | reused |

### 각 정정의 근거

#### E1 `/specs[0]/open[0]` — **`sz==0` div-by-zero 는 죽은 가드다** (판정반전)

기존 문장: 「…334행에서 div-by-zero 패닉 경로가 **실제로 존재한다** — 게임이 이 상황을 상위에서 막는지는 이 범위에서 확인 불가」.
**상위가 막는 게 아니라 이 함수 자신이 막는다.** 4중 근거:

1. `Effect::is_in_range`(`_gcbc/g06.ll:51634~51781`) = `dist_sq <= total^2` 이고 `total` 은 **비음수 항의 합뿐**이다
   (`effect.range` + `caster_r` + `stat_buff_cached.range` + `(level−1)*growth_range` + `range_adjust` + `target.radius()`).
   **하한 사거리가 없다** ⟹ 거리 0 이면 `0 <= total²` 로 **항상 in-range**.
2. `get_input_target` 이 `None` 을 내는 곳은 정확히 3곳(`_gaibc/m04.ll` 블록43 champ null / 블록45 `!is_in_range` / 블록70 `!visible && !is_nontarget`).
   거리 0 에서는 1.때문에 블록45 가 못 뜨고, 블록43 은 `ult` 가 L281 에서 이미 champ 를 얻은 뒤라 못 뜬다.
3. 남는 블록70 은 `!visible` 을 **전제**한다. 그런데 `ult` 는 L328 에서 같은 `visible` 을 다시 보고 `!visible` 이면 **L340 `safe_move_avoiding_enemy_well`** 로 나간다
   ⟹ L329~L335(`isqrt` · `/ sz`)에 **올 수 없다**.
4. 오라클 실행 확증 — champ 와 target 좌표를 완전히 같게 두고 `casting` 4값 × `eff_range {0, 200000}` = **8/8 전부 `Ult(..)` 반환, `Move` 0건**
   (`A6_o2_samepos.tsv`: `Ult(Target{23})` / `Ult(Pos{15000,913000})` / `Ult(Dir{0,0})` / `Ult(None)`).

⟹ IR 의 `panic_const_div_by_zero` 는 LLVM 이 남긴 **죽은 가드**이고 **재현할 필요가 없다.**
★범위: `ult` 안에서의 판정이다. `get_input_target` 이 다른 경로로 `None` 을 내게 바뀌면 다시 봐야 한다.

#### E3~E9 `/specs[3]` — `src_line` 오기 4건 + 그 파생 4건

`defensive_crisis`(`m10.ll:33864~34294`)의 모든 명령에 대해 `!dbg` 의 **inlinedAt 사슬 전체**를 펼쳐
`buff_value.rs` 프레임만 모으면 **실제로 등장하는 줄 = {17, 19, 21, 22, 28, 30, 32, 33, 34, 35, 41, 43, 44, 45, 47, 48, 53}** 이다.
**L42 는 이 함수 어디에도 명령이 없다.** 그런데 `consts[1]`·`[2]`·`[3]` 이 전부 `src_line: 42` 였다.

| 행 | IR | 사슬 | 결론 |
|---|---|---|---|
| `consts[0]` (값 1) | `m10.ll:33897` `%21 = sub i64 1, %20` | `buff_value.rs:22` (인라인 없음) | ~~21~~ → **22**. L21 에는 `llvm.lifetime.start` 하나뿐 |
| `consts[1]` (값 13) | `m10.ll:34074` `icmp eq i64 %72, 13` | `entity.rs:1775` ← `buff_value.rs:43` ← `41` | ~~42~~ → **43** |
| `consts[2]` (값 2) | `m10.ll:34081` `icmp ugt i64 %76, 2` | `entity.rs:1693` ← `buff_value.rs:44` ← `41` | ~~42~~ → **44** |
| `consts[3]` (값 4) | `m10.ll:34086` `icmp ugt i64 %76, 4` | `entity.rs:1701` ← `buff_value.rs:45` ← `41` | ~~42~~ → **45** |

덤으로 **구조도 한 겹 틀렸다**: `logic` 은 `[L42] (c1,c2,c3) = if e.ty == 13 { … } else { (0,0,0) }` 로 적어 두었지만,
`ty == 13` 비교는 소스에 직접 쓰여 있는 게 아니라 **`Entity::skill_cooldown`/`skill2_cooldown`/`ult_cooldown`
(entity.rs:1775 / 1791 / 1806) 각각의 안**에 있고, LLVM 이 CSE 해서 **L43 하나로** 접은 것이다.
슬롯 3칸은 `L43`(skill) / `L44`(skill2) / `L45`(ult) 에 **한 줄씩** 있다
(`m10.ll:34109` +184=0xb8 @L43 · `34119` +192=0xc0 @L44 · `34127` +200=0xc8 @L45).
**동작 서술은 맞다** — 바뀌는 건 줄 번호와 "그 비교가 어디 있는가"다.

⚠**이 오류가 5라운드를 살아남은 이유**: `qcspec`·`specgate` 어디에도 **`src_line` 을 대조하는 검사가 없다.**
`consts[].value` 는 C1 이 본문과 대조하지만 `src_line` 은 아무도 안 본다. 도구를 하나 만들어 두었다 →
`_verify6\A\srclinecheck.py`(전 배치 대조용) · `dbgchain2.py`(사슬 펼치기) · `atline.py`(특정 줄의 명령 열거).

⚠**단 `srclinecheck.py` 의 「불일치」를 그대로 믿지 마라** — 나도 처음엔 `specs[1]` 에서 7건, `specs[0]` 에서 2건이
불일치로 나왔는데 **전부 오탐**이었다: ①`switch` 는 **케이스 값 전부가 한 명령**이라 모든 태그가 같은 줄(`action_score.rs:526`)을 갖는다.
②`phi` 는 `!dbg` 가 아예 없거나 `line: 0` 이다(`specs[1]` 의 200/80/−10, `specs[2]` 의 5/16). 손으로 사슬을 펴서 하나씩 확인했다.
`specs[0]` 의 `consts[3]`·`[4]`(`L293`)는 `switch i32 %43` 의 `!56416 = abstract_input.rs:293` 으로 **정확히 맞았다**.

#### E10 / E11 — `&mut` 인자가 정말 안 변하나 (수법 ⓒ 바이트 diff)

* `specs[3]/open[0]` 의 잔여 물음 「`rnd`·`debug` 는 `check_kill_die_tick` 에서 변경될 수 있다」
  → `defensive_crisis` 호출 **전후로 `StdRng` 320B · `DebugFrameData` 224B 를 통째로 바이트 비교**한 결과
  **14/14 케이스 전부 `rnd_changed=false · debug_changed=false`**(`A6_o5.tsv`). **해소.**
  ★범위: `check_kill_die_tick` 이 실제로 실행된 케이스만 잰 값이다.
* `specs[4]/open[0]` 의 잔여 물음 「`rnd` 가 `PlayerState::strategy` 안에서 소비될 수 있다(그 함수는 안 봄)」
  → `player.strategy(&mut rnd, &game)` 을 **직접 호출**해 전후 320B 를 비교 = **안 바뀐다**(`A6_o4.tsv`).
  ★범위: 상태 1종에서만 잰 값이라 「난수를 절대 안 쓴다」로 일반화하지 말 것(`_shared.TeamColorStrategy_random` 경로가 있다). ⟹ **보강으로만 달았고 `open` 은 열어 두었다.**

#### E12 — `defensive_crisis` 의 세 번째 조각이 `aux` 에 없다

`fnparts.py defensive_crisis` = 조각 3개: **본체 `m10.ll:33864~34294`** · **`call_mut` 심 `m10.ll:55868~55976`** · **이터레이터 `m01.ll:37431~37606`(175줄)**.
그런데 `history[0]` 의 `aux` 에는 `m10.ll:55868~55976` **하나만** 올라가 있다. `m01.ll` 조각은 **아무 라운드도 읽지 않았다**(미탐색).
(`notes[0]` 의 「별도 define 은 3개뿐」이라는 서술 자체는 **반증 실패 = 유지**. 도구 출력과 정확히 일치한다.)

### `ev_up` 158행

| 스펙 | `mem`(→ev3) | `consts`(→ev2) | `knobs`(→ev2) | 계 |
|---|---|---|---|---|
| `00 ult` | 24 | 9 | 4 | 37 |
| `01 calculate_jungle_action_score` | 8 | 9 | 6 | 23 |
| `02 sub_plan` | 21 | 5 | 4 | 30 |
| `03 defensive_crisis` | 22 | 4 | 6 | 32 |
| `04 handle_line_defense` | 26 | 5 | 5 | 36 |
| **계** | **101** | **32** | **25** | **158** |

적용 시 배치 A 의 등급 분포: `{4:183, 3:8, 2:3, 5:2}` → `{3:104, 2:60, 4:32}` (**ev≥4 185행 → 32행, 94.4% → 16.3%**).
남는 `ev≥4` 32행은 실행으로 못 가른 것들이다 — `dyn EffectType` vtable 슬롯 3행, `can_ult` 의 CC 집합,
`cooltime_use_count`, `approach_dodge_steer`, `Minion` 계수(미니언이 안 스폰돼 태그 1 경로를 못 탐), 팀 bounds-check 상수 등.

#### 근거가 된 실행 산출물

| 프로브 | 무엇 | 결과 |
|---|---|---|
| `A6_o1.rs` → `A6_o1.tsv` | `mem` 전행 오프셋 + 크기·stride + 신규 4종 | **81/81 MATCH · MISMATCH 0** |
| `A6_o6.rs` → `A6_o6.tsv` | `o1` 이 못 덮은 열거형 페이로드·팻포인터·PlayerState 오프셋 | **10/10 MATCH** |
| `A6_o2.rs` → `A6_o2_main/_radius/_samepos.tsv` | `specs[0] ult` (케이스당 1프로세스) | level/casting/margin/radius_mult/same_pos |
| `A6_o3.rs` → `A6_o3.tsv` | `specs[1]` 계수표 20행 + `specs[4]` 태그·튜토리얼·version 스윕 | 전건 `game==mine MATCH` |
| `A6_o4.rs` → `A6_o4.tsv` | `specs[2]` 3분기 + sret 바이트 + version 0..8 | 3분기 전수, v0..8 동일 |
| `A6_o5.rs` → `A6_o5.tsv` · `A6_o5_tps.tsv` | `specs[3] defensive_crisis` 14케이스 + tps 5값 | cc_threat 축 전수 |

전 프로브가 `real_setting()` + `setting_ok()=true`(`width/height=960000 tps=60 champ_radius=10000 visible=130000`)이고
`init_tower`/`init_nexus` 를 부르지 않는다(TEMPLATE ②).

### 새 발견 6건 (정정은 아니지만 명세에 없던 사실)

1. ★**bumpalo `Vec` 의 `len` 은 `+0x18` 이 맞다 — 처음으로 실행으로 갈랐다.**
   5차 배치A 는 `twin_towers` 의 `len==cap==2` 라 `+0x10`/`+0x18` 을 못 갈라 「범위 한정: `len != cap` 인 Vec 이 필요」로 닫았다.
   이번에 `Vec::with_capacity_in(8)` + `push` 3회로 만들었더니 `words = [ptr, bump, 8, 3]` ⟹ **`cap@+0x10` · `len@+0x18` 확정**
   (`A6_o1.tsv BUMPVEC`). 이 한 줄이 `specs[2]/mem[6]`·`specs[3]/mem[11]`·`specs[4]/mem[17]` 세 행을 동시에 받친다.
2. ★**`Entity::radius()` 의 `as usize` 는 진짜 부호확장이다 — 음수 `radius_mult` 로 판별했다.**
   5차는 `radius_mult=0` 이라 `sext`/`zext` 가 **비판별**이었다. `mult = −50` 을 넣으니 `radius() = 5000`
   (= `10000*(100−50)/100`)이고, 영확장 가설의 예측값은 `429,496,734,600` 이다 ⟹ **sext 확정**.
   동시에 기준값 `100`(대립가설 `1000` 은 `9500` 을 예측)도 갈렸다. `mult = −100` 이면 **반경이 0 이 된다**(`A6_o2_radius.tsv` 6/6).
3. ★**`die_tick` 의 계수 `60` 은 `tps` 가 아니라 하드코딩 리터럴이다.**
   `tick_per_second` 를 `1 / 6 / 30 / 60 / 120` 으로 바꿔도 `check_kill_die_tick` 반환이 **전부 `die=60` 고정**(`A6_o5_tps.tsv`).
   ⟹ `specs[3]/knobs[6]` 의 「전 사망예측 공통 배율」은 설정으로 못 바꾸는 값이고 **바이트패치/재구현 전용 노브**다.
4. ★**`die_imminent` 의 부등호가 `<` 인 것을 경계에서 갈랐다.** 같은 스윕에서 `tps=30`(즉 `die 60 < tps*2 60`)이
   **`die_imminent = 0`** 이다 ⟹ `<=` 가 아니라 **strict `<`**. `tps=60`(60<120)·`tps=120`(60<240)은 1, `tps=1/6/30`은 0 으로 정확히 갈린다.
5. **`cool <= tps` 창의 임계가 `tps` 를 따라간다.** 5차는 `tps=60` 고정으로 `cool 60 통과 / 61 탈락`만 봤다.
   이번엔 `tps=30` 에서 `cool 30 통과 / 31 탈락` ⟹ 상수 60 이 아니라 **`tps` 자체가 창**임을 실행으로 확정(`specs[3]/knobs[1]`).
6. **`AttackNexusPlan`(16B) 의 필드 배치를 실측했다.** 5차는 「필드가 `in:module` 사유라 `offset_of!` 불가」로 오프셋을 못 쟀다.
   `AttackNexusPlan::new(1, Mid)` → `[1, 1]`, `new(0, Bottom)` → `[0, 2]` ⟹ **`team@+0x0` · `line@+0x8`**.
   덤으로 `derive(Debug)` 가 `AttackNexusPlan { team: 1, line: Mid }` 로 **private 필드를 그대로 찍는다**(수법 ⓑ).

### 반증 시도했으나 **명세가 버틴 것** (= 반증 실패, 명세 유지)

| 대상 | 시도 | 결과 |
|---|---|---|
| `specs[4]/notes[0]` 「`_version`·`_debug` 는 전혀 안 쓰인다」 | `version 0..=8` × 라인 3종 = 27회 호출 + `DebugFrameData` 224B 바이트 diff | 27/27 동일 · `debug_bytes_changed = 0/27` ⟹ **유지** |
| `specs[2]/sig.params[1]` 「version 분기 없음」 | `sub_plan` 을 `version 0..=8` 로 × 케이스 4종 | 전부 동일(tag·페이로드 바이트까지) ⟹ **유지** |
| `specs[2]/notes[0]` 「`DW_OP_not` 이중은 아티팩트」 | `no_twin`(`twin1_len=0`) → `AttackNexus`, `len=2` → `LineDefense` | 분기 방향이 `has_enemy_twin_tower = !is_empty()` 와 일치 ⟹ **유지** |
| `specs[3]/notes[0]` 「별도 define 3개뿐」 | `fnparts.py defensive_crisis` 재실행 | 조각 정확히 3개 ⟹ **유지**(다만 E12 의 `aux` 누락 보강) |
| `specs[0]/knobs[7]` 「우물회피 임계 4,000,000 @ L134」 | `m04.ll:43950` `icmp ugt i64 %310, 4000000` 의 사슬 | `abstract_input.rs:134` 단일 프레임 ⟹ **유지** |
| `specs[1]` 계수표 전체 | 20행 재실행 + `hp` 스윕 7점 | `game == mine` 20/20, `hp=496` 경계에서 `hp<value` 대립가설 기각 ⟹ **유지** |
| `specs[1]/knobs[6]` 「양 팀 캠프 전부」 | `camp0=0`(블루) 6개 · `camp0=1`(레드) 6개 | **둘 다 `based=5`** ⟹ **유지** |

---

## ③ `ev<=3` 표본 재확인

현재 배치 A 의 `ev<=3` 은 13행(`ev2` 3 · `ev3` 8 · `ev5` 2)이다. 이번에 **11행을 다시 쟀다.**

| 행 | 현 등급 | 재확인 | 판정 |
|---|---|---|---|
| `/specs[0]/mem[11]` `Entity+0x470 radius_mult` | 3 | `A6_o1` offset MATCH + `A6_o2` 음수 mult 로 **sext 실행 확정** | 유지(오프셋은 ev3 상한) |
| `/specs[0]/mem[27]` `Option<Input>+0x8 payload` | 3 | `A6_o1 OPTINPUT payload@+0x8 = Target{target_id:7}` 왕복 | 유지 |
| `/specs[0]/consts[8]` `Input::Ult 태그 5` | 3 | `A6_o1` `Some(Input::Ult) tag@+0x0 = 5` | **ev2 로 상향 제출** |
| `/specs[0]/consts[6]` `VisibleState::Visible = 0` | **5** | `A6_o2 TAG VisibleState Visible=0 / Unknown=2` | **ev2 로 상향 제출**(ev5 는 오탐, ⑤ 참조) |
| `/specs[1]/consts[6]`·`[7]` `20/40` arm | 3 | `A6_o3 HPSWEEP` `hp=496` 에서 40 채택, 대립가설 25 기각 | **ev2 로 상향 제출** |
| `/specs[1]/knobs[6]` 정글 캠프 팀 게이트 | 3 | `A6_o3` `camp0=0`·`camp0=1` 둘 다 `based=5` | **ev2 로 상향 제출** |
| `/specs[2]/consts[1]` `SubPlan::Recall 태그 5` | 3 | `A6_o4` `in_heal_low → tag=5 / Discriminant(3)` | **ev2 로 상향 제출** |
| `/specs[3]/knobs[0]` `die < tps*2` | 2 | `A6_o5_tps` 5점 스윕으로 **경계(tps=30)까지** 재확인 | 유지(근거 강화) |
| `/specs[3]/knobs[2]` 반경 여유분 30000 | 2 | `A6_o5 SETUP` `max_range=120000`·`(mr+30000)^2=22,500,000,000`·`filter_dist_ok=true` | 유지 |
| `/specs[3]/knobs[7]` judgement 파이프라인 | 2 | `A6_o6 PS+0x218=80 · PS+0x450=1000` | 유지 |
| `/specs[3]/notes[0]` 조각 3개 | 3 | `fnparts` 재실행 | 유지 + E12 보강 |
| `/specs[3]/knobs[19]` judgement 원본 | **5** | `A6_o6 PS+0x218=80`(`mkgame` 이 넣은 값) | **ev2 로 상향 제출**(ev5 는 오탐) |
| `/specs[4]/mem[7]` `AbstractGame vtable +0x40` | 3 | ⚠**이번 라운드 재측정 안 함**(5차 `A5_o7.tsv VT … get_game_mode CONFIRMED` 그대로 인용) | **미검증** |

---

## ④ 게이트 실측

```
$ PYTHONIOENCODING=utf-8 python -X utf8 specgate.py
G1 자기모순=0  G10 class 오분류=0  G11 ev 지시 미반영=2  G2 호출부 전수=0  G3 형제 함수=0
G4 술어 시그니처=0  G5 sig 정본 대조=0  G6 logic 미반영=0  G7 과열림=0  G8 표에 옛 값=0  G9 callees 오염=4
```
* **G11 = 2 는 전부 배치 A 것이고, 내 `patch.json` 이 정확히 그 5행을 덮는다**:
  `[03] consts[2]·consts[3]·knobs[1]`(history[9] 지시) · `[04] knobs[0]·consts[2]`(history[5] 지시).
  다섯 행 모두 `ev_up` 에 들어 있고 `applypatch` 가 붙이는 태그(`· 오라클 실행 확증(6차 배치A: …)`)가
  `mkspec3.evtier` 의 tier2 키워드에 걸리므로 **적용 즉시 G11 = 0 이 된다.**
  ⚠나는 `_spec` 를 못 고치므로(금지) 이 게이트는 **패치 적용 전에는 0 이 될 수 없다.**
* **G9 = 4 는 전부 배치 B/C/D 몫**(`07`·`13`·`14`·`15`)이고 등급은 `손확인`이다 — 배치 A 는 0.

```
$ PYTHONIOENCODING=utf-8 python -X utf8 auditrounds.py
커버리지 결손 11 · 유실 0 · STALE 0 · ev되돌아감 0
  ### ★커버리지 — 보고서는 있는데 patch.json 이 없는 배치 11건  (3차 A~D · 4차 A~D · 5차 A~C)
  ### 반영 대기 — patch.json 은 냈는데 아직 안 붙인 배치 4건  (5차 D · 6차 A · 6차 B · 6차 C)
```
⟹ **불변 기준 `유실 0 · STALE 0 · ev되돌아감 0` 통과.**
* `커버리지 결손 11` 은 **3~5차가 산문 보고서만 냈고 `patch.json` 이 없다**는 지적으로, 이번 라운드의 회귀가 아니다.
  ★ 그중 `[5차 A]` 가 내 앞 라운드다 — ⑤-5 에 적은 「146행 전량 유실」의 기계적 근거가 바로 이 줄이다.
* `[6차 A]` 는 **반영 대기**로 정확히 분류됐다(`applypatch.py 6 --only A` 를 돌리면 된다).
  ⚠나는 `_spec` 를 못 고치므로(금지) 적용은 오케스트레이터 몫이다.
* ⚠**세션 도중에 `auditrounds.py` 가 갱신됐다.** 착수 직후 실측은 `대조 128건 · 유실 0 · STALE 0 · ev되돌아감 0`(6차 patch.json 이 하나도 없던 시점),
  중간 실측은 `대조 520건 · 유실 16 · STALE 14 · ev되돌아감 371`(미적용 패치를 유실로 셌다), 최종은 위 표다.
  **셋 다 「1~5차 회귀 0」이라는 점에서는 같다.**

```
$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose _verify6/A/A6_REPORT.md
총 690건  오귀속=0  밀림=0  부분일치=3  확인불가=18  OK=669
```
⟹ **불변 게이트 통과: 오귀속 0 · 밀림 0.**
* `부분일치 3건` 은 전부 **팻포인터 뒤 워드**다 — `_spec/specs20.json` 산문의 `cache+0x8`, 그리고 그것을 인용한 내 보고서 문장 2곳(`cache+0x8`·`Effect+0x8`).
  `tcx` 는 `&dyn Trait` 를 `game`(16B) 한 필드로만 보므로 **뒤 절반은 원래 못 뚫는다**(`METHOD_MAP §1 ⑦ 한계`). 오귀속이 아니다.
  이 계열은 `A6_o6.tsv EFFECT_ARC` 에서 **실행으로** 확인해 두었다(`Effect+0x8` 의 값 == `&*ef.ty` 팻포인터의 vtable 워드).
* `확인불가 18건` 은 전부 **vtable 슬롯**(`dyn EffectType +0x10/+0xf8/+0x118`, `AbstractGame +0x40` 등)으로 `tcx` 소관 밖이다.

**세 게이트 종합 판정: `오귀속 0 · 밀림 0` 통과 · `auditrounds` 의 `유실 0 · STALE 0 · ev되돌아감 0` 통과 ·
`specgate` 는 `G11=2` 만 남고 그 2건(5행)을 내 `patch.json` 이 그대로 덮는다(적용 즉시 0).**

---

## ⑤ 브리핑 · `BRIEF_FACTS` 의 오류

1. **`BRIEF.md §2` 의 「`ev>=4` 가 아직 669행」이 생성물과 어긋난다.**
   같은 라운드의 `BRIEF_FACTS.md §1` 은 계 858행 · `ev≥4` **91.4%** = **784행**이다(내가 직접 세도 `mem+consts+knobs` = 858, `ev≥4` = 784).
   669 는 어느 스코프로도 안 나온다. 브리핑 §0 이 스스로 경계한 「기억으로 사실을 쓴 것」 유형이 **산문 절에 하나 남아 있다.**
2. **`BRIEF_FACTS.md §3` 의 분모가 §1 과 다르다.** 제목은 「`open` 11건 · `notes` 8건」(20함수 전체)인데 지시문은 배치 A 에게 「`open` 5건 · `notes` 3건」(내 몫)이라 말한다.
   둘 다 맞지만 **§1 처럼 스코프 한 줄을 §3 에도 박아야** 5차의 「분모 296행」 사고가 재발하지 않는다. (내 몫을 직접 세니 `open` 5 · `notes` 3 으로 일치했다.)
3. ★**산출물 계약 자체의 결함 — `patch.json` 으로는 정수 필드를 못 고친다.**
   `applypatch.apply_error` 는 `isinstance(cur, str)` 인 값만 치환한다. 그래서 이번 최대 수확인
   **`consts[].src_line` 오기 4건이 구조로는 반영이 안 된다.** 나는 `meaning` 문면에 `~~42~~ → **43**` 형태로 실어 두었지만,
   **정수 갱신은 별도 반영이 필요하다**: `/specs[3]/consts[0].src_line 21→22` · `[1] 42→43` · `[2] 42→44` · `[3] 42→45`.
   (제안: `apply_error` 에 `"new_int"` 같은 키를 두거나, `int` 값이면 `old` 를 문자열화해 비교하도록 고칠 것.)
4. ★**도구 결함 — `mkspec3.evtier` 의 `ev5` 판정이 오탐을 낸다.**
   근거 문면에 「추정」·「보인다」가 **어디에** 있든 `ev5`(근거 없음)로 떨어뜨린다. 그런데 배치 A 의 `ev5` 2건은 **둘 다 오탐**이었다:
   `/specs[0]/consts[6]` 은 「태그가 0이면 '**보인다**'」(가시성을 서술하는 동사), `/specs[3]/knobs[19]` 는 「위협 **추정**이 정확해진다」(추정치라는 명사).
   ⟹ 불확실성 어휘는 **문장 끝 서술어 위치**에서만 잡도록 좁혀야 한다. 지금은 「이 필드는 '보인다'를 뜻한다」라고 정확히 쓸수록 등급이 내려간다.
5. **5차 배치 A 의 `ev` 상향 146행은 전량 유실됐다.**
   `_verify5/A/A5_REPORT.md §①` 은 「`ev4→ev2` 상향 146행 … 배치 A `ev≥4` 94.4% → 24.0%」라고 적었는데,
   6차 착수 시점 실측이 **94.4%(185/196)** 로 **5차 착수값과 완전히 동일**하다 ⟹ 146행이 하나도 반영되지 않았다.
   (반면 5차의 산문 정정 E1 `caster_r` 은 `logic`·`history[7]` 에 **제대로 들어가 있다**.) 계약을 만든 판단은 맞았다.

---

## ⑥ `found_by` 집계 — **이번 라운드의 결론**

| 값 | 건수 | 내역 |
|---|---|---|
| `reused` | **11** | E1(IR 콜리 독해 + 같은 좌표 오라클) · E2(같은 파일 `history` 대조) · E3~E9(`!dbg` inlinedAt 사슬) · E12(`fnparts`) |
| `inherited` | **2** | E10 · E11 — 둘 다 **ⓒ 구조체 통째 바이트 diff**(5차 배치C 발명)를 `&mut` 인자에 처음 적용 |
| `new` | **0** | — |

### 이게 뜻하는 것

브리핑의 예측은 「`reused` 실오류가 ≈ 0 이면 내 주장이 맞다 / 여러 건이면 틀렸다」였다.
**결과는 `reused` 11건이다. 배치 A 범위에서 그 주장은 성립하지 않았다.**

특히 **E3~E9(`src_line` 오기 7건)는 새 계측기가 만든 게 아니다.** `!dbg` 의 `inlinedAt` 사슬을 끝까지 타는 방법은
`SPEC_GUIDE` 에 문서화돼 있고 도구도 이미 두 개(`dbgchain.py`·`dloc.py`) 있었으며, 배치 A 는 **3차에 이미 그 방법을 썼다.**
5차에 안 쓴 것뿐이다. ⟹ 이 오류들은 **계측기가 없어서 안 보인 게 아니라, 있는 계측기를 그 축에 겨눈 적이 없어서** 살아남았다.

★그래서 이번 라운드가 실제로 가리키는 결론은 「명세가 불안정하다」도 「계측기가 오류를 만든다」도 아니고 **이것**이다:

> **`src_line` 은 5라운드 동안 어떤 게이트도 검사하지 않는 축이었다.**
> `qcspec` C1 은 `consts[].value` 를 본문과 대조하지만 `src_line` 은 보지 않고, `specgate` G1~G11 어디에도 줄 번호 검사가 없다.
> 검사받지 않는 축에서는 **라운드를 아무리 돌려도 오류가 줄지 않는다** — 실제로 이 4건은 1차부터 그대로였다.

⟹ **권고: 라운드를 더 도는 것보다 `srclinecheck.py` 를 `specgate` 에 `G12` 로 넣어라.**
그러면 20함수 전체에서 같은 계열 오류가 한 번에 드러나고, 다음 라운드의 `reused` 는 진짜로 0 에 수렴할 수 있다.
(내 도구는 `switch`/`phi` 때문에 오탐이 섞이므로, **「주장한 줄이 그 함수의 `!dbg` 줄 집합에 아예 없을 때만」** 경고하도록
좁히면 오탐 0 이 된다 — `specs[3]` 의 `L42` 가 정확히 그 경우였고, `specs[1]`·`specs[2]` 의 오탐 9건은 전부 걸러진다.)

부수적으로 `inherited` 2건은 브리핑 예측대로 **계측기 확산**의 결과였다(ⓒ 를 `&mut` 인자에 적용).
`new` 가 0 인 것도 의미가 있다 — **이번에 배치 A 는 새 계측기를 하나도 발명하지 않았고, 그래도 실오류가 9건 나왔다.**

---

## ⑦ 부록 — 산출물 목록

| 파일 | 내용 |
|---|---|
| `patch.json` | ★산출물 계약(정정 13 · ev 상향 158 · 브리핑 오류 4) |
| `mkpatch.py` | 위 파일 생성기 |
| `A6_o1.rs` / `.tsv` | `mem` 오프셋 일괄 대조 81/81 |
| `A6_o2.rs` / `A6_o2_main.tsv` · `A6_o2_radius.tsv` · `A6_o2_samepos.tsv` | `specs[0] ult` |
| `A6_o3.rs` / `.tsv` | `specs[1]` 계수표 · `specs[4]` 태그/튜토리얼/version |
| `A6_o4.rs` / `.tsv` | `specs[2]` 3분기 · `strategy` rnd diff |
| `A6_o5.rs` / `.tsv` · `A6_o5_tps.tsv` | `specs[3]` cc_threat 축 · tps 스윕 |
| `A6_o6.rs` / `.tsv` | 열거형 페이로드·팻포인터·PlayerState 오프셋 10/10 |
| `srclinecheck.py` | ★`consts[].src_line` 기계 대조(전 배치용, `G12` 후보) |
| `dbglines.py` · `dbgchain2.py` · `atline.py` | `!dbg` 사슬 펼치기 / 특정 줄의 명령 열거 |
| `srcline.txt` | `srclinecheck.py` 출력 원본 |
| `_dump04.txt` | `specs[0]~[4]` 정본 덤프(작업용) |
| `tmpl.rs` | `real_setting`/`setting_ok`/`mkgame` (TEMPLATE 모듈판) |
