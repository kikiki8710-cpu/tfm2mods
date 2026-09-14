# -*- coding: utf-8 -*-
"""23차 C 오라클 드라이버 — 케이스당 프로세스 1개(TEMPLATE 함정 ③). 로그 = oracle/o23c_<fn>.log"""
import subprocess, os, sys, io
EXE = os.path.join(os.environ.get("LOCALAPPDATA", r"C:\Users\jungs\AppData\Local"), "Temp", "tfm2_spanprobe", "o23c.exe")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.stdout.reconfigure(encoding="utf-8", errors="replace")

CASES = {
    # 150: tutorial tick cover cdmg tdmg thp chp cmax threat seen version
    "150": [
        # 기본: 커버 0 · 라인전 · 마무리 1방 · 위협 없음
        "0 1000 0 1000 100 900 2000 2000 0 0 55",     # finish_now, no threat → true
        "0 1000 0 1000 100 900 2000 2000 1 1 55",     # threat(dist==r, seen) · absorb(2000>500+100) → true
        "0 1000 0 1000 100 900 100 2000 1 1 55",      # threat · hp 100 ≤ floor 500+100 → absorb false → false
        "0 1000 0 1000 100 900 100 2000 2 1 55",      # dist==r+1 → 비위협 → true
        "0 1000 0 1000 100 900 100 2000 1 0 55",      # seen=0 → is_recent_visible false → 비위협 → true
        "0 1000 0 1000 100 900 100 2000 3 1 55",      # tower dist 180000 → 위협 → false
        "0 1000 0 1000 100 900 100 2000 4 1 55",      # tower dist 180001 → 비위협 → true
        "0 1000 0 1000 0 900 100 2000 1 1 55",        # tower_dmg 0 → absorb true → true(finish_now)
        "0 1000 0 500 100 900 2000 2000 0 0 55",      # finish_soon(1000>=900) · !finish_now · count 0 → false
        "0 1000 0 400 100 900 2000 2000 0 0 55",      # 800 < 900 → not_soon → false
        "0 1000 0 450 100 900 2000 2000 0 0 55",      # 900 >= 900 경계 → soon · count0 → false
        "0 1000 0 0 100 900 2000 2000 0 0 55",        # dmg 0 → false
        "0 16199 0 1000 100 900 2000 2000 0 0 55",    # tick 16199 < 16200 → 라인전 → true
        "0 16200 0 1000 100 900 2000 2000 0 0 55",    # tick 16200 → 라인전 종료 → false
        "1 16200 0 1000 100 900 2000 2000 0 0 55",    # tutorial First(1) → 검사 생략 → true
        "5 16200 0 1000 100 900 2000 2000 0 0 55",    # MidBottom(5) → 검사 → false
        "7 16200 0 1000 100 900 2000 2000 0 0 55",    # Line(7) → false
        "8 16200 0 1000 100 900 2000 2000 0 0 55",    # Total(8) → false
        "6 16200 0 1000 100 900 2000 2000 0 0 55",    # JungleOnly(6) → 생략 → true
        "0 1000 1 500 100 900 2000 2000 0 0 55",      # cover 1 targeted, tdmg 100 < minion hp → will_die false → count==1 → ret true(cover)
        "0 1000 1 500 100000 900 2000 2000 0 0 55",   # cover 1 targeted, will_die → 계속 → finish_soon·count1·absorb? tower_dmg 100000 → absorb false → false
        "0 1000 1 500 0 900 2000 2000 0 0 55",        # tdmg 0 → will_die = hp <= 0 false → cover → true
        "0 1000 2 500 100000 900 2000 2000 0 0 55",   # cover 2 → true
        "0 1000 3 500 100000 900 2000 2000 0 0 55",   # cover 1 not targeted → will_die false → true
        "0 1000 1 500 100000 900 2000 2000 0 0 55",   # (중복 확인)
        "0 1000 1 1000 100000 900 2000 2000 0 0 55",  # cover1 will_die · finish_now · tower_dmg 100000 → absorb false · threat none → true
        "0 1000 1 500 100000 900 400000 2000 0 0 55", # cover1 will_die · !finish_now · absorb(400000>500+100000) · no threat → true
        "0 1000 1 500 100000 900 400000 2000 1 1 55", # + threat → false
        "0 1000 0 1000 100 900 601 2000 1 1 55",      # hp 601 > 500+100 → absorb true → true (경계)
        "0 1000 0 1000 100 900 600 2000 1 1 55",      # hp 600 → absorb false → false (경계)
        "0 1000 0 452 100 900 2000 2000 0 0 55",      # finish_soon 경계 스윕(dmg*2 == 900?)
        "0 1000 0 453 100 900 2000 2000 0 0 55",
        "0 1000 0 454 100 900 2000 2000 0 0 55",
        "0 1000 0 455 100 900 2000 2000 0 0 55",
        "0 1000 0 908 100 900 2000 2000 0 0 55",      # finish_now 경계(dmg == 900?)
        "0 1000 0 909 100 900 2000 2000 0 0 55",
        "0 1000 0 1000 100 900 2000 2000 2 1 55",     # threat=2 (y축 r+1) → 비위협 → true(finish_now·absorb) — absorb 로 가려짐
        "0 1000 0 1000 100 900 100 2000 2 1 55",      # threat=2, absorb false → !threat → true
        "0 1000 0 1000 100 900 100 2000 1 1 55",      # threat=1(y축 r), absorb false → false
        "0 1000 1 500 100000 900 400000 2000 2 1 55", # cover1·will_die·!finish_now·absorb·threat r+1 → true
        "1 16200 0 1000 100 900 2000 2000 0 0 55",    # tutorial First(1) → 검사 생략 → true (타워 폴백)
        "5 16200 0 1000 100 900 2000 2000 0 0 55",    # MidBottom(5) → 검사 → false
        "6 16200 0 1000 100 900 2000 2000 0 0 55",    # JungleOnly(6) → 생략 → true
        "2 16200 0 1000 100 900 2000 2000 0 0 55",    # TopSolo(2) → 생략 → true
    ],
    # 156: tutorial kind threat seen vis tick
    "156": [
        "0 0 0 1 1 1000", "0 0 1 1 1 1000", "0 0 2 1 1 1000", "0 0 3 1 1 1000",
        "0 1 0 1 1 1000", "0 1 1 1 1 1000", "0 1 2 1 1 1000", "0 1 3 1 1 1000",
        "0 0 1 0 1 1000", "0 1 1 0 1 1000",      # seen=0
        "0 0 1 1 0 1000", "0 1 1 1 0 1000",      # vis=0 (visible_state 미설정)
    ],
    # 151/153/154: tutorial case level
    "151": ["0 0 -1", "0 1 -1", "0 2 -1", "0 3 -1", "0 4 -1", "0 5 -1", "0 6 -1", "0 7 -1", "0 4 5", "0 5 5", "0 6 5"],
    "153": ["0 0 -1", "0 1 -1", "0 2 -1", "0 2 3", "0 1 3", "0 3 3", "0 4 3", "0 0 3", "0 2 2", "0 5 3", "0 6 3", "0 7 3", "0 5 5", "0 6 5"],
    "154": ["0 0 -1", "0 1 -1", "0 2 -1", "0 2 5", "0 1 5", "0 3 5", "0 4 5", "0 0 5", "0 2 4"],
    # 157: tutorial case
    "157": ["0 0", "0 1", "0 2", "0 3", "0 4"],
}

fns = sys.argv[1:] or list(CASES.keys())
for fn in fns:
    out = io.open(os.path.join(HERE, "o23c_%s.log" % fn), "w", encoding="utf-8")
    for i, c in enumerate(CASES[fn]):
        cmd = [EXE, fn] + c.split()
        try:
            p = subprocess.run(cmd, capture_output=True, timeout=300)
            so = p.stdout.decode("utf-8", "replace"); se = p.stderr.decode("utf-8", "replace")
            res = [ln for ln in so.splitlines() if ln.startswith("RESULT") or ln.startswith("pre") or ln.startswith("minions")]
            line = "case%d\targs=%s\texit=%d\n\t%s" % (i, c, p.returncode, "\n\t".join(res))
            if p.returncode != 0:
                line += "\n\tSTDERR: " + se.strip().replace("\n", " | ")[:400]
        except Exception as ex:
            line = "case%d\targs=%s\tEXC %s" % (i, c, ex)
        print(line); out.write(line + "\n")
    out.close()
