# -*- coding: utf-8 -*-
r"""calc 입력표 — 추적기 사각지대(21 §4-A). sites.rs 의 Tf::Compute 창마다 콜백이 읽는 레지스터·프레임 입력을 뽑는다.

    python -X utf8 calc_table.py > calc_table.md

창이 SAME 이어도 콜백이 창 밖 레지스터·슬롯을 읽으면 조용히 틀릴 수 있어서 AUTO 창도 전부 표에 넣는다.
"""
import json
import os
import re

ROOT = r'F:\claude\tfm2mods\tfm2_detail_tactics\src'
HERE = os.path.dirname(os.path.abspath(__file__))
REG = 'rax rcx rdx rbx rsp rbp rsi rdi r8 r9 r10 r11 r12 r13 r14 r15'.split()

calc = {}
for fn in ['extra_combat.rs', 'extra_lane.rs', 'extra_ops.rs', 'verified_candidates.rs']:
    s = open(os.path.join(ROOT, fn), encoding='utf-8').read()
    for m in re.finditer(r'pub(?:\(crate\))? fn (\w+)\(\s*(\w+|_)\s*:\s*&(?:crate::micro::)?Frame\s*,\s*(?:v|_v)\s*:\s*i64\)\s*->\s*Option<u64>\s*\{', s):
        i, depth = m.end(), 1
        while depth and i < len(s):
            depth += {'{': 1, '}': -1}.get(s[i], 0)
            i += 1
        body = s[m.end():i - 1]
        ins = []
        for a, b in re.findall(r'f\.reg\((\d+)\)|f\.scalar\(Src::(\w+\([^)]*\))\)', body):
            ins.append(REG[int(a)] if a else b)
        calc[m.group(1)] = dict(file=fn, inputs=sorted(set(ins)))

sites = open(os.path.join(ROOT, 'sites.rs'), encoding='utf-8').read()
track = json.load(open(os.path.join(HERE, 'track.json'), encoding='utf-8'))
wby = {w['name']: w for w in track['windows']}
print('| 창 | 전술 | 추적기 | 신 후보 rva | 콜백 | 입력(창 시점) |')
print('|---|---|---|---|---|---|')
n = pure = 0
for m in re.finditer(r'Patch \{ name: "([^"]+)", tactic: (\d+).*', sites):
    name = m.group(1)
    fns = re.findall(r'Tf::Compute\(crate::(?:\w+::)*(\w+)\)', m.group(0))
    for f in sorted(set(fns)):
        c = calc.get(f)
        n += 1
        if c is None:
            print('| `%s` | ? | ? | ? | `%s` | 콜백 못 찾음 |' % (name, f))
            continue
        if not c['inputs']:
            pure += 1
            continue
        w = wby.get(name, {})
        print('| `%s` | %s | %s | %s | `%s` (%s) | %s |' % (
            name, w.get('tactic_id'), w.get('code'), w.get('new_rva'), f, c['file'], ' · '.join(c['inputs'])))
print()
print('창×콜백 %d · 입력 없는 순수 콜백 %d(표 제외)' % (n, pure))
