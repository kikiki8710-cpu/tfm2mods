# -*- coding: utf-8 -*-
r"""ainoret — noreturn 으로 **오판된 함수**를 전수로 찾아 해제한다

    python MIG\ainoret.py <decomp디렉터리> [--fix] [--port 8081]

★배경: Ghidra 의 `Non-Returning Functions - Discovered` 분석기는 호출 패턴만 보고 함수를
noreturn 으로 단정한다. 한 번 그렇게 표시되면 **그 함수를 호출하는 모든 함수의 디컴이
호출 지점에서 끊긴다** — 경고도 에러도 없이.

⚠**분석기를 꺼도 이미 붙은 표시는 사라지지 않는다.** 0.5.8 실측: `0x142b1b410`(HeapAlloc
래퍼)을 풀고 분석기를 껐는데도 `memcpy` 가 여전히 noreturn 이라 AUCTION 최종선택기
(`0xe65b10`, 19,492 B)가 **디컴 49행**으로 나왔다.

이 도구는 디컴 산출물에서 `/* WARNING: Subroutine does not return */` 바로 다음 줄의
호출 대상을 모아 **빈도순**으로 보여준다. 진짜 noreturn(패닉·abort)과 오판을 가르는 기준:
  · 진짜: `FUN_1431a37c0`(rust panic) 계열 — 풀면 안 된다
  · 오판: 할당·복사 유틸(`memcpy`·`HeapAlloc` 래퍼) — 정상 반환하므로 풀어야 한다
`--fix` 는 **이름이 알려진 C 런타임 유틸**과 `--extra` 로 지정한 것만 푼다(패닉은 건드리지 않음).
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

WARN = '/* WARNING: Subroutine does not return */'
CALL = re.compile(r'([A-Za-z_][A-Za-z0-9_]*)\s*\(')
# 정상 반환하는 게 확실한 C 런타임/할당 유틸 — 오판이면 풀어도 안전하다.
SAFE = re.compile(r'^(memcpy|memmove|memset|malloc|calloc|realloc|free|strcpy|strlen|'
                  r'wcscpy|wcslen|_alloca_probe|__chkstk)$', re.I)


def hit(port, path, timeout=600, **q):
    u = 'http://127.0.0.1:%d/%s' % (port, path)
    if q:
        u += '?' + urllib.parse.urlencode(q)
    return urllib.request.urlopen(u, timeout=timeout).read().decode('utf-8', 'replace').strip()


def collect(root):
    """{호출대상: 등장횟수}"""
    cnt = collections.Counter()
    where = {}
    for dp, _, fns in os.walk(root):
        for f in sorted(fns):
            if not f.endswith('.md') or f == 'INDEX.md':
                continue
            p = os.path.join(dp, f)
            rel = os.path.relpath(p, root).replace('\\', '/')
            lines = io.open(p, encoding='utf-8').read().split('\n')
            for i, ln in enumerate(lines):
                if WARN not in ln:
                    continue
                for j in range(i + 1, min(i + 4, len(lines))):
                    m = CALL.search(lines[j])
                    if m and m.group(1) not in ('if', 'while', 'for', 'switch', 'return'):
                        cnt[m.group(1)] += 1
                        where.setdefault(m.group(1), rel)
                        break
    return cnt, where


def resolve(port, name):
    """이름 → 절대주소. FUN_140xxxx 는 이름에서 바로, 나머지는 서버에 물어본다."""
    m = re.match(r'^FUN_([0-9a-fA-F]{9,})$', name)
    if m:
        return '0x' + m.group(1).lower()
    try:
        r = hit(port, 'searchFunctions', query=name, limit=5)
    except Exception:
        return None
    for ln in r.split('\n'):
        mm = re.search(r'\bat\s+([0-9a-f]{9,})', ln) or re.search(r'@\s*([0-9a-f]{9,})', ln)
        if mm and name in ln:
            return '0x' + mm.group(1)
    return None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('root')
    ap.add_argument('--port', type=int, default=8081)
    ap.add_argument('--fix', action='store_true')
    ap.add_argument('--extra', default='', help='추가로 풀 이름(콤마 구분)')
    a = ap.parse_args()

    cnt, where = collect(a.root)
    if not cnt:
        print('noreturn 경고 없음 — 깨끗하다')
        return
    extra = set(x.strip() for x in a.extra.split(',') if x.strip())
    print('noreturn 으로 잘린 호출 %d종:' % len(cnt))
    print('  %-34s %-7s %s' % ('호출 대상', '횟수', '판정'))
    targets = []
    for nm, c in cnt.most_common():
        safe = bool(SAFE.match(nm)) or nm in extra
        verdict = '★오판(해제 대상)' if safe else '(패닉 등 — 유지)'
        print('  %-34s %-7d %s   %s' % (nm, c, verdict, where.get(nm, '')))
        if safe:
            targets.append(nm)

    if not a.fix:
        print()
        print('(--fix 를 주면 ★표시된 것만 해제한다)')
        return

    print()
    print('== noreturn 해제 ==')
    for nm in targets:
        addr = resolve(a.port, nm)
        if not addr:
            print('  %s: 주소 못 찾음 — 수동 확인 필요' % nm)
            continue
        try:
            print('  %s' % hit(a.port, 'set_noreturn', address=addr, value='false'))
        except Exception as e:
            print('  %s(%s): %s' % (nm, addr, e))
    print()
    print('  %s' % hit(a.port, 'save_program', timeout=1800))
    print()
    print('다음: aibound.py --fix 로 경계를 다시 잡고, aifill 로 재수신한다.')


if __name__ == '__main__':
    main()
