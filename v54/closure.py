# -*- coding: utf-8 -*-
"""함수의 **재현 폐포**를 잰다 — 직접 call 을 1~2단계 따라가며 크기·call 수를 합산.

완전 재구현 원칙상 callee 도 Rust 로 구현해야 하므로, 표적의 진짜 비용은 폐포 크기다.
사용: python closure.py <rva> [depth]"""
import sys, io, struct
import bisect as _bs
import pefile, capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
pe = pefile.PE(EXE, fast_load=True)
data = pe.__data__
pd = [s for s in pe.sections if s.Name.rstrip(b'\x00') == b'.pdata'][0]
raw = data[pd.PointerToRawData: pd.PointerToRawData + pd.SizeOfRawData]
fs = []
for i in range(0, len(raw) - 11, 12):
    b, e, _ = struct.unpack_from('<III', raw, i)
    if b == 0:
        break
    fs.append((b, e))
fs.sort()
st = [f[0] for f in fs]


def own(r):
    i = _bs.bisect_right(st, r) - 1
    return fs[i] if i >= 0 and fs[i][0] <= r < fs[i][1] else None


def off(rva):
    for s in pe.sections:
        if s.VirtualAddress <= rva < s.VirtualAddress + max(s.Misc_VirtualSize, s.SizeOfRawData):
            return s.PointerToRawData + (rva - s.VirtualAddress)
    return None


md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)


def scan(b, e):
    """(직접 callee rva 집합, 간접 call 수)"""
    d, ind = set(), 0
    o = off(b)
    if o is None:
        return d, 0
    for x in md.disasm(data[o:o + (e - b)], 0x140000000 + b):
        if x.mnemonic == 'call':
            if x.op_str.startswith('0x'):
                d.add(int(x.op_str, 16) - 0x140000000)
            else:
                ind += 1
    return d, ind


root = int(sys.argv[1], 16)
maxd = int(sys.argv[2]) if len(sys.argv) > 2 else 2
seen, frontier, total, ind_total = {}, [(root, 0)], 0, 0
while frontier:
    r, dep = frontier.pop()
    f = own(r)
    if not f or f in seen:
        continue
    d, ind = scan(*f)
    seen[f] = (dep, f[1] - f[0], len(d), ind)
    total += f[1] - f[0]
    ind_total += ind
    if dep < maxd:
        for c in d:
            frontier.append((c, dep + 1))

rows = sorted(seen.items(), key=lambda kv: (kv[1][0], -kv[1][1]))
print('%-12s %5s %7s %6s %6s' % ('함수', '깊이', '크기', '직접', '간접'))
for (b, e), (dep, size, nd, ni) in rows[:26]:
    print('%#-12x %5d %7d %6d %6d' % (b, dep, size, nd, ni))
print('\n폐포: 함수 %d개 · 총 %d바이트 · 간접호출 %d개 (깊이 %d까지)'
      % (len(seen), total, ind_total, maxd))
