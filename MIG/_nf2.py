exec(open('srcmap.py',encoding='utf-8').read())
import pickle,collections,struct,bisect
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
def load(p,c):
    img=Img(p); img.prep()
    strs,p2s=pickle.load(open(c,'rb'))
    return img,strs,p2s
def leascan(pack, modnames):
    img,strs,p2s=pack
    T=set()
    for r,t in strs.items():
        if t in modnames: T.add(r)
    for r,t in p2s.items():
        if t in modnames: T.add(r)
    # scan .text
    for nm,va,sz,pr in img.secs:
        if nm=='.text': tva,tsz,tpr=va,sz,pr
    d=img.data; out=collections.defaultdict(collections.Counter)
    i=tpr; end=tpr+tsz
    starts=img._starts; fs=img._fs
    while i<end-8:
        b0=d[i]
        if (b0==0x48 or b0==0x4c) and d[i+1]==0x8d and (d[i+2]&0xC7)==0x05:
            rel=struct.unpack_from('<i',d,i+3)[0]
            rva=tva+(i-tpr)
            tgt=rva+7+rel
            if tgt in T:
                k=bisect.bisect_right(starts,rva)-1
                if k>=0 and fs[k][0]<=rva<fs[k][1]:
                    out[fs[k]][strs.get(tgt) or p2s.get(tgt)]+=1
            i+=1
        else: i+=1
    return out
A=load(E57,'_s57c.pkl'); B=load(E58,'_s58c.pkl')
MOD={'game-ai\src\lib.rs'}
for tag,pack in (('57',A),('58',B)):
    o=leascan(pack,MOD)
    print("==",tag,len(o))
    for (b,e),c in sorted(o.items(), key=lambda x:-(x[0][1]-x[0][0])):
        print("  %08x sz=%-8s %s"%(b,hex(e-b),c.most_common(2)))
