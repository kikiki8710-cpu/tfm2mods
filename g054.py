# -*- coding: utf-8 -*-
# g054.py — 0.5.3/0.5.4 콜그래프 (e8 direct call 선형스캔)
import struct, collections, bisect
import bp054 as B

def graph(d, secs, fns):
    va, vsz, rr, rs = B.sec(secs, ".text")
    blob = d[rr:rr+rs]
    starts = [f[0] for f in fns]
    ends = {f[0]: f[1] for f in fns}
    caller = collections.defaultdict(collections.Counter)
    callee = collections.defaultdict(collections.Counter)
    i, n = 0, len(blob)
    while True:
        i = blob.find(b"\xe8", i)
        if i < 0 or i+5 > n: break
        rel = struct.unpack_from("<i", blob, i+1)[0]
        site = va + i
        t = site + 5 + rel
        if va <= t < va + rs:
            j = bisect.bisect_right(starts, site) - 1
            if j >= 0 and site < ends[starts[j]]:
                w = starts[j]
                caller[t][w] += 1
                callee[w][t] += 1
        i += 1
    return caller, callee

CO, EO = graph(B.DO, B.SO, B.FO)
CN, EN = graph(B.DN, B.SN, B.FN)
