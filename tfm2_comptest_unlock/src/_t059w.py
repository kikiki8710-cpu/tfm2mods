# -*- coding: utf-8 -*-
import io,sys,difflib
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
exec(open(r"C:\tfm2mods\_mig057.py",encoding="utf-8").read())
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md=Cs(CS_ARCH_X86,CS_MODE_64)
OO,NO=0x888cc0,0xb62c80
so,sn=O.fn[OO]["size"],N.fn[NO]["size"]
def dis(img,st,sz):
    out=[]
    for i in md.disasm(img.read(st,sz),st): out.append((i.address,i.mnemonic,i.op_str,i.bytes.hex()))
    return out
A,B=dis(O,OO,so),dis(N,NO,sn)
print(f"구 {len(A)}명령 / 신 {len(B)}명령")
ka=[a[1]+" "+ (a[2].split(",")[0] if a[2] else "") for a in A]
kb=[b[1]+" "+ (b[2].split(",")[0] if b[2] else "") for b in B]
sm=difflib.SequenceMatcher(None,ka,kb,autojunk=False)
def mapidx(i):
    for t,i1,j1,n in sm.get_opcodes():
        if t=="equal" and i1<=i<i1+n: return j1+(i-i1)
    return None
# ① DRIVE 사이트 대응 확인
tgt=0x888d20
ia=[k for k,a in enumerate(A) if a[0]==tgt][0]
jb=mapidx(ia)
print(f"\n[DRIVE 사이트] 구 idx{ia} 0x{tgt:x} → 신 idx{jb} 0x{B[jb][0]:x}" if jb is not None else "  대응 없음")
if jb is not None:
    for d in range(0,4):
        print(f"   구 {A[ia+d][1]} {A[ia+d][2]}   |   신 {B[jb+d][1]} {B[jb+d][2]}")
# ② [rbp+0x17470] 참조 위치
print("\n[p4 로드 0x17470 참조]")
for k,a in enumerate(A):
    if "0x17470" in a[2]:
        j=mapidx(k)
        nn=f"신 idx{j} 0x{B[j][0]:x}: {B[j][1]} {B[j][2]}" if j is not None else "★대응 없음(변경 구간)"
        print(f"  구 0x{a[0]:x}: {a[1]} {a[2]}  →  {nn}")
