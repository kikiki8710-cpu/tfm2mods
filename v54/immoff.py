# -*- coding: utf-8 -*-
"""immoff.py — 구간 내 모든 즉시값/변위를 '명령바이트 + imm오프셋 + 폭'으로 뽑는다(노브 후보표용).
사용: python immoff.py <ver> <lo(hex)> <hi(hex)> [minabs]
출력: RVA | bytes | mnemonic op | imm값 | imm_off | width
imm_off = 명령 시작으로부터 즉시값이 놓인 바이트 오프셋(바이트패치 위치).
조건분기(jcc)도 함께 표기해 'jcc→jmp/nop' 개입 후보로 쓴다."""
import sys, struct
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner
from pe2 import BASE
import capstone
ver, lo, hi = sys.argv[1], int(sys.argv[2],16), int(sys.argv[3],16)
S = Scanner(ver); f = S.func_of(lo)
JCC = set('jo jno jb jae je jne jbe ja js jns jp jnp jl jge jle jg'.split())
for i in S.disf(f):
    a = i.address - BASE
    if not (lo <= a < hi): continue
    b = i.bytes
    outs = []
    for op in i.operands:
        if op.type == capstone.x86.X86_OP_IMM:
            v = op.imm
            for w in (1,2,4,8):
                for off in range(len(b)-w+1):
                    if w==1: cand = struct.unpack_from('<b', b, off)[0]
                    elif w==2: cand = struct.unpack_from('<h', b, off)[0]
                    elif w==4: cand = struct.unpack_from('<i', b, off)[0]
                    else: cand = struct.unpack_from('<q', b, off)[0]
                    if cand == v and off+w == len(b):
                        outs.append(('IMM', v, off, w)); break
                else: continue
                break
            else: outs.append(('IMM', v, -1, 0))
        elif op.type == capstone.x86.X86_OP_MEM and op.mem.disp and abs(op.mem.disp) > 0:
            d = op.mem.disp
            for w in (1,4):
                for off in range(len(b)-w+1):
                    cand = struct.unpack_from('<b' if w==1 else '<i', b, off)[0]
                    if cand == d:
                        outs.append(('DISP', d, off, w)); break
                else: continue
                break
    tag = ' <JCC>' if i.mnemonic in JCC else ''
    s = '  '.join('%s=%d(0x%x) off=%d w=%d'%(t,v,v&0xffffffffffffffff,o,w) for t,v,o,w in outs)
    print('%06x  %-26s %-32s %s%s'%(a, b.hex(), i.mnemonic+' '+i.op_str, s, tag))
