# -*- coding: utf-8 -*-
"""<file.ll> <define 시작줄>: 그 함수 본문에서 sret(%0) 및 %0 기반 gep 로 향하는 store/memcpy/call(sret 전달) 을 전부 열거.
   + 함수 안의 직접 call 심볼 목록(순서·줄) + threadlocal 참조."""
import re, sys, io
P, START = sys.argv[1], int(sys.argv[2])
lines = io.open(P, encoding="utf-8", errors="replace", newline="").read().split("\n")
md = {}
for ln in lines:
    if ln.startswith("!") and " = " in ln:
        k, v = ln.split(" = ", 1); md[k] = v
def loc(n, depth=0):
    v = md.get(n)
    if v is None: return "?"
    m = re.match(r"!DILocation\(line: (\d+)(?:, column: \d+)?, scope: (!\d+)(?:, inlinedAt: (!\d+))?", v)
    if not m: return "?"
    s = md.get(m.group(2), ""); f = ""
    mm = re.search(r"file: (!\d+)", s)
    if mm:
        fm = re.search(r'filename: "([^"]+)"', md.get(mm.group(1), ""))
        if fm: f = fm.group(1).split("\\")[-1].split("/")[-1]
    out = f"{f}:{m.group(1)}"
    if m.group(3) and depth < 12: out += "<" + loc(m.group(3), depth+1)
    return out
end = START
while not lines[end-1].startswith("}"): end += 1
print("fn", START, "~", end, "lines", end-START+1)
# %0 파생 포인터 추적: %x = getelementptr ... ptr %0, i64 N  → off
derived = {"%0": 0}
body = lines[START-1:end]
for i, ln in enumerate(body):
    m = re.match(r"\s*(%\d+) = getelementptr [^,]*, ptr (%[\w.]+), i64 (-?\d+)", ln)
    if m and m.group(2) in derived:
        derived[m.group(1)] = derived[m.group(2)] + int(m.group(3))
    m = re.match(r"\s*(%\d+) = getelementptr [^,]*, ptr (%[\w.]+), i64 (%\d+)", ln)
    if m and m.group(2) in derived:
        derived[m.group(1)] = f"{derived[m.group(2)]}+{m.group(3)}"
print("derived ptrs:", {k: v for k, v in derived.items()})
calls = []
for i, ln in enumerate(body):
    n = START + i
    dm = re.search(r"!dbg (!\d+)", ln); L = loc(dm.group(1)) if dm else ""
    core = re.sub(r", !dbg !\d+", "", ln)
    if "#dbg_" in ln: continue
    m = re.match(r"\s*store (\w+) ([^,]+), ptr (%[\w.]+)", core)
    if m and m.group(3) in derived:
        print(f"STORE {n}| +{derived[m.group(3)]} {m.group(1)} {m.group(2)[:40]}   ;; {L}")
    m = re.search(r"llvm\.memcpy[^(]*\(ptr [^%]*(%[\w.]+), ptr [^%]*(%[\w.]+), i64 (\d+)", core)
    if m and (m.group(1) in derived):
        print(f"MEMCPY {n}| +{derived[m.group(1)]} <- {m.group(2)} {m.group(3)}B   ;; {L}")
    m = re.search(r"llvm\.memset[^(]*\(ptr [^%]*(%[\w.]+), i8 (-?\d+), i64 (\d+)", core)
    if m and (m.group(1) in derived):
        print(f"MEMSET {n}| +{derived[m.group(1)]} = {m.group(2)} x{m.group(3)}B   ;; {L}")
    for m in re.finditer(r"(?:call|invoke)\b[^@\n]*@([A-Za-z0-9_$.]+)\(", core):
        s = m.group(1)
        if s.startswith("llvm.") and not s.startswith("llvm.threadlocal"): continue
        srets = [k for k in derived if re.search(r"sret\(\[\d+ x i8\]\)[^,]*" + re.escape(k) + r"\b", core)]
        calls.append((n, s, L, srets))
    if re.search(r"(?:call|invoke)\b[^@\n]*%\d+\(", core):
        calls.append((n, "<indirect>", L, []))
print("--- calls (%d)" % len(calls))
for n, s, L, srets in calls:
    print(f"CALL {n}| {s[:110]}  {'sret->'+str(srets) if srets else ''}   ;; {L}")
