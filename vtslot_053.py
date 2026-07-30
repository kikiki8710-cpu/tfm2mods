# -*- coding: utf-8 -*-
# vtslot_053.py — ai_adjust `disc19_repro.rs` 의 게임 vtable 슬롯 RVA 테이블(match 아암)을
#   0.5.2 → 0.5.3 으로 재핀하기 위한 1단계 진단.
#   ① 소스에서 match 아암 RVA 전수 추출
#   ② 각 RVA 가 0.5.2 exe 에서 "함수 시작"인지 판정(=베이스 버전 확인. 아니면 0.5.1 잔재)
#   ③ 앵커맵(_anchor_052_053.pkl) 조회 → 0.5.3 후보
#   ④ 후보 검증: 크기 / 진입 24B 바이트 동일성 / 콜러 수
# 사용: python vtslot_053.py
import re, sys, io, pickle, collections, bisect
import bytepatch_053 as B      # DO/DN(바이트) SO/SN(섹션) FO/FN(.pdata 함수경계) — 이 모듈이 stdout 을 utf-8 로 감싼다

SRC = r"C:\tfm2mods\tfm2_ai_adjust\src\disc19_repro.rs"
ANCHOR = r"C:\tfm2mods\_anchor_052_053.pkl"

roff, owner = B.roff, B.owner


def fstart_set(fns):
    return {f[0] for f in fns}


def fsize(fns, rva):
    f = owner(fns, rva)
    return (f[1] - f[0]) if f else 0


def entry(d, secs, rva, n=24):
    o = roff(secs, rva)
    return d[o:o + n] if o is not None else b""


def callers(d, secs, fns):
    """.text 전역 e8/e9 스캔 → Counter(target)"""
    for n, va, vsz, rr, rs in secs:
        if n == ".text":
            break
    blob = d[rr:rr + rs]
    tgt = collections.Counter()
    i = 0
    n_ = len(blob)
    while True:
        i = blob.find(b"\xe8", i)
        if i < 0 or i + 5 > n_:
            break
        rel = int.from_bytes(blob[i + 1:i + 5], "little", signed=True)
        t = va + i + 5 + rel
        if va <= t < va + rs:
            tgt[t] += 1
        i += 1
    return tgt


# ── ① 소스에서 match 아암 RVA 추출 ────────────────────────────────
txt = open(SRC, encoding="utf-8").read()
lines = txt.splitlines()
sites = collections.OrderedDict()      # rva -> [(lineno, 원문)]
for ln, s in enumerate(lines, 1):
    # match 아암: `0xXXXX =>` 또는 `0xXXXX |`
    for m in re.finditer(r"0x([0-9a-f]{6,7})\s*(=>|\|)", s):
        r = int(m.group(1), 16)
        sites.setdefault(r, []).append((ln, s.strip()[:100]))

print(f"[1] disc19_repro.rs match 아암 고유 RVA = {len(sites)}종\n")

# ── ② 0.5.2 / 0.5.3 함수시작 판정 ────────────────────────────────
FSO = fstart_set(B.FO)
FSN = fstart_set(B.FN)

print("[2] 콜러 스캔 중...", file=sys.stderr)
CO = callers(B.DO, B.SO, B.FO)
CN = callers(B.DN, B.SN, B.FN)
print(f"    0.5.2 call타겟 {len(CO)} / 0.5.3 {len(CN)}\n", file=sys.stderr)

_raw_anc = pickle.load(open(ANCHOR, "rb"))
anc = {}
for k, v in _raw_anc.items():
    kk = int(k, 16) if isinstance(k, str) else int(k)
    vv = int(v, 16) if isinstance(v, str) else int(v)
    anc[kk] = vv
print(f"[3] 앵커맵 {len(anc)}쌍 로드\n")

rows = []
for r in sorted(sites):
    in052 = r in FSO
    e052 = entry(B.DO, B.SO, r)
    sz052 = fsize(B.FO, r) if in052 else 0
    cand = anc.get(r)
    ok_e = ok_sz = None
    csz = 0
    if cand:
        e053 = entry(B.DN, B.SN, cand)
        csz = fsize(B.FN, cand)
        ok_e = (e053 == e052)
        ok_sz = (csz == sz052)
    rows.append(dict(rva=r, in052=in052, sz=sz052, e=e052, co=CO.get(r, 0),
                     cand=cand, csz=csz, cn=CN.get(cand, 0) if cand else 0,
                     ok_e=ok_e, ok_sz=ok_sz, lines=sites[r]))

n052 = sum(1 for x in rows if x["in052"])
nanc = sum(1 for x in rows if x["cand"])
nok = sum(1 for x in rows if x["ok_e"])
print(f"[4] 요약: 0.5.2 함수시작 {n052}/{len(rows)} · 앵커맵 히트 {nanc}/{len(rows)} · 진입24B 동일 {nok}/{len(rows)}\n")

print("=" * 132)
print(f"{'0.5.2 RVA':<12}{'fn시작':<7}{'크기':<7}{'콜러':<7}| {'0.5.3 후보':<12}{'크기':<7}{'콜러':<7}{'진입=':<6}{'크기=':<6} 소스줄")
print("=" * 132)
for x in rows:
    c = f"0x{x['cand']:x}" if x["cand"] else "-"
    print(f"0x{x['rva']:<10x}{'Y' if x['in052'] else '·':<7}{x['sz']:<7}{x['co']:<7}| "
          f"{c:<12}{x['csz'] or '-':<7}{x['cn'] or '-':<7}"
          f"{('Y' if x['ok_e'] else 'N') if x['cand'] else '-':<6}"
          f"{('Y' if x['ok_sz'] else 'N') if x['cand'] else '-':<6} L{x['lines'][0][0]}")

print("\n" + "=" * 132)
print("0.5.2 에서 함수시작이 아닌 RVA (= 0.5.1 잔재 의심 · 0.5.2 마이그에서도 미재핀)")
print("=" * 132)
for x in rows:
    if not x["in052"]:
        ow = owner(B.FO, x["rva"])
        print(f"  0x{x['rva']:<10x} 소속함수={'0x%x' % ow[0] if ow else '(.text 밖/미매핑)'} L{x['lines'][0][0]}  {x['lines'][0][1]}")
