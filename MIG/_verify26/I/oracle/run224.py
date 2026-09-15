# -*- coding: utf-8 -*-
"""26차 I · o224 드라이버 — 케이스당 프로세스 2개(score/expect 분리 · TLS MAX_RANGE_CACHE). 값 대조 + 예측값(손계산). → o224_cases.log"""
import subprocess, os, io, sys, re
sys.stdout.reconfigure(encoding='utf-8')
EXE = os.path.join(os.environ['TEMP'], 'tfm2_spanprobe', 'o224.exe')
LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'o224_cases.log')
# (id, argv, 손계산 예측(None=expect 만 대조), 설명)  — 기본: cc=90 D=60000 erng=100000 champ (480000,480000) hp=1 · max_range_cached(e0,champ)=20000 관측
CASES = [
 ('G0',  'goal=0 D=40000', 0, '래퍼: goal≠RunAway(End) → 0 (v21_defensive_cc_score 미호출)'),
 ('G1',  'cc=none D=40000', 0, 'L550 cc_time None → 0'),
 ('G2',  'cc=0 D=40000', 0, 'L550 Some(0) → 0'),
 ('B1',  'cc=90 D=40000', 99, '기본 직접위협: clamp(30)+35(사거리 안)+22(direct)+0+0+0(hp% 100)+12(itr 50000) = 99'),
 ('B2',  'cc=3 D=40000', 79, 'cc/3=1 → clamp 하한 10 → 79'),
 ('B3',  'cc=300 D=40000', 114, 'cc/3=100 → clamp 상한 45 → 114'),
 ('B4',  'cc=135 D=40000', 114, 'cc/3=45 → 45 (경계) → 114'),
 ('B5',  'cc=138 D=40000', 114, 'cc/3=46 → 45 → 114'),
 ('G3',  'same=1 D=40000', 0, 'L554 같은 팀 → 0'),
 ('G4',  'tty=2 D=40000', 0, 'L555 ty≠Champion(13) → 0'),
 ('G5',  'imm=1 D=40000', 0, 'L556 cc_immune → 0'),
 ('G6',  'ccst=hard D=40000', 0, 'L557 Airborne(is_cc) → 0'),
 ('G6b', 'ccst=soft D=40000', 99, 'BlockAttack 은 is_cc 아님 → 99'),
 ('G6c', 'ccst=both D=40000', 0, 'BlockAttack+Stun → 0'),
 ('G7',  'vis=0 D=40000', 0, 'L558 비가시 → 0'),
 ('G8',  'ct=ally D=40000', 0, 'L564 CastingTarget::Ally.check(champ, 적) 거짓 → 0'),
 ('F1',  'D=130008 adm=100 hp=100 maxhp=200', 76, 'L570 경계: cast 120000+10000+8 = 130008 → 통과 · 30+16+0+24+0+6 = 76'),
 ('F2',  'D=130009 adm=100 hp=100 maxhp=200', 0, 'L570 경계+1 → 0'),
 ('F3',  'D=120000 adm=100 hp=100 maxhp=200', 95, 'L602 경계: dist==cast_range → +35 (30+35+24+6)'),
 ('F4',  'D=120001 adm=100 hp=100 maxhp=200', 76, 'L602 경계+1 → +16'),
 ('F5',  'D=130000 adm=100 hp=100 maxhp=200 erng=100000 egrowth=5000 clvl=3', 95, 'effect.range: growth*(level-1)=10000 → cast 130000 → +35'),
 ('F6',  'D=130001 adm=100 hp=100 maxhp=200 erng=100000 egrowth=5000 clvl=3', 76, '동상 +1 → +16'),
 ('F7',  'D=127000 adm=100 hp=100 maxhp=200 cbr=7000', 95, 'stat_buff_cached.range 7000 → cast 127000 → +35'),
 ('F8',  'D=127001 adm=100 hp=100 maxhp=200 cbr=7000', 76, '동상 +1 → +16'),
 ('F9',  'D=130016 adm=100 hp=100 maxhp=200 cms=2', 76, 'L570 여유 move_speed*8: cms=2 → 130016 통과'),
 ('F10', 'D=130017 adm=100 hp=100 maxhp=200 cms=2', 0, '동상 +1 → 0'),
 ('I1',  'D=40000 adm=35 hp=100 maxhp=100', 123, 'incoming*100=3500 ≥ hp*35 → +24 : 30+35+22+24+12'),
 ('I2',  'D=40000 adm=34 hp=100 maxhp=100', 109, '3400 < 3500, ≥1500 → +10'),
 ('I3',  'D=40000 adm=15 hp=100 maxhp=100', 109, '1500 ≥ 1500 → +10 (경계)'),
 ('I4',  'D=40000 adm=14 hp=100 maxhp=100', 99, '1400 < 1500 → 0'),
 ('I5',  'D=40000 rdm=20 rtw=30 hp=100 maxhp=100', 109, 'incoming = 0+20+0+30/2 = 35 → 3500 ≥ 3500 → +24? (sdiv 30/2=15 → 35 → +24 → 123) — 예측 재계산: 20+15=35 → +24 → 123'),
 ('I6',  'D=40000 rtw=29 hp=100 maxhp=100', 99, 'rtw/2 = 14 (sdiv) → 1400 < 1500 → 0'),
 ('N1',  'D=40000 ne=1 na=0 hp=100 maxhp=100', 107, 'ne: e0+1 = 2 > na 1 → +8'),
 ('N2',  'D=40000 ne=4 na=0 hp=100 maxhp=100', 123, 'ne=5 > na=1 → min(32,24)=24'),
 ('N3',  'D=40000 ne=3 na=1 hp=100 maxhp=100', 115, 'ne=4 > na=2 → 16'),
 ('N4',  'D=40000 ne=1 na=2 hp=100 maxhp=100', 99, 'ne=2 ≤ na=3 → 0'),
 ('H1',  'D=40000 hp=44 maxhp=100', 113, 'hp_ratio 44 < 45 → +14'),
 ('H2',  'D=40000 hp=45 maxhp=100', 105, 'hp_ratio 45 → <65 → +6'),
 ('H3',  'D=40000 hp=64 maxhp=100', 105, 'hp_ratio 64 → +6'),
 ('H4',  'D=40000 hp=65 maxhp=100', 99, 'hp_ratio 65 → 0'),
 ('T1',  'D=45018', 99, 'direct_threat 경계: catch = 20000+25000+1*18 = 45018 → direct'),
 ('T2',  'D=45019 adm=100 hp=100 maxhp=200', 106, '경계+1 → not direct(22 없음) · +12(itr 50000) : 30+35+0+24+0+6+12 = 107? → 손계산: 30+35+24+6+12 = 107'),
 ('T3',  'D=46800 tms=100', 99, 'tms=100 → catch 46800 → direct(경계)'),
 ('T4',  'D=46801 tms=100 adm=100 hp=100 maxhp=200', 107, '동상 +1 → not direct → 107'),
 ('R1',  'D=50000 adm=100 hp=100 maxhp=200', 107, 'itr 경계: max_range 20000+30000 = 50000 → +12'),
 ('R2',  'D=50001 adm=100 hp=100 maxhp=200', 95, 'itr 경계+1 → 0 : 30+35+24+6 = 95'),
 ('X1',  'D=40000 cc=300 adm=100 hp=40 maxhp=100 ne=4 na=0', 160, '45+35+22+24+24+14+12 = 176 → min 160'),
 ('X2',  'D=40000 cc=300 adm=100 hp=40 maxhp=100 ne=2 na=0', 160, '45+35+22+24+16+14+12 = 168 → 160'),
 ('X3',  'D=40000 cc=300 adm=100 hp=40 maxhp=100 ne=1 na=0', 160, '45+35+22+24+8+14+12 = 160 (딱 상한)'),
 ('X4',  'D=40000 cc=300 adm=100 hp=40 maxhp=100', 152, '45+35+22+24+0+14+12 = 152'),
 ('V1',  'D=40000 ver=1', 99, 'version 은 is_enemy_well_danger 에만 전달 → 동일'),
]
def run(argv):
    p = subprocess.run([EXE] + argv.split(), capture_output=True, timeout=120)
    o = p.stdout.decode('utf-8', 'replace') + p.stderr.decode('utf-8', 'replace')
    res = [l for l in o.split('\n') if l.startswith('RESULT')]
    return (res[0] if res else 'EXIT rc=%d %s' % (p.returncode, o[-200:].replace('\n', ' | ')))
def val(r):
    m = re.search(r'val=(-?\d+)', r); return int(m.group(1)) if m else None
out = []; ok_n = 0; ok_pred = 0
for cid, argv, pred, desc in CASES:
    rs = run(argv + ' mode=score'); re_ = run(argv + ' mode=expect')
    vs, ve = val(rs), val(re_)
    verdict = 'OK' if (vs is not None and vs == ve) else 'MISMATCH'
    pv = '' if pred is None else ('pred_ok' if vs == pred else 'PRED_DIFF(%s)' % pred)
    if verdict == 'OK': ok_n += 1
    if pred is not None and vs == pred: ok_pred += 1
    out.append('%s\t%s\t%s\tscore=%s expect=%s\t%s\t%s\n   %s' % (cid, verdict, pv, vs, ve, argv, desc, re_))
out.append('TOTAL score==expect %d/%d · 손계산 예측 일치 %d/%d' % (ok_n, len(CASES), ok_pred, sum(1 for c in CASES if c[2] is not None)))
txt = '\n'.join(out)
io.open(LOG, 'w', encoding='utf-8').write(txt)
print(txt)
