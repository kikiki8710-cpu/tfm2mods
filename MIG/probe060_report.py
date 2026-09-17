# -*- coding: utf-8 -*-
"""
probe060_report.py — tfm2_judge_verify060 1단계 로그(probe060.txt) → 함수별 최종 발화수·콜러 RVA 표.
출력: _next/probe060_result.md(발화 오름차순 · 미발화 목록 · 콜러 RVA) · probe060_result.json
사용: python probe060_report.py [로그 경로]
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
LOG = sys.argv[1] if len(sys.argv) > 1 else r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_judge_verify060\probe060.txt"
TBL = os.path.join(HERE, "..", "tfm2_judge_verify060", "src", "probe_tbl.rs")
probes = {int(m.group(1)): (int(m.group(2), 16), m.group(3), int(m.group(4)), m.group(5)) for m in re.finditer(r"idx: (\d+), rva: 0x([0-9a-f]+), name: \"([^\"]*)\", spec_i: (\d+), verdict: \"([^\"]*)\"", io.open(TBL, encoding="utf-8").read())}
last = {}; installed = None; frames = 0
for line in io.open(LOG, encoding="utf-8", errors="replace"):
    m = re.match(r"\s*\[(\d+)\] ([0-9a-f]+) (\S+) hits=(\d+) \(\+(\d+)\) rets\[([^\]]*)\](\s*\+ovf)?", line)
    if m: last[int(m.group(1))] = (int(m.group(4)), m.group(6).split(), bool(m.group(7)))
    elif line.startswith("[install] base"): installed = line.strip()
    elif line.startswith("[f"): frames = int(re.match(r"\[f(\d+)\]", line).group(1))
rows = []
for i, (rva, name, si, vd) in sorted(probes.items()):
    h, rets, ovf = last.get(i, (0, [], False))
    rows.append(dict(idx=i, rva="%x" % rva, name=name, spec_i=si, verdict=vd, hits=h, rets=rets, ovf=ovf))
rows.sort(key=lambda r: r["hits"])
zero = [r for r in rows if r["hits"] == 0]
L = [u"# probe060 1단계 결과 — 0.6.0 발화수(관리 화면 배경 리그 sim · 마지막 스냅샷 f%d)" % frames, u"", u"%s" % installed, u"",
     u"발화 %d / 미발화 %d (프로브 %d)" % (len(rows) - len(zero), len(zero), len(rows)), u"",
     u"## 미발화(sweep 표본 없음 — 실경기/다른 씬 필요)", u""] + [u"- [%d] `%s` %s (i=%d · %s)" % (r["idx"], r["rva"], r["name"], r["spec_i"], r["verdict"]) for r in zero]
L += [u"", u"## 발화 오름차순(콜러 리턴 RVA ≤8 · +ovf = 9 이상)", u"", u"| idx | rva | 함수 | i | 판정 | 발화 | 콜러 |", u"|---|---|---|---|---|---|---|"]
L += [u"| %d | `%s` | %s | %d | %s | %d | %s%s |" % (r["idx"], r["rva"], r["name"][:44], r["spec_i"], r["verdict"][:10], r["hits"], u" ".join(r["rets"]), u" +ovf" if r["ovf"] else u"") for r in rows if r["hits"]]
io.open(os.path.join(HERE, "_next", "probe060_result.md"), "w", encoding="utf-8").write(u"\n".join(L))
json.dump(rows, io.open(os.path.join(HERE, "_next", "probe060_result.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(u"발화 %d · 미발화 %d · 프레임 %d" % (len(rows) - len(zero), len(zero), frames))
for r in zero: print(u"  0 [%d] %s %s" % (r["idx"], r["rva"], r["name"]))
