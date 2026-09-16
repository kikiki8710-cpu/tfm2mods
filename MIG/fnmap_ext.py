#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""fnmap_ext.py — 실측으로 드러난 「지도 밖 호출자」를 AI함수지도 노드로 편입한다(계층 `ext`). (2026-09-16 신설)

왜: 판 8 리턴 주소 실측에서 지도 658 노드의 「깊이 0」 129개는 대부분 **지도 밖** 함수(game_core 틱 루프 · enum 디스패처 조각 ·
    TLS/캐시 래퍼)가 부르고 있었다. 그 호출자를 노드로 넣어야 「매 틱 → get_input → 디스패처 → 서브플랜 → 잎」 사슬이 한 지도에서 이어진다.

무엇을 넣나(2단까지):
  1단 = `retedges.json` 의 `ext`(실측 호출자 · 지도 밖) → 노드 + 실측 간선(ext → 지도 노드 · 횟수)
  2단 = 1단 노드를 **정적으로** 부르는 함수(.text `call/jmp rel32` 전수 · `.pdata` 로 감싸는 함수) — 지도 안이면 간선만, 밖이면 노드 추가(그 위는 노드 없이 호출자 수만 기록)
  이름 = `fnmap_ret.EXTNAME`(rvaname 확정 / [추정]) · 없으면 미명명(FUN_)
멱등: 계층 `ext` 노드를 전부 지우고 다시 만든다 · JS/CSS 는 마커 1회.
사용: python -X utf8 MIG\\fnmap_ext.py [_next\\retedges.json] [--map <html>]
"""
import io, json, os, re, struct, sys, bisect
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import fnmap_ret, retedges
EXE = retedges.EXE
MARK = "/* EXTNODES-JS v1 */"
ROOT = "1879080"   # game_core 틱 루프 — 여기서 멈춘다(그 위는 엔진)

def text_calls(wanted):
    u"""`.text` 전수 1회 스캔: call/jmp rel32 의 타깃이 wanted 에 있으면 {tgt: [site]}"""
    d = open(EXE, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]; nsec = struct.unpack_from("<H", d, pe + 6)[0]; oh = struct.unpack_from("<H", d, pe + 20)[0]
    for i in range(nsec):
        o = pe + 24 + oh + i * 40
        if d[o:o + 8].rstrip(b"\0") == b".text":
            vsz, va, rsz, ra = struct.unpack_from("<IIII", d, o + 8); break
    buf = d[ra:ra + rsz]; out = {}
    for m in re.finditer(b"[\xe8\xe9]", buf):
        i = m.start()
        if i + 5 > len(buf): break
        rel = struct.unpack_from("<i", buf, i + 1)[0]; tgt = va + i + 5 + rel
        if tgt in wanted: out.setdefault(tgt, []).append(va + i)
    return out

def main():
    av = sys.argv[1:]
    src = [a for a in av if not a.startswith("--") and (av.index(a) == 0 or av[av.index(a) - 1] != "--map")]
    src = src[0] if src else os.path.join(HERE, "_next", "retedges.json")
    H = av[av.index("--map") + 1] if "--map" in av else fnmap_ret.DEF_MAP
    j = json.load(io.open(src, encoding="utf-8"))
    h = io.open(H, encoding="utf-8").read()
    m1 = re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S)
    d = [n for n in json.loads(m1.group(1)) if n.get("L") != "ext"]      # 멱등: ext 노드 제거
    for n in d:
        n["o"] = [t for t in n.get("o") or [] if not t.startswith("x")]  # (예약) ext 간선 표식 없음 — 아래서 다시 채움
    BY = {n["a"]: n for n in d}
    starts, ends = retedges.pdata_starts()
    # ── 1단: 실측 지도 밖 호출자
    ext = {}
    for e in j["ext"]:
        r = ext.setdefault(e["s_rva"], {"o": {}, "n": 0})
        r["o"][e["t"]] = r["o"].get(e["t"], 0) + e["n"]; r["n"] += e["n"]
    # ext 노드가 서로를 부르는 경우(실측 ic 에 ext 가 있으면) 는 ext→ext 로도 이어진다
    L1 = set(ext)
    # ── 2단: 1단 노드의 정적 호출자
    calls = text_calls({int(a, 16) for a in L1})
    L2 = {}
    for a in L1:
        for site in calls.get(int(a, 16), []):
            o = retedges.owner(starts, ends, site)
            if o is None: continue
            oa = "%x" % o
            if oa == a: continue
            if oa in BY: BY[oa]["o"].append(a) if a not in BY[oa]["o"] else None
            elif oa in L1: ext[oa]["o"].setdefault(a, 0)
            else: L2.setdefault(oa, set()).add(a)
    # 2단 노드의 정적 호출자 수만(노드는 안 만든다)
    calls2 = text_calls({int(a, 16) for a in L2})
    def size_of(a):
        v = int(a, 16); return (ends.get(v, v) - v) if v in ends else 0
    def name_of(a):
        nm = fnmap_ret.EXTNAME.get(a)
        if a == ROOT: return nm or u"game_core 시뮬레이션 틱 루프", "re", u"AI 루트. 매 틱 에이전트마다 Agent::get_input 을 부른다(패닉 Location mode.rs:396/526/276 · 판 8 실측 1.64억회). 그 위는 엔진(여기서 멈춤)."
        if nm and not nm.startswith(u"[추정]"): return nm, "re", u"rvaname 패닉 Location 지문으로 확정 · 판 8 리턴 주소 실측으로 편입(지도 밖 호출자)"
        if nm: return nm[len(u"[추정] "):], "cg", u"지문 없음 — 판 8 실측 콜리 집합으로 추정한 이름(확정은 ghidra) · 지도 밖 호출자 편입"
        return None, None, u"지문 없는 소형 래퍼/디스패처 조각 — 판 8 리턴 주소 실측으로 편입(지도 밖 호출자) · 이름 미확정"
    new = []
    for a, r in sorted(ext.items(), key=lambda kv: -kv[1]["n"]):
        nm, v, x = name_of(a)
        callees = sorted(r["o"], key=lambda t: -r["o"][t])
        new.append({"a": a, "n": nm, "m": u"exe·지도 밖", "b": size_of(a), "i": 0, "l": [], "v": v, "f": "", "x": x, "ai": 0,
                    "o": callees, "c": [], "L": "ext", "ext": 1, "ret": r["n"], "oc": {t: c for t, c in r["o"].items()},
                    "im": [[t, c] for t, c in sorted(r["o"].items(), key=lambda kv: -kv[1])], "ic": [], "ie": []})
    for a, tg in L2.items():
        nsite = len(calls2.get(int(a, 16), []))
        new.append({"a": a, "n": fnmap_ret.EXTNAME.get(a), "m": u"exe·지도 밖(2단)", "b": size_of(a), "i": 0, "l": [], "v": "cg" if a in fnmap_ret.EXTNAME else None, "f": "",
                    "x": u"정적 xref(.text call/jmp rel32 → .pdata)로 올라간 2단 호출자 · 실측 아님 · 이 함수를 정적으로 부르는 곳 %d(노드로 안 올림)" % nsite,
                    "ai": 0, "o": sorted(tg), "c": [], "L": "ext", "ext": 2, "ret": 0, "oc": {}, "im": [], "ic": [], "ie": []})
    d += new
    BY = {n["a"]: n for n in d}
    # c(호출자 목록) 재구성: ext 가 관련된 것만 갱신(기존 정적 c 는 유지)
    for n in new:
        for t in n["o"]:
            if t in BY and n["a"] not in BY[t]["c"]: BY[t]["c"].append(n["a"])
    for n in d:
        if n.get("L") == "ext": continue
        for t in n["o"]:
            if t in BY and BY[t].get("L") == "ext" and n["a"] not in BY[t]["c"]: BY[t]["c"].append(n["a"])
    h = h[:m1.start(1)] + json.dumps(d, ensure_ascii=False, separators=(",", ":")) + h[m1.end(1):]
    # ── JS/CSS(1회)
    if MARK not in h:
        def rep(a, b):
            nonlocal h
            assert h.count(a) == 1, (a[:60], h.count(a)); h = h.replace(a, b)
        rep(u"            etc:{label:'기타', tip:'세 서브트리 어디서도 닿지 않고 모듈로도 못 가른 함수'}};",
            u"            etc:{label:'기타', tip:'세 서브트리 어디서도 닿지 않고 모듈로도 못 가른 함수'},\n"
            u"            " + MARK + u" ext:{label:'지도 밖 호출자', tip:'판 8 리턴 주소 실측으로 드러난 지도 밖 호출자(game_core 틱 루프 · enum 디스패처 조각 · TLS/캐시 래퍼) + 그 정적 호출자(2단). 「깊이 0」이 진짜 뿌리(틱 루프)로 수렴한다'}};")
        rep(u"  var LYORD = ['plan','act','path','excl','etc'];", u"  var LYORD = ['plan','act','path','excl','etc','ext'];")
        rep(u":root{--ly-plan:#15628F; --ly-act:#B5541A; --ly-path:#2E7D4F; --ly-excl:#7A5C9E; --ly-etc:#8496AB}",
            u":root{--ly-plan:#15628F; --ly-act:#B5541A; --ly-path:#2E7D4F; --ly-excl:#7A5C9E; --ly-etc:#8496AB; --ly-ext:#9A2F5C}")
        rep(u"@media (prefers-color-scheme: dark){ :root:not([data-theme=\"light\"]){--ly-plan:#6BB8EB; --ly-act:#F0A15E; --ly-path:#6FCF97; --ly-excl:#B79BE0; --ly-etc:#607388} }",
            u"@media (prefers-color-scheme: dark){ :root:not([data-theme=\"light\"]){--ly-plan:#6BB8EB; --ly-act:#F0A15E; --ly-path:#6FCF97; --ly-excl:#B79BE0; --ly-etc:#607388; --ly-ext:#E58AB4} }")
        rep(u":root[data-theme=\"dark\"]{--ly-plan:#6BB8EB; --ly-act:#F0A15E; --ly-path:#6FCF97; --ly-excl:#B79BE0; --ly-etc:#607388}",
            u":root[data-theme=\"dark\"]{--ly-plan:#6BB8EB; --ly-act:#F0A15E; --ly-path:#6FCF97; --ly-excl:#B79BE0; --ly-etc:#607388; --ly-ext:#E58AB4}")
        rep(u".badge.ly-excl{color:var(--ly-excl); border-color:var(--ly-excl)}",
            u".badge.ly-excl{color:var(--ly-excl); border-color:var(--ly-excl)}\n.badge.ly-ext{color:var(--ly-ext); border-color:var(--ly-ext)}\n.fn .ly.ext{background:var(--ly-ext)}")
        rep(u"<i style=\"background:var(--ly-etc)\"></i>기타 · 옅은 띠 = 모듈로 추정",
            u"<i style=\"background:var(--ly-etc)\"></i>기타 <i style=\"background:var(--ly-ext)\"></i>지도 밖 호출자(실측 편입) · 옅은 띠 = 모듈로 추정")
    if "/* EXTNODES-JS v2 */" not in h:
        a2 = u"  var named = DATA.filter(function(d){return d.n;});\n  var edges = DATA.reduce(function(s,d){return s + d.o.length;},0);"
        b2 = (u"  /* EXTNODES-JS v2 */ var CORE = DATA.filter(function(d){return d.L!=='ext';}); var EXTN = DATA.length - CORE.length;\n"
              u"  var named = CORE.filter(function(d){return d.n;});\n  var edges = DATA.reduce(function(s,d){return s + d.o.length;},0);")
        assert h.count(a2) == 1; h = h.replace(a2, b2)
        a3 = u"    [fmt(DATA.length),'식별한 함수'],\n    [fmt(named.length),'이름 확정'],\n    [fmt(DATA.length-named.length),'미확정'],"
        b3 = u"    [fmt(CORE.length),'식별한 함수'],\n    [fmt(EXTN),'지도 밖 호출자 편입'],\n    [fmt(named.length),'이름 확정'],\n    [fmt(CORE.length-named.length),'미확정'],"
        assert h.count(a3) == 1; h = h.replace(a3, b3)
    io.open(H, "w", encoding="utf-8").write(h)
    n1 = sum(1 for n in new if n["ext"] == 1); n2 = len(new) - n1
    e_in = sum(1 for n in d if n.get("L") != "ext" for t in n["o"] if t in BY and BY[t].get("L") == "ext")
    print(u"ext 노드 1단 %d + 2단 %d = %d · 지도 안→ext 정적 간선 %d · 총 노드 %d → %s" % (n1, n2, len(new), e_in, len(d), H))
    # 요약 md
    L = [u"# 지도 밖 호출자 편입(fnmap_ext · %s)" % os.path.basename(src), u"", u"| rva | 이름 | 단 | 근거 | 실측 호출수 | → 콜리(지도) | 정적 호출자(지도 안) |", u"|---|---|---|---|---|---|---|"]
    nm = lambda a: (BY[a].get("n") or "FUN_" + a) if a in BY else a
    for n in new:
        L.append(u"| `%s` | %s | %d | %s | %s | %s | %s |" % (n["a"], n["n"] or u"(미명명)", n["ext"], {"re": u"확정", "cg": u"추정/정적", None: u"미확정"}[n["v"]],
                 u"{:,}".format(n["ret"]), u" · ".join(nm(t) for t in n["o"][:5]) + (u" …" if len(n["o"]) > 5 else u""), u" · ".join(nm(c) for c in n["c"][:4])))
    io.open(os.path.join(HERE, "_next", "fnmap_ext.md"), "w", encoding="utf-8").write(u"\n".join(L))

if __name__ == "__main__":
    main()
