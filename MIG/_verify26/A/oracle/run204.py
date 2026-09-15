# -*- coding: utf-8 -*-
"""26차 A · o204 드라이버 — 케이스당 프로세스 1개(CHAMP_POWERS_MEMO TLS · 함정 ③). 예측(명세 logic 독해) ↔ 실행 대조.
사용: python -X utf8 run204.py → o204_cases.log
예측 = 람다(parsed dict) -> (ok:bool, 설명). parsed: 키 RESULT/PLAYER/ALLYi/ENEMYi/PANIC/expdmg 의 필드 dict."""
import subprocess, os, io, sys, re
sys.stdout.reconfigure(encoding='utf-8')
EXE = os.path.join(os.environ['TEMP'], 'tfm2_spanprobe', 'o204.exe')
LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'o204_cases.log')

def run(argv):
    p = subprocess.run([EXE] + argv.split(), capture_output=True, timeout=120)
    return p.stdout.decode('utf-8', 'replace') + p.stderr.decode('utf-8', 'replace')

def kv(s):
    d = {}
    for tok in re.findall(r'([\w.>-]+)=(\S+)', s): d[tok[0]] = tok[1]
    return d

def parse(out):
    r = {'raw': out}
    for l in out.split('\n'):
        if l.startswith('RESULT\tPANIC'): r['PANIC'] = l.split('\t', 2)[2]
        elif l.startswith('RESULT\t'): r['RESULT'] = kv(l)
        elif l.startswith('PLAYER\t'): r['PLAYER'] = kv(l); r['PLAYER_rp'] = re.findall(r'\{from=(\d+) tick=(\d+) value=(\d+)\}', l)
        elif l.startswith('ALLY') or l.startswith('ENEMY'):
            k = l.split('\t')[0]; r[k] = kv(l); r[k + '_rp'] = re.findall(r'\{from=(\d+) tick=(\d+) value=(\d+)\}', l)
        elif l.startswith('expdmg'): r['expdmg'] = kv(l)
        elif l.startswith('world'): r['world'] = kv(l)
    return r

def I(d, k, key): return int(d[k][key]) if k in d and key in d[k] else -999999
def dmg(r, key='e0->me'): return int(r['expdmg'][key]) if key in r.get('expdmg', {}) else -777777

CASES = [
 ('C1', '', lambda r: (I(r,'RESULT','na.len')==0 and I(r,'RESULT','ne.len')==0 and I(r,'RESULT','wave_tag')==0 and I(r,'RESULT','version')==55 and I(r,'RESULT','v3tb')==0 and I(r,'PLAYER','act')==0 and r['RESULT']['rnd_used']=='false' and r['RESULT']['debug_changed']=='false', 'na/ne 0 · wave None · version 55 · v3tb false · action RunAway · rnd/debug 미사용'), '기본 세계: 블랙보드 행동 None → with_action 목록 빔'),
 ('C1b', 'mx=960000 my=960000', lambda r: (I(r,'RESULT','cx')==29 and I(r,'RESULT','cy')==29, 'cx=cy=29 (960000/32000=30 → min 29)'), 'map_cell_index 포화 29'),
 ('C1c', 'mx=959999 my=32000', lambda r: (I(r,'RESULT','cx')==29 and I(r,'RESULT','cy')==1, 'cx=29 cy=1'), 'map_cell_index /32000'),
 ('C2', 'aact=0 adist=200000', lambda r: (I(r,'RESULT','na.len')==4, 'na.len=4 (dist 200000 ≤ 200000 포함 · 나 제외)'), 'near_allies 거리 경계 포함'),
 ('C3', 'aact=0 adist=200001', lambda r: (I(r,'RESULT','na.len')==0, 'na.len=0 (200001 제외)'), 'near_allies 거리 경계 제외'),
 ('C4', 'aact=0 adist=0', lambda r: (I(r,'RESULT','na.len')==4 and I(r,'ALLY0','team')==0 and I(r,'ALLY0','act')==0, 'na.len=4 (같은 좌표 · 자기 id 제외) · team 0 · act RunAway'), '자기 제외는 id 비교'),
 ('C5', 'eact=0 edist=200000 evis=1 lastd=120', lambda r: (I(r,'RESULT','ne.len')==5 and I(r,'ENEMY0','team')==1, 'ne.len=5 (last_visible+120 >= tick) · team 1'), 'is_recent_visible 경계 120 포함'),
 ('C6', 'eact=0 edist=200000 evis=1 lastd=121', lambda r: (I(r,'RESULT','ne.len')==0, 'ne.len=0 (121 틱 전 → 비가시)'), 'is_recent_visible 경계 121 제외'),
 ('C7', 'eact=0 edist=200000 egvis=1', lambda r: (True, '관측: game.is_visible(visible_state) 경로 ne.len=%s' % I(r,'RESULT','ne.len')), 'game.is_visible 경로(관측)'),
 ('C8', 'edist=1000 evis=1', lambda r: (I(r,'RESULT','ne.len')==0, 'ne.len=0 (블랙보드 행동 None → a.is_some() 실패)'), '적 팀 블랙보드 행동이 없으면 제외'),
 ('C8b', 'eact=0 edist=200001 evis=1', lambda r: (I(r,'RESULT','ne.len')==0, 'ne.len=0 (200001)'), 'near_enemies 거리 경계'),
 ('C9', 'eonly=0 eact=6 edist=50000 evis=1 eatk=100 erng=100000 ecd=0 estate=0', lambda r: (I(r,'PLAYER','rd')==dmg(r)>>1 and I(r,'PLAYER','ad')==0 and len(r['PLAYER_rp'])==1 and r['PLAYER_rp'][0][1]=='0' and int(r['PLAYER_rp'][0][2])==dmg(r) and dmg(r)>0, 'rd=dmg>>1 · ad=0 · rp=[{e0,0,dmg}]'), '적 Attack(나) · 지정형 · 미시전 → risk_damage(L1700) + risk_possible(L1845) · adjust /2'),
 ('C10', 'eonly=0 eact=6 edist=50000 evis=1 eatk=100 erng=100000 ecd=0 estate=3', lambda r: (I(r,'PLAYER','ad')==dmg(r) and I(r,'PLAYER','rd')==0 and len(r['PLAYER_rp'])==1, 'ad=dmg · rd=0 · rp 1'), '적 Attack(나) · 지정형 · 시전 중(action_state 3) → applyed_damage(L1702)'),
 ('C11', 'eonly=0 eact=6 edist=50000 evis=1 eatk=100 erng=100000 ecd=0 estate=3 ecast=1', lambda r: (I(r,'PLAYER','rd')==dmg(r)>>1 and I(r,'PLAYER','ad')==0, 'rd=dmg>>1 · ad=0 (논타겟은 항상 risk)'), '논타겟(Position) 시전 중이어도 risk(L1699 casting 먼저)'),
 ('C12', 'eonly=0 eact=6 edist=50000 evis=1 eatk=100 erng=100000 ecd=120 estate=0', lambda r: (len(r['PLAYER_rp'])==1 and r['PLAYER_rp'][0][1]=='120', 'rp=[{e0,120,dmg}]'), 'PossibleGain tick 경계 120 포함'),
 ('C13', 'eonly=0 eact=6 edist=50000 evis=1 eatk=100 erng=100000 ecd=121 estate=0', lambda r: (len(r['PLAYER_rp'])==0 and I(r,'PLAYER','rd')==dmg(r)>>1, 'rp 0 · rd 는 그대로'), 'PossibleGain tick 경계 121 제외(risk_damage 는 무관)'),
 ('C14a', 'eonly=0 eact=0 edist=120020 evis=1 eatk=100 erng=100000 ecd=0', lambda r: (len(r['PLAYER_rp'])==0 and I(r,'RESULT','ne.len')==1, 'RunAway 적 → 적 행동 루프 continue → rp 0'), 'RunAway 적은 위협 적재 자체를 안 함(L1687)'),
 ('C14b', 'eonly=0 eact=10 edist=120020 evis=1 eatk=100 erng=100000 ecd=0', lambda r: (len(r['PLAYER_rp'])==1, 'reach=100000+20+10000+10000=120020 ≥ dist → rp 1 (range_adjust 0 가정)'), '도달 경계(Stop 행동 · match _ arm) 포함'),
 ('C14c', 'eonly=0 eact=10 edist=120021 evis=1 eatk=100 erng=100000 ecd=0', lambda r: (len(r['PLAYER_rp'])==0, 'dist 120021 > reach → rp 0'), '도달 경계 제외'),
 ('C14d', 'eonly=0 eact=10 edist=120040 evis=1 eatk=100 erng=100000 ecd=0 espd=2', lambda r: (len(r['PLAYER_rp'])==1, 'spd 2 → +40 → reach 120040 → rp 1'), 'speed×20 계수 확인'),
 ('C14e', 'eonly=0 eact=10 edist=120041 evis=1 eatk=100 erng=100000 ecd=0 espd=2', lambda r: (len(r['PLAYER_rp'])==0, '120041 → rp 0'), 'speed×20 계수 경계'),
 ('C15', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=none', lambda r: (I(r,'PLAYER','rpt')==dmg(r,'twr->me') and I(r,'PLAYER','rd')==0 and dmg(r,'twr->me')>0, 'rpt=dmg(반감 없음) · rd 0'), '타워 nearest None → risk_possible_tower(L2161)'),
 ('C16', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=me', lambda r: (I(r,'PLAYER','rd')==dmg(r,'twr->me')>>1 and I(r,'PLAYER','rpt')==0, 'rd=dmg>>1 · rpt 0'), '타워 nearest == 나 → risk_damage(L2133)'),
 ('C17a', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=e0 eact=0', lambda r: (I(r,'PLAYER','rpt')==dmg(r,'twr->me') and I(r,'PLAYER','rd')==0, 'rpt=dmg (타깃 챔피언 e0 의 블랙보드 행동 RunAway)'), '타워 타깃=적 챔프 RunAway → L2147'),
 ('C17b', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=e0 eact=6', lambda r: (I(r,'PLAYER','rpt')==0 and I(r,'PLAYER','rd')==0, 'rpt 0 (타깃 챔프가 RunAway 아님 → 아무것도 안 함)'), '타워 타깃=적 챔프 Attack → 가산 없음'),
 ('C17c', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=e0', lambda r: (I(r,'PLAYER','rpt')==0, 'rpt 0 (블랙보드 None ≠ Some(RunAway))'), '타워 타깃=적 챔프 · 블랙보드 None'),
 ('C18', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=gone', lambda r: (I(r,'PLAYER','rpt')==dmg(r,'twr->me'), 'rpt=dmg (타깃 엔티티 없음 · 우리 미니언 0 < 2)'), '타워 타깃 소실 → L2157'),
 ('C19a', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=none tick=10000000', lambda r: (I(r,'PLAYER','rpt')==0, 'rpt 0 (tick 10000000 > 9999999 게이트)'), '타워 게이트 tick > disable_tick'),
 ('C19b', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=none tick=10801 tut=3', lambda r: (I(r,'PLAYER','rpt')==0, 'rpt 0 (Bottom(3) → 2v2 10800 · 10801 > 10800)'), '2v2 게이트 초과'),
 ('C19c', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=none tick=10800 tut=3', lambda r: (I(r,'PLAYER','rpt')==dmg(r,'twr->me'), 'rpt=dmg (10800 > 10800 거짓 → 타워 블록 실행)'), '2v2 게이트 경계 포함(`>`)'),
 ('C19d', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=none tick=14401 tut=5', lambda r: (I(r,'PLAYER','rpt')==0, 'rpt 0 (MidBottom → 3v3 14400)'), '3v3 게이트'),
 ('C19e', 'twr=1 tatk=100 trng=100000 tgap=5000 tnear=none tick=14400 tut=5', lambda r: (I(r,'PLAYER','rpt')==dmg(r,'twr->me'), 'rpt=dmg'), '3v3 게이트 경계'),
 ('C20', 'mact=6 atk=100 arng=100000 mcd=0 eonly=0 edist=50000 evis=1 eact=0', lambda r: (I(r,'PLAYER','act')==0 and len(r['ENEMY0_rp'])==0 and I(r,'ENEMY0','rd')==0 and I(r,'ENEMY0','ad')==0 and I(r,'RESULT','ne.len')==1, 'player.act=0 유지 · ENEMY0 rp 0 · rd/ad 0 (L2004~2100 사장)'), '★내 블랙보드 행동 Attack 이어도 parameter.player.action 은 RunAway 고정 → L2004~2100 사장'),
 ('C21', 'aact=6 aatk=100 arng=100000 acd=0 astate=0 adist=0 eonly=0 edist=50000 evis=1 eact=0', lambda r: (I(r,'RESULT','na.len')==4 and I(r,'ENEMY0','rd')==(4*dmg(r,'a1->e0'))>>1 and len(r['ENEMY0_rp'])==4 and I(r,'ENEMY0','ad')==0 and len(r['ALLY0_rp'])==0, 'ENEMY0 rd=(4·dmg)>>1 · rp 4 · ALLY rp 0'), '아군 4명 Attack(e0) 미시전 → e0.risk_damage ×4(L1897) + e0.risk_possible ×4(L1963)'),
 ('C21b', 'aact=6 aatk=100 arng=100000 acd=0 astate=3 adist=0 eonly=0 edist=50000 evis=1 eact=0', lambda r: (I(r,'ENEMY0','ad')==4*dmg(r,'a1->e0') and I(r,'ENEMY0','rd')==0, 'ENEMY0 ad=4·dmg · rd 0'), '아군 시전 중(action_state 3) → applyed(L1899)'),
 ('C22', 'eonly=0 eact=6 etgt=1 eatk=100 erng=100000 ecd=0 estate=0 edist=50000 evis=1 aact=0 adist=0', lambda r: (I(r,'PLAYER','rd')==0 and len(r['PLAYER_rp'])==1 and I(r,'ALLY0','rd')==dmg(r)>>1 and len(r['ALLY0_rp'])==1 and I(r,'ALLY1','rd')==0 and len(r['ALLY1_rp'])==1, 'PLAYER rd 0 rp 1 · ALLY0(=pos1 대상) rd=dmg>>1 rp 1 · ALLY1 rd 0 rp 1'), '적 Attack(아군 pos1) → 아군 target.risk_damage(L1709) · 전원 risk_possible(L1798)'),
 ('C23', 'version=1', lambda r: (I(r,'RESULT','version')==1, 'version=1 그대로 저장'), 'version 저장(분기 없음)'),
 ('C24', 'me=2', lambda r: (I(r,'PLAYER','pos')==2 and I(r,'PLAYER','team')==0, 'pos=2 team=0'), 'player.pos = position as usize'),
 ('C25a', 'eonly=0 eact=8 edist=50000 evis=1 elvl=1 eatk=100', lambda r: ('PANIC' in r, 'PANIC (level 1 → skill2_effect() None → unwrap)'), 'Skill2 행동 + 레벨≤2 → unwrap_failed'),
 ('C25b', 'eonly=0 eact=8 edist=50000 evis=1 elvl=3 eatk=100', lambda r: (True, '관측: level 3 skill2_effect 유무 → %s' % ('PANIC' if 'PANIC' in r else 'OK')), 'Skill2 행동 + 레벨 3(default 챔프 skill2_effect?)'),
 ('C26a', 'eonly=0 eact=0 edist=50000 evis=1 eatk=-2', lambda r: ('PANIC' not in r and I(r,'RESULT','ne.len')==1, 'OK (RunAway 는 unwrap 전 continue)'), '적 attack_effect None + RunAway → 패닉 없음'),
 ('C26b', 'eonly=0 eact=10 edist=50000 evis=1 eatk=-2', lambda r: ('PANIC' in r, 'PANIC (L1781 attack_effect unwrap)'), '적 attack_effect None + Stop → unwrap_failed'),
 ('C27', 'eonly=0 eact=6 edist=50000 evis=1 eatk=100 erng=100000 ecd=0 estate=0 seed=7 tick=4321', lambda r: (I(r,'PLAYER','rd')==dmg(r)>>1 and r['RESULT']['rnd_used']=='false', 'seed/tick 무관 · rnd 미소비'), 'rnd 미소비 재확인'),
 ('C28a', 'eonly=0 eact=9 edist=50000 evis=1 elvl=4 eatk=100', lambda r: ('PANIC' in r, 'PANIC (level 4 → ult_effect() None → unwrap L1756)'), 'Ult 행동 + 레벨≤4 → unwrap_failed'),
 ('C28b', 'eonly=0 eact=9 edist=50000 evis=1 elvl=5 eatk=100', lambda r: ('PANIC' not in r and len(r['PLAYER_rp'])==1, 'OK · rp 1(평타만 · 궁은 PossibleGain 제외)'), 'Ult 행동 + 레벨 5 → 궁 arm 통과 · 궁은 risk_possible 에 안 잡힘'),
 ('C29a', 'eonly=0 eact=7 edist=50000 evis=1 eatk=100 eskl=70 erng=100000 ecd=0 escd=0 estate=0', lambda r: (I(r,'PLAYER','rd')==dmg(r,'e0skl->me')>>1 and I(r,'PLAYER','ad')==0 and len(r['PLAYER_rp'])==2 and sorted(int(x[2]) for x in r['PLAYER_rp'])==sorted([dmg(r), dmg(r,'e0skl->me')]), 'rd=skl>>1 · ad 0 · rp 2(평타+스킬)'), '적 Skill(나) 미시전 → risk_damage(L1719) · rp 평타(L1845)+스킬(L1858)'),
 ('C29b', 'eonly=0 eact=7 edist=50000 evis=1 eatk=100 eskl=70 erng=100000 ecd=0 escd=0 estate=4', lambda r: (I(r,'PLAYER','ad')==dmg(r,'e0skl->me') and I(r,'PLAYER','rd')==0, 'ad=skl · rd 0'), '적 Skill 시전 중(action_state 4) → applyed(L1721)'),
 ('C29c', 'eonly=0 eact=7 edist=50000 evis=1 eatk=100 eskl=70 erng=100000 ecd=0 escd=121 estate=0', lambda r: (len(r['PLAYER_rp'])==1 and int(r['PLAYER_rp'][0][2])==dmg(r), 'rp 1(평타만 · skill_tick 121 제외)'), '스킬 쿨다운 121 → 스킬 PossibleGain 제외'),
 ('C29d', 'eonly=0 eact=7 edist=50000 evis=1 eatk=100 eskl=70 erng=100000 ecd=121 escd=120 estate=0', lambda r: (len(r['PLAYER_rp'])==1 and int(r['PLAYER_rp'][0][2])==dmg(r,'e0skl->me') and r['PLAYER_rp'][0][1]=='120', 'rp 1(스킬만 tick 120)'), '평타 121 제외·스킬 120 포함(각각 독립)'),
 ('C30', 'aact=0 askl=60 arng=100000 acd=0 adist=0 eonly=0 edist=50000 evis=1 eact=0', lambda r: (len(r['ENEMY0_rp'])==0 and I(r,'ENEMY0','rd')==0 and I(r,'RESULT','na.len')==4, 'ENEMY0 rp 0 (아군 RunAway → L1884 continue 가 L1944~1993 위협 적재까지 건너뜀)'), '아군 RunAway 면 위협 적재도 없음(L1884 continue 범위)'),
 ('C30b', 'aact=10 aatk=100 askl=60 arng=100000 acd=0 adist=0 eonly=0 edist=50000 evis=1 eact=0', lambda r: (len(r['ENEMY0_rp'])==8 and I(r,'ENEMY0','rd')==0, 'ENEMY0 rp 8 (아군 4 × 평타 L1963 + 스킬 L1976) · rd 0'), '아군 Stop(match _ arm) → 대상 가산 없음 · 위협 적재는 됨'),
 ('C31z', 'etwr=1 mtatk=100 trng=100000 tgap=5000 mtnear=none eonly=0 evis=1 eact=0 meoff=60000', lambda r: (I(r,'ENEMY0','rpt')==0 and I(r,'ENEMY0','rd')==0 and I(r,'RESULT','ne.len')==1, 'ENEMY0 0 (나-내타워 155000 > 150000 · 내 타워 대안조건은 near_allies 기준이라 e0 근접은 무효 → near_towers 에 내 타워 없음)'), '★near_towers 필터 비대칭: 내 팀 타워는 champ 150000 또는 아군 70000 만(적 근접은 무관)'),
 ('C31a', 'etwr=1 mtatk=100 trng=100000 tgap=5000 mtnear=none eonly=0 evis=1 eact=0', lambda r: (I(r,'ENEMY0','rpt')==dmg(r,'mtw->e0') and I(r,'ENEMY0','rd')==0 and dmg(r,'mtw->e0')>0, 'ENEMY0 rpt=dmg · rd 0'), '적 챔프 p 에 대한 내 타워 위협: nearest None → p.risk_possible_tower(L2206)'),
 ('C31b', 'etwr=1 mtatk=100 trng=100000 tgap=5000 mtnear=e0 eonly=0 evis=1 eact=0', lambda r: (I(r,'ENEMY0','rd')==dmg(r,'mtw->e0')>>1 and I(r,'ENEMY0','rpt')==0, 'ENEMY0 rd=dmg>>1'), '내 타워 nearest == p → p.risk_damage(L2187)'),
 ('C31c', 'etwr=1 mtatk=100 trng=100000 tgap=5000 mtnear=gone eonly=0 evis=1 eact=0', lambda r: (I(r,'ENEMY0','rd')==dmg(r,'mtw->e0')>>1 and I(r,'ENEMY0','rpt')==0, 'ENEMY0 rd=dmg>>1 (타깃 소실 · 적 미니언 0 < 3)'), '내 타워 타깃 소실 → cnt<3 → risk_damage(L2200)'),
 ('C31d', 'etwr=1 mtatk=100 trng=100000 tgap=5000 mtnear=twr eonly=0 evis=1 eact=0', lambda r: (I(r,'ENEMY0','rpt')==dmg(r,'mtw->e0') and I(r,'ENEMY0','rd')==0, 'ENEMY0 rpt=dmg (타깃 = 비챔피언 엔티티)'), '내 타워 타깃 비챔피언 → p.risk_possible_tower(L2190)'),
 ('C31e', 'etwr=1 mtatk=100 trng=100000 tgap=5000 mtnear=e1 eonly=0 evis=1 eact=0', lambda r: (I(r,'ENEMY0','rpt')==0 and I(r,'ENEMY0','rd')==0, 'ENEMY0 0 (타깃 다른 챔피언 e1 → 아무것도 안 함)'), '내 타워 타깃 = 다른 챔피언 → 가산 없음(L2189)'),
 ('C32z', 'atwr=1 tatk=100 trng=100000 tgap=5000 tnear=none aact=0 meoff=150000', lambda r: (all(I(r,'ALLY%d'%i,'rpt')==0 for i in range(4)) and I(r,'RESULT','na.len')==4, 'ALLY 0 (나-적타워 245000 · 적 타워 대안조건은 near_enemies 기준이라 아군 근접은 무효)'), '★near_towers 필터 비대칭: 적 타워는 champ 150000 또는 적 70000 만'),
 ('C32a', 'atwr=1 tatk=100 trng=100000 tgap=5000 tnear=none aact=0', lambda r: (all(I(r,'ALLY%d'%i,'rpt')==dmg(r,'twr->a1') for i in range(4)) and I(r,'ALLY0','rd')==0, 'ALLY rpt=dmg ×4'), '아군 p 에 대한 적 타워 위협: nearest None → L2253'),
 ('C32b', 'atwr=1 tatk=100 trng=100000 tgap=5000 tnear=a1 aact=0', lambda r: (I(r,'ALLY0','rd')==dmg(r,'twr->a1')>>1 and I(r,'ALLY0','rpt')==0 and I(r,'ALLY1','rpt')==0 and I(r,'ALLY1','rd')==0, 'ALLY0(pos1) rd=dmg>>1 · ALLY1 0(타깃 다른 챔피언)'), '적 타워 nearest == 아군 pos1 → L2234 · 다른 아군은 L2236 is_champion → 없음'),
 ('C32c', 'atwr=1 tatk=100 trng=100000 tgap=5000 tnear=gone aact=0', lambda r: (all(I(r,'ALLY%d'%i,'rd')==dmg(r,'twr->a1')>>1 for i in range(4)), 'ALLY rd=dmg>>1 ×4 (타깃 소실 · 내 팀 미니언 0 < 3)'), '적 타워 타깃 소실 → L2247'),
 ('C33a', 'etwr=1 mtatk=100 trng=100000 tgap=5000 mtnear=none eonly=0 evis=1 eact=0 tick=10000000', lambda r: (I(r,'ENEMY0','rpt')==0 and I(r,'ENEMY0','rd')==0 and I(r,'RESULT','ne.len')==1, 'ENEMY0 0 (2117 게이트가 2170~2206 도 스킵)'), '타워 게이트가 적 챔프 타워 블록(배치 C 꼬리·D)까지 덮음'),
 ('C33b', 'atwr=1 tatk=100 trng=100000 tgap=5000 tnear=none aact=0 tick=10000000', lambda r: (all(I(r,'ALLY%d'%i,'rpt')==0 for i in range(4)) and I(r,'RESULT','na.len')==4, 'ALLY 0 (2117 게이트가 2217~2253 도 스킵)'), '타워 게이트가 아군 타워 블록(배치 D)까지 덮음'),
]

def main():
    lines = []
    npass = nfail = 0
    for cid, argv, pred, desc in CASES:
        out = run(argv)
        r = parse(out)
        try:
            ok, msg = pred(r)
        except Exception as e:
            ok, msg = False, 'pred error %r' % e
        npass += ok; nfail += (not ok)
        lines.append('== %s [%s] %s\n   argv: %s\n   예측: %s\n   %s' % (cid, 'MATCH' if ok else 'MISMATCH', desc, argv, msg,
                     '\n   '.join(l for l in out.split('\n') if l.startswith(('RESULT', 'PLAYER', 'ALLY', 'ENEMY', 'expdmg', 'world')))))
    txt = '\n'.join(lines) + '\n\nMATCH %d / MISMATCH %d\n' % (npass, nfail)
    io.open(LOG, 'w', encoding='utf-8').write(txt)
    print(txt)

if __name__ == '__main__':
    main()
