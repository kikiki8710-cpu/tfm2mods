# -*- coding: utf-8 -*-
"""census_run.py - 0.5.4 경로/거리 상수 전수 집계 배치 (2026-08-05)"""
import sys
sys.path.insert(0, r'C:\tfm2mods\v54')
import cen

VALS = [
    (640, 4, None, '보행 1칸 비용(직교)'),
    (896, 4, None, '보행 1칸 비용(대각)'),
    (1281, 4, None, '위험셀 가산(0x501)'),
    (32000, 4, None, '셀크기(월드)'),
    (16000, 4, None, '셀중심(월드)'),
    (900, 4, None, '노드수 30x30'),
    (899, 4, None, '노드 상한'),
    (14400000000, 8, None, '120000^2'),
    (6400000000, 8, None, '80000^2'),
    (8100000001, 8, None, '90000^2+1'),
    (50137730271000, 8, None, '속도계수'),
    (3000, 4, None, '기저속도'),
    (40000, 4, None, '위치추정 여유'),
    (300001, 4, None, '위치추정 임계'),
    (300000, 4, None, '위치추정 임계(=)'),
]
FILT = ['path_field', 'path_finder', 'free_dist', 'minion_wave', 'local.rs']
for v, w, _f, desc in VALS:
    by = cen.sites('054', v, w, None, False)
    tot = sum(len(x) for x in by.values())
    rel = {k: x for k, x in by.items() if any(f in k for f in FILT)}
    reltot = sum(len(x) for x in rel.values())
    print('### %-14s (0x%x) w%d  %-20s  전체 %4d사이트 / 경로계열 %3d사이트'
          % (v, v & 0xffffffffffffffff, w, desc, tot, reltot))
    for k in sorted(rel, key=lambda k: -len(rel[k])):
        print('     [%3d] %s' % (len(rel[k]), k[:90]))
    # 경로계열 밖 상위 3개도 표기
    oth = {k: x for k, x in by.items() if k not in rel}
    for k in sorted(oth, key=lambda k: -len(oth[k]))[:3]:
        print('      (타) [%3d] %s' % (len(oth[k]), k[:90]))
    sys.stdout.flush()
