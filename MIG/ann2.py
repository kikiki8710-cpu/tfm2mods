"""ann2.py <ll> <line> [before] [after] — 저장지점 주변을 요약해서 본다(호출 이름 전개)."""
import io, os, sys, re, subprocess
sys.argv = sys.argv
if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

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


SYM = re.compile(r'@(_RN[A-Za-z0-9_$.]+)')


def shorten(s):
    def rep(m):
        n = m.group(1)
        # v0 mangling: keep alphabetic runs that follow length digits
        parts = re.findall(r'(\d+)([A-Za-z_][A-Za-z0-9_]*)', n)
        segs = []
        for ln_, txt in parts:
            k = int(ln_)
            segs.append(txt[:k])
        segs = [x for x in segs if not re.fullmatch(r'[A-Za-z]', x)]
        return '@' + '::'.join(segs[-4:])
    return SYM.sub(rep, s)


L = int(sys.argv[2])
b = int(sys.argv[3]) if len(sys.argv) > 3 else 40
a = int(sys.argv[4]) if len(sys.argv) > 4 else 6
KEEP = re.compile(r'(invoke|call |icmp|br |store|phi|select|getelementptr|load |switch|add |sub |mul |zext|trunc|^\s*\w+:)')
for i in range(L - b - 1, min(L + a, len(lines))):
    ln = lines[i]
    if '#dbg' in ln or 'llvm.lifetime' in ln or 'noalias.scope.decl' in ln:
        continue
    if not KEEP.search(ln) and ln.strip():
        continue
    m = re.search(r'!dbg (![0-9]+)', ln)
    t = tag(m.group(1)) if m else ''
    body = re.sub(r',?\s*!(dbg|alias\.scope|noalias|range|noundef|nonnull|invariant\.load|align|tbaa|prof)\s*![0-9]+', '', ln)
    body = re.sub(r'(ptr|i64|i8|i32|i1) (noalias |noundef |nonnull |readonly |align \d+ |captures\([^)]*\) |dereferenceable\(\d+\) |dereferenceable_or_null\(\d+\) |zeroext |dead_on_unwind |writable |writeonly |dead_on_return )+', r'\1 ', body)
    body = shorten(body)
    print('%-7d %-110s %s' % (i + 1, body[:110], t))
