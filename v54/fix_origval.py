# -*- coding: utf-8 -*-
"""orig_val() 안에 잘못 들어간 '설명문 문단'을 실제 원본값(숫자)으로 정정한다.
   증상: 편집기의 '원본값' 자리에 설명 문단 전체가 표시된다.
   방법: 그 문단 안의 `원본 N` / `원본 0xNN` 을 뽑아 숫자로 교체. 못 뽑으면 손대지 않고 보고."""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()

m = re.search(r'\nfn orig_val\(', t)
nxt = re.search(r'\nfn \w+\(', t[m.end():])
s0, e0 = m.start(), m.end() + (nxt.start() if nxt else 0)
seg = t[s0:e0]

KEY = re.compile(r'"([a-z][a-zA-Z0-9_]{2,})"\s*=>\s*"')
NUM = re.compile(r'원본\s*(?:값\s*)?\**\s*(0[xX][0-9a-fA-F]+|[0-9][0-9,]*)')

fixed, skipped = [], []
out, pos = [], 0
for mm in KEY.finditer(seg):
    p = mm.end(); j, n = p, len(seg)
    while j < n:
        if seg[j] == '\\': j += 2; continue
        if seg[j] == '"': break
        j += 1
    body = seg[p:j]
    if len(body) <= 30 and '**' not in body:
        continue                                   # 정상(숫자 또는 짧은 문구)
    mn = NUM.search(body)
    if not mn:
        # 2차: 문단 끝에 숫자만 덧붙은 형태(⛔설명 … 150000)
        mt = re.search(r'(\d[\d,]*)\s*$', body.strip())
        if mt:
            mn = mt
        else:
            skipped.append((mm.group(1), body[:60]))
            continue
    raw = mn.group(1).replace(',', '')
    val = str(int(raw, 16)) if raw.lower().startswith('0x') else str(int(raw))
    out.append((p, j, val))
    fixed.append((mm.group(1), val, len(body)))

for p, j, val in reversed(out):
    seg = seg[:p] + val + seg[j:]
t = t[:s0] + seg + t[e0:]
io.open(P, 'w', encoding='utf-8', newline='\n').write(t)

print('원본값 맵 정정 %d건' % len(fixed))
for k, v, ln in fixed:
    print('   %-22s 설명문(%3d자) -> %s' % (k, ln, v))
print('\n원본값을 못 뽑아 손대지 않음 %d건' % len(skipped))
for k, b in skipped:
    print('   %-22s %s' % (k, b))
