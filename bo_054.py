# -*- coding: utf-8 -*-
"""tfm2_banpick_order 0.5.3 -> 0.5.4 RVA 재핀 (capstone/pefile 오프라인)."""
import sys, struct, re, bisect, difflib, json
sys.path.insert(0, r'C:\tfm2mods')
import importlib.util
spec = importlib.util.spec_from_file_location("_its", r"C:\tfm2mods\_it_scan.py")
# _it_scan loads 0.5.2/0.5.3 at import (slow but fine); we only need the Exe class
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_REG_RIP, X86_OP_IMM, X86_GRP_JUMP, X86_GRP_CALL

BASE = 0x140000000
E53P = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"
E54P = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe"

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

O = Exe(E53P); N = Exe(E54P)

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
        # fallback: use longest fixed run
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

if __name__=='__main__':
    print("0.5.3 pdata", len(O.pdata()), "0.5.4 pdata", len(N.pdata()))
    sites = {
      'AI_SITE1':0x10a04e2,'AI_JOIN1':0x10a05f0,'AI_SITE2':0x10a3cf8,'AI_JOIN2':0x10a3dc9,
      'AITURN_SITE':0x1828213,'AITURN_JOIN':0x18282fa,'SFX_SITE':0x1c56245,'SFX_END':0x1c56294,
      'HL_site':0x193b434,'HL_join':0x193b570,
      'J_cur':0x1c6605d,'J_cur_join':0x1c66323,'J_next':0x1c66374,'J_next_join':0x1c66434,
      'K_A':0x1c5a0b2,'K_A_join':0x1c5a288,'K_A_done':0x1c5a274,
      'K_C':0x1c5a9b1,'K_C_join':0x1c5aa67,'K_C_done':0x1c5ab4e,
      'K_B':0x1c5a5b9,'K_B_join':0x1c5a90b,
      'L_site':0x1c6fb05,'L_j1':0x1c6fc40,'L_j2':0x1c70508,'L_j3':0x1c6fc3a,
      'M_site':0x1c5aa99,'M_join':0x1c5ab31,
      'AI6_1':0x188e206,'AI6_1j':0x188e6b6,'AI6_2':0x188e403,'AI6_3':0x188f79e,'AI6_3j':0x188f8df,
      'AI6_4':0x18905fc,'AI6_4j':0x189083f,'AI6_5':0x18906e8,'AI6_6':0x1891090,'AI6_6j':0x1891390,
    }
    for k,v in sites.items():
        f=O.func_of(v)
        print(f"{k:12s} {v:#x}  container={f and (hex(f[0]),hex(f[1]))}")
