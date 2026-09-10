#!/usr/bin/env python3
"""callcount.py — exe 함수 하나가 **직접 call 하는 대상별 호출 횟수**를 센다.

왜: IR 은 콜리 이름과 호출 횟수를 알고, exe 는 콜리 RVA 와 호출 횟수를 안다.
    이미 아는 몇 개(possible_risk 0xd83230 · position_eval_at 0xd84db0 · 자기자신)를 닻으로 삼아
    **횟수 다중도로 나머지 이름↔RVA 를 맞춘다.** aimap 은 AI 계층만 덮어 game_core 콜리는 이 방법이 필요하다.

사용: python MIG\\callcount.py 0xd57540 [크기]
"""
import struct
import sys

import capstone

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
BASE = 0x140000000


def sections(d):
    pe = struct.unpack_from('<I', d, 0x3c)[0]
    n = struct.unpack_from('<H', d, pe + 6)[0]
    opt = struct.unpack_from('<H', d, pe + 20)[0]
    return [struct.unpack_from('<IIII', d, pe + 24 + opt + i * 40 + 8) for i in range(n)]


def main():
    d = open(EXE, 'rb').read()
    secs = sections(d)

    def off(r):
        for vsz, va, rsz, ra in secs:
            if va <= r < va + max(vsz, rsz):
                return ra + r - va
        return None

    start = int(sys.argv[1], 16)
    size = int(sys.argv[2]) if len(sys.argv) > 2 else 9212
    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    cnt, first = {}, {}
    o = off(start)
    for ins in cs.disasm(d[o:o + size], BASE + start):
        if ins.mnemonic == 'call' and ins.op_str.startswith('0x'):
            t = int(ins.op_str, 16) - BASE
            cnt[t] = cnt.get(t, 0) + 1
            first.setdefault(t, ins.address - BASE)
    print('0x%x (%d B) 직접 call 대상 %d종' % (start, size, len(cnt)))
    for t, c in sorted(cnt.items(), key=lambda x: -x[1]):
        print('   %2d회  0x%-9x  (첫 호출 @0x%x)' % (c, t, first[t]))


if __name__ == '__main__':
    main()
