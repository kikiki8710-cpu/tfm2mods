# -*- coding: utf-8 -*-
"""_gaibc/m*.ll + _gcbc/g*.ll 전량을 스트리밍해 define 별 {callees, tls refs, file, line} 그래프를 pickle 로 저장.
   (sed 루프 금지 — 파이썬 한 번 훑기)"""
import re, io, os, glob, pickle, sys, time
OUT = os.path.join(os.path.dirname(__file__), "cg.pkl")
files = sorted(glob.glob(r"C:\tfm2mods\_gaibc\m*.ll")) + sorted(glob.glob(r"C:\tfm2mods\_gcbc\g*.ll"))
DEF = re.compile(r"^define [^@]*@([^\s(]+)\(")
CALL = re.compile(r"(?:call|invoke)\b[^@]*@([A-Za-z0-9_$.]+)\(")
TLS = re.compile(r"llvm\.threadlocal\.address[^@]*@([A-Za-z0-9_$.]+)")
GLOB_TLS = re.compile(r"^@([A-Za-z0-9_$.]+) = [^\n]*thread_local")
graph = {}   # sym -> dict(file, line, callees=set, tls=set, indirect=int)
tls_globals = {}
t0 = time.time()
for fp in files:
    base = os.path.basename(fp)
    cur = None
    with io.open(fp, encoding="utf-8", errors="replace") as f:
        for ln, line in enumerate(f, 1):
            if cur is None:
                if line.startswith("define "):
                    m = DEF.match(line)
                    if m:
                        cur = m.group(1)
                        if cur not in graph:
                            graph[cur] = {"file": base, "line": ln, "callees": set(), "tls": set(), "indirect": 0}
                        curd = graph[cur]
                elif line.startswith("@") and "thread_local" in line:
                    m = GLOB_TLS.match(line)
                    if m: tls_globals[m.group(1)] = (base, ln)
            else:
                if line.startswith("}"):
                    cur = None; continue
                if "call" in line:
                    for m in CALL.finditer(line):
                        s = m.group(1)
                        if s.startswith("llvm."):
                            if s.startswith("llvm.threadlocal.address"):
                                for t in TLS.finditer(line): curd["tls"].add(t.group(1))
                            continue
                        curd["callees"].add(s)
                    if re.search(r"(?:call|invoke)\b[^@\n]*%\d+\(", line) or re.search(r"(?:call|invoke)\b[^@\n]*%[A-Za-z_][\w.]*\(", line):
                        curd["indirect"] += 1
                elif "llvm.threadlocal.address" in line:
                    for t in TLS.finditer(line): curd["tls"].add(t.group(1))
    print(base, "defs so far", len(graph), "tls globals", len(tls_globals), "%.0fs" % (time.time() - t0), flush=True)
pickle.dump({"graph": graph, "tls_globals": tls_globals}, open(OUT, "wb"))
print("saved", OUT, len(graph))
