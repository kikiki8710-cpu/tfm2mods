# -*- coding: utf-8 -*-
"""모드가 쓰는 구조체 오프셋 후보를 0.5.3 / 0.5.4 exe 에서 disp32 출현수로 대조한다.
   판정 원리: 그 버전에 **존재하는** 필드 오프셋이면 게임 코드가 disp32 로 여러 번 접근한다.
   0.5.3 에서 많고 0.5.4 에서 0 이면 = 그 오프셋은 0.5.4 에서 죽었다(모드가 헛것을 읽는다)."""
import sys, io, struct
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
from pe2 import load

CAND = [
    ('vt+0x20 (seed/posA)', [0xeab8, 0xeaf8, 0xeb28]),
    ('vt+0x28 (tick/posB)', [0xeac0, 0xeb00, 0xeb30]),
    ('athlete: id',         [0x698, 0x810, 0x800]),
    ('athlete: +8 필드',    [0x6a0, 0x818, 0x808]),
    ('athlete: team',       [0x6a8, 0x820, 0x810]),
    ('athlete stride',      [0x8d0, 0x8c0]),
]

txt = {}
for ver in ('053', '054'):
    p = load(ver)
    sec = [s for s in p.sections if s[0] == '.text'][0]
    txt[ver] = p.rd(sec[1], sec[2])

for name, cands in CAND:
    print('== %s ==' % name)
    for c in cands:
        pat = struct.pack('<I', c)
        a, b = txt['053'].count(pat), txt['054'].count(pat)
        flag = ''
        if a > 0 and b == 0:
            flag = '   ← 0.5.4에서 죽음'
        elif b > a * 2 and b > 3:
            flag = '   ← 0.5.4 유력'
        print('   %#7x   0.5.3=%4d   0.5.4=%4d%s' % (c, a, b, flag))
    print()
