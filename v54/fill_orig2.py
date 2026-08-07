# -*- coding: utf-8 -*-
"""원본값 2차 보강.
   ①토글: 숫자가 아니라 상태다 — 설명의 `0(기본)` / `1(기본)` / `기본 off` 에서 뽑아 켜짐/꺼짐으로.
   ②사이트마다 원본이 다른 키: 1차에서 '값이 하나여야 한다'는 조건에 걸려 빠졌다 — `a / b` 로 병기.
   ⚠여전히 근거가 없으면 손대지 않는다."""
import sys, io, re, os, glob, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
in_tab, desc, orig = set(st['in_tab']), st['desc'], st['orig']

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
src = ''
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    src += '\n' + io.open(f, encoding='utf-8').read()

multi = {}
for fm in re.finditer(r'\nunsafe fn (\w+)\(\)\s*\{', src):
    s0 = fm.end(); nx = src.find('\nunsafe fn ', s0)
    body = src[s0:nx if nx > 0 else len(src)]
    v2k = dict(re.findall(r'let\s+(\w+)\s*=\s*tune\(\s*"([a-zA-Z0-9_]+)"', body))
    for var, key in v2k.items():
        vals = []
        for o in re.findall(r'\bb[14]\(\s*%s\s*,\s*([0-9_]+|0x[0-9a-fA-F_]+)\s*\)' % re.escape(var), body):
            o = o.replace('_', '')
            vals.append(int(o, 16) if o.lower().startswith('0x') else int(o))
        if vals:
            uniq = sorted(set(vals))
            multi[key] = ' / '.join(str(x) for x in uniq)

# 토글 판정: 설명에 `N(기본)` 또는 `기본 off/0` 이 있으면 상태로 표기
TOG_N = re.compile(r'([01])\s*\(\s*기본\s*\)')
TOG_OFF = re.compile(r'기본\s*(?:=\s*)?(?:off|OFF|0\b|꺼짐)')
TOG_ON = re.compile(r'기본\s*(?:=\s*)?(?:on|ON|1\b|켜짐)')

add, skip = {}, []
for k in sorted(in_tab):
    if k in orig:
        continue
    d = desc.get(k, '')
    m = TOG_N.search(d)
    if m:
        add[k] = '켜짐' if m.group(1) == '1' else '꺼짐'
    elif TOG_OFF.search(d):
        add[k] = '꺼짐'
    elif TOG_ON.search(d):
        add[k] = '켜짐'
    elif k in multi:
        add[k] = multi[k]
    else:
        skip.append(k)

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
anc = ' // ★[08-06] 원본값 자동 보강'
assert anc in t
ins = (' // ★[08-06] 2차 보강 — 토글은 켜짐/꺼짐, 사이트마다 다른 값은 병기.\n'
       + ''.join(' "%s" => "%s",\n' % (k, v) for k, v in sorted(add.items())))
io.open(P, 'w', encoding='utf-8', newline='\n').write(t.replace(anc, ins + anc, 1))

print('2차 보강 %d개' % len(add))
print('\n끝내 근거가 없어 비워 둔 것 %d개:' % len(skip))
for i in range(0, len(skip), 6):
    print('   ' + '  '.join(skip[i:i + 6]))
