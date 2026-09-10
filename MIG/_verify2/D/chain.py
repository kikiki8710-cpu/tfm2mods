# -*- coding: utf-8 -*-
u"""chain.py — !dbg id 의 인라인 체인을 DISubprogram 이름까지 붙여 펼친다.
사용: python -X utf8 chain.py <ll> <id,id,...>
"""
import io, re, sys, os
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
path = sys.argv[1]
ids = [int(x) for x in sys.argv[2].split(',')]
md = {}
rx = re.compile(r'^!(\d+) = (.*)$')
with io.open(path, encoding='utf-8', errors='replace') as f:
    for ln in f:
        if ln.startswith('!'):
            m = rx.match(ln.rstrip('\n'))
            if m:
                md[int(m.group(1))] = m.group(2)
RXFN = re.compile(r'filename:\s*"([^"]*)"')


def sname(sid, d=0):
    while sid in md and d < 20:
        s = md[sid]
        m = re.search(r'name:\s*"([^"]*)"', s)
        if s.startswith('!DISubprogram') and m:
            f = re.search(r'file:\s*!(\d+)', s)
            l = re.search(r'line:\s*(\d+)', s)
            fn = ''
            if f:
                fs = md.get(int(f.group(1)), '')
                mm = RXFN.search(fs)
                fn = mm.group(1) if mm else ''
            return '%s (%s:%s)' % (m.group(1), os.path.basename(fn.replace('\\', '/')), l.group(1) if l else '?')
        m2 = re.search(r'scope:\s*!(\d+)', s)
        if not m2:
            return '?'
        sid = int(m2.group(1))
        d += 1
    return '?'


for i in ids:
    cur = i
    out = []
    for _ in range(40):
        s = md.get(cur, '')
        m = re.search(r'line:\s*(\d+).*?scope:\s*!(\d+)(?:.*?inlinedAt:\s*!(\d+))?', s)
        if not m:
            break
        out.append('L%s@%s' % (m.group(1), sname(int(m.group(2)))))
        if m.group(3):
            cur = int(m.group(3))
            continue
        break
    print('!%d: %s' % (i, '  <=  '.join(out)))
