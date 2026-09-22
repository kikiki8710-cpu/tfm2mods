"""paths from root's direct callees (by IR line) to TLS-touching functions (shortest), using tls3.json"""
import io, json, re, collections, sys
SP = r"C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad\scratchpad25B"
d = json.load(io.open(SP + r"\tls3.json", encoding="utf-8"))
hits = d["hits"]; callees = d["callees"]
calls = json.load(io.open(SP + r"\calls.json", encoding="utf-8"))
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
SKIP=("game_ai","game_core","core","std","bumpalo","alloc","rand","option","Option","ops","function","impls","closure","iter","traits","iterator","adapters","Iterator","FnMut","FnOnce","call_mut","call_once","try_fold","fold","any","check","map","Map","filter","Filter","slice","Iter","ref","mut","vec","Vec")
def short(s):
    out=[t for t in v0toks(s) if t not in SKIP]
    return "::".join(out[-3:])
def bfs_paths(start):
    prev={start:None}; q=collections.deque([start])
    while q:
        x=q.popleft()
        for c in callees.get(x,()):
            if c in callees and c not in prev:
                prev[c]=x; q.append(c)
    return prev
def path(prev, t):
    p=[];
    while t is not None: p.append(t); t=prev[t]
    return list(reversed(p))
want = sys.argv[1:]
for ln, root, chain, sym in calls:
    if sym not in callees: continue
    if want and not any(w in sym for w in want): continue
    prev = bfs_paths(sym)
    found = [(t, tl) for t, tl in hits.items() if t in prev]
    if not found: continue
    print("### %d root L%s chain %s  %s" % (ln, root, chain, short(sym)))
    seen=set()
    for t, tl in sorted(found, key=lambda x: len(path(prev, x[0]))):
        key=tuple(tl)
        p = path(prev, t)
        print("   ", ", ".join(tl), "  <=  ", " > ".join(short(x) for x in p[1:]) or "(self)")
