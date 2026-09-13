#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""locfind.py — 패닉 Location(파일:줄[:열]) 상수를 참조하는 IR define 을 역추적한다 (2026-09-13 · ghidra-re 스크래치 승격 · `@anon.<hash>.N` 대응).
사용: python -X utf8 MIG\locfind.py <file> <line> [col]
"""
"""locfind.py <file-substr> <line> [col] : _gaibc m*.ll 에서 (파일,줄[,칸]) 패닉 Location 상수와 그것을 참조하는 define"""
import sys, re, io, os, glob, struct
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
IR = r"C:\tfm2mods\_gaibc"
fsub, line = sys.argv[1], int(sys.argv[2]); col = int(sys.argv[3]) if len(sys.argv)>3 else None
def unesc(s):
    out = bytearray(); i = 0
    while i < len(s):
        if s[i] == '\\' and s[i+1:i+2] == '\\': out += b'\\'; i += 2  # LLVM `\\` = 백슬래시 1개(경로 구분자) — 09-13 ghidra-re 보고 결함 정정
        elif s[i] == '\\': out += bytes([int(s[i+1:i+3],16)]); i += 3
        else: out += s[i].encode(); i += 1
    return bytes(out)
for path in sorted(glob.glob(os.path.join(IR, 'm*.ll'))):
    txt = io.open(path, encoding='utf-8', errors='replace').read()
    pa = {}
    for m in re.finditer(r'^@([\w.$]+) = private unnamed_addr constant \[\d+ x i8\] c"([^"]*)"', txt, re.M):
        if fsub in m.group(2): pa[m.group(1)] = m.group(2)
    if not pa: continue
    hits = []
    for m in re.finditer(r'^@([\w.$]+) = private unnamed_addr constant <\{ ptr, \[16 x i8\] \}> <\{ ptr @([\w.$]+), \[16 x i8\] c"([^"]*)" \}>', txt, re.M):
        if m.group(2) in pa:
            raw = unesc(m.group(3))
            if len(raw) != 16: continue
            ln_, line_, col_ = struct.unpack('<QII', raw)
            if line_ == line and (col is None or col_ == col):
                hits.append((m.group(1), m.group(2), line_, col_))
    lines = txt.split('\n')
    for al, pal, l_, c_ in hits:
        print("%s  @%s  %s:%d:%d" % (os.path.basename(path), al, pa[pal].replace('\\\\','\\').replace('\\00',''), l_, c_))
        cur = None; pat = re.compile(r'@'+re.escape(al)+r'(?![\w.$])')
        for ln in lines:
            if ln.startswith('define '): cur = ln
            elif ln.startswith('}'): cur = None
            elif cur and pat.search(ln):
                m2 = re.search(r'@([\w$.]+)\(', cur)
                print("     ref-in define: %s" % m2.group(1)[:220])