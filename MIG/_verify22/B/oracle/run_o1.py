# -*- coding: utf-8 -*-
"""v22B_o1.exe 를 케이스당 프로세스 1개로 돌려 v22B_o1.tsv 에 모은다."""
import subprocess, io, os, sys
EXE = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\v22B_o1.exe"
OUT = r"C:\tfm2mods\MIG\_verify22\B\oracle\v22B_o1.tsv"
cases = [
    ["sm_safe", "1"], ["sm_safe", "2"], ["sm_near", "1"], ["sm_near", "2"],
    ["sm_tdanger", "1"], ["sm_tdanger", "2"],
    ["sm_sdanger", "1"], ["sm_sdanger", "2"],
    ["sm_sdanger_tsafe", "1"], ["sm_sdanger_tsafe", "2"],
    ["cv_pos", "1000", "0"], ["cv_pos", "149999", "0"], ["cv_pos", "150000", "0"], ["cv_pos", "150001", "0"], ["cv_pos", "400000", "0"],
    ["cv_pos", "1000", "1"], ["cv_pos", "-300000", "0"], ["cv_pos", "-1000", "1"],
    ["cv_dir", "5000", "-7000"], ["cv_dir", "-5000", "7000"], ["cv_dir", "0", "0"],
    ["as_default"], ["as_siege", "0"], ["as_siege", "15000"], ["as_siege", "20030"], ["as_siege", "20031"], ["as_siege", "25000"],
]
env = dict(os.environ); env.pop("DM_NO_DODGE", None)
with io.open(OUT, "w", encoding="utf-8") as f:
    for c in cases:
        p = subprocess.run([EXE] + c, capture_output=True, text=True, env=env, timeout=120)
        f.write("### %s (rc=%s)\n" % (" ".join(c), p.returncode))
        f.write(p.stdout)
        if p.stderr.strip(): f.write("STDERR: " + p.stderr[-600:] + "\n")
        print("###", " ".join(c), "rc", p.returncode)
        for ln in p.stdout.splitlines():
            if ln.startswith(("RESULT", "PRED", "DANGER", "SKILL", "TOWER", "SIGN")): print("  ", ln)
        if p.stderr.strip(): print("   STDERR:", p.stderr.strip()[-300:])
