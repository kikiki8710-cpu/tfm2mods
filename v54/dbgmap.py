# -*- coding: utf-8 -*-
"""dbgmap.py - derived-Debug fmt 함수에서 (필드명, 구조체 오프셋) 회수.
  python dbgmap.py <ver> <fmt_fn_rva_hex> [helper_rva_hex=5f740]
전략: call helper 직전 40개 명령 내에서
  · lea rdx,[rip+X] + mov r8d,len  → 필드명 문자열
  · lea r9,[rbp/rsp+S]            → 값 fat-ptr 로컬 → [S] 에 저장된 lea <SELF>+disp 추적
  · 구조체명 = 첫 call(DebugStruct 생성) 근처 lea rdx,[rip+X]+mov r8d,len
"""
import sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner
from pe2 import BASE
import capstone
X = capstone.x86
R64 = {}
for b in ['ax','bx','cx','dx','si','di','bp','sp']:
    R64['r'+b] = 'r'+b; R64['e'+b] = 'r'+b; R64[b] = 'r'+b
for n in range(8,16):
    R64['r%d'%n]='r%d'%n; R64['r%dd'%n]='r%d'%n; R64['r%dw'%n]='r%d'%n; R64['r%db'%n]='r%d'%n
def rn(i, r):
    return R64.get(i.reg_name(r), i.reg_name(r))

ver = sys.argv[1]
fn = int(sys.argv[2], 16)
helper = int(sys.argv[3], 16) if len(sys.argv) > 3 else 0x5f740
S = Scanner(ver)
E = S.e
f = S.func_of(fn)
ins = S.disf(f)


def rdstr(rva, n):
    b = E.rd(rva, n)
    try:
        return b.decode('utf-8')
    except Exception:
        return repr(b)

# 1) 모든 rip-rel lea 로 문자열 후보 수집 (명령 인덱스별)
# 2) 스택슬롯 -> (src reg, disp) 저장 추적
stack = {}   # rbp-disp -> ('fld', off) or ('imm', v)
regs = {}    # reg name -> ('fld', off)
out = []
names = []
for k, i in enumerate(ins):
    m, o = i.mnemonic, i.op_str
    if m == 'lea' and len(i.operands) == 2 and i.operands[1].type == X.X86_OP_MEM:
        mem = i.operands[1].mem
        dst = rn(i, i.operands[0].reg)
        if mem.base and i.reg_name(mem.base) == 'rip':
            tgt = (i.address - BASE) + i.size + mem.disp
            regs[dst] = ('str', tgt)
        elif mem.base and i.reg_name(mem.base) not in ('rsp', 'rbp'):
            regs[dst] = ('fld', i.reg_name(mem.base), mem.disp)
        elif mem.base and i.reg_name(mem.base) in ('rsp', 'rbp'):
            regs[dst] = ('stk', i.reg_name(mem.base), mem.disp)
        else:
            regs.pop(dst, None)
    elif m == 'mov' and len(i.operands) == 2:
        a, b = i.operands
        if a.type == X.X86_OP_REG and b.type == X.X86_OP_IMM:
            regs[rn(i, a.reg)] = ('imm', b.imm)
        elif a.type == X.X86_OP_REG and b.type == X.X86_OP_REG:
            v = regs.get(rn(i, b.reg))
            if v: regs[rn(i, a.reg)] = v
            else: regs.pop(rn(i, a.reg), None)
        elif a.type == X.X86_OP_MEM and b.type == X.X86_OP_REG and a.mem.base and i.reg_name(a.mem.base) in ('rsp','rbp'):
            v = regs.get(rn(i, b.reg))
            stack[(i.reg_name(a.mem.base), a.mem.disp)] = v
        elif a.type == X.X86_OP_REG and b.type == X.X86_OP_MEM and b.mem.base and i.reg_name(b.mem.base) in ('rsp','rbp'):
            v = stack.get((i.reg_name(b.mem.base), b.mem.disp))
            if v: regs[rn(i, a.reg)] = v
            else: regs.pop(rn(i, a.reg), None)
        else:
            if a.type == X.X86_OP_REG: regs.pop(rn(i, a.reg), None)
    elif m == 'call':
        tgt = i.operands[0].imm - BASE if i.operands and i.operands[0].type == X.X86_OP_IMM else None
        nm = regs.get('rdx'); ln = regs.get('r8')
        s = None
        if nm and nm[0] == 'str' and ln and ln[0] == 'imm' and 0 < ln[1] < 64:
            s = rdstr(nm[1], ln[1])
        v = regs.get('r9')
        off = None
        if v and v[0] == 'stk':
            sl = stack.get((v[1], v[2]))
            if sl and sl[0] == 'fld':
                off = (sl[1], sl[2])
        elif v and v[0] == 'fld':
            off = (v[1], v[2])
        if tgt == helper:
            out.append((i.address - BASE, s, off))
        elif s:
            names.append((i.address - BASE, s, tgt))
        regs = {}
print('=== fmt fn %06x  (helper %x)' % (fn, helper))
print('-- 이름 후보(다른 call 인자):')
for a, s, t in names[:6]:
    print('   %06x  %-40r  call %s' % (a, s, hex(t) if t else '?'))
print('-- 필드:')
for a, s, off in out:
    print('   %06x  name=%-32r  off=%s' % (a, s, ('%s+0x%x' % off) if off else '?'))
