# -*- coding: utf-8 -*-
"""053 노브 사이트 각각의 '명령 지문'을 만들고, 054 대응 함수에서 같은 지문의 개수를 센다.
지문 = (mnemonic, imm값, 피연산자 종류열, mem.disp) — 레지스터는 무시(재할당 허용)."""
import io, sys, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0,'C:/tfm2mods/v54')
from pe2 import load, BASE
import capstone
E={}
def ee(v):
    if v not in E: E[v]=load(v)
    return E[v]
DC={}
def disf(ver,s,e):
    k=(ver,s)
    if k in DC: return DC[k]
    x=ee(ver); r=list(x.md.disasm(x.rd(s,e-s), BASE+s)); DC[k]=r; return r
def fr(ver,s):
    for a,b in ee(ver).funcs():
        if a==s: return (a,b)
    return None
def fp(i, strict_disp=True):
    """명령 지문"""
    kinds=[]; imm=None; disp=None
    for op in i.operands:
        if op.type==capstone.x86.X86_OP_IMM: kinds.append('i'); imm=op.imm
        elif op.type==capstone.x86.X86_OP_REG: kinds.append('r')
        elif op.type==capstone.x86.X86_OP_MEM:
            kinds.append('m'); disp=op.mem.disp
    return (i.mnemonic, imm, ''.join(kinds), disp if strict_disp else None)
def run(label, ver3, f3, ver4, f4, sites, strict_disp=True):
    r3=fr(ver3,f3); r4=fr(ver4,f4)
    if not r3 or not r4:
        print('!! 함수 범위 없음', label); return
    i3=disf(ver3,*r3); i4=disf(ver4,*r4)
    by3=collections.Counter(fp(i,strict_disp) for i in i3)
    by4=collections.Counter(fp(i,strict_disp) for i in i4)
    at={i.address-BASE:i for i in i3}
    groups=collections.OrderedDict()
    miss=[]
    for a in sites:
        i=at.get(a)
        if i is None: miss.append(a); continue
        groups.setdefault(fp(i,strict_disp),[]).append(a)
    print('## %s   053 %06x(%dB) -> 054 %06x(%dB)   노브사이트 %d'%(label,f3,r3[1]-r3[0],f4,r4[1]-r4[0],len(sites)))
    if miss: print('   (명령경계 불일치·디코드실패 %d: %s)'%(len(miss),' '.join('%06x'%x for x in miss)))
    tot3=tot4=0; bad=[]
    for sg,addrs in groups.items():
        c3=by3[sg]; c4=by4[sg]; tot3+=c3; tot4+=c4
        st='OK ' if c3==c4 else '<<<%+d>>>'%(c4-c3)
        if c3!=c4: bad.append((sg,c3,c4,addrs))
        print('   %s %-7s imm=%-12s ops=%-4s disp=%-8s | 053 %-3d 054 %-3d  (노브 %d곳: %s)'%(
            st, sg[0], sg[1], sg[2], hex(sg[3]) if sg[3] else '-', c3, c4, len(addrs), ' '.join('%06x'%x for x in addrs[:6])))
    print('   >> 지문총계 053 %d / 054 %d   불일치 %d종'%(tot3,tot4,len(bad)))
    return bad
