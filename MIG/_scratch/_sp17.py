exec(open('srcmap.py',encoding='utf-8').read())
import struct,collections
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
# callers of 0xcc21e0 (store subplan into entity+0x768)
def xref(img,tgt):
    d=img.data; out=[]
    for nm,va,sz,pr in img.secs:
        if nm!='.text': continue
        for i in range(sz-5):
            b=d[pr+i]
            if b in (0xe8,0xe9):
                rel=struct.unpack_from('<i',d,pr+i+1)[0]
                if va+i+5+rel==tgt: out.append((va+i,'call' if b==0xe8 else 'jmp'))
    return out
for t in (0xcc21e0,):
    r=xref(A,t)
    print(hex(t),"xrefs",len(r))
    c=collections.Counter()
    for x,y in r:
        fr=A.frange(x); c[fr[0] if fr else 0]+=1
    for f,n in c.most_common(): print("   fn %s sz=%s n=%d"%(hex(f),hex(A.frange(f)[1]-f),n))
