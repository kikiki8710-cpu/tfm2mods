# -*- coding: utf-8 -*-
"""함수 내 Rust panic Location{ptr,len,line,col} 참조 추출 → 소스파일:라인 지문."""
import sys, re, struct
sys.path.insert(0, r'C:\tfm2mods')
from _it54 import O, N, BASE

def locs(E, rva):
    f = E.func_of(rva)
    b = E.read(f[0], f[1]-f[0])
    out = []
    rn, rva_, rvs, rpr, rsr = next(x for x in E.secs if x[0] == '.rdata')
    for ins in E.md.disasm(b, BASE + f[0]):
        if ins.mnemonic != 'lea': continue
        m = re.search(r'\[rip \+ (0x[0-9a-f]+)\]', ins.op_str)
        if not m: continue
        t = ins.address + ins.size + int(m.group(1), 16) - BASE
        o = E.off(t)
        if o is None: continue
        try:
            p, L, line, col = struct.unpack_from('<QQII', E.data, o)
        except Exception:
            continue
        if not (0 < L < 200): continue
        pr = p - BASE
        po = E.off(pr)
        if po is None: continue
        s = E.data[po:po+L]
        if not all(0x20 <= c < 0x7f for c in s): continue
        s = s.decode('latin1')
        if not s.endswith('.rs'): continue
        out.append((hex(ins.address - BASE), s.replace(chr(92),'/').split('/')[-1], line, col))
    return out

if __name__ == '__main__':
    import collections
    for tag, E, fs in (('0.5.3', O, [0x1925ab0, 0x2256320, 0x18f6c30, 0x997740, 0x220420, 0x1951b80, 0x20da080, 0x229a410]),
                       ('0.5.4', N, [0x235bf20, 0x1ccf9a0, 0x2323aa0, 0x9d5f20, 0x222200, 0x2392ed0, 0x19b8e30, 0x1d13e60])):
        for f in fs:
            L = locs(E, f)
            c = collections.Counter(x[1] for x in L)
            print(tag, hex(f), c.most_common(4), 'lines', sorted({x[2] for x in L})[:8])
