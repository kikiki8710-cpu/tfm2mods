# 8차 배치 C — 축 `sig.params[].role`(인자의 역할) 게이트 수립

> 게임 0.5.8 · 2026-09-11 · 담당 축 124행(20함수 전량) · 도시에 `_verify8/DOSSIER_C.md`(FRESH, 착수 시·제출 직전 2회 확인)

## 0. 한 줄 결론

| 항목 | 값 |
|---|---|
| **확정 sret 규약** | **`params[]` 에 싣는다** — `params[0]`, `name` = `(sret)`, `i` = **0**, 소스 인자는 `i` = 1..n (= `07`·`15` 가 이미 쓰는 형식) |
| 후보 4벌 총 적발 | **44건** (A 10 · B 5 · C 15 · D 14) |
| 반증으로 걸러낸 오탐 | **42건** (A 10/10 · C 15/15 · D 14/14 · B 3/5) |
| 살아남은 실오류 | **2건** — `02`·`17` 의 sret 미등재 |
| 통합 게이트(`gate.py`) 적발 | **강한 후보 3건**(P2 x2 + P3 x1) · 약한 후보 11건(기각 근거 아님) |
| 게이트 밖 수작업 발견 | **1건** — `03 defensive_crisis` `target` 의 「좌표」 주장이 거짓 |
| `patch.json` 항목 | **errors 10 · ev_up 0 · brief_errors 5** (`applypatch.py 8 --only C --dry` = **정정 10/10 성공 · 실패 0**) |

판정 어휘: 이 라운드에 `재료 부재`·`표기 불가` 로 닫은 것은 없다. **`보류` 1건** = `02`·`17` 의 sret 행 실제 삽입(도구에 행 추가 연산이 생기면 재개). **`미탐색` 1건** = 한국어 낱말 나열(「id 와 좌표」) 대조의 기계화(§6).

---

## 1. 실제로 실행한 것 (명령줄 그대로)

```
python -X utf8 dossierfresh.py 8 C                        # 착수 시·제출 직전 2회 · 둘 다 FRESH

PYTHONIOENCODING=utf-8 python -X utf8 _gates/params_role/B_rolecheck.py          # 후보 B 전량
PYTHONIOENCODING=utf-8 python -X utf8 _gates/params_role/D_paramrole.py          # 후보 D 전량
PYTHONIOENCODING=utf-8 python -X utf8 _gates/params_role/A_paramuse.py 0 1 2 ... 19   # 후보 A (인덱스 인자 필수)
# 후보 C 는 그대로는 못 돈다 — §2-0
cp _gates/mem_dir/C_memdir.py _verify8/C/run/memdir.py
cp _gates/params_role/C_rolechk.py _verify8/C/run/
cd _verify8/C/run && PYTHONPATH=. PYTHONIOENCODING=utf-8 python -X utf8 C_rolechk.py

PYTHONIOENCODING=utf-8 python -X utf8 _verify8/C/run/dump_defs.py 1 2 13 17 19 7 15 0   # define 헤더 원문
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/C/run/truth.py                    # 20함수 정렬 실태표
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/C/run/audit.py                    # 124행 전수 실사용 대조

PYTHONIOENCODING=utf-8 python -X utf8 _verify8/C/gate.py                         # 통합판(강)
PYTHONIOENCODING=utf-8 python -X utf8 _verify8/C/gate.py --weak                  # 통합판(강+약)
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 8 --only C --dry             # 사전 검증
PYTHONIOENCODING=utf-8 python -X utf8 specgate.py --gate G12                     # 여력분
```

재현용 보조 스크립트는 전부 `_verify8/C/run/` 에 있다(`dump_defs.py`·`truth.py`·`audit.py`, 그리고 `role` 경로 판 `patch_role.json`).

---

## 2. 후보 4벌 — **왜 결과가 갈렸나**

### 2-0. 먼저, 후보 C 는 그대로는 실행되지 않는다
`C_rolechk.py:26` 이 `import memdir` 인데 **`MIG` 루트에 `memdir.py` 가 없다**(7차 산출물이 `_gates/mem_dir/C_memdir.py` 로 이관되며 이름이 바뀐 것으로 보인다). `ModuleNotFoundError`.
⟹ `_gates/mem_dir/C_memdir.py` 를 `memdir.py` 로 복사해 `PYTHONPATH` 에 얹어야 돈다. 원본은 손대지 않았다(금지 준수).

### 2-1. 갈린 지점 셋

| 갈림 | 누가 틀렸나 | 무엇 | 실측 대가 |
|---|---|---|---|
| (1) `define` 헤더 파싱 | **B** | `ir_args()` 가 `ln[ln.index("(")+1:ln.rindex(")")]` — **반환 타입의 괄호**를 인자 괄호로 잡는다. `range(i64 -9223372036854775808, 206)`(m05.ll:39984) · `range(i8 0, 6)`(m04.ll:62570) · `range(i64 2, 22)`(m10.ll:11483) | B 적발 5건 중 **3건이 순전히 이 버그** |
| (2) sret 정렬 | **C** | `params[0].i == 0` 이면 sret 형이라 판정 — **sret 를 아예 안 싣는 명세**(02·17)는 이 판정을 통과해 인자가 통째로 한 칸 밀린 채 검사된다 | C 적발 15건 중 **7건이 이 밀림** |
| (3) 절(clause) 분해 | **D**·**A** | 「cache(+0x0), context(+0x8) 를 읽음. blackboard(+0x10) **는 안 씀**」 한 행에서 (a)「안 씀」을 인자 전체 부정으로 (b)부정절의 0x10 을 「있어야 하는데 없다」로 읽어 **한 행에 오탐 2건** | D 14건 중 8건 · A 10건 중 1건 |

★그리고 **아무도 안 본 것이 하나 더 있다**: `role` 산문 안의 **IR 줄 인용**(`m10.ll:33874` + 백틱 `store i64 %0, ptr %11`). G13 이 `knobs.where` 에서 쓰는 수법 그대로 쓸 수 있고, 절 분해·오프셋 귀속 문제가 전혀 없는 **가장 강한 검사**다. 통합판에 **P6** 으로 넣었다.

### 2-2. 정렬 실태 — 원자료 (`run/truth.py` 출력, `define` 헤더 원문 대조)

| # | 함수 | IR 인자 | sret(IR) | spec params | params[0] | 정렬 |
|---|---|---|---|---|---|---|
| 00 | ult | 7 | **O** | 7 | `(sret)` i=1 | 1:1 (★`i` 번호만 이탈) |
| 01 | calculate_jungle_action_score | **7** | - | 7 | `_rnd` i=1 | 1:1 ← B 는 8 이라고 셌다 |
| 02 | sub_plan(AttackNexus) | 8 | **O** | 7 | `self` i=1 | **IR+1 — sret 미등재** |
| 05 | v50_fold_dive_episode | 3 | - | 5 | | IR-2 (소거 2, role 에 명기) |
| 06 | v2_response_retreat_stance | 5 | - | 6 | | IR-1 (소거 1, role 에 명기) |
| 07 | sub_plan(EpicHuntAndBattle) | 8 | **O** | 8 | `(sret)` **i=0** | 1:1 ★규약 정본 |
| 13 | target_bush_v30 | **5** | - | 3 | | IR+2 (SROA, role 에 `%k` 명기) ← B 는 6 이라고 셌다 |
| 15 | single_try_engage | 8 | **O**(속성 없음) | 8 | `(sret ret)` **i=0** | 1:1 ★규약 정본 |
| 17 | new(DeathMatchBattle) | 5 | **O** | 4 | `version` i=1 | **IR+1 — sret 미등재** |
| 19 | best_jungle_goal | **7** | - | 7 | `version` i=1 | 1:1 ← B 는 8 이라고 셌다 |

(나머지 10함수는 전부 1:1 · sret 없음.)

⚠`15 single_try_engage` 는 `internal fastcc` 라 **`sret(..)` 속성이 없는데도 `%0` 이 반환 슬롯**이다(`ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(144) %0`). `"sret(" in line` 만 보는 판정기(D)는 이걸 못 본다 — 통합판은 `dead_on_unwind AND writable AND writeonly` 를 2차 판정으로 둔다.

---

## 3. ★sret 규약 — 확정과 그 근거

> **확정: sret out-ptr 은 `params[]` 에 싣는다. `params[0]` · `name` = `(sret)` · `i` = 0. 소스 인자는 `i` = 1..n.**

근거 넷:

1. **다수** — sret 형 5함수(`00 02 07 15 17`) 중 **3함수(`00 07 15`)가 이미 싣는다**.
2. **기계 정렬이 자명해진다** — 싣는 쪽이면 `params[j]` ↔ IR `%j` 가 SROA 없는 함수에서 그대로 1:1 이다. 안 싣는 쪽은 게이트가 매번 sret 여부를 추측해야 하고, **그 추측 실패가 이번 오탐 44건 중 7건의 직접 원인**이었다(후보 C).
3. **정보가 중복되지 않는다** — `sig.tcx` 가 이미 소스 시그니처 정본이라 「소스 인자만 싣는다」는 중복일 뿐이다. 반면 `(sret)` 행은 **반환값 레이아웃**이라는 다른 칸에 없는 사실을 담는다(`00`: `+0x0=태그(-1=None, 5=Input::Ult), +0x8=InputTarget(24B)` / `07`: `태그 i64 @+0x0` / `15`: `tag 오프셋 0x0(support_target 의 Option 니치)에 -1 저장 = None`).
4. **되돌리는 비용이 더 크다** — 「안 싣는다」로 통일하면 행 **삭제 3건**, 「싣는다」면 행 **추가 2건**이다. 둘 다 `applypatch` 에 연산이 없지만(아래) 후자가 손실이 작고, 삭제는 3의 정보를 파괴한다.

`i` 번호는 **`07`·`15` 식이 정본**(sret=0, 소스 1..n). `00` 이 유일한 이탈로 sret 에 `i=1` 을 주고 소스 인자를 2..7 로 매겼다 — `sig.tcx` = `fn(usize, &mut StdRng, &PlayerState, &OperationData, &PositioningScoreData, &Entity) -> Option<Input>` 이 **소스 인자 6개**임으로 검산하면 결정적이다. `patch.json` 의 7건이 이 정정이다.

### 규약을 어긴 `02`·`17` 을 왜 못 고쳤나
`applypatch.py` 에는 **배열 원소 추가/삭제 연산이 없다**(`apply_error` 는 스칼라 치환 / 문면 치환 / 배열 원소 키 치환 셋뿐). 그래서:
- 기계로 가능: `00` 의 `i` 재번호 7건 → `errors[]` 로 냈다.
- 기계로 불가능: `02`·`17` 의 `(sret)` 행 삽입 → **인접 행(`params[0]`)의 `note` 에 사실을 박아 두는 것으로 갈음**했다(`kind: 보강`). 그 문면이 「이 표는 sret 를 빠뜨렸다 · 표의 자리 번호가 IR 보다 한 칸 앞선다 · IR 원문은 이것이다」를 담는다.
- **메인이 손으로 넣어야 할 행 2개**(내용 확정):

```json
// specs20.json -> specs[2].signature.params 의 맨 앞에
{"i": 0, "name": "(sret)", "type": "&mut SubPlan(72B)",
 "note": "반환값 out-ptr. m12.ll:34867 ptr dead_on_unwind noalias noundef writable writeonly sret([72 x i8]) align 8 captures(none) dereferenceable(72) %0. 본문이 태그·페이로드를 여기에 store 한다"}

// specs20.json -> specs[17].signature.params 의 맨 앞에
{"i": 0, "name": "(sret)", "type": "&mut DeathMatchBattle(384B)",
 "note": "반환값 out-ptr. m05.ll:17695 ptr dead_on_unwind noalias noundef writable writeonly sret([384 x i8]) align 8 captures(none) dereferenceable(384) %0"}
```
넣은 뒤 `02`·`17` 의 기존 `params[].i` 는 손댈 필요가 없다(이미 소스 1..n). 통합 게이트의 P2 가 자동으로 조용해진다.

**판정: `보류`**(재개 가능 — 도구에 행 추가가 생기거나 메인이 손으로 넣으면 그 자리에서 닫힌다).

---

## 4. ★적발 반증 — 44건 중 42건이 오탐이었다

> 도시에 §1-3: *「적발 건수는 게이트의 성능이 아니라 **가설**이다.」* 그대로였다.

### 4-1. 후보 D (14건 → **14/14 오탐**)

| 대상 | D 의 판정 | 반증 |
|---|---|---|
| `01 data` `02 self` `09 data` `10 data` `14 data` (C1 5건) | 「'안 씀'인데 포인터 사용」 | **절 분해 실패.** 전부 「A(+0x0), B(+0x8) 를 읽음. **C(+0x10) 는 안 씀**」 형태 — 부정의 대상은 인자가 아니라 **필드 하나**다 |
| `01 data 0x10` `10 data 0x8` `14 data 0x10` (C2 3건) | 「role 이 적은 오프셋이 IR 에 없다」 | **부정절의 오프셋**을 긍정 주장으로 읽었다. 실제로 없어야 맞는 값이다 |
| `15 data +0x8` `18 data +0x1e0` (C2 2건) | 동 | **베이스가 다르다.** `data.cache(+0x0)` 를 **로드한 뒤**의 오프셋이다(「그 +0x0/+0x8 이 &dyn AbstractGame 팻포인터」 / 「cache.player_champion(+0x1e0)」). 인자 자신의 gep 사슬엔 당연히 없다 |
| `19 version +0x8` `19 team_plan +0x20` `19 debug +0x28` (C2 3건) | 동 | **클로저 환경의 오프셋**이다. 인자에서 파생된 주소가 아니라 인자가 **담기는** 자리다 |
| `17 goal +0x10` (C2 1건) | 동 | **타입 레이아웃 서술**이다. m05.ll:17708/17713 이 태그(+0x0)와 +0x8 만 로드하고 m05.ll:17769 가 **24B 통째 memcpy** 한다 — `TryKill` 의 둘째 `usize @+0x10` 은 그 24B 안이다 |

### 4-2. 후보 C (15건 → **15/15 오탐**)

- `02` 5건 · `17` 2건 = **sret 미보정으로 인자가 한 칸 밀림**(`17 version` 을 `%0`=sret 슬롯에 대고 「54회 접근」이라 했다).
- `06 rnd` 1건 = **소거 인자 미보정**(`self` 가 IR 인자로 안 넘어오므로 `rnd`=`%1` 인데 `%2`(player)에 댔다).
- `08 team_plan` · `15 self` 2건 = 어휘 `없다` 가 너무 넓다. 「**쓰기는** 없다」·「team_plan **부작용은** 없다」는 인자 미사용 주장이 아니다.
- `07 goal_data 0x78` 1건 = 구조 서술(`epic(0x78)` 안의 tick 두 개 → 절대 0x98·0xa0 = 0x78+0x20·0x78+0x28).
- `15 data 0x8` `18 data 0x1e0` `19 team_plan 0x20` `19 debug 0x28` 4건 = D 와 같은 베이스/클로저 문제.

### 4-3. 후보 A (10건 → **10/10 오탐**)

| 대상 | 반증 |
|---|---|
| `05 _version` `05 _tps` `06 rnd` | **소거 인자 미보정** — A 는 sret 보정만 한다. `05` 는 IR 인자가 3개뿐인데 5개로 대응시켰다 |
| `09 version` `15 rnd` `15 debug` | 판정식 버그: `cu and not cp` 가지가 `readnone` 또는 `uses==0` 만 통과시킨다 — **call 인자로만 쓰인 것**(`callonly=True`)도 「불일치」로 찍는다 |
| `03 version` `19 debug` | `store <ty> %k, ptr %alloca` — **인자를 `&`로 넘기려고 스필하는 store** 다. 「전달」의 구현 수단이지 사용이 아니다 |
| `02 data` | 절 분해 실패(4-1 과 동형) |
| `16 tick` | `CLAIM_PASS` 가 「**호출부는** 40/50/60 리터럴을 **넘긴다**」의 '넘긴다'를 이 함수의 전달 주장으로 읽었다. 주어가 호출부다 |

★A 의 공은 따로 있다: **sret 규약 불일치를 이름 붙여 지적한 유일한 후보**다(`★인자표가 sret 를 세지 않는다 ... 규약 불일치`). 통합판의 P2 는 A 의 이 판정을 승계했다.

### 4-4. 후보 B (5건 → **2건 진짜 · 3건 무효**)

| 대상 | B | 실제 |
|---|---|---|
| `specs[1] 8vs7` | 인자 개수 불변식 깨짐 | **오탐** — IR 인자는 **7개**. `range(i64 ..., 206)` 를 인자 괄호로 잡은 파서 버그 |
| `specs[19] 8vs5` | 동 | **오탐 2중** — IR 인자는 **7개**(`range(i8 0,6)` 버그) + `19 version`·`19 debug` 의 「**is_cleared 호출 인자 자리**에 poison」을 이 함수 인자 소거로 오독(B 가 `09` 에서 자체 적발한 바로 그 함정의 재발 — 필터가 `피호출자` 라는 **낱말**만 보기 때문) |
| `specs[13] 6vs3` | 동 | **수치 오류 + 해석 오류** — IR 인자는 **5개**이고 차이는 sret 가 아니라 **SROA 인자 승격**이다(`self.line`→`i8 %0`, `player`→`i64 %1`+`i32 %2`, `data`→`ptr %3`+`ptr %4`). role 이 이미 `%k` 로 전부 적어 두었으므로 **명세는 옳다** |
| `specs[2] 8vs7` | 동 | ★**진짜** — sret 미등재 |
| `specs[17] 5vs4` | 동 | ★**진짜** — sret 미등재 |

B 의 (2) 속성 주장 대조(`readnone`/`readonly`/`captures(none)`)는 **적발 0** 인데, 통합판에서 정렬을 고친 뒤 다시 돌려도 **0** 이다(주장 11행 전수 통과). 살아 있는 검사인지 확인했다: `02 _team_plan` role 「readnone — 미사용」 ↔ IR `ptr noundef nonnull readnone align 8 captures(none) %6` / `14 _positioning_score` role 「readonly 로 표기됐지만 사용 0회」 ↔ IR `ptr noalias noundef readonly align 8 captures(none) dereferenceable(2760) %6`.

---

## 5. 통합 게이트 `gate.py` (= `G16` 승격 후보)

진입점 `check_spec(sp) -> [(params 인덱스, 사유, 상세)]` — `specgate.py` 의 `G13`(`whereline.check_spec`)과 같은 계약.

### 5-1. 설계 — **기각에 쓸 수 있는 것만 기각에 쓴다**

`SPEC_RUNBOOK §S5-b` 의 G12 교훈(*귀속은 명세를 **구제**할 수는 있어도 **기각**할 수는 없다*)을 그대로 적용했다. §4 가 보여주듯 **오프셋 인용 축은 기각에 쓸 수 없다** — `+0xNNN` 이 그 인자의 gep 오프셋이라는 보장이 구조적으로 없다(하위 객체 / 클로저 환경 / 타입 레이아웃 / memcpy 안). **약한 후보로 강등**했다(`--weak` 로만 출력).

| 코드 | 무엇 | 대상 행 | 이번 적발 |
|---|---|---|---|
| **P1** 정렬 불변식 | IR `define` 인자 수 ↔ params 수 (sret·소거·SROA 보정 후) | 20함수 | 0 (20/20 정렬 성공) |
| **P2** sret 규약 | sret out-ptr 이 `params[0]`(i=0)로 실려 있는가 | 5함수 | **2** (`02`·`17`) |
| **P3** `i` 번호 규약 | `i` = 소스 위치(1-based), sret=0 — **`sig.tcx` 인자 수로 검산** | 20함수 | **1** (`00`) |
| **P4** 속성 주장 | role 의 `readnone`/`readonly`/`captures(none)`/`writeonly` ↔ `define` 원문 | 11행 | 0 |
| **P5** 전면 미사용 | 절 전체가 인자를 부정하는데 call 인자 밖에서 쓰이나 | 59행 | 0 (강) / 8 (약) |
| **P6** IR 인용 대조 | role 의 `mNN.ll:LINE` 존재 + 백틱 인용 명령이 그 줄에 있나 | 5행 | 0 (5/5 통과) |
| **E** 오프셋 인용 | 인용 오프셋 ⊆ 인자 사슬(깊이 ≤1) | 44행 | **약한 후보 전용** — 11 |

**합계: 강 3 · 약 11.**

### 5-2. 통합판이 후보들에게서 가져온 것

- **A** → sret 규약 불일치 판정 · `@심볼` 뒤 괄호 파싱
- **B** → 인자 개수 불변식 · 속성 주장 대조 · 「피호출자 poison != 소거」 교훈
- **C** → **절 단위 분해** · SROA 인자 제외
- **D** → sret 자리 보정 · gep 루트 추적(오프셋 귀속)
- **새로** → 소거를 **잔차로 계산**(주장으로 세지 않는다) · `internal fastcc` sret 2차 판정 · 스필 store 구제 · `sig.tcx` 검산(P3) · IR 인용 대조(P6) · 오프셋 축 강등

### 5-3. 7차에 메인이 밟은 함정을 피한 방법
> *「추론을 후보에 그냥 더했더니 오탐이 16 → 22 로 늘었다.」*

이번 통합판에서 추론(절 분해 · 스필 인식 · 깊이 1 오프셋 · 타객체 신호어)은 **전부 「걸러내는」 방향으로만** 걸었다. 후보를 늘리는 추론은 하나도 넣지 않았다. 결과: 초판 7건 → 정제 3건(줄어드는 방향). **한 번도 늘지 않았다.**

### 5-4. 숨은 결함이 없는지 — 124행 전수 실측 (`run/audit.py`)
P5 를 좁히면 「진짜 위반을 덮는 것 아닌가」가 당연한 의심이라, **124행 전량**에 대해 `부정 주장 여부 x (call 밖 사용 - 스필)` 을 표로 떴다. 걸린 행은 **8행**이고 **8행 전부** 「A 를 읽음. **B 는 안 씀**」 형태의 부분 부정이었다(`01/2 data` `02/0 self` `02/4 data` `09/2 data` `10/3 data` `13/0 self` `13/2 data` `14/4 data`). 표본 검증: `09 data` 는 off0={0x8}·0x10 없음 ⟹ 「blackboard(0x10)은 안 읽음」이 **참**, `10 data` 는 off0={0x10}·0x8 없음 ⟹ 「context(+0x8)는 안 쓴다」가 **참**.
⟹ **이 축의 「미사용」 주장 59행은 전수 참이다.** P5 가 0 을 내는 것은 결함이 아니라 실측이다(6차 `offset_of!` 계측기가 0 을 낸 것과 같은 성격).

---

## 6. 게이트 밖에서 손으로 찾은 실오류 1건

**`specs[3] defensive_crisis` `params[4] target`** — role: 「위기 판정 대상 챔피언(보통 아군 자신). **id 와 좌표만** 직접 쓰인다」

IR 원문(`m10.ll:33864~34294`, `%4` = `ptr ... dereferenceable(1728)` = `&Entity`):
```
33929   store ptr %4, ptr %33, align 8, !dbg !40740                          <- 클로저 환경에 담김
33981   %46 = getelementptr inbounds nuw i8, ptr %4, i64 1472, !dbg !40815   <- +0x5c0
33982   %47 = load i64, ptr %46, align 8, !dbg !40815, !noundef !8
33983   %48 = call fastcc ... @...AbstractGameWithCache21player_by_champion_id(... %18, i64 noundef %47)
```
- `+0x5c0`(1472) 하나뿐이고 그 값은 `player_by_champion_id` 의 인자다 ⟹ **id ✓**.
- 좌표는 **없다**: 같은 범위에 `ptr %4, i64 1632`(x=0x660) · `i64 1640`(y=0x668) gep 가 **0건**이다(`00 target`·`09 target_enemy` 가 같은 `&Entity`(1728B)에서 0x660/0x668 을 읽으므로 오프셋은 확정).
- `%4` 가 클로저 환경으로 escape 하므로 좌표 사용이 **어딘가에** 있을 수는 있으나, role 이 「**직접**」이라고 한정했으므로 그 escape 는 구제가 되지 않는다.

⟹ `errors[]` 로 정정(`behavior_change: false` — 서술 오류이지 재구현을 틀리게 하지는 않는다. 다만 재구현자가 없는 좌표 읽기를 찾게 만든다).

⚠**이 결함은 통합 게이트가 못 잡는다.** role 이 `+0x` 로 인용하지 않고 「id 와 좌표」라는 **한국어 낱말**로 적었기 때문이다. 「명명 필드 나열 ↔ 실제 gep 오프셋 개수」 대조(가칭 P7)를 넣으면 잡히지만, 한국어 나열 파싱은 오탐이 지배할 것이 뻔하고 **1건 잡으려고 규칙을 넣는 것이 정확히 7차 G12 의 과대적합**이라 넣지 않았다.
판정: **`미탐색`(범위 = 기계 검사로는 미탐색. 손으로는 20함수 124행 전수 확인 완료 — 이 1건 외엔 없다)**.

---

## 7. ★내 지시(도시에)의 오류 — 5건

`patch.json` 의 `brief_errors[]` 와 같은 내용. 요지:

1. **경로 문법이 틀렸다.** §5 가 `/specs[3]/sig/params[1]/role` 을 예시로 싣는데 `applypatch` 는 **v2** 에 적용하고 v2 의 키는 **`note`** 다(`mkspec3.py:295` = `"role": p.get("note")`). 실측: `role` 판 = **정정 7/10, 실패 3건**(`old` 값이 정본과 다르다(현재 None)) / `note` 판 = **정정 10/10**. 두 판 모두 `_verify8/C/run/` 에 남겼다. — **이 축을 맡는 배치는 전원이 밟는다.**
2. **임무 3번이 현재 도구로 실행 불가능하다.** 「sret 규약을 확정하고 안 맞는 명세를 `errors[]` 로 고쳐라」는 어느 쪽으로 통일하든 행 추가(2건) 또는 삭제(3건)를 요구하는데 `applypatch` 에 둘 다 없다. §5 가 스스로 「append 는 구현이 없다」고 적어 놓고 임무에서 그것을 요구한다. ⟹ 규약은 확정했고 기계 가능분만 냈다(§3).
3. **「B 가 지목한 5건」 목록이 부정확하다.** 「앞 넷은 sret out-ptr 을 `params` 에 안 실은 것으로 보인다」 → 실제로 sret 인 것은 **`02`·`17` 둘뿐**이다. `specs[1] 8vs7`·`specs[19] 8vs5` 는 B 의 파서 버그이고 `specs[13] 6vs3` 은 SROA(수치도 5vs3 이 맞다). **판정 반전 3건**으로 센다.
4. **「`피호출자` 가 들어간 role 은 제외해야 한다」는 처방이 불충분하다.** `19 version`·`19 debug` 는 「is_cleared **호출 인자 자리**에는 poison」·「is_cleared **호출 시** poison」이라 그 낱말이 없다. 낱말 필터로는 못 막는다 — **소거는 잔차로 세야 한다**(통합판은 그렇게 고쳤다).
5. §4 게이트 현황 줄이 사전순으로 섞여(`G1 ... G12 ... G2 G3 ...`) 자기 몫을 눈으로 찾기 어렵다. 사소하지만 매 라운드 같은 자리다.

**관측 하나 더**(오류 주장이 아님): 도시에 §4 는 `G9 callees 오염=4` 라고 적는데 지금 `specgate.py` 헤더의 `G9` 는 **0** 이다. 나는 `_gates/`·`MIG` 루트를 손대지 않았으므로 다른 배치/메인의 도구 갱신으로 보인다. **정본 해시(`dossierfresh`)는 FRESH** — 즉 신선도 검사는 명세 3종만 보고 **게이트 출력의 신선도는 안 본다**. §4 숫자가 생성 시점 스냅숏임을 명기하는 편이 좋겠다.

---

## 8. 여력분 — `G12` 잔여 8건

전수 반증은 못 했다(`미탐색`이 아니라 **부분 검증**). 확인한 것:

| 대상 | 판정 | 근거 |
|---|---|---|
| `09 consts[0]` src_line=1209 (값 2) | **오탐** | meaning 이 「IR 에선 `shl i64 %30,1` 로 접혔다」고 적었고 **실제로 m15.ll:35471 `%31 = shl i64 %30, 1, !dbg !46299` 가 있다.** 리터럴 2 는 소멸했으므로 `srclinecheck` 의 후보 [1203,1280] 은 전부 **무관한 2**(bounds check)다 |
| `09 consts[5]` src_line=1280 (값 4) | **오탐** | 동. m15.ll:35776 `%191 = shl i128 %176, 2, !dbg !46432` |
| `02 consts[4]` src_line=50 (값 16) | **오탐** | 값 16 의 유일한 실거처는 m12.ll:34973 `%68 = phi i64 [ 5, %49 ], [ 2, %61 ], [ 16, %55 ]` — **phi 인입**이라 `!dbg` 가 없다. 게이트의 후보 37 은 **귀속 추론**으로 얻은 것이고 `S5-b` 규칙상 **귀속은 기각 근거가 될 수 없다** |
| `01 consts[10]`(값 0) · `16 consts[5]`(값 0) | **오탐 추정** | 값 0 은 IR 어디에나 있고 어디에도 없다. 강한 후보가 원리적으로 안 생긴다 |
| `05 consts[2]`(값 1) · `05 consts[8]`(값 7) · `09 consts[4]`(값 9) | **미검증** | `!dbg` 사슬을 읽어야 한다. 손대지 않았다 |

★**G12 에 제안**(도구는 안 고쳤다 — 금지 준수):
> **강한 후보가 0개인 상수는 「불일치」가 아니라 「검사 불가(접힘/소멸)」로 분류하라.**
> 지금 판정식은 「강한 후보가 있는데 claim 이 그 안에 없으면 불일치」인데, `shl` 로 접힌 상수·값 0/1 상수는 **강한 후보가 아예 0개**다. 그때 약한 후보(귀속)만 남고 그걸로 판정하면 7차가 이미 밟은 함정을 다시 밟는다. 위 5건이 정확히 그 형태다. 이 분류 하나만으로 **8 → 3**.

---

## 9. 산출물

```
_verify8/C/patch.json     errors 10 · ev_up 0 · brief_errors 5   (--dry 10/10 성공 · 실패 0)
_verify8/C/gate.py        통합 게이트(G16 후보). check_spec(sp) 진입점
_verify8/C/REPORT.md      이 문서
_verify8/C/run/           재현 보조 — dump_defs.py · truth.py · audit.py · memdir.py(C 후보 구동용 복사본)
                          patch_role.json / patch_note.json (§7-1 경로 실험의 두 판)
```

`ev_up` 이 비어 있는 이유: 이번 라운드의 증거는 전부 **IR 등급(ev4)** 이다(`define` 헤더 · 본문 사용 · gep 사슬). 대상 행들은 이미 ev4 이상(다수가 ev2~3)이라 **올릴 근거가 없다.** 없는 근거로 등급을 올리는 것은 도시에가 금지한 「추론으로 남의 주장 뒤집기」와 같은 종류의 잘못이다.
