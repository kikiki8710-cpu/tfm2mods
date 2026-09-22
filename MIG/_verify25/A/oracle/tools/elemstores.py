# -*- coding: utf-8 -*-
"""action_candidates 범위 안에서 원소 alloca(184B) 별 store/memcpy 전수 → 기록 바이트 범위"""
from load import *
import re, sys
targets = sys.argv[1:]
regs = {t: (t, 0) for t in targets}
out = {t: [] for t in targets}
def sz(ty):
    return {'i8':1,'i16':2,'i32':4,'i64':8,'ptr':8,'i1':1}.get(ty, 0)
for i in range(LO, HI+1):
    s = L(i)
    m = re.match(r'\s*(%\d+) = getelementptr inbounds (?:nuw )?i8, ptr (%\d+), i64 (\d+)', s)
    if m and m.group(2) in regs:
        b, o = regs[m.group(2)]; regs[m.group(1)] = (b, o + int(m.group(3))); continue
    m = re.match(r'\s*store (\S+) ([^,]+), ptr (%\d+)', s)
    if m and m.group(3) in regs:
        b, o = regs[m.group(3)]; n = sz(m.group(1))
        out[b].append((o, o+n, i, 'store %s %s' % (m.group(1), m.group(2)[:24]), chain(i))); continue
    if 'llvm.memcpy' in s:
        mm = re.search(r'\(ptr [^%]*(%\d+), ptr [^%]*(%\d+), i64 (\d+)', s)
        if mm and mm.group(1) in regs:
            b, o = regs[mm.group(1)]; n = int(mm.group(3))
            out[b].append((o, o+n, i, 'memcpy%dB from %s' % (n, mm.group(2)), chain(i)))
        continue
    m = re.search(r'(?:call|invoke) [^@]*@(\S+?)\(([^)]*)\)', s)
    if m:
        for r, (b, o) in regs.items():
            if re.search(r'ptr [^,]*' + re.escape(r) + r'(?:[,)]|$)', m.group(2)):
                out[b].append((o, None, i, 'CALL %s' % m.group(1)[-50:], chain(i)))
for t in targets:
    print('====', t)
    for r in sorted(out[t], key=lambda x: (x[2])):
        print('  %#5x..%s  L%d  %s  ;L%s' % (r[0], ('%#x' % r[1]) if r[1] is not None else '?', r[2], r[3], r[4]))
