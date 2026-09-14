# -*- coding: utf-8 -*-
"""22차 A 전용: .ll 통째 로드 → 지정 범위를 !dbg 사슬(innermost→root) 주석과 함께 덤프."""
import io, re, os, sys, pickle
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
GAIBC = r'C:\tfm2mods\_gaibc'
GCBC = r'C:\tfm2mods\_gcbc'
_cache = {}

def load(name):
    if name in _cache:
        return _cache[name]
    base = GCBC if name.startswith('g') else GAIBC
    p = os.path.join(base, name)
    lines = io.open(p, 'r', encoding='utf-8', errors='replace').read().split('\n')
    md = {}
    rx = re.compile(r'^!(\d+) = (.*)$')
    for ln in lines:
        if ln.startswith('!'):
            m = rx.match(ln)
            if m:
                md[int(m.group(1))] = m.group(2)
    _cache[name] = (lines, md)
    return lines, md

def sp_of_scope(md, i):
    seen = set()
    while i is not None and i not in seen:
        seen.add(i)
        s = md.get(i, '')
        m = re.search(r'DISubprogram\(name: "([^"]+)"', s)
        if m:
            f = re.search(r'file: !(\d+)', s)
            l = re.search(r'line: (\d+)', s)
            return m.group(1), (f and int(f.group(1))), (l and int(l.group(1)))
        m = re.search(r'scope: !(\d+)', s)
        i = int(m.group(1)) if m else None
    return '?', None, None

def filename(md, i):
    s = md.get(i, '')
    m = re.search(r'filename: "([^"]+)"', s)
    return m.group(1) if m else '?'

def chain(md, dbg):
    out = []
    i = dbg
    seen = set()
    while i is not None and i not in seen:
        seen.add(i)
        s = md.get(i, '')
        m = re.search(r'DILocation\(line: (\d+),.*?scope: !(\d+)(?:, inlinedAt: !(\d+))?', s)
        if not m:
            break
        line, scope, ia = int(m.group(1)), int(m.group(2)), m.group(3)
        nm, fi, _ = sp_of_scope(md, scope)
        out.append((line, nm, os.path.basename(filename(md, fi)) if fi else '?'))
        i = int(ia) if ia else None
    return out

DBG_RX = re.compile(r'!dbg !(\d+)')

def dump(name, frm, to, out=None, full_chain=True):
    lines, md = load(name)
    res = []
    for n in range(frm, to + 1):
        ln = lines[n - 1]
        m = DBG_RX.search(ln)
        ann = ''
        if m:
            c = chain(md, int(m.group(1)))
            if c:
                if full_chain:
                    ann = ' ;; ' + ' <- '.join(f'{f}:{l}@{fn}' for (l, fn, f) in c)
                else:
                    ann = f' ;; root {c[-1][2]}:{c[-1][0]}'
        res.append(f'{n}\t{ln}{ann}')
    txt = '\n'.join(res)
    if out:
        io.open(out, 'w', encoding='utf-8').write(txt)
    return txt

if __name__ == '__main__':
    name, frm, to = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    out = sys.argv[4] if len(sys.argv) > 4 else None
    t = dump(name, frm, to, out)
    if not out:
        print(t)
