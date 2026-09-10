#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""rmeta 원시 바이트 구간을 '읽을 수 있게' 덤프. 사용: dqraw.py <crate> <off> <len>
비출력 바이트는 '.' 로, 한글 런은 그대로 보인다."""
import io
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
SDK = r'C:\tfm2mods\sdk_058\mod-sdk\deps'


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


crate, off, ln = sys.argv[1], int(sys.argv[2], 0), int(sys.argv[3], 0)
hits = sorted(x for x in os.listdir(SDK)
              if x.startswith('lib' + crate + '-') and x.endswith('.rlib'))
d = rmeta_of(os.path.join(SDK, hits[0]))
b = d[off:off + ln]
out = []
i = 0
while i < len(b):
    c = b[i]
    if 0xea <= c <= 0xed or 0xe2 <= c <= 0xe3:
        try:
            out.append(b[i:i + 3].decode('utf-8'))
            i += 3
            continue
        except Exception:
            pass
    if 0x20 <= c <= 0x7e:
        out.append(chr(c))
    else:
        out.append('\x01')
    i += 1
s = ''.join(out)
s = re.sub('\x01{1,}', ' ~ ', s)
print(s)
