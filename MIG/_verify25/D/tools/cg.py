"""cg.py — game_ai(_gaibc) + game_core 일부(_gcbc g07 camp_pos 등) IR 에서 define→callee 그래프와 TLS 전역 참조를 뽑아 cg.json 으로 저장."""
import io, re, os, json, glob, sys, time
OUT = r"C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad\scratchpad25D"
files = sorted(glob.glob(r"C:\tfm2mods\_gaibc\m*.ll")) + sorted(glob.glob(r"C:\tfm2mods\_gcbc\g*.ll"))
graph = {}      # fn -> set(callees)
tlsref = {}     # fn -> set(tls globals)
tlsglob = {}    # global name -> file
t0 = time.time()
callre = re.compile(r"(?:call|invoke)\b[^@]*@([\w.$]+)\(")
globre = re.compile(r"@(_[\w.$]*(?:__RUST_STD_INTERNAL_VAL|MEMO|CACHE|CTX|BEAMS|GRID)[\w.$]*)")
for f in files:
    cur = None
    with io.open(f, encoding="utf-8", errors="replace") as fh:
        for l in fh:
            if l.startswith("define"):
                m = re.search(r"@([\w.$]+)\(", l)
                cur = m.group(1) if m else None
                if cur is not None:
                    graph.setdefault(cur, set()); tlsref.setdefault(cur, set())
                continue
            if l.startswith("}"):
                cur = None; continue
            if l.startswith("@") and "thread_local" in l:
                m = re.match(r"@([\w.$]+) =", l)
                if m: tlsglob[m.group(1)] = os.path.basename(f)
                continue
            if cur is None: continue
            if "call" in l or "invoke" in l:
                m = callre.search(l)
                if m and not m.group(1).startswith("llvm."):
                    graph[cur].add(m.group(1))
            if "__RUST_STD_INTERNAL_VAL" in l or "MEMO" in l or "CACHE" in l or "CTX" in l or "BEAMS" in l or "GRID" in l:
                for g in globre.findall(l):
                    tlsref[cur].add(g)
    print(os.path.basename(f), len(graph), "%.0fs" % (time.time() - t0), flush=True)
json.dump({"graph": {k: sorted(v) for k, v in graph.items()}, "tls": {k: sorted(v) for k, v in tlsref.items() if v}, "tlsglob": tlsglob},
          io.open(OUT + r"\cg.json", "w", encoding="utf-8"))
print("defines", len(graph), "tls globals", len(tlsglob), "fns with tls refs", sum(1 for v in tlsref.values() if v))
