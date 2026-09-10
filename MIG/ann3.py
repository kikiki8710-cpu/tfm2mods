"""ann3.py <ll> <line[,line...]> [before] [after] — 저장지점 주변 요약(호출/분기/self-store 만)."""
import io, os, sys, re
if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

RE_DL = re.compile(r'^(![0-9]+) = !DILocation\(line: (\d+)[^\n]*?(?:scope: (![0-9]+))?(?:.*?inlinedAt: (![0-9]+))?\)')
RE_SCF = re.compile(r'^(![0-9]+) = (?:distinct )?!DI(?:Subprogram|LexicalBlock|LexicalBlockFile)\([^\n]*?file: (![0-9]+)')
RE_F = re.compile(r'^(![0-9]+) = !DIFile\(filename: "([^"]*)"')

fn = sys.argv[1]
lines = io.open(fn, encoding='utf-8', errors='replace').read().split('\n')
meta, sc2f, files = {}, {}, {}
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


def tag(mid):
    out = []
    d = 0
    while mid in meta and d < 8:
        line, sc, inl = meta[mid]
        f = files.get(sc2f.get(sc, ''), '?')
        out.append('%s:%d' % (os.path.basename(f.replace('\\', '/')), line))
        if not inl:
            break
        mid = inl
        d += 1
    return '<'.join(out)


SYM = re.compile(r'@(_R[A-Za-z0-9_$.]+)')


def clean(body):
    body = re.sub(r',?\s*!(dbg|alias\.scope|noalias|range|noundef|nonnull|invariant\.load|align|tbaa|prof)\s*![0-9]+', '', body)
    body = re.sub(r'(noalias |noundef |nonnull |readonly |writeonly |captures\([^)]*\) |dereferenceable\(\d+\) |dereferenceable_or_null\(\d+\) |zeroext |dead_on_unwind |writable |dead_on_return |inbounds |nuw |nsw |tail |fastcc |align \d+)', '', body)
    body = SYM.sub(lambda m: '@' + m.group(1)[-52:], body)
    return body.strip()


b = int(sys.argv[3]) if len(sys.argv) > 3 else 30
a = int(sys.argv[4]) if len(sys.argv) > 4 else 8
for L in [int(x) for x in sys.argv[2].split(',')]:
    print('########## %s:%d' % (os.path.basename(fn), L))
    for i in range(max(0, L - b - 1), min(L + a, len(lines))):
        ln = lines[i]
        if '#dbg' in ln or 'llvm.lifetime' in ln or 'noalias.scope.decl' in ln or 'llvm.assume' in ln:
            continue
        keep = False
        if re.search(r'(invoke|call) ', ln) and 'llvm.' not in ln:
            keep = True
        if re.search(r'(icmp|br |switch|select|phi|ret )', ln):
            keep = True
        if re.match(r'^\s*store ', ln) and re.search(r'ptr %\d+, align|ptr %\w+,', ln):
            keep = True
        if re.search(r'ptr %0, i64 \d+', ln):
            keep = True
        if re.match(r'^\w+:', ln.strip()) and ';' in ln:
            keep = True
        if not keep:
            continue
        m = re.search(r'!dbg (![0-9]+)', ln)
        t = tag(m.group(1)) if m else ''
        print('%-7d %-112s %s' % (i + 1, clean(ln)[:112], t))
    print()
