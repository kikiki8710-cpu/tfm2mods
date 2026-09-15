# -*- coding: utf-8 -*-
"""26차 I · o227 드라이버 — 케이스당 프로세스 1개(SIEGE_STANCE_CACHE TLS). 예측 ↔ 실행. → o227_cases.log"""
import subprocess, os, io, sys, re
sys.stdout.reconfigure(encoding='utf-8')
EXE = os.path.join(os.environ['TEMP'], 'tfm2_spanprobe', 'o227.exe')
LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'o227_cases.log')
CASES = [
 ('W1',  'pos=on tick=9999999', 'false', 'L620: tick == tower_attack_disable_tick → ult 거짓 → false(위치 무관)'),
 ('W1b', 'pos=on tick=9999998', 'true', 'L620: tick < disable → 판정 진행 → 타워 위 → true'),
 ('W2',  'pos=far', 'false', '먼 위치(1,1) · 조준 없음 → in_range 거짓 → false'),
 ('W3',  'pos=R', 'true', '경계: dist = R (dx²≤R²) → true'),
 ('W4',  'pos=R1', 'false', '경계: dist = R+1 → false'),
 ('W5',  'pos=R tbrange=7000', 'true', 'stat_buff_cached.range 7000 가산 → 새 R 경계 true'),
 ('W5b', 'pos=R1 tbrange=7000', 'false', '동상 R+1 false'),
 ('W6',  'pos=R growth=3000 tlvl=4', 'true', 'growth*(level-1)=9000 가산 → 경계 true'),
 ('W6b', 'pos=R1 growth=3000 tlvl=4', 'false', '동상 R+1 false'),
 ('W7',  'pos=R tmult=50 cmult=20', 'true', 'radius_mult: t 10000*150/100 · champ 10000*120/100 가산 → 경계 true'),
 ('W7b', 'pos=R1 tmult=50 cmult=20', 'false', '동상 R+1 false'),
 ('W8',  'pos=on teff=0', 'false', '타워 attack_effect None → continue → false'),
 ('W9',  'pos=far ne=me', 'true', 'nearest_enemy = 나 → L658 in_range 무관 true'),
 ('W10', 'pos=far ne=a1', 'false', 'nearest_enemy = 아군 a1 · 스탠스 None → L661 미니언 0 < 2 && in_range(false) → false'),
 ('W11', 'pos=on ne=a1', 'true', 'nearest_enemy = a1 · in_range → cnt 0<2 && in_range → true'),
 ('W12', 'pos=on ne=gone', 'true', 'nearest_enemy = 존재하지 않는 id · 스탠스 None → L661 경로 → in_range → true'),
 ('W13', 'pos=far ne=gone', 'false', '동상 far → false'),
 ('W14', 'pos=on tower=mid', 'true', 'mid 타워에서도 동일(라인 무관)'),
 ('W15', 'pos=Rm', 'true', 'R-1 → true'),
 ('W16', 'pos=on ver=1', 'true', 'version 은 v47_siege_stance 로만 전달 — 본문 분기 없음 → true'),
]
def run(argv):
    p = subprocess.run([EXE] + argv.split(), capture_output=True, timeout=120)
    o = p.stdout.decode('utf-8', 'replace') + p.stderr.decode('utf-8', 'replace')
    res = [l for l in o.split('\n') if l.startswith('RESULT')]
    inf = [l for l in o.split('\n') if l.startswith('tower\t')]
    return (res[0] if res else 'EXIT rc=%d %s' % (p.returncode, o[-200:].replace('\n', ' | '))), (inf[0] if inf else '')
out = []; ok_n = 0
for cid, argv, pred, desc in CASES:
    r, inf = run(argv)
    m = re.search(r'val=(\w+)', r)
    got = m.group(1) if m else r
    verdict = 'OK' if got == pred else 'MISMATCH'
    if verdict == 'OK': ok_n += 1
    out.append('%s\t%s\tpred=%s got=%s\t%s\t%s\n   %s' % (cid, verdict, pred, got, argv, desc, inf))
out.append('TOTAL %d/%d OK' % (ok_n, len(CASES)))
txt = '\n'.join(out)
io.open(LOG, 'w', encoding='utf-8').write(txt)
print(txt)
