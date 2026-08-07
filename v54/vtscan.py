# -*- coding: utf-8 -*-
"""World/Sim 트레이트 오브젝트의 **구체 vtable 을 .rdata 에서 역추적**하는 도구.

배경: game-ai 는 World 를 `&dyn Sim` 팻포인터(World+0x00=data, World+0x08=vtable)로 잡고
      `call [vt+0x20]`, `call [vt+0x28]`, `call [vt+0x1e0]` 등을 쓴다. 슬롯의 의미를
      알려면 구체 vtable 이 필요한데, lea 로 실린 .rdata 주소 중 "슬롯 배열처럼 생긴 것"을
      찾아 조건(슬롯이 전부 .text 포인터, 길이 ≥ N)으로 거른다.

  python vtscan.py 054 0x1e0        # 최소 길이 0x1e0 바이트 이상인 vtable 후보 전수
"""
import sys, struct, re
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
from scan import Scanner, src_of


def main(ver, minlen):
    e = load(ver)
    secs = {n: (va, vsz, ra, rsz) for n, va, vsz, ra, rsz in e.sections}
    tva, tvsz = secs['.text'][0], secs['.text'][1]
    rd = [s for s in e.sections if s[0] == '.rdata'][0]
    rva0, rvsz = rd[1], rd[2]

    def istext(v):
        if not (BASE <= v < BASE + 0x8000000):
            return False
        r = v - BASE
        return tva <= r < tva + tvsz

    cands = []
    a = rva0
    while a < rva0 + rvsz - 8:
        v = e.u64(a)
        if v is None:
            break
        # vtable 시작 = drop_in_place(.text) + size + align 패턴
        if istext(v):
            sz = e.u64(a + 8)
            al = e.u64(a + 0x10)
            if sz is not None and al in (1, 2, 4, 8, 16) and 0 < (sz or 0) < 0x100000:
                n = 3
                while True:
                    w = e.u64(a + n * 8)
                    if w is None or not istext(w):
                        break
                    n += 1
                if n * 8 >= minlen:
                    cands.append((a, n, sz, al))
                    a += n * 8
                    continue
        a += 8
    print('vtable 후보 %d개 (슬롯 %d개 이상)' % (len(cands), minlen // 8))
    for a, n, sz, al in cands:
        print('  rdata %08x  slots=%d  size=%d align=%d   [+0x20]=%06x  [+0x28]=%06x'
              % (a, n, sz, al, (e.u64(a + 0x20) or BASE) - BASE, (e.u64(a + 0x28) or BASE) - BASE))


if __name__ == '__main__':
    main(sys.argv[1], int(sys.argv[2], 0))
