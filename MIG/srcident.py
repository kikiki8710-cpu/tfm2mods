#!/usr/bin/env python3
"""srcident.py — [STALE 2026-09-09 · 0.5.8] DILocation 기반 1차 방법. **판정 정본 = panicloc.py**.
⚠이 스크립트의 "소스가 다르다" 결과는 오독이었다: DILocation(명령 디버그 줄)과 #[track_caller] 패닉 Location 상수는
  성격이 달라 섞어 비교하면 안 된다(Location 상수는 명령에 !dbg 를 안 달고 데이터로만 남음). 결론 = SDK 비트코드 == exe(game_ai).
  근거 = REPORT\tfm2_ai_adjust\03_시행착오.md 「잔차의 뿌리를 통제실험으로 못박은 날」 · MEM DONE.md.

srcident.py — SDK 비트코드 rlib 과 출시 exe 가 **같은 소스에서 나왔는지** 판정한다.

왜: agent_link 트윈에서 "상태가 바이트까지 같은데 결정이 갈리는" 잔차 0.006% 가 남았고,
    코드 차이의 마지막 가정이 "SDK rlib == 게임 exe" 다. 이게 깨져 있으면 아무리 맞춰도 100% 는 불가능하다.

방법: 양쪽 다 패닉 `Location { file, line, col }` 을 데이터로 들고 있다.
  - exe: `.rdata` 에 {file 문자열 VA(8) · len(8) · line(4) · col(4)} 16B 구조체가 깔려 있다.
  - IR : `!DILocation(line:, column:, scope:)` + `!DIFile(filename:)`.
  파일별 **줄 번호 집합**을 비교한다. 소스가 같으면 exe 쪽 줄은 IR 쪽의 부분집합이어야 한다
  (exe 는 LTO 로 일부 패닉이 접혀 사라질 수 있으나, exe 에만 있고 IR 에 없는 줄은 나오면 안 된다).

사용: python MIG\\srcident.py [파일조각]
"""
import glob
import io
import os
import re
import struct
import sys

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
BASE = 0x140000000


def sections(d):
    pe = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, pe + 6)[0]
    opt = struct.unpack_from('<H', d, pe + 20)[0]
    return [struct.unpack_from('<IIII', d, pe + 24 + opt + i * 40 + 8) for i in range(nsec)]


def main():
    want = sys.argv[1] if len(sys.argv) > 1 else '.rs'
    d = open(EXE, 'rb').read()
    secs = sections(d)

    def va2off(va):
        r = va - BASE
        for vsz, sva, rsz, ra in secs:
            if sva <= r < sva + max(vsz, rsz):
                o = ra + r - sva
                return o if o < len(d) else None
        return None

    # 1) exe 에서 .rs 로 끝나는 경로 문자열 위치 수집
    strs = {}          # off -> path
    for m in re.finditer(rb'[A-Za-z0-9_\-./\\]{6,180}\.rs', d):
        strs[m.start()] = m.group(0).decode('ascii', 'replace').replace('\\', '/')

    off2va = {}
    for vsz, sva, rsz, ra in secs:
        for o in range(ra, min(ra + max(vsz, rsz), len(d))):
            pass
    # 역매핑은 섹션 단위로 계산(전수 루프 대신)

    def off2va_f(o):
        for vsz, sva, rsz, ra in secs:
            if ra <= o < ra + rsz:
                return BASE + sva + (o - ra)
        return None

    strva = {}
    for o, p in strs.items():
        v = off2va_f(o)
        if v is not None:
            strva[v] = (p, len(p))

    # 2) Location 구조체 스캔: {ptr, len, line, col}
    exe_lines = {}
    for vsz, sva, rsz, ra in secs:
        blob = d[ra:ra + rsz]
        for i in range(0, len(blob) - 16, 8):
            ptr, ln = struct.unpack_from('<QQ', blob, i)
            if ptr not in strva:
                continue
            path, plen = strva[ptr]
            if ln != plen:
                continue
            line, col = struct.unpack_from('<II', blob, i + 16)
            if not (1 <= line <= 200000 and 1 <= col <= 500):
                continue
            exe_lines.setdefault(path, set()).add(line)

    # 3) IR 쪽 DILocation 줄 수집
    ir_lines = {}
    for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')) + sorted(glob.glob(r'C:\tfm2mods\_gcbc\g*.ll')):
        t = io.open(f, encoding='utf-8', errors='ignore').read()
        files = {}
        for m in re.finditer(r'^!(\d+) = !DIFile\(filename: "([^"]+)"', t, re.M):
            files[m.group(1)] = m.group(2).replace('\\', '/')
        # 스코프는 DISubprogram 이 아니라 DILexicalBlock* 일 수 있고, DILocation 에 column 이 없기도 하다.
        #   file: 이 나올 때까지 scope 체인을 태서 파일을 찾는다.
        md = dict(re.findall(r'^!(\d+) = (.*)$', t, re.M))
        cache = {}

        def sfile(n, depth=0):
            if n in cache:
                return cache[n]
            v = md.get(n, '')
            fm = re.search(r'file: !(\d+)', v)
            if fm and fm.group(1) in files:
                cache[n] = fm.group(1)
                return cache[n]
            sm = re.search(r'scope: !(\d+)', v)
            r = sfile(sm.group(1), depth + 1) if (sm and depth < 20) else None
            cache[n] = r
            return r

        for m in re.finditer(r'^!(\d+) = !DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)', t, re.M):
            fid = sfile(m.group(3))
            if fid:
                ir_lines.setdefault(files[fid], set()).add(int(m.group(2)))

    def norm(p):
        return re.split(r'[\\/]', p)[-1]        # exe 는 basename 만 들고 있다 — 양쪽 다 basename 으로 맞춘다

    ir_by_base = {}
    for p, s in ir_lines.items():
        ir_by_base.setdefault(norm(p), set()).update(s)

    print('exe 쪽 .rs 경로 %d개 · IR 쪽 %d개' % (len(exe_lines), len(ir_by_base)))
    matched = miss_file = 0
    bad = []
    for p, s in sorted(exe_lines.items()):
        if want not in p:
            continue
        b = norm(p)
        if b not in ir_by_base:
            miss_file += 1
            continue
        matched += 1
        extra = s - ir_by_base[b]
        if extra:
            bad.append((b, len(s), sorted(extra)[:8]))
    print('IR 에서 짝을 찾은 파일 %d개 · 못 찾은 파일 %d개' % (matched, miss_file))
    if not bad:
        print('★exe 의 패닉 줄이 전부 IR 안에 있다 = 같은 소스 빌드로 모순 없음')
    else:
        print('⚠exe 에만 있고 IR 에 없는 줄 = 소스가 다르다는 증거 (%d 파일)' % len(bad))
        for b, n, e in bad[:25]:
            print('   %-52s exe줄 %d개 중 IR 밖: %s' % (b, n, e))


if __name__ == '__main__':
    main()
