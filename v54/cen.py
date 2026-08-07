# -*- coding: utf-8 -*-
"""cen.py - 진짜 immediate 피연산자만 세는 상수 사이트 전수조사 (srcmap2 집계).
   ⚠mem.disp(구조체 오프셋/포인터 산술)는 기본 제외 — 오독 방지.
  python cen.py <ver> <value> [width=4] [srcfilter] [--mem]
"""
import sys, struct, re, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner
import ls2, capstone
from pe2 import BASE

_S = {}
def sc(ver):
    if ver not in _S:
        _S[ver] = Scanner(ver)
    return _S[ver]

_SM = {}
def sm(ver):
    if ver not in _SM:
        m = {}
        for s, e, src, l in ls2.rows(ver):
            m[s] = ls2.short_of(src)
        _SM[ver] = m
    return _SM[ver]


def sites(ver, val, width=4, filt=None, allow_mem=False):
    S = sc(ver); M = sm(ver)
    pats = set()
    for w, f in ((1, '<b'), (1, '<B'), (2, '<h'), (2, '<H'), (4, '<i'), (4, '<I'), (8, '<q'), (8, '<Q')):
        if w > width:
            continue
        try:
            pats.add(struct.pack(f, val))
        except Exception:
            pass
    offs = set()
    for p in pats:
        for m in re.finditer(re.escape(p), S.body):
            offs.add(S.tva + m.start())
    fmap = collections.defaultdict(set)
    for o in sorted(offs):
        f = S.func_of(o)
        if not f:
            continue
        src = M.get(f[0], '(nosrc)')
        if filt and filt.lower() not in src.lower():
            continue
        fmap[f].add(o)
    by = collections.defaultdict(list)
    for f, os_ in fmap.items():
        src = M.get(f[0], '(nosrc)')
        for i in S.disf(f):
            a = i.address - BASE
            if not any(a <= o < a + i.size for o in os_):
                continue
            hit = False
            for op in i.operands:
                if op.type == capstone.x86.X86_OP_IMM and op.imm == val:
                    hit = True
                elif allow_mem and op.type == capstone.x86.X86_OP_MEM and op.mem.disp == val:
                    hit = True
            if hit:
                by[src].append((a, f[0], i))
        S._dis.clear()
    for k in by:
        by[k].sort()
    return by


def show(by, val, tag='', maxn=400):
    tot = sum(len(v) for v in by.values())
    print('== %s val=%d (0x%x) -> 총 %d사이트 / %d파일그룹' % (tag, val, val & 0xffffffffffffffff, tot, len(by)))
    for src in sorted(by, key=lambda k: -len(by[k])):
        print('  [%3d] %s' % (len(by[src]), src[:100]))
        for a, fs, i in by[src][:maxn]:
            print('        %06x fn %06x  %-26s %s %s' % (a, fs, i.bytes.hex(), i.mnemonic, i.op_str))
    return tot


if __name__ == '__main__':
    v = int(sys.argv[2], 0)
    w = int(sys.argv[3]) if len(sys.argv) > 3 else 4
    fl = sys.argv[4] if len(sys.argv) > 4 and not sys.argv[4].startswith('--') else None
    am = '--mem' in sys.argv
    show(sites(sys.argv[1], v, w, fl, am), v, sys.argv[1])
