# -*- coding: utf-8 -*-
"""데미지시트 desc sanity — 이 값이 틀리면 임의 바이트를 vtable 로 삼아 호출 = AV.
0.5.2 disc14 크래시 2·3차의 진범이 바로 이 상수 방치였다(rva_053.rs 주석).
desc 레이아웃 = {drop, size, align, ... , vt+0x30 = base데미지쌍 게터}
"""
import io, sys

sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000

CASES = [('C8C  (053)', '053', 0x31be1a8), ('C8C  (054)', '054', 0x3288e48),
         ('DISC7(053)', '053', 0x31bcef8), ('DISC7(054)', '054', 0x327fba0)]

for tag, ver, rva in CASES:
    E = load(ver)
    q = [E.u64(rva + i * 8) for i in range(8)]
    print('%s  %x' % (tag, rva))
    print('   drop=%x  size=%x  align=%x' % (q[0], q[1], q[2]))
    print('   +0x18..0x38 = %s' % ' '.join('%x' % x for x in q[3:8]))
    # vt+0x30 = desc 를 vtable 로 봤을 때 7번째 슬롯
    print('   vt+0x30 = %x  (RVA %x)' % (q[6], q[6] - B if q[6] > B else 0))
