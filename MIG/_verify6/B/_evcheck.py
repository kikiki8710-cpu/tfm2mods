# -*- coding: utf-8 -*-
u"""`patch.json` 의 ev_up 이 실제로 등급을 내리는지 **파일을 쓰지 않고** 모의 검증한다.
applypatch 가 붙일 태그 + mkspec3 의 evtier/ev_mem 를 그대로 써서 파생값을 계산한다."""
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
sys.path.insert(0, MIG)
import mkspec3 as M                                   # evtier / ev_mem

V2 = json.load(io.open(os.path.join(MIG, "_spec", "specs20.json"), encoding="utf-8"))
PJ = json.load(io.open(os.path.join(MIG, "_verify6", "B", "patch.json"), encoding="utf-8"))
EVMARK = {1: u"런타임 실측 DIFF=0", 2: u"오라클 실행 확증", 3: u"tcx 정본 대조"}


def resolve(spec, field, idx):
    if field == "mem":
        r, w = spec.get("reads") or [], spec.get("writes") or []
        return (r, idx) if idx < len(r) else (w, idx - len(r))
    if field == "knobs":
        k, n = spec.get("knobs") or [], spec.get("new_knobs") or []
        return (k, idx) if idx < len(k) else (n, idx - len(k))
    if field == "consts":
        return spec.get("constants") or [], idx
    return spec.get(field), idx


bad = 0
for u in PJ["ev_up"]:
    i = int(u["path"].split("[")[1].split("]")[0])
    field = u["path"].split("/")[2].split("[")[0]
    j = int(u["path"].split(field + "[")[1].split("]")[0])
    arr, jj = resolve(V2["specs"][i], field, j)
    row = dict(arr[jj])
    want = 3 if (field == "mem" and u["to"] < 3) else u["to"]
    tag = u" · %s(6차 배치B: %s)" % (EVMARK[want], u["evidence"])
    for k in ("meaning", "effect", "note", "value", "what", "name"):
        if isinstance(row.get(k), str):
            row[k] += tag
            break
    else:
        print(u"[설명란 없음] %s" % u["path"]); bad += 1; continue
    blob = u" ".join(v for v in row.values() if isinstance(v, str))
    got = M.ev_mem(blob) if field == "mem" else M.evtier(blob)
    if got != want:
        print(u"[파생 불일치] %s  want=%d got=%d" % (u["path"], want, got)); bad += 1
print(u"\nev_up %d건 · 파생 불일치 %d건" % (len(PJ["ev_up"]), bad))
