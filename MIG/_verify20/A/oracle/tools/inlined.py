# -*- coding: utf-8 -*-
"""본문 !dbg 사슬의 인라인 프레임(DISubprogram 이름·파일·호출 줄)을 집계 — callees ev4 행의 '인라인' 확정 근거."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import irlib
SPEC = json.load(open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json', encoding='utf-8'))['specs']
idx = int(sys.argv[1])
s = SPEC[idx]
ranges = [(s['ir']['file'], s['ir']['frm'], s['ir']['to'])] + [(x['file'], x['frm'], x['to']) for x in s['ir'].get('aux', [])]
agg = collections.defaultdict(set)
for (f, a, b) in ranges:
    lines, md = irlib.load(f)
    seen = set()
    for k in range(a - 1, b):
        d = irlib.dbg_of(lines[k])
        if d is None or d in seen:
            continue
        seen.add(d)
        ch = irlib.chain(md, d)
        for j, (l, fn, fb) in enumerate(ch[:-1]):
            base = re.split(r'[\\/]', fb)[-1]
            if 'library' in fb or 'rustc' in fb or '.cargo' in fb:
                continue
            caller = ch[j + 1]
            agg[(re.sub(r'<.*', '', fn), base)].add('%s:%d' % (re.split(r'[\\/]', caller[2])[-1], caller[0]))
for (fn, base), sites in sorted(agg.items()):
    print('%-45s %-22s %s' % (fn, base, ' '.join(sorted(sites))[:160]))
