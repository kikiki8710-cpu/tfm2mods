# 없앤 두 판단층(포탑 회피 · 전력 비교)을 지금 있는 원본 키로 근사해 A/B 프리셋에 얹는다.
#  ⚠값 대응은 1:1이 아니다 — 아래 EST 의 근거를 주석으로 같이 적어 둔다.
import io, os, sys, shutil
sys.stdout.reconfigure(encoding="utf-8")
CD = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\config"

# (키, 값, 원본, 근거)
EST = [
    # ── 포탑 회피 ← 8차 tower_threat=80 · tower_range=140000 · ally_tower_range=150000 ──
    ("sc_turret_radius", "150000", "150000", "8차 ally_tower_range=150000 과 이미 일치 — 원본 유지"),
    ("sc_dive_margin",    "25000",  "15000", "★추정. 포탑 사거리에 더해 '다이브 위험'으로 보는 여유. 8차가 포탑을 꺼린 만큼 넓힘"),
    ("mv_tower_cap",        "130",    "100", "★추정. 포탑 유무가 점수를 좌우하는 상한. tower_threat=80(약간 불리해도 뺀다)의 강도"),
    ("pe_tower_margin",   "24000",  "18000", "★추정. 자리 평가에서도 포탑을 조금 더 멀리서 의식"),
    ("ae_tower_shift",        "5",      "6", "★추정·가장 거친 값. 시프트라 한 단계가 곧 2배다. 되돌릴 땐 6"),
    # ── 전력 비교 ← 8차 numbers_threat=80 · numbers_range=150000 · min_enemy=1 ──
    ("sc_ally_radius",   "150000", "150000", "8차 numbers_range=150000 과 이미 일치 — 원본 유지 (이 키가 '적' 반경)"),
    ("sc_enemy_radius",  "150000", "100000", "8차는 아군도 같은 150000 반경으로 셌다. 원본은 아군만 100000이라 상황을 불리하게 본다"),
    ("pe_count_radius",  "150000", "120000", "고립 판정 반경도 8차 눈금(150000)에 맞춤"),
    ("mv0_adv_hi",          "340",    "300", "★추정. 적이 2명 이상 많을 때 도주 배율. numbers_threat=80(한타 신중)"),
    ("mv0_adv_p1",          "230",    "200", "★추정. 적이 1명 많을 때"),
    ("sc_adv_0",             "75",     "80", "★추정. 동수에서 덜 달라붙음"),
    ("sc_adv_m1",            "55",     "60", "★추정. 1명 열세에서 덜 달라붙음"),
]

HDR = ["", "# " + "=" * 62,
       "#  없앤 판단층의 근사 — 포탑 회피 · 전력 비교",
       "#   8차가 쓰던 tower_* / numbers_* 는 지금 모드에 없다(원본 순수화로 제거).",
       "#   그 '의도'를 게임 원본 키로 옮긴 값이다. 값 대응은 1:1이 아니라 추정이 섞여 있다.",
       "#   ⚠8차가 실제로 겪은 동작이 아니다 — 당시 그 층은 오프셋 오류로 거의 발화하지 않았다.",
       "#     여기 값은 '8차가 의도한 바'를 지금 구조로 옮긴 것이다.",
       "#   되돌리려면 각 줄 끝의 (원본 N) 값을 넣으면 된다.",
       "# " + "=" * 62]

def apply(fname):
    p = os.path.join(CD, fname)
    shutil.copyfile(p, p + ".bak_pre_est")
    lines = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
    done, out = set(), []
    for ln in lines:
        t = ln.strip()
        if t and not t.startswith("#") and "=" in t:
            k = t.split("=", 1)[0].strip()
            hit = next((e for e in EST if e[0] == k), None)
            if hit:
                out.append("%s = %s   # 원본 %s · %s" % (k, hit[1], hit[2], hit[3]))
                done.add(k); continue
        out.append(ln)
    add = [e for e in EST if e[0] not in done]
    if add:
        out += HDR
        for k, v, o, why in add:
            out.append("%s = %s   # 원본 %s · %s" % (k, v, o, why))
    txt = "\n".join(out)
    if not txt.endswith("\n"): txt += "\n"
    io.open(p, "w", encoding="utf-8", newline="\n").write(txt)
    return len(done), len(add)

def load(p):
    d = {}
    for ln in io.open(os.path.join(CD, p), encoding="utf-8", errors="replace"):
        t = ln.strip()
        if not t or t.startswith("#") or "=" not in t: continue
        k, v = t.split("=", 1)
        d[k.strip()] = v.split("#")[0].strip()
    return d

for f in ["테스트A.cfg", "테스트B.cfg"]:
    edited, added = apply(f)
    d = load(f)
    ok = all(d.get(k) == v for k, v, _, _ in EST)
    print("%-12s 기존줄 교체 %2d · 새로 추가 %2d · 총 %3d키 · 값 확인 %s"
          % (f, edited, added, len(d), "OK" if ok else "★실패"))

print("\n적용한 값")
for k, v, o, why in EST:
    mark = "  " if v == o else "→ "
    print("  %s%-18s %8s (원본 %s)  %s" % (mark, k, v, o, why))
