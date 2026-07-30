# -*- coding: utf-8 -*-
# desc_053.py — probe_basedmg_r9 가 shadow-CALL 의 this 로 쓰는 .rdata desc 2종을 0.5.3 에서 재핀.
#   C8C  : `lea r9,[rip→desc]` 직후 `call [r10+0x28]`(41 ff 52 28) 사이트의 타겟
#   DISC7: disc19 핸들러(0.5.3 = 0xdece30) 안의 `lea r9,[rip→desc]` 타겟
#   검증  : desc 레이아웃 {vtable_ptr, size=0x6a8, align=8} sanity 로 오답 배제.
#   이 값이 틀리면 임의 바이트를 vtable 로 삼아 호출 = AV(0.5.2 disc14 크래시 진범) ⟹ sanity 통과분만 채택.
import sys, re, json
import bytepatch_053 as B

DO, DN, SO, SN, FO, FN = B.DO, B.DN, B.SO, B.SN, B.FO, B.FN
roff, owner = B.roff, B.owner


def tb(d, secs):
    for nm, va, vsz, rr, rs in secs:
        if nm == ".text":
            return va, d[rr:rr + rs]


VAO, BO = tb(DO, SO)
VAN, BN = tb(DN, SN)


def u64(d, secs, rva):
    o = roff(secs, rva)
    return int.from_bytes(d[o:o + 8], "little") if o else None


def sane_desc(d, secs, rva, ib):
    """desc = {vtable_ptr, 0x6a8, 8} 인지 검사"""
    o = roff(secs, rva)
    if o is None:
        return None
    vt = int.from_bytes(d[o:o + 8], "little")
    sz = int.from_bytes(d[o + 8:o + 16], "little")
    al = int.from_bytes(d[o + 16:o + 24], "little")
    return dict(vt=vt, size=sz, align=al,
                ok=(sz == 0x6a8 and al == 8 and ib <= vt < ib + 0x5000000))


def lea_r9_sites(d, secs, blob, va, lo=None, hi=None, need_call=True):
    """`4c 8d 0d <rel32>` (lea r9,[rip+x]) — need_call 이면 뒤에 call [r10+0x28] 동반 조건"""
    out = []
    pat = re.compile(rb'\x4c\x8d\x0d(....)', re.S)
    for m in pat.finditer(blob):
        site = va + m.start()
        if lo is not None and not (lo <= site < hi):
            continue
        rel = int.from_bytes(m.group(1), "little", signed=True)
        tgt = site + 7 + rel
        tail = blob[m.start() + 7: m.start() + 7 + 32]
        has = b"\x41\xff\x52\x28" in tail
        if need_call and not has:
            continue
        out.append((site, tgt, has))
    return out


print("=" * 76)
print("① OLD 실측 — 0.5.2 확정 desc 의 호출부 지문 확인 (방법 검증)")
print("=" * 76)
for nm, rva in [("C8C", 0x381e1e0), ("DISC7", 0x38d1918)]:
    s = sane_desc(DO, SO, rva, B.IBO)
    print(f"  {nm} 0x{rva:x} → vt={s['vt']:#x} size={s['size']:#x} align={s['align']} sane={s['ok']}")
old_sites = lea_r9_sites(DO, SO, BO, VAO)
print(f"  OLD `lea r9,[rip]` + `call [r10+0x28]` 사이트: {len(old_sites)}건")
tg = {}
for site, tgt, _ in old_sites:
    tg.setdefault(tgt, []).append(site)
for t, ss in sorted(tg.items()):
    s = sane_desc(DO, SO, t, B.IBO)
    mark = ""
    if t == 0x381e1e0:
        mark = "  ←C8C(확정)"
    if t == 0x38d1918:
        mark = "  ←DISC7(확정)"
    print(f"    desc 0x{t:x} sane={s['ok'] if s else None} 사이트{len(ss)}건 {[hex(x) for x in ss[:4]]}{mark}")

print()
print("=" * 76)
print("② NEW 0.5.3 — 같은 지문")
print("=" * 76)
new_sites = lea_r9_sites(DN, SN, BN, VAN)
print(f"  NEW 사이트: {len(new_sites)}건")
tgn = {}
for site, tgt, _ in new_sites:
    tgn.setdefault(tgt, []).append(site)
for t, ss in sorted(tgn.items()):
    s = sane_desc(DN, SN, t, B.IBN)
    fset = sorted({owner(FN, x)[0] for x in ss if owner(FN, x)})
    print(f"    desc 0x{t:x} sane={s['ok'] if s else None} 사이트{len(ss)}건 "
          f"fn={[hex(x) for x in fset[:4]]}")

print()
print("=" * 76)
print("③ DISC7 = disc19 핸들러(0.5.3 0xdece30) 내부의 desc")
print("=" * 76)
f = owner(FN, 0xdece30)
print(f"  disc19 범위 0x{f[0]:x}..0x{f[1]:x}")
inner = lea_r9_sites(DN, SN, BN, VAN, f[0], f[1], need_call=False)
for site, tgt, has in inner:
    s = sane_desc(DN, SN, tgt, B.IBN)
    print(f"    @0x{site:x} → desc 0x{tgt:x} sane={s['ok'] if s else None} call[r10+0x28]동반={has}")
# 0.5.2 disc19(0x2380820) 내부와 비교
fo = owner(FO, 0x2380820)
print(f"  (참고) 0.5.2 disc19 0x{fo[0]:x}..0x{fo[1]:x}")
for site, tgt, has in lea_r9_sites(DO, SO, BO, VAO, fo[0], fo[1], need_call=False):
    s = sane_desc(DO, SO, tgt, B.IBO)
    print(f"    @0x{site:x} → desc 0x{tgt:x} sane={s['ok'] if s else None} call동반={has}")
