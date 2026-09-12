# 6차 반증검증 — 배치 C (`specs[10]`~`specs[14]`) / 게임 0.5.8 / 2026-09-11

산출물은 전부 `C:\tfm2mods\MIG\_verify6\C\` 안에만 있다.
⛔`_spec\specs20.json` · `_spec\specs20_v3.json` · `distruct.json` · `dienum.json` **읽기만 했다**(수정 0).

---

## 1. 결론 한 줄

> **실오류 2건**(그중 `reused` **2건**, 이번 라운드 최초 발견 **1건**) · **분류오류 1건** · **보강 5건** ·
> **판정반전 0건** · **새 발견 5건**(§5 N1~N5) · **`ev` 상향 213행**(mem 102 → ev3 / consts 74 + knobs 37 → ev2).
> **값·오프셋·상수·분기 오류는 이번에도 0 — 배치 C 는 4·5·6차 3라운드 연속 「값 오류 0」이다.**

한 줄 더: **내 기존 기법(`reused`)만으로도 실오류가 1건 나왔다.** 즉 「오류는 새 계측기가 만든다」는
주장은 **완전히는 맞지 않는다.** 다만 그 1건은 **값이 아니라 숫자의 좌표계(리맵 인덱스↔메모리 태그)**
문제이고, 위치도 **담당 함수 밖을 설명하는 `knobs` 행**이다. 자세한 해석은 **§8**.

---

## 2. `patch.json` 요약

`_verify6\C\patch.json` — 생성기 = `_verify6\C\mkpatch.py`(손으로 213줄을 옮기면 또 새기 때문에 규칙으로 만들었다).

```
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 6 --only C --dry
[C] 정정 8/8 · ev상향 213/213
정정 8 성공 / 0 실패 · ev상향 213 성공 / 0 실패 · **동작 변경 0건**
발견 경위: reused=8
```
**적용 실패 0.**

### 2-1. `errors` 8건

| # | path | kind | 요지 | `found_by` | 최초발견 |
|---|---|---|---|---|---|
| E1 | `/specs[11]/knobs[11]` | **실오류** | `SubPlan::merge` 보존 화이트리스트 `{4,7,8,9,11,12,15,16}` 은 **리맵 인덱스**(`tag>1 ? tag−2 : 6`)다. **SubPlan 메모리 태그로는 `{6,9,10,11,13,14,17,18}`** | `reused` | **6차** |
| E2 | `/specs[12]/logic` | **실오류** | `chat_allowed(data.context, chat)` → `&chat` (tcx: `fn(&GameContext, &Chat)`) | `reused` | 5차(미반영) |
| E3 | `/specs[13]/open[0]` | 분류오류 | 「재료 부재」 문면 제거 + **구조 확정**으로 대체 | `reused` | 5차(class 미반영) + 6차 확정 |
| E4 | `/specs[11]/logic` | 보강 | `goal_allowed` 줄범위 `90~95` → `90~96`(L96 = `Nexus|Battle|Recall` arm) | `reused` | **6차** |
| E5 | `/specs[11]/knobs[16]` | 보강 | 로밍/에고 식에 **선행 컷오프 `500−roaming < 1`** 이 빠져 있었다 | `reused` | **6차** |
| E6 | `/specs[13]/open[1]` | 보강 | `v30` = **AI 내부 버전 번호 30**(도입 시점 표식) — 근거 3중 | `reused` | 5차 N1(미반영) + 6차 보강 |
| E7 | `/specs[13]/logic` | 보강 | `LineGankCoverPlan::update`(cover.rs:25)는 **본문이 비어 있다**(MIR `bb0: T return`) | `reused` | 5차 N4(미반영) |
| E8 | `/specs[10]/consts[4]` | 보강 | 25000 의 **선형 가산 분해** `20000 + effect.range + margin`, 140000 하드컷 | `reused` | 5차 N2(미반영) |

**8건 중 4건(E2·E6·E7·E8)은 5차 배치C 가 이미 낸 결론인데 `patch5.py` 에 실리지 않아 정본에 없었다.**
`patch.json` 계약이 없었기 때문이고, 이번 라운드가 그걸 메우는 것이 주 목적이었다.

#### E1 상세 (이번 라운드 유일한 **신규 실오류**)
```
구: value = "case {4,7,8,9,11,12,15,16}"
신: switch 키는 리맵 인덱스 (tag>1 ? tag−2 : 6) 기준 ⟹ SubPlan 메모리 태그로는
    {6 Jungle, 9 Hide, 10 EpicCheck, 11 EpicHunt, 13 SerpenCheck, 14 SerpenHunt,
     17 DefenseNexus, 18 Steal}
```
근거 = `_gaibc/m12.ll:36960~36974`
```llvm
%3 = load i64, ptr %0          ; SubPlan 원시 태그
%5 = add nsw i64 %3, -2
%6 = icmp samesign ugt i64 %3, 1
%7 = select i1 %6, i64 %5, i64 6
switch i64 %7, label %8 [ i64 4  i64 7  i64 8  i64 9  i64 11  i64 12  i64 15  i64 16 ]
```
switch 피연산자가 **원시 태그가 아니다.** 이 해석에서만 `shared.SubPlan_merge` 의 보존표
(Jungle 6 · Hide 9 · EpicCheck 10 · EpicHunt 11 · SerpenCheck 13 · SerpenHunt 14 · DefenseNexus 17 · Steal 18)와
같은 명세 `closed[]` 의 「`EpicPoke(12)`·`SerpenPoke(15)` 는 switch 에 **없어** 항상 덮어쓰기」가
**동시에** 성립한다. 원시 태그로 읽으면 12·15 가 switch 에 있으므로 두 기록이 서로 모순한다.
⟹ `knobs` 한 행이 `shared` 정본·`closed` 와 충돌하고 있었고, **`G5`/`G6`/`G8` 은 `logic`↔표만 보므로
`knobs ↔ shared` 충돌을 구조적으로 못 잡는다**(5차 배치B 가 지적한 사각지대와 같은 종류).

### 2-2. `ev_up` 213행 — 필드·명세별

| 명세 | mem→ev3 | consts→ev2 | knobs→ev2 | 계 |
|---|---|---|---|---|
| `specs[10]` | 19 | 6 | 7 | **32** |
| `specs[11]` | 20 | 19 | 11 | **50** |
| `specs[12]` | 23 | 12 | 6 | **41** |
| `specs[13]` | 15 | 16 | 8 | **39** |
| `specs[14]` | 25 | 21 | 5 | **51** |
| **계** | **102** | **74** | **37** | **213** |

배치 C 의 `ev≥4` 는 **239행**이었고 그중 **212행(88.7%)** 이 내려간다(213행 중 1행 = `12/consts[11]` 은
`ev3→ev2`). **잔여 27행은 전부 `knobs`**,
그리고 전부 **담당 함수 밖 코드**를 설명하는 행이다(`specs[11]` 11개 = `SubPlan::merge`·`passive_plan`·
`handler` 다른 구간 / `specs[12]` 13개 = `handle_chat_inner` 내부 / `specs[13]` 2개 = 셀 클램프·미니맵 /
`specs[14]` 1개 = 선택기 셋). 이 27행은 **다른 함수의 오라클을 새로 세워야** 내려간다 — 범위를 명시해 남긴다.

⚠**`mem` 은 전부 `to: 3`** 으로 냈다(`BRIEF §1`: 오프셋의 정본은 tcx, 오라클은 동급 교차검증).

---

## 3. `ev<=3` 표본 재확인 — **뒤집힌 것 0건**

배치 C 의 현행 `ev<=3` 은 15행(ev2 4 / ev3 11)이다. 그중 **8행을 이번에 독립 재측정**했다.

| 행 | 기존 ev | 이번 재확인 | 결과 |
|---|---|---|---|
| `10/mem[3]` `Strategy+0xf` object_finish | 3 | `o5_mask.out §O6-F`: 24B 날바이트 `05 00 00 00 05 00 00 00 02 00 00 00 00 00 00 **01** …` 의 `+0xf`=1, `Debug` 가 `object_finish: BattlePriority` | **유지** |
| `11/mem[14]` `BigPlan+0x0` tag=4 | 3 | `tcxdict --enum BigPlan`: SinglePlanLine **메모리 태그 4**(니치, untagged=DeathMatchBattle, niche_start=2) | **유지** |
| `12/mem[14]` `TraceEventType+0x0` | 3 | `tcxdict --enum TraceEventType`: niche_start = 2^63, CallHandled = **idx 15** ⟹ 태그 2^63+15 = i64 −9223372036854775793 | **유지** |
| `12/consts[11]` −9223372036854775793 | 3 | 동상 + 5차 날바이트 관측 | **유지** |
| `14/mem[25]` `LineGankerPlan+0x18` setup_limit | 3 | ⛔`offset_of!` **불가 — private 필드(E0616)**. `tcxdict` 재조회 `0x18 setup_limit usize` | **유지**(경로 제약 기록) |
| `14/mem[26]` `+0x20` wait_limit | 3 | 동상 | **유지** |
| `13` self.line `+0x20` (sig.params) | — | ⛔`offset_of!` 불가 — `LineGankCoverPlan::line` private(E0616) | 경로 제약 기록 |
| `14/knobs[3]` `is_top_side` 극성 | 2 | 이번에 재측정하지 **않았다**(3·5차 오라클 유지, 범위 명시) | 유지 |

재측정하지 않은 7행 = vtable 슬롯 4행(`10/mem[10][11]` · `11/mem[4][5]` · `12/mem[6]`) ·
`12/knobs[19]`(ff_call 8칸) · `12/knobs[20]`(글로벌 궁 예약, **입력 판별력 부재로 미검증** 표기 그대로) ·
`13/knobs[7]`(부시 ID 그리드). **범위를 명시해 남긴다.**

### 3-1. `notes` 3건 — **반증 시도 전건 실패(= 명세가 맞다)**

| notes | 반증 방법 | 결과 |
|---|---|---|
| `10/notes[0]` phase 리터럴 3 이 768 로 접힘 | IR 담당범위(`m10.ll:49611~50100`) grep + `o5_mask` 실행 | `and i24 %4, 65534` / `icmp eq i24 %8, 768` **만** 있고 리터럴 3 없음. 실행에서도 `ObjectPhase::Hunt` 바이트 = 3, 게이트 기대값 768 **일치** ⟹ 유지 |
| `10/notes[1]` 적 챔프 5칸 완전 언롤 | 같은 범위 IR | 슬롯 로드가 **5회 복제**(상대줄 189·248·307·366·425, 간격 ~59줄), 루프 변수·`phi` 없음(phi 2개는 다른 용도) ⟹ 유지 |
| `14/notes[0]` cover 판 ↔ ganker 판 **동일 복제본** | rmeta SourceMap 재측정 | `cover.rs:134~186` ↔ `ganker.rs:248~300` 줄 길이 **53/53 일치**(불일치 0). §4 의 구조 분석이 오히려 이를 보강 ⟹ 유지 |

### 3-2. `open` 2건 — **둘 다 해소**

**`13/open[0]`(L151↔L156 이 같은 값인 이유)** — ★**「미탐색」이 맞았고, 이번에 구조까지 확정했다.**
`tcxq lines game_ai line_gank\cover.rs 130 200` 실측:

| 줄 | 149 | 150 | 151 | 152 | 153 | 154 | 155 | 156 | 157 | 158 |
|---|---|---|---|---|---|---|---|---|---|---|
| Top | 56 | 36 | **54** | 19 | 46 | 56 | 21 | **56** | 14 | 12 |
| Bottom(179~188) | 56 | 36 | **56** | 19 | 46 | 58 | 21 | **58** | 14 | 12 |

가설 「**동일 텍스트 + 들여쓰기 2칸**」이 10줄을 동시에 설명한다(반례 0):
- `L150(36)` ↔ `L153(46)` 차이 **10** = 술어 문자수 차 **8**(`tower.ty.is_tower2()` 20자 → `info.nearest_enemy.is_some()` 28자) + 들여쓰기 **2**
- `L152(19)` ↔ `L155(21)` **+2** · `L157(14)` ↔ `L158(12)` **−2**(닫는 괄호가 한 단 깊다)
- `L151(54)` ↔ `L156(56)` **+2** — 값이 같은(3/6) **같은 반환식**이 한 단 더 들여쓰기된 것
- Bottom 블록은 리터럴 자리수(3·6 → 15·20) 때문에 전 줄이 Top 대비 **+2**

⟹ 소스는 `else if` 가 **아니라 `else { if … }`** 다. 그래서 「2차타워」 팔과 「적 미인지」 팔이
같은 값을 내면서도 **따로 쓰일 수밖에 없다.** 「원래 다른 값이었을 것」이라는 흔적은 없다.

**`13/open[1]`(`v30` 이 무엇의 버전인가)** — **AI 내부 버전 번호 30**. 근거 3중:
1. `_tcx/game_ai.json` 이름 집계 — `vNN_` 접두가 **v2·v3·v15~v17·v21~v28·v30·v46~v48·v50·v54·v55·v57** 에 걸쳐 190개,
   `_vNN` 접미가 **v3·v15·v26·v30·v32·v37·v41·v46·v54**.
2. ★개발자 주석(`_docs/game_ai.txt`)이 **`v54+`·`v<54`·`v92+`** 처럼 **부등호 비교**로 쓴다 ⟹ 라벨이 아니라 **버전 임계**.
   (`v92` 가 나오므로 이 번호는 우리가 프로브에 넣는 `version` 인자의 0~7 범위와 **같은 축이 아니다** — 개발 이터레이션 번호다.)
3. 그런데 **이 플랜에는 런타임 버전 분기가 없다** — `LineGankCoverPlan::sub_plan`·`next_plan`·`target_bush_v30`
   전 구간에서 `version` 인자 비교 0건. 버전 선택은 **호출부 하드와이어**(갱커: `update`→v30 / `sub_plan`·`next_plan`→v41).

---

## 4. 이번 라운드에 새로 쓴 계측기 — ⓐ `offset_of!` 일괄 대조 (**`inherited`**)

5차 배치A 가 발명했고 **배치 C 는 이번이 처음**이다. 결과 = **MISMATCH 0**.

| 프로브 | 대상 | 결과 |
|---|---|---|
| `o1_off.rs` / `.out` | game_core 구조체 — 오프셋 **36** + 크기 **13** | 전건 OK |
| `o2_off_ai.rs` / `.out` | game_ai 구조체(`LegacyPlanHandler`·`LineGankerPlan`·`LineGankCoverPlan`) — 오프셋 **12** + 크기 **3** | 전건 OK |
| `o3_off2.rs` / `.out` | **튜플 필드** `mf_swap.0`/`.1` + 중첩 `team_plan.objective` — 오프셋 **3** | 전건 OK |
| **계** | **오프셋 51 + 구조체 크기 16 = 67 검사** | **MISMATCH 0** |

★`mf_swap: (u8, usize)` 는 rustc 가 튜플 필드를 재배치할 수 있어 `.0`=+0 이라는 보장이 없다.
명세는 `.0=+0x1610` · `.1=+0x1618` 로 적었고 **실행 결과가 그대로였다**(`o3_off2.out`).

⛔**`offset_of!` 로 못 간 3행 = private 필드**(rustc E0616): `LineGankerPlan::setup_limit`·`::wait_limit` ·
`LineGankCoverPlan::line`. **범위 한정**이다 — 「불가」가 아니라 「이 경로로는 불가」이고, tcx 정본으로 확인했다.

### 4-1. `memchk.py` — `mem` 표 **112행 전량**을 `tcxdict` 로 행별 재조회

`offset_of!` 가 닿지 못하는 행(열거형 페이로드·`Vec` 내부·private·vtable 슬롯·팻포인터 뒤 절반)을 메웠다.

```
집계  MATCH=88 · CHECK=2 · NOHIT=3 · SKIP=19        (_verify6/C/memchk.tsv)
```
- `CHECK` 2 / `NOHIT` 3 = 전부 `AbstractGameWithCache.game` **팻포인터**다. tcx 는 `game` 을 +0x0 의 16B
  `&dyn AbstractGame` 으로 주므로 데이터 절반 = +0x0, **vtable 절반 = +0x8**(Rust ABI). `tcxdict` 가
  뒤 절반을 필드로 세지 않는 것은 `METHOD_MAP ⑦` 에 명시된 한계이고 **오류가 아니다.**
- `SKIP` 19 = vtable 슬롯 5 + 열거형 페이로드 14. 열거형 14행은 `tcxdict --enum` 으로 **개별 확인**했다:
  `BigGoal`(tag@+0x0 1B Direct, `Line.line`@+0x1) · `BigPlan`(SinglePlanLine 태그 **4**, 페이로드 +0x8;
  `SinglePlanLine` 32B = chats@0x0 / in_recall@0x18 / line@0x19 ⟹ BigPlan +0x8/+0x20/+0x21 **정확히 일치**) ·
  `TraceEventType::CallHandled`(+0x8/+0x20/+0x38/+0x50/+0x68/+0x80/+0x84 **7필드 전건 일치**) ·
  `PendingTraceEvent`(184B = event@0x0 176B + **tick@0xb0**).

---

## 5. 이번 라운드에 새로 얻은 실행 커버리지 (`reused` = SDK 오라클)

### N1 ★5차 `§O10-D`(「version 0~5 무영향」)는 **판별력 0 인 vacuous 증거**였다 — 이번에 제대로 잰다
5차 프로브는 `is_enemy_well_danger` 를 team0 플레이어로 **team0 진영 좌표**에서만 쟀고 **162칸이 전부 false** 였다.
이번(`o4_s10.out §O6-A/A2/B`): 32000 격자 **900칸** 전수 →

```
player.team=0  true 21칸  bbox=(800000,0)~(928000,160000)     ← 적(team1) 우물
player.team=1  true 21칸  bbox=(0,800000)~(160000,928000)     ← 적(team0) 우물
version 0..7 불일치 = 0   (true 21칸을 실제로 밟은 상태에서)
```
경계도 `history[3]` 주장과 정확히 맞는다(team1 플레이어 기준): `(0,800000)` true / `(0,799999)` **false** /
`(160000,896000)` true / `(160001,896000)` **false**. `(64001,960000)` 이 true 인 것은 2번 사각형
`(0,896000,160000,960000)` 에 들어가기 때문으로, **두 사각형 OR** 주장과 정합이다.
⟹ 명세는 **유지**되고 근거만 강해졌다. ★교훈 = **「전건 동일」은 「전건 false」와 구별해서 읽어야 한다.**

### N2 `is_ignored_well_enemy` 팀 조건을 실행으로 갈랐다 (`§O6-C`, `Entity` 복제 주입 ⑦)
`player.team=0` 고정, `Entity` 를 복제해 `team`·`x`·`y` 만 바꿔 6조합:
**`enemy.team==1` ∧ 적(team1) 우물 안** 에서만 true. ⟹ `logic` 의 `enemy.team == Player(1−player.team)` 확인.

### N3 ★`objective_entity_id_for_main_objective` 의 **「첫 원소」 경로를 처음 밟았다** (`§O6-D`)
5차는 `start_game` 직후에 쟀기 때문에 `epic/serpen live_list` 가 비어 **태그 12개 전부 None** 이었고,
5차 보고는 「0/1 의 첫 원소 경로는 재확인 못 함」으로 범위를 남겼다. **300틱을 돌리면 바로 채워진다**:
```
tick=300  epic.live=1  serpen.live=1   tag0=Some(41)  tag1=Some(40)
```
⟹ `MobaMode+0x1a0/0x1a8`(epic ptr/len)·`+0x1d0/0x1d8`(serpen) 읽기 경로가 **실행으로 확인**됐다.

### N4 `specs[10]` 진입 마스크를 **날바이트 전수**로 확정 (`o5_mask.out §O6-E`)
`MainObjective` 25 케이스의 3바이트를 읽어 `(v & 65534) == 768` 을 평가 →
**통과 집합이 정확히 `{Morgard, Serpen} × Hunt × {with_battle=false,true}` 4개**(오검출 0).
`ObjectPhase` 바이트 = Setup 1 / Assemble 2 / **Hunt 3**, `with_battle` 는 마스크 밖 ⟹ 명세 그대로.

### N5 (IR 재독으로 확인만) 담당 함수 밖 `knobs` 4행
`11/knobs[11]`(merge switch, §2-1 E1 로 정정) · `11/knobs[12]`(Steal `icmp ugt` = max, `m12.ll:37171`) ·
`11/knobs[16]`(로밍/에고, `m13.ll:6892~6903` — E5 로 컷오프 보강) · `11/knobs[17]`(`icmp ult %499, 14400000001`, `m13.ll:7512`) ·
`12/knobs[12]`(`gen_range(0,1000) < ego*700/1000`, `m13.ll:6xx→31001~31004`) · `12/knobs[13]`(`70 − roaming*20/1000`, `m13.ll:30868~30870`).
**IR 독해는 ev4** 라 상향하지 않는다(값은 전건 일치).

---

## 6. 게이트 실측

```
PYTHONIOENCODING=utf-8 python -X utf8 specgate.py
   G1 자기모순=0  G10 class 오분류=0  G11 ev 지시 미반영=2  G2 호출부 전수=0  G3 형제 함수=0
   G4 술어 시그니처=0  G5 sig 정본 대조=0  G6 logic 미반영=0  G7 과열림=0  G8 표에 옛 값=0  G9 callees 오염=4
```
- **G11 = 2 는 내 몫이 아니다** — `[03] defensive_crisis`(3행) · `[04] handle_line_defense`(2행), 둘 다 배치 A.
- **G9 = 4 는 전부 `손확인` 등급**(`BRIEF §4` 가 정상이라고 명시). 그중 2건이 내 것(`[13]`·`[14]` 의
  `line`/`nearest_enemy`/`position`/`team`)인데, 이것들은 `logic` 이 **필드 접근**으로 쓴 이름이고
  tcx 에 동명 **함수**가 있어서 잡힌 것이다 — 실제 호출이 아니다. 정정 불요.

```
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 6 --only C --dry
   정정 8 성공 / 0 실패 · ev상향 213 성공 / 0 실패 · 동작 변경 0건 · 발견 경위: reused=8
```

```
PYTHONIOENCODING=utf-8 python -X utf8 auditrounds.py --round 5
   대조 128건 · 유실 0 · STALE 0 · ev되돌아감 0  → 전건 유지됨
PYTHONIOENCODING=utf-8 python -X utf8 auditrounds.py          (= 전 라운드, 6차 포함)
   유실 8 · STALE 6 · ev되돌아감 213  → ★조치 필요
```
```
PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose _verify6/C/C6_REPORT.md
   총 693건  **오귀속=0  밀림=0**  부분일치=1  확인불가=18  OK=674
```
★**불변 게이트 `오귀속 0 · 밀림 0` 충족.** 내 보고서에서 나온 `확인불가` 1건은
`TraceEventType +0x0` 을 산문으로 적은 것이고 tcx 응답은 `.tag` = 내 주장 그대로다(오류 아님).

⚠**뒤쪽 8/6/213 은 내 patch.json 의 내용 그 자체다**(아직 미적용이므로 정의상 전건이 「유실」로 잡힌다).
patch.json 을 쓰기 **전**에는 `0 · 0 · 0` 이었다. 이건 게이트와 계약이 충돌하는 것이고 §7-⑦에 적었다.

---

## 7. 브리핑·`BRIEF_FACTS`·도구의 오류 (7건)

### ① ★`BRIEF_FACTS.md §1` 표의 **`D` 행과 `계` 행이 stale 이다**
파일 mtime: `BRIEF_FACTS.md` **15:49** < `_spec/specs20.json` **15:51** < `specs20_v3.json` **15:53**
⟹ **5차 배치D 의 `patch.json` 이 적용되기 전에 생성된 수치**다. §1 의 정의(mem+consts+knobs)로 직접 세면:

| 배치 | 브리핑 (ev1/2/3/4/5 · ev≥4) | **실측 (현행 정본)** |
|---|---|---|
| A | 0/3/8/183/2 · 94.4% | 0/3/8/183/2 · 94.4% ✅ |
| B | 0/7/19/166/0 · 86.5% | 0/**6**/**20**/166/0 · 86.5% (1행 이동) |
| C | 0/5/10/239/0 · 94.1% | 0/**4**/**11**/239/0 · 94.1% (1행 이동) |
| **D** | 0/**0**/**22**/**194**/0 · **89.8%** | 0/**35**/**100**/**81**/0 · **37.5%** ❌ |
| **계** | 0/15/59/782/2 · **91.4%** | 0/**48**/**139**/**669**/2 · **78.2%** ❌ |

★이건 단순 오기가 아니라 **「생성된 사실 절이니 믿어도 된다」는 이 파일의 전제를 깨는 종류**다 —
생성물이라도 **생성 시각이 정본보다 이르면 틀린다.**
조치안 = `mkbrief.py` 가 정본의 mtime/해시를 헤더에 같이 찍고, 배치가 그것과 현재 정본을 대조하게 할 것.

### ② `BRIEF.md §2` 의 `669행` 과 `BRIEF_FACTS §1` 의 `782+2행` 이 **서로 다르다**
669 는 **현행** 정본의 `ev4` 단독 수(정확), 784 는 stale. 즉 **두 브리핑 파일이 서로 다른 시점의 정본을 본다.**
부수로 `669` 는 `ev4` 인데 라벨이 `ev>=4` 라 `ev5` 2행이 빠져 있다(실제 `ev>=4` = **671**).

### ③ ★`MIG\SPEC_GUIDE.md §1 fnparts` 의 「≠」가 **5차 지적 뒤에도 그대로다** (지시대로 확인함)
현행 문면: *「(5차 실측: `LineGankCoverPlan::target_bush_v30` **≠** 담당 함수의 `LineGankerPlan::target_bush_v30`)」*
5차 실측의 결론은 **정반대**이고 정본 JSON `specs[14]/notes[0]` 에는 **판정반전**으로 이미 들어가 있다 —
두 함수는 **문자 단위 동일 복제본**이다(6차 재측정: 줄 길이 **53/53 일치**, 다른 것은 `self` 타입과
그에 따른 `line` 오프셋 `+0x20`↔`+0x28` 뿐).
현행 문면은 「소유 타입이 다르면 그 조각을 버려라」로 읽혀 **관측 경로를 스스로 닫는다** — 실제로 그 조각
(cover 판 `define` + `LineGankCoverPlan::sub_plan` pub 래퍼)이 5차에 `specs[13]` 39행·`specs[14]` 51행을
실행 검증한 **유일한 통로**였다.
정정안 = 「소유 타입이 다르면 **오프셋이 다르다**는 뜻이지 본문이 다르다는 뜻이 아니다 — 버리지 말고 오프셋만 환산해서 쓰라.」

### ④ ★`auditrounds.py` 의 「전 라운드 회귀 · 유실 0」은 **배치 A·B·C 에 대해 공허하다**
실재하는 `patch.json` 은 `_verify5\D\patch.json` **하나뿐**이고, 도구는 patch.json 이 있는 (라운드,배치)만
대조한다(`auditrounds.py:74~76`) ⟹ 출력의 「대조 128건」은 **전부 5차 배치 D 몫**이다.
반증(전부 내 5차 결론인데 정본에 없다): `13/open[1]` v30 · `13` 의 `LineGankCoverPlan::update` 빈 본문 ·
`10` 25000 선형 분해 · **ev 상향 171행**. 그런데도 도구는 「유실 0 · 전건 유지됨」을 찍는다.
⟹ `BRIEF §4` 가 이걸 제출 게이트로 지정했으므로 **게이트가 초록인 것이 안전의 증거가 아니다.**
최소 조치 = 헤더에 「대조 대상 = patch.json 이 있는 (라운드,배치) 목록」을 찍을 것.

### ⑤ ★`mkspec3.classify()` 가 **판정 어휘를 부정하는 문장까지 그 어휘로 분류한다**
`specs[13]/open[0]` 의 문면은 5차에 「**「재료 부재」가 아니라** 「미탐색」이다」로 고쳐졌는데,
`CLASS` 가 부분문자열 `재료 부재` 를 먼저 잡아(`mkspec3.py:111`, `classify` 는 `:154~167`)
class 가 계속 `재료 부재` 로 앉아 있었다 — **정정문 자체가 오분류의 원인**이다.
같은 문면의 `알 수 없다`(→재료 부재)까지 이중으로 걸린다. `specgate G10`(class 오분류=0)도 못 잡는다.
이번 정정 문면에는 **판정 어휘 낱말을 아예 넣지 않는 방식**으로 우회했다.
조치안 = ①`CLASS` 매칭 전에 `「<어휘>」가 아니(라|다)` 부정형을 제거하거나 ②class 를 **명시 필드**로 받을 것.

### ⑥ `BRIEF_FACTS.md §3` 표의 인용문이 **부정어 직전에서 잘려 판정이 뒤집혀 보인다**
`09` 행은 「⚠단 **「재료 부재」가 아니라**」 에서 끊기고, `13` 행(분류 `재료 부재`)도
「소스에서 두 분기를 따로 쓴 이유(원래 다른 값이었는지」 에서 끊긴다. 값 자체는 생성물이라 맞고
**절단 길이만** 문제다 — 하필 부정어 직전이라 원문과 반대로 읽힌다.

### ⑦ ★`BRIEF §1`(patch.json 을 써라)과 `BRIEF §4`(게이트 = `auditrounds` 유실·STALE 0)가 **서로 배타적이다**
실측: patch.json 을 쓰기 **전** = `유실 0 · STALE 0 · ev되돌아감 0` / **쓴 직후** = `유실 8 · STALE 6 · ev되돌아감 213`
(= 내 patch 의 내용 그 자체, 아직 미적용이므로 당연하다). 게이트를 글자대로 지키려면 patch.json 을
비워야 한다 — 계약과 정반대다. 조치안 = `auditrounds` 가 **현재 라운드를 기본 제외**(또는 `--applied-only`)하고,
`BRIEF §4` 가 「자기 라운드 제외」를 명시할 것.

---

## 8. ★`found_by` 집계 — 이번 라운드의 결론

### 8-1. 숫자

| `found_by` | 정정(errors) | ev 상향에 쓰인 근거 | 비고 |
|---|---|---|---|
| `reused` | **8건 (100%)** | consts 74 + knobs 37 + mem 39행 | SDK 오라클 실행 · IR 독해 · rmeta 줄 길이 산술 · `tcxdict`/`tcxq` · `_docs` grep · 바이트 덤프 ⓒ |
| `inherited` (ⓐ `offset_of!`) | **0건** | **mem 51 오프셋 + 구조체 16 크기의 런타임 교차검증** | MISMATCH **0** — 오류를 하나도 못 찾았다 |
| `new` | **0건** | — | 이번에 새로 만든 기법 없음 |

**정정 8건 중 이번 라운드에 처음 발견된 것 = 4건**(E1 실오류 · E4·E5 보강 · E3 의 구조 확정분).
나머지 4건(E2·E6·E7·E8)은 **5차 결론의 미반영분 재제출**이다.

### 8-2. 해석 — 네 주장은 **부분적으로만 맞다**

> 네 예측: 「`reused` 실오류가 ≈0 이면 명세는 안정적이고, 발견은 계측기가 만든 것이었다.」

**실측은 `reused` 실오류 1건**(E1)이다. 0 이 아니다. 그러니 **「오류는 전적으로 새 계측기가 만든다」는
강한 형태의 주장은 반증됐다.** 다만 세 가지를 같이 봐야 한다.

1. ★**새 계측기(`inherited` ⓐ)는 오류를 0건 찾았다.** 5차에 배치 A 가 `offset_of!` 로 `mem` 45행을
   MISMATCH 0 으로 덮은 것과 같은 결과다. 즉 **계측기를 붙였다고 자동으로 오류가 나오지는 않는다** —
   3·5차에 오류가 쏟아진 이유는 「계측기가 새로워서」가 아니라 **「그 계측기가 처음으로 *한 번도 실행 안 된 영역*을
   밟았기 때문」**이다. 이번 ⓐ 가 밟은 영역(`mem` 오프셋)은 이미 `tcxaudit` 이 덮고 있던 곳이라 새 정보가 없었다.
2. ★**나온 1건의 성격이 다르다.** E1 은 **값이 틀린 것이 아니라 숫자의 좌표계**(리맵 인덱스 ↔ 메모리 태그)가
   빠진 것이고, 위치도 **담당 함수 밖을 설명하는 `knobs` 행**이다. 배치 C 의
   **값·오프셋·상수·분기 오류는 4·5·6차 3라운드 연속 0** 이다(`offset_of!` 63행 + `tcxdict` 112행 +
   오라클 수천 케이스로 이번에 다시 확인).
3. ★**진짜 불안정한 것은 명세가 아니라 「보고→반영」 구간이다.** 이번 정정 8건 중 **4건이 5차에 이미 나왔던
   결론**이고, `ev` 상향 **171행**도 그대로 증발해 있었다. 그리고 그 증발을 잡으라고 만든 `auditrounds` 는
   **공허하게 초록**이었다(§7-④). **라운드를 더 도는 것보다 이 구간을 고치는 쪽의 기대수익이 훨씬 크다.**

⟹ 실무 결론: **「같은 조건이면 오류가 안 나온다」는 아니다. 하지만 나오는 오류의 *등급*이 라운드마다
내려가고 있다** — 3차 값/분기 오류 → 5차 문면·시그니처 → 6차 **좌표계 표기 1건 + 보강**.
수렴 판정 기준을 「오류 0건」이 아니라 **「`behavior_change=true` 0건」**(이번 라운드 **0건**)으로 바꾸면
배치 C 는 이미 **3라운드 연속 수렴**이다.

---

## 9. 산출물 목록 (`MIG\_verify6\C\`)

| 파일 | 무엇 |
|---|---|
| `C6_REPORT.md` | 이 보고서 |
| **`patch.json`** | ★기계 판독 패치 — errors 8 · ev_up 213 · brief_errors 7 (`applypatch --dry` 적용 실패 **0**) |
| `mkpatch.py` | patch.json 생성기(213행을 손으로 옮기지 않기 위해) |
| `o1_off.rs` / `.out` | ⓐ `offset_of!` — game_core 오프셋 36 + 크기 13, MISMATCH 0 |
| `o2_off_ai.rs` / `.out` | ⓐ `offset_of!` — game_ai 오프셋 12 + 크기 3, MISMATCH 0 (private 3행은 E0616 로 제외) |
| `o3_off2.rs` / `.out` | ⓐ 튜플 필드 `mf_swap.0/.1` + `team_plan.objective`, MISMATCH 0 |
| `o4_s10.rs` / `.out` | `specs[10]` — 우물 사각형 900칸·버전 0~7·`is_ignored_well_enemy` 6조합·`objective_entity_id` **Some** |
| `o5_mask.rs` / `.out` | `specs[10]` 진입 마스크 25 케이스 날바이트 + `Strategy` 24B 덤프/Debug |
| `memchk.py` / `memchk.tsv` | `mem` **112행 전량** `tcxdict` 행별 재조회 (MATCH 88 / CHECK 2 / NOHIT 3 / SKIP 19) |
| `_spec10..14.json` · `_lite11..14.json` · `_shared.json` | 정본에서 **읽기만** 해서 떼낸 대조용 사본 |

**정본 미수정 증거**: `_spec\specs20.json`(2026-09-11 15:51) · `_spec\specs20_v3.json`(15:53) ·
`dienum.json`(09-10 23:40) · `distruct.json`(09-10 23:07) — 전부 내 첫 파일 쓰기보다 이전 시각이다.

### 이번에 밟은 함정 1건 (다음 세션용)
`/specs[11]/knobs[12]` 로 적었다가 `applypatch --dry` 가 **「`old` 가 그 행 어느 키에도 없다」로 반려**했다.
원인 = v3 `knobs` 배열을 **눈으로 세다 한 칸 밀린 것**(실제는 `knobs[11]`).
★`applypatch` 의 `old` 대조가 이 실수를 조용히 넘기지 않고 잡아 줬다 — **이 계약의 실효를 내가 직접 확인한 셈**이다.
교훈: 배열 인덱스는 눈으로 세지 말고 **스크립트로 뽑아라**(그래서 `mkpatch.py` 를 만들었다).
