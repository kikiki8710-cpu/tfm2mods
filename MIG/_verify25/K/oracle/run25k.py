# -*- coding: utf-8 -*-
"""run25k.py — o25k.exe 케이스 드라이버(케이스당 프로세스 1개 · TEMPLATE 함정 ③). 로그 = o25k_<case>.log · 요약 stdout"""
import subprocess, io, os, sys
EXE = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o25k.exe"
OUT = os.path.dirname(os.path.abspath(__file__))
sys.stdout.reconfigure(encoding="utf-8", errors="replace")

cases = [
    # ── 190 AttackNexus ──
    ("n_base",        ["n_base"]),
    ("n_base_t1",     ["n_base", "team=1", "pos=2"]),
    ("n_tower7",      ["n_base", "x=542000", "y=368000", "atk=1", "range=100000"]),                 # 미드 1차 타워(id7) 50000 앞
    ("n_nexus_on",    ["n_base", "x=824000", "y=96000", "atk=1", "range=100000", "towers_off=1", "nexus_on=1"]),
    ("n_nexus_off",   ["n_base", "x=824000", "y=96000", "atk=1", "range=100000", "towers_off=1"]),  # 넥서스 can_target=0 → 후보 없음 기대
    ("n_range_eq",    ["n_base", "atk=1", "range=100000", "towers_off=1", "nexus_on=1", "range_delta=0"]),
    ("n_range_gt",    ["n_base", "atk=1", "range=100000", "towers_off=1", "nexus_on=1", "range_delta=1"]),
    ("n_hp55",        ["n_base", "x=824000", "y=96000", "atk=1", "range=100000", "towers_off=1", "nexus_on=1", "maxhp=1000", "hp=550"]),
    ("n_hp56",        ["n_base", "x=824000", "y=96000", "atk=1", "range=100000", "towers_off=1", "nexus_on=1", "maxhp=1000", "hp=560"]),
    ("n_hp55_tower",  ["n_base", "x=542000", "y=368000", "atk=1", "range=100000", "maxhp=1000", "hp=550"]),
    ("n_hp56_tower",  ["n_base", "x=542000", "y=368000", "atk=1", "range=100000", "maxhp=1000", "hp=560"]),
    # ── 191 EpicCheck ──
    ("e_base_t0",     ["e_base", "team=0", "pos=1"]),
    ("e_base_t1",     ["e_base", "team=1", "pos=1"]),
    ("e_mc1_own",     ["e_base", "team=0", "pos=1", "mc=1"]),
    ("e_enemyside_far", ["e_at", "team=0", "pos=1", "x=200000", "y=700000"]),
    ("e_enemyside_far_mc1", ["e_at", "team=0", "pos=1", "x=200000", "y=700000", "mc=1"]),
    ("e_stump_eq",    ["e_at", "team=0", "pos=1", "x=326000", "y=448000"]),   # Stump(blue)+(70000,0)
    ("e_stump_gt",    ["e_at", "team=0", "pos=1", "x=326001", "y=448000"]),
    ("e_morg_eq",     ["e_at", "team=0", "pos=1", "x=138000", "y=288000"]),   # Morgard(blue)-(150000,0) · 적진 · mc0=0
    ("e_morg_gt",     ["e_at", "team=0", "pos=1", "x=137999", "y=288000"]),
    ("e_t1_ownside",   ["e_at", "team=1", "pos=1", "x=700000", "y=200000"]),
    ("e_t0_enemyside", ["e_at", "team=0", "pos=1", "x=700000", "y=200000"]),
    ("e_t1_enemyside", ["e_at", "team=1", "pos=1", "x=200000", "y=700000"]),
    ("e_t1_stump_eq",  ["e_at", "team=1", "pos=1", "x=448000", "y=326000"]),   # Stump(red)=(448000,256000)+(0,70000)
    ("e_t1_stump_gt",  ["e_at", "team=1", "pos=1", "x=448000", "y=326001"]),
    ("e_v20_5",       ["e_v20_5"]),
    ("e_v20_15",      ["e_v20_15"]),
    ("e_v20_15_dbg",  ["e_v20_15", "dbg=1"]),
    ("e_v20_31",      ["e_v20_31"]),
    ("e_v20_31_dbg",  ["e_v20_31", "dbg=1"]),
]
only = sys.argv[1:]
for name, args in cases:
    if only and name not in only: continue
    r = subprocess.run([EXE] + args, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=300)
    log = os.path.join(OUT, "o25k_%s.log" % name)
    io.open(log, "w", encoding="utf-8").write("$ o25k.exe %s\nrc=%s\n%s\n%s" % (" ".join(args), r.returncode, r.stdout, r.stderr))
    summ = [l for l in r.stdout.split("\n") if l.startswith(("summary", "self.move_check", "rnd", "predict191", "predict_tower", "debug.infos"))]
    print("==", name, "rc=%s" % r.returncode)
    for l in summ: print("   ", l[:260])
    if r.returncode != 0: print("   STDERR:", r.stderr[-400:])
