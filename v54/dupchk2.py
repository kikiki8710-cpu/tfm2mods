# -*- coding: utf-8 -*-
"""함수 범위를 갈라 본다 — orig_val()(원본값 맵)과 설명 맵은 서로 다른 맵인데
   섞어 세면 '중복 45건' 같은 오탐이 난다. 동시에 orig_val 안에 설명문이 들어간 진짜 결함을 찾는다."""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

t = io.open('C:/tfm2mods/ai_adjust_editor/src/main.rs', encoding='utf-8').read()

fns = [(m.group(1), m.start()) for m in re.finditer(r'\nfn (\w+)\(', t)]
fns.append(('<EOF>', len(t)))
span = {}
for i in range(len(fns) - 1):
    span[fns[i][0]] = (fns[i][1], fns[i + 1][1])

KEY = re.compile(r'"([a-z][a-zA-Z0-9_]{2,})"\s*=>\s*"')


def entries(fn):
    s, e = span[fn]
    seg = t[s:e]
    out = []
    for m in KEY.finditer(seg):
        p = m.end(); j, n = p, len(seg)
        while j < n:
            if seg[j] == '\\': j += 2; continue
            if seg[j] == '"': break
            j += 1
        out.append((m.group(1), seg[p:j]))
    return out


cands = [f for f in span if re.search(r'orig|desc|help|tip|expl', f, re.I)]
print('후보 맵 함수: %s' % ', '.join(cands))
for f in cands:
    e = entries(f)
    print('  %-14s 항목 %d' % (f, len(e)))

# ① orig_val 안에 '설명문'이 들어간 것 = 원본값 자리에 문단이 표시되는 결함
print('\n== 원본값 맵인데 값이 설명문인 항목 ==')
bad = []
for f in cands:
    if 'orig' not in f.lower():
        continue
    for k, v in entries(f):
        if len(v) > 30 or '**' in v or '원본' in v:
            bad.append((f, k, v))
for f, k, v in bad:
    print('  [%s] %-20s %s' % (f, k, v[:100]))
if not bad:
    print('  없음')

# ② 설명 맵 안에서만 중복 판정
print('\n== 설명 맵 내부 중복 ==')
for f in cands:
    if 'orig' in f.lower():
        continue
    seen = {}
    for k, v in entries(f):
        seen.setdefault(k, []).append(v)
    dup = {k: v for k, v in seen.items() if len(v) > 1}
    print('  %-14s 중복 키 %d' % (f, len(dup)))
    for k, v in list(dup.items())[:12]:
        print('     %-22s %d개  앞=%d자 / 뒤=%s' % (k, len(v), len(v[0]), '/'.join(str(len(x)) for x in v[1:])))
