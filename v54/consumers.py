# -*- coding: utf-8 -*-
"""consumers.py - sub_plan/plan 핸들러 -> 0.5.4 신규 경로/거리 시스템 도달성 (2026-08-05)"""
import sys
sys.path.insert(0, r'C:\tfm2mods\v54')
import cg, ls2

SUB = [('line_defense','d76b20'),('line_attack','e65070'),('line_safe','e706f0'),
       ('line_total','d9f400'),('line_wait','e71dd0'),('recall','dabbc0'),
       ('jungle','d84880'),('battle','da3570'),('death_battle','e19fd0'),
       ('hide','d81820'),('epic_check','d752f0'),('epic_hunt','d860d0'),
       ('epic_poke','e6bb50'),('serpen_check','d7ffd0'),('serpen_hunt','e14a60'),
       ('serpen_poke','e66fc0'),('attack_nexus','da1850'),('defense_nexus','dac090'),
       ('steal','e1e390')]
PLAN = [('passive_line','cd5540'),('single_line','d66b50'),('passive_jungle','e619c0'),
        ('unknown7','dd4b20'),('ganker','dfe090'),('cover','cd7ab0'),
        ('epic_hunt_poke','d8ba90'),('epic_hunt_battle','cd7410'),
        ('serpen_hunt_poke','e1ff20'),('serpen_hunt_battle','cd7860'),
        ('defense_nexus_p','cd6980')]

# 신규 시스템 진입점
T = {}
T['free_dist_calc'] = [0xe0d740, 0xdfbd40]
pf = [0xde5a50,0xde6020,0xde6450,0xde6b60,0xde7140,0xde7530,0xde7960,0xde7e80,
      0xde82a0,0xde8850,0xde8c80,0xde92d0,0xde96f0,0xde9b10]
T['path_field_flood'] = pf
astar = [0xc4a1c0,0xc4b320,0xc4c640,0xc4d9a0,0xc4ecc0,0xc4fe50,0xc51010,0xc52640,
         0xc53c70,0xc54cd0,0xc56090,0xc57450,0xc585a0,0xc596f0,0xc5a760,0xc5b9e0,
         0xc5ca50,0xc5dcd0,0xc5f080,0xc60190]
T['astar_entry'] = astar
T['minion_wave_risk'] = [0xd30f80, 0xd2b390, 0xd2bb10, 0xd2c630]
T['pos_estimate'] = [0xd2ff50, 0xd31590, 0xd31760, 0xd320a0]
fdq = [0xdea090,0xdea7f0,0xdeaef0,0xdeb780,0xdebe80,0xdec850,0xdecf50,0xded650,
       0xdedd50,0xdee480,0xdeeb80,0xdef280,0xdef9d0,0xdf0120,0xdf0820,0xdf0ff0,
       0xdf16f0,0xdf1df0,0xdf24f0,0xdf2c40]
T['free_dist_query'] = fdq

allt = []
for v in T.values():
    allt += v
cols = list(T.keys())
print('%-20s %s' % ('handler', ' '.join('%-16s' % c for c in cols)))
for label, lst in (('SUB', SUB), ('PLAN', PLAN)):
    print('---- %s ----' % label)
    for nm, rv in lst:
        r = int(rv, 16)
        res = cg.reach('054', [r], allt, depth=10)
        found, nseen = res[r]
        row = []
        for c in cols:
            hits = [t for t in T[c] if t in found]
            if hits:
                d = min(found[t] for t in hits)
                row.append('O(%d개,d%d)' % (len(hits), d))
            else:
                row.append('-')
        print('%-20s %s   [%d fn]' % (nm, ' '.join('%-16s' % x for x in row), nseen))
        sys.stdout.flush()
