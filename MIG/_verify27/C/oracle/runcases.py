# -*- coding: utf-8 -*-
"""오라클 exe 를 케이스당 프로세스 1개로 돌려 로그를 모은다. 사용: runcases.py <exe이름> <n_cases> <logdir>"""
import subprocess, sys, os, io
exe = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\%s.exe" % sys.argv[1]
n = int(sys.argv[2]); logdir = sys.argv[3]
os.makedirs(logdir, exist_ok=True)
summary = []
for c in range(n):
    try:
        r = subprocess.run([exe, str(c)], capture_output=True, timeout=120)
        out = r.stdout.decode("utf-8", "replace") + r.stderr.decode("utf-8", "replace")
        code = r.returncode
    except subprocess.TimeoutExpired:
        out = "TIMEOUT"; code = -1
    io.open(os.path.join(logdir, "%s_case%d.log" % (sys.argv[1], c)), "w", encoding="utf-8").write(out)
    last = [l for l in out.splitlines() if l.startswith("case=")]
    summary.append("case %2d exit=%s %s" % (c, code, last[-1] if last else out.strip().splitlines()[-1:] ))
print("\n".join(summary))
