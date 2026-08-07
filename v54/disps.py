# -*- coding: utf-8 -*-
"""함수가 쓰는 메모리 displacement 목록(레지스터 베이스만, rsp/rbp/rip 제외) + 인덱스 유무.
  python disps.py 054 da1850            # 목록
  python disps.py 054 da1850 idx        # 인덱스레지스터 있는 것만(배열 접근)
"""
import io, sys, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
import capstone
ver, rva = sys.argv[1], int(sys.argv[2], 16)
only_idx = len(sys.argv) > 3 and sys.argv[3] == 'idx'
e = load(ver)
f = e.func_of(rva)
ins = list(e.md.disasm(e.rd(f[0], f[1] - f[0]), BASE + f[0]))
c = collections.Counter(); ex = {}
for i in ins:
    for op in i.operands:
        if op.type != capstone.x86.X86_OP_MEM: continue
        b = i.reg_name(op.mem.base) if op.mem.base else None
        if b in (None, 'rsp', 'rbp', 'rip'): continue
        hasidx = op.mem.index != 0
        if only_idx and not hasidx: continue
        k = (op.mem.disp, hasidx, op.mem.scale)
        c[k] += 1
        ex.setdefault(k, '%06x %s %s' % (i.address - BASE, i.mnemonic, i.op_str))
print('fn %06x-%06x' % f)
for (d, hi, sc), n in sorted(c.items()):
    print('  +0x%-6x idx=%s scale=%d  x%-3d  %s' % (d & 0xffffffff, 'Y' if hi else 'n', sc, n, ex[(d, hi, sc)]))
