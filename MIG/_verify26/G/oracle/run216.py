# -*- coding: utf-8 -*-
"""run216 — o216.exe 를 케이스당 프로세스 1개로 돌려 진리표를 만든다(26차 G)."""
import subprocess, sys, io, os
EXE = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o216.exe"
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "o216_truth.log")
B = "tvis=0 arng=200000 mdist=300000 enemyt=150000 eatk=5000"   # 가시 타워 · 사거리 밖 · 적 5 이 walkup 위치 근접 · die_tick=60
cases = [
    ("g00 base: 차단 조건 전부 거짓 → false", B),
    ("g01 has_runaway → true (path A)", B + " runaway=1"),
    ("g02 risk_damage 1e5, champ hp 1 → high_risk → true", B + " spf=998:100000"),
    ("g03 risk 350 mhp 1000 (35000>=35000 경계) → true", B + " spf=998:350 mhp=1000"),
    ("g04 risk 349 mhp 1000 → false", B + " spf=998:349 mhp=1000"),
    ("g05 risk 350 mhp 0 (max(hp,1)=1) → true", B + " spf=998:350 mhp=0"),
    ("g06 risk 1 mhp 0 → 100>=35 true", B + " spf=998:1 mhp=0"),
    # path B (die_tick > tps*2)
    ("g10 hp 5001 (안 죽음) runaway=1 risk 1e5 → path B → false", B + " mhp=5001 runaway=1 spf=998:100000"),
    ("g11 hp 5001 runaway=0 → false", B + " mhp=5001"),
    # action 종류
    ("g20 atag=16 Skill (skill_effect None) → false", B + " runaway=1 atag=16"),
    ("g21 atag=17 Skill2 (level<=2 → NONE) → false", B + " runaway=1 atag=17"),
    ("g22 atag=18 Ult (level<=4 → NONE) → false", B + " runaway=1 atag=18"),
    ("g23 atag=3 RunAway → false", B + " runaway=1 atag=3"),
    ("g24 atag=19 Stop → false", B + " runaway=1 atag=19"),
    ("g25 atag=14 Trace → false", B + " runaway=1 atag=14"),
    ("g26 atag=0 (암묵 AroundPosition) → false", B + " runaway=1 atag=0"),
    # target 종류
    ("g30 target 적 챔피언 → false(비챔피언 전용)", B + " runaway=1 tgt=e0"),
    ("g31 target 아군 타워 → false(같은 팀)", B + " runaway=1 tgt=m"),
    ("g32 target 적 넥서스 → ?", B + " runaway=1 tgt=n"),
    # effect / range
    ("g40 attack_effect None → false", B + " runaway=1 atk=-1"),
    ("g41 in range(mdist 50000 < 200000) → false", B + " runaway=1 mdist=50000"),
    ("g42 atk=0 dmg 0 < hp 1 → v22 검사 경로(결과 관측)", B + " runaway=1 atk=0"),
    ("g43 tvis=2 (안 보임 → walkup=타워 위치 → tower_focus) → true", B + " tvis=2"),
    ("g44 tvis=1 (Invisible) → 동일 → true", B + " tvis=1"),
    # attackers
    ("g50 적 전부 멀리(attackers 빈) runaway=1 → false", B + " runaway=1 enemyt=500000"),
    ("g51 적 1 만 근접 → attackers 1 → true", B + " runaway=1 nenemy=1"),
    ("g52 vis=0 evis=1(visible_state 경로) → true", B + " runaway=1 vis=0 evis=1"),
    ("g53 vis=0 evis=0 → attackers 빈 → false", B + " runaway=1 vis=0 evis=0"),
    ("g54 적 사거리 경계: r=140000 → 적을 walkup 에서 140000 → 포함", B + " runaway=1 nenemy=1 enemyt=345000"),
    ("g55 적 사거리 경계+1: 140001 → 제외", B + " runaway=1 nenemy=1 enemyt=345001"),
    ("g56 dbg=0 이면 infos 없이 같은 판정", B + " runaway=1 dbg=0"),
    ("g57 casting Position(caster_radius 0) → walkup 10000 만큼 target 쪽 → 적 경계 다시", B + " runaway=1 nenemy=1 enemyt=335000 cpos=1"),
    ("g58 casting Position 경계+1", B + " runaway=1 nenemy=1 enemyt=335001 cpos=1"),
]
lines = []
for name, args in cases:
    p = subprocess.run([EXE] + args.split(), capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=120)
    res = [l for l in p.stdout.splitlines() if l.startswith("RESULT")]
    est = [l for l in p.stdout.splitlines() if l.startswith("est")]
    r = res[0] if res else ("NO RESULT rc=%s stderr=%s" % (p.returncode, p.stderr[-300:].replace("\n", " ")))
    lines.append("%-62s | %-75s | %s || %s" % (name, args, r, est[0][:160] if est else ""))
    print(lines[-1])
io.open(OUT, "w", encoding="utf-8").write("\n".join(lines) + "\n")
print("->", OUT)
