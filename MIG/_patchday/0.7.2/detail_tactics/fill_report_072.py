# -*- coding: utf-8 -*-
"""24_0.7.2_마이그.md 의 {{TABLE}} 을 전술 판정표로 채운다(다시 돌리면 표를 새로 만든다)."""
import glob
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
DOC = os.path.join(r'F:\claude\tfm2\.claude\worktrees\salary-transfer-fee-analysis-1bf168', 'mods_report', 'tfm2_detail_tactics', '24_0.7.2_마이그.md')
V = {}
for p in sorted(glob.glob(os.path.join(HERE, 're', 'G[1-6].json'))):
    d = json.load(open(p, encoding='utf-8'))
    for k, t in d['tactics'].items():
        t['g'] = os.path.basename(p)[:2]
        V[k] = t
T = json.load(open(os.path.join(HERE, 'track.json'), encoding='utf-8'))
nwin = {}
for w in T['windows']:
    nwin[w['tactic_id']] = nwin.get(w['tactic_id'], 0) + 1
rows = ['| # | 전술 | 탭 | 창 | 추적기 | 등급 | spec | 처리 | 그룹 |', '|---|---|---|---|---|---|---|---|---|']
for t in T['tactics']:
    v = V[t['id']]
    rows.append('| %d | `%s` | %s | %d | %s | **%s** | %s | %s | %s |' % (
        t['index'], t['id'], t['tab'] or '숨김', nwin.get(t['id'], 0), t['worst'], v['grade'], v['spec'], v['proposal']['action'], v['g']))
TABLE = '\n'.join(rows)
s = open(DOC, encoding='utf-8').read()
if '{{TABLE}}' in s:
    s = s.replace('{{TABLE}}', TABLE)
else:
    s = re.sub(r'\| # \| 전술 \| 탭 \|.*?\n(?=\n## 5)', TABLE + '\n', s, flags=re.S)
open(DOC, 'w', encoding='utf-8', newline='\n').write(s)
print(len(rows) - 2)
