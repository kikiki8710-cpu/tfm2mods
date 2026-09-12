# -*- coding: utf-8 -*-
u"""메타데이터 id 를 소스 사슬로 푼다. 사용: python -X utf8 res.py <ll파일> <id> [<id>...]"""
import os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, MIG)
import srclinecheck as S

f = sys.argv[1]
src, meta = S.load(f)
for x in sys.argv[2:]:
    print(u"!%s -> %s   raw=%s" % (x, S.chain(meta, x), meta.get(x, "?")[:160]))
