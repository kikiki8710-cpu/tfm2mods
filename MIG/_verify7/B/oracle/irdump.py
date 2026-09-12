# -*- coding: utf-8 -*-
u"""7차 배치B 조사보조 — 담당 함수 IR 범위에서 `!dbg` 사슬(담당 .rs 프레임)을 뽑아 본다.

사용:
  python -X utf8 irdump.py <specidx>                 범위 안 own-file 줄 히스토그램
  python -X utf8 irdump.py <specidx> --line L        그 소스줄을 가리키는 IR 줄 전부
  python -X utf8 irdump.py <specidx> --val V         그 리터럴을 쓰는 IR 줄 + 사슬 전문
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import srclinecheck as S

D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
i = int(sys.argv[1])
sp = D["specs"][i]
f, a, b = sp["ir"]["file"], sp["ir"]["frm"], sp["ir"]["to"]
own = sp["src"].split("\\")[-1]
src, meta = S.load(f)
print(u"specs[%d] %s  own=%s  %s:%d~%d" % (i, sp["name"], own, f, a, b))

mode = sys.argv[2] if len(sys.argv) > 2 else None
arg = sys.argv[3] if len(sys.argv) > 3 else None

def chains(k):
    ln = src[k]
    m = S.DBG.search(ln)
    if not m:
        return None
    return S.chain(meta, m.group(1))

if mode is None:
    hist = {}
    for k in range(a - 1, min(b, len(src))):
        if "#dbg_" in src[k]:
            continue
        ch = chains(k)
        if not ch:
            continue
        for (fn, li) in ch:
            if fn == own:
                hist[li] = hist.get(li, 0) + 1
    for li in sorted(hist):
        print(u"  L%-6d %d" % (li, hist[li]))
elif mode == "--line":
    L = int(arg)
    for k in range(a - 1, min(b, len(src))):
        if "#dbg_" in src[k]:
            continue
        ch = chains(k)
        if not ch:
            continue
        if any(fn == own and li == L for (fn, li) in ch):
            print(u"%7d| %s" % (k + 1, src[k].strip()[:190]))
            print(u"        chain=%s" % (ch,))
elif mode == "--val":
    pat = re.compile(r"(?<![\w.\-])" + re.escape(arg) + r"(?![\w.])")
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln or not pat.search(ln):
            continue
        ch = chains(k)
        if not ch:
            continue
        if not any(fn == own for (fn, li) in ch):
            continue
        print(u"%7d| %s" % (k + 1, ln.strip()[:190]))
        print(u"        chain=%s" % (ch,))
