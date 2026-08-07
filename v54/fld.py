# -*- coding: utf-8 -*-
"""필드 오프셋 사용처 스캐너 (소스 필터 + 문맥). scan.py 의 소스필터판.
  python fld.py 054 0x658 game-ai 2        # game-ai 소스 함수 안에서 disp=0x658, 앞뒤 2줄
  python fld.py 054 0x658,0x610 game-ai 0  # 두 오프셋을 **같은 함수**에서 쓰는 곳만
"""
import io, os, re, struct, sys, bisect, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
import capstone

D = r'C:\tfm2mods\v54'

def srcmap2(ver):
    rows = []
    for ln in io.open(os.path.join(D, '%s_srcmap2.tsv' % ver), encoding='utf-8'):
        s, e, src, lines = ln.rstrip('\n').split('\t')
        rows.append((int(s, 16), int(e, 16), src, lines))
    rows.sort()
    return rows

def main():
    ver = sys.argv[1]
    vals = [int(x, 0) for x in sys.argv[2].split(',')]
    filt = sys.argv[3] if len(sys.argv) > 3 else ''
    ctx = int(sys.argv[4]) if len(sys.argv) > 4 else 0
    e = load(ver)
    sm = [r for r in srcmap2(ver) if filt.lower() in r[2].lower()]
    print('대상 함수 %d개 (필터=%s)' % (len(sm), filt))
    tot = 0
    for s, en, src, lines in sm:
        ins = list(e.md.disasm(e.rd(s, en - s), BASE + s))
        found = collections.defaultdict(list)
        for k, i in enumerate(ins):
            for op in i.operands:
                if op.type == capstone.x86.X86_OP_MEM and op.mem.disp in vals and op.mem.base != 0:
                    bn = i.reg_name(op.mem.base)
                    if bn in ('rsp', 'rbp', 'rip'):
                        continue
                    found[op.mem.disp].append(k)
        if len(found) < len(vals):
            continue
        print('--- fn %06x-%06x  %s [줄 %s]' % (s, en, src[:90], lines[:60]))
        ks = sorted(set(x for v in found.values() for x in v))
        shown = set()
        for k in ks:
            for j in range(max(0, k - ctx), min(len(ins), k + ctx + 1)):
                if j in shown: continue
                shown.add(j)
                i = ins[j]
                print('    %06x  %-22s %s %s' % (i.address - BASE, i.bytes.hex(), i.mnemonic, i.op_str))
            tot += 1
    print('총 %d 사이트' % tot)

main()
