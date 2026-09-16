import sys,struct; sys.path.insert(0,r'C:\tfm2mods\MIG\scr060\w11')
from pe import *
pe=PE(sys.argv[1]); t=int(sys.argv[2],16); n=int(sys.argv[3])
for i in range(n):
    off=struct.unpack_from('<i',pe.d,pe.rva2off(t-base)+i*4)[0]
    print(i,'->%x'%(t+off-base))
