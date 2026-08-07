# -*- coding: utf-8 -*-
"""reach.py — 함수 내부 CFG 도달성. 특정 명령들을 '차단'했을 때 target 이 여전히 도달 가능한가?
  python reach.py 054 <fn내RVA> <target> [block1,block2,...]
차단주소(block)는 '그 명령을 실행할 수 없다'로 취급 → 해당 명령 이후/분기 모두 끊는다.
용도: "version>=2 검사를 통과하지 않고도 이 호출에 닿는가?" 판정.
"""
import sys, io
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner
from pe2 import BASE
import capstone
JCC=set('jo jno jb jae je jne jbe ja js jns jp jnp jl jge jle jg'.split())
ver=sys.argv[1]; anchor=int(sys.argv[2],16); tgt=int(sys.argv[3],16)
blocks=set(int(x,16) for x in sys.argv[4].split(',')) if len(sys.argv)>4 and sys.argv[4] else set()
S=Scanner(ver); f=S.func_of(anchor); ins=S.disf(f)
idx={i.address-BASE:k for k,i in enumerate(ins)}
seen=set(); stack=[f[0]]; path={f[0]:None}
while stack:
    a=stack.pop()
    if a in seen or a in blocks or a not in idx: continue
    seen.add(a); i=ins[idx[a]]
    nxts=[]
    if i.mnemonic in JCC:
        for op in i.operands:
            if op.type==capstone.x86.X86_OP_IMM: nxts.append(op.imm-BASE)
        nxts.append(i.address-BASE+i.size)
    elif i.mnemonic=='jmp':
        for op in i.operands:
            if op.type==capstone.x86.X86_OP_IMM: nxts.append(op.imm-BASE)
    elif i.mnemonic in ('ret','ud2'):
        pass
    else:
        nxts.append(i.address-BASE+i.size)
    for n in nxts:
        if n not in path: path[n]=a
        stack.append(n)
print('fn %06x-%06x  방문 %d명령  차단 %s'%(f[0],f[1],len(seen),[hex(b) for b in blocks]))
print('target %06x 도달: %s'%(tgt, 'YES' if tgt in seen else 'NO'))
if tgt in seen:
    p=[]; c=tgt
    while c is not None and len(p)<400:
        p.append(c); c=path.get(c)
    print('  역경로(최대40): '+' <- '.join('%06x'%x for x in p[:40]))
