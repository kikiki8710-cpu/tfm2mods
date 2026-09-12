# -*- coding: utf-8 -*-
u"""배치 C 전용 — specs[10..14] 의 consts[].src_line 을 IR !dbg inlinedAt 사슬과 대조하고,
본문이 실제로 참조하는 담당 .rs 줄 집합을 함께 찍는다. (srclinecheck 의 리포트 루프를 배치 범위로 좁힌 것)"""
import io, json, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import srclinecheck as S

D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
lo = int(sys.argv[1]) if len(sys.argv) > 1 else 10
hi = int(sys.argv[2]) if len(sys.argv) > 2 else 15

for i in range(lo, hi):
    sp = D["specs"][i]
    f, a, b = sp["ir"]["file"], sp["ir"]["frm"], sp["ir"]["to"]
    own = sp["src"].split("\\")[-1]
    src, meta = S.load(f)
    print(u"\n===== specs[%d] %s  (%s  %s:%d~%d) =====" % (i, sp["name"], own, f, a, b))
    ownlines = {}
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        m = S.DBG.search(ln)
        if not m:
            continue
        for (fn, li) in S.chain(meta, m.group(1)):
            if fn == own:
                ownlines[li] = ownlines.get(li, 0) + 1
    print(u"  본문이 참조하는 %s 줄 = %s" % (own, sorted(x for x in ownlines if x)))
    for j, c in enumerate(sp.get("consts") or []):
        val, claim = c.get("value"), c.get("src_line")
        pat = re.compile(r"(?<![\w.\-])" + re.escape(str(val)) + r"(?![\w.])")
        found = {}
        hits = []
        for k in range(a - 1, min(b, len(src))):
            ln = src[k]
            if "#dbg_" in ln or not pat.search(ln):
                continue
            m = S.DBG.search(ln)
            if not m:
                continue
            ch = S.chain(meta, m.group(1))
            hits.append((k + 1, ln.strip()[:110], ch))
            for (fn, li) in ch:
                if fn == own:
                    found[li] = found.get(li, 0) + 1
        cands = sorted(x for x in found if x)
        ok = claim in found
        print(u"  consts[%d] value=%-14s claim=L%-6s cands=%-30s %s"
              % (j, val, claim, cands if cands else u"(리터럴 미검출)",
                 u"OK" if ok else (u"**불일치**" if cands else u"(보류)")))
        if not ok and cands and "-v" in sys.argv:
            for (lnno, txt, ch) in hits[:14]:
                print(u"      %s:%d  %s" % (f, lnno, txt))
                print(u"         chain=%s" % (ch,))
