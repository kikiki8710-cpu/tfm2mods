# -*- coding: utf-8 -*-
"""ls2.py - srcmap2 기반 함수 목록 조회 (0.5.4 경로/거리 조사용, 2026-08-05)
  python ls2.py <ver> <패턴...>   : 소스경로에 패턴이 든 함수 전부 (RVA/크기/줄)
"""
import io, os, sys
if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
D = r'C:\tfm2mods\v54'
BS = chr(92)


def rows(ver, which='2'):
    out = []
    p = os.path.join(D, '%s_srcmap%s.tsv' % (ver, which))
    for ln in io.open(p, encoding='utf-8'):
        s, e, src, l = ln.rstrip('\n').split('\t')
        out.append((int(s, 16), int(e, 16), src, l))
    return out


def short_of(src):
    return ' | '.join(x.split(BS)[-1] for x in src.split(' | '))


if __name__ == '__main__':
    ver = sys.argv[1]
    pats = [p.lower() for p in sys.argv[2:]]
    n = 0
    for s, e, src, l in rows(ver):
        low = src.lower()
        if any(p in low for p in pats):
            print('%06x-%06x %6dB  %-72s [%s]' % (s, e, e - s, short_of(src)[:72], l[:64]))
            n += 1
    print('-- %d개' % n)

