# -*- coding: utf-8 -*-
# match2_053.py — 구조매칭 + 국소 앵커 투표로 정밀도를 올린 2차 매처.
#   1차(match_053.py)의 한계: 함수 본체까지 재생성돼 skel 완전일치가 7.6% 뿐 → L3 후보 다중.
#   보강: 0.5.2 에서 타겟 주변 ±K 함수 중 "양쪽 유일 skel" 앵커들이 0.5.3 어디로 갔는지 투표시켜
#         타겟이 있을 구역을 좁힌 뒤, 그 구역 안에서 니모닉 유사도 1위를 고른다.
#         (링커가 블록 단위로 재배치했으므로 전역 단조성은 없지만 국소 인접성은 보존된다 —
#          실측 근거: CONDGATE 0x21338d0→0xc550b0 과 MOVEPRI 0x2134240→0xc559e0 이 쌍으로 이동)
import json, pickle, math, io, sys, collections, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

A = pickle.load(open(r"C:\tfm2mods\_fnidx_052.pkl", "rb"))
B = pickle.load(open(r"C:\tfm2mods\_fnidx_053.pkl", "rb"))
rows = json.load(open(r"C:\tfm2mods\_rva_catalog.json", encoding="utf-8"))

EXCLUDE_MODS = {"tfm2_transfer_tweak"}      # 유저 지시 2026-07-29: 불필요 → 대응 제외
K_NEIGHBORS = 40                            # 앵커 투표에 쓸 이웃 함수 수(양쪽 각각)
CLUSTER_WIN = 0x60000                       # 투표 클러스터 폭 (384KB)

A_starts = sorted(A["idx"])
B_starts = sorted(B["idx"])
au = {k: v[0] for k, v in A["by_skel"].items() if len(v) == 1}
bu = {k: v[0] for k, v in B["by_skel"].items() if len(v) == 1}
ANCHOR = {}                                  # 0.5.2 rva -> 0.5.3 rva (양쪽 유일 skel)
for k in set(au) & set(bu):
    ANCHOR[au[k]] = bu[k]
anchor_starts = sorted(ANCHOR)
print(f"앵커 {len(ANCHOR)}개 / 0.5.2 {len(A_starts)}함수 / 0.5.3 {len(B_starts)}함수")

B_by_size = collections.defaultdict(list)
for r, v in B["idx"].items():
    B_by_size[v["size"] // 64].append(r)

def container(rva):
    i = bisect.bisect_right(A_starts, rva) - 1
    if i < 0:
        return None
    s = A_starts[i]
    return s if s <= rva < s + A["idx"][s]["size"] else None

def cos(m1, m2):
    if not m1 or not m2:
        return 0.0
    dot = sum(v * m2.get(k, 0) for k, v in m1.items())
    n1 = math.sqrt(sum(v * v for v in m1.values()))
    n2 = math.sqrt(sum(v * v for v in m2.values()))
    return dot / (n1 * n2) if n1 and n2 else 0.0

def anchor_zone(old_start):
    """타겟 주변 앵커들이 0.5.3 에서 몰려 있는 구역 [lo, hi] 목록을 신뢰도 순으로."""
    i = bisect.bisect_left(anchor_starts, old_start)
    near = anchor_starts[max(0, i - K_NEIGHBORS): i + K_NEIGHBORS]
    if not near:
        return []
    tgt = sorted(ANCHOR[a] for a in near)
    # 슬라이딩 윈도로 최다 밀집 구간들을 뽑는다
    zones = []
    j = 0
    for i2 in range(len(tgt)):
        while tgt[i2] - tgt[j] > CLUSTER_WIN:
            j += 1
        zones.append((i2 - j + 1, tgt[j], tgt[i2]))
    zones.sort(reverse=True)
    out, used = [], []
    for cnt, lo, hi in zones:
        if any(not (hi < l or lo > h) for l, h in used):
            continue
        used.append((lo, hi))
        out.append((cnt, lo, hi))
        if len(out) >= 3:
            break
    return out

def match_fn(old_start):
    a = A["idx"].get(old_start)
    if a is None:
        return "NOT_A_FUNCTION", None, {}
    # 1) skel 완전일치 유일
    c = B["by_skel"].get(a["skel"], [])
    if len(c) == 1:
        return "L1_EXACT", c[0], {}
    zones = anchor_zone(old_start)
    def in_zone(r):
        for cnt, lo, hi in zones:
            if lo - CLUSTER_WIN <= r <= hi + CLUSTER_WIN:
                return cnt
        return 0
    # 2) skel 다중 → 앵커 구역으로 가린다
    if len(c) > 1:
        z = [r for r in c if in_zone(r)]
        if len(z) == 1:
            return "L1_ZONE", z[0], {"n": len(c)}
        return "L1_MULTI", None, {"n": len(c), "cands": [hex(x) for x in c[:6]]}
    # 3) head 일치
    h = B["by_head"].get(a["head"], [])
    if len(h) == 1:
        return "L2_HEAD", h[0], {}
    if len(h) > 1:
        z = [r for r in h if in_zone(r)]
        pool = z if z else h
        best = sorted(pool, key=lambda r: (-cos(a["mnem"], B["idx"][r]["mnem"]),
                                           abs(B["idx"][r]["size"] - a["size"])))
        s = cos(a["mnem"], B["idx"][best[0]]["mnem"])
        if len(z) == 1:
            return "L2_ZONE", z[0], {"n": len(h)}
        if s > 0.995:
            return "L2_HEAD", best[0], {"n": len(h), "cos": round(s, 4)}
        return "L2_MULTI", None, {"n": len(h), "cands": [hex(x) for x in best[:5]]}
    # 4) 니모닉 유사도 (앵커 구역 우선)
    lo_s, hi_s = a["size"] * 0.6, a["size"] * 1.7
    pool = []
    for bk in range(int(lo_s) // 64, int(hi_s) // 64 + 1):
        pool += B_by_size.get(bk, [])
    scored = []
    for r in pool:
        v = B["idx"][r]
        if not (lo_s <= v["size"] <= hi_s):
            continue
        sc = cos(a["mnem"], v["mnem"])
        if sc < 0.9:
            continue
        # 0.5.3 함수는 0.5.2 대비 대체로 소폭 커진다(재컴파일). 실측 타겟들이 0.95~1.12 대역이라
        # 그 밖은 감점만 하고 배제는 하지 않는다(판단 근거를 남기기 위해).
        ratio = v["size"] / a["size"]
        fit = 1.0 if 0.90 <= ratio <= 1.20 else 0.0
        scored.append((sc, fit, in_zone(r), r, ratio))
    if not scored:
        return "NONE", None, {}
    # ★유사도가 1순위. 앵커 구역과 크기비는 동점(0.002 이내)일 때만 가르는 타이브레이커.
    #   (앵커에 가산점을 주면 cos 0.9995 정답이 cos 0.9969 오답에 밀리는 사고가 난다 — 실측)
    scored.sort(key=lambda x: (-round(x[0], 3), -x[1], -x[2], -x[0]))
    top = scored[0]
    close = [x for x in scored if top[0] - x[0] <= 0.002]
    second = scored[1] if len(scored) > 1 else (0, 0, 0, 0, 0)
    info = {"cos": round(top[0], 4), "second": round(second[0], 4), "zone": top[2],
            "ratio": round(top[4], 3), "nclose": len(close),
            "cands": [f"{hex(r)}(cos{sc:.4f},z{z},r{rt:.2f})" for sc, ft, z, r, rt in scored[:5]]}
    if len(close) == 1 and top[0] >= 0.99 and top[1]:
        return ("L3_ZONE" if top[2] else "L3_SIM"), top[3], info
    if len(close) > 1:
        # 근접 후보 다수 → 앵커 지지가 유일하게 있는 놈이면 채택, 아니면 사람이 확정
        z = [x for x in close if x[2] and x[1]]
        if len(z) == 1 and z[0][0] >= 0.99:
            return "L3_ZONE", z[0][3], info
        return "L3_AMBIG", None, info
    return ("L3_WEAK" if top[0] >= 0.97 else "NONE"), None, info

uniq = collections.OrderedDict()
for r in rows:
    if r["mod"] in EXCLUDE_MODS:
        continue
    for hx in r["rva"].split(","):
        uniq.setdefault(int(hx, 16), []).append(r)

fn_cache, out = {}, []
for n_done, (v, refs) in enumerate(sorted(uniq.items()), 1):
    cs = container(v)
    rec = {"old": hex(v), "refs": [{k: rr[k] for k in ("mod", "name", "kind", "file", "line", "note")}
                                   for rr in refs]}
    if cs is None:
        rec.update(grade="NOT_IN_TEXT", new=None)
        out.append(rec)
        continue
    if cs not in fn_cache:
        fn_cache[cs] = match_fn(cs)
    grade, newfn, info = fn_cache[cs]
    rec.update(container_old=hex(cs), offset_in_fn=v - cs, grade=grade, info=info)
    if newfn is not None:
        rec["container_new"] = hex(newfn)
        if v == cs:
            rec["new"] = hex(newfn)
        elif grade == "L1_EXACT":
            rec["new"] = hex(newfn + (v - cs))
            rec["mid"] = "L1_EXACT 컨테이너 = 오프셋 보존 신뢰"
        else:
            rec["new"] = None
            rec["mid"] = f"컨테이너 {grade} → 함수내 오프셋 재도출 필요"
    else:
        rec["new"] = None
    out.append(rec)
    if n_done % 50 == 0:
        print(f"  ... {n_done}/{len(uniq)}", flush=True)

stat = collections.Counter(r["grade"] for r in out)
print("\n=== 등급 집계(고유 주소 %d) ===" % len(out))
for k, c in stat.most_common():
    print(f"  {k:<12} {c}")
solved = sum(1 for r in out if r.get("new"))
print(f"  해결 {solved}/{len(out)} ({solved/len(out)*100:.0f}%)")

json.dump(out, open(r"C:\tfm2mods\_rva_match2_053.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)
print("생성: _rva_match2_053.json")
