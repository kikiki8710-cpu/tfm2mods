# -*- coding: utf-8 -*-
"""숫자가 없는 토글형 노브의 orig_val 을 짧은 '원본 상태' 문구로 정정."""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
m = re.search(r'\nfn orig_val\(', t)
nx = re.search(r'\nfn \w+\(', t[m.end():])
s0, e0 = m.start(), m.end() + (nx.start() if nx else 0)
seg = t[s0:e0]

SHORT = {'cf_flee_kill_off': '몰살 함', 'cf_filter_off': '검열 함', 'pe_noise_exempt': '100'}
KEY = re.compile(r'"([a-z][a-zA-Z0-9_]{2,})"\s*=>\s*"')
out = []
for mm in KEY.finditer(seg):
    k = mm.group(1)
    if k not in SHORT:
        continue
    p = mm.end(); j, n = p, len(seg)
    while j < n:
        if seg[j] == '\\':
            j += 2; continue
        if seg[j] == '"':
            break
        j += 1
    body = seg[p:j]
    if len(body) > 30 or '**' in body:
        out.append((p, j, SHORT[k], k, len(body)))

for p, j, v, k, ln in reversed(out):
    seg = seg[:p] + v + seg[j:]
io.open(P, 'w', encoding='utf-8', newline='\n').write(t[:s0] + seg + t[e0:])
for _, _, v, k, ln in out:
    print('  %-20s 설명문(%d자) -> %s' % (k, ln, v))
print('토글 %d건 정정' % len(out))
