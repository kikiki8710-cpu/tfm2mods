# -*- coding: utf-8 -*-
u"""★G12 제안 수정판 — `srclinecheck.py` 의 **오탐 3종**을 막는다(7차 배치D).

원본은 「리터럴과 `!dbg` 가 **같은 텍스트 줄**에 있어야」 후보로 센다. LLVM 출력에는 그렇지 않은
정상 형태가 세 가지 있고, 배치 D 의 G12 10건 중 **8건이 전부 그 형태였다**(실오류는 2건뿐).

  ① `switch` case 줄에 `!dbg` 가 없다 — `!dbg` 는 switch **헤더**(와 닫는 `]`)에 붙는다
        switch i64 %125, label %126 [
          i64 3, label %127        ← 여기엔 !dbg 가 없다
        ], !dbg !38930             ← 여기 있다 (= modes.rs:256)
     ⟹ 15 consts[5][6][7](3/4/7@256) · 16 consts[6](3@2402) · 18 consts[0][1](1/2@635) **6건이 오탐**
     수정: switch 블록 안의 case 줄은 **헤더/푸터의 `!dbg` 를 상속**시킨다.

  ② `phi` 로만 남은 초기화 상수는 `!dbg line: 0` 이다
        %12 = phi i64 [ %80, %74 ], [ 0, %3 ], ... !dbg(line 0)   ← `let mut range = 0`(battle.rs:2398)
     ⟹ 16 consts[5](0@2398) 오탐. 원본은 줄 0 을 후보에서 빼면서(옳다) **판정보류로도 안 돌린다**.
     수정: 리터럴의 사슬이 **줄 0 밖에 없으면 판정보류**(불일치 아님).

  ③ 병합된 위치(merged location)도 `line: 0` 이다
        %155 = select i1 %149, i8 21, i8 22   ← 674/676 두 push 가 합쳐져 !dbg 루트가 epic.rs:0
     ⟹ 18 consts[5][6](21/22@674/676) 오탐. 같은 ② 규칙으로 걸러진다.

  ★그리고 **후보 목록 자체의 오염**: 리터럴 정규식이 `%3`·`%21`·`align 4`·`dereferenceable(816)`
    까지 긁는다. 18 consts[5]/[6] 의 「실제 후보 = [638]」은 전부 `%21`/`%22` 였다.
    수정: LLVM **타입 접두**(`i8 3`, `i64 -1`, `ptr null`)가 붙은 것만 진짜 리터럴로 센다.

용법: python -X utf8 srclinecheck2.py        (전 20함수 · 원본과 건수 비교)
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
sys.path.insert(0, MIG)
import srclinecheck as SLC

DBG = SLC.DBG


def check_spec(sp):
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    own = str(sp.get("src") or "").split("\\")[-1]
    try:
        src, meta = SLC.load(f)
    except Exception:
        return []
    # ── ①switch 상속: case 줄 → 그 switch 의 !dbg
    inherit = {}
    k = a - 1
    lim = min(b, len(src))
    while k < lim:
        st = src[k].strip()
        if st.startswith("switch "):
            body, j = [], k
            while j < lim and "]" not in src[j]:
                body.append(j); j += 1
            if j < lim:
                body.append(j)
                m = DBG.search(src[j]) or DBG.search(src[k])
                if m:
                    for x in body:
                        inherit[x] = m.group(1)
            k = j + 1
            continue
        k += 1

    out = []
    for jdx, c in enumerate(sp.get("consts") or []):
        val, claim = c.get("value"), c.get("src_line")
        if not isinstance(claim, int):
            continue
        v = re.escape(str(val))
        # ★타입 접두가 붙은 진짜 상수 피연산자만
        real = re.compile(r"(?:\b(?:i1|i8|i16|i32|i64|i128|float|double)\s+|,\s*)" + v + r"(?![\w.])")
        found, zero_only = {}, True
        for x in range(a - 1, lim):
            ln = src[x]
            if "#dbg_" in ln or not real.search(ln):
                continue
            m = DBG.search(ln)
            nid = m.group(1) if m else inherit.get(x)
            if not nid:
                continue
            for (fn, li) in SLC.chain(meta, nid):
                if fn == own:
                    found[li] = found.get(li, 0) + 1
                    if li:
                        zero_only = False
        cands = sorted(x for x in found if x)
        if not cands:
            continue                     # 리터럴 접힘 / 줄0 뿐 → 판정보류
        if claim not in found:
            out.append((jdx, claim, cands))
    return out


if __name__ == "__main__":
    D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
    o = n = 0
    for i in range(20):
        sp = D["specs"][i]
        a = SLC.check_spec(sp)
        b = check_spec(sp)
        o += len(a); n += len(b)
        if a or b:
            print(u"specs[%-2d] %-46s 원본 %d → 수정판 %d   %s"
                  % (i, sp["name"][:46], len(a), len(b),
                     u"남은 것 = " + str([(x[0], x[1], x[2][:4]) for x in b]) if b else u""))
    print(u"\n---- G12 총계: 원본 %d건 → 수정판 **%d건** (오탐 %d건 제거)" % (o, n, o - n))
