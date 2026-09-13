# -*- coding: utf-8 -*-
"""함수 범위 안의 모든 store/atomicrmw/memcpy/memset + 인자 포인터 전달 call 을 루트(인자/alloca/load)까지 추적해 나열한다.
사용: python stores.py <ll> <from> <to> [rootfilter]"""
import re, sys
sys.path.insert(0, __import__('os').path.dirname(__file__))
from irlib import LL

ll = LL(sys.argv[1]); a = int(sys.argv[2]); b = int(sys.argv[3])
flt = sys.argv[4] if len(sys.argv) > 4 else None
defs = {}   # %name -> (lineno, text)
for n in range(a, b + 1):
    ln = ll.line(n)
    m = re.match(r'\s*(%[\w.]+) = (.*)', ln)
    if m:
        defs[m.group(1)] = (n, m.group(2))

def trace(v, depth=0):
    """포인터 SSA 값을 루트까지: returns (root, offset_expr_list)"""
    path = []
    seen = set()
    while True:
        if v in seen or depth > 40: return v, path
        seen.add(v); depth += 1
        if v not in defs:
            return v, path
        n, t = defs[v]
        if t.startswith('getelementptr'):
            m = re.match(r'getelementptr (inbounds )?(nuw )?(\S+), ptr (%[\w.]+), (.*?)(, !dbg.*)?$', t)
            if not m:
                m2 = re.match(r'getelementptr[^,]*, ptr (%[\w.]+), (.*?)(, !dbg.*)?$', t)
                if not m2: return v, path
                base, idx = m2.group(1), m2.group(2)
            else:
                base, idx = m.group(4), m.group(5)
            path.append(('gep', idx.strip(), n))
            v = base
        elif t.startswith('load ptr'):
            m = re.match(r'load ptr, ptr (%[\w.]+)', t)
            path.append(('load', n))
            if not m: return v, path
            v = m.group(1)
        elif t.startswith('phi'):
            path.append(('phi', t[:120], n)); return v, path
        elif t.startswith('select'):
            path.append(('select', t[:120], n)); return v, path
        elif t.startswith('alloca'):
            path.append(('alloca', n)); return v, path
        elif t.startswith('inttoptr') or t.startswith('bitcast'):
            path.append((t.split(' ')[0], n)); return v, path
        elif t.startswith('call') or t.startswith('invoke'):
            path.append(('call', t[:160], n)); return v, path
        else:
            path.append(('?', t[:100], n)); return v, path

def fmt(path):
    return ' <- '.join(
        (f"gep[{p[1]}]" if p[0] == 'gep' else p[0] if len(p) == 2 else f"{p[0]}({p[1][:60]})") for p in path)

print('DEFINE:', ll.line(a)[:600])
for n in range(a, b + 1):
    ln = ll.line(n).strip()
    kind = None
    if ln.startswith('store ') or re.match(r'%[\w.]+ = atomicrmw', ln) or ln.startswith('atomicrmw'):
        kind = 'store' if ln.startswith('store') else 'atomicrmw'
        m = re.search(r'ptr (%[\w.]+)', ln.split(',', 1)[1] if kind == 'store' else ln)
        if kind == 'store':
            # store <ty> <val>, ptr <p>
            m = re.match(r'store (volatile )?(.+), ptr (%[\w.]+)', ln)
            ptr = m.group(3) if m else '?'
            val = m.group(2)[:60] if m else '?'
        else:
            m = re.search(r'atomicrmw \w+ ptr (%[\w.]+), (.+?)(,| !dbg)', ln)
            ptr = m.group(1) if m else '?'; val = m.group(2)[:40] if m else '?'
    elif 'llvm.memcpy' in ln or 'llvm.memset' in ln or 'llvm.memmove' in ln:
        kind = 'mem'
        m = re.search(r'\(ptr [^,]*?(%[\w.]+), ', ln)
        ptr = m.group(1) if m else '?'
        val = ln[ln.find('@llvm'):][:90]
    else:
        continue
    root, path = trace(ptr)
    if flt and not root.startswith(flt): continue
    rl = ll.root_of_line(n)
    rs = f"{rl[1]}:{rl[0]}" if rl else '-'
    print(f"{n:6d} {kind:9s} root={root:8s} {fmt(path):70s} val={val:60s} src={rs}")
