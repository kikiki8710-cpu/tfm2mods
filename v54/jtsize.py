# -*- coding: utf-8 -*-
"""★JT 실엔트리수 = '다음 JT 베이스까지의 간격'으로 확정.
   기존 jtscan 의 엔트리수는 '타깃이 함수 밖으로 나갈 때까지' 걸어서 세므로
   거대 함수에서는 **다음 테이블까지 먹어 들어가 과대계상**된다(054 handler.rs 실사고).
  python jtsize.py 054            # 전체 표 재계산 → *_jtsize.tsv
  python jtsize.py 054 e939b0     # 그 함수의 JT 만
"""
import io, os, sys, struct
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load
D = r'C:\tfm2mods\v54'
ver = sys.argv[1]
only = int(sys.argv[2], 16) if len(sys.argv) > 2 else None
rows = []
for ln in io.open(os.path.join(D, '%s_jt.tsv' % ver), encoding='utf-8'):
    p = ln.rstrip('\n').split('\t')
    rows.append((int(p[0], 16), int(p[1], 16), int(p[2], 16), p[3], p[4], int(p[5]), p[6]))
bases = sorted(set(r[1] for r in rows))
nxt = {b: (bases[i + 1] if i + 1 < len(bases) else b + 4096) for i, b in enumerate(bases)}
e = load(ver)
out = []
for jmp, jt, fn, br, ir, oldn, src in sorted(rows):
    if only and fn != only:
        continue
    n = (nxt[jt] - jt) // 4
    ent = []
    for k in range(min(n, 300)):
        v = e.u32(jt + 4 * k)
        d = struct.unpack('<i', struct.pack('<I', v))[0]
        ent.append(jt + d)
    out.append((jmp, jt, fn, n, oldn, len(set(ent)), src))
if only:
    for jmp, jt, fn, n, oldn, uq, src in out:
        print('jmp %06x  JT %06x  실엔트리 %-4d (구스캐너 %-4d)  서로다른타깃 %d' % (jmp, jt, n, oldn, uq))
else:
    with io.open(os.path.join(D, '%s_jtsize.tsv' % ver), 'w', encoding='utf-8') as f:
        for jmp, jt, fn, n, oldn, uq, src in out:
            f.write('%06x\t%06x\t%06x\t%d\t%d\t%d\t%s\n' % (jmp, jt, fn, n, oldn, uq, src))
    print('%d행 기록. 과대계상(구>실) %d건' % (len(out), sum(1 for r in out if r[4] > r[3])))
