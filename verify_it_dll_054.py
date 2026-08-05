# -*- coding: utf-8 -*-
"""verify_it_dll_054.py — 배포된 tfm2_item_tactics.dll 이 정말 0.5.4 athlete 오프셋으로 빌드됐는지.

소스가 맞아도 배포본이 구 빌드면 의미가 없다(stale dll 은 이 프로젝트 단골 사고).
stride 는 0x8d0(구) / 0x8c0(신) 로 판별력이 있으므로, dll .text 를 선형 디스어셈해
두 즉값의 출현을 센다.

기대: 0x8c0 만 나오고 0x8d0 은 0.
"""
import io, sys, collections
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

TARGETS = [
    ("배포본(0.5.4)", r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_item_tactics\tfm2_item_tactics.dll"),
]

WATCH = {
    0x8d0: "구 ATH_STRIDE (0.5.3)",
    0x8c0: "신 ATH_STRIDE (0.5.4)",
    0x810: "0.5.3=athlete_id / 0.5.4=team",
    0x800: "0.5.4=athlete_id",
    0x820: "0.5.3=team",
}

for label, path in TARGETS:
    pe = pefile.PE(path, fast_load=True)
    base = pe.OPTIONAL_HEADER.ImageBase
    text = None
    for s in pe.sections:
        if s.Name.rstrip(b"\x00") == b".text":
            text, tva = s.get_data(), base + s.VirtualAddress
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    hist = collections.Counter()
    for ins in md.disasm(text, tva):
        for op in ins.operands:
            if op.type == 2:          # IMM
                if op.imm in WATCH:
                    hist[op.imm] += 1
            elif op.type == 3:        # MEM disp
                if op.mem.disp in WATCH:
                    hist[op.mem.disp] += 1
    print(f"\n===== {label}")
    print(f"  {path}")
    print(f"  .text {len(text):,}B")
    for v, desc in WATCH.items():
        n = hist.get(v, 0)
        mark = ""
        if v == 0x8d0 and n:
            mark = "   <== !! 구 stride 잔존"
        if v == 0x8c0 and n:
            mark = "   <== OK 신 stride 사용중"
        print(f"    0x{v:<5x} {n:>5}회   {desc}{mark}")
