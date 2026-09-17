# -*- coding: utf-8 -*-
"""probe060_delta.py — probe060.txt 에서 [MARK …] 이후 구간의 함수별 발화 증분(실경기 등 특정 구간 채집) · 마크 이전 미발화였다가 발화한 함수"""
import io, re, sys, os
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
LOG = sys.argv[1] if len(sys.argv) > 1 else r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_judge_verify060\probe060.txt"
TBL = os.path.join(HERE, "..", "tfm2_judge_verify060", "src", "probe_tbl.rs")
names = {int(m.group(1)): (m.group(2), m.group(3)) for m in re.finditer(r"idx: (\d+), rva: 0x([0-9a-f]+), name: \"([^\"]*)\"", io.open(TBL, encoding="utf-8").read())}
before, after, rets_after = {}, {}, {}
seen_mark = False
for line in io.open(LOG, encoding="utf-8", errors="replace"):
    if line.startswith("[MARK"): seen_mark = True; continue
    if line.startswith("#####"):            # 새 INIT(다른 인스턴스) — 마크 뒤 첫 INIT 는 중복 인스턴스였음 → 카운터 리셋 구분 위해 표기만
        continue
    m = re.match(r"\s*\[(\d+)\] ([0-9a-f]+) (\S+) hits=(\d+) \(\+(\d+)\) rets\[([^\]]*)\]", line)
    if not m: continue
    i, h = int(m.group(1)), int(m.group(4))
    if not seen_mark: before[i] = h
    else:
        # 같은 인스턴스(16328)의 누적 hits 는 단조 증가 · 중복 인스턴스(7812)는 작은 값에서 시작 → 마크 이전 값보다 작으면 무시
        if h >= before.get(i, 0): after[i] = h; rets_after[i] = m.group(6).split()
rows = []
for i in sorted(names):
    b = before.get(i, 0); a = after.get(i, b)
    rows.append((a - b, b, a, i, names[i][0], names[i][1], rets_after.get(i, [])))
newly = [r for r in rows if r[1] == 0 and r[0] > 0]
still0 = [r for r in rows if r[2] == 0]
L = [u"# probe060 실경기 구간 증분(MARK 이후 · Bo3 3세트 즉시결과)", u"", u"마크 이전 미발화 → 실경기에서 발화 %d · 여전히 미발화 %d" % (len(newly), len(still0)), u"",
     u"## 실경기에서 처음 발화", u""] + [u"- [%d] `%s` %s +%d rets[%s]" % (r[3], r[4], r[5], r[0], u" ".join(r[6])) for r in newly]
L += [u"", u"## 여전히 미발화(배경 sim + 실경기 Bo3)", u""] + [u"- [%d] `%s` %s" % (r[3], r[4], r[5]) for r in still0]
L += [u"", u"## 실경기 증분 상위 30", u"", u"| idx | rva | 함수 | 증분 | 누적 |", u"|---|---|---|---|---|"]
L += [u"| %d | `%s` | %s | %d | %d |" % (r[3], r[4], r[5][:44], r[0], r[2]) for r in sorted(rows, key=lambda r: -r[0])[:30]]
io.open(os.path.join(HERE, "_next", "probe060_delta_match.md"), "w", encoding="utf-8").write(u"\n".join(L))
print(u"실경기 신규 발화 %d · 여전히 0: %d" % (len(newly), len(still0)))
for r in newly: print(u"  + [%d] %s %s +%d" % (r[3], r[4], r[5], r[0]))
for r in still0: print(u"  0 [%d] %s %s" % (r[3], r[4], r[5]))
