#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""r8addr.py — 라운드 JSON(`_spec\\rN\\*.json`) 의 exe 주소 후보를 **aimap 패닉 Location ∩ IR 본문 줄범위**로 찾는다. (2026-09-13 신설)

왜: addspec 전에 `--exe` 를 채워야 하는데, 잎 선별표(`_next\\subtree_e4c5c0.md`)의 라벨은 dllmatch 추정이라 틀린다
   (09-13 ghidra-re: 11건 중 9건이 클로저 인스턴스 오라벨). rvaverify S1(반증형)을 **탐색형**으로 뒤집어
   「같은 모듈(mod) 의 exe 함수 중 IR 본문 줄범위 안의 Location 을 가장 많이 가진 것」을 후보로 낸다.
   후보는 **추정**이다 — 확정은 fnprobe(인자 수·상수)·rvaverify·런타임(ev1)이 한다.

사용: python -X utf8 MIG\\r8addr.py _spec\\r8 [--table _next\\subtree_e4c5c0.md]
출력: 명세별 (IR 줄범위, 표 후보, aimap 상위 후보 3 = rva/크기/적중줄/전체줄)
"""
import io, json, os, re, sys, glob
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import rvaverify as RV

def main():
    a = sys.argv[1:]
    d = a[0] if a else os.path.join(HERE, "_spec", "r8")
    tbl = a[a.index("--table") + 1] if "--table" in a else os.path.join(HERE, "_next", "subtree_e4c5c0.md")
    aimap = json.load(io.open(os.path.join(HERE, "aimap.json"), encoding="utf-8"))["info"]
    trows = {}
    if os.path.exists(tbl):
        for ln in io.open(tbl, encoding="utf-8"):
            m = re.match(r"\|\s*\d+\s*\|\s*`(0x[0-9a-f]+)`\s*\|\s*`([^`]+)`\s*\|\s*([^|]+?)\s*\|", ln)
            if m: trows.setdefault((m.group(2).split("::")[-1], m.group(3)), []).append(m.group(1))
    for p in sorted(glob.glob(os.path.join(d, "*.json"))):
        j = json.load(io.open(p, encoding="utf-8"))
        sp = dict(ir=dict(file=j["ir_file"], frm=j["ir_from"], to=j["ir_to"]), src_line=j.get("src_line"))
        rng = RV.ir_body_range(sp)
        mod = os.path.splitext(os.path.basename(j["src"].replace("\\", "/")))[0]
        name = j["name"].split("::")[-1]
        cands = []
        if rng:
            lo, hi = rng
            for rva, inf in aimap.items():
                if (inf.get("mod") or "").split("/")[-1] != mod: continue  # aimap mod = 경로형(plan_legacy/old/epic)
                ls = inf.get("lines") or []
                hit = [l for l in ls if lo <= l <= hi]
                if hit: cands.append((len(hit), -len(ls), rva, inf.get("bytes"), sorted(set(hit)), len(ls)))
            cands.sort(reverse=True)
        print(u"■ %s  (%s · IR %s:%d-%d · 본문줄 %s · mod=%s)" % (os.path.basename(p), name, j["ir_file"], j["ir_from"], j["ir_to"], rng, mod))
        print(u"   표 후보: %s" % (trows.get((name, mod)) or u"없음"))
        for c in cands[:3]:
            print(u"   aimap: %s %5sB  적중 %d/%d  줄 %s" % (c[2], c[3], c[0], c[5], c[4][:8]))
        if not cands: print(u"   aimap: 후보 없음(같은 mod 에 본문 줄범위 Location 없음 — 인라인/aux 확인)")

if __name__ == "__main__":
    main()
