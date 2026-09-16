import sys,struct; sys.path.insert(0,r'C:\tfm2mods\MIG\scr060\w11')
from pe import *
pe=PE(sys.argv[1]); tgt=int(sys.argv[2],16)
for n,va,vs,ro,rs in pe.secs:
    if n!='.text': continue
    d=pe.d[ro:ro+rs]
    i=0
    while True:
        i=d.find(b'\xe8',i)
        if i<0: break
        rel=struct.unpack_from('<i',d,i+1)[0]
        if (va+i+5+rel)&0xffffffff==(tgt-base)&0xffffffff:
            print('call at %x'%(va+i))
        i+=1
    # also lea rip
    i=0
    while True:
        i=d.find(b'\x48\x8d',i)
        if i<0: break
        if d[i+2]&0xc7==0x05:
            rel=struct.unpack_from('<i',d,i+3)[0]
            if va+i+7+rel==tgt-base: print('lea at %x'%(va+i))
        i+=1
