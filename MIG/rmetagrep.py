#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""lib.rmeta 원본 바이트에서 패턴 주변 문맥(문자열 런)을 뽑는다.

rmetadocs.py 는 주석만 뽑아 **어느 항목의 주석인지**를 잃는다.
여기서는 패턴(필드명·함수명)을 rmeta 에서 직접 찾고 그 앞뒤 N 바이트의
읽을 수 있는 런을 전부 출력한다 → 주석과 항목의 인접 관계를 복원한다.

  python rmetagrep.py game_ai mf_swap [--ctx 3000] [--max 20]
"""
import io
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

SDK = r'C:\tfm2mods\sdk_058\mod-sdk\deps'
RUN = re.compile(
    rb'(?:[\xea-\xed][\x80-\xbf]{2}|[\xe2-\xe3][\x80-\xbf]{2}|[\x20-\x7e]|\xc2[\xa0-\xbf]){6,}')


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


def load(crate):
    hits = [x for x in os.listdir(SDK)
            if x.startswith('lib' + crate + '-') and x.endswith('.rlib')]
    if not hits:
        sys.exit('%s: rlib 없음' % crate)
    return rmeta_of(os.path.join(SDK, sorted(hits)[0]))


def main():
    crate = sys.argv[1]
    pat = sys.argv[2].encode('utf-8')
    ctx = 3000
    mx = 20
    if '--ctx' in sys.argv:
        ctx = int(sys.argv[sys.argv.index('--ctx') + 1])
    if '--max' in sys.argv:
        mx = int(sys.argv[sys.argv.index('--max') + 1])
    d = load(crate)
    n = 0
    for m in re.finditer(re.escape(pat), d):
        n += 1
        if n > mx:
            print('... (더 있음)')
            break
        a = max(0, m.start() - ctx)
        b = min(len(d), m.end() + ctx)
        print('=== hit #%d @0x%x ===' % (n, m.start()))
        for r in RUN.finditer(d[a:b]):
            try:
                s = r.group(0).decode('utf-8')
            except UnicodeDecodeError:
                continue
            off = a + r.start()
            mark = '>>' if (m.start() - 30) <= off <= m.end() else '  '
            print('%s @0x%-8x %s' % (mark, off, s))
        print()
    if n == 0:
        print('없음: %s' % sys.argv[2])


main()
