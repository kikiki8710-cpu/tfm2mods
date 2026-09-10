#!/usr/bin/env python3
"""coreglobals.py — game_core / game_ai 의 **가변 전역**(비-TLS)을 전수 나열한다.

왜: 모드는 game_ai 뿐 아니라 **game_core 도 자기 사본을 링크**한다.
    게임이 시작 시 데이터 파일에서 채우는 전역이 있다면 내 사본 쪽은 비어 있고,
    그 값을 읽는 판단은 조용히 갈린다. game_ai 는 스캔했으나(=pf_stats 카운터뿐)
    game_core 는 미점검이었다(2026-09-09 세션의 유일한 미확인 구멍).

판정: `@sym = ... global ...`(constant 아님) 중 zeroinitializer 가 아닌 것 = 초기값이 있는 것,
      zeroinitializer = 런타임에 채워지는 것. OnceLock/Lazy 류는 후자다.
사용: python MIG\\coreglobals.py [--all]
"""
import glob
import io
import os
import re
import sys


def demangle_tail(sym):
    out = []
    for n, w in re.findall(r'(\d+)([A-Za-z_][A-Za-z0-9_]*)', sym):
        n = int(n)
        if 2 <= n <= len(w):
            out.append(w[:n])
    return '::'.join(out[-5:]) if out else sym


def main():
    show_all = '--all' in sys.argv
    rows = []
    for f in sorted(glob.glob(r'C:\tfm2mods\_gcbc\g*.ll')) + sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')):
        for line in io.open(f, encoding='utf-8', errors='ignore'):
            if not line.startswith('@'):
                continue
            if ' constant ' in line or 'thread_local' in line:
                continue
            m = re.match(r'@("?)([^"\s=]+)\1 = ([^\n]*?)\bglobal\b\s+([^\n,]*)', line)
            if not m:
                continue
            sym, mid, ty = m.group(2), m.group(3), m.group(4)
            if 'external' in mid:
                continue
            zero = 'zeroinitializer' in line
            sz = re.search(r'\[(\d+) x i8\]', ty)
            rows.append((os.path.basename(f), sym, int(sz.group(1)) if sz else 0, zero, mid.strip()))

    core = [r for r in rows if r[0].startswith('g')]
    ai = [r for r in rows if r[0].startswith('m')]
    print('가변 전역 — game_core %d개 · game_ai %d개' % (len(core), len(ai)))
    for label, group in (('game_core', core), ('game_ai', ai)):
        print()
        print('=== %s ===' % label)
        seen = set()
        for mod, sym, sz, zero, mid in sorted(group, key=lambda x: -x[2]):
            nm = demangle_tail(sym)
            if nm in seen and not show_all:
                continue
            seen.add(nm)
            print('  %-9s %7dB %-6s %s' % (mod, sz, '0초기' if zero else '값있음', nm[:88]))


if __name__ == '__main__':
    main()
