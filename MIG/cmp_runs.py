# -*- coding: utf-8 -*-
# 두 판의 발화수 표 대조 — 같은 리플레이면 ±2~4%, 다른 리플레이면 함수별로 크게 갈린다
import io, re, sys
sys.stdout.reconfigure(encoding="utf-8")
D = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_judge_verify\_r17_probe1"
A, B = sys.argv[1], sys.argv[2]
def hits(p):
    t = io.open(D + "\\" + p, encoding="utf-8", errors="replace").read()
    sec = t.split("--- 발화")[1].split("\n--- ")[0]
    out = {}
    for m in re.finditer(r"^\s*(\d+)\s+0x([0-9a-f]+)\s+(\d+)\s+(\d+)\s+\d+\s+\S+\s+(\S+)", sec, re.M):
        out[m.group(2)] = (m.group(5), int(m.group(4)))
    return out
a, b = hits(A), hits(B)
print("A", len(a), "B", len(b))
rows = []
for k in set(a) | set(b):
    na, nb = a.get(k, ("?", 0))[1], b.get(k, ("?", 0))[1]
    nm = (a.get(k) or b.get(k))[0]
    r = (nb - na) / na if na else float("inf")
    rows.append((abs(r) if r != float("inf") else 9e9, k, nm, na, nb, r))
rows.sort(reverse=True)
big = [x for x in rows if x[0] > 0.10]
print("|Δ|>10%% 인 함수 %d / %d" % (len(big), len(rows)))
for _, k, nm, na, nb, r in rows[:40]:
    print("  %s %-45s %14s %14s %+.1f%%" % (k, nm[:45], "{:,}".format(na), "{:,}".format(nb), r * 100 if r != float("inf") else 0))
