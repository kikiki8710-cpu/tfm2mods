exec(open('srcmap.py',encoding='utf-8').read())
import pickle,collections
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
s57,p57=pickle.load(open('_s57c.pkl','rb')); s58,p58=pickle.load(open('_s58c.pkl','rb'))
def owners(img,strs,p2s):
    out=collections.defaultdict(list)
    for (b,e) in img._fs:
        if e<=b or e-b>0x20000: continue
        code=img.code(b,e-b)
        if not code: continue
        hits=collections.Counter()
        for ins in md.disasm(code,BASE+b):
            for op in ins.operands:
                if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                    t=ins.address+ins.size+op.mem.disp-BASE
                    if t in strs: hits[strs[t]]+=1
                    elif t in p2s: hits[p2s[t]]+=1
        if hits:
            top=hits.most_common(1)[0][0]
            if 'plan_legacy' in top: out[top].append((b,e-b))
    return out
import os
if os.path.exists('_own57.pkl'): o7=pickle.load(open('_own57.pkl','rb'))
else: o7=owners(A,s57,p57); pickle.dump(o7,open('_own57.pkl','wb'))
if os.path.exists('_own58.pkl'): o8=pickle.load(open('_own58.pkl','rb'))
else: o8=owners(B,s58,p58); pickle.dump(o8,open('_own58.pkl','wb'))
keys=sorted(set(o7)|set(o8))
print("%-58s %-22s %-22s"%("module","0.5.7 (n, totalsz)","0.5.8 (n, totalsz)"))
for k in keys:
    a=o7.get(k,[]); b=o8.get(k,[])
    ta=sum(x[1] for x in a); tb=sum(x[1] for x in b)
    flag=""
    if len(a)!=len(b) or abs(ta-tb)>max(64,ta*0.02): flag="  <== 변화"
    print("%-58s (%2d, %6s)        (%2d, %6s)%s"%(k.replace('game-ai'+chr(92)+'src'+chr(92),''),len(a),hex(ta),len(b),hex(tb),flag))
