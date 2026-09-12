# -*- coding: utf-8 -*-
u"""주어진 .ll 의 !dbg 번호들의 inlinedAt 사슬을 (파일, 줄)로 펼쳐 찍는다. (srclinecheck.chain 재사용)"""
import os, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import srclinecheck as S

f = sys.argv[1]
src, meta = S.load(f)
for n in sys.argv[2:]:
    n = n.lstrip("!")
    print(u"!%s -> %s" % (n, S.chain(meta, n)))
