# check_applied.py 의 EXPECT 표를 **검증기에서 자동 산출**한다.
#  손으로 적으면 그룹 간 배분이 어긋나 "미적용 0건인데 FAIL"이 뜬다(2026-08-04 실사고).
#  1차(verify_deep.py)와 2차(gen_wave2.py)의 체크 목록을 그대로 세어 쓴다.
import re, io, sys, json, os
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))

exp = {}

# ── 1차: verify_deep.py 의 C 리스트를 그대로 실행해 라벨 접두로 그룹 분류 ──
g = {}
src = io.open(os.path.join(HERE, "verify_deep.py"), encoding="utf-8").read()
src = src.split("okc, fails")[0] + "\nRESULT = C\n"
ns = {}
exec(compile(src, "verify_deep", "exec"), ns)
PFX = [("mv2_", "move2"), ("bv_", "bv"), ("ae_", "ae"), ("th_", "th"),
       ("rt_", "rt"), ("jg_", "rt")]
for lab, rva, cands, w, want in ns["RESULT"]:
    for p, grp in PFX:
        if lab.startswith(p):
            g[grp] = g.get(grp, 0) + 1; break
    else:
        raise SystemExit("분류 실패: " + lab)
exp.update(g)

# ── 2차: wave2_sites.json ──
d = json.load(io.open(os.path.join(HERE, "wave2_sites.json"), encoding="utf-8"))
g2 = {}
for k, v in d.items():
    n = len(v["sites"]) + len(v.get("sites_p1", []))
    g2[v["grp"]] = g2.get(v["grp"], 0) + n
# eh_band_low_cmp 는 코드에서 eh_band_low 로 흡수(+1 사이트) — 그룹은 그대로 eh
exp.update(g2)

print("자동 산출 EXPECT:")
for k in sorted(exp): print("  %-8s %d" % (k, exp[k]))
print("  합계 %d" % sum(exp.values()))

if "--write" in sys.argv:
    P = os.path.join(HERE, "check_applied.py")
    s = io.open(P, encoding="utf-8").read()
    i = s.index("    # ── 2026-08-04 배선분")
    j = s.index("}", i)
    block = ("    # ── 2026-08-04 배선분 — gen_expect.py 가 검증기에서 자동 산출(손으로 고치지 말 것) ──\n"
             + "".join('    "%s_imm.txt": %d,\n' % (k, exp[k]) for k in sorted(exp)))
    s = s[:i] + block + s[j:]
    io.open(P, "w", encoding="utf-8").write(s)
    print("\ncheck_applied.py EXPECT 갱신 완료")
