import sys; sys.path.insert(0,r'C:\tfm2mods\MIG\scr060\w11')
from pe import *
pe=PE(sys.argv[1]); a=int(sys.argv[2],16); n=int(sys.argv[3]) if len(sys.argv)>3 else 16
for i in range(n):
    v=pe.q(a+i*8); print(hex(a+i*8),'+%x'%(i*8),hex(v),('RVA=%x'%(v-base)) if base<=v<base+0x5000000 else '')
