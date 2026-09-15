# -*- coding: utf-8 -*-
u"""argscan.py — exe 함수의 **진입 스택 인자 슬롯 전수 스캔**(capstone 선형 디스어셈 · 디컴 불필요).
용도(2026-09-14 신설 · 03 §34 교훈): internal(fastcc) 함수는 LTO ArgumentPromotion 으로 exe 인자 배치가 IR 과 달라질 수
있다(#103 evaluate_steal_for_target = exe sret+13 vs IR sret+11 → 재호출 AV). sweep 편입 전에 **exe 스택 인자 수 = IR 인자 수**
를 확인한다. 프롤로그(push×N + sub rsp + lea rbp,[rsp+K])로 entry_rsp 를 환산해 홈공간 위 `[entry+0x20..]` 접근을 센다.

사용: python -X utf8 argscan.py <rva_hex> [<end_rva_hex>]   (끝 주소 없으면 .pdata 로 함수 길이)
      python -X utf8 argscan.py 0xd9bce0 --caller 0xd9ac10   # 호출부 쪽: caller 안의 call 직전 22줄(rcx..r9/[rsp+0x20..] 스토어)
출력: 슬롯별 접근 명령(크기 = 포인터 8/정수/바이트 → 타입 힌트). ⚠비교는 사람이 IR define 과 한다(r11 internal 6 = 6/6 일치 · RE 09-14).
원본 = ghidra-re 서브에이전트 스크래치 argscan.py/argscan4.py(09-14) 를 도구로 승격. 09-15 `__chkstk` 프롤로그(sub rsp,rax) 지원.
"""
import sys, re, struct
import pefile, capstone
EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
BASE = 0x140000000
pe = pefile.PE(EXE, fast_load=True)
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail = True


def pdata_end(rva):
    for s in pe.sections:
        if s.Name.rstrip(b"\0") == b".pdata":
            d = s.get_data()
            for i in range(0, len(d) - 11, 12):
                b, e, _ = struct.unpack_from("<III", d, i)
                if b == rva:
                    return e
    return None


def scan_callee(s, e):
    insns = list(md.disasm(pe.get_data(s, e - s), BASE + s))
    depth = 8; rbp_off = None; sub = 0; pushes = []; last_eax = None
    for ins in insns[:40]:
        if ins.mnemonic == "push" and ins.op_str in ("rbp", "rbx", "rdi", "rsi", "r12", "r13", "r14", "r15"):
            depth += 8; pushes.append(ins.op_str); continue
        # ★09-15(23차 E 적발 · 165 evaluate_action sub=0x1318): 큰 프레임은 `mov eax, N ; call __chkstk ; sub rsp, rax`
        #   → `int(' rax')` ValueError. 직전 `mov eax, imm` 을 sub 값으로 쓴다(scratchpad23E/argscan2.py 승격).
        if ins.mnemonic == "mov" and ins.op_str.startswith("eax, "):
            try:
                last_eax = int(ins.op_str.split(",")[1], 0)
            except ValueError:
                pass
            continue
        if ins.mnemonic == "sub" and ins.op_str.startswith("rsp,"):
            v = ins.op_str.split(",")[1].strip()
            sub = last_eax if (v == "rax" and last_eax is not None) else int(v, 0)
            depth += sub; continue
        if ins.mnemonic == "lea" and ins.op_str.startswith("rbp, [rsp"):
            mm = re.search(r"rsp \+ (0x[0-9a-f]+|\d+)", ins.op_str); rbp_off = int(mm.group(1), 0) if mm else 0; continue
        if ins.mnemonic == "mov" and ins.op_str == "rbp, rsp":
            rbp_off = 0; continue
    print(u"== rva=%#x pushes=%s sub=%#x rbp=rsp+%s frame_depth=%#x" % (s, pushes, sub, rbp_off, depth))
    hits = {}
    for ins in insns:
        for op in ins.operands:
            if op.type == capstone.x86.X86_OP_MEM and op.mem.index == 0:
                b = op.mem.base
                if b == capstone.x86.X86_REG_RSP:
                    off = op.mem.disp
                elif b == capstone.x86.X86_REG_RBP and rbp_off is not None:
                    off = op.mem.disp + rbp_off
                else:
                    continue
                ent = off - depth
                if ent >= 0x20:
                    hits.setdefault(ent, []).append((ins.address, u"%s %s" % (ins.mnemonic, ins.op_str), op.size))
    for ent in sorted(hits):
        argn = (ent - 0x20) // 8 + 5
        print(u"  entry_rsp+%#x (arg%d · IR %%%d) x%d:" % (ent, argn, argn - 1, len(hits[ent])))
        for a, t, sz in hits[ent][:6]:
            print(u"     %x: %s  [size %d]" % (a, t, sz))
    print(u"  ⟹ 스택 인자 %d개 + 레지스터 4 = exe 인자 %d" % (len(hits), len(hits) + 4))


def scan_caller(callee, caller_s, caller_e, n=22):
    ins = list(md.disasm(pe.get_data(caller_s, caller_e - caller_s), BASE + caller_s))
    calls = [i for i, x in enumerate(ins) if x.mnemonic == "call" and x.op_str == "0x%x" % (BASE + callee)]
    print(u"== caller %#x → callee %#x : call %d곳" % (caller_s, callee, len(calls)))
    for idx in calls:
        print(u"--- call@%x" % (ins[idx].address - BASE))
        for x in ins[max(0, idx - n):idx + 1]:
            print(u"  %x: %s %s" % (x.address - BASE, x.mnemonic, x.op_str))


if __name__ == "__main__":
    a = sys.argv[1:]
    if not a:
        print(__doc__); sys.exit(0)
    rva = int(a[0], 16)
    if "--caller" in a:
        c = int(a[a.index("--caller") + 1], 16)
        ce = pdata_end(c) or c + 0x8000
        scan_caller(rva, c, ce)
    else:
        end = int(a[1], 16) if len(a) > 1 and not a[1].startswith("--") else pdata_end(rva)
        if not end:
            print(u"끝 주소를 .pdata 에서 못 찾음 — 두 번째 인자로 주라"); sys.exit(1)
        scan_callee(rva, end)
