#!/usr/bin/env python3
"""cachekeys.py — 캐시 래퍼가 부르는 `LocalKey::with` **인스턴스 심볼**을 뽑는다.

왜: 관찰자 사본의 잔차는 캐시 이력 차이다. 게임 쪽 접근자(exe RVA)를 후킹해
    같은 인자로 **내 사본의 같은 인스턴스**를 같이 부르면 내 캐시가 게임과 같은 시점에
    같은 값으로 채워진다(실측: position_eval_at 이 캐시가 있는데도 미러링돼 DIFF 0).
    그러려면 내 사본 쪽 심볼 이름과 인자 개수를 알아야 한다.

사용: python MIG\\cachekeys.py [함수이름조각 ...]
"""
import glob
import io
import re
import sys


def main():
    wants = [w.lower() for w in sys.argv[1:]] or ['champion_hp_value', 'dn_cache', 'defense_nexus']
    for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')):
        t = io.open(f, encoding='utf-8', errors='ignore').read()
        for m in re.finditer(r'^define ([^@\n]*)@(_RINvMs2_NtNtCs9ec1k27omRZ_3std6thread5local[A-Za-z0-9_.$]*)\(([^\n]*?)\) unnamed_addr', t, re.M):
            sym = m.group(2)
            low = sym.lower()
            if not any(w in low for w in wants):
                continue
            args = re.findall(r'([a-z0-9]+(?:\s+[a-z_]+(?:\([^)]*\))?)*)\s+%(\d+)', m.group(3))
            print('%s' % f[-8:])
            print('  ret : %s' % m.group(1).strip())
            print('  sym : %s' % sym)
            print('  인자 %d개:' % len(args))
            for a, n in args:
                print('     %%%s | %s' % (n, a[:90]))
            print()


if __name__ == '__main__':
    main()
