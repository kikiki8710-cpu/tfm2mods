"""0.6.4 → 0.6.5 함수별 '의미 차이' 요약 (2026-10-21).

명령 순서 정렬(func_diff.py)은 컴파일러 재배치에 약하다. 여기서는 함수마다 아래 다중집합을 비교한다.
  상수   = 즉치(이미지 주소 제외)
  필드   = 구조체 필드 변위(베이스가 rip/rsp/rbp 가 아닌 메모리 피연산자)
  호출   = 직접 호출 대상(구 → 신 함수 대응으로 바꿔서 비교)
구 쪽 필드·큰 상수는 '배치 이동 표'(정렬된 같은 명령들에서 일관되게 바뀐 값, func_diff 의 const_pairs)로 먼저 옮긴다.
남는 추가/삭제가 실질 변화 후보다(재배치·레지스터·프레임·구조체 이동은 사라진다).
출력: semantic_065.csv · semantic_065.md (변화 있는 함수만, 변화 큰 순)
사용: python -B -X utf8 tools/func_semantic.py <migration_065 폴더>
"""
import collections, csv, json, os, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
import _x065  # noqa: E402
from _x065 import O, N, PO, PN, insns  # noqa: E402
from capstone.x86 import X86_OP_IMM, X86_OP_MEM  # noqa: E402
_x065.cs.detail = True

mig = sys.argv[1]
atlas = list(csv.DictReader(open(os.path.join(mig, "atlas_064_065.csv"), encoding="utf-8-sig")))
am = json.load(open(os.path.join(mig, "aidiff_map.json"), encoding="utf-8"))
fpair = {int(r["old"], 16): int(r["new"], 16) for r in atlas if r["new"]}
for k, (n, v, _) in am["map"].items():
    fpair.setdefault(int(k, 16), int(n, 16))
_g = os.path.join(mig, "fnmap_global_065.json")  # global_fnmap.py — 아틀라스 밖 피호출자(런타임·std 포함)까지
if os.path.exists(_g):
    for k, v in json.load(open(_g, encoding="utf-8")).items():
        fpair.setdefault(int(k, 16), int(v, 16))

# 배치 이동 표: 같은 명령 자리에서 old→new 로 바뀐 값. old 하나가 거의 한 new 로만 가면 채택.
cp = list(csv.DictReader(open(os.path.join(mig, "const_pairs_065.csv"), encoding="utf-8-sig")))
by_old = collections.defaultdict(collections.Counter)
for r in cp:
    by_old[(r["what"], int(r["old"]))][int(r["new"])] += int(r["count"])
shift = {}
for (what, ov), c in by_old.items():
    nv, cnt = c.most_common(1)[0]
    tot = sum(c.values())
    if cnt / tot < 0.8:
        continue
    if what == "필드변위":
        shift[("d", ov)] = nv
    elif ov >= 0x100 and (nv - ov) % 8 == 0 and abs(nv - ov) <= 0x400:  # 구조체 크기·배열 보폭류만
        shift[("i", ov)] = nv


def feats(img, idx, f, is_old):
    con, fld, cal = collections.Counter(), collections.Counter(), collections.Counter()
    for i in insns(img, f, int(idx[f]["size"])):
        if i.mnemonic == "call":
            for op in i.operands:
                if op.type == X86_OP_IMM:
                    t = (op.imm & 0xFFFFFFFFFFFFFFFF) - 0x140000000
                    if is_old:
                        t = fpair.get(t, ("?", t))
                    cal[t if not isinstance(t, tuple) else f"old:{t[1]:#x}"] += 1
            continue
        if i.mnemonic.startswith("j"):
            continue
        for op in i.operands:
            if op.type == X86_OP_IMM:
                v = op.imm & 0xFFFFFFFFFFFFFFFF
                if 0x140000000 <= v < 0x150000000:
                    continue
                if is_old:
                    v = shift.get(("i", v), v)
                con[v] += 1
            elif op.type == X86_OP_MEM:
                base = i.reg_name(op.mem.base) if op.mem.base else ""
                if base in ("rip", "rsp", "rbp", "esp", "ebp") or not op.mem.disp:
                    continue
                d = op.mem.disp & 0xFFFFFFFFFFFFFFFF
                if is_old:
                    d = shift.get(("d", d), d)
                fld[d] += 1
    return con, fld, cal


def fmt(c, hexa=True):
    return ", ".join(f"{(hex(k) if hexa and isinstance(k, int) else k)}×{v}" if v > 1 else f"{(hex(k) if hexa and isinstance(k, int) else k)}"
                     for k, v in sorted(c.items(), key=lambda x: (-x[1], str(x[0])))[:14])


rows = []
for r in atlas:
    if not r["new"]:
        continue
    o, n = int(r["old"], 16), int(r["new"], 16)
    if o not in PO or n not in PN:
        continue
    co, fo, ko = feats(O, PO, o, True)
    cn, fn, kn = feats(N, PN, n, False)
    d = dict(c_add=cn - co, c_rem=co - cn, f_add=fn - fo, f_rem=fo - fn, k_add=kn - ko, k_rem=ko - kn)
    score = sum(sum(v.values()) for v in d.values())
    scaled = fn.get(0x288, 0) and not fo.get(0x288, 0)
    rows.append((r, d, score, bool(scaled)))

rows.sort(key=lambda x: -x[2])
with open(os.path.join(mig, "semantic_065.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.writer(h)
    w.writerow(["old", "new", "name", "area", "fn_verdict", "score", "debuff_ratio", "const_added", "const_removed", "field_added", "field_removed", "call_added", "call_removed"])
    for r, d, s, sc in rows:
        w.writerow([r["old"], r["new"], r["name"], r["area"], r["verdict"], s, int(sc), fmt(d["c_add"]), fmt(d["c_rem"]), fmt(d["f_add"]), fmt(d["f_rem"]), fmt(d["k_add"]), fmt(d["k_rem"])])
chg = [x for x in rows if x[2] > 0]
with open(os.path.join(mig, "semantic_065.md"), "w", encoding="utf-8") as h:
    h.write(f"# 0.6.4→0.6.5 함수 의미 차이 (func_semantic.py)\n\n함수 {len(rows)} 중 차이 있음 {len(chg)} · 배치 이동 표 {len(shift)}항목 적용 후\n\n")
    for r, d, s, sc in chg:
        h.write(f"## {r['old']} → {r['new']} {r['name']} [{r['area']}] {r['verdict']} · 차이 {s}{' · debuff_ratio 신규' if sc else ''}\n")
        for k, lab in (("c_add", "상수+"), ("c_rem", "상수−"), ("f_add", "필드+"), ("f_rem", "필드−"), ("k_add", "호출+"), ("k_rem", "호출−")):
            if d[k]:
                h.write(f"- {lab} {fmt(d[k])}\n")
        h.write("\n")
dist = collections.Counter("0" if s == 0 else "1-5" if s <= 5 else "6-20" if s <= 20 else "21-80" if s <= 80 else "80+" for _, _, s, _ in rows)
print(len(rows), "차이 있음", len(chg), dict(dist), "debuff 신규", sum(1 for x in rows if x[3]), "shift", len(shift))
