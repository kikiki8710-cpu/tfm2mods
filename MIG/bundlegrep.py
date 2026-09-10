#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""bundlegrep.py — 게임 **데이터/에셋 번들** 전체를 바이트로 훑어 패턴을 찾는다.

왜 필요한가: 지금까지의 "부재 확정" 판정은 **SDK rlib/rmeta + IR + exe** 범위였고
**게임 데이터 번들은 한 번도 안 봤다**(`bundle_unpacked_full` 1.1GB · `TFM2.gg` 567MB ·
`db` 19MB · `config` · `ModData` · `bundle.game_data`). "배포본에 없다"를 쓰려면 여기까지 봐야 한다.

ASCII / UTF-16LE 양쪽으로 찾고, zlib/gzip 로 압축된 조각도 풀어서 한 번 더 본다.

사용:  python -X utf8 bundlegrep.py PATTERN [PATTERN...]
       python -X utf8 bundlegrep.py --list          # 스캔 대상과 크기만
"""
import io
import os
import sys
import zlib

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

GAME = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2'
# ⚠`mods\`(15GB)는 우리 모드라 제외. 게임 소유 자산만.
ROOTS = ['bundle.game_data', 'bundle_unpacked_full', 'TFM2.gg', 'db', 'config',
         'ModData', 'mod-sdk-stable', 'TeamfightManager2.exe', 'TFM2ModUploader.exe']
CHUNK = 8 << 20
ZMAGIC = (b'\x78\x9c', b'\x78\x01', b'\x78\xda', b'\x1f\x8b')


def targets():
    for r in ROOTS:
        p = os.path.join(GAME, r)
        if os.path.isfile(p):
            yield p
        elif os.path.isdir(p):
            for dp, _, fs in os.walk(p):
                for f in fs:
                    yield os.path.join(dp, f)


def pats(words):
    out = []
    for w in words:
        b = w.encode('ascii', 'ignore')
        out.append((w + ' [ascii]', b))
        out.append((w + ' [utf16le]', b.decode('ascii').encode('utf-16-le')))
    return out


def scan_bytes(buf, ps, hits, where, base=0):
    for label, pb in ps:
        i = buf.find(pb)
        while i != -1:
            hits.append((label, where, base + i, buf[max(0, i-60):i+len(pb)+60]))
            i = buf.find(pb, i + 1)


def main():
    words = [a for a in sys.argv[1:] if not a.startswith('--')]
    files = sorted(set(targets()))
    total = sum(os.path.getsize(f) for f in files if os.path.exists(f))
    if '--list' in sys.argv or not words:
        print('스캔 대상 %d 파일 / %.1f GB' % (len(files), total / 1e9))
        for f in files[:20]:
            print('   %10d  %s' % (os.path.getsize(f), f[len(GAME) + 1:]))
        if len(files) > 20:
            print('   ... 외 %d개' % (len(files) - 20))
        return

    ps = pats(words)
    hits = []
    done = 0
    zdone = 0
    for f in files:
        try:
            sz = os.path.getsize(f)
        except OSError:
            continue
        rel = f[len(GAME) + 1:]
        try:
            with io.open(f, 'rb') as fh:
                tail = b''
                base = 0
                while True:
                    buf = fh.read(CHUNK)
                    if not buf:
                        break
                    blk = tail + buf
                    scan_bytes(blk, ps, hits, rel, base - len(tail))
                    # 압축 조각 시도(작은 파일에 한해)
                    if sz < (64 << 20):
                        for m in ZMAGIC:
                            j = blk.find(m)
                            while j != -1 and zdone < 4000:
                                try:
                                    dec = zlib.decompressobj(47 if m == b'\x1f\x8b' else 15)
                                    out = dec.decompress(blk[j:j + (4 << 20)])
                                    if len(out) > 64:
                                        scan_bytes(out, ps, hits, rel + '  [zlib@%d]' % (base + j))
                                        zdone += 1
                                except Exception:
                                    pass
                                j = blk.find(m, j + 1)
                    base += len(buf)
                    tail = blk[-256:]
        except OSError:
            continue
        done += sz
        if done and int(done / 2e8) != int((done - sz) / 2e8):
            sys.stderr.write('  ... %.2f / %.2f GB\n' % (done / 1e9, total / 1e9))

    print('스캔 완료: %d 파일 / %.2f GB / 압축조각 %d개 해제' % (len(files), total / 1e9, zdone))
    print('패턴:', ', '.join(words))
    if not hits:
        print()
        print('★히트 0건 — 이 범위(게임 데이터 번들 전체)에 **실체 없음**')
        return
    print()
    print('히트 %d건' % len(hits))
    for label, where, off, ctx in hits[:60]:
        try:
            s = ctx.decode('utf-8', 'replace')
        except Exception:
            s = repr(ctx)
        print('  [%s] %s @%d' % (label, where, off))
        print('      %s' % s.replace('\n', ' ')[:200])


if __name__ == '__main__':
    main()
