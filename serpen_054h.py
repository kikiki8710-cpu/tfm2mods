# -*- coding: utf-8 -*-
import sys, io
sys.path.insert(0, r"C:\tfm2mods")
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from s54lib import O, Nw
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)
o=O(); n=Nw()
def show(img, addr, back=0x60, fwd=0x10):
    b=img.read(addr-back, back+fwd)
    for i in md.disasm(b, addr-back):
        mark = " ★" if i.address==addr else ""
        print(f"   {i.address:#x}: {i.mnemonic} {i.op_str}{mark}")
print("== 0.5.3 컨테이너 0xfce740, SERPEN 콜사이트 0xfd5253 ==")
show(o, 0xfd5253)
print("\n== 0.5.4 후보A 콜사이트 0x106089a ==")
show(n, 0x106089a)
print("\n== 0.5.4 후보B 콜사이트 0x1060a50 ==")
show(n, 0x1060a50)
