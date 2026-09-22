"""run199.py — o199.exe 케이스 드라이버(케이스당 프로세스 1개 · TLS 메모 함정 ③). 결과 = o199_cases.log / 요약 stdout
python -X utf8 run199.py"""
import subprocess, io, sys, os
EXE = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o199.exe"
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "o199_cases.log")

cases = []
def add(tagname, **kw): cases.append((tagname, kw))

# ── 기본 · 타워 조준(L207)
add("base")
add("base_v0", version=0)
add("base_cpos1", cpos=1)
add("base_tick1", tick=1)
add("base_tick30000", tick=30000)
for twr in (1, 2): add(f"twr{twr}", jud=100, twr=twr, e0dx=100000, eatk=150000)
add("twr1_flee", jud=100, twr=1, catk=-2, cnoskill=1, e0dx=180000, eatk=150000)   # L207 뒤 L224 도주가 이기는가
# ── L419 in_range / L409 노출 RunAway
for cvis in (0, 1): add(f"inrange_cvis{cvis}", jud=100, e0dx=100000, eatk=150000, cvis=cvis)
for dx in (150000, 150001): add(f"L409_{dx}", jud=100, e0dx=dx, eatk=-2, catk=-2, cnoskill=1, cvis=1)
# ── L355 Trace 경계(d > mr²) · near 반경 160000 경계
for dx in (119999, 120000, 120001, 159999, 160000): add(f"trace_{dx}", jud=100, catk=100000, e0dx=dx, eatk=-2)
add("trace_cvis", jud=100, catk=100000, e0dx=130000, eatk=-2, cvis=1)      # res 비어있지 않음 + 노출 → L409 미실행(중첩) 판별
add("empty_cvis_140k", jud=100, catk=-2, cnoskill=1, e0dx=140000, eatk=-2, cvis=1)   # res 비어있음 + 노출 + 140k → L409 실행
add("empty_cvis_150k", jud=100, catk=-2, cnoskill=1, e0dx=150000, eatk=-2, cvis=1)
add("empty_cvis_150k1", jud=100, catk=-2, cnoskill=1, e0dx=150001, eatk=-2, cvis=1)
add("empty_cvis_v0", jud=80, catk=-2, cnoskill=1, e0dx=140000, eatk=-2, cvis=1, version=0)
add("trace_cvis_v0", jud=80, catk=100000, e0dx=130000, eatk=-2, cvis=1, version=0)
add("trace_cvis_e1", jud=100, catk=100000, e0dx=130000, eatk=-2, cvis=1, e1dx=-140000, e1atk=-2)
# ── L234 도주 경계(dist² <= emr²) · 상태·가시성 조건
for dx in (179999, 180000, 180001): add(f"flee_{dx}", jud=100, catk=-2, cnoskill=1, e0dx=dx, eatk=150000)
for st in (0, 1, 2, 3, 4): add(f"flee_st{st}", jud=100, catk=-2, cnoskill=1, e0dx=180000, eatk=150000, e0st=st)
add("flee_noseen", jud=100, catk=-2, cnoskill=1, e0dx=180000, eatk=150000, seen=0)
add("flee_noevis", jud=100, catk=-2, cnoskill=1, e0dx=180000, eatk=150000, evis=0)
add("flee_noboth", jud=100, catk=-2, cnoskill=1, e0dx=180000, eatk=150000, seen=0, evis=0)
add("flee_e1", jud=100, catk=-2, cnoskill=1, e0dx=900000, e1dx=180000, e1atk=150000)   # 두 번째 슬롯이 도주 원인
add("flee_diag", jud=100, catk=-2, cnoskill=1, e0dx=108000, e0dy=144000, eatk=150000)  # 108k²+144k² = 180k² 정확
add("flee_diag1", jud=100, catk=-2, cnoskill=1, e0dx=108000, e0dy=144001, eatk=150000)
# ── L244 돌진(rush) → get_input → L262/L265
add("rush_base", jud=100, e0rush=1, e0dx=100000, eatk=150000)
add("rush_trace", jud=100, catk=100000, e0dx=130000, eatk=-2, e0rush=1)
add("rush_far", jud=100, e0rush=1)
# ── 두 적(루프 순서 · Trace + in_range)
add("two_enemies", jud=100, catk=100000, e0dx=130000, eatk=-2, e1dx=-100000, e1atk=150000)
add("two_enemies_v0", jud=80, catk=100000, e0dx=130000, eatk=-2, e1dx=-100000, e1atk=150000, version=0)
# ── version 0(jrng None → rnd 소비) · 시드
for seed in (1, 2, 3, 4):
    add(f"v0_trace_s{seed}", jud=80, catk=100000, e0dx=130000, eatk=-2, version=0, seed=seed, cvis=1)
    add(f"v0_flee_s{seed}", jud=80, catk=-2, cnoskill=1, e0dx=150000, eatk=150000, version=0, seed=seed)
    add(f"v0_inrange_s{seed}", jud=80, e0dx=100000, eatk=150000, version=0, seed=seed, cvis=1)
# ── 정확도 중간값(롤 무작위 · version 2 = NoiseRng 결정적)
for jud in (0, 50, 80):
    add(f"jud{jud}_flee", jud=jud, catk=-2, cnoskill=1, e0dx=160000, eatk=150000)
    add(f"jud{jud}_trace", jud=jud, catk=100000, e0dx=125000, eatk=-2)

def run():
    out = io.open(OUT, "w", encoding="utf-8")
    n = 0; match = 0; mism = []; rnddiff = []; crash = []
    for name, kw in cases:
        args = [EXE] + [f"{k}={v}" for k, v in kw.items()]
        try:
            r = subprocess.run(args, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=120)
            txt = r.stdout + r.stderr
        except Exception as e:
            txt = f"EXC {e}"
        out.write(f"### {name} {' '.join(args[1:])}\n{txt}\n")
        n += 1
        res = [l for l in txt.splitlines() if l.startswith("result\t")]
        tls = [l for l in txt.splitlines() if l.startswith("tls_after")]
        if not res:
            crash.append(name); print(f"CRASH {name}: {txt.strip().splitlines()[-1:] }"); continue
        v = res[0].split("\t")[1]
        short = res[0].split("\t")
        gm = short[2] + " " + short[3]
        if v == "MATCH": match += 1
        elif v == "MATCH_RND_DIFF": rnddiff.append(name)
        else: mism.append(name)
        print(f"{v:14} {name:18} {gm:40} {tls[0].split(chr(9))[1] if tls else ''}")
    out.close()
    print(f"\ncases={n} MATCH={match} RND_DIFF={rnddiff} MISMATCH={mism} CRASH={crash}")

if __name__ == "__main__":
    run()
