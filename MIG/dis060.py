# -*- coding: utf-8 -*-
"""dis060.py — Ghidra 없이 capstone 으로 0.6.0(기본)/0.5.8 exe 함수 디스어셈(pdata 경계). 사용: python dis060.py <rva hex> [old] [--max N]"""
import sys, os
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mig060 as M
args = [a for a in sys.argv[1:] if not a.startswith("--")]
mx = 4000
for a in sys.argv[1:]:
    if a.startswith("--max="): mx = int(a[6:])
rva = int(args[0], 16)
tag = "old" if len(args) > 1 and args[1] == "old" else "new"
ex = M.Exe(M.OLD if tag == "old" else M.NEW, tag)
start = ex.owner(rva) or rva
end = ex.ends.get(start, start + 0x400)
print("; %s fn %x..%x (%dB)" % (tag, start, end, end - start))
n = 0
for ins in ex.md.disasm(ex.img[start:end], start):
    print("%x: %s %s" % (ins.address, ins.mnemonic, ins.op_str))
    n += 1
    if n >= mx: print("; ...truncated"); break
