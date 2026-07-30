# -*- coding: utf-8 -*-
# dov_053.py — asset-get clone family(3모드 공통) 형제 매핑 확정.
#   대상 0.5.2 값: comptest LOADER 0x5ac950 / illust ASSET_GET 0x99c860 · ANIM_GET 0x5ab7d0
#                  draft_overlay LOADER 0x40f3d0 · ANIM_GET 0x40e250 · BANPICK_LOADER 0xeb17d0
#   방법: ①각 형제의 콜러 수·크기·진입24B 지문 ②family 전원 열거 ③문자열 xref(lea→직후 call) 용도 판별
import re, collections, bisect
import bytepatch_053 as B      # DO/DN(바이트) SO/SN(섹션) FO/FN(.pdata 함수경계)

roff, owner = B.roff, B.owner


def sec(secs, nm):
    for n, va, vsz, rr, rs in secs:
        if n == nm:
            return va, vsz, rr, rs


def callers(d, secs, fns):
    """.text 전역 e8 스캔 → {target: Counter(caller_fn)}"""
    va, vsz, rr, rs = sec(secs, ".text")
    blob = d[rr:rr + rs]
    starts = [f[0] for f in fns]
    tgt = collections.Counter()
    tgt_fn = collections.defaultdict(set)
    i = 0
    n = len(blob)
    while True:
        i = blob.find(b"\xe8", i)
        if i < 0 or i + 5 > n:
            break
        rel = int.from_bytes(blob[i + 1:i + 5], "little", signed=True)
        site = va + i
        t = site + 5 + rel
        if va <= t < va + rs:
            tgt[t] += 1
            j = bisect.bisect_right(starts, site) - 1
            if j >= 0:
                tgt_fn[t].add(starts[j])
        i += 1
    return tgt, tgt_fn


def find_str(d, secs, lit, where=".rdata"):
    va, vsz, rr, rs = sec(secs, where)
    blob = d[rr:rr + rs]
    return [va + m.start() for m in re.finditer(re.escape(lit), blob)]


def lea_to(d, secs, target):
    va, vsz, rr, rs = sec(secs, ".text")
    blob = d[rr:rr + rs]
    out = []
    for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d](....)', blob, re.S):
        s = va + m.start()
        rel = int.from_bytes(m.group(1), "little", signed=True)
        if s + 7 + rel == target:
            out.append(s)
    return out


def call_after(d, secs, site, window=80):
    o = roff(secs, site)
    blob = d[o:o + window]
    m = re.search(rb'\xe8(....)', blob, re.S)
    if not m:
        return None
    s = site + m.start()
    rel = int.from_bytes(m.group(1), "little", signed=True)
    return s + 5 + rel


def entry(d, secs, rva, n=24):
    o = roff(secs, rva)
    return d[o:o + n] if o is not None else b""


print("콜러 스캔 중...")
CO, CFO = callers(B.DO, B.SO, B.FO)
CN, CFN = callers(B.DN, B.SN, B.FN)
print(f"  0.5.2 call타겟 {len(CO)} / 0.5.3 {len(CN)}\n")

OLD_SIB = {
    "comptest LOADER":        0x5ac950,
    "illust RVA_ASSET_GET":   0x99c860,
    "illust RVA_ANIM_GET":    0x5ab7d0,
    "draft LOADER":           0x40f3d0,
    "draft ANIM_GET":         0x40e250,
    "draft BANPICK_LOADER":   0xeb17d0,
}
print("=" * 100)
print("0.5.2 형제 지문")
print("=" * 100)
fam_sig = collections.Counter()
for nm, r in OLD_SIB.items():
    f = owner(B.FO, r)
    sz = (f[1] - f[0]) if f else 0
    e = entry(B.DO, B.SO, r)
    fam_sig[e] += 1
    print(f"  {nm:24s} 0x{r:<9x} 콜러={CO.get(r,0):<5d} 콜러함수={len(CFO.get(r,()))!s:<5s} "
          f"크기={sz:<6d} fn시작={'0x%x' % f[0] if f else '?'} 진입24B={e.hex(' ')}")

print("\n0.5.2 family(진입24B 동일) 전원 — 콜러 수 스펙트럼")
for e, _ in fam_sig.most_common():
    same = [f[0] for f in B.FO if entry(B.DO, B.SO, f[0]) == e]
    print(f"  sig {e.hex(' ')[:35]}… → {len(same)}개")
    for s in sorted(same, key=lambda x: -CO.get(x, 0))[:12]:
        print(f"      0x{s:<9x} 콜러={CO.get(s,0):<5d} 크기={owner(B.FO,s)[1]-s}")

print("\n" + "=" * 100)
print("0.5.3 family 후보 — 0.5.2 형제들의 진입24B 와 동일한 함수 전원")
print("=" * 100)
for e, _ in fam_sig.most_common():
    same = [f[0] for f in B.FN if entry(B.DN, B.SN, f[0]) == e]
    print(f"  sig {e.hex(' ')[:35]}… → {len(same)}개 (콜러 상위)")
    for s in sorted(same, key=lambda x: -CN.get(x, 0))[:14]:
        print(f"      0x{s:<9x} 콜러={CN.get(s,0):<5d} 크기={owner(B.FN,s)[1]-s}")

print("\n" + "=" * 100)
print("문자열 xref → 직후 call 집계 (용도 판별)")
print("=" * 100)
LITS = [b"asset/base/ui/layout/main", b"asset/base/ui/layout/strategy",
        b"asset/base/ui/layout/training", b"asset/base/ui/layout/player_info",
        b"asset/base/ui/layout/banpick", b"#anim", b"asset/base/ui/image"]
for tag, d, secs, fns, C in (("0.5.2", B.DO, B.SO, B.FO, CO), ("0.5.3", B.DN, B.SN, B.FN, CN)):
    print(f"--- {tag} ---")
    for lit in LITS:
        votes = collections.Counter()
        for a in find_str(d, secs, lit):
            for s in lea_to(d, secs, a):
                t = call_after(d, secs, s)
                if t:
                    votes[t] += 1
        if votes:
            print(f"  {lit.decode():34s} → " +
                  ", ".join(f"0x{t:x}×{c}(콜러{C.get(t,0)})" for t, c in votes.most_common(4)))
        else:
            print(f"  {lit.decode():34s} → (없음)")
