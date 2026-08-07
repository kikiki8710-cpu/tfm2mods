# -*- coding: utf-8 -*-
"""immediate histogram of a function (and optional pair-diff)"""
import sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner, src_of
import capstone
def hist(ver, fs, lo=256):
    S = Scanner(ver); f = S.func_of(fs)
    h = collections.Counter()
    for i in S.disf(f):
        for op in i.operands:
            if op.type == capstone.x86.X86_OP_IMM and abs(op.imm) >= lo:
                h[op.imm] += 1
    return f, h
if __name__=='__main__':
    a = hist(sys.argv[1], int(sys.argv[2],16))
    if len(sys.argv) > 4:
        b = hist(sys.argv[3], int(sys.argv[4],16))
        ks = sorted(set(a[1])|set(b[1]))
        print('%-14s %-6s %-6s' % ('imm','A','B'))
        for k in ks:
            x,y = a[1].get(k,0), b[1].get(k,0)
            mark = '  ' if x==y else '*'
            print('%s%-12d 0x%-10x %3d %3d' % (mark,k,k&0xffffffffffffffff,x,y))
    else:
        for k,v in sorted(a[1].items()):
            print('%-12d 0x%-10x %d' % (k,k&0xffffffffffffffff,v))
