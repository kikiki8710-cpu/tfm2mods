# -*- coding: utf-8 -*-
"""26차 F 스크래치 — IR 범위 분석 유틸.
사용: python -X utf8 irutil.py <ll> <from> <to> <cmd> [args]
 cmd = stores <%arg>   : %arg 에서 파생된 포인터로의 store/memcpy/memset/call(ptr 인자) 전수
       loads  <%arg>   : 파생 포인터 load 전수(오프셋)
       dbg <id>...     : !dbg 사슬 루트
       lit <n>         : 리터럴 n 이 등장하는 줄 + 사슬
       calls           : call 줄 전수 + 사슬
       anon            : @anon.* / LocalKey / threadlocal 참조
"""
import io, re, sys, os
sys.stdout.reconfigure(encoding='utf-8', errors='replace')

def load(path):
    if not os.path.isabs(path):
        for b in (r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc'):
            p = os.path.join(b, path)
            if os.path.exists(p): path = p; break
    with io.open(path, 'r', encoding='utf-8', errors='replace') as f:
        lines = f.read().split('\n')
    md = {}
    rx = re.compile(r'^!(\d+) = (.*)$')
    for ln in lines:
        if ln.startswith('!'):
            m = rx.match(ln)
            if m: md[int(m.group(1))] = m.group(2)
    return lines, md

def chain(md, i, depth=64):
    out = []
    seen = set()
    while i is not None and i not in seen and depth > 0:
        seen.add(i); depth -= 1
        s = md.get(i, '')
        l = re.search(r'line: (\d+)', s)
        sc = re.search(r'scope: !(\d+)', s)
        fn = None; fl = None
        j = int(sc.group(1)) if sc else None
        seen2 = set()
        while j is not None and j not in seen2:
            seen2.add(j)
            t = md.get(j, '')
            m = re.search(r'DISubprogram\(name: "([^"]+)"', t)
            if m:
                fn = m.group(1)
                f = re.search(r'file: !(\d+)', t)
                if f:
                    ft = md.get(int(f.group(1)), '')
                    fm = re.search(r'filename: "([^"]+)"', ft)
                    fl = fm.group(1) if fm else None
                break
            m = re.search(r'scope: !(\d+)', t)
            j = int(m.group(1)) if m else None
        out.append((l.group(1) if l else '?', fn, fl))
        ia = re.search(r'inlinedAt: !(\d+)', s)
        i = int(ia.group(1)) if ia else None
    return out

def chain_str(md, i):
    c = chain(md, i)
    return '<'.join(x[0] for x in c) + '  [' + ' | '.join('%s@%s' % (x[1], (x[2] or '').split('\\')[-1]) for x in c) + ']'

def dbg_of(line):
    m = re.search(r'!dbg !(\d+)', line)
    return int(m.group(1)) if m else None

def derived(lines, frm, to, root):
    """root 예 '%0'. gep/bitcast/phi/select 로 파생된 SSA 이름 집합과 오프셋 추정."""
    names = {root: 0}
    changed = True
    body = [(k, lines[k-1]) for k in range(frm, to+1)]
    while changed:
        changed = False
        for k, ln in body:
            m = re.match(r'\s*(%[\w.]+) = (.*)', ln)
            if not m: continue
            dst, rhs = m.group(1), m.group(2)
            if dst in names: continue
            g = re.match(r'getelementptr inbounds(?: nuw)? i8, ptr (%[\w.]+), i64 (-?\d+)', rhs)
            if g and g.group(1) in names:
                names[dst] = names[g.group(1)] + int(g.group(2)); changed = True; continue
            g = re.match(r'getelementptr (?:inbounds )?(?:nuw )?(\S+), ptr (%[\w.]+), i64 (-?\d+)(?:, i64 (-?\d+))?', rhs)
            if g and g.group(2) in names:
                names[dst] = None; changed = True; continue
            g = re.match(r'(?:bitcast|addrspacecast) .* (%[\w.]+) to', rhs)
            if g and g.group(1) in names:
                names[dst] = names[g.group(1)]; changed = True; continue
            g = re.match(r'select i1 [^,]+, ptr (%[\w.]+), ptr (%[\w.]+)', rhs)
            if g and (g.group(1) in names or g.group(2) in names):
                names[dst] = None; changed = True; continue
            if rhs.startswith('phi ptr'):
                srcs = re.findall(r'\[ (%[\w.]+),', rhs)
                if any(s in names for s in srcs):
                    offs = set(names.get(s) for s in srcs if s in names)
                    names[dst] = offs.pop() if len(offs) == 1 and len(srcs) == sum(1 for s in srcs if s in names) else None
                    changed = True; continue
    return names

def cmd_stores(lines, md, frm, to, root):
    names = derived(lines, frm, to, root)
    for k in range(frm, to+1):
        ln = lines[k-1]
        s = ln.strip()
        hit = None
        m = re.match(r'store (.+?) (%[\w.]+|-?\d+|[^,]+), ptr (%[\w.]+)', s)
        if m and m.group(3) in names:
            hit = ('store', m.group(3), m.group(1), m.group(2))
        m2 = re.search(r'llvm\.mem(cpy|set|move)\.[^(]*\(ptr[^%]*(%[\w.]+),', s)
        if m2 and m2.group(2) in names:
            hit = ('mem' + m2.group(1), m2.group(2), '', '')
        if hit is None and (s.startswith('call') or s.startswith('tail call') or s.startswith('invoke') or '= call' in s or '= tail call' in s or '= invoke' in s):
            for nm in re.findall(r'ptr [^,)]*?(%[\w.]+)', s):
                if nm in names and 'readonly' not in s.split(nm)[0][-80:] and 'llvm.lifetime' not in s and 'llvm.dbg' not in s:
                    hit = ('callarg', nm, '', '')
                    break
        if hit:
            off = names[hit[1]]
            d = dbg_of(ln)
            print('%d\t%s\t%s\toff=%s\t%s\t;%s' % (k, hit[0], hit[1], ('0x%x' % off) if off is not None else '?', s[:150], chain_str(md, d) if d else '(no dbg)'))

def cmd_loads(lines, md, frm, to, root):
    names = derived(lines, frm, to, root)
    for k in range(frm, to+1):
        ln = lines[k-1]; s = ln.strip()
        m = re.match(r'(%[\w.]+) = load (\S+), ptr (%[\w.]+)', s)
        if m and m.group(3) in names:
            off = names[m.group(3)]
            d = dbg_of(ln)
            print('%d\tload\t%s\toff=%s\t%s\t;%s' % (k, m.group(2), ('0x%x' % off) if off is not None else '?', s[:100], chain_str(md, d) if d else '(no dbg)'))

def cmd_lit(lines, md, frm, to, n):
    rx = re.compile(r'(?<![\w.%!@#-])' + re.escape(n) + r'(?![\w.])')
    for k in range(frm, to+1):
        ln = lines[k-1]
        s = re.sub(r', !dbg.*$', '', ln.strip())
        s2 = re.sub(r'%[\w.]+|!\w+|@[\w.$]+|#\d+|align \d+|i\d+', ' ', s)
        if rx.search(s2):
            d = dbg_of(ln)
            print('%d\t%s\t;%s' % (k, s[:160], chain_str(md, d) if d else '(no dbg)'))

def cmd_calls(lines, md, frm, to):
    for k in range(frm, to+1):
        ln = lines[k-1]; s = ln.strip()
        if re.search(r'\b(call|invoke)\b', s) and 'llvm.dbg' not in s and 'llvm.lifetime' not in s and 'noalias.scope' not in s:
            m = re.search(r'@([\w.$]+)\(', s)
            ind = re.search(r'(call|invoke) [^@]*?(%[\w.]+)\(', s)
            nm = m.group(1) if m else ('INDIRECT ' + (ind.group(2) if ind else '?'))
            d = dbg_of(ln)
            print('%d\t%s\t;%s' % (k, nm[:140], chain_str(md, d) if d else '(no dbg)'))

def cmd_anon(lines, md, frm, to):
    for k in range(frm, to+1):
        ln = lines[k-1]
        if re.search(r'@anon\.|LocalKey|threadlocal|call_once|thread_local', ln):
            print('%d\t%s' % (k, ln.strip()[:200]))

if __name__ == '__main__':
    path, frm, to, cmd = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
    lines, md = load(path)
    if cmd == 'stores': cmd_stores(lines, md, frm, to, sys.argv[5])
    elif cmd == 'loads': cmd_loads(lines, md, frm, to, sys.argv[5])
    elif cmd == 'dbg':
        for a in sys.argv[5:]: print(a, chain_str(md, int(a)))
    elif cmd == 'lit': cmd_lit(lines, md, frm, to, sys.argv[5])
    elif cmd == 'calls': cmd_calls(lines, md, frm, to)
    elif cmd == 'anon': cmd_anon(lines, md, frm, to)
