# -*- coding: utf-8 -*-
import io, os, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
D = 'C:/tfm2mods/v54'
SEP = chr(92)

def rows(ver):
    out = []
    for ln in io.open(os.path.join(D, '%s_srcmap.tsv' % ver), encoding='utf-8'):
        s, e, src, lines = ln.rstrip('\n').split('\t')
        out.append((int(s,16), int(e,16), src, set(int(x) for x in lines.split(',') if x)))
    return out

R = {'053': rows('053'), '054': rows('054')}

def sel(ver, pat):
    return [r for r in R[ver] if pat.lower() in r[2].lower()]

def short(src):
    return ' | '.join(p.split(SEP)[-1] for p in src.split(' | '))

def go(pat):
    a = sorted(sel('053', pat), key=lambda r: -(r[1]-r[0]))
    b = sorted(sel('054', pat), key=lambda r: -(r[1]-r[0]))
    print('#### %s   053 %dfn/%dB  ->  054 %dfn/%dB' % (pat, len(a), sum(x[1]-x[0] for x in a), len(b), sum(x[1]-x[0] for x in b)))
    used = set()
    for s, e, src, ls in a:
        best = None
        for j, (s2, e2, src2, ls2) in enumerate(b):
            if j in used: continue
            hit = sum(1 for x in ls if any(abs(x-y) <= 4 for y in ls2))
            sc = (hit, -abs((e-s)-(e2-s2)))
            if hit and (best is None or sc > best[0]): best = (sc, j, hit)
        if best:
            j = best[1]; used.add(j); s2, e2, src2, ls2 = b[j]
            d = (e2-s2)-(e-s)
            print('  %06x %6dB %-40s -> %06x %6dB %-40s  d%+d (%+.0f%%) line%d/%d' % (s, e-s, short(src)[:40], s2, e2-s2, short(src2)[:40], d, 100.0*d/(e-s), best[2], len(ls)))
        else:
            print('  %06x %6dB %-40s -> ***NOMATCH  lines:%s' % (s, e-s, short(src)[:40], sorted(ls)[:12]))
    for j, (s2, e2, src2, ls2) in enumerate(b):
        if j not in used:
            print('  ***054NEW: %06x %6dB %-40s  lines:%s' % (s2, e2-s2, short(src2)[:40], sorted(ls2)[:12]))
    print()

for p in sys.argv[1:]: go(p)
