exec(open('srcmap.py',encoding='utf-8').read())
import pickle,difflib,collections
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
callers,callees=pickle.load(open('_cg57.pkl','rb'))
h57=pickle.load(open('_nh57.pkl','rb')); h58=pickle.load(open('_nh58.pkl','rb'))
byh58=collections.defaultdict(list)
for r,v in h58.items(): byh58[v[0]].append(r)
for dead,lbl in ((0xde48f0,'line_attack'),(0xde2470,'line_total')):
    cs=sorted(callees.get(dead,()))
    print("== %s %s : callees=%d"%(lbl,hex(dead),len(cs)))
    for c in cs:
        n=len(callers.get(c,()))
        alive = h57[c][0] in byh58 if c in h57 else None
        print("   %s sz=%-7s callers=%-3d 0.5.8동일구조=%s"%(hex(c),hex(A.frange(c)[1]-c) if A.frange(c) else '?',n,alive))
# counterpart of 0xe71820 by token similarity, limited candidates by ninsn
def toks(img,r):
    fr=img.frange(r)
    if not fr: return None
    b,e=fr; o=[]
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        o.append(ins.mnemonic+" "+re.sub(r'\b0x[0-9a-f]+\b','I',re.sub(r'rip [-+] 0x[0-9a-f]+','rip+X',ins.op_str)))
    return o
t7=toks(A,0xe71820); print("\n0xe71820 ins=%d"%len(t7))
best=[]
for r,(h,n,s) in h58.items():
    if abs(n-len(t7))>len(t7)*0.15: continue
    t8=toks(B,r)
    if not t8: continue
    q=difflib.SequenceMatcher(None,t7,t8,autojunk=False)
    if q.quick_ratio()>0.85:
        best.append((q.ratio(),r))
best.sort(reverse=True)
print("best:",[("%#x"%r,round(x,3)) for x,r in best[:3]])
if best:
    r=best[0][1]; b,e=B.frange(r)
    for ins in md.disasm(B.code(b,e-b),BASE+b):
        for op in ins.operands:
            if op.type==X86_OP_MEM and op.mem.disp==0x768:
                print("   0.5.8 %08x %s %s"%(ins.address-BASE,ins.mnemonic,ins.op_str))
