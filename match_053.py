# -*- coding: utf-8 -*-
# match_053.py — 카탈로그(_rva_catalog.json)의 전 모드 RVA를 0.5.2 → 0.5.3 으로 구조 매칭.
#   전제: fnindex.py 로 _fnidx_052.pkl / _fnidx_053.pkl 를 먼저 빌드.
#   판정 등급:
#     L1_EXACT   skel(전체 명령 구조) 완전일치 + 신 exe 에서 유일   → 그대로 교체 가능
#     L1_MULTI   skel 일치하나 후보 다중                            → 인접·크기로 좁히거나 ghidra 확정
#     L2_HEAD    앞 24명령 구조 일치 + 유일                         → 본체 일부 변경, 훅 지점은 대개 안전
#     L3_SIM     니모닉 코사인 상위 + 크기 근사                     → 후보 제시(사람/ghidra 확정 필요)
#     NONE       후보 없음                                          → 로직 변경 = ghidra-re 필요
#   mid-func 사이트(byte-patch 등)는 컨테이너 함수를 먼저 매칭하고 함수내 오프셋을 옮긴다.
#   ⚠ 오프셋 이전은 L1_EXACT 컨테이너에서만 신뢰(구조가 같아야 오프셋이 보존됨).
import json, pickle, math, io, sys, collections, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

A = pickle.load(open(r"C:\tfm2mods\_fnidx_052.pkl", "rb"))   # OLD 0.5.2
B = pickle.load(open(r"C:\tfm2mods\_fnidx_053.pkl", "rb"))   # NEW 0.5.3
rows = json.load(open(r"C:\tfm2mods\_rva_catalog.json", encoding="utf-8"))
print(f"index: 0.5.2 {len(A['idx'])}함수 / 0.5.3 {len(B['idx'])}함수")

A_starts = sorted(A["idx"])
B_by_skel = B["by_skel"]
B_by_head = B["by_head"]

# 니모닉 코사인 후보 검색용: 크기 버킷으로 좁힌 뒤 비교 (전수 비교는 132,960 × N 이라 과하다)
B_by_size = collections.defaultdict(list)
for r, v in B["idx"].items():
    B_by_size[v["size"] // 64].append(r)

def container(rva):
    """rva 를 포함하는 0.5.2 함수 시작을 찾는다 (없으면 None)."""
    i = bisect.bisect_right(A_starts, rva) - 1
    if i < 0:
        return None
    s = A_starts[i]
    return s if s <= rva < s + A["idx"][s]["size"] else None

def cos(m1, m2):
    if not m1 or not m2:
        return 0.0
    keys = set(m1) | set(m2)
    dot = sum(m1.get(k, 0) * m2.get(k, 0) for k in keys)
    n1 = math.sqrt(sum(v * v for v in m1.values()))
    n2 = math.sqrt(sum(v * v for v in m2.values()))
    return dot / (n1 * n2) if n1 and n2 else 0.0

def match_fn(old_start):
    """0.5.2 함수 시작 → 0.5.3 후보. (grade, new_rva|None, info)"""
    a = A["idx"].get(old_start)
    if a is None:
        return "NOT_A_FUNCTION", None, {}
    c = B_by_skel.get(a["skel"], [])
    if len(c) == 1:
        return "L1_EXACT", c[0], {"skel": a["skel"][:8]}
    if len(c) > 1:
        return "L1_MULTI", None, {"cands": [hex(x) for x in c[:8]], "n": len(c)}
    h = B_by_head.get(a["head"], [])
    if len(h) == 1:
        return "L2_HEAD", h[0], {"head": a["head"][:8]}
    if len(h) > 1:
        # head 다중 → 그 안에서 니모닉/크기로 최적 1개
        best = sorted(h, key=lambda r: (-cos(a["mnem"], B["idx"][r]["mnem"]),
                                        abs(B["idx"][r]["size"] - a["size"])))
        s = cos(a["mnem"], B["idx"][best[0]]["mnem"])
        return ("L2_HEAD" if s > 0.995 and len(h) <= 4 else "L2_MULTI"), \
               (best[0] if s > 0.995 else None), {"n": len(h), "cos": round(s, 4),
                                                  "cands": [hex(x) for x in best[:5]]}
    # 니모닉 유사도 — 크기 ±50% 버킷만 훑는다
    lo, hi = a["size"] * 0.6, a["size"] * 1.6
    pool = []
    for bk in range(int(lo) // 64, int(hi) // 64 + 1):
        pool += B_by_size.get(bk, [])
    if not pool:
        return "NONE", None, {}
    scored = []
    for r in pool:
        v = B["idx"][r]
        if not (lo <= v["size"] <= hi):
            continue
        scored.append((cos(a["mnem"], v["mnem"]), r))
    if not scored:
        return "NONE", None, {}
    scored.sort(reverse=True)
    top, second = scored[0], (scored[1] if len(scored) > 1 else (0, 0))
    info = {"cos": round(top[0], 4), "second": round(second[0], 4),
            "cands": [hex(r) for _, r in scored[:5]]}
    if top[0] >= 0.995 and top[0] - second[0] >= 0.002:
        return "L3_SIM", top[1], info
    return ("L3_WEAK" if top[0] >= 0.97 else "NONE"), None, info

# 고유 주소로 접기
uniq = collections.OrderedDict()
for r in rows:
    for hx in r["rva"].split(","):
        v = int(hx, 16)
        uniq.setdefault(v, []).append(r)

fn_cache = {}
out = []
for v, refs in sorted(uniq.items()):
    cs = container(v)
    rec = {"old": hex(v), "refs": [{k: rr[k] for k in ("mod", "name", "kind", "file", "line", "note")}
                                   for rr in refs]}
    if cs is None:
        rec.update(grade="NOT_IN_TEXT", new=None)
        out.append(rec); continue
    if cs not in fn_cache:
        fn_cache[cs] = match_fn(cs)
    grade, newfn, info = fn_cache[cs]
    rec["container_old"] = hex(cs)
    rec["offset_in_fn"] = v - cs
    rec["grade"] = grade
    rec["info"] = info
    if newfn is not None:
        rec["container_new"] = hex(newfn)
        if v == cs:
            rec["new"] = hex(newfn)                       # 함수 시작 그 자체
        elif grade == "L1_EXACT":
            rec["new"] = hex(newfn + (v - cs))            # 구조 동일 → 오프셋 보존
            rec["note_"] = "mid-func: L1_EXACT 컨테이너라 오프셋 이전 신뢰"
        else:
            rec["new"] = None
            rec["note_"] = f"mid-func: 컨테이너는 {grade} → 오프셋 재확인 필요"
    else:
        rec["new"] = None
    out.append(rec)

stat = collections.Counter(r["grade"] for r in out)
print("\n=== 등급 집계(고유 주소 %d) ===" % len(out))
for k, c in stat.most_common():
    print(f"  {k:<14} {c}")

json.dump(out, open(r"C:\tfm2mods\_rva_match_053.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)

# 모드별 표
bymod = collections.defaultdict(list)
for r in out:
    for ref in r["refs"]:
        bymod[ref["mod"]].append((ref, r))
with open(r"C:\tfm2mods\_rva_match_053.md", "w", encoding="utf-8") as g:
    g.write("# RVA 구조매칭 결과 0.5.2 → 0.5.3\n\n")
    g.write("> 방식: `.pdata` 함수경계 + 명령 스켈레톤 해시 + 니모닉 코사인. "
            "연속바이트 마스크시그(migrate_rva.py)는 이번 패치에서 전멸해 사용 불가.\n\n")
    g.write("등급: **L1_EXACT**=구조 완전일치·유일(교체 가능) / **L2_HEAD**=앞 24명령 일치 / "
            "**L3_SIM**=니모닉 유사 상위(확정 필요) / **L1_MULTI·L2_MULTI·L3_WEAK**=후보 다중 / "
            "**NONE**=미발견(ghidra-re 필요)\n\n")
    g.write("전체 집계: " + " / ".join(f"{k} {c}" for k, c in stat.most_common()) + "\n")
    for mod in sorted(bymod):
        items = bymod[mod]
        cnt = collections.Counter(r["grade"] for _, r in items)
        done = sum(1 for _, r in items if r.get("new"))
        g.write(f"\n## {mod} — {done}/{len(items)} 해결 · "
                + " / ".join(f"{k} {c}" for k, c in cnt.most_common()) + "\n\n")
        g.write("| 상수 | 0.5.2 | → 0.5.3 | 등급 | 종류 | 위치 |\n|---|---|---|---|---|---|\n")
        for ref, r in sorted(items, key=lambda x: (x[0]["file"], x[0]["line"])):
            new = r.get("new") or (", ".join(r.get("info", {}).get("cands", [])[:2]) or "—")
            g.write(f"| `{ref['name']}` | `{r['old']}` | `{new}` | {r['grade']} | "
                    f"{ref['kind']} | {ref['file']}:{ref['line']} |\n")
print("\n생성: _rva_match_053.json / _rva_match_053.md")
