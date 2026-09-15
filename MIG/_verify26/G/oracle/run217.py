# -*- coding: utf-8 -*-
"""run217 — o217.exe 를 케이스당 프로세스 1개로 돌려 진리표를 만든다(26차 G)."""
import subprocess, sys, io, os
EXE = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o217.exe"
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "o217_truth.log")
L = "adm=50 thp=10"   # lethal_now 기본
cases = [
    ("c00 lethal baseline", L),
    ("c01 goal Trace -> 0", L + " goal=0"),
    ("c02 goal End -> 0", L + " goal=7"),
    ("c03 target ally -> 0", L + " tgt=a1"),
    ("c04 target tower -> 0", L + " tgt=t"),
    ("c05 undying -> 0", L + " undying=1"),
    ("c06 not visible -> 0", L + " vis=0"),
    ("c07 nonlethal no ready -> 0", "adm=50 thp=100"),
    ("c08 current==hp (>=) lethal", "adm=100 thp=100"),
    ("c09 current==hp-1 nonlethal", "adm=99 thp=100"),
    ("c0a thp=0 → max(hp,1)=1, adm=1 lethal", "adm=1 thp=0"),
    # combo / heavy (champ.attack_effect 주입 → ready_damage)
    ("c10 catk=500 thp=100 adm=50 (combo?)", "adm=50 thp=100 catk=500"),
    ("c11 catk=100 thp=100 (ready==hp combo?)", "adm=50 thp=100 catk=100"),
    ("c12 catk=99 thp=100 (heavy? 99*100>=7000)", "adm=50 thp=100 catk=99"),
    ("c13 catk=70 thp=100 (heavy 경계 7000>=7000)", "adm=50 thp=100 catk=70"),
    ("c14 catk=69 thp=100 (6900<7000 → 0)", "adm=50 thp=100 catk=69"),
    ("c15 catk=70 thp=100 far(mdist=200000, close 아님)", "adm=50 thp=100 catk=70 mdist=200000"),
    # range term (range=max_range_cached=20000 기본 · spd=1)
    ("c20 mdist=30000 (<=range+15000 → +10)", L + " mdist=30000"),
    ("c21 mdist=35000 (경계 → +10)", L + " mdist=35000"),
    ("c22 mdist=35001 (spd*12=12 → 0)", L + " mdist=35001"),
    ("c23 mspd=2000 mdist=44000 (<=range+24000 → +4)", L + " mspd=2000 mdist=44000"),
    ("c24 mspd=2000 mdist=44001 (→0)", L + " mspd=2000 mdist=44001"),
    ("c25 mspd=2000 mdist=35000 (+10 우선)", L + " mspd=2000 mdist=35000"),
    # incoming term (my_hp=100)
    ("c30 applyed=60 mhp=100 (6000>=6000 → +10)", L + " mhp=100 spf=988:60"),
    ("c31 applyed=59 mhp=100 (→ +5)", L + " mhp=100 spf=988:59"),
    ("c32 applyed=30 mhp=100 (3000>=3000 → +5)", L + " mhp=100 spf=988:30"),
    ("c33 applyed=29 mhp=100 (→ 0)", L + " mhp=100 spf=988:29"),
    ("c34 risk=60 mhp=100 (+10)", L + " mhp=100 spf=998:60"),
    ("c35 tower=120 mhp=100 (/2=60 → +10)", L + " mhp=100 spf=9b0:120"),
    ("c36 tower=119 mhp=100 (/2=59 → +5)", L + " mhp=100 spf=9b0:119"),
    ("c37 mhp=0 → max(hp,1)=1 applyed=1 (+10)", L + " mhp=0 spf=988:1"),
    # low hp term
    ("c40 thp=35 tmax=100 adm=100 (3500<=3500 → +8)", "adm=100 thp=35 tmax=100"),
    ("c41 thp=36 tmax=100 adm=100 (→ 0)", "adm=100 thp=36 tmax=100"),
    ("c42 tmax=0 thp=0 adm=1 (max(tmax,1)=1: 0<=35 → +8)", "adm=1 thp=0 tmax=0"),
    # allies / enemies
    ("c50 enemies far, target only (ne=1) na=0 → ne>na lethal → 0 add", L + " enemy=200000"),
    ("c51 enemies far, ally 1 near (na=1,ne=1 → +5)", L + " enemy=200000 ally=50000 nally=1"),
    ("c52 enemies far, ally 4 near (+5)", L + " enemy=200000 ally=50000 nally=4"),
    ("c53 ally at 120000 (경계 포함 → na=1 → +5)", L + " enemy=200000 ally=120000 nally=1"),
    ("c54 ally at 120001 (제외 → na=0 → ne>na)", L + " enemy=200000 ally=120001 nally=1"),
    ("c55 enemy at 120000 near champ vis (포함)", L + " enemy=120000 nenemy=1"),
    ("c56 enemies vis=0 이면 target 도 안 보임 → gate 0", L + " enemy=50000 vis=0"),
    ("c57 heavy(catk=70) ne=5 na=0 → -15", "adm=50 thp=100 catk=70"),
    ("c58 heavy(catk=70) ne=1(far) na=0 → 1<0+2 → no -15", "adm=50 thp=100 catk=70 enemy=200000"),
    ("c59 heavy(catk=70) ne=2 na=0 → 2<2 false → -15", "adm=50 thp=100 catk=70 enemy=200000 nenemy=3"),
    ("c5a heavy(catk=70) ne=2 na=1 → 2<3 → no -15", "adm=50 thp=100 catk=70 enemy=200000 nenemy=3 ally=50000 nally=1"),
    ("c5b combo(catk=500) ne=5 na=0 → combo → no -15", "adm=50 thp=100 catk=500"),
    # clamp
    ("c60 clamp95: lethal+range10+inc10+low8+5", "adm=100 thp=35 tmax=100 mdist=30000 mhp=100 spf=988:60 enemy=200000 ally=50000 nally=4"),
]
lines = []
for name, args in cases:
    p = subprocess.run([EXE] + args.split(), capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=120)
    res = [l for l in p.stdout.splitlines() if l.startswith("RESULT")]
    r = res[0] if res else ("NO RESULT rc=%s stderr=%s" % (p.returncode, p.stderr[-300:].replace("\n", " ")))
    lines.append("%-55s | %-60s | %s" % (name, args, r))
    print(lines[-1])
io.open(OUT, "w", encoding="utf-8").write("\n".join(lines) + "\n")
print("->", OUT)
