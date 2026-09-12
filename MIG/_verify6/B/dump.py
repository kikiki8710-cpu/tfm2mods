# -*- coding: utf-8 -*-
# 배치B 전용 요약 덤퍼 — siblings/callees 는 자동생성이라 접는다.
import json, io, sys

d = json.load(io.open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json', encoding='utf-8'))
out = []
idxs = [int(x) for x in sys.argv[1:]] or [5, 6, 7, 8, 9]
for i in idxs:
    s = d['specs'][i]
    out.append('=' * 70)
    out.append('### specs[%d] %s  (%s:%s)' % (i, s['name'], s['src'], s.get('src_line')))
    out.append('sym: %s' % s.get('sym'))
    out.append('ir: %s' % json.dumps(s.get('ir'), ensure_ascii=False))
    out.append('one_line: %s' % s.get('one_line'))
    sig = s.get('sig', {})
    out.append('sig.tcx: %s' % sig.get('tcx'))
    out.append('sig.vis: %s  path: %s  mir: %s  ev: %s' % (sig.get('vis'), sig.get('path'), sig.get('mir'), sig.get('ev')))
    for p in sig.get('params', []):
        out.append('  p%s %s : %s  [ev%s] %s' % (p.get('i'), p.get('name'), p.get('type'), p.get('ev'), p.get('role')))
    out.append('  ret: %s' % sig.get('ret'))
    out.append('--- logic ---')
    out.append(s.get('logic', ''))
    out.append('--- mem (%d) ---' % len(s.get('mem', [])))
    for j, m in enumerate(s.get('mem', [])):
        out.append('  mem[%d] ev%s %s %s+%s = %s | chk=%s | %s' % (
            j, m.get('ev'), m.get('dir'), m.get('base'), m.get('offset'), m.get('name'), m.get('chk'), m.get('note', '')))
        if m.get('value'):
            out.append('          value: %s' % m.get('value'))
    out.append('--- consts (%d) ---' % len(s.get('consts', [])))
    for j, c in enumerate(s.get('consts', [])):
        out.append('  consts[%d] ev%s value=%s line=%s kind=%s | %s' % (
            j, c.get('ev'), c.get('value'), c.get('src_line'), c.get('kind'), c.get('meaning')))
    out.append('--- knobs (%d) ---' % len(s.get('knobs', [])))
    for j, k in enumerate(s.get('knobs', [])):
        out.append('  knobs[%d] ev%s [%s] %s @ %s value=%s' % (
            j, k.get('ev'), k.get('src'), k.get('what'), k.get('where'), k.get('value')))
        out.append('          effect: %s' % k.get('effect'))
    out.append('--- open (%d) ---' % len(s.get('open', [])))
    for j, o in enumerate(s.get('open', [])):
        out.append('  open[%d] ev%s class=%s | %s' % (j, o.get('ev'), o.get('class'), o.get('q')))
    out.append('--- notes (%d) ---' % len(s.get('notes', [])))
    for j, n in enumerate(s.get('notes', [])):
        out.append('  notes[%d] %s' % (j, json.dumps(n, ensure_ascii=False) if not isinstance(n, str) else n))
    out.append('--- closed (%d) ---' % len(s.get('closed', [])))
    for j, n in enumerate(s.get('closed', [])):
        out.append('  closed[%d] %s' % (j, json.dumps(n, ensure_ascii=False) if not isinstance(n, str) else n))
    out.append('--- history ---')
    h = s.get('history')
    out.append(json.dumps(h, ensure_ascii=False, indent=1) if h else '(없음)')
    out.append('--- callers ---')
    out.append(json.dumps(s.get('callers'), ensure_ascii=False))
    out.append('--- callees names ---')
    out.append(', '.join('%s(%s)' % (c.get('name'), c.get('vis')) for c in s.get('callees', [])))
    out.append('--- callees_unmatched ---')
    out.append(json.dumps(s.get('callees_unmatched', {}).get('names'), ensure_ascii=False))
    out.append('--- v2_extra ---')
    out.append(json.dumps(s.get('v2_extra'), ensure_ascii=False, indent=1) if s.get('v2_extra') else '(없음)')
    out.append('--- rounds/base_round ---')
    out.append('%s | %s' % (json.dumps(s.get('rounds'), ensure_ascii=False), s.get('base_round')))

sys.stdout.write('\n'.join(out))
