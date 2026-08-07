# -*- coding: utf-8 -*-
"""cfg 의 `-1`(=원본 유지)을 **실제 원본 숫자**로 펼친다.

⚠안전 조건을 만족하는 것만 바꾼다. 아래에 해당하면 `-1` 그대로 둔다 —
  ①원본값이 숫자가 아님(`주사위 굴림`·`검열 함`·`약 387`·`10 / 12 / 15` 등)
  ②원본값을 모름
  ③**인코딩 변환이 걸린 키**(제곱 저장/자동 변환) — 편집기가 보여주는 원본은 '변환 전' 숫자라
    그대로 cfg 에 적으면 전혀 다른 값이 된다. 예: `nxd_near_dist` 원본 표기 120000, 실제 cfg 값 14400000001(=120000²+1)
  ④토글이지만 기본 상태를 숫자로 확정 못 하는 것
"""
import sys, io, re, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

# 편집기 orig_val 원문(비숫자 포함) + 설명
ed = io.open('C:/tfm2mods/ai_adjust_editor/src/main.rs', encoding='utf-8').read()
m = re.search(r'\nfn orig_val\(', ed)
nx = re.search(r'\nfn \w+\(', ed[m.end():])
seg = ed[m.start(): m.end() + (nx.start() if nx else 0)]
ORIG = {}
for mm in re.finditer(r'"([a-z][a-zA-Z0-9_]{2,})"\s*=>\s*"([^"]*)"', seg):
    ORIG.setdefault(mm.group(1), mm.group(2))
for mm in re.finditer(r'((?:"[a-z][a-zA-Z0-9_]{2,}"\s*\|\s*)+"[a-z][a-zA-Z0-9_]{2,}")\s*=>\s*"([^"]*)"', seg):
    for k in re.findall(r'"([^"]+)"', mm.group(1)):
        ORIG.setdefault(k, mm.group(2))

st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
desc = st['desc']
ENC = re.compile(r'제곱|인코딩|자동 변환|거리\s*제곱')
NUM = re.compile(r'^-?\d+$')
CLS = re.compile(r'_class_(melee|range|magician|util|assassin)$')

SRC = (r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2'
       r'\mods\tfm2_ai_adjust\config\테스트C.cfg')
t = io.open(SRC, encoding='utf-8').read()

out, done, kept = [], 0, []
for ln in t.split('\n'):
    s = ln.strip()
    if not s or s.startswith('#') or '=' not in s:
        out.append(ln); continue
    k, v = [x.strip() for x in s.split('=', 1)]
    if v != '-1':
        out.append(ln); continue
    base = CLS.sub('', k)
    o = ORIG.get(base)
    d = desc.get(base, '')
    if o is None or not NUM.match(o):
        kept.append((k, o or '(원본값 미상)')); out.append(ln); continue
    if ENC.search(d):
        kept.append((k, o + ' ⚠인코딩 변환')); out.append(ln); continue
    out.append(re.sub(r'=\s*-1\s*$', '= %s' % o, ln))
    done += 1

body = '\n'.join(out)
io.open(SRC, 'w', encoding='utf-8', newline='\n').write(body)
print('-1 → 실제 원본값 치환 %d개' % done)
print('\n그대로 둔 %d개 (사유 붙임):' % len(kept))
for k, r in kept:
    print('   %-26s %s' % (k, r))
