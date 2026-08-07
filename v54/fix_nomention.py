# -*- coding: utf-8 -*-
"""설명에 원본값이 안 적힌 노브 — 코드의 b1/b4(var, ORIG) 에서 실제 원본값을 뽑아 설명 끝에 덧붙인다.
   ★사용자가 '원래 얼마였는지'를 모르면 되돌릴 수도, 얼마나 바꾸는지도 가늠할 수 없다."""
import sys, io, re, os, glob
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
src = ''
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    src += '\n' + io.open(f, encoding='utf-8').read()

code_orig = {}
for fm in re.finditer(r'\nunsafe fn (\w+)\(\)\s*\{', src):
    s0 = fm.end(); nx = src.find('\nunsafe fn ', s0)
    body = src[s0:nx if nx > 0 else len(src)]
    v2k = dict(re.findall(r'let\s+(\w+)\s*=\s*tune\(\s*"([a-zA-Z0-9_]+)"', body))
    for var, key in v2k.items():
        vals = set()
        for o in re.findall(r'\bb[14]\(\s*%s\s*,\s*([0-9_]+|0x[0-9a-fA-F_]+)\s*\)' % re.escape(var), body):
            o = o.replace('_', '')
            vals.add(int(o, 16) if o.lower().startswith('0x') else int(o))
        if len(vals) == 1:
            code_orig[key] = vals.pop()

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
m = re.search(r'\nfn desc_static\(', t)
nx = re.search(r'\nfn \w+\(', t[m.end():])
s0, e0 = m.start(), m.end() + (nx.start() if nx else len(t) - m.end())
seg = t[s0:e0]

KEY = re.compile(r'"([a-z][a-zA-Z0-9_]{2,})"\s*=>\s*"')
DIG = re.compile(r'원본\s*(?:값\s*)?\**\s*(0[xX][0-9a-fA-F]+|[0-9][0-9,]*)')
edits, skipped = [], []
for mm in KEY.finditer(seg):
    k = mm.group(1)
    if k not in code_orig:
        continue
    p = mm.end(); j, n = p, len(seg)
    while j < n:
        if seg[j] == '\\':
            j += 2; continue
        if seg[j] == '"':
            break
        j += 1
    body = seg[p:j]
    if not re.search(r'[가-힣]', body) or DIG.search(body):
        continue
    edits.append((p, j, body, code_orig[k], k))

for p, j, body, val, k in reversed(edits):
    add = ' (원본 %d)' % val
    nb = body.rstrip()
    if nb.endswith('-1=원본'):
        nb = nb[:-len('-1=원본')].rstrip() + add + ' -1=원본'
    else:
        nb = nb + add
    seg = seg[:p] + nb + seg[j:]

io.open(P, 'w', encoding='utf-8', newline='\n').write(t[:s0] + seg + t[e0:])
print('원본값 덧붙임 %d건' % len(edits))
for _, _, _, v, k in edits:
    print('   %-22s (원본 %s)' % (k, v))
