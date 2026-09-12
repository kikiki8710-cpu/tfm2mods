exec(open('srcmap.py',encoding='utf-8').read())
import pickle
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
callers,callees=pickle.load(open('_cg57.pkl','rb'))
for f in (0xcc2340,0xcbf880,0xcc21e0):
    fr=A.frange(f)
    print("fn %s sz=%s callers=%d callees=%d"%(hex(f),hex(fr[1]-f) if fr else '?',len(callers.get(f,())),len(callees.get(f,()))))
    if fr:
        for i,ins in enumerate(md.disasm(A.code(f,min(fr[1]-f,120)),BASE+f)):
            if i>16: break
            print("     %08x %-9s %s"%(ins.address-BASE,ins.mnemonic,ins.op_str))
