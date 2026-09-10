# 검증 리포트 — 배치 A (00~04)

> 2026-09-11 / 게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24`
> 대상 = `_verify\00..04.json` · 정본 반영처 = `_spec\specs20.json`
> §11 원문 = `REPORT\tfm2_ai_adjust\RE\2026-09-11_20함수-반증검증-4갈래-정정13건.md`

⚠**먼저**: `_verify\REPORT_A.md` 는 **쓰지 못했다** — 이 하네스가 서브에이전트의 보고서 .md 작성을 차단한다("Include this content in your final response instead"). 보조 산출물은 규칙대로 `A_` 접두로 디스크에 있다:

```
_tcx\mirdump_game_core.txt      403MB  ★game_core MIR 전량(17,575 바디) — 이번에 처음 뽑음
_verify\A_mirget.py                919B  위 덤프에서 함수 본문 추출
_verify\A_probe.rs                2.9KB  SDK 실행 오라클 프로브
_verify\A_probe.tsv                920B  그 실행 출력
```

## 0. 판정 한 줄

| # | 함수 | 판정 | 요지 |
|---|---|---|---|
| 00 | `abstract_input::ult` | **✅확인 + ➕보강** | IR 전량 재대조 — 상수·분기극성·오프셋 오류 **없음**. 사소한 내부 stale 1건 |
| 01 | `calculate_jungle_action_score` | **⚠정정(중대)** | `is_jungle`/`is_minion` **시그니처·술어 이름이 틀렸다**. 「camp_type.0 < 2 = 정글 티어 컷」 노브는 **존재하지 않는다** |
| 02 | `AttackNexusPlan::sub_plan` | **✅확인 + ➕보강** | 오류 **없음**. `unknown` 5건 중 4건을 닫았다 |
| 03 | `defensive_crisis` | **⚠정정 + 🔁범위정정** | 본문은 전부 맞다. `new_knobs` 의 **`judge_accuracy(PlayerState+0x180)` 오귀속** + 치역 오류 |
| 04 | `handle_line_defense` | **⚠정정 + 🔁범위정정** | `morgard_exists` 가 (b)(c)를 포함한다는 **구조 서술이 틀렸다**. 「표기 확정 불가」·「{0,7,8} 은 추정」은 **과한 판정** |

**★교차 발견(내 5개 밖·전 배치 영향)**: `METHOD_MAP §4 함정1` 과 `_oracle\README §2` 의 **「LineType 선언 순서 = Mid,Bottom,Top」 / 「ObjectPhase 선언 순서 = Setup,Assemble,Hunt,None」 은 둘 다 거짓**이다(§7). 이번 브리핑에도 그대로 실려 있었다.

## 1. 기계 검증 (5건 공통)

`tcxaudit.py --prose _verify/00..04.json` → **698건 중 오귀속 2 · 밀림 0 · 부분일치 1 · 확인불가 18(전부 vtable) · OK 677**

- 오귀속 2건은 **내 담당 파일이 아니다** — `specs20.json`/`resolved_*.json` 의 `~~PlayerState+0x2496~~` 취소선을 도구가 집은 **오탐**(브리핑 예고분). 이미 정정 표기된 옛 값.
- 내 5개에서 나온 지적은 전부 「이름 미기재(오프셋만 검증)」 수준, 값은 맞다.
- `tcxdict` 크기 대조 — **오프셋이 타입 크기를 넘는 주장 0건** (Entity 1728 / PlayerState 2528 / MapDef 28112 / AbstractGameWithCache 8840 / MobaMode 640 / GameSetting 5432 / Effect 56 / Strategy 24 / Blackboard 744, 오라클 `size_of` 로도 재확인).
- `tcxdict --enum` 재확인: SubPlan(Niche; LineDefense2/Recall5/AttackNexus16) · EntityType(0..13) · Input(Ult=5, 페이로드+0x8) · LineType(Top0/Mid1/Bottom2) · CastingType(0..3) · MinionActionType(Push2) · LineStyle(Aggressive0) · MorgardDefenseStrategy(Gather0/Battle1) · TutorialType(0..8) · Position(Top0..Support4) · VisibleState(Visible0) — **어긋난 태그 0건**.

## 2. `00_ult` — ✅확인(오류 없음) + ➕보강 2

IR `_gaibc/m04.ll:43967~44365` 전량 재독해. **판정 상수·분기 극성·오프셋 전부 일치.**

확인: `level>4 → ult_effect(+0x538)`(`%29=champ+1480`, `icmp ugt %30,4`) · `Option<Effect>` 니치 `Effect+0x30==-1` · `switch casting` **case 1/2 만, default→GET_INPUT_TARGET**(`%43` switch default `%95`) · Position(1)→vtable **+0x118**(`%47+280`) `on_caster()` / Direction(2)→vtable **+0xf8**(`%60+248`) `.0==1`=`is_some()` · 사거리 6항 누산 + `usub.sat(total,150000)`(`%201~%206`) · **가시성 분기 극성 명세대로**(champ.team 태그1 Neutral→접근계산 / Player(0)→`target.visible_state[team](+0x38, stride24)` 태그`==0`→접근계산, 아니면 `safe_move(target.x,target.y)`).
독립 교차검증: `Entity::is_visible_from` MIR(entity.rs:1481) `switchInt(other.team)->[0:Player, 1:const true]` + `self.visible_state[team] discr==0` ✓

**➕보강**
1. **`Entity::radius` 의 `radius_mult` 는 `i32 → usize` 부호확장**이다(MIR `_2 = _3 as usize (IntToInt)`, IR `sext i32 %162 to i64`). ⟹ **음수 `radius_mult` 는 0 이 아니라 거대한 usize 로 접혀 반경이 오버플로**한다. 재현 시 `as usize` 를 빼면 동작이 갈린다.
2. `Position::as_index` MIR(entity.rs:580~581) = `discriminant as usize` — **재배치 없음**. (00·01·02 세 명세가 공통으로 "미확정"으로 남긴 항목)

⚠사소: `00.resolved` 의 *"can_move … 전 구현 117개 중 true 8개"* 는 `_shared.can_move_구현체`("134개 / true 9개, 이전 8개 정정")와 모순 — **00 쪽이 stale**.

## 3. `01_calculate_jungle_action_score` — ⚠정정 (중대 2 + 파생 3)

### 3‑1 ⚠ `EntityType::is_jungle` 은 **인자를 받는다**. 「camp_type.0 < 2」는 소스에 없는 상수다

tcx 시그니처(`_tcx/game_core.json`):
```
pub EntityType::is_jungle(&self, usize) -> bool         entity.rs:1377  mir=True
pub EntityType::is_minion(&self, LineType) -> bool      entity.rs:1256  mir=True
pub EntityType::is_any_type_minion(&self) -> bool       entity.rs:1260  mir=True
pub EntityType::is_any_jungle(&self) -> bool            entity.rs:1381  mir=True
```
MIR 본문(`_tcx\mirdump_game_core.txt`):
```
_7 = copy (((((*_1) as Jungle).0: Jungle).2: (usize, JungleType)).0: usize)
_6 = Eq(move _7, copy _2)     ← 인자와의 동등 비교. 상수 2 는 어디에도 없다
```
⟹ 명세의 *"is_jungle() 내부 임계(entity.rs:1378): camp_type.__0 < 2"* 는 **오독**. IR 의 `icmp ult camp_type.0, 2` 는 **`x==0 || x==1` 의 LLVM 접힘**이다.

**소스 줄 복원(줄 길이 산술, ±0)** — `rmeta_srcmap game_ai action_score.rs`:

| 줄 | 실측 문자수 | 복원 | 검산 |
|---|---|---|---|
| 526 | 33 | `  let coef = if t.ty.is_nexus() {` | 33 ✓ |
| 528 | 29 | `  } else if t.ty.is_tower() {` | 29 ✓ |
| **530** | **52** | **`  } else if t.ty.is_jungle(0) \|\| t.ty.is_jungle(1) {`** | 2+10+17+4+17+2 = **52 ✓** |
| **536** | **39** | **`  } else if t.ty.is_any_type_minion() {`** | 2+10+25+2 = **39 ✓** |
| **542** | **57** | **`  let based = if t.ty.is_jungle(0) \|\| t.ty.is_jungle(1) {`** | **57 ✓** |

(`t.ty` 필드접근 vs `t.ty()` 도 이 5줄 동시 일치로 확정 — 접근자면 줄마다 +2/+4 어긋난다.)

### 3‑2 ⚠ `-10` 경로의 술어는 `is_minion` 이 아니라 **`is_any_type_minion`**

`is_minion(&self, LineType)` MIR 은 **LineType 판별자 비교를 반드시 수반**(`_9=discriminant(Minion.0.0); _10=discriminant(_2); _6=Eq(_9,_10)`). 그런데 m05.ll:40027 switch 에서 **태그 1 → 곧바로 `phi [-10]`**, LineType 로드·비교가 **0개**이고 이 함수엔 LineType 지역변수도 없다. `is_any_type_minion` MIR = `Eq(discriminant, 1)` 한 줄로 IR 과 정확히 일치(+ 39자 일치).
⟹ `01.resolved` 의 *"is_minion 술어 이름 ★확정 = entity.rs:1256"* 은 **틀렸다.** 여기서 불리는 것은 **entity.rs:1260 `is_any_type_minion`**. (태그 1 = Minion 자체는 유효)

### 3‑3 ⚠ 파생: 「정글 캠프 티어 컷」 노브는 존재하지 않는다 — 3항목 삭제 대상

- `constants` `{value:2, src_line:530, "is_jungle 내부 임계 … 2 이상이면 coef=0"}` ✂
- `knobs` `{"is_jungle 의 camp_type.0 임계", value:2}` ✂
- `new_knobs` `{"정글 캠프 티어 컷", "camp_type.0 < 2", "티어 2 이상 캠프는 계수 0"}` ✂

**같은 파일의 `resolved`(camp_type.__0 = 팀 인덱스)와 위 3항목이 서로 모순**이었고, `resolved` 가 맞다.
**진짜 노브**: 소스가 `is_jungle(0) || is_jungle(1)` = *"양 팀 캠프 전부"* 로 못박고 있다. `is_jungle(player.info.team)` 이면 **아군 진영 캠프만**, `is_jungle(1 - team)` 이면 **카운터정글 전용**이 된다. 「티어」와 무관. (형제 `is_bee_jungle(&self, usize)` 도 `camp_type == (arg, JungleType::Bee)` 튜플 비교 — 같은 축, 교차검증됨)

### 3‑4 🔁범위정정: 「원본 .rs 없어 확인 불가」 2건이 뒤집혔다
- *"술어 소스 본문 … 독립 define 이 없어 이름 미확정"* → **재료 부재가 아니라 미탐색**. 네 술어 전부 `mir=True`, `mirdump.exe -Crate game_core` 로 4분 만에 본문이 나왔다.
- *"527·529·532~535·537~541 줄에 대응 IR 이 없다 … 확인 불가"* → **줄 길이로 전부 설명된다**: 527=7 `    200` · 529=6 `    80` · 532/534=8 · 533=12 `    } else {` · 535=5 · 537=7 `    -10` · 538=10 `  } else {` · 539=5 · 540=4 `  };` · 541=0. **소멸한 코드 없음.**

### 3‑5 ➕보강: 최종식 수신자
548줄 실측 46자. DWARF 가 `Ord::min` 의 self/other 를 남겼다 — `%57(sdiv) = self`, `%32(coef) = other`.
⟹ 소스는 **`(coef * value / hp as i64).min(coef) + based`**(46자 ±0). `smin` 가환이라 **동작 동일**, 재현 표기만 정정.

### 3‑6 나머지 확인
`PlayerState+0x930/+0x9c0` · `cache+0x1e0` · `Entity+0x68/+0x98/+0x670` · 계수 200/80/40/20/−10/0 · based 5 · `smin`/`sdiv` 패닉 2종 · `expected_damage_target` 5인자 — **전부 IR 일치**.

## 4. `02_sub_plan (AttackNexusPlan)` — ✅확인 + ➕보강 4

IR `m12.ll:34867~34976` 전량 재독해. **오류 없음.** `fountains(map+0x6d70, stride32)` `+0/+8/+16/+24 = lx/ly/rx/ry` · `x>=lx && x<=rx` · `y<ly || y>ry` · `hp(+0x670) < stat_cached.hp(+0x628)`→tag5 · `twin_towers[1-team].len(+0x148+32*(1-team))==0`→tag16 · else tag2 + `store 0/self.line/2` at `+0x8/+0x9/+0xa` — 바이트 단위 일치.

**➕보강 (unknown 5 중 4 종결)**
1. **fountains·nexus_pos 를 실행으로 확증** — `MapDef::moba(&GameSetting::default())`:
   `fountain(0)=(0,896000,64000,960000)` / `fountain(1)=(892000,0,960000,64000)` / `nexus_pos(0)=(96000,864000)` / `nexus_pos(1)=(864000,96000)` — 비대칭 `892000` 포함 **완전 일치**. IR 독해 → **실행 확증** 승격.
2. **bumpalo Vec 레이아웃 확정**(명세: "앞 24B 필드명 미확인") — `+0x0 buf.ptr.pointer` / `+0x8 buf.a : &bumpalo::Bump` / `+0x10 buf.cap` / `+0x18 len`. **두 번째 워드가 cap 이 아니라 Bump 참조**다.
3. `Position::as_index` 재배치 없음(MIR).
4. **「한 줄 `&&` 인지 중첩 if 인지 IR 로 구분 불가」 → 구분된다.** DWARF `DILocalVariable`: `champ`(36)·`lx/ly/rx/ry`(37)·**`is_in_heal_area`(38)**·**`has_enemy_twin_tower`(45)**, `dloc 62967→38 / 62969→41` ⟹ **영역 판정(38)과 HP 판정(41)은 다른 줄**, 38 은 이름 붙은 `let`. 줄 길이 L38=91/L41=58, 후보 `let is_in_heal_area = champ.x >= lx && … <= ry;` = 89자로 **잔차 2자**(표기 불가 아님, 잔차).

## 5. `03_defensive_crisis` — ⚠정정 1(중대) + 🔁범위정정

**본문은 오류 없음**: `tps=setting+0x12f8` · `enemy_ix=1-team` · `EntityType==13` 게이트 · `level>2→skill2_effect(+0x500)`/`level>4→ult_effect(+0x538)` · 쿨다운 `+0xb8/+0xc0/+0xc8` · `icmp ugt cool,tps` 로 **`cool<=tps` 만 통과** · 니치 `+0x4f8==-1` · `effect_cc_time.0==1` · `die < tps<<1` — 전부 일치. `towers` 빈 Vec(`inttoptr(8)`+memset0) ✓, `player_by_champion_id`(simulation.rs:1915, MIR `for t in 0..2`) ✓, `writes: []` ✓.

### ⚠ `judge_accuracy` 는 `PlayerState+0x180` 의 **필드가 아니다**
명세 `new_knobs`: `judge_accuracy(PlayerState+0x180)` / `value: "0~1000"` / *"0 이면 ×0.500~1.500"*

- `tcxdict PlayerState 0x180` → **`info.parameter.stat.like_champion.buf.inner`** = `info.parameter : AthleteParameter(744B)` 의 **시작 주소**.
- IR `m15.ll:31277~31279`: `%47 = gep %2, 384` → `%48 = invoke @…AthleteParameter14judge_accuracy(dereferenceable(744) %47)` ⟹ **오프셋은 AthleteParameter 시작, judge_accuracy 는 인라인 안 된 pub 메서드**(player.rs:337, 본체 `_gcbc/g15.ll:125794`). **`+0x180` 에서 u64 를 읽으면 `Vec<String>` 포인터를 읽는다** — 재구현 시 조용히 쓰레기.
- 본체(반환 `range(i64 100, 1001)`):
  ```
  j = self+0x98  (AthleteParameter.stat.judgement)
  r = self+0x2d0 (AthleteParameter.judgement_mental_ratio)
  x = umin(r * j / 1000, 100);  return x*9 + 100     ⟹ 치역 100..=1000 (9 간격 이산)
  ```
- **오라클 실행 확증**: `AthleteParameter::new(&AthleteStat::default()).judge_accuracy() = 100`.

⟹ 정정 ①`where` = **`PlayerState+0x180 = info.parameter : AthleteParameter`, judge_accuracy 는 그 위의 pub 메서드** ②`value` = ~~0~1000~~ → **100~1000**, *"0 이면 ×0.500~1.500"* 은 **도달 불가**(최악 acc=100 → h=450 → **×0.550~1.450**) ③➕**새 노브 2개**: `AthleteStat.judgement`(param+0x98) 와 **`judgement_mental_ratio`(param+0x2d0 = PlayerState+0x450)** — 후자가 **멘탈에 따라 판단 정확도를 흔드는 곱수**로, 명세 어디에도 없다. `min(..,100)` 포화 존재.

⚠**같은 오귀속이 `_shared.전투_위협모델` 에도 있다**(*"judge_accuracy(PlayerState+0x180) 기반 균등 난수"*). 공유 파일이라 손대지 않았다 — **메인 세션 정정 필요.**

🔁 `03.unknown` 의 *"check_kill_die_tick 반환값이 '사망까지 남은 틱'이라는 것은 추정"* 은 이미 `resolved` 에서 확정됐는데 `unknown` 에 옛 문장이 남아 있다(다음 세션이 "추정"으로 오독할 위험).

## 6. `04_handle_line_defense` — ⚠정정 1(구조) + 🔁범위정정 3

### 6‑1 ⚠ `morgard_exists` 는 (b)(c)를 **포함하지 않는다** — 549줄의 별개 항이다
- tcx: `pub morgard_exists(&GameContext) -> bool`, **mir=True** — **인자에 `game` 이 아예 없다.** 구조상 (b)(c)가 들어갈 수 없다.
- MIR 전문(`_tcx/mirdump_game_ai.txt`): `_3 = discriminant(ctx.tutorial) @runner.rs:263; switchInt -> [0:true, 7:true, 8:true, otherwise:false]` — **튜토리얼 게이트 한 줄이 전부.**
- `dloc m04.ll` inlinedAt 루트:
  ```
  !66990 → spawn_epic(runner.rs:263) ← morgard_exists(rule_scope.rs:46) ← handle_line_defense:549
  !66998 → as_moba(game.rs:231)                                          ← handle_line_defense:549  (morgard_exists 프레임 없음)
  !67010 → remain_epic_time(game.rs:210)                                 ← handle_line_defense:549  (없음)
  ```
- **본문 복원 ±0**: rule_scope.rs L45=54자/L46=**31자**/L47=1자, `_tcx` MIR span `46:20-46:32` ⟹ 20~31칸 = `spawn_epic()`(12자).
  ⟹ **`pub fn morgard_exists(context: &GameContext) -> bool { context.tutorial.spawn_epic() }`** 확정(`  context.tutorial.spawn_epic()` = 31자, 칸 위치까지 일치).

### 6‑2 🔁 「소스 표기 확정 불가」 → **확정됐다**
`defense_nexus.rs` 문자수: L544=**39**·L545=17·L546=3·L549=**134**
- **L544 = `  if !line_exists(data.context, line) {`** = 39 ✓±0 (L545 `    return false;`=17, L546 `  }`=3) ⟹ **`if !… { return false; }` 형태 확정**
- **L549 후보 ±0**: `  if !morgard_exists(data.context) || data.cache.game.get_game_mode().as_moba().unwrap().remain_epic_time(1 - player.info.team) == 0 {` = 2+32+4+93+5+2 = **134 ✓**, dloc 결과와 정합. ⟹ **`!morgard_exists(..)` 확정**(두 번째 항은 검산 일치이나 유일성 미보장).

### 6‑3 🔁 「{0,7,8} 은 추정」 → **실행으로 확정**
오라클 `TutorialType::spawn_epic()` 전 9값: **None0 true / First1~JungleOnly6 전부 false / Line7 true / Total8 true** + MIR switch 와 일치. **추정 아님.**

### 6‑4 🔁 나머지 unknown 3건도 닫혔다
| 명세 unknown | 실측 |
|---|---|
| `champions` 2번째 인자를 '팀'으로 읽은 근거가 약함 | **확정.** 본체 `_gcbc/g15.ll:109887~109895`: `%8=cache+480(player_champion)`, `%9=[5 x ptr] gep index %2` ⟹ **2번째 인자 = 팀**. tcx sig `champions(&self, usize, &Bump)` |
| `MobaMode+0x240` 은 추정, 크기 교차검증 못 함 | **확정.** `tcxdict MobaMode 0x240 → epic_minion_buff_time[0]`, MobaMode 640B. MIR `remain_epic_time(team) = self.<f2>[team]` + 경계검사 — **시간 산술 없이 배열 직독** |
| Blackboard 인덱스 의미 미확정 | `_shared` 로 이미 확정. `unknown` 의 옛 문장 정리 필요 |

### 6‑5 ⚠사소: `new_knobs` 좌표표가 팀을 안 밝힌다
*"Top 820000/80000 · Mid 817000/144000 · Bottom 880000/144000"* 는 **team1 값만**이다. 오라클 실측:
```
Top    t0 (80000,820000)   t1 (820000,80000)
Mid    t0 (144000,817000)  t1 (817000,144000)
Bottom t0 (144000,880000)  t1 (880000,144000)
```
➕**부수 확증**: 프로브의 `GameSetting::default()` 는 `tick_per_second = 0` 인 영행렬인데도 좌표가 정상값으로 나왔다 ⟹ *"setting 은 본문에서 한 번도 로드되지 않는다"* 가 **실행으로 확증**됐다.

### 6‑6 본문 확인
`vtable+0x40 get_game_mode` · `MobaMode+0x240[1-team]==0→false` · tower 6오프셋 + `Option::or` · `twin_towers[team]` min_by_key 폴백 + `tower0.or(nearest)` · `has_line_defense_threat(player,data,line,tower.id+0x5c0)` · **is_recent_visible 먼저, 거리 `<40000000001` 나중** · `Strategy+0xe morgard_defense`(`PlayerState::strategy` 반환은 **`game_core::Strategy` 24B**, TeamStrategy 아님) · `Battle → (n-1) <u 2` / `Gather → n >u 1` — **전부 IR 일치**. `tower` 정의 줄도 명세대로 simulation.rs:**1822** 맞다.

## 7. ★교차 발견 — 공유 문서 2곳의 「선언 순서」 주장이 거짓

> `METHOD_MAP §4 함정1` / `_oracle README §2`: *"LineType 은 태그 Top0/Mid1/Bottom2 인데 **선언 순서가 Mid,Bottom,Top**"*, *"ObjectPhase … **선언 순서 Setup,Assemble,Hunt,None**"*

**둘 다 틀렸다.** tcx 의 **variant 별 `def_span`**:
```
LineType::Top    player.rs:991     ObjectPhase::None      team_plan.rs:1096
LineType::Mid    player.rs:992     ObjectPhase::Setup     team_plan.rs:1097
LineType::Bottom player.rs:993     ObjectPhase::Assemble  team_plan.rs:1098
                                   ObjectPhase::Hunt      team_plan.rs:1099
```
`tcxdict --enum` 의 `idx`(=rustc `VariantIdx`=선언 순서)도 태그와 **완전히 같다**.
독립 교차검증(줄 길이, 문자수): `989: 19 "pub enum LineType {"` / `990: 12 "  #[default]"` / `991: 6 "  Top,"` / `992: 6 "  Mid,"` / `993: 8 "  Bottom"` / `994: 1 "}"`. `  Bottom,`(9자)는 991/992(6자)에 물리적으로 못 들어간다.
➕덤: **`LineType::default() = Top`**(오라클 실행으로도 확인).

⟹ **진짜 함정은 "메모리 태그 ≠ 논리 인덱스(니치 밀림)" 하나뿐**(SubPlan idx+2 등 — 이건 실재·검증됨). **"선언 순서 ≠ 태그"는 실증 사례 0건**이다. 두 문서의 그 문장은 다음 세션을 헛다리 짚게 만든다.

## 8. 방법론 소득

1. ★**`mirdump.exe -Crate game_core` 가 이번에 처음 돌았다**(`_tcx` 엔 game_ai 판만 있었다). 산출 = `_tcx\mirdump_game_core.txt`(403MB, **17,575 바디**, ~4분). `xinl=1` 인 **작은 game_core 헬퍼 술어 본문이 전부 여기 있다**(`is_*`, `Entity::radius`, `is_visible_from`, `Position::as_index`, `remain_epic_time`, `player_by_champion_id`…). 이번 정정 3건 중 2건이 여기서 나왔다. 추출기 `A_mirget.py`. **2026-09-11 상설화 완료 = `_tcx\mirdump_game_core.txt`.**
2. ★**"IR 에 `< N` 상수가 보인다"를 소스 임계로 읽지 마라** — `x==0||x==1` → `icmp ult x,2` 는 표준 접힘이다. **호출 술어의 tcx 시그니처를 먼저 보라**; 인자를 받는 술어면 상수는 십중팔구 접힘. (함정 톱10 추가 후보)
3. **줄 길이 산술은 "인자 개수 판별기"로도 쓸 수 있다** — `is_jungle(0)||is_jungle(1)`(52자)·`is_any_type_minion()`(39자)을 ±0으로 갈랐다.
4. **오라클은 「값 확증」에 특히 싸다** — `MapDef::moba` 한 번으로 12쌍 확증. `GameSetting::default()` 가 영행렬인 점을 역이용하면 **"이 인자를 정말 안 읽나"** 도 실행으로 증명된다.
5. `tcxaudit --prose` 는 **JSON 명세 파일에 그대로 먹는다**.

## 9. 남긴 것 (미탐색 — 불가 아님)

- `02` L38 `is_in_heal_area` 표기 **잔차 2자** — 미탐색 = 다른 표기 후보 / exe 디스어셈.
- `01` L531(20자) — `if hp <= value {` 와 `if value >= hp {` 가 **둘 다 20자**라 **표기 불가**(외연 동일). 동작은 확정(`hp>value → 20`). ⚠명세의 `if hp > value {`(19자)는 **길이가 안 맞는다** = then/else 가 뒤집힌 형태가 원문.
- `01` L548 의 `hp as i64` 자리 — 46자 검산 ±0이나 동일 길이 대안 여지 = **잔차**.
- `judgement_mental_ratio`(AthleteParameter+0x2d0)를 **누가 언제 갱신하는지** — `AthleteParameter::update` **미탐색**.
- `00` 의 `get_input_target` / `linear_cast_range_with_margin` 내부는 이번에 재검증하지 않았다 = **미탐색**.

