# -*- coding: utf-8 -*-
u"""담당 5함수의 mem 행 전수를 tcxdict 정본으로 독립 재조회한다. (12차 배치C)
`chk` 컬럼(mkspec3 가 tcxaudit 로 박은 것)을 믿지 않고 `walk(want=)` 를 직접 두드린다."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
sys.path.insert(0, MIG)
import tcxdict as TD

D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

# base 문자열 → tcx 타입명 정규화
def norm_base(b):
    b = re.sub(u"[（(].*?[)）]", u" ", b or u"")
    b = re.sub(u"::vtable$", u"", b.strip())
    return b.strip()


def off_of(o):
    o = str(o or "").strip().lower()
    try:
        return int(o, 16) if o.startswith("0x") else int(o)
    except Exception:
        return None


rows_out = []
for i in (10, 11, 12, 13, 14):
    s = D["specs"][i]
    for j, r in enumerate(s.get("mem") or []):
        b, o = norm_base(r.get("base")), off_of(r.get("offset"))
        if o is None:
            rows_out.append((i, j, b, r.get("offset"), r.get("name"), u"오프셋파싱실패", u""))
            continue
        try:
            cands = TD.walk(b, want=o)
        except Exception as e:
            rows_out.append((i, j, b, "0x%x" % o, r.get("name"), u"조회실패:%s" % type(e).__name__, u""))
            continue
        exact = [c for c in cands if c[0] == o]
        got = u" | ".join(u"%s : %s" % (c[1], c[2]) for c in exact[:8]) or u"(해당 오프셋 leaf 없음)"
        rows_out.append((i, j, b, "0x%x" % o, r.get("name"), u"OK" if exact else u"NOFIELD", got))

for t in rows_out:
    print(u"[%02d] mem[%2d] %-28s %-8s | 명세=%-46s | %s | tcx=%s"
          % (t[0], t[1], t[2], t[3], (t[4] or u"")[:46], t[5], t[6][:170]))
