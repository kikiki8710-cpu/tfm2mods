# -*- coding: utf-8 -*-
"""원본값 4차 보강 — 남은 두 갈래.
   ①`tune("키", -1);  // … 원본 N …` 처럼 **같은 줄 주석**에 원본이 적힌 것
   ②토글: cfg 로더 arm 이 쓰는 static 의 초기값(`AtomicBool::new(false)` → 꺼짐)"""
import sys, io, re, os, glob, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
in_tab, orig = set(st['in_tab']), st['orig']

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
lines, src = [], ''
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    s = io.open(f, encoding='utf-8').read()
    src += '\n' + s
    lines += s.split('\n')

NUM = re.compile(r'원본\s*(?:값\s*)?\**\s*(0[xX][0-9a-fA-F]+|-?[0-9][0-9,]*)')

# ① tune 줄(및 바로 윗줄) 주석에서 원본값
comment = {}
for i, ln in enumerate(lines):
    m = re.search(r'tune\(\s*"([a-zA-Z0-9_]+)"', ln)
    if not m:
        continue
    k = m.group(1)
    for cand in (ln, lines[i - 1] if i else ''):
        c = cand.split('//', 1)[1] if '//' in cand else ''
        mm = NUM.search(c)
        if mm:
            raw = mm.group(1).replace(',', '')
            v = int(raw, 16) if raw.lower().lstrip('-').startswith('0x') else int(raw)
            if v != -1:
                comment.setdefault(k, str(v))
            break

# ② 토글: cfg arm "key" => { … STATIC.store … } → static 초기값
tog = {}
for m in re.finditer(r'"([a-zA-Z0-9_]+)"\s*=>\s*\{(?=(.{0,300}))', src, re.S):
    k, body = m.group(1), m.group(2)
    ms = re.search(r'\b([A-Z][A-Z0-9_]{2,})\s*\.\s*store', body)
    if not ms:
        continue
    md = re.search(r'static %s\s*:\s*Atomic(\w+)\s*=\s*Atomic\w+::new\(([^)]+)\)' % ms.group(1), src)
    if not md:
        continue
    init = md.group(2).strip()
    if init in ('false', '0'):
        tog[k] = '꺼짐'
    elif init == 'true':
        tog[k] = '켜짐'
    elif re.fullmatch(r'-?\d+', init):
        tog[k] = init

add, skip = {}, []
for k in sorted(in_tab):
    if k in orig:
        continue
    if k in comment:
        add[k] = comment[k]
    elif k in tog:
        add[k] = tog[k]
    else:
        skip.append(k)

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
anc = ' // ★[08-06] 3차 보강'
assert anc in t
ins = (' // ★[08-06] 4차 보강 — tune 줄 주석의 `원본 N`, 토글은 static 초기값.\n'
       + ''.join(' "%s" => "%s",\n' % (k, v) for k, v in sorted(add.items())))
io.open(P, 'w', encoding='utf-8', newline='\n').write(t.replace(anc, ins + anc, 1))

print('4차 보강 %d개' % len(add))
print('\n남은 %d개:' % len(skip))
for i in range(0, len(skip), 6):
    print('   ' + '  '.join(skip[i:i + 6]))
