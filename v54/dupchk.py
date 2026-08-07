# -*- coding: utf-8 -*-
"""중복 설명 — Rust match 는 **앞의 arm** 만 쓴다. 앞이 낡은 쪽이면 사용자는 틀린 설명을 본다."""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

t = io.open('C:/tfm2mods/ai_adjust_editor/src/main.rs', encoding='utf-8').read()
KEY = re.compile(r'"([a-z][a-zA-Z0-9_]{2,})"\s*=>\s*"')

occ = {}
for m in KEY.finditer(t):
    s0 = m.end(); j, n = s0, len(t)
    while j < n:
        if t[j] == '\\': j += 2; continue
        if t[j] == '"': break
        j += 1
    body = t[s0:j]
    if re.search(r'[가-힣]', body):
        occ.setdefault(m.group(1), []).append(body)

diff, same = [], 0
for k, v in sorted(occ.items()):
    if len(v) < 2:
        continue
    if len(set(v)) == 1:
        same += 1
    else:
        diff.append((k, v))

print('설명 중복 키 = %d (내용 동일 %d / 내용 다름 %d)' % (len(diff) + same, same, len(diff)))
print('\n== 내용이 다른 중복 — 앞의 것이 화면에 나온다 ==')
for k, v in diff:
    print('\n[%s]' % k)
    for i, b in enumerate(v):
        mark = '★화면' if i == 0 else '  가림'
        print('  %s(%3d자) %s' % (mark, len(b), b[:96].replace('\n', ' ')))
