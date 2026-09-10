exec(open('srcmap.py',encoding='utf-8').read())
import struct
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
def xref(img,tgt):
    d=img.data; out=[]
    for nm,va,sz,pr in img.secs:
        if nm!='.text': continue
        for i in range(sz-5):
            b=d[pr+i]
            if b in (0xe8,0xe9):
                rel=struct.unpack_from('<i',d,pr+i+1)[0]
                if va+i+5+rel==tgt: out.append((va+i,'call' if b==0xe8 else 'jmp'))
        # qword/dword refs
        break
    # data refs (vtable/JT) : qword absolute
    for nm,va,sz,pr in img.secs:
        if nm not in ('.rdata','.data'): continue
        for i in range(0,sz-8,8):
            if struct.unpack_from('<Q',d,pr+i)[0]==BASE+tgt: out.append((va+i,'qword'))
    return out
A=Img(E57);A.prep()
for t in (0xde48f0,0xde2470):
    r=xref(A,t)
    print(hex(t),[(hex(x),y) for x,y in r])
    for x,y in r:
        if y in ('call','jmp'):
            fr=A.frange(x); print("   caller fn",hex(fr[0]),"sz",hex(fr[1]-fr[0]))
