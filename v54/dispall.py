# -*- coding: utf-8 -*-
"""dispall.py — .text **전역**에서 특정 구조체 오프셋(disp32/disp8)을 쓰는 명령 전수.
fld.py 는 srcmap2 에 등재된 함수만 본다(=소스 없는 함수를 놓친다). 이건 .pdata 의 모든 함수를 본다.
  python dispall.py 054 0x2888 [ctx]
"""
import sys, io, os, struct, re, bisect
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
import capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
D = r'C:\tfm2mods\v54'

def srcmap2(ver):
    rows=[]
    for ln in io.open(os.path.join(D,'%s_srcmap2.tsv'%ver),encoding='utf-8'):
        s,e,src,l=ln.rstrip('\n').split('\t'); rows.append((int(s,16),int(e,16),src))
    rows.sort(); return rows

ver=sys.argv[1]; want=int(sys.argv[2],0); ctx=int(sys.argv[3]) if len(sys.argv)>3 else 0
SRCF=sys.argv[4].lower() if len(sys.argv)>4 else ''
e=load(ver); sm=srcmap2(ver); ks=[r[0] for r in sm]
def src(rva):
    i=bisect.bisect_right(ks,rva)-1
    return sm[i][2] if i>=0 and sm[i][0]<=rva<sm[i][1] else ''
_,tva,tvsz,tra,trsz=[s for s in e.sections if s[0]=='.text'][0]
body=e.raw[tra:tra+trsz]
# 후보 함수 = disp 바이트열이 등장하는 위치의 소속 함수
cands=set()
pats=[struct.pack('<i',want)]
if -128<=want<128: pats.append(bytes([want & 0xff]))
for p in pats:
    for m in re.finditer(re.escape(p), body):
        f=e.func_of(tva+m.start())
        if f and (not SRCF or SRCF in src(f[0]).lower()): cands.add(f)
md=capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail=True
tot=0
for f in sorted(cands):
    ins=list(md.disasm(body[f[0]-tva:f[1]-tva], BASE+f[0]))
    hits=[k for k,i in enumerate(ins) if any(o.type==capstone.x86.X86_OP_MEM and o.mem.disp==want and o.mem.base!=0 for o in i.operands)]
    if not hits: continue
    print('--- fn %06x-%06x  %s'%(f[0],f[1],src(f[0])[:80]))
    shown=set()
    for k in hits:
        for j in range(max(0,k-ctx), min(len(ins),k+ctx+1)):
            if j in shown: continue
            shown.add(j); i=ins[j]
            print('  %s%06x  %-22s %s %s'%('>' if j==k else ' ', i.address-BASE, i.bytes.hex(), i.mnemonic, i.op_str))
        tot+=1
print('총 %d 사이트 / 함수 %d개'%(tot,len(cands)))
