# -*- coding: utf-8 -*-
"""함수 범위 안에서 정수 리터럴 V 가 나오는 명령의 !dbg 루트 줄(명세 src 파일 기준) 집계. 사용: python constroots.py <ll> <from> <to> <srcfile-leaf> V [V...]"""
import re, sys
sys.path.insert(0, __import__('os').path.dirname(__file__))
from irlib import LL
ll = LL(sys.argv[1]); a = int(sys.argv[2]); b = int(sys.argv[3]); leaf = sys.argv[4]
vals = sys.argv[5:]
for v in vals:
    rx = re.compile(r'(?<![\w.%!@-])' + re.escape(v) + r'(?![\d.])')
    hits = {}
    for n in range(a, b + 1):
        ln = ll.line(n)
        if ln.lstrip().startswith(('#dbg', '!', ';')): continue
        # strip metadata tail & align/gep
        body = re.sub(r', !dbg.*$', '', ln)
        body = re.sub(r'align \d+', '', body)
        body = re.sub(r'!\w+ !\d+', '', body)
        if 'getelementptr' in body: continue
        if not rx.search(body): continue
        d = ll.dbg_of(n)
        c = ll.chain(d) if d else []
        rootl = None; rootfn = None
        for x in c:
            if x[2] and x[2].replace(chr(92), '/').endswith(leaf):
                rootl = x[1]; rootfn = x[3]
        key = (rootl if rootl else ('~' + (c[-1][2] or '?').split(chr(92))[-1] + ':' + str(c[-1][1]) if c else 'nodbg'))
        hits.setdefault(key, []).append((n, body.strip()[:70]))
    print(f"== {v}")
    for k in sorted(hits, key=lambda k: (isinstance(k, str), k)):
        print(f"   root={k!s:20s} n={len(hits[k])}  e.g. {hits[k][0][0]}: {hits[k][0][1]}")
