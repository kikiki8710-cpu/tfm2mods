# -*- coding: utf-8 -*-
# mig3_053c.py — 앵커 없는 4건(DISP/LOADING/EF1EA0/dealloc) 을 지문으로 확정.
#   DISP  : 거부메시지 문자열 → 오프셋 테이블 → 테이블을 lea 하는 함수
#   LOADING/EF1EA0 : 유일콜러의 0.5.3 대응 안의 call 타겟을 크기·프롤로그·콜러수로 필터
#   dealloc: alloc impl(0x28f7df0) 인접 + 0.5.2 dealloc 실체 지문(3인자·크기 51)
import collections, re, struct
import bytepatch_053 as B
import dov_053b as G

A, CO, EO, CN, EN = G.A, G.CO, G.EO, G.CN, G.EN
roff, owner = B.roff, B.owner


def sec(secs, nm):
    for n, va, vsz, rr, rs in secs:
        if n == nm:
            return va, vsz, rr, rs


def head(d, secs, r, n=16):
    o = roff(secs, r)
    return d[o:o + n].hex(' ') if o is not None else "?"


def size_of(fns, r):
    f = owner(fns, r)
    return (f[1] - f[0]) if f else 0


def find_str(d, secs, lit, where=".rdata"):
    va, vsz, rr, rs = sec(secs, where)
    return [va + m.start() for m in re.finditer(re.escape(lit), d[rr:rr + rs])]


def qwords_to(d, secs, target):
    """target(RVA)의 절대주소(ib+rva)를 담은 8바이트 워드 위치를 .rdata/.data 에서"""
    out = []
    for w in (".rdata", ".data"):
        s = sec(secs, w)
        if not s:
            continue
        va, vsz, rr, rs = s
        pat = struct.pack("<Q", target)
        for m in re.finditer(re.escape(pat), d[rr:rr + rs]):
            out.append((w, va + m.start()))
    return out


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


IBO, IBN = B.IBO, B.IBN
print("=" * 100)
print("① DISP_RVA — 거부 메시지 문자열 테이블 경로")
print("=" * 100)
for tag, d, secs, fns, ib in (("0.5.2", B.DO, B.SO, B.FO, IBO), ("0.5.3", B.DN, B.SN, B.FN, IBN)):
    lit = b"training.comp_test.not_enough_roster"
    addrs = find_str(d, secs, lit)
    print(f"  {tag}: 문자열 {len(addrs)}곳 " + ", ".join(f"0x{a:x}" for a in addrs[:3]))
    for a in addrs[:2]:
        tbl = qwords_to(d, secs, ib + a)
        print(f"     abs=0x{ib+a:x} → 테이블 참조 {len(tbl)}곳 " +
              ", ".join(f"{w}@0x{r:x}" for w, r in tbl[:4]))
        for w, r in tbl[:3]:
            # 테이블 시작(엔트리 index 2 = -0x20) 을 lea 하는 사이트
            for base_off in (0, -0x10, -0x20, -0x30, -0x40):
                sites = lea_to(d, secs, r + base_off)
                if sites:
                    for s in sites[:4]:
                        f = owner(fns, s)
                        print(f"        lea 테이블{base_off:+#x} @0x{s:x} → 함수 0x{f[0]:x} "
                              f"크기={f[1]-f[0]} 선두16B={head(d,secs,f[0])}")

print("\n" + "=" * 100)
print("② LOADING / EF1EA0 — 유일콜러의 0.5.3 대응 내부 call 후보 필터")
print("=" * 100)
CASES = [("LOADING_RVA", 0xd186f0, 0xcf7e40,
          bytes.fromhex("55415741565657534881ec88000000")),
         ("EF1EA0_RVA", 0xe58c30, 0xec3b40,
          bytes.fromhex("55415741564155415456575348")),
         ("ITEMCONV_RVA", 0xed8770, 0xec3b40,
          bytes.fromhex("41574156415541545657555348"))]
for nm, old, caller, prol in CASES:
    so = size_of(B.FO, old)
    nf = A.get(caller)
    print(f"  {nm} 0.5.2=0x{old:x} 크기={so} 콜러=0x{caller:x} → 0.5.3 콜러={'0x%x' % nf if nf else '없음'}")
    if not nf:
        continue
    cands = set(EN.get(nf, {}))
    ok = []
    for t in cands:
        st = size_of(B.FN, t)
        o = roff(B.SN, t)
        h = B.DN[o:o + len(prol)] if o is not None else b""
        r = st / max(1, so)
        if 0.7 <= r <= 1.8:
            ok.append((abs(r - 1), t, st, h == prol, h.hex(' ')[:32]))
    ok.sort()
    print(f"     콜리 {len(cands)}개 중 크기비 0.7~1.8 = {len(ok)}개")
    for _, t, st, pm, h in ok[:8]:
        print(f"        0x{t:<9x} 크기={st:<6d}(비 {st/max(1,so):.2f}) 프롤로그일치={'★YES' if pm else 'no'} "
              f"콜러={sum(CN.get(t,{}).values()):<5d} {h}")

print("\n" + "=" * 100)
print("③ dealloc 실체 — 0.5.2 0x25c4d90 ↔ 0.5.3 alloc impl 0x28f7df0 인접")
print("=" * 100)
print(f"  0.5.2 alloc 실체 0x25c4d30 크기={size_of(B.FO,0x25c4d30)} 선두={head(B.DO,B.SO,0x25c4d30,20)}")
print(f"  0.5.2 dealloc실체 0x25c4d90 크기={size_of(B.FO,0x25c4d90)} 선두={head(B.DO,B.SO,0x25c4d90,20)}")
starts = sorted(f[0] for f in B.FN)
i = starts.index(0x28f7df0) if 0x28f7df0 in starts else -1
print(f"  0.5.3 alloc impl 0x28f7df0 크기={size_of(B.FN,0x28f7df0)} 선두={head(B.DN,B.SN,0x28f7df0,20)}")
if i >= 0:
    for j in range(max(0, i - 2), min(len(starts), i + 6)):
        s = starts[j]
        print(f"     {'→' if j==i else ' '} 0x{s:<9x} 크기={size_of(B.FN,s):<5d} "
              f"콜러={sum(CN.get(s,{}).values()):<6d} 선두20B={head(B.DN,B.SN,s,20)}")
