#!/usr/bin/env python3
"""memofns.py — game_ai / game_core 안의 **메모 캐시를 쓰는 함수**를 전수 조사한다.

왜: 관찰자(shadow) 사본의 잔차는 "게임은 캐시 적중, 나는 미스" 에서 나온다.
    게임이 부르는 모든 캐시 헬퍼를 사본에도 같이 흘려주면 캐시 이력이 수렴한다
    (실측: 헬퍼 2종만 미러링해도 interaction_score 잔차 0.0047% → 0.0017%).
    그러려면 먼저 **어떤 함수가 캐시를 쓰는지** 알아야 한다.

판정 기준: 함수 본문에서 `std::thread::local::LocalKey::with` 를 부르고,
           그 클로저가 hashbrown/RawTable 또는 `*_uncached` 를 부르는 것.
           (이름이 `*_cached` 로 끝나는 것도 같이 표시)

사용: python MIG\\memofns.py
"""
import glob
import io
import os
import re


def short(sym):
    out = []
    for n, w in re.findall(r'(\d+)([A-Za-z_][A-Za-z0-9_]*)', sym):
        n = int(n)
        if 2 <= n <= len(w):
            out.append(w[:n])
    return '::'.join(out[-4:])


def main():
    rows = []
    for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')) + sorted(glob.glob(r'C:\tfm2mods\_gcbc\g*.ll')):
        t = io.open(f, encoding='utf-8', errors='ignore').read()
        # 함수 단위로 쪼갠다
        for m in re.finditer(r'^define [^\n]*@([A-Za-z0-9_.$]+)\(', t, re.M):
            sym = m.group(1)
            i = m.start()
            j = t.find('\n}\n', i)
            if j == -1:
                continue
            body = t[i:j]
            if len(body) > 400000:
                continue
            has_local = 'thread5localINtB6_8LocalKey' in body or '3std6thread5local' in body
            if not has_local:
                continue
            uncached = 'uncached' in body
            table = 'hashbrown' in body or 'RawTable' in body
            if not (uncached or table):
                continue
            nm = short(sym)
            rows.append((os.path.basename(f), nm, len(body.split('\n')), uncached, table, sym))

    seen = set()
    print('메모 캐시를 쓰는 함수 %d개(중복 포함)' % len(rows))
    print('%-9s %-58s %5s %s' % ('모듈', '이름', '줄', '표시'))
    for mod, nm, n, unc, tab, sym in sorted(rows, key=lambda x: x[1]):
        if nm in seen:
            continue
        seen.add(nm)
        tag = ('uncached ' if unc else '') + ('table' if tab else '')
        print('%-9s %-58s %5d %s' % (mod, nm[:58], n, tag))


if __name__ == '__main__':
    main()
