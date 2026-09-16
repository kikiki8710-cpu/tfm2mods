import sys; sys.path.insert(0,r'C:\tfm2mods\MIG\scr060\w11')
from pe import *
pe=PE(sys.argv[1])
for a in sys.argv[2:]:
    a=int(a,16)
    print(hex(a), repr(pe.bytes(a,120).split(b'\0')[0][:120]))
