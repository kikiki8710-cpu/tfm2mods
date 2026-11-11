# usage: pro.py <fn_or_any_rva>... : owner 프롤로그(push 순서·sub) + rbp 덮어쓰기 명령 목록
import sys
sys.path.insert(0, r'F:\claude\tfm2mods\MIG'); import aidiff as A
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
N=A.Img(r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'); md=Cs(CS_ARCH_X86,CS_MODE_64)
for a in sys.argv[1:]:
    f=N.owner(int(a,16)); ins=list(md.disasm(N.code(f[0],f[1]-f[0]),A.BASE+f[0]))
    pro=[]
    for i in ins[:14]:
        pro.append('%s %s'%(i.mnemonic,i.op_str))
        if i.mnemonic=='sub' and i.op_str.startswith('rsp'): break
    rbpw=['%x %s %s'%(i.address-A.BASE,i.mnemonic,i.op_str) for i in ins[14:] if (i.op_str.startswith('rbp,') or i.op_str.startswith('ebp,') or i.op_str.startswith('bpl,')) and i.mnemonic not in ('cmp','test')]
    print('%s owner %x-%x | %s'%(a,f[0],f[1],' ; '.join(pro)))
    print('   rbp writes after prologue:',rbpw[:8], '(+%d)'%max(0,len(rbpw)-8))
