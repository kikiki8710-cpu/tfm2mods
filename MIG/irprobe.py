#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""irprobe.py — IR define 하나의 프로파일: 인자·call 집계·DILocation 줄 집합 (2026-09-13 · ghidra-re 스크래치 승격 · fnprobe.py 의 IR 쪽 짝).
사용: python -X utf8 MIG\irprobe.py <m##.ll> <start> <end>
"""
"""irprobe.py <m10.ll> <start> <end>  : define 시그니처, call 목록(순서), DILocation 파일:줄 집합, panic Location alloc 참조"""
import sys, re, io, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
IR = r"C:\tfm2mods\_gaibc\\"
fn, a, b = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
lines = io.open(IR+fn, encoding='utf-8', errors='replace').read().split('\n')
body = lines[a-1:b]
# metadata map for the file (only what we need): DILocation and DIFile/DISubprogram
meta = {}
for ln in lines:
    if ln.startswith('!') and ' = ' in ln:
        k, v = ln.split(' = ', 1); meta[k] = v
def dbg_of(ln):
    m = re.search(r'!dbg !(\d+)', ln)
    return '!'+m.group(1) if m else None
def loc_chain(k, depth=0):
    v = meta.get(k, '')
    m = re.search(r'line: (\d+)', v); sc = re.search(r'scope: !(\d+)', v); ia = re.search(r'inlinedAt: !(\d+)', v)
    line = int(m.group(1)) if m else None
    # file via scope -> DISubprogram/DILexicalBlock ... file
    f = None; s = sc.group(1) if sc else None
    for _ in range(6):
        if s is None: break
        sv = meta.get('!'+s, '')
        fm = re.search(r'file: !(\d+)', sv)
        if fm:
            fv = meta.get('!'+fm.group(1), '')
            nm = re.search(r'filename: "([^"]+)"', fv)
            f = nm.group(1) if nm else None; break
        sm = re.search(r'scope: !(\d+)', sv); s = sm.group(1) if sm else None
    return (f, line), (loc_chain('!'+ia.group(1), depth+1) if ia and depth < 8 else None)
print("== define:", body[0][:400])
calls = []
locs = collections.Counter()
for ln in body:
    m = re.search(r'\b(?:call|invoke)\b[^@]*@([\w$.]+)', ln)
    if m: calls.append((m.group(1), dbg_of(ln)))
    d = dbg_of(ln)
    if d:
        c = loc_chain(d)
        locs[c[0]] += 1
        # root inlinedAt
print("== calls (%d):" % len(calls))
seen = collections.Counter()
for c, d in calls:
    seen[c] += 1
for c, n in seen.items():
    print("   %3d  %s" % (n, c[:160]))
print("== DILocation (file,line) distinct %d:" % len(locs))
byf = collections.defaultdict(list)
for (f, l), n in locs.items(): byf[f].append(l)
for f, ls in byf.items(): print("   %s: %s" % (f, sorted(set(x for x in ls if x))))
# panic locations referenced: alloc symbols with Location shape in this file
allocs = set(re.findall(r'@(alloc_[0-9a-f]+)', '\n'.join(body)))
print("== alloc refs:", len(allocs))
txt = '\n'.join(lines)
for al in sorted(allocs):
    m = re.search(r'^@'+al+r' = .*$', txt, re.M)
    if m and 'ptr @' in m.group(0):
        mm = re.search(r'ptr @(alloc_[0-9a-f]+), \[16 x i8\] c"((?:\\[0-9A-F]{2}|[^"])*)"', m.group(0))
        if mm:
            raw = bytes(int(x,16) for x in re.findall(r'\\([0-9A-F]{2})', mm.group(2))) if '\\' in mm.group(2) else None
            pm = re.search(r'^@'+mm.group(1)+r' = .*c"([^"]*)"', txt, re.M)
            path = pm.group(1) if pm else '?'
            import struct
            if raw and len(raw)==16:
                ln_, line_, col_ = struct.unpack('<QII', raw)
                print("   Location %s:%d:%d" % (path.replace('\\\\','\\'), line_, col_))
            else:
                print("   Location %s raw=%r" % (path, mm.group(2)))