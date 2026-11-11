# python -X utf8 fdiff.py <old_start> <old_end> <new_start> <new_end> : opcode-shape diff (regs/disp normalized)
import sys, re, difflib
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
o=A.Img(r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe'); n=A.Img(r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe')
md=Cs(CS_ARCH_X86,CS_MODE_64)
def seq(img,s,e):
    out=[]
    for x in md.disasm(img.code(s,e-s),A.BASE+s):
        t=x.op_str
        t=re.sub(r'rip \+ 0x[0-9a-f]+','rip+X',t)
        if x.mnemonic.startswith('j') or x.mnemonic=='call': t=re.sub(r'0x[0-9a-f]+','T',t)
        t=re.sub(r'\b(r[0-9]+[dwb]?|[re]?[abcd]x|[abcd]l|[re]?[sd]il?|[re]?[sb]pl?)\b','R',t)
        t=re.sub(r'(rsp|R) \+ 0x[0-9a-f]+\]','\\1+D]',t)
        out.append((x.address-A.BASE, x.mnemonic+' '+t, x.mnemonic+' '+x.op_str))
    return out
a=seq(o,int(sys.argv[1],16),int(sys.argv[2],16)); b=seq(n,int(sys.argv[3],16),int(sys.argv[4],16))
sm=difflib.SequenceMatcher(None,[x[1] for x in a],[x[1] for x in b],autojunk=False)
for tag,i1,i2,j1,j2 in sm.get_opcodes():
    if tag=='equal': continue
    print('---',tag)
    for x in a[i1:i2]: print('  - %x  %s'%(x[0],x[2]))
    for x in b[j1:j2]: print('  + %x  %s'%(x[0],x[2]))