"""run186.py — o186.exe 케이스 드라이버(케이스당 프로세스 1개). 결과 = o186_cases.log / 요약 stdout"""
import subprocess, io, sys, itertools
EXE = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o186.exe"
OUT = r"C:\tfm2mods\MIG\_verify24\F\oracle\o186_cases.log"

cases = []
def add(mode, **kw): cases.append((mode, kw))

# ── L 레거시(공격자 0) ── 경계: value-5 < hp · value+total*3/10 < hp · total*3/10 < hp · 29999 · flag · ty · acc/seed
for ty in (0, 1, 2):
    for thp in (95, 96):                       # value=100 → 95 < hp ?
        add("L", ty=ty, dmg=100, thp=thp, tmax=400)
    for thp in (220, 221):                     # 100 + 120 < hp ?  (tmax 400 → 120)
        add("L", ty=ty, dmg=100, thp=thp, tmax=400)
    for thp in (120, 121):                     # can kill(dmg 1000) · 0 + 120 < hp ?
        add("L", ty=ty, dmg=1000, thp=thp, tmax=400)
for cool in (29, 30, 31):                      # cool_pre = 1000*cool > 29999 ?
    for flag_pos in (0, 1):
        add("L", ty=1, dmg=100, thp=300, tmax=400, cool=cool, pos=flag_pos)
        add("L", ty=1, dmg=100, thp=221, tmax=400, cool=cool, pos=flag_pos)   # L393 경로(flag 무관)
# flag: tutorial·tick 경계 (first_spawn 18000 → boundary 16200)
for tut in (0, 1, 5, 7, 8):
    for tick in (16199, 16200):
        add("L", ty=1, dmg=100, thp=300, tmax=400, cool=10, pos=0, tut=tut, tick=tick)
# L359 (can kill · total*3/10 >= hp) flag 15/20
for pos in (0, 1):
    add("L", ty=1, dmg=1000, thp=120, tmax=400, pos=pos)
    add("L", ty=0, dmg=1000, thp=120, tmax=400, pos=pos)
# 미니언 nearest_enemy = 구조물 + 적 에픽버프(buff1)
for mne in (1, 2, 3, 4):
    for b1 in (0, 1):
        add("L", ty=1, dmg=100, thp=300, tmax=400, mne=mne, buff1=b1)
add("L", ty=1, dmg=100, thp=300, tmax=400, mne=0, buff1=1)
# 난수: acc<1000 · version 0(jrng None) / 55
for lh in (0, 50):
    for ver in (0, 55):
        for seed in (1, 2, 3, 4, 5, 6):
            add("L", ty=1, dmg=100, thp=150, tmax=400, last_hit=lh, version=ver, seed=seed)
            add("L", ty=1, dmg=1000, thp=120, tmax=400, last_hit=lh, version=ver, seed=seed)
            add("L", ty=2, dmg=100, thp=96, tmax=400, last_hit=lh, version=ver, seed=seed)
# speed_mult 변형(start/cool 스케일)
add("L", ty=1, dmg=100, thp=300, tmax=400, cool=60, speed=200)
add("L", ty=1, dmg=100, thp=300, tmax=400, cool=60, speed=50)

# ── W 스냅샷 ── 경계: pre>999 · predicted>value+5 · death<=cool+start · death>start+5 · predicted<=2v · predicted>3v · concurrent/earlier
for P in (0, 1, 105, 106, 200, 201, 300, 301):
    for D in (15, 16, 70, 71):
        add("W", ty=1, dmg=100, P=P, D=D, cool=60, start=10)
for ty in (0, 2):
    for P in (1, 106, 200, 301):
        add("W", ty=ty, dmg=100, P=P, D=71, cool=60, start=10)
        add("W", ty=ty, dmg=100, P=P, D=70, cool=60, start=10)
# concurrent / earlier
for P2 in (0, 1, 105, 106):
    for D2 in (69, 70, 71):
        add("W", ty=1, dmg=100, P=50, D=70, cool=60, start=10, n2=1, P2=P2, D2=D2)
add("W", ty=1, dmg=100, P=50, D=70, cool=60, start=10, n2=2, P2=50, D2=10, P3=50, D3=100)
add("W", ty=1, dmg=100, P=50, D=70, cool=60, start=10, n2=2, P2=50, D2=90, P3=50, D3=100)
# flag 변형(urgency +3)
for pos in (0, 1):
    add("W", ty=1, dmg=100, P=50, D=70, cool=60, start=10, pos=pos)
    add("W", ty=1, dmg=100, P=50, D=20, cool=60, start=10, pos=pos)
    add("W", ty=1, dmg=100, P=50, D=10, cool=60, start=10, pos=pos)
    add("W", ty=1, dmg=100, P=400, D=100, cool=29, start=10, pos=pos)
    add("W", ty=1, dmg=100, P=400, D=100, cool=30, start=10, pos=pos)
# 미등재(miss) → 레거시
add("W", ty=1, dmg=100, P=400, D=100, miss=1, thp=300, tmax=400)
# 난수
for lh in (0, 50):
    for ver in (0, 55):
        for seed in (1, 2, 3, 4):
            add("W", ty=1, dmg=100, P=400, D=100, last_hit=lh, version=ver, seed=seed)
            add("W", ty=1, dmg=100, P=50, D=70, last_hit=lh, version=ver, seed=seed)
            add("W", ty=1, dmg=100, P=150, D=100, last_hit=lh, version=ver, seed=seed)

# ── S 구조물 ──
for kind in (0, 1, 2):
    for ty in (1, 2):
        for thp in (100, 101, 1000):
            add("S", kind=kind, ty=ty, dmg=100, thp=thp, tmax=1000)
for tgt in (0, 1, 2, 3):
    for ty in (1, 2):
        for b0 in (0, 1):
            add("S", kind=0, ty=ty, dmg=100, thp=1000, tmax=1000, tgt=tgt, buff0=b0, cdmg=100, ehp=100)
for ehp in (99, 100, 101):
    add("S", kind=0, ty=1, dmg=100, thp=1000, tmax=1000, tgt=2, cdmg=100, ehp=ehp)
# ★타워 자신의 attack_effect 로 판정(IR %958 = t+0x490): tdmg 경계 · 타워 사거리 안 미니언
for tdmg in (99, 100, 101):
    for ty in (1, 2):
        add("S", kind=0, ty=ty, dmg=100, thp=1000, tmax=1000, tgt=2, cdmg=5, ehp=100, tdmg=tdmg)
for inr in (0, 1, 2):
    add("S", kind=0, ty=1, dmg=100, thp=1000, tmax=1000, tgt=2, cdmg=5, ehp=100, tdmg=500, inrange=inr, minions=1)
    add("S", kind=1, ty=1, dmg=100, thp=1000, tmax=1000, tgt=3, cdmg=5, tdmg=500, inrange=inr, minions=1)
add("S", kind=0, ty=1, dmg=100, thp=1000, tmax=1000, tgt=2, cdmg=5, ehp=100, tdmg=500, tnoatk=1)   # 타워 attack_effect None → 패닉 예상
for inr in (0, 1, 2, 3):
    add("S", kind=0, ty=1, dmg=100, thp=1000, tmax=1000, tgt=3, inrange=inr, minions=1)
    add("S", kind=0, ty=2, dmg=100, thp=1000, tmax=1000, tgt=2, cdmg=1000, ehp=100, inrange=inr, minions=1)
add("S", kind=2, ty=1, dmg=100, thp=50, tmax=1000)     # min(200, 200*100/50=400)=200
add("S", kind=2, ty=1, dmg=100, thp=400, tmax=1000)    # 200*100/400 = 50
add("S", kind=0, ty=2, dmg=1000, thp=1000, tmax=1000, tgt=3, inrange=2, minions=1, buff0=1)   # 240 · 240*1000/1000

# ── C 챔피언 ──
for sa in (0, 1, 2, 3, 4):
    for satgt in (0, 1, 2, 3, 4):
        add("C", sa=sa, satgt=satgt)
add("C", cteam=0, cpos=1, sa=1, satgt=0)   # 같은 팀 → bonus 0

res = []
with io.open(OUT, "w", encoding="utf-8") as f:
    for i, (mode, kw) in enumerate(cases):
        args = [EXE, mode] + ["%s=%s" % (k, v) for k, v in kw.items()]
        try:
            p = subprocess.run(args, capture_output=True, text=True, timeout=120)
            out = p.stdout.strip().splitlines()
        except Exception as e:
            out = ["EXC %s" % e]
        last = out[-1] if out else "NOOUT"
        info = [l for l in out if l.startswith("mode=")]
        f.write("#%d %s %s\n  %s\n  %s\n" % (i, mode, " ".join("%s=%s" % kv for kv in kw.items()), info[0] if info else "", last))
        verdict = "MATCH" if "\tMATCH" in last else ("SKIP" if "SKIP" in last else "MISMATCH")
        sync = "rng_sync=true" in last
        res.append((i, mode, kw, verdict, sync, last, info[0] if info else ""))
tot = len(res); m = sum(1 for r in res if r[3] == "MATCH"); sk = sum(1 for r in res if r[3] == "SKIP"); ns = sum(1 for r in res if r[3] == "MATCH" and not r[4])
print("cases=%d MATCH=%d MISMATCH=%d SKIP=%d rng_desync(among MATCH)=%d" % (tot, m, tot - m - sk, sk, ns))
for r in res:
    if r[3] != "MATCH" or not r[4]:
        print("  #%d %s %s -> %s | %s" % (r[0], r[1], r[2], r[5], r[6][:200]))
# 경로 커버리지
from collections import Counter
c = Counter()
for r in res:
    for tag in ("L32", "L37", "L46", "L55", "L72", "L97", "L152", "L159", "L178", "L365", "L393", "L329", "L333", "L359", "CHAMP", "coef=", "cond=true", "cond=false"):
        if tag in r[6]: c[tag] += 1
print("coverage", dict(c))
