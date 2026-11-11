# -*- coding: utf-8 -*-
import sys, os, pickle, struct, re
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_REG_RIP, X86_OP_IMM
SP = os.path.dirname(os.path.abspath(__file__))
OLD = r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe'
NEW = r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
_c = {}
def img(v):
    if v not in _c:
        I = A.Img(OLD if v == 'o' else NEW)
        pk = os.path.join(SP, 'loc_%s.pkl' % v)
        if os.path.exists(pk):
            I.loc = pickle.load(open(pk, 'rb'))
        else:
            I.loc = A.find_locations(I); pickle.dump(I.loc, open(pk, 'wb'))
        _c[v] = I
    return _c[v]
def fn(v, rva):
    return img(v).owner(rva)
def rows(v, b, e):
    I = img(v)
    return list(md.disasm(I.code(b, e - b), A.BASE + b))
def note(v, x):
    I = img(v)
    for op in x.operands:
        if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
            tg = x.address + x.size + op.mem.disp - A.BASE
            L = I.loc.get(tg)
            if L: return '   ; LOC %s:%d:%d' % (L[0].split(chr(92))[-1], L[1], L[2])
            return '   ; ->%x' % tg
    return ''
def fmt(v, x, mark=''):
    r = x.address - A.BASE
    return '%s %8x  %-24s %s %s%s' % (mark or ' ', r, x.bytes.hex(), x.mnemonic, x.op_str, note(v, x))
def show(v, a, b=None, n=None, mark=None):
    """a..b 디스어셈 (a 가 함수 중간이면 owner 경계로)"""
    f = fn(v, a)
    if b is None: b = f[1]
    if n is not None:
        rs = rows(v, f[0], f[1])
        idx = next(i for i, x in enumerate(rs) if x.address - A.BASE >= a)
        sel = rs[max(0, idx - n): idx + n]
    else:
        sel = [x for x in rows(v, f[0], f[1]) if a <= x.address - A.BASE < b]
    for x in sel:
        r = x.address - A.BASE
        print(fmt(v, x, '>' if mark and r == mark else ' '))
def grep(v, func_rva, pat, ctx=0):
    f = fn(v, func_rva)
    rs = rows(v, f[0], f[1])
    rx = re.compile(pat)
    for i, x in enumerate(rs):
        s = '%s %s' % (x.mnemonic, x.op_str)
        if rx.search(s):
            for y in rs[max(0, i - ctx): i + ctx + 1]:
                print(fmt(v, y, '>' if y is x else ''))
            if ctx: print('   ---')
def calls_to(v, target):
    """text 전체에서 E8 call target"""
    I = img(v); out = []
    for nm, va, sz, pr in I.secs:
        if nm != '.text': continue
        d = I.data[pr:pr + sz]
        for m in re.finditer(b'\xe8', d):
            i = m.start()
            if i + 5 > len(d): break
            rel = struct.unpack_from('<i', d, i + 1)[0]
            if va + i + 5 + rel == target:
                out.append(va + i)
    return out
def locs_in(v, func_rva):
    f = fn(v, func_rva); out = []
    for x in rows(v, f[0], f[1]):
        n = note(v, x)
        if 'LOC' in n: out.append((x.address - A.BASE, n.split('LOC ')[1]))
    return out

import difflib
def norm(x):
    s = x.mnemonic
    ops = x.op_str
    ops = re.sub(r'0x[0-9a-f]{6,}', 'ADDR', ops)
    ops = re.sub(r'\[rbp [+-] 0x[0-9a-f]+\]', '[rbp+S]', ops)
    ops = re.sub(r'\[rsp [+-] 0x[0-9a-f]+\]', '[rsp+S]', ops)
    ops = re.sub(r'\[rip [+-] 0x[0-9a-f]+\]', '[rip+X]', ops)
    return s + ' ' + ops
def fdiff(fo, fn_, ctx_only=True, out=None, lim=None):
    """두 함수 명령 정렬 diff — 바뀐 블록만 출력"""
    a = rows('o', *fn('o', fo)); b = rows('n', *fn('n', fn_))
    na = [norm(x) for x in a]; nb = [norm(x) for x in b]
    sm = difflib.SequenceMatcher(None, na, nb, autojunk=False)
    res = []
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == 'equal': continue
        res.append((tag, a[i1:i2], b[j1:j2], a[i1].address - A.BASE if i1 < len(a) else None, b[j1].address - A.BASE if j1 < len(b) else None))
    return res, sm
def pdiff(fo, fn_, lo=None, hi=None, maxl=30):
    res, sm = fdiff(fo, fn_)
    for tag, aa, bb, ia, ib in res:
        if lo is not None and ia is not None and not (lo <= ia < hi): continue
        print('=== %s old@%s new@%s (%d/%d)' % (tag, ia and hex(ia), ib and hex(ib), len(aa), len(bb)))
        for x in aa[:maxl]: print('  -', fmt('o', x))
        for x in bb[:maxl]: print('  +', fmt('n', x))
def amap(fo, fn_):
    """구 rva -> 신 rva (equal 블록)"""
    a = rows('o', *fn('o', fo)); b = rows('n', *fn('n', fn_))
    na = [norm(x) for x in a]; nb = [norm(x) for x in b]
    sm = difflib.SequenceMatcher(None, na, nb, autojunk=False)
    m = {}
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == 'equal':
            for k in range(i2 - i1):
                m[a[i1 + k].address - A.BASE] = b[j1 + k].address - A.BASE
    return m

def riprefs(v, target):
    """text 전체에서 [rip+disp32] 가 target 을 가리키는 명령 시작 후보 (4C/48 8D xx 형태 LEA 위주)"""
    I = img(v); out = []
    for nm, va, sz, pr in I.secs:
        if nm != '.text': continue
        d = I.data[pr:pr + sz]
        # disp32 위치 i: insn_end = i+4 (LEA r, [rip+d] 는 disp 가 마지막)
        for i in range(0, len(d) - 4):
            pass
    return out
def riprefs_fast(v, target):
    import numpy as np
    I = img(v); out = []
    for nm, va, sz, pr in I.secs:
        if nm != '.text': continue
        d = np.frombuffer(I.data[pr:pr + sz], dtype=np.uint8)
        n = len(d) - 4
        disp = d[0:n].astype(np.int64) | (d[1:n+1].astype(np.int64) << 8) | (d[2:n+2].astype(np.int64) << 16) | (d[3:n+3].astype(np.int64) << 24)
        disp = np.where(disp >= 2**31, disp - 2**32, disp)
        idx = np.arange(n) + va + 4 + disp
        hits = np.nonzero(idx == target)[0]
        for i in hits:
            out.append(va + int(i))  # disp 위치
    return out
