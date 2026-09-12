# -*- coding: utf-8 -*-
u"""IR 범위 안의 명령별 `!dbg !N` 을 **inlinedAt 루트까지** 타서 (파일, 줄) 로 환원한다.
`consts[].src_line` 이 실제로 그 줄에서 나온 값인지 기계 대조하기 위한 도구.
사용: python -X utf8 dbglines.py <m04.ll> <from> <to> [찾을리터럴 ...]
"""
import io, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")

IRDIR = r"C:\tfm2mods\_gaibc"
f, a, b = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
lits = sys.argv[4:]
path = os.path.join(IRDIR, f)
lines = io.open(path, encoding="utf-8", errors="replace").read().split("\n")

# 메타데이터 인덱스 (한 번만)
meta = {}
mre = re.compile(r"^!(\d+) = (.*)$")
for ln in lines:
    if ln.startswith("!"):
        m = mre.match(ln)
        if m:
            meta[m.group(1)] = m.group(2)

LOC = re.compile(r"!DILocation\(line: (\d+), column: (\d+), scope: !(\d+)(?:, inlinedAt: !(\d+))?\)")
SCOPE = re.compile(r"!DILocation\(line: (\d+), scope: !(\d+)(?:, inlinedAt: !(\d+))?\)")
FILEOF = re.compile(r'filename: "([^"]+)"')


def root(n, depth=0):
    u"""inlinedAt 루트까지 타고 (line, scope) 반환"""
    cur = n
    last = None
    while cur and cur in meta and depth < 64:
        t = meta[cur]
        m = LOC.search(t) or SCOPE.search(t)
        if not m:
            break
        g = m.groups()
        line = int(g[0])
        scope = g[-2] if len(g) == 4 else g[1]
        inl = g[-1]
        last = (line, scope)
        if not inl:
            break
        cur = inl
        depth += 1
    return last


def scope_file(sid, depth=0):
    u"""scope !N 에서 file 이름을 캔다(DISubprogram/DILexicalBlock 을 타고 올라감)"""
    cur = sid
    while cur and cur in meta and depth < 32:
        t = meta[cur]
        m = re.search(r"file: !(\d+)", t)
        if m:
            fm = meta.get(m.group(1), "")
            fn = FILEOF.search(fm)
            if fn:
                return fn.group(1)
        m2 = re.search(r"scope: !(\d+)", t)
        if not m2:
            return "?"
        cur = m2.group(1)
        depth += 1
    return "?"


def innermost(n):
    u"""가장 안쪽 DILocation 의 (line, scope) — 같은 파일 안 클로저는 이쪽이 진짜 소스 줄이다."""
    t = meta.get(n, "")
    m = LOC.search(t) or SCOPE.search(t)
    if not m:
        return None
    g = m.groups()
    return (int(g[0]), g[-2] if len(g) == 4 else g[1])


DBG = re.compile(r"!dbg !(\d+)")
agg = {}
hits = []
for i in range(a - 1, min(b, len(lines))):
    ln = lines[i]
    if "#dbg_" in ln:
        continue
    m = DBG.search(ln)
    if not m:
        continue
    r = root(m.group(1))
    inm = innermost(m.group(1))
    if not r:
        continue
    key = (scope_file(r[1]), r[0])
    ikey = (scope_file(inm[1]), inm[0]) if inm else key
    agg[(ikey, key)] = agg.get((ikey, key), 0) + 1
    for lit in lits:
        if re.search(r"(?<![\w.])" + re.escape(lit) + r"(?![\w.])", ln):
            hits.append((lit, i + 1, ikey, key, ln.strip()[:130]))

print(u"== %s:%d~%d — (최내곽 줄 | inlinedAt 루트) 집계 ==" % (f, a, b))
for k in sorted(agg, key=lambda x: (x[0][0], x[0][1], x[1][1])):
    ik, rk = k
    print(u"  inner %-26s L%-6d | root %-22s L%-6d  %d개"
          % (ik[0].split("\\")[-1], ik[1], rk[0].split("\\")[-1], rk[1], agg[k]))
if lits:
    print(u"\n== 리터럴 출처 ==")
    for lit, irln, ik, rk, txt in hits:
        print(u"  %-12s IR:%-7d inner=%s:L%-5d root=%s:L%-5d  %s"
              % (lit, irln, ik[0].split("\\")[-1], ik[1], rk[0].split("\\")[-1], rk[1], txt))
