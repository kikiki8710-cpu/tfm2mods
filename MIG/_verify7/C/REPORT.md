# 7차 반증검증 — 배치 C (담당 `10`~`14`) 보고

> **본 보고는 `_verify7/C/patch.json` 이다.** 이 산문은 부수적이며, 도시에 §5-b 의 4항목
> (판정 어휘 / 실행한 명령줄 그대로 / 내 지시의 오류 / 판정 반전도 오류로 셈)을 담는다.
>
> 사전 검증: `python -X utf8 applypatch.py 7 --only C --dry` → **정정 21/21 · ev상향 2/2 · 실패 0**

| 산출물 | 내용 |
|---|---|
| `_verify7/C/patch.json` | 정정 **21** · ev상향 **2** · brief_errors **6** |
| `_verify7/C/kind_proposal.json` | `consts[].kind` 37행 재분류 제안 (patch 로는 못 냄 — §3) |
| `_verify7/C/oracle/` | 오라클 소스 2 · 로그 · 신설 계측기 4 |

---

## §0 먼저 — **내 지시(도시에)의 오류**부터

### D1 ★★도시에의 신선도 계약이 **자기 자신을 안 본다** (최대 발견)

§0 은 `_spec/specs20*.json` 해시만 확인시킨다. 나는 그대로 돌렸고 **일치했다**:

```bash
python -c "import hashlib,io;print(hashlib.sha256(io.open('_spec/specs20_v3.json','rb').read()).hexdigest()[:16])"
# dab6d7c803e1be63   ← 도시에 §0 과 일치
```

그런데 **도시에 본문은 `mkdossier.py` 의 산출물**이고 그 도구가 라운드 중에 바뀐다.

```bash
stat -c '%y %n' mkdossier.py _verify7/DOSSIER_C.md _spec/specs20_v3.json
# 2026-09-11 18:09:28  mkdossier.py
# 2026-09-11 17:58:27  _verify7/DOSSIER_C.md      ← 디스크의 현재 판
# 2026-09-11 17:07:14  _spec/specs20_v3.json
```

내가 받아 읽은 판은 본문에 `이 파일 생성 = 2026-09-11 17:41:21` 이라고 적혀 있었다.
**`_spec` 해시는 둘 다 같은데 지시문은 네 곳이 달랐다**(전부 배치 A 가 고친 것):

| 자리 | 17:41 판(내가 읽은 것) | 17:58 판(현재) |
|---|---|---|
| §4 머리말 | 「G12 47건」이 배치 몫처럼 읽힘 | **「이 배치 몫 = 15건」 명시** |
| §4 | (없음) | **「게이트의 실제 후보를 믿지 마라 — 배치A 가 8건 전수 오탐 확인」** |
| §5 | `{"entries":[{path,old,new,why}]}` | **`{"errors":[...], "ev_up":[...]}`** (실제 계약) |
| §5 규칙 | (없음) | **「`consts.kind` 는 파생 필드라 `errors[]` 로 못 쓴다」** |

17:41 판대로 보고했으면 `patch.json` 은 **0건 적용**이었다(5차 317행 유실과 같은 형태).
⟹ **§0 표에 도시에 자신의 sha256 과 `mkdossier.py` 의 sha256 을 넣어라.**
이번엔 내가 계약을 스스로 파헤쳐 같은 4건을 독립 재발견해 손해가 없었지만, **그건 우연이다.**

### D2 §1 `ev≥4` 의 **분모가 안 적혀 있다**
`mkdossier.py:49 EV_FIELDS = ("mem","consts","knobs")` — `notes`/`open` 은 안 센다.
그래서 표는 `specs[10]` 을 `0` 으로 적지만 실제로 `notes[0]`·`notes[1]` 이 ev4 다.
**5차의 「ev 집계 스코프 미기재 → 배치B 오적발」과 같은 형태의 재발.**

### D3 §5 규칙에 아직 남아 있는 `"op": "append"`
`applypatch.py` 에 `op` 처리가 **전혀 없다**(`main()` 은 `errors`/`ev_up` 만 읽고
`apply_error` 에 append 분기가 없다). 그 지시대로 새 `notes` 를 내면 **조용히 무시**된다.

### D4 §4-b 와 §5 가 **한 파일 안에서 서로 반대**
§4-b 는 `consts.kind` 186행을 「아직 게이트가 없는 축」으로 표적 지정하는데,
§5 는 같은 파일에서 「파생 필드라 `errors[]` 로 못 쓴다」고 적는다.
§4-b 를 **「무측정 축」**으로 고치고, 진짜로 저장된 `mem.dir`·`sig.params.role` 과 **분리**해야 한다
(그 둘은 검사 가능하고 실제로 이번에 결함이 나왔다 — §2·§4).

---

## §1 G12(`consts.src_line`) — 담당 12건이 **전부 오탐**이었다

### 진단
`srclinecheck.py` 초판은 「한 줄 = 한 명령」을 전제하고 `!dbg` 없는 줄을 버린다.
그런데 LLVM 텍스트 IR 에서 **`switch` 는 여러 줄로 인쇄되고 `!dbg` 는 닫는 `]` 줄에만 붙는다**:

```llvm
  switch i8 %28, label %44 [      ; ← !dbg 없음
    i8 0, label %38               ; ← 케이스 값(= 태그 리터럴!) — !dbg 없음
    i8 2, label %38
  ], !dbg !36362                  ; ← !dbg 는 여기
```

⟹ **열거형 태그 상수는 거의 전부 switch 케이스로 내려가므로 구조적으로 전부 오탐된다.**
`invoke` 도 같다(인자 리터럴은 머리줄, `!dbg` 는 `to label ..` 줄).

### 실행한 명령줄과 결과
```bash
python -X utf8 _verify7/C/oracle/g12scan.py 10 15          # 담당분 재현 → 12건
python -X utf8 _verify7/C/oracle/dbgchain.py m13.ll 36347 36356 36362 36368 36374 36380
#  !36347 -> [('chat.rs', 12)]
#  !36362 -> [('runner.rs', 283), ('rule_scope.rs', 38), ('chat.rs', 12)]   ← 주장한 12 가 사슬에 있다
python -X utf8 _verify7/C/oracle/dbgchain.py m13.ll 21961 21980 21981 21982 21983 21990
#  !21961 -> [('rule_scope.rs',91), ('rule_scope.rs',101), ('handler.rs',429)]  ← 주장한 429 가 있다
python -X utf8 _verify7/C/oracle/dbgchain.py m08.ll 55639 55641 55643 55741 55882
#  !55643 -> [('ganker.rs',261), ('ganker.rs',54)]
```
**12건 전부 주장한 줄이 사슬에 실재한다.**

### ★상류와의 독립 재현 — 그리고 내 설계가 틀렸던 것
같은 라운드에 배치 A·D 가 **독립적으로** 같은 진단에 도달해 상류 `srclinecheck.py` 를 이미 고쳤다
(18:09 판, 전역 **47 → 10**). 재측정:
```bash
python -X utf8 specgate.py --gate G12          # 총 10건, 그중 배치 C(10~14) 몫 = 0
```
내가 만든 `srclinecheck2.py` 는 진단은 같았지만 **고치는 방향이 틀렸다** — 귀속시킨 `!dbg` 를
후보 집합에 그냥 더했다. G12 판정식이 「후보가 있는데 claim 이 없으면 불일치」라
**후보를 늘리면 오탐이 늘어난다**(실측 전역 10 → 19). 상류는 같은 함정을 이미 밟고 문서로 남겼다:
「귀속은 **약한 후보**다 — 구제에만 쓰고 기각에는 쓰지 않는다」.
⟹ `_verify7/C/oracle/srclinecheck2.py` 상단에 **STALE/폐기 마킹 + 이유**를 박아 두었다
(같은 오답을 두 번 만들지 않기 위해 파일은 남긴다).

### 판정 반전 (내 것 포함)
| # | 전에 | 지금 | 비고 |
|---|---|---|---|
| 1 | 도시에 §4: 담당 G12 12건이 결함 | **12건 전부 오탐** | 상류와 독립 재현 일치 |
| 2 | **내 중간 결론**: `specs[14] consts[4]` 252 → 265 | **철회 — 252 가 옳다** | ↓ |

②가 이번 라운드에서 **내가 만든 판정 반전**이다. 근거:
소스에는 `Top => if team==0 {2} else {16}` 이 **252 줄에 실재**한다(`logic` 확정).
IR 은 252/253/254 세 arm 의 비교를 `%57 = icmp eq i64 %10, 0`(m08.ll:94663) **하나로 CSE** 했고
그 `!dbg !55636` 이 **line 0**(ganker.rs:54 프레임)이라 강한 후보에서 252 가 사라졌을 뿐이다.
같은 줄의 `select i1 %57, i64 2, i64 16`(!55741)은 **252 로 정상 귀속**된다 —
즉 그 줄 자체는 IR 에 살아 있고 **조건 피연산자만 줄을 잃었다.**
「IR 에 안 남았다」를 「소스에 없다」로 읽는 것이 바로 이 게이트의 오탐 기제이고,
나는 한 번 그 함정에 빠졌다가 상류 교정판 재측정으로 빠져나왔다.

### G9(callees 오염) 2건 — **손확인 등급, 정상**
`line`/`nearest_enemy`/`position`/`team` 4개는 담당 두 함수에서 **전부 필드**다
(`self.line` = plan+0x28/+0x20 · `Tower.nearest_enemy` 태그 = Entity+0x88 `gep 136` ·
`info.position` = PlayerState+0x9c0 `gep 2496` · `info.team` = +0x930 `gep 2352`; 메서드 `call` 0건).
RUNBOOK §S5-c 가 말한 대로 **눈으로 보고 넘기는 것이 정상**인 등급이다. `kind:"오탐"` 으로 기록했다.

---

## §2 `mem.dir` — **무검사 축에서 실오류 4건** (신설 계측기 `memdir.py`)

### 계측기
`_verify7/C/oracle/memdir.py` — IR 의 `gep` 사슬을 접어 **SSA → 가능 오프셋 집합**을 만들고
`load`/`store`/`memcpy`/`memset` 을 모아 `dir=r` ↔ load, `dir=w` ↔ store 를 **필요조건**으로 대조한다.
만들면서 밟은 함정 3개(전부 주석에 남김):
1. `%\S+` 정규식이 **쉼표까지 삼켜** gep 를 하나도 못 잡았다(첫 판 전건 오탐 97/112).
2. `phi ptr` 은 들어오는 값의 **합집합**이어야 한다. 「전부 같을 때만」으로 하면
   `live_list.ptr` 처럼 두 갈래(epic/serpen)가 합류한 뒤 load 되는 필드를 통째로 놓친다.
3. 오프셋이 **정수 `phi` 로 선택**되는 경우(`phi i64 [416,..],[448,..],[384,..]` → `gep i8, ptr %c, i64 %16`)
   를 안 따라가면 `top/mid/bottom_tower(2)` **6행이 통째로 오탐**된다.

```bash
python -X utf8 _verify7/C/oracle/memdir.py 10 15     # 담당분 → 결함후보 15
python -X utf8 _verify7/C/oracle/memdir.py 0 20      # 전 20함수 → 결함후보 48 / mem 451행
```

### 판정 (담당 15건 삼분)
| 유형 | 건수 | 판정 |
|---|---|---|
| **실오류/오분류** | **4** | 아래 표 |
| 레지스터 전달 인자 | 2 | `사실 서술` — `i24 MainObjective` 는 `load` 가 아니라 `and`/`icmp` 로 읽힌다. **축 정의의 구멍** |
| 콜리 안 접근 | 9 | `사실 서술` — gep 는 있는데 **인자로 넘어가** 콜리가 읽고 쓴다(`grow_one`·`passive_plan`·`sub_plan` merge) |

**실오류 4건** (전부 patch.json 에 반영):

| 행 | 명세가 적은 것 | 실측 |
|---|---|---|
| `specs[10] mem[12]` `MobaMode+0x198 live_list.cap` `dir=r` · note「gep +408」 | 읽는다 | **`gep .. i64 408` 0건 · load 0건.** 408 은 `#dbg_value(.., DW_OP_plus_uconst, 408, ..)` = Vec **변수의 주소**로만 나온다. 실제 접근은 `len(+0x1a8)`·`ptr(+0x1a0)` 뿐 |
| `specs[10] mem[15]` `+0x1c8 serpen…cap` · note「gep +456」 | 읽는다 | 동일. **`gep .. i64 456` 0건** |
| `specs[13] mem[8]` `Entity+0x70` 「Top/Bottom 분기에서만 **꺼냄**」 | 읽는다 | **`gep .. i64 112` 0건.** 페이로드는 Entity **절대 오프셋**으로 직접 접근(`+0x88`=gep 136, `+0x128`=gep 296) |
| `specs[14] mem[15]` `Entity+0x70` `dir=r` | 읽는다 | note 는 이미 「dbg_value 로만 등장」이라 적어 두고 **`dir` 만 `r` 로 남아 있었다** |

⟹ 이 4행의 옳은 값은 **`dir: "-"`**(= `specs[14]` 의 `setup_limit`/`wait_limit` 와 같은 「참조용 행」).

### ⚠계약 한계 — `dir` 을 **패치로 못 고친다**
`dir` 은 v2 정본의 `reads`/`writes` 행에 **키 자체가 없고** `mkspec3.py:302~319` 가 배열 소속으로 유도한다
(명시 `dir` 이 있으면 존중). `applypatch` 의 경로 문법은 **행에 없는 키를 새로 만들 수 없다** —
`/specs[10]/mem[12]/dir` 에 `old="r"` 을 주면 `현재 None` 으로 거부된다(실측).
6차가 적발한 「**계약이 못 가리키는 자리는 영원히 안 고쳐진다**」와 같은 형태다.
⟹ 이번엔 `note` 에 참인 사실을 박았고, **메인이 v2 4행에 `"dir": "-"` 를 직접 넣어야 한다**(brief_errors 에 기재).

### 다음 라운드 게이트 제안 = **G14 `mem.dir`**
`memdir.check_spec(sp)` 를 그대로 `specgate` 에 물리면 된다. 단 **올리는 등급을 셋으로 갈라라**
(그래야 오탐을 손으로 넘기는 습관이 안 생긴다):
- `실오류` — 접근이 0건이고 인자로도 안 넘어감
- `판정보류(콜리)` — 그 오프셋 gep 가 **call/invoke 인자로만** 소비됨
- `판정보류(레지스터)` — 그 베이스가 스칼라 승격 인자

남은 진짜 한계는 **베이스 미구분**이다 — 오프셋 숫자만 보므로 다른 구조체의 같은 오프셋이
서로의 알리바이가 된다(실측: `specs[10] MainObjective+0x0` 이 `OperationData+0x0` 의 load 로 통과했다).
다음 판은 **SSA 루트를 베이스 태그로 달고** 대조해야 한다.

---

## §3 `consts.kind` — **무검사 축이 아니라 「무측정 축」**이었다

### 발견
`consts[].kind` 는 **명세에 저장된 값이 아니다.** `mkspec3.py:325~334` 가 `meaning` 문자열의
낱말로 유도하고 **잔여를 전부 `임계` 로 떨군다**:

```python
kind = ("태그"   if any(k in m for k in ("태그", "판별자", "variant")) else
        "센티널" if any(k in m for k in ("센티널", "니치", "0xff", "MAX")) else
        "인덱스" if "인덱스" in m else "임계")      # ← 임계 = 잔여 버킷
```

⟹ 전 20함수 「임계 111행」은 **분류 결과가 아니라 키워드가 안 걸린 나머지**다.
이 축은 **독립 정보가 0** 이라, 검사기를 붙여도 재는 것은 명세의 **표기 습관**뿐이다.
도시에 §4-b 의 「186행 무검사」는 **무측정**으로 고쳐 적어야 한다(D4).

### 그래도 재 봤다 — IR 문맥으로 측정하는 계측기
`_verify7/C/oracle/kindchk.py` — 리터럴이 IR 에서 **어떤 옵코드의 피연산자로 등장하는지**를 세어
(`EQ`=switch 케이스·`icmp eq/ne` / `ORD`=순서비교 / `ARI` / `IDX` / `ST` / `ARG`)
선언된 `kind` 의 필요 문맥과 대조한다. 「필요 문맥이 **하나도** 없을 때만」 올려 오탐을 낮춘다.

```bash
python -X utf8 _verify7/C/oracle/kindchk.py 10 15   # 담당 74행 · 불일치 32 · 판정보류 10
python -X utf8 _verify7/C/oracle/kindchk.py 0 20    # 전체 186행 · 불일치 72 · 판정보류 20
```

대표 사례:
- `specs[13]` 16행 중 **14행이 「반환 부시」** 인데 `kind=임계` — 이 함수의 `ret` 자체가
  「usize — 부시 인덱스. LLVM range(i64 2, 22)」다. 순서비교 0건.
- `specs[14]` 「목표 부시 ID」 12행이 `kind=임계`. IR 에서는 `select i1 %57, i64 A, i64 B` 의 결과값뿐.
- `specs[11]` `TutorialType::None/First/TopSolo/…` **9행이 `kind=임계`** 인데
  같은 표의 `consts[1]·[2]·[3]` 은 이미 `태그` 다 — **한 표 안에서 분류가 갈려 있었다.**
- `specs[10] consts[0]=65534` 는 `and` 마스크인데 `태그`, `consts[1]=768` 은 `icmp eq` 기대값인데 `임계`
  — **둘이 서로 바뀌어 있다.**
- `specs[14] consts[5]=32000` 은 `kind=임계` 인데 `meaning` 이 스스로 **「(임계값 아님)」** 이라고 적는다.

### 제안 (patch 로 못 내므로 별도 파일)
`_verify7/C/kind_proposal.json` — **37행**의 `now`/`should`/`why`.
그리고 **도구를 이렇게 고쳐라**:
1. `임계` 를 **잔여 버킷에서 빼라.** 안 걸리면 `미분류` 로 떨궈야 축이 자기 상태를 드러낸다.
2. 어휘에 **`마스크`·`계수` 를 추가**하라 — 65534(진입 마스크)·32000(셀 크기)·100(백분율 환산)이
   현재 넷 중 어디에도 안 맞는다(전부 `임계` 로 떨어져 있다).
3. 장기적으로는 **`meaning` 낱말이 아니라 `kindchk.py` 의 IR 문맥으로 유도**하라.
   그래야 `kind` 가 비로소 **측정값**이 되고, 게이트로 검사할 대상이 생긴다.

---

## §4 `sig.params.role` — 담당분 **결함 0** (신설 계측기 `rolechk.py`)

`role` 은 자유서술이지만 **검사 가능한 주장 두 종류**가 들어 있다:
①「안 쓴다」류 ②오프셋 인용(`+0x930`).

```bash
python -X utf8 _verify7/C/oracle/rolechk.py 10 15   # 담당 31행 · 결함후보 0
python -X utf8 _verify7/C/oracle/rolechk.py 0 20    # 전체 124행 · 결함후보 15
```

초판은 담당분에서 **6건을 올렸는데 전부 오탐**이었고, 고치면서 나온 설계 규칙 3개가 이 축의 핵심이다:
1. ★**절 단위로 쪼개라.** 「cache(+0x0), context(+0x8) 를 읽음. blackboard(+0x10) 는 안 씀」
   한 줄을 통으로 보면 ①「안 씀」이 있으니 인자 전체가 미사용이라 읽고 ②부정절의 `0x10` 을
   「있어야 하는데 없다」로 읽어 **한 행에서 오탐 2건**이 난다(`specs[14] i=5` 실측).
   **긍정절과 부정절은 요구가 정반대다** — 부정절의 인용 오프셋은 **없어야 참**이다.
2. **스칼라 승격(SROA) 인자는 건너뛰라.** `specs[13]` 은 세 인자가 전부 승격돼
   소스 오프셋(`0x20`·`0x930`·`0x9c0`)이 IR 에 gep 로 안 남는다(`role` 이 스스로 그렇게 적는다).
3. 부정 어휘 목록에 **활용형을 넣어라.** 「안 쓴다」가 목록에 없어서 `specs[10] i=4` 가 오탐이었다.

전역 잔여 15건은 다른 배치 담당이라 손대지 않았다. 다만 `specs[17] i=1 version` 이
「%0 를 베이스로 54회 접근」으로 뜨는 것은 **sret 보정 실패**로 보인다(패치 안 냄, 범위 명시 = 미탐색).

---

## §5 `knobs.where` — G13 1건 + G13 이 **못 보는 형태** 1건

### (a) `specs[14] knobs[0]` — 줄번호 오류 (G13 적발)
```bash
awk 'NR>=94569 && NR<=94941{if ($0 ~ /, 41[^0-9]/) printf "%d: %s\n", NR, $0}' /c/tfm2mods/_gaibc/m08.ll
# 94618:   %33 = icmp ult i64 %32, 41, !dbg !55574
sed -n '94643p' /c/tfm2mods/_gaibc/m08.ll
# %45 = getelementptr inbounds nuw i8, ptr %44, i64 384, !dbg !55613   ← 무관
```
`94643` → **`94618`**. 범위 안에서 리터럴 41 이 있는 줄은 94618 **하나뿐**이다.

### (b) `specs[13] knobs[9]` — **G13 이 구조적으로 못 보는 자리** (신규)
`where = "_gaibc/m02.ll:9260·9265"`, `value = "umin(coord/32000, 29)"`.
G13(`whereline.py`)은 **백틱 인용**만 대조하는데 이 행은 인용이 `value` 칸에 백틱 없이 있어 **통과**한다.
실측:
```bash
awk 'NR>=11483 && NR<=11731' /c/tfm2mods/_gaibc/m10.ll | grep -cE "32000|umin|, 29"      # → 0
awk 'NR>=9255 && NR<=9270' /c/tfm2mods/_gaibc/m02.ll | grep -E "umin|32000|29"           # → (없음)
grep -n "umin.i64(i64 .*, i64 29)" /c/tfm2mods/_gaibc/m02.ll | head -3
#  9281: ... llvm.umin.i64(..., i64 29), !dbg !19054
python -X utf8 _verify7/C/oracle/dbgchain.py m02.ll 19043 19054
#  !19043 -> [('trace.rs', 147)]      ← cover.rs 가 아니다
```
⟹ ①인용한 두 줄에 그 명령이 **없다** ②근처의 진짜 클램프(9277~9289)는 **`trace.rs:147~148`** 다
③`target_bush_v30` 본문에는 **클램프 자체가 0건**이다. **이 노브는 이 함수의 노브가 아니다.**
실측 확인된 진짜 site 로 고쳐 적었다: `m08.ll:94830 udiv 32000` → `94836/94843 umin(..,29)`
(`!dbg`→`ganker.rs:55` = `specs[14] LineGankerPlan::update`). cover 쪽 대응 호출부 줄은 **미탐색**.

### G13 확장 제안
`whereline` 은 **`where` 칸의 백틱 인용**만 본다. `value`/`effect` 칸에 적힌 명령·상수도
같은 줄에서 대조해야 한다 — 이번 1건이 그 사각지대에서 나왔다.

---

## §6 오라클 — `ev≥4` 를 2건 내렸다

### 만든 것
| 파일 | 무엇 |
|---|---|
| `oracle/o7c.rs` → `o7c.out` | §A `TutorialType` 9종 술어 전수 · §B `position_exists` 45칸 · §C `chat_allowed` **57×9=513칸** · §D 라인코드 스윕 |
| `oracle/o7c2.rs` → `o7c2.out` | `chat_allowed` 2차원 스윕(태그 47/48/49 × b1 0..15 × b2 0..7 × tutorial 9) |

```bash
sh _verify3/build.sh "C:/tfm2mods/MIG/_verify7/C/oracle/o7c.rs"
"C:/Users/jungs/AppData/Local/Temp/tfm2_spanprobe/o7c.exe" > _verify7/C/oracle/o7c.out 2> _verify7/C/oracle/o7c.err
# exit=0 · stderr 0바이트 · setting_ok true width=960000 height=960000 tps=60 champ_radius=10000
```
`TEMPLATE.rs` 준수: `real_setting()`(44줄) + `setting_ok()` 출력 · `init_tower/init_nexus` 미호출 ·
TLS 메모 함수(`check_kill_die_tick`·`camp_pos`) 미사용이라 한 프로세스에서 쓸어도 된다(함정 ③ 비해당).

**수법**: `Chat` 은 tcx 로 `size=24 align=8 tag=Direct i8 @+0x0 valid 0..=56` 이고 페이로드가
`usize`/`Position`/`LineType` 뿐(전부 0 이 유효값)이라 **0 으로 채운 24B 에 태그 바이트만 써서
57개 variant 를 전부 조립**했다(`transmute::<[u8;24], Chat>`). 생성자 없이 전수 진리표가 나온다.

### 확정한 것
**§O7C-A** — `TutorialType` 9종 전수(9/9):
```
allow  epic=[0,7,8]  serpen=[0,5,7,8]  top_minion=[0,2,7,8]
       mid_minion=[0,4,5,7,8]  bottom_minion=[0,1,3,5,7,8]
player_count  None=5 First=2 TopSolo=1 Bottom=2 MidSolo=1 MidBottom=3 JungleOnly=1 Line=4 Total=5
```
⟹ `specs[12] knobs[5]`(세르펜 허용집합 {0,5,7,8}) **ev 4→2**.
부수로 `specs[12] closed[7]`(「`player_count()==5` vs `>=5` 는 **표기 불가**」)의 **전제가 실행 확인**됐다 —
최댓값이 정말 5다. 판정은 그대로 유지(반증 아님).

**§O7C-B** — `position_exists` 45칸이 명세와 **완전 일치**
(Top{0,2,7,8} / Jungle{0,6,8} / Mid{0,4,5,7,8} / Bottom·Support{0,1,3,5,7,8}).
그리고 각 집합이 `spawn_*_minion` 집합과 **1:1로 같다** — `logic` 의 주석 대응이 실행으로 확정됐다.

**§O7C-C** — `chat_allowed` 513칸. 무조건 허용 태그 = **16개**
{0,3,4,6,7,15,16,17,19,23,43,51,52,53,54,56}. 명세 `knobs[6]` 은 「미열거 **15개**」라고 적는데,
switch 케이스를 세면 정확히 맞는다:
```bash
awk 'NR>=53208 && NR<=53251' /c/tfm2mods/_gaibc/m13.ll | grep -cE "^    i8 [0-9]+, label"   # → 42
# 57 − 42 = 15 = default arm.  16번째(태그 51 EarlyPlan)는 **열거돼 있는데** 조건이 9칸 전부 참이었을 뿐
```
⟹ **`knobs[6]` 은 옳다**(오탐 아님, 반증 아님). 일반 경기(tutorial=None)는 **차단 0**,
Line(7)은 **태그 18(CounterJungle) 하나만** 차단된다.

**§O7C-D / o7c2** — `specs[12] knobs[7]`(「라인코드 ≥3 → 라인 존재검사 우회」) **ev 4→2**.
태그 47/48/49/50/55 × 라인바이트 0..7 × tutorial 9 전수에서 **≥3 이면 전 튜토리얼 허용**이고
≤2 면 그 `LineType` 의 `position_exists` 집합과 **완전히 일치**했다.

★**부수 발견(명세에 없던 것)** — `PlayCall(47)`/`PlayPhaseChange(48)`/`PlayPropose(49)` 는
**`+0x1` 의 첫 `u8` 에 두 번째 게이트**가 있다. 9 tutorial × 16 값 전수에서 다음 모델과 완전 정합:

| `+0x1` 값 | 요구 | 허용 tutorial |
|---|---|---|
| 0 | Top ∧ Mid ∧ Bottom | {0,7,8} |
| 1 | Mid ∧ Bottom | {0,5,7,8} |
| 2 | Top | {0,2,7,8} |
| 3 | Mid | {0,4,5,7,8} |
| 4 | Bottom | {0,1,3,5,7,8} |
| ≥5 | 없음 | 전부 |

즉 `chat_allowed(PlayCall(a, line, _)) = 요구라인존재(a) && (line>=3 || position_exists(line))`.
`GankPlan(50)`·`CounterRequest(55)` 는 `+0x1` 게이트가 없고 `+0x2` 라인 게이트만 있다
(tcx 가 말하는 「GankPlan 만 `line` 이 `+0x2`」와 정합). `ev_up` 근거에 함께 실었다.

### 못 내린 `ev≥4` — **판정 어휘와 범위**
| 대상 | 판정 | 범위 |
|---|---|---|
| `specs[11] knobs[11]~[21]` 11건 | **미탐색** | 전부 `BigPlan::sub_plan` / `SubPlan::merge` 내부다. 진입하려면 `LegacyPlanHandler` 실체를 만들어 `sub_plan` 을 직접 호출해야 하는데 **이번 배치에서 시도하지 않았다**(재료 부재가 아니다 — `LegacyPlanHandler` 구성 가능성 자체를 안 봤다) |
| `specs[12] knobs[8]~[18]` 11건 | **미탐색** | `handle_chat_inner`(`vis=in:game_ai`, m13.ll:29692~33371) 내부. 이번엔 `chat_allowed`/`position_exists` 계층까지만 팠다. 상위 `pub` 진입점(`handle_chat`)이 있으므로 **막힌 게 아니라 안 한 것** |
| `specs[13] knobs[9]` | **해소 방향 전환** | 이 함수의 노브가 아님을 실측(§5-b). `where` 를 고쳤고 ev 는 그대로 |
| `specs[13] knobs[10]` (미니맵 1/3 스케일·+20 오프셋) | **분류오류 후보 · 사실 서술** | `_gvbc/v10.ll` = **UI 렌더링**이다. 판단함수의 노브가 아니라 `notes` 감. 구조 변경이라 patch 는 안 냄 |
| `specs[14] knobs[6]` (「부시 선택기가 셋이다」) | **사실 서술** | 물음이 아니라 확정 사실 — `knobs` 가 아니라 `notes[]` 로 가야 한다(G10 계열) |
| `specs[10] notes[0]·[1]` | **표기 불가** | 「리터럴 3 이 768 로 접혔다」·「슬롯 5칸이 완전 언롤」은 **IR 인코딩 사실**이라 실행으로 갈리지 않는다 |

---

## §7 이번 라운드의 판정 반전 전량 (도시에 §5-b #4 — 오류로 셈)

| # | 전에 | 지금 | 근거 |
|---|---|---|---|
| 1~12 | G12 담당 12건 = 결함 | **전부 오탐** | §1 |
| 13 | (내 중간 결론) `specs[14] consts[4]` 252→265 | **철회, 252 가 옳다** | §1 |
| 14 | `specs[10] mem[12]` note 「gep +408」 | **`gep 408` 0건** | §2 |
| 15 | `specs[10] mem[15]` note 「gep +456」 | **`gep 456` 0건** | §2 |
| 16 | `specs[13] mem[8]` 「Top/Bottom 분기에서만 꺼냄」 | **`gep 112` 0건** — 절대 오프셋으로 직접 접근 | §2 |
| 17 | `specs[14] knobs[0]` `m08.ll 94643` | **94618** | §5-a |
| 18 | `specs[13] knobs[9]` `m02.ll:9260·9265` | **그 줄에 없다 · 진짜 site 는 trace.rs 이고 이 함수엔 클램프 자체가 없다** | §5-b |
| 19 | 도시에 §4-b 「`consts.kind` 무검사 축」 | **무측정 축**(파생값·잔여버킷) | §3 |
| 20 | (내 설계) `srclinecheck2.py` 가 G12 의 답 | **폐기** — 후보를 늘려 오탐이 는다(10→19) | §1 |

**총 20건** (그중 **2건이 내 자신의 판정 반전** — #13, #20).

---

## §8 다음 라운드가 바로 쓸 것

1. **`memdir.py` → G14**(`mem.dir` 451행). 등급을 `실오류`/`판정보류(콜리)`/`판정보류(레지스터)` 셋으로.
   진짜 한계는 **베이스 미구분** — SSA 루트에 베이스 태그를 달아야 거짓 음성이 빠진다.
2. **`mkspec3.kind` 를 고쳐라.** `임계` 를 잔여 버킷에서 빼고 `미분류` 를 만들라.
   어휘에 `마스크`·`계수` 추가. 장기적으로는 `kindchk.py` 의 **IR 문맥 유도**로 교체.
3. **`whereline`(G13)을 `value`/`effect` 칸까지** 확장. 이번 1건이 그 사각지대에서 나왔다.
4. **`rolechk.py` → G15**(`sig.params.role` 124행). 절 단위 분해 + SROA 인자 제외가 필수 설계다.
5. **`applypatch` 에 키 추가 연산**(`op:"set_key"`)을 넣어라. 없으면 `mem[].dir` 같은
   **파생-with-override 필드는 영원히 못 고친다.**
6. **`mkdossier` §0 에 도시에 자신과 `mkdossier.py` 의 해시**를 넣어라(D1).
7. 이 배치의 계측기 4개(`memdir`/`kindchk`/`rolechk`/`g12scan`)를 **MIG 루트로 승격 + `mktools.py`** —
   안 하면 다음 세션이 같은 걸 다시 만든다(6차에 `mkpatch.py` 가 네 번 재발명됐다).

---

## §9 부록 — 이번에 내가 밟은 함정 (다음 배치가 안 밟도록)

1. ⚠**Bash heredoc 으로 파이썬 문자열을 만들지 마라** — 도시에가 경고했는데 한 번 밟았다.
   `r"C:\tfm2mods\..."` 의 `\t` 가 **탭으로 치환**돼 `OSError: Invalid argument` 가 났다.
   경로가 들어가는 수정은 **Write/Edit 도구**로 해야 한다.
2. ⚠**도구가 라운드 중에 바뀐다.** `srclinecheck.py`(18:09) · `applypatch.py`(18:06) ·
   `mkdossier.py`(18:09) 가 내가 돌던 도중 갱신됐다. **기준선 수치를 인용하지 말고 그 자리에서 재측정하라**
   (METHOD_MAP §2-C). 실제로 내 G12 기준선이 47 → 10 으로 바뀌어 내 결론의 근거가 갈아엎어졌다.
3. ⚠**한 번의 Read 로 60KB 이상 읽지 마라** — 도시에 §3 의 5개 명세 파일(20~49KB)은 한 번에 하나씩
   읽었고 문제없었다. 대신 v3 JSON 을 `python -c` 로 **필요한 절만 뽑아 읽는 것**이 훨씬 쌌다.
