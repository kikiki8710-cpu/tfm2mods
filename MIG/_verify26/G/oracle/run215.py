# -*- coding: utf-8 -*-
"""run215 — o215.exe 를 케이스당 프로세스 1개로 돌려 진리표를 만든다(26차 G)."""
import subprocess, sys, io, os
EXE = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o215.exe"
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "o215_truth.log")
cases = [
    # 게이트
    ("c00 baseline e0 charm60 atk", "tgt=e0 charm=60"),
    ("c01 other action(downcast 실패)", "tgt=e0 charm=60 otheract=1"),
    ("c02 target ally", "tgt=a1 charm=60"),
    ("c03 target tower(ty!=13)", "tgt=t charm=60"),
    ("c04 cc_immune", "tgt=e0 charm=60 immune=1"),
    ("c05 target stunned", "tgt=e0 charm=60 cc=stun"),
    ("c06 target BlockAttack(is_cc false)", "tgt=e0 charm=60 cc=block"),
    ("c07 target BlockSkill+Charm(2번째 원소 is_cc)", "tgt=e0 charm=60 cc=charm"),
    ("c08 charm0 atk(cc None) -> 0", "tgt=e0 charm=0"),
    # cc_time clamp
    ("c10 charm=35 -> 12", "tgt=e0 charm=35"),
    ("c11 charm=36 -> 12", "tgt=e0 charm=36"),
    ("c12 charm=39 -> 13", "tgt=e0 charm=39"),
    ("c13 charm=89 -> 29", "tgt=e0 charm=89"),
    ("c14 charm=90 -> 30", "tgt=e0 charm=90"),
    ("c15 charm=300 -> 30", "tgt=e0 charm=300"),
    ("c16 charm0 stun45 -> cc 45", "tgt=e0 charm=0 eff=stun:45"),
    ("c17 charm60 stun45 -> max 60", "tgt=e0 charm=60 eff=stun:45"),
    ("c18 charm30 stun90 -> max 90", "tgt=e0 charm=30 eff=stun:90"),
    # ally/enemy near
    ("c20 ally 4 near(119000)", "tgt=e0 charm=60 ally=119000"),
    ("c21 ally 4 at 120000(경계, <120000^2+1 → 포함)", "tgt=e0 charm=60 ally=120000 nally=1"),
    ("c22 ally 1 at 120001(제외)", "tgt=e0 charm=60 ally=120001 nally=1"),
    ("c23 ally 2 near", "tgt=e0 charm=60 ally=50000 nally=2"),
    ("c24 enemy 4 near no vis", "tgt=e0 charm=60 enemy=50000"),
    ("c25 enemy 4 near vis", "tgt=e0 charm=60 enemy=50000 vis=1"),
    ("c26 enemy 1 near vis", "tgt=e0 charm=60 enemy=50000 vis=1 nenemy=1"),
    ("c27 enemy at 90000 vis(경계 포함)", "tgt=e0 charm=60 enemy=90000 vis=1 nenemy=1"),
    ("c28 enemy at 90001 vis(제외)", "tgt=e0 charm=60 enemy=90001 vis=1 nenemy=1"),
    # focus/support
    ("c30 goal RunAway", "tgt=e0 charm=60 goal=4"),
    ("c31 goal End", "tgt=e0 charm=60 goal=7"),
    ("c32 goal Trace focus=e1(far)", "tgt=e0 charm=60 goal=0 focus=e1"),
    ("c33 goal Trace focus=e1 near", "tgt=e0 charm=60 goal=0 focus=e1 enemy=50000 nenemy=1"),
    ("c34 goal Trace focus=a1 near(팀 다름 → 0)", "tgt=e0 charm=60 goal=0 focus=a1 ally=50000 nally=1"),
    ("c35 goal RunAway sup=e0", "tgt=e0 charm=60 goal=4 sup=e0"),
    ("c36 goal RunAway sup=e1", "tgt=e0 charm=60 goal=4 sup=e1"),
    ("c37 goal Assassin focus=e0", "tgt=e0 charm=60 goal=5"),
    ("c38 goal AssassinReady focus=none", "tgt=e0 charm=60 goal=6 focus=none"),
    # near_enemies 파라미터
    ("c40 np ad0 rd0", "tgt=e0 charm=60 np=1"),
    ("c41 np ad500 rd200 thp1000", "tgt=e0 charm=60 np=1 ad=500 rd=200 thp=1000"),
    ("c42 np adm5000 thp100(ult 상한35)", "tgt=e0 charm=60 np=1 adm=5000 thp=100 ad=100"),
    ("c43 np thp0(max(hp,1))", "tgt=e0 charm=60 np=1 thp=0 ad=10"),
    ("c44 np hv=100 ult50 thp1000 ad100", "tgt=e0 charm=60 np=1 ad=100 thp=1000 tpf=a8:1000"),
    ("c45 np attack_value=50", "tgt=e0 charm=60 np=1 ad=100 thp=1000 tpf=a8:50"),
    ("c46 np attack_value=200", "tgt=e0 charm=60 np=1 ad=100 thp=1000 tpf=a8:200"),
    ("c47 np hv100 adm5000 thp20000 (ult 24, incoming 0)", "tgt=e0 charm=60 np=1 ad=100 thp=20000 adm=5000 tpf=a8:1000"),
    ("c48 np hv100 adm5000 thp100 ad100 (35/22 상한)", "tgt=e0 charm=0 eff=stun:36 np=1 ad=100 thp=100 adm=5000 tpf=a8:1000 goal=4"),
    ("c49 np hv100 ad0 rd0 (incoming 0 → 항 없음)", "tgt=e0 charm=60 np=1 ad=0 rd=0 thp=1000 tpf=a8:1000"),
    ("c4a np hv100 rd=300 ad=0 thp=1000", "tgt=e0 charm=60 np=1 ad=0 rd=300 thp=1000 tpf=a8:1000"),
    # clamp 70
    ("c50 max everything", "tgt=e0 charm=300 ally=50000 enemy=50000 vis=1 goal=0 np=1 ad=5000 rd=5000 adm=5000 thp=100"),
    ("c51 max no np", "tgt=e0 charm=300 ally=50000 enemy=50000 vis=1 goal=0"),
]
lines = []
for name, args in cases:
    p = subprocess.run([EXE] + args.split(), capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=120)
    res = [l for l in p.stdout.splitlines() if l.startswith("RESULT")]
    r = res[0] if res else ("NO RESULT rc=%s stderr=%s" % (p.returncode, p.stderr[-300:].replace("\n", " ")))
    lines.append("%-45s | %-70s | %s" % (name, args, r))
    print(lines[-1])
io.open(OUT, "w", encoding="utf-8").write("\n".join(lines) + "\n")
print("->", OUT)
