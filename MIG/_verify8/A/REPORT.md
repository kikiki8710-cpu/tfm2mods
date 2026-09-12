# 8차 배치 A — 축 `mem[].dir` 게이트 수립 (게임 0.5.8)

> 담당: 축 하나(`mem[].dir` 451행)를 끝까지 닫는다. 산출물 = `gate.py`(G14 후보) · `patch.json` · 이 문서.
> 신선도: 착수 시 `FRESH`, `patch.json` 제출 직전 재확인 `FRESH` (`python -X utf8 dossierfresh.py 8 A`).

---

## 0. 한 줄 결론

**후보 4벌이 갈린 원인은 독해력이 아니라 판정식 두 가지**(①부재를 결함으로 셀 것인가 ②오프셋 칸을
어디까지 파싱할 것인가)였다. 통합판은 **모순만 적발**하고 나머지 계측은 전부 구제로 돌렸다.
통합판이 낸 적발 3건을 IR 원문으로 전수 반증해 **3/3 오탐**임을 확인하고 게이트를 고쳤다.
**최종 적발 0** — 그 0 이 무능이 아님을 **변이 시험**(행마다 `dir` 을 뒤집어 되잡는지)으로 반증했다
(**238/448 = 53.1% 포착**). 별도로 IR 을 따라가다 **명세 실오류 1건(두 칸)** 과 **보강 2건**을 찾아
`patch.json` 으로 냈다.

| 항목 | 수 |
|---|---|
| 통합 중간판 적발 | 7 → 3 |
| 그중 IR 반증으로 걸러낸 **오탐** | **3 / 3 (100%)** |
| 최종 게이트 적발 | **0** |
| `errors[]` | **4** (실오류 2칸 = 결함 1건 · 보강 2) |
| `ev_up[]` | **1** |
| `brief_errors[]`(내 지시의 오류) | **4** |
| `applypatch.py 8 --only A --dry` | 정정 4/4 · ev상향 1/1 · 실패 0 · **동작 변경 2건** |

---

## 1. 실제로 실행한 명령 (그대로)

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 8 A                      # 착수 시 · 제출 직전 2회 → 둘 다 FRESH

# 후보 4벌 전역 실행
PYTHONIOENCODING=utf-8 python -X utf8 _gates/mem_dir/A_memdir.py 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19
PYTHONIOENCODING=utf-8 python -X utf8 _gates/mem_dir/B_memdircheck.py
PYTHONIOENCODING=utf-8 python -X utf8 _gates/mem_dir/C_memdir.py 0 20
PYTHONIOENCODING=utf-8 python -X utf8 _gates/mem_dir/D_memdir.py

# IR 원문 반증 (예시 — 전량은 §3)
cd /c/tfm2mods/_gaibc && sed -n '58175,58644p' m04.ll | grep -n 'i64 32\b'
cd /c/tfm2mods/_gaibc && sed -n '29383,29689p' m13.ll | grep -n 'i64 2136\|i64 2144\|memcpy\|grow_one'
cd /c/tfm2mods/_gaibc && awk 'NR>=29646 && NR<=29653 {print NR": "$0}' m13.ll
cd /c/tfm2mods/_gaibc && sed -n '17695,17825p' m05.ll | grep -nE 'memset|memcpy|store'
cd /c/tfm2mods/_gaibc && sed -n '34909,34935p' m12.ll
cd /c/tfm2mods/_gaibc && sed -n '44058,44076p' m04.ll
cd /c/tfm2mods/_gaibc && sed -n '94569,94941p' m08.ll | grep -nE 'i64 (24|32|40|41)[,)]'

# vtable 정본 대조
PYTHONIOENCODING=utf-8 python -X utf8 divtable.py EffectType

# 통합 게이트 · 검출력 반증 · 보강 조사
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/A/gate.py
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/A/mutate.py
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/A/survey_rw.py

# 제출 전 사전검증
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 8 --only A --dry
```

---

## 2. 임무 ①: 후보 4벌은 왜 갈리나

전역 실행 실측 — **A=7 · B=3 · C=46 · D=7**.

### 2-1. 가장 큰 갈림 = **부재를 결함으로 세는가** (46 vs 3~7)

`C_memdir.py` 는 판정식이 *「`dir=r` 인데 그 오프셋으로 가는 `load` 가 범위 안에 하나도 없다 → 결함 후보」*
다. A·B·D 는 같은 경우를 **보류**한다. 이 한 줄이 46 과 3~7 을 가른다.

C 의 46건이 실제로 무엇인지 표본을 뜯어 보면 전부 **C 자신의 docstring 이 한계 2·3 으로 적어 둔 것**이다:

| 유형 | 예 | IR 실측 |
|---|---|---|
| 콜리 안에서만 접근 | specs[4] `BrainMinionParameter+0x20` | `has_line_defense_threat`(defense_nexus.rs:588) 안에서 읽는다 |
| 레지스터 승격된 인자 | specs[10] `MainObjective(i24)+0x1/+0x2` | `load` 가 아니라 `and`/`lshr` — 원리적으로 `dir` 축 밖 |
| 폭이 넓은 한 번의 쓰기 | specs[17] `DeathMatchBattle +0x128~+0x168` **8행** | m05.ll:17822 `%52 = gep %0, i64 288` → `memset(%52, 0, 80)` 이 0x120~0x16F 를 통째로 덮는다 |
| 힙 원소 표기 | specs[12] `0x860[len]` | 구조체 오프셋이 아니다(§4 참조) |

⟹ **부재는 결함이 아니다.** 이 축의 실패 모드는 **방향 뒤집힘(r↔w)** 뿐이고, 그건
「그 자리가 반대 방향으로만 관측된다」로만 말할 수 있다.

### 2-2. A=7 과 D=7 은 **수만 같고 집합이 다르다**

| 적발 | A | D |
|---|---|---|
| specs[0] `sret+0x8` (`Input::Ult.target`) | — | ○ |
| specs[4] `BrainMinionParameter+0x20` | ○ | ○ |
| specs[5] `LegacyPlanHandler+0x888` | ○ | ○ |
| specs[11] `LegacyPlanHandler+0x5e8` | ○ | — |
| specs[12] `GameContext+0x38` | ○ | — |
| specs[12] `CallHandled+0x38` | — | ○ |
| specs[12] `LegacyPlanHandler+0x858` / `+0x860` | ○○ | ○○ |
| specs[14] `LineGankerPlan+0x20` (`dir=-`) | ○ | ○ |

차이의 원인은 **오프셋 칸 파싱**과 **`dir=-` 처리**다.
A 는 `int(str,16)` 이 `0x860[len]` 에서 예외를 내 그 행을 「범위밖」으로 버리고,
D 는 `try/except continue` 로 스킵한다. B·C 는 정규식 접두 매칭이라 `0x860` 으로 **오인**한다.
`dir=-` 2행은 A·D 가 「claim 이 관측집합에 없다」로 **불일치에 포함**시키고 B·C 는 스킵한다.

### 2-3. B=3 이 가장 적은 이유 = **`addr` 구제**

B 만 *「주소만 만들어 호출 인자로 넘긴 경우(`addr`)」* 를 센다. `dir=r` 은 `addr` 로 충족,
`dir=w` 는 `addr` 뿐이면 판정보류. 그래서 A·D 가 잡는 `+0x858`(`grow_one` 에 `&mut RawVec` 전달)이
B 에선 보류로 빠진다. **이 구제가 옳았다**는 것을 IR 로 확인했다(§3-2).

### 2-4. 정리 — 갈림은 계측이 아니라 **판정식**

| 갈림점 | A | B | C | D |
|---|---|---|---|---|
| 부재 | 보류 | 보류 | **결함** | 보류 |
| `addr` 구제 | 없음 | r 만 | 없음 | 없음 |
| `0x860[len]` | 예외→버림 | 0x860 오인 | 0x860 오인 | 예외→스킵 |
| `dir=-` | 불일치로 셈 | 스킵 | 스킵 | 불일치로 셈 |
| 접근 폭 | 무시 | 무시 | 무시 | 무시 |
| ptr phi/select | 미추적 | 미추적 | **합집합** | 미추적 |

C 만 유일하게 포인터 추적이 정교한데(ptr phi 합집합·정수 phi 오프셋), **그 정교함은 적발 수에
거의 기여하지 않았다** — 46건의 대부분은 판정식 때문이다. 「계측을 정교하게 하면 더 잡는다」가
이 축에선 **거짓**이었고, 정교한 계측은 오히려 **구제**를 늘린다(§5 표).

---

## 3. 임무 ③: 통합판의 적발을 내가 반증했다 — **3/3 오탐**

통합 중간판(접기만 + 폭·addr 구제)이 3건을 냈다. 전부 IR 원문으로 확인했다.

### 3-1. specs[4] `BrainMinionParameter +0x20 minion_count dir=r` → **오탐(베이스 충돌)**

게이트는 「오프셋 32 가 이 범위에서 `store` 로만 쓰인다」고 했다. 원문:

```
m04.ll:58336   %76 = getelementptr inbounds nuw i8, ptr %7, i64 32, !dbg !67144
m04.ll:58337   store ptr %2, ptr %76, align 8, !dbg !67144, !alias.scope !67151
```

`%7` 은 바로 앞에서 `llvm.lifetime.start` 로 열린 **클로저 캡처 구조체**이고 +8/+16/+24/+32 에
포인터 4개를 담는다. `BrainMinionParameter` 와 무관하다. `minion_count` 자체는 명세 note 대로
콜리 `has_line_defense_threat`(defense_nexus.rs:588) 안에서 읽힌다.
⟹ **오프셋만 접어 보면 남의 구조체가 반증으로 둔갑한다.** 이걸 막으려고 **base 동정**을 넣었다(§5).

### 3-2. specs[12] `LegacyPlanHandler +0x860 pending_trace_events.ptr dir=w` → **오탐(콜리가 쓴다)**

```
m13.ll:29566   %64 = getelementptr inbounds nuw i8, ptr %0, i64 2136           ; = +0x858
m13.ll:29634   invoke void @_RNvMs3_..alloc7raw_vec..RawVec..PendingTraceEvent..8grow_one..
                 (ptr noalias noundef nonnull align 8 dereferenceable(24) %64)
```

`grow_one` 에 넘어가는 것은 `&mut RawVec` = `+0x858` 이고 **`dereferenceable(24)`** 가
cap(+0x858)·ptr(+0x860)·len(+0x868) 세 워드를 명시한다. 즉 `+0x860` 쓰기는 **콜리가** 한다.
명세가 맞다. ⟹ 게이트에 **`dereferenceable(N)` 범위 보류**를 넣었다.

### 3-3. specs[12] `+0x860[len] pending_trace_events[len] dir=w` → **오탐(좌표계 혼동) + 명세 실오류**

```
m13.ll:29646   %96 = getelementptr inbounds nuw i8, ptr %0, i64 2144            ; = +0x860
m13.ll:29647   %97 = load ptr, ptr %96, align 8, ...
m13.ll:29649   %98 = getelementptr inbounds nuw { { i64, [21 x i64] }, i64 }, ptr %97, i64 %88
m13.ll:29652   call void @llvm.memcpy.p0.p0.i64(ptr .. dereferenceable(184) %98,
                 ptr .. dereferenceable(184) %12, i64 184, i1 false)
```

쓰기 대상은 `%0+0x860` 이 **아니라** `load(%0+0x860) + len*184` 다. 게이트가 `0x860` 으로 읽은 것이
오탐이고 — **동시에 명세 쪽도 틀렸다.** 같은 상황(Vec push)인 `specs[5]/mem[27]` 은
base=`V50DiveEpisode(힙 원소)` · offset=`0x0` 로 적고 note 에
*「★쓰기 대상은 `LegacyPlanHandler+0x890` **자체가 아니라** 거기서 읽은 힙 포인터 + len×104 다」*
라고 못 박아 뒀다. **두 행이 같은 구조를 서로 다르게 적고 있었다**(G1 자기모순 계열).
⟹ `errors[]` 2칸(base·offset)으로 냈다.

### 3-4. 다른 후보가 잡았고 통합판이 **구제한** 것들 — 구제가 옳았는지도 확인했다

| 후보 적발 | 통합판 판정 | IR 근거 |
|---|---|---|
| D: specs[0] `sret+0x8 dir=w` | OK(폭 구제) | m04.ll:44178 `%126 = gep %0, i64 8` → `memcpy(%126, %8, 24)` |
| A·D: specs[5] `+0x888 dir=w` | 보류(`&mut` 전달) | m13.ll:29235 `%116 = gep %0, i64 2184` → `grow_one(dereferenceable(24) %116)` |
| A: specs[11] `+0x5e8 dir=r` | 보류(호출 전달) | m13.ll:12446 `%81 = gep %0, i64 1512` → `drop_glue::<BigPlan>(dereferenceable(384) %81)` — 콜리가 읽는다 |
| C: specs[2] `fountains[team]+0x10/+0x18 dir=r` | OK(시프트 정렬) | m12.ll:34916 `%32 = gep %30, i64 16` → `%33 = load i64` / 34928 `%40 = gep %30, i64 24` → `%41 = load i64` |
| C: specs[17] `+0x128~+0x168 dir=w` 8행 | OK(폭 구제) | m05.ll:17822 `memset(%52(=+0x120), 0, 80)` |
| A·D: specs[14] `LineGankerPlan+0x20 dir=-` | 대상 아님 | 그 범위의 `gep .. i64 32` 는 `%39`(=GameContext) 의 `map` 이고(m08.ll:94844), `%0`(LineGankerPlan) 에는 +0x18/+0x20 gep 자체가 없다 ⟹ **`dir=-` 가 맞다** |

⟹ **판정 반전을 낼 뻔한 것 = 6건**(A·D 가 잡은 것 중 명세가 옳았던 것). 지시대로 「적발을 성과로」
받아들였으면 그만큼 맞는 명세를 틀리게 고쳤을 것이다.

---

## 4. 임무 ④: 남은 진짜 오류 (`patch.json`)

| # | 경로 | kind | 내용 | behavior_change |
|---|---|---|---|---|
| 1 | `/specs[12]/mem[13]/base` | 실오류 | `LegacyPlanHandler` → `PendingTraceEvent(힙 원소)` | **true** |
| 2 | `/specs[12]/mem[13]/offset` | 실오류 | `0x860[len]` → `0x0` | **true** |
| 3 | `/specs[12]/mem[13]/note` | 보강 | IR 사슬(29646→29652)·`grow_one` 분리 명시 | false |
| 4 | `/specs[5]/mem[29]/name` | 보강 | `cap / .ptr` → `cap(+0x888) · .ptr(+0x890)` — 한 행이 두 오프셋을 덮는데 offset 칸엔 하나뿐 | false |

`ev_up[]` 1건: `/specs[0]/mem[23]`(`dyn EffectType vtable +0x10 align`) **4 → 3**.
`divtable.py EffectType` 이 vtable 전역(`@anon.7530…1131`, g02.ll · 슬롯 35 · 296B)을 찾아
`0x0=drop_glue`, 메서드 `0x18~` 를 실측했고 같은 실행에서 `0xf8=linear_move_speed`·`0x118=on_caster`
(= mem[24]/mem[25])도 재확인됐다. IR 이 그 값을 **정렬값으로** 쓰는 것도 확인:
`m04.ll:44062 %48 = gep %47, i64 16` → `44063 %49 = load i64 .. !invariant.load` →
`44064 add nsw %49, -1` → `44065 and %50, -16` → `44066 gep %45, i64 %51`(Arc 페이로드).
⚠`chk` 칸(`확인불가(vtable 슬롯)`)은 `mkspec3` 가 `tcxaudit` 으로 만드는 **파생**이라 `errors[]` 로
못 고친다. 원인은 근거 부재가 아니라 `tcxaudit` 이 ADT 아닌 vtable 을 못 보는 것이다(METHOD_MAP ⑦ 한계).

---

## 5. 임무 ②·⑤: 통합 게이트 `gate.py` (= `G14` 후보)

`check_spec(sp) -> [(mem 인덱스, 사유, 상세)]` — `srclinecheck.py`(G12)·`whereline.py`(G13) 와 같은 형식.
`specgate.py` 의 `gate13` 패턴 그대로 붙는다(`import gate as G; rows = G.check_spec(sp)`).

### 판정식

```
0) base 동정: 같은 base 의 다른 행들이 주장대로 확인되는 (루트, 시프트)를 찾는다. 못 찾으면 보류.
1) claim 방향의 접근(폭 포함)이 그 자리를 덮으면 OK
2) 주소가 호출 인자로 나갔고 dereferenceable(N) 이 그 자리를 덮으면
   · readonly 없음(&mut) → 양방향 보류   · readonly 있음 → r 주장만 구제
3) 그러고도 남고 반대 방향 관측이 **동정된 자리에** 있으면 → 불일치
```

★**귀속·추론은 구제에만.** 추가 계측(ptr phi 합집합·정수 phi 오프셋·접근 폭·`dereferenceable`·
base 동정)을 전부 구제 방향으로만 배치했다. 7차 `G12` 가 「귀속을 후보에 더했더니 16→22 로 늘었다」로
증명한 그 함정을 이 축에서도 그대로 밟을 수 있었다 — 실제로 **계측을 켤 때마다 적발이 줄었다.**

### 계측을 켤 때마다 적발이 줄고 검출력은 올랐다

| 구성 | 실제 명세 적발 | 변이 포착(`mutate.py`) |
|---|---|---|
| 접기만(1판) | 7 (전수 오탐) | — |
| + 접근 폭·`addr` 구제 | 3 (전수 오탐) | 10.0% (45/448) |
| + `readonly` 분리 | **0** | 34.6% (155/448) |
| + base 동정(루트만) | **0** | 57.8% (259/448) |
| **+ 시프트 정렬(최종)** | **0** | **53.1% (238/448)** |

★`readonly` 분리가 결정적이다. `dereferenceable(N)` 만 보고 보류하면 `Entity`(deref 1728)를 넘기는
호출 하나가 그 구조체 **전 필드를 보류**로 만들어 게이트가 무력화된다(포착 10%).
LLVM 이 `&T` 인자에 `readonly` 를, `&mut T` 에 안 붙이는 것을 그대로 쓴다.

마지막 행에서 포착이 57.8 → 53.1 로 내려간 것은 시프트 정렬이 **구제도 늘렸기** 때문이고,
그 대가로 `MapDef.fountains[team]` 처럼 **gep 로 도달하는 base**(명세는 상대 오프셋, IR 은 절대
오프셋)를 제대로 읽게 됐다. 두 구성 모두 실제 적발은 0 이라 **정확도 손실 없이 좌표계를 고친 것**이다.

### 변이 시험 = 「적발 0」의 반증 장치

`mutate.py` 는 행마다 `dir` 을 뒤집어 넣고 그 행을 되잡는지 센다(정본은 안 건드린다 — 메모리 사본).
적발 0 이 ①축이 깨끗하다 인지 ②게이트가 무능한지 가르는 유일한 방법이다.
함수별 포착률은 `check_favorable_engage_formation` 79% · `sub_plan(07)` 86% · `target_bush_v30` 67%,
반대로 `v3_fall_back_to_passive`·`handle_chat` 0% 다 — 후자는 구조체를 통째로 `&mut` 로 콜리에 넘기는
함수라 **원리적으로 이 축에서 안 보인다**(한계 1).

### 보류 91행의 사유 분포 (`gate.py` 출력)

| 사유 | 행 |
|---|---|
| base 미동정(그 함수에 그 구조체 행이 하나뿐) | 58 |
| `held_r`(readonly 로 콜리에 전달) | 19 |
| 증인 없음(그 자리 관측 자체가 없음) | 8 |
| `held_rw`(`&mut` 로 콜리에 전달) | 6 |

---

## 6. 판정 어휘

- **적발 0** — `mem[].dir` 448 검사행에서 **IR 과 모순되는 방향 주장 0건**.
  단 이것은 **「이 방법으로」** 다: 아래 세 부류는 이 축의 계측으로 판정되지 않는다.
- **표기 불가 (범위: `dir` 축 · IR 계측)** — 레지스터 승격된 인자(specs[10] `MainObjective i24`)의
  필드 읽기는 `load` 가 아니라 `and`/`lshr` 다. `dir`(r/w) 칸으로는 담을 형식이 없고
  **별도 값(예 `reg`)이 필요**하다. 오프셋 자체는 `tcx` 로 알지만 *방향*이 이 축의 문제다.
- **재료 부재 (범위: 담당 IR 줄범위 + `_gaibc`)** — 콜리 안에서만 이뤄지는 접근.
  시도한 재료: ①담당 줄범위 IR 전량 ②`dereferenceable` 속성 ③`readonly` 속성.
  콜리 본문까지 펴려면 **콜리 정의를 따라가는 인터프로시저 확장**이 필요하다(미탐색 — 다음 라운드 제안).
- **미탐색** — 「한 자리를 읽기도 쓰기도 하는데 명세엔 한쪽만 적힌 행」. `survey_rw.py` 로 후보 57건을
  뽑았으나 **표본이 오탐 지배**(`AbstractGameWithCache+0x0` 류 베이스 충돌, `shift -360` 같은
  잡음 앵커)여서 **한 건도 확정하지 않았다.** 이 축의 `errors[]` 로 내지 않는다.
  ⚠`applypatch` 에 **append 가 없어** 어차피 행을 못 늘린다는 구조적 벽과도 겹친다.
- **사실 서술** — `mem` 451행 중 `ev=4` 는 1행뿐(`specs[0]/mem[23]`)이고 나머지 450행은 `ev=3` 이다.
  `ev` 축은 숫자가 작을수록 강하므로 4 는 **위반이 아니라 약한 근거**다(`mem` 상한 3 은 「그 이상
  강해질 수 없다」는 뜻). 이번에 `ev_up` 으로 3 으로 내렸다.

---

## 7. ★내 지시(도시에)의 오류 — 4건 (`brief_errors[]` 와 동일)

1. **§1 「후보 4벌이 각각 불일치 3 / 46 / 7」 — 4벌에 수치가 3개.**
   실제 A=7 · B=3 · C=46 · D=7 이고 A 와 D 는 **수만 같고 집합이 다르다**(§2-2 표).
   임무가 「왜 갈리는지 규명하라」인데 지시문이 갈림의 한 축(A↔D)을 지워 놓았다.
2. **§1·§3 의 「451행」이 검사 대상 행수가 아니다.** 기계 검사 가능한 행은 **448**이다
   (`dir=-` 2행 + `0x860[len]` 1행 제외). 「451행을 닫아라」를 그대로 받으면 그 3행을 억지로
   판정하게 되는데, 실제로 A·D 가 `dir=-` 를 불일치로 세어 **각 1건씩 오탐**을 냈다.
3. **§5 `behavior_change` 의 셈 단위가 정의돼 있지 않다.** 이번 실오류는 **한 결함이 두 칸**
   (`base`·`offset`)에 걸쳐 있어 `--dry` 가 「동작 변경 2건」으로 센다. 결함 단위인지 칸 단위인지
   정하지 않으면 라운드 간 집계가 어긋난다(나는 칸 단위로 두 번 `true` 를 냈다).
4. **§4 의 `G12 src_line 대조=8` 이 다른 두 정본과 안 맞는다.** `srclinecheck.py` docstring 은
   7차 교정을 **「47 → 10건」**, `SPEC_RUNBOOK §S5-b` 는 **「47 → 13건」**이라 적는다 —
   세 자리가 셋 다 다르다(8/10/13). 내 축이 아니라 손대지 않았지만 다음 라운드가 어느 수를
   기준선으로 삼을지 모른다. (**판정 반전도 오류로 센다**는 규칙에 따라 1건으로 셈.)

---

## 8. 다음 라운드에 넘기는 제안 (내가 고치지 않은 것)

- **`gate.py` → `G14` 승격**: `specgate.py` 의 `GATES`/`NAMES` 에 `"G14": gate14` 를 더하고
  `gate13` 과 같은 3줄 패턴으로 부르면 된다. 도구 승격은 메인 몫이라 `MIG` 루트 파일은 안 건드렸다.
- **인터프로시저 확장**(한계 1 해소): 콜리 정의의 IR 범위를 따라가 `&mut` 로 넘긴 구조체의 실제
  store 오프셋을 회수하면 보류 25행(`held_r` 19 + `held_rw` 6)이 판정 가능해진다.
- **`consts.kind`·`sig.params.role` 축**: `SPEC_RUNBOOK §S5-c` 표에 남은 무검사 축.
  이번 경험상 **새 게이트는 첫 적발을 전수 반증하기 전엔 수치를 보고하지 말 것**.
- **명세 스키마**: 힙 원소를 가리키는 행의 표기 convention(`<원소타입>(힙 원소)` + `0x0`)을
  `SPEC_GUIDE.md` 에 못 박아라. 이번 실오류는 그 convention 이 문서에 없고 **한 행의 관례로만**
  존재해서 생겼다(specs[5] 은 지켰고 specs[12] 는 몰랐다).

---

## 9. 산출물

```
MIG/_verify8/A/gate.py        통합 게이트(G14 후보) · check_spec(sp) 진입점
MIG/_verify8/A/mutate.py      변이 시험 — 「적발 0」의 반증 장치
MIG/_verify8/A/survey_rw.py   한쪽만 적힌 dir 후보 조사(오탐 지배 — 결론 없음, 기록용)
MIG/_verify8/A/patch.json     errors 4 · ev_up 1 · brief_errors 4  (--dry 통과)
MIG/_verify8/A/REPORT.md      이 문서
```
`_spec/`·`_gates/`·`MIG` 루트 도구·다른 배치 폴더에는 **쓰지 않았다.**
