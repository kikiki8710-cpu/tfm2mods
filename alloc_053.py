# -*- coding: utf-8 -*-
# alloc_053.py — Rust 전역 할당자 3종(alloc/dealloc/realloc) 클러스터로 ALLOC_RVA 를 재핀.
#   근거: 0.5.2 에서 alloc(0x25c4d30) / dealloc(0x25c4d90) / realloc(0x25c4dd0) 이 0x60·0x40 간격으로 인접.
#         0.5.3 realloc = 0x28e3b10 이 별도 매칭에서 "확정"이므로, 그 이웃 함수들을 경계 단위로 훑어 대응을 찾는다.
#   ALLOC 은 후킹이 아니라 직접 CALL 대상이라 주소가 틀리면 즉시 크래시 → 실측 필수.
import struct, sys, io
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
md = Cs(CS_ARCH_X86, CS_MODE_64)


def load(p):
    d = open(p, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    opt = pe + 24
    ib = struct.unpack_from("<Q", d, opt + 24)[0]
    sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(nsec):
        o = sectab + i * 40
        nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, vsz, rraw, rsz))
    magic = struct.unpack_from("<H", d, opt)[0]
    ddir = opt + (112 if magic == 0x20b else 96)
    ex_rva, ex_sz = struct.unpack_from("<II", d, ddir + 3 * 8)
    return d, ib, secs, ex_rva, ex_sz


def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            off = rva - va
            if off >= rsz:
                return None
            return rraw + off
    return None


def pdata(d, secs, ex_rva, ex_sz):
    """.pdata RUNTIME_FUNCTION → [(start, end)] 정렬"""
    po = roff(secs, ex_rva)
    out = []
    for i in range(ex_sz // 12):
        s, e, u = struct.unpack_from("<III", d, po + i * 12)
        out.append((s, e))
    out.sort()
    return out


def fn_at(fns, rva):
    lo, hi = 0, len(fns) - 1
    while lo <= hi:
        m = (lo + hi) // 2
        if fns[m][0] <= rva < fns[m][1]:
            return m
        if rva < fns[m][0]:
            hi = m - 1
        else:
            lo = m + 1
    return None


def dump(tag, d, secs, ib, s, e, maxn=40):
    o = roff(secs, s)
    b = d[o:o + min(e - s, 200)]
    print(f"  {tag} rva=0x{s:x} size={e-s}")
    n = 0
    for i in md.disasm(b, ib + s):
        print(f"      {i.address-ib:#010x}  {i.bytes.hex(' '):<24s} {i.mnemonic} {i.op_str}")
        n += 1
        if n >= maxn:
            break


DO, IBO, SO, EO, ZO = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe")
DN, IBN, SN, EN, ZN = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe")
FO = pdata(DO, SO, EO, ZO)
FN = pdata(DN, SN, EN, ZN)
print(f"함수 수: 0.5.2={len(FO)}  0.5.3={len(FN)}\n")

print("=== 0.5.2 할당자 클러스터 ===")
for nm, rva in [("ALLOC  ", 0x25c4d30), ("DEALLOC", 0x25c4d90), ("REALLOC", 0x25c4dd0)]:
    k = fn_at(FO, rva)
    if k is None:
        print(f"  {nm} 0x{rva:x} — .pdata에 없음")
        continue
    dump(nm, DO, SO, IBO, *FO[k], maxn=14)
    print()

print("=== 0.5.3 REALLOC 확정(0x28e3b10) 주변 ±6 함수 ===")
k = fn_at(FN, 0x28e3b10)
print(f"  (realloc index={k})")
for j in range(max(0, k - 6), min(len(FN), k + 7)):
    s, e = FN[j]
    mark = " ←REALLOC" if s == 0x28e3b10 else ""
    o = roff(SN, s)
    print(f"  [{j-k:+d}] rva=0x{s:x} size={e-s} head={DN[o:o+16].hex(' ')}{mark}")
print()
for j in range(max(0, k - 6), min(len(FN), k + 7)):
    if FN[j][0] == 0x28e3b10:
        continue
    dump(f"[{j-k:+d}]", DN, SN, IBN, *FN[j], maxn=12)
    print()
