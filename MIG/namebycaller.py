#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""namebycaller.py — 지도 미명명(`?`) exe 함수에 「호출자 차집합」으로 IR 심볼 이름을 붙인다 (2026-09-13)

원리: 호출자 X 의 exe 콜리 집합(cg_exe) 과 X 의 IR 콜리 집합(reach_tree 캐시 `rc_*.json`)을 **이미 이름이 붙은 것끼리 지운다**.
      남는 exe RVA 가 1개·남는 IR game_ai 심볼이 1개면 그 둘이 짝이다(모듈 힌트·크기로 재검). 2×2 이상이면 후보로만 낸다.
      dllmatch(문자열/상수 지문)·percolate(이웃 전파)가 못 잇는 것 = 문자열이 없는 작은 클로저/모노모프 — 이 방법이 그 빈틈을 메운다.

사용: python -X utf8 MIG\\namebycaller.py [subtree json] [--apply]
출력: RVA · 후보 심볼 · 근거(호출자 · 남은 수) · --apply 면 `_next\\reach\\names_bycaller.json` 저장(지도 갱신은 fnmap_update 로).
"""
import io, os, re, sys, json, collections, hashlib

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "_next", "reach")
sys.path.insert(0, HERE)
import reach_tree as RT


def main():
    a = [x for x in sys.argv[1:] if x.endswith(".json")]
    sub_p = a[0] if a else os.path.join(HERE, "_next", "subtree_e4c5c0.json")
    sub = json.load(io.open(sub_p, encoding="utf-8"))
    rows = {int(r["a"], 16): r for r in sub["rows"]}
    cg = {int(k): set(v) for k, v in json.load(io.open(os.path.join(HERE, "cg_exe.json"), encoding="utf-8")).items()}
    idx = RT.defidx()
    ai_syms = {s: RT.toks(s) for s in idx if "7game_ai" in s}
    short2sym = {}
    for s in idx: short2sym.setdefault(RT.short(s), s); short2sym[s] = s
    # rva -> sym (reach_tree 와 같은 규칙) — 지도 전체(651)에 대해
    h = io.open(r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust\AI함수지도.html", encoding="utf-8").read()
    fnd = {int(e["a"], 16): e for e in json.loads(re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S).group(1)) if not e["a"].startswith("spec-")}
    r2s = {}
    for x in json.load(io.open(os.path.join(HERE, "dllmatch.json"), encoding="utf-8")):
        if x["jaccard"] >= 0.6 and x["mangled"] in idx: r2s[x["rva"]] = x["mangled"]
    for s in json.load(io.open(os.path.join(HERE, "_spec", "specs20_v3.json"), encoding="utf-8"))["specs"]:
        ad = (s.get("exe") or {}).get("addr"); sym = s.get("sym")
        if ad and sym and sym in idx: r2s[int(ad, 16)] = sym
    for r, e in fnd.items():
        if r in r2s or not e.get("n") or e["n"] == "?": continue
        name = re.sub(r'\(.*', '', e["n"]).split("::")[-1].split(" ")[0]; mod = (e["m"] or "").split("/")[-1]
        c = [s for s, t in ai_syms.items() if t and t[-1] == name and mod in t]
        if not c: c = [s for s, t in ai_syms.items() if t and t[-1] == name]
        plain = [s for s in c if not re.search(r'(0|s\d*_0)B\w*_$', s) and not re.search(r'\d+_0E', s)]
        if len(plain) == 1: r2s[r] = plain[0]
        elif len(c) == 1: r2s[r] = c[0]
    for r, s in RT.OVERRIDE.items():
        if s in idx: r2s[r] = s
    s2r = {s: r for r, s in r2s.items()}
    unnamed = [r for r, row in rows.items() if row["name"] == "?" or r not in r2s]
    res = {}
    for r in unnamed:
        callers = [x for x, cs in cg.items() if r in cs]
        cands = collections.Counter(); notes = []
        for x in callers:
            xs = r2s.get(x)
            if not xs: notes.append("호출자 0x%x 심볼없음" % x); continue
            hh = hashlib.md5(xs.encode()).hexdigest()[:12]
            js = [os.path.join(OUT, f) for f in os.listdir(OUT) if f.startswith("rc_%s_" % hh)]
            if not js: notes.append("호출자 0x%x reach 캐시 없음" % x); continue
            j = json.load(io.open(js[0], encoding="utf-8"))
            ir_callees = {short2sym.get(rec["sym"], rec["sym"]) for rec in j["calls_live"] + j["calls_dead"]}
            ir_ai = {s for s in ir_callees if "7game_ai" in s and s in idx}
            exe_callees = cg.get(x, set())
            # 이름 붙은 짝 제거
            matched_syms = {r2s[c] for c in exe_callees if c in r2s}
            rest_ir = ir_ai - matched_syms
            rest_exe = {c for c in exe_callees if c not in r2s}
            for s in rest_ir:
                cands[s] += 1
            notes.append("호출자 0x%x(%s): exe 잔여 %d · IR 잔여 %d" % (x, RT.short(xs)[:30], len(rest_exe), len(rest_ir)))
        res[r] = (cands, notes)
    print(u"미명명/미연결 %d" % len(unnamed))
    out = {}
    for r in unnamed:
        cands, notes = res[r]; row = rows[r]
        mod = (row["mod"] or "").split("/")[-1]
        ranked = sorted(cands.items(), key=lambda kv: (-(kv[1] + (2 if mod in RT.toks(kv[0]) else 0)), kv[0]))
        print(u"\n0x%x %s [%s] %dB 호출자%d 콜리%d" % (r, row["name"], row["mod"], row["bytes"], row["callers"], row["callees"]))
        for n in notes: print(u"    " + n)
        for s, c in ranked[:6]:
            print(u"    %s%d  %s" % ("★" if mod in RT.toks(s) else " ", c, RT.short(s)[:140]))
        if ranked: out["%x" % r] = [[s, c, mod in RT.toks(s)] for s, c in ranked[:6]]
    if "--apply" in sys.argv:
        json.dump(out, io.open(os.path.join(OUT, "names_bycaller.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print(u"-> names_bycaller.json")
    return 0


if __name__ == "__main__":
    sys.exit(main())
