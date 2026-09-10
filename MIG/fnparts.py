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
                # ⚠백레퍼런스(`B2L_`·`s_`)에서 길이를 잘못 집으면 `J_`·`_12Abs` 같은
                #   쓰레기 성분이 나온다(4차 실측: fold 조각이 `J_::compare::r` 로 표시됨).
                #   밑줄 시작·1글자·언더바 끝은 이름이 아니다.
                if re.fullmatch(r'[A-Za-z][A-Za-z0-9_]*', c) and len(c) >= 2 and not c.endswith('_'):
                    out.append(c)
            i = j
        else:
            i += 1
    return out


def count_disub(want):
    """DWARF 에 클로저·서브프로그램이 있는데 `define` 이 없으면 = **전부 인라인**.
    4차에서 3명이 "조각이 정말 없는 건지 못 찾은 건지" 몰라 재확인 grep 을 돌렸다."""
    n = 0
    for fn in sorted(os.listdir(IRDIR)):
        if not fn.endswith('.ll'):
            continue
        for ln in io.open(os.path.join(IRDIR, fn), encoding='utf-8', errors='replace'):
            if '!DISubprogram(' in ln and want in ln:
                n += 1
    return n


def count_declares(want):
    """`define` 은 없고 `declare` 만 있는 경우 = **외부 크레이트 심볼**(본체가 이 IR 에 없다).
    4차 보고: `fnparts range_adjust` 가 '이름 오타 의심'을 띄워 작성자가 이름만 계속 바꿔
    재시도하게 만들었다. 실제 원인은 game_core 외부 심볼이었다."""
    n = 0
    for fn in sorted(os.listdir(IRDIR)):
        if not fn.endswith('.ll'):
            continue
        for ln in io.open(os.path.join(IRDIR, fn), encoding='utf-8', errors='replace'):
            if ln.startswith('declare') and want in ln:
                n += 1
    return n


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
                    # ⚠`or want in sym` 를 두면 부분일치 오탐이 난다 — 4차 실측:
                    #   `fnparts CastingTarget` 이 `fmt::Debug::fmt` 을 조각으로 반환했다.
                    #   **엉뚱한 함수 본문을 읽게 되는** 조용한 오답이라 제거한다.
                    cur = want in mangled_parts(sym)
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
                    # ⚠클로저 두 개가 둘 다 `entity::Entity::call_mut` 로 뭉개져
                    #   어느 것이 어느 술어인지 구분이 안 됐다(4차 보고).
                    #   담당 함수 이름 뒤에 붙는 접미(`...retreat_stance0`, `s0_0`)를 살린다.
                    tail = ''
                    m2 = re.search(re.escape(want) + r'([sS]?[0-9_]*)', sym)
                    if m2 and m2.group(1):
                        tail = '  <%s%s>' % (want[:10], m2.group(1)[:6])
                    hits.append((fn, start + 1, i + 1, i - start, kind,
                                 parts[-3:] + ([tail.strip()] if tail else [])))
                cur = None
    return hits


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return
    want = sys.argv[1]
    # ⚠`sub_plan` 처럼 흔한 이름은 조각이 473개까지 나온다(4차 실측).
    #   `attack_nexus::sub_plan` 형태를 지원해 모듈로 좁힌다.
    scope = None
    if '::' in want:
        parts = [p for p in want.split('::') if p]
        scope, want = parts[-2], parts[-1]
    hits = scan(want)
    if scope:
        hits = [h for h in hits if any(scope in p for p in h[5]) or scope in h[0]]
        if not hits:
            print('`%s` 범위에서 못 찾음 — 범위 없이 다시 쳐 보라.' % scope)
            return
    if not hits:
        nd = count_declares(want)
        if nd:
            print('본체 없음 — `declare` 만 %d건.' % nd)
            print('  ⟹ **외부 크레이트 심볼**(game_core 등)이라 이 IR 에 본문이 없다.')
            print('     이름을 바꿔가며 재시도하지 마라. `unknown` 에 "외부 심볼"로 적으면 된다.')
        else:
            print('없음: %s  (이름 성분이 정확한지 확인 — 예: sub_plan, defensive_crisis)' % want)
        return
    print('`%s` 의 조각 %d개' % (want, len(hits)))
    print()
    print('%-9s %8s %8s %6s  %-12s %s' % ('파일', '시작', '끝', '줄수', '종류', '이름 성분'))
    for fn, a, b, n, kind, parts in sorted(hits, key=lambda h: (h[4] != '본체', -h[3])):
        print('%-9s %8d %8d %6d  %-12s %s' % (fn, a, b, n, kind, '::'.join(parts)))
    print()
    nsub = count_disub(want)
    if nsub > len(hits):
        print('※ DWARF 서브프로그램 %d개 vs 별도 define %d개 — 차이 %d개는 **전부 인라인**돼'
              % (nsub, len(hits), nsub - len(hits)))
        print('   별도 조각이 없다. (클로저가 본문 안에 녹아 있다는 뜻 — 더 뒤지지 마라.)')
        print()
    big = [h for h in hits if h[3] >= 20 and h[4] != '본체']
    if big:
        print('⚠담당 범위 밖일 가능성이 큰 조각(20줄 이상, 본체 아님):')
        for fn, a, b, n, kind, parts in sorted(big, key=lambda h: -h[3])[:8]:
            print('   %s %d~%d (%d줄, %s)' % (fn, a, b, n, kind))


if __name__ == '__main__':
    main()
