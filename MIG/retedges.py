#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""retedges.py — probe20.txt 의 리턴 주소 히스토그램(`RET` 행)을 「호출자 함수 → 피호출 함수」 간선으로 환산한다. (2026-09-16 신설)

왜: 지도의 선은 exe 직접 호출(`call rel32`)만이라 vtable·점프테이블·fn-포인터 경유 호출이 안 보인다(r17 진입점 45 가 「깊이 0」으로 보인 이유).
    진입부 프로브 스텁이 리턴 주소를 세면(probe.rs RETREC) **실제로 밟은** 호출 간선이 횟수와 함께 나온다.

방법: 리턴 주소(rva) → `.pdata` RUNTIME_FUNCTION 으로 감싸는 함수 시작 rva(= 호출자) → 지도 노드(`AI함수지도.html` fndata) 와 대조.
      지도 밖 호출자(game_core 등)는 `ext` 로 남긴다(노드는 안 만들고 상세 패널에만).

사용: python -X utf8 MIG\\retedges.py <probe20.txt> [--out _next\\retedges.json] [--tag 판7]
출력: json {edges:[{s,t,n,kind}], ext:[{s_rva,t,n}], meta}, 요약 md
"""
import io, json, os, re, struct, sys, bisect
import pefile
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
MAP = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust\AI함수지도.html"
HERE = os.path.dirname(os.path.abspath(__file__))

def pdata_starts():
    pe = pefile.PE(EXE, fast_load=True)
    img = pe.get_memory_mapped_image()
    pd = [s for s in pe.sections if s.Name.startswith(b'.pdata')][0]
    raw = img[pd.VirtualAddress: pd.VirtualAddress + pd.Misc_VirtualSize]
    starts, ends = [], {}
    for i in range(0, len(raw) - 11, 12):
        b, e, u = struct.unpack_from('<III', raw, i)
        if b == 0: break
        starts.append(b); ends[b] = e
    starts.sort()
    return starts, ends

def owner(starts, ends, rva):
    i = bisect.bisect_right(starts, rva) - 1
    if i < 0: return None
    b = starts[i]
    return b if rva < ends[b] + 16 else None   # 리턴 주소가 함수 끝 직후일 수도(tail) → 약간 여유

def newest_map():
    cands = [MAP, MAP.replace(r"\tfm2\mods_report", r"\tfm2\.claude\worktrees\swap-order-button-style-fe9e04\mods_report")]
    cands = [c for c in cands if os.path.exists(c)]
    return max(cands, key=os.path.getmtime)

def main():
    av = sys.argv[1:]
    src = av[0]
    out = av[av.index("--out") + 1] if "--out" in av else os.path.join(HERE, "_next", "retedges.json")
    tag = av[av.index("--tag") + 1] if "--tag" in av else ""
    txt = io.open(src, encoding="utf-8", errors="replace").read()
    rows = re.findall(r"^\s+RET\s+(\d+)\s+0x([0-9a-f]+)\s+(rva|abs|overflow)\s*(?:0x([0-9a-f]+))?\s+(\d+)", txt, re.M)
    starts, ends = pdata_starts()
    h = io.open(newest_map(), encoding="utf-8").read()
    data = json.loads(re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S).group(1))
    by = {int(n["a"], 16): n for n in data if not n["a"].startswith("spec-")}
    direct = set()
    for n in data:
        if n["a"].startswith("spec-"): continue
        for t in n.get("o") or []: direct.add((n["a"], t))
    edges, ext, ovf, unres = {}, {}, {}, {}
    for idx, callee, kind, ret, cnt in rows:
        callee_rva = int(callee, 16); cnt = int(cnt)
        if kind == "overflow":
            ovf[callee_rva] = cnt; continue
        if kind == "abs":
            unres[("abs", ret, callee_rva)] = cnt; continue
        ret_rva = int(ret, 16)
        o = owner(starts, ends, ret_rva)
        if o is None:
            unres[("nofn", ret, callee_rva)] = cnt; continue
        tn = by.get(callee_rva)
        if o in by and tn:
            key = (by[o]["a"], tn["a"]); edges[key] = edges.get(key, 0) + cnt
        else:
            key = (o, callee_rva); ext[key] = ext.get(key, 0) + cnt
    E = [{"s": s, "t": t, "n": n, "kind": "direct" if (s, t) in direct else "indirect"} for (s, t), n in edges.items()]
    E.sort(key=lambda e: -e["n"])
    X = [{"s_rva": "%x" % s, "t": "%x" % t, "n": n} for (s, t), n in ext.items()]
    X.sort(key=lambda e: -e["n"])
    mp = re.search(u"진입부 (\d+)", txt)
    j = {"meta": {"src": src, "tag": tag, "rows": len(rows), "probes": int(mp.group(1)) if mp else 0, "edges": len(E), "indirect_new": sum(1 for e in E if e["kind"] == "indirect"),
                  "ext": len(X), "overflow": {"%x" % k: v for k, v in ovf.items()}, "unresolved": len(unres)},
         "edges": E, "ext": X}
    os.makedirs(os.path.dirname(out), exist_ok=True)
    json.dump(j, io.open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    # 요약
    L = [u"# 실측 호출 간선(리턴 주소 히스토그램 · %s · %s)" % (tag, os.path.basename(src)), u"",
         u"> RET 행 %d → 지도 안 간선 %d(직접 호출과 겹침 %d · **새 간접 간선 %d**) · 지도 밖 호출자 %d · 오버플로 슬롯 %d · 미해소 %d" % (
             len(rows), len(E), sum(1 for e in E if e["kind"] == "direct"), j["meta"]["indirect_new"], len(X), len(ovf), len(unres)), u"",
         u"## 새 간접 간선(정적 지도에 없던 것 · 횟수순)", u"", u"| 호출자 | → 피호출 | 횟수 |", u"|---|---|---|"]
    nm = lambda a: (by[int(a, 16)].get("n") or ("FUN_" + a)) if int(a, 16) in by else a
    for e in E:
        if e["kind"] == "indirect":
            L.append(u"| `%s` %s | `%s` %s | %s |" % (e["s"], nm(e["s"]), e["t"], nm(e["t"]), u"{:,}".format(e["n"])))
    L += [u"", u"## 지도 밖 호출자(game_core 등 · 상위 40)", u"", u"| 호출자 rva | → 피호출 | 횟수 |", u"|---|---|---|"]
    for e in X[:40]:
        L.append(u"| `0x%s` | `%s` %s | %s |" % (e["s_rva"], e["t"], nm(e["t"]), u"{:,}".format(e["n"])))
    io.open(out[:-5] + ".md", "w", encoding="utf-8").write(u"\n".join(L))
    print(u"RET %d → edges %d (direct %d · indirect %d) · ext %d · ovf %d · unresolved %d → %s" % (
        len(rows), len(E), len(E) - j["meta"]["indirect_new"], j["meta"]["indirect_new"], len(X), len(ovf), len(unres), out))

if __name__ == "__main__":
    main()
