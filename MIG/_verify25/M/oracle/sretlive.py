"""sretlive.py <file.ll> <define_line> [<argname>=%0] — 지정 define 본문에서 sret(또는 지정 인자)로부터 gep 로 파생된 포인터에 대한 store/memcpy/memset/call 인자 전달을 오프셋과 함께 나열한다."""
import sys, re, io

TYSZ = {'i1':1,'i8':1,'i16':2,'i32':4,'i64':8,'ptr':8,'double':8,'float':4,'i128':16}

def tysize(t):
    t = t.strip()
    if t in TYSZ: return TYSZ[t]
    m = re.match(r'\[(\d+) x (\w+)\]', t)
    if m: return int(m.group(1))*tysize(m.group(2))
    m = re.match(r'\{(.*)\}$', t)
    if m:
        # crude: sum of members
        parts = [p.strip() for p in m.group(1).split(',')]
        return sum(tysize(p) for p in parts)
    return None

def main():
    path, line = sys.argv[1], int(sys.argv[2])
    root = sys.argv[3] if len(sys.argv) > 3 else '%0'
    with io.open(path, encoding='utf-8', errors='replace') as f:
        lines = f.readlines()
    i = line - 1
    body = []
    while True:
        body.append((i+1, lines[i].rstrip('\n')))
        if lines[i].startswith('}'): break
        i += 1
    off = {root: 0}
    out = []
    for ln, s in body:
        st = s.strip()
        m = re.match(r'(%[\w.]+) = getelementptr (?:inbounds )?(?:nuw )?i8, ptr (%[\w.]+), i64 (-?\d+)', st)
        if m and m.group(2) in off:
            off[m.group(1)] = off[m.group(2)] + int(m.group(3)); continue
        m = re.match(r'(%[\w.]+) = getelementptr (?:inbounds )?(?:nuw )?(.+?), ptr (%[\w.]+), i64 (-?\d+)(?:, i64 (-?\d+))?', st)
        if m and m.group(3) in off and m.group(1) not in off:
            out.append((ln, 'GEP-typed', m.group(3), off[m.group(3)], st[:160])); continue
        m = re.match(r'store (?:volatile )?(.+?) (.+?), ptr (%[\w.]+)', st)
        if m and m.group(3) in off:
            sz = tysize(m.group(1))
            o = off[m.group(3)]
            out.append((ln, 'store', o, (o + sz) if sz else None, f'{m.group(1)} {m.group(2)[:60]}')); continue
        m = re.search(r'@llvm\.mem(cpy|set)\.[\w.]+\(ptr (?:[\w() ,]+ )?(%[\w.]+), (?:i8 (\S+?), )?(?:ptr (?:[\w() ,]+ )?(%[\w.]+), )?i64 (\d+)', st)
        if m:
            dst = m.group(2); n = int(m.group(5))
            if dst in off:
                o = off[dst]
                out.append((ln, 'mem'+m.group(1)+'-dst', o, o+n, f'src={m.group(4)} val={m.group(3)}'))
            src = m.group(4)
            if src in off:
                o = off[src]
                out.append((ln, 'mem'+m.group(1)+'-SRC(read)', o, o+n, f'dst={dst}'))
            continue
        if ('call ' in st or 'invoke ' in st) and '@llvm.' not in st:
            for name, o in list(off.items()):
                if re.search(re.escape(name) + r'\b', st):
                    cm = re.search(r'@(\S+?)\(', st)
                    out.append((ln, 'CALLARG', o, None, f'{name} -> @{(cm.group(1) if cm else "?")[:90]}'))
    for r in out:
        print('\t'.join(str(x) for x in r))
    print('lines', body[0][0], '-', body[-1][0])

main()
