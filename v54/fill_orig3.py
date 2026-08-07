# -*- coding: utf-8 -*-
"""원본값 3차 보강 — ★`tune("키", 원본값)` 의 **두 번째 인자가 곧 원본값**이다.
   1·2차 추출기는 `b1/b4(var, ORIG)` 형태만 봐서 이 흔한 형태를 통째로 놓쳤다.
   ⚠두 번째 인자가 -1 이면 '원본 유지' 뜻이라 원본값이 아니다 — 건너뛴다."""
import sys, io, re, os, glob, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
in_tab, orig, desc = set(st['in_tab']), st['orig'], st['desc']

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
src = ''
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    src += '\n' + io.open(f, encoding='utf-8').read()

# tune("key", <expr>) 의 2번째 인자
found = {}
for m in re.finditer(r'tune\(\s*"([a-zA-Z0-9_]+)"\s*,\s*([^),]+)\)', src):
    k, a = m.group(1), m.group(2).strip()
    if not re.fullmatch(r'-?(?:0[xX][0-9a-fA-F_]+|[0-9][0-9_]*)', a):
        continue
    a = a.replace('_', '')
    v = int(a, 16) if a.lower().lstrip('-').startswith('0x') else int(a)
    if v == -1:
        continue                      # '원본 유지' 지시값이지 원본값이 아니다
    found.setdefault(k, set()).add(v)

add, skip = {}, []
for k in sorted(in_tab):
    if k in orig:
        continue
    if k in found:
        vs = sorted(found[k])
        add[k] = str(vs[0]) if len(vs) == 1 else ' / '.join(str(x) for x in vs)
    else:
        skip.append(k)

# 설명에 음수 원본이 적힌 것 보정(예: rt_a_slope 원본 −800)
NEG = re.compile(r'원본이?\s*\**\s*음수\s*\**\s*[\(（]\s*[−-]\s*([0-9]+)')
for k in list(skip):
    m = NEG.search(desc.get(k, ''))
    if m:
        add[k] = '-' + m.group(1)
        skip.remove(k)

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
anc = ' // ★[08-06] 2차 보강'
assert anc in t
ins = (' // ★[08-06] 3차 보강 — tune("키", 원본값) 의 2번째 인자에서 직접 추출.\n'
       + ''.join(' "%s" => "%s",\n' % (k, v) for k, v in sorted(add.items())))
io.open(P, 'w', encoding='utf-8', newline='\n').write(t.replace(anc, ins + anc, 1))

print('3차 보강 %d개' % len(add))
print('\n남은 %d개:' % len(skip))
for i in range(0, len(skip), 6):
    print('   ' + '  '.join(skip[i:i + 6]))
