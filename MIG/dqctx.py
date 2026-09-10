#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""rmeta 한국어 주석을 **주변 식별자 컨텍스트와 함께** 조회한다.

rmetadocs.py 는 주석 본문만 뽑아 "누구의 주석인지"를 잃는다.
여기서는 주석 런의 앞뒤 바이트에서 ASCII 식별자를 긁어 같이 보여준다.

사용:
  python dqctx.py game_ai resolve_fight          # 키워드(주석 or 컨텍스트)로 필터
  python dqctx.py game_ai --re "v42|FightLine"
  python dqctx.py game_core --near 600 sub_goal
"""
import io
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

SDK = r'C:\tfm2mods\sdk_058\mod-sdk\deps'
RUN = re.compile(
    rb'(?:[\xea-\xed][\x80-\xbf]{2}|[\xe2-\xe3][\x80-\xbf]{2}|[\x20-\x7e]|\xc2[\xa0-\xbf]){12,}')
IDENT = re.compile(rb'[A-Za-z_][A-Za-z0-9_]{2,}')


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
    args = [a for a in sys.argv[1:]]
    crate = args.pop(0)
    near = 400
    rx = None
    pats = []
    while args:
        a = args.pop(0)
        if a == '--near':
            near = int(args.pop(0))
        elif a == '--re':
            rx = re.compile(args.pop(0), re.I)
        else:
            pats.append(a)
    d = load(crate)
    n = 0
    for m in RUN.finditer(d):
        try:
            s = m.group(0).decode('utf-8')
        except UnicodeDecodeError:
            continue
        if not re.search(r'[가-힣]', s) or len(s) < 12:
            continue
        a, b = m.start(), m.end()
        pre = d[max(0, a - near):a]
        post = d[b:b + near]
        pids = [x.decode('ascii') for x in IDENT.findall(pre)][-8:]
        nids = [x.decode('ascii') for x in IDENT.findall(post)][:8]
        blob = s + ' || ' + ' '.join(pids) + ' || ' + ' '.join(nids)
        if rx is not None:
            if not rx.search(blob):
                continue
        elif pats and not any(p.lower() in blob.lower() for p in pats):
            continue
        n += 1
        print('=== @%d  PRE[%s]  POST[%s]' % (a, ' '.join(pids), ' '.join(nids)))
        print(s.strip())
    print('--- %d hit' % n)


main()
