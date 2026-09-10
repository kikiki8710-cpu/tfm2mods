#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""fnparts.py — 한 함수의 **조각 전부**(클로저·모노모픽 인스턴스)를 전 IR 에서 찾는다.

왜 필요한가 (1~3차 명세 배치 실측):
  `unknown` 478건 중 **64건(13.4%)이 "판정의 알맹이가 담당 범위 밖에 있었다"** 였다.
  이터레이터 술어(`filter`/`min_by_key` 의 클로저)가 담당 줄범위엔 `call_mut` 심만 남고
  본체는 **다른 .ll 파일**에 있다. 담당자마다 `grep -n "^define.*<함수명>" *.ll` 를
  손으로 돌렸고, 그걸 안 한 사람은 "술어가 없다"고 잘못 결론낼 뻔했다.
  ⟹ 배치를 띄울 때 **조각 목록을 미리 줘야 한다.**

사용:
  python fnparts.py defensive_crisis
  python fnparts.py v2_response_retreat_stance
"""
import io
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

IRDIR = r'C:\tfm2mods\_gaibc'
RE_DEF = re.compile(r'^define\b[^\n]*?@("[^"]+"|[\w.$]+)\(', re.M)


def mangled_parts(sym):
    """v0 망글의 <길이><이름> 성분을 순차 파싱(정규식으로는 해시 속 숫자에 걸린다)."""
    out, i, n = [], 0, len(sym)
    while i < n:
        if sym[i].isdigit() and (i == 0 or not sym[i - 1].isdigit()):
            j = i
            while j < n and sym[j].isdigit():
                j += 1
            ln = int(sym[i:j])
            if 0 < ln <= n - j:
                c = sym[j:j + ln]
                if re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]*', c):
                    out.append(c)
            i = j
        else:
            i += 1
    return out


def scan(want):
    hits = []
    for fn in sorted(os.listdir(IRDIR)):
        if not fn.endswith('.ll'):
            continue
        path = os.path.join(IRDIR, fn)
        lines = io.open(path, encoding='utf-8', errors='replace').read().split('\n')
        cur, start, sym = None, 0, ''
        for i, ln in enumerate(lines):
            if ln.startswith('define'):
                m = RE_DEF.match(ln)
                if m:
                    sym = m.group(1).strip('"')
                    cur = want in mangled_parts(sym) or want in sym
                    start = i
            elif ln == '}' and cur is not None:
                if cur:
                    parts = mangled_parts(sym)
                    # 조각 종류 추정: 클로저 / 모노모픽 인스턴스 / 본체
                    kind = '본체'
                    if any(p.startswith('closure') for p in parts):
                        kind = '클로저'
                    elif any(p in ('call_mut', 'call_once', 'call') for p in parts):
                        kind = 'call_mut 심'
                    elif any(p in ('fold', 'next', 'from_iter_in', 'min_by_key',
                                   'max_by_key', 'filter', 'map') for p in parts):
                        kind = '이터레이터'
                    hits.append((fn, start + 1, i + 1, i - start, kind, parts[-3:]))
                cur = None
    return hits


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return
    want = sys.argv[1]
    hits = scan(want)
    if not hits:
        print('없음: %s  (이름 성분이 정확한지 확인 — 예: sub_plan, defensive_crisis)' % want)
        return
    print('`%s` 의 조각 %d개' % (want, len(hits)))
    print()
    print('%-9s %8s %8s %6s  %-12s %s' % ('파일', '시작', '끝', '줄수', '종류', '이름 성분'))
    for fn, a, b, n, kind, parts in sorted(hits, key=lambda h: -h[3]):
        print('%-9s %8d %8d %6d  %-12s %s' % (fn, a, b, n, kind, '::'.join(parts)))
    print()
    big = [h for h in hits if h[3] >= 20 and h[4] != '본체']
    if big:
        print('⚠담당 범위 밖일 가능성이 큰 조각(20줄 이상, 본체 아님):')
        for fn, a, b, n, kind, parts in sorted(big, key=lambda h: -h[3])[:8]:
            print('   %s %d~%d (%d줄, %s)' % (fn, a, b, n, kind))


if __name__ == '__main__':
    main()
