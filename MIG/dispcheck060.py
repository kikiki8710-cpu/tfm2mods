# -*- coding: utf-8 -*-
"""
dispcheck060.py — mig060_same 「오프셋만」 판정의 변위 짝(구→신)을 §A 구조체 표로 역참조해
「같은 필드의 이동」인지 검사한다(교훈: 명령 정렬은 변위 짝의 일관성만 보지, 그 변위가 같은 필드인지는 모른다).
입력 = _next/mig060_same.json(147 · disp=[addr, base reg, old, new]) · 규칙 = spec_patch_060 §A(아래 RULES/PAIRS 에 옮김)
출력 = _next/dispcheck060.md(함수별 판정 · 미설명 짝 목록) · dispcheck060.json
사용: python dispcheck060.py
"""
import io, json, os, sys, collections
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
J = json.load(io.open(os.path.join(HERE, "_next", "mig060_same.json"), encoding="utf-8"))
V = json.load(io.open(os.path.join(HERE, "_spec", "specs20_v060.json"), encoding="utf-8"))
VERD = {sp["v060"]["addr_058"]: sp["v060"]["verdict"] for sp in V["specs"] if sp["v060"]["addr_058"]}

# ── §A 정확 짝(구→신) : 구조체 이름 ──
PAIRS = {}
def P(name, pairs):
    for o, n in pairs: PAIRS.setdefault((o, n), name)
P("PlayerState", [(0x928, 0x9f8), (0x930, 0xa00), (0x9c0, 0xa90), (0x4a0, 0x510), (0x4e8, 0x558), (0x998, 0xa68), (0x448, 0x478), (0x450, 0x480), (0x464, 0x49c), (0x510, 0x580), (0x4f8, 0x568)])
P("Effect vt", [(0x38, 0x50), (0x40, 0x58), (0x48, 0x60), (0x50, 0x68), (0xe8, 0x108)])
P("Action vt", [(0x50, 0x80), (0x60, 0x90)])
P("Blackboard", [(0x78, 0x1d0), (0xf8, 0x2f8), (0x100, 0x300), (0x1e0, 0x3e8)])
P("TeamPlan", [(0x41f, 0xcd5), (0x420, 0xcd6), (0x421, 0xcd7), (0x80, 0xa0), (0x88, 0xa8), (0xc0, 0x2e8), (0x230, 0x520), (0x2d0, 0x5c0), (0x41c, 0xcc4), (0x0, 0x20)])
P("LPH", [(0x990, 0x1408), (0x1800, 0x24a2), (0x1801, 0x24a3), (0x1802, 0x24a6), (0x1803, 0x24a7), (0x1815, 0x24f0), (0x15f8, 0x2218), (0x1610, 0x2230), (0x1618, 0x2238), (0x800, 0x1200), (0x7b8, 0x1188), (0x7c8, 0x1198), (0x7e8, 0x11e8), (0x1b8, 0x3e0)])
P("AgentVerHamster", [(0x2910, 0x3608)])
P("Blackboard goal 레코드(재배열 0xc8/0xd0/0xf8/0x100→0x2f0/0x2f8/0x300/0x308)", [(0xc8, 0x2f0), (0xd0, 0x2f8), (0xf8, 0x300), (0x100, 0x308)])
P("Entity 능력 슬롯 -0x38(기본공격 0x4c8→0x490 · skill0 0x500→0x4c8 · 0x530→0x4f8 · 0x4f8→0x4c0)", [(0x4c8, 0x490), (0x500, 0x4c8), (0x530, 0x4f8), (0x4f8, 0x4c0), (0x538, 0x500)])
P("Entity +0x3fc/+0x400 스왑", [(0x3fc, 0x400), (0x400, 0x3fc)])
P("LPH", [(0x5e8, 0xf58), (0x7c0, 0x1190), (0x868, 0x1268), (0x858, 0x1258), (0x517, 0xdcd), (0x518, 0xdce), (0x519, 0xdcf)])
P("GameCache lead", [(0x21c0, 0x23a0), (0x21d0, 0x23b0), (0x21e0, 0x23c0)])
P("per-champ cache", [(0x320, 0x350)])
P("game_view", [(0x258, 0x260)])
# ── 구간 규칙 (lo, hi, delta, name) : old in [lo,hi) and new-old == delta ──
RULES = [
    (0x928, 0x1000, 0xd0, "PlayerState ≥0x928 +0xd0"),
    (0x4a0, 0x928, 0x70, "PlayerState 0x4a0..0x928 +0x70"),
    (0x448, 0x464, 0x30, "PlayerState 0x448.. +0x30"),
    (0x464, 0x4a0, 0x38, "PlayerState 0x464.. +0x38"),
    (0x58, 0x160, 0x20, "Effect vt ≥0x58 +0x20"),
    (0x38, 0x58, 0x18, "Effect vt 0x38..0x50 +0x18"),
    (0x21c0, 0x2400, 0x1e0, "GameCache ≥0x21c0 +0x1e0"),
    (0x7b8, 0x800, 0x9d0, "LPH 0x7b8..0x800 +0x9d0"),
    (0x800, 0x900, 0xa00, "LPH 0x800..0x900 +0xa00"),
]
# TeamPlan 이 LPH+0xf8 에 인라인: LPH 변위 = 0xf8 + TeamPlan 변위
TP_PAIRS = {(o, n) for (o, n), name in PAIRS.items() if name == "TeamPlan"}
RULES += [

]
def classify(old, new):
    if old == new: return "="
    k = PAIRS.get((old, new))
    if k: return k
    for lo, hi, d, name in RULES:
        if lo <= old < hi and new - old == d: return name
    if (old - 0xf8, new - 0xf8) in TP_PAIRS: return "TeamPlan via LPH+0xf8"
    return None

out = []; unexplained = collections.Counter(); per_fn = []
for r in J:
    disp = r.get("disp") or []
    rows = []; bad = []
    for addr, reg, o, n in disp:
        oi, ni = int(o, 16), int(n, 16); c = classify(oi, ni)
        rows.append((addr, reg, o, n, c))
        if c is None: bad.append((addr, reg, o, n)); unexplained[(o, n)] += 1
    per_fn.append(dict(old=r["old"], new=r["new"], name=r["name"], strict=r.get("strict"), verdict=VERD.get(r["old"], "?"), ndisp=len(disp), nbad=len(bad), bad=bad, rows=rows))

L = [u"# dispcheck060 — 「오프셋만」 변위 짝의 §A 역참조 검사(09-17)", u"",
     u"대상 = mig060_same 147 · 변위 짝 총 %d · 미설명 %d(함수 %d)" % (sum(f["ndisp"] for f in per_fn), sum(f["nbad"] for f in per_fn), sum(1 for f in per_fn if f["nbad"])), u"",
     u"## 미설명 짝 빈도(구→신 · 함수 수)", u"", u"| 구 | 신 | Δ | 건수 | 등장 함수 |", u"|---|---|---|---|---|"]
where = collections.defaultdict(list)
for f in per_fn:
    for addr, reg, o, n in f["bad"]: where[(o, n)].append(u"%s(%s %s)" % (f["old"], reg, addr[-5:]))
for (o, n), cnt in unexplained.most_common():
    L.append(u"| `%s` | `%s` | %+#x | %d | %s |" % (o, n, int(n, 16) - int(o, 16), cnt, u" ".join(where[(o, n)][:8]) + (u" …" if len(where[(o, n)]) > 8 else u"")))
L += [u"", u"## 함수별", u"", u"| 구 | 신 | 함수 | 판정(v060) | strict | 변위 짝 | 미설명 | 미설명 상세 |", u"|---|---|---|---|---|---|---|---|"]
for f in sorted(per_fn, key=lambda x: -x["nbad"]):
    L.append(u"| `%s` | `%s` | %s | %s | %s | %d | %d | %s |" % (f["old"], f["new"], f["name"][:40], f["verdict"][:24], (f["strict"] or "")[:24], f["ndisp"], f["nbad"], u" ".join(u"%s:%s→%s" % (reg, o, n) for _, reg, o, n in f["bad"][:6])))
io.open(os.path.join(HERE, "_next", "dispcheck060.md"), "w", encoding="utf-8").write(u"\n".join(L))
json.dump(per_fn, io.open(os.path.join(HERE, "_next", "dispcheck060.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(u"변위 짝 %d · 미설명 %d · 함수 %d/%d" % (sum(f["ndisp"] for f in per_fn), sum(f["nbad"] for f in per_fn), sum(1 for f in per_fn if f["nbad"]), len(per_fn)))
for (o, n), cnt in unexplained.most_common(40): print(u"  %s→%s Δ%+#x ×%d" % (o, n, int(n, 16) - int(o, 16), cnt))
