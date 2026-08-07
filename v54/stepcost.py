# -*- coding: utf-8 -*-
"""stepcost.py - free_dist 그래디언트 소비자의 스텝비용 관용구 **완전** 전수.
   두 형태를 모두 잡는다(블록 재배치로 lea/add 가 떨어져 있어도 레지스터로 짝짓기):
     A) lea r32,[b + i*S] ... add r32, imm   → 직교=imm, 대각=imm+S
     B) lea r32,[b + i*S + disp]             → 직교=disp, 대각=disp+S  (융합형)
  python stepcost.py <ver> [srcfilter]
"""
import sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
import cen, capstone
from pe2 import BASE
X = capstone.x86


def is32(i):
    for b in i.bytes:
        if 0x40 <= b <= 0x4f:
            return not (b & 8)
        break
    return True


def run(ver, filt='free_dist,path_f'):
    S = cen.sc(ver); M = cen.sm(ver)
    pats = [p.strip().lower() for p in filt.split(',')] if filt else None
    A, Bf, orphanA, orphanL = [], [], [], []
    for f in S.funcs:
        src = M.get(f[0], '(nosrc)')
        if pats and not any(p in src.lower() for p in pats):
            continue
        ins = list(S.disf(f)); S._dis.clear()
        leas = {}   # reg -> list of lea (scale2, no disp)
        for i in ins:
            if i.mnemonic == 'lea' and is32(i):
                m = i.operands[1].mem
                if m.index and m.base and m.scale >= 2:
                    if m.disp and 1 <= m.disp <= 32:
                        Bf.append((i.address - BASE, f[0], src, m.scale, m.disp,
                                   i.bytes.hex(), i.encoding.disp_offset, i.encoding.disp_size))
                    elif not m.disp:
                        leas.setdefault(i.operands[0].reg, []).append(i)
        used = set()
        for i in ins:
            if i.mnemonic != 'add' or not is32(i):
                continue
            o = i.operands
            if o[0].type != X.X86_OP_REG or o[1].type != X.X86_OP_IMM:
                continue
            if not (1 <= o[1].imm <= 32):
                continue
            cands = leas.get(o[0].reg, [])
            if not cands:
                orphanA.append((i.address - BASE, f[0], src, o[1].imm, i.bytes.hex()))
                continue
            L = min(cands, key=lambda x: abs(x.address - i.address))
            used.add(L.address)
            A.append((i.address - BASE, f[0], src, L.address - BASE, L.operands[1].mem.scale,
                      o[1].imm, L.bytes.hex(), i.bytes.hex(), i.encoding.imm_offset,
                      L.encoding.modrm_offset + 1))   # SIB 바이트 위치
        for r, lst in leas.items():
            for L in lst:
                if L.address not in used:
                    orphanL.append((L.address - BASE, f[0], src, L.operands[1].mem.scale, L.bytes.hex()))
    return A, Bf, orphanA, orphanL


if __name__ == '__main__':
    ver = sys.argv[1]
    filt = sys.argv[2] if len(sys.argv) > 2 else 'free_dist,path_f'
    A, Bf, oA, oL = run(ver, filt)
    print('== A) 분리형 lea(scale)+add(imm) : %d' % len(A))
    for r in A:
        print('   add %06x %-10s imm=%-3d imm_off=%d | lea %06x %-12s S=%d sib_off=%d | fn %06x %s'
              % (r[0], r[7], r[5], r[8], r[3], r[6], r[4], r[9], r[1], r[2][:44]))
    print('== B) 융합형 lea[b+i*S+disp] : %d' % len(Bf))
    for r in Bf:
        print('   lea %06x %-14s S=%d disp=%d disp_off=%d w=%d | fn %06x %s'
              % (r[0], r[5], r[3], r[4], r[6], r[7], r[1], r[2][:44]))
    print('== 짝 못 찾은 add : %d' % len(oA))
    for r in oA:
        print('   add %06x %-10s imm=%d fn %06x %s' % (r[0], r[4], r[3], r[1], r[2][:40]))
    print('== 짝 못 찾은 lea(scale) : %d' % len(oL))
    for r in oL[:20]:
        print('   lea %06x %-12s S=%d fn %06x %s' % (r[0], r[4], r[3], r[1], r[2][:40]))
