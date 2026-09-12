# 9차 배치A — 축 `consts[].src_line`(G12) 남은 8건 가리기

> 결론 한 줄: **8건 전부 오탐(8/8).** 실오류 0. 고친 게이트는 전 20함수에서 **8 → 0건**이고,
> 동시에 **검사 가능 상수가 145 → 153** 으로 늘었으며, 변이 시험 포착률은 **94.8% → 100%** 다.

---

## §0 실행한 명령줄 (그대로)

```bash
cd /c/tfm2mods/MIG && python -X utf8 dossierfresh.py 9 A        # 착수 시 FRESH / 제출 직전 FRESH
cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 specgate.py --gate G12

# 8건을 IR 원문으로 가리기 (조사 보조 2종을 새로 만들었다)
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 inspect.py 9 0
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 inspect.py 9 4
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 inspect.py 9 5
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 inspect.py 16 5
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 inspect.py 1 10
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 inspect.py 2 4
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 inspect.py 5 2
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 inspect.py 5 8
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 res.py m05.ll 44405 44413 44396 44402 44410 44345
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 res.py m15.ll 46436 46437 46438 46432 46222 46224 46226
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 res.py m13.ll 35929 35937 35942 35951 35953 35963 35955 35916
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 res.py m12.ll 62985 62970 62988 62969 62968
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 res.py m10.ll 56148 56262 56166 56276 56302 56282
cd /c/tfm2mods && sed -n '40030,40103p' _gaibc/m05.ll    # 01 의 switch -> phi 전량
cd /c/tfm2mods && sed -n '35740,35800p' _gaibc/m15.ll    # 09 의 is_front/is_rear/is_flank 블록
cd /c/tfm2mods && sed -n '29100,29160p' _gaibc/m13.ll    # 05 의 end_plan phi
cd /c/tfm2mods && sed -n '34925,34976p' _gaibc/m12.ll    # 02 의 SubPlan phi
cd /c/tfm2mods && sed -n '51913,51975p' _gaibc/m10.ll    # 16 의 range 초기화

# 고친 판 · 전 20함수 재측정 · 변이 시험 · 면죄율 · 신규 커버리지
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 gate.py
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 mutate.py gate
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 mutate.py srclinecheck
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 excuse.py
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 newcov.py
cd /c/tfm2mods/MIG/_verify9/A && PYTHONIOENCODING=utf-8 python -X utf8 mkpatch9a.py   # kind 안정성 사전검사 포함
cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 9 --only A --dry
```

산출물: `gate.py`(통합 게이트) · `mutate.py`(변이) · `excuse.py`(면죄율) · `newcov.py`(신규 커버리지)
· `inspect.py`·`res.py`(조사 보조) · `mkpatch9a.py`(patch 생성 + kind 사전검사) · `patch.json`.
⛔`MIG` 루트의 도구는 **하나도 고치지 않았다**(§4 에 적용안만 낸다).

---

## §1 8건 전수 판정 — **8/8 오탐**

| # | 대상 | 값 | 주장 | 게이트 후보 | 판정 | IR 원문 근거 |
|---|---|---|---|---|---|---|
| 1 | `01 calculate_jungle_action_score` c10 | 0 | L536 | [542,548] | **오탐** | `m05.ll:40074 %32 = phi i64 [ 0, %45 ], ...` 인입 -> 블록 `%45`(switch default) 종결자 `br label %31, !dbg !44413` = `!DILocation(line: 536)` |
| 2 | `02 sub_plan` c4 | 16 | L50 | [37] | **오탐** | 유일한 강후보가 `m12.ll:34916 getelementptr inbounds nuw i8, ptr %30, i64 16` = **필드 오프셋**. 진짜 16 은 `m12.ll:34973 %68 = phi i64 [ 5,%49 ],[ 2,%61 ],[ 16,%55 ]` 인입 |
| 3 | `05 v50_fold_dive_episode` c2 | 1 | L146 | [157] | **오탐** | `m13.ll:29154` phi 인입 `[ 1, %33 ]` -> `%33` 종결자 `!35929` = `line: 146` |
| 4 | `05 v50_fold_dive_episode` c8 | 7 | L152 | [139] | **오탐** | 같은 phi 의 `[ 7, %66 ]` -> `%66` 종결자 `!35963` = `line: 152`. 후보 139 는 **end_reason 쪽**(`%23 = icmp eq i8 %2, 7`) |
| 5 | `09 check_favorable_engage_formation` c0 | 2 | L1209 | [1203,1280] | **오탐** | 접힌 자리 `m15.ll:35471 %31 = shl i64 %30, 1, !dbg !46299` = `line: 1209` |
| 6 | `09 ...` c4 | 9 | L1283 | [1286] | **오탐** | `m15.ll:35754 %184 = mul i128 %175, 9, !dbg !46436` = `line: 0`(병합). 소비자 `%189 = icmp sgt i128 %188, %184, !dbg !46437` = `line: 1283`. 변수 `!46224 = DILocalVariable(name "is_rear_direction", line: 1283)` |
| 7 | `09 ...` c5 | 4 | L1280 | [1233] | **오탐** | 접힌 자리 `m15.ll:35776 %191 = shl i128 %176, 2, !dbg !46432` = `line: 1280`. 변수 `!46222 = DILocalVariable(name "is_front", line: 1280)` |
| 8 | `16 max_range_nearly_can_use` c5 | 0 | L2398 | [2403,2410,2417,2424] | **오탐** | `m10.ll:51931 #dbg_value(i64 0, !56148, ...)` + `!56148 = DILocalVariable(name "range", file: !1808, line: 2398)` |

★**8차 배치C 가 오탐이라 한 3건(09c0·09c5·02c4)은 재확인해 동의**하되, 근거는 더 강해졌다 —
「리터럴이 소멸했다」가 아니라 **접힌 그 명령의 `!dbg` 가 주장한 줄 바로 그것**이다(09c0=1209 · 09c5=1280).
즉 그 둘은 「검사 불가」가 아니라 **IR 로 확정 가능**한 항목이었다.

### 오탐을 만든 원인은 7가지였고 서로 다르다

| # | 원인 | 해당 |
|---|---|---|
| ① | `litpat` 가 **phi 인입을 아예 매치하지 못한다** — `[ 1, %33 ]` 은 타입 접두도 쉼표 뒤도 아니다. ⟹ 7차에 넣은 `_term_dbg` **phi 구제 경로가 한 번도 발화한 적 없는 죽은 코드**였다 | 1·3·4 |
| ② | `getelementptr` 오프셋이 `i64` **타입 접두를 달고** 7차 화이트리스트를 통과 | 2 |
| ③ | 곱셈이 `shl` 로 접힘(x2 -> `shl _,1`, x4 -> `shl _,2`) | 5·7 |
| ④ | **병합 위치**(`!DILocation(line: 0)`) — CSE 로 한 명령이 두 소스줄을 섬기면 LLVM 이 line 0 을 준다 | 6 |
| ⑤ | `#dbg_value` 줄을 통째로 버림 — 그 안에 **변수 선언줄**이라는 정답이 있었다 | 8 |
| ⑥ | `phi` 자신의 `!dbg` 를 강한 후보로 셈(phi 위치는 병합 위치라 못 믿는다) | 1·3·4 |
| ⑦ | `shl` 의 **시프트 자릿수**를 상수로 착각(`shl i128 %176, 2` 의 2 는 사실 x4) | 5 |

---

## §2 고친 게이트 — 전 20함수 재측정 (★7차의 「고치다 16->22」 재발 여부)

```
현행 srclinecheck.py   G12 = 8건    (specgate.py --gate G12)
9차 통합판 gate.py     G12 = 0건    (gate.py 단독 실행 = 전 20함수)
```

★**늘지 않는 것이 구조적으로 보장된다.** 이번에 더한 것은 거의 전부 두 종류다:
**(a) 후보 제거**(gep 제외 · phi 자기 `!dbg` 강등) **(b) 약한 후보 추가**(구제 전용).
판정식이 「강한 후보가 있는데 claim 이 강·약 어디에도 없으면 불일치」이므로,
강한 후보를 **늘리지 않는 한** 적발은 늘 수 없다. 7차가 실패한 이유는 귀속을 **강한 후보에 섞었기** 때문이다.
예외로 딱 하나 강한 후보를 **늘렸다** — `invoke` 두 줄 처리(§3). 그래도 0건을 유지했다.

| 지표 | 현행 `srclinecheck.py` | 9차 통합판 |
|---|---|---|
| G12 적발 | 8건(전부 오탐) | **0건** |
| 검사 가능 상수 | 145 | **153** |
| 검사 불가(접힘·범위 밖) | 41 | **33** |
| 면죄율(맞다고 인정하는 줄 / 함수 참조줄) | 12.3% (412/3361) | 13.3% (446/3361) |

면죄율이 **+1.0%p(상수당 +0.22줄)** 밖에 안 올랐다 = 구제를 늘렸어도 게이트가 눈이 멀지 않았다.
면죄율 50% 초과(사실상 검사 안 됨)는 통합판에서도 **1개**뿐이다(`16 consts[0]` 8/13 — 값 -1 이
`Option` 니치와 `level-1` 양쪽에 쓰여 원래 줄 공간이 좁다).

---

## §3 변이 시험 — 「0건」이 무능이 아님의 근거

일부러 틀린 `src_line` 을 넣고 되잡는지 센다(정본 불변 · 메모리 사본).
분모는 **검사 가능 상수**(강한 후보가 있는 것)로 통일했다 — 접힌 상수는 어느 게이트도 못 잡으니 무능의 증거가 아니다.

| 변이 | 뜻 | 현행 `srclinecheck` | **9차 통합판** |
|---|---|---|---|
| `far` | `claim+1000`(함수 밖) | 145/153 = 94.8% | **153/153 = 100%** |
| `mix` | 그 함수가 참조하는 **다른 소스 줄**로 바꿔치기(강·약 후보는 제외) | 145/153 = 94.8% | **153/153 = 100%** |
| `adj` | `claim+1`(한 줄 밀림) | 140/147 = 95.2% | **147/147 = 100%** |

⟹ **적발 0 은 「축이 깨끗하다」쪽이다.** 게이트는 넣은 변이를 전부 되잡는다.
현행판이 놓친 8건은 `05 consts[5][6][7][11][12][13]` · `08 consts[0]` · `10 consts[4]` 로,
전부 `invoke` 인자 상수(`i64 noundef 11`)라 **매치 자체가 안 되던 것**이다 — 이번에 흡수해 검사 범위에 들어왔다.

`newcov.py` 실측 — 새로 검사 가능해진 8개 중 **6개는 claim 이 강한 후보에 정확히 들어맞았고**(직접 확증),
2개는 구제로 통과했다:

```
  specs[ 5] v50_fold_dive_episode  consts[5 ] val=4     claim=L149  강=[149]          claim(강)
  specs[ 5] v50_fold_dive_episode  consts[6 ] val=5     claim=L150  강=[153]          claim(약·구제)
  specs[ 5] v50_fold_dive_episode  consts[7 ] val=6     claim=L151  강=[150,151,152]  claim(강)
  specs[ 5] v50_fold_dive_episode  consts[11] val=11    claim=L146  강=[146]          claim(강)
  specs[ 5] v50_fold_dive_episode  consts[12] val=13    claim=L147  강=[147]          claim(강)
  specs[ 5] v50_fold_dive_episode  consts[13] val=12    claim=L151  강=[151]          claim(강)
  specs[ 8] is_end                 consts[0 ] val=4     claim=L164  강=[169,174]      claim(약·구제)
  specs[10] should_end_..._battle  consts[4 ] val=25000 claim=L1233 강=[1230,1233]    claim(강)
```
`08 consts[0]` 의 구제는 **명세 본문이 스스로 적어둔 사실**(「`src_line=164` 는 맞지만 IR 에 리터럴이 없다 —
살아남는 건 L169·L174 뿐」, ev=3)과 일치한다 = 독립 확인.

---

## §4 `srclinecheck.py` 를 어떻게 고치면 되는가 (메인이 적용 — 나는 안 고쳤다)

`_verify9/A/gate.py` 가 통합판 전문이다. `MIG\srclinecheck.py` 로 옮길 때의 최소 diff 는 이것이다.

1. **`litpat`** — 인자 속성 끼임을 허용한다(gep 은 3번에서 따로 막으므로 여기선 안 좁힌다).
   ```python
   _ATTR = r"(?:(?:noundef|zeroext|signext|immarg|nonnull|inreg|returned|range\([^)]*\))\s+)*"
   def litpat(val):
       return re.compile(r"(?:\b" + _TY + r"\s+" + _ATTR + r"|,\s*)" + re.escape(str(val)) + r"(?![\w.])")
   ```
2. **phi 인입을 매치 대상에 넣는다** — `litpat` 에 `\[` 를 더하지 말 것(`[4 x i64]` 같은 타입이 딸려 온다).
   phi 줄만 `PHIIN` 으로 따로 인식한다:
   ```python
   ISPHI = re.compile(r"^%[\w.$]+\s*=\s*phi\b")
   ...
   isphi = bool(ISPHI.match(s))
   hit = bool(pat.search(ln)) or (isphi and any(v == str(val) for v, _ in PHIIN.findall(ln)))
   ```
   ★이게 **가장 큰 한 줄**이다 — 7차에 넣은 `_term_dbg` 구제가 이것 없이는 영영 발화하지 않는다.
3. **`getelementptr` 명령은 후보에서 제외**(`GEP = re.compile(r"^(?:%[\w.$]+\s*=\s*)?getelementptr\b")`).
   후보를 **줄이기만** 하므로 새 오탐을 만들 수 없다.
4. **phi 자신의 `!dbg` 는 약한 후보로 강등**(`(wk if isphi else st).append(...)`).
5. **접힌 곱셈 구제(약)** — 값이 2^k(k>=1)이면 `shl _, k` 줄의 `!dbg` 를 약한 후보로.
6. **병합 위치 구제(약)** — 매치한 명령의 자기 `!dbg` 가 담당 파일 줄 0(또는 없음)이면 그 값의
   **소비자**(`%N` 을 피연산자로 쓰는 줄) `!dbg` 를 약한 후보로.
7. **`#dbg_value` 구제(약)** — `#dbg_value(<ty> <val>, !VAR, ...)` 의 `!VAR` 가 담당 파일의
   `DILocalVariable` 이면 그 `line:` 을, 뒤쪽 `!LOC` 이 0 이 아니면 그 줄도 약한 후보로.
8. **`invoke` 두 줄 처리(강)** — 매치 줄에 `!dbg` 가 없고 **다음 줄이 `to label ...`** 이면 그 줄의 `!dbg` 를 쓴다.
   같은 명령의 이어쓰기라 **추론이 아니다**. (7차 배치B 후보 `_gates/src_line/B_srclinecheck2.py` 의 수확)

⚠**8차 배치C 제안(「강한 후보 0개면 검사 불가로 분류」)은 적용할 것이 없다** — 이미 `check_spec` 의
`cands = sorted(x for x in found if x)` + `if cands and ...` 가 그 판정이다. 8건 중 **0건**이 달라진다.

⚠`srclinecheck.py` 의 `__main__` 리포트 루프는 `litpat` 을 안 쓰고 **초판 정규식**(`(?<![\w.\-])val(?![\w.])`)을
그대로 쓴다. `check_spec` 과 **다른 답을 내는 죽은 경로**이므로 같이 갈아끼우거나 지워야 한다(이번엔 안 건드렸다).

---

## §5 판정 어휘

- `02 consts[4]`(값 16 @ L50) = **표기 불가(IR 귀속 방식 한정)**. 동작·값은 확정이고 줄만 IR 로 못 가린다.
  phi 인입의 인입 블록 `%55` 종결자가 `!62988`=L47 인데 **else 팔 `%61` 종결자도 같은 `!62988`** 이라
  팔을 못 가른다(= `if` 식 자체의 위치). ⟹ 귀속에 판별력이 없고, 「구제에만 쓴다」 원칙대로 **기각 근거로 쓰지 않았다.**
  L50 은 기존 `rmeta_srcmap` 글자수 근거로 유지한다. 시도한 재료 범위: ①phi 인입 귀속 ②소비자 귀속
  (`store i64 %68, !dbg !62968` = line 0) ③`#dbg_value`(해당 값에 없음) ④`shl` 접힘(해당 없음).
- 나머지 7건 = **오탐 확정**(IR 원문으로 주장한 줄을 직접 되찾음).
- **실오류 0건** — 이 축에 남은 값 오류는 없다.

## §6 내 지시(도시에)의 오류 — 4건

`patch.json` 의 `brief_errors[]` 에 그대로 실었다. 요지:

1. ★**「8차 배치C 제안을 검증하고 맞으면 코드 수준으로 반영하라」는 지시가 성립하지 않는다.**
   그 제안(강한 후보 0 -> 검사 불가)은 **이미 구현돼 있고**, 8건 중 **0건**을 바꾼다.
   8건 전부 강한 후보를 갖고 있었다(오탐의 원인은 「없어서」가 아니라 **엉뚱한 게 있어서**다).
2. ★**8차 배치C 의 귀속이 절반만 맞았다(판정 반전 1건).** `02 consts[4]` 를 「phi 인입이라 `!dbg` 없음」으로
   설명했는데 그건 **정답을 못 본 이유**고, **오답을 낸 이유는 gep 오프셋**이다. 7차가 「gep 잡음 제거」라고
   적었지만 gep 인덱스는 `i64` 타입 접두를 달고 화이트리스트를 그냥 통과한다.
3. 도시에 §1 이 `_gates/src_line/` 을 「결과가 갈리는 후보들」로 소개하지만, 두 파일은 7차에 이미 본체로
   흡수된 **선행 제안**이다. 실제로 남아 있던 미흡수분은 배치B 의 `invoke` 두 줄 하나였다(이번에 흡수).
4. 도시에 §2 필독 정본 표에 **`MIG\srclinecheck.py` 자신이 없다**(지시문에만 있다). 도시에만 읽고 착수하면
   G12 본체를 안 읽고 시작한다.

## §7 patch.json

- `errors[]` **8항목** — 전부 `kind="오탐"`. 값(`src_line`)은 **하나도 안 바꾼다**(전부 맞았다).
  바꾸는 것은 `meaning` 뿐으로, **그 줄이 IR 어디서 나온 근거인지**를 박아 넣는다
  (다음 라운드가 같은 8건을 또 가리지 않게 하는 것이 목적).
- `behavior_change` = 전건 `false`. `src_line` 은 재구현 동작에 들어가지 않는 출처 표기이고,
  값·의미는 이번에 바뀌지 않았다.
- `found_by` = `new` 5 / `reused` 3(8차 배치C 가 짚었던 09c0·09c5·02c4).
- `ev_up[]` = 없음. 근거는 **IR 독해**이고 이 축의 `ev` 파생은 실행 확증 문면에서 나오므로 등급을 올리지 않았다.
- ★`meaning` 을 고치면 `mkspec3._kind` 가 낱말을 다시 읽어 **`consts.kind` 가 뒤집힐 수 있다**(G15 가 다시 열린다).
  그래서 `mkpatch9a.py` 가 고친 문면으로 `_kind` 를 **재계산해 비교**한다 — 8/8 `OK`(변동 없음).
- 사전 검증: `applypatch.py 9 --only A --dry` -> `정정 8/8 성공 · 실패 0 · 동작 변경 0건 · new=5 · reused=3`.
- 도시에 신선도: 착수 시 **FRESH** / 제출 직전 **FRESH**.
