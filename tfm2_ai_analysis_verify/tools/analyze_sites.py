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
import argparse, collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REL = {1: "<", 2: ">=", 3: "<=", 4: ">", 5: "==", 6: "!=", 0: None}
STAGES = ["미도달", "한쪽만", "양쪽", "경계확정", "도달"]  # 도달 = 값 기록 지점(비교 아님)이 실행됨
RANK = {"미도달": 0, "도달": 1, "한쪽만": 2, "양쪽": 3, "경계확정": 4}


def last_line(path):
    """파일 끝에서 거꾸로 읽어 마지막 완전한 줄만 돌려준다(누적 스냅샷이라 마지막 줄 = 최종값)."""
    with open(path, "rb") as f:
        f.seek(0, 2); end = f.tell(); buf = b""; pos = end
        while pos > 0:
            step = min(1 << 20, pos); pos -= step; f.seek(pos)
            buf = f.read(step) + buf
            body = buf.rstrip(b"\n")
            i = body.rfind(b"\n")
            if i >= 0:
                return body[i + 1:].decode("utf-8")
        body = buf.rstrip(b"\n")
        if not body:
            sys.exit("스냅샷 없음")
        return body.decode("utf-8")


OP_RE = re.compile(r"(>=|<=|≥|≤|<|>)\s*(-?\d[\d,]*)|(-?\d[\d,]*)\s*%?\s*(이상|이하|미만|초과)")


def boundary_of(op, n):
    """비교를 '참/거짓이 바뀌는 첫 값'(>= 쪽 시작값)으로 정규화한다. 정수 비교 전제."""
    return {">=": n, "≥": n, "이상": n, ">": n + 1, "초과": n + 1,
            "<": n, "미만": n, "<=": n + 1, "≤": n + 1, "이하": n + 1}[op]


def text_bounds(meaning):
    out = []
    for a, b, c, d in OP_RE.findall(meaning or ""):
        op, num = (a, b) if a else (d, c)
        try:
            n = int(num.replace(",", ""))
        except ValueError:
            continue
        out.append((op, n, boundary_of(op, n)))
    return out


def site_boundary(T, rel):
    if T is None or rel not in (">=", ">", "<", "<="):
        return None
    return boundary_of(rel, T)


def boundary_check(site, meaning):
    """명령의 경계 vs 서술의 경계. 일치 / 경계 어긋남(±1) / 서술에 해당 수치 없음 / 대상 아님."""
    B = site_boundary(site["T"], site["rel"])
    if B is None:
        return "대상 아님", None
    tb = text_bounds(meaning)
    if not tb:
        return "서술에 부등호 없음", None
    if any(x[2] == B for x in tb):
        return "일치", None
    near = [x for x in tb if abs(x[1] - site["T"]) <= 1 or abs(x[2] - B) <= 1]
    if near:
        return "경계 어긋남(±1)", near
    return "서술에 해당 수치 없음", None


def merge_snaps(snaps):
    """같은 지점 표 회차들의 누적 스냅샷 합산: 횟수는 더하고, 최소/최대는 각각 min/max, 표본은 이어 붙인다."""
    NONE = (1 << 64) - 1
    out = {"run_id": "+".join(x["run_id"] for x in snaps), "t_ms": sum(x["t_ms"] for x in snaps),
           "total_hits": sum(x["total_hits"] for x in snaps), "armed": max(x["armed"] for x in snaps),
           "resting": 0, "refused": max(x["refused"] for x in snaps), "sites": []}
    acc = {}
    for sn in snaps:
        for o in sn["sites"]:
            e = acc.get(o["id"])
            if e is None:
                acc[o["id"]] = {"id": o["id"], "rva": o["rva"], "st": o["st"], "c": [list(c) for c in o["c"]],
                                "rmin": o["rmin"], "rmax": o["rmax"], "s": list(o["s"])}
                continue
            for k in range(3):
                a, b = e["c"][k], o["c"][k]
                for i in range(8):
                    a[i] += b[i]
                a[8] = min(a[8], b[8]) if b[0] else a[8]
                a[9] = max(a[9], b[9])
            e["rmin"] = min(e["rmin"], o["rmin"]) if o["rmin"] != NONE else e["rmin"]
            e["rmax"] = max(e["rmax"], o["rmax"])
            e["s"] = (e["s"] + o["s"])[-16:]
    out["sites"] = list(acc.values())
    return out


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
    op = site.get("vlhs") if site["trace_kind"] == "VALUE" else site["ops"][0]
    return op[1]["size"] if op else 8


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("jsonl", nargs="+", help="여러 개면 같은 지점 표의 회차로 보고 카운터를 합산")
    ap.add_argument("--catalog", default=os.path.join(HERE, "sites", "site_catalog.json"))
    ap.add_argument("--out", default=None)
    a = ap.parse_args()
    snaps = [json.loads(last_line(p)) for p in a.jsonl]
    snap = merge_snaps(snaps) if len(snaps) > 1 else snaps[0]
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
        # 양쪽 = 소비 조건이 참·거짓 둘 다 나옴(조건을 모르면 임계점 아래·위 둘 다).
        # 경계확정 = T-1·T·T+1 모두. 부호 없는 T=0 은 T-1 이 원리상 없으므로 T·T+1 만 본다.
        cond_known = bool(s["rel"]) and s["trace_kind"] != "TEST" or (s["trace_kind"] == "TEST" and s["rel"] in ("==", "!="))
        cond_false = hits - rfail - cond if s["rel"] else None
        unsigned_zero = T == 0 and s["signed"] != "s"
        if hits == 0:
            stage = 0
        elif s["trace_kind"] == "VALUE":
            stage = 4
        else:
            if cond_known:
                both = cond > 0 and cond_false > 0
            elif s["trace_kind"] == "TEST":
                both = equal > 0 and above > 0
            else:
                both = (below > 0 or unsigned_zero and equal > 0) and above > 0
            edge = T is not None and equal and tp1 and (tm1 or unsigned_zero)
            stage = 3 if (both and edge) else (2 if both else 1)
        site_rows.append({
            "id": sid, "rva": hex(s["rva"]), "function": s["function"], "area": s["area"], "insn": s["insn"],
            "consumer": s["consumer"], "kind": s["trace_kind"], "signed": s["signed"], "rel": s["rel"], "T": T,
            "match_hits": hits, "menu_hits": c[2][0], "ingame_hits": c[0][0], "gametest_hits": c[1][0],
            "rhs_min": decode_key(o["rmin"], 8, s["signed"]) if o and s["trace_kind"] in ("CMP_DYN", "VALUE") and o["rmin"] != (1 << 64) - 1 else None,
            "rhs_max": decode_key(o["rmax"], 8, s["signed"]) if o and s["trace_kind"] in ("CMP_DYN", "VALUE") and o["rmin"] != (1 << 64) - 1 else None,
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
        best_stage = max((r["stage"] for r in e["sites"]), key=lambda x: RANK[x])
        bchecks = []
        for r in e["sites"]:
            b, near = boundary_check(r, e["meaning"])
            if b != "대상 아님":
                bchecks.append({"site": r["rva"], "insn": r["insn"], "rel": r["rel"], "T": r["T"], "check": b,
                                "text": [f"{o} {n}" for o, n, _ in near] if near else None, "stage": r["stage"]})
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
                           "stage": best_stage, "value_check": sorted(set(vcheck)), "boundary_check": bchecks,
                           "site_ids": [r["id"] for r in e["sites"]]})

    stage_count = collections.Counter(c["stage"] for c in out_claims)
    site_stage = collections.Counter(r["stage"] for r in site_rows)
    vc = collections.Counter(v for c in out_claims for v in c["value_check"])
    bc = collections.Counter()
    for c in out_claims:
        ks = {b["check"] for b in c["boundary_check"]}
        bc["일치" if "일치" in ks else ("경계 어긋남(±1)" if "경계 어긋남(±1)" in ks else (sorted(ks)[0] if ks else "대상 아님"))] += 1
    summary = {"run_id": snap["run_id"], "t_ms": snap["t_ms"], "total_hits": snap["total_hits"],
               "armed": snap["armed"], "resting": snap["resting"], "refused": snap["refused"],
               "sites": len(site_rows), "site_stage": dict(site_stage), "claims": len(out_claims),
               "claim_stage": dict(stage_count), "claim_value_check": dict(vc), "claim_boundary_check": dict(bc)}
    out = a.out or os.path.join(os.path.dirname(os.path.abspath(a.jsonl[0])), f"site_report_{snap['run_id'][:40]}")
    os.makedirs(out, exist_ok=True)
    slim = [{**{k: v for k, v in r.items() if k != "claims"}, "records": [c["record"] for c in r["claims"]]} for r in site_rows]
    json.dump({"summary": summary, "sites": slim}, open(os.path.join(out, "sites.json"), "w", encoding="utf-8"), ensure_ascii=False, separators=(",", ":"))
    json.dump({"summary": summary, "claims": out_claims}, open(os.path.join(out, "claims.json"), "w", encoding="utf-8"), ensure_ascii=False, separators=(",", ":"))
    write_md(out, summary, site_rows, out_claims)
    print(json.dumps(summary, ensure_ascii=False, indent=1))


def fmt(v):
    return "" if v is None else str(v)


def write_md(out, summary, site_rows, claims):
    L = [f"# 임계점 트레이서 판독 — run {summary['run_id']}", "",
         f"- 경과 {summary['t_ms']/1000:.0f}초 · 관측점 총 적중 {summary['total_hits']:,} · 무장 {summary['armed']} / 휴식 {summary['resting']} / 거절 {summary['refused']}",
         f"- 지점 {summary['sites']}개 단계: " + ", ".join(f"{k} {summary['site_stage'].get(k,0)}" for k in STAGES),
         f"- 분석 기록 {summary['claims']}건 단계: " + ", ".join(f"{k} {summary['claim_stage'].get(k,0)}" for k in STAGES),
         f"- 값 대조(정적): {summary['claim_value_check']}",
         f"- 경계 대조(서술 부등호 vs 명령): {summary['claim_boundary_check']}", "",
         "단계 뜻: 미도달=경기 중 실행 안 됨 / 도달=값 기록 지점이 실행됨 / 한쪽만=조건이 늘 한쪽 / 양쪽=실제로 갈림 / 경계확정=T-1·T·T+1 모두 관측", "",
         "## 경기 중 도달한 비교 지점 (적중순 상위 400)", "",
         "| 지점 | 함수 | 명령 → 소비 | 경기 적중(InGame/GameTest) | 아래/같음/위 | T-1/T+1 | 조건 참/거짓 | 관측 범위 | 단계 | 대표 기록 |",
         "|---|---|---|---:|---|---|---|---|---|---|"]
    hit = sorted([r for r in site_rows if r["match_hits"]], key=lambda r: -r["match_hits"])
    for r in hit[:400]:
        cl = r["claims"][0]
        L.append(f"| `{r['rva']}` | `{r['function']}` | `{r['insn']}` → `{fmt(r['consumer'])}` | {r['match_hits']:,} ({r['ingame_hits']:,}/{r['gametest_hits']:,}) "
                 f"| {r['below']:,}/{r['equal']:,}/{r['above']:,} | {r['t_minus1']}/{r['t_plus1']} | {r['cond_true']:,}/{fmt(r['cond_false'])} "
                 f"| {fmt(r['lhs_min'])} ~ {fmt(r['lhs_max'])} | {r['stage']} | {cl['semantic'][:50]} |")
    L += ["", "## 값 대조에서 '다름'이 나온 기록", "",
          "| 기록 | 함수 | 기록값 | 의미 | 단계 |", "|---|---|---|---|---|"]
    for c in claims:
        if "다름" in c["value_check"]:
            L.append(f"| {c['semantic'][:60]} | `{c['function']}` | {str(c['values'])[:40]} | {c['meaning'][:60]} | {c['stage']} |")
    L += ["", "## 경계 대조: 서술의 부등호·값이 명령과 1 차이 나는 기록 (포함/미포함 오기 후보)", "",
          "| 기록 | 함수 | 지점 | 실제 명령 → 조건 | 서술 속 표현 | 실측 단계 | 의미(앞부분) |", "|---|---|---|---|---|---|---|"]
    seen = set()
    for c in claims:
        ks = {b["check"] for b in c["boundary_check"]}
        if "경계 어긋남(±1)" in ks and "일치" not in ks:
            for b in c["boundary_check"]:
                if b["check"] == "경계 어긋남(±1)" and (c["semantic"], b["site"]) not in seen:
                    seen.add((c["semantic"], b["site"]))
                    L.append(f"| {c['semantic'][:55]} | `{c['function']}` | `{b['site']}` | `{b['insn']}` → {b['rel']} {b['T']} | {', '.join(b['text'] or [])} | {b['stage']} | {c['meaning'][:70]} |")
    open(os.path.join(out, "report.md"), "w", encoding="utf-8").write("\n".join(L) + "\n")


if __name__ == "__main__":
    main()
