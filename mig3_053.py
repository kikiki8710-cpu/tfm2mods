# -*- coding: utf-8 -*-
# mig3_053.py — comptest_unlock / banpick_illust / draft_overlay 의 함수시작 RVA 를 일괄 재핀.
#   판정 = ①앵커맵 직접쌍 ②정방향 콜러-대응 투표 ③역방향 투표(순도) ④크기비·콜러수 규모
#   dov_053b 에서 검증된 절차(LOADER 193/194·역방향 98%)를 그대로 일반화.
import collections, pickle, sys, io
import bytepatch_053 as B
import dov_053b as G

A, CO, EO, CN, EN = G.A, G.CO, G.EO, G.CN, G.EN
roff, owner = B.roff, B.owner
RA = {}
for k, v in A.items():
    RA.setdefault(v, k)

TARGETS = [
    # (모드, 상수, 0.5.2 RVA, 표의 후보/메모)
    ("comptest", "DISP_RVA",        0xd3f780,  "미해결"),
    ("comptest", "CT_REGION_LO",    0xe7ccd0,  "유력 0x17e0240"),
    ("comptest", "RUN_RVA",         0xd0a440,  "유력 0x18f1180"),
    ("comptest", "LOADING_RVA",     0xd186f0,  "미해결"),
    ("comptest", "FN_DD_SETOPT_RVA",0x242f250, "확정 0x1bfc80(item_tactics 실측)"),
    ("comptest", "ITEMCONV_RVA",    0xed8770,  "확정 0x18429d0"),
    ("comptest", "COLLECT_RVA",     0xd0bd80,  "미해결"),
    ("comptest", "EF1EA0_RVA",      0xe58c30,  "미해결"),
    ("comptest", "ATH_GET_SC_RVA",  0xe3b200,  "확정 0x1794280"),
    ("comptest", "ORACLE_RVA",      0x1d94720, "유력 0xeb6590"),
    ("comptest", "SLOT_RVA",        0xd1acf0,  "확정 0x1904640"),
    ("comptest", "PARSER_RVA",      0x24b5a00, "확정 0x1a6530(실측)"),
    ("comptest", "RUST_ALLOC_RVA",  0x8b7f80,  "0.5.3 분해 → 0x28f7df0 3인자"),
    ("comptest", "RUST_DEALLOC_RVA",0x8b7f90,  "dealloc"),
    ("illust",   "RVA_FX_SET",      0x11e2370, "유력 0x1bd8e50"),
    ("illust",   "RVA_CARD_DRAW",   0x11f9030, "확정 0x1bee8e0"),
    ("illust",   "RVA_ILLUST_GET",  0xfdabe0,  "유력 0x1e91400"),
    ("illust",   "RVA_SUBMIT",      0x248b1c0, "미해결"),
    ("illust",   "RVA_SUBMIT_TEXT", 0x248b400, "미해결"),
    ("illust",   "RVA_IMG_BUILD",   0x248c130, "미해결"),
    ("illust",   "RVA_IMG_UV",      0x248c7c0, "확정 0x186f70"),
    ("illust",   "RVA_IMG_FLAG",    0x248cd40, "확정 0x187420"),
    ("illust",   "RVA_IMG_COLOR",   0xff0c20,  "유력 0x1875b0"),
    ("illust",   "RVA_IMG_SHADER",  0x248e850, "미해결"),
    ("illust",   "RVA_TEXT_BUILD",  0x248c1e0, "확정 0x1165380"),
    ("illust",   "RVA_NAME_GET",    0x1217630, "미해결"),
    ("illust",   "RVA_SPRITE_CALC", 0x121aca0, "유력 0x1c1e4e0"),
    ("illust",   "RVA_GAME_ALLOC",  0x8b7f80,  "= RUST_ALLOC"),
    ("illust",   "RVA_GAME_FREE",   0x8b7f90,  "= RUST_DEALLOC"),
]


def size_of(fns, r):
    f = owner(fns, r)
    return (f[1] - f[0]) if f else 0


def fwd_votes(old):
    """0.5.2 old 를 부르는 함수 → 0.5.3 대응이 부르는 타겟 집계"""
    v = collections.Counter()
    mapped = 0
    for cf, cnt in CO.get(old, {}).items():
        if cf not in A:
            continue
        mapped += 1
        for t, k in EN.get(A[cf], {}).items():
            v[t] += 1 if k == cnt else 0.4
    return v, mapped, len(CO.get(old, {}))


def rev_purity(new, old):
    """0.5.3 new 의 콜러 → 0.5.2 대응이 old 를 부르는 비율"""
    hit = mapped = 0
    for cf in CN.get(new, {}):
        of = RA.get(cf)
        if of is None:
            continue
        mapped += 1
        if old in EO.get(of, {}):
            hit += 1
    return hit, mapped


print("\n" + "=" * 108)
print(f"{'모드':9s} {'상수':18s} {'0.5.2':>9s}  {'→ 0.5.3':>10s} {'판정':6s} 근거")
print("=" * 108)
out = {}
for mod, name, old, memo in TARGETS:
    direct = A.get(old)
    v, mapped, ncall = fwd_votes(old)
    top = v.most_common(3)
    best = top[0][0] if top else None
    so = size_of(B.FO, old)
    lines = []
    cand = None
    grade = "?"
    if direct and (not best or direct == best):
        cand, grade = direct, "확정"
    elif best and top and (len(top) < 2 or top[0][1] >= top[1][1] * 1.5):
        cand, grade = best, "유력"
    elif direct:
        cand, grade = direct, "충돌"
    elif best:
        cand, grade = best, "약함"
    if cand:
        hit, rm = rev_purity(cand, old)
        sn = size_of(B.FN, cand)
        pur = hit * 100 // max(1, rm)
        if grade in ("유력", "약함") and pur >= 80 and rm >= 3:
            grade = "확정"
        if grade == "확정" and rm >= 3 and pur < 60:
            grade = "의심"
        lines.append(f"콜러 {ncall}→{len(CN.get(cand,{}))} 역순도 {pur}%({hit}/{rm}) "
                     f"크기 {so}→{sn}({sn/max(1,so):.2f}) 앵커={'O' if direct==cand else ('X' if direct else '-')}")
        if top and len(top) > 1:
            lines.append(f"2위 0x{top[1][0]:x}({top[1][1]:.0f}표 vs {top[0][1]:.0f})")
    out[(mod, name)] = (old, cand, grade)
    print(f"  {mod:8s} {name:18s} {old:#9x}  {('0x%x' % cand) if cand else '—':>10s} {grade:6s} "
          f"{' | '.join(lines)}")
    if memo:
        print(f"      표: {memo}")

pickle.dump({f"{m}.{n}": (o, c, g) for (m, n), (o, c, g) in out.items()},
            open(r"C:\tfm2mods\_mig3_053.pkl", "wb"))
print("\n저장: _mig3_053.pkl")
