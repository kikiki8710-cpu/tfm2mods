# -*- coding: utf-8 -*-
u"""제안한 `meaning` 수정이 `mkspec3` 의 `kind` 파생을 실제로 뒤집는지 **시뮬레이션**한다.
(정본은 건드리지 않는다 — 읽기만 한다)"""
import io, json, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")

_EVTAIL = re.compile(u"\\s*·\\s*(?:런타임 실측 DIFF=0|오라클 실행 확증|tcx 정본 대조|실행)\\s*\\(")


def derive(m):
    kmat = _EVTAIL.split(m or u"")[0]
    return (u"태그" if any(k in kmat for k in (u"태그", u"판별자", u"variant")) else
            u"센티널" if any(k in kmat for k in (u"센티널", u"니치", u"0xff", u"MAX")) else
            u"인덱스" if u"인덱스" in kmat else u"임계")


V2 = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20.json", encoding="utf-8"))
V3 = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
PROP = json.load(io.open(r"C:\tfm2mods\MIG\_verify8\B\_prop.json", encoding="utf-8"))

ok = bad = 0
for p in PROP:
    i, j = p["i"], p["j"]
    cur = (V2["specs"][i].get("constants") or [])[j]
    old, new, want = cur.get("meaning") or u"", p["new"], p["want"]
    now3 = V3["specs"][i]["consts"][j].get("kind")
    if old != p.get("old", old):
        print(u"!! [%02d]c%-2d old 불일치" % (i, j)); bad += 1; continue
    d0, d1 = derive(old), derive(new)
    flag = u"OK" if d1 == want else u"**실패**"
    if d1 == want and d0 == now3:
        ok += 1
    else:
        bad += 1
    print(u"[%02d]c%-2d val=%-10s 정본kind=%-4s 재현=%-4s → 수정후=%-5s (목표 %s) %s"
          % (i, j, cur.get("value"), now3, d0, d1, want, flag))
print(u"\n성공 %d / 실패 %d" % (ok, bad))
