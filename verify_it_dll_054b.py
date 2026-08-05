# -*- coding: utf-8 -*-
"""verify_it_dll_054b.py — 배포 dll 에 1회 남은 0x8d0 이 무엇인지 실제 명령으로 확인.

소스(주석 제외)에는 살아 있는 0x8d0 이 없다. 그렇다면 dll 의 1회는
(a) 스택 프레임 크기 등 무관한 즉값이거나 (b) 진짜 잔존 오프셋이다.
전자면 무해, 후자면 조인/순회가 깨진다 -> 명령을 직접 찍어 판별한다.
"""
import io, sys
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

PATH = (r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2"
        r"\mods\tfm2_item_tactics\tfm2_item_tactics.dll")

pe = pefile.PE(PATH, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
for s in pe.sections:
    if s.Name.rstrip(b"\x00") == b".text":
        text, tva = s.get_data(), base + s.VirtualAddress

md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True
insns = list(md.disasm(text, tva))
idx = {ins.address: i for i, ins in enumerate(insns)}

for want, tag in ((0x8d0, "구 stride"), (0x820, "0.5.3 team"), (0x8c0, "신 stride")):
    hits = []
    for i, ins in enumerate(insns):
        hit = False
        for op in ins.operands:
            if op.type == 2 and op.imm == want:
                hit = True
            elif op.type == 3 and op.mem.disp == want:
                hit = True
        if hit:
            hits.append(i)
    print(f"\n===== 0x{want:x} ({tag}) — {len(hits)}회")
    for i in hits[:8]:
        lo = max(0, i - 3)
        hi = min(len(insns), i + 4)
        for j in range(lo, hi):
            ins = insns[j]
            mark = "  <<<" if j == i else ""
            print(f"    {ins.address - base:#010x}  {ins.mnemonic:<8} {ins.op_str}{mark}")
        print("    " + "-" * 60)
