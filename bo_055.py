# -*- coding: utf-8 -*-
"""tfm2_banpick_order 0.5.4 -> 0.5.5 RVA 재핀 (capstone/pefile 오프라인).
bo_054.py 의 Exe/make_pattern/find 를 0.5.4->0.5.5 로 재사용."""
import sys, struct, bisect
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_REG_RIP, X86_OP_IMM, X86_GRP_JUMP, X86_GRP_CALL

BASE = 0x140000000
E54P = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe"
E55P = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.5\TeamfightManager2.exe"

class Exe:
    def __init__(self, path):
        self.path = path
        self.pe = pefile.PE(path, fast_load=True)
        self.data = open(path,'rb').read()
        self.secs = []
        for s in self.pe.sections:
            nm = s.Name.rstrip(b'\x00').decode('latin1')
            self.secs.append((nm, s.VirtualAddress, s.Misc_VirtualSize, s.PointerToRawData, s.SizeOfRawData))
        self.text = next(x for x in self.secs if x[0]=='.text')
        self.md = Cs(CS_ARCH_X86, CS_MODE_64); self.md.detail = True
        self._pdata=None
    def off(self, rva):
        for nm,va,vs,pr,sr in self.secs:
            if va <= rva < va+max(vs,sr):
                d = rva-va
                if d < sr: return pr+d
        return None
    def read(self, rva, n):
        o=self.off(rva); return None if o is None else self.data[o:o+n]
    def text_blob(self):
        nm,va,vs,pr,sr = self.text; return self.data[pr:pr+sr], va
    def pdata(self):
        if self._pdata is not None: return self._pdata
        sec = next((x for x in self.secs if x[0]=='.pdata'), None); out=[]
        if sec:
            nm,va,vs,pr,sr = sec; blob=self.data[pr:pr+min(vs,sr)]
            for i in range(0,len(blob)-11,12):
                s,e,u = struct.unpack_from('<III', blob, i)
                if s==0 and e==0: continue
                out.append((s,e))
            out.sort()
        self._pdata=out; return out
    def func_of(self, rva):
        pd=self.pdata(); starts=[s for s,e in pd]
        i=bisect.bisect_right(starts,rva)-1
        if i>=0 and pd[i][0]<=rva<pd[i][1]: return pd[i]
        return None
    def dis(self, rva, n=30, maxb=256):
        b=self.read(rva,maxb); out=[]
        if not b: return out
        for ins in self.md.disasm(b, BASE+rva):
            out.append(ins)
            if len(out)>=n: break
        return out

O = Exe(E54P); N = Exe(E55P)

def make_pattern(E, rva, nbytes):
    b = E.read(rva, nbytes+16)
    pat=bytearray(); mask=bytearray()
    for ins in E.md.disasm(b, BASE+rva):
        if len(pat)>=nbytes: break
        wild = any(op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP for op in ins.operands)
        if (ins.group(X86_GRP_JUMP) or ins.group(X86_GRP_CALL)) and any(op.type==X86_OP_IMM for op in ins.operands): wild=True
        for bb in ins.bytes:
            pat.append(bb); mask.append(0 if wild else 1)
    return bytes(pat), bytes(mask)

def find(E, pat, mask, limit=40):
    text, va = E.text_blob()
    pre=bytearray()
    for b,m in zip(pat,mask):
        if m: pre.append(b)
        else: break
    if len(pre)<4:
        best=(0,0); cur=0
        for i,m in enumerate(mask):
            cur = cur+1 if m else 0
            if cur>best[0]: best=(cur,i-cur+1)
        ln,st=best; pre=bytes(pat[st:st+ln]); off=st
    else:
        pre=bytes(pre); off=0
    hits=[]; s=0
    while True:
        i=text.find(pre,s)
        if i<0: break
        s=i+1; j=i-off
        if j<0: continue
        seg=text[j:j+len(pat)]
        if len(seg)==len(pat) and all(mask[k]==0 or seg[k]==pat[k] for k in range(len(pat))):
            hits.append(va+j)
            if len(hits)>limit: break
    return hits

# 0.5.4 container 시작(현행 소스 기준) — 0.5.5 재핀 대상
CONT54 = {
 'AI1':    0x149e380,  # AI 밴 스코어러1
 'AI2':    0x14a1e60,  # AI 밴 스코어러2
 'AITURN': 0x211dd40,  # 서버 AI턴
 'DRAIN':  0x1e19640,  # 드레인 update
 'MATCHUI':0x237c030,  # match_ui (HL)
 'AI6a':   0x215e050,
 'AI6b':   0x215f680,
 'AI6c':   0x2160680,
 'AI6d':   0x2161200,
}

if __name__=='__main__':
    print("0.5.4 .text vsz", O.text[2], "0.5.5 .text vsz", N.text[2])
    print("0.5.4 pdata", len(O.pdata()), "0.5.5 pdata", len(N.pdata()))
    # 컨테이너 마스크시그(48B)로 0.5.5 재핀
    for k,rva in CONT54.items():
        f=O.func_of(rva)
        pat,mask = make_pattern(O, rva, 48)
        hits = find(N, pat, mask, limit=8)
        print(f"{k:8s} 0.5.4 {rva:#x} cont={f and (hex(f[0]),hex(f[1]))} -> 0.5.5 hits={[hex(h) for h in hits]}")
