# -*- coding: utf-8 -*-
u"""G12 수동 확인 보조 — 리터럴이 나온 IR 줄을 **원문 그대로** + inlinedAt 사슬과 함께 찍는다.

srclinecheck 의 후보 목록은 정규식이 SSA 레지스터(`%3`)·정렬(`align 4`)·크기까지 긁어
오염된다. 이 도구는 매칭 줄 원문을 보여주므로 사람이 진짜 리터럴 피연산자인지 가른다.

용법: python -X utf8 g12probe.py <specidx> <value> [--real]
      --real = 진짜 상수 피연산자(`i8 3`, `i64 -1` 꼴)만 추린다
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
sys.path.insert(0, MIG)
import srclinecheck as SLC

D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
idx = int(sys.argv[1])
val = sys.argv[2]
realonly = "--real" in sys.argv
sp = D["specs"][idx]
f, a, b = sp["ir"]["file"], sp["ir"]["frm"], sp["ir"]["to"]
own = sp["src"].split("\\")[-1]
src, meta = SLC.load(f)
pat = re.compile(r"(?<![\w.\-])" + re.escape(val) + r"(?![\w.])")
# 진짜 상수 피연산자: LLVM 타입 뒤에 오는 값
realpat = re.compile(r"\b(i1|i8|i16|i32|i64|i128|ptr)\s+" + re.escape(val) + r"(?![\w.])")
print(u"# specs[%d] %s  %s:%d~%d  own=%s  value=%s" % (idx, sp["name"], f, a, b, own, val))
for k in range(a - 1, min(b, len(src))):
    ln = src[k]
    if "#dbg_" in ln or not pat.search(ln):
        continue
    m = SLC.DBG.search(ln)
    if not m:
        continue
    isreal = bool(realpat.search(ln))
    if realonly and not isreal:
        continue
    ch = SLC.chain(meta, m.group(1))
    print(u"%-7d %s %s" % (k + 1, u"REAL" if isreal else u"reg?", ln.strip()[:190]))
    print(u"        chain=%s" % (ch,))
