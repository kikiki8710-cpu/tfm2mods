# -*- coding: utf-8 -*-
from bo_054 import O,N,BASE
def dis(E,s,e):
    b=E.read(s,e-s); out=[]
    for ins in E.md.disasm(b,BASE+s):
        out.append((ins.address-BASE, ins.mnemonic, ins.op_str, bytes(ins.bytes).hex()))
    return out
def cmp_range(os_,oe,ns_,ne,tag=''):
    a=dis(O,os_,oe); b=dis(N,ns_,ne)
    print(f"--- {tag}: old[{os_:#x},{oe:#x}) {len(a)}ins  new[{ns_:#x},{ne:#x}) {len(b)}ins")
    for i in range(max(len(a),len(b))):
        x=a[i] if i<len(a) else None; y=b[i] if i<len(b) else None
        xs=f"{x[0]:#x} {x[1]} {x[2]}" if x else '-'
        ys=f"{y[0]:#x} {y[1]} {y[2]}" if y else '-'
        mark='' if (x and y and x[1]==y[1] and x[3]==y[3]) else ('  <<<' if (x and y and x[1]==y[1]) else '  <<<DIFF')
        print(f"  {xs:<48} | {ys:<48}{mark}")
