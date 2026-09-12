exec(open('srcmap.py',encoding='utf-8').read())
import struct,pickle,collections
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
# global call graph for 0.5.7
def callgraph(img):
    callers=collections.defaultdict(set); callees=collections.defaultdict(set)
    d=img.data
    for nm,va,sz,pr in img.secs:
        if nm!='.text': continue
        i=0
        while i<sz-5:
            if d[pr+i]==0xe8:
                rel=struct.unpack_from('<i',d,pr+i+1)[0]
                t=va+i+5+rel
                fr=img.frange(va+i)
                if fr and img.frange(t) and img.frange(t)[0]==t:
                    callers[t].add(fr[0]); callees[fr[0]].add(t)
            i+=1
    return callers,callees
import os,pickle
if os.path.exists('_cg57.pkl'): callers,callees=pickle.load(open('_cg57.pkl','rb'))
else:
    callers,callees=callgraph(A); pickle.dump((callers,callees),open('_cg57.pkl','wb'))
print("cg done", len(callees))
h57=pickle.load(open('_nh57.pkl','rb')); h58=pickle.load(open('_nh58.pkl','rb'))
set58={v[0] for v in h58.values()}
for dead in (0xde48f0,0xde2470):
    print("== callees of %s (0.5.7 %s)"%(hex(dead),'line_attack' if dead==0xde48f0 else 'line_total'))
    for c in sorted(callees.get(dead,())):
        cs=callers.get(c,set())
        only = (cs=={dead})
        gone = (h57[c][0] not in set58) if c in h57 else None
        if only:
            print("   ONLY-CALLER  %s sz=%s  0.5.8에 동일구조 존재=%s"%(hex(c),hex(A.frange(c)[1]-c),not gone))
# also which functions call both dead handlers only
print("\n== 0.5.8 counterpart of 0xe71820 (direct [+0x768]=7 writer)")
h=h57[0xe71820][0]
cand=[r for r,v in h58.items() if v[0]==h]
print("   exact hash match:",[hex(x) for x in cand])
