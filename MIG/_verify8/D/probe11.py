# -*- coding: utf-8 -*-
u"""probe11 — G9 '손확인 4건' 을 7차 배치B 제안(다른 Self 타입이면 후보에서 빼라)으로 판정한다.

판정 근거
  G9 손확인은 「logic 이 `.name` 으로 적었는데 tcx 에 동명 함수가 있다」로만 뽑는다.
  Rust 는 필드 `x` 와 메서드 `x()` 가 공존하므로 이것만으로는 못 가른다.
  ⟹ 동명 함수의 **Self 타입**(path 의 마지막 바로 앞 세그먼트)이,
     그 이름을 필드로 가진 구조체(distruct)와 **하나도 겹치지 않으면**
     logic 이 말한 `a.b.name` 의 name 은 그 함수가 아니다 ⟹ 후보에서 뺀다.
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
sys.path.insert(0, MIG)
import spec3lib as L

DS = json.load(io.open(os.path.join(MIG, "distruct.json"), encoding="utf-8"))
owners = {}
for sname, rec in DS.items():
    for f in (rec or {}).get("fields") or []:
        owners.setdefault(f.get("name"), set()).add(sname.split("<")[0].split("::")[-1])

for name in ("target_bush", "line", "nearest_enemy", "position", "team", "chats"):
    fns = L.fnlookup(name)
    selfs = set()
    for x in fns:
        segs = x["path"].split("::")
        if len(segs) >= 2:
            selfs.add(segs[-2])
    own = owners.get(name, set())
    print(u"\n== %s ==" % name)
    print(u"  필드 소유 구조체 %d개: %s" % (len(own), sorted(own)[:10]))
    print(u"  동명 함수 Self %d개: %s" % (len(selfs), sorted(selfs)[:10]))
    print(u"  교집합: %s  ⟹ %s" % (sorted(own & selfs)[:8],
                                  u"후보 유지" if (own & selfs) else u"★후보에서 제외(다른 Self 타입)"))
