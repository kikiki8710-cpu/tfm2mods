"""o194 드라이버 — 케이스당 프로세스 1개. 결과 요약을 o194_cases.log 에 남긴다."""
import subprocess, os, io, sys
EXE = os.path.join(os.environ['LOCALAPPDATA'], 'Temp', 'tfm2_spanprobe', 'o194.exe')
CASES = [
 # (이름, 인자)  — 분기별 ≥2 케이스
 ("C01 L39 top→Morgard·L43 AP",            "bushi=0 cm=0"),
 ("C02 L39 top→Morgard·L43 AP (bush 2)",   "bushi=2 cm=0"),
 ("C03 L48 bottom→Serpen·L50 AP",          "bushi=6 cm=0"),
 ("C04 L48 bottom→Serpen·L50 AP (bush 8)", "bushi=8 cm=0"),
 ("C05 L45 at_camp→cm=1·esm 유지·L96 AB",  "bushi=0 cm=0 at_camp=1 esm=1"),
 ("C06 L52 at_camp(bottom)→cm=1·L96 AB",   "bushi=6 cm=0 at_camp=1 esm=1"),
 ("C07 L30 cm=1·!vis→esm=0·L96 AB",        "bushi=0 cm=1 esm=1"),
 ("C08 L30 cm=1·!vis→esm=0·L96 AB ol=0",   "bushi=5 cm=1 esm=1 ol=0"),
 ("C09 L96 AB ol=2",                       "bushi=5 cm=1 ol=2"),
 ("C10 L94 AB target (near+lv)",           "bushi=0 cm=1 near=1 lv=1000"),
 ("C11 L94 AB target (near, tick≤120 기본bb)", "bushi=0 cm=1 near=1 tick=100"),
 ("C12 L61 recent 아님 → L96",             "bushi=0 cm=1 near=1"),
 ("C13 L62 거리 밖(near=0, lv) → L96",     "bushi=0 cm=1 lv=1000"),
 ("C14 L91 vis·nearest None → RunAway",    "bushi=0 cm=1 vis=1"),
 ("C15 L91 RunAway esm→1 (esm0=0)",        "bushi=6 cm=1 vis=1 esm=0"),
 ("C16 L67 Some·!dominated·can1v1win=false → L84 RunAway+L87 battle", "bushi=0 cm=1 vis=1 near=1 lv=1000 cdmg=100 edmg=100"),
 ("C16b L67 Some·can1v1win=true → L80 battle+L81 Trace", "bushi=0 cm=1 vis=1 near=1 lv=1000 cdmg=5000 edmg=1 ehp=10 chp=100000"),
 ("C16c L81 Trace (bush 6·npos 2)", "bushi=6 cm=1 vis=1 near=1 npos=2 lv=1000 cdmg=5000 edmg=1 ehp=10 chp=100000"),
 ("C17 L67 Some·dominated(at_bush,nearc,evis)", "bushi=0 cm=1 vis=1 near=1 lv=1000 at_bush=1 nearc=1 evis=1"),
 ("C18 L67 Some·evis nearc(allies4,enemies1)", "bushi=0 cm=1 vis=1 near=1 lv=1000 nearc=1 evis=1"),
 ("C19 L107 vis&!cm·evis=0 → L132 RunAway+L133 battle", "bushi=0 cm=0 vis=1"),
 ("C20 L107·evis nearc·can1v1win=false → L126 RunAway+L128 battle", "bushi=0 cm=0 vis=1 evis=1 nearc=1 cdmg=100 edmg=100"),
 ("C20b L107·can1v1win=true → L124 battle only(Trace 없음)", "bushi=0 cm=0 vis=1 evis=1 nearc=1 cdmg=5000 edmg=1 ehp=10 chp=100000"),
 ("C21 L107·dominated(at_bush nearc evis) → L126 RunAway only", "bushi=0 cm=0 vis=1 evis=1 nearc=1 at_bush=1"),
 ("C22 L107·evis=1 far(nearc=0) → nearest Some(거리 무관)·L126", "bushi=0 cm=0 vis=1 evis=1 cdmg=100 edmg=100"),
 ("C22b L107·evis far·can1v1win=true → L124", "bushi=0 cm=0 vis=1 evis=1 cdmg=5000 edmg=1 ehp=10 chp=100000"),
 ("C23 vis·at_camp → esm=1·cm→1·L91 RunAway·L107 skip", "bushi=0 cm=0 vis=1 at_camp=1"),
 ("C24 steal Attack+Skill (jhp0 at_j)",    "bushi=3 cm=1 ticks=200 jhp0=1 at_j=1"),
 ("C25 steal hp=1 > dmg=0 → 없음",         "bushi=3 cm=1 ticks=200 at_j=1"),
 ("C26 steal 정글 범위 밖(bush 0)",         "bushi=0 cm=1 ticks=200 jhp0=1"),
 ("C27 team=1 Morgard blue_side=false",    "bushi=0 cm=0 team=1"),
 ("C28 team=1 at_camp",                    "bushi=6 cm=0 team=1 at_camp=1"),
 ("C29 seed 변화 rnd 대조",                 "bushi=0 cm=0 seed=777"),
 ("C30 seed 변화 AB rnd 대조",              "bushi=5 cm=1 seed=777"),
 ("C31 pos=0 Top",                         "bushi=0 cm=0 pos=0"),
 ("C32 version=2",                         "bushi=0 cm=1 vis=1 version=2"),
 ("C35 경계 L42 dist=100000(=1e10, > 아님) → check_move=1", "bushi=0 cm=0 at_camp_d=100000"),
 ("C36 경계 L42 dist=100001(>1e10) → AroundPosition", "bushi=0 cm=0 at_camp_d=100001"),
 ("C37 경계 L62 enemy@bush+250000 → nearest Some(L94)", "bushi=0 cm=1 near_d=250000 lv=1000"),
 ("C38 경계 L62 enemy@bush+250001 → None(L96)", "bushi=0 cm=1 near_d=250001 lv=1000"),
 ("C39 경계 L72 enemy@champ+150000 evis → enemies=1 dominated(at_bush)", "bushi=0 cm=1 vis=1 near=1 lv=1000 at_bush=1 evis=1 nearc_d=150000"),
 ("C40 경계 L72 enemy@champ+150001 evis → enemies=0 → !dominated → can1v1win=false → L84+L87", "bushi=0 cm=1 vis=1 near=1 lv=1000 at_bush=1 evis=1 nearc_d=150001 cdmg=100 edmg=100"),
 ("C41 경계 L117 enemy@champ+150000 evis (cm=0 vis) → enemies=1", "bushi=0 cm=0 vis=1 evis=1 at_bush=1 nearc_d=150000"),
 ("C42 경계 L117 enemy@champ+150001 evis → enemies=0 → !dominated → can1v1win", "bushi=0 cm=0 vis=1 evis=1 at_bush=1 nearc_d=150001 cdmg=100 edmg=100"),
 ("C33 esm0=1·!vis·!cm → L107 미진입(조건은 esm 아님)", "bushi=0 cm=0 esm=1"),
 ("C34 esm0=1·!vis·!cm·evis → L107 미진입", "bushi=0 cm=0 esm=1 evis=1 cdmg=100 edmg=100"),
]
out = io.open(os.path.join(os.path.dirname(__file__), 'o194_cases.log'), 'w', encoding='utf-8')
summ = []
for name, args in CASES:
    r = subprocess.run([EXE] + args.split(), capture_output=True, text=True, encoding='utf-8', errors='replace')
    txt = r.stdout + r.stderr
    res = [l for l in txt.splitlines() if l.startswith('RESULT')]
    tags = [l for l in txt.splitlines() if l.startswith('tags got')]
    pred = [l for l in txt.splitlines() if l.startswith('predict:')]
    selfl = [l for l in txt.splitlines() if l.startswith('self_after')]
    verdict = res[0] if res else 'CRASH rc=%s' % r.returncode
    summ.append((name, args, verdict, tags[0] if tags else '', pred[0][:160] if pred else '', selfl[0] if selfl else ''))
    out.write('=' * 100 + '\n## %s | %s\n' % (name, args) + txt + '\n')
out.close()
for s in summ:
    print(s[0], '|', s[1], '|', s[2])
    print('    ', s[3]); print('    ', s[4]); print('    ', s[5])
n = sum(1 for s in summ if s[2] == 'RESULT MATCH')
print('MATCH %d / %d' % (n, len(summ)))
