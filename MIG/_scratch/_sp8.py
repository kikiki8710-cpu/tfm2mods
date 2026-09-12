exec(open('srcmap.py',encoding='utf-8').read())
import struct
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
def xref(img,tgt):
    d=img.data; out=[]
    for nm,va,sz,pr in img.secs:
        if nm=='.text':
            for i in range(sz-5):
                b=d[pr+i]
                if b in (0xe8,0xe9):
                    rel=struct.unpack_from('<i',d,pr+i+1)[0]
                    if va+i+5+rel==tgt: out.append((va+i,'call' if b==0xe8 else 'jmp'))
        elif nm in ('.rdata','.data'):
            for i in range(0,sz-8,8):
                if struct.unpack_from('<Q',d,pr+i)[0]==BASE+tgt: out.append((va+i,'qword'))
        # lea rip-rel refs
    return out
A=Img(E57);A.prep(); B=Img(E58);B.prep()
for tag,img,fns in (("0.5.7",A,[0xcbf340,0xdb2760]),("0.5.8",B,[0xe35bd0,0xcaf9f0])):
    for f in fns:
        r=xref(img,f)
        print(tag,"xrefs to",hex(f))
        for x,y in r:
            fr=img.frange(x)
            print("    %s @%s  in fn %s sz=%s"%(y,hex(x),hex(fr[0]) if fr else "?",hex(fr[1]-fr[0]) if fr else "?"))
