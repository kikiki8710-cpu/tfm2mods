# -*- coding: utf-8 -*-
"""dis.py <rva_hex> <nbytes_hex> [--find call|<regex>] — exe 선형 디스어셈(capstone)"""
import sys, re
import pefile, capstone
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
EXE = r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.8\TeamfightManager2.exe"
BASE = 0x140000000
pe = pefile.PE(EXE, fast_load=True)
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail = True
rva = int(sys.argv[1], 16); n = int(sys.argv[2], 16)
flt = None
if len(sys.argv) > 4 and sys.argv[3] == "--find":
    flt = re.compile(sys.argv[4])
data = pe.get_data(rva, n)
for ins in md.disasm(data, BASE + rva):
    s = "%x: %s %s" % (ins.address - BASE, ins.mnemonic, ins.op_str)
    if flt is None or flt.search(s):
        print(s)
