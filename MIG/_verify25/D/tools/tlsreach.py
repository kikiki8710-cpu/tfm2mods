"""tlsreach.py — action_candidates 의 직접 콜 사이트(IR 줄 순) 별 전이적 TLS 도달 집합 + 최단 경로."""
import io, re, json, sys
from collections import deque
OUT = r"C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad\scratchpad25D"
cg = json.load(io.open(OUT + r"\cg.json", encoding="utf-8"))
G = cg["graph"]; T = cg["tls"]; TG = cg["tlsglob"]
def lk_type(sym):
    """LocalKey<RefCell<T>>::with<..> 심볼에서 T 이름을 뽑는다(길이 접두 식별자 중 4with 앞 마지막)."""
    head = sym.split("4with")[0]
    toks = []
    i = 0
    while i < len(head):
        m = re.match(r"(\d+)([A-Za-z_])", head[i:])
        if m:
            n = int(m.group(1)); s = i + len(m.group(1))
            ident = head[s:s + n]
            if re.fullmatch(r"[A-Za-z_]\w*", ident or "x"):
                toks.append(ident); i = s + n; continue
        i += 1
    return toks[-1] if toks else sym[:60]
# LocalKey::with 콜리를 TLS 참조로 승격(전역은 @anon 상수 경유 간접호출이라 이름 그래프에 안 잡힌다)
for fn, cs in G.items():
    for c in cs:
        if "8LocalKey" in c and "4with" in c:
            T.setdefault(fn, []).append("LK:" + lk_type(c))
def short(g):
    if g.startswith("LK:"): return g[3:]
    toks = re.findall(r"\d+([A-Z][A-Z0-9_]+)", g)
    toks = [t for t in toks if t != "RUST_STD_INTERNAL_VAL"]
    if toks: return toks[-1]
    if "thread_local6native4lazy" in g: return "std::lazy::Storage"
    return g[-40:]
# reachable tls with path (BFS, depth cap 12)
memo = {}
def reach(fn, cap=12):
    if fn in memo: return memo[fn]
    res = {}
    seen = {fn: None}
    dq = deque([(fn, 0)])
    while dq:
        cur, d = dq.popleft()
        for g in T.get(cur, []):
            k = short(g)
            if k not in res:
                # path
                p = []; x = cur
                while x is not None: p.append(x); x = seen[x]
                res[k] = list(reversed(p))
        if d >= cap: continue
        for c in G.get(cur, []):
            if c not in seen:
                seen[c] = cur; dq.append((c, d + 1))
    memo[fn] = res
    return res
def nice(fn):
    m = re.search(r"game_ai(\w+)$|game_core(\w+)$", fn)
    s = fn
    s = re.sub(r"^_R[A-Za-z]*Nt[A-Za-z0-9_]*?7game_ai", "ai:", s)
    s = re.sub(r"^_R[A-Za-z]*Nt[A-Za-z0-9_]*?9game_core", "gc:", s)
    return s[:120]
root = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9epic_huntNtB2_15EpicHuntSubPlan17action_candidates"
# ordered call sites from fn.ll
lines = io.open(OUT + r"\fn.ll", encoding="utf-8").read().split("\n")
ann = {}
for l in io.open(r"C:\tfm2mods\MIG\_next\reach\eaeda0.ll", encoding="utf-8", errors="replace"):
    m = re.match(r"\s*(\d+)\|(.*?)(?:;L(\S+))?\s*$", l.rstrip("\n"))
    if m: ann[int(m.group(1))] = m.group(3)
keys = sorted(ann)
def rootline(n):
    c = ann.get(n)
    if c is None and (n + 1) in ann: c = ann.get(n + 1)
    return c.split("<")[-1] if c else "?"
rows = []
for l in lines:
    m = re.match(r"(\d+)\| (.*)$", l)
    if not m: continue
    n = int(m.group(1)); body = m.group(2)
    if "call" not in body and "invoke" not in body: continue
    mm = re.search(r"(?:call|invoke)\b[^@]*@([\w.$]+)\(", body)
    if not mm: continue
    callee = mm.group(1)
    if callee.startswith("llvm.") or "drop_glue" in callee or "unwrap_failed" in callee or "panic" in callee: continue
    r = reach(callee)
    if not r and "SmallAction" in callee and ("bumpalo" in callee or "raw_vec" in callee or "alloc" in callee): continue
    rows.append((n, rootline(n), nice(callee), r))
print("# 직접 콜 사이트(IR 줄 순) · 루트 소스줄 · 콜리 · 전이 도달 TLS")
for n, rl, c, r in rows:
    print("%6d L%-4s %-100s %s" % (n, rl, c, ", ".join(sorted(r)) if r else "-"))
print()
print("# TLS 별 최단 경로 예시 (직접 콜리 기준)")
shown = set()
for n, rl, c, r in rows:
    for k, p in r.items():
        if (c, k) in shown: continue
        shown.add((c, k))
        print("L%s %s -> %s : %s" % (rl, c[:60], k, " > ".join(nice(x)[:70] for x in p)))
