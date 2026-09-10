#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""corpus.py — "640개를 다 파악한다"의 **실제 분량**을 잰다.

세는 것:
  - game_ai IR 에서 **본체가 있는** 함수(define)와 그 IR 줄 수
  - 크레이트별(게임 코드 vs core/std 제네릭 인스턴스) 분리
  - 크기 분포 — 작은 것 몇 개, 큰 것 몇 개인지 (총량보다 이게 일정을 좌우한다)
"""
import io
import os
import re
import sys
from collections import defaultdict

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

IRDIR = r'C:\tfm2mods\_gaibc'
RE_DEF = re.compile(r'^define\b[^\n]*?@("[^"]+"|[\w.$]+)\(', re.M)


def crate_of(sym):
    """망글 심볼에서 소속 크레이트를 뽑는다(v0 망글의 크레이트 성분)."""
    s = sym.strip('"')
    for c in ('game_ai', 'game_core', 'bumpalo', 'hashbrown', 'rand_chacha',
              'rand', 'ahash', 'alloc', 'core', 'std'):
        if ('Nt' + 'Cs') in s or True:
            if c in s:
                return c
    return '기타'


def main():
    fns = {}          # sym -> (파일, 줄수)
    for fn in sorted(os.listdir(IRDIR)):
        if not fn.endswith('.ll'):
            continue
        path = os.path.join(IRDIR, fn)
        lines = io.open(path, encoding='utf-8', errors='replace').read().split('\n')
        cur, start = None, 0
        for i, ln in enumerate(lines):
            if ln.startswith('define'):
                m = RE_DEF.match(ln)
                if m:
                    cur, start = m.group(1), i
            elif ln == '}' and cur is not None:
                n = i - start
                # 같은 심볼이 여러 cgu 에 중복될 수 있다 - 가장 큰 것만.
                if cur not in fns or n > fns[cur][1]:
                    fns[cur] = (fn, n)
                cur = None

    bycrate = defaultdict(list)
    for sym, (f, n) in fns.items():
        bycrate[crate_of(sym)].append(n)

    print('본체 있는 함수(define) 총 %d개 · IR 총 %s줄' % (len(fns), f'{sum(n for _, n in fns.values()):,}'))
    print()
    print('%-14s %7s %12s %8s' % ('크레이트', '함수', 'IR줄', '중앙값'))
    for c, ns in sorted(bycrate.items(), key=lambda kv: -sum(kv[1])):
        ns.sort()
        print('%-14s %7d %12s %8d' % (c, len(ns), f'{sum(ns):,}', ns[len(ns) // 2]))

    ga = sorted(bycrate.get('game_ai', []))
    if not ga:
        return
    print()
    print('★game_ai 만: %d개 · %s줄' % (len(ga), f'{sum(ga):,}'))
    print()
    print('크기 분포(IR 줄 수)')
    buckets = [(0, 20, '~20  단순 접근자·래퍼'), (20, 100, '20~100  작은 술어'),
               (100, 500, '100~500  보통 판단함수'), (500, 2000, '500~2000  큰 함수'),
               (2000, 10**9, '2000~  거대 함수')]
    for lo, hi, label in buckets:
        sel = [n for n in ga if lo <= n < hi]
        if not sel:
            continue
        print('  %-26s %5d개  (%5.1f%%)  누적 %s줄  (%4.1f%%)'
              % (label, len(sel), 100.0 * len(sel) / len(ga),
                 f'{sum(sel):,}', 100.0 * sum(sel) / sum(ga)))
    print()
    print('  상위 10개가 차지하는 줄 = %s (%.1f%%)'
          % (f'{sum(ga[-10:]):,}', 100.0 * sum(ga[-10:]) / sum(ga)))
    print('  상위 50개가 차지하는 줄 = %s (%.1f%%)'
          % (f'{sum(ga[-50:]):,}', 100.0 * sum(ga[-50:]) / sum(ga)))


if __name__ == '__main__':
    main()
