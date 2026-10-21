"""상세전술 후보 중 '관측이 덜 된' 손잡이를 골라 관측 방법을 붙인다 (2026-10-21).

입력: tactic_coverage.json(후보↔손잡이 지점), site_report sites.json, gate_analysis gates.json, function_categories.csv
손잡이 지점(즉치 = 손잡이 값 ±1, 거리는 제곱 포함)을 상태별로 나눈다.
  ok    = 경계확정·양쪽·(값 기록) 도달
  dbg   = 미도달인데 막는 분기가 ctx 디버그 플래그(+0x39/+0x3b/+0x3c)·싱글라인 → 로그용 사본, 관측 불필요
  one   = 비교가 한쪽만 (손잡이 문턱 반대편이 안 나옴)
  deep  = 미도달, 이 지점 쪽 분기는 실행됐는데 더 아래(지점 없는 블록)에서 막힘
  untr  = 미도달, 막는 분기가 미계측(추적 안 됨·CFG 밖)
  gate1 = 미도달, 한쪽만 비교가 막음(조건 정체 RE 필요)
  fn    = 함수가 분석 범위 밖이거나 통째 미도달
방법: M1 손잡이 특정 · M2 블록 입구 계측 · M3 상황 만들기 · M4 막는 조건 RE
사용: python -B -X utf8 tools/observation_plan.py <measured_summary 폴더> <site_report 폴더> <gates 폴더>
"""
import collections, csv, json, os, re, sys

ms, rep, gd = sys.argv[1:4]
cat = {r["f"]: r for r in csv.DictReader(open(os.path.join(ms, "function_categories.csv"), encoding="utf-8-sig"))}
rows = json.load(open(os.path.join(ms, "tactic_coverage.json"), encoding="utf-8"))
S = {x["rva"]: x for x in json.load(open(os.path.join(rep, "sites.json"), encoding="utf-8"))["sites"]}
G = json.load(open(os.path.join(gd, "gates.json"), encoding="utf-8"))
gate = {x["gate_site"]: x for x in G["gates"]}
why = {r["site"]: r for r in G["sites"]}
DEBUG = re.compile(r"\+ 0x3[9bc]\]|0xf8\], 0")


def imm_ok(k):
    s = S[k[1]]
    if s["T"] is None:
        return s["kind"] == "VALUE"
    return any(abs(abs(int(s["T"])) - v) <= 1 for v in k[4])


def state(k):
    s = S[k[1]]
    if s["stage"] in ("경계확정", "양쪽", "도달"):
        return "ok"
    if s["stage"] == "한쪽만":
        return "one"
    r = why.get(k[1])
    if not r:
        return "fn"
    g = gate.get(r.get("gate")) if r.get("gate") else None
    if g and (DEBUG.search(g["insn"] or "") or g["gate_site"] in ("0xd3ffb0", "0xdc5a4b")):
        return "dbg"
    w = r["why"]
    return "gate1" if w.startswith("한쪽만") else "deep" if w.startswith("더 아래") else "untr" if w.startswith(("추적", "CFG")) else "dbg" if w.startswith("panic") else "fn"


METHOD = {"one": "M3 상황 만들기", "deep": "M2 블록 입구 계측", "untr": "M2 블록 입구 계측", "gate1": "M4 막는 조건 RE", "fn": "M4 막는 조건 RE"}
plan = []
for x in rows:
    ks = [k for k in x["knob"] if imm_ok(k)]
    st = collections.Counter(state(k) for k in ks)
    bad = [k for k in ks if state(k) in METHOD]
    if ks and not bad:
        continue
    methods = sorted({METHOD[state(k)] for k in bad}) if ks else ["M1 손잡이 특정"]
    tier = 1 if not ks or st["ok"] == 0 else (2 if x["stage"] in ("A", "A~B") else 3)
    detail = []
    for k in bad[:6]:
        s = S[k[1]]; r = why.get(k[1]) or {}
        g = gate.get(r.get("gate")) if r.get("gate") else None
        d = f"`{k[1]}` `{k[2][:30]}` {state(k)}"
        if state(k) == "one":
            d += f" 관측 {s['lhs_min']}..{s['lhs_max']}"
        if g and state(k) in ("gate1",):
            d += f" ← `{g['gate_site']}` `{(g['insn'] or '')[:30]}` 안 나온 쪽 {g.get('never')}"
        detail.append(d)
    plan.append(dict(tier=tier, stage=x["stage"], name=x["name"], section=x["section"], st=dict(st), methods=methods,
                     fns=" · ".join(x["fns"][:5]), detail=detail,
                     bad_sites=[{"rva": k[1], "function": S[k[1]]["function"], "state": state(k)} for k in bad]))
pri = {"A": 0, "A~B": 1, "B": 2, "B~C": 3, "C": 4}
plan.sort(key=lambda p: (p["tier"], pri.get(p["stage"], 5), p["name"]))
json.dump(plan, open(os.path.join(ms, "observation_plan.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
mc = collections.Counter(m for p in plan for m in p["methods"])
L = ["# 상세전술 — 관측이 덜 된 손잡이와 관측 방법 (0.6.4 · 2~6회차 기준)", "",
     "> 대상 = 기획 v5 후보 중 손잡이(근거 문장 수치 = 즉치 ±1, 거리는 제곱 포함) 지점이 아직 덜 관측된 것. 디버그 사본(ctx+0x39/+0x3b/+0x3c·싱글라인에 막힌 같은 상수)·panic 은 관측 불필요로 제외.",
     "> 상태: ok 측정됨 · one 한쪽만 · deep 더 아래에서 막힘 · untr 막는 분기 미계측 · gate1 한쪽만 비교가 막음 · fn 함수 밖/통째 미도달 · dbg 디버그 사본.", "",
     "## 관측 방법", "",
     "| 방법 | 무엇을 | 어떻게 | 비용 |", "|---|---|---|---|",
     "| **M1 손잡이 특정** | 손잡이가 동적 문턱·계산값이라 수치로 지점을 못 찾은 후보 | 기획 근거 문장 → RE 로 손잡이 비교 명령 RVA 확정 → `extra_v054.txt` 에 추가 → 일반 MOBA 회차 | RE + 1회차 |",
     "| **M2 블록 입구 계측** | 손잡이 지점이 미도달인데 막는 곳이 지점 없는 블록 | 마지막 실행 블록 → 손잡이 지점까지 지배자 사슬의 각 블록 첫 명령에 값 기록 지점 자동 추가(도달 수로 흐름이 끊기는 블록 특정) → 그 블록의 분기만 M4 | 도구 1개 + 1회차 |",
     "| **M3 상황 만들기** | 비교가 한쪽만(대개 체력이 문턱 아래로 안 내려감) | ① 표본 늘리기(배경 경기·장기 시즌) ② 불리한 매치업(약팀으로 강팀 상대·레벨 차) ③ 팀 전술 공격적·다이브 ④ 값 비트맵/범위로 문턱 근처 빈도 확인 | 플레이 시간 |",
     "| **M4 막는 조건 RE** | 한쪽만 비교가 막거나 함수 통째 미도달 | gate 비교의 값 정체·생산자 RE(지금까지 한 방식) → 일반 플레이 가능/불가 판정 | RE |",
     "", "| 방법 | 후보 수 |", "|---|---:|"] + [f"| {m} | {n} |" for m, n in sorted(mc.items())] + [""]
TIER = {1: "1순위 — 손잡이가 한 번도 측정 안 됨(또는 손잡이 미특정)", 2: "2순위 — A·A~B 후보, 주 사본은 측정됐지만 일부 사본·반대편 미측정",
        3: "3순위 — B 이하 후보, 일부 사본·반대편 미측정"}
for t in (1, 2, 3):
    pp = [p for p in plan if p["tier"] == t]
    L += [f"## {TIER[t]} ({len(pp)})", "", "| 후보(단계) | 방법 | 손잡이 상태 | 근거 함수(범주) | 덜 된 지점 |", "|---|---|---|---|---|"]
    for p in pp:
        L.append(f"| {p['name']} ({p['stage']}) | {' · '.join(m.split()[0] for m in p['methods'])} | {p['st'] or '손잡이 미특정'} | {p['fns']} | {'<br>'.join(p['detail'][:4])} |")
    L.append("")
open(os.path.join(ms, "observation_plan.md"), "w", encoding="utf-8", newline="\n").write("\n".join(L) + "\n")
print(len(plan), dict(mc), collections.Counter(p["tier"] for p in plan))
