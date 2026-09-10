#!/usr/bin/env python3
"""callees.py — exe 함수 하나가 **직접 call 하는 대상**을 스캐폴딩 정보(명령수·소스파일)와 함께 나열한다.

왜: 에이전트 단위 트윈 잔차를 좁히려면 `get_input` 아래 계층을 하나씩 대조해야 하는데,
    IR 의 함수 이름 ↔ exe RVA 매핑이 없는 함수(패닉 Location 없음)는 스캐폴딩으로 못 찾는다.
    호출 순서와 크기로 IR 쪽 호출 목록과 맞춘다.

사용: python MIG\\callees.py 0xe900b0 [크기]
"""
import glob
import io
import os
import re
import struct
import sys

import capstone

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
BASE = 0x140000000
ROOT = r'C:\tfm2mods\MIG\decomp\0.5.8'


def sections(d):
    pe = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, pe + 6)[0]
    opt = struct.unpack_from('<H', d, pe + 20)[0]
    return [struct.unpack_from('<IIII', d, pe + 24 + opt + i * 40 + 8) for i in range(nsec)]


def main():
    d = open(EXE, 'rb').read()
    secs = sections(d)

    def off(r):
        for vsz, va, rsz, ra in secs:
            if va <= r < va + max(vsz, rsz):
                return ra + r - va
        return None

    info = {}
    for md in glob.glob(os.path.join(ROOT, '**', '*.md'), recursive=True):
        t = io.open(md, encoding='utf-8').read()
        key = os.path.relpath(md, ROOT).replace(os.sep, '/')
        for m in re.finditer(r'^## `(0x[0-9a-f]+)`.*?\n\| RVA \| `0x[0-9a-f]+` ~ `(0x[0-9a-f]+)` \((\d+) B\) \|\n\| 명령 수 \| (\d+) \|', t, re.M | re.S):
            info[int(m.group(1), 16)] = (int(m.group(3)), int(m.group(4)), key)

    start = int(sys.argv[1], 16)
    size = int(sys.argv[2]) if len(sys.argv) > 2 else (info.get(start, (4096,))[0])
    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    seen, order = {}, []
    for ins in cs.disasm(d[off(start):off(start) + size], BASE + start):
        if ins.mnemonic == 'call' and ins.op_str.startswith('0x'):
            t = int(ins.op_str, 16) - BASE
            if t not in seen:
                seen[t] = ins.address - BASE
                order.append(t)
    print('0x%x (%d B) 이 직접 call 하는 대상 %d개' % (start, size, len(order)))
    for t in order:
        i = info.get(t)
        print('   0x%-8x @0x%-8x  %s' % (t, seen[t], ('%5d명령  %s' % (i[1], i[2])) if i else '(census 밖 / AI 계층 밖)'))


if __name__ == '__main__':
    main()
