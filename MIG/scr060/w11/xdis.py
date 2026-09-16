import sys; sys.path.insert(0,r'C:\tfm2mods\MIG\scr060\w11')
from pe import *
pe=PE(sys.argv[1]); s=int(sys.argv[2],16); e=int(sys.argv[3],16)
for ins in pe.disasm(s,e):
    ex=''
    if ins.mnemonic in('call','jmp') and ins.op_str.startswith('0x'):
        ex=' ; ->%x'%(int(ins.op_str,16)-base)
    for op in ins.operands:
        if op.type==3 and op.mem.base==41:  # rip
            t=ins.address+ins.size+op.mem.disp
            st=pe.str_at(t)
            ex+=' ; rip=%x'%(t-base)+((' "'+st+'"') if st else '')
    print(("%x: %s %s%s"%(ins.address-base,ins.mnemonic,ins.op_str,ex)).encode("ascii","replace").decode())
