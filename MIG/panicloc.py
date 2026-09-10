#!/usr/bin/env python3
"""panicloc.py — SDK 비트코드와 게임 exe 의 **패닉 Location 상수**를 같은 형식으로 뽑아 대조한다.

왜: `srcident.py` 는 IR 의 `DILocation`(디버그 줄) 과 exe 의 Location 상수를 비교했는데,
    둘은 성격이 달라 "함수 본문 마지막 줄 다음"이 늘 빠진 것처럼 보였다(2026-09-09 오독).
    양쪽 다 **Location 상수**(file,line,col)로 맞춰야 사과 대 사과가 된다.

IR 표현: `@alloc_X = private unnamed_addr constant <{ [N x i8] }> <{ [N x i8] c"...path..." }>`
         `@alloc_Y = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_X, [16 x i8] c"<len u64><line u32><col u32>" }>`
exe 표현: `.rdata` 의 {문자열 VA(8) · len(8) · line(4) · col(4)}.

판정: exe 에 있는 (파일, 줄) 이 IR 에도 있어야 한다. 없으면 **소스가 다르다**.
사용: python MIG\\panicloc.py [파일basename ...]
"""
import glob
import io
import os
import re
import struct
import sys

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
BASE = 0x140000000


def unesc(s):
    out = bytearray()
    i = 0
    while i < len(s):
        if s[i] == '\\' and i + 2 < len(s) and all(c in '0123456789abcdefABCDEF' for c in s[i + 1:i + 3]):
            out.append(int(s[i + 1:i + 3], 16))
            i += 3
        elif s[i] == '\\' and i + 1 < len(s):
            out.append(ord(s[i + 1]))       # `\\` 같은 비-16진 이스케이프
            i += 2
        else:
            out.append(ord(s[i]))
            i += 1
    return bytes(out)


def ir_locations():
    locs = {}
    for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')) + sorted(glob.glob(r'C:\tfm2mods\_gcbc\g*.ll')):
        paths = {}
        pend = []
        for line in io.open(f, encoding='utf-8', errors='ignore'):
            if not line.startswith('@'):
                continue
            nm = line.split(' =', 1)[0].strip()
            m = re.search(r'\[\d+ x i8\] c"((?:[^"\\]|\\..)*)"', line)
            if m and 'ptr @' not in line:
                b = unesc(m.group(1)).rstrip(b'\x00')      # IR 쪽 경로 상수는 NUL 로 끝난다
                if b.endswith(b'.rs'):
                    paths[nm] = re.split(rb'[\\/]', b)[-1].decode('ascii', 'replace')
                continue
            m2 = re.search(r'ptr (@[A-Za-z0-9_.$]+), \[16 x i8\] c"((?:[^"\\]|\\..)*)"', line)
            if m2:
                pend.append((m2.group(1), unesc(m2.group(2))))
        for pref, blob in pend:
            if pref not in paths or len(blob) < 16:
                continue
            ln, col = struct.unpack_from('<II', blob, 8)
            if 1 <= ln <= 200000 and 1 <= col <= 500:
                locs.setdefault(paths[pref], set()).add(ln)
    return locs


def sections(d):
    pe = struct.unpack_from('<I', d, 0x3c)[0]
    n = struct.unpack_from('<H', d, pe + 6)[0]
    opt = struct.unpack_from('<H', d, pe + 20)[0]
    return [struct.unpack_from('<IIII', d, pe + 24 + opt + i * 40 + 8) for i in range(n)]


def exe_locations():
    d = open(EXE, 'rb').read()
    secs = sections(d)

    def off2va(o):
        for vsz, sva, rsz, ra in secs:
            if ra <= o < ra + rsz:
                return BASE + sva + (o - ra)
        return None

    strva = {}
    for m in re.finditer(rb'[A-Za-z0-9_./\\-]{4,180}[.]rs', d):
        v = off2va(m.start())
        if v is not None:
            strva[v] = (re.split(rb'[\\/]', m.group(0))[-1].decode('ascii', 'replace'), len(m.group(0)))
    out = {}
    for vsz, sva, rsz, ra in secs:
        blob = d[ra:ra + rsz]
        for i in range(0, len(blob) - 16, 8):
            ptr, ln = struct.unpack_from('<QQ', blob, i)
            if ptr not in strva:
                continue
            base, plen = strva[ptr]
            if ln != plen:
                continue
            line, col = struct.unpack_from('<II', blob, i + 16)
            if 1 <= line <= 200000 and 1 <= col <= 500:
                out.setdefault(base, set()).add(line)
    return out


def main():
    want = set(sys.argv[1:])
    ir, exe = ir_locations(), exe_locations()
    common = sorted(set(ir) & set(exe))
    print('IR 파일 %d · exe 파일 %d · 공통 %d' % (len(ir), len(exe), len(common)))
    tot = out = 0
    bad = []
    for b in common:
        if want and b not in want:
            continue
        tot += len(exe[b])
        extra = exe[b] - ir[b]
        if extra:
            out += len(extra)
            bad.append((b, len(exe[b]), sorted(extra)))
    print('exe 패닉 위치 %d개 중 IR 에 없는 것 %d개' % (tot, out))
    if not bad:
        print('★전부 일치 = SDK 비트코드와 게임 exe 는 같은 소스')
    for b, n, e in sorted(bad, key=lambda x: -len(x[2]))[:20]:
        print('   %-28s exe %3d개 중 %3d개 없음: %s' % (b, n, len(e), e[:10]))


if __name__ == '__main__':
    main()
