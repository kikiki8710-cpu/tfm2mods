# -*- coding: utf-8 -*-
u"""G12(`srclinecheck`) 가 「실제 후보」로 내놓는 줄이 무엇인지 **그 줄 원문까지** 찍는다.
7차 배치A 가 G12 8건을 전수 반증할 때 쓴 조회 도구.
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
sys.path.insert(0, MIG)
import srclinecheck as SLC

D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
TARGETS = [(0, 3), (0, 4), (1, 3), (1, 5), (1, 8), (1, 10), (2, 1), (2, 4)]

for (i, j) in TARGETS:
    sp = D["specs"][i]
    f, a, b = sp["ir"]["file"], sp["ir"]["frm"], sp["ir"]["to"]
    own = sp["src"].split("\\")[-1]
    src, meta = SLC.load(f)
    c = sp["consts"][j]
    val, claim = c["value"], c["src_line"]
    pat = re.compile(r"(?<![\w.\-])" + re.escape(str(val)) + r"(?![\w.])")
    print(u"\n=== specs[%d] %s consts[%d] value=%s  주장 src_line=%s" % (i, sp["name"], j, val, claim))
    n = 0
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln or not pat.search(ln):
            continue
        m = SLC.DBG.search(ln)
        if not m:
            continue
        ch = [(fn, li) for (fn, li) in SLC.chain(meta, m.group(1)) if fn == own]
        if not ch:
            continue
        print(u"   L%-7d %-96s -> %s" % (k + 1, ln.strip()[:96], ch))
        n += 1
        if n >= 6:
            print(u"   ... (이하 생략)")
            break
    if n == 0:
        print(u"   (후보 0 — 판정보류)")
