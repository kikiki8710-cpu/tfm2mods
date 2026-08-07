# -*- coding: utf-8 -*-
"""posmodel.py - 적 위치추정 모델 4사이트의 상수 사이트 정밀 열거 (2026-08-05)"""
import sys
sys.path.insert(0, r'C:\tfm2mods\v54')
import cen
for v, w, d in [(260001, 4, '300001-40000 융합상수'),
                (50137730271000, 8, '속도계수 (>>43 = 5.7)'),
                (3000, 4, '기저속도'),
                (40000, 4, '측정오차 여유'),
                (300001, 4, '판정 임계'),
                (0x2e8, 4, 'per-team struct stride 744'),
                (0x8c0, 4, 'entity stride 2240')]:
    by = cen.sites('054', v, w, 'path_finder', False)
    tot = sum(len(x) for x in by.values())
    print('### %d (0x%x) %-24s path_finder계열 %d사이트' % (v, v & (2**64-1), d, tot))
    for k in by:
        for a, fs, i in by[k]:
            print('    %06x fn %06x  %-28s %s %s' % (a, fs, i.bytes.hex(), i.mnemonic, i.op_str))
    sys.stdout.flush()
