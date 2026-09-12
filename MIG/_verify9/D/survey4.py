# -*- coding: utf-8 -*-
u"""임무① — **한 함수 안에서** 같은 구조체가 여러 철자로 적혔는가.

`G14(memdir)` 의 `base_anchors_for` 는 `m2["base"] != base` **문자열 동일**로 형제 행을 모은다.
철자가 갈리면 형제가 안 모여 **동정 실패 → 보류**가 되고 게이트가 그만큼 무력해진다.
"""
import io, json, os, re, sys, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))


def head(b):
    u"""철자에서 **타입 머리**만 뽑는다(괄호주석·dyn·vtable·경로 꼬리 제거)."""
    s = re.sub(r"\s*\(.*?\)\s*$", u"", (b or u"").strip())
    s = re.sub(r"^dyn\s+", u"", s)
    s = re.sub(r"\s*(?:::)?vtable$", u"", s)
    s = re.sub(r"^bumpalo\s+", u"", s)
    s = re.sub(r"\(bumpalo[^)]*\)", u"", s)
    return s.strip()


tot = 0
for i, sp in enumerate(D["specs"]):
    g = collections.defaultdict(list)
    for j, m in enumerate(sp.get("mem") or []):
        g[head(m.get("base"))].append((j, m.get("base"), m.get("offset"), m.get("dir")))
    for h, rows in sorted(g.items()):
        sp_set = {r[1] for r in rows}
        if len(sp_set) > 1:
            tot += len(rows)
            print(u"\n specs[%-2d] %s  머리=%r  철자 %d종 / %d행"
                  % (i, sp["name"], h, len(sp_set), len(rows)))
            for (j, b, o, d) in rows:
                print(u"      mem[%-2d] base=%-40r off=%-8s dir=%s" % (j, b, o, d))
print(u"\n==== 철자 갈린 행 총 %d" % tot)
