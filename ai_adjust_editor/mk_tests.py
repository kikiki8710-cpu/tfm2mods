# 테스트용 프리셋 2종 생성 — 둘 다 "지금 모드에 없는 키"는 제거한다.
#   테스트A = 14차 + 8차 라인전   (14차의 나머지 튜닝을 그대로 유지)
#   테스트B = 현재 기본값 + 8차 라인전 (신규 설정값을 전부 살린 상태)
import io, os, sys, re
sys.stdout.reconfigure(encoding="utf-8")

CD = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\config"
LANE = ["dd_frontier_mult", "dd_cover_count", "dd_ratio_thr", "dd_lane_margin",
        "dd_near_dist", "dd_main_near_dist"]

def load(p):
    d = {}
    for ln in io.open(os.path.join(CD, p), encoding="utf-8", errors="replace"):
        t = ln.strip()
        if not t or t.startswith("#") or "=" not in t: continue
        k, v = t.split("=", 1); d[k.strip()] = v.strip()
    return d

CUR  = load("default.txt")          # 지금 모드가 아는 키 = 이 목록이 기준
EIGHT = load("8차.cfg")
LANE_V = {k: EIGHT[k] for k in LANE if k in EIGHT}

def build(base_file, out_name, header):
    """base_file 의 줄 순서·주석을 유지하되
       ① 지금 모드에 없는 키 줄은 버리고 ② 라인전 6개는 8차 값으로 교체."""
    lines = io.open(os.path.join(CD, base_file), encoding="utf-8", errors="replace").read().split("\n")
    out, done, dropped = [], set(), []
    for ln in lines:
        t = ln.strip()
        if t and not t.startswith("#") and "=" in t:
            k = t.split("=", 1)[0].strip()
            if k not in CUR:                      # 지금 모드가 모르는 키 = 제거
                dropped.append(k); continue
            if k in LANE_V:
                sep = " = " if " = " in ln else "="
                out.append("%s%s%s" % (k, sep, LANE_V[k])); done.add(k); continue
        out.append(ln)
    miss = [k for k in LANE_V if k not in done]
    if miss:
        out.append(""); out.append("# ── 8차 라인전 값 ──")
        for k in miss: out.append("%s = %s" % (k, LANE_V[k]))
    # 주석 3줄 이상 연속 공백 정리
    txt = "\n".join(header + out)
    txt = re.sub(r'\n{4,}', "\n\n\n", txt)
    if not txt.endswith("\n"): txt += "\n"
    io.open(os.path.join(CD, out_name), "w", encoding="utf-8", newline="\n").write(txt)
    return dropped

HDR = lambda title, base, note: [
    "# " + "=" * 58,
    "#  " + title,
    "#   기준       = " + base,
    "#   덮어쓴 값  = 8차의 라인전 6개",
    "#                dd_frontier_mult 22 / dd_cover_count 2 / dd_ratio_thr 51",
    "#                dd_lane_margin 1200 / dd_near_dist·dd_main_near_dist 8차값",
    "#   " + note,
    "#   ※ 지금 모드에 없는 키는 전부 제거했습니다.",
    "# " + "=" * 58, ""]

dA = build("AI개선모드 14차.cfg", "테스트A.cfg",
           HDR("테스트A — 14차 + 8차 라인전", "AI개선모드 14차",
               "그 밖의 항목은 14차 튜닝 그대로(신규 설정값은 대부분 없음 = 게임 원본으로 동작)"))
dB = build("default.txt", "테스트B.cfg",
           HDR("테스트B — 기본값 + 8차 라인전", "현재 기본값(default.txt)",
               "라인전 외에는 전부 현재 기본값 = 신규 설정값을 전부 살린 상태"))

for name, dropped in [("테스트A.cfg", dA), ("테스트B.cfg", dB)]:
    d = load(name)
    ok = all(d.get(k) == LANE_V[k] for k in LANE_V)
    unknown = [k for k in d if k not in CUR]
    missing = sorted(set(CUR) - set(d))
    print("%-12s 키 %3d개 · 제거 %3d개 · 라인전 반영 %s · 모르는 키 %d개 · 기본값 대비 빠진 키 %d개"
          % (name, len(d), len(dropped), "OK" if ok else "★실패", len(unknown), len(missing)))

print("\n라인전 최종값:", " ".join("%s=%s" % (k, v) for k, v in LANE_V.items()))
