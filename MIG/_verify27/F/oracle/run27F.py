# -*- coding: utf-8 -*-
"""27차 F 오라클 러너 — 케이스당 프로세스 1개(TLS 메모 규칙). 출력 = o27F_cases.log (RESULT 줄만 모아 요약)."""
import io, os, subprocess, sys
EXE = os.path.join(os.environ.get("TEMP", r"C:\Users\jungs\AppData\Local\Temp"), "tfm2_spanprobe", "o27F.exe")
HERE = os.path.dirname(os.path.abspath(__file__))
CASES = [
    # 267 expected_dps: a s s2 lv
    "dps 100 0 0 1", "dps 0 200 0 1", "dps 0 0 300 3", "dps 0 0 300 2", "dps 100 200 300 3", "dps 100 200 300 2",
    "dps 0 0 0 1", "dps 7 13 29 5", "dps 1 1 1 3", "dps 12345 0 0 1",
    # 262 target_score: hp maxhp dx dmg
    "ts 1000 1000 0 0", "ts 400 1000 0 0", "ts 200 1000 0 0", "ts 3 1000 0 0", "ts 0 1000 0 0",
    "ts 1000 1000 150000 0", "ts 1000 1000 300000 0", "ts 1000 1000 900000 0",
    "ts 55 1000 0 50", "ts 56 1000 0 50", "ts 250 1000 0 50", "ts 500 1000 0 50", "ts 510 1000 0 50", "ts 260 1000 100000 50",
    "ts 1000 0 0 0", "ts 1000 1000 5000 7",
    # 266 can_near_enemies_range: x y d tick pat ms
    "cnr 100000 100000 50000 0 0 0", "cnr 456500 7500 0 0 0 0", "cnr 456500 7500 200000 0 0 0", "cnr 456500 7500 2000000 0 0 0",
    "cnr 456500 7500 0 1000 2 0", "cnr 456500 7500 0 1000 2 1000", "cnr 500000 7500 0 1000 2 500",
    "cnr 456500 7500 0 1000 1 0", "cnr 456500 7500 0 1000 1 3000", "cnr 456500 7500 100000 1000 1 3000",
    "cnr 456500 7500 0 5000 0 0", "cnr 456500 7500 0 20000 0 0", "cnr 800000 100000 300000 700 0 200",
    "cnr 456500 7500 0 190 0 0", "cnr 456500 7500 0 180 0 0",
    # 265 v54: amount range ex spread
    "v54 100 200000 0 20000", "v54 100 200000 1 20000", "v54 100 200000 2 20000", "v54 100 10000 0 20000",
    "v54 0 200000 0 20000", "v54 1000 500000 0 20000", "v54 100 45000 0 20000", "v54 100 200000 0 60000", "v54 50 200000 0 5000",
    "v54 100 200000 2 20000 0 55", "v54 100 200000 0 20000 3000 2", "v54 700 200000 0 20000", "v54 100 20000 0 20000", "v54 100 19999 0 20000",
]
out = io.open(os.path.join(HERE, "o27F_cases.log"), "w", encoding="utf-8")
summ = {}
for c in CASES:
    try:
        r = subprocess.run([EXE] + c.split(), capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=120)
        lines = [l for l in (r.stdout or "").splitlines() if l.startswith(("DPS", "TS", "CNR", "V54"))]
        res = lines[-1] if lines else ("(no RESULT) rc=%s stderr=%s" % (r.returncode, (r.stderr or "")[-300:].replace("\n", " ")))
    except Exception as e:
        res = "(EXC) %r" % e
    out.write("## %s\n%s\n\n" % (c, res))
    fam = c.split()[0]
    verdict = "MATCH" if res.rstrip().endswith("MATCH") else ("DIFF" if res.rstrip().endswith("DIFF") else "ERR")
    summ.setdefault(fam, {}).setdefault(verdict, 0)
    summ[fam][verdict] += 1
    print("%-40s %s" % (c, verdict if verdict != "DIFF" else res[-400:]))
out.write("\n== summary ==\n%s\n" % summ)
out.close()
print("summary", summ)
