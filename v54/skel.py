# -*- coding: utf-8 -*-
"""두 함수가 **같은 함수인가**를 골격(mnemonic 열)로 판정한다.

RVA·상수는 버전마다 바뀌므로 비교 대상에서 뺀다. 남는 것은 명령 종류의 순서 =
컴파일 결과의 뼈대. 이게 90% 넘게 겹치면 "같은 함수인데 자리만 옮긴 것"이고,
60% 아래면 "로직이 바뀐 것"으로 본다.

  python skel.py cc9d70 ca87f0
"""
import io, sys, difflib

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE

_c = {}


def skel(ver, start):
    if ver not in _c:
        _c[ver] = load(ver)
    e = _c[ver]
    f = e.func_of(start)
    if not f:
        return None, None
    out = []
    for i in e.dis(f[0], f[1] - f[0]):
        out.append(i.mnemonic)
    return out, f


def cmp(a_rva, b_rva, verbose=False):
    sa, fa = skel('053', a_rva)
    sb, fb = skel('054', b_rva)
    if sa is None or sb is None:
        print('함수 범위를 못 찾음')
        return
    # ⚠autojunk=False 필수 — 켜두면 mov/lea 처럼 자주 나오는 mnemonic 을
    #   "잡음"으로 버려서 6천 명령짜리 함수의 일치율이 3% 로 무너진다(실측).
    sm = difflib.SequenceMatcher(None, sa, sb, autojunk=False)
    r = sm.ratio()
    print('0.5.3 %06x-%06x  명령 %d개' % (fa[0], fa[1], len(sa)))
    print('0.5.4 %06x-%06x  명령 %d개' % (fb[0], fb[1], len(sb)))
    print('골격 일치율 = %.1f%%' % (r * 100))
    print('판정 =', '같은 함수(자리이동)' if r >= 0.90 else
          ('같은 함수 + 부분수정' if r >= 0.70 else
           ('대폭 수정' if r >= 0.45 else '다른 함수로 봐야 함')))
    if verbose:
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            if tag != 'equal':
                print('  %-8s 0.5.3[%d:%d] %s  →  0.5.4[%d:%d] %s'
                      % (tag, i1, i2, ' '.join(sa[i1:i2][:8]), j1, j2, ' '.join(sb[j1:j2][:8])))
    return r


if __name__ == '__main__':
    a = sys.argv[1:]
    cmp(int(a[0], 16), int(a[1], 16), verbose=('-v' in a))
