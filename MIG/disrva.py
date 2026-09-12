#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""disrva.py — 현행 exe 의 RVA 구간을 capstone 으로 디스어셈블해 찍는다 (Ghidra 없이 빠르게 · 읽기 전용).

사용: python MIG\disrva.py <시작RVA> <끝RVA|+길이>   예) disrva.py 0xcafa57 +0xd0 / disrva.py 0xcafdaa 0xcafdf2
"""
import sys
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
BASE = 0x140000000

def main():
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    a = int(sys.argv[1], 16)
    b = int(sys.argv[2], 16) if not sys.argv[2].startswith("+") else a + int(sys.argv[2][1:], 16)
    pe = pefile.PE(EXE, fast_load=True)
    d = pe.get_memory_mapped_image()
    cs = Cs(CS_ARCH_X86, CS_MODE_64)
    for ins in cs.disasm(bytes(d[a:b]), BASE + a):
        print("%08x  %-24s %s %s" % (ins.address - BASE, ins.bytes.hex(), ins.mnemonic, ins.op_str))

if __name__ == "__main__":
    main()
