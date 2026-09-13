# -*- coding: utf-8 -*-
"""20차 배치A 전용 IR 보조 — .ll 통째 로드, !dbg 루트 해석, 포인터 gep 누적 오프셋."""
import io, re, os, sys, json
if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

GAIBC = r'C:\tfm2mods\_gaibc'
_cache = {}

def load(name):
    if name in _cache:
        return _cache[name]
    p = os.path.join(GAIBC, name)
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

def sp_of_scope(md, i, seen=None):
    seen = seen or set()
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
    """[(line, fn, file)] from innermost to root"""
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
        out.append((line, nm, filename(md, fi) if fi else '?'))
        i = int(ia) if ia else None
    return out

def root(md, dbg):
    c = chain(md, dbg)
    return c[-1] if c else None

DBG_RX = re.compile(r'!dbg !(\d+)')
def dbg_of(line):
    m = DBG_RX.search(line)
    return int(m.group(1)) if m else None

GEP_RX = re.compile(r'^\s*(%[\w.]+) = getelementptr(?: inbounds| nuw| nusw)* (.+?), ptr (%[\w.]+|@[\w.$]+), (.*?)(?:, !dbg|$)')
SIMPLE_IDX = re.compile(r'^i64 (-?\d+)$')

def ptr_map(body):
    """SSA 포인터 → (root, const_offset or None(가변), text). phi/select 는 후보 합집합 리스트."""
    pm = {}
    for ln in body:
        m = GEP_RX.match(ln)
        if m:
            dst, ty, base, idx = m.group(1), m.group(2), m.group(3), m.group(4)
            if ty != 'i8':
                idx = 'VAR'
            mi = SIMPLE_IDX.match(idx.strip())
            if base in pm:
                for (r, off) in pm[base]:
                    if mi and off is not None:
                        pm.setdefault(dst, []).append((r, off + int(mi.group(1))))
                    else:
                        pm.setdefault(dst, []).append((r, None))
            else:
                if mi:
                    pm.setdefault(dst, []).append((base, int(mi.group(1))))
                else:
                    pm.setdefault(dst, []).append((base, None))
            continue
        m = re.match(r'^\s*(%[\w.]+) = phi ptr (.*?)(?:, !dbg|$)', ln)
        if m:
            dst = m.group(1)
            for v in re.findall(r'\[ (%[\w.]+|@[\w.$]+|null), %[\w.]+ \]', m.group(2)):
                if v in pm:
                    pm.setdefault(dst, []).extend(pm[v])
                elif v != 'null':
                    pm.setdefault(dst, []).append((v, 0))
            continue
        m = re.match(r'^\s*(%[\w.]+) = select i1 %[\w.]+, ptr (%[\w.]+|@[\w.$]+|null), ptr (%[\w.]+|@[\w.$]+|null)', ln)
        if m:
            dst = m.group(1)
            for v in (m.group(2), m.group(3)):
                if v in pm:
                    pm.setdefault(dst, []).extend(pm[v])
                elif v != 'null':
                    pm.setdefault(dst, []).append((v, 0))
    return pm

def resolve(pm, v):
    if v in pm:
        return pm[v]
    return [(v, 0)]
