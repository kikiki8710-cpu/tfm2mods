# -*- coding: utf-8 -*-
import json, io, sys
sys.stdout.reconfigure(encoding='utf-8')
v = json.load(io.open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json', encoding='utf-8'))
S = v['specs']
def show(i, key, idx=None, sub=None):
    x = S[i][key]
    if idx is not None: x = x[idx]
    if sub is not None: x = x[sub]
    print(f'--- /specs[{i}]/{key}' + (f'[{idx}]' if idx is not None else '') + (f'/{sub}' if sub else ''))
    print(json.dumps(x, ensure_ascii=False) if not isinstance(x, str) else x)
show(112, 'logic'); show(112, 'sig', None, 'ret')
show(112, 'notes', 0); show(112, 'open', 0); show(112, 'open', 1); show(112, 'open', 2)
show(113, 'logic'); show(113, 'sig', None, 'ret'); show(113, 'knobs', 4); show(113, 'open', 1); show(113, 'open', 2); show(113, 'open', 0); show(113, 'open', 3); show(113, 'open', 4)
show(114, 'logic'); show(114, 'sig', None, 'ret'); show(114, 'consts', 0); show(114, 'knobs', 1); show(114, 'notes', 0); show(114, 'mem', 39); show(114, 'mem', 2); show(114, 'open', 1); show(114, 'open', 3)
show(115, 'logic'); show(115, 'open', 1); show(115, 'open', 2)
show(116, 'logic'); show(116, 'notes', 0); show(116, 'open', 0); show(116, 'mem', 7); show(116, 'sig', None, 'ret')
show(117, 'consts', 0); show(117, 'notes', 0); show(117, 'logic')
