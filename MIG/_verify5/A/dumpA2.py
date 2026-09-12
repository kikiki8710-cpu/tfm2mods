import json, io
P = r'C:\tfm2mods\MIG\_spec\specs20_v3.json'
d = json.load(io.open(P, encoding='utf-8'))
SKIP = {'callees', 'callers', 'siblings', 'calls_raw', 'callees_unmatched'}
out = io.open(r'C:\tfm2mods\MIG\_verify5\A\specA_core.txt', 'w', encoding='utf-8')
for i in range(0, 5):
    s = d['specs'][i]
    out.write('=' * 90 + '\n')
    out.write('### specs[%d] %s / %s\n' % (i, s.get('id'), s.get('name')))
    for k, v in s.items():
        if k in SKIP:
            out.write('-- %s: (skipped, n=%s)\n' % (k, len(v) if isinstance(v, (list, dict)) else '?'))
            continue
        out.write('-- %s:\n' % k)
        if isinstance(v, str):
            out.write(v + '\n')
        else:
            out.write(json.dumps(v, ensure_ascii=False, indent=1) + '\n')
out.close()
# ev histogram per spec
def evrows(s):
    rows = []
    for fld in ('mem', 'consts', 'knobs', 'open', 'closed', 'notes', 'history'):
        v = s.get(fld)
        if isinstance(v, list):
            for j, r in enumerate(v):
                if isinstance(r, dict):
                    rows.append((fld, j, r.get('ev')))
    return rows
print('spec | total | ev counts')
for i in range(0, 5):
    s = d['specs'][i]
    rows = evrows(s)
    from collections import Counter
    c = Counter(str(r[2]) for r in rows)
    print(i, s['id'], len(rows), dict(sorted(c.items())))
