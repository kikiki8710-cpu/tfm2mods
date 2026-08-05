# -*- coding: utf-8 -*-
"""매칭 후보 검증: 니모닉 시퀀스 유사도 + 콜러수 + 크기."""
import sys, re, difflib
sys.path.insert(0, r'C:\tfm2mods')
from _it54 import O, N, BASE
from _it54g import GO, GN

GPR = (r'\b(?:r(?:ax|bx|cx|dx|si|di|8|9|1[0-5])'
       r'|e(?:ax|bx|cx|dx|si|di)|r(?:8|9|1[0-5])d'
       r'|[abcd][lh]|sil|dil|r(?:8|9|1[0-5])[bw])\b')
def norm(mn, op, regs=True):
    s = op
    s = re.sub(r'0x[0-9a-f]{6,}', 'ADDR', s)
    s = re.sub(r'\[rip \+ [^\]]+\]', '[rip+X]', s)
    if regs:
        s = re.sub(GPR, 'R', s)
        s = re.sub(r'\[rsp \+ 0x[0-9a-f]+\]', '[rsp+X]', s)
    return f"{mn} {s}".strip()

def body(E, rva, regs=True):
    f = E.func_of(rva)
    if not f: return None, []
    b = E.read(f[0], f[1]-f[0])
    out = []
    for ins in E.md.disasm(b, BASE + f[0]):
        out.append(norm(ins.mnemonic, ins.op_str, regs))
    return f, out

def cmp2(name, orva, nrva, regs=True, show=0):
    fo, a = body(O, orva, regs); fn, b = body(N, nrva, regs)
    if fo is None or fn is None:
        print(f"{name}: pdata MISS  o={fo} n={fn}"); return 0
    r = difflib.SequenceMatcher(None, a, b, autojunk=False).ratio()
    co = len(GO['callers'].get(fo[0], [])); cn = len(GN['callers'].get(fn[0], []))
    print(f"{name:28s} 0.5.3 {orva:#x}(sz {fo[1]-fo[0]:#x},ins {len(a)},callers {co}) -> "
          f"0.5.4 {nrva:#x}(sz {fn[1]-fn[0]:#x},ins {len(b)},callers {cn})  sim={r:.4f}")
    print(f"   프롤로그 o={O.hexat(fo[0],24)}")
    print(f"           n={N.hexat(fn[0],24)}")
    if show:
        sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
        k=0
        for tag,i1,i2,j1,j2 in sm.get_opcodes():
            if tag=='equal': continue
            k+=1
            if k>show: break
            print(f"   [{tag}] o+{i1}..{i2} n+{j1}..{j2}")
            for x in a[i1:min(i2,i1+6)]: print("     - ",x)
            for x in b[j1:min(j2,j1+6)]: print("     + ",x)
    return r
