# -*- coding: utf-8 -*-
u"""패치 적용 **후**의 게이트 결과를 시뮬레이션한다(정본은 안 건드린다)."""
import io, json, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.path.insert(0, r"C:\tfm2mods\MIG\_verify8\B")
import gate as G

_T = re.compile(u"\\s*·\\s*(?:런타임 실측 DIFF=0|오라클 실행 확증|tcx 정본 대조|실행)\\s*\\(")


def derive(m):
    k = _T.split(m or u"")[0]
    return (u"태그" if any(w in k for w in (u"태그", u"판별자", u"variant")) else
            u"센티널" if any(w in k for w in (u"센티널", u"니치", u"0xff", u"MAX")) else
            u"인덱스" if u"인덱스" in k else u"임계")


D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
P = json.load(io.open(r"C:\tfm2mods\MIG\_verify8\B\patch.json", encoding="utf-8"))
for e in P["errors"]:
    i, j = map(int, re.findall(r"\[(\d+)\]", e["path"]))
    c = D["specs"][i]["consts"][j]
    c["meaning"] = e["new"]
    c["kind"] = derive(e["new"])

from collections import Counter
tally, kinds = Counter(), Counter()
for i in range(20):
    for r in G.check_spec(D["specs"][i]):
        tally[r[1]] += 1
    for c in D["specs"][i]["consts"]:
        kinds[c["kind"]] += 1
print(u"패치 후 kind 분포:", dict(kinds))
print(u"패치 후 게이트:", dict(tally))
