#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""reach_tree.py — 루트 함수에서 IR 호출 그래프를 따라가며 「도달 가능성」을 봉인한다 (교훈 68 · 2026-09-13)

원리: 루트(`update`)의 IR 심볼부터 BFS. 각 함수를 `irann.py`(주석) → `reach.py`(CFG 접기: version=2 ·
      get_game_mode=Moba)로 처리해 **살아있는 호출부의 콜리만** 다음 단계로 간다(IR 에 define 이 있는 game_ai 심볼 전부 —
      exe 에서 인라인된 중간 함수도 따라가야 exe 콜그래프의 간선이 재현된다). 같은 BFS 를 **접기 없이** 한 번 더 돌려
      「무제약 도달 집합」을 만들고, 두 집합의 차 = **NA(사장)**.
      서브트리 선별표의 RVA 는 dllmatch(jaccard≥0.6) · specs20_v3 · 이름+모듈 유일매칭 · 손 오버라이드(`OVERRIDE`)로 심볼에 잇는다.

사용: python -X utf8 MIG\\reach_tree.py [subtree json=_next\\subtree_e4c5c0.json] [--mode 0] [--version 2] [--max 800]
산출: `_next\\reach\\tree_<rva>.json` + 콘솔 표(NA / 경로없음 / 미확인 / live)
"""
import io, os, re, sys, json, subprocess, collections, hashlib

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
CORP = r"C:\tfm2mods\_gaibc"
OUT = os.path.join(HERE, "_next", "reach")
DEFRE = re.compile(rb"^define[^@]*@([A-Za-z0-9_$.]+)\(")
TOK = re.compile(r'(\d+)([A-Za-z_][A-Za-z0-9_]*)')

# RVA -> IR 심볼 손 오버라이드(근거: 원장 §1 · DONE.md · reach 출력 대조). 자동 매칭이 모호한 것만.
OVERRIDE = {
    0xcaf9f0: "_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy5typesNtB2_7BigPlan8sub_plan",           # 원장 #02 호스트
    0xdefa20: "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic13hunt_and_pokeNtB2_19EpicHuntAndPokePlan6is_end",   # 원장 #08
    0xdf0a90: "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen13hunt_and_pokeNtB2_21SerpenHuntAndPokePlan6is_end",
    0xdefcd0: "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic13hunt_and_pokeNtB2_19EpicHuntAndPokePlan8sub_plan",   # DONE epic_hunt_poke
    0xdf0e90: "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen13hunt_and_pokeNtB2_21SerpenHuntAndPokePlan8sub_plan",  # DONE serpen_hunt_poke
    0xccc010: "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic15hunt_and_battleNtB2_21EpicHuntAndBattlePlan8sub_plan",  # 원장 #07
    0xccc3c0: "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen15hunt_and_battleNtB2_23SerpenHuntAndBattlePlan8sub_plan",  # DONE SERPEN_HUNT_BATTLE
    0xe7a8c0: "_RNvCshdEBA0ozCnw_7game_ai12upgrade_item",   # DONE judge upgrade_item(lib.rs)
    0xe7b640: "_RNvCshdEBA0ozCnw_7game_ai8buy_item",
}


def defidx():
    p = os.path.join(OUT, "defidx.json")
    if os.path.exists(p):
        return json.load(io.open(p, encoding="utf-8"))
    idx = {}
    for fn in sorted(os.listdir(CORP)):
        if not fn.endswith(".ll"): continue
        with io.open(os.path.join(CORP, fn), "rb") as f:
            cur = None
            for i, raw in enumerate(f, 1):
                if raw.startswith(b"define"):
                    m = DEFRE.match(raw); cur = m.group(1).decode() if m else None
                    if cur: idx[cur] = [fn, i, None]
                elif cur and raw.startswith(b"}"):
                    idx[cur][2] = i; cur = None
    json.dump(idx, io.open(p, "w", encoding="utf-8"))
    return idx


def short(sym):
    u"""irann 의 SYM 축약과 같은 규칙 — reach 출력 심볼과 맞추기 위해."""
    for rx, rep in (
        (re.compile(r'^_RNv[A-Za-z0-9_]*?7game_ai([0-9]+)([A-Za-z0-9_]+)'), lambda m: 'ai::' + m.group(2)),
        (re.compile(r'^_RNv[A-Za-z0-9_]*?9game_core([0-9]+)([A-Za-z0-9_]+)'), lambda m: 'gc::' + m.group(2)),
        (re.compile(r'^_RINv[A-Za-z0-9_]*?4core([0-9]+)([A-Za-z0-9_]+)'), lambda m: 'core::' + m.group(2)),
        (re.compile(r'^_RNv[A-Za-z0-9_]*?4core([0-9]+)([A-Za-z0-9_]+)'), lambda m: 'core::' + m.group(2)),
    ):
        s2 = rx.sub(rep, sym)
        if s2 != sym: return s2
    return sym


def toks(sym):
    out = []; i = 0
    while i < len(sym):
        m = TOK.match(sym, i)
        if m:
            n = int(m.group(1)); ident = sym[m.end(1):m.end(1) + n]
            if len(ident) == n and re.match(r'^[A-Za-z_][A-Za-z0-9_]*$', ident):
                out.append(ident); i = m.end(1) + n; continue
        i += 1
    return out


def analyze(sym, idx, mode, ver):
    u"""심볼 하나: irann → reach. 반환 (live callee shorts, dead callee shorts, 사장블록수, 접힘) · 캐시는 파일."""
    fn, s, e = idx[sym]
    h = hashlib.md5(sym.encode()).hexdigest()[:12]
    ann = os.path.join(OUT, "ir_%s.ll" % h)
    if not os.path.exists(ann) or os.path.getsize(ann) == 0:
        subprocess.run([sys.executable, "-X", "utf8", os.path.join(HERE, "irann.py"), os.path.join(CORP, fn), "@" + sym + "(", "-o", ann], capture_output=True)
    tag = "m%s_v%s" % (mode if mode is not None else "x", ver if ver is not None else "x")
    js = os.path.join(OUT, "rc_%s_%s.json" % (h, tag))
    if not os.path.exists(js):
        args = [sys.executable, "-X", "utf8", os.path.join(HERE, "reach.py"), ann, "--json", js]
        if mode is not None: args += ["--gamemode", str(mode)]
        if ver is not None: args += ["--version", str(ver)]
        subprocess.run(args, capture_output=True)
    if not os.path.exists(js):
        return None
    j = json.load(io.open(js, encoding="utf-8"))
    live = collections.Counter(r["sym"] for r in j["calls_live"])
    dead = collections.Counter(r["sym"] for r in j["calls_dead"])
    return live, dead, len(j["dead_blocks"]), j["folded"]


def bfs(root_sym, idx, short2sym, mode, ver, limit):
    seen = {}; q = [root_sym]; edges = collections.defaultdict(set); deadedges = collections.defaultdict(set)
    while q and len(seen) < limit:
        s = q.pop(0)
        if s in seen: continue
        r = analyze(s, idx, mode, ver)
        seen[s] = r
        if r is None: continue
        live, dead, _, _ = r
        for c in live:
            full = short2sym.get(c)
            if full and full in idx:
                edges[full].add(s)
                if full not in seen: q.append(full)
        for c in dead:
            full = short2sym.get(c)
            if full: deadedges[full].add(s)
    return seen, edges, deadedges


def main():
    a = [x for x in sys.argv[1:] if x.endswith(".json")]
    sub_p = a[0] if a else os.path.join(HERE, "_next", "subtree_e4c5c0.json")
    mode = int(sys.argv[sys.argv.index("--mode") + 1]) if "--mode" in sys.argv else 0
    ver = int(sys.argv[sys.argv.index("--version") + 1]) if "--version" in sys.argv else 2
    limit = int(sys.argv[sys.argv.index("--max") + 1]) if "--max" in sys.argv else 6000
    sub = json.load(io.open(sub_p, encoding="utf-8"))
    root = int(sub["root"], 16) if isinstance(sub["root"], str) else sub["root"]
    rows = {int(r["a"], 16): r for r in sub["rows"]}
    idx = defidx()
    ai_syms = {s: toks(s) for s in idx if "7game_ai" in s}
    short2sym = {}
    for s in idx:
        short2sym.setdefault(short(s), s); short2sym[s] = s
    # rva -> sym
    r2s = {}; how = {}
    for x in json.load(io.open(os.path.join(HERE, "dllmatch.json"), encoding="utf-8")):
        if x["jaccard"] >= 0.6 and x["mangled"] in idx: r2s[x["rva"]] = x["mangled"]; how[x["rva"]] = "dllmatch%.2f" % x["jaccard"]
    for s in json.load(io.open(os.path.join(HERE, "_spec", "specs20_v3.json"), encoding="utf-8"))["specs"]:
        ad = (s.get("exe") or {}).get("addr"); sym = s.get("sym")
        if ad and sym and sym in idx: r2s[int(ad, 16)] = sym; how[int(ad, 16)] = "spec"
    for r, row in rows.items():
        if r in r2s or row["name"] == "?": continue
        name = re.sub(r'\(.*', '', row["name"]).split("::")[-1].split(" ")[0]; mod = row["mod"].split("/")[-1]
        c = [s for s, t in ai_syms.items() if t and t[-1] == name and mod in t]
        if not c: c = [s for s, t in ai_syms.items() if t and t[-1] == name]
        plain = [s for s in c if not re.search(r'(0|s\d*_0)B\w*_$', s) and not re.search(r'\d+_0E', s)]   # 클로저/모노모프 제외
        if len(plain) == 1: r2s[r] = plain[0]; how[r] = "name+mod"
        elif len(c) == 1: r2s[r] = c[0]; how[r] = "name"
    for r, s in OVERRIDE.items():
        if s in idx: r2s[r] = s; how[r] = "override"
    root_sym = r2s.get(root)
    if not root_sym:
        print(u"루트 심볼 없음"); return 1
    print(u"루트 %s · 제약 BFS(mode=%s version=%s) …" % (short(root_sym), mode, ver))
    seenC, edgesC, deadC = bfs(root_sym, idx, short2sym, mode, ver, limit)
    print(u"  제약 도달 %d 함수" % len(seenC))
    seenU, edgesU, deadU = bfs(root_sym, idx, short2sym, None, None, limit)
    print(u"  무제약 도달 %d 함수" % len(seenU))
    liveset = set(seenC); allset = set(seenU)
    out = []
    for r, row in rows.items():
        if r == root: continue
        s = r2s.get(r)
        if not s: st = "미확인(심볼 미연결)"
        elif s in liveset: st = "live"
        elif s in allset: st = "NA(사장)"
        else: st = "경로없음(IR 그래프 밖·exe cg 불일치)"
        out.append((st, r, row, s))
    order = {"NA(사장)": 0, "경로없음(IR 그래프 밖·exe cg 불일치)": 1, "미확인(심볼 미연결)": 2, "live": 3}
    out.sort(key=lambda x: (order[x[0]], x[2]["kind"], -x[2]["callers"]))
    cnt = collections.Counter(x[0] for x in out)
    print(u"\n루트 0x%x · 서브트리 %d · 판정: %s" % (root, len(out), dict(cnt)))
    for st, r, row, s in out:
        if st == "live": continue
        via = ""
        if st.startswith("NA"):
            via = u" ← 사장 호출부: " + ", ".join(short(x)[:40] for x in list(deadC.get(s, []))[:3])
        print(u"  %-28s 0x%x %-42s %-20s %5dB 호출자%d %s [%s]%s" % (st, r, row["name"][:42], row["mod"], row["bytes"], row["callers"], row["kind"], how.get(r, ""), via))
    json.dump({"root": "%x" % root, "mode": mode, "version": ver,
               "verdict": [{"st": st, "rva": "%x" % r, "name": row["name"], "mod": row["mod"], "kind": row["kind"], "sym": s, "how": how.get(r)} for st, r, row, s in out],
               "live_syms": sorted(liveset), "all_syms": sorted(allset),
               "dead_edges": {k: sorted(v) for k, v in deadC.items() if k not in liveset},
               "folded": {s: seenC[s][3] for s in seenC if seenC[s] and seenC[s][3]}},
              io.open(os.path.join(OUT, "tree_%x.json" % root), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(u"-> %s" % os.path.join(OUT, "tree_%x.json" % root))
    return 0


if __name__ == "__main__":
    sys.exit(main())
