#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""rlib 들의 `lib.rmeta` 에서 **한국어 문서화 주석**을 빠르게 훑는다.

앞 판(rmeta_docs.py)은 바이트마다 LEB128 을 시도해 137MB 에 못 쓴다.
여기서는 **UTF-8 한글 선행바이트(EA~ED)** 를 regex 로 먼저 찾고 그 주변만 편다 — O(n) 한 번.
"""
import io
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

SDK = r'C:\tfm2mods\sdk_058\mod-sdk\deps'
OUTDIR = r'C:\Users\jungs\AppData\Local\Temp\claude'
# 한글 음절/자모 + 흔한 기호까지 포함한 UTF-8 런
RUN = re.compile(
    rb'(?:[\xea-\xed][\x80-\xbf]{2}|[\xe2-\xe3][\x80-\xbf]{2}|[\x20-\x7e]|\xc2[\xa0-\xbf]){12,}')


def rmeta_of(path):
    f = io.open(path, 'rb')
    if f.read(8) != b'!<arch>\n':
        return None
    while True:
        h = f.read(60)
        if len(h) < 60:
            return None
        name = h[0:16].decode('ascii', 'replace').strip().rstrip('/')
        size = int(h[48:58].decode('ascii').strip())
        off = f.tell()
        if name == 'lib.rmeta':
            f.seek(off)
            return f.read(size)
        f.seek(off + size + (size & 1))


def scan(crate):
    hits = [x for x in os.listdir(SDK)
            if x.startswith('lib' + crate + '-') and x.endswith('.rlib')]
    if not hits:
        print('%s: rlib 없음' % crate)
        return
    d = rmeta_of(os.path.join(SDK, sorted(hits)[0]))
    if d is None:
        print('%s: rmeta 없음' % crate)
        return
    out = []
    for m in RUN.finditer(d):
        try:
            s = m.group(0).decode('utf-8')
        except UnicodeDecodeError:
            continue
        # 한글이 실제로 들어 있고 문장다운 것만
        if re.search(r'[가-힣]', s) and len(s) >= 12:
            out.append(s.strip())
    seen, uniq = set(), []
    for s in out:
        if s not in seen:
            seen.add(s)
            uniq.append(s)
    p = os.path.join(OUTDIR, 'docs_%s.txt' % crate)
    io.open(p, 'w', encoding='utf-8', newline='').write('\n'.join(uniq))
    print('%-12s rmeta %7.1f MB · 한글 주석 %5d개 · %7d 자 → %s'
          % (crate, len(d) / 1048576.0, len(uniq), sum(len(x) for x in uniq), p))


for c in sys.argv[1:] or ['game_ai', 'game_core', 'game_view']:
    scan(c)
