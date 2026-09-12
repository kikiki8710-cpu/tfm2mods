import json, io, sys
d = json.load(io.open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json', encoding='utf-8'))
from collections import Counter
tot = Counter()
for i in range(0, 5):
    s = d['specs'][i]
    print('=' * 80)
    print('specs[%d] %s' % (i, s['id']))
    for fld in ('mem', 'consts', 'knobs', 'open', 'notes'):
        v = s.get(fld) or []
        for j, r in enumerate(v):
            ev = r.get('ev')
            tot[str(ev)] += 1
            key = r.get('name') or r.get('what') or r.get('q') or ''
            val = r.get('offset') if fld == 'mem' else r.get('value')
            print('  %-7s[%2d] ev=%-4s %-22s %s' % (fld, j, ev, str(val)[:22], str(key)[:70]))
    # sig.params
    for p in (s.get('sig') or {}).get('params') or []:
        tot[str(p.get('ev'))] += 1
    tot[str((s.get('sig') or {}).get('ev'))] += 1
print('=' * 80)
print('배치 A ev 분포(mem/consts/knobs/open/notes + sig+params):', dict(sorted(tot.items())))
n = sum(tot.values())
ge4 = tot['4'] + tot['5']
print('총 %d행, ev>=4 %d행 (%.1f%%), ev2 %d행, ev3 %d행' % (n, ge4, 100.0 * ge4 / n, tot['2'], tot['3']))
