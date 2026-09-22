"""TLS scan v3: full-depth BFS over game_ai defines; TLS identified by LocalKey::with closure owner; per-direct-callee forward closure."""
import io, re, json, os, collections, sys
SP = r"C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad\scratchpad25B"
CORP = r"C:\tfm2mods\_gaibc"
idx = json.load(io.open(r"C:\tfm2mods\MIG\_next\reach\defidx.json", encoding="utf-8"))
ROOT = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12line_defenseNtB2_18LineDefenseSubPlan17action_candidates"
MAXD = int(sys.argv[1]) if len(sys.argv) > 1 else 12
files = {}
def body(sym):
    e = idx.get(sym)
    if not e: return None
    fn, s, t = e
    if fn not in files:
        files[fn] = io.open(os.path.join(CORP, fn), encoding="utf-8", errors="replace").read().split("\n")
    return files[fn][s-1:t]
CALLRE = re.compile(r"(?:call|invoke)\s+[^@]*@([\w\.\$]+)\(")
WITHRE = re.compile(r"@(_R\w*?8LocalKeyI(.*?)EE4with(\w*?))\(")
TLADDR = re.compile(r"llvm\.threadlocal\.address\.p0\(ptr @([\w\.\$]+)\)")
def v0toks(sym):
    out=[]; i=0; n=len(sym)
    while i<n:
        if sym[i].isdigit():
            j=i
            while j<n and sym[j].isdigit(): j+=1
            k=int(sym[i:j])
            if 0<k<=n-j and j<n and (sym[j].isalpha() or sym[j]=='_'):
                out.append(sym[j:j+k]); i=j+k; continue
            i=j
        else: i+=1
    return out
def tlsid(m):
    ty = v0toks(m.group(2)); own = v0toks(m.group(3))
    own = [t for t in own if t not in ("game_ai","game_core","core","std","option","Option")]
    return "%s@%s" % (ty[-1] if ty else "?", "::".join(own[:2]))
seen = {ROOT: 0}; parent = {ROOT: None}
q = collections.deque([ROOT])
hits = collections.defaultdict(set)  # sym -> set(tlsid)
callees = {}
while q:
    s = q.popleft(); d = seen[s]
    b = body(s)
    if b is None: continue
    cs = []
    for l in b:
        if "@" not in l: continue
        m = WITHRE.search(l)
        if m and ("call" in l or "invoke" in l):
            hits[s].add(tlsid(m))
        m2 = TLADDR.search(l)
        if m2:
            hits[s].add("TLADDR:" + "::".join(v0toks(m2.group(1))[-2:]))
        m = CALLRE.search(l)
        if m:
            c = m.group(1)
            if c.startswith("llvm."): continue
            cs.append(c)
            if d < MAXD and c not in seen and c in idx:
                seen[c] = d + 1; parent[c] = s; q.append(c)
    callees[s] = cs
def short(s):
    out=[t for t in v0toks(s) if t not in ("game_ai","game_core","core","std","bumpalo","alloc","rand","option","Option")]
    return "::".join(out[-3:])
print("visited", len(seen), "fns with TLS", len(hits))
# forward closure per direct callee
def closure(start):
    st=[start]; vis=set(); tl=set(); via={}
    while st:
        x=st.pop()
        if x in vis: continue
        vis.add(x)
        for t in hits.get(x,()):
            tl.add(t); via.setdefault(t, x)
        for c in callees.get(x,()):
            if c in callees and c not in vis: st.append(c)
    return tl, via
print("== direct callees (game_ai, in IR order of first appearance) and transitive TLS ==")
gc_decl = set()
for c in dict.fromkeys(callees[ROOT]):
    if c not in idx:
        if "9game_core" in c: gc_decl.add(c)
        continue
    tl, via = closure(c)
    name = short(c)
    if tl:
        print("  ", name)
        for t in sorted(tl): print("       ", t, "  (in", short(via[t]) + ")")
    else:
        print("  ", name, "-> TLS none")
print("== game_core callees (declared only; need _gcbc) ==")
for c in sorted(gc_decl): print("  ", short(c), c)
json.dump({"hits": {k: sorted(v) for k, v in hits.items()}, "callees": callees, "seen": seen}, io.open(SP + r"\tls3.json", "w", encoding="utf-8"))
