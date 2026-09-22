# -*- coding: utf-8 -*-
"""m02.ll 27592~36306 (action_candidates) 통째 로드 + 주석본 사슬 사전. import 해서 쓴다."""
import io, re, sys, os
sys.stdout.reconfigure(encoding='utf-8')
M02 = r'C:\tfm2mods\_gaibc\m02.ll'
ANN = r'C:\tfm2mods\MIG\_next\reach\cbbdb0.ll'
LO, HI = 27592, 36306
_cache = {}
def lines():
    if 'l' not in _cache:
        with io.open(M02, encoding='utf-8', errors='replace') as f:
            allx = f.read().split('\n')
        _cache['l'] = allx
    return _cache['l']
def L(n):  # 1-based
    return lines()[n-1]
def rng(a, b):
    return [(i, L(i)) for i in range(a, b+1)]
def ann():
    """원문줄 -> ;L 사슬"""
    if 'a' not in _cache:
        d = {}
        with io.open(ANN, encoding='utf-8', errors='replace') as f:
            for ln in f:
                m = re.match(r'\s*(\d+)\|(.*)', ln)
                if not m: continue
                n = int(m.group(1)); body = m.group(2)
                cm = re.search(r';L(\S+)\s*$', body)
                d[n] = cm.group(1) if cm else ''
        _cache['a'] = d
    return _cache['a']
def chain(n):
    return ann().get(n, '')
def dbgline(n):
    """원문 줄의 !dbg !N 을 DILocation line 으로 푼다(느림: 메타 사전 1회 구축)"""
    md = meta()
    m = re.search(r'!dbg !(\d+)', L(n))
    if not m: return None
    return diloc(int(m.group(1)))
def meta():
    if 'm' not in _cache:
        d = {}
        for ln in lines():
            if ln.startswith('!') and ' = ' in ln:
                k, v = ln.split(' = ', 1)
                d[k[1:]] = v
        _cache['m'] = d
    return _cache['m']
def diloc(i):
    v = meta().get(str(i), '')
    m = re.search(r'line: (\d+)', v)
    ia = re.search(r'inlinedAt: !(\d+)', v)
    sc = re.search(r'scope: !(\d+)', v)
    return (int(m.group(1)) if m else None, int(ia.group(1)) if ia else None, int(sc.group(1)) if sc else None)
def dichain(i):
    out = []
    while i is not None:
        ln, ia, sc = diloc(i)
        out.append(ln); i = ia
    return out
