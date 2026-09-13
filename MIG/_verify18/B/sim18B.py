# -*- coding: utf-8 -*-
u"""patch.json 을 v3 사본에 적용해 담당 게이트를 재실행(정본 미수정)."""
import sys, io, json, re, copy
sys.path.insert(0, r"C:\tfm2mods\MIG")
V3 = r"C:\tfm2mods\MIG\_spec\specs20_v3.json"
D = json.load(io.open(V3, encoding="utf-8"))
P = json.load(io.open(r"C:\tfm2mods\MIG\_verify18\B\patch.json", encoding="utf-8"))
PATH = re.compile(r"^/specs\[(\d+)\](?:/(\w+))?/(\w+)(?:\[(\d+)\])?(?:/(\w+))?$")
V2KEY = {"note": "role", "meaning": "meaning"}

def apply(e):
    m = PATH.match(e["path"]); i, outer, field, idx, key = m.groups(); i = int(i)
    sp = D["specs"][i]
    cont = sp[outer] if outer else sp
    if e.get("op") == "insert":
        arr = cont[field]
        row = dict(e["new"])
        if field == "params" and "note" in row:
            row["role"] = row.pop("note")
        if field == "mem" and "note" in row:
            row["chk"] = "sim"; row["ev"] = 4
        arr.insert(int(e["at"]), row); return
    old, new = e["old"], e["new"]
    if idx is None:
        cur = cont[field]
        if isinstance(cur, str):
            assert old in cur, e["path"]; cont[field] = cur.replace(old, new)
        else:
            raise Exception(e["path"])
        return
    row = cont[field][int(idx)]
    if field in ("open", "notes"):
        assert old in row["q"], e["path"]; row["q"] = row["q"].replace(old, new); return
    k = key
    if k not in row and V2KEY.get(k) in row: k = V2KEY[k]
    cur = row[k]
    if isinstance(cur, str):
        assert old in cur, e["path"]; row[k] = cur.replace(old, new)
    else:
        assert cur == old, (e["path"], cur, old); row[k] = new

# 삽입은 뒤 인덱스를 미니 — 같은 배열의 인덱스 경로가 없도록 patch 를 짰다(62 mem: 삽입만 · 66 params: 삽입만)
for e in P["errors"]:
    apply(e)

# open→notes 재분류 (mkspec3.classify)
import mkspec3
for i in (62, 63, 64, 65, 66):
    sp = D["specs"][i]
    keep = []
    for x in sp["open"]:
        k = mkspec3.classify(x["q"])
        x["class"] = k
        if k == u"사실 서술":
            sp["notes"].append(x)
        else:
            keep.append(x)
    sp["open"] = keep
    print(i, sp["name"], "open", len(sp["open"]), "notes", len(sp["notes"]),
          [x["class"] for x in sp["open"]])

print("\n== G16 paramrole")
import paramrole
for i in (62, 63, 64, 65, 66):
    r = paramrole.check_spec(D["specs"][i])
    print(i, r if r else "OK")
    if i in (65, 66):
        m, a, f = paramrole.align(D["specs"][i]); print("   align", m)

print("\n== G12 srclinecheck")
import srclinecheck
for i in (62, 63, 64, 65, 66):
    print(i, srclinecheck.check_spec(D["specs"][i]) or "OK")

print("\n== G15 kind 파생(63 consts[5] · 62 c6 · 64 c4)")
import kindchk
for i, j in ((63, 5), (62, 6), (64, 4), (62, 7), (64, 5)):
    x = D["specs"][i]["consts"][j]
    k = mkspec3._kind(D["specs"][i], x, x["meaning"])
    print(i, j, x["value"], "->", k, "| neg:", kindchk.neg_hit(x["meaning"], k))

print("\n== G20 sharedchk R4")
import sharedchk
err, soft = sharedchk.check(D["specs"])
for r in err:
    if r[0] == "R4" and any(int(x[0]) in (62, 63, 64, 65, 66) for x in r[2]):
        print(r)
print("R4 done")

print("\n== specgate G7/G10/G18 (담당 함수)")
import specgate
specgate.D = D
specgate.IS_V3 = True
out = []
def flag(g, i, msg, det=u""):
    out.append((g, i, msg, det[:100]))
specgate.flag = flag
for i in (62, 63, 64, 65, 66):
    for name in ("gate7", "gate10", "gate18"):
        fn = getattr(specgate, name, None)
        if fn:
            try:
                fn(i, D["specs"][i])
            except Exception as ex:
                print(name, i, "EXC", ex)
for o in out:
    print(o)
print("specgate done", len(out))
