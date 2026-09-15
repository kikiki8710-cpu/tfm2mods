# -*- coding: utf-8 -*-
"""26차 I · o223 드라이버 — 케이스당 프로세스 2개(mode=score / mode=expect 를 **따로** 띄운다: interaction_score 계열 TLS 메모가
한 프로세스 안에서 두 번째 호출을 재생하므로 같은 프로세스 비교는 무효 — 템플릿 함정 ③). 값 + rnd 320B 해시(호출 순서) 대조.
→ o223_cases.log"""
import subprocess, os, io, sys, re
sys.stdout.reconfigure(encoding='utf-8')
EXE = os.path.join(os.environ['TEMP'], 'tfm2_spanprobe', 'o223.exe')
LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'o223_cases.log')
CASES = [
 ('A1', 'act=attack tgt=e0', 'match', 'Attack 대상 존재 → base + calculate_action_score(Push) · bonus100=false'),
 ('A2', 'act=attack tgt=none', 'match', 'Attack 대상 없음(999999) → extra=-99999'),
 ('A3', 'act=attack tgt=e0 base=1', 'match', 'e0=본진(넥서스) 타격 미니언 → +100 (bonus100=true)'),
 ('A3b', 'act=attack tgt=e0 base=2', 'match', 'e0=쌍둥이타워 타격 미니언 → +100'),
 ('A4', 'act=attack tgt=e0 base=1 ver=1', 'match', 'version=1 → +100 게이트 닫힘(bonus100=false)'),
 ('A4b', 'act=attack tgt=e0 base=1 ver=2', 'match', 'version=2 → +100 (ugt 1 경계)'),
 ('A5', 'act=attack tgt=e0 aeff=0', 'PANIC', 'attack_effect None → unwrap 패닉(49518)'),
 ('S1', 'act=skill tgt=e0 seff=1', 'match', 'Skill → cas(skill, skill_effect, cooldown_reduce(false))'),
 ('S2', 'act=skill tgt=e0 seff=2', 'PANIC', 'skill_effect None(명시) → 패닉(49540)'),
 ('S3', 'act=skill tgt=none seff=1', 'match', 'Skill 대상 없음 → -99999'),
 ('T1', 'act=skill2 tgt=e0 s2eff=1 lvl=3', 'match', 'Skill2 level 3 → cas(skill2, skill2_effect)'),
 ('T2', 'act=skill2 tgt=e0 s2eff=1 lvl=2', 'PANIC', 'Skill2 level 2 → 정적 None unwrap 패닉(49558)'),
 ('T3', 'act=skill2 tgt=e0 s2eff=2 lvl=3', 'PANIC', 'Skill2 level 3 이지만 skill2_effect None(명시) → 패닉'),
 ('T4', 'act=skill2 tgt=none s2eff=1 lvl=1', 'match', 'Skill2 대상 없음 → -99999 (level 검사 전에 None 분기)'),
 ('U1', 'act=ult tgt=e0 ueff=1', 'match', 'Ult → 가산 0 (L329 매치 없음)'),
 ('U2', 'act=ult tgt=e0 base=1 ueff=1', 'match', 'Ult + 본진타격 미니언 → +100 만(L319 대상, L329 가산 0)'),
 ('U3', 'act=ult tgt=none ueff=1', 'match', 'Ult 대상 없음 → 가산 0(-99999 아님)'),
 ('R1', 'act=around tgt=e0 minion=1 front=e0,e1 near=1', 'match', 'Around · e0 적 미니언 · 넥서스 최근접 전방미니언=e0 → +5'),
 ('R2', 'act=around tgt=e1 minion=1 front=e0,e1 near=1', 'match', 'Around · e1 은 최근접 아님 → 0'),
 ('R3', 'act=around tgt=e0 minion=0 front=e0,e1 near=1', 'match', 'Around · e0 챔피언(미니언 아님) → 0'),
 ('R4', 'act=around tgt=e0 minion=1 front=none near=1', 'match', 'Around · 전방미니언 없음 → 0'),
 ('R5', 'act=around tgt=e0 minion=1 front=e1 near=1', 'match', 'Around · 전방미니언 e1 만 → nearest=e1≠e0 → 0'),
 ('R6', 'act=around tgt=e0 minion=1 front=e0 near=1 frontown=1', 'match', 'Around · 내 팀 블랙보드는 무관(bb[1-team] 만) → +5'),
 ('R7', 'act=aroundhide tgt=e0 minion=1 front=e0,e1 near=1', 'match', 'AroundHide 도 같은 가지 → +5'),
 ('R8', 'act=around tgt=none minion=1 front=e0 near=1', 'match', 'Around 대상 없음 → 0(-99999 아님)'),
 ('R9', 'act=around tgt=e0 minion=1 front=e1,e0 near=1', 'match', 'Around · 전방 순서 바꿔도 최근접=e0 → +5'),
 ('R10', 'act=around tgt=e0 minion=1 front=e0,e1 near=1 base=1', 'match', 'Around + e0 본진타격 → +100 없음(L319 공격류만) · +5'),
]
def run(argv):
    p = subprocess.run([EXE] + argv.split(), capture_output=True, timeout=120)
    o = p.stdout.decode('utf-8', 'replace') + p.stderr.decode('utf-8', 'replace')
    res = [l for l in o.split('\n') if l.startswith('RESULT')]
    return (res[0] if res else 'EXIT rc=%d %s' % (p.returncode, o[-200:].replace('\n', ' | '))), o
def kv(line):
    return dict(re.findall(r'(\w+)=([^\t]+)', line))
out = []
ok_n = 0
for cid, argv, pred, desc in CASES:
    rs, os_ = run(argv + ' mode=score')
    re_, oe = run(argv + ' mode=expect')
    act = [l for l in os_.split('\n') if l.startswith('action') or l.startswith('champ_effects')]
    if pred == 'PANIC':
        verdict = 'OK' if 'PANIC' in rs else 'MISMATCH'
    else:
        s, e = kv(rs), kv(re_)
        verdict = 'OK' if ('PANIC' not in rs and 'PANIC' not in re_ and s.get('val') == e.get('val') and s.get('rnd_changed') == e.get('rnd_changed')) else 'MISMATCH'
    if verdict == 'OK': ok_n += 1
    out.append('%s\t%s\t%s\t%s\n   %s\n   %s\n   %s' % (cid, verdict, argv, desc, ' | '.join(act), rs, re_))
out.append('TOTAL %d/%d OK' % (ok_n, len(CASES)))
txt = '\n'.join(out)
io.open(LOG, 'w', encoding='utf-8').write(txt)
print(txt)
