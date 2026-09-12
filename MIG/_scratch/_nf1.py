exec(open('srcmap.py',encoding='utf-8').read())
import pickle,collections,os
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
def load(p,c):
    img=Img(p); img.prep()
    strs,p2s=pickle.load(open(c,'rb'))
    return img,strs,p2s
A=load(E57,'_s57c.pkl'); B=load(E58,'_s58c.pkl')
def modof(pack,rva):
    img,strs,p2s=pack
    fr,h=analyze(img,strs,p2s,rva)
    return fr,h
for lbl,r in [("replan_throttle",0xdf58f0),("plan_driver",0xdf2f00),("df3d70",0xdf3d70),
              ("db0850",0xdb0850),("db0f40",0xdb0f40),("e723c0",0xe723c0),("auction",0xe8b800)]:
    fr,h=modof(A,r)
    print("57 %-16s %s sz=%s  %s"%(lbl,hex(r),hex(fr[1]-fr[0]) if fr else '?', h.most_common(3) if h else ''))
