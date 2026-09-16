import sys,re; sys.path.insert(0,r'C:\tfm2mods\MIG\scr060\w11')
from pe import *
pe=PE(sys.argv[1]); s=int(sys.argv[2],16); e=int(sys.argv[3],16)
for ins in pe.disasm(s,e):
    if ins.mnemonic=='nop': continue
    ops=ins.op_str
    ops=re.sub(r'0x14[0-9a-f]{7}','ADDR',ops)
    ops=re.sub(r'rip \+ 0x[0-9a-f]+','rip+X',ops)
    ops=re.sub(r'\[(rbp|rsp) [+-] 0x[0-9a-f]+\]','[STK]',ops)
    print(ins.mnemonic,ops)
