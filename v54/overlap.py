# -*- coding: utf-8 -*-
"""★패치 사이트끼리 **겹치는지** 검사한다.

`check054.py` 는 사이트 하나하나가 자기 명령 안에 들어가는지만 봤다. 하지만
**서로 다른 두 사이트가 같은 바이트 구간을 쓰면** 한쪽이 다른 쪽 명령을 깨뜨린다.
그러면 CPU 가 엉뚱한 명령을 실행해 **null 역참조 같은 전혀 다른 크래시**가 난다
(2026-08-05 실사고: RIP=`cab817` `imul r8d,eax,0x290` 인데 faultAddr=0x98).

또 하나 본다: 사이트의 쓰기 구간이 **자기 명령의 경계를 넘어 다음 명령을 침범**하는지.
"""
import io, os, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import sites as S1
import sites2 as S2
import reloc as R
B = 0x140000000
E4 = R.E4
_c = {}


def ins_map(rva):
    f = E4.func_of(rva)
    if not f:
        return None, None
    if f[0] not in _c:
        _c[f[0]] = {i.address - B: i for i in R.insns(E4, f[0], f[1])}
    return f, _c[f[0]]


def main():
    site = S1.parse() + S2.parse()
    # 쓰기 구간 [rva+off, rva+off+w)
    iv = []
    for x in site:
        iv.append((x['rva'] + x['off'], x['rva'] + x['off'] + x['w'], x))
    iv.sort(key=lambda t: (t[0], t[1]))

    print('사이트 %d개 — 쓰기 구간 겹침 검사' % len(site))
    bad = 0
    for k in range(len(iv) - 1):
        a0, a1, xa = iv[k]
        b0, b1, xb = iv[k + 1]
        if b0 < a1 and not (xa['rva'] == xb['rva'] and xa['off'] == xb['off'] and xa['w'] == xb['w']):
            bad += 1
            print('  ★겹침  [%06x,%06x) %s:%d   ↔   [%06x,%06x) %s:%d'
                  % (a0, a1, xa['file'], xa['line'], b0, b1, xb['file'], xb['line']))
    print('  겹침 %d건' % bad)

    print('\n명령 경계 침범 검사')
    over = 0
    for x in site:
        f, m = ins_map(x['rva'])
        if not m:
            continue
        i = m.get(x['rva'])
        if i is None:
            print('  ★명령시작 아님  %06x  %s:%d' % (x['rva'], x['file'], x['line']))
            over += 1
            continue
        if x['off'] + x['w'] > len(i.bytes):
            print('  ★경계침범  %06x off%d w%d > 길이%d  (%s %s)  %s:%d'
                  % (x['rva'], x['off'], x['w'], len(i.bytes), i.mnemonic, i.op_str,
                     x['file'], x['line']))
            over += 1
    print('  침범 %d건' % over)

    # 같은 RVA 를 다른 (off,w) 로 패치하는 경우 = 가드가 보류 처리 → 원본값 검사 없이 통과
    g = collections.defaultdict(set)
    for x in site:
        g[x['rva']].add((x['off'], x['w']))
    dup = {k: v for k, v in g.items() if len(v) > 1}
    print('\n같은 주소를 다른 (off,width) 로 패치 %d건 (원본값 가드가 보류 처리 = 무방비)' % len(dup))
    for k, v in list(dup.items())[:20]:
        print('  %06x  %s' % (k, sorted(v)))


if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    main()
