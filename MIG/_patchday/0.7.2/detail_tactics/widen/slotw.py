# usage: slotw.py <fn_rva>:<disp> ... : 함수 안 [rbp+disp] 쓰기/읽기 횟수와 쓰기 명령
import sys
sys.path.insert(0, r'F:\claude\tfm2mods\MIG'); import aidiff as A
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
N=A.Img(r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'); md=Cs(CS_ARCH_X86,CS_MODE_64)
for a in sys.argv[1:]:
    f,d=[int(x,16) for x in a.split(':')]; f=N.owner(f)
    tag='rbp + 0x%x]'%d; w=[];r=0
    for i in md.disasm(N.code(f[0],f[1]-f[0]),A.BASE+f[0]):
        if tag in i.op_str:
            if i.op_str.split(',')[0].strip().endswith(tag) and i.mnemonic.startswith('mov'): w.append('%x %s %s'%(i.address-A.BASE,i.mnemonic,i.op_str))
            else: r+=1
    print('%s owner %x: writes %d %s | reads %d'%(a,f[0],len(w),w[:4],r))
