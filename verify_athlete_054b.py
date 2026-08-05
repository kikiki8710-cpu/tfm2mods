# -*- coding: utf-8 -*-
"""verify_athlete_054b.py — 로스터 순회 함수 1곳을 0.5.3/0.5.4 에서 명령단위로 대조.

앞선 히스토그램은 team(0x820->0x810, 양쪽 138회로 동수)은 확인해 줬지만
**id 오프셋은 가려내지 못했다**. 그래서 모드 주석(lib.rs L2327)이 인용한 구체 사이트
  0.5.3 `0x1740380` : `add rbx,0x8d0` -> `mov r12,[rbx+0x810]`  (id 로 알려진 자리)
를 0.5.3 에서 재현 확인하고, 같은 모양의 0.5.4 사이트를 찾아 disp 를 읽는다.

판정 기준:
  0.5.4 대응 사이트가 `add rbx,0x8c0` -> `mov r12,[rbx+0x800]` 이면  id = 0x800  (모드 소스가 맞음)
  0.5.4 대응 사이트가 `add rbx,0x8c0` -> `mov r12,[rbx+0x810]` 이면  id = 0x810  (모드 소스가 틀림 = 조인 실패)
"""
import io, sys
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
BASE = 0x140000000


def load(path):
    pe = pefile.PE(path, fast_load=True)
    for s in pe.sections:
        if s.Name.rstrip(b"\x00") == b".text":
            return s.get_data(), BASE + s.VirtualAddress
    raise SystemExit("no .text")


def dis_at(data, va, addr, n=60):
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    off = addr - va
    return list(md.disasm(data[off: off + n * 8], addr))[:n]


P53 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"
P54 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe"

d3, v3 = load(P53)
d4, v4 = load(P54)


def scan_idiom(data, va, stride, label):
    """`add reg, stride` 직후 16명령 안의 `mov r64, [같은reg + disp]` 를 전부 수집."""
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    imm = stride.to_bytes(4, "little")
    out = []
    start = 0
    hits = 0
    while True:
        i = data.find(imm, start)
        if i < 0:
            break
        start = i + 1
        s = max(0, i - 8)
        insns = list(md.disasm(data[s: i + 8 + 16 * 8], va + s))
        for idx, ins in enumerate(insns):
            if not (ins.address <= va + i < ins.address + ins.size):
                continue
            if ins.mnemonic != "add" or f"0x{stride:x}" not in ins.op_str:
                break
            if len(ins.operands) != 2 or ins.operands[0].type != 1:  # reg
                break
            reg = ins.operands[0].reg
            hits += 1
            for nxt in insns[idx + 1: idx + 17]:
                if nxt.mnemonic != "mov" or len(nxt.operands) != 2:
                    continue
                dst, src = nxt.operands
                if src.type == 3 and src.mem.base == reg and src.mem.index == 0 and dst.type == 1:
                    if 0x700 <= src.mem.disp <= 0x900:
                        out.append((ins.address, nxt.address, src.mem.disp, nxt.op_str))
            break
    print(f"\n===== {label}  `add reg,0x{stride:x}` {hits}곳 -> 직후 [reg+disp] 로드 {len(out)}건")
    import collections
    h = collections.Counter(d for _, _, d, _ in out)
    for d, n in h.most_common(10):
        print(f"    +0x{d:<5x} {n:>4}회")
    return out, h


o3, h3 = scan_idiom(d3, v3, 0x8d0, "0.5.3")
o4, h4 = scan_idiom(d4, v4, 0x8c0, "0.5.4")

print("\n===== 0.5.3 -> 0.5.4  같은 순위끼리 대응")
t3 = h3.most_common(10)
t4 = h4.most_common(10)
for i in range(min(len(t3), len(t4))):
    d3v, n3 = t3[i]
    d4v, n4 = t4[i]
    mark = "  <== -0x10 시프트 일치" if d3v - d4v == 0x10 and n3 == n4 else ("  (동수)" if n3 == n4 else "")
    print(f"  #{i+1}  0.5.3 +0x{d3v:<5x} {n3:>4}회   |   0.5.4 +0x{d4v:<5x} {n4:>4}회{mark}")

print("\n===== 모드 주석이 인용한 0.5.3 사이트 0x1740380 주변")
for ins in dis_at(d3, v3, 0x140000000 + 0x1740380, 26):
    print(f"  {ins.address - BASE:#010x}  {ins.mnemonic:<8} {ins.op_str}")
