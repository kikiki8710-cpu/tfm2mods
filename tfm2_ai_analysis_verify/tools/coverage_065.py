"""0.6.4 에서 측정된(실행된) 지점이 0.6.5 지점 표에 들어 있는지 대조하고, 빠진 것을 추가 지점으로 낸다 (2026-10-21).

입력: 0.6.4 합산 결과 sites.json(stage) · 0.6.5 site_catalog.json · atlas_to_065.py 의 명령 대응
출력(<out>): coverage_065.csv(지점별: 0.6.4 rva·단계 → 0.6.5 rva·표 포함 여부) · extra_v065_cover.txt(표에 없는 대응 지점)
사용: python -B -X utf8 tools/coverage_065.py <0.6.4 sites.json> <migration_065 폴더> <out 폴더>
"""
import collections, csv, json, os, sys, types
HERE = os.path.dirname(os.path.abspath(__file__))
old_json, mig, out = sys.argv[1:4]
os.makedirs(out, exist_ok=True)

# atlas_to_065 의 명령 대응(tr)을 재사용한다 — 모듈이 실행 시 산출물을 쓰므로 임시 폴더로 돌린다.
tmp = os.path.join(out, "_atlas_to_065_tmp")
argv0 = sys.argv
sys.argv = ["atlas_to_065.py", mig, tmp]
mod = types.ModuleType("atlas_to_065")
mod.__file__ = os.path.join(HERE, "atlas_to_065.py")
exec(compile(open(mod.__file__, encoding="utf-8").read(), mod.__file__, "exec"), mod.__dict__)
sys.argv = argv0
tr = mod.tr

old = json.load(open(old_json, encoding="utf-8"))["sites"]
cat = json.load(open(os.path.join(HERE, "sites", "site_catalog.json"), encoding="utf-8"))
new_rvas = {s["rva"] for s in cat["sites"]}
rows, missing = [], []
for s in old:
    if s["stage"] == "미도달":
        continue
    o = int(s["rva"], 16)
    n = tr(o)
    inn = n is not None and n in new_rvas
    rows.append(dict(old=s["rva"], stage=s["stage"], function=s["function"], insn=s["insn"], new=hex(n) if n else "", in_table=int(inn)))
    if n is not None and not inn:
        missing.append((n, s))
with open(os.path.join(out, "coverage_065.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)
with open(os.path.join(out, "extra_v065_cover.txt"), "w", encoding="utf-8") as h:
    h.write("# 0.6.4 에서 측정됐지만 0.6.5 지점 표에 없던 대응 지점 (coverage_065.py)\n")
    for n, s in sorted(missing, key=lambda x: x[0]):
        h.write(f"{n:#x} cover:{s['stage']} (0.6.4 {s['rva']} {s['insn']})\n")
c = collections.Counter((r["stage"], "표에있음" if r["in_table"] else ("대응없음" if not r["new"] else "표에없음")) for r in rows)
print("측정 지점", len(rows), dict(c))
print("추가 지점", len(missing))
