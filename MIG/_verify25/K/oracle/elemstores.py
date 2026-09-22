# -*- coding: utf-8 -*-
"""elemstores.py — 함수 본문에서 alloca 기준 store/memcpy/memset 을 오프셋으로 전수 열거(원소 live 바이트 확정 재료).
사용: python -X utf8 elemstores.py <file.ll> <a> <b> [alloca 이름 필터...]"""
import io, re, sys
f, a, b = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
flt = sys.argv[4:]
L = io.open(f, encoding="utf-8", errors="replace").read().split("\n")[a-1:b]
allocas = {}
base = {}   # reg -> (allocaReg, off)
GEP = re.compile(r"^\s*(%\d+) = getelementptr inbounds(?: nuw)? i8, ptr (%\d+), i64 (-?\d+)")
GEP2 = re.compile(r"^\s*(%\d+) = getelementptr inbounds(?: nuw)? (\{[^}]*\}|\[[^\]]*\]|[^,]+), ptr (%\d+), i64 (-?\d+)(?:, i64 (-?\d+))?")
ST = re.compile(r"^\s*store (\S+) (.+?), ptr (%\d+),")
MC = re.compile(r"llvm\.mem(cpy|set)\.p0\S*\(ptr [^%]*(%\d+), (?:ptr [^%]*(%\d+)|i8 (-?\d+)), i64 (\d+)")
for i, l in enumerate(L):
    m = re.match(r"^\s*(%\d+) = alloca \[(\d+) x i8\]", l)
    if m:
        allocas[m.group(1)] = int(m.group(2)); base[m.group(1)] = (m.group(1), 0); continue
    m = GEP.match(l)
    if m and m.group(2) in base:
        b0, o = base[m.group(2)]; base[m.group(1)] = (b0, o + int(m.group(3))); continue
    m = GEP2.match(l)
    if m and m.group(3) in base and m.group(4) == "0" and m.group(5) is None:
        base[m.group(1)] = base[m.group(3)]; continue
    m = ST.match(l)
    if m and m.group(3) in base:
        b0, o = base[m.group(3)]
        if not flt or b0 in flt:
            print("L%-6d %s+%-4d store %-6s %s" % (a + i, b0, o, m.group(1), m.group(2)[:60]))
        continue
    m = MC.search(l)
    if m and m.group(2) in base:
        b0, o = base[m.group(2)]
        src = m.group(3) or ("i8 " + m.group(4))
        srcd = ""
        if m.group(3) in base:
            srcd = " <- %s+%d" % base[m.group(3)]
        if not flt or b0 in flt or (m.group(3) in base and base[m.group(3)][0] in flt):
            print("L%-6d %s+%-4d mem%s %sB src=%s%s" % (a + i, b0, o, m.group(1), m.group(5), src, srcd))
print("allocas:", allocas)
