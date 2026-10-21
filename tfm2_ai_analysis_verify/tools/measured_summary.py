"""측정된 함수·계수·임계점 정리 (2026-10-21).

analyze_sites.py 의 claims.json·sites.json(여러 회차 합산)에서 "실제로 확인된 것"만 모아 정리한다.
  - 분류별·영역별·함수별 실행/양쪽/경계확정/값 대조 집계
  - 의미 이름이 붙은 임계점(threshold)·계수(coefficient)·열거(enum) 전량 표
  - 불일치(값 '다름', 경계 ±1) 목록
사용: python -B -X utf8 tools/measured_summary.py <site_report 폴더> <출력 폴더>
"""
import collections, csv, json, os, sys

ATLAS = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust\function_atlas_064_20261018"
src, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
C = json.load(open(os.path.join(src, "claims.json"), encoding="utf-8"))["claims"]
S = {s["id"]: s for s in json.load(open(os.path.join(src, "sites.json"), encoding="utf-8"))["sites"]}
fname = {}
for r in csv.DictReader(open(os.path.join(ATLAS, "functions.csv"), encoding="utf-8-sig")):
    fname[r["RVA(0.6.4)"]] = r["의미ID"]
C = [c for c in C if c["class"] != "extra"]  # v0.5.3 추가 지점은 분석 기록이 아니다
# 같은 기록이 같은 지점 묶음으로 여러 번 들어 있는 경우(아틀라스 행 중복)는 하나로
_seen = set(); _u = []
for c in C:
    k = (c["record"], tuple(sorted(c["site_ids"])))
    if k not in _seen:
        _seen.add(k); _u.append(c)
DUP = len(C) - len(_u); C = _u

EXEC = ("도달", "한쪽만", "양쪽", "경계확정")
VCMP = ("양쪽", "경계확정")


def vc(c):
    v = set(c["value_check"])
    return "다름" if "다름" in v else ("같음" if "같음" in v else ("±1" if v else "-"))


def bc(c):
    k = [b["check"] for b in c["boundary_check"]]
    return "±1 어긋남" if "경계 어긋남(±1)" in k else ("일치" if "일치" in k else "-")


def obs(c):
    lo = [S[i]["lhs_min"] for i in c["site_ids"] if S[i]["lhs_min"] is not None]
    hi = [S[i]["lhs_max"] for i in c["site_ids"] if S[i]["lhs_max"] is not None]
    return (min(lo), max(hi)) if lo else (None, None)


def hits(c):
    return sum(S[i]["match_hits"] + S[i]["menu_hits"] for i in c["site_ids"])


def esc(t):
    return str(t).replace("|", "/").replace("\n", " ")


L = ["# 측정된 함수·계수·임계점 정리 (0.6.4 · 2~6회차 합산)", "",
     f"> 원본 = `{os.path.relpath(src, out)}`(analyze_sites.py 합산). 분석 기록(claim) {len(C):,}개(같은 기록·같은 지점 중복 {DUP:,}개 제거) · 함수 {len({c['function'] for c in C})}개.",
     "> 단계: **도달** = 값 기록 지점이 실행됨 / **한쪽만** = 비교가 한쪽 결과만 / **양쪽** = 비교 참·거짓 모두 / **경계확정** = T-1·T·T+1 모두 관측(임계점 자체를 실측으로 확인).",
     "> 값 대조: 분석 서술의 수치가 실제 명령의 즉치·상수와 같은가. 경계 대조: 서술의 부등호(포함/미포함)가 명령의 분기와 같은가.", ""]

# 1. 총괄
st = collections.Counter(c["stage"] for c in C)
ex = sum(st[k] for k in EXEC)
L += ["## 1. 총괄", "",
      "| 항목 | 수 |", "|---|---:|",
      f"| 분석 기록 | {len(C):,} |",
      f"| 실행 확인(도달 이상) | {ex:,} ({ex / len(C):.1%}) |",
      f"| 비교 양쪽 관측 | {st['양쪽'] + st['경계확정']:,} (그중 경계확정 {st['경계확정']:,}) |",
      f"| 비교 한쪽만 | {st['한쪽만']:,} |",
      f"| 미도달 | {st['미도달']:,} |"]
v = collections.Counter(vc(c) for c in C)
b = collections.Counter(bc(c) for c in C)
L += [f"| 값 대조 같음 / ±1(표기) / 다름 | {v['같음']:,} / {v['±1']:,} / {v['다름']:,} |",
      f"| 경계 대조 일치 / ±1 어긋남 | {b['일치']:,} / {b['±1 어긋남']:,} |", ""]

# 2. 분류별
L += ["## 2. 분류별", "", "| 분류 | 기록 | 실행 | 양쪽 | 경계확정 | 값 같음 | 값 다름 | 경계 일치 | 경계 ±1 |", "|---|---:|---:|---:|---:|---:|---:|---:|---:|"]
for k, n in collections.Counter(c["class"] for c in C).most_common():
    g = [c for c in C if c["class"] == k]
    L.append(f"| {k} | {n:,} | {sum(c['stage'] in EXEC for c in g):,} | {sum(c['stage'] == '양쪽' for c in g):,} | {sum(c['stage'] == '경계확정' for c in g):,} | "
             f"{sum(vc(c) == '같음' for c in g):,} | {sum(vc(c) == '다름' for c in g):,} | {sum(bc(c) == '일치' for c in g):,} | {sum(bc(c) == '±1 어긋남' for c in g):,} |")
L.append("")

# 3. 영역별
L += ["## 3. 영역별", "", "| 영역 | 함수 | 기록 | 실행률 | 양쪽+경계확정 | 경계확정 | 값 다름 |", "|---|---:|---:|---:|---:|---:|---:|"]
areas = collections.defaultdict(list)
for c in C:
    areas[c["area"]].append(c)
for a, g in sorted(areas.items(), key=lambda kv: -len(kv[1])):
    e = sum(c["stage"] in EXEC for c in g)
    L.append(f"| {a} | {len({c['function'] for c in g})} | {len(g):,} | {e / len(g):.0%} | {sum(c['stage'] in VCMP for c in g):,} | {sum(c['stage'] == '경계확정' for c in g):,} | {sum(vc(c) == '다름' for c in g):,} |")
L.append("")

# 4. 함수별
funcs = collections.defaultdict(list)
for c in C:
    funcs[c["function"]].append(c)
rows = []
for f, g in funcs.items():
    n = len(g); e = sum(c["stage"] in EXEC for c in g)
    rows.append(dict(f=f, name=fname.get(f, ""), area=g[0]["area"], n=n, e=e, both=sum(c["stage"] in VCMP for c in g),
                     edge=sum(c["stage"] == "경계확정" for c in g), one=sum(c["stage"] == "한쪽만" for c in g),
                     un=sum(c["stage"] == "미도달" for c in g), vs=sum(vc(c) == "같음" for c in g), vd=sum(vc(c) == "다름" for c in g),
                     hits=sum(hits(c) for c in g)))
with open(os.path.join(out, "functions.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(rows[0])); w.writeheader(); w.writerows(sorted(rows, key=lambda r: (-r["e"] / r["n"], -r["n"])))
full = [r for r in rows if r["un"] == 0]
none = [r for r in rows if r["e"] == 0]
L += ["## 4. 함수별", "",
      f"- 함수 {len(rows)}개 중 **전 기록 실행 {len(full)}개** · 일부 실행 {len(rows) - len(full) - len(none)}개 · 전부 미도달 {len(none)}개. 전량 = `functions.csv`.",
      "", "### 4-1. 경계확정이 많은 함수 상위 40", "",
      "| 함수 | 의미 | 영역 | 기록 | 실행 | 양쪽 | 경계확정 | 값 다름 |", "|---|---|---|---:|---:|---:|---:|---:|"]
for r in sorted(rows, key=lambda r: (-r["edge"], -r["both"]))[:40]:
    L.append(f"| `{r['f']}` | {esc(r['name'])[:60]} | {r['area']} | {r['n']} | {r['e']} | {r['both']} | {r['edge']} | {r['vd']} |")
L += ["", "### 4-2. 전 기록이 실행되고 비교가 모두 양쪽/경계확정인 함수 (완전 관측)", "",
      "| 함수 | 의미 | 영역 | 기록 | 경계확정 |", "|---|---|---|---:|---:|"]
comp = []
for f, g in funcs.items():
    cmp_ = [c for c in g if any(S[i]["kind"] != "VALUE" for i in c["site_ids"])]
    if g and all(c["stage"] in EXEC for c in g) and cmp_ and all(c["stage"] in VCMP for c in cmp_):
        comp.append((f, g))
for f, g in sorted(comp, key=lambda x: -len(x[1])):
    L.append(f"| `{f}` | {esc(fname.get(f, ''))[:60]} | {g[0]['area']} | {len(g)} | {sum(c['stage'] == '경계확정' for c in g)} |")
L.append(f"\n({len(comp)}개)\n")

# 5. 이름 붙은 임계점·계수·열거
def table(title, klass):
    g = [c for c in C if c["class"] == klass]
    g.sort(key=lambda c: (["경계확정", "양쪽", "한쪽만", "도달", "미도달"].index(c["stage"]), c["area"], c["semantic"]))
    T = [f"### {title} ({len(g)})", "", "| 의미 | 함수 | 값 | 단계 | 관측 범위 | 적중 | 값 대조 | 경계 대조 | 서술 |", "|---|---|---|---|---|---:|---|---|---|"]
    shown = set()
    for c in g:
        lo, hi = obs(c)
        key = (c["semantic"], c["function"], str(c["values"]), c["stage"], lo, hi)
        if key in shown:
            continue
        shown.add(key)
        T.append(f"| {esc(c['semantic'])} | `{c['function']}` | {esc(c['values'])[:40]} | {c['stage']} | {'' if lo is None else f'{lo}..{hi}'} | {hits(c):,} | {vc(c)} | {bc(c)} | {esc(c['meaning'])[:90]} |")
    return T + [""]


L += ["## 5. 의미 이름이 붙은 임계점·계수·열거 (전량)", "",
      "분석 아틀라스가 이름을 붙인 수치(threshold·coefficient·enum·dynamic_input)만 모았다. 정렬 = 경계확정 → 양쪽 → 한쪽만 → 도달 → 미도달.", ""]
L += table("임계점 threshold", "threshold") + table("계수 coefficient", "coefficient") + table("열거 enum", "enum") + table("동적 입력 dynamic_input", "dynamic_input")

# 6. 불일치
L += ["## 6. 불일치", "", "### 6-1. 경계 대조 ±1 어긋남 (서술의 포함/미포함이 명령과 1 차이 — 서술 오기 후보)", "",
      "| 의미 | 함수 | 명령 | 명령 경계 | 지점 수 | 단계 | 서술 |", "|---|---|---|---|---:|---|---|"]
grp = collections.OrderedDict()
for c in C:
    for bb in c["boundary_check"]:
        if bb["check"] == "경계 어긋남(±1)" and c["semantic"]:
            k = (c["semantic"], c["function"], bb["insn"].split(",")[-1].strip(), bb["rel"], bb["T"])
            e = grp.setdefault(k, {"sites": set(), "stages": collections.Counter(), "insn": bb["insn"], "meaning": c["meaning"]})
            if bb["site"] not in e["sites"]:
                e["sites"].add(bb["site"]); e["stages"][bb["stage"]] += 1
for (sem, f, _, rel, T), e in grp.items():
    stg = " · ".join(f"{k} {v}" for k, v in e["stages"].most_common())
    L.append(f"| {esc(sem)} | `{f}` | `{e['insn']}` | {rel} {T} | {len(e['sites'])} | {stg} | {esc(e['meaning'])[:100]} |")
dv = [c for c in C if vc(c) == "다름"]
L += ["", f"### 6-2. 값 대조 '다름' ({len(dv)}) — 영역별", "", "| 영역 | 수 | 그중 실행 확인 |", "|---|---:|---:|"]
for a, n in collections.Counter(c["area"] for c in dv).most_common():
    L.append(f"| {a} | {n} | {sum(c['stage'] in EXEC for c in dv if c['area'] == a)} |")
L.append("\n전량 = `value_mismatch.csv`.\n")
with open(os.path.join(out, "value_mismatch.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.writer(h); w.writerow(["record", "semantic", "class", "function", "area", "values", "stage", "meaning"])
    for c in dv:
        w.writerow([c["record"], c["semantic"], c["class"], c["function"], c["area"], c["values"], c["stage"], c["meaning"]])
with open(os.path.join(out, "edge_confirmed.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.writer(h); w.writerow(["record", "semantic", "class", "function", "function_name", "area", "values", "obs_min", "obs_max", "hits", "value_check", "boundary_check", "meaning"])
    for c in C:
        if c["stage"] == "경계확정":
            lo, hi = obs(c)
            w.writerow([c["record"], c["semantic"], c["class"], c["function"], fname.get(c["function"], ""), c["area"], c["values"], lo, hi, hits(c), vc(c), bc(c), c["meaning"]])
open(os.path.join(out, "README.md"), "w", encoding="utf-8", newline="\n").write("\n".join(L) + "\n")

# ── 함수별 관측 범주 (지점 = 명령 단위). 추가 지점(extra) 만 있는 명령은 뺀다.
CATS = [
    ("A", "임계점 전부 실측 확인", "모든 지점 실행 · 비교 지점이 전부 경계확정(T-1·T·T+1)"),
    ("B", "비교 전부 양쪽 관측", "모든 지점 실행 · 비교 지점이 전부 양쪽 이상(일부는 경계 미확정)"),
    ("C", "값 기록만 · 전부 실행", "비교 지점 없음 · 값 기록 지점 전부 실행"),
    ("D", "전부 실행 · 일부 비교 한쪽만", "모든 지점 실행 · 한쪽 결과만 본 비교가 있음"),
    ("E", "대부분 실행(90% 이상)", "미도달 지점이 있으나 실행 비율 ≥ 90%"),
    ("F", "절반 이상 실행(50~90%)", "실행 비율 50% 이상 90% 미만"),
    ("G", "절반 미만 실행", "실행 비율 < 50%"),
    ("H", "전부 미도달", "한 지점도 실행되지 않음"),
]
SITES_ALL = json.load(open(os.path.join(src, "sites.json"), encoding="utf-8"))["sites"]
byfn = collections.defaultdict(list)
for s in SITES_ALL:
    if [r for r in s.get("records", []) if r != "extra"]:  # 기록이 extra 뿐 = v0.5.3 추가 지점
        byfn[s["function"]].append(s)
cat_rows = []
for f, ss in byfn.items():
    cmp_ = [s for s in ss if s["kind"] != "VALUE"]
    st_ = collections.Counter(s["stage"] for s in ss)
    n = len(ss); ex_ = n - st_["미도달"]
    if ex_ == 0:
        k = "H"
    elif st_["미도달"]:
        k = "E" if ex_ / n >= 0.9 else ("F" if ex_ / n >= 0.5 else "G")
    elif not cmp_:
        k = "C"
    elif all(s["stage"] == "경계확정" for s in cmp_):
        k = "A"
    elif all(s["stage"] in VCMP for s in cmp_):
        k = "B"
    else:
        k = "D"
    cat_rows.append(dict(cat=k, f=f, name=fname.get(f, ""), area=ss[0]["area"], sites=n, cmp=len(cmp_), value=n - len(cmp_),
                         edge=st_["경계확정"], both=st_["양쪽"], one=st_["한쪽만"], reach=st_["도달"], un=st_["미도달"],
                         exec_pct=round(100 * ex_ / n), hits=sum(s["match_hits"] + s["menu_hits"] for s in ss)))
order = {c[0]: i for i, c in enumerate(CATS)}
cat_rows.sort(key=lambda r: (order[r["cat"]], r["area"], -r["sites"]))
with open(os.path.join(out, "function_categories.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(cat_rows[0])); w.writeheader(); w.writerows(cat_rows)
cnt = collections.Counter(r["cat"] for r in cat_rows)
M = ["# 관측한 함수 — 관측 종류별 목록 (0.6.4 · 2~6회차 합산)", "",
     "> 단위 = 측정 지점(명령). 비교 지점 = cmp/test(임계점), 값 기록 지점 = 계수·상수 로드 등. 추가 지점(v0.5.3 extra)만 있는 명령은 제외.",
     "> 경계확정 = T-1·T·T+1 모두 관측 / 양쪽 = 참·거짓 모두 / 한쪽만 = 한 결과만 / 도달 = 값 기록 지점 실행 / 미도달 = 실행 안 됨.", "",
     "| 범주 | 이름 | 기준 | 함수 | 지점 |", "|---|---|---|---:|---:|"]
for k, nm, desc in CATS:
    M.append(f"| {k} | {nm} | {desc} | {cnt[k]} | {sum(r['sites'] for r in cat_rows if r['cat'] == k):,} |")
M += ["", f"합계 함수 {len(cat_rows)} · 지점 {sum(r['sites'] for r in cat_rows):,}. 표 = `function_categories.csv`.", ""]
# 영역 × 범주
areas_ = sorted({r["area"] for r in cat_rows})
M += ["## 영역 × 범주 (함수 수)", "", "| 영역 | " + " | ".join(c[0] for c in CATS) + " | 합 |", "|---|" + "---:|" * (len(CATS) + 1)]
for a in areas_:
    rr = [r for r in cat_rows if r["area"] == a]
    M.append(f"| {a} | " + " | ".join(str(sum(r['cat'] == c[0] for r in rr)) for c in CATS) + f" | {len(rr)} |")
M.append("")
for k, nm, desc in CATS:
    rr = [r for r in cat_rows if r["cat"] == k]
    M += [f"## {k}. {nm} ({len(rr)})", "", f"기준: {desc}", "",
          "| 함수 | 의미 | 영역 | 지점 | 비교 | 경계확정 | 양쪽 | 한쪽만 | 값 도달 | 미도달 | 실행% | 적중 |",
          "|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for r in rr:
        M.append(f"| `{r['f']}` | {esc(r['name'])[:70]} | {r['area']} | {r['sites']} | {r['cmp']} | {r['edge']} | {r['both']} | {r['one']} | {r['reach']} | {r['un']} | {r['exec_pct']} | {r['hits']:,} |")
    M.append("")
open(os.path.join(out, "function_categories.md"), "w", encoding="utf-8", newline="\n").write("\n".join(M) + "\n")
print(dict(cnt))
print(len(C), ex, st["경계확정"], len(rows), len(full), len(comp))
