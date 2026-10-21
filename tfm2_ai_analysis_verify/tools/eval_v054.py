"""v0.5.4 추가 지점 평가 (2026-10-21).

M1(손잡이 명령)  : extra_v054_m1.txt 의 후보별 지점 → 실측 단계·값 범위·출현 값·분기 결과.
M2(블록 입구)    : extra_v054_m2.txt 의 손잡이별 지배자 사슬 → 어디까지 실행됐나(경계 블록) + 그 블록 끝 분기·직전 비교의 실측.
사용: python -B -X utf8 tools/eval_v054.py <site_report 폴더> <site_catalog.json> <출력 md>
"""
import bisect, collections, csv, json, os, sys
import pefile, capstone
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gate_analysis import EXE, ATLAS, build_cfg, dominators, jcc  # noqa: E402

rep, cat_p, out = sys.argv[1:4]
HERE = os.path.dirname(os.path.abspath(__file__))
S = {int(s["rva"], 16): s for s in json.load(open(os.path.join(rep, "sites.json"), encoding="utf-8"))["sites"]}
cat = {s["rva"]: s for s in json.load(open(cat_p, encoding="utf-8"))["sites"]}


def site_of(r):
    if r in cat:
        return r
    c = [x for x in cat if r - 15 <= x < r]
    return max(c) if c else None


def desc(r):
    s = S.get(r)
    if not s:
        return "지점 없음"
    h = s["match_hits"] + s["menu_hits"]
    t = f"{s['stage']} {h:,}회"
    if s["lhs_min"] is not None:
        t += f" 값 {s['lhs_min']}..{s['lhs_max']}"
    if s.get("values_seen") and len(s["values_seen"]) <= 8:
        t += f" 출현 {s['values_seen']}"
    if s["kind"] != "VALUE" and h:
        t += f" 참 {s['cond_true']:,}/거짓 {s['cond_false']:,}"
    return t


L = ["# v0.5.4 추가 지점 평가 — 상세전술 손잡이(M1)·블록 입구(M2) (0.6.4 · 2~7회차)", ""]
# ── M1
g = collections.OrderedDict()
for l in open(os.path.join(HERE, "sites", "extra_v054_m1.txt"), encoding="utf-8"):
    l = l.split("#")[0].strip()
    if l:
        p = l.split(None, 2)
        g.setdefault(p[1], []).append((int(p[0], 16), p[2] if len(p) > 2 else ""))
L += ["## M1 — 후보별 손잡이 실측", "", "| 후보 | 지점 | 실행 | 양쪽 이상 |", "|---|---:|---:|---:|"]
summ = []
for k, rs in g.items():
    st = [S.get(site_of(r)) for r, _ in rs]
    ex = sum(1 for s in st if s and s["stage"] != "미도달")
    both = sum(1 for s in st if s and s["stage"] in ("양쪽", "경계확정"))
    summ.append((k, len(rs), ex, both))
    L.append(f"| {k} | {len(rs)} | {ex} | {both} |")
L.append("")
for k, rs in g.items():
    L += [f"### {k}", "", "| 지점 | 명령 | 메모 | 실측 |", "|---|---|---|---|"]
    for r, note in rs:
        s0 = site_of(r)
        L.append(f"| `{hex(r)}` | `{cat[s0]['insn'] if s0 else '-'}` | {note} | {desc(s0) if s0 else '지점 없음'} |")
    L.append("")
# ── M2
funcs = {}
for r in csv.DictReader(open(os.path.join(ATLAS, "functions.csv"), encoding="utf-8-sig")):
    funcs[int(r["RVA(0.6.4)"], 16)] = int(r["끝RVA"], 16)
starts = sorted(funcs)
pe = pefile.PE(EXE, fast_load=True)
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail = True; md.skipdata = True
knobs = collections.OrderedDict()
for l in open(os.path.join(HERE, "sites", "extra_v054_m2.txt"), encoding="utf-8"):
    l = l.split("#")[0].strip()
    if not l:
        continue
    p = l.split()
    i = p.index("손잡이")
    knobs.setdefault((p[i + 1], " ".join(p[2:i])), True)
executed = {r for r, s in S.items() if s["stage"] != "미도달"}
cache = {}
L += ["## M2 — 손잡이까지 흐름이 끊기는 곳", "",
      "경계 블록 = 손잡이 블록의 지배자 사슬 중 실행이 확인된 가장 깊은 블록. 그 블록 끝 분기가 손잡이 쪽으로 안 간 것이 막는 조건이다.", "",
      "| 후보 | 손잡이 | 경계 블록 | 블록 끝 분기 | 직전 비교(실측) | 미실행 사슬 |", "|---|---|---|---|---|---:|"]
for (knob, name) in knobs:
    k = int(knob, 16)
    i = bisect.bisect_right(starts, k) - 1
    if i < 0:
        continue
    st = starts[i]; en = funcs[st]
    if st not in cache:
        insns = list(md.disasm(pe.get_data(st, en - st + 1), st))
        leaders, blocks, succ = build_cfg(insns, st, en)
        idom, _ = dominators(st, list(blocks), succ)
        cache[st] = (leaders, blocks, idom)
    leaders, blocks, idom = cache[st]
    blk = lambda a: leaders[bisect.bisect_right(leaders, a) - 1]
    exb = {blk(a) for a in executed if st <= a <= en} | {st}
    chain, n = [], blk(k)
    while n in idom:
        chain.append(n)
        if n in exb or idom[n] == n:
            break
        n = idom[n]
    front = chain[-1]
    last = blocks[front][-1]
    prod = next((x for x in reversed(blocks[front][:-1]) if x.mnemonic in ("cmp", "test", "or", "and", "sub", "add", "dec", "inc", "bt", "xor")), None)
    pdesc = f"`{prod.mnemonic} {prod.op_str}` " + (desc(prod.address) if prod and prod.address in S else "미계측") if prod else "-"
    L.append(f"| {name} | `{knob}` | `{hex(front)}` | `{last.mnemonic} {last.op_str}` | {pdesc} | {len(chain) - 1} |")
open(out, "w", encoding="utf-8", newline="\n").write("\n".join(L) + "\n")
print("\n".join(f"{k}: {n}지점 실행 {e} 양쪽 {b}" for k, n, e, b in summ))
