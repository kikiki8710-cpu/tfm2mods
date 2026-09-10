"""ann.py <ll> <start> <end> — IR 를 소스줄 주석과 함께 덤프(inlinedAt 루트 + 인라인 체인)."""
import io, os, sys, re
if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
RE_DL = re.compile(r'^(![0-9]+) = !DILocation\(line: (\d+)[^\n]*?(?:scope: (![0-9]+))?(?:.*?inlinedAt: (![0-9]+))?\)')
RE_SC = re.compile(r'^(![0-9]+) = (?:distinct )?!DI(?:Subprogram|LexicalBlock|LexicalBlockFile)\([^\n]*?scope: (![0-9]+)')
RE_SCF = re.compile(r'^(![0-9]+) = (?:distinct )?!DI(?:Subprogram|LexicalBlock|LexicalBlockFile)\([^\n]*?file: (![0-9]+)')
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
        m = RE_SCF.match(ln)
        if m:
            sc2f[m.group(1)] = m.group(2)
        m = RE_SC.match(ln)
        if m:
            scp[m.group(1)] = m.group(2)
        m = RE_SCN.match(ln)
        if m:
            scn[m.group(1)] = m.group(2)


def sname(sc, d=0):
    while sc and d < 12:
        if sc in scn:
            return scn[sc]
        sc = scp.get(sc)
        d += 1
    return ''


def tag(mid):
    out = []
    d = 0
    while mid in meta and d < 8:
        line, sc, inl = meta[mid]
        f = files.get(sc2f.get(sc, ''), '?')
        out.append('%s:%d' % (os.path.basename(f.replace('\\', '/')), line))
        nm = sname(sc)
        if nm:
            out[-1] += '(%s)' % nm
        if not inl:
            break
        mid = inl
        d += 1
    return ' < '.join(out)


a, b = int(sys.argv[2]), int(sys.argv[3])
for i in range(a - 1, min(b, len(lines))):
    ln = lines[i]
    m = re.search(r'!dbg (![0-9]+)', ln)
    t = tag(m.group(1)) if m else ''
    body = re.sub(r',?\s*!(dbg|alias\.scope|noalias|range|noundef|nonnull|invariant\.load|align|tbaa|prof|srcloc|annotation|misexpect)\s*![0-9]+', '', ln)
    print('%-7d %-96s %s' % (i + 1, body[:96], t))
