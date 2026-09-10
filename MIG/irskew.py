#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""irskew.py - SDK rlib(IR) 과 배포 exe 의 **소스 어긋남(skew)** 을 전수 탐지한다.

왜 필요한가 (2026-09-10 실사고):
  `defense_nexus::nexus_final_stand_uncached` 에서 게임과 내 링크 사본이 0.1~0.3% 갈렸다.
  원인은 재현 오류가 아니라 소스가 다른 것이었다:
      exe  : minion[+0x68]==1 && minion[+0x88]==1 && minion[+0x90]==*(nexus+0x5c0)  ("넥서스를 공격 중인가")
      rlib : dx*dx+dy*dy < 14_400_000_001                                            ("반경 120,000 안인가")
  => `agent_link` 의 대전제("링크한 SDK rlib == exe 의 game_ai")가 최소 한 함수에서 거짓.

  기존 "SDK==exe (패닉 위치 528/528 일치)" 판정은 **필요조건일 뿐**이다.
  패닉을 만들지 않는 조건식 변경은 그 검사를 그대로 통과한다. 이 도구가 그 사각지대를 메운다.

원리:
  IR 함수의 **판별력 있는 상수**(큰 정수 리터럴)를 뽑아, 대응하는 exe 함수 본문 바이트에
  그 상수가 리틀엔디안으로 들어 있는지 본다. 하나도 없으면 그 함수는 skew 의심이다.

설계상 반드시 필요한 두 가지 (둘 다 빠뜨렸다가 확인된 사례를 놓쳤다):
  1) **함수 끝 = 같은 파일의 다음 DISubprogram 시작 줄**.
     DILocation 최대값으로 잡으면 인라인된 콜리의 줄까지 들어와 범위가 부풀고 후보가 애매해진다.
  2) **인라인되는 헬퍼의 상수까지 합칠 것. 헬퍼는 다른 모듈 파일에 있을 수 있다**(m00..m15).
     확인된 사례의 반경 상수는 m04 의 함수가 아니라 m11 의 try_fold 인스턴스 안에 있었다.

판정의 한계(반드시 읽을 것):
  - 상수는 접힐 수 있다(`x < N` -> `x <= N-1` 등). 그래서 N, N+-1 을 함께 본다.
  - "없음"은 **의심**이지 확정이 아니다. 확정은 해당 함수 디스어셈으로 한다.
  - "있음"은 강한 무죄 신호다(그 상수를 쓰는 코드가 exe 에 그대로 있다).

사용:
  python MIG/irskew.py                 # 전체 스캔
  python MIG/irskew.py --name nexus    # 이름 필터(검증용)
  python MIG/irskew.py --all           # 무죄 함수도 출력
"""
import argparse
import glob
import io
import json
import os
import re
import struct
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
AIMAP = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'aimap.json')
IRDIR = r'C:\tfm2mods\_gaibc'

MIN_CONST = 100000
MAX_CONST = 1 << 52
DENY = {0xFFFFFFFFFFFFFFF0, 0x28F5C28F5C28F5C3, 576460752303424, 4278190335,
        1114112, 1835008, 2097143, 2097151, 99999999, 999999999, 9999999}

DEF_RE = re.compile(r'^define[^\n]*@([A-Za-z0-9_.$]+)\(', re.M)
SUB_RE = re.compile(r'!DISubprogram\(name: "([^"]+)", linkageName: "([^"]+)"[^)]*?file: (![0-9]+), line: (\d+)')
FILE_RE = re.compile(r'(![0-9]+) = !DIFile\(filename: "([^"]+)"')
CALL_RE = re.compile(r'(?:call|invoke).*?@([A-Za-z0-9_.$]+)\(')
NUM_RE = re.compile(r'(?<![\w.])(-?\d{6,})(?![\w.])')


def sections(d):
    pe = struct.unpack_from('<I', d, 0x3c)[0]
    n = struct.unpack_from('<H', d, pe + 6)[0]
    opt = struct.unpack_from('<H', d, pe + 20)[0]
    return [struct.unpack_from('<IIII', d, pe + 24 + opt + i * 40 + 8) for i in range(n)]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--name', default='')
    ap.add_argument('--all', action='store_true')
    a = ap.parse_args()

    # 1패스: 모든 모듈의 define 을 (상수집합, 콜리목록) 으로 색인.
    #        본문 텍스트는 들고 있지 않는다(수백 MB -> OOM 위험).
    sym_const, sym_calls, top = {}, {}, {}
    for f in sorted(glob.glob(os.path.join(IRDIR, 'm*.ll'))):
        t = io.open(f, encoding='utf-8', errors='ignore').read()
        fmap = dict(FILE_RE.findall(t))
        starts = {}
        for m in SUB_RE.finditer(t):
            starts[m.group(2)] = (m.group(1), fmap.get(m.group(3), '?'), int(m.group(4)))
        per_file = {}
        for _n, _f, _ln in starts.values():
            per_file.setdefault(_f, set()).add(_ln)
        per_file = {k: sorted(v) for k, v in per_file.items()}

        for m in DEF_RE.finditer(t):
            sym = m.group(1)
            j = t.find('\n}\n', m.start())
            body = t[m.start():j if j > 0 else m.start() + 20000]
            cs = set()
            for c in NUM_RE.findall(body):
                v = abs(int(c))
                if MIN_CONST <= v < MAX_CONST and v not in DENY:
                    cs.add(v)
            sym_const[sym] = cs
            sym_calls[sym] = [x for x in set(CALL_RE.findall(body)) if not x.startswith('llvm.')]
            if sym in starts:
                nm, src, ln = starts[sym]
                nxt = [v for v in per_file.get(src, []) if v > ln]
                top[sym] = (nm, src, ln, (nxt[0] - 1) if nxt else ln + 400)
        del t

    # 2패스: 인라인되는 헬퍼(자기 DISubprogram 이 없는 내부 함수)의 상수를 합친다.
    def gather(sym, depth=3):
        out, seen, q = set(sym_const.get(sym, ())), {sym}, [sym]
        for _ in range(depth):
            nxt = []
            for s in q:
                for c in sym_calls.get(s, ()):
                    # exe 는 DISubprogram 이 있는 제네릭 인스턴스(try_fold 등)도 **인라인**한다.
                    #   'top 이면 별도 함수' 라는 규칙으로 걸렀다가 확인된 사례를 놓쳤다.
                    #   과다 포함은 판정을 보수적으로만 만든다(강한 의심 -> 약한 신호).
                    if c in seen or c not in sym_const:
                        continue
                    seen.add(c)
                    out |= sym_const[c]
                    nxt.append(c)
            q = nxt
            if not q:
                break
        return out

    d = open(EXE, 'rb').read()
    secs = sections(d)

    def off(r):
        for vsz, va, rsz, ra in secs:
            if va <= r < va + max(vsz, rsz):
                return ra + r - va
        return None

    info = json.load(open(AIMAP)).get('info', {})
    by_mod = {}
    for rva, e in info.items():
        if not isinstance(e, dict):
            continue
        mod = str(e.get('mod', ''))
        ls = [l for l in e.get('lines', []) if isinstance(l, int)]
        if not mod or not ls:
            continue
        by_mod.setdefault(mod.replace(chr(92), '/').split('/')[-1], []).append(
            (min(ls), max(ls), int(rva, 16), int(e.get('bytes', 0))))
    for k in by_mod:
        by_mod[k].sort()

    rows = []
    for sym, (nm, src, ln, end) in top.items():
        if a.name and a.name not in nm:
            continue
        consts = gather(sym)
        if not consts:
            continue
        base = src.replace(chr(92), '/').split('/')[-1].replace('.rs', '')
        hit = [c for c in by_mod.get(base, []) if ln <= c[0] <= end]
        if len(hit) != 1:
            continue
        _lo, _hi, rva, sz = hit[0]
        o = off(rva)
        if o is None or sz <= 0:
            continue
        eb = d[o:o + sz]
        found, missing = [], []
        for v in sorted(consts):
            ok = False
            for delta in (0, -1, 1):
                for width in (8, 4):
                    try:
                        if eb.find((v + delta).to_bytes(width, 'little')) >= 0:
                            ok = True
                            break
                    except OverflowError:
                        pass
                if ok:
                    break
            (found if ok else missing).append(v)
        rows.append((nm, base, ln, rva, sz, found, missing))

    rows.sort(key=lambda r: (-len(r[6]), -len(r[5])))
    susp = [r for r in rows if r[6] and not r[5]]
    part = [r for r in rows if r[6] and r[5]]
    print('IR 함수 %d개 대조' % len(rows))
    print('  전부 누락(skew 강한 의심) %d · 일부 누락 %d · 전부 일치 %d'
          % (len(susp), len(part), len(rows) - len(susp) - len(part)))
    print()
    print('=== 전부 누락 (디스어셈으로 확인할 것) ===')
    for nm, base, ln, rva, sz, fo, mi in susp[:50]:
        print('  %-42s %s:%-5d exe 0x%-8x %5dB  누락 %s' % (nm[:42], base, ln, rva, sz, mi[:4]))
    if part:
        print()
        print('=== 일부 누락 (약한 신호) ===')
        for nm, base, ln, rva, sz, fo, mi in part[:25]:
            print('  %-42s exe 0x%-8x 일치 %d / 누락 %d %s' % (nm[:42], rva, len(fo), len(mi), mi[:3]))
    if a.all:
        print()
        print('=== 전부 일치 (무죄) ===')
        for nm, base, ln, rva, sz, fo, mi in rows:
            if not mi:
                print('  %-42s exe 0x%-8x 상수 %d개 일치' % (nm[:42], rva, len(fo)))


main()
