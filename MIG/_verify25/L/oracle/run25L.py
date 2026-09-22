"""run25L.py — o25L.exe 케이스 드라이버(케이스당 프로세스 1개 · TLS 메모 콜리). 결과는 oracle/<name>.log 에 저장, 요약표를 stdout 에."""
import subprocess, os, sys, io
EXE = r'C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o25L.exe'
OUT = os.path.dirname(os.path.abspath(__file__))
# (이름, [인자])  — 192: pteam=0 ppos=1 (블루 정글러) 기본 · at=<jt> side=<0|1> dx dy 로 좌표
CASES = [
    # ── 192 SerpenCheck ──
    ('s_base_mc0',        ['192', 'mc=0']),                                    # 아군 본진 · !enemy → Rhino 거리 far → mc 0 유지 → Rhino 경유
    ('s_base_mc1',        ['192', 'mc=1']),                                    # mc=1 → 바로 Serpen
    ('s_rhino_own_eq',    ['192', 'mc=0', 'at=0', 'side=1', 'dx=70000']),      # 아군 Rhino dx=70000 → dsq=4.9e9 <4900000001 → mc=1
    ('s_rhino_own_gt',    ['192', 'mc=0', 'at=0', 'side=1', 'dx=70000', 'dy=1']),  # dsq=4900000001 → false → mc 0
    ('s_rhino_enemy',     ['192', 'mc=0', 'at=0', 'side=0']),                  # 적 Rhino(적 진영) → 즉시 mc=1 (Rhino 거리 안 봄)
    ('s_enemy_far',       ['192', 'mc=0', 'x=900000', 'y=100000']),            # 적 진영 깊숙 → mc=1 → Serpen far → Serpen
    ('s_serpen_eq',       ['192', 'mc=0', 'at=5', 'side=1', 'dx=150000']),     # Serpen dsq=2.25e10 → not ugt → Serpen 직행(mc 무관)
    ('s_serpen_gt',       ['192', 'mc=0', 'at=5', 'side=1', 'dx=150000', 'dy=1']),  # ugt → mc(아군측 x=822000>y → 적 진영? → mc=1) → Serpen
    ('s_serpen_gt_blue',  ['192', 'mc=0', 'at=5', 'side=1', 'dx=-150000', 'dy=1']), # (522000,672001) 아군측 · Rhino far → mc 0 → Rhino
    ('s_serpen_on',       ['192', 'mc=0', 'at=5', 'side=1']),                  # (672000,672000) 경계 x==y → blue → 아군 → Rhino far → mc 0 · Serpen 근접
    ('s_diag_plus1',      ['192', 'mc=0', 'x=672001', 'y=672000']),            # x>y → 적 진영 → mc=1
    ('s_t1_base',         ['192', 'mc=0', 'pteam=1']),                         # 팀1 본진(레드) → !enemy → 레드 Rhino far → mc 0 → 레드 Rhino
    ('s_t1_enemy',        ['192', 'mc=0', 'pteam=1', 'x=100000', 'y=900000']), # 팀1 이 블루 진영 → mc=1
    ('s_near_enemy_champ',['192', 'mc=1', 'x=900000', 'y=60000', 'ticks=120']),# 적 챔프 스폰 근처 → is_recent_visible 판정·is_visible → battle_action
    ('s_dbg',             ['192', 'mc=0', 'dbg=1']),                           # context.debug=1 (posture None 이면 infos 미기록)
    # ── 193 Jungle ──
    ('j_own_rhino',       ['193', 'steam=0', 'scamp=0', 'cm=0', 'ticks=120']),   # 자기 팀 캠프 → L22 직행 · check_move 미기록
    ('j_enemy_rhino_base',['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120']),   # 적 Rhino · 아군 본진 → Serpen 경유(out_line) · cm 0
    ('j_enemy_mush_base', ['193', 'steam=1', 'scamp=1', 'cm=0', 'ticks=120']),   # 적 Mushroom → Morgard 경유
    ('j_enemy_stump_base',['193', 'steam=1', 'scamp=2', 'cm=0', 'ticks=120']),   # 적 Stump → Morgard 경유
    ('j_enemy_bee_base',  ['193', 'steam=1', 'scamp=3', 'cm=0', 'ticks=120']),   # 적 Bee → Serpen 경유
    ('j_morg_eq',         ['193', 'steam=1', 'scamp=1', 'cm=0', 'ticks=120', 'at=4', 'side=1', 'dx=-100000']),  # (188000,288000) 아군측 · dsq=1e10 → not ugt → cm=1 → 캠프
    ('j_morg_gt',         ['193', 'steam=1', 'scamp=1', 'cm=0', 'ticks=120', 'at=4', 'side=1', 'dx=-100000', 'dy=1']),  # 1e10+1 → 경유 out_line
    ('j_serp_eq',         ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'at=5', 'side=1', 'dx=-100000']),  # (572000,672000) 아군측
    ('j_serp_gt',         ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'at=5', 'side=1', 'dx=-100000', 'dy=1']),
    ('j_enemy_side',      ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'x=900000', 'y=100000']),  # 적 진영 → cm=1 즉시 → 캠프
    ('j_cm1',             ['193', 'steam=1', 'scamp=0', 'cm=1', 'ticks=120']),   # cm=1 → 직행
    ('j_at_camp',         ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'at=0', 'side=0', 'dx=20000']),  # 적 Rhino 옆 → RunAway(캠프근접) + Attack/Skill 후보 + cm=1
    ('j_at_camp_lvl3',    ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'at=0', 'side=0', 'dx=20000', 'lvl=3']),  # level>2 → skill2 경로
    ('j_at_camp_eff',     ['193', 'steam=1', 'scamp=3', 'cm=0', 'ticks=120', 'at=3', 'side=0', 'dx=40000', 'eff=200000']),  # 사거리 주입 → Bee 3마리 Attack
    ('j_range_eq',        ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'at=0', 'side=0', 'dx=25030']),  # max=25030 (atk 5000? 실측) 경계
    ('j_own_at_camp',     ['193', 'steam=0', 'scamp=3', 'cm=0', 'ticks=120', 'at=3', 'side=1', 'dx=30000']),  # 자기 Bee 캠프 옆
    ('j_t1_enemy_mush',   ['193', 'steam=0', 'scamp=1', 'cm=0', 'ticks=120', 'pteam=1']),  # 팀1 이 블루 Mushroom 노림 → 레드 본진(아군) → Morgard 경유
    ('j_tick0_nomobs',    ['193', 'steam=1', 'scamp=0', 'cm=0']),                # 몹 없음(live_list 빈) → 공격 후보 0
    # ── 추가 경계 ──
    ('s_rv_eq',           ['192', 'mc=1', 'x=763000', 'y=15000']),                 # 적 챔프 id23(913000,15000) dsq=2.25e10 <22500000001 → RunAway
    ('s_rv_gt',           ['192', 'mc=1', 'x=762999', 'y=15000']),                 # 150001² → 없음
    ('s_vis_battle',      ['192', 'mc=1', 'ticks=120', 'x=700000', 'y=250000', 'post=2']),  # is_visible=true → battle_action 경로
    ('j_range_gt',        ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'at=0', 'side=0', 'dx=25031']),  # max²<dsq → Attack/Skill 없음
    ('j_camp150k_eq',     ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'at=0', 'side=0', 'dx=150000']),  # L142 dsq=2.25e10 → RunAway
    ('j_camp150k_gt',     ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'at=0', 'side=0', 'dx=150001']),  # → 없음(jungles 도 far)
    ('j_jungles_eq',      ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'x=635000', 'y=341000']),   # Bee 몹(785000,341000) dsq=2.25e10 → L143 RunAway (캠프 Rhino far)
    ('j_jungles_gt',      ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'x=634999', 'y=341000']),   # → 없음
    ('j_vis',             ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'x=700000', 'y=250000', 'post=2']),  # is_visible → RunAway(L142 c2)
    ('s_posture_try',     ['192', 'mc=1', 'ticks=120', 'obj=1', 'oph=3', 'owb=1', 'at=5', 'side=1', 'dx=-60000']),  # TeamPlan.objective=Serpen 주입 — posture 여전히 None(기록용)
    # ── 2차 추가(03:20~): 논타겟 윈드업 · unwrap 패닉 경로 · skill2 후보 · 팀1 적 캠프 ──
    ('s_nt_skill_pos',    ['192', 'mc=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=4', 'ntcast=1', 'ntstart=1000']),   # 적 챔프 Skill 윈드업(+0x78 = 윈드업 경과틱 ≥ react_ticks) · casting=Position · 근접 → L50 hn → RunAway 단일
    ('s_nt_skill_dir',    ['192', 'mc=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=4', 'ntcast=2', 'ntstart=1000']),   # casting=Direction
    ('s_nt_skill_tgt',    ['192', 'mc=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=4', 'ntcast=0', 'ntstart=1000']),   # casting=Targeting(0) → 논타겟 아님 → 일반 경로
    ('s_nt_skill_far',    ['192', 'mc=1', 'ticks=120', 'x=500000', 'y=500000', 'nt=4', 'ntcast=1', 'ntrange=50000', 'ntstart=1000']),  # 사거리 밖 → is_in_range false
    ('s_nt_skill2_lvl1',  ['192', 'mc=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=5', 'ntlvl=1', 'ntinj=0', 'ntstart=1000']),   # Skill2 상태 + level 1 → 정적 None unwrap → 패닉 기대(serpen_check.rs:39?)
    ('s_nt_skill2_lvl3',  ['192', 'mc=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=5', 'ntlvl=3', 'ntcast=1', 'ntstart=1000']),  # level 3 → skill2_effect 사용 → hn
    ('s_nt_ult_lvl4',     ['192', 'mc=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=6', 'ntlvl=4', 'ntinj=0', 'ntstart=1000']),   # Ult 상태 + level 4 → 정적 None unwrap → 패닉 기대
    ('s_nt_ult_lvl5',     ['192', 'mc=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=6', 'ntlvl=5', 'ntcast=2', 'ntstart=1000']),  # level 5 → ult_effect
    ('j_nt_skill_pos',    ['193', 'steam=1', 'scamp=0', 'cm=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=4', 'ntcast=1', 'ntstart=1000']),  # 193 L129 hn → RunAway 단일
    ('j_nt_skill_tgt',    ['193', 'steam=1', 'scamp=0', 'cm=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=4', 'ntcast=0', 'ntstart=1000']),  # Targeting → 일반 경로
    ('s_nt_react_lo',     ['192', 'mc=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=4', 'ntcast=1', 'ntstart=1']),   # 윈드업 경과 1틱 → react_ticks 미달 → hn false (콜리 계약 기록용)
    ('j_nt_skill2_lvl1',  ['193', 'steam=1', 'scamp=0', 'cm=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=5', 'ntlvl=1', 'ntinj=0', 'ntstart=1000']),  # 패닉 기대(jungle.rs:118?)
    ('j_nt_ult_lvl4',     ['193', 'steam=1', 'scamp=0', 'cm=1', 'ticks=120', 'x=880000', 'y=40000', 'nt=6', 'ntlvl=4', 'ntinj=0', 'ntstart=1000']),  # 패닉 기대(jungle.rs:120?)
    ('j_skill2_lvl3',     ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'at=0', 'side=0', 'dx=20000', 'lvl=3', 'eff2=30000']),  # skill2_effect 주입 + level 3 → Skill2 후보
    ('j_skill2_lvl2',     ['193', 'steam=1', 'scamp=0', 'cm=0', 'ticks=120', 'at=0', 'side=0', 'dx=20000', 'lvl=2', 'eff2=30000']),  # level 2 → skill2 안 봄
    ('j_t1_enemy_rhino',  ['193', 'steam=0', 'scamp=0', 'cm=0', 'ticks=120', 'pteam=1']),   # 팀1 이 블루 Rhino 노림 → 레드 본진(아군) → Serpen 경유
    ('j_t1_enemy_side',   ['193', 'steam=0', 'scamp=0', 'cm=0', 'ticks=120', 'pteam=1', 'x=100000', 'y=900000']),  # 팀1 이 블루 진영(적 진영) → cm=1 즉시
    ('j_t1_serp_eq',      ['193', 'steam=0', 'scamp=0', 'cm=0', 'ticks=120', 'pteam=1', 'at=5', 'side=0', 'dx=100000']),  # 레드측 (772000,672000) dsq=1e10 → not ugt → cm=1
    ('j_t1_serp_gt',      ['193', 'steam=0', 'scamp=0', 'cm=0', 'ticks=120', 'pteam=1', 'at=5', 'side=0', 'dx=100000', 'dy=-1']),  # 1e10+1 → out_line
    ('s_t1_rhino_eq',     ['192', 'mc=0', 'pteam=1', 'at=0', 'side=0', 'dx=-70000']),   # 팀1 아군(레드) Rhino dx=-70000 → 4.9e9 → mc=1
    ('s_t1_rhino_gt',     ['192', 'mc=0', 'pteam=1', 'at=0', 'side=0', 'dx=-70000', 'dy=1']),  # → mc 0
    ('s_others_near',     ['192', 'mc=1', 'ticks=600', 'x=700000', 'y=250000']),   # 600틱(미니언 스폰?) others 순회 경로 기록용
]
sel = sys.argv[1:]
rows = []
for name, args in CASES:
    if sel and not any(s in name for s in sel):
        continue
    p = subprocess.run([EXE] + args, capture_output=True, text=True, encoding='utf-8', errors='replace', timeout=120)
    out = p.stdout + p.stderr
    io.open(os.path.join(OUT, name + '.log'), 'w', encoding='utf-8').write(' '.join(args) + '\n' + out)
    res = [l for l in out.split('\n') if l.startswith('RESULT')]
    game = [l for l in out.split('\n') if l.startswith('game\t')]
    mine = [l for l in out.split('\n') if l.startswith('mine\t')]
    elems = [l for l in out.split('\n') if l.startswith('elem[')]
    panic = [l for l in out.split('\n') if 'panicked' in l]
    r = res[0].split('\t')[1] if res else ('PANIC' if panic else 'NORESULT')
    rows.append((name, r, game[0][5:] if game else '', mine[0][5:] if mine else '', panic[0] if panic else ''))
    print('%-22s %-9s rc=%d' % (name, r, p.returncode))
    for l in game + mine: print('   ', l[:230])
    for l in elems: print('   ', l[:230])
    for l in panic: print('   ', l[:200])
print('\nSUMMARY', sum(1 for r in rows if r[1] == 'MATCH'), '/', len(rows), 'MATCH')
