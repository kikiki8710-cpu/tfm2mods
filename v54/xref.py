# -*- coding: utf-8 -*-
import io, os, sys, struct, bisect, collections, re, pickle
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, 'C:/tfm2mods/v54')
from pe2 import load, BASE
D = 'C:/tfm2mods/v54'
SC = 'C:/Users/dev/AppData/Local/Temp/claude/C--Users-dev-Desktop-claude-tfm2--claude-worktrees-item-tactics-conflict-check-86a5d3/e2b9bb3b-0660-4ff5-9ee7-51903acd7108/scratchpad'
SEP = chr(92)
_c = {}
def srcmap(ver):
    if ver in _c: return _c[ver]
    r = {}
    for ln in io.open(os.path.join(D, '%s_srcmap.tsv' % ver), encoding='utf-8'):
        s, e, src, lines = ln.rstrip('\n').split('\t')
        r[int(s,16)] = (src, lines)
    _c[ver] = r
    return r

class G:
    def __init__(self, ver):
        self.ver = ver
        self.e = load(ver)
        nm, va, vsz, ra, rsz = [s for s in self.e.sections if s[0]=='.text'][0]
        self.tva, self.body = va, self.e.raw[ra:ra+rsz]
        self.funcs = [f for f in self.e.funcs() if va <= f[0] < va+vsz]
        self.fs = [f[0] for f in self.funcs]
        self.fset = set(self.fs)
        self.sz = {s:(e-s) for s,e in self.funcs}
        cp = os.path.join(SC, 'g_%s.pkl' % ver)
        if os.path.exists(cp):
            self.callers, self.callees = pickle.load(open(cp,'rb'))
        else:
            self._build(); pickle.dump((dict(self.callers),dict(self.callees)), open(cp,'wb'))
            self.callers=collections.defaultdict(set,self.callers); self.callees=collections.defaultdict(set,self.callees)
    def fo(self, rva):
        i = bisect.bisect_right(self.fs, rva) - 1
        if i >= 0 and self.funcs[i][0] <= rva < self.funcs[i][1]: return self.funcs[i][0]
        return None
    def _build(self):
        self.callers = collections.defaultdict(set); self.callees = collections.defaultdict(set)
        b = self.body; tva = self.tva; fset = self.fset
        unp = struct.Struct('<i').unpack_from
        fs = self.fs; funcs = self.funcs; br = bisect.bisect_right
        cur_lo = cur_hi = cur_f = -1
        for m in re.finditer(rb'\xe8', b):
            i = m.start()
            if i+5 > len(b): break
            rel = unp(b, i+1)[0]
            tgt = tva + i + 5 + rel
            if tgt not in fset: continue
            site = tva + i
            if not (cur_lo <= site < cur_hi):
                k = br(fs, site)-1
                if k < 0 or not (funcs[k][0] <= site < funcs[k][1]):
                    cur_lo = cur_hi = -1; continue
                cur_lo, cur_hi, cur_f = funcs[k][0], funcs[k][1], funcs[k][0]
            self.callers[tgt].add(cur_f); self.callees[cur_f].add(tgt)
    def desc(self, f):
        s = srcmap(self.ver).get(f, ('(nosrc)',''))
        sh = ' | '.join(p.split(SEP)[-1] for p in s[0].split(' | '))
        return '%06x %6dB %-46s [%s]' % (f, self.sz.get(f,0), sh[:46], s[1][:46])

if __name__ == '__main__':
    ver = sys.argv[1]
    g = G(ver)
    for a in sys.argv[2:]:
        f = g.fo(int(a,16))
        print('== self:', g.desc(f))
        print('-- callers (%d) --' % len(g.callers[f]))
        for c in sorted(g.callers[f]): print('   ', g.desc(c))
        print('-- callees (%d) --' % len(g.callees[f]))
        for c in sorted(g.callees[f]): print('   ', g.desc(c))
        print()
