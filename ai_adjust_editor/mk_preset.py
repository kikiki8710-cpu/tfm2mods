# 14차 위에 8차의 라인전 값만 덮어쓴 프리셋 생성.
#  ★값이 있는 줄만 갈아끼우고 주석·순서·나머지 키는 14차 그대로 둔다(diff 최소화).
import io, os, sys
sys.stdout.reconfigure(encoding="utf-8")

CD   = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\config"
BASE = "AI개선모드 14차.cfg"
FROM = "8차.cfg"
OUT  = "14차+8차라인전.cfg"

# 8차에서 가져올 라인전 키 (현재 모드에 살아 있는 것만)
TAKE = ["dd_frontier_mult", "dd_cover_count", "dd_ratio_thr", "dd_lane_margin",
        "dd_near_dist", "dd_main_near_dist"]

def load(p):
    d = {}
    for ln in io.open(os.path.join(CD, p), encoding="utf-8", errors="replace"):
        t = ln.strip()
        if not t or t.startswith("#") or "=" not in t: continue
        k, v = t.split("=", 1); d[k.strip()] = v.strip()
    return d

src = load(FROM)
cur = load(os.path.join(CD, "default.txt")) if os.path.exists(os.path.join(CD, "default.txt")) else {}

lines = io.open(os.path.join(CD, BASE), encoding="utf-8", errors="replace").read().split("\n")
done, out = set(), []
for ln in lines:
    t = ln.strip()
    if t and not t.startswith("#") and "=" in t:
        k = t.split("=", 1)[0].strip()
        if k in TAKE and k in src:
            sep = " = " if " = " in ln else "="
            out.append("%s%s%s" % (k, sep, src[k])); done.add(k); continue
    out.append(ln)

miss = [k for k in TAKE if k not in done]
if miss:  # 14차에 그 줄이 없으면 끝에 덧붙인다
    out.append("")
    out.append("# ── 8차 라인전 값 (14차에 없던 키) ──")
    for k in miss:
        if k in src: out.append("%s = %s" % (k, src[k]))

hdr = ["# ══════════════════════════════════════════════════════════",
       "#  14차 + 8차 라인전",
       "#   기준 = AI개선모드 14차",
       "#   덮어쓴 값 = 8차의 라인전 설정 6개 (아래)",
       "#     dd_frontier_mult / dd_cover_count / dd_ratio_thr",
       "#     dd_lane_margin / dd_near_dist / dd_main_near_dist",
       "#   그 밖은 14차 그대로. 다른 계열(귀환·위협·넥서스)은 손대지 않았다.",
       "# ══════════════════════════════════════════════════════════"]
txt = "\n".join(hdr + out)
if not txt.endswith("\n"): txt += "\n"
io.open(os.path.join(CD, OUT), "w", encoding="utf-8", newline="\n").write(txt)

# 검증
res = load(OUT)
print("생성: %s  (키 %d개)" % (OUT, len(res)))
print("\n%-20s %10s %10s %10s" % ("키", "14차", "8차", "결과"))
b14 = load(BASE)
for k in TAKE:
    print("%-20s %10s %10s %10s %s" % (k, b14.get(k, "—"), src.get(k, "—"), res.get(k, "—"),
                                       "OK" if res.get(k) == src.get(k) else "★불일치"))
other = [k for k in set(b14) & set(res) if b14[k] != res[k] and k not in TAKE]
print("\n의도치 않게 바뀐 키: %d개 %s" % (len(other), " ".join(sorted(other))))
dead = [k for k in res if cur and k not in cur]
print("현재 모드에 없는 키(무시됨): %d개" % len(dead))
if dead: print("  " + " ".join(sorted(dead)))
