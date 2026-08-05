# -*- coding: utf-8 -*-
"""verify_athlete_054c.py — 0.5.3 로스터 순회 루프(0x1740380)의 0.5.4 대응 사이트를 찾아 id disp 확정.

0.5.3 실측(054b.py):
    0x1740380  add rbx, 0x8d0
    ...
    0x1740397  mov r12, qword ptr [rbx + 0x810]      <- athlete_id 읽기 (모드 주석과 일치)

같은 루프 모양(add reg,stride / mov r64,[reg+disp] / cmp reg,reg / je / jne 로 되돌기)을
0.5.4 에서 찾아 disp 를 읽는다. 여러 개 나오면 전부 출력해 사람이 판단한다.
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


def find_loop(path, stride, label):
    data, va = load(path)
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    imm = stride.to_bytes(4, "little")
    found = []
    start = 0
    while True:
        i = data.find(imm, start)
        if i < 0:
            break
        start = i + 1
        s = max(0, i - 8)
        insns = list(md.disasm(data[s: i + 8 + 24 * 8], va + s))
        for idx, ins in enumerate(insns):
            if not (ins.address <= va + i < ins.address + ins.size):
                continue
            if ins.mnemonic != "add" or f"0x{stride:x}" not in ins.op_str:
                break
            if len(ins.operands) != 2 or ins.operands[0].type != 1:
                break
            reg = ins.operands[0].reg
            tail = insns[idx + 1: idx + 16]
            # 루프 지문: 같은 reg 를 cmp 로 끝 검사 + 그 reg 기준 [reg+disp] 로 64bit 로드 + 되돌기 분기
            has_cmp = any(t.mnemonic == "cmp" and t.operands and t.operands[0].type == 1
                          and t.operands[0].reg == reg for t in tail)
            back = any(t.mnemonic in ("jne", "je", "jmp") and t.op_str.startswith("0x")
                       and int(t.op_str, 16) <= ins.address for t in tail)
            loads = [(t.address, t.operands[1].mem.disp, t.op_str)
                     for t in tail
                     if t.mnemonic == "mov" and len(t.operands) == 2
                     and t.operands[1].type == 3 and t.operands[1].mem.base == reg
                     and t.operands[1].mem.index == 0 and t.operands[0].type == 1
                     and 0x700 <= t.operands[1].mem.disp <= 0x900]
            if has_cmp and back and loads:
                found.append((ins.address, loads))
            break
    print(f"\n===== {label}  루프 지문(add reg,0x{stride:x} + cmp + 되돌기 + [reg+disp] 로드) {len(found)}곳")
    for addr, loads in found:
        print(f"  add @ {addr - BASE:#010x}")
        for la, disp, ops in loads:
            print(f"      {la - BASE:#010x}  mov {ops}      -> disp +0x{disp:x}")
    return found


find_loop(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe", 0x8d0, "0.5.3")
find_loop(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe", 0x8c0, "0.5.4")
