# -*- coding: utf-8 -*-
import io,sys,re,difflib
sys.path.insert(0,r'C:\tfm2mods\v54')
from s import E
from pe2 import BASE
def norm(ver,s,e_):
    ex=E(ver); out=[]
    for i in ex.dis(s,e_-s):
        op=i.op_str
        op=re.sub(r'rip \+ 0x[0-9a-f]+','rip+X',op)
        op=re.sub(r'\b0x14[0-9a-f]{5,6}\b','TGT',op)
        out.append('%s %s'%(i.mnemonic,op))
    return out
if __name__=='__main__':
    a=norm('053',int(sys.argv[1],16),int(sys.argv[2],16))
    b=norm('054',int(sys.argv[3],16),int(sys.argv[4],16))
    d=list(difflib.unified_diff(a,b,'053','054',lineterm='',n=3))
    io.open(r'C:\tfm2mods\v54\_diff.txt','w',encoding='utf-8').write('\n'.join(d))
    print('053 %d insn / 054 %d insn / diff lines %d'%(len(a),len(b),len(d)))
