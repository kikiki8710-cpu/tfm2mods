# -*- coding: utf-8 -*-
# illust_054.py — banpick_illust 0.5.3→0.5.4 재핀 ①.rdata float 상수 ②SLOTS
import re, struct
import bp054 as B
roff, sec = B.roff, B.sec

CONSTS = [("RVA_C_CARD_RECT", 0x32a7ad0, 4), ("RVA_C_SNAP_RECT", 0x32a7b00, 4),
          ("RVA_C_NORMAL", 0x32a7b10, 2), ("RVA_C_LINE_DIR", 0x32a7b30, 2),
          ("RVA_C_LINE_START", 0x32a7b40, 2), ("RVA_C_LINE_ANCHOR", 0x32a7b50, 2)]

def search(d, secs, pat, where=(".rdata", ".data")):
    out = []
    for w in where:
        s = sec(secs, w)
        if not s: continue
        va, vsz, rr, rs = s
        for m in re.finditer(re.escape(pat), d[rr:rr + rs]):
            out.append((w, va + m.start()))
    return out

print("="*100)
print("① .rdata float 상수 — 0.5.3 바이트를 0.5.4 에서 검색")
print("="*100)
deltas = []
for nm, r, n in CONSTS:
    o = roff(B.SO, r)
    pat = B.DO[o:o+n*4]
    vals = struct.unpack("<"+"f"*n, pat)
    hits = search(B.DN, B.SN, pat)
    print(f"  {nm:18s} 0.5.3 0x{r:<9x} {list(vals)} → 0.5.4 후보 {len(hits)}개: "
          + ", ".join(f"{w}@0x{a:x}" for w,a in hits[:8]))
    if len(hits)==1: deltas.append((nm, hits[0][1]-r))

print("\n  ▶ 블록 통째 검색 0x32a7ad0..0x32a7b60 (0x90B)")
o = roff(B.SO, 0x32a7ad0)
blk = B.DO[o:o+0x90]
hits = search(B.DN, B.SN, blk)
print("     → " + ", ".join(f"{w}@0x{a:x} (delta {a-0x32a7ad0:+#x})" for w,a in hits[:6]))
print("  단일매치 델타들:", [(n, hex(d)) for n,d in deltas])

print("\n" + "="*100)
print("② RVA_SLOTS — 0.5.4 .rdata 의 32B 이상 0 패딩 후보")
print("="*100)
o = roff(B.SO, 0x3f11000)
print(f"  0.5.3 슬롯 0x3f11000 주변 48B: {B.DO[o-16:o+32].hex(' ')}")
va, vsz, rr, rs = sec(B.SN, ".rdata")
blob = B.DN[rr:rr+rs]
runs = []
for m in re.finditer(rb'\x00{4096,}', blob):
    runs.append((m.end()-m.start(), va+m.start(), va+m.end()))
runs.sort(reverse=True)
print(f"  0.5.4 .rdata(0x{va:x}~0x{va+rs:x}) 4096B+ 0런 상위:")
for L,a,e in runs[:5]:
    print(f"     0x{a:x}..0x{e:x}  {L}B")
if runs:
    L,a,e = runs[0]
    cand = (a + 0xfff) & ~0xfff
    if cand+16 <= e:
        print(f"  ▶ 권장 SLOTS = 0x{cand:x} (최장런 내부 4K 정렬)")
