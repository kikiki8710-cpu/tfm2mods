import json,pickle,bisect,struct,sys,re,capstone
ROOT='C:/tfm2mods'
def load(p):
    d=open(p,'rb').read(); pe=struct.unpack_from('<I',d,0x3c)[0]; n=struct.unpack_from('<H',d,pe+6)[0]; oh=struct.unpack_from('<H',d,pe+20)[0]
    secs=[]; o=pe+24+oh
    for i in range(n):
        name=d[o:o+8].rstrip(b'\0'); vs,va,rs,ro=struct.unpack_from('<IIII',d,o+8); secs.append((name,va,vs,ro,rs)); o+=40
    return d,secs
def rd(img,rva,n):
    d,secs=img
    for name,va,vs,ro,rs in secs:
        if va<=rva<va+vs: return d[ro+rva-va:ro+rva-va+n]
def L(p):
    P=pickle.load(open(p,'rb'))['idx']; return {(int(k,16) if isinstance(k,str) else k):v for k,v in P.items()}
O=load(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.6.0\TeamfightManager2.exe"); N=load(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.6.1\TeamfightManager2.exe")
PO=L(ROOT+'/_fnidx_060.pkl'); PN=L(ROOT+'/_fnidx_061.pkl'); SO=sorted(PO); SN=sorted(PN)
CO=pickle.load(open(ROOT+'/_cg_060.pkl','rb')); CN=pickle.load(open(ROOT+'/_cg_061.pkl','rb'))
cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
def owner(S,r): i=bisect.bisect_right(S,r)-1; return S[i] if i>=0 else None
def fninfo(P,C,r): f=P.get(r); return f"sz{f['size']} c{len(C['caller'].get(r,[]))}/{len(C['callee'].get(r,[]))}" if f else '?'
def norm(img,P,fn):
    b=rd(img,fn,int(P[fn]['size'])); out=[]
    for i in cs.disasm(b,0x140000000+fn):
        s=f"{i.mnemonic} {i.op_str}"; s=re.sub(r'0x14[0-9a-f]{7}','ADDR',s); s=re.sub(r'rip [+-] 0x[0-9a-f]+','rip',s); out.append(s)
    return out
def dis1(img,r,n=1):
    b=rd(img,r,16); out=[]
    for i in cs.disasm(b,0x140000000+r):
        out.append(f"{i.mnemonic} {i.op_str}");
        if len(out)>=n: break
    return ' ; '.join(out)
m=json.load(open(ROOT+'/MIG/repin_map_061_6.aligned.json',encoding='utf-8'))
for mod in sys.argv[1:]:
    print('=====',mod)
    for old,e in m[mod].items():
        o=int(old,16); n=int(e['new'],16) if e.get('new') else None; na=e.get('new_aligned')
        oo=owner(SO,o); no=owner(SN,n) if n else None
        so=PO[oo]; sn=PN[no] if no else None
        same = (sn and so['skel']==sn['skel'])
        ob=rd(O,o,12).hex(); nb=rd(N,n,12).hex() if n else '-'
        sa=e.get('site_align','')
        print(f"{e['name'][:26]:26} {old:>10}->{e.get('new') or 'FAIL':>10} {e['kind'][:20]:20} own {oo:#x}+{o-oo:#x}->{(no or 0):#x}+{(n-no if n else 0):#x} {fninfo(PO,CO,oo)}->{fninfo(PN,CN,no) if no else '?'} skel={'S' if same else 'd'} bytes={'S' if ob==nb else 'D'} {('align='+str(na)+' '+str(sa)) if na else ''}")
        if ob!=nb and n: print(f"      old {ob} `{dis1(O,o)}`\n      new {nb} `{dis1(N,n)}`")
