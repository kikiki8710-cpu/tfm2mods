exec(open('srcmap.py',encoding='utf-8').read())
import pickle,collections,struct
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
o7=pickle.load(open('_own57.pkl','rb'))
callers,callees=pickle.load(open('_cg57.pkl','rb'))
for k in ('game-ai'+chr(92)+'src'+chr(92)+'plan_legacy'+chr(92)+'sub_plan'+chr(92)+'line_attack.rs',
          'game-ai'+chr(92)+'src'+chr(92)+'plan_legacy'+chr(92)+'sub_plan'+chr(92)+'line_total.rs'):
    print("==",k)
    for b,sz in sorted(o7[k]):
        cs=sorted(callers.get(b,()))
        print("   %s sz=%-7s callers=%s"%(hex(b),hex(sz),[hex(x) for x in cs] or "없음(간접/JT?)"))
