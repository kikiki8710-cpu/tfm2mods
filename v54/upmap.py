# -*- coding: utf-8 -*-
"""upmap.py - 신규 경로/거리 시스템의 상위 소비자 지도 (역방향 BFS, 소스파일 집계)"""
import sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
import cg

S, fwd, rev = cg.build('054')
seeds = {}
seeds['astar'] = [0xc4a1c0,0xc4b320,0xc4c640,0xc4d9a0,0xc4ecc0,0xc4fe50,0xc51010,0xc52640,
                  0xc53c70,0xc54cd0,0xc56090,0xc57450,0xc585a0,0xc596f0,0xc5a760,0xc5b9e0,
                  0xc5ca50,0xc5dcd0,0xc5f080,0xc60190]
seeds['free_dist_calc'] = [0xe0d740]
seeds['minion_wave_risk'] = [0xd30f80,0xd2b390,0xd2bb10,0xd2c630]
seeds['pos_estimate'] = [0xd2ff50,0xd31590,0xd31760,0xd320a0]
for k, sd in seeds.items():
    print('===== %s =====' % k)
    cur = set(sd); seen = set(sd)
    for d in range(1, 6):
        nx = set()
        for f in cur:
            for c in rev.get(f, ()):
                if c not in seen:
                    seen.add(c); nx.add(c)
        agg = collections.Counter()
        for c in nx:
            agg[cg.srcname('054', c)] += 1
        print('  -- depth %d : %d fn' % (d, len(nx)))
        for s, n in agg.most_common(14):
            print('     [%2d] %s' % (n, s[:88]))
        cur = nx
        if not nx:
            break
    sys.stdout.flush()
