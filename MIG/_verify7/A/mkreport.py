# -*- coding: utf-8 -*-
u"""`_verify7/A/REPORT.md` 생성기 (7차 배치A). `auditrounds.py` 가 보고서 존재를 커버리지로 세므로 산출물이다."""
import io, os

HERE = os.path.dirname(os.path.abspath(__file__))
TEXT = r'''# 7차 반증검증 — 배치 A (`00`~`04`) 산문 보고

> 본 보고는 부수적이다. **기계 적용분 = `_verify7/A/patch.json`** (정정 25 · ev상향 6, `applypatch.py 7 --only A --dry` = **25/25 · 6/6 성공, 실패 0**).
> 게임 0.5.8 · 정본 해시 확인: `_spec/specs20_v3.json` = `dab6d7c803e1be63` (도시에 §0 과 일치) · `_spec/specs20.json` = `6320fa42c1107160`.

---

## §0 판정 어휘 요약 (도시에 §5-b ①)

| 항목 | 판정 | 범위 |
|---|---|---|
| G12 8건(`consts.src_line`) | **오탐 8/8** — 명세는 옳다 | `srclinecheck.py` 현 판정식 기준. 아래 §2 에 줄 원문으로 전수 반증 |
| G13 7건(`knobs.where`) | **실오류 7/7** — 전부 고침 | 줄번호 4건 · 인용 문면 3건 |
| G13 이 **못 잡은** 같은 축 | **실오류 8건 추가 발견** | 백틱 IR 인용이 없는 `where` 는 줄 존재만 검사되기 때문 |
| `mem.dir`(§4-b) | **불일치 0** (담당 5함수 107행 중 89행 검사, 18행은 범위밖/접힘) | 오프셋 기준 대조. 베이스 타입은 안 봄 — §4 에 검사기 설계 |
| `consts.kind`(§4-b) | **분류오류 3건** + **축 자체가 patch 불가** | `kind` 는 v2 에 없는 파생 필드 |
| `sig.params.role`(§4-b) | **규약 불일치 1건**(specs[2] 가 sret 를 안 셈) · 문면 오탐 2건 | 「안 씀/전달만」 주장은 5함수 32행 전부 IR 과 일치 |
| `ev>=4` 내리기 | **6행** 실행으로 내림 | `A7_o1.tsv` |
| 명세 전문 반증 | **판정반전 1건**(`is_minion` → `is_any_type_minion`) · 소스 표기 2건 확정 | |

---

## §1 실제로 실행한 것 (도시에 §5-b ②) — 명령줄 그대로

```bash
cd /c/tfm2mods/MIG

# (1) 정본 스탬프 확인
python -c "import hashlib,io;print(hashlib.sha256(io.open('_spec/specs20_v3.json','rb').read()).hexdigest()[:16])"
python -c "import hashlib,io;print(hashlib.sha256(io.open('_spec/specs20.json','rb').read()).hexdigest()[:16])"

# (2) IR 사슬 조회 도구(이번에 만든 것 — 전부 _verify7/A/ 안)
python -X utf8 _verify7/A/irchain.py m04.ll 43967 44120 --grep "switch|icmp|store|call|phi|br "
python -X utf8 _verify7/A/irchain.py m04.ll 44240 44330 --grep "= (add|sub|mul|udiv|phi|icmp|select|load|call|getelementptr)"
python -X utf8 _verify7/A/irchain.py m12.ll 34867 34976
python -X utf8 _verify7/A/irchain.py m10.ll 33864 33900
python -X utf8 _verify7/A/dbgid.py m05.ll 44402 44396 44405 44413 44410 44412 44407 44411 44409 44393 44417 44416

# (3) 원문 확인(사슬이 아니라 그 줄 자체)
sed -n '44048,44060p' C:/tfm2mods/_gaibc/m04.ll
sed -n '40015,40105p' C:/tfm2mods/_gaibc/m05.ll
awk 'NR>=43967 && NR<=44365 && (/usub.sat/ || /150000/ || /isqrt/ || /100/ ) {print NR": "$0}' C:/tfm2mods/_gaibc/m04.ll
awk 'NR>=40031 && NR<=40100 && (/phi i64/ || /select i1/ || /icmp ult/ || /icmp sgt/) {print NR": "$0}' C:/tfm2mods/_gaibc/m05.ll
awk 'NR>=60440 && NR<=60460 {print NR": "$0}' C:/tfm2mods/_gaibc/m04.ll
awk 'NR>=34070 && NR<=34095 {print NR": "$0}' C:/tfm2mods/_gaibc/m10.ll
awk '(NR>=44060 && NR<=44092) {print NR": "$0}' C:/tfm2mods/_gaibc/m04.ll | grep -v "#dbg_"

# (4) G12 후보 목록의 정체를 줄 원문까지 찍어 전수 반증
python -X utf8 _verify7/A/g12probe.py | tee _verify7/A/oracle/A7_g12probe.txt

# (5) §4-b 무검사 축 — 이번에 만든 시제품 검사기 2개
python -X utf8 _verify7/A/memdir.py   0 1 2 3 4 | tee _verify7/A/oracle/A7_memdir.txt
python -X utf8 _verify7/A/paramuse.py 0 1 2 3 4 | tee _verify7/A/oracle/A7_paramuse.txt

# (6) 소스 줄 길이(rmeta SourceMap) — L42/L50 복원
python -X utf8 rmeta_srcmap.py game_ai "old\\attack_nexus.rs" 30 55

# (7) tcx 정본 조회
python -X utf8 tcxdict.py --enum SubPlan
python -X utf8 tcxdict.py --enum EntityType
python -X utf8 tcxdict.py Minion

# (8) SDK 실행 오라클 (한 프로세스 = 이 배치의 전 케이스. TLS 메모 함수가 아님을 확인하고 합쳤다)
sh _verify3/build.sh "C:/tfm2mods/MIG/_verify7/A/oracle/A7_o1.rs"
"C:/Users/jungs/AppData/Local/Temp/tfm2_spanprobe/A7_o1.exe" | tee _verify7/A/oracle/A7_o1.tsv

# (9) 제출 전 적용 가능성 사전 검증
python -X utf8 applypatch.py 7 --only A --dry
```

`setting_ok true width=960000 height=960000 tps=60 champ_radius=10000 visible=130000` — 오라클 세팅 무결.

---

## §2 G12 8건은 **전부 오탐이다** (도시에 §4 의 주 표적)

도시에는 이 8건을 「실제 후보 = [...]」와 함께 준다. **그 후보 목록이 잡음이다.**
`srclinecheck.py` 의 판정식은 「그 리터럴이 들어 있고 **그 줄 자체에 `!dbg` 가 붙은** 명령」만 센다. 세 가지가 새어 나간다.

1. **`switch` 의 case 라벨 줄에는 `!dbg` 가 없다.** 종결자의 `!dbg` 는 닫는 `]` 줄에 붙는다.
   ```
   44051:   switch i32 %43, label %95 [
   44052:     i32 1, label %44        <- 여기에 !dbg 가 없다 = G12 가 못 본다
   44053:     i32 2, label %57
   44054:   ], !dbg !56416            <- !56416 = abstract_input.rs:293
   ```
2. **`phi` 의 상수 인입은 사슬이 `line 0`** 이라 도구가 스스로 버린다(초판 오탐 교훈으로 넣은 필터).
   `m05.ll:40043 %32 = phi i64 [ 0, %45 ], [ -10, %46 ], [ 80, %37 ], [ 200, %22 ]` -> `action_score.rs:0`.
3. **리터럴 정규식이 SSA 레지스터와 오프셋 숫자까지 잡는다.** `(?<![\w.\-])5(?![\w.])` 는 `ptr %5` 에 걸리고,
   `16` 은 `getelementptr .. i64 16` 에 걸린다. 그래서 「실제 후보 = [36, 37]」 같은 목록이 나온다.

`_verify7/A/g12probe.py` 로 8건의 후보가 나온 **줄 원문**을 전수 확인했다(`oracle/A7_g12probe.txt`). 결론:

| 게이트 지적 | 주장 | 진짜 근거 | 후보 목록의 정체 | 판정 |
|---|---|---|---|---|
| `[00] consts[3]` 1 | 293 | switch `i32 1, label %44`(m04.ll:44052) · 종결자 `!dbg !56416` = **293** | `icmp eq i64 %70, 1`(310, 같은 행 meaning 이 이미 언급) + `%1`/함수호출 | **오탐** |
| `[00] consts[4]` 2 | 293 | switch `i32 2, label %57`(44053) · **293** | 281 의 bounds-check 2 · 328 의 visible_state len 2 | **오탐** |
| `[01] consts[3]` 2 | 528 | switch case `i64 2, label %37`(40033) -> arm 블록 `%37` 종결자 `!dbg !44410` = **528** | 522 bounds-check · 523 `%2` · 530·542 의 `ult .., 2` | **오탐** |
| `[01] consts[5]` 4 | 530 | switch case `i64 4, label %38` -> arm 블록 `%38` `!dbg !44411` = entity.rs:1378 <- **530** | 542 의 `icmp eq i64 %29, 4`(두 번째 is_jungle) | **오탐** |
| `[01] consts[8]` 1 | 536 | switch case `i64 1, label %46` -> arm 블록 `%46` `!dbg !44413` = **536** | `ptr %1`(522) | **오탐** |
| `[01] consts[10]` 0 | 536 | default 블록 `%45` `!dbg !44413` = **536** | phi(line 0) · 548 `icmp eq %27, 0` · 542 `select .. 5, 0` | **오탐** |
| `[02] consts[1]` 5 | 42 | phi `[ 5, %49 ]`(34973, `!dbg` 없음). 블록 `%49` 종결자 = L41, **소스 L42** 는 줄길이로 확정(§7) | `ptr %5`(36·37) · `[5 x ptr]` | **오탐** |
| `[02] consts[4]` 16 | 50 | phi `[ 16, %55 ]`(34973). L50 = 58자 복원 ±0 | `i64 %16`(36) · `gep .. i64 16`(37) | **오탐** |

**만약 도시에의 「실제 후보」를 믿고 8건을 고쳤다면 전부 판정 반전 오류가 됐을 것이다.**
이 라운드에서 지시가 낸 가장 큰 오류다(§5 참조).

**G12 를 고치는 법(다음 라운드 제안)** — 세 가지면 된다.
1. `switch` 를 만나면 case 라벨 줄의 리터럴을 **종결자(`]` 줄)의 `!dbg`** 에 귀속시킨다.
2. `phi` 의 상수 인입은 **그 인입 블록의 종결자 `!dbg`** 에 귀속시킨다(현행은 line 0 으로 버려진다).
3. 리터럴 매칭에서 **`%`/`!` 로 시작하는 토큰과 `getelementptr`/`align`/`dereferenceable` 의 인자**를 제외한다.
   이것만 해도 이번 8건의 후보 목록이 전부 비고 「판정보류」로 떨어진다.

---

## §3 G13 7건 — **전부 실오류. 그리고 같은 축에서 8건을 더 찾았다**

### 3-a. 게이트가 잡은 7건 (전부 고침)

| 행 | 옛 값 | 확정 값 | 확인한 줄 원문 |
|---|---|---|---|
| `[00] knobs[0]` | m04.ll **44033** | **44025** | `%31 = icmp ugt i64 %30, 4, !dbg !56400` (entity.rs:1701 <- abstract_input.rs:286) |
| `[00] knobs[1]` | m04.ll **44283** `llvm.usub.sat.i64(%205, 150000)` | **44313** `llvm.usub.sat.i64(i64 %205, i64 150000)` | 담당 범위 전체에서 `usub.sat` 는 44313 한 곳뿐. 44283 은 `load i32`(entity.rs:1511) |
| `[03] knobs[1]` | 줄은 맞고 인용이 `icmp ugt %85, %17` | `icmp ugt i64 %85, %17` | m10.ll:34144 원문에 `i64` 가 있다 |
| `[03] knobs[3]` | `icmp ugt %76, 2` | `icmp ugt i64 %76, 2` | m10.ll:34081 |
| `[03] knobs[4]` | `icmp ugt %76, 4` | `icmp ugt i64 %76, 4` | m10.ll:34086 |
| `[04] knobs[5]` | m04.ll **60446** | **60449** | 60446 은 `#dbg_value`. 실제 `%23 = icmp slt i64 %22, -3000` 은 60449 |
| `[04] knobs[6]` | m04.ll **60451** | **60455** | 60451 은 빈 줄. 실제 `%27 = icmp slt i32 %26, -2` 는 60455 |

`[03]` 3건은 **줄번호가 옳고 인용에서 타입만 빠진 것**인데도 게이트가 걸린다 —
`whereline.norm()` 이 SSA 번호만 지우고 타입은 남기기 때문이다. 이건 게이트가 맞다(인용은 IR 원문이어야 한다).

### 3-b. 게이트가 **못 잡은** 같은 축 8건 (`found_by: new`)

`whereline.check_spec` 은 **백틱 IR 인용이 있는 행만** 문면을 대조하고, 없으면 「줄이 존재하는가」만 본다.
`.ll` 이 1,500만 줄이라 **아무 줄번호나 존재한다** => 사실상 무검사다. 그 구멍에서 나온 것들:

| 행 | 옛 값 | 확정 값 |
|---|---|---|
| `[00] knobs[2]` | `44251~44254, casting==0` | **44245~44247** `icmp eq i32 %158, 0`. 옛 인용은 `entity.rs:1511~1512` 의 **radius_mult==0** 자리 = 다른 조건 |
| `[00] knobs[3]` | `44238·44240` | **44266·44268** `add nsw i64 %168, 100` / `udiv i64 %172, 100` (44238~44240 은 `#dbg_value` 뿐) |
| `[01] knobs[0]` | `40016, phi [200, %22]` | **40043** |
| `[01] knobs[1]` | `40016, phi [80, %37]` | **40043** |
| `[01] knobs[4]` | `40016, phi [-10, %46]` | **40043** (40016 = `icmp eq ptr %20, null`, champ null 검사) |
| `[01] knobs[2]` | `40045, select **false-arm**` | **40066** `select i1 %43, i64 20, i64 40` — 20 은 IR **true-arm** |
| `[01] knobs[3]` | `40045, select **true-arm**` | **40066** — 40 은 IR **false-arm** |
| `[01] knobs[5]` | `40085, select ? 5 : 0` | **40091** `select i1 %55, i64 5, i64 0` |

`[01] knobs[2]·[3]` 은 줄번호뿐 아니라 **arm 방향이 뒤집혀** 있었다. 같은 명세의 `consts[6]/[7]` 이
「then/else 반전형 확정」이라 적고 있으니 **자기모순**인데 `G1` 은 `knobs.where` 를 보지 않는다.
(노브를 바꿔 쓰려는 사람이 이 표를 믿으면 20 과 40 을 반대로 넣는다 — 실사용 영향이 있는 오류다.)

**G13 강화 제안**: 백틱 인용이 없는 `where` 에 대해서도 **그 줄(±3)의 `!dbg` 사슬에 `where` 가 적은 `.rs:줄` 이
들어 있는지**를 검사하면 위 8건이 전부 걸린다(전부 사슬이 전혀 다른 줄을 가리켰다).

---

## §4 §4-b 무검사 축 3개 — 표본 결과와 **검사기 설계**

### 4-a. `mem.dir` — 담당 5함수 전수, **불일치 0** (`memdir.py`, `oracle/A7_memdir.txt`)

107행 중 89행이 담당 IR 범위 안에서 관측됐고 **주장한 방향과 모두 일치**했다. 나머지 18행은
「범위밖/접힘」(gep 오프셋 0 접힘 · `has_line_defense_threat` 같은 aux 범위)이다.
유일한 `불일치`(`[04] mem[26]` BrainMinionParameter+0x20)는 **내 도구의 오탐**이다 — 그 행은 aux 범위
(m04.ll:60453~60454, `load i32` = 읽기)에 있는데 도구가 담당 범위 안의 다른 구조체 offset 32 store 와
충돌시켰다. 실제 IR 을 열어 `r` 이 맞음을 확인했다.

=> 6차 교훈의 **반대 사례**다. 계측기를 붙였다고 오류가 나오는 게 아니다 — `mem.dir` 은 이미
`tcxaudit`·`offset_of!` 로 덮인 영역이라 새 정보가 없었다.

**검사기 설계(G14 제안)**: 위 시제품에 두 가지를 더해야 게이트가 된다.
1. **베이스 타입 결합** — 지금은 오프셋 숫자만 본다. gep 사슬의 뿌리 SSA 를 `define` 인자·`dereferenceable(N)` 으로
   타입 추정해 `mem[].base` 와 대조해야 `OperationData+0x0` 과 `Entity+0x0` 이 안 섞인다.
2. **aux 범위 지원** — `spec.aux[]`(fnparts 범위)를 같이 스캔한다. 그러지 않으면 `[04]` 처럼 27행 중 9행이
   「범위밖」으로 빠져 3분의 1이 무검사로 남는다.

### 4-b. `consts.kind` — **분류오류 3건 + 축 자체가 patch.json 으로 쓸 수 없다**

**가장 중요한 발견**: `kind` 는 v2 정본(`specs20.json`)에 **존재하지 않는 파생 필드**다.
`mkspec3.py:329` 가 `meaning` 문자열의 낱말로 정한다:

```
태그    <- "태그" | "판별자" | "variant"
센티넬  <- "센티넬" | "니치" | "0xff" | "MAX"     (태그 키워드가 없을 때만)
인덱스  <- "인덱스"                               (위 둘이 없을 때만)
임계    <- 그 외 전부
```

=> **한 낱말만 스쳐도 분류가 튄다.** 실제로 배치 A 의 3건이 그 사고였다:

| 행 | 현재 | 맞는 값 | 왜 튀었나 |
|---|---|---|---|
| `[00] consts[0]` 2 | 인덱스 | **임계** | meaning 에 "team **인덱스** bounds-check" 라는 낱말. 이 `2` 는 첨자가 아니라 `icmp ult .., 2` 의 **비교 상한**이다. 같은 사실을 적은 `[01] consts[0]`·`[02] consts[0]` 은 이미 `임계` — 코퍼스가 갈려 있었다 |
| `[00] consts[2]` -1 | 태그 | **센티넬** | "Option 의 None **판별자**" 가 니치보다 먼저 걸렸다. 같은 사실을 `[03] consts[4]`·`[05]`·`[15]`·`[16]`·`[18]` 은 전부 `센티넬` 로 쓴다 |
| `[04] consts[3]` 1 | 인덱스 | **임계** | meaning 의 "같은 리터럴이 549줄의 적팀 **인덱스** 산출에도 쓰임" 이라는 **부수 언급**. 이 행의 `src_line` 은 578(= `near_enemy_champion >u 1`)이고 그 자리 용도는 비교 임계다 |

**규칙화**: `kind` 는 **`src_line` 이 가리키는 그 용도**를 기술해야 한다(다른 줄의 부수 용도가 아니라).
같은 형태가 `[06] consts[0]`(버전 게이트 `version<2` 인데 `인덱스`)에도 있다 — **배치 B 몫**이라 손대지 않았다.

**검사기 설계(G15 제안)** — IR 로 기계 판정 가능하다:

| kind | IR 조건 |
|---|---|
| `태그` | 그 리터럴이 `switch` case 이거나, `!range` 가 붙은 판별자 load 와 `icmp eq` 로 비교된다 |
| `센티넬` | `icmp eq` 상대가 니치 판별자(`tcxdict --enum` 의 니치 범위 밖 값)이다 |
| `인덱스` | 그 값이 `getelementptr` 의 인덱스 피연산자이거나 인덱스 산술(`sub 1, %t`)의 피연산자다 |
| `임계` | `icmp ult/ugt/slt/sgt` 의 비교 상대이거나 산술 계수다 |

단 **`kind` 를 patch.json 으로 고칠 수 없다**는 구조를 먼저 풀어야 한다. 둘 중 하나다 —
(a) v2 `constants[]` 에 `kind` 를 실제 필드로 승격, 또는 (b) `applypatch` 가 파생 필드에 대해 거부 대신
「파생 규칙 입력(meaning)을 고쳐라」를 안내. 이번엔 (b) 를 손으로 해서 `meaning` 의 낱말을 고쳤다.

### 4-c. `sig.params.role` — 「안 씀/전달만」 주장 32행 **전부 IR 과 일치**, 규약 불일치 1건

`paramuse.py` 로 `define` 헤더의 파라미터 속성(`readnone`/`readonly`)과 본문 SSA 등장 횟수를 대조했다.

- **규약 불일치**: `[02] sub_plan` 의 인자표는 **sret 를 세지 않는다**(`i=1` 이 `self`). 그런데 `[00] ult` 는
  **센다**(`i=1` 이 `(sret)`). IR 은 sub_plan 도 `void @...(ptr sret %0, ptr %1 self, ...)` 라
  **`self` 는 IR 인자 2번**이다. 두 명세가 같은 `i` 컬럼에 다른 규약을 쓰고 있다.
  => 재구현자가 `i` 를 IR 인자 번호로 읽으면 `[02]` 에서 한 칸씩 밀린다. (본 배치는 `[02]` 의 인덱스
  전면 재배치가 다른 경로를 깨뜨릴 수 있어 **patch 로 바꾸지 않고 보고만 한다** — 축 규약을 먼저 정해야 한다.)
- **문면 오탐 2건**(내 도구 쪽): `[02] p[4] data` 의 "blackboard(0x10) **미사용**" 은 인자가 아니라 **필드**에 대한
  진술인데 「미사용」 낱말에 걸렸다. `[03] p[0] version` 의 「call 밖 사용」은 실체가
  `m10.ll:33874 store i64 %0, ptr %11` = **by-value 인자의 스택 spill**(`#dbg_declare` 용)이었다.
  후자는 다음 라운드가 또 팔 자리라 그 사실을 `role`(v2 `note`)에 박아 넣었다(patch 에 포함).
- 나머지는 전부 일치: `[01] _rnd`=`readnone` · `[02] rnd/_team_plan/_debug`=`readnone` · `[02] version` 은
  속성은 없지만 **본문 등장 0회**(「한 번도 참조되지 않음」 확증) · `[04] _version` 등장 0회 · `[04] _debug`=`readnone`.

**검사기 설계(G16 제안)**: 시제품에 (1) **sret 정렬 자동 보정**(구현함) (2) 「미사용」류 낱말이 **인자에 대한
진술인지** 가르는 스코프 판정(괄호 안 필드 언급 제외) (3) `store %param, alloca` + `#dbg_declare` 패턴을
「전달만」으로 인정 — 이 셋을 넣으면 오탐 0 이 된다.

---

## §5 내 지시(도시에)의 오류 (도시에 §5-b ③)

1. **§5 의 `patch.json` 예시가 도구 계약과 다르다.**
   도시에는 `{"round": "r7", "entries": [{path, old, new, why}]}` 를 보여주는데,
   `applypatch.py` 는 `{"round": 7, "errors": [{path, kind, old, new, evidence, behavior_change, found_by}],
   "ev_up": [...], "brief_errors": [...]}` 만 읽는다(`applypatch.py:20~40` 계약 주석 + `main()` 의
   `pj.get("errors")` / `pj.get("ev_up")`).
   **예시대로 냈으면 `entries` 가 통째로 무시돼 0건 적용이었다** — 도시에 §5 가 경고하는 바로 그
   「5차 317행 유실」을 지시 자신이 재현시킬 뻔했다.
2. **§5 가 `ev_up` 배열의 존재를 알려주지 않는다.** 「`ev` 를 직접 쓰지 마라」만 적혀 있어,
   그 말을 따르면 ev 하향을 **보고할 수단이 없다**. 실제 수단은 `ev_up`(`from`/`to`/`evidence`)이고
   도구가 근거 문면을 대신 붙인다(`apply_evup`).
3. **§4 의 G12 8건 「실제 후보」가 잡음이다**(§2 전수 반증). 지시를 그대로 따르면 8건 전부
   판정 반전 오류가 된다. 「게이트가 실제 후보를 같이 준다」가 그 신뢰도를 과장한다.
4. **§4 가 요구한 `consts.kind` 는 patch.json 으로 쓸 수 없는 파생 필드**인데 그 말이 없다(§4-b).
   첫 제출에서 `/specs[i]/consts[j]/kind` 3건이 전부 「현재 None」으로 거부됐다.
5. **§4 머리 건수가 오독을 부른다.** 「이 배치 몫만 추렸다」 아래에 `[G12 ...] 47건` / `[G13 ...] 8건`
   이라는 **전역 합계**가 적혀 있다. 실제 나열은 8건/7건이고 지시 본문의 「15건」이 맞다.
6. **§1 의 `ev>=4(미실행)` 집계가 「오라클로 내릴 수 있는 행」을 과대평가한다.** `00`=9 · `03`=15 인데
   대부분이 **담당 함수 밖**(`_gcbc/g06.ll` · `m15.ll` 전투 위협모델 등)의 knobs 라 이 배치 범위의 실행 대상이 아니다.
   실제로 내린 것은 배치 A 전체에서 6행이다.

### 5-b. 도구 쪽 결함 (지시가 아니라 파이프라인)

7. **`applypatch.already()` 가 대상 필드가 아니라 명세 전체를 뒤진다.**
   `[03] knobs[3]·[4]` 의 새 인용 `icmp ugt i64 %76, 2` 가 **같은 명세의 `consts[2].meaning`** 에
   이미 있다는 이유로 「이미 적용」으로 분류돼 **조용히 버려졌다**(첫 `--dry` 실측 2건).
   이번엔 `new` 에 괄호를 포함시켜 피했지만, `already()` 는 **그 경로가 가리키는 값**으로 좁혀야 한다.
   (「조용한 no-op 금지」가 이 도구의 설계 목적인데 그 목적을 `already()` 가 뚫고 있다.)

---

## §6 판정 반전 (도시에 §5-b (4) — 반전도 오류로 센다)

1. **`[01] closed[1]` — tag 1 경로의 술어**
   ~~is_minion 으로 추정 · 미확정~~ -> **`is_any_type_minion`(entity.rs:1260) 확정**.
   오라클 `A7_o1.tsv PRED`: 판별자 1 을 주입하면 `is_any_type_minion=true` 인데
   `is_minion(line)` 은 **라인마다 갈린다**(Top=true / Mid=false / Bottom=false)
   => LineType 로드가 **0개**인 `m05.ll:40031` switch 와 양립 불가.
   `history[4]` 가 MIR 로 이미 확정해 둔 결론을 **실행으로 독립 확인**한 것이고, `closed[1]` 문면만
   1차 값으로 남아 있었다(G7/G8 이 `closed` 를 안 본다).
2. **`[01] knobs[2]/[3]` 의 select arm 방향**이 `consts[6]/[7]` 과 반대였다 -> 반전 정정(§3-b).
3. **`[00] mem[24]/[25]` 의 슬롯 이름** ~~(이름 미상)~~ -> `linear_move_speed`(+0xf8) / `on_caster`(+0x118).
   `history[3]` 가 「1~6차는 Arc<dyn> 이라 원리적으로 불가로 포기」를 뒤집어 확정해 둔 값인데
   **mem 표에만 옛 값이 남아 있었다**.

---

## §7 새로 확정한 사실 (반증이 아니라 보강)

1. **`old\attack_nexus.rs` L42 = `      return SubPlan::Recall(RecallSubPlan::default());`** (±0)
   rmeta SourceMap 실측 L42 = 56자 -> 내용 55 = indent 6 + 49자. `RecallSubPlan` 은 tcx 기준 **0B ZST** 라
   IR 에 페이로드 store 가 없다(태그만 `phi [ 5, %49 ]` -> store). 오라클에서 이 표기가 **컴파일되고 태그 5** 를 만든다.
   같은 방식으로 L50 = `SubPlan::AttackNexus(AttackNexusSubPlan::default())`(기존 기재)도 **태그 16** 으로 실행 확증.
   => `logic` 이 인자 없는 `SubPlan::Recall` 로 적고 있던 것을 정정(G6 계열).
   부수로 L41=59 · L43=6 · L47=30 · L48=138 · L49=13 · L51=6 · L52=4 가 전부 복원 문자열과 ±0 로 맞아
   **L47 의 `if has_enemy_twin_tower {` 긍정형**(29자)이 줄길이로도 확증됐다.
2. **미니언 계수 `-10` 실행 확증** — 6차까지 오라클에 미니언 케이스가 **한 번도 없었다**
   (`A6_o3` 은 Nexus/Tower/Jungle/Champion 뿐이라 `consts[8]`·`consts[9]`·`knobs[4]` 가 ev4 로 남아 있었다).
   `A7_o1.tsv MINION`: 판별자 1 주입 -> `game = -4960 = min(-10, -10*496/1)` **MATCH**,
   대립가설 `coef = 0` 기각(그 경우 0 이어야 한다).
3. **`Option<usize>` = 16B · `Some.discr = 1` / `None.discr = 0`** 실측(`Some(0)` 도 1)
   => `[03] consts[6]` 실행 확증 + `history[9]` 의 「`Some(0)` 도 `is_some()` 이라 위협으로 센다」와 정합.
4. **`[00] knobs[6]` 은 옳다**(반증 실패): m04.ll:44068 = `gep .. i64 280`(0x118) -> 44070 `tail call .. i1`
   = abstract_input.rs:296(on_caster), 44085 = `gep .. i64 248`(0xf8) -> 44087 `tail call { i64, i64 }`
   = 310(linear_move_speed). `shared.EffectType_vtable` 과 완전 일치.
5. **`[04] knobs[0]~[3]` 의 IR 줄은 전부 정확하다**(58583 · 58635 · 58636 · 58200 전수 확인).
   틀린 것은 aux 범위(`has_line_defense_threat`, 60400~)를 인용한 `knobs[5]/[6]` 뿐이었다
   => **오류는 담당 범위 밖을 인용한 자리에 몰린다**는 가설이 이 배치에서 성립한다
   (`[00] knobs[2]/[3]` · `[01] knobs[0]~[5]` 도 인라인된 다른 파일 프레임을 가리키는 자리였다).

---

## §8 하지 **않은** 것과 그 이유 (범위 명시)

- **`[04] knobs[5]/[6]`(-3000 / -2) 의 `ev` 는 내리지 않았다.** `history[5]` 가 3차에 「점등 성공」했다고
  적지만 그것은 **OR·AND 구조**를 켠 것이고, `-3000` / `-2` 의 **경계를 쓸어본 기록은 없다**.
  경계 스윕 없이 ev2 로 내리면 과잉 주장이다. => **미탐색**(방법은 있다: `Blackboard::minion_state` 를
  직접 써서 from_mid 를 -2999/-3000/-3001 로 스윕).
- **`[02] sig.params` 의 `i` 컬럼 재배치**는 하지 않았다(§4-c). 인덱스를 한 칸 미는 변경이라
  다른 배치·도구의 경로 참조를 깨뜨릴 수 있고, **축 규약을 먼저 정해야** 한다.
- **`[06] consts[0]` 의 같은 형태 분류오류**는 **배치 B 몫**이라 손대지 않고 보고만 했다.
- **오라클 프로세스 분리**: `TEMPLATE.rs` 함정 (3)(TLS 메모)에 해당하는 함수(`check_kill_die_tick` 계열)는
  이번 프로브에 **넣지 않았다.** 넣었다면 케이스당 프로세스 1개가 필요했다.
  `calculate_jungle_action_score` 는 순수 함수(store 0개)라 한 프로세스에 합쳤다.
- **오라클 프로브의 자잘한 결함**: `A7_o1.rs` 의 `orig = tag64(&e)` 가 `Entity+0x0`(TeamType)을 읽어
  0 을 돌려줬다. 그래서 「Tower(원본)」 라벨 행은 **실제로는 tag 0(None)** 이고 원복도 2 가 아니라 0 으로 됐다
  (`EntityType::None` 은 페이로드가 없어 드롭 오해석은 없다). **미니언 판정은 영향 없다**
  (tag 를 명시적으로 1 로 써서 쟀고 대립가설을 기각했다). 라벨만 오해 소지가 있어 적어 둔다.

---

## §9 산출물

```
_verify7/A/patch.json          기계 적용분 (정정 25 · ev상향 6, --dry 25/25 · 6/6)
_verify7/A/REPORT.md           이 문서 (생성기 = _verify7/A/mkreport.py)
_verify7/A/irchain.py          IR 줄범위 -> !dbg inlinedAt 사슬 펼치기 (조회)
_verify7/A/dbgid.py            !dbg 메타 id -> 파일:줄 사슬 (조회)
_verify7/A/g12probe.py         G12 후보의 줄 원문 전수 확인 (G12 반증용)
_verify7/A/memdir.py           §4-b `mem.dir` 검사기 시제품 (G14 후보)
_verify7/A/paramuse.py         §4-b `sig.params.role` 검사기 시제품 (G16 후보)
_verify7/A/oracle/A7_o1.rs     SDK 실행 오라클 소스
_verify7/A/oracle/A7_o1.tsv    그 출력
_verify7/A/oracle/tmpl.rs      TEMPLATE.rs 계열 공통 셋업(6차분 복사)
_verify7/A/oracle/A7_memdir.txt / A7_paramuse.txt / A7_g12probe.txt
```

도구 3개(`memdir`/`paramuse`/`g12probe`)는 **시제품**이다. MIG 루트로 승격하려면
§4 의 설계 항목을 채우고 `mktools.py` 를 다시 돌려야 한다(METHOD_MAP §0 규칙).
'''

io.open(os.path.join(HERE, "REPORT.md"), "w", encoding="utf-8").write(TEXT)
print("wrote", os.path.join(HERE, "REPORT.md"), len(TEXT), "chars")
