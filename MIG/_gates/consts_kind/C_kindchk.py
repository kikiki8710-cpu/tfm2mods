# -*- coding: utf-8 -*-
u"""`consts[].kind`(상수 종류) 기계 대조 — **§4-b 무검사 축 186행의 첫 계측기.** (7차 배치C 신설)

## 발상
상수의 **종류는 쓰이는 문맥이 결정한다.** 임계(threshold)는 `<`/`>` 로 비교되는 것이고,
태그는 `switch`/`==` 로 비교되는 것이다. 그러니 IR 에서 그 리터럴이 **어떤 명령의 피연산자로
등장하는지**를 세면, 선언한 `kind` 와 대조할 수 있다.

관측 문맥(옵코드에서 기계적으로 뽑는다):
| 표기 | 무엇 |
|---|---|
| `EQ`  | `switch` 케이스 값 · `icmp eq/ne` |
| `ORD` | `icmp ult/ule/ugt/uge/slt/sle/sgt/sge` — **임계의 정의** |
| `ARI` | `add/sub/mul/udiv/sdiv/srem/urem/shl/lshr/ashr/and/or/xor/umin/umax/smin/smax` |
| `IDX` | `getelementptr` 인덱스 |
| `ST`  | `store <ty> K, ptr ..` — 그 값을 **써 넣는다** |
| `ARG` | `call`/`invoke` 의 인자 — **이 범위에서는 용도가 안 보인다**(콜리 안) |

## 기대 문맥
| kind | 필요 문맥(하나라도) |
|---|---|
| 임계 | `ORD` (없고 `ARG` 만 있으면 **판정보류** — 콜리가 비교한다) |
| 태그 | `EQ` 또는 `ST` |
| 인덱스 | `IDX` 또는 `ARI`(clamp/umin) |
| 센티널 | `EQ` 또는 `ST` |

⚠`switch` 케이스 값과 `invoke` 인자는 **줄에 `!dbg` 가 없어** G12 초판이 놓치던 자리다.
여기서는 줄 자체를 보므로 그 문제가 없다(대신 `!dbg` 를 안 쓴다).

## 한계
같은 리터럴이 한 함수에서 여러 역할을 겸하면 문맥이 섞인다(예 `0`). 그래서 이 도구는
**「필요 문맥이 하나도 없다」일 때만** 결함 후보로 올린다 — 거짓 양성을 구조적으로 낮춘다.
"""
import io, json, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import srclinecheck as S

ORDOPS = ("ult", "ule", "ugt", "uge", "slt", "sle", "sgt", "sge")
ARIOPS = ("add", "sub", "mul", "udiv", "sdiv", "srem", "urem", "shl", "lshr",
          "ashr", "and", "or", "xor", "umin", "umax", "smin", "smax")
SW_OPEN = re.compile(r"^\s*switch\b.*\[\s*$")
SW_CLOSE = re.compile(r"^\s*\]")

NEED = {u"임계": {"ORD"}, u"태그": {"EQ", "ST"}, u"인덱스": {"IDX", "ARI"},
        u"센티널": {"EQ", "ST"}}


def contexts(src, a, b, val):
    pat = re.compile(r"(?<![\w.\-])" + re.escape(str(val)) + r"(?![\w.])")
    got = {}
    lo, hi = a - 1, min(b, len(src))
    insw = False
    for k in range(lo, hi):
        ln = src[k]
        if SW_OPEN.match(ln):
            insw = True
            continue
        if insw and SW_CLOSE.match(ln):
            insw = False
            continue
        if "#dbg_" in ln or not pat.search(ln):
            continue
        tags = set()
        if insw:
            tags.add("EQ")
        else:
            m = re.search(r"icmp (?:samesign )?(\w+)", ln)
            if m:
                tags.add("ORD" if m.group(1) in ORDOPS else
                         ("EQ" if m.group(1) in ("eq", "ne") else "?"))
            if re.search(r"= (?:tail )?(?:nsw |nuw |exact )*(" + "|".join(ARIOPS) + r")\b", ln) \
               or re.search(r"llvm\.(u|s)(min|max)\.", ln):
                tags.add("ARI")
            if "getelementptr" in ln:
                tags.add("IDX")
            if re.match(r"^\s*store ", ln):
                tags.add("ST")
            if re.search(r"\b(?:call|invoke)\b", ln):
                tags.add("ARG")
        for t in tags:
            got.setdefault(t, []).append(k + 1)
    return got


def check_spec(sp):
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    src, _m = S.load(f)
    out = []
    for j, c in enumerate(sp.get("consts") or []):
        kind, val = (c.get("kind") or "").strip(), c.get("value")
        need = NEED.get(kind)
        if need is None or val is None:
            continue
        got = contexts(src, a, b, val)
        if not got:
            out.append((j, kind, val, got, u"판정보류(리터럴 미검출 — 접힘)"))
        elif need & set(got):
            continue
        elif set(got) <= {"ARG", "?"}:
            out.append((j, kind, val, got, u"판정보류(콜리 인자로만 등장)"))
        else:
            out.append((j, kind, val, got, u"**불일치**"))
    return out


if __name__ == "__main__":
    D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
    lo = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    hi = int(sys.argv[2]) if len(sys.argv) > 2 else 20
    bad = hold = n = 0
    for i in range(lo, hi):
        sp = D["specs"][i]
        rows = check_spec(sp)
        n += len(sp.get("consts") or [])
        b2 = [r for r in rows if r[4].startswith("**")]
        bad += len(b2); hold += len(rows) - len(b2)
        print(u"specs[%2d] %-46s consts %2d행 · 불일치 %d · 판정보류 %d"
              % (i, sp["name"], len(sp.get("consts") or []), len(b2), len(rows) - len(b2)))
        for (j, kind, val, got, why) in rows:
            print(u"    consts[%2d] value=%-14r kind=%-4s 관측문맥=%-28s %s"
                  % (j, val, kind, sorted(got), why))
    print(u"\n합계: consts %d행 · **불일치 %d** · 판정보류 %d" % (n, bad, hold))
