# -*- coding: utf-8 -*-
u"""IR 범위를 훑어 (a) GEP 오프셋↔load/store (b) call/invoke 대상 (c) 리터럴 소비 지점을 뽑는다.
명세→IR 이 아니라 **IR→명세** 방향 감사를 위한 것.
사용: python irwalk.py <spec_idx> [mode]   mode = mem | call | lit | all
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
IRDIR = r"C:\tfm2mods\_gaibc"

D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"] if isinstance(D, dict) else D
i = int(sys.argv[1])
mode = sys.argv[2] if len(sys.argv) > 2 else "all"
sp = S[i]
ir = sp["ir"]
src = io.open(os.path.join(IRDIR, ir["file"]), encoding="utf-8", errors="replace").read().split("\n")
lo, hi = ir["frm"], ir["to"]
body = src[lo - 1:hi]

RG = re.compile(r"^\s*(%\w+) = getelementptr[^,]*,\s*ptr (%\w+|@\S+), i64 (-?\d+)")
RL = re.compile(r"^\s*(%\w+) = (?:tail )?load (\w+), ptr (%\w+|@\S+)")
RS = re.compile(r"^\s*store (?:volatile )?([\w\s]+?) (\S+), ptr (%\w+|@\S+)")
RC = re.compile(r"(?:call|invoke)\b[^@]*@([\w.$]+)")

gep = {}      # %reg -> (base, off, line)
for n, ln in enumerate(body):
    m = RG.match(ln)
    if m:
        gep[m.group(1)] = (m.group(2), int(m.group(3)), lo + n)

if mode in ("mem", "all"):
    print("== load/store (ptr 가 GEP 이면 오프셋 표시) ==")
    for n, ln in enumerate(body):
        L = lo + n
        m = RL.match(ln)
        if m:
            p = m.group(3)
            b, o = gep.get(p, (p, None))[0:2] if p in gep else (p, None)
            print("L%-6d r  %-6s off=%-6s base=%s   | %s" % (L, m.group(2), hex(o) if o is not None else "-", b, ln.strip()[:130]))
        m = RS.match(ln)
        if m:
            p = m.group(3)
            b, o = gep.get(p, (p, None))[0:2] if p in gep else (p, None)
            print("L%-6d w  %-6s off=%-6s base=%s   | %s" % (L, m.group(1).strip(), hex(o) if o is not None else "-", b, ln.strip()[:130]))

if mode in ("call", "all"):
    print("== call/invoke ==")
    seen = {}
    for n, ln in enumerate(body):
        for m in RC.finditer(ln):
            seen.setdefault(m.group(1), []).append(lo + n)
    for k in sorted(seen):
        print("  %-100s %s" % (k[:100], seen[k][:6]))

if mode in ("lit", "all"):
    print("== 리터럴 소비(icmp/select/phi/switch/store) ==")
    RX = re.compile(r"^\s*(?:%\w+ = )?(icmp|fcmp|select|phi|switch|store|ret|add|sub|mul|shl|and|or)\b")
    for n, ln in enumerate(body):
        if RX.match(ln) and re.search(r"(?<![\w.%])-?\d+(?![\w.])", ln.split(", !")[0]):
            print("L%-6d %s" % (lo + n, ln.strip()[:170]))
