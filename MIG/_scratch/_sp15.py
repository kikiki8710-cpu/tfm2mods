exec(open('srcmap.py',encoding='utf-8').read())
import pickle,collections
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
un57,un58=pickle.load(open('_unm.pkl','rb'))
print("unmatched",len(un57))
res=[]
for b in un57:
    fr=A.frange(b)
    if not fr: continue
    e=fr[1]
    if e-b>0x20000: continue
    vals=set(); q=set()
    for ins in md.disasm(A.code(b,e-b),BASE+b):
        ops=ins.operands
        if ins.mnemonic in ('mov',) and len(ops)==2 and ops[1].type==X86_OP_IMM and 2<=ops[1].imm<=20:
            if ops[0].type==X86_OP_MEM and ops[0].size==8: q.add(ops[1].imm)
            elif ops[0].type==X86_OP_REG: vals.add(ops[1].imm)
    if 3 in q and 5 in q: res.append((b,e-b,sorted(q),'Q'))
    elif ({3,5}<=(q|vals)) and len((q|vals)&{2,3,4,5,6,7})>=4: res.append((b,e-b,sorted(q|vals),'M'))
print("cand",len(res))
for b,s,v,k in sorted(res,key=lambda x:-x[1])[:40]:
    print("  %s sz=%s %s %s"%(hex(b),hex(s),k,v))
