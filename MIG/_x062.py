# 0.6.1 -> 0.6.2 마이그 보조: 구/신 exe 로더 + 디스어셈 헬퍼 (import 용)
import pickle,struct,re,capstone,bisect,collections
ROOT='C:/tfm2mods'
def load(p):
    d=open(p,'rb').read(); pe=struct.unpack_from('<I',d,0x3c)[0]; ns=struct.unpack_from('<H',d,pe+6)[0]; oh=struct.unpack_from('<H',d,pe+20)[0]
    secs=[]; o=pe+24+oh
    for i in range(ns):
        name=d[o:o+8].rstrip(b'\0'); vs,va,rs,ro=struct.unpack_from('<IIII',d,o+8); secs.append((name,va,vs,ro)); o+=40
    return d,secs
def rd(img,rva,k):
    d,secs=img
    for name,va,vs,ro in secs:
        if va<=rva<va+vs: return d[ro+rva-va:ro+rva-va+k]
O=load(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.6.1\TeamfightManager2.exe")
N=load(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.6.2\TeamfightManager2.exe")
PO=pickle.load(open(ROOT+'/_fnidx_061.pkl','rb'))['idx']; PN=pickle.load(open(ROOT+'/_fnidx_062.pkl','rb'))['idx']
CO=pickle.load(open(ROOT+'/_cg_061.pkl','rb')); CN=pickle.load(open(ROOT+'/_cg_062.pkl','rb'))
SO=sorted(PO); SN=sorted(PN)
cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
def owner(S,r): i=bisect.bisect_right(S,r)-1; return S[i] if i>=0 else None
def insns(img,start,size):
    return list(cs.disasm(rd(img,start,size),0x140000000+start))
def dis(img,a,n=20,before=0):
    out=[]
    for i in cs.disasm(rd(img,a-before,before+n*15),0x140000000+a-before):
        out.append('%x: %-8s %s'%(i.address-0x140000000,i.mnemonic,i.op_str))
        if len(out)>=n+before//3: break
    return '\n'.join(out)
def calls_in(img,f,size):
    r=[]
    for i in insns(img,f,size):
        if i.mnemonic=='call' and i.op_str.startswith('0x'): r.append((i.address-0x140000000,int(i.op_str,16)-0x140000000))
    return r
