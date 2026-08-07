# -*- coding: utf-8 -*-
"""cg.py - .text 전역 콜그래프(근사) + 도달성 질의. (numpy 선형스캔, e8/e9 rel32)
   ⚠근사: 데이터 바이트를 call 로 오인할 수 있으나 대상이 .pdata 함수시작인 것만 채택.
  python cg.py <ver> reach <target_rva,...> --roots <root_rva,...> [--depth N]
  python cg.py <ver> callers <rva> [--depth N]
"""
import sys, collections, bisect
import numpy as np
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner
import ls2

_G = {}

def build(ver):
    if ver in _G:
        return _G[ver]
    S = Scanner(ver)
    b = np.frombuffer(S.body, dtype=np.uint8)
    n = len(b) - 5
    starts = np.array(S.fstarts, dtype=np.int64)
    ends = np.array([f[1] for f in S.funcs], dtype=np.int64)
    fwd = collections.defaultdict(set)
    rev = collections.defaultdict(set)
    for opb in (0xe8, 0xe9):
        idx = np.nonzero(b[0:n] == opb)[0]
        d = (b[idx+1].astype(np.int64) | (b[idx+2].astype(np.int64) << 8)
             | (b[idx+3].astype(np.int64) << 16) | (b[idx+4].astype(np.int64) << 24))
        d = np.where(d >= 2**31, d - 2**32, d)
        tgt = S.tva + idx + 5 + d
        ok = np.isin(tgt, starts)
        src = S.tva + idx[ok]
        tg = tgt[ok]
        pos = np.searchsorted(starts, src, side='right') - 1
        good = (pos >= 0) & (starts[pos] <= src) & (src < ends[pos])
        for s, t in zip(starts[pos[good]], tg[good]):
            fwd[int(s)].add(int(t))
            rev[int(t)].add(int(s))
    _G[ver] = (S, fwd, rev)
    return _G[ver]


def srcname(ver, f):
    m = getattr(srcname, '_m', {}).get(ver)
    if m is None:
        m = {s: ls2.short_of(src) for s, e, src, l in ls2.rows(ver)}
        if not hasattr(srcname, '_m'):
            srcname._m = {}
        srcname._m[ver] = m
    return m.get(f, '?')


def reach(ver, roots, targets, depth=8):
    S, fwd, rev = build(ver)
    tset = set(targets)
    out = {}
    for r in roots:
        seen = {r}
        cur = {r}
        found = {}
        for d in range(depth):
            nxt = set()
            for f in cur:
                for t in fwd.get(f, ()):
                    if t in tset and t not in found:
                        found[t] = d + 1
                    if t not in seen:
                        seen.add(t)
                        nxt.add(t)
            cur = nxt
            if not cur:
                break
        out[r] = (found, len(seen))
    return out


if __name__ == '__main__':
    ver = sys.argv[1]
    if sys.argv[2] == 'callers':
        S, fwd, rev = build(ver)
        t = int(sys.argv[3], 16)
        dep = int(sys.argv[4]) if len(sys.argv) > 4 else 1
        cur = {t}; seen = {t}
        for d in range(dep):
            nx = set()
            for f in cur:
                for c in rev.get(f, ()):
                    if c not in seen:
                        seen.add(c); nx.add(c)
            print('--- depth %d: %d' % (d + 1, len(nx)))
            for c in sorted(nx):
                print('   %06x  %s' % (c, srcname(ver, c)[:95]))
            cur = nx
