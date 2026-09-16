import sys; sys.path.insert(0,r'C:\tfm2mods\MIG\scr060\w11')
from pe import *
pe=PE(sys.argv[1]); pat=sys.argv[2].encode()
i=0
while True:
    i=pe.d.find(pat,i)
    if i<0: break
    for n,va,vs,ro,rs in pe.secs:
        if ro<=i<ro+rs:
            print(hex(va+i-ro+base), repr(pe.d[max(i-60,0):i+200]))
    i+=1
