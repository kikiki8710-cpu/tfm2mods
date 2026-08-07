# -*- coding: utf-8 -*-
"""constant scanner: byte-search LE encoding in .text -> attribute to func -> verify by disasm"""
import io, os, re, struct, sys, bisect, collections

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
import capstone

D = r'C:\tfm2mods\v54'
_cache = {}

def srcmap(ver):
    if ver in _cache: return _cache[ver]
    rows = {}
    for ln in io.open(os.path.join(D, '%s_srcmap.tsv' % ver), encoding='utf-8'):
        s, e, src, lines = ln.rstrip('\n').split('\t')
        rows[int(s,16)] = (src, lines)
    _cache[ver] = rows
    return rows

def src_of(ver, fstart):
    return srcmap(ver).get(fstart, (None, None))

class Scanner:
    def __init__(self, ver):
        self.ver = ver
        self.e = load(ver)
        nm, va, vsz, ra, rsz = [s for s in self.e.sections if s[0]=='.text'][0]
        self.tva, self.tra, self.trsz = va, ra, rsz
        self.body = self.e.raw[ra:ra+rsz]
        self.funcs = [f for f in self.e.funcs() if va <= f[0] < va+vsz]
        self.fstarts = [f[0] for f in self.funcs]
        self._dis = {}
    def func_of(self, rva):
        i = bisect.bisect_right(self.fstarts, rva) - 1
        if i >= 0 and self.funcs[i][0] <= rva < self.funcs[i][1]:
            return self.funcs[i]
        return None
    def disf(self, f):
        if f in self._dis: return self._dis[f]
        s, e = f
        ins = list(self.e.md.disasm(self.e.rd(s, e-s), BASE+s))
        if len(self._dis) > 3000: self._dis.clear()
        self._dis[f] = ins
        return ins
    def hits(self, val, width=4):
        if width == 4: pat = struct.pack('<i' if val < 0 else '<I', val)
        else: pat = struct.pack('<q' if val < 0 else '<Q', val)
        out, s2 = [], set()
        for m in re.finditer(re.escape(pat), self.body):
            rva = self.tva + m.start()
            f = self.func_of(rva)
            if not f: continue
            for i in self.disf(f):
                a = i.address - BASE
                if not (a <= rva < a + i.size): continue
                ok = False
                for op in i.operands:
                    if op.type == capstone.x86.X86_OP_IMM and op.imm == val: ok = True
                    elif op.type == capstone.x86.X86_OP_MEM and op.mem.disp == val: ok = True
                if ok and a not in s2:
                    s2.add(a); out.append((a, f, i))
                break
        out.sort()
        return out
    def report(self, val, width=4, label='', maxn=200, ctx=0):
        hs = self.hits(val, width)
        bysrc = collections.defaultdict(list)
        for a, f, i in hs:
            src, lines = src_of(self.ver, f[0])
            bysrc[src or '(nosrc)'].append((a, f, i, lines))
        print('== [%s] %s %d (0x%x) total %d hits, %d srcs' % (self.ver, label, val, val, len(hs), len(bysrc)))
        for src in sorted(bysrc, key=lambda k: -len(bysrc[k])):
            lst = bysrc[src]
            print('  %-95s %d' % (src[:95], len(lst)))
            for a, f, i, lines in lst[:maxn]:
                print('      %06x [fn %06x] %s %s' % (a, f[0], i.mnemonic, i.op_str))
                if ctx:
                    ins = self.disf(f)
                    for k, x in enumerate(ins):
                        if x.address - BASE == a:
                            for y in ins[max(0,k-ctx):k+ctx+1]:
                                print('           %06x  %s %s' % (y.address-BASE, y.mnemonic, y.op_str))
                            break
        print()

if __name__ == '__main__':
    val = int(sys.argv[2], 0)
    w = int(sys.argv[3]) if len(sys.argv) > 3 else 4
    Scanner(sys.argv[1]).report(val, w, ctx=int(sys.argv[4]) if len(sys.argv)>4 else 0)
