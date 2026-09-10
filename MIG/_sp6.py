exec(open('srcmap.py',encoding='utf-8').read())
import struct
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
d=A.data
# find JT tables containing 0xde48f0 / 0xde2470
for TGT in (0xde48f0,0xde2470):
    hits=[]
    for nm,va,sz,pr in A.secs:
        if nm not in ('.rdata','.data','.text'): continue
        for i in range(0,sz-4,4):
            v=struct.unpack_from('<i',d,pr+i)[0]
            T=(TGT-v)&0xffffffff
            p=va+i
            if 0<=p-T<=4*24 and (p-T)%4==0:
                hits.append((hex(p),hex(T),(p-T)//4))
    print(hex(TGT),hits[:20])
