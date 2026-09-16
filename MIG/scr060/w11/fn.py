import sys,struct; sys.path.insert(0,r'C:\tfm2mods\MIG\scr060\w11')
from pe import *
pe=PE(sys.argv[1]); a=int(sys.argv[2],16)-base
# find pdata entry containing a
for n,va,vs,ro,rs in pe.secs:
    if n=='.pdata':
        d=pe.d[ro:ro+rs]
        lo,hi=0,rs//12
        for i in range(hi):
            s,e,u=struct.unpack_from('<III',d,i*12)
            if s<=a<e: print('func %x-%x size %x'%(s,e,e-s)); break
