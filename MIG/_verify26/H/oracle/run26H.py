"""run26H.py — o26H.exe 케이스 드라이버(케이스당 프로세스 1개 · TLS 메모 콜리). 결과는 oracle/<name>.log, 요약표를 stdout 에."""
import subprocess, os, sys, io
EXE = r'C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o26H.exe'
OUT = os.path.dirname(os.path.abspath(__file__))
CASES = [
    # ── 219 calculate_score_parameter_value ── goal 0 Trace/1 Protect/2 Kiting/3 KitingBack/4 RunAway/5 Assassin/6 AssassinReady/7 End · tactic 0..6
    ('c_g0_f0',        ['fn=219', 'goal=0', 'fi=0', 'ne=3', 'na=2']),            # Trace focus=enemy[0] → 60/30/30 · self 30 · ally 30
    ('c_g0_f2',        ['fn=219', 'goal=0', 'fi=2', 'ne=3', 'na=2']),            # focus = enemy[2]
    ('c_g0_nof',       ['fn=219', 'goal=0', 'ne=3', 'na=2']),                    # focus 없음 → 전원 30
    ('c_g5',           ['fn=219', 'goal=5', 'fi=1', 'ne=3', 'na=2']),            # Assassin → 전원 60(focus 무관)
    ('c_g5_nof',       ['fn=219', 'goal=5', 'ne=3', 'na=2']),
    ('c_g1_f1',        ['fn=219', 'goal=1', 'fi=1', 'ne=3', 'na=2']),            # Protect → 80/50 · self 50 · ally 50
    ('c_g2_nof',       ['fn=219', 'goal=2', 'ne=3', 'na=2']),                    # Kiting
    ('c_g3_f0',        ['fn=219', 'goal=3', 'fi=0', 'ne=3', 'na=2']),            # KitingBack
    ('c_g6',           ['fn=219', 'goal=6', 'fi=0', 'ne=3', 'na=2']),            # AssassinReady → self 100 · 적/아군 10
    ('c_g4',           ['fn=219', 'goal=4', 'ne=3', 'na=2']),                    # RunAway · 타워 근접 없음 · csf?
    ('c_g4_st',        ['fn=219', 'goal=4', 'ne=3', 'na=2', 'st=100']),          # support_target Some(100) → v15 인자 변화
    ('c_g7',           ['fn=219', 'goal=7', 'ne=3', 'na=2']),                    # End
    ('c_g4_nt',        ['fn=219', 'goal=4', 'ne=3', 'na=2', 'nt=0', 'eeff=50000', 'post=2']),   # 내 챔프·적 챔프를 타워0 곁으로 · 가시성 갱신
    ('c_g4_nt_far',    ['fn=219', 'goal=4', 'ne=3', 'na=2', 'nt=0', 'mdx=50001', 'eeff=50000', 'post=2']),  # 타워↔나 50001 → 후보 제외
    ('c_g4_nt_eq',     ['fn=219', 'goal=4', 'ne=3', 'na=2', 'nt=0', 'mdx=50000', 'eeff=50000', 'post=2']),  # 50000² < 50000²+1 → 후보
    ('c_g4_nt_novis',  ['fn=219', 'goal=4', 'ne=3', 'na=2', 'nt=0', 'eeff=50000']),             # post 없음 → 가시성 미갱신
    ('c_g4_nt_out',    ['fn=219', 'goal=4', 'ne=3', 'na=2', 'nt=0', 'edx=20001', 'eeff=50000', 'post=2']),  # d2c=20001² > (10000+10000)² → 밖 → 10
    ('c_g4_nt_edy',    ['fn=219', 'goal=4', 'ne=3', 'na=2', 'nt=0', 'edx=12000', 'edy=16000', 'eeff=50000', 'post=2']),  # 20000 정확 → 안
    ('c_empty',        ['fn=219', 'goal=0', 'fi=0', 'ne=0', 'na=0']),            # 빈 Vec
    ('c_t1',           ['fn=219', 'goal=0', 'fi=0', 'ne=3', 'na=2', 'tactic=1']),  # Frontline 100/100/70/140
    ('c_t2',           ['fn=219', 'goal=1', 'fi=0', 'ne=3', 'na=2', 'tactic=2']),  # BacklineDPS 90/90/130/100
    ('c_t3',           ['fn=219', 'goal=1', 'fi=0', 'ne=3', 'na=2', 'tactic=3']),  # SkillBurst · can_skill=true → 120
    ('c_t3_down',      ['fn=219', 'goal=1', 'fi=0', 'ne=3', 'na=2', 'tactic=3', 'scd=100000']),  # skill 쿨 → usable 0 → 60
    ('c_t4',           ['fn=219', 'goal=6', 'fi=0', 'ne=3', 'na=2', 'tactic=4']),  # Peel 70/150/110/140
    ('c_t5',           ['fn=219', 'goal=4', 'ne=3', 'na=2', 'tactic=5']),          # AllIn 130/100/90/100
    ('c_t6',           ['fn=219', 'goal=7', 'ne=3', 'na=2', 'tactic=6']),          # Disengage 100/100/150/100
    ('c_t4_g0',        ['fn=219', 'goal=0', 'fi=1', 'ne=3', 'na=2', 'tactic=4']),  # 30*70/100=21 · 60*70/100=42 · util 30*150/100=45
    ('c_t1_pteam1',    ['fn=219', 'goal=0', 'fi=1', 'ne=2', 'na=1', 'tactic=1', 'pteam=1', 'ppos=3']),
    # ── 221 precompute_champion_powers ──
    ('p_base',         ['fn=221']),
    ('p_ticks60',      ['fn=221', 'ticks=60']),                                   # 캐시 값이 채워지는가
    ('p_ticks600',     ['fn=221', 'ticks=600']),
    ('p_lvl3_s2',      ['fn=221', 'lvl=3', 'seff2=30000', 'cc=100']),             # level>2 → skill2 슬롯 포함
    ('p_lvl2_s2',      ['fn=221', 'lvl=2', 'seff2=30000', 'cc=100']),             # level 2 → skill2 제외(&None)
    ('p_lvl5_ult',     ['fn=221', 'lvl=5', 'ueff=30000', 'cc=100']),              # level>4 → ult 슬롯
    ('p_lvl4_ult',     ['fn=221', 'lvl=4', 'ueff=30000', 'cc=100']),
    ('p_cc',           ['fn=221', 'seff=30000', 'cc=100']),                       # skill 에 cc → effect_cc_time Some?
    ('p_t1p3',         ['fn=221', 'pteam=1', 'ppos=3', 'team=1', 'pos=3']),
    ('p_tick100',      ['fn=221', 'tick=100']),
    # ── 222 nexus_last_stand_uncached ──
    ('n_base',         ['fn=222']),
    ('n_nexus',        ['fn=222', 'atnexus=20000', 'eeff=50000']),               # 적이 넥서스 사거리 안 → true
    ('n_nexus_far',    ['fn=222', 'atnexus=200000', 'eeff=50000']),              # 사거리 밖
    ('n_twin',         ['fn=222', 'attwin=0', 'twdx=20000', 'eeff=50000']),      # 쌍둥이 타워 사거리 안 → true
    ('n_twin1',        ['fn=222', 'attwin=1', 'twdx=20000', 'eeff=50000']),
    ('n_me',           ['fn=222', 'atme=15000', 'eeff=50000']),                  # 나에게 닿음 · 미니언 없음 → false
    ('n_me_minions',   ['fn=222', 'atme=15000', 'eeff=50000', 'ticks=600']),     # 미니언 세운 뒤(본진 타격 여부는 실측)
    # ('n_noeff', ['fn=222', 'atnexus=1000', 'enoeff=1']),  # ⛔세계 오염: 챔피언 attack_effect=None 이면 AbstractGameWithCache::new 가 simulation.rs:1603 에서 패닉(함수 밖) — L214 None 가지는 IR 로만
    ('n_t1',           ['fn=222', 'pteam=1', 'ppos=2', 'atnexus=20000', 'eeff=50000']),
]
only = sys.argv[1:]
rows = []
for name, a in CASES:
    if only and not any(name.startswith(o) for o in only): continue
    try:
        r = subprocess.run([EXE] + a, capture_output=True, timeout=300)
        out = r.stdout.decode('utf-8', 'replace') + r.stderr.decode('utf-8', 'replace')
    except subprocess.TimeoutExpired:
        out = 'TIMEOUT'
    io.open(os.path.join(OUT, name + '.log'), 'w', encoding='utf-8').write(out)
    res = [l for l in out.splitlines() if l.startswith('RESULT')]
    game = [l for l in out.splitlines() if l.startswith('game')]
    diag = [l for l in out.splitlines() if l.startswith('diag')]
    panic = [l for l in out.splitlines() if 'panicked' in l]
    rows.append((name, res[0].split('\t')[-1] if res else ('PANIC ' + panic[0][:100] if panic else 'NORESULT'), game[0][5:140] if game else '', diag[0][5:160] if diag else ''))
for r in rows: print('%-16s %-9s %s | %s' % r)
