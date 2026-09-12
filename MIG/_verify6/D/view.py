import json, io, sys
d = json.load(io.open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json', encoding='utf-8'))
idx = int(sys.argv[1])
what = sys.argv[2] if len(sys.argv) > 2 else 'all'
s = d['specs'][idx]
out = []
if what in ('all', 'head'):
    out.append('=== %d %s  src=%s:%s  ir=%s' % (idx, s['name'], s['src'], s.get('src_line'), s.get('ir')))
    out.append('one_line: ' + s.get('one_line', ''))
    sig = s.get('sig', {})
    out.append('sig.tcx: ' + str(sig.get('tcx')))
    out.append('sig.vis: %s  path=%s  mir=%s ev=%s' % (sig.get('vis'), sig.get('path'), sig.get('mir'), sig.get('ev')))
    for p in sig.get('params', []):
        out.append('  p%s %s : %s  [ev%s] %s' % (p.get('i'), p.get('name'), p.get('type'), p.get('ev'), p.get('role')))
    out.append('ret: ' + str(sig.get('ret')))
if what in ('all', 'logic'):
    out.append('--- logic ---')
    out.append(s.get('logic', ''))
if what in ('all', 'mem'):
    out.append('--- mem (%d) ---' % len(s.get('mem', [])))
    for j, m in enumerate(s.get('mem', [])):
        out.append('[%d] ev%s %s +%s %s dir=%s chk=%s val=%s :: %s' % (
            j, m.get('ev'), m.get('base'), m.get('offset'), m.get('name'), m.get('dir'), m.get('chk'), m.get('value'), m.get('note')))
if what in ('all', 'consts'):
    out.append('--- consts (%d) ---' % len(s.get('consts', [])))
    for j, c in enumerate(s.get('consts', [])):
        out.append('[%d] ev%s val=%s line=%s kind=%s :: %s' % (j, c.get('ev'), c.get('value'), c.get('src_line'), c.get('kind'), c.get('meaning')))
if what in ('all', 'knobs'):
    out.append('--- knobs (%d) ---' % len(s.get('knobs', [])))
    for j, k in enumerate(s.get('knobs', [])):
        out.append('[%d] ev%s what=%s where=%s value=%s src=%s :: %s' % (j, k.get('ev'), k.get('what'), k.get('where'), k.get('value'), k.get('src'), k.get('effect')))
if what in ('all', 'rest'):
    for key in ('open', 'closed', 'notes', 'callers', 'siblings', 'history', 'exe', 'callees_unmatched', 'callees_note', 'base_round', 'rounds'):
        v = s.get(key)
        if not v:
            continue
        out.append('--- %s ---' % key)
        if isinstance(v, list):
            for j, x in enumerate(v):
                out.append('[%d] %s' % (j, json.dumps(x, ensure_ascii=False) if not isinstance(x, str) else x))
        else:
            out.append(json.dumps(v, ensure_ascii=False))
if what == 'callees':
    out.append('--- callees (%d) ---' % len(s.get('callees', [])))
    for j, c in enumerate(s.get('callees', [])):
        out.append('[%d] %s | %s | %s | %s | %s mir=%s xinl=%s ev=%s' % (j, c.get('name'), c.get('path'), c.get('vis'), c.get('sig'), c.get('at'), c.get('mir'), c.get('xinl'), c.get('ev')))
print('\n'.join(out))
