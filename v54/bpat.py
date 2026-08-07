# -*- coding: utf-8 -*-
"""bpat.py - 바이트 패턴(정규식) 사이트 전수 + 소스집계. 노브 사이트 세기용.
  python bpat.py <ver> <hexpat with '..' wildcards> [srcfilter]
"""
import sys, re, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner
import cen
from pe2 import BASE

def find(ver, pat, filt=None):
    S = cen.sc(ver); M = cen.sm(ver)
    rx = b''
    for i in range(0, len(pat), 2):
        t = pat[i:i+2]
        rx += b'.' if t == '..' else re.escape(bytes([int(t, 16)]))
    out = collections.defaultdict(list)
    for m in re.finditer(rx, S.body, re.DOTALL):
        rva = S.tva + m.start()
        f = S.func_of(rva)
        if not f:
            continue
        src = M.get(f[0], '(nosrc)')
        if filt and filt.lower() not in src.lower():
            continue
        ok = False
        for i in S.disf(f):
            a = i.address - BASE
            if a == rva and i.size == len(pat) // 2:
                ok = (a, f[0], i)
                break
            if a > rva:
                break
        if ok:
            out[src].append(ok)
        S._dis.clear()
    return out

if __name__ == '__main__':
    by = find(sys.argv[1], sys.argv[2], sys.argv[3] if len(sys.argv) > 3 else None)
    tot = sum(len(v) for v in by.values())
    print('== pat %s -> %d사이트 / %d그룹' % (sys.argv[2], tot, len(by)))
    for k in sorted(by, key=lambda x: -len(by[x])):
        print('  [%3d] %s' % (len(by[k]), k[:95]))
        for a, fs, i in by[k][:60]:
            print('        %06x fn %06x  %-22s %s %s' % (a, fs, i.bytes.hex(), i.mnemonic, i.op_str))
