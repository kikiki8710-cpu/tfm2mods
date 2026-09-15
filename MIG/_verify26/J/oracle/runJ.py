# -*- coding: utf-8 -*-
"""runJ — 26차 배치J 오라클 드라이버. 케이스당 프로세스 1개(TLS 메모 격리). 결과 = oJ_cases.log"""
import subprocess, sys, io, os
EXE = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\oJ.exe"
LOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), "oJ_cases.log")
cases = []
def C(name, *kv): cases.append((name, list(kv)))
# ---- 233 nexus_final_stand_uncached: (nexus?, twin empty?, enemy champ in range?)
C("F0 기본(쌍둥이 2 존재)", "fn=final")
C("F1 twin 비움·적챔프 멀리", "fn=final", "twin=1")
C("F2 twin 비움·적챔프0 넥서스 좌표·평타 range100000", "fn=final", "twin=1", "ex=0", "eatk=50", "erng=100000")
C("F3 twin 비움·적챔프0 넥서스+150000·range100000(사거리 밖)", "fn=final", "twin=1", "ex=0", "dx=150000", "eatk=50", "erng=100000")
C("F4 twin 비움·적챔프0 넥서스+150000·range200000(사거리 안)", "fn=final", "twin=1", "ex=0", "dx=150000", "eatk=50", "erng=200000")
C("F5 twin 비움·적챔프 넥서스 좌표·attack_effect None", "fn=final", "twin=1", "ex=0", "eatk=-2")
C("F6 twin 존재·적챔프 넥서스 좌표·range100000(쌍둥이 게이트)", "fn=final", "ex=0", "eatk=50", "erng=100000")
C("F7 nexus None·twin 비움·적챔프 넥서스 좌표", "fn=final", "twin=1", "nexus=0", "ex=0", "eatk=50", "erng=100000")
C("F8 팀1 관점·twin 비움·적(팀0)챔프 넥서스 좌표", "fn=final", "team=1", "twin=1", "ex=0", "eatk=50", "erng=100000")
C("F9 F2 와 같은데 cached 래퍼(nexus_final_stand)", "fn=final", "twin=1", "ex=0", "eatk=50", "erng=100000", "cached=1")
C("F10 twin 비움·적챔프4(Support) 만 넥서스 좌표", "fn=final", "twin=1", "ex=4", "eatk=50", "erng=100000")
# ---- 232 v30
C("V0 기본(라인전·attack e0·원위치)", "fn=v30", "fst=100000", "act=attack", "tgt=e0", "ehp=10000")
C("V1 near 적탑타워 +20000·attack e0", "fn=v30", "fst=100000", "act=attack", "tgt=e0", "ehp=10000", "near=1", "dx=20000")
C("V2 near·attack 대상=아군 a1(hp 10000)", "fn=v30", "fst=100000", "act=attack", "tgt=a1", "thp=10000", "near=1", "dx=20000")
C("V3 near·attack 대상=적 넥서스(비챔프·hp 10000)", "fn=v30", "fst=100000", "act=attack", "tgt=n1", "thp=10000", "near=1", "dx=20000")
C("V4 near·attack·대상 hp 1(한방)", "fn=v30", "fst=100000", "act=attack", "tgt=e0", "ehp=1", "near=1", "dx=20000")
C("V5 near·attack·라인전 아님(fst=1000)", "fn=v30", "fst=1000", "act=attack", "tgt=e0", "ehp=10000", "near=1", "dx=20000")
C("V6 near·attack·tower_attack_disable_tick=0", "fn=v30", "fst=100000", "tad=0", "act=attack", "tgt=e0", "ehp=10000", "near=1", "dx=20000")
C("V7 near·around e0", "fn=v30", "fst=100000", "act=around", "tgt=e0", "ehp=10000", "near=1", "dx=20000")
C("V8 near·attack·내 attack_effect None", "fn=v30", "fst=100000", "act=attack", "tgt=e0", "ehp=10000", "near=1", "dx=20000", "atk=0")
C("V9 near·skill·skill_effect Some", "fn=v30", "fst=100000", "act=skill", "tgt=e0", "ehp=10000", "near=1", "dx=20000", "skl=1")
C("V10 near·skill·skill_effect None(skl=0)", "fn=v30", "fst=100000", "act=skill", "tgt=e0", "ehp=10000", "near=1", "dx=20000", "skl=0")
C("V10b near·skill·skill_effect default(Swordman)", "fn=v30", "fst=100000", "act=skill", "tgt=e0", "ehp=10000", "near=1", "dx=20000")
C("V11b near·skill2·level 3(skill2 default)", "fn=v30", "fst=100000", "act=skill2", "tgt=e0", "ehp=10000", "near=1", "dx=20000", "lv=3")
C("V16 near·attack·라인전 아님(tick 100000 ≥ fst)", "fn=v30", "fst=100000", "tick=100000", "act=attack", "tgt=e0", "ehp=10000", "near=1", "dx=20000")
C("V11 near·skill2·level 1", "fn=v30", "fst=100000", "act=skill2", "tgt=e0", "ehp=10000", "near=1", "dx=20000")
C("V12 near·attack·대상 없음(bad id)", "fn=v30", "fst=100000", "act=attack", "tgt=bad", "near=1", "dx=20000")
C("V13 near dx=400000(타워 사거리 밖)", "fn=v30", "fst=100000", "act=attack", "tgt=e0", "ehp=10000", "near=1", "dx=400000")
C("V14 tick=0·fst=1800(경계: 0<usub(1800,1800)=0 거짓)", "fn=v30", "fst=1800", "tick=0", "act=attack", "tgt=e0", "ehp=10000", "near=1", "dx=20000")
C("V15 tick=0·fst=1801(경계: 0<1 참)", "fn=v30", "fst=1801", "tick=0", "act=attack", "tgt=e0", "ehp=10000", "near=1", "dx=20000")
# ---- 230/231 score
for plan in ("safe", "wait"):
    P = plan[0].upper()
    C(f"S{P}0 attack bad target · game", "fn=score", f"plan={plan}", "act=attack", "tgt=bad", "what=game")
    C(f"S{P}0 attack bad target · mine", "fn=score", f"plan={plan}", "act=attack", "tgt=bad", "what=mine")
    C(f"S{P}1 attack e0 · game", "fn=score", f"plan={plan}", "act=attack", "tgt=e0", "what=game")
    C(f"S{P}1 attack e0 · mine", "fn=score", f"plan={plan}", "act=attack", "tgt=e0", "what=mine")
    C(f"S{P}2 skill bad target · game", "fn=score", f"plan={plan}", "act=skill", "tgt=bad", "what=game", "skl=1")
    C(f"S{P}2 skill bad target · mine", "fn=score", f"plan={plan}", "act=skill", "tgt=bad", "what=mine", "skl=1")
    C(f"S{P}3 skill2 e0 level1(unwrap 패닉 예상)", "fn=score", f"plan={plan}", "act=skill2", "tgt=e0", "what=game")
    C(f"S{P}4 around n0(아군 넥서스) · game", "fn=score", f"plan={plan}", "act=around", "tgt=n0", "what=game")
    C(f"S{P}4 around n0 · eval", "fn=score", f"plan={plan}", "act=around", "tgt=n0", "what=eval")
    C(f"S{P}4 around n0 · mine", "fn=score", f"plan={plan}", "act=around", "tgt=n0", "what=mine")
    C(f"S{P}5 around t0(아군 탑타워) · game", "fn=score", f"plan={plan}", "act=around", "tgt=t0", "what=game")
    C(f"S{P}5 around t0 · eval", "fn=score", f"plan={plan}", "act=around", "tgt=t0", "what=eval")
    C(f"S{P}5 around t0 · mine", "fn=score", f"plan={plan}", "act=around", "tgt=t0", "what=mine")
    C(f"S{P}6 around bad · game", "fn=score", f"plan={plan}", "act=around", "tgt=bad", "what=game")
    C(f"S{P}6 around bad · eval", "fn=score", f"plan={plan}", "act=around", "tgt=bad", "what=eval")
    C(f"S{P}7 attack e0 line=Mid · game", "fn=score", f"plan={plan}", "act=attack", "tgt=e0", "what=game", "line=1")
# ---- 228 hna
C("H0 기본", "fn=hna")
C("H1 적 쌍둥이 비움", "fn=hna", "twin=1")
C("H2 적 쌍둥이 비움·전방미니언 Top=a0 Mid=a2 Bot=a3", "fn=hna", "twin=1", "fm=a0,a2,a3")
C("H3 적 쌍둥이 비움·전방미니언 Mid=a2 만", "fn=hna", "twin=1", "fm=-,a2,-")
C("H4 tick 30000·적 쌍둥이 비움", "fn=hna", "twin=1", "tick=30000")
only = sys.argv[1:]
with io.open(LOG, "a", encoding="utf-8") as f:
    for name, kv in cases:
        if only and not any(name.startswith(o) for o in only): continue
        try:
            r = subprocess.run([EXE] + kv, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=120)
            out = r.stdout.strip().split("\n")
            res = [l for l in out if l.startswith("RESULT") or l.startswith("pre") or l.startswith("mine") or l.startswith("tower") or l.startswith("line_exists") or l.startswith("champ_eff")]
            err = r.stderr.strip().split("\n")[:3] if r.returncode != 0 else []
            line = f"[{name}] {' '.join(kv)}\n    " + "\n    ".join(res) + (f"\n    EXIT={r.returncode} " + " | ".join(err) if err else "")
        except Exception as e:
            line = f"[{name}] EXC {e}"
        print(line); f.write(line + "\n")
