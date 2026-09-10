import io, os, sys, re
if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
RE_DL = re.compile(r'^(![0-9]+) = !DILocation\(line: (\d+)[^\n]*?(?:scope: (![0-9]+))?(?:.*?inlinedAt: (![0-9]+))?\)')
RE_SC = re.compile(r'^(![0-9]+) = (?:distinct )?!DI(?:Subprogram|LexicalBlock|LexicalBlockFile)\([^\n]*?file: (![0-9]+)')
RE_SP = re.compile(RE_SC.pattern.replace('file: ', 'scope: '))
RE_SCN = re.compile(r'^(![0-9]+) = distinct !DISubprogram\(name: "([^"]*)"')
RE_F = re.compile(r'^(![0-9]+) = !DIFile\(filename: "([^"]*)"')

fn = sys.argv[1]
lines = io.open(fn, encoding='utf-8', errors='replace').read().split('\n')
meta, sc2f, files, scn, scp = {}, {}, {}, {}, {}
for ln in lines:
    if not ln.startswith('!'):
        continue
    if '!DILocation' in ln:
        m = RE_DL.match(ln)
        if m:
            meta[m.group(1)] = (int(m.group(2)), m.group(3), m.group(4))
    elif '!DIFile(' in ln:
        m = RE_F.match(ln)
        if m:
            files[m.group(1)] = m.group(2)
    else:
        if 'DISubprogram' in ln or 'DILexicalBlock' in ln:
            m = RE_SC.match(ln)
            if m:
                sc2f[m.group(1)] = m.group(2)
            m2 = RE_SP.match(ln)
            if m2:
                scp[m2.group(1)] = m2.group(2)
        m = RE_SCN.match(ln)
        if m:
            scn[m.group(1)] = m.group(2)


def base(p):
    return os.path.basename(p.replace('\\', '/'))


def chain(mid):
    out = []
    d = 0
    while mid in meta and d < 16:
        line, sc, inl = meta[mid]
        sc0, d2 = sc, 0
        f = '?'
        while sc0 and d2 < 12:
            if sc0 in sc2f:
                f = files.get(sc2f[sc0], '?')
                break
            sc0 = scp.get(sc0)
            d2 += 1
        out.append('%s:%d[%s]' % (base(f), line, scn.get(sc, '')))
        if not inl:
            break
        mid = inl
        d += 1
    return ' <- '.join(out)


for a in sys.argv[2:]:
    if a.startswith('!'):
        print(a, chain(a))
    else:
        L = int(a)
        m = re.search(r'!dbg (![0-9]+)', lines[L - 1])
        print('%-8s %s' % (L, chain(m.group(1)) if m else 'no dbg'))
