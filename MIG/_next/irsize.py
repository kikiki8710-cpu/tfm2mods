# -*- coding: utf-8 -*-
u"""irsize — `_gaibc` 의 `define` 본문 크기를 전수 측정해 '가장 긴 함수' 순위를 낸다. (2026-09-11)

왜 IR 인가: tcx 의 `sp`(def_span)는 **함수 시그니처 범위**라 `l2-l` 이 최대 17줄로 나온다
(본문 길이가 아니다 — 실측으로 확인). 소스 원문이 없으므로 **IR define 본문 줄 수**가
현실적인 복잡도 대리지표다. 분기 밀도(`br`/`switch`/`select`/`phi`)도 같이 센다.
"""
import io, os, re, sys, json
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
CORP = r"C:\tfm2mods\_gaibc"
DEF = re.compile(rb"^define[^@]*@([A-Za-z0-9_$.]+)\(")
BR = (b" br ", b" switch ", b" select ", b" phi ", b" invoke ", b" call ")

rows = []
for fn in sorted(os.listdir(CORP)):
    if not fn.endswith(".ll"):
        continue
    with io.open(os.path.join(CORP, fn), "rb") as f:
        cur = None
        for i, raw in enumerate(f, 1):
            if raw.startswith(b"define"):
                m = DEF.match(raw)
                cur = {"file": fn, "sym": (m.group(1).decode("utf-8", "replace") if m else "?"),
                       "frm": i, "n": 0, "br": 0, "sw": 0, "sel": 0, "phi": 0, "call": 0}
                continue
            if cur is None:
                continue
            cur["n"] += 1
            if b" br " in raw: cur["br"] += 1
            elif b" switch " in raw: cur["sw"] += 1
            if b" select " in raw: cur["sel"] += 1
            if b" phi " in raw: cur["phi"] += 1
            if b" call " in raw or b" invoke " in raw: cur["call"] += 1
            if raw.startswith(b"}"):
                cur["to"] = i
                rows.append(cur)
                cur = None
print(u"define %d개 측정" % len(rows))
rows.sort(key=lambda r: -r["n"])
io.open(r"C:\tfm2mods\MIG\_verify4\irsize.json", "w", encoding="utf-8").write(
    json.dumps(rows[:400], ensure_ascii=False))
print(u"\n%7s %6s %5s %5s %5s %6s  %s" % (u"IR줄", u"br", u"switch", u"select", u"phi", u"call", u"심볼"))
for r in rows[:30]:
    print(u"%7d %6d %5d %5d %5d %6d  %s" % (r["n"], r["br"], r["sw"], r["sel"], r["phi"],
                                            r["call"], r["sym"][:96]))
print(u"\n2000줄 이상 %d개 · 5000줄 이상 %d개 · 10000줄 이상 %d개"
      % (sum(1 for r in rows if r["n"] >= 2000), sum(1 for r in rows if r["n"] >= 5000),
         sum(1 for r in rows if r["n"] >= 10000)))
