#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""lib.rmeta 의 읽을 수 있는 문자열 런을 **전부**(ASCII 전용 포함) 오프셋과 함께 덤프.

rmetadocs.py 는 한글이 든 것만 남겨 ASCII 전용 코드표(`0=None 1=Serpen ...`)를 잃는다.
  python rmetadump.py game_ai > out.txt
"""
import io
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

SDK = r'C:\tfm2mods\sdk_058\mod-sdk\deps'
RUN = re.compile(
    rb'(?:[\xea-\xed][\x80-\xbf]{2}|[\xe2-\xe3][\x80-\xbf]{2}|[\x20-\x7e]|\xc2[\xa0-\xbf]){10,}')


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


crate = sys.argv[1]
lo = int(sys.argv[2], 0) if len(sys.argv) > 2 else 0
hi = int(sys.argv[3], 0) if len(sys.argv) > 3 else 1 << 40
hits = [x for x in os.listdir(SDK)
        if x.startswith('lib' + crate + '-') and x.endswith('.rlib')]
d = rmeta_of(os.path.join(SDK, sorted(hits)[0]))
w = sys.stdout
for m in RUN.finditer(d):
    if not (lo <= m.start() < hi):
        continue
    try:
        s = m.group(0).decode('utf-8')
    except UnicodeDecodeError:
        continue
    w.write('%08x\t%s\n' % (m.start(), s))
