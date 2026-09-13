#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""reach.py — IR 한 함수의 CFG 에서 「알려진 상수 조건」을 접어 사장 블록·사장 호출부를 가른다 (교훈 68 도구화 · 2026-09-13)

왜: 큰 함수(예 `LegacyPlanHandler::update` 14,476줄)는 분기 도달 가능성부터 갈라야 작업량이 준다(tower_dive 1/4).
    지금까지는 손으로 IR 을 읽었다. 이 도구는 ①`irann.py` 주석본을 블록/간선으로 파싱 ②`--const %N=0|1` 로 준
    i1 값(예 version 비교)과 `--tag <SSA>=<정수>` 로 준 switch/eq 값을 접어 ③진입 블록에서 도달 불가한 블록을 구하고
    ④그 안의 call/invoke 를 「사장 호출부」로, 살아있는 호출부는 「지배 분기 조건 체인」과 함께 출력한다.

사용:
    python -X utf8 MIG\\reach.py <annotated.ll> [--const %1714=0 --const %3917=1 ...] [--eq %218=1 ...] [--json out.json]
    (`--eq %v=K` : `icmp eq i64 %v, K` 는 K 일 때만 참 — 다른 상수와의 eq 는 거짓으로 접는다)
    version 비교 SSA 는 `vbr.py <ll> <define줄>` 로 먼저 뽑는다(alias 는 alloca 재로드까지 손으로 확인).

출력: 살아있는 호출부(콜리 심볼·소스줄·지배조건) / 사장 호출부 / 콜리별 요약(전부 사장 = NA 후보).
한계: 상수 접기는 br i1 과 switch 의 직접 피연산자만. `and/or/select/phi` 전파 없음 — 필요하면 --const 를 더 준다.
"""
import io, re, sys, json, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

LABEL = re.compile(r'^(\d+|[A-Za-z_.][\w.]*):')
BR2 = re.compile(r'^\s*br i1 (%\d+|true|false), label %(\S+), label %(\S+)')
BR1 = re.compile(r'^\s*br label %(\S+)')
SW = re.compile(r'^\s*switch (\w+) (%\d+|\d+), label %(\S+) \[(.*?)\]')
SWCASE = re.compile(r'\w+ (-?\d+), label %(\S+)')
INVOKE = re.compile(r'to label %([\w.]+) unwind label %([\w.]+)')   # 09-14 21차: `\S+` 가 `%4214,` 의 쉼표까지 먹어 20블록을 사장으로 오판
CALLSYM = re.compile(r'(?:call|invoke)\b[^@]*@([^\s(]+)')
CMPEQ = re.compile(r'^\s*(%\d+) = icmp (eq|ne) i64 (%\d+), (-?\d+)')
CMPRNG = re.compile(r'^\s*(%\d+) = icmp (ult|ugt|ule|uge|slt|sgt|sle|sge) i64 (%\d+), (-?\d+)')
DEF = re.compile(r'^define .*@(\S+)\(')


def parse(path):
    lines = io.open(path, encoding="utf-8", errors="replace").read().split("\n")
    blocks = collections.OrderedDict()   # name -> {'lines': [(lineno, text)], 'succ': [(target, cond)], 'term': str}
    cur = None; fname = None
    for i, ln in enumerate(lines, 1):
        m = DEF.match(ln)
        if m:
            fname = m.group(1); cur = "entry"; blocks[cur] = {"lines": [], "succ": [], "term": ""}; continue
        if ln.startswith("}"):
            break
        m = LABEL.match(ln)
        if m and cur is not None:
            cur = m.group(1); blocks[cur] = {"lines": [], "succ": [], "term": ""}; continue
        if cur is None:
            continue
        blocks[cur]["lines"].append((i, ln))
    return fname, blocks, lines


def edges(blocks, consts, eqs, cmpdefs):
    u"""각 블록의 후속 간선을 만든다. consts: {'%c': 0/1}. eqs: {'%v': K} → `icmp eq %v, K'` 를 K==K' 로 접음."""
    folded = {}
    for name, b in blocks.items():
        succ = []
        merged = []   # invoke 의 `to label` 후속줄 · switch 의 case 줄들을 한 줄로 합친다
        k = 0; L = b["lines"]
        while k < len(L):
            i, ln = L[k]; s = ln.strip(); k += 1
            if (s.startswith("invoke") or "= invoke" in s) and "to label" not in s:
                while k < len(L) and "to label" not in s:
                    s += " " + L[k][1].strip(); k += 1
            elif s.startswith("switch") and "]" not in s:
                while k < len(L) and "]" not in s:
                    s += " " + L[k][1].strip(); k += 1
            merged.append((i, s))
        for i, s in merged:
            m = BR2.match(s)
            if m:
                c, t, f = m.groups()
                v = None
                if c == "true": v = 1
                elif c == "false": v = 0
                elif c in consts: v = consts[c]
                elif c in cmpdefs:
                    op, src, k = cmpdefs[c]
                    if src in eqs:
                        val = eqs[src]
                        if op == "eq": v = 1 if val == k else 0
                        elif op == "ne": v = 0 if val == k else 1
                        elif op in ("ult", "slt"): v = 1 if val < k else 0
                        elif op in ("ugt", "sgt"): v = 1 if val > k else 0
                        elif op in ("ule", "sle"): v = 1 if val <= k else 0
                        elif op in ("uge", "sge"): v = 1 if val >= k else 0
                if v is None:
                    succ.append((t, "%s=1" % c)); succ.append((f, "%s=0" % c))
                else:
                    folded[(name, c)] = v
                    succ.append((t if v else f, "%s==%d(접힘)" % (c, v)))
                b["term"] = s; break
            m = BR1.match(s)
            if m:
                succ.append((m.group(1), "")); b["term"] = s; break
            m = SW.match(s)
            if m:
                _, var, dflt, cases = m.groups()
                cs = SWCASE.findall(cases)
                if var in eqs:
                    val = eqs[var]; hit = [t for k, t in cs if int(k) == val]
                    succ.append((hit[0] if hit else dflt, "%s==%d(switch접힘)" % (var, val)))
                    folded[(name, var)] = val
                else:
                    succ.append((dflt, "%s=default" % var))
                    for k, t in cs: succ.append((t, "%s=%s" % (var, k)))
                b["term"] = s; break
            m = INVOKE.search(s)
            if m and (s.startswith("invoke") or "= invoke" in s):
                succ.append((m.group(1), "")); succ.append((m.group(2), "unwind"))
                b["term"] = s; break
            if s.startswith("ret ") or s == "ret void" or s.startswith("unreachable") or s.startswith("resume "):
                b["term"] = s; break
            if s.startswith("cleanupret") or s.startswith("catchret"):
                mm = re.search(r'to label %(\S+)', s)
                if mm: succ.append((mm.group(1), ""))
                b["term"] = s; break
            if s.startswith("catchswitch"):
                for t in re.findall(r'label %(\S+)', s): succ.append((t, "eh"))
                b["term"] = s; break
        b["succ"] = succ
    return folded


def reachable(blocks, entry="entry"):
    seen = set(); st = [entry]
    while st:
        n = st.pop()
        if n in seen or n not in blocks: continue
        seen.add(n)
        for t, _ in blocks[n]["succ"]: st.append(t)
    return seen


def dominators(blocks, live, entry="entry"):
    u"""단순 반복 지배자 계산(살아있는 블록만)."""
    preds = collections.defaultdict(set)
    for n in live:
        for t, _ in blocks[n]["succ"]:
            if t in live: preds[t].add(n)
    dom = {n: set(live) for n in live}; dom[entry] = {entry}
    order = list(live)
    changed = True
    while changed:
        changed = False
        for n in order:
            if n == entry: continue
            ps = [dom[p] for p in preds[n] if p in dom]
            new = (set.intersection(*ps) if ps else set()) | {n}
            if new != dom[n]:
                dom[n] = new; changed = True
    return dom, preds


def cond_chain(blocks, dom, preds, n, entry="entry"):
    u"""n 을 지배하는 블록들 중 조건 분기(br i1/switch)에서 n 쪽으로만 가는 간선의 조건 = 반드시 지난 조건."""
    out = []
    for d in dom[n]:
        if d == n: continue
        b = blocks[d]
        conds = [(t, c) for t, c in b["succ"] if c and not c.endswith("(접힘)")]
        if len(conds) < 2: continue
        # d 의 후속 중 n 을 지배-경유하는 것: t 가 n 의 지배자이거나 t==n
        via = [c for t, c in conds if t == n or (t in dom.get(n, ()) )]
        if len(via) == 1:
            out.append(via[0])
    return out


def main():
    a = sys.argv[1:]
    if not a: print(__doc__); return 2
    path = a[0]; consts = {}; eqs = {}; jout = None; ver = None; gmode = None
    i = 1
    while i < len(a):
        if a[i] == "--const": k, v = a[i+1].split("="); consts[k] = int(v); i += 2
        elif a[i] == "--eq": k, v = a[i+1].split("="); eqs[k] = int(v); i += 2
        elif a[i] == "--json": jout = a[i+1]; i += 2
        elif a[i] == "--version": ver = int(a[i+1]); i += 2
        elif a[i] == "--gamemode": gmode = int(a[i+1]); i += 2
        else: i += 1
    fname, blocks, lines = parse(path)
    if ver is not None:
        # irann 의 `;; version = i64 %N` 바인딩 전부를 version 값으로 접는다
        for ln in lines:
            m = re.match(r'\s*;; version = i64 (%\d+)$', ln.strip()) or re.match(r'\s*;; version = i64 (%\d+)', ln)
            if m: eqs.setdefault(m.group(1), ver)
    if gmode is not None:
        # get_game_mode = `dyn AbstractGame` vtable 슬롯 +64 · 반환 { i64 tag, ptr } · tag 0=Moba 1=SingleLane 2=DeathMatch (tcxdict --enum GameMode)
        defs = {}
        for ln in lines:
            m = re.match(r'\s*(%\d+) = (.*)', ln)
            if m: defs[m.group(1)] = m.group(2)
        def is_slot64(f):
            d = defs.get(f, "")
            m = re.match(r'load ptr, ptr (%\d+)', d)
            if not m: return False
            g = defs.get(m.group(1), "")
            return re.match(r'gep (%\d+), i64 64(?!\d)', g) is not None
        calls = set()
        for v, d in defs.items():
            m = re.match(r'(?:tail )?call \{ i64, ptr \} (%\d+)\(', d)
            if m and is_slot64(m.group(1)): calls.add(v)
        for v, d in defs.items():
            m = re.match(r'extractvalue \{ i64, ptr \} (%\d+), 0', d)
            if m and m.group(1) in calls: eqs.setdefault(v, gmode)
    cmpdefs = {}
    for name, b in blocks.items():
        for _, ln in b["lines"]:
            m = CMPEQ.match(ln) or CMPRNG.match(ln)
            if m: cmpdefs[m.group(1)] = (m.group(2), m.group(3), int(m.group(4)))
    folded = edges(blocks, consts, eqs, cmpdefs)
    live = reachable(blocks)
    dead = [n for n in blocks if n not in live]
    dom, preds = dominators(blocks, live)
    print(u"함수 %s · 블록 %d · 살아있음 %d · 사장 %d · 접힌 분기 %d" % (fname, len(blocks), len(live), len(dead), len(folded)))
    for (bn, c), v in sorted(folded.items()): print(u"  접힘 @%s: %s -> %s" % (bn, c, v))
    calls_live = []; calls_dead = []
    for name, b in blocks.items():
        for lno, ln in b["lines"]:
            m = CALLSYM.search(ln)
            if not m: continue
            sym = m.group(1)
            if sym.startswith("llvm.") or sym.startswith("__CxxFrameHandler"): continue
            loc = re.search(r';L(\S+)$', ln); loc = loc.group(1) if loc else ""
            rec = {"line": lno, "block": name, "sym": sym, "loc": loc}
            if name in live:
                rec["conds"] = cond_chain(blocks, dom, preds, name)
                calls_live.append(rec)
            else:
                calls_dead.append(rec)
    by = collections.defaultdict(lambda: [0, 0])
    for r in calls_live: by[r["sym"]][0] += 1
    for r in calls_dead: by[r["sym"]][1] += 1
    print(u"\n## 콜리 요약 (live/dead) — dead 만 있는 콜리 = NA 후보")
    for sym, (l, d) in sorted(by.items(), key=lambda x: (x[1][0] > 0, x[0])):
        print(u"  %s  live=%d dead=%d  %s" % ("NA?" if l == 0 else "   ", l, d, sym))
    print(u"\n## 사장 호출부 (%d)" % len(calls_dead))
    for r in calls_dead: print(u"  L%d @%s %s ;L%s" % (r["line"], r["block"], r["sym"], r["loc"]))
    print(u"\n## 살아있는 호출부 (%d) — 지배 조건" % len(calls_live))
    for r in calls_live: print(u"  L%d @%s %s ;L%s  <= %s" % (r["line"], r["block"], r["sym"], r["loc"], " & ".join(r["conds"]) or "(무조건)"))
    if jout:
        json.dump({"fn": fname, "folded": [[k[0], k[1], v] for k, v in folded.items()], "dead_blocks": dead,
                   "calls_live": calls_live, "calls_dead": calls_dead}, io.open(jout, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print(u"-> %s" % jout)
    return 0


if __name__ == "__main__":
    sys.exit(main())
