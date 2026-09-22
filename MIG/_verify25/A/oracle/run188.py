# -*- coding: utf-8 -*-
"""25차 A · o188 드라이버 — 케이스당 프로세스 1개(TLS 메모 함정 ③). 예측(명세 logic 독해) ↔ 실행 대조.
사용: python -X utf8 run188.py  → o188_cases.log
예측 표기: 'PANIC' / 원소 태그 이름 열 + self_diff 기대(없으면 빈 문자열) — 접두 일치가 아니라 전체 일치."""
import subprocess, os, io, sys, re
sys.stdout.reconfigure(encoding='utf-8')
EXE = os.path.join(os.environ['TEMP'], 'tfm2_spanprobe', 'o188.exe')
LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'o188_cases.log')
# (id, argv, 예측 tags, 예측 self_diff 부분문자열(None=무관), 설명)
CASES = [
 ('C1',  'goal=7', 'PANIC', '', 'End → bp:351 todo!() 패닉'),
 ('C2',  'goal=6 focus=e0', 'AroundHide', '', 'AssassinReady → bp:146 AroundHide 1개 · L537 base_battle 생략 · L560 조기 반환'),
 ('C2b', 'goal=6 focus=e0 near=60000 vis=1 mhp=1000', 'AroundHide', '', 'AssassinReady + 근접 가시 적 → L560 반환이라 치명 판정(L566~) 없음'),
 ('C3',  'goal=4', 'RunAway RunAway', '', 'RunAway 스폰(분수 근접<300000) → bp:287 + bp:292(to_heal_area 재사용) 2개'),
 ('C4',  'goal=4 mx=480000 my=480000', 'RunAway', '', 'RunAway 맵 중앙(분수·타워 모두 멀다) → bp:292(d_t>=150000²) 1개'),
 ('C5',  'goal=4 mx=48000 my=352000', 'RunAway', '', 'RunAway 내 타워(48000,272000)에서 80000(<150000, >50000) · 분수 멀다 → bp:300 1개, KitingBack 없음'),
 ('C6',  'goal=4 mx=48000 my=302000 near=20000 vis=1 mhp=1000', 'RunAway', '', 'RunAway 내 타워 30000 안 + 적 근접 가시 → bp:301 진입 · 타워 사거리 0+radius 안 적? 없으면 RunAway 1개(관측)'),
 ('C7',  'goal=0 focus=e0', 'Trace', '', 'Trace 원거리 → bp:140 Trace(avoid=0, range_only=0)'),
 ('C8',  'goal=0 focus=e0 tactic=2', 'Trace', '', 'Trace + BacklineDPS → bp:136 Trace(range_only=1)'),
 ('C9',  'goal=0 focus=e0 avoid=1', 'Trace', '', 'Trace + avoid → Trace(avoid=1)'),
 ('C10', 'goal=0 focus=e0 near=50000 vis=1 mhp=1000 far=1,2,3,4', 'Attack', '', 'Trace 사거리 안 가시 → base_battle Attack(e0) 있고 focus 가까움 → other 없음 · L539 Attack · L564 반환'),
 ('C11a','goal=0 focus=e0 near=150000 vis=1 mhp=1000 ehp=1000 eatk=0 far=1,2,3,4 kdt=1', 'Trace', '', 'Trace 원거리 가시 적 1 · 적 무피해 → die_tick=60000(불사 센티널)≠0 → all_lethal=false → Trace 유지 (첫 예측 RunAway 는 die_tick 값 오예측 — 1차 실행으로 정정)'),
 ('C11d','goal=0 focus=e0 near=150000 vis=1 mhp=0 ehp=1000 eatk=100000 far=1,2,3,4 kdt=1', 'RunAway', '+0x2d:0->3', '내 hp=0 → die_tick==0 → 전원 치명 → L604 bail=3 · L605 truncate · L606 RunAway 1개'),
 ('C10b','goal=0 focus=e0 near=50000 vis=1 evis=1 mhp=1000 far=1,2,3,4', None, '', 'C10 + game.is_visible(visible_state) → base_battle_action Attack(e0)? 관측(콜리 계약)'),
 ('C11b','goal=0 focus=e0 near=150000 vis=1 mhp=100 ehp=1000 eatk=500 far=1,2,3,4 kdt=1', 'Trace', '', 'Trace 원거리 가시 적 1 · 적 강타 → die_tick!=0 → all_lethal=false → Trace 유지 · bail 0'),
 ('C11c','goal=0 focus=e0 near=150000 vis=0 mhp=1000 ehp=1000 eatk=0 far=1,2,3,4', 'Trace', '', '비가시 → L568 near_enemies 빈 → L572 반환(치명 판정 없음)'),
 ('C12', 'goal=1 focus=a1', 'Trace', '', 'Protect 아군 focus · 가시 적 없음 → real_focus=a1 → bp:155 Trace(a1, avoid=0)'),
 ('C12b','goal=1 focus=a1 avoid=1', 'Trace', '', 'Protect + avoid → bp:153 Trace(a1, avoid=1)'),
 ('C13a','goal=2 focus=e0', 'RunAway', '', 'Kiting · base_battle 빈 · 근접가시 적 없음 · chase=false(비가시) → bp:227 RunAway'),
 ('C13b','goal=2 focus=e0 vis=1', 'Trace', '', 'Kiting · 원거리(>200000) 가시 → nearest None · chase=true → bp:222 Trace(e0)'),
 ('C13c','goal=2 focus=e0 near=150000 vis=1 mhp=1000 far=1,2,3,4 eatk=0 ehp=1000', 'Trace', '', 'Kiting · 150000 가시(사거리 밖) → bp:207 Trace(e0) (그 뒤 치명 판정: eatk=0 → die 0 → RunAway 로 대체될 수 있음 — 관측)'),
 ('C13d','goal=2 focus=e0 near=60000 vis=1 evis=1 mhp=1000 ehp=1000 eatk=0 far=1,2,3,4', 'RunAway AroundPosition', '', 'Kiting · 사거리 안 + game.is_visible → bp:190 RunAway + bp:198 AroundPosition(radius 24000 · outline 0) · rnd 소비(new_with_radius→wait_around)'),
 ('C13e','goal=2 focus=e0 near=60000 vis=1 evis=0 mhp=1000 ehp=1000 eatk=0 far=1,2,3,4', 'Trace', '', 'Kiting · 사거리 안이나 game.is_visible 거짓(블랙보드 recent 만) → bp:207 Trace — bp:189 의 is_visible 은 vtable 실가시'),
 ('C14', 'goal=3 focus=e0', 'RunAway', '', 'KitingBack · base_battle 빈 → bp:241 RunAway'),
 ('C15', 'goal=0 focus=e0 sup=e1', None, '', 'support_target=적 e1(원거리) → bp:110 self.support_target=Some(e1) (값 동일) · Trace(e1)? + Trace(e0) — v21 판정 관측'),
 ('C16a','goal=0 focus=e0 sup=a1', 'Trace', '', 'support_target=아군 a1 · 가시 적 없음 → bp:109 블록 skip → self 무변경'),
 ('C16b','goal=0 focus=e0 sup=a1 near=150000 vis=1 mhp=1000 eatk=500 ehp=1000', None, '+0x8:', 'support_target=아군 a1 + 가시 적 → bp:110 self.support_target ← 가장 가까운 가시 적 id (self_diff +0x8)'),
 ('C17', 'goal=0 focus=e0 hold=999999', 'Trace', '', 'v48_claim_hold_until>tick → L399 경로, 궤적/캐스트라인 없음 → force_runaway=false → 정상'),
 ('C18', 'goal=0 focus=e0 near=60000 vis=1 mhp=1000 cast=0 elapsed=3000 far=1,2,3,4', None, None, 'force_runaway 시도: 적 e0 Skill(Position) 캐스팅 중 · dodge_risk 는 zeroed PositioningScoreData → 관측'),
 ('C19', 'goal=0 focus=e0 mx=322000 my=48000 mhp=1000', None, '', '적 top 타워(272000,48000) 50000 거리 · atk range 100000 → L549 in_my_reach → v3_tower_burst_feasible → Attack(tower)? 관측'),
 ('C20', 'goal=4 near=60000 vis=1 mhp=1000 eatk=0 ehp=1000', 'RunAway RunAway', '', 'RunAway + 근접 가시 적 → L544/L560 switch 로 타워·치명 판정 생략 → bail 0 유지'),
 ('C21', 'goal=4 mx=48000 my=302000 version=1', None, None, 'version<2: bp:293 else 가지(bp:320~345 AroundRunAway) 살아있는지 관측'),
 ('C22', 'goal=5 focus=e0', 'Trace', '', 'Assassin(5) = Trace 와 같은 bp:123 가지 → Trace(e0)'),
 ('C23', 'goal=0 focus=none', 'Trace', '', 'focus 엔티티 없음(999999) → bp:75 real_focus_id=focus 그대로 · Trace(target=999999)'),
 ('C24', 'goal=0 focus=e0 atk=0', None, None, '내 attack_effect None: Trace 경로는 unwrap 안 함(bp:170 은 Kiting 전용) → Trace 예상 · L549 is_some_and false'),
 ('C25', 'goal=2 focus=e0 atk=0 near=150000 vis=1 mhp=1000 far=1,2,3,4', 'PANIC', None, 'Kiting + attack_effect None + 근접 가시 적 → bp:170 unwrap 패닉(unwrap_failed)'),
]
def run(argv):
    p = subprocess.run([EXE] + argv.split(), capture_output=True, timeout=120)
    out = p.stdout.decode('utf-8', 'replace') + p.stderr.decode('utf-8', 'replace')
    return out
def parse(out):
    tags = None; diff = ''; panic = None; extra = {}
    for l in out.split('\n'):
        if l.startswith('RESULT\tPANIC'):
            panic = l.split('\t', 2)[2]
        elif l.startswith('RESULT\t'):
            body = l.split('\t')[-1]
            tags = ' '.join(re.findall(r'([A-Za-z]+)\(\d+\)', body))
            extra['detail'] = body
            extra['rnd'] = re.search(r'rnd_used=(\w+)', l).group(1)
        elif l.startswith('self_diff\t'):
            diff = l.split('\t', 1)[1].strip()
        elif l.startswith('self_after\t'):
            extra['after'] = l.split('\t', 1)[1]
        elif l.startswith('kdt\t'):
            extra['kdt'] = l.split('\t', 1)[1]
        elif l.startswith('world\t'):
            extra['world'] = l.split('\t', 1)[1]
    return tags, diff, panic, extra
lines = []
n_match = n_mis = n_obs = 0
for cid, argv, want, wdiff, desc in CASES:
    out = run(argv)
    tags, diff, panic, ex = parse(out)
    got = 'PANIC' if panic is not None else (tags if tags is not None else 'NORESULT')
    if want is None:
        verdict = 'OBS'; n_obs += 1
    else:
        ok_tags = (got == want)
        ok_diff = True if wdiff is None else ((wdiff in diff) if wdiff else (diff == ''))
        verdict = 'MATCH' if (ok_tags and ok_diff) else 'MISMATCH'
        if verdict == 'MATCH': n_match += 1
        else: n_mis += 1
    lines.append('%s\t%s\t%s\n  argv : %s\n  want : %s | diff %r\n  got  : %s | diff %r | rnd_used=%s%s\n  detail: %s\n  self_after: %s\n  %s' % (
        cid, verdict, desc, argv, want, wdiff, got, diff, ex.get('rnd'), (' | kdt ' + ex['kdt']) if 'kdt' in ex else '',
        (panic if panic is not None else ex.get('detail', '')), ex.get('after', ''), ex.get('world', '')))
    print(lines[-1].split('\n')[0])
hdr = 'o188 truth table · MATCH %d · MISMATCH %d · OBS %d · total %d\n' % (n_match, n_mis, n_obs, len(CASES))
io.open(LOG, 'w', encoding='utf-8').write(hdr + '\n'.join(lines) + '\n')
print(hdr)
