"""0.6.5 지점 표 정정 적용 (2026-10-22).

tools/sites065/fix_065.txt 를 읽어
  ① <out>/coverage_065_v2.csv  — 구→신 대응에 remap 적용(0.6.4 측정 지점 재측정 분모)
  ② tools/sites065/extra_v065_fix.txt — add 지점(gen_sites --extra 형식)
  ③ <out>/exclude_065.txt — 분모 제외 지점(현재 site_catalog 기준으로 펼침) `rva 분류 메모`
     + unreached_065.csv 가 주어지면 「0.6.4 데스매치 전용」 지점과, 9회차에 함수째 0회인 데스매치 함수(rl 구역 포함)의 지점을 deathmatch 로 추가
     ⚠ 제외는 9회차 final 에서 0회인 지점에만 적용한다(실행된 지점 = 도달 가능 — RE 주소 범위가 넓어 실행 지점을 덮은 사고, 10-22).
사용: python -B -X utf8 tools/fix_065.py <coverage_065.csv> <out> <9회차 final.json> [unreached_065.csv]
"""
import collections, csv, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
cov, out, final9 = sys.argv[1:4]
unreached = sys.argv[4] if len(sys.argv) > 4 else None
hit9 = {int(s["rva"], 16) for s in json.load(open(final9, encoding="utf-8"))["sites"]}
os.makedirs(out, exist_ok=True)
remap, add, excl, exclr, exclf = {}, [], [], [], []
for line in open(os.path.join(HERE, "sites065", "fix_065.txt"), encoding="utf-8"):
    line = line.split("#", 1)[0].strip() if not line.lstrip().startswith("add") else line.strip()
    if not line or line.startswith("#"):
        continue
    p = line.split()
    k = p[0]
    if k == "remap":
        remap[int(p[1], 16)] = int(p[2], 16)
    elif k == "add":
        add.append((int(p[1], 16), " ".join(p[2:])))
    elif k == "excl":
        excl.append((int(p[1], 16), p[2], " ".join(p[3:])))
    elif k == "exclr":
        exclr.append((int(p[1], 16), int(p[2], 16), p[3], " ".join(p[4:])))
    elif k == "exclf":
        exclf.append((p[1].lower(), p[2], " ".join(p[3:])))

# ① 대응표
rows = list(csv.DictReader(open(cov, encoding="utf-8-sig")))
n = 0
for r in rows:
    o = int(r["old"], 16)
    if o in remap:
        r["new"] = hex(remap[o]); n += 1
with open(os.path.join(out, "coverage_065_v2.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)
# ② 추가 지점
with open(os.path.join(HERE, "sites065", "extra_v065_fix.txt"), "w", encoding="utf-8") as h:
    h.write("# 0.6.5 대응 정정으로 추가하는 지점 (fix_065.py · RE 2026-10-22_065_미도달원인)\n")
    for rva, note in add:
        h.write(f"{rva:#x} fix:{note}\n")
# ③ 분모 제외
cat = json.load(open(os.path.join(HERE, "sites", "site_catalog.json"), encoding="utf-8"))["sites"]
fn = {s["rva"]: (s["function"].lower(), s["area"]) for s in cat}
ex = {}
for rva, cls, note in excl:
    if rva in fn:
        ex[rva] = (cls, note)
for a, b, cls, note in exclr:
    for rva in fn:
        if a <= rva <= b:
            ex.setdefault(rva, (cls, note))
for f, cls, note in exclf:
    for rva, (g, _) in fn.items():
        if g == f:
            ex.setdefault(rva, (cls, note))
if unreached:
    ur = list(csv.DictReader(open(unreached, encoding="utf-8-sig")))
    why = collections.defaultdict(set)
    for r in ur:
        why[r["function"].lower()].add(r["why"])
    fhit = collections.Counter(fn[r][0] for r in hit9 if r in fn)
    dmf = {f for f, w in why.items() if w == {"데스매치 전용"} and fhit[f] == 0}
    for r in ur:
        if r["why"] == "데스매치 전용":
            ex.setdefault(int(r["new"], 16), ("deathmatch", "0.6.4 데스매치(3회차) 전용"))
    for rva, (g, area) in fn.items():
        if g in dmf or (area == "rl" and fhit[g] == 0):
            ex.setdefault(rva, ("deathmatch", f"데스매치 전용 함수 {g}"))
kept = sum(1 for r in ex if r in hit9)
ex = {r: v for r, v in ex.items() if r not in hit9}
print("실행된 지점이라 제외 안 함", kept)
with open(os.path.join(out, "exclude_065.txt"), "w", encoding="utf-8") as h:
    h.write("# 분모 제외 지점 (fix_065.py) — rva 분류 메모\n")
    for rva in sorted(ex):
        h.write(f"{rva:#x} {ex[rva][0]} {ex[rva][1]}\n")
print("remap", n, "· add", len(add), "· 제외", len(ex), dict(collections.Counter(v[0] for v in ex.values())))
