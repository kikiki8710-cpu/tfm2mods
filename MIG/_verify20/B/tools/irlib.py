# -*- coding: utf-8 -*-
"""20B 작업용: .ll 통째 로드 → 함수 범위·메타데이터·!dbg 루트 줄 해석."""
import io, os, re, sys, json, pickle
if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

BASES = (r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc', r'C:\tfm2mods\_gvbc')
CACHE = os.path.dirname(os.path.abspath(__file__))

class LL:
    def __init__(self, name):
        path = name
        if not os.path.isabs(path):
            for b in BASES:
                p = os.path.join(b, name)
                if os.path.exists(p):
                    path = p; break
        self.path = path
        self.name = os.path.basename(path)
        self.lines = io.open(path, 'r', encoding='utf-8', errors='replace').read().split('\n')
        self.md = {}
        rx = re.compile(r'^!(\d+) = (.*)$')
        for ln in self.lines:
            if ln.startswith('!'):
                m = rx.match(ln)
                if m:
                    self.md[int(m.group(1))] = m.group(2)
        self.files = {}
        for k, v in self.md.items():
            m = re.match(r'!DIFile\(filename: "([^"]+)"', v)
            if m:
                self.files[k] = m.group(1)

    def line(self, n):  # 1-based
        return self.lines[n - 1]

    def loc(self, i):
        """!DILocation id → (line, file, scope_fn, inlinedAt)"""
        s = self.md.get(i, '')
        if '!DILocation' not in s[:30]:
            return None
        l = re.search(r'line: (\d+)', s)
        sc = re.search(r'scope: !(\d+)', s)
        ia = re.search(r'inlinedAt: !(\d+)', s)
        fn, file = self.scope_fn(int(sc.group(1))) if sc else (None, None)
        return (int(l.group(1)) if l else 0, file, fn, int(ia.group(1)) if ia else None)

    def scope_fn(self, i):
        seen = set()
        while i is not None and i not in seen:
            seen.add(i)
            s = self.md.get(i, '')
            m = re.search(r'DISubprogram\(name: "([^"]+)"', s)
            if m:
                f = re.search(r'file: !(\d+)', s)
                return m.group(1), (self.files.get(int(f.group(1))) if f else None)
            m = re.search(r'file: !(\d+)', s)
            file = self.files.get(int(m.group(1))) if m else None
            m = re.search(r'scope: !(\d+)', s)
            i = int(m.group(1)) if m else None
        return None, None

    def chain(self, i):
        out = []
        seen = set()
        while i is not None and i not in seen:
            seen.add(i)
            r = self.loc(i)
            if r is None: break
            out.append((i,) + r)
            i = r[3]
        return out

    def root(self, i):
        c = self.chain(i)
        return c[-1] if c else None

    def dbg_of(self, n):
        m = re.search(r'!dbg !(\d+)', self.line(n))
        return int(m.group(1)) if m else None

    def root_of_line(self, n):
        d = self.dbg_of(n)
        return self.root(d) if d else None

    def find_define(self, sym):
        for i, ln in enumerate(self.lines, 1):
            if ln.startswith('define') and ('@' + sym + '(') in ln:
                return i
        return None

    def func_range(self, start):
        for i in range(start, len(self.lines) + 1):
            if self.lines[i - 1] == '}':
                return start, i
        return start, None

def get(name):
    return LL(name)
