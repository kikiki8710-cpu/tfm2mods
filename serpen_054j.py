# -*- coding: utf-8 -*-
import sys, io
sys.path.insert(0, r"C:\tfm2mods")
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from s54lib import O, Nw
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)
o=O(); n=Nw(); o.sites(); n.sites()
for tag, img, tgt in (("053 LAUNCHER", o, 0xeb8810), ("054 LAUNCHER", n, 0x13b53d0)):
    cs = img.by_tgt.get(tgt, [])
    print(f"[{tag}] {tgt:#x} 콜사이트 {len(cs)}")
    for s in cs:
        w = img.owner(s)
        print(f"   site={s:#x} retaddr={s+5:#x} in {w:#x}(size={img.fn[w]['size']}) +{s-w:#x}")
print()
print("== UIALLOC 본문 ==")
for tag, img, r in (("053", o, 0x28f7df0), ("054", n, 0x29bb920)):
    print(f" [{tag}] {r:#x}")
    for i in md.disasm(img.body(r), r):
        print(f"   {i.address:#x}: {i.mnemonic} {i.op_str}")
