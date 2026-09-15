# -*- coding: utf-8 -*-
"""26차 F · o214 드라이버 — 케이스당 프로세스 1개(TLS 메모 함정 ③ · max_range_cached). 예측(명세 logic 독해) ↔ 실행 대조.
사용: python -X utf8 run214.py → o214_cases.log
예측 = 'PANIC' 또는 delta(score - interaction_score - calculate_action_score) 정수 / 'score=-99999' / None(관측만)."""
import subprocess, os, io, sys, re
sys.stdout.reconfigure(encoding='utf-8')
EXE = os.path.join(os.environ['TEMP'], 'tfm2_spanprobe', 'o214.exe')
LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'o214_cases.log')
# (id, argv, 예측, 설명)
CASES = [
 ('S1',  'act=stop', 0, 'Stop → _ arm → base 그대로(delta 0)'),
 ('R1',  'act=runaway', None, 'RunAway sup=none → base-(cp+dcp) · v15 false 기대 → -6 없음. cp/dcp(내부 헬퍼) 관측'),
 ('R2',  'act=runaway sup=a1', None, 'RunAway sup=아군 a1 → v15_can_keep_support_pressure 관측(-6 발생 여부)'),
 ('R2b', 'act=runaway sup=e0 near=60000 vis=1 evis=1 mhp=1000', None, 'RunAway sup=적 e0 근접 → v15 관측'),
 ('R3',  'act=recall', None, 'Recall 은 RunAway 와 같은 arm(논리 idx 1) → R1 과 동일 delta 기대'),
 ('T1',  'act=trace tgt=e0 sup=e0', 10, 'Trace 대상==support_target · far(dist²>mr²) → base+10'),
 ('T2',  'act=trace tgt=e0 sup=e0 e0x=65000 e0y=913000', 0, 'Trace 대상==support_target · 근접(50000<mr 165000) → base'),
 ('T3',  'act=trace tgt=e0 sup=none', 0, 'Trace sup 없음 · avoid 없음 → base'),
 ('T4',  'act=trace tgt=none sup=none', 0, 'Trace 대상 소실 → Trace arm 은 -99999 없음 → base'),
 ('T4b', 'act=trace tgt=none sup=none', 'score_not_-99999', 'T4 의 score 가 -99999 가 아님을 별도 확인'),
 ('T5',  'act=trace_avoid tgt=e0 sup=none avoid=1 mx=600000 my=600000 e0x=292000 e0y=48000 ctwt=1', None, 'Trace avoid 플래그 양쪽 ON · 대상 적 탑타워(272000,48000) 근방 · 나는 멀리 → 4조건 관측(-30 기대)'),
 ('T6',  'act=trace_avoid tgt=e0 sup=none avoid=0 mx=600000 my=600000 e0x=292000 e0y=48000', 0, 'self.avoid_unnecessary_tower_trace=false → 감점 경로 진입 안 함 → base'),
 ('T7',  'act=trace tgt=e0 sup=none avoid=1 mx=600000 my=600000 e0x=292000 e0y=48000', 0, 'action.avoid_unnecessary_tower=false(new) → 감점 경로 진입 안 함 → base'),
 ('A1',  'act=attack tgt=nexus', 200, 'Attack Nexus → +200'),
 ('A2',  'act=attack tgt=e0', 0, 'Attack Champion → ty 분기 없음 → cas+base (+v17 0 기대)'),
 ('A3',  'act=attack tgt=none', 'score=-99999', 'Attack 대상 소실 → -99999'),
 ('A3b', 'act=attack tgt=none atk=0', 'score=-99999', '대상 소실 판정(L622)이 attack_effect unwrap(L623)보다 먼저 → -99999(패닉 아님)'),
 ('A4',  'act=attack tgt=e0 atk=0', 'PANIC', '대상 있음 + attack_effect None → L623 unwrap 패닉'),
 ('A5',  'act=attack tgt=tower', None, 'Attack Tower version=55(>1) → v3_tower_burst_feasible 관측(+100 or 0)'),
 ('A5b', 'act=attack tgt=tower version=1', 0, 'Attack Tower version=1 → `version>1` 거짓 → 가산 없음'),
 ('A6',  'act=attack tgt=jungle jidx=6 run=600 thp=1000 tmax=1000', 'max1', '적 진영 정글(camp 1) · dmg_est 50 < hp 1000 → max(score,1)'),
 ('A6b', 'act=attack tgt=jungle jidx=6 run=600 thp=40 tmax=1000', 80, '적 진영 정글 · dmg_est 50 ≥ hp 40 → +80'),
 ('A6c', 'act=attack tgt=jungle jidx=6 run=600 thp=50 tmax=1000', 80, '경계: dmg_est 50 == hp 50 → `dmg < hp` 거짓 → +80'),
 ('A7',  'act=attack tgt=jungle jidx=0 run=600 thp=40 tmax=1000', 'max1', '내 진영 정글(camp 0) · 한 방이어도 max(score,1)'),
 ('A8',  'act=attack tgt=epic run=600 thp=1000 tmax=1000', 'div3', 'Epic · dmg 50 < hp 1000 → max(score/3,1)'),
 ('A8b', 'act=attack tgt=epic run=600 thp=40 tmax=1000', 100, 'Epic · dmg 50 ≥ hp 40 → +100'),
 ('A9',  'act=attack tgt=serpen run=600 thp=1000 tmax=1000', 'div3', 'Serpen · 처치 불가 → max(score/3,1)'),
 ('A9b', 'act=attack tgt=serpen run=600 thp=50 tmax=1000', 100, 'Serpen · 경계 dmg==hp → +100'),
 ('K1',  'act=skill tgt=e0', None, 'Skill Champion goal=Trace → v21+v17 관측(0 기대)'),
 ('K2',  'act=skill tgt=none', 'score=-99999', 'Skill 대상 소실 → -99999 (skill_effect Some)'),
 ('K3',  'act=skill tgt=none sk=none', 'PANIC', 'skill_effect None → L654 unwrap 이 L658 대상 검사보다 먼저 → 패닉'),
 ('K4',  'act=skill tgt=e0 sk=rush goal=4', 'score=-99999', '돌진 스킬 + goal RunAway → -99999'),
 ('K4b', 'act=skill tgt=e0 sk=rush goal=0', None, '돌진 스킬 + goal Trace → 정상 경로(관측)'),
 ('K4c', 'act=skill tgt=none sk=rush goal=4', 'score=-99999', '돌진+RunAway 배제(L654)가 대상 검사(L658)보다 먼저 — 어느 쪽이든 -99999 (구분 불가 · 관측)'),
 ('K5',  'act=skill tgt=e0 sk=stun stun=120 goal=4 e0x=45000 e0y=913000', None, 'Stun 스킬 · goal RunAway · dist 30000 → proximity (80000-30000)/10000=5 → +(cc*3+3)*5 + v21 + v17 (aux4 cc 로 계산)'),
 ('K5b', 'act=skill tgt=e0 sk=stun stun=120 goal=4 e0x=105000 e0y=913000', None, 'Stun · RunAway · dist 90000 ≥ 80000 → proximity 0 → v21+v17 만'),
 ('K5c', 'act=skill tgt=e0 sk=stun stun=120 goal=0 e0x=45000 e0y=913000', None, 'Stun · goal Trace → 근접 보너스 없음 → v21+v17 만'),
 ('K6',  'act=skill tgt=nexus', 200, 'Skill Nexus → +200'),
 ('K7',  'act=skill2 tgt=e0 lv=1', 'PANIC', 'Skill2 · level 1 ≤ 2 → skill2_effect() None → unwrap 패닉'),
 ('K7b', 'act=skill2 tgt=e0 lv=3', None, 'Skill2 · level 3 > 2 → 정상(관측)'),
 ('K7c', 'act=skill2 tgt=nexus lv=3', 200, 'Skill2 Nexus → +200'),
 ('U1',  'act=ult tgt=e0 lv=4', 'PANIC', 'Ult · level 4 ≤ 4 → ult_effect() None → unwrap 패닉'),
 ('U2',  'act=ult tgt=e0 lv=5 udm=100 ehp=50 emax=1000', 40, 'Ult Champion · dmg 100 ≥ hp 50 → +40'),
 ('U2b', 'act=ult tgt=e0 lv=5 udm=100 ehp=100 emax=1000', 40, '경계 dmg == hp → `dmg >= hp` 참 → +40'),
 ('U3',  'act=ult tgt=e0 lv=5 udm=100 ehp=710 emax=1000', -30, 'Ult Champion · 300<710 · ratio 71>70 · cc None · buff None → -30'),
 ('U4',  'act=ult tgt=e0 lv=5 udm=100 ehp=700 emax=1000', 0, 'ratio 70 → `>70` 거짓 → 0'),
 ('U5',  'act=ult tgt=e0 lv=5 udm=100 ehp=300 emax=400', 0, 'dmg*3 == hp 300 → `<` 거짓 → 0 (ratio 75)'),
 ('U5b', 'act=ult tgt=e0 lv=5 udm=100 ehp=301 emax=400', -30, 'dmg*3 300 < 301 · ratio 75 → -30'),
 ('U6',  'act=ult tgt=e0 lv=5 ul=stun ehp=710 emax=1000', None, 'Ult Stun(cc Some) · dmg 0 → +40 없음 · cc 있어 -30 없음 → v16/v21 관측'),
 ('U7',  'act=ult tgt=nexus lv=5', 200, 'Ult Nexus → +200'),
 ('U8',  'act=ult tgt=e0 lv=5 ul=rush goal=4', 'score=-99999', '돌진 궁 + RunAway → -99999'),
 ('U9',  'act=ult tgt=none lv=5', 'score=-99999', 'Ult 대상 소실 → -99999'),
 ('U10', 'act=ult tgt=jungle jidx=6 run=600 lv=5 thp=1000 tmax=1000', 'max1', 'Ult 적 정글 · attack_effect 기준 dmg_est 50 < 1000 → max(score,1)'),
 ('U10b','act=ult tgt=jungle jidx=6 run=600 lv=5 thp=40 tmax=1000', 80, 'Ult 적 정글 · dmg_est(attack_effect 50) ≥ 40 → +80 (궁 피해가 아니라 평타 이펙트 기준)'),
 ('U10c','act=ult tgt=jungle jidx=6 run=600 lv=5 thp=80 tmax=1000 udm=1000', 'max1', 'Ult 적 정글 · 궁 dmg 1000 이지만 판정은 attack_effect 50 < 80 → max(score,1) (궁 피해 무관 확인)'),
 ('U11', 'act=ult tgt=epic run=600 lv=5 thp=40 tmax=1000', 0, 'Ult Epic → Ult arm 에 Epic/Serpen 분기 없음(_ → score) → delta 0'),
 ('U12', 'act=ult tgt=tower lv=5', 0, 'Ult Tower → Ult arm 에 Tower 분기 없음 → 0'),
 ('K8',  'act=skill tgt=tower', 0, 'Skill Tower → Skill arm 에 Tower 분기 없음 → 0'),
 ('K9',  'act=skill tgt=epic run=600 thp=1000 tmax=1000', 'div3', 'Skill Epic · skill dmg 60 < 1000 → max(score/3,1)'),
 ('K9b', 'act=skill tgt=epic run=600 thp=60 tmax=1000', 100, 'Skill Epic · 경계 skill dmg 60 == hp → +100 (스킬 이펙트 기준)'),
 ('K10', 'act=skill tgt=jungle jidx=6 run=600 thp=55 tmax=1000', 'max1', 'Skill 적 정글 · 판정은 attack_effect 50 < 55 (스킬 60 아님) → max(score,1)'),
 ('K10b','act=skill tgt=jungle jidx=6 run=600 thp=50 tmax=1000', 80, 'Skill 적 정글 · attack_effect 50 ≥ 50 → +80'),
 ('A2b', 'act=attack tgt=e0 version=1', 0, 'version 1: Attack Champion 경로 무관'),
 ('T1b', 'act=trace tgt=e0 sup=e0 e0x=180000 e0y=913000', 0, '경계: dist 165000 == mr 165000 → dist²>mr² 거짓 → base'),
 ('T1c', 'act=trace tgt=e0 sup=e0 e0x=180001 e0y=913000', 10, '경계: dist 165001 > mr → +10'),
 ('T5b', 'act=trace_avoid tgt=e0 sup=e0 avoid=1 mx=600000 my=600000 e0x=292000 e0y=48000', -30, 'avoid 감점(L815 return)이 support +10(L819)보다 먼저 → -30 (둘 다 성립해도 -20 아님)'),
 ('K5d', 'act=skill tgt=e0 sk=stun stun=120 goal=4 e0x=85000 e0y=913000', 363, 'Stun · RunAway · dist 70000 → proximity 1 → (120*3+3)*1 = 363'),
 ('K5e', 'act=skill tgt=e0 sk=stun stun=120 goal=4 e0x=95000 e0y=913000', 0, 'dist 80000 → 80000.saturating_sub(80000)=0 → 0'),
 ('K5f', 'act=skill tgt=e0 sk=stun stun=120 goal=4 e0x=94999 e0y=913000', 0, 'dist 79999 → 1/10000 = 0 → 0'),
 ('K5g', 'act=skill tgt=e0 sk=stun stun=1 goal=4 e0x=45000 e0y=913000', 30, 'cc=1 → (1*3+3)*5 = 30'),
 ('U13', 'act=ult tgt=e0 lv=5 ul=stun stun=120 goal=4 e0x=45000 e0y=913000', None, 'Ult Stun · RunAway · dist 30000 → (cc*3+3)*5 = 1815 + v16 gambler 등 관측'),
]
def run(argv):
    p = subprocess.run([EXE] + argv.split(), capture_output=True, timeout=120)
    return p.stdout.decode('utf-8', 'replace') + p.stderr.decode('utf-8', 'replace')
def parse(out):
    r = {'panic': None, 'score': None, 'base': None, 'cas': None, 'delta': None, 'aux': ''}
    for l in out.split('\n'):
        if l.startswith('RESULT\tPANIC'):
            r['panic'] = l.split('\t', 2)[2]
        elif l.startswith('RESULT\t'):
            for kv in l.split('\t')[1:]:
                k, v = kv.split('=', 1)
                r[k] = v
            for k in ('score', 'base', 'cas', 'delta'):
                r[k] = int(r[k])
        elif l.startswith('aux'):
            r['aux'] += l.replace('\t', ' ') + ' || '
        elif l.startswith('world'):
            r['world'] = l.replace('\t', ' ')
    return r
def judge(pred, r):
    if pred is None: return 'OBS'
    if pred == 'PANIC': return 'MATCH' if r['panic'] else 'MISMATCH'
    if r['panic']: return 'MISMATCH(panic)'
    if pred == 'score=-99999': return 'MATCH' if r['score'] == -99999 else 'MISMATCH'
    if pred == 'score_not_-99999': return 'MATCH' if r['score'] != -99999 else 'MISMATCH'
    if pred == 'max1': return 'MATCH' if r['score'] == max(r['base'] + r['cas'], 1) else 'MISMATCH'
    if pred == 'div3':
        s = r['base'] + r['cas']
        q = int(s / 3) if s >= 0 else -int((-s) / 3)   # sdiv 절삭
        return 'MATCH' if r['score'] == max(q, 1) else 'MISMATCH'
    return 'MATCH' if r['delta'] == pred else 'MISMATCH'
if __name__ == '__main__':
    only = sys.argv[1:]
    with io.open(LOG, 'w', encoding='utf-8') as f:
        n = {'MATCH': 0, 'MISMATCH': 0, 'OBS': 0}
        for cid, argv, pred, desc in CASES:
            if only and cid not in only: continue
            out = run(argv)
            r = parse(out)
            j = judge(pred, r)
            n[j.split('(')[0]] = n.get(j.split('(')[0], 0) + 1
            line = '%s\t%s\tpred=%s\tscore=%s base=%s cas=%s delta=%s panic=%s\t%s\n   argv: %s\n   %s\n   %s' % (
                cid, j, pred, r['score'], r['base'], r['cas'], r['delta'], (r['panic'] or '')[:80], desc, argv, r.get('world', ''), r['aux'])
            print(line); f.write(line + '\n')
        print(n); f.write(str(n) + '\n')
