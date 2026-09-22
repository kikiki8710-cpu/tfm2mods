"""run198.py — o198.exe 를 케이스당 프로세스 1개로 돌려 진리표를 만든다(TLS 메모 함수 check_kill_die_tick 경유 → 프로세스 분리 필수)."""
import io, os, subprocess, sys
EXE = os.path.expandvars(r"%LOCALAPPDATA%\Temp\tfm2_spanprobe\o198.exe")
HERE = os.path.dirname(os.path.abspath(__file__))
CASES = [
    ("R0", "nr=0 epic=0 hp=500 max=1000"),
    ("R1", "nr=1 epic=0 hp=290 max=1000"),
    ("R1b", "nr=1 epic=0 hp=289 max=1000"),
    ("R2", "nr=1 epic=0 hp=300 max=1000"),
    ("R2b", "nr=1 epic=0 hp=299 max=1000"),
    ("E1", "nr=0 epic=1 focus=me hp=3d max=100000"),
    ("E2", "nr=0 epic=1 focus=me hp=3d+1 max=100000"),
    ("E3", "nr=0 epic=1 focus=none hp=d max=100000"),
    ("E4", "nr=0 epic=1 focus=none hp=d+1 max=100000"),
    ("E5", "nr=0 epic=1 focus=other hp=3d max=100000"),
    ("E6", "nr=0 epic=1 focus=other hp=d max=100000"),
    ("E7", "nr=0 epic=1 focus=me hp=3d-1 max=100000"),
    ("E8", "nr=1 epic=1 focus=me hp=3d+1 max=1000"),
    ("E9", "nr=1 epic=1 focus=none hp=d+1 max=400"),
    ("N1", "nr=0 epic=1 focus=me hp=3d+1 near=1 max=100000"),
    ("N2", "nr=0 epic=1 focus=me hp=3d+1 near=1 dx=50000 max=100000"),
    ("N3", "nr=0 epic=0 hp=500 enemy=1 max=100000"),
    ("N4", "nr=0 epic=1 focus=none hp=d+1 near=1 enemy=1 max=100000"),
]
out = io.open(os.path.join(HERE, "o198_truth.log"), "w", encoding="utf-8")
summary = []
for name, argv in CASES:
    p = subprocess.run([EXE] + argv.split(), capture_output=True, text=True, encoding="utf-8", errors="replace")
    txt = p.stdout + p.stderr
    out.write("==== %s  %s\n%s\n" % (name, argv, txt))
    d = {}
    tags = []
    for l in txt.splitlines():
        f = l.split("\t")
        if f[0] == "need_recall": d["nr"] = " ".join(f[1:])
        if f[0] == "recall": d["recall"] = " ".join(f[1:])
        if f[0] == "res_len": d["len"] = f[1]
        if f[0].startswith("elem["): tags.append(f[1])
        if f[0] == "rnd_changed": d["rnd"] = f[1]
        if f[0] == "champ_set": d["champ"] = " ".join(f[1:])
    summary.append("%-4s rc=%d %-46s | %s | %s | len=%s tags=%s rnd=%s" % (name, p.returncode, argv, d.get("nr"), d.get("recall"), d.get("len"), ",".join(tags), d.get("rnd")))
out.close()
s = "\n".join(summary)
io.open(os.path.join(HERE, "o198_summary.txt"), "w", encoding="utf-8").write(s + "\n")
print(s)
