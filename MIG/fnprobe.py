#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""fnprobe.py — exe 함수 하나의 프로파일(capstone · Ghidra 대체). (2026-09-13 · ghidra-re 스크래치 승격)
.pdata 경계 · 인자 레지스터 read-before-write · call/tail-jmp 콜리 · rip-rel 패닉 Location(파일:줄:열) · 점프테이블 · .text 전수 호출부 스캔.
사용: python -X utf8 MIG\fnprobe.py <rva>[v]   (v = 상세)
왜: Ghidra 두 서버가 내려가 있어도 「이 RVA 가 어느 IR 함수인가」를 판정할 수 있어야 한다(09-13 6건 판정 실증 · RE 2026-09-13_주소6건).
짝 = irprobe.py(IR 쪽 인자·call·DILocation 줄 집합) · locfind.py(패닉 Location → 참조 define 역추적).
"""
import sys, struct, pefile, re, bisect
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import *
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
BASE = 0x140000000
pe = pefile.PE(EXE, fast_load=True)
img = pe.get_memory_mapped_image()
secs = {s.Name.rstrip(b'\0').decode(): (s.VirtualAddress, s.Misc_VirtualSize) for s in pe.sections}
pd = [s for s in pe.sections if s.Name.startswith(b'.pdata')][0]
pdata = img[pd.VirtualAddress: pd.VirtualAddress + pd.Misc_VirtualSize]
rf = {}
for i in range(0, len(pdata) - 11, 12):
    b, e, u = struct.unpack_from('<III', pdata, i)
    if b == 0: break
    rf[b] = e
starts = sorted(rf)
def bounds(r):
    if r in rf: return r, rf[r]
    i = bisect.bisect_right(starts, r) - 1
    return starts[i], rf[starts[i]]
def in_sec(r, n):
    va, sz = secs[n]; return va <= r < va + sz
def rd_str(r, n=80):
    s = img[r:r+n]; out = []
    for c in s:
        if 32 <= c < 127: out.append(chr(c))
        else: break
    return ''.join(out)
def try_loc(r):
    if not in_sec(r, '.rdata'): return None
    try: p, l, line, col = struct.unpack_from('<QQII', img, r)
    except Exception: return None
    p -= BASE
    if 0 < l < 400 and 0 < line < 100000 and 0 < col < 1000 and in_sec(p, '.rdata'):
        s = img[p:p+l]
        try: s = s.decode('utf-8')
        except Exception: return None
        if s.endswith('.rs'): return (s, line, col)
    return None
cs = Cs(CS_ARCH_X86, CS_MODE_64); cs.detail = True
text_va, text_sz = secs['.text']
text = bytes(img[text_va:text_va+text_sz])
def callers(target):
    res = []
    for m in re.finditer(rb'[\xe8\xe9]', text):
        i = m.start()
        if i + 5 > len(text): continue
        rel = struct.unpack_from('<i', text, i+1)[0]
        if text_va + i + 5 + rel == target:
            res.append((text_va + i, 'call' if text[i]==0xe8 else 'jmp'))
    return res
ARGREG = {X86_REG_RCX:'rcx', X86_REG_RDX:'rdx', X86_REG_R8:'r8', X86_REG_R9:'r9',
          X86_REG_ECX:'rcx', X86_REG_EDX:'rdx', X86_REG_R8D:'r8', X86_REG_R9D:'r9',
          X86_REG_CL:'rcx', X86_REG_DL:'rdx', X86_REG_R8B:'r8', X86_REG_R9B:'r9',
          X86_REG_CX:'rcx', X86_REG_DX:'rdx', X86_REG_R8W:'r8', X86_REG_R9W:'r9',
          X86_REG_XMM0:'xmm0', X86_REG_XMM1:'xmm1', X86_REG_XMM2:'xmm2', X86_REG_XMM3:'xmm3'}
def analyze(r, verbose=False):
    s, e = bounds(r)
    print("="*100)
    print("RVA 0x%x  .pdata bounds 0x%x~0x%x (%d B)%s" % (r, s, e, e-s, '' if s==r else '  ** not a pdata start'))
    code = bytes(img[s:e])
    insns = list(cs.disasm(code, BASE + s))
    print("insn count %d" % len(insns))
    written = set(); read_first = []
    stack_args = set(); frame = 0
    calls = []; tailjmps=[]; locs = []; strs = []; jts = []; consts = set(); rets = 0
    for k, ins in enumerate(insns):
        if ins.mnemonic == 'sub' and ins.operands and ins.operands[0].type==X86_OP_REG and ins.operands[0].reg==X86_REG_RSP and ins.operands[1].type==X86_OP_IMM and frame==0:
            frame = ins.operands[1].imm
        if ins.mnemonic == 'ret': rets += 1
        rr, rw = ins.regs_access()
        for x in rr:
            if x in ARGREG and ARGREG[x] not in written and ARGREG[x] not in read_first:
                read_first.append(ARGREG[x])
        for x in rw:
            if x in ARGREG: written.add(ARGREG[x])
        for op in ins.operands:
            if op.type == X86_OP_MEM and op.mem.base == X86_REG_RSP and frame and op.mem.disp >= frame + 0x28 and op.mem.disp < frame + 0x200:
                stack_args.add(op.mem.disp - frame - 8)
            if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
                t = ins.address + ins.size + op.mem.disp - BASE
                L = try_loc(t)
                if L: locs.append((ins.address-BASE, t, L))
                elif in_sec(t, '.rdata'):
                    st = rd_str(t)
                    if len(st) >= 6: strs.append((ins.address-BASE, t, st))
                    if ins.mnemonic=='lea' and k+8 < len(insns):
                        seg = insns[k:k+8]
                        if any(i.mnemonic=='movsxd' for i in seg) and any(i.mnemonic=='jmp' and i.operands[0].type==X86_OP_REG for i in seg):
                            jts.append((ins.address-BASE, t))
            if op.type == X86_OP_IMM and ins.mnemonic in ('cmp','mov','and','test','add','sub','imul','movabs') and (op.imm > 0xff or op.imm < -0xff):
                consts.add(op.imm & 0xffffffffffffffff if op.imm<0 else op.imm)
        if ins.mnemonic == 'call':
            if ins.operands[0].type == X86_OP_IMM:
                calls.append((ins.address-BASE, ins.operands[0].imm - BASE))
            else:
                calls.append((ins.address-BASE, 'IND:'+ins.op_str))
        if ins.mnemonic == 'jmp' and ins.operands[0].type == X86_OP_IMM:
            t = ins.operands[0].imm - BASE
            if not (s <= t < e): tailjmps.append((ins.address-BASE, t))
    print("frame sub rsp=0x%x  ret=%d  argregs read-before-write: %s  stack-arg reads(ret-rel): %s" % (frame, rets, read_first, sorted('0x%x'%x for x in stack_args)))
    print("calls (order, dedup):")
    seen=set()
    for a,t in calls:
        if t in seen: continue
        seen.add(t)
        if isinstance(t,int):
            ts, te = bounds(t); print("   @%x  call 0x%x  (%dB)" % (a, t, te-ts))
        else: print("   @%x  call %s" % (a, t))
    for a,t in tailjmps:
        ts,te=bounds(t); print("   @%x  TAIL-JMP 0x%x (%dB)" % (a,t,te-ts))
    print("panic Location:")
    for a,t,L in locs: print("   @%x  ->0x%x  %s:%d:%d" % (a,t,L[0],L[1],L[2]))
    print("string refs:")
    for a,t,st in strs[:20]: print("   @%x  ->0x%x  %r" % (a,t,st[:70]))
    if jts: print("jump-table lea:", ['@%x->0x%x'%x for x in jts])
    big = sorted(c for c in consts if c > 0x100 and c < 0x100000000)
    print("consts(>0xff):", ['0x%x'%c for c in big[:40]])
    cl = callers(r)
    print("callers (.text raw scan) %d:" % len(cl))
    for a,k in cl:
        cs_, ce_ = bounds(a); print("   @%x %s  in fn 0x%x (%dB)" % (a,k,cs_,ce_-cs_))
    if verbose:
        for ins in insns: print("%08x  %-20s %s %s" % (ins.address-BASE, ins.bytes.hex(), ins.mnemonic, ins.op_str))
for a in sys.argv[1:]:
    analyze(int(a.rstrip('v'),16), a.endswith('v'))