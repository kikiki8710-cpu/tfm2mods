# -*- coding: utf-8 -*-
u"""probe12 — G9 손확인 구제 규칙 두 안의 건전성 비교.

안① (7차 배치B 제안) 동명 함수의 **Self 타입**이 필드 소유 구조체와 안 겹치면 후보 제외.
안② (이 라운드 제안) **이 명세의 `mem` 표가 그 필드를 가진 구조체를 base 로 싣고 있으면** 후보 제외.

건전성 시험 = 「G9 가 원래 잡아야 하는 것(4차 D-E2 의 Entity 4메서드)을 안 지우는가」.
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
sys.path.insert(0, MIG)
import spec3lib as L

D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
DS = json.load(io.open(os.path.join(MIG, "distruct.json"), encoding="utf-8"))
OWN = {}
for s, r in DS.items():
    for f in (r or {}).get("fields") or []:
        OWN.setdefault(f.get("name"), set()).add(s.split("<")[0].split("::")[-1])


def selfs(n):
    return set(x["path"].split("::")[-2] for x in L.fnlookup(n) if len(x["path"].split("::")) >= 2)


def membases(sp):
    out = set()
    for x in sp.get("mem") or []:
        b = (x.get("base") or u"").split("(")[0].strip()
        if b:
            out.add(b.split("::")[-1])
    # self 타입도 포함(`self.line` 꼴)
    p = (sp.get("sig") or {}).get("path") or u""
    if "::" in p:
        out.add(p.split("::")[-2])
    return out


CASES = [(7, "target_bush"), (13, "line"), (13, "nearest_enemy"), (13, "position"), (13, "team"),
         (14, "line"), (14, "nearest_enemy"), (14, "position"), (14, "team"), (15, "chats")]
REG = [(16, "attack_effect"), (16, "skill_effect"), (16, "skill2_cooldown"), (16, "ult_cooldown")]

for label, rows in ((u"현재 G9 손확인 10칸", CASES), (u"★회귀(원래 잡아야 하는 것)", REG)):
    print(u"\n──── %s ────" % label)
    for i, n in rows:
        o, s_ = OWN.get(n, set()), selfs(n)
        mb = membases(S[i])
        a1 = not (o & s_)                      # 안① 제외?
        a2 = bool(o & mb)                      # 안② 제외?
        print(u"  [%02d] %-16s 안①=%-4s 안②=%-4s  (mem base 교집합=%s)"
              % (i, n, u"제외" if a1 else u"유지", u"제외" if a2 else u"유지", sorted(o & mb)[:4]))
