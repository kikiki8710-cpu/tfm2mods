# -*- coding: utf-8 -*-
r"""aibound — Ghidra 의 함수 경계를 **`.pdata` 진실값**과 대조한다

    python MIG\aibound.py <exe> [--port 8081] [--fix]

★왜 필요한가 (2026-09-05 실측으로 드러난 함정)

PE64 의 `.pdata`(RUNTIME_FUNCTION 테이블)는 링커가 적은 **함수 경계의 정답**이다.
그런데 Ghidra 는 흐름 분석으로 경계를 다시 정하기 때문에, **noreturn 오판이 하나 있으면
그 함수를 호출하는 모든 함수가 호출 지점에서 잘린 채로 경계가 굳는다.**

실측: `0xcdacd0` 은 `.pdata` 상 **7,857 B**(1,632 명령)인데 Ghidra 는 **66 B** 로 잡고 있었다.
원인은 `0x142b1b410`(HeapAlloc 래퍼)의 noreturn 오판. 디컴 결과는 5 줄짜리 래퍼처럼 보이고,
**에러도 경고도 없다** — 그래서 산출물만 봐서는 로직이 통째로 빠진 걸 알 수 없다.

⚠**noreturn 을 풀어도 경계는 저절로 안 늘어난다.** 이미 확정된 함수는 다시 만들어야 한다.
그래서 `--fix` 는 `.pdata` 범위를 body 로 **강제 지정해 재생성**한다(`/create_function` 의
`end`+`recreate` 인자, 이 프로젝트가 플러그인에 추가한 것).

기본 동작은 대조만 하고 아무것도 바꾸지 않는다.
"""
import argparse
import io
import os
import re
import struct
import sys
import time
import urllib.parse
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

BASE_IMG = 0x140000000
BODY_RE = re.compile(r'Body:\s*([0-9a-fA-F]+)\s*-\s*([0-9a-fA-F]+)')


def pdata_funcs(path):
    d = io.open(path, 'rb').read()
    e = struct.unpack_from('<I', d, 0x3c)[0]
    n = struct.unpack_from('<H', d, e + 6)[0]
    ss = e + 24 + struct.unpack_from('<H', d, e + 20)[0]
    secs = []
    for i in range(n):
        o = ss + i * 40
        vsz, va, rsz, pr = struct.unpack_from('<IIII', d, o + 8)
        secs.append((va, max(vsz, rsz), pr))
    opt = e + 24
    magic = struct.unpack_from('<H', d, opt)[0]
    dd = opt + (112 if magic == 0x20b else 96)
    prv, prs = struct.unpack_from('<II', d, dd + 3 * 8)
    off = None
    for va, sz, pr in secs:
        if va <= prv < va + sz:
            off = pr + (prv - va)
    out = {}
    for k in range(prs // 12):
        b, en, _ = struct.unpack_from('<III', d, off + 12 * k)
        if en > b:
            out[b] = en                      # start -> end (exclusive)
    return out


def hit(port, path, timeout=300, **q):
    u = 'http://127.0.0.1:%d/%s' % (port, path)
    if q:
        u += '?' + urllib.parse.urlencode(q)
    return urllib.request.urlopen(u, timeout=timeout).read().decode('utf-8', 'replace').strip()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('exe')
    ap.add_argument('--port', type=int, default=8081)
    ap.add_argument('-p', '--prefix', default='game-ai')
    ap.add_argument('--fix', action='store_true', help='어긋난 함수를 .pdata 범위로 재생성')
    ap.add_argument('--rebuild', action='store_true',
                    help='★경계가 맞아 보여도 **전 함수**를 clear+재디스어셈+setBody 로 재구성. '
                         'noreturn 오판을 푼 뒤에는 이게 필요하다 — 경계는 정상인데 '
                         '`CALL` 명령의 fall-through 가 없어 디컴만 끊긴 함수가 남기 때문이다')
    ap.add_argument('--min-ratio', type=float, default=0.9,
                    help='Ghidra body / pdata 크기 비가 이 값 미만이면 "잘림"으로 본다')
    a = ap.parse_args()

    import aidiff
    img, fn = aidiff.load(a.exe, a.prefix, False)
    pd = pdata_funcs(a.exe)
    print('AI 계층 함수 %d개 · .pdata 엔트리 %d개' % (len(fn), len(pd)))

    if a.rebuild:
        # ★noreturn 을 푼 직후에 쓴다. 경계가 맞아도 명령의 flow 가 옛 상태로 굳어 있으면
        #   디컴만 조용히 끊긴다(실측: AUCTION 최종선택기가 19,492 B 인데 디컴 46행).
        items = sorted(fn)
        print('== 전 함수 재구성 %d건 ==' % len(items))
        good = bad = 0
        t0 = time.time()
        for i, (b, e) in enumerate(items, 1):
            end = pd.get(b, e) - 1
            try:
                r = hit(a.port, 'create_function', timeout=1800,
                        address='0x%x' % (b + BASE_IMG), end='0x%x' % (end + BASE_IMG),
                        recreate='true')
                if ': OK' in r:
                    good += 1
                else:
                    bad += 1
                    print('  ! %s' % r.strip()[:120])
            except Exception as ex:
                bad += 1
                print('  ! 0x%x %s' % (b, type(ex).__name__))
            if i % 50 == 0:
                print('   …%d/%d  (%.0f초)' % (i, len(items), time.time() - t0))
                sys.stdout.flush()
        print('  성공 %d · 실패 %d  (%.0f초)' % (good, bad, time.time() - t0))
        try:
            print('  %s' % hit(a.port, 'save_program', timeout=1800))
        except Exception as ex:
            print('  저장 보류(%s) — ghidra_cycle.ps1 로 종료하면 저장된다' % type(ex).__name__)
        return

    short, missing, ok = [], [], 0
    for i, (b, e) in enumerate(sorted(fn)):
        if i % 100 == 0:
            print('   …%d/%d' % (i, len(fn)), file=sys.stderr)
        true_end = pd.get(b, e)
        try:
            r = hit(a.port, 'get_function_by_address', address='0x%x' % (b + BASE_IMG))
        except Exception as ex:
            missing.append((b, true_end, 'HTTP %s' % type(ex).__name__))
            continue
        m = BODY_RE.search(r)
        if not m:
            missing.append((b, true_end, r.split('\n')[0][:60]))
            continue
        gs = int(m.group(1), 16) - BASE_IMG
        ge = int(m.group(2), 16) - BASE_IMG + 1          # Ghidra 는 마지막 주소(포함)
        tsz, gsz = true_end - b, ge - gs
        if tsz > 0 and float(gsz) / tsz < a.min_ratio:
            short.append((b, true_end, ge, tsz, gsz))
        else:
            ok += 1

    print()
    print('정상 %d · ★경계 잘림 %d · 조회실패 %d' % (ok, len(short), len(missing)))
    if short:
        short.sort(key=lambda x: x[3] - x[4], reverse=True)
        print()
        print('가장 크게 잘린 것부터:')
        print('  %-12s %-10s %-10s %s' % ('RVA', 'pdata크기', 'Ghidra', '잃은 바이트'))
        for b, te, ge, tsz, gsz in short[:30]:
            print('  0x%-10x %-10d %-10d %d' % (b, tsz, gsz, tsz - gsz))
        lost = sum(x[3] - x[4] for x in short)
        print()
        print('  잘려나간 총 바이트 = %d (전체 AI 계층의 %.1f%%)'
              % (lost, 100.0 * lost / max(1, sum(pd.get(b, e) - b for b, e in fn))))

    if a.fix and short:
        print()
        print('== .pdata 범위로 재생성 %d건 ==' % len(short))
        good = bad = 0
        for b, te, ge, tsz, gsz in short:
            try:
                r = hit(a.port, 'create_function', timeout=600,
                        address='0x%x' % (b + BASE_IMG), end='0x%x' % (te + BASE_IMG - 1),
                        recreate='true')
                if ': OK' in r or 'recreated' in r:
                    good += 1
                else:
                    bad += 1
                    print('  ! %s' % r.strip()[:120])
            except Exception as ex:
                bad += 1
                print('  ! 0x%x %s' % (b, ex))
        print('  성공 %d · 실패 %d' % (good, bad))
        print('  %s' % hit(a.port, 'save_program', timeout=600))


if __name__ == '__main__':
    main()
