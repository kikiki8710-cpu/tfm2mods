# -*- coding: utf-8 -*-
"""logic(의사코드) 안의 오프셋/상수가 mem/consts/knobs 정본에 실제로 있는지 기계 대조.
G5/G6 가 보지 않는 부류(= logic 산문에만 있는 오프셋·임계)를 찾는다."""
import io, json, re, sys

sys.stdout.reconfigure(encoding='utf-8', errors='replace')
SP = r'C:\tfm2mods\MIG\_spec\specs20_v3.json'
d = json.load(io.open(SP, encoding='utf-8'))
S = d['specs']

for i in range(5, 10):
    sp = S[i]
    print('=' * 78)
    print('IDX %d %s' % (i, sp['name']))
    logic = sp['logic']
    memoffs = set()
    for m in sp['mem']:
        o = str(m.get('offset', ''))
        if o.startswith('0x'):
            memoffs.add(int(o, 16))
    # logic 안의 모든 +0x.. 오프셋
    found = {}
    for mo in re.finditer(r'([A-Za-z_][A-Za-z_0-9:.\[\]]*)\s*\+\s*(0x[0-9a-fA-F]+)', logic):
        found.setdefault(int(mo.group(2), 16), set()).add(mo.group(1))
    # vtable[0x..] 형태
    for mo in re.finditer(r'vtable\[(0x[0-9a-fA-F]+)\]', logic):
        found.setdefault(int(mo.group(1), 16), set()).add('vtable')
    for mo in re.finditer(r'\[(0x[0-9a-fA-F]+)\]', logic):
        found.setdefault(int(mo.group(1), 16), set()).add('[]idx')
    miss = sorted(o for o in found if o not in memoffs)
    print('  logic 오프셋 %d종 · mem 등재 %d종 · **미등재 %d종**' % (len(found), len(memoffs), len(miss)))
    for o in miss:
        print('    !! 0x%x  (logic 표기: %s)' % (o, ','.join(sorted(found[o]))))
    # 상수 대조
    cvals = set()
    for c in sp['consts']:
        v = c.get('value')
        if isinstance(v, int):
            cvals.add(v)
    for k in sp['knobs']:
        v = k.get('value')
        if isinstance(v, int):
            cvals.add(v)
    # logic 안의 큰 리터럴(>=3자리) + 비교 임계
    lits = set()
    for mo in re.finditer(r'(?<![\w.x])(\d{2,})(?![\w])', logic):
        lits.add(int(mo.group(1)))
    big = sorted(v for v in lits if v >= 15 and v not in cvals and v not in memoffs)
    print('  logic 리터럴(>=15, consts/knobs/mem 미등재) %d종: %s' % (len(big), big[:40]))
