#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""IR 함수 본문 추출기 — 파일 통째 1패스(sed 루프 금지).
사용:
  python irfn2.py find <정규식>            # _gaibc 전 .ll 에서 define 줄 검색
  python irfn2.py body <파일> <정규식>     # 첫 매치 define 부터 닫는 '}' 까지 출력
  python irfn2.py blocks <파일> <정규식> <라벨,라벨,...>   # 특정 블록만
"""
import io, os, re, sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

DIRS = [r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc', r'C:\tfm2mods\_gvbc']

def resolve(p):
    if os.path.isabs(p):
        return p
    for d in DIRS:
        q = os.path.join(d, p)
        if os.path.exists(q):
            return q
    return p

def files():
    for d in DIRS:
        if not os.path.isdir(d):
            continue
        for fn in sorted(os.listdir(d)):
            if fn.endswith('.ll'):
                yield os.path.join(d, fn)

cmd = sys.argv[1]
if cmd == 'find':
    rx = re.compile(sys.argv[2])
    for f in files():
        with io.open(f, 'r', encoding='utf-8', errors='replace') as fh:
            for i, ln in enumerate(fh, 1):
                if ln.startswith('define') and rx.search(ln):
                    print('%s:%d: %s' % (os.path.basename(f), i, ln[:300].rstrip()))
elif cmd in ('body', 'blocks'):
    path = resolve(sys.argv[2])
    rx = re.compile(sys.argv[3])
    want = set(sys.argv[4].split(',')) if cmd == 'blocks' else None
    out = []
    inside = False
    with io.open(path, 'r', encoding='utf-8', errors='replace') as fh:
        for i, ln in enumerate(fh, 1):
            if not inside:
                if ln.startswith('define') and rx.search(ln):
                    inside = True
                    out.append((i, ln.rstrip()))
                continue
            out.append((i, ln.rstrip()))
            if ln.rstrip() == '}':
                break
    if want is None:
        for i, ln in out:
            print('%d| %s' % (i, ln))
    else:
        cur = None
        for i, ln in out:
            m = re.match(r'^(\S+):\s*(;.*)?$', ln)
            if m:
                cur = m.group(1)
            if cur in want or ln.startswith('define'):
                print('%d| %s' % (i, ln))
