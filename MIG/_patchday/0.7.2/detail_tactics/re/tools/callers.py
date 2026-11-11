# python -X utf8 callers.py <old|new> <target_rva>...   : E8 rel32 direct call sites (text section)
import sys, struct
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
P={'old':r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe','new':r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'}
img=A.Img(P[sys.argv[1]])
for nm,va,sz,pr in img.secs:
    if nm=='.text':
        blob=img.data[pr:pr+sz]; tva=va
for t in sys.argv[2:]:
    T=int(t,16); res=[]
    i=blob.find(b'\xe8')
    while i!=-1 and i+5<=len(blob):
        r=struct.unpack_from('<i',blob,i+1)[0]
        if tva+i+5+r==T:
            f=img.owner(tva+i)
            res.append('%x(fn %s) ret=%x'%(tva+i,hex(f[0]) if f else '?',tva+i+5))
        i=blob.find(b'\xe8',i+1)
    print(t,len(res)); [print('   ',x) for x in res]