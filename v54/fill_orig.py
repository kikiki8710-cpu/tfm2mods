# -*- coding: utf-8 -*-
"""편집기의 '원본값' 칸이 비어 보이는 키를 채운다.
   출처 우선순위: ①설명문의 `원본 N` ②코드의 b1/b4(var, ORIG) 실측
   ⚠추정으로 채우지 않는다 — 두 출처 어디에도 없으면 그대로 둔다(빈 값이 틀린 값보다 낫다)."""
import sys, io, re, os, glob, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
in_tab, desc, orig = set(st['in_tab']), st['desc'], st['orig']

# ── 코드에서 실제 원본값 수집 (b1/b4 두 번째 인자) ──────────
SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
src = ''
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    src += '\n' + io.open(f, encoding='utf-8').read()
code = {}
for fm in re.finditer(r'\nunsafe fn (\w+)\(\)\s*\{', src):
    s0 = fm.end(); nx = src.find('\nunsafe fn ', s0)
    body = src[s0:nx if nx > 0 else len(src)]
    v2k = dict(re.findall(r'let\s+(\w+)\s*=\s*tune\(\s*"([a-zA-Z0-9_]+)"', body))
    for var, key in v2k.items():
        vals = set()
        for o in re.findall(r'\bb[14]\(\s*%s\s*,\s*([0-9_]+|0x[0-9a-fA-F_]+)\s*\)' % re.escape(var), body):
            o = o.replace('_', '')
            vals.add(int(o, 16) if o.lower().startswith('0x') else int(o))
        for tm in re.finditer(r'let v = (?:b4|b1)\(%s,\s*([0-9_]+)\);' % re.escape(var), body):
            vals.add(int(tm.group(1).replace('_', '')))
        if len(vals) == 1:
            code[key] = vals.pop()

DIG = re.compile(r'원본\s*(?:값\s*)?\**\s*(0[xX][0-9a-fA-F]+|-?[0-9][0-9,]*)')
add, skip = {}, []
for k in sorted(in_tab):
    if k in orig:
        continue
    d = desc.get(k, '')
    m = DIG.search(d)
    if m:
        raw = m.group(1).replace(',', '')
        add[k] = str(int(raw, 16)) if raw.lower().startswith('0x') else str(int(raw))
    elif k in code:
        add[k] = str(code[k])
    else:
        skip.append(k)

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
anc = ' // ★[0.5.4 신설] 경로/거리 시스템\n'
assert anc in t, '삽입 앵커 없음'
ins = (' // ★[08-06] 원본값 자동 보강 — 설명문의 `원본 N` 또는 코드 b1/b4 실측에서 뽑았다.\n'
       + ''.join(' "%s" => "%s",\n' % (k, v) for k, v in sorted(add.items())))
io.open(P, 'w', encoding='utf-8', newline='\n').write(t.replace(anc, ins + anc, 1))

print('원본값 보강 %d개 (기존 %d → %d)' % (len(add), len(orig), len(orig) + len(add)))
print('\n채우지 못해 그대로 둔 것 %d개 (설명·코드 어디에도 원본값이 없음):' % len(skip))
for i in range(0, min(len(skip), 48), 6):
    print('   ' + '  '.join(skip[i:i + 6]))
if len(skip) > 48:
    print('   ... 외 %d개' % (len(skip) - 48))
