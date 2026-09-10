#!/usr/bin/env python3
"""tlsscan.py — game_ai / game_core 의 **TLS 전역(메모 캐시)** 전수와 크기.

왜: 링크 사본은 게임과 **별도의 TLS 인스턴스**를 갖는다. 캐시가 에포크(seed,tick)로 무효화되지 않으면
    같은 입력에도 다른 값을 돌려줄 수 있다 — 트윈 잔차 0.025% 의 남은 후보.
"""
import glob
import os
import re

RAW = r'C:\tfm2mods\_gaibc\_ga_raw.txt'
DEM = r'C:\tfm2mods\_gaibc\_nm_defined.txt'


def load_map():
    try:
        raw = [l.split()[-1] for l in open(RAW, encoding='utf-8', errors='ignore') if len(l.split()) >= 2]
        dem = [l.split(None, 2)[-1].strip() for l in open(DEM, encoding='utf-8', errors='ignore') if len(l.split()) >= 2]
        return dict(zip(raw, dem))
    except OSError:
        return {}


def demangle(sym):
    """v0 망글에서 경로 조각만 뽑는 최소 디망글(길이 접두 + 식별자)."""
    out = []
    for m in re.finditer(r'(\d+)([A-Za-z_][A-Za-z0-9_]*)', sym):
        n, w = int(m.group(1)), m.group(2)
        if 3 <= n <= len(w):
            out.append(w[:n])
    return '::'.join(out)


def main():
    D = load_map()
    found = {}
    for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')) + sorted(glob.glob(r'C:\tfm2mods\_gcbc\*.ll')):
        mod = os.path.basename(f)
        for l in open(f, encoding='utf-8', errors='ignore'):
            if not (l.startswith('@') and 'thread_local' in l):
                continue
            sym = l.split(' = ')[0].strip('@').strip('"')
            ext = ' = external ' in l
            sz = re.search(r'<\{ \[(\d+) x i8\]', l)
            base = re.sub(r'0*23___RUST_STD_INTERNAL_VAL$', '', sym)
            base = re.sub(r'0s_0.*$', '', base)
            name = D.get(sym) or D.get(base) or demangle(base) or base
            key = name
            if key not in found or (found[key][2] and not ext):
                found[key] = [int(sz.group(1)) if sz else 0, mod, ext]
    print('TLS 전역 %d종' % len(found))
    for k, (s, m, e) in sorted(found.items(), key=lambda x: -x[1][0]):
        print('   %7d B  %-9s %s%s' % (s, m, '[ext]' if e else '     ', k[-88:]))


if __name__ == '__main__':
    main()
