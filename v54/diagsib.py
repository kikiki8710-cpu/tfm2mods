# -*- coding: utf-8 -*-
"""diagsib.py - free_dist 그래디언트 소비자의 '스텝비용 = base + scale*is_diag' 관용구 전수.
   패턴: lea r32,[rA + rB*S] 직후 add r32, imm8  (32비트 전용, 즉 REX.W 없음)
   → 직교=imm, 대각=imm+S. S 는 SIB 스케일 2비트라 {1,2,4,8}만.
  python diagsib.py 054 [base값] [srcfilter]
"""
import sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
import cen, capstone
from pe2 import BASE
X = capstone.x86

ver = sys.argv[1]
want = int(sys.argv[2]) if len(sys.argv) > 2 else None
filt = sys.argv[3] if len(sys.argv) > 3 else None
S = cen.sc(ver); M = cen.sm(ver)
out = collections.defaultdict(list)
tot = 0
R32 = set(capstone.x86.X86_REG_EAX + i for i in range(0))


def is32(insn):
    # REX.W 없음 = 32비트 피연산자
    for b in insn.bytes:
        if 0x40 <= b <= 0x4f:
            return not (b & 8)
        break
    return True


for f in S.funcs:
    src = M.get(f[0], '(nosrc)')
    if filt and filt.lower() not in src.lower():
        continue
    ins = list(S.disf(f))
    for a, b in zip(ins, ins[1:]):
        if a.mnemonic != 'lea' or b.mnemonic != 'add':
            continue
        if not is32(a) or not is32(b):
            continue
        if a.operands[0].type != X.X86_OP_REG:
            continue
        m = a.operands[1].mem
        if m.index == 0 or m.base == 0 or m.scale < 2 or m.disp:
            continue
        if b.operands[0].type != X.X86_OP_REG or b.operands[0].reg != a.operands[0].reg:
            continue
        if b.operands[1].type != X.X86_OP_IMM:
            continue
        c = b.operands[1].imm
        if want is not None and c != want:
            continue
        if not (1 <= c <= 32):
            continue
        out[src].append((a.address - BASE, f[0], m.scale, c, a.bytes.hex(),
                         b.address - BASE, b.bytes.hex(), b.encoding.imm_offset))
        tot += 1
    S._dis.clear()
print('== 총 %d사이트 / %d그룹' % (tot, len(out)))
for k in sorted(out, key=lambda x: -len(out[x])):
    print('  [%3d] %s' % (len(out[k]), k[:90]))
    for r in out[k]:
        print('        lea %06x %-12s S=%d | add %06x %-10s base=%d imm_off=%d | fn %06x'
              % (r[0], r[4], r[2], r[5], r[6], r[3], r[7], r[1]))

# --- 융합형: lea r32,[base + index*S + disp] 단일 명령 (disp=직교비용, S=대각가산)
print()
print('=== 융합형 lea r32,[b + i*S + disp] (disp=%s) ===' % want)
out2 = collections.defaultdict(list)
t2 = 0
for f in S.funcs:
    src = M.get(f[0], '(nosrc)')
    if filt and filt.lower() not in src.lower():
        continue
    for i in S.disf(f):
        if i.mnemonic != 'lea' or not is32(i):
            continue
        m = i.operands[1].mem
        if m.index == 0 or m.base == 0 or m.scale < 2:
            continue
        if want is not None and m.disp != want:
            continue
        if not (1 <= m.disp <= 32):
            continue
        e = i.encoding
        out2[src].append((i.address - BASE, f[0], m.scale, m.disp, i.bytes.hex(),
                          e.disp_offset, e.disp_size))
        t2 += 1
    S._dis.clear()
print('총 %d사이트 / %d그룹' % (t2, len(out2)))
for k in sorted(out2, key=lambda x: -len(out2[k := x])):
    print('  [%3d] %s' % (len(out2[k]), k[:90]))
    for r in out2[k]:
        print('        lea %06x %-14s S=%d disp=%d disp_off=%d w=%d | fn %06x'
              % (r[0], r[4], r[2], r[3], r[5], r[6], r[1]))
