#!/usr/bin/env python3
"""cachedfns.py — game_ai 의 **메모 래퍼**(`*_cached` / `*_uncached` 관례)를 전수 나열한다.

왜: 관찰자 사본의 잔차 = "게임은 캐시 적중, 나는 미스". 게임이 부르는 캐시 헬퍼를
    사본에도 흘려주면 캐시 이력이 수렴한다(헬퍼 2종만으로 0.0047%→0.0017% 실측).
    미러링 대상을 정하려면 캐시 래퍼 목록이 먼저 필요하다.

출력: DISubprogram 이름 · 소스파일:줄 · 외부링크 여부(사본에서 부를 수 있나) · 짝(uncached) 유무
사용: python MIG\\cachedfns.py
"""
import glob
import io
import re


def main():
    subs = {}      # name -> (file, line, module)
    defined = {}   # name -> (linkage, mangled)
    for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')):
        t = io.open(f, encoding='utf-8', errors='ignore').read()
        files = {m.group(1): m.group(2) for m in re.finditer(r'^!(\d+) = !DIFile\(filename: "([^"]+)"', t, re.M)}
        for m in re.finditer(r'^!\d+ = (?:distinct )?!DISubprogram\(([^\n]*)', t, re.M):
            b = m.group(1)
            nm = re.search(r'name: "([^"]+)"', b)
            fi = re.search(r'file: !(\d+)', b)
            ln = re.search(r'line: (\d+)', b)
            lk = re.search(r'linkageName: "([^"]+)"', b)
            if not (nm and fi and ln):
                continue
            n = nm.group(1)
            if not (n.endswith('_cached') or n.endswith('_uncached')):
                continue
            p = files.get(fi.group(1), '?')
            if 'game-ai' not in p:
                continue
            subs.setdefault(n, (re.split(r'[\\/]', p)[-1], int(ln.group(1)), f[-8:], lk.group(1) if lk else ''))
        for m in re.finditer(r'^define ([^@\n]*)@([A-Za-z0-9_.$]+)\(', t, re.M):
            defined[m.group(2)] = m.group(1).strip()

    print('game_ai 의 캐시 래퍼 %d개' % len(subs))
    print('%-42s %-22s %6s %-9s %s' % ('이름', '소스', '줄', '모듈', '링크(사본에서 호출 가능?)'))
    for n, (src, ln, mod, mangled) in sorted(subs.items()):
        lk = defined.get(mangled, '')
        if not lk and mangled:
            lk = '(정의 없음=인라인)'
        elif 'internal' in lk:
            lk = 'internal (raw 패치 필요)'
        elif lk:
            lk = '외부 OK'
        pair = '\u2714' if (n.replace('_cached', '_uncached') in subs or n.replace('_uncached', '_cached') in subs) else ''
        print('%-42s %-22s %6d %-9s %-26s %s' % (n[:42], src, ln, mod, lk, pair))


if __name__ == '__main__':
    main()
