#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""retmerge.py — 여러 판의 `retedges.py` 출력을 합친다(간선 합집합 · 횟수 합산 · 판별 존재 표). (2026-09-16 신설)

왜: 한 판의 리턴 주소 실측은 하한이다(그 판에서 안 밟은 간선은 없다). 리플레이를 바꿔 돌리면 다른 팀·챔피언·전개가
    다른 간선을 밟는다 — 판마다 따로 보지 말고 합쳐서 지도에 얹고, 「어느 판에만 있던 간선」을 따로 보고한다.

사용: python -X utf8 MIG\\retmerge.py _next\\retedges_run8.json _next\\retedges_run9.json --out _next\\retedges.json --tag "판 8+9"
출력: 합본 json(edges/ext 는 fnmap_ret.py 가 그대로 읽는다 · 각 항목에 `runs`=[판별 횟수]) + md(판별 고유 간선)
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))

def main():
    av = sys.argv[1:]
    out = av[av.index("--out") + 1] if "--out" in av else os.path.join(HERE, "_next", "retedges.json")
    tag = av[av.index("--tag") + 1] if "--tag" in av else ""
    srcs = [a for i, a in enumerate(av) if not a.startswith("--") and (i == 0 or not av[i - 1].startswith("--"))]
    J = [json.load(io.open(p, encoding="utf-8")) for p in srcs]
    tags = [(j.get("meta") or {}).get("tag") or os.path.basename(p) for j, p in zip(J, srcs)]
    E, X = {}, {}
    for k, j in enumerate(J):
        for e in j["edges"]:
            r = E.setdefault((e["s"], e["t"]), {"s": e["s"], "t": e["t"], "n": 0, "kind": e["kind"], "runs": [0] * len(J)})
            r["n"] += e["n"]; r["runs"][k] += e["n"]
        for e in j["ext"]:
            r = X.setdefault((e["s_rva"], e["t"]), {"s_rva": e["s_rva"], "t": e["t"], "n": 0, "runs": [0] * len(J)})
            r["n"] += e["n"]; r["runs"][k] += e["n"]
    edges = sorted(E.values(), key=lambda e: -e["n"]); ext = sorted(X.values(), key=lambda e: -e["n"])
    probes = max(((j.get("meta") or {}).get("probes") or 0) for j in J)
    rows = sum(((j.get("meta") or {}).get("rows") or 0) for j in J)
    meta = {"src": srcs, "tags": tags, "tag": tag, "rows": rows, "probes": probes, "edges": len(edges),
            "indirect_new": sum(1 for e in edges if e["kind"] == "indirect"), "ext": len(ext),
            "only_in": [sum(1 for e in edges if e["runs"][k] and all(v == 0 for i, v in enumerate(e["runs"]) if i != k)) for k in range(len(J))]}
    json.dump({"meta": meta, "edges": edges, "ext": ext}, io.open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    # 이름은 가장 최근 지도에서
    try:
        sys.path.insert(0, HERE); import retedges
        h = io.open(retedges.newest_map(), encoding="utf-8").read()
        data = json.loads(re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S).group(1))
        by = {n["a"]: n for n in data}
        nm = lambda a: (by[a].get("n") or ("FUN_" + a)) if a in by else a
    except Exception:
        nm = lambda a: a
    L = [u"# 실측 호출 간선 합본(%s = %s)" % (tag, u" + ".join(tags)), u"",
         u"> 지도 안 간선 %d(새 간접 %d) · 지도 밖 호출자 %d · 판별 고유 간선 = %s" % (
             len(edges), meta["indirect_new"], len(ext), u" · ".join(u"%s 에만 %d" % (t, c) for t, c in zip(tags, meta["only_in"]))), u""]
    for k, t in enumerate(tags):
        L += [u"## %s 에만 있던 지도 안 간선(횟수순 · 상위 40)" % t, u"", u"| 호출자 | → 피호출 | 횟수 | 종류 |", u"|---|---|---|---|"]
        cnt = 0
        for e in edges:
            if e["runs"][k] and all(v == 0 for i, v in enumerate(e["runs"]) if i != k):
                L.append(u"| `%s` %s | `%s` %s | %s | %s |" % (e["s"], nm(e["s"]), e["t"], nm(e["t"]), u"{:,}".format(e["n"]), e["kind"]))
                cnt += 1
                if cnt >= 40: break
        L.append(u"")
    L += [u"## 새 간접 간선 합본(정적 지도에 없던 것 · 횟수순)", u"", u"| 호출자 | → 피호출 | 합계 | " + u" | ".join(tags) + u" |", u"|---|---|---|" + u"---|" * len(tags)]
    for e in edges:
        if e["kind"] == "indirect":
            L.append(u"| `%s` %s | `%s` %s | %s | %s |" % (e["s"], nm(e["s"]), e["t"], nm(e["t"]), u"{:,}".format(e["n"]), u" | ".join(u"{:,}".format(v) for v in e["runs"])))
    io.open(out[:-5] + ".md", "w", encoding="utf-8").write(u"\n".join(L))
    print(u"합본 %s: edges %d(indirect %d) · ext %d · 판별 고유 %s → %s" % (tag, len(edges), meta["indirect_new"], len(ext), meta["only_in"], out))

if __name__ == "__main__":
    main()
