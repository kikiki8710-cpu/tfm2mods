# -*- coding: utf-8 -*-
u"""> ⚠ **STALE / 폐기 — 채택하지 마라.** (최신 = MIG 루트의 `srclinecheck.py`, 2026-09-11 18:09 판)
>
> 이 파일은 7차 배치C 가 G12 오탐을 독립 진단하며 만든 **중간 산출물**이다. 진단(「`switch` 케이스
> 값 줄에 `!dbg` 가 없어 태그 상수가 통째로 오탐된다」)은 상류 교정과 **같은 결론**이었지만,
> **고치는 방향이 틀렸다.**
>
> 여기서는 귀속시킨 `!dbg` 를 **후보 집합에 그냥 더한다.** 그런데 G12 의 판정식은
> 「후보가 있는데 claim 이 그 안에 없으면 불일치」라서 **후보를 늘리면 오탐이 늘어난다** —
> 실측으로 이 파일은 배치C 12건을 0 으로 만드는 대신 **전 20함수에서 10 → 19 로 늘렸다.**
> 상류가 이미 같은 함정을 밟고 문서로 남겨 두었다(`srclinecheck.py` 머리말:
> 「귀속은 **약한 후보**다 — 구제에만 쓰고 기각에는 쓰지 않는다. 16 → 22 로 늘었다」).
>
> ⟹ **올바른 설계 = ①리터럴 화이트리스트(타입 접두·쉼표 뒤만 상수) ②귀속은 claim 구제에만.**
> 남겨 두는 이유는 「같은 오답을 두 번 만들지 않기 위해」다. 아래 본문은 원문 그대로 둔다.

srclinecheck(G12) 수정판 — **다중행 LLVM 명령의 `!dbg` 를 연속행에 이어 준다.**

## 왜 필요한가 (7차 배치C 발견)
G12 초판은 `한 줄 = 한 명령` 을 전제하고 `!dbg` 가 없는 줄을 통째로 버린다. 그런데 LLVM 텍스트 IR 에서
**두 종류의 명령이 여러 줄에 걸쳐 인쇄되고, `!dbg` 는 마지막 줄에만 붙는다**:

```llvm
  switch i8 %28, label %44 [      ; ← 여기엔 !dbg 가 없다
    i8 0, label %38               ; ← 케이스 값(리터럴!)도 없다
    i8 2, label %38
  ], !dbg !36362                  ; ← !dbg 는 여기

  %39 = invoke ... (i64 25000)    ; ← 인자(리터럴!)는 여기
          to label %53 unwind label %50, !dbg !36406   ; ← !dbg 는 여기
```

⟹ **`match`/`switch` 의 케이스 값과 `invoke` 의 인자 리터럴은 G12 에 보이지 않는다.**
열거형 태그 상수는 거의 전부 switch 케이스로 내려가므로, **태그 종류의 `consts` 는 구조적으로
전부 「불일치」로 오탐**된다. 7차 배치C 담당분 12건 중 **11건이 이 오탐**이었다.

## 고친 방법
범위를 훑으며 **논리 명령 단위**로 묶는다:
- `switch ... [` 로 열려 `]` 로 닫히는 블록 → 닫는 줄의 `!dbg` 를 블록 전 줄에 적용
- `invoke`/`callbr` 머리줄 → 뒤따르는 `to label ... unwind label ...` 줄의 `!dbg` 를 머리줄에 적용
그 외 `!dbg` 없는 줄은 **초판과 똑같이 버린다**(과잉 귀속 방지 — 이웃 명령의 줄을 빌려주면
진짜 오류를 가린다).
"""
import io, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import srclinecheck as S

SW_OPEN = re.compile(r"^\s*switch\b.*\[\s*$")
SW_CLOSE = re.compile(r"^\s*\]")
INV_HEAD = re.compile(r"^\s*(?:%\S+\s*=\s*)?(?:invoke|callbr)\b")
INV_CONT = re.compile(r"^\s*to label\b")


def dbg_map(src, a, b):
    u"""[a-1, b) 구간의 각 줄 → 그 줄이 속한 **논리 명령**의 `!dbg` 번호(없으면 None)."""
    lo, hi = a - 1, min(b, len(src))
    out = {}
    k = lo
    while k < hi:
        ln = src[k]
        if SW_OPEN.match(ln):
            j = k + 1
            while j < hi and not SW_CLOSE.match(src[j]):
                j += 1
            m = S.DBG.search(src[j]) if j < hi else None
            n = m.group(1) if m else None
            for t in range(k, min(j + 1, hi)):
                out[t] = n
            k = j + 1
            continue
        if INV_HEAD.match(ln) and S.DBG.search(ln) is None:
            j = k + 1
            while j < hi and not INV_CONT.match(src[j]):
                if src[j].strip() == "" or src[j].endswith(":"):
                    break
                j += 1
            m = S.DBG.search(src[j]) if (j < hi and INV_CONT.match(src[j])) else None
            n = m.group(1) if m else None
            for t in range(k, min(j + 1, hi)):
                out[t] = n
            k = j + 1
            continue
        m = S.DBG.search(ln)
        out[k] = m.group(1) if m else None
        k += 1
    return out


def check_spec(sp):
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    own = str(sp.get("src") or "").split("\\")[-1]
    src, meta = S.load(f)
    dm = dbg_map(src, a, b)
    out = []
    for j, c in enumerate(sp.get("consts") or []):
        val, claim = c.get("value"), c.get("src_line")
        if not isinstance(claim, int):
            continue
        # ★SSA 레지스터(`%2`)를 리터럴로 세지 않는다 — 7차 배치A 가 지적한 세 번째 결함.
        pat = re.compile(r"(?<![\w.\-%])" + re.escape(str(val)) + r"(?![\w.])")
        found = {}
        for k in range(a - 1, min(b, len(src))):
            ln = src[k]
            if "#dbg_" in ln or not pat.search(ln):
                continue
            n = dm.get(k)
            if not n:
                continue
            for (fn, li) in S.chain(meta, n):
                if fn == own:
                    found[li] = found.get(li, 0) + 1
        cands = sorted(x for x in found if x)
        if cands and claim not in found:
            out.append((j, claim, cands))
    return out


if __name__ == "__main__":
    import json
    D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
    lo = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    hi = int(sys.argv[2]) if len(sys.argv) > 2 else 20
    tot_old = tot_new = 0
    for i in range(lo, hi):
        sp = D["specs"][i]
        old = S.check_spec(sp)
        new = check_spec(sp)
        tot_old += len(old); tot_new += len(new)
        oldset = {r[0] for r in old}; newset = {r[0] for r in new}
        print(u"specs[%2d] %-46s 초판 %2d건 → 수정판 %2d건   (오탐으로 사라짐: %s / 남은 실오류: %s)"
              % (i, sp["name"], len(old), len(new),
                 sorted(oldset - newset), sorted(newset)))
        for (j, claim, cands) in new:
            print(u"          consts[%d] value=%r claim=L%s  실제후보=%s"
                  % (j, sp["consts"][j].get("value"), claim, cands[:10]))
    print(u"\n합계: 초판 %d건 → 수정판 %d건" % (tot_old, tot_new))
