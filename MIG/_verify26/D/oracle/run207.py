# -*- coding: utf-8 -*-
"""26차 D · o207 드라이버 — 케이스당 프로세스 1개. 재구현(명세 logic) ↔ 실행 대조. 결과 = o207_cases.log"""
import subprocess, os, sys
sys.stdout.reconfigure(encoding='utf-8')
EXE = os.path.join(os.environ['TEMP'], 'tfm2_spanprobe', 'o207.exe')
LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'o207_cases.log')
cases = []
for s in range(1, 61):
    cases.append(('R%d' % s, 'seed=%d' % s))
# 특수: 조기반환 · 캐시 우회(len>8) · version 1 · judge 경계 · dir 고정 · 손상 없음(dps 0)
cases += [
 ('E0a', 'seed=7 na=0 ne=3'), ('E0b', 'seed=7 na=3 ne=0'),
 ('B9a', 'seed=11 na=9 ne=3'), ('B9b', 'seed=12 na=3 ne=9'), ('B9c', 'seed=13 na=9 ne=9'),
 ('V1', 'seed=21 version=1'), ('V1b', 'seed=22 version=1 na=9'),
 ('J999', 'seed=31 judge=999'), ('J1000', 'seed=31 judge=1000'), ('J0', 'seed=32 judge=0'),
 ('D-1', 'seed=41 dir=-1'), ('D0', 'seed=41 dir=0'), ('D1', 'seed=41 dir=1'),
 ('ND', 'seed=51 nodmg=1'), ('NDT', 'seed=52 nodmg=1 tower=1'), ('T0', 'seed=53 tower=0'), ('T1', 'seed=53 tower=1 tdmg=5000'),
 ('FAR', 'seed=61 spread=400000'), ('FAR2', 'seed=62 spread=400000 tower=1'),
 ('ME3', 'seed=71 me=3 na=2'), ('ME4', 'seed=72 me=4 na=1 ne=5'),
 ('V1c', 'seed=25 version=1 tick=59'), ('V1d', 'seed=26 version=1 tick=120 judge=500'), ('V0', 'seed=27 version=0 tick=7777 judge=100'), ('V1e', 'seed=28 version=1 tick=0 judge=10'),
]
out_lines = []
n_match = n_diff = n_panic = 0
for cid, argv in cases:
    p = subprocess.run([EXE] + argv.split(), capture_output=True, timeout=120)
    out = p.stdout.decode('utf-8', 'replace') + p.stderr.decode('utf-8', 'replace')
    res = [l for l in out.split('\n') if l.startswith('RESULT')]
    r = res[0] if res else 'RESULT\tNONE rc=%s' % p.returncode
    if 'MATCH' in r: n_match += 1
    elif 'PANIC' in r: n_panic += 1
    else: n_diff += 1
    out_lines.append('=== %s  %s\n%s' % (cid, argv, out))
    print(cid, argv, '→', r.replace('\t', ' '))
open(LOG, 'w', encoding='utf-8').write('\n'.join(out_lines))
print('SUMMARY match=%d diff=%d panic=%d total=%d' % (n_match, n_diff, n_panic, len(cases)))
