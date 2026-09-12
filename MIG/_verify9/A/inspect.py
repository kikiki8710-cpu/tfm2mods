# -*- coding: utf-8 -*-
u"""G12 적발 8건을 IR 원문으로 가리기 위한 조사 보조.

사용: python -X utf8 inspect.py <spec_idx> <const_idx>
       각 매치 줄의 **원문 · 줄번호 · 강/약 후보 · 사슬 전체**를 그대로 찍는다.
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, MIG)
import srclinecheck as S

D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

i = int(sys.argv[1])
j = int(sys.argv[2])
sp = D["specs"][i]
ir = sp["ir"]
f, a, b = ir["file"], ir["frm"], ir["to"]
own = sp["src"].split("\\")[-1]
src, meta = S.load(f)
c = sp["consts"][j]
val, claim = c["value"], c["src_line"]
pat = S.litpat(val)
print(u"== specs[%d] %s  consts[%d] value=%s claim=L%s  own=%s  %s:%d~%d"
      % (i, sp["name"], j, val, claim, own, f, a, b))
print(u"   meaning: %s" % c.get("meaning"))
print()
nmatch = 0
for k in range(a - 1, min(b, len(src))):
    ln = src[k]
    if "#dbg_" in ln or not pat.search(ln):
        continue
    nmatch += 1
    st, wk = S.dbg_ids(src, meta, k, a, b, val)
    print(u"-- %s:%d  %s" % (f, k + 1, ln.strip()[:300]))
    for did in st:
        ch = S.chain(meta, did)
        print(u"     [강] !%s  사슬=%s" % (did, ch))
    for did in wk:
        ch = S.chain(meta, did)
        print(u"     [약] !%s  사슬=%s" % (did, ch))
    if not st and not wk:
        print(u"     (!dbg 없음)")
print(u"\n매치 줄 수 = %d" % nmatch)
