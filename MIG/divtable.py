#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""divtable.py — `dyn Trait` 의 **vtable 슬롯 오프셋 → 함수 이름**.

왜 필요한가 (1~3차 명세 배치 실측):
  `unknown` 478건 중 **36건(7.5%)이 "vtable 슬롯 이름을 못 찾음"** 이었다.
  담당자들이 DWARF `vtable_type$` 를 뒤졌는데 거기엔 `__method8` `__method62` 처럼
  **번호만** 있다(game_core 트레이트 impl DWARF 가 `_gaibc` 에 없다).
  ⟹ 대부분 "원리적으로 불가"로 포기했다.

  그런데 한 담당자가 우회로를 찾았다: **vtable 전역 상수의 함수포인터 배열을 순서대로 읽는다.**
  이건 IR 에 그대로 있다. 그 방법을 도구로 만든다.

vtable 전역 레이아웃 (Rust):
  <{ ptr drop_glue, [16 x i8] size+align, ptr method0, ptr method1, ... }>
  ⟹ 바이트 오프셋 0x00=drop / 0x08=size / 0x10=align / 0x18부터 8B 씩 메서드

⚠담당자 경고(실측): 값 목록을 **쉼표 단위로 순서대로** 뽑아야 한다.
  `grep '@_RNvX'` 처럼 심볼 접두로 필터하면 `_RNvY`(shim) 항목이 빠져 **슬롯이 밀린다**.
  실제로 그렇게 0x40 을 엉뚱한 함수로 오독할 뻔한 사례가 있었다.

사용:
  python divtable.py AbstractGame            # 후보 vtable 목록
  python divtable.py AbstractGame 0x28       # 그 슬롯이 무슨 함수인지
"""
import io
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

IRDIR = r'C:\tfm2mods\_gaibc'
# 전역 상수 한 줄 전체를 잡는다(값 목록이 매우 길다).
RE_GLOB = re.compile(r'^(@[\w.]+) = (?:private )?(?:unnamed_addr )?constant <\{([^\n]*)$', re.M)


def mangled_parts(sym):
    out, i, n = [], 0, len(sym)
    while i < n:
        if sym[i].isdigit() and (i == 0 or not sym[i - 1].isdigit()):
            j = i
            while j < n and sym[j].isdigit():
                j += 1
            ln = int(sym[i:j])
            if 0 < ln <= n - j:
                c = sym[j:j + ln]
                # ⚠`NtB4_12AbstractGame` 처럼 백참조(`B4_`) 뒤에서 길이를 잘못 집으면
                #   `_12Abs` 같은 쓰레기 성분이 나온다. 밑줄로 시작하면 버린다.
                if re.fullmatch(r'[A-Za-z][A-Za-z0-9_]*', c):
                    out.append(c)
            i = j
        else:
            i += 1
    return out


def slots_of(valpart):
    """값 목록에서 **순서대로** 항목을 뽑아 (바이트오프셋 -> 심볼) 을 만든다.

    항목은 `ptr @sym` / `[16 x i8] c"..."` / `ptr null` 등. 크기를 누적해 오프셋을 센다.
    """
    out, off = {}, 0
    # ⚠전역 한 줄은 `constant <{타입목록}> <{값목록}>` 이다. **타입 선언부를 같이 세면**
    #   오프셋이 통째로 밀린다(1차 시험에서 최소 오프셋이 0x328 로 나온 원인).
    #   `}> <{` 뒤의 값 목록만 취한다.
    m = re.search(r'\}>\s*<\{(.*)$', valpart)
    if m:
        valpart = m.group(1)
    # `c"..."` 안에 쉼표가 들어갈 수 있으니 문자열 리터럴을 먼저 치환해 보호한다.
    lits = []

    def stash(m):
        lits.append(m.group(0))
        return '\x00LIT%d\x00' % (len(lits) - 1)

    v = re.sub(r'c"(?:[^"\\]|\\.)*"', stash, valpart)
    for item in v.split(','):
        item = item.strip()
        if not item:
            continue
        m = re.match(r'\[(\d+) x i8\]', item)
        if m:
            off += int(m.group(1))
            continue
        if item.startswith('ptr'):
            s = re.search(r'@([\w.$]+)', item)
            if s:
                out[off] = s.group(1)
            off += 8
            continue
        m = re.match(r'i(\d+)\b', item)
        if m:
            off += max(int(m.group(1)) // 8, 1)
            continue
        off += 8          # 모르는 항목도 포인터 크기로 가정(순서를 잃지 않는 게 중요)
    return out


def find(want):
    res = []
    for fn in sorted(os.listdir(IRDIR)):
        if not fn.endswith('.ll'):
            continue
        text = io.open(os.path.join(IRDIR, fn), encoding='utf-8', errors='replace').read()
        for name, val in RE_GLOB.findall(text):
            if want.lower() not in val.lower():
                continue
            sl = slots_of(val)
            if len(sl) < 3:
                continue
            res.append((fn, name, sl))
    return res


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return
    want = sys.argv[1]
    q = None
    if len(sys.argv) > 2:
        a = sys.argv[2]
        q = int(a, 16) if a.lower().startswith('0x') else int(a)

    res = find(want)
    if not res:
        print('없음: %s' % want)
        return
    # 슬롯이 가장 많은 것 = 진짜 트레이트 vtable(작은 건 클로저 vtable)
    res.sort(key=lambda r: -len(r[2]))
    for fn, name, sl in res[:2]:
        print('=== %s  (%s · 슬롯 %d개) ===' % (name[:44], fn, len(sl)))
        if q is not None:
            sym = sl.get(q)
            if sym:
                p = mangled_parts(sym)
                print('  %s → %s' % (hex(q), '::'.join(p[-3:]) or sym))
                print('     %s' % sym[:110])
            else:
                near = sorted(sl)
                print('  %s → (그 오프셋에 항목 없음). 가진 오프셋 예: %s'
                      % (hex(q), ' '.join(hex(x) for x in near[:8])))
        else:
            for off in sorted(sl)[:40]:
                p = mangled_parts(sl[off])
                print('  %-7s %s' % (hex(off), '::'.join(p[-3:]) or sl[off][:60]))
        print()


if __name__ == '__main__':
    main()
