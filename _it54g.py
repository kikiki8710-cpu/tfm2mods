# -*- coding: utf-8 -*-
"""0.5.3<->0.5.4 함수 대응: 문자열-xref 앵커 + 콜그래프 양방향 투표."""
import sys, os, re, struct, bisect, pickle
import numpy as np
sys.path.insert(0, r'C:\tfm2mods')
from _it54 import O, N, BASE

CACHE = r'C:\tfm2mods\_g54_cache.pkl'

def sec(E, nm):
    return next(x for x in E.secs if x[0] == nm)

def build(E, tag):
    tn, tva, tvs, tpr, tsr = sec(E, '.text')
    blob = E.data[tpr:tpr+tsr]
    arr = np.frombuffer(blob, dtype=np.uint8)
    funcs = sorted(set(s for s, e in E.pdata()))
    fstart = np.array(funcs, dtype=np.int64)
    fset = set(funcs)
    def fn_of(rva):
        i = np.searchsorted(fstart, rva, 'right') - 1
        if i < 0: return None
        return int(fstart[i])
    # ---- call graph (e8 rel32) ----
    idx = np.where(arr[:-5] == 0xe8)[0]
    rel = (arr[idx+1].astype(np.int64) | (arr[idx+2].astype(np.int64) << 8)
           | (arr[idx+3].astype(np.int64) << 16)
           | (arr[idx+4].astype(np.int8).astype(np.int64) << 24))
    tgt = tva + idx + 5 + rel
    site = tva + idx
    ok = np.array([int(t) in fset for t in tgt])
    site = site[ok]; tgt = tgt[ok]
    callers = {}   # callee -> list of (caller_fn, site)
    callees = {}   # caller_fn -> list of callee
    for s, t in zip(site.tolist(), tgt.tolist()):
        c = fn_of(s)
        if c is None: continue
        callers.setdefault(t, []).append((c, s))
        callees.setdefault(c, []).append(t)
    # ---- string refs (lea rip-rel into .rdata) ----
    rn, rva_, rvs, rpr, rsr = sec(E, '.rdata')
    rblob = E.data[rpr:rpr+rsr]
    rx = re.compile(rb'[\x48\x4c\x49\x4d]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', re.DOTALL)
    fn_str = {}
    for m in rx.finditer(blob):
        i = m.start()
        if i + 7 > len(blob): continue
        disp = struct.unpack_from('<i', blob, i + 3)[0]
        s_rva = tva + i
        t_rva = s_rva + 7 + disp
        if not (rva_ <= t_rva < rva_ + rsr): continue
        off = rpr + (t_rva - rva_)
        # printable run
        j = off
        lim = min(off + 200, rpr + rsr)
        while j < lim and 0x20 <= E.data[j] < 0x7f:
            j += 1
        L = j - off
        if L < 8: continue
        sv = E.data[off:j].decode('latin1')
        f = fn_of(s_rva)
        if f is None: continue
        fn_str.setdefault(f, set()).add(sv)
    return dict(funcs=funcs, fstart=fstart, callers=callers, callees=callees, fn_str=fn_str, tva=tva)

if os.path.exists(CACHE):
    G = pickle.load(open(CACHE, 'rb'))
else:
    G = {'o': build(O, 'o'), 'n': build(N, 'n')}
    pickle.dump(G, open(CACHE, 'wb'))
GO, GN = G['o'], G['n']

def fn_of(g, rva):
    fs = g['fstart']
    i = np.searchsorted(fs, rva, 'right') - 1
    if i < 0: return None
    return int(fs[i])

if __name__ == '__main__':
    print('0.5.3 funcs', len(GO['funcs']), 'str-fn', len(GO['fn_str']))
    print('0.5.4 funcs', len(GN['funcs']), 'str-fn', len(GN['fn_str']))
