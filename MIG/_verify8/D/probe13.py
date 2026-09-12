# -*- coding: utf-8 -*-
u"""probe13 — 안③: `gate9` 의 `memn`(mem 이름에서 필드명 수확) 이 **주석 꼬리 때문에 깨진다**.

현행: `memn.add(part) for part in name.split('.')`
      → `ty.Tower.nearest_enemy 태그(Option<(usize,usize)>, range 0..2)` 는
        마지막 조각이 `nearest_enemy 태그(...)` 라 `nearest_enemy` 가 **안 들어간다.**
      ⟹ 명세가 「이건 필드다」를 자기 표에 이미 써 놨는데도 G9 가 손확인으로 올린다.
안③ = 이름을 **식별자 단위로 토큰화**한다(순수 버그 수정, 추론 없음).
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")

for i, names in ((13, ("line", "nearest_enemy", "position", "team")),
                 (14, ("line", "nearest_enemy", "position", "team")),
                 (7, ("target_bush",)), (15, ("chats",)),
                 (16, ("attack_effect", "skill_effect", "skill2_cooldown", "ult_cooldown"))):
    cur, new = set(), set()
    for x in S[i].get("mem") or []:
        nm = x.get("name") or u""
        for p in nm.split("."):
            cur.add(p)
        for p in IDENT.findall(nm):
            new.add(p)
    print(u"[%02d] %s" % (i, S[i]["name"]))
    for n in names:
        print(u"     %-16s 현행memn=%-5s  안③memn=%-5s  %s"
              % (n, n in cur, n in new,
                 u"→ 구제됨" if (n not in cur and n in new) else u""))
