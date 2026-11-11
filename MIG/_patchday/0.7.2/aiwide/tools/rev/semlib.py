import sys, os, pickle, collections, struct
T=r'F:\claude\tfm2mods\MIG\_patchday\0.7.2\aiwide\tools'
sys.path.insert(0,T)
from common import *
HERE2=os.path.dirname(os.path.abspath(__file__))
_g=pickle.load(open(os.path.join(HERE2,'gmap.pkl'),'rb'))
G=_g['G']
for _ko,(_kn,_d,_n) in pairs.items(): G[_ko[0]]=_kn[0]
GI={v:k for k,v in G.items()}
LO,LN=locs()
SS=('movss','addss','subss','mulss','divss','comiss','ucomiss','maxss','minss','cvtss2sd','cmpss')
SD=('movsd','addsd','subsd','mulsd','divsd','comisd','ucomisd','maxsd','minsd','cvtsd2ss','cmpsd')
# 0.7.1→0.7.2 보폭·크기 정규화(구 쪽 즉치만 신 값으로 바꾼다 — 신 쪽에 같은 수가 다른 뜻으로 있어도 건드리지 않게).
#   Blackboard 보폭 0xf18→0x1010 : `imul r,r,0xf18`→`imul r,r,0x1010` 261함수(예 0xe37100→0xe38d40 assign.rs @e37172/e38db2)
#   BigPlan 크기 0x4c8→0x4d8     : memcpy 크기 `mov r8d,0x4c8`→`mov r8d,0x4d8`(예 handler.rs 0xe3a040→0xe3c350 @e3dad3/e4007d)
#   근거·빈도 = aiwide\공통원인.md §2. 0.7.1 판 {0xea0:0xf18,0x400:0x4c8} 은 0.7.0→0.7.1 용이라 뺐다.
_M=0xffffffffffffffff
NORM={0xf18:0x1010, (-0xf18)&_M:(-0x1010)&_M, 0x4c8:0x4d8, (-0x4c8)&_M:(-0x4d8)&_M}
def norm(v,old=True):
    v&=_M
    return NORM.get(v,v) if old else v
def toks(img,k,old):
    LOC=LO if old else LN
    out=[]
    for i in dis(img,k):
        mn=i.mnemonic; a=i.address-BASE
        if mn.startswith('nop') or mn=='int3': continue
        if mn=='call' or mn=='jmp':
            op=i.operands[0]
            if op.type==X86_OP_IMM:
                t=op.imm-BASE
                if k[0]<=t<k[1]: continue
                if old:
                    out.append(('call','%x'%G[t]) if t in G else ('call?','o%x'%t))
                else:
                    out.append(('call','%x'%t) if t in GI else ('call?','n%x'%t))
                out[-1]=out[-1]+(a,)
            elif op.type==X86_OP_MEM and op.mem.base not in (X86_REG_RIP,X86_REG_RSP,X86_REG_RBP):
                out.append(('vcall','+%x'%op.mem.disp,a))
            continue
        if mn.startswith('j'):
            out.append(('jcc','',a)); continue
        for op in i.operands:
            if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                t=i.address+i.size+op.mem.disp-BASE
                if t in LOC: continue
                if mn in SS: out.append(('f','%g'%struct.unpack('<f',img.code(t,4))[0],a))
                elif mn in SD: out.append(('d','%g'%struct.unpack('<d',img.code(t,8))[0],a))
                elif mn in ('movaps','movups','movdqa','movdqu','andps','xorps','andpd','xorpd','orps','mulps','addps','subps','minps','maxps','cmpps','movapd','mulpd','addpd'):
                    out.append(('x',img.code(t,16).hex(),a))
            elif op.type==X86_OP_IMM:
                v=op.imm
                if 0x140000000<=v<0x146000000: continue
                if mn in ('sub','add') and i.operands[0].type==X86_OP_REG and md.reg_name(i.operands[0].reg)=='rsp': continue
                if mn in ('cmp','test','mov','movabs','imul','and','or','add','sub','xor') and abs(v)>=3 and (v&0xffffffffffffffff) not in (0xff,0xffffffff,0xffffffffffffffff,0xffff):
                    out.append(('imm','%s %#x'%('cmp' if mn in ('cmp','test') else 'op',norm(v,old)),a))
    return out
def delta(ko,kn):
    O=toks(io_,ko,True); N=toks(inn,kn,False)
    co=collections.Counter((t,v) for t,v,_ in O if t!='jcc'); cn=collections.Counter((t,v) for t,v,_ in N if t!='jcc')
    jo=sum(1 for t,_,_ in O if t=='jcc'); jn=sum(1 for t,_,_ in N if t=='jcc')
    ao=collections.defaultdict(list); an=collections.defaultdict(list)
    for t,v,x in O: ao[(t,v)].append(x)
    for t,v,x in N: an[(t,v)].append(x)
    d=[(kk,co[kk],cn[kk],ao[kk][:4],an[kk][:4]) for kk in sorted(set(co)|set(cn)) if co[kk]!=cn[kk]]
    return d,(jo,jn)
