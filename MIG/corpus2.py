#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""corpus2.py — "파악해야 할 소스 함수"의 진짜 개수.

corpus.py 의 3,556개는 **모노모픽 인스턴스**까지 센 수다
(`calculate_path<Around>` `calculate_path<Trace>` … 가 전부 따로 잡힌다).
사람이 이해하는 단위는 **소스 함수 1개**이므로 `DISubprogram(file:, line:)` 로 접는다.

또 하나 접는 축: 클로저(`closure$N`)는 부모 함수의 일부다 — 별도 항목이 아니라
부모에 합산해야 "함수 하나 이해" 단위가 된다.
"""
import io
import os
import re
import sys
from collections import defaultdict

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

IRDIR = r'C:\tfm2mods\_gaibc'
RE_DEF = re.compile(r'^define\b[^\n]*?@("[^"]+"|[\w.$]+)\([^\n]*?!dbg (![0-9]+)', re.M)
RE_SUB = re.compile(r'^(![0-9]+) = (?:distinct )?!DISubprogram\(name: "([^"]*)"[^\n]*?file: (![0-9]+), line: ([0-9]+)', re.M)
RE_FILE = re.compile(r'^(![0-9]+) = !DIFile\(filename: "([^"]*)"', re.M)


def main():
    # (파일, 줄) -> [이름, 최대 IR 줄수, 인스턴스 수]
    src = {}
    for fn in sorted(os.listdir(IRDIR)):
        if not fn.endswith('.ll'):
            continue
        text = io.open(os.path.join(IRDIR, fn), encoding='utf-8', errors='replace').read()
        files = dict(RE_FILE.findall(text))
        subs = {}
        for mid, name, fid, line in RE_SUB.findall(text):
            subs[mid] = (name, files.get(fid, '?'), int(line))
        lines = text.split('\n')
        cur, start = None, 0
        for i, ln in enumerate(lines):
            if ln.startswith('define'):
                m = RE_DEF.match(ln)
                cur, start = (m.group(2) if m else None), i
            elif ln == '}' and cur is not None:
                info = subs.get(cur)
                if info:
                    name, f, line = info
                    key = (f, line)
                    n = i - start
                    if key not in src:
                        src[key] = [name, n, 0]
                    src[key][1] = max(src[key][1], n)
                    src[key][2] += 1
                cur = None

    ga = {k: v for k, v in src.items() if k[0].startswith('game-ai')}
    print('소스 함수(파일:줄 기준) 총 %d개 · 그중 game-ai %d개' % (len(src), len(ga)))
    print()

    # 클로저를 부모에 합산: 같은 파일에서 클로저는 부모 줄 근처에 산다.
    # 여기서는 이름으로만 분리해 규모를 보인다.
    clo = {k: v for k, v in ga.items() if 'closure' in v[0]}
    top = {k: v for k, v in ga.items() if 'closure' not in v[0]}
    print('  최상위 함수 %d개 (IR %s줄) / 클로저 %d개 (IR %s줄)'
          % (len(top), f'{sum(v[1] for v in top.values()):,}',
             len(clo), f'{sum(v[1] for v in clo.values()):,}'))
    print()

    ns = sorted(v[1] for v in top.values())
    print('최상위 함수 크기 분포(IR 줄)')
    for lo, hi, label in [(0, 20, '~20'), (20, 100, '20~100'), (100, 500, '100~500'),
                          (500, 2000, '500~2000'), (2000, 10**9, '2000~')]:
        sel = [n for n in ns if lo <= n < hi]
        if sel:
            print('  %-10s %5d개 (%5.1f%%)  누적 %10s줄' %
                  (label, len(sel), 100.0 * len(sel) / len(ns), f'{sum(sel):,}'))
    print('  합계      %5d개              누적 %10s줄' % (len(ns), f'{sum(ns):,}'))
    print()

    byfile = defaultdict(lambda: [0, 0])
    for (f, _), v in top.items():
        byfile[f][0] += 1
        byfile[f][1] += v[1]
    print('파일별 상위 20 (최상위 함수 수 / IR 줄)')
    for f, (c, n) in sorted(byfile.items(), key=lambda kv: -kv[1][1])[:20]:
        print('  %-58s %4d %9s' % (f.replace('game-ai\\\\src\\\\', ''), c, f'{n:,}'))


if __name__ == '__main__':
    main()
