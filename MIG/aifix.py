# -*- coding: utf-8 -*-
r"""aifix — 디컴이 실패한 자리를 **Ghidra 쪽에서 고쳐** 다시 뽑는다

    python MIG\aifix.py <decomp디렉터리> [--port 8081] [--noreturn 0x142b1b410,...]

`aifill` 이 채운 산출물에는 두 종류의 구멍이 남는다. 둘 다 원인이 Ghidra 쪽에 있어
스크립트만으로는 못 메우고, **Ghidra DB 를 고쳐야** 풀린다.

  ① `DECOMP FAILED: … 함수미정의` — 대상 주소의 xref 가 `.rdata` DATA 뿐(간접호출 전용
     진입점)이라 오토애널라이저가 함수를 만들지 않았다. → `/create_function`
  ② **noreturn 오판** — 정상 반환하는 할당 래퍼가 noreturn 으로 표시돼 있으면, 그것을
     호출하는 **모든 함수의 디컴이 첫 호출에서 잘린다**(에러 없이 조용히).
     0.5.8 은 `0x142b1b410`(HeapAlloc 래퍼)이 그랬고 14개 함수가 통째로 사라졌다.
     → `/set_noreturn?value=false`

두 엔드포인트는 이 프로젝트에서 GhidraMCP 플러그인에 직접 추가한 것이다(2026-09-05).
정본 플러그인 소스 = `C:\Users\jungs\Downloads\GhidraMCP-src`.

절차: noreturn 해제 → 함수 생성 → **저장** → 그 다음 `aifill --force --only …` 로 재수신.
"""
import argparse
import collections
import io
import os
import re
import sys
import urllib.parse
import urllib.request

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

BASE_IMG = 0x140000000
FAIL_RE = re.compile(r'\[RVA 0x([0-9a-f]+)\]\s*\n// DECOMP FAILED: *(.*)')
FAIL_RE2 = re.compile(r'// fn @ \S+\s+\(RVA 0x([0-9a-f]+)\)\s*\n// DECOMP FAILED: *(.*)')


def hit(port, path, timeout=600, **q):
    u = 'http://127.0.0.1:%d/%s' % (port, path)
    if q:
        u += '?' + urllib.parse.urlencode(q)
    return urllib.request.urlopen(u, timeout=timeout).read().decode('utf-8', 'replace').strip()


def scan_fails(root):
    """{사유: [(rva, 파일)]}"""
    out = collections.defaultdict(list)
    for dp, _, fns in os.walk(root):
        for f in sorted(fns):
            if not f.endswith('.md') or f == 'INDEX.md':
                continue
            p = os.path.join(dp, f)
            txt = io.open(p, encoding='utf-8').read()
            if 'DECOMP FAILED' not in txt:
                continue
            rel = os.path.relpath(p, root)
            for rx in (FAIL_RE, FAIL_RE2):
                for m in rx.finditer(txt):
                    reason = m.group(2).strip()[:46] or '(사유 없음)'
                    out[reason].append((m.group(1), rel))
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('root')
    ap.add_argument('--port', type=int, default=8081)
    ap.add_argument('--noreturn', default='',
                    help='noreturn 을 해제할 절대주소(콤마 구분). 예 0x142b1b410')
    ap.add_argument('--dry', action='store_true', help='조사만 하고 Ghidra 는 안 건드린다')
    a = ap.parse_args()

    fails = scan_fails(a.root)
    total = sum(len(v) for v in fails.values())
    print('디컴 실패 %d건 — 사유별:' % total)
    for k, v in sorted(fails.items(), key=lambda x: -len(x[1])):
        print('  %3d건  %s' % (len(v), k))
    print()

    if a.noreturn:
        print('== noreturn 해제 ==')
        for addr in [x.strip() for x in a.noreturn.split(',') if x.strip()]:
            print('  %s' % hit(a.port, 'set_noreturn', address=addr, value='false'))
        print()

    if a.dry:
        print('(--dry: Ghidra 변경 없음)')
        return

    # 함수 미정의 계열만 골라 생성 시도 — 디컴파일러 자체 실패는 함수 생성으로 안 풀린다.
    targets = []
    for reason, items in fails.items():
        if ('미정의' in reason) or ('No function' in reason) or ('만들지' in reason):
            targets += [rva for rva, _ in items]
    targets = sorted(set(targets))
    if not targets:
        print('함수 생성 대상 없음')
        return

    print('== 함수 생성 %d건 ==' % len(targets))
    ok = fail = 0
    CHUNK = 25
    for i in range(0, len(targets), CHUNK):
        batch = targets[i:i + CHUNK]
        addrs = ','.join('0x%x' % (int(x, 16) + BASE_IMG) for x in batch)
        res = hit(a.port, 'create_function', address=addrs)
        for line in res.split('\n'):
            line = line.strip()
            if not line:
                continue
            if ': OK' in line or 'already exists' in line:
                ok += 1
            else:
                fail += 1
                print('  ! %s' % line)
    print('  생성/기존 %d · 실패 %d' % (ok, fail))
    print()
    print('== 저장 ==')
    print('  %s' % hit(a.port, 'save_program'))
    print()
    print('다음: python MIG\\aifill.py %s --force --passes fill,cap,jt' % a.root)
    print('      (이미 채운 본문도 다시 받아야 새로 생긴 함수의 디컴이 반영된다)')


if __name__ == '__main__':
    main()
