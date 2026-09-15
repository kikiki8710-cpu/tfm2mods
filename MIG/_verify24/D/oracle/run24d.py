"""run24d — o24d.exe 케이스 드라이버(케이스당 프로세스 1개). 결과 = o24d_<case>.log + 요약 표 stdout."""
import subprocess, io, os, sys
EXE = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o24d.exe"
OUT = os.path.dirname(os.path.abspath(__file__))
# argv 순서: kind cx cy hp maxhp e0dx e0dy seen0 e1dx e1dy seen1 vis tick tgt dive twocall ally1 ovrisk ovrpt e0dmg cdmg ovap ovad ovupb evis push
D = dict(cx=480000, cy=480000, hp=1000, maxhp=2000, e0dx=100000, e0dy=0, seen0=1, e1dx=900000, e1dy=0, seen1=1, vis=1, tick=1000,
         tgt=0, dive=0, twocall=0, ally1=900000, ovrisk=400, ovrpt=300, e0dmg=-1, cdmg=-1, ovap=-1, ovad=-1, ovupb=-1, evis=1, push=1, tdmg=-1, msp=-1)
ORDER = ["cx","cy","hp","maxhp","e0dx","e0dy","seen0","e1dx","e1dy","seen1","vis","tick","tgt","dive","twocall","ally1","ovrisk","ovrpt","e0dmg","cdmg","ovap","ovad","ovupb","evis","push","tdmg","msp"]
CASES = {
 "R1_base_twocall": ("runaway", dict(twocall=1)),
 "R2_threat": ("runaway", dict(e0dmg=300, cdmg=200)),
 "R3_threat_hp500": ("runaway", dict(e0dmg=300, cdmg=200, hp=500)),
 "R3b_threat_hp1999": ("runaway", dict(e0dmg=300, cdmg=200, hp=1999)),
 "R4_two_enemies": ("runaway", dict(e1dx=-90000)),
 "R4b_two_enemies_ally": ("runaway", dict(e1dx=-90000, ally1=50000)),
 "R4c_no_seen": ("runaway", dict(seen0=0)),
 "R4d_seen_far149999": ("runaway", dict(e0dx=149999, e0dy=0)),
 "R4e_seen_150000": ("runaway", dict(e0dx=150000, e0dy=0)),
 "R4f_seen_150001": ("runaway", dict(e0dx=150001, e0dy=0)),
 "R5_applyed_ge_hp": ("runaway", dict(ovad=1000)),
 "R5b_applyed_999": ("runaway", dict(ovad=999)),
 "R6_fountain": ("runaway", dict(cx=30000, cy=930000)),
 "R7_enemy_tower_novis": ("runaway", dict(cx=600000, cy=380000, vis=0)),
 "R7b_enemy_tower_far_novis": ("runaway", dict(cx=700000, cy=520000, vis=0)),
 "R8_novis_notower": ("runaway", dict(vis=0)),
 "R9_ally_tower": ("runaway", dict(cx=380000, cy=600000)),
 "R10_tower_dist_149999": ("runaway", dict(cx=368000, cy=592000+149999, vis=0)),
 "R10b_tower_dist_150000": ("runaway", dict(cx=368000, cy=592000+150000, vis=0)),
 "T1_base": ("trace", dict(e0dmg=300, cdmg=200)),
 "T1b_base_ms1000": ("trace", dict(e0dmg=300, cdmg=200, msp=1000)),
 "T1c_ms1000_hp500": ("trace", dict(e0dmg=300, cdmg=200, msp=1000, hp=500)),
 "T8_enemy_tower_tdmg": ("trace", dict(cx=600000, cy=380000, e0dx=-20000, e0dmg=300, cdmg=200, tdmg=400, msp=1000)),
 "T8b_enemy_tower_tdmg_hp500": ("trace", dict(cx=600000, cy=380000, e0dx=-20000, e0dmg=300, cdmg=200, tdmg=400, msp=1000, hp=500)),
 "T9_ally_tower_tdmg": ("trace", dict(cx=380000, cy=600000, e0dx=30000, e0dmg=300, cdmg=200, tdmg=400, msp=1000)),
 "R11_enemy_tower_tdmg": ("runaway", dict(cx=600000, cy=380000, e0dmg=300, cdmg=200, tdmg=400)),
 "R11b_enemy_tower_tdmg_hp500": ("runaway", dict(cx=600000, cy=380000, e0dmg=300, cdmg=200, tdmg=400, hp=500)),
 "R11c_enemy_tower_tdmg_novis": ("runaway", dict(cx=600000, cy=380000, e0dmg=300, cdmg=200, tdmg=400, vis=0)),
 "R11d_enemy_tower_tdmg_risk3000": ("runaway", dict(cx=600000, cy=380000, e0dmg=300, cdmg=200, tdmg=400, ovrisk=3000, ovrpt=2000)),
 "R11e_risk3000_hp1999": ("runaway", dict(cx=600000, cy=380000, e0dmg=300, cdmg=200, tdmg=400, ovrisk=3000, ovrpt=2000, hp=1999)),
 "A4_ally_tower_tdmg": ("around", dict(cx=380000, cy=600000, e0dx=30000, e0dmg=300, cdmg=200, tdmg=400)),
 "T2_dive": ("trace", dict(e0dmg=300, cdmg=200, dive=1)),
 "T3_ally_tower": ("trace", dict(cx=380000, cy=600000, e0dx=30000, e0dmg=300, cdmg=200)),
 "T4_enemy_tower": ("trace", dict(cx=600000, cy=380000, e0dx=-20000, e0dmg=300, cdmg=200)),
 "T5_far_target": ("trace", dict(e0dx=140000, e1dx=-100000, e0dmg=300, cdmg=200)),
 "T6_hp500": ("trace", dict(e0dmg=300, cdmg=200, hp=500)),
 "T7_two_enemies_ally": ("trace", dict(e1dx=-90000, ally1=50000, e0dmg=300, cdmg=200)),
 "A1_base": ("around", dict(e0dmg=300, cdmg=200)),
 "A2_ally_tower": ("around", dict(cx=380000, cy=600000, e0dx=30000, e0dmg=300, cdmg=200)),
 "A3_noseen": ("around", dict(seen0=0, e0dmg=300, cdmg=200)),
 "AT1_attack": ("attack", dict(e0dmg=300, cdmg=200)),
}
sel = sys.argv[1:] or list(CASES)
rows = []
for name in sel:
    kind, ov = CASES[name]
    args = dict(D); args.update(ov)
    argv = [EXE, kind] + [str(args[k]) for k in ORDER]
    p = subprocess.run(argv, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=120)
    txt = p.stdout + p.stderr
    io.open(os.path.join(OUT, f"o24d_{name}.log"), "w", encoding="utf-8").write(" ".join(argv[1:]) + "\n" + txt)
    res = [l for l in txt.splitlines() if l.startswith("RESULT183")]
    ic = [l for l in txt.splitlines() if l.startswith("ICTX")]
    two = [l for l in txt.splitlines() if l.startswith("TWOCALL")]
    r = res[0].split("\t") if res else ["RESULT183", "rc=%d" % p.returncode, txt[-300:]]
    summary = "\t".join(r[1:10]) if res else " ".join(r[1:])
    icok = ic[0].split("\t")[-1] if ic else "?"
    rows.append((name, summary, icok, " | ".join(two)))
    print(name, "|", summary, "| ICTX", icok, ("| " + " | ".join(two)) if two else "")
