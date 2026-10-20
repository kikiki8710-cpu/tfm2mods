"""임계점 트레이서 판독기 (v0.4.0).

sites_<run_id>.jsonl(모드가 남긴 누적 카운터)의 마지막 스냅샷과 tools/sites/site_catalog.json 을 합쳐
분석 기록(주장)마다 "실제 게임에서 그 비교에 들어온 값"을 판정한다.

판정 단계(경기 장면 = scene 4/5 의 메인+배경 스레드 합계 기준):
  미도달      경기 중 한 번도 실행되지 않음 (틀렸다는 뜻 아님 — 그 상황이 안 나왔다)
  한쪽만      임계점의 한쪽 값만 들어옴 (조건이 늘 참이거나 늘 거짓)
  양쪽        아래·위 둘 다 들어옴 → 이 비교가 실제로 갈림
  경계확정    T-1, T, T+1 이 모두 들어옴 → 포함/미포함까지 실측으로 확정 (CMP 즉치만)
값 대조(정적): 기록값이 실제 비교 상수 T 와 같은지. T±1 이면 포함/미포함 표기 차이로 본다(같은 뜻일 수 있음).

사용: python -B -X utf8 tools/analyze_sites.py <sites_<run>.jsonl> [--out DIR]
"""
import argparse, collections, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REL = {1: "<", 2: ">=", 3: "<=", 4: ">", 5: "==", 6: "!=", 0: None}
STAGES = ["미도달", "한쪽만", "양쪽", "경계확정"]


def decode_key(key, size, signed):
    if signed == "s":
        v = key ^ (1 << 63)
        return v - (1 << 64) if v >= 1 << 63 else v
    return key


def imm_value(site):
    op = site["ops"][1]
    if not op or op[0] != "imm":
        return None
    v, size = op[1]["value"], op[1]["size"]
    m = (1 << (8 * size)) - 1
    v &= m
    if site["signed"] == "s" and v >> (8 * size - 1):
        v -= 1 << (8 * size)
    return v


def lhs_size(site):
    op = site["ops"][0]
    return op[1]["size"] if op else 8


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("jsonl")
    ap.add_argument("--catalog", default=os.path.join(HERE, "sites", "site_catalog.json"))
    ap.add_argument("--out", default=None)
    a = ap.parse_args()
    lines = [l for l in open(a.jsonl, encoding="utf-8") if l.strip()]
    if not lines:
        sys.exit("스냅샷 없음")
    snap = json.loads(lines[-1])
    cat = json.load(open(a.catalog, encoding="utf-8"))
    sites = {s["id"]: s for s in cat["sites"]}
    obs = {o["id"]: o for o in snap["sites"]}

    site_rows = []
    for sid, s in sites.items():
        o = obs.get(sid)
        c = o["c"] if o else [[0] * 10] * 3
        m = [c[0][k] + c[1][k] for k in range(8)]  # 경기 장면(메인+배경)
        hits, below, equal, above, tm1, tp1, cond, rfail = m
        T = imm_value(s) if s["trace_kind"] == "CMP_IMM" else None
        mins = [c[k][8] for k in (0, 1) if c[k][0]]
        maxs = [c[k][9] for k in (0, 1) if c[k][0]]
        size = lhs_size(s)
        lo = decode_key(min(mins), size, s["signed"]) if mins else None
        hi = decode_key(max(maxs), size, s["signed"]) if maxs else None
        if hits == 0:
            stage = 0
        elif s["trace_kind"] == "TEST":
            stage = 2 if (equal and above) else 1
        elif below and above:
            stage = 3 if (T is not None and tm1 and equal and tp1) else 2
        else:
            stage = 1
        site_rows.append({
            "id": sid, "rva": hex(s["rva"]), "function": s["function"], "area": s["area"], "insn": s["insn"],
            "consumer": s["consumer"], "kind": s["trace_kind"], "signed": s["signed"], "rel": s["rel"], "T": T,
            "match_hits": hits, "menu_hits": c[2][0], "main_hits": c[0][0], "bg_hits": c[1][0],
            "below": below, "equal": equal, "above": above, "t_minus1": tm1, "t_plus1": tp1,
            "cond_true": cond, "cond_false": hits - rfail - cond if s["rel"] else None, "read_fail": rfail,
            "lhs_min": lo, "lhs_max": hi, "samples": o["s"] if o else [], "state": o["st"] if o else None,
            "stage": STAGES[stage], "claims": s["claims"],
        })

    # 주장(기록) 단위로 묶기
    claims = collections.OrderedDict()
    for r in site_rows:
        for cl in r["claims"]:
            k = cl["record"]
            e = claims.setdefault(k, {"record": k, "semantic": cl["semantic"], "class": cl["class"], "values": cl["values"],
                                      "function": r["function"], "area": r["area"], "meaning": cl["meaning"], "sites": []})
            e["sites"].append(r)
    out_claims = []
    for e in claims.values():
        best = max(STAGES.index(r["stage"]) for r in e["sites"])
        vcheck = []
        for r in e["sites"]:
            if r["T"] is None or not e["values"]:
                continue
            vals = [v for v in e["values"] if isinstance(v, (int, float))]
            if any(v == r["T"] for v in vals):
                vcheck.append("같음")
            elif any(abs(v - r["T"]) == 1 for v in vals):
                vcheck.append("±1(포함/미포함 표기)")
            else:
                vcheck.append("다름")
        out_claims.append({**{k: e[k] for k in ("record", "semantic", "class", "values", "function", "area", "meaning")},
                           "stage": STAGES[best], "value_check": sorted(set(vcheck)),
                           "site_ids": [r["id"] for r in e["sites"]]})

    stage_count = collections.Counter(c["stage"] for c in out_claims)
    site_stage = collections.Counter(r["stage"] for r in site_rows)
    vc = collections.Counter(v for c in out_claims for v in c["value_check"])
    summary = {"run_id": snap["run_id"], "t_ms": snap["t_ms"], "total_hits": snap["total_hits"],
               "armed": snap["armed"], "resting": snap["resting"], "refused": snap["refused"],
               "sites": len(site_rows), "site_stage": dict(site_stage), "claims": len(out_claims),
               "claim_stage": dict(stage_count), "claim_value_check": dict(vc)}
    out = a.out or os.path.join(os.path.dirname(os.path.abspath(a.jsonl)), f"site_report_{snap['run_id']}")
    os.makedirs(out, exist_ok=True)
    json.dump({"summary": summary, "sites": site_rows}, open(os.path.join(out, "sites.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    json.dump({"summary": summary, "claims": out_claims}, open(os.path.join(out, "claims.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    write_md(out, summary, site_rows, out_claims)
    print(json.dumps(summary, ensure_ascii=False, indent=1))


def fmt(v):
    return "" if v is None else str(v)


def write_md(out, summary, site_rows, claims):
    L = [f"# 임계점 트레이서 판독 — run {summary['run_id']}", "",
         f"- 경과 {summary['t_ms']/1000:.0f}초 · 관측점 총 적중 {summary['total_hits']:,} · 무장 {summary['armed']} / 휴식 {summary['resting']} / 거절 {summary['refused']}",
         f"- 지점 {summary['sites']}개 단계: " + ", ".join(f"{k} {summary['site_stage'].get(k,0)}" for k in STAGES),
         f"- 분석 기록 {summary['claims']}건 단계: " + ", ".join(f"{k} {summary['claim_stage'].get(k,0)}" for k in STAGES),
         f"- 값 대조(정적): {summary['claim_value_check']}", "",
         "단계 뜻: 미도달=경기 중 실행 안 됨 / 한쪽만=조건이 늘 한쪽 / 양쪽=실제로 갈림 / 경계확정=T-1·T·T+1 모두 관측", "",
         "## 경기 중 도달한 비교 지점 (적중순 상위 400)", "",
         "| 지점 | 함수 | 명령 → 소비 | 경기 적중(메인/배경) | 아래/같음/위 | T-1/T+1 | 조건 참/거짓 | 관측 범위 | 단계 | 대표 기록 |",
         "|---|---|---|---:|---|---|---|---|---|---|"]
    hit = sorted([r for r in site_rows if r["match_hits"]], key=lambda r: -r["match_hits"])
    for r in hit[:400]:
        cl = r["claims"][0]
        L.append(f"| `{r['rva']}` | `{r['function']}` | `{r['insn']}` → `{fmt(r['consumer'])}` | {r['match_hits']:,} ({r['main_hits']:,}/{r['bg_hits']:,}) "
                 f"| {r['below']:,}/{r['equal']:,}/{r['above']:,} | {r['t_minus1']}/{r['t_plus1']} | {r['cond_true']:,}/{fmt(r['cond_false'])} "
                 f"| {fmt(r['lhs_min'])} ~ {fmt(r['lhs_max'])} | {r['stage']} | {cl['semantic'][:50]} |")
    L += ["", "## 값 대조에서 '다름'이 나온 기록", "",
          "| 기록 | 함수 | 기록값 | 의미 | 단계 |", "|---|---|---|---|---|"]
    for c in claims:
        if "다름" in c["value_check"]:
            L.append(f"| {c['semantic'][:60]} | `{c['function']}` | {str(c['values'])[:40]} | {c['meaning'][:60]} | {c['stage']} |")
    open(os.path.join(out, "report.md"), "w", encoding="utf-8").write("\n".join(L) + "\n")


if __name__ == "__main__":
    main()
